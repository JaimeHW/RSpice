#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq45_e834, eq45_e834_d_n0, eq45_e834_d_n1, eq45_e834_d_n2, eq45_e834_d_n3, eq45_e834_d_n4, eq45_e834_d_n5, eq45_e834_d_n6, eq45_e834_d_n7, eq45_e834_d_n8, eq45_e834_d_n9, eq45_e834_d_n10, eq45_e834_d_n11, eq45_e834_d_n12, eq45_e834_d_n13, eq45_e834_d_n14, eq45_e834_d_n15, eq45_e834_d_n16, eq45_e834_d_n17, eq45_e834_d_n18, eq45_e834_d_n19, eq45_e834_d_n20, eq45_e834_d_n21, eq45_e834_d_n22, eq45_e834_d_b0, eq45_e834_d_b1, eq45_e834_d_b2, eq45_e834_d_b3, eq45_e834_d_b4, eq45_e834_d_b5, eq45_e834_d_b6, eq45_e834_d_b7, eq45_e834_d_b8, eq45_e834_d_b9, eq45_e834_d_b10, eq45_e834_d_b11, eq45_e834_d_b12, eq45_e834_d_b13, eq45_e834_d_b14, eq45_e834_d_b15, eq45_e834_d_b16, eq45_e834_d_b17, eq45_e834_d_b18, eq45_e834_d_b19, eq45_e834_d_b20, eq45_e834_d_b21, eq45_e834_d_b22, eq45_e834_d_b23, eq45_e834_d_b24, eq45_e834_d_b25, eq45_e834_d_b26, eq45_e834_d_b27, eq45_e834_d_b28, eq45_e834_d_b29, eq45_e834_d_b30, eq45_e834_d_b31, eq45_e834_d_b32, eq45_e834_d_b33, eq45_e834_d_b34, eq45_e834_d_b35, eq45_e834_d_b36, eq45_e834_d_b37, eq45_e834_d_b38, eq45_e834_d_b39, eq45_e834_d_b40, eq45_e834_d_b41, eq45_e834_d_b42, eq45_e834_d_b43, eq45_e834_d_b44, eq45_e834_d_b45, eq45_e834_d_b46, eq45_e834_d_b47, eq45_e834_d_b48, eq45_e834_d_b49, eq45_e834_d_b50, eq45_e834_d_b51, eq45_e834_d_b52, eq45_e834_d_b53, eq45_e834_d_b54,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq45_e830: f64 = (p.p144 * s.v[367]);
        let eq45_e830_d_n0: f64 = (p.p144 * s.dn[367][0]);
        let eq45_e830_d_n1: f64 = (p.p144 * s.dn[367][1]);
        let eq45_e830_d_n2: f64 = (p.p144 * s.dn[367][2]);
        let eq45_e830_d_n3: f64 = (p.p144 * s.dn[367][3]);
        let eq45_e830_d_n4: f64 = (p.p144 * s.dn[367][4]);
        let eq45_e830_d_n5: f64 = (p.p144 * s.dn[367][5]);
        let eq45_e830_d_n6: f64 = (p.p144 * s.dn[367][6]);
        let eq45_e830_d_n7: f64 = (p.p144 * s.dn[367][7]);
        let eq45_e830_d_n8: f64 = (p.p144 * s.dn[367][8]);
        let eq45_e830_d_n9: f64 = (p.p144 * s.dn[367][9]);
        let eq45_e830_d_n10: f64 = (p.p144 * s.dn[367][10]);
        let eq45_e830_d_n11: f64 = (p.p144 * s.dn[367][11]);
        let eq45_e830_d_n12: f64 = (p.p144 * s.dn[367][12]);
        let eq45_e830_d_n13: f64 = (p.p144 * s.dn[367][13]);
        let eq45_e830_d_n14: f64 = (p.p144 * s.dn[367][14]);
        let eq45_e830_d_n15: f64 = (p.p144 * s.dn[367][15]);
        let eq45_e830_d_n16: f64 = (p.p144 * s.dn[367][16]);
        let eq45_e830_d_n17: f64 = (p.p144 * s.dn[367][17]);
        let eq45_e830_d_n18: f64 = (p.p144 * s.dn[367][18]);
        let eq45_e830_d_n19: f64 = (p.p144 * s.dn[367][19]);
        let eq45_e830_d_n20: f64 = (p.p144 * s.dn[367][20]);
        let eq45_e830_d_n21: f64 = (p.p144 * s.dn[367][21]);
        let eq45_e830_d_n22: f64 = (p.p144 * s.dn[367][22]);
        let eq45_e830_d_b0: f64 = (p.p144 * s.db[367][0]);
        let eq45_e830_d_b1: f64 = (p.p144 * s.db[367][1]);
        let eq45_e830_d_b2: f64 = (p.p144 * s.db[367][2]);
        let eq45_e830_d_b3: f64 = (p.p144 * s.db[367][3]);
        let eq45_e830_d_b4: f64 = (p.p144 * s.db[367][4]);
        let eq45_e830_d_b5: f64 = (p.p144 * s.db[367][5]);
        let eq45_e830_d_b6: f64 = (p.p144 * s.db[367][6]);
        let eq45_e830_d_b7: f64 = (p.p144 * s.db[367][7]);
        let eq45_e830_d_b8: f64 = (p.p144 * s.db[367][8]);
        let eq45_e830_d_b9: f64 = (p.p144 * s.db[367][9]);
        let eq45_e830_d_b10: f64 = (p.p144 * s.db[367][10]);
        let eq45_e830_d_b11: f64 = (p.p144 * s.db[367][11]);
        let eq45_e830_d_b12: f64 = (p.p144 * s.db[367][12]);
        let eq45_e830_d_b13: f64 = (p.p144 * s.db[367][13]);
        let eq45_e830_d_b14: f64 = (p.p144 * s.db[367][14]);
        let eq45_e830_d_b15: f64 = (p.p144 * s.db[367][15]);
        let eq45_e830_d_b16: f64 = (p.p144 * s.db[367][16]);
        let eq45_e830_d_b17: f64 = (p.p144 * s.db[367][17]);
        let eq45_e830_d_b18: f64 = (p.p144 * s.db[367][18]);
        let eq45_e830_d_b19: f64 = (p.p144 * s.db[367][19]);
        let eq45_e830_d_b20: f64 = (p.p144 * s.db[367][20]);
        let eq45_e830_d_b21: f64 = (p.p144 * s.db[367][21]);
        let eq45_e830_d_b22: f64 = (p.p144 * s.db[367][22]);
        let eq45_e830_d_b23: f64 = (p.p144 * s.db[367][23]);
        let eq45_e830_d_b24: f64 = (p.p144 * s.db[367][24]);
        let eq45_e830_d_b25: f64 = (p.p144 * s.db[367][25]);
        let eq45_e830_d_b26: f64 = (p.p144 * s.db[367][26]);
        let eq45_e830_d_b27: f64 = (p.p144 * s.db[367][27]);
        let eq45_e830_d_b28: f64 = (p.p144 * s.db[367][28]);
        let eq45_e830_d_b29: f64 = (p.p144 * s.db[367][29]);
        let eq45_e830_d_b30: f64 = (p.p144 * s.db[367][30]);
        let eq45_e830_d_b31: f64 = (p.p144 * s.db[367][31]);
        let eq45_e830_d_b32: f64 = (p.p144 * s.db[367][32]);
        let eq45_e830_d_b33: f64 = (p.p144 * s.db[367][33]);
        let eq45_e830_d_b34: f64 = (p.p144 * s.db[367][34]);
        let eq45_e830_d_b35: f64 = (p.p144 * s.db[367][35]);
        let eq45_e830_d_b36: f64 = (p.p144 * s.db[367][36]);
        let eq45_e830_d_b37: f64 = (p.p144 * s.db[367][37]);
        let eq45_e830_d_b38: f64 = (p.p144 * s.db[367][38]);
        let eq45_e830_d_b39: f64 = (p.p144 * s.db[367][39]);
        let eq45_e830_d_b40: f64 = (p.p144 * s.db[367][40]);
        let eq45_e830_d_b41: f64 = (p.p144 * s.db[367][41]);
        let eq45_e830_d_b42: f64 = (p.p144 * s.db[367][42]);
        let eq45_e830_d_b43: f64 = (p.p144 * s.db[367][43]);
        let eq45_e830_d_b44: f64 = (p.p144 * s.db[367][44]);
        let eq45_e830_d_b45: f64 = (p.p144 * s.db[367][45]);
        let eq45_e830_d_b46: f64 = (p.p144 * s.db[367][46]);
        let eq45_e830_d_b47: f64 = (p.p144 * s.db[367][47]);
        let eq45_e830_d_b48: f64 = (p.p144 * s.db[367][48]);
        let eq45_e830_d_b49: f64 = (p.p144 * s.db[367][49]);
        let eq45_e830_d_b50: f64 = (p.p144 * s.db[367][50]);
        let eq45_e830_d_b51: f64 = (p.p144 * s.db[367][51]);
        let eq45_e830_d_b52: f64 = (p.p144 * s.db[367][52]);
        let eq45_e830_d_b53: f64 = (p.p144 * s.db[367][53]);
        let eq45_e830_d_b54: f64 = (p.p144 * s.db[367][54]);
        let eq45_e832: f64 = (eq45_e830 * (nv6 - 0.0));
        let eq45_e832_d_n0: f64 = (eq45_e830_d_n0 * (nv6 - 0.0));
        let eq45_e832_d_n1: f64 = (eq45_e830_d_n1 * (nv6 - 0.0));
        let eq45_e832_d_n2: f64 = (eq45_e830_d_n2 * (nv6 - 0.0));
        let eq45_e832_d_n3: f64 = (eq45_e830_d_n3 * (nv6 - 0.0));
        let eq45_e832_d_n4: f64 = (eq45_e830_d_n4 * (nv6 - 0.0));
        let eq45_e832_d_n5: f64 = (eq45_e830_d_n5 * (nv6 - 0.0));
        let eq45_e832_d_n6: f64 = ((eq45_e830_d_n6 * (nv6 - 0.0)) + eq45_e830);
        let eq45_e832_d_n7: f64 = (eq45_e830_d_n7 * (nv6 - 0.0));
        let eq45_e832_d_n8: f64 = (eq45_e830_d_n8 * (nv6 - 0.0));
        let eq45_e832_d_n9: f64 = (eq45_e830_d_n9 * (nv6 - 0.0));
        let eq45_e832_d_n10: f64 = (eq45_e830_d_n10 * (nv6 - 0.0));
        let eq45_e832_d_n11: f64 = (eq45_e830_d_n11 * (nv6 - 0.0));
        let eq45_e832_d_n12: f64 = (eq45_e830_d_n12 * (nv6 - 0.0));
        let eq45_e832_d_n13: f64 = (eq45_e830_d_n13 * (nv6 - 0.0));
        let eq45_e832_d_n14: f64 = (eq45_e830_d_n14 * (nv6 - 0.0));
        let eq45_e832_d_n15: f64 = (eq45_e830_d_n15 * (nv6 - 0.0));
        let eq45_e832_d_n16: f64 = (eq45_e830_d_n16 * (nv6 - 0.0));
        let eq45_e832_d_n17: f64 = (eq45_e830_d_n17 * (nv6 - 0.0));
        let eq45_e832_d_n18: f64 = (eq45_e830_d_n18 * (nv6 - 0.0));
        let eq45_e832_d_n19: f64 = (eq45_e830_d_n19 * (nv6 - 0.0));
        let eq45_e832_d_n20: f64 = (eq45_e830_d_n20 * (nv6 - 0.0));
        let eq45_e832_d_n21: f64 = (eq45_e830_d_n21 * (nv6 - 0.0));
        let eq45_e832_d_n22: f64 = (eq45_e830_d_n22 * (nv6 - 0.0));
        let eq45_e832_d_b0: f64 = (eq45_e830_d_b0 * (nv6 - 0.0));
        let eq45_e832_d_b1: f64 = (eq45_e830_d_b1 * (nv6 - 0.0));
        let eq45_e832_d_b2: f64 = (eq45_e830_d_b2 * (nv6 - 0.0));
        let eq45_e832_d_b3: f64 = (eq45_e830_d_b3 * (nv6 - 0.0));
        let eq45_e832_d_b4: f64 = (eq45_e830_d_b4 * (nv6 - 0.0));
        let eq45_e832_d_b5: f64 = (eq45_e830_d_b5 * (nv6 - 0.0));
        let eq45_e832_d_b6: f64 = (eq45_e830_d_b6 * (nv6 - 0.0));
        let eq45_e832_d_b7: f64 = (eq45_e830_d_b7 * (nv6 - 0.0));
        let eq45_e832_d_b8: f64 = (eq45_e830_d_b8 * (nv6 - 0.0));
        let eq45_e832_d_b9: f64 = (eq45_e830_d_b9 * (nv6 - 0.0));
        let eq45_e832_d_b10: f64 = (eq45_e830_d_b10 * (nv6 - 0.0));
        let eq45_e832_d_b11: f64 = (eq45_e830_d_b11 * (nv6 - 0.0));
        let eq45_e832_d_b12: f64 = (eq45_e830_d_b12 * (nv6 - 0.0));
        let eq45_e832_d_b13: f64 = (eq45_e830_d_b13 * (nv6 - 0.0));
        let eq45_e832_d_b14: f64 = (eq45_e830_d_b14 * (nv6 - 0.0));
        let eq45_e832_d_b15: f64 = (eq45_e830_d_b15 * (nv6 - 0.0));
        let eq45_e832_d_b16: f64 = (eq45_e830_d_b16 * (nv6 - 0.0));
        let eq45_e832_d_b17: f64 = (eq45_e830_d_b17 * (nv6 - 0.0));
        let eq45_e832_d_b18: f64 = (eq45_e830_d_b18 * (nv6 - 0.0));
        let eq45_e832_d_b19: f64 = (eq45_e830_d_b19 * (nv6 - 0.0));
        let eq45_e832_d_b20: f64 = (eq45_e830_d_b20 * (nv6 - 0.0));
        let eq45_e832_d_b21: f64 = (eq45_e830_d_b21 * (nv6 - 0.0));
        let eq45_e832_d_b22: f64 = (eq45_e830_d_b22 * (nv6 - 0.0));
        let eq45_e832_d_b23: f64 = (eq45_e830_d_b23 * (nv6 - 0.0));
        let eq45_e832_d_b24: f64 = (eq45_e830_d_b24 * (nv6 - 0.0));
        let eq45_e832_d_b25: f64 = (eq45_e830_d_b25 * (nv6 - 0.0));
        let eq45_e832_d_b26: f64 = (eq45_e830_d_b26 * (nv6 - 0.0));
        let eq45_e832_d_b27: f64 = (eq45_e830_d_b27 * (nv6 - 0.0));
        let eq45_e832_d_b28: f64 = (eq45_e830_d_b28 * (nv6 - 0.0));
        let eq45_e832_d_b29: f64 = (eq45_e830_d_b29 * (nv6 - 0.0));
        let eq45_e832_d_b30: f64 = (eq45_e830_d_b30 * (nv6 - 0.0));
        let eq45_e832_d_b31: f64 = (eq45_e830_d_b31 * (nv6 - 0.0));
        let eq45_e832_d_b32: f64 = (eq45_e830_d_b32 * (nv6 - 0.0));
        let eq45_e832_d_b33: f64 = (eq45_e830_d_b33 * (nv6 - 0.0));
        let eq45_e832_d_b34: f64 = (eq45_e830_d_b34 * (nv6 - 0.0));
        let eq45_e832_d_b35: f64 = (eq45_e830_d_b35 * (nv6 - 0.0));
        let eq45_e832_d_b36: f64 = (eq45_e830_d_b36 * (nv6 - 0.0));
        let eq45_e832_d_b37: f64 = (eq45_e830_d_b37 * (nv6 - 0.0));
        let eq45_e832_d_b38: f64 = (eq45_e830_d_b38 * (nv6 - 0.0));
        let eq45_e832_d_b39: f64 = (eq45_e830_d_b39 * (nv6 - 0.0));
        let eq45_e832_d_b40: f64 = (eq45_e830_d_b40 * (nv6 - 0.0));
        let eq45_e832_d_b41: f64 = (eq45_e830_d_b41 * (nv6 - 0.0));
        let eq45_e832_d_b42: f64 = (eq45_e830_d_b42 * (nv6 - 0.0));
        let eq45_e832_d_b43: f64 = (eq45_e830_d_b43 * (nv6 - 0.0));
        let eq45_e832_d_b44: f64 = (eq45_e830_d_b44 * (nv6 - 0.0));
        let eq45_e832_d_b45: f64 = (eq45_e830_d_b45 * (nv6 - 0.0));
        let eq45_e832_d_b46: f64 = (eq45_e830_d_b46 * (nv6 - 0.0));
        let eq45_e832_d_b47: f64 = (eq45_e830_d_b47 * (nv6 - 0.0));
        let eq45_e832_d_b48: f64 = (eq45_e830_d_b48 * (nv6 - 0.0));
        let eq45_e832_d_b49: f64 = (eq45_e830_d_b49 * (nv6 - 0.0));
        let eq45_e832_d_b50: f64 = (eq45_e830_d_b50 * (nv6 - 0.0));
        let eq45_e832_d_b51: f64 = (eq45_e830_d_b51 * (nv6 - 0.0));
        let eq45_e832_d_b52: f64 = (eq45_e830_d_b52 * (nv6 - 0.0));
        let eq45_e832_d_b53: f64 = (eq45_e830_d_b53 * (nv6 - 0.0));
        let eq45_e832_d_b54: f64 = (eq45_e830_d_b54 * (nv6 - 0.0));
        (eq45_e832, eq45_e832_d_n0, eq45_e832_d_n1, eq45_e832_d_n2, eq45_e832_d_n3, eq45_e832_d_n4, eq45_e832_d_n5, eq45_e832_d_n6, eq45_e832_d_n7, eq45_e832_d_n8, eq45_e832_d_n9, eq45_e832_d_n10, eq45_e832_d_n11, eq45_e832_d_n12, eq45_e832_d_n13, eq45_e832_d_n14, eq45_e832_d_n15, eq45_e832_d_n16, eq45_e832_d_n17, eq45_e832_d_n18, eq45_e832_d_n19, eq45_e832_d_n20, eq45_e832_d_n21, eq45_e832_d_n22, eq45_e832_d_b0, eq45_e832_d_b1, eq45_e832_d_b2, eq45_e832_d_b3, eq45_e832_d_b4, eq45_e832_d_b5, eq45_e832_d_b6, eq45_e832_d_b7, eq45_e832_d_b8, eq45_e832_d_b9, eq45_e832_d_b10, eq45_e832_d_b11, eq45_e832_d_b12, eq45_e832_d_b13, eq45_e832_d_b14, eq45_e832_d_b15, eq45_e832_d_b16, eq45_e832_d_b17, eq45_e832_d_b18, eq45_e832_d_b19, eq45_e832_d_b20, eq45_e832_d_b21, eq45_e832_d_b22, eq45_e832_d_b23, eq45_e832_d_b24, eq45_e832_d_b25, eq45_e832_d_b26, eq45_e832_d_b27, eq45_e832_d_b28, eq45_e832_d_b29, eq45_e832_d_b30, eq45_e832_d_b31, eq45_e832_d_b32, eq45_e832_d_b33, eq45_e832_d_b34, eq45_e832_d_b35, eq45_e832_d_b36, eq45_e832_d_b37, eq45_e832_d_b38, eq45_e832_d_b39, eq45_e832_d_b40, eq45_e832_d_b41, eq45_e832_d_b42, eq45_e832_d_b43, eq45_e832_d_b44, eq45_e832_d_b45, eq45_e832_d_b46, eq45_e832_d_b47, eq45_e832_d_b48, eq45_e832_d_b49, eq45_e832_d_b50, eq45_e832_d_b51, eq45_e832_d_b52, eq45_e832_d_b53, eq45_e832_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e834;
        let eq45_node_derivatives: [f64; 23] = [eq45_e834_d_n0, eq45_e834_d_n1, eq45_e834_d_n2, eq45_e834_d_n3, eq45_e834_d_n4, eq45_e834_d_n5, eq45_e834_d_n6, eq45_e834_d_n7, eq45_e834_d_n8, eq45_e834_d_n9, eq45_e834_d_n10, eq45_e834_d_n11, eq45_e834_d_n12, eq45_e834_d_n13, eq45_e834_d_n14, eq45_e834_d_n15, eq45_e834_d_n16, eq45_e834_d_n17, eq45_e834_d_n18, eq45_e834_d_n19, eq45_e834_d_n20, eq45_e834_d_n21, eq45_e834_d_n22];
        let eq45_branch_derivatives: [f64; 55] = [eq45_e834_d_b0, eq45_e834_d_b1, eq45_e834_d_b2, eq45_e834_d_b3, eq45_e834_d_b4, eq45_e834_d_b5, eq45_e834_d_b6, eq45_e834_d_b7, eq45_e834_d_b8, eq45_e834_d_b9, eq45_e834_d_b10, eq45_e834_d_b11, eq45_e834_d_b12, eq45_e834_d_b13, eq45_e834_d_b14, eq45_e834_d_b15, eq45_e834_d_b16, eq45_e834_d_b17, eq45_e834_d_b18, eq45_e834_d_b19, eq45_e834_d_b20, eq45_e834_d_b21, eq45_e834_d_b22, eq45_e834_d_b23, eq45_e834_d_b24, eq45_e834_d_b25, eq45_e834_d_b26, eq45_e834_d_b27, eq45_e834_d_b28, eq45_e834_d_b29, eq45_e834_d_b30, eq45_e834_d_b31, eq45_e834_d_b32, eq45_e834_d_b33, eq45_e834_d_b34, eq45_e834_d_b35, eq45_e834_d_b36, eq45_e834_d_b37, eq45_e834_d_b38, eq45_e834_d_b39, eq45_e834_d_b40, eq45_e834_d_b41, eq45_e834_d_b42, eq45_e834_d_b43, eq45_e834_d_b44, eq45_e834_d_b45, eq45_e834_d_b46, eq45_e834_d_b47, eq45_e834_d_b48, eq45_e834_d_b49, eq45_e834_d_b50, eq45_e834_d_b51, eq45_e834_d_b52, eq45_e834_d_b53, eq45_e834_d_b54];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let (eq46_e852, eq46_e852_d_n6,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq46_e849: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, (nv6 - 0.0));
        let eq46_e850: f64 = (p.p144 * eq46_e849);
        let eq46_e850_d_n6: f64 = (p.p144 * ddt_scale);
        (eq46_e850, eq46_e850_d_n6,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e852;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (eq46_value),
            6,
            multiplicity * (eq46_e852_d_n6),
        );
        let (eq47_e867,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e867;
        stamper.stamp_potential_const_local(
            25,
            eq47_value,
        );
        let (eq48_e882,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e882;
        stamper.stamp_potential_const_local(
            26,
            eq48_value,
        );
        let (eq49_e897,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e897;
        stamper.stamp_potential_const_local(
            27,
            eq49_value,
        );
        let (eq50_e912,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e912;
        stamper.stamp_potential_const_local(
            28,
            eq50_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq51_e915: f64 = (p.p6 * s.v[41]);
        let eq51_e915_d_n0: f64 = (p.p6 * s.dn[41][0]);
        let eq51_e915_d_n1: f64 = (p.p6 * s.dn[41][1]);
        let eq51_e915_d_n2: f64 = (p.p6 * s.dn[41][2]);
        let eq51_e915_d_n3: f64 = (p.p6 * s.dn[41][3]);
        let eq51_e915_d_n4: f64 = (p.p6 * s.dn[41][4]);
        let eq51_e915_d_n5: f64 = (p.p6 * s.dn[41][5]);
        let eq51_e915_d_n6: f64 = (p.p6 * s.dn[41][6]);
        let eq51_e915_d_n7: f64 = (p.p6 * s.dn[41][7]);
        let eq51_e915_d_n8: f64 = (p.p6 * s.dn[41][8]);
        let eq51_e915_d_n9: f64 = (p.p6 * s.dn[41][9]);
        let eq51_e915_d_n10: f64 = (p.p6 * s.dn[41][10]);
        let eq51_e915_d_n11: f64 = (p.p6 * s.dn[41][11]);
        let eq51_e915_d_n12: f64 = (p.p6 * s.dn[41][12]);
        let eq51_e915_d_n13: f64 = (p.p6 * s.dn[41][13]);
        let eq51_e915_d_n14: f64 = (p.p6 * s.dn[41][14]);
        let eq51_e915_d_n15: f64 = (p.p6 * s.dn[41][15]);
        let eq51_e915_d_n16: f64 = (p.p6 * s.dn[41][16]);
        let eq51_e915_d_n17: f64 = (p.p6 * s.dn[41][17]);
        let eq51_e915_d_n18: f64 = (p.p6 * s.dn[41][18]);
        let eq51_e915_d_n19: f64 = (p.p6 * s.dn[41][19]);
        let eq51_e915_d_n20: f64 = (p.p6 * s.dn[41][20]);
        let eq51_e915_d_n21: f64 = (p.p6 * s.dn[41][21]);
        let eq51_e915_d_n22: f64 = (p.p6 * s.dn[41][22]);
        let eq51_e915_d_b0: f64 = (p.p6 * s.db[41][0]);
        let eq51_e915_d_b1: f64 = (p.p6 * s.db[41][1]);
        let eq51_e915_d_b2: f64 = (p.p6 * s.db[41][2]);
        let eq51_e915_d_b3: f64 = (p.p6 * s.db[41][3]);
        let eq51_e915_d_b4: f64 = (p.p6 * s.db[41][4]);
        let eq51_e915_d_b5: f64 = (p.p6 * s.db[41][5]);
        let eq51_e915_d_b6: f64 = (p.p6 * s.db[41][6]);
        let eq51_e915_d_b7: f64 = (p.p6 * s.db[41][7]);
        let eq51_e915_d_b8: f64 = (p.p6 * s.db[41][8]);
        let eq51_e915_d_b9: f64 = (p.p6 * s.db[41][9]);
        let eq51_e915_d_b10: f64 = (p.p6 * s.db[41][10]);
        let eq51_e915_d_b11: f64 = (p.p6 * s.db[41][11]);
        let eq51_e915_d_b12: f64 = (p.p6 * s.db[41][12]);
        let eq51_e915_d_b13: f64 = (p.p6 * s.db[41][13]);
        let eq51_e915_d_b14: f64 = (p.p6 * s.db[41][14]);
        let eq51_e915_d_b15: f64 = (p.p6 * s.db[41][15]);
        let eq51_e915_d_b16: f64 = (p.p6 * s.db[41][16]);
        let eq51_e915_d_b17: f64 = (p.p6 * s.db[41][17]);
        let eq51_e915_d_b18: f64 = (p.p6 * s.db[41][18]);
        let eq51_e915_d_b19: f64 = (p.p6 * s.db[41][19]);
        let eq51_e915_d_b20: f64 = (p.p6 * s.db[41][20]);
        let eq51_e915_d_b21: f64 = (p.p6 * s.db[41][21]);
        let eq51_e915_d_b22: f64 = (p.p6 * s.db[41][22]);
        let eq51_e915_d_b23: f64 = (p.p6 * s.db[41][23]);
        let eq51_e915_d_b24: f64 = (p.p6 * s.db[41][24]);
        let eq51_e915_d_b25: f64 = (p.p6 * s.db[41][25]);
        let eq51_e915_d_b26: f64 = (p.p6 * s.db[41][26]);
        let eq51_e915_d_b27: f64 = (p.p6 * s.db[41][27]);
        let eq51_e915_d_b28: f64 = (p.p6 * s.db[41][28]);
        let eq51_e915_d_b29: f64 = (p.p6 * s.db[41][29]);
        let eq51_e915_d_b30: f64 = (p.p6 * s.db[41][30]);
        let eq51_e915_d_b31: f64 = (p.p6 * s.db[41][31]);
        let eq51_e915_d_b32: f64 = (p.p6 * s.db[41][32]);
        let eq51_e915_d_b33: f64 = (p.p6 * s.db[41][33]);
        let eq51_e915_d_b34: f64 = (p.p6 * s.db[41][34]);
        let eq51_e915_d_b35: f64 = (p.p6 * s.db[41][35]);
        let eq51_e915_d_b36: f64 = (p.p6 * s.db[41][36]);
        let eq51_e915_d_b37: f64 = (p.p6 * s.db[41][37]);
        let eq51_e915_d_b38: f64 = (p.p6 * s.db[41][38]);
        let eq51_e915_d_b39: f64 = (p.p6 * s.db[41][39]);
        let eq51_e915_d_b40: f64 = (p.p6 * s.db[41][40]);
        let eq51_e915_d_b41: f64 = (p.p6 * s.db[41][41]);
        let eq51_e915_d_b42: f64 = (p.p6 * s.db[41][42]);
        let eq51_e915_d_b43: f64 = (p.p6 * s.db[41][43]);
        let eq51_e915_d_b44: f64 = (p.p6 * s.db[41][44]);
        let eq51_e915_d_b45: f64 = (p.p6 * s.db[41][45]);
        let eq51_e915_d_b46: f64 = (p.p6 * s.db[41][46]);
        let eq51_e915_d_b47: f64 = (p.p6 * s.db[41][47]);
        let eq51_e915_d_b48: f64 = (p.p6 * s.db[41][48]);
        let eq51_e915_d_b49: f64 = (p.p6 * s.db[41][49]);
        let eq51_e915_d_b50: f64 = (p.p6 * s.db[41][50]);
        let eq51_e915_d_b51: f64 = (p.p6 * s.db[41][51]);
        let eq51_e915_d_b52: f64 = (p.p6 * s.db[41][52]);
        let eq51_e915_d_b53: f64 = (p.p6 * s.db[41][53]);
        let eq51_e915_d_b54: f64 = (p.p6 * s.db[41][54]);
        let eq51_e917: f64 = (eq51_e915 * s.v[94]);
        let eq51_e917_d_n0: f64 = ((eq51_e915_d_n0 * s.v[94]) + (eq51_e915 * s.dn[94][0]));
        let eq51_e917_d_n1: f64 = ((eq51_e915_d_n1 * s.v[94]) + (eq51_e915 * s.dn[94][1]));
        let eq51_e917_d_n2: f64 = ((eq51_e915_d_n2 * s.v[94]) + (eq51_e915 * s.dn[94][2]));
        let eq51_e917_d_n3: f64 = ((eq51_e915_d_n3 * s.v[94]) + (eq51_e915 * s.dn[94][3]));
        let eq51_e917_d_n4: f64 = ((eq51_e915_d_n4 * s.v[94]) + (eq51_e915 * s.dn[94][4]));
        let eq51_e917_d_n5: f64 = ((eq51_e915_d_n5 * s.v[94]) + (eq51_e915 * s.dn[94][5]));
        let eq51_e917_d_n6: f64 = ((eq51_e915_d_n6 * s.v[94]) + (eq51_e915 * s.dn[94][6]));
        let eq51_e917_d_n7: f64 = ((eq51_e915_d_n7 * s.v[94]) + (eq51_e915 * s.dn[94][7]));
        let eq51_e917_d_n8: f64 = ((eq51_e915_d_n8 * s.v[94]) + (eq51_e915 * s.dn[94][8]));
        let eq51_e917_d_n9: f64 = ((eq51_e915_d_n9 * s.v[94]) + (eq51_e915 * s.dn[94][9]));
        let eq51_e917_d_n10: f64 = ((eq51_e915_d_n10 * s.v[94]) + (eq51_e915 * s.dn[94][10]));
        let eq51_e917_d_n11: f64 = ((eq51_e915_d_n11 * s.v[94]) + (eq51_e915 * s.dn[94][11]));
        let eq51_e917_d_n12: f64 = ((eq51_e915_d_n12 * s.v[94]) + (eq51_e915 * s.dn[94][12]));
        let eq51_e917_d_n13: f64 = ((eq51_e915_d_n13 * s.v[94]) + (eq51_e915 * s.dn[94][13]));
        let eq51_e917_d_n14: f64 = ((eq51_e915_d_n14 * s.v[94]) + (eq51_e915 * s.dn[94][14]));
        let eq51_e917_d_n15: f64 = ((eq51_e915_d_n15 * s.v[94]) + (eq51_e915 * s.dn[94][15]));
        let eq51_e917_d_n16: f64 = ((eq51_e915_d_n16 * s.v[94]) + (eq51_e915 * s.dn[94][16]));
        let eq51_e917_d_n17: f64 = ((eq51_e915_d_n17 * s.v[94]) + (eq51_e915 * s.dn[94][17]));
        let eq51_e917_d_n18: f64 = ((eq51_e915_d_n18 * s.v[94]) + (eq51_e915 * s.dn[94][18]));
        let eq51_e917_d_n19: f64 = ((eq51_e915_d_n19 * s.v[94]) + (eq51_e915 * s.dn[94][19]));
        let eq51_e917_d_n20: f64 = ((eq51_e915_d_n20 * s.v[94]) + (eq51_e915 * s.dn[94][20]));
        let eq51_e917_d_n21: f64 = ((eq51_e915_d_n21 * s.v[94]) + (eq51_e915 * s.dn[94][21]));
        let eq51_e917_d_n22: f64 = ((eq51_e915_d_n22 * s.v[94]) + (eq51_e915 * s.dn[94][22]));
        let eq51_e917_d_b0: f64 = ((eq51_e915_d_b0 * s.v[94]) + (eq51_e915 * s.db[94][0]));
        let eq51_e917_d_b1: f64 = ((eq51_e915_d_b1 * s.v[94]) + (eq51_e915 * s.db[94][1]));
        let eq51_e917_d_b2: f64 = ((eq51_e915_d_b2 * s.v[94]) + (eq51_e915 * s.db[94][2]));
        let eq51_e917_d_b3: f64 = ((eq51_e915_d_b3 * s.v[94]) + (eq51_e915 * s.db[94][3]));
        let eq51_e917_d_b4: f64 = ((eq51_e915_d_b4 * s.v[94]) + (eq51_e915 * s.db[94][4]));
        let eq51_e917_d_b5: f64 = ((eq51_e915_d_b5 * s.v[94]) + (eq51_e915 * s.db[94][5]));
        let eq51_e917_d_b6: f64 = ((eq51_e915_d_b6 * s.v[94]) + (eq51_e915 * s.db[94][6]));
        let eq51_e917_d_b7: f64 = ((eq51_e915_d_b7 * s.v[94]) + (eq51_e915 * s.db[94][7]));
        let eq51_e917_d_b8: f64 = ((eq51_e915_d_b8 * s.v[94]) + (eq51_e915 * s.db[94][8]));
        let eq51_e917_d_b9: f64 = ((eq51_e915_d_b9 * s.v[94]) + (eq51_e915 * s.db[94][9]));
        let eq51_e917_d_b10: f64 = ((eq51_e915_d_b10 * s.v[94]) + (eq51_e915 * s.db[94][10]));
        let eq51_e917_d_b11: f64 = ((eq51_e915_d_b11 * s.v[94]) + (eq51_e915 * s.db[94][11]));
        let eq51_e917_d_b12: f64 = ((eq51_e915_d_b12 * s.v[94]) + (eq51_e915 * s.db[94][12]));
        let eq51_e917_d_b13: f64 = ((eq51_e915_d_b13 * s.v[94]) + (eq51_e915 * s.db[94][13]));
        let eq51_e917_d_b14: f64 = ((eq51_e915_d_b14 * s.v[94]) + (eq51_e915 * s.db[94][14]));
        let eq51_e917_d_b15: f64 = ((eq51_e915_d_b15 * s.v[94]) + (eq51_e915 * s.db[94][15]));
        let eq51_e917_d_b16: f64 = ((eq51_e915_d_b16 * s.v[94]) + (eq51_e915 * s.db[94][16]));
        let eq51_e917_d_b17: f64 = ((eq51_e915_d_b17 * s.v[94]) + (eq51_e915 * s.db[94][17]));
        let eq51_e917_d_b18: f64 = ((eq51_e915_d_b18 * s.v[94]) + (eq51_e915 * s.db[94][18]));
        let eq51_e917_d_b19: f64 = ((eq51_e915_d_b19 * s.v[94]) + (eq51_e915 * s.db[94][19]));
        let eq51_e917_d_b20: f64 = ((eq51_e915_d_b20 * s.v[94]) + (eq51_e915 * s.db[94][20]));
        let eq51_e917_d_b21: f64 = ((eq51_e915_d_b21 * s.v[94]) + (eq51_e915 * s.db[94][21]));
        let eq51_e917_d_b22: f64 = ((eq51_e915_d_b22 * s.v[94]) + (eq51_e915 * s.db[94][22]));
        let eq51_e917_d_b23: f64 = ((eq51_e915_d_b23 * s.v[94]) + (eq51_e915 * s.db[94][23]));
        let eq51_e917_d_b24: f64 = ((eq51_e915_d_b24 * s.v[94]) + (eq51_e915 * s.db[94][24]));
        let eq51_e917_d_b25: f64 = ((eq51_e915_d_b25 * s.v[94]) + (eq51_e915 * s.db[94][25]));
        let eq51_e917_d_b26: f64 = ((eq51_e915_d_b26 * s.v[94]) + (eq51_e915 * s.db[94][26]));
        let eq51_e917_d_b27: f64 = ((eq51_e915_d_b27 * s.v[94]) + (eq51_e915 * s.db[94][27]));
        let eq51_e917_d_b28: f64 = ((eq51_e915_d_b28 * s.v[94]) + (eq51_e915 * s.db[94][28]));
        let eq51_e917_d_b29: f64 = ((eq51_e915_d_b29 * s.v[94]) + (eq51_e915 * s.db[94][29]));
        let eq51_e917_d_b30: f64 = ((eq51_e915_d_b30 * s.v[94]) + (eq51_e915 * s.db[94][30]));
        let eq51_e917_d_b31: f64 = ((eq51_e915_d_b31 * s.v[94]) + (eq51_e915 * s.db[94][31]));
        let eq51_e917_d_b32: f64 = ((eq51_e915_d_b32 * s.v[94]) + (eq51_e915 * s.db[94][32]));
        let eq51_e917_d_b33: f64 = ((eq51_e915_d_b33 * s.v[94]) + (eq51_e915 * s.db[94][33]));
        let eq51_e917_d_b34: f64 = ((eq51_e915_d_b34 * s.v[94]) + (eq51_e915 * s.db[94][34]));
        let eq51_e917_d_b35: f64 = ((eq51_e915_d_b35 * s.v[94]) + (eq51_e915 * s.db[94][35]));
        let eq51_e917_d_b36: f64 = ((eq51_e915_d_b36 * s.v[94]) + (eq51_e915 * s.db[94][36]));
        let eq51_e917_d_b37: f64 = ((eq51_e915_d_b37 * s.v[94]) + (eq51_e915 * s.db[94][37]));
        let eq51_e917_d_b38: f64 = ((eq51_e915_d_b38 * s.v[94]) + (eq51_e915 * s.db[94][38]));
        let eq51_e917_d_b39: f64 = ((eq51_e915_d_b39 * s.v[94]) + (eq51_e915 * s.db[94][39]));
        let eq51_e917_d_b40: f64 = ((eq51_e915_d_b40 * s.v[94]) + (eq51_e915 * s.db[94][40]));
        let eq51_e917_d_b41: f64 = ((eq51_e915_d_b41 * s.v[94]) + (eq51_e915 * s.db[94][41]));
        let eq51_e917_d_b42: f64 = ((eq51_e915_d_b42 * s.v[94]) + (eq51_e915 * s.db[94][42]));
        let eq51_e917_d_b43: f64 = ((eq51_e915_d_b43 * s.v[94]) + (eq51_e915 * s.db[94][43]));
        let eq51_e917_d_b44: f64 = ((eq51_e915_d_b44 * s.v[94]) + (eq51_e915 * s.db[94][44]));
        let eq51_e917_d_b45: f64 = ((eq51_e915_d_b45 * s.v[94]) + (eq51_e915 * s.db[94][45]));
        let eq51_e917_d_b46: f64 = ((eq51_e915_d_b46 * s.v[94]) + (eq51_e915 * s.db[94][46]));
        let eq51_e917_d_b47: f64 = ((eq51_e915_d_b47 * s.v[94]) + (eq51_e915 * s.db[94][47]));
        let eq51_e917_d_b48: f64 = ((eq51_e915_d_b48 * s.v[94]) + (eq51_e915 * s.db[94][48]));
        let eq51_e917_d_b49: f64 = ((eq51_e915_d_b49 * s.v[94]) + (eq51_e915 * s.db[94][49]));
        let eq51_e917_d_b50: f64 = ((eq51_e915_d_b50 * s.v[94]) + (eq51_e915 * s.db[94][50]));
        let eq51_e917_d_b51: f64 = ((eq51_e915_d_b51 * s.v[94]) + (eq51_e915 * s.db[94][51]));
        let eq51_e917_d_b52: f64 = ((eq51_e915_d_b52 * s.v[94]) + (eq51_e915 * s.db[94][52]));
        let eq51_e917_d_b53: f64 = ((eq51_e915_d_b53 * s.v[94]) + (eq51_e915 * s.db[94][53]));
        let eq51_e917_d_b54: f64 = ((eq51_e915_d_b54 * s.v[94]) + (eq51_e915 * s.db[94][54]));
        let eq51_e920: f64 = (p.p6 * s.v[379]);
        let eq51_e920_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq51_e920_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq51_e920_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq51_e920_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq51_e920_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq51_e920_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq51_e920_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq51_e920_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq51_e920_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq51_e920_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq51_e920_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq51_e920_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq51_e920_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq51_e920_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq51_e920_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq51_e920_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq51_e920_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq51_e920_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq51_e920_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq51_e920_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq51_e920_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq51_e920_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq51_e920_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq51_e920_d_b0: f64 = (p.p6 * s.db[379][0]);
        let eq51_e920_d_b1: f64 = (p.p6 * s.db[379][1]);
        let eq51_e920_d_b2: f64 = (p.p6 * s.db[379][2]);
        let eq51_e920_d_b3: f64 = (p.p6 * s.db[379][3]);
        let eq51_e920_d_b4: f64 = (p.p6 * s.db[379][4]);
        let eq51_e920_d_b5: f64 = (p.p6 * s.db[379][5]);
        let eq51_e920_d_b6: f64 = (p.p6 * s.db[379][6]);
        let eq51_e920_d_b7: f64 = (p.p6 * s.db[379][7]);
        let eq51_e920_d_b8: f64 = (p.p6 * s.db[379][8]);
        let eq51_e920_d_b9: f64 = (p.p6 * s.db[379][9]);
        let eq51_e920_d_b10: f64 = (p.p6 * s.db[379][10]);
        let eq51_e920_d_b11: f64 = (p.p6 * s.db[379][11]);
        let eq51_e920_d_b12: f64 = (p.p6 * s.db[379][12]);
        let eq51_e920_d_b13: f64 = (p.p6 * s.db[379][13]);
        let eq51_e920_d_b14: f64 = (p.p6 * s.db[379][14]);
        let eq51_e920_d_b15: f64 = (p.p6 * s.db[379][15]);
        let eq51_e920_d_b16: f64 = (p.p6 * s.db[379][16]);
        let eq51_e920_d_b17: f64 = (p.p6 * s.db[379][17]);
        let eq51_e920_d_b18: f64 = (p.p6 * s.db[379][18]);
        let eq51_e920_d_b19: f64 = (p.p6 * s.db[379][19]);
        let eq51_e920_d_b20: f64 = (p.p6 * s.db[379][20]);
        let eq51_e920_d_b21: f64 = (p.p6 * s.db[379][21]);
        let eq51_e920_d_b22: f64 = (p.p6 * s.db[379][22]);
        let eq51_e920_d_b23: f64 = (p.p6 * s.db[379][23]);
        let eq51_e920_d_b24: f64 = (p.p6 * s.db[379][24]);
        let eq51_e920_d_b25: f64 = (p.p6 * s.db[379][25]);
        let eq51_e920_d_b26: f64 = (p.p6 * s.db[379][26]);
        let eq51_e920_d_b27: f64 = (p.p6 * s.db[379][27]);
        let eq51_e920_d_b28: f64 = (p.p6 * s.db[379][28]);
        let eq51_e920_d_b29: f64 = (p.p6 * s.db[379][29]);
        let eq51_e920_d_b30: f64 = (p.p6 * s.db[379][30]);
        let eq51_e920_d_b31: f64 = (p.p6 * s.db[379][31]);
        let eq51_e920_d_b32: f64 = (p.p6 * s.db[379][32]);
        let eq51_e920_d_b33: f64 = (p.p6 * s.db[379][33]);
        let eq51_e920_d_b34: f64 = (p.p6 * s.db[379][34]);
        let eq51_e920_d_b35: f64 = (p.p6 * s.db[379][35]);
        let eq51_e920_d_b36: f64 = (p.p6 * s.db[379][36]);
        let eq51_e920_d_b37: f64 = (p.p6 * s.db[379][37]);
        let eq51_e920_d_b38: f64 = (p.p6 * s.db[379][38]);
        let eq51_e920_d_b39: f64 = (p.p6 * s.db[379][39]);
        let eq51_e920_d_b40: f64 = (p.p6 * s.db[379][40]);
        let eq51_e920_d_b41: f64 = (p.p6 * s.db[379][41]);
        let eq51_e920_d_b42: f64 = (p.p6 * s.db[379][42]);
        let eq51_e920_d_b43: f64 = (p.p6 * s.db[379][43]);
        let eq51_e920_d_b44: f64 = (p.p6 * s.db[379][44]);
        let eq51_e920_d_b45: f64 = (p.p6 * s.db[379][45]);
        let eq51_e920_d_b46: f64 = (p.p6 * s.db[379][46]);
        let eq51_e920_d_b47: f64 = (p.p6 * s.db[379][47]);
        let eq51_e920_d_b48: f64 = (p.p6 * s.db[379][48]);
        let eq51_e920_d_b49: f64 = (p.p6 * s.db[379][49]);
        let eq51_e920_d_b50: f64 = (p.p6 * s.db[379][50]);
        let eq51_e920_d_b51: f64 = (p.p6 * s.db[379][51]);
        let eq51_e920_d_b52: f64 = (p.p6 * s.db[379][52]);
        let eq51_e920_d_b53: f64 = (p.p6 * s.db[379][53]);
        let eq51_e920_d_b54: f64 = (p.p6 * s.db[379][54]);
        let eq51_e922: f64 = (eq51_e920 * (nv7 - nv8));
        let eq51_e922_d_n0: f64 = (eq51_e920_d_n0 * (nv7 - nv8));
        let eq51_e922_d_n1: f64 = (eq51_e920_d_n1 * (nv7 - nv8));
        let eq51_e922_d_n2: f64 = (eq51_e920_d_n2 * (nv7 - nv8));
        let eq51_e922_d_n3: f64 = (eq51_e920_d_n3 * (nv7 - nv8));
        let eq51_e922_d_n4: f64 = (eq51_e920_d_n4 * (nv7 - nv8));
        let eq51_e922_d_n5: f64 = (eq51_e920_d_n5 * (nv7 - nv8));
        let eq51_e922_d_n6: f64 = (eq51_e920_d_n6 * (nv7 - nv8));
        let eq51_e922_d_n7: f64 = ((eq51_e920_d_n7 * (nv7 - nv8)) + eq51_e920);
        let eq51_e922_d_n8: f64 = ((eq51_e920_d_n8 * (nv7 - nv8)) + (-eq51_e920));
        let eq51_e922_d_n9: f64 = (eq51_e920_d_n9 * (nv7 - nv8));
        let eq51_e922_d_n10: f64 = (eq51_e920_d_n10 * (nv7 - nv8));
        let eq51_e922_d_n11: f64 = (eq51_e920_d_n11 * (nv7 - nv8));
        let eq51_e922_d_n12: f64 = (eq51_e920_d_n12 * (nv7 - nv8));
        let eq51_e922_d_n13: f64 = (eq51_e920_d_n13 * (nv7 - nv8));
        let eq51_e922_d_n14: f64 = (eq51_e920_d_n14 * (nv7 - nv8));
        let eq51_e922_d_n15: f64 = (eq51_e920_d_n15 * (nv7 - nv8));
        let eq51_e922_d_n16: f64 = (eq51_e920_d_n16 * (nv7 - nv8));
        let eq51_e922_d_n17: f64 = (eq51_e920_d_n17 * (nv7 - nv8));
        let eq51_e922_d_n18: f64 = (eq51_e920_d_n18 * (nv7 - nv8));
        let eq51_e922_d_n19: f64 = (eq51_e920_d_n19 * (nv7 - nv8));
        let eq51_e922_d_n20: f64 = (eq51_e920_d_n20 * (nv7 - nv8));
        let eq51_e922_d_n21: f64 = (eq51_e920_d_n21 * (nv7 - nv8));
        let eq51_e922_d_n22: f64 = (eq51_e920_d_n22 * (nv7 - nv8));
        let eq51_e922_d_b0: f64 = (eq51_e920_d_b0 * (nv7 - nv8));
        let eq51_e922_d_b1: f64 = (eq51_e920_d_b1 * (nv7 - nv8));
        let eq51_e922_d_b2: f64 = (eq51_e920_d_b2 * (nv7 - nv8));
        let eq51_e922_d_b3: f64 = (eq51_e920_d_b3 * (nv7 - nv8));
        let eq51_e922_d_b4: f64 = (eq51_e920_d_b4 * (nv7 - nv8));
        let eq51_e922_d_b5: f64 = (eq51_e920_d_b5 * (nv7 - nv8));
        let eq51_e922_d_b6: f64 = (eq51_e920_d_b6 * (nv7 - nv8));
        let eq51_e922_d_b7: f64 = (eq51_e920_d_b7 * (nv7 - nv8));
        let eq51_e922_d_b8: f64 = (eq51_e920_d_b8 * (nv7 - nv8));
        let eq51_e922_d_b9: f64 = (eq51_e920_d_b9 * (nv7 - nv8));
        let eq51_e922_d_b10: f64 = (eq51_e920_d_b10 * (nv7 - nv8));
        let eq51_e922_d_b11: f64 = (eq51_e920_d_b11 * (nv7 - nv8));
        let eq51_e922_d_b12: f64 = (eq51_e920_d_b12 * (nv7 - nv8));
        let eq51_e922_d_b13: f64 = (eq51_e920_d_b13 * (nv7 - nv8));
        let eq51_e922_d_b14: f64 = (eq51_e920_d_b14 * (nv7 - nv8));
        let eq51_e922_d_b15: f64 = (eq51_e920_d_b15 * (nv7 - nv8));
        let eq51_e922_d_b16: f64 = (eq51_e920_d_b16 * (nv7 - nv8));
        let eq51_e922_d_b17: f64 = (eq51_e920_d_b17 * (nv7 - nv8));
        let eq51_e922_d_b18: f64 = (eq51_e920_d_b18 * (nv7 - nv8));
        let eq51_e922_d_b19: f64 = (eq51_e920_d_b19 * (nv7 - nv8));
        let eq51_e922_d_b20: f64 = (eq51_e920_d_b20 * (nv7 - nv8));
        let eq51_e922_d_b21: f64 = (eq51_e920_d_b21 * (nv7 - nv8));
        let eq51_e922_d_b22: f64 = (eq51_e920_d_b22 * (nv7 - nv8));
        let eq51_e922_d_b23: f64 = (eq51_e920_d_b23 * (nv7 - nv8));
        let eq51_e922_d_b24: f64 = (eq51_e920_d_b24 * (nv7 - nv8));
        let eq51_e922_d_b25: f64 = (eq51_e920_d_b25 * (nv7 - nv8));
        let eq51_e922_d_b26: f64 = (eq51_e920_d_b26 * (nv7 - nv8));
        let eq51_e922_d_b27: f64 = (eq51_e920_d_b27 * (nv7 - nv8));
        let eq51_e922_d_b28: f64 = (eq51_e920_d_b28 * (nv7 - nv8));
        let eq51_e922_d_b29: f64 = (eq51_e920_d_b29 * (nv7 - nv8));
        let eq51_e922_d_b30: f64 = (eq51_e920_d_b30 * (nv7 - nv8));
        let eq51_e922_d_b31: f64 = (eq51_e920_d_b31 * (nv7 - nv8));
        let eq51_e922_d_b32: f64 = (eq51_e920_d_b32 * (nv7 - nv8));
        let eq51_e922_d_b33: f64 = (eq51_e920_d_b33 * (nv7 - nv8));
        let eq51_e922_d_b34: f64 = (eq51_e920_d_b34 * (nv7 - nv8));
        let eq51_e922_d_b35: f64 = (eq51_e920_d_b35 * (nv7 - nv8));
        let eq51_e922_d_b36: f64 = (eq51_e920_d_b36 * (nv7 - nv8));
        let eq51_e922_d_b37: f64 = (eq51_e920_d_b37 * (nv7 - nv8));
        let eq51_e922_d_b38: f64 = (eq51_e920_d_b38 * (nv7 - nv8));
        let eq51_e922_d_b39: f64 = (eq51_e920_d_b39 * (nv7 - nv8));
        let eq51_e922_d_b40: f64 = (eq51_e920_d_b40 * (nv7 - nv8));
        let eq51_e922_d_b41: f64 = (eq51_e920_d_b41 * (nv7 - nv8));
        let eq51_e922_d_b42: f64 = (eq51_e920_d_b42 * (nv7 - nv8));
        let eq51_e922_d_b43: f64 = (eq51_e920_d_b43 * (nv7 - nv8));
        let eq51_e922_d_b44: f64 = (eq51_e920_d_b44 * (nv7 - nv8));
        let eq51_e922_d_b45: f64 = (eq51_e920_d_b45 * (nv7 - nv8));
        let eq51_e922_d_b46: f64 = (eq51_e920_d_b46 * (nv7 - nv8));
        let eq51_e922_d_b47: f64 = (eq51_e920_d_b47 * (nv7 - nv8));
        let eq51_e922_d_b48: f64 = (eq51_e920_d_b48 * (nv7 - nv8));
        let eq51_e922_d_b49: f64 = (eq51_e920_d_b49 * (nv7 - nv8));
        let eq51_e922_d_b50: f64 = (eq51_e920_d_b50 * (nv7 - nv8));
        let eq51_e922_d_b51: f64 = (eq51_e920_d_b51 * (nv7 - nv8));
        let eq51_e922_d_b52: f64 = (eq51_e920_d_b52 * (nv7 - nv8));
        let eq51_e922_d_b53: f64 = (eq51_e920_d_b53 * (nv7 - nv8));
        let eq51_e922_d_b54: f64 = (eq51_e920_d_b54 * (nv7 - nv8));
        let eq51_e923: f64 = (eq51_e917 + eq51_e922);
        let eq51_e923_d_n0: f64 = (eq51_e917_d_n0 + eq51_e922_d_n0);
        let eq51_e923_d_n1: f64 = (eq51_e917_d_n1 + eq51_e922_d_n1);
        let eq51_e923_d_n2: f64 = (eq51_e917_d_n2 + eq51_e922_d_n2);
        let eq51_e923_d_n3: f64 = (eq51_e917_d_n3 + eq51_e922_d_n3);
        let eq51_e923_d_n4: f64 = (eq51_e917_d_n4 + eq51_e922_d_n4);
        let eq51_e923_d_n5: f64 = (eq51_e917_d_n5 + eq51_e922_d_n5);
        let eq51_e923_d_n6: f64 = (eq51_e917_d_n6 + eq51_e922_d_n6);
        let eq51_e923_d_n7: f64 = (eq51_e917_d_n7 + eq51_e922_d_n7);
        let eq51_e923_d_n8: f64 = (eq51_e917_d_n8 + eq51_e922_d_n8);
        let eq51_e923_d_n9: f64 = (eq51_e917_d_n9 + eq51_e922_d_n9);
        let eq51_e923_d_n10: f64 = (eq51_e917_d_n10 + eq51_e922_d_n10);
        let eq51_e923_d_n11: f64 = (eq51_e917_d_n11 + eq51_e922_d_n11);
        let eq51_e923_d_n12: f64 = (eq51_e917_d_n12 + eq51_e922_d_n12);
        let eq51_e923_d_n13: f64 = (eq51_e917_d_n13 + eq51_e922_d_n13);
        let eq51_e923_d_n14: f64 = (eq51_e917_d_n14 + eq51_e922_d_n14);
        let eq51_e923_d_n15: f64 = (eq51_e917_d_n15 + eq51_e922_d_n15);
        let eq51_e923_d_n16: f64 = (eq51_e917_d_n16 + eq51_e922_d_n16);
        let eq51_e923_d_n17: f64 = (eq51_e917_d_n17 + eq51_e922_d_n17);
        let eq51_e923_d_n18: f64 = (eq51_e917_d_n18 + eq51_e922_d_n18);
        let eq51_e923_d_n19: f64 = (eq51_e917_d_n19 + eq51_e922_d_n19);
        let eq51_e923_d_n20: f64 = (eq51_e917_d_n20 + eq51_e922_d_n20);
        let eq51_e923_d_n21: f64 = (eq51_e917_d_n21 + eq51_e922_d_n21);
        let eq51_e923_d_n22: f64 = (eq51_e917_d_n22 + eq51_e922_d_n22);
        let eq51_e923_d_b0: f64 = (eq51_e917_d_b0 + eq51_e922_d_b0);
        let eq51_e923_d_b1: f64 = (eq51_e917_d_b1 + eq51_e922_d_b1);
        let eq51_e923_d_b2: f64 = (eq51_e917_d_b2 + eq51_e922_d_b2);
        let eq51_e923_d_b3: f64 = (eq51_e917_d_b3 + eq51_e922_d_b3);
        let eq51_e923_d_b4: f64 = (eq51_e917_d_b4 + eq51_e922_d_b4);
        let eq51_e923_d_b5: f64 = (eq51_e917_d_b5 + eq51_e922_d_b5);
        let eq51_e923_d_b6: f64 = (eq51_e917_d_b6 + eq51_e922_d_b6);
        let eq51_e923_d_b7: f64 = (eq51_e917_d_b7 + eq51_e922_d_b7);
        let eq51_e923_d_b8: f64 = (eq51_e917_d_b8 + eq51_e922_d_b8);
        let eq51_e923_d_b9: f64 = (eq51_e917_d_b9 + eq51_e922_d_b9);
        let eq51_e923_d_b10: f64 = (eq51_e917_d_b10 + eq51_e922_d_b10);
        let eq51_e923_d_b11: f64 = (eq51_e917_d_b11 + eq51_e922_d_b11);
        let eq51_e923_d_b12: f64 = (eq51_e917_d_b12 + eq51_e922_d_b12);
        let eq51_e923_d_b13: f64 = (eq51_e917_d_b13 + eq51_e922_d_b13);
        let eq51_e923_d_b14: f64 = (eq51_e917_d_b14 + eq51_e922_d_b14);
        let eq51_e923_d_b15: f64 = (eq51_e917_d_b15 + eq51_e922_d_b15);
        let eq51_e923_d_b16: f64 = (eq51_e917_d_b16 + eq51_e922_d_b16);
        let eq51_e923_d_b17: f64 = (eq51_e917_d_b17 + eq51_e922_d_b17);
        let eq51_e923_d_b18: f64 = (eq51_e917_d_b18 + eq51_e922_d_b18);
        let eq51_e923_d_b19: f64 = (eq51_e917_d_b19 + eq51_e922_d_b19);
        let eq51_e923_d_b20: f64 = (eq51_e917_d_b20 + eq51_e922_d_b20);
        let eq51_e923_d_b21: f64 = (eq51_e917_d_b21 + eq51_e922_d_b21);
        let eq51_e923_d_b22: f64 = (eq51_e917_d_b22 + eq51_e922_d_b22);
        let eq51_e923_d_b23: f64 = (eq51_e917_d_b23 + eq51_e922_d_b23);
        let eq51_e923_d_b24: f64 = (eq51_e917_d_b24 + eq51_e922_d_b24);
        let eq51_e923_d_b25: f64 = (eq51_e917_d_b25 + eq51_e922_d_b25);
        let eq51_e923_d_b26: f64 = (eq51_e917_d_b26 + eq51_e922_d_b26);
        let eq51_e923_d_b27: f64 = (eq51_e917_d_b27 + eq51_e922_d_b27);
        let eq51_e923_d_b28: f64 = (eq51_e917_d_b28 + eq51_e922_d_b28);
        let eq51_e923_d_b29: f64 = (eq51_e917_d_b29 + eq51_e922_d_b29);
        let eq51_e923_d_b30: f64 = (eq51_e917_d_b30 + eq51_e922_d_b30);
        let eq51_e923_d_b31: f64 = (eq51_e917_d_b31 + eq51_e922_d_b31);
        let eq51_e923_d_b32: f64 = (eq51_e917_d_b32 + eq51_e922_d_b32);
        let eq51_e923_d_b33: f64 = (eq51_e917_d_b33 + eq51_e922_d_b33);
        let eq51_e923_d_b34: f64 = (eq51_e917_d_b34 + eq51_e922_d_b34);
        let eq51_e923_d_b35: f64 = (eq51_e917_d_b35 + eq51_e922_d_b35);
        let eq51_e923_d_b36: f64 = (eq51_e917_d_b36 + eq51_e922_d_b36);
        let eq51_e923_d_b37: f64 = (eq51_e917_d_b37 + eq51_e922_d_b37);
        let eq51_e923_d_b38: f64 = (eq51_e917_d_b38 + eq51_e922_d_b38);
        let eq51_e923_d_b39: f64 = (eq51_e917_d_b39 + eq51_e922_d_b39);
        let eq51_e923_d_b40: f64 = (eq51_e917_d_b40 + eq51_e922_d_b40);
        let eq51_e923_d_b41: f64 = (eq51_e917_d_b41 + eq51_e922_d_b41);
        let eq51_e923_d_b42: f64 = (eq51_e917_d_b42 + eq51_e922_d_b42);
        let eq51_e923_d_b43: f64 = (eq51_e917_d_b43 + eq51_e922_d_b43);
        let eq51_e923_d_b44: f64 = (eq51_e917_d_b44 + eq51_e922_d_b44);
        let eq51_e923_d_b45: f64 = (eq51_e917_d_b45 + eq51_e922_d_b45);
        let eq51_e923_d_b46: f64 = (eq51_e917_d_b46 + eq51_e922_d_b46);
        let eq51_e923_d_b47: f64 = (eq51_e917_d_b47 + eq51_e922_d_b47);
        let eq51_e923_d_b48: f64 = (eq51_e917_d_b48 + eq51_e922_d_b48);
        let eq51_e923_d_b49: f64 = (eq51_e917_d_b49 + eq51_e922_d_b49);
        let eq51_e923_d_b50: f64 = (eq51_e917_d_b50 + eq51_e922_d_b50);
        let eq51_e923_d_b51: f64 = (eq51_e917_d_b51 + eq51_e922_d_b51);
        let eq51_e923_d_b52: f64 = (eq51_e917_d_b52 + eq51_e922_d_b52);
        let eq51_e923_d_b53: f64 = (eq51_e917_d_b53 + eq51_e922_d_b53);
        let eq51_e923_d_b54: f64 = (eq51_e917_d_b54 + eq51_e922_d_b54);
        let eq51_value: f64 = eq51_e923;
        let eq51_node_derivatives: [f64; 23] = [eq51_e923_d_n0, eq51_e923_d_n1, eq51_e923_d_n2, eq51_e923_d_n3, eq51_e923_d_n4, eq51_e923_d_n5, eq51_e923_d_n6, eq51_e923_d_n7, eq51_e923_d_n8, eq51_e923_d_n9, eq51_e923_d_n10, eq51_e923_d_n11, eq51_e923_d_n12, eq51_e923_d_n13, eq51_e923_d_n14, eq51_e923_d_n15, eq51_e923_d_n16, eq51_e923_d_n17, eq51_e923_d_n18, eq51_e923_d_n19, eq51_e923_d_n20, eq51_e923_d_n21, eq51_e923_d_n22];
        let eq51_branch_derivatives: [f64; 55] = [eq51_e923_d_b0, eq51_e923_d_b1, eq51_e923_d_b2, eq51_e923_d_b3, eq51_e923_d_b4, eq51_e923_d_b5, eq51_e923_d_b6, eq51_e923_d_b7, eq51_e923_d_b8, eq51_e923_d_b9, eq51_e923_d_b10, eq51_e923_d_b11, eq51_e923_d_b12, eq51_e923_d_b13, eq51_e923_d_b14, eq51_e923_d_b15, eq51_e923_d_b16, eq51_e923_d_b17, eq51_e923_d_b18, eq51_e923_d_b19, eq51_e923_d_b20, eq51_e923_d_b21, eq51_e923_d_b22, eq51_e923_d_b23, eq51_e923_d_b24, eq51_e923_d_b25, eq51_e923_d_b26, eq51_e923_d_b27, eq51_e923_d_b28, eq51_e923_d_b29, eq51_e923_d_b30, eq51_e923_d_b31, eq51_e923_d_b32, eq51_e923_d_b33, eq51_e923_d_b34, eq51_e923_d_b35, eq51_e923_d_b36, eq51_e923_d_b37, eq51_e923_d_b38, eq51_e923_d_b39, eq51_e923_d_b40, eq51_e923_d_b41, eq51_e923_d_b42, eq51_e923_d_b43, eq51_e923_d_b44, eq51_e923_d_b45, eq51_e923_d_b46, eq51_e923_d_b47, eq51_e923_d_b48, eq51_e923_d_b49, eq51_e923_d_b50, eq51_e923_d_b51, eq51_e923_d_b52, eq51_e923_d_b53, eq51_e923_d_b54];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_8(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq52_e926: f64 = (p.p6 * s.v[41]);
        let eq52_e926_d_n0: f64 = (p.p6 * s.dn[41][0]);
        let eq52_e926_d_n1: f64 = (p.p6 * s.dn[41][1]);
        let eq52_e926_d_n2: f64 = (p.p6 * s.dn[41][2]);
        let eq52_e926_d_n3: f64 = (p.p6 * s.dn[41][3]);
        let eq52_e926_d_n4: f64 = (p.p6 * s.dn[41][4]);
        let eq52_e926_d_n5: f64 = (p.p6 * s.dn[41][5]);
        let eq52_e926_d_n6: f64 = (p.p6 * s.dn[41][6]);
        let eq52_e926_d_n7: f64 = (p.p6 * s.dn[41][7]);
        let eq52_e926_d_n8: f64 = (p.p6 * s.dn[41][8]);
        let eq52_e926_d_n9: f64 = (p.p6 * s.dn[41][9]);
        let eq52_e926_d_n10: f64 = (p.p6 * s.dn[41][10]);
        let eq52_e926_d_n11: f64 = (p.p6 * s.dn[41][11]);
        let eq52_e926_d_n12: f64 = (p.p6 * s.dn[41][12]);
        let eq52_e926_d_n13: f64 = (p.p6 * s.dn[41][13]);
        let eq52_e926_d_n14: f64 = (p.p6 * s.dn[41][14]);
        let eq52_e926_d_n15: f64 = (p.p6 * s.dn[41][15]);
        let eq52_e926_d_n16: f64 = (p.p6 * s.dn[41][16]);
        let eq52_e926_d_n17: f64 = (p.p6 * s.dn[41][17]);
        let eq52_e926_d_n18: f64 = (p.p6 * s.dn[41][18]);
        let eq52_e926_d_n19: f64 = (p.p6 * s.dn[41][19]);
        let eq52_e926_d_n20: f64 = (p.p6 * s.dn[41][20]);
        let eq52_e926_d_n21: f64 = (p.p6 * s.dn[41][21]);
        let eq52_e926_d_n22: f64 = (p.p6 * s.dn[41][22]);
        let eq52_e926_d_b0: f64 = (p.p6 * s.db[41][0]);
        let eq52_e926_d_b1: f64 = (p.p6 * s.db[41][1]);
        let eq52_e926_d_b2: f64 = (p.p6 * s.db[41][2]);
        let eq52_e926_d_b3: f64 = (p.p6 * s.db[41][3]);
        let eq52_e926_d_b4: f64 = (p.p6 * s.db[41][4]);
        let eq52_e926_d_b5: f64 = (p.p6 * s.db[41][5]);
        let eq52_e926_d_b6: f64 = (p.p6 * s.db[41][6]);
        let eq52_e926_d_b7: f64 = (p.p6 * s.db[41][7]);
        let eq52_e926_d_b8: f64 = (p.p6 * s.db[41][8]);
        let eq52_e926_d_b9: f64 = (p.p6 * s.db[41][9]);
        let eq52_e926_d_b10: f64 = (p.p6 * s.db[41][10]);
        let eq52_e926_d_b11: f64 = (p.p6 * s.db[41][11]);
        let eq52_e926_d_b12: f64 = (p.p6 * s.db[41][12]);
        let eq52_e926_d_b13: f64 = (p.p6 * s.db[41][13]);
        let eq52_e926_d_b14: f64 = (p.p6 * s.db[41][14]);
        let eq52_e926_d_b15: f64 = (p.p6 * s.db[41][15]);
        let eq52_e926_d_b16: f64 = (p.p6 * s.db[41][16]);
        let eq52_e926_d_b17: f64 = (p.p6 * s.db[41][17]);
        let eq52_e926_d_b18: f64 = (p.p6 * s.db[41][18]);
        let eq52_e926_d_b19: f64 = (p.p6 * s.db[41][19]);
        let eq52_e926_d_b20: f64 = (p.p6 * s.db[41][20]);
        let eq52_e926_d_b21: f64 = (p.p6 * s.db[41][21]);
        let eq52_e926_d_b22: f64 = (p.p6 * s.db[41][22]);
        let eq52_e926_d_b23: f64 = (p.p6 * s.db[41][23]);
        let eq52_e926_d_b24: f64 = (p.p6 * s.db[41][24]);
        let eq52_e926_d_b25: f64 = (p.p6 * s.db[41][25]);
        let eq52_e926_d_b26: f64 = (p.p6 * s.db[41][26]);
        let eq52_e926_d_b27: f64 = (p.p6 * s.db[41][27]);
        let eq52_e926_d_b28: f64 = (p.p6 * s.db[41][28]);
        let eq52_e926_d_b29: f64 = (p.p6 * s.db[41][29]);
        let eq52_e926_d_b30: f64 = (p.p6 * s.db[41][30]);
        let eq52_e926_d_b31: f64 = (p.p6 * s.db[41][31]);
        let eq52_e926_d_b32: f64 = (p.p6 * s.db[41][32]);
        let eq52_e926_d_b33: f64 = (p.p6 * s.db[41][33]);
        let eq52_e926_d_b34: f64 = (p.p6 * s.db[41][34]);
        let eq52_e926_d_b35: f64 = (p.p6 * s.db[41][35]);
        let eq52_e926_d_b36: f64 = (p.p6 * s.db[41][36]);
        let eq52_e926_d_b37: f64 = (p.p6 * s.db[41][37]);
        let eq52_e926_d_b38: f64 = (p.p6 * s.db[41][38]);
        let eq52_e926_d_b39: f64 = (p.p6 * s.db[41][39]);
        let eq52_e926_d_b40: f64 = (p.p6 * s.db[41][40]);
        let eq52_e926_d_b41: f64 = (p.p6 * s.db[41][41]);
        let eq52_e926_d_b42: f64 = (p.p6 * s.db[41][42]);
        let eq52_e926_d_b43: f64 = (p.p6 * s.db[41][43]);
        let eq52_e926_d_b44: f64 = (p.p6 * s.db[41][44]);
        let eq52_e926_d_b45: f64 = (p.p6 * s.db[41][45]);
        let eq52_e926_d_b46: f64 = (p.p6 * s.db[41][46]);
        let eq52_e926_d_b47: f64 = (p.p6 * s.db[41][47]);
        let eq52_e926_d_b48: f64 = (p.p6 * s.db[41][48]);
        let eq52_e926_d_b49: f64 = (p.p6 * s.db[41][49]);
        let eq52_e926_d_b50: f64 = (p.p6 * s.db[41][50]);
        let eq52_e926_d_b51: f64 = (p.p6 * s.db[41][51]);
        let eq52_e926_d_b52: f64 = (p.p6 * s.db[41][52]);
        let eq52_e926_d_b53: f64 = (p.p6 * s.db[41][53]);
        let eq52_e926_d_b54: f64 = (p.p6 * s.db[41][54]);
        let eq52_e929: f64 = (p.p4 * p.p5);
        let eq52_e931: f64 = (eq52_e929 * s.v[332]);
        let eq52_e931_d_n0: f64 = (eq52_e929 * s.dn[332][0]);
        let eq52_e931_d_n1: f64 = (eq52_e929 * s.dn[332][1]);
        let eq52_e931_d_n2: f64 = (eq52_e929 * s.dn[332][2]);
        let eq52_e931_d_n3: f64 = (eq52_e929 * s.dn[332][3]);
        let eq52_e931_d_n4: f64 = (eq52_e929 * s.dn[332][4]);
        let eq52_e931_d_n5: f64 = (eq52_e929 * s.dn[332][5]);
        let eq52_e931_d_n6: f64 = (eq52_e929 * s.dn[332][6]);
        let eq52_e931_d_n7: f64 = (eq52_e929 * s.dn[332][7]);
        let eq52_e931_d_n8: f64 = (eq52_e929 * s.dn[332][8]);
        let eq52_e931_d_n9: f64 = (eq52_e929 * s.dn[332][9]);
        let eq52_e931_d_n10: f64 = (eq52_e929 * s.dn[332][10]);
        let eq52_e931_d_n11: f64 = (eq52_e929 * s.dn[332][11]);
        let eq52_e931_d_n12: f64 = (eq52_e929 * s.dn[332][12]);
        let eq52_e931_d_n13: f64 = (eq52_e929 * s.dn[332][13]);
        let eq52_e931_d_n14: f64 = (eq52_e929 * s.dn[332][14]);
        let eq52_e931_d_n15: f64 = (eq52_e929 * s.dn[332][15]);
        let eq52_e931_d_n16: f64 = (eq52_e929 * s.dn[332][16]);
        let eq52_e931_d_n17: f64 = (eq52_e929 * s.dn[332][17]);
        let eq52_e931_d_n18: f64 = (eq52_e929 * s.dn[332][18]);
        let eq52_e931_d_n19: f64 = (eq52_e929 * s.dn[332][19]);
        let eq52_e931_d_n20: f64 = (eq52_e929 * s.dn[332][20]);
        let eq52_e931_d_n21: f64 = (eq52_e929 * s.dn[332][21]);
        let eq52_e931_d_n22: f64 = (eq52_e929 * s.dn[332][22]);
        let eq52_e931_d_b0: f64 = (eq52_e929 * s.db[332][0]);
        let eq52_e931_d_b1: f64 = (eq52_e929 * s.db[332][1]);
        let eq52_e931_d_b2: f64 = (eq52_e929 * s.db[332][2]);
        let eq52_e931_d_b3: f64 = (eq52_e929 * s.db[332][3]);
        let eq52_e931_d_b4: f64 = (eq52_e929 * s.db[332][4]);
        let eq52_e931_d_b5: f64 = (eq52_e929 * s.db[332][5]);
        let eq52_e931_d_b6: f64 = (eq52_e929 * s.db[332][6]);
        let eq52_e931_d_b7: f64 = (eq52_e929 * s.db[332][7]);
        let eq52_e931_d_b8: f64 = (eq52_e929 * s.db[332][8]);
        let eq52_e931_d_b9: f64 = (eq52_e929 * s.db[332][9]);
        let eq52_e931_d_b10: f64 = (eq52_e929 * s.db[332][10]);
        let eq52_e931_d_b11: f64 = (eq52_e929 * s.db[332][11]);
        let eq52_e931_d_b12: f64 = (eq52_e929 * s.db[332][12]);
        let eq52_e931_d_b13: f64 = (eq52_e929 * s.db[332][13]);
        let eq52_e931_d_b14: f64 = (eq52_e929 * s.db[332][14]);
        let eq52_e931_d_b15: f64 = (eq52_e929 * s.db[332][15]);
        let eq52_e931_d_b16: f64 = (eq52_e929 * s.db[332][16]);
        let eq52_e931_d_b17: f64 = (eq52_e929 * s.db[332][17]);
        let eq52_e931_d_b18: f64 = (eq52_e929 * s.db[332][18]);
        let eq52_e931_d_b19: f64 = (eq52_e929 * s.db[332][19]);
        let eq52_e931_d_b20: f64 = (eq52_e929 * s.db[332][20]);
        let eq52_e931_d_b21: f64 = (eq52_e929 * s.db[332][21]);
        let eq52_e931_d_b22: f64 = (eq52_e929 * s.db[332][22]);
        let eq52_e931_d_b23: f64 = (eq52_e929 * s.db[332][23]);
        let eq52_e931_d_b24: f64 = (eq52_e929 * s.db[332][24]);
        let eq52_e931_d_b25: f64 = (eq52_e929 * s.db[332][25]);
        let eq52_e931_d_b26: f64 = (eq52_e929 * s.db[332][26]);
        let eq52_e931_d_b27: f64 = (eq52_e929 * s.db[332][27]);
        let eq52_e931_d_b28: f64 = (eq52_e929 * s.db[332][28]);
        let eq52_e931_d_b29: f64 = (eq52_e929 * s.db[332][29]);
        let eq52_e931_d_b30: f64 = (eq52_e929 * s.db[332][30]);
        let eq52_e931_d_b31: f64 = (eq52_e929 * s.db[332][31]);
        let eq52_e931_d_b32: f64 = (eq52_e929 * s.db[332][32]);
        let eq52_e931_d_b33: f64 = (eq52_e929 * s.db[332][33]);
        let eq52_e931_d_b34: f64 = (eq52_e929 * s.db[332][34]);
        let eq52_e931_d_b35: f64 = (eq52_e929 * s.db[332][35]);
        let eq52_e931_d_b36: f64 = (eq52_e929 * s.db[332][36]);
        let eq52_e931_d_b37: f64 = (eq52_e929 * s.db[332][37]);
        let eq52_e931_d_b38: f64 = (eq52_e929 * s.db[332][38]);
        let eq52_e931_d_b39: f64 = (eq52_e929 * s.db[332][39]);
        let eq52_e931_d_b40: f64 = (eq52_e929 * s.db[332][40]);
        let eq52_e931_d_b41: f64 = (eq52_e929 * s.db[332][41]);
        let eq52_e931_d_b42: f64 = (eq52_e929 * s.db[332][42]);
        let eq52_e931_d_b43: f64 = (eq52_e929 * s.db[332][43]);
        let eq52_e931_d_b44: f64 = (eq52_e929 * s.db[332][44]);
        let eq52_e931_d_b45: f64 = (eq52_e929 * s.db[332][45]);
        let eq52_e931_d_b46: f64 = (eq52_e929 * s.db[332][46]);
        let eq52_e931_d_b47: f64 = (eq52_e929 * s.db[332][47]);
        let eq52_e931_d_b48: f64 = (eq52_e929 * s.db[332][48]);
        let eq52_e931_d_b49: f64 = (eq52_e929 * s.db[332][49]);
        let eq52_e931_d_b50: f64 = (eq52_e929 * s.db[332][50]);
        let eq52_e931_d_b51: f64 = (eq52_e929 * s.db[332][51]);
        let eq52_e931_d_b52: f64 = (eq52_e929 * s.db[332][52]);
        let eq52_e931_d_b53: f64 = (eq52_e929 * s.db[332][53]);
        let eq52_e931_d_b54: f64 = (eq52_e929 * s.db[332][54]);
        let eq52_e932: f64 = (eq52_e926 * eq52_e931);
        let eq52_e932_d_n0: f64 = ((eq52_e926_d_n0 * eq52_e931) + (eq52_e926 * eq52_e931_d_n0));
        let eq52_e932_d_n1: f64 = ((eq52_e926_d_n1 * eq52_e931) + (eq52_e926 * eq52_e931_d_n1));
        let eq52_e932_d_n2: f64 = ((eq52_e926_d_n2 * eq52_e931) + (eq52_e926 * eq52_e931_d_n2));
        let eq52_e932_d_n3: f64 = ((eq52_e926_d_n3 * eq52_e931) + (eq52_e926 * eq52_e931_d_n3));
        let eq52_e932_d_n4: f64 = ((eq52_e926_d_n4 * eq52_e931) + (eq52_e926 * eq52_e931_d_n4));
        let eq52_e932_d_n5: f64 = ((eq52_e926_d_n5 * eq52_e931) + (eq52_e926 * eq52_e931_d_n5));
        let eq52_e932_d_n6: f64 = ((eq52_e926_d_n6 * eq52_e931) + (eq52_e926 * eq52_e931_d_n6));
        let eq52_e932_d_n7: f64 = ((eq52_e926_d_n7 * eq52_e931) + (eq52_e926 * eq52_e931_d_n7));
        let eq52_e932_d_n8: f64 = ((eq52_e926_d_n8 * eq52_e931) + (eq52_e926 * eq52_e931_d_n8));
        let eq52_e932_d_n9: f64 = ((eq52_e926_d_n9 * eq52_e931) + (eq52_e926 * eq52_e931_d_n9));
        let eq52_e932_d_n10: f64 = ((eq52_e926_d_n10 * eq52_e931) + (eq52_e926 * eq52_e931_d_n10));
        let eq52_e932_d_n11: f64 = ((eq52_e926_d_n11 * eq52_e931) + (eq52_e926 * eq52_e931_d_n11));
        let eq52_e932_d_n12: f64 = ((eq52_e926_d_n12 * eq52_e931) + (eq52_e926 * eq52_e931_d_n12));
        let eq52_e932_d_n13: f64 = ((eq52_e926_d_n13 * eq52_e931) + (eq52_e926 * eq52_e931_d_n13));
        let eq52_e932_d_n14: f64 = ((eq52_e926_d_n14 * eq52_e931) + (eq52_e926 * eq52_e931_d_n14));
        let eq52_e932_d_n15: f64 = ((eq52_e926_d_n15 * eq52_e931) + (eq52_e926 * eq52_e931_d_n15));
        let eq52_e932_d_n16: f64 = ((eq52_e926_d_n16 * eq52_e931) + (eq52_e926 * eq52_e931_d_n16));
        let eq52_e932_d_n17: f64 = ((eq52_e926_d_n17 * eq52_e931) + (eq52_e926 * eq52_e931_d_n17));
        let eq52_e932_d_n18: f64 = ((eq52_e926_d_n18 * eq52_e931) + (eq52_e926 * eq52_e931_d_n18));
        let eq52_e932_d_n19: f64 = ((eq52_e926_d_n19 * eq52_e931) + (eq52_e926 * eq52_e931_d_n19));
        let eq52_e932_d_n20: f64 = ((eq52_e926_d_n20 * eq52_e931) + (eq52_e926 * eq52_e931_d_n20));
        let eq52_e932_d_n21: f64 = ((eq52_e926_d_n21 * eq52_e931) + (eq52_e926 * eq52_e931_d_n21));
        let eq52_e932_d_n22: f64 = ((eq52_e926_d_n22 * eq52_e931) + (eq52_e926 * eq52_e931_d_n22));
        let eq52_e932_d_b0: f64 = ((eq52_e926_d_b0 * eq52_e931) + (eq52_e926 * eq52_e931_d_b0));
        let eq52_e932_d_b1: f64 = ((eq52_e926_d_b1 * eq52_e931) + (eq52_e926 * eq52_e931_d_b1));
        let eq52_e932_d_b2: f64 = ((eq52_e926_d_b2 * eq52_e931) + (eq52_e926 * eq52_e931_d_b2));
        let eq52_e932_d_b3: f64 = ((eq52_e926_d_b3 * eq52_e931) + (eq52_e926 * eq52_e931_d_b3));
        let eq52_e932_d_b4: f64 = ((eq52_e926_d_b4 * eq52_e931) + (eq52_e926 * eq52_e931_d_b4));
        let eq52_e932_d_b5: f64 = ((eq52_e926_d_b5 * eq52_e931) + (eq52_e926 * eq52_e931_d_b5));
        let eq52_e932_d_b6: f64 = ((eq52_e926_d_b6 * eq52_e931) + (eq52_e926 * eq52_e931_d_b6));
        let eq52_e932_d_b7: f64 = ((eq52_e926_d_b7 * eq52_e931) + (eq52_e926 * eq52_e931_d_b7));
        let eq52_e932_d_b8: f64 = ((eq52_e926_d_b8 * eq52_e931) + (eq52_e926 * eq52_e931_d_b8));
        let eq52_e932_d_b9: f64 = ((eq52_e926_d_b9 * eq52_e931) + (eq52_e926 * eq52_e931_d_b9));
        let eq52_e932_d_b10: f64 = ((eq52_e926_d_b10 * eq52_e931) + (eq52_e926 * eq52_e931_d_b10));
        let eq52_e932_d_b11: f64 = ((eq52_e926_d_b11 * eq52_e931) + (eq52_e926 * eq52_e931_d_b11));
        let eq52_e932_d_b12: f64 = ((eq52_e926_d_b12 * eq52_e931) + (eq52_e926 * eq52_e931_d_b12));
        let eq52_e932_d_b13: f64 = ((eq52_e926_d_b13 * eq52_e931) + (eq52_e926 * eq52_e931_d_b13));
        let eq52_e932_d_b14: f64 = ((eq52_e926_d_b14 * eq52_e931) + (eq52_e926 * eq52_e931_d_b14));
        let eq52_e932_d_b15: f64 = ((eq52_e926_d_b15 * eq52_e931) + (eq52_e926 * eq52_e931_d_b15));
        let eq52_e932_d_b16: f64 = ((eq52_e926_d_b16 * eq52_e931) + (eq52_e926 * eq52_e931_d_b16));
        let eq52_e932_d_b17: f64 = ((eq52_e926_d_b17 * eq52_e931) + (eq52_e926 * eq52_e931_d_b17));
        let eq52_e932_d_b18: f64 = ((eq52_e926_d_b18 * eq52_e931) + (eq52_e926 * eq52_e931_d_b18));
        let eq52_e932_d_b19: f64 = ((eq52_e926_d_b19 * eq52_e931) + (eq52_e926 * eq52_e931_d_b19));
        let eq52_e932_d_b20: f64 = ((eq52_e926_d_b20 * eq52_e931) + (eq52_e926 * eq52_e931_d_b20));
        let eq52_e932_d_b21: f64 = ((eq52_e926_d_b21 * eq52_e931) + (eq52_e926 * eq52_e931_d_b21));
        let eq52_e932_d_b22: f64 = ((eq52_e926_d_b22 * eq52_e931) + (eq52_e926 * eq52_e931_d_b22));
        let eq52_e932_d_b23: f64 = ((eq52_e926_d_b23 * eq52_e931) + (eq52_e926 * eq52_e931_d_b23));
        let eq52_e932_d_b24: f64 = ((eq52_e926_d_b24 * eq52_e931) + (eq52_e926 * eq52_e931_d_b24));
        let eq52_e932_d_b25: f64 = ((eq52_e926_d_b25 * eq52_e931) + (eq52_e926 * eq52_e931_d_b25));
        let eq52_e932_d_b26: f64 = ((eq52_e926_d_b26 * eq52_e931) + (eq52_e926 * eq52_e931_d_b26));
        let eq52_e932_d_b27: f64 = ((eq52_e926_d_b27 * eq52_e931) + (eq52_e926 * eq52_e931_d_b27));
        let eq52_e932_d_b28: f64 = ((eq52_e926_d_b28 * eq52_e931) + (eq52_e926 * eq52_e931_d_b28));
        let eq52_e932_d_b29: f64 = ((eq52_e926_d_b29 * eq52_e931) + (eq52_e926 * eq52_e931_d_b29));
        let eq52_e932_d_b30: f64 = ((eq52_e926_d_b30 * eq52_e931) + (eq52_e926 * eq52_e931_d_b30));
        let eq52_e932_d_b31: f64 = ((eq52_e926_d_b31 * eq52_e931) + (eq52_e926 * eq52_e931_d_b31));
        let eq52_e932_d_b32: f64 = ((eq52_e926_d_b32 * eq52_e931) + (eq52_e926 * eq52_e931_d_b32));
        let eq52_e932_d_b33: f64 = ((eq52_e926_d_b33 * eq52_e931) + (eq52_e926 * eq52_e931_d_b33));
        let eq52_e932_d_b34: f64 = ((eq52_e926_d_b34 * eq52_e931) + (eq52_e926 * eq52_e931_d_b34));
        let eq52_e932_d_b35: f64 = ((eq52_e926_d_b35 * eq52_e931) + (eq52_e926 * eq52_e931_d_b35));
        let eq52_e932_d_b36: f64 = ((eq52_e926_d_b36 * eq52_e931) + (eq52_e926 * eq52_e931_d_b36));
        let eq52_e932_d_b37: f64 = ((eq52_e926_d_b37 * eq52_e931) + (eq52_e926 * eq52_e931_d_b37));
        let eq52_e932_d_b38: f64 = ((eq52_e926_d_b38 * eq52_e931) + (eq52_e926 * eq52_e931_d_b38));
        let eq52_e932_d_b39: f64 = ((eq52_e926_d_b39 * eq52_e931) + (eq52_e926 * eq52_e931_d_b39));
        let eq52_e932_d_b40: f64 = ((eq52_e926_d_b40 * eq52_e931) + (eq52_e926 * eq52_e931_d_b40));
        let eq52_e932_d_b41: f64 = ((eq52_e926_d_b41 * eq52_e931) + (eq52_e926 * eq52_e931_d_b41));
        let eq52_e932_d_b42: f64 = ((eq52_e926_d_b42 * eq52_e931) + (eq52_e926 * eq52_e931_d_b42));
        let eq52_e932_d_b43: f64 = ((eq52_e926_d_b43 * eq52_e931) + (eq52_e926 * eq52_e931_d_b43));
        let eq52_e932_d_b44: f64 = ((eq52_e926_d_b44 * eq52_e931) + (eq52_e926 * eq52_e931_d_b44));
        let eq52_e932_d_b45: f64 = ((eq52_e926_d_b45 * eq52_e931) + (eq52_e926 * eq52_e931_d_b45));
        let eq52_e932_d_b46: f64 = ((eq52_e926_d_b46 * eq52_e931) + (eq52_e926 * eq52_e931_d_b46));
        let eq52_e932_d_b47: f64 = ((eq52_e926_d_b47 * eq52_e931) + (eq52_e926 * eq52_e931_d_b47));
        let eq52_e932_d_b48: f64 = ((eq52_e926_d_b48 * eq52_e931) + (eq52_e926 * eq52_e931_d_b48));
        let eq52_e932_d_b49: f64 = ((eq52_e926_d_b49 * eq52_e931) + (eq52_e926 * eq52_e931_d_b49));
        let eq52_e932_d_b50: f64 = ((eq52_e926_d_b50 * eq52_e931) + (eq52_e926 * eq52_e931_d_b50));
        let eq52_e932_d_b51: f64 = ((eq52_e926_d_b51 * eq52_e931) + (eq52_e926 * eq52_e931_d_b51));
        let eq52_e932_d_b52: f64 = ((eq52_e926_d_b52 * eq52_e931) + (eq52_e926 * eq52_e931_d_b52));
        let eq52_e932_d_b53: f64 = ((eq52_e926_d_b53 * eq52_e931) + (eq52_e926 * eq52_e931_d_b53));
        let eq52_e932_d_b54: f64 = ((eq52_e926_d_b54 * eq52_e931) + (eq52_e926 * eq52_e931_d_b54));
        let eq52_value: f64 = eq52_e932;
        let eq52_node_derivatives: [f64; 23] = [eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22];
        let eq52_branch_derivatives: [f64; 55] = [eq52_e932_d_b0, eq52_e932_d_b1, eq52_e932_d_b2, eq52_e932_d_b3, eq52_e932_d_b4, eq52_e932_d_b5, eq52_e932_d_b6, eq52_e932_d_b7, eq52_e932_d_b8, eq52_e932_d_b9, eq52_e932_d_b10, eq52_e932_d_b11, eq52_e932_d_b12, eq52_e932_d_b13, eq52_e932_d_b14, eq52_e932_d_b15, eq52_e932_d_b16, eq52_e932_d_b17, eq52_e932_d_b18, eq52_e932_d_b19, eq52_e932_d_b20, eq52_e932_d_b21, eq52_e932_d_b22, eq52_e932_d_b23, eq52_e932_d_b24, eq52_e932_d_b25, eq52_e932_d_b26, eq52_e932_d_b27, eq52_e932_d_b28, eq52_e932_d_b29, eq52_e932_d_b30, eq52_e932_d_b31, eq52_e932_d_b32, eq52_e932_d_b33, eq52_e932_d_b34, eq52_e932_d_b35, eq52_e932_d_b36, eq52_e932_d_b37, eq52_e932_d_b38, eq52_e932_d_b39, eq52_e932_d_b40, eq52_e932_d_b41, eq52_e932_d_b42, eq52_e932_d_b43, eq52_e932_d_b44, eq52_e932_d_b45, eq52_e932_d_b46, eq52_e932_d_b47, eq52_e932_d_b48, eq52_e932_d_b49, eq52_e932_d_b50, eq52_e932_d_b51, eq52_e932_d_b52, eq52_e932_d_b53, eq52_e932_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq52_value),
            &eq52_node_derivatives,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e938, eq53_e938_d_n0, eq53_e938_d_n1, eq53_e938_d_n2, eq53_e938_d_n3, eq53_e938_d_n4, eq53_e938_d_n5, eq53_e938_d_n6, eq53_e938_d_n7, eq53_e938_d_n8, eq53_e938_d_n9, eq53_e938_d_n10, eq53_e938_d_n11, eq53_e938_d_n12, eq53_e938_d_n13, eq53_e938_d_n14, eq53_e938_d_n15, eq53_e938_d_n16, eq53_e938_d_n17, eq53_e938_d_n18, eq53_e938_d_n19, eq53_e938_d_n20, eq53_e938_d_n21, eq53_e938_d_n22, eq53_e938_d_b0, eq53_e938_d_b1, eq53_e938_d_b2, eq53_e938_d_b3, eq53_e938_d_b4, eq53_e938_d_b5, eq53_e938_d_b6, eq53_e938_d_b7, eq53_e938_d_b8, eq53_e938_d_b9, eq53_e938_d_b10, eq53_e938_d_b11, eq53_e938_d_b12, eq53_e938_d_b13, eq53_e938_d_b14, eq53_e938_d_b15, eq53_e938_d_b16, eq53_e938_d_b17, eq53_e938_d_b18, eq53_e938_d_b19, eq53_e938_d_b20, eq53_e938_d_b21, eq53_e938_d_b22, eq53_e938_d_b23, eq53_e938_d_b24, eq53_e938_d_b25, eq53_e938_d_b26, eq53_e938_d_b27, eq53_e938_d_b28, eq53_e938_d_b29, eq53_e938_d_b30, eq53_e938_d_b31, eq53_e938_d_b32, eq53_e938_d_b33, eq53_e938_d_b34, eq53_e938_d_b35, eq53_e938_d_b36, eq53_e938_d_b37, eq53_e938_d_b38, eq53_e938_d_b39, eq53_e938_d_b40, eq53_e938_d_b41, eq53_e938_d_b42, eq53_e938_d_b43, eq53_e938_d_b44, eq53_e938_d_b45, eq53_e938_d_b46, eq53_e938_d_b47, eq53_e938_d_b48, eq53_e938_d_b49, eq53_e938_d_b50, eq53_e938_d_b51, eq53_e938_d_b52, eq53_e938_d_b53, eq53_e938_d_b54,) = {
    if s.b[423] {
        let eq53_e936: f64 = (p.p6 * s.v[206]);
        let eq53_e936_d_n0: f64 = (p.p6 * s.dn[206][0]);
        let eq53_e936_d_n1: f64 = (p.p6 * s.dn[206][1]);
        let eq53_e936_d_n2: f64 = (p.p6 * s.dn[206][2]);
        let eq53_e936_d_n3: f64 = (p.p6 * s.dn[206][3]);
        let eq53_e936_d_n4: f64 = (p.p6 * s.dn[206][4]);
        let eq53_e936_d_n5: f64 = (p.p6 * s.dn[206][5]);
        let eq53_e936_d_n6: f64 = (p.p6 * s.dn[206][6]);
        let eq53_e936_d_n7: f64 = (p.p6 * s.dn[206][7]);
        let eq53_e936_d_n8: f64 = (p.p6 * s.dn[206][8]);
        let eq53_e936_d_n9: f64 = (p.p6 * s.dn[206][9]);
        let eq53_e936_d_n10: f64 = (p.p6 * s.dn[206][10]);
        let eq53_e936_d_n11: f64 = (p.p6 * s.dn[206][11]);
        let eq53_e936_d_n12: f64 = (p.p6 * s.dn[206][12]);
        let eq53_e936_d_n13: f64 = (p.p6 * s.dn[206][13]);
        let eq53_e936_d_n14: f64 = (p.p6 * s.dn[206][14]);
        let eq53_e936_d_n15: f64 = (p.p6 * s.dn[206][15]);
        let eq53_e936_d_n16: f64 = (p.p6 * s.dn[206][16]);
        let eq53_e936_d_n17: f64 = (p.p6 * s.dn[206][17]);
        let eq53_e936_d_n18: f64 = (p.p6 * s.dn[206][18]);
        let eq53_e936_d_n19: f64 = (p.p6 * s.dn[206][19]);
        let eq53_e936_d_n20: f64 = (p.p6 * s.dn[206][20]);
        let eq53_e936_d_n21: f64 = (p.p6 * s.dn[206][21]);
        let eq53_e936_d_n22: f64 = (p.p6 * s.dn[206][22]);
        let eq53_e936_d_b0: f64 = (p.p6 * s.db[206][0]);
        let eq53_e936_d_b1: f64 = (p.p6 * s.db[206][1]);
        let eq53_e936_d_b2: f64 = (p.p6 * s.db[206][2]);
        let eq53_e936_d_b3: f64 = (p.p6 * s.db[206][3]);
        let eq53_e936_d_b4: f64 = (p.p6 * s.db[206][4]);
        let eq53_e936_d_b5: f64 = (p.p6 * s.db[206][5]);
        let eq53_e936_d_b6: f64 = (p.p6 * s.db[206][6]);
        let eq53_e936_d_b7: f64 = (p.p6 * s.db[206][7]);
        let eq53_e936_d_b8: f64 = (p.p6 * s.db[206][8]);
        let eq53_e936_d_b9: f64 = (p.p6 * s.db[206][9]);
        let eq53_e936_d_b10: f64 = (p.p6 * s.db[206][10]);
        let eq53_e936_d_b11: f64 = (p.p6 * s.db[206][11]);
        let eq53_e936_d_b12: f64 = (p.p6 * s.db[206][12]);
        let eq53_e936_d_b13: f64 = (p.p6 * s.db[206][13]);
        let eq53_e936_d_b14: f64 = (p.p6 * s.db[206][14]);
        let eq53_e936_d_b15: f64 = (p.p6 * s.db[206][15]);
        let eq53_e936_d_b16: f64 = (p.p6 * s.db[206][16]);
        let eq53_e936_d_b17: f64 = (p.p6 * s.db[206][17]);
        let eq53_e936_d_b18: f64 = (p.p6 * s.db[206][18]);
        let eq53_e936_d_b19: f64 = (p.p6 * s.db[206][19]);
        let eq53_e936_d_b20: f64 = (p.p6 * s.db[206][20]);
        let eq53_e936_d_b21: f64 = (p.p6 * s.db[206][21]);
        let eq53_e936_d_b22: f64 = (p.p6 * s.db[206][22]);
        let eq53_e936_d_b23: f64 = (p.p6 * s.db[206][23]);
        let eq53_e936_d_b24: f64 = (p.p6 * s.db[206][24]);
        let eq53_e936_d_b25: f64 = (p.p6 * s.db[206][25]);
        let eq53_e936_d_b26: f64 = (p.p6 * s.db[206][26]);
        let eq53_e936_d_b27: f64 = (p.p6 * s.db[206][27]);
        let eq53_e936_d_b28: f64 = (p.p6 * s.db[206][28]);
        let eq53_e936_d_b29: f64 = (p.p6 * s.db[206][29]);
        let eq53_e936_d_b30: f64 = (p.p6 * s.db[206][30]);
        let eq53_e936_d_b31: f64 = (p.p6 * s.db[206][31]);
        let eq53_e936_d_b32: f64 = (p.p6 * s.db[206][32]);
        let eq53_e936_d_b33: f64 = (p.p6 * s.db[206][33]);
        let eq53_e936_d_b34: f64 = (p.p6 * s.db[206][34]);
        let eq53_e936_d_b35: f64 = (p.p6 * s.db[206][35]);
        let eq53_e936_d_b36: f64 = (p.p6 * s.db[206][36]);
        let eq53_e936_d_b37: f64 = (p.p6 * s.db[206][37]);
        let eq53_e936_d_b38: f64 = (p.p6 * s.db[206][38]);
        let eq53_e936_d_b39: f64 = (p.p6 * s.db[206][39]);
        let eq53_e936_d_b40: f64 = (p.p6 * s.db[206][40]);
        let eq53_e936_d_b41: f64 = (p.p6 * s.db[206][41]);
        let eq53_e936_d_b42: f64 = (p.p6 * s.db[206][42]);
        let eq53_e936_d_b43: f64 = (p.p6 * s.db[206][43]);
        let eq53_e936_d_b44: f64 = (p.p6 * s.db[206][44]);
        let eq53_e936_d_b45: f64 = (p.p6 * s.db[206][45]);
        let eq53_e936_d_b46: f64 = (p.p6 * s.db[206][46]);
        let eq53_e936_d_b47: f64 = (p.p6 * s.db[206][47]);
        let eq53_e936_d_b48: f64 = (p.p6 * s.db[206][48]);
        let eq53_e936_d_b49: f64 = (p.p6 * s.db[206][49]);
        let eq53_e936_d_b50: f64 = (p.p6 * s.db[206][50]);
        let eq53_e936_d_b51: f64 = (p.p6 * s.db[206][51]);
        let eq53_e936_d_b52: f64 = (p.p6 * s.db[206][52]);
        let eq53_e936_d_b53: f64 = (p.p6 * s.db[206][53]);
        let eq53_e936_d_b54: f64 = (p.p6 * s.db[206][54]);
        (eq53_e936, eq53_e936_d_n0, eq53_e936_d_n1, eq53_e936_d_n2, eq53_e936_d_n3, eq53_e936_d_n4, eq53_e936_d_n5, eq53_e936_d_n6, eq53_e936_d_n7, eq53_e936_d_n8, eq53_e936_d_n9, eq53_e936_d_n10, eq53_e936_d_n11, eq53_e936_d_n12, eq53_e936_d_n13, eq53_e936_d_n14, eq53_e936_d_n15, eq53_e936_d_n16, eq53_e936_d_n17, eq53_e936_d_n18, eq53_e936_d_n19, eq53_e936_d_n20, eq53_e936_d_n21, eq53_e936_d_n22, eq53_e936_d_b0, eq53_e936_d_b1, eq53_e936_d_b2, eq53_e936_d_b3, eq53_e936_d_b4, eq53_e936_d_b5, eq53_e936_d_b6, eq53_e936_d_b7, eq53_e936_d_b8, eq53_e936_d_b9, eq53_e936_d_b10, eq53_e936_d_b11, eq53_e936_d_b12, eq53_e936_d_b13, eq53_e936_d_b14, eq53_e936_d_b15, eq53_e936_d_b16, eq53_e936_d_b17, eq53_e936_d_b18, eq53_e936_d_b19, eq53_e936_d_b20, eq53_e936_d_b21, eq53_e936_d_b22, eq53_e936_d_b23, eq53_e936_d_b24, eq53_e936_d_b25, eq53_e936_d_b26, eq53_e936_d_b27, eq53_e936_d_b28, eq53_e936_d_b29, eq53_e936_d_b30, eq53_e936_d_b31, eq53_e936_d_b32, eq53_e936_d_b33, eq53_e936_d_b34, eq53_e936_d_b35, eq53_e936_d_b36, eq53_e936_d_b37, eq53_e936_d_b38, eq53_e936_d_b39, eq53_e936_d_b40, eq53_e936_d_b41, eq53_e936_d_b42, eq53_e936_d_b43, eq53_e936_d_b44, eq53_e936_d_b45, eq53_e936_d_b46, eq53_e936_d_b47, eq53_e936_d_b48, eq53_e936_d_b49, eq53_e936_d_b50, eq53_e936_d_b51, eq53_e936_d_b52, eq53_e936_d_b53, eq53_e936_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e938;
        let eq53_node_derivatives: [f64; 23] = [eq53_e938_d_n0, eq53_e938_d_n1, eq53_e938_d_n2, eq53_e938_d_n3, eq53_e938_d_n4, eq53_e938_d_n5, eq53_e938_d_n6, eq53_e938_d_n7, eq53_e938_d_n8, eq53_e938_d_n9, eq53_e938_d_n10, eq53_e938_d_n11, eq53_e938_d_n12, eq53_e938_d_n13, eq53_e938_d_n14, eq53_e938_d_n15, eq53_e938_d_n16, eq53_e938_d_n17, eq53_e938_d_n18, eq53_e938_d_n19, eq53_e938_d_n20, eq53_e938_d_n21, eq53_e938_d_n22];
        let eq53_branch_derivatives: [f64; 55] = [eq53_e938_d_b0, eq53_e938_d_b1, eq53_e938_d_b2, eq53_e938_d_b3, eq53_e938_d_b4, eq53_e938_d_b5, eq53_e938_d_b6, eq53_e938_d_b7, eq53_e938_d_b8, eq53_e938_d_b9, eq53_e938_d_b10, eq53_e938_d_b11, eq53_e938_d_b12, eq53_e938_d_b13, eq53_e938_d_b14, eq53_e938_d_b15, eq53_e938_d_b16, eq53_e938_d_b17, eq53_e938_d_b18, eq53_e938_d_b19, eq53_e938_d_b20, eq53_e938_d_b21, eq53_e938_d_b22, eq53_e938_d_b23, eq53_e938_d_b24, eq53_e938_d_b25, eq53_e938_d_b26, eq53_e938_d_b27, eq53_e938_d_b28, eq53_e938_d_b29, eq53_e938_d_b30, eq53_e938_d_b31, eq53_e938_d_b32, eq53_e938_d_b33, eq53_e938_d_b34, eq53_e938_d_b35, eq53_e938_d_b36, eq53_e938_d_b37, eq53_e938_d_b38, eq53_e938_d_b39, eq53_e938_d_b40, eq53_e938_d_b41, eq53_e938_d_b42, eq53_e938_d_b43, eq53_e938_d_b44, eq53_e938_d_b45, eq53_e938_d_b46, eq53_e938_d_b47, eq53_e938_d_b48, eq53_e938_d_b49, eq53_e938_d_b50, eq53_e938_d_b51, eq53_e938_d_b52, eq53_e938_d_b53, eq53_e938_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq53_value),
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e944, eq54_e944_d_n0, eq54_e944_d_n1, eq54_e944_d_n2, eq54_e944_d_n3, eq54_e944_d_n4, eq54_e944_d_n5, eq54_e944_d_n6, eq54_e944_d_n7, eq54_e944_d_n8, eq54_e944_d_n9, eq54_e944_d_n10, eq54_e944_d_n11, eq54_e944_d_n12, eq54_e944_d_n13, eq54_e944_d_n14, eq54_e944_d_n15, eq54_e944_d_n16, eq54_e944_d_n17, eq54_e944_d_n18, eq54_e944_d_n19, eq54_e944_d_n20, eq54_e944_d_n21, eq54_e944_d_n22, eq54_e944_d_b0, eq54_e944_d_b1, eq54_e944_d_b2, eq54_e944_d_b3, eq54_e944_d_b4, eq54_e944_d_b5, eq54_e944_d_b6, eq54_e944_d_b7, eq54_e944_d_b8, eq54_e944_d_b9, eq54_e944_d_b10, eq54_e944_d_b11, eq54_e944_d_b12, eq54_e944_d_b13, eq54_e944_d_b14, eq54_e944_d_b15, eq54_e944_d_b16, eq54_e944_d_b17, eq54_e944_d_b18, eq54_e944_d_b19, eq54_e944_d_b20, eq54_e944_d_b21, eq54_e944_d_b22, eq54_e944_d_b23, eq54_e944_d_b24, eq54_e944_d_b25, eq54_e944_d_b26, eq54_e944_d_b27, eq54_e944_d_b28, eq54_e944_d_b29, eq54_e944_d_b30, eq54_e944_d_b31, eq54_e944_d_b32, eq54_e944_d_b33, eq54_e944_d_b34, eq54_e944_d_b35, eq54_e944_d_b36, eq54_e944_d_b37, eq54_e944_d_b38, eq54_e944_d_b39, eq54_e944_d_b40, eq54_e944_d_b41, eq54_e944_d_b42, eq54_e944_d_b43, eq54_e944_d_b44, eq54_e944_d_b45, eq54_e944_d_b46, eq54_e944_d_b47, eq54_e944_d_b48, eq54_e944_d_b49, eq54_e944_d_b50, eq54_e944_d_b51, eq54_e944_d_b52, eq54_e944_d_b53, eq54_e944_d_b54,) = {
    if s.b[423] {
        let eq54_e942: f64 = (p.p6 * s.v[207]);
        let eq54_e942_d_n0: f64 = (p.p6 * s.dn[207][0]);
        let eq54_e942_d_n1: f64 = (p.p6 * s.dn[207][1]);
        let eq54_e942_d_n2: f64 = (p.p6 * s.dn[207][2]);
        let eq54_e942_d_n3: f64 = (p.p6 * s.dn[207][3]);
        let eq54_e942_d_n4: f64 = (p.p6 * s.dn[207][4]);
        let eq54_e942_d_n5: f64 = (p.p6 * s.dn[207][5]);
        let eq54_e942_d_n6: f64 = (p.p6 * s.dn[207][6]);
        let eq54_e942_d_n7: f64 = (p.p6 * s.dn[207][7]);
        let eq54_e942_d_n8: f64 = (p.p6 * s.dn[207][8]);
        let eq54_e942_d_n9: f64 = (p.p6 * s.dn[207][9]);
        let eq54_e942_d_n10: f64 = (p.p6 * s.dn[207][10]);
        let eq54_e942_d_n11: f64 = (p.p6 * s.dn[207][11]);
        let eq54_e942_d_n12: f64 = (p.p6 * s.dn[207][12]);
        let eq54_e942_d_n13: f64 = (p.p6 * s.dn[207][13]);
        let eq54_e942_d_n14: f64 = (p.p6 * s.dn[207][14]);
        let eq54_e942_d_n15: f64 = (p.p6 * s.dn[207][15]);
        let eq54_e942_d_n16: f64 = (p.p6 * s.dn[207][16]);
        let eq54_e942_d_n17: f64 = (p.p6 * s.dn[207][17]);
        let eq54_e942_d_n18: f64 = (p.p6 * s.dn[207][18]);
        let eq54_e942_d_n19: f64 = (p.p6 * s.dn[207][19]);
        let eq54_e942_d_n20: f64 = (p.p6 * s.dn[207][20]);
        let eq54_e942_d_n21: f64 = (p.p6 * s.dn[207][21]);
        let eq54_e942_d_n22: f64 = (p.p6 * s.dn[207][22]);
        let eq54_e942_d_b0: f64 = (p.p6 * s.db[207][0]);
        let eq54_e942_d_b1: f64 = (p.p6 * s.db[207][1]);
        let eq54_e942_d_b2: f64 = (p.p6 * s.db[207][2]);
        let eq54_e942_d_b3: f64 = (p.p6 * s.db[207][3]);
        let eq54_e942_d_b4: f64 = (p.p6 * s.db[207][4]);
        let eq54_e942_d_b5: f64 = (p.p6 * s.db[207][5]);
        let eq54_e942_d_b6: f64 = (p.p6 * s.db[207][6]);
        let eq54_e942_d_b7: f64 = (p.p6 * s.db[207][7]);
        let eq54_e942_d_b8: f64 = (p.p6 * s.db[207][8]);
        let eq54_e942_d_b9: f64 = (p.p6 * s.db[207][9]);
        let eq54_e942_d_b10: f64 = (p.p6 * s.db[207][10]);
        let eq54_e942_d_b11: f64 = (p.p6 * s.db[207][11]);
        let eq54_e942_d_b12: f64 = (p.p6 * s.db[207][12]);
        let eq54_e942_d_b13: f64 = (p.p6 * s.db[207][13]);
        let eq54_e942_d_b14: f64 = (p.p6 * s.db[207][14]);
        let eq54_e942_d_b15: f64 = (p.p6 * s.db[207][15]);
        let eq54_e942_d_b16: f64 = (p.p6 * s.db[207][16]);
        let eq54_e942_d_b17: f64 = (p.p6 * s.db[207][17]);
        let eq54_e942_d_b18: f64 = (p.p6 * s.db[207][18]);
        let eq54_e942_d_b19: f64 = (p.p6 * s.db[207][19]);
        let eq54_e942_d_b20: f64 = (p.p6 * s.db[207][20]);
        let eq54_e942_d_b21: f64 = (p.p6 * s.db[207][21]);
        let eq54_e942_d_b22: f64 = (p.p6 * s.db[207][22]);
        let eq54_e942_d_b23: f64 = (p.p6 * s.db[207][23]);
        let eq54_e942_d_b24: f64 = (p.p6 * s.db[207][24]);
        let eq54_e942_d_b25: f64 = (p.p6 * s.db[207][25]);
        let eq54_e942_d_b26: f64 = (p.p6 * s.db[207][26]);
        let eq54_e942_d_b27: f64 = (p.p6 * s.db[207][27]);
        let eq54_e942_d_b28: f64 = (p.p6 * s.db[207][28]);
        let eq54_e942_d_b29: f64 = (p.p6 * s.db[207][29]);
        let eq54_e942_d_b30: f64 = (p.p6 * s.db[207][30]);
        let eq54_e942_d_b31: f64 = (p.p6 * s.db[207][31]);
        let eq54_e942_d_b32: f64 = (p.p6 * s.db[207][32]);
        let eq54_e942_d_b33: f64 = (p.p6 * s.db[207][33]);
        let eq54_e942_d_b34: f64 = (p.p6 * s.db[207][34]);
        let eq54_e942_d_b35: f64 = (p.p6 * s.db[207][35]);
        let eq54_e942_d_b36: f64 = (p.p6 * s.db[207][36]);
        let eq54_e942_d_b37: f64 = (p.p6 * s.db[207][37]);
        let eq54_e942_d_b38: f64 = (p.p6 * s.db[207][38]);
        let eq54_e942_d_b39: f64 = (p.p6 * s.db[207][39]);
        let eq54_e942_d_b40: f64 = (p.p6 * s.db[207][40]);
        let eq54_e942_d_b41: f64 = (p.p6 * s.db[207][41]);
        let eq54_e942_d_b42: f64 = (p.p6 * s.db[207][42]);
        let eq54_e942_d_b43: f64 = (p.p6 * s.db[207][43]);
        let eq54_e942_d_b44: f64 = (p.p6 * s.db[207][44]);
        let eq54_e942_d_b45: f64 = (p.p6 * s.db[207][45]);
        let eq54_e942_d_b46: f64 = (p.p6 * s.db[207][46]);
        let eq54_e942_d_b47: f64 = (p.p6 * s.db[207][47]);
        let eq54_e942_d_b48: f64 = (p.p6 * s.db[207][48]);
        let eq54_e942_d_b49: f64 = (p.p6 * s.db[207][49]);
        let eq54_e942_d_b50: f64 = (p.p6 * s.db[207][50]);
        let eq54_e942_d_b51: f64 = (p.p6 * s.db[207][51]);
        let eq54_e942_d_b52: f64 = (p.p6 * s.db[207][52]);
        let eq54_e942_d_b53: f64 = (p.p6 * s.db[207][53]);
        let eq54_e942_d_b54: f64 = (p.p6 * s.db[207][54]);
        (eq54_e942, eq54_e942_d_n0, eq54_e942_d_n1, eq54_e942_d_n2, eq54_e942_d_n3, eq54_e942_d_n4, eq54_e942_d_n5, eq54_e942_d_n6, eq54_e942_d_n7, eq54_e942_d_n8, eq54_e942_d_n9, eq54_e942_d_n10, eq54_e942_d_n11, eq54_e942_d_n12, eq54_e942_d_n13, eq54_e942_d_n14, eq54_e942_d_n15, eq54_e942_d_n16, eq54_e942_d_n17, eq54_e942_d_n18, eq54_e942_d_n19, eq54_e942_d_n20, eq54_e942_d_n21, eq54_e942_d_n22, eq54_e942_d_b0, eq54_e942_d_b1, eq54_e942_d_b2, eq54_e942_d_b3, eq54_e942_d_b4, eq54_e942_d_b5, eq54_e942_d_b6, eq54_e942_d_b7, eq54_e942_d_b8, eq54_e942_d_b9, eq54_e942_d_b10, eq54_e942_d_b11, eq54_e942_d_b12, eq54_e942_d_b13, eq54_e942_d_b14, eq54_e942_d_b15, eq54_e942_d_b16, eq54_e942_d_b17, eq54_e942_d_b18, eq54_e942_d_b19, eq54_e942_d_b20, eq54_e942_d_b21, eq54_e942_d_b22, eq54_e942_d_b23, eq54_e942_d_b24, eq54_e942_d_b25, eq54_e942_d_b26, eq54_e942_d_b27, eq54_e942_d_b28, eq54_e942_d_b29, eq54_e942_d_b30, eq54_e942_d_b31, eq54_e942_d_b32, eq54_e942_d_b33, eq54_e942_d_b34, eq54_e942_d_b35, eq54_e942_d_b36, eq54_e942_d_b37, eq54_e942_d_b38, eq54_e942_d_b39, eq54_e942_d_b40, eq54_e942_d_b41, eq54_e942_d_b42, eq54_e942_d_b43, eq54_e942_d_b44, eq54_e942_d_b45, eq54_e942_d_b46, eq54_e942_d_b47, eq54_e942_d_b48, eq54_e942_d_b49, eq54_e942_d_b50, eq54_e942_d_b51, eq54_e942_d_b52, eq54_e942_d_b53, eq54_e942_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e944;
        let eq54_node_derivatives: [f64; 23] = [eq54_e944_d_n0, eq54_e944_d_n1, eq54_e944_d_n2, eq54_e944_d_n3, eq54_e944_d_n4, eq54_e944_d_n5, eq54_e944_d_n6, eq54_e944_d_n7, eq54_e944_d_n8, eq54_e944_d_n9, eq54_e944_d_n10, eq54_e944_d_n11, eq54_e944_d_n12, eq54_e944_d_n13, eq54_e944_d_n14, eq54_e944_d_n15, eq54_e944_d_n16, eq54_e944_d_n17, eq54_e944_d_n18, eq54_e944_d_n19, eq54_e944_d_n20, eq54_e944_d_n21, eq54_e944_d_n22];
        let eq54_branch_derivatives: [f64; 55] = [eq54_e944_d_b0, eq54_e944_d_b1, eq54_e944_d_b2, eq54_e944_d_b3, eq54_e944_d_b4, eq54_e944_d_b5, eq54_e944_d_b6, eq54_e944_d_b7, eq54_e944_d_b8, eq54_e944_d_b9, eq54_e944_d_b10, eq54_e944_d_b11, eq54_e944_d_b12, eq54_e944_d_b13, eq54_e944_d_b14, eq54_e944_d_b15, eq54_e944_d_b16, eq54_e944_d_b17, eq54_e944_d_b18, eq54_e944_d_b19, eq54_e944_d_b20, eq54_e944_d_b21, eq54_e944_d_b22, eq54_e944_d_b23, eq54_e944_d_b24, eq54_e944_d_b25, eq54_e944_d_b26, eq54_e944_d_b27, eq54_e944_d_b28, eq54_e944_d_b29, eq54_e944_d_b30, eq54_e944_d_b31, eq54_e944_d_b32, eq54_e944_d_b33, eq54_e944_d_b34, eq54_e944_d_b35, eq54_e944_d_b36, eq54_e944_d_b37, eq54_e944_d_b38, eq54_e944_d_b39, eq54_e944_d_b40, eq54_e944_d_b41, eq54_e944_d_b42, eq54_e944_d_b43, eq54_e944_d_b44, eq54_e944_d_b45, eq54_e944_d_b46, eq54_e944_d_b47, eq54_e944_d_b48, eq54_e944_d_b49, eq54_e944_d_b50, eq54_e944_d_b51, eq54_e944_d_b52, eq54_e944_d_b53, eq54_e944_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq54_value),
            &eq54_node_derivatives,
            &eq54_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_9(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq55_e957, eq55_e957_d_n0, eq55_e957_d_n1, eq55_e957_d_n2, eq55_e957_d_n3, eq55_e957_d_n4, eq55_e957_d_n5, eq55_e957_d_n6, eq55_e957_d_n7, eq55_e957_d_n8, eq55_e957_d_n9, eq55_e957_d_n10, eq55_e957_d_n11, eq55_e957_d_n12, eq55_e957_d_n13, eq55_e957_d_n14, eq55_e957_d_n15, eq55_e957_d_n16, eq55_e957_d_n17, eq55_e957_d_n18, eq55_e957_d_n19, eq55_e957_d_n20, eq55_e957_d_n21, eq55_e957_d_n22, eq55_e957_d_b0, eq55_e957_d_b1, eq55_e957_d_b2, eq55_e957_d_b3, eq55_e957_d_b4, eq55_e957_d_b5, eq55_e957_d_b6, eq55_e957_d_b7, eq55_e957_d_b8, eq55_e957_d_b9, eq55_e957_d_b10, eq55_e957_d_b11, eq55_e957_d_b12, eq55_e957_d_b13, eq55_e957_d_b14, eq55_e957_d_b15, eq55_e957_d_b16, eq55_e957_d_b17, eq55_e957_d_b18, eq55_e957_d_b19, eq55_e957_d_b20, eq55_e957_d_b21, eq55_e957_d_b22, eq55_e957_d_b23, eq55_e957_d_b24, eq55_e957_d_b25, eq55_e957_d_b26, eq55_e957_d_b27, eq55_e957_d_b28, eq55_e957_d_b29, eq55_e957_d_b30, eq55_e957_d_b31, eq55_e957_d_b32, eq55_e957_d_b33, eq55_e957_d_b34, eq55_e957_d_b35, eq55_e957_d_b36, eq55_e957_d_b37, eq55_e957_d_b38, eq55_e957_d_b39, eq55_e957_d_b40, eq55_e957_d_b41, eq55_e957_d_b42, eq55_e957_d_b43, eq55_e957_d_b44, eq55_e957_d_b45, eq55_e957_d_b46, eq55_e957_d_b47, eq55_e957_d_b48, eq55_e957_d_b49, eq55_e957_d_b50, eq55_e957_d_b51, eq55_e957_d_b52, eq55_e957_d_b53, eq55_e957_d_b54,) = {
    if (!s.b[423]) {
        let eq55_e951: f64 = 0.0;
        let eq55_e953: f64 = (eq55_e951 * (nv9 - nv8));
        let eq55_e953_d_n8: f64 = (-eq55_e951);
        let eq55_e954: f64 = (s.v[206] + eq55_e953);
        let eq55_e954_d_n8: f64 = (s.dn[206][8] + eq55_e953_d_n8);
        let eq55_e954_d_n9: f64 = (s.dn[206][9] + eq55_e951);
        let eq55_e955: f64 = (p.p6 * eq55_e954);
        let eq55_e955_d_n0: f64 = (p.p6 * s.dn[206][0]);
        let eq55_e955_d_n1: f64 = (p.p6 * s.dn[206][1]);
        let eq55_e955_d_n2: f64 = (p.p6 * s.dn[206][2]);
        let eq55_e955_d_n3: f64 = (p.p6 * s.dn[206][3]);
        let eq55_e955_d_n4: f64 = (p.p6 * s.dn[206][4]);
        let eq55_e955_d_n5: f64 = (p.p6 * s.dn[206][5]);
        let eq55_e955_d_n6: f64 = (p.p6 * s.dn[206][6]);
        let eq55_e955_d_n7: f64 = (p.p6 * s.dn[206][7]);
        let eq55_e955_d_n8: f64 = (p.p6 * eq55_e954_d_n8);
        let eq55_e955_d_n9: f64 = (p.p6 * eq55_e954_d_n9);
        let eq55_e955_d_n10: f64 = (p.p6 * s.dn[206][10]);
        let eq55_e955_d_n11: f64 = (p.p6 * s.dn[206][11]);
        let eq55_e955_d_n12: f64 = (p.p6 * s.dn[206][12]);
        let eq55_e955_d_n13: f64 = (p.p6 * s.dn[206][13]);
        let eq55_e955_d_n14: f64 = (p.p6 * s.dn[206][14]);
        let eq55_e955_d_n15: f64 = (p.p6 * s.dn[206][15]);
        let eq55_e955_d_n16: f64 = (p.p6 * s.dn[206][16]);
        let eq55_e955_d_n17: f64 = (p.p6 * s.dn[206][17]);
        let eq55_e955_d_n18: f64 = (p.p6 * s.dn[206][18]);
        let eq55_e955_d_n19: f64 = (p.p6 * s.dn[206][19]);
        let eq55_e955_d_n20: f64 = (p.p6 * s.dn[206][20]);
        let eq55_e955_d_n21: f64 = (p.p6 * s.dn[206][21]);
        let eq55_e955_d_n22: f64 = (p.p6 * s.dn[206][22]);
        let eq55_e955_d_b0: f64 = (p.p6 * s.db[206][0]);
        let eq55_e955_d_b1: f64 = (p.p6 * s.db[206][1]);
        let eq55_e955_d_b2: f64 = (p.p6 * s.db[206][2]);
        let eq55_e955_d_b3: f64 = (p.p6 * s.db[206][3]);
        let eq55_e955_d_b4: f64 = (p.p6 * s.db[206][4]);
        let eq55_e955_d_b5: f64 = (p.p6 * s.db[206][5]);
        let eq55_e955_d_b6: f64 = (p.p6 * s.db[206][6]);
        let eq55_e955_d_b7: f64 = (p.p6 * s.db[206][7]);
        let eq55_e955_d_b8: f64 = (p.p6 * s.db[206][8]);
        let eq55_e955_d_b9: f64 = (p.p6 * s.db[206][9]);
        let eq55_e955_d_b10: f64 = (p.p6 * s.db[206][10]);
        let eq55_e955_d_b11: f64 = (p.p6 * s.db[206][11]);
        let eq55_e955_d_b12: f64 = (p.p6 * s.db[206][12]);
        let eq55_e955_d_b13: f64 = (p.p6 * s.db[206][13]);
        let eq55_e955_d_b14: f64 = (p.p6 * s.db[206][14]);
        let eq55_e955_d_b15: f64 = (p.p6 * s.db[206][15]);
        let eq55_e955_d_b16: f64 = (p.p6 * s.db[206][16]);
        let eq55_e955_d_b17: f64 = (p.p6 * s.db[206][17]);
        let eq55_e955_d_b18: f64 = (p.p6 * s.db[206][18]);
        let eq55_e955_d_b19: f64 = (p.p6 * s.db[206][19]);
        let eq55_e955_d_b20: f64 = (p.p6 * s.db[206][20]);
        let eq55_e955_d_b21: f64 = (p.p6 * s.db[206][21]);
        let eq55_e955_d_b22: f64 = (p.p6 * s.db[206][22]);
        let eq55_e955_d_b23: f64 = (p.p6 * s.db[206][23]);
        let eq55_e955_d_b24: f64 = (p.p6 * s.db[206][24]);
        let eq55_e955_d_b25: f64 = (p.p6 * s.db[206][25]);
        let eq55_e955_d_b26: f64 = (p.p6 * s.db[206][26]);
        let eq55_e955_d_b27: f64 = (p.p6 * s.db[206][27]);
        let eq55_e955_d_b28: f64 = (p.p6 * s.db[206][28]);
        let eq55_e955_d_b29: f64 = (p.p6 * s.db[206][29]);
        let eq55_e955_d_b30: f64 = (p.p6 * s.db[206][30]);
        let eq55_e955_d_b31: f64 = (p.p6 * s.db[206][31]);
        let eq55_e955_d_b32: f64 = (p.p6 * s.db[206][32]);
        let eq55_e955_d_b33: f64 = (p.p6 * s.db[206][33]);
        let eq55_e955_d_b34: f64 = (p.p6 * s.db[206][34]);
        let eq55_e955_d_b35: f64 = (p.p6 * s.db[206][35]);
        let eq55_e955_d_b36: f64 = (p.p6 * s.db[206][36]);
        let eq55_e955_d_b37: f64 = (p.p6 * s.db[206][37]);
        let eq55_e955_d_b38: f64 = (p.p6 * s.db[206][38]);
        let eq55_e955_d_b39: f64 = (p.p6 * s.db[206][39]);
        let eq55_e955_d_b40: f64 = (p.p6 * s.db[206][40]);
        let eq55_e955_d_b41: f64 = (p.p6 * s.db[206][41]);
        let eq55_e955_d_b42: f64 = (p.p6 * s.db[206][42]);
        let eq55_e955_d_b43: f64 = (p.p6 * s.db[206][43]);
        let eq55_e955_d_b44: f64 = (p.p6 * s.db[206][44]);
        let eq55_e955_d_b45: f64 = (p.p6 * s.db[206][45]);
        let eq55_e955_d_b46: f64 = (p.p6 * s.db[206][46]);
        let eq55_e955_d_b47: f64 = (p.p6 * s.db[206][47]);
        let eq55_e955_d_b48: f64 = (p.p6 * s.db[206][48]);
        let eq55_e955_d_b49: f64 = (p.p6 * s.db[206][49]);
        let eq55_e955_d_b50: f64 = (p.p6 * s.db[206][50]);
        let eq55_e955_d_b51: f64 = (p.p6 * s.db[206][51]);
        let eq55_e955_d_b52: f64 = (p.p6 * s.db[206][52]);
        let eq55_e955_d_b53: f64 = (p.p6 * s.db[206][53]);
        let eq55_e955_d_b54: f64 = (p.p6 * s.db[206][54]);
        (eq55_e955, eq55_e955_d_n0, eq55_e955_d_n1, eq55_e955_d_n2, eq55_e955_d_n3, eq55_e955_d_n4, eq55_e955_d_n5, eq55_e955_d_n6, eq55_e955_d_n7, eq55_e955_d_n8, eq55_e955_d_n9, eq55_e955_d_n10, eq55_e955_d_n11, eq55_e955_d_n12, eq55_e955_d_n13, eq55_e955_d_n14, eq55_e955_d_n15, eq55_e955_d_n16, eq55_e955_d_n17, eq55_e955_d_n18, eq55_e955_d_n19, eq55_e955_d_n20, eq55_e955_d_n21, eq55_e955_d_n22, eq55_e955_d_b0, eq55_e955_d_b1, eq55_e955_d_b2, eq55_e955_d_b3, eq55_e955_d_b4, eq55_e955_d_b5, eq55_e955_d_b6, eq55_e955_d_b7, eq55_e955_d_b8, eq55_e955_d_b9, eq55_e955_d_b10, eq55_e955_d_b11, eq55_e955_d_b12, eq55_e955_d_b13, eq55_e955_d_b14, eq55_e955_d_b15, eq55_e955_d_b16, eq55_e955_d_b17, eq55_e955_d_b18, eq55_e955_d_b19, eq55_e955_d_b20, eq55_e955_d_b21, eq55_e955_d_b22, eq55_e955_d_b23, eq55_e955_d_b24, eq55_e955_d_b25, eq55_e955_d_b26, eq55_e955_d_b27, eq55_e955_d_b28, eq55_e955_d_b29, eq55_e955_d_b30, eq55_e955_d_b31, eq55_e955_d_b32, eq55_e955_d_b33, eq55_e955_d_b34, eq55_e955_d_b35, eq55_e955_d_b36, eq55_e955_d_b37, eq55_e955_d_b38, eq55_e955_d_b39, eq55_e955_d_b40, eq55_e955_d_b41, eq55_e955_d_b42, eq55_e955_d_b43, eq55_e955_d_b44, eq55_e955_d_b45, eq55_e955_d_b46, eq55_e955_d_b47, eq55_e955_d_b48, eq55_e955_d_b49, eq55_e955_d_b50, eq55_e955_d_b51, eq55_e955_d_b52, eq55_e955_d_b53, eq55_e955_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e957;
        let eq55_node_derivatives: [f64; 23] = [eq55_e957_d_n0, eq55_e957_d_n1, eq55_e957_d_n2, eq55_e957_d_n3, eq55_e957_d_n4, eq55_e957_d_n5, eq55_e957_d_n6, eq55_e957_d_n7, eq55_e957_d_n8, eq55_e957_d_n9, eq55_e957_d_n10, eq55_e957_d_n11, eq55_e957_d_n12, eq55_e957_d_n13, eq55_e957_d_n14, eq55_e957_d_n15, eq55_e957_d_n16, eq55_e957_d_n17, eq55_e957_d_n18, eq55_e957_d_n19, eq55_e957_d_n20, eq55_e957_d_n21, eq55_e957_d_n22];
        let eq55_branch_derivatives: [f64; 55] = [eq55_e957_d_b0, eq55_e957_d_b1, eq55_e957_d_b2, eq55_e957_d_b3, eq55_e957_d_b4, eq55_e957_d_b5, eq55_e957_d_b6, eq55_e957_d_b7, eq55_e957_d_b8, eq55_e957_d_b9, eq55_e957_d_b10, eq55_e957_d_b11, eq55_e957_d_b12, eq55_e957_d_b13, eq55_e957_d_b14, eq55_e957_d_b15, eq55_e957_d_b16, eq55_e957_d_b17, eq55_e957_d_b18, eq55_e957_d_b19, eq55_e957_d_b20, eq55_e957_d_b21, eq55_e957_d_b22, eq55_e957_d_b23, eq55_e957_d_b24, eq55_e957_d_b25, eq55_e957_d_b26, eq55_e957_d_b27, eq55_e957_d_b28, eq55_e957_d_b29, eq55_e957_d_b30, eq55_e957_d_b31, eq55_e957_d_b32, eq55_e957_d_b33, eq55_e957_d_b34, eq55_e957_d_b35, eq55_e957_d_b36, eq55_e957_d_b37, eq55_e957_d_b38, eq55_e957_d_b39, eq55_e957_d_b40, eq55_e957_d_b41, eq55_e957_d_b42, eq55_e957_d_b43, eq55_e957_d_b44, eq55_e957_d_b45, eq55_e957_d_b46, eq55_e957_d_b47, eq55_e957_d_b48, eq55_e957_d_b49, eq55_e957_d_b50, eq55_e957_d_b51, eq55_e957_d_b52, eq55_e957_d_b53, eq55_e957_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e970, eq56_e970_d_n0, eq56_e970_d_n1, eq56_e970_d_n2, eq56_e970_d_n3, eq56_e970_d_n4, eq56_e970_d_n5, eq56_e970_d_n6, eq56_e970_d_n7, eq56_e970_d_n8, eq56_e970_d_n9, eq56_e970_d_n10, eq56_e970_d_n11, eq56_e970_d_n12, eq56_e970_d_n13, eq56_e970_d_n14, eq56_e970_d_n15, eq56_e970_d_n16, eq56_e970_d_n17, eq56_e970_d_n18, eq56_e970_d_n19, eq56_e970_d_n20, eq56_e970_d_n21, eq56_e970_d_n22, eq56_e970_d_b0, eq56_e970_d_b1, eq56_e970_d_b2, eq56_e970_d_b3, eq56_e970_d_b4, eq56_e970_d_b5, eq56_e970_d_b6, eq56_e970_d_b7, eq56_e970_d_b8, eq56_e970_d_b9, eq56_e970_d_b10, eq56_e970_d_b11, eq56_e970_d_b12, eq56_e970_d_b13, eq56_e970_d_b14, eq56_e970_d_b15, eq56_e970_d_b16, eq56_e970_d_b17, eq56_e970_d_b18, eq56_e970_d_b19, eq56_e970_d_b20, eq56_e970_d_b21, eq56_e970_d_b22, eq56_e970_d_b23, eq56_e970_d_b24, eq56_e970_d_b25, eq56_e970_d_b26, eq56_e970_d_b27, eq56_e970_d_b28, eq56_e970_d_b29, eq56_e970_d_b30, eq56_e970_d_b31, eq56_e970_d_b32, eq56_e970_d_b33, eq56_e970_d_b34, eq56_e970_d_b35, eq56_e970_d_b36, eq56_e970_d_b37, eq56_e970_d_b38, eq56_e970_d_b39, eq56_e970_d_b40, eq56_e970_d_b41, eq56_e970_d_b42, eq56_e970_d_b43, eq56_e970_d_b44, eq56_e970_d_b45, eq56_e970_d_b46, eq56_e970_d_b47, eq56_e970_d_b48, eq56_e970_d_b49, eq56_e970_d_b50, eq56_e970_d_b51, eq56_e970_d_b52, eq56_e970_d_b53, eq56_e970_d_b54,) = {
    if (!s.b[423]) {
        let eq56_e964: f64 = 0.0;
        let eq56_e966: f64 = (eq56_e964 * (nv9 - nv7));
        let eq56_e966_d_n7: f64 = (-eq56_e964);
        let eq56_e967: f64 = (s.v[207] + eq56_e966);
        let eq56_e967_d_n7: f64 = (s.dn[207][7] + eq56_e966_d_n7);
        let eq56_e967_d_n9: f64 = (s.dn[207][9] + eq56_e964);
        let eq56_e968: f64 = (p.p6 * eq56_e967);
        let eq56_e968_d_n0: f64 = (p.p6 * s.dn[207][0]);
        let eq56_e968_d_n1: f64 = (p.p6 * s.dn[207][1]);
        let eq56_e968_d_n2: f64 = (p.p6 * s.dn[207][2]);
        let eq56_e968_d_n3: f64 = (p.p6 * s.dn[207][3]);
        let eq56_e968_d_n4: f64 = (p.p6 * s.dn[207][4]);
        let eq56_e968_d_n5: f64 = (p.p6 * s.dn[207][5]);
        let eq56_e968_d_n6: f64 = (p.p6 * s.dn[207][6]);
        let eq56_e968_d_n7: f64 = (p.p6 * eq56_e967_d_n7);
        let eq56_e968_d_n8: f64 = (p.p6 * s.dn[207][8]);
        let eq56_e968_d_n9: f64 = (p.p6 * eq56_e967_d_n9);
        let eq56_e968_d_n10: f64 = (p.p6 * s.dn[207][10]);
        let eq56_e968_d_n11: f64 = (p.p6 * s.dn[207][11]);
        let eq56_e968_d_n12: f64 = (p.p6 * s.dn[207][12]);
        let eq56_e968_d_n13: f64 = (p.p6 * s.dn[207][13]);
        let eq56_e968_d_n14: f64 = (p.p6 * s.dn[207][14]);
        let eq56_e968_d_n15: f64 = (p.p6 * s.dn[207][15]);
        let eq56_e968_d_n16: f64 = (p.p6 * s.dn[207][16]);
        let eq56_e968_d_n17: f64 = (p.p6 * s.dn[207][17]);
        let eq56_e968_d_n18: f64 = (p.p6 * s.dn[207][18]);
        let eq56_e968_d_n19: f64 = (p.p6 * s.dn[207][19]);
        let eq56_e968_d_n20: f64 = (p.p6 * s.dn[207][20]);
        let eq56_e968_d_n21: f64 = (p.p6 * s.dn[207][21]);
        let eq56_e968_d_n22: f64 = (p.p6 * s.dn[207][22]);
        let eq56_e968_d_b0: f64 = (p.p6 * s.db[207][0]);
        let eq56_e968_d_b1: f64 = (p.p6 * s.db[207][1]);
        let eq56_e968_d_b2: f64 = (p.p6 * s.db[207][2]);
        let eq56_e968_d_b3: f64 = (p.p6 * s.db[207][3]);
        let eq56_e968_d_b4: f64 = (p.p6 * s.db[207][4]);
        let eq56_e968_d_b5: f64 = (p.p6 * s.db[207][5]);
        let eq56_e968_d_b6: f64 = (p.p6 * s.db[207][6]);
        let eq56_e968_d_b7: f64 = (p.p6 * s.db[207][7]);
        let eq56_e968_d_b8: f64 = (p.p6 * s.db[207][8]);
        let eq56_e968_d_b9: f64 = (p.p6 * s.db[207][9]);
        let eq56_e968_d_b10: f64 = (p.p6 * s.db[207][10]);
        let eq56_e968_d_b11: f64 = (p.p6 * s.db[207][11]);
        let eq56_e968_d_b12: f64 = (p.p6 * s.db[207][12]);
        let eq56_e968_d_b13: f64 = (p.p6 * s.db[207][13]);
        let eq56_e968_d_b14: f64 = (p.p6 * s.db[207][14]);
        let eq56_e968_d_b15: f64 = (p.p6 * s.db[207][15]);
        let eq56_e968_d_b16: f64 = (p.p6 * s.db[207][16]);
        let eq56_e968_d_b17: f64 = (p.p6 * s.db[207][17]);
        let eq56_e968_d_b18: f64 = (p.p6 * s.db[207][18]);
        let eq56_e968_d_b19: f64 = (p.p6 * s.db[207][19]);
        let eq56_e968_d_b20: f64 = (p.p6 * s.db[207][20]);
        let eq56_e968_d_b21: f64 = (p.p6 * s.db[207][21]);
        let eq56_e968_d_b22: f64 = (p.p6 * s.db[207][22]);
        let eq56_e968_d_b23: f64 = (p.p6 * s.db[207][23]);
        let eq56_e968_d_b24: f64 = (p.p6 * s.db[207][24]);
        let eq56_e968_d_b25: f64 = (p.p6 * s.db[207][25]);
        let eq56_e968_d_b26: f64 = (p.p6 * s.db[207][26]);
        let eq56_e968_d_b27: f64 = (p.p6 * s.db[207][27]);
        let eq56_e968_d_b28: f64 = (p.p6 * s.db[207][28]);
        let eq56_e968_d_b29: f64 = (p.p6 * s.db[207][29]);
        let eq56_e968_d_b30: f64 = (p.p6 * s.db[207][30]);
        let eq56_e968_d_b31: f64 = (p.p6 * s.db[207][31]);
        let eq56_e968_d_b32: f64 = (p.p6 * s.db[207][32]);
        let eq56_e968_d_b33: f64 = (p.p6 * s.db[207][33]);
        let eq56_e968_d_b34: f64 = (p.p6 * s.db[207][34]);
        let eq56_e968_d_b35: f64 = (p.p6 * s.db[207][35]);
        let eq56_e968_d_b36: f64 = (p.p6 * s.db[207][36]);
        let eq56_e968_d_b37: f64 = (p.p6 * s.db[207][37]);
        let eq56_e968_d_b38: f64 = (p.p6 * s.db[207][38]);
        let eq56_e968_d_b39: f64 = (p.p6 * s.db[207][39]);
        let eq56_e968_d_b40: f64 = (p.p6 * s.db[207][40]);
        let eq56_e968_d_b41: f64 = (p.p6 * s.db[207][41]);
        let eq56_e968_d_b42: f64 = (p.p6 * s.db[207][42]);
        let eq56_e968_d_b43: f64 = (p.p6 * s.db[207][43]);
        let eq56_e968_d_b44: f64 = (p.p6 * s.db[207][44]);
        let eq56_e968_d_b45: f64 = (p.p6 * s.db[207][45]);
        let eq56_e968_d_b46: f64 = (p.p6 * s.db[207][46]);
        let eq56_e968_d_b47: f64 = (p.p6 * s.db[207][47]);
        let eq56_e968_d_b48: f64 = (p.p6 * s.db[207][48]);
        let eq56_e968_d_b49: f64 = (p.p6 * s.db[207][49]);
        let eq56_e968_d_b50: f64 = (p.p6 * s.db[207][50]);
        let eq56_e968_d_b51: f64 = (p.p6 * s.db[207][51]);
        let eq56_e968_d_b52: f64 = (p.p6 * s.db[207][52]);
        let eq56_e968_d_b53: f64 = (p.p6 * s.db[207][53]);
        let eq56_e968_d_b54: f64 = (p.p6 * s.db[207][54]);
        (eq56_e968, eq56_e968_d_n0, eq56_e968_d_n1, eq56_e968_d_n2, eq56_e968_d_n3, eq56_e968_d_n4, eq56_e968_d_n5, eq56_e968_d_n6, eq56_e968_d_n7, eq56_e968_d_n8, eq56_e968_d_n9, eq56_e968_d_n10, eq56_e968_d_n11, eq56_e968_d_n12, eq56_e968_d_n13, eq56_e968_d_n14, eq56_e968_d_n15, eq56_e968_d_n16, eq56_e968_d_n17, eq56_e968_d_n18, eq56_e968_d_n19, eq56_e968_d_n20, eq56_e968_d_n21, eq56_e968_d_n22, eq56_e968_d_b0, eq56_e968_d_b1, eq56_e968_d_b2, eq56_e968_d_b3, eq56_e968_d_b4, eq56_e968_d_b5, eq56_e968_d_b6, eq56_e968_d_b7, eq56_e968_d_b8, eq56_e968_d_b9, eq56_e968_d_b10, eq56_e968_d_b11, eq56_e968_d_b12, eq56_e968_d_b13, eq56_e968_d_b14, eq56_e968_d_b15, eq56_e968_d_b16, eq56_e968_d_b17, eq56_e968_d_b18, eq56_e968_d_b19, eq56_e968_d_b20, eq56_e968_d_b21, eq56_e968_d_b22, eq56_e968_d_b23, eq56_e968_d_b24, eq56_e968_d_b25, eq56_e968_d_b26, eq56_e968_d_b27, eq56_e968_d_b28, eq56_e968_d_b29, eq56_e968_d_b30, eq56_e968_d_b31, eq56_e968_d_b32, eq56_e968_d_b33, eq56_e968_d_b34, eq56_e968_d_b35, eq56_e968_d_b36, eq56_e968_d_b37, eq56_e968_d_b38, eq56_e968_d_b39, eq56_e968_d_b40, eq56_e968_d_b41, eq56_e968_d_b42, eq56_e968_d_b43, eq56_e968_d_b44, eq56_e968_d_b45, eq56_e968_d_b46, eq56_e968_d_b47, eq56_e968_d_b48, eq56_e968_d_b49, eq56_e968_d_b50, eq56_e968_d_b51, eq56_e968_d_b52, eq56_e968_d_b53, eq56_e968_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e970;
        let eq56_node_derivatives: [f64; 23] = [eq56_e970_d_n0, eq56_e970_d_n1, eq56_e970_d_n2, eq56_e970_d_n3, eq56_e970_d_n4, eq56_e970_d_n5, eq56_e970_d_n6, eq56_e970_d_n7, eq56_e970_d_n8, eq56_e970_d_n9, eq56_e970_d_n10, eq56_e970_d_n11, eq56_e970_d_n12, eq56_e970_d_n13, eq56_e970_d_n14, eq56_e970_d_n15, eq56_e970_d_n16, eq56_e970_d_n17, eq56_e970_d_n18, eq56_e970_d_n19, eq56_e970_d_n20, eq56_e970_d_n21, eq56_e970_d_n22];
        let eq56_branch_derivatives: [f64; 55] = [eq56_e970_d_b0, eq56_e970_d_b1, eq56_e970_d_b2, eq56_e970_d_b3, eq56_e970_d_b4, eq56_e970_d_b5, eq56_e970_d_b6, eq56_e970_d_b7, eq56_e970_d_b8, eq56_e970_d_b9, eq56_e970_d_b10, eq56_e970_d_b11, eq56_e970_d_b12, eq56_e970_d_b13, eq56_e970_d_b14, eq56_e970_d_b15, eq56_e970_d_b16, eq56_e970_d_b17, eq56_e970_d_b18, eq56_e970_d_b19, eq56_e970_d_b20, eq56_e970_d_b21, eq56_e970_d_b22, eq56_e970_d_b23, eq56_e970_d_b24, eq56_e970_d_b25, eq56_e970_d_b26, eq56_e970_d_b27, eq56_e970_d_b28, eq56_e970_d_b29, eq56_e970_d_b30, eq56_e970_d_b31, eq56_e970_d_b32, eq56_e970_d_b33, eq56_e970_d_b34, eq56_e970_d_b35, eq56_e970_d_b36, eq56_e970_d_b37, eq56_e970_d_b38, eq56_e970_d_b39, eq56_e970_d_b40, eq56_e970_d_b41, eq56_e970_d_b42, eq56_e970_d_b43, eq56_e970_d_b44, eq56_e970_d_b45, eq56_e970_d_b46, eq56_e970_d_b47, eq56_e970_d_b48, eq56_e970_d_b49, eq56_e970_d_b50, eq56_e970_d_b51, eq56_e970_d_b52, eq56_e970_d_b53, eq56_e970_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e980, eq57_e980_d_n0, eq57_e980_d_n1, eq57_e980_d_n2, eq57_e980_d_n3, eq57_e980_d_n4, eq57_e980_d_n5, eq57_e980_d_n6, eq57_e980_d_n7, eq57_e980_d_n8, eq57_e980_d_n9, eq57_e980_d_n10, eq57_e980_d_n11, eq57_e980_d_n12, eq57_e980_d_n13, eq57_e980_d_n14, eq57_e980_d_n15, eq57_e980_d_n16, eq57_e980_d_n17, eq57_e980_d_n18, eq57_e980_d_n19, eq57_e980_d_n20, eq57_e980_d_n21, eq57_e980_d_n22, eq57_e980_d_b0, eq57_e980_d_b1, eq57_e980_d_b2, eq57_e980_d_b3, eq57_e980_d_b4, eq57_e980_d_b5, eq57_e980_d_b6, eq57_e980_d_b7, eq57_e980_d_b8, eq57_e980_d_b9, eq57_e980_d_b10, eq57_e980_d_b11, eq57_e980_d_b12, eq57_e980_d_b13, eq57_e980_d_b14, eq57_e980_d_b15, eq57_e980_d_b16, eq57_e980_d_b17, eq57_e980_d_b18, eq57_e980_d_b19, eq57_e980_d_b20, eq57_e980_d_b21, eq57_e980_d_b22, eq57_e980_d_b23, eq57_e980_d_b24, eq57_e980_d_b25, eq57_e980_d_b26, eq57_e980_d_b27, eq57_e980_d_b28, eq57_e980_d_b29, eq57_e980_d_b30, eq57_e980_d_b31, eq57_e980_d_b32, eq57_e980_d_b33, eq57_e980_d_b34, eq57_e980_d_b35, eq57_e980_d_b36, eq57_e980_d_b37, eq57_e980_d_b38, eq57_e980_d_b39, eq57_e980_d_b40, eq57_e980_d_b41, eq57_e980_d_b42, eq57_e980_d_b43, eq57_e980_d_b44, eq57_e980_d_b45, eq57_e980_d_b46, eq57_e980_d_b47, eq57_e980_d_b48, eq57_e980_d_b49, eq57_e980_d_b50, eq57_e980_d_b51, eq57_e980_d_b52, eq57_e980_d_b53, eq57_e980_d_b54,) = {
    if (s.b[424] && s.b[427]) {
        let eq57_e976: f64 = (p.p6 * s.v[142]);
        let eq57_e976_d_n0: f64 = (p.p6 * s.dn[142][0]);
        let eq57_e976_d_n1: f64 = (p.p6 * s.dn[142][1]);
        let eq57_e976_d_n2: f64 = (p.p6 * s.dn[142][2]);
        let eq57_e976_d_n3: f64 = (p.p6 * s.dn[142][3]);
        let eq57_e976_d_n4: f64 = (p.p6 * s.dn[142][4]);
        let eq57_e976_d_n5: f64 = (p.p6 * s.dn[142][5]);
        let eq57_e976_d_n6: f64 = (p.p6 * s.dn[142][6]);
        let eq57_e976_d_n7: f64 = (p.p6 * s.dn[142][7]);
        let eq57_e976_d_n8: f64 = (p.p6 * s.dn[142][8]);
        let eq57_e976_d_n9: f64 = (p.p6 * s.dn[142][9]);
        let eq57_e976_d_n10: f64 = (p.p6 * s.dn[142][10]);
        let eq57_e976_d_n11: f64 = (p.p6 * s.dn[142][11]);
        let eq57_e976_d_n12: f64 = (p.p6 * s.dn[142][12]);
        let eq57_e976_d_n13: f64 = (p.p6 * s.dn[142][13]);
        let eq57_e976_d_n14: f64 = (p.p6 * s.dn[142][14]);
        let eq57_e976_d_n15: f64 = (p.p6 * s.dn[142][15]);
        let eq57_e976_d_n16: f64 = (p.p6 * s.dn[142][16]);
        let eq57_e976_d_n17: f64 = (p.p6 * s.dn[142][17]);
        let eq57_e976_d_n18: f64 = (p.p6 * s.dn[142][18]);
        let eq57_e976_d_n19: f64 = (p.p6 * s.dn[142][19]);
        let eq57_e976_d_n20: f64 = (p.p6 * s.dn[142][20]);
        let eq57_e976_d_n21: f64 = (p.p6 * s.dn[142][21]);
        let eq57_e976_d_n22: f64 = (p.p6 * s.dn[142][22]);
        let eq57_e976_d_b0: f64 = (p.p6 * s.db[142][0]);
        let eq57_e976_d_b1: f64 = (p.p6 * s.db[142][1]);
        let eq57_e976_d_b2: f64 = (p.p6 * s.db[142][2]);
        let eq57_e976_d_b3: f64 = (p.p6 * s.db[142][3]);
        let eq57_e976_d_b4: f64 = (p.p6 * s.db[142][4]);
        let eq57_e976_d_b5: f64 = (p.p6 * s.db[142][5]);
        let eq57_e976_d_b6: f64 = (p.p6 * s.db[142][6]);
        let eq57_e976_d_b7: f64 = (p.p6 * s.db[142][7]);
        let eq57_e976_d_b8: f64 = (p.p6 * s.db[142][8]);
        let eq57_e976_d_b9: f64 = (p.p6 * s.db[142][9]);
        let eq57_e976_d_b10: f64 = (p.p6 * s.db[142][10]);
        let eq57_e976_d_b11: f64 = (p.p6 * s.db[142][11]);
        let eq57_e976_d_b12: f64 = (p.p6 * s.db[142][12]);
        let eq57_e976_d_b13: f64 = (p.p6 * s.db[142][13]);
        let eq57_e976_d_b14: f64 = (p.p6 * s.db[142][14]);
        let eq57_e976_d_b15: f64 = (p.p6 * s.db[142][15]);
        let eq57_e976_d_b16: f64 = (p.p6 * s.db[142][16]);
        let eq57_e976_d_b17: f64 = (p.p6 * s.db[142][17]);
        let eq57_e976_d_b18: f64 = (p.p6 * s.db[142][18]);
        let eq57_e976_d_b19: f64 = (p.p6 * s.db[142][19]);
        let eq57_e976_d_b20: f64 = (p.p6 * s.db[142][20]);
        let eq57_e976_d_b21: f64 = (p.p6 * s.db[142][21]);
        let eq57_e976_d_b22: f64 = (p.p6 * s.db[142][22]);
        let eq57_e976_d_b23: f64 = (p.p6 * s.db[142][23]);
        let eq57_e976_d_b24: f64 = (p.p6 * s.db[142][24]);
        let eq57_e976_d_b25: f64 = (p.p6 * s.db[142][25]);
        let eq57_e976_d_b26: f64 = (p.p6 * s.db[142][26]);
        let eq57_e976_d_b27: f64 = (p.p6 * s.db[142][27]);
        let eq57_e976_d_b28: f64 = (p.p6 * s.db[142][28]);
        let eq57_e976_d_b29: f64 = (p.p6 * s.db[142][29]);
        let eq57_e976_d_b30: f64 = (p.p6 * s.db[142][30]);
        let eq57_e976_d_b31: f64 = (p.p6 * s.db[142][31]);
        let eq57_e976_d_b32: f64 = (p.p6 * s.db[142][32]);
        let eq57_e976_d_b33: f64 = (p.p6 * s.db[142][33]);
        let eq57_e976_d_b34: f64 = (p.p6 * s.db[142][34]);
        let eq57_e976_d_b35: f64 = (p.p6 * s.db[142][35]);
        let eq57_e976_d_b36: f64 = (p.p6 * s.db[142][36]);
        let eq57_e976_d_b37: f64 = (p.p6 * s.db[142][37]);
        let eq57_e976_d_b38: f64 = (p.p6 * s.db[142][38]);
        let eq57_e976_d_b39: f64 = (p.p6 * s.db[142][39]);
        let eq57_e976_d_b40: f64 = (p.p6 * s.db[142][40]);
        let eq57_e976_d_b41: f64 = (p.p6 * s.db[142][41]);
        let eq57_e976_d_b42: f64 = (p.p6 * s.db[142][42]);
        let eq57_e976_d_b43: f64 = (p.p6 * s.db[142][43]);
        let eq57_e976_d_b44: f64 = (p.p6 * s.db[142][44]);
        let eq57_e976_d_b45: f64 = (p.p6 * s.db[142][45]);
        let eq57_e976_d_b46: f64 = (p.p6 * s.db[142][46]);
        let eq57_e976_d_b47: f64 = (p.p6 * s.db[142][47]);
        let eq57_e976_d_b48: f64 = (p.p6 * s.db[142][48]);
        let eq57_e976_d_b49: f64 = (p.p6 * s.db[142][49]);
        let eq57_e976_d_b50: f64 = (p.p6 * s.db[142][50]);
        let eq57_e976_d_b51: f64 = (p.p6 * s.db[142][51]);
        let eq57_e976_d_b52: f64 = (p.p6 * s.db[142][52]);
        let eq57_e976_d_b53: f64 = (p.p6 * s.db[142][53]);
        let eq57_e976_d_b54: f64 = (p.p6 * s.db[142][54]);
        let eq57_e978: f64 = (eq57_e976 * (nv0 - nv18));
        let eq57_e978_d_n0: f64 = ((eq57_e976_d_n0 * (nv0 - nv18)) + eq57_e976);
        let eq57_e978_d_n1: f64 = (eq57_e976_d_n1 * (nv0 - nv18));
        let eq57_e978_d_n2: f64 = (eq57_e976_d_n2 * (nv0 - nv18));
        let eq57_e978_d_n3: f64 = (eq57_e976_d_n3 * (nv0 - nv18));
        let eq57_e978_d_n4: f64 = (eq57_e976_d_n4 * (nv0 - nv18));
        let eq57_e978_d_n5: f64 = (eq57_e976_d_n5 * (nv0 - nv18));
        let eq57_e978_d_n6: f64 = (eq57_e976_d_n6 * (nv0 - nv18));
        let eq57_e978_d_n7: f64 = (eq57_e976_d_n7 * (nv0 - nv18));
        let eq57_e978_d_n8: f64 = (eq57_e976_d_n8 * (nv0 - nv18));
        let eq57_e978_d_n9: f64 = (eq57_e976_d_n9 * (nv0 - nv18));
        let eq57_e978_d_n10: f64 = (eq57_e976_d_n10 * (nv0 - nv18));
        let eq57_e978_d_n11: f64 = (eq57_e976_d_n11 * (nv0 - nv18));
        let eq57_e978_d_n12: f64 = (eq57_e976_d_n12 * (nv0 - nv18));
        let eq57_e978_d_n13: f64 = (eq57_e976_d_n13 * (nv0 - nv18));
        let eq57_e978_d_n14: f64 = (eq57_e976_d_n14 * (nv0 - nv18));
        let eq57_e978_d_n15: f64 = (eq57_e976_d_n15 * (nv0 - nv18));
        let eq57_e978_d_n16: f64 = (eq57_e976_d_n16 * (nv0 - nv18));
        let eq57_e978_d_n17: f64 = (eq57_e976_d_n17 * (nv0 - nv18));
        let eq57_e978_d_n18: f64 = ((eq57_e976_d_n18 * (nv0 - nv18)) + (-eq57_e976));
        let eq57_e978_d_n19: f64 = (eq57_e976_d_n19 * (nv0 - nv18));
        let eq57_e978_d_n20: f64 = (eq57_e976_d_n20 * (nv0 - nv18));
        let eq57_e978_d_n21: f64 = (eq57_e976_d_n21 * (nv0 - nv18));
        let eq57_e978_d_n22: f64 = (eq57_e976_d_n22 * (nv0 - nv18));
        let eq57_e978_d_b0: f64 = (eq57_e976_d_b0 * (nv0 - nv18));
        let eq57_e978_d_b1: f64 = (eq57_e976_d_b1 * (nv0 - nv18));
        let eq57_e978_d_b2: f64 = (eq57_e976_d_b2 * (nv0 - nv18));
        let eq57_e978_d_b3: f64 = (eq57_e976_d_b3 * (nv0 - nv18));
        let eq57_e978_d_b4: f64 = (eq57_e976_d_b4 * (nv0 - nv18));
        let eq57_e978_d_b5: f64 = (eq57_e976_d_b5 * (nv0 - nv18));
        let eq57_e978_d_b6: f64 = (eq57_e976_d_b6 * (nv0 - nv18));
        let eq57_e978_d_b7: f64 = (eq57_e976_d_b7 * (nv0 - nv18));
        let eq57_e978_d_b8: f64 = (eq57_e976_d_b8 * (nv0 - nv18));
        let eq57_e978_d_b9: f64 = (eq57_e976_d_b9 * (nv0 - nv18));
        let eq57_e978_d_b10: f64 = (eq57_e976_d_b10 * (nv0 - nv18));
        let eq57_e978_d_b11: f64 = (eq57_e976_d_b11 * (nv0 - nv18));
        let eq57_e978_d_b12: f64 = (eq57_e976_d_b12 * (nv0 - nv18));
        let eq57_e978_d_b13: f64 = (eq57_e976_d_b13 * (nv0 - nv18));
        let eq57_e978_d_b14: f64 = (eq57_e976_d_b14 * (nv0 - nv18));
        let eq57_e978_d_b15: f64 = (eq57_e976_d_b15 * (nv0 - nv18));
        let eq57_e978_d_b16: f64 = (eq57_e976_d_b16 * (nv0 - nv18));
        let eq57_e978_d_b17: f64 = (eq57_e976_d_b17 * (nv0 - nv18));
        let eq57_e978_d_b18: f64 = (eq57_e976_d_b18 * (nv0 - nv18));
        let eq57_e978_d_b19: f64 = (eq57_e976_d_b19 * (nv0 - nv18));
        let eq57_e978_d_b20: f64 = (eq57_e976_d_b20 * (nv0 - nv18));
        let eq57_e978_d_b21: f64 = (eq57_e976_d_b21 * (nv0 - nv18));
        let eq57_e978_d_b22: f64 = (eq57_e976_d_b22 * (nv0 - nv18));
        let eq57_e978_d_b23: f64 = (eq57_e976_d_b23 * (nv0 - nv18));
        let eq57_e978_d_b24: f64 = (eq57_e976_d_b24 * (nv0 - nv18));
        let eq57_e978_d_b25: f64 = (eq57_e976_d_b25 * (nv0 - nv18));
        let eq57_e978_d_b26: f64 = (eq57_e976_d_b26 * (nv0 - nv18));
        let eq57_e978_d_b27: f64 = (eq57_e976_d_b27 * (nv0 - nv18));
        let eq57_e978_d_b28: f64 = (eq57_e976_d_b28 * (nv0 - nv18));
        let eq57_e978_d_b29: f64 = (eq57_e976_d_b29 * (nv0 - nv18));
        let eq57_e978_d_b30: f64 = (eq57_e976_d_b30 * (nv0 - nv18));
        let eq57_e978_d_b31: f64 = (eq57_e976_d_b31 * (nv0 - nv18));
        let eq57_e978_d_b32: f64 = (eq57_e976_d_b32 * (nv0 - nv18));
        let eq57_e978_d_b33: f64 = (eq57_e976_d_b33 * (nv0 - nv18));
        let eq57_e978_d_b34: f64 = (eq57_e976_d_b34 * (nv0 - nv18));
        let eq57_e978_d_b35: f64 = (eq57_e976_d_b35 * (nv0 - nv18));
        let eq57_e978_d_b36: f64 = (eq57_e976_d_b36 * (nv0 - nv18));
        let eq57_e978_d_b37: f64 = (eq57_e976_d_b37 * (nv0 - nv18));
        let eq57_e978_d_b38: f64 = (eq57_e976_d_b38 * (nv0 - nv18));
        let eq57_e978_d_b39: f64 = (eq57_e976_d_b39 * (nv0 - nv18));
        let eq57_e978_d_b40: f64 = (eq57_e976_d_b40 * (nv0 - nv18));
        let eq57_e978_d_b41: f64 = (eq57_e976_d_b41 * (nv0 - nv18));
        let eq57_e978_d_b42: f64 = (eq57_e976_d_b42 * (nv0 - nv18));
        let eq57_e978_d_b43: f64 = (eq57_e976_d_b43 * (nv0 - nv18));
        let eq57_e978_d_b44: f64 = (eq57_e976_d_b44 * (nv0 - nv18));
        let eq57_e978_d_b45: f64 = (eq57_e976_d_b45 * (nv0 - nv18));
        let eq57_e978_d_b46: f64 = (eq57_e976_d_b46 * (nv0 - nv18));
        let eq57_e978_d_b47: f64 = (eq57_e976_d_b47 * (nv0 - nv18));
        let eq57_e978_d_b48: f64 = (eq57_e976_d_b48 * (nv0 - nv18));
        let eq57_e978_d_b49: f64 = (eq57_e976_d_b49 * (nv0 - nv18));
        let eq57_e978_d_b50: f64 = (eq57_e976_d_b50 * (nv0 - nv18));
        let eq57_e978_d_b51: f64 = (eq57_e976_d_b51 * (nv0 - nv18));
        let eq57_e978_d_b52: f64 = (eq57_e976_d_b52 * (nv0 - nv18));
        let eq57_e978_d_b53: f64 = (eq57_e976_d_b53 * (nv0 - nv18));
        let eq57_e978_d_b54: f64 = (eq57_e976_d_b54 * (nv0 - nv18));
        (eq57_e978, eq57_e978_d_n0, eq57_e978_d_n1, eq57_e978_d_n2, eq57_e978_d_n3, eq57_e978_d_n4, eq57_e978_d_n5, eq57_e978_d_n6, eq57_e978_d_n7, eq57_e978_d_n8, eq57_e978_d_n9, eq57_e978_d_n10, eq57_e978_d_n11, eq57_e978_d_n12, eq57_e978_d_n13, eq57_e978_d_n14, eq57_e978_d_n15, eq57_e978_d_n16, eq57_e978_d_n17, eq57_e978_d_n18, eq57_e978_d_n19, eq57_e978_d_n20, eq57_e978_d_n21, eq57_e978_d_n22, eq57_e978_d_b0, eq57_e978_d_b1, eq57_e978_d_b2, eq57_e978_d_b3, eq57_e978_d_b4, eq57_e978_d_b5, eq57_e978_d_b6, eq57_e978_d_b7, eq57_e978_d_b8, eq57_e978_d_b9, eq57_e978_d_b10, eq57_e978_d_b11, eq57_e978_d_b12, eq57_e978_d_b13, eq57_e978_d_b14, eq57_e978_d_b15, eq57_e978_d_b16, eq57_e978_d_b17, eq57_e978_d_b18, eq57_e978_d_b19, eq57_e978_d_b20, eq57_e978_d_b21, eq57_e978_d_b22, eq57_e978_d_b23, eq57_e978_d_b24, eq57_e978_d_b25, eq57_e978_d_b26, eq57_e978_d_b27, eq57_e978_d_b28, eq57_e978_d_b29, eq57_e978_d_b30, eq57_e978_d_b31, eq57_e978_d_b32, eq57_e978_d_b33, eq57_e978_d_b34, eq57_e978_d_b35, eq57_e978_d_b36, eq57_e978_d_b37, eq57_e978_d_b38, eq57_e978_d_b39, eq57_e978_d_b40, eq57_e978_d_b41, eq57_e978_d_b42, eq57_e978_d_b43, eq57_e978_d_b44, eq57_e978_d_b45, eq57_e978_d_b46, eq57_e978_d_b47, eq57_e978_d_b48, eq57_e978_d_b49, eq57_e978_d_b50, eq57_e978_d_b51, eq57_e978_d_b52, eq57_e978_d_b53, eq57_e978_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e980;
        let eq57_node_derivatives: [f64; 23] = [eq57_e980_d_n0, eq57_e980_d_n1, eq57_e980_d_n2, eq57_e980_d_n3, eq57_e980_d_n4, eq57_e980_d_n5, eq57_e980_d_n6, eq57_e980_d_n7, eq57_e980_d_n8, eq57_e980_d_n9, eq57_e980_d_n10, eq57_e980_d_n11, eq57_e980_d_n12, eq57_e980_d_n13, eq57_e980_d_n14, eq57_e980_d_n15, eq57_e980_d_n16, eq57_e980_d_n17, eq57_e980_d_n18, eq57_e980_d_n19, eq57_e980_d_n20, eq57_e980_d_n21, eq57_e980_d_n22];
        let eq57_branch_derivatives: [f64; 55] = [eq57_e980_d_b0, eq57_e980_d_b1, eq57_e980_d_b2, eq57_e980_d_b3, eq57_e980_d_b4, eq57_e980_d_b5, eq57_e980_d_b6, eq57_e980_d_b7, eq57_e980_d_b8, eq57_e980_d_b9, eq57_e980_d_b10, eq57_e980_d_b11, eq57_e980_d_b12, eq57_e980_d_b13, eq57_e980_d_b14, eq57_e980_d_b15, eq57_e980_d_b16, eq57_e980_d_b17, eq57_e980_d_b18, eq57_e980_d_b19, eq57_e980_d_b20, eq57_e980_d_b21, eq57_e980_d_b22, eq57_e980_d_b23, eq57_e980_d_b24, eq57_e980_d_b25, eq57_e980_d_b26, eq57_e980_d_b27, eq57_e980_d_b28, eq57_e980_d_b29, eq57_e980_d_b30, eq57_e980_d_b31, eq57_e980_d_b32, eq57_e980_d_b33, eq57_e980_d_b34, eq57_e980_d_b35, eq57_e980_d_b36, eq57_e980_d_b37, eq57_e980_d_b38, eq57_e980_d_b39, eq57_e980_d_b40, eq57_e980_d_b41, eq57_e980_d_b42, eq57_e980_d_b43, eq57_e980_d_b44, eq57_e980_d_b45, eq57_e980_d_b46, eq57_e980_d_b47, eq57_e980_d_b48, eq57_e980_d_b49, eq57_e980_d_b50, eq57_e980_d_b51, eq57_e980_d_b52, eq57_e980_d_b53, eq57_e980_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(18),
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_10(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let (eq58_e990, eq58_e990_d_n0, eq58_e990_d_n1, eq58_e990_d_n2, eq58_e990_d_n3, eq58_e990_d_n4, eq58_e990_d_n5, eq58_e990_d_n6, eq58_e990_d_n7, eq58_e990_d_n8, eq58_e990_d_n9, eq58_e990_d_n10, eq58_e990_d_n11, eq58_e990_d_n12, eq58_e990_d_n13, eq58_e990_d_n14, eq58_e990_d_n15, eq58_e990_d_n16, eq58_e990_d_n17, eq58_e990_d_n18, eq58_e990_d_n19, eq58_e990_d_n20, eq58_e990_d_n21, eq58_e990_d_n22, eq58_e990_d_b0, eq58_e990_d_b1, eq58_e990_d_b2, eq58_e990_d_b3, eq58_e990_d_b4, eq58_e990_d_b5, eq58_e990_d_b6, eq58_e990_d_b7, eq58_e990_d_b8, eq58_e990_d_b9, eq58_e990_d_b10, eq58_e990_d_b11, eq58_e990_d_b12, eq58_e990_d_b13, eq58_e990_d_b14, eq58_e990_d_b15, eq58_e990_d_b16, eq58_e990_d_b17, eq58_e990_d_b18, eq58_e990_d_b19, eq58_e990_d_b20, eq58_e990_d_b21, eq58_e990_d_b22, eq58_e990_d_b23, eq58_e990_d_b24, eq58_e990_d_b25, eq58_e990_d_b26, eq58_e990_d_b27, eq58_e990_d_b28, eq58_e990_d_b29, eq58_e990_d_b30, eq58_e990_d_b31, eq58_e990_d_b32, eq58_e990_d_b33, eq58_e990_d_b34, eq58_e990_d_b35, eq58_e990_d_b36, eq58_e990_d_b37, eq58_e990_d_b38, eq58_e990_d_b39, eq58_e990_d_b40, eq58_e990_d_b41, eq58_e990_d_b42, eq58_e990_d_b43, eq58_e990_d_b44, eq58_e990_d_b45, eq58_e990_d_b46, eq58_e990_d_b47, eq58_e990_d_b48, eq58_e990_d_b49, eq58_e990_d_b50, eq58_e990_d_b51, eq58_e990_d_b52, eq58_e990_d_b53, eq58_e990_d_b54,) = {
    if (s.b[424] && s.b[427]) {
        let eq58_e986: f64 = (p.p6 * s.v[143]);
        let eq58_e986_d_n0: f64 = (p.p6 * s.dn[143][0]);
        let eq58_e986_d_n1: f64 = (p.p6 * s.dn[143][1]);
        let eq58_e986_d_n2: f64 = (p.p6 * s.dn[143][2]);
        let eq58_e986_d_n3: f64 = (p.p6 * s.dn[143][3]);
        let eq58_e986_d_n4: f64 = (p.p6 * s.dn[143][4]);
        let eq58_e986_d_n5: f64 = (p.p6 * s.dn[143][5]);
        let eq58_e986_d_n6: f64 = (p.p6 * s.dn[143][6]);
        let eq58_e986_d_n7: f64 = (p.p6 * s.dn[143][7]);
        let eq58_e986_d_n8: f64 = (p.p6 * s.dn[143][8]);
        let eq58_e986_d_n9: f64 = (p.p6 * s.dn[143][9]);
        let eq58_e986_d_n10: f64 = (p.p6 * s.dn[143][10]);
        let eq58_e986_d_n11: f64 = (p.p6 * s.dn[143][11]);
        let eq58_e986_d_n12: f64 = (p.p6 * s.dn[143][12]);
        let eq58_e986_d_n13: f64 = (p.p6 * s.dn[143][13]);
        let eq58_e986_d_n14: f64 = (p.p6 * s.dn[143][14]);
        let eq58_e986_d_n15: f64 = (p.p6 * s.dn[143][15]);
        let eq58_e986_d_n16: f64 = (p.p6 * s.dn[143][16]);
        let eq58_e986_d_n17: f64 = (p.p6 * s.dn[143][17]);
        let eq58_e986_d_n18: f64 = (p.p6 * s.dn[143][18]);
        let eq58_e986_d_n19: f64 = (p.p6 * s.dn[143][19]);
        let eq58_e986_d_n20: f64 = (p.p6 * s.dn[143][20]);
        let eq58_e986_d_n21: f64 = (p.p6 * s.dn[143][21]);
        let eq58_e986_d_n22: f64 = (p.p6 * s.dn[143][22]);
        let eq58_e986_d_b0: f64 = (p.p6 * s.db[143][0]);
        let eq58_e986_d_b1: f64 = (p.p6 * s.db[143][1]);
        let eq58_e986_d_b2: f64 = (p.p6 * s.db[143][2]);
        let eq58_e986_d_b3: f64 = (p.p6 * s.db[143][3]);
        let eq58_e986_d_b4: f64 = (p.p6 * s.db[143][4]);
        let eq58_e986_d_b5: f64 = (p.p6 * s.db[143][5]);
        let eq58_e986_d_b6: f64 = (p.p6 * s.db[143][6]);
        let eq58_e986_d_b7: f64 = (p.p6 * s.db[143][7]);
        let eq58_e986_d_b8: f64 = (p.p6 * s.db[143][8]);
        let eq58_e986_d_b9: f64 = (p.p6 * s.db[143][9]);
        let eq58_e986_d_b10: f64 = (p.p6 * s.db[143][10]);
        let eq58_e986_d_b11: f64 = (p.p6 * s.db[143][11]);
        let eq58_e986_d_b12: f64 = (p.p6 * s.db[143][12]);
        let eq58_e986_d_b13: f64 = (p.p6 * s.db[143][13]);
        let eq58_e986_d_b14: f64 = (p.p6 * s.db[143][14]);
        let eq58_e986_d_b15: f64 = (p.p6 * s.db[143][15]);
        let eq58_e986_d_b16: f64 = (p.p6 * s.db[143][16]);
        let eq58_e986_d_b17: f64 = (p.p6 * s.db[143][17]);
        let eq58_e986_d_b18: f64 = (p.p6 * s.db[143][18]);
        let eq58_e986_d_b19: f64 = (p.p6 * s.db[143][19]);
        let eq58_e986_d_b20: f64 = (p.p6 * s.db[143][20]);
        let eq58_e986_d_b21: f64 = (p.p6 * s.db[143][21]);
        let eq58_e986_d_b22: f64 = (p.p6 * s.db[143][22]);
        let eq58_e986_d_b23: f64 = (p.p6 * s.db[143][23]);
        let eq58_e986_d_b24: f64 = (p.p6 * s.db[143][24]);
        let eq58_e986_d_b25: f64 = (p.p6 * s.db[143][25]);
        let eq58_e986_d_b26: f64 = (p.p6 * s.db[143][26]);
        let eq58_e986_d_b27: f64 = (p.p6 * s.db[143][27]);
        let eq58_e986_d_b28: f64 = (p.p6 * s.db[143][28]);
        let eq58_e986_d_b29: f64 = (p.p6 * s.db[143][29]);
        let eq58_e986_d_b30: f64 = (p.p6 * s.db[143][30]);
        let eq58_e986_d_b31: f64 = (p.p6 * s.db[143][31]);
        let eq58_e986_d_b32: f64 = (p.p6 * s.db[143][32]);
        let eq58_e986_d_b33: f64 = (p.p6 * s.db[143][33]);
        let eq58_e986_d_b34: f64 = (p.p6 * s.db[143][34]);
        let eq58_e986_d_b35: f64 = (p.p6 * s.db[143][35]);
        let eq58_e986_d_b36: f64 = (p.p6 * s.db[143][36]);
        let eq58_e986_d_b37: f64 = (p.p6 * s.db[143][37]);
        let eq58_e986_d_b38: f64 = (p.p6 * s.db[143][38]);
        let eq58_e986_d_b39: f64 = (p.p6 * s.db[143][39]);
        let eq58_e986_d_b40: f64 = (p.p6 * s.db[143][40]);
        let eq58_e986_d_b41: f64 = (p.p6 * s.db[143][41]);
        let eq58_e986_d_b42: f64 = (p.p6 * s.db[143][42]);
        let eq58_e986_d_b43: f64 = (p.p6 * s.db[143][43]);
        let eq58_e986_d_b44: f64 = (p.p6 * s.db[143][44]);
        let eq58_e986_d_b45: f64 = (p.p6 * s.db[143][45]);
        let eq58_e986_d_b46: f64 = (p.p6 * s.db[143][46]);
        let eq58_e986_d_b47: f64 = (p.p6 * s.db[143][47]);
        let eq58_e986_d_b48: f64 = (p.p6 * s.db[143][48]);
        let eq58_e986_d_b49: f64 = (p.p6 * s.db[143][49]);
        let eq58_e986_d_b50: f64 = (p.p6 * s.db[143][50]);
        let eq58_e986_d_b51: f64 = (p.p6 * s.db[143][51]);
        let eq58_e986_d_b52: f64 = (p.p6 * s.db[143][52]);
        let eq58_e986_d_b53: f64 = (p.p6 * s.db[143][53]);
        let eq58_e986_d_b54: f64 = (p.p6 * s.db[143][54]);
        let eq58_e988: f64 = (eq58_e986 * (nv22 - nv2));
        let eq58_e988_d_n0: f64 = (eq58_e986_d_n0 * (nv22 - nv2));
        let eq58_e988_d_n1: f64 = (eq58_e986_d_n1 * (nv22 - nv2));
        let eq58_e988_d_n2: f64 = ((eq58_e986_d_n2 * (nv22 - nv2)) + (-eq58_e986));
        let eq58_e988_d_n3: f64 = (eq58_e986_d_n3 * (nv22 - nv2));
        let eq58_e988_d_n4: f64 = (eq58_e986_d_n4 * (nv22 - nv2));
        let eq58_e988_d_n5: f64 = (eq58_e986_d_n5 * (nv22 - nv2));
        let eq58_e988_d_n6: f64 = (eq58_e986_d_n6 * (nv22 - nv2));
        let eq58_e988_d_n7: f64 = (eq58_e986_d_n7 * (nv22 - nv2));
        let eq58_e988_d_n8: f64 = (eq58_e986_d_n8 * (nv22 - nv2));
        let eq58_e988_d_n9: f64 = (eq58_e986_d_n9 * (nv22 - nv2));
        let eq58_e988_d_n10: f64 = (eq58_e986_d_n10 * (nv22 - nv2));
        let eq58_e988_d_n11: f64 = (eq58_e986_d_n11 * (nv22 - nv2));
        let eq58_e988_d_n12: f64 = (eq58_e986_d_n12 * (nv22 - nv2));
        let eq58_e988_d_n13: f64 = (eq58_e986_d_n13 * (nv22 - nv2));
        let eq58_e988_d_n14: f64 = (eq58_e986_d_n14 * (nv22 - nv2));
        let eq58_e988_d_n15: f64 = (eq58_e986_d_n15 * (nv22 - nv2));
        let eq58_e988_d_n16: f64 = (eq58_e986_d_n16 * (nv22 - nv2));
        let eq58_e988_d_n17: f64 = (eq58_e986_d_n17 * (nv22 - nv2));
        let eq58_e988_d_n18: f64 = (eq58_e986_d_n18 * (nv22 - nv2));
        let eq58_e988_d_n19: f64 = (eq58_e986_d_n19 * (nv22 - nv2));
        let eq58_e988_d_n20: f64 = (eq58_e986_d_n20 * (nv22 - nv2));
        let eq58_e988_d_n21: f64 = (eq58_e986_d_n21 * (nv22 - nv2));
        let eq58_e988_d_n22: f64 = ((eq58_e986_d_n22 * (nv22 - nv2)) + eq58_e986);
        let eq58_e988_d_b0: f64 = (eq58_e986_d_b0 * (nv22 - nv2));
        let eq58_e988_d_b1: f64 = (eq58_e986_d_b1 * (nv22 - nv2));
        let eq58_e988_d_b2: f64 = (eq58_e986_d_b2 * (nv22 - nv2));
        let eq58_e988_d_b3: f64 = (eq58_e986_d_b3 * (nv22 - nv2));
        let eq58_e988_d_b4: f64 = (eq58_e986_d_b4 * (nv22 - nv2));
        let eq58_e988_d_b5: f64 = (eq58_e986_d_b5 * (nv22 - nv2));
        let eq58_e988_d_b6: f64 = (eq58_e986_d_b6 * (nv22 - nv2));
        let eq58_e988_d_b7: f64 = (eq58_e986_d_b7 * (nv22 - nv2));
        let eq58_e988_d_b8: f64 = (eq58_e986_d_b8 * (nv22 - nv2));
        let eq58_e988_d_b9: f64 = (eq58_e986_d_b9 * (nv22 - nv2));
        let eq58_e988_d_b10: f64 = (eq58_e986_d_b10 * (nv22 - nv2));
        let eq58_e988_d_b11: f64 = (eq58_e986_d_b11 * (nv22 - nv2));
        let eq58_e988_d_b12: f64 = (eq58_e986_d_b12 * (nv22 - nv2));
        let eq58_e988_d_b13: f64 = (eq58_e986_d_b13 * (nv22 - nv2));
        let eq58_e988_d_b14: f64 = (eq58_e986_d_b14 * (nv22 - nv2));
        let eq58_e988_d_b15: f64 = (eq58_e986_d_b15 * (nv22 - nv2));
        let eq58_e988_d_b16: f64 = (eq58_e986_d_b16 * (nv22 - nv2));
        let eq58_e988_d_b17: f64 = (eq58_e986_d_b17 * (nv22 - nv2));
        let eq58_e988_d_b18: f64 = (eq58_e986_d_b18 * (nv22 - nv2));
        let eq58_e988_d_b19: f64 = (eq58_e986_d_b19 * (nv22 - nv2));
        let eq58_e988_d_b20: f64 = (eq58_e986_d_b20 * (nv22 - nv2));
        let eq58_e988_d_b21: f64 = (eq58_e986_d_b21 * (nv22 - nv2));
        let eq58_e988_d_b22: f64 = (eq58_e986_d_b22 * (nv22 - nv2));
        let eq58_e988_d_b23: f64 = (eq58_e986_d_b23 * (nv22 - nv2));
        let eq58_e988_d_b24: f64 = (eq58_e986_d_b24 * (nv22 - nv2));
        let eq58_e988_d_b25: f64 = (eq58_e986_d_b25 * (nv22 - nv2));
        let eq58_e988_d_b26: f64 = (eq58_e986_d_b26 * (nv22 - nv2));
        let eq58_e988_d_b27: f64 = (eq58_e986_d_b27 * (nv22 - nv2));
        let eq58_e988_d_b28: f64 = (eq58_e986_d_b28 * (nv22 - nv2));
        let eq58_e988_d_b29: f64 = (eq58_e986_d_b29 * (nv22 - nv2));
        let eq58_e988_d_b30: f64 = (eq58_e986_d_b30 * (nv22 - nv2));
        let eq58_e988_d_b31: f64 = (eq58_e986_d_b31 * (nv22 - nv2));
        let eq58_e988_d_b32: f64 = (eq58_e986_d_b32 * (nv22 - nv2));
        let eq58_e988_d_b33: f64 = (eq58_e986_d_b33 * (nv22 - nv2));
        let eq58_e988_d_b34: f64 = (eq58_e986_d_b34 * (nv22 - nv2));
        let eq58_e988_d_b35: f64 = (eq58_e986_d_b35 * (nv22 - nv2));
        let eq58_e988_d_b36: f64 = (eq58_e986_d_b36 * (nv22 - nv2));
        let eq58_e988_d_b37: f64 = (eq58_e986_d_b37 * (nv22 - nv2));
        let eq58_e988_d_b38: f64 = (eq58_e986_d_b38 * (nv22 - nv2));
        let eq58_e988_d_b39: f64 = (eq58_e986_d_b39 * (nv22 - nv2));
        let eq58_e988_d_b40: f64 = (eq58_e986_d_b40 * (nv22 - nv2));
        let eq58_e988_d_b41: f64 = (eq58_e986_d_b41 * (nv22 - nv2));
        let eq58_e988_d_b42: f64 = (eq58_e986_d_b42 * (nv22 - nv2));
        let eq58_e988_d_b43: f64 = (eq58_e986_d_b43 * (nv22 - nv2));
        let eq58_e988_d_b44: f64 = (eq58_e986_d_b44 * (nv22 - nv2));
        let eq58_e988_d_b45: f64 = (eq58_e986_d_b45 * (nv22 - nv2));
        let eq58_e988_d_b46: f64 = (eq58_e986_d_b46 * (nv22 - nv2));
        let eq58_e988_d_b47: f64 = (eq58_e986_d_b47 * (nv22 - nv2));
        let eq58_e988_d_b48: f64 = (eq58_e986_d_b48 * (nv22 - nv2));
        let eq58_e988_d_b49: f64 = (eq58_e986_d_b49 * (nv22 - nv2));
        let eq58_e988_d_b50: f64 = (eq58_e986_d_b50 * (nv22 - nv2));
        let eq58_e988_d_b51: f64 = (eq58_e986_d_b51 * (nv22 - nv2));
        let eq58_e988_d_b52: f64 = (eq58_e986_d_b52 * (nv22 - nv2));
        let eq58_e988_d_b53: f64 = (eq58_e986_d_b53 * (nv22 - nv2));
        let eq58_e988_d_b54: f64 = (eq58_e986_d_b54 * (nv22 - nv2));
        (eq58_e988, eq58_e988_d_n0, eq58_e988_d_n1, eq58_e988_d_n2, eq58_e988_d_n3, eq58_e988_d_n4, eq58_e988_d_n5, eq58_e988_d_n6, eq58_e988_d_n7, eq58_e988_d_n8, eq58_e988_d_n9, eq58_e988_d_n10, eq58_e988_d_n11, eq58_e988_d_n12, eq58_e988_d_n13, eq58_e988_d_n14, eq58_e988_d_n15, eq58_e988_d_n16, eq58_e988_d_n17, eq58_e988_d_n18, eq58_e988_d_n19, eq58_e988_d_n20, eq58_e988_d_n21, eq58_e988_d_n22, eq58_e988_d_b0, eq58_e988_d_b1, eq58_e988_d_b2, eq58_e988_d_b3, eq58_e988_d_b4, eq58_e988_d_b5, eq58_e988_d_b6, eq58_e988_d_b7, eq58_e988_d_b8, eq58_e988_d_b9, eq58_e988_d_b10, eq58_e988_d_b11, eq58_e988_d_b12, eq58_e988_d_b13, eq58_e988_d_b14, eq58_e988_d_b15, eq58_e988_d_b16, eq58_e988_d_b17, eq58_e988_d_b18, eq58_e988_d_b19, eq58_e988_d_b20, eq58_e988_d_b21, eq58_e988_d_b22, eq58_e988_d_b23, eq58_e988_d_b24, eq58_e988_d_b25, eq58_e988_d_b26, eq58_e988_d_b27, eq58_e988_d_b28, eq58_e988_d_b29, eq58_e988_d_b30, eq58_e988_d_b31, eq58_e988_d_b32, eq58_e988_d_b33, eq58_e988_d_b34, eq58_e988_d_b35, eq58_e988_d_b36, eq58_e988_d_b37, eq58_e988_d_b38, eq58_e988_d_b39, eq58_e988_d_b40, eq58_e988_d_b41, eq58_e988_d_b42, eq58_e988_d_b43, eq58_e988_d_b44, eq58_e988_d_b45, eq58_e988_d_b46, eq58_e988_d_b47, eq58_e988_d_b48, eq58_e988_d_b49, eq58_e988_d_b50, eq58_e988_d_b51, eq58_e988_d_b52, eq58_e988_d_b53, eq58_e988_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e990;
        let eq58_node_derivatives: [f64; 23] = [eq58_e990_d_n0, eq58_e990_d_n1, eq58_e990_d_n2, eq58_e990_d_n3, eq58_e990_d_n4, eq58_e990_d_n5, eq58_e990_d_n6, eq58_e990_d_n7, eq58_e990_d_n8, eq58_e990_d_n9, eq58_e990_d_n10, eq58_e990_d_n11, eq58_e990_d_n12, eq58_e990_d_n13, eq58_e990_d_n14, eq58_e990_d_n15, eq58_e990_d_n16, eq58_e990_d_n17, eq58_e990_d_n18, eq58_e990_d_n19, eq58_e990_d_n20, eq58_e990_d_n21, eq58_e990_d_n22];
        let eq58_branch_derivatives: [f64; 55] = [eq58_e990_d_b0, eq58_e990_d_b1, eq58_e990_d_b2, eq58_e990_d_b3, eq58_e990_d_b4, eq58_e990_d_b5, eq58_e990_d_b6, eq58_e990_d_b7, eq58_e990_d_b8, eq58_e990_d_b9, eq58_e990_d_b10, eq58_e990_d_b11, eq58_e990_d_b12, eq58_e990_d_b13, eq58_e990_d_b14, eq58_e990_d_b15, eq58_e990_d_b16, eq58_e990_d_b17, eq58_e990_d_b18, eq58_e990_d_b19, eq58_e990_d_b20, eq58_e990_d_b21, eq58_e990_d_b22, eq58_e990_d_b23, eq58_e990_d_b24, eq58_e990_d_b25, eq58_e990_d_b26, eq58_e990_d_b27, eq58_e990_d_b28, eq58_e990_d_b29, eq58_e990_d_b30, eq58_e990_d_b31, eq58_e990_d_b32, eq58_e990_d_b33, eq58_e990_d_b34, eq58_e990_d_b35, eq58_e990_d_b36, eq58_e990_d_b37, eq58_e990_d_b38, eq58_e990_d_b39, eq58_e990_d_b40, eq58_e990_d_b41, eq58_e990_d_b42, eq58_e990_d_b43, eq58_e990_d_b44, eq58_e990_d_b45, eq58_e990_d_b46, eq58_e990_d_b47, eq58_e990_d_b48, eq58_e990_d_b49, eq58_e990_d_b50, eq58_e990_d_b51, eq58_e990_d_b52, eq58_e990_d_b53, eq58_e990_d_b54];
        stamper.stamp_current_dense_local(
            Some(22),
            Some(2),
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e1001, eq59_e1001_d_n0, eq59_e1001_d_n1, eq59_e1001_d_n2, eq59_e1001_d_n3, eq59_e1001_d_n4, eq59_e1001_d_n5, eq59_e1001_d_n6, eq59_e1001_d_n7, eq59_e1001_d_n8, eq59_e1001_d_n9, eq59_e1001_d_n10, eq59_e1001_d_n11, eq59_e1001_d_n12, eq59_e1001_d_n13, eq59_e1001_d_n14, eq59_e1001_d_n15, eq59_e1001_d_n16, eq59_e1001_d_n17, eq59_e1001_d_n18, eq59_e1001_d_n19, eq59_e1001_d_n20, eq59_e1001_d_n21, eq59_e1001_d_n22, eq59_e1001_d_b0, eq59_e1001_d_b1, eq59_e1001_d_b2, eq59_e1001_d_b3, eq59_e1001_d_b4, eq59_e1001_d_b5, eq59_e1001_d_b6, eq59_e1001_d_b7, eq59_e1001_d_b8, eq59_e1001_d_b9, eq59_e1001_d_b10, eq59_e1001_d_b11, eq59_e1001_d_b12, eq59_e1001_d_b13, eq59_e1001_d_b14, eq59_e1001_d_b15, eq59_e1001_d_b16, eq59_e1001_d_b17, eq59_e1001_d_b18, eq59_e1001_d_b19, eq59_e1001_d_b20, eq59_e1001_d_b21, eq59_e1001_d_b22, eq59_e1001_d_b23, eq59_e1001_d_b24, eq59_e1001_d_b25, eq59_e1001_d_b26, eq59_e1001_d_b27, eq59_e1001_d_b28, eq59_e1001_d_b29, eq59_e1001_d_b30, eq59_e1001_d_b31, eq59_e1001_d_b32, eq59_e1001_d_b33, eq59_e1001_d_b34, eq59_e1001_d_b35, eq59_e1001_d_b36, eq59_e1001_d_b37, eq59_e1001_d_b38, eq59_e1001_d_b39, eq59_e1001_d_b40, eq59_e1001_d_b41, eq59_e1001_d_b42, eq59_e1001_d_b43, eq59_e1001_d_b44, eq59_e1001_d_b45, eq59_e1001_d_b46, eq59_e1001_d_b47, eq59_e1001_d_b48, eq59_e1001_d_b49, eq59_e1001_d_b50, eq59_e1001_d_b51, eq59_e1001_d_b52, eq59_e1001_d_b53, eq59_e1001_d_b54,) = {
    if (s.b[424] && (!s.b[427])) {
        let eq59_e997: f64 = (p.p6 * s.v[142]);
        let eq59_e997_d_n0: f64 = (p.p6 * s.dn[142][0]);
        let eq59_e997_d_n1: f64 = (p.p6 * s.dn[142][1]);
        let eq59_e997_d_n2: f64 = (p.p6 * s.dn[142][2]);
        let eq59_e997_d_n3: f64 = (p.p6 * s.dn[142][3]);
        let eq59_e997_d_n4: f64 = (p.p6 * s.dn[142][4]);
        let eq59_e997_d_n5: f64 = (p.p6 * s.dn[142][5]);
        let eq59_e997_d_n6: f64 = (p.p6 * s.dn[142][6]);
        let eq59_e997_d_n7: f64 = (p.p6 * s.dn[142][7]);
        let eq59_e997_d_n8: f64 = (p.p6 * s.dn[142][8]);
        let eq59_e997_d_n9: f64 = (p.p6 * s.dn[142][9]);
        let eq59_e997_d_n10: f64 = (p.p6 * s.dn[142][10]);
        let eq59_e997_d_n11: f64 = (p.p6 * s.dn[142][11]);
        let eq59_e997_d_n12: f64 = (p.p6 * s.dn[142][12]);
        let eq59_e997_d_n13: f64 = (p.p6 * s.dn[142][13]);
        let eq59_e997_d_n14: f64 = (p.p6 * s.dn[142][14]);
        let eq59_e997_d_n15: f64 = (p.p6 * s.dn[142][15]);
        let eq59_e997_d_n16: f64 = (p.p6 * s.dn[142][16]);
        let eq59_e997_d_n17: f64 = (p.p6 * s.dn[142][17]);
        let eq59_e997_d_n18: f64 = (p.p6 * s.dn[142][18]);
        let eq59_e997_d_n19: f64 = (p.p6 * s.dn[142][19]);
        let eq59_e997_d_n20: f64 = (p.p6 * s.dn[142][20]);
        let eq59_e997_d_n21: f64 = (p.p6 * s.dn[142][21]);
        let eq59_e997_d_n22: f64 = (p.p6 * s.dn[142][22]);
        let eq59_e997_d_b0: f64 = (p.p6 * s.db[142][0]);
        let eq59_e997_d_b1: f64 = (p.p6 * s.db[142][1]);
        let eq59_e997_d_b2: f64 = (p.p6 * s.db[142][2]);
        let eq59_e997_d_b3: f64 = (p.p6 * s.db[142][3]);
        let eq59_e997_d_b4: f64 = (p.p6 * s.db[142][4]);
        let eq59_e997_d_b5: f64 = (p.p6 * s.db[142][5]);
        let eq59_e997_d_b6: f64 = (p.p6 * s.db[142][6]);
        let eq59_e997_d_b7: f64 = (p.p6 * s.db[142][7]);
        let eq59_e997_d_b8: f64 = (p.p6 * s.db[142][8]);
        let eq59_e997_d_b9: f64 = (p.p6 * s.db[142][9]);
        let eq59_e997_d_b10: f64 = (p.p6 * s.db[142][10]);
        let eq59_e997_d_b11: f64 = (p.p6 * s.db[142][11]);
        let eq59_e997_d_b12: f64 = (p.p6 * s.db[142][12]);
        let eq59_e997_d_b13: f64 = (p.p6 * s.db[142][13]);
        let eq59_e997_d_b14: f64 = (p.p6 * s.db[142][14]);
        let eq59_e997_d_b15: f64 = (p.p6 * s.db[142][15]);
        let eq59_e997_d_b16: f64 = (p.p6 * s.db[142][16]);
        let eq59_e997_d_b17: f64 = (p.p6 * s.db[142][17]);
        let eq59_e997_d_b18: f64 = (p.p6 * s.db[142][18]);
        let eq59_e997_d_b19: f64 = (p.p6 * s.db[142][19]);
        let eq59_e997_d_b20: f64 = (p.p6 * s.db[142][20]);
        let eq59_e997_d_b21: f64 = (p.p6 * s.db[142][21]);
        let eq59_e997_d_b22: f64 = (p.p6 * s.db[142][22]);
        let eq59_e997_d_b23: f64 = (p.p6 * s.db[142][23]);
        let eq59_e997_d_b24: f64 = (p.p6 * s.db[142][24]);
        let eq59_e997_d_b25: f64 = (p.p6 * s.db[142][25]);
        let eq59_e997_d_b26: f64 = (p.p6 * s.db[142][26]);
        let eq59_e997_d_b27: f64 = (p.p6 * s.db[142][27]);
        let eq59_e997_d_b28: f64 = (p.p6 * s.db[142][28]);
        let eq59_e997_d_b29: f64 = (p.p6 * s.db[142][29]);
        let eq59_e997_d_b30: f64 = (p.p6 * s.db[142][30]);
        let eq59_e997_d_b31: f64 = (p.p6 * s.db[142][31]);
        let eq59_e997_d_b32: f64 = (p.p6 * s.db[142][32]);
        let eq59_e997_d_b33: f64 = (p.p6 * s.db[142][33]);
        let eq59_e997_d_b34: f64 = (p.p6 * s.db[142][34]);
        let eq59_e997_d_b35: f64 = (p.p6 * s.db[142][35]);
        let eq59_e997_d_b36: f64 = (p.p6 * s.db[142][36]);
        let eq59_e997_d_b37: f64 = (p.p6 * s.db[142][37]);
        let eq59_e997_d_b38: f64 = (p.p6 * s.db[142][38]);
        let eq59_e997_d_b39: f64 = (p.p6 * s.db[142][39]);
        let eq59_e997_d_b40: f64 = (p.p6 * s.db[142][40]);
        let eq59_e997_d_b41: f64 = (p.p6 * s.db[142][41]);
        let eq59_e997_d_b42: f64 = (p.p6 * s.db[142][42]);
        let eq59_e997_d_b43: f64 = (p.p6 * s.db[142][43]);
        let eq59_e997_d_b44: f64 = (p.p6 * s.db[142][44]);
        let eq59_e997_d_b45: f64 = (p.p6 * s.db[142][45]);
        let eq59_e997_d_b46: f64 = (p.p6 * s.db[142][46]);
        let eq59_e997_d_b47: f64 = (p.p6 * s.db[142][47]);
        let eq59_e997_d_b48: f64 = (p.p6 * s.db[142][48]);
        let eq59_e997_d_b49: f64 = (p.p6 * s.db[142][49]);
        let eq59_e997_d_b50: f64 = (p.p6 * s.db[142][50]);
        let eq59_e997_d_b51: f64 = (p.p6 * s.db[142][51]);
        let eq59_e997_d_b52: f64 = (p.p6 * s.db[142][52]);
        let eq59_e997_d_b53: f64 = (p.p6 * s.db[142][53]);
        let eq59_e997_d_b54: f64 = (p.p6 * s.db[142][54]);
        let eq59_e999: f64 = (eq59_e997 * (nv0 - nv7));
        let eq59_e999_d_n0: f64 = ((eq59_e997_d_n0 * (nv0 - nv7)) + eq59_e997);
        let eq59_e999_d_n1: f64 = (eq59_e997_d_n1 * (nv0 - nv7));
        let eq59_e999_d_n2: f64 = (eq59_e997_d_n2 * (nv0 - nv7));
        let eq59_e999_d_n3: f64 = (eq59_e997_d_n3 * (nv0 - nv7));
        let eq59_e999_d_n4: f64 = (eq59_e997_d_n4 * (nv0 - nv7));
        let eq59_e999_d_n5: f64 = (eq59_e997_d_n5 * (nv0 - nv7));
        let eq59_e999_d_n6: f64 = (eq59_e997_d_n6 * (nv0 - nv7));
        let eq59_e999_d_n7: f64 = ((eq59_e997_d_n7 * (nv0 - nv7)) + (-eq59_e997));
        let eq59_e999_d_n8: f64 = (eq59_e997_d_n8 * (nv0 - nv7));
        let eq59_e999_d_n9: f64 = (eq59_e997_d_n9 * (nv0 - nv7));
        let eq59_e999_d_n10: f64 = (eq59_e997_d_n10 * (nv0 - nv7));
        let eq59_e999_d_n11: f64 = (eq59_e997_d_n11 * (nv0 - nv7));
        let eq59_e999_d_n12: f64 = (eq59_e997_d_n12 * (nv0 - nv7));
        let eq59_e999_d_n13: f64 = (eq59_e997_d_n13 * (nv0 - nv7));
        let eq59_e999_d_n14: f64 = (eq59_e997_d_n14 * (nv0 - nv7));
        let eq59_e999_d_n15: f64 = (eq59_e997_d_n15 * (nv0 - nv7));
        let eq59_e999_d_n16: f64 = (eq59_e997_d_n16 * (nv0 - nv7));
        let eq59_e999_d_n17: f64 = (eq59_e997_d_n17 * (nv0 - nv7));
        let eq59_e999_d_n18: f64 = (eq59_e997_d_n18 * (nv0 - nv7));
        let eq59_e999_d_n19: f64 = (eq59_e997_d_n19 * (nv0 - nv7));
        let eq59_e999_d_n20: f64 = (eq59_e997_d_n20 * (nv0 - nv7));
        let eq59_e999_d_n21: f64 = (eq59_e997_d_n21 * (nv0 - nv7));
        let eq59_e999_d_n22: f64 = (eq59_e997_d_n22 * (nv0 - nv7));
        let eq59_e999_d_b0: f64 = (eq59_e997_d_b0 * (nv0 - nv7));
        let eq59_e999_d_b1: f64 = (eq59_e997_d_b1 * (nv0 - nv7));
        let eq59_e999_d_b2: f64 = (eq59_e997_d_b2 * (nv0 - nv7));
        let eq59_e999_d_b3: f64 = (eq59_e997_d_b3 * (nv0 - nv7));
        let eq59_e999_d_b4: f64 = (eq59_e997_d_b4 * (nv0 - nv7));
        let eq59_e999_d_b5: f64 = (eq59_e997_d_b5 * (nv0 - nv7));
        let eq59_e999_d_b6: f64 = (eq59_e997_d_b6 * (nv0 - nv7));
        let eq59_e999_d_b7: f64 = (eq59_e997_d_b7 * (nv0 - nv7));
        let eq59_e999_d_b8: f64 = (eq59_e997_d_b8 * (nv0 - nv7));
        let eq59_e999_d_b9: f64 = (eq59_e997_d_b9 * (nv0 - nv7));
        let eq59_e999_d_b10: f64 = (eq59_e997_d_b10 * (nv0 - nv7));
        let eq59_e999_d_b11: f64 = (eq59_e997_d_b11 * (nv0 - nv7));
        let eq59_e999_d_b12: f64 = (eq59_e997_d_b12 * (nv0 - nv7));
        let eq59_e999_d_b13: f64 = (eq59_e997_d_b13 * (nv0 - nv7));
        let eq59_e999_d_b14: f64 = (eq59_e997_d_b14 * (nv0 - nv7));
        let eq59_e999_d_b15: f64 = (eq59_e997_d_b15 * (nv0 - nv7));
        let eq59_e999_d_b16: f64 = (eq59_e997_d_b16 * (nv0 - nv7));
        let eq59_e999_d_b17: f64 = (eq59_e997_d_b17 * (nv0 - nv7));
        let eq59_e999_d_b18: f64 = (eq59_e997_d_b18 * (nv0 - nv7));
        let eq59_e999_d_b19: f64 = (eq59_e997_d_b19 * (nv0 - nv7));
        let eq59_e999_d_b20: f64 = (eq59_e997_d_b20 * (nv0 - nv7));
        let eq59_e999_d_b21: f64 = (eq59_e997_d_b21 * (nv0 - nv7));
        let eq59_e999_d_b22: f64 = (eq59_e997_d_b22 * (nv0 - nv7));
        let eq59_e999_d_b23: f64 = (eq59_e997_d_b23 * (nv0 - nv7));
        let eq59_e999_d_b24: f64 = (eq59_e997_d_b24 * (nv0 - nv7));
        let eq59_e999_d_b25: f64 = (eq59_e997_d_b25 * (nv0 - nv7));
        let eq59_e999_d_b26: f64 = (eq59_e997_d_b26 * (nv0 - nv7));
        let eq59_e999_d_b27: f64 = (eq59_e997_d_b27 * (nv0 - nv7));
        let eq59_e999_d_b28: f64 = (eq59_e997_d_b28 * (nv0 - nv7));
        let eq59_e999_d_b29: f64 = (eq59_e997_d_b29 * (nv0 - nv7));
        let eq59_e999_d_b30: f64 = (eq59_e997_d_b30 * (nv0 - nv7));
        let eq59_e999_d_b31: f64 = (eq59_e997_d_b31 * (nv0 - nv7));
        let eq59_e999_d_b32: f64 = (eq59_e997_d_b32 * (nv0 - nv7));
        let eq59_e999_d_b33: f64 = (eq59_e997_d_b33 * (nv0 - nv7));
        let eq59_e999_d_b34: f64 = (eq59_e997_d_b34 * (nv0 - nv7));
        let eq59_e999_d_b35: f64 = (eq59_e997_d_b35 * (nv0 - nv7));
        let eq59_e999_d_b36: f64 = (eq59_e997_d_b36 * (nv0 - nv7));
        let eq59_e999_d_b37: f64 = (eq59_e997_d_b37 * (nv0 - nv7));
        let eq59_e999_d_b38: f64 = (eq59_e997_d_b38 * (nv0 - nv7));
        let eq59_e999_d_b39: f64 = (eq59_e997_d_b39 * (nv0 - nv7));
        let eq59_e999_d_b40: f64 = (eq59_e997_d_b40 * (nv0 - nv7));
        let eq59_e999_d_b41: f64 = (eq59_e997_d_b41 * (nv0 - nv7));
        let eq59_e999_d_b42: f64 = (eq59_e997_d_b42 * (nv0 - nv7));
        let eq59_e999_d_b43: f64 = (eq59_e997_d_b43 * (nv0 - nv7));
        let eq59_e999_d_b44: f64 = (eq59_e997_d_b44 * (nv0 - nv7));
        let eq59_e999_d_b45: f64 = (eq59_e997_d_b45 * (nv0 - nv7));
        let eq59_e999_d_b46: f64 = (eq59_e997_d_b46 * (nv0 - nv7));
        let eq59_e999_d_b47: f64 = (eq59_e997_d_b47 * (nv0 - nv7));
        let eq59_e999_d_b48: f64 = (eq59_e997_d_b48 * (nv0 - nv7));
        let eq59_e999_d_b49: f64 = (eq59_e997_d_b49 * (nv0 - nv7));
        let eq59_e999_d_b50: f64 = (eq59_e997_d_b50 * (nv0 - nv7));
        let eq59_e999_d_b51: f64 = (eq59_e997_d_b51 * (nv0 - nv7));
        let eq59_e999_d_b52: f64 = (eq59_e997_d_b52 * (nv0 - nv7));
        let eq59_e999_d_b53: f64 = (eq59_e997_d_b53 * (nv0 - nv7));
        let eq59_e999_d_b54: f64 = (eq59_e997_d_b54 * (nv0 - nv7));
        (eq59_e999, eq59_e999_d_n0, eq59_e999_d_n1, eq59_e999_d_n2, eq59_e999_d_n3, eq59_e999_d_n4, eq59_e999_d_n5, eq59_e999_d_n6, eq59_e999_d_n7, eq59_e999_d_n8, eq59_e999_d_n9, eq59_e999_d_n10, eq59_e999_d_n11, eq59_e999_d_n12, eq59_e999_d_n13, eq59_e999_d_n14, eq59_e999_d_n15, eq59_e999_d_n16, eq59_e999_d_n17, eq59_e999_d_n18, eq59_e999_d_n19, eq59_e999_d_n20, eq59_e999_d_n21, eq59_e999_d_n22, eq59_e999_d_b0, eq59_e999_d_b1, eq59_e999_d_b2, eq59_e999_d_b3, eq59_e999_d_b4, eq59_e999_d_b5, eq59_e999_d_b6, eq59_e999_d_b7, eq59_e999_d_b8, eq59_e999_d_b9, eq59_e999_d_b10, eq59_e999_d_b11, eq59_e999_d_b12, eq59_e999_d_b13, eq59_e999_d_b14, eq59_e999_d_b15, eq59_e999_d_b16, eq59_e999_d_b17, eq59_e999_d_b18, eq59_e999_d_b19, eq59_e999_d_b20, eq59_e999_d_b21, eq59_e999_d_b22, eq59_e999_d_b23, eq59_e999_d_b24, eq59_e999_d_b25, eq59_e999_d_b26, eq59_e999_d_b27, eq59_e999_d_b28, eq59_e999_d_b29, eq59_e999_d_b30, eq59_e999_d_b31, eq59_e999_d_b32, eq59_e999_d_b33, eq59_e999_d_b34, eq59_e999_d_b35, eq59_e999_d_b36, eq59_e999_d_b37, eq59_e999_d_b38, eq59_e999_d_b39, eq59_e999_d_b40, eq59_e999_d_b41, eq59_e999_d_b42, eq59_e999_d_b43, eq59_e999_d_b44, eq59_e999_d_b45, eq59_e999_d_b46, eq59_e999_d_b47, eq59_e999_d_b48, eq59_e999_d_b49, eq59_e999_d_b50, eq59_e999_d_b51, eq59_e999_d_b52, eq59_e999_d_b53, eq59_e999_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1001;
        let eq59_node_derivatives: [f64; 23] = [eq59_e1001_d_n0, eq59_e1001_d_n1, eq59_e1001_d_n2, eq59_e1001_d_n3, eq59_e1001_d_n4, eq59_e1001_d_n5, eq59_e1001_d_n6, eq59_e1001_d_n7, eq59_e1001_d_n8, eq59_e1001_d_n9, eq59_e1001_d_n10, eq59_e1001_d_n11, eq59_e1001_d_n12, eq59_e1001_d_n13, eq59_e1001_d_n14, eq59_e1001_d_n15, eq59_e1001_d_n16, eq59_e1001_d_n17, eq59_e1001_d_n18, eq59_e1001_d_n19, eq59_e1001_d_n20, eq59_e1001_d_n21, eq59_e1001_d_n22];
        let eq59_branch_derivatives: [f64; 55] = [eq59_e1001_d_b0, eq59_e1001_d_b1, eq59_e1001_d_b2, eq59_e1001_d_b3, eq59_e1001_d_b4, eq59_e1001_d_b5, eq59_e1001_d_b6, eq59_e1001_d_b7, eq59_e1001_d_b8, eq59_e1001_d_b9, eq59_e1001_d_b10, eq59_e1001_d_b11, eq59_e1001_d_b12, eq59_e1001_d_b13, eq59_e1001_d_b14, eq59_e1001_d_b15, eq59_e1001_d_b16, eq59_e1001_d_b17, eq59_e1001_d_b18, eq59_e1001_d_b19, eq59_e1001_d_b20, eq59_e1001_d_b21, eq59_e1001_d_b22, eq59_e1001_d_b23, eq59_e1001_d_b24, eq59_e1001_d_b25, eq59_e1001_d_b26, eq59_e1001_d_b27, eq59_e1001_d_b28, eq59_e1001_d_b29, eq59_e1001_d_b30, eq59_e1001_d_b31, eq59_e1001_d_b32, eq59_e1001_d_b33, eq59_e1001_d_b34, eq59_e1001_d_b35, eq59_e1001_d_b36, eq59_e1001_d_b37, eq59_e1001_d_b38, eq59_e1001_d_b39, eq59_e1001_d_b40, eq59_e1001_d_b41, eq59_e1001_d_b42, eq59_e1001_d_b43, eq59_e1001_d_b44, eq59_e1001_d_b45, eq59_e1001_d_b46, eq59_e1001_d_b47, eq59_e1001_d_b48, eq59_e1001_d_b49, eq59_e1001_d_b50, eq59_e1001_d_b51, eq59_e1001_d_b52, eq59_e1001_d_b53, eq59_e1001_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_11(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq60_e1012, eq60_e1012_d_n0, eq60_e1012_d_n1, eq60_e1012_d_n2, eq60_e1012_d_n3, eq60_e1012_d_n4, eq60_e1012_d_n5, eq60_e1012_d_n6, eq60_e1012_d_n7, eq60_e1012_d_n8, eq60_e1012_d_n9, eq60_e1012_d_n10, eq60_e1012_d_n11, eq60_e1012_d_n12, eq60_e1012_d_n13, eq60_e1012_d_n14, eq60_e1012_d_n15, eq60_e1012_d_n16, eq60_e1012_d_n17, eq60_e1012_d_n18, eq60_e1012_d_n19, eq60_e1012_d_n20, eq60_e1012_d_n21, eq60_e1012_d_n22, eq60_e1012_d_b0, eq60_e1012_d_b1, eq60_e1012_d_b2, eq60_e1012_d_b3, eq60_e1012_d_b4, eq60_e1012_d_b5, eq60_e1012_d_b6, eq60_e1012_d_b7, eq60_e1012_d_b8, eq60_e1012_d_b9, eq60_e1012_d_b10, eq60_e1012_d_b11, eq60_e1012_d_b12, eq60_e1012_d_b13, eq60_e1012_d_b14, eq60_e1012_d_b15, eq60_e1012_d_b16, eq60_e1012_d_b17, eq60_e1012_d_b18, eq60_e1012_d_b19, eq60_e1012_d_b20, eq60_e1012_d_b21, eq60_e1012_d_b22, eq60_e1012_d_b23, eq60_e1012_d_b24, eq60_e1012_d_b25, eq60_e1012_d_b26, eq60_e1012_d_b27, eq60_e1012_d_b28, eq60_e1012_d_b29, eq60_e1012_d_b30, eq60_e1012_d_b31, eq60_e1012_d_b32, eq60_e1012_d_b33, eq60_e1012_d_b34, eq60_e1012_d_b35, eq60_e1012_d_b36, eq60_e1012_d_b37, eq60_e1012_d_b38, eq60_e1012_d_b39, eq60_e1012_d_b40, eq60_e1012_d_b41, eq60_e1012_d_b42, eq60_e1012_d_b43, eq60_e1012_d_b44, eq60_e1012_d_b45, eq60_e1012_d_b46, eq60_e1012_d_b47, eq60_e1012_d_b48, eq60_e1012_d_b49, eq60_e1012_d_b50, eq60_e1012_d_b51, eq60_e1012_d_b52, eq60_e1012_d_b53, eq60_e1012_d_b54,) = {
    if (s.b[424] && (!s.b[427])) {
        let eq60_e1008: f64 = (p.p6 * s.v[143]);
        let eq60_e1008_d_n0: f64 = (p.p6 * s.dn[143][0]);
        let eq60_e1008_d_n1: f64 = (p.p6 * s.dn[143][1]);
        let eq60_e1008_d_n2: f64 = (p.p6 * s.dn[143][2]);
        let eq60_e1008_d_n3: f64 = (p.p6 * s.dn[143][3]);
        let eq60_e1008_d_n4: f64 = (p.p6 * s.dn[143][4]);
        let eq60_e1008_d_n5: f64 = (p.p6 * s.dn[143][5]);
        let eq60_e1008_d_n6: f64 = (p.p6 * s.dn[143][6]);
        let eq60_e1008_d_n7: f64 = (p.p6 * s.dn[143][7]);
        let eq60_e1008_d_n8: f64 = (p.p6 * s.dn[143][8]);
        let eq60_e1008_d_n9: f64 = (p.p6 * s.dn[143][9]);
        let eq60_e1008_d_n10: f64 = (p.p6 * s.dn[143][10]);
        let eq60_e1008_d_n11: f64 = (p.p6 * s.dn[143][11]);
        let eq60_e1008_d_n12: f64 = (p.p6 * s.dn[143][12]);
        let eq60_e1008_d_n13: f64 = (p.p6 * s.dn[143][13]);
        let eq60_e1008_d_n14: f64 = (p.p6 * s.dn[143][14]);
        let eq60_e1008_d_n15: f64 = (p.p6 * s.dn[143][15]);
        let eq60_e1008_d_n16: f64 = (p.p6 * s.dn[143][16]);
        let eq60_e1008_d_n17: f64 = (p.p6 * s.dn[143][17]);
        let eq60_e1008_d_n18: f64 = (p.p6 * s.dn[143][18]);
        let eq60_e1008_d_n19: f64 = (p.p6 * s.dn[143][19]);
        let eq60_e1008_d_n20: f64 = (p.p6 * s.dn[143][20]);
        let eq60_e1008_d_n21: f64 = (p.p6 * s.dn[143][21]);
        let eq60_e1008_d_n22: f64 = (p.p6 * s.dn[143][22]);
        let eq60_e1008_d_b0: f64 = (p.p6 * s.db[143][0]);
        let eq60_e1008_d_b1: f64 = (p.p6 * s.db[143][1]);
        let eq60_e1008_d_b2: f64 = (p.p6 * s.db[143][2]);
        let eq60_e1008_d_b3: f64 = (p.p6 * s.db[143][3]);
        let eq60_e1008_d_b4: f64 = (p.p6 * s.db[143][4]);
        let eq60_e1008_d_b5: f64 = (p.p6 * s.db[143][5]);
        let eq60_e1008_d_b6: f64 = (p.p6 * s.db[143][6]);
        let eq60_e1008_d_b7: f64 = (p.p6 * s.db[143][7]);
        let eq60_e1008_d_b8: f64 = (p.p6 * s.db[143][8]);
        let eq60_e1008_d_b9: f64 = (p.p6 * s.db[143][9]);
        let eq60_e1008_d_b10: f64 = (p.p6 * s.db[143][10]);
        let eq60_e1008_d_b11: f64 = (p.p6 * s.db[143][11]);
        let eq60_e1008_d_b12: f64 = (p.p6 * s.db[143][12]);
        let eq60_e1008_d_b13: f64 = (p.p6 * s.db[143][13]);
        let eq60_e1008_d_b14: f64 = (p.p6 * s.db[143][14]);
        let eq60_e1008_d_b15: f64 = (p.p6 * s.db[143][15]);
        let eq60_e1008_d_b16: f64 = (p.p6 * s.db[143][16]);
        let eq60_e1008_d_b17: f64 = (p.p6 * s.db[143][17]);
        let eq60_e1008_d_b18: f64 = (p.p6 * s.db[143][18]);
        let eq60_e1008_d_b19: f64 = (p.p6 * s.db[143][19]);
        let eq60_e1008_d_b20: f64 = (p.p6 * s.db[143][20]);
        let eq60_e1008_d_b21: f64 = (p.p6 * s.db[143][21]);
        let eq60_e1008_d_b22: f64 = (p.p6 * s.db[143][22]);
        let eq60_e1008_d_b23: f64 = (p.p6 * s.db[143][23]);
        let eq60_e1008_d_b24: f64 = (p.p6 * s.db[143][24]);
        let eq60_e1008_d_b25: f64 = (p.p6 * s.db[143][25]);
        let eq60_e1008_d_b26: f64 = (p.p6 * s.db[143][26]);
        let eq60_e1008_d_b27: f64 = (p.p6 * s.db[143][27]);
        let eq60_e1008_d_b28: f64 = (p.p6 * s.db[143][28]);
        let eq60_e1008_d_b29: f64 = (p.p6 * s.db[143][29]);
        let eq60_e1008_d_b30: f64 = (p.p6 * s.db[143][30]);
        let eq60_e1008_d_b31: f64 = (p.p6 * s.db[143][31]);
        let eq60_e1008_d_b32: f64 = (p.p6 * s.db[143][32]);
        let eq60_e1008_d_b33: f64 = (p.p6 * s.db[143][33]);
        let eq60_e1008_d_b34: f64 = (p.p6 * s.db[143][34]);
        let eq60_e1008_d_b35: f64 = (p.p6 * s.db[143][35]);
        let eq60_e1008_d_b36: f64 = (p.p6 * s.db[143][36]);
        let eq60_e1008_d_b37: f64 = (p.p6 * s.db[143][37]);
        let eq60_e1008_d_b38: f64 = (p.p6 * s.db[143][38]);
        let eq60_e1008_d_b39: f64 = (p.p6 * s.db[143][39]);
        let eq60_e1008_d_b40: f64 = (p.p6 * s.db[143][40]);
        let eq60_e1008_d_b41: f64 = (p.p6 * s.db[143][41]);
        let eq60_e1008_d_b42: f64 = (p.p6 * s.db[143][42]);
        let eq60_e1008_d_b43: f64 = (p.p6 * s.db[143][43]);
        let eq60_e1008_d_b44: f64 = (p.p6 * s.db[143][44]);
        let eq60_e1008_d_b45: f64 = (p.p6 * s.db[143][45]);
        let eq60_e1008_d_b46: f64 = (p.p6 * s.db[143][46]);
        let eq60_e1008_d_b47: f64 = (p.p6 * s.db[143][47]);
        let eq60_e1008_d_b48: f64 = (p.p6 * s.db[143][48]);
        let eq60_e1008_d_b49: f64 = (p.p6 * s.db[143][49]);
        let eq60_e1008_d_b50: f64 = (p.p6 * s.db[143][50]);
        let eq60_e1008_d_b51: f64 = (p.p6 * s.db[143][51]);
        let eq60_e1008_d_b52: f64 = (p.p6 * s.db[143][52]);
        let eq60_e1008_d_b53: f64 = (p.p6 * s.db[143][53]);
        let eq60_e1008_d_b54: f64 = (p.p6 * s.db[143][54]);
        let eq60_e1010: f64 = (eq60_e1008 * (nv8 - nv2));
        let eq60_e1010_d_n0: f64 = (eq60_e1008_d_n0 * (nv8 - nv2));
        let eq60_e1010_d_n1: f64 = (eq60_e1008_d_n1 * (nv8 - nv2));
        let eq60_e1010_d_n2: f64 = ((eq60_e1008_d_n2 * (nv8 - nv2)) + (-eq60_e1008));
        let eq60_e1010_d_n3: f64 = (eq60_e1008_d_n3 * (nv8 - nv2));
        let eq60_e1010_d_n4: f64 = (eq60_e1008_d_n4 * (nv8 - nv2));
        let eq60_e1010_d_n5: f64 = (eq60_e1008_d_n5 * (nv8 - nv2));
        let eq60_e1010_d_n6: f64 = (eq60_e1008_d_n6 * (nv8 - nv2));
        let eq60_e1010_d_n7: f64 = (eq60_e1008_d_n7 * (nv8 - nv2));
        let eq60_e1010_d_n8: f64 = ((eq60_e1008_d_n8 * (nv8 - nv2)) + eq60_e1008);
        let eq60_e1010_d_n9: f64 = (eq60_e1008_d_n9 * (nv8 - nv2));
        let eq60_e1010_d_n10: f64 = (eq60_e1008_d_n10 * (nv8 - nv2));
        let eq60_e1010_d_n11: f64 = (eq60_e1008_d_n11 * (nv8 - nv2));
        let eq60_e1010_d_n12: f64 = (eq60_e1008_d_n12 * (nv8 - nv2));
        let eq60_e1010_d_n13: f64 = (eq60_e1008_d_n13 * (nv8 - nv2));
        let eq60_e1010_d_n14: f64 = (eq60_e1008_d_n14 * (nv8 - nv2));
        let eq60_e1010_d_n15: f64 = (eq60_e1008_d_n15 * (nv8 - nv2));
        let eq60_e1010_d_n16: f64 = (eq60_e1008_d_n16 * (nv8 - nv2));
        let eq60_e1010_d_n17: f64 = (eq60_e1008_d_n17 * (nv8 - nv2));
        let eq60_e1010_d_n18: f64 = (eq60_e1008_d_n18 * (nv8 - nv2));
        let eq60_e1010_d_n19: f64 = (eq60_e1008_d_n19 * (nv8 - nv2));
        let eq60_e1010_d_n20: f64 = (eq60_e1008_d_n20 * (nv8 - nv2));
        let eq60_e1010_d_n21: f64 = (eq60_e1008_d_n21 * (nv8 - nv2));
        let eq60_e1010_d_n22: f64 = (eq60_e1008_d_n22 * (nv8 - nv2));
        let eq60_e1010_d_b0: f64 = (eq60_e1008_d_b0 * (nv8 - nv2));
        let eq60_e1010_d_b1: f64 = (eq60_e1008_d_b1 * (nv8 - nv2));
        let eq60_e1010_d_b2: f64 = (eq60_e1008_d_b2 * (nv8 - nv2));
        let eq60_e1010_d_b3: f64 = (eq60_e1008_d_b3 * (nv8 - nv2));
        let eq60_e1010_d_b4: f64 = (eq60_e1008_d_b4 * (nv8 - nv2));
        let eq60_e1010_d_b5: f64 = (eq60_e1008_d_b5 * (nv8 - nv2));
        let eq60_e1010_d_b6: f64 = (eq60_e1008_d_b6 * (nv8 - nv2));
        let eq60_e1010_d_b7: f64 = (eq60_e1008_d_b7 * (nv8 - nv2));
        let eq60_e1010_d_b8: f64 = (eq60_e1008_d_b8 * (nv8 - nv2));
        let eq60_e1010_d_b9: f64 = (eq60_e1008_d_b9 * (nv8 - nv2));
        let eq60_e1010_d_b10: f64 = (eq60_e1008_d_b10 * (nv8 - nv2));
        let eq60_e1010_d_b11: f64 = (eq60_e1008_d_b11 * (nv8 - nv2));
        let eq60_e1010_d_b12: f64 = (eq60_e1008_d_b12 * (nv8 - nv2));
        let eq60_e1010_d_b13: f64 = (eq60_e1008_d_b13 * (nv8 - nv2));
        let eq60_e1010_d_b14: f64 = (eq60_e1008_d_b14 * (nv8 - nv2));
        let eq60_e1010_d_b15: f64 = (eq60_e1008_d_b15 * (nv8 - nv2));
        let eq60_e1010_d_b16: f64 = (eq60_e1008_d_b16 * (nv8 - nv2));
        let eq60_e1010_d_b17: f64 = (eq60_e1008_d_b17 * (nv8 - nv2));
        let eq60_e1010_d_b18: f64 = (eq60_e1008_d_b18 * (nv8 - nv2));
        let eq60_e1010_d_b19: f64 = (eq60_e1008_d_b19 * (nv8 - nv2));
        let eq60_e1010_d_b20: f64 = (eq60_e1008_d_b20 * (nv8 - nv2));
        let eq60_e1010_d_b21: f64 = (eq60_e1008_d_b21 * (nv8 - nv2));
        let eq60_e1010_d_b22: f64 = (eq60_e1008_d_b22 * (nv8 - nv2));
        let eq60_e1010_d_b23: f64 = (eq60_e1008_d_b23 * (nv8 - nv2));
        let eq60_e1010_d_b24: f64 = (eq60_e1008_d_b24 * (nv8 - nv2));
        let eq60_e1010_d_b25: f64 = (eq60_e1008_d_b25 * (nv8 - nv2));
        let eq60_e1010_d_b26: f64 = (eq60_e1008_d_b26 * (nv8 - nv2));
        let eq60_e1010_d_b27: f64 = (eq60_e1008_d_b27 * (nv8 - nv2));
        let eq60_e1010_d_b28: f64 = (eq60_e1008_d_b28 * (nv8 - nv2));
        let eq60_e1010_d_b29: f64 = (eq60_e1008_d_b29 * (nv8 - nv2));
        let eq60_e1010_d_b30: f64 = (eq60_e1008_d_b30 * (nv8 - nv2));
        let eq60_e1010_d_b31: f64 = (eq60_e1008_d_b31 * (nv8 - nv2));
        let eq60_e1010_d_b32: f64 = (eq60_e1008_d_b32 * (nv8 - nv2));
        let eq60_e1010_d_b33: f64 = (eq60_e1008_d_b33 * (nv8 - nv2));
        let eq60_e1010_d_b34: f64 = (eq60_e1008_d_b34 * (nv8 - nv2));
        let eq60_e1010_d_b35: f64 = (eq60_e1008_d_b35 * (nv8 - nv2));
        let eq60_e1010_d_b36: f64 = (eq60_e1008_d_b36 * (nv8 - nv2));
        let eq60_e1010_d_b37: f64 = (eq60_e1008_d_b37 * (nv8 - nv2));
        let eq60_e1010_d_b38: f64 = (eq60_e1008_d_b38 * (nv8 - nv2));
        let eq60_e1010_d_b39: f64 = (eq60_e1008_d_b39 * (nv8 - nv2));
        let eq60_e1010_d_b40: f64 = (eq60_e1008_d_b40 * (nv8 - nv2));
        let eq60_e1010_d_b41: f64 = (eq60_e1008_d_b41 * (nv8 - nv2));
        let eq60_e1010_d_b42: f64 = (eq60_e1008_d_b42 * (nv8 - nv2));
        let eq60_e1010_d_b43: f64 = (eq60_e1008_d_b43 * (nv8 - nv2));
        let eq60_e1010_d_b44: f64 = (eq60_e1008_d_b44 * (nv8 - nv2));
        let eq60_e1010_d_b45: f64 = (eq60_e1008_d_b45 * (nv8 - nv2));
        let eq60_e1010_d_b46: f64 = (eq60_e1008_d_b46 * (nv8 - nv2));
        let eq60_e1010_d_b47: f64 = (eq60_e1008_d_b47 * (nv8 - nv2));
        let eq60_e1010_d_b48: f64 = (eq60_e1008_d_b48 * (nv8 - nv2));
        let eq60_e1010_d_b49: f64 = (eq60_e1008_d_b49 * (nv8 - nv2));
        let eq60_e1010_d_b50: f64 = (eq60_e1008_d_b50 * (nv8 - nv2));
        let eq60_e1010_d_b51: f64 = (eq60_e1008_d_b51 * (nv8 - nv2));
        let eq60_e1010_d_b52: f64 = (eq60_e1008_d_b52 * (nv8 - nv2));
        let eq60_e1010_d_b53: f64 = (eq60_e1008_d_b53 * (nv8 - nv2));
        let eq60_e1010_d_b54: f64 = (eq60_e1008_d_b54 * (nv8 - nv2));
        (eq60_e1010, eq60_e1010_d_n0, eq60_e1010_d_n1, eq60_e1010_d_n2, eq60_e1010_d_n3, eq60_e1010_d_n4, eq60_e1010_d_n5, eq60_e1010_d_n6, eq60_e1010_d_n7, eq60_e1010_d_n8, eq60_e1010_d_n9, eq60_e1010_d_n10, eq60_e1010_d_n11, eq60_e1010_d_n12, eq60_e1010_d_n13, eq60_e1010_d_n14, eq60_e1010_d_n15, eq60_e1010_d_n16, eq60_e1010_d_n17, eq60_e1010_d_n18, eq60_e1010_d_n19, eq60_e1010_d_n20, eq60_e1010_d_n21, eq60_e1010_d_n22, eq60_e1010_d_b0, eq60_e1010_d_b1, eq60_e1010_d_b2, eq60_e1010_d_b3, eq60_e1010_d_b4, eq60_e1010_d_b5, eq60_e1010_d_b6, eq60_e1010_d_b7, eq60_e1010_d_b8, eq60_e1010_d_b9, eq60_e1010_d_b10, eq60_e1010_d_b11, eq60_e1010_d_b12, eq60_e1010_d_b13, eq60_e1010_d_b14, eq60_e1010_d_b15, eq60_e1010_d_b16, eq60_e1010_d_b17, eq60_e1010_d_b18, eq60_e1010_d_b19, eq60_e1010_d_b20, eq60_e1010_d_b21, eq60_e1010_d_b22, eq60_e1010_d_b23, eq60_e1010_d_b24, eq60_e1010_d_b25, eq60_e1010_d_b26, eq60_e1010_d_b27, eq60_e1010_d_b28, eq60_e1010_d_b29, eq60_e1010_d_b30, eq60_e1010_d_b31, eq60_e1010_d_b32, eq60_e1010_d_b33, eq60_e1010_d_b34, eq60_e1010_d_b35, eq60_e1010_d_b36, eq60_e1010_d_b37, eq60_e1010_d_b38, eq60_e1010_d_b39, eq60_e1010_d_b40, eq60_e1010_d_b41, eq60_e1010_d_b42, eq60_e1010_d_b43, eq60_e1010_d_b44, eq60_e1010_d_b45, eq60_e1010_d_b46, eq60_e1010_d_b47, eq60_e1010_d_b48, eq60_e1010_d_b49, eq60_e1010_d_b50, eq60_e1010_d_b51, eq60_e1010_d_b52, eq60_e1010_d_b53, eq60_e1010_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1012;
        let eq60_node_derivatives: [f64; 23] = [eq60_e1012_d_n0, eq60_e1012_d_n1, eq60_e1012_d_n2, eq60_e1012_d_n3, eq60_e1012_d_n4, eq60_e1012_d_n5, eq60_e1012_d_n6, eq60_e1012_d_n7, eq60_e1012_d_n8, eq60_e1012_d_n9, eq60_e1012_d_n10, eq60_e1012_d_n11, eq60_e1012_d_n12, eq60_e1012_d_n13, eq60_e1012_d_n14, eq60_e1012_d_n15, eq60_e1012_d_n16, eq60_e1012_d_n17, eq60_e1012_d_n18, eq60_e1012_d_n19, eq60_e1012_d_n20, eq60_e1012_d_n21, eq60_e1012_d_n22];
        let eq60_branch_derivatives: [f64; 55] = [eq60_e1012_d_b0, eq60_e1012_d_b1, eq60_e1012_d_b2, eq60_e1012_d_b3, eq60_e1012_d_b4, eq60_e1012_d_b5, eq60_e1012_d_b6, eq60_e1012_d_b7, eq60_e1012_d_b8, eq60_e1012_d_b9, eq60_e1012_d_b10, eq60_e1012_d_b11, eq60_e1012_d_b12, eq60_e1012_d_b13, eq60_e1012_d_b14, eq60_e1012_d_b15, eq60_e1012_d_b16, eq60_e1012_d_b17, eq60_e1012_d_b18, eq60_e1012_d_b19, eq60_e1012_d_b20, eq60_e1012_d_b21, eq60_e1012_d_b22, eq60_e1012_d_b23, eq60_e1012_d_b24, eq60_e1012_d_b25, eq60_e1012_d_b26, eq60_e1012_d_b27, eq60_e1012_d_b28, eq60_e1012_d_b29, eq60_e1012_d_b30, eq60_e1012_d_b31, eq60_e1012_d_b32, eq60_e1012_d_b33, eq60_e1012_d_b34, eq60_e1012_d_b35, eq60_e1012_d_b36, eq60_e1012_d_b37, eq60_e1012_d_b38, eq60_e1012_d_b39, eq60_e1012_d_b40, eq60_e1012_d_b41, eq60_e1012_d_b42, eq60_e1012_d_b43, eq60_e1012_d_b44, eq60_e1012_d_b45, eq60_e1012_d_b46, eq60_e1012_d_b47, eq60_e1012_d_b48, eq60_e1012_d_b49, eq60_e1012_d_b50, eq60_e1012_d_b51, eq60_e1012_d_b52, eq60_e1012_d_b53, eq60_e1012_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq60_value),
            &eq60_node_derivatives,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1019,) = {
    if ((!s.b[424]) && s.b[428]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e1019;
        stamper.stamp_potential_const_local(
            29,
            eq61_value,
        );
        let (eq62_e1026,) = {
    if ((!s.b[424]) && s.b[428]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq62_value: f64 = eq62_e1026;
        stamper.stamp_potential_const_local(
            30,
            eq62_value,
        );
        let (eq63_e1034,) = {
    if ((!s.b[424]) && (!s.b[428])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e1034;
        stamper.stamp_potential_const_local(
            31,
            eq63_value,
        );
        let (eq64_e1042,) = {
    if ((!s.b[424]) && (!s.b[428])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e1042;
        stamper.stamp_potential_const_local(
            32,
            eq64_value,
        );
        let (eq65_e1050,) = {
    if s.b[429] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq65_value: f64 = eq65_e1050;
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (eq65_value),
        );
        let (eq66_e1068,) = {
    if ((s.b[429] && s.b[430]) && s.b[431]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e1068;
        stamper.stamp_current_const_local(
            Some(0),
            Some(18),
            multiplicity * (eq66_value),
        );
        let (eq67_e1086,) = {
    if ((s.b[429] && s.b[430]) && s.b[431]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1086;
        stamper.stamp_current_const_local(
            Some(2),
            Some(22),
            multiplicity * (eq67_value),
        );
        let (eq68_e1105,) = {
    if ((s.b[429] && s.b[430]) && (!s.b[431])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e1105;
        stamper.stamp_current_const_local(
            Some(0),
            Some(7),
            multiplicity * (eq68_value),
        );
        let (eq69_e1124,) = {
    if ((s.b[429] && s.b[430]) && (!s.b[431])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq69_value: f64 = eq69_e1124;
        stamper.stamp_current_const_local(
            Some(2),
            Some(8),
            multiplicity * (eq69_value),
        );
        let (eq70_e1137,) = {
    if s.b[432] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e1137;
        stamper.stamp_current_const_local(
            Some(9),
            Some(8),
            multiplicity * (eq70_value),
        );
        let (eq71_e1150,) = {
    if s.b[432] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e1150;
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (eq71_value),
        );
    }

    pub(super) fn stamp_transient_equations_block_12(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq72_e1166, eq72_e1166_d_n0, eq72_e1166_d_n1, eq72_e1166_d_n2, eq72_e1166_d_n3, eq72_e1166_d_n4, eq72_e1166_d_n5, eq72_e1166_d_n6, eq72_e1166_d_n7, eq72_e1166_d_n8, eq72_e1166_d_n9, eq72_e1166_d_n10, eq72_e1166_d_n11, eq72_e1166_d_n12, eq72_e1166_d_n13, eq72_e1166_d_n14, eq72_e1166_d_n15, eq72_e1166_d_n16, eq72_e1166_d_n17, eq72_e1166_d_n18, eq72_e1166_d_n19, eq72_e1166_d_n20, eq72_e1166_d_n21, eq72_e1166_d_n22, eq72_e1166_d_b0, eq72_e1166_d_b1, eq72_e1166_d_b2, eq72_e1166_d_b3, eq72_e1166_d_b4, eq72_e1166_d_b5, eq72_e1166_d_b6, eq72_e1166_d_b7, eq72_e1166_d_b8, eq72_e1166_d_b9, eq72_e1166_d_b10, eq72_e1166_d_b11, eq72_e1166_d_b12, eq72_e1166_d_b13, eq72_e1166_d_b14, eq72_e1166_d_b15, eq72_e1166_d_b16, eq72_e1166_d_b17, eq72_e1166_d_b18, eq72_e1166_d_b19, eq72_e1166_d_b20, eq72_e1166_d_b21, eq72_e1166_d_b22, eq72_e1166_d_b23, eq72_e1166_d_b24, eq72_e1166_d_b25, eq72_e1166_d_b26, eq72_e1166_d_b27, eq72_e1166_d_b28, eq72_e1166_d_b29, eq72_e1166_d_b30, eq72_e1166_d_b31, eq72_e1166_d_b32, eq72_e1166_d_b33, eq72_e1166_d_b34, eq72_e1166_d_b35, eq72_e1166_d_b36, eq72_e1166_d_b37, eq72_e1166_d_b38, eq72_e1166_d_b39, eq72_e1166_d_b40, eq72_e1166_d_b41, eq72_e1166_d_b42, eq72_e1166_d_b43, eq72_e1166_d_b44, eq72_e1166_d_b45, eq72_e1166_d_b46, eq72_e1166_d_b47, eq72_e1166_d_b48, eq72_e1166_d_b49, eq72_e1166_d_b50, eq72_e1166_d_b51, eq72_e1166_d_b52, eq72_e1166_d_b53, eq72_e1166_d_b54,) = {
    if (s.b[433] && s.b[434]) {
        let eq72_e1156: f64 = (p.p6 * s.v[48]);
        let eq72_e1156_d_n0: f64 = (p.p6 * s.dn[48][0]);
        let eq72_e1156_d_n1: f64 = (p.p6 * s.dn[48][1]);
        let eq72_e1156_d_n2: f64 = (p.p6 * s.dn[48][2]);
        let eq72_e1156_d_n3: f64 = (p.p6 * s.dn[48][3]);
        let eq72_e1156_d_n4: f64 = (p.p6 * s.dn[48][4]);
        let eq72_e1156_d_n5: f64 = (p.p6 * s.dn[48][5]);
        let eq72_e1156_d_n6: f64 = (p.p6 * s.dn[48][6]);
        let eq72_e1156_d_n7: f64 = (p.p6 * s.dn[48][7]);
        let eq72_e1156_d_n8: f64 = (p.p6 * s.dn[48][8]);
        let eq72_e1156_d_n9: f64 = (p.p6 * s.dn[48][9]);
        let eq72_e1156_d_n10: f64 = (p.p6 * s.dn[48][10]);
        let eq72_e1156_d_n11: f64 = (p.p6 * s.dn[48][11]);
        let eq72_e1156_d_n12: f64 = (p.p6 * s.dn[48][12]);
        let eq72_e1156_d_n13: f64 = (p.p6 * s.dn[48][13]);
        let eq72_e1156_d_n14: f64 = (p.p6 * s.dn[48][14]);
        let eq72_e1156_d_n15: f64 = (p.p6 * s.dn[48][15]);
        let eq72_e1156_d_n16: f64 = (p.p6 * s.dn[48][16]);
        let eq72_e1156_d_n17: f64 = (p.p6 * s.dn[48][17]);
        let eq72_e1156_d_n18: f64 = (p.p6 * s.dn[48][18]);
        let eq72_e1156_d_n19: f64 = (p.p6 * s.dn[48][19]);
        let eq72_e1156_d_n20: f64 = (p.p6 * s.dn[48][20]);
        let eq72_e1156_d_n21: f64 = (p.p6 * s.dn[48][21]);
        let eq72_e1156_d_n22: f64 = (p.p6 * s.dn[48][22]);
        let eq72_e1156_d_b0: f64 = (p.p6 * s.db[48][0]);
        let eq72_e1156_d_b1: f64 = (p.p6 * s.db[48][1]);
        let eq72_e1156_d_b2: f64 = (p.p6 * s.db[48][2]);
        let eq72_e1156_d_b3: f64 = (p.p6 * s.db[48][3]);
        let eq72_e1156_d_b4: f64 = (p.p6 * s.db[48][4]);
        let eq72_e1156_d_b5: f64 = (p.p6 * s.db[48][5]);
        let eq72_e1156_d_b6: f64 = (p.p6 * s.db[48][6]);
        let eq72_e1156_d_b7: f64 = (p.p6 * s.db[48][7]);
        let eq72_e1156_d_b8: f64 = (p.p6 * s.db[48][8]);
        let eq72_e1156_d_b9: f64 = (p.p6 * s.db[48][9]);
        let eq72_e1156_d_b10: f64 = (p.p6 * s.db[48][10]);
        let eq72_e1156_d_b11: f64 = (p.p6 * s.db[48][11]);
        let eq72_e1156_d_b12: f64 = (p.p6 * s.db[48][12]);
        let eq72_e1156_d_b13: f64 = (p.p6 * s.db[48][13]);
        let eq72_e1156_d_b14: f64 = (p.p6 * s.db[48][14]);
        let eq72_e1156_d_b15: f64 = (p.p6 * s.db[48][15]);
        let eq72_e1156_d_b16: f64 = (p.p6 * s.db[48][16]);
        let eq72_e1156_d_b17: f64 = (p.p6 * s.db[48][17]);
        let eq72_e1156_d_b18: f64 = (p.p6 * s.db[48][18]);
        let eq72_e1156_d_b19: f64 = (p.p6 * s.db[48][19]);
        let eq72_e1156_d_b20: f64 = (p.p6 * s.db[48][20]);
        let eq72_e1156_d_b21: f64 = (p.p6 * s.db[48][21]);
        let eq72_e1156_d_b22: f64 = (p.p6 * s.db[48][22]);
        let eq72_e1156_d_b23: f64 = (p.p6 * s.db[48][23]);
        let eq72_e1156_d_b24: f64 = (p.p6 * s.db[48][24]);
        let eq72_e1156_d_b25: f64 = (p.p6 * s.db[48][25]);
        let eq72_e1156_d_b26: f64 = (p.p6 * s.db[48][26]);
        let eq72_e1156_d_b27: f64 = (p.p6 * s.db[48][27]);
        let eq72_e1156_d_b28: f64 = (p.p6 * s.db[48][28]);
        let eq72_e1156_d_b29: f64 = (p.p6 * s.db[48][29]);
        let eq72_e1156_d_b30: f64 = (p.p6 * s.db[48][30]);
        let eq72_e1156_d_b31: f64 = (p.p6 * s.db[48][31]);
        let eq72_e1156_d_b32: f64 = (p.p6 * s.db[48][32]);
        let eq72_e1156_d_b33: f64 = (p.p6 * s.db[48][33]);
        let eq72_e1156_d_b34: f64 = (p.p6 * s.db[48][34]);
        let eq72_e1156_d_b35: f64 = (p.p6 * s.db[48][35]);
        let eq72_e1156_d_b36: f64 = (p.p6 * s.db[48][36]);
        let eq72_e1156_d_b37: f64 = (p.p6 * s.db[48][37]);
        let eq72_e1156_d_b38: f64 = (p.p6 * s.db[48][38]);
        let eq72_e1156_d_b39: f64 = (p.p6 * s.db[48][39]);
        let eq72_e1156_d_b40: f64 = (p.p6 * s.db[48][40]);
        let eq72_e1156_d_b41: f64 = (p.p6 * s.db[48][41]);
        let eq72_e1156_d_b42: f64 = (p.p6 * s.db[48][42]);
        let eq72_e1156_d_b43: f64 = (p.p6 * s.db[48][43]);
        let eq72_e1156_d_b44: f64 = (p.p6 * s.db[48][44]);
        let eq72_e1156_d_b45: f64 = (p.p6 * s.db[48][45]);
        let eq72_e1156_d_b46: f64 = (p.p6 * s.db[48][46]);
        let eq72_e1156_d_b47: f64 = (p.p6 * s.db[48][47]);
        let eq72_e1156_d_b48: f64 = (p.p6 * s.db[48][48]);
        let eq72_e1156_d_b49: f64 = (p.p6 * s.db[48][49]);
        let eq72_e1156_d_b50: f64 = (p.p6 * s.db[48][50]);
        let eq72_e1156_d_b51: f64 = (p.p6 * s.db[48][51]);
        let eq72_e1156_d_b52: f64 = (p.p6 * s.db[48][52]);
        let eq72_e1156_d_b53: f64 = (p.p6 * s.db[48][53]);
        let eq72_e1156_d_b54: f64 = (p.p6 * s.db[48][54]);
        let eq72_e1158: f64 = (eq72_e1156 * s.v[233]);
        let eq72_e1158_d_n0: f64 = ((eq72_e1156_d_n0 * s.v[233]) + (eq72_e1156 * s.dn[233][0]));
        let eq72_e1158_d_n1: f64 = ((eq72_e1156_d_n1 * s.v[233]) + (eq72_e1156 * s.dn[233][1]));
        let eq72_e1158_d_n2: f64 = ((eq72_e1156_d_n2 * s.v[233]) + (eq72_e1156 * s.dn[233][2]));
        let eq72_e1158_d_n3: f64 = ((eq72_e1156_d_n3 * s.v[233]) + (eq72_e1156 * s.dn[233][3]));
        let eq72_e1158_d_n4: f64 = ((eq72_e1156_d_n4 * s.v[233]) + (eq72_e1156 * s.dn[233][4]));
        let eq72_e1158_d_n5: f64 = ((eq72_e1156_d_n5 * s.v[233]) + (eq72_e1156 * s.dn[233][5]));
        let eq72_e1158_d_n6: f64 = ((eq72_e1156_d_n6 * s.v[233]) + (eq72_e1156 * s.dn[233][6]));
        let eq72_e1158_d_n7: f64 = ((eq72_e1156_d_n7 * s.v[233]) + (eq72_e1156 * s.dn[233][7]));
        let eq72_e1158_d_n8: f64 = ((eq72_e1156_d_n8 * s.v[233]) + (eq72_e1156 * s.dn[233][8]));
        let eq72_e1158_d_n9: f64 = ((eq72_e1156_d_n9 * s.v[233]) + (eq72_e1156 * s.dn[233][9]));
        let eq72_e1158_d_n10: f64 = ((eq72_e1156_d_n10 * s.v[233]) + (eq72_e1156 * s.dn[233][10]));
        let eq72_e1158_d_n11: f64 = ((eq72_e1156_d_n11 * s.v[233]) + (eq72_e1156 * s.dn[233][11]));
        let eq72_e1158_d_n12: f64 = ((eq72_e1156_d_n12 * s.v[233]) + (eq72_e1156 * s.dn[233][12]));
        let eq72_e1158_d_n13: f64 = ((eq72_e1156_d_n13 * s.v[233]) + (eq72_e1156 * s.dn[233][13]));
        let eq72_e1158_d_n14: f64 = ((eq72_e1156_d_n14 * s.v[233]) + (eq72_e1156 * s.dn[233][14]));
        let eq72_e1158_d_n15: f64 = ((eq72_e1156_d_n15 * s.v[233]) + (eq72_e1156 * s.dn[233][15]));
        let eq72_e1158_d_n16: f64 = ((eq72_e1156_d_n16 * s.v[233]) + (eq72_e1156 * s.dn[233][16]));
        let eq72_e1158_d_n17: f64 = ((eq72_e1156_d_n17 * s.v[233]) + (eq72_e1156 * s.dn[233][17]));
        let eq72_e1158_d_n18: f64 = ((eq72_e1156_d_n18 * s.v[233]) + (eq72_e1156 * s.dn[233][18]));
        let eq72_e1158_d_n19: f64 = ((eq72_e1156_d_n19 * s.v[233]) + (eq72_e1156 * s.dn[233][19]));
        let eq72_e1158_d_n20: f64 = ((eq72_e1156_d_n20 * s.v[233]) + (eq72_e1156 * s.dn[233][20]));
        let eq72_e1158_d_n21: f64 = ((eq72_e1156_d_n21 * s.v[233]) + (eq72_e1156 * s.dn[233][21]));
        let eq72_e1158_d_n22: f64 = ((eq72_e1156_d_n22 * s.v[233]) + (eq72_e1156 * s.dn[233][22]));
        let eq72_e1158_d_b0: f64 = ((eq72_e1156_d_b0 * s.v[233]) + (eq72_e1156 * s.db[233][0]));
        let eq72_e1158_d_b1: f64 = ((eq72_e1156_d_b1 * s.v[233]) + (eq72_e1156 * s.db[233][1]));
        let eq72_e1158_d_b2: f64 = ((eq72_e1156_d_b2 * s.v[233]) + (eq72_e1156 * s.db[233][2]));
        let eq72_e1158_d_b3: f64 = ((eq72_e1156_d_b3 * s.v[233]) + (eq72_e1156 * s.db[233][3]));
        let eq72_e1158_d_b4: f64 = ((eq72_e1156_d_b4 * s.v[233]) + (eq72_e1156 * s.db[233][4]));
        let eq72_e1158_d_b5: f64 = ((eq72_e1156_d_b5 * s.v[233]) + (eq72_e1156 * s.db[233][5]));
        let eq72_e1158_d_b6: f64 = ((eq72_e1156_d_b6 * s.v[233]) + (eq72_e1156 * s.db[233][6]));
        let eq72_e1158_d_b7: f64 = ((eq72_e1156_d_b7 * s.v[233]) + (eq72_e1156 * s.db[233][7]));
        let eq72_e1158_d_b8: f64 = ((eq72_e1156_d_b8 * s.v[233]) + (eq72_e1156 * s.db[233][8]));
        let eq72_e1158_d_b9: f64 = ((eq72_e1156_d_b9 * s.v[233]) + (eq72_e1156 * s.db[233][9]));
        let eq72_e1158_d_b10: f64 = ((eq72_e1156_d_b10 * s.v[233]) + (eq72_e1156 * s.db[233][10]));
        let eq72_e1158_d_b11: f64 = ((eq72_e1156_d_b11 * s.v[233]) + (eq72_e1156 * s.db[233][11]));
        let eq72_e1158_d_b12: f64 = ((eq72_e1156_d_b12 * s.v[233]) + (eq72_e1156 * s.db[233][12]));
        let eq72_e1158_d_b13: f64 = ((eq72_e1156_d_b13 * s.v[233]) + (eq72_e1156 * s.db[233][13]));
        let eq72_e1158_d_b14: f64 = ((eq72_e1156_d_b14 * s.v[233]) + (eq72_e1156 * s.db[233][14]));
        let eq72_e1158_d_b15: f64 = ((eq72_e1156_d_b15 * s.v[233]) + (eq72_e1156 * s.db[233][15]));
        let eq72_e1158_d_b16: f64 = ((eq72_e1156_d_b16 * s.v[233]) + (eq72_e1156 * s.db[233][16]));
        let eq72_e1158_d_b17: f64 = ((eq72_e1156_d_b17 * s.v[233]) + (eq72_e1156 * s.db[233][17]));
        let eq72_e1158_d_b18: f64 = ((eq72_e1156_d_b18 * s.v[233]) + (eq72_e1156 * s.db[233][18]));
        let eq72_e1158_d_b19: f64 = ((eq72_e1156_d_b19 * s.v[233]) + (eq72_e1156 * s.db[233][19]));
        let eq72_e1158_d_b20: f64 = ((eq72_e1156_d_b20 * s.v[233]) + (eq72_e1156 * s.db[233][20]));
        let eq72_e1158_d_b21: f64 = ((eq72_e1156_d_b21 * s.v[233]) + (eq72_e1156 * s.db[233][21]));
        let eq72_e1158_d_b22: f64 = ((eq72_e1156_d_b22 * s.v[233]) + (eq72_e1156 * s.db[233][22]));
        let eq72_e1158_d_b23: f64 = ((eq72_e1156_d_b23 * s.v[233]) + (eq72_e1156 * s.db[233][23]));
        let eq72_e1158_d_b24: f64 = ((eq72_e1156_d_b24 * s.v[233]) + (eq72_e1156 * s.db[233][24]));
        let eq72_e1158_d_b25: f64 = ((eq72_e1156_d_b25 * s.v[233]) + (eq72_e1156 * s.db[233][25]));
        let eq72_e1158_d_b26: f64 = ((eq72_e1156_d_b26 * s.v[233]) + (eq72_e1156 * s.db[233][26]));
        let eq72_e1158_d_b27: f64 = ((eq72_e1156_d_b27 * s.v[233]) + (eq72_e1156 * s.db[233][27]));
        let eq72_e1158_d_b28: f64 = ((eq72_e1156_d_b28 * s.v[233]) + (eq72_e1156 * s.db[233][28]));
        let eq72_e1158_d_b29: f64 = ((eq72_e1156_d_b29 * s.v[233]) + (eq72_e1156 * s.db[233][29]));
        let eq72_e1158_d_b30: f64 = ((eq72_e1156_d_b30 * s.v[233]) + (eq72_e1156 * s.db[233][30]));
        let eq72_e1158_d_b31: f64 = ((eq72_e1156_d_b31 * s.v[233]) + (eq72_e1156 * s.db[233][31]));
        let eq72_e1158_d_b32: f64 = ((eq72_e1156_d_b32 * s.v[233]) + (eq72_e1156 * s.db[233][32]));
        let eq72_e1158_d_b33: f64 = ((eq72_e1156_d_b33 * s.v[233]) + (eq72_e1156 * s.db[233][33]));
        let eq72_e1158_d_b34: f64 = ((eq72_e1156_d_b34 * s.v[233]) + (eq72_e1156 * s.db[233][34]));
        let eq72_e1158_d_b35: f64 = ((eq72_e1156_d_b35 * s.v[233]) + (eq72_e1156 * s.db[233][35]));
        let eq72_e1158_d_b36: f64 = ((eq72_e1156_d_b36 * s.v[233]) + (eq72_e1156 * s.db[233][36]));
        let eq72_e1158_d_b37: f64 = ((eq72_e1156_d_b37 * s.v[233]) + (eq72_e1156 * s.db[233][37]));
        let eq72_e1158_d_b38: f64 = ((eq72_e1156_d_b38 * s.v[233]) + (eq72_e1156 * s.db[233][38]));
        let eq72_e1158_d_b39: f64 = ((eq72_e1156_d_b39 * s.v[233]) + (eq72_e1156 * s.db[233][39]));
        let eq72_e1158_d_b40: f64 = ((eq72_e1156_d_b40 * s.v[233]) + (eq72_e1156 * s.db[233][40]));
        let eq72_e1158_d_b41: f64 = ((eq72_e1156_d_b41 * s.v[233]) + (eq72_e1156 * s.db[233][41]));
        let eq72_e1158_d_b42: f64 = ((eq72_e1156_d_b42 * s.v[233]) + (eq72_e1156 * s.db[233][42]));
        let eq72_e1158_d_b43: f64 = ((eq72_e1156_d_b43 * s.v[233]) + (eq72_e1156 * s.db[233][43]));
        let eq72_e1158_d_b44: f64 = ((eq72_e1156_d_b44 * s.v[233]) + (eq72_e1156 * s.db[233][44]));
        let eq72_e1158_d_b45: f64 = ((eq72_e1156_d_b45 * s.v[233]) + (eq72_e1156 * s.db[233][45]));
        let eq72_e1158_d_b46: f64 = ((eq72_e1156_d_b46 * s.v[233]) + (eq72_e1156 * s.db[233][46]));
        let eq72_e1158_d_b47: f64 = ((eq72_e1156_d_b47 * s.v[233]) + (eq72_e1156 * s.db[233][47]));
        let eq72_e1158_d_b48: f64 = ((eq72_e1156_d_b48 * s.v[233]) + (eq72_e1156 * s.db[233][48]));
        let eq72_e1158_d_b49: f64 = ((eq72_e1156_d_b49 * s.v[233]) + (eq72_e1156 * s.db[233][49]));
        let eq72_e1158_d_b50: f64 = ((eq72_e1156_d_b50 * s.v[233]) + (eq72_e1156 * s.db[233][50]));
        let eq72_e1158_d_b51: f64 = ((eq72_e1156_d_b51 * s.v[233]) + (eq72_e1156 * s.db[233][51]));
        let eq72_e1158_d_b52: f64 = ((eq72_e1156_d_b52 * s.v[233]) + (eq72_e1156 * s.db[233][52]));
        let eq72_e1158_d_b53: f64 = ((eq72_e1156_d_b53 * s.v[233]) + (eq72_e1156 * s.db[233][53]));
        let eq72_e1158_d_b54: f64 = ((eq72_e1156_d_b54 * s.v[233]) + (eq72_e1156 * s.db[233][54]));
        let eq72_e1161: f64 = (p.p6 * s.v[379]);
        let eq72_e1161_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq72_e1161_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq72_e1161_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq72_e1161_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq72_e1161_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq72_e1161_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq72_e1161_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq72_e1161_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq72_e1161_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq72_e1161_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq72_e1161_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq72_e1161_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq72_e1161_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq72_e1161_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq72_e1161_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq72_e1161_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq72_e1161_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq72_e1161_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq72_e1161_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq72_e1161_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq72_e1161_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq72_e1161_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq72_e1161_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq72_e1161_d_b0: f64 = (p.p6 * s.db[379][0]);
        let eq72_e1161_d_b1: f64 = (p.p6 * s.db[379][1]);
        let eq72_e1161_d_b2: f64 = (p.p6 * s.db[379][2]);
        let eq72_e1161_d_b3: f64 = (p.p6 * s.db[379][3]);
        let eq72_e1161_d_b4: f64 = (p.p6 * s.db[379][4]);
        let eq72_e1161_d_b5: f64 = (p.p6 * s.db[379][5]);
        let eq72_e1161_d_b6: f64 = (p.p6 * s.db[379][6]);
        let eq72_e1161_d_b7: f64 = (p.p6 * s.db[379][7]);
        let eq72_e1161_d_b8: f64 = (p.p6 * s.db[379][8]);
        let eq72_e1161_d_b9: f64 = (p.p6 * s.db[379][9]);
        let eq72_e1161_d_b10: f64 = (p.p6 * s.db[379][10]);
        let eq72_e1161_d_b11: f64 = (p.p6 * s.db[379][11]);
        let eq72_e1161_d_b12: f64 = (p.p6 * s.db[379][12]);
        let eq72_e1161_d_b13: f64 = (p.p6 * s.db[379][13]);
        let eq72_e1161_d_b14: f64 = (p.p6 * s.db[379][14]);
        let eq72_e1161_d_b15: f64 = (p.p6 * s.db[379][15]);
        let eq72_e1161_d_b16: f64 = (p.p6 * s.db[379][16]);
        let eq72_e1161_d_b17: f64 = (p.p6 * s.db[379][17]);
        let eq72_e1161_d_b18: f64 = (p.p6 * s.db[379][18]);
        let eq72_e1161_d_b19: f64 = (p.p6 * s.db[379][19]);
        let eq72_e1161_d_b20: f64 = (p.p6 * s.db[379][20]);
        let eq72_e1161_d_b21: f64 = (p.p6 * s.db[379][21]);
        let eq72_e1161_d_b22: f64 = (p.p6 * s.db[379][22]);
        let eq72_e1161_d_b23: f64 = (p.p6 * s.db[379][23]);
        let eq72_e1161_d_b24: f64 = (p.p6 * s.db[379][24]);
        let eq72_e1161_d_b25: f64 = (p.p6 * s.db[379][25]);
        let eq72_e1161_d_b26: f64 = (p.p6 * s.db[379][26]);
        let eq72_e1161_d_b27: f64 = (p.p6 * s.db[379][27]);
        let eq72_e1161_d_b28: f64 = (p.p6 * s.db[379][28]);
        let eq72_e1161_d_b29: f64 = (p.p6 * s.db[379][29]);
        let eq72_e1161_d_b30: f64 = (p.p6 * s.db[379][30]);
        let eq72_e1161_d_b31: f64 = (p.p6 * s.db[379][31]);
        let eq72_e1161_d_b32: f64 = (p.p6 * s.db[379][32]);
        let eq72_e1161_d_b33: f64 = (p.p6 * s.db[379][33]);
        let eq72_e1161_d_b34: f64 = (p.p6 * s.db[379][34]);
        let eq72_e1161_d_b35: f64 = (p.p6 * s.db[379][35]);
        let eq72_e1161_d_b36: f64 = (p.p6 * s.db[379][36]);
        let eq72_e1161_d_b37: f64 = (p.p6 * s.db[379][37]);
        let eq72_e1161_d_b38: f64 = (p.p6 * s.db[379][38]);
        let eq72_e1161_d_b39: f64 = (p.p6 * s.db[379][39]);
        let eq72_e1161_d_b40: f64 = (p.p6 * s.db[379][40]);
        let eq72_e1161_d_b41: f64 = (p.p6 * s.db[379][41]);
        let eq72_e1161_d_b42: f64 = (p.p6 * s.db[379][42]);
        let eq72_e1161_d_b43: f64 = (p.p6 * s.db[379][43]);
        let eq72_e1161_d_b44: f64 = (p.p6 * s.db[379][44]);
        let eq72_e1161_d_b45: f64 = (p.p6 * s.db[379][45]);
        let eq72_e1161_d_b46: f64 = (p.p6 * s.db[379][46]);
        let eq72_e1161_d_b47: f64 = (p.p6 * s.db[379][47]);
        let eq72_e1161_d_b48: f64 = (p.p6 * s.db[379][48]);
        let eq72_e1161_d_b49: f64 = (p.p6 * s.db[379][49]);
        let eq72_e1161_d_b50: f64 = (p.p6 * s.db[379][50]);
        let eq72_e1161_d_b51: f64 = (p.p6 * s.db[379][51]);
        let eq72_e1161_d_b52: f64 = (p.p6 * s.db[379][52]);
        let eq72_e1161_d_b53: f64 = (p.p6 * s.db[379][53]);
        let eq72_e1161_d_b54: f64 = (p.p6 * s.db[379][54]);
        let eq72_e1163: f64 = (eq72_e1161 * (nv15 - nv7));
        let eq72_e1163_d_n0: f64 = (eq72_e1161_d_n0 * (nv15 - nv7));
        let eq72_e1163_d_n1: f64 = (eq72_e1161_d_n1 * (nv15 - nv7));
        let eq72_e1163_d_n2: f64 = (eq72_e1161_d_n2 * (nv15 - nv7));
        let eq72_e1163_d_n3: f64 = (eq72_e1161_d_n3 * (nv15 - nv7));
        let eq72_e1163_d_n4: f64 = (eq72_e1161_d_n4 * (nv15 - nv7));
        let eq72_e1163_d_n5: f64 = (eq72_e1161_d_n5 * (nv15 - nv7));
        let eq72_e1163_d_n6: f64 = (eq72_e1161_d_n6 * (nv15 - nv7));
        let eq72_e1163_d_n7: f64 = ((eq72_e1161_d_n7 * (nv15 - nv7)) + (-eq72_e1161));
        let eq72_e1163_d_n8: f64 = (eq72_e1161_d_n8 * (nv15 - nv7));
        let eq72_e1163_d_n9: f64 = (eq72_e1161_d_n9 * (nv15 - nv7));
        let eq72_e1163_d_n10: f64 = (eq72_e1161_d_n10 * (nv15 - nv7));
        let eq72_e1163_d_n11: f64 = (eq72_e1161_d_n11 * (nv15 - nv7));
        let eq72_e1163_d_n12: f64 = (eq72_e1161_d_n12 * (nv15 - nv7));
        let eq72_e1163_d_n13: f64 = (eq72_e1161_d_n13 * (nv15 - nv7));
        let eq72_e1163_d_n14: f64 = (eq72_e1161_d_n14 * (nv15 - nv7));
        let eq72_e1163_d_n15: f64 = ((eq72_e1161_d_n15 * (nv15 - nv7)) + eq72_e1161);
        let eq72_e1163_d_n16: f64 = (eq72_e1161_d_n16 * (nv15 - nv7));
        let eq72_e1163_d_n17: f64 = (eq72_e1161_d_n17 * (nv15 - nv7));
        let eq72_e1163_d_n18: f64 = (eq72_e1161_d_n18 * (nv15 - nv7));
        let eq72_e1163_d_n19: f64 = (eq72_e1161_d_n19 * (nv15 - nv7));
        let eq72_e1163_d_n20: f64 = (eq72_e1161_d_n20 * (nv15 - nv7));
        let eq72_e1163_d_n21: f64 = (eq72_e1161_d_n21 * (nv15 - nv7));
        let eq72_e1163_d_n22: f64 = (eq72_e1161_d_n22 * (nv15 - nv7));
        let eq72_e1163_d_b0: f64 = (eq72_e1161_d_b0 * (nv15 - nv7));
        let eq72_e1163_d_b1: f64 = (eq72_e1161_d_b1 * (nv15 - nv7));
        let eq72_e1163_d_b2: f64 = (eq72_e1161_d_b2 * (nv15 - nv7));
        let eq72_e1163_d_b3: f64 = (eq72_e1161_d_b3 * (nv15 - nv7));
        let eq72_e1163_d_b4: f64 = (eq72_e1161_d_b4 * (nv15 - nv7));
        let eq72_e1163_d_b5: f64 = (eq72_e1161_d_b5 * (nv15 - nv7));
        let eq72_e1163_d_b6: f64 = (eq72_e1161_d_b6 * (nv15 - nv7));
        let eq72_e1163_d_b7: f64 = (eq72_e1161_d_b7 * (nv15 - nv7));
        let eq72_e1163_d_b8: f64 = (eq72_e1161_d_b8 * (nv15 - nv7));
        let eq72_e1163_d_b9: f64 = (eq72_e1161_d_b9 * (nv15 - nv7));
        let eq72_e1163_d_b10: f64 = (eq72_e1161_d_b10 * (nv15 - nv7));
        let eq72_e1163_d_b11: f64 = (eq72_e1161_d_b11 * (nv15 - nv7));
        let eq72_e1163_d_b12: f64 = (eq72_e1161_d_b12 * (nv15 - nv7));
        let eq72_e1163_d_b13: f64 = (eq72_e1161_d_b13 * (nv15 - nv7));
        let eq72_e1163_d_b14: f64 = (eq72_e1161_d_b14 * (nv15 - nv7));
        let eq72_e1163_d_b15: f64 = (eq72_e1161_d_b15 * (nv15 - nv7));
        let eq72_e1163_d_b16: f64 = (eq72_e1161_d_b16 * (nv15 - nv7));
        let eq72_e1163_d_b17: f64 = (eq72_e1161_d_b17 * (nv15 - nv7));
        let eq72_e1163_d_b18: f64 = (eq72_e1161_d_b18 * (nv15 - nv7));
        let eq72_e1163_d_b19: f64 = (eq72_e1161_d_b19 * (nv15 - nv7));
        let eq72_e1163_d_b20: f64 = (eq72_e1161_d_b20 * (nv15 - nv7));
        let eq72_e1163_d_b21: f64 = (eq72_e1161_d_b21 * (nv15 - nv7));
        let eq72_e1163_d_b22: f64 = (eq72_e1161_d_b22 * (nv15 - nv7));
        let eq72_e1163_d_b23: f64 = (eq72_e1161_d_b23 * (nv15 - nv7));
        let eq72_e1163_d_b24: f64 = (eq72_e1161_d_b24 * (nv15 - nv7));
        let eq72_e1163_d_b25: f64 = (eq72_e1161_d_b25 * (nv15 - nv7));
        let eq72_e1163_d_b26: f64 = (eq72_e1161_d_b26 * (nv15 - nv7));
        let eq72_e1163_d_b27: f64 = (eq72_e1161_d_b27 * (nv15 - nv7));
        let eq72_e1163_d_b28: f64 = (eq72_e1161_d_b28 * (nv15 - nv7));
        let eq72_e1163_d_b29: f64 = (eq72_e1161_d_b29 * (nv15 - nv7));
        let eq72_e1163_d_b30: f64 = (eq72_e1161_d_b30 * (nv15 - nv7));
        let eq72_e1163_d_b31: f64 = (eq72_e1161_d_b31 * (nv15 - nv7));
        let eq72_e1163_d_b32: f64 = (eq72_e1161_d_b32 * (nv15 - nv7));
        let eq72_e1163_d_b33: f64 = (eq72_e1161_d_b33 * (nv15 - nv7));
        let eq72_e1163_d_b34: f64 = (eq72_e1161_d_b34 * (nv15 - nv7));
        let eq72_e1163_d_b35: f64 = (eq72_e1161_d_b35 * (nv15 - nv7));
        let eq72_e1163_d_b36: f64 = (eq72_e1161_d_b36 * (nv15 - nv7));
        let eq72_e1163_d_b37: f64 = (eq72_e1161_d_b37 * (nv15 - nv7));
        let eq72_e1163_d_b38: f64 = (eq72_e1161_d_b38 * (nv15 - nv7));
        let eq72_e1163_d_b39: f64 = (eq72_e1161_d_b39 * (nv15 - nv7));
        let eq72_e1163_d_b40: f64 = (eq72_e1161_d_b40 * (nv15 - nv7));
        let eq72_e1163_d_b41: f64 = (eq72_e1161_d_b41 * (nv15 - nv7));
        let eq72_e1163_d_b42: f64 = (eq72_e1161_d_b42 * (nv15 - nv7));
        let eq72_e1163_d_b43: f64 = (eq72_e1161_d_b43 * (nv15 - nv7));
        let eq72_e1163_d_b44: f64 = (eq72_e1161_d_b44 * (nv15 - nv7));
        let eq72_e1163_d_b45: f64 = (eq72_e1161_d_b45 * (nv15 - nv7));
        let eq72_e1163_d_b46: f64 = (eq72_e1161_d_b46 * (nv15 - nv7));
        let eq72_e1163_d_b47: f64 = (eq72_e1161_d_b47 * (nv15 - nv7));
        let eq72_e1163_d_b48: f64 = (eq72_e1161_d_b48 * (nv15 - nv7));
        let eq72_e1163_d_b49: f64 = (eq72_e1161_d_b49 * (nv15 - nv7));
        let eq72_e1163_d_b50: f64 = (eq72_e1161_d_b50 * (nv15 - nv7));
        let eq72_e1163_d_b51: f64 = (eq72_e1161_d_b51 * (nv15 - nv7));
        let eq72_e1163_d_b52: f64 = (eq72_e1161_d_b52 * (nv15 - nv7));
        let eq72_e1163_d_b53: f64 = (eq72_e1161_d_b53 * (nv15 - nv7));
        let eq72_e1163_d_b54: f64 = (eq72_e1161_d_b54 * (nv15 - nv7));
        let eq72_e1164: f64 = (eq72_e1158 + eq72_e1163);
        let eq72_e1164_d_n0: f64 = (eq72_e1158_d_n0 + eq72_e1163_d_n0);
        let eq72_e1164_d_n1: f64 = (eq72_e1158_d_n1 + eq72_e1163_d_n1);
        let eq72_e1164_d_n2: f64 = (eq72_e1158_d_n2 + eq72_e1163_d_n2);
        let eq72_e1164_d_n3: f64 = (eq72_e1158_d_n3 + eq72_e1163_d_n3);
        let eq72_e1164_d_n4: f64 = (eq72_e1158_d_n4 + eq72_e1163_d_n4);
        let eq72_e1164_d_n5: f64 = (eq72_e1158_d_n5 + eq72_e1163_d_n5);
        let eq72_e1164_d_n6: f64 = (eq72_e1158_d_n6 + eq72_e1163_d_n6);
        let eq72_e1164_d_n7: f64 = (eq72_e1158_d_n7 + eq72_e1163_d_n7);
        let eq72_e1164_d_n8: f64 = (eq72_e1158_d_n8 + eq72_e1163_d_n8);
        let eq72_e1164_d_n9: f64 = (eq72_e1158_d_n9 + eq72_e1163_d_n9);
        let eq72_e1164_d_n10: f64 = (eq72_e1158_d_n10 + eq72_e1163_d_n10);
        let eq72_e1164_d_n11: f64 = (eq72_e1158_d_n11 + eq72_e1163_d_n11);
        let eq72_e1164_d_n12: f64 = (eq72_e1158_d_n12 + eq72_e1163_d_n12);
        let eq72_e1164_d_n13: f64 = (eq72_e1158_d_n13 + eq72_e1163_d_n13);
        let eq72_e1164_d_n14: f64 = (eq72_e1158_d_n14 + eq72_e1163_d_n14);
        let eq72_e1164_d_n15: f64 = (eq72_e1158_d_n15 + eq72_e1163_d_n15);
        let eq72_e1164_d_n16: f64 = (eq72_e1158_d_n16 + eq72_e1163_d_n16);
        let eq72_e1164_d_n17: f64 = (eq72_e1158_d_n17 + eq72_e1163_d_n17);
        let eq72_e1164_d_n18: f64 = (eq72_e1158_d_n18 + eq72_e1163_d_n18);
        let eq72_e1164_d_n19: f64 = (eq72_e1158_d_n19 + eq72_e1163_d_n19);
        let eq72_e1164_d_n20: f64 = (eq72_e1158_d_n20 + eq72_e1163_d_n20);
        let eq72_e1164_d_n21: f64 = (eq72_e1158_d_n21 + eq72_e1163_d_n21);
        let eq72_e1164_d_n22: f64 = (eq72_e1158_d_n22 + eq72_e1163_d_n22);
        let eq72_e1164_d_b0: f64 = (eq72_e1158_d_b0 + eq72_e1163_d_b0);
        let eq72_e1164_d_b1: f64 = (eq72_e1158_d_b1 + eq72_e1163_d_b1);
        let eq72_e1164_d_b2: f64 = (eq72_e1158_d_b2 + eq72_e1163_d_b2);
        let eq72_e1164_d_b3: f64 = (eq72_e1158_d_b3 + eq72_e1163_d_b3);
        let eq72_e1164_d_b4: f64 = (eq72_e1158_d_b4 + eq72_e1163_d_b4);
        let eq72_e1164_d_b5: f64 = (eq72_e1158_d_b5 + eq72_e1163_d_b5);
        let eq72_e1164_d_b6: f64 = (eq72_e1158_d_b6 + eq72_e1163_d_b6);
        let eq72_e1164_d_b7: f64 = (eq72_e1158_d_b7 + eq72_e1163_d_b7);
        let eq72_e1164_d_b8: f64 = (eq72_e1158_d_b8 + eq72_e1163_d_b8);
        let eq72_e1164_d_b9: f64 = (eq72_e1158_d_b9 + eq72_e1163_d_b9);
        let eq72_e1164_d_b10: f64 = (eq72_e1158_d_b10 + eq72_e1163_d_b10);
        let eq72_e1164_d_b11: f64 = (eq72_e1158_d_b11 + eq72_e1163_d_b11);
        let eq72_e1164_d_b12: f64 = (eq72_e1158_d_b12 + eq72_e1163_d_b12);
        let eq72_e1164_d_b13: f64 = (eq72_e1158_d_b13 + eq72_e1163_d_b13);
        let eq72_e1164_d_b14: f64 = (eq72_e1158_d_b14 + eq72_e1163_d_b14);
        let eq72_e1164_d_b15: f64 = (eq72_e1158_d_b15 + eq72_e1163_d_b15);
        let eq72_e1164_d_b16: f64 = (eq72_e1158_d_b16 + eq72_e1163_d_b16);
        let eq72_e1164_d_b17: f64 = (eq72_e1158_d_b17 + eq72_e1163_d_b17);
        let eq72_e1164_d_b18: f64 = (eq72_e1158_d_b18 + eq72_e1163_d_b18);
        let eq72_e1164_d_b19: f64 = (eq72_e1158_d_b19 + eq72_e1163_d_b19);
        let eq72_e1164_d_b20: f64 = (eq72_e1158_d_b20 + eq72_e1163_d_b20);
        let eq72_e1164_d_b21: f64 = (eq72_e1158_d_b21 + eq72_e1163_d_b21);
        let eq72_e1164_d_b22: f64 = (eq72_e1158_d_b22 + eq72_e1163_d_b22);
        let eq72_e1164_d_b23: f64 = (eq72_e1158_d_b23 + eq72_e1163_d_b23);
        let eq72_e1164_d_b24: f64 = (eq72_e1158_d_b24 + eq72_e1163_d_b24);
        let eq72_e1164_d_b25: f64 = (eq72_e1158_d_b25 + eq72_e1163_d_b25);
        let eq72_e1164_d_b26: f64 = (eq72_e1158_d_b26 + eq72_e1163_d_b26);
        let eq72_e1164_d_b27: f64 = (eq72_e1158_d_b27 + eq72_e1163_d_b27);
        let eq72_e1164_d_b28: f64 = (eq72_e1158_d_b28 + eq72_e1163_d_b28);
        let eq72_e1164_d_b29: f64 = (eq72_e1158_d_b29 + eq72_e1163_d_b29);
        let eq72_e1164_d_b30: f64 = (eq72_e1158_d_b30 + eq72_e1163_d_b30);
        let eq72_e1164_d_b31: f64 = (eq72_e1158_d_b31 + eq72_e1163_d_b31);
        let eq72_e1164_d_b32: f64 = (eq72_e1158_d_b32 + eq72_e1163_d_b32);
        let eq72_e1164_d_b33: f64 = (eq72_e1158_d_b33 + eq72_e1163_d_b33);
        let eq72_e1164_d_b34: f64 = (eq72_e1158_d_b34 + eq72_e1163_d_b34);
        let eq72_e1164_d_b35: f64 = (eq72_e1158_d_b35 + eq72_e1163_d_b35);
        let eq72_e1164_d_b36: f64 = (eq72_e1158_d_b36 + eq72_e1163_d_b36);
        let eq72_e1164_d_b37: f64 = (eq72_e1158_d_b37 + eq72_e1163_d_b37);
        let eq72_e1164_d_b38: f64 = (eq72_e1158_d_b38 + eq72_e1163_d_b38);
        let eq72_e1164_d_b39: f64 = (eq72_e1158_d_b39 + eq72_e1163_d_b39);
        let eq72_e1164_d_b40: f64 = (eq72_e1158_d_b40 + eq72_e1163_d_b40);
        let eq72_e1164_d_b41: f64 = (eq72_e1158_d_b41 + eq72_e1163_d_b41);
        let eq72_e1164_d_b42: f64 = (eq72_e1158_d_b42 + eq72_e1163_d_b42);
        let eq72_e1164_d_b43: f64 = (eq72_e1158_d_b43 + eq72_e1163_d_b43);
        let eq72_e1164_d_b44: f64 = (eq72_e1158_d_b44 + eq72_e1163_d_b44);
        let eq72_e1164_d_b45: f64 = (eq72_e1158_d_b45 + eq72_e1163_d_b45);
        let eq72_e1164_d_b46: f64 = (eq72_e1158_d_b46 + eq72_e1163_d_b46);
        let eq72_e1164_d_b47: f64 = (eq72_e1158_d_b47 + eq72_e1163_d_b47);
        let eq72_e1164_d_b48: f64 = (eq72_e1158_d_b48 + eq72_e1163_d_b48);
        let eq72_e1164_d_b49: f64 = (eq72_e1158_d_b49 + eq72_e1163_d_b49);
        let eq72_e1164_d_b50: f64 = (eq72_e1158_d_b50 + eq72_e1163_d_b50);
        let eq72_e1164_d_b51: f64 = (eq72_e1158_d_b51 + eq72_e1163_d_b51);
        let eq72_e1164_d_b52: f64 = (eq72_e1158_d_b52 + eq72_e1163_d_b52);
        let eq72_e1164_d_b53: f64 = (eq72_e1158_d_b53 + eq72_e1163_d_b53);
        let eq72_e1164_d_b54: f64 = (eq72_e1158_d_b54 + eq72_e1163_d_b54);
        (eq72_e1164, eq72_e1164_d_n0, eq72_e1164_d_n1, eq72_e1164_d_n2, eq72_e1164_d_n3, eq72_e1164_d_n4, eq72_e1164_d_n5, eq72_e1164_d_n6, eq72_e1164_d_n7, eq72_e1164_d_n8, eq72_e1164_d_n9, eq72_e1164_d_n10, eq72_e1164_d_n11, eq72_e1164_d_n12, eq72_e1164_d_n13, eq72_e1164_d_n14, eq72_e1164_d_n15, eq72_e1164_d_n16, eq72_e1164_d_n17, eq72_e1164_d_n18, eq72_e1164_d_n19, eq72_e1164_d_n20, eq72_e1164_d_n21, eq72_e1164_d_n22, eq72_e1164_d_b0, eq72_e1164_d_b1, eq72_e1164_d_b2, eq72_e1164_d_b3, eq72_e1164_d_b4, eq72_e1164_d_b5, eq72_e1164_d_b6, eq72_e1164_d_b7, eq72_e1164_d_b8, eq72_e1164_d_b9, eq72_e1164_d_b10, eq72_e1164_d_b11, eq72_e1164_d_b12, eq72_e1164_d_b13, eq72_e1164_d_b14, eq72_e1164_d_b15, eq72_e1164_d_b16, eq72_e1164_d_b17, eq72_e1164_d_b18, eq72_e1164_d_b19, eq72_e1164_d_b20, eq72_e1164_d_b21, eq72_e1164_d_b22, eq72_e1164_d_b23, eq72_e1164_d_b24, eq72_e1164_d_b25, eq72_e1164_d_b26, eq72_e1164_d_b27, eq72_e1164_d_b28, eq72_e1164_d_b29, eq72_e1164_d_b30, eq72_e1164_d_b31, eq72_e1164_d_b32, eq72_e1164_d_b33, eq72_e1164_d_b34, eq72_e1164_d_b35, eq72_e1164_d_b36, eq72_e1164_d_b37, eq72_e1164_d_b38, eq72_e1164_d_b39, eq72_e1164_d_b40, eq72_e1164_d_b41, eq72_e1164_d_b42, eq72_e1164_d_b43, eq72_e1164_d_b44, eq72_e1164_d_b45, eq72_e1164_d_b46, eq72_e1164_d_b47, eq72_e1164_d_b48, eq72_e1164_d_b49, eq72_e1164_d_b50, eq72_e1164_d_b51, eq72_e1164_d_b52, eq72_e1164_d_b53, eq72_e1164_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1166;
        let eq72_node_derivatives: [f64; 23] = [eq72_e1166_d_n0, eq72_e1166_d_n1, eq72_e1166_d_n2, eq72_e1166_d_n3, eq72_e1166_d_n4, eq72_e1166_d_n5, eq72_e1166_d_n6, eq72_e1166_d_n7, eq72_e1166_d_n8, eq72_e1166_d_n9, eq72_e1166_d_n10, eq72_e1166_d_n11, eq72_e1166_d_n12, eq72_e1166_d_n13, eq72_e1166_d_n14, eq72_e1166_d_n15, eq72_e1166_d_n16, eq72_e1166_d_n17, eq72_e1166_d_n18, eq72_e1166_d_n19, eq72_e1166_d_n20, eq72_e1166_d_n21, eq72_e1166_d_n22];
        let eq72_branch_derivatives: [f64; 55] = [eq72_e1166_d_b0, eq72_e1166_d_b1, eq72_e1166_d_b2, eq72_e1166_d_b3, eq72_e1166_d_b4, eq72_e1166_d_b5, eq72_e1166_d_b6, eq72_e1166_d_b7, eq72_e1166_d_b8, eq72_e1166_d_b9, eq72_e1166_d_b10, eq72_e1166_d_b11, eq72_e1166_d_b12, eq72_e1166_d_b13, eq72_e1166_d_b14, eq72_e1166_d_b15, eq72_e1166_d_b16, eq72_e1166_d_b17, eq72_e1166_d_b18, eq72_e1166_d_b19, eq72_e1166_d_b20, eq72_e1166_d_b21, eq72_e1166_d_b22, eq72_e1166_d_b23, eq72_e1166_d_b24, eq72_e1166_d_b25, eq72_e1166_d_b26, eq72_e1166_d_b27, eq72_e1166_d_b28, eq72_e1166_d_b29, eq72_e1166_d_b30, eq72_e1166_d_b31, eq72_e1166_d_b32, eq72_e1166_d_b33, eq72_e1166_d_b34, eq72_e1166_d_b35, eq72_e1166_d_b36, eq72_e1166_d_b37, eq72_e1166_d_b38, eq72_e1166_d_b39, eq72_e1166_d_b40, eq72_e1166_d_b41, eq72_e1166_d_b42, eq72_e1166_d_b43, eq72_e1166_d_b44, eq72_e1166_d_b45, eq72_e1166_d_b46, eq72_e1166_d_b47, eq72_e1166_d_b48, eq72_e1166_d_b49, eq72_e1166_d_b50, eq72_e1166_d_b51, eq72_e1166_d_b52, eq72_e1166_d_b53, eq72_e1166_d_b54];
        stamper.stamp_current_dense_local(
            Some(15),
            Some(7),
            multiplicity * (eq72_value),
            &eq72_node_derivatives,
            &eq72_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1173,) = {
    if (s.b[433] && (!s.b[434])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e1173;
        stamper.stamp_potential_const_local(
            33,
            eq73_value,
        );
        let (eq74_e1178,) = {
    if (!s.b[433]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e1178;
        stamper.stamp_potential_const_local(
            34,
            eq74_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_13(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let (eq75_e1194, eq75_e1194_d_n0, eq75_e1194_d_n1, eq75_e1194_d_n2, eq75_e1194_d_n3, eq75_e1194_d_n4, eq75_e1194_d_n5, eq75_e1194_d_n6, eq75_e1194_d_n7, eq75_e1194_d_n8, eq75_e1194_d_n9, eq75_e1194_d_n10, eq75_e1194_d_n11, eq75_e1194_d_n12, eq75_e1194_d_n13, eq75_e1194_d_n14, eq75_e1194_d_n15, eq75_e1194_d_n16, eq75_e1194_d_n17, eq75_e1194_d_n18, eq75_e1194_d_n19, eq75_e1194_d_n20, eq75_e1194_d_n21, eq75_e1194_d_n22, eq75_e1194_d_b0, eq75_e1194_d_b1, eq75_e1194_d_b2, eq75_e1194_d_b3, eq75_e1194_d_b4, eq75_e1194_d_b5, eq75_e1194_d_b6, eq75_e1194_d_b7, eq75_e1194_d_b8, eq75_e1194_d_b9, eq75_e1194_d_b10, eq75_e1194_d_b11, eq75_e1194_d_b12, eq75_e1194_d_b13, eq75_e1194_d_b14, eq75_e1194_d_b15, eq75_e1194_d_b16, eq75_e1194_d_b17, eq75_e1194_d_b18, eq75_e1194_d_b19, eq75_e1194_d_b20, eq75_e1194_d_b21, eq75_e1194_d_b22, eq75_e1194_d_b23, eq75_e1194_d_b24, eq75_e1194_d_b25, eq75_e1194_d_b26, eq75_e1194_d_b27, eq75_e1194_d_b28, eq75_e1194_d_b29, eq75_e1194_d_b30, eq75_e1194_d_b31, eq75_e1194_d_b32, eq75_e1194_d_b33, eq75_e1194_d_b34, eq75_e1194_d_b35, eq75_e1194_d_b36, eq75_e1194_d_b37, eq75_e1194_d_b38, eq75_e1194_d_b39, eq75_e1194_d_b40, eq75_e1194_d_b41, eq75_e1194_d_b42, eq75_e1194_d_b43, eq75_e1194_d_b44, eq75_e1194_d_b45, eq75_e1194_d_b46, eq75_e1194_d_b47, eq75_e1194_d_b48, eq75_e1194_d_b49, eq75_e1194_d_b50, eq75_e1194_d_b51, eq75_e1194_d_b52, eq75_e1194_d_b53, eq75_e1194_d_b54,) = {
    if (s.b[448] && s.b[449]) {
        let eq75_e1184: f64 = (p.p6 * s.v[52]);
        let eq75_e1184_d_n0: f64 = (p.p6 * s.dn[52][0]);
        let eq75_e1184_d_n1: f64 = (p.p6 * s.dn[52][1]);
        let eq75_e1184_d_n2: f64 = (p.p6 * s.dn[52][2]);
        let eq75_e1184_d_n3: f64 = (p.p6 * s.dn[52][3]);
        let eq75_e1184_d_n4: f64 = (p.p6 * s.dn[52][4]);
        let eq75_e1184_d_n5: f64 = (p.p6 * s.dn[52][5]);
        let eq75_e1184_d_n6: f64 = (p.p6 * s.dn[52][6]);
        let eq75_e1184_d_n7: f64 = (p.p6 * s.dn[52][7]);
        let eq75_e1184_d_n8: f64 = (p.p6 * s.dn[52][8]);
        let eq75_e1184_d_n9: f64 = (p.p6 * s.dn[52][9]);
        let eq75_e1184_d_n10: f64 = (p.p6 * s.dn[52][10]);
        let eq75_e1184_d_n11: f64 = (p.p6 * s.dn[52][11]);
        let eq75_e1184_d_n12: f64 = (p.p6 * s.dn[52][12]);
        let eq75_e1184_d_n13: f64 = (p.p6 * s.dn[52][13]);
        let eq75_e1184_d_n14: f64 = (p.p6 * s.dn[52][14]);
        let eq75_e1184_d_n15: f64 = (p.p6 * s.dn[52][15]);
        let eq75_e1184_d_n16: f64 = (p.p6 * s.dn[52][16]);
        let eq75_e1184_d_n17: f64 = (p.p6 * s.dn[52][17]);
        let eq75_e1184_d_n18: f64 = (p.p6 * s.dn[52][18]);
        let eq75_e1184_d_n19: f64 = (p.p6 * s.dn[52][19]);
        let eq75_e1184_d_n20: f64 = (p.p6 * s.dn[52][20]);
        let eq75_e1184_d_n21: f64 = (p.p6 * s.dn[52][21]);
        let eq75_e1184_d_n22: f64 = (p.p6 * s.dn[52][22]);
        let eq75_e1184_d_b0: f64 = (p.p6 * s.db[52][0]);
        let eq75_e1184_d_b1: f64 = (p.p6 * s.db[52][1]);
        let eq75_e1184_d_b2: f64 = (p.p6 * s.db[52][2]);
        let eq75_e1184_d_b3: f64 = (p.p6 * s.db[52][3]);
        let eq75_e1184_d_b4: f64 = (p.p6 * s.db[52][4]);
        let eq75_e1184_d_b5: f64 = (p.p6 * s.db[52][5]);
        let eq75_e1184_d_b6: f64 = (p.p6 * s.db[52][6]);
        let eq75_e1184_d_b7: f64 = (p.p6 * s.db[52][7]);
        let eq75_e1184_d_b8: f64 = (p.p6 * s.db[52][8]);
        let eq75_e1184_d_b9: f64 = (p.p6 * s.db[52][9]);
        let eq75_e1184_d_b10: f64 = (p.p6 * s.db[52][10]);
        let eq75_e1184_d_b11: f64 = (p.p6 * s.db[52][11]);
        let eq75_e1184_d_b12: f64 = (p.p6 * s.db[52][12]);
        let eq75_e1184_d_b13: f64 = (p.p6 * s.db[52][13]);
        let eq75_e1184_d_b14: f64 = (p.p6 * s.db[52][14]);
        let eq75_e1184_d_b15: f64 = (p.p6 * s.db[52][15]);
        let eq75_e1184_d_b16: f64 = (p.p6 * s.db[52][16]);
        let eq75_e1184_d_b17: f64 = (p.p6 * s.db[52][17]);
        let eq75_e1184_d_b18: f64 = (p.p6 * s.db[52][18]);
        let eq75_e1184_d_b19: f64 = (p.p6 * s.db[52][19]);
        let eq75_e1184_d_b20: f64 = (p.p6 * s.db[52][20]);
        let eq75_e1184_d_b21: f64 = (p.p6 * s.db[52][21]);
        let eq75_e1184_d_b22: f64 = (p.p6 * s.db[52][22]);
        let eq75_e1184_d_b23: f64 = (p.p6 * s.db[52][23]);
        let eq75_e1184_d_b24: f64 = (p.p6 * s.db[52][24]);
        let eq75_e1184_d_b25: f64 = (p.p6 * s.db[52][25]);
        let eq75_e1184_d_b26: f64 = (p.p6 * s.db[52][26]);
        let eq75_e1184_d_b27: f64 = (p.p6 * s.db[52][27]);
        let eq75_e1184_d_b28: f64 = (p.p6 * s.db[52][28]);
        let eq75_e1184_d_b29: f64 = (p.p6 * s.db[52][29]);
        let eq75_e1184_d_b30: f64 = (p.p6 * s.db[52][30]);
        let eq75_e1184_d_b31: f64 = (p.p6 * s.db[52][31]);
        let eq75_e1184_d_b32: f64 = (p.p6 * s.db[52][32]);
        let eq75_e1184_d_b33: f64 = (p.p6 * s.db[52][33]);
        let eq75_e1184_d_b34: f64 = (p.p6 * s.db[52][34]);
        let eq75_e1184_d_b35: f64 = (p.p6 * s.db[52][35]);
        let eq75_e1184_d_b36: f64 = (p.p6 * s.db[52][36]);
        let eq75_e1184_d_b37: f64 = (p.p6 * s.db[52][37]);
        let eq75_e1184_d_b38: f64 = (p.p6 * s.db[52][38]);
        let eq75_e1184_d_b39: f64 = (p.p6 * s.db[52][39]);
        let eq75_e1184_d_b40: f64 = (p.p6 * s.db[52][40]);
        let eq75_e1184_d_b41: f64 = (p.p6 * s.db[52][41]);
        let eq75_e1184_d_b42: f64 = (p.p6 * s.db[52][42]);
        let eq75_e1184_d_b43: f64 = (p.p6 * s.db[52][43]);
        let eq75_e1184_d_b44: f64 = (p.p6 * s.db[52][44]);
        let eq75_e1184_d_b45: f64 = (p.p6 * s.db[52][45]);
        let eq75_e1184_d_b46: f64 = (p.p6 * s.db[52][46]);
        let eq75_e1184_d_b47: f64 = (p.p6 * s.db[52][47]);
        let eq75_e1184_d_b48: f64 = (p.p6 * s.db[52][48]);
        let eq75_e1184_d_b49: f64 = (p.p6 * s.db[52][49]);
        let eq75_e1184_d_b50: f64 = (p.p6 * s.db[52][50]);
        let eq75_e1184_d_b51: f64 = (p.p6 * s.db[52][51]);
        let eq75_e1184_d_b52: f64 = (p.p6 * s.db[52][52]);
        let eq75_e1184_d_b53: f64 = (p.p6 * s.db[52][53]);
        let eq75_e1184_d_b54: f64 = (p.p6 * s.db[52][54]);
        let eq75_e1186: f64 = (eq75_e1184 * s.v[245]);
        let eq75_e1186_d_n0: f64 = ((eq75_e1184_d_n0 * s.v[245]) + (eq75_e1184 * s.dn[245][0]));
        let eq75_e1186_d_n1: f64 = ((eq75_e1184_d_n1 * s.v[245]) + (eq75_e1184 * s.dn[245][1]));
        let eq75_e1186_d_n2: f64 = ((eq75_e1184_d_n2 * s.v[245]) + (eq75_e1184 * s.dn[245][2]));
        let eq75_e1186_d_n3: f64 = ((eq75_e1184_d_n3 * s.v[245]) + (eq75_e1184 * s.dn[245][3]));
        let eq75_e1186_d_n4: f64 = ((eq75_e1184_d_n4 * s.v[245]) + (eq75_e1184 * s.dn[245][4]));
        let eq75_e1186_d_n5: f64 = ((eq75_e1184_d_n5 * s.v[245]) + (eq75_e1184 * s.dn[245][5]));
        let eq75_e1186_d_n6: f64 = ((eq75_e1184_d_n6 * s.v[245]) + (eq75_e1184 * s.dn[245][6]));
        let eq75_e1186_d_n7: f64 = ((eq75_e1184_d_n7 * s.v[245]) + (eq75_e1184 * s.dn[245][7]));
        let eq75_e1186_d_n8: f64 = ((eq75_e1184_d_n8 * s.v[245]) + (eq75_e1184 * s.dn[245][8]));
        let eq75_e1186_d_n9: f64 = ((eq75_e1184_d_n9 * s.v[245]) + (eq75_e1184 * s.dn[245][9]));
        let eq75_e1186_d_n10: f64 = ((eq75_e1184_d_n10 * s.v[245]) + (eq75_e1184 * s.dn[245][10]));
        let eq75_e1186_d_n11: f64 = ((eq75_e1184_d_n11 * s.v[245]) + (eq75_e1184 * s.dn[245][11]));
        let eq75_e1186_d_n12: f64 = ((eq75_e1184_d_n12 * s.v[245]) + (eq75_e1184 * s.dn[245][12]));
        let eq75_e1186_d_n13: f64 = ((eq75_e1184_d_n13 * s.v[245]) + (eq75_e1184 * s.dn[245][13]));
        let eq75_e1186_d_n14: f64 = ((eq75_e1184_d_n14 * s.v[245]) + (eq75_e1184 * s.dn[245][14]));
        let eq75_e1186_d_n15: f64 = ((eq75_e1184_d_n15 * s.v[245]) + (eq75_e1184 * s.dn[245][15]));
        let eq75_e1186_d_n16: f64 = ((eq75_e1184_d_n16 * s.v[245]) + (eq75_e1184 * s.dn[245][16]));
        let eq75_e1186_d_n17: f64 = ((eq75_e1184_d_n17 * s.v[245]) + (eq75_e1184 * s.dn[245][17]));
        let eq75_e1186_d_n18: f64 = ((eq75_e1184_d_n18 * s.v[245]) + (eq75_e1184 * s.dn[245][18]));
        let eq75_e1186_d_n19: f64 = ((eq75_e1184_d_n19 * s.v[245]) + (eq75_e1184 * s.dn[245][19]));
        let eq75_e1186_d_n20: f64 = ((eq75_e1184_d_n20 * s.v[245]) + (eq75_e1184 * s.dn[245][20]));
        let eq75_e1186_d_n21: f64 = ((eq75_e1184_d_n21 * s.v[245]) + (eq75_e1184 * s.dn[245][21]));
        let eq75_e1186_d_n22: f64 = ((eq75_e1184_d_n22 * s.v[245]) + (eq75_e1184 * s.dn[245][22]));
        let eq75_e1186_d_b0: f64 = ((eq75_e1184_d_b0 * s.v[245]) + (eq75_e1184 * s.db[245][0]));
        let eq75_e1186_d_b1: f64 = ((eq75_e1184_d_b1 * s.v[245]) + (eq75_e1184 * s.db[245][1]));
        let eq75_e1186_d_b2: f64 = ((eq75_e1184_d_b2 * s.v[245]) + (eq75_e1184 * s.db[245][2]));
        let eq75_e1186_d_b3: f64 = ((eq75_e1184_d_b3 * s.v[245]) + (eq75_e1184 * s.db[245][3]));
        let eq75_e1186_d_b4: f64 = ((eq75_e1184_d_b4 * s.v[245]) + (eq75_e1184 * s.db[245][4]));
        let eq75_e1186_d_b5: f64 = ((eq75_e1184_d_b5 * s.v[245]) + (eq75_e1184 * s.db[245][5]));
        let eq75_e1186_d_b6: f64 = ((eq75_e1184_d_b6 * s.v[245]) + (eq75_e1184 * s.db[245][6]));
        let eq75_e1186_d_b7: f64 = ((eq75_e1184_d_b7 * s.v[245]) + (eq75_e1184 * s.db[245][7]));
        let eq75_e1186_d_b8: f64 = ((eq75_e1184_d_b8 * s.v[245]) + (eq75_e1184 * s.db[245][8]));
        let eq75_e1186_d_b9: f64 = ((eq75_e1184_d_b9 * s.v[245]) + (eq75_e1184 * s.db[245][9]));
        let eq75_e1186_d_b10: f64 = ((eq75_e1184_d_b10 * s.v[245]) + (eq75_e1184 * s.db[245][10]));
        let eq75_e1186_d_b11: f64 = ((eq75_e1184_d_b11 * s.v[245]) + (eq75_e1184 * s.db[245][11]));
        let eq75_e1186_d_b12: f64 = ((eq75_e1184_d_b12 * s.v[245]) + (eq75_e1184 * s.db[245][12]));
        let eq75_e1186_d_b13: f64 = ((eq75_e1184_d_b13 * s.v[245]) + (eq75_e1184 * s.db[245][13]));
        let eq75_e1186_d_b14: f64 = ((eq75_e1184_d_b14 * s.v[245]) + (eq75_e1184 * s.db[245][14]));
        let eq75_e1186_d_b15: f64 = ((eq75_e1184_d_b15 * s.v[245]) + (eq75_e1184 * s.db[245][15]));
        let eq75_e1186_d_b16: f64 = ((eq75_e1184_d_b16 * s.v[245]) + (eq75_e1184 * s.db[245][16]));
        let eq75_e1186_d_b17: f64 = ((eq75_e1184_d_b17 * s.v[245]) + (eq75_e1184 * s.db[245][17]));
        let eq75_e1186_d_b18: f64 = ((eq75_e1184_d_b18 * s.v[245]) + (eq75_e1184 * s.db[245][18]));
        let eq75_e1186_d_b19: f64 = ((eq75_e1184_d_b19 * s.v[245]) + (eq75_e1184 * s.db[245][19]));
        let eq75_e1186_d_b20: f64 = ((eq75_e1184_d_b20 * s.v[245]) + (eq75_e1184 * s.db[245][20]));
        let eq75_e1186_d_b21: f64 = ((eq75_e1184_d_b21 * s.v[245]) + (eq75_e1184 * s.db[245][21]));
        let eq75_e1186_d_b22: f64 = ((eq75_e1184_d_b22 * s.v[245]) + (eq75_e1184 * s.db[245][22]));
        let eq75_e1186_d_b23: f64 = ((eq75_e1184_d_b23 * s.v[245]) + (eq75_e1184 * s.db[245][23]));
        let eq75_e1186_d_b24: f64 = ((eq75_e1184_d_b24 * s.v[245]) + (eq75_e1184 * s.db[245][24]));
        let eq75_e1186_d_b25: f64 = ((eq75_e1184_d_b25 * s.v[245]) + (eq75_e1184 * s.db[245][25]));
        let eq75_e1186_d_b26: f64 = ((eq75_e1184_d_b26 * s.v[245]) + (eq75_e1184 * s.db[245][26]));
        let eq75_e1186_d_b27: f64 = ((eq75_e1184_d_b27 * s.v[245]) + (eq75_e1184 * s.db[245][27]));
        let eq75_e1186_d_b28: f64 = ((eq75_e1184_d_b28 * s.v[245]) + (eq75_e1184 * s.db[245][28]));
        let eq75_e1186_d_b29: f64 = ((eq75_e1184_d_b29 * s.v[245]) + (eq75_e1184 * s.db[245][29]));
        let eq75_e1186_d_b30: f64 = ((eq75_e1184_d_b30 * s.v[245]) + (eq75_e1184 * s.db[245][30]));
        let eq75_e1186_d_b31: f64 = ((eq75_e1184_d_b31 * s.v[245]) + (eq75_e1184 * s.db[245][31]));
        let eq75_e1186_d_b32: f64 = ((eq75_e1184_d_b32 * s.v[245]) + (eq75_e1184 * s.db[245][32]));
        let eq75_e1186_d_b33: f64 = ((eq75_e1184_d_b33 * s.v[245]) + (eq75_e1184 * s.db[245][33]));
        let eq75_e1186_d_b34: f64 = ((eq75_e1184_d_b34 * s.v[245]) + (eq75_e1184 * s.db[245][34]));
        let eq75_e1186_d_b35: f64 = ((eq75_e1184_d_b35 * s.v[245]) + (eq75_e1184 * s.db[245][35]));
        let eq75_e1186_d_b36: f64 = ((eq75_e1184_d_b36 * s.v[245]) + (eq75_e1184 * s.db[245][36]));
        let eq75_e1186_d_b37: f64 = ((eq75_e1184_d_b37 * s.v[245]) + (eq75_e1184 * s.db[245][37]));
        let eq75_e1186_d_b38: f64 = ((eq75_e1184_d_b38 * s.v[245]) + (eq75_e1184 * s.db[245][38]));
        let eq75_e1186_d_b39: f64 = ((eq75_e1184_d_b39 * s.v[245]) + (eq75_e1184 * s.db[245][39]));
        let eq75_e1186_d_b40: f64 = ((eq75_e1184_d_b40 * s.v[245]) + (eq75_e1184 * s.db[245][40]));
        let eq75_e1186_d_b41: f64 = ((eq75_e1184_d_b41 * s.v[245]) + (eq75_e1184 * s.db[245][41]));
        let eq75_e1186_d_b42: f64 = ((eq75_e1184_d_b42 * s.v[245]) + (eq75_e1184 * s.db[245][42]));
        let eq75_e1186_d_b43: f64 = ((eq75_e1184_d_b43 * s.v[245]) + (eq75_e1184 * s.db[245][43]));
        let eq75_e1186_d_b44: f64 = ((eq75_e1184_d_b44 * s.v[245]) + (eq75_e1184 * s.db[245][44]));
        let eq75_e1186_d_b45: f64 = ((eq75_e1184_d_b45 * s.v[245]) + (eq75_e1184 * s.db[245][45]));
        let eq75_e1186_d_b46: f64 = ((eq75_e1184_d_b46 * s.v[245]) + (eq75_e1184 * s.db[245][46]));
        let eq75_e1186_d_b47: f64 = ((eq75_e1184_d_b47 * s.v[245]) + (eq75_e1184 * s.db[245][47]));
        let eq75_e1186_d_b48: f64 = ((eq75_e1184_d_b48 * s.v[245]) + (eq75_e1184 * s.db[245][48]));
        let eq75_e1186_d_b49: f64 = ((eq75_e1184_d_b49 * s.v[245]) + (eq75_e1184 * s.db[245][49]));
        let eq75_e1186_d_b50: f64 = ((eq75_e1184_d_b50 * s.v[245]) + (eq75_e1184 * s.db[245][50]));
        let eq75_e1186_d_b51: f64 = ((eq75_e1184_d_b51 * s.v[245]) + (eq75_e1184 * s.db[245][51]));
        let eq75_e1186_d_b52: f64 = ((eq75_e1184_d_b52 * s.v[245]) + (eq75_e1184 * s.db[245][52]));
        let eq75_e1186_d_b53: f64 = ((eq75_e1184_d_b53 * s.v[245]) + (eq75_e1184 * s.db[245][53]));
        let eq75_e1186_d_b54: f64 = ((eq75_e1184_d_b54 * s.v[245]) + (eq75_e1184 * s.db[245][54]));
        let eq75_e1189: f64 = (p.p6 * s.v[379]);
        let eq75_e1189_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq75_e1189_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq75_e1189_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq75_e1189_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq75_e1189_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq75_e1189_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq75_e1189_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq75_e1189_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq75_e1189_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq75_e1189_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq75_e1189_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq75_e1189_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq75_e1189_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq75_e1189_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq75_e1189_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq75_e1189_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq75_e1189_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq75_e1189_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq75_e1189_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq75_e1189_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq75_e1189_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq75_e1189_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq75_e1189_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq75_e1189_d_b0: f64 = (p.p6 * s.db[379][0]);
        let eq75_e1189_d_b1: f64 = (p.p6 * s.db[379][1]);
        let eq75_e1189_d_b2: f64 = (p.p6 * s.db[379][2]);
        let eq75_e1189_d_b3: f64 = (p.p6 * s.db[379][3]);
        let eq75_e1189_d_b4: f64 = (p.p6 * s.db[379][4]);
        let eq75_e1189_d_b5: f64 = (p.p6 * s.db[379][5]);
        let eq75_e1189_d_b6: f64 = (p.p6 * s.db[379][6]);
        let eq75_e1189_d_b7: f64 = (p.p6 * s.db[379][7]);
        let eq75_e1189_d_b8: f64 = (p.p6 * s.db[379][8]);
        let eq75_e1189_d_b9: f64 = (p.p6 * s.db[379][9]);
        let eq75_e1189_d_b10: f64 = (p.p6 * s.db[379][10]);
        let eq75_e1189_d_b11: f64 = (p.p6 * s.db[379][11]);
        let eq75_e1189_d_b12: f64 = (p.p6 * s.db[379][12]);
        let eq75_e1189_d_b13: f64 = (p.p6 * s.db[379][13]);
        let eq75_e1189_d_b14: f64 = (p.p6 * s.db[379][14]);
        let eq75_e1189_d_b15: f64 = (p.p6 * s.db[379][15]);
        let eq75_e1189_d_b16: f64 = (p.p6 * s.db[379][16]);
        let eq75_e1189_d_b17: f64 = (p.p6 * s.db[379][17]);
        let eq75_e1189_d_b18: f64 = (p.p6 * s.db[379][18]);
        let eq75_e1189_d_b19: f64 = (p.p6 * s.db[379][19]);
        let eq75_e1189_d_b20: f64 = (p.p6 * s.db[379][20]);
        let eq75_e1189_d_b21: f64 = (p.p6 * s.db[379][21]);
        let eq75_e1189_d_b22: f64 = (p.p6 * s.db[379][22]);
        let eq75_e1189_d_b23: f64 = (p.p6 * s.db[379][23]);
        let eq75_e1189_d_b24: f64 = (p.p6 * s.db[379][24]);
        let eq75_e1189_d_b25: f64 = (p.p6 * s.db[379][25]);
        let eq75_e1189_d_b26: f64 = (p.p6 * s.db[379][26]);
        let eq75_e1189_d_b27: f64 = (p.p6 * s.db[379][27]);
        let eq75_e1189_d_b28: f64 = (p.p6 * s.db[379][28]);
        let eq75_e1189_d_b29: f64 = (p.p6 * s.db[379][29]);
        let eq75_e1189_d_b30: f64 = (p.p6 * s.db[379][30]);
        let eq75_e1189_d_b31: f64 = (p.p6 * s.db[379][31]);
        let eq75_e1189_d_b32: f64 = (p.p6 * s.db[379][32]);
        let eq75_e1189_d_b33: f64 = (p.p6 * s.db[379][33]);
        let eq75_e1189_d_b34: f64 = (p.p6 * s.db[379][34]);
        let eq75_e1189_d_b35: f64 = (p.p6 * s.db[379][35]);
        let eq75_e1189_d_b36: f64 = (p.p6 * s.db[379][36]);
        let eq75_e1189_d_b37: f64 = (p.p6 * s.db[379][37]);
        let eq75_e1189_d_b38: f64 = (p.p6 * s.db[379][38]);
        let eq75_e1189_d_b39: f64 = (p.p6 * s.db[379][39]);
        let eq75_e1189_d_b40: f64 = (p.p6 * s.db[379][40]);
        let eq75_e1189_d_b41: f64 = (p.p6 * s.db[379][41]);
        let eq75_e1189_d_b42: f64 = (p.p6 * s.db[379][42]);
        let eq75_e1189_d_b43: f64 = (p.p6 * s.db[379][43]);
        let eq75_e1189_d_b44: f64 = (p.p6 * s.db[379][44]);
        let eq75_e1189_d_b45: f64 = (p.p6 * s.db[379][45]);
        let eq75_e1189_d_b46: f64 = (p.p6 * s.db[379][46]);
        let eq75_e1189_d_b47: f64 = (p.p6 * s.db[379][47]);
        let eq75_e1189_d_b48: f64 = (p.p6 * s.db[379][48]);
        let eq75_e1189_d_b49: f64 = (p.p6 * s.db[379][49]);
        let eq75_e1189_d_b50: f64 = (p.p6 * s.db[379][50]);
        let eq75_e1189_d_b51: f64 = (p.p6 * s.db[379][51]);
        let eq75_e1189_d_b52: f64 = (p.p6 * s.db[379][52]);
        let eq75_e1189_d_b53: f64 = (p.p6 * s.db[379][53]);
        let eq75_e1189_d_b54: f64 = (p.p6 * s.db[379][54]);
        let eq75_e1191: f64 = (eq75_e1189 * (nv8 - nv19));
        let eq75_e1191_d_n0: f64 = (eq75_e1189_d_n0 * (nv8 - nv19));
        let eq75_e1191_d_n1: f64 = (eq75_e1189_d_n1 * (nv8 - nv19));
        let eq75_e1191_d_n2: f64 = (eq75_e1189_d_n2 * (nv8 - nv19));
        let eq75_e1191_d_n3: f64 = (eq75_e1189_d_n3 * (nv8 - nv19));
        let eq75_e1191_d_n4: f64 = (eq75_e1189_d_n4 * (nv8 - nv19));
        let eq75_e1191_d_n5: f64 = (eq75_e1189_d_n5 * (nv8 - nv19));
        let eq75_e1191_d_n6: f64 = (eq75_e1189_d_n6 * (nv8 - nv19));
        let eq75_e1191_d_n7: f64 = (eq75_e1189_d_n7 * (nv8 - nv19));
        let eq75_e1191_d_n8: f64 = ((eq75_e1189_d_n8 * (nv8 - nv19)) + eq75_e1189);
        let eq75_e1191_d_n9: f64 = (eq75_e1189_d_n9 * (nv8 - nv19));
        let eq75_e1191_d_n10: f64 = (eq75_e1189_d_n10 * (nv8 - nv19));
        let eq75_e1191_d_n11: f64 = (eq75_e1189_d_n11 * (nv8 - nv19));
        let eq75_e1191_d_n12: f64 = (eq75_e1189_d_n12 * (nv8 - nv19));
        let eq75_e1191_d_n13: f64 = (eq75_e1189_d_n13 * (nv8 - nv19));
        let eq75_e1191_d_n14: f64 = (eq75_e1189_d_n14 * (nv8 - nv19));
        let eq75_e1191_d_n15: f64 = (eq75_e1189_d_n15 * (nv8 - nv19));
        let eq75_e1191_d_n16: f64 = (eq75_e1189_d_n16 * (nv8 - nv19));
        let eq75_e1191_d_n17: f64 = (eq75_e1189_d_n17 * (nv8 - nv19));
        let eq75_e1191_d_n18: f64 = (eq75_e1189_d_n18 * (nv8 - nv19));
        let eq75_e1191_d_n19: f64 = ((eq75_e1189_d_n19 * (nv8 - nv19)) + (-eq75_e1189));
        let eq75_e1191_d_n20: f64 = (eq75_e1189_d_n20 * (nv8 - nv19));
        let eq75_e1191_d_n21: f64 = (eq75_e1189_d_n21 * (nv8 - nv19));
        let eq75_e1191_d_n22: f64 = (eq75_e1189_d_n22 * (nv8 - nv19));
        let eq75_e1191_d_b0: f64 = (eq75_e1189_d_b0 * (nv8 - nv19));
        let eq75_e1191_d_b1: f64 = (eq75_e1189_d_b1 * (nv8 - nv19));
        let eq75_e1191_d_b2: f64 = (eq75_e1189_d_b2 * (nv8 - nv19));
        let eq75_e1191_d_b3: f64 = (eq75_e1189_d_b3 * (nv8 - nv19));
        let eq75_e1191_d_b4: f64 = (eq75_e1189_d_b4 * (nv8 - nv19));
        let eq75_e1191_d_b5: f64 = (eq75_e1189_d_b5 * (nv8 - nv19));
        let eq75_e1191_d_b6: f64 = (eq75_e1189_d_b6 * (nv8 - nv19));
        let eq75_e1191_d_b7: f64 = (eq75_e1189_d_b7 * (nv8 - nv19));
        let eq75_e1191_d_b8: f64 = (eq75_e1189_d_b8 * (nv8 - nv19));
        let eq75_e1191_d_b9: f64 = (eq75_e1189_d_b9 * (nv8 - nv19));
        let eq75_e1191_d_b10: f64 = (eq75_e1189_d_b10 * (nv8 - nv19));
        let eq75_e1191_d_b11: f64 = (eq75_e1189_d_b11 * (nv8 - nv19));
        let eq75_e1191_d_b12: f64 = (eq75_e1189_d_b12 * (nv8 - nv19));
        let eq75_e1191_d_b13: f64 = (eq75_e1189_d_b13 * (nv8 - nv19));
        let eq75_e1191_d_b14: f64 = (eq75_e1189_d_b14 * (nv8 - nv19));
        let eq75_e1191_d_b15: f64 = (eq75_e1189_d_b15 * (nv8 - nv19));
        let eq75_e1191_d_b16: f64 = (eq75_e1189_d_b16 * (nv8 - nv19));
        let eq75_e1191_d_b17: f64 = (eq75_e1189_d_b17 * (nv8 - nv19));
        let eq75_e1191_d_b18: f64 = (eq75_e1189_d_b18 * (nv8 - nv19));
        let eq75_e1191_d_b19: f64 = (eq75_e1189_d_b19 * (nv8 - nv19));
        let eq75_e1191_d_b20: f64 = (eq75_e1189_d_b20 * (nv8 - nv19));
        let eq75_e1191_d_b21: f64 = (eq75_e1189_d_b21 * (nv8 - nv19));
        let eq75_e1191_d_b22: f64 = (eq75_e1189_d_b22 * (nv8 - nv19));
        let eq75_e1191_d_b23: f64 = (eq75_e1189_d_b23 * (nv8 - nv19));
        let eq75_e1191_d_b24: f64 = (eq75_e1189_d_b24 * (nv8 - nv19));
        let eq75_e1191_d_b25: f64 = (eq75_e1189_d_b25 * (nv8 - nv19));
        let eq75_e1191_d_b26: f64 = (eq75_e1189_d_b26 * (nv8 - nv19));
        let eq75_e1191_d_b27: f64 = (eq75_e1189_d_b27 * (nv8 - nv19));
        let eq75_e1191_d_b28: f64 = (eq75_e1189_d_b28 * (nv8 - nv19));
        let eq75_e1191_d_b29: f64 = (eq75_e1189_d_b29 * (nv8 - nv19));
        let eq75_e1191_d_b30: f64 = (eq75_e1189_d_b30 * (nv8 - nv19));
        let eq75_e1191_d_b31: f64 = (eq75_e1189_d_b31 * (nv8 - nv19));
        let eq75_e1191_d_b32: f64 = (eq75_e1189_d_b32 * (nv8 - nv19));
        let eq75_e1191_d_b33: f64 = (eq75_e1189_d_b33 * (nv8 - nv19));
        let eq75_e1191_d_b34: f64 = (eq75_e1189_d_b34 * (nv8 - nv19));
        let eq75_e1191_d_b35: f64 = (eq75_e1189_d_b35 * (nv8 - nv19));
        let eq75_e1191_d_b36: f64 = (eq75_e1189_d_b36 * (nv8 - nv19));
        let eq75_e1191_d_b37: f64 = (eq75_e1189_d_b37 * (nv8 - nv19));
        let eq75_e1191_d_b38: f64 = (eq75_e1189_d_b38 * (nv8 - nv19));
        let eq75_e1191_d_b39: f64 = (eq75_e1189_d_b39 * (nv8 - nv19));
        let eq75_e1191_d_b40: f64 = (eq75_e1189_d_b40 * (nv8 - nv19));
        let eq75_e1191_d_b41: f64 = (eq75_e1189_d_b41 * (nv8 - nv19));
        let eq75_e1191_d_b42: f64 = (eq75_e1189_d_b42 * (nv8 - nv19));
        let eq75_e1191_d_b43: f64 = (eq75_e1189_d_b43 * (nv8 - nv19));
        let eq75_e1191_d_b44: f64 = (eq75_e1189_d_b44 * (nv8 - nv19));
        let eq75_e1191_d_b45: f64 = (eq75_e1189_d_b45 * (nv8 - nv19));
        let eq75_e1191_d_b46: f64 = (eq75_e1189_d_b46 * (nv8 - nv19));
        let eq75_e1191_d_b47: f64 = (eq75_e1189_d_b47 * (nv8 - nv19));
        let eq75_e1191_d_b48: f64 = (eq75_e1189_d_b48 * (nv8 - nv19));
        let eq75_e1191_d_b49: f64 = (eq75_e1189_d_b49 * (nv8 - nv19));
        let eq75_e1191_d_b50: f64 = (eq75_e1189_d_b50 * (nv8 - nv19));
        let eq75_e1191_d_b51: f64 = (eq75_e1189_d_b51 * (nv8 - nv19));
        let eq75_e1191_d_b52: f64 = (eq75_e1189_d_b52 * (nv8 - nv19));
        let eq75_e1191_d_b53: f64 = (eq75_e1189_d_b53 * (nv8 - nv19));
        let eq75_e1191_d_b54: f64 = (eq75_e1189_d_b54 * (nv8 - nv19));
        let eq75_e1192: f64 = (eq75_e1186 + eq75_e1191);
        let eq75_e1192_d_n0: f64 = (eq75_e1186_d_n0 + eq75_e1191_d_n0);
        let eq75_e1192_d_n1: f64 = (eq75_e1186_d_n1 + eq75_e1191_d_n1);
        let eq75_e1192_d_n2: f64 = (eq75_e1186_d_n2 + eq75_e1191_d_n2);
        let eq75_e1192_d_n3: f64 = (eq75_e1186_d_n3 + eq75_e1191_d_n3);
        let eq75_e1192_d_n4: f64 = (eq75_e1186_d_n4 + eq75_e1191_d_n4);
        let eq75_e1192_d_n5: f64 = (eq75_e1186_d_n5 + eq75_e1191_d_n5);
        let eq75_e1192_d_n6: f64 = (eq75_e1186_d_n6 + eq75_e1191_d_n6);
        let eq75_e1192_d_n7: f64 = (eq75_e1186_d_n7 + eq75_e1191_d_n7);
        let eq75_e1192_d_n8: f64 = (eq75_e1186_d_n8 + eq75_e1191_d_n8);
        let eq75_e1192_d_n9: f64 = (eq75_e1186_d_n9 + eq75_e1191_d_n9);
        let eq75_e1192_d_n10: f64 = (eq75_e1186_d_n10 + eq75_e1191_d_n10);
        let eq75_e1192_d_n11: f64 = (eq75_e1186_d_n11 + eq75_e1191_d_n11);
        let eq75_e1192_d_n12: f64 = (eq75_e1186_d_n12 + eq75_e1191_d_n12);
        let eq75_e1192_d_n13: f64 = (eq75_e1186_d_n13 + eq75_e1191_d_n13);
        let eq75_e1192_d_n14: f64 = (eq75_e1186_d_n14 + eq75_e1191_d_n14);
        let eq75_e1192_d_n15: f64 = (eq75_e1186_d_n15 + eq75_e1191_d_n15);
        let eq75_e1192_d_n16: f64 = (eq75_e1186_d_n16 + eq75_e1191_d_n16);
        let eq75_e1192_d_n17: f64 = (eq75_e1186_d_n17 + eq75_e1191_d_n17);
        let eq75_e1192_d_n18: f64 = (eq75_e1186_d_n18 + eq75_e1191_d_n18);
        let eq75_e1192_d_n19: f64 = (eq75_e1186_d_n19 + eq75_e1191_d_n19);
        let eq75_e1192_d_n20: f64 = (eq75_e1186_d_n20 + eq75_e1191_d_n20);
        let eq75_e1192_d_n21: f64 = (eq75_e1186_d_n21 + eq75_e1191_d_n21);
        let eq75_e1192_d_n22: f64 = (eq75_e1186_d_n22 + eq75_e1191_d_n22);
        let eq75_e1192_d_b0: f64 = (eq75_e1186_d_b0 + eq75_e1191_d_b0);
        let eq75_e1192_d_b1: f64 = (eq75_e1186_d_b1 + eq75_e1191_d_b1);
        let eq75_e1192_d_b2: f64 = (eq75_e1186_d_b2 + eq75_e1191_d_b2);
        let eq75_e1192_d_b3: f64 = (eq75_e1186_d_b3 + eq75_e1191_d_b3);
        let eq75_e1192_d_b4: f64 = (eq75_e1186_d_b4 + eq75_e1191_d_b4);
        let eq75_e1192_d_b5: f64 = (eq75_e1186_d_b5 + eq75_e1191_d_b5);
        let eq75_e1192_d_b6: f64 = (eq75_e1186_d_b6 + eq75_e1191_d_b6);
        let eq75_e1192_d_b7: f64 = (eq75_e1186_d_b7 + eq75_e1191_d_b7);
        let eq75_e1192_d_b8: f64 = (eq75_e1186_d_b8 + eq75_e1191_d_b8);
        let eq75_e1192_d_b9: f64 = (eq75_e1186_d_b9 + eq75_e1191_d_b9);
        let eq75_e1192_d_b10: f64 = (eq75_e1186_d_b10 + eq75_e1191_d_b10);
        let eq75_e1192_d_b11: f64 = (eq75_e1186_d_b11 + eq75_e1191_d_b11);
        let eq75_e1192_d_b12: f64 = (eq75_e1186_d_b12 + eq75_e1191_d_b12);
        let eq75_e1192_d_b13: f64 = (eq75_e1186_d_b13 + eq75_e1191_d_b13);
        let eq75_e1192_d_b14: f64 = (eq75_e1186_d_b14 + eq75_e1191_d_b14);
        let eq75_e1192_d_b15: f64 = (eq75_e1186_d_b15 + eq75_e1191_d_b15);
        let eq75_e1192_d_b16: f64 = (eq75_e1186_d_b16 + eq75_e1191_d_b16);
        let eq75_e1192_d_b17: f64 = (eq75_e1186_d_b17 + eq75_e1191_d_b17);
        let eq75_e1192_d_b18: f64 = (eq75_e1186_d_b18 + eq75_e1191_d_b18);
        let eq75_e1192_d_b19: f64 = (eq75_e1186_d_b19 + eq75_e1191_d_b19);
        let eq75_e1192_d_b20: f64 = (eq75_e1186_d_b20 + eq75_e1191_d_b20);
        let eq75_e1192_d_b21: f64 = (eq75_e1186_d_b21 + eq75_e1191_d_b21);
        let eq75_e1192_d_b22: f64 = (eq75_e1186_d_b22 + eq75_e1191_d_b22);
        let eq75_e1192_d_b23: f64 = (eq75_e1186_d_b23 + eq75_e1191_d_b23);
        let eq75_e1192_d_b24: f64 = (eq75_e1186_d_b24 + eq75_e1191_d_b24);
        let eq75_e1192_d_b25: f64 = (eq75_e1186_d_b25 + eq75_e1191_d_b25);
        let eq75_e1192_d_b26: f64 = (eq75_e1186_d_b26 + eq75_e1191_d_b26);
        let eq75_e1192_d_b27: f64 = (eq75_e1186_d_b27 + eq75_e1191_d_b27);
        let eq75_e1192_d_b28: f64 = (eq75_e1186_d_b28 + eq75_e1191_d_b28);
        let eq75_e1192_d_b29: f64 = (eq75_e1186_d_b29 + eq75_e1191_d_b29);
        let eq75_e1192_d_b30: f64 = (eq75_e1186_d_b30 + eq75_e1191_d_b30);
        let eq75_e1192_d_b31: f64 = (eq75_e1186_d_b31 + eq75_e1191_d_b31);
        let eq75_e1192_d_b32: f64 = (eq75_e1186_d_b32 + eq75_e1191_d_b32);
        let eq75_e1192_d_b33: f64 = (eq75_e1186_d_b33 + eq75_e1191_d_b33);
        let eq75_e1192_d_b34: f64 = (eq75_e1186_d_b34 + eq75_e1191_d_b34);
        let eq75_e1192_d_b35: f64 = (eq75_e1186_d_b35 + eq75_e1191_d_b35);
        let eq75_e1192_d_b36: f64 = (eq75_e1186_d_b36 + eq75_e1191_d_b36);
        let eq75_e1192_d_b37: f64 = (eq75_e1186_d_b37 + eq75_e1191_d_b37);
        let eq75_e1192_d_b38: f64 = (eq75_e1186_d_b38 + eq75_e1191_d_b38);
        let eq75_e1192_d_b39: f64 = (eq75_e1186_d_b39 + eq75_e1191_d_b39);
        let eq75_e1192_d_b40: f64 = (eq75_e1186_d_b40 + eq75_e1191_d_b40);
        let eq75_e1192_d_b41: f64 = (eq75_e1186_d_b41 + eq75_e1191_d_b41);
        let eq75_e1192_d_b42: f64 = (eq75_e1186_d_b42 + eq75_e1191_d_b42);
        let eq75_e1192_d_b43: f64 = (eq75_e1186_d_b43 + eq75_e1191_d_b43);
        let eq75_e1192_d_b44: f64 = (eq75_e1186_d_b44 + eq75_e1191_d_b44);
        let eq75_e1192_d_b45: f64 = (eq75_e1186_d_b45 + eq75_e1191_d_b45);
        let eq75_e1192_d_b46: f64 = (eq75_e1186_d_b46 + eq75_e1191_d_b46);
        let eq75_e1192_d_b47: f64 = (eq75_e1186_d_b47 + eq75_e1191_d_b47);
        let eq75_e1192_d_b48: f64 = (eq75_e1186_d_b48 + eq75_e1191_d_b48);
        let eq75_e1192_d_b49: f64 = (eq75_e1186_d_b49 + eq75_e1191_d_b49);
        let eq75_e1192_d_b50: f64 = (eq75_e1186_d_b50 + eq75_e1191_d_b50);
        let eq75_e1192_d_b51: f64 = (eq75_e1186_d_b51 + eq75_e1191_d_b51);
        let eq75_e1192_d_b52: f64 = (eq75_e1186_d_b52 + eq75_e1191_d_b52);
        let eq75_e1192_d_b53: f64 = (eq75_e1186_d_b53 + eq75_e1191_d_b53);
        let eq75_e1192_d_b54: f64 = (eq75_e1186_d_b54 + eq75_e1191_d_b54);
        (eq75_e1192, eq75_e1192_d_n0, eq75_e1192_d_n1, eq75_e1192_d_n2, eq75_e1192_d_n3, eq75_e1192_d_n4, eq75_e1192_d_n5, eq75_e1192_d_n6, eq75_e1192_d_n7, eq75_e1192_d_n8, eq75_e1192_d_n9, eq75_e1192_d_n10, eq75_e1192_d_n11, eq75_e1192_d_n12, eq75_e1192_d_n13, eq75_e1192_d_n14, eq75_e1192_d_n15, eq75_e1192_d_n16, eq75_e1192_d_n17, eq75_e1192_d_n18, eq75_e1192_d_n19, eq75_e1192_d_n20, eq75_e1192_d_n21, eq75_e1192_d_n22, eq75_e1192_d_b0, eq75_e1192_d_b1, eq75_e1192_d_b2, eq75_e1192_d_b3, eq75_e1192_d_b4, eq75_e1192_d_b5, eq75_e1192_d_b6, eq75_e1192_d_b7, eq75_e1192_d_b8, eq75_e1192_d_b9, eq75_e1192_d_b10, eq75_e1192_d_b11, eq75_e1192_d_b12, eq75_e1192_d_b13, eq75_e1192_d_b14, eq75_e1192_d_b15, eq75_e1192_d_b16, eq75_e1192_d_b17, eq75_e1192_d_b18, eq75_e1192_d_b19, eq75_e1192_d_b20, eq75_e1192_d_b21, eq75_e1192_d_b22, eq75_e1192_d_b23, eq75_e1192_d_b24, eq75_e1192_d_b25, eq75_e1192_d_b26, eq75_e1192_d_b27, eq75_e1192_d_b28, eq75_e1192_d_b29, eq75_e1192_d_b30, eq75_e1192_d_b31, eq75_e1192_d_b32, eq75_e1192_d_b33, eq75_e1192_d_b34, eq75_e1192_d_b35, eq75_e1192_d_b36, eq75_e1192_d_b37, eq75_e1192_d_b38, eq75_e1192_d_b39, eq75_e1192_d_b40, eq75_e1192_d_b41, eq75_e1192_d_b42, eq75_e1192_d_b43, eq75_e1192_d_b44, eq75_e1192_d_b45, eq75_e1192_d_b46, eq75_e1192_d_b47, eq75_e1192_d_b48, eq75_e1192_d_b49, eq75_e1192_d_b50, eq75_e1192_d_b51, eq75_e1192_d_b52, eq75_e1192_d_b53, eq75_e1192_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1194;
        let eq75_node_derivatives: [f64; 23] = [eq75_e1194_d_n0, eq75_e1194_d_n1, eq75_e1194_d_n2, eq75_e1194_d_n3, eq75_e1194_d_n4, eq75_e1194_d_n5, eq75_e1194_d_n6, eq75_e1194_d_n7, eq75_e1194_d_n8, eq75_e1194_d_n9, eq75_e1194_d_n10, eq75_e1194_d_n11, eq75_e1194_d_n12, eq75_e1194_d_n13, eq75_e1194_d_n14, eq75_e1194_d_n15, eq75_e1194_d_n16, eq75_e1194_d_n17, eq75_e1194_d_n18, eq75_e1194_d_n19, eq75_e1194_d_n20, eq75_e1194_d_n21, eq75_e1194_d_n22];
        let eq75_branch_derivatives: [f64; 55] = [eq75_e1194_d_b0, eq75_e1194_d_b1, eq75_e1194_d_b2, eq75_e1194_d_b3, eq75_e1194_d_b4, eq75_e1194_d_b5, eq75_e1194_d_b6, eq75_e1194_d_b7, eq75_e1194_d_b8, eq75_e1194_d_b9, eq75_e1194_d_b10, eq75_e1194_d_b11, eq75_e1194_d_b12, eq75_e1194_d_b13, eq75_e1194_d_b14, eq75_e1194_d_b15, eq75_e1194_d_b16, eq75_e1194_d_b17, eq75_e1194_d_b18, eq75_e1194_d_b19, eq75_e1194_d_b20, eq75_e1194_d_b21, eq75_e1194_d_b22, eq75_e1194_d_b23, eq75_e1194_d_b24, eq75_e1194_d_b25, eq75_e1194_d_b26, eq75_e1194_d_b27, eq75_e1194_d_b28, eq75_e1194_d_b29, eq75_e1194_d_b30, eq75_e1194_d_b31, eq75_e1194_d_b32, eq75_e1194_d_b33, eq75_e1194_d_b34, eq75_e1194_d_b35, eq75_e1194_d_b36, eq75_e1194_d_b37, eq75_e1194_d_b38, eq75_e1194_d_b39, eq75_e1194_d_b40, eq75_e1194_d_b41, eq75_e1194_d_b42, eq75_e1194_d_b43, eq75_e1194_d_b44, eq75_e1194_d_b45, eq75_e1194_d_b46, eq75_e1194_d_b47, eq75_e1194_d_b48, eq75_e1194_d_b49, eq75_e1194_d_b50, eq75_e1194_d_b51, eq75_e1194_d_b52, eq75_e1194_d_b53, eq75_e1194_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(19),
            multiplicity * (eq75_value),
            &eq75_node_derivatives,
            &eq75_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1201,) = {
    if (s.b[448] && (!s.b[449])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq76_value: f64 = eq76_e1201;
        stamper.stamp_potential_const_local(
            35,
            eq76_value,
        );
        let (eq77_e1206,) = {
    if (!s.b[448]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq77_value: f64 = eq77_e1206;
        stamper.stamp_potential_const_local(
            36,
            eq77_value,
        );
        let (eq78_e1214,) = {
    if ((!s.b[448]) && (!s.b[457])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq78_value: f64 = eq78_e1214;
        stamper.stamp_potential_const_local(
            37,
            eq78_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_14(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq79_e1230, eq79_e1230_d_n0, eq79_e1230_d_n1, eq79_e1230_d_n2, eq79_e1230_d_n3, eq79_e1230_d_n4, eq79_e1230_d_n5, eq79_e1230_d_n6, eq79_e1230_d_n7, eq79_e1230_d_n8, eq79_e1230_d_n9, eq79_e1230_d_n10, eq79_e1230_d_n11, eq79_e1230_d_n12, eq79_e1230_d_n13, eq79_e1230_d_n14, eq79_e1230_d_n15, eq79_e1230_d_n16, eq79_e1230_d_n17, eq79_e1230_d_n18, eq79_e1230_d_n19, eq79_e1230_d_n20, eq79_e1230_d_n21, eq79_e1230_d_n22, eq79_e1230_d_b0, eq79_e1230_d_b1, eq79_e1230_d_b2, eq79_e1230_d_b3, eq79_e1230_d_b4, eq79_e1230_d_b5, eq79_e1230_d_b6, eq79_e1230_d_b7, eq79_e1230_d_b8, eq79_e1230_d_b9, eq79_e1230_d_b10, eq79_e1230_d_b11, eq79_e1230_d_b12, eq79_e1230_d_b13, eq79_e1230_d_b14, eq79_e1230_d_b15, eq79_e1230_d_b16, eq79_e1230_d_b17, eq79_e1230_d_b18, eq79_e1230_d_b19, eq79_e1230_d_b20, eq79_e1230_d_b21, eq79_e1230_d_b22, eq79_e1230_d_b23, eq79_e1230_d_b24, eq79_e1230_d_b25, eq79_e1230_d_b26, eq79_e1230_d_b27, eq79_e1230_d_b28, eq79_e1230_d_b29, eq79_e1230_d_b30, eq79_e1230_d_b31, eq79_e1230_d_b32, eq79_e1230_d_b33, eq79_e1230_d_b34, eq79_e1230_d_b35, eq79_e1230_d_b36, eq79_e1230_d_b37, eq79_e1230_d_b38, eq79_e1230_d_b39, eq79_e1230_d_b40, eq79_e1230_d_b41, eq79_e1230_d_b42, eq79_e1230_d_b43, eq79_e1230_d_b44, eq79_e1230_d_b45, eq79_e1230_d_b46, eq79_e1230_d_b47, eq79_e1230_d_b48, eq79_e1230_d_b49, eq79_e1230_d_b50, eq79_e1230_d_b51, eq79_e1230_d_b52, eq79_e1230_d_b53, eq79_e1230_d_b54,) = {
    if (s.b[463] && s.b[464]) {
        let eq79_e1220: f64 = (p.p6 * s.v[56]);
        let eq79_e1220_d_n0: f64 = (p.p6 * s.dn[56][0]);
        let eq79_e1220_d_n1: f64 = (p.p6 * s.dn[56][1]);
        let eq79_e1220_d_n2: f64 = (p.p6 * s.dn[56][2]);
        let eq79_e1220_d_n3: f64 = (p.p6 * s.dn[56][3]);
        let eq79_e1220_d_n4: f64 = (p.p6 * s.dn[56][4]);
        let eq79_e1220_d_n5: f64 = (p.p6 * s.dn[56][5]);
        let eq79_e1220_d_n6: f64 = (p.p6 * s.dn[56][6]);
        let eq79_e1220_d_n7: f64 = (p.p6 * s.dn[56][7]);
        let eq79_e1220_d_n8: f64 = (p.p6 * s.dn[56][8]);
        let eq79_e1220_d_n9: f64 = (p.p6 * s.dn[56][9]);
        let eq79_e1220_d_n10: f64 = (p.p6 * s.dn[56][10]);
        let eq79_e1220_d_n11: f64 = (p.p6 * s.dn[56][11]);
        let eq79_e1220_d_n12: f64 = (p.p6 * s.dn[56][12]);
        let eq79_e1220_d_n13: f64 = (p.p6 * s.dn[56][13]);
        let eq79_e1220_d_n14: f64 = (p.p6 * s.dn[56][14]);
        let eq79_e1220_d_n15: f64 = (p.p6 * s.dn[56][15]);
        let eq79_e1220_d_n16: f64 = (p.p6 * s.dn[56][16]);
        let eq79_e1220_d_n17: f64 = (p.p6 * s.dn[56][17]);
        let eq79_e1220_d_n18: f64 = (p.p6 * s.dn[56][18]);
        let eq79_e1220_d_n19: f64 = (p.p6 * s.dn[56][19]);
        let eq79_e1220_d_n20: f64 = (p.p6 * s.dn[56][20]);
        let eq79_e1220_d_n21: f64 = (p.p6 * s.dn[56][21]);
        let eq79_e1220_d_n22: f64 = (p.p6 * s.dn[56][22]);
        let eq79_e1220_d_b0: f64 = (p.p6 * s.db[56][0]);
        let eq79_e1220_d_b1: f64 = (p.p6 * s.db[56][1]);
        let eq79_e1220_d_b2: f64 = (p.p6 * s.db[56][2]);
        let eq79_e1220_d_b3: f64 = (p.p6 * s.db[56][3]);
        let eq79_e1220_d_b4: f64 = (p.p6 * s.db[56][4]);
        let eq79_e1220_d_b5: f64 = (p.p6 * s.db[56][5]);
        let eq79_e1220_d_b6: f64 = (p.p6 * s.db[56][6]);
        let eq79_e1220_d_b7: f64 = (p.p6 * s.db[56][7]);
        let eq79_e1220_d_b8: f64 = (p.p6 * s.db[56][8]);
        let eq79_e1220_d_b9: f64 = (p.p6 * s.db[56][9]);
        let eq79_e1220_d_b10: f64 = (p.p6 * s.db[56][10]);
        let eq79_e1220_d_b11: f64 = (p.p6 * s.db[56][11]);
        let eq79_e1220_d_b12: f64 = (p.p6 * s.db[56][12]);
        let eq79_e1220_d_b13: f64 = (p.p6 * s.db[56][13]);
        let eq79_e1220_d_b14: f64 = (p.p6 * s.db[56][14]);
        let eq79_e1220_d_b15: f64 = (p.p6 * s.db[56][15]);
        let eq79_e1220_d_b16: f64 = (p.p6 * s.db[56][16]);
        let eq79_e1220_d_b17: f64 = (p.p6 * s.db[56][17]);
        let eq79_e1220_d_b18: f64 = (p.p6 * s.db[56][18]);
        let eq79_e1220_d_b19: f64 = (p.p6 * s.db[56][19]);
        let eq79_e1220_d_b20: f64 = (p.p6 * s.db[56][20]);
        let eq79_e1220_d_b21: f64 = (p.p6 * s.db[56][21]);
        let eq79_e1220_d_b22: f64 = (p.p6 * s.db[56][22]);
        let eq79_e1220_d_b23: f64 = (p.p6 * s.db[56][23]);
        let eq79_e1220_d_b24: f64 = (p.p6 * s.db[56][24]);
        let eq79_e1220_d_b25: f64 = (p.p6 * s.db[56][25]);
        let eq79_e1220_d_b26: f64 = (p.p6 * s.db[56][26]);
        let eq79_e1220_d_b27: f64 = (p.p6 * s.db[56][27]);
        let eq79_e1220_d_b28: f64 = (p.p6 * s.db[56][28]);
        let eq79_e1220_d_b29: f64 = (p.p6 * s.db[56][29]);
        let eq79_e1220_d_b30: f64 = (p.p6 * s.db[56][30]);
        let eq79_e1220_d_b31: f64 = (p.p6 * s.db[56][31]);
        let eq79_e1220_d_b32: f64 = (p.p6 * s.db[56][32]);
        let eq79_e1220_d_b33: f64 = (p.p6 * s.db[56][33]);
        let eq79_e1220_d_b34: f64 = (p.p6 * s.db[56][34]);
        let eq79_e1220_d_b35: f64 = (p.p6 * s.db[56][35]);
        let eq79_e1220_d_b36: f64 = (p.p6 * s.db[56][36]);
        let eq79_e1220_d_b37: f64 = (p.p6 * s.db[56][37]);
        let eq79_e1220_d_b38: f64 = (p.p6 * s.db[56][38]);
        let eq79_e1220_d_b39: f64 = (p.p6 * s.db[56][39]);
        let eq79_e1220_d_b40: f64 = (p.p6 * s.db[56][40]);
        let eq79_e1220_d_b41: f64 = (p.p6 * s.db[56][41]);
        let eq79_e1220_d_b42: f64 = (p.p6 * s.db[56][42]);
        let eq79_e1220_d_b43: f64 = (p.p6 * s.db[56][43]);
        let eq79_e1220_d_b44: f64 = (p.p6 * s.db[56][44]);
        let eq79_e1220_d_b45: f64 = (p.p6 * s.db[56][45]);
        let eq79_e1220_d_b46: f64 = (p.p6 * s.db[56][46]);
        let eq79_e1220_d_b47: f64 = (p.p6 * s.db[56][47]);
        let eq79_e1220_d_b48: f64 = (p.p6 * s.db[56][48]);
        let eq79_e1220_d_b49: f64 = (p.p6 * s.db[56][49]);
        let eq79_e1220_d_b50: f64 = (p.p6 * s.db[56][50]);
        let eq79_e1220_d_b51: f64 = (p.p6 * s.db[56][51]);
        let eq79_e1220_d_b52: f64 = (p.p6 * s.db[56][52]);
        let eq79_e1220_d_b53: f64 = (p.p6 * s.db[56][53]);
        let eq79_e1220_d_b54: f64 = (p.p6 * s.db[56][54]);
        let eq79_e1222: f64 = (eq79_e1220 * s.v[257]);
        let eq79_e1222_d_n0: f64 = ((eq79_e1220_d_n0 * s.v[257]) + (eq79_e1220 * s.dn[257][0]));
        let eq79_e1222_d_n1: f64 = ((eq79_e1220_d_n1 * s.v[257]) + (eq79_e1220 * s.dn[257][1]));
        let eq79_e1222_d_n2: f64 = ((eq79_e1220_d_n2 * s.v[257]) + (eq79_e1220 * s.dn[257][2]));
        let eq79_e1222_d_n3: f64 = ((eq79_e1220_d_n3 * s.v[257]) + (eq79_e1220 * s.dn[257][3]));
        let eq79_e1222_d_n4: f64 = ((eq79_e1220_d_n4 * s.v[257]) + (eq79_e1220 * s.dn[257][4]));
        let eq79_e1222_d_n5: f64 = ((eq79_e1220_d_n5 * s.v[257]) + (eq79_e1220 * s.dn[257][5]));
        let eq79_e1222_d_n6: f64 = ((eq79_e1220_d_n6 * s.v[257]) + (eq79_e1220 * s.dn[257][6]));
        let eq79_e1222_d_n7: f64 = ((eq79_e1220_d_n7 * s.v[257]) + (eq79_e1220 * s.dn[257][7]));
        let eq79_e1222_d_n8: f64 = ((eq79_e1220_d_n8 * s.v[257]) + (eq79_e1220 * s.dn[257][8]));
        let eq79_e1222_d_n9: f64 = ((eq79_e1220_d_n9 * s.v[257]) + (eq79_e1220 * s.dn[257][9]));
        let eq79_e1222_d_n10: f64 = ((eq79_e1220_d_n10 * s.v[257]) + (eq79_e1220 * s.dn[257][10]));
        let eq79_e1222_d_n11: f64 = ((eq79_e1220_d_n11 * s.v[257]) + (eq79_e1220 * s.dn[257][11]));
        let eq79_e1222_d_n12: f64 = ((eq79_e1220_d_n12 * s.v[257]) + (eq79_e1220 * s.dn[257][12]));
        let eq79_e1222_d_n13: f64 = ((eq79_e1220_d_n13 * s.v[257]) + (eq79_e1220 * s.dn[257][13]));
        let eq79_e1222_d_n14: f64 = ((eq79_e1220_d_n14 * s.v[257]) + (eq79_e1220 * s.dn[257][14]));
        let eq79_e1222_d_n15: f64 = ((eq79_e1220_d_n15 * s.v[257]) + (eq79_e1220 * s.dn[257][15]));
        let eq79_e1222_d_n16: f64 = ((eq79_e1220_d_n16 * s.v[257]) + (eq79_e1220 * s.dn[257][16]));
        let eq79_e1222_d_n17: f64 = ((eq79_e1220_d_n17 * s.v[257]) + (eq79_e1220 * s.dn[257][17]));
        let eq79_e1222_d_n18: f64 = ((eq79_e1220_d_n18 * s.v[257]) + (eq79_e1220 * s.dn[257][18]));
        let eq79_e1222_d_n19: f64 = ((eq79_e1220_d_n19 * s.v[257]) + (eq79_e1220 * s.dn[257][19]));
        let eq79_e1222_d_n20: f64 = ((eq79_e1220_d_n20 * s.v[257]) + (eq79_e1220 * s.dn[257][20]));
        let eq79_e1222_d_n21: f64 = ((eq79_e1220_d_n21 * s.v[257]) + (eq79_e1220 * s.dn[257][21]));
        let eq79_e1222_d_n22: f64 = ((eq79_e1220_d_n22 * s.v[257]) + (eq79_e1220 * s.dn[257][22]));
        let eq79_e1222_d_b0: f64 = ((eq79_e1220_d_b0 * s.v[257]) + (eq79_e1220 * s.db[257][0]));
        let eq79_e1222_d_b1: f64 = ((eq79_e1220_d_b1 * s.v[257]) + (eq79_e1220 * s.db[257][1]));
        let eq79_e1222_d_b2: f64 = ((eq79_e1220_d_b2 * s.v[257]) + (eq79_e1220 * s.db[257][2]));
        let eq79_e1222_d_b3: f64 = ((eq79_e1220_d_b3 * s.v[257]) + (eq79_e1220 * s.db[257][3]));
        let eq79_e1222_d_b4: f64 = ((eq79_e1220_d_b4 * s.v[257]) + (eq79_e1220 * s.db[257][4]));
        let eq79_e1222_d_b5: f64 = ((eq79_e1220_d_b5 * s.v[257]) + (eq79_e1220 * s.db[257][5]));
        let eq79_e1222_d_b6: f64 = ((eq79_e1220_d_b6 * s.v[257]) + (eq79_e1220 * s.db[257][6]));
        let eq79_e1222_d_b7: f64 = ((eq79_e1220_d_b7 * s.v[257]) + (eq79_e1220 * s.db[257][7]));
        let eq79_e1222_d_b8: f64 = ((eq79_e1220_d_b8 * s.v[257]) + (eq79_e1220 * s.db[257][8]));
        let eq79_e1222_d_b9: f64 = ((eq79_e1220_d_b9 * s.v[257]) + (eq79_e1220 * s.db[257][9]));
        let eq79_e1222_d_b10: f64 = ((eq79_e1220_d_b10 * s.v[257]) + (eq79_e1220 * s.db[257][10]));
        let eq79_e1222_d_b11: f64 = ((eq79_e1220_d_b11 * s.v[257]) + (eq79_e1220 * s.db[257][11]));
        let eq79_e1222_d_b12: f64 = ((eq79_e1220_d_b12 * s.v[257]) + (eq79_e1220 * s.db[257][12]));
        let eq79_e1222_d_b13: f64 = ((eq79_e1220_d_b13 * s.v[257]) + (eq79_e1220 * s.db[257][13]));
        let eq79_e1222_d_b14: f64 = ((eq79_e1220_d_b14 * s.v[257]) + (eq79_e1220 * s.db[257][14]));
        let eq79_e1222_d_b15: f64 = ((eq79_e1220_d_b15 * s.v[257]) + (eq79_e1220 * s.db[257][15]));
        let eq79_e1222_d_b16: f64 = ((eq79_e1220_d_b16 * s.v[257]) + (eq79_e1220 * s.db[257][16]));
        let eq79_e1222_d_b17: f64 = ((eq79_e1220_d_b17 * s.v[257]) + (eq79_e1220 * s.db[257][17]));
        let eq79_e1222_d_b18: f64 = ((eq79_e1220_d_b18 * s.v[257]) + (eq79_e1220 * s.db[257][18]));
        let eq79_e1222_d_b19: f64 = ((eq79_e1220_d_b19 * s.v[257]) + (eq79_e1220 * s.db[257][19]));
        let eq79_e1222_d_b20: f64 = ((eq79_e1220_d_b20 * s.v[257]) + (eq79_e1220 * s.db[257][20]));
        let eq79_e1222_d_b21: f64 = ((eq79_e1220_d_b21 * s.v[257]) + (eq79_e1220 * s.db[257][21]));
        let eq79_e1222_d_b22: f64 = ((eq79_e1220_d_b22 * s.v[257]) + (eq79_e1220 * s.db[257][22]));
        let eq79_e1222_d_b23: f64 = ((eq79_e1220_d_b23 * s.v[257]) + (eq79_e1220 * s.db[257][23]));
        let eq79_e1222_d_b24: f64 = ((eq79_e1220_d_b24 * s.v[257]) + (eq79_e1220 * s.db[257][24]));
        let eq79_e1222_d_b25: f64 = ((eq79_e1220_d_b25 * s.v[257]) + (eq79_e1220 * s.db[257][25]));
        let eq79_e1222_d_b26: f64 = ((eq79_e1220_d_b26 * s.v[257]) + (eq79_e1220 * s.db[257][26]));
        let eq79_e1222_d_b27: f64 = ((eq79_e1220_d_b27 * s.v[257]) + (eq79_e1220 * s.db[257][27]));
        let eq79_e1222_d_b28: f64 = ((eq79_e1220_d_b28 * s.v[257]) + (eq79_e1220 * s.db[257][28]));
        let eq79_e1222_d_b29: f64 = ((eq79_e1220_d_b29 * s.v[257]) + (eq79_e1220 * s.db[257][29]));
        let eq79_e1222_d_b30: f64 = ((eq79_e1220_d_b30 * s.v[257]) + (eq79_e1220 * s.db[257][30]));
        let eq79_e1222_d_b31: f64 = ((eq79_e1220_d_b31 * s.v[257]) + (eq79_e1220 * s.db[257][31]));
        let eq79_e1222_d_b32: f64 = ((eq79_e1220_d_b32 * s.v[257]) + (eq79_e1220 * s.db[257][32]));
        let eq79_e1222_d_b33: f64 = ((eq79_e1220_d_b33 * s.v[257]) + (eq79_e1220 * s.db[257][33]));
        let eq79_e1222_d_b34: f64 = ((eq79_e1220_d_b34 * s.v[257]) + (eq79_e1220 * s.db[257][34]));
        let eq79_e1222_d_b35: f64 = ((eq79_e1220_d_b35 * s.v[257]) + (eq79_e1220 * s.db[257][35]));
        let eq79_e1222_d_b36: f64 = ((eq79_e1220_d_b36 * s.v[257]) + (eq79_e1220 * s.db[257][36]));
        let eq79_e1222_d_b37: f64 = ((eq79_e1220_d_b37 * s.v[257]) + (eq79_e1220 * s.db[257][37]));
        let eq79_e1222_d_b38: f64 = ((eq79_e1220_d_b38 * s.v[257]) + (eq79_e1220 * s.db[257][38]));
        let eq79_e1222_d_b39: f64 = ((eq79_e1220_d_b39 * s.v[257]) + (eq79_e1220 * s.db[257][39]));
        let eq79_e1222_d_b40: f64 = ((eq79_e1220_d_b40 * s.v[257]) + (eq79_e1220 * s.db[257][40]));
        let eq79_e1222_d_b41: f64 = ((eq79_e1220_d_b41 * s.v[257]) + (eq79_e1220 * s.db[257][41]));
        let eq79_e1222_d_b42: f64 = ((eq79_e1220_d_b42 * s.v[257]) + (eq79_e1220 * s.db[257][42]));
        let eq79_e1222_d_b43: f64 = ((eq79_e1220_d_b43 * s.v[257]) + (eq79_e1220 * s.db[257][43]));
        let eq79_e1222_d_b44: f64 = ((eq79_e1220_d_b44 * s.v[257]) + (eq79_e1220 * s.db[257][44]));
        let eq79_e1222_d_b45: f64 = ((eq79_e1220_d_b45 * s.v[257]) + (eq79_e1220 * s.db[257][45]));
        let eq79_e1222_d_b46: f64 = ((eq79_e1220_d_b46 * s.v[257]) + (eq79_e1220 * s.db[257][46]));
        let eq79_e1222_d_b47: f64 = ((eq79_e1220_d_b47 * s.v[257]) + (eq79_e1220 * s.db[257][47]));
        let eq79_e1222_d_b48: f64 = ((eq79_e1220_d_b48 * s.v[257]) + (eq79_e1220 * s.db[257][48]));
        let eq79_e1222_d_b49: f64 = ((eq79_e1220_d_b49 * s.v[257]) + (eq79_e1220 * s.db[257][49]));
        let eq79_e1222_d_b50: f64 = ((eq79_e1220_d_b50 * s.v[257]) + (eq79_e1220 * s.db[257][50]));
        let eq79_e1222_d_b51: f64 = ((eq79_e1220_d_b51 * s.v[257]) + (eq79_e1220 * s.db[257][51]));
        let eq79_e1222_d_b52: f64 = ((eq79_e1220_d_b52 * s.v[257]) + (eq79_e1220 * s.db[257][52]));
        let eq79_e1222_d_b53: f64 = ((eq79_e1220_d_b53 * s.v[257]) + (eq79_e1220 * s.db[257][53]));
        let eq79_e1222_d_b54: f64 = ((eq79_e1220_d_b54 * s.v[257]) + (eq79_e1220 * s.db[257][54]));
        let eq79_e1225: f64 = (p.p6 * s.v[379]);
        let eq79_e1225_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq79_e1225_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq79_e1225_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq79_e1225_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq79_e1225_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq79_e1225_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq79_e1225_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq79_e1225_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq79_e1225_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq79_e1225_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq79_e1225_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq79_e1225_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq79_e1225_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq79_e1225_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq79_e1225_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq79_e1225_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq79_e1225_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq79_e1225_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq79_e1225_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq79_e1225_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq79_e1225_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq79_e1225_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq79_e1225_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq79_e1225_d_b0: f64 = (p.p6 * s.db[379][0]);
        let eq79_e1225_d_b1: f64 = (p.p6 * s.db[379][1]);
        let eq79_e1225_d_b2: f64 = (p.p6 * s.db[379][2]);
        let eq79_e1225_d_b3: f64 = (p.p6 * s.db[379][3]);
        let eq79_e1225_d_b4: f64 = (p.p6 * s.db[379][4]);
        let eq79_e1225_d_b5: f64 = (p.p6 * s.db[379][5]);
        let eq79_e1225_d_b6: f64 = (p.p6 * s.db[379][6]);
        let eq79_e1225_d_b7: f64 = (p.p6 * s.db[379][7]);
        let eq79_e1225_d_b8: f64 = (p.p6 * s.db[379][8]);
        let eq79_e1225_d_b9: f64 = (p.p6 * s.db[379][9]);
        let eq79_e1225_d_b10: f64 = (p.p6 * s.db[379][10]);
        let eq79_e1225_d_b11: f64 = (p.p6 * s.db[379][11]);
        let eq79_e1225_d_b12: f64 = (p.p6 * s.db[379][12]);
        let eq79_e1225_d_b13: f64 = (p.p6 * s.db[379][13]);
        let eq79_e1225_d_b14: f64 = (p.p6 * s.db[379][14]);
        let eq79_e1225_d_b15: f64 = (p.p6 * s.db[379][15]);
        let eq79_e1225_d_b16: f64 = (p.p6 * s.db[379][16]);
        let eq79_e1225_d_b17: f64 = (p.p6 * s.db[379][17]);
        let eq79_e1225_d_b18: f64 = (p.p6 * s.db[379][18]);
        let eq79_e1225_d_b19: f64 = (p.p6 * s.db[379][19]);
        let eq79_e1225_d_b20: f64 = (p.p6 * s.db[379][20]);
        let eq79_e1225_d_b21: f64 = (p.p6 * s.db[379][21]);
        let eq79_e1225_d_b22: f64 = (p.p6 * s.db[379][22]);
        let eq79_e1225_d_b23: f64 = (p.p6 * s.db[379][23]);
        let eq79_e1225_d_b24: f64 = (p.p6 * s.db[379][24]);
        let eq79_e1225_d_b25: f64 = (p.p6 * s.db[379][25]);
        let eq79_e1225_d_b26: f64 = (p.p6 * s.db[379][26]);
        let eq79_e1225_d_b27: f64 = (p.p6 * s.db[379][27]);
        let eq79_e1225_d_b28: f64 = (p.p6 * s.db[379][28]);
        let eq79_e1225_d_b29: f64 = (p.p6 * s.db[379][29]);
        let eq79_e1225_d_b30: f64 = (p.p6 * s.db[379][30]);
        let eq79_e1225_d_b31: f64 = (p.p6 * s.db[379][31]);
        let eq79_e1225_d_b32: f64 = (p.p6 * s.db[379][32]);
        let eq79_e1225_d_b33: f64 = (p.p6 * s.db[379][33]);
        let eq79_e1225_d_b34: f64 = (p.p6 * s.db[379][34]);
        let eq79_e1225_d_b35: f64 = (p.p6 * s.db[379][35]);
        let eq79_e1225_d_b36: f64 = (p.p6 * s.db[379][36]);
        let eq79_e1225_d_b37: f64 = (p.p6 * s.db[379][37]);
        let eq79_e1225_d_b38: f64 = (p.p6 * s.db[379][38]);
        let eq79_e1225_d_b39: f64 = (p.p6 * s.db[379][39]);
        let eq79_e1225_d_b40: f64 = (p.p6 * s.db[379][40]);
        let eq79_e1225_d_b41: f64 = (p.p6 * s.db[379][41]);
        let eq79_e1225_d_b42: f64 = (p.p6 * s.db[379][42]);
        let eq79_e1225_d_b43: f64 = (p.p6 * s.db[379][43]);
        let eq79_e1225_d_b44: f64 = (p.p6 * s.db[379][44]);
        let eq79_e1225_d_b45: f64 = (p.p6 * s.db[379][45]);
        let eq79_e1225_d_b46: f64 = (p.p6 * s.db[379][46]);
        let eq79_e1225_d_b47: f64 = (p.p6 * s.db[379][47]);
        let eq79_e1225_d_b48: f64 = (p.p6 * s.db[379][48]);
        let eq79_e1225_d_b49: f64 = (p.p6 * s.db[379][49]);
        let eq79_e1225_d_b50: f64 = (p.p6 * s.db[379][50]);
        let eq79_e1225_d_b51: f64 = (p.p6 * s.db[379][51]);
        let eq79_e1225_d_b52: f64 = (p.p6 * s.db[379][52]);
        let eq79_e1225_d_b53: f64 = (p.p6 * s.db[379][53]);
        let eq79_e1225_d_b54: f64 = (p.p6 * s.db[379][54]);
        let eq79_e1227: f64 = (eq79_e1225 * (nv16 - nv15));
        let eq79_e1227_d_n0: f64 = (eq79_e1225_d_n0 * (nv16 - nv15));
        let eq79_e1227_d_n1: f64 = (eq79_e1225_d_n1 * (nv16 - nv15));
        let eq79_e1227_d_n2: f64 = (eq79_e1225_d_n2 * (nv16 - nv15));
        let eq79_e1227_d_n3: f64 = (eq79_e1225_d_n3 * (nv16 - nv15));
        let eq79_e1227_d_n4: f64 = (eq79_e1225_d_n4 * (nv16 - nv15));
        let eq79_e1227_d_n5: f64 = (eq79_e1225_d_n5 * (nv16 - nv15));
        let eq79_e1227_d_n6: f64 = (eq79_e1225_d_n6 * (nv16 - nv15));
        let eq79_e1227_d_n7: f64 = (eq79_e1225_d_n7 * (nv16 - nv15));
        let eq79_e1227_d_n8: f64 = (eq79_e1225_d_n8 * (nv16 - nv15));
        let eq79_e1227_d_n9: f64 = (eq79_e1225_d_n9 * (nv16 - nv15));
        let eq79_e1227_d_n10: f64 = (eq79_e1225_d_n10 * (nv16 - nv15));
        let eq79_e1227_d_n11: f64 = (eq79_e1225_d_n11 * (nv16 - nv15));
        let eq79_e1227_d_n12: f64 = (eq79_e1225_d_n12 * (nv16 - nv15));
        let eq79_e1227_d_n13: f64 = (eq79_e1225_d_n13 * (nv16 - nv15));
        let eq79_e1227_d_n14: f64 = (eq79_e1225_d_n14 * (nv16 - nv15));
        let eq79_e1227_d_n15: f64 = ((eq79_e1225_d_n15 * (nv16 - nv15)) + (-eq79_e1225));
        let eq79_e1227_d_n16: f64 = ((eq79_e1225_d_n16 * (nv16 - nv15)) + eq79_e1225);
        let eq79_e1227_d_n17: f64 = (eq79_e1225_d_n17 * (nv16 - nv15));
        let eq79_e1227_d_n18: f64 = (eq79_e1225_d_n18 * (nv16 - nv15));
        let eq79_e1227_d_n19: f64 = (eq79_e1225_d_n19 * (nv16 - nv15));
        let eq79_e1227_d_n20: f64 = (eq79_e1225_d_n20 * (nv16 - nv15));
        let eq79_e1227_d_n21: f64 = (eq79_e1225_d_n21 * (nv16 - nv15));
        let eq79_e1227_d_n22: f64 = (eq79_e1225_d_n22 * (nv16 - nv15));
        let eq79_e1227_d_b0: f64 = (eq79_e1225_d_b0 * (nv16 - nv15));
        let eq79_e1227_d_b1: f64 = (eq79_e1225_d_b1 * (nv16 - nv15));
        let eq79_e1227_d_b2: f64 = (eq79_e1225_d_b2 * (nv16 - nv15));
        let eq79_e1227_d_b3: f64 = (eq79_e1225_d_b3 * (nv16 - nv15));
        let eq79_e1227_d_b4: f64 = (eq79_e1225_d_b4 * (nv16 - nv15));
        let eq79_e1227_d_b5: f64 = (eq79_e1225_d_b5 * (nv16 - nv15));
        let eq79_e1227_d_b6: f64 = (eq79_e1225_d_b6 * (nv16 - nv15));
        let eq79_e1227_d_b7: f64 = (eq79_e1225_d_b7 * (nv16 - nv15));
        let eq79_e1227_d_b8: f64 = (eq79_e1225_d_b8 * (nv16 - nv15));
        let eq79_e1227_d_b9: f64 = (eq79_e1225_d_b9 * (nv16 - nv15));
        let eq79_e1227_d_b10: f64 = (eq79_e1225_d_b10 * (nv16 - nv15));
        let eq79_e1227_d_b11: f64 = (eq79_e1225_d_b11 * (nv16 - nv15));
        let eq79_e1227_d_b12: f64 = (eq79_e1225_d_b12 * (nv16 - nv15));
        let eq79_e1227_d_b13: f64 = (eq79_e1225_d_b13 * (nv16 - nv15));
        let eq79_e1227_d_b14: f64 = (eq79_e1225_d_b14 * (nv16 - nv15));
        let eq79_e1227_d_b15: f64 = (eq79_e1225_d_b15 * (nv16 - nv15));
        let eq79_e1227_d_b16: f64 = (eq79_e1225_d_b16 * (nv16 - nv15));
        let eq79_e1227_d_b17: f64 = (eq79_e1225_d_b17 * (nv16 - nv15));
        let eq79_e1227_d_b18: f64 = (eq79_e1225_d_b18 * (nv16 - nv15));
        let eq79_e1227_d_b19: f64 = (eq79_e1225_d_b19 * (nv16 - nv15));
        let eq79_e1227_d_b20: f64 = (eq79_e1225_d_b20 * (nv16 - nv15));
        let eq79_e1227_d_b21: f64 = (eq79_e1225_d_b21 * (nv16 - nv15));
        let eq79_e1227_d_b22: f64 = (eq79_e1225_d_b22 * (nv16 - nv15));
        let eq79_e1227_d_b23: f64 = (eq79_e1225_d_b23 * (nv16 - nv15));
        let eq79_e1227_d_b24: f64 = (eq79_e1225_d_b24 * (nv16 - nv15));
        let eq79_e1227_d_b25: f64 = (eq79_e1225_d_b25 * (nv16 - nv15));
        let eq79_e1227_d_b26: f64 = (eq79_e1225_d_b26 * (nv16 - nv15));
        let eq79_e1227_d_b27: f64 = (eq79_e1225_d_b27 * (nv16 - nv15));
        let eq79_e1227_d_b28: f64 = (eq79_e1225_d_b28 * (nv16 - nv15));
        let eq79_e1227_d_b29: f64 = (eq79_e1225_d_b29 * (nv16 - nv15));
        let eq79_e1227_d_b30: f64 = (eq79_e1225_d_b30 * (nv16 - nv15));
        let eq79_e1227_d_b31: f64 = (eq79_e1225_d_b31 * (nv16 - nv15));
        let eq79_e1227_d_b32: f64 = (eq79_e1225_d_b32 * (nv16 - nv15));
        let eq79_e1227_d_b33: f64 = (eq79_e1225_d_b33 * (nv16 - nv15));
        let eq79_e1227_d_b34: f64 = (eq79_e1225_d_b34 * (nv16 - nv15));
        let eq79_e1227_d_b35: f64 = (eq79_e1225_d_b35 * (nv16 - nv15));
        let eq79_e1227_d_b36: f64 = (eq79_e1225_d_b36 * (nv16 - nv15));
        let eq79_e1227_d_b37: f64 = (eq79_e1225_d_b37 * (nv16 - nv15));
        let eq79_e1227_d_b38: f64 = (eq79_e1225_d_b38 * (nv16 - nv15));
        let eq79_e1227_d_b39: f64 = (eq79_e1225_d_b39 * (nv16 - nv15));
        let eq79_e1227_d_b40: f64 = (eq79_e1225_d_b40 * (nv16 - nv15));
        let eq79_e1227_d_b41: f64 = (eq79_e1225_d_b41 * (nv16 - nv15));
        let eq79_e1227_d_b42: f64 = (eq79_e1225_d_b42 * (nv16 - nv15));
        let eq79_e1227_d_b43: f64 = (eq79_e1225_d_b43 * (nv16 - nv15));
        let eq79_e1227_d_b44: f64 = (eq79_e1225_d_b44 * (nv16 - nv15));
        let eq79_e1227_d_b45: f64 = (eq79_e1225_d_b45 * (nv16 - nv15));
        let eq79_e1227_d_b46: f64 = (eq79_e1225_d_b46 * (nv16 - nv15));
        let eq79_e1227_d_b47: f64 = (eq79_e1225_d_b47 * (nv16 - nv15));
        let eq79_e1227_d_b48: f64 = (eq79_e1225_d_b48 * (nv16 - nv15));
        let eq79_e1227_d_b49: f64 = (eq79_e1225_d_b49 * (nv16 - nv15));
        let eq79_e1227_d_b50: f64 = (eq79_e1225_d_b50 * (nv16 - nv15));
        let eq79_e1227_d_b51: f64 = (eq79_e1225_d_b51 * (nv16 - nv15));
        let eq79_e1227_d_b52: f64 = (eq79_e1225_d_b52 * (nv16 - nv15));
        let eq79_e1227_d_b53: f64 = (eq79_e1225_d_b53 * (nv16 - nv15));
        let eq79_e1227_d_b54: f64 = (eq79_e1225_d_b54 * (nv16 - nv15));
        let eq79_e1228: f64 = (eq79_e1222 + eq79_e1227);
        let eq79_e1228_d_n0: f64 = (eq79_e1222_d_n0 + eq79_e1227_d_n0);
        let eq79_e1228_d_n1: f64 = (eq79_e1222_d_n1 + eq79_e1227_d_n1);
        let eq79_e1228_d_n2: f64 = (eq79_e1222_d_n2 + eq79_e1227_d_n2);
        let eq79_e1228_d_n3: f64 = (eq79_e1222_d_n3 + eq79_e1227_d_n3);
        let eq79_e1228_d_n4: f64 = (eq79_e1222_d_n4 + eq79_e1227_d_n4);
        let eq79_e1228_d_n5: f64 = (eq79_e1222_d_n5 + eq79_e1227_d_n5);
        let eq79_e1228_d_n6: f64 = (eq79_e1222_d_n6 + eq79_e1227_d_n6);
        let eq79_e1228_d_n7: f64 = (eq79_e1222_d_n7 + eq79_e1227_d_n7);
        let eq79_e1228_d_n8: f64 = (eq79_e1222_d_n8 + eq79_e1227_d_n8);
        let eq79_e1228_d_n9: f64 = (eq79_e1222_d_n9 + eq79_e1227_d_n9);
        let eq79_e1228_d_n10: f64 = (eq79_e1222_d_n10 + eq79_e1227_d_n10);
        let eq79_e1228_d_n11: f64 = (eq79_e1222_d_n11 + eq79_e1227_d_n11);
        let eq79_e1228_d_n12: f64 = (eq79_e1222_d_n12 + eq79_e1227_d_n12);
        let eq79_e1228_d_n13: f64 = (eq79_e1222_d_n13 + eq79_e1227_d_n13);
        let eq79_e1228_d_n14: f64 = (eq79_e1222_d_n14 + eq79_e1227_d_n14);
        let eq79_e1228_d_n15: f64 = (eq79_e1222_d_n15 + eq79_e1227_d_n15);
        let eq79_e1228_d_n16: f64 = (eq79_e1222_d_n16 + eq79_e1227_d_n16);
        let eq79_e1228_d_n17: f64 = (eq79_e1222_d_n17 + eq79_e1227_d_n17);
        let eq79_e1228_d_n18: f64 = (eq79_e1222_d_n18 + eq79_e1227_d_n18);
        let eq79_e1228_d_n19: f64 = (eq79_e1222_d_n19 + eq79_e1227_d_n19);
        let eq79_e1228_d_n20: f64 = (eq79_e1222_d_n20 + eq79_e1227_d_n20);
        let eq79_e1228_d_n21: f64 = (eq79_e1222_d_n21 + eq79_e1227_d_n21);
        let eq79_e1228_d_n22: f64 = (eq79_e1222_d_n22 + eq79_e1227_d_n22);
        let eq79_e1228_d_b0: f64 = (eq79_e1222_d_b0 + eq79_e1227_d_b0);
        let eq79_e1228_d_b1: f64 = (eq79_e1222_d_b1 + eq79_e1227_d_b1);
        let eq79_e1228_d_b2: f64 = (eq79_e1222_d_b2 + eq79_e1227_d_b2);
        let eq79_e1228_d_b3: f64 = (eq79_e1222_d_b3 + eq79_e1227_d_b3);
        let eq79_e1228_d_b4: f64 = (eq79_e1222_d_b4 + eq79_e1227_d_b4);
        let eq79_e1228_d_b5: f64 = (eq79_e1222_d_b5 + eq79_e1227_d_b5);
        let eq79_e1228_d_b6: f64 = (eq79_e1222_d_b6 + eq79_e1227_d_b6);
        let eq79_e1228_d_b7: f64 = (eq79_e1222_d_b7 + eq79_e1227_d_b7);
        let eq79_e1228_d_b8: f64 = (eq79_e1222_d_b8 + eq79_e1227_d_b8);
        let eq79_e1228_d_b9: f64 = (eq79_e1222_d_b9 + eq79_e1227_d_b9);
        let eq79_e1228_d_b10: f64 = (eq79_e1222_d_b10 + eq79_e1227_d_b10);
        let eq79_e1228_d_b11: f64 = (eq79_e1222_d_b11 + eq79_e1227_d_b11);
        let eq79_e1228_d_b12: f64 = (eq79_e1222_d_b12 + eq79_e1227_d_b12);
        let eq79_e1228_d_b13: f64 = (eq79_e1222_d_b13 + eq79_e1227_d_b13);
        let eq79_e1228_d_b14: f64 = (eq79_e1222_d_b14 + eq79_e1227_d_b14);
        let eq79_e1228_d_b15: f64 = (eq79_e1222_d_b15 + eq79_e1227_d_b15);
        let eq79_e1228_d_b16: f64 = (eq79_e1222_d_b16 + eq79_e1227_d_b16);
        let eq79_e1228_d_b17: f64 = (eq79_e1222_d_b17 + eq79_e1227_d_b17);
        let eq79_e1228_d_b18: f64 = (eq79_e1222_d_b18 + eq79_e1227_d_b18);
        let eq79_e1228_d_b19: f64 = (eq79_e1222_d_b19 + eq79_e1227_d_b19);
        let eq79_e1228_d_b20: f64 = (eq79_e1222_d_b20 + eq79_e1227_d_b20);
        let eq79_e1228_d_b21: f64 = (eq79_e1222_d_b21 + eq79_e1227_d_b21);
        let eq79_e1228_d_b22: f64 = (eq79_e1222_d_b22 + eq79_e1227_d_b22);
        let eq79_e1228_d_b23: f64 = (eq79_e1222_d_b23 + eq79_e1227_d_b23);
        let eq79_e1228_d_b24: f64 = (eq79_e1222_d_b24 + eq79_e1227_d_b24);
        let eq79_e1228_d_b25: f64 = (eq79_e1222_d_b25 + eq79_e1227_d_b25);
        let eq79_e1228_d_b26: f64 = (eq79_e1222_d_b26 + eq79_e1227_d_b26);
        let eq79_e1228_d_b27: f64 = (eq79_e1222_d_b27 + eq79_e1227_d_b27);
        let eq79_e1228_d_b28: f64 = (eq79_e1222_d_b28 + eq79_e1227_d_b28);
        let eq79_e1228_d_b29: f64 = (eq79_e1222_d_b29 + eq79_e1227_d_b29);
        let eq79_e1228_d_b30: f64 = (eq79_e1222_d_b30 + eq79_e1227_d_b30);
        let eq79_e1228_d_b31: f64 = (eq79_e1222_d_b31 + eq79_e1227_d_b31);
        let eq79_e1228_d_b32: f64 = (eq79_e1222_d_b32 + eq79_e1227_d_b32);
        let eq79_e1228_d_b33: f64 = (eq79_e1222_d_b33 + eq79_e1227_d_b33);
        let eq79_e1228_d_b34: f64 = (eq79_e1222_d_b34 + eq79_e1227_d_b34);
        let eq79_e1228_d_b35: f64 = (eq79_e1222_d_b35 + eq79_e1227_d_b35);
        let eq79_e1228_d_b36: f64 = (eq79_e1222_d_b36 + eq79_e1227_d_b36);
        let eq79_e1228_d_b37: f64 = (eq79_e1222_d_b37 + eq79_e1227_d_b37);
        let eq79_e1228_d_b38: f64 = (eq79_e1222_d_b38 + eq79_e1227_d_b38);
        let eq79_e1228_d_b39: f64 = (eq79_e1222_d_b39 + eq79_e1227_d_b39);
        let eq79_e1228_d_b40: f64 = (eq79_e1222_d_b40 + eq79_e1227_d_b40);
        let eq79_e1228_d_b41: f64 = (eq79_e1222_d_b41 + eq79_e1227_d_b41);
        let eq79_e1228_d_b42: f64 = (eq79_e1222_d_b42 + eq79_e1227_d_b42);
        let eq79_e1228_d_b43: f64 = (eq79_e1222_d_b43 + eq79_e1227_d_b43);
        let eq79_e1228_d_b44: f64 = (eq79_e1222_d_b44 + eq79_e1227_d_b44);
        let eq79_e1228_d_b45: f64 = (eq79_e1222_d_b45 + eq79_e1227_d_b45);
        let eq79_e1228_d_b46: f64 = (eq79_e1222_d_b46 + eq79_e1227_d_b46);
        let eq79_e1228_d_b47: f64 = (eq79_e1222_d_b47 + eq79_e1227_d_b47);
        let eq79_e1228_d_b48: f64 = (eq79_e1222_d_b48 + eq79_e1227_d_b48);
        let eq79_e1228_d_b49: f64 = (eq79_e1222_d_b49 + eq79_e1227_d_b49);
        let eq79_e1228_d_b50: f64 = (eq79_e1222_d_b50 + eq79_e1227_d_b50);
        let eq79_e1228_d_b51: f64 = (eq79_e1222_d_b51 + eq79_e1227_d_b51);
        let eq79_e1228_d_b52: f64 = (eq79_e1222_d_b52 + eq79_e1227_d_b52);
        let eq79_e1228_d_b53: f64 = (eq79_e1222_d_b53 + eq79_e1227_d_b53);
        let eq79_e1228_d_b54: f64 = (eq79_e1222_d_b54 + eq79_e1227_d_b54);
        (eq79_e1228, eq79_e1228_d_n0, eq79_e1228_d_n1, eq79_e1228_d_n2, eq79_e1228_d_n3, eq79_e1228_d_n4, eq79_e1228_d_n5, eq79_e1228_d_n6, eq79_e1228_d_n7, eq79_e1228_d_n8, eq79_e1228_d_n9, eq79_e1228_d_n10, eq79_e1228_d_n11, eq79_e1228_d_n12, eq79_e1228_d_n13, eq79_e1228_d_n14, eq79_e1228_d_n15, eq79_e1228_d_n16, eq79_e1228_d_n17, eq79_e1228_d_n18, eq79_e1228_d_n19, eq79_e1228_d_n20, eq79_e1228_d_n21, eq79_e1228_d_n22, eq79_e1228_d_b0, eq79_e1228_d_b1, eq79_e1228_d_b2, eq79_e1228_d_b3, eq79_e1228_d_b4, eq79_e1228_d_b5, eq79_e1228_d_b6, eq79_e1228_d_b7, eq79_e1228_d_b8, eq79_e1228_d_b9, eq79_e1228_d_b10, eq79_e1228_d_b11, eq79_e1228_d_b12, eq79_e1228_d_b13, eq79_e1228_d_b14, eq79_e1228_d_b15, eq79_e1228_d_b16, eq79_e1228_d_b17, eq79_e1228_d_b18, eq79_e1228_d_b19, eq79_e1228_d_b20, eq79_e1228_d_b21, eq79_e1228_d_b22, eq79_e1228_d_b23, eq79_e1228_d_b24, eq79_e1228_d_b25, eq79_e1228_d_b26, eq79_e1228_d_b27, eq79_e1228_d_b28, eq79_e1228_d_b29, eq79_e1228_d_b30, eq79_e1228_d_b31, eq79_e1228_d_b32, eq79_e1228_d_b33, eq79_e1228_d_b34, eq79_e1228_d_b35, eq79_e1228_d_b36, eq79_e1228_d_b37, eq79_e1228_d_b38, eq79_e1228_d_b39, eq79_e1228_d_b40, eq79_e1228_d_b41, eq79_e1228_d_b42, eq79_e1228_d_b43, eq79_e1228_d_b44, eq79_e1228_d_b45, eq79_e1228_d_b46, eq79_e1228_d_b47, eq79_e1228_d_b48, eq79_e1228_d_b49, eq79_e1228_d_b50, eq79_e1228_d_b51, eq79_e1228_d_b52, eq79_e1228_d_b53, eq79_e1228_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_value: f64 = eq79_e1230;
        let eq79_node_derivatives: [f64; 23] = [eq79_e1230_d_n0, eq79_e1230_d_n1, eq79_e1230_d_n2, eq79_e1230_d_n3, eq79_e1230_d_n4, eq79_e1230_d_n5, eq79_e1230_d_n6, eq79_e1230_d_n7, eq79_e1230_d_n8, eq79_e1230_d_n9, eq79_e1230_d_n10, eq79_e1230_d_n11, eq79_e1230_d_n12, eq79_e1230_d_n13, eq79_e1230_d_n14, eq79_e1230_d_n15, eq79_e1230_d_n16, eq79_e1230_d_n17, eq79_e1230_d_n18, eq79_e1230_d_n19, eq79_e1230_d_n20, eq79_e1230_d_n21, eq79_e1230_d_n22];
        let eq79_branch_derivatives: [f64; 55] = [eq79_e1230_d_b0, eq79_e1230_d_b1, eq79_e1230_d_b2, eq79_e1230_d_b3, eq79_e1230_d_b4, eq79_e1230_d_b5, eq79_e1230_d_b6, eq79_e1230_d_b7, eq79_e1230_d_b8, eq79_e1230_d_b9, eq79_e1230_d_b10, eq79_e1230_d_b11, eq79_e1230_d_b12, eq79_e1230_d_b13, eq79_e1230_d_b14, eq79_e1230_d_b15, eq79_e1230_d_b16, eq79_e1230_d_b17, eq79_e1230_d_b18, eq79_e1230_d_b19, eq79_e1230_d_b20, eq79_e1230_d_b21, eq79_e1230_d_b22, eq79_e1230_d_b23, eq79_e1230_d_b24, eq79_e1230_d_b25, eq79_e1230_d_b26, eq79_e1230_d_b27, eq79_e1230_d_b28, eq79_e1230_d_b29, eq79_e1230_d_b30, eq79_e1230_d_b31, eq79_e1230_d_b32, eq79_e1230_d_b33, eq79_e1230_d_b34, eq79_e1230_d_b35, eq79_e1230_d_b36, eq79_e1230_d_b37, eq79_e1230_d_b38, eq79_e1230_d_b39, eq79_e1230_d_b40, eq79_e1230_d_b41, eq79_e1230_d_b42, eq79_e1230_d_b43, eq79_e1230_d_b44, eq79_e1230_d_b45, eq79_e1230_d_b46, eq79_e1230_d_b47, eq79_e1230_d_b48, eq79_e1230_d_b49, eq79_e1230_d_b50, eq79_e1230_d_b51, eq79_e1230_d_b52, eq79_e1230_d_b53, eq79_e1230_d_b54];
        stamper.stamp_current_dense_local(
            Some(16),
            Some(15),
            multiplicity * (eq79_value),
            &eq79_node_derivatives,
            &eq79_branch_derivatives,
            multiplicity,
        );
        let (eq80_e1237,) = {
    if (s.b[463] && (!s.b[464])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq80_value: f64 = eq80_e1237;
        stamper.stamp_potential_const_local(
            38,
            eq80_value,
        );
        let (eq81_e1242,) = {
    if (!s.b[463]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq81_value: f64 = eq81_e1242;
        stamper.stamp_potential_const_local(
            39,
            eq81_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_15(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv19 = ctx.node_voltage(nodes[19]);
        let nv20 = ctx.node_voltage(nodes[20]);
        let (eq82_e1258, eq82_e1258_d_n0, eq82_e1258_d_n1, eq82_e1258_d_n2, eq82_e1258_d_n3, eq82_e1258_d_n4, eq82_e1258_d_n5, eq82_e1258_d_n6, eq82_e1258_d_n7, eq82_e1258_d_n8, eq82_e1258_d_n9, eq82_e1258_d_n10, eq82_e1258_d_n11, eq82_e1258_d_n12, eq82_e1258_d_n13, eq82_e1258_d_n14, eq82_e1258_d_n15, eq82_e1258_d_n16, eq82_e1258_d_n17, eq82_e1258_d_n18, eq82_e1258_d_n19, eq82_e1258_d_n20, eq82_e1258_d_n21, eq82_e1258_d_n22, eq82_e1258_d_b0, eq82_e1258_d_b1, eq82_e1258_d_b2, eq82_e1258_d_b3, eq82_e1258_d_b4, eq82_e1258_d_b5, eq82_e1258_d_b6, eq82_e1258_d_b7, eq82_e1258_d_b8, eq82_e1258_d_b9, eq82_e1258_d_b10, eq82_e1258_d_b11, eq82_e1258_d_b12, eq82_e1258_d_b13, eq82_e1258_d_b14, eq82_e1258_d_b15, eq82_e1258_d_b16, eq82_e1258_d_b17, eq82_e1258_d_b18, eq82_e1258_d_b19, eq82_e1258_d_b20, eq82_e1258_d_b21, eq82_e1258_d_b22, eq82_e1258_d_b23, eq82_e1258_d_b24, eq82_e1258_d_b25, eq82_e1258_d_b26, eq82_e1258_d_b27, eq82_e1258_d_b28, eq82_e1258_d_b29, eq82_e1258_d_b30, eq82_e1258_d_b31, eq82_e1258_d_b32, eq82_e1258_d_b33, eq82_e1258_d_b34, eq82_e1258_d_b35, eq82_e1258_d_b36, eq82_e1258_d_b37, eq82_e1258_d_b38, eq82_e1258_d_b39, eq82_e1258_d_b40, eq82_e1258_d_b41, eq82_e1258_d_b42, eq82_e1258_d_b43, eq82_e1258_d_b44, eq82_e1258_d_b45, eq82_e1258_d_b46, eq82_e1258_d_b47, eq82_e1258_d_b48, eq82_e1258_d_b49, eq82_e1258_d_b50, eq82_e1258_d_b51, eq82_e1258_d_b52, eq82_e1258_d_b53, eq82_e1258_d_b54,) = {
    if (s.b[478] && s.b[479]) {
        let eq82_e1248: f64 = (p.p6 * s.v[60]);
        let eq82_e1248_d_n0: f64 = (p.p6 * s.dn[60][0]);
        let eq82_e1248_d_n1: f64 = (p.p6 * s.dn[60][1]);
        let eq82_e1248_d_n2: f64 = (p.p6 * s.dn[60][2]);
        let eq82_e1248_d_n3: f64 = (p.p6 * s.dn[60][3]);
        let eq82_e1248_d_n4: f64 = (p.p6 * s.dn[60][4]);
        let eq82_e1248_d_n5: f64 = (p.p6 * s.dn[60][5]);
        let eq82_e1248_d_n6: f64 = (p.p6 * s.dn[60][6]);
        let eq82_e1248_d_n7: f64 = (p.p6 * s.dn[60][7]);
        let eq82_e1248_d_n8: f64 = (p.p6 * s.dn[60][8]);
        let eq82_e1248_d_n9: f64 = (p.p6 * s.dn[60][9]);
        let eq82_e1248_d_n10: f64 = (p.p6 * s.dn[60][10]);
        let eq82_e1248_d_n11: f64 = (p.p6 * s.dn[60][11]);
        let eq82_e1248_d_n12: f64 = (p.p6 * s.dn[60][12]);
        let eq82_e1248_d_n13: f64 = (p.p6 * s.dn[60][13]);
        let eq82_e1248_d_n14: f64 = (p.p6 * s.dn[60][14]);
        let eq82_e1248_d_n15: f64 = (p.p6 * s.dn[60][15]);
        let eq82_e1248_d_n16: f64 = (p.p6 * s.dn[60][16]);
        let eq82_e1248_d_n17: f64 = (p.p6 * s.dn[60][17]);
        let eq82_e1248_d_n18: f64 = (p.p6 * s.dn[60][18]);
        let eq82_e1248_d_n19: f64 = (p.p6 * s.dn[60][19]);
        let eq82_e1248_d_n20: f64 = (p.p6 * s.dn[60][20]);
        let eq82_e1248_d_n21: f64 = (p.p6 * s.dn[60][21]);
        let eq82_e1248_d_n22: f64 = (p.p6 * s.dn[60][22]);
        let eq82_e1248_d_b0: f64 = (p.p6 * s.db[60][0]);
        let eq82_e1248_d_b1: f64 = (p.p6 * s.db[60][1]);
        let eq82_e1248_d_b2: f64 = (p.p6 * s.db[60][2]);
        let eq82_e1248_d_b3: f64 = (p.p6 * s.db[60][3]);
        let eq82_e1248_d_b4: f64 = (p.p6 * s.db[60][4]);
        let eq82_e1248_d_b5: f64 = (p.p6 * s.db[60][5]);
        let eq82_e1248_d_b6: f64 = (p.p6 * s.db[60][6]);
        let eq82_e1248_d_b7: f64 = (p.p6 * s.db[60][7]);
        let eq82_e1248_d_b8: f64 = (p.p6 * s.db[60][8]);
        let eq82_e1248_d_b9: f64 = (p.p6 * s.db[60][9]);
        let eq82_e1248_d_b10: f64 = (p.p6 * s.db[60][10]);
        let eq82_e1248_d_b11: f64 = (p.p6 * s.db[60][11]);
        let eq82_e1248_d_b12: f64 = (p.p6 * s.db[60][12]);
        let eq82_e1248_d_b13: f64 = (p.p6 * s.db[60][13]);
        let eq82_e1248_d_b14: f64 = (p.p6 * s.db[60][14]);
        let eq82_e1248_d_b15: f64 = (p.p6 * s.db[60][15]);
        let eq82_e1248_d_b16: f64 = (p.p6 * s.db[60][16]);
        let eq82_e1248_d_b17: f64 = (p.p6 * s.db[60][17]);
        let eq82_e1248_d_b18: f64 = (p.p6 * s.db[60][18]);
        let eq82_e1248_d_b19: f64 = (p.p6 * s.db[60][19]);
        let eq82_e1248_d_b20: f64 = (p.p6 * s.db[60][20]);
        let eq82_e1248_d_b21: f64 = (p.p6 * s.db[60][21]);
        let eq82_e1248_d_b22: f64 = (p.p6 * s.db[60][22]);
        let eq82_e1248_d_b23: f64 = (p.p6 * s.db[60][23]);
        let eq82_e1248_d_b24: f64 = (p.p6 * s.db[60][24]);
        let eq82_e1248_d_b25: f64 = (p.p6 * s.db[60][25]);
        let eq82_e1248_d_b26: f64 = (p.p6 * s.db[60][26]);
        let eq82_e1248_d_b27: f64 = (p.p6 * s.db[60][27]);
        let eq82_e1248_d_b28: f64 = (p.p6 * s.db[60][28]);
        let eq82_e1248_d_b29: f64 = (p.p6 * s.db[60][29]);
        let eq82_e1248_d_b30: f64 = (p.p6 * s.db[60][30]);
        let eq82_e1248_d_b31: f64 = (p.p6 * s.db[60][31]);
        let eq82_e1248_d_b32: f64 = (p.p6 * s.db[60][32]);
        let eq82_e1248_d_b33: f64 = (p.p6 * s.db[60][33]);
        let eq82_e1248_d_b34: f64 = (p.p6 * s.db[60][34]);
        let eq82_e1248_d_b35: f64 = (p.p6 * s.db[60][35]);
        let eq82_e1248_d_b36: f64 = (p.p6 * s.db[60][36]);
        let eq82_e1248_d_b37: f64 = (p.p6 * s.db[60][37]);
        let eq82_e1248_d_b38: f64 = (p.p6 * s.db[60][38]);
        let eq82_e1248_d_b39: f64 = (p.p6 * s.db[60][39]);
        let eq82_e1248_d_b40: f64 = (p.p6 * s.db[60][40]);
        let eq82_e1248_d_b41: f64 = (p.p6 * s.db[60][41]);
        let eq82_e1248_d_b42: f64 = (p.p6 * s.db[60][42]);
        let eq82_e1248_d_b43: f64 = (p.p6 * s.db[60][43]);
        let eq82_e1248_d_b44: f64 = (p.p6 * s.db[60][44]);
        let eq82_e1248_d_b45: f64 = (p.p6 * s.db[60][45]);
        let eq82_e1248_d_b46: f64 = (p.p6 * s.db[60][46]);
        let eq82_e1248_d_b47: f64 = (p.p6 * s.db[60][47]);
        let eq82_e1248_d_b48: f64 = (p.p6 * s.db[60][48]);
        let eq82_e1248_d_b49: f64 = (p.p6 * s.db[60][49]);
        let eq82_e1248_d_b50: f64 = (p.p6 * s.db[60][50]);
        let eq82_e1248_d_b51: f64 = (p.p6 * s.db[60][51]);
        let eq82_e1248_d_b52: f64 = (p.p6 * s.db[60][52]);
        let eq82_e1248_d_b53: f64 = (p.p6 * s.db[60][53]);
        let eq82_e1248_d_b54: f64 = (p.p6 * s.db[60][54]);
        let eq82_e1250: f64 = (eq82_e1248 * s.v[269]);
        let eq82_e1250_d_n0: f64 = ((eq82_e1248_d_n0 * s.v[269]) + (eq82_e1248 * s.dn[269][0]));
        let eq82_e1250_d_n1: f64 = ((eq82_e1248_d_n1 * s.v[269]) + (eq82_e1248 * s.dn[269][1]));
        let eq82_e1250_d_n2: f64 = ((eq82_e1248_d_n2 * s.v[269]) + (eq82_e1248 * s.dn[269][2]));
        let eq82_e1250_d_n3: f64 = ((eq82_e1248_d_n3 * s.v[269]) + (eq82_e1248 * s.dn[269][3]));
        let eq82_e1250_d_n4: f64 = ((eq82_e1248_d_n4 * s.v[269]) + (eq82_e1248 * s.dn[269][4]));
        let eq82_e1250_d_n5: f64 = ((eq82_e1248_d_n5 * s.v[269]) + (eq82_e1248 * s.dn[269][5]));
        let eq82_e1250_d_n6: f64 = ((eq82_e1248_d_n6 * s.v[269]) + (eq82_e1248 * s.dn[269][6]));
        let eq82_e1250_d_n7: f64 = ((eq82_e1248_d_n7 * s.v[269]) + (eq82_e1248 * s.dn[269][7]));
        let eq82_e1250_d_n8: f64 = ((eq82_e1248_d_n8 * s.v[269]) + (eq82_e1248 * s.dn[269][8]));
        let eq82_e1250_d_n9: f64 = ((eq82_e1248_d_n9 * s.v[269]) + (eq82_e1248 * s.dn[269][9]));
        let eq82_e1250_d_n10: f64 = ((eq82_e1248_d_n10 * s.v[269]) + (eq82_e1248 * s.dn[269][10]));
        let eq82_e1250_d_n11: f64 = ((eq82_e1248_d_n11 * s.v[269]) + (eq82_e1248 * s.dn[269][11]));
        let eq82_e1250_d_n12: f64 = ((eq82_e1248_d_n12 * s.v[269]) + (eq82_e1248 * s.dn[269][12]));
        let eq82_e1250_d_n13: f64 = ((eq82_e1248_d_n13 * s.v[269]) + (eq82_e1248 * s.dn[269][13]));
        let eq82_e1250_d_n14: f64 = ((eq82_e1248_d_n14 * s.v[269]) + (eq82_e1248 * s.dn[269][14]));
        let eq82_e1250_d_n15: f64 = ((eq82_e1248_d_n15 * s.v[269]) + (eq82_e1248 * s.dn[269][15]));
        let eq82_e1250_d_n16: f64 = ((eq82_e1248_d_n16 * s.v[269]) + (eq82_e1248 * s.dn[269][16]));
        let eq82_e1250_d_n17: f64 = ((eq82_e1248_d_n17 * s.v[269]) + (eq82_e1248 * s.dn[269][17]));
        let eq82_e1250_d_n18: f64 = ((eq82_e1248_d_n18 * s.v[269]) + (eq82_e1248 * s.dn[269][18]));
        let eq82_e1250_d_n19: f64 = ((eq82_e1248_d_n19 * s.v[269]) + (eq82_e1248 * s.dn[269][19]));
        let eq82_e1250_d_n20: f64 = ((eq82_e1248_d_n20 * s.v[269]) + (eq82_e1248 * s.dn[269][20]));
        let eq82_e1250_d_n21: f64 = ((eq82_e1248_d_n21 * s.v[269]) + (eq82_e1248 * s.dn[269][21]));
        let eq82_e1250_d_n22: f64 = ((eq82_e1248_d_n22 * s.v[269]) + (eq82_e1248 * s.dn[269][22]));
        let eq82_e1250_d_b0: f64 = ((eq82_e1248_d_b0 * s.v[269]) + (eq82_e1248 * s.db[269][0]));
        let eq82_e1250_d_b1: f64 = ((eq82_e1248_d_b1 * s.v[269]) + (eq82_e1248 * s.db[269][1]));
        let eq82_e1250_d_b2: f64 = ((eq82_e1248_d_b2 * s.v[269]) + (eq82_e1248 * s.db[269][2]));
        let eq82_e1250_d_b3: f64 = ((eq82_e1248_d_b3 * s.v[269]) + (eq82_e1248 * s.db[269][3]));
        let eq82_e1250_d_b4: f64 = ((eq82_e1248_d_b4 * s.v[269]) + (eq82_e1248 * s.db[269][4]));
        let eq82_e1250_d_b5: f64 = ((eq82_e1248_d_b5 * s.v[269]) + (eq82_e1248 * s.db[269][5]));
        let eq82_e1250_d_b6: f64 = ((eq82_e1248_d_b6 * s.v[269]) + (eq82_e1248 * s.db[269][6]));
        let eq82_e1250_d_b7: f64 = ((eq82_e1248_d_b7 * s.v[269]) + (eq82_e1248 * s.db[269][7]));
        let eq82_e1250_d_b8: f64 = ((eq82_e1248_d_b8 * s.v[269]) + (eq82_e1248 * s.db[269][8]));
        let eq82_e1250_d_b9: f64 = ((eq82_e1248_d_b9 * s.v[269]) + (eq82_e1248 * s.db[269][9]));
        let eq82_e1250_d_b10: f64 = ((eq82_e1248_d_b10 * s.v[269]) + (eq82_e1248 * s.db[269][10]));
        let eq82_e1250_d_b11: f64 = ((eq82_e1248_d_b11 * s.v[269]) + (eq82_e1248 * s.db[269][11]));
        let eq82_e1250_d_b12: f64 = ((eq82_e1248_d_b12 * s.v[269]) + (eq82_e1248 * s.db[269][12]));
        let eq82_e1250_d_b13: f64 = ((eq82_e1248_d_b13 * s.v[269]) + (eq82_e1248 * s.db[269][13]));
        let eq82_e1250_d_b14: f64 = ((eq82_e1248_d_b14 * s.v[269]) + (eq82_e1248 * s.db[269][14]));
        let eq82_e1250_d_b15: f64 = ((eq82_e1248_d_b15 * s.v[269]) + (eq82_e1248 * s.db[269][15]));
        let eq82_e1250_d_b16: f64 = ((eq82_e1248_d_b16 * s.v[269]) + (eq82_e1248 * s.db[269][16]));
        let eq82_e1250_d_b17: f64 = ((eq82_e1248_d_b17 * s.v[269]) + (eq82_e1248 * s.db[269][17]));
        let eq82_e1250_d_b18: f64 = ((eq82_e1248_d_b18 * s.v[269]) + (eq82_e1248 * s.db[269][18]));
        let eq82_e1250_d_b19: f64 = ((eq82_e1248_d_b19 * s.v[269]) + (eq82_e1248 * s.db[269][19]));
        let eq82_e1250_d_b20: f64 = ((eq82_e1248_d_b20 * s.v[269]) + (eq82_e1248 * s.db[269][20]));
        let eq82_e1250_d_b21: f64 = ((eq82_e1248_d_b21 * s.v[269]) + (eq82_e1248 * s.db[269][21]));
        let eq82_e1250_d_b22: f64 = ((eq82_e1248_d_b22 * s.v[269]) + (eq82_e1248 * s.db[269][22]));
        let eq82_e1250_d_b23: f64 = ((eq82_e1248_d_b23 * s.v[269]) + (eq82_e1248 * s.db[269][23]));
        let eq82_e1250_d_b24: f64 = ((eq82_e1248_d_b24 * s.v[269]) + (eq82_e1248 * s.db[269][24]));
        let eq82_e1250_d_b25: f64 = ((eq82_e1248_d_b25 * s.v[269]) + (eq82_e1248 * s.db[269][25]));
        let eq82_e1250_d_b26: f64 = ((eq82_e1248_d_b26 * s.v[269]) + (eq82_e1248 * s.db[269][26]));
        let eq82_e1250_d_b27: f64 = ((eq82_e1248_d_b27 * s.v[269]) + (eq82_e1248 * s.db[269][27]));
        let eq82_e1250_d_b28: f64 = ((eq82_e1248_d_b28 * s.v[269]) + (eq82_e1248 * s.db[269][28]));
        let eq82_e1250_d_b29: f64 = ((eq82_e1248_d_b29 * s.v[269]) + (eq82_e1248 * s.db[269][29]));
        let eq82_e1250_d_b30: f64 = ((eq82_e1248_d_b30 * s.v[269]) + (eq82_e1248 * s.db[269][30]));
        let eq82_e1250_d_b31: f64 = ((eq82_e1248_d_b31 * s.v[269]) + (eq82_e1248 * s.db[269][31]));
        let eq82_e1250_d_b32: f64 = ((eq82_e1248_d_b32 * s.v[269]) + (eq82_e1248 * s.db[269][32]));
        let eq82_e1250_d_b33: f64 = ((eq82_e1248_d_b33 * s.v[269]) + (eq82_e1248 * s.db[269][33]));
        let eq82_e1250_d_b34: f64 = ((eq82_e1248_d_b34 * s.v[269]) + (eq82_e1248 * s.db[269][34]));
        let eq82_e1250_d_b35: f64 = ((eq82_e1248_d_b35 * s.v[269]) + (eq82_e1248 * s.db[269][35]));
        let eq82_e1250_d_b36: f64 = ((eq82_e1248_d_b36 * s.v[269]) + (eq82_e1248 * s.db[269][36]));
        let eq82_e1250_d_b37: f64 = ((eq82_e1248_d_b37 * s.v[269]) + (eq82_e1248 * s.db[269][37]));
        let eq82_e1250_d_b38: f64 = ((eq82_e1248_d_b38 * s.v[269]) + (eq82_e1248 * s.db[269][38]));
        let eq82_e1250_d_b39: f64 = ((eq82_e1248_d_b39 * s.v[269]) + (eq82_e1248 * s.db[269][39]));
        let eq82_e1250_d_b40: f64 = ((eq82_e1248_d_b40 * s.v[269]) + (eq82_e1248 * s.db[269][40]));
        let eq82_e1250_d_b41: f64 = ((eq82_e1248_d_b41 * s.v[269]) + (eq82_e1248 * s.db[269][41]));
        let eq82_e1250_d_b42: f64 = ((eq82_e1248_d_b42 * s.v[269]) + (eq82_e1248 * s.db[269][42]));
        let eq82_e1250_d_b43: f64 = ((eq82_e1248_d_b43 * s.v[269]) + (eq82_e1248 * s.db[269][43]));
        let eq82_e1250_d_b44: f64 = ((eq82_e1248_d_b44 * s.v[269]) + (eq82_e1248 * s.db[269][44]));
        let eq82_e1250_d_b45: f64 = ((eq82_e1248_d_b45 * s.v[269]) + (eq82_e1248 * s.db[269][45]));
        let eq82_e1250_d_b46: f64 = ((eq82_e1248_d_b46 * s.v[269]) + (eq82_e1248 * s.db[269][46]));
        let eq82_e1250_d_b47: f64 = ((eq82_e1248_d_b47 * s.v[269]) + (eq82_e1248 * s.db[269][47]));
        let eq82_e1250_d_b48: f64 = ((eq82_e1248_d_b48 * s.v[269]) + (eq82_e1248 * s.db[269][48]));
        let eq82_e1250_d_b49: f64 = ((eq82_e1248_d_b49 * s.v[269]) + (eq82_e1248 * s.db[269][49]));
        let eq82_e1250_d_b50: f64 = ((eq82_e1248_d_b50 * s.v[269]) + (eq82_e1248 * s.db[269][50]));
        let eq82_e1250_d_b51: f64 = ((eq82_e1248_d_b51 * s.v[269]) + (eq82_e1248 * s.db[269][51]));
        let eq82_e1250_d_b52: f64 = ((eq82_e1248_d_b52 * s.v[269]) + (eq82_e1248 * s.db[269][52]));
        let eq82_e1250_d_b53: f64 = ((eq82_e1248_d_b53 * s.v[269]) + (eq82_e1248 * s.db[269][53]));
        let eq82_e1250_d_b54: f64 = ((eq82_e1248_d_b54 * s.v[269]) + (eq82_e1248 * s.db[269][54]));
        let eq82_e1253: f64 = (p.p6 * s.v[379]);
        let eq82_e1253_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq82_e1253_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq82_e1253_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq82_e1253_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq82_e1253_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq82_e1253_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq82_e1253_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq82_e1253_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq82_e1253_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq82_e1253_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq82_e1253_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq82_e1253_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq82_e1253_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq82_e1253_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq82_e1253_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq82_e1253_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq82_e1253_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq82_e1253_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq82_e1253_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq82_e1253_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq82_e1253_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq82_e1253_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq82_e1253_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq82_e1253_d_b0: f64 = (p.p6 * s.db[379][0]);
        let eq82_e1253_d_b1: f64 = (p.p6 * s.db[379][1]);
        let eq82_e1253_d_b2: f64 = (p.p6 * s.db[379][2]);
        let eq82_e1253_d_b3: f64 = (p.p6 * s.db[379][3]);
        let eq82_e1253_d_b4: f64 = (p.p6 * s.db[379][4]);
        let eq82_e1253_d_b5: f64 = (p.p6 * s.db[379][5]);
        let eq82_e1253_d_b6: f64 = (p.p6 * s.db[379][6]);
        let eq82_e1253_d_b7: f64 = (p.p6 * s.db[379][7]);
        let eq82_e1253_d_b8: f64 = (p.p6 * s.db[379][8]);
        let eq82_e1253_d_b9: f64 = (p.p6 * s.db[379][9]);
        let eq82_e1253_d_b10: f64 = (p.p6 * s.db[379][10]);
        let eq82_e1253_d_b11: f64 = (p.p6 * s.db[379][11]);
        let eq82_e1253_d_b12: f64 = (p.p6 * s.db[379][12]);
        let eq82_e1253_d_b13: f64 = (p.p6 * s.db[379][13]);
        let eq82_e1253_d_b14: f64 = (p.p6 * s.db[379][14]);
        let eq82_e1253_d_b15: f64 = (p.p6 * s.db[379][15]);
        let eq82_e1253_d_b16: f64 = (p.p6 * s.db[379][16]);
        let eq82_e1253_d_b17: f64 = (p.p6 * s.db[379][17]);
        let eq82_e1253_d_b18: f64 = (p.p6 * s.db[379][18]);
        let eq82_e1253_d_b19: f64 = (p.p6 * s.db[379][19]);
        let eq82_e1253_d_b20: f64 = (p.p6 * s.db[379][20]);
        let eq82_e1253_d_b21: f64 = (p.p6 * s.db[379][21]);
        let eq82_e1253_d_b22: f64 = (p.p6 * s.db[379][22]);
        let eq82_e1253_d_b23: f64 = (p.p6 * s.db[379][23]);
        let eq82_e1253_d_b24: f64 = (p.p6 * s.db[379][24]);
        let eq82_e1253_d_b25: f64 = (p.p6 * s.db[379][25]);
        let eq82_e1253_d_b26: f64 = (p.p6 * s.db[379][26]);
        let eq82_e1253_d_b27: f64 = (p.p6 * s.db[379][27]);
        let eq82_e1253_d_b28: f64 = (p.p6 * s.db[379][28]);
        let eq82_e1253_d_b29: f64 = (p.p6 * s.db[379][29]);
        let eq82_e1253_d_b30: f64 = (p.p6 * s.db[379][30]);
        let eq82_e1253_d_b31: f64 = (p.p6 * s.db[379][31]);
        let eq82_e1253_d_b32: f64 = (p.p6 * s.db[379][32]);
        let eq82_e1253_d_b33: f64 = (p.p6 * s.db[379][33]);
        let eq82_e1253_d_b34: f64 = (p.p6 * s.db[379][34]);
        let eq82_e1253_d_b35: f64 = (p.p6 * s.db[379][35]);
        let eq82_e1253_d_b36: f64 = (p.p6 * s.db[379][36]);
        let eq82_e1253_d_b37: f64 = (p.p6 * s.db[379][37]);
        let eq82_e1253_d_b38: f64 = (p.p6 * s.db[379][38]);
        let eq82_e1253_d_b39: f64 = (p.p6 * s.db[379][39]);
        let eq82_e1253_d_b40: f64 = (p.p6 * s.db[379][40]);
        let eq82_e1253_d_b41: f64 = (p.p6 * s.db[379][41]);
        let eq82_e1253_d_b42: f64 = (p.p6 * s.db[379][42]);
        let eq82_e1253_d_b43: f64 = (p.p6 * s.db[379][43]);
        let eq82_e1253_d_b44: f64 = (p.p6 * s.db[379][44]);
        let eq82_e1253_d_b45: f64 = (p.p6 * s.db[379][45]);
        let eq82_e1253_d_b46: f64 = (p.p6 * s.db[379][46]);
        let eq82_e1253_d_b47: f64 = (p.p6 * s.db[379][47]);
        let eq82_e1253_d_b48: f64 = (p.p6 * s.db[379][48]);
        let eq82_e1253_d_b49: f64 = (p.p6 * s.db[379][49]);
        let eq82_e1253_d_b50: f64 = (p.p6 * s.db[379][50]);
        let eq82_e1253_d_b51: f64 = (p.p6 * s.db[379][51]);
        let eq82_e1253_d_b52: f64 = (p.p6 * s.db[379][52]);
        let eq82_e1253_d_b53: f64 = (p.p6 * s.db[379][53]);
        let eq82_e1253_d_b54: f64 = (p.p6 * s.db[379][54]);
        let eq82_e1255: f64 = (eq82_e1253 * (nv19 - nv20));
        let eq82_e1255_d_n0: f64 = (eq82_e1253_d_n0 * (nv19 - nv20));
        let eq82_e1255_d_n1: f64 = (eq82_e1253_d_n1 * (nv19 - nv20));
        let eq82_e1255_d_n2: f64 = (eq82_e1253_d_n2 * (nv19 - nv20));
        let eq82_e1255_d_n3: f64 = (eq82_e1253_d_n3 * (nv19 - nv20));
        let eq82_e1255_d_n4: f64 = (eq82_e1253_d_n4 * (nv19 - nv20));
        let eq82_e1255_d_n5: f64 = (eq82_e1253_d_n5 * (nv19 - nv20));
        let eq82_e1255_d_n6: f64 = (eq82_e1253_d_n6 * (nv19 - nv20));
        let eq82_e1255_d_n7: f64 = (eq82_e1253_d_n7 * (nv19 - nv20));
        let eq82_e1255_d_n8: f64 = (eq82_e1253_d_n8 * (nv19 - nv20));
        let eq82_e1255_d_n9: f64 = (eq82_e1253_d_n9 * (nv19 - nv20));
        let eq82_e1255_d_n10: f64 = (eq82_e1253_d_n10 * (nv19 - nv20));
        let eq82_e1255_d_n11: f64 = (eq82_e1253_d_n11 * (nv19 - nv20));
        let eq82_e1255_d_n12: f64 = (eq82_e1253_d_n12 * (nv19 - nv20));
        let eq82_e1255_d_n13: f64 = (eq82_e1253_d_n13 * (nv19 - nv20));
        let eq82_e1255_d_n14: f64 = (eq82_e1253_d_n14 * (nv19 - nv20));
        let eq82_e1255_d_n15: f64 = (eq82_e1253_d_n15 * (nv19 - nv20));
        let eq82_e1255_d_n16: f64 = (eq82_e1253_d_n16 * (nv19 - nv20));
        let eq82_e1255_d_n17: f64 = (eq82_e1253_d_n17 * (nv19 - nv20));
        let eq82_e1255_d_n18: f64 = (eq82_e1253_d_n18 * (nv19 - nv20));
        let eq82_e1255_d_n19: f64 = ((eq82_e1253_d_n19 * (nv19 - nv20)) + eq82_e1253);
        let eq82_e1255_d_n20: f64 = ((eq82_e1253_d_n20 * (nv19 - nv20)) + (-eq82_e1253));
        let eq82_e1255_d_n21: f64 = (eq82_e1253_d_n21 * (nv19 - nv20));
        let eq82_e1255_d_n22: f64 = (eq82_e1253_d_n22 * (nv19 - nv20));
        let eq82_e1255_d_b0: f64 = (eq82_e1253_d_b0 * (nv19 - nv20));
        let eq82_e1255_d_b1: f64 = (eq82_e1253_d_b1 * (nv19 - nv20));
        let eq82_e1255_d_b2: f64 = (eq82_e1253_d_b2 * (nv19 - nv20));
        let eq82_e1255_d_b3: f64 = (eq82_e1253_d_b3 * (nv19 - nv20));
        let eq82_e1255_d_b4: f64 = (eq82_e1253_d_b4 * (nv19 - nv20));
        let eq82_e1255_d_b5: f64 = (eq82_e1253_d_b5 * (nv19 - nv20));
        let eq82_e1255_d_b6: f64 = (eq82_e1253_d_b6 * (nv19 - nv20));
        let eq82_e1255_d_b7: f64 = (eq82_e1253_d_b7 * (nv19 - nv20));
        let eq82_e1255_d_b8: f64 = (eq82_e1253_d_b8 * (nv19 - nv20));
        let eq82_e1255_d_b9: f64 = (eq82_e1253_d_b9 * (nv19 - nv20));
        let eq82_e1255_d_b10: f64 = (eq82_e1253_d_b10 * (nv19 - nv20));
        let eq82_e1255_d_b11: f64 = (eq82_e1253_d_b11 * (nv19 - nv20));
        let eq82_e1255_d_b12: f64 = (eq82_e1253_d_b12 * (nv19 - nv20));
        let eq82_e1255_d_b13: f64 = (eq82_e1253_d_b13 * (nv19 - nv20));
        let eq82_e1255_d_b14: f64 = (eq82_e1253_d_b14 * (nv19 - nv20));
        let eq82_e1255_d_b15: f64 = (eq82_e1253_d_b15 * (nv19 - nv20));
        let eq82_e1255_d_b16: f64 = (eq82_e1253_d_b16 * (nv19 - nv20));
        let eq82_e1255_d_b17: f64 = (eq82_e1253_d_b17 * (nv19 - nv20));
        let eq82_e1255_d_b18: f64 = (eq82_e1253_d_b18 * (nv19 - nv20));
        let eq82_e1255_d_b19: f64 = (eq82_e1253_d_b19 * (nv19 - nv20));
        let eq82_e1255_d_b20: f64 = (eq82_e1253_d_b20 * (nv19 - nv20));
        let eq82_e1255_d_b21: f64 = (eq82_e1253_d_b21 * (nv19 - nv20));
        let eq82_e1255_d_b22: f64 = (eq82_e1253_d_b22 * (nv19 - nv20));
        let eq82_e1255_d_b23: f64 = (eq82_e1253_d_b23 * (nv19 - nv20));
        let eq82_e1255_d_b24: f64 = (eq82_e1253_d_b24 * (nv19 - nv20));
        let eq82_e1255_d_b25: f64 = (eq82_e1253_d_b25 * (nv19 - nv20));
        let eq82_e1255_d_b26: f64 = (eq82_e1253_d_b26 * (nv19 - nv20));
        let eq82_e1255_d_b27: f64 = (eq82_e1253_d_b27 * (nv19 - nv20));
        let eq82_e1255_d_b28: f64 = (eq82_e1253_d_b28 * (nv19 - nv20));
        let eq82_e1255_d_b29: f64 = (eq82_e1253_d_b29 * (nv19 - nv20));
        let eq82_e1255_d_b30: f64 = (eq82_e1253_d_b30 * (nv19 - nv20));
        let eq82_e1255_d_b31: f64 = (eq82_e1253_d_b31 * (nv19 - nv20));
        let eq82_e1255_d_b32: f64 = (eq82_e1253_d_b32 * (nv19 - nv20));
        let eq82_e1255_d_b33: f64 = (eq82_e1253_d_b33 * (nv19 - nv20));
        let eq82_e1255_d_b34: f64 = (eq82_e1253_d_b34 * (nv19 - nv20));
        let eq82_e1255_d_b35: f64 = (eq82_e1253_d_b35 * (nv19 - nv20));
        let eq82_e1255_d_b36: f64 = (eq82_e1253_d_b36 * (nv19 - nv20));
        let eq82_e1255_d_b37: f64 = (eq82_e1253_d_b37 * (nv19 - nv20));
        let eq82_e1255_d_b38: f64 = (eq82_e1253_d_b38 * (nv19 - nv20));
        let eq82_e1255_d_b39: f64 = (eq82_e1253_d_b39 * (nv19 - nv20));
        let eq82_e1255_d_b40: f64 = (eq82_e1253_d_b40 * (nv19 - nv20));
        let eq82_e1255_d_b41: f64 = (eq82_e1253_d_b41 * (nv19 - nv20));
        let eq82_e1255_d_b42: f64 = (eq82_e1253_d_b42 * (nv19 - nv20));
        let eq82_e1255_d_b43: f64 = (eq82_e1253_d_b43 * (nv19 - nv20));
        let eq82_e1255_d_b44: f64 = (eq82_e1253_d_b44 * (nv19 - nv20));
        let eq82_e1255_d_b45: f64 = (eq82_e1253_d_b45 * (nv19 - nv20));
        let eq82_e1255_d_b46: f64 = (eq82_e1253_d_b46 * (nv19 - nv20));
        let eq82_e1255_d_b47: f64 = (eq82_e1253_d_b47 * (nv19 - nv20));
        let eq82_e1255_d_b48: f64 = (eq82_e1253_d_b48 * (nv19 - nv20));
        let eq82_e1255_d_b49: f64 = (eq82_e1253_d_b49 * (nv19 - nv20));
        let eq82_e1255_d_b50: f64 = (eq82_e1253_d_b50 * (nv19 - nv20));
        let eq82_e1255_d_b51: f64 = (eq82_e1253_d_b51 * (nv19 - nv20));
        let eq82_e1255_d_b52: f64 = (eq82_e1253_d_b52 * (nv19 - nv20));
        let eq82_e1255_d_b53: f64 = (eq82_e1253_d_b53 * (nv19 - nv20));
        let eq82_e1255_d_b54: f64 = (eq82_e1253_d_b54 * (nv19 - nv20));
        let eq82_e1256: f64 = (eq82_e1250 + eq82_e1255);
        let eq82_e1256_d_n0: f64 = (eq82_e1250_d_n0 + eq82_e1255_d_n0);
        let eq82_e1256_d_n1: f64 = (eq82_e1250_d_n1 + eq82_e1255_d_n1);
        let eq82_e1256_d_n2: f64 = (eq82_e1250_d_n2 + eq82_e1255_d_n2);
        let eq82_e1256_d_n3: f64 = (eq82_e1250_d_n3 + eq82_e1255_d_n3);
        let eq82_e1256_d_n4: f64 = (eq82_e1250_d_n4 + eq82_e1255_d_n4);
        let eq82_e1256_d_n5: f64 = (eq82_e1250_d_n5 + eq82_e1255_d_n5);
        let eq82_e1256_d_n6: f64 = (eq82_e1250_d_n6 + eq82_e1255_d_n6);
        let eq82_e1256_d_n7: f64 = (eq82_e1250_d_n7 + eq82_e1255_d_n7);
        let eq82_e1256_d_n8: f64 = (eq82_e1250_d_n8 + eq82_e1255_d_n8);
        let eq82_e1256_d_n9: f64 = (eq82_e1250_d_n9 + eq82_e1255_d_n9);
        let eq82_e1256_d_n10: f64 = (eq82_e1250_d_n10 + eq82_e1255_d_n10);
        let eq82_e1256_d_n11: f64 = (eq82_e1250_d_n11 + eq82_e1255_d_n11);
        let eq82_e1256_d_n12: f64 = (eq82_e1250_d_n12 + eq82_e1255_d_n12);
        let eq82_e1256_d_n13: f64 = (eq82_e1250_d_n13 + eq82_e1255_d_n13);
        let eq82_e1256_d_n14: f64 = (eq82_e1250_d_n14 + eq82_e1255_d_n14);
        let eq82_e1256_d_n15: f64 = (eq82_e1250_d_n15 + eq82_e1255_d_n15);
        let eq82_e1256_d_n16: f64 = (eq82_e1250_d_n16 + eq82_e1255_d_n16);
        let eq82_e1256_d_n17: f64 = (eq82_e1250_d_n17 + eq82_e1255_d_n17);
        let eq82_e1256_d_n18: f64 = (eq82_e1250_d_n18 + eq82_e1255_d_n18);
        let eq82_e1256_d_n19: f64 = (eq82_e1250_d_n19 + eq82_e1255_d_n19);
        let eq82_e1256_d_n20: f64 = (eq82_e1250_d_n20 + eq82_e1255_d_n20);
        let eq82_e1256_d_n21: f64 = (eq82_e1250_d_n21 + eq82_e1255_d_n21);
        let eq82_e1256_d_n22: f64 = (eq82_e1250_d_n22 + eq82_e1255_d_n22);
        let eq82_e1256_d_b0: f64 = (eq82_e1250_d_b0 + eq82_e1255_d_b0);
        let eq82_e1256_d_b1: f64 = (eq82_e1250_d_b1 + eq82_e1255_d_b1);
        let eq82_e1256_d_b2: f64 = (eq82_e1250_d_b2 + eq82_e1255_d_b2);
        let eq82_e1256_d_b3: f64 = (eq82_e1250_d_b3 + eq82_e1255_d_b3);
        let eq82_e1256_d_b4: f64 = (eq82_e1250_d_b4 + eq82_e1255_d_b4);
        let eq82_e1256_d_b5: f64 = (eq82_e1250_d_b5 + eq82_e1255_d_b5);
        let eq82_e1256_d_b6: f64 = (eq82_e1250_d_b6 + eq82_e1255_d_b6);
        let eq82_e1256_d_b7: f64 = (eq82_e1250_d_b7 + eq82_e1255_d_b7);
        let eq82_e1256_d_b8: f64 = (eq82_e1250_d_b8 + eq82_e1255_d_b8);
        let eq82_e1256_d_b9: f64 = (eq82_e1250_d_b9 + eq82_e1255_d_b9);
        let eq82_e1256_d_b10: f64 = (eq82_e1250_d_b10 + eq82_e1255_d_b10);
        let eq82_e1256_d_b11: f64 = (eq82_e1250_d_b11 + eq82_e1255_d_b11);
        let eq82_e1256_d_b12: f64 = (eq82_e1250_d_b12 + eq82_e1255_d_b12);
        let eq82_e1256_d_b13: f64 = (eq82_e1250_d_b13 + eq82_e1255_d_b13);
        let eq82_e1256_d_b14: f64 = (eq82_e1250_d_b14 + eq82_e1255_d_b14);
        let eq82_e1256_d_b15: f64 = (eq82_e1250_d_b15 + eq82_e1255_d_b15);
        let eq82_e1256_d_b16: f64 = (eq82_e1250_d_b16 + eq82_e1255_d_b16);
        let eq82_e1256_d_b17: f64 = (eq82_e1250_d_b17 + eq82_e1255_d_b17);
        let eq82_e1256_d_b18: f64 = (eq82_e1250_d_b18 + eq82_e1255_d_b18);
        let eq82_e1256_d_b19: f64 = (eq82_e1250_d_b19 + eq82_e1255_d_b19);
        let eq82_e1256_d_b20: f64 = (eq82_e1250_d_b20 + eq82_e1255_d_b20);
        let eq82_e1256_d_b21: f64 = (eq82_e1250_d_b21 + eq82_e1255_d_b21);
        let eq82_e1256_d_b22: f64 = (eq82_e1250_d_b22 + eq82_e1255_d_b22);
        let eq82_e1256_d_b23: f64 = (eq82_e1250_d_b23 + eq82_e1255_d_b23);
        let eq82_e1256_d_b24: f64 = (eq82_e1250_d_b24 + eq82_e1255_d_b24);
        let eq82_e1256_d_b25: f64 = (eq82_e1250_d_b25 + eq82_e1255_d_b25);
        let eq82_e1256_d_b26: f64 = (eq82_e1250_d_b26 + eq82_e1255_d_b26);
        let eq82_e1256_d_b27: f64 = (eq82_e1250_d_b27 + eq82_e1255_d_b27);
        let eq82_e1256_d_b28: f64 = (eq82_e1250_d_b28 + eq82_e1255_d_b28);
        let eq82_e1256_d_b29: f64 = (eq82_e1250_d_b29 + eq82_e1255_d_b29);
        let eq82_e1256_d_b30: f64 = (eq82_e1250_d_b30 + eq82_e1255_d_b30);
        let eq82_e1256_d_b31: f64 = (eq82_e1250_d_b31 + eq82_e1255_d_b31);
        let eq82_e1256_d_b32: f64 = (eq82_e1250_d_b32 + eq82_e1255_d_b32);
        let eq82_e1256_d_b33: f64 = (eq82_e1250_d_b33 + eq82_e1255_d_b33);
        let eq82_e1256_d_b34: f64 = (eq82_e1250_d_b34 + eq82_e1255_d_b34);
        let eq82_e1256_d_b35: f64 = (eq82_e1250_d_b35 + eq82_e1255_d_b35);
        let eq82_e1256_d_b36: f64 = (eq82_e1250_d_b36 + eq82_e1255_d_b36);
        let eq82_e1256_d_b37: f64 = (eq82_e1250_d_b37 + eq82_e1255_d_b37);
        let eq82_e1256_d_b38: f64 = (eq82_e1250_d_b38 + eq82_e1255_d_b38);
        let eq82_e1256_d_b39: f64 = (eq82_e1250_d_b39 + eq82_e1255_d_b39);
        let eq82_e1256_d_b40: f64 = (eq82_e1250_d_b40 + eq82_e1255_d_b40);
        let eq82_e1256_d_b41: f64 = (eq82_e1250_d_b41 + eq82_e1255_d_b41);
        let eq82_e1256_d_b42: f64 = (eq82_e1250_d_b42 + eq82_e1255_d_b42);
        let eq82_e1256_d_b43: f64 = (eq82_e1250_d_b43 + eq82_e1255_d_b43);
        let eq82_e1256_d_b44: f64 = (eq82_e1250_d_b44 + eq82_e1255_d_b44);
        let eq82_e1256_d_b45: f64 = (eq82_e1250_d_b45 + eq82_e1255_d_b45);
        let eq82_e1256_d_b46: f64 = (eq82_e1250_d_b46 + eq82_e1255_d_b46);
        let eq82_e1256_d_b47: f64 = (eq82_e1250_d_b47 + eq82_e1255_d_b47);
        let eq82_e1256_d_b48: f64 = (eq82_e1250_d_b48 + eq82_e1255_d_b48);
        let eq82_e1256_d_b49: f64 = (eq82_e1250_d_b49 + eq82_e1255_d_b49);
        let eq82_e1256_d_b50: f64 = (eq82_e1250_d_b50 + eq82_e1255_d_b50);
        let eq82_e1256_d_b51: f64 = (eq82_e1250_d_b51 + eq82_e1255_d_b51);
        let eq82_e1256_d_b52: f64 = (eq82_e1250_d_b52 + eq82_e1255_d_b52);
        let eq82_e1256_d_b53: f64 = (eq82_e1250_d_b53 + eq82_e1255_d_b53);
        let eq82_e1256_d_b54: f64 = (eq82_e1250_d_b54 + eq82_e1255_d_b54);
        (eq82_e1256, eq82_e1256_d_n0, eq82_e1256_d_n1, eq82_e1256_d_n2, eq82_e1256_d_n3, eq82_e1256_d_n4, eq82_e1256_d_n5, eq82_e1256_d_n6, eq82_e1256_d_n7, eq82_e1256_d_n8, eq82_e1256_d_n9, eq82_e1256_d_n10, eq82_e1256_d_n11, eq82_e1256_d_n12, eq82_e1256_d_n13, eq82_e1256_d_n14, eq82_e1256_d_n15, eq82_e1256_d_n16, eq82_e1256_d_n17, eq82_e1256_d_n18, eq82_e1256_d_n19, eq82_e1256_d_n20, eq82_e1256_d_n21, eq82_e1256_d_n22, eq82_e1256_d_b0, eq82_e1256_d_b1, eq82_e1256_d_b2, eq82_e1256_d_b3, eq82_e1256_d_b4, eq82_e1256_d_b5, eq82_e1256_d_b6, eq82_e1256_d_b7, eq82_e1256_d_b8, eq82_e1256_d_b9, eq82_e1256_d_b10, eq82_e1256_d_b11, eq82_e1256_d_b12, eq82_e1256_d_b13, eq82_e1256_d_b14, eq82_e1256_d_b15, eq82_e1256_d_b16, eq82_e1256_d_b17, eq82_e1256_d_b18, eq82_e1256_d_b19, eq82_e1256_d_b20, eq82_e1256_d_b21, eq82_e1256_d_b22, eq82_e1256_d_b23, eq82_e1256_d_b24, eq82_e1256_d_b25, eq82_e1256_d_b26, eq82_e1256_d_b27, eq82_e1256_d_b28, eq82_e1256_d_b29, eq82_e1256_d_b30, eq82_e1256_d_b31, eq82_e1256_d_b32, eq82_e1256_d_b33, eq82_e1256_d_b34, eq82_e1256_d_b35, eq82_e1256_d_b36, eq82_e1256_d_b37, eq82_e1256_d_b38, eq82_e1256_d_b39, eq82_e1256_d_b40, eq82_e1256_d_b41, eq82_e1256_d_b42, eq82_e1256_d_b43, eq82_e1256_d_b44, eq82_e1256_d_b45, eq82_e1256_d_b46, eq82_e1256_d_b47, eq82_e1256_d_b48, eq82_e1256_d_b49, eq82_e1256_d_b50, eq82_e1256_d_b51, eq82_e1256_d_b52, eq82_e1256_d_b53, eq82_e1256_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e1258;
        let eq82_node_derivatives: [f64; 23] = [eq82_e1258_d_n0, eq82_e1258_d_n1, eq82_e1258_d_n2, eq82_e1258_d_n3, eq82_e1258_d_n4, eq82_e1258_d_n5, eq82_e1258_d_n6, eq82_e1258_d_n7, eq82_e1258_d_n8, eq82_e1258_d_n9, eq82_e1258_d_n10, eq82_e1258_d_n11, eq82_e1258_d_n12, eq82_e1258_d_n13, eq82_e1258_d_n14, eq82_e1258_d_n15, eq82_e1258_d_n16, eq82_e1258_d_n17, eq82_e1258_d_n18, eq82_e1258_d_n19, eq82_e1258_d_n20, eq82_e1258_d_n21, eq82_e1258_d_n22];
        let eq82_branch_derivatives: [f64; 55] = [eq82_e1258_d_b0, eq82_e1258_d_b1, eq82_e1258_d_b2, eq82_e1258_d_b3, eq82_e1258_d_b4, eq82_e1258_d_b5, eq82_e1258_d_b6, eq82_e1258_d_b7, eq82_e1258_d_b8, eq82_e1258_d_b9, eq82_e1258_d_b10, eq82_e1258_d_b11, eq82_e1258_d_b12, eq82_e1258_d_b13, eq82_e1258_d_b14, eq82_e1258_d_b15, eq82_e1258_d_b16, eq82_e1258_d_b17, eq82_e1258_d_b18, eq82_e1258_d_b19, eq82_e1258_d_b20, eq82_e1258_d_b21, eq82_e1258_d_b22, eq82_e1258_d_b23, eq82_e1258_d_b24, eq82_e1258_d_b25, eq82_e1258_d_b26, eq82_e1258_d_b27, eq82_e1258_d_b28, eq82_e1258_d_b29, eq82_e1258_d_b30, eq82_e1258_d_b31, eq82_e1258_d_b32, eq82_e1258_d_b33, eq82_e1258_d_b34, eq82_e1258_d_b35, eq82_e1258_d_b36, eq82_e1258_d_b37, eq82_e1258_d_b38, eq82_e1258_d_b39, eq82_e1258_d_b40, eq82_e1258_d_b41, eq82_e1258_d_b42, eq82_e1258_d_b43, eq82_e1258_d_b44, eq82_e1258_d_b45, eq82_e1258_d_b46, eq82_e1258_d_b47, eq82_e1258_d_b48, eq82_e1258_d_b49, eq82_e1258_d_b50, eq82_e1258_d_b51, eq82_e1258_d_b52, eq82_e1258_d_b53, eq82_e1258_d_b54];
        stamper.stamp_current_dense_local(
            Some(19),
            Some(20),
            multiplicity * (eq82_value),
            &eq82_node_derivatives,
            &eq82_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1265,) = {
    if (s.b[478] && (!s.b[479])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq83_value: f64 = eq83_e1265;
        stamper.stamp_potential_const_local(
            40,
            eq83_value,
        );
        let (eq84_e1270,) = {
    if (!s.b[478]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq84_value: f64 = eq84_e1270;
        stamper.stamp_potential_const_local(
            41,
            eq84_value,
        );
        let (eq85_e1278,) = {
    if ((!s.b[478]) && (!s.b[487])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq85_value: f64 = eq85_e1278;
        stamper.stamp_potential_const_local(
            42,
            eq85_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_16(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq86_e1294, eq86_e1294_d_n0, eq86_e1294_d_n1, eq86_e1294_d_n2, eq86_e1294_d_n3, eq86_e1294_d_n4, eq86_e1294_d_n5, eq86_e1294_d_n6, eq86_e1294_d_n7, eq86_e1294_d_n8, eq86_e1294_d_n9, eq86_e1294_d_n10, eq86_e1294_d_n11, eq86_e1294_d_n12, eq86_e1294_d_n13, eq86_e1294_d_n14, eq86_e1294_d_n15, eq86_e1294_d_n16, eq86_e1294_d_n17, eq86_e1294_d_n18, eq86_e1294_d_n19, eq86_e1294_d_n20, eq86_e1294_d_n21, eq86_e1294_d_n22, eq86_e1294_d_b0, eq86_e1294_d_b1, eq86_e1294_d_b2, eq86_e1294_d_b3, eq86_e1294_d_b4, eq86_e1294_d_b5, eq86_e1294_d_b6, eq86_e1294_d_b7, eq86_e1294_d_b8, eq86_e1294_d_b9, eq86_e1294_d_b10, eq86_e1294_d_b11, eq86_e1294_d_b12, eq86_e1294_d_b13, eq86_e1294_d_b14, eq86_e1294_d_b15, eq86_e1294_d_b16, eq86_e1294_d_b17, eq86_e1294_d_b18, eq86_e1294_d_b19, eq86_e1294_d_b20, eq86_e1294_d_b21, eq86_e1294_d_b22, eq86_e1294_d_b23, eq86_e1294_d_b24, eq86_e1294_d_b25, eq86_e1294_d_b26, eq86_e1294_d_b27, eq86_e1294_d_b28, eq86_e1294_d_b29, eq86_e1294_d_b30, eq86_e1294_d_b31, eq86_e1294_d_b32, eq86_e1294_d_b33, eq86_e1294_d_b34, eq86_e1294_d_b35, eq86_e1294_d_b36, eq86_e1294_d_b37, eq86_e1294_d_b38, eq86_e1294_d_b39, eq86_e1294_d_b40, eq86_e1294_d_b41, eq86_e1294_d_b42, eq86_e1294_d_b43, eq86_e1294_d_b44, eq86_e1294_d_b45, eq86_e1294_d_b46, eq86_e1294_d_b47, eq86_e1294_d_b48, eq86_e1294_d_b49, eq86_e1294_d_b50, eq86_e1294_d_b51, eq86_e1294_d_b52, eq86_e1294_d_b53, eq86_e1294_d_b54,) = {
    if (s.b[493] && s.b[494]) {
        let eq86_e1284: f64 = (p.p6 * s.v[64]);
        let eq86_e1284_d_n0: f64 = (p.p6 * s.dn[64][0]);
        let eq86_e1284_d_n1: f64 = (p.p6 * s.dn[64][1]);
        let eq86_e1284_d_n2: f64 = (p.p6 * s.dn[64][2]);
        let eq86_e1284_d_n3: f64 = (p.p6 * s.dn[64][3]);
        let eq86_e1284_d_n4: f64 = (p.p6 * s.dn[64][4]);
        let eq86_e1284_d_n5: f64 = (p.p6 * s.dn[64][5]);
        let eq86_e1284_d_n6: f64 = (p.p6 * s.dn[64][6]);
        let eq86_e1284_d_n7: f64 = (p.p6 * s.dn[64][7]);
        let eq86_e1284_d_n8: f64 = (p.p6 * s.dn[64][8]);
        let eq86_e1284_d_n9: f64 = (p.p6 * s.dn[64][9]);
        let eq86_e1284_d_n10: f64 = (p.p6 * s.dn[64][10]);
        let eq86_e1284_d_n11: f64 = (p.p6 * s.dn[64][11]);
        let eq86_e1284_d_n12: f64 = (p.p6 * s.dn[64][12]);
        let eq86_e1284_d_n13: f64 = (p.p6 * s.dn[64][13]);
        let eq86_e1284_d_n14: f64 = (p.p6 * s.dn[64][14]);
        let eq86_e1284_d_n15: f64 = (p.p6 * s.dn[64][15]);
        let eq86_e1284_d_n16: f64 = (p.p6 * s.dn[64][16]);
        let eq86_e1284_d_n17: f64 = (p.p6 * s.dn[64][17]);
        let eq86_e1284_d_n18: f64 = (p.p6 * s.dn[64][18]);
        let eq86_e1284_d_n19: f64 = (p.p6 * s.dn[64][19]);
        let eq86_e1284_d_n20: f64 = (p.p6 * s.dn[64][20]);
        let eq86_e1284_d_n21: f64 = (p.p6 * s.dn[64][21]);
        let eq86_e1284_d_n22: f64 = (p.p6 * s.dn[64][22]);
        let eq86_e1284_d_b0: f64 = (p.p6 * s.db[64][0]);
        let eq86_e1284_d_b1: f64 = (p.p6 * s.db[64][1]);
        let eq86_e1284_d_b2: f64 = (p.p6 * s.db[64][2]);
        let eq86_e1284_d_b3: f64 = (p.p6 * s.db[64][3]);
        let eq86_e1284_d_b4: f64 = (p.p6 * s.db[64][4]);
        let eq86_e1284_d_b5: f64 = (p.p6 * s.db[64][5]);
        let eq86_e1284_d_b6: f64 = (p.p6 * s.db[64][6]);
        let eq86_e1284_d_b7: f64 = (p.p6 * s.db[64][7]);
        let eq86_e1284_d_b8: f64 = (p.p6 * s.db[64][8]);
        let eq86_e1284_d_b9: f64 = (p.p6 * s.db[64][9]);
        let eq86_e1284_d_b10: f64 = (p.p6 * s.db[64][10]);
        let eq86_e1284_d_b11: f64 = (p.p6 * s.db[64][11]);
        let eq86_e1284_d_b12: f64 = (p.p6 * s.db[64][12]);
        let eq86_e1284_d_b13: f64 = (p.p6 * s.db[64][13]);
        let eq86_e1284_d_b14: f64 = (p.p6 * s.db[64][14]);
        let eq86_e1284_d_b15: f64 = (p.p6 * s.db[64][15]);
        let eq86_e1284_d_b16: f64 = (p.p6 * s.db[64][16]);
        let eq86_e1284_d_b17: f64 = (p.p6 * s.db[64][17]);
        let eq86_e1284_d_b18: f64 = (p.p6 * s.db[64][18]);
        let eq86_e1284_d_b19: f64 = (p.p6 * s.db[64][19]);
        let eq86_e1284_d_b20: f64 = (p.p6 * s.db[64][20]);
        let eq86_e1284_d_b21: f64 = (p.p6 * s.db[64][21]);
        let eq86_e1284_d_b22: f64 = (p.p6 * s.db[64][22]);
        let eq86_e1284_d_b23: f64 = (p.p6 * s.db[64][23]);
        let eq86_e1284_d_b24: f64 = (p.p6 * s.db[64][24]);
        let eq86_e1284_d_b25: f64 = (p.p6 * s.db[64][25]);
        let eq86_e1284_d_b26: f64 = (p.p6 * s.db[64][26]);
        let eq86_e1284_d_b27: f64 = (p.p6 * s.db[64][27]);
        let eq86_e1284_d_b28: f64 = (p.p6 * s.db[64][28]);
        let eq86_e1284_d_b29: f64 = (p.p6 * s.db[64][29]);
        let eq86_e1284_d_b30: f64 = (p.p6 * s.db[64][30]);
        let eq86_e1284_d_b31: f64 = (p.p6 * s.db[64][31]);
        let eq86_e1284_d_b32: f64 = (p.p6 * s.db[64][32]);
        let eq86_e1284_d_b33: f64 = (p.p6 * s.db[64][33]);
        let eq86_e1284_d_b34: f64 = (p.p6 * s.db[64][34]);
        let eq86_e1284_d_b35: f64 = (p.p6 * s.db[64][35]);
        let eq86_e1284_d_b36: f64 = (p.p6 * s.db[64][36]);
        let eq86_e1284_d_b37: f64 = (p.p6 * s.db[64][37]);
        let eq86_e1284_d_b38: f64 = (p.p6 * s.db[64][38]);
        let eq86_e1284_d_b39: f64 = (p.p6 * s.db[64][39]);
        let eq86_e1284_d_b40: f64 = (p.p6 * s.db[64][40]);
        let eq86_e1284_d_b41: f64 = (p.p6 * s.db[64][41]);
        let eq86_e1284_d_b42: f64 = (p.p6 * s.db[64][42]);
        let eq86_e1284_d_b43: f64 = (p.p6 * s.db[64][43]);
        let eq86_e1284_d_b44: f64 = (p.p6 * s.db[64][44]);
        let eq86_e1284_d_b45: f64 = (p.p6 * s.db[64][45]);
        let eq86_e1284_d_b46: f64 = (p.p6 * s.db[64][46]);
        let eq86_e1284_d_b47: f64 = (p.p6 * s.db[64][47]);
        let eq86_e1284_d_b48: f64 = (p.p6 * s.db[64][48]);
        let eq86_e1284_d_b49: f64 = (p.p6 * s.db[64][49]);
        let eq86_e1284_d_b50: f64 = (p.p6 * s.db[64][50]);
        let eq86_e1284_d_b51: f64 = (p.p6 * s.db[64][51]);
        let eq86_e1284_d_b52: f64 = (p.p6 * s.db[64][52]);
        let eq86_e1284_d_b53: f64 = (p.p6 * s.db[64][53]);
        let eq86_e1284_d_b54: f64 = (p.p6 * s.db[64][54]);
        let eq86_e1286: f64 = (eq86_e1284 * s.v[281]);
        let eq86_e1286_d_n0: f64 = ((eq86_e1284_d_n0 * s.v[281]) + (eq86_e1284 * s.dn[281][0]));
        let eq86_e1286_d_n1: f64 = ((eq86_e1284_d_n1 * s.v[281]) + (eq86_e1284 * s.dn[281][1]));
        let eq86_e1286_d_n2: f64 = ((eq86_e1284_d_n2 * s.v[281]) + (eq86_e1284 * s.dn[281][2]));
        let eq86_e1286_d_n3: f64 = ((eq86_e1284_d_n3 * s.v[281]) + (eq86_e1284 * s.dn[281][3]));
        let eq86_e1286_d_n4: f64 = ((eq86_e1284_d_n4 * s.v[281]) + (eq86_e1284 * s.dn[281][4]));
        let eq86_e1286_d_n5: f64 = ((eq86_e1284_d_n5 * s.v[281]) + (eq86_e1284 * s.dn[281][5]));
        let eq86_e1286_d_n6: f64 = ((eq86_e1284_d_n6 * s.v[281]) + (eq86_e1284 * s.dn[281][6]));
        let eq86_e1286_d_n7: f64 = ((eq86_e1284_d_n7 * s.v[281]) + (eq86_e1284 * s.dn[281][7]));
        let eq86_e1286_d_n8: f64 = ((eq86_e1284_d_n8 * s.v[281]) + (eq86_e1284 * s.dn[281][8]));
        let eq86_e1286_d_n9: f64 = ((eq86_e1284_d_n9 * s.v[281]) + (eq86_e1284 * s.dn[281][9]));
        let eq86_e1286_d_n10: f64 = ((eq86_e1284_d_n10 * s.v[281]) + (eq86_e1284 * s.dn[281][10]));
        let eq86_e1286_d_n11: f64 = ((eq86_e1284_d_n11 * s.v[281]) + (eq86_e1284 * s.dn[281][11]));
        let eq86_e1286_d_n12: f64 = ((eq86_e1284_d_n12 * s.v[281]) + (eq86_e1284 * s.dn[281][12]));
        let eq86_e1286_d_n13: f64 = ((eq86_e1284_d_n13 * s.v[281]) + (eq86_e1284 * s.dn[281][13]));
        let eq86_e1286_d_n14: f64 = ((eq86_e1284_d_n14 * s.v[281]) + (eq86_e1284 * s.dn[281][14]));
        let eq86_e1286_d_n15: f64 = ((eq86_e1284_d_n15 * s.v[281]) + (eq86_e1284 * s.dn[281][15]));
        let eq86_e1286_d_n16: f64 = ((eq86_e1284_d_n16 * s.v[281]) + (eq86_e1284 * s.dn[281][16]));
        let eq86_e1286_d_n17: f64 = ((eq86_e1284_d_n17 * s.v[281]) + (eq86_e1284 * s.dn[281][17]));
        let eq86_e1286_d_n18: f64 = ((eq86_e1284_d_n18 * s.v[281]) + (eq86_e1284 * s.dn[281][18]));
        let eq86_e1286_d_n19: f64 = ((eq86_e1284_d_n19 * s.v[281]) + (eq86_e1284 * s.dn[281][19]));
        let eq86_e1286_d_n20: f64 = ((eq86_e1284_d_n20 * s.v[281]) + (eq86_e1284 * s.dn[281][20]));
        let eq86_e1286_d_n21: f64 = ((eq86_e1284_d_n21 * s.v[281]) + (eq86_e1284 * s.dn[281][21]));
        let eq86_e1286_d_n22: f64 = ((eq86_e1284_d_n22 * s.v[281]) + (eq86_e1284 * s.dn[281][22]));
        let eq86_e1286_d_b0: f64 = ((eq86_e1284_d_b0 * s.v[281]) + (eq86_e1284 * s.db[281][0]));
        let eq86_e1286_d_b1: f64 = ((eq86_e1284_d_b1 * s.v[281]) + (eq86_e1284 * s.db[281][1]));
        let eq86_e1286_d_b2: f64 = ((eq86_e1284_d_b2 * s.v[281]) + (eq86_e1284 * s.db[281][2]));
        let eq86_e1286_d_b3: f64 = ((eq86_e1284_d_b3 * s.v[281]) + (eq86_e1284 * s.db[281][3]));
        let eq86_e1286_d_b4: f64 = ((eq86_e1284_d_b4 * s.v[281]) + (eq86_e1284 * s.db[281][4]));
        let eq86_e1286_d_b5: f64 = ((eq86_e1284_d_b5 * s.v[281]) + (eq86_e1284 * s.db[281][5]));
        let eq86_e1286_d_b6: f64 = ((eq86_e1284_d_b6 * s.v[281]) + (eq86_e1284 * s.db[281][6]));
        let eq86_e1286_d_b7: f64 = ((eq86_e1284_d_b7 * s.v[281]) + (eq86_e1284 * s.db[281][7]));
        let eq86_e1286_d_b8: f64 = ((eq86_e1284_d_b8 * s.v[281]) + (eq86_e1284 * s.db[281][8]));
        let eq86_e1286_d_b9: f64 = ((eq86_e1284_d_b9 * s.v[281]) + (eq86_e1284 * s.db[281][9]));
        let eq86_e1286_d_b10: f64 = ((eq86_e1284_d_b10 * s.v[281]) + (eq86_e1284 * s.db[281][10]));
        let eq86_e1286_d_b11: f64 = ((eq86_e1284_d_b11 * s.v[281]) + (eq86_e1284 * s.db[281][11]));
        let eq86_e1286_d_b12: f64 = ((eq86_e1284_d_b12 * s.v[281]) + (eq86_e1284 * s.db[281][12]));
        let eq86_e1286_d_b13: f64 = ((eq86_e1284_d_b13 * s.v[281]) + (eq86_e1284 * s.db[281][13]));
        let eq86_e1286_d_b14: f64 = ((eq86_e1284_d_b14 * s.v[281]) + (eq86_e1284 * s.db[281][14]));
        let eq86_e1286_d_b15: f64 = ((eq86_e1284_d_b15 * s.v[281]) + (eq86_e1284 * s.db[281][15]));
        let eq86_e1286_d_b16: f64 = ((eq86_e1284_d_b16 * s.v[281]) + (eq86_e1284 * s.db[281][16]));
        let eq86_e1286_d_b17: f64 = ((eq86_e1284_d_b17 * s.v[281]) + (eq86_e1284 * s.db[281][17]));
        let eq86_e1286_d_b18: f64 = ((eq86_e1284_d_b18 * s.v[281]) + (eq86_e1284 * s.db[281][18]));
        let eq86_e1286_d_b19: f64 = ((eq86_e1284_d_b19 * s.v[281]) + (eq86_e1284 * s.db[281][19]));
        let eq86_e1286_d_b20: f64 = ((eq86_e1284_d_b20 * s.v[281]) + (eq86_e1284 * s.db[281][20]));
        let eq86_e1286_d_b21: f64 = ((eq86_e1284_d_b21 * s.v[281]) + (eq86_e1284 * s.db[281][21]));
        let eq86_e1286_d_b22: f64 = ((eq86_e1284_d_b22 * s.v[281]) + (eq86_e1284 * s.db[281][22]));
        let eq86_e1286_d_b23: f64 = ((eq86_e1284_d_b23 * s.v[281]) + (eq86_e1284 * s.db[281][23]));
        let eq86_e1286_d_b24: f64 = ((eq86_e1284_d_b24 * s.v[281]) + (eq86_e1284 * s.db[281][24]));
        let eq86_e1286_d_b25: f64 = ((eq86_e1284_d_b25 * s.v[281]) + (eq86_e1284 * s.db[281][25]));
        let eq86_e1286_d_b26: f64 = ((eq86_e1284_d_b26 * s.v[281]) + (eq86_e1284 * s.db[281][26]));
        let eq86_e1286_d_b27: f64 = ((eq86_e1284_d_b27 * s.v[281]) + (eq86_e1284 * s.db[281][27]));
        let eq86_e1286_d_b28: f64 = ((eq86_e1284_d_b28 * s.v[281]) + (eq86_e1284 * s.db[281][28]));
        let eq86_e1286_d_b29: f64 = ((eq86_e1284_d_b29 * s.v[281]) + (eq86_e1284 * s.db[281][29]));
        let eq86_e1286_d_b30: f64 = ((eq86_e1284_d_b30 * s.v[281]) + (eq86_e1284 * s.db[281][30]));
        let eq86_e1286_d_b31: f64 = ((eq86_e1284_d_b31 * s.v[281]) + (eq86_e1284 * s.db[281][31]));
        let eq86_e1286_d_b32: f64 = ((eq86_e1284_d_b32 * s.v[281]) + (eq86_e1284 * s.db[281][32]));
        let eq86_e1286_d_b33: f64 = ((eq86_e1284_d_b33 * s.v[281]) + (eq86_e1284 * s.db[281][33]));
        let eq86_e1286_d_b34: f64 = ((eq86_e1284_d_b34 * s.v[281]) + (eq86_e1284 * s.db[281][34]));
        let eq86_e1286_d_b35: f64 = ((eq86_e1284_d_b35 * s.v[281]) + (eq86_e1284 * s.db[281][35]));
        let eq86_e1286_d_b36: f64 = ((eq86_e1284_d_b36 * s.v[281]) + (eq86_e1284 * s.db[281][36]));
        let eq86_e1286_d_b37: f64 = ((eq86_e1284_d_b37 * s.v[281]) + (eq86_e1284 * s.db[281][37]));
        let eq86_e1286_d_b38: f64 = ((eq86_e1284_d_b38 * s.v[281]) + (eq86_e1284 * s.db[281][38]));
        let eq86_e1286_d_b39: f64 = ((eq86_e1284_d_b39 * s.v[281]) + (eq86_e1284 * s.db[281][39]));
        let eq86_e1286_d_b40: f64 = ((eq86_e1284_d_b40 * s.v[281]) + (eq86_e1284 * s.db[281][40]));
        let eq86_e1286_d_b41: f64 = ((eq86_e1284_d_b41 * s.v[281]) + (eq86_e1284 * s.db[281][41]));
        let eq86_e1286_d_b42: f64 = ((eq86_e1284_d_b42 * s.v[281]) + (eq86_e1284 * s.db[281][42]));
        let eq86_e1286_d_b43: f64 = ((eq86_e1284_d_b43 * s.v[281]) + (eq86_e1284 * s.db[281][43]));
        let eq86_e1286_d_b44: f64 = ((eq86_e1284_d_b44 * s.v[281]) + (eq86_e1284 * s.db[281][44]));
        let eq86_e1286_d_b45: f64 = ((eq86_e1284_d_b45 * s.v[281]) + (eq86_e1284 * s.db[281][45]));
        let eq86_e1286_d_b46: f64 = ((eq86_e1284_d_b46 * s.v[281]) + (eq86_e1284 * s.db[281][46]));
        let eq86_e1286_d_b47: f64 = ((eq86_e1284_d_b47 * s.v[281]) + (eq86_e1284 * s.db[281][47]));
        let eq86_e1286_d_b48: f64 = ((eq86_e1284_d_b48 * s.v[281]) + (eq86_e1284 * s.db[281][48]));
        let eq86_e1286_d_b49: f64 = ((eq86_e1284_d_b49 * s.v[281]) + (eq86_e1284 * s.db[281][49]));
        let eq86_e1286_d_b50: f64 = ((eq86_e1284_d_b50 * s.v[281]) + (eq86_e1284 * s.db[281][50]));
        let eq86_e1286_d_b51: f64 = ((eq86_e1284_d_b51 * s.v[281]) + (eq86_e1284 * s.db[281][51]));
        let eq86_e1286_d_b52: f64 = ((eq86_e1284_d_b52 * s.v[281]) + (eq86_e1284 * s.db[281][52]));
        let eq86_e1286_d_b53: f64 = ((eq86_e1284_d_b53 * s.v[281]) + (eq86_e1284 * s.db[281][53]));
        let eq86_e1286_d_b54: f64 = ((eq86_e1284_d_b54 * s.v[281]) + (eq86_e1284 * s.db[281][54]));
        let eq86_e1289: f64 = (p.p6 * s.v[379]);
        let eq86_e1289_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq86_e1289_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq86_e1289_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq86_e1289_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq86_e1289_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq86_e1289_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq86_e1289_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq86_e1289_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq86_e1289_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq86_e1289_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq86_e1289_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq86_e1289_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq86_e1289_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq86_e1289_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq86_e1289_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq86_e1289_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq86_e1289_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq86_e1289_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq86_e1289_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq86_e1289_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq86_e1289_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq86_e1289_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq86_e1289_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq86_e1289_d_b0: f64 = (p.p6 * s.db[379][0]);
        let eq86_e1289_d_b1: f64 = (p.p6 * s.db[379][1]);
        let eq86_e1289_d_b2: f64 = (p.p6 * s.db[379][2]);
        let eq86_e1289_d_b3: f64 = (p.p6 * s.db[379][3]);
        let eq86_e1289_d_b4: f64 = (p.p6 * s.db[379][4]);
        let eq86_e1289_d_b5: f64 = (p.p6 * s.db[379][5]);
        let eq86_e1289_d_b6: f64 = (p.p6 * s.db[379][6]);
        let eq86_e1289_d_b7: f64 = (p.p6 * s.db[379][7]);
        let eq86_e1289_d_b8: f64 = (p.p6 * s.db[379][8]);
        let eq86_e1289_d_b9: f64 = (p.p6 * s.db[379][9]);
        let eq86_e1289_d_b10: f64 = (p.p6 * s.db[379][10]);
        let eq86_e1289_d_b11: f64 = (p.p6 * s.db[379][11]);
        let eq86_e1289_d_b12: f64 = (p.p6 * s.db[379][12]);
        let eq86_e1289_d_b13: f64 = (p.p6 * s.db[379][13]);
        let eq86_e1289_d_b14: f64 = (p.p6 * s.db[379][14]);
        let eq86_e1289_d_b15: f64 = (p.p6 * s.db[379][15]);
        let eq86_e1289_d_b16: f64 = (p.p6 * s.db[379][16]);
        let eq86_e1289_d_b17: f64 = (p.p6 * s.db[379][17]);
        let eq86_e1289_d_b18: f64 = (p.p6 * s.db[379][18]);
        let eq86_e1289_d_b19: f64 = (p.p6 * s.db[379][19]);
        let eq86_e1289_d_b20: f64 = (p.p6 * s.db[379][20]);
        let eq86_e1289_d_b21: f64 = (p.p6 * s.db[379][21]);
        let eq86_e1289_d_b22: f64 = (p.p6 * s.db[379][22]);
        let eq86_e1289_d_b23: f64 = (p.p6 * s.db[379][23]);
        let eq86_e1289_d_b24: f64 = (p.p6 * s.db[379][24]);
        let eq86_e1289_d_b25: f64 = (p.p6 * s.db[379][25]);
        let eq86_e1289_d_b26: f64 = (p.p6 * s.db[379][26]);
        let eq86_e1289_d_b27: f64 = (p.p6 * s.db[379][27]);
        let eq86_e1289_d_b28: f64 = (p.p6 * s.db[379][28]);
        let eq86_e1289_d_b29: f64 = (p.p6 * s.db[379][29]);
        let eq86_e1289_d_b30: f64 = (p.p6 * s.db[379][30]);
        let eq86_e1289_d_b31: f64 = (p.p6 * s.db[379][31]);
        let eq86_e1289_d_b32: f64 = (p.p6 * s.db[379][32]);
        let eq86_e1289_d_b33: f64 = (p.p6 * s.db[379][33]);
        let eq86_e1289_d_b34: f64 = (p.p6 * s.db[379][34]);
        let eq86_e1289_d_b35: f64 = (p.p6 * s.db[379][35]);
        let eq86_e1289_d_b36: f64 = (p.p6 * s.db[379][36]);
        let eq86_e1289_d_b37: f64 = (p.p6 * s.db[379][37]);
        let eq86_e1289_d_b38: f64 = (p.p6 * s.db[379][38]);
        let eq86_e1289_d_b39: f64 = (p.p6 * s.db[379][39]);
        let eq86_e1289_d_b40: f64 = (p.p6 * s.db[379][40]);
        let eq86_e1289_d_b41: f64 = (p.p6 * s.db[379][41]);
        let eq86_e1289_d_b42: f64 = (p.p6 * s.db[379][42]);
        let eq86_e1289_d_b43: f64 = (p.p6 * s.db[379][43]);
        let eq86_e1289_d_b44: f64 = (p.p6 * s.db[379][44]);
        let eq86_e1289_d_b45: f64 = (p.p6 * s.db[379][45]);
        let eq86_e1289_d_b46: f64 = (p.p6 * s.db[379][46]);
        let eq86_e1289_d_b47: f64 = (p.p6 * s.db[379][47]);
        let eq86_e1289_d_b48: f64 = (p.p6 * s.db[379][48]);
        let eq86_e1289_d_b49: f64 = (p.p6 * s.db[379][49]);
        let eq86_e1289_d_b50: f64 = (p.p6 * s.db[379][50]);
        let eq86_e1289_d_b51: f64 = (p.p6 * s.db[379][51]);
        let eq86_e1289_d_b52: f64 = (p.p6 * s.db[379][52]);
        let eq86_e1289_d_b53: f64 = (p.p6 * s.db[379][53]);
        let eq86_e1289_d_b54: f64 = (p.p6 * s.db[379][54]);
        let eq86_e1291: f64 = (eq86_e1289 * (nv17 - nv16));
        let eq86_e1291_d_n0: f64 = (eq86_e1289_d_n0 * (nv17 - nv16));
        let eq86_e1291_d_n1: f64 = (eq86_e1289_d_n1 * (nv17 - nv16));
        let eq86_e1291_d_n2: f64 = (eq86_e1289_d_n2 * (nv17 - nv16));
        let eq86_e1291_d_n3: f64 = (eq86_e1289_d_n3 * (nv17 - nv16));
        let eq86_e1291_d_n4: f64 = (eq86_e1289_d_n4 * (nv17 - nv16));
        let eq86_e1291_d_n5: f64 = (eq86_e1289_d_n5 * (nv17 - nv16));
        let eq86_e1291_d_n6: f64 = (eq86_e1289_d_n6 * (nv17 - nv16));
        let eq86_e1291_d_n7: f64 = (eq86_e1289_d_n7 * (nv17 - nv16));
        let eq86_e1291_d_n8: f64 = (eq86_e1289_d_n8 * (nv17 - nv16));
        let eq86_e1291_d_n9: f64 = (eq86_e1289_d_n9 * (nv17 - nv16));
        let eq86_e1291_d_n10: f64 = (eq86_e1289_d_n10 * (nv17 - nv16));
        let eq86_e1291_d_n11: f64 = (eq86_e1289_d_n11 * (nv17 - nv16));
        let eq86_e1291_d_n12: f64 = (eq86_e1289_d_n12 * (nv17 - nv16));
        let eq86_e1291_d_n13: f64 = (eq86_e1289_d_n13 * (nv17 - nv16));
        let eq86_e1291_d_n14: f64 = (eq86_e1289_d_n14 * (nv17 - nv16));
        let eq86_e1291_d_n15: f64 = (eq86_e1289_d_n15 * (nv17 - nv16));
        let eq86_e1291_d_n16: f64 = ((eq86_e1289_d_n16 * (nv17 - nv16)) + (-eq86_e1289));
        let eq86_e1291_d_n17: f64 = ((eq86_e1289_d_n17 * (nv17 - nv16)) + eq86_e1289);
        let eq86_e1291_d_n18: f64 = (eq86_e1289_d_n18 * (nv17 - nv16));
        let eq86_e1291_d_n19: f64 = (eq86_e1289_d_n19 * (nv17 - nv16));
        let eq86_e1291_d_n20: f64 = (eq86_e1289_d_n20 * (nv17 - nv16));
        let eq86_e1291_d_n21: f64 = (eq86_e1289_d_n21 * (nv17 - nv16));
        let eq86_e1291_d_n22: f64 = (eq86_e1289_d_n22 * (nv17 - nv16));
        let eq86_e1291_d_b0: f64 = (eq86_e1289_d_b0 * (nv17 - nv16));
        let eq86_e1291_d_b1: f64 = (eq86_e1289_d_b1 * (nv17 - nv16));
        let eq86_e1291_d_b2: f64 = (eq86_e1289_d_b2 * (nv17 - nv16));
        let eq86_e1291_d_b3: f64 = (eq86_e1289_d_b3 * (nv17 - nv16));
        let eq86_e1291_d_b4: f64 = (eq86_e1289_d_b4 * (nv17 - nv16));
        let eq86_e1291_d_b5: f64 = (eq86_e1289_d_b5 * (nv17 - nv16));
        let eq86_e1291_d_b6: f64 = (eq86_e1289_d_b6 * (nv17 - nv16));
        let eq86_e1291_d_b7: f64 = (eq86_e1289_d_b7 * (nv17 - nv16));
        let eq86_e1291_d_b8: f64 = (eq86_e1289_d_b8 * (nv17 - nv16));
        let eq86_e1291_d_b9: f64 = (eq86_e1289_d_b9 * (nv17 - nv16));
        let eq86_e1291_d_b10: f64 = (eq86_e1289_d_b10 * (nv17 - nv16));
        let eq86_e1291_d_b11: f64 = (eq86_e1289_d_b11 * (nv17 - nv16));
        let eq86_e1291_d_b12: f64 = (eq86_e1289_d_b12 * (nv17 - nv16));
        let eq86_e1291_d_b13: f64 = (eq86_e1289_d_b13 * (nv17 - nv16));
        let eq86_e1291_d_b14: f64 = (eq86_e1289_d_b14 * (nv17 - nv16));
        let eq86_e1291_d_b15: f64 = (eq86_e1289_d_b15 * (nv17 - nv16));
        let eq86_e1291_d_b16: f64 = (eq86_e1289_d_b16 * (nv17 - nv16));
        let eq86_e1291_d_b17: f64 = (eq86_e1289_d_b17 * (nv17 - nv16));
        let eq86_e1291_d_b18: f64 = (eq86_e1289_d_b18 * (nv17 - nv16));
        let eq86_e1291_d_b19: f64 = (eq86_e1289_d_b19 * (nv17 - nv16));
        let eq86_e1291_d_b20: f64 = (eq86_e1289_d_b20 * (nv17 - nv16));
        let eq86_e1291_d_b21: f64 = (eq86_e1289_d_b21 * (nv17 - nv16));
        let eq86_e1291_d_b22: f64 = (eq86_e1289_d_b22 * (nv17 - nv16));
        let eq86_e1291_d_b23: f64 = (eq86_e1289_d_b23 * (nv17 - nv16));
        let eq86_e1291_d_b24: f64 = (eq86_e1289_d_b24 * (nv17 - nv16));
        let eq86_e1291_d_b25: f64 = (eq86_e1289_d_b25 * (nv17 - nv16));
        let eq86_e1291_d_b26: f64 = (eq86_e1289_d_b26 * (nv17 - nv16));
        let eq86_e1291_d_b27: f64 = (eq86_e1289_d_b27 * (nv17 - nv16));
        let eq86_e1291_d_b28: f64 = (eq86_e1289_d_b28 * (nv17 - nv16));
        let eq86_e1291_d_b29: f64 = (eq86_e1289_d_b29 * (nv17 - nv16));
        let eq86_e1291_d_b30: f64 = (eq86_e1289_d_b30 * (nv17 - nv16));
        let eq86_e1291_d_b31: f64 = (eq86_e1289_d_b31 * (nv17 - nv16));
        let eq86_e1291_d_b32: f64 = (eq86_e1289_d_b32 * (nv17 - nv16));
        let eq86_e1291_d_b33: f64 = (eq86_e1289_d_b33 * (nv17 - nv16));
        let eq86_e1291_d_b34: f64 = (eq86_e1289_d_b34 * (nv17 - nv16));
        let eq86_e1291_d_b35: f64 = (eq86_e1289_d_b35 * (nv17 - nv16));
        let eq86_e1291_d_b36: f64 = (eq86_e1289_d_b36 * (nv17 - nv16));
        let eq86_e1291_d_b37: f64 = (eq86_e1289_d_b37 * (nv17 - nv16));
        let eq86_e1291_d_b38: f64 = (eq86_e1289_d_b38 * (nv17 - nv16));
        let eq86_e1291_d_b39: f64 = (eq86_e1289_d_b39 * (nv17 - nv16));
        let eq86_e1291_d_b40: f64 = (eq86_e1289_d_b40 * (nv17 - nv16));
        let eq86_e1291_d_b41: f64 = (eq86_e1289_d_b41 * (nv17 - nv16));
        let eq86_e1291_d_b42: f64 = (eq86_e1289_d_b42 * (nv17 - nv16));
        let eq86_e1291_d_b43: f64 = (eq86_e1289_d_b43 * (nv17 - nv16));
        let eq86_e1291_d_b44: f64 = (eq86_e1289_d_b44 * (nv17 - nv16));
        let eq86_e1291_d_b45: f64 = (eq86_e1289_d_b45 * (nv17 - nv16));
        let eq86_e1291_d_b46: f64 = (eq86_e1289_d_b46 * (nv17 - nv16));
        let eq86_e1291_d_b47: f64 = (eq86_e1289_d_b47 * (nv17 - nv16));
        let eq86_e1291_d_b48: f64 = (eq86_e1289_d_b48 * (nv17 - nv16));
        let eq86_e1291_d_b49: f64 = (eq86_e1289_d_b49 * (nv17 - nv16));
        let eq86_e1291_d_b50: f64 = (eq86_e1289_d_b50 * (nv17 - nv16));
        let eq86_e1291_d_b51: f64 = (eq86_e1289_d_b51 * (nv17 - nv16));
        let eq86_e1291_d_b52: f64 = (eq86_e1289_d_b52 * (nv17 - nv16));
        let eq86_e1291_d_b53: f64 = (eq86_e1289_d_b53 * (nv17 - nv16));
        let eq86_e1291_d_b54: f64 = (eq86_e1289_d_b54 * (nv17 - nv16));
        let eq86_e1292: f64 = (eq86_e1286 + eq86_e1291);
        let eq86_e1292_d_n0: f64 = (eq86_e1286_d_n0 + eq86_e1291_d_n0);
        let eq86_e1292_d_n1: f64 = (eq86_e1286_d_n1 + eq86_e1291_d_n1);
        let eq86_e1292_d_n2: f64 = (eq86_e1286_d_n2 + eq86_e1291_d_n2);
        let eq86_e1292_d_n3: f64 = (eq86_e1286_d_n3 + eq86_e1291_d_n3);
        let eq86_e1292_d_n4: f64 = (eq86_e1286_d_n4 + eq86_e1291_d_n4);
        let eq86_e1292_d_n5: f64 = (eq86_e1286_d_n5 + eq86_e1291_d_n5);
        let eq86_e1292_d_n6: f64 = (eq86_e1286_d_n6 + eq86_e1291_d_n6);
        let eq86_e1292_d_n7: f64 = (eq86_e1286_d_n7 + eq86_e1291_d_n7);
        let eq86_e1292_d_n8: f64 = (eq86_e1286_d_n8 + eq86_e1291_d_n8);
        let eq86_e1292_d_n9: f64 = (eq86_e1286_d_n9 + eq86_e1291_d_n9);
        let eq86_e1292_d_n10: f64 = (eq86_e1286_d_n10 + eq86_e1291_d_n10);
        let eq86_e1292_d_n11: f64 = (eq86_e1286_d_n11 + eq86_e1291_d_n11);
        let eq86_e1292_d_n12: f64 = (eq86_e1286_d_n12 + eq86_e1291_d_n12);
        let eq86_e1292_d_n13: f64 = (eq86_e1286_d_n13 + eq86_e1291_d_n13);
        let eq86_e1292_d_n14: f64 = (eq86_e1286_d_n14 + eq86_e1291_d_n14);
        let eq86_e1292_d_n15: f64 = (eq86_e1286_d_n15 + eq86_e1291_d_n15);
        let eq86_e1292_d_n16: f64 = (eq86_e1286_d_n16 + eq86_e1291_d_n16);
        let eq86_e1292_d_n17: f64 = (eq86_e1286_d_n17 + eq86_e1291_d_n17);
        let eq86_e1292_d_n18: f64 = (eq86_e1286_d_n18 + eq86_e1291_d_n18);
        let eq86_e1292_d_n19: f64 = (eq86_e1286_d_n19 + eq86_e1291_d_n19);
        let eq86_e1292_d_n20: f64 = (eq86_e1286_d_n20 + eq86_e1291_d_n20);
        let eq86_e1292_d_n21: f64 = (eq86_e1286_d_n21 + eq86_e1291_d_n21);
        let eq86_e1292_d_n22: f64 = (eq86_e1286_d_n22 + eq86_e1291_d_n22);
        let eq86_e1292_d_b0: f64 = (eq86_e1286_d_b0 + eq86_e1291_d_b0);
        let eq86_e1292_d_b1: f64 = (eq86_e1286_d_b1 + eq86_e1291_d_b1);
        let eq86_e1292_d_b2: f64 = (eq86_e1286_d_b2 + eq86_e1291_d_b2);
        let eq86_e1292_d_b3: f64 = (eq86_e1286_d_b3 + eq86_e1291_d_b3);
        let eq86_e1292_d_b4: f64 = (eq86_e1286_d_b4 + eq86_e1291_d_b4);
        let eq86_e1292_d_b5: f64 = (eq86_e1286_d_b5 + eq86_e1291_d_b5);
        let eq86_e1292_d_b6: f64 = (eq86_e1286_d_b6 + eq86_e1291_d_b6);
        let eq86_e1292_d_b7: f64 = (eq86_e1286_d_b7 + eq86_e1291_d_b7);
        let eq86_e1292_d_b8: f64 = (eq86_e1286_d_b8 + eq86_e1291_d_b8);
        let eq86_e1292_d_b9: f64 = (eq86_e1286_d_b9 + eq86_e1291_d_b9);
        let eq86_e1292_d_b10: f64 = (eq86_e1286_d_b10 + eq86_e1291_d_b10);
        let eq86_e1292_d_b11: f64 = (eq86_e1286_d_b11 + eq86_e1291_d_b11);
        let eq86_e1292_d_b12: f64 = (eq86_e1286_d_b12 + eq86_e1291_d_b12);
        let eq86_e1292_d_b13: f64 = (eq86_e1286_d_b13 + eq86_e1291_d_b13);
        let eq86_e1292_d_b14: f64 = (eq86_e1286_d_b14 + eq86_e1291_d_b14);
        let eq86_e1292_d_b15: f64 = (eq86_e1286_d_b15 + eq86_e1291_d_b15);
        let eq86_e1292_d_b16: f64 = (eq86_e1286_d_b16 + eq86_e1291_d_b16);
        let eq86_e1292_d_b17: f64 = (eq86_e1286_d_b17 + eq86_e1291_d_b17);
        let eq86_e1292_d_b18: f64 = (eq86_e1286_d_b18 + eq86_e1291_d_b18);
        let eq86_e1292_d_b19: f64 = (eq86_e1286_d_b19 + eq86_e1291_d_b19);
        let eq86_e1292_d_b20: f64 = (eq86_e1286_d_b20 + eq86_e1291_d_b20);
        let eq86_e1292_d_b21: f64 = (eq86_e1286_d_b21 + eq86_e1291_d_b21);
        let eq86_e1292_d_b22: f64 = (eq86_e1286_d_b22 + eq86_e1291_d_b22);
        let eq86_e1292_d_b23: f64 = (eq86_e1286_d_b23 + eq86_e1291_d_b23);
        let eq86_e1292_d_b24: f64 = (eq86_e1286_d_b24 + eq86_e1291_d_b24);
        let eq86_e1292_d_b25: f64 = (eq86_e1286_d_b25 + eq86_e1291_d_b25);
        let eq86_e1292_d_b26: f64 = (eq86_e1286_d_b26 + eq86_e1291_d_b26);
        let eq86_e1292_d_b27: f64 = (eq86_e1286_d_b27 + eq86_e1291_d_b27);
        let eq86_e1292_d_b28: f64 = (eq86_e1286_d_b28 + eq86_e1291_d_b28);
        let eq86_e1292_d_b29: f64 = (eq86_e1286_d_b29 + eq86_e1291_d_b29);
        let eq86_e1292_d_b30: f64 = (eq86_e1286_d_b30 + eq86_e1291_d_b30);
        let eq86_e1292_d_b31: f64 = (eq86_e1286_d_b31 + eq86_e1291_d_b31);
        let eq86_e1292_d_b32: f64 = (eq86_e1286_d_b32 + eq86_e1291_d_b32);
        let eq86_e1292_d_b33: f64 = (eq86_e1286_d_b33 + eq86_e1291_d_b33);
        let eq86_e1292_d_b34: f64 = (eq86_e1286_d_b34 + eq86_e1291_d_b34);
        let eq86_e1292_d_b35: f64 = (eq86_e1286_d_b35 + eq86_e1291_d_b35);
        let eq86_e1292_d_b36: f64 = (eq86_e1286_d_b36 + eq86_e1291_d_b36);
        let eq86_e1292_d_b37: f64 = (eq86_e1286_d_b37 + eq86_e1291_d_b37);
        let eq86_e1292_d_b38: f64 = (eq86_e1286_d_b38 + eq86_e1291_d_b38);
        let eq86_e1292_d_b39: f64 = (eq86_e1286_d_b39 + eq86_e1291_d_b39);
        let eq86_e1292_d_b40: f64 = (eq86_e1286_d_b40 + eq86_e1291_d_b40);
        let eq86_e1292_d_b41: f64 = (eq86_e1286_d_b41 + eq86_e1291_d_b41);
        let eq86_e1292_d_b42: f64 = (eq86_e1286_d_b42 + eq86_e1291_d_b42);
        let eq86_e1292_d_b43: f64 = (eq86_e1286_d_b43 + eq86_e1291_d_b43);
        let eq86_e1292_d_b44: f64 = (eq86_e1286_d_b44 + eq86_e1291_d_b44);
        let eq86_e1292_d_b45: f64 = (eq86_e1286_d_b45 + eq86_e1291_d_b45);
        let eq86_e1292_d_b46: f64 = (eq86_e1286_d_b46 + eq86_e1291_d_b46);
        let eq86_e1292_d_b47: f64 = (eq86_e1286_d_b47 + eq86_e1291_d_b47);
        let eq86_e1292_d_b48: f64 = (eq86_e1286_d_b48 + eq86_e1291_d_b48);
        let eq86_e1292_d_b49: f64 = (eq86_e1286_d_b49 + eq86_e1291_d_b49);
        let eq86_e1292_d_b50: f64 = (eq86_e1286_d_b50 + eq86_e1291_d_b50);
        let eq86_e1292_d_b51: f64 = (eq86_e1286_d_b51 + eq86_e1291_d_b51);
        let eq86_e1292_d_b52: f64 = (eq86_e1286_d_b52 + eq86_e1291_d_b52);
        let eq86_e1292_d_b53: f64 = (eq86_e1286_d_b53 + eq86_e1291_d_b53);
        let eq86_e1292_d_b54: f64 = (eq86_e1286_d_b54 + eq86_e1291_d_b54);
        (eq86_e1292, eq86_e1292_d_n0, eq86_e1292_d_n1, eq86_e1292_d_n2, eq86_e1292_d_n3, eq86_e1292_d_n4, eq86_e1292_d_n5, eq86_e1292_d_n6, eq86_e1292_d_n7, eq86_e1292_d_n8, eq86_e1292_d_n9, eq86_e1292_d_n10, eq86_e1292_d_n11, eq86_e1292_d_n12, eq86_e1292_d_n13, eq86_e1292_d_n14, eq86_e1292_d_n15, eq86_e1292_d_n16, eq86_e1292_d_n17, eq86_e1292_d_n18, eq86_e1292_d_n19, eq86_e1292_d_n20, eq86_e1292_d_n21, eq86_e1292_d_n22, eq86_e1292_d_b0, eq86_e1292_d_b1, eq86_e1292_d_b2, eq86_e1292_d_b3, eq86_e1292_d_b4, eq86_e1292_d_b5, eq86_e1292_d_b6, eq86_e1292_d_b7, eq86_e1292_d_b8, eq86_e1292_d_b9, eq86_e1292_d_b10, eq86_e1292_d_b11, eq86_e1292_d_b12, eq86_e1292_d_b13, eq86_e1292_d_b14, eq86_e1292_d_b15, eq86_e1292_d_b16, eq86_e1292_d_b17, eq86_e1292_d_b18, eq86_e1292_d_b19, eq86_e1292_d_b20, eq86_e1292_d_b21, eq86_e1292_d_b22, eq86_e1292_d_b23, eq86_e1292_d_b24, eq86_e1292_d_b25, eq86_e1292_d_b26, eq86_e1292_d_b27, eq86_e1292_d_b28, eq86_e1292_d_b29, eq86_e1292_d_b30, eq86_e1292_d_b31, eq86_e1292_d_b32, eq86_e1292_d_b33, eq86_e1292_d_b34, eq86_e1292_d_b35, eq86_e1292_d_b36, eq86_e1292_d_b37, eq86_e1292_d_b38, eq86_e1292_d_b39, eq86_e1292_d_b40, eq86_e1292_d_b41, eq86_e1292_d_b42, eq86_e1292_d_b43, eq86_e1292_d_b44, eq86_e1292_d_b45, eq86_e1292_d_b46, eq86_e1292_d_b47, eq86_e1292_d_b48, eq86_e1292_d_b49, eq86_e1292_d_b50, eq86_e1292_d_b51, eq86_e1292_d_b52, eq86_e1292_d_b53, eq86_e1292_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_value: f64 = eq86_e1294;
        let eq86_node_derivatives: [f64; 23] = [eq86_e1294_d_n0, eq86_e1294_d_n1, eq86_e1294_d_n2, eq86_e1294_d_n3, eq86_e1294_d_n4, eq86_e1294_d_n5, eq86_e1294_d_n6, eq86_e1294_d_n7, eq86_e1294_d_n8, eq86_e1294_d_n9, eq86_e1294_d_n10, eq86_e1294_d_n11, eq86_e1294_d_n12, eq86_e1294_d_n13, eq86_e1294_d_n14, eq86_e1294_d_n15, eq86_e1294_d_n16, eq86_e1294_d_n17, eq86_e1294_d_n18, eq86_e1294_d_n19, eq86_e1294_d_n20, eq86_e1294_d_n21, eq86_e1294_d_n22];
        let eq86_branch_derivatives: [f64; 55] = [eq86_e1294_d_b0, eq86_e1294_d_b1, eq86_e1294_d_b2, eq86_e1294_d_b3, eq86_e1294_d_b4, eq86_e1294_d_b5, eq86_e1294_d_b6, eq86_e1294_d_b7, eq86_e1294_d_b8, eq86_e1294_d_b9, eq86_e1294_d_b10, eq86_e1294_d_b11, eq86_e1294_d_b12, eq86_e1294_d_b13, eq86_e1294_d_b14, eq86_e1294_d_b15, eq86_e1294_d_b16, eq86_e1294_d_b17, eq86_e1294_d_b18, eq86_e1294_d_b19, eq86_e1294_d_b20, eq86_e1294_d_b21, eq86_e1294_d_b22, eq86_e1294_d_b23, eq86_e1294_d_b24, eq86_e1294_d_b25, eq86_e1294_d_b26, eq86_e1294_d_b27, eq86_e1294_d_b28, eq86_e1294_d_b29, eq86_e1294_d_b30, eq86_e1294_d_b31, eq86_e1294_d_b32, eq86_e1294_d_b33, eq86_e1294_d_b34, eq86_e1294_d_b35, eq86_e1294_d_b36, eq86_e1294_d_b37, eq86_e1294_d_b38, eq86_e1294_d_b39, eq86_e1294_d_b40, eq86_e1294_d_b41, eq86_e1294_d_b42, eq86_e1294_d_b43, eq86_e1294_d_b44, eq86_e1294_d_b45, eq86_e1294_d_b46, eq86_e1294_d_b47, eq86_e1294_d_b48, eq86_e1294_d_b49, eq86_e1294_d_b50, eq86_e1294_d_b51, eq86_e1294_d_b52, eq86_e1294_d_b53, eq86_e1294_d_b54];
        stamper.stamp_current_dense_local(
            Some(17),
            Some(16),
            multiplicity * (eq86_value),
            &eq86_node_derivatives,
            &eq86_branch_derivatives,
            multiplicity,
        );
        let (eq87_e1301,) = {
    if (s.b[493] && (!s.b[494])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq87_value: f64 = eq87_e1301;
        stamper.stamp_potential_const_local(
            43,
            eq87_value,
        );
        let (eq88_e1306,) = {
    if (!s.b[493]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq88_value: f64 = eq88_e1306;
        stamper.stamp_potential_const_local(
            44,
            eq88_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_17(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let (eq89_e1322, eq89_e1322_d_n0, eq89_e1322_d_n1, eq89_e1322_d_n2, eq89_e1322_d_n3, eq89_e1322_d_n4, eq89_e1322_d_n5, eq89_e1322_d_n6, eq89_e1322_d_n7, eq89_e1322_d_n8, eq89_e1322_d_n9, eq89_e1322_d_n10, eq89_e1322_d_n11, eq89_e1322_d_n12, eq89_e1322_d_n13, eq89_e1322_d_n14, eq89_e1322_d_n15, eq89_e1322_d_n16, eq89_e1322_d_n17, eq89_e1322_d_n18, eq89_e1322_d_n19, eq89_e1322_d_n20, eq89_e1322_d_n21, eq89_e1322_d_n22, eq89_e1322_d_b0, eq89_e1322_d_b1, eq89_e1322_d_b2, eq89_e1322_d_b3, eq89_e1322_d_b4, eq89_e1322_d_b5, eq89_e1322_d_b6, eq89_e1322_d_b7, eq89_e1322_d_b8, eq89_e1322_d_b9, eq89_e1322_d_b10, eq89_e1322_d_b11, eq89_e1322_d_b12, eq89_e1322_d_b13, eq89_e1322_d_b14, eq89_e1322_d_b15, eq89_e1322_d_b16, eq89_e1322_d_b17, eq89_e1322_d_b18, eq89_e1322_d_b19, eq89_e1322_d_b20, eq89_e1322_d_b21, eq89_e1322_d_b22, eq89_e1322_d_b23, eq89_e1322_d_b24, eq89_e1322_d_b25, eq89_e1322_d_b26, eq89_e1322_d_b27, eq89_e1322_d_b28, eq89_e1322_d_b29, eq89_e1322_d_b30, eq89_e1322_d_b31, eq89_e1322_d_b32, eq89_e1322_d_b33, eq89_e1322_d_b34, eq89_e1322_d_b35, eq89_e1322_d_b36, eq89_e1322_d_b37, eq89_e1322_d_b38, eq89_e1322_d_b39, eq89_e1322_d_b40, eq89_e1322_d_b41, eq89_e1322_d_b42, eq89_e1322_d_b43, eq89_e1322_d_b44, eq89_e1322_d_b45, eq89_e1322_d_b46, eq89_e1322_d_b47, eq89_e1322_d_b48, eq89_e1322_d_b49, eq89_e1322_d_b50, eq89_e1322_d_b51, eq89_e1322_d_b52, eq89_e1322_d_b53, eq89_e1322_d_b54,) = {
    if (s.b[508] && s.b[509]) {
        let eq89_e1312: f64 = (p.p6 * s.v[68]);
        let eq89_e1312_d_n0: f64 = (p.p6 * s.dn[68][0]);
        let eq89_e1312_d_n1: f64 = (p.p6 * s.dn[68][1]);
        let eq89_e1312_d_n2: f64 = (p.p6 * s.dn[68][2]);
        let eq89_e1312_d_n3: f64 = (p.p6 * s.dn[68][3]);
        let eq89_e1312_d_n4: f64 = (p.p6 * s.dn[68][4]);
        let eq89_e1312_d_n5: f64 = (p.p6 * s.dn[68][5]);
        let eq89_e1312_d_n6: f64 = (p.p6 * s.dn[68][6]);
        let eq89_e1312_d_n7: f64 = (p.p6 * s.dn[68][7]);
        let eq89_e1312_d_n8: f64 = (p.p6 * s.dn[68][8]);
        let eq89_e1312_d_n9: f64 = (p.p6 * s.dn[68][9]);
        let eq89_e1312_d_n10: f64 = (p.p6 * s.dn[68][10]);
        let eq89_e1312_d_n11: f64 = (p.p6 * s.dn[68][11]);
        let eq89_e1312_d_n12: f64 = (p.p6 * s.dn[68][12]);
        let eq89_e1312_d_n13: f64 = (p.p6 * s.dn[68][13]);
        let eq89_e1312_d_n14: f64 = (p.p6 * s.dn[68][14]);
        let eq89_e1312_d_n15: f64 = (p.p6 * s.dn[68][15]);
        let eq89_e1312_d_n16: f64 = (p.p6 * s.dn[68][16]);
        let eq89_e1312_d_n17: f64 = (p.p6 * s.dn[68][17]);
        let eq89_e1312_d_n18: f64 = (p.p6 * s.dn[68][18]);
        let eq89_e1312_d_n19: f64 = (p.p6 * s.dn[68][19]);
        let eq89_e1312_d_n20: f64 = (p.p6 * s.dn[68][20]);
        let eq89_e1312_d_n21: f64 = (p.p6 * s.dn[68][21]);
        let eq89_e1312_d_n22: f64 = (p.p6 * s.dn[68][22]);
        let eq89_e1312_d_b0: f64 = (p.p6 * s.db[68][0]);
        let eq89_e1312_d_b1: f64 = (p.p6 * s.db[68][1]);
        let eq89_e1312_d_b2: f64 = (p.p6 * s.db[68][2]);
        let eq89_e1312_d_b3: f64 = (p.p6 * s.db[68][3]);
        let eq89_e1312_d_b4: f64 = (p.p6 * s.db[68][4]);
        let eq89_e1312_d_b5: f64 = (p.p6 * s.db[68][5]);
        let eq89_e1312_d_b6: f64 = (p.p6 * s.db[68][6]);
        let eq89_e1312_d_b7: f64 = (p.p6 * s.db[68][7]);
        let eq89_e1312_d_b8: f64 = (p.p6 * s.db[68][8]);
        let eq89_e1312_d_b9: f64 = (p.p6 * s.db[68][9]);
        let eq89_e1312_d_b10: f64 = (p.p6 * s.db[68][10]);
        let eq89_e1312_d_b11: f64 = (p.p6 * s.db[68][11]);
        let eq89_e1312_d_b12: f64 = (p.p6 * s.db[68][12]);
        let eq89_e1312_d_b13: f64 = (p.p6 * s.db[68][13]);
        let eq89_e1312_d_b14: f64 = (p.p6 * s.db[68][14]);
        let eq89_e1312_d_b15: f64 = (p.p6 * s.db[68][15]);
        let eq89_e1312_d_b16: f64 = (p.p6 * s.db[68][16]);
        let eq89_e1312_d_b17: f64 = (p.p6 * s.db[68][17]);
        let eq89_e1312_d_b18: f64 = (p.p6 * s.db[68][18]);
        let eq89_e1312_d_b19: f64 = (p.p6 * s.db[68][19]);
        let eq89_e1312_d_b20: f64 = (p.p6 * s.db[68][20]);
        let eq89_e1312_d_b21: f64 = (p.p6 * s.db[68][21]);
        let eq89_e1312_d_b22: f64 = (p.p6 * s.db[68][22]);
        let eq89_e1312_d_b23: f64 = (p.p6 * s.db[68][23]);
        let eq89_e1312_d_b24: f64 = (p.p6 * s.db[68][24]);
        let eq89_e1312_d_b25: f64 = (p.p6 * s.db[68][25]);
        let eq89_e1312_d_b26: f64 = (p.p6 * s.db[68][26]);
        let eq89_e1312_d_b27: f64 = (p.p6 * s.db[68][27]);
        let eq89_e1312_d_b28: f64 = (p.p6 * s.db[68][28]);
        let eq89_e1312_d_b29: f64 = (p.p6 * s.db[68][29]);
        let eq89_e1312_d_b30: f64 = (p.p6 * s.db[68][30]);
        let eq89_e1312_d_b31: f64 = (p.p6 * s.db[68][31]);
        let eq89_e1312_d_b32: f64 = (p.p6 * s.db[68][32]);
        let eq89_e1312_d_b33: f64 = (p.p6 * s.db[68][33]);
        let eq89_e1312_d_b34: f64 = (p.p6 * s.db[68][34]);
        let eq89_e1312_d_b35: f64 = (p.p6 * s.db[68][35]);
        let eq89_e1312_d_b36: f64 = (p.p6 * s.db[68][36]);
        let eq89_e1312_d_b37: f64 = (p.p6 * s.db[68][37]);
        let eq89_e1312_d_b38: f64 = (p.p6 * s.db[68][38]);
        let eq89_e1312_d_b39: f64 = (p.p6 * s.db[68][39]);
        let eq89_e1312_d_b40: f64 = (p.p6 * s.db[68][40]);
        let eq89_e1312_d_b41: f64 = (p.p6 * s.db[68][41]);
        let eq89_e1312_d_b42: f64 = (p.p6 * s.db[68][42]);
        let eq89_e1312_d_b43: f64 = (p.p6 * s.db[68][43]);
        let eq89_e1312_d_b44: f64 = (p.p6 * s.db[68][44]);
        let eq89_e1312_d_b45: f64 = (p.p6 * s.db[68][45]);
        let eq89_e1312_d_b46: f64 = (p.p6 * s.db[68][46]);
        let eq89_e1312_d_b47: f64 = (p.p6 * s.db[68][47]);
        let eq89_e1312_d_b48: f64 = (p.p6 * s.db[68][48]);
        let eq89_e1312_d_b49: f64 = (p.p6 * s.db[68][49]);
        let eq89_e1312_d_b50: f64 = (p.p6 * s.db[68][50]);
        let eq89_e1312_d_b51: f64 = (p.p6 * s.db[68][51]);
        let eq89_e1312_d_b52: f64 = (p.p6 * s.db[68][52]);
        let eq89_e1312_d_b53: f64 = (p.p6 * s.db[68][53]);
        let eq89_e1312_d_b54: f64 = (p.p6 * s.db[68][54]);
        let eq89_e1314: f64 = (eq89_e1312 * s.v[293]);
        let eq89_e1314_d_n0: f64 = ((eq89_e1312_d_n0 * s.v[293]) + (eq89_e1312 * s.dn[293][0]));
        let eq89_e1314_d_n1: f64 = ((eq89_e1312_d_n1 * s.v[293]) + (eq89_e1312 * s.dn[293][1]));
        let eq89_e1314_d_n2: f64 = ((eq89_e1312_d_n2 * s.v[293]) + (eq89_e1312 * s.dn[293][2]));
        let eq89_e1314_d_n3: f64 = ((eq89_e1312_d_n3 * s.v[293]) + (eq89_e1312 * s.dn[293][3]));
        let eq89_e1314_d_n4: f64 = ((eq89_e1312_d_n4 * s.v[293]) + (eq89_e1312 * s.dn[293][4]));
        let eq89_e1314_d_n5: f64 = ((eq89_e1312_d_n5 * s.v[293]) + (eq89_e1312 * s.dn[293][5]));
        let eq89_e1314_d_n6: f64 = ((eq89_e1312_d_n6 * s.v[293]) + (eq89_e1312 * s.dn[293][6]));
        let eq89_e1314_d_n7: f64 = ((eq89_e1312_d_n7 * s.v[293]) + (eq89_e1312 * s.dn[293][7]));
        let eq89_e1314_d_n8: f64 = ((eq89_e1312_d_n8 * s.v[293]) + (eq89_e1312 * s.dn[293][8]));
        let eq89_e1314_d_n9: f64 = ((eq89_e1312_d_n9 * s.v[293]) + (eq89_e1312 * s.dn[293][9]));
        let eq89_e1314_d_n10: f64 = ((eq89_e1312_d_n10 * s.v[293]) + (eq89_e1312 * s.dn[293][10]));
        let eq89_e1314_d_n11: f64 = ((eq89_e1312_d_n11 * s.v[293]) + (eq89_e1312 * s.dn[293][11]));
        let eq89_e1314_d_n12: f64 = ((eq89_e1312_d_n12 * s.v[293]) + (eq89_e1312 * s.dn[293][12]));
        let eq89_e1314_d_n13: f64 = ((eq89_e1312_d_n13 * s.v[293]) + (eq89_e1312 * s.dn[293][13]));
        let eq89_e1314_d_n14: f64 = ((eq89_e1312_d_n14 * s.v[293]) + (eq89_e1312 * s.dn[293][14]));
        let eq89_e1314_d_n15: f64 = ((eq89_e1312_d_n15 * s.v[293]) + (eq89_e1312 * s.dn[293][15]));
        let eq89_e1314_d_n16: f64 = ((eq89_e1312_d_n16 * s.v[293]) + (eq89_e1312 * s.dn[293][16]));
        let eq89_e1314_d_n17: f64 = ((eq89_e1312_d_n17 * s.v[293]) + (eq89_e1312 * s.dn[293][17]));
        let eq89_e1314_d_n18: f64 = ((eq89_e1312_d_n18 * s.v[293]) + (eq89_e1312 * s.dn[293][18]));
        let eq89_e1314_d_n19: f64 = ((eq89_e1312_d_n19 * s.v[293]) + (eq89_e1312 * s.dn[293][19]));
        let eq89_e1314_d_n20: f64 = ((eq89_e1312_d_n20 * s.v[293]) + (eq89_e1312 * s.dn[293][20]));
        let eq89_e1314_d_n21: f64 = ((eq89_e1312_d_n21 * s.v[293]) + (eq89_e1312 * s.dn[293][21]));
        let eq89_e1314_d_n22: f64 = ((eq89_e1312_d_n22 * s.v[293]) + (eq89_e1312 * s.dn[293][22]));
        let eq89_e1314_d_b0: f64 = ((eq89_e1312_d_b0 * s.v[293]) + (eq89_e1312 * s.db[293][0]));
        let eq89_e1314_d_b1: f64 = ((eq89_e1312_d_b1 * s.v[293]) + (eq89_e1312 * s.db[293][1]));
        let eq89_e1314_d_b2: f64 = ((eq89_e1312_d_b2 * s.v[293]) + (eq89_e1312 * s.db[293][2]));
        let eq89_e1314_d_b3: f64 = ((eq89_e1312_d_b3 * s.v[293]) + (eq89_e1312 * s.db[293][3]));
        let eq89_e1314_d_b4: f64 = ((eq89_e1312_d_b4 * s.v[293]) + (eq89_e1312 * s.db[293][4]));
        let eq89_e1314_d_b5: f64 = ((eq89_e1312_d_b5 * s.v[293]) + (eq89_e1312 * s.db[293][5]));
        let eq89_e1314_d_b6: f64 = ((eq89_e1312_d_b6 * s.v[293]) + (eq89_e1312 * s.db[293][6]));
        let eq89_e1314_d_b7: f64 = ((eq89_e1312_d_b7 * s.v[293]) + (eq89_e1312 * s.db[293][7]));
        let eq89_e1314_d_b8: f64 = ((eq89_e1312_d_b8 * s.v[293]) + (eq89_e1312 * s.db[293][8]));
        let eq89_e1314_d_b9: f64 = ((eq89_e1312_d_b9 * s.v[293]) + (eq89_e1312 * s.db[293][9]));
        let eq89_e1314_d_b10: f64 = ((eq89_e1312_d_b10 * s.v[293]) + (eq89_e1312 * s.db[293][10]));
        let eq89_e1314_d_b11: f64 = ((eq89_e1312_d_b11 * s.v[293]) + (eq89_e1312 * s.db[293][11]));
        let eq89_e1314_d_b12: f64 = ((eq89_e1312_d_b12 * s.v[293]) + (eq89_e1312 * s.db[293][12]));
        let eq89_e1314_d_b13: f64 = ((eq89_e1312_d_b13 * s.v[293]) + (eq89_e1312 * s.db[293][13]));
        let eq89_e1314_d_b14: f64 = ((eq89_e1312_d_b14 * s.v[293]) + (eq89_e1312 * s.db[293][14]));
        let eq89_e1314_d_b15: f64 = ((eq89_e1312_d_b15 * s.v[293]) + (eq89_e1312 * s.db[293][15]));
        let eq89_e1314_d_b16: f64 = ((eq89_e1312_d_b16 * s.v[293]) + (eq89_e1312 * s.db[293][16]));
        let eq89_e1314_d_b17: f64 = ((eq89_e1312_d_b17 * s.v[293]) + (eq89_e1312 * s.db[293][17]));
        let eq89_e1314_d_b18: f64 = ((eq89_e1312_d_b18 * s.v[293]) + (eq89_e1312 * s.db[293][18]));
        let eq89_e1314_d_b19: f64 = ((eq89_e1312_d_b19 * s.v[293]) + (eq89_e1312 * s.db[293][19]));
        let eq89_e1314_d_b20: f64 = ((eq89_e1312_d_b20 * s.v[293]) + (eq89_e1312 * s.db[293][20]));
        let eq89_e1314_d_b21: f64 = ((eq89_e1312_d_b21 * s.v[293]) + (eq89_e1312 * s.db[293][21]));
        let eq89_e1314_d_b22: f64 = ((eq89_e1312_d_b22 * s.v[293]) + (eq89_e1312 * s.db[293][22]));
        let eq89_e1314_d_b23: f64 = ((eq89_e1312_d_b23 * s.v[293]) + (eq89_e1312 * s.db[293][23]));
        let eq89_e1314_d_b24: f64 = ((eq89_e1312_d_b24 * s.v[293]) + (eq89_e1312 * s.db[293][24]));
        let eq89_e1314_d_b25: f64 = ((eq89_e1312_d_b25 * s.v[293]) + (eq89_e1312 * s.db[293][25]));
        let eq89_e1314_d_b26: f64 = ((eq89_e1312_d_b26 * s.v[293]) + (eq89_e1312 * s.db[293][26]));
        let eq89_e1314_d_b27: f64 = ((eq89_e1312_d_b27 * s.v[293]) + (eq89_e1312 * s.db[293][27]));
        let eq89_e1314_d_b28: f64 = ((eq89_e1312_d_b28 * s.v[293]) + (eq89_e1312 * s.db[293][28]));
        let eq89_e1314_d_b29: f64 = ((eq89_e1312_d_b29 * s.v[293]) + (eq89_e1312 * s.db[293][29]));
        let eq89_e1314_d_b30: f64 = ((eq89_e1312_d_b30 * s.v[293]) + (eq89_e1312 * s.db[293][30]));
        let eq89_e1314_d_b31: f64 = ((eq89_e1312_d_b31 * s.v[293]) + (eq89_e1312 * s.db[293][31]));
        let eq89_e1314_d_b32: f64 = ((eq89_e1312_d_b32 * s.v[293]) + (eq89_e1312 * s.db[293][32]));
        let eq89_e1314_d_b33: f64 = ((eq89_e1312_d_b33 * s.v[293]) + (eq89_e1312 * s.db[293][33]));
        let eq89_e1314_d_b34: f64 = ((eq89_e1312_d_b34 * s.v[293]) + (eq89_e1312 * s.db[293][34]));
        let eq89_e1314_d_b35: f64 = ((eq89_e1312_d_b35 * s.v[293]) + (eq89_e1312 * s.db[293][35]));
        let eq89_e1314_d_b36: f64 = ((eq89_e1312_d_b36 * s.v[293]) + (eq89_e1312 * s.db[293][36]));
        let eq89_e1314_d_b37: f64 = ((eq89_e1312_d_b37 * s.v[293]) + (eq89_e1312 * s.db[293][37]));
        let eq89_e1314_d_b38: f64 = ((eq89_e1312_d_b38 * s.v[293]) + (eq89_e1312 * s.db[293][38]));
        let eq89_e1314_d_b39: f64 = ((eq89_e1312_d_b39 * s.v[293]) + (eq89_e1312 * s.db[293][39]));
        let eq89_e1314_d_b40: f64 = ((eq89_e1312_d_b40 * s.v[293]) + (eq89_e1312 * s.db[293][40]));
        let eq89_e1314_d_b41: f64 = ((eq89_e1312_d_b41 * s.v[293]) + (eq89_e1312 * s.db[293][41]));
        let eq89_e1314_d_b42: f64 = ((eq89_e1312_d_b42 * s.v[293]) + (eq89_e1312 * s.db[293][42]));
        let eq89_e1314_d_b43: f64 = ((eq89_e1312_d_b43 * s.v[293]) + (eq89_e1312 * s.db[293][43]));
        let eq89_e1314_d_b44: f64 = ((eq89_e1312_d_b44 * s.v[293]) + (eq89_e1312 * s.db[293][44]));
        let eq89_e1314_d_b45: f64 = ((eq89_e1312_d_b45 * s.v[293]) + (eq89_e1312 * s.db[293][45]));
        let eq89_e1314_d_b46: f64 = ((eq89_e1312_d_b46 * s.v[293]) + (eq89_e1312 * s.db[293][46]));
        let eq89_e1314_d_b47: f64 = ((eq89_e1312_d_b47 * s.v[293]) + (eq89_e1312 * s.db[293][47]));
        let eq89_e1314_d_b48: f64 = ((eq89_e1312_d_b48 * s.v[293]) + (eq89_e1312 * s.db[293][48]));
        let eq89_e1314_d_b49: f64 = ((eq89_e1312_d_b49 * s.v[293]) + (eq89_e1312 * s.db[293][49]));
        let eq89_e1314_d_b50: f64 = ((eq89_e1312_d_b50 * s.v[293]) + (eq89_e1312 * s.db[293][50]));
        let eq89_e1314_d_b51: f64 = ((eq89_e1312_d_b51 * s.v[293]) + (eq89_e1312 * s.db[293][51]));
        let eq89_e1314_d_b52: f64 = ((eq89_e1312_d_b52 * s.v[293]) + (eq89_e1312 * s.db[293][52]));
        let eq89_e1314_d_b53: f64 = ((eq89_e1312_d_b53 * s.v[293]) + (eq89_e1312 * s.db[293][53]));
        let eq89_e1314_d_b54: f64 = ((eq89_e1312_d_b54 * s.v[293]) + (eq89_e1312 * s.db[293][54]));
        let eq89_e1317: f64 = (p.p6 * s.v[379]);
        let eq89_e1317_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq89_e1317_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq89_e1317_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq89_e1317_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq89_e1317_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq89_e1317_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq89_e1317_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq89_e1317_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq89_e1317_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq89_e1317_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq89_e1317_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq89_e1317_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq89_e1317_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq89_e1317_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq89_e1317_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq89_e1317_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq89_e1317_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq89_e1317_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq89_e1317_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq89_e1317_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq89_e1317_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq89_e1317_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq89_e1317_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq89_e1317_d_b0: f64 = (p.p6 * s.db[379][0]);
        let eq89_e1317_d_b1: f64 = (p.p6 * s.db[379][1]);
        let eq89_e1317_d_b2: f64 = (p.p6 * s.db[379][2]);
        let eq89_e1317_d_b3: f64 = (p.p6 * s.db[379][3]);
        let eq89_e1317_d_b4: f64 = (p.p6 * s.db[379][4]);
        let eq89_e1317_d_b5: f64 = (p.p6 * s.db[379][5]);
        let eq89_e1317_d_b6: f64 = (p.p6 * s.db[379][6]);
        let eq89_e1317_d_b7: f64 = (p.p6 * s.db[379][7]);
        let eq89_e1317_d_b8: f64 = (p.p6 * s.db[379][8]);
        let eq89_e1317_d_b9: f64 = (p.p6 * s.db[379][9]);
        let eq89_e1317_d_b10: f64 = (p.p6 * s.db[379][10]);
        let eq89_e1317_d_b11: f64 = (p.p6 * s.db[379][11]);
        let eq89_e1317_d_b12: f64 = (p.p6 * s.db[379][12]);
        let eq89_e1317_d_b13: f64 = (p.p6 * s.db[379][13]);
        let eq89_e1317_d_b14: f64 = (p.p6 * s.db[379][14]);
        let eq89_e1317_d_b15: f64 = (p.p6 * s.db[379][15]);
        let eq89_e1317_d_b16: f64 = (p.p6 * s.db[379][16]);
        let eq89_e1317_d_b17: f64 = (p.p6 * s.db[379][17]);
        let eq89_e1317_d_b18: f64 = (p.p6 * s.db[379][18]);
        let eq89_e1317_d_b19: f64 = (p.p6 * s.db[379][19]);
        let eq89_e1317_d_b20: f64 = (p.p6 * s.db[379][20]);
        let eq89_e1317_d_b21: f64 = (p.p6 * s.db[379][21]);
        let eq89_e1317_d_b22: f64 = (p.p6 * s.db[379][22]);
        let eq89_e1317_d_b23: f64 = (p.p6 * s.db[379][23]);
        let eq89_e1317_d_b24: f64 = (p.p6 * s.db[379][24]);
        let eq89_e1317_d_b25: f64 = (p.p6 * s.db[379][25]);
        let eq89_e1317_d_b26: f64 = (p.p6 * s.db[379][26]);
        let eq89_e1317_d_b27: f64 = (p.p6 * s.db[379][27]);
        let eq89_e1317_d_b28: f64 = (p.p6 * s.db[379][28]);
        let eq89_e1317_d_b29: f64 = (p.p6 * s.db[379][29]);
        let eq89_e1317_d_b30: f64 = (p.p6 * s.db[379][30]);
        let eq89_e1317_d_b31: f64 = (p.p6 * s.db[379][31]);
        let eq89_e1317_d_b32: f64 = (p.p6 * s.db[379][32]);
        let eq89_e1317_d_b33: f64 = (p.p6 * s.db[379][33]);
        let eq89_e1317_d_b34: f64 = (p.p6 * s.db[379][34]);
        let eq89_e1317_d_b35: f64 = (p.p6 * s.db[379][35]);
        let eq89_e1317_d_b36: f64 = (p.p6 * s.db[379][36]);
        let eq89_e1317_d_b37: f64 = (p.p6 * s.db[379][37]);
        let eq89_e1317_d_b38: f64 = (p.p6 * s.db[379][38]);
        let eq89_e1317_d_b39: f64 = (p.p6 * s.db[379][39]);
        let eq89_e1317_d_b40: f64 = (p.p6 * s.db[379][40]);
        let eq89_e1317_d_b41: f64 = (p.p6 * s.db[379][41]);
        let eq89_e1317_d_b42: f64 = (p.p6 * s.db[379][42]);
        let eq89_e1317_d_b43: f64 = (p.p6 * s.db[379][43]);
        let eq89_e1317_d_b44: f64 = (p.p6 * s.db[379][44]);
        let eq89_e1317_d_b45: f64 = (p.p6 * s.db[379][45]);
        let eq89_e1317_d_b46: f64 = (p.p6 * s.db[379][46]);
        let eq89_e1317_d_b47: f64 = (p.p6 * s.db[379][47]);
        let eq89_e1317_d_b48: f64 = (p.p6 * s.db[379][48]);
        let eq89_e1317_d_b49: f64 = (p.p6 * s.db[379][49]);
        let eq89_e1317_d_b50: f64 = (p.p6 * s.db[379][50]);
        let eq89_e1317_d_b51: f64 = (p.p6 * s.db[379][51]);
        let eq89_e1317_d_b52: f64 = (p.p6 * s.db[379][52]);
        let eq89_e1317_d_b53: f64 = (p.p6 * s.db[379][53]);
        let eq89_e1317_d_b54: f64 = (p.p6 * s.db[379][54]);
        let eq89_e1319: f64 = (eq89_e1317 * (nv20 - nv21));
        let eq89_e1319_d_n0: f64 = (eq89_e1317_d_n0 * (nv20 - nv21));
        let eq89_e1319_d_n1: f64 = (eq89_e1317_d_n1 * (nv20 - nv21));
        let eq89_e1319_d_n2: f64 = (eq89_e1317_d_n2 * (nv20 - nv21));
        let eq89_e1319_d_n3: f64 = (eq89_e1317_d_n3 * (nv20 - nv21));
        let eq89_e1319_d_n4: f64 = (eq89_e1317_d_n4 * (nv20 - nv21));
        let eq89_e1319_d_n5: f64 = (eq89_e1317_d_n5 * (nv20 - nv21));
        let eq89_e1319_d_n6: f64 = (eq89_e1317_d_n6 * (nv20 - nv21));
        let eq89_e1319_d_n7: f64 = (eq89_e1317_d_n7 * (nv20 - nv21));
        let eq89_e1319_d_n8: f64 = (eq89_e1317_d_n8 * (nv20 - nv21));
        let eq89_e1319_d_n9: f64 = (eq89_e1317_d_n9 * (nv20 - nv21));
        let eq89_e1319_d_n10: f64 = (eq89_e1317_d_n10 * (nv20 - nv21));
        let eq89_e1319_d_n11: f64 = (eq89_e1317_d_n11 * (nv20 - nv21));
        let eq89_e1319_d_n12: f64 = (eq89_e1317_d_n12 * (nv20 - nv21));
        let eq89_e1319_d_n13: f64 = (eq89_e1317_d_n13 * (nv20 - nv21));
        let eq89_e1319_d_n14: f64 = (eq89_e1317_d_n14 * (nv20 - nv21));
        let eq89_e1319_d_n15: f64 = (eq89_e1317_d_n15 * (nv20 - nv21));
        let eq89_e1319_d_n16: f64 = (eq89_e1317_d_n16 * (nv20 - nv21));
        let eq89_e1319_d_n17: f64 = (eq89_e1317_d_n17 * (nv20 - nv21));
        let eq89_e1319_d_n18: f64 = (eq89_e1317_d_n18 * (nv20 - nv21));
        let eq89_e1319_d_n19: f64 = (eq89_e1317_d_n19 * (nv20 - nv21));
        let eq89_e1319_d_n20: f64 = ((eq89_e1317_d_n20 * (nv20 - nv21)) + eq89_e1317);
        let eq89_e1319_d_n21: f64 = ((eq89_e1317_d_n21 * (nv20 - nv21)) + (-eq89_e1317));
        let eq89_e1319_d_n22: f64 = (eq89_e1317_d_n22 * (nv20 - nv21));
        let eq89_e1319_d_b0: f64 = (eq89_e1317_d_b0 * (nv20 - nv21));
        let eq89_e1319_d_b1: f64 = (eq89_e1317_d_b1 * (nv20 - nv21));
        let eq89_e1319_d_b2: f64 = (eq89_e1317_d_b2 * (nv20 - nv21));
        let eq89_e1319_d_b3: f64 = (eq89_e1317_d_b3 * (nv20 - nv21));
        let eq89_e1319_d_b4: f64 = (eq89_e1317_d_b4 * (nv20 - nv21));
        let eq89_e1319_d_b5: f64 = (eq89_e1317_d_b5 * (nv20 - nv21));
        let eq89_e1319_d_b6: f64 = (eq89_e1317_d_b6 * (nv20 - nv21));
        let eq89_e1319_d_b7: f64 = (eq89_e1317_d_b7 * (nv20 - nv21));
        let eq89_e1319_d_b8: f64 = (eq89_e1317_d_b8 * (nv20 - nv21));
        let eq89_e1319_d_b9: f64 = (eq89_e1317_d_b9 * (nv20 - nv21));
        let eq89_e1319_d_b10: f64 = (eq89_e1317_d_b10 * (nv20 - nv21));
        let eq89_e1319_d_b11: f64 = (eq89_e1317_d_b11 * (nv20 - nv21));
        let eq89_e1319_d_b12: f64 = (eq89_e1317_d_b12 * (nv20 - nv21));
        let eq89_e1319_d_b13: f64 = (eq89_e1317_d_b13 * (nv20 - nv21));
        let eq89_e1319_d_b14: f64 = (eq89_e1317_d_b14 * (nv20 - nv21));
        let eq89_e1319_d_b15: f64 = (eq89_e1317_d_b15 * (nv20 - nv21));
        let eq89_e1319_d_b16: f64 = (eq89_e1317_d_b16 * (nv20 - nv21));
        let eq89_e1319_d_b17: f64 = (eq89_e1317_d_b17 * (nv20 - nv21));
        let eq89_e1319_d_b18: f64 = (eq89_e1317_d_b18 * (nv20 - nv21));
        let eq89_e1319_d_b19: f64 = (eq89_e1317_d_b19 * (nv20 - nv21));
        let eq89_e1319_d_b20: f64 = (eq89_e1317_d_b20 * (nv20 - nv21));
        let eq89_e1319_d_b21: f64 = (eq89_e1317_d_b21 * (nv20 - nv21));
        let eq89_e1319_d_b22: f64 = (eq89_e1317_d_b22 * (nv20 - nv21));
        let eq89_e1319_d_b23: f64 = (eq89_e1317_d_b23 * (nv20 - nv21));
        let eq89_e1319_d_b24: f64 = (eq89_e1317_d_b24 * (nv20 - nv21));
        let eq89_e1319_d_b25: f64 = (eq89_e1317_d_b25 * (nv20 - nv21));
        let eq89_e1319_d_b26: f64 = (eq89_e1317_d_b26 * (nv20 - nv21));
        let eq89_e1319_d_b27: f64 = (eq89_e1317_d_b27 * (nv20 - nv21));
        let eq89_e1319_d_b28: f64 = (eq89_e1317_d_b28 * (nv20 - nv21));
        let eq89_e1319_d_b29: f64 = (eq89_e1317_d_b29 * (nv20 - nv21));
        let eq89_e1319_d_b30: f64 = (eq89_e1317_d_b30 * (nv20 - nv21));
        let eq89_e1319_d_b31: f64 = (eq89_e1317_d_b31 * (nv20 - nv21));
        let eq89_e1319_d_b32: f64 = (eq89_e1317_d_b32 * (nv20 - nv21));
        let eq89_e1319_d_b33: f64 = (eq89_e1317_d_b33 * (nv20 - nv21));
        let eq89_e1319_d_b34: f64 = (eq89_e1317_d_b34 * (nv20 - nv21));
        let eq89_e1319_d_b35: f64 = (eq89_e1317_d_b35 * (nv20 - nv21));
        let eq89_e1319_d_b36: f64 = (eq89_e1317_d_b36 * (nv20 - nv21));
        let eq89_e1319_d_b37: f64 = (eq89_e1317_d_b37 * (nv20 - nv21));
        let eq89_e1319_d_b38: f64 = (eq89_e1317_d_b38 * (nv20 - nv21));
        let eq89_e1319_d_b39: f64 = (eq89_e1317_d_b39 * (nv20 - nv21));
        let eq89_e1319_d_b40: f64 = (eq89_e1317_d_b40 * (nv20 - nv21));
        let eq89_e1319_d_b41: f64 = (eq89_e1317_d_b41 * (nv20 - nv21));
        let eq89_e1319_d_b42: f64 = (eq89_e1317_d_b42 * (nv20 - nv21));
        let eq89_e1319_d_b43: f64 = (eq89_e1317_d_b43 * (nv20 - nv21));
        let eq89_e1319_d_b44: f64 = (eq89_e1317_d_b44 * (nv20 - nv21));
        let eq89_e1319_d_b45: f64 = (eq89_e1317_d_b45 * (nv20 - nv21));
        let eq89_e1319_d_b46: f64 = (eq89_e1317_d_b46 * (nv20 - nv21));
        let eq89_e1319_d_b47: f64 = (eq89_e1317_d_b47 * (nv20 - nv21));
        let eq89_e1319_d_b48: f64 = (eq89_e1317_d_b48 * (nv20 - nv21));
        let eq89_e1319_d_b49: f64 = (eq89_e1317_d_b49 * (nv20 - nv21));
        let eq89_e1319_d_b50: f64 = (eq89_e1317_d_b50 * (nv20 - nv21));
        let eq89_e1319_d_b51: f64 = (eq89_e1317_d_b51 * (nv20 - nv21));
        let eq89_e1319_d_b52: f64 = (eq89_e1317_d_b52 * (nv20 - nv21));
        let eq89_e1319_d_b53: f64 = (eq89_e1317_d_b53 * (nv20 - nv21));
        let eq89_e1319_d_b54: f64 = (eq89_e1317_d_b54 * (nv20 - nv21));
        let eq89_e1320: f64 = (eq89_e1314 + eq89_e1319);
        let eq89_e1320_d_n0: f64 = (eq89_e1314_d_n0 + eq89_e1319_d_n0);
        let eq89_e1320_d_n1: f64 = (eq89_e1314_d_n1 + eq89_e1319_d_n1);
        let eq89_e1320_d_n2: f64 = (eq89_e1314_d_n2 + eq89_e1319_d_n2);
        let eq89_e1320_d_n3: f64 = (eq89_e1314_d_n3 + eq89_e1319_d_n3);
        let eq89_e1320_d_n4: f64 = (eq89_e1314_d_n4 + eq89_e1319_d_n4);
        let eq89_e1320_d_n5: f64 = (eq89_e1314_d_n5 + eq89_e1319_d_n5);
        let eq89_e1320_d_n6: f64 = (eq89_e1314_d_n6 + eq89_e1319_d_n6);
        let eq89_e1320_d_n7: f64 = (eq89_e1314_d_n7 + eq89_e1319_d_n7);
        let eq89_e1320_d_n8: f64 = (eq89_e1314_d_n8 + eq89_e1319_d_n8);
        let eq89_e1320_d_n9: f64 = (eq89_e1314_d_n9 + eq89_e1319_d_n9);
        let eq89_e1320_d_n10: f64 = (eq89_e1314_d_n10 + eq89_e1319_d_n10);
        let eq89_e1320_d_n11: f64 = (eq89_e1314_d_n11 + eq89_e1319_d_n11);
        let eq89_e1320_d_n12: f64 = (eq89_e1314_d_n12 + eq89_e1319_d_n12);
        let eq89_e1320_d_n13: f64 = (eq89_e1314_d_n13 + eq89_e1319_d_n13);
        let eq89_e1320_d_n14: f64 = (eq89_e1314_d_n14 + eq89_e1319_d_n14);
        let eq89_e1320_d_n15: f64 = (eq89_e1314_d_n15 + eq89_e1319_d_n15);
        let eq89_e1320_d_n16: f64 = (eq89_e1314_d_n16 + eq89_e1319_d_n16);
        let eq89_e1320_d_n17: f64 = (eq89_e1314_d_n17 + eq89_e1319_d_n17);
        let eq89_e1320_d_n18: f64 = (eq89_e1314_d_n18 + eq89_e1319_d_n18);
        let eq89_e1320_d_n19: f64 = (eq89_e1314_d_n19 + eq89_e1319_d_n19);
        let eq89_e1320_d_n20: f64 = (eq89_e1314_d_n20 + eq89_e1319_d_n20);
        let eq89_e1320_d_n21: f64 = (eq89_e1314_d_n21 + eq89_e1319_d_n21);
        let eq89_e1320_d_n22: f64 = (eq89_e1314_d_n22 + eq89_e1319_d_n22);
        let eq89_e1320_d_b0: f64 = (eq89_e1314_d_b0 + eq89_e1319_d_b0);
        let eq89_e1320_d_b1: f64 = (eq89_e1314_d_b1 + eq89_e1319_d_b1);
        let eq89_e1320_d_b2: f64 = (eq89_e1314_d_b2 + eq89_e1319_d_b2);
        let eq89_e1320_d_b3: f64 = (eq89_e1314_d_b3 + eq89_e1319_d_b3);
        let eq89_e1320_d_b4: f64 = (eq89_e1314_d_b4 + eq89_e1319_d_b4);
        let eq89_e1320_d_b5: f64 = (eq89_e1314_d_b5 + eq89_e1319_d_b5);
        let eq89_e1320_d_b6: f64 = (eq89_e1314_d_b6 + eq89_e1319_d_b6);
        let eq89_e1320_d_b7: f64 = (eq89_e1314_d_b7 + eq89_e1319_d_b7);
        let eq89_e1320_d_b8: f64 = (eq89_e1314_d_b8 + eq89_e1319_d_b8);
        let eq89_e1320_d_b9: f64 = (eq89_e1314_d_b9 + eq89_e1319_d_b9);
        let eq89_e1320_d_b10: f64 = (eq89_e1314_d_b10 + eq89_e1319_d_b10);
        let eq89_e1320_d_b11: f64 = (eq89_e1314_d_b11 + eq89_e1319_d_b11);
        let eq89_e1320_d_b12: f64 = (eq89_e1314_d_b12 + eq89_e1319_d_b12);
        let eq89_e1320_d_b13: f64 = (eq89_e1314_d_b13 + eq89_e1319_d_b13);
        let eq89_e1320_d_b14: f64 = (eq89_e1314_d_b14 + eq89_e1319_d_b14);
        let eq89_e1320_d_b15: f64 = (eq89_e1314_d_b15 + eq89_e1319_d_b15);
        let eq89_e1320_d_b16: f64 = (eq89_e1314_d_b16 + eq89_e1319_d_b16);
        let eq89_e1320_d_b17: f64 = (eq89_e1314_d_b17 + eq89_e1319_d_b17);
        let eq89_e1320_d_b18: f64 = (eq89_e1314_d_b18 + eq89_e1319_d_b18);
        let eq89_e1320_d_b19: f64 = (eq89_e1314_d_b19 + eq89_e1319_d_b19);
        let eq89_e1320_d_b20: f64 = (eq89_e1314_d_b20 + eq89_e1319_d_b20);
        let eq89_e1320_d_b21: f64 = (eq89_e1314_d_b21 + eq89_e1319_d_b21);
        let eq89_e1320_d_b22: f64 = (eq89_e1314_d_b22 + eq89_e1319_d_b22);
        let eq89_e1320_d_b23: f64 = (eq89_e1314_d_b23 + eq89_e1319_d_b23);
        let eq89_e1320_d_b24: f64 = (eq89_e1314_d_b24 + eq89_e1319_d_b24);
        let eq89_e1320_d_b25: f64 = (eq89_e1314_d_b25 + eq89_e1319_d_b25);
        let eq89_e1320_d_b26: f64 = (eq89_e1314_d_b26 + eq89_e1319_d_b26);
        let eq89_e1320_d_b27: f64 = (eq89_e1314_d_b27 + eq89_e1319_d_b27);
        let eq89_e1320_d_b28: f64 = (eq89_e1314_d_b28 + eq89_e1319_d_b28);
        let eq89_e1320_d_b29: f64 = (eq89_e1314_d_b29 + eq89_e1319_d_b29);
        let eq89_e1320_d_b30: f64 = (eq89_e1314_d_b30 + eq89_e1319_d_b30);
        let eq89_e1320_d_b31: f64 = (eq89_e1314_d_b31 + eq89_e1319_d_b31);
        let eq89_e1320_d_b32: f64 = (eq89_e1314_d_b32 + eq89_e1319_d_b32);
        let eq89_e1320_d_b33: f64 = (eq89_e1314_d_b33 + eq89_e1319_d_b33);
        let eq89_e1320_d_b34: f64 = (eq89_e1314_d_b34 + eq89_e1319_d_b34);
        let eq89_e1320_d_b35: f64 = (eq89_e1314_d_b35 + eq89_e1319_d_b35);
        let eq89_e1320_d_b36: f64 = (eq89_e1314_d_b36 + eq89_e1319_d_b36);
        let eq89_e1320_d_b37: f64 = (eq89_e1314_d_b37 + eq89_e1319_d_b37);
        let eq89_e1320_d_b38: f64 = (eq89_e1314_d_b38 + eq89_e1319_d_b38);
        let eq89_e1320_d_b39: f64 = (eq89_e1314_d_b39 + eq89_e1319_d_b39);
        let eq89_e1320_d_b40: f64 = (eq89_e1314_d_b40 + eq89_e1319_d_b40);
        let eq89_e1320_d_b41: f64 = (eq89_e1314_d_b41 + eq89_e1319_d_b41);
        let eq89_e1320_d_b42: f64 = (eq89_e1314_d_b42 + eq89_e1319_d_b42);
        let eq89_e1320_d_b43: f64 = (eq89_e1314_d_b43 + eq89_e1319_d_b43);
        let eq89_e1320_d_b44: f64 = (eq89_e1314_d_b44 + eq89_e1319_d_b44);
        let eq89_e1320_d_b45: f64 = (eq89_e1314_d_b45 + eq89_e1319_d_b45);
        let eq89_e1320_d_b46: f64 = (eq89_e1314_d_b46 + eq89_e1319_d_b46);
        let eq89_e1320_d_b47: f64 = (eq89_e1314_d_b47 + eq89_e1319_d_b47);
        let eq89_e1320_d_b48: f64 = (eq89_e1314_d_b48 + eq89_e1319_d_b48);
        let eq89_e1320_d_b49: f64 = (eq89_e1314_d_b49 + eq89_e1319_d_b49);
        let eq89_e1320_d_b50: f64 = (eq89_e1314_d_b50 + eq89_e1319_d_b50);
        let eq89_e1320_d_b51: f64 = (eq89_e1314_d_b51 + eq89_e1319_d_b51);
        let eq89_e1320_d_b52: f64 = (eq89_e1314_d_b52 + eq89_e1319_d_b52);
        let eq89_e1320_d_b53: f64 = (eq89_e1314_d_b53 + eq89_e1319_d_b53);
        let eq89_e1320_d_b54: f64 = (eq89_e1314_d_b54 + eq89_e1319_d_b54);
        (eq89_e1320, eq89_e1320_d_n0, eq89_e1320_d_n1, eq89_e1320_d_n2, eq89_e1320_d_n3, eq89_e1320_d_n4, eq89_e1320_d_n5, eq89_e1320_d_n6, eq89_e1320_d_n7, eq89_e1320_d_n8, eq89_e1320_d_n9, eq89_e1320_d_n10, eq89_e1320_d_n11, eq89_e1320_d_n12, eq89_e1320_d_n13, eq89_e1320_d_n14, eq89_e1320_d_n15, eq89_e1320_d_n16, eq89_e1320_d_n17, eq89_e1320_d_n18, eq89_e1320_d_n19, eq89_e1320_d_n20, eq89_e1320_d_n21, eq89_e1320_d_n22, eq89_e1320_d_b0, eq89_e1320_d_b1, eq89_e1320_d_b2, eq89_e1320_d_b3, eq89_e1320_d_b4, eq89_e1320_d_b5, eq89_e1320_d_b6, eq89_e1320_d_b7, eq89_e1320_d_b8, eq89_e1320_d_b9, eq89_e1320_d_b10, eq89_e1320_d_b11, eq89_e1320_d_b12, eq89_e1320_d_b13, eq89_e1320_d_b14, eq89_e1320_d_b15, eq89_e1320_d_b16, eq89_e1320_d_b17, eq89_e1320_d_b18, eq89_e1320_d_b19, eq89_e1320_d_b20, eq89_e1320_d_b21, eq89_e1320_d_b22, eq89_e1320_d_b23, eq89_e1320_d_b24, eq89_e1320_d_b25, eq89_e1320_d_b26, eq89_e1320_d_b27, eq89_e1320_d_b28, eq89_e1320_d_b29, eq89_e1320_d_b30, eq89_e1320_d_b31, eq89_e1320_d_b32, eq89_e1320_d_b33, eq89_e1320_d_b34, eq89_e1320_d_b35, eq89_e1320_d_b36, eq89_e1320_d_b37, eq89_e1320_d_b38, eq89_e1320_d_b39, eq89_e1320_d_b40, eq89_e1320_d_b41, eq89_e1320_d_b42, eq89_e1320_d_b43, eq89_e1320_d_b44, eq89_e1320_d_b45, eq89_e1320_d_b46, eq89_e1320_d_b47, eq89_e1320_d_b48, eq89_e1320_d_b49, eq89_e1320_d_b50, eq89_e1320_d_b51, eq89_e1320_d_b52, eq89_e1320_d_b53, eq89_e1320_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_value: f64 = eq89_e1322;
        let eq89_node_derivatives: [f64; 23] = [eq89_e1322_d_n0, eq89_e1322_d_n1, eq89_e1322_d_n2, eq89_e1322_d_n3, eq89_e1322_d_n4, eq89_e1322_d_n5, eq89_e1322_d_n6, eq89_e1322_d_n7, eq89_e1322_d_n8, eq89_e1322_d_n9, eq89_e1322_d_n10, eq89_e1322_d_n11, eq89_e1322_d_n12, eq89_e1322_d_n13, eq89_e1322_d_n14, eq89_e1322_d_n15, eq89_e1322_d_n16, eq89_e1322_d_n17, eq89_e1322_d_n18, eq89_e1322_d_n19, eq89_e1322_d_n20, eq89_e1322_d_n21, eq89_e1322_d_n22];
        let eq89_branch_derivatives: [f64; 55] = [eq89_e1322_d_b0, eq89_e1322_d_b1, eq89_e1322_d_b2, eq89_e1322_d_b3, eq89_e1322_d_b4, eq89_e1322_d_b5, eq89_e1322_d_b6, eq89_e1322_d_b7, eq89_e1322_d_b8, eq89_e1322_d_b9, eq89_e1322_d_b10, eq89_e1322_d_b11, eq89_e1322_d_b12, eq89_e1322_d_b13, eq89_e1322_d_b14, eq89_e1322_d_b15, eq89_e1322_d_b16, eq89_e1322_d_b17, eq89_e1322_d_b18, eq89_e1322_d_b19, eq89_e1322_d_b20, eq89_e1322_d_b21, eq89_e1322_d_b22, eq89_e1322_d_b23, eq89_e1322_d_b24, eq89_e1322_d_b25, eq89_e1322_d_b26, eq89_e1322_d_b27, eq89_e1322_d_b28, eq89_e1322_d_b29, eq89_e1322_d_b30, eq89_e1322_d_b31, eq89_e1322_d_b32, eq89_e1322_d_b33, eq89_e1322_d_b34, eq89_e1322_d_b35, eq89_e1322_d_b36, eq89_e1322_d_b37, eq89_e1322_d_b38, eq89_e1322_d_b39, eq89_e1322_d_b40, eq89_e1322_d_b41, eq89_e1322_d_b42, eq89_e1322_d_b43, eq89_e1322_d_b44, eq89_e1322_d_b45, eq89_e1322_d_b46, eq89_e1322_d_b47, eq89_e1322_d_b48, eq89_e1322_d_b49, eq89_e1322_d_b50, eq89_e1322_d_b51, eq89_e1322_d_b52, eq89_e1322_d_b53, eq89_e1322_d_b54];
        stamper.stamp_current_dense_local(
            Some(20),
            Some(21),
            multiplicity * (eq89_value),
            &eq89_node_derivatives,
            &eq89_branch_derivatives,
            multiplicity,
        );
        let (eq90_e1329,) = {
    if (s.b[508] && (!s.b[509])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq90_value: f64 = eq90_e1329;
        stamper.stamp_potential_const_local(
            45,
            eq90_value,
        );
        let (eq91_e1334,) = {
    if (!s.b[508]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq91_value: f64 = eq91_e1334;
        stamper.stamp_potential_const_local(
            46,
            eq91_value,
        );
        let (eq92_e1342,) = {
    if ((!s.b[508]) && (!s.b[517])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq92_value: f64 = eq92_e1342;
        stamper.stamp_potential_const_local(
            47,
            eq92_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_18(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq93_e1358, eq93_e1358_d_n0, eq93_e1358_d_n1, eq93_e1358_d_n2, eq93_e1358_d_n3, eq93_e1358_d_n4, eq93_e1358_d_n5, eq93_e1358_d_n6, eq93_e1358_d_n7, eq93_e1358_d_n8, eq93_e1358_d_n9, eq93_e1358_d_n10, eq93_e1358_d_n11, eq93_e1358_d_n12, eq93_e1358_d_n13, eq93_e1358_d_n14, eq93_e1358_d_n15, eq93_e1358_d_n16, eq93_e1358_d_n17, eq93_e1358_d_n18, eq93_e1358_d_n19, eq93_e1358_d_n20, eq93_e1358_d_n21, eq93_e1358_d_n22, eq93_e1358_d_b0, eq93_e1358_d_b1, eq93_e1358_d_b2, eq93_e1358_d_b3, eq93_e1358_d_b4, eq93_e1358_d_b5, eq93_e1358_d_b6, eq93_e1358_d_b7, eq93_e1358_d_b8, eq93_e1358_d_b9, eq93_e1358_d_b10, eq93_e1358_d_b11, eq93_e1358_d_b12, eq93_e1358_d_b13, eq93_e1358_d_b14, eq93_e1358_d_b15, eq93_e1358_d_b16, eq93_e1358_d_b17, eq93_e1358_d_b18, eq93_e1358_d_b19, eq93_e1358_d_b20, eq93_e1358_d_b21, eq93_e1358_d_b22, eq93_e1358_d_b23, eq93_e1358_d_b24, eq93_e1358_d_b25, eq93_e1358_d_b26, eq93_e1358_d_b27, eq93_e1358_d_b28, eq93_e1358_d_b29, eq93_e1358_d_b30, eq93_e1358_d_b31, eq93_e1358_d_b32, eq93_e1358_d_b33, eq93_e1358_d_b34, eq93_e1358_d_b35, eq93_e1358_d_b36, eq93_e1358_d_b37, eq93_e1358_d_b38, eq93_e1358_d_b39, eq93_e1358_d_b40, eq93_e1358_d_b41, eq93_e1358_d_b42, eq93_e1358_d_b43, eq93_e1358_d_b44, eq93_e1358_d_b45, eq93_e1358_d_b46, eq93_e1358_d_b47, eq93_e1358_d_b48, eq93_e1358_d_b49, eq93_e1358_d_b50, eq93_e1358_d_b51, eq93_e1358_d_b52, eq93_e1358_d_b53, eq93_e1358_d_b54,) = {
    if (s.b[523] && s.b[524]) {
        let eq93_e1348: f64 = (p.p6 * s.v[72]);
        let eq93_e1348_d_n0: f64 = (p.p6 * s.dn[72][0]);
        let eq93_e1348_d_n1: f64 = (p.p6 * s.dn[72][1]);
        let eq93_e1348_d_n2: f64 = (p.p6 * s.dn[72][2]);
        let eq93_e1348_d_n3: f64 = (p.p6 * s.dn[72][3]);
        let eq93_e1348_d_n4: f64 = (p.p6 * s.dn[72][4]);
        let eq93_e1348_d_n5: f64 = (p.p6 * s.dn[72][5]);
        let eq93_e1348_d_n6: f64 = (p.p6 * s.dn[72][6]);
        let eq93_e1348_d_n7: f64 = (p.p6 * s.dn[72][7]);
        let eq93_e1348_d_n8: f64 = (p.p6 * s.dn[72][8]);
        let eq93_e1348_d_n9: f64 = (p.p6 * s.dn[72][9]);
        let eq93_e1348_d_n10: f64 = (p.p6 * s.dn[72][10]);
        let eq93_e1348_d_n11: f64 = (p.p6 * s.dn[72][11]);
        let eq93_e1348_d_n12: f64 = (p.p6 * s.dn[72][12]);
        let eq93_e1348_d_n13: f64 = (p.p6 * s.dn[72][13]);
        let eq93_e1348_d_n14: f64 = (p.p6 * s.dn[72][14]);
        let eq93_e1348_d_n15: f64 = (p.p6 * s.dn[72][15]);
        let eq93_e1348_d_n16: f64 = (p.p6 * s.dn[72][16]);
        let eq93_e1348_d_n17: f64 = (p.p6 * s.dn[72][17]);
        let eq93_e1348_d_n18: f64 = (p.p6 * s.dn[72][18]);
        let eq93_e1348_d_n19: f64 = (p.p6 * s.dn[72][19]);
        let eq93_e1348_d_n20: f64 = (p.p6 * s.dn[72][20]);
        let eq93_e1348_d_n21: f64 = (p.p6 * s.dn[72][21]);
        let eq93_e1348_d_n22: f64 = (p.p6 * s.dn[72][22]);
        let eq93_e1348_d_b0: f64 = (p.p6 * s.db[72][0]);
        let eq93_e1348_d_b1: f64 = (p.p6 * s.db[72][1]);
        let eq93_e1348_d_b2: f64 = (p.p6 * s.db[72][2]);
        let eq93_e1348_d_b3: f64 = (p.p6 * s.db[72][3]);
        let eq93_e1348_d_b4: f64 = (p.p6 * s.db[72][4]);
        let eq93_e1348_d_b5: f64 = (p.p6 * s.db[72][5]);
        let eq93_e1348_d_b6: f64 = (p.p6 * s.db[72][6]);
        let eq93_e1348_d_b7: f64 = (p.p6 * s.db[72][7]);
        let eq93_e1348_d_b8: f64 = (p.p6 * s.db[72][8]);
        let eq93_e1348_d_b9: f64 = (p.p6 * s.db[72][9]);
        let eq93_e1348_d_b10: f64 = (p.p6 * s.db[72][10]);
        let eq93_e1348_d_b11: f64 = (p.p6 * s.db[72][11]);
        let eq93_e1348_d_b12: f64 = (p.p6 * s.db[72][12]);
        let eq93_e1348_d_b13: f64 = (p.p6 * s.db[72][13]);
        let eq93_e1348_d_b14: f64 = (p.p6 * s.db[72][14]);
        let eq93_e1348_d_b15: f64 = (p.p6 * s.db[72][15]);
        let eq93_e1348_d_b16: f64 = (p.p6 * s.db[72][16]);
        let eq93_e1348_d_b17: f64 = (p.p6 * s.db[72][17]);
        let eq93_e1348_d_b18: f64 = (p.p6 * s.db[72][18]);
        let eq93_e1348_d_b19: f64 = (p.p6 * s.db[72][19]);
        let eq93_e1348_d_b20: f64 = (p.p6 * s.db[72][20]);
        let eq93_e1348_d_b21: f64 = (p.p6 * s.db[72][21]);
        let eq93_e1348_d_b22: f64 = (p.p6 * s.db[72][22]);
        let eq93_e1348_d_b23: f64 = (p.p6 * s.db[72][23]);
        let eq93_e1348_d_b24: f64 = (p.p6 * s.db[72][24]);
        let eq93_e1348_d_b25: f64 = (p.p6 * s.db[72][25]);
        let eq93_e1348_d_b26: f64 = (p.p6 * s.db[72][26]);
        let eq93_e1348_d_b27: f64 = (p.p6 * s.db[72][27]);
        let eq93_e1348_d_b28: f64 = (p.p6 * s.db[72][28]);
        let eq93_e1348_d_b29: f64 = (p.p6 * s.db[72][29]);
        let eq93_e1348_d_b30: f64 = (p.p6 * s.db[72][30]);
        let eq93_e1348_d_b31: f64 = (p.p6 * s.db[72][31]);
        let eq93_e1348_d_b32: f64 = (p.p6 * s.db[72][32]);
        let eq93_e1348_d_b33: f64 = (p.p6 * s.db[72][33]);
        let eq93_e1348_d_b34: f64 = (p.p6 * s.db[72][34]);
        let eq93_e1348_d_b35: f64 = (p.p6 * s.db[72][35]);
        let eq93_e1348_d_b36: f64 = (p.p6 * s.db[72][36]);
        let eq93_e1348_d_b37: f64 = (p.p6 * s.db[72][37]);
        let eq93_e1348_d_b38: f64 = (p.p6 * s.db[72][38]);
        let eq93_e1348_d_b39: f64 = (p.p6 * s.db[72][39]);
        let eq93_e1348_d_b40: f64 = (p.p6 * s.db[72][40]);
        let eq93_e1348_d_b41: f64 = (p.p6 * s.db[72][41]);
        let eq93_e1348_d_b42: f64 = (p.p6 * s.db[72][42]);
        let eq93_e1348_d_b43: f64 = (p.p6 * s.db[72][43]);
        let eq93_e1348_d_b44: f64 = (p.p6 * s.db[72][44]);
        let eq93_e1348_d_b45: f64 = (p.p6 * s.db[72][45]);
        let eq93_e1348_d_b46: f64 = (p.p6 * s.db[72][46]);
        let eq93_e1348_d_b47: f64 = (p.p6 * s.db[72][47]);
        let eq93_e1348_d_b48: f64 = (p.p6 * s.db[72][48]);
        let eq93_e1348_d_b49: f64 = (p.p6 * s.db[72][49]);
        let eq93_e1348_d_b50: f64 = (p.p6 * s.db[72][50]);
        let eq93_e1348_d_b51: f64 = (p.p6 * s.db[72][51]);
        let eq93_e1348_d_b52: f64 = (p.p6 * s.db[72][52]);
        let eq93_e1348_d_b53: f64 = (p.p6 * s.db[72][53]);
        let eq93_e1348_d_b54: f64 = (p.p6 * s.db[72][54]);
        let eq93_e1350: f64 = (eq93_e1348 * s.v[305]);
        let eq93_e1350_d_n0: f64 = ((eq93_e1348_d_n0 * s.v[305]) + (eq93_e1348 * s.dn[305][0]));
        let eq93_e1350_d_n1: f64 = ((eq93_e1348_d_n1 * s.v[305]) + (eq93_e1348 * s.dn[305][1]));
        let eq93_e1350_d_n2: f64 = ((eq93_e1348_d_n2 * s.v[305]) + (eq93_e1348 * s.dn[305][2]));
        let eq93_e1350_d_n3: f64 = ((eq93_e1348_d_n3 * s.v[305]) + (eq93_e1348 * s.dn[305][3]));
        let eq93_e1350_d_n4: f64 = ((eq93_e1348_d_n4 * s.v[305]) + (eq93_e1348 * s.dn[305][4]));
        let eq93_e1350_d_n5: f64 = ((eq93_e1348_d_n5 * s.v[305]) + (eq93_e1348 * s.dn[305][5]));
        let eq93_e1350_d_n6: f64 = ((eq93_e1348_d_n6 * s.v[305]) + (eq93_e1348 * s.dn[305][6]));
        let eq93_e1350_d_n7: f64 = ((eq93_e1348_d_n7 * s.v[305]) + (eq93_e1348 * s.dn[305][7]));
        let eq93_e1350_d_n8: f64 = ((eq93_e1348_d_n8 * s.v[305]) + (eq93_e1348 * s.dn[305][8]));
        let eq93_e1350_d_n9: f64 = ((eq93_e1348_d_n9 * s.v[305]) + (eq93_e1348 * s.dn[305][9]));
        let eq93_e1350_d_n10: f64 = ((eq93_e1348_d_n10 * s.v[305]) + (eq93_e1348 * s.dn[305][10]));
        let eq93_e1350_d_n11: f64 = ((eq93_e1348_d_n11 * s.v[305]) + (eq93_e1348 * s.dn[305][11]));
        let eq93_e1350_d_n12: f64 = ((eq93_e1348_d_n12 * s.v[305]) + (eq93_e1348 * s.dn[305][12]));
        let eq93_e1350_d_n13: f64 = ((eq93_e1348_d_n13 * s.v[305]) + (eq93_e1348 * s.dn[305][13]));
        let eq93_e1350_d_n14: f64 = ((eq93_e1348_d_n14 * s.v[305]) + (eq93_e1348 * s.dn[305][14]));
        let eq93_e1350_d_n15: f64 = ((eq93_e1348_d_n15 * s.v[305]) + (eq93_e1348 * s.dn[305][15]));
        let eq93_e1350_d_n16: f64 = ((eq93_e1348_d_n16 * s.v[305]) + (eq93_e1348 * s.dn[305][16]));
        let eq93_e1350_d_n17: f64 = ((eq93_e1348_d_n17 * s.v[305]) + (eq93_e1348 * s.dn[305][17]));
        let eq93_e1350_d_n18: f64 = ((eq93_e1348_d_n18 * s.v[305]) + (eq93_e1348 * s.dn[305][18]));
        let eq93_e1350_d_n19: f64 = ((eq93_e1348_d_n19 * s.v[305]) + (eq93_e1348 * s.dn[305][19]));
        let eq93_e1350_d_n20: f64 = ((eq93_e1348_d_n20 * s.v[305]) + (eq93_e1348 * s.dn[305][20]));
        let eq93_e1350_d_n21: f64 = ((eq93_e1348_d_n21 * s.v[305]) + (eq93_e1348 * s.dn[305][21]));
        let eq93_e1350_d_n22: f64 = ((eq93_e1348_d_n22 * s.v[305]) + (eq93_e1348 * s.dn[305][22]));
        let eq93_e1350_d_b0: f64 = ((eq93_e1348_d_b0 * s.v[305]) + (eq93_e1348 * s.db[305][0]));
        let eq93_e1350_d_b1: f64 = ((eq93_e1348_d_b1 * s.v[305]) + (eq93_e1348 * s.db[305][1]));
        let eq93_e1350_d_b2: f64 = ((eq93_e1348_d_b2 * s.v[305]) + (eq93_e1348 * s.db[305][2]));
        let eq93_e1350_d_b3: f64 = ((eq93_e1348_d_b3 * s.v[305]) + (eq93_e1348 * s.db[305][3]));
        let eq93_e1350_d_b4: f64 = ((eq93_e1348_d_b4 * s.v[305]) + (eq93_e1348 * s.db[305][4]));
        let eq93_e1350_d_b5: f64 = ((eq93_e1348_d_b5 * s.v[305]) + (eq93_e1348 * s.db[305][5]));
        let eq93_e1350_d_b6: f64 = ((eq93_e1348_d_b6 * s.v[305]) + (eq93_e1348 * s.db[305][6]));
        let eq93_e1350_d_b7: f64 = ((eq93_e1348_d_b7 * s.v[305]) + (eq93_e1348 * s.db[305][7]));
        let eq93_e1350_d_b8: f64 = ((eq93_e1348_d_b8 * s.v[305]) + (eq93_e1348 * s.db[305][8]));
        let eq93_e1350_d_b9: f64 = ((eq93_e1348_d_b9 * s.v[305]) + (eq93_e1348 * s.db[305][9]));
        let eq93_e1350_d_b10: f64 = ((eq93_e1348_d_b10 * s.v[305]) + (eq93_e1348 * s.db[305][10]));
        let eq93_e1350_d_b11: f64 = ((eq93_e1348_d_b11 * s.v[305]) + (eq93_e1348 * s.db[305][11]));
        let eq93_e1350_d_b12: f64 = ((eq93_e1348_d_b12 * s.v[305]) + (eq93_e1348 * s.db[305][12]));
        let eq93_e1350_d_b13: f64 = ((eq93_e1348_d_b13 * s.v[305]) + (eq93_e1348 * s.db[305][13]));
        let eq93_e1350_d_b14: f64 = ((eq93_e1348_d_b14 * s.v[305]) + (eq93_e1348 * s.db[305][14]));
        let eq93_e1350_d_b15: f64 = ((eq93_e1348_d_b15 * s.v[305]) + (eq93_e1348 * s.db[305][15]));
        let eq93_e1350_d_b16: f64 = ((eq93_e1348_d_b16 * s.v[305]) + (eq93_e1348 * s.db[305][16]));
        let eq93_e1350_d_b17: f64 = ((eq93_e1348_d_b17 * s.v[305]) + (eq93_e1348 * s.db[305][17]));
        let eq93_e1350_d_b18: f64 = ((eq93_e1348_d_b18 * s.v[305]) + (eq93_e1348 * s.db[305][18]));
        let eq93_e1350_d_b19: f64 = ((eq93_e1348_d_b19 * s.v[305]) + (eq93_e1348 * s.db[305][19]));
        let eq93_e1350_d_b20: f64 = ((eq93_e1348_d_b20 * s.v[305]) + (eq93_e1348 * s.db[305][20]));
        let eq93_e1350_d_b21: f64 = ((eq93_e1348_d_b21 * s.v[305]) + (eq93_e1348 * s.db[305][21]));
        let eq93_e1350_d_b22: f64 = ((eq93_e1348_d_b22 * s.v[305]) + (eq93_e1348 * s.db[305][22]));
        let eq93_e1350_d_b23: f64 = ((eq93_e1348_d_b23 * s.v[305]) + (eq93_e1348 * s.db[305][23]));
        let eq93_e1350_d_b24: f64 = ((eq93_e1348_d_b24 * s.v[305]) + (eq93_e1348 * s.db[305][24]));
        let eq93_e1350_d_b25: f64 = ((eq93_e1348_d_b25 * s.v[305]) + (eq93_e1348 * s.db[305][25]));
        let eq93_e1350_d_b26: f64 = ((eq93_e1348_d_b26 * s.v[305]) + (eq93_e1348 * s.db[305][26]));
        let eq93_e1350_d_b27: f64 = ((eq93_e1348_d_b27 * s.v[305]) + (eq93_e1348 * s.db[305][27]));
        let eq93_e1350_d_b28: f64 = ((eq93_e1348_d_b28 * s.v[305]) + (eq93_e1348 * s.db[305][28]));
        let eq93_e1350_d_b29: f64 = ((eq93_e1348_d_b29 * s.v[305]) + (eq93_e1348 * s.db[305][29]));
        let eq93_e1350_d_b30: f64 = ((eq93_e1348_d_b30 * s.v[305]) + (eq93_e1348 * s.db[305][30]));
        let eq93_e1350_d_b31: f64 = ((eq93_e1348_d_b31 * s.v[305]) + (eq93_e1348 * s.db[305][31]));
        let eq93_e1350_d_b32: f64 = ((eq93_e1348_d_b32 * s.v[305]) + (eq93_e1348 * s.db[305][32]));
        let eq93_e1350_d_b33: f64 = ((eq93_e1348_d_b33 * s.v[305]) + (eq93_e1348 * s.db[305][33]));
        let eq93_e1350_d_b34: f64 = ((eq93_e1348_d_b34 * s.v[305]) + (eq93_e1348 * s.db[305][34]));
        let eq93_e1350_d_b35: f64 = ((eq93_e1348_d_b35 * s.v[305]) + (eq93_e1348 * s.db[305][35]));
        let eq93_e1350_d_b36: f64 = ((eq93_e1348_d_b36 * s.v[305]) + (eq93_e1348 * s.db[305][36]));
        let eq93_e1350_d_b37: f64 = ((eq93_e1348_d_b37 * s.v[305]) + (eq93_e1348 * s.db[305][37]));
        let eq93_e1350_d_b38: f64 = ((eq93_e1348_d_b38 * s.v[305]) + (eq93_e1348 * s.db[305][38]));
        let eq93_e1350_d_b39: f64 = ((eq93_e1348_d_b39 * s.v[305]) + (eq93_e1348 * s.db[305][39]));
        let eq93_e1350_d_b40: f64 = ((eq93_e1348_d_b40 * s.v[305]) + (eq93_e1348 * s.db[305][40]));
        let eq93_e1350_d_b41: f64 = ((eq93_e1348_d_b41 * s.v[305]) + (eq93_e1348 * s.db[305][41]));
        let eq93_e1350_d_b42: f64 = ((eq93_e1348_d_b42 * s.v[305]) + (eq93_e1348 * s.db[305][42]));
        let eq93_e1350_d_b43: f64 = ((eq93_e1348_d_b43 * s.v[305]) + (eq93_e1348 * s.db[305][43]));
        let eq93_e1350_d_b44: f64 = ((eq93_e1348_d_b44 * s.v[305]) + (eq93_e1348 * s.db[305][44]));
        let eq93_e1350_d_b45: f64 = ((eq93_e1348_d_b45 * s.v[305]) + (eq93_e1348 * s.db[305][45]));
        let eq93_e1350_d_b46: f64 = ((eq93_e1348_d_b46 * s.v[305]) + (eq93_e1348 * s.db[305][46]));
        let eq93_e1350_d_b47: f64 = ((eq93_e1348_d_b47 * s.v[305]) + (eq93_e1348 * s.db[305][47]));
        let eq93_e1350_d_b48: f64 = ((eq93_e1348_d_b48 * s.v[305]) + (eq93_e1348 * s.db[305][48]));
        let eq93_e1350_d_b49: f64 = ((eq93_e1348_d_b49 * s.v[305]) + (eq93_e1348 * s.db[305][49]));
        let eq93_e1350_d_b50: f64 = ((eq93_e1348_d_b50 * s.v[305]) + (eq93_e1348 * s.db[305][50]));
        let eq93_e1350_d_b51: f64 = ((eq93_e1348_d_b51 * s.v[305]) + (eq93_e1348 * s.db[305][51]));
        let eq93_e1350_d_b52: f64 = ((eq93_e1348_d_b52 * s.v[305]) + (eq93_e1348 * s.db[305][52]));
        let eq93_e1350_d_b53: f64 = ((eq93_e1348_d_b53 * s.v[305]) + (eq93_e1348 * s.db[305][53]));
        let eq93_e1350_d_b54: f64 = ((eq93_e1348_d_b54 * s.v[305]) + (eq93_e1348 * s.db[305][54]));
        let eq93_e1353: f64 = (p.p6 * s.v[379]);
        let eq93_e1353_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq93_e1353_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq93_e1353_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq93_e1353_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq93_e1353_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq93_e1353_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq93_e1353_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq93_e1353_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq93_e1353_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq93_e1353_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq93_e1353_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq93_e1353_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq93_e1353_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq93_e1353_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq93_e1353_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq93_e1353_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq93_e1353_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq93_e1353_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq93_e1353_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq93_e1353_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq93_e1353_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq93_e1353_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq93_e1353_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq93_e1353_d_b0: f64 = (p.p6 * s.db[379][0]);
        let eq93_e1353_d_b1: f64 = (p.p6 * s.db[379][1]);
        let eq93_e1353_d_b2: f64 = (p.p6 * s.db[379][2]);
        let eq93_e1353_d_b3: f64 = (p.p6 * s.db[379][3]);
        let eq93_e1353_d_b4: f64 = (p.p6 * s.db[379][4]);
        let eq93_e1353_d_b5: f64 = (p.p6 * s.db[379][5]);
        let eq93_e1353_d_b6: f64 = (p.p6 * s.db[379][6]);
        let eq93_e1353_d_b7: f64 = (p.p6 * s.db[379][7]);
        let eq93_e1353_d_b8: f64 = (p.p6 * s.db[379][8]);
        let eq93_e1353_d_b9: f64 = (p.p6 * s.db[379][9]);
        let eq93_e1353_d_b10: f64 = (p.p6 * s.db[379][10]);
        let eq93_e1353_d_b11: f64 = (p.p6 * s.db[379][11]);
        let eq93_e1353_d_b12: f64 = (p.p6 * s.db[379][12]);
        let eq93_e1353_d_b13: f64 = (p.p6 * s.db[379][13]);
        let eq93_e1353_d_b14: f64 = (p.p6 * s.db[379][14]);
        let eq93_e1353_d_b15: f64 = (p.p6 * s.db[379][15]);
        let eq93_e1353_d_b16: f64 = (p.p6 * s.db[379][16]);
        let eq93_e1353_d_b17: f64 = (p.p6 * s.db[379][17]);
        let eq93_e1353_d_b18: f64 = (p.p6 * s.db[379][18]);
        let eq93_e1353_d_b19: f64 = (p.p6 * s.db[379][19]);
        let eq93_e1353_d_b20: f64 = (p.p6 * s.db[379][20]);
        let eq93_e1353_d_b21: f64 = (p.p6 * s.db[379][21]);
        let eq93_e1353_d_b22: f64 = (p.p6 * s.db[379][22]);
        let eq93_e1353_d_b23: f64 = (p.p6 * s.db[379][23]);
        let eq93_e1353_d_b24: f64 = (p.p6 * s.db[379][24]);
        let eq93_e1353_d_b25: f64 = (p.p6 * s.db[379][25]);
        let eq93_e1353_d_b26: f64 = (p.p6 * s.db[379][26]);
        let eq93_e1353_d_b27: f64 = (p.p6 * s.db[379][27]);
        let eq93_e1353_d_b28: f64 = (p.p6 * s.db[379][28]);
        let eq93_e1353_d_b29: f64 = (p.p6 * s.db[379][29]);
        let eq93_e1353_d_b30: f64 = (p.p6 * s.db[379][30]);
        let eq93_e1353_d_b31: f64 = (p.p6 * s.db[379][31]);
        let eq93_e1353_d_b32: f64 = (p.p6 * s.db[379][32]);
        let eq93_e1353_d_b33: f64 = (p.p6 * s.db[379][33]);
        let eq93_e1353_d_b34: f64 = (p.p6 * s.db[379][34]);
        let eq93_e1353_d_b35: f64 = (p.p6 * s.db[379][35]);
        let eq93_e1353_d_b36: f64 = (p.p6 * s.db[379][36]);
        let eq93_e1353_d_b37: f64 = (p.p6 * s.db[379][37]);
        let eq93_e1353_d_b38: f64 = (p.p6 * s.db[379][38]);
        let eq93_e1353_d_b39: f64 = (p.p6 * s.db[379][39]);
        let eq93_e1353_d_b40: f64 = (p.p6 * s.db[379][40]);
        let eq93_e1353_d_b41: f64 = (p.p6 * s.db[379][41]);
        let eq93_e1353_d_b42: f64 = (p.p6 * s.db[379][42]);
        let eq93_e1353_d_b43: f64 = (p.p6 * s.db[379][43]);
        let eq93_e1353_d_b44: f64 = (p.p6 * s.db[379][44]);
        let eq93_e1353_d_b45: f64 = (p.p6 * s.db[379][45]);
        let eq93_e1353_d_b46: f64 = (p.p6 * s.db[379][46]);
        let eq93_e1353_d_b47: f64 = (p.p6 * s.db[379][47]);
        let eq93_e1353_d_b48: f64 = (p.p6 * s.db[379][48]);
        let eq93_e1353_d_b49: f64 = (p.p6 * s.db[379][49]);
        let eq93_e1353_d_b50: f64 = (p.p6 * s.db[379][50]);
        let eq93_e1353_d_b51: f64 = (p.p6 * s.db[379][51]);
        let eq93_e1353_d_b52: f64 = (p.p6 * s.db[379][52]);
        let eq93_e1353_d_b53: f64 = (p.p6 * s.db[379][53]);
        let eq93_e1353_d_b54: f64 = (p.p6 * s.db[379][54]);
        let eq93_e1355: f64 = (eq93_e1353 * (nv18 - nv17));
        let eq93_e1355_d_n0: f64 = (eq93_e1353_d_n0 * (nv18 - nv17));
        let eq93_e1355_d_n1: f64 = (eq93_e1353_d_n1 * (nv18 - nv17));
        let eq93_e1355_d_n2: f64 = (eq93_e1353_d_n2 * (nv18 - nv17));
        let eq93_e1355_d_n3: f64 = (eq93_e1353_d_n3 * (nv18 - nv17));
        let eq93_e1355_d_n4: f64 = (eq93_e1353_d_n4 * (nv18 - nv17));
        let eq93_e1355_d_n5: f64 = (eq93_e1353_d_n5 * (nv18 - nv17));
        let eq93_e1355_d_n6: f64 = (eq93_e1353_d_n6 * (nv18 - nv17));
        let eq93_e1355_d_n7: f64 = (eq93_e1353_d_n7 * (nv18 - nv17));
        let eq93_e1355_d_n8: f64 = (eq93_e1353_d_n8 * (nv18 - nv17));
        let eq93_e1355_d_n9: f64 = (eq93_e1353_d_n9 * (nv18 - nv17));
        let eq93_e1355_d_n10: f64 = (eq93_e1353_d_n10 * (nv18 - nv17));
        let eq93_e1355_d_n11: f64 = (eq93_e1353_d_n11 * (nv18 - nv17));
        let eq93_e1355_d_n12: f64 = (eq93_e1353_d_n12 * (nv18 - nv17));
        let eq93_e1355_d_n13: f64 = (eq93_e1353_d_n13 * (nv18 - nv17));
        let eq93_e1355_d_n14: f64 = (eq93_e1353_d_n14 * (nv18 - nv17));
        let eq93_e1355_d_n15: f64 = (eq93_e1353_d_n15 * (nv18 - nv17));
        let eq93_e1355_d_n16: f64 = (eq93_e1353_d_n16 * (nv18 - nv17));
        let eq93_e1355_d_n17: f64 = ((eq93_e1353_d_n17 * (nv18 - nv17)) + (-eq93_e1353));
        let eq93_e1355_d_n18: f64 = ((eq93_e1353_d_n18 * (nv18 - nv17)) + eq93_e1353);
        let eq93_e1355_d_n19: f64 = (eq93_e1353_d_n19 * (nv18 - nv17));
        let eq93_e1355_d_n20: f64 = (eq93_e1353_d_n20 * (nv18 - nv17));
        let eq93_e1355_d_n21: f64 = (eq93_e1353_d_n21 * (nv18 - nv17));
        let eq93_e1355_d_n22: f64 = (eq93_e1353_d_n22 * (nv18 - nv17));
        let eq93_e1355_d_b0: f64 = (eq93_e1353_d_b0 * (nv18 - nv17));
        let eq93_e1355_d_b1: f64 = (eq93_e1353_d_b1 * (nv18 - nv17));
        let eq93_e1355_d_b2: f64 = (eq93_e1353_d_b2 * (nv18 - nv17));
        let eq93_e1355_d_b3: f64 = (eq93_e1353_d_b3 * (nv18 - nv17));
        let eq93_e1355_d_b4: f64 = (eq93_e1353_d_b4 * (nv18 - nv17));
        let eq93_e1355_d_b5: f64 = (eq93_e1353_d_b5 * (nv18 - nv17));
        let eq93_e1355_d_b6: f64 = (eq93_e1353_d_b6 * (nv18 - nv17));
        let eq93_e1355_d_b7: f64 = (eq93_e1353_d_b7 * (nv18 - nv17));
        let eq93_e1355_d_b8: f64 = (eq93_e1353_d_b8 * (nv18 - nv17));
        let eq93_e1355_d_b9: f64 = (eq93_e1353_d_b9 * (nv18 - nv17));
        let eq93_e1355_d_b10: f64 = (eq93_e1353_d_b10 * (nv18 - nv17));
        let eq93_e1355_d_b11: f64 = (eq93_e1353_d_b11 * (nv18 - nv17));
        let eq93_e1355_d_b12: f64 = (eq93_e1353_d_b12 * (nv18 - nv17));
        let eq93_e1355_d_b13: f64 = (eq93_e1353_d_b13 * (nv18 - nv17));
        let eq93_e1355_d_b14: f64 = (eq93_e1353_d_b14 * (nv18 - nv17));
        let eq93_e1355_d_b15: f64 = (eq93_e1353_d_b15 * (nv18 - nv17));
        let eq93_e1355_d_b16: f64 = (eq93_e1353_d_b16 * (nv18 - nv17));
        let eq93_e1355_d_b17: f64 = (eq93_e1353_d_b17 * (nv18 - nv17));
        let eq93_e1355_d_b18: f64 = (eq93_e1353_d_b18 * (nv18 - nv17));
        let eq93_e1355_d_b19: f64 = (eq93_e1353_d_b19 * (nv18 - nv17));
        let eq93_e1355_d_b20: f64 = (eq93_e1353_d_b20 * (nv18 - nv17));
        let eq93_e1355_d_b21: f64 = (eq93_e1353_d_b21 * (nv18 - nv17));
        let eq93_e1355_d_b22: f64 = (eq93_e1353_d_b22 * (nv18 - nv17));
        let eq93_e1355_d_b23: f64 = (eq93_e1353_d_b23 * (nv18 - nv17));
        let eq93_e1355_d_b24: f64 = (eq93_e1353_d_b24 * (nv18 - nv17));
        let eq93_e1355_d_b25: f64 = (eq93_e1353_d_b25 * (nv18 - nv17));
        let eq93_e1355_d_b26: f64 = (eq93_e1353_d_b26 * (nv18 - nv17));
        let eq93_e1355_d_b27: f64 = (eq93_e1353_d_b27 * (nv18 - nv17));
        let eq93_e1355_d_b28: f64 = (eq93_e1353_d_b28 * (nv18 - nv17));
        let eq93_e1355_d_b29: f64 = (eq93_e1353_d_b29 * (nv18 - nv17));
        let eq93_e1355_d_b30: f64 = (eq93_e1353_d_b30 * (nv18 - nv17));
        let eq93_e1355_d_b31: f64 = (eq93_e1353_d_b31 * (nv18 - nv17));
        let eq93_e1355_d_b32: f64 = (eq93_e1353_d_b32 * (nv18 - nv17));
        let eq93_e1355_d_b33: f64 = (eq93_e1353_d_b33 * (nv18 - nv17));
        let eq93_e1355_d_b34: f64 = (eq93_e1353_d_b34 * (nv18 - nv17));
        let eq93_e1355_d_b35: f64 = (eq93_e1353_d_b35 * (nv18 - nv17));
        let eq93_e1355_d_b36: f64 = (eq93_e1353_d_b36 * (nv18 - nv17));
        let eq93_e1355_d_b37: f64 = (eq93_e1353_d_b37 * (nv18 - nv17));
        let eq93_e1355_d_b38: f64 = (eq93_e1353_d_b38 * (nv18 - nv17));
        let eq93_e1355_d_b39: f64 = (eq93_e1353_d_b39 * (nv18 - nv17));
        let eq93_e1355_d_b40: f64 = (eq93_e1353_d_b40 * (nv18 - nv17));
        let eq93_e1355_d_b41: f64 = (eq93_e1353_d_b41 * (nv18 - nv17));
        let eq93_e1355_d_b42: f64 = (eq93_e1353_d_b42 * (nv18 - nv17));
        let eq93_e1355_d_b43: f64 = (eq93_e1353_d_b43 * (nv18 - nv17));
        let eq93_e1355_d_b44: f64 = (eq93_e1353_d_b44 * (nv18 - nv17));
        let eq93_e1355_d_b45: f64 = (eq93_e1353_d_b45 * (nv18 - nv17));
        let eq93_e1355_d_b46: f64 = (eq93_e1353_d_b46 * (nv18 - nv17));
        let eq93_e1355_d_b47: f64 = (eq93_e1353_d_b47 * (nv18 - nv17));
        let eq93_e1355_d_b48: f64 = (eq93_e1353_d_b48 * (nv18 - nv17));
        let eq93_e1355_d_b49: f64 = (eq93_e1353_d_b49 * (nv18 - nv17));
        let eq93_e1355_d_b50: f64 = (eq93_e1353_d_b50 * (nv18 - nv17));
        let eq93_e1355_d_b51: f64 = (eq93_e1353_d_b51 * (nv18 - nv17));
        let eq93_e1355_d_b52: f64 = (eq93_e1353_d_b52 * (nv18 - nv17));
        let eq93_e1355_d_b53: f64 = (eq93_e1353_d_b53 * (nv18 - nv17));
        let eq93_e1355_d_b54: f64 = (eq93_e1353_d_b54 * (nv18 - nv17));
        let eq93_e1356: f64 = (eq93_e1350 + eq93_e1355);
        let eq93_e1356_d_n0: f64 = (eq93_e1350_d_n0 + eq93_e1355_d_n0);
        let eq93_e1356_d_n1: f64 = (eq93_e1350_d_n1 + eq93_e1355_d_n1);
        let eq93_e1356_d_n2: f64 = (eq93_e1350_d_n2 + eq93_e1355_d_n2);
        let eq93_e1356_d_n3: f64 = (eq93_e1350_d_n3 + eq93_e1355_d_n3);
        let eq93_e1356_d_n4: f64 = (eq93_e1350_d_n4 + eq93_e1355_d_n4);
        let eq93_e1356_d_n5: f64 = (eq93_e1350_d_n5 + eq93_e1355_d_n5);
        let eq93_e1356_d_n6: f64 = (eq93_e1350_d_n6 + eq93_e1355_d_n6);
        let eq93_e1356_d_n7: f64 = (eq93_e1350_d_n7 + eq93_e1355_d_n7);
        let eq93_e1356_d_n8: f64 = (eq93_e1350_d_n8 + eq93_e1355_d_n8);
        let eq93_e1356_d_n9: f64 = (eq93_e1350_d_n9 + eq93_e1355_d_n9);
        let eq93_e1356_d_n10: f64 = (eq93_e1350_d_n10 + eq93_e1355_d_n10);
        let eq93_e1356_d_n11: f64 = (eq93_e1350_d_n11 + eq93_e1355_d_n11);
        let eq93_e1356_d_n12: f64 = (eq93_e1350_d_n12 + eq93_e1355_d_n12);
        let eq93_e1356_d_n13: f64 = (eq93_e1350_d_n13 + eq93_e1355_d_n13);
        let eq93_e1356_d_n14: f64 = (eq93_e1350_d_n14 + eq93_e1355_d_n14);
        let eq93_e1356_d_n15: f64 = (eq93_e1350_d_n15 + eq93_e1355_d_n15);
        let eq93_e1356_d_n16: f64 = (eq93_e1350_d_n16 + eq93_e1355_d_n16);
        let eq93_e1356_d_n17: f64 = (eq93_e1350_d_n17 + eq93_e1355_d_n17);
        let eq93_e1356_d_n18: f64 = (eq93_e1350_d_n18 + eq93_e1355_d_n18);
        let eq93_e1356_d_n19: f64 = (eq93_e1350_d_n19 + eq93_e1355_d_n19);
        let eq93_e1356_d_n20: f64 = (eq93_e1350_d_n20 + eq93_e1355_d_n20);
        let eq93_e1356_d_n21: f64 = (eq93_e1350_d_n21 + eq93_e1355_d_n21);
        let eq93_e1356_d_n22: f64 = (eq93_e1350_d_n22 + eq93_e1355_d_n22);
        let eq93_e1356_d_b0: f64 = (eq93_e1350_d_b0 + eq93_e1355_d_b0);
        let eq93_e1356_d_b1: f64 = (eq93_e1350_d_b1 + eq93_e1355_d_b1);
        let eq93_e1356_d_b2: f64 = (eq93_e1350_d_b2 + eq93_e1355_d_b2);
        let eq93_e1356_d_b3: f64 = (eq93_e1350_d_b3 + eq93_e1355_d_b3);
        let eq93_e1356_d_b4: f64 = (eq93_e1350_d_b4 + eq93_e1355_d_b4);
        let eq93_e1356_d_b5: f64 = (eq93_e1350_d_b5 + eq93_e1355_d_b5);
        let eq93_e1356_d_b6: f64 = (eq93_e1350_d_b6 + eq93_e1355_d_b6);
        let eq93_e1356_d_b7: f64 = (eq93_e1350_d_b7 + eq93_e1355_d_b7);
        let eq93_e1356_d_b8: f64 = (eq93_e1350_d_b8 + eq93_e1355_d_b8);
        let eq93_e1356_d_b9: f64 = (eq93_e1350_d_b9 + eq93_e1355_d_b9);
        let eq93_e1356_d_b10: f64 = (eq93_e1350_d_b10 + eq93_e1355_d_b10);
        let eq93_e1356_d_b11: f64 = (eq93_e1350_d_b11 + eq93_e1355_d_b11);
        let eq93_e1356_d_b12: f64 = (eq93_e1350_d_b12 + eq93_e1355_d_b12);
        let eq93_e1356_d_b13: f64 = (eq93_e1350_d_b13 + eq93_e1355_d_b13);
        let eq93_e1356_d_b14: f64 = (eq93_e1350_d_b14 + eq93_e1355_d_b14);
        let eq93_e1356_d_b15: f64 = (eq93_e1350_d_b15 + eq93_e1355_d_b15);
        let eq93_e1356_d_b16: f64 = (eq93_e1350_d_b16 + eq93_e1355_d_b16);
        let eq93_e1356_d_b17: f64 = (eq93_e1350_d_b17 + eq93_e1355_d_b17);
        let eq93_e1356_d_b18: f64 = (eq93_e1350_d_b18 + eq93_e1355_d_b18);
        let eq93_e1356_d_b19: f64 = (eq93_e1350_d_b19 + eq93_e1355_d_b19);
        let eq93_e1356_d_b20: f64 = (eq93_e1350_d_b20 + eq93_e1355_d_b20);
        let eq93_e1356_d_b21: f64 = (eq93_e1350_d_b21 + eq93_e1355_d_b21);
        let eq93_e1356_d_b22: f64 = (eq93_e1350_d_b22 + eq93_e1355_d_b22);
        let eq93_e1356_d_b23: f64 = (eq93_e1350_d_b23 + eq93_e1355_d_b23);
        let eq93_e1356_d_b24: f64 = (eq93_e1350_d_b24 + eq93_e1355_d_b24);
        let eq93_e1356_d_b25: f64 = (eq93_e1350_d_b25 + eq93_e1355_d_b25);
        let eq93_e1356_d_b26: f64 = (eq93_e1350_d_b26 + eq93_e1355_d_b26);
        let eq93_e1356_d_b27: f64 = (eq93_e1350_d_b27 + eq93_e1355_d_b27);
        let eq93_e1356_d_b28: f64 = (eq93_e1350_d_b28 + eq93_e1355_d_b28);
        let eq93_e1356_d_b29: f64 = (eq93_e1350_d_b29 + eq93_e1355_d_b29);
        let eq93_e1356_d_b30: f64 = (eq93_e1350_d_b30 + eq93_e1355_d_b30);
        let eq93_e1356_d_b31: f64 = (eq93_e1350_d_b31 + eq93_e1355_d_b31);
        let eq93_e1356_d_b32: f64 = (eq93_e1350_d_b32 + eq93_e1355_d_b32);
        let eq93_e1356_d_b33: f64 = (eq93_e1350_d_b33 + eq93_e1355_d_b33);
        let eq93_e1356_d_b34: f64 = (eq93_e1350_d_b34 + eq93_e1355_d_b34);
        let eq93_e1356_d_b35: f64 = (eq93_e1350_d_b35 + eq93_e1355_d_b35);
        let eq93_e1356_d_b36: f64 = (eq93_e1350_d_b36 + eq93_e1355_d_b36);
        let eq93_e1356_d_b37: f64 = (eq93_e1350_d_b37 + eq93_e1355_d_b37);
        let eq93_e1356_d_b38: f64 = (eq93_e1350_d_b38 + eq93_e1355_d_b38);
        let eq93_e1356_d_b39: f64 = (eq93_e1350_d_b39 + eq93_e1355_d_b39);
        let eq93_e1356_d_b40: f64 = (eq93_e1350_d_b40 + eq93_e1355_d_b40);
        let eq93_e1356_d_b41: f64 = (eq93_e1350_d_b41 + eq93_e1355_d_b41);
        let eq93_e1356_d_b42: f64 = (eq93_e1350_d_b42 + eq93_e1355_d_b42);
        let eq93_e1356_d_b43: f64 = (eq93_e1350_d_b43 + eq93_e1355_d_b43);
        let eq93_e1356_d_b44: f64 = (eq93_e1350_d_b44 + eq93_e1355_d_b44);
        let eq93_e1356_d_b45: f64 = (eq93_e1350_d_b45 + eq93_e1355_d_b45);
        let eq93_e1356_d_b46: f64 = (eq93_e1350_d_b46 + eq93_e1355_d_b46);
        let eq93_e1356_d_b47: f64 = (eq93_e1350_d_b47 + eq93_e1355_d_b47);
        let eq93_e1356_d_b48: f64 = (eq93_e1350_d_b48 + eq93_e1355_d_b48);
        let eq93_e1356_d_b49: f64 = (eq93_e1350_d_b49 + eq93_e1355_d_b49);
        let eq93_e1356_d_b50: f64 = (eq93_e1350_d_b50 + eq93_e1355_d_b50);
        let eq93_e1356_d_b51: f64 = (eq93_e1350_d_b51 + eq93_e1355_d_b51);
        let eq93_e1356_d_b52: f64 = (eq93_e1350_d_b52 + eq93_e1355_d_b52);
        let eq93_e1356_d_b53: f64 = (eq93_e1350_d_b53 + eq93_e1355_d_b53);
        let eq93_e1356_d_b54: f64 = (eq93_e1350_d_b54 + eq93_e1355_d_b54);
        (eq93_e1356, eq93_e1356_d_n0, eq93_e1356_d_n1, eq93_e1356_d_n2, eq93_e1356_d_n3, eq93_e1356_d_n4, eq93_e1356_d_n5, eq93_e1356_d_n6, eq93_e1356_d_n7, eq93_e1356_d_n8, eq93_e1356_d_n9, eq93_e1356_d_n10, eq93_e1356_d_n11, eq93_e1356_d_n12, eq93_e1356_d_n13, eq93_e1356_d_n14, eq93_e1356_d_n15, eq93_e1356_d_n16, eq93_e1356_d_n17, eq93_e1356_d_n18, eq93_e1356_d_n19, eq93_e1356_d_n20, eq93_e1356_d_n21, eq93_e1356_d_n22, eq93_e1356_d_b0, eq93_e1356_d_b1, eq93_e1356_d_b2, eq93_e1356_d_b3, eq93_e1356_d_b4, eq93_e1356_d_b5, eq93_e1356_d_b6, eq93_e1356_d_b7, eq93_e1356_d_b8, eq93_e1356_d_b9, eq93_e1356_d_b10, eq93_e1356_d_b11, eq93_e1356_d_b12, eq93_e1356_d_b13, eq93_e1356_d_b14, eq93_e1356_d_b15, eq93_e1356_d_b16, eq93_e1356_d_b17, eq93_e1356_d_b18, eq93_e1356_d_b19, eq93_e1356_d_b20, eq93_e1356_d_b21, eq93_e1356_d_b22, eq93_e1356_d_b23, eq93_e1356_d_b24, eq93_e1356_d_b25, eq93_e1356_d_b26, eq93_e1356_d_b27, eq93_e1356_d_b28, eq93_e1356_d_b29, eq93_e1356_d_b30, eq93_e1356_d_b31, eq93_e1356_d_b32, eq93_e1356_d_b33, eq93_e1356_d_b34, eq93_e1356_d_b35, eq93_e1356_d_b36, eq93_e1356_d_b37, eq93_e1356_d_b38, eq93_e1356_d_b39, eq93_e1356_d_b40, eq93_e1356_d_b41, eq93_e1356_d_b42, eq93_e1356_d_b43, eq93_e1356_d_b44, eq93_e1356_d_b45, eq93_e1356_d_b46, eq93_e1356_d_b47, eq93_e1356_d_b48, eq93_e1356_d_b49, eq93_e1356_d_b50, eq93_e1356_d_b51, eq93_e1356_d_b52, eq93_e1356_d_b53, eq93_e1356_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq93_value: f64 = eq93_e1358;
        let eq93_node_derivatives: [f64; 23] = [eq93_e1358_d_n0, eq93_e1358_d_n1, eq93_e1358_d_n2, eq93_e1358_d_n3, eq93_e1358_d_n4, eq93_e1358_d_n5, eq93_e1358_d_n6, eq93_e1358_d_n7, eq93_e1358_d_n8, eq93_e1358_d_n9, eq93_e1358_d_n10, eq93_e1358_d_n11, eq93_e1358_d_n12, eq93_e1358_d_n13, eq93_e1358_d_n14, eq93_e1358_d_n15, eq93_e1358_d_n16, eq93_e1358_d_n17, eq93_e1358_d_n18, eq93_e1358_d_n19, eq93_e1358_d_n20, eq93_e1358_d_n21, eq93_e1358_d_n22];
        let eq93_branch_derivatives: [f64; 55] = [eq93_e1358_d_b0, eq93_e1358_d_b1, eq93_e1358_d_b2, eq93_e1358_d_b3, eq93_e1358_d_b4, eq93_e1358_d_b5, eq93_e1358_d_b6, eq93_e1358_d_b7, eq93_e1358_d_b8, eq93_e1358_d_b9, eq93_e1358_d_b10, eq93_e1358_d_b11, eq93_e1358_d_b12, eq93_e1358_d_b13, eq93_e1358_d_b14, eq93_e1358_d_b15, eq93_e1358_d_b16, eq93_e1358_d_b17, eq93_e1358_d_b18, eq93_e1358_d_b19, eq93_e1358_d_b20, eq93_e1358_d_b21, eq93_e1358_d_b22, eq93_e1358_d_b23, eq93_e1358_d_b24, eq93_e1358_d_b25, eq93_e1358_d_b26, eq93_e1358_d_b27, eq93_e1358_d_b28, eq93_e1358_d_b29, eq93_e1358_d_b30, eq93_e1358_d_b31, eq93_e1358_d_b32, eq93_e1358_d_b33, eq93_e1358_d_b34, eq93_e1358_d_b35, eq93_e1358_d_b36, eq93_e1358_d_b37, eq93_e1358_d_b38, eq93_e1358_d_b39, eq93_e1358_d_b40, eq93_e1358_d_b41, eq93_e1358_d_b42, eq93_e1358_d_b43, eq93_e1358_d_b44, eq93_e1358_d_b45, eq93_e1358_d_b46, eq93_e1358_d_b47, eq93_e1358_d_b48, eq93_e1358_d_b49, eq93_e1358_d_b50, eq93_e1358_d_b51, eq93_e1358_d_b52, eq93_e1358_d_b53, eq93_e1358_d_b54];
        stamper.stamp_current_dense_local(
            Some(18),
            Some(17),
            multiplicity * (eq93_value),
            &eq93_node_derivatives,
            &eq93_branch_derivatives,
            multiplicity,
        );
        let (eq94_e1365,) = {
    if (s.b[523] && (!s.b[524])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq94_value: f64 = eq94_e1365;
        stamper.stamp_potential_const_local(
            48,
            eq94_value,
        );
        let (eq95_e1370,) = {
    if (!s.b[523]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq95_value: f64 = eq95_e1370;
        stamper.stamp_potential_const_local(
            49,
            eq95_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_19(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv21 = ctx.node_voltage(nodes[21]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let (eq96_e1386, eq96_e1386_d_n0, eq96_e1386_d_n1, eq96_e1386_d_n2, eq96_e1386_d_n3, eq96_e1386_d_n4, eq96_e1386_d_n5, eq96_e1386_d_n6, eq96_e1386_d_n7, eq96_e1386_d_n8, eq96_e1386_d_n9, eq96_e1386_d_n10, eq96_e1386_d_n11, eq96_e1386_d_n12, eq96_e1386_d_n13, eq96_e1386_d_n14, eq96_e1386_d_n15, eq96_e1386_d_n16, eq96_e1386_d_n17, eq96_e1386_d_n18, eq96_e1386_d_n19, eq96_e1386_d_n20, eq96_e1386_d_n21, eq96_e1386_d_n22, eq96_e1386_d_b0, eq96_e1386_d_b1, eq96_e1386_d_b2, eq96_e1386_d_b3, eq96_e1386_d_b4, eq96_e1386_d_b5, eq96_e1386_d_b6, eq96_e1386_d_b7, eq96_e1386_d_b8, eq96_e1386_d_b9, eq96_e1386_d_b10, eq96_e1386_d_b11, eq96_e1386_d_b12, eq96_e1386_d_b13, eq96_e1386_d_b14, eq96_e1386_d_b15, eq96_e1386_d_b16, eq96_e1386_d_b17, eq96_e1386_d_b18, eq96_e1386_d_b19, eq96_e1386_d_b20, eq96_e1386_d_b21, eq96_e1386_d_b22, eq96_e1386_d_b23, eq96_e1386_d_b24, eq96_e1386_d_b25, eq96_e1386_d_b26, eq96_e1386_d_b27, eq96_e1386_d_b28, eq96_e1386_d_b29, eq96_e1386_d_b30, eq96_e1386_d_b31, eq96_e1386_d_b32, eq96_e1386_d_b33, eq96_e1386_d_b34, eq96_e1386_d_b35, eq96_e1386_d_b36, eq96_e1386_d_b37, eq96_e1386_d_b38, eq96_e1386_d_b39, eq96_e1386_d_b40, eq96_e1386_d_b41, eq96_e1386_d_b42, eq96_e1386_d_b43, eq96_e1386_d_b44, eq96_e1386_d_b45, eq96_e1386_d_b46, eq96_e1386_d_b47, eq96_e1386_d_b48, eq96_e1386_d_b49, eq96_e1386_d_b50, eq96_e1386_d_b51, eq96_e1386_d_b52, eq96_e1386_d_b53, eq96_e1386_d_b54,) = {
    if (s.b[538] && s.b[539]) {
        let eq96_e1376: f64 = (p.p6 * s.v[76]);
        let eq96_e1376_d_n0: f64 = (p.p6 * s.dn[76][0]);
        let eq96_e1376_d_n1: f64 = (p.p6 * s.dn[76][1]);
        let eq96_e1376_d_n2: f64 = (p.p6 * s.dn[76][2]);
        let eq96_e1376_d_n3: f64 = (p.p6 * s.dn[76][3]);
        let eq96_e1376_d_n4: f64 = (p.p6 * s.dn[76][4]);
        let eq96_e1376_d_n5: f64 = (p.p6 * s.dn[76][5]);
        let eq96_e1376_d_n6: f64 = (p.p6 * s.dn[76][6]);
        let eq96_e1376_d_n7: f64 = (p.p6 * s.dn[76][7]);
        let eq96_e1376_d_n8: f64 = (p.p6 * s.dn[76][8]);
        let eq96_e1376_d_n9: f64 = (p.p6 * s.dn[76][9]);
        let eq96_e1376_d_n10: f64 = (p.p6 * s.dn[76][10]);
        let eq96_e1376_d_n11: f64 = (p.p6 * s.dn[76][11]);
        let eq96_e1376_d_n12: f64 = (p.p6 * s.dn[76][12]);
        let eq96_e1376_d_n13: f64 = (p.p6 * s.dn[76][13]);
        let eq96_e1376_d_n14: f64 = (p.p6 * s.dn[76][14]);
        let eq96_e1376_d_n15: f64 = (p.p6 * s.dn[76][15]);
        let eq96_e1376_d_n16: f64 = (p.p6 * s.dn[76][16]);
        let eq96_e1376_d_n17: f64 = (p.p6 * s.dn[76][17]);
        let eq96_e1376_d_n18: f64 = (p.p6 * s.dn[76][18]);
        let eq96_e1376_d_n19: f64 = (p.p6 * s.dn[76][19]);
        let eq96_e1376_d_n20: f64 = (p.p6 * s.dn[76][20]);
        let eq96_e1376_d_n21: f64 = (p.p6 * s.dn[76][21]);
        let eq96_e1376_d_n22: f64 = (p.p6 * s.dn[76][22]);
        let eq96_e1376_d_b0: f64 = (p.p6 * s.db[76][0]);
        let eq96_e1376_d_b1: f64 = (p.p6 * s.db[76][1]);
        let eq96_e1376_d_b2: f64 = (p.p6 * s.db[76][2]);
        let eq96_e1376_d_b3: f64 = (p.p6 * s.db[76][3]);
        let eq96_e1376_d_b4: f64 = (p.p6 * s.db[76][4]);
        let eq96_e1376_d_b5: f64 = (p.p6 * s.db[76][5]);
        let eq96_e1376_d_b6: f64 = (p.p6 * s.db[76][6]);
        let eq96_e1376_d_b7: f64 = (p.p6 * s.db[76][7]);
        let eq96_e1376_d_b8: f64 = (p.p6 * s.db[76][8]);
        let eq96_e1376_d_b9: f64 = (p.p6 * s.db[76][9]);
        let eq96_e1376_d_b10: f64 = (p.p6 * s.db[76][10]);
        let eq96_e1376_d_b11: f64 = (p.p6 * s.db[76][11]);
        let eq96_e1376_d_b12: f64 = (p.p6 * s.db[76][12]);
        let eq96_e1376_d_b13: f64 = (p.p6 * s.db[76][13]);
        let eq96_e1376_d_b14: f64 = (p.p6 * s.db[76][14]);
        let eq96_e1376_d_b15: f64 = (p.p6 * s.db[76][15]);
        let eq96_e1376_d_b16: f64 = (p.p6 * s.db[76][16]);
        let eq96_e1376_d_b17: f64 = (p.p6 * s.db[76][17]);
        let eq96_e1376_d_b18: f64 = (p.p6 * s.db[76][18]);
        let eq96_e1376_d_b19: f64 = (p.p6 * s.db[76][19]);
        let eq96_e1376_d_b20: f64 = (p.p6 * s.db[76][20]);
        let eq96_e1376_d_b21: f64 = (p.p6 * s.db[76][21]);
        let eq96_e1376_d_b22: f64 = (p.p6 * s.db[76][22]);
        let eq96_e1376_d_b23: f64 = (p.p6 * s.db[76][23]);
        let eq96_e1376_d_b24: f64 = (p.p6 * s.db[76][24]);
        let eq96_e1376_d_b25: f64 = (p.p6 * s.db[76][25]);
        let eq96_e1376_d_b26: f64 = (p.p6 * s.db[76][26]);
        let eq96_e1376_d_b27: f64 = (p.p6 * s.db[76][27]);
        let eq96_e1376_d_b28: f64 = (p.p6 * s.db[76][28]);
        let eq96_e1376_d_b29: f64 = (p.p6 * s.db[76][29]);
        let eq96_e1376_d_b30: f64 = (p.p6 * s.db[76][30]);
        let eq96_e1376_d_b31: f64 = (p.p6 * s.db[76][31]);
        let eq96_e1376_d_b32: f64 = (p.p6 * s.db[76][32]);
        let eq96_e1376_d_b33: f64 = (p.p6 * s.db[76][33]);
        let eq96_e1376_d_b34: f64 = (p.p6 * s.db[76][34]);
        let eq96_e1376_d_b35: f64 = (p.p6 * s.db[76][35]);
        let eq96_e1376_d_b36: f64 = (p.p6 * s.db[76][36]);
        let eq96_e1376_d_b37: f64 = (p.p6 * s.db[76][37]);
        let eq96_e1376_d_b38: f64 = (p.p6 * s.db[76][38]);
        let eq96_e1376_d_b39: f64 = (p.p6 * s.db[76][39]);
        let eq96_e1376_d_b40: f64 = (p.p6 * s.db[76][40]);
        let eq96_e1376_d_b41: f64 = (p.p6 * s.db[76][41]);
        let eq96_e1376_d_b42: f64 = (p.p6 * s.db[76][42]);
        let eq96_e1376_d_b43: f64 = (p.p6 * s.db[76][43]);
        let eq96_e1376_d_b44: f64 = (p.p6 * s.db[76][44]);
        let eq96_e1376_d_b45: f64 = (p.p6 * s.db[76][45]);
        let eq96_e1376_d_b46: f64 = (p.p6 * s.db[76][46]);
        let eq96_e1376_d_b47: f64 = (p.p6 * s.db[76][47]);
        let eq96_e1376_d_b48: f64 = (p.p6 * s.db[76][48]);
        let eq96_e1376_d_b49: f64 = (p.p6 * s.db[76][49]);
        let eq96_e1376_d_b50: f64 = (p.p6 * s.db[76][50]);
        let eq96_e1376_d_b51: f64 = (p.p6 * s.db[76][51]);
        let eq96_e1376_d_b52: f64 = (p.p6 * s.db[76][52]);
        let eq96_e1376_d_b53: f64 = (p.p6 * s.db[76][53]);
        let eq96_e1376_d_b54: f64 = (p.p6 * s.db[76][54]);
        let eq96_e1378: f64 = (eq96_e1376 * s.v[317]);
        let eq96_e1378_d_n0: f64 = ((eq96_e1376_d_n0 * s.v[317]) + (eq96_e1376 * s.dn[317][0]));
        let eq96_e1378_d_n1: f64 = ((eq96_e1376_d_n1 * s.v[317]) + (eq96_e1376 * s.dn[317][1]));
        let eq96_e1378_d_n2: f64 = ((eq96_e1376_d_n2 * s.v[317]) + (eq96_e1376 * s.dn[317][2]));
        let eq96_e1378_d_n3: f64 = ((eq96_e1376_d_n3 * s.v[317]) + (eq96_e1376 * s.dn[317][3]));
        let eq96_e1378_d_n4: f64 = ((eq96_e1376_d_n4 * s.v[317]) + (eq96_e1376 * s.dn[317][4]));
        let eq96_e1378_d_n5: f64 = ((eq96_e1376_d_n5 * s.v[317]) + (eq96_e1376 * s.dn[317][5]));
        let eq96_e1378_d_n6: f64 = ((eq96_e1376_d_n6 * s.v[317]) + (eq96_e1376 * s.dn[317][6]));
        let eq96_e1378_d_n7: f64 = ((eq96_e1376_d_n7 * s.v[317]) + (eq96_e1376 * s.dn[317][7]));
        let eq96_e1378_d_n8: f64 = ((eq96_e1376_d_n8 * s.v[317]) + (eq96_e1376 * s.dn[317][8]));
        let eq96_e1378_d_n9: f64 = ((eq96_e1376_d_n9 * s.v[317]) + (eq96_e1376 * s.dn[317][9]));
        let eq96_e1378_d_n10: f64 = ((eq96_e1376_d_n10 * s.v[317]) + (eq96_e1376 * s.dn[317][10]));
        let eq96_e1378_d_n11: f64 = ((eq96_e1376_d_n11 * s.v[317]) + (eq96_e1376 * s.dn[317][11]));
        let eq96_e1378_d_n12: f64 = ((eq96_e1376_d_n12 * s.v[317]) + (eq96_e1376 * s.dn[317][12]));
        let eq96_e1378_d_n13: f64 = ((eq96_e1376_d_n13 * s.v[317]) + (eq96_e1376 * s.dn[317][13]));
        let eq96_e1378_d_n14: f64 = ((eq96_e1376_d_n14 * s.v[317]) + (eq96_e1376 * s.dn[317][14]));
        let eq96_e1378_d_n15: f64 = ((eq96_e1376_d_n15 * s.v[317]) + (eq96_e1376 * s.dn[317][15]));
        let eq96_e1378_d_n16: f64 = ((eq96_e1376_d_n16 * s.v[317]) + (eq96_e1376 * s.dn[317][16]));
        let eq96_e1378_d_n17: f64 = ((eq96_e1376_d_n17 * s.v[317]) + (eq96_e1376 * s.dn[317][17]));
        let eq96_e1378_d_n18: f64 = ((eq96_e1376_d_n18 * s.v[317]) + (eq96_e1376 * s.dn[317][18]));
        let eq96_e1378_d_n19: f64 = ((eq96_e1376_d_n19 * s.v[317]) + (eq96_e1376 * s.dn[317][19]));
        let eq96_e1378_d_n20: f64 = ((eq96_e1376_d_n20 * s.v[317]) + (eq96_e1376 * s.dn[317][20]));
        let eq96_e1378_d_n21: f64 = ((eq96_e1376_d_n21 * s.v[317]) + (eq96_e1376 * s.dn[317][21]));
        let eq96_e1378_d_n22: f64 = ((eq96_e1376_d_n22 * s.v[317]) + (eq96_e1376 * s.dn[317][22]));
        let eq96_e1378_d_b0: f64 = ((eq96_e1376_d_b0 * s.v[317]) + (eq96_e1376 * s.db[317][0]));
        let eq96_e1378_d_b1: f64 = ((eq96_e1376_d_b1 * s.v[317]) + (eq96_e1376 * s.db[317][1]));
        let eq96_e1378_d_b2: f64 = ((eq96_e1376_d_b2 * s.v[317]) + (eq96_e1376 * s.db[317][2]));
        let eq96_e1378_d_b3: f64 = ((eq96_e1376_d_b3 * s.v[317]) + (eq96_e1376 * s.db[317][3]));
        let eq96_e1378_d_b4: f64 = ((eq96_e1376_d_b4 * s.v[317]) + (eq96_e1376 * s.db[317][4]));
        let eq96_e1378_d_b5: f64 = ((eq96_e1376_d_b5 * s.v[317]) + (eq96_e1376 * s.db[317][5]));
        let eq96_e1378_d_b6: f64 = ((eq96_e1376_d_b6 * s.v[317]) + (eq96_e1376 * s.db[317][6]));
        let eq96_e1378_d_b7: f64 = ((eq96_e1376_d_b7 * s.v[317]) + (eq96_e1376 * s.db[317][7]));
        let eq96_e1378_d_b8: f64 = ((eq96_e1376_d_b8 * s.v[317]) + (eq96_e1376 * s.db[317][8]));
        let eq96_e1378_d_b9: f64 = ((eq96_e1376_d_b9 * s.v[317]) + (eq96_e1376 * s.db[317][9]));
        let eq96_e1378_d_b10: f64 = ((eq96_e1376_d_b10 * s.v[317]) + (eq96_e1376 * s.db[317][10]));
        let eq96_e1378_d_b11: f64 = ((eq96_e1376_d_b11 * s.v[317]) + (eq96_e1376 * s.db[317][11]));
        let eq96_e1378_d_b12: f64 = ((eq96_e1376_d_b12 * s.v[317]) + (eq96_e1376 * s.db[317][12]));
        let eq96_e1378_d_b13: f64 = ((eq96_e1376_d_b13 * s.v[317]) + (eq96_e1376 * s.db[317][13]));
        let eq96_e1378_d_b14: f64 = ((eq96_e1376_d_b14 * s.v[317]) + (eq96_e1376 * s.db[317][14]));
        let eq96_e1378_d_b15: f64 = ((eq96_e1376_d_b15 * s.v[317]) + (eq96_e1376 * s.db[317][15]));
        let eq96_e1378_d_b16: f64 = ((eq96_e1376_d_b16 * s.v[317]) + (eq96_e1376 * s.db[317][16]));
        let eq96_e1378_d_b17: f64 = ((eq96_e1376_d_b17 * s.v[317]) + (eq96_e1376 * s.db[317][17]));
        let eq96_e1378_d_b18: f64 = ((eq96_e1376_d_b18 * s.v[317]) + (eq96_e1376 * s.db[317][18]));
        let eq96_e1378_d_b19: f64 = ((eq96_e1376_d_b19 * s.v[317]) + (eq96_e1376 * s.db[317][19]));
        let eq96_e1378_d_b20: f64 = ((eq96_e1376_d_b20 * s.v[317]) + (eq96_e1376 * s.db[317][20]));
        let eq96_e1378_d_b21: f64 = ((eq96_e1376_d_b21 * s.v[317]) + (eq96_e1376 * s.db[317][21]));
        let eq96_e1378_d_b22: f64 = ((eq96_e1376_d_b22 * s.v[317]) + (eq96_e1376 * s.db[317][22]));
        let eq96_e1378_d_b23: f64 = ((eq96_e1376_d_b23 * s.v[317]) + (eq96_e1376 * s.db[317][23]));
        let eq96_e1378_d_b24: f64 = ((eq96_e1376_d_b24 * s.v[317]) + (eq96_e1376 * s.db[317][24]));
        let eq96_e1378_d_b25: f64 = ((eq96_e1376_d_b25 * s.v[317]) + (eq96_e1376 * s.db[317][25]));
        let eq96_e1378_d_b26: f64 = ((eq96_e1376_d_b26 * s.v[317]) + (eq96_e1376 * s.db[317][26]));
        let eq96_e1378_d_b27: f64 = ((eq96_e1376_d_b27 * s.v[317]) + (eq96_e1376 * s.db[317][27]));
        let eq96_e1378_d_b28: f64 = ((eq96_e1376_d_b28 * s.v[317]) + (eq96_e1376 * s.db[317][28]));
        let eq96_e1378_d_b29: f64 = ((eq96_e1376_d_b29 * s.v[317]) + (eq96_e1376 * s.db[317][29]));
        let eq96_e1378_d_b30: f64 = ((eq96_e1376_d_b30 * s.v[317]) + (eq96_e1376 * s.db[317][30]));
        let eq96_e1378_d_b31: f64 = ((eq96_e1376_d_b31 * s.v[317]) + (eq96_e1376 * s.db[317][31]));
        let eq96_e1378_d_b32: f64 = ((eq96_e1376_d_b32 * s.v[317]) + (eq96_e1376 * s.db[317][32]));
        let eq96_e1378_d_b33: f64 = ((eq96_e1376_d_b33 * s.v[317]) + (eq96_e1376 * s.db[317][33]));
        let eq96_e1378_d_b34: f64 = ((eq96_e1376_d_b34 * s.v[317]) + (eq96_e1376 * s.db[317][34]));
        let eq96_e1378_d_b35: f64 = ((eq96_e1376_d_b35 * s.v[317]) + (eq96_e1376 * s.db[317][35]));
        let eq96_e1378_d_b36: f64 = ((eq96_e1376_d_b36 * s.v[317]) + (eq96_e1376 * s.db[317][36]));
        let eq96_e1378_d_b37: f64 = ((eq96_e1376_d_b37 * s.v[317]) + (eq96_e1376 * s.db[317][37]));
        let eq96_e1378_d_b38: f64 = ((eq96_e1376_d_b38 * s.v[317]) + (eq96_e1376 * s.db[317][38]));
        let eq96_e1378_d_b39: f64 = ((eq96_e1376_d_b39 * s.v[317]) + (eq96_e1376 * s.db[317][39]));
        let eq96_e1378_d_b40: f64 = ((eq96_e1376_d_b40 * s.v[317]) + (eq96_e1376 * s.db[317][40]));
        let eq96_e1378_d_b41: f64 = ((eq96_e1376_d_b41 * s.v[317]) + (eq96_e1376 * s.db[317][41]));
        let eq96_e1378_d_b42: f64 = ((eq96_e1376_d_b42 * s.v[317]) + (eq96_e1376 * s.db[317][42]));
        let eq96_e1378_d_b43: f64 = ((eq96_e1376_d_b43 * s.v[317]) + (eq96_e1376 * s.db[317][43]));
        let eq96_e1378_d_b44: f64 = ((eq96_e1376_d_b44 * s.v[317]) + (eq96_e1376 * s.db[317][44]));
        let eq96_e1378_d_b45: f64 = ((eq96_e1376_d_b45 * s.v[317]) + (eq96_e1376 * s.db[317][45]));
        let eq96_e1378_d_b46: f64 = ((eq96_e1376_d_b46 * s.v[317]) + (eq96_e1376 * s.db[317][46]));
        let eq96_e1378_d_b47: f64 = ((eq96_e1376_d_b47 * s.v[317]) + (eq96_e1376 * s.db[317][47]));
        let eq96_e1378_d_b48: f64 = ((eq96_e1376_d_b48 * s.v[317]) + (eq96_e1376 * s.db[317][48]));
        let eq96_e1378_d_b49: f64 = ((eq96_e1376_d_b49 * s.v[317]) + (eq96_e1376 * s.db[317][49]));
        let eq96_e1378_d_b50: f64 = ((eq96_e1376_d_b50 * s.v[317]) + (eq96_e1376 * s.db[317][50]));
        let eq96_e1378_d_b51: f64 = ((eq96_e1376_d_b51 * s.v[317]) + (eq96_e1376 * s.db[317][51]));
        let eq96_e1378_d_b52: f64 = ((eq96_e1376_d_b52 * s.v[317]) + (eq96_e1376 * s.db[317][52]));
        let eq96_e1378_d_b53: f64 = ((eq96_e1376_d_b53 * s.v[317]) + (eq96_e1376 * s.db[317][53]));
        let eq96_e1378_d_b54: f64 = ((eq96_e1376_d_b54 * s.v[317]) + (eq96_e1376 * s.db[317][54]));
        let eq96_e1381: f64 = (p.p6 * s.v[379]);
        let eq96_e1381_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq96_e1381_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq96_e1381_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq96_e1381_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq96_e1381_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq96_e1381_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq96_e1381_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq96_e1381_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq96_e1381_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq96_e1381_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq96_e1381_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq96_e1381_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq96_e1381_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq96_e1381_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq96_e1381_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq96_e1381_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq96_e1381_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq96_e1381_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq96_e1381_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq96_e1381_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq96_e1381_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq96_e1381_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq96_e1381_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq96_e1381_d_b0: f64 = (p.p6 * s.db[379][0]);
        let eq96_e1381_d_b1: f64 = (p.p6 * s.db[379][1]);
        let eq96_e1381_d_b2: f64 = (p.p6 * s.db[379][2]);
        let eq96_e1381_d_b3: f64 = (p.p6 * s.db[379][3]);
        let eq96_e1381_d_b4: f64 = (p.p6 * s.db[379][4]);
        let eq96_e1381_d_b5: f64 = (p.p6 * s.db[379][5]);
        let eq96_e1381_d_b6: f64 = (p.p6 * s.db[379][6]);
        let eq96_e1381_d_b7: f64 = (p.p6 * s.db[379][7]);
        let eq96_e1381_d_b8: f64 = (p.p6 * s.db[379][8]);
        let eq96_e1381_d_b9: f64 = (p.p6 * s.db[379][9]);
        let eq96_e1381_d_b10: f64 = (p.p6 * s.db[379][10]);
        let eq96_e1381_d_b11: f64 = (p.p6 * s.db[379][11]);
        let eq96_e1381_d_b12: f64 = (p.p6 * s.db[379][12]);
        let eq96_e1381_d_b13: f64 = (p.p6 * s.db[379][13]);
        let eq96_e1381_d_b14: f64 = (p.p6 * s.db[379][14]);
        let eq96_e1381_d_b15: f64 = (p.p6 * s.db[379][15]);
        let eq96_e1381_d_b16: f64 = (p.p6 * s.db[379][16]);
        let eq96_e1381_d_b17: f64 = (p.p6 * s.db[379][17]);
        let eq96_e1381_d_b18: f64 = (p.p6 * s.db[379][18]);
        let eq96_e1381_d_b19: f64 = (p.p6 * s.db[379][19]);
        let eq96_e1381_d_b20: f64 = (p.p6 * s.db[379][20]);
        let eq96_e1381_d_b21: f64 = (p.p6 * s.db[379][21]);
        let eq96_e1381_d_b22: f64 = (p.p6 * s.db[379][22]);
        let eq96_e1381_d_b23: f64 = (p.p6 * s.db[379][23]);
        let eq96_e1381_d_b24: f64 = (p.p6 * s.db[379][24]);
        let eq96_e1381_d_b25: f64 = (p.p6 * s.db[379][25]);
        let eq96_e1381_d_b26: f64 = (p.p6 * s.db[379][26]);
        let eq96_e1381_d_b27: f64 = (p.p6 * s.db[379][27]);
        let eq96_e1381_d_b28: f64 = (p.p6 * s.db[379][28]);
        let eq96_e1381_d_b29: f64 = (p.p6 * s.db[379][29]);
        let eq96_e1381_d_b30: f64 = (p.p6 * s.db[379][30]);
        let eq96_e1381_d_b31: f64 = (p.p6 * s.db[379][31]);
        let eq96_e1381_d_b32: f64 = (p.p6 * s.db[379][32]);
        let eq96_e1381_d_b33: f64 = (p.p6 * s.db[379][33]);
        let eq96_e1381_d_b34: f64 = (p.p6 * s.db[379][34]);
        let eq96_e1381_d_b35: f64 = (p.p6 * s.db[379][35]);
        let eq96_e1381_d_b36: f64 = (p.p6 * s.db[379][36]);
        let eq96_e1381_d_b37: f64 = (p.p6 * s.db[379][37]);
        let eq96_e1381_d_b38: f64 = (p.p6 * s.db[379][38]);
        let eq96_e1381_d_b39: f64 = (p.p6 * s.db[379][39]);
        let eq96_e1381_d_b40: f64 = (p.p6 * s.db[379][40]);
        let eq96_e1381_d_b41: f64 = (p.p6 * s.db[379][41]);
        let eq96_e1381_d_b42: f64 = (p.p6 * s.db[379][42]);
        let eq96_e1381_d_b43: f64 = (p.p6 * s.db[379][43]);
        let eq96_e1381_d_b44: f64 = (p.p6 * s.db[379][44]);
        let eq96_e1381_d_b45: f64 = (p.p6 * s.db[379][45]);
        let eq96_e1381_d_b46: f64 = (p.p6 * s.db[379][46]);
        let eq96_e1381_d_b47: f64 = (p.p6 * s.db[379][47]);
        let eq96_e1381_d_b48: f64 = (p.p6 * s.db[379][48]);
        let eq96_e1381_d_b49: f64 = (p.p6 * s.db[379][49]);
        let eq96_e1381_d_b50: f64 = (p.p6 * s.db[379][50]);
        let eq96_e1381_d_b51: f64 = (p.p6 * s.db[379][51]);
        let eq96_e1381_d_b52: f64 = (p.p6 * s.db[379][52]);
        let eq96_e1381_d_b53: f64 = (p.p6 * s.db[379][53]);
        let eq96_e1381_d_b54: f64 = (p.p6 * s.db[379][54]);
        let eq96_e1383: f64 = (eq96_e1381 * (nv21 - nv22));
        let eq96_e1383_d_n0: f64 = (eq96_e1381_d_n0 * (nv21 - nv22));
        let eq96_e1383_d_n1: f64 = (eq96_e1381_d_n1 * (nv21 - nv22));
        let eq96_e1383_d_n2: f64 = (eq96_e1381_d_n2 * (nv21 - nv22));
        let eq96_e1383_d_n3: f64 = (eq96_e1381_d_n3 * (nv21 - nv22));
        let eq96_e1383_d_n4: f64 = (eq96_e1381_d_n4 * (nv21 - nv22));
        let eq96_e1383_d_n5: f64 = (eq96_e1381_d_n5 * (nv21 - nv22));
        let eq96_e1383_d_n6: f64 = (eq96_e1381_d_n6 * (nv21 - nv22));
        let eq96_e1383_d_n7: f64 = (eq96_e1381_d_n7 * (nv21 - nv22));
        let eq96_e1383_d_n8: f64 = (eq96_e1381_d_n8 * (nv21 - nv22));
        let eq96_e1383_d_n9: f64 = (eq96_e1381_d_n9 * (nv21 - nv22));
        let eq96_e1383_d_n10: f64 = (eq96_e1381_d_n10 * (nv21 - nv22));
        let eq96_e1383_d_n11: f64 = (eq96_e1381_d_n11 * (nv21 - nv22));
        let eq96_e1383_d_n12: f64 = (eq96_e1381_d_n12 * (nv21 - nv22));
        let eq96_e1383_d_n13: f64 = (eq96_e1381_d_n13 * (nv21 - nv22));
        let eq96_e1383_d_n14: f64 = (eq96_e1381_d_n14 * (nv21 - nv22));
        let eq96_e1383_d_n15: f64 = (eq96_e1381_d_n15 * (nv21 - nv22));
        let eq96_e1383_d_n16: f64 = (eq96_e1381_d_n16 * (nv21 - nv22));
        let eq96_e1383_d_n17: f64 = (eq96_e1381_d_n17 * (nv21 - nv22));
        let eq96_e1383_d_n18: f64 = (eq96_e1381_d_n18 * (nv21 - nv22));
        let eq96_e1383_d_n19: f64 = (eq96_e1381_d_n19 * (nv21 - nv22));
        let eq96_e1383_d_n20: f64 = (eq96_e1381_d_n20 * (nv21 - nv22));
        let eq96_e1383_d_n21: f64 = ((eq96_e1381_d_n21 * (nv21 - nv22)) + eq96_e1381);
        let eq96_e1383_d_n22: f64 = ((eq96_e1381_d_n22 * (nv21 - nv22)) + (-eq96_e1381));
        let eq96_e1383_d_b0: f64 = (eq96_e1381_d_b0 * (nv21 - nv22));
        let eq96_e1383_d_b1: f64 = (eq96_e1381_d_b1 * (nv21 - nv22));
        let eq96_e1383_d_b2: f64 = (eq96_e1381_d_b2 * (nv21 - nv22));
        let eq96_e1383_d_b3: f64 = (eq96_e1381_d_b3 * (nv21 - nv22));
        let eq96_e1383_d_b4: f64 = (eq96_e1381_d_b4 * (nv21 - nv22));
        let eq96_e1383_d_b5: f64 = (eq96_e1381_d_b5 * (nv21 - nv22));
        let eq96_e1383_d_b6: f64 = (eq96_e1381_d_b6 * (nv21 - nv22));
        let eq96_e1383_d_b7: f64 = (eq96_e1381_d_b7 * (nv21 - nv22));
        let eq96_e1383_d_b8: f64 = (eq96_e1381_d_b8 * (nv21 - nv22));
        let eq96_e1383_d_b9: f64 = (eq96_e1381_d_b9 * (nv21 - nv22));
        let eq96_e1383_d_b10: f64 = (eq96_e1381_d_b10 * (nv21 - nv22));
        let eq96_e1383_d_b11: f64 = (eq96_e1381_d_b11 * (nv21 - nv22));
        let eq96_e1383_d_b12: f64 = (eq96_e1381_d_b12 * (nv21 - nv22));
        let eq96_e1383_d_b13: f64 = (eq96_e1381_d_b13 * (nv21 - nv22));
        let eq96_e1383_d_b14: f64 = (eq96_e1381_d_b14 * (nv21 - nv22));
        let eq96_e1383_d_b15: f64 = (eq96_e1381_d_b15 * (nv21 - nv22));
        let eq96_e1383_d_b16: f64 = (eq96_e1381_d_b16 * (nv21 - nv22));
        let eq96_e1383_d_b17: f64 = (eq96_e1381_d_b17 * (nv21 - nv22));
        let eq96_e1383_d_b18: f64 = (eq96_e1381_d_b18 * (nv21 - nv22));
        let eq96_e1383_d_b19: f64 = (eq96_e1381_d_b19 * (nv21 - nv22));
        let eq96_e1383_d_b20: f64 = (eq96_e1381_d_b20 * (nv21 - nv22));
        let eq96_e1383_d_b21: f64 = (eq96_e1381_d_b21 * (nv21 - nv22));
        let eq96_e1383_d_b22: f64 = (eq96_e1381_d_b22 * (nv21 - nv22));
        let eq96_e1383_d_b23: f64 = (eq96_e1381_d_b23 * (nv21 - nv22));
        let eq96_e1383_d_b24: f64 = (eq96_e1381_d_b24 * (nv21 - nv22));
        let eq96_e1383_d_b25: f64 = (eq96_e1381_d_b25 * (nv21 - nv22));
        let eq96_e1383_d_b26: f64 = (eq96_e1381_d_b26 * (nv21 - nv22));
        let eq96_e1383_d_b27: f64 = (eq96_e1381_d_b27 * (nv21 - nv22));
        let eq96_e1383_d_b28: f64 = (eq96_e1381_d_b28 * (nv21 - nv22));
        let eq96_e1383_d_b29: f64 = (eq96_e1381_d_b29 * (nv21 - nv22));
        let eq96_e1383_d_b30: f64 = (eq96_e1381_d_b30 * (nv21 - nv22));
        let eq96_e1383_d_b31: f64 = (eq96_e1381_d_b31 * (nv21 - nv22));
        let eq96_e1383_d_b32: f64 = (eq96_e1381_d_b32 * (nv21 - nv22));
        let eq96_e1383_d_b33: f64 = (eq96_e1381_d_b33 * (nv21 - nv22));
        let eq96_e1383_d_b34: f64 = (eq96_e1381_d_b34 * (nv21 - nv22));
        let eq96_e1383_d_b35: f64 = (eq96_e1381_d_b35 * (nv21 - nv22));
        let eq96_e1383_d_b36: f64 = (eq96_e1381_d_b36 * (nv21 - nv22));
        let eq96_e1383_d_b37: f64 = (eq96_e1381_d_b37 * (nv21 - nv22));
        let eq96_e1383_d_b38: f64 = (eq96_e1381_d_b38 * (nv21 - nv22));
        let eq96_e1383_d_b39: f64 = (eq96_e1381_d_b39 * (nv21 - nv22));
        let eq96_e1383_d_b40: f64 = (eq96_e1381_d_b40 * (nv21 - nv22));
        let eq96_e1383_d_b41: f64 = (eq96_e1381_d_b41 * (nv21 - nv22));
        let eq96_e1383_d_b42: f64 = (eq96_e1381_d_b42 * (nv21 - nv22));
        let eq96_e1383_d_b43: f64 = (eq96_e1381_d_b43 * (nv21 - nv22));
        let eq96_e1383_d_b44: f64 = (eq96_e1381_d_b44 * (nv21 - nv22));
        let eq96_e1383_d_b45: f64 = (eq96_e1381_d_b45 * (nv21 - nv22));
        let eq96_e1383_d_b46: f64 = (eq96_e1381_d_b46 * (nv21 - nv22));
        let eq96_e1383_d_b47: f64 = (eq96_e1381_d_b47 * (nv21 - nv22));
        let eq96_e1383_d_b48: f64 = (eq96_e1381_d_b48 * (nv21 - nv22));
        let eq96_e1383_d_b49: f64 = (eq96_e1381_d_b49 * (nv21 - nv22));
        let eq96_e1383_d_b50: f64 = (eq96_e1381_d_b50 * (nv21 - nv22));
        let eq96_e1383_d_b51: f64 = (eq96_e1381_d_b51 * (nv21 - nv22));
        let eq96_e1383_d_b52: f64 = (eq96_e1381_d_b52 * (nv21 - nv22));
        let eq96_e1383_d_b53: f64 = (eq96_e1381_d_b53 * (nv21 - nv22));
        let eq96_e1383_d_b54: f64 = (eq96_e1381_d_b54 * (nv21 - nv22));
        let eq96_e1384: f64 = (eq96_e1378 + eq96_e1383);
        let eq96_e1384_d_n0: f64 = (eq96_e1378_d_n0 + eq96_e1383_d_n0);
        let eq96_e1384_d_n1: f64 = (eq96_e1378_d_n1 + eq96_e1383_d_n1);
        let eq96_e1384_d_n2: f64 = (eq96_e1378_d_n2 + eq96_e1383_d_n2);
        let eq96_e1384_d_n3: f64 = (eq96_e1378_d_n3 + eq96_e1383_d_n3);
        let eq96_e1384_d_n4: f64 = (eq96_e1378_d_n4 + eq96_e1383_d_n4);
        let eq96_e1384_d_n5: f64 = (eq96_e1378_d_n5 + eq96_e1383_d_n5);
        let eq96_e1384_d_n6: f64 = (eq96_e1378_d_n6 + eq96_e1383_d_n6);
        let eq96_e1384_d_n7: f64 = (eq96_e1378_d_n7 + eq96_e1383_d_n7);
        let eq96_e1384_d_n8: f64 = (eq96_e1378_d_n8 + eq96_e1383_d_n8);
        let eq96_e1384_d_n9: f64 = (eq96_e1378_d_n9 + eq96_e1383_d_n9);
        let eq96_e1384_d_n10: f64 = (eq96_e1378_d_n10 + eq96_e1383_d_n10);
        let eq96_e1384_d_n11: f64 = (eq96_e1378_d_n11 + eq96_e1383_d_n11);
        let eq96_e1384_d_n12: f64 = (eq96_e1378_d_n12 + eq96_e1383_d_n12);
        let eq96_e1384_d_n13: f64 = (eq96_e1378_d_n13 + eq96_e1383_d_n13);
        let eq96_e1384_d_n14: f64 = (eq96_e1378_d_n14 + eq96_e1383_d_n14);
        let eq96_e1384_d_n15: f64 = (eq96_e1378_d_n15 + eq96_e1383_d_n15);
        let eq96_e1384_d_n16: f64 = (eq96_e1378_d_n16 + eq96_e1383_d_n16);
        let eq96_e1384_d_n17: f64 = (eq96_e1378_d_n17 + eq96_e1383_d_n17);
        let eq96_e1384_d_n18: f64 = (eq96_e1378_d_n18 + eq96_e1383_d_n18);
        let eq96_e1384_d_n19: f64 = (eq96_e1378_d_n19 + eq96_e1383_d_n19);
        let eq96_e1384_d_n20: f64 = (eq96_e1378_d_n20 + eq96_e1383_d_n20);
        let eq96_e1384_d_n21: f64 = (eq96_e1378_d_n21 + eq96_e1383_d_n21);
        let eq96_e1384_d_n22: f64 = (eq96_e1378_d_n22 + eq96_e1383_d_n22);
        let eq96_e1384_d_b0: f64 = (eq96_e1378_d_b0 + eq96_e1383_d_b0);
        let eq96_e1384_d_b1: f64 = (eq96_e1378_d_b1 + eq96_e1383_d_b1);
        let eq96_e1384_d_b2: f64 = (eq96_e1378_d_b2 + eq96_e1383_d_b2);
        let eq96_e1384_d_b3: f64 = (eq96_e1378_d_b3 + eq96_e1383_d_b3);
        let eq96_e1384_d_b4: f64 = (eq96_e1378_d_b4 + eq96_e1383_d_b4);
        let eq96_e1384_d_b5: f64 = (eq96_e1378_d_b5 + eq96_e1383_d_b5);
        let eq96_e1384_d_b6: f64 = (eq96_e1378_d_b6 + eq96_e1383_d_b6);
        let eq96_e1384_d_b7: f64 = (eq96_e1378_d_b7 + eq96_e1383_d_b7);
        let eq96_e1384_d_b8: f64 = (eq96_e1378_d_b8 + eq96_e1383_d_b8);
        let eq96_e1384_d_b9: f64 = (eq96_e1378_d_b9 + eq96_e1383_d_b9);
        let eq96_e1384_d_b10: f64 = (eq96_e1378_d_b10 + eq96_e1383_d_b10);
        let eq96_e1384_d_b11: f64 = (eq96_e1378_d_b11 + eq96_e1383_d_b11);
        let eq96_e1384_d_b12: f64 = (eq96_e1378_d_b12 + eq96_e1383_d_b12);
        let eq96_e1384_d_b13: f64 = (eq96_e1378_d_b13 + eq96_e1383_d_b13);
        let eq96_e1384_d_b14: f64 = (eq96_e1378_d_b14 + eq96_e1383_d_b14);
        let eq96_e1384_d_b15: f64 = (eq96_e1378_d_b15 + eq96_e1383_d_b15);
        let eq96_e1384_d_b16: f64 = (eq96_e1378_d_b16 + eq96_e1383_d_b16);
        let eq96_e1384_d_b17: f64 = (eq96_e1378_d_b17 + eq96_e1383_d_b17);
        let eq96_e1384_d_b18: f64 = (eq96_e1378_d_b18 + eq96_e1383_d_b18);
        let eq96_e1384_d_b19: f64 = (eq96_e1378_d_b19 + eq96_e1383_d_b19);
        let eq96_e1384_d_b20: f64 = (eq96_e1378_d_b20 + eq96_e1383_d_b20);
        let eq96_e1384_d_b21: f64 = (eq96_e1378_d_b21 + eq96_e1383_d_b21);
        let eq96_e1384_d_b22: f64 = (eq96_e1378_d_b22 + eq96_e1383_d_b22);
        let eq96_e1384_d_b23: f64 = (eq96_e1378_d_b23 + eq96_e1383_d_b23);
        let eq96_e1384_d_b24: f64 = (eq96_e1378_d_b24 + eq96_e1383_d_b24);
        let eq96_e1384_d_b25: f64 = (eq96_e1378_d_b25 + eq96_e1383_d_b25);
        let eq96_e1384_d_b26: f64 = (eq96_e1378_d_b26 + eq96_e1383_d_b26);
        let eq96_e1384_d_b27: f64 = (eq96_e1378_d_b27 + eq96_e1383_d_b27);
        let eq96_e1384_d_b28: f64 = (eq96_e1378_d_b28 + eq96_e1383_d_b28);
        let eq96_e1384_d_b29: f64 = (eq96_e1378_d_b29 + eq96_e1383_d_b29);
        let eq96_e1384_d_b30: f64 = (eq96_e1378_d_b30 + eq96_e1383_d_b30);
        let eq96_e1384_d_b31: f64 = (eq96_e1378_d_b31 + eq96_e1383_d_b31);
        let eq96_e1384_d_b32: f64 = (eq96_e1378_d_b32 + eq96_e1383_d_b32);
        let eq96_e1384_d_b33: f64 = (eq96_e1378_d_b33 + eq96_e1383_d_b33);
        let eq96_e1384_d_b34: f64 = (eq96_e1378_d_b34 + eq96_e1383_d_b34);
        let eq96_e1384_d_b35: f64 = (eq96_e1378_d_b35 + eq96_e1383_d_b35);
        let eq96_e1384_d_b36: f64 = (eq96_e1378_d_b36 + eq96_e1383_d_b36);
        let eq96_e1384_d_b37: f64 = (eq96_e1378_d_b37 + eq96_e1383_d_b37);
        let eq96_e1384_d_b38: f64 = (eq96_e1378_d_b38 + eq96_e1383_d_b38);
        let eq96_e1384_d_b39: f64 = (eq96_e1378_d_b39 + eq96_e1383_d_b39);
        let eq96_e1384_d_b40: f64 = (eq96_e1378_d_b40 + eq96_e1383_d_b40);
        let eq96_e1384_d_b41: f64 = (eq96_e1378_d_b41 + eq96_e1383_d_b41);
        let eq96_e1384_d_b42: f64 = (eq96_e1378_d_b42 + eq96_e1383_d_b42);
        let eq96_e1384_d_b43: f64 = (eq96_e1378_d_b43 + eq96_e1383_d_b43);
        let eq96_e1384_d_b44: f64 = (eq96_e1378_d_b44 + eq96_e1383_d_b44);
        let eq96_e1384_d_b45: f64 = (eq96_e1378_d_b45 + eq96_e1383_d_b45);
        let eq96_e1384_d_b46: f64 = (eq96_e1378_d_b46 + eq96_e1383_d_b46);
        let eq96_e1384_d_b47: f64 = (eq96_e1378_d_b47 + eq96_e1383_d_b47);
        let eq96_e1384_d_b48: f64 = (eq96_e1378_d_b48 + eq96_e1383_d_b48);
        let eq96_e1384_d_b49: f64 = (eq96_e1378_d_b49 + eq96_e1383_d_b49);
        let eq96_e1384_d_b50: f64 = (eq96_e1378_d_b50 + eq96_e1383_d_b50);
        let eq96_e1384_d_b51: f64 = (eq96_e1378_d_b51 + eq96_e1383_d_b51);
        let eq96_e1384_d_b52: f64 = (eq96_e1378_d_b52 + eq96_e1383_d_b52);
        let eq96_e1384_d_b53: f64 = (eq96_e1378_d_b53 + eq96_e1383_d_b53);
        let eq96_e1384_d_b54: f64 = (eq96_e1378_d_b54 + eq96_e1383_d_b54);
        (eq96_e1384, eq96_e1384_d_n0, eq96_e1384_d_n1, eq96_e1384_d_n2, eq96_e1384_d_n3, eq96_e1384_d_n4, eq96_e1384_d_n5, eq96_e1384_d_n6, eq96_e1384_d_n7, eq96_e1384_d_n8, eq96_e1384_d_n9, eq96_e1384_d_n10, eq96_e1384_d_n11, eq96_e1384_d_n12, eq96_e1384_d_n13, eq96_e1384_d_n14, eq96_e1384_d_n15, eq96_e1384_d_n16, eq96_e1384_d_n17, eq96_e1384_d_n18, eq96_e1384_d_n19, eq96_e1384_d_n20, eq96_e1384_d_n21, eq96_e1384_d_n22, eq96_e1384_d_b0, eq96_e1384_d_b1, eq96_e1384_d_b2, eq96_e1384_d_b3, eq96_e1384_d_b4, eq96_e1384_d_b5, eq96_e1384_d_b6, eq96_e1384_d_b7, eq96_e1384_d_b8, eq96_e1384_d_b9, eq96_e1384_d_b10, eq96_e1384_d_b11, eq96_e1384_d_b12, eq96_e1384_d_b13, eq96_e1384_d_b14, eq96_e1384_d_b15, eq96_e1384_d_b16, eq96_e1384_d_b17, eq96_e1384_d_b18, eq96_e1384_d_b19, eq96_e1384_d_b20, eq96_e1384_d_b21, eq96_e1384_d_b22, eq96_e1384_d_b23, eq96_e1384_d_b24, eq96_e1384_d_b25, eq96_e1384_d_b26, eq96_e1384_d_b27, eq96_e1384_d_b28, eq96_e1384_d_b29, eq96_e1384_d_b30, eq96_e1384_d_b31, eq96_e1384_d_b32, eq96_e1384_d_b33, eq96_e1384_d_b34, eq96_e1384_d_b35, eq96_e1384_d_b36, eq96_e1384_d_b37, eq96_e1384_d_b38, eq96_e1384_d_b39, eq96_e1384_d_b40, eq96_e1384_d_b41, eq96_e1384_d_b42, eq96_e1384_d_b43, eq96_e1384_d_b44, eq96_e1384_d_b45, eq96_e1384_d_b46, eq96_e1384_d_b47, eq96_e1384_d_b48, eq96_e1384_d_b49, eq96_e1384_d_b50, eq96_e1384_d_b51, eq96_e1384_d_b52, eq96_e1384_d_b53, eq96_e1384_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e1386;
        let eq96_node_derivatives: [f64; 23] = [eq96_e1386_d_n0, eq96_e1386_d_n1, eq96_e1386_d_n2, eq96_e1386_d_n3, eq96_e1386_d_n4, eq96_e1386_d_n5, eq96_e1386_d_n6, eq96_e1386_d_n7, eq96_e1386_d_n8, eq96_e1386_d_n9, eq96_e1386_d_n10, eq96_e1386_d_n11, eq96_e1386_d_n12, eq96_e1386_d_n13, eq96_e1386_d_n14, eq96_e1386_d_n15, eq96_e1386_d_n16, eq96_e1386_d_n17, eq96_e1386_d_n18, eq96_e1386_d_n19, eq96_e1386_d_n20, eq96_e1386_d_n21, eq96_e1386_d_n22];
        let eq96_branch_derivatives: [f64; 55] = [eq96_e1386_d_b0, eq96_e1386_d_b1, eq96_e1386_d_b2, eq96_e1386_d_b3, eq96_e1386_d_b4, eq96_e1386_d_b5, eq96_e1386_d_b6, eq96_e1386_d_b7, eq96_e1386_d_b8, eq96_e1386_d_b9, eq96_e1386_d_b10, eq96_e1386_d_b11, eq96_e1386_d_b12, eq96_e1386_d_b13, eq96_e1386_d_b14, eq96_e1386_d_b15, eq96_e1386_d_b16, eq96_e1386_d_b17, eq96_e1386_d_b18, eq96_e1386_d_b19, eq96_e1386_d_b20, eq96_e1386_d_b21, eq96_e1386_d_b22, eq96_e1386_d_b23, eq96_e1386_d_b24, eq96_e1386_d_b25, eq96_e1386_d_b26, eq96_e1386_d_b27, eq96_e1386_d_b28, eq96_e1386_d_b29, eq96_e1386_d_b30, eq96_e1386_d_b31, eq96_e1386_d_b32, eq96_e1386_d_b33, eq96_e1386_d_b34, eq96_e1386_d_b35, eq96_e1386_d_b36, eq96_e1386_d_b37, eq96_e1386_d_b38, eq96_e1386_d_b39, eq96_e1386_d_b40, eq96_e1386_d_b41, eq96_e1386_d_b42, eq96_e1386_d_b43, eq96_e1386_d_b44, eq96_e1386_d_b45, eq96_e1386_d_b46, eq96_e1386_d_b47, eq96_e1386_d_b48, eq96_e1386_d_b49, eq96_e1386_d_b50, eq96_e1386_d_b51, eq96_e1386_d_b52, eq96_e1386_d_b53, eq96_e1386_d_b54];
        stamper.stamp_current_dense_local(
            Some(21),
            Some(22),
            multiplicity * (eq96_value),
            &eq96_node_derivatives,
            &eq96_branch_derivatives,
            multiplicity,
        );
        let (eq97_e1393,) = {
    if (s.b[538] && (!s.b[539])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq97_value: f64 = eq97_e1393;
        stamper.stamp_potential_const_local(
            50,
            eq97_value,
        );
        let (eq98_e1398,) = {
    if (!s.b[538]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq98_value: f64 = eq98_e1398;
        stamper.stamp_potential_const_local(
            51,
            eq98_value,
        );
        let (eq99_e1406,) = {
    if ((!s.b[538]) && (!s.b[547])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq99_value: f64 = eq99_e1406;
        stamper.stamp_potential_const_local(
            52,
            eq99_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_20(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq100_e1414, eq100_e1414_d_n0, eq100_e1414_d_n1, eq100_e1414_d_n2, eq100_e1414_d_n3, eq100_e1414_d_n4, eq100_e1414_d_n5, eq100_e1414_d_n6, eq100_e1414_d_n7, eq100_e1414_d_n8, eq100_e1414_d_n9, eq100_e1414_d_n10, eq100_e1414_d_n11, eq100_e1414_d_n12, eq100_e1414_d_n13, eq100_e1414_d_n14, eq100_e1414_d_n15, eq100_e1414_d_n16, eq100_e1414_d_n17, eq100_e1414_d_n18, eq100_e1414_d_n19, eq100_e1414_d_n20, eq100_e1414_d_n21, eq100_e1414_d_n22, eq100_e1414_d_b0, eq100_e1414_d_b1, eq100_e1414_d_b2, eq100_e1414_d_b3, eq100_e1414_d_b4, eq100_e1414_d_b5, eq100_e1414_d_b6, eq100_e1414_d_b7, eq100_e1414_d_b8, eq100_e1414_d_b9, eq100_e1414_d_b10, eq100_e1414_d_b11, eq100_e1414_d_b12, eq100_e1414_d_b13, eq100_e1414_d_b14, eq100_e1414_d_b15, eq100_e1414_d_b16, eq100_e1414_d_b17, eq100_e1414_d_b18, eq100_e1414_d_b19, eq100_e1414_d_b20, eq100_e1414_d_b21, eq100_e1414_d_b22, eq100_e1414_d_b23, eq100_e1414_d_b24, eq100_e1414_d_b25, eq100_e1414_d_b26, eq100_e1414_d_b27, eq100_e1414_d_b28, eq100_e1414_d_b29, eq100_e1414_d_b30, eq100_e1414_d_b31, eq100_e1414_d_b32, eq100_e1414_d_b33, eq100_e1414_d_b34, eq100_e1414_d_b35, eq100_e1414_d_b36, eq100_e1414_d_b37, eq100_e1414_d_b38, eq100_e1414_d_b39, eq100_e1414_d_b40, eq100_e1414_d_b41, eq100_e1414_d_b42, eq100_e1414_d_b43, eq100_e1414_d_b44, eq100_e1414_d_b45, eq100_e1414_d_b46, eq100_e1414_d_b47, eq100_e1414_d_b48, eq100_e1414_d_b49, eq100_e1414_d_b50, eq100_e1414_d_b51, eq100_e1414_d_b52, eq100_e1414_d_b53, eq100_e1414_d_b54,) = {
    if s.b[553] {
        let eq100_e1410: f64 = (p.p6 * s.v[318]);
        let eq100_e1410_d_n0: f64 = (p.p6 * s.dn[318][0]);
        let eq100_e1410_d_n1: f64 = (p.p6 * s.dn[318][1]);
        let eq100_e1410_d_n2: f64 = (p.p6 * s.dn[318][2]);
        let eq100_e1410_d_n3: f64 = (p.p6 * s.dn[318][3]);
        let eq100_e1410_d_n4: f64 = (p.p6 * s.dn[318][4]);
        let eq100_e1410_d_n5: f64 = (p.p6 * s.dn[318][5]);
        let eq100_e1410_d_n6: f64 = (p.p6 * s.dn[318][6]);
        let eq100_e1410_d_n7: f64 = (p.p6 * s.dn[318][7]);
        let eq100_e1410_d_n8: f64 = (p.p6 * s.dn[318][8]);
        let eq100_e1410_d_n9: f64 = (p.p6 * s.dn[318][9]);
        let eq100_e1410_d_n10: f64 = (p.p6 * s.dn[318][10]);
        let eq100_e1410_d_n11: f64 = (p.p6 * s.dn[318][11]);
        let eq100_e1410_d_n12: f64 = (p.p6 * s.dn[318][12]);
        let eq100_e1410_d_n13: f64 = (p.p6 * s.dn[318][13]);
        let eq100_e1410_d_n14: f64 = (p.p6 * s.dn[318][14]);
        let eq100_e1410_d_n15: f64 = (p.p6 * s.dn[318][15]);
        let eq100_e1410_d_n16: f64 = (p.p6 * s.dn[318][16]);
        let eq100_e1410_d_n17: f64 = (p.p6 * s.dn[318][17]);
        let eq100_e1410_d_n18: f64 = (p.p6 * s.dn[318][18]);
        let eq100_e1410_d_n19: f64 = (p.p6 * s.dn[318][19]);
        let eq100_e1410_d_n20: f64 = (p.p6 * s.dn[318][20]);
        let eq100_e1410_d_n21: f64 = (p.p6 * s.dn[318][21]);
        let eq100_e1410_d_n22: f64 = (p.p6 * s.dn[318][22]);
        let eq100_e1410_d_b0: f64 = (p.p6 * s.db[318][0]);
        let eq100_e1410_d_b1: f64 = (p.p6 * s.db[318][1]);
        let eq100_e1410_d_b2: f64 = (p.p6 * s.db[318][2]);
        let eq100_e1410_d_b3: f64 = (p.p6 * s.db[318][3]);
        let eq100_e1410_d_b4: f64 = (p.p6 * s.db[318][4]);
        let eq100_e1410_d_b5: f64 = (p.p6 * s.db[318][5]);
        let eq100_e1410_d_b6: f64 = (p.p6 * s.db[318][6]);
        let eq100_e1410_d_b7: f64 = (p.p6 * s.db[318][7]);
        let eq100_e1410_d_b8: f64 = (p.p6 * s.db[318][8]);
        let eq100_e1410_d_b9: f64 = (p.p6 * s.db[318][9]);
        let eq100_e1410_d_b10: f64 = (p.p6 * s.db[318][10]);
        let eq100_e1410_d_b11: f64 = (p.p6 * s.db[318][11]);
        let eq100_e1410_d_b12: f64 = (p.p6 * s.db[318][12]);
        let eq100_e1410_d_b13: f64 = (p.p6 * s.db[318][13]);
        let eq100_e1410_d_b14: f64 = (p.p6 * s.db[318][14]);
        let eq100_e1410_d_b15: f64 = (p.p6 * s.db[318][15]);
        let eq100_e1410_d_b16: f64 = (p.p6 * s.db[318][16]);
        let eq100_e1410_d_b17: f64 = (p.p6 * s.db[318][17]);
        let eq100_e1410_d_b18: f64 = (p.p6 * s.db[318][18]);
        let eq100_e1410_d_b19: f64 = (p.p6 * s.db[318][19]);
        let eq100_e1410_d_b20: f64 = (p.p6 * s.db[318][20]);
        let eq100_e1410_d_b21: f64 = (p.p6 * s.db[318][21]);
        let eq100_e1410_d_b22: f64 = (p.p6 * s.db[318][22]);
        let eq100_e1410_d_b23: f64 = (p.p6 * s.db[318][23]);
        let eq100_e1410_d_b24: f64 = (p.p6 * s.db[318][24]);
        let eq100_e1410_d_b25: f64 = (p.p6 * s.db[318][25]);
        let eq100_e1410_d_b26: f64 = (p.p6 * s.db[318][26]);
        let eq100_e1410_d_b27: f64 = (p.p6 * s.db[318][27]);
        let eq100_e1410_d_b28: f64 = (p.p6 * s.db[318][28]);
        let eq100_e1410_d_b29: f64 = (p.p6 * s.db[318][29]);
        let eq100_e1410_d_b30: f64 = (p.p6 * s.db[318][30]);
        let eq100_e1410_d_b31: f64 = (p.p6 * s.db[318][31]);
        let eq100_e1410_d_b32: f64 = (p.p6 * s.db[318][32]);
        let eq100_e1410_d_b33: f64 = (p.p6 * s.db[318][33]);
        let eq100_e1410_d_b34: f64 = (p.p6 * s.db[318][34]);
        let eq100_e1410_d_b35: f64 = (p.p6 * s.db[318][35]);
        let eq100_e1410_d_b36: f64 = (p.p6 * s.db[318][36]);
        let eq100_e1410_d_b37: f64 = (p.p6 * s.db[318][37]);
        let eq100_e1410_d_b38: f64 = (p.p6 * s.db[318][38]);
        let eq100_e1410_d_b39: f64 = (p.p6 * s.db[318][39]);
        let eq100_e1410_d_b40: f64 = (p.p6 * s.db[318][40]);
        let eq100_e1410_d_b41: f64 = (p.p6 * s.db[318][41]);
        let eq100_e1410_d_b42: f64 = (p.p6 * s.db[318][42]);
        let eq100_e1410_d_b43: f64 = (p.p6 * s.db[318][43]);
        let eq100_e1410_d_b44: f64 = (p.p6 * s.db[318][44]);
        let eq100_e1410_d_b45: f64 = (p.p6 * s.db[318][45]);
        let eq100_e1410_d_b46: f64 = (p.p6 * s.db[318][46]);
        let eq100_e1410_d_b47: f64 = (p.p6 * s.db[318][47]);
        let eq100_e1410_d_b48: f64 = (p.p6 * s.db[318][48]);
        let eq100_e1410_d_b49: f64 = (p.p6 * s.db[318][49]);
        let eq100_e1410_d_b50: f64 = (p.p6 * s.db[318][50]);
        let eq100_e1410_d_b51: f64 = (p.p6 * s.db[318][51]);
        let eq100_e1410_d_b52: f64 = (p.p6 * s.db[318][52]);
        let eq100_e1410_d_b53: f64 = (p.p6 * s.db[318][53]);
        let eq100_e1410_d_b54: f64 = (p.p6 * s.db[318][54]);
        let eq100_e1412: f64 = (eq100_e1410 * (nv1 - nv9));
        let eq100_e1412_d_n0: f64 = (eq100_e1410_d_n0 * (nv1 - nv9));
        let eq100_e1412_d_n1: f64 = ((eq100_e1410_d_n1 * (nv1 - nv9)) + eq100_e1410);
        let eq100_e1412_d_n2: f64 = (eq100_e1410_d_n2 * (nv1 - nv9));
        let eq100_e1412_d_n3: f64 = (eq100_e1410_d_n3 * (nv1 - nv9));
        let eq100_e1412_d_n4: f64 = (eq100_e1410_d_n4 * (nv1 - nv9));
        let eq100_e1412_d_n5: f64 = (eq100_e1410_d_n5 * (nv1 - nv9));
        let eq100_e1412_d_n6: f64 = (eq100_e1410_d_n6 * (nv1 - nv9));
        let eq100_e1412_d_n7: f64 = (eq100_e1410_d_n7 * (nv1 - nv9));
        let eq100_e1412_d_n8: f64 = (eq100_e1410_d_n8 * (nv1 - nv9));
        let eq100_e1412_d_n9: f64 = ((eq100_e1410_d_n9 * (nv1 - nv9)) + (-eq100_e1410));
        let eq100_e1412_d_n10: f64 = (eq100_e1410_d_n10 * (nv1 - nv9));
        let eq100_e1412_d_n11: f64 = (eq100_e1410_d_n11 * (nv1 - nv9));
        let eq100_e1412_d_n12: f64 = (eq100_e1410_d_n12 * (nv1 - nv9));
        let eq100_e1412_d_n13: f64 = (eq100_e1410_d_n13 * (nv1 - nv9));
        let eq100_e1412_d_n14: f64 = (eq100_e1410_d_n14 * (nv1 - nv9));
        let eq100_e1412_d_n15: f64 = (eq100_e1410_d_n15 * (nv1 - nv9));
        let eq100_e1412_d_n16: f64 = (eq100_e1410_d_n16 * (nv1 - nv9));
        let eq100_e1412_d_n17: f64 = (eq100_e1410_d_n17 * (nv1 - nv9));
        let eq100_e1412_d_n18: f64 = (eq100_e1410_d_n18 * (nv1 - nv9));
        let eq100_e1412_d_n19: f64 = (eq100_e1410_d_n19 * (nv1 - nv9));
        let eq100_e1412_d_n20: f64 = (eq100_e1410_d_n20 * (nv1 - nv9));
        let eq100_e1412_d_n21: f64 = (eq100_e1410_d_n21 * (nv1 - nv9));
        let eq100_e1412_d_n22: f64 = (eq100_e1410_d_n22 * (nv1 - nv9));
        let eq100_e1412_d_b0: f64 = (eq100_e1410_d_b0 * (nv1 - nv9));
        let eq100_e1412_d_b1: f64 = (eq100_e1410_d_b1 * (nv1 - nv9));
        let eq100_e1412_d_b2: f64 = (eq100_e1410_d_b2 * (nv1 - nv9));
        let eq100_e1412_d_b3: f64 = (eq100_e1410_d_b3 * (nv1 - nv9));
        let eq100_e1412_d_b4: f64 = (eq100_e1410_d_b4 * (nv1 - nv9));
        let eq100_e1412_d_b5: f64 = (eq100_e1410_d_b5 * (nv1 - nv9));
        let eq100_e1412_d_b6: f64 = (eq100_e1410_d_b6 * (nv1 - nv9));
        let eq100_e1412_d_b7: f64 = (eq100_e1410_d_b7 * (nv1 - nv9));
        let eq100_e1412_d_b8: f64 = (eq100_e1410_d_b8 * (nv1 - nv9));
        let eq100_e1412_d_b9: f64 = (eq100_e1410_d_b9 * (nv1 - nv9));
        let eq100_e1412_d_b10: f64 = (eq100_e1410_d_b10 * (nv1 - nv9));
        let eq100_e1412_d_b11: f64 = (eq100_e1410_d_b11 * (nv1 - nv9));
        let eq100_e1412_d_b12: f64 = (eq100_e1410_d_b12 * (nv1 - nv9));
        let eq100_e1412_d_b13: f64 = (eq100_e1410_d_b13 * (nv1 - nv9));
        let eq100_e1412_d_b14: f64 = (eq100_e1410_d_b14 * (nv1 - nv9));
        let eq100_e1412_d_b15: f64 = (eq100_e1410_d_b15 * (nv1 - nv9));
        let eq100_e1412_d_b16: f64 = (eq100_e1410_d_b16 * (nv1 - nv9));
        let eq100_e1412_d_b17: f64 = (eq100_e1410_d_b17 * (nv1 - nv9));
        let eq100_e1412_d_b18: f64 = (eq100_e1410_d_b18 * (nv1 - nv9));
        let eq100_e1412_d_b19: f64 = (eq100_e1410_d_b19 * (nv1 - nv9));
        let eq100_e1412_d_b20: f64 = (eq100_e1410_d_b20 * (nv1 - nv9));
        let eq100_e1412_d_b21: f64 = (eq100_e1410_d_b21 * (nv1 - nv9));
        let eq100_e1412_d_b22: f64 = (eq100_e1410_d_b22 * (nv1 - nv9));
        let eq100_e1412_d_b23: f64 = (eq100_e1410_d_b23 * (nv1 - nv9));
        let eq100_e1412_d_b24: f64 = (eq100_e1410_d_b24 * (nv1 - nv9));
        let eq100_e1412_d_b25: f64 = (eq100_e1410_d_b25 * (nv1 - nv9));
        let eq100_e1412_d_b26: f64 = (eq100_e1410_d_b26 * (nv1 - nv9));
        let eq100_e1412_d_b27: f64 = (eq100_e1410_d_b27 * (nv1 - nv9));
        let eq100_e1412_d_b28: f64 = (eq100_e1410_d_b28 * (nv1 - nv9));
        let eq100_e1412_d_b29: f64 = (eq100_e1410_d_b29 * (nv1 - nv9));
        let eq100_e1412_d_b30: f64 = (eq100_e1410_d_b30 * (nv1 - nv9));
        let eq100_e1412_d_b31: f64 = (eq100_e1410_d_b31 * (nv1 - nv9));
        let eq100_e1412_d_b32: f64 = (eq100_e1410_d_b32 * (nv1 - nv9));
        let eq100_e1412_d_b33: f64 = (eq100_e1410_d_b33 * (nv1 - nv9));
        let eq100_e1412_d_b34: f64 = (eq100_e1410_d_b34 * (nv1 - nv9));
        let eq100_e1412_d_b35: f64 = (eq100_e1410_d_b35 * (nv1 - nv9));
        let eq100_e1412_d_b36: f64 = (eq100_e1410_d_b36 * (nv1 - nv9));
        let eq100_e1412_d_b37: f64 = (eq100_e1410_d_b37 * (nv1 - nv9));
        let eq100_e1412_d_b38: f64 = (eq100_e1410_d_b38 * (nv1 - nv9));
        let eq100_e1412_d_b39: f64 = (eq100_e1410_d_b39 * (nv1 - nv9));
        let eq100_e1412_d_b40: f64 = (eq100_e1410_d_b40 * (nv1 - nv9));
        let eq100_e1412_d_b41: f64 = (eq100_e1410_d_b41 * (nv1 - nv9));
        let eq100_e1412_d_b42: f64 = (eq100_e1410_d_b42 * (nv1 - nv9));
        let eq100_e1412_d_b43: f64 = (eq100_e1410_d_b43 * (nv1 - nv9));
        let eq100_e1412_d_b44: f64 = (eq100_e1410_d_b44 * (nv1 - nv9));
        let eq100_e1412_d_b45: f64 = (eq100_e1410_d_b45 * (nv1 - nv9));
        let eq100_e1412_d_b46: f64 = (eq100_e1410_d_b46 * (nv1 - nv9));
        let eq100_e1412_d_b47: f64 = (eq100_e1410_d_b47 * (nv1 - nv9));
        let eq100_e1412_d_b48: f64 = (eq100_e1410_d_b48 * (nv1 - nv9));
        let eq100_e1412_d_b49: f64 = (eq100_e1410_d_b49 * (nv1 - nv9));
        let eq100_e1412_d_b50: f64 = (eq100_e1410_d_b50 * (nv1 - nv9));
        let eq100_e1412_d_b51: f64 = (eq100_e1410_d_b51 * (nv1 - nv9));
        let eq100_e1412_d_b52: f64 = (eq100_e1410_d_b52 * (nv1 - nv9));
        let eq100_e1412_d_b53: f64 = (eq100_e1410_d_b53 * (nv1 - nv9));
        let eq100_e1412_d_b54: f64 = (eq100_e1410_d_b54 * (nv1 - nv9));
        (eq100_e1412, eq100_e1412_d_n0, eq100_e1412_d_n1, eq100_e1412_d_n2, eq100_e1412_d_n3, eq100_e1412_d_n4, eq100_e1412_d_n5, eq100_e1412_d_n6, eq100_e1412_d_n7, eq100_e1412_d_n8, eq100_e1412_d_n9, eq100_e1412_d_n10, eq100_e1412_d_n11, eq100_e1412_d_n12, eq100_e1412_d_n13, eq100_e1412_d_n14, eq100_e1412_d_n15, eq100_e1412_d_n16, eq100_e1412_d_n17, eq100_e1412_d_n18, eq100_e1412_d_n19, eq100_e1412_d_n20, eq100_e1412_d_n21, eq100_e1412_d_n22, eq100_e1412_d_b0, eq100_e1412_d_b1, eq100_e1412_d_b2, eq100_e1412_d_b3, eq100_e1412_d_b4, eq100_e1412_d_b5, eq100_e1412_d_b6, eq100_e1412_d_b7, eq100_e1412_d_b8, eq100_e1412_d_b9, eq100_e1412_d_b10, eq100_e1412_d_b11, eq100_e1412_d_b12, eq100_e1412_d_b13, eq100_e1412_d_b14, eq100_e1412_d_b15, eq100_e1412_d_b16, eq100_e1412_d_b17, eq100_e1412_d_b18, eq100_e1412_d_b19, eq100_e1412_d_b20, eq100_e1412_d_b21, eq100_e1412_d_b22, eq100_e1412_d_b23, eq100_e1412_d_b24, eq100_e1412_d_b25, eq100_e1412_d_b26, eq100_e1412_d_b27, eq100_e1412_d_b28, eq100_e1412_d_b29, eq100_e1412_d_b30, eq100_e1412_d_b31, eq100_e1412_d_b32, eq100_e1412_d_b33, eq100_e1412_d_b34, eq100_e1412_d_b35, eq100_e1412_d_b36, eq100_e1412_d_b37, eq100_e1412_d_b38, eq100_e1412_d_b39, eq100_e1412_d_b40, eq100_e1412_d_b41, eq100_e1412_d_b42, eq100_e1412_d_b43, eq100_e1412_d_b44, eq100_e1412_d_b45, eq100_e1412_d_b46, eq100_e1412_d_b47, eq100_e1412_d_b48, eq100_e1412_d_b49, eq100_e1412_d_b50, eq100_e1412_d_b51, eq100_e1412_d_b52, eq100_e1412_d_b53, eq100_e1412_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_value: f64 = eq100_e1414;
        let eq100_node_derivatives: [f64; 23] = [eq100_e1414_d_n0, eq100_e1414_d_n1, eq100_e1414_d_n2, eq100_e1414_d_n3, eq100_e1414_d_n4, eq100_e1414_d_n5, eq100_e1414_d_n6, eq100_e1414_d_n7, eq100_e1414_d_n8, eq100_e1414_d_n9, eq100_e1414_d_n10, eq100_e1414_d_n11, eq100_e1414_d_n12, eq100_e1414_d_n13, eq100_e1414_d_n14, eq100_e1414_d_n15, eq100_e1414_d_n16, eq100_e1414_d_n17, eq100_e1414_d_n18, eq100_e1414_d_n19, eq100_e1414_d_n20, eq100_e1414_d_n21, eq100_e1414_d_n22];
        let eq100_branch_derivatives: [f64; 55] = [eq100_e1414_d_b0, eq100_e1414_d_b1, eq100_e1414_d_b2, eq100_e1414_d_b3, eq100_e1414_d_b4, eq100_e1414_d_b5, eq100_e1414_d_b6, eq100_e1414_d_b7, eq100_e1414_d_b8, eq100_e1414_d_b9, eq100_e1414_d_b10, eq100_e1414_d_b11, eq100_e1414_d_b12, eq100_e1414_d_b13, eq100_e1414_d_b14, eq100_e1414_d_b15, eq100_e1414_d_b16, eq100_e1414_d_b17, eq100_e1414_d_b18, eq100_e1414_d_b19, eq100_e1414_d_b20, eq100_e1414_d_b21, eq100_e1414_d_b22, eq100_e1414_d_b23, eq100_e1414_d_b24, eq100_e1414_d_b25, eq100_e1414_d_b26, eq100_e1414_d_b27, eq100_e1414_d_b28, eq100_e1414_d_b29, eq100_e1414_d_b30, eq100_e1414_d_b31, eq100_e1414_d_b32, eq100_e1414_d_b33, eq100_e1414_d_b34, eq100_e1414_d_b35, eq100_e1414_d_b36, eq100_e1414_d_b37, eq100_e1414_d_b38, eq100_e1414_d_b39, eq100_e1414_d_b40, eq100_e1414_d_b41, eq100_e1414_d_b42, eq100_e1414_d_b43, eq100_e1414_d_b44, eq100_e1414_d_b45, eq100_e1414_d_b46, eq100_e1414_d_b47, eq100_e1414_d_b48, eq100_e1414_d_b49, eq100_e1414_d_b50, eq100_e1414_d_b51, eq100_e1414_d_b52, eq100_e1414_d_b53, eq100_e1414_d_b54];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq100_value),
            &eq100_node_derivatives,
            &eq100_branch_derivatives,
            multiplicity,
        );
        let (eq101_e1418,) = {
    if s.b[553] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq101_value: f64 = eq101_e1418;
        stamper.stamp_potential_const_local(
            53,
            eq101_value,
        );
        let (eq102_e1429, eq102_e1429_d_n0, eq102_e1429_d_n1, eq102_e1429_d_n2, eq102_e1429_d_n3, eq102_e1429_d_n4, eq102_e1429_d_n5, eq102_e1429_d_n6, eq102_e1429_d_n7, eq102_e1429_d_n8, eq102_e1429_d_n9, eq102_e1429_d_n10, eq102_e1429_d_n11, eq102_e1429_d_n12, eq102_e1429_d_n13, eq102_e1429_d_n14, eq102_e1429_d_n15, eq102_e1429_d_n16, eq102_e1429_d_n17, eq102_e1429_d_n18, eq102_e1429_d_n19, eq102_e1429_d_n20, eq102_e1429_d_n21, eq102_e1429_d_n22, eq102_e1429_d_b0, eq102_e1429_d_b1, eq102_e1429_d_b2, eq102_e1429_d_b3, eq102_e1429_d_b4, eq102_e1429_d_b5, eq102_e1429_d_b6, eq102_e1429_d_b7, eq102_e1429_d_b8, eq102_e1429_d_b9, eq102_e1429_d_b10, eq102_e1429_d_b11, eq102_e1429_d_b12, eq102_e1429_d_b13, eq102_e1429_d_b14, eq102_e1429_d_b15, eq102_e1429_d_b16, eq102_e1429_d_b17, eq102_e1429_d_b18, eq102_e1429_d_b19, eq102_e1429_d_b20, eq102_e1429_d_b21, eq102_e1429_d_b22, eq102_e1429_d_b23, eq102_e1429_d_b24, eq102_e1429_d_b25, eq102_e1429_d_b26, eq102_e1429_d_b27, eq102_e1429_d_b28, eq102_e1429_d_b29, eq102_e1429_d_b30, eq102_e1429_d_b31, eq102_e1429_d_b32, eq102_e1429_d_b33, eq102_e1429_d_b34, eq102_e1429_d_b35, eq102_e1429_d_b36, eq102_e1429_d_b37, eq102_e1429_d_b38, eq102_e1429_d_b39, eq102_e1429_d_b40, eq102_e1429_d_b41, eq102_e1429_d_b42, eq102_e1429_d_b43, eq102_e1429_d_b44, eq102_e1429_d_b45, eq102_e1429_d_b46, eq102_e1429_d_b47, eq102_e1429_d_b48, eq102_e1429_d_b49, eq102_e1429_d_b50, eq102_e1429_d_b51, eq102_e1429_d_b52, eq102_e1429_d_b53, eq102_e1429_d_b54,) = {
    if ((!s.b[553]) && s.b[555]) {
        let eq102_e1425: f64 = (p.p6 * s.v[319]);
        let eq102_e1425_d_n0: f64 = (p.p6 * s.dn[319][0]);
        let eq102_e1425_d_n1: f64 = (p.p6 * s.dn[319][1]);
        let eq102_e1425_d_n2: f64 = (p.p6 * s.dn[319][2]);
        let eq102_e1425_d_n3: f64 = (p.p6 * s.dn[319][3]);
        let eq102_e1425_d_n4: f64 = (p.p6 * s.dn[319][4]);
        let eq102_e1425_d_n5: f64 = (p.p6 * s.dn[319][5]);
        let eq102_e1425_d_n6: f64 = (p.p6 * s.dn[319][6]);
        let eq102_e1425_d_n7: f64 = (p.p6 * s.dn[319][7]);
        let eq102_e1425_d_n8: f64 = (p.p6 * s.dn[319][8]);
        let eq102_e1425_d_n9: f64 = (p.p6 * s.dn[319][9]);
        let eq102_e1425_d_n10: f64 = (p.p6 * s.dn[319][10]);
        let eq102_e1425_d_n11: f64 = (p.p6 * s.dn[319][11]);
        let eq102_e1425_d_n12: f64 = (p.p6 * s.dn[319][12]);
        let eq102_e1425_d_n13: f64 = (p.p6 * s.dn[319][13]);
        let eq102_e1425_d_n14: f64 = (p.p6 * s.dn[319][14]);
        let eq102_e1425_d_n15: f64 = (p.p6 * s.dn[319][15]);
        let eq102_e1425_d_n16: f64 = (p.p6 * s.dn[319][16]);
        let eq102_e1425_d_n17: f64 = (p.p6 * s.dn[319][17]);
        let eq102_e1425_d_n18: f64 = (p.p6 * s.dn[319][18]);
        let eq102_e1425_d_n19: f64 = (p.p6 * s.dn[319][19]);
        let eq102_e1425_d_n20: f64 = (p.p6 * s.dn[319][20]);
        let eq102_e1425_d_n21: f64 = (p.p6 * s.dn[319][21]);
        let eq102_e1425_d_n22: f64 = (p.p6 * s.dn[319][22]);
        let eq102_e1425_d_b0: f64 = (p.p6 * s.db[319][0]);
        let eq102_e1425_d_b1: f64 = (p.p6 * s.db[319][1]);
        let eq102_e1425_d_b2: f64 = (p.p6 * s.db[319][2]);
        let eq102_e1425_d_b3: f64 = (p.p6 * s.db[319][3]);
        let eq102_e1425_d_b4: f64 = (p.p6 * s.db[319][4]);
        let eq102_e1425_d_b5: f64 = (p.p6 * s.db[319][5]);
        let eq102_e1425_d_b6: f64 = (p.p6 * s.db[319][6]);
        let eq102_e1425_d_b7: f64 = (p.p6 * s.db[319][7]);
        let eq102_e1425_d_b8: f64 = (p.p6 * s.db[319][8]);
        let eq102_e1425_d_b9: f64 = (p.p6 * s.db[319][9]);
        let eq102_e1425_d_b10: f64 = (p.p6 * s.db[319][10]);
        let eq102_e1425_d_b11: f64 = (p.p6 * s.db[319][11]);
        let eq102_e1425_d_b12: f64 = (p.p6 * s.db[319][12]);
        let eq102_e1425_d_b13: f64 = (p.p6 * s.db[319][13]);
        let eq102_e1425_d_b14: f64 = (p.p6 * s.db[319][14]);
        let eq102_e1425_d_b15: f64 = (p.p6 * s.db[319][15]);
        let eq102_e1425_d_b16: f64 = (p.p6 * s.db[319][16]);
        let eq102_e1425_d_b17: f64 = (p.p6 * s.db[319][17]);
        let eq102_e1425_d_b18: f64 = (p.p6 * s.db[319][18]);
        let eq102_e1425_d_b19: f64 = (p.p6 * s.db[319][19]);
        let eq102_e1425_d_b20: f64 = (p.p6 * s.db[319][20]);
        let eq102_e1425_d_b21: f64 = (p.p6 * s.db[319][21]);
        let eq102_e1425_d_b22: f64 = (p.p6 * s.db[319][22]);
        let eq102_e1425_d_b23: f64 = (p.p6 * s.db[319][23]);
        let eq102_e1425_d_b24: f64 = (p.p6 * s.db[319][24]);
        let eq102_e1425_d_b25: f64 = (p.p6 * s.db[319][25]);
        let eq102_e1425_d_b26: f64 = (p.p6 * s.db[319][26]);
        let eq102_e1425_d_b27: f64 = (p.p6 * s.db[319][27]);
        let eq102_e1425_d_b28: f64 = (p.p6 * s.db[319][28]);
        let eq102_e1425_d_b29: f64 = (p.p6 * s.db[319][29]);
        let eq102_e1425_d_b30: f64 = (p.p6 * s.db[319][30]);
        let eq102_e1425_d_b31: f64 = (p.p6 * s.db[319][31]);
        let eq102_e1425_d_b32: f64 = (p.p6 * s.db[319][32]);
        let eq102_e1425_d_b33: f64 = (p.p6 * s.db[319][33]);
        let eq102_e1425_d_b34: f64 = (p.p6 * s.db[319][34]);
        let eq102_e1425_d_b35: f64 = (p.p6 * s.db[319][35]);
        let eq102_e1425_d_b36: f64 = (p.p6 * s.db[319][36]);
        let eq102_e1425_d_b37: f64 = (p.p6 * s.db[319][37]);
        let eq102_e1425_d_b38: f64 = (p.p6 * s.db[319][38]);
        let eq102_e1425_d_b39: f64 = (p.p6 * s.db[319][39]);
        let eq102_e1425_d_b40: f64 = (p.p6 * s.db[319][40]);
        let eq102_e1425_d_b41: f64 = (p.p6 * s.db[319][41]);
        let eq102_e1425_d_b42: f64 = (p.p6 * s.db[319][42]);
        let eq102_e1425_d_b43: f64 = (p.p6 * s.db[319][43]);
        let eq102_e1425_d_b44: f64 = (p.p6 * s.db[319][44]);
        let eq102_e1425_d_b45: f64 = (p.p6 * s.db[319][45]);
        let eq102_e1425_d_b46: f64 = (p.p6 * s.db[319][46]);
        let eq102_e1425_d_b47: f64 = (p.p6 * s.db[319][47]);
        let eq102_e1425_d_b48: f64 = (p.p6 * s.db[319][48]);
        let eq102_e1425_d_b49: f64 = (p.p6 * s.db[319][49]);
        let eq102_e1425_d_b50: f64 = (p.p6 * s.db[319][50]);
        let eq102_e1425_d_b51: f64 = (p.p6 * s.db[319][51]);
        let eq102_e1425_d_b52: f64 = (p.p6 * s.db[319][52]);
        let eq102_e1425_d_b53: f64 = (p.p6 * s.db[319][53]);
        let eq102_e1425_d_b54: f64 = (p.p6 * s.db[319][54]);
        let eq102_e1427: f64 = (eq102_e1425 * (nv1 - nv10));
        let eq102_e1427_d_n0: f64 = (eq102_e1425_d_n0 * (nv1 - nv10));
        let eq102_e1427_d_n1: f64 = ((eq102_e1425_d_n1 * (nv1 - nv10)) + eq102_e1425);
        let eq102_e1427_d_n2: f64 = (eq102_e1425_d_n2 * (nv1 - nv10));
        let eq102_e1427_d_n3: f64 = (eq102_e1425_d_n3 * (nv1 - nv10));
        let eq102_e1427_d_n4: f64 = (eq102_e1425_d_n4 * (nv1 - nv10));
        let eq102_e1427_d_n5: f64 = (eq102_e1425_d_n5 * (nv1 - nv10));
        let eq102_e1427_d_n6: f64 = (eq102_e1425_d_n6 * (nv1 - nv10));
        let eq102_e1427_d_n7: f64 = (eq102_e1425_d_n7 * (nv1 - nv10));
        let eq102_e1427_d_n8: f64 = (eq102_e1425_d_n8 * (nv1 - nv10));
        let eq102_e1427_d_n9: f64 = (eq102_e1425_d_n9 * (nv1 - nv10));
        let eq102_e1427_d_n10: f64 = ((eq102_e1425_d_n10 * (nv1 - nv10)) + (-eq102_e1425));
        let eq102_e1427_d_n11: f64 = (eq102_e1425_d_n11 * (nv1 - nv10));
        let eq102_e1427_d_n12: f64 = (eq102_e1425_d_n12 * (nv1 - nv10));
        let eq102_e1427_d_n13: f64 = (eq102_e1425_d_n13 * (nv1 - nv10));
        let eq102_e1427_d_n14: f64 = (eq102_e1425_d_n14 * (nv1 - nv10));
        let eq102_e1427_d_n15: f64 = (eq102_e1425_d_n15 * (nv1 - nv10));
        let eq102_e1427_d_n16: f64 = (eq102_e1425_d_n16 * (nv1 - nv10));
        let eq102_e1427_d_n17: f64 = (eq102_e1425_d_n17 * (nv1 - nv10));
        let eq102_e1427_d_n18: f64 = (eq102_e1425_d_n18 * (nv1 - nv10));
        let eq102_e1427_d_n19: f64 = (eq102_e1425_d_n19 * (nv1 - nv10));
        let eq102_e1427_d_n20: f64 = (eq102_e1425_d_n20 * (nv1 - nv10));
        let eq102_e1427_d_n21: f64 = (eq102_e1425_d_n21 * (nv1 - nv10));
        let eq102_e1427_d_n22: f64 = (eq102_e1425_d_n22 * (nv1 - nv10));
        let eq102_e1427_d_b0: f64 = (eq102_e1425_d_b0 * (nv1 - nv10));
        let eq102_e1427_d_b1: f64 = (eq102_e1425_d_b1 * (nv1 - nv10));
        let eq102_e1427_d_b2: f64 = (eq102_e1425_d_b2 * (nv1 - nv10));
        let eq102_e1427_d_b3: f64 = (eq102_e1425_d_b3 * (nv1 - nv10));
        let eq102_e1427_d_b4: f64 = (eq102_e1425_d_b4 * (nv1 - nv10));
        let eq102_e1427_d_b5: f64 = (eq102_e1425_d_b5 * (nv1 - nv10));
        let eq102_e1427_d_b6: f64 = (eq102_e1425_d_b6 * (nv1 - nv10));
        let eq102_e1427_d_b7: f64 = (eq102_e1425_d_b7 * (nv1 - nv10));
        let eq102_e1427_d_b8: f64 = (eq102_e1425_d_b8 * (nv1 - nv10));
        let eq102_e1427_d_b9: f64 = (eq102_e1425_d_b9 * (nv1 - nv10));
        let eq102_e1427_d_b10: f64 = (eq102_e1425_d_b10 * (nv1 - nv10));
        let eq102_e1427_d_b11: f64 = (eq102_e1425_d_b11 * (nv1 - nv10));
        let eq102_e1427_d_b12: f64 = (eq102_e1425_d_b12 * (nv1 - nv10));
        let eq102_e1427_d_b13: f64 = (eq102_e1425_d_b13 * (nv1 - nv10));
        let eq102_e1427_d_b14: f64 = (eq102_e1425_d_b14 * (nv1 - nv10));
        let eq102_e1427_d_b15: f64 = (eq102_e1425_d_b15 * (nv1 - nv10));
        let eq102_e1427_d_b16: f64 = (eq102_e1425_d_b16 * (nv1 - nv10));
        let eq102_e1427_d_b17: f64 = (eq102_e1425_d_b17 * (nv1 - nv10));
        let eq102_e1427_d_b18: f64 = (eq102_e1425_d_b18 * (nv1 - nv10));
        let eq102_e1427_d_b19: f64 = (eq102_e1425_d_b19 * (nv1 - nv10));
        let eq102_e1427_d_b20: f64 = (eq102_e1425_d_b20 * (nv1 - nv10));
        let eq102_e1427_d_b21: f64 = (eq102_e1425_d_b21 * (nv1 - nv10));
        let eq102_e1427_d_b22: f64 = (eq102_e1425_d_b22 * (nv1 - nv10));
        let eq102_e1427_d_b23: f64 = (eq102_e1425_d_b23 * (nv1 - nv10));
        let eq102_e1427_d_b24: f64 = (eq102_e1425_d_b24 * (nv1 - nv10));
        let eq102_e1427_d_b25: f64 = (eq102_e1425_d_b25 * (nv1 - nv10));
        let eq102_e1427_d_b26: f64 = (eq102_e1425_d_b26 * (nv1 - nv10));
        let eq102_e1427_d_b27: f64 = (eq102_e1425_d_b27 * (nv1 - nv10));
        let eq102_e1427_d_b28: f64 = (eq102_e1425_d_b28 * (nv1 - nv10));
        let eq102_e1427_d_b29: f64 = (eq102_e1425_d_b29 * (nv1 - nv10));
        let eq102_e1427_d_b30: f64 = (eq102_e1425_d_b30 * (nv1 - nv10));
        let eq102_e1427_d_b31: f64 = (eq102_e1425_d_b31 * (nv1 - nv10));
        let eq102_e1427_d_b32: f64 = (eq102_e1425_d_b32 * (nv1 - nv10));
        let eq102_e1427_d_b33: f64 = (eq102_e1425_d_b33 * (nv1 - nv10));
        let eq102_e1427_d_b34: f64 = (eq102_e1425_d_b34 * (nv1 - nv10));
        let eq102_e1427_d_b35: f64 = (eq102_e1425_d_b35 * (nv1 - nv10));
        let eq102_e1427_d_b36: f64 = (eq102_e1425_d_b36 * (nv1 - nv10));
        let eq102_e1427_d_b37: f64 = (eq102_e1425_d_b37 * (nv1 - nv10));
        let eq102_e1427_d_b38: f64 = (eq102_e1425_d_b38 * (nv1 - nv10));
        let eq102_e1427_d_b39: f64 = (eq102_e1425_d_b39 * (nv1 - nv10));
        let eq102_e1427_d_b40: f64 = (eq102_e1425_d_b40 * (nv1 - nv10));
        let eq102_e1427_d_b41: f64 = (eq102_e1425_d_b41 * (nv1 - nv10));
        let eq102_e1427_d_b42: f64 = (eq102_e1425_d_b42 * (nv1 - nv10));
        let eq102_e1427_d_b43: f64 = (eq102_e1425_d_b43 * (nv1 - nv10));
        let eq102_e1427_d_b44: f64 = (eq102_e1425_d_b44 * (nv1 - nv10));
        let eq102_e1427_d_b45: f64 = (eq102_e1425_d_b45 * (nv1 - nv10));
        let eq102_e1427_d_b46: f64 = (eq102_e1425_d_b46 * (nv1 - nv10));
        let eq102_e1427_d_b47: f64 = (eq102_e1425_d_b47 * (nv1 - nv10));
        let eq102_e1427_d_b48: f64 = (eq102_e1425_d_b48 * (nv1 - nv10));
        let eq102_e1427_d_b49: f64 = (eq102_e1425_d_b49 * (nv1 - nv10));
        let eq102_e1427_d_b50: f64 = (eq102_e1425_d_b50 * (nv1 - nv10));
        let eq102_e1427_d_b51: f64 = (eq102_e1425_d_b51 * (nv1 - nv10));
        let eq102_e1427_d_b52: f64 = (eq102_e1425_d_b52 * (nv1 - nv10));
        let eq102_e1427_d_b53: f64 = (eq102_e1425_d_b53 * (nv1 - nv10));
        let eq102_e1427_d_b54: f64 = (eq102_e1425_d_b54 * (nv1 - nv10));
        (eq102_e1427, eq102_e1427_d_n0, eq102_e1427_d_n1, eq102_e1427_d_n2, eq102_e1427_d_n3, eq102_e1427_d_n4, eq102_e1427_d_n5, eq102_e1427_d_n6, eq102_e1427_d_n7, eq102_e1427_d_n8, eq102_e1427_d_n9, eq102_e1427_d_n10, eq102_e1427_d_n11, eq102_e1427_d_n12, eq102_e1427_d_n13, eq102_e1427_d_n14, eq102_e1427_d_n15, eq102_e1427_d_n16, eq102_e1427_d_n17, eq102_e1427_d_n18, eq102_e1427_d_n19, eq102_e1427_d_n20, eq102_e1427_d_n21, eq102_e1427_d_n22, eq102_e1427_d_b0, eq102_e1427_d_b1, eq102_e1427_d_b2, eq102_e1427_d_b3, eq102_e1427_d_b4, eq102_e1427_d_b5, eq102_e1427_d_b6, eq102_e1427_d_b7, eq102_e1427_d_b8, eq102_e1427_d_b9, eq102_e1427_d_b10, eq102_e1427_d_b11, eq102_e1427_d_b12, eq102_e1427_d_b13, eq102_e1427_d_b14, eq102_e1427_d_b15, eq102_e1427_d_b16, eq102_e1427_d_b17, eq102_e1427_d_b18, eq102_e1427_d_b19, eq102_e1427_d_b20, eq102_e1427_d_b21, eq102_e1427_d_b22, eq102_e1427_d_b23, eq102_e1427_d_b24, eq102_e1427_d_b25, eq102_e1427_d_b26, eq102_e1427_d_b27, eq102_e1427_d_b28, eq102_e1427_d_b29, eq102_e1427_d_b30, eq102_e1427_d_b31, eq102_e1427_d_b32, eq102_e1427_d_b33, eq102_e1427_d_b34, eq102_e1427_d_b35, eq102_e1427_d_b36, eq102_e1427_d_b37, eq102_e1427_d_b38, eq102_e1427_d_b39, eq102_e1427_d_b40, eq102_e1427_d_b41, eq102_e1427_d_b42, eq102_e1427_d_b43, eq102_e1427_d_b44, eq102_e1427_d_b45, eq102_e1427_d_b46, eq102_e1427_d_b47, eq102_e1427_d_b48, eq102_e1427_d_b49, eq102_e1427_d_b50, eq102_e1427_d_b51, eq102_e1427_d_b52, eq102_e1427_d_b53, eq102_e1427_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_value: f64 = eq102_e1429;
        let eq102_node_derivatives: [f64; 23] = [eq102_e1429_d_n0, eq102_e1429_d_n1, eq102_e1429_d_n2, eq102_e1429_d_n3, eq102_e1429_d_n4, eq102_e1429_d_n5, eq102_e1429_d_n6, eq102_e1429_d_n7, eq102_e1429_d_n8, eq102_e1429_d_n9, eq102_e1429_d_n10, eq102_e1429_d_n11, eq102_e1429_d_n12, eq102_e1429_d_n13, eq102_e1429_d_n14, eq102_e1429_d_n15, eq102_e1429_d_n16, eq102_e1429_d_n17, eq102_e1429_d_n18, eq102_e1429_d_n19, eq102_e1429_d_n20, eq102_e1429_d_n21, eq102_e1429_d_n22];
        let eq102_branch_derivatives: [f64; 55] = [eq102_e1429_d_b0, eq102_e1429_d_b1, eq102_e1429_d_b2, eq102_e1429_d_b3, eq102_e1429_d_b4, eq102_e1429_d_b5, eq102_e1429_d_b6, eq102_e1429_d_b7, eq102_e1429_d_b8, eq102_e1429_d_b9, eq102_e1429_d_b10, eq102_e1429_d_b11, eq102_e1429_d_b12, eq102_e1429_d_b13, eq102_e1429_d_b14, eq102_e1429_d_b15, eq102_e1429_d_b16, eq102_e1429_d_b17, eq102_e1429_d_b18, eq102_e1429_d_b19, eq102_e1429_d_b20, eq102_e1429_d_b21, eq102_e1429_d_b22, eq102_e1429_d_b23, eq102_e1429_d_b24, eq102_e1429_d_b25, eq102_e1429_d_b26, eq102_e1429_d_b27, eq102_e1429_d_b28, eq102_e1429_d_b29, eq102_e1429_d_b30, eq102_e1429_d_b31, eq102_e1429_d_b32, eq102_e1429_d_b33, eq102_e1429_d_b34, eq102_e1429_d_b35, eq102_e1429_d_b36, eq102_e1429_d_b37, eq102_e1429_d_b38, eq102_e1429_d_b39, eq102_e1429_d_b40, eq102_e1429_d_b41, eq102_e1429_d_b42, eq102_e1429_d_b43, eq102_e1429_d_b44, eq102_e1429_d_b45, eq102_e1429_d_b46, eq102_e1429_d_b47, eq102_e1429_d_b48, eq102_e1429_d_b49, eq102_e1429_d_b50, eq102_e1429_d_b51, eq102_e1429_d_b52, eq102_e1429_d_b53, eq102_e1429_d_b54];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq102_value),
            &eq102_node_derivatives,
            &eq102_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_21(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq103_e1440, eq103_e1440_d_n0, eq103_e1440_d_n1, eq103_e1440_d_n2, eq103_e1440_d_n3, eq103_e1440_d_n4, eq103_e1440_d_n5, eq103_e1440_d_n6, eq103_e1440_d_n7, eq103_e1440_d_n8, eq103_e1440_d_n9, eq103_e1440_d_n10, eq103_e1440_d_n11, eq103_e1440_d_n12, eq103_e1440_d_n13, eq103_e1440_d_n14, eq103_e1440_d_n15, eq103_e1440_d_n16, eq103_e1440_d_n17, eq103_e1440_d_n18, eq103_e1440_d_n19, eq103_e1440_d_n20, eq103_e1440_d_n21, eq103_e1440_d_n22, eq103_e1440_d_b0, eq103_e1440_d_b1, eq103_e1440_d_b2, eq103_e1440_d_b3, eq103_e1440_d_b4, eq103_e1440_d_b5, eq103_e1440_d_b6, eq103_e1440_d_b7, eq103_e1440_d_b8, eq103_e1440_d_b9, eq103_e1440_d_b10, eq103_e1440_d_b11, eq103_e1440_d_b12, eq103_e1440_d_b13, eq103_e1440_d_b14, eq103_e1440_d_b15, eq103_e1440_d_b16, eq103_e1440_d_b17, eq103_e1440_d_b18, eq103_e1440_d_b19, eq103_e1440_d_b20, eq103_e1440_d_b21, eq103_e1440_d_b22, eq103_e1440_d_b23, eq103_e1440_d_b24, eq103_e1440_d_b25, eq103_e1440_d_b26, eq103_e1440_d_b27, eq103_e1440_d_b28, eq103_e1440_d_b29, eq103_e1440_d_b30, eq103_e1440_d_b31, eq103_e1440_d_b32, eq103_e1440_d_b33, eq103_e1440_d_b34, eq103_e1440_d_b35, eq103_e1440_d_b36, eq103_e1440_d_b37, eq103_e1440_d_b38, eq103_e1440_d_b39, eq103_e1440_d_b40, eq103_e1440_d_b41, eq103_e1440_d_b42, eq103_e1440_d_b43, eq103_e1440_d_b44, eq103_e1440_d_b45, eq103_e1440_d_b46, eq103_e1440_d_b47, eq103_e1440_d_b48, eq103_e1440_d_b49, eq103_e1440_d_b50, eq103_e1440_d_b51, eq103_e1440_d_b52, eq103_e1440_d_b53, eq103_e1440_d_b54,) = {
    if ((!s.b[553]) && s.b[555]) {
        let eq103_e1436: f64 = (p.p6 * s.v[320]);
        let eq103_e1436_d_n0: f64 = (p.p6 * s.dn[320][0]);
        let eq103_e1436_d_n1: f64 = (p.p6 * s.dn[320][1]);
        let eq103_e1436_d_n2: f64 = (p.p6 * s.dn[320][2]);
        let eq103_e1436_d_n3: f64 = (p.p6 * s.dn[320][3]);
        let eq103_e1436_d_n4: f64 = (p.p6 * s.dn[320][4]);
        let eq103_e1436_d_n5: f64 = (p.p6 * s.dn[320][5]);
        let eq103_e1436_d_n6: f64 = (p.p6 * s.dn[320][6]);
        let eq103_e1436_d_n7: f64 = (p.p6 * s.dn[320][7]);
        let eq103_e1436_d_n8: f64 = (p.p6 * s.dn[320][8]);
        let eq103_e1436_d_n9: f64 = (p.p6 * s.dn[320][9]);
        let eq103_e1436_d_n10: f64 = (p.p6 * s.dn[320][10]);
        let eq103_e1436_d_n11: f64 = (p.p6 * s.dn[320][11]);
        let eq103_e1436_d_n12: f64 = (p.p6 * s.dn[320][12]);
        let eq103_e1436_d_n13: f64 = (p.p6 * s.dn[320][13]);
        let eq103_e1436_d_n14: f64 = (p.p6 * s.dn[320][14]);
        let eq103_e1436_d_n15: f64 = (p.p6 * s.dn[320][15]);
        let eq103_e1436_d_n16: f64 = (p.p6 * s.dn[320][16]);
        let eq103_e1436_d_n17: f64 = (p.p6 * s.dn[320][17]);
        let eq103_e1436_d_n18: f64 = (p.p6 * s.dn[320][18]);
        let eq103_e1436_d_n19: f64 = (p.p6 * s.dn[320][19]);
        let eq103_e1436_d_n20: f64 = (p.p6 * s.dn[320][20]);
        let eq103_e1436_d_n21: f64 = (p.p6 * s.dn[320][21]);
        let eq103_e1436_d_n22: f64 = (p.p6 * s.dn[320][22]);
        let eq103_e1436_d_b0: f64 = (p.p6 * s.db[320][0]);
        let eq103_e1436_d_b1: f64 = (p.p6 * s.db[320][1]);
        let eq103_e1436_d_b2: f64 = (p.p6 * s.db[320][2]);
        let eq103_e1436_d_b3: f64 = (p.p6 * s.db[320][3]);
        let eq103_e1436_d_b4: f64 = (p.p6 * s.db[320][4]);
        let eq103_e1436_d_b5: f64 = (p.p6 * s.db[320][5]);
        let eq103_e1436_d_b6: f64 = (p.p6 * s.db[320][6]);
        let eq103_e1436_d_b7: f64 = (p.p6 * s.db[320][7]);
        let eq103_e1436_d_b8: f64 = (p.p6 * s.db[320][8]);
        let eq103_e1436_d_b9: f64 = (p.p6 * s.db[320][9]);
        let eq103_e1436_d_b10: f64 = (p.p6 * s.db[320][10]);
        let eq103_e1436_d_b11: f64 = (p.p6 * s.db[320][11]);
        let eq103_e1436_d_b12: f64 = (p.p6 * s.db[320][12]);
        let eq103_e1436_d_b13: f64 = (p.p6 * s.db[320][13]);
        let eq103_e1436_d_b14: f64 = (p.p6 * s.db[320][14]);
        let eq103_e1436_d_b15: f64 = (p.p6 * s.db[320][15]);
        let eq103_e1436_d_b16: f64 = (p.p6 * s.db[320][16]);
        let eq103_e1436_d_b17: f64 = (p.p6 * s.db[320][17]);
        let eq103_e1436_d_b18: f64 = (p.p6 * s.db[320][18]);
        let eq103_e1436_d_b19: f64 = (p.p6 * s.db[320][19]);
        let eq103_e1436_d_b20: f64 = (p.p6 * s.db[320][20]);
        let eq103_e1436_d_b21: f64 = (p.p6 * s.db[320][21]);
        let eq103_e1436_d_b22: f64 = (p.p6 * s.db[320][22]);
        let eq103_e1436_d_b23: f64 = (p.p6 * s.db[320][23]);
        let eq103_e1436_d_b24: f64 = (p.p6 * s.db[320][24]);
        let eq103_e1436_d_b25: f64 = (p.p6 * s.db[320][25]);
        let eq103_e1436_d_b26: f64 = (p.p6 * s.db[320][26]);
        let eq103_e1436_d_b27: f64 = (p.p6 * s.db[320][27]);
        let eq103_e1436_d_b28: f64 = (p.p6 * s.db[320][28]);
        let eq103_e1436_d_b29: f64 = (p.p6 * s.db[320][29]);
        let eq103_e1436_d_b30: f64 = (p.p6 * s.db[320][30]);
        let eq103_e1436_d_b31: f64 = (p.p6 * s.db[320][31]);
        let eq103_e1436_d_b32: f64 = (p.p6 * s.db[320][32]);
        let eq103_e1436_d_b33: f64 = (p.p6 * s.db[320][33]);
        let eq103_e1436_d_b34: f64 = (p.p6 * s.db[320][34]);
        let eq103_e1436_d_b35: f64 = (p.p6 * s.db[320][35]);
        let eq103_e1436_d_b36: f64 = (p.p6 * s.db[320][36]);
        let eq103_e1436_d_b37: f64 = (p.p6 * s.db[320][37]);
        let eq103_e1436_d_b38: f64 = (p.p6 * s.db[320][38]);
        let eq103_e1436_d_b39: f64 = (p.p6 * s.db[320][39]);
        let eq103_e1436_d_b40: f64 = (p.p6 * s.db[320][40]);
        let eq103_e1436_d_b41: f64 = (p.p6 * s.db[320][41]);
        let eq103_e1436_d_b42: f64 = (p.p6 * s.db[320][42]);
        let eq103_e1436_d_b43: f64 = (p.p6 * s.db[320][43]);
        let eq103_e1436_d_b44: f64 = (p.p6 * s.db[320][44]);
        let eq103_e1436_d_b45: f64 = (p.p6 * s.db[320][45]);
        let eq103_e1436_d_b46: f64 = (p.p6 * s.db[320][46]);
        let eq103_e1436_d_b47: f64 = (p.p6 * s.db[320][47]);
        let eq103_e1436_d_b48: f64 = (p.p6 * s.db[320][48]);
        let eq103_e1436_d_b49: f64 = (p.p6 * s.db[320][49]);
        let eq103_e1436_d_b50: f64 = (p.p6 * s.db[320][50]);
        let eq103_e1436_d_b51: f64 = (p.p6 * s.db[320][51]);
        let eq103_e1436_d_b52: f64 = (p.p6 * s.db[320][52]);
        let eq103_e1436_d_b53: f64 = (p.p6 * s.db[320][53]);
        let eq103_e1436_d_b54: f64 = (p.p6 * s.db[320][54]);
        let eq103_e1438: f64 = (eq103_e1436 * (nv10 - nv9));
        let eq103_e1438_d_n0: f64 = (eq103_e1436_d_n0 * (nv10 - nv9));
        let eq103_e1438_d_n1: f64 = (eq103_e1436_d_n1 * (nv10 - nv9));
        let eq103_e1438_d_n2: f64 = (eq103_e1436_d_n2 * (nv10 - nv9));
        let eq103_e1438_d_n3: f64 = (eq103_e1436_d_n3 * (nv10 - nv9));
        let eq103_e1438_d_n4: f64 = (eq103_e1436_d_n4 * (nv10 - nv9));
        let eq103_e1438_d_n5: f64 = (eq103_e1436_d_n5 * (nv10 - nv9));
        let eq103_e1438_d_n6: f64 = (eq103_e1436_d_n6 * (nv10 - nv9));
        let eq103_e1438_d_n7: f64 = (eq103_e1436_d_n7 * (nv10 - nv9));
        let eq103_e1438_d_n8: f64 = (eq103_e1436_d_n8 * (nv10 - nv9));
        let eq103_e1438_d_n9: f64 = ((eq103_e1436_d_n9 * (nv10 - nv9)) + (-eq103_e1436));
        let eq103_e1438_d_n10: f64 = ((eq103_e1436_d_n10 * (nv10 - nv9)) + eq103_e1436);
        let eq103_e1438_d_n11: f64 = (eq103_e1436_d_n11 * (nv10 - nv9));
        let eq103_e1438_d_n12: f64 = (eq103_e1436_d_n12 * (nv10 - nv9));
        let eq103_e1438_d_n13: f64 = (eq103_e1436_d_n13 * (nv10 - nv9));
        let eq103_e1438_d_n14: f64 = (eq103_e1436_d_n14 * (nv10 - nv9));
        let eq103_e1438_d_n15: f64 = (eq103_e1436_d_n15 * (nv10 - nv9));
        let eq103_e1438_d_n16: f64 = (eq103_e1436_d_n16 * (nv10 - nv9));
        let eq103_e1438_d_n17: f64 = (eq103_e1436_d_n17 * (nv10 - nv9));
        let eq103_e1438_d_n18: f64 = (eq103_e1436_d_n18 * (nv10 - nv9));
        let eq103_e1438_d_n19: f64 = (eq103_e1436_d_n19 * (nv10 - nv9));
        let eq103_e1438_d_n20: f64 = (eq103_e1436_d_n20 * (nv10 - nv9));
        let eq103_e1438_d_n21: f64 = (eq103_e1436_d_n21 * (nv10 - nv9));
        let eq103_e1438_d_n22: f64 = (eq103_e1436_d_n22 * (nv10 - nv9));
        let eq103_e1438_d_b0: f64 = (eq103_e1436_d_b0 * (nv10 - nv9));
        let eq103_e1438_d_b1: f64 = (eq103_e1436_d_b1 * (nv10 - nv9));
        let eq103_e1438_d_b2: f64 = (eq103_e1436_d_b2 * (nv10 - nv9));
        let eq103_e1438_d_b3: f64 = (eq103_e1436_d_b3 * (nv10 - nv9));
        let eq103_e1438_d_b4: f64 = (eq103_e1436_d_b4 * (nv10 - nv9));
        let eq103_e1438_d_b5: f64 = (eq103_e1436_d_b5 * (nv10 - nv9));
        let eq103_e1438_d_b6: f64 = (eq103_e1436_d_b6 * (nv10 - nv9));
        let eq103_e1438_d_b7: f64 = (eq103_e1436_d_b7 * (nv10 - nv9));
        let eq103_e1438_d_b8: f64 = (eq103_e1436_d_b8 * (nv10 - nv9));
        let eq103_e1438_d_b9: f64 = (eq103_e1436_d_b9 * (nv10 - nv9));
        let eq103_e1438_d_b10: f64 = (eq103_e1436_d_b10 * (nv10 - nv9));
        let eq103_e1438_d_b11: f64 = (eq103_e1436_d_b11 * (nv10 - nv9));
        let eq103_e1438_d_b12: f64 = (eq103_e1436_d_b12 * (nv10 - nv9));
        let eq103_e1438_d_b13: f64 = (eq103_e1436_d_b13 * (nv10 - nv9));
        let eq103_e1438_d_b14: f64 = (eq103_e1436_d_b14 * (nv10 - nv9));
        let eq103_e1438_d_b15: f64 = (eq103_e1436_d_b15 * (nv10 - nv9));
        let eq103_e1438_d_b16: f64 = (eq103_e1436_d_b16 * (nv10 - nv9));
        let eq103_e1438_d_b17: f64 = (eq103_e1436_d_b17 * (nv10 - nv9));
        let eq103_e1438_d_b18: f64 = (eq103_e1436_d_b18 * (nv10 - nv9));
        let eq103_e1438_d_b19: f64 = (eq103_e1436_d_b19 * (nv10 - nv9));
        let eq103_e1438_d_b20: f64 = (eq103_e1436_d_b20 * (nv10 - nv9));
        let eq103_e1438_d_b21: f64 = (eq103_e1436_d_b21 * (nv10 - nv9));
        let eq103_e1438_d_b22: f64 = (eq103_e1436_d_b22 * (nv10 - nv9));
        let eq103_e1438_d_b23: f64 = (eq103_e1436_d_b23 * (nv10 - nv9));
        let eq103_e1438_d_b24: f64 = (eq103_e1436_d_b24 * (nv10 - nv9));
        let eq103_e1438_d_b25: f64 = (eq103_e1436_d_b25 * (nv10 - nv9));
        let eq103_e1438_d_b26: f64 = (eq103_e1436_d_b26 * (nv10 - nv9));
        let eq103_e1438_d_b27: f64 = (eq103_e1436_d_b27 * (nv10 - nv9));
        let eq103_e1438_d_b28: f64 = (eq103_e1436_d_b28 * (nv10 - nv9));
        let eq103_e1438_d_b29: f64 = (eq103_e1436_d_b29 * (nv10 - nv9));
        let eq103_e1438_d_b30: f64 = (eq103_e1436_d_b30 * (nv10 - nv9));
        let eq103_e1438_d_b31: f64 = (eq103_e1436_d_b31 * (nv10 - nv9));
        let eq103_e1438_d_b32: f64 = (eq103_e1436_d_b32 * (nv10 - nv9));
        let eq103_e1438_d_b33: f64 = (eq103_e1436_d_b33 * (nv10 - nv9));
        let eq103_e1438_d_b34: f64 = (eq103_e1436_d_b34 * (nv10 - nv9));
        let eq103_e1438_d_b35: f64 = (eq103_e1436_d_b35 * (nv10 - nv9));
        let eq103_e1438_d_b36: f64 = (eq103_e1436_d_b36 * (nv10 - nv9));
        let eq103_e1438_d_b37: f64 = (eq103_e1436_d_b37 * (nv10 - nv9));
        let eq103_e1438_d_b38: f64 = (eq103_e1436_d_b38 * (nv10 - nv9));
        let eq103_e1438_d_b39: f64 = (eq103_e1436_d_b39 * (nv10 - nv9));
        let eq103_e1438_d_b40: f64 = (eq103_e1436_d_b40 * (nv10 - nv9));
        let eq103_e1438_d_b41: f64 = (eq103_e1436_d_b41 * (nv10 - nv9));
        let eq103_e1438_d_b42: f64 = (eq103_e1436_d_b42 * (nv10 - nv9));
        let eq103_e1438_d_b43: f64 = (eq103_e1436_d_b43 * (nv10 - nv9));
        let eq103_e1438_d_b44: f64 = (eq103_e1436_d_b44 * (nv10 - nv9));
        let eq103_e1438_d_b45: f64 = (eq103_e1436_d_b45 * (nv10 - nv9));
        let eq103_e1438_d_b46: f64 = (eq103_e1436_d_b46 * (nv10 - nv9));
        let eq103_e1438_d_b47: f64 = (eq103_e1436_d_b47 * (nv10 - nv9));
        let eq103_e1438_d_b48: f64 = (eq103_e1436_d_b48 * (nv10 - nv9));
        let eq103_e1438_d_b49: f64 = (eq103_e1436_d_b49 * (nv10 - nv9));
        let eq103_e1438_d_b50: f64 = (eq103_e1436_d_b50 * (nv10 - nv9));
        let eq103_e1438_d_b51: f64 = (eq103_e1436_d_b51 * (nv10 - nv9));
        let eq103_e1438_d_b52: f64 = (eq103_e1436_d_b52 * (nv10 - nv9));
        let eq103_e1438_d_b53: f64 = (eq103_e1436_d_b53 * (nv10 - nv9));
        let eq103_e1438_d_b54: f64 = (eq103_e1436_d_b54 * (nv10 - nv9));
        (eq103_e1438, eq103_e1438_d_n0, eq103_e1438_d_n1, eq103_e1438_d_n2, eq103_e1438_d_n3, eq103_e1438_d_n4, eq103_e1438_d_n5, eq103_e1438_d_n6, eq103_e1438_d_n7, eq103_e1438_d_n8, eq103_e1438_d_n9, eq103_e1438_d_n10, eq103_e1438_d_n11, eq103_e1438_d_n12, eq103_e1438_d_n13, eq103_e1438_d_n14, eq103_e1438_d_n15, eq103_e1438_d_n16, eq103_e1438_d_n17, eq103_e1438_d_n18, eq103_e1438_d_n19, eq103_e1438_d_n20, eq103_e1438_d_n21, eq103_e1438_d_n22, eq103_e1438_d_b0, eq103_e1438_d_b1, eq103_e1438_d_b2, eq103_e1438_d_b3, eq103_e1438_d_b4, eq103_e1438_d_b5, eq103_e1438_d_b6, eq103_e1438_d_b7, eq103_e1438_d_b8, eq103_e1438_d_b9, eq103_e1438_d_b10, eq103_e1438_d_b11, eq103_e1438_d_b12, eq103_e1438_d_b13, eq103_e1438_d_b14, eq103_e1438_d_b15, eq103_e1438_d_b16, eq103_e1438_d_b17, eq103_e1438_d_b18, eq103_e1438_d_b19, eq103_e1438_d_b20, eq103_e1438_d_b21, eq103_e1438_d_b22, eq103_e1438_d_b23, eq103_e1438_d_b24, eq103_e1438_d_b25, eq103_e1438_d_b26, eq103_e1438_d_b27, eq103_e1438_d_b28, eq103_e1438_d_b29, eq103_e1438_d_b30, eq103_e1438_d_b31, eq103_e1438_d_b32, eq103_e1438_d_b33, eq103_e1438_d_b34, eq103_e1438_d_b35, eq103_e1438_d_b36, eq103_e1438_d_b37, eq103_e1438_d_b38, eq103_e1438_d_b39, eq103_e1438_d_b40, eq103_e1438_d_b41, eq103_e1438_d_b42, eq103_e1438_d_b43, eq103_e1438_d_b44, eq103_e1438_d_b45, eq103_e1438_d_b46, eq103_e1438_d_b47, eq103_e1438_d_b48, eq103_e1438_d_b49, eq103_e1438_d_b50, eq103_e1438_d_b51, eq103_e1438_d_b52, eq103_e1438_d_b53, eq103_e1438_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_value: f64 = eq103_e1440;
        let eq103_node_derivatives: [f64; 23] = [eq103_e1440_d_n0, eq103_e1440_d_n1, eq103_e1440_d_n2, eq103_e1440_d_n3, eq103_e1440_d_n4, eq103_e1440_d_n5, eq103_e1440_d_n6, eq103_e1440_d_n7, eq103_e1440_d_n8, eq103_e1440_d_n9, eq103_e1440_d_n10, eq103_e1440_d_n11, eq103_e1440_d_n12, eq103_e1440_d_n13, eq103_e1440_d_n14, eq103_e1440_d_n15, eq103_e1440_d_n16, eq103_e1440_d_n17, eq103_e1440_d_n18, eq103_e1440_d_n19, eq103_e1440_d_n20, eq103_e1440_d_n21, eq103_e1440_d_n22];
        let eq103_branch_derivatives: [f64; 55] = [eq103_e1440_d_b0, eq103_e1440_d_b1, eq103_e1440_d_b2, eq103_e1440_d_b3, eq103_e1440_d_b4, eq103_e1440_d_b5, eq103_e1440_d_b6, eq103_e1440_d_b7, eq103_e1440_d_b8, eq103_e1440_d_b9, eq103_e1440_d_b10, eq103_e1440_d_b11, eq103_e1440_d_b12, eq103_e1440_d_b13, eq103_e1440_d_b14, eq103_e1440_d_b15, eq103_e1440_d_b16, eq103_e1440_d_b17, eq103_e1440_d_b18, eq103_e1440_d_b19, eq103_e1440_d_b20, eq103_e1440_d_b21, eq103_e1440_d_b22, eq103_e1440_d_b23, eq103_e1440_d_b24, eq103_e1440_d_b25, eq103_e1440_d_b26, eq103_e1440_d_b27, eq103_e1440_d_b28, eq103_e1440_d_b29, eq103_e1440_d_b30, eq103_e1440_d_b31, eq103_e1440_d_b32, eq103_e1440_d_b33, eq103_e1440_d_b34, eq103_e1440_d_b35, eq103_e1440_d_b36, eq103_e1440_d_b37, eq103_e1440_d_b38, eq103_e1440_d_b39, eq103_e1440_d_b40, eq103_e1440_d_b41, eq103_e1440_d_b42, eq103_e1440_d_b43, eq103_e1440_d_b44, eq103_e1440_d_b45, eq103_e1440_d_b46, eq103_e1440_d_b47, eq103_e1440_d_b48, eq103_e1440_d_b49, eq103_e1440_d_b50, eq103_e1440_d_b51, eq103_e1440_d_b52, eq103_e1440_d_b53, eq103_e1440_d_b54];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq103_value),
            &eq103_node_derivatives,
            &eq103_branch_derivatives,
            multiplicity,
        );
        let (eq104_e1448,) = {
    if ((!s.b[553]) && (!s.b[555])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq104_value: f64 = eq104_e1448;
        stamper.stamp_potential_const_local(
            54,
            eq104_value,
        );
        let (eq105_e1456,) = {
    if ((!s.b[553]) && (!s.b[555])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq105_value: f64 = eq105_e1456;
        stamper.stamp_potential_const_local(
            55,
            eq105_value,
        );
        let eq106_e1459: f64 = (p.p6 * s.v[369]);
        let eq106_e1459_d_n0: f64 = (p.p6 * s.dn[369][0]);
        let eq106_e1459_d_n1: f64 = (p.p6 * s.dn[369][1]);
        let eq106_e1459_d_n2: f64 = (p.p6 * s.dn[369][2]);
        let eq106_e1459_d_n3: f64 = (p.p6 * s.dn[369][3]);
        let eq106_e1459_d_n4: f64 = (p.p6 * s.dn[369][4]);
        let eq106_e1459_d_n5: f64 = (p.p6 * s.dn[369][5]);
        let eq106_e1459_d_n6: f64 = (p.p6 * s.dn[369][6]);
        let eq106_e1459_d_n7: f64 = (p.p6 * s.dn[369][7]);
        let eq106_e1459_d_n8: f64 = (p.p6 * s.dn[369][8]);
        let eq106_e1459_d_n9: f64 = (p.p6 * s.dn[369][9]);
        let eq106_e1459_d_n10: f64 = (p.p6 * s.dn[369][10]);
        let eq106_e1459_d_n11: f64 = (p.p6 * s.dn[369][11]);
        let eq106_e1459_d_n12: f64 = (p.p6 * s.dn[369][12]);
        let eq106_e1459_d_n13: f64 = (p.p6 * s.dn[369][13]);
        let eq106_e1459_d_n14: f64 = (p.p6 * s.dn[369][14]);
        let eq106_e1459_d_n15: f64 = (p.p6 * s.dn[369][15]);
        let eq106_e1459_d_n16: f64 = (p.p6 * s.dn[369][16]);
        let eq106_e1459_d_n17: f64 = (p.p6 * s.dn[369][17]);
        let eq106_e1459_d_n18: f64 = (p.p6 * s.dn[369][18]);
        let eq106_e1459_d_n19: f64 = (p.p6 * s.dn[369][19]);
        let eq106_e1459_d_n20: f64 = (p.p6 * s.dn[369][20]);
        let eq106_e1459_d_n21: f64 = (p.p6 * s.dn[369][21]);
        let eq106_e1459_d_n22: f64 = (p.p6 * s.dn[369][22]);
        let eq106_e1459_d_b0: f64 = (p.p6 * s.db[369][0]);
        let eq106_e1459_d_b1: f64 = (p.p6 * s.db[369][1]);
        let eq106_e1459_d_b2: f64 = (p.p6 * s.db[369][2]);
        let eq106_e1459_d_b3: f64 = (p.p6 * s.db[369][3]);
        let eq106_e1459_d_b4: f64 = (p.p6 * s.db[369][4]);
        let eq106_e1459_d_b5: f64 = (p.p6 * s.db[369][5]);
        let eq106_e1459_d_b6: f64 = (p.p6 * s.db[369][6]);
        let eq106_e1459_d_b7: f64 = (p.p6 * s.db[369][7]);
        let eq106_e1459_d_b8: f64 = (p.p6 * s.db[369][8]);
        let eq106_e1459_d_b9: f64 = (p.p6 * s.db[369][9]);
        let eq106_e1459_d_b10: f64 = (p.p6 * s.db[369][10]);
        let eq106_e1459_d_b11: f64 = (p.p6 * s.db[369][11]);
        let eq106_e1459_d_b12: f64 = (p.p6 * s.db[369][12]);
        let eq106_e1459_d_b13: f64 = (p.p6 * s.db[369][13]);
        let eq106_e1459_d_b14: f64 = (p.p6 * s.db[369][14]);
        let eq106_e1459_d_b15: f64 = (p.p6 * s.db[369][15]);
        let eq106_e1459_d_b16: f64 = (p.p6 * s.db[369][16]);
        let eq106_e1459_d_b17: f64 = (p.p6 * s.db[369][17]);
        let eq106_e1459_d_b18: f64 = (p.p6 * s.db[369][18]);
        let eq106_e1459_d_b19: f64 = (p.p6 * s.db[369][19]);
        let eq106_e1459_d_b20: f64 = (p.p6 * s.db[369][20]);
        let eq106_e1459_d_b21: f64 = (p.p6 * s.db[369][21]);
        let eq106_e1459_d_b22: f64 = (p.p6 * s.db[369][22]);
        let eq106_e1459_d_b23: f64 = (p.p6 * s.db[369][23]);
        let eq106_e1459_d_b24: f64 = (p.p6 * s.db[369][24]);
        let eq106_e1459_d_b25: f64 = (p.p6 * s.db[369][25]);
        let eq106_e1459_d_b26: f64 = (p.p6 * s.db[369][26]);
        let eq106_e1459_d_b27: f64 = (p.p6 * s.db[369][27]);
        let eq106_e1459_d_b28: f64 = (p.p6 * s.db[369][28]);
        let eq106_e1459_d_b29: f64 = (p.p6 * s.db[369][29]);
        let eq106_e1459_d_b30: f64 = (p.p6 * s.db[369][30]);
        let eq106_e1459_d_b31: f64 = (p.p6 * s.db[369][31]);
        let eq106_e1459_d_b32: f64 = (p.p6 * s.db[369][32]);
        let eq106_e1459_d_b33: f64 = (p.p6 * s.db[369][33]);
        let eq106_e1459_d_b34: f64 = (p.p6 * s.db[369][34]);
        let eq106_e1459_d_b35: f64 = (p.p6 * s.db[369][35]);
        let eq106_e1459_d_b36: f64 = (p.p6 * s.db[369][36]);
        let eq106_e1459_d_b37: f64 = (p.p6 * s.db[369][37]);
        let eq106_e1459_d_b38: f64 = (p.p6 * s.db[369][38]);
        let eq106_e1459_d_b39: f64 = (p.p6 * s.db[369][39]);
        let eq106_e1459_d_b40: f64 = (p.p6 * s.db[369][40]);
        let eq106_e1459_d_b41: f64 = (p.p6 * s.db[369][41]);
        let eq106_e1459_d_b42: f64 = (p.p6 * s.db[369][42]);
        let eq106_e1459_d_b43: f64 = (p.p6 * s.db[369][43]);
        let eq106_e1459_d_b44: f64 = (p.p6 * s.db[369][44]);
        let eq106_e1459_d_b45: f64 = (p.p6 * s.db[369][45]);
        let eq106_e1459_d_b46: f64 = (p.p6 * s.db[369][46]);
        let eq106_e1459_d_b47: f64 = (p.p6 * s.db[369][47]);
        let eq106_e1459_d_b48: f64 = (p.p6 * s.db[369][48]);
        let eq106_e1459_d_b49: f64 = (p.p6 * s.db[369][49]);
        let eq106_e1459_d_b50: f64 = (p.p6 * s.db[369][50]);
        let eq106_e1459_d_b51: f64 = (p.p6 * s.db[369][51]);
        let eq106_e1459_d_b52: f64 = (p.p6 * s.db[369][52]);
        let eq106_e1459_d_b53: f64 = (p.p6 * s.db[369][53]);
        let eq106_e1459_d_b54: f64 = (p.p6 * s.db[369][54]);
        let eq106_value: f64 = eq106_e1459;
        let eq106_node_derivatives: [f64; 23] = [eq106_e1459_d_n0, eq106_e1459_d_n1, eq106_e1459_d_n2, eq106_e1459_d_n3, eq106_e1459_d_n4, eq106_e1459_d_n5, eq106_e1459_d_n6, eq106_e1459_d_n7, eq106_e1459_d_n8, eq106_e1459_d_n9, eq106_e1459_d_n10, eq106_e1459_d_n11, eq106_e1459_d_n12, eq106_e1459_d_n13, eq106_e1459_d_n14, eq106_e1459_d_n15, eq106_e1459_d_n16, eq106_e1459_d_n17, eq106_e1459_d_n18, eq106_e1459_d_n19, eq106_e1459_d_n20, eq106_e1459_d_n21, eq106_e1459_d_n22];
        let eq106_branch_derivatives: [f64; 55] = [eq106_e1459_d_b0, eq106_e1459_d_b1, eq106_e1459_d_b2, eq106_e1459_d_b3, eq106_e1459_d_b4, eq106_e1459_d_b5, eq106_e1459_d_b6, eq106_e1459_d_b7, eq106_e1459_d_b8, eq106_e1459_d_b9, eq106_e1459_d_b10, eq106_e1459_d_b11, eq106_e1459_d_b12, eq106_e1459_d_b13, eq106_e1459_d_b14, eq106_e1459_d_b15, eq106_e1459_d_b16, eq106_e1459_d_b17, eq106_e1459_d_b18, eq106_e1459_d_b19, eq106_e1459_d_b20, eq106_e1459_d_b21, eq106_e1459_d_b22, eq106_e1459_d_b23, eq106_e1459_d_b24, eq106_e1459_d_b25, eq106_e1459_d_b26, eq106_e1459_d_b27, eq106_e1459_d_b28, eq106_e1459_d_b29, eq106_e1459_d_b30, eq106_e1459_d_b31, eq106_e1459_d_b32, eq106_e1459_d_b33, eq106_e1459_d_b34, eq106_e1459_d_b35, eq106_e1459_d_b36, eq106_e1459_d_b37, eq106_e1459_d_b38, eq106_e1459_d_b39, eq106_e1459_d_b40, eq106_e1459_d_b41, eq106_e1459_d_b42, eq106_e1459_d_b43, eq106_e1459_d_b44, eq106_e1459_d_b45, eq106_e1459_d_b46, eq106_e1459_d_b47, eq106_e1459_d_b48, eq106_e1459_d_b49, eq106_e1459_d_b50, eq106_e1459_d_b51, eq106_e1459_d_b52, eq106_e1459_d_b53, eq106_e1459_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * (eq106_value),
            &eq106_node_derivatives,
            &eq106_branch_derivatives,
            multiplicity,
        );
        let eq107_e1462: f64 = (p.p6 * s.v[370]);
        let eq107_e1462_d_n0: f64 = (p.p6 * s.dn[370][0]);
        let eq107_e1462_d_n1: f64 = (p.p6 * s.dn[370][1]);
        let eq107_e1462_d_n2: f64 = (p.p6 * s.dn[370][2]);
        let eq107_e1462_d_n3: f64 = (p.p6 * s.dn[370][3]);
        let eq107_e1462_d_n4: f64 = (p.p6 * s.dn[370][4]);
        let eq107_e1462_d_n5: f64 = (p.p6 * s.dn[370][5]);
        let eq107_e1462_d_n6: f64 = (p.p6 * s.dn[370][6]);
        let eq107_e1462_d_n7: f64 = (p.p6 * s.dn[370][7]);
        let eq107_e1462_d_n8: f64 = (p.p6 * s.dn[370][8]);
        let eq107_e1462_d_n9: f64 = (p.p6 * s.dn[370][9]);
        let eq107_e1462_d_n10: f64 = (p.p6 * s.dn[370][10]);
        let eq107_e1462_d_n11: f64 = (p.p6 * s.dn[370][11]);
        let eq107_e1462_d_n12: f64 = (p.p6 * s.dn[370][12]);
        let eq107_e1462_d_n13: f64 = (p.p6 * s.dn[370][13]);
        let eq107_e1462_d_n14: f64 = (p.p6 * s.dn[370][14]);
        let eq107_e1462_d_n15: f64 = (p.p6 * s.dn[370][15]);
        let eq107_e1462_d_n16: f64 = (p.p6 * s.dn[370][16]);
        let eq107_e1462_d_n17: f64 = (p.p6 * s.dn[370][17]);
        let eq107_e1462_d_n18: f64 = (p.p6 * s.dn[370][18]);
        let eq107_e1462_d_n19: f64 = (p.p6 * s.dn[370][19]);
        let eq107_e1462_d_n20: f64 = (p.p6 * s.dn[370][20]);
        let eq107_e1462_d_n21: f64 = (p.p6 * s.dn[370][21]);
        let eq107_e1462_d_n22: f64 = (p.p6 * s.dn[370][22]);
        let eq107_e1462_d_b0: f64 = (p.p6 * s.db[370][0]);
        let eq107_e1462_d_b1: f64 = (p.p6 * s.db[370][1]);
        let eq107_e1462_d_b2: f64 = (p.p6 * s.db[370][2]);
        let eq107_e1462_d_b3: f64 = (p.p6 * s.db[370][3]);
        let eq107_e1462_d_b4: f64 = (p.p6 * s.db[370][4]);
        let eq107_e1462_d_b5: f64 = (p.p6 * s.db[370][5]);
        let eq107_e1462_d_b6: f64 = (p.p6 * s.db[370][6]);
        let eq107_e1462_d_b7: f64 = (p.p6 * s.db[370][7]);
        let eq107_e1462_d_b8: f64 = (p.p6 * s.db[370][8]);
        let eq107_e1462_d_b9: f64 = (p.p6 * s.db[370][9]);
        let eq107_e1462_d_b10: f64 = (p.p6 * s.db[370][10]);
        let eq107_e1462_d_b11: f64 = (p.p6 * s.db[370][11]);
        let eq107_e1462_d_b12: f64 = (p.p6 * s.db[370][12]);
        let eq107_e1462_d_b13: f64 = (p.p6 * s.db[370][13]);
        let eq107_e1462_d_b14: f64 = (p.p6 * s.db[370][14]);
        let eq107_e1462_d_b15: f64 = (p.p6 * s.db[370][15]);
        let eq107_e1462_d_b16: f64 = (p.p6 * s.db[370][16]);
        let eq107_e1462_d_b17: f64 = (p.p6 * s.db[370][17]);
        let eq107_e1462_d_b18: f64 = (p.p6 * s.db[370][18]);
        let eq107_e1462_d_b19: f64 = (p.p6 * s.db[370][19]);
        let eq107_e1462_d_b20: f64 = (p.p6 * s.db[370][20]);
        let eq107_e1462_d_b21: f64 = (p.p6 * s.db[370][21]);
        let eq107_e1462_d_b22: f64 = (p.p6 * s.db[370][22]);
        let eq107_e1462_d_b23: f64 = (p.p6 * s.db[370][23]);
        let eq107_e1462_d_b24: f64 = (p.p6 * s.db[370][24]);
        let eq107_e1462_d_b25: f64 = (p.p6 * s.db[370][25]);
        let eq107_e1462_d_b26: f64 = (p.p6 * s.db[370][26]);
        let eq107_e1462_d_b27: f64 = (p.p6 * s.db[370][27]);
        let eq107_e1462_d_b28: f64 = (p.p6 * s.db[370][28]);
        let eq107_e1462_d_b29: f64 = (p.p6 * s.db[370][29]);
        let eq107_e1462_d_b30: f64 = (p.p6 * s.db[370][30]);
        let eq107_e1462_d_b31: f64 = (p.p6 * s.db[370][31]);
        let eq107_e1462_d_b32: f64 = (p.p6 * s.db[370][32]);
        let eq107_e1462_d_b33: f64 = (p.p6 * s.db[370][33]);
        let eq107_e1462_d_b34: f64 = (p.p6 * s.db[370][34]);
        let eq107_e1462_d_b35: f64 = (p.p6 * s.db[370][35]);
        let eq107_e1462_d_b36: f64 = (p.p6 * s.db[370][36]);
        let eq107_e1462_d_b37: f64 = (p.p6 * s.db[370][37]);
        let eq107_e1462_d_b38: f64 = (p.p6 * s.db[370][38]);
        let eq107_e1462_d_b39: f64 = (p.p6 * s.db[370][39]);
        let eq107_e1462_d_b40: f64 = (p.p6 * s.db[370][40]);
        let eq107_e1462_d_b41: f64 = (p.p6 * s.db[370][41]);
        let eq107_e1462_d_b42: f64 = (p.p6 * s.db[370][42]);
        let eq107_e1462_d_b43: f64 = (p.p6 * s.db[370][43]);
        let eq107_e1462_d_b44: f64 = (p.p6 * s.db[370][44]);
        let eq107_e1462_d_b45: f64 = (p.p6 * s.db[370][45]);
        let eq107_e1462_d_b46: f64 = (p.p6 * s.db[370][46]);
        let eq107_e1462_d_b47: f64 = (p.p6 * s.db[370][47]);
        let eq107_e1462_d_b48: f64 = (p.p6 * s.db[370][48]);
        let eq107_e1462_d_b49: f64 = (p.p6 * s.db[370][49]);
        let eq107_e1462_d_b50: f64 = (p.p6 * s.db[370][50]);
        let eq107_e1462_d_b51: f64 = (p.p6 * s.db[370][51]);
        let eq107_e1462_d_b52: f64 = (p.p6 * s.db[370][52]);
        let eq107_e1462_d_b53: f64 = (p.p6 * s.db[370][53]);
        let eq107_e1462_d_b54: f64 = (p.p6 * s.db[370][54]);
        let eq107_value: f64 = eq107_e1462;
        let eq107_node_derivatives: [f64; 23] = [eq107_e1462_d_n0, eq107_e1462_d_n1, eq107_e1462_d_n2, eq107_e1462_d_n3, eq107_e1462_d_n4, eq107_e1462_d_n5, eq107_e1462_d_n6, eq107_e1462_d_n7, eq107_e1462_d_n8, eq107_e1462_d_n9, eq107_e1462_d_n10, eq107_e1462_d_n11, eq107_e1462_d_n12, eq107_e1462_d_n13, eq107_e1462_d_n14, eq107_e1462_d_n15, eq107_e1462_d_n16, eq107_e1462_d_n17, eq107_e1462_d_n18, eq107_e1462_d_n19, eq107_e1462_d_n20, eq107_e1462_d_n21, eq107_e1462_d_n22];
        let eq107_branch_derivatives: [f64; 55] = [eq107_e1462_d_b0, eq107_e1462_d_b1, eq107_e1462_d_b2, eq107_e1462_d_b3, eq107_e1462_d_b4, eq107_e1462_d_b5, eq107_e1462_d_b6, eq107_e1462_d_b7, eq107_e1462_d_b8, eq107_e1462_d_b9, eq107_e1462_d_b10, eq107_e1462_d_b11, eq107_e1462_d_b12, eq107_e1462_d_b13, eq107_e1462_d_b14, eq107_e1462_d_b15, eq107_e1462_d_b16, eq107_e1462_d_b17, eq107_e1462_d_b18, eq107_e1462_d_b19, eq107_e1462_d_b20, eq107_e1462_d_b21, eq107_e1462_d_b22, eq107_e1462_d_b23, eq107_e1462_d_b24, eq107_e1462_d_b25, eq107_e1462_d_b26, eq107_e1462_d_b27, eq107_e1462_d_b28, eq107_e1462_d_b29, eq107_e1462_d_b30, eq107_e1462_d_b31, eq107_e1462_d_b32, eq107_e1462_d_b33, eq107_e1462_d_b34, eq107_e1462_d_b35, eq107_e1462_d_b36, eq107_e1462_d_b37, eq107_e1462_d_b38, eq107_e1462_d_b39, eq107_e1462_d_b40, eq107_e1462_d_b41, eq107_e1462_d_b42, eq107_e1462_d_b43, eq107_e1462_d_b44, eq107_e1462_d_b45, eq107_e1462_d_b46, eq107_e1462_d_b47, eq107_e1462_d_b48, eq107_e1462_d_b49, eq107_e1462_d_b50, eq107_e1462_d_b51, eq107_e1462_d_b52, eq107_e1462_d_b53, eq107_e1462_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * (eq107_value),
            &eq107_node_derivatives,
            &eq107_branch_derivatives,
            multiplicity,
        );
        let (eq108_e1471,) = {
    if s.b[567] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq108_value: f64 = eq108_e1471;
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (eq108_value),
        );
    }
}
