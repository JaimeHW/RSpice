#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq22_e682, eq22_e682_d_n0, eq22_e682_d_n1, eq22_e682_d_n2, eq22_e682_d_n3, eq22_e682_d_n4, eq22_e682_d_n5, eq22_e682_d_n6, eq22_e682_d_n7, eq22_e682_d_n8, eq22_e682_d_n9, eq22_e682_d_n10, eq22_e682_d_n11, eq22_e682_d_n12, eq22_e682_d_n13, eq22_e682_d_n14, eq22_e682_d_n15, eq22_e682_d_n16, eq22_e682_d_n17, eq22_e682_d_n18, eq22_e682_d_n19, eq22_e682_d_n20, eq22_e682_d_n21, eq22_e682_d_n22, eq22_e682_d_n23, eq22_e682_d_n24, eq22_e682_d_n25, eq22_e682_d_n26, eq22_e682_d_n27, eq22_e682_d_n28, eq22_e682_d_n29,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq22_e661: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[227]);
        let eq22_e661_d_n0: f64 = (s.dn[227][0] * ddt_scale);
        let eq22_e661_d_n1: f64 = (s.dn[227][1] * ddt_scale);
        let eq22_e661_d_n2: f64 = (s.dn[227][2] * ddt_scale);
        let eq22_e661_d_n3: f64 = (s.dn[227][3] * ddt_scale);
        let eq22_e661_d_n4: f64 = (s.dn[227][4] * ddt_scale);
        let eq22_e661_d_n5: f64 = (s.dn[227][5] * ddt_scale);
        let eq22_e661_d_n6: f64 = (s.dn[227][6] * ddt_scale);
        let eq22_e661_d_n7: f64 = (s.dn[227][7] * ddt_scale);
        let eq22_e661_d_n8: f64 = (s.dn[227][8] * ddt_scale);
        let eq22_e661_d_n9: f64 = (s.dn[227][9] * ddt_scale);
        let eq22_e661_d_n10: f64 = (s.dn[227][10] * ddt_scale);
        let eq22_e661_d_n11: f64 = (s.dn[227][11] * ddt_scale);
        let eq22_e661_d_n12: f64 = (s.dn[227][12] * ddt_scale);
        let eq22_e661_d_n13: f64 = (s.dn[227][13] * ddt_scale);
        let eq22_e661_d_n14: f64 = (s.dn[227][14] * ddt_scale);
        let eq22_e661_d_n15: f64 = (s.dn[227][15] * ddt_scale);
        let eq22_e661_d_n16: f64 = (s.dn[227][16] * ddt_scale);
        let eq22_e661_d_n17: f64 = (s.dn[227][17] * ddt_scale);
        let eq22_e661_d_n18: f64 = (s.dn[227][18] * ddt_scale);
        let eq22_e661_d_n19: f64 = (s.dn[227][19] * ddt_scale);
        let eq22_e661_d_n20: f64 = (s.dn[227][20] * ddt_scale);
        let eq22_e661_d_n21: f64 = (s.dn[227][21] * ddt_scale);
        let eq22_e661_d_n22: f64 = (s.dn[227][22] * ddt_scale);
        let eq22_e661_d_n23: f64 = (s.dn[227][23] * ddt_scale);
        let eq22_e661_d_n24: f64 = (s.dn[227][24] * ddt_scale);
        let eq22_e661_d_n25: f64 = (s.dn[227][25] * ddt_scale);
        let eq22_e661_d_n26: f64 = (s.dn[227][26] * ddt_scale);
        let eq22_e661_d_n27: f64 = (s.dn[227][27] * ddt_scale);
        let eq22_e661_d_n28: f64 = (s.dn[227][28] * ddt_scale);
        let eq22_e661_d_n29: f64 = (s.dn[227][29] * ddt_scale);
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
            multiplicity * (eq22_value),
            nodes,
            &eq22_node_derivatives,
            branches,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e690,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e690;
        stamper.stamp_potential_const(
            branches[10],
            eq23_value,
        );
        let (eq24_e698,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e698;
        stamper.stamp_potential_const(
            branches[11],
            eq24_value,
        );
        let (eq25_e706,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e706;
        stamper.stamp_potential_const(
            branches[12],
            eq25_value,
        );
        let (eq26_e714,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e714;
        stamper.stamp_potential_const(
            branches[13],
            eq26_value,
        );
        let (eq27_e722,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e722;
        stamper.stamp_potential_const(
            branches[14],
            eq27_value,
        );
        let (eq28_e730,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e730;
        stamper.stamp_potential_const(
            branches[15],
            eq28_value,
        );
        let (eq29_e738,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e738;
        stamper.stamp_potential_const(
            branches[16],
            eq29_value,
        );
        let (eq30_e746,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e746;
        stamper.stamp_potential_const(
            branches[17],
            eq30_value,
        );
        let (eq31_e754, eq31_e754_d_n0, eq31_e754_d_n1, eq31_e754_d_n2, eq31_e754_d_n3, eq31_e754_d_n4, eq31_e754_d_n5, eq31_e754_d_n6, eq31_e754_d_n7, eq31_e754_d_n8, eq31_e754_d_n9, eq31_e754_d_n10, eq31_e754_d_n11, eq31_e754_d_n12, eq31_e754_d_n13, eq31_e754_d_n14, eq31_e754_d_n15, eq31_e754_d_n16, eq31_e754_d_n17, eq31_e754_d_n18, eq31_e754_d_n19, eq31_e754_d_n20, eq31_e754_d_n21, eq31_e754_d_n22, eq31_e754_d_n23, eq31_e754_d_n24, eq31_e754_d_n25, eq31_e754_d_n26, eq31_e754_d_n27, eq31_e754_d_n28, eq31_e754_d_n29,) = {
    if s.b[320] {
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
            multiplicity * (eq31_value),
            nodes,
            &eq31_node_derivatives,
            branches,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e759,) = {
    if (!s.b[320]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e759;
        stamper.stamp_potential_const(
            branches[18],
            eq32_value,
        );
        let (eq33_e769, eq33_e769_d_n0, eq33_e769_d_n1, eq33_e769_d_n2, eq33_e769_d_n3, eq33_e769_d_n4, eq33_e769_d_n5, eq33_e769_d_n6, eq33_e769_d_n7, eq33_e769_d_n8, eq33_e769_d_n9, eq33_e769_d_n10, eq33_e769_d_n11, eq33_e769_d_n12, eq33_e769_d_n13, eq33_e769_d_n14, eq33_e769_d_n15, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_d_n18, eq33_e769_d_n19, eq33_e769_d_n20, eq33_e769_d_n21, eq33_e769_d_n22, eq33_e769_d_n23, eq33_e769_d_n24, eq33_e769_d_n25, eq33_e769_d_n26, eq33_e769_d_n27, eq33_e769_d_n28, eq33_e769_d_n29,) = {
    if s.b[466] {
        let eq33_e762: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[209]);
        let eq33_e762_d_n0: f64 = (s.dn[209][0] * ddt_scale);
        let eq33_e762_d_n1: f64 = (s.dn[209][1] * ddt_scale);
        let eq33_e762_d_n2: f64 = (s.dn[209][2] * ddt_scale);
        let eq33_e762_d_n3: f64 = (s.dn[209][3] * ddt_scale);
        let eq33_e762_d_n4: f64 = (s.dn[209][4] * ddt_scale);
        let eq33_e762_d_n5: f64 = (s.dn[209][5] * ddt_scale);
        let eq33_e762_d_n6: f64 = (s.dn[209][6] * ddt_scale);
        let eq33_e762_d_n7: f64 = (s.dn[209][7] * ddt_scale);
        let eq33_e762_d_n8: f64 = (s.dn[209][8] * ddt_scale);
        let eq33_e762_d_n9: f64 = (s.dn[209][9] * ddt_scale);
        let eq33_e762_d_n10: f64 = (s.dn[209][10] * ddt_scale);
        let eq33_e762_d_n11: f64 = (s.dn[209][11] * ddt_scale);
        let eq33_e762_d_n12: f64 = (s.dn[209][12] * ddt_scale);
        let eq33_e762_d_n13: f64 = (s.dn[209][13] * ddt_scale);
        let eq33_e762_d_n14: f64 = (s.dn[209][14] * ddt_scale);
        let eq33_e762_d_n15: f64 = (s.dn[209][15] * ddt_scale);
        let eq33_e762_d_n16: f64 = (s.dn[209][16] * ddt_scale);
        let eq33_e762_d_n17: f64 = (s.dn[209][17] * ddt_scale);
        let eq33_e762_d_n18: f64 = (s.dn[209][18] * ddt_scale);
        let eq33_e762_d_n19: f64 = (s.dn[209][19] * ddt_scale);
        let eq33_e762_d_n20: f64 = (s.dn[209][20] * ddt_scale);
        let eq33_e762_d_n21: f64 = (s.dn[209][21] * ddt_scale);
        let eq33_e762_d_n22: f64 = (s.dn[209][22] * ddt_scale);
        let eq33_e762_d_n23: f64 = (s.dn[209][23] * ddt_scale);
        let eq33_e762_d_n24: f64 = (s.dn[209][24] * ddt_scale);
        let eq33_e762_d_n25: f64 = (s.dn[209][25] * ddt_scale);
        let eq33_e762_d_n26: f64 = (s.dn[209][26] * ddt_scale);
        let eq33_e762_d_n27: f64 = (s.dn[209][27] * ddt_scale);
        let eq33_e762_d_n28: f64 = (s.dn[209][28] * ddt_scale);
        let eq33_e762_d_n29: f64 = (s.dn[209][29] * ddt_scale);
        let eq33_e765: f64 = (p.p355 * (nv7 - nv16));
        let eq33_e765_d_n7: f64 = p.p355;
        let eq33_e765_d_n16: f64 = (-p.p355);
        let eq33_e766: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq33_e765);
        let eq33_e766_d_n7: f64 = (eq33_e765_d_n7 * ddt_scale);
        let eq33_e766_d_n16: f64 = (eq33_e765_d_n16 * ddt_scale);
        let eq33_e767: f64 = (eq33_e762 + eq33_e766);
        let eq33_e767_d_n7: f64 = (eq33_e762_d_n7 + eq33_e766_d_n7);
        let eq33_e767_d_n16: f64 = (eq33_e762_d_n16 + eq33_e766_d_n16);
        (eq33_e767, eq33_e762_d_n0, eq33_e762_d_n1, eq33_e762_d_n2, eq33_e762_d_n3, eq33_e762_d_n4, eq33_e762_d_n5, eq33_e762_d_n6, eq33_e767_d_n7, eq33_e762_d_n8, eq33_e762_d_n9, eq33_e762_d_n10, eq33_e762_d_n11, eq33_e762_d_n12, eq33_e762_d_n13, eq33_e762_d_n14, eq33_e762_d_n15, eq33_e767_d_n16, eq33_e762_d_n17, eq33_e762_d_n18, eq33_e762_d_n19, eq33_e762_d_n20, eq33_e762_d_n21, eq33_e762_d_n22, eq33_e762_d_n23, eq33_e762_d_n24, eq33_e762_d_n25, eq33_e762_d_n26, eq33_e762_d_n27, eq33_e762_d_n28, eq33_e762_d_n29,)
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
            multiplicity * (eq33_value),
            nodes,
            &eq33_node_derivatives,
            branches,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e779, eq34_e779_d_n0, eq34_e779_d_n1, eq34_e779_d_n2, eq34_e779_d_n3, eq34_e779_d_n4, eq34_e779_d_n5, eq34_e779_d_n6, eq34_e779_d_n7, eq34_e779_d_n8, eq34_e779_d_n9, eq34_e779_d_n10, eq34_e779_d_n11, eq34_e779_d_n12, eq34_e779_d_n13, eq34_e779_d_n14, eq34_e779_d_n15, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_d_n18, eq34_e779_d_n19, eq34_e779_d_n20, eq34_e779_d_n21, eq34_e779_d_n22, eq34_e779_d_n23, eq34_e779_d_n24, eq34_e779_d_n25, eq34_e779_d_n26, eq34_e779_d_n27, eq34_e779_d_n28, eq34_e779_d_n29,) = {
    if s.b[466] {
        let eq34_e772: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[210]);
        let eq34_e772_d_n0: f64 = (s.dn[210][0] * ddt_scale);
        let eq34_e772_d_n1: f64 = (s.dn[210][1] * ddt_scale);
        let eq34_e772_d_n2: f64 = (s.dn[210][2] * ddt_scale);
        let eq34_e772_d_n3: f64 = (s.dn[210][3] * ddt_scale);
        let eq34_e772_d_n4: f64 = (s.dn[210][4] * ddt_scale);
        let eq34_e772_d_n5: f64 = (s.dn[210][5] * ddt_scale);
        let eq34_e772_d_n6: f64 = (s.dn[210][6] * ddt_scale);
        let eq34_e772_d_n7: f64 = (s.dn[210][7] * ddt_scale);
        let eq34_e772_d_n8: f64 = (s.dn[210][8] * ddt_scale);
        let eq34_e772_d_n9: f64 = (s.dn[210][9] * ddt_scale);
        let eq34_e772_d_n10: f64 = (s.dn[210][10] * ddt_scale);
        let eq34_e772_d_n11: f64 = (s.dn[210][11] * ddt_scale);
        let eq34_e772_d_n12: f64 = (s.dn[210][12] * ddt_scale);
        let eq34_e772_d_n13: f64 = (s.dn[210][13] * ddt_scale);
        let eq34_e772_d_n14: f64 = (s.dn[210][14] * ddt_scale);
        let eq34_e772_d_n15: f64 = (s.dn[210][15] * ddt_scale);
        let eq34_e772_d_n16: f64 = (s.dn[210][16] * ddt_scale);
        let eq34_e772_d_n17: f64 = (s.dn[210][17] * ddt_scale);
        let eq34_e772_d_n18: f64 = (s.dn[210][18] * ddt_scale);
        let eq34_e772_d_n19: f64 = (s.dn[210][19] * ddt_scale);
        let eq34_e772_d_n20: f64 = (s.dn[210][20] * ddt_scale);
        let eq34_e772_d_n21: f64 = (s.dn[210][21] * ddt_scale);
        let eq34_e772_d_n22: f64 = (s.dn[210][22] * ddt_scale);
        let eq34_e772_d_n23: f64 = (s.dn[210][23] * ddt_scale);
        let eq34_e772_d_n24: f64 = (s.dn[210][24] * ddt_scale);
        let eq34_e772_d_n25: f64 = (s.dn[210][25] * ddt_scale);
        let eq34_e772_d_n26: f64 = (s.dn[210][26] * ddt_scale);
        let eq34_e772_d_n27: f64 = (s.dn[210][27] * ddt_scale);
        let eq34_e772_d_n28: f64 = (s.dn[210][28] * ddt_scale);
        let eq34_e772_d_n29: f64 = (s.dn[210][29] * ddt_scale);
        let eq34_e775: f64 = (p.p355 * (nv7 - nv17));
        let eq34_e775_d_n7: f64 = p.p355;
        let eq34_e775_d_n17: f64 = (-p.p355);
        let eq34_e776: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq34_e775);
        let eq34_e776_d_n7: f64 = (eq34_e775_d_n7 * ddt_scale);
        let eq34_e776_d_n17: f64 = (eq34_e775_d_n17 * ddt_scale);
        let eq34_e777: f64 = (eq34_e772 + eq34_e776);
        let eq34_e777_d_n7: f64 = (eq34_e772_d_n7 + eq34_e776_d_n7);
        let eq34_e777_d_n17: f64 = (eq34_e772_d_n17 + eq34_e776_d_n17);
        (eq34_e777, eq34_e772_d_n0, eq34_e772_d_n1, eq34_e772_d_n2, eq34_e772_d_n3, eq34_e772_d_n4, eq34_e772_d_n5, eq34_e772_d_n6, eq34_e777_d_n7, eq34_e772_d_n8, eq34_e772_d_n9, eq34_e772_d_n10, eq34_e772_d_n11, eq34_e772_d_n12, eq34_e772_d_n13, eq34_e772_d_n14, eq34_e772_d_n15, eq34_e772_d_n16, eq34_e777_d_n17, eq34_e772_d_n18, eq34_e772_d_n19, eq34_e772_d_n20, eq34_e772_d_n21, eq34_e772_d_n22, eq34_e772_d_n23, eq34_e772_d_n24, eq34_e772_d_n25, eq34_e772_d_n26, eq34_e772_d_n27, eq34_e772_d_n28, eq34_e772_d_n29,)
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
            multiplicity * (eq34_value),
            nodes,
            &eq34_node_derivatives,
            branches,
            &eq34_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq35_e789, eq35_e789_d_n0, eq35_e789_d_n1, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n5, eq35_e789_d_n6, eq35_e789_d_n7, eq35_e789_d_n8, eq35_e789_d_n9, eq35_e789_d_n10, eq35_e789_d_n11, eq35_e789_d_n12, eq35_e789_d_n13, eq35_e789_d_n14, eq35_e789_d_n15, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_d_n18, eq35_e789_d_n19, eq35_e789_d_n20, eq35_e789_d_n21, eq35_e789_d_n22, eq35_e789_d_n23, eq35_e789_d_n24, eq35_e789_d_n25, eq35_e789_d_n26, eq35_e789_d_n27, eq35_e789_d_n28, eq35_e789_d_n29,) = {
    if s.b[466] {
        let eq35_e782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, s.v[211]);
        let eq35_e782_d_n0: f64 = (s.dn[211][0] * ddt_scale);
        let eq35_e782_d_n1: f64 = (s.dn[211][1] * ddt_scale);
        let eq35_e782_d_n2: f64 = (s.dn[211][2] * ddt_scale);
        let eq35_e782_d_n3: f64 = (s.dn[211][3] * ddt_scale);
        let eq35_e782_d_n4: f64 = (s.dn[211][4] * ddt_scale);
        let eq35_e782_d_n5: f64 = (s.dn[211][5] * ddt_scale);
        let eq35_e782_d_n6: f64 = (s.dn[211][6] * ddt_scale);
        let eq35_e782_d_n7: f64 = (s.dn[211][7] * ddt_scale);
        let eq35_e782_d_n8: f64 = (s.dn[211][8] * ddt_scale);
        let eq35_e782_d_n9: f64 = (s.dn[211][9] * ddt_scale);
        let eq35_e782_d_n10: f64 = (s.dn[211][10] * ddt_scale);
        let eq35_e782_d_n11: f64 = (s.dn[211][11] * ddt_scale);
        let eq35_e782_d_n12: f64 = (s.dn[211][12] * ddt_scale);
        let eq35_e782_d_n13: f64 = (s.dn[211][13] * ddt_scale);
        let eq35_e782_d_n14: f64 = (s.dn[211][14] * ddt_scale);
        let eq35_e782_d_n15: f64 = (s.dn[211][15] * ddt_scale);
        let eq35_e782_d_n16: f64 = (s.dn[211][16] * ddt_scale);
        let eq35_e782_d_n17: f64 = (s.dn[211][17] * ddt_scale);
        let eq35_e782_d_n18: f64 = (s.dn[211][18] * ddt_scale);
        let eq35_e782_d_n19: f64 = (s.dn[211][19] * ddt_scale);
        let eq35_e782_d_n20: f64 = (s.dn[211][20] * ddt_scale);
        let eq35_e782_d_n21: f64 = (s.dn[211][21] * ddt_scale);
        let eq35_e782_d_n22: f64 = (s.dn[211][22] * ddt_scale);
        let eq35_e782_d_n23: f64 = (s.dn[211][23] * ddt_scale);
        let eq35_e782_d_n24: f64 = (s.dn[211][24] * ddt_scale);
        let eq35_e782_d_n25: f64 = (s.dn[211][25] * ddt_scale);
        let eq35_e782_d_n26: f64 = (s.dn[211][26] * ddt_scale);
        let eq35_e782_d_n27: f64 = (s.dn[211][27] * ddt_scale);
        let eq35_e782_d_n28: f64 = (s.dn[211][28] * ddt_scale);
        let eq35_e782_d_n29: f64 = (s.dn[211][29] * ddt_scale);
        let eq35_e785: f64 = (p.p355 * (nv2 - nv16));
        let eq35_e785_d_n2: f64 = p.p355;
        let eq35_e785_d_n16: f64 = (-p.p355);
        let eq35_e786: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq35_e785);
        let eq35_e786_d_n2: f64 = (eq35_e785_d_n2 * ddt_scale);
        let eq35_e786_d_n16: f64 = (eq35_e785_d_n16 * ddt_scale);
        let eq35_e787: f64 = (eq35_e782 + eq35_e786);
        let eq35_e787_d_n2: f64 = (eq35_e782_d_n2 + eq35_e786_d_n2);
        let eq35_e787_d_n16: f64 = (eq35_e782_d_n16 + eq35_e786_d_n16);
        (eq35_e787, eq35_e782_d_n0, eq35_e782_d_n1, eq35_e787_d_n2, eq35_e782_d_n3, eq35_e782_d_n4, eq35_e782_d_n5, eq35_e782_d_n6, eq35_e782_d_n7, eq35_e782_d_n8, eq35_e782_d_n9, eq35_e782_d_n10, eq35_e782_d_n11, eq35_e782_d_n12, eq35_e782_d_n13, eq35_e782_d_n14, eq35_e782_d_n15, eq35_e787_d_n16, eq35_e782_d_n17, eq35_e782_d_n18, eq35_e782_d_n19, eq35_e782_d_n20, eq35_e782_d_n21, eq35_e782_d_n22, eq35_e782_d_n23, eq35_e782_d_n24, eq35_e782_d_n25, eq35_e782_d_n26, eq35_e782_d_n27, eq35_e782_d_n28, eq35_e782_d_n29,)
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
            multiplicity * (eq35_value),
            nodes,
            &eq35_node_derivatives,
            branches,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let (eq36_e793,) = {
    if s.b[466] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq36_value: f64 = eq36_e793;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[17]),
            multiplicity * (eq36_value),
        );
        let (eq37_e803, eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29,) = {
    if s.b[466] {
        let eq37_e796: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[213]);
        let eq37_e796_d_n0: f64 = (s.dn[213][0] * ddt_scale);
        let eq37_e796_d_n1: f64 = (s.dn[213][1] * ddt_scale);
        let eq37_e796_d_n2: f64 = (s.dn[213][2] * ddt_scale);
        let eq37_e796_d_n3: f64 = (s.dn[213][3] * ddt_scale);
        let eq37_e796_d_n4: f64 = (s.dn[213][4] * ddt_scale);
        let eq37_e796_d_n5: f64 = (s.dn[213][5] * ddt_scale);
        let eq37_e796_d_n6: f64 = (s.dn[213][6] * ddt_scale);
        let eq37_e796_d_n7: f64 = (s.dn[213][7] * ddt_scale);
        let eq37_e796_d_n8: f64 = (s.dn[213][8] * ddt_scale);
        let eq37_e796_d_n9: f64 = (s.dn[213][9] * ddt_scale);
        let eq37_e796_d_n10: f64 = (s.dn[213][10] * ddt_scale);
        let eq37_e796_d_n11: f64 = (s.dn[213][11] * ddt_scale);
        let eq37_e796_d_n12: f64 = (s.dn[213][12] * ddt_scale);
        let eq37_e796_d_n13: f64 = (s.dn[213][13] * ddt_scale);
        let eq37_e796_d_n14: f64 = (s.dn[213][14] * ddt_scale);
        let eq37_e796_d_n15: f64 = (s.dn[213][15] * ddt_scale);
        let eq37_e796_d_n16: f64 = (s.dn[213][16] * ddt_scale);
        let eq37_e796_d_n17: f64 = (s.dn[213][17] * ddt_scale);
        let eq37_e796_d_n18: f64 = (s.dn[213][18] * ddt_scale);
        let eq37_e796_d_n19: f64 = (s.dn[213][19] * ddt_scale);
        let eq37_e796_d_n20: f64 = (s.dn[213][20] * ddt_scale);
        let eq37_e796_d_n21: f64 = (s.dn[213][21] * ddt_scale);
        let eq37_e796_d_n22: f64 = (s.dn[213][22] * ddt_scale);
        let eq37_e796_d_n23: f64 = (s.dn[213][23] * ddt_scale);
        let eq37_e796_d_n24: f64 = (s.dn[213][24] * ddt_scale);
        let eq37_e796_d_n25: f64 = (s.dn[213][25] * ddt_scale);
        let eq37_e796_d_n26: f64 = (s.dn[213][26] * ddt_scale);
        let eq37_e796_d_n27: f64 = (s.dn[213][27] * ddt_scale);
        let eq37_e796_d_n28: f64 = (s.dn[213][28] * ddt_scale);
        let eq37_e796_d_n29: f64 = (s.dn[213][29] * ddt_scale);
        let eq37_e799: f64 = (p.p355 * (nv7 - nv9));
        let eq37_e799_d_n7: f64 = p.p355;
        let eq37_e799_d_n9: f64 = (-p.p355);
        let eq37_e800: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq37_e799);
        let eq37_e800_d_n7: f64 = (eq37_e799_d_n7 * ddt_scale);
        let eq37_e800_d_n9: f64 = (eq37_e799_d_n9 * ddt_scale);
        let eq37_e801: f64 = (eq37_e796 + eq37_e800);
        let eq37_e801_d_n7: f64 = (eq37_e796_d_n7 + eq37_e800_d_n7);
        let eq37_e801_d_n9: f64 = (eq37_e796_d_n9 + eq37_e800_d_n9);
        (eq37_e801, eq37_e796_d_n0, eq37_e796_d_n1, eq37_e796_d_n2, eq37_e796_d_n3, eq37_e796_d_n4, eq37_e796_d_n5, eq37_e796_d_n6, eq37_e801_d_n7, eq37_e796_d_n8, eq37_e801_d_n9, eq37_e796_d_n10, eq37_e796_d_n11, eq37_e796_d_n12, eq37_e796_d_n13, eq37_e796_d_n14, eq37_e796_d_n15, eq37_e796_d_n16, eq37_e796_d_n17, eq37_e796_d_n18, eq37_e796_d_n19, eq37_e796_d_n20, eq37_e796_d_n21, eq37_e796_d_n22, eq37_e796_d_n23, eq37_e796_d_n24, eq37_e796_d_n25, eq37_e796_d_n26, eq37_e796_d_n27, eq37_e796_d_n28, eq37_e796_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e803;
        let eq37_node_derivatives: [f64; 30] = [eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29];
        let eq37_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq37_value),
            nodes,
            &eq37_node_derivatives,
            branches,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let (eq38_e814, eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29,) = {
    if (!s.b[466]) {
        let eq38_e807: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, s.v[209]);
        let eq38_e807_d_n0: f64 = (s.dn[209][0] * ddt_scale);
        let eq38_e807_d_n1: f64 = (s.dn[209][1] * ddt_scale);
        let eq38_e807_d_n2: f64 = (s.dn[209][2] * ddt_scale);
        let eq38_e807_d_n3: f64 = (s.dn[209][3] * ddt_scale);
        let eq38_e807_d_n4: f64 = (s.dn[209][4] * ddt_scale);
        let eq38_e807_d_n5: f64 = (s.dn[209][5] * ddt_scale);
        let eq38_e807_d_n6: f64 = (s.dn[209][6] * ddt_scale);
        let eq38_e807_d_n7: f64 = (s.dn[209][7] * ddt_scale);
        let eq38_e807_d_n8: f64 = (s.dn[209][8] * ddt_scale);
        let eq38_e807_d_n9: f64 = (s.dn[209][9] * ddt_scale);
        let eq38_e807_d_n10: f64 = (s.dn[209][10] * ddt_scale);
        let eq38_e807_d_n11: f64 = (s.dn[209][11] * ddt_scale);
        let eq38_e807_d_n12: f64 = (s.dn[209][12] * ddt_scale);
        let eq38_e807_d_n13: f64 = (s.dn[209][13] * ddt_scale);
        let eq38_e807_d_n14: f64 = (s.dn[209][14] * ddt_scale);
        let eq38_e807_d_n15: f64 = (s.dn[209][15] * ddt_scale);
        let eq38_e807_d_n16: f64 = (s.dn[209][16] * ddt_scale);
        let eq38_e807_d_n17: f64 = (s.dn[209][17] * ddt_scale);
        let eq38_e807_d_n18: f64 = (s.dn[209][18] * ddt_scale);
        let eq38_e807_d_n19: f64 = (s.dn[209][19] * ddt_scale);
        let eq38_e807_d_n20: f64 = (s.dn[209][20] * ddt_scale);
        let eq38_e807_d_n21: f64 = (s.dn[209][21] * ddt_scale);
        let eq38_e807_d_n22: f64 = (s.dn[209][22] * ddt_scale);
        let eq38_e807_d_n23: f64 = (s.dn[209][23] * ddt_scale);
        let eq38_e807_d_n24: f64 = (s.dn[209][24] * ddt_scale);
        let eq38_e807_d_n25: f64 = (s.dn[209][25] * ddt_scale);
        let eq38_e807_d_n26: f64 = (s.dn[209][26] * ddt_scale);
        let eq38_e807_d_n27: f64 = (s.dn[209][27] * ddt_scale);
        let eq38_e807_d_n28: f64 = (s.dn[209][28] * ddt_scale);
        let eq38_e807_d_n29: f64 = (s.dn[209][29] * ddt_scale);
        let eq38_e810: f64 = (p.p355 * (nv2 - nv16));
        let eq38_e810_d_n2: f64 = p.p355;
        let eq38_e810_d_n16: f64 = (-p.p355);
        let eq38_e811: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq38_e810);
        let eq38_e811_d_n2: f64 = (eq38_e810_d_n2 * ddt_scale);
        let eq38_e811_d_n16: f64 = (eq38_e810_d_n16 * ddt_scale);
        let eq38_e812: f64 = (eq38_e807 + eq38_e811);
        let eq38_e812_d_n2: f64 = (eq38_e807_d_n2 + eq38_e811_d_n2);
        let eq38_e812_d_n16: f64 = (eq38_e807_d_n16 + eq38_e811_d_n16);
        (eq38_e812, eq38_e807_d_n0, eq38_e807_d_n1, eq38_e812_d_n2, eq38_e807_d_n3, eq38_e807_d_n4, eq38_e807_d_n5, eq38_e807_d_n6, eq38_e807_d_n7, eq38_e807_d_n8, eq38_e807_d_n9, eq38_e807_d_n10, eq38_e807_d_n11, eq38_e807_d_n12, eq38_e807_d_n13, eq38_e807_d_n14, eq38_e807_d_n15, eq38_e812_d_n16, eq38_e807_d_n17, eq38_e807_d_n18, eq38_e807_d_n19, eq38_e807_d_n20, eq38_e807_d_n21, eq38_e807_d_n22, eq38_e807_d_n23, eq38_e807_d_n24, eq38_e807_d_n25, eq38_e807_d_n26, eq38_e807_d_n27, eq38_e807_d_n28, eq38_e807_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e814;
        let eq38_node_derivatives: [f64; 30] = [eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            multiplicity * (eq38_value),
            nodes,
            &eq38_node_derivatives,
            branches,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e825, eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29,) = {
    if (!s.b[466]) {
        let eq39_e818: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, s.v[210]);
        let eq39_e818_d_n0: f64 = (s.dn[210][0] * ddt_scale);
        let eq39_e818_d_n1: f64 = (s.dn[210][1] * ddt_scale);
        let eq39_e818_d_n2: f64 = (s.dn[210][2] * ddt_scale);
        let eq39_e818_d_n3: f64 = (s.dn[210][3] * ddt_scale);
        let eq39_e818_d_n4: f64 = (s.dn[210][4] * ddt_scale);
        let eq39_e818_d_n5: f64 = (s.dn[210][5] * ddt_scale);
        let eq39_e818_d_n6: f64 = (s.dn[210][6] * ddt_scale);
        let eq39_e818_d_n7: f64 = (s.dn[210][7] * ddt_scale);
        let eq39_e818_d_n8: f64 = (s.dn[210][8] * ddt_scale);
        let eq39_e818_d_n9: f64 = (s.dn[210][9] * ddt_scale);
        let eq39_e818_d_n10: f64 = (s.dn[210][10] * ddt_scale);
        let eq39_e818_d_n11: f64 = (s.dn[210][11] * ddt_scale);
        let eq39_e818_d_n12: f64 = (s.dn[210][12] * ddt_scale);
        let eq39_e818_d_n13: f64 = (s.dn[210][13] * ddt_scale);
        let eq39_e818_d_n14: f64 = (s.dn[210][14] * ddt_scale);
        let eq39_e818_d_n15: f64 = (s.dn[210][15] * ddt_scale);
        let eq39_e818_d_n16: f64 = (s.dn[210][16] * ddt_scale);
        let eq39_e818_d_n17: f64 = (s.dn[210][17] * ddt_scale);
        let eq39_e818_d_n18: f64 = (s.dn[210][18] * ddt_scale);
        let eq39_e818_d_n19: f64 = (s.dn[210][19] * ddt_scale);
        let eq39_e818_d_n20: f64 = (s.dn[210][20] * ddt_scale);
        let eq39_e818_d_n21: f64 = (s.dn[210][21] * ddt_scale);
        let eq39_e818_d_n22: f64 = (s.dn[210][22] * ddt_scale);
        let eq39_e818_d_n23: f64 = (s.dn[210][23] * ddt_scale);
        let eq39_e818_d_n24: f64 = (s.dn[210][24] * ddt_scale);
        let eq39_e818_d_n25: f64 = (s.dn[210][25] * ddt_scale);
        let eq39_e818_d_n26: f64 = (s.dn[210][26] * ddt_scale);
        let eq39_e818_d_n27: f64 = (s.dn[210][27] * ddt_scale);
        let eq39_e818_d_n28: f64 = (s.dn[210][28] * ddt_scale);
        let eq39_e818_d_n29: f64 = (s.dn[210][29] * ddt_scale);
        let eq39_e821: f64 = (p.p355 * (nv2 - nv17));
        let eq39_e821_d_n2: f64 = p.p355;
        let eq39_e821_d_n17: f64 = (-p.p355);
        let eq39_e822: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq39_e821);
        let eq39_e822_d_n2: f64 = (eq39_e821_d_n2 * ddt_scale);
        let eq39_e822_d_n17: f64 = (eq39_e821_d_n17 * ddt_scale);
        let eq39_e823: f64 = (eq39_e818 + eq39_e822);
        let eq39_e823_d_n2: f64 = (eq39_e818_d_n2 + eq39_e822_d_n2);
        let eq39_e823_d_n17: f64 = (eq39_e818_d_n17 + eq39_e822_d_n17);
        (eq39_e823, eq39_e818_d_n0, eq39_e818_d_n1, eq39_e823_d_n2, eq39_e818_d_n3, eq39_e818_d_n4, eq39_e818_d_n5, eq39_e818_d_n6, eq39_e818_d_n7, eq39_e818_d_n8, eq39_e818_d_n9, eq39_e818_d_n10, eq39_e818_d_n11, eq39_e818_d_n12, eq39_e818_d_n13, eq39_e818_d_n14, eq39_e818_d_n15, eq39_e818_d_n16, eq39_e823_d_n17, eq39_e818_d_n18, eq39_e818_d_n19, eq39_e818_d_n20, eq39_e818_d_n21, eq39_e818_d_n22, eq39_e818_d_n23, eq39_e818_d_n24, eq39_e818_d_n25, eq39_e818_d_n26, eq39_e818_d_n27, eq39_e818_d_n28, eq39_e818_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e825;
        let eq39_node_derivatives: [f64; 30] = [eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            multiplicity * (eq39_value),
            nodes,
            &eq39_node_derivatives,
            branches,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e836, eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29,) = {
    if (!s.b[466]) {
        let eq40_e829: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, s.v[211]);
        let eq40_e829_d_n0: f64 = (s.dn[211][0] * ddt_scale);
        let eq40_e829_d_n1: f64 = (s.dn[211][1] * ddt_scale);
        let eq40_e829_d_n2: f64 = (s.dn[211][2] * ddt_scale);
        let eq40_e829_d_n3: f64 = (s.dn[211][3] * ddt_scale);
        let eq40_e829_d_n4: f64 = (s.dn[211][4] * ddt_scale);
        let eq40_e829_d_n5: f64 = (s.dn[211][5] * ddt_scale);
        let eq40_e829_d_n6: f64 = (s.dn[211][6] * ddt_scale);
        let eq40_e829_d_n7: f64 = (s.dn[211][7] * ddt_scale);
        let eq40_e829_d_n8: f64 = (s.dn[211][8] * ddt_scale);
        let eq40_e829_d_n9: f64 = (s.dn[211][9] * ddt_scale);
        let eq40_e829_d_n10: f64 = (s.dn[211][10] * ddt_scale);
        let eq40_e829_d_n11: f64 = (s.dn[211][11] * ddt_scale);
        let eq40_e829_d_n12: f64 = (s.dn[211][12] * ddt_scale);
        let eq40_e829_d_n13: f64 = (s.dn[211][13] * ddt_scale);
        let eq40_e829_d_n14: f64 = (s.dn[211][14] * ddt_scale);
        let eq40_e829_d_n15: f64 = (s.dn[211][15] * ddt_scale);
        let eq40_e829_d_n16: f64 = (s.dn[211][16] * ddt_scale);
        let eq40_e829_d_n17: f64 = (s.dn[211][17] * ddt_scale);
        let eq40_e829_d_n18: f64 = (s.dn[211][18] * ddt_scale);
        let eq40_e829_d_n19: f64 = (s.dn[211][19] * ddt_scale);
        let eq40_e829_d_n20: f64 = (s.dn[211][20] * ddt_scale);
        let eq40_e829_d_n21: f64 = (s.dn[211][21] * ddt_scale);
        let eq40_e829_d_n22: f64 = (s.dn[211][22] * ddt_scale);
        let eq40_e829_d_n23: f64 = (s.dn[211][23] * ddt_scale);
        let eq40_e829_d_n24: f64 = (s.dn[211][24] * ddt_scale);
        let eq40_e829_d_n25: f64 = (s.dn[211][25] * ddt_scale);
        let eq40_e829_d_n26: f64 = (s.dn[211][26] * ddt_scale);
        let eq40_e829_d_n27: f64 = (s.dn[211][27] * ddt_scale);
        let eq40_e829_d_n28: f64 = (s.dn[211][28] * ddt_scale);
        let eq40_e829_d_n29: f64 = (s.dn[211][29] * ddt_scale);
        let eq40_e832: f64 = (p.p355 * (nv7 - nv16));
        let eq40_e832_d_n7: f64 = p.p355;
        let eq40_e832_d_n16: f64 = (-p.p355);
        let eq40_e833: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, eq40_e832);
        let eq40_e833_d_n7: f64 = (eq40_e832_d_n7 * ddt_scale);
        let eq40_e833_d_n16: f64 = (eq40_e832_d_n16 * ddt_scale);
        let eq40_e834: f64 = (eq40_e829 + eq40_e833);
        let eq40_e834_d_n7: f64 = (eq40_e829_d_n7 + eq40_e833_d_n7);
        let eq40_e834_d_n16: f64 = (eq40_e829_d_n16 + eq40_e833_d_n16);
        (eq40_e834, eq40_e829_d_n0, eq40_e829_d_n1, eq40_e829_d_n2, eq40_e829_d_n3, eq40_e829_d_n4, eq40_e829_d_n5, eq40_e829_d_n6, eq40_e834_d_n7, eq40_e829_d_n8, eq40_e829_d_n9, eq40_e829_d_n10, eq40_e829_d_n11, eq40_e829_d_n12, eq40_e829_d_n13, eq40_e829_d_n14, eq40_e829_d_n15, eq40_e834_d_n16, eq40_e829_d_n17, eq40_e829_d_n18, eq40_e829_d_n19, eq40_e829_d_n20, eq40_e829_d_n21, eq40_e829_d_n22, eq40_e829_d_n23, eq40_e829_d_n24, eq40_e829_d_n25, eq40_e829_d_n26, eq40_e829_d_n27, eq40_e829_d_n28, eq40_e829_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e836;
        let eq40_node_derivatives: [f64; 30] = [eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            multiplicity * (eq40_value),
            nodes,
            &eq40_node_derivatives,
            branches,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let (eq41_e841,) = {
    if (!s.b[466]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e841;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[17]),
            multiplicity * (eq41_value),
        );
        let (eq42_e846,) = {
    if (!s.b[466]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq42_value: f64 = eq42_e846;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq42_value),
        );
        let eq43_e848: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, s.v[212]);
        let eq43_e848_d_n0: f64 = (s.dn[212][0] * ddt_scale);
        let eq43_e848_d_n1: f64 = (s.dn[212][1] * ddt_scale);
        let eq43_e848_d_n2: f64 = (s.dn[212][2] * ddt_scale);
        let eq43_e848_d_n3: f64 = (s.dn[212][3] * ddt_scale);
        let eq43_e848_d_n4: f64 = (s.dn[212][4] * ddt_scale);
        let eq43_e848_d_n5: f64 = (s.dn[212][5] * ddt_scale);
        let eq43_e848_d_n6: f64 = (s.dn[212][6] * ddt_scale);
        let eq43_e848_d_n7: f64 = (s.dn[212][7] * ddt_scale);
        let eq43_e848_d_n8: f64 = (s.dn[212][8] * ddt_scale);
        let eq43_e848_d_n9: f64 = (s.dn[212][9] * ddt_scale);
        let eq43_e848_d_n10: f64 = (s.dn[212][10] * ddt_scale);
        let eq43_e848_d_n11: f64 = (s.dn[212][11] * ddt_scale);
        let eq43_e848_d_n12: f64 = (s.dn[212][12] * ddt_scale);
        let eq43_e848_d_n13: f64 = (s.dn[212][13] * ddt_scale);
        let eq43_e848_d_n14: f64 = (s.dn[212][14] * ddt_scale);
        let eq43_e848_d_n15: f64 = (s.dn[212][15] * ddt_scale);
        let eq43_e848_d_n16: f64 = (s.dn[212][16] * ddt_scale);
        let eq43_e848_d_n17: f64 = (s.dn[212][17] * ddt_scale);
        let eq43_e848_d_n18: f64 = (s.dn[212][18] * ddt_scale);
        let eq43_e848_d_n19: f64 = (s.dn[212][19] * ddt_scale);
        let eq43_e848_d_n20: f64 = (s.dn[212][20] * ddt_scale);
        let eq43_e848_d_n21: f64 = (s.dn[212][21] * ddt_scale);
        let eq43_e848_d_n22: f64 = (s.dn[212][22] * ddt_scale);
        let eq43_e848_d_n23: f64 = (s.dn[212][23] * ddt_scale);
        let eq43_e848_d_n24: f64 = (s.dn[212][24] * ddt_scale);
        let eq43_e848_d_n25: f64 = (s.dn[212][25] * ddt_scale);
        let eq43_e848_d_n26: f64 = (s.dn[212][26] * ddt_scale);
        let eq43_e848_d_n27: f64 = (s.dn[212][27] * ddt_scale);
        let eq43_e848_d_n28: f64 = (s.dn[212][28] * ddt_scale);
        let eq43_e848_d_n29: f64 = (s.dn[212][29] * ddt_scale);
        let eq43_e851: f64 = (p.p355 * (nv3 - nv16));
        let eq43_e851_d_n3: f64 = p.p355;
        let eq43_e851_d_n16: f64 = (-p.p355);
        let eq43_e852: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, eq43_e851);
        let eq43_e852_d_n3: f64 = (eq43_e851_d_n3 * ddt_scale);
        let eq43_e852_d_n16: f64 = (eq43_e851_d_n16 * ddt_scale);
        let eq43_e853: f64 = (eq43_e848 + eq43_e852);
        let eq43_e853_d_n3: f64 = (eq43_e848_d_n3 + eq43_e852_d_n3);
        let eq43_e853_d_n16: f64 = (eq43_e848_d_n16 + eq43_e852_d_n16);
        let eq43_value: f64 = eq43_e853;
        let eq43_node_derivatives: [f64; 30] = [eq43_e848_d_n0, eq43_e848_d_n1, eq43_e848_d_n2, eq43_e853_d_n3, eq43_e848_d_n4, eq43_e848_d_n5, eq43_e848_d_n6, eq43_e848_d_n7, eq43_e848_d_n8, eq43_e848_d_n9, eq43_e848_d_n10, eq43_e848_d_n11, eq43_e848_d_n12, eq43_e848_d_n13, eq43_e848_d_n14, eq43_e848_d_n15, eq43_e853_d_n16, eq43_e848_d_n17, eq43_e848_d_n18, eq43_e848_d_n19, eq43_e848_d_n20, eq43_e848_d_n21, eq43_e848_d_n22, eq43_e848_d_n23, eq43_e848_d_n24, eq43_e848_d_n25, eq43_e848_d_n26, eq43_e848_d_n27, eq43_e848_d_n28, eq43_e848_d_n29];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[16]),
            multiplicity * (eq43_value),
            nodes,
            &eq43_node_derivatives,
            branches,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e861, eq44_e861_d_n0, eq44_e861_d_n1, eq44_e861_d_n2, eq44_e861_d_n3, eq44_e861_d_n4, eq44_e861_d_n5, eq44_e861_d_n6, eq44_e861_d_n7, eq44_e861_d_n8, eq44_e861_d_n9, eq44_e861_d_n10, eq44_e861_d_n11, eq44_e861_d_n12, eq44_e861_d_n13, eq44_e861_d_n14, eq44_e861_d_n15, eq44_e861_d_n16, eq44_e861_d_n17, eq44_e861_d_n18, eq44_e861_d_n19, eq44_e861_d_n20, eq44_e861_d_n21, eq44_e861_d_n22, eq44_e861_d_n23, eq44_e861_d_n24, eq44_e861_d_n25, eq44_e861_d_n26, eq44_e861_d_n27, eq44_e861_d_n28, eq44_e861_d_n29,) = {
    if s.b[467] {
        let eq44_e858: f64 = (s.v[0] * (nv16 - nv15));
        let eq44_e858_d_n15: f64 = (-s.v[0]);
        let eq44_e858_d_n16: f64 = s.v[0];
        let eq44_e859: f64 = (s.v[202] + eq44_e858);
        let eq44_e859_d_n15: f64 = (s.dn[202][15] + eq44_e858_d_n15);
        let eq44_e859_d_n16: f64 = (s.dn[202][16] + eq44_e858_d_n16);
        (eq44_e859, s.dn[202][0], s.dn[202][1], s.dn[202][2], s.dn[202][3], s.dn[202][4], s.dn[202][5], s.dn[202][6], s.dn[202][7], s.dn[202][8], s.dn[202][9], s.dn[202][10], s.dn[202][11], s.dn[202][12], s.dn[202][13], s.dn[202][14], eq44_e859_d_n15, eq44_e859_d_n16, s.dn[202][17], s.dn[202][18], s.dn[202][19], s.dn[202][20], s.dn[202][21], s.dn[202][22], s.dn[202][23], s.dn[202][24], s.dn[202][25], s.dn[202][26], s.dn[202][27], s.dn[202][28], s.dn[202][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e861;
        let eq44_node_derivatives: [f64; 30] = [eq44_e861_d_n0, eq44_e861_d_n1, eq44_e861_d_n2, eq44_e861_d_n3, eq44_e861_d_n4, eq44_e861_d_n5, eq44_e861_d_n6, eq44_e861_d_n7, eq44_e861_d_n8, eq44_e861_d_n9, eq44_e861_d_n10, eq44_e861_d_n11, eq44_e861_d_n12, eq44_e861_d_n13, eq44_e861_d_n14, eq44_e861_d_n15, eq44_e861_d_n16, eq44_e861_d_n17, eq44_e861_d_n18, eq44_e861_d_n19, eq44_e861_d_n20, eq44_e861_d_n21, eq44_e861_d_n22, eq44_e861_d_n23, eq44_e861_d_n24, eq44_e861_d_n25, eq44_e861_d_n26, eq44_e861_d_n27, eq44_e861_d_n28, eq44_e861_d_n29];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[16]),
            Some(nodes[15]),
            multiplicity * (eq44_value),
            nodes,
            &eq44_node_derivatives,
            branches,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq45_e866,) = {
    if (!s.b[467]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq45_value: f64 = eq45_e866;
        stamper.stamp_potential_const(
            branches[19],
            eq45_value,
        );
        let (eq46_e876, eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29,) = {
    if s.b[613] {
        let eq46_e869: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 20, s.v[203]);
        let eq46_e869_d_n0: f64 = (s.dn[203][0] * ddt_scale);
        let eq46_e869_d_n1: f64 = (s.dn[203][1] * ddt_scale);
        let eq46_e869_d_n2: f64 = (s.dn[203][2] * ddt_scale);
        let eq46_e869_d_n3: f64 = (s.dn[203][3] * ddt_scale);
        let eq46_e869_d_n4: f64 = (s.dn[203][4] * ddt_scale);
        let eq46_e869_d_n5: f64 = (s.dn[203][5] * ddt_scale);
        let eq46_e869_d_n6: f64 = (s.dn[203][6] * ddt_scale);
        let eq46_e869_d_n7: f64 = (s.dn[203][7] * ddt_scale);
        let eq46_e869_d_n8: f64 = (s.dn[203][8] * ddt_scale);
        let eq46_e869_d_n9: f64 = (s.dn[203][9] * ddt_scale);
        let eq46_e869_d_n10: f64 = (s.dn[203][10] * ddt_scale);
        let eq46_e869_d_n11: f64 = (s.dn[203][11] * ddt_scale);
        let eq46_e869_d_n12: f64 = (s.dn[203][12] * ddt_scale);
        let eq46_e869_d_n13: f64 = (s.dn[203][13] * ddt_scale);
        let eq46_e869_d_n14: f64 = (s.dn[203][14] * ddt_scale);
        let eq46_e869_d_n15: f64 = (s.dn[203][15] * ddt_scale);
        let eq46_e869_d_n16: f64 = (s.dn[203][16] * ddt_scale);
        let eq46_e869_d_n17: f64 = (s.dn[203][17] * ddt_scale);
        let eq46_e869_d_n18: f64 = (s.dn[203][18] * ddt_scale);
        let eq46_e869_d_n19: f64 = (s.dn[203][19] * ddt_scale);
        let eq46_e869_d_n20: f64 = (s.dn[203][20] * ddt_scale);
        let eq46_e869_d_n21: f64 = (s.dn[203][21] * ddt_scale);
        let eq46_e869_d_n22: f64 = (s.dn[203][22] * ddt_scale);
        let eq46_e869_d_n23: f64 = (s.dn[203][23] * ddt_scale);
        let eq46_e869_d_n24: f64 = (s.dn[203][24] * ddt_scale);
        let eq46_e869_d_n25: f64 = (s.dn[203][25] * ddt_scale);
        let eq46_e869_d_n26: f64 = (s.dn[203][26] * ddt_scale);
        let eq46_e869_d_n27: f64 = (s.dn[203][27] * ddt_scale);
        let eq46_e869_d_n28: f64 = (s.dn[203][28] * ddt_scale);
        let eq46_e869_d_n29: f64 = (s.dn[203][29] * ddt_scale);
        let eq46_e872: f64 = (p.p355 * (nv7 - nv15));
        let eq46_e872_d_n7: f64 = p.p355;
        let eq46_e872_d_n15: f64 = (-p.p355);
        let eq46_e873: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 21, eq46_e872);
        let eq46_e873_d_n7: f64 = (eq46_e872_d_n7 * ddt_scale);
        let eq46_e873_d_n15: f64 = (eq46_e872_d_n15 * ddt_scale);
        let eq46_e874: f64 = (eq46_e869 + eq46_e873);
        let eq46_e874_d_n7: f64 = (eq46_e869_d_n7 + eq46_e873_d_n7);
        let eq46_e874_d_n15: f64 = (eq46_e869_d_n15 + eq46_e873_d_n15);
        (eq46_e874, eq46_e869_d_n0, eq46_e869_d_n1, eq46_e869_d_n2, eq46_e869_d_n3, eq46_e869_d_n4, eq46_e869_d_n5, eq46_e869_d_n6, eq46_e874_d_n7, eq46_e869_d_n8, eq46_e869_d_n9, eq46_e869_d_n10, eq46_e869_d_n11, eq46_e869_d_n12, eq46_e869_d_n13, eq46_e869_d_n14, eq46_e874_d_n15, eq46_e869_d_n16, eq46_e869_d_n17, eq46_e869_d_n18, eq46_e869_d_n19, eq46_e869_d_n20, eq46_e869_d_n21, eq46_e869_d_n22, eq46_e869_d_n23, eq46_e869_d_n24, eq46_e869_d_n25, eq46_e869_d_n26, eq46_e869_d_n27, eq46_e869_d_n28, eq46_e869_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e876;
        let eq46_node_derivatives: [f64; 30] = [eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29];
        let eq46_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            multiplicity * (eq46_value),
            nodes,
            &eq46_node_derivatives,
            branches,
            &eq46_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq47_e886, eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29,) = {
    if s.b[613] {
        let eq47_e879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 22, s.v[204]);
        let eq47_e879_d_n0: f64 = (s.dn[204][0] * ddt_scale);
        let eq47_e879_d_n1: f64 = (s.dn[204][1] * ddt_scale);
        let eq47_e879_d_n2: f64 = (s.dn[204][2] * ddt_scale);
        let eq47_e879_d_n3: f64 = (s.dn[204][3] * ddt_scale);
        let eq47_e879_d_n4: f64 = (s.dn[204][4] * ddt_scale);
        let eq47_e879_d_n5: f64 = (s.dn[204][5] * ddt_scale);
        let eq47_e879_d_n6: f64 = (s.dn[204][6] * ddt_scale);
        let eq47_e879_d_n7: f64 = (s.dn[204][7] * ddt_scale);
        let eq47_e879_d_n8: f64 = (s.dn[204][8] * ddt_scale);
        let eq47_e879_d_n9: f64 = (s.dn[204][9] * ddt_scale);
        let eq47_e879_d_n10: f64 = (s.dn[204][10] * ddt_scale);
        let eq47_e879_d_n11: f64 = (s.dn[204][11] * ddt_scale);
        let eq47_e879_d_n12: f64 = (s.dn[204][12] * ddt_scale);
        let eq47_e879_d_n13: f64 = (s.dn[204][13] * ddt_scale);
        let eq47_e879_d_n14: f64 = (s.dn[204][14] * ddt_scale);
        let eq47_e879_d_n15: f64 = (s.dn[204][15] * ddt_scale);
        let eq47_e879_d_n16: f64 = (s.dn[204][16] * ddt_scale);
        let eq47_e879_d_n17: f64 = (s.dn[204][17] * ddt_scale);
        let eq47_e879_d_n18: f64 = (s.dn[204][18] * ddt_scale);
        let eq47_e879_d_n19: f64 = (s.dn[204][19] * ddt_scale);
        let eq47_e879_d_n20: f64 = (s.dn[204][20] * ddt_scale);
        let eq47_e879_d_n21: f64 = (s.dn[204][21] * ddt_scale);
        let eq47_e879_d_n22: f64 = (s.dn[204][22] * ddt_scale);
        let eq47_e879_d_n23: f64 = (s.dn[204][23] * ddt_scale);
        let eq47_e879_d_n24: f64 = (s.dn[204][24] * ddt_scale);
        let eq47_e879_d_n25: f64 = (s.dn[204][25] * ddt_scale);
        let eq47_e879_d_n26: f64 = (s.dn[204][26] * ddt_scale);
        let eq47_e879_d_n27: f64 = (s.dn[204][27] * ddt_scale);
        let eq47_e879_d_n28: f64 = (s.dn[204][28] * ddt_scale);
        let eq47_e879_d_n29: f64 = (s.dn[204][29] * ddt_scale);
        let eq47_e882: f64 = (p.p355 * (nv7 - nv16));
        let eq47_e882_d_n7: f64 = p.p355;
        let eq47_e882_d_n16: f64 = (-p.p355);
        let eq47_e883: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 23, eq47_e882);
        let eq47_e883_d_n7: f64 = (eq47_e882_d_n7 * ddt_scale);
        let eq47_e883_d_n16: f64 = (eq47_e882_d_n16 * ddt_scale);
        let eq47_e884: f64 = (eq47_e879 + eq47_e883);
        let eq47_e884_d_n7: f64 = (eq47_e879_d_n7 + eq47_e883_d_n7);
        let eq47_e884_d_n16: f64 = (eq47_e879_d_n16 + eq47_e883_d_n16);
        (eq47_e884, eq47_e879_d_n0, eq47_e879_d_n1, eq47_e879_d_n2, eq47_e879_d_n3, eq47_e879_d_n4, eq47_e879_d_n5, eq47_e879_d_n6, eq47_e884_d_n7, eq47_e879_d_n8, eq47_e879_d_n9, eq47_e879_d_n10, eq47_e879_d_n11, eq47_e879_d_n12, eq47_e879_d_n13, eq47_e879_d_n14, eq47_e879_d_n15, eq47_e884_d_n16, eq47_e879_d_n17, eq47_e879_d_n18, eq47_e879_d_n19, eq47_e879_d_n20, eq47_e879_d_n21, eq47_e879_d_n22, eq47_e879_d_n23, eq47_e879_d_n24, eq47_e879_d_n25, eq47_e879_d_n26, eq47_e879_d_n27, eq47_e879_d_n28, eq47_e879_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e886;
        let eq47_node_derivatives: [f64; 30] = [eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            multiplicity * (eq47_value),
            nodes,
            &eq47_node_derivatives,
            branches,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let (eq48_e896, eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29,) = {
    if s.b[613] {
        let eq48_e889: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 24, s.v[205]);
        let eq48_e889_d_n0: f64 = (s.dn[205][0] * ddt_scale);
        let eq48_e889_d_n1: f64 = (s.dn[205][1] * ddt_scale);
        let eq48_e889_d_n2: f64 = (s.dn[205][2] * ddt_scale);
        let eq48_e889_d_n3: f64 = (s.dn[205][3] * ddt_scale);
        let eq48_e889_d_n4: f64 = (s.dn[205][4] * ddt_scale);
        let eq48_e889_d_n5: f64 = (s.dn[205][5] * ddt_scale);
        let eq48_e889_d_n6: f64 = (s.dn[205][6] * ddt_scale);
        let eq48_e889_d_n7: f64 = (s.dn[205][7] * ddt_scale);
        let eq48_e889_d_n8: f64 = (s.dn[205][8] * ddt_scale);
        let eq48_e889_d_n9: f64 = (s.dn[205][9] * ddt_scale);
        let eq48_e889_d_n10: f64 = (s.dn[205][10] * ddt_scale);
        let eq48_e889_d_n11: f64 = (s.dn[205][11] * ddt_scale);
        let eq48_e889_d_n12: f64 = (s.dn[205][12] * ddt_scale);
        let eq48_e889_d_n13: f64 = (s.dn[205][13] * ddt_scale);
        let eq48_e889_d_n14: f64 = (s.dn[205][14] * ddt_scale);
        let eq48_e889_d_n15: f64 = (s.dn[205][15] * ddt_scale);
        let eq48_e889_d_n16: f64 = (s.dn[205][16] * ddt_scale);
        let eq48_e889_d_n17: f64 = (s.dn[205][17] * ddt_scale);
        let eq48_e889_d_n18: f64 = (s.dn[205][18] * ddt_scale);
        let eq48_e889_d_n19: f64 = (s.dn[205][19] * ddt_scale);
        let eq48_e889_d_n20: f64 = (s.dn[205][20] * ddt_scale);
        let eq48_e889_d_n21: f64 = (s.dn[205][21] * ddt_scale);
        let eq48_e889_d_n22: f64 = (s.dn[205][22] * ddt_scale);
        let eq48_e889_d_n23: f64 = (s.dn[205][23] * ddt_scale);
        let eq48_e889_d_n24: f64 = (s.dn[205][24] * ddt_scale);
        let eq48_e889_d_n25: f64 = (s.dn[205][25] * ddt_scale);
        let eq48_e889_d_n26: f64 = (s.dn[205][26] * ddt_scale);
        let eq48_e889_d_n27: f64 = (s.dn[205][27] * ddt_scale);
        let eq48_e889_d_n28: f64 = (s.dn[205][28] * ddt_scale);
        let eq48_e889_d_n29: f64 = (s.dn[205][29] * ddt_scale);
        let eq48_e892: f64 = (p.p355 * (nv2 - nv15));
        let eq48_e892_d_n2: f64 = p.p355;
        let eq48_e892_d_n15: f64 = (-p.p355);
        let eq48_e893: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 25, eq48_e892);
        let eq48_e893_d_n2: f64 = (eq48_e892_d_n2 * ddt_scale);
        let eq48_e893_d_n15: f64 = (eq48_e892_d_n15 * ddt_scale);
        let eq48_e894: f64 = (eq48_e889 + eq48_e893);
        let eq48_e894_d_n2: f64 = (eq48_e889_d_n2 + eq48_e893_d_n2);
        let eq48_e894_d_n15: f64 = (eq48_e889_d_n15 + eq48_e893_d_n15);
        (eq48_e894, eq48_e889_d_n0, eq48_e889_d_n1, eq48_e894_d_n2, eq48_e889_d_n3, eq48_e889_d_n4, eq48_e889_d_n5, eq48_e889_d_n6, eq48_e889_d_n7, eq48_e889_d_n8, eq48_e889_d_n9, eq48_e889_d_n10, eq48_e889_d_n11, eq48_e889_d_n12, eq48_e889_d_n13, eq48_e889_d_n14, eq48_e894_d_n15, eq48_e889_d_n16, eq48_e889_d_n17, eq48_e889_d_n18, eq48_e889_d_n19, eq48_e889_d_n20, eq48_e889_d_n21, eq48_e889_d_n22, eq48_e889_d_n23, eq48_e889_d_n24, eq48_e889_d_n25, eq48_e889_d_n26, eq48_e889_d_n27, eq48_e889_d_n28, eq48_e889_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e896;
        let eq48_node_derivatives: [f64; 30] = [eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29];
        let eq48_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            multiplicity * (eq48_value),
            nodes,
            &eq48_node_derivatives,
            branches,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq49_e900,) = {
    if s.b[613] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e900;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[16]),
            multiplicity * (eq49_value),
        );
        let (eq50_e910, eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29,) = {
    if s.b[613] {
        let eq50_e903: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 26, s.v[207]);
        let eq50_e903_d_n0: f64 = (s.dn[207][0] * ddt_scale);
        let eq50_e903_d_n1: f64 = (s.dn[207][1] * ddt_scale);
        let eq50_e903_d_n2: f64 = (s.dn[207][2] * ddt_scale);
        let eq50_e903_d_n3: f64 = (s.dn[207][3] * ddt_scale);
        let eq50_e903_d_n4: f64 = (s.dn[207][4] * ddt_scale);
        let eq50_e903_d_n5: f64 = (s.dn[207][5] * ddt_scale);
        let eq50_e903_d_n6: f64 = (s.dn[207][6] * ddt_scale);
        let eq50_e903_d_n7: f64 = (s.dn[207][7] * ddt_scale);
        let eq50_e903_d_n8: f64 = (s.dn[207][8] * ddt_scale);
        let eq50_e903_d_n9: f64 = (s.dn[207][9] * ddt_scale);
        let eq50_e903_d_n10: f64 = (s.dn[207][10] * ddt_scale);
        let eq50_e903_d_n11: f64 = (s.dn[207][11] * ddt_scale);
        let eq50_e903_d_n12: f64 = (s.dn[207][12] * ddt_scale);
        let eq50_e903_d_n13: f64 = (s.dn[207][13] * ddt_scale);
        let eq50_e903_d_n14: f64 = (s.dn[207][14] * ddt_scale);
        let eq50_e903_d_n15: f64 = (s.dn[207][15] * ddt_scale);
        let eq50_e903_d_n16: f64 = (s.dn[207][16] * ddt_scale);
        let eq50_e903_d_n17: f64 = (s.dn[207][17] * ddt_scale);
        let eq50_e903_d_n18: f64 = (s.dn[207][18] * ddt_scale);
        let eq50_e903_d_n19: f64 = (s.dn[207][19] * ddt_scale);
        let eq50_e903_d_n20: f64 = (s.dn[207][20] * ddt_scale);
        let eq50_e903_d_n21: f64 = (s.dn[207][21] * ddt_scale);
        let eq50_e903_d_n22: f64 = (s.dn[207][22] * ddt_scale);
        let eq50_e903_d_n23: f64 = (s.dn[207][23] * ddt_scale);
        let eq50_e903_d_n24: f64 = (s.dn[207][24] * ddt_scale);
        let eq50_e903_d_n25: f64 = (s.dn[207][25] * ddt_scale);
        let eq50_e903_d_n26: f64 = (s.dn[207][26] * ddt_scale);
        let eq50_e903_d_n27: f64 = (s.dn[207][27] * ddt_scale);
        let eq50_e903_d_n28: f64 = (s.dn[207][28] * ddt_scale);
        let eq50_e903_d_n29: f64 = (s.dn[207][29] * ddt_scale);
        let eq50_e906: f64 = (p.p355 * (nv7 - nv9));
        let eq50_e906_d_n7: f64 = p.p355;
        let eq50_e906_d_n9: f64 = (-p.p355);
        let eq50_e907: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 27, eq50_e906);
        let eq50_e907_d_n7: f64 = (eq50_e906_d_n7 * ddt_scale);
        let eq50_e907_d_n9: f64 = (eq50_e906_d_n9 * ddt_scale);
        let eq50_e908: f64 = (eq50_e903 + eq50_e907);
        let eq50_e908_d_n7: f64 = (eq50_e903_d_n7 + eq50_e907_d_n7);
        let eq50_e908_d_n9: f64 = (eq50_e903_d_n9 + eq50_e907_d_n9);
        (eq50_e908, eq50_e903_d_n0, eq50_e903_d_n1, eq50_e903_d_n2, eq50_e903_d_n3, eq50_e903_d_n4, eq50_e903_d_n5, eq50_e903_d_n6, eq50_e908_d_n7, eq50_e903_d_n8, eq50_e908_d_n9, eq50_e903_d_n10, eq50_e903_d_n11, eq50_e903_d_n12, eq50_e903_d_n13, eq50_e903_d_n14, eq50_e903_d_n15, eq50_e903_d_n16, eq50_e903_d_n17, eq50_e903_d_n18, eq50_e903_d_n19, eq50_e903_d_n20, eq50_e903_d_n21, eq50_e903_d_n22, eq50_e903_d_n23, eq50_e903_d_n24, eq50_e903_d_n25, eq50_e903_d_n26, eq50_e903_d_n27, eq50_e903_d_n28, eq50_e903_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e910;
        let eq50_node_derivatives: [f64; 30] = [eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29];
        let eq50_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq50_value),
            nodes,
            &eq50_node_derivatives,
            branches,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e921, eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29,) = {
    if (!s.b[613]) {
        let eq51_e914: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 28, s.v[203]);
        let eq51_e914_d_n0: f64 = (s.dn[203][0] * ddt_scale);
        let eq51_e914_d_n1: f64 = (s.dn[203][1] * ddt_scale);
        let eq51_e914_d_n2: f64 = (s.dn[203][2] * ddt_scale);
        let eq51_e914_d_n3: f64 = (s.dn[203][3] * ddt_scale);
        let eq51_e914_d_n4: f64 = (s.dn[203][4] * ddt_scale);
        let eq51_e914_d_n5: f64 = (s.dn[203][5] * ddt_scale);
        let eq51_e914_d_n6: f64 = (s.dn[203][6] * ddt_scale);
        let eq51_e914_d_n7: f64 = (s.dn[203][7] * ddt_scale);
        let eq51_e914_d_n8: f64 = (s.dn[203][8] * ddt_scale);
        let eq51_e914_d_n9: f64 = (s.dn[203][9] * ddt_scale);
        let eq51_e914_d_n10: f64 = (s.dn[203][10] * ddt_scale);
        let eq51_e914_d_n11: f64 = (s.dn[203][11] * ddt_scale);
        let eq51_e914_d_n12: f64 = (s.dn[203][12] * ddt_scale);
        let eq51_e914_d_n13: f64 = (s.dn[203][13] * ddt_scale);
        let eq51_e914_d_n14: f64 = (s.dn[203][14] * ddt_scale);
        let eq51_e914_d_n15: f64 = (s.dn[203][15] * ddt_scale);
        let eq51_e914_d_n16: f64 = (s.dn[203][16] * ddt_scale);
        let eq51_e914_d_n17: f64 = (s.dn[203][17] * ddt_scale);
        let eq51_e914_d_n18: f64 = (s.dn[203][18] * ddt_scale);
        let eq51_e914_d_n19: f64 = (s.dn[203][19] * ddt_scale);
        let eq51_e914_d_n20: f64 = (s.dn[203][20] * ddt_scale);
        let eq51_e914_d_n21: f64 = (s.dn[203][21] * ddt_scale);
        let eq51_e914_d_n22: f64 = (s.dn[203][22] * ddt_scale);
        let eq51_e914_d_n23: f64 = (s.dn[203][23] * ddt_scale);
        let eq51_e914_d_n24: f64 = (s.dn[203][24] * ddt_scale);
        let eq51_e914_d_n25: f64 = (s.dn[203][25] * ddt_scale);
        let eq51_e914_d_n26: f64 = (s.dn[203][26] * ddt_scale);
        let eq51_e914_d_n27: f64 = (s.dn[203][27] * ddt_scale);
        let eq51_e914_d_n28: f64 = (s.dn[203][28] * ddt_scale);
        let eq51_e914_d_n29: f64 = (s.dn[203][29] * ddt_scale);
        let eq51_e917: f64 = (p.p355 * (nv2 - nv15));
        let eq51_e917_d_n2: f64 = p.p355;
        let eq51_e917_d_n15: f64 = (-p.p355);
        let eq51_e918: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 29, eq51_e917);
        let eq51_e918_d_n2: f64 = (eq51_e917_d_n2 * ddt_scale);
        let eq51_e918_d_n15: f64 = (eq51_e917_d_n15 * ddt_scale);
        let eq51_e919: f64 = (eq51_e914 + eq51_e918);
        let eq51_e919_d_n2: f64 = (eq51_e914_d_n2 + eq51_e918_d_n2);
        let eq51_e919_d_n15: f64 = (eq51_e914_d_n15 + eq51_e918_d_n15);
        (eq51_e919, eq51_e914_d_n0, eq51_e914_d_n1, eq51_e919_d_n2, eq51_e914_d_n3, eq51_e914_d_n4, eq51_e914_d_n5, eq51_e914_d_n6, eq51_e914_d_n7, eq51_e914_d_n8, eq51_e914_d_n9, eq51_e914_d_n10, eq51_e914_d_n11, eq51_e914_d_n12, eq51_e914_d_n13, eq51_e914_d_n14, eq51_e919_d_n15, eq51_e914_d_n16, eq51_e914_d_n17, eq51_e914_d_n18, eq51_e914_d_n19, eq51_e914_d_n20, eq51_e914_d_n21, eq51_e914_d_n22, eq51_e914_d_n23, eq51_e914_d_n24, eq51_e914_d_n25, eq51_e914_d_n26, eq51_e914_d_n27, eq51_e914_d_n28, eq51_e914_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e921;
        let eq51_node_derivatives: [f64; 30] = [eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            multiplicity * (eq51_value),
            nodes,
            &eq51_node_derivatives,
            branches,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e932, eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22, eq52_e932_d_n23, eq52_e932_d_n24, eq52_e932_d_n25, eq52_e932_d_n26, eq52_e932_d_n27, eq52_e932_d_n28, eq52_e932_d_n29,) = {
    if (!s.b[613]) {
        let eq52_e925: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 30, s.v[204]);
        let eq52_e925_d_n0: f64 = (s.dn[204][0] * ddt_scale);
        let eq52_e925_d_n1: f64 = (s.dn[204][1] * ddt_scale);
        let eq52_e925_d_n2: f64 = (s.dn[204][2] * ddt_scale);
        let eq52_e925_d_n3: f64 = (s.dn[204][3] * ddt_scale);
        let eq52_e925_d_n4: f64 = (s.dn[204][4] * ddt_scale);
        let eq52_e925_d_n5: f64 = (s.dn[204][5] * ddt_scale);
        let eq52_e925_d_n6: f64 = (s.dn[204][6] * ddt_scale);
        let eq52_e925_d_n7: f64 = (s.dn[204][7] * ddt_scale);
        let eq52_e925_d_n8: f64 = (s.dn[204][8] * ddt_scale);
        let eq52_e925_d_n9: f64 = (s.dn[204][9] * ddt_scale);
        let eq52_e925_d_n10: f64 = (s.dn[204][10] * ddt_scale);
        let eq52_e925_d_n11: f64 = (s.dn[204][11] * ddt_scale);
        let eq52_e925_d_n12: f64 = (s.dn[204][12] * ddt_scale);
        let eq52_e925_d_n13: f64 = (s.dn[204][13] * ddt_scale);
        let eq52_e925_d_n14: f64 = (s.dn[204][14] * ddt_scale);
        let eq52_e925_d_n15: f64 = (s.dn[204][15] * ddt_scale);
        let eq52_e925_d_n16: f64 = (s.dn[204][16] * ddt_scale);
        let eq52_e925_d_n17: f64 = (s.dn[204][17] * ddt_scale);
        let eq52_e925_d_n18: f64 = (s.dn[204][18] * ddt_scale);
        let eq52_e925_d_n19: f64 = (s.dn[204][19] * ddt_scale);
        let eq52_e925_d_n20: f64 = (s.dn[204][20] * ddt_scale);
        let eq52_e925_d_n21: f64 = (s.dn[204][21] * ddt_scale);
        let eq52_e925_d_n22: f64 = (s.dn[204][22] * ddt_scale);
        let eq52_e925_d_n23: f64 = (s.dn[204][23] * ddt_scale);
        let eq52_e925_d_n24: f64 = (s.dn[204][24] * ddt_scale);
        let eq52_e925_d_n25: f64 = (s.dn[204][25] * ddt_scale);
        let eq52_e925_d_n26: f64 = (s.dn[204][26] * ddt_scale);
        let eq52_e925_d_n27: f64 = (s.dn[204][27] * ddt_scale);
        let eq52_e925_d_n28: f64 = (s.dn[204][28] * ddt_scale);
        let eq52_e925_d_n29: f64 = (s.dn[204][29] * ddt_scale);
        let eq52_e928: f64 = (p.p355 * (nv2 - nv16));
        let eq52_e928_d_n2: f64 = p.p355;
        let eq52_e928_d_n16: f64 = (-p.p355);
        let eq52_e929: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 31, eq52_e928);
        let eq52_e929_d_n2: f64 = (eq52_e928_d_n2 * ddt_scale);
        let eq52_e929_d_n16: f64 = (eq52_e928_d_n16 * ddt_scale);
        let eq52_e930: f64 = (eq52_e925 + eq52_e929);
        let eq52_e930_d_n2: f64 = (eq52_e925_d_n2 + eq52_e929_d_n2);
        let eq52_e930_d_n16: f64 = (eq52_e925_d_n16 + eq52_e929_d_n16);
        (eq52_e930, eq52_e925_d_n0, eq52_e925_d_n1, eq52_e930_d_n2, eq52_e925_d_n3, eq52_e925_d_n4, eq52_e925_d_n5, eq52_e925_d_n6, eq52_e925_d_n7, eq52_e925_d_n8, eq52_e925_d_n9, eq52_e925_d_n10, eq52_e925_d_n11, eq52_e925_d_n12, eq52_e925_d_n13, eq52_e925_d_n14, eq52_e925_d_n15, eq52_e930_d_n16, eq52_e925_d_n17, eq52_e925_d_n18, eq52_e925_d_n19, eq52_e925_d_n20, eq52_e925_d_n21, eq52_e925_d_n22, eq52_e925_d_n23, eq52_e925_d_n24, eq52_e925_d_n25, eq52_e925_d_n26, eq52_e925_d_n27, eq52_e925_d_n28, eq52_e925_d_n29,)
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
            multiplicity * (eq52_value),
            nodes,
            &eq52_node_derivatives,
            branches,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e943, eq53_e943_d_n0, eq53_e943_d_n1, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n5, eq53_e943_d_n6, eq53_e943_d_n7, eq53_e943_d_n8, eq53_e943_d_n9, eq53_e943_d_n10, eq53_e943_d_n11, eq53_e943_d_n12, eq53_e943_d_n13, eq53_e943_d_n14, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_d_n17, eq53_e943_d_n18, eq53_e943_d_n19, eq53_e943_d_n20, eq53_e943_d_n21, eq53_e943_d_n22, eq53_e943_d_n23, eq53_e943_d_n24, eq53_e943_d_n25, eq53_e943_d_n26, eq53_e943_d_n27, eq53_e943_d_n28, eq53_e943_d_n29,) = {
    if (!s.b[613]) {
        let eq53_e936: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 32, s.v[205]);
        let eq53_e936_d_n0: f64 = (s.dn[205][0] * ddt_scale);
        let eq53_e936_d_n1: f64 = (s.dn[205][1] * ddt_scale);
        let eq53_e936_d_n2: f64 = (s.dn[205][2] * ddt_scale);
        let eq53_e936_d_n3: f64 = (s.dn[205][3] * ddt_scale);
        let eq53_e936_d_n4: f64 = (s.dn[205][4] * ddt_scale);
        let eq53_e936_d_n5: f64 = (s.dn[205][5] * ddt_scale);
        let eq53_e936_d_n6: f64 = (s.dn[205][6] * ddt_scale);
        let eq53_e936_d_n7: f64 = (s.dn[205][7] * ddt_scale);
        let eq53_e936_d_n8: f64 = (s.dn[205][8] * ddt_scale);
        let eq53_e936_d_n9: f64 = (s.dn[205][9] * ddt_scale);
        let eq53_e936_d_n10: f64 = (s.dn[205][10] * ddt_scale);
        let eq53_e936_d_n11: f64 = (s.dn[205][11] * ddt_scale);
        let eq53_e936_d_n12: f64 = (s.dn[205][12] * ddt_scale);
        let eq53_e936_d_n13: f64 = (s.dn[205][13] * ddt_scale);
        let eq53_e936_d_n14: f64 = (s.dn[205][14] * ddt_scale);
        let eq53_e936_d_n15: f64 = (s.dn[205][15] * ddt_scale);
        let eq53_e936_d_n16: f64 = (s.dn[205][16] * ddt_scale);
        let eq53_e936_d_n17: f64 = (s.dn[205][17] * ddt_scale);
        let eq53_e936_d_n18: f64 = (s.dn[205][18] * ddt_scale);
        let eq53_e936_d_n19: f64 = (s.dn[205][19] * ddt_scale);
        let eq53_e936_d_n20: f64 = (s.dn[205][20] * ddt_scale);
        let eq53_e936_d_n21: f64 = (s.dn[205][21] * ddt_scale);
        let eq53_e936_d_n22: f64 = (s.dn[205][22] * ddt_scale);
        let eq53_e936_d_n23: f64 = (s.dn[205][23] * ddt_scale);
        let eq53_e936_d_n24: f64 = (s.dn[205][24] * ddt_scale);
        let eq53_e936_d_n25: f64 = (s.dn[205][25] * ddt_scale);
        let eq53_e936_d_n26: f64 = (s.dn[205][26] * ddt_scale);
        let eq53_e936_d_n27: f64 = (s.dn[205][27] * ddt_scale);
        let eq53_e936_d_n28: f64 = (s.dn[205][28] * ddt_scale);
        let eq53_e936_d_n29: f64 = (s.dn[205][29] * ddt_scale);
        let eq53_e939: f64 = (p.p355 * (nv7 - nv15));
        let eq53_e939_d_n7: f64 = p.p355;
        let eq53_e939_d_n15: f64 = (-p.p355);
        let eq53_e940: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 33, eq53_e939);
        let eq53_e940_d_n7: f64 = (eq53_e939_d_n7 * ddt_scale);
        let eq53_e940_d_n15: f64 = (eq53_e939_d_n15 * ddt_scale);
        let eq53_e941: f64 = (eq53_e936 + eq53_e940);
        let eq53_e941_d_n7: f64 = (eq53_e936_d_n7 + eq53_e940_d_n7);
        let eq53_e941_d_n15: f64 = (eq53_e936_d_n15 + eq53_e940_d_n15);
        (eq53_e941, eq53_e936_d_n0, eq53_e936_d_n1, eq53_e936_d_n2, eq53_e936_d_n3, eq53_e936_d_n4, eq53_e936_d_n5, eq53_e936_d_n6, eq53_e941_d_n7, eq53_e936_d_n8, eq53_e936_d_n9, eq53_e936_d_n10, eq53_e936_d_n11, eq53_e936_d_n12, eq53_e936_d_n13, eq53_e936_d_n14, eq53_e941_d_n15, eq53_e936_d_n16, eq53_e936_d_n17, eq53_e936_d_n18, eq53_e936_d_n19, eq53_e936_d_n20, eq53_e936_d_n21, eq53_e936_d_n22, eq53_e936_d_n23, eq53_e936_d_n24, eq53_e936_d_n25, eq53_e936_d_n26, eq53_e936_d_n27, eq53_e936_d_n28, eq53_e936_d_n29,)
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
            multiplicity * (eq53_value),
            nodes,
            &eq53_node_derivatives,
            branches,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e948,) = {
    if (!s.b[613]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e948;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[16]),
            multiplicity * (eq54_value),
        );
        let (eq55_e953,) = {
    if (!s.b[613]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e953;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq55_value),
        );
        let eq56_e955: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 34, s.v[206]);
        let eq56_e955_d_n0: f64 = (s.dn[206][0] * ddt_scale);
        let eq56_e955_d_n1: f64 = (s.dn[206][1] * ddt_scale);
        let eq56_e955_d_n2: f64 = (s.dn[206][2] * ddt_scale);
        let eq56_e955_d_n3: f64 = (s.dn[206][3] * ddt_scale);
        let eq56_e955_d_n4: f64 = (s.dn[206][4] * ddt_scale);
        let eq56_e955_d_n5: f64 = (s.dn[206][5] * ddt_scale);
        let eq56_e955_d_n6: f64 = (s.dn[206][6] * ddt_scale);
        let eq56_e955_d_n7: f64 = (s.dn[206][7] * ddt_scale);
        let eq56_e955_d_n8: f64 = (s.dn[206][8] * ddt_scale);
        let eq56_e955_d_n9: f64 = (s.dn[206][9] * ddt_scale);
        let eq56_e955_d_n10: f64 = (s.dn[206][10] * ddt_scale);
        let eq56_e955_d_n11: f64 = (s.dn[206][11] * ddt_scale);
        let eq56_e955_d_n12: f64 = (s.dn[206][12] * ddt_scale);
        let eq56_e955_d_n13: f64 = (s.dn[206][13] * ddt_scale);
        let eq56_e955_d_n14: f64 = (s.dn[206][14] * ddt_scale);
        let eq56_e955_d_n15: f64 = (s.dn[206][15] * ddt_scale);
        let eq56_e955_d_n16: f64 = (s.dn[206][16] * ddt_scale);
        let eq56_e955_d_n17: f64 = (s.dn[206][17] * ddt_scale);
        let eq56_e955_d_n18: f64 = (s.dn[206][18] * ddt_scale);
        let eq56_e955_d_n19: f64 = (s.dn[206][19] * ddt_scale);
        let eq56_e955_d_n20: f64 = (s.dn[206][20] * ddt_scale);
        let eq56_e955_d_n21: f64 = (s.dn[206][21] * ddt_scale);
        let eq56_e955_d_n22: f64 = (s.dn[206][22] * ddt_scale);
        let eq56_e955_d_n23: f64 = (s.dn[206][23] * ddt_scale);
        let eq56_e955_d_n24: f64 = (s.dn[206][24] * ddt_scale);
        let eq56_e955_d_n25: f64 = (s.dn[206][25] * ddt_scale);
        let eq56_e955_d_n26: f64 = (s.dn[206][26] * ddt_scale);
        let eq56_e955_d_n27: f64 = (s.dn[206][27] * ddt_scale);
        let eq56_e955_d_n28: f64 = (s.dn[206][28] * ddt_scale);
        let eq56_e955_d_n29: f64 = (s.dn[206][29] * ddt_scale);
        let eq56_e958: f64 = (p.p355 * (nv3 - nv15));
        let eq56_e958_d_n3: f64 = p.p355;
        let eq56_e958_d_n15: f64 = (-p.p355);
        let eq56_e959: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 35, eq56_e958);
        let eq56_e959_d_n3: f64 = (eq56_e958_d_n3 * ddt_scale);
        let eq56_e959_d_n15: f64 = (eq56_e958_d_n15 * ddt_scale);
        let eq56_e960: f64 = (eq56_e955 + eq56_e959);
        let eq56_e960_d_n3: f64 = (eq56_e955_d_n3 + eq56_e959_d_n3);
        let eq56_e960_d_n15: f64 = (eq56_e955_d_n15 + eq56_e959_d_n15);
        let eq56_value: f64 = eq56_e960;
        let eq56_node_derivatives: [f64; 30] = [eq56_e955_d_n0, eq56_e955_d_n1, eq56_e955_d_n2, eq56_e960_d_n3, eq56_e955_d_n4, eq56_e955_d_n5, eq56_e955_d_n6, eq56_e955_d_n7, eq56_e955_d_n8, eq56_e955_d_n9, eq56_e955_d_n10, eq56_e955_d_n11, eq56_e955_d_n12, eq56_e955_d_n13, eq56_e955_d_n14, eq56_e960_d_n15, eq56_e955_d_n16, eq56_e955_d_n17, eq56_e955_d_n18, eq56_e955_d_n19, eq56_e955_d_n20, eq56_e955_d_n21, eq56_e955_d_n22, eq56_e955_d_n23, eq56_e955_d_n24, eq56_e955_d_n25, eq56_e955_d_n26, eq56_e955_d_n27, eq56_e955_d_n28, eq56_e955_d_n29];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[15]),
            multiplicity * (eq56_value),
            nodes,
            &eq56_node_derivatives,
            branches,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e968, eq57_e968_d_n0, eq57_e968_d_n1, eq57_e968_d_n2, eq57_e968_d_n3, eq57_e968_d_n4, eq57_e968_d_n5, eq57_e968_d_n6, eq57_e968_d_n7, eq57_e968_d_n8, eq57_e968_d_n9, eq57_e968_d_n10, eq57_e968_d_n11, eq57_e968_d_n12, eq57_e968_d_n13, eq57_e968_d_n14, eq57_e968_d_n15, eq57_e968_d_n16, eq57_e968_d_n17, eq57_e968_d_n18, eq57_e968_d_n19, eq57_e968_d_n20, eq57_e968_d_n21, eq57_e968_d_n22, eq57_e968_d_n23, eq57_e968_d_n24, eq57_e968_d_n25, eq57_e968_d_n26, eq57_e968_d_n27, eq57_e968_d_n28, eq57_e968_d_n29,) = {
    if s.b[614] {
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
            multiplicity * (eq57_value),
            nodes,
            &eq57_node_derivatives,
            branches,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e973,) = {
    if (!s.b[614]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e973;
        stamper.stamp_potential_const(
            branches[20],
            eq58_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq59_e983, eq59_e983_d_n0, eq59_e983_d_n1, eq59_e983_d_n2, eq59_e983_d_n3, eq59_e983_d_n4, eq59_e983_d_n5, eq59_e983_d_n6, eq59_e983_d_n7, eq59_e983_d_n8, eq59_e983_d_n9, eq59_e983_d_n10, eq59_e983_d_n11, eq59_e983_d_n12, eq59_e983_d_n13, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_d_n16, eq59_e983_d_n17, eq59_e983_d_n18, eq59_e983_d_n19, eq59_e983_d_n20, eq59_e983_d_n21, eq59_e983_d_n22, eq59_e983_d_n23, eq59_e983_d_n24, eq59_e983_d_n25, eq59_e983_d_n26, eq59_e983_d_n27, eq59_e983_d_n28, eq59_e983_d_n29,) = {
    if s.b[760] {
        let eq59_e976: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 36, s.v[197]);
        let eq59_e976_d_n0: f64 = (s.dn[197][0] * ddt_scale);
        let eq59_e976_d_n1: f64 = (s.dn[197][1] * ddt_scale);
        let eq59_e976_d_n2: f64 = (s.dn[197][2] * ddt_scale);
        let eq59_e976_d_n3: f64 = (s.dn[197][3] * ddt_scale);
        let eq59_e976_d_n4: f64 = (s.dn[197][4] * ddt_scale);
        let eq59_e976_d_n5: f64 = (s.dn[197][5] * ddt_scale);
        let eq59_e976_d_n6: f64 = (s.dn[197][6] * ddt_scale);
        let eq59_e976_d_n7: f64 = (s.dn[197][7] * ddt_scale);
        let eq59_e976_d_n8: f64 = (s.dn[197][8] * ddt_scale);
        let eq59_e976_d_n9: f64 = (s.dn[197][9] * ddt_scale);
        let eq59_e976_d_n10: f64 = (s.dn[197][10] * ddt_scale);
        let eq59_e976_d_n11: f64 = (s.dn[197][11] * ddt_scale);
        let eq59_e976_d_n12: f64 = (s.dn[197][12] * ddt_scale);
        let eq59_e976_d_n13: f64 = (s.dn[197][13] * ddt_scale);
        let eq59_e976_d_n14: f64 = (s.dn[197][14] * ddt_scale);
        let eq59_e976_d_n15: f64 = (s.dn[197][15] * ddt_scale);
        let eq59_e976_d_n16: f64 = (s.dn[197][16] * ddt_scale);
        let eq59_e976_d_n17: f64 = (s.dn[197][17] * ddt_scale);
        let eq59_e976_d_n18: f64 = (s.dn[197][18] * ddt_scale);
        let eq59_e976_d_n19: f64 = (s.dn[197][19] * ddt_scale);
        let eq59_e976_d_n20: f64 = (s.dn[197][20] * ddt_scale);
        let eq59_e976_d_n21: f64 = (s.dn[197][21] * ddt_scale);
        let eq59_e976_d_n22: f64 = (s.dn[197][22] * ddt_scale);
        let eq59_e976_d_n23: f64 = (s.dn[197][23] * ddt_scale);
        let eq59_e976_d_n24: f64 = (s.dn[197][24] * ddt_scale);
        let eq59_e976_d_n25: f64 = (s.dn[197][25] * ddt_scale);
        let eq59_e976_d_n26: f64 = (s.dn[197][26] * ddt_scale);
        let eq59_e976_d_n27: f64 = (s.dn[197][27] * ddt_scale);
        let eq59_e976_d_n28: f64 = (s.dn[197][28] * ddt_scale);
        let eq59_e976_d_n29: f64 = (s.dn[197][29] * ddt_scale);
        let eq59_e979: f64 = (p.p355 * (nv7 - nv14));
        let eq59_e979_d_n7: f64 = p.p355;
        let eq59_e979_d_n14: f64 = (-p.p355);
        let eq59_e980: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 37, eq59_e979);
        let eq59_e980_d_n7: f64 = (eq59_e979_d_n7 * ddt_scale);
        let eq59_e980_d_n14: f64 = (eq59_e979_d_n14 * ddt_scale);
        let eq59_e981: f64 = (eq59_e976 + eq59_e980);
        let eq59_e981_d_n7: f64 = (eq59_e976_d_n7 + eq59_e980_d_n7);
        let eq59_e981_d_n14: f64 = (eq59_e976_d_n14 + eq59_e980_d_n14);
        (eq59_e981, eq59_e976_d_n0, eq59_e976_d_n1, eq59_e976_d_n2, eq59_e976_d_n3, eq59_e976_d_n4, eq59_e976_d_n5, eq59_e976_d_n6, eq59_e981_d_n7, eq59_e976_d_n8, eq59_e976_d_n9, eq59_e976_d_n10, eq59_e976_d_n11, eq59_e976_d_n12, eq59_e976_d_n13, eq59_e981_d_n14, eq59_e976_d_n15, eq59_e976_d_n16, eq59_e976_d_n17, eq59_e976_d_n18, eq59_e976_d_n19, eq59_e976_d_n20, eq59_e976_d_n21, eq59_e976_d_n22, eq59_e976_d_n23, eq59_e976_d_n24, eq59_e976_d_n25, eq59_e976_d_n26, eq59_e976_d_n27, eq59_e976_d_n28, eq59_e976_d_n29,)
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
            multiplicity * (eq59_value),
            nodes,
            &eq59_node_derivatives,
            branches,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let (eq60_e993, eq60_e993_d_n0, eq60_e993_d_n1, eq60_e993_d_n2, eq60_e993_d_n3, eq60_e993_d_n4, eq60_e993_d_n5, eq60_e993_d_n6, eq60_e993_d_n7, eq60_e993_d_n8, eq60_e993_d_n9, eq60_e993_d_n10, eq60_e993_d_n11, eq60_e993_d_n12, eq60_e993_d_n13, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_d_n16, eq60_e993_d_n17, eq60_e993_d_n18, eq60_e993_d_n19, eq60_e993_d_n20, eq60_e993_d_n21, eq60_e993_d_n22, eq60_e993_d_n23, eq60_e993_d_n24, eq60_e993_d_n25, eq60_e993_d_n26, eq60_e993_d_n27, eq60_e993_d_n28, eq60_e993_d_n29,) = {
    if s.b[760] {
        let eq60_e986: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 38, s.v[198]);
        let eq60_e986_d_n0: f64 = (s.dn[198][0] * ddt_scale);
        let eq60_e986_d_n1: f64 = (s.dn[198][1] * ddt_scale);
        let eq60_e986_d_n2: f64 = (s.dn[198][2] * ddt_scale);
        let eq60_e986_d_n3: f64 = (s.dn[198][3] * ddt_scale);
        let eq60_e986_d_n4: f64 = (s.dn[198][4] * ddt_scale);
        let eq60_e986_d_n5: f64 = (s.dn[198][5] * ddt_scale);
        let eq60_e986_d_n6: f64 = (s.dn[198][6] * ddt_scale);
        let eq60_e986_d_n7: f64 = (s.dn[198][7] * ddt_scale);
        let eq60_e986_d_n8: f64 = (s.dn[198][8] * ddt_scale);
        let eq60_e986_d_n9: f64 = (s.dn[198][9] * ddt_scale);
        let eq60_e986_d_n10: f64 = (s.dn[198][10] * ddt_scale);
        let eq60_e986_d_n11: f64 = (s.dn[198][11] * ddt_scale);
        let eq60_e986_d_n12: f64 = (s.dn[198][12] * ddt_scale);
        let eq60_e986_d_n13: f64 = (s.dn[198][13] * ddt_scale);
        let eq60_e986_d_n14: f64 = (s.dn[198][14] * ddt_scale);
        let eq60_e986_d_n15: f64 = (s.dn[198][15] * ddt_scale);
        let eq60_e986_d_n16: f64 = (s.dn[198][16] * ddt_scale);
        let eq60_e986_d_n17: f64 = (s.dn[198][17] * ddt_scale);
        let eq60_e986_d_n18: f64 = (s.dn[198][18] * ddt_scale);
        let eq60_e986_d_n19: f64 = (s.dn[198][19] * ddt_scale);
        let eq60_e986_d_n20: f64 = (s.dn[198][20] * ddt_scale);
        let eq60_e986_d_n21: f64 = (s.dn[198][21] * ddt_scale);
        let eq60_e986_d_n22: f64 = (s.dn[198][22] * ddt_scale);
        let eq60_e986_d_n23: f64 = (s.dn[198][23] * ddt_scale);
        let eq60_e986_d_n24: f64 = (s.dn[198][24] * ddt_scale);
        let eq60_e986_d_n25: f64 = (s.dn[198][25] * ddt_scale);
        let eq60_e986_d_n26: f64 = (s.dn[198][26] * ddt_scale);
        let eq60_e986_d_n27: f64 = (s.dn[198][27] * ddt_scale);
        let eq60_e986_d_n28: f64 = (s.dn[198][28] * ddt_scale);
        let eq60_e986_d_n29: f64 = (s.dn[198][29] * ddt_scale);
        let eq60_e989: f64 = (p.p355 * (nv7 - nv15));
        let eq60_e989_d_n7: f64 = p.p355;
        let eq60_e989_d_n15: f64 = (-p.p355);
        let eq60_e990: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 39, eq60_e989);
        let eq60_e990_d_n7: f64 = (eq60_e989_d_n7 * ddt_scale);
        let eq60_e990_d_n15: f64 = (eq60_e989_d_n15 * ddt_scale);
        let eq60_e991: f64 = (eq60_e986 + eq60_e990);
        let eq60_e991_d_n7: f64 = (eq60_e986_d_n7 + eq60_e990_d_n7);
        let eq60_e991_d_n15: f64 = (eq60_e986_d_n15 + eq60_e990_d_n15);
        (eq60_e991, eq60_e986_d_n0, eq60_e986_d_n1, eq60_e986_d_n2, eq60_e986_d_n3, eq60_e986_d_n4, eq60_e986_d_n5, eq60_e986_d_n6, eq60_e991_d_n7, eq60_e986_d_n8, eq60_e986_d_n9, eq60_e986_d_n10, eq60_e986_d_n11, eq60_e986_d_n12, eq60_e986_d_n13, eq60_e986_d_n14, eq60_e991_d_n15, eq60_e986_d_n16, eq60_e986_d_n17, eq60_e986_d_n18, eq60_e986_d_n19, eq60_e986_d_n20, eq60_e986_d_n21, eq60_e986_d_n22, eq60_e986_d_n23, eq60_e986_d_n24, eq60_e986_d_n25, eq60_e986_d_n26, eq60_e986_d_n27, eq60_e986_d_n28, eq60_e986_d_n29,)
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
            multiplicity * (eq60_value),
            nodes,
            &eq60_node_derivatives,
            branches,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1003, eq61_e1003_d_n0, eq61_e1003_d_n1, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n5, eq61_e1003_d_n6, eq61_e1003_d_n7, eq61_e1003_d_n8, eq61_e1003_d_n9, eq61_e1003_d_n10, eq61_e1003_d_n11, eq61_e1003_d_n12, eq61_e1003_d_n13, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_d_n16, eq61_e1003_d_n17, eq61_e1003_d_n18, eq61_e1003_d_n19, eq61_e1003_d_n20, eq61_e1003_d_n21, eq61_e1003_d_n22, eq61_e1003_d_n23, eq61_e1003_d_n24, eq61_e1003_d_n25, eq61_e1003_d_n26, eq61_e1003_d_n27, eq61_e1003_d_n28, eq61_e1003_d_n29,) = {
    if s.b[760] {
        let eq61_e996: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 40, s.v[199]);
        let eq61_e996_d_n0: f64 = (s.dn[199][0] * ddt_scale);
        let eq61_e996_d_n1: f64 = (s.dn[199][1] * ddt_scale);
        let eq61_e996_d_n2: f64 = (s.dn[199][2] * ddt_scale);
        let eq61_e996_d_n3: f64 = (s.dn[199][3] * ddt_scale);
        let eq61_e996_d_n4: f64 = (s.dn[199][4] * ddt_scale);
        let eq61_e996_d_n5: f64 = (s.dn[199][5] * ddt_scale);
        let eq61_e996_d_n6: f64 = (s.dn[199][6] * ddt_scale);
        let eq61_e996_d_n7: f64 = (s.dn[199][7] * ddt_scale);
        let eq61_e996_d_n8: f64 = (s.dn[199][8] * ddt_scale);
        let eq61_e996_d_n9: f64 = (s.dn[199][9] * ddt_scale);
        let eq61_e996_d_n10: f64 = (s.dn[199][10] * ddt_scale);
        let eq61_e996_d_n11: f64 = (s.dn[199][11] * ddt_scale);
        let eq61_e996_d_n12: f64 = (s.dn[199][12] * ddt_scale);
        let eq61_e996_d_n13: f64 = (s.dn[199][13] * ddt_scale);
        let eq61_e996_d_n14: f64 = (s.dn[199][14] * ddt_scale);
        let eq61_e996_d_n15: f64 = (s.dn[199][15] * ddt_scale);
        let eq61_e996_d_n16: f64 = (s.dn[199][16] * ddt_scale);
        let eq61_e996_d_n17: f64 = (s.dn[199][17] * ddt_scale);
        let eq61_e996_d_n18: f64 = (s.dn[199][18] * ddt_scale);
        let eq61_e996_d_n19: f64 = (s.dn[199][19] * ddt_scale);
        let eq61_e996_d_n20: f64 = (s.dn[199][20] * ddt_scale);
        let eq61_e996_d_n21: f64 = (s.dn[199][21] * ddt_scale);
        let eq61_e996_d_n22: f64 = (s.dn[199][22] * ddt_scale);
        let eq61_e996_d_n23: f64 = (s.dn[199][23] * ddt_scale);
        let eq61_e996_d_n24: f64 = (s.dn[199][24] * ddt_scale);
        let eq61_e996_d_n25: f64 = (s.dn[199][25] * ddt_scale);
        let eq61_e996_d_n26: f64 = (s.dn[199][26] * ddt_scale);
        let eq61_e996_d_n27: f64 = (s.dn[199][27] * ddt_scale);
        let eq61_e996_d_n28: f64 = (s.dn[199][28] * ddt_scale);
        let eq61_e996_d_n29: f64 = (s.dn[199][29] * ddt_scale);
        let eq61_e999: f64 = (p.p355 * (nv2 - nv14));
        let eq61_e999_d_n2: f64 = p.p355;
        let eq61_e999_d_n14: f64 = (-p.p355);
        let eq61_e1000: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 41, eq61_e999);
        let eq61_e1000_d_n2: f64 = (eq61_e999_d_n2 * ddt_scale);
        let eq61_e1000_d_n14: f64 = (eq61_e999_d_n14 * ddt_scale);
        let eq61_e1001: f64 = (eq61_e996 + eq61_e1000);
        let eq61_e1001_d_n2: f64 = (eq61_e996_d_n2 + eq61_e1000_d_n2);
        let eq61_e1001_d_n14: f64 = (eq61_e996_d_n14 + eq61_e1000_d_n14);
        (eq61_e1001, eq61_e996_d_n0, eq61_e996_d_n1, eq61_e1001_d_n2, eq61_e996_d_n3, eq61_e996_d_n4, eq61_e996_d_n5, eq61_e996_d_n6, eq61_e996_d_n7, eq61_e996_d_n8, eq61_e996_d_n9, eq61_e996_d_n10, eq61_e996_d_n11, eq61_e996_d_n12, eq61_e996_d_n13, eq61_e1001_d_n14, eq61_e996_d_n15, eq61_e996_d_n16, eq61_e996_d_n17, eq61_e996_d_n18, eq61_e996_d_n19, eq61_e996_d_n20, eq61_e996_d_n21, eq61_e996_d_n22, eq61_e996_d_n23, eq61_e996_d_n24, eq61_e996_d_n25, eq61_e996_d_n26, eq61_e996_d_n27, eq61_e996_d_n28, eq61_e996_d_n29,)
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
            multiplicity * (eq61_value),
            nodes,
            &eq61_node_derivatives,
            branches,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1007,) = {
    if s.b[760] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq62_value: f64 = eq62_e1007;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[15]),
            multiplicity * (eq62_value),
        );
        let (eq63_e1017, eq63_e1017_d_n0, eq63_e1017_d_n1, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n5, eq63_e1017_d_n6, eq63_e1017_d_n7, eq63_e1017_d_n8, eq63_e1017_d_n9, eq63_e1017_d_n10, eq63_e1017_d_n11, eq63_e1017_d_n12, eq63_e1017_d_n13, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_d_n16, eq63_e1017_d_n17, eq63_e1017_d_n18, eq63_e1017_d_n19, eq63_e1017_d_n20, eq63_e1017_d_n21, eq63_e1017_d_n22, eq63_e1017_d_n23, eq63_e1017_d_n24, eq63_e1017_d_n25, eq63_e1017_d_n26, eq63_e1017_d_n27, eq63_e1017_d_n28, eq63_e1017_d_n29,) = {
    if s.b[760] {
        let eq63_e1010: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 42, s.v[201]);
        let eq63_e1010_d_n0: f64 = (s.dn[201][0] * ddt_scale);
        let eq63_e1010_d_n1: f64 = (s.dn[201][1] * ddt_scale);
        let eq63_e1010_d_n2: f64 = (s.dn[201][2] * ddt_scale);
        let eq63_e1010_d_n3: f64 = (s.dn[201][3] * ddt_scale);
        let eq63_e1010_d_n4: f64 = (s.dn[201][4] * ddt_scale);
        let eq63_e1010_d_n5: f64 = (s.dn[201][5] * ddt_scale);
        let eq63_e1010_d_n6: f64 = (s.dn[201][6] * ddt_scale);
        let eq63_e1010_d_n7: f64 = (s.dn[201][7] * ddt_scale);
        let eq63_e1010_d_n8: f64 = (s.dn[201][8] * ddt_scale);
        let eq63_e1010_d_n9: f64 = (s.dn[201][9] * ddt_scale);
        let eq63_e1010_d_n10: f64 = (s.dn[201][10] * ddt_scale);
        let eq63_e1010_d_n11: f64 = (s.dn[201][11] * ddt_scale);
        let eq63_e1010_d_n12: f64 = (s.dn[201][12] * ddt_scale);
        let eq63_e1010_d_n13: f64 = (s.dn[201][13] * ddt_scale);
        let eq63_e1010_d_n14: f64 = (s.dn[201][14] * ddt_scale);
        let eq63_e1010_d_n15: f64 = (s.dn[201][15] * ddt_scale);
        let eq63_e1010_d_n16: f64 = (s.dn[201][16] * ddt_scale);
        let eq63_e1010_d_n17: f64 = (s.dn[201][17] * ddt_scale);
        let eq63_e1010_d_n18: f64 = (s.dn[201][18] * ddt_scale);
        let eq63_e1010_d_n19: f64 = (s.dn[201][19] * ddt_scale);
        let eq63_e1010_d_n20: f64 = (s.dn[201][20] * ddt_scale);
        let eq63_e1010_d_n21: f64 = (s.dn[201][21] * ddt_scale);
        let eq63_e1010_d_n22: f64 = (s.dn[201][22] * ddt_scale);
        let eq63_e1010_d_n23: f64 = (s.dn[201][23] * ddt_scale);
        let eq63_e1010_d_n24: f64 = (s.dn[201][24] * ddt_scale);
        let eq63_e1010_d_n25: f64 = (s.dn[201][25] * ddt_scale);
        let eq63_e1010_d_n26: f64 = (s.dn[201][26] * ddt_scale);
        let eq63_e1010_d_n27: f64 = (s.dn[201][27] * ddt_scale);
        let eq63_e1010_d_n28: f64 = (s.dn[201][28] * ddt_scale);
        let eq63_e1010_d_n29: f64 = (s.dn[201][29] * ddt_scale);
        let eq63_e1013: f64 = (p.p355 * (nv7 - nv9));
        let eq63_e1013_d_n7: f64 = p.p355;
        let eq63_e1013_d_n9: f64 = (-p.p355);
        let eq63_e1014: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 43, eq63_e1013);
        let eq63_e1014_d_n7: f64 = (eq63_e1013_d_n7 * ddt_scale);
        let eq63_e1014_d_n9: f64 = (eq63_e1013_d_n9 * ddt_scale);
        let eq63_e1015: f64 = (eq63_e1010 + eq63_e1014);
        let eq63_e1015_d_n7: f64 = (eq63_e1010_d_n7 + eq63_e1014_d_n7);
        let eq63_e1015_d_n9: f64 = (eq63_e1010_d_n9 + eq63_e1014_d_n9);
        (eq63_e1015, eq63_e1010_d_n0, eq63_e1010_d_n1, eq63_e1010_d_n2, eq63_e1010_d_n3, eq63_e1010_d_n4, eq63_e1010_d_n5, eq63_e1010_d_n6, eq63_e1015_d_n7, eq63_e1010_d_n8, eq63_e1015_d_n9, eq63_e1010_d_n10, eq63_e1010_d_n11, eq63_e1010_d_n12, eq63_e1010_d_n13, eq63_e1010_d_n14, eq63_e1010_d_n15, eq63_e1010_d_n16, eq63_e1010_d_n17, eq63_e1010_d_n18, eq63_e1010_d_n19, eq63_e1010_d_n20, eq63_e1010_d_n21, eq63_e1010_d_n22, eq63_e1010_d_n23, eq63_e1010_d_n24, eq63_e1010_d_n25, eq63_e1010_d_n26, eq63_e1010_d_n27, eq63_e1010_d_n28, eq63_e1010_d_n29,)
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
            multiplicity * (eq63_value),
            nodes,
            &eq63_node_derivatives,
            branches,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e1028, eq64_e1028_d_n0, eq64_e1028_d_n1, eq64_e1028_d_n2, eq64_e1028_d_n3, eq64_e1028_d_n4, eq64_e1028_d_n5, eq64_e1028_d_n6, eq64_e1028_d_n7, eq64_e1028_d_n8, eq64_e1028_d_n9, eq64_e1028_d_n10, eq64_e1028_d_n11, eq64_e1028_d_n12, eq64_e1028_d_n13, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_d_n16, eq64_e1028_d_n17, eq64_e1028_d_n18, eq64_e1028_d_n19, eq64_e1028_d_n20, eq64_e1028_d_n21, eq64_e1028_d_n22, eq64_e1028_d_n23, eq64_e1028_d_n24, eq64_e1028_d_n25, eq64_e1028_d_n26, eq64_e1028_d_n27, eq64_e1028_d_n28, eq64_e1028_d_n29,) = {
    if (!s.b[760]) {
        let eq64_e1021: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 44, s.v[197]);
        let eq64_e1021_d_n0: f64 = (s.dn[197][0] * ddt_scale);
        let eq64_e1021_d_n1: f64 = (s.dn[197][1] * ddt_scale);
        let eq64_e1021_d_n2: f64 = (s.dn[197][2] * ddt_scale);
        let eq64_e1021_d_n3: f64 = (s.dn[197][3] * ddt_scale);
        let eq64_e1021_d_n4: f64 = (s.dn[197][4] * ddt_scale);
        let eq64_e1021_d_n5: f64 = (s.dn[197][5] * ddt_scale);
        let eq64_e1021_d_n6: f64 = (s.dn[197][6] * ddt_scale);
        let eq64_e1021_d_n7: f64 = (s.dn[197][7] * ddt_scale);
        let eq64_e1021_d_n8: f64 = (s.dn[197][8] * ddt_scale);
        let eq64_e1021_d_n9: f64 = (s.dn[197][9] * ddt_scale);
        let eq64_e1021_d_n10: f64 = (s.dn[197][10] * ddt_scale);
        let eq64_e1021_d_n11: f64 = (s.dn[197][11] * ddt_scale);
        let eq64_e1021_d_n12: f64 = (s.dn[197][12] * ddt_scale);
        let eq64_e1021_d_n13: f64 = (s.dn[197][13] * ddt_scale);
        let eq64_e1021_d_n14: f64 = (s.dn[197][14] * ddt_scale);
        let eq64_e1021_d_n15: f64 = (s.dn[197][15] * ddt_scale);
        let eq64_e1021_d_n16: f64 = (s.dn[197][16] * ddt_scale);
        let eq64_e1021_d_n17: f64 = (s.dn[197][17] * ddt_scale);
        let eq64_e1021_d_n18: f64 = (s.dn[197][18] * ddt_scale);
        let eq64_e1021_d_n19: f64 = (s.dn[197][19] * ddt_scale);
        let eq64_e1021_d_n20: f64 = (s.dn[197][20] * ddt_scale);
        let eq64_e1021_d_n21: f64 = (s.dn[197][21] * ddt_scale);
        let eq64_e1021_d_n22: f64 = (s.dn[197][22] * ddt_scale);
        let eq64_e1021_d_n23: f64 = (s.dn[197][23] * ddt_scale);
        let eq64_e1021_d_n24: f64 = (s.dn[197][24] * ddt_scale);
        let eq64_e1021_d_n25: f64 = (s.dn[197][25] * ddt_scale);
        let eq64_e1021_d_n26: f64 = (s.dn[197][26] * ddt_scale);
        let eq64_e1021_d_n27: f64 = (s.dn[197][27] * ddt_scale);
        let eq64_e1021_d_n28: f64 = (s.dn[197][28] * ddt_scale);
        let eq64_e1021_d_n29: f64 = (s.dn[197][29] * ddt_scale);
        let eq64_e1024: f64 = (p.p355 * (nv2 - nv14));
        let eq64_e1024_d_n2: f64 = p.p355;
        let eq64_e1024_d_n14: f64 = (-p.p355);
        let eq64_e1025: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 45, eq64_e1024);
        let eq64_e1025_d_n2: f64 = (eq64_e1024_d_n2 * ddt_scale);
        let eq64_e1025_d_n14: f64 = (eq64_e1024_d_n14 * ddt_scale);
        let eq64_e1026: f64 = (eq64_e1021 + eq64_e1025);
        let eq64_e1026_d_n2: f64 = (eq64_e1021_d_n2 + eq64_e1025_d_n2);
        let eq64_e1026_d_n14: f64 = (eq64_e1021_d_n14 + eq64_e1025_d_n14);
        (eq64_e1026, eq64_e1021_d_n0, eq64_e1021_d_n1, eq64_e1026_d_n2, eq64_e1021_d_n3, eq64_e1021_d_n4, eq64_e1021_d_n5, eq64_e1021_d_n6, eq64_e1021_d_n7, eq64_e1021_d_n8, eq64_e1021_d_n9, eq64_e1021_d_n10, eq64_e1021_d_n11, eq64_e1021_d_n12, eq64_e1021_d_n13, eq64_e1026_d_n14, eq64_e1021_d_n15, eq64_e1021_d_n16, eq64_e1021_d_n17, eq64_e1021_d_n18, eq64_e1021_d_n19, eq64_e1021_d_n20, eq64_e1021_d_n21, eq64_e1021_d_n22, eq64_e1021_d_n23, eq64_e1021_d_n24, eq64_e1021_d_n25, eq64_e1021_d_n26, eq64_e1021_d_n27, eq64_e1021_d_n28, eq64_e1021_d_n29,)
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
            multiplicity * (eq64_value),
            nodes,
            &eq64_node_derivatives,
            branches,
            &eq64_branch_derivatives,
            multiplicity,
        );
        let (eq65_e1039, eq65_e1039_d_n0, eq65_e1039_d_n1, eq65_e1039_d_n2, eq65_e1039_d_n3, eq65_e1039_d_n4, eq65_e1039_d_n5, eq65_e1039_d_n6, eq65_e1039_d_n7, eq65_e1039_d_n8, eq65_e1039_d_n9, eq65_e1039_d_n10, eq65_e1039_d_n11, eq65_e1039_d_n12, eq65_e1039_d_n13, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_d_n16, eq65_e1039_d_n17, eq65_e1039_d_n18, eq65_e1039_d_n19, eq65_e1039_d_n20, eq65_e1039_d_n21, eq65_e1039_d_n22, eq65_e1039_d_n23, eq65_e1039_d_n24, eq65_e1039_d_n25, eq65_e1039_d_n26, eq65_e1039_d_n27, eq65_e1039_d_n28, eq65_e1039_d_n29,) = {
    if (!s.b[760]) {
        let eq65_e1032: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 46, s.v[198]);
        let eq65_e1032_d_n0: f64 = (s.dn[198][0] * ddt_scale);
        let eq65_e1032_d_n1: f64 = (s.dn[198][1] * ddt_scale);
        let eq65_e1032_d_n2: f64 = (s.dn[198][2] * ddt_scale);
        let eq65_e1032_d_n3: f64 = (s.dn[198][3] * ddt_scale);
        let eq65_e1032_d_n4: f64 = (s.dn[198][4] * ddt_scale);
        let eq65_e1032_d_n5: f64 = (s.dn[198][5] * ddt_scale);
        let eq65_e1032_d_n6: f64 = (s.dn[198][6] * ddt_scale);
        let eq65_e1032_d_n7: f64 = (s.dn[198][7] * ddt_scale);
        let eq65_e1032_d_n8: f64 = (s.dn[198][8] * ddt_scale);
        let eq65_e1032_d_n9: f64 = (s.dn[198][9] * ddt_scale);
        let eq65_e1032_d_n10: f64 = (s.dn[198][10] * ddt_scale);
        let eq65_e1032_d_n11: f64 = (s.dn[198][11] * ddt_scale);
        let eq65_e1032_d_n12: f64 = (s.dn[198][12] * ddt_scale);
        let eq65_e1032_d_n13: f64 = (s.dn[198][13] * ddt_scale);
        let eq65_e1032_d_n14: f64 = (s.dn[198][14] * ddt_scale);
        let eq65_e1032_d_n15: f64 = (s.dn[198][15] * ddt_scale);
        let eq65_e1032_d_n16: f64 = (s.dn[198][16] * ddt_scale);
        let eq65_e1032_d_n17: f64 = (s.dn[198][17] * ddt_scale);
        let eq65_e1032_d_n18: f64 = (s.dn[198][18] * ddt_scale);
        let eq65_e1032_d_n19: f64 = (s.dn[198][19] * ddt_scale);
        let eq65_e1032_d_n20: f64 = (s.dn[198][20] * ddt_scale);
        let eq65_e1032_d_n21: f64 = (s.dn[198][21] * ddt_scale);
        let eq65_e1032_d_n22: f64 = (s.dn[198][22] * ddt_scale);
        let eq65_e1032_d_n23: f64 = (s.dn[198][23] * ddt_scale);
        let eq65_e1032_d_n24: f64 = (s.dn[198][24] * ddt_scale);
        let eq65_e1032_d_n25: f64 = (s.dn[198][25] * ddt_scale);
        let eq65_e1032_d_n26: f64 = (s.dn[198][26] * ddt_scale);
        let eq65_e1032_d_n27: f64 = (s.dn[198][27] * ddt_scale);
        let eq65_e1032_d_n28: f64 = (s.dn[198][28] * ddt_scale);
        let eq65_e1032_d_n29: f64 = (s.dn[198][29] * ddt_scale);
        let eq65_e1035: f64 = (p.p355 * (nv2 - nv15));
        let eq65_e1035_d_n2: f64 = p.p355;
        let eq65_e1035_d_n15: f64 = (-p.p355);
        let eq65_e1036: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 47, eq65_e1035);
        let eq65_e1036_d_n2: f64 = (eq65_e1035_d_n2 * ddt_scale);
        let eq65_e1036_d_n15: f64 = (eq65_e1035_d_n15 * ddt_scale);
        let eq65_e1037: f64 = (eq65_e1032 + eq65_e1036);
        let eq65_e1037_d_n2: f64 = (eq65_e1032_d_n2 + eq65_e1036_d_n2);
        let eq65_e1037_d_n15: f64 = (eq65_e1032_d_n15 + eq65_e1036_d_n15);
        (eq65_e1037, eq65_e1032_d_n0, eq65_e1032_d_n1, eq65_e1037_d_n2, eq65_e1032_d_n3, eq65_e1032_d_n4, eq65_e1032_d_n5, eq65_e1032_d_n6, eq65_e1032_d_n7, eq65_e1032_d_n8, eq65_e1032_d_n9, eq65_e1032_d_n10, eq65_e1032_d_n11, eq65_e1032_d_n12, eq65_e1032_d_n13, eq65_e1032_d_n14, eq65_e1037_d_n15, eq65_e1032_d_n16, eq65_e1032_d_n17, eq65_e1032_d_n18, eq65_e1032_d_n19, eq65_e1032_d_n20, eq65_e1032_d_n21, eq65_e1032_d_n22, eq65_e1032_d_n23, eq65_e1032_d_n24, eq65_e1032_d_n25, eq65_e1032_d_n26, eq65_e1032_d_n27, eq65_e1032_d_n28, eq65_e1032_d_n29,)
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
            multiplicity * (eq65_value),
            nodes,
            &eq65_node_derivatives,
            branches,
            &eq65_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1050, eq66_e1050_d_n0, eq66_e1050_d_n1, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n5, eq66_e1050_d_n6, eq66_e1050_d_n7, eq66_e1050_d_n8, eq66_e1050_d_n9, eq66_e1050_d_n10, eq66_e1050_d_n11, eq66_e1050_d_n12, eq66_e1050_d_n13, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_d_n16, eq66_e1050_d_n17, eq66_e1050_d_n18, eq66_e1050_d_n19, eq66_e1050_d_n20, eq66_e1050_d_n21, eq66_e1050_d_n22, eq66_e1050_d_n23, eq66_e1050_d_n24, eq66_e1050_d_n25, eq66_e1050_d_n26, eq66_e1050_d_n27, eq66_e1050_d_n28, eq66_e1050_d_n29,) = {
    if (!s.b[760]) {
        let eq66_e1043: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 48, s.v[199]);
        let eq66_e1043_d_n0: f64 = (s.dn[199][0] * ddt_scale);
        let eq66_e1043_d_n1: f64 = (s.dn[199][1] * ddt_scale);
        let eq66_e1043_d_n2: f64 = (s.dn[199][2] * ddt_scale);
        let eq66_e1043_d_n3: f64 = (s.dn[199][3] * ddt_scale);
        let eq66_e1043_d_n4: f64 = (s.dn[199][4] * ddt_scale);
        let eq66_e1043_d_n5: f64 = (s.dn[199][5] * ddt_scale);
        let eq66_e1043_d_n6: f64 = (s.dn[199][6] * ddt_scale);
        let eq66_e1043_d_n7: f64 = (s.dn[199][7] * ddt_scale);
        let eq66_e1043_d_n8: f64 = (s.dn[199][8] * ddt_scale);
        let eq66_e1043_d_n9: f64 = (s.dn[199][9] * ddt_scale);
        let eq66_e1043_d_n10: f64 = (s.dn[199][10] * ddt_scale);
        let eq66_e1043_d_n11: f64 = (s.dn[199][11] * ddt_scale);
        let eq66_e1043_d_n12: f64 = (s.dn[199][12] * ddt_scale);
        let eq66_e1043_d_n13: f64 = (s.dn[199][13] * ddt_scale);
        let eq66_e1043_d_n14: f64 = (s.dn[199][14] * ddt_scale);
        let eq66_e1043_d_n15: f64 = (s.dn[199][15] * ddt_scale);
        let eq66_e1043_d_n16: f64 = (s.dn[199][16] * ddt_scale);
        let eq66_e1043_d_n17: f64 = (s.dn[199][17] * ddt_scale);
        let eq66_e1043_d_n18: f64 = (s.dn[199][18] * ddt_scale);
        let eq66_e1043_d_n19: f64 = (s.dn[199][19] * ddt_scale);
        let eq66_e1043_d_n20: f64 = (s.dn[199][20] * ddt_scale);
        let eq66_e1043_d_n21: f64 = (s.dn[199][21] * ddt_scale);
        let eq66_e1043_d_n22: f64 = (s.dn[199][22] * ddt_scale);
        let eq66_e1043_d_n23: f64 = (s.dn[199][23] * ddt_scale);
        let eq66_e1043_d_n24: f64 = (s.dn[199][24] * ddt_scale);
        let eq66_e1043_d_n25: f64 = (s.dn[199][25] * ddt_scale);
        let eq66_e1043_d_n26: f64 = (s.dn[199][26] * ddt_scale);
        let eq66_e1043_d_n27: f64 = (s.dn[199][27] * ddt_scale);
        let eq66_e1043_d_n28: f64 = (s.dn[199][28] * ddt_scale);
        let eq66_e1043_d_n29: f64 = (s.dn[199][29] * ddt_scale);
        let eq66_e1046: f64 = (p.p355 * (nv7 - nv14));
        let eq66_e1046_d_n7: f64 = p.p355;
        let eq66_e1046_d_n14: f64 = (-p.p355);
        let eq66_e1047: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 49, eq66_e1046);
        let eq66_e1047_d_n7: f64 = (eq66_e1046_d_n7 * ddt_scale);
        let eq66_e1047_d_n14: f64 = (eq66_e1046_d_n14 * ddt_scale);
        let eq66_e1048: f64 = (eq66_e1043 + eq66_e1047);
        let eq66_e1048_d_n7: f64 = (eq66_e1043_d_n7 + eq66_e1047_d_n7);
        let eq66_e1048_d_n14: f64 = (eq66_e1043_d_n14 + eq66_e1047_d_n14);
        (eq66_e1048, eq66_e1043_d_n0, eq66_e1043_d_n1, eq66_e1043_d_n2, eq66_e1043_d_n3, eq66_e1043_d_n4, eq66_e1043_d_n5, eq66_e1043_d_n6, eq66_e1048_d_n7, eq66_e1043_d_n8, eq66_e1043_d_n9, eq66_e1043_d_n10, eq66_e1043_d_n11, eq66_e1043_d_n12, eq66_e1043_d_n13, eq66_e1048_d_n14, eq66_e1043_d_n15, eq66_e1043_d_n16, eq66_e1043_d_n17, eq66_e1043_d_n18, eq66_e1043_d_n19, eq66_e1043_d_n20, eq66_e1043_d_n21, eq66_e1043_d_n22, eq66_e1043_d_n23, eq66_e1043_d_n24, eq66_e1043_d_n25, eq66_e1043_d_n26, eq66_e1043_d_n27, eq66_e1043_d_n28, eq66_e1043_d_n29,)
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
            multiplicity * (eq66_value),
            nodes,
            &eq66_node_derivatives,
            branches,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let (eq67_e1055,) = {
    if (!s.b[760]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1055;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[15]),
            multiplicity * (eq67_value),
        );
        let (eq68_e1060,) = {
    if (!s.b[760]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e1060;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq68_value),
        );
        let eq69_e1062: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 50, s.v[200]);
        let eq69_e1062_d_n0: f64 = (s.dn[200][0] * ddt_scale);
        let eq69_e1062_d_n1: f64 = (s.dn[200][1] * ddt_scale);
        let eq69_e1062_d_n2: f64 = (s.dn[200][2] * ddt_scale);
        let eq69_e1062_d_n3: f64 = (s.dn[200][3] * ddt_scale);
        let eq69_e1062_d_n4: f64 = (s.dn[200][4] * ddt_scale);
        let eq69_e1062_d_n5: f64 = (s.dn[200][5] * ddt_scale);
        let eq69_e1062_d_n6: f64 = (s.dn[200][6] * ddt_scale);
        let eq69_e1062_d_n7: f64 = (s.dn[200][7] * ddt_scale);
        let eq69_e1062_d_n8: f64 = (s.dn[200][8] * ddt_scale);
        let eq69_e1062_d_n9: f64 = (s.dn[200][9] * ddt_scale);
        let eq69_e1062_d_n10: f64 = (s.dn[200][10] * ddt_scale);
        let eq69_e1062_d_n11: f64 = (s.dn[200][11] * ddt_scale);
        let eq69_e1062_d_n12: f64 = (s.dn[200][12] * ddt_scale);
        let eq69_e1062_d_n13: f64 = (s.dn[200][13] * ddt_scale);
        let eq69_e1062_d_n14: f64 = (s.dn[200][14] * ddt_scale);
        let eq69_e1062_d_n15: f64 = (s.dn[200][15] * ddt_scale);
        let eq69_e1062_d_n16: f64 = (s.dn[200][16] * ddt_scale);
        let eq69_e1062_d_n17: f64 = (s.dn[200][17] * ddt_scale);
        let eq69_e1062_d_n18: f64 = (s.dn[200][18] * ddt_scale);
        let eq69_e1062_d_n19: f64 = (s.dn[200][19] * ddt_scale);
        let eq69_e1062_d_n20: f64 = (s.dn[200][20] * ddt_scale);
        let eq69_e1062_d_n21: f64 = (s.dn[200][21] * ddt_scale);
        let eq69_e1062_d_n22: f64 = (s.dn[200][22] * ddt_scale);
        let eq69_e1062_d_n23: f64 = (s.dn[200][23] * ddt_scale);
        let eq69_e1062_d_n24: f64 = (s.dn[200][24] * ddt_scale);
        let eq69_e1062_d_n25: f64 = (s.dn[200][25] * ddt_scale);
        let eq69_e1062_d_n26: f64 = (s.dn[200][26] * ddt_scale);
        let eq69_e1062_d_n27: f64 = (s.dn[200][27] * ddt_scale);
        let eq69_e1062_d_n28: f64 = (s.dn[200][28] * ddt_scale);
        let eq69_e1062_d_n29: f64 = (s.dn[200][29] * ddt_scale);
        let eq69_e1065: f64 = (p.p355 * (nv3 - nv14));
        let eq69_e1065_d_n3: f64 = p.p355;
        let eq69_e1065_d_n14: f64 = (-p.p355);
        let eq69_e1066: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 51, eq69_e1065);
        let eq69_e1066_d_n3: f64 = (eq69_e1065_d_n3 * ddt_scale);
        let eq69_e1066_d_n14: f64 = (eq69_e1065_d_n14 * ddt_scale);
        let eq69_e1067: f64 = (eq69_e1062 + eq69_e1066);
        let eq69_e1067_d_n3: f64 = (eq69_e1062_d_n3 + eq69_e1066_d_n3);
        let eq69_e1067_d_n14: f64 = (eq69_e1062_d_n14 + eq69_e1066_d_n14);
        let eq69_value: f64 = eq69_e1067;
        let eq69_node_derivatives: [f64; 30] = [eq69_e1062_d_n0, eq69_e1062_d_n1, eq69_e1062_d_n2, eq69_e1067_d_n3, eq69_e1062_d_n4, eq69_e1062_d_n5, eq69_e1062_d_n6, eq69_e1062_d_n7, eq69_e1062_d_n8, eq69_e1062_d_n9, eq69_e1062_d_n10, eq69_e1062_d_n11, eq69_e1062_d_n12, eq69_e1062_d_n13, eq69_e1067_d_n14, eq69_e1062_d_n15, eq69_e1062_d_n16, eq69_e1062_d_n17, eq69_e1062_d_n18, eq69_e1062_d_n19, eq69_e1062_d_n20, eq69_e1062_d_n21, eq69_e1062_d_n22, eq69_e1062_d_n23, eq69_e1062_d_n24, eq69_e1062_d_n25, eq69_e1062_d_n26, eq69_e1062_d_n27, eq69_e1062_d_n28, eq69_e1062_d_n29];
        let eq69_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[14]),
            multiplicity * (eq69_value),
            nodes,
            &eq69_node_derivatives,
            branches,
            &eq69_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq70_e1075, eq70_e1075_d_n0, eq70_e1075_d_n1, eq70_e1075_d_n2, eq70_e1075_d_n3, eq70_e1075_d_n4, eq70_e1075_d_n5, eq70_e1075_d_n6, eq70_e1075_d_n7, eq70_e1075_d_n8, eq70_e1075_d_n9, eq70_e1075_d_n10, eq70_e1075_d_n11, eq70_e1075_d_n12, eq70_e1075_d_n13, eq70_e1075_d_n14, eq70_e1075_d_n15, eq70_e1075_d_n16, eq70_e1075_d_n17, eq70_e1075_d_n18, eq70_e1075_d_n19, eq70_e1075_d_n20, eq70_e1075_d_n21, eq70_e1075_d_n22, eq70_e1075_d_n23, eq70_e1075_d_n24, eq70_e1075_d_n25, eq70_e1075_d_n26, eq70_e1075_d_n27, eq70_e1075_d_n28, eq70_e1075_d_n29,) = {
    if s.b[761] {
        let eq70_e1072: f64 = (s.v[0] * (nv14 - nv5));
        let eq70_e1072_d_n5: f64 = (-s.v[0]);
        let eq70_e1072_d_n14: f64 = s.v[0];
        let eq70_e1073: f64 = (s.v[190] + eq70_e1072);
        let eq70_e1073_d_n5: f64 = (s.dn[190][5] + eq70_e1072_d_n5);
        let eq70_e1073_d_n14: f64 = (s.dn[190][14] + eq70_e1072_d_n14);
        (eq70_e1073, s.dn[190][0], s.dn[190][1], s.dn[190][2], s.dn[190][3], s.dn[190][4], eq70_e1073_d_n5, s.dn[190][6], s.dn[190][7], s.dn[190][8], s.dn[190][9], s.dn[190][10], s.dn[190][11], s.dn[190][12], s.dn[190][13], eq70_e1073_d_n14, s.dn[190][15], s.dn[190][16], s.dn[190][17], s.dn[190][18], s.dn[190][19], s.dn[190][20], s.dn[190][21], s.dn[190][22], s.dn[190][23], s.dn[190][24], s.dn[190][25], s.dn[190][26], s.dn[190][27], s.dn[190][28], s.dn[190][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e1075;
        let eq70_node_derivatives: [f64; 30] = [eq70_e1075_d_n0, eq70_e1075_d_n1, eq70_e1075_d_n2, eq70_e1075_d_n3, eq70_e1075_d_n4, eq70_e1075_d_n5, eq70_e1075_d_n6, eq70_e1075_d_n7, eq70_e1075_d_n8, eq70_e1075_d_n9, eq70_e1075_d_n10, eq70_e1075_d_n11, eq70_e1075_d_n12, eq70_e1075_d_n13, eq70_e1075_d_n14, eq70_e1075_d_n15, eq70_e1075_d_n16, eq70_e1075_d_n17, eq70_e1075_d_n18, eq70_e1075_d_n19, eq70_e1075_d_n20, eq70_e1075_d_n21, eq70_e1075_d_n22, eq70_e1075_d_n23, eq70_e1075_d_n24, eq70_e1075_d_n25, eq70_e1075_d_n26, eq70_e1075_d_n27, eq70_e1075_d_n28, eq70_e1075_d_n29];
        let eq70_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            multiplicity * (eq70_value),
            nodes,
            &eq70_node_derivatives,
            branches,
            &eq70_branch_derivatives,
            multiplicity,
        );
        let (eq71_e1080,) = {
    if (!s.b[761]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e1080;
        stamper.stamp_potential_const(
            branches[21],
            eq71_value,
        );
        let (eq72_e1090, eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29,) = {
    if s.b[907] {
        let eq72_e1083: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 52, s.v[191]);
        let eq72_e1083_d_n0: f64 = (s.dn[191][0] * ddt_scale);
        let eq72_e1083_d_n1: f64 = (s.dn[191][1] * ddt_scale);
        let eq72_e1083_d_n2: f64 = (s.dn[191][2] * ddt_scale);
        let eq72_e1083_d_n3: f64 = (s.dn[191][3] * ddt_scale);
        let eq72_e1083_d_n4: f64 = (s.dn[191][4] * ddt_scale);
        let eq72_e1083_d_n5: f64 = (s.dn[191][5] * ddt_scale);
        let eq72_e1083_d_n6: f64 = (s.dn[191][6] * ddt_scale);
        let eq72_e1083_d_n7: f64 = (s.dn[191][7] * ddt_scale);
        let eq72_e1083_d_n8: f64 = (s.dn[191][8] * ddt_scale);
        let eq72_e1083_d_n9: f64 = (s.dn[191][9] * ddt_scale);
        let eq72_e1083_d_n10: f64 = (s.dn[191][10] * ddt_scale);
        let eq72_e1083_d_n11: f64 = (s.dn[191][11] * ddt_scale);
        let eq72_e1083_d_n12: f64 = (s.dn[191][12] * ddt_scale);
        let eq72_e1083_d_n13: f64 = (s.dn[191][13] * ddt_scale);
        let eq72_e1083_d_n14: f64 = (s.dn[191][14] * ddt_scale);
        let eq72_e1083_d_n15: f64 = (s.dn[191][15] * ddt_scale);
        let eq72_e1083_d_n16: f64 = (s.dn[191][16] * ddt_scale);
        let eq72_e1083_d_n17: f64 = (s.dn[191][17] * ddt_scale);
        let eq72_e1083_d_n18: f64 = (s.dn[191][18] * ddt_scale);
        let eq72_e1083_d_n19: f64 = (s.dn[191][19] * ddt_scale);
        let eq72_e1083_d_n20: f64 = (s.dn[191][20] * ddt_scale);
        let eq72_e1083_d_n21: f64 = (s.dn[191][21] * ddt_scale);
        let eq72_e1083_d_n22: f64 = (s.dn[191][22] * ddt_scale);
        let eq72_e1083_d_n23: f64 = (s.dn[191][23] * ddt_scale);
        let eq72_e1083_d_n24: f64 = (s.dn[191][24] * ddt_scale);
        let eq72_e1083_d_n25: f64 = (s.dn[191][25] * ddt_scale);
        let eq72_e1083_d_n26: f64 = (s.dn[191][26] * ddt_scale);
        let eq72_e1083_d_n27: f64 = (s.dn[191][27] * ddt_scale);
        let eq72_e1083_d_n28: f64 = (s.dn[191][28] * ddt_scale);
        let eq72_e1083_d_n29: f64 = (s.dn[191][29] * ddt_scale);
        let eq72_e1086: f64 = (p.p355 * (nv7 - nv5));
        let eq72_e1086_d_n5: f64 = (-p.p355);
        let eq72_e1086_d_n7: f64 = p.p355;
        let eq72_e1087: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 53, eq72_e1086);
        let eq72_e1087_d_n5: f64 = (eq72_e1086_d_n5 * ddt_scale);
        let eq72_e1087_d_n7: f64 = (eq72_e1086_d_n7 * ddt_scale);
        let eq72_e1088: f64 = (eq72_e1083 + eq72_e1087);
        let eq72_e1088_d_n5: f64 = (eq72_e1083_d_n5 + eq72_e1087_d_n5);
        let eq72_e1088_d_n7: f64 = (eq72_e1083_d_n7 + eq72_e1087_d_n7);
        (eq72_e1088, eq72_e1083_d_n0, eq72_e1083_d_n1, eq72_e1083_d_n2, eq72_e1083_d_n3, eq72_e1083_d_n4, eq72_e1088_d_n5, eq72_e1083_d_n6, eq72_e1088_d_n7, eq72_e1083_d_n8, eq72_e1083_d_n9, eq72_e1083_d_n10, eq72_e1083_d_n11, eq72_e1083_d_n12, eq72_e1083_d_n13, eq72_e1083_d_n14, eq72_e1083_d_n15, eq72_e1083_d_n16, eq72_e1083_d_n17, eq72_e1083_d_n18, eq72_e1083_d_n19, eq72_e1083_d_n20, eq72_e1083_d_n21, eq72_e1083_d_n22, eq72_e1083_d_n23, eq72_e1083_d_n24, eq72_e1083_d_n25, eq72_e1083_d_n26, eq72_e1083_d_n27, eq72_e1083_d_n28, eq72_e1083_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1090;
        let eq72_node_derivatives: [f64; 30] = [eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29];
        let eq72_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            multiplicity * (eq72_value),
            nodes,
            &eq72_node_derivatives,
            branches,
            &eq72_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1100, eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29,) = {
    if s.b[907] {
        let eq73_e1093: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 54, s.v[192]);
        let eq73_e1093_d_n0: f64 = (s.dn[192][0] * ddt_scale);
        let eq73_e1093_d_n1: f64 = (s.dn[192][1] * ddt_scale);
        let eq73_e1093_d_n2: f64 = (s.dn[192][2] * ddt_scale);
        let eq73_e1093_d_n3: f64 = (s.dn[192][3] * ddt_scale);
        let eq73_e1093_d_n4: f64 = (s.dn[192][4] * ddt_scale);
        let eq73_e1093_d_n5: f64 = (s.dn[192][5] * ddt_scale);
        let eq73_e1093_d_n6: f64 = (s.dn[192][6] * ddt_scale);
        let eq73_e1093_d_n7: f64 = (s.dn[192][7] * ddt_scale);
        let eq73_e1093_d_n8: f64 = (s.dn[192][8] * ddt_scale);
        let eq73_e1093_d_n9: f64 = (s.dn[192][9] * ddt_scale);
        let eq73_e1093_d_n10: f64 = (s.dn[192][10] * ddt_scale);
        let eq73_e1093_d_n11: f64 = (s.dn[192][11] * ddt_scale);
        let eq73_e1093_d_n12: f64 = (s.dn[192][12] * ddt_scale);
        let eq73_e1093_d_n13: f64 = (s.dn[192][13] * ddt_scale);
        let eq73_e1093_d_n14: f64 = (s.dn[192][14] * ddt_scale);
        let eq73_e1093_d_n15: f64 = (s.dn[192][15] * ddt_scale);
        let eq73_e1093_d_n16: f64 = (s.dn[192][16] * ddt_scale);
        let eq73_e1093_d_n17: f64 = (s.dn[192][17] * ddt_scale);
        let eq73_e1093_d_n18: f64 = (s.dn[192][18] * ddt_scale);
        let eq73_e1093_d_n19: f64 = (s.dn[192][19] * ddt_scale);
        let eq73_e1093_d_n20: f64 = (s.dn[192][20] * ddt_scale);
        let eq73_e1093_d_n21: f64 = (s.dn[192][21] * ddt_scale);
        let eq73_e1093_d_n22: f64 = (s.dn[192][22] * ddt_scale);
        let eq73_e1093_d_n23: f64 = (s.dn[192][23] * ddt_scale);
        let eq73_e1093_d_n24: f64 = (s.dn[192][24] * ddt_scale);
        let eq73_e1093_d_n25: f64 = (s.dn[192][25] * ddt_scale);
        let eq73_e1093_d_n26: f64 = (s.dn[192][26] * ddt_scale);
        let eq73_e1093_d_n27: f64 = (s.dn[192][27] * ddt_scale);
        let eq73_e1093_d_n28: f64 = (s.dn[192][28] * ddt_scale);
        let eq73_e1093_d_n29: f64 = (s.dn[192][29] * ddt_scale);
        let eq73_e1096: f64 = (p.p355 * (nv7 - nv14));
        let eq73_e1096_d_n7: f64 = p.p355;
        let eq73_e1096_d_n14: f64 = (-p.p355);
        let eq73_e1097: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 55, eq73_e1096);
        let eq73_e1097_d_n7: f64 = (eq73_e1096_d_n7 * ddt_scale);
        let eq73_e1097_d_n14: f64 = (eq73_e1096_d_n14 * ddt_scale);
        let eq73_e1098: f64 = (eq73_e1093 + eq73_e1097);
        let eq73_e1098_d_n7: f64 = (eq73_e1093_d_n7 + eq73_e1097_d_n7);
        let eq73_e1098_d_n14: f64 = (eq73_e1093_d_n14 + eq73_e1097_d_n14);
        (eq73_e1098, eq73_e1093_d_n0, eq73_e1093_d_n1, eq73_e1093_d_n2, eq73_e1093_d_n3, eq73_e1093_d_n4, eq73_e1093_d_n5, eq73_e1093_d_n6, eq73_e1098_d_n7, eq73_e1093_d_n8, eq73_e1093_d_n9, eq73_e1093_d_n10, eq73_e1093_d_n11, eq73_e1093_d_n12, eq73_e1093_d_n13, eq73_e1098_d_n14, eq73_e1093_d_n15, eq73_e1093_d_n16, eq73_e1093_d_n17, eq73_e1093_d_n18, eq73_e1093_d_n19, eq73_e1093_d_n20, eq73_e1093_d_n21, eq73_e1093_d_n22, eq73_e1093_d_n23, eq73_e1093_d_n24, eq73_e1093_d_n25, eq73_e1093_d_n26, eq73_e1093_d_n27, eq73_e1093_d_n28, eq73_e1093_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1100;
        let eq73_node_derivatives: [f64; 30] = [eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29];
        let eq73_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            multiplicity * (eq73_value),
            nodes,
            &eq73_node_derivatives,
            branches,
            &eq73_branch_derivatives,
            multiplicity,
        );
        let (eq74_e1110, eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29,) = {
    if s.b[907] {
        let eq74_e1103: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 56, s.v[193]);
        let eq74_e1103_d_n0: f64 = (s.dn[193][0] * ddt_scale);
        let eq74_e1103_d_n1: f64 = (s.dn[193][1] * ddt_scale);
        let eq74_e1103_d_n2: f64 = (s.dn[193][2] * ddt_scale);
        let eq74_e1103_d_n3: f64 = (s.dn[193][3] * ddt_scale);
        let eq74_e1103_d_n4: f64 = (s.dn[193][4] * ddt_scale);
        let eq74_e1103_d_n5: f64 = (s.dn[193][5] * ddt_scale);
        let eq74_e1103_d_n6: f64 = (s.dn[193][6] * ddt_scale);
        let eq74_e1103_d_n7: f64 = (s.dn[193][7] * ddt_scale);
        let eq74_e1103_d_n8: f64 = (s.dn[193][8] * ddt_scale);
        let eq74_e1103_d_n9: f64 = (s.dn[193][9] * ddt_scale);
        let eq74_e1103_d_n10: f64 = (s.dn[193][10] * ddt_scale);
        let eq74_e1103_d_n11: f64 = (s.dn[193][11] * ddt_scale);
        let eq74_e1103_d_n12: f64 = (s.dn[193][12] * ddt_scale);
        let eq74_e1103_d_n13: f64 = (s.dn[193][13] * ddt_scale);
        let eq74_e1103_d_n14: f64 = (s.dn[193][14] * ddt_scale);
        let eq74_e1103_d_n15: f64 = (s.dn[193][15] * ddt_scale);
        let eq74_e1103_d_n16: f64 = (s.dn[193][16] * ddt_scale);
        let eq74_e1103_d_n17: f64 = (s.dn[193][17] * ddt_scale);
        let eq74_e1103_d_n18: f64 = (s.dn[193][18] * ddt_scale);
        let eq74_e1103_d_n19: f64 = (s.dn[193][19] * ddt_scale);
        let eq74_e1103_d_n20: f64 = (s.dn[193][20] * ddt_scale);
        let eq74_e1103_d_n21: f64 = (s.dn[193][21] * ddt_scale);
        let eq74_e1103_d_n22: f64 = (s.dn[193][22] * ddt_scale);
        let eq74_e1103_d_n23: f64 = (s.dn[193][23] * ddt_scale);
        let eq74_e1103_d_n24: f64 = (s.dn[193][24] * ddt_scale);
        let eq74_e1103_d_n25: f64 = (s.dn[193][25] * ddt_scale);
        let eq74_e1103_d_n26: f64 = (s.dn[193][26] * ddt_scale);
        let eq74_e1103_d_n27: f64 = (s.dn[193][27] * ddt_scale);
        let eq74_e1103_d_n28: f64 = (s.dn[193][28] * ddt_scale);
        let eq74_e1103_d_n29: f64 = (s.dn[193][29] * ddt_scale);
        let eq74_e1106: f64 = (p.p355 * (nv2 - nv5));
        let eq74_e1106_d_n2: f64 = p.p355;
        let eq74_e1106_d_n5: f64 = (-p.p355);
        let eq74_e1107: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 57, eq74_e1106);
        let eq74_e1107_d_n2: f64 = (eq74_e1106_d_n2 * ddt_scale);
        let eq74_e1107_d_n5: f64 = (eq74_e1106_d_n5 * ddt_scale);
        let eq74_e1108: f64 = (eq74_e1103 + eq74_e1107);
        let eq74_e1108_d_n2: f64 = (eq74_e1103_d_n2 + eq74_e1107_d_n2);
        let eq74_e1108_d_n5: f64 = (eq74_e1103_d_n5 + eq74_e1107_d_n5);
        (eq74_e1108, eq74_e1103_d_n0, eq74_e1103_d_n1, eq74_e1108_d_n2, eq74_e1103_d_n3, eq74_e1103_d_n4, eq74_e1108_d_n5, eq74_e1103_d_n6, eq74_e1103_d_n7, eq74_e1103_d_n8, eq74_e1103_d_n9, eq74_e1103_d_n10, eq74_e1103_d_n11, eq74_e1103_d_n12, eq74_e1103_d_n13, eq74_e1103_d_n14, eq74_e1103_d_n15, eq74_e1103_d_n16, eq74_e1103_d_n17, eq74_e1103_d_n18, eq74_e1103_d_n19, eq74_e1103_d_n20, eq74_e1103_d_n21, eq74_e1103_d_n22, eq74_e1103_d_n23, eq74_e1103_d_n24, eq74_e1103_d_n25, eq74_e1103_d_n26, eq74_e1103_d_n27, eq74_e1103_d_n28, eq74_e1103_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1110;
        let eq74_node_derivatives: [f64; 30] = [eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29];
        let eq74_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[5]),
            multiplicity * (eq74_value),
            nodes,
            &eq74_node_derivatives,
            branches,
            &eq74_branch_derivatives,
            multiplicity,
        );
        let (eq75_e1114,) = {
    if s.b[907] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e1114;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[14]),
            multiplicity * (eq75_value),
        );
        let (eq76_e1124, eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29,) = {
    if s.b[907] {
        let eq76_e1117: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 58, s.v[195]);
        let eq76_e1117_d_n0: f64 = (s.dn[195][0] * ddt_scale);
        let eq76_e1117_d_n1: f64 = (s.dn[195][1] * ddt_scale);
        let eq76_e1117_d_n2: f64 = (s.dn[195][2] * ddt_scale);
        let eq76_e1117_d_n3: f64 = (s.dn[195][3] * ddt_scale);
        let eq76_e1117_d_n4: f64 = (s.dn[195][4] * ddt_scale);
        let eq76_e1117_d_n5: f64 = (s.dn[195][5] * ddt_scale);
        let eq76_e1117_d_n6: f64 = (s.dn[195][6] * ddt_scale);
        let eq76_e1117_d_n7: f64 = (s.dn[195][7] * ddt_scale);
        let eq76_e1117_d_n8: f64 = (s.dn[195][8] * ddt_scale);
        let eq76_e1117_d_n9: f64 = (s.dn[195][9] * ddt_scale);
        let eq76_e1117_d_n10: f64 = (s.dn[195][10] * ddt_scale);
        let eq76_e1117_d_n11: f64 = (s.dn[195][11] * ddt_scale);
        let eq76_e1117_d_n12: f64 = (s.dn[195][12] * ddt_scale);
        let eq76_e1117_d_n13: f64 = (s.dn[195][13] * ddt_scale);
        let eq76_e1117_d_n14: f64 = (s.dn[195][14] * ddt_scale);
        let eq76_e1117_d_n15: f64 = (s.dn[195][15] * ddt_scale);
        let eq76_e1117_d_n16: f64 = (s.dn[195][16] * ddt_scale);
        let eq76_e1117_d_n17: f64 = (s.dn[195][17] * ddt_scale);
        let eq76_e1117_d_n18: f64 = (s.dn[195][18] * ddt_scale);
        let eq76_e1117_d_n19: f64 = (s.dn[195][19] * ddt_scale);
        let eq76_e1117_d_n20: f64 = (s.dn[195][20] * ddt_scale);
        let eq76_e1117_d_n21: f64 = (s.dn[195][21] * ddt_scale);
        let eq76_e1117_d_n22: f64 = (s.dn[195][22] * ddt_scale);
        let eq76_e1117_d_n23: f64 = (s.dn[195][23] * ddt_scale);
        let eq76_e1117_d_n24: f64 = (s.dn[195][24] * ddt_scale);
        let eq76_e1117_d_n25: f64 = (s.dn[195][25] * ddt_scale);
        let eq76_e1117_d_n26: f64 = (s.dn[195][26] * ddt_scale);
        let eq76_e1117_d_n27: f64 = (s.dn[195][27] * ddt_scale);
        let eq76_e1117_d_n28: f64 = (s.dn[195][28] * ddt_scale);
        let eq76_e1117_d_n29: f64 = (s.dn[195][29] * ddt_scale);
        let eq76_e1120: f64 = (p.p355 * (nv7 - nv9));
        let eq76_e1120_d_n7: f64 = p.p355;
        let eq76_e1120_d_n9: f64 = (-p.p355);
        let eq76_e1121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 59, eq76_e1120);
        let eq76_e1121_d_n7: f64 = (eq76_e1120_d_n7 * ddt_scale);
        let eq76_e1121_d_n9: f64 = (eq76_e1120_d_n9 * ddt_scale);
        let eq76_e1122: f64 = (eq76_e1117 + eq76_e1121);
        let eq76_e1122_d_n7: f64 = (eq76_e1117_d_n7 + eq76_e1121_d_n7);
        let eq76_e1122_d_n9: f64 = (eq76_e1117_d_n9 + eq76_e1121_d_n9);
        (eq76_e1122, eq76_e1117_d_n0, eq76_e1117_d_n1, eq76_e1117_d_n2, eq76_e1117_d_n3, eq76_e1117_d_n4, eq76_e1117_d_n5, eq76_e1117_d_n6, eq76_e1122_d_n7, eq76_e1117_d_n8, eq76_e1122_d_n9, eq76_e1117_d_n10, eq76_e1117_d_n11, eq76_e1117_d_n12, eq76_e1117_d_n13, eq76_e1117_d_n14, eq76_e1117_d_n15, eq76_e1117_d_n16, eq76_e1117_d_n17, eq76_e1117_d_n18, eq76_e1117_d_n19, eq76_e1117_d_n20, eq76_e1117_d_n21, eq76_e1117_d_n22, eq76_e1117_d_n23, eq76_e1117_d_n24, eq76_e1117_d_n25, eq76_e1117_d_n26, eq76_e1117_d_n27, eq76_e1117_d_n28, eq76_e1117_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1124;
        let eq76_node_derivatives: [f64; 30] = [eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29];
        let eq76_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq76_value),
            nodes,
            &eq76_node_derivatives,
            branches,
            &eq76_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1135, eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29,) = {
    if (!s.b[907]) {
        let eq77_e1128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 60, s.v[191]);
        let eq77_e1128_d_n0: f64 = (s.dn[191][0] * ddt_scale);
        let eq77_e1128_d_n1: f64 = (s.dn[191][1] * ddt_scale);
        let eq77_e1128_d_n2: f64 = (s.dn[191][2] * ddt_scale);
        let eq77_e1128_d_n3: f64 = (s.dn[191][3] * ddt_scale);
        let eq77_e1128_d_n4: f64 = (s.dn[191][4] * ddt_scale);
        let eq77_e1128_d_n5: f64 = (s.dn[191][5] * ddt_scale);
        let eq77_e1128_d_n6: f64 = (s.dn[191][6] * ddt_scale);
        let eq77_e1128_d_n7: f64 = (s.dn[191][7] * ddt_scale);
        let eq77_e1128_d_n8: f64 = (s.dn[191][8] * ddt_scale);
        let eq77_e1128_d_n9: f64 = (s.dn[191][9] * ddt_scale);
        let eq77_e1128_d_n10: f64 = (s.dn[191][10] * ddt_scale);
        let eq77_e1128_d_n11: f64 = (s.dn[191][11] * ddt_scale);
        let eq77_e1128_d_n12: f64 = (s.dn[191][12] * ddt_scale);
        let eq77_e1128_d_n13: f64 = (s.dn[191][13] * ddt_scale);
        let eq77_e1128_d_n14: f64 = (s.dn[191][14] * ddt_scale);
        let eq77_e1128_d_n15: f64 = (s.dn[191][15] * ddt_scale);
        let eq77_e1128_d_n16: f64 = (s.dn[191][16] * ddt_scale);
        let eq77_e1128_d_n17: f64 = (s.dn[191][17] * ddt_scale);
        let eq77_e1128_d_n18: f64 = (s.dn[191][18] * ddt_scale);
        let eq77_e1128_d_n19: f64 = (s.dn[191][19] * ddt_scale);
        let eq77_e1128_d_n20: f64 = (s.dn[191][20] * ddt_scale);
        let eq77_e1128_d_n21: f64 = (s.dn[191][21] * ddt_scale);
        let eq77_e1128_d_n22: f64 = (s.dn[191][22] * ddt_scale);
        let eq77_e1128_d_n23: f64 = (s.dn[191][23] * ddt_scale);
        let eq77_e1128_d_n24: f64 = (s.dn[191][24] * ddt_scale);
        let eq77_e1128_d_n25: f64 = (s.dn[191][25] * ddt_scale);
        let eq77_e1128_d_n26: f64 = (s.dn[191][26] * ddt_scale);
        let eq77_e1128_d_n27: f64 = (s.dn[191][27] * ddt_scale);
        let eq77_e1128_d_n28: f64 = (s.dn[191][28] * ddt_scale);
        let eq77_e1128_d_n29: f64 = (s.dn[191][29] * ddt_scale);
        let eq77_e1131: f64 = (p.p355 * (nv2 - nv5));
        let eq77_e1131_d_n2: f64 = p.p355;
        let eq77_e1131_d_n5: f64 = (-p.p355);
        let eq77_e1132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 61, eq77_e1131);
        let eq77_e1132_d_n2: f64 = (eq77_e1131_d_n2 * ddt_scale);
        let eq77_e1132_d_n5: f64 = (eq77_e1131_d_n5 * ddt_scale);
        let eq77_e1133: f64 = (eq77_e1128 + eq77_e1132);
        let eq77_e1133_d_n2: f64 = (eq77_e1128_d_n2 + eq77_e1132_d_n2);
        let eq77_e1133_d_n5: f64 = (eq77_e1128_d_n5 + eq77_e1132_d_n5);
        (eq77_e1133, eq77_e1128_d_n0, eq77_e1128_d_n1, eq77_e1133_d_n2, eq77_e1128_d_n3, eq77_e1128_d_n4, eq77_e1133_d_n5, eq77_e1128_d_n6, eq77_e1128_d_n7, eq77_e1128_d_n8, eq77_e1128_d_n9, eq77_e1128_d_n10, eq77_e1128_d_n11, eq77_e1128_d_n12, eq77_e1128_d_n13, eq77_e1128_d_n14, eq77_e1128_d_n15, eq77_e1128_d_n16, eq77_e1128_d_n17, eq77_e1128_d_n18, eq77_e1128_d_n19, eq77_e1128_d_n20, eq77_e1128_d_n21, eq77_e1128_d_n22, eq77_e1128_d_n23, eq77_e1128_d_n24, eq77_e1128_d_n25, eq77_e1128_d_n26, eq77_e1128_d_n27, eq77_e1128_d_n28, eq77_e1128_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1135;
        let eq77_node_derivatives: [f64; 30] = [eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29];
        let eq77_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[5]),
            multiplicity * (eq77_value),
            nodes,
            &eq77_node_derivatives,
            branches,
            &eq77_branch_derivatives,
            multiplicity,
        );
        let (eq78_e1146, eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29,) = {
    if (!s.b[907]) {
        let eq78_e1139: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 62, s.v[192]);
        let eq78_e1139_d_n0: f64 = (s.dn[192][0] * ddt_scale);
        let eq78_e1139_d_n1: f64 = (s.dn[192][1] * ddt_scale);
        let eq78_e1139_d_n2: f64 = (s.dn[192][2] * ddt_scale);
        let eq78_e1139_d_n3: f64 = (s.dn[192][3] * ddt_scale);
        let eq78_e1139_d_n4: f64 = (s.dn[192][4] * ddt_scale);
        let eq78_e1139_d_n5: f64 = (s.dn[192][5] * ddt_scale);
        let eq78_e1139_d_n6: f64 = (s.dn[192][6] * ddt_scale);
        let eq78_e1139_d_n7: f64 = (s.dn[192][7] * ddt_scale);
        let eq78_e1139_d_n8: f64 = (s.dn[192][8] * ddt_scale);
        let eq78_e1139_d_n9: f64 = (s.dn[192][9] * ddt_scale);
        let eq78_e1139_d_n10: f64 = (s.dn[192][10] * ddt_scale);
        let eq78_e1139_d_n11: f64 = (s.dn[192][11] * ddt_scale);
        let eq78_e1139_d_n12: f64 = (s.dn[192][12] * ddt_scale);
        let eq78_e1139_d_n13: f64 = (s.dn[192][13] * ddt_scale);
        let eq78_e1139_d_n14: f64 = (s.dn[192][14] * ddt_scale);
        let eq78_e1139_d_n15: f64 = (s.dn[192][15] * ddt_scale);
        let eq78_e1139_d_n16: f64 = (s.dn[192][16] * ddt_scale);
        let eq78_e1139_d_n17: f64 = (s.dn[192][17] * ddt_scale);
        let eq78_e1139_d_n18: f64 = (s.dn[192][18] * ddt_scale);
        let eq78_e1139_d_n19: f64 = (s.dn[192][19] * ddt_scale);
        let eq78_e1139_d_n20: f64 = (s.dn[192][20] * ddt_scale);
        let eq78_e1139_d_n21: f64 = (s.dn[192][21] * ddt_scale);
        let eq78_e1139_d_n22: f64 = (s.dn[192][22] * ddt_scale);
        let eq78_e1139_d_n23: f64 = (s.dn[192][23] * ddt_scale);
        let eq78_e1139_d_n24: f64 = (s.dn[192][24] * ddt_scale);
        let eq78_e1139_d_n25: f64 = (s.dn[192][25] * ddt_scale);
        let eq78_e1139_d_n26: f64 = (s.dn[192][26] * ddt_scale);
        let eq78_e1139_d_n27: f64 = (s.dn[192][27] * ddt_scale);
        let eq78_e1139_d_n28: f64 = (s.dn[192][28] * ddt_scale);
        let eq78_e1139_d_n29: f64 = (s.dn[192][29] * ddt_scale);
        let eq78_e1142: f64 = (p.p355 * (nv2 - nv14));
        let eq78_e1142_d_n2: f64 = p.p355;
        let eq78_e1142_d_n14: f64 = (-p.p355);
        let eq78_e1143: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 63, eq78_e1142);
        let eq78_e1143_d_n2: f64 = (eq78_e1142_d_n2 * ddt_scale);
        let eq78_e1143_d_n14: f64 = (eq78_e1142_d_n14 * ddt_scale);
        let eq78_e1144: f64 = (eq78_e1139 + eq78_e1143);
        let eq78_e1144_d_n2: f64 = (eq78_e1139_d_n2 + eq78_e1143_d_n2);
        let eq78_e1144_d_n14: f64 = (eq78_e1139_d_n14 + eq78_e1143_d_n14);
        (eq78_e1144, eq78_e1139_d_n0, eq78_e1139_d_n1, eq78_e1144_d_n2, eq78_e1139_d_n3, eq78_e1139_d_n4, eq78_e1139_d_n5, eq78_e1139_d_n6, eq78_e1139_d_n7, eq78_e1139_d_n8, eq78_e1139_d_n9, eq78_e1139_d_n10, eq78_e1139_d_n11, eq78_e1139_d_n12, eq78_e1139_d_n13, eq78_e1144_d_n14, eq78_e1139_d_n15, eq78_e1139_d_n16, eq78_e1139_d_n17, eq78_e1139_d_n18, eq78_e1139_d_n19, eq78_e1139_d_n20, eq78_e1139_d_n21, eq78_e1139_d_n22, eq78_e1139_d_n23, eq78_e1139_d_n24, eq78_e1139_d_n25, eq78_e1139_d_n26, eq78_e1139_d_n27, eq78_e1139_d_n28, eq78_e1139_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_value: f64 = eq78_e1146;
        let eq78_node_derivatives: [f64; 30] = [eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29];
        let eq78_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            multiplicity * (eq78_value),
            nodes,
            &eq78_node_derivatives,
            branches,
            &eq78_branch_derivatives,
            multiplicity,
        );
        let (eq79_e1157, eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29,) = {
    if (!s.b[907]) {
        let eq79_e1150: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 64, s.v[193]);
        let eq79_e1150_d_n0: f64 = (s.dn[193][0] * ddt_scale);
        let eq79_e1150_d_n1: f64 = (s.dn[193][1] * ddt_scale);
        let eq79_e1150_d_n2: f64 = (s.dn[193][2] * ddt_scale);
        let eq79_e1150_d_n3: f64 = (s.dn[193][3] * ddt_scale);
        let eq79_e1150_d_n4: f64 = (s.dn[193][4] * ddt_scale);
        let eq79_e1150_d_n5: f64 = (s.dn[193][5] * ddt_scale);
        let eq79_e1150_d_n6: f64 = (s.dn[193][6] * ddt_scale);
        let eq79_e1150_d_n7: f64 = (s.dn[193][7] * ddt_scale);
        let eq79_e1150_d_n8: f64 = (s.dn[193][8] * ddt_scale);
        let eq79_e1150_d_n9: f64 = (s.dn[193][9] * ddt_scale);
        let eq79_e1150_d_n10: f64 = (s.dn[193][10] * ddt_scale);
        let eq79_e1150_d_n11: f64 = (s.dn[193][11] * ddt_scale);
        let eq79_e1150_d_n12: f64 = (s.dn[193][12] * ddt_scale);
        let eq79_e1150_d_n13: f64 = (s.dn[193][13] * ddt_scale);
        let eq79_e1150_d_n14: f64 = (s.dn[193][14] * ddt_scale);
        let eq79_e1150_d_n15: f64 = (s.dn[193][15] * ddt_scale);
        let eq79_e1150_d_n16: f64 = (s.dn[193][16] * ddt_scale);
        let eq79_e1150_d_n17: f64 = (s.dn[193][17] * ddt_scale);
        let eq79_e1150_d_n18: f64 = (s.dn[193][18] * ddt_scale);
        let eq79_e1150_d_n19: f64 = (s.dn[193][19] * ddt_scale);
        let eq79_e1150_d_n20: f64 = (s.dn[193][20] * ddt_scale);
        let eq79_e1150_d_n21: f64 = (s.dn[193][21] * ddt_scale);
        let eq79_e1150_d_n22: f64 = (s.dn[193][22] * ddt_scale);
        let eq79_e1150_d_n23: f64 = (s.dn[193][23] * ddt_scale);
        let eq79_e1150_d_n24: f64 = (s.dn[193][24] * ddt_scale);
        let eq79_e1150_d_n25: f64 = (s.dn[193][25] * ddt_scale);
        let eq79_e1150_d_n26: f64 = (s.dn[193][26] * ddt_scale);
        let eq79_e1150_d_n27: f64 = (s.dn[193][27] * ddt_scale);
        let eq79_e1150_d_n28: f64 = (s.dn[193][28] * ddt_scale);
        let eq79_e1150_d_n29: f64 = (s.dn[193][29] * ddt_scale);
        let eq79_e1153: f64 = (p.p355 * (nv7 - nv5));
        let eq79_e1153_d_n5: f64 = (-p.p355);
        let eq79_e1153_d_n7: f64 = p.p355;
        let eq79_e1154: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 65, eq79_e1153);
        let eq79_e1154_d_n5: f64 = (eq79_e1153_d_n5 * ddt_scale);
        let eq79_e1154_d_n7: f64 = (eq79_e1153_d_n7 * ddt_scale);
        let eq79_e1155: f64 = (eq79_e1150 + eq79_e1154);
        let eq79_e1155_d_n5: f64 = (eq79_e1150_d_n5 + eq79_e1154_d_n5);
        let eq79_e1155_d_n7: f64 = (eq79_e1150_d_n7 + eq79_e1154_d_n7);
        (eq79_e1155, eq79_e1150_d_n0, eq79_e1150_d_n1, eq79_e1150_d_n2, eq79_e1150_d_n3, eq79_e1150_d_n4, eq79_e1155_d_n5, eq79_e1150_d_n6, eq79_e1155_d_n7, eq79_e1150_d_n8, eq79_e1150_d_n9, eq79_e1150_d_n10, eq79_e1150_d_n11, eq79_e1150_d_n12, eq79_e1150_d_n13, eq79_e1150_d_n14, eq79_e1150_d_n15, eq79_e1150_d_n16, eq79_e1150_d_n17, eq79_e1150_d_n18, eq79_e1150_d_n19, eq79_e1150_d_n20, eq79_e1150_d_n21, eq79_e1150_d_n22, eq79_e1150_d_n23, eq79_e1150_d_n24, eq79_e1150_d_n25, eq79_e1150_d_n26, eq79_e1150_d_n27, eq79_e1150_d_n28, eq79_e1150_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_value: f64 = eq79_e1157;
        let eq79_node_derivatives: [f64; 30] = [eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29];
        let eq79_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            multiplicity * (eq79_value),
            nodes,
            &eq79_node_derivatives,
            branches,
            &eq79_branch_derivatives,
            multiplicity,
        );
        let (eq80_e1162,) = {
    if (!s.b[907]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq80_value: f64 = eq80_e1162;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[14]),
            multiplicity * (eq80_value),
        );
        let (eq81_e1167,) = {
    if (!s.b[907]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq81_value: f64 = eq81_e1167;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq81_value),
        );
    }

    pub(super) fn stamp_transient_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq82_e1169: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 66, s.v[194]);
        let eq82_e1169_d_n0: f64 = (s.dn[194][0] * ddt_scale);
        let eq82_e1169_d_n1: f64 = (s.dn[194][1] * ddt_scale);
        let eq82_e1169_d_n2: f64 = (s.dn[194][2] * ddt_scale);
        let eq82_e1169_d_n3: f64 = (s.dn[194][3] * ddt_scale);
        let eq82_e1169_d_n4: f64 = (s.dn[194][4] * ddt_scale);
        let eq82_e1169_d_n5: f64 = (s.dn[194][5] * ddt_scale);
        let eq82_e1169_d_n6: f64 = (s.dn[194][6] * ddt_scale);
        let eq82_e1169_d_n7: f64 = (s.dn[194][7] * ddt_scale);
        let eq82_e1169_d_n8: f64 = (s.dn[194][8] * ddt_scale);
        let eq82_e1169_d_n9: f64 = (s.dn[194][9] * ddt_scale);
        let eq82_e1169_d_n10: f64 = (s.dn[194][10] * ddt_scale);
        let eq82_e1169_d_n11: f64 = (s.dn[194][11] * ddt_scale);
        let eq82_e1169_d_n12: f64 = (s.dn[194][12] * ddt_scale);
        let eq82_e1169_d_n13: f64 = (s.dn[194][13] * ddt_scale);
        let eq82_e1169_d_n14: f64 = (s.dn[194][14] * ddt_scale);
        let eq82_e1169_d_n15: f64 = (s.dn[194][15] * ddt_scale);
        let eq82_e1169_d_n16: f64 = (s.dn[194][16] * ddt_scale);
        let eq82_e1169_d_n17: f64 = (s.dn[194][17] * ddt_scale);
        let eq82_e1169_d_n18: f64 = (s.dn[194][18] * ddt_scale);
        let eq82_e1169_d_n19: f64 = (s.dn[194][19] * ddt_scale);
        let eq82_e1169_d_n20: f64 = (s.dn[194][20] * ddt_scale);
        let eq82_e1169_d_n21: f64 = (s.dn[194][21] * ddt_scale);
        let eq82_e1169_d_n22: f64 = (s.dn[194][22] * ddt_scale);
        let eq82_e1169_d_n23: f64 = (s.dn[194][23] * ddt_scale);
        let eq82_e1169_d_n24: f64 = (s.dn[194][24] * ddt_scale);
        let eq82_e1169_d_n25: f64 = (s.dn[194][25] * ddt_scale);
        let eq82_e1169_d_n26: f64 = (s.dn[194][26] * ddt_scale);
        let eq82_e1169_d_n27: f64 = (s.dn[194][27] * ddt_scale);
        let eq82_e1169_d_n28: f64 = (s.dn[194][28] * ddt_scale);
        let eq82_e1169_d_n29: f64 = (s.dn[194][29] * ddt_scale);
        let eq82_e1172: f64 = (p.p355 * (nv3 - nv5));
        let eq82_e1172_d_n3: f64 = p.p355;
        let eq82_e1172_d_n5: f64 = (-p.p355);
        let eq82_e1173: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 67, eq82_e1172);
        let eq82_e1173_d_n3: f64 = (eq82_e1172_d_n3 * ddt_scale);
        let eq82_e1173_d_n5: f64 = (eq82_e1172_d_n5 * ddt_scale);
        let eq82_e1174: f64 = (eq82_e1169 + eq82_e1173);
        let eq82_e1174_d_n3: f64 = (eq82_e1169_d_n3 + eq82_e1173_d_n3);
        let eq82_e1174_d_n5: f64 = (eq82_e1169_d_n5 + eq82_e1173_d_n5);
        let eq82_value: f64 = eq82_e1174;
        let eq82_node_derivatives: [f64; 30] = [eq82_e1169_d_n0, eq82_e1169_d_n1, eq82_e1169_d_n2, eq82_e1174_d_n3, eq82_e1169_d_n4, eq82_e1174_d_n5, eq82_e1169_d_n6, eq82_e1169_d_n7, eq82_e1169_d_n8, eq82_e1169_d_n9, eq82_e1169_d_n10, eq82_e1169_d_n11, eq82_e1169_d_n12, eq82_e1169_d_n13, eq82_e1169_d_n14, eq82_e1169_d_n15, eq82_e1169_d_n16, eq82_e1169_d_n17, eq82_e1169_d_n18, eq82_e1169_d_n19, eq82_e1169_d_n20, eq82_e1169_d_n21, eq82_e1169_d_n22, eq82_e1169_d_n23, eq82_e1169_d_n24, eq82_e1169_d_n25, eq82_e1169_d_n26, eq82_e1169_d_n27, eq82_e1169_d_n28, eq82_e1169_d_n29];
        let eq82_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            multiplicity * (eq82_value),
            nodes,
            &eq82_node_derivatives,
            branches,
            &eq82_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1182, eq83_e1182_d_n0, eq83_e1182_d_n1, eq83_e1182_d_n2, eq83_e1182_d_n3, eq83_e1182_d_n4, eq83_e1182_d_n5, eq83_e1182_d_n6, eq83_e1182_d_n7, eq83_e1182_d_n8, eq83_e1182_d_n9, eq83_e1182_d_n10, eq83_e1182_d_n11, eq83_e1182_d_n12, eq83_e1182_d_n13, eq83_e1182_d_n14, eq83_e1182_d_n15, eq83_e1182_d_n16, eq83_e1182_d_n17, eq83_e1182_d_n18, eq83_e1182_d_n19, eq83_e1182_d_n20, eq83_e1182_d_n21, eq83_e1182_d_n22, eq83_e1182_d_n23, eq83_e1182_d_n24, eq83_e1182_d_n25, eq83_e1182_d_n26, eq83_e1182_d_n27, eq83_e1182_d_n28, eq83_e1182_d_n29,) = {
    if s.b[908] {
        let eq83_e1179: f64 = (s.v[0] * (nv9 - nv10));
        let eq83_e1179_d_n9: f64 = s.v[0];
        let eq83_e1179_d_n10: f64 = (-s.v[0]);
        let eq83_e1180: f64 = (s.v[166] + eq83_e1179);
        let eq83_e1180_d_n9: f64 = (s.dn[166][9] + eq83_e1179_d_n9);
        let eq83_e1180_d_n10: f64 = (s.dn[166][10] + eq83_e1179_d_n10);
        (eq83_e1180, s.dn[166][0], s.dn[166][1], s.dn[166][2], s.dn[166][3], s.dn[166][4], s.dn[166][5], s.dn[166][6], s.dn[166][7], s.dn[166][8], eq83_e1180_d_n9, eq83_e1180_d_n10, s.dn[166][11], s.dn[166][12], s.dn[166][13], s.dn[166][14], s.dn[166][15], s.dn[166][16], s.dn[166][17], s.dn[166][18], s.dn[166][19], s.dn[166][20], s.dn[166][21], s.dn[166][22], s.dn[166][23], s.dn[166][24], s.dn[166][25], s.dn[166][26], s.dn[166][27], s.dn[166][28], s.dn[166][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1182;
        let eq83_node_derivatives: [f64; 30] = [eq83_e1182_d_n0, eq83_e1182_d_n1, eq83_e1182_d_n2, eq83_e1182_d_n3, eq83_e1182_d_n4, eq83_e1182_d_n5, eq83_e1182_d_n6, eq83_e1182_d_n7, eq83_e1182_d_n8, eq83_e1182_d_n9, eq83_e1182_d_n10, eq83_e1182_d_n11, eq83_e1182_d_n12, eq83_e1182_d_n13, eq83_e1182_d_n14, eq83_e1182_d_n15, eq83_e1182_d_n16, eq83_e1182_d_n17, eq83_e1182_d_n18, eq83_e1182_d_n19, eq83_e1182_d_n20, eq83_e1182_d_n21, eq83_e1182_d_n22, eq83_e1182_d_n23, eq83_e1182_d_n24, eq83_e1182_d_n25, eq83_e1182_d_n26, eq83_e1182_d_n27, eq83_e1182_d_n28, eq83_e1182_d_n29];
        let eq83_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[10]),
            multiplicity * (eq83_value),
            nodes,
            &eq83_node_derivatives,
            branches,
            &eq83_branch_derivatives,
            multiplicity,
        );
        let (eq84_e1187,) = {
    if (!s.b[908]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq84_value: f64 = eq84_e1187;
        stamper.stamp_potential_const(
            branches[22],
            eq84_value,
        );
        let (eq85_e1197, eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29,) = {
    if s.b[1054] {
        let eq85_e1190: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 68, s.v[167]);
        let eq85_e1190_d_n0: f64 = (s.dn[167][0] * ddt_scale);
        let eq85_e1190_d_n1: f64 = (s.dn[167][1] * ddt_scale);
        let eq85_e1190_d_n2: f64 = (s.dn[167][2] * ddt_scale);
        let eq85_e1190_d_n3: f64 = (s.dn[167][3] * ddt_scale);
        let eq85_e1190_d_n4: f64 = (s.dn[167][4] * ddt_scale);
        let eq85_e1190_d_n5: f64 = (s.dn[167][5] * ddt_scale);
        let eq85_e1190_d_n6: f64 = (s.dn[167][6] * ddt_scale);
        let eq85_e1190_d_n7: f64 = (s.dn[167][7] * ddt_scale);
        let eq85_e1190_d_n8: f64 = (s.dn[167][8] * ddt_scale);
        let eq85_e1190_d_n9: f64 = (s.dn[167][9] * ddt_scale);
        let eq85_e1190_d_n10: f64 = (s.dn[167][10] * ddt_scale);
        let eq85_e1190_d_n11: f64 = (s.dn[167][11] * ddt_scale);
        let eq85_e1190_d_n12: f64 = (s.dn[167][12] * ddt_scale);
        let eq85_e1190_d_n13: f64 = (s.dn[167][13] * ddt_scale);
        let eq85_e1190_d_n14: f64 = (s.dn[167][14] * ddt_scale);
        let eq85_e1190_d_n15: f64 = (s.dn[167][15] * ddt_scale);
        let eq85_e1190_d_n16: f64 = (s.dn[167][16] * ddt_scale);
        let eq85_e1190_d_n17: f64 = (s.dn[167][17] * ddt_scale);
        let eq85_e1190_d_n18: f64 = (s.dn[167][18] * ddt_scale);
        let eq85_e1190_d_n19: f64 = (s.dn[167][19] * ddt_scale);
        let eq85_e1190_d_n20: f64 = (s.dn[167][20] * ddt_scale);
        let eq85_e1190_d_n21: f64 = (s.dn[167][21] * ddt_scale);
        let eq85_e1190_d_n22: f64 = (s.dn[167][22] * ddt_scale);
        let eq85_e1190_d_n23: f64 = (s.dn[167][23] * ddt_scale);
        let eq85_e1190_d_n24: f64 = (s.dn[167][24] * ddt_scale);
        let eq85_e1190_d_n25: f64 = (s.dn[167][25] * ddt_scale);
        let eq85_e1190_d_n26: f64 = (s.dn[167][26] * ddt_scale);
        let eq85_e1190_d_n27: f64 = (s.dn[167][27] * ddt_scale);
        let eq85_e1190_d_n28: f64 = (s.dn[167][28] * ddt_scale);
        let eq85_e1190_d_n29: f64 = (s.dn[167][29] * ddt_scale);
        let eq85_e1193: f64 = (p.p355 * (nv7 - nv10));
        let eq85_e1193_d_n7: f64 = p.p355;
        let eq85_e1193_d_n10: f64 = (-p.p355);
        let eq85_e1194: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 69, eq85_e1193);
        let eq85_e1194_d_n7: f64 = (eq85_e1193_d_n7 * ddt_scale);
        let eq85_e1194_d_n10: f64 = (eq85_e1193_d_n10 * ddt_scale);
        let eq85_e1195: f64 = (eq85_e1190 + eq85_e1194);
        let eq85_e1195_d_n7: f64 = (eq85_e1190_d_n7 + eq85_e1194_d_n7);
        let eq85_e1195_d_n10: f64 = (eq85_e1190_d_n10 + eq85_e1194_d_n10);
        (eq85_e1195, eq85_e1190_d_n0, eq85_e1190_d_n1, eq85_e1190_d_n2, eq85_e1190_d_n3, eq85_e1190_d_n4, eq85_e1190_d_n5, eq85_e1190_d_n6, eq85_e1195_d_n7, eq85_e1190_d_n8, eq85_e1190_d_n9, eq85_e1195_d_n10, eq85_e1190_d_n11, eq85_e1190_d_n12, eq85_e1190_d_n13, eq85_e1190_d_n14, eq85_e1190_d_n15, eq85_e1190_d_n16, eq85_e1190_d_n17, eq85_e1190_d_n18, eq85_e1190_d_n19, eq85_e1190_d_n20, eq85_e1190_d_n21, eq85_e1190_d_n22, eq85_e1190_d_n23, eq85_e1190_d_n24, eq85_e1190_d_n25, eq85_e1190_d_n26, eq85_e1190_d_n27, eq85_e1190_d_n28, eq85_e1190_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq85_value: f64 = eq85_e1197;
        let eq85_node_derivatives: [f64; 30] = [eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29];
        let eq85_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            multiplicity * (eq85_value),
            nodes,
            &eq85_node_derivatives,
            branches,
            &eq85_branch_derivatives,
            multiplicity,
        );
        let (eq86_e1207, eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29,) = {
    if s.b[1054] {
        let eq86_e1200: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 70, s.v[168]);
        let eq86_e1200_d_n0: f64 = (s.dn[168][0] * ddt_scale);
        let eq86_e1200_d_n1: f64 = (s.dn[168][1] * ddt_scale);
        let eq86_e1200_d_n2: f64 = (s.dn[168][2] * ddt_scale);
        let eq86_e1200_d_n3: f64 = (s.dn[168][3] * ddt_scale);
        let eq86_e1200_d_n4: f64 = (s.dn[168][4] * ddt_scale);
        let eq86_e1200_d_n5: f64 = (s.dn[168][5] * ddt_scale);
        let eq86_e1200_d_n6: f64 = (s.dn[168][6] * ddt_scale);
        let eq86_e1200_d_n7: f64 = (s.dn[168][7] * ddt_scale);
        let eq86_e1200_d_n8: f64 = (s.dn[168][8] * ddt_scale);
        let eq86_e1200_d_n9: f64 = (s.dn[168][9] * ddt_scale);
        let eq86_e1200_d_n10: f64 = (s.dn[168][10] * ddt_scale);
        let eq86_e1200_d_n11: f64 = (s.dn[168][11] * ddt_scale);
        let eq86_e1200_d_n12: f64 = (s.dn[168][12] * ddt_scale);
        let eq86_e1200_d_n13: f64 = (s.dn[168][13] * ddt_scale);
        let eq86_e1200_d_n14: f64 = (s.dn[168][14] * ddt_scale);
        let eq86_e1200_d_n15: f64 = (s.dn[168][15] * ddt_scale);
        let eq86_e1200_d_n16: f64 = (s.dn[168][16] * ddt_scale);
        let eq86_e1200_d_n17: f64 = (s.dn[168][17] * ddt_scale);
        let eq86_e1200_d_n18: f64 = (s.dn[168][18] * ddt_scale);
        let eq86_e1200_d_n19: f64 = (s.dn[168][19] * ddt_scale);
        let eq86_e1200_d_n20: f64 = (s.dn[168][20] * ddt_scale);
        let eq86_e1200_d_n21: f64 = (s.dn[168][21] * ddt_scale);
        let eq86_e1200_d_n22: f64 = (s.dn[168][22] * ddt_scale);
        let eq86_e1200_d_n23: f64 = (s.dn[168][23] * ddt_scale);
        let eq86_e1200_d_n24: f64 = (s.dn[168][24] * ddt_scale);
        let eq86_e1200_d_n25: f64 = (s.dn[168][25] * ddt_scale);
        let eq86_e1200_d_n26: f64 = (s.dn[168][26] * ddt_scale);
        let eq86_e1200_d_n27: f64 = (s.dn[168][27] * ddt_scale);
        let eq86_e1200_d_n28: f64 = (s.dn[168][28] * ddt_scale);
        let eq86_e1200_d_n29: f64 = (s.dn[168][29] * ddt_scale);
        let eq86_e1203: f64 = (p.p355 * (nv7 - nv9));
        let eq86_e1203_d_n7: f64 = p.p355;
        let eq86_e1203_d_n9: f64 = (-p.p355);
        let eq86_e1204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 71, eq86_e1203);
        let eq86_e1204_d_n7: f64 = (eq86_e1203_d_n7 * ddt_scale);
        let eq86_e1204_d_n9: f64 = (eq86_e1203_d_n9 * ddt_scale);
        let eq86_e1205: f64 = (eq86_e1200 + eq86_e1204);
        let eq86_e1205_d_n7: f64 = (eq86_e1200_d_n7 + eq86_e1204_d_n7);
        let eq86_e1205_d_n9: f64 = (eq86_e1200_d_n9 + eq86_e1204_d_n9);
        (eq86_e1205, eq86_e1200_d_n0, eq86_e1200_d_n1, eq86_e1200_d_n2, eq86_e1200_d_n3, eq86_e1200_d_n4, eq86_e1200_d_n5, eq86_e1200_d_n6, eq86_e1205_d_n7, eq86_e1200_d_n8, eq86_e1205_d_n9, eq86_e1200_d_n10, eq86_e1200_d_n11, eq86_e1200_d_n12, eq86_e1200_d_n13, eq86_e1200_d_n14, eq86_e1200_d_n15, eq86_e1200_d_n16, eq86_e1200_d_n17, eq86_e1200_d_n18, eq86_e1200_d_n19, eq86_e1200_d_n20, eq86_e1200_d_n21, eq86_e1200_d_n22, eq86_e1200_d_n23, eq86_e1200_d_n24, eq86_e1200_d_n25, eq86_e1200_d_n26, eq86_e1200_d_n27, eq86_e1200_d_n28, eq86_e1200_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_value: f64 = eq86_e1207;
        let eq86_node_derivatives: [f64; 30] = [eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29];
        let eq86_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq86_value),
            nodes,
            &eq86_node_derivatives,
            branches,
            &eq86_branch_derivatives,
            multiplicity,
        );
        let (eq87_e1217, eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29,) = {
    if s.b[1054] {
        let eq87_e1210: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 72, s.v[169]);
        let eq87_e1210_d_n0: f64 = (s.dn[169][0] * ddt_scale);
        let eq87_e1210_d_n1: f64 = (s.dn[169][1] * ddt_scale);
        let eq87_e1210_d_n2: f64 = (s.dn[169][2] * ddt_scale);
        let eq87_e1210_d_n3: f64 = (s.dn[169][3] * ddt_scale);
        let eq87_e1210_d_n4: f64 = (s.dn[169][4] * ddt_scale);
        let eq87_e1210_d_n5: f64 = (s.dn[169][5] * ddt_scale);
        let eq87_e1210_d_n6: f64 = (s.dn[169][6] * ddt_scale);
        let eq87_e1210_d_n7: f64 = (s.dn[169][7] * ddt_scale);
        let eq87_e1210_d_n8: f64 = (s.dn[169][8] * ddt_scale);
        let eq87_e1210_d_n9: f64 = (s.dn[169][9] * ddt_scale);
        let eq87_e1210_d_n10: f64 = (s.dn[169][10] * ddt_scale);
        let eq87_e1210_d_n11: f64 = (s.dn[169][11] * ddt_scale);
        let eq87_e1210_d_n12: f64 = (s.dn[169][12] * ddt_scale);
        let eq87_e1210_d_n13: f64 = (s.dn[169][13] * ddt_scale);
        let eq87_e1210_d_n14: f64 = (s.dn[169][14] * ddt_scale);
        let eq87_e1210_d_n15: f64 = (s.dn[169][15] * ddt_scale);
        let eq87_e1210_d_n16: f64 = (s.dn[169][16] * ddt_scale);
        let eq87_e1210_d_n17: f64 = (s.dn[169][17] * ddt_scale);
        let eq87_e1210_d_n18: f64 = (s.dn[169][18] * ddt_scale);
        let eq87_e1210_d_n19: f64 = (s.dn[169][19] * ddt_scale);
        let eq87_e1210_d_n20: f64 = (s.dn[169][20] * ddt_scale);
        let eq87_e1210_d_n21: f64 = (s.dn[169][21] * ddt_scale);
        let eq87_e1210_d_n22: f64 = (s.dn[169][22] * ddt_scale);
        let eq87_e1210_d_n23: f64 = (s.dn[169][23] * ddt_scale);
        let eq87_e1210_d_n24: f64 = (s.dn[169][24] * ddt_scale);
        let eq87_e1210_d_n25: f64 = (s.dn[169][25] * ddt_scale);
        let eq87_e1210_d_n26: f64 = (s.dn[169][26] * ddt_scale);
        let eq87_e1210_d_n27: f64 = (s.dn[169][27] * ddt_scale);
        let eq87_e1210_d_n28: f64 = (s.dn[169][28] * ddt_scale);
        let eq87_e1210_d_n29: f64 = (s.dn[169][29] * ddt_scale);
        let eq87_e1213: f64 = (p.p355 * (nv2 - nv10));
        let eq87_e1213_d_n2: f64 = p.p355;
        let eq87_e1213_d_n10: f64 = (-p.p355);
        let eq87_e1214: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 73, eq87_e1213);
        let eq87_e1214_d_n2: f64 = (eq87_e1213_d_n2 * ddt_scale);
        let eq87_e1214_d_n10: f64 = (eq87_e1213_d_n10 * ddt_scale);
        let eq87_e1215: f64 = (eq87_e1210 + eq87_e1214);
        let eq87_e1215_d_n2: f64 = (eq87_e1210_d_n2 + eq87_e1214_d_n2);
        let eq87_e1215_d_n10: f64 = (eq87_e1210_d_n10 + eq87_e1214_d_n10);
        (eq87_e1215, eq87_e1210_d_n0, eq87_e1210_d_n1, eq87_e1215_d_n2, eq87_e1210_d_n3, eq87_e1210_d_n4, eq87_e1210_d_n5, eq87_e1210_d_n6, eq87_e1210_d_n7, eq87_e1210_d_n8, eq87_e1210_d_n9, eq87_e1215_d_n10, eq87_e1210_d_n11, eq87_e1210_d_n12, eq87_e1210_d_n13, eq87_e1210_d_n14, eq87_e1210_d_n15, eq87_e1210_d_n16, eq87_e1210_d_n17, eq87_e1210_d_n18, eq87_e1210_d_n19, eq87_e1210_d_n20, eq87_e1210_d_n21, eq87_e1210_d_n22, eq87_e1210_d_n23, eq87_e1210_d_n24, eq87_e1210_d_n25, eq87_e1210_d_n26, eq87_e1210_d_n27, eq87_e1210_d_n28, eq87_e1210_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq87_value: f64 = eq87_e1217;
        let eq87_node_derivatives: [f64; 30] = [eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29];
        let eq87_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            multiplicity * (eq87_value),
            nodes,
            &eq87_node_derivatives,
            branches,
            &eq87_branch_derivatives,
            multiplicity,
        );
        let (eq88_e1221,) = {
    if s.b[1054] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq88_value: f64 = eq88_e1221;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[9]),
            multiplicity * (eq88_value),
        );
        let (eq89_e1231, eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29,) = {
    if s.b[1054] {
        let eq89_e1224: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 74, s.v[171]);
        let eq89_e1224_d_n0: f64 = (s.dn[171][0] * ddt_scale);
        let eq89_e1224_d_n1: f64 = (s.dn[171][1] * ddt_scale);
        let eq89_e1224_d_n2: f64 = (s.dn[171][2] * ddt_scale);
        let eq89_e1224_d_n3: f64 = (s.dn[171][3] * ddt_scale);
        let eq89_e1224_d_n4: f64 = (s.dn[171][4] * ddt_scale);
        let eq89_e1224_d_n5: f64 = (s.dn[171][5] * ddt_scale);
        let eq89_e1224_d_n6: f64 = (s.dn[171][6] * ddt_scale);
        let eq89_e1224_d_n7: f64 = (s.dn[171][7] * ddt_scale);
        let eq89_e1224_d_n8: f64 = (s.dn[171][8] * ddt_scale);
        let eq89_e1224_d_n9: f64 = (s.dn[171][9] * ddt_scale);
        let eq89_e1224_d_n10: f64 = (s.dn[171][10] * ddt_scale);
        let eq89_e1224_d_n11: f64 = (s.dn[171][11] * ddt_scale);
        let eq89_e1224_d_n12: f64 = (s.dn[171][12] * ddt_scale);
        let eq89_e1224_d_n13: f64 = (s.dn[171][13] * ddt_scale);
        let eq89_e1224_d_n14: f64 = (s.dn[171][14] * ddt_scale);
        let eq89_e1224_d_n15: f64 = (s.dn[171][15] * ddt_scale);
        let eq89_e1224_d_n16: f64 = (s.dn[171][16] * ddt_scale);
        let eq89_e1224_d_n17: f64 = (s.dn[171][17] * ddt_scale);
        let eq89_e1224_d_n18: f64 = (s.dn[171][18] * ddt_scale);
        let eq89_e1224_d_n19: f64 = (s.dn[171][19] * ddt_scale);
        let eq89_e1224_d_n20: f64 = (s.dn[171][20] * ddt_scale);
        let eq89_e1224_d_n21: f64 = (s.dn[171][21] * ddt_scale);
        let eq89_e1224_d_n22: f64 = (s.dn[171][22] * ddt_scale);
        let eq89_e1224_d_n23: f64 = (s.dn[171][23] * ddt_scale);
        let eq89_e1224_d_n24: f64 = (s.dn[171][24] * ddt_scale);
        let eq89_e1224_d_n25: f64 = (s.dn[171][25] * ddt_scale);
        let eq89_e1224_d_n26: f64 = (s.dn[171][26] * ddt_scale);
        let eq89_e1224_d_n27: f64 = (s.dn[171][27] * ddt_scale);
        let eq89_e1224_d_n28: f64 = (s.dn[171][28] * ddt_scale);
        let eq89_e1224_d_n29: f64 = (s.dn[171][29] * ddt_scale);
        let eq89_e1227: f64 = (p.p355 * (nv7 - nv9));
        let eq89_e1227_d_n7: f64 = p.p355;
        let eq89_e1227_d_n9: f64 = (-p.p355);
        let eq89_e1228: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 75, eq89_e1227);
        let eq89_e1228_d_n7: f64 = (eq89_e1227_d_n7 * ddt_scale);
        let eq89_e1228_d_n9: f64 = (eq89_e1227_d_n9 * ddt_scale);
        let eq89_e1229: f64 = (eq89_e1224 + eq89_e1228);
        let eq89_e1229_d_n7: f64 = (eq89_e1224_d_n7 + eq89_e1228_d_n7);
        let eq89_e1229_d_n9: f64 = (eq89_e1224_d_n9 + eq89_e1228_d_n9);
        (eq89_e1229, eq89_e1224_d_n0, eq89_e1224_d_n1, eq89_e1224_d_n2, eq89_e1224_d_n3, eq89_e1224_d_n4, eq89_e1224_d_n5, eq89_e1224_d_n6, eq89_e1229_d_n7, eq89_e1224_d_n8, eq89_e1229_d_n9, eq89_e1224_d_n10, eq89_e1224_d_n11, eq89_e1224_d_n12, eq89_e1224_d_n13, eq89_e1224_d_n14, eq89_e1224_d_n15, eq89_e1224_d_n16, eq89_e1224_d_n17, eq89_e1224_d_n18, eq89_e1224_d_n19, eq89_e1224_d_n20, eq89_e1224_d_n21, eq89_e1224_d_n22, eq89_e1224_d_n23, eq89_e1224_d_n24, eq89_e1224_d_n25, eq89_e1224_d_n26, eq89_e1224_d_n27, eq89_e1224_d_n28, eq89_e1224_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_value: f64 = eq89_e1231;
        let eq89_node_derivatives: [f64; 30] = [eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29];
        let eq89_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq89_value),
            nodes,
            &eq89_node_derivatives,
            branches,
            &eq89_branch_derivatives,
            multiplicity,
        );
        let (eq90_e1242, eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29,) = {
    if (!s.b[1054]) {
        let eq90_e1235: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 76, s.v[167]);
        let eq90_e1235_d_n0: f64 = (s.dn[167][0] * ddt_scale);
        let eq90_e1235_d_n1: f64 = (s.dn[167][1] * ddt_scale);
        let eq90_e1235_d_n2: f64 = (s.dn[167][2] * ddt_scale);
        let eq90_e1235_d_n3: f64 = (s.dn[167][3] * ddt_scale);
        let eq90_e1235_d_n4: f64 = (s.dn[167][4] * ddt_scale);
        let eq90_e1235_d_n5: f64 = (s.dn[167][5] * ddt_scale);
        let eq90_e1235_d_n6: f64 = (s.dn[167][6] * ddt_scale);
        let eq90_e1235_d_n7: f64 = (s.dn[167][7] * ddt_scale);
        let eq90_e1235_d_n8: f64 = (s.dn[167][8] * ddt_scale);
        let eq90_e1235_d_n9: f64 = (s.dn[167][9] * ddt_scale);
        let eq90_e1235_d_n10: f64 = (s.dn[167][10] * ddt_scale);
        let eq90_e1235_d_n11: f64 = (s.dn[167][11] * ddt_scale);
        let eq90_e1235_d_n12: f64 = (s.dn[167][12] * ddt_scale);
        let eq90_e1235_d_n13: f64 = (s.dn[167][13] * ddt_scale);
        let eq90_e1235_d_n14: f64 = (s.dn[167][14] * ddt_scale);
        let eq90_e1235_d_n15: f64 = (s.dn[167][15] * ddt_scale);
        let eq90_e1235_d_n16: f64 = (s.dn[167][16] * ddt_scale);
        let eq90_e1235_d_n17: f64 = (s.dn[167][17] * ddt_scale);
        let eq90_e1235_d_n18: f64 = (s.dn[167][18] * ddt_scale);
        let eq90_e1235_d_n19: f64 = (s.dn[167][19] * ddt_scale);
        let eq90_e1235_d_n20: f64 = (s.dn[167][20] * ddt_scale);
        let eq90_e1235_d_n21: f64 = (s.dn[167][21] * ddt_scale);
        let eq90_e1235_d_n22: f64 = (s.dn[167][22] * ddt_scale);
        let eq90_e1235_d_n23: f64 = (s.dn[167][23] * ddt_scale);
        let eq90_e1235_d_n24: f64 = (s.dn[167][24] * ddt_scale);
        let eq90_e1235_d_n25: f64 = (s.dn[167][25] * ddt_scale);
        let eq90_e1235_d_n26: f64 = (s.dn[167][26] * ddt_scale);
        let eq90_e1235_d_n27: f64 = (s.dn[167][27] * ddt_scale);
        let eq90_e1235_d_n28: f64 = (s.dn[167][28] * ddt_scale);
        let eq90_e1235_d_n29: f64 = (s.dn[167][29] * ddt_scale);
        let eq90_e1238: f64 = (p.p355 * (nv2 - nv10));
        let eq90_e1238_d_n2: f64 = p.p355;
        let eq90_e1238_d_n10: f64 = (-p.p355);
        let eq90_e1239: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 77, eq90_e1238);
        let eq90_e1239_d_n2: f64 = (eq90_e1238_d_n2 * ddt_scale);
        let eq90_e1239_d_n10: f64 = (eq90_e1238_d_n10 * ddt_scale);
        let eq90_e1240: f64 = (eq90_e1235 + eq90_e1239);
        let eq90_e1240_d_n2: f64 = (eq90_e1235_d_n2 + eq90_e1239_d_n2);
        let eq90_e1240_d_n10: f64 = (eq90_e1235_d_n10 + eq90_e1239_d_n10);
        (eq90_e1240, eq90_e1235_d_n0, eq90_e1235_d_n1, eq90_e1240_d_n2, eq90_e1235_d_n3, eq90_e1235_d_n4, eq90_e1235_d_n5, eq90_e1235_d_n6, eq90_e1235_d_n7, eq90_e1235_d_n8, eq90_e1235_d_n9, eq90_e1240_d_n10, eq90_e1235_d_n11, eq90_e1235_d_n12, eq90_e1235_d_n13, eq90_e1235_d_n14, eq90_e1235_d_n15, eq90_e1235_d_n16, eq90_e1235_d_n17, eq90_e1235_d_n18, eq90_e1235_d_n19, eq90_e1235_d_n20, eq90_e1235_d_n21, eq90_e1235_d_n22, eq90_e1235_d_n23, eq90_e1235_d_n24, eq90_e1235_d_n25, eq90_e1235_d_n26, eq90_e1235_d_n27, eq90_e1235_d_n28, eq90_e1235_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq90_value: f64 = eq90_e1242;
        let eq90_node_derivatives: [f64; 30] = [eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29];
        let eq90_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            multiplicity * (eq90_value),
            nodes,
            &eq90_node_derivatives,
            branches,
            &eq90_branch_derivatives,
            multiplicity,
        );
        let (eq91_e1253, eq91_e1253_d_n0, eq91_e1253_d_n1, eq91_e1253_d_n2, eq91_e1253_d_n3, eq91_e1253_d_n4, eq91_e1253_d_n5, eq91_e1253_d_n6, eq91_e1253_d_n7, eq91_e1253_d_n8, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_d_n11, eq91_e1253_d_n12, eq91_e1253_d_n13, eq91_e1253_d_n14, eq91_e1253_d_n15, eq91_e1253_d_n16, eq91_e1253_d_n17, eq91_e1253_d_n18, eq91_e1253_d_n19, eq91_e1253_d_n20, eq91_e1253_d_n21, eq91_e1253_d_n22, eq91_e1253_d_n23, eq91_e1253_d_n24, eq91_e1253_d_n25, eq91_e1253_d_n26, eq91_e1253_d_n27, eq91_e1253_d_n28, eq91_e1253_d_n29,) = {
    if (!s.b[1054]) {
        let eq91_e1246: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 78, s.v[168]);
        let eq91_e1246_d_n0: f64 = (s.dn[168][0] * ddt_scale);
        let eq91_e1246_d_n1: f64 = (s.dn[168][1] * ddt_scale);
        let eq91_e1246_d_n2: f64 = (s.dn[168][2] * ddt_scale);
        let eq91_e1246_d_n3: f64 = (s.dn[168][3] * ddt_scale);
        let eq91_e1246_d_n4: f64 = (s.dn[168][4] * ddt_scale);
        let eq91_e1246_d_n5: f64 = (s.dn[168][5] * ddt_scale);
        let eq91_e1246_d_n6: f64 = (s.dn[168][6] * ddt_scale);
        let eq91_e1246_d_n7: f64 = (s.dn[168][7] * ddt_scale);
        let eq91_e1246_d_n8: f64 = (s.dn[168][8] * ddt_scale);
        let eq91_e1246_d_n9: f64 = (s.dn[168][9] * ddt_scale);
        let eq91_e1246_d_n10: f64 = (s.dn[168][10] * ddt_scale);
        let eq91_e1246_d_n11: f64 = (s.dn[168][11] * ddt_scale);
        let eq91_e1246_d_n12: f64 = (s.dn[168][12] * ddt_scale);
        let eq91_e1246_d_n13: f64 = (s.dn[168][13] * ddt_scale);
        let eq91_e1246_d_n14: f64 = (s.dn[168][14] * ddt_scale);
        let eq91_e1246_d_n15: f64 = (s.dn[168][15] * ddt_scale);
        let eq91_e1246_d_n16: f64 = (s.dn[168][16] * ddt_scale);
        let eq91_e1246_d_n17: f64 = (s.dn[168][17] * ddt_scale);
        let eq91_e1246_d_n18: f64 = (s.dn[168][18] * ddt_scale);
        let eq91_e1246_d_n19: f64 = (s.dn[168][19] * ddt_scale);
        let eq91_e1246_d_n20: f64 = (s.dn[168][20] * ddt_scale);
        let eq91_e1246_d_n21: f64 = (s.dn[168][21] * ddt_scale);
        let eq91_e1246_d_n22: f64 = (s.dn[168][22] * ddt_scale);
        let eq91_e1246_d_n23: f64 = (s.dn[168][23] * ddt_scale);
        let eq91_e1246_d_n24: f64 = (s.dn[168][24] * ddt_scale);
        let eq91_e1246_d_n25: f64 = (s.dn[168][25] * ddt_scale);
        let eq91_e1246_d_n26: f64 = (s.dn[168][26] * ddt_scale);
        let eq91_e1246_d_n27: f64 = (s.dn[168][27] * ddt_scale);
        let eq91_e1246_d_n28: f64 = (s.dn[168][28] * ddt_scale);
        let eq91_e1246_d_n29: f64 = (s.dn[168][29] * ddt_scale);
        let eq91_e1249: f64 = (p.p355 * (nv2 - nv9));
        let eq91_e1249_d_n2: f64 = p.p355;
        let eq91_e1249_d_n9: f64 = (-p.p355);
        let eq91_e1250: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 79, eq91_e1249);
        let eq91_e1250_d_n2: f64 = (eq91_e1249_d_n2 * ddt_scale);
        let eq91_e1250_d_n9: f64 = (eq91_e1249_d_n9 * ddt_scale);
        let eq91_e1251: f64 = (eq91_e1246 + eq91_e1250);
        let eq91_e1251_d_n2: f64 = (eq91_e1246_d_n2 + eq91_e1250_d_n2);
        let eq91_e1251_d_n9: f64 = (eq91_e1246_d_n9 + eq91_e1250_d_n9);
        (eq91_e1251, eq91_e1246_d_n0, eq91_e1246_d_n1, eq91_e1251_d_n2, eq91_e1246_d_n3, eq91_e1246_d_n4, eq91_e1246_d_n5, eq91_e1246_d_n6, eq91_e1246_d_n7, eq91_e1246_d_n8, eq91_e1251_d_n9, eq91_e1246_d_n10, eq91_e1246_d_n11, eq91_e1246_d_n12, eq91_e1246_d_n13, eq91_e1246_d_n14, eq91_e1246_d_n15, eq91_e1246_d_n16, eq91_e1246_d_n17, eq91_e1246_d_n18, eq91_e1246_d_n19, eq91_e1246_d_n20, eq91_e1246_d_n21, eq91_e1246_d_n22, eq91_e1246_d_n23, eq91_e1246_d_n24, eq91_e1246_d_n25, eq91_e1246_d_n26, eq91_e1246_d_n27, eq91_e1246_d_n28, eq91_e1246_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq91_value: f64 = eq91_e1253;
        let eq91_node_derivatives: [f64; 30] = [eq91_e1253_d_n0, eq91_e1253_d_n1, eq91_e1253_d_n2, eq91_e1253_d_n3, eq91_e1253_d_n4, eq91_e1253_d_n5, eq91_e1253_d_n6, eq91_e1253_d_n7, eq91_e1253_d_n8, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_d_n11, eq91_e1253_d_n12, eq91_e1253_d_n13, eq91_e1253_d_n14, eq91_e1253_d_n15, eq91_e1253_d_n16, eq91_e1253_d_n17, eq91_e1253_d_n18, eq91_e1253_d_n19, eq91_e1253_d_n20, eq91_e1253_d_n21, eq91_e1253_d_n22, eq91_e1253_d_n23, eq91_e1253_d_n24, eq91_e1253_d_n25, eq91_e1253_d_n26, eq91_e1253_d_n27, eq91_e1253_d_n28, eq91_e1253_d_n29];
        let eq91_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[9]),
            multiplicity * (eq91_value),
            nodes,
            &eq91_node_derivatives,
            branches,
            &eq91_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_8(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq92_e1264, eq92_e1264_d_n0, eq92_e1264_d_n1, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n5, eq92_e1264_d_n6, eq92_e1264_d_n7, eq92_e1264_d_n8, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_d_n11, eq92_e1264_d_n12, eq92_e1264_d_n13, eq92_e1264_d_n14, eq92_e1264_d_n15, eq92_e1264_d_n16, eq92_e1264_d_n17, eq92_e1264_d_n18, eq92_e1264_d_n19, eq92_e1264_d_n20, eq92_e1264_d_n21, eq92_e1264_d_n22, eq92_e1264_d_n23, eq92_e1264_d_n24, eq92_e1264_d_n25, eq92_e1264_d_n26, eq92_e1264_d_n27, eq92_e1264_d_n28, eq92_e1264_d_n29,) = {
    if (!s.b[1054]) {
        let eq92_e1257: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 80, s.v[169]);
        let eq92_e1257_d_n0: f64 = (s.dn[169][0] * ddt_scale);
        let eq92_e1257_d_n1: f64 = (s.dn[169][1] * ddt_scale);
        let eq92_e1257_d_n2: f64 = (s.dn[169][2] * ddt_scale);
        let eq92_e1257_d_n3: f64 = (s.dn[169][3] * ddt_scale);
        let eq92_e1257_d_n4: f64 = (s.dn[169][4] * ddt_scale);
        let eq92_e1257_d_n5: f64 = (s.dn[169][5] * ddt_scale);
        let eq92_e1257_d_n6: f64 = (s.dn[169][6] * ddt_scale);
        let eq92_e1257_d_n7: f64 = (s.dn[169][7] * ddt_scale);
        let eq92_e1257_d_n8: f64 = (s.dn[169][8] * ddt_scale);
        let eq92_e1257_d_n9: f64 = (s.dn[169][9] * ddt_scale);
        let eq92_e1257_d_n10: f64 = (s.dn[169][10] * ddt_scale);
        let eq92_e1257_d_n11: f64 = (s.dn[169][11] * ddt_scale);
        let eq92_e1257_d_n12: f64 = (s.dn[169][12] * ddt_scale);
        let eq92_e1257_d_n13: f64 = (s.dn[169][13] * ddt_scale);
        let eq92_e1257_d_n14: f64 = (s.dn[169][14] * ddt_scale);
        let eq92_e1257_d_n15: f64 = (s.dn[169][15] * ddt_scale);
        let eq92_e1257_d_n16: f64 = (s.dn[169][16] * ddt_scale);
        let eq92_e1257_d_n17: f64 = (s.dn[169][17] * ddt_scale);
        let eq92_e1257_d_n18: f64 = (s.dn[169][18] * ddt_scale);
        let eq92_e1257_d_n19: f64 = (s.dn[169][19] * ddt_scale);
        let eq92_e1257_d_n20: f64 = (s.dn[169][20] * ddt_scale);
        let eq92_e1257_d_n21: f64 = (s.dn[169][21] * ddt_scale);
        let eq92_e1257_d_n22: f64 = (s.dn[169][22] * ddt_scale);
        let eq92_e1257_d_n23: f64 = (s.dn[169][23] * ddt_scale);
        let eq92_e1257_d_n24: f64 = (s.dn[169][24] * ddt_scale);
        let eq92_e1257_d_n25: f64 = (s.dn[169][25] * ddt_scale);
        let eq92_e1257_d_n26: f64 = (s.dn[169][26] * ddt_scale);
        let eq92_e1257_d_n27: f64 = (s.dn[169][27] * ddt_scale);
        let eq92_e1257_d_n28: f64 = (s.dn[169][28] * ddt_scale);
        let eq92_e1257_d_n29: f64 = (s.dn[169][29] * ddt_scale);
        let eq92_e1260: f64 = (p.p355 * (nv7 - nv10));
        let eq92_e1260_d_n7: f64 = p.p355;
        let eq92_e1260_d_n10: f64 = (-p.p355);
        let eq92_e1261: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 81, eq92_e1260);
        let eq92_e1261_d_n7: f64 = (eq92_e1260_d_n7 * ddt_scale);
        let eq92_e1261_d_n10: f64 = (eq92_e1260_d_n10 * ddt_scale);
        let eq92_e1262: f64 = (eq92_e1257 + eq92_e1261);
        let eq92_e1262_d_n7: f64 = (eq92_e1257_d_n7 + eq92_e1261_d_n7);
        let eq92_e1262_d_n10: f64 = (eq92_e1257_d_n10 + eq92_e1261_d_n10);
        (eq92_e1262, eq92_e1257_d_n0, eq92_e1257_d_n1, eq92_e1257_d_n2, eq92_e1257_d_n3, eq92_e1257_d_n4, eq92_e1257_d_n5, eq92_e1257_d_n6, eq92_e1262_d_n7, eq92_e1257_d_n8, eq92_e1257_d_n9, eq92_e1262_d_n10, eq92_e1257_d_n11, eq92_e1257_d_n12, eq92_e1257_d_n13, eq92_e1257_d_n14, eq92_e1257_d_n15, eq92_e1257_d_n16, eq92_e1257_d_n17, eq92_e1257_d_n18, eq92_e1257_d_n19, eq92_e1257_d_n20, eq92_e1257_d_n21, eq92_e1257_d_n22, eq92_e1257_d_n23, eq92_e1257_d_n24, eq92_e1257_d_n25, eq92_e1257_d_n26, eq92_e1257_d_n27, eq92_e1257_d_n28, eq92_e1257_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq92_value: f64 = eq92_e1264;
        let eq92_node_derivatives: [f64; 30] = [eq92_e1264_d_n0, eq92_e1264_d_n1, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n5, eq92_e1264_d_n6, eq92_e1264_d_n7, eq92_e1264_d_n8, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_d_n11, eq92_e1264_d_n12, eq92_e1264_d_n13, eq92_e1264_d_n14, eq92_e1264_d_n15, eq92_e1264_d_n16, eq92_e1264_d_n17, eq92_e1264_d_n18, eq92_e1264_d_n19, eq92_e1264_d_n20, eq92_e1264_d_n21, eq92_e1264_d_n22, eq92_e1264_d_n23, eq92_e1264_d_n24, eq92_e1264_d_n25, eq92_e1264_d_n26, eq92_e1264_d_n27, eq92_e1264_d_n28, eq92_e1264_d_n29];
        let eq92_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            multiplicity * (eq92_value),
            nodes,
            &eq92_node_derivatives,
            branches,
            &eq92_branch_derivatives,
            multiplicity,
        );
        let (eq93_e1269,) = {
    if (!s.b[1054]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq93_value: f64 = eq93_e1269;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq93_value),
        );
        let (eq94_e1274,) = {
    if (!s.b[1054]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq94_value: f64 = eq94_e1274;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq94_value),
        );
        let eq95_e1276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 82, s.v[170]);
        let eq95_e1276_d_n0: f64 = (s.dn[170][0] * ddt_scale);
        let eq95_e1276_d_n1: f64 = (s.dn[170][1] * ddt_scale);
        let eq95_e1276_d_n2: f64 = (s.dn[170][2] * ddt_scale);
        let eq95_e1276_d_n3: f64 = (s.dn[170][3] * ddt_scale);
        let eq95_e1276_d_n4: f64 = (s.dn[170][4] * ddt_scale);
        let eq95_e1276_d_n5: f64 = (s.dn[170][5] * ddt_scale);
        let eq95_e1276_d_n6: f64 = (s.dn[170][6] * ddt_scale);
        let eq95_e1276_d_n7: f64 = (s.dn[170][7] * ddt_scale);
        let eq95_e1276_d_n8: f64 = (s.dn[170][8] * ddt_scale);
        let eq95_e1276_d_n9: f64 = (s.dn[170][9] * ddt_scale);
        let eq95_e1276_d_n10: f64 = (s.dn[170][10] * ddt_scale);
        let eq95_e1276_d_n11: f64 = (s.dn[170][11] * ddt_scale);
        let eq95_e1276_d_n12: f64 = (s.dn[170][12] * ddt_scale);
        let eq95_e1276_d_n13: f64 = (s.dn[170][13] * ddt_scale);
        let eq95_e1276_d_n14: f64 = (s.dn[170][14] * ddt_scale);
        let eq95_e1276_d_n15: f64 = (s.dn[170][15] * ddt_scale);
        let eq95_e1276_d_n16: f64 = (s.dn[170][16] * ddt_scale);
        let eq95_e1276_d_n17: f64 = (s.dn[170][17] * ddt_scale);
        let eq95_e1276_d_n18: f64 = (s.dn[170][18] * ddt_scale);
        let eq95_e1276_d_n19: f64 = (s.dn[170][19] * ddt_scale);
        let eq95_e1276_d_n20: f64 = (s.dn[170][20] * ddt_scale);
        let eq95_e1276_d_n21: f64 = (s.dn[170][21] * ddt_scale);
        let eq95_e1276_d_n22: f64 = (s.dn[170][22] * ddt_scale);
        let eq95_e1276_d_n23: f64 = (s.dn[170][23] * ddt_scale);
        let eq95_e1276_d_n24: f64 = (s.dn[170][24] * ddt_scale);
        let eq95_e1276_d_n25: f64 = (s.dn[170][25] * ddt_scale);
        let eq95_e1276_d_n26: f64 = (s.dn[170][26] * ddt_scale);
        let eq95_e1276_d_n27: f64 = (s.dn[170][27] * ddt_scale);
        let eq95_e1276_d_n28: f64 = (s.dn[170][28] * ddt_scale);
        let eq95_e1276_d_n29: f64 = (s.dn[170][29] * ddt_scale);
        let eq95_e1279: f64 = (p.p355 * (nv3 - nv10));
        let eq95_e1279_d_n3: f64 = p.p355;
        let eq95_e1279_d_n10: f64 = (-p.p355);
        let eq95_e1280: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 83, eq95_e1279);
        let eq95_e1280_d_n3: f64 = (eq95_e1279_d_n3 * ddt_scale);
        let eq95_e1280_d_n10: f64 = (eq95_e1279_d_n10 * ddt_scale);
        let eq95_e1281: f64 = (eq95_e1276 + eq95_e1280);
        let eq95_e1281_d_n3: f64 = (eq95_e1276_d_n3 + eq95_e1280_d_n3);
        let eq95_e1281_d_n10: f64 = (eq95_e1276_d_n10 + eq95_e1280_d_n10);
        let eq95_value: f64 = eq95_e1281;
        let eq95_node_derivatives: [f64; 30] = [eq95_e1276_d_n0, eq95_e1276_d_n1, eq95_e1276_d_n2, eq95_e1281_d_n3, eq95_e1276_d_n4, eq95_e1276_d_n5, eq95_e1276_d_n6, eq95_e1276_d_n7, eq95_e1276_d_n8, eq95_e1276_d_n9, eq95_e1281_d_n10, eq95_e1276_d_n11, eq95_e1276_d_n12, eq95_e1276_d_n13, eq95_e1276_d_n14, eq95_e1276_d_n15, eq95_e1276_d_n16, eq95_e1276_d_n17, eq95_e1276_d_n18, eq95_e1276_d_n19, eq95_e1276_d_n20, eq95_e1276_d_n21, eq95_e1276_d_n22, eq95_e1276_d_n23, eq95_e1276_d_n24, eq95_e1276_d_n25, eq95_e1276_d_n26, eq95_e1276_d_n27, eq95_e1276_d_n28, eq95_e1276_d_n29];
        let eq95_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            multiplicity * (eq95_value),
            nodes,
            &eq95_node_derivatives,
            branches,
            &eq95_branch_derivatives,
            multiplicity,
        );
        let (eq96_e1289, eq96_e1289_d_n0, eq96_e1289_d_n1, eq96_e1289_d_n2, eq96_e1289_d_n3, eq96_e1289_d_n4, eq96_e1289_d_n5, eq96_e1289_d_n6, eq96_e1289_d_n7, eq96_e1289_d_n8, eq96_e1289_d_n9, eq96_e1289_d_n10, eq96_e1289_d_n11, eq96_e1289_d_n12, eq96_e1289_d_n13, eq96_e1289_d_n14, eq96_e1289_d_n15, eq96_e1289_d_n16, eq96_e1289_d_n17, eq96_e1289_d_n18, eq96_e1289_d_n19, eq96_e1289_d_n20, eq96_e1289_d_n21, eq96_e1289_d_n22, eq96_e1289_d_n23, eq96_e1289_d_n24, eq96_e1289_d_n25, eq96_e1289_d_n26, eq96_e1289_d_n27, eq96_e1289_d_n28, eq96_e1289_d_n29,) = {
    if s.b[1055] {
        let eq96_e1286: f64 = (s.v[0] * (nv10 - nv11));
        let eq96_e1286_d_n10: f64 = s.v[0];
        let eq96_e1286_d_n11: f64 = (-s.v[0]);
        let eq96_e1287: f64 = (s.v[172] + eq96_e1286);
        let eq96_e1287_d_n10: f64 = (s.dn[172][10] + eq96_e1286_d_n10);
        let eq96_e1287_d_n11: f64 = (s.dn[172][11] + eq96_e1286_d_n11);
        (eq96_e1287, s.dn[172][0], s.dn[172][1], s.dn[172][2], s.dn[172][3], s.dn[172][4], s.dn[172][5], s.dn[172][6], s.dn[172][7], s.dn[172][8], s.dn[172][9], eq96_e1287_d_n10, eq96_e1287_d_n11, s.dn[172][12], s.dn[172][13], s.dn[172][14], s.dn[172][15], s.dn[172][16], s.dn[172][17], s.dn[172][18], s.dn[172][19], s.dn[172][20], s.dn[172][21], s.dn[172][22], s.dn[172][23], s.dn[172][24], s.dn[172][25], s.dn[172][26], s.dn[172][27], s.dn[172][28], s.dn[172][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e1289;
        let eq96_node_derivatives: [f64; 30] = [eq96_e1289_d_n0, eq96_e1289_d_n1, eq96_e1289_d_n2, eq96_e1289_d_n3, eq96_e1289_d_n4, eq96_e1289_d_n5, eq96_e1289_d_n6, eq96_e1289_d_n7, eq96_e1289_d_n8, eq96_e1289_d_n9, eq96_e1289_d_n10, eq96_e1289_d_n11, eq96_e1289_d_n12, eq96_e1289_d_n13, eq96_e1289_d_n14, eq96_e1289_d_n15, eq96_e1289_d_n16, eq96_e1289_d_n17, eq96_e1289_d_n18, eq96_e1289_d_n19, eq96_e1289_d_n20, eq96_e1289_d_n21, eq96_e1289_d_n22, eq96_e1289_d_n23, eq96_e1289_d_n24, eq96_e1289_d_n25, eq96_e1289_d_n26, eq96_e1289_d_n27, eq96_e1289_d_n28, eq96_e1289_d_n29];
        let eq96_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[11]),
            multiplicity * (eq96_value),
            nodes,
            &eq96_node_derivatives,
            branches,
            &eq96_branch_derivatives,
            multiplicity,
        );
        let (eq97_e1294,) = {
    if (!s.b[1055]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq97_value: f64 = eq97_e1294;
        stamper.stamp_potential_const(
            branches[23],
            eq97_value,
        );
        let (eq98_e1304, eq98_e1304_d_n0, eq98_e1304_d_n1, eq98_e1304_d_n2, eq98_e1304_d_n3, eq98_e1304_d_n4, eq98_e1304_d_n5, eq98_e1304_d_n6, eq98_e1304_d_n7, eq98_e1304_d_n8, eq98_e1304_d_n9, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_d_n12, eq98_e1304_d_n13, eq98_e1304_d_n14, eq98_e1304_d_n15, eq98_e1304_d_n16, eq98_e1304_d_n17, eq98_e1304_d_n18, eq98_e1304_d_n19, eq98_e1304_d_n20, eq98_e1304_d_n21, eq98_e1304_d_n22, eq98_e1304_d_n23, eq98_e1304_d_n24, eq98_e1304_d_n25, eq98_e1304_d_n26, eq98_e1304_d_n27, eq98_e1304_d_n28, eq98_e1304_d_n29,) = {
    if s.b[1201] {
        let eq98_e1297: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 84, s.v[173]);
        let eq98_e1297_d_n0: f64 = (s.dn[173][0] * ddt_scale);
        let eq98_e1297_d_n1: f64 = (s.dn[173][1] * ddt_scale);
        let eq98_e1297_d_n2: f64 = (s.dn[173][2] * ddt_scale);
        let eq98_e1297_d_n3: f64 = (s.dn[173][3] * ddt_scale);
        let eq98_e1297_d_n4: f64 = (s.dn[173][4] * ddt_scale);
        let eq98_e1297_d_n5: f64 = (s.dn[173][5] * ddt_scale);
        let eq98_e1297_d_n6: f64 = (s.dn[173][6] * ddt_scale);
        let eq98_e1297_d_n7: f64 = (s.dn[173][7] * ddt_scale);
        let eq98_e1297_d_n8: f64 = (s.dn[173][8] * ddt_scale);
        let eq98_e1297_d_n9: f64 = (s.dn[173][9] * ddt_scale);
        let eq98_e1297_d_n10: f64 = (s.dn[173][10] * ddt_scale);
        let eq98_e1297_d_n11: f64 = (s.dn[173][11] * ddt_scale);
        let eq98_e1297_d_n12: f64 = (s.dn[173][12] * ddt_scale);
        let eq98_e1297_d_n13: f64 = (s.dn[173][13] * ddt_scale);
        let eq98_e1297_d_n14: f64 = (s.dn[173][14] * ddt_scale);
        let eq98_e1297_d_n15: f64 = (s.dn[173][15] * ddt_scale);
        let eq98_e1297_d_n16: f64 = (s.dn[173][16] * ddt_scale);
        let eq98_e1297_d_n17: f64 = (s.dn[173][17] * ddt_scale);
        let eq98_e1297_d_n18: f64 = (s.dn[173][18] * ddt_scale);
        let eq98_e1297_d_n19: f64 = (s.dn[173][19] * ddt_scale);
        let eq98_e1297_d_n20: f64 = (s.dn[173][20] * ddt_scale);
        let eq98_e1297_d_n21: f64 = (s.dn[173][21] * ddt_scale);
        let eq98_e1297_d_n22: f64 = (s.dn[173][22] * ddt_scale);
        let eq98_e1297_d_n23: f64 = (s.dn[173][23] * ddt_scale);
        let eq98_e1297_d_n24: f64 = (s.dn[173][24] * ddt_scale);
        let eq98_e1297_d_n25: f64 = (s.dn[173][25] * ddt_scale);
        let eq98_e1297_d_n26: f64 = (s.dn[173][26] * ddt_scale);
        let eq98_e1297_d_n27: f64 = (s.dn[173][27] * ddt_scale);
        let eq98_e1297_d_n28: f64 = (s.dn[173][28] * ddt_scale);
        let eq98_e1297_d_n29: f64 = (s.dn[173][29] * ddt_scale);
        let eq98_e1300: f64 = (p.p355 * (nv7 - nv11));
        let eq98_e1300_d_n7: f64 = p.p355;
        let eq98_e1300_d_n11: f64 = (-p.p355);
        let eq98_e1301: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 85, eq98_e1300);
        let eq98_e1301_d_n7: f64 = (eq98_e1300_d_n7 * ddt_scale);
        let eq98_e1301_d_n11: f64 = (eq98_e1300_d_n11 * ddt_scale);
        let eq98_e1302: f64 = (eq98_e1297 + eq98_e1301);
        let eq98_e1302_d_n7: f64 = (eq98_e1297_d_n7 + eq98_e1301_d_n7);
        let eq98_e1302_d_n11: f64 = (eq98_e1297_d_n11 + eq98_e1301_d_n11);
        (eq98_e1302, eq98_e1297_d_n0, eq98_e1297_d_n1, eq98_e1297_d_n2, eq98_e1297_d_n3, eq98_e1297_d_n4, eq98_e1297_d_n5, eq98_e1297_d_n6, eq98_e1302_d_n7, eq98_e1297_d_n8, eq98_e1297_d_n9, eq98_e1297_d_n10, eq98_e1302_d_n11, eq98_e1297_d_n12, eq98_e1297_d_n13, eq98_e1297_d_n14, eq98_e1297_d_n15, eq98_e1297_d_n16, eq98_e1297_d_n17, eq98_e1297_d_n18, eq98_e1297_d_n19, eq98_e1297_d_n20, eq98_e1297_d_n21, eq98_e1297_d_n22, eq98_e1297_d_n23, eq98_e1297_d_n24, eq98_e1297_d_n25, eq98_e1297_d_n26, eq98_e1297_d_n27, eq98_e1297_d_n28, eq98_e1297_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq98_value: f64 = eq98_e1304;
        let eq98_node_derivatives: [f64; 30] = [eq98_e1304_d_n0, eq98_e1304_d_n1, eq98_e1304_d_n2, eq98_e1304_d_n3, eq98_e1304_d_n4, eq98_e1304_d_n5, eq98_e1304_d_n6, eq98_e1304_d_n7, eq98_e1304_d_n8, eq98_e1304_d_n9, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_d_n12, eq98_e1304_d_n13, eq98_e1304_d_n14, eq98_e1304_d_n15, eq98_e1304_d_n16, eq98_e1304_d_n17, eq98_e1304_d_n18, eq98_e1304_d_n19, eq98_e1304_d_n20, eq98_e1304_d_n21, eq98_e1304_d_n22, eq98_e1304_d_n23, eq98_e1304_d_n24, eq98_e1304_d_n25, eq98_e1304_d_n26, eq98_e1304_d_n27, eq98_e1304_d_n28, eq98_e1304_d_n29];
        let eq98_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            multiplicity * (eq98_value),
            nodes,
            &eq98_node_derivatives,
            branches,
            &eq98_branch_derivatives,
            multiplicity,
        );
        let (eq99_e1314, eq99_e1314_d_n0, eq99_e1314_d_n1, eq99_e1314_d_n2, eq99_e1314_d_n3, eq99_e1314_d_n4, eq99_e1314_d_n5, eq99_e1314_d_n6, eq99_e1314_d_n7, eq99_e1314_d_n8, eq99_e1314_d_n9, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_d_n12, eq99_e1314_d_n13, eq99_e1314_d_n14, eq99_e1314_d_n15, eq99_e1314_d_n16, eq99_e1314_d_n17, eq99_e1314_d_n18, eq99_e1314_d_n19, eq99_e1314_d_n20, eq99_e1314_d_n21, eq99_e1314_d_n22, eq99_e1314_d_n23, eq99_e1314_d_n24, eq99_e1314_d_n25, eq99_e1314_d_n26, eq99_e1314_d_n27, eq99_e1314_d_n28, eq99_e1314_d_n29,) = {
    if s.b[1201] {
        let eq99_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 86, s.v[174]);
        let eq99_e1307_d_n0: f64 = (s.dn[174][0] * ddt_scale);
        let eq99_e1307_d_n1: f64 = (s.dn[174][1] * ddt_scale);
        let eq99_e1307_d_n2: f64 = (s.dn[174][2] * ddt_scale);
        let eq99_e1307_d_n3: f64 = (s.dn[174][3] * ddt_scale);
        let eq99_e1307_d_n4: f64 = (s.dn[174][4] * ddt_scale);
        let eq99_e1307_d_n5: f64 = (s.dn[174][5] * ddt_scale);
        let eq99_e1307_d_n6: f64 = (s.dn[174][6] * ddt_scale);
        let eq99_e1307_d_n7: f64 = (s.dn[174][7] * ddt_scale);
        let eq99_e1307_d_n8: f64 = (s.dn[174][8] * ddt_scale);
        let eq99_e1307_d_n9: f64 = (s.dn[174][9] * ddt_scale);
        let eq99_e1307_d_n10: f64 = (s.dn[174][10] * ddt_scale);
        let eq99_e1307_d_n11: f64 = (s.dn[174][11] * ddt_scale);
        let eq99_e1307_d_n12: f64 = (s.dn[174][12] * ddt_scale);
        let eq99_e1307_d_n13: f64 = (s.dn[174][13] * ddt_scale);
        let eq99_e1307_d_n14: f64 = (s.dn[174][14] * ddt_scale);
        let eq99_e1307_d_n15: f64 = (s.dn[174][15] * ddt_scale);
        let eq99_e1307_d_n16: f64 = (s.dn[174][16] * ddt_scale);
        let eq99_e1307_d_n17: f64 = (s.dn[174][17] * ddt_scale);
        let eq99_e1307_d_n18: f64 = (s.dn[174][18] * ddt_scale);
        let eq99_e1307_d_n19: f64 = (s.dn[174][19] * ddt_scale);
        let eq99_e1307_d_n20: f64 = (s.dn[174][20] * ddt_scale);
        let eq99_e1307_d_n21: f64 = (s.dn[174][21] * ddt_scale);
        let eq99_e1307_d_n22: f64 = (s.dn[174][22] * ddt_scale);
        let eq99_e1307_d_n23: f64 = (s.dn[174][23] * ddt_scale);
        let eq99_e1307_d_n24: f64 = (s.dn[174][24] * ddt_scale);
        let eq99_e1307_d_n25: f64 = (s.dn[174][25] * ddt_scale);
        let eq99_e1307_d_n26: f64 = (s.dn[174][26] * ddt_scale);
        let eq99_e1307_d_n27: f64 = (s.dn[174][27] * ddt_scale);
        let eq99_e1307_d_n28: f64 = (s.dn[174][28] * ddt_scale);
        let eq99_e1307_d_n29: f64 = (s.dn[174][29] * ddt_scale);
        let eq99_e1310: f64 = (p.p355 * (nv7 - nv10));
        let eq99_e1310_d_n7: f64 = p.p355;
        let eq99_e1310_d_n10: f64 = (-p.p355);
        let eq99_e1311: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 87, eq99_e1310);
        let eq99_e1311_d_n7: f64 = (eq99_e1310_d_n7 * ddt_scale);
        let eq99_e1311_d_n10: f64 = (eq99_e1310_d_n10 * ddt_scale);
        let eq99_e1312: f64 = (eq99_e1307 + eq99_e1311);
        let eq99_e1312_d_n7: f64 = (eq99_e1307_d_n7 + eq99_e1311_d_n7);
        let eq99_e1312_d_n10: f64 = (eq99_e1307_d_n10 + eq99_e1311_d_n10);
        (eq99_e1312, eq99_e1307_d_n0, eq99_e1307_d_n1, eq99_e1307_d_n2, eq99_e1307_d_n3, eq99_e1307_d_n4, eq99_e1307_d_n5, eq99_e1307_d_n6, eq99_e1312_d_n7, eq99_e1307_d_n8, eq99_e1307_d_n9, eq99_e1312_d_n10, eq99_e1307_d_n11, eq99_e1307_d_n12, eq99_e1307_d_n13, eq99_e1307_d_n14, eq99_e1307_d_n15, eq99_e1307_d_n16, eq99_e1307_d_n17, eq99_e1307_d_n18, eq99_e1307_d_n19, eq99_e1307_d_n20, eq99_e1307_d_n21, eq99_e1307_d_n22, eq99_e1307_d_n23, eq99_e1307_d_n24, eq99_e1307_d_n25, eq99_e1307_d_n26, eq99_e1307_d_n27, eq99_e1307_d_n28, eq99_e1307_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq99_value: f64 = eq99_e1314;
        let eq99_node_derivatives: [f64; 30] = [eq99_e1314_d_n0, eq99_e1314_d_n1, eq99_e1314_d_n2, eq99_e1314_d_n3, eq99_e1314_d_n4, eq99_e1314_d_n5, eq99_e1314_d_n6, eq99_e1314_d_n7, eq99_e1314_d_n8, eq99_e1314_d_n9, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_d_n12, eq99_e1314_d_n13, eq99_e1314_d_n14, eq99_e1314_d_n15, eq99_e1314_d_n16, eq99_e1314_d_n17, eq99_e1314_d_n18, eq99_e1314_d_n19, eq99_e1314_d_n20, eq99_e1314_d_n21, eq99_e1314_d_n22, eq99_e1314_d_n23, eq99_e1314_d_n24, eq99_e1314_d_n25, eq99_e1314_d_n26, eq99_e1314_d_n27, eq99_e1314_d_n28, eq99_e1314_d_n29];
        let eq99_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            multiplicity * (eq99_value),
            nodes,
            &eq99_node_derivatives,
            branches,
            &eq99_branch_derivatives,
            multiplicity,
        );
        let (eq100_e1324, eq100_e1324_d_n0, eq100_e1324_d_n1, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n5, eq100_e1324_d_n6, eq100_e1324_d_n7, eq100_e1324_d_n8, eq100_e1324_d_n9, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_d_n12, eq100_e1324_d_n13, eq100_e1324_d_n14, eq100_e1324_d_n15, eq100_e1324_d_n16, eq100_e1324_d_n17, eq100_e1324_d_n18, eq100_e1324_d_n19, eq100_e1324_d_n20, eq100_e1324_d_n21, eq100_e1324_d_n22, eq100_e1324_d_n23, eq100_e1324_d_n24, eq100_e1324_d_n25, eq100_e1324_d_n26, eq100_e1324_d_n27, eq100_e1324_d_n28, eq100_e1324_d_n29,) = {
    if s.b[1201] {
        let eq100_e1317: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 88, s.v[175]);
        let eq100_e1317_d_n0: f64 = (s.dn[175][0] * ddt_scale);
        let eq100_e1317_d_n1: f64 = (s.dn[175][1] * ddt_scale);
        let eq100_e1317_d_n2: f64 = (s.dn[175][2] * ddt_scale);
        let eq100_e1317_d_n3: f64 = (s.dn[175][3] * ddt_scale);
        let eq100_e1317_d_n4: f64 = (s.dn[175][4] * ddt_scale);
        let eq100_e1317_d_n5: f64 = (s.dn[175][5] * ddt_scale);
        let eq100_e1317_d_n6: f64 = (s.dn[175][6] * ddt_scale);
        let eq100_e1317_d_n7: f64 = (s.dn[175][7] * ddt_scale);
        let eq100_e1317_d_n8: f64 = (s.dn[175][8] * ddt_scale);
        let eq100_e1317_d_n9: f64 = (s.dn[175][9] * ddt_scale);
        let eq100_e1317_d_n10: f64 = (s.dn[175][10] * ddt_scale);
        let eq100_e1317_d_n11: f64 = (s.dn[175][11] * ddt_scale);
        let eq100_e1317_d_n12: f64 = (s.dn[175][12] * ddt_scale);
        let eq100_e1317_d_n13: f64 = (s.dn[175][13] * ddt_scale);
        let eq100_e1317_d_n14: f64 = (s.dn[175][14] * ddt_scale);
        let eq100_e1317_d_n15: f64 = (s.dn[175][15] * ddt_scale);
        let eq100_e1317_d_n16: f64 = (s.dn[175][16] * ddt_scale);
        let eq100_e1317_d_n17: f64 = (s.dn[175][17] * ddt_scale);
        let eq100_e1317_d_n18: f64 = (s.dn[175][18] * ddt_scale);
        let eq100_e1317_d_n19: f64 = (s.dn[175][19] * ddt_scale);
        let eq100_e1317_d_n20: f64 = (s.dn[175][20] * ddt_scale);
        let eq100_e1317_d_n21: f64 = (s.dn[175][21] * ddt_scale);
        let eq100_e1317_d_n22: f64 = (s.dn[175][22] * ddt_scale);
        let eq100_e1317_d_n23: f64 = (s.dn[175][23] * ddt_scale);
        let eq100_e1317_d_n24: f64 = (s.dn[175][24] * ddt_scale);
        let eq100_e1317_d_n25: f64 = (s.dn[175][25] * ddt_scale);
        let eq100_e1317_d_n26: f64 = (s.dn[175][26] * ddt_scale);
        let eq100_e1317_d_n27: f64 = (s.dn[175][27] * ddt_scale);
        let eq100_e1317_d_n28: f64 = (s.dn[175][28] * ddt_scale);
        let eq100_e1317_d_n29: f64 = (s.dn[175][29] * ddt_scale);
        let eq100_e1320: f64 = (p.p355 * (nv2 - nv11));
        let eq100_e1320_d_n2: f64 = p.p355;
        let eq100_e1320_d_n11: f64 = (-p.p355);
        let eq100_e1321: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 89, eq100_e1320);
        let eq100_e1321_d_n2: f64 = (eq100_e1320_d_n2 * ddt_scale);
        let eq100_e1321_d_n11: f64 = (eq100_e1320_d_n11 * ddt_scale);
        let eq100_e1322: f64 = (eq100_e1317 + eq100_e1321);
        let eq100_e1322_d_n2: f64 = (eq100_e1317_d_n2 + eq100_e1321_d_n2);
        let eq100_e1322_d_n11: f64 = (eq100_e1317_d_n11 + eq100_e1321_d_n11);
        (eq100_e1322, eq100_e1317_d_n0, eq100_e1317_d_n1, eq100_e1322_d_n2, eq100_e1317_d_n3, eq100_e1317_d_n4, eq100_e1317_d_n5, eq100_e1317_d_n6, eq100_e1317_d_n7, eq100_e1317_d_n8, eq100_e1317_d_n9, eq100_e1317_d_n10, eq100_e1322_d_n11, eq100_e1317_d_n12, eq100_e1317_d_n13, eq100_e1317_d_n14, eq100_e1317_d_n15, eq100_e1317_d_n16, eq100_e1317_d_n17, eq100_e1317_d_n18, eq100_e1317_d_n19, eq100_e1317_d_n20, eq100_e1317_d_n21, eq100_e1317_d_n22, eq100_e1317_d_n23, eq100_e1317_d_n24, eq100_e1317_d_n25, eq100_e1317_d_n26, eq100_e1317_d_n27, eq100_e1317_d_n28, eq100_e1317_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_value: f64 = eq100_e1324;
        let eq100_node_derivatives: [f64; 30] = [eq100_e1324_d_n0, eq100_e1324_d_n1, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n5, eq100_e1324_d_n6, eq100_e1324_d_n7, eq100_e1324_d_n8, eq100_e1324_d_n9, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_d_n12, eq100_e1324_d_n13, eq100_e1324_d_n14, eq100_e1324_d_n15, eq100_e1324_d_n16, eq100_e1324_d_n17, eq100_e1324_d_n18, eq100_e1324_d_n19, eq100_e1324_d_n20, eq100_e1324_d_n21, eq100_e1324_d_n22, eq100_e1324_d_n23, eq100_e1324_d_n24, eq100_e1324_d_n25, eq100_e1324_d_n26, eq100_e1324_d_n27, eq100_e1324_d_n28, eq100_e1324_d_n29];
        let eq100_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            multiplicity * (eq100_value),
            nodes,
            &eq100_node_derivatives,
            branches,
            &eq100_branch_derivatives,
            multiplicity,
        );
        let (eq101_e1328,) = {
    if s.b[1201] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq101_value: f64 = eq101_e1328;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[10]),
            multiplicity * (eq101_value),
        );
        let (eq102_e1338, eq102_e1338_d_n0, eq102_e1338_d_n1, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n5, eq102_e1338_d_n6, eq102_e1338_d_n7, eq102_e1338_d_n8, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_d_n12, eq102_e1338_d_n13, eq102_e1338_d_n14, eq102_e1338_d_n15, eq102_e1338_d_n16, eq102_e1338_d_n17, eq102_e1338_d_n18, eq102_e1338_d_n19, eq102_e1338_d_n20, eq102_e1338_d_n21, eq102_e1338_d_n22, eq102_e1338_d_n23, eq102_e1338_d_n24, eq102_e1338_d_n25, eq102_e1338_d_n26, eq102_e1338_d_n27, eq102_e1338_d_n28, eq102_e1338_d_n29,) = {
    if s.b[1201] {
        let eq102_e1331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 90, s.v[177]);
        let eq102_e1331_d_n0: f64 = (s.dn[177][0] * ddt_scale);
        let eq102_e1331_d_n1: f64 = (s.dn[177][1] * ddt_scale);
        let eq102_e1331_d_n2: f64 = (s.dn[177][2] * ddt_scale);
        let eq102_e1331_d_n3: f64 = (s.dn[177][3] * ddt_scale);
        let eq102_e1331_d_n4: f64 = (s.dn[177][4] * ddt_scale);
        let eq102_e1331_d_n5: f64 = (s.dn[177][5] * ddt_scale);
        let eq102_e1331_d_n6: f64 = (s.dn[177][6] * ddt_scale);
        let eq102_e1331_d_n7: f64 = (s.dn[177][7] * ddt_scale);
        let eq102_e1331_d_n8: f64 = (s.dn[177][8] * ddt_scale);
        let eq102_e1331_d_n9: f64 = (s.dn[177][9] * ddt_scale);
        let eq102_e1331_d_n10: f64 = (s.dn[177][10] * ddt_scale);
        let eq102_e1331_d_n11: f64 = (s.dn[177][11] * ddt_scale);
        let eq102_e1331_d_n12: f64 = (s.dn[177][12] * ddt_scale);
        let eq102_e1331_d_n13: f64 = (s.dn[177][13] * ddt_scale);
        let eq102_e1331_d_n14: f64 = (s.dn[177][14] * ddt_scale);
        let eq102_e1331_d_n15: f64 = (s.dn[177][15] * ddt_scale);
        let eq102_e1331_d_n16: f64 = (s.dn[177][16] * ddt_scale);
        let eq102_e1331_d_n17: f64 = (s.dn[177][17] * ddt_scale);
        let eq102_e1331_d_n18: f64 = (s.dn[177][18] * ddt_scale);
        let eq102_e1331_d_n19: f64 = (s.dn[177][19] * ddt_scale);
        let eq102_e1331_d_n20: f64 = (s.dn[177][20] * ddt_scale);
        let eq102_e1331_d_n21: f64 = (s.dn[177][21] * ddt_scale);
        let eq102_e1331_d_n22: f64 = (s.dn[177][22] * ddt_scale);
        let eq102_e1331_d_n23: f64 = (s.dn[177][23] * ddt_scale);
        let eq102_e1331_d_n24: f64 = (s.dn[177][24] * ddt_scale);
        let eq102_e1331_d_n25: f64 = (s.dn[177][25] * ddt_scale);
        let eq102_e1331_d_n26: f64 = (s.dn[177][26] * ddt_scale);
        let eq102_e1331_d_n27: f64 = (s.dn[177][27] * ddt_scale);
        let eq102_e1331_d_n28: f64 = (s.dn[177][28] * ddt_scale);
        let eq102_e1331_d_n29: f64 = (s.dn[177][29] * ddt_scale);
        let eq102_e1334: f64 = (p.p355 * (nv7 - nv9));
        let eq102_e1334_d_n7: f64 = p.p355;
        let eq102_e1334_d_n9: f64 = (-p.p355);
        let eq102_e1335: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 91, eq102_e1334);
        let eq102_e1335_d_n7: f64 = (eq102_e1334_d_n7 * ddt_scale);
        let eq102_e1335_d_n9: f64 = (eq102_e1334_d_n9 * ddt_scale);
        let eq102_e1336: f64 = (eq102_e1331 + eq102_e1335);
        let eq102_e1336_d_n7: f64 = (eq102_e1331_d_n7 + eq102_e1335_d_n7);
        let eq102_e1336_d_n9: f64 = (eq102_e1331_d_n9 + eq102_e1335_d_n9);
        (eq102_e1336, eq102_e1331_d_n0, eq102_e1331_d_n1, eq102_e1331_d_n2, eq102_e1331_d_n3, eq102_e1331_d_n4, eq102_e1331_d_n5, eq102_e1331_d_n6, eq102_e1336_d_n7, eq102_e1331_d_n8, eq102_e1336_d_n9, eq102_e1331_d_n10, eq102_e1331_d_n11, eq102_e1331_d_n12, eq102_e1331_d_n13, eq102_e1331_d_n14, eq102_e1331_d_n15, eq102_e1331_d_n16, eq102_e1331_d_n17, eq102_e1331_d_n18, eq102_e1331_d_n19, eq102_e1331_d_n20, eq102_e1331_d_n21, eq102_e1331_d_n22, eq102_e1331_d_n23, eq102_e1331_d_n24, eq102_e1331_d_n25, eq102_e1331_d_n26, eq102_e1331_d_n27, eq102_e1331_d_n28, eq102_e1331_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_value: f64 = eq102_e1338;
        let eq102_node_derivatives: [f64; 30] = [eq102_e1338_d_n0, eq102_e1338_d_n1, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n5, eq102_e1338_d_n6, eq102_e1338_d_n7, eq102_e1338_d_n8, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_d_n12, eq102_e1338_d_n13, eq102_e1338_d_n14, eq102_e1338_d_n15, eq102_e1338_d_n16, eq102_e1338_d_n17, eq102_e1338_d_n18, eq102_e1338_d_n19, eq102_e1338_d_n20, eq102_e1338_d_n21, eq102_e1338_d_n22, eq102_e1338_d_n23, eq102_e1338_d_n24, eq102_e1338_d_n25, eq102_e1338_d_n26, eq102_e1338_d_n27, eq102_e1338_d_n28, eq102_e1338_d_n29];
        let eq102_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq102_value),
            nodes,
            &eq102_node_derivatives,
            branches,
            &eq102_branch_derivatives,
            multiplicity,
        );
        let (eq103_e1349, eq103_e1349_d_n0, eq103_e1349_d_n1, eq103_e1349_d_n2, eq103_e1349_d_n3, eq103_e1349_d_n4, eq103_e1349_d_n5, eq103_e1349_d_n6, eq103_e1349_d_n7, eq103_e1349_d_n8, eq103_e1349_d_n9, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_d_n12, eq103_e1349_d_n13, eq103_e1349_d_n14, eq103_e1349_d_n15, eq103_e1349_d_n16, eq103_e1349_d_n17, eq103_e1349_d_n18, eq103_e1349_d_n19, eq103_e1349_d_n20, eq103_e1349_d_n21, eq103_e1349_d_n22, eq103_e1349_d_n23, eq103_e1349_d_n24, eq103_e1349_d_n25, eq103_e1349_d_n26, eq103_e1349_d_n27, eq103_e1349_d_n28, eq103_e1349_d_n29,) = {
    if (!s.b[1201]) {
        let eq103_e1342: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 92, s.v[173]);
        let eq103_e1342_d_n0: f64 = (s.dn[173][0] * ddt_scale);
        let eq103_e1342_d_n1: f64 = (s.dn[173][1] * ddt_scale);
        let eq103_e1342_d_n2: f64 = (s.dn[173][2] * ddt_scale);
        let eq103_e1342_d_n3: f64 = (s.dn[173][3] * ddt_scale);
        let eq103_e1342_d_n4: f64 = (s.dn[173][4] * ddt_scale);
        let eq103_e1342_d_n5: f64 = (s.dn[173][5] * ddt_scale);
        let eq103_e1342_d_n6: f64 = (s.dn[173][6] * ddt_scale);
        let eq103_e1342_d_n7: f64 = (s.dn[173][7] * ddt_scale);
        let eq103_e1342_d_n8: f64 = (s.dn[173][8] * ddt_scale);
        let eq103_e1342_d_n9: f64 = (s.dn[173][9] * ddt_scale);
        let eq103_e1342_d_n10: f64 = (s.dn[173][10] * ddt_scale);
        let eq103_e1342_d_n11: f64 = (s.dn[173][11] * ddt_scale);
        let eq103_e1342_d_n12: f64 = (s.dn[173][12] * ddt_scale);
        let eq103_e1342_d_n13: f64 = (s.dn[173][13] * ddt_scale);
        let eq103_e1342_d_n14: f64 = (s.dn[173][14] * ddt_scale);
        let eq103_e1342_d_n15: f64 = (s.dn[173][15] * ddt_scale);
        let eq103_e1342_d_n16: f64 = (s.dn[173][16] * ddt_scale);
        let eq103_e1342_d_n17: f64 = (s.dn[173][17] * ddt_scale);
        let eq103_e1342_d_n18: f64 = (s.dn[173][18] * ddt_scale);
        let eq103_e1342_d_n19: f64 = (s.dn[173][19] * ddt_scale);
        let eq103_e1342_d_n20: f64 = (s.dn[173][20] * ddt_scale);
        let eq103_e1342_d_n21: f64 = (s.dn[173][21] * ddt_scale);
        let eq103_e1342_d_n22: f64 = (s.dn[173][22] * ddt_scale);
        let eq103_e1342_d_n23: f64 = (s.dn[173][23] * ddt_scale);
        let eq103_e1342_d_n24: f64 = (s.dn[173][24] * ddt_scale);
        let eq103_e1342_d_n25: f64 = (s.dn[173][25] * ddt_scale);
        let eq103_e1342_d_n26: f64 = (s.dn[173][26] * ddt_scale);
        let eq103_e1342_d_n27: f64 = (s.dn[173][27] * ddt_scale);
        let eq103_e1342_d_n28: f64 = (s.dn[173][28] * ddt_scale);
        let eq103_e1342_d_n29: f64 = (s.dn[173][29] * ddt_scale);
        let eq103_e1345: f64 = (p.p355 * (nv2 - nv11));
        let eq103_e1345_d_n2: f64 = p.p355;
        let eq103_e1345_d_n11: f64 = (-p.p355);
        let eq103_e1346: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 93, eq103_e1345);
        let eq103_e1346_d_n2: f64 = (eq103_e1345_d_n2 * ddt_scale);
        let eq103_e1346_d_n11: f64 = (eq103_e1345_d_n11 * ddt_scale);
        let eq103_e1347: f64 = (eq103_e1342 + eq103_e1346);
        let eq103_e1347_d_n2: f64 = (eq103_e1342_d_n2 + eq103_e1346_d_n2);
        let eq103_e1347_d_n11: f64 = (eq103_e1342_d_n11 + eq103_e1346_d_n11);
        (eq103_e1347, eq103_e1342_d_n0, eq103_e1342_d_n1, eq103_e1347_d_n2, eq103_e1342_d_n3, eq103_e1342_d_n4, eq103_e1342_d_n5, eq103_e1342_d_n6, eq103_e1342_d_n7, eq103_e1342_d_n8, eq103_e1342_d_n9, eq103_e1342_d_n10, eq103_e1347_d_n11, eq103_e1342_d_n12, eq103_e1342_d_n13, eq103_e1342_d_n14, eq103_e1342_d_n15, eq103_e1342_d_n16, eq103_e1342_d_n17, eq103_e1342_d_n18, eq103_e1342_d_n19, eq103_e1342_d_n20, eq103_e1342_d_n21, eq103_e1342_d_n22, eq103_e1342_d_n23, eq103_e1342_d_n24, eq103_e1342_d_n25, eq103_e1342_d_n26, eq103_e1342_d_n27, eq103_e1342_d_n28, eq103_e1342_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_value: f64 = eq103_e1349;
        let eq103_node_derivatives: [f64; 30] = [eq103_e1349_d_n0, eq103_e1349_d_n1, eq103_e1349_d_n2, eq103_e1349_d_n3, eq103_e1349_d_n4, eq103_e1349_d_n5, eq103_e1349_d_n6, eq103_e1349_d_n7, eq103_e1349_d_n8, eq103_e1349_d_n9, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_d_n12, eq103_e1349_d_n13, eq103_e1349_d_n14, eq103_e1349_d_n15, eq103_e1349_d_n16, eq103_e1349_d_n17, eq103_e1349_d_n18, eq103_e1349_d_n19, eq103_e1349_d_n20, eq103_e1349_d_n21, eq103_e1349_d_n22, eq103_e1349_d_n23, eq103_e1349_d_n24, eq103_e1349_d_n25, eq103_e1349_d_n26, eq103_e1349_d_n27, eq103_e1349_d_n28, eq103_e1349_d_n29];
        let eq103_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            multiplicity * (eq103_value),
            nodes,
            &eq103_node_derivatives,
            branches,
            &eq103_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_9(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq104_e1360, eq104_e1360_d_n0, eq104_e1360_d_n1, eq104_e1360_d_n2, eq104_e1360_d_n3, eq104_e1360_d_n4, eq104_e1360_d_n5, eq104_e1360_d_n6, eq104_e1360_d_n7, eq104_e1360_d_n8, eq104_e1360_d_n9, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_d_n12, eq104_e1360_d_n13, eq104_e1360_d_n14, eq104_e1360_d_n15, eq104_e1360_d_n16, eq104_e1360_d_n17, eq104_e1360_d_n18, eq104_e1360_d_n19, eq104_e1360_d_n20, eq104_e1360_d_n21, eq104_e1360_d_n22, eq104_e1360_d_n23, eq104_e1360_d_n24, eq104_e1360_d_n25, eq104_e1360_d_n26, eq104_e1360_d_n27, eq104_e1360_d_n28, eq104_e1360_d_n29,) = {
    if (!s.b[1201]) {
        let eq104_e1353: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 94, s.v[174]);
        let eq104_e1353_d_n0: f64 = (s.dn[174][0] * ddt_scale);
        let eq104_e1353_d_n1: f64 = (s.dn[174][1] * ddt_scale);
        let eq104_e1353_d_n2: f64 = (s.dn[174][2] * ddt_scale);
        let eq104_e1353_d_n3: f64 = (s.dn[174][3] * ddt_scale);
        let eq104_e1353_d_n4: f64 = (s.dn[174][4] * ddt_scale);
        let eq104_e1353_d_n5: f64 = (s.dn[174][5] * ddt_scale);
        let eq104_e1353_d_n6: f64 = (s.dn[174][6] * ddt_scale);
        let eq104_e1353_d_n7: f64 = (s.dn[174][7] * ddt_scale);
        let eq104_e1353_d_n8: f64 = (s.dn[174][8] * ddt_scale);
        let eq104_e1353_d_n9: f64 = (s.dn[174][9] * ddt_scale);
        let eq104_e1353_d_n10: f64 = (s.dn[174][10] * ddt_scale);
        let eq104_e1353_d_n11: f64 = (s.dn[174][11] * ddt_scale);
        let eq104_e1353_d_n12: f64 = (s.dn[174][12] * ddt_scale);
        let eq104_e1353_d_n13: f64 = (s.dn[174][13] * ddt_scale);
        let eq104_e1353_d_n14: f64 = (s.dn[174][14] * ddt_scale);
        let eq104_e1353_d_n15: f64 = (s.dn[174][15] * ddt_scale);
        let eq104_e1353_d_n16: f64 = (s.dn[174][16] * ddt_scale);
        let eq104_e1353_d_n17: f64 = (s.dn[174][17] * ddt_scale);
        let eq104_e1353_d_n18: f64 = (s.dn[174][18] * ddt_scale);
        let eq104_e1353_d_n19: f64 = (s.dn[174][19] * ddt_scale);
        let eq104_e1353_d_n20: f64 = (s.dn[174][20] * ddt_scale);
        let eq104_e1353_d_n21: f64 = (s.dn[174][21] * ddt_scale);
        let eq104_e1353_d_n22: f64 = (s.dn[174][22] * ddt_scale);
        let eq104_e1353_d_n23: f64 = (s.dn[174][23] * ddt_scale);
        let eq104_e1353_d_n24: f64 = (s.dn[174][24] * ddt_scale);
        let eq104_e1353_d_n25: f64 = (s.dn[174][25] * ddt_scale);
        let eq104_e1353_d_n26: f64 = (s.dn[174][26] * ddt_scale);
        let eq104_e1353_d_n27: f64 = (s.dn[174][27] * ddt_scale);
        let eq104_e1353_d_n28: f64 = (s.dn[174][28] * ddt_scale);
        let eq104_e1353_d_n29: f64 = (s.dn[174][29] * ddt_scale);
        let eq104_e1356: f64 = (p.p355 * (nv2 - nv10));
        let eq104_e1356_d_n2: f64 = p.p355;
        let eq104_e1356_d_n10: f64 = (-p.p355);
        let eq104_e1357: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 95, eq104_e1356);
        let eq104_e1357_d_n2: f64 = (eq104_e1356_d_n2 * ddt_scale);
        let eq104_e1357_d_n10: f64 = (eq104_e1356_d_n10 * ddt_scale);
        let eq104_e1358: f64 = (eq104_e1353 + eq104_e1357);
        let eq104_e1358_d_n2: f64 = (eq104_e1353_d_n2 + eq104_e1357_d_n2);
        let eq104_e1358_d_n10: f64 = (eq104_e1353_d_n10 + eq104_e1357_d_n10);
        (eq104_e1358, eq104_e1353_d_n0, eq104_e1353_d_n1, eq104_e1358_d_n2, eq104_e1353_d_n3, eq104_e1353_d_n4, eq104_e1353_d_n5, eq104_e1353_d_n6, eq104_e1353_d_n7, eq104_e1353_d_n8, eq104_e1353_d_n9, eq104_e1358_d_n10, eq104_e1353_d_n11, eq104_e1353_d_n12, eq104_e1353_d_n13, eq104_e1353_d_n14, eq104_e1353_d_n15, eq104_e1353_d_n16, eq104_e1353_d_n17, eq104_e1353_d_n18, eq104_e1353_d_n19, eq104_e1353_d_n20, eq104_e1353_d_n21, eq104_e1353_d_n22, eq104_e1353_d_n23, eq104_e1353_d_n24, eq104_e1353_d_n25, eq104_e1353_d_n26, eq104_e1353_d_n27, eq104_e1353_d_n28, eq104_e1353_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq104_value: f64 = eq104_e1360;
        let eq104_node_derivatives: [f64; 30] = [eq104_e1360_d_n0, eq104_e1360_d_n1, eq104_e1360_d_n2, eq104_e1360_d_n3, eq104_e1360_d_n4, eq104_e1360_d_n5, eq104_e1360_d_n6, eq104_e1360_d_n7, eq104_e1360_d_n8, eq104_e1360_d_n9, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_d_n12, eq104_e1360_d_n13, eq104_e1360_d_n14, eq104_e1360_d_n15, eq104_e1360_d_n16, eq104_e1360_d_n17, eq104_e1360_d_n18, eq104_e1360_d_n19, eq104_e1360_d_n20, eq104_e1360_d_n21, eq104_e1360_d_n22, eq104_e1360_d_n23, eq104_e1360_d_n24, eq104_e1360_d_n25, eq104_e1360_d_n26, eq104_e1360_d_n27, eq104_e1360_d_n28, eq104_e1360_d_n29];
        let eq104_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            multiplicity * (eq104_value),
            nodes,
            &eq104_node_derivatives,
            branches,
            &eq104_branch_derivatives,
            multiplicity,
        );
        let (eq105_e1371, eq105_e1371_d_n0, eq105_e1371_d_n1, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n5, eq105_e1371_d_n6, eq105_e1371_d_n7, eq105_e1371_d_n8, eq105_e1371_d_n9, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_d_n12, eq105_e1371_d_n13, eq105_e1371_d_n14, eq105_e1371_d_n15, eq105_e1371_d_n16, eq105_e1371_d_n17, eq105_e1371_d_n18, eq105_e1371_d_n19, eq105_e1371_d_n20, eq105_e1371_d_n21, eq105_e1371_d_n22, eq105_e1371_d_n23, eq105_e1371_d_n24, eq105_e1371_d_n25, eq105_e1371_d_n26, eq105_e1371_d_n27, eq105_e1371_d_n28, eq105_e1371_d_n29,) = {
    if (!s.b[1201]) {
        let eq105_e1364: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 96, s.v[175]);
        let eq105_e1364_d_n0: f64 = (s.dn[175][0] * ddt_scale);
        let eq105_e1364_d_n1: f64 = (s.dn[175][1] * ddt_scale);
        let eq105_e1364_d_n2: f64 = (s.dn[175][2] * ddt_scale);
        let eq105_e1364_d_n3: f64 = (s.dn[175][3] * ddt_scale);
        let eq105_e1364_d_n4: f64 = (s.dn[175][4] * ddt_scale);
        let eq105_e1364_d_n5: f64 = (s.dn[175][5] * ddt_scale);
        let eq105_e1364_d_n6: f64 = (s.dn[175][6] * ddt_scale);
        let eq105_e1364_d_n7: f64 = (s.dn[175][7] * ddt_scale);
        let eq105_e1364_d_n8: f64 = (s.dn[175][8] * ddt_scale);
        let eq105_e1364_d_n9: f64 = (s.dn[175][9] * ddt_scale);
        let eq105_e1364_d_n10: f64 = (s.dn[175][10] * ddt_scale);
        let eq105_e1364_d_n11: f64 = (s.dn[175][11] * ddt_scale);
        let eq105_e1364_d_n12: f64 = (s.dn[175][12] * ddt_scale);
        let eq105_e1364_d_n13: f64 = (s.dn[175][13] * ddt_scale);
        let eq105_e1364_d_n14: f64 = (s.dn[175][14] * ddt_scale);
        let eq105_e1364_d_n15: f64 = (s.dn[175][15] * ddt_scale);
        let eq105_e1364_d_n16: f64 = (s.dn[175][16] * ddt_scale);
        let eq105_e1364_d_n17: f64 = (s.dn[175][17] * ddt_scale);
        let eq105_e1364_d_n18: f64 = (s.dn[175][18] * ddt_scale);
        let eq105_e1364_d_n19: f64 = (s.dn[175][19] * ddt_scale);
        let eq105_e1364_d_n20: f64 = (s.dn[175][20] * ddt_scale);
        let eq105_e1364_d_n21: f64 = (s.dn[175][21] * ddt_scale);
        let eq105_e1364_d_n22: f64 = (s.dn[175][22] * ddt_scale);
        let eq105_e1364_d_n23: f64 = (s.dn[175][23] * ddt_scale);
        let eq105_e1364_d_n24: f64 = (s.dn[175][24] * ddt_scale);
        let eq105_e1364_d_n25: f64 = (s.dn[175][25] * ddt_scale);
        let eq105_e1364_d_n26: f64 = (s.dn[175][26] * ddt_scale);
        let eq105_e1364_d_n27: f64 = (s.dn[175][27] * ddt_scale);
        let eq105_e1364_d_n28: f64 = (s.dn[175][28] * ddt_scale);
        let eq105_e1364_d_n29: f64 = (s.dn[175][29] * ddt_scale);
        let eq105_e1367: f64 = (p.p355 * (nv7 - nv11));
        let eq105_e1367_d_n7: f64 = p.p355;
        let eq105_e1367_d_n11: f64 = (-p.p355);
        let eq105_e1368: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 97, eq105_e1367);
        let eq105_e1368_d_n7: f64 = (eq105_e1367_d_n7 * ddt_scale);
        let eq105_e1368_d_n11: f64 = (eq105_e1367_d_n11 * ddt_scale);
        let eq105_e1369: f64 = (eq105_e1364 + eq105_e1368);
        let eq105_e1369_d_n7: f64 = (eq105_e1364_d_n7 + eq105_e1368_d_n7);
        let eq105_e1369_d_n11: f64 = (eq105_e1364_d_n11 + eq105_e1368_d_n11);
        (eq105_e1369, eq105_e1364_d_n0, eq105_e1364_d_n1, eq105_e1364_d_n2, eq105_e1364_d_n3, eq105_e1364_d_n4, eq105_e1364_d_n5, eq105_e1364_d_n6, eq105_e1369_d_n7, eq105_e1364_d_n8, eq105_e1364_d_n9, eq105_e1364_d_n10, eq105_e1369_d_n11, eq105_e1364_d_n12, eq105_e1364_d_n13, eq105_e1364_d_n14, eq105_e1364_d_n15, eq105_e1364_d_n16, eq105_e1364_d_n17, eq105_e1364_d_n18, eq105_e1364_d_n19, eq105_e1364_d_n20, eq105_e1364_d_n21, eq105_e1364_d_n22, eq105_e1364_d_n23, eq105_e1364_d_n24, eq105_e1364_d_n25, eq105_e1364_d_n26, eq105_e1364_d_n27, eq105_e1364_d_n28, eq105_e1364_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_value: f64 = eq105_e1371;
        let eq105_node_derivatives: [f64; 30] = [eq105_e1371_d_n0, eq105_e1371_d_n1, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n5, eq105_e1371_d_n6, eq105_e1371_d_n7, eq105_e1371_d_n8, eq105_e1371_d_n9, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_d_n12, eq105_e1371_d_n13, eq105_e1371_d_n14, eq105_e1371_d_n15, eq105_e1371_d_n16, eq105_e1371_d_n17, eq105_e1371_d_n18, eq105_e1371_d_n19, eq105_e1371_d_n20, eq105_e1371_d_n21, eq105_e1371_d_n22, eq105_e1371_d_n23, eq105_e1371_d_n24, eq105_e1371_d_n25, eq105_e1371_d_n26, eq105_e1371_d_n27, eq105_e1371_d_n28, eq105_e1371_d_n29];
        let eq105_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            multiplicity * (eq105_value),
            nodes,
            &eq105_node_derivatives,
            branches,
            &eq105_branch_derivatives,
            multiplicity,
        );
        let (eq106_e1376,) = {
    if (!s.b[1201]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq106_value: f64 = eq106_e1376;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[10]),
            multiplicity * (eq106_value),
        );
        let (eq107_e1381,) = {
    if (!s.b[1201]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq107_value: f64 = eq107_e1381;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq107_value),
        );
        let eq108_e1383: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 98, s.v[176]);
        let eq108_e1383_d_n0: f64 = (s.dn[176][0] * ddt_scale);
        let eq108_e1383_d_n1: f64 = (s.dn[176][1] * ddt_scale);
        let eq108_e1383_d_n2: f64 = (s.dn[176][2] * ddt_scale);
        let eq108_e1383_d_n3: f64 = (s.dn[176][3] * ddt_scale);
        let eq108_e1383_d_n4: f64 = (s.dn[176][4] * ddt_scale);
        let eq108_e1383_d_n5: f64 = (s.dn[176][5] * ddt_scale);
        let eq108_e1383_d_n6: f64 = (s.dn[176][6] * ddt_scale);
        let eq108_e1383_d_n7: f64 = (s.dn[176][7] * ddt_scale);
        let eq108_e1383_d_n8: f64 = (s.dn[176][8] * ddt_scale);
        let eq108_e1383_d_n9: f64 = (s.dn[176][9] * ddt_scale);
        let eq108_e1383_d_n10: f64 = (s.dn[176][10] * ddt_scale);
        let eq108_e1383_d_n11: f64 = (s.dn[176][11] * ddt_scale);
        let eq108_e1383_d_n12: f64 = (s.dn[176][12] * ddt_scale);
        let eq108_e1383_d_n13: f64 = (s.dn[176][13] * ddt_scale);
        let eq108_e1383_d_n14: f64 = (s.dn[176][14] * ddt_scale);
        let eq108_e1383_d_n15: f64 = (s.dn[176][15] * ddt_scale);
        let eq108_e1383_d_n16: f64 = (s.dn[176][16] * ddt_scale);
        let eq108_e1383_d_n17: f64 = (s.dn[176][17] * ddt_scale);
        let eq108_e1383_d_n18: f64 = (s.dn[176][18] * ddt_scale);
        let eq108_e1383_d_n19: f64 = (s.dn[176][19] * ddt_scale);
        let eq108_e1383_d_n20: f64 = (s.dn[176][20] * ddt_scale);
        let eq108_e1383_d_n21: f64 = (s.dn[176][21] * ddt_scale);
        let eq108_e1383_d_n22: f64 = (s.dn[176][22] * ddt_scale);
        let eq108_e1383_d_n23: f64 = (s.dn[176][23] * ddt_scale);
        let eq108_e1383_d_n24: f64 = (s.dn[176][24] * ddt_scale);
        let eq108_e1383_d_n25: f64 = (s.dn[176][25] * ddt_scale);
        let eq108_e1383_d_n26: f64 = (s.dn[176][26] * ddt_scale);
        let eq108_e1383_d_n27: f64 = (s.dn[176][27] * ddt_scale);
        let eq108_e1383_d_n28: f64 = (s.dn[176][28] * ddt_scale);
        let eq108_e1383_d_n29: f64 = (s.dn[176][29] * ddt_scale);
        let eq108_e1386: f64 = (p.p355 * (nv3 - nv11));
        let eq108_e1386_d_n3: f64 = p.p355;
        let eq108_e1386_d_n11: f64 = (-p.p355);
        let eq108_e1387: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 99, eq108_e1386);
        let eq108_e1387_d_n3: f64 = (eq108_e1386_d_n3 * ddt_scale);
        let eq108_e1387_d_n11: f64 = (eq108_e1386_d_n11 * ddt_scale);
        let eq108_e1388: f64 = (eq108_e1383 + eq108_e1387);
        let eq108_e1388_d_n3: f64 = (eq108_e1383_d_n3 + eq108_e1387_d_n3);
        let eq108_e1388_d_n11: f64 = (eq108_e1383_d_n11 + eq108_e1387_d_n11);
        let eq108_value: f64 = eq108_e1388;
        let eq108_node_derivatives: [f64; 30] = [eq108_e1383_d_n0, eq108_e1383_d_n1, eq108_e1383_d_n2, eq108_e1388_d_n3, eq108_e1383_d_n4, eq108_e1383_d_n5, eq108_e1383_d_n6, eq108_e1383_d_n7, eq108_e1383_d_n8, eq108_e1383_d_n9, eq108_e1383_d_n10, eq108_e1388_d_n11, eq108_e1383_d_n12, eq108_e1383_d_n13, eq108_e1383_d_n14, eq108_e1383_d_n15, eq108_e1383_d_n16, eq108_e1383_d_n17, eq108_e1383_d_n18, eq108_e1383_d_n19, eq108_e1383_d_n20, eq108_e1383_d_n21, eq108_e1383_d_n22, eq108_e1383_d_n23, eq108_e1383_d_n24, eq108_e1383_d_n25, eq108_e1383_d_n26, eq108_e1383_d_n27, eq108_e1383_d_n28, eq108_e1383_d_n29];
        let eq108_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[11]),
            multiplicity * (eq108_value),
            nodes,
            &eq108_node_derivatives,
            branches,
            &eq108_branch_derivatives,
            multiplicity,
        );
        let (eq109_e1396, eq109_e1396_d_n0, eq109_e1396_d_n1, eq109_e1396_d_n2, eq109_e1396_d_n3, eq109_e1396_d_n4, eq109_e1396_d_n5, eq109_e1396_d_n6, eq109_e1396_d_n7, eq109_e1396_d_n8, eq109_e1396_d_n9, eq109_e1396_d_n10, eq109_e1396_d_n11, eq109_e1396_d_n12, eq109_e1396_d_n13, eq109_e1396_d_n14, eq109_e1396_d_n15, eq109_e1396_d_n16, eq109_e1396_d_n17, eq109_e1396_d_n18, eq109_e1396_d_n19, eq109_e1396_d_n20, eq109_e1396_d_n21, eq109_e1396_d_n22, eq109_e1396_d_n23, eq109_e1396_d_n24, eq109_e1396_d_n25, eq109_e1396_d_n26, eq109_e1396_d_n27, eq109_e1396_d_n28, eq109_e1396_d_n29,) = {
    if s.b[1202] {
        let eq109_e1393: f64 = (s.v[0] * (nv11 - nv12));
        let eq109_e1393_d_n11: f64 = s.v[0];
        let eq109_e1393_d_n12: f64 = (-s.v[0]);
        let eq109_e1394: f64 = (s.v[178] + eq109_e1393);
        let eq109_e1394_d_n11: f64 = (s.dn[178][11] + eq109_e1393_d_n11);
        let eq109_e1394_d_n12: f64 = (s.dn[178][12] + eq109_e1393_d_n12);
        (eq109_e1394, s.dn[178][0], s.dn[178][1], s.dn[178][2], s.dn[178][3], s.dn[178][4], s.dn[178][5], s.dn[178][6], s.dn[178][7], s.dn[178][8], s.dn[178][9], s.dn[178][10], eq109_e1394_d_n11, eq109_e1394_d_n12, s.dn[178][13], s.dn[178][14], s.dn[178][15], s.dn[178][16], s.dn[178][17], s.dn[178][18], s.dn[178][19], s.dn[178][20], s.dn[178][21], s.dn[178][22], s.dn[178][23], s.dn[178][24], s.dn[178][25], s.dn[178][26], s.dn[178][27], s.dn[178][28], s.dn[178][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq109_value: f64 = eq109_e1396;
        let eq109_node_derivatives: [f64; 30] = [eq109_e1396_d_n0, eq109_e1396_d_n1, eq109_e1396_d_n2, eq109_e1396_d_n3, eq109_e1396_d_n4, eq109_e1396_d_n5, eq109_e1396_d_n6, eq109_e1396_d_n7, eq109_e1396_d_n8, eq109_e1396_d_n9, eq109_e1396_d_n10, eq109_e1396_d_n11, eq109_e1396_d_n12, eq109_e1396_d_n13, eq109_e1396_d_n14, eq109_e1396_d_n15, eq109_e1396_d_n16, eq109_e1396_d_n17, eq109_e1396_d_n18, eq109_e1396_d_n19, eq109_e1396_d_n20, eq109_e1396_d_n21, eq109_e1396_d_n22, eq109_e1396_d_n23, eq109_e1396_d_n24, eq109_e1396_d_n25, eq109_e1396_d_n26, eq109_e1396_d_n27, eq109_e1396_d_n28, eq109_e1396_d_n29];
        let eq109_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            multiplicity * (eq109_value),
            nodes,
            &eq109_node_derivatives,
            branches,
            &eq109_branch_derivatives,
            multiplicity,
        );
        let (eq110_e1401,) = {
    if (!s.b[1202]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq110_value: f64 = eq110_e1401;
        stamper.stamp_potential_const(
            branches[24],
            eq110_value,
        );
        let (eq111_e1411, eq111_e1411_d_n0, eq111_e1411_d_n1, eq111_e1411_d_n2, eq111_e1411_d_n3, eq111_e1411_d_n4, eq111_e1411_d_n5, eq111_e1411_d_n6, eq111_e1411_d_n7, eq111_e1411_d_n8, eq111_e1411_d_n9, eq111_e1411_d_n10, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_d_n13, eq111_e1411_d_n14, eq111_e1411_d_n15, eq111_e1411_d_n16, eq111_e1411_d_n17, eq111_e1411_d_n18, eq111_e1411_d_n19, eq111_e1411_d_n20, eq111_e1411_d_n21, eq111_e1411_d_n22, eq111_e1411_d_n23, eq111_e1411_d_n24, eq111_e1411_d_n25, eq111_e1411_d_n26, eq111_e1411_d_n27, eq111_e1411_d_n28, eq111_e1411_d_n29,) = {
    if s.b[1348] {
        let eq111_e1404: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 100, s.v[179]);
        let eq111_e1404_d_n0: f64 = (s.dn[179][0] * ddt_scale);
        let eq111_e1404_d_n1: f64 = (s.dn[179][1] * ddt_scale);
        let eq111_e1404_d_n2: f64 = (s.dn[179][2] * ddt_scale);
        let eq111_e1404_d_n3: f64 = (s.dn[179][3] * ddt_scale);
        let eq111_e1404_d_n4: f64 = (s.dn[179][4] * ddt_scale);
        let eq111_e1404_d_n5: f64 = (s.dn[179][5] * ddt_scale);
        let eq111_e1404_d_n6: f64 = (s.dn[179][6] * ddt_scale);
        let eq111_e1404_d_n7: f64 = (s.dn[179][7] * ddt_scale);
        let eq111_e1404_d_n8: f64 = (s.dn[179][8] * ddt_scale);
        let eq111_e1404_d_n9: f64 = (s.dn[179][9] * ddt_scale);
        let eq111_e1404_d_n10: f64 = (s.dn[179][10] * ddt_scale);
        let eq111_e1404_d_n11: f64 = (s.dn[179][11] * ddt_scale);
        let eq111_e1404_d_n12: f64 = (s.dn[179][12] * ddt_scale);
        let eq111_e1404_d_n13: f64 = (s.dn[179][13] * ddt_scale);
        let eq111_e1404_d_n14: f64 = (s.dn[179][14] * ddt_scale);
        let eq111_e1404_d_n15: f64 = (s.dn[179][15] * ddt_scale);
        let eq111_e1404_d_n16: f64 = (s.dn[179][16] * ddt_scale);
        let eq111_e1404_d_n17: f64 = (s.dn[179][17] * ddt_scale);
        let eq111_e1404_d_n18: f64 = (s.dn[179][18] * ddt_scale);
        let eq111_e1404_d_n19: f64 = (s.dn[179][19] * ddt_scale);
        let eq111_e1404_d_n20: f64 = (s.dn[179][20] * ddt_scale);
        let eq111_e1404_d_n21: f64 = (s.dn[179][21] * ddt_scale);
        let eq111_e1404_d_n22: f64 = (s.dn[179][22] * ddt_scale);
        let eq111_e1404_d_n23: f64 = (s.dn[179][23] * ddt_scale);
        let eq111_e1404_d_n24: f64 = (s.dn[179][24] * ddt_scale);
        let eq111_e1404_d_n25: f64 = (s.dn[179][25] * ddt_scale);
        let eq111_e1404_d_n26: f64 = (s.dn[179][26] * ddt_scale);
        let eq111_e1404_d_n27: f64 = (s.dn[179][27] * ddt_scale);
        let eq111_e1404_d_n28: f64 = (s.dn[179][28] * ddt_scale);
        let eq111_e1404_d_n29: f64 = (s.dn[179][29] * ddt_scale);
        let eq111_e1407: f64 = (p.p355 * (nv7 - nv12));
        let eq111_e1407_d_n7: f64 = p.p355;
        let eq111_e1407_d_n12: f64 = (-p.p355);
        let eq111_e1408: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 101, eq111_e1407);
        let eq111_e1408_d_n7: f64 = (eq111_e1407_d_n7 * ddt_scale);
        let eq111_e1408_d_n12: f64 = (eq111_e1407_d_n12 * ddt_scale);
        let eq111_e1409: f64 = (eq111_e1404 + eq111_e1408);
        let eq111_e1409_d_n7: f64 = (eq111_e1404_d_n7 + eq111_e1408_d_n7);
        let eq111_e1409_d_n12: f64 = (eq111_e1404_d_n12 + eq111_e1408_d_n12);
        (eq111_e1409, eq111_e1404_d_n0, eq111_e1404_d_n1, eq111_e1404_d_n2, eq111_e1404_d_n3, eq111_e1404_d_n4, eq111_e1404_d_n5, eq111_e1404_d_n6, eq111_e1409_d_n7, eq111_e1404_d_n8, eq111_e1404_d_n9, eq111_e1404_d_n10, eq111_e1404_d_n11, eq111_e1409_d_n12, eq111_e1404_d_n13, eq111_e1404_d_n14, eq111_e1404_d_n15, eq111_e1404_d_n16, eq111_e1404_d_n17, eq111_e1404_d_n18, eq111_e1404_d_n19, eq111_e1404_d_n20, eq111_e1404_d_n21, eq111_e1404_d_n22, eq111_e1404_d_n23, eq111_e1404_d_n24, eq111_e1404_d_n25, eq111_e1404_d_n26, eq111_e1404_d_n27, eq111_e1404_d_n28, eq111_e1404_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e1411;
        let eq111_node_derivatives: [f64; 30] = [eq111_e1411_d_n0, eq111_e1411_d_n1, eq111_e1411_d_n2, eq111_e1411_d_n3, eq111_e1411_d_n4, eq111_e1411_d_n5, eq111_e1411_d_n6, eq111_e1411_d_n7, eq111_e1411_d_n8, eq111_e1411_d_n9, eq111_e1411_d_n10, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_d_n13, eq111_e1411_d_n14, eq111_e1411_d_n15, eq111_e1411_d_n16, eq111_e1411_d_n17, eq111_e1411_d_n18, eq111_e1411_d_n19, eq111_e1411_d_n20, eq111_e1411_d_n21, eq111_e1411_d_n22, eq111_e1411_d_n23, eq111_e1411_d_n24, eq111_e1411_d_n25, eq111_e1411_d_n26, eq111_e1411_d_n27, eq111_e1411_d_n28, eq111_e1411_d_n29];
        let eq111_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            multiplicity * (eq111_value),
            nodes,
            &eq111_node_derivatives,
            branches,
            &eq111_branch_derivatives,
            multiplicity,
        );
        let (eq112_e1421, eq112_e1421_d_n0, eq112_e1421_d_n1, eq112_e1421_d_n2, eq112_e1421_d_n3, eq112_e1421_d_n4, eq112_e1421_d_n5, eq112_e1421_d_n6, eq112_e1421_d_n7, eq112_e1421_d_n8, eq112_e1421_d_n9, eq112_e1421_d_n10, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_d_n13, eq112_e1421_d_n14, eq112_e1421_d_n15, eq112_e1421_d_n16, eq112_e1421_d_n17, eq112_e1421_d_n18, eq112_e1421_d_n19, eq112_e1421_d_n20, eq112_e1421_d_n21, eq112_e1421_d_n22, eq112_e1421_d_n23, eq112_e1421_d_n24, eq112_e1421_d_n25, eq112_e1421_d_n26, eq112_e1421_d_n27, eq112_e1421_d_n28, eq112_e1421_d_n29,) = {
    if s.b[1348] {
        let eq112_e1414: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 102, s.v[180]);
        let eq112_e1414_d_n0: f64 = (s.dn[180][0] * ddt_scale);
        let eq112_e1414_d_n1: f64 = (s.dn[180][1] * ddt_scale);
        let eq112_e1414_d_n2: f64 = (s.dn[180][2] * ddt_scale);
        let eq112_e1414_d_n3: f64 = (s.dn[180][3] * ddt_scale);
        let eq112_e1414_d_n4: f64 = (s.dn[180][4] * ddt_scale);
        let eq112_e1414_d_n5: f64 = (s.dn[180][5] * ddt_scale);
        let eq112_e1414_d_n6: f64 = (s.dn[180][6] * ddt_scale);
        let eq112_e1414_d_n7: f64 = (s.dn[180][7] * ddt_scale);
        let eq112_e1414_d_n8: f64 = (s.dn[180][8] * ddt_scale);
        let eq112_e1414_d_n9: f64 = (s.dn[180][9] * ddt_scale);
        let eq112_e1414_d_n10: f64 = (s.dn[180][10] * ddt_scale);
        let eq112_e1414_d_n11: f64 = (s.dn[180][11] * ddt_scale);
        let eq112_e1414_d_n12: f64 = (s.dn[180][12] * ddt_scale);
        let eq112_e1414_d_n13: f64 = (s.dn[180][13] * ddt_scale);
        let eq112_e1414_d_n14: f64 = (s.dn[180][14] * ddt_scale);
        let eq112_e1414_d_n15: f64 = (s.dn[180][15] * ddt_scale);
        let eq112_e1414_d_n16: f64 = (s.dn[180][16] * ddt_scale);
        let eq112_e1414_d_n17: f64 = (s.dn[180][17] * ddt_scale);
        let eq112_e1414_d_n18: f64 = (s.dn[180][18] * ddt_scale);
        let eq112_e1414_d_n19: f64 = (s.dn[180][19] * ddt_scale);
        let eq112_e1414_d_n20: f64 = (s.dn[180][20] * ddt_scale);
        let eq112_e1414_d_n21: f64 = (s.dn[180][21] * ddt_scale);
        let eq112_e1414_d_n22: f64 = (s.dn[180][22] * ddt_scale);
        let eq112_e1414_d_n23: f64 = (s.dn[180][23] * ddt_scale);
        let eq112_e1414_d_n24: f64 = (s.dn[180][24] * ddt_scale);
        let eq112_e1414_d_n25: f64 = (s.dn[180][25] * ddt_scale);
        let eq112_e1414_d_n26: f64 = (s.dn[180][26] * ddt_scale);
        let eq112_e1414_d_n27: f64 = (s.dn[180][27] * ddt_scale);
        let eq112_e1414_d_n28: f64 = (s.dn[180][28] * ddt_scale);
        let eq112_e1414_d_n29: f64 = (s.dn[180][29] * ddt_scale);
        let eq112_e1417: f64 = (p.p355 * (nv7 - nv11));
        let eq112_e1417_d_n7: f64 = p.p355;
        let eq112_e1417_d_n11: f64 = (-p.p355);
        let eq112_e1418: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 103, eq112_e1417);
        let eq112_e1418_d_n7: f64 = (eq112_e1417_d_n7 * ddt_scale);
        let eq112_e1418_d_n11: f64 = (eq112_e1417_d_n11 * ddt_scale);
        let eq112_e1419: f64 = (eq112_e1414 + eq112_e1418);
        let eq112_e1419_d_n7: f64 = (eq112_e1414_d_n7 + eq112_e1418_d_n7);
        let eq112_e1419_d_n11: f64 = (eq112_e1414_d_n11 + eq112_e1418_d_n11);
        (eq112_e1419, eq112_e1414_d_n0, eq112_e1414_d_n1, eq112_e1414_d_n2, eq112_e1414_d_n3, eq112_e1414_d_n4, eq112_e1414_d_n5, eq112_e1414_d_n6, eq112_e1419_d_n7, eq112_e1414_d_n8, eq112_e1414_d_n9, eq112_e1414_d_n10, eq112_e1419_d_n11, eq112_e1414_d_n12, eq112_e1414_d_n13, eq112_e1414_d_n14, eq112_e1414_d_n15, eq112_e1414_d_n16, eq112_e1414_d_n17, eq112_e1414_d_n18, eq112_e1414_d_n19, eq112_e1414_d_n20, eq112_e1414_d_n21, eq112_e1414_d_n22, eq112_e1414_d_n23, eq112_e1414_d_n24, eq112_e1414_d_n25, eq112_e1414_d_n26, eq112_e1414_d_n27, eq112_e1414_d_n28, eq112_e1414_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_value: f64 = eq112_e1421;
        let eq112_node_derivatives: [f64; 30] = [eq112_e1421_d_n0, eq112_e1421_d_n1, eq112_e1421_d_n2, eq112_e1421_d_n3, eq112_e1421_d_n4, eq112_e1421_d_n5, eq112_e1421_d_n6, eq112_e1421_d_n7, eq112_e1421_d_n8, eq112_e1421_d_n9, eq112_e1421_d_n10, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_d_n13, eq112_e1421_d_n14, eq112_e1421_d_n15, eq112_e1421_d_n16, eq112_e1421_d_n17, eq112_e1421_d_n18, eq112_e1421_d_n19, eq112_e1421_d_n20, eq112_e1421_d_n21, eq112_e1421_d_n22, eq112_e1421_d_n23, eq112_e1421_d_n24, eq112_e1421_d_n25, eq112_e1421_d_n26, eq112_e1421_d_n27, eq112_e1421_d_n28, eq112_e1421_d_n29];
        let eq112_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            multiplicity * (eq112_value),
            nodes,
            &eq112_node_derivatives,
            branches,
            &eq112_branch_derivatives,
            multiplicity,
        );
        let (eq113_e1431, eq113_e1431_d_n0, eq113_e1431_d_n1, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n5, eq113_e1431_d_n6, eq113_e1431_d_n7, eq113_e1431_d_n8, eq113_e1431_d_n9, eq113_e1431_d_n10, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_d_n13, eq113_e1431_d_n14, eq113_e1431_d_n15, eq113_e1431_d_n16, eq113_e1431_d_n17, eq113_e1431_d_n18, eq113_e1431_d_n19, eq113_e1431_d_n20, eq113_e1431_d_n21, eq113_e1431_d_n22, eq113_e1431_d_n23, eq113_e1431_d_n24, eq113_e1431_d_n25, eq113_e1431_d_n26, eq113_e1431_d_n27, eq113_e1431_d_n28, eq113_e1431_d_n29,) = {
    if s.b[1348] {
        let eq113_e1424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 104, s.v[181]);
        let eq113_e1424_d_n0: f64 = (s.dn[181][0] * ddt_scale);
        let eq113_e1424_d_n1: f64 = (s.dn[181][1] * ddt_scale);
        let eq113_e1424_d_n2: f64 = (s.dn[181][2] * ddt_scale);
        let eq113_e1424_d_n3: f64 = (s.dn[181][3] * ddt_scale);
        let eq113_e1424_d_n4: f64 = (s.dn[181][4] * ddt_scale);
        let eq113_e1424_d_n5: f64 = (s.dn[181][5] * ddt_scale);
        let eq113_e1424_d_n6: f64 = (s.dn[181][6] * ddt_scale);
        let eq113_e1424_d_n7: f64 = (s.dn[181][7] * ddt_scale);
        let eq113_e1424_d_n8: f64 = (s.dn[181][8] * ddt_scale);
        let eq113_e1424_d_n9: f64 = (s.dn[181][9] * ddt_scale);
        let eq113_e1424_d_n10: f64 = (s.dn[181][10] * ddt_scale);
        let eq113_e1424_d_n11: f64 = (s.dn[181][11] * ddt_scale);
        let eq113_e1424_d_n12: f64 = (s.dn[181][12] * ddt_scale);
        let eq113_e1424_d_n13: f64 = (s.dn[181][13] * ddt_scale);
        let eq113_e1424_d_n14: f64 = (s.dn[181][14] * ddt_scale);
        let eq113_e1424_d_n15: f64 = (s.dn[181][15] * ddt_scale);
        let eq113_e1424_d_n16: f64 = (s.dn[181][16] * ddt_scale);
        let eq113_e1424_d_n17: f64 = (s.dn[181][17] * ddt_scale);
        let eq113_e1424_d_n18: f64 = (s.dn[181][18] * ddt_scale);
        let eq113_e1424_d_n19: f64 = (s.dn[181][19] * ddt_scale);
        let eq113_e1424_d_n20: f64 = (s.dn[181][20] * ddt_scale);
        let eq113_e1424_d_n21: f64 = (s.dn[181][21] * ddt_scale);
        let eq113_e1424_d_n22: f64 = (s.dn[181][22] * ddt_scale);
        let eq113_e1424_d_n23: f64 = (s.dn[181][23] * ddt_scale);
        let eq113_e1424_d_n24: f64 = (s.dn[181][24] * ddt_scale);
        let eq113_e1424_d_n25: f64 = (s.dn[181][25] * ddt_scale);
        let eq113_e1424_d_n26: f64 = (s.dn[181][26] * ddt_scale);
        let eq113_e1424_d_n27: f64 = (s.dn[181][27] * ddt_scale);
        let eq113_e1424_d_n28: f64 = (s.dn[181][28] * ddt_scale);
        let eq113_e1424_d_n29: f64 = (s.dn[181][29] * ddt_scale);
        let eq113_e1427: f64 = (p.p355 * (nv2 - nv12));
        let eq113_e1427_d_n2: f64 = p.p355;
        let eq113_e1427_d_n12: f64 = (-p.p355);
        let eq113_e1428: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 105, eq113_e1427);
        let eq113_e1428_d_n2: f64 = (eq113_e1427_d_n2 * ddt_scale);
        let eq113_e1428_d_n12: f64 = (eq113_e1427_d_n12 * ddt_scale);
        let eq113_e1429: f64 = (eq113_e1424 + eq113_e1428);
        let eq113_e1429_d_n2: f64 = (eq113_e1424_d_n2 + eq113_e1428_d_n2);
        let eq113_e1429_d_n12: f64 = (eq113_e1424_d_n12 + eq113_e1428_d_n12);
        (eq113_e1429, eq113_e1424_d_n0, eq113_e1424_d_n1, eq113_e1429_d_n2, eq113_e1424_d_n3, eq113_e1424_d_n4, eq113_e1424_d_n5, eq113_e1424_d_n6, eq113_e1424_d_n7, eq113_e1424_d_n8, eq113_e1424_d_n9, eq113_e1424_d_n10, eq113_e1424_d_n11, eq113_e1429_d_n12, eq113_e1424_d_n13, eq113_e1424_d_n14, eq113_e1424_d_n15, eq113_e1424_d_n16, eq113_e1424_d_n17, eq113_e1424_d_n18, eq113_e1424_d_n19, eq113_e1424_d_n20, eq113_e1424_d_n21, eq113_e1424_d_n22, eq113_e1424_d_n23, eq113_e1424_d_n24, eq113_e1424_d_n25, eq113_e1424_d_n26, eq113_e1424_d_n27, eq113_e1424_d_n28, eq113_e1424_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_value: f64 = eq113_e1431;
        let eq113_node_derivatives: [f64; 30] = [eq113_e1431_d_n0, eq113_e1431_d_n1, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n5, eq113_e1431_d_n6, eq113_e1431_d_n7, eq113_e1431_d_n8, eq113_e1431_d_n9, eq113_e1431_d_n10, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_d_n13, eq113_e1431_d_n14, eq113_e1431_d_n15, eq113_e1431_d_n16, eq113_e1431_d_n17, eq113_e1431_d_n18, eq113_e1431_d_n19, eq113_e1431_d_n20, eq113_e1431_d_n21, eq113_e1431_d_n22, eq113_e1431_d_n23, eq113_e1431_d_n24, eq113_e1431_d_n25, eq113_e1431_d_n26, eq113_e1431_d_n27, eq113_e1431_d_n28, eq113_e1431_d_n29];
        let eq113_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            multiplicity * (eq113_value),
            nodes,
            &eq113_node_derivatives,
            branches,
            &eq113_branch_derivatives,
            multiplicity,
        );
        let (eq114_e1435,) = {
    if s.b[1348] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq114_value: f64 = eq114_e1435;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[11]),
            multiplicity * (eq114_value),
        );
        let (eq115_e1445, eq115_e1445_d_n0, eq115_e1445_d_n1, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n5, eq115_e1445_d_n6, eq115_e1445_d_n7, eq115_e1445_d_n8, eq115_e1445_d_n9, eq115_e1445_d_n10, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_d_n13, eq115_e1445_d_n14, eq115_e1445_d_n15, eq115_e1445_d_n16, eq115_e1445_d_n17, eq115_e1445_d_n18, eq115_e1445_d_n19, eq115_e1445_d_n20, eq115_e1445_d_n21, eq115_e1445_d_n22, eq115_e1445_d_n23, eq115_e1445_d_n24, eq115_e1445_d_n25, eq115_e1445_d_n26, eq115_e1445_d_n27, eq115_e1445_d_n28, eq115_e1445_d_n29,) = {
    if s.b[1348] {
        let eq115_e1438: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 106, s.v[183]);
        let eq115_e1438_d_n0: f64 = (s.dn[183][0] * ddt_scale);
        let eq115_e1438_d_n1: f64 = (s.dn[183][1] * ddt_scale);
        let eq115_e1438_d_n2: f64 = (s.dn[183][2] * ddt_scale);
        let eq115_e1438_d_n3: f64 = (s.dn[183][3] * ddt_scale);
        let eq115_e1438_d_n4: f64 = (s.dn[183][4] * ddt_scale);
        let eq115_e1438_d_n5: f64 = (s.dn[183][5] * ddt_scale);
        let eq115_e1438_d_n6: f64 = (s.dn[183][6] * ddt_scale);
        let eq115_e1438_d_n7: f64 = (s.dn[183][7] * ddt_scale);
        let eq115_e1438_d_n8: f64 = (s.dn[183][8] * ddt_scale);
        let eq115_e1438_d_n9: f64 = (s.dn[183][9] * ddt_scale);
        let eq115_e1438_d_n10: f64 = (s.dn[183][10] * ddt_scale);
        let eq115_e1438_d_n11: f64 = (s.dn[183][11] * ddt_scale);
        let eq115_e1438_d_n12: f64 = (s.dn[183][12] * ddt_scale);
        let eq115_e1438_d_n13: f64 = (s.dn[183][13] * ddt_scale);
        let eq115_e1438_d_n14: f64 = (s.dn[183][14] * ddt_scale);
        let eq115_e1438_d_n15: f64 = (s.dn[183][15] * ddt_scale);
        let eq115_e1438_d_n16: f64 = (s.dn[183][16] * ddt_scale);
        let eq115_e1438_d_n17: f64 = (s.dn[183][17] * ddt_scale);
        let eq115_e1438_d_n18: f64 = (s.dn[183][18] * ddt_scale);
        let eq115_e1438_d_n19: f64 = (s.dn[183][19] * ddt_scale);
        let eq115_e1438_d_n20: f64 = (s.dn[183][20] * ddt_scale);
        let eq115_e1438_d_n21: f64 = (s.dn[183][21] * ddt_scale);
        let eq115_e1438_d_n22: f64 = (s.dn[183][22] * ddt_scale);
        let eq115_e1438_d_n23: f64 = (s.dn[183][23] * ddt_scale);
        let eq115_e1438_d_n24: f64 = (s.dn[183][24] * ddt_scale);
        let eq115_e1438_d_n25: f64 = (s.dn[183][25] * ddt_scale);
        let eq115_e1438_d_n26: f64 = (s.dn[183][26] * ddt_scale);
        let eq115_e1438_d_n27: f64 = (s.dn[183][27] * ddt_scale);
        let eq115_e1438_d_n28: f64 = (s.dn[183][28] * ddt_scale);
        let eq115_e1438_d_n29: f64 = (s.dn[183][29] * ddt_scale);
        let eq115_e1441: f64 = (p.p355 * (nv7 - nv9));
        let eq115_e1441_d_n7: f64 = p.p355;
        let eq115_e1441_d_n9: f64 = (-p.p355);
        let eq115_e1442: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 107, eq115_e1441);
        let eq115_e1442_d_n7: f64 = (eq115_e1441_d_n7 * ddt_scale);
        let eq115_e1442_d_n9: f64 = (eq115_e1441_d_n9 * ddt_scale);
        let eq115_e1443: f64 = (eq115_e1438 + eq115_e1442);
        let eq115_e1443_d_n7: f64 = (eq115_e1438_d_n7 + eq115_e1442_d_n7);
        let eq115_e1443_d_n9: f64 = (eq115_e1438_d_n9 + eq115_e1442_d_n9);
        (eq115_e1443, eq115_e1438_d_n0, eq115_e1438_d_n1, eq115_e1438_d_n2, eq115_e1438_d_n3, eq115_e1438_d_n4, eq115_e1438_d_n5, eq115_e1438_d_n6, eq115_e1443_d_n7, eq115_e1438_d_n8, eq115_e1443_d_n9, eq115_e1438_d_n10, eq115_e1438_d_n11, eq115_e1438_d_n12, eq115_e1438_d_n13, eq115_e1438_d_n14, eq115_e1438_d_n15, eq115_e1438_d_n16, eq115_e1438_d_n17, eq115_e1438_d_n18, eq115_e1438_d_n19, eq115_e1438_d_n20, eq115_e1438_d_n21, eq115_e1438_d_n22, eq115_e1438_d_n23, eq115_e1438_d_n24, eq115_e1438_d_n25, eq115_e1438_d_n26, eq115_e1438_d_n27, eq115_e1438_d_n28, eq115_e1438_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq115_value: f64 = eq115_e1445;
        let eq115_node_derivatives: [f64; 30] = [eq115_e1445_d_n0, eq115_e1445_d_n1, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n5, eq115_e1445_d_n6, eq115_e1445_d_n7, eq115_e1445_d_n8, eq115_e1445_d_n9, eq115_e1445_d_n10, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_d_n13, eq115_e1445_d_n14, eq115_e1445_d_n15, eq115_e1445_d_n16, eq115_e1445_d_n17, eq115_e1445_d_n18, eq115_e1445_d_n19, eq115_e1445_d_n20, eq115_e1445_d_n21, eq115_e1445_d_n22, eq115_e1445_d_n23, eq115_e1445_d_n24, eq115_e1445_d_n25, eq115_e1445_d_n26, eq115_e1445_d_n27, eq115_e1445_d_n28, eq115_e1445_d_n29];
        let eq115_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq115_value),
            nodes,
            &eq115_node_derivatives,
            branches,
            &eq115_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_10(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq116_e1456, eq116_e1456_d_n0, eq116_e1456_d_n1, eq116_e1456_d_n2, eq116_e1456_d_n3, eq116_e1456_d_n4, eq116_e1456_d_n5, eq116_e1456_d_n6, eq116_e1456_d_n7, eq116_e1456_d_n8, eq116_e1456_d_n9, eq116_e1456_d_n10, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_d_n13, eq116_e1456_d_n14, eq116_e1456_d_n15, eq116_e1456_d_n16, eq116_e1456_d_n17, eq116_e1456_d_n18, eq116_e1456_d_n19, eq116_e1456_d_n20, eq116_e1456_d_n21, eq116_e1456_d_n22, eq116_e1456_d_n23, eq116_e1456_d_n24, eq116_e1456_d_n25, eq116_e1456_d_n26, eq116_e1456_d_n27, eq116_e1456_d_n28, eq116_e1456_d_n29,) = {
    if (!s.b[1348]) {
        let eq116_e1449: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 108, s.v[179]);
        let eq116_e1449_d_n0: f64 = (s.dn[179][0] * ddt_scale);
        let eq116_e1449_d_n1: f64 = (s.dn[179][1] * ddt_scale);
        let eq116_e1449_d_n2: f64 = (s.dn[179][2] * ddt_scale);
        let eq116_e1449_d_n3: f64 = (s.dn[179][3] * ddt_scale);
        let eq116_e1449_d_n4: f64 = (s.dn[179][4] * ddt_scale);
        let eq116_e1449_d_n5: f64 = (s.dn[179][5] * ddt_scale);
        let eq116_e1449_d_n6: f64 = (s.dn[179][6] * ddt_scale);
        let eq116_e1449_d_n7: f64 = (s.dn[179][7] * ddt_scale);
        let eq116_e1449_d_n8: f64 = (s.dn[179][8] * ddt_scale);
        let eq116_e1449_d_n9: f64 = (s.dn[179][9] * ddt_scale);
        let eq116_e1449_d_n10: f64 = (s.dn[179][10] * ddt_scale);
        let eq116_e1449_d_n11: f64 = (s.dn[179][11] * ddt_scale);
        let eq116_e1449_d_n12: f64 = (s.dn[179][12] * ddt_scale);
        let eq116_e1449_d_n13: f64 = (s.dn[179][13] * ddt_scale);
        let eq116_e1449_d_n14: f64 = (s.dn[179][14] * ddt_scale);
        let eq116_e1449_d_n15: f64 = (s.dn[179][15] * ddt_scale);
        let eq116_e1449_d_n16: f64 = (s.dn[179][16] * ddt_scale);
        let eq116_e1449_d_n17: f64 = (s.dn[179][17] * ddt_scale);
        let eq116_e1449_d_n18: f64 = (s.dn[179][18] * ddt_scale);
        let eq116_e1449_d_n19: f64 = (s.dn[179][19] * ddt_scale);
        let eq116_e1449_d_n20: f64 = (s.dn[179][20] * ddt_scale);
        let eq116_e1449_d_n21: f64 = (s.dn[179][21] * ddt_scale);
        let eq116_e1449_d_n22: f64 = (s.dn[179][22] * ddt_scale);
        let eq116_e1449_d_n23: f64 = (s.dn[179][23] * ddt_scale);
        let eq116_e1449_d_n24: f64 = (s.dn[179][24] * ddt_scale);
        let eq116_e1449_d_n25: f64 = (s.dn[179][25] * ddt_scale);
        let eq116_e1449_d_n26: f64 = (s.dn[179][26] * ddt_scale);
        let eq116_e1449_d_n27: f64 = (s.dn[179][27] * ddt_scale);
        let eq116_e1449_d_n28: f64 = (s.dn[179][28] * ddt_scale);
        let eq116_e1449_d_n29: f64 = (s.dn[179][29] * ddt_scale);
        let eq116_e1452: f64 = (p.p355 * (nv2 - nv12));
        let eq116_e1452_d_n2: f64 = p.p355;
        let eq116_e1452_d_n12: f64 = (-p.p355);
        let eq116_e1453: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 109, eq116_e1452);
        let eq116_e1453_d_n2: f64 = (eq116_e1452_d_n2 * ddt_scale);
        let eq116_e1453_d_n12: f64 = (eq116_e1452_d_n12 * ddt_scale);
        let eq116_e1454: f64 = (eq116_e1449 + eq116_e1453);
        let eq116_e1454_d_n2: f64 = (eq116_e1449_d_n2 + eq116_e1453_d_n2);
        let eq116_e1454_d_n12: f64 = (eq116_e1449_d_n12 + eq116_e1453_d_n12);
        (eq116_e1454, eq116_e1449_d_n0, eq116_e1449_d_n1, eq116_e1454_d_n2, eq116_e1449_d_n3, eq116_e1449_d_n4, eq116_e1449_d_n5, eq116_e1449_d_n6, eq116_e1449_d_n7, eq116_e1449_d_n8, eq116_e1449_d_n9, eq116_e1449_d_n10, eq116_e1449_d_n11, eq116_e1454_d_n12, eq116_e1449_d_n13, eq116_e1449_d_n14, eq116_e1449_d_n15, eq116_e1449_d_n16, eq116_e1449_d_n17, eq116_e1449_d_n18, eq116_e1449_d_n19, eq116_e1449_d_n20, eq116_e1449_d_n21, eq116_e1449_d_n22, eq116_e1449_d_n23, eq116_e1449_d_n24, eq116_e1449_d_n25, eq116_e1449_d_n26, eq116_e1449_d_n27, eq116_e1449_d_n28, eq116_e1449_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq116_value: f64 = eq116_e1456;
        let eq116_node_derivatives: [f64; 30] = [eq116_e1456_d_n0, eq116_e1456_d_n1, eq116_e1456_d_n2, eq116_e1456_d_n3, eq116_e1456_d_n4, eq116_e1456_d_n5, eq116_e1456_d_n6, eq116_e1456_d_n7, eq116_e1456_d_n8, eq116_e1456_d_n9, eq116_e1456_d_n10, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_d_n13, eq116_e1456_d_n14, eq116_e1456_d_n15, eq116_e1456_d_n16, eq116_e1456_d_n17, eq116_e1456_d_n18, eq116_e1456_d_n19, eq116_e1456_d_n20, eq116_e1456_d_n21, eq116_e1456_d_n22, eq116_e1456_d_n23, eq116_e1456_d_n24, eq116_e1456_d_n25, eq116_e1456_d_n26, eq116_e1456_d_n27, eq116_e1456_d_n28, eq116_e1456_d_n29];
        let eq116_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            multiplicity * (eq116_value),
            nodes,
            &eq116_node_derivatives,
            branches,
            &eq116_branch_derivatives,
            multiplicity,
        );
        let (eq117_e1467, eq117_e1467_d_n0, eq117_e1467_d_n1, eq117_e1467_d_n2, eq117_e1467_d_n3, eq117_e1467_d_n4, eq117_e1467_d_n5, eq117_e1467_d_n6, eq117_e1467_d_n7, eq117_e1467_d_n8, eq117_e1467_d_n9, eq117_e1467_d_n10, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_d_n13, eq117_e1467_d_n14, eq117_e1467_d_n15, eq117_e1467_d_n16, eq117_e1467_d_n17, eq117_e1467_d_n18, eq117_e1467_d_n19, eq117_e1467_d_n20, eq117_e1467_d_n21, eq117_e1467_d_n22, eq117_e1467_d_n23, eq117_e1467_d_n24, eq117_e1467_d_n25, eq117_e1467_d_n26, eq117_e1467_d_n27, eq117_e1467_d_n28, eq117_e1467_d_n29,) = {
    if (!s.b[1348]) {
        let eq117_e1460: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 110, s.v[180]);
        let eq117_e1460_d_n0: f64 = (s.dn[180][0] * ddt_scale);
        let eq117_e1460_d_n1: f64 = (s.dn[180][1] * ddt_scale);
        let eq117_e1460_d_n2: f64 = (s.dn[180][2] * ddt_scale);
        let eq117_e1460_d_n3: f64 = (s.dn[180][3] * ddt_scale);
        let eq117_e1460_d_n4: f64 = (s.dn[180][4] * ddt_scale);
        let eq117_e1460_d_n5: f64 = (s.dn[180][5] * ddt_scale);
        let eq117_e1460_d_n6: f64 = (s.dn[180][6] * ddt_scale);
        let eq117_e1460_d_n7: f64 = (s.dn[180][7] * ddt_scale);
        let eq117_e1460_d_n8: f64 = (s.dn[180][8] * ddt_scale);
        let eq117_e1460_d_n9: f64 = (s.dn[180][9] * ddt_scale);
        let eq117_e1460_d_n10: f64 = (s.dn[180][10] * ddt_scale);
        let eq117_e1460_d_n11: f64 = (s.dn[180][11] * ddt_scale);
        let eq117_e1460_d_n12: f64 = (s.dn[180][12] * ddt_scale);
        let eq117_e1460_d_n13: f64 = (s.dn[180][13] * ddt_scale);
        let eq117_e1460_d_n14: f64 = (s.dn[180][14] * ddt_scale);
        let eq117_e1460_d_n15: f64 = (s.dn[180][15] * ddt_scale);
        let eq117_e1460_d_n16: f64 = (s.dn[180][16] * ddt_scale);
        let eq117_e1460_d_n17: f64 = (s.dn[180][17] * ddt_scale);
        let eq117_e1460_d_n18: f64 = (s.dn[180][18] * ddt_scale);
        let eq117_e1460_d_n19: f64 = (s.dn[180][19] * ddt_scale);
        let eq117_e1460_d_n20: f64 = (s.dn[180][20] * ddt_scale);
        let eq117_e1460_d_n21: f64 = (s.dn[180][21] * ddt_scale);
        let eq117_e1460_d_n22: f64 = (s.dn[180][22] * ddt_scale);
        let eq117_e1460_d_n23: f64 = (s.dn[180][23] * ddt_scale);
        let eq117_e1460_d_n24: f64 = (s.dn[180][24] * ddt_scale);
        let eq117_e1460_d_n25: f64 = (s.dn[180][25] * ddt_scale);
        let eq117_e1460_d_n26: f64 = (s.dn[180][26] * ddt_scale);
        let eq117_e1460_d_n27: f64 = (s.dn[180][27] * ddt_scale);
        let eq117_e1460_d_n28: f64 = (s.dn[180][28] * ddt_scale);
        let eq117_e1460_d_n29: f64 = (s.dn[180][29] * ddt_scale);
        let eq117_e1463: f64 = (p.p355 * (nv2 - nv11));
        let eq117_e1463_d_n2: f64 = p.p355;
        let eq117_e1463_d_n11: f64 = (-p.p355);
        let eq117_e1464: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 111, eq117_e1463);
        let eq117_e1464_d_n2: f64 = (eq117_e1463_d_n2 * ddt_scale);
        let eq117_e1464_d_n11: f64 = (eq117_e1463_d_n11 * ddt_scale);
        let eq117_e1465: f64 = (eq117_e1460 + eq117_e1464);
        let eq117_e1465_d_n2: f64 = (eq117_e1460_d_n2 + eq117_e1464_d_n2);
        let eq117_e1465_d_n11: f64 = (eq117_e1460_d_n11 + eq117_e1464_d_n11);
        (eq117_e1465, eq117_e1460_d_n0, eq117_e1460_d_n1, eq117_e1465_d_n2, eq117_e1460_d_n3, eq117_e1460_d_n4, eq117_e1460_d_n5, eq117_e1460_d_n6, eq117_e1460_d_n7, eq117_e1460_d_n8, eq117_e1460_d_n9, eq117_e1460_d_n10, eq117_e1465_d_n11, eq117_e1460_d_n12, eq117_e1460_d_n13, eq117_e1460_d_n14, eq117_e1460_d_n15, eq117_e1460_d_n16, eq117_e1460_d_n17, eq117_e1460_d_n18, eq117_e1460_d_n19, eq117_e1460_d_n20, eq117_e1460_d_n21, eq117_e1460_d_n22, eq117_e1460_d_n23, eq117_e1460_d_n24, eq117_e1460_d_n25, eq117_e1460_d_n26, eq117_e1460_d_n27, eq117_e1460_d_n28, eq117_e1460_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq117_value: f64 = eq117_e1467;
        let eq117_node_derivatives: [f64; 30] = [eq117_e1467_d_n0, eq117_e1467_d_n1, eq117_e1467_d_n2, eq117_e1467_d_n3, eq117_e1467_d_n4, eq117_e1467_d_n5, eq117_e1467_d_n6, eq117_e1467_d_n7, eq117_e1467_d_n8, eq117_e1467_d_n9, eq117_e1467_d_n10, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_d_n13, eq117_e1467_d_n14, eq117_e1467_d_n15, eq117_e1467_d_n16, eq117_e1467_d_n17, eq117_e1467_d_n18, eq117_e1467_d_n19, eq117_e1467_d_n20, eq117_e1467_d_n21, eq117_e1467_d_n22, eq117_e1467_d_n23, eq117_e1467_d_n24, eq117_e1467_d_n25, eq117_e1467_d_n26, eq117_e1467_d_n27, eq117_e1467_d_n28, eq117_e1467_d_n29];
        let eq117_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            multiplicity * (eq117_value),
            nodes,
            &eq117_node_derivatives,
            branches,
            &eq117_branch_derivatives,
            multiplicity,
        );
        let (eq118_e1478, eq118_e1478_d_n0, eq118_e1478_d_n1, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n5, eq118_e1478_d_n6, eq118_e1478_d_n7, eq118_e1478_d_n8, eq118_e1478_d_n9, eq118_e1478_d_n10, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_d_n13, eq118_e1478_d_n14, eq118_e1478_d_n15, eq118_e1478_d_n16, eq118_e1478_d_n17, eq118_e1478_d_n18, eq118_e1478_d_n19, eq118_e1478_d_n20, eq118_e1478_d_n21, eq118_e1478_d_n22, eq118_e1478_d_n23, eq118_e1478_d_n24, eq118_e1478_d_n25, eq118_e1478_d_n26, eq118_e1478_d_n27, eq118_e1478_d_n28, eq118_e1478_d_n29,) = {
    if (!s.b[1348]) {
        let eq118_e1471: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 112, s.v[181]);
        let eq118_e1471_d_n0: f64 = (s.dn[181][0] * ddt_scale);
        let eq118_e1471_d_n1: f64 = (s.dn[181][1] * ddt_scale);
        let eq118_e1471_d_n2: f64 = (s.dn[181][2] * ddt_scale);
        let eq118_e1471_d_n3: f64 = (s.dn[181][3] * ddt_scale);
        let eq118_e1471_d_n4: f64 = (s.dn[181][4] * ddt_scale);
        let eq118_e1471_d_n5: f64 = (s.dn[181][5] * ddt_scale);
        let eq118_e1471_d_n6: f64 = (s.dn[181][6] * ddt_scale);
        let eq118_e1471_d_n7: f64 = (s.dn[181][7] * ddt_scale);
        let eq118_e1471_d_n8: f64 = (s.dn[181][8] * ddt_scale);
        let eq118_e1471_d_n9: f64 = (s.dn[181][9] * ddt_scale);
        let eq118_e1471_d_n10: f64 = (s.dn[181][10] * ddt_scale);
        let eq118_e1471_d_n11: f64 = (s.dn[181][11] * ddt_scale);
        let eq118_e1471_d_n12: f64 = (s.dn[181][12] * ddt_scale);
        let eq118_e1471_d_n13: f64 = (s.dn[181][13] * ddt_scale);
        let eq118_e1471_d_n14: f64 = (s.dn[181][14] * ddt_scale);
        let eq118_e1471_d_n15: f64 = (s.dn[181][15] * ddt_scale);
        let eq118_e1471_d_n16: f64 = (s.dn[181][16] * ddt_scale);
        let eq118_e1471_d_n17: f64 = (s.dn[181][17] * ddt_scale);
        let eq118_e1471_d_n18: f64 = (s.dn[181][18] * ddt_scale);
        let eq118_e1471_d_n19: f64 = (s.dn[181][19] * ddt_scale);
        let eq118_e1471_d_n20: f64 = (s.dn[181][20] * ddt_scale);
        let eq118_e1471_d_n21: f64 = (s.dn[181][21] * ddt_scale);
        let eq118_e1471_d_n22: f64 = (s.dn[181][22] * ddt_scale);
        let eq118_e1471_d_n23: f64 = (s.dn[181][23] * ddt_scale);
        let eq118_e1471_d_n24: f64 = (s.dn[181][24] * ddt_scale);
        let eq118_e1471_d_n25: f64 = (s.dn[181][25] * ddt_scale);
        let eq118_e1471_d_n26: f64 = (s.dn[181][26] * ddt_scale);
        let eq118_e1471_d_n27: f64 = (s.dn[181][27] * ddt_scale);
        let eq118_e1471_d_n28: f64 = (s.dn[181][28] * ddt_scale);
        let eq118_e1471_d_n29: f64 = (s.dn[181][29] * ddt_scale);
        let eq118_e1474: f64 = (p.p355 * (nv7 - nv12));
        let eq118_e1474_d_n7: f64 = p.p355;
        let eq118_e1474_d_n12: f64 = (-p.p355);
        let eq118_e1475: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 113, eq118_e1474);
        let eq118_e1475_d_n7: f64 = (eq118_e1474_d_n7 * ddt_scale);
        let eq118_e1475_d_n12: f64 = (eq118_e1474_d_n12 * ddt_scale);
        let eq118_e1476: f64 = (eq118_e1471 + eq118_e1475);
        let eq118_e1476_d_n7: f64 = (eq118_e1471_d_n7 + eq118_e1475_d_n7);
        let eq118_e1476_d_n12: f64 = (eq118_e1471_d_n12 + eq118_e1475_d_n12);
        (eq118_e1476, eq118_e1471_d_n0, eq118_e1471_d_n1, eq118_e1471_d_n2, eq118_e1471_d_n3, eq118_e1471_d_n4, eq118_e1471_d_n5, eq118_e1471_d_n6, eq118_e1476_d_n7, eq118_e1471_d_n8, eq118_e1471_d_n9, eq118_e1471_d_n10, eq118_e1471_d_n11, eq118_e1476_d_n12, eq118_e1471_d_n13, eq118_e1471_d_n14, eq118_e1471_d_n15, eq118_e1471_d_n16, eq118_e1471_d_n17, eq118_e1471_d_n18, eq118_e1471_d_n19, eq118_e1471_d_n20, eq118_e1471_d_n21, eq118_e1471_d_n22, eq118_e1471_d_n23, eq118_e1471_d_n24, eq118_e1471_d_n25, eq118_e1471_d_n26, eq118_e1471_d_n27, eq118_e1471_d_n28, eq118_e1471_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq118_value: f64 = eq118_e1478;
        let eq118_node_derivatives: [f64; 30] = [eq118_e1478_d_n0, eq118_e1478_d_n1, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n5, eq118_e1478_d_n6, eq118_e1478_d_n7, eq118_e1478_d_n8, eq118_e1478_d_n9, eq118_e1478_d_n10, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_d_n13, eq118_e1478_d_n14, eq118_e1478_d_n15, eq118_e1478_d_n16, eq118_e1478_d_n17, eq118_e1478_d_n18, eq118_e1478_d_n19, eq118_e1478_d_n20, eq118_e1478_d_n21, eq118_e1478_d_n22, eq118_e1478_d_n23, eq118_e1478_d_n24, eq118_e1478_d_n25, eq118_e1478_d_n26, eq118_e1478_d_n27, eq118_e1478_d_n28, eq118_e1478_d_n29];
        let eq118_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            multiplicity * (eq118_value),
            nodes,
            &eq118_node_derivatives,
            branches,
            &eq118_branch_derivatives,
            multiplicity,
        );
        let (eq119_e1483,) = {
    if (!s.b[1348]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq119_value: f64 = eq119_e1483;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[11]),
            multiplicity * (eq119_value),
        );
        let (eq120_e1488,) = {
    if (!s.b[1348]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq120_value: f64 = eq120_e1488;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq120_value),
        );
        let eq121_e1490: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 114, s.v[182]);
        let eq121_e1490_d_n0: f64 = (s.dn[182][0] * ddt_scale);
        let eq121_e1490_d_n1: f64 = (s.dn[182][1] * ddt_scale);
        let eq121_e1490_d_n2: f64 = (s.dn[182][2] * ddt_scale);
        let eq121_e1490_d_n3: f64 = (s.dn[182][3] * ddt_scale);
        let eq121_e1490_d_n4: f64 = (s.dn[182][4] * ddt_scale);
        let eq121_e1490_d_n5: f64 = (s.dn[182][5] * ddt_scale);
        let eq121_e1490_d_n6: f64 = (s.dn[182][6] * ddt_scale);
        let eq121_e1490_d_n7: f64 = (s.dn[182][7] * ddt_scale);
        let eq121_e1490_d_n8: f64 = (s.dn[182][8] * ddt_scale);
        let eq121_e1490_d_n9: f64 = (s.dn[182][9] * ddt_scale);
        let eq121_e1490_d_n10: f64 = (s.dn[182][10] * ddt_scale);
        let eq121_e1490_d_n11: f64 = (s.dn[182][11] * ddt_scale);
        let eq121_e1490_d_n12: f64 = (s.dn[182][12] * ddt_scale);
        let eq121_e1490_d_n13: f64 = (s.dn[182][13] * ddt_scale);
        let eq121_e1490_d_n14: f64 = (s.dn[182][14] * ddt_scale);
        let eq121_e1490_d_n15: f64 = (s.dn[182][15] * ddt_scale);
        let eq121_e1490_d_n16: f64 = (s.dn[182][16] * ddt_scale);
        let eq121_e1490_d_n17: f64 = (s.dn[182][17] * ddt_scale);
        let eq121_e1490_d_n18: f64 = (s.dn[182][18] * ddt_scale);
        let eq121_e1490_d_n19: f64 = (s.dn[182][19] * ddt_scale);
        let eq121_e1490_d_n20: f64 = (s.dn[182][20] * ddt_scale);
        let eq121_e1490_d_n21: f64 = (s.dn[182][21] * ddt_scale);
        let eq121_e1490_d_n22: f64 = (s.dn[182][22] * ddt_scale);
        let eq121_e1490_d_n23: f64 = (s.dn[182][23] * ddt_scale);
        let eq121_e1490_d_n24: f64 = (s.dn[182][24] * ddt_scale);
        let eq121_e1490_d_n25: f64 = (s.dn[182][25] * ddt_scale);
        let eq121_e1490_d_n26: f64 = (s.dn[182][26] * ddt_scale);
        let eq121_e1490_d_n27: f64 = (s.dn[182][27] * ddt_scale);
        let eq121_e1490_d_n28: f64 = (s.dn[182][28] * ddt_scale);
        let eq121_e1490_d_n29: f64 = (s.dn[182][29] * ddt_scale);
        let eq121_e1493: f64 = (p.p355 * (nv3 - nv12));
        let eq121_e1493_d_n3: f64 = p.p355;
        let eq121_e1493_d_n12: f64 = (-p.p355);
        let eq121_e1494: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 115, eq121_e1493);
        let eq121_e1494_d_n3: f64 = (eq121_e1493_d_n3 * ddt_scale);
        let eq121_e1494_d_n12: f64 = (eq121_e1493_d_n12 * ddt_scale);
        let eq121_e1495: f64 = (eq121_e1490 + eq121_e1494);
        let eq121_e1495_d_n3: f64 = (eq121_e1490_d_n3 + eq121_e1494_d_n3);
        let eq121_e1495_d_n12: f64 = (eq121_e1490_d_n12 + eq121_e1494_d_n12);
        let eq121_value: f64 = eq121_e1495;
        let eq121_node_derivatives: [f64; 30] = [eq121_e1490_d_n0, eq121_e1490_d_n1, eq121_e1490_d_n2, eq121_e1495_d_n3, eq121_e1490_d_n4, eq121_e1490_d_n5, eq121_e1490_d_n6, eq121_e1490_d_n7, eq121_e1490_d_n8, eq121_e1490_d_n9, eq121_e1490_d_n10, eq121_e1490_d_n11, eq121_e1495_d_n12, eq121_e1490_d_n13, eq121_e1490_d_n14, eq121_e1490_d_n15, eq121_e1490_d_n16, eq121_e1490_d_n17, eq121_e1490_d_n18, eq121_e1490_d_n19, eq121_e1490_d_n20, eq121_e1490_d_n21, eq121_e1490_d_n22, eq121_e1490_d_n23, eq121_e1490_d_n24, eq121_e1490_d_n25, eq121_e1490_d_n26, eq121_e1490_d_n27, eq121_e1490_d_n28, eq121_e1490_d_n29];
        let eq121_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[12]),
            multiplicity * (eq121_value),
            nodes,
            &eq121_node_derivatives,
            branches,
            &eq121_branch_derivatives,
            multiplicity,
        );
        let (eq122_e1503, eq122_e1503_d_n0, eq122_e1503_d_n1, eq122_e1503_d_n2, eq122_e1503_d_n3, eq122_e1503_d_n4, eq122_e1503_d_n5, eq122_e1503_d_n6, eq122_e1503_d_n7, eq122_e1503_d_n8, eq122_e1503_d_n9, eq122_e1503_d_n10, eq122_e1503_d_n11, eq122_e1503_d_n12, eq122_e1503_d_n13, eq122_e1503_d_n14, eq122_e1503_d_n15, eq122_e1503_d_n16, eq122_e1503_d_n17, eq122_e1503_d_n18, eq122_e1503_d_n19, eq122_e1503_d_n20, eq122_e1503_d_n21, eq122_e1503_d_n22, eq122_e1503_d_n23, eq122_e1503_d_n24, eq122_e1503_d_n25, eq122_e1503_d_n26, eq122_e1503_d_n27, eq122_e1503_d_n28, eq122_e1503_d_n29,) = {
    if s.b[1349] {
        let eq122_e1500: f64 = (s.v[0] * (nv12 - nv13));
        let eq122_e1500_d_n12: f64 = s.v[0];
        let eq122_e1500_d_n13: f64 = (-s.v[0]);
        let eq122_e1501: f64 = (s.v[184] + eq122_e1500);
        let eq122_e1501_d_n12: f64 = (s.dn[184][12] + eq122_e1500_d_n12);
        let eq122_e1501_d_n13: f64 = (s.dn[184][13] + eq122_e1500_d_n13);
        (eq122_e1501, s.dn[184][0], s.dn[184][1], s.dn[184][2], s.dn[184][3], s.dn[184][4], s.dn[184][5], s.dn[184][6], s.dn[184][7], s.dn[184][8], s.dn[184][9], s.dn[184][10], s.dn[184][11], eq122_e1501_d_n12, eq122_e1501_d_n13, s.dn[184][14], s.dn[184][15], s.dn[184][16], s.dn[184][17], s.dn[184][18], s.dn[184][19], s.dn[184][20], s.dn[184][21], s.dn[184][22], s.dn[184][23], s.dn[184][24], s.dn[184][25], s.dn[184][26], s.dn[184][27], s.dn[184][28], s.dn[184][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_value: f64 = eq122_e1503;
        let eq122_node_derivatives: [f64; 30] = [eq122_e1503_d_n0, eq122_e1503_d_n1, eq122_e1503_d_n2, eq122_e1503_d_n3, eq122_e1503_d_n4, eq122_e1503_d_n5, eq122_e1503_d_n6, eq122_e1503_d_n7, eq122_e1503_d_n8, eq122_e1503_d_n9, eq122_e1503_d_n10, eq122_e1503_d_n11, eq122_e1503_d_n12, eq122_e1503_d_n13, eq122_e1503_d_n14, eq122_e1503_d_n15, eq122_e1503_d_n16, eq122_e1503_d_n17, eq122_e1503_d_n18, eq122_e1503_d_n19, eq122_e1503_d_n20, eq122_e1503_d_n21, eq122_e1503_d_n22, eq122_e1503_d_n23, eq122_e1503_d_n24, eq122_e1503_d_n25, eq122_e1503_d_n26, eq122_e1503_d_n27, eq122_e1503_d_n28, eq122_e1503_d_n29];
        let eq122_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[13]),
            multiplicity * (eq122_value),
            nodes,
            &eq122_node_derivatives,
            branches,
            &eq122_branch_derivatives,
            multiplicity,
        );
        let (eq123_e1508,) = {
    if (!s.b[1349]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq123_value: f64 = eq123_e1508;
        stamper.stamp_potential_const(
            branches[25],
            eq123_value,
        );
        let (eq124_e1518, eq124_e1518_d_n0, eq124_e1518_d_n1, eq124_e1518_d_n2, eq124_e1518_d_n3, eq124_e1518_d_n4, eq124_e1518_d_n5, eq124_e1518_d_n6, eq124_e1518_d_n7, eq124_e1518_d_n8, eq124_e1518_d_n9, eq124_e1518_d_n10, eq124_e1518_d_n11, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_d_n14, eq124_e1518_d_n15, eq124_e1518_d_n16, eq124_e1518_d_n17, eq124_e1518_d_n18, eq124_e1518_d_n19, eq124_e1518_d_n20, eq124_e1518_d_n21, eq124_e1518_d_n22, eq124_e1518_d_n23, eq124_e1518_d_n24, eq124_e1518_d_n25, eq124_e1518_d_n26, eq124_e1518_d_n27, eq124_e1518_d_n28, eq124_e1518_d_n29,) = {
    if s.b[1495] {
        let eq124_e1511: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 116, s.v[185]);
        let eq124_e1511_d_n0: f64 = (s.dn[185][0] * ddt_scale);
        let eq124_e1511_d_n1: f64 = (s.dn[185][1] * ddt_scale);
        let eq124_e1511_d_n2: f64 = (s.dn[185][2] * ddt_scale);
        let eq124_e1511_d_n3: f64 = (s.dn[185][3] * ddt_scale);
        let eq124_e1511_d_n4: f64 = (s.dn[185][4] * ddt_scale);
        let eq124_e1511_d_n5: f64 = (s.dn[185][5] * ddt_scale);
        let eq124_e1511_d_n6: f64 = (s.dn[185][6] * ddt_scale);
        let eq124_e1511_d_n7: f64 = (s.dn[185][7] * ddt_scale);
        let eq124_e1511_d_n8: f64 = (s.dn[185][8] * ddt_scale);
        let eq124_e1511_d_n9: f64 = (s.dn[185][9] * ddt_scale);
        let eq124_e1511_d_n10: f64 = (s.dn[185][10] * ddt_scale);
        let eq124_e1511_d_n11: f64 = (s.dn[185][11] * ddt_scale);
        let eq124_e1511_d_n12: f64 = (s.dn[185][12] * ddt_scale);
        let eq124_e1511_d_n13: f64 = (s.dn[185][13] * ddt_scale);
        let eq124_e1511_d_n14: f64 = (s.dn[185][14] * ddt_scale);
        let eq124_e1511_d_n15: f64 = (s.dn[185][15] * ddt_scale);
        let eq124_e1511_d_n16: f64 = (s.dn[185][16] * ddt_scale);
        let eq124_e1511_d_n17: f64 = (s.dn[185][17] * ddt_scale);
        let eq124_e1511_d_n18: f64 = (s.dn[185][18] * ddt_scale);
        let eq124_e1511_d_n19: f64 = (s.dn[185][19] * ddt_scale);
        let eq124_e1511_d_n20: f64 = (s.dn[185][20] * ddt_scale);
        let eq124_e1511_d_n21: f64 = (s.dn[185][21] * ddt_scale);
        let eq124_e1511_d_n22: f64 = (s.dn[185][22] * ddt_scale);
        let eq124_e1511_d_n23: f64 = (s.dn[185][23] * ddt_scale);
        let eq124_e1511_d_n24: f64 = (s.dn[185][24] * ddt_scale);
        let eq124_e1511_d_n25: f64 = (s.dn[185][25] * ddt_scale);
        let eq124_e1511_d_n26: f64 = (s.dn[185][26] * ddt_scale);
        let eq124_e1511_d_n27: f64 = (s.dn[185][27] * ddt_scale);
        let eq124_e1511_d_n28: f64 = (s.dn[185][28] * ddt_scale);
        let eq124_e1511_d_n29: f64 = (s.dn[185][29] * ddt_scale);
        let eq124_e1514: f64 = (p.p355 * (nv7 - nv13));
        let eq124_e1514_d_n7: f64 = p.p355;
        let eq124_e1514_d_n13: f64 = (-p.p355);
        let eq124_e1515: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 117, eq124_e1514);
        let eq124_e1515_d_n7: f64 = (eq124_e1514_d_n7 * ddt_scale);
        let eq124_e1515_d_n13: f64 = (eq124_e1514_d_n13 * ddt_scale);
        let eq124_e1516: f64 = (eq124_e1511 + eq124_e1515);
        let eq124_e1516_d_n7: f64 = (eq124_e1511_d_n7 + eq124_e1515_d_n7);
        let eq124_e1516_d_n13: f64 = (eq124_e1511_d_n13 + eq124_e1515_d_n13);
        (eq124_e1516, eq124_e1511_d_n0, eq124_e1511_d_n1, eq124_e1511_d_n2, eq124_e1511_d_n3, eq124_e1511_d_n4, eq124_e1511_d_n5, eq124_e1511_d_n6, eq124_e1516_d_n7, eq124_e1511_d_n8, eq124_e1511_d_n9, eq124_e1511_d_n10, eq124_e1511_d_n11, eq124_e1511_d_n12, eq124_e1516_d_n13, eq124_e1511_d_n14, eq124_e1511_d_n15, eq124_e1511_d_n16, eq124_e1511_d_n17, eq124_e1511_d_n18, eq124_e1511_d_n19, eq124_e1511_d_n20, eq124_e1511_d_n21, eq124_e1511_d_n22, eq124_e1511_d_n23, eq124_e1511_d_n24, eq124_e1511_d_n25, eq124_e1511_d_n26, eq124_e1511_d_n27, eq124_e1511_d_n28, eq124_e1511_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_value: f64 = eq124_e1518;
        let eq124_node_derivatives: [f64; 30] = [eq124_e1518_d_n0, eq124_e1518_d_n1, eq124_e1518_d_n2, eq124_e1518_d_n3, eq124_e1518_d_n4, eq124_e1518_d_n5, eq124_e1518_d_n6, eq124_e1518_d_n7, eq124_e1518_d_n8, eq124_e1518_d_n9, eq124_e1518_d_n10, eq124_e1518_d_n11, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_d_n14, eq124_e1518_d_n15, eq124_e1518_d_n16, eq124_e1518_d_n17, eq124_e1518_d_n18, eq124_e1518_d_n19, eq124_e1518_d_n20, eq124_e1518_d_n21, eq124_e1518_d_n22, eq124_e1518_d_n23, eq124_e1518_d_n24, eq124_e1518_d_n25, eq124_e1518_d_n26, eq124_e1518_d_n27, eq124_e1518_d_n28, eq124_e1518_d_n29];
        let eq124_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[13]),
            multiplicity * (eq124_value),
            nodes,
            &eq124_node_derivatives,
            branches,
            &eq124_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1528, eq125_e1528_d_n0, eq125_e1528_d_n1, eq125_e1528_d_n2, eq125_e1528_d_n3, eq125_e1528_d_n4, eq125_e1528_d_n5, eq125_e1528_d_n6, eq125_e1528_d_n7, eq125_e1528_d_n8, eq125_e1528_d_n9, eq125_e1528_d_n10, eq125_e1528_d_n11, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_d_n14, eq125_e1528_d_n15, eq125_e1528_d_n16, eq125_e1528_d_n17, eq125_e1528_d_n18, eq125_e1528_d_n19, eq125_e1528_d_n20, eq125_e1528_d_n21, eq125_e1528_d_n22, eq125_e1528_d_n23, eq125_e1528_d_n24, eq125_e1528_d_n25, eq125_e1528_d_n26, eq125_e1528_d_n27, eq125_e1528_d_n28, eq125_e1528_d_n29,) = {
    if s.b[1495] {
        let eq125_e1521: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 118, s.v[186]);
        let eq125_e1521_d_n0: f64 = (s.dn[186][0] * ddt_scale);
        let eq125_e1521_d_n1: f64 = (s.dn[186][1] * ddt_scale);
        let eq125_e1521_d_n2: f64 = (s.dn[186][2] * ddt_scale);
        let eq125_e1521_d_n3: f64 = (s.dn[186][3] * ddt_scale);
        let eq125_e1521_d_n4: f64 = (s.dn[186][4] * ddt_scale);
        let eq125_e1521_d_n5: f64 = (s.dn[186][5] * ddt_scale);
        let eq125_e1521_d_n6: f64 = (s.dn[186][6] * ddt_scale);
        let eq125_e1521_d_n7: f64 = (s.dn[186][7] * ddt_scale);
        let eq125_e1521_d_n8: f64 = (s.dn[186][8] * ddt_scale);
        let eq125_e1521_d_n9: f64 = (s.dn[186][9] * ddt_scale);
        let eq125_e1521_d_n10: f64 = (s.dn[186][10] * ddt_scale);
        let eq125_e1521_d_n11: f64 = (s.dn[186][11] * ddt_scale);
        let eq125_e1521_d_n12: f64 = (s.dn[186][12] * ddt_scale);
        let eq125_e1521_d_n13: f64 = (s.dn[186][13] * ddt_scale);
        let eq125_e1521_d_n14: f64 = (s.dn[186][14] * ddt_scale);
        let eq125_e1521_d_n15: f64 = (s.dn[186][15] * ddt_scale);
        let eq125_e1521_d_n16: f64 = (s.dn[186][16] * ddt_scale);
        let eq125_e1521_d_n17: f64 = (s.dn[186][17] * ddt_scale);
        let eq125_e1521_d_n18: f64 = (s.dn[186][18] * ddt_scale);
        let eq125_e1521_d_n19: f64 = (s.dn[186][19] * ddt_scale);
        let eq125_e1521_d_n20: f64 = (s.dn[186][20] * ddt_scale);
        let eq125_e1521_d_n21: f64 = (s.dn[186][21] * ddt_scale);
        let eq125_e1521_d_n22: f64 = (s.dn[186][22] * ddt_scale);
        let eq125_e1521_d_n23: f64 = (s.dn[186][23] * ddt_scale);
        let eq125_e1521_d_n24: f64 = (s.dn[186][24] * ddt_scale);
        let eq125_e1521_d_n25: f64 = (s.dn[186][25] * ddt_scale);
        let eq125_e1521_d_n26: f64 = (s.dn[186][26] * ddt_scale);
        let eq125_e1521_d_n27: f64 = (s.dn[186][27] * ddt_scale);
        let eq125_e1521_d_n28: f64 = (s.dn[186][28] * ddt_scale);
        let eq125_e1521_d_n29: f64 = (s.dn[186][29] * ddt_scale);
        let eq125_e1524: f64 = (p.p355 * (nv7 - nv12));
        let eq125_e1524_d_n7: f64 = p.p355;
        let eq125_e1524_d_n12: f64 = (-p.p355);
        let eq125_e1525: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 119, eq125_e1524);
        let eq125_e1525_d_n7: f64 = (eq125_e1524_d_n7 * ddt_scale);
        let eq125_e1525_d_n12: f64 = (eq125_e1524_d_n12 * ddt_scale);
        let eq125_e1526: f64 = (eq125_e1521 + eq125_e1525);
        let eq125_e1526_d_n7: f64 = (eq125_e1521_d_n7 + eq125_e1525_d_n7);
        let eq125_e1526_d_n12: f64 = (eq125_e1521_d_n12 + eq125_e1525_d_n12);
        (eq125_e1526, eq125_e1521_d_n0, eq125_e1521_d_n1, eq125_e1521_d_n2, eq125_e1521_d_n3, eq125_e1521_d_n4, eq125_e1521_d_n5, eq125_e1521_d_n6, eq125_e1526_d_n7, eq125_e1521_d_n8, eq125_e1521_d_n9, eq125_e1521_d_n10, eq125_e1521_d_n11, eq125_e1526_d_n12, eq125_e1521_d_n13, eq125_e1521_d_n14, eq125_e1521_d_n15, eq125_e1521_d_n16, eq125_e1521_d_n17, eq125_e1521_d_n18, eq125_e1521_d_n19, eq125_e1521_d_n20, eq125_e1521_d_n21, eq125_e1521_d_n22, eq125_e1521_d_n23, eq125_e1521_d_n24, eq125_e1521_d_n25, eq125_e1521_d_n26, eq125_e1521_d_n27, eq125_e1521_d_n28, eq125_e1521_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_value: f64 = eq125_e1528;
        let eq125_node_derivatives: [f64; 30] = [eq125_e1528_d_n0, eq125_e1528_d_n1, eq125_e1528_d_n2, eq125_e1528_d_n3, eq125_e1528_d_n4, eq125_e1528_d_n5, eq125_e1528_d_n6, eq125_e1528_d_n7, eq125_e1528_d_n8, eq125_e1528_d_n9, eq125_e1528_d_n10, eq125_e1528_d_n11, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_d_n14, eq125_e1528_d_n15, eq125_e1528_d_n16, eq125_e1528_d_n17, eq125_e1528_d_n18, eq125_e1528_d_n19, eq125_e1528_d_n20, eq125_e1528_d_n21, eq125_e1528_d_n22, eq125_e1528_d_n23, eq125_e1528_d_n24, eq125_e1528_d_n25, eq125_e1528_d_n26, eq125_e1528_d_n27, eq125_e1528_d_n28, eq125_e1528_d_n29];
        let eq125_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            multiplicity * (eq125_value),
            nodes,
            &eq125_node_derivatives,
            branches,
            &eq125_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1538, eq126_e1538_d_n0, eq126_e1538_d_n1, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n5, eq126_e1538_d_n6, eq126_e1538_d_n7, eq126_e1538_d_n8, eq126_e1538_d_n9, eq126_e1538_d_n10, eq126_e1538_d_n11, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_d_n14, eq126_e1538_d_n15, eq126_e1538_d_n16, eq126_e1538_d_n17, eq126_e1538_d_n18, eq126_e1538_d_n19, eq126_e1538_d_n20, eq126_e1538_d_n21, eq126_e1538_d_n22, eq126_e1538_d_n23, eq126_e1538_d_n24, eq126_e1538_d_n25, eq126_e1538_d_n26, eq126_e1538_d_n27, eq126_e1538_d_n28, eq126_e1538_d_n29,) = {
    if s.b[1495] {
        let eq126_e1531: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 120, s.v[187]);
        let eq126_e1531_d_n0: f64 = (s.dn[187][0] * ddt_scale);
        let eq126_e1531_d_n1: f64 = (s.dn[187][1] * ddt_scale);
        let eq126_e1531_d_n2: f64 = (s.dn[187][2] * ddt_scale);
        let eq126_e1531_d_n3: f64 = (s.dn[187][3] * ddt_scale);
        let eq126_e1531_d_n4: f64 = (s.dn[187][4] * ddt_scale);
        let eq126_e1531_d_n5: f64 = (s.dn[187][5] * ddt_scale);
        let eq126_e1531_d_n6: f64 = (s.dn[187][6] * ddt_scale);
        let eq126_e1531_d_n7: f64 = (s.dn[187][7] * ddt_scale);
        let eq126_e1531_d_n8: f64 = (s.dn[187][8] * ddt_scale);
        let eq126_e1531_d_n9: f64 = (s.dn[187][9] * ddt_scale);
        let eq126_e1531_d_n10: f64 = (s.dn[187][10] * ddt_scale);
        let eq126_e1531_d_n11: f64 = (s.dn[187][11] * ddt_scale);
        let eq126_e1531_d_n12: f64 = (s.dn[187][12] * ddt_scale);
        let eq126_e1531_d_n13: f64 = (s.dn[187][13] * ddt_scale);
        let eq126_e1531_d_n14: f64 = (s.dn[187][14] * ddt_scale);
        let eq126_e1531_d_n15: f64 = (s.dn[187][15] * ddt_scale);
        let eq126_e1531_d_n16: f64 = (s.dn[187][16] * ddt_scale);
        let eq126_e1531_d_n17: f64 = (s.dn[187][17] * ddt_scale);
        let eq126_e1531_d_n18: f64 = (s.dn[187][18] * ddt_scale);
        let eq126_e1531_d_n19: f64 = (s.dn[187][19] * ddt_scale);
        let eq126_e1531_d_n20: f64 = (s.dn[187][20] * ddt_scale);
        let eq126_e1531_d_n21: f64 = (s.dn[187][21] * ddt_scale);
        let eq126_e1531_d_n22: f64 = (s.dn[187][22] * ddt_scale);
        let eq126_e1531_d_n23: f64 = (s.dn[187][23] * ddt_scale);
        let eq126_e1531_d_n24: f64 = (s.dn[187][24] * ddt_scale);
        let eq126_e1531_d_n25: f64 = (s.dn[187][25] * ddt_scale);
        let eq126_e1531_d_n26: f64 = (s.dn[187][26] * ddt_scale);
        let eq126_e1531_d_n27: f64 = (s.dn[187][27] * ddt_scale);
        let eq126_e1531_d_n28: f64 = (s.dn[187][28] * ddt_scale);
        let eq126_e1531_d_n29: f64 = (s.dn[187][29] * ddt_scale);
        let eq126_e1534: f64 = (p.p355 * (nv2 - nv13));
        let eq126_e1534_d_n2: f64 = p.p355;
        let eq126_e1534_d_n13: f64 = (-p.p355);
        let eq126_e1535: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 121, eq126_e1534);
        let eq126_e1535_d_n2: f64 = (eq126_e1534_d_n2 * ddt_scale);
        let eq126_e1535_d_n13: f64 = (eq126_e1534_d_n13 * ddt_scale);
        let eq126_e1536: f64 = (eq126_e1531 + eq126_e1535);
        let eq126_e1536_d_n2: f64 = (eq126_e1531_d_n2 + eq126_e1535_d_n2);
        let eq126_e1536_d_n13: f64 = (eq126_e1531_d_n13 + eq126_e1535_d_n13);
        (eq126_e1536, eq126_e1531_d_n0, eq126_e1531_d_n1, eq126_e1536_d_n2, eq126_e1531_d_n3, eq126_e1531_d_n4, eq126_e1531_d_n5, eq126_e1531_d_n6, eq126_e1531_d_n7, eq126_e1531_d_n8, eq126_e1531_d_n9, eq126_e1531_d_n10, eq126_e1531_d_n11, eq126_e1531_d_n12, eq126_e1536_d_n13, eq126_e1531_d_n14, eq126_e1531_d_n15, eq126_e1531_d_n16, eq126_e1531_d_n17, eq126_e1531_d_n18, eq126_e1531_d_n19, eq126_e1531_d_n20, eq126_e1531_d_n21, eq126_e1531_d_n22, eq126_e1531_d_n23, eq126_e1531_d_n24, eq126_e1531_d_n25, eq126_e1531_d_n26, eq126_e1531_d_n27, eq126_e1531_d_n28, eq126_e1531_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_value: f64 = eq126_e1538;
        let eq126_node_derivatives: [f64; 30] = [eq126_e1538_d_n0, eq126_e1538_d_n1, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n5, eq126_e1538_d_n6, eq126_e1538_d_n7, eq126_e1538_d_n8, eq126_e1538_d_n9, eq126_e1538_d_n10, eq126_e1538_d_n11, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_d_n14, eq126_e1538_d_n15, eq126_e1538_d_n16, eq126_e1538_d_n17, eq126_e1538_d_n18, eq126_e1538_d_n19, eq126_e1538_d_n20, eq126_e1538_d_n21, eq126_e1538_d_n22, eq126_e1538_d_n23, eq126_e1538_d_n24, eq126_e1538_d_n25, eq126_e1538_d_n26, eq126_e1538_d_n27, eq126_e1538_d_n28, eq126_e1538_d_n29];
        let eq126_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[13]),
            multiplicity * (eq126_value),
            nodes,
            &eq126_node_derivatives,
            branches,
            &eq126_branch_derivatives,
            multiplicity,
        );
        let (eq127_e1542,) = {
    if s.b[1495] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq127_value: f64 = eq127_e1542;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[12]),
            multiplicity * (eq127_value),
        );
    }

    pub(super) fn stamp_transient_equations_block_11(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let nv28 = ctx.node_voltage(nodes[28]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let (eq128_e1552, eq128_e1552_d_n0, eq128_e1552_d_n1, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n5, eq128_e1552_d_n6, eq128_e1552_d_n7, eq128_e1552_d_n8, eq128_e1552_d_n9, eq128_e1552_d_n10, eq128_e1552_d_n11, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_d_n14, eq128_e1552_d_n15, eq128_e1552_d_n16, eq128_e1552_d_n17, eq128_e1552_d_n18, eq128_e1552_d_n19, eq128_e1552_d_n20, eq128_e1552_d_n21, eq128_e1552_d_n22, eq128_e1552_d_n23, eq128_e1552_d_n24, eq128_e1552_d_n25, eq128_e1552_d_n26, eq128_e1552_d_n27, eq128_e1552_d_n28, eq128_e1552_d_n29,) = {
    if s.b[1495] {
        let eq128_e1545: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 122, s.v[189]);
        let eq128_e1545_d_n0: f64 = (s.dn[189][0] * ddt_scale);
        let eq128_e1545_d_n1: f64 = (s.dn[189][1] * ddt_scale);
        let eq128_e1545_d_n2: f64 = (s.dn[189][2] * ddt_scale);
        let eq128_e1545_d_n3: f64 = (s.dn[189][3] * ddt_scale);
        let eq128_e1545_d_n4: f64 = (s.dn[189][4] * ddt_scale);
        let eq128_e1545_d_n5: f64 = (s.dn[189][5] * ddt_scale);
        let eq128_e1545_d_n6: f64 = (s.dn[189][6] * ddt_scale);
        let eq128_e1545_d_n7: f64 = (s.dn[189][7] * ddt_scale);
        let eq128_e1545_d_n8: f64 = (s.dn[189][8] * ddt_scale);
        let eq128_e1545_d_n9: f64 = (s.dn[189][9] * ddt_scale);
        let eq128_e1545_d_n10: f64 = (s.dn[189][10] * ddt_scale);
        let eq128_e1545_d_n11: f64 = (s.dn[189][11] * ddt_scale);
        let eq128_e1545_d_n12: f64 = (s.dn[189][12] * ddt_scale);
        let eq128_e1545_d_n13: f64 = (s.dn[189][13] * ddt_scale);
        let eq128_e1545_d_n14: f64 = (s.dn[189][14] * ddt_scale);
        let eq128_e1545_d_n15: f64 = (s.dn[189][15] * ddt_scale);
        let eq128_e1545_d_n16: f64 = (s.dn[189][16] * ddt_scale);
        let eq128_e1545_d_n17: f64 = (s.dn[189][17] * ddt_scale);
        let eq128_e1545_d_n18: f64 = (s.dn[189][18] * ddt_scale);
        let eq128_e1545_d_n19: f64 = (s.dn[189][19] * ddt_scale);
        let eq128_e1545_d_n20: f64 = (s.dn[189][20] * ddt_scale);
        let eq128_e1545_d_n21: f64 = (s.dn[189][21] * ddt_scale);
        let eq128_e1545_d_n22: f64 = (s.dn[189][22] * ddt_scale);
        let eq128_e1545_d_n23: f64 = (s.dn[189][23] * ddt_scale);
        let eq128_e1545_d_n24: f64 = (s.dn[189][24] * ddt_scale);
        let eq128_e1545_d_n25: f64 = (s.dn[189][25] * ddt_scale);
        let eq128_e1545_d_n26: f64 = (s.dn[189][26] * ddt_scale);
        let eq128_e1545_d_n27: f64 = (s.dn[189][27] * ddt_scale);
        let eq128_e1545_d_n28: f64 = (s.dn[189][28] * ddt_scale);
        let eq128_e1545_d_n29: f64 = (s.dn[189][29] * ddt_scale);
        let eq128_e1548: f64 = (p.p355 * (nv7 - nv9));
        let eq128_e1548_d_n7: f64 = p.p355;
        let eq128_e1548_d_n9: f64 = (-p.p355);
        let eq128_e1549: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 123, eq128_e1548);
        let eq128_e1549_d_n7: f64 = (eq128_e1548_d_n7 * ddt_scale);
        let eq128_e1549_d_n9: f64 = (eq128_e1548_d_n9 * ddt_scale);
        let eq128_e1550: f64 = (eq128_e1545 + eq128_e1549);
        let eq128_e1550_d_n7: f64 = (eq128_e1545_d_n7 + eq128_e1549_d_n7);
        let eq128_e1550_d_n9: f64 = (eq128_e1545_d_n9 + eq128_e1549_d_n9);
        (eq128_e1550, eq128_e1545_d_n0, eq128_e1545_d_n1, eq128_e1545_d_n2, eq128_e1545_d_n3, eq128_e1545_d_n4, eq128_e1545_d_n5, eq128_e1545_d_n6, eq128_e1550_d_n7, eq128_e1545_d_n8, eq128_e1550_d_n9, eq128_e1545_d_n10, eq128_e1545_d_n11, eq128_e1545_d_n12, eq128_e1545_d_n13, eq128_e1545_d_n14, eq128_e1545_d_n15, eq128_e1545_d_n16, eq128_e1545_d_n17, eq128_e1545_d_n18, eq128_e1545_d_n19, eq128_e1545_d_n20, eq128_e1545_d_n21, eq128_e1545_d_n22, eq128_e1545_d_n23, eq128_e1545_d_n24, eq128_e1545_d_n25, eq128_e1545_d_n26, eq128_e1545_d_n27, eq128_e1545_d_n28, eq128_e1545_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_value: f64 = eq128_e1552;
        let eq128_node_derivatives: [f64; 30] = [eq128_e1552_d_n0, eq128_e1552_d_n1, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n5, eq128_e1552_d_n6, eq128_e1552_d_n7, eq128_e1552_d_n8, eq128_e1552_d_n9, eq128_e1552_d_n10, eq128_e1552_d_n11, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_d_n14, eq128_e1552_d_n15, eq128_e1552_d_n16, eq128_e1552_d_n17, eq128_e1552_d_n18, eq128_e1552_d_n19, eq128_e1552_d_n20, eq128_e1552_d_n21, eq128_e1552_d_n22, eq128_e1552_d_n23, eq128_e1552_d_n24, eq128_e1552_d_n25, eq128_e1552_d_n26, eq128_e1552_d_n27, eq128_e1552_d_n28, eq128_e1552_d_n29];
        let eq128_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq128_value),
            nodes,
            &eq128_node_derivatives,
            branches,
            &eq128_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1563, eq129_e1563_d_n0, eq129_e1563_d_n1, eq129_e1563_d_n2, eq129_e1563_d_n3, eq129_e1563_d_n4, eq129_e1563_d_n5, eq129_e1563_d_n6, eq129_e1563_d_n7, eq129_e1563_d_n8, eq129_e1563_d_n9, eq129_e1563_d_n10, eq129_e1563_d_n11, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_d_n14, eq129_e1563_d_n15, eq129_e1563_d_n16, eq129_e1563_d_n17, eq129_e1563_d_n18, eq129_e1563_d_n19, eq129_e1563_d_n20, eq129_e1563_d_n21, eq129_e1563_d_n22, eq129_e1563_d_n23, eq129_e1563_d_n24, eq129_e1563_d_n25, eq129_e1563_d_n26, eq129_e1563_d_n27, eq129_e1563_d_n28, eq129_e1563_d_n29,) = {
    if (!s.b[1495]) {
        let eq129_e1556: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 124, s.v[185]);
        let eq129_e1556_d_n0: f64 = (s.dn[185][0] * ddt_scale);
        let eq129_e1556_d_n1: f64 = (s.dn[185][1] * ddt_scale);
        let eq129_e1556_d_n2: f64 = (s.dn[185][2] * ddt_scale);
        let eq129_e1556_d_n3: f64 = (s.dn[185][3] * ddt_scale);
        let eq129_e1556_d_n4: f64 = (s.dn[185][4] * ddt_scale);
        let eq129_e1556_d_n5: f64 = (s.dn[185][5] * ddt_scale);
        let eq129_e1556_d_n6: f64 = (s.dn[185][6] * ddt_scale);
        let eq129_e1556_d_n7: f64 = (s.dn[185][7] * ddt_scale);
        let eq129_e1556_d_n8: f64 = (s.dn[185][8] * ddt_scale);
        let eq129_e1556_d_n9: f64 = (s.dn[185][9] * ddt_scale);
        let eq129_e1556_d_n10: f64 = (s.dn[185][10] * ddt_scale);
        let eq129_e1556_d_n11: f64 = (s.dn[185][11] * ddt_scale);
        let eq129_e1556_d_n12: f64 = (s.dn[185][12] * ddt_scale);
        let eq129_e1556_d_n13: f64 = (s.dn[185][13] * ddt_scale);
        let eq129_e1556_d_n14: f64 = (s.dn[185][14] * ddt_scale);
        let eq129_e1556_d_n15: f64 = (s.dn[185][15] * ddt_scale);
        let eq129_e1556_d_n16: f64 = (s.dn[185][16] * ddt_scale);
        let eq129_e1556_d_n17: f64 = (s.dn[185][17] * ddt_scale);
        let eq129_e1556_d_n18: f64 = (s.dn[185][18] * ddt_scale);
        let eq129_e1556_d_n19: f64 = (s.dn[185][19] * ddt_scale);
        let eq129_e1556_d_n20: f64 = (s.dn[185][20] * ddt_scale);
        let eq129_e1556_d_n21: f64 = (s.dn[185][21] * ddt_scale);
        let eq129_e1556_d_n22: f64 = (s.dn[185][22] * ddt_scale);
        let eq129_e1556_d_n23: f64 = (s.dn[185][23] * ddt_scale);
        let eq129_e1556_d_n24: f64 = (s.dn[185][24] * ddt_scale);
        let eq129_e1556_d_n25: f64 = (s.dn[185][25] * ddt_scale);
        let eq129_e1556_d_n26: f64 = (s.dn[185][26] * ddt_scale);
        let eq129_e1556_d_n27: f64 = (s.dn[185][27] * ddt_scale);
        let eq129_e1556_d_n28: f64 = (s.dn[185][28] * ddt_scale);
        let eq129_e1556_d_n29: f64 = (s.dn[185][29] * ddt_scale);
        let eq129_e1559: f64 = (p.p355 * (nv2 - nv13));
        let eq129_e1559_d_n2: f64 = p.p355;
        let eq129_e1559_d_n13: f64 = (-p.p355);
        let eq129_e1560: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 125, eq129_e1559);
        let eq129_e1560_d_n2: f64 = (eq129_e1559_d_n2 * ddt_scale);
        let eq129_e1560_d_n13: f64 = (eq129_e1559_d_n13 * ddt_scale);
        let eq129_e1561: f64 = (eq129_e1556 + eq129_e1560);
        let eq129_e1561_d_n2: f64 = (eq129_e1556_d_n2 + eq129_e1560_d_n2);
        let eq129_e1561_d_n13: f64 = (eq129_e1556_d_n13 + eq129_e1560_d_n13);
        (eq129_e1561, eq129_e1556_d_n0, eq129_e1556_d_n1, eq129_e1561_d_n2, eq129_e1556_d_n3, eq129_e1556_d_n4, eq129_e1556_d_n5, eq129_e1556_d_n6, eq129_e1556_d_n7, eq129_e1556_d_n8, eq129_e1556_d_n9, eq129_e1556_d_n10, eq129_e1556_d_n11, eq129_e1556_d_n12, eq129_e1561_d_n13, eq129_e1556_d_n14, eq129_e1556_d_n15, eq129_e1556_d_n16, eq129_e1556_d_n17, eq129_e1556_d_n18, eq129_e1556_d_n19, eq129_e1556_d_n20, eq129_e1556_d_n21, eq129_e1556_d_n22, eq129_e1556_d_n23, eq129_e1556_d_n24, eq129_e1556_d_n25, eq129_e1556_d_n26, eq129_e1556_d_n27, eq129_e1556_d_n28, eq129_e1556_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_value: f64 = eq129_e1563;
        let eq129_node_derivatives: [f64; 30] = [eq129_e1563_d_n0, eq129_e1563_d_n1, eq129_e1563_d_n2, eq129_e1563_d_n3, eq129_e1563_d_n4, eq129_e1563_d_n5, eq129_e1563_d_n6, eq129_e1563_d_n7, eq129_e1563_d_n8, eq129_e1563_d_n9, eq129_e1563_d_n10, eq129_e1563_d_n11, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_d_n14, eq129_e1563_d_n15, eq129_e1563_d_n16, eq129_e1563_d_n17, eq129_e1563_d_n18, eq129_e1563_d_n19, eq129_e1563_d_n20, eq129_e1563_d_n21, eq129_e1563_d_n22, eq129_e1563_d_n23, eq129_e1563_d_n24, eq129_e1563_d_n25, eq129_e1563_d_n26, eq129_e1563_d_n27, eq129_e1563_d_n28, eq129_e1563_d_n29];
        let eq129_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[13]),
            multiplicity * (eq129_value),
            nodes,
            &eq129_node_derivatives,
            branches,
            &eq129_branch_derivatives,
            multiplicity,
        );
        let (eq130_e1574, eq130_e1574_d_n0, eq130_e1574_d_n1, eq130_e1574_d_n2, eq130_e1574_d_n3, eq130_e1574_d_n4, eq130_e1574_d_n5, eq130_e1574_d_n6, eq130_e1574_d_n7, eq130_e1574_d_n8, eq130_e1574_d_n9, eq130_e1574_d_n10, eq130_e1574_d_n11, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_d_n14, eq130_e1574_d_n15, eq130_e1574_d_n16, eq130_e1574_d_n17, eq130_e1574_d_n18, eq130_e1574_d_n19, eq130_e1574_d_n20, eq130_e1574_d_n21, eq130_e1574_d_n22, eq130_e1574_d_n23, eq130_e1574_d_n24, eq130_e1574_d_n25, eq130_e1574_d_n26, eq130_e1574_d_n27, eq130_e1574_d_n28, eq130_e1574_d_n29,) = {
    if (!s.b[1495]) {
        let eq130_e1567: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 126, s.v[186]);
        let eq130_e1567_d_n0: f64 = (s.dn[186][0] * ddt_scale);
        let eq130_e1567_d_n1: f64 = (s.dn[186][1] * ddt_scale);
        let eq130_e1567_d_n2: f64 = (s.dn[186][2] * ddt_scale);
        let eq130_e1567_d_n3: f64 = (s.dn[186][3] * ddt_scale);
        let eq130_e1567_d_n4: f64 = (s.dn[186][4] * ddt_scale);
        let eq130_e1567_d_n5: f64 = (s.dn[186][5] * ddt_scale);
        let eq130_e1567_d_n6: f64 = (s.dn[186][6] * ddt_scale);
        let eq130_e1567_d_n7: f64 = (s.dn[186][7] * ddt_scale);
        let eq130_e1567_d_n8: f64 = (s.dn[186][8] * ddt_scale);
        let eq130_e1567_d_n9: f64 = (s.dn[186][9] * ddt_scale);
        let eq130_e1567_d_n10: f64 = (s.dn[186][10] * ddt_scale);
        let eq130_e1567_d_n11: f64 = (s.dn[186][11] * ddt_scale);
        let eq130_e1567_d_n12: f64 = (s.dn[186][12] * ddt_scale);
        let eq130_e1567_d_n13: f64 = (s.dn[186][13] * ddt_scale);
        let eq130_e1567_d_n14: f64 = (s.dn[186][14] * ddt_scale);
        let eq130_e1567_d_n15: f64 = (s.dn[186][15] * ddt_scale);
        let eq130_e1567_d_n16: f64 = (s.dn[186][16] * ddt_scale);
        let eq130_e1567_d_n17: f64 = (s.dn[186][17] * ddt_scale);
        let eq130_e1567_d_n18: f64 = (s.dn[186][18] * ddt_scale);
        let eq130_e1567_d_n19: f64 = (s.dn[186][19] * ddt_scale);
        let eq130_e1567_d_n20: f64 = (s.dn[186][20] * ddt_scale);
        let eq130_e1567_d_n21: f64 = (s.dn[186][21] * ddt_scale);
        let eq130_e1567_d_n22: f64 = (s.dn[186][22] * ddt_scale);
        let eq130_e1567_d_n23: f64 = (s.dn[186][23] * ddt_scale);
        let eq130_e1567_d_n24: f64 = (s.dn[186][24] * ddt_scale);
        let eq130_e1567_d_n25: f64 = (s.dn[186][25] * ddt_scale);
        let eq130_e1567_d_n26: f64 = (s.dn[186][26] * ddt_scale);
        let eq130_e1567_d_n27: f64 = (s.dn[186][27] * ddt_scale);
        let eq130_e1567_d_n28: f64 = (s.dn[186][28] * ddt_scale);
        let eq130_e1567_d_n29: f64 = (s.dn[186][29] * ddt_scale);
        let eq130_e1570: f64 = (p.p355 * (nv2 - nv12));
        let eq130_e1570_d_n2: f64 = p.p355;
        let eq130_e1570_d_n12: f64 = (-p.p355);
        let eq130_e1571: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 127, eq130_e1570);
        let eq130_e1571_d_n2: f64 = (eq130_e1570_d_n2 * ddt_scale);
        let eq130_e1571_d_n12: f64 = (eq130_e1570_d_n12 * ddt_scale);
        let eq130_e1572: f64 = (eq130_e1567 + eq130_e1571);
        let eq130_e1572_d_n2: f64 = (eq130_e1567_d_n2 + eq130_e1571_d_n2);
        let eq130_e1572_d_n12: f64 = (eq130_e1567_d_n12 + eq130_e1571_d_n12);
        (eq130_e1572, eq130_e1567_d_n0, eq130_e1567_d_n1, eq130_e1572_d_n2, eq130_e1567_d_n3, eq130_e1567_d_n4, eq130_e1567_d_n5, eq130_e1567_d_n6, eq130_e1567_d_n7, eq130_e1567_d_n8, eq130_e1567_d_n9, eq130_e1567_d_n10, eq130_e1567_d_n11, eq130_e1572_d_n12, eq130_e1567_d_n13, eq130_e1567_d_n14, eq130_e1567_d_n15, eq130_e1567_d_n16, eq130_e1567_d_n17, eq130_e1567_d_n18, eq130_e1567_d_n19, eq130_e1567_d_n20, eq130_e1567_d_n21, eq130_e1567_d_n22, eq130_e1567_d_n23, eq130_e1567_d_n24, eq130_e1567_d_n25, eq130_e1567_d_n26, eq130_e1567_d_n27, eq130_e1567_d_n28, eq130_e1567_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_value: f64 = eq130_e1574;
        let eq130_node_derivatives: [f64; 30] = [eq130_e1574_d_n0, eq130_e1574_d_n1, eq130_e1574_d_n2, eq130_e1574_d_n3, eq130_e1574_d_n4, eq130_e1574_d_n5, eq130_e1574_d_n6, eq130_e1574_d_n7, eq130_e1574_d_n8, eq130_e1574_d_n9, eq130_e1574_d_n10, eq130_e1574_d_n11, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_d_n14, eq130_e1574_d_n15, eq130_e1574_d_n16, eq130_e1574_d_n17, eq130_e1574_d_n18, eq130_e1574_d_n19, eq130_e1574_d_n20, eq130_e1574_d_n21, eq130_e1574_d_n22, eq130_e1574_d_n23, eq130_e1574_d_n24, eq130_e1574_d_n25, eq130_e1574_d_n26, eq130_e1574_d_n27, eq130_e1574_d_n28, eq130_e1574_d_n29];
        let eq130_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            multiplicity * (eq130_value),
            nodes,
            &eq130_node_derivatives,
            branches,
            &eq130_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1585, eq131_e1585_d_n0, eq131_e1585_d_n1, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n5, eq131_e1585_d_n6, eq131_e1585_d_n7, eq131_e1585_d_n8, eq131_e1585_d_n9, eq131_e1585_d_n10, eq131_e1585_d_n11, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_d_n14, eq131_e1585_d_n15, eq131_e1585_d_n16, eq131_e1585_d_n17, eq131_e1585_d_n18, eq131_e1585_d_n19, eq131_e1585_d_n20, eq131_e1585_d_n21, eq131_e1585_d_n22, eq131_e1585_d_n23, eq131_e1585_d_n24, eq131_e1585_d_n25, eq131_e1585_d_n26, eq131_e1585_d_n27, eq131_e1585_d_n28, eq131_e1585_d_n29,) = {
    if (!s.b[1495]) {
        let eq131_e1578: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 128, s.v[187]);
        let eq131_e1578_d_n0: f64 = (s.dn[187][0] * ddt_scale);
        let eq131_e1578_d_n1: f64 = (s.dn[187][1] * ddt_scale);
        let eq131_e1578_d_n2: f64 = (s.dn[187][2] * ddt_scale);
        let eq131_e1578_d_n3: f64 = (s.dn[187][3] * ddt_scale);
        let eq131_e1578_d_n4: f64 = (s.dn[187][4] * ddt_scale);
        let eq131_e1578_d_n5: f64 = (s.dn[187][5] * ddt_scale);
        let eq131_e1578_d_n6: f64 = (s.dn[187][6] * ddt_scale);
        let eq131_e1578_d_n7: f64 = (s.dn[187][7] * ddt_scale);
        let eq131_e1578_d_n8: f64 = (s.dn[187][8] * ddt_scale);
        let eq131_e1578_d_n9: f64 = (s.dn[187][9] * ddt_scale);
        let eq131_e1578_d_n10: f64 = (s.dn[187][10] * ddt_scale);
        let eq131_e1578_d_n11: f64 = (s.dn[187][11] * ddt_scale);
        let eq131_e1578_d_n12: f64 = (s.dn[187][12] * ddt_scale);
        let eq131_e1578_d_n13: f64 = (s.dn[187][13] * ddt_scale);
        let eq131_e1578_d_n14: f64 = (s.dn[187][14] * ddt_scale);
        let eq131_e1578_d_n15: f64 = (s.dn[187][15] * ddt_scale);
        let eq131_e1578_d_n16: f64 = (s.dn[187][16] * ddt_scale);
        let eq131_e1578_d_n17: f64 = (s.dn[187][17] * ddt_scale);
        let eq131_e1578_d_n18: f64 = (s.dn[187][18] * ddt_scale);
        let eq131_e1578_d_n19: f64 = (s.dn[187][19] * ddt_scale);
        let eq131_e1578_d_n20: f64 = (s.dn[187][20] * ddt_scale);
        let eq131_e1578_d_n21: f64 = (s.dn[187][21] * ddt_scale);
        let eq131_e1578_d_n22: f64 = (s.dn[187][22] * ddt_scale);
        let eq131_e1578_d_n23: f64 = (s.dn[187][23] * ddt_scale);
        let eq131_e1578_d_n24: f64 = (s.dn[187][24] * ddt_scale);
        let eq131_e1578_d_n25: f64 = (s.dn[187][25] * ddt_scale);
        let eq131_e1578_d_n26: f64 = (s.dn[187][26] * ddt_scale);
        let eq131_e1578_d_n27: f64 = (s.dn[187][27] * ddt_scale);
        let eq131_e1578_d_n28: f64 = (s.dn[187][28] * ddt_scale);
        let eq131_e1578_d_n29: f64 = (s.dn[187][29] * ddt_scale);
        let eq131_e1581: f64 = (p.p355 * (nv7 - nv13));
        let eq131_e1581_d_n7: f64 = p.p355;
        let eq131_e1581_d_n13: f64 = (-p.p355);
        let eq131_e1582: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 129, eq131_e1581);
        let eq131_e1582_d_n7: f64 = (eq131_e1581_d_n7 * ddt_scale);
        let eq131_e1582_d_n13: f64 = (eq131_e1581_d_n13 * ddt_scale);
        let eq131_e1583: f64 = (eq131_e1578 + eq131_e1582);
        let eq131_e1583_d_n7: f64 = (eq131_e1578_d_n7 + eq131_e1582_d_n7);
        let eq131_e1583_d_n13: f64 = (eq131_e1578_d_n13 + eq131_e1582_d_n13);
        (eq131_e1583, eq131_e1578_d_n0, eq131_e1578_d_n1, eq131_e1578_d_n2, eq131_e1578_d_n3, eq131_e1578_d_n4, eq131_e1578_d_n5, eq131_e1578_d_n6, eq131_e1583_d_n7, eq131_e1578_d_n8, eq131_e1578_d_n9, eq131_e1578_d_n10, eq131_e1578_d_n11, eq131_e1578_d_n12, eq131_e1583_d_n13, eq131_e1578_d_n14, eq131_e1578_d_n15, eq131_e1578_d_n16, eq131_e1578_d_n17, eq131_e1578_d_n18, eq131_e1578_d_n19, eq131_e1578_d_n20, eq131_e1578_d_n21, eq131_e1578_d_n22, eq131_e1578_d_n23, eq131_e1578_d_n24, eq131_e1578_d_n25, eq131_e1578_d_n26, eq131_e1578_d_n27, eq131_e1578_d_n28, eq131_e1578_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_value: f64 = eq131_e1585;
        let eq131_node_derivatives: [f64; 30] = [eq131_e1585_d_n0, eq131_e1585_d_n1, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n5, eq131_e1585_d_n6, eq131_e1585_d_n7, eq131_e1585_d_n8, eq131_e1585_d_n9, eq131_e1585_d_n10, eq131_e1585_d_n11, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_d_n14, eq131_e1585_d_n15, eq131_e1585_d_n16, eq131_e1585_d_n17, eq131_e1585_d_n18, eq131_e1585_d_n19, eq131_e1585_d_n20, eq131_e1585_d_n21, eq131_e1585_d_n22, eq131_e1585_d_n23, eq131_e1585_d_n24, eq131_e1585_d_n25, eq131_e1585_d_n26, eq131_e1585_d_n27, eq131_e1585_d_n28, eq131_e1585_d_n29];
        let eq131_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[13]),
            multiplicity * (eq131_value),
            nodes,
            &eq131_node_derivatives,
            branches,
            &eq131_branch_derivatives,
            multiplicity,
        );
        let (eq132_e1590,) = {
    if (!s.b[1495]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq132_value: f64 = eq132_e1590;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[12]),
            multiplicity * (eq132_value),
        );
        let (eq133_e1595,) = {
    if (!s.b[1495]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq133_value: f64 = eq133_e1595;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[9]),
            multiplicity * (eq133_value),
        );
        let eq134_e1597: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 130, s.v[188]);
        let eq134_e1597_d_n0: f64 = (s.dn[188][0] * ddt_scale);
        let eq134_e1597_d_n1: f64 = (s.dn[188][1] * ddt_scale);
        let eq134_e1597_d_n2: f64 = (s.dn[188][2] * ddt_scale);
        let eq134_e1597_d_n3: f64 = (s.dn[188][3] * ddt_scale);
        let eq134_e1597_d_n4: f64 = (s.dn[188][4] * ddt_scale);
        let eq134_e1597_d_n5: f64 = (s.dn[188][5] * ddt_scale);
        let eq134_e1597_d_n6: f64 = (s.dn[188][6] * ddt_scale);
        let eq134_e1597_d_n7: f64 = (s.dn[188][7] * ddt_scale);
        let eq134_e1597_d_n8: f64 = (s.dn[188][8] * ddt_scale);
        let eq134_e1597_d_n9: f64 = (s.dn[188][9] * ddt_scale);
        let eq134_e1597_d_n10: f64 = (s.dn[188][10] * ddt_scale);
        let eq134_e1597_d_n11: f64 = (s.dn[188][11] * ddt_scale);
        let eq134_e1597_d_n12: f64 = (s.dn[188][12] * ddt_scale);
        let eq134_e1597_d_n13: f64 = (s.dn[188][13] * ddt_scale);
        let eq134_e1597_d_n14: f64 = (s.dn[188][14] * ddt_scale);
        let eq134_e1597_d_n15: f64 = (s.dn[188][15] * ddt_scale);
        let eq134_e1597_d_n16: f64 = (s.dn[188][16] * ddt_scale);
        let eq134_e1597_d_n17: f64 = (s.dn[188][17] * ddt_scale);
        let eq134_e1597_d_n18: f64 = (s.dn[188][18] * ddt_scale);
        let eq134_e1597_d_n19: f64 = (s.dn[188][19] * ddt_scale);
        let eq134_e1597_d_n20: f64 = (s.dn[188][20] * ddt_scale);
        let eq134_e1597_d_n21: f64 = (s.dn[188][21] * ddt_scale);
        let eq134_e1597_d_n22: f64 = (s.dn[188][22] * ddt_scale);
        let eq134_e1597_d_n23: f64 = (s.dn[188][23] * ddt_scale);
        let eq134_e1597_d_n24: f64 = (s.dn[188][24] * ddt_scale);
        let eq134_e1597_d_n25: f64 = (s.dn[188][25] * ddt_scale);
        let eq134_e1597_d_n26: f64 = (s.dn[188][26] * ddt_scale);
        let eq134_e1597_d_n27: f64 = (s.dn[188][27] * ddt_scale);
        let eq134_e1597_d_n28: f64 = (s.dn[188][28] * ddt_scale);
        let eq134_e1597_d_n29: f64 = (s.dn[188][29] * ddt_scale);
        let eq134_e1600: f64 = (p.p355 * (nv3 - nv13));
        let eq134_e1600_d_n3: f64 = p.p355;
        let eq134_e1600_d_n13: f64 = (-p.p355);
        let eq134_e1601: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 131, eq134_e1600);
        let eq134_e1601_d_n3: f64 = (eq134_e1600_d_n3 * ddt_scale);
        let eq134_e1601_d_n13: f64 = (eq134_e1600_d_n13 * ddt_scale);
        let eq134_e1602: f64 = (eq134_e1597 + eq134_e1601);
        let eq134_e1602_d_n3: f64 = (eq134_e1597_d_n3 + eq134_e1601_d_n3);
        let eq134_e1602_d_n13: f64 = (eq134_e1597_d_n13 + eq134_e1601_d_n13);
        let eq134_value: f64 = eq134_e1602;
        let eq134_node_derivatives: [f64; 30] = [eq134_e1597_d_n0, eq134_e1597_d_n1, eq134_e1597_d_n2, eq134_e1602_d_n3, eq134_e1597_d_n4, eq134_e1597_d_n5, eq134_e1597_d_n6, eq134_e1597_d_n7, eq134_e1597_d_n8, eq134_e1597_d_n9, eq134_e1597_d_n10, eq134_e1597_d_n11, eq134_e1597_d_n12, eq134_e1602_d_n13, eq134_e1597_d_n14, eq134_e1597_d_n15, eq134_e1597_d_n16, eq134_e1597_d_n17, eq134_e1597_d_n18, eq134_e1597_d_n19, eq134_e1597_d_n20, eq134_e1597_d_n21, eq134_e1597_d_n22, eq134_e1597_d_n23, eq134_e1597_d_n24, eq134_e1597_d_n25, eq134_e1597_d_n26, eq134_e1597_d_n27, eq134_e1597_d_n28, eq134_e1597_d_n29];
        let eq134_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[13]),
            multiplicity * (eq134_value),
            nodes,
            &eq134_node_derivatives,
            branches,
            &eq134_branch_derivatives,
            multiplicity,
        );
        let (eq135_e1610, eq135_e1610_d_n0, eq135_e1610_d_n1, eq135_e1610_d_n2, eq135_e1610_d_n3, eq135_e1610_d_n4, eq135_e1610_d_n5, eq135_e1610_d_n6, eq135_e1610_d_n7, eq135_e1610_d_n8, eq135_e1610_d_n9, eq135_e1610_d_n10, eq135_e1610_d_n11, eq135_e1610_d_n12, eq135_e1610_d_n13, eq135_e1610_d_n14, eq135_e1610_d_n15, eq135_e1610_d_n16, eq135_e1610_d_n17, eq135_e1610_d_n18, eq135_e1610_d_n19, eq135_e1610_d_n20, eq135_e1610_d_n21, eq135_e1610_d_n22, eq135_e1610_d_n23, eq135_e1610_d_n24, eq135_e1610_d_n25, eq135_e1610_d_n26, eq135_e1610_d_n27, eq135_e1610_d_n28, eq135_e1610_d_n29,) = {
    if s.b[1496] {
        let eq135_e1607: f64 = (s.v[0] * (nv13 - nv19));
        let eq135_e1607_d_n13: f64 = s.v[0];
        let eq135_e1607_d_n19: f64 = (-s.v[0]);
        let eq135_e1608: f64 = (s.v[154] + eq135_e1607);
        let eq135_e1608_d_n13: f64 = (s.dn[154][13] + eq135_e1607_d_n13);
        let eq135_e1608_d_n19: f64 = (s.dn[154][19] + eq135_e1607_d_n19);
        (eq135_e1608, s.dn[154][0], s.dn[154][1], s.dn[154][2], s.dn[154][3], s.dn[154][4], s.dn[154][5], s.dn[154][6], s.dn[154][7], s.dn[154][8], s.dn[154][9], s.dn[154][10], s.dn[154][11], s.dn[154][12], eq135_e1608_d_n13, s.dn[154][14], s.dn[154][15], s.dn[154][16], s.dn[154][17], s.dn[154][18], eq135_e1608_d_n19, s.dn[154][20], s.dn[154][21], s.dn[154][22], s.dn[154][23], s.dn[154][24], s.dn[154][25], s.dn[154][26], s.dn[154][27], s.dn[154][28], s.dn[154][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_value: f64 = eq135_e1610;
        let eq135_node_derivatives: [f64; 30] = [eq135_e1610_d_n0, eq135_e1610_d_n1, eq135_e1610_d_n2, eq135_e1610_d_n3, eq135_e1610_d_n4, eq135_e1610_d_n5, eq135_e1610_d_n6, eq135_e1610_d_n7, eq135_e1610_d_n8, eq135_e1610_d_n9, eq135_e1610_d_n10, eq135_e1610_d_n11, eq135_e1610_d_n12, eq135_e1610_d_n13, eq135_e1610_d_n14, eq135_e1610_d_n15, eq135_e1610_d_n16, eq135_e1610_d_n17, eq135_e1610_d_n18, eq135_e1610_d_n19, eq135_e1610_d_n20, eq135_e1610_d_n21, eq135_e1610_d_n22, eq135_e1610_d_n23, eq135_e1610_d_n24, eq135_e1610_d_n25, eq135_e1610_d_n26, eq135_e1610_d_n27, eq135_e1610_d_n28, eq135_e1610_d_n29];
        let eq135_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[19]),
            multiplicity * (eq135_value),
            nodes,
            &eq135_node_derivatives,
            branches,
            &eq135_branch_derivatives,
            multiplicity,
        );
        let (eq136_e1615,) = {
    if (!s.b[1496]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq136_value: f64 = eq136_e1615;
        stamper.stamp_potential_const(
            branches[26],
            eq136_value,
        );
        let (eq137_e1623, eq137_e1623_d_n0, eq137_e1623_d_n1, eq137_e1623_d_n2, eq137_e1623_d_n3, eq137_e1623_d_n4, eq137_e1623_d_n5, eq137_e1623_d_n6, eq137_e1623_d_n7, eq137_e1623_d_n8, eq137_e1623_d_n9, eq137_e1623_d_n10, eq137_e1623_d_n11, eq137_e1623_d_n12, eq137_e1623_d_n13, eq137_e1623_d_n14, eq137_e1623_d_n15, eq137_e1623_d_n16, eq137_e1623_d_n17, eq137_e1623_d_n18, eq137_e1623_d_n19, eq137_e1623_d_n20, eq137_e1623_d_n21, eq137_e1623_d_n22, eq137_e1623_d_n23, eq137_e1623_d_n24, eq137_e1623_d_n25, eq137_e1623_d_n26, eq137_e1623_d_n27, eq137_e1623_d_n28, eq137_e1623_d_n29,) = {
    if s.b[1642] {
        let eq137_e1620: f64 = (s.v[0] * (nv18 - nv17));
        let eq137_e1620_d_n17: f64 = (-s.v[0]);
        let eq137_e1620_d_n18: f64 = s.v[0];
        let eq137_e1621: f64 = (s.v[160] + eq137_e1620);
        let eq137_e1621_d_n17: f64 = (s.dn[160][17] + eq137_e1620_d_n17);
        let eq137_e1621_d_n18: f64 = (s.dn[160][18] + eq137_e1620_d_n18);
        (eq137_e1621, s.dn[160][0], s.dn[160][1], s.dn[160][2], s.dn[160][3], s.dn[160][4], s.dn[160][5], s.dn[160][6], s.dn[160][7], s.dn[160][8], s.dn[160][9], s.dn[160][10], s.dn[160][11], s.dn[160][12], s.dn[160][13], s.dn[160][14], s.dn[160][15], s.dn[160][16], eq137_e1621_d_n17, eq137_e1621_d_n18, s.dn[160][19], s.dn[160][20], s.dn[160][21], s.dn[160][22], s.dn[160][23], s.dn[160][24], s.dn[160][25], s.dn[160][26], s.dn[160][27], s.dn[160][28], s.dn[160][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_value: f64 = eq137_e1623;
        let eq137_node_derivatives: [f64; 30] = [eq137_e1623_d_n0, eq137_e1623_d_n1, eq137_e1623_d_n2, eq137_e1623_d_n3, eq137_e1623_d_n4, eq137_e1623_d_n5, eq137_e1623_d_n6, eq137_e1623_d_n7, eq137_e1623_d_n8, eq137_e1623_d_n9, eq137_e1623_d_n10, eq137_e1623_d_n11, eq137_e1623_d_n12, eq137_e1623_d_n13, eq137_e1623_d_n14, eq137_e1623_d_n15, eq137_e1623_d_n16, eq137_e1623_d_n17, eq137_e1623_d_n18, eq137_e1623_d_n19, eq137_e1623_d_n20, eq137_e1623_d_n21, eq137_e1623_d_n22, eq137_e1623_d_n23, eq137_e1623_d_n24, eq137_e1623_d_n25, eq137_e1623_d_n26, eq137_e1623_d_n27, eq137_e1623_d_n28, eq137_e1623_d_n29];
        let eq137_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[18]),
            Some(nodes[17]),
            multiplicity * (eq137_value),
            nodes,
            &eq137_node_derivatives,
            branches,
            &eq137_branch_derivatives,
            multiplicity,
        );
        let (eq138_e1628,) = {
    if (!s.b[1642]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq138_value: f64 = eq138_e1628;
        stamper.stamp_potential_const(
            branches[27],
            eq138_value,
        );
        let (eq139_e1632,) = {
    if s.b[1933] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq139_value: f64 = eq139_e1632;
        stamper.stamp_potential_const(
            branches[28],
            eq139_value,
        );
        let (eq140_e1636,) = {
    if s.b[1933] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq140_value: f64 = eq140_e1636;
        stamper.stamp_potential_const(
            branches[29],
            eq140_value,
        );
        let (eq141_e1644, eq141_e1644_d_n0, eq141_e1644_d_n1, eq141_e1644_d_n2, eq141_e1644_d_n3, eq141_e1644_d_n4, eq141_e1644_d_n5, eq141_e1644_d_n6, eq141_e1644_d_n7, eq141_e1644_d_n8, eq141_e1644_d_n9, eq141_e1644_d_n10, eq141_e1644_d_n11, eq141_e1644_d_n12, eq141_e1644_d_n13, eq141_e1644_d_n14, eq141_e1644_d_n15, eq141_e1644_d_n16, eq141_e1644_d_n17, eq141_e1644_d_n18, eq141_e1644_d_n19, eq141_e1644_d_n20, eq141_e1644_d_n21, eq141_e1644_d_n22, eq141_e1644_d_n23, eq141_e1644_d_n24, eq141_e1644_d_n25, eq141_e1644_d_n26, eq141_e1644_d_n27, eq141_e1644_d_n28, eq141_e1644_d_n29,) = {
    if s.b[1933] {
        let eq141_e1641: f64 = (s.v[0] * (nv5 - nv9));
        let eq141_e1641_d_n5: f64 = s.v[0];
        let eq141_e1641_d_n9: f64 = (-s.v[0]);
        let eq141_e1642: f64 = (s.v[115] + eq141_e1641);
        let eq141_e1642_d_n5: f64 = (s.dn[115][5] + eq141_e1641_d_n5);
        let eq141_e1642_d_n9: f64 = (s.dn[115][9] + eq141_e1641_d_n9);
        (eq141_e1642, s.dn[115][0], s.dn[115][1], s.dn[115][2], s.dn[115][3], s.dn[115][4], eq141_e1642_d_n5, s.dn[115][6], s.dn[115][7], s.dn[115][8], eq141_e1642_d_n9, s.dn[115][10], s.dn[115][11], s.dn[115][12], s.dn[115][13], s.dn[115][14], s.dn[115][15], s.dn[115][16], s.dn[115][17], s.dn[115][18], s.dn[115][19], s.dn[115][20], s.dn[115][21], s.dn[115][22], s.dn[115][23], s.dn[115][24], s.dn[115][25], s.dn[115][26], s.dn[115][27], s.dn[115][28], s.dn[115][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_value: f64 = eq141_e1644;
        let eq141_node_derivatives: [f64; 30] = [eq141_e1644_d_n0, eq141_e1644_d_n1, eq141_e1644_d_n2, eq141_e1644_d_n3, eq141_e1644_d_n4, eq141_e1644_d_n5, eq141_e1644_d_n6, eq141_e1644_d_n7, eq141_e1644_d_n8, eq141_e1644_d_n9, eq141_e1644_d_n10, eq141_e1644_d_n11, eq141_e1644_d_n12, eq141_e1644_d_n13, eq141_e1644_d_n14, eq141_e1644_d_n15, eq141_e1644_d_n16, eq141_e1644_d_n17, eq141_e1644_d_n18, eq141_e1644_d_n19, eq141_e1644_d_n20, eq141_e1644_d_n21, eq141_e1644_d_n22, eq141_e1644_d_n23, eq141_e1644_d_n24, eq141_e1644_d_n25, eq141_e1644_d_n26, eq141_e1644_d_n27, eq141_e1644_d_n28, eq141_e1644_d_n29];
        let eq141_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[9]),
            multiplicity * (eq141_value),
            nodes,
            &eq141_node_derivatives,
            branches,
            &eq141_branch_derivatives,
            multiplicity,
        );
        let (eq142_e1656, eq142_e1656_d_n0, eq142_e1656_d_n1, eq142_e1656_d_n2, eq142_e1656_d_n3, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n6, eq142_e1656_d_n7, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n10, eq142_e1656_d_n11, eq142_e1656_d_n12, eq142_e1656_d_n13, eq142_e1656_d_n14, eq142_e1656_d_n15, eq142_e1656_d_n16, eq142_e1656_d_n17, eq142_e1656_d_n18, eq142_e1656_d_n19, eq142_e1656_d_n20, eq142_e1656_d_n21, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n24, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n27, eq142_e1656_d_n28, eq142_e1656_d_n29,) = {
    if (!s.b[1933]) {
        let eq142_e1649: f64 = (s.v[115] - (nv29 - 0.0));
        let eq142_e1649_d_n29: f64 = (s.dn[115][29] - 1.0);
        let eq142_e1652: f64 = (p.p323 * (nv28 - 0.0));
        let eq142_e1652_d_n28: f64 = p.p323;
        let eq142_e1653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 132, eq142_e1652);
        let eq142_e1653_d_n28: f64 = (eq142_e1652_d_n28 * ddt_scale);
        let eq142_e1654: f64 = (eq142_e1649 - eq142_e1653);
        let eq142_e1654_d_n28: f64 = (s.dn[115][28] - eq142_e1653_d_n28);
        (eq142_e1654, s.dn[115][0], s.dn[115][1], s.dn[115][2], s.dn[115][3], s.dn[115][4], s.dn[115][5], s.dn[115][6], s.dn[115][7], s.dn[115][8], s.dn[115][9], s.dn[115][10], s.dn[115][11], s.dn[115][12], s.dn[115][13], s.dn[115][14], s.dn[115][15], s.dn[115][16], s.dn[115][17], s.dn[115][18], s.dn[115][19], s.dn[115][20], s.dn[115][21], s.dn[115][22], s.dn[115][23], s.dn[115][24], s.dn[115][25], s.dn[115][26], s.dn[115][27], eq142_e1654_d_n28, eq142_e1649_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_value: f64 = eq142_e1656;
        let eq142_node_derivatives: [f64; 30] = [eq142_e1656_d_n0, eq142_e1656_d_n1, eq142_e1656_d_n2, eq142_e1656_d_n3, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n6, eq142_e1656_d_n7, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n10, eq142_e1656_d_n11, eq142_e1656_d_n12, eq142_e1656_d_n13, eq142_e1656_d_n14, eq142_e1656_d_n15, eq142_e1656_d_n16, eq142_e1656_d_n17, eq142_e1656_d_n18, eq142_e1656_d_n19, eq142_e1656_d_n20, eq142_e1656_d_n21, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n24, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n27, eq142_e1656_d_n28, eq142_e1656_d_n29];
        let eq142_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[28]),
            None,
            multiplicity * (eq142_value),
            nodes,
            &eq142_node_derivatives,
            branches,
            &eq142_branch_derivatives,
            multiplicity,
        );
        let (eq143_e1670, eq143_e1670_d_n28, eq143_e1670_d_n29,) = {
    if (!s.b[1933]) {
        let eq143_e1661: f64 = ((nv28 - 0.0) - (nv29 - 0.0));
        let eq143_e1661_d_n29: f64 = (-1.0);
        let eq143_e1664: f64 = (p.p323 / 3.0);
        let eq143_e1666: f64 = (eq143_e1664 * (nv29 - 0.0));
        let eq143_e1667: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 133, eq143_e1666);
        let eq143_e1667_d_n29: f64 = (eq143_e1664 * ddt_scale);
        let eq143_e1668: f64 = (eq143_e1661 - eq143_e1667);
        let eq143_e1668_d_n29: f64 = (eq143_e1661_d_n29 - eq143_e1667_d_n29);
        (eq143_e1668, 1.0, eq143_e1668_d_n29,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq143_value: f64 = eq143_e1670;
        stamper.stamp_current_node2(
            Some(nodes[29]),
            None,
            multiplicity * (eq143_value),
            nodes[28],
            multiplicity * (eq143_e1670_d_n28),
            nodes[29],
            multiplicity * (eq143_e1670_d_n29),
        );
    }

    pub(super) fn stamp_transient_equations_block_12(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq144_e1679, eq144_e1679_d_n0, eq144_e1679_d_n1, eq144_e1679_d_n2, eq144_e1679_d_n3, eq144_e1679_d_n4, eq144_e1679_d_n5, eq144_e1679_d_n6, eq144_e1679_d_n7, eq144_e1679_d_n8, eq144_e1679_d_n9, eq144_e1679_d_n10, eq144_e1679_d_n11, eq144_e1679_d_n12, eq144_e1679_d_n13, eq144_e1679_d_n14, eq144_e1679_d_n15, eq144_e1679_d_n16, eq144_e1679_d_n17, eq144_e1679_d_n18, eq144_e1679_d_n19, eq144_e1679_d_n20, eq144_e1679_d_n21, eq144_e1679_d_n22, eq144_e1679_d_n23, eq144_e1679_d_n24, eq144_e1679_d_n25, eq144_e1679_d_n26, eq144_e1679_d_n27, eq144_e1679_d_n28, eq144_e1679_d_n29,) = {
    if (!s.b[1933]) {
        let eq144_e1676: f64 = (s.v[0] * (nv5 - nv9));
        let eq144_e1676_d_n5: f64 = s.v[0];
        let eq144_e1676_d_n9: f64 = (-s.v[0]);
        let eq144_e1677: f64 = (s.v[116] + eq144_e1676);
        let eq144_e1677_d_n5: f64 = (s.dn[116][5] + eq144_e1676_d_n5);
        let eq144_e1677_d_n9: f64 = (s.dn[116][9] + eq144_e1676_d_n9);
        (eq144_e1677, s.dn[116][0], s.dn[116][1], s.dn[116][2], s.dn[116][3], s.dn[116][4], eq144_e1677_d_n5, s.dn[116][6], s.dn[116][7], s.dn[116][8], eq144_e1677_d_n9, s.dn[116][10], s.dn[116][11], s.dn[116][12], s.dn[116][13], s.dn[116][14], s.dn[116][15], s.dn[116][16], s.dn[116][17], s.dn[116][18], s.dn[116][19], s.dn[116][20], s.dn[116][21], s.dn[116][22], s.dn[116][23], s.dn[116][24], s.dn[116][25], s.dn[116][26], s.dn[116][27], s.dn[116][28], s.dn[116][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_value: f64 = eq144_e1679;
        let eq144_node_derivatives: [f64; 30] = [eq144_e1679_d_n0, eq144_e1679_d_n1, eq144_e1679_d_n2, eq144_e1679_d_n3, eq144_e1679_d_n4, eq144_e1679_d_n5, eq144_e1679_d_n6, eq144_e1679_d_n7, eq144_e1679_d_n8, eq144_e1679_d_n9, eq144_e1679_d_n10, eq144_e1679_d_n11, eq144_e1679_d_n12, eq144_e1679_d_n13, eq144_e1679_d_n14, eq144_e1679_d_n15, eq144_e1679_d_n16, eq144_e1679_d_n17, eq144_e1679_d_n18, eq144_e1679_d_n19, eq144_e1679_d_n20, eq144_e1679_d_n21, eq144_e1679_d_n22, eq144_e1679_d_n23, eq144_e1679_d_n24, eq144_e1679_d_n25, eq144_e1679_d_n26, eq144_e1679_d_n27, eq144_e1679_d_n28, eq144_e1679_d_n29];
        let eq144_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[9]),
            multiplicity * (eq144_value),
            nodes,
            &eq144_node_derivatives,
            branches,
            &eq144_branch_derivatives,
            multiplicity,
        );
        let eq145_e1681: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 134, s.v[117]);
        let eq145_e1681_d_n0: f64 = (s.dn[117][0] * ddt_scale);
        let eq145_e1681_d_n1: f64 = (s.dn[117][1] * ddt_scale);
        let eq145_e1681_d_n2: f64 = (s.dn[117][2] * ddt_scale);
        let eq145_e1681_d_n3: f64 = (s.dn[117][3] * ddt_scale);
        let eq145_e1681_d_n4: f64 = (s.dn[117][4] * ddt_scale);
        let eq145_e1681_d_n5: f64 = (s.dn[117][5] * ddt_scale);
        let eq145_e1681_d_n6: f64 = (s.dn[117][6] * ddt_scale);
        let eq145_e1681_d_n7: f64 = (s.dn[117][7] * ddt_scale);
        let eq145_e1681_d_n8: f64 = (s.dn[117][8] * ddt_scale);
        let eq145_e1681_d_n9: f64 = (s.dn[117][9] * ddt_scale);
        let eq145_e1681_d_n10: f64 = (s.dn[117][10] * ddt_scale);
        let eq145_e1681_d_n11: f64 = (s.dn[117][11] * ddt_scale);
        let eq145_e1681_d_n12: f64 = (s.dn[117][12] * ddt_scale);
        let eq145_e1681_d_n13: f64 = (s.dn[117][13] * ddt_scale);
        let eq145_e1681_d_n14: f64 = (s.dn[117][14] * ddt_scale);
        let eq145_e1681_d_n15: f64 = (s.dn[117][15] * ddt_scale);
        let eq145_e1681_d_n16: f64 = (s.dn[117][16] * ddt_scale);
        let eq145_e1681_d_n17: f64 = (s.dn[117][17] * ddt_scale);
        let eq145_e1681_d_n18: f64 = (s.dn[117][18] * ddt_scale);
        let eq145_e1681_d_n19: f64 = (s.dn[117][19] * ddt_scale);
        let eq145_e1681_d_n20: f64 = (s.dn[117][20] * ddt_scale);
        let eq145_e1681_d_n21: f64 = (s.dn[117][21] * ddt_scale);
        let eq145_e1681_d_n22: f64 = (s.dn[117][22] * ddt_scale);
        let eq145_e1681_d_n23: f64 = (s.dn[117][23] * ddt_scale);
        let eq145_e1681_d_n24: f64 = (s.dn[117][24] * ddt_scale);
        let eq145_e1681_d_n25: f64 = (s.dn[117][25] * ddt_scale);
        let eq145_e1681_d_n26: f64 = (s.dn[117][26] * ddt_scale);
        let eq145_e1681_d_n27: f64 = (s.dn[117][27] * ddt_scale);
        let eq145_e1681_d_n28: f64 = (s.dn[117][28] * ddt_scale);
        let eq145_e1681_d_n29: f64 = (s.dn[117][29] * ddt_scale);
        let eq145_e1684: f64 = (p.p355 * (nv8 - nv9));
        let eq145_e1684_d_n8: f64 = p.p355;
        let eq145_e1684_d_n9: f64 = (-p.p355);
        let eq145_e1685: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 135, eq145_e1684);
        let eq145_e1685_d_n8: f64 = (eq145_e1684_d_n8 * ddt_scale);
        let eq145_e1685_d_n9: f64 = (eq145_e1684_d_n9 * ddt_scale);
        let eq145_e1686: f64 = (eq145_e1681 + eq145_e1685);
        let eq145_e1686_d_n8: f64 = (eq145_e1681_d_n8 + eq145_e1685_d_n8);
        let eq145_e1686_d_n9: f64 = (eq145_e1681_d_n9 + eq145_e1685_d_n9);
        let eq145_value: f64 = eq145_e1686;
        let eq145_node_derivatives: [f64; 30] = [eq145_e1681_d_n0, eq145_e1681_d_n1, eq145_e1681_d_n2, eq145_e1681_d_n3, eq145_e1681_d_n4, eq145_e1681_d_n5, eq145_e1681_d_n6, eq145_e1681_d_n7, eq145_e1686_d_n8, eq145_e1686_d_n9, eq145_e1681_d_n10, eq145_e1681_d_n11, eq145_e1681_d_n12, eq145_e1681_d_n13, eq145_e1681_d_n14, eq145_e1681_d_n15, eq145_e1681_d_n16, eq145_e1681_d_n17, eq145_e1681_d_n18, eq145_e1681_d_n19, eq145_e1681_d_n20, eq145_e1681_d_n21, eq145_e1681_d_n22, eq145_e1681_d_n23, eq145_e1681_d_n24, eq145_e1681_d_n25, eq145_e1681_d_n26, eq145_e1681_d_n27, eq145_e1681_d_n28, eq145_e1681_d_n29];
        let eq145_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            multiplicity * (eq145_value),
            nodes,
            &eq145_node_derivatives,
            branches,
            &eq145_branch_derivatives,
            multiplicity,
        );
        let eq146_e1688: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 136, s.v[118]);
        let eq146_e1688_d_n0: f64 = (s.dn[118][0] * ddt_scale);
        let eq146_e1688_d_n1: f64 = (s.dn[118][1] * ddt_scale);
        let eq146_e1688_d_n2: f64 = (s.dn[118][2] * ddt_scale);
        let eq146_e1688_d_n3: f64 = (s.dn[118][3] * ddt_scale);
        let eq146_e1688_d_n4: f64 = (s.dn[118][4] * ddt_scale);
        let eq146_e1688_d_n5: f64 = (s.dn[118][5] * ddt_scale);
        let eq146_e1688_d_n6: f64 = (s.dn[118][6] * ddt_scale);
        let eq146_e1688_d_n7: f64 = (s.dn[118][7] * ddt_scale);
        let eq146_e1688_d_n8: f64 = (s.dn[118][8] * ddt_scale);
        let eq146_e1688_d_n9: f64 = (s.dn[118][9] * ddt_scale);
        let eq146_e1688_d_n10: f64 = (s.dn[118][10] * ddt_scale);
        let eq146_e1688_d_n11: f64 = (s.dn[118][11] * ddt_scale);
        let eq146_e1688_d_n12: f64 = (s.dn[118][12] * ddt_scale);
        let eq146_e1688_d_n13: f64 = (s.dn[118][13] * ddt_scale);
        let eq146_e1688_d_n14: f64 = (s.dn[118][14] * ddt_scale);
        let eq146_e1688_d_n15: f64 = (s.dn[118][15] * ddt_scale);
        let eq146_e1688_d_n16: f64 = (s.dn[118][16] * ddt_scale);
        let eq146_e1688_d_n17: f64 = (s.dn[118][17] * ddt_scale);
        let eq146_e1688_d_n18: f64 = (s.dn[118][18] * ddt_scale);
        let eq146_e1688_d_n19: f64 = (s.dn[118][19] * ddt_scale);
        let eq146_e1688_d_n20: f64 = (s.dn[118][20] * ddt_scale);
        let eq146_e1688_d_n21: f64 = (s.dn[118][21] * ddt_scale);
        let eq146_e1688_d_n22: f64 = (s.dn[118][22] * ddt_scale);
        let eq146_e1688_d_n23: f64 = (s.dn[118][23] * ddt_scale);
        let eq146_e1688_d_n24: f64 = (s.dn[118][24] * ddt_scale);
        let eq146_e1688_d_n25: f64 = (s.dn[118][25] * ddt_scale);
        let eq146_e1688_d_n26: f64 = (s.dn[118][26] * ddt_scale);
        let eq146_e1688_d_n27: f64 = (s.dn[118][27] * ddt_scale);
        let eq146_e1688_d_n28: f64 = (s.dn[118][28] * ddt_scale);
        let eq146_e1688_d_n29: f64 = (s.dn[118][29] * ddt_scale);
        let eq146_e1691: f64 = (p.p355 * (nv8 - nv5));
        let eq146_e1691_d_n5: f64 = (-p.p355);
        let eq146_e1691_d_n8: f64 = p.p355;
        let eq146_e1692: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 137, eq146_e1691);
        let eq146_e1692_d_n5: f64 = (eq146_e1691_d_n5 * ddt_scale);
        let eq146_e1692_d_n8: f64 = (eq146_e1691_d_n8 * ddt_scale);
        let eq146_e1693: f64 = (eq146_e1688 + eq146_e1692);
        let eq146_e1693_d_n5: f64 = (eq146_e1688_d_n5 + eq146_e1692_d_n5);
        let eq146_e1693_d_n8: f64 = (eq146_e1688_d_n8 + eq146_e1692_d_n8);
        let eq146_value: f64 = eq146_e1693;
        let eq146_node_derivatives: [f64; 30] = [eq146_e1688_d_n0, eq146_e1688_d_n1, eq146_e1688_d_n2, eq146_e1688_d_n3, eq146_e1688_d_n4, eq146_e1693_d_n5, eq146_e1688_d_n6, eq146_e1688_d_n7, eq146_e1693_d_n8, eq146_e1688_d_n9, eq146_e1688_d_n10, eq146_e1688_d_n11, eq146_e1688_d_n12, eq146_e1688_d_n13, eq146_e1688_d_n14, eq146_e1688_d_n15, eq146_e1688_d_n16, eq146_e1688_d_n17, eq146_e1688_d_n18, eq146_e1688_d_n19, eq146_e1688_d_n20, eq146_e1688_d_n21, eq146_e1688_d_n22, eq146_e1688_d_n23, eq146_e1688_d_n24, eq146_e1688_d_n25, eq146_e1688_d_n26, eq146_e1688_d_n27, eq146_e1688_d_n28, eq146_e1688_d_n29];
        let eq146_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            multiplicity * (eq146_value),
            nodes,
            &eq146_node_derivatives,
            branches,
            &eq146_branch_derivatives,
            multiplicity,
        );
        let (eq147_e1701, eq147_e1701_d_n0, eq147_e1701_d_n1, eq147_e1701_d_n2, eq147_e1701_d_n3, eq147_e1701_d_n4, eq147_e1701_d_n5, eq147_e1701_d_n6, eq147_e1701_d_n7, eq147_e1701_d_n8, eq147_e1701_d_n9, eq147_e1701_d_n10, eq147_e1701_d_n11, eq147_e1701_d_n12, eq147_e1701_d_n13, eq147_e1701_d_n14, eq147_e1701_d_n15, eq147_e1701_d_n16, eq147_e1701_d_n17, eq147_e1701_d_n18, eq147_e1701_d_n19, eq147_e1701_d_n20, eq147_e1701_d_n21, eq147_e1701_d_n22, eq147_e1701_d_n23, eq147_e1701_d_n24, eq147_e1701_d_n25, eq147_e1701_d_n26, eq147_e1701_d_n27, eq147_e1701_d_n28, eq147_e1701_d_n29,) = {
    if s.b[1934] {
        let eq147_e1698: f64 = (s.v[0] * (nv8 - nv13));
        let eq147_e1698_d_n8: f64 = s.v[0];
        let eq147_e1698_d_n13: f64 = (-s.v[0]);
        let eq147_e1699: f64 = (s.v[122] + eq147_e1698);
        let eq147_e1699_d_n8: f64 = (s.dn[122][8] + eq147_e1698_d_n8);
        let eq147_e1699_d_n13: f64 = (s.dn[122][13] + eq147_e1698_d_n13);
        (eq147_e1699, s.dn[122][0], s.dn[122][1], s.dn[122][2], s.dn[122][3], s.dn[122][4], s.dn[122][5], s.dn[122][6], s.dn[122][7], eq147_e1699_d_n8, s.dn[122][9], s.dn[122][10], s.dn[122][11], s.dn[122][12], eq147_e1699_d_n13, s.dn[122][14], s.dn[122][15], s.dn[122][16], s.dn[122][17], s.dn[122][18], s.dn[122][19], s.dn[122][20], s.dn[122][21], s.dn[122][22], s.dn[122][23], s.dn[122][24], s.dn[122][25], s.dn[122][26], s.dn[122][27], s.dn[122][28], s.dn[122][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_value: f64 = eq147_e1701;
        let eq147_node_derivatives: [f64; 30] = [eq147_e1701_d_n0, eq147_e1701_d_n1, eq147_e1701_d_n2, eq147_e1701_d_n3, eq147_e1701_d_n4, eq147_e1701_d_n5, eq147_e1701_d_n6, eq147_e1701_d_n7, eq147_e1701_d_n8, eq147_e1701_d_n9, eq147_e1701_d_n10, eq147_e1701_d_n11, eq147_e1701_d_n12, eq147_e1701_d_n13, eq147_e1701_d_n14, eq147_e1701_d_n15, eq147_e1701_d_n16, eq147_e1701_d_n17, eq147_e1701_d_n18, eq147_e1701_d_n19, eq147_e1701_d_n20, eq147_e1701_d_n21, eq147_e1701_d_n22, eq147_e1701_d_n23, eq147_e1701_d_n24, eq147_e1701_d_n25, eq147_e1701_d_n26, eq147_e1701_d_n27, eq147_e1701_d_n28, eq147_e1701_d_n29];
        let eq147_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[13]),
            multiplicity * (eq147_value),
            nodes,
            &eq147_node_derivatives,
            branches,
            &eq147_branch_derivatives,
            multiplicity,
        );
        let (eq148_e1709, eq148_e1709_d_n0, eq148_e1709_d_n1, eq148_e1709_d_n2, eq148_e1709_d_n3, eq148_e1709_d_n4, eq148_e1709_d_n5, eq148_e1709_d_n6, eq148_e1709_d_n7, eq148_e1709_d_n8, eq148_e1709_d_n9, eq148_e1709_d_n10, eq148_e1709_d_n11, eq148_e1709_d_n12, eq148_e1709_d_n13, eq148_e1709_d_n14, eq148_e1709_d_n15, eq148_e1709_d_n16, eq148_e1709_d_n17, eq148_e1709_d_n18, eq148_e1709_d_n19, eq148_e1709_d_n20, eq148_e1709_d_n21, eq148_e1709_d_n22, eq148_e1709_d_n23, eq148_e1709_d_n24, eq148_e1709_d_n25, eq148_e1709_d_n26, eq148_e1709_d_n27, eq148_e1709_d_n28, eq148_e1709_d_n29,) = {
    if s.b[1934] {
        let eq148_e1706: f64 = (s.v[0] * (nv8 - nv17));
        let eq148_e1706_d_n8: f64 = s.v[0];
        let eq148_e1706_d_n17: f64 = (-s.v[0]);
        let eq148_e1707: f64 = (s.v[123] + eq148_e1706);
        let eq148_e1707_d_n8: f64 = (s.dn[123][8] + eq148_e1706_d_n8);
        let eq148_e1707_d_n17: f64 = (s.dn[123][17] + eq148_e1706_d_n17);
        (eq148_e1707, s.dn[123][0], s.dn[123][1], s.dn[123][2], s.dn[123][3], s.dn[123][4], s.dn[123][5], s.dn[123][6], s.dn[123][7], eq148_e1707_d_n8, s.dn[123][9], s.dn[123][10], s.dn[123][11], s.dn[123][12], s.dn[123][13], s.dn[123][14], s.dn[123][15], s.dn[123][16], eq148_e1707_d_n17, s.dn[123][18], s.dn[123][19], s.dn[123][20], s.dn[123][21], s.dn[123][22], s.dn[123][23], s.dn[123][24], s.dn[123][25], s.dn[123][26], s.dn[123][27], s.dn[123][28], s.dn[123][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_value: f64 = eq148_e1709;
        let eq148_node_derivatives: [f64; 30] = [eq148_e1709_d_n0, eq148_e1709_d_n1, eq148_e1709_d_n2, eq148_e1709_d_n3, eq148_e1709_d_n4, eq148_e1709_d_n5, eq148_e1709_d_n6, eq148_e1709_d_n7, eq148_e1709_d_n8, eq148_e1709_d_n9, eq148_e1709_d_n10, eq148_e1709_d_n11, eq148_e1709_d_n12, eq148_e1709_d_n13, eq148_e1709_d_n14, eq148_e1709_d_n15, eq148_e1709_d_n16, eq148_e1709_d_n17, eq148_e1709_d_n18, eq148_e1709_d_n19, eq148_e1709_d_n20, eq148_e1709_d_n21, eq148_e1709_d_n22, eq148_e1709_d_n23, eq148_e1709_d_n24, eq148_e1709_d_n25, eq148_e1709_d_n26, eq148_e1709_d_n27, eq148_e1709_d_n28, eq148_e1709_d_n29];
        let eq148_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[17]),
            multiplicity * (eq148_value),
            nodes,
            &eq148_node_derivatives,
            branches,
            &eq148_branch_derivatives,
            multiplicity,
        );
        let (eq149_e1719, eq149_e1719_d_n0, eq149_e1719_d_n1, eq149_e1719_d_n2, eq149_e1719_d_n3, eq149_e1719_d_n4, eq149_e1719_d_n5, eq149_e1719_d_n6, eq149_e1719_d_n7, eq149_e1719_d_n8, eq149_e1719_d_n9, eq149_e1719_d_n10, eq149_e1719_d_n11, eq149_e1719_d_n12, eq149_e1719_d_n13, eq149_e1719_d_n14, eq149_e1719_d_n15, eq149_e1719_d_n16, eq149_e1719_d_n17, eq149_e1719_d_n18, eq149_e1719_d_n19, eq149_e1719_d_n20, eq149_e1719_d_n21, eq149_e1719_d_n22, eq149_e1719_d_n23, eq149_e1719_d_n24, eq149_e1719_d_n25, eq149_e1719_d_n26, eq149_e1719_d_n27, eq149_e1719_d_n28, eq149_e1719_d_n29,) = {
    if (s.b[1934] && s.b[2055]) {
        let eq149_e1716: f64 = (s.v[0] * (nv8 - nv13));
        let eq149_e1716_d_n8: f64 = s.v[0];
        let eq149_e1716_d_n13: f64 = (-s.v[0]);
        let eq149_e1717: f64 = (s.v[134] + eq149_e1716);
        let eq149_e1717_d_n8: f64 = (s.dn[134][8] + eq149_e1716_d_n8);
        let eq149_e1717_d_n13: f64 = (s.dn[134][13] + eq149_e1716_d_n13);
        (eq149_e1717, s.dn[134][0], s.dn[134][1], s.dn[134][2], s.dn[134][3], s.dn[134][4], s.dn[134][5], s.dn[134][6], s.dn[134][7], eq149_e1717_d_n8, s.dn[134][9], s.dn[134][10], s.dn[134][11], s.dn[134][12], eq149_e1717_d_n13, s.dn[134][14], s.dn[134][15], s.dn[134][16], s.dn[134][17], s.dn[134][18], s.dn[134][19], s.dn[134][20], s.dn[134][21], s.dn[134][22], s.dn[134][23], s.dn[134][24], s.dn[134][25], s.dn[134][26], s.dn[134][27], s.dn[134][28], s.dn[134][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_value: f64 = eq149_e1719;
        let eq149_node_derivatives: [f64; 30] = [eq149_e1719_d_n0, eq149_e1719_d_n1, eq149_e1719_d_n2, eq149_e1719_d_n3, eq149_e1719_d_n4, eq149_e1719_d_n5, eq149_e1719_d_n6, eq149_e1719_d_n7, eq149_e1719_d_n8, eq149_e1719_d_n9, eq149_e1719_d_n10, eq149_e1719_d_n11, eq149_e1719_d_n12, eq149_e1719_d_n13, eq149_e1719_d_n14, eq149_e1719_d_n15, eq149_e1719_d_n16, eq149_e1719_d_n17, eq149_e1719_d_n18, eq149_e1719_d_n19, eq149_e1719_d_n20, eq149_e1719_d_n21, eq149_e1719_d_n22, eq149_e1719_d_n23, eq149_e1719_d_n24, eq149_e1719_d_n25, eq149_e1719_d_n26, eq149_e1719_d_n27, eq149_e1719_d_n28, eq149_e1719_d_n29];
        let eq149_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[13]),
            multiplicity * (eq149_value),
            nodes,
            &eq149_node_derivatives,
            branches,
            &eq149_branch_derivatives,
            multiplicity,
        );
        let (eq150_e1729, eq150_e1729_d_n0, eq150_e1729_d_n1, eq150_e1729_d_n2, eq150_e1729_d_n3, eq150_e1729_d_n4, eq150_e1729_d_n5, eq150_e1729_d_n6, eq150_e1729_d_n7, eq150_e1729_d_n8, eq150_e1729_d_n9, eq150_e1729_d_n10, eq150_e1729_d_n11, eq150_e1729_d_n12, eq150_e1729_d_n13, eq150_e1729_d_n14, eq150_e1729_d_n15, eq150_e1729_d_n16, eq150_e1729_d_n17, eq150_e1729_d_n18, eq150_e1729_d_n19, eq150_e1729_d_n20, eq150_e1729_d_n21, eq150_e1729_d_n22, eq150_e1729_d_n23, eq150_e1729_d_n24, eq150_e1729_d_n25, eq150_e1729_d_n26, eq150_e1729_d_n27, eq150_e1729_d_n28, eq150_e1729_d_n29,) = {
    if (s.b[1934] && s.b[2055]) {
        let eq150_e1726: f64 = (s.v[0] * (nv8 - nv17));
        let eq150_e1726_d_n8: f64 = s.v[0];
        let eq150_e1726_d_n17: f64 = (-s.v[0]);
        let eq150_e1727: f64 = (s.v[135] + eq150_e1726);
        let eq150_e1727_d_n8: f64 = (s.dn[135][8] + eq150_e1726_d_n8);
        let eq150_e1727_d_n17: f64 = (s.dn[135][17] + eq150_e1726_d_n17);
        (eq150_e1727, s.dn[135][0], s.dn[135][1], s.dn[135][2], s.dn[135][3], s.dn[135][4], s.dn[135][5], s.dn[135][6], s.dn[135][7], eq150_e1727_d_n8, s.dn[135][9], s.dn[135][10], s.dn[135][11], s.dn[135][12], s.dn[135][13], s.dn[135][14], s.dn[135][15], s.dn[135][16], eq150_e1727_d_n17, s.dn[135][18], s.dn[135][19], s.dn[135][20], s.dn[135][21], s.dn[135][22], s.dn[135][23], s.dn[135][24], s.dn[135][25], s.dn[135][26], s.dn[135][27], s.dn[135][28], s.dn[135][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_value: f64 = eq150_e1729;
        let eq150_node_derivatives: [f64; 30] = [eq150_e1729_d_n0, eq150_e1729_d_n1, eq150_e1729_d_n2, eq150_e1729_d_n3, eq150_e1729_d_n4, eq150_e1729_d_n5, eq150_e1729_d_n6, eq150_e1729_d_n7, eq150_e1729_d_n8, eq150_e1729_d_n9, eq150_e1729_d_n10, eq150_e1729_d_n11, eq150_e1729_d_n12, eq150_e1729_d_n13, eq150_e1729_d_n14, eq150_e1729_d_n15, eq150_e1729_d_n16, eq150_e1729_d_n17, eq150_e1729_d_n18, eq150_e1729_d_n19, eq150_e1729_d_n20, eq150_e1729_d_n21, eq150_e1729_d_n22, eq150_e1729_d_n23, eq150_e1729_d_n24, eq150_e1729_d_n25, eq150_e1729_d_n26, eq150_e1729_d_n27, eq150_e1729_d_n28, eq150_e1729_d_n29];
        let eq150_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[17]),
            multiplicity * (eq150_value),
            nodes,
            &eq150_node_derivatives,
            branches,
            &eq150_branch_derivatives,
            multiplicity,
        );
        let (eq151_e1739, eq151_e1739_d_n0, eq151_e1739_d_n1, eq151_e1739_d_n2, eq151_e1739_d_n3, eq151_e1739_d_n4, eq151_e1739_d_n5, eq151_e1739_d_n6, eq151_e1739_d_n7, eq151_e1739_d_n8, eq151_e1739_d_n9, eq151_e1739_d_n10, eq151_e1739_d_n11, eq151_e1739_d_n12, eq151_e1739_d_n13, eq151_e1739_d_n14, eq151_e1739_d_n15, eq151_e1739_d_n16, eq151_e1739_d_n17, eq151_e1739_d_n18, eq151_e1739_d_n19, eq151_e1739_d_n20, eq151_e1739_d_n21, eq151_e1739_d_n22, eq151_e1739_d_n23, eq151_e1739_d_n24, eq151_e1739_d_n25, eq151_e1739_d_n26, eq151_e1739_d_n27, eq151_e1739_d_n28, eq151_e1739_d_n29,) = {
    if (s.b[1934] && s.b[2176]) {
        let eq151_e1736: f64 = (s.v[0] * (nv8 - nv9));
        let eq151_e1736_d_n8: f64 = s.v[0];
        let eq151_e1736_d_n9: f64 = (-s.v[0]);
        let eq151_e1737: f64 = (s.v[128] + eq151_e1736);
        let eq151_e1737_d_n8: f64 = (s.dn[128][8] + eq151_e1736_d_n8);
        let eq151_e1737_d_n9: f64 = (s.dn[128][9] + eq151_e1736_d_n9);
        (eq151_e1737, s.dn[128][0], s.dn[128][1], s.dn[128][2], s.dn[128][3], s.dn[128][4], s.dn[128][5], s.dn[128][6], s.dn[128][7], eq151_e1737_d_n8, eq151_e1737_d_n9, s.dn[128][10], s.dn[128][11], s.dn[128][12], s.dn[128][13], s.dn[128][14], s.dn[128][15], s.dn[128][16], s.dn[128][17], s.dn[128][18], s.dn[128][19], s.dn[128][20], s.dn[128][21], s.dn[128][22], s.dn[128][23], s.dn[128][24], s.dn[128][25], s.dn[128][26], s.dn[128][27], s.dn[128][28], s.dn[128][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_value: f64 = eq151_e1739;
        let eq151_node_derivatives: [f64; 30] = [eq151_e1739_d_n0, eq151_e1739_d_n1, eq151_e1739_d_n2, eq151_e1739_d_n3, eq151_e1739_d_n4, eq151_e1739_d_n5, eq151_e1739_d_n6, eq151_e1739_d_n7, eq151_e1739_d_n8, eq151_e1739_d_n9, eq151_e1739_d_n10, eq151_e1739_d_n11, eq151_e1739_d_n12, eq151_e1739_d_n13, eq151_e1739_d_n14, eq151_e1739_d_n15, eq151_e1739_d_n16, eq151_e1739_d_n17, eq151_e1739_d_n18, eq151_e1739_d_n19, eq151_e1739_d_n20, eq151_e1739_d_n21, eq151_e1739_d_n22, eq151_e1739_d_n23, eq151_e1739_d_n24, eq151_e1739_d_n25, eq151_e1739_d_n26, eq151_e1739_d_n27, eq151_e1739_d_n28, eq151_e1739_d_n29];
        let eq151_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            multiplicity * (eq151_value),
            nodes,
            &eq151_node_derivatives,
            branches,
            &eq151_branch_derivatives,
            multiplicity,
        );
        let (eq152_e1749, eq152_e1749_d_n0, eq152_e1749_d_n1, eq152_e1749_d_n2, eq152_e1749_d_n3, eq152_e1749_d_n4, eq152_e1749_d_n5, eq152_e1749_d_n6, eq152_e1749_d_n7, eq152_e1749_d_n8, eq152_e1749_d_n9, eq152_e1749_d_n10, eq152_e1749_d_n11, eq152_e1749_d_n12, eq152_e1749_d_n13, eq152_e1749_d_n14, eq152_e1749_d_n15, eq152_e1749_d_n16, eq152_e1749_d_n17, eq152_e1749_d_n18, eq152_e1749_d_n19, eq152_e1749_d_n20, eq152_e1749_d_n21, eq152_e1749_d_n22, eq152_e1749_d_n23, eq152_e1749_d_n24, eq152_e1749_d_n25, eq152_e1749_d_n26, eq152_e1749_d_n27, eq152_e1749_d_n28, eq152_e1749_d_n29,) = {
    if (s.b[1934] && s.b[2176]) {
        let eq152_e1746: f64 = (s.v[0] * (nv8 - nv5));
        let eq152_e1746_d_n5: f64 = (-s.v[0]);
        let eq152_e1746_d_n8: f64 = s.v[0];
        let eq152_e1747: f64 = (s.v[129] + eq152_e1746);
        let eq152_e1747_d_n5: f64 = (s.dn[129][5] + eq152_e1746_d_n5);
        let eq152_e1747_d_n8: f64 = (s.dn[129][8] + eq152_e1746_d_n8);
        (eq152_e1747, s.dn[129][0], s.dn[129][1], s.dn[129][2], s.dn[129][3], s.dn[129][4], eq152_e1747_d_n5, s.dn[129][6], s.dn[129][7], eq152_e1747_d_n8, s.dn[129][9], s.dn[129][10], s.dn[129][11], s.dn[129][12], s.dn[129][13], s.dn[129][14], s.dn[129][15], s.dn[129][16], s.dn[129][17], s.dn[129][18], s.dn[129][19], s.dn[129][20], s.dn[129][21], s.dn[129][22], s.dn[129][23], s.dn[129][24], s.dn[129][25], s.dn[129][26], s.dn[129][27], s.dn[129][28], s.dn[129][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_value: f64 = eq152_e1749;
        let eq152_node_derivatives: [f64; 30] = [eq152_e1749_d_n0, eq152_e1749_d_n1, eq152_e1749_d_n2, eq152_e1749_d_n3, eq152_e1749_d_n4, eq152_e1749_d_n5, eq152_e1749_d_n6, eq152_e1749_d_n7, eq152_e1749_d_n8, eq152_e1749_d_n9, eq152_e1749_d_n10, eq152_e1749_d_n11, eq152_e1749_d_n12, eq152_e1749_d_n13, eq152_e1749_d_n14, eq152_e1749_d_n15, eq152_e1749_d_n16, eq152_e1749_d_n17, eq152_e1749_d_n18, eq152_e1749_d_n19, eq152_e1749_d_n20, eq152_e1749_d_n21, eq152_e1749_d_n22, eq152_e1749_d_n23, eq152_e1749_d_n24, eq152_e1749_d_n25, eq152_e1749_d_n26, eq152_e1749_d_n27, eq152_e1749_d_n28, eq152_e1749_d_n29];
        let eq152_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            multiplicity * (eq152_value),
            nodes,
            &eq152_node_derivatives,
            branches,
            &eq152_branch_derivatives,
            multiplicity,
        );
        let (eq153_e1761, eq153_e1761_d_n0, eq153_e1761_d_n1, eq153_e1761_d_n2, eq153_e1761_d_n3, eq153_e1761_d_n4, eq153_e1761_d_n5, eq153_e1761_d_n6, eq153_e1761_d_n7, eq153_e1761_d_n8, eq153_e1761_d_n9, eq153_e1761_d_n10, eq153_e1761_d_n11, eq153_e1761_d_n12, eq153_e1761_d_n13, eq153_e1761_d_n14, eq153_e1761_d_n15, eq153_e1761_d_n16, eq153_e1761_d_n17, eq153_e1761_d_n18, eq153_e1761_d_n19, eq153_e1761_d_n20, eq153_e1761_d_n21, eq153_e1761_d_n22, eq153_e1761_d_n23, eq153_e1761_d_n24, eq153_e1761_d_n25, eq153_e1761_d_n26, eq153_e1761_d_n27, eq153_e1761_d_n28, eq153_e1761_d_n29,) = {
    if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
        let eq153_e1758: f64 = (s.v[0] * (nv8 - nv9));
        let eq153_e1758_d_n8: f64 = s.v[0];
        let eq153_e1758_d_n9: f64 = (-s.v[0]);
        let eq153_e1759: f64 = (s.v[140] + eq153_e1758);
        let eq153_e1759_d_n8: f64 = (s.dn[140][8] + eq153_e1758_d_n8);
        let eq153_e1759_d_n9: f64 = (s.dn[140][9] + eq153_e1758_d_n9);
        (eq153_e1759, s.dn[140][0], s.dn[140][1], s.dn[140][2], s.dn[140][3], s.dn[140][4], s.dn[140][5], s.dn[140][6], s.dn[140][7], eq153_e1759_d_n8, eq153_e1759_d_n9, s.dn[140][10], s.dn[140][11], s.dn[140][12], s.dn[140][13], s.dn[140][14], s.dn[140][15], s.dn[140][16], s.dn[140][17], s.dn[140][18], s.dn[140][19], s.dn[140][20], s.dn[140][21], s.dn[140][22], s.dn[140][23], s.dn[140][24], s.dn[140][25], s.dn[140][26], s.dn[140][27], s.dn[140][28], s.dn[140][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_value: f64 = eq153_e1761;
        let eq153_node_derivatives: [f64; 30] = [eq153_e1761_d_n0, eq153_e1761_d_n1, eq153_e1761_d_n2, eq153_e1761_d_n3, eq153_e1761_d_n4, eq153_e1761_d_n5, eq153_e1761_d_n6, eq153_e1761_d_n7, eq153_e1761_d_n8, eq153_e1761_d_n9, eq153_e1761_d_n10, eq153_e1761_d_n11, eq153_e1761_d_n12, eq153_e1761_d_n13, eq153_e1761_d_n14, eq153_e1761_d_n15, eq153_e1761_d_n16, eq153_e1761_d_n17, eq153_e1761_d_n18, eq153_e1761_d_n19, eq153_e1761_d_n20, eq153_e1761_d_n21, eq153_e1761_d_n22, eq153_e1761_d_n23, eq153_e1761_d_n24, eq153_e1761_d_n25, eq153_e1761_d_n26, eq153_e1761_d_n27, eq153_e1761_d_n28, eq153_e1761_d_n29];
        let eq153_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            multiplicity * (eq153_value),
            nodes,
            &eq153_node_derivatives,
            branches,
            &eq153_branch_derivatives,
            multiplicity,
        );
        let (eq154_e1773, eq154_e1773_d_n0, eq154_e1773_d_n1, eq154_e1773_d_n2, eq154_e1773_d_n3, eq154_e1773_d_n4, eq154_e1773_d_n5, eq154_e1773_d_n6, eq154_e1773_d_n7, eq154_e1773_d_n8, eq154_e1773_d_n9, eq154_e1773_d_n10, eq154_e1773_d_n11, eq154_e1773_d_n12, eq154_e1773_d_n13, eq154_e1773_d_n14, eq154_e1773_d_n15, eq154_e1773_d_n16, eq154_e1773_d_n17, eq154_e1773_d_n18, eq154_e1773_d_n19, eq154_e1773_d_n20, eq154_e1773_d_n21, eq154_e1773_d_n22, eq154_e1773_d_n23, eq154_e1773_d_n24, eq154_e1773_d_n25, eq154_e1773_d_n26, eq154_e1773_d_n27, eq154_e1773_d_n28, eq154_e1773_d_n29,) = {
    if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
        let eq154_e1770: f64 = (s.v[0] * (nv8 - nv5));
        let eq154_e1770_d_n5: f64 = (-s.v[0]);
        let eq154_e1770_d_n8: f64 = s.v[0];
        let eq154_e1771: f64 = (s.v[141] + eq154_e1770);
        let eq154_e1771_d_n5: f64 = (s.dn[141][5] + eq154_e1770_d_n5);
        let eq154_e1771_d_n8: f64 = (s.dn[141][8] + eq154_e1770_d_n8);
        (eq154_e1771, s.dn[141][0], s.dn[141][1], s.dn[141][2], s.dn[141][3], s.dn[141][4], eq154_e1771_d_n5, s.dn[141][6], s.dn[141][7], eq154_e1771_d_n8, s.dn[141][9], s.dn[141][10], s.dn[141][11], s.dn[141][12], s.dn[141][13], s.dn[141][14], s.dn[141][15], s.dn[141][16], s.dn[141][17], s.dn[141][18], s.dn[141][19], s.dn[141][20], s.dn[141][21], s.dn[141][22], s.dn[141][23], s.dn[141][24], s.dn[141][25], s.dn[141][26], s.dn[141][27], s.dn[141][28], s.dn[141][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_value: f64 = eq154_e1773;
        let eq154_node_derivatives: [f64; 30] = [eq154_e1773_d_n0, eq154_e1773_d_n1, eq154_e1773_d_n2, eq154_e1773_d_n3, eq154_e1773_d_n4, eq154_e1773_d_n5, eq154_e1773_d_n6, eq154_e1773_d_n7, eq154_e1773_d_n8, eq154_e1773_d_n9, eq154_e1773_d_n10, eq154_e1773_d_n11, eq154_e1773_d_n12, eq154_e1773_d_n13, eq154_e1773_d_n14, eq154_e1773_d_n15, eq154_e1773_d_n16, eq154_e1773_d_n17, eq154_e1773_d_n18, eq154_e1773_d_n19, eq154_e1773_d_n20, eq154_e1773_d_n21, eq154_e1773_d_n22, eq154_e1773_d_n23, eq154_e1773_d_n24, eq154_e1773_d_n25, eq154_e1773_d_n26, eq154_e1773_d_n27, eq154_e1773_d_n28, eq154_e1773_d_n29];
        let eq154_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            multiplicity * (eq154_value),
            nodes,
            &eq154_node_derivatives,
            branches,
            &eq154_branch_derivatives,
            multiplicity,
        );
        let (eq155_e1781, eq155_e1781_d_n0, eq155_e1781_d_n1, eq155_e1781_d_n2, eq155_e1781_d_n3, eq155_e1781_d_n4, eq155_e1781_d_n5, eq155_e1781_d_n6, eq155_e1781_d_n7, eq155_e1781_d_n8, eq155_e1781_d_n9, eq155_e1781_d_n10, eq155_e1781_d_n11, eq155_e1781_d_n12, eq155_e1781_d_n13, eq155_e1781_d_n14, eq155_e1781_d_n15, eq155_e1781_d_n16, eq155_e1781_d_n17, eq155_e1781_d_n18, eq155_e1781_d_n19, eq155_e1781_d_n20, eq155_e1781_d_n21, eq155_e1781_d_n22, eq155_e1781_d_n23, eq155_e1781_d_n24, eq155_e1781_d_n25, eq155_e1781_d_n26, eq155_e1781_d_n27, eq155_e1781_d_n28, eq155_e1781_d_n29,) = {
    if s.b[2418] {
        let eq155_e1778: f64 = (s.v[0] * (nv8 - nv7));
        let eq155_e1778_d_n7: f64 = (-s.v[0]);
        let eq155_e1778_d_n8: f64 = s.v[0];
        let eq155_e1779: f64 = (s.v[235] + eq155_e1778);
        let eq155_e1779_d_n7: f64 = (s.dn[235][7] + eq155_e1778_d_n7);
        let eq155_e1779_d_n8: f64 = (s.dn[235][8] + eq155_e1778_d_n8);
        (eq155_e1779, s.dn[235][0], s.dn[235][1], s.dn[235][2], s.dn[235][3], s.dn[235][4], s.dn[235][5], s.dn[235][6], eq155_e1779_d_n7, eq155_e1779_d_n8, s.dn[235][9], s.dn[235][10], s.dn[235][11], s.dn[235][12], s.dn[235][13], s.dn[235][14], s.dn[235][15], s.dn[235][16], s.dn[235][17], s.dn[235][18], s.dn[235][19], s.dn[235][20], s.dn[235][21], s.dn[235][22], s.dn[235][23], s.dn[235][24], s.dn[235][25], s.dn[235][26], s.dn[235][27], s.dn[235][28], s.dn[235][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_value: f64 = eq155_e1781;
        let eq155_node_derivatives: [f64; 30] = [eq155_e1781_d_n0, eq155_e1781_d_n1, eq155_e1781_d_n2, eq155_e1781_d_n3, eq155_e1781_d_n4, eq155_e1781_d_n5, eq155_e1781_d_n6, eq155_e1781_d_n7, eq155_e1781_d_n8, eq155_e1781_d_n9, eq155_e1781_d_n10, eq155_e1781_d_n11, eq155_e1781_d_n12, eq155_e1781_d_n13, eq155_e1781_d_n14, eq155_e1781_d_n15, eq155_e1781_d_n16, eq155_e1781_d_n17, eq155_e1781_d_n18, eq155_e1781_d_n19, eq155_e1781_d_n20, eq155_e1781_d_n21, eq155_e1781_d_n22, eq155_e1781_d_n23, eq155_e1781_d_n24, eq155_e1781_d_n25, eq155_e1781_d_n26, eq155_e1781_d_n27, eq155_e1781_d_n28, eq155_e1781_d_n29];
        let eq155_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            multiplicity * (eq155_value),
            nodes,
            &eq155_node_derivatives,
            branches,
            &eq155_branch_derivatives,
            multiplicity,
        );
        let (eq156_e1791, eq156_e1791_d_n0, eq156_e1791_d_n1, eq156_e1791_d_n2, eq156_e1791_d_n3, eq156_e1791_d_n4, eq156_e1791_d_n5, eq156_e1791_d_n6, eq156_e1791_d_n7, eq156_e1791_d_n8, eq156_e1791_d_n9, eq156_e1791_d_n10, eq156_e1791_d_n11, eq156_e1791_d_n12, eq156_e1791_d_n13, eq156_e1791_d_n14, eq156_e1791_d_n15, eq156_e1791_d_n16, eq156_e1791_d_n17, eq156_e1791_d_n18, eq156_e1791_d_n19, eq156_e1791_d_n20, eq156_e1791_d_n21, eq156_e1791_d_n22, eq156_e1791_d_n23, eq156_e1791_d_n24, eq156_e1791_d_n25, eq156_e1791_d_n26, eq156_e1791_d_n27, eq156_e1791_d_n28, eq156_e1791_d_n29,) = {
    if (s.b[2418] && s.b[2479]) {
        let eq156_e1788: f64 = (s.v[0] * (nv8 - nv7));
        let eq156_e1788_d_n7: f64 = (-s.v[0]);
        let eq156_e1788_d_n8: f64 = s.v[0];
        let eq156_e1789: f64 = (s.v[238] + eq156_e1788);
        let eq156_e1789_d_n7: f64 = (s.dn[238][7] + eq156_e1788_d_n7);
        let eq156_e1789_d_n8: f64 = (s.dn[238][8] + eq156_e1788_d_n8);
        (eq156_e1789, s.dn[238][0], s.dn[238][1], s.dn[238][2], s.dn[238][3], s.dn[238][4], s.dn[238][5], s.dn[238][6], eq156_e1789_d_n7, eq156_e1789_d_n8, s.dn[238][9], s.dn[238][10], s.dn[238][11], s.dn[238][12], s.dn[238][13], s.dn[238][14], s.dn[238][15], s.dn[238][16], s.dn[238][17], s.dn[238][18], s.dn[238][19], s.dn[238][20], s.dn[238][21], s.dn[238][22], s.dn[238][23], s.dn[238][24], s.dn[238][25], s.dn[238][26], s.dn[238][27], s.dn[238][28], s.dn[238][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_value: f64 = eq156_e1791;
        let eq156_node_derivatives: [f64; 30] = [eq156_e1791_d_n0, eq156_e1791_d_n1, eq156_e1791_d_n2, eq156_e1791_d_n3, eq156_e1791_d_n4, eq156_e1791_d_n5, eq156_e1791_d_n6, eq156_e1791_d_n7, eq156_e1791_d_n8, eq156_e1791_d_n9, eq156_e1791_d_n10, eq156_e1791_d_n11, eq156_e1791_d_n12, eq156_e1791_d_n13, eq156_e1791_d_n14, eq156_e1791_d_n15, eq156_e1791_d_n16, eq156_e1791_d_n17, eq156_e1791_d_n18, eq156_e1791_d_n19, eq156_e1791_d_n20, eq156_e1791_d_n21, eq156_e1791_d_n22, eq156_e1791_d_n23, eq156_e1791_d_n24, eq156_e1791_d_n25, eq156_e1791_d_n26, eq156_e1791_d_n27, eq156_e1791_d_n28, eq156_e1791_d_n29];
        let eq156_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            multiplicity * (eq156_value),
            nodes,
            &eq156_node_derivatives,
            branches,
            &eq156_branch_derivatives,
            multiplicity,
        );
        let (eq157_e1796, eq157_e1796_d_n0, eq157_e1796_d_n1, eq157_e1796_d_n2, eq157_e1796_d_n3, eq157_e1796_d_n4, eq157_e1796_d_n5, eq157_e1796_d_n6, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_d_n9, eq157_e1796_d_n10, eq157_e1796_d_n11, eq157_e1796_d_n12, eq157_e1796_d_n13, eq157_e1796_d_n14, eq157_e1796_d_n15, eq157_e1796_d_n16, eq157_e1796_d_n17, eq157_e1796_d_n18, eq157_e1796_d_n19, eq157_e1796_d_n20, eq157_e1796_d_n21, eq157_e1796_d_n22, eq157_e1796_d_n23, eq157_e1796_d_n24, eq157_e1796_d_n25, eq157_e1796_d_n26, eq157_e1796_d_n27, eq157_e1796_d_n28, eq157_e1796_d_n29,) = {
    if s.b[2418] {
        let eq157_e1794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 138, s.v[242]);
        let eq157_e1794_d_n0: f64 = (s.dn[242][0] * ddt_scale);
        let eq157_e1794_d_n1: f64 = (s.dn[242][1] * ddt_scale);
        let eq157_e1794_d_n2: f64 = (s.dn[242][2] * ddt_scale);
        let eq157_e1794_d_n3: f64 = (s.dn[242][3] * ddt_scale);
        let eq157_e1794_d_n4: f64 = (s.dn[242][4] * ddt_scale);
        let eq157_e1794_d_n5: f64 = (s.dn[242][5] * ddt_scale);
        let eq157_e1794_d_n6: f64 = (s.dn[242][6] * ddt_scale);
        let eq157_e1794_d_n7: f64 = (s.dn[242][7] * ddt_scale);
        let eq157_e1794_d_n8: f64 = (s.dn[242][8] * ddt_scale);
        let eq157_e1794_d_n9: f64 = (s.dn[242][9] * ddt_scale);
        let eq157_e1794_d_n10: f64 = (s.dn[242][10] * ddt_scale);
        let eq157_e1794_d_n11: f64 = (s.dn[242][11] * ddt_scale);
        let eq157_e1794_d_n12: f64 = (s.dn[242][12] * ddt_scale);
        let eq157_e1794_d_n13: f64 = (s.dn[242][13] * ddt_scale);
        let eq157_e1794_d_n14: f64 = (s.dn[242][14] * ddt_scale);
        let eq157_e1794_d_n15: f64 = (s.dn[242][15] * ddt_scale);
        let eq157_e1794_d_n16: f64 = (s.dn[242][16] * ddt_scale);
        let eq157_e1794_d_n17: f64 = (s.dn[242][17] * ddt_scale);
        let eq157_e1794_d_n18: f64 = (s.dn[242][18] * ddt_scale);
        let eq157_e1794_d_n19: f64 = (s.dn[242][19] * ddt_scale);
        let eq157_e1794_d_n20: f64 = (s.dn[242][20] * ddt_scale);
        let eq157_e1794_d_n21: f64 = (s.dn[242][21] * ddt_scale);
        let eq157_e1794_d_n22: f64 = (s.dn[242][22] * ddt_scale);
        let eq157_e1794_d_n23: f64 = (s.dn[242][23] * ddt_scale);
        let eq157_e1794_d_n24: f64 = (s.dn[242][24] * ddt_scale);
        let eq157_e1794_d_n25: f64 = (s.dn[242][25] * ddt_scale);
        let eq157_e1794_d_n26: f64 = (s.dn[242][26] * ddt_scale);
        let eq157_e1794_d_n27: f64 = (s.dn[242][27] * ddt_scale);
        let eq157_e1794_d_n28: f64 = (s.dn[242][28] * ddt_scale);
        let eq157_e1794_d_n29: f64 = (s.dn[242][29] * ddt_scale);
        (eq157_e1794, eq157_e1794_d_n0, eq157_e1794_d_n1, eq157_e1794_d_n2, eq157_e1794_d_n3, eq157_e1794_d_n4, eq157_e1794_d_n5, eq157_e1794_d_n6, eq157_e1794_d_n7, eq157_e1794_d_n8, eq157_e1794_d_n9, eq157_e1794_d_n10, eq157_e1794_d_n11, eq157_e1794_d_n12, eq157_e1794_d_n13, eq157_e1794_d_n14, eq157_e1794_d_n15, eq157_e1794_d_n16, eq157_e1794_d_n17, eq157_e1794_d_n18, eq157_e1794_d_n19, eq157_e1794_d_n20, eq157_e1794_d_n21, eq157_e1794_d_n22, eq157_e1794_d_n23, eq157_e1794_d_n24, eq157_e1794_d_n25, eq157_e1794_d_n26, eq157_e1794_d_n27, eq157_e1794_d_n28, eq157_e1794_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_value: f64 = eq157_e1796;
        let eq157_node_derivatives: [f64; 30] = [eq157_e1796_d_n0, eq157_e1796_d_n1, eq157_e1796_d_n2, eq157_e1796_d_n3, eq157_e1796_d_n4, eq157_e1796_d_n5, eq157_e1796_d_n6, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_d_n9, eq157_e1796_d_n10, eq157_e1796_d_n11, eq157_e1796_d_n12, eq157_e1796_d_n13, eq157_e1796_d_n14, eq157_e1796_d_n15, eq157_e1796_d_n16, eq157_e1796_d_n17, eq157_e1796_d_n18, eq157_e1796_d_n19, eq157_e1796_d_n20, eq157_e1796_d_n21, eq157_e1796_d_n22, eq157_e1796_d_n23, eq157_e1796_d_n24, eq157_e1796_d_n25, eq157_e1796_d_n26, eq157_e1796_d_n27, eq157_e1796_d_n28, eq157_e1796_d_n29];
        let eq157_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            multiplicity * (eq157_value),
            nodes,
            &eq157_node_derivatives,
            branches,
            &eq157_branch_derivatives,
            multiplicity,
        );
        let (eq158_e1804, eq158_e1804_d_n0, eq158_e1804_d_n1, eq158_e1804_d_n2, eq158_e1804_d_n3, eq158_e1804_d_n4, eq158_e1804_d_n5, eq158_e1804_d_n6, eq158_e1804_d_n7, eq158_e1804_d_n8, eq158_e1804_d_n9, eq158_e1804_d_n10, eq158_e1804_d_n11, eq158_e1804_d_n12, eq158_e1804_d_n13, eq158_e1804_d_n14, eq158_e1804_d_n15, eq158_e1804_d_n16, eq158_e1804_d_n17, eq158_e1804_d_n18, eq158_e1804_d_n19, eq158_e1804_d_n20, eq158_e1804_d_n21, eq158_e1804_d_n22, eq158_e1804_d_n23, eq158_e1804_d_n24, eq158_e1804_d_n25, eq158_e1804_d_n26, eq158_e1804_d_n27, eq158_e1804_d_n28, eq158_e1804_d_n29,) = {
    if (s.b[2418] && s.b[2546]) {
        let eq158_e1802: f64 = ((nv8 - nv7) / s.v[241]);
        let eq158_e1802_d_n0: f64 = (-(((nv8 - nv7) * s.dn[241][0]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n1: f64 = (-(((nv8 - nv7) * s.dn[241][1]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n2: f64 = (-(((nv8 - nv7) * s.dn[241][2]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n3: f64 = (-(((nv8 - nv7) * s.dn[241][3]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n4: f64 = (-(((nv8 - nv7) * s.dn[241][4]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n5: f64 = (-(((nv8 - nv7) * s.dn[241][5]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n6: f64 = (-(((nv8 - nv7) * s.dn[241][6]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n7: f64 = (((-s.v[241]) - ((nv8 - nv7) * s.dn[241][7])) / (s.v[241] * s.v[241]));
        let eq158_e1802_d_n8: f64 = ((s.v[241] - ((nv8 - nv7) * s.dn[241][8])) / (s.v[241] * s.v[241]));
        let eq158_e1802_d_n9: f64 = (-(((nv8 - nv7) * s.dn[241][9]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n10: f64 = (-(((nv8 - nv7) * s.dn[241][10]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n11: f64 = (-(((nv8 - nv7) * s.dn[241][11]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n12: f64 = (-(((nv8 - nv7) * s.dn[241][12]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n13: f64 = (-(((nv8 - nv7) * s.dn[241][13]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n14: f64 = (-(((nv8 - nv7) * s.dn[241][14]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n15: f64 = (-(((nv8 - nv7) * s.dn[241][15]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n16: f64 = (-(((nv8 - nv7) * s.dn[241][16]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n17: f64 = (-(((nv8 - nv7) * s.dn[241][17]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n18: f64 = (-(((nv8 - nv7) * s.dn[241][18]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n19: f64 = (-(((nv8 - nv7) * s.dn[241][19]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n20: f64 = (-(((nv8 - nv7) * s.dn[241][20]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n21: f64 = (-(((nv8 - nv7) * s.dn[241][21]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n22: f64 = (-(((nv8 - nv7) * s.dn[241][22]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n23: f64 = (-(((nv8 - nv7) * s.dn[241][23]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n24: f64 = (-(((nv8 - nv7) * s.dn[241][24]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n25: f64 = (-(((nv8 - nv7) * s.dn[241][25]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n26: f64 = (-(((nv8 - nv7) * s.dn[241][26]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n27: f64 = (-(((nv8 - nv7) * s.dn[241][27]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n28: f64 = (-(((nv8 - nv7) * s.dn[241][28]) / (s.v[241] * s.v[241])));
        let eq158_e1802_d_n29: f64 = (-(((nv8 - nv7) * s.dn[241][29]) / (s.v[241] * s.v[241])));
        (eq158_e1802, eq158_e1802_d_n0, eq158_e1802_d_n1, eq158_e1802_d_n2, eq158_e1802_d_n3, eq158_e1802_d_n4, eq158_e1802_d_n5, eq158_e1802_d_n6, eq158_e1802_d_n7, eq158_e1802_d_n8, eq158_e1802_d_n9, eq158_e1802_d_n10, eq158_e1802_d_n11, eq158_e1802_d_n12, eq158_e1802_d_n13, eq158_e1802_d_n14, eq158_e1802_d_n15, eq158_e1802_d_n16, eq158_e1802_d_n17, eq158_e1802_d_n18, eq158_e1802_d_n19, eq158_e1802_d_n20, eq158_e1802_d_n21, eq158_e1802_d_n22, eq158_e1802_d_n23, eq158_e1802_d_n24, eq158_e1802_d_n25, eq158_e1802_d_n26, eq158_e1802_d_n27, eq158_e1802_d_n28, eq158_e1802_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq158_value: f64 = eq158_e1804;
        let eq158_node_derivatives: [f64; 30] = [eq158_e1804_d_n0, eq158_e1804_d_n1, eq158_e1804_d_n2, eq158_e1804_d_n3, eq158_e1804_d_n4, eq158_e1804_d_n5, eq158_e1804_d_n6, eq158_e1804_d_n7, eq158_e1804_d_n8, eq158_e1804_d_n9, eq158_e1804_d_n10, eq158_e1804_d_n11, eq158_e1804_d_n12, eq158_e1804_d_n13, eq158_e1804_d_n14, eq158_e1804_d_n15, eq158_e1804_d_n16, eq158_e1804_d_n17, eq158_e1804_d_n18, eq158_e1804_d_n19, eq158_e1804_d_n20, eq158_e1804_d_n21, eq158_e1804_d_n22, eq158_e1804_d_n23, eq158_e1804_d_n24, eq158_e1804_d_n25, eq158_e1804_d_n26, eq158_e1804_d_n27, eq158_e1804_d_n28, eq158_e1804_d_n29];
        let eq158_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            multiplicity * (eq158_value),
            nodes,
            &eq158_node_derivatives,
            branches,
            &eq158_branch_derivatives,
            multiplicity,
        );
        let (eq159_e1809,) = {
    if (!s.b[2418]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq159_value: f64 = eq159_e1809;
        stamper.stamp_potential_const(
            branches[30],
            eq159_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_13(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let (eq160_e1815, eq160_e1815_d_n0, eq160_e1815_d_n1, eq160_e1815_d_n2, eq160_e1815_d_n3, eq160_e1815_d_n4, eq160_e1815_d_n5, eq160_e1815_d_n6, eq160_e1815_d_n7, eq160_e1815_d_n8, eq160_e1815_d_n9, eq160_e1815_d_n10, eq160_e1815_d_n11, eq160_e1815_d_n12, eq160_e1815_d_n13, eq160_e1815_d_n14, eq160_e1815_d_n15, eq160_e1815_d_n16, eq160_e1815_d_n17, eq160_e1815_d_n18, eq160_e1815_d_n19, eq160_e1815_d_n20, eq160_e1815_d_n21, eq160_e1815_d_n22, eq160_e1815_d_n23, eq160_e1815_d_n24, eq160_e1815_d_n25, eq160_e1815_d_n26, eq160_e1815_d_n27, eq160_e1815_d_n28, eq160_e1815_d_n29,) = {
    if (s.b[2547] && s.b[2669]) {
        (s.v[148], s.dn[148][0], s.dn[148][1], s.dn[148][2], s.dn[148][3], s.dn[148][4], s.dn[148][5], s.dn[148][6], s.dn[148][7], s.dn[148][8], s.dn[148][9], s.dn[148][10], s.dn[148][11], s.dn[148][12], s.dn[148][13], s.dn[148][14], s.dn[148][15], s.dn[148][16], s.dn[148][17], s.dn[148][18], s.dn[148][19], s.dn[148][20], s.dn[148][21], s.dn[148][22], s.dn[148][23], s.dn[148][24], s.dn[148][25], s.dn[148][26], s.dn[148][27], s.dn[148][28], s.dn[148][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_value: f64 = eq160_e1815;
        let eq160_node_derivatives: [f64; 30] = [eq160_e1815_d_n0, eq160_e1815_d_n1, eq160_e1815_d_n2, eq160_e1815_d_n3, eq160_e1815_d_n4, eq160_e1815_d_n5, eq160_e1815_d_n6, eq160_e1815_d_n7, eq160_e1815_d_n8, eq160_e1815_d_n9, eq160_e1815_d_n10, eq160_e1815_d_n11, eq160_e1815_d_n12, eq160_e1815_d_n13, eq160_e1815_d_n14, eq160_e1815_d_n15, eq160_e1815_d_n16, eq160_e1815_d_n17, eq160_e1815_d_n18, eq160_e1815_d_n19, eq160_e1815_d_n20, eq160_e1815_d_n21, eq160_e1815_d_n22, eq160_e1815_d_n23, eq160_e1815_d_n24, eq160_e1815_d_n25, eq160_e1815_d_n26, eq160_e1815_d_n27, eq160_e1815_d_n28, eq160_e1815_d_n29];
        let eq160_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            multiplicity * (eq160_value),
            nodes,
            &eq160_node_derivatives,
            branches,
            &eq160_branch_derivatives,
            multiplicity,
        );
        let (eq161_e1821, eq161_e1821_d_n0, eq161_e1821_d_n1, eq161_e1821_d_n2, eq161_e1821_d_n3, eq161_e1821_d_n4, eq161_e1821_d_n5, eq161_e1821_d_n6, eq161_e1821_d_n7, eq161_e1821_d_n8, eq161_e1821_d_n9, eq161_e1821_d_n10, eq161_e1821_d_n11, eq161_e1821_d_n12, eq161_e1821_d_n13, eq161_e1821_d_n14, eq161_e1821_d_n15, eq161_e1821_d_n16, eq161_e1821_d_n17, eq161_e1821_d_n18, eq161_e1821_d_n19, eq161_e1821_d_n20, eq161_e1821_d_n21, eq161_e1821_d_n22, eq161_e1821_d_n23, eq161_e1821_d_n24, eq161_e1821_d_n25, eq161_e1821_d_n26, eq161_e1821_d_n27, eq161_e1821_d_n28, eq161_e1821_d_n29,) = {
    if (s.b[2547] && s.b[2669]) {
        (s.v[149], s.dn[149][0], s.dn[149][1], s.dn[149][2], s.dn[149][3], s.dn[149][4], s.dn[149][5], s.dn[149][6], s.dn[149][7], s.dn[149][8], s.dn[149][9], s.dn[149][10], s.dn[149][11], s.dn[149][12], s.dn[149][13], s.dn[149][14], s.dn[149][15], s.dn[149][16], s.dn[149][17], s.dn[149][18], s.dn[149][19], s.dn[149][20], s.dn[149][21], s.dn[149][22], s.dn[149][23], s.dn[149][24], s.dn[149][25], s.dn[149][26], s.dn[149][27], s.dn[149][28], s.dn[149][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_value: f64 = eq161_e1821;
        let eq161_node_derivatives: [f64; 30] = [eq161_e1821_d_n0, eq161_e1821_d_n1, eq161_e1821_d_n2, eq161_e1821_d_n3, eq161_e1821_d_n4, eq161_e1821_d_n5, eq161_e1821_d_n6, eq161_e1821_d_n7, eq161_e1821_d_n8, eq161_e1821_d_n9, eq161_e1821_d_n10, eq161_e1821_d_n11, eq161_e1821_d_n12, eq161_e1821_d_n13, eq161_e1821_d_n14, eq161_e1821_d_n15, eq161_e1821_d_n16, eq161_e1821_d_n17, eq161_e1821_d_n18, eq161_e1821_d_n19, eq161_e1821_d_n20, eq161_e1821_d_n21, eq161_e1821_d_n22, eq161_e1821_d_n23, eq161_e1821_d_n24, eq161_e1821_d_n25, eq161_e1821_d_n26, eq161_e1821_d_n27, eq161_e1821_d_n28, eq161_e1821_d_n29];
        let eq161_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            multiplicity * (eq161_value),
            nodes,
            &eq161_node_derivatives,
            branches,
            &eq161_branch_derivatives,
            multiplicity,
        );
        let (eq162_e1828, eq162_e1828_d_n0, eq162_e1828_d_n1, eq162_e1828_d_n2, eq162_e1828_d_n3, eq162_e1828_d_n4, eq162_e1828_d_n5, eq162_e1828_d_n6, eq162_e1828_d_n7, eq162_e1828_d_n8, eq162_e1828_d_n9, eq162_e1828_d_n10, eq162_e1828_d_n11, eq162_e1828_d_n12, eq162_e1828_d_n13, eq162_e1828_d_n14, eq162_e1828_d_n15, eq162_e1828_d_n16, eq162_e1828_d_n17, eq162_e1828_d_n18, eq162_e1828_d_n19, eq162_e1828_d_n20, eq162_e1828_d_n21, eq162_e1828_d_n22, eq162_e1828_d_n23, eq162_e1828_d_n24, eq162_e1828_d_n25, eq162_e1828_d_n26, eq162_e1828_d_n27, eq162_e1828_d_n28, eq162_e1828_d_n29,) = {
    if (s.b[2547] && (!s.b[2669])) {
        (s.v[148], s.dn[148][0], s.dn[148][1], s.dn[148][2], s.dn[148][3], s.dn[148][4], s.dn[148][5], s.dn[148][6], s.dn[148][7], s.dn[148][8], s.dn[148][9], s.dn[148][10], s.dn[148][11], s.dn[148][12], s.dn[148][13], s.dn[148][14], s.dn[148][15], s.dn[148][16], s.dn[148][17], s.dn[148][18], s.dn[148][19], s.dn[148][20], s.dn[148][21], s.dn[148][22], s.dn[148][23], s.dn[148][24], s.dn[148][25], s.dn[148][26], s.dn[148][27], s.dn[148][28], s.dn[148][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_value: f64 = eq162_e1828;
        let eq162_node_derivatives: [f64; 30] = [eq162_e1828_d_n0, eq162_e1828_d_n1, eq162_e1828_d_n2, eq162_e1828_d_n3, eq162_e1828_d_n4, eq162_e1828_d_n5, eq162_e1828_d_n6, eq162_e1828_d_n7, eq162_e1828_d_n8, eq162_e1828_d_n9, eq162_e1828_d_n10, eq162_e1828_d_n11, eq162_e1828_d_n12, eq162_e1828_d_n13, eq162_e1828_d_n14, eq162_e1828_d_n15, eq162_e1828_d_n16, eq162_e1828_d_n17, eq162_e1828_d_n18, eq162_e1828_d_n19, eq162_e1828_d_n20, eq162_e1828_d_n21, eq162_e1828_d_n22, eq162_e1828_d_n23, eq162_e1828_d_n24, eq162_e1828_d_n25, eq162_e1828_d_n26, eq162_e1828_d_n27, eq162_e1828_d_n28, eq162_e1828_d_n29];
        let eq162_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[19]),
            Some(nodes[18]),
            multiplicity * (eq162_value),
            nodes,
            &eq162_node_derivatives,
            branches,
            &eq162_branch_derivatives,
            multiplicity,
        );
        let (eq163_e1835, eq163_e1835_d_n0, eq163_e1835_d_n1, eq163_e1835_d_n2, eq163_e1835_d_n3, eq163_e1835_d_n4, eq163_e1835_d_n5, eq163_e1835_d_n6, eq163_e1835_d_n7, eq163_e1835_d_n8, eq163_e1835_d_n9, eq163_e1835_d_n10, eq163_e1835_d_n11, eq163_e1835_d_n12, eq163_e1835_d_n13, eq163_e1835_d_n14, eq163_e1835_d_n15, eq163_e1835_d_n16, eq163_e1835_d_n17, eq163_e1835_d_n18, eq163_e1835_d_n19, eq163_e1835_d_n20, eq163_e1835_d_n21, eq163_e1835_d_n22, eq163_e1835_d_n23, eq163_e1835_d_n24, eq163_e1835_d_n25, eq163_e1835_d_n26, eq163_e1835_d_n27, eq163_e1835_d_n28, eq163_e1835_d_n29,) = {
    if (s.b[2547] && (!s.b[2669])) {
        (s.v[149], s.dn[149][0], s.dn[149][1], s.dn[149][2], s.dn[149][3], s.dn[149][4], s.dn[149][5], s.dn[149][6], s.dn[149][7], s.dn[149][8], s.dn[149][9], s.dn[149][10], s.dn[149][11], s.dn[149][12], s.dn[149][13], s.dn[149][14], s.dn[149][15], s.dn[149][16], s.dn[149][17], s.dn[149][18], s.dn[149][19], s.dn[149][20], s.dn[149][21], s.dn[149][22], s.dn[149][23], s.dn[149][24], s.dn[149][25], s.dn[149][26], s.dn[149][27], s.dn[149][28], s.dn[149][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_value: f64 = eq163_e1835;
        let eq163_node_derivatives: [f64; 30] = [eq163_e1835_d_n0, eq163_e1835_d_n1, eq163_e1835_d_n2, eq163_e1835_d_n3, eq163_e1835_d_n4, eq163_e1835_d_n5, eq163_e1835_d_n6, eq163_e1835_d_n7, eq163_e1835_d_n8, eq163_e1835_d_n9, eq163_e1835_d_n10, eq163_e1835_d_n11, eq163_e1835_d_n12, eq163_e1835_d_n13, eq163_e1835_d_n14, eq163_e1835_d_n15, eq163_e1835_d_n16, eq163_e1835_d_n17, eq163_e1835_d_n18, eq163_e1835_d_n19, eq163_e1835_d_n20, eq163_e1835_d_n21, eq163_e1835_d_n22, eq163_e1835_d_n23, eq163_e1835_d_n24, eq163_e1835_d_n25, eq163_e1835_d_n26, eq163_e1835_d_n27, eq163_e1835_d_n28, eq163_e1835_d_n29];
        let eq163_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[18]),
            Some(nodes[19]),
            multiplicity * (eq163_value),
            nodes,
            &eq163_node_derivatives,
            branches,
            &eq163_branch_derivatives,
            multiplicity,
        );
        let (eq164_e1841, eq164_e1841_d_n0, eq164_e1841_d_n1, eq164_e1841_d_n2, eq164_e1841_d_n3, eq164_e1841_d_n4, eq164_e1841_d_n5, eq164_e1841_d_n6, eq164_e1841_d_n7, eq164_e1841_d_n8, eq164_e1841_d_n9, eq164_e1841_d_n10, eq164_e1841_d_n11, eq164_e1841_d_n12, eq164_e1841_d_n13, eq164_e1841_d_n14, eq164_e1841_d_n15, eq164_e1841_d_n16, eq164_e1841_d_n17, eq164_e1841_d_n18, eq164_e1841_d_n19, eq164_e1841_d_n20, eq164_e1841_d_n21, eq164_e1841_d_n22, eq164_e1841_d_n23, eq164_e1841_d_n24, eq164_e1841_d_n25, eq164_e1841_d_n26, eq164_e1841_d_n27, eq164_e1841_d_n28, eq164_e1841_d_n29,) = {
    if s.b[2670] {
        let eq164_e1839: f64 = ((nv0 - nv18) / s.v[1]);
        let eq164_e1839_d_n0: f64 = ((s.v[1] - ((nv0 - nv18) * s.dn[1][0])) / (s.v[1] * s.v[1]));
        let eq164_e1839_d_n1: f64 = (-(((nv0 - nv18) * s.dn[1][1]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n2: f64 = (-(((nv0 - nv18) * s.dn[1][2]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n3: f64 = (-(((nv0 - nv18) * s.dn[1][3]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n4: f64 = (-(((nv0 - nv18) * s.dn[1][4]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n5: f64 = (-(((nv0 - nv18) * s.dn[1][5]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n6: f64 = (-(((nv0 - nv18) * s.dn[1][6]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n7: f64 = (-(((nv0 - nv18) * s.dn[1][7]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n8: f64 = (-(((nv0 - nv18) * s.dn[1][8]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n9: f64 = (-(((nv0 - nv18) * s.dn[1][9]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n10: f64 = (-(((nv0 - nv18) * s.dn[1][10]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n11: f64 = (-(((nv0 - nv18) * s.dn[1][11]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n12: f64 = (-(((nv0 - nv18) * s.dn[1][12]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n13: f64 = (-(((nv0 - nv18) * s.dn[1][13]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n14: f64 = (-(((nv0 - nv18) * s.dn[1][14]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n15: f64 = (-(((nv0 - nv18) * s.dn[1][15]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n16: f64 = (-(((nv0 - nv18) * s.dn[1][16]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n17: f64 = (-(((nv0 - nv18) * s.dn[1][17]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n18: f64 = (((-s.v[1]) - ((nv0 - nv18) * s.dn[1][18])) / (s.v[1] * s.v[1]));
        let eq164_e1839_d_n19: f64 = (-(((nv0 - nv18) * s.dn[1][19]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n20: f64 = (-(((nv0 - nv18) * s.dn[1][20]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n21: f64 = (-(((nv0 - nv18) * s.dn[1][21]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n22: f64 = (-(((nv0 - nv18) * s.dn[1][22]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n23: f64 = (-(((nv0 - nv18) * s.dn[1][23]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n24: f64 = (-(((nv0 - nv18) * s.dn[1][24]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n25: f64 = (-(((nv0 - nv18) * s.dn[1][25]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n26: f64 = (-(((nv0 - nv18) * s.dn[1][26]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n27: f64 = (-(((nv0 - nv18) * s.dn[1][27]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n28: f64 = (-(((nv0 - nv18) * s.dn[1][28]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n29: f64 = (-(((nv0 - nv18) * s.dn[1][29]) / (s.v[1] * s.v[1])));
        (eq164_e1839, eq164_e1839_d_n0, eq164_e1839_d_n1, eq164_e1839_d_n2, eq164_e1839_d_n3, eq164_e1839_d_n4, eq164_e1839_d_n5, eq164_e1839_d_n6, eq164_e1839_d_n7, eq164_e1839_d_n8, eq164_e1839_d_n9, eq164_e1839_d_n10, eq164_e1839_d_n11, eq164_e1839_d_n12, eq164_e1839_d_n13, eq164_e1839_d_n14, eq164_e1839_d_n15, eq164_e1839_d_n16, eq164_e1839_d_n17, eq164_e1839_d_n18, eq164_e1839_d_n19, eq164_e1839_d_n20, eq164_e1839_d_n21, eq164_e1839_d_n22, eq164_e1839_d_n23, eq164_e1839_d_n24, eq164_e1839_d_n25, eq164_e1839_d_n26, eq164_e1839_d_n27, eq164_e1839_d_n28, eq164_e1839_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_value: f64 = eq164_e1841;
        let eq164_node_derivatives: [f64; 30] = [eq164_e1841_d_n0, eq164_e1841_d_n1, eq164_e1841_d_n2, eq164_e1841_d_n3, eq164_e1841_d_n4, eq164_e1841_d_n5, eq164_e1841_d_n6, eq164_e1841_d_n7, eq164_e1841_d_n8, eq164_e1841_d_n9, eq164_e1841_d_n10, eq164_e1841_d_n11, eq164_e1841_d_n12, eq164_e1841_d_n13, eq164_e1841_d_n14, eq164_e1841_d_n15, eq164_e1841_d_n16, eq164_e1841_d_n17, eq164_e1841_d_n18, eq164_e1841_d_n19, eq164_e1841_d_n20, eq164_e1841_d_n21, eq164_e1841_d_n22, eq164_e1841_d_n23, eq164_e1841_d_n24, eq164_e1841_d_n25, eq164_e1841_d_n26, eq164_e1841_d_n27, eq164_e1841_d_n28, eq164_e1841_d_n29];
        let eq164_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[18]),
            multiplicity * (eq164_value),
            nodes,
            &eq164_node_derivatives,
            branches,
            &eq164_branch_derivatives,
            multiplicity,
        );
        let (eq165_e1846,) = {
    if (!s.b[2670]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq165_value: f64 = eq165_e1846;
        stamper.stamp_potential_const(
            branches[31],
            eq165_value,
        );
        let (eq166_e1852, eq166_e1852_d_n0, eq166_e1852_d_n1, eq166_e1852_d_n2, eq166_e1852_d_n3, eq166_e1852_d_n4, eq166_e1852_d_n5, eq166_e1852_d_n6, eq166_e1852_d_n7, eq166_e1852_d_n8, eq166_e1852_d_n9, eq166_e1852_d_n10, eq166_e1852_d_n11, eq166_e1852_d_n12, eq166_e1852_d_n13, eq166_e1852_d_n14, eq166_e1852_d_n15, eq166_e1852_d_n16, eq166_e1852_d_n17, eq166_e1852_d_n18, eq166_e1852_d_n19, eq166_e1852_d_n20, eq166_e1852_d_n21, eq166_e1852_d_n22, eq166_e1852_d_n23, eq166_e1852_d_n24, eq166_e1852_d_n25, eq166_e1852_d_n26, eq166_e1852_d_n27, eq166_e1852_d_n28, eq166_e1852_d_n29,) = {
    if s.b[2671] {
        let eq166_e1850: f64 = ((nv19 - nv2) / s.v[2]);
        let eq166_e1850_d_n0: f64 = (-(((nv19 - nv2) * s.dn[2][0]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n1: f64 = (-(((nv19 - nv2) * s.dn[2][1]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n2: f64 = (((-s.v[2]) - ((nv19 - nv2) * s.dn[2][2])) / (s.v[2] * s.v[2]));
        let eq166_e1850_d_n3: f64 = (-(((nv19 - nv2) * s.dn[2][3]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n4: f64 = (-(((nv19 - nv2) * s.dn[2][4]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n5: f64 = (-(((nv19 - nv2) * s.dn[2][5]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n6: f64 = (-(((nv19 - nv2) * s.dn[2][6]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n7: f64 = (-(((nv19 - nv2) * s.dn[2][7]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n8: f64 = (-(((nv19 - nv2) * s.dn[2][8]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n9: f64 = (-(((nv19 - nv2) * s.dn[2][9]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n10: f64 = (-(((nv19 - nv2) * s.dn[2][10]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n11: f64 = (-(((nv19 - nv2) * s.dn[2][11]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n12: f64 = (-(((nv19 - nv2) * s.dn[2][12]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n13: f64 = (-(((nv19 - nv2) * s.dn[2][13]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n14: f64 = (-(((nv19 - nv2) * s.dn[2][14]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n15: f64 = (-(((nv19 - nv2) * s.dn[2][15]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n16: f64 = (-(((nv19 - nv2) * s.dn[2][16]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n17: f64 = (-(((nv19 - nv2) * s.dn[2][17]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n18: f64 = (-(((nv19 - nv2) * s.dn[2][18]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n19: f64 = ((s.v[2] - ((nv19 - nv2) * s.dn[2][19])) / (s.v[2] * s.v[2]));
        let eq166_e1850_d_n20: f64 = (-(((nv19 - nv2) * s.dn[2][20]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n21: f64 = (-(((nv19 - nv2) * s.dn[2][21]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n22: f64 = (-(((nv19 - nv2) * s.dn[2][22]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n23: f64 = (-(((nv19 - nv2) * s.dn[2][23]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n24: f64 = (-(((nv19 - nv2) * s.dn[2][24]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n25: f64 = (-(((nv19 - nv2) * s.dn[2][25]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n26: f64 = (-(((nv19 - nv2) * s.dn[2][26]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n27: f64 = (-(((nv19 - nv2) * s.dn[2][27]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n28: f64 = (-(((nv19 - nv2) * s.dn[2][28]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n29: f64 = (-(((nv19 - nv2) * s.dn[2][29]) / (s.v[2] * s.v[2])));
        (eq166_e1850, eq166_e1850_d_n0, eq166_e1850_d_n1, eq166_e1850_d_n2, eq166_e1850_d_n3, eq166_e1850_d_n4, eq166_e1850_d_n5, eq166_e1850_d_n6, eq166_e1850_d_n7, eq166_e1850_d_n8, eq166_e1850_d_n9, eq166_e1850_d_n10, eq166_e1850_d_n11, eq166_e1850_d_n12, eq166_e1850_d_n13, eq166_e1850_d_n14, eq166_e1850_d_n15, eq166_e1850_d_n16, eq166_e1850_d_n17, eq166_e1850_d_n18, eq166_e1850_d_n19, eq166_e1850_d_n20, eq166_e1850_d_n21, eq166_e1850_d_n22, eq166_e1850_d_n23, eq166_e1850_d_n24, eq166_e1850_d_n25, eq166_e1850_d_n26, eq166_e1850_d_n27, eq166_e1850_d_n28, eq166_e1850_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_value: f64 = eq166_e1852;
        let eq166_node_derivatives: [f64; 30] = [eq166_e1852_d_n0, eq166_e1852_d_n1, eq166_e1852_d_n2, eq166_e1852_d_n3, eq166_e1852_d_n4, eq166_e1852_d_n5, eq166_e1852_d_n6, eq166_e1852_d_n7, eq166_e1852_d_n8, eq166_e1852_d_n9, eq166_e1852_d_n10, eq166_e1852_d_n11, eq166_e1852_d_n12, eq166_e1852_d_n13, eq166_e1852_d_n14, eq166_e1852_d_n15, eq166_e1852_d_n16, eq166_e1852_d_n17, eq166_e1852_d_n18, eq166_e1852_d_n19, eq166_e1852_d_n20, eq166_e1852_d_n21, eq166_e1852_d_n22, eq166_e1852_d_n23, eq166_e1852_d_n24, eq166_e1852_d_n25, eq166_e1852_d_n26, eq166_e1852_d_n27, eq166_e1852_d_n28, eq166_e1852_d_n29];
        let eq166_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[19]),
            Some(nodes[2]),
            multiplicity * (eq166_value),
            nodes,
            &eq166_node_derivatives,
            branches,
            &eq166_branch_derivatives,
            multiplicity,
        );
        let (eq167_e1857,) = {
    if (!s.b[2671]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq167_value: f64 = eq167_e1857;
        stamper.stamp_potential_const(
            branches[32],
            eq167_value,
        );
        let (eq168_e1863, eq168_e1863_d_n1, eq168_e1863_d_n6,) = {
    if s.b[2672] {
        let eq168_e1861: f64 = ((nv1 - nv6) / s.v[5]);
        let eq168_e1861_d_n1: f64 = (1.0 / s.v[5]);
        let eq168_e1861_d_n6: f64 = (-1.0 / s.v[5]);
        (eq168_e1861, eq168_e1861_d_n1, eq168_e1861_d_n6,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq168_value: f64 = eq168_e1863;
        stamper.stamp_current_node2(
            Some(nodes[1]),
            Some(nodes[6]),
            multiplicity * (eq168_value),
            nodes[1],
            multiplicity * (eq168_e1863_d_n1),
            nodes[6],
            multiplicity * (eq168_e1863_d_n6),
        );
        let (eq169_e1868,) = {
    if (!s.b[2672]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq169_value: f64 = eq169_e1868;
        stamper.stamp_potential_const(
            branches[33],
            eq169_value,
        );
        let (eq170_e1874, eq170_e1874_d_n6, eq170_e1874_d_n7,) = {
    if s.b[2673] {
        let eq170_e1872: f64 = ((nv6 - nv7) / s.v[6]);
        let eq170_e1872_d_n6: f64 = (1.0 / s.v[6]);
        let eq170_e1872_d_n7: f64 = (-1.0 / s.v[6]);
        (eq170_e1872, eq170_e1872_d_n6, eq170_e1872_d_n7,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq170_value: f64 = eq170_e1874;
        stamper.stamp_current_node2(
            Some(nodes[6]),
            Some(nodes[7]),
            multiplicity * (eq170_value),
            nodes[6],
            multiplicity * (eq170_e1874_d_n6),
            nodes[7],
            multiplicity * (eq170_e1874_d_n7),
        );
        let (eq171_e1879,) = {
    if (!s.b[2673]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq171_value: f64 = eq171_e1879;
        stamper.stamp_potential_const(
            branches[34],
            eq171_value,
        );
        let eq172_e1881: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 139, s.v[214]);
        let eq172_e1881_d_n0: f64 = (s.dn[214][0] * ddt_scale);
        let eq172_e1881_d_n1: f64 = (s.dn[214][1] * ddt_scale);
        let eq172_e1881_d_n2: f64 = (s.dn[214][2] * ddt_scale);
        let eq172_e1881_d_n3: f64 = (s.dn[214][3] * ddt_scale);
        let eq172_e1881_d_n4: f64 = (s.dn[214][4] * ddt_scale);
        let eq172_e1881_d_n5: f64 = (s.dn[214][5] * ddt_scale);
        let eq172_e1881_d_n6: f64 = (s.dn[214][6] * ddt_scale);
        let eq172_e1881_d_n7: f64 = (s.dn[214][7] * ddt_scale);
        let eq172_e1881_d_n8: f64 = (s.dn[214][8] * ddt_scale);
        let eq172_e1881_d_n9: f64 = (s.dn[214][9] * ddt_scale);
        let eq172_e1881_d_n10: f64 = (s.dn[214][10] * ddt_scale);
        let eq172_e1881_d_n11: f64 = (s.dn[214][11] * ddt_scale);
        let eq172_e1881_d_n12: f64 = (s.dn[214][12] * ddt_scale);
        let eq172_e1881_d_n13: f64 = (s.dn[214][13] * ddt_scale);
        let eq172_e1881_d_n14: f64 = (s.dn[214][14] * ddt_scale);
        let eq172_e1881_d_n15: f64 = (s.dn[214][15] * ddt_scale);
        let eq172_e1881_d_n16: f64 = (s.dn[214][16] * ddt_scale);
        let eq172_e1881_d_n17: f64 = (s.dn[214][17] * ddt_scale);
        let eq172_e1881_d_n18: f64 = (s.dn[214][18] * ddt_scale);
        let eq172_e1881_d_n19: f64 = (s.dn[214][19] * ddt_scale);
        let eq172_e1881_d_n20: f64 = (s.dn[214][20] * ddt_scale);
        let eq172_e1881_d_n21: f64 = (s.dn[214][21] * ddt_scale);
        let eq172_e1881_d_n22: f64 = (s.dn[214][22] * ddt_scale);
        let eq172_e1881_d_n23: f64 = (s.dn[214][23] * ddt_scale);
        let eq172_e1881_d_n24: f64 = (s.dn[214][24] * ddt_scale);
        let eq172_e1881_d_n25: f64 = (s.dn[214][25] * ddt_scale);
        let eq172_e1881_d_n26: f64 = (s.dn[214][26] * ddt_scale);
        let eq172_e1881_d_n27: f64 = (s.dn[214][27] * ddt_scale);
        let eq172_e1881_d_n28: f64 = (s.dn[214][28] * ddt_scale);
        let eq172_e1881_d_n29: f64 = (s.dn[214][29] * ddt_scale);
        let eq172_value: f64 = eq172_e1881;
        let eq172_node_derivatives: [f64; 30] = [eq172_e1881_d_n0, eq172_e1881_d_n1, eq172_e1881_d_n2, eq172_e1881_d_n3, eq172_e1881_d_n4, eq172_e1881_d_n5, eq172_e1881_d_n6, eq172_e1881_d_n7, eq172_e1881_d_n8, eq172_e1881_d_n9, eq172_e1881_d_n10, eq172_e1881_d_n11, eq172_e1881_d_n12, eq172_e1881_d_n13, eq172_e1881_d_n14, eq172_e1881_d_n15, eq172_e1881_d_n16, eq172_e1881_d_n17, eq172_e1881_d_n18, eq172_e1881_d_n19, eq172_e1881_d_n20, eq172_e1881_d_n21, eq172_e1881_d_n22, eq172_e1881_d_n23, eq172_e1881_d_n24, eq172_e1881_d_n25, eq172_e1881_d_n26, eq172_e1881_d_n27, eq172_e1881_d_n28, eq172_e1881_d_n29];
        let eq172_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            multiplicity * (eq172_value),
            nodes,
            &eq172_node_derivatives,
            branches,
            &eq172_branch_derivatives,
            multiplicity,
        );
        let eq173_e1883: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 140, s.v[215]);
        let eq173_e1883_d_n0: f64 = (s.dn[215][0] * ddt_scale);
        let eq173_e1883_d_n1: f64 = (s.dn[215][1] * ddt_scale);
        let eq173_e1883_d_n2: f64 = (s.dn[215][2] * ddt_scale);
        let eq173_e1883_d_n3: f64 = (s.dn[215][3] * ddt_scale);
        let eq173_e1883_d_n4: f64 = (s.dn[215][4] * ddt_scale);
        let eq173_e1883_d_n5: f64 = (s.dn[215][5] * ddt_scale);
        let eq173_e1883_d_n6: f64 = (s.dn[215][6] * ddt_scale);
        let eq173_e1883_d_n7: f64 = (s.dn[215][7] * ddt_scale);
        let eq173_e1883_d_n8: f64 = (s.dn[215][8] * ddt_scale);
        let eq173_e1883_d_n9: f64 = (s.dn[215][9] * ddt_scale);
        let eq173_e1883_d_n10: f64 = (s.dn[215][10] * ddt_scale);
        let eq173_e1883_d_n11: f64 = (s.dn[215][11] * ddt_scale);
        let eq173_e1883_d_n12: f64 = (s.dn[215][12] * ddt_scale);
        let eq173_e1883_d_n13: f64 = (s.dn[215][13] * ddt_scale);
        let eq173_e1883_d_n14: f64 = (s.dn[215][14] * ddt_scale);
        let eq173_e1883_d_n15: f64 = (s.dn[215][15] * ddt_scale);
        let eq173_e1883_d_n16: f64 = (s.dn[215][16] * ddt_scale);
        let eq173_e1883_d_n17: f64 = (s.dn[215][17] * ddt_scale);
        let eq173_e1883_d_n18: f64 = (s.dn[215][18] * ddt_scale);
        let eq173_e1883_d_n19: f64 = (s.dn[215][19] * ddt_scale);
        let eq173_e1883_d_n20: f64 = (s.dn[215][20] * ddt_scale);
        let eq173_e1883_d_n21: f64 = (s.dn[215][21] * ddt_scale);
        let eq173_e1883_d_n22: f64 = (s.dn[215][22] * ddt_scale);
        let eq173_e1883_d_n23: f64 = (s.dn[215][23] * ddt_scale);
        let eq173_e1883_d_n24: f64 = (s.dn[215][24] * ddt_scale);
        let eq173_e1883_d_n25: f64 = (s.dn[215][25] * ddt_scale);
        let eq173_e1883_d_n26: f64 = (s.dn[215][26] * ddt_scale);
        let eq173_e1883_d_n27: f64 = (s.dn[215][27] * ddt_scale);
        let eq173_e1883_d_n28: f64 = (s.dn[215][28] * ddt_scale);
        let eq173_e1883_d_n29: f64 = (s.dn[215][29] * ddt_scale);
        let eq173_value: f64 = eq173_e1883;
        let eq173_node_derivatives: [f64; 30] = [eq173_e1883_d_n0, eq173_e1883_d_n1, eq173_e1883_d_n2, eq173_e1883_d_n3, eq173_e1883_d_n4, eq173_e1883_d_n5, eq173_e1883_d_n6, eq173_e1883_d_n7, eq173_e1883_d_n8, eq173_e1883_d_n9, eq173_e1883_d_n10, eq173_e1883_d_n11, eq173_e1883_d_n12, eq173_e1883_d_n13, eq173_e1883_d_n14, eq173_e1883_d_n15, eq173_e1883_d_n16, eq173_e1883_d_n17, eq173_e1883_d_n18, eq173_e1883_d_n19, eq173_e1883_d_n20, eq173_e1883_d_n21, eq173_e1883_d_n22, eq173_e1883_d_n23, eq173_e1883_d_n24, eq173_e1883_d_n25, eq173_e1883_d_n26, eq173_e1883_d_n27, eq173_e1883_d_n28, eq173_e1883_d_n29];
        let eq173_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[0]),
            multiplicity * (eq173_value),
            nodes,
            &eq173_node_derivatives,
            branches,
            &eq173_branch_derivatives,
            multiplicity,
        );
        let eq174_e1885: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 141, s.v[216]);
        let eq174_e1885_d_n0: f64 = (s.dn[216][0] * ddt_scale);
        let eq174_e1885_d_n1: f64 = (s.dn[216][1] * ddt_scale);
        let eq174_e1885_d_n2: f64 = (s.dn[216][2] * ddt_scale);
        let eq174_e1885_d_n3: f64 = (s.dn[216][3] * ddt_scale);
        let eq174_e1885_d_n4: f64 = (s.dn[216][4] * ddt_scale);
        let eq174_e1885_d_n5: f64 = (s.dn[216][5] * ddt_scale);
        let eq174_e1885_d_n6: f64 = (s.dn[216][6] * ddt_scale);
        let eq174_e1885_d_n7: f64 = (s.dn[216][7] * ddt_scale);
        let eq174_e1885_d_n8: f64 = (s.dn[216][8] * ddt_scale);
        let eq174_e1885_d_n9: f64 = (s.dn[216][9] * ddt_scale);
        let eq174_e1885_d_n10: f64 = (s.dn[216][10] * ddt_scale);
        let eq174_e1885_d_n11: f64 = (s.dn[216][11] * ddt_scale);
        let eq174_e1885_d_n12: f64 = (s.dn[216][12] * ddt_scale);
        let eq174_e1885_d_n13: f64 = (s.dn[216][13] * ddt_scale);
        let eq174_e1885_d_n14: f64 = (s.dn[216][14] * ddt_scale);
        let eq174_e1885_d_n15: f64 = (s.dn[216][15] * ddt_scale);
        let eq174_e1885_d_n16: f64 = (s.dn[216][16] * ddt_scale);
        let eq174_e1885_d_n17: f64 = (s.dn[216][17] * ddt_scale);
        let eq174_e1885_d_n18: f64 = (s.dn[216][18] * ddt_scale);
        let eq174_e1885_d_n19: f64 = (s.dn[216][19] * ddt_scale);
        let eq174_e1885_d_n20: f64 = (s.dn[216][20] * ddt_scale);
        let eq174_e1885_d_n21: f64 = (s.dn[216][21] * ddt_scale);
        let eq174_e1885_d_n22: f64 = (s.dn[216][22] * ddt_scale);
        let eq174_e1885_d_n23: f64 = (s.dn[216][23] * ddt_scale);
        let eq174_e1885_d_n24: f64 = (s.dn[216][24] * ddt_scale);
        let eq174_e1885_d_n25: f64 = (s.dn[216][25] * ddt_scale);
        let eq174_e1885_d_n26: f64 = (s.dn[216][26] * ddt_scale);
        let eq174_e1885_d_n27: f64 = (s.dn[216][27] * ddt_scale);
        let eq174_e1885_d_n28: f64 = (s.dn[216][28] * ddt_scale);
        let eq174_e1885_d_n29: f64 = (s.dn[216][29] * ddt_scale);
        let eq174_value: f64 = eq174_e1885;
        let eq174_node_derivatives: [f64; 30] = [eq174_e1885_d_n0, eq174_e1885_d_n1, eq174_e1885_d_n2, eq174_e1885_d_n3, eq174_e1885_d_n4, eq174_e1885_d_n5, eq174_e1885_d_n6, eq174_e1885_d_n7, eq174_e1885_d_n8, eq174_e1885_d_n9, eq174_e1885_d_n10, eq174_e1885_d_n11, eq174_e1885_d_n12, eq174_e1885_d_n13, eq174_e1885_d_n14, eq174_e1885_d_n15, eq174_e1885_d_n16, eq174_e1885_d_n17, eq174_e1885_d_n18, eq174_e1885_d_n19, eq174_e1885_d_n20, eq174_e1885_d_n21, eq174_e1885_d_n22, eq174_e1885_d_n23, eq174_e1885_d_n24, eq174_e1885_d_n25, eq174_e1885_d_n26, eq174_e1885_d_n27, eq174_e1885_d_n28, eq174_e1885_d_n29];
        let eq174_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            multiplicity * (eq174_value),
            nodes,
            &eq174_node_derivatives,
            branches,
            &eq174_branch_derivatives,
            multiplicity,
        );
        let eq175_e1887: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 142, s.v[218]);
        let eq175_e1887_d_n0: f64 = (s.dn[218][0] * ddt_scale);
        let eq175_e1887_d_n1: f64 = (s.dn[218][1] * ddt_scale);
        let eq175_e1887_d_n2: f64 = (s.dn[218][2] * ddt_scale);
        let eq175_e1887_d_n3: f64 = (s.dn[218][3] * ddt_scale);
        let eq175_e1887_d_n4: f64 = (s.dn[218][4] * ddt_scale);
        let eq175_e1887_d_n5: f64 = (s.dn[218][5] * ddt_scale);
        let eq175_e1887_d_n6: f64 = (s.dn[218][6] * ddt_scale);
        let eq175_e1887_d_n7: f64 = (s.dn[218][7] * ddt_scale);
        let eq175_e1887_d_n8: f64 = (s.dn[218][8] * ddt_scale);
        let eq175_e1887_d_n9: f64 = (s.dn[218][9] * ddt_scale);
        let eq175_e1887_d_n10: f64 = (s.dn[218][10] * ddt_scale);
        let eq175_e1887_d_n11: f64 = (s.dn[218][11] * ddt_scale);
        let eq175_e1887_d_n12: f64 = (s.dn[218][12] * ddt_scale);
        let eq175_e1887_d_n13: f64 = (s.dn[218][13] * ddt_scale);
        let eq175_e1887_d_n14: f64 = (s.dn[218][14] * ddt_scale);
        let eq175_e1887_d_n15: f64 = (s.dn[218][15] * ddt_scale);
        let eq175_e1887_d_n16: f64 = (s.dn[218][16] * ddt_scale);
        let eq175_e1887_d_n17: f64 = (s.dn[218][17] * ddt_scale);
        let eq175_e1887_d_n18: f64 = (s.dn[218][18] * ddt_scale);
        let eq175_e1887_d_n19: f64 = (s.dn[218][19] * ddt_scale);
        let eq175_e1887_d_n20: f64 = (s.dn[218][20] * ddt_scale);
        let eq175_e1887_d_n21: f64 = (s.dn[218][21] * ddt_scale);
        let eq175_e1887_d_n22: f64 = (s.dn[218][22] * ddt_scale);
        let eq175_e1887_d_n23: f64 = (s.dn[218][23] * ddt_scale);
        let eq175_e1887_d_n24: f64 = (s.dn[218][24] * ddt_scale);
        let eq175_e1887_d_n25: f64 = (s.dn[218][25] * ddt_scale);
        let eq175_e1887_d_n26: f64 = (s.dn[218][26] * ddt_scale);
        let eq175_e1887_d_n27: f64 = (s.dn[218][27] * ddt_scale);
        let eq175_e1887_d_n28: f64 = (s.dn[218][28] * ddt_scale);
        let eq175_e1887_d_n29: f64 = (s.dn[218][29] * ddt_scale);
        let eq175_value: f64 = eq175_e1887;
        let eq175_node_derivatives: [f64; 30] = [eq175_e1887_d_n0, eq175_e1887_d_n1, eq175_e1887_d_n2, eq175_e1887_d_n3, eq175_e1887_d_n4, eq175_e1887_d_n5, eq175_e1887_d_n6, eq175_e1887_d_n7, eq175_e1887_d_n8, eq175_e1887_d_n9, eq175_e1887_d_n10, eq175_e1887_d_n11, eq175_e1887_d_n12, eq175_e1887_d_n13, eq175_e1887_d_n14, eq175_e1887_d_n15, eq175_e1887_d_n16, eq175_e1887_d_n17, eq175_e1887_d_n18, eq175_e1887_d_n19, eq175_e1887_d_n20, eq175_e1887_d_n21, eq175_e1887_d_n22, eq175_e1887_d_n23, eq175_e1887_d_n24, eq175_e1887_d_n25, eq175_e1887_d_n26, eq175_e1887_d_n27, eq175_e1887_d_n28, eq175_e1887_d_n29];
        let eq175_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[2]),
            multiplicity * (eq175_value),
            nodes,
            &eq175_node_derivatives,
            branches,
            &eq175_branch_derivatives,
            multiplicity,
        );
        let eq176_e1889: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 143, s.v[217]);
        let eq176_e1889_d_n0: f64 = (s.dn[217][0] * ddt_scale);
        let eq176_e1889_d_n1: f64 = (s.dn[217][1] * ddt_scale);
        let eq176_e1889_d_n2: f64 = (s.dn[217][2] * ddt_scale);
        let eq176_e1889_d_n3: f64 = (s.dn[217][3] * ddt_scale);
        let eq176_e1889_d_n4: f64 = (s.dn[217][4] * ddt_scale);
        let eq176_e1889_d_n5: f64 = (s.dn[217][5] * ddt_scale);
        let eq176_e1889_d_n6: f64 = (s.dn[217][6] * ddt_scale);
        let eq176_e1889_d_n7: f64 = (s.dn[217][7] * ddt_scale);
        let eq176_e1889_d_n8: f64 = (s.dn[217][8] * ddt_scale);
        let eq176_e1889_d_n9: f64 = (s.dn[217][9] * ddt_scale);
        let eq176_e1889_d_n10: f64 = (s.dn[217][10] * ddt_scale);
        let eq176_e1889_d_n11: f64 = (s.dn[217][11] * ddt_scale);
        let eq176_e1889_d_n12: f64 = (s.dn[217][12] * ddt_scale);
        let eq176_e1889_d_n13: f64 = (s.dn[217][13] * ddt_scale);
        let eq176_e1889_d_n14: f64 = (s.dn[217][14] * ddt_scale);
        let eq176_e1889_d_n15: f64 = (s.dn[217][15] * ddt_scale);
        let eq176_e1889_d_n16: f64 = (s.dn[217][16] * ddt_scale);
        let eq176_e1889_d_n17: f64 = (s.dn[217][17] * ddt_scale);
        let eq176_e1889_d_n18: f64 = (s.dn[217][18] * ddt_scale);
        let eq176_e1889_d_n19: f64 = (s.dn[217][19] * ddt_scale);
        let eq176_e1889_d_n20: f64 = (s.dn[217][20] * ddt_scale);
        let eq176_e1889_d_n21: f64 = (s.dn[217][21] * ddt_scale);
        let eq176_e1889_d_n22: f64 = (s.dn[217][22] * ddt_scale);
        let eq176_e1889_d_n23: f64 = (s.dn[217][23] * ddt_scale);
        let eq176_e1889_d_n24: f64 = (s.dn[217][24] * ddt_scale);
        let eq176_e1889_d_n25: f64 = (s.dn[217][25] * ddt_scale);
        let eq176_e1889_d_n26: f64 = (s.dn[217][26] * ddt_scale);
        let eq176_e1889_d_n27: f64 = (s.dn[217][27] * ddt_scale);
        let eq176_e1889_d_n28: f64 = (s.dn[217][28] * ddt_scale);
        let eq176_e1889_d_n29: f64 = (s.dn[217][29] * ddt_scale);
        let eq176_value: f64 = eq176_e1889;
        let eq176_node_derivatives: [f64; 30] = [eq176_e1889_d_n0, eq176_e1889_d_n1, eq176_e1889_d_n2, eq176_e1889_d_n3, eq176_e1889_d_n4, eq176_e1889_d_n5, eq176_e1889_d_n6, eq176_e1889_d_n7, eq176_e1889_d_n8, eq176_e1889_d_n9, eq176_e1889_d_n10, eq176_e1889_d_n11, eq176_e1889_d_n12, eq176_e1889_d_n13, eq176_e1889_d_n14, eq176_e1889_d_n15, eq176_e1889_d_n16, eq176_e1889_d_n17, eq176_e1889_d_n18, eq176_e1889_d_n19, eq176_e1889_d_n20, eq176_e1889_d_n21, eq176_e1889_d_n22, eq176_e1889_d_n23, eq176_e1889_d_n24, eq176_e1889_d_n25, eq176_e1889_d_n26, eq176_e1889_d_n27, eq176_e1889_d_n28, eq176_e1889_d_n29];
        let eq176_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            multiplicity * (eq176_value),
            nodes,
            &eq176_node_derivatives,
            branches,
            &eq176_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_14(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq177_e1891: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 144, s.v[219]);
        let eq177_e1891_d_n0: f64 = (s.dn[219][0] * ddt_scale);
        let eq177_e1891_d_n1: f64 = (s.dn[219][1] * ddt_scale);
        let eq177_e1891_d_n2: f64 = (s.dn[219][2] * ddt_scale);
        let eq177_e1891_d_n3: f64 = (s.dn[219][3] * ddt_scale);
        let eq177_e1891_d_n4: f64 = (s.dn[219][4] * ddt_scale);
        let eq177_e1891_d_n5: f64 = (s.dn[219][5] * ddt_scale);
        let eq177_e1891_d_n6: f64 = (s.dn[219][6] * ddt_scale);
        let eq177_e1891_d_n7: f64 = (s.dn[219][7] * ddt_scale);
        let eq177_e1891_d_n8: f64 = (s.dn[219][8] * ddt_scale);
        let eq177_e1891_d_n9: f64 = (s.dn[219][9] * ddt_scale);
        let eq177_e1891_d_n10: f64 = (s.dn[219][10] * ddt_scale);
        let eq177_e1891_d_n11: f64 = (s.dn[219][11] * ddt_scale);
        let eq177_e1891_d_n12: f64 = (s.dn[219][12] * ddt_scale);
        let eq177_e1891_d_n13: f64 = (s.dn[219][13] * ddt_scale);
        let eq177_e1891_d_n14: f64 = (s.dn[219][14] * ddt_scale);
        let eq177_e1891_d_n15: f64 = (s.dn[219][15] * ddt_scale);
        let eq177_e1891_d_n16: f64 = (s.dn[219][16] * ddt_scale);
        let eq177_e1891_d_n17: f64 = (s.dn[219][17] * ddt_scale);
        let eq177_e1891_d_n18: f64 = (s.dn[219][18] * ddt_scale);
        let eq177_e1891_d_n19: f64 = (s.dn[219][19] * ddt_scale);
        let eq177_e1891_d_n20: f64 = (s.dn[219][20] * ddt_scale);
        let eq177_e1891_d_n21: f64 = (s.dn[219][21] * ddt_scale);
        let eq177_e1891_d_n22: f64 = (s.dn[219][22] * ddt_scale);
        let eq177_e1891_d_n23: f64 = (s.dn[219][23] * ddt_scale);
        let eq177_e1891_d_n24: f64 = (s.dn[219][24] * ddt_scale);
        let eq177_e1891_d_n25: f64 = (s.dn[219][25] * ddt_scale);
        let eq177_e1891_d_n26: f64 = (s.dn[219][26] * ddt_scale);
        let eq177_e1891_d_n27: f64 = (s.dn[219][27] * ddt_scale);
        let eq177_e1891_d_n28: f64 = (s.dn[219][28] * ddt_scale);
        let eq177_e1891_d_n29: f64 = (s.dn[219][29] * ddt_scale);
        let eq177_value: f64 = eq177_e1891;
        let eq177_node_derivatives: [f64; 30] = [eq177_e1891_d_n0, eq177_e1891_d_n1, eq177_e1891_d_n2, eq177_e1891_d_n3, eq177_e1891_d_n4, eq177_e1891_d_n5, eq177_e1891_d_n6, eq177_e1891_d_n7, eq177_e1891_d_n8, eq177_e1891_d_n9, eq177_e1891_d_n10, eq177_e1891_d_n11, eq177_e1891_d_n12, eq177_e1891_d_n13, eq177_e1891_d_n14, eq177_e1891_d_n15, eq177_e1891_d_n16, eq177_e1891_d_n17, eq177_e1891_d_n18, eq177_e1891_d_n19, eq177_e1891_d_n20, eq177_e1891_d_n21, eq177_e1891_d_n22, eq177_e1891_d_n23, eq177_e1891_d_n24, eq177_e1891_d_n25, eq177_e1891_d_n26, eq177_e1891_d_n27, eq177_e1891_d_n28, eq177_e1891_d_n29];
        let eq177_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            multiplicity * (eq177_value),
            nodes,
            &eq177_node_derivatives,
            branches,
            &eq177_branch_derivatives,
            multiplicity,
        );
        let (eq178_e1908,) = {
    if s.b[2686] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq178_value: f64 = eq178_e1908;
        stamper.stamp_current_const(
            Some(nodes[8]),
            Some(nodes[9]),
            multiplicity * (eq178_value),
        );
        let (eq179_e1925,) = {
    if s.b[2686] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq179_value: f64 = eq179_e1925;
        stamper.stamp_current_const(
            Some(nodes[8]),
            Some(nodes[5]),
            multiplicity * (eq179_value),
        );
        let (eq180_e1942,) = {
    if s.b[2686] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq180_value: f64 = eq180_e1942;
        stamper.stamp_current_const(
            Some(nodes[8]),
            Some(nodes[13]),
            multiplicity * (eq180_value),
        );
        let (eq181_e1959,) = {
    if s.b[2686] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq181_value: f64 = eq181_e1959;
        stamper.stamp_current_const(
            Some(nodes[8]),
            Some(nodes[17]),
            multiplicity * (eq181_value),
        );
        let (eq182_e1966,) = {
    if s.b[2686] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq182_value: f64 = eq182_e1966;
        stamper.stamp_current_const(
            Some(nodes[5]),
            Some(nodes[9]),
            multiplicity * (eq182_value),
        );
        let (eq183_e1972,) = {
    if s.b[2686] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq183_value: f64 = eq183_e1972;
        stamper.stamp_current_const(
            Some(nodes[5]),
            Some(nodes[9]),
            multiplicity * (eq183_value),
        );
        let (eq184_e1992,) = {
    if (s.b[2686] && s.b[2688]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq184_value: f64 = eq184_e1992;
        stamper.stamp_current_const(
            Some(nodes[9]),
            Some(nodes[10]),
            multiplicity * (eq184_value),
        );
        let (eq185_e2012,) = {
    if (s.b[2686] && s.b[2689]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq185_value: f64 = eq185_e2012;
        stamper.stamp_current_const(
            Some(nodes[10]),
            Some(nodes[11]),
            multiplicity * (eq185_value),
        );
        let (eq186_e2032,) = {
    if (s.b[2686] && s.b[2690]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq186_value: f64 = eq186_e2032;
        stamper.stamp_current_const(
            Some(nodes[11]),
            Some(nodes[12]),
            multiplicity * (eq186_value),
        );
        let (eq187_e2052,) = {
    if (s.b[2686] && s.b[2691]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq187_value: f64 = eq187_e2052;
        stamper.stamp_current_const(
            Some(nodes[12]),
            Some(nodes[13]),
            multiplicity * (eq187_value),
        );
        let (eq188_e2072,) = {
    if (s.b[2686] && s.b[2692]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq188_value: f64 = eq188_e2072;
        stamper.stamp_current_const(
            Some(nodes[14]),
            Some(nodes[5]),
            multiplicity * (eq188_value),
        );
        let (eq189_e2092,) = {
    if (s.b[2686] && s.b[2693]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq189_value: f64 = eq189_e2092;
        stamper.stamp_current_const(
            Some(nodes[15]),
            Some(nodes[14]),
            multiplicity * (eq189_value),
        );
        let (eq190_e2112,) = {
    if (s.b[2686] && s.b[2694]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq190_value: f64 = eq190_e2112;
        stamper.stamp_current_const(
            Some(nodes[16]),
            Some(nodes[15]),
            multiplicity * (eq190_value),
        );
        let (eq191_e2132,) = {
    if (s.b[2686] && s.b[2695]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq191_value: f64 = eq191_e2132;
        stamper.stamp_current_const(
            Some(nodes[17]),
            Some(nodes[16]),
            multiplicity * (eq191_value),
        );
        let (eq192_e2146,) = {
    if (s.b[2686] && s.b[2696]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq192_value: f64 = eq192_e2146;
        stamper.stamp_current_const(
            Some(nodes[19]),
            Some(nodes[2]),
            multiplicity * (eq192_value),
        );
        let (eq193_e2160,) = {
    if (s.b[2686] && s.b[2697]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq193_value: f64 = eq193_e2160;
        stamper.stamp_current_const(
            Some(nodes[0]),
            Some(nodes[18]),
            multiplicity * (eq193_value),
        );
        let (eq194_e2167, eq194_e2167_d_n4,) = {
    if s.b[2700] {
        let eq194_e2164: f64 = (p.p321 * (nv4 - 0.0));
        let eq194_e2164_d_n4: f64 = p.p321;
        let eq194_e2165: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 145, eq194_e2164);
        let eq194_e2165_d_n4: f64 = (eq194_e2164_d_n4 * ddt_scale);
        (eq194_e2165, eq194_e2165_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq194_value: f64 = eq194_e2167;
        stamper.stamp_current_node1(
            Some(nodes[4]),
            None,
            multiplicity * (eq194_value),
            nodes[4],
            multiplicity * (eq194_e2167_d_n4),
        );
        let (eq195_e2172, eq195_e2172_d_n0, eq195_e2172_d_n1, eq195_e2172_d_n2, eq195_e2172_d_n3, eq195_e2172_d_n4, eq195_e2172_d_n5, eq195_e2172_d_n6, eq195_e2172_d_n7, eq195_e2172_d_n8, eq195_e2172_d_n9, eq195_e2172_d_n10, eq195_e2172_d_n11, eq195_e2172_d_n12, eq195_e2172_d_n13, eq195_e2172_d_n14, eq195_e2172_d_n15, eq195_e2172_d_n16, eq195_e2172_d_n17, eq195_e2172_d_n18, eq195_e2172_d_n19, eq195_e2172_d_n20, eq195_e2172_d_n21, eq195_e2172_d_n22, eq195_e2172_d_n23, eq195_e2172_d_n24, eq195_e2172_d_n25, eq195_e2172_d_n26, eq195_e2172_d_n27, eq195_e2172_d_n28, eq195_e2172_d_n29,) = {
    if s.b[2700] {
        let eq195_e2170: f64 = (-s.v[114]);
        let eq195_e2170_d_n0: f64 = (-s.dn[114][0]);
        let eq195_e2170_d_n1: f64 = (-s.dn[114][1]);
        let eq195_e2170_d_n2: f64 = (-s.dn[114][2]);
        let eq195_e2170_d_n3: f64 = (-s.dn[114][3]);
        let eq195_e2170_d_n4: f64 = (-s.dn[114][4]);
        let eq195_e2170_d_n5: f64 = (-s.dn[114][5]);
        let eq195_e2170_d_n6: f64 = (-s.dn[114][6]);
        let eq195_e2170_d_n7: f64 = (-s.dn[114][7]);
        let eq195_e2170_d_n8: f64 = (-s.dn[114][8]);
        let eq195_e2170_d_n9: f64 = (-s.dn[114][9]);
        let eq195_e2170_d_n10: f64 = (-s.dn[114][10]);
        let eq195_e2170_d_n11: f64 = (-s.dn[114][11]);
        let eq195_e2170_d_n12: f64 = (-s.dn[114][12]);
        let eq195_e2170_d_n13: f64 = (-s.dn[114][13]);
        let eq195_e2170_d_n14: f64 = (-s.dn[114][14]);
        let eq195_e2170_d_n15: f64 = (-s.dn[114][15]);
        let eq195_e2170_d_n16: f64 = (-s.dn[114][16]);
        let eq195_e2170_d_n17: f64 = (-s.dn[114][17]);
        let eq195_e2170_d_n18: f64 = (-s.dn[114][18]);
        let eq195_e2170_d_n19: f64 = (-s.dn[114][19]);
        let eq195_e2170_d_n20: f64 = (-s.dn[114][20]);
        let eq195_e2170_d_n21: f64 = (-s.dn[114][21]);
        let eq195_e2170_d_n22: f64 = (-s.dn[114][22]);
        let eq195_e2170_d_n23: f64 = (-s.dn[114][23]);
        let eq195_e2170_d_n24: f64 = (-s.dn[114][24]);
        let eq195_e2170_d_n25: f64 = (-s.dn[114][25]);
        let eq195_e2170_d_n26: f64 = (-s.dn[114][26]);
        let eq195_e2170_d_n27: f64 = (-s.dn[114][27]);
        let eq195_e2170_d_n28: f64 = (-s.dn[114][28]);
        let eq195_e2170_d_n29: f64 = (-s.dn[114][29]);
        (eq195_e2170, eq195_e2170_d_n0, eq195_e2170_d_n1, eq195_e2170_d_n2, eq195_e2170_d_n3, eq195_e2170_d_n4, eq195_e2170_d_n5, eq195_e2170_d_n6, eq195_e2170_d_n7, eq195_e2170_d_n8, eq195_e2170_d_n9, eq195_e2170_d_n10, eq195_e2170_d_n11, eq195_e2170_d_n12, eq195_e2170_d_n13, eq195_e2170_d_n14, eq195_e2170_d_n15, eq195_e2170_d_n16, eq195_e2170_d_n17, eq195_e2170_d_n18, eq195_e2170_d_n19, eq195_e2170_d_n20, eq195_e2170_d_n21, eq195_e2170_d_n22, eq195_e2170_d_n23, eq195_e2170_d_n24, eq195_e2170_d_n25, eq195_e2170_d_n26, eq195_e2170_d_n27, eq195_e2170_d_n28, eq195_e2170_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq195_value: f64 = eq195_e2172;
        let eq195_node_derivatives: [f64; 30] = [eq195_e2172_d_n0, eq195_e2172_d_n1, eq195_e2172_d_n2, eq195_e2172_d_n3, eq195_e2172_d_n4, eq195_e2172_d_n5, eq195_e2172_d_n6, eq195_e2172_d_n7, eq195_e2172_d_n8, eq195_e2172_d_n9, eq195_e2172_d_n10, eq195_e2172_d_n11, eq195_e2172_d_n12, eq195_e2172_d_n13, eq195_e2172_d_n14, eq195_e2172_d_n15, eq195_e2172_d_n16, eq195_e2172_d_n17, eq195_e2172_d_n18, eq195_e2172_d_n19, eq195_e2172_d_n20, eq195_e2172_d_n21, eq195_e2172_d_n22, eq195_e2172_d_n23, eq195_e2172_d_n24, eq195_e2172_d_n25, eq195_e2172_d_n26, eq195_e2172_d_n27, eq195_e2172_d_n28, eq195_e2172_d_n29];
        let eq195_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            multiplicity * (eq195_value),
            nodes,
            &eq195_node_derivatives,
            branches,
            &eq195_branch_derivatives,
            multiplicity,
        );
        let (eq196_e2178, eq196_e2178_d_n4,) = {
    if s.b[2700] {
        let eq196_e2176: f64 = ((nv4 - 0.0) / p.p320);
        let eq196_e2176_d_n4: f64 = (1.0 / p.p320);
        (eq196_e2176, eq196_e2176_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq196_value: f64 = eq196_e2178;
        stamper.stamp_current_node1(
            Some(nodes[4]),
            None,
            multiplicity * (eq196_value),
            nodes[4],
            multiplicity * (eq196_e2178_d_n4),
        );
        let (eq197_e2183,) = {
    if (!s.b[2700]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq197_value: f64 = eq197_e2183;
        stamper.stamp_potential_const(
            branches[35],
            eq197_value,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let (eq8_e421, eq8_e421_d_n20, eq8_e421_d_n21, eq8_e421_q, eq8_e421_q_d_n20, eq8_e421_q_d_n21,) = {
    if s.b[308] {
        let eq8_e418: f64 = (p.p330 * (nv21 - nv20));
        let eq8_e418_d_n20: f64 = (-p.p330);
        let eq8_e418_d_n21: f64 = p.p330;
        let eq8_e419_q: f64 = eq8_e418;
        (eq8_e418, eq8_e418_d_n20, eq8_e418_d_n21, eq8_e419_q, eq8_e418_d_n20, eq8_e418_d_n21,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[21]),
            Some(nodes[20]),
            nodes[20],
            multiplicity * (eq8_e421_q_d_n20),
            nodes[21],
            multiplicity * (eq8_e421_q_d_n21),
        );
        let (eq9_e428, eq9_e428_d_n20, eq9_e428_q, eq9_e428_q_d_n20,) = {
    if s.b[308] {
        let eq9_e425: f64 = (p.p332 * (nv20 - 0.0));
        let eq9_e425_d_n20: f64 = p.p332;
        let eq9_e426_q: f64 = eq9_e425;
        (eq9_e425, eq9_e425_d_n20, eq9_e426_q, eq9_e425_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[20]),
            None,
            nodes[20],
            multiplicity * (eq9_e428_q_d_n20),
        );
        let (eq17_e564, eq17_e564_d_n0, eq17_e564_d_n1, eq17_e564_d_n2, eq17_e564_d_n3, eq17_e564_d_n4, eq17_e564_d_n5, eq17_e564_d_n6, eq17_e564_d_n7, eq17_e564_d_n8, eq17_e564_d_n9, eq17_e564_d_n10, eq17_e564_d_n11, eq17_e564_d_n12, eq17_e564_d_n13, eq17_e564_d_n14, eq17_e564_d_n15, eq17_e564_d_n16, eq17_e564_d_n17, eq17_e564_d_n18, eq17_e564_d_n19, eq17_e564_d_n20, eq17_e564_d_n21, eq17_e564_d_n22, eq17_e564_d_n23, eq17_e564_d_n24, eq17_e564_d_n25, eq17_e564_d_n26, eq17_e564_d_n27, eq17_e564_d_n28, eq17_e564_d_n29, eq17_e564_q, eq17_e564_q_d_n0, eq17_e564_q_d_n1, eq17_e564_q_d_n2, eq17_e564_q_d_n3, eq17_e564_q_d_n4, eq17_e564_q_d_n5, eq17_e564_q_d_n6, eq17_e564_q_d_n7, eq17_e564_q_d_n8, eq17_e564_q_d_n9, eq17_e564_q_d_n10, eq17_e564_q_d_n11, eq17_e564_q_d_n12, eq17_e564_q_d_n13, eq17_e564_q_d_n14, eq17_e564_q_d_n15, eq17_e564_q_d_n16, eq17_e564_q_d_n17, eq17_e564_q_d_n18, eq17_e564_q_d_n19, eq17_e564_q_d_n20, eq17_e564_q_d_n21, eq17_e564_q_d_n22, eq17_e564_q_d_n23, eq17_e564_q_d_n24, eq17_e564_q_d_n25, eq17_e564_q_d_n26, eq17_e564_q_d_n27, eq17_e564_q_d_n28, eq17_e564_q_d_n29,) = {
    if ((!s.b[308]) && s.b[309]) {
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
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq22_e682, eq22_e682_d_n0, eq22_e682_d_n1, eq22_e682_d_n2, eq22_e682_d_n3, eq22_e682_d_n4, eq22_e682_d_n5, eq22_e682_d_n6, eq22_e682_d_n7, eq22_e682_d_n8, eq22_e682_d_n9, eq22_e682_d_n10, eq22_e682_d_n11, eq22_e682_d_n12, eq22_e682_d_n13, eq22_e682_d_n14, eq22_e682_d_n15, eq22_e682_d_n16, eq22_e682_d_n17, eq22_e682_d_n18, eq22_e682_d_n19, eq22_e682_d_n20, eq22_e682_d_n21, eq22_e682_d_n22, eq22_e682_d_n23, eq22_e682_d_n24, eq22_e682_d_n25, eq22_e682_d_n26, eq22_e682_d_n27, eq22_e682_d_n28, eq22_e682_d_n29, eq22_e682_q, eq22_e682_q_d_n0, eq22_e682_q_d_n1, eq22_e682_q_d_n2, eq22_e682_q_d_n3, eq22_e682_q_d_n4, eq22_e682_q_d_n5, eq22_e682_q_d_n6, eq22_e682_q_d_n7, eq22_e682_q_d_n8, eq22_e682_q_d_n9, eq22_e682_q_d_n10, eq22_e682_q_d_n11, eq22_e682_q_d_n12, eq22_e682_q_d_n13, eq22_e682_q_d_n14, eq22_e682_q_d_n15, eq22_e682_q_d_n16, eq22_e682_q_d_n17, eq22_e682_q_d_n18, eq22_e682_q_d_n19, eq22_e682_q_d_n20, eq22_e682_q_d_n21, eq22_e682_q_d_n22, eq22_e682_q_d_n23, eq22_e682_q_d_n24, eq22_e682_q_d_n25, eq22_e682_q_d_n26, eq22_e682_q_d_n27, eq22_e682_q_d_n28, eq22_e682_q_d_n29,) = {
    if ((!s.b[308]) && s.b[309]) {
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
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq33_e769, eq33_e769_d_n0, eq33_e769_d_n1, eq33_e769_d_n2, eq33_e769_d_n3, eq33_e769_d_n4, eq33_e769_d_n5, eq33_e769_d_n6, eq33_e769_d_n7, eq33_e769_d_n8, eq33_e769_d_n9, eq33_e769_d_n10, eq33_e769_d_n11, eq33_e769_d_n12, eq33_e769_d_n13, eq33_e769_d_n14, eq33_e769_d_n15, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_d_n18, eq33_e769_d_n19, eq33_e769_d_n20, eq33_e769_d_n21, eq33_e769_d_n22, eq33_e769_d_n23, eq33_e769_d_n24, eq33_e769_d_n25, eq33_e769_d_n26, eq33_e769_d_n27, eq33_e769_d_n28, eq33_e769_d_n29, eq33_e769_q, eq33_e769_q_d_n0, eq33_e769_q_d_n1, eq33_e769_q_d_n2, eq33_e769_q_d_n3, eq33_e769_q_d_n4, eq33_e769_q_d_n5, eq33_e769_q_d_n6, eq33_e769_q_d_n7, eq33_e769_q_d_n8, eq33_e769_q_d_n9, eq33_e769_q_d_n10, eq33_e769_q_d_n11, eq33_e769_q_d_n12, eq33_e769_q_d_n13, eq33_e769_q_d_n14, eq33_e769_q_d_n15, eq33_e769_q_d_n16, eq33_e769_q_d_n17, eq33_e769_q_d_n18, eq33_e769_q_d_n19, eq33_e769_q_d_n20, eq33_e769_q_d_n21, eq33_e769_q_d_n22, eq33_e769_q_d_n23, eq33_e769_q_d_n24, eq33_e769_q_d_n25, eq33_e769_q_d_n26, eq33_e769_q_d_n27, eq33_e769_q_d_n28, eq33_e769_q_d_n29,) = {
    if s.b[466] {
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
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq34_e779, eq34_e779_d_n0, eq34_e779_d_n1, eq34_e779_d_n2, eq34_e779_d_n3, eq34_e779_d_n4, eq34_e779_d_n5, eq34_e779_d_n6, eq34_e779_d_n7, eq34_e779_d_n8, eq34_e779_d_n9, eq34_e779_d_n10, eq34_e779_d_n11, eq34_e779_d_n12, eq34_e779_d_n13, eq34_e779_d_n14, eq34_e779_d_n15, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_d_n18, eq34_e779_d_n19, eq34_e779_d_n20, eq34_e779_d_n21, eq34_e779_d_n22, eq34_e779_d_n23, eq34_e779_d_n24, eq34_e779_d_n25, eq34_e779_d_n26, eq34_e779_d_n27, eq34_e779_d_n28, eq34_e779_d_n29, eq34_e779_q, eq34_e779_q_d_n0, eq34_e779_q_d_n1, eq34_e779_q_d_n2, eq34_e779_q_d_n3, eq34_e779_q_d_n4, eq34_e779_q_d_n5, eq34_e779_q_d_n6, eq34_e779_q_d_n7, eq34_e779_q_d_n8, eq34_e779_q_d_n9, eq34_e779_q_d_n10, eq34_e779_q_d_n11, eq34_e779_q_d_n12, eq34_e779_q_d_n13, eq34_e779_q_d_n14, eq34_e779_q_d_n15, eq34_e779_q_d_n16, eq34_e779_q_d_n17, eq34_e779_q_d_n18, eq34_e779_q_d_n19, eq34_e779_q_d_n20, eq34_e779_q_d_n21, eq34_e779_q_d_n22, eq34_e779_q_d_n23, eq34_e779_q_d_n24, eq34_e779_q_d_n25, eq34_e779_q_d_n26, eq34_e779_q_d_n27, eq34_e779_q_d_n28, eq34_e779_q_d_n29,) = {
    if s.b[466] {
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
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq35_e789, eq35_e789_d_n0, eq35_e789_d_n1, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n5, eq35_e789_d_n6, eq35_e789_d_n7, eq35_e789_d_n8, eq35_e789_d_n9, eq35_e789_d_n10, eq35_e789_d_n11, eq35_e789_d_n12, eq35_e789_d_n13, eq35_e789_d_n14, eq35_e789_d_n15, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_d_n18, eq35_e789_d_n19, eq35_e789_d_n20, eq35_e789_d_n21, eq35_e789_d_n22, eq35_e789_d_n23, eq35_e789_d_n24, eq35_e789_d_n25, eq35_e789_d_n26, eq35_e789_d_n27, eq35_e789_d_n28, eq35_e789_d_n29, eq35_e789_q, eq35_e789_q_d_n0, eq35_e789_q_d_n1, eq35_e789_q_d_n2, eq35_e789_q_d_n3, eq35_e789_q_d_n4, eq35_e789_q_d_n5, eq35_e789_q_d_n6, eq35_e789_q_d_n7, eq35_e789_q_d_n8, eq35_e789_q_d_n9, eq35_e789_q_d_n10, eq35_e789_q_d_n11, eq35_e789_q_d_n12, eq35_e789_q_d_n13, eq35_e789_q_d_n14, eq35_e789_q_d_n15, eq35_e789_q_d_n16, eq35_e789_q_d_n17, eq35_e789_q_d_n18, eq35_e789_q_d_n19, eq35_e789_q_d_n20, eq35_e789_q_d_n21, eq35_e789_q_d_n22, eq35_e789_q_d_n23, eq35_e789_q_d_n24, eq35_e789_q_d_n25, eq35_e789_q_d_n26, eq35_e789_q_d_n27, eq35_e789_q_d_n28, eq35_e789_q_d_n29,) = {
    if s.b[466] {
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
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq37_e803, eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29, eq37_e803_q, eq37_e803_q_d_n0, eq37_e803_q_d_n1, eq37_e803_q_d_n2, eq37_e803_q_d_n3, eq37_e803_q_d_n4, eq37_e803_q_d_n5, eq37_e803_q_d_n6, eq37_e803_q_d_n7, eq37_e803_q_d_n8, eq37_e803_q_d_n9, eq37_e803_q_d_n10, eq37_e803_q_d_n11, eq37_e803_q_d_n12, eq37_e803_q_d_n13, eq37_e803_q_d_n14, eq37_e803_q_d_n15, eq37_e803_q_d_n16, eq37_e803_q_d_n17, eq37_e803_q_d_n18, eq37_e803_q_d_n19, eq37_e803_q_d_n20, eq37_e803_q_d_n21, eq37_e803_q_d_n22, eq37_e803_q_d_n23, eq37_e803_q_d_n24, eq37_e803_q_d_n25, eq37_e803_q_d_n26, eq37_e803_q_d_n27, eq37_e803_q_d_n28, eq37_e803_q_d_n29,) = {
    if s.b[466] {
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
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq38_e814, eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29, eq38_e814_q, eq38_e814_q_d_n0, eq38_e814_q_d_n1, eq38_e814_q_d_n2, eq38_e814_q_d_n3, eq38_e814_q_d_n4, eq38_e814_q_d_n5, eq38_e814_q_d_n6, eq38_e814_q_d_n7, eq38_e814_q_d_n8, eq38_e814_q_d_n9, eq38_e814_q_d_n10, eq38_e814_q_d_n11, eq38_e814_q_d_n12, eq38_e814_q_d_n13, eq38_e814_q_d_n14, eq38_e814_q_d_n15, eq38_e814_q_d_n16, eq38_e814_q_d_n17, eq38_e814_q_d_n18, eq38_e814_q_d_n19, eq38_e814_q_d_n20, eq38_e814_q_d_n21, eq38_e814_q_d_n22, eq38_e814_q_d_n23, eq38_e814_q_d_n24, eq38_e814_q_d_n25, eq38_e814_q_d_n26, eq38_e814_q_d_n27, eq38_e814_q_d_n28, eq38_e814_q_d_n29,) = {
    if (!s.b[466]) {
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
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e825, eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29, eq39_e825_q, eq39_e825_q_d_n0, eq39_e825_q_d_n1, eq39_e825_q_d_n2, eq39_e825_q_d_n3, eq39_e825_q_d_n4, eq39_e825_q_d_n5, eq39_e825_q_d_n6, eq39_e825_q_d_n7, eq39_e825_q_d_n8, eq39_e825_q_d_n9, eq39_e825_q_d_n10, eq39_e825_q_d_n11, eq39_e825_q_d_n12, eq39_e825_q_d_n13, eq39_e825_q_d_n14, eq39_e825_q_d_n15, eq39_e825_q_d_n16, eq39_e825_q_d_n17, eq39_e825_q_d_n18, eq39_e825_q_d_n19, eq39_e825_q_d_n20, eq39_e825_q_d_n21, eq39_e825_q_d_n22, eq39_e825_q_d_n23, eq39_e825_q_d_n24, eq39_e825_q_d_n25, eq39_e825_q_d_n26, eq39_e825_q_d_n27, eq39_e825_q_d_n28, eq39_e825_q_d_n29,) = {
    if (!s.b[466]) {
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
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e836, eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29, eq40_e836_q, eq40_e836_q_d_n0, eq40_e836_q_d_n1, eq40_e836_q_d_n2, eq40_e836_q_d_n3, eq40_e836_q_d_n4, eq40_e836_q_d_n5, eq40_e836_q_d_n6, eq40_e836_q_d_n7, eq40_e836_q_d_n8, eq40_e836_q_d_n9, eq40_e836_q_d_n10, eq40_e836_q_d_n11, eq40_e836_q_d_n12, eq40_e836_q_d_n13, eq40_e836_q_d_n14, eq40_e836_q_d_n15, eq40_e836_q_d_n16, eq40_e836_q_d_n17, eq40_e836_q_d_n18, eq40_e836_q_d_n19, eq40_e836_q_d_n20, eq40_e836_q_d_n21, eq40_e836_q_d_n22, eq40_e836_q_d_n23, eq40_e836_q_d_n24, eq40_e836_q_d_n25, eq40_e836_q_d_n26, eq40_e836_q_d_n27, eq40_e836_q_d_n28, eq40_e836_q_d_n29,) = {
    if (!s.b[466]) {
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
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
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
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq46_e876, eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29, eq46_e876_q, eq46_e876_q_d_n0, eq46_e876_q_d_n1, eq46_e876_q_d_n2, eq46_e876_q_d_n3, eq46_e876_q_d_n4, eq46_e876_q_d_n5, eq46_e876_q_d_n6, eq46_e876_q_d_n7, eq46_e876_q_d_n8, eq46_e876_q_d_n9, eq46_e876_q_d_n10, eq46_e876_q_d_n11, eq46_e876_q_d_n12, eq46_e876_q_d_n13, eq46_e876_q_d_n14, eq46_e876_q_d_n15, eq46_e876_q_d_n16, eq46_e876_q_d_n17, eq46_e876_q_d_n18, eq46_e876_q_d_n19, eq46_e876_q_d_n20, eq46_e876_q_d_n21, eq46_e876_q_d_n22, eq46_e876_q_d_n23, eq46_e876_q_d_n24, eq46_e876_q_d_n25, eq46_e876_q_d_n26, eq46_e876_q_d_n27, eq46_e876_q_d_n28, eq46_e876_q_d_n29,) = {
    if s.b[613] {
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
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e886, eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29, eq47_e886_q, eq47_e886_q_d_n0, eq47_e886_q_d_n1, eq47_e886_q_d_n2, eq47_e886_q_d_n3, eq47_e886_q_d_n4, eq47_e886_q_d_n5, eq47_e886_q_d_n6, eq47_e886_q_d_n7, eq47_e886_q_d_n8, eq47_e886_q_d_n9, eq47_e886_q_d_n10, eq47_e886_q_d_n11, eq47_e886_q_d_n12, eq47_e886_q_d_n13, eq47_e886_q_d_n14, eq47_e886_q_d_n15, eq47_e886_q_d_n16, eq47_e886_q_d_n17, eq47_e886_q_d_n18, eq47_e886_q_d_n19, eq47_e886_q_d_n20, eq47_e886_q_d_n21, eq47_e886_q_d_n22, eq47_e886_q_d_n23, eq47_e886_q_d_n24, eq47_e886_q_d_n25, eq47_e886_q_d_n26, eq47_e886_q_d_n27, eq47_e886_q_d_n28, eq47_e886_q_d_n29,) = {
    if s.b[613] {
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
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq48_e896, eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29, eq48_e896_q, eq48_e896_q_d_n0, eq48_e896_q_d_n1, eq48_e896_q_d_n2, eq48_e896_q_d_n3, eq48_e896_q_d_n4, eq48_e896_q_d_n5, eq48_e896_q_d_n6, eq48_e896_q_d_n7, eq48_e896_q_d_n8, eq48_e896_q_d_n9, eq48_e896_q_d_n10, eq48_e896_q_d_n11, eq48_e896_q_d_n12, eq48_e896_q_d_n13, eq48_e896_q_d_n14, eq48_e896_q_d_n15, eq48_e896_q_d_n16, eq48_e896_q_d_n17, eq48_e896_q_d_n18, eq48_e896_q_d_n19, eq48_e896_q_d_n20, eq48_e896_q_d_n21, eq48_e896_q_d_n22, eq48_e896_q_d_n23, eq48_e896_q_d_n24, eq48_e896_q_d_n25, eq48_e896_q_d_n26, eq48_e896_q_d_n27, eq48_e896_q_d_n28, eq48_e896_q_d_n29,) = {
    if s.b[613] {
        let eq48_e889_q: f64 = s.v[205];
        let eq48_e892: f64 = (p.p355 * (nv2 - nv15));
        let eq48_e892_d_n2: f64 = p.p355;
        let eq48_e892_d_n15: f64 = (-p.p355);
        let eq48_e893_q: f64 = eq48_e892;
        let eq48_e894: f64 = (s.v[205] + eq48_e892);
        let eq48_e894_d_n2: f64 = (s.dn[205][2] + eq48_e892_d_n2);
        let eq48_e894_d_n15: f64 = (s.dn[205][15] + eq48_e892_d_n15);
        let eq48_e894_q: f64 = (eq48_e889_q + eq48_e893_q);
        let eq48_e894_q_d_n2: f64 = (s.dn[205][2] + eq48_e892_d_n2);
        let eq48_e894_q_d_n15: f64 = (s.dn[205][15] + eq48_e892_d_n15);
        (eq48_e894, s.dn[205][0], s.dn[205][1], eq48_e894_d_n2, s.dn[205][3], s.dn[205][4], s.dn[205][5], s.dn[205][6], s.dn[205][7], s.dn[205][8], s.dn[205][9], s.dn[205][10], s.dn[205][11], s.dn[205][12], s.dn[205][13], s.dn[205][14], eq48_e894_d_n15, s.dn[205][16], s.dn[205][17], s.dn[205][18], s.dn[205][19], s.dn[205][20], s.dn[205][21], s.dn[205][22], s.dn[205][23], s.dn[205][24], s.dn[205][25], s.dn[205][26], s.dn[205][27], s.dn[205][28], s.dn[205][29], eq48_e894_q, s.dn[205][0], s.dn[205][1], eq48_e894_q_d_n2, s.dn[205][3], s.dn[205][4], s.dn[205][5], s.dn[205][6], s.dn[205][7], s.dn[205][8], s.dn[205][9], s.dn[205][10], s.dn[205][11], s.dn[205][12], s.dn[205][13], s.dn[205][14], eq48_e894_q_d_n15, s.dn[205][16], s.dn[205][17], s.dn[205][18], s.dn[205][19], s.dn[205][20], s.dn[205][21], s.dn[205][22], s.dn[205][23], s.dn[205][24], s.dn[205][25], s.dn[205][26], s.dn[205][27], s.dn[205][28], s.dn[205][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_reactive_node_derivatives: [f64; 30] = [eq48_e896_q_d_n0, eq48_e896_q_d_n1, eq48_e896_q_d_n2, eq48_e896_q_d_n3, eq48_e896_q_d_n4, eq48_e896_q_d_n5, eq48_e896_q_d_n6, eq48_e896_q_d_n7, eq48_e896_q_d_n8, eq48_e896_q_d_n9, eq48_e896_q_d_n10, eq48_e896_q_d_n11, eq48_e896_q_d_n12, eq48_e896_q_d_n13, eq48_e896_q_d_n14, eq48_e896_q_d_n15, eq48_e896_q_d_n16, eq48_e896_q_d_n17, eq48_e896_q_d_n18, eq48_e896_q_d_n19, eq48_e896_q_d_n20, eq48_e896_q_d_n21, eq48_e896_q_d_n22, eq48_e896_q_d_n23, eq48_e896_q_d_n24, eq48_e896_q_d_n25, eq48_e896_q_d_n26, eq48_e896_q_d_n27, eq48_e896_q_d_n28, eq48_e896_q_d_n29];
        let eq48_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq50_e910, eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29, eq50_e910_q, eq50_e910_q_d_n0, eq50_e910_q_d_n1, eq50_e910_q_d_n2, eq50_e910_q_d_n3, eq50_e910_q_d_n4, eq50_e910_q_d_n5, eq50_e910_q_d_n6, eq50_e910_q_d_n7, eq50_e910_q_d_n8, eq50_e910_q_d_n9, eq50_e910_q_d_n10, eq50_e910_q_d_n11, eq50_e910_q_d_n12, eq50_e910_q_d_n13, eq50_e910_q_d_n14, eq50_e910_q_d_n15, eq50_e910_q_d_n16, eq50_e910_q_d_n17, eq50_e910_q_d_n18, eq50_e910_q_d_n19, eq50_e910_q_d_n20, eq50_e910_q_d_n21, eq50_e910_q_d_n22, eq50_e910_q_d_n23, eq50_e910_q_d_n24, eq50_e910_q_d_n25, eq50_e910_q_d_n26, eq50_e910_q_d_n27, eq50_e910_q_d_n28, eq50_e910_q_d_n29,) = {
    if s.b[613] {
        let eq50_e903_q: f64 = s.v[207];
        let eq50_e906: f64 = (p.p355 * (nv7 - nv9));
        let eq50_e906_d_n7: f64 = p.p355;
        let eq50_e906_d_n9: f64 = (-p.p355);
        let eq50_e907_q: f64 = eq50_e906;
        let eq50_e908: f64 = (s.v[207] + eq50_e906);
        let eq50_e908_d_n7: f64 = (s.dn[207][7] + eq50_e906_d_n7);
        let eq50_e908_d_n9: f64 = (s.dn[207][9] + eq50_e906_d_n9);
        let eq50_e908_q: f64 = (eq50_e903_q + eq50_e907_q);
        let eq50_e908_q_d_n7: f64 = (s.dn[207][7] + eq50_e906_d_n7);
        let eq50_e908_q_d_n9: f64 = (s.dn[207][9] + eq50_e906_d_n9);
        (eq50_e908, s.dn[207][0], s.dn[207][1], s.dn[207][2], s.dn[207][3], s.dn[207][4], s.dn[207][5], s.dn[207][6], eq50_e908_d_n7, s.dn[207][8], eq50_e908_d_n9, s.dn[207][10], s.dn[207][11], s.dn[207][12], s.dn[207][13], s.dn[207][14], s.dn[207][15], s.dn[207][16], s.dn[207][17], s.dn[207][18], s.dn[207][19], s.dn[207][20], s.dn[207][21], s.dn[207][22], s.dn[207][23], s.dn[207][24], s.dn[207][25], s.dn[207][26], s.dn[207][27], s.dn[207][28], s.dn[207][29], eq50_e908_q, s.dn[207][0], s.dn[207][1], s.dn[207][2], s.dn[207][3], s.dn[207][4], s.dn[207][5], s.dn[207][6], eq50_e908_q_d_n7, s.dn[207][8], eq50_e908_q_d_n9, s.dn[207][10], s.dn[207][11], s.dn[207][12], s.dn[207][13], s.dn[207][14], s.dn[207][15], s.dn[207][16], s.dn[207][17], s.dn[207][18], s.dn[207][19], s.dn[207][20], s.dn[207][21], s.dn[207][22], s.dn[207][23], s.dn[207][24], s.dn[207][25], s.dn[207][26], s.dn[207][27], s.dn[207][28], s.dn[207][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 30] = [eq50_e910_q_d_n0, eq50_e910_q_d_n1, eq50_e910_q_d_n2, eq50_e910_q_d_n3, eq50_e910_q_d_n4, eq50_e910_q_d_n5, eq50_e910_q_d_n6, eq50_e910_q_d_n7, eq50_e910_q_d_n8, eq50_e910_q_d_n9, eq50_e910_q_d_n10, eq50_e910_q_d_n11, eq50_e910_q_d_n12, eq50_e910_q_d_n13, eq50_e910_q_d_n14, eq50_e910_q_d_n15, eq50_e910_q_d_n16, eq50_e910_q_d_n17, eq50_e910_q_d_n18, eq50_e910_q_d_n19, eq50_e910_q_d_n20, eq50_e910_q_d_n21, eq50_e910_q_d_n22, eq50_e910_q_d_n23, eq50_e910_q_d_n24, eq50_e910_q_d_n25, eq50_e910_q_d_n26, eq50_e910_q_d_n27, eq50_e910_q_d_n28, eq50_e910_q_d_n29];
        let eq50_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq51_e921, eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29, eq51_e921_q, eq51_e921_q_d_n0, eq51_e921_q_d_n1, eq51_e921_q_d_n2, eq51_e921_q_d_n3, eq51_e921_q_d_n4, eq51_e921_q_d_n5, eq51_e921_q_d_n6, eq51_e921_q_d_n7, eq51_e921_q_d_n8, eq51_e921_q_d_n9, eq51_e921_q_d_n10, eq51_e921_q_d_n11, eq51_e921_q_d_n12, eq51_e921_q_d_n13, eq51_e921_q_d_n14, eq51_e921_q_d_n15, eq51_e921_q_d_n16, eq51_e921_q_d_n17, eq51_e921_q_d_n18, eq51_e921_q_d_n19, eq51_e921_q_d_n20, eq51_e921_q_d_n21, eq51_e921_q_d_n22, eq51_e921_q_d_n23, eq51_e921_q_d_n24, eq51_e921_q_d_n25, eq51_e921_q_d_n26, eq51_e921_q_d_n27, eq51_e921_q_d_n28, eq51_e921_q_d_n29,) = {
    if (!s.b[613]) {
        let eq51_e914_q: f64 = s.v[203];
        let eq51_e917: f64 = (p.p355 * (nv2 - nv15));
        let eq51_e917_d_n2: f64 = p.p355;
        let eq51_e917_d_n15: f64 = (-p.p355);
        let eq51_e918_q: f64 = eq51_e917;
        let eq51_e919: f64 = (s.v[203] + eq51_e917);
        let eq51_e919_d_n2: f64 = (s.dn[203][2] + eq51_e917_d_n2);
        let eq51_e919_d_n15: f64 = (s.dn[203][15] + eq51_e917_d_n15);
        let eq51_e919_q: f64 = (eq51_e914_q + eq51_e918_q);
        let eq51_e919_q_d_n2: f64 = (s.dn[203][2] + eq51_e917_d_n2);
        let eq51_e919_q_d_n15: f64 = (s.dn[203][15] + eq51_e917_d_n15);
        (eq51_e919, s.dn[203][0], s.dn[203][1], eq51_e919_d_n2, s.dn[203][3], s.dn[203][4], s.dn[203][5], s.dn[203][6], s.dn[203][7], s.dn[203][8], s.dn[203][9], s.dn[203][10], s.dn[203][11], s.dn[203][12], s.dn[203][13], s.dn[203][14], eq51_e919_d_n15, s.dn[203][16], s.dn[203][17], s.dn[203][18], s.dn[203][19], s.dn[203][20], s.dn[203][21], s.dn[203][22], s.dn[203][23], s.dn[203][24], s.dn[203][25], s.dn[203][26], s.dn[203][27], s.dn[203][28], s.dn[203][29], eq51_e919_q, s.dn[203][0], s.dn[203][1], eq51_e919_q_d_n2, s.dn[203][3], s.dn[203][4], s.dn[203][5], s.dn[203][6], s.dn[203][7], s.dn[203][8], s.dn[203][9], s.dn[203][10], s.dn[203][11], s.dn[203][12], s.dn[203][13], s.dn[203][14], eq51_e919_q_d_n15, s.dn[203][16], s.dn[203][17], s.dn[203][18], s.dn[203][19], s.dn[203][20], s.dn[203][21], s.dn[203][22], s.dn[203][23], s.dn[203][24], s.dn[203][25], s.dn[203][26], s.dn[203][27], s.dn[203][28], s.dn[203][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 30] = [eq51_e921_q_d_n0, eq51_e921_q_d_n1, eq51_e921_q_d_n2, eq51_e921_q_d_n3, eq51_e921_q_d_n4, eq51_e921_q_d_n5, eq51_e921_q_d_n6, eq51_e921_q_d_n7, eq51_e921_q_d_n8, eq51_e921_q_d_n9, eq51_e921_q_d_n10, eq51_e921_q_d_n11, eq51_e921_q_d_n12, eq51_e921_q_d_n13, eq51_e921_q_d_n14, eq51_e921_q_d_n15, eq51_e921_q_d_n16, eq51_e921_q_d_n17, eq51_e921_q_d_n18, eq51_e921_q_d_n19, eq51_e921_q_d_n20, eq51_e921_q_d_n21, eq51_e921_q_d_n22, eq51_e921_q_d_n23, eq51_e921_q_d_n24, eq51_e921_q_d_n25, eq51_e921_q_d_n26, eq51_e921_q_d_n27, eq51_e921_q_d_n28, eq51_e921_q_d_n29];
        let eq51_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq52_e932, eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22, eq52_e932_d_n23, eq52_e932_d_n24, eq52_e932_d_n25, eq52_e932_d_n26, eq52_e932_d_n27, eq52_e932_d_n28, eq52_e932_d_n29, eq52_e932_q, eq52_e932_q_d_n0, eq52_e932_q_d_n1, eq52_e932_q_d_n2, eq52_e932_q_d_n3, eq52_e932_q_d_n4, eq52_e932_q_d_n5, eq52_e932_q_d_n6, eq52_e932_q_d_n7, eq52_e932_q_d_n8, eq52_e932_q_d_n9, eq52_e932_q_d_n10, eq52_e932_q_d_n11, eq52_e932_q_d_n12, eq52_e932_q_d_n13, eq52_e932_q_d_n14, eq52_e932_q_d_n15, eq52_e932_q_d_n16, eq52_e932_q_d_n17, eq52_e932_q_d_n18, eq52_e932_q_d_n19, eq52_e932_q_d_n20, eq52_e932_q_d_n21, eq52_e932_q_d_n22, eq52_e932_q_d_n23, eq52_e932_q_d_n24, eq52_e932_q_d_n25, eq52_e932_q_d_n26, eq52_e932_q_d_n27, eq52_e932_q_d_n28, eq52_e932_q_d_n29,) = {
    if (!s.b[613]) {
        let eq52_e925_q: f64 = s.v[204];
        let eq52_e928: f64 = (p.p355 * (nv2 - nv16));
        let eq52_e928_d_n2: f64 = p.p355;
        let eq52_e928_d_n16: f64 = (-p.p355);
        let eq52_e929_q: f64 = eq52_e928;
        let eq52_e930: f64 = (s.v[204] + eq52_e928);
        let eq52_e930_d_n2: f64 = (s.dn[204][2] + eq52_e928_d_n2);
        let eq52_e930_d_n16: f64 = (s.dn[204][16] + eq52_e928_d_n16);
        let eq52_e930_q: f64 = (eq52_e925_q + eq52_e929_q);
        let eq52_e930_q_d_n2: f64 = (s.dn[204][2] + eq52_e928_d_n2);
        let eq52_e930_q_d_n16: f64 = (s.dn[204][16] + eq52_e928_d_n16);
        (eq52_e930, s.dn[204][0], s.dn[204][1], eq52_e930_d_n2, s.dn[204][3], s.dn[204][4], s.dn[204][5], s.dn[204][6], s.dn[204][7], s.dn[204][8], s.dn[204][9], s.dn[204][10], s.dn[204][11], s.dn[204][12], s.dn[204][13], s.dn[204][14], s.dn[204][15], eq52_e930_d_n16, s.dn[204][17], s.dn[204][18], s.dn[204][19], s.dn[204][20], s.dn[204][21], s.dn[204][22], s.dn[204][23], s.dn[204][24], s.dn[204][25], s.dn[204][26], s.dn[204][27], s.dn[204][28], s.dn[204][29], eq52_e930_q, s.dn[204][0], s.dn[204][1], eq52_e930_q_d_n2, s.dn[204][3], s.dn[204][4], s.dn[204][5], s.dn[204][6], s.dn[204][7], s.dn[204][8], s.dn[204][9], s.dn[204][10], s.dn[204][11], s.dn[204][12], s.dn[204][13], s.dn[204][14], s.dn[204][15], eq52_e930_q_d_n16, s.dn[204][17], s.dn[204][18], s.dn[204][19], s.dn[204][20], s.dn[204][21], s.dn[204][22], s.dn[204][23], s.dn[204][24], s.dn[204][25], s.dn[204][26], s.dn[204][27], s.dn[204][28], s.dn[204][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 30] = [eq52_e932_q_d_n0, eq52_e932_q_d_n1, eq52_e932_q_d_n2, eq52_e932_q_d_n3, eq52_e932_q_d_n4, eq52_e932_q_d_n5, eq52_e932_q_d_n6, eq52_e932_q_d_n7, eq52_e932_q_d_n8, eq52_e932_q_d_n9, eq52_e932_q_d_n10, eq52_e932_q_d_n11, eq52_e932_q_d_n12, eq52_e932_q_d_n13, eq52_e932_q_d_n14, eq52_e932_q_d_n15, eq52_e932_q_d_n16, eq52_e932_q_d_n17, eq52_e932_q_d_n18, eq52_e932_q_d_n19, eq52_e932_q_d_n20, eq52_e932_q_d_n21, eq52_e932_q_d_n22, eq52_e932_q_d_n23, eq52_e932_q_d_n24, eq52_e932_q_d_n25, eq52_e932_q_d_n26, eq52_e932_q_d_n27, eq52_e932_q_d_n28, eq52_e932_q_d_n29];
        let eq52_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq53_e943, eq53_e943_d_n0, eq53_e943_d_n1, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n5, eq53_e943_d_n6, eq53_e943_d_n7, eq53_e943_d_n8, eq53_e943_d_n9, eq53_e943_d_n10, eq53_e943_d_n11, eq53_e943_d_n12, eq53_e943_d_n13, eq53_e943_d_n14, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_d_n17, eq53_e943_d_n18, eq53_e943_d_n19, eq53_e943_d_n20, eq53_e943_d_n21, eq53_e943_d_n22, eq53_e943_d_n23, eq53_e943_d_n24, eq53_e943_d_n25, eq53_e943_d_n26, eq53_e943_d_n27, eq53_e943_d_n28, eq53_e943_d_n29, eq53_e943_q, eq53_e943_q_d_n0, eq53_e943_q_d_n1, eq53_e943_q_d_n2, eq53_e943_q_d_n3, eq53_e943_q_d_n4, eq53_e943_q_d_n5, eq53_e943_q_d_n6, eq53_e943_q_d_n7, eq53_e943_q_d_n8, eq53_e943_q_d_n9, eq53_e943_q_d_n10, eq53_e943_q_d_n11, eq53_e943_q_d_n12, eq53_e943_q_d_n13, eq53_e943_q_d_n14, eq53_e943_q_d_n15, eq53_e943_q_d_n16, eq53_e943_q_d_n17, eq53_e943_q_d_n18, eq53_e943_q_d_n19, eq53_e943_q_d_n20, eq53_e943_q_d_n21, eq53_e943_q_d_n22, eq53_e943_q_d_n23, eq53_e943_q_d_n24, eq53_e943_q_d_n25, eq53_e943_q_d_n26, eq53_e943_q_d_n27, eq53_e943_q_d_n28, eq53_e943_q_d_n29,) = {
    if (!s.b[613]) {
        let eq53_e936_q: f64 = s.v[205];
        let eq53_e939: f64 = (p.p355 * (nv7 - nv15));
        let eq53_e939_d_n7: f64 = p.p355;
        let eq53_e939_d_n15: f64 = (-p.p355);
        let eq53_e940_q: f64 = eq53_e939;
        let eq53_e941: f64 = (s.v[205] + eq53_e939);
        let eq53_e941_d_n7: f64 = (s.dn[205][7] + eq53_e939_d_n7);
        let eq53_e941_d_n15: f64 = (s.dn[205][15] + eq53_e939_d_n15);
        let eq53_e941_q: f64 = (eq53_e936_q + eq53_e940_q);
        let eq53_e941_q_d_n7: f64 = (s.dn[205][7] + eq53_e939_d_n7);
        let eq53_e941_q_d_n15: f64 = (s.dn[205][15] + eq53_e939_d_n15);
        (eq53_e941, s.dn[205][0], s.dn[205][1], s.dn[205][2], s.dn[205][3], s.dn[205][4], s.dn[205][5], s.dn[205][6], eq53_e941_d_n7, s.dn[205][8], s.dn[205][9], s.dn[205][10], s.dn[205][11], s.dn[205][12], s.dn[205][13], s.dn[205][14], eq53_e941_d_n15, s.dn[205][16], s.dn[205][17], s.dn[205][18], s.dn[205][19], s.dn[205][20], s.dn[205][21], s.dn[205][22], s.dn[205][23], s.dn[205][24], s.dn[205][25], s.dn[205][26], s.dn[205][27], s.dn[205][28], s.dn[205][29], eq53_e941_q, s.dn[205][0], s.dn[205][1], s.dn[205][2], s.dn[205][3], s.dn[205][4], s.dn[205][5], s.dn[205][6], eq53_e941_q_d_n7, s.dn[205][8], s.dn[205][9], s.dn[205][10], s.dn[205][11], s.dn[205][12], s.dn[205][13], s.dn[205][14], eq53_e941_q_d_n15, s.dn[205][16], s.dn[205][17], s.dn[205][18], s.dn[205][19], s.dn[205][20], s.dn[205][21], s.dn[205][22], s.dn[205][23], s.dn[205][24], s.dn[205][25], s.dn[205][26], s.dn[205][27], s.dn[205][28], s.dn[205][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 30] = [eq53_e943_q_d_n0, eq53_e943_q_d_n1, eq53_e943_q_d_n2, eq53_e943_q_d_n3, eq53_e943_q_d_n4, eq53_e943_q_d_n5, eq53_e943_q_d_n6, eq53_e943_q_d_n7, eq53_e943_q_d_n8, eq53_e943_q_d_n9, eq53_e943_q_d_n10, eq53_e943_q_d_n11, eq53_e943_q_d_n12, eq53_e943_q_d_n13, eq53_e943_q_d_n14, eq53_e943_q_d_n15, eq53_e943_q_d_n16, eq53_e943_q_d_n17, eq53_e943_q_d_n18, eq53_e943_q_d_n19, eq53_e943_q_d_n20, eq53_e943_q_d_n21, eq53_e943_q_d_n22, eq53_e943_q_d_n23, eq53_e943_q_d_n24, eq53_e943_q_d_n25, eq53_e943_q_d_n26, eq53_e943_q_d_n27, eq53_e943_q_d_n28, eq53_e943_q_d_n29];
        let eq53_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
        let eq56_e955_q: f64 = s.v[206];
        let eq56_e958: f64 = (p.p355 * (nv3 - nv15));
        let eq56_e958_d_n3: f64 = p.p355;
        let eq56_e958_d_n15: f64 = (-p.p355);
        let eq56_e959_q: f64 = eq56_e958;
        let eq56_e960: f64 = (s.v[206] + eq56_e958);
        let eq56_e960_d_n3: f64 = (s.dn[206][3] + eq56_e958_d_n3);
        let eq56_e960_d_n15: f64 = (s.dn[206][15] + eq56_e958_d_n15);
        let eq56_e960_q: f64 = (eq56_e955_q + eq56_e959_q);
        let eq56_e960_q_d_n3: f64 = (s.dn[206][3] + eq56_e958_d_n3);
        let eq56_e960_q_d_n15: f64 = (s.dn[206][15] + eq56_e958_d_n15);
        let eq56_reactive_node_derivatives: [f64; 30] = [s.dn[206][0], s.dn[206][1], s.dn[206][2], eq56_e960_q_d_n3, s.dn[206][4], s.dn[206][5], s.dn[206][6], s.dn[206][7], s.dn[206][8], s.dn[206][9], s.dn[206][10], s.dn[206][11], s.dn[206][12], s.dn[206][13], s.dn[206][14], eq56_e960_q_d_n15, s.dn[206][16], s.dn[206][17], s.dn[206][18], s.dn[206][19], s.dn[206][20], s.dn[206][21], s.dn[206][22], s.dn[206][23], s.dn[206][24], s.dn[206][25], s.dn[206][26], s.dn[206][27], s.dn[206][28], s.dn[206][29]];
        let eq56_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[15]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq59_e983, eq59_e983_d_n0, eq59_e983_d_n1, eq59_e983_d_n2, eq59_e983_d_n3, eq59_e983_d_n4, eq59_e983_d_n5, eq59_e983_d_n6, eq59_e983_d_n7, eq59_e983_d_n8, eq59_e983_d_n9, eq59_e983_d_n10, eq59_e983_d_n11, eq59_e983_d_n12, eq59_e983_d_n13, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_d_n16, eq59_e983_d_n17, eq59_e983_d_n18, eq59_e983_d_n19, eq59_e983_d_n20, eq59_e983_d_n21, eq59_e983_d_n22, eq59_e983_d_n23, eq59_e983_d_n24, eq59_e983_d_n25, eq59_e983_d_n26, eq59_e983_d_n27, eq59_e983_d_n28, eq59_e983_d_n29, eq59_e983_q, eq59_e983_q_d_n0, eq59_e983_q_d_n1, eq59_e983_q_d_n2, eq59_e983_q_d_n3, eq59_e983_q_d_n4, eq59_e983_q_d_n5, eq59_e983_q_d_n6, eq59_e983_q_d_n7, eq59_e983_q_d_n8, eq59_e983_q_d_n9, eq59_e983_q_d_n10, eq59_e983_q_d_n11, eq59_e983_q_d_n12, eq59_e983_q_d_n13, eq59_e983_q_d_n14, eq59_e983_q_d_n15, eq59_e983_q_d_n16, eq59_e983_q_d_n17, eq59_e983_q_d_n18, eq59_e983_q_d_n19, eq59_e983_q_d_n20, eq59_e983_q_d_n21, eq59_e983_q_d_n22, eq59_e983_q_d_n23, eq59_e983_q_d_n24, eq59_e983_q_d_n25, eq59_e983_q_d_n26, eq59_e983_q_d_n27, eq59_e983_q_d_n28, eq59_e983_q_d_n29,) = {
    if s.b[760] {
        let eq59_e976_q: f64 = s.v[197];
        let eq59_e979: f64 = (p.p355 * (nv7 - nv14));
        let eq59_e979_d_n7: f64 = p.p355;
        let eq59_e979_d_n14: f64 = (-p.p355);
        let eq59_e980_q: f64 = eq59_e979;
        let eq59_e981: f64 = (s.v[197] + eq59_e979);
        let eq59_e981_d_n7: f64 = (s.dn[197][7] + eq59_e979_d_n7);
        let eq59_e981_d_n14: f64 = (s.dn[197][14] + eq59_e979_d_n14);
        let eq59_e981_q: f64 = (eq59_e976_q + eq59_e980_q);
        let eq59_e981_q_d_n7: f64 = (s.dn[197][7] + eq59_e979_d_n7);
        let eq59_e981_q_d_n14: f64 = (s.dn[197][14] + eq59_e979_d_n14);
        (eq59_e981, s.dn[197][0], s.dn[197][1], s.dn[197][2], s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], eq59_e981_d_n7, s.dn[197][8], s.dn[197][9], s.dn[197][10], s.dn[197][11], s.dn[197][12], s.dn[197][13], eq59_e981_d_n14, s.dn[197][15], s.dn[197][16], s.dn[197][17], s.dn[197][18], s.dn[197][19], s.dn[197][20], s.dn[197][21], s.dn[197][22], s.dn[197][23], s.dn[197][24], s.dn[197][25], s.dn[197][26], s.dn[197][27], s.dn[197][28], s.dn[197][29], eq59_e981_q, s.dn[197][0], s.dn[197][1], s.dn[197][2], s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], eq59_e981_q_d_n7, s.dn[197][8], s.dn[197][9], s.dn[197][10], s.dn[197][11], s.dn[197][12], s.dn[197][13], eq59_e981_q_d_n14, s.dn[197][15], s.dn[197][16], s.dn[197][17], s.dn[197][18], s.dn[197][19], s.dn[197][20], s.dn[197][21], s.dn[197][22], s.dn[197][23], s.dn[197][24], s.dn[197][25], s.dn[197][26], s.dn[197][27], s.dn[197][28], s.dn[197][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_reactive_node_derivatives: [f64; 30] = [eq59_e983_q_d_n0, eq59_e983_q_d_n1, eq59_e983_q_d_n2, eq59_e983_q_d_n3, eq59_e983_q_d_n4, eq59_e983_q_d_n5, eq59_e983_q_d_n6, eq59_e983_q_d_n7, eq59_e983_q_d_n8, eq59_e983_q_d_n9, eq59_e983_q_d_n10, eq59_e983_q_d_n11, eq59_e983_q_d_n12, eq59_e983_q_d_n13, eq59_e983_q_d_n14, eq59_e983_q_d_n15, eq59_e983_q_d_n16, eq59_e983_q_d_n17, eq59_e983_q_d_n18, eq59_e983_q_d_n19, eq59_e983_q_d_n20, eq59_e983_q_d_n21, eq59_e983_q_d_n22, eq59_e983_q_d_n23, eq59_e983_q_d_n24, eq59_e983_q_d_n25, eq59_e983_q_d_n26, eq59_e983_q_d_n27, eq59_e983_q_d_n28, eq59_e983_q_d_n29];
        let eq59_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            nodes,
            &eq59_reactive_node_derivatives,
            branches,
            &eq59_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq60_e993, eq60_e993_d_n0, eq60_e993_d_n1, eq60_e993_d_n2, eq60_e993_d_n3, eq60_e993_d_n4, eq60_e993_d_n5, eq60_e993_d_n6, eq60_e993_d_n7, eq60_e993_d_n8, eq60_e993_d_n9, eq60_e993_d_n10, eq60_e993_d_n11, eq60_e993_d_n12, eq60_e993_d_n13, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_d_n16, eq60_e993_d_n17, eq60_e993_d_n18, eq60_e993_d_n19, eq60_e993_d_n20, eq60_e993_d_n21, eq60_e993_d_n22, eq60_e993_d_n23, eq60_e993_d_n24, eq60_e993_d_n25, eq60_e993_d_n26, eq60_e993_d_n27, eq60_e993_d_n28, eq60_e993_d_n29, eq60_e993_q, eq60_e993_q_d_n0, eq60_e993_q_d_n1, eq60_e993_q_d_n2, eq60_e993_q_d_n3, eq60_e993_q_d_n4, eq60_e993_q_d_n5, eq60_e993_q_d_n6, eq60_e993_q_d_n7, eq60_e993_q_d_n8, eq60_e993_q_d_n9, eq60_e993_q_d_n10, eq60_e993_q_d_n11, eq60_e993_q_d_n12, eq60_e993_q_d_n13, eq60_e993_q_d_n14, eq60_e993_q_d_n15, eq60_e993_q_d_n16, eq60_e993_q_d_n17, eq60_e993_q_d_n18, eq60_e993_q_d_n19, eq60_e993_q_d_n20, eq60_e993_q_d_n21, eq60_e993_q_d_n22, eq60_e993_q_d_n23, eq60_e993_q_d_n24, eq60_e993_q_d_n25, eq60_e993_q_d_n26, eq60_e993_q_d_n27, eq60_e993_q_d_n28, eq60_e993_q_d_n29,) = {
    if s.b[760] {
        let eq60_e986_q: f64 = s.v[198];
        let eq60_e989: f64 = (p.p355 * (nv7 - nv15));
        let eq60_e989_d_n7: f64 = p.p355;
        let eq60_e989_d_n15: f64 = (-p.p355);
        let eq60_e990_q: f64 = eq60_e989;
        let eq60_e991: f64 = (s.v[198] + eq60_e989);
        let eq60_e991_d_n7: f64 = (s.dn[198][7] + eq60_e989_d_n7);
        let eq60_e991_d_n15: f64 = (s.dn[198][15] + eq60_e989_d_n15);
        let eq60_e991_q: f64 = (eq60_e986_q + eq60_e990_q);
        let eq60_e991_q_d_n7: f64 = (s.dn[198][7] + eq60_e989_d_n7);
        let eq60_e991_q_d_n15: f64 = (s.dn[198][15] + eq60_e989_d_n15);
        (eq60_e991, s.dn[198][0], s.dn[198][1], s.dn[198][2], s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], eq60_e991_d_n7, s.dn[198][8], s.dn[198][9], s.dn[198][10], s.dn[198][11], s.dn[198][12], s.dn[198][13], s.dn[198][14], eq60_e991_d_n15, s.dn[198][16], s.dn[198][17], s.dn[198][18], s.dn[198][19], s.dn[198][20], s.dn[198][21], s.dn[198][22], s.dn[198][23], s.dn[198][24], s.dn[198][25], s.dn[198][26], s.dn[198][27], s.dn[198][28], s.dn[198][29], eq60_e991_q, s.dn[198][0], s.dn[198][1], s.dn[198][2], s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], eq60_e991_q_d_n7, s.dn[198][8], s.dn[198][9], s.dn[198][10], s.dn[198][11], s.dn[198][12], s.dn[198][13], s.dn[198][14], eq60_e991_q_d_n15, s.dn[198][16], s.dn[198][17], s.dn[198][18], s.dn[198][19], s.dn[198][20], s.dn[198][21], s.dn[198][22], s.dn[198][23], s.dn[198][24], s.dn[198][25], s.dn[198][26], s.dn[198][27], s.dn[198][28], s.dn[198][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_reactive_node_derivatives: [f64; 30] = [eq60_e993_q_d_n0, eq60_e993_q_d_n1, eq60_e993_q_d_n2, eq60_e993_q_d_n3, eq60_e993_q_d_n4, eq60_e993_q_d_n5, eq60_e993_q_d_n6, eq60_e993_q_d_n7, eq60_e993_q_d_n8, eq60_e993_q_d_n9, eq60_e993_q_d_n10, eq60_e993_q_d_n11, eq60_e993_q_d_n12, eq60_e993_q_d_n13, eq60_e993_q_d_n14, eq60_e993_q_d_n15, eq60_e993_q_d_n16, eq60_e993_q_d_n17, eq60_e993_q_d_n18, eq60_e993_q_d_n19, eq60_e993_q_d_n20, eq60_e993_q_d_n21, eq60_e993_q_d_n22, eq60_e993_q_d_n23, eq60_e993_q_d_n24, eq60_e993_q_d_n25, eq60_e993_q_d_n26, eq60_e993_q_d_n27, eq60_e993_q_d_n28, eq60_e993_q_d_n29];
        let eq60_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            nodes,
            &eq60_reactive_node_derivatives,
            branches,
            &eq60_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1003, eq61_e1003_d_n0, eq61_e1003_d_n1, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n5, eq61_e1003_d_n6, eq61_e1003_d_n7, eq61_e1003_d_n8, eq61_e1003_d_n9, eq61_e1003_d_n10, eq61_e1003_d_n11, eq61_e1003_d_n12, eq61_e1003_d_n13, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_d_n16, eq61_e1003_d_n17, eq61_e1003_d_n18, eq61_e1003_d_n19, eq61_e1003_d_n20, eq61_e1003_d_n21, eq61_e1003_d_n22, eq61_e1003_d_n23, eq61_e1003_d_n24, eq61_e1003_d_n25, eq61_e1003_d_n26, eq61_e1003_d_n27, eq61_e1003_d_n28, eq61_e1003_d_n29, eq61_e1003_q, eq61_e1003_q_d_n0, eq61_e1003_q_d_n1, eq61_e1003_q_d_n2, eq61_e1003_q_d_n3, eq61_e1003_q_d_n4, eq61_e1003_q_d_n5, eq61_e1003_q_d_n6, eq61_e1003_q_d_n7, eq61_e1003_q_d_n8, eq61_e1003_q_d_n9, eq61_e1003_q_d_n10, eq61_e1003_q_d_n11, eq61_e1003_q_d_n12, eq61_e1003_q_d_n13, eq61_e1003_q_d_n14, eq61_e1003_q_d_n15, eq61_e1003_q_d_n16, eq61_e1003_q_d_n17, eq61_e1003_q_d_n18, eq61_e1003_q_d_n19, eq61_e1003_q_d_n20, eq61_e1003_q_d_n21, eq61_e1003_q_d_n22, eq61_e1003_q_d_n23, eq61_e1003_q_d_n24, eq61_e1003_q_d_n25, eq61_e1003_q_d_n26, eq61_e1003_q_d_n27, eq61_e1003_q_d_n28, eq61_e1003_q_d_n29,) = {
    if s.b[760] {
        let eq61_e996_q: f64 = s.v[199];
        let eq61_e999: f64 = (p.p355 * (nv2 - nv14));
        let eq61_e999_d_n2: f64 = p.p355;
        let eq61_e999_d_n14: f64 = (-p.p355);
        let eq61_e1000_q: f64 = eq61_e999;
        let eq61_e1001: f64 = (s.v[199] + eq61_e999);
        let eq61_e1001_d_n2: f64 = (s.dn[199][2] + eq61_e999_d_n2);
        let eq61_e1001_d_n14: f64 = (s.dn[199][14] + eq61_e999_d_n14);
        let eq61_e1001_q: f64 = (eq61_e996_q + eq61_e1000_q);
        let eq61_e1001_q_d_n2: f64 = (s.dn[199][2] + eq61_e999_d_n2);
        let eq61_e1001_q_d_n14: f64 = (s.dn[199][14] + eq61_e999_d_n14);
        (eq61_e1001, s.dn[199][0], s.dn[199][1], eq61_e1001_d_n2, s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], s.dn[199][7], s.dn[199][8], s.dn[199][9], s.dn[199][10], s.dn[199][11], s.dn[199][12], s.dn[199][13], eq61_e1001_d_n14, s.dn[199][15], s.dn[199][16], s.dn[199][17], s.dn[199][18], s.dn[199][19], s.dn[199][20], s.dn[199][21], s.dn[199][22], s.dn[199][23], s.dn[199][24], s.dn[199][25], s.dn[199][26], s.dn[199][27], s.dn[199][28], s.dn[199][29], eq61_e1001_q, s.dn[199][0], s.dn[199][1], eq61_e1001_q_d_n2, s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], s.dn[199][7], s.dn[199][8], s.dn[199][9], s.dn[199][10], s.dn[199][11], s.dn[199][12], s.dn[199][13], eq61_e1001_q_d_n14, s.dn[199][15], s.dn[199][16], s.dn[199][17], s.dn[199][18], s.dn[199][19], s.dn[199][20], s.dn[199][21], s.dn[199][22], s.dn[199][23], s.dn[199][24], s.dn[199][25], s.dn[199][26], s.dn[199][27], s.dn[199][28], s.dn[199][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_reactive_node_derivatives: [f64; 30] = [eq61_e1003_q_d_n0, eq61_e1003_q_d_n1, eq61_e1003_q_d_n2, eq61_e1003_q_d_n3, eq61_e1003_q_d_n4, eq61_e1003_q_d_n5, eq61_e1003_q_d_n6, eq61_e1003_q_d_n7, eq61_e1003_q_d_n8, eq61_e1003_q_d_n9, eq61_e1003_q_d_n10, eq61_e1003_q_d_n11, eq61_e1003_q_d_n12, eq61_e1003_q_d_n13, eq61_e1003_q_d_n14, eq61_e1003_q_d_n15, eq61_e1003_q_d_n16, eq61_e1003_q_d_n17, eq61_e1003_q_d_n18, eq61_e1003_q_d_n19, eq61_e1003_q_d_n20, eq61_e1003_q_d_n21, eq61_e1003_q_d_n22, eq61_e1003_q_d_n23, eq61_e1003_q_d_n24, eq61_e1003_q_d_n25, eq61_e1003_q_d_n26, eq61_e1003_q_d_n27, eq61_e1003_q_d_n28, eq61_e1003_q_d_n29];
        let eq61_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            nodes,
            &eq61_reactive_node_derivatives,
            branches,
            &eq61_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq63_e1017, eq63_e1017_d_n0, eq63_e1017_d_n1, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n5, eq63_e1017_d_n6, eq63_e1017_d_n7, eq63_e1017_d_n8, eq63_e1017_d_n9, eq63_e1017_d_n10, eq63_e1017_d_n11, eq63_e1017_d_n12, eq63_e1017_d_n13, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_d_n16, eq63_e1017_d_n17, eq63_e1017_d_n18, eq63_e1017_d_n19, eq63_e1017_d_n20, eq63_e1017_d_n21, eq63_e1017_d_n22, eq63_e1017_d_n23, eq63_e1017_d_n24, eq63_e1017_d_n25, eq63_e1017_d_n26, eq63_e1017_d_n27, eq63_e1017_d_n28, eq63_e1017_d_n29, eq63_e1017_q, eq63_e1017_q_d_n0, eq63_e1017_q_d_n1, eq63_e1017_q_d_n2, eq63_e1017_q_d_n3, eq63_e1017_q_d_n4, eq63_e1017_q_d_n5, eq63_e1017_q_d_n6, eq63_e1017_q_d_n7, eq63_e1017_q_d_n8, eq63_e1017_q_d_n9, eq63_e1017_q_d_n10, eq63_e1017_q_d_n11, eq63_e1017_q_d_n12, eq63_e1017_q_d_n13, eq63_e1017_q_d_n14, eq63_e1017_q_d_n15, eq63_e1017_q_d_n16, eq63_e1017_q_d_n17, eq63_e1017_q_d_n18, eq63_e1017_q_d_n19, eq63_e1017_q_d_n20, eq63_e1017_q_d_n21, eq63_e1017_q_d_n22, eq63_e1017_q_d_n23, eq63_e1017_q_d_n24, eq63_e1017_q_d_n25, eq63_e1017_q_d_n26, eq63_e1017_q_d_n27, eq63_e1017_q_d_n28, eq63_e1017_q_d_n29,) = {
    if s.b[760] {
        let eq63_e1010_q: f64 = s.v[201];
        let eq63_e1013: f64 = (p.p355 * (nv7 - nv9));
        let eq63_e1013_d_n7: f64 = p.p355;
        let eq63_e1013_d_n9: f64 = (-p.p355);
        let eq63_e1014_q: f64 = eq63_e1013;
        let eq63_e1015: f64 = (s.v[201] + eq63_e1013);
        let eq63_e1015_d_n7: f64 = (s.dn[201][7] + eq63_e1013_d_n7);
        let eq63_e1015_d_n9: f64 = (s.dn[201][9] + eq63_e1013_d_n9);
        let eq63_e1015_q: f64 = (eq63_e1010_q + eq63_e1014_q);
        let eq63_e1015_q_d_n7: f64 = (s.dn[201][7] + eq63_e1013_d_n7);
        let eq63_e1015_q_d_n9: f64 = (s.dn[201][9] + eq63_e1013_d_n9);
        (eq63_e1015, s.dn[201][0], s.dn[201][1], s.dn[201][2], s.dn[201][3], s.dn[201][4], s.dn[201][5], s.dn[201][6], eq63_e1015_d_n7, s.dn[201][8], eq63_e1015_d_n9, s.dn[201][10], s.dn[201][11], s.dn[201][12], s.dn[201][13], s.dn[201][14], s.dn[201][15], s.dn[201][16], s.dn[201][17], s.dn[201][18], s.dn[201][19], s.dn[201][20], s.dn[201][21], s.dn[201][22], s.dn[201][23], s.dn[201][24], s.dn[201][25], s.dn[201][26], s.dn[201][27], s.dn[201][28], s.dn[201][29], eq63_e1015_q, s.dn[201][0], s.dn[201][1], s.dn[201][2], s.dn[201][3], s.dn[201][4], s.dn[201][5], s.dn[201][6], eq63_e1015_q_d_n7, s.dn[201][8], eq63_e1015_q_d_n9, s.dn[201][10], s.dn[201][11], s.dn[201][12], s.dn[201][13], s.dn[201][14], s.dn[201][15], s.dn[201][16], s.dn[201][17], s.dn[201][18], s.dn[201][19], s.dn[201][20], s.dn[201][21], s.dn[201][22], s.dn[201][23], s.dn[201][24], s.dn[201][25], s.dn[201][26], s.dn[201][27], s.dn[201][28], s.dn[201][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_reactive_node_derivatives: [f64; 30] = [eq63_e1017_q_d_n0, eq63_e1017_q_d_n1, eq63_e1017_q_d_n2, eq63_e1017_q_d_n3, eq63_e1017_q_d_n4, eq63_e1017_q_d_n5, eq63_e1017_q_d_n6, eq63_e1017_q_d_n7, eq63_e1017_q_d_n8, eq63_e1017_q_d_n9, eq63_e1017_q_d_n10, eq63_e1017_q_d_n11, eq63_e1017_q_d_n12, eq63_e1017_q_d_n13, eq63_e1017_q_d_n14, eq63_e1017_q_d_n15, eq63_e1017_q_d_n16, eq63_e1017_q_d_n17, eq63_e1017_q_d_n18, eq63_e1017_q_d_n19, eq63_e1017_q_d_n20, eq63_e1017_q_d_n21, eq63_e1017_q_d_n22, eq63_e1017_q_d_n23, eq63_e1017_q_d_n24, eq63_e1017_q_d_n25, eq63_e1017_q_d_n26, eq63_e1017_q_d_n27, eq63_e1017_q_d_n28, eq63_e1017_q_d_n29];
        let eq63_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq63_reactive_node_derivatives,
            branches,
            &eq63_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq64_e1028, eq64_e1028_d_n0, eq64_e1028_d_n1, eq64_e1028_d_n2, eq64_e1028_d_n3, eq64_e1028_d_n4, eq64_e1028_d_n5, eq64_e1028_d_n6, eq64_e1028_d_n7, eq64_e1028_d_n8, eq64_e1028_d_n9, eq64_e1028_d_n10, eq64_e1028_d_n11, eq64_e1028_d_n12, eq64_e1028_d_n13, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_d_n16, eq64_e1028_d_n17, eq64_e1028_d_n18, eq64_e1028_d_n19, eq64_e1028_d_n20, eq64_e1028_d_n21, eq64_e1028_d_n22, eq64_e1028_d_n23, eq64_e1028_d_n24, eq64_e1028_d_n25, eq64_e1028_d_n26, eq64_e1028_d_n27, eq64_e1028_d_n28, eq64_e1028_d_n29, eq64_e1028_q, eq64_e1028_q_d_n0, eq64_e1028_q_d_n1, eq64_e1028_q_d_n2, eq64_e1028_q_d_n3, eq64_e1028_q_d_n4, eq64_e1028_q_d_n5, eq64_e1028_q_d_n6, eq64_e1028_q_d_n7, eq64_e1028_q_d_n8, eq64_e1028_q_d_n9, eq64_e1028_q_d_n10, eq64_e1028_q_d_n11, eq64_e1028_q_d_n12, eq64_e1028_q_d_n13, eq64_e1028_q_d_n14, eq64_e1028_q_d_n15, eq64_e1028_q_d_n16, eq64_e1028_q_d_n17, eq64_e1028_q_d_n18, eq64_e1028_q_d_n19, eq64_e1028_q_d_n20, eq64_e1028_q_d_n21, eq64_e1028_q_d_n22, eq64_e1028_q_d_n23, eq64_e1028_q_d_n24, eq64_e1028_q_d_n25, eq64_e1028_q_d_n26, eq64_e1028_q_d_n27, eq64_e1028_q_d_n28, eq64_e1028_q_d_n29,) = {
    if (!s.b[760]) {
        let eq64_e1021_q: f64 = s.v[197];
        let eq64_e1024: f64 = (p.p355 * (nv2 - nv14));
        let eq64_e1024_d_n2: f64 = p.p355;
        let eq64_e1024_d_n14: f64 = (-p.p355);
        let eq64_e1025_q: f64 = eq64_e1024;
        let eq64_e1026: f64 = (s.v[197] + eq64_e1024);
        let eq64_e1026_d_n2: f64 = (s.dn[197][2] + eq64_e1024_d_n2);
        let eq64_e1026_d_n14: f64 = (s.dn[197][14] + eq64_e1024_d_n14);
        let eq64_e1026_q: f64 = (eq64_e1021_q + eq64_e1025_q);
        let eq64_e1026_q_d_n2: f64 = (s.dn[197][2] + eq64_e1024_d_n2);
        let eq64_e1026_q_d_n14: f64 = (s.dn[197][14] + eq64_e1024_d_n14);
        (eq64_e1026, s.dn[197][0], s.dn[197][1], eq64_e1026_d_n2, s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], s.dn[197][7], s.dn[197][8], s.dn[197][9], s.dn[197][10], s.dn[197][11], s.dn[197][12], s.dn[197][13], eq64_e1026_d_n14, s.dn[197][15], s.dn[197][16], s.dn[197][17], s.dn[197][18], s.dn[197][19], s.dn[197][20], s.dn[197][21], s.dn[197][22], s.dn[197][23], s.dn[197][24], s.dn[197][25], s.dn[197][26], s.dn[197][27], s.dn[197][28], s.dn[197][29], eq64_e1026_q, s.dn[197][0], s.dn[197][1], eq64_e1026_q_d_n2, s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], s.dn[197][7], s.dn[197][8], s.dn[197][9], s.dn[197][10], s.dn[197][11], s.dn[197][12], s.dn[197][13], eq64_e1026_q_d_n14, s.dn[197][15], s.dn[197][16], s.dn[197][17], s.dn[197][18], s.dn[197][19], s.dn[197][20], s.dn[197][21], s.dn[197][22], s.dn[197][23], s.dn[197][24], s.dn[197][25], s.dn[197][26], s.dn[197][27], s.dn[197][28], s.dn[197][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_reactive_node_derivatives: [f64; 30] = [eq64_e1028_q_d_n0, eq64_e1028_q_d_n1, eq64_e1028_q_d_n2, eq64_e1028_q_d_n3, eq64_e1028_q_d_n4, eq64_e1028_q_d_n5, eq64_e1028_q_d_n6, eq64_e1028_q_d_n7, eq64_e1028_q_d_n8, eq64_e1028_q_d_n9, eq64_e1028_q_d_n10, eq64_e1028_q_d_n11, eq64_e1028_q_d_n12, eq64_e1028_q_d_n13, eq64_e1028_q_d_n14, eq64_e1028_q_d_n15, eq64_e1028_q_d_n16, eq64_e1028_q_d_n17, eq64_e1028_q_d_n18, eq64_e1028_q_d_n19, eq64_e1028_q_d_n20, eq64_e1028_q_d_n21, eq64_e1028_q_d_n22, eq64_e1028_q_d_n23, eq64_e1028_q_d_n24, eq64_e1028_q_d_n25, eq64_e1028_q_d_n26, eq64_e1028_q_d_n27, eq64_e1028_q_d_n28, eq64_e1028_q_d_n29];
        let eq64_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            nodes,
            &eq64_reactive_node_derivatives,
            branches,
            &eq64_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq65_e1039, eq65_e1039_d_n0, eq65_e1039_d_n1, eq65_e1039_d_n2, eq65_e1039_d_n3, eq65_e1039_d_n4, eq65_e1039_d_n5, eq65_e1039_d_n6, eq65_e1039_d_n7, eq65_e1039_d_n8, eq65_e1039_d_n9, eq65_e1039_d_n10, eq65_e1039_d_n11, eq65_e1039_d_n12, eq65_e1039_d_n13, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_d_n16, eq65_e1039_d_n17, eq65_e1039_d_n18, eq65_e1039_d_n19, eq65_e1039_d_n20, eq65_e1039_d_n21, eq65_e1039_d_n22, eq65_e1039_d_n23, eq65_e1039_d_n24, eq65_e1039_d_n25, eq65_e1039_d_n26, eq65_e1039_d_n27, eq65_e1039_d_n28, eq65_e1039_d_n29, eq65_e1039_q, eq65_e1039_q_d_n0, eq65_e1039_q_d_n1, eq65_e1039_q_d_n2, eq65_e1039_q_d_n3, eq65_e1039_q_d_n4, eq65_e1039_q_d_n5, eq65_e1039_q_d_n6, eq65_e1039_q_d_n7, eq65_e1039_q_d_n8, eq65_e1039_q_d_n9, eq65_e1039_q_d_n10, eq65_e1039_q_d_n11, eq65_e1039_q_d_n12, eq65_e1039_q_d_n13, eq65_e1039_q_d_n14, eq65_e1039_q_d_n15, eq65_e1039_q_d_n16, eq65_e1039_q_d_n17, eq65_e1039_q_d_n18, eq65_e1039_q_d_n19, eq65_e1039_q_d_n20, eq65_e1039_q_d_n21, eq65_e1039_q_d_n22, eq65_e1039_q_d_n23, eq65_e1039_q_d_n24, eq65_e1039_q_d_n25, eq65_e1039_q_d_n26, eq65_e1039_q_d_n27, eq65_e1039_q_d_n28, eq65_e1039_q_d_n29,) = {
    if (!s.b[760]) {
        let eq65_e1032_q: f64 = s.v[198];
        let eq65_e1035: f64 = (p.p355 * (nv2 - nv15));
        let eq65_e1035_d_n2: f64 = p.p355;
        let eq65_e1035_d_n15: f64 = (-p.p355);
        let eq65_e1036_q: f64 = eq65_e1035;
        let eq65_e1037: f64 = (s.v[198] + eq65_e1035);
        let eq65_e1037_d_n2: f64 = (s.dn[198][2] + eq65_e1035_d_n2);
        let eq65_e1037_d_n15: f64 = (s.dn[198][15] + eq65_e1035_d_n15);
        let eq65_e1037_q: f64 = (eq65_e1032_q + eq65_e1036_q);
        let eq65_e1037_q_d_n2: f64 = (s.dn[198][2] + eq65_e1035_d_n2);
        let eq65_e1037_q_d_n15: f64 = (s.dn[198][15] + eq65_e1035_d_n15);
        (eq65_e1037, s.dn[198][0], s.dn[198][1], eq65_e1037_d_n2, s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], s.dn[198][7], s.dn[198][8], s.dn[198][9], s.dn[198][10], s.dn[198][11], s.dn[198][12], s.dn[198][13], s.dn[198][14], eq65_e1037_d_n15, s.dn[198][16], s.dn[198][17], s.dn[198][18], s.dn[198][19], s.dn[198][20], s.dn[198][21], s.dn[198][22], s.dn[198][23], s.dn[198][24], s.dn[198][25], s.dn[198][26], s.dn[198][27], s.dn[198][28], s.dn[198][29], eq65_e1037_q, s.dn[198][0], s.dn[198][1], eq65_e1037_q_d_n2, s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], s.dn[198][7], s.dn[198][8], s.dn[198][9], s.dn[198][10], s.dn[198][11], s.dn[198][12], s.dn[198][13], s.dn[198][14], eq65_e1037_q_d_n15, s.dn[198][16], s.dn[198][17], s.dn[198][18], s.dn[198][19], s.dn[198][20], s.dn[198][21], s.dn[198][22], s.dn[198][23], s.dn[198][24], s.dn[198][25], s.dn[198][26], s.dn[198][27], s.dn[198][28], s.dn[198][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_reactive_node_derivatives: [f64; 30] = [eq65_e1039_q_d_n0, eq65_e1039_q_d_n1, eq65_e1039_q_d_n2, eq65_e1039_q_d_n3, eq65_e1039_q_d_n4, eq65_e1039_q_d_n5, eq65_e1039_q_d_n6, eq65_e1039_q_d_n7, eq65_e1039_q_d_n8, eq65_e1039_q_d_n9, eq65_e1039_q_d_n10, eq65_e1039_q_d_n11, eq65_e1039_q_d_n12, eq65_e1039_q_d_n13, eq65_e1039_q_d_n14, eq65_e1039_q_d_n15, eq65_e1039_q_d_n16, eq65_e1039_q_d_n17, eq65_e1039_q_d_n18, eq65_e1039_q_d_n19, eq65_e1039_q_d_n20, eq65_e1039_q_d_n21, eq65_e1039_q_d_n22, eq65_e1039_q_d_n23, eq65_e1039_q_d_n24, eq65_e1039_q_d_n25, eq65_e1039_q_d_n26, eq65_e1039_q_d_n27, eq65_e1039_q_d_n28, eq65_e1039_q_d_n29];
        let eq65_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq65_reactive_node_derivatives,
            branches,
            &eq65_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1050, eq66_e1050_d_n0, eq66_e1050_d_n1, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n5, eq66_e1050_d_n6, eq66_e1050_d_n7, eq66_e1050_d_n8, eq66_e1050_d_n9, eq66_e1050_d_n10, eq66_e1050_d_n11, eq66_e1050_d_n12, eq66_e1050_d_n13, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_d_n16, eq66_e1050_d_n17, eq66_e1050_d_n18, eq66_e1050_d_n19, eq66_e1050_d_n20, eq66_e1050_d_n21, eq66_e1050_d_n22, eq66_e1050_d_n23, eq66_e1050_d_n24, eq66_e1050_d_n25, eq66_e1050_d_n26, eq66_e1050_d_n27, eq66_e1050_d_n28, eq66_e1050_d_n29, eq66_e1050_q, eq66_e1050_q_d_n0, eq66_e1050_q_d_n1, eq66_e1050_q_d_n2, eq66_e1050_q_d_n3, eq66_e1050_q_d_n4, eq66_e1050_q_d_n5, eq66_e1050_q_d_n6, eq66_e1050_q_d_n7, eq66_e1050_q_d_n8, eq66_e1050_q_d_n9, eq66_e1050_q_d_n10, eq66_e1050_q_d_n11, eq66_e1050_q_d_n12, eq66_e1050_q_d_n13, eq66_e1050_q_d_n14, eq66_e1050_q_d_n15, eq66_e1050_q_d_n16, eq66_e1050_q_d_n17, eq66_e1050_q_d_n18, eq66_e1050_q_d_n19, eq66_e1050_q_d_n20, eq66_e1050_q_d_n21, eq66_e1050_q_d_n22, eq66_e1050_q_d_n23, eq66_e1050_q_d_n24, eq66_e1050_q_d_n25, eq66_e1050_q_d_n26, eq66_e1050_q_d_n27, eq66_e1050_q_d_n28, eq66_e1050_q_d_n29,) = {
    if (!s.b[760]) {
        let eq66_e1043_q: f64 = s.v[199];
        let eq66_e1046: f64 = (p.p355 * (nv7 - nv14));
        let eq66_e1046_d_n7: f64 = p.p355;
        let eq66_e1046_d_n14: f64 = (-p.p355);
        let eq66_e1047_q: f64 = eq66_e1046;
        let eq66_e1048: f64 = (s.v[199] + eq66_e1046);
        let eq66_e1048_d_n7: f64 = (s.dn[199][7] + eq66_e1046_d_n7);
        let eq66_e1048_d_n14: f64 = (s.dn[199][14] + eq66_e1046_d_n14);
        let eq66_e1048_q: f64 = (eq66_e1043_q + eq66_e1047_q);
        let eq66_e1048_q_d_n7: f64 = (s.dn[199][7] + eq66_e1046_d_n7);
        let eq66_e1048_q_d_n14: f64 = (s.dn[199][14] + eq66_e1046_d_n14);
        (eq66_e1048, s.dn[199][0], s.dn[199][1], s.dn[199][2], s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], eq66_e1048_d_n7, s.dn[199][8], s.dn[199][9], s.dn[199][10], s.dn[199][11], s.dn[199][12], s.dn[199][13], eq66_e1048_d_n14, s.dn[199][15], s.dn[199][16], s.dn[199][17], s.dn[199][18], s.dn[199][19], s.dn[199][20], s.dn[199][21], s.dn[199][22], s.dn[199][23], s.dn[199][24], s.dn[199][25], s.dn[199][26], s.dn[199][27], s.dn[199][28], s.dn[199][29], eq66_e1048_q, s.dn[199][0], s.dn[199][1], s.dn[199][2], s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], eq66_e1048_q_d_n7, s.dn[199][8], s.dn[199][9], s.dn[199][10], s.dn[199][11], s.dn[199][12], s.dn[199][13], eq66_e1048_q_d_n14, s.dn[199][15], s.dn[199][16], s.dn[199][17], s.dn[199][18], s.dn[199][19], s.dn[199][20], s.dn[199][21], s.dn[199][22], s.dn[199][23], s.dn[199][24], s.dn[199][25], s.dn[199][26], s.dn[199][27], s.dn[199][28], s.dn[199][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_reactive_node_derivatives: [f64; 30] = [eq66_e1050_q_d_n0, eq66_e1050_q_d_n1, eq66_e1050_q_d_n2, eq66_e1050_q_d_n3, eq66_e1050_q_d_n4, eq66_e1050_q_d_n5, eq66_e1050_q_d_n6, eq66_e1050_q_d_n7, eq66_e1050_q_d_n8, eq66_e1050_q_d_n9, eq66_e1050_q_d_n10, eq66_e1050_q_d_n11, eq66_e1050_q_d_n12, eq66_e1050_q_d_n13, eq66_e1050_q_d_n14, eq66_e1050_q_d_n15, eq66_e1050_q_d_n16, eq66_e1050_q_d_n17, eq66_e1050_q_d_n18, eq66_e1050_q_d_n19, eq66_e1050_q_d_n20, eq66_e1050_q_d_n21, eq66_e1050_q_d_n22, eq66_e1050_q_d_n23, eq66_e1050_q_d_n24, eq66_e1050_q_d_n25, eq66_e1050_q_d_n26, eq66_e1050_q_d_n27, eq66_e1050_q_d_n28, eq66_e1050_q_d_n29];
        let eq66_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            nodes,
            &eq66_reactive_node_derivatives,
            branches,
            &eq66_reactive_branch_derivatives,
            multiplicity,
        );
        let eq69_e1062_q: f64 = s.v[200];
        let eq69_e1065: f64 = (p.p355 * (nv3 - nv14));
        let eq69_e1065_d_n3: f64 = p.p355;
        let eq69_e1065_d_n14: f64 = (-p.p355);
        let eq69_e1066_q: f64 = eq69_e1065;
        let eq69_e1067: f64 = (s.v[200] + eq69_e1065);
        let eq69_e1067_d_n3: f64 = (s.dn[200][3] + eq69_e1065_d_n3);
        let eq69_e1067_d_n14: f64 = (s.dn[200][14] + eq69_e1065_d_n14);
        let eq69_e1067_q: f64 = (eq69_e1062_q + eq69_e1066_q);
        let eq69_e1067_q_d_n3: f64 = (s.dn[200][3] + eq69_e1065_d_n3);
        let eq69_e1067_q_d_n14: f64 = (s.dn[200][14] + eq69_e1065_d_n14);
        let eq69_reactive_node_derivatives: [f64; 30] = [s.dn[200][0], s.dn[200][1], s.dn[200][2], eq69_e1067_q_d_n3, s.dn[200][4], s.dn[200][5], s.dn[200][6], s.dn[200][7], s.dn[200][8], s.dn[200][9], s.dn[200][10], s.dn[200][11], s.dn[200][12], s.dn[200][13], eq69_e1067_q_d_n14, s.dn[200][15], s.dn[200][16], s.dn[200][17], s.dn[200][18], s.dn[200][19], s.dn[200][20], s.dn[200][21], s.dn[200][22], s.dn[200][23], s.dn[200][24], s.dn[200][25], s.dn[200][26], s.dn[200][27], s.dn[200][28], s.dn[200][29]];
        let eq69_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[14]),
            nodes,
            &eq69_reactive_node_derivatives,
            branches,
            &eq69_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq72_e1090, eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29, eq72_e1090_q, eq72_e1090_q_d_n0, eq72_e1090_q_d_n1, eq72_e1090_q_d_n2, eq72_e1090_q_d_n3, eq72_e1090_q_d_n4, eq72_e1090_q_d_n5, eq72_e1090_q_d_n6, eq72_e1090_q_d_n7, eq72_e1090_q_d_n8, eq72_e1090_q_d_n9, eq72_e1090_q_d_n10, eq72_e1090_q_d_n11, eq72_e1090_q_d_n12, eq72_e1090_q_d_n13, eq72_e1090_q_d_n14, eq72_e1090_q_d_n15, eq72_e1090_q_d_n16, eq72_e1090_q_d_n17, eq72_e1090_q_d_n18, eq72_e1090_q_d_n19, eq72_e1090_q_d_n20, eq72_e1090_q_d_n21, eq72_e1090_q_d_n22, eq72_e1090_q_d_n23, eq72_e1090_q_d_n24, eq72_e1090_q_d_n25, eq72_e1090_q_d_n26, eq72_e1090_q_d_n27, eq72_e1090_q_d_n28, eq72_e1090_q_d_n29,) = {
    if s.b[907] {
        let eq72_e1083_q: f64 = s.v[191];
        let eq72_e1086: f64 = (p.p355 * (nv7 - nv5));
        let eq72_e1086_d_n5: f64 = (-p.p355);
        let eq72_e1086_d_n7: f64 = p.p355;
        let eq72_e1087_q: f64 = eq72_e1086;
        let eq72_e1088: f64 = (s.v[191] + eq72_e1086);
        let eq72_e1088_d_n5: f64 = (s.dn[191][5] + eq72_e1086_d_n5);
        let eq72_e1088_d_n7: f64 = (s.dn[191][7] + eq72_e1086_d_n7);
        let eq72_e1088_q: f64 = (eq72_e1083_q + eq72_e1087_q);
        let eq72_e1088_q_d_n5: f64 = (s.dn[191][5] + eq72_e1086_d_n5);
        let eq72_e1088_q_d_n7: f64 = (s.dn[191][7] + eq72_e1086_d_n7);
        (eq72_e1088, s.dn[191][0], s.dn[191][1], s.dn[191][2], s.dn[191][3], s.dn[191][4], eq72_e1088_d_n5, s.dn[191][6], eq72_e1088_d_n7, s.dn[191][8], s.dn[191][9], s.dn[191][10], s.dn[191][11], s.dn[191][12], s.dn[191][13], s.dn[191][14], s.dn[191][15], s.dn[191][16], s.dn[191][17], s.dn[191][18], s.dn[191][19], s.dn[191][20], s.dn[191][21], s.dn[191][22], s.dn[191][23], s.dn[191][24], s.dn[191][25], s.dn[191][26], s.dn[191][27], s.dn[191][28], s.dn[191][29], eq72_e1088_q, s.dn[191][0], s.dn[191][1], s.dn[191][2], s.dn[191][3], s.dn[191][4], eq72_e1088_q_d_n5, s.dn[191][6], eq72_e1088_q_d_n7, s.dn[191][8], s.dn[191][9], s.dn[191][10], s.dn[191][11], s.dn[191][12], s.dn[191][13], s.dn[191][14], s.dn[191][15], s.dn[191][16], s.dn[191][17], s.dn[191][18], s.dn[191][19], s.dn[191][20], s.dn[191][21], s.dn[191][22], s.dn[191][23], s.dn[191][24], s.dn[191][25], s.dn[191][26], s.dn[191][27], s.dn[191][28], s.dn[191][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_reactive_node_derivatives: [f64; 30] = [eq72_e1090_q_d_n0, eq72_e1090_q_d_n1, eq72_e1090_q_d_n2, eq72_e1090_q_d_n3, eq72_e1090_q_d_n4, eq72_e1090_q_d_n5, eq72_e1090_q_d_n6, eq72_e1090_q_d_n7, eq72_e1090_q_d_n8, eq72_e1090_q_d_n9, eq72_e1090_q_d_n10, eq72_e1090_q_d_n11, eq72_e1090_q_d_n12, eq72_e1090_q_d_n13, eq72_e1090_q_d_n14, eq72_e1090_q_d_n15, eq72_e1090_q_d_n16, eq72_e1090_q_d_n17, eq72_e1090_q_d_n18, eq72_e1090_q_d_n19, eq72_e1090_q_d_n20, eq72_e1090_q_d_n21, eq72_e1090_q_d_n22, eq72_e1090_q_d_n23, eq72_e1090_q_d_n24, eq72_e1090_q_d_n25, eq72_e1090_q_d_n26, eq72_e1090_q_d_n27, eq72_e1090_q_d_n28, eq72_e1090_q_d_n29];
        let eq72_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq72_reactive_node_derivatives,
            branches,
            &eq72_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1100, eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29, eq73_e1100_q, eq73_e1100_q_d_n0, eq73_e1100_q_d_n1, eq73_e1100_q_d_n2, eq73_e1100_q_d_n3, eq73_e1100_q_d_n4, eq73_e1100_q_d_n5, eq73_e1100_q_d_n6, eq73_e1100_q_d_n7, eq73_e1100_q_d_n8, eq73_e1100_q_d_n9, eq73_e1100_q_d_n10, eq73_e1100_q_d_n11, eq73_e1100_q_d_n12, eq73_e1100_q_d_n13, eq73_e1100_q_d_n14, eq73_e1100_q_d_n15, eq73_e1100_q_d_n16, eq73_e1100_q_d_n17, eq73_e1100_q_d_n18, eq73_e1100_q_d_n19, eq73_e1100_q_d_n20, eq73_e1100_q_d_n21, eq73_e1100_q_d_n22, eq73_e1100_q_d_n23, eq73_e1100_q_d_n24, eq73_e1100_q_d_n25, eq73_e1100_q_d_n26, eq73_e1100_q_d_n27, eq73_e1100_q_d_n28, eq73_e1100_q_d_n29,) = {
    if s.b[907] {
        let eq73_e1093_q: f64 = s.v[192];
        let eq73_e1096: f64 = (p.p355 * (nv7 - nv14));
        let eq73_e1096_d_n7: f64 = p.p355;
        let eq73_e1096_d_n14: f64 = (-p.p355);
        let eq73_e1097_q: f64 = eq73_e1096;
        let eq73_e1098: f64 = (s.v[192] + eq73_e1096);
        let eq73_e1098_d_n7: f64 = (s.dn[192][7] + eq73_e1096_d_n7);
        let eq73_e1098_d_n14: f64 = (s.dn[192][14] + eq73_e1096_d_n14);
        let eq73_e1098_q: f64 = (eq73_e1093_q + eq73_e1097_q);
        let eq73_e1098_q_d_n7: f64 = (s.dn[192][7] + eq73_e1096_d_n7);
        let eq73_e1098_q_d_n14: f64 = (s.dn[192][14] + eq73_e1096_d_n14);
        (eq73_e1098, s.dn[192][0], s.dn[192][1], s.dn[192][2], s.dn[192][3], s.dn[192][4], s.dn[192][5], s.dn[192][6], eq73_e1098_d_n7, s.dn[192][8], s.dn[192][9], s.dn[192][10], s.dn[192][11], s.dn[192][12], s.dn[192][13], eq73_e1098_d_n14, s.dn[192][15], s.dn[192][16], s.dn[192][17], s.dn[192][18], s.dn[192][19], s.dn[192][20], s.dn[192][21], s.dn[192][22], s.dn[192][23], s.dn[192][24], s.dn[192][25], s.dn[192][26], s.dn[192][27], s.dn[192][28], s.dn[192][29], eq73_e1098_q, s.dn[192][0], s.dn[192][1], s.dn[192][2], s.dn[192][3], s.dn[192][4], s.dn[192][5], s.dn[192][6], eq73_e1098_q_d_n7, s.dn[192][8], s.dn[192][9], s.dn[192][10], s.dn[192][11], s.dn[192][12], s.dn[192][13], eq73_e1098_q_d_n14, s.dn[192][15], s.dn[192][16], s.dn[192][17], s.dn[192][18], s.dn[192][19], s.dn[192][20], s.dn[192][21], s.dn[192][22], s.dn[192][23], s.dn[192][24], s.dn[192][25], s.dn[192][26], s.dn[192][27], s.dn[192][28], s.dn[192][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 30] = [eq73_e1100_q_d_n0, eq73_e1100_q_d_n1, eq73_e1100_q_d_n2, eq73_e1100_q_d_n3, eq73_e1100_q_d_n4, eq73_e1100_q_d_n5, eq73_e1100_q_d_n6, eq73_e1100_q_d_n7, eq73_e1100_q_d_n8, eq73_e1100_q_d_n9, eq73_e1100_q_d_n10, eq73_e1100_q_d_n11, eq73_e1100_q_d_n12, eq73_e1100_q_d_n13, eq73_e1100_q_d_n14, eq73_e1100_q_d_n15, eq73_e1100_q_d_n16, eq73_e1100_q_d_n17, eq73_e1100_q_d_n18, eq73_e1100_q_d_n19, eq73_e1100_q_d_n20, eq73_e1100_q_d_n21, eq73_e1100_q_d_n22, eq73_e1100_q_d_n23, eq73_e1100_q_d_n24, eq73_e1100_q_d_n25, eq73_e1100_q_d_n26, eq73_e1100_q_d_n27, eq73_e1100_q_d_n28, eq73_e1100_q_d_n29];
        let eq73_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
