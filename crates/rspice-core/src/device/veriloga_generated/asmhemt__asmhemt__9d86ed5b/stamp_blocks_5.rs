#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_19(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq130_e1665, eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22, eq130_e1665_d_b0, eq130_e1665_d_b1, eq130_e1665_d_b2, eq130_e1665_d_b3, eq130_e1665_d_b4, eq130_e1665_d_b5, eq130_e1665_d_b6, eq130_e1665_d_b7, eq130_e1665_d_b8, eq130_e1665_d_b9, eq130_e1665_d_b10, eq130_e1665_d_b11, eq130_e1665_d_b12, eq130_e1665_d_b13, eq130_e1665_d_b14, eq130_e1665_d_b15, eq130_e1665_d_b16, eq130_e1665_d_b17, eq130_e1665_d_b18, eq130_e1665_d_b19, eq130_e1665_d_b20, eq130_e1665_d_b21, eq130_e1665_d_b22, eq130_e1665_d_b23, eq130_e1665_d_b24, eq130_e1665_d_b25, eq130_e1665_d_b26, eq130_e1665_d_b27, eq130_e1665_d_b28, eq130_e1665_d_b29, eq130_e1665_d_b30, eq130_e1665_d_b31, eq130_e1665_d_b32, eq130_e1665_d_b33, eq130_e1665_d_b34, eq130_e1665_d_b35, eq130_e1665_d_b36, eq130_e1665_d_b37, eq130_e1665_d_b38, eq130_e1665_d_b39, eq130_e1665_d_b40, eq130_e1665_d_b41, eq130_e1665_d_b42, eq130_e1665_d_b43, eq130_e1665_d_b44, eq130_e1665_d_b45, eq130_e1665_d_b46, eq130_e1665_d_b47, eq130_e1665_d_b48, eq130_e1665_d_b49, eq130_e1665_d_b50, eq130_e1665_d_b51, eq130_e1665_d_b52, eq130_e1665_d_b53, eq130_e1665_d_b54,) = {
    if (((!s.b[570]) && s.b[573]) && (!s.b[574])) {
        let eq130_e1660: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 29, s.v[228]);
        let eq130_e1661: f64 = (p.p7 * eq130_e1660);
        let eq130_e1661_d_n0: f64 = (p.p7 * (s.dn[228][0] * ddt_scale));
        let eq130_e1661_d_n1: f64 = (p.p7 * (s.dn[228][1] * ddt_scale));
        let eq130_e1661_d_n2: f64 = (p.p7 * (s.dn[228][2] * ddt_scale));
        let eq130_e1661_d_n3: f64 = (p.p7 * (s.dn[228][3] * ddt_scale));
        let eq130_e1661_d_n4: f64 = (p.p7 * (s.dn[228][4] * ddt_scale));
        let eq130_e1661_d_n5: f64 = (p.p7 * (s.dn[228][5] * ddt_scale));
        let eq130_e1661_d_n6: f64 = (p.p7 * (s.dn[228][6] * ddt_scale));
        let eq130_e1661_d_n7: f64 = (p.p7 * (s.dn[228][7] * ddt_scale));
        let eq130_e1661_d_n8: f64 = (p.p7 * (s.dn[228][8] * ddt_scale));
        let eq130_e1661_d_n9: f64 = (p.p7 * (s.dn[228][9] * ddt_scale));
        let eq130_e1661_d_n10: f64 = (p.p7 * (s.dn[228][10] * ddt_scale));
        let eq130_e1661_d_n11: f64 = (p.p7 * (s.dn[228][11] * ddt_scale));
        let eq130_e1661_d_n12: f64 = (p.p7 * (s.dn[228][12] * ddt_scale));
        let eq130_e1661_d_n13: f64 = (p.p7 * (s.dn[228][13] * ddt_scale));
        let eq130_e1661_d_n14: f64 = (p.p7 * (s.dn[228][14] * ddt_scale));
        let eq130_e1661_d_n15: f64 = (p.p7 * (s.dn[228][15] * ddt_scale));
        let eq130_e1661_d_n16: f64 = (p.p7 * (s.dn[228][16] * ddt_scale));
        let eq130_e1661_d_n17: f64 = (p.p7 * (s.dn[228][17] * ddt_scale));
        let eq130_e1661_d_n18: f64 = (p.p7 * (s.dn[228][18] * ddt_scale));
        let eq130_e1661_d_n19: f64 = (p.p7 * (s.dn[228][19] * ddt_scale));
        let eq130_e1661_d_n20: f64 = (p.p7 * (s.dn[228][20] * ddt_scale));
        let eq130_e1661_d_n21: f64 = (p.p7 * (s.dn[228][21] * ddt_scale));
        let eq130_e1661_d_n22: f64 = (p.p7 * (s.dn[228][22] * ddt_scale));
        let eq130_e1661_d_b0: f64 = (p.p7 * (s.db[228][0] * ddt_scale));
        let eq130_e1661_d_b1: f64 = (p.p7 * (s.db[228][1] * ddt_scale));
        let eq130_e1661_d_b2: f64 = (p.p7 * (s.db[228][2] * ddt_scale));
        let eq130_e1661_d_b3: f64 = (p.p7 * (s.db[228][3] * ddt_scale));
        let eq130_e1661_d_b4: f64 = (p.p7 * (s.db[228][4] * ddt_scale));
        let eq130_e1661_d_b5: f64 = (p.p7 * (s.db[228][5] * ddt_scale));
        let eq130_e1661_d_b6: f64 = (p.p7 * (s.db[228][6] * ddt_scale));
        let eq130_e1661_d_b7: f64 = (p.p7 * (s.db[228][7] * ddt_scale));
        let eq130_e1661_d_b8: f64 = (p.p7 * (s.db[228][8] * ddt_scale));
        let eq130_e1661_d_b9: f64 = (p.p7 * (s.db[228][9] * ddt_scale));
        let eq130_e1661_d_b10: f64 = (p.p7 * (s.db[228][10] * ddt_scale));
        let eq130_e1661_d_b11: f64 = (p.p7 * (s.db[228][11] * ddt_scale));
        let eq130_e1661_d_b12: f64 = (p.p7 * (s.db[228][12] * ddt_scale));
        let eq130_e1661_d_b13: f64 = (p.p7 * (s.db[228][13] * ddt_scale));
        let eq130_e1661_d_b14: f64 = (p.p7 * (s.db[228][14] * ddt_scale));
        let eq130_e1661_d_b15: f64 = (p.p7 * (s.db[228][15] * ddt_scale));
        let eq130_e1661_d_b16: f64 = (p.p7 * (s.db[228][16] * ddt_scale));
        let eq130_e1661_d_b17: f64 = (p.p7 * (s.db[228][17] * ddt_scale));
        let eq130_e1661_d_b18: f64 = (p.p7 * (s.db[228][18] * ddt_scale));
        let eq130_e1661_d_b19: f64 = (p.p7 * (s.db[228][19] * ddt_scale));
        let eq130_e1661_d_b20: f64 = (p.p7 * (s.db[228][20] * ddt_scale));
        let eq130_e1661_d_b21: f64 = (p.p7 * (s.db[228][21] * ddt_scale));
        let eq130_e1661_d_b22: f64 = (p.p7 * (s.db[228][22] * ddt_scale));
        let eq130_e1661_d_b23: f64 = (p.p7 * (s.db[228][23] * ddt_scale));
        let eq130_e1661_d_b24: f64 = (p.p7 * (s.db[228][24] * ddt_scale));
        let eq130_e1661_d_b25: f64 = (p.p7 * (s.db[228][25] * ddt_scale));
        let eq130_e1661_d_b26: f64 = (p.p7 * (s.db[228][26] * ddt_scale));
        let eq130_e1661_d_b27: f64 = (p.p7 * (s.db[228][27] * ddt_scale));
        let eq130_e1661_d_b28: f64 = (p.p7 * (s.db[228][28] * ddt_scale));
        let eq130_e1661_d_b29: f64 = (p.p7 * (s.db[228][29] * ddt_scale));
        let eq130_e1661_d_b30: f64 = (p.p7 * (s.db[228][30] * ddt_scale));
        let eq130_e1661_d_b31: f64 = (p.p7 * (s.db[228][31] * ddt_scale));
        let eq130_e1661_d_b32: f64 = (p.p7 * (s.db[228][32] * ddt_scale));
        let eq130_e1661_d_b33: f64 = (p.p7 * (s.db[228][33] * ddt_scale));
        let eq130_e1661_d_b34: f64 = (p.p7 * (s.db[228][34] * ddt_scale));
        let eq130_e1661_d_b35: f64 = (p.p7 * (s.db[228][35] * ddt_scale));
        let eq130_e1661_d_b36: f64 = (p.p7 * (s.db[228][36] * ddt_scale));
        let eq130_e1661_d_b37: f64 = (p.p7 * (s.db[228][37] * ddt_scale));
        let eq130_e1661_d_b38: f64 = (p.p7 * (s.db[228][38] * ddt_scale));
        let eq130_e1661_d_b39: f64 = (p.p7 * (s.db[228][39] * ddt_scale));
        let eq130_e1661_d_b40: f64 = (p.p7 * (s.db[228][40] * ddt_scale));
        let eq130_e1661_d_b41: f64 = (p.p7 * (s.db[228][41] * ddt_scale));
        let eq130_e1661_d_b42: f64 = (p.p7 * (s.db[228][42] * ddt_scale));
        let eq130_e1661_d_b43: f64 = (p.p7 * (s.db[228][43] * ddt_scale));
        let eq130_e1661_d_b44: f64 = (p.p7 * (s.db[228][44] * ddt_scale));
        let eq130_e1661_d_b45: f64 = (p.p7 * (s.db[228][45] * ddt_scale));
        let eq130_e1661_d_b46: f64 = (p.p7 * (s.db[228][46] * ddt_scale));
        let eq130_e1661_d_b47: f64 = (p.p7 * (s.db[228][47] * ddt_scale));
        let eq130_e1661_d_b48: f64 = (p.p7 * (s.db[228][48] * ddt_scale));
        let eq130_e1661_d_b49: f64 = (p.p7 * (s.db[228][49] * ddt_scale));
        let eq130_e1661_d_b50: f64 = (p.p7 * (s.db[228][50] * ddt_scale));
        let eq130_e1661_d_b51: f64 = (p.p7 * (s.db[228][51] * ddt_scale));
        let eq130_e1661_d_b52: f64 = (p.p7 * (s.db[228][52] * ddt_scale));
        let eq130_e1661_d_b53: f64 = (p.p7 * (s.db[228][53] * ddt_scale));
        let eq130_e1661_d_b54: f64 = (p.p7 * (s.db[228][54] * ddt_scale));
        let eq130_e1663: f64 = (eq130_e1661 * p.p246);
        let eq130_e1663_d_n0: f64 = (eq130_e1661_d_n0 * p.p246);
        let eq130_e1663_d_n1: f64 = (eq130_e1661_d_n1 * p.p246);
        let eq130_e1663_d_n2: f64 = (eq130_e1661_d_n2 * p.p246);
        let eq130_e1663_d_n3: f64 = (eq130_e1661_d_n3 * p.p246);
        let eq130_e1663_d_n4: f64 = (eq130_e1661_d_n4 * p.p246);
        let eq130_e1663_d_n5: f64 = (eq130_e1661_d_n5 * p.p246);
        let eq130_e1663_d_n6: f64 = (eq130_e1661_d_n6 * p.p246);
        let eq130_e1663_d_n7: f64 = (eq130_e1661_d_n7 * p.p246);
        let eq130_e1663_d_n8: f64 = (eq130_e1661_d_n8 * p.p246);
        let eq130_e1663_d_n9: f64 = (eq130_e1661_d_n9 * p.p246);
        let eq130_e1663_d_n10: f64 = (eq130_e1661_d_n10 * p.p246);
        let eq130_e1663_d_n11: f64 = (eq130_e1661_d_n11 * p.p246);
        let eq130_e1663_d_n12: f64 = (eq130_e1661_d_n12 * p.p246);
        let eq130_e1663_d_n13: f64 = (eq130_e1661_d_n13 * p.p246);
        let eq130_e1663_d_n14: f64 = (eq130_e1661_d_n14 * p.p246);
        let eq130_e1663_d_n15: f64 = (eq130_e1661_d_n15 * p.p246);
        let eq130_e1663_d_n16: f64 = (eq130_e1661_d_n16 * p.p246);
        let eq130_e1663_d_n17: f64 = (eq130_e1661_d_n17 * p.p246);
        let eq130_e1663_d_n18: f64 = (eq130_e1661_d_n18 * p.p246);
        let eq130_e1663_d_n19: f64 = (eq130_e1661_d_n19 * p.p246);
        let eq130_e1663_d_n20: f64 = (eq130_e1661_d_n20 * p.p246);
        let eq130_e1663_d_n21: f64 = (eq130_e1661_d_n21 * p.p246);
        let eq130_e1663_d_n22: f64 = (eq130_e1661_d_n22 * p.p246);
        let eq130_e1663_d_b0: f64 = (eq130_e1661_d_b0 * p.p246);
        let eq130_e1663_d_b1: f64 = (eq130_e1661_d_b1 * p.p246);
        let eq130_e1663_d_b2: f64 = (eq130_e1661_d_b2 * p.p246);
        let eq130_e1663_d_b3: f64 = (eq130_e1661_d_b3 * p.p246);
        let eq130_e1663_d_b4: f64 = (eq130_e1661_d_b4 * p.p246);
        let eq130_e1663_d_b5: f64 = (eq130_e1661_d_b5 * p.p246);
        let eq130_e1663_d_b6: f64 = (eq130_e1661_d_b6 * p.p246);
        let eq130_e1663_d_b7: f64 = (eq130_e1661_d_b7 * p.p246);
        let eq130_e1663_d_b8: f64 = (eq130_e1661_d_b8 * p.p246);
        let eq130_e1663_d_b9: f64 = (eq130_e1661_d_b9 * p.p246);
        let eq130_e1663_d_b10: f64 = (eq130_e1661_d_b10 * p.p246);
        let eq130_e1663_d_b11: f64 = (eq130_e1661_d_b11 * p.p246);
        let eq130_e1663_d_b12: f64 = (eq130_e1661_d_b12 * p.p246);
        let eq130_e1663_d_b13: f64 = (eq130_e1661_d_b13 * p.p246);
        let eq130_e1663_d_b14: f64 = (eq130_e1661_d_b14 * p.p246);
        let eq130_e1663_d_b15: f64 = (eq130_e1661_d_b15 * p.p246);
        let eq130_e1663_d_b16: f64 = (eq130_e1661_d_b16 * p.p246);
        let eq130_e1663_d_b17: f64 = (eq130_e1661_d_b17 * p.p246);
        let eq130_e1663_d_b18: f64 = (eq130_e1661_d_b18 * p.p246);
        let eq130_e1663_d_b19: f64 = (eq130_e1661_d_b19 * p.p246);
        let eq130_e1663_d_b20: f64 = (eq130_e1661_d_b20 * p.p246);
        let eq130_e1663_d_b21: f64 = (eq130_e1661_d_b21 * p.p246);
        let eq130_e1663_d_b22: f64 = (eq130_e1661_d_b22 * p.p246);
        let eq130_e1663_d_b23: f64 = (eq130_e1661_d_b23 * p.p246);
        let eq130_e1663_d_b24: f64 = (eq130_e1661_d_b24 * p.p246);
        let eq130_e1663_d_b25: f64 = (eq130_e1661_d_b25 * p.p246);
        let eq130_e1663_d_b26: f64 = (eq130_e1661_d_b26 * p.p246);
        let eq130_e1663_d_b27: f64 = (eq130_e1661_d_b27 * p.p246);
        let eq130_e1663_d_b28: f64 = (eq130_e1661_d_b28 * p.p246);
        let eq130_e1663_d_b29: f64 = (eq130_e1661_d_b29 * p.p246);
        let eq130_e1663_d_b30: f64 = (eq130_e1661_d_b30 * p.p246);
        let eq130_e1663_d_b31: f64 = (eq130_e1661_d_b31 * p.p246);
        let eq130_e1663_d_b32: f64 = (eq130_e1661_d_b32 * p.p246);
        let eq130_e1663_d_b33: f64 = (eq130_e1661_d_b33 * p.p246);
        let eq130_e1663_d_b34: f64 = (eq130_e1661_d_b34 * p.p246);
        let eq130_e1663_d_b35: f64 = (eq130_e1661_d_b35 * p.p246);
        let eq130_e1663_d_b36: f64 = (eq130_e1661_d_b36 * p.p246);
        let eq130_e1663_d_b37: f64 = (eq130_e1661_d_b37 * p.p246);
        let eq130_e1663_d_b38: f64 = (eq130_e1661_d_b38 * p.p246);
        let eq130_e1663_d_b39: f64 = (eq130_e1661_d_b39 * p.p246);
        let eq130_e1663_d_b40: f64 = (eq130_e1661_d_b40 * p.p246);
        let eq130_e1663_d_b41: f64 = (eq130_e1661_d_b41 * p.p246);
        let eq130_e1663_d_b42: f64 = (eq130_e1661_d_b42 * p.p246);
        let eq130_e1663_d_b43: f64 = (eq130_e1661_d_b43 * p.p246);
        let eq130_e1663_d_b44: f64 = (eq130_e1661_d_b44 * p.p246);
        let eq130_e1663_d_b45: f64 = (eq130_e1661_d_b45 * p.p246);
        let eq130_e1663_d_b46: f64 = (eq130_e1661_d_b46 * p.p246);
        let eq130_e1663_d_b47: f64 = (eq130_e1661_d_b47 * p.p246);
        let eq130_e1663_d_b48: f64 = (eq130_e1661_d_b48 * p.p246);
        let eq130_e1663_d_b49: f64 = (eq130_e1661_d_b49 * p.p246);
        let eq130_e1663_d_b50: f64 = (eq130_e1661_d_b50 * p.p246);
        let eq130_e1663_d_b51: f64 = (eq130_e1661_d_b51 * p.p246);
        let eq130_e1663_d_b52: f64 = (eq130_e1661_d_b52 * p.p246);
        let eq130_e1663_d_b53: f64 = (eq130_e1661_d_b53 * p.p246);
        let eq130_e1663_d_b54: f64 = (eq130_e1661_d_b54 * p.p246);
        (eq130_e1663, eq130_e1663_d_n0, eq130_e1663_d_n1, eq130_e1663_d_n2, eq130_e1663_d_n3, eq130_e1663_d_n4, eq130_e1663_d_n5, eq130_e1663_d_n6, eq130_e1663_d_n7, eq130_e1663_d_n8, eq130_e1663_d_n9, eq130_e1663_d_n10, eq130_e1663_d_n11, eq130_e1663_d_n12, eq130_e1663_d_n13, eq130_e1663_d_n14, eq130_e1663_d_n15, eq130_e1663_d_n16, eq130_e1663_d_n17, eq130_e1663_d_n18, eq130_e1663_d_n19, eq130_e1663_d_n20, eq130_e1663_d_n21, eq130_e1663_d_n22, eq130_e1663_d_b0, eq130_e1663_d_b1, eq130_e1663_d_b2, eq130_e1663_d_b3, eq130_e1663_d_b4, eq130_e1663_d_b5, eq130_e1663_d_b6, eq130_e1663_d_b7, eq130_e1663_d_b8, eq130_e1663_d_b9, eq130_e1663_d_b10, eq130_e1663_d_b11, eq130_e1663_d_b12, eq130_e1663_d_b13, eq130_e1663_d_b14, eq130_e1663_d_b15, eq130_e1663_d_b16, eq130_e1663_d_b17, eq130_e1663_d_b18, eq130_e1663_d_b19, eq130_e1663_d_b20, eq130_e1663_d_b21, eq130_e1663_d_b22, eq130_e1663_d_b23, eq130_e1663_d_b24, eq130_e1663_d_b25, eq130_e1663_d_b26, eq130_e1663_d_b27, eq130_e1663_d_b28, eq130_e1663_d_b29, eq130_e1663_d_b30, eq130_e1663_d_b31, eq130_e1663_d_b32, eq130_e1663_d_b33, eq130_e1663_d_b34, eq130_e1663_d_b35, eq130_e1663_d_b36, eq130_e1663_d_b37, eq130_e1663_d_b38, eq130_e1663_d_b39, eq130_e1663_d_b40, eq130_e1663_d_b41, eq130_e1663_d_b42, eq130_e1663_d_b43, eq130_e1663_d_b44, eq130_e1663_d_b45, eq130_e1663_d_b46, eq130_e1663_d_b47, eq130_e1663_d_b48, eq130_e1663_d_b49, eq130_e1663_d_b50, eq130_e1663_d_b51, eq130_e1663_d_b52, eq130_e1663_d_b53, eq130_e1663_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_value: f64 = eq130_e1665;
        let eq130_node_derivatives: [f64; 23] = [eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22];
        let eq130_branch_derivatives: [f64; 55] = [eq130_e1665_d_b0, eq130_e1665_d_b1, eq130_e1665_d_b2, eq130_e1665_d_b3, eq130_e1665_d_b4, eq130_e1665_d_b5, eq130_e1665_d_b6, eq130_e1665_d_b7, eq130_e1665_d_b8, eq130_e1665_d_b9, eq130_e1665_d_b10, eq130_e1665_d_b11, eq130_e1665_d_b12, eq130_e1665_d_b13, eq130_e1665_d_b14, eq130_e1665_d_b15, eq130_e1665_d_b16, eq130_e1665_d_b17, eq130_e1665_d_b18, eq130_e1665_d_b19, eq130_e1665_d_b20, eq130_e1665_d_b21, eq130_e1665_d_b22, eq130_e1665_d_b23, eq130_e1665_d_b24, eq130_e1665_d_b25, eq130_e1665_d_b26, eq130_e1665_d_b27, eq130_e1665_d_b28, eq130_e1665_d_b29, eq130_e1665_d_b30, eq130_e1665_d_b31, eq130_e1665_d_b32, eq130_e1665_d_b33, eq130_e1665_d_b34, eq130_e1665_d_b35, eq130_e1665_d_b36, eq130_e1665_d_b37, eq130_e1665_d_b38, eq130_e1665_d_b39, eq130_e1665_d_b40, eq130_e1665_d_b41, eq130_e1665_d_b42, eq130_e1665_d_b43, eq130_e1665_d_b44, eq130_e1665_d_b45, eq130_e1665_d_b46, eq130_e1665_d_b47, eq130_e1665_d_b48, eq130_e1665_d_b49, eq130_e1665_d_b50, eq130_e1665_d_b51, eq130_e1665_d_b52, eq130_e1665_d_b53, eq130_e1665_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq130_value),
            &eq130_node_derivatives,
            &eq130_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1677, eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22, eq131_e1677_d_b0, eq131_e1677_d_b1, eq131_e1677_d_b2, eq131_e1677_d_b3, eq131_e1677_d_b4, eq131_e1677_d_b5, eq131_e1677_d_b6, eq131_e1677_d_b7, eq131_e1677_d_b8, eq131_e1677_d_b9, eq131_e1677_d_b10, eq131_e1677_d_b11, eq131_e1677_d_b12, eq131_e1677_d_b13, eq131_e1677_d_b14, eq131_e1677_d_b15, eq131_e1677_d_b16, eq131_e1677_d_b17, eq131_e1677_d_b18, eq131_e1677_d_b19, eq131_e1677_d_b20, eq131_e1677_d_b21, eq131_e1677_d_b22, eq131_e1677_d_b23, eq131_e1677_d_b24, eq131_e1677_d_b25, eq131_e1677_d_b26, eq131_e1677_d_b27, eq131_e1677_d_b28, eq131_e1677_d_b29, eq131_e1677_d_b30, eq131_e1677_d_b31, eq131_e1677_d_b32, eq131_e1677_d_b33, eq131_e1677_d_b34, eq131_e1677_d_b35, eq131_e1677_d_b36, eq131_e1677_d_b37, eq131_e1677_d_b38, eq131_e1677_d_b39, eq131_e1677_d_b40, eq131_e1677_d_b41, eq131_e1677_d_b42, eq131_e1677_d_b43, eq131_e1677_d_b44, eq131_e1677_d_b45, eq131_e1677_d_b46, eq131_e1677_d_b47, eq131_e1677_d_b48, eq131_e1677_d_b49, eq131_e1677_d_b50, eq131_e1677_d_b51, eq131_e1677_d_b52, eq131_e1677_d_b53, eq131_e1677_d_b54,) = {
    if ((!s.b[570]) && s.b[573]) {
        let eq131_e1673: f64 = (p.p251 * s.v[228]);
        let eq131_e1674: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 30, eq131_e1673);
        let eq131_e1674_d_n0: f64 = ((p.p251 * s.dn[228][0]) * ddt_scale);
        let eq131_e1674_d_n1: f64 = ((p.p251 * s.dn[228][1]) * ddt_scale);
        let eq131_e1674_d_n2: f64 = ((p.p251 * s.dn[228][2]) * ddt_scale);
        let eq131_e1674_d_n3: f64 = ((p.p251 * s.dn[228][3]) * ddt_scale);
        let eq131_e1674_d_n4: f64 = ((p.p251 * s.dn[228][4]) * ddt_scale);
        let eq131_e1674_d_n5: f64 = ((p.p251 * s.dn[228][5]) * ddt_scale);
        let eq131_e1674_d_n6: f64 = ((p.p251 * s.dn[228][6]) * ddt_scale);
        let eq131_e1674_d_n7: f64 = ((p.p251 * s.dn[228][7]) * ddt_scale);
        let eq131_e1674_d_n8: f64 = ((p.p251 * s.dn[228][8]) * ddt_scale);
        let eq131_e1674_d_n9: f64 = ((p.p251 * s.dn[228][9]) * ddt_scale);
        let eq131_e1674_d_n10: f64 = ((p.p251 * s.dn[228][10]) * ddt_scale);
        let eq131_e1674_d_n11: f64 = ((p.p251 * s.dn[228][11]) * ddt_scale);
        let eq131_e1674_d_n12: f64 = ((p.p251 * s.dn[228][12]) * ddt_scale);
        let eq131_e1674_d_n13: f64 = ((p.p251 * s.dn[228][13]) * ddt_scale);
        let eq131_e1674_d_n14: f64 = ((p.p251 * s.dn[228][14]) * ddt_scale);
        let eq131_e1674_d_n15: f64 = ((p.p251 * s.dn[228][15]) * ddt_scale);
        let eq131_e1674_d_n16: f64 = ((p.p251 * s.dn[228][16]) * ddt_scale);
        let eq131_e1674_d_n17: f64 = ((p.p251 * s.dn[228][17]) * ddt_scale);
        let eq131_e1674_d_n18: f64 = ((p.p251 * s.dn[228][18]) * ddt_scale);
        let eq131_e1674_d_n19: f64 = ((p.p251 * s.dn[228][19]) * ddt_scale);
        let eq131_e1674_d_n20: f64 = ((p.p251 * s.dn[228][20]) * ddt_scale);
        let eq131_e1674_d_n21: f64 = ((p.p251 * s.dn[228][21]) * ddt_scale);
        let eq131_e1674_d_n22: f64 = ((p.p251 * s.dn[228][22]) * ddt_scale);
        let eq131_e1674_d_b0: f64 = ((p.p251 * s.db[228][0]) * ddt_scale);
        let eq131_e1674_d_b1: f64 = ((p.p251 * s.db[228][1]) * ddt_scale);
        let eq131_e1674_d_b2: f64 = ((p.p251 * s.db[228][2]) * ddt_scale);
        let eq131_e1674_d_b3: f64 = ((p.p251 * s.db[228][3]) * ddt_scale);
        let eq131_e1674_d_b4: f64 = ((p.p251 * s.db[228][4]) * ddt_scale);
        let eq131_e1674_d_b5: f64 = ((p.p251 * s.db[228][5]) * ddt_scale);
        let eq131_e1674_d_b6: f64 = ((p.p251 * s.db[228][6]) * ddt_scale);
        let eq131_e1674_d_b7: f64 = ((p.p251 * s.db[228][7]) * ddt_scale);
        let eq131_e1674_d_b8: f64 = ((p.p251 * s.db[228][8]) * ddt_scale);
        let eq131_e1674_d_b9: f64 = ((p.p251 * s.db[228][9]) * ddt_scale);
        let eq131_e1674_d_b10: f64 = ((p.p251 * s.db[228][10]) * ddt_scale);
        let eq131_e1674_d_b11: f64 = ((p.p251 * s.db[228][11]) * ddt_scale);
        let eq131_e1674_d_b12: f64 = ((p.p251 * s.db[228][12]) * ddt_scale);
        let eq131_e1674_d_b13: f64 = ((p.p251 * s.db[228][13]) * ddt_scale);
        let eq131_e1674_d_b14: f64 = ((p.p251 * s.db[228][14]) * ddt_scale);
        let eq131_e1674_d_b15: f64 = ((p.p251 * s.db[228][15]) * ddt_scale);
        let eq131_e1674_d_b16: f64 = ((p.p251 * s.db[228][16]) * ddt_scale);
        let eq131_e1674_d_b17: f64 = ((p.p251 * s.db[228][17]) * ddt_scale);
        let eq131_e1674_d_b18: f64 = ((p.p251 * s.db[228][18]) * ddt_scale);
        let eq131_e1674_d_b19: f64 = ((p.p251 * s.db[228][19]) * ddt_scale);
        let eq131_e1674_d_b20: f64 = ((p.p251 * s.db[228][20]) * ddt_scale);
        let eq131_e1674_d_b21: f64 = ((p.p251 * s.db[228][21]) * ddt_scale);
        let eq131_e1674_d_b22: f64 = ((p.p251 * s.db[228][22]) * ddt_scale);
        let eq131_e1674_d_b23: f64 = ((p.p251 * s.db[228][23]) * ddt_scale);
        let eq131_e1674_d_b24: f64 = ((p.p251 * s.db[228][24]) * ddt_scale);
        let eq131_e1674_d_b25: f64 = ((p.p251 * s.db[228][25]) * ddt_scale);
        let eq131_e1674_d_b26: f64 = ((p.p251 * s.db[228][26]) * ddt_scale);
        let eq131_e1674_d_b27: f64 = ((p.p251 * s.db[228][27]) * ddt_scale);
        let eq131_e1674_d_b28: f64 = ((p.p251 * s.db[228][28]) * ddt_scale);
        let eq131_e1674_d_b29: f64 = ((p.p251 * s.db[228][29]) * ddt_scale);
        let eq131_e1674_d_b30: f64 = ((p.p251 * s.db[228][30]) * ddt_scale);
        let eq131_e1674_d_b31: f64 = ((p.p251 * s.db[228][31]) * ddt_scale);
        let eq131_e1674_d_b32: f64 = ((p.p251 * s.db[228][32]) * ddt_scale);
        let eq131_e1674_d_b33: f64 = ((p.p251 * s.db[228][33]) * ddt_scale);
        let eq131_e1674_d_b34: f64 = ((p.p251 * s.db[228][34]) * ddt_scale);
        let eq131_e1674_d_b35: f64 = ((p.p251 * s.db[228][35]) * ddt_scale);
        let eq131_e1674_d_b36: f64 = ((p.p251 * s.db[228][36]) * ddt_scale);
        let eq131_e1674_d_b37: f64 = ((p.p251 * s.db[228][37]) * ddt_scale);
        let eq131_e1674_d_b38: f64 = ((p.p251 * s.db[228][38]) * ddt_scale);
        let eq131_e1674_d_b39: f64 = ((p.p251 * s.db[228][39]) * ddt_scale);
        let eq131_e1674_d_b40: f64 = ((p.p251 * s.db[228][40]) * ddt_scale);
        let eq131_e1674_d_b41: f64 = ((p.p251 * s.db[228][41]) * ddt_scale);
        let eq131_e1674_d_b42: f64 = ((p.p251 * s.db[228][42]) * ddt_scale);
        let eq131_e1674_d_b43: f64 = ((p.p251 * s.db[228][43]) * ddt_scale);
        let eq131_e1674_d_b44: f64 = ((p.p251 * s.db[228][44]) * ddt_scale);
        let eq131_e1674_d_b45: f64 = ((p.p251 * s.db[228][45]) * ddt_scale);
        let eq131_e1674_d_b46: f64 = ((p.p251 * s.db[228][46]) * ddt_scale);
        let eq131_e1674_d_b47: f64 = ((p.p251 * s.db[228][47]) * ddt_scale);
        let eq131_e1674_d_b48: f64 = ((p.p251 * s.db[228][48]) * ddt_scale);
        let eq131_e1674_d_b49: f64 = ((p.p251 * s.db[228][49]) * ddt_scale);
        let eq131_e1674_d_b50: f64 = ((p.p251 * s.db[228][50]) * ddt_scale);
        let eq131_e1674_d_b51: f64 = ((p.p251 * s.db[228][51]) * ddt_scale);
        let eq131_e1674_d_b52: f64 = ((p.p251 * s.db[228][52]) * ddt_scale);
        let eq131_e1674_d_b53: f64 = ((p.p251 * s.db[228][53]) * ddt_scale);
        let eq131_e1674_d_b54: f64 = ((p.p251 * s.db[228][54]) * ddt_scale);
        let eq131_e1675: f64 = (p.p7 * eq131_e1674);
        let eq131_e1675_d_n0: f64 = (p.p7 * eq131_e1674_d_n0);
        let eq131_e1675_d_n1: f64 = (p.p7 * eq131_e1674_d_n1);
        let eq131_e1675_d_n2: f64 = (p.p7 * eq131_e1674_d_n2);
        let eq131_e1675_d_n3: f64 = (p.p7 * eq131_e1674_d_n3);
        let eq131_e1675_d_n4: f64 = (p.p7 * eq131_e1674_d_n4);
        let eq131_e1675_d_n5: f64 = (p.p7 * eq131_e1674_d_n5);
        let eq131_e1675_d_n6: f64 = (p.p7 * eq131_e1674_d_n6);
        let eq131_e1675_d_n7: f64 = (p.p7 * eq131_e1674_d_n7);
        let eq131_e1675_d_n8: f64 = (p.p7 * eq131_e1674_d_n8);
        let eq131_e1675_d_n9: f64 = (p.p7 * eq131_e1674_d_n9);
        let eq131_e1675_d_n10: f64 = (p.p7 * eq131_e1674_d_n10);
        let eq131_e1675_d_n11: f64 = (p.p7 * eq131_e1674_d_n11);
        let eq131_e1675_d_n12: f64 = (p.p7 * eq131_e1674_d_n12);
        let eq131_e1675_d_n13: f64 = (p.p7 * eq131_e1674_d_n13);
        let eq131_e1675_d_n14: f64 = (p.p7 * eq131_e1674_d_n14);
        let eq131_e1675_d_n15: f64 = (p.p7 * eq131_e1674_d_n15);
        let eq131_e1675_d_n16: f64 = (p.p7 * eq131_e1674_d_n16);
        let eq131_e1675_d_n17: f64 = (p.p7 * eq131_e1674_d_n17);
        let eq131_e1675_d_n18: f64 = (p.p7 * eq131_e1674_d_n18);
        let eq131_e1675_d_n19: f64 = (p.p7 * eq131_e1674_d_n19);
        let eq131_e1675_d_n20: f64 = (p.p7 * eq131_e1674_d_n20);
        let eq131_e1675_d_n21: f64 = (p.p7 * eq131_e1674_d_n21);
        let eq131_e1675_d_n22: f64 = (p.p7 * eq131_e1674_d_n22);
        let eq131_e1675_d_b0: f64 = (p.p7 * eq131_e1674_d_b0);
        let eq131_e1675_d_b1: f64 = (p.p7 * eq131_e1674_d_b1);
        let eq131_e1675_d_b2: f64 = (p.p7 * eq131_e1674_d_b2);
        let eq131_e1675_d_b3: f64 = (p.p7 * eq131_e1674_d_b3);
        let eq131_e1675_d_b4: f64 = (p.p7 * eq131_e1674_d_b4);
        let eq131_e1675_d_b5: f64 = (p.p7 * eq131_e1674_d_b5);
        let eq131_e1675_d_b6: f64 = (p.p7 * eq131_e1674_d_b6);
        let eq131_e1675_d_b7: f64 = (p.p7 * eq131_e1674_d_b7);
        let eq131_e1675_d_b8: f64 = (p.p7 * eq131_e1674_d_b8);
        let eq131_e1675_d_b9: f64 = (p.p7 * eq131_e1674_d_b9);
        let eq131_e1675_d_b10: f64 = (p.p7 * eq131_e1674_d_b10);
        let eq131_e1675_d_b11: f64 = (p.p7 * eq131_e1674_d_b11);
        let eq131_e1675_d_b12: f64 = (p.p7 * eq131_e1674_d_b12);
        let eq131_e1675_d_b13: f64 = (p.p7 * eq131_e1674_d_b13);
        let eq131_e1675_d_b14: f64 = (p.p7 * eq131_e1674_d_b14);
        let eq131_e1675_d_b15: f64 = (p.p7 * eq131_e1674_d_b15);
        let eq131_e1675_d_b16: f64 = (p.p7 * eq131_e1674_d_b16);
        let eq131_e1675_d_b17: f64 = (p.p7 * eq131_e1674_d_b17);
        let eq131_e1675_d_b18: f64 = (p.p7 * eq131_e1674_d_b18);
        let eq131_e1675_d_b19: f64 = (p.p7 * eq131_e1674_d_b19);
        let eq131_e1675_d_b20: f64 = (p.p7 * eq131_e1674_d_b20);
        let eq131_e1675_d_b21: f64 = (p.p7 * eq131_e1674_d_b21);
        let eq131_e1675_d_b22: f64 = (p.p7 * eq131_e1674_d_b22);
        let eq131_e1675_d_b23: f64 = (p.p7 * eq131_e1674_d_b23);
        let eq131_e1675_d_b24: f64 = (p.p7 * eq131_e1674_d_b24);
        let eq131_e1675_d_b25: f64 = (p.p7 * eq131_e1674_d_b25);
        let eq131_e1675_d_b26: f64 = (p.p7 * eq131_e1674_d_b26);
        let eq131_e1675_d_b27: f64 = (p.p7 * eq131_e1674_d_b27);
        let eq131_e1675_d_b28: f64 = (p.p7 * eq131_e1674_d_b28);
        let eq131_e1675_d_b29: f64 = (p.p7 * eq131_e1674_d_b29);
        let eq131_e1675_d_b30: f64 = (p.p7 * eq131_e1674_d_b30);
        let eq131_e1675_d_b31: f64 = (p.p7 * eq131_e1674_d_b31);
        let eq131_e1675_d_b32: f64 = (p.p7 * eq131_e1674_d_b32);
        let eq131_e1675_d_b33: f64 = (p.p7 * eq131_e1674_d_b33);
        let eq131_e1675_d_b34: f64 = (p.p7 * eq131_e1674_d_b34);
        let eq131_e1675_d_b35: f64 = (p.p7 * eq131_e1674_d_b35);
        let eq131_e1675_d_b36: f64 = (p.p7 * eq131_e1674_d_b36);
        let eq131_e1675_d_b37: f64 = (p.p7 * eq131_e1674_d_b37);
        let eq131_e1675_d_b38: f64 = (p.p7 * eq131_e1674_d_b38);
        let eq131_e1675_d_b39: f64 = (p.p7 * eq131_e1674_d_b39);
        let eq131_e1675_d_b40: f64 = (p.p7 * eq131_e1674_d_b40);
        let eq131_e1675_d_b41: f64 = (p.p7 * eq131_e1674_d_b41);
        let eq131_e1675_d_b42: f64 = (p.p7 * eq131_e1674_d_b42);
        let eq131_e1675_d_b43: f64 = (p.p7 * eq131_e1674_d_b43);
        let eq131_e1675_d_b44: f64 = (p.p7 * eq131_e1674_d_b44);
        let eq131_e1675_d_b45: f64 = (p.p7 * eq131_e1674_d_b45);
        let eq131_e1675_d_b46: f64 = (p.p7 * eq131_e1674_d_b46);
        let eq131_e1675_d_b47: f64 = (p.p7 * eq131_e1674_d_b47);
        let eq131_e1675_d_b48: f64 = (p.p7 * eq131_e1674_d_b48);
        let eq131_e1675_d_b49: f64 = (p.p7 * eq131_e1674_d_b49);
        let eq131_e1675_d_b50: f64 = (p.p7 * eq131_e1674_d_b50);
        let eq131_e1675_d_b51: f64 = (p.p7 * eq131_e1674_d_b51);
        let eq131_e1675_d_b52: f64 = (p.p7 * eq131_e1674_d_b52);
        let eq131_e1675_d_b53: f64 = (p.p7 * eq131_e1674_d_b53);
        let eq131_e1675_d_b54: f64 = (p.p7 * eq131_e1674_d_b54);
        (eq131_e1675, eq131_e1675_d_n0, eq131_e1675_d_n1, eq131_e1675_d_n2, eq131_e1675_d_n3, eq131_e1675_d_n4, eq131_e1675_d_n5, eq131_e1675_d_n6, eq131_e1675_d_n7, eq131_e1675_d_n8, eq131_e1675_d_n9, eq131_e1675_d_n10, eq131_e1675_d_n11, eq131_e1675_d_n12, eq131_e1675_d_n13, eq131_e1675_d_n14, eq131_e1675_d_n15, eq131_e1675_d_n16, eq131_e1675_d_n17, eq131_e1675_d_n18, eq131_e1675_d_n19, eq131_e1675_d_n20, eq131_e1675_d_n21, eq131_e1675_d_n22, eq131_e1675_d_b0, eq131_e1675_d_b1, eq131_e1675_d_b2, eq131_e1675_d_b3, eq131_e1675_d_b4, eq131_e1675_d_b5, eq131_e1675_d_b6, eq131_e1675_d_b7, eq131_e1675_d_b8, eq131_e1675_d_b9, eq131_e1675_d_b10, eq131_e1675_d_b11, eq131_e1675_d_b12, eq131_e1675_d_b13, eq131_e1675_d_b14, eq131_e1675_d_b15, eq131_e1675_d_b16, eq131_e1675_d_b17, eq131_e1675_d_b18, eq131_e1675_d_b19, eq131_e1675_d_b20, eq131_e1675_d_b21, eq131_e1675_d_b22, eq131_e1675_d_b23, eq131_e1675_d_b24, eq131_e1675_d_b25, eq131_e1675_d_b26, eq131_e1675_d_b27, eq131_e1675_d_b28, eq131_e1675_d_b29, eq131_e1675_d_b30, eq131_e1675_d_b31, eq131_e1675_d_b32, eq131_e1675_d_b33, eq131_e1675_d_b34, eq131_e1675_d_b35, eq131_e1675_d_b36, eq131_e1675_d_b37, eq131_e1675_d_b38, eq131_e1675_d_b39, eq131_e1675_d_b40, eq131_e1675_d_b41, eq131_e1675_d_b42, eq131_e1675_d_b43, eq131_e1675_d_b44, eq131_e1675_d_b45, eq131_e1675_d_b46, eq131_e1675_d_b47, eq131_e1675_d_b48, eq131_e1675_d_b49, eq131_e1675_d_b50, eq131_e1675_d_b51, eq131_e1675_d_b52, eq131_e1675_d_b53, eq131_e1675_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_value: f64 = eq131_e1677;
        let eq131_node_derivatives: [f64; 23] = [eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22];
        let eq131_branch_derivatives: [f64; 55] = [eq131_e1677_d_b0, eq131_e1677_d_b1, eq131_e1677_d_b2, eq131_e1677_d_b3, eq131_e1677_d_b4, eq131_e1677_d_b5, eq131_e1677_d_b6, eq131_e1677_d_b7, eq131_e1677_d_b8, eq131_e1677_d_b9, eq131_e1677_d_b10, eq131_e1677_d_b11, eq131_e1677_d_b12, eq131_e1677_d_b13, eq131_e1677_d_b14, eq131_e1677_d_b15, eq131_e1677_d_b16, eq131_e1677_d_b17, eq131_e1677_d_b18, eq131_e1677_d_b19, eq131_e1677_d_b20, eq131_e1677_d_b21, eq131_e1677_d_b22, eq131_e1677_d_b23, eq131_e1677_d_b24, eq131_e1677_d_b25, eq131_e1677_d_b26, eq131_e1677_d_b27, eq131_e1677_d_b28, eq131_e1677_d_b29, eq131_e1677_d_b30, eq131_e1677_d_b31, eq131_e1677_d_b32, eq131_e1677_d_b33, eq131_e1677_d_b34, eq131_e1677_d_b35, eq131_e1677_d_b36, eq131_e1677_d_b37, eq131_e1677_d_b38, eq131_e1677_d_b39, eq131_e1677_d_b40, eq131_e1677_d_b41, eq131_e1677_d_b42, eq131_e1677_d_b43, eq131_e1677_d_b44, eq131_e1677_d_b45, eq131_e1677_d_b46, eq131_e1677_d_b47, eq131_e1677_d_b48, eq131_e1677_d_b49, eq131_e1677_d_b50, eq131_e1677_d_b51, eq131_e1677_d_b52, eq131_e1677_d_b53, eq131_e1677_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq131_value),
            &eq131_node_derivatives,
            &eq131_branch_derivatives,
            multiplicity,
        );
        let (eq132_e1686, eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22, eq132_e1686_d_b0, eq132_e1686_d_b1, eq132_e1686_d_b2, eq132_e1686_d_b3, eq132_e1686_d_b4, eq132_e1686_d_b5, eq132_e1686_d_b6, eq132_e1686_d_b7, eq132_e1686_d_b8, eq132_e1686_d_b9, eq132_e1686_d_b10, eq132_e1686_d_b11, eq132_e1686_d_b12, eq132_e1686_d_b13, eq132_e1686_d_b14, eq132_e1686_d_b15, eq132_e1686_d_b16, eq132_e1686_d_b17, eq132_e1686_d_b18, eq132_e1686_d_b19, eq132_e1686_d_b20, eq132_e1686_d_b21, eq132_e1686_d_b22, eq132_e1686_d_b23, eq132_e1686_d_b24, eq132_e1686_d_b25, eq132_e1686_d_b26, eq132_e1686_d_b27, eq132_e1686_d_b28, eq132_e1686_d_b29, eq132_e1686_d_b30, eq132_e1686_d_b31, eq132_e1686_d_b32, eq132_e1686_d_b33, eq132_e1686_d_b34, eq132_e1686_d_b35, eq132_e1686_d_b36, eq132_e1686_d_b37, eq132_e1686_d_b38, eq132_e1686_d_b39, eq132_e1686_d_b40, eq132_e1686_d_b41, eq132_e1686_d_b42, eq132_e1686_d_b43, eq132_e1686_d_b44, eq132_e1686_d_b45, eq132_e1686_d_b46, eq132_e1686_d_b47, eq132_e1686_d_b48, eq132_e1686_d_b49, eq132_e1686_d_b50, eq132_e1686_d_b51, eq132_e1686_d_b52, eq132_e1686_d_b53, eq132_e1686_d_b54,) = {
    if (s.b[575] && s.b[576]) {
        let eq132_e1683: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 31, s.v[241]);
        let eq132_e1684: f64 = (p.p7 * eq132_e1683);
        let eq132_e1684_d_n0: f64 = (p.p7 * (s.dn[241][0] * ddt_scale));
        let eq132_e1684_d_n1: f64 = (p.p7 * (s.dn[241][1] * ddt_scale));
        let eq132_e1684_d_n2: f64 = (p.p7 * (s.dn[241][2] * ddt_scale));
        let eq132_e1684_d_n3: f64 = (p.p7 * (s.dn[241][3] * ddt_scale));
        let eq132_e1684_d_n4: f64 = (p.p7 * (s.dn[241][4] * ddt_scale));
        let eq132_e1684_d_n5: f64 = (p.p7 * (s.dn[241][5] * ddt_scale));
        let eq132_e1684_d_n6: f64 = (p.p7 * (s.dn[241][6] * ddt_scale));
        let eq132_e1684_d_n7: f64 = (p.p7 * (s.dn[241][7] * ddt_scale));
        let eq132_e1684_d_n8: f64 = (p.p7 * (s.dn[241][8] * ddt_scale));
        let eq132_e1684_d_n9: f64 = (p.p7 * (s.dn[241][9] * ddt_scale));
        let eq132_e1684_d_n10: f64 = (p.p7 * (s.dn[241][10] * ddt_scale));
        let eq132_e1684_d_n11: f64 = (p.p7 * (s.dn[241][11] * ddt_scale));
        let eq132_e1684_d_n12: f64 = (p.p7 * (s.dn[241][12] * ddt_scale));
        let eq132_e1684_d_n13: f64 = (p.p7 * (s.dn[241][13] * ddt_scale));
        let eq132_e1684_d_n14: f64 = (p.p7 * (s.dn[241][14] * ddt_scale));
        let eq132_e1684_d_n15: f64 = (p.p7 * (s.dn[241][15] * ddt_scale));
        let eq132_e1684_d_n16: f64 = (p.p7 * (s.dn[241][16] * ddt_scale));
        let eq132_e1684_d_n17: f64 = (p.p7 * (s.dn[241][17] * ddt_scale));
        let eq132_e1684_d_n18: f64 = (p.p7 * (s.dn[241][18] * ddt_scale));
        let eq132_e1684_d_n19: f64 = (p.p7 * (s.dn[241][19] * ddt_scale));
        let eq132_e1684_d_n20: f64 = (p.p7 * (s.dn[241][20] * ddt_scale));
        let eq132_e1684_d_n21: f64 = (p.p7 * (s.dn[241][21] * ddt_scale));
        let eq132_e1684_d_n22: f64 = (p.p7 * (s.dn[241][22] * ddt_scale));
        let eq132_e1684_d_b0: f64 = (p.p7 * (s.db[241][0] * ddt_scale));
        let eq132_e1684_d_b1: f64 = (p.p7 * (s.db[241][1] * ddt_scale));
        let eq132_e1684_d_b2: f64 = (p.p7 * (s.db[241][2] * ddt_scale));
        let eq132_e1684_d_b3: f64 = (p.p7 * (s.db[241][3] * ddt_scale));
        let eq132_e1684_d_b4: f64 = (p.p7 * (s.db[241][4] * ddt_scale));
        let eq132_e1684_d_b5: f64 = (p.p7 * (s.db[241][5] * ddt_scale));
        let eq132_e1684_d_b6: f64 = (p.p7 * (s.db[241][6] * ddt_scale));
        let eq132_e1684_d_b7: f64 = (p.p7 * (s.db[241][7] * ddt_scale));
        let eq132_e1684_d_b8: f64 = (p.p7 * (s.db[241][8] * ddt_scale));
        let eq132_e1684_d_b9: f64 = (p.p7 * (s.db[241][9] * ddt_scale));
        let eq132_e1684_d_b10: f64 = (p.p7 * (s.db[241][10] * ddt_scale));
        let eq132_e1684_d_b11: f64 = (p.p7 * (s.db[241][11] * ddt_scale));
        let eq132_e1684_d_b12: f64 = (p.p7 * (s.db[241][12] * ddt_scale));
        let eq132_e1684_d_b13: f64 = (p.p7 * (s.db[241][13] * ddt_scale));
        let eq132_e1684_d_b14: f64 = (p.p7 * (s.db[241][14] * ddt_scale));
        let eq132_e1684_d_b15: f64 = (p.p7 * (s.db[241][15] * ddt_scale));
        let eq132_e1684_d_b16: f64 = (p.p7 * (s.db[241][16] * ddt_scale));
        let eq132_e1684_d_b17: f64 = (p.p7 * (s.db[241][17] * ddt_scale));
        let eq132_e1684_d_b18: f64 = (p.p7 * (s.db[241][18] * ddt_scale));
        let eq132_e1684_d_b19: f64 = (p.p7 * (s.db[241][19] * ddt_scale));
        let eq132_e1684_d_b20: f64 = (p.p7 * (s.db[241][20] * ddt_scale));
        let eq132_e1684_d_b21: f64 = (p.p7 * (s.db[241][21] * ddt_scale));
        let eq132_e1684_d_b22: f64 = (p.p7 * (s.db[241][22] * ddt_scale));
        let eq132_e1684_d_b23: f64 = (p.p7 * (s.db[241][23] * ddt_scale));
        let eq132_e1684_d_b24: f64 = (p.p7 * (s.db[241][24] * ddt_scale));
        let eq132_e1684_d_b25: f64 = (p.p7 * (s.db[241][25] * ddt_scale));
        let eq132_e1684_d_b26: f64 = (p.p7 * (s.db[241][26] * ddt_scale));
        let eq132_e1684_d_b27: f64 = (p.p7 * (s.db[241][27] * ddt_scale));
        let eq132_e1684_d_b28: f64 = (p.p7 * (s.db[241][28] * ddt_scale));
        let eq132_e1684_d_b29: f64 = (p.p7 * (s.db[241][29] * ddt_scale));
        let eq132_e1684_d_b30: f64 = (p.p7 * (s.db[241][30] * ddt_scale));
        let eq132_e1684_d_b31: f64 = (p.p7 * (s.db[241][31] * ddt_scale));
        let eq132_e1684_d_b32: f64 = (p.p7 * (s.db[241][32] * ddt_scale));
        let eq132_e1684_d_b33: f64 = (p.p7 * (s.db[241][33] * ddt_scale));
        let eq132_e1684_d_b34: f64 = (p.p7 * (s.db[241][34] * ddt_scale));
        let eq132_e1684_d_b35: f64 = (p.p7 * (s.db[241][35] * ddt_scale));
        let eq132_e1684_d_b36: f64 = (p.p7 * (s.db[241][36] * ddt_scale));
        let eq132_e1684_d_b37: f64 = (p.p7 * (s.db[241][37] * ddt_scale));
        let eq132_e1684_d_b38: f64 = (p.p7 * (s.db[241][38] * ddt_scale));
        let eq132_e1684_d_b39: f64 = (p.p7 * (s.db[241][39] * ddt_scale));
        let eq132_e1684_d_b40: f64 = (p.p7 * (s.db[241][40] * ddt_scale));
        let eq132_e1684_d_b41: f64 = (p.p7 * (s.db[241][41] * ddt_scale));
        let eq132_e1684_d_b42: f64 = (p.p7 * (s.db[241][42] * ddt_scale));
        let eq132_e1684_d_b43: f64 = (p.p7 * (s.db[241][43] * ddt_scale));
        let eq132_e1684_d_b44: f64 = (p.p7 * (s.db[241][44] * ddt_scale));
        let eq132_e1684_d_b45: f64 = (p.p7 * (s.db[241][45] * ddt_scale));
        let eq132_e1684_d_b46: f64 = (p.p7 * (s.db[241][46] * ddt_scale));
        let eq132_e1684_d_b47: f64 = (p.p7 * (s.db[241][47] * ddt_scale));
        let eq132_e1684_d_b48: f64 = (p.p7 * (s.db[241][48] * ddt_scale));
        let eq132_e1684_d_b49: f64 = (p.p7 * (s.db[241][49] * ddt_scale));
        let eq132_e1684_d_b50: f64 = (p.p7 * (s.db[241][50] * ddt_scale));
        let eq132_e1684_d_b51: f64 = (p.p7 * (s.db[241][51] * ddt_scale));
        let eq132_e1684_d_b52: f64 = (p.p7 * (s.db[241][52] * ddt_scale));
        let eq132_e1684_d_b53: f64 = (p.p7 * (s.db[241][53] * ddt_scale));
        let eq132_e1684_d_b54: f64 = (p.p7 * (s.db[241][54] * ddt_scale));
        (eq132_e1684, eq132_e1684_d_n0, eq132_e1684_d_n1, eq132_e1684_d_n2, eq132_e1684_d_n3, eq132_e1684_d_n4, eq132_e1684_d_n5, eq132_e1684_d_n6, eq132_e1684_d_n7, eq132_e1684_d_n8, eq132_e1684_d_n9, eq132_e1684_d_n10, eq132_e1684_d_n11, eq132_e1684_d_n12, eq132_e1684_d_n13, eq132_e1684_d_n14, eq132_e1684_d_n15, eq132_e1684_d_n16, eq132_e1684_d_n17, eq132_e1684_d_n18, eq132_e1684_d_n19, eq132_e1684_d_n20, eq132_e1684_d_n21, eq132_e1684_d_n22, eq132_e1684_d_b0, eq132_e1684_d_b1, eq132_e1684_d_b2, eq132_e1684_d_b3, eq132_e1684_d_b4, eq132_e1684_d_b5, eq132_e1684_d_b6, eq132_e1684_d_b7, eq132_e1684_d_b8, eq132_e1684_d_b9, eq132_e1684_d_b10, eq132_e1684_d_b11, eq132_e1684_d_b12, eq132_e1684_d_b13, eq132_e1684_d_b14, eq132_e1684_d_b15, eq132_e1684_d_b16, eq132_e1684_d_b17, eq132_e1684_d_b18, eq132_e1684_d_b19, eq132_e1684_d_b20, eq132_e1684_d_b21, eq132_e1684_d_b22, eq132_e1684_d_b23, eq132_e1684_d_b24, eq132_e1684_d_b25, eq132_e1684_d_b26, eq132_e1684_d_b27, eq132_e1684_d_b28, eq132_e1684_d_b29, eq132_e1684_d_b30, eq132_e1684_d_b31, eq132_e1684_d_b32, eq132_e1684_d_b33, eq132_e1684_d_b34, eq132_e1684_d_b35, eq132_e1684_d_b36, eq132_e1684_d_b37, eq132_e1684_d_b38, eq132_e1684_d_b39, eq132_e1684_d_b40, eq132_e1684_d_b41, eq132_e1684_d_b42, eq132_e1684_d_b43, eq132_e1684_d_b44, eq132_e1684_d_b45, eq132_e1684_d_b46, eq132_e1684_d_b47, eq132_e1684_d_b48, eq132_e1684_d_b49, eq132_e1684_d_b50, eq132_e1684_d_b51, eq132_e1684_d_b52, eq132_e1684_d_b53, eq132_e1684_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq132_value: f64 = eq132_e1686;
        let eq132_node_derivatives: [f64; 23] = [eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22];
        let eq132_branch_derivatives: [f64; 55] = [eq132_e1686_d_b0, eq132_e1686_d_b1, eq132_e1686_d_b2, eq132_e1686_d_b3, eq132_e1686_d_b4, eq132_e1686_d_b5, eq132_e1686_d_b6, eq132_e1686_d_b7, eq132_e1686_d_b8, eq132_e1686_d_b9, eq132_e1686_d_b10, eq132_e1686_d_b11, eq132_e1686_d_b12, eq132_e1686_d_b13, eq132_e1686_d_b14, eq132_e1686_d_b15, eq132_e1686_d_b16, eq132_e1686_d_b17, eq132_e1686_d_b18, eq132_e1686_d_b19, eq132_e1686_d_b20, eq132_e1686_d_b21, eq132_e1686_d_b22, eq132_e1686_d_b23, eq132_e1686_d_b24, eq132_e1686_d_b25, eq132_e1686_d_b26, eq132_e1686_d_b27, eq132_e1686_d_b28, eq132_e1686_d_b29, eq132_e1686_d_b30, eq132_e1686_d_b31, eq132_e1686_d_b32, eq132_e1686_d_b33, eq132_e1686_d_b34, eq132_e1686_d_b35, eq132_e1686_d_b36, eq132_e1686_d_b37, eq132_e1686_d_b38, eq132_e1686_d_b39, eq132_e1686_d_b40, eq132_e1686_d_b41, eq132_e1686_d_b42, eq132_e1686_d_b43, eq132_e1686_d_b44, eq132_e1686_d_b45, eq132_e1686_d_b46, eq132_e1686_d_b47, eq132_e1686_d_b48, eq132_e1686_d_b49, eq132_e1686_d_b50, eq132_e1686_d_b51, eq132_e1686_d_b52, eq132_e1686_d_b53, eq132_e1686_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(19),
            multiplicity * (eq132_value),
            &eq132_node_derivatives,
            &eq132_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_20(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[240][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[240][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[240][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[240][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[240][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[240][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[240][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[240][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[240][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[240][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[240][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[240][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[240][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[240][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[240][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[240][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[240][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[240][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[240][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[240][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[240][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[240][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[240][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[240][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[240][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[240][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[240][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[240][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[240][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[240][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[240][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[240][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[240][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[240][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[240][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[240][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[240][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[240][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[240][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[240][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[240][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[240][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[240][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[240][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[240][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[240][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[240][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[240][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[240][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[240][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[240][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[240][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[240][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[240][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[240][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[240][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[240][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[240][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[240][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[240][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[240][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[240][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[240][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[240][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[240][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[240][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[240][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[240][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[240][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[240][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[240][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[240][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[240][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[240][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[240][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[240][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[240][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[240][54] * ddt_scale));
        let (eq133_e1697, eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22, eq133_e1697_d_b0, eq133_e1697_d_b1, eq133_e1697_d_b2, eq133_e1697_d_b3, eq133_e1697_d_b4, eq133_e1697_d_b5, eq133_e1697_d_b6, eq133_e1697_d_b7, eq133_e1697_d_b8, eq133_e1697_d_b9, eq133_e1697_d_b10, eq133_e1697_d_b11, eq133_e1697_d_b12, eq133_e1697_d_b13, eq133_e1697_d_b14, eq133_e1697_d_b15, eq133_e1697_d_b16, eq133_e1697_d_b17, eq133_e1697_d_b18, eq133_e1697_d_b19, eq133_e1697_d_b20, eq133_e1697_d_b21, eq133_e1697_d_b22, eq133_e1697_d_b23, eq133_e1697_d_b24, eq133_e1697_d_b25, eq133_e1697_d_b26, eq133_e1697_d_b27, eq133_e1697_d_b28, eq133_e1697_d_b29, eq133_e1697_d_b30, eq133_e1697_d_b31, eq133_e1697_d_b32, eq133_e1697_d_b33, eq133_e1697_d_b34, eq133_e1697_d_b35, eq133_e1697_d_b36, eq133_e1697_d_b37, eq133_e1697_d_b38, eq133_e1697_d_b39, eq133_e1697_d_b40, eq133_e1697_d_b41, eq133_e1697_d_b42, eq133_e1697_d_b43, eq133_e1697_d_b44, eq133_e1697_d_b45, eq133_e1697_d_b46, eq133_e1697_d_b47, eq133_e1697_d_b48, eq133_e1697_d_b49, eq133_e1697_d_b50, eq133_e1697_d_b51, eq133_e1697_d_b52, eq133_e1697_d_b53, eq133_e1697_d_b54,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq133_e1694: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 32, s.v[240]);
        let eq133_e1695: f64 = (p.p7 * eq133_e1694);
        (eq133_e1695, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq133_value: f64 = eq133_e1697;
        let eq133_node_derivatives: [f64; 23] = [eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22];
        let eq133_branch_derivatives: [f64; 55] = [eq133_e1697_d_b0, eq133_e1697_d_b1, eq133_e1697_d_b2, eq133_e1697_d_b3, eq133_e1697_d_b4, eq133_e1697_d_b5, eq133_e1697_d_b6, eq133_e1697_d_b7, eq133_e1697_d_b8, eq133_e1697_d_b9, eq133_e1697_d_b10, eq133_e1697_d_b11, eq133_e1697_d_b12, eq133_e1697_d_b13, eq133_e1697_d_b14, eq133_e1697_d_b15, eq133_e1697_d_b16, eq133_e1697_d_b17, eq133_e1697_d_b18, eq133_e1697_d_b19, eq133_e1697_d_b20, eq133_e1697_d_b21, eq133_e1697_d_b22, eq133_e1697_d_b23, eq133_e1697_d_b24, eq133_e1697_d_b25, eq133_e1697_d_b26, eq133_e1697_d_b27, eq133_e1697_d_b28, eq133_e1697_d_b29, eq133_e1697_d_b30, eq133_e1697_d_b31, eq133_e1697_d_b32, eq133_e1697_d_b33, eq133_e1697_d_b34, eq133_e1697_d_b35, eq133_e1697_d_b36, eq133_e1697_d_b37, eq133_e1697_d_b38, eq133_e1697_d_b39, eq133_e1697_d_b40, eq133_e1697_d_b41, eq133_e1697_d_b42, eq133_e1697_d_b43, eq133_e1697_d_b44, eq133_e1697_d_b45, eq133_e1697_d_b46, eq133_e1697_d_b47, eq133_e1697_d_b48, eq133_e1697_d_b49, eq133_e1697_d_b50, eq133_e1697_d_b51, eq133_e1697_d_b52, eq133_e1697_d_b53, eq133_e1697_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(19),
            multiplicity * (eq133_value),
            &eq133_node_derivatives,
            &eq133_branch_derivatives,
            multiplicity,
        );
        let (eq134_e1710, eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22, eq134_e1710_d_b0, eq134_e1710_d_b1, eq134_e1710_d_b2, eq134_e1710_d_b3, eq134_e1710_d_b4, eq134_e1710_d_b5, eq134_e1710_d_b6, eq134_e1710_d_b7, eq134_e1710_d_b8, eq134_e1710_d_b9, eq134_e1710_d_b10, eq134_e1710_d_b11, eq134_e1710_d_b12, eq134_e1710_d_b13, eq134_e1710_d_b14, eq134_e1710_d_b15, eq134_e1710_d_b16, eq134_e1710_d_b17, eq134_e1710_d_b18, eq134_e1710_d_b19, eq134_e1710_d_b20, eq134_e1710_d_b21, eq134_e1710_d_b22, eq134_e1710_d_b23, eq134_e1710_d_b24, eq134_e1710_d_b25, eq134_e1710_d_b26, eq134_e1710_d_b27, eq134_e1710_d_b28, eq134_e1710_d_b29, eq134_e1710_d_b30, eq134_e1710_d_b31, eq134_e1710_d_b32, eq134_e1710_d_b33, eq134_e1710_d_b34, eq134_e1710_d_b35, eq134_e1710_d_b36, eq134_e1710_d_b37, eq134_e1710_d_b38, eq134_e1710_d_b39, eq134_e1710_d_b40, eq134_e1710_d_b41, eq134_e1710_d_b42, eq134_e1710_d_b43, eq134_e1710_d_b44, eq134_e1710_d_b45, eq134_e1710_d_b46, eq134_e1710_d_b47, eq134_e1710_d_b48, eq134_e1710_d_b49, eq134_e1710_d_b50, eq134_e1710_d_b51, eq134_e1710_d_b52, eq134_e1710_d_b53, eq134_e1710_d_b54,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq134_e1705: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 33, s.v[240]);
        let eq134_e1706: f64 = (p.p7 * eq134_e1705);
        let eq134_e1708: f64 = (eq134_e1706 * p.p246);
        let eq134_e1708_d_n0: f64 = (__rspice_deriv_cse_0 * p.p246);
        let eq134_e1708_d_n1: f64 = (__rspice_deriv_cse_1 * p.p246);
        let eq134_e1708_d_n2: f64 = (__rspice_deriv_cse_2 * p.p246);
        let eq134_e1708_d_n3: f64 = (__rspice_deriv_cse_3 * p.p246);
        let eq134_e1708_d_n4: f64 = (__rspice_deriv_cse_4 * p.p246);
        let eq134_e1708_d_n5: f64 = (__rspice_deriv_cse_5 * p.p246);
        let eq134_e1708_d_n6: f64 = (__rspice_deriv_cse_6 * p.p246);
        let eq134_e1708_d_n7: f64 = (__rspice_deriv_cse_7 * p.p246);
        let eq134_e1708_d_n8: f64 = (__rspice_deriv_cse_8 * p.p246);
        let eq134_e1708_d_n9: f64 = (__rspice_deriv_cse_9 * p.p246);
        let eq134_e1708_d_n10: f64 = (__rspice_deriv_cse_10 * p.p246);
        let eq134_e1708_d_n11: f64 = (__rspice_deriv_cse_11 * p.p246);
        let eq134_e1708_d_n12: f64 = (__rspice_deriv_cse_12 * p.p246);
        let eq134_e1708_d_n13: f64 = (__rspice_deriv_cse_13 * p.p246);
        let eq134_e1708_d_n14: f64 = (__rspice_deriv_cse_14 * p.p246);
        let eq134_e1708_d_n15: f64 = (__rspice_deriv_cse_15 * p.p246);
        let eq134_e1708_d_n16: f64 = (__rspice_deriv_cse_16 * p.p246);
        let eq134_e1708_d_n17: f64 = (__rspice_deriv_cse_17 * p.p246);
        let eq134_e1708_d_n18: f64 = (__rspice_deriv_cse_18 * p.p246);
        let eq134_e1708_d_n19: f64 = (__rspice_deriv_cse_19 * p.p246);
        let eq134_e1708_d_n20: f64 = (__rspice_deriv_cse_20 * p.p246);
        let eq134_e1708_d_n21: f64 = (__rspice_deriv_cse_21 * p.p246);
        let eq134_e1708_d_n22: f64 = (__rspice_deriv_cse_22 * p.p246);
        let eq134_e1708_d_b0: f64 = (__rspice_deriv_cse_23 * p.p246);
        let eq134_e1708_d_b1: f64 = (__rspice_deriv_cse_24 * p.p246);
        let eq134_e1708_d_b2: f64 = (__rspice_deriv_cse_25 * p.p246);
        let eq134_e1708_d_b3: f64 = (__rspice_deriv_cse_26 * p.p246);
        let eq134_e1708_d_b4: f64 = (__rspice_deriv_cse_27 * p.p246);
        let eq134_e1708_d_b5: f64 = (__rspice_deriv_cse_28 * p.p246);
        let eq134_e1708_d_b6: f64 = (__rspice_deriv_cse_29 * p.p246);
        let eq134_e1708_d_b7: f64 = (__rspice_deriv_cse_30 * p.p246);
        let eq134_e1708_d_b8: f64 = (__rspice_deriv_cse_31 * p.p246);
        let eq134_e1708_d_b9: f64 = (__rspice_deriv_cse_32 * p.p246);
        let eq134_e1708_d_b10: f64 = (__rspice_deriv_cse_33 * p.p246);
        let eq134_e1708_d_b11: f64 = (__rspice_deriv_cse_34 * p.p246);
        let eq134_e1708_d_b12: f64 = (__rspice_deriv_cse_35 * p.p246);
        let eq134_e1708_d_b13: f64 = (__rspice_deriv_cse_36 * p.p246);
        let eq134_e1708_d_b14: f64 = (__rspice_deriv_cse_37 * p.p246);
        let eq134_e1708_d_b15: f64 = (__rspice_deriv_cse_38 * p.p246);
        let eq134_e1708_d_b16: f64 = (__rspice_deriv_cse_39 * p.p246);
        let eq134_e1708_d_b17: f64 = (__rspice_deriv_cse_40 * p.p246);
        let eq134_e1708_d_b18: f64 = (__rspice_deriv_cse_41 * p.p246);
        let eq134_e1708_d_b19: f64 = (__rspice_deriv_cse_42 * p.p246);
        let eq134_e1708_d_b20: f64 = (__rspice_deriv_cse_43 * p.p246);
        let eq134_e1708_d_b21: f64 = (__rspice_deriv_cse_44 * p.p246);
        let eq134_e1708_d_b22: f64 = (__rspice_deriv_cse_45 * p.p246);
        let eq134_e1708_d_b23: f64 = (__rspice_deriv_cse_46 * p.p246);
        let eq134_e1708_d_b24: f64 = (__rspice_deriv_cse_47 * p.p246);
        let eq134_e1708_d_b25: f64 = (__rspice_deriv_cse_48 * p.p246);
        let eq134_e1708_d_b26: f64 = (__rspice_deriv_cse_49 * p.p246);
        let eq134_e1708_d_b27: f64 = (__rspice_deriv_cse_50 * p.p246);
        let eq134_e1708_d_b28: f64 = (__rspice_deriv_cse_51 * p.p246);
        let eq134_e1708_d_b29: f64 = (__rspice_deriv_cse_52 * p.p246);
        let eq134_e1708_d_b30: f64 = (__rspice_deriv_cse_53 * p.p246);
        let eq134_e1708_d_b31: f64 = (__rspice_deriv_cse_54 * p.p246);
        let eq134_e1708_d_b32: f64 = (__rspice_deriv_cse_55 * p.p246);
        let eq134_e1708_d_b33: f64 = (__rspice_deriv_cse_56 * p.p246);
        let eq134_e1708_d_b34: f64 = (__rspice_deriv_cse_57 * p.p246);
        let eq134_e1708_d_b35: f64 = (__rspice_deriv_cse_58 * p.p246);
        let eq134_e1708_d_b36: f64 = (__rspice_deriv_cse_59 * p.p246);
        let eq134_e1708_d_b37: f64 = (__rspice_deriv_cse_60 * p.p246);
        let eq134_e1708_d_b38: f64 = (__rspice_deriv_cse_61 * p.p246);
        let eq134_e1708_d_b39: f64 = (__rspice_deriv_cse_62 * p.p246);
        let eq134_e1708_d_b40: f64 = (__rspice_deriv_cse_63 * p.p246);
        let eq134_e1708_d_b41: f64 = (__rspice_deriv_cse_64 * p.p246);
        let eq134_e1708_d_b42: f64 = (__rspice_deriv_cse_65 * p.p246);
        let eq134_e1708_d_b43: f64 = (__rspice_deriv_cse_66 * p.p246);
        let eq134_e1708_d_b44: f64 = (__rspice_deriv_cse_67 * p.p246);
        let eq134_e1708_d_b45: f64 = (__rspice_deriv_cse_68 * p.p246);
        let eq134_e1708_d_b46: f64 = (__rspice_deriv_cse_69 * p.p246);
        let eq134_e1708_d_b47: f64 = (__rspice_deriv_cse_70 * p.p246);
        let eq134_e1708_d_b48: f64 = (__rspice_deriv_cse_71 * p.p246);
        let eq134_e1708_d_b49: f64 = (__rspice_deriv_cse_72 * p.p246);
        let eq134_e1708_d_b50: f64 = (__rspice_deriv_cse_73 * p.p246);
        let eq134_e1708_d_b51: f64 = (__rspice_deriv_cse_74 * p.p246);
        let eq134_e1708_d_b52: f64 = (__rspice_deriv_cse_75 * p.p246);
        let eq134_e1708_d_b53: f64 = (__rspice_deriv_cse_76 * p.p246);
        let eq134_e1708_d_b54: f64 = (__rspice_deriv_cse_77 * p.p246);
        (eq134_e1708, eq134_e1708_d_n0, eq134_e1708_d_n1, eq134_e1708_d_n2, eq134_e1708_d_n3, eq134_e1708_d_n4, eq134_e1708_d_n5, eq134_e1708_d_n6, eq134_e1708_d_n7, eq134_e1708_d_n8, eq134_e1708_d_n9, eq134_e1708_d_n10, eq134_e1708_d_n11, eq134_e1708_d_n12, eq134_e1708_d_n13, eq134_e1708_d_n14, eq134_e1708_d_n15, eq134_e1708_d_n16, eq134_e1708_d_n17, eq134_e1708_d_n18, eq134_e1708_d_n19, eq134_e1708_d_n20, eq134_e1708_d_n21, eq134_e1708_d_n22, eq134_e1708_d_b0, eq134_e1708_d_b1, eq134_e1708_d_b2, eq134_e1708_d_b3, eq134_e1708_d_b4, eq134_e1708_d_b5, eq134_e1708_d_b6, eq134_e1708_d_b7, eq134_e1708_d_b8, eq134_e1708_d_b9, eq134_e1708_d_b10, eq134_e1708_d_b11, eq134_e1708_d_b12, eq134_e1708_d_b13, eq134_e1708_d_b14, eq134_e1708_d_b15, eq134_e1708_d_b16, eq134_e1708_d_b17, eq134_e1708_d_b18, eq134_e1708_d_b19, eq134_e1708_d_b20, eq134_e1708_d_b21, eq134_e1708_d_b22, eq134_e1708_d_b23, eq134_e1708_d_b24, eq134_e1708_d_b25, eq134_e1708_d_b26, eq134_e1708_d_b27, eq134_e1708_d_b28, eq134_e1708_d_b29, eq134_e1708_d_b30, eq134_e1708_d_b31, eq134_e1708_d_b32, eq134_e1708_d_b33, eq134_e1708_d_b34, eq134_e1708_d_b35, eq134_e1708_d_b36, eq134_e1708_d_b37, eq134_e1708_d_b38, eq134_e1708_d_b39, eq134_e1708_d_b40, eq134_e1708_d_b41, eq134_e1708_d_b42, eq134_e1708_d_b43, eq134_e1708_d_b44, eq134_e1708_d_b45, eq134_e1708_d_b46, eq134_e1708_d_b47, eq134_e1708_d_b48, eq134_e1708_d_b49, eq134_e1708_d_b50, eq134_e1708_d_b51, eq134_e1708_d_b52, eq134_e1708_d_b53, eq134_e1708_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq134_value: f64 = eq134_e1710;
        let eq134_node_derivatives: [f64; 23] = [eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22];
        let eq134_branch_derivatives: [f64; 55] = [eq134_e1710_d_b0, eq134_e1710_d_b1, eq134_e1710_d_b2, eq134_e1710_d_b3, eq134_e1710_d_b4, eq134_e1710_d_b5, eq134_e1710_d_b6, eq134_e1710_d_b7, eq134_e1710_d_b8, eq134_e1710_d_b9, eq134_e1710_d_b10, eq134_e1710_d_b11, eq134_e1710_d_b12, eq134_e1710_d_b13, eq134_e1710_d_b14, eq134_e1710_d_b15, eq134_e1710_d_b16, eq134_e1710_d_b17, eq134_e1710_d_b18, eq134_e1710_d_b19, eq134_e1710_d_b20, eq134_e1710_d_b21, eq134_e1710_d_b22, eq134_e1710_d_b23, eq134_e1710_d_b24, eq134_e1710_d_b25, eq134_e1710_d_b26, eq134_e1710_d_b27, eq134_e1710_d_b28, eq134_e1710_d_b29, eq134_e1710_d_b30, eq134_e1710_d_b31, eq134_e1710_d_b32, eq134_e1710_d_b33, eq134_e1710_d_b34, eq134_e1710_d_b35, eq134_e1710_d_b36, eq134_e1710_d_b37, eq134_e1710_d_b38, eq134_e1710_d_b39, eq134_e1710_d_b40, eq134_e1710_d_b41, eq134_e1710_d_b42, eq134_e1710_d_b43, eq134_e1710_d_b44, eq134_e1710_d_b45, eq134_e1710_d_b46, eq134_e1710_d_b47, eq134_e1710_d_b48, eq134_e1710_d_b49, eq134_e1710_d_b50, eq134_e1710_d_b51, eq134_e1710_d_b52, eq134_e1710_d_b53, eq134_e1710_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(19),
            multiplicity * (eq134_value),
            &eq134_node_derivatives,
            &eq134_branch_derivatives,
            multiplicity,
        );
        let (eq135_e1722, eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22, eq135_e1722_d_b0, eq135_e1722_d_b1, eq135_e1722_d_b2, eq135_e1722_d_b3, eq135_e1722_d_b4, eq135_e1722_d_b5, eq135_e1722_d_b6, eq135_e1722_d_b7, eq135_e1722_d_b8, eq135_e1722_d_b9, eq135_e1722_d_b10, eq135_e1722_d_b11, eq135_e1722_d_b12, eq135_e1722_d_b13, eq135_e1722_d_b14, eq135_e1722_d_b15, eq135_e1722_d_b16, eq135_e1722_d_b17, eq135_e1722_d_b18, eq135_e1722_d_b19, eq135_e1722_d_b20, eq135_e1722_d_b21, eq135_e1722_d_b22, eq135_e1722_d_b23, eq135_e1722_d_b24, eq135_e1722_d_b25, eq135_e1722_d_b26, eq135_e1722_d_b27, eq135_e1722_d_b28, eq135_e1722_d_b29, eq135_e1722_d_b30, eq135_e1722_d_b31, eq135_e1722_d_b32, eq135_e1722_d_b33, eq135_e1722_d_b34, eq135_e1722_d_b35, eq135_e1722_d_b36, eq135_e1722_d_b37, eq135_e1722_d_b38, eq135_e1722_d_b39, eq135_e1722_d_b40, eq135_e1722_d_b41, eq135_e1722_d_b42, eq135_e1722_d_b43, eq135_e1722_d_b44, eq135_e1722_d_b45, eq135_e1722_d_b46, eq135_e1722_d_b47, eq135_e1722_d_b48, eq135_e1722_d_b49, eq135_e1722_d_b50, eq135_e1722_d_b51, eq135_e1722_d_b52, eq135_e1722_d_b53, eq135_e1722_d_b54,) = {
    if ((s.b[575] && s.b[576]) && (!s.b[577])) {
        let eq135_e1719: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 34, s.v[240]);
        let eq135_e1720: f64 = (p.p7 * eq135_e1719);
        (eq135_e1720, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_value: f64 = eq135_e1722;
        let eq135_node_derivatives: [f64; 23] = [eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22];
        let eq135_branch_derivatives: [f64; 55] = [eq135_e1722_d_b0, eq135_e1722_d_b1, eq135_e1722_d_b2, eq135_e1722_d_b3, eq135_e1722_d_b4, eq135_e1722_d_b5, eq135_e1722_d_b6, eq135_e1722_d_b7, eq135_e1722_d_b8, eq135_e1722_d_b9, eq135_e1722_d_b10, eq135_e1722_d_b11, eq135_e1722_d_b12, eq135_e1722_d_b13, eq135_e1722_d_b14, eq135_e1722_d_b15, eq135_e1722_d_b16, eq135_e1722_d_b17, eq135_e1722_d_b18, eq135_e1722_d_b19, eq135_e1722_d_b20, eq135_e1722_d_b21, eq135_e1722_d_b22, eq135_e1722_d_b23, eq135_e1722_d_b24, eq135_e1722_d_b25, eq135_e1722_d_b26, eq135_e1722_d_b27, eq135_e1722_d_b28, eq135_e1722_d_b29, eq135_e1722_d_b30, eq135_e1722_d_b31, eq135_e1722_d_b32, eq135_e1722_d_b33, eq135_e1722_d_b34, eq135_e1722_d_b35, eq135_e1722_d_b36, eq135_e1722_d_b37, eq135_e1722_d_b38, eq135_e1722_d_b39, eq135_e1722_d_b40, eq135_e1722_d_b41, eq135_e1722_d_b42, eq135_e1722_d_b43, eq135_e1722_d_b44, eq135_e1722_d_b45, eq135_e1722_d_b46, eq135_e1722_d_b47, eq135_e1722_d_b48, eq135_e1722_d_b49, eq135_e1722_d_b50, eq135_e1722_d_b51, eq135_e1722_d_b52, eq135_e1722_d_b53, eq135_e1722_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(19),
            multiplicity * (eq135_value),
            &eq135_node_derivatives,
            &eq135_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_21(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq136_e1736, eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22, eq136_e1736_d_b0, eq136_e1736_d_b1, eq136_e1736_d_b2, eq136_e1736_d_b3, eq136_e1736_d_b4, eq136_e1736_d_b5, eq136_e1736_d_b6, eq136_e1736_d_b7, eq136_e1736_d_b8, eq136_e1736_d_b9, eq136_e1736_d_b10, eq136_e1736_d_b11, eq136_e1736_d_b12, eq136_e1736_d_b13, eq136_e1736_d_b14, eq136_e1736_d_b15, eq136_e1736_d_b16, eq136_e1736_d_b17, eq136_e1736_d_b18, eq136_e1736_d_b19, eq136_e1736_d_b20, eq136_e1736_d_b21, eq136_e1736_d_b22, eq136_e1736_d_b23, eq136_e1736_d_b24, eq136_e1736_d_b25, eq136_e1736_d_b26, eq136_e1736_d_b27, eq136_e1736_d_b28, eq136_e1736_d_b29, eq136_e1736_d_b30, eq136_e1736_d_b31, eq136_e1736_d_b32, eq136_e1736_d_b33, eq136_e1736_d_b34, eq136_e1736_d_b35, eq136_e1736_d_b36, eq136_e1736_d_b37, eq136_e1736_d_b38, eq136_e1736_d_b39, eq136_e1736_d_b40, eq136_e1736_d_b41, eq136_e1736_d_b42, eq136_e1736_d_b43, eq136_e1736_d_b44, eq136_e1736_d_b45, eq136_e1736_d_b46, eq136_e1736_d_b47, eq136_e1736_d_b48, eq136_e1736_d_b49, eq136_e1736_d_b50, eq136_e1736_d_b51, eq136_e1736_d_b52, eq136_e1736_d_b53, eq136_e1736_d_b54,) = {
    if ((s.b[575] && s.b[576]) && (!s.b[577])) {
        let eq136_e1731: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 35, s.v[240]);
        let eq136_e1732: f64 = (p.p7 * eq136_e1731);
        let eq136_e1732_d_n0: f64 = (p.p7 * (s.dn[240][0] * ddt_scale));
        let eq136_e1732_d_n1: f64 = (p.p7 * (s.dn[240][1] * ddt_scale));
        let eq136_e1732_d_n2: f64 = (p.p7 * (s.dn[240][2] * ddt_scale));
        let eq136_e1732_d_n3: f64 = (p.p7 * (s.dn[240][3] * ddt_scale));
        let eq136_e1732_d_n4: f64 = (p.p7 * (s.dn[240][4] * ddt_scale));
        let eq136_e1732_d_n5: f64 = (p.p7 * (s.dn[240][5] * ddt_scale));
        let eq136_e1732_d_n6: f64 = (p.p7 * (s.dn[240][6] * ddt_scale));
        let eq136_e1732_d_n7: f64 = (p.p7 * (s.dn[240][7] * ddt_scale));
        let eq136_e1732_d_n8: f64 = (p.p7 * (s.dn[240][8] * ddt_scale));
        let eq136_e1732_d_n9: f64 = (p.p7 * (s.dn[240][9] * ddt_scale));
        let eq136_e1732_d_n10: f64 = (p.p7 * (s.dn[240][10] * ddt_scale));
        let eq136_e1732_d_n11: f64 = (p.p7 * (s.dn[240][11] * ddt_scale));
        let eq136_e1732_d_n12: f64 = (p.p7 * (s.dn[240][12] * ddt_scale));
        let eq136_e1732_d_n13: f64 = (p.p7 * (s.dn[240][13] * ddt_scale));
        let eq136_e1732_d_n14: f64 = (p.p7 * (s.dn[240][14] * ddt_scale));
        let eq136_e1732_d_n15: f64 = (p.p7 * (s.dn[240][15] * ddt_scale));
        let eq136_e1732_d_n16: f64 = (p.p7 * (s.dn[240][16] * ddt_scale));
        let eq136_e1732_d_n17: f64 = (p.p7 * (s.dn[240][17] * ddt_scale));
        let eq136_e1732_d_n18: f64 = (p.p7 * (s.dn[240][18] * ddt_scale));
        let eq136_e1732_d_n19: f64 = (p.p7 * (s.dn[240][19] * ddt_scale));
        let eq136_e1732_d_n20: f64 = (p.p7 * (s.dn[240][20] * ddt_scale));
        let eq136_e1732_d_n21: f64 = (p.p7 * (s.dn[240][21] * ddt_scale));
        let eq136_e1732_d_n22: f64 = (p.p7 * (s.dn[240][22] * ddt_scale));
        let eq136_e1732_d_b0: f64 = (p.p7 * (s.db[240][0] * ddt_scale));
        let eq136_e1732_d_b1: f64 = (p.p7 * (s.db[240][1] * ddt_scale));
        let eq136_e1732_d_b2: f64 = (p.p7 * (s.db[240][2] * ddt_scale));
        let eq136_e1732_d_b3: f64 = (p.p7 * (s.db[240][3] * ddt_scale));
        let eq136_e1732_d_b4: f64 = (p.p7 * (s.db[240][4] * ddt_scale));
        let eq136_e1732_d_b5: f64 = (p.p7 * (s.db[240][5] * ddt_scale));
        let eq136_e1732_d_b6: f64 = (p.p7 * (s.db[240][6] * ddt_scale));
        let eq136_e1732_d_b7: f64 = (p.p7 * (s.db[240][7] * ddt_scale));
        let eq136_e1732_d_b8: f64 = (p.p7 * (s.db[240][8] * ddt_scale));
        let eq136_e1732_d_b9: f64 = (p.p7 * (s.db[240][9] * ddt_scale));
        let eq136_e1732_d_b10: f64 = (p.p7 * (s.db[240][10] * ddt_scale));
        let eq136_e1732_d_b11: f64 = (p.p7 * (s.db[240][11] * ddt_scale));
        let eq136_e1732_d_b12: f64 = (p.p7 * (s.db[240][12] * ddt_scale));
        let eq136_e1732_d_b13: f64 = (p.p7 * (s.db[240][13] * ddt_scale));
        let eq136_e1732_d_b14: f64 = (p.p7 * (s.db[240][14] * ddt_scale));
        let eq136_e1732_d_b15: f64 = (p.p7 * (s.db[240][15] * ddt_scale));
        let eq136_e1732_d_b16: f64 = (p.p7 * (s.db[240][16] * ddt_scale));
        let eq136_e1732_d_b17: f64 = (p.p7 * (s.db[240][17] * ddt_scale));
        let eq136_e1732_d_b18: f64 = (p.p7 * (s.db[240][18] * ddt_scale));
        let eq136_e1732_d_b19: f64 = (p.p7 * (s.db[240][19] * ddt_scale));
        let eq136_e1732_d_b20: f64 = (p.p7 * (s.db[240][20] * ddt_scale));
        let eq136_e1732_d_b21: f64 = (p.p7 * (s.db[240][21] * ddt_scale));
        let eq136_e1732_d_b22: f64 = (p.p7 * (s.db[240][22] * ddt_scale));
        let eq136_e1732_d_b23: f64 = (p.p7 * (s.db[240][23] * ddt_scale));
        let eq136_e1732_d_b24: f64 = (p.p7 * (s.db[240][24] * ddt_scale));
        let eq136_e1732_d_b25: f64 = (p.p7 * (s.db[240][25] * ddt_scale));
        let eq136_e1732_d_b26: f64 = (p.p7 * (s.db[240][26] * ddt_scale));
        let eq136_e1732_d_b27: f64 = (p.p7 * (s.db[240][27] * ddt_scale));
        let eq136_e1732_d_b28: f64 = (p.p7 * (s.db[240][28] * ddt_scale));
        let eq136_e1732_d_b29: f64 = (p.p7 * (s.db[240][29] * ddt_scale));
        let eq136_e1732_d_b30: f64 = (p.p7 * (s.db[240][30] * ddt_scale));
        let eq136_e1732_d_b31: f64 = (p.p7 * (s.db[240][31] * ddt_scale));
        let eq136_e1732_d_b32: f64 = (p.p7 * (s.db[240][32] * ddt_scale));
        let eq136_e1732_d_b33: f64 = (p.p7 * (s.db[240][33] * ddt_scale));
        let eq136_e1732_d_b34: f64 = (p.p7 * (s.db[240][34] * ddt_scale));
        let eq136_e1732_d_b35: f64 = (p.p7 * (s.db[240][35] * ddt_scale));
        let eq136_e1732_d_b36: f64 = (p.p7 * (s.db[240][36] * ddt_scale));
        let eq136_e1732_d_b37: f64 = (p.p7 * (s.db[240][37] * ddt_scale));
        let eq136_e1732_d_b38: f64 = (p.p7 * (s.db[240][38] * ddt_scale));
        let eq136_e1732_d_b39: f64 = (p.p7 * (s.db[240][39] * ddt_scale));
        let eq136_e1732_d_b40: f64 = (p.p7 * (s.db[240][40] * ddt_scale));
        let eq136_e1732_d_b41: f64 = (p.p7 * (s.db[240][41] * ddt_scale));
        let eq136_e1732_d_b42: f64 = (p.p7 * (s.db[240][42] * ddt_scale));
        let eq136_e1732_d_b43: f64 = (p.p7 * (s.db[240][43] * ddt_scale));
        let eq136_e1732_d_b44: f64 = (p.p7 * (s.db[240][44] * ddt_scale));
        let eq136_e1732_d_b45: f64 = (p.p7 * (s.db[240][45] * ddt_scale));
        let eq136_e1732_d_b46: f64 = (p.p7 * (s.db[240][46] * ddt_scale));
        let eq136_e1732_d_b47: f64 = (p.p7 * (s.db[240][47] * ddt_scale));
        let eq136_e1732_d_b48: f64 = (p.p7 * (s.db[240][48] * ddt_scale));
        let eq136_e1732_d_b49: f64 = (p.p7 * (s.db[240][49] * ddt_scale));
        let eq136_e1732_d_b50: f64 = (p.p7 * (s.db[240][50] * ddt_scale));
        let eq136_e1732_d_b51: f64 = (p.p7 * (s.db[240][51] * ddt_scale));
        let eq136_e1732_d_b52: f64 = (p.p7 * (s.db[240][52] * ddt_scale));
        let eq136_e1732_d_b53: f64 = (p.p7 * (s.db[240][53] * ddt_scale));
        let eq136_e1732_d_b54: f64 = (p.p7 * (s.db[240][54] * ddt_scale));
        let eq136_e1734: f64 = (eq136_e1732 * p.p246);
        let eq136_e1734_d_n0: f64 = (eq136_e1732_d_n0 * p.p246);
        let eq136_e1734_d_n1: f64 = (eq136_e1732_d_n1 * p.p246);
        let eq136_e1734_d_n2: f64 = (eq136_e1732_d_n2 * p.p246);
        let eq136_e1734_d_n3: f64 = (eq136_e1732_d_n3 * p.p246);
        let eq136_e1734_d_n4: f64 = (eq136_e1732_d_n4 * p.p246);
        let eq136_e1734_d_n5: f64 = (eq136_e1732_d_n5 * p.p246);
        let eq136_e1734_d_n6: f64 = (eq136_e1732_d_n6 * p.p246);
        let eq136_e1734_d_n7: f64 = (eq136_e1732_d_n7 * p.p246);
        let eq136_e1734_d_n8: f64 = (eq136_e1732_d_n8 * p.p246);
        let eq136_e1734_d_n9: f64 = (eq136_e1732_d_n9 * p.p246);
        let eq136_e1734_d_n10: f64 = (eq136_e1732_d_n10 * p.p246);
        let eq136_e1734_d_n11: f64 = (eq136_e1732_d_n11 * p.p246);
        let eq136_e1734_d_n12: f64 = (eq136_e1732_d_n12 * p.p246);
        let eq136_e1734_d_n13: f64 = (eq136_e1732_d_n13 * p.p246);
        let eq136_e1734_d_n14: f64 = (eq136_e1732_d_n14 * p.p246);
        let eq136_e1734_d_n15: f64 = (eq136_e1732_d_n15 * p.p246);
        let eq136_e1734_d_n16: f64 = (eq136_e1732_d_n16 * p.p246);
        let eq136_e1734_d_n17: f64 = (eq136_e1732_d_n17 * p.p246);
        let eq136_e1734_d_n18: f64 = (eq136_e1732_d_n18 * p.p246);
        let eq136_e1734_d_n19: f64 = (eq136_e1732_d_n19 * p.p246);
        let eq136_e1734_d_n20: f64 = (eq136_e1732_d_n20 * p.p246);
        let eq136_e1734_d_n21: f64 = (eq136_e1732_d_n21 * p.p246);
        let eq136_e1734_d_n22: f64 = (eq136_e1732_d_n22 * p.p246);
        let eq136_e1734_d_b0: f64 = (eq136_e1732_d_b0 * p.p246);
        let eq136_e1734_d_b1: f64 = (eq136_e1732_d_b1 * p.p246);
        let eq136_e1734_d_b2: f64 = (eq136_e1732_d_b2 * p.p246);
        let eq136_e1734_d_b3: f64 = (eq136_e1732_d_b3 * p.p246);
        let eq136_e1734_d_b4: f64 = (eq136_e1732_d_b4 * p.p246);
        let eq136_e1734_d_b5: f64 = (eq136_e1732_d_b5 * p.p246);
        let eq136_e1734_d_b6: f64 = (eq136_e1732_d_b6 * p.p246);
        let eq136_e1734_d_b7: f64 = (eq136_e1732_d_b7 * p.p246);
        let eq136_e1734_d_b8: f64 = (eq136_e1732_d_b8 * p.p246);
        let eq136_e1734_d_b9: f64 = (eq136_e1732_d_b9 * p.p246);
        let eq136_e1734_d_b10: f64 = (eq136_e1732_d_b10 * p.p246);
        let eq136_e1734_d_b11: f64 = (eq136_e1732_d_b11 * p.p246);
        let eq136_e1734_d_b12: f64 = (eq136_e1732_d_b12 * p.p246);
        let eq136_e1734_d_b13: f64 = (eq136_e1732_d_b13 * p.p246);
        let eq136_e1734_d_b14: f64 = (eq136_e1732_d_b14 * p.p246);
        let eq136_e1734_d_b15: f64 = (eq136_e1732_d_b15 * p.p246);
        let eq136_e1734_d_b16: f64 = (eq136_e1732_d_b16 * p.p246);
        let eq136_e1734_d_b17: f64 = (eq136_e1732_d_b17 * p.p246);
        let eq136_e1734_d_b18: f64 = (eq136_e1732_d_b18 * p.p246);
        let eq136_e1734_d_b19: f64 = (eq136_e1732_d_b19 * p.p246);
        let eq136_e1734_d_b20: f64 = (eq136_e1732_d_b20 * p.p246);
        let eq136_e1734_d_b21: f64 = (eq136_e1732_d_b21 * p.p246);
        let eq136_e1734_d_b22: f64 = (eq136_e1732_d_b22 * p.p246);
        let eq136_e1734_d_b23: f64 = (eq136_e1732_d_b23 * p.p246);
        let eq136_e1734_d_b24: f64 = (eq136_e1732_d_b24 * p.p246);
        let eq136_e1734_d_b25: f64 = (eq136_e1732_d_b25 * p.p246);
        let eq136_e1734_d_b26: f64 = (eq136_e1732_d_b26 * p.p246);
        let eq136_e1734_d_b27: f64 = (eq136_e1732_d_b27 * p.p246);
        let eq136_e1734_d_b28: f64 = (eq136_e1732_d_b28 * p.p246);
        let eq136_e1734_d_b29: f64 = (eq136_e1732_d_b29 * p.p246);
        let eq136_e1734_d_b30: f64 = (eq136_e1732_d_b30 * p.p246);
        let eq136_e1734_d_b31: f64 = (eq136_e1732_d_b31 * p.p246);
        let eq136_e1734_d_b32: f64 = (eq136_e1732_d_b32 * p.p246);
        let eq136_e1734_d_b33: f64 = (eq136_e1732_d_b33 * p.p246);
        let eq136_e1734_d_b34: f64 = (eq136_e1732_d_b34 * p.p246);
        let eq136_e1734_d_b35: f64 = (eq136_e1732_d_b35 * p.p246);
        let eq136_e1734_d_b36: f64 = (eq136_e1732_d_b36 * p.p246);
        let eq136_e1734_d_b37: f64 = (eq136_e1732_d_b37 * p.p246);
        let eq136_e1734_d_b38: f64 = (eq136_e1732_d_b38 * p.p246);
        let eq136_e1734_d_b39: f64 = (eq136_e1732_d_b39 * p.p246);
        let eq136_e1734_d_b40: f64 = (eq136_e1732_d_b40 * p.p246);
        let eq136_e1734_d_b41: f64 = (eq136_e1732_d_b41 * p.p246);
        let eq136_e1734_d_b42: f64 = (eq136_e1732_d_b42 * p.p246);
        let eq136_e1734_d_b43: f64 = (eq136_e1732_d_b43 * p.p246);
        let eq136_e1734_d_b44: f64 = (eq136_e1732_d_b44 * p.p246);
        let eq136_e1734_d_b45: f64 = (eq136_e1732_d_b45 * p.p246);
        let eq136_e1734_d_b46: f64 = (eq136_e1732_d_b46 * p.p246);
        let eq136_e1734_d_b47: f64 = (eq136_e1732_d_b47 * p.p246);
        let eq136_e1734_d_b48: f64 = (eq136_e1732_d_b48 * p.p246);
        let eq136_e1734_d_b49: f64 = (eq136_e1732_d_b49 * p.p246);
        let eq136_e1734_d_b50: f64 = (eq136_e1732_d_b50 * p.p246);
        let eq136_e1734_d_b51: f64 = (eq136_e1732_d_b51 * p.p246);
        let eq136_e1734_d_b52: f64 = (eq136_e1732_d_b52 * p.p246);
        let eq136_e1734_d_b53: f64 = (eq136_e1732_d_b53 * p.p246);
        let eq136_e1734_d_b54: f64 = (eq136_e1732_d_b54 * p.p246);
        (eq136_e1734, eq136_e1734_d_n0, eq136_e1734_d_n1, eq136_e1734_d_n2, eq136_e1734_d_n3, eq136_e1734_d_n4, eq136_e1734_d_n5, eq136_e1734_d_n6, eq136_e1734_d_n7, eq136_e1734_d_n8, eq136_e1734_d_n9, eq136_e1734_d_n10, eq136_e1734_d_n11, eq136_e1734_d_n12, eq136_e1734_d_n13, eq136_e1734_d_n14, eq136_e1734_d_n15, eq136_e1734_d_n16, eq136_e1734_d_n17, eq136_e1734_d_n18, eq136_e1734_d_n19, eq136_e1734_d_n20, eq136_e1734_d_n21, eq136_e1734_d_n22, eq136_e1734_d_b0, eq136_e1734_d_b1, eq136_e1734_d_b2, eq136_e1734_d_b3, eq136_e1734_d_b4, eq136_e1734_d_b5, eq136_e1734_d_b6, eq136_e1734_d_b7, eq136_e1734_d_b8, eq136_e1734_d_b9, eq136_e1734_d_b10, eq136_e1734_d_b11, eq136_e1734_d_b12, eq136_e1734_d_b13, eq136_e1734_d_b14, eq136_e1734_d_b15, eq136_e1734_d_b16, eq136_e1734_d_b17, eq136_e1734_d_b18, eq136_e1734_d_b19, eq136_e1734_d_b20, eq136_e1734_d_b21, eq136_e1734_d_b22, eq136_e1734_d_b23, eq136_e1734_d_b24, eq136_e1734_d_b25, eq136_e1734_d_b26, eq136_e1734_d_b27, eq136_e1734_d_b28, eq136_e1734_d_b29, eq136_e1734_d_b30, eq136_e1734_d_b31, eq136_e1734_d_b32, eq136_e1734_d_b33, eq136_e1734_d_b34, eq136_e1734_d_b35, eq136_e1734_d_b36, eq136_e1734_d_b37, eq136_e1734_d_b38, eq136_e1734_d_b39, eq136_e1734_d_b40, eq136_e1734_d_b41, eq136_e1734_d_b42, eq136_e1734_d_b43, eq136_e1734_d_b44, eq136_e1734_d_b45, eq136_e1734_d_b46, eq136_e1734_d_b47, eq136_e1734_d_b48, eq136_e1734_d_b49, eq136_e1734_d_b50, eq136_e1734_d_b51, eq136_e1734_d_b52, eq136_e1734_d_b53, eq136_e1734_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq136_value: f64 = eq136_e1736;
        let eq136_node_derivatives: [f64; 23] = [eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22];
        let eq136_branch_derivatives: [f64; 55] = [eq136_e1736_d_b0, eq136_e1736_d_b1, eq136_e1736_d_b2, eq136_e1736_d_b3, eq136_e1736_d_b4, eq136_e1736_d_b5, eq136_e1736_d_b6, eq136_e1736_d_b7, eq136_e1736_d_b8, eq136_e1736_d_b9, eq136_e1736_d_b10, eq136_e1736_d_b11, eq136_e1736_d_b12, eq136_e1736_d_b13, eq136_e1736_d_b14, eq136_e1736_d_b15, eq136_e1736_d_b16, eq136_e1736_d_b17, eq136_e1736_d_b18, eq136_e1736_d_b19, eq136_e1736_d_b20, eq136_e1736_d_b21, eq136_e1736_d_b22, eq136_e1736_d_b23, eq136_e1736_d_b24, eq136_e1736_d_b25, eq136_e1736_d_b26, eq136_e1736_d_b27, eq136_e1736_d_b28, eq136_e1736_d_b29, eq136_e1736_d_b30, eq136_e1736_d_b31, eq136_e1736_d_b32, eq136_e1736_d_b33, eq136_e1736_d_b34, eq136_e1736_d_b35, eq136_e1736_d_b36, eq136_e1736_d_b37, eq136_e1736_d_b38, eq136_e1736_d_b39, eq136_e1736_d_b40, eq136_e1736_d_b41, eq136_e1736_d_b42, eq136_e1736_d_b43, eq136_e1736_d_b44, eq136_e1736_d_b45, eq136_e1736_d_b46, eq136_e1736_d_b47, eq136_e1736_d_b48, eq136_e1736_d_b49, eq136_e1736_d_b50, eq136_e1736_d_b51, eq136_e1736_d_b52, eq136_e1736_d_b53, eq136_e1736_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(19),
            multiplicity * (eq136_value),
            &eq136_node_derivatives,
            &eq136_branch_derivatives,
            multiplicity,
        );
        let (eq137_e1747, eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22, eq137_e1747_d_b0, eq137_e1747_d_b1, eq137_e1747_d_b2, eq137_e1747_d_b3, eq137_e1747_d_b4, eq137_e1747_d_b5, eq137_e1747_d_b6, eq137_e1747_d_b7, eq137_e1747_d_b8, eq137_e1747_d_b9, eq137_e1747_d_b10, eq137_e1747_d_b11, eq137_e1747_d_b12, eq137_e1747_d_b13, eq137_e1747_d_b14, eq137_e1747_d_b15, eq137_e1747_d_b16, eq137_e1747_d_b17, eq137_e1747_d_b18, eq137_e1747_d_b19, eq137_e1747_d_b20, eq137_e1747_d_b21, eq137_e1747_d_b22, eq137_e1747_d_b23, eq137_e1747_d_b24, eq137_e1747_d_b25, eq137_e1747_d_b26, eq137_e1747_d_b27, eq137_e1747_d_b28, eq137_e1747_d_b29, eq137_e1747_d_b30, eq137_e1747_d_b31, eq137_e1747_d_b32, eq137_e1747_d_b33, eq137_e1747_d_b34, eq137_e1747_d_b35, eq137_e1747_d_b36, eq137_e1747_d_b37, eq137_e1747_d_b38, eq137_e1747_d_b39, eq137_e1747_d_b40, eq137_e1747_d_b41, eq137_e1747_d_b42, eq137_e1747_d_b43, eq137_e1747_d_b44, eq137_e1747_d_b45, eq137_e1747_d_b46, eq137_e1747_d_b47, eq137_e1747_d_b48, eq137_e1747_d_b49, eq137_e1747_d_b50, eq137_e1747_d_b51, eq137_e1747_d_b52, eq137_e1747_d_b53, eq137_e1747_d_b54,) = {
    if (s.b[575] && s.b[576]) {
        let eq137_e1743: f64 = (p.p251 * s.v[240]);
        let eq137_e1744: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 36, eq137_e1743);
        let eq137_e1744_d_n0: f64 = ((p.p251 * s.dn[240][0]) * ddt_scale);
        let eq137_e1744_d_n1: f64 = ((p.p251 * s.dn[240][1]) * ddt_scale);
        let eq137_e1744_d_n2: f64 = ((p.p251 * s.dn[240][2]) * ddt_scale);
        let eq137_e1744_d_n3: f64 = ((p.p251 * s.dn[240][3]) * ddt_scale);
        let eq137_e1744_d_n4: f64 = ((p.p251 * s.dn[240][4]) * ddt_scale);
        let eq137_e1744_d_n5: f64 = ((p.p251 * s.dn[240][5]) * ddt_scale);
        let eq137_e1744_d_n6: f64 = ((p.p251 * s.dn[240][6]) * ddt_scale);
        let eq137_e1744_d_n7: f64 = ((p.p251 * s.dn[240][7]) * ddt_scale);
        let eq137_e1744_d_n8: f64 = ((p.p251 * s.dn[240][8]) * ddt_scale);
        let eq137_e1744_d_n9: f64 = ((p.p251 * s.dn[240][9]) * ddt_scale);
        let eq137_e1744_d_n10: f64 = ((p.p251 * s.dn[240][10]) * ddt_scale);
        let eq137_e1744_d_n11: f64 = ((p.p251 * s.dn[240][11]) * ddt_scale);
        let eq137_e1744_d_n12: f64 = ((p.p251 * s.dn[240][12]) * ddt_scale);
        let eq137_e1744_d_n13: f64 = ((p.p251 * s.dn[240][13]) * ddt_scale);
        let eq137_e1744_d_n14: f64 = ((p.p251 * s.dn[240][14]) * ddt_scale);
        let eq137_e1744_d_n15: f64 = ((p.p251 * s.dn[240][15]) * ddt_scale);
        let eq137_e1744_d_n16: f64 = ((p.p251 * s.dn[240][16]) * ddt_scale);
        let eq137_e1744_d_n17: f64 = ((p.p251 * s.dn[240][17]) * ddt_scale);
        let eq137_e1744_d_n18: f64 = ((p.p251 * s.dn[240][18]) * ddt_scale);
        let eq137_e1744_d_n19: f64 = ((p.p251 * s.dn[240][19]) * ddt_scale);
        let eq137_e1744_d_n20: f64 = ((p.p251 * s.dn[240][20]) * ddt_scale);
        let eq137_e1744_d_n21: f64 = ((p.p251 * s.dn[240][21]) * ddt_scale);
        let eq137_e1744_d_n22: f64 = ((p.p251 * s.dn[240][22]) * ddt_scale);
        let eq137_e1744_d_b0: f64 = ((p.p251 * s.db[240][0]) * ddt_scale);
        let eq137_e1744_d_b1: f64 = ((p.p251 * s.db[240][1]) * ddt_scale);
        let eq137_e1744_d_b2: f64 = ((p.p251 * s.db[240][2]) * ddt_scale);
        let eq137_e1744_d_b3: f64 = ((p.p251 * s.db[240][3]) * ddt_scale);
        let eq137_e1744_d_b4: f64 = ((p.p251 * s.db[240][4]) * ddt_scale);
        let eq137_e1744_d_b5: f64 = ((p.p251 * s.db[240][5]) * ddt_scale);
        let eq137_e1744_d_b6: f64 = ((p.p251 * s.db[240][6]) * ddt_scale);
        let eq137_e1744_d_b7: f64 = ((p.p251 * s.db[240][7]) * ddt_scale);
        let eq137_e1744_d_b8: f64 = ((p.p251 * s.db[240][8]) * ddt_scale);
        let eq137_e1744_d_b9: f64 = ((p.p251 * s.db[240][9]) * ddt_scale);
        let eq137_e1744_d_b10: f64 = ((p.p251 * s.db[240][10]) * ddt_scale);
        let eq137_e1744_d_b11: f64 = ((p.p251 * s.db[240][11]) * ddt_scale);
        let eq137_e1744_d_b12: f64 = ((p.p251 * s.db[240][12]) * ddt_scale);
        let eq137_e1744_d_b13: f64 = ((p.p251 * s.db[240][13]) * ddt_scale);
        let eq137_e1744_d_b14: f64 = ((p.p251 * s.db[240][14]) * ddt_scale);
        let eq137_e1744_d_b15: f64 = ((p.p251 * s.db[240][15]) * ddt_scale);
        let eq137_e1744_d_b16: f64 = ((p.p251 * s.db[240][16]) * ddt_scale);
        let eq137_e1744_d_b17: f64 = ((p.p251 * s.db[240][17]) * ddt_scale);
        let eq137_e1744_d_b18: f64 = ((p.p251 * s.db[240][18]) * ddt_scale);
        let eq137_e1744_d_b19: f64 = ((p.p251 * s.db[240][19]) * ddt_scale);
        let eq137_e1744_d_b20: f64 = ((p.p251 * s.db[240][20]) * ddt_scale);
        let eq137_e1744_d_b21: f64 = ((p.p251 * s.db[240][21]) * ddt_scale);
        let eq137_e1744_d_b22: f64 = ((p.p251 * s.db[240][22]) * ddt_scale);
        let eq137_e1744_d_b23: f64 = ((p.p251 * s.db[240][23]) * ddt_scale);
        let eq137_e1744_d_b24: f64 = ((p.p251 * s.db[240][24]) * ddt_scale);
        let eq137_e1744_d_b25: f64 = ((p.p251 * s.db[240][25]) * ddt_scale);
        let eq137_e1744_d_b26: f64 = ((p.p251 * s.db[240][26]) * ddt_scale);
        let eq137_e1744_d_b27: f64 = ((p.p251 * s.db[240][27]) * ddt_scale);
        let eq137_e1744_d_b28: f64 = ((p.p251 * s.db[240][28]) * ddt_scale);
        let eq137_e1744_d_b29: f64 = ((p.p251 * s.db[240][29]) * ddt_scale);
        let eq137_e1744_d_b30: f64 = ((p.p251 * s.db[240][30]) * ddt_scale);
        let eq137_e1744_d_b31: f64 = ((p.p251 * s.db[240][31]) * ddt_scale);
        let eq137_e1744_d_b32: f64 = ((p.p251 * s.db[240][32]) * ddt_scale);
        let eq137_e1744_d_b33: f64 = ((p.p251 * s.db[240][33]) * ddt_scale);
        let eq137_e1744_d_b34: f64 = ((p.p251 * s.db[240][34]) * ddt_scale);
        let eq137_e1744_d_b35: f64 = ((p.p251 * s.db[240][35]) * ddt_scale);
        let eq137_e1744_d_b36: f64 = ((p.p251 * s.db[240][36]) * ddt_scale);
        let eq137_e1744_d_b37: f64 = ((p.p251 * s.db[240][37]) * ddt_scale);
        let eq137_e1744_d_b38: f64 = ((p.p251 * s.db[240][38]) * ddt_scale);
        let eq137_e1744_d_b39: f64 = ((p.p251 * s.db[240][39]) * ddt_scale);
        let eq137_e1744_d_b40: f64 = ((p.p251 * s.db[240][40]) * ddt_scale);
        let eq137_e1744_d_b41: f64 = ((p.p251 * s.db[240][41]) * ddt_scale);
        let eq137_e1744_d_b42: f64 = ((p.p251 * s.db[240][42]) * ddt_scale);
        let eq137_e1744_d_b43: f64 = ((p.p251 * s.db[240][43]) * ddt_scale);
        let eq137_e1744_d_b44: f64 = ((p.p251 * s.db[240][44]) * ddt_scale);
        let eq137_e1744_d_b45: f64 = ((p.p251 * s.db[240][45]) * ddt_scale);
        let eq137_e1744_d_b46: f64 = ((p.p251 * s.db[240][46]) * ddt_scale);
        let eq137_e1744_d_b47: f64 = ((p.p251 * s.db[240][47]) * ddt_scale);
        let eq137_e1744_d_b48: f64 = ((p.p251 * s.db[240][48]) * ddt_scale);
        let eq137_e1744_d_b49: f64 = ((p.p251 * s.db[240][49]) * ddt_scale);
        let eq137_e1744_d_b50: f64 = ((p.p251 * s.db[240][50]) * ddt_scale);
        let eq137_e1744_d_b51: f64 = ((p.p251 * s.db[240][51]) * ddt_scale);
        let eq137_e1744_d_b52: f64 = ((p.p251 * s.db[240][52]) * ddt_scale);
        let eq137_e1744_d_b53: f64 = ((p.p251 * s.db[240][53]) * ddt_scale);
        let eq137_e1744_d_b54: f64 = ((p.p251 * s.db[240][54]) * ddt_scale);
        let eq137_e1745: f64 = (p.p7 * eq137_e1744);
        let eq137_e1745_d_n0: f64 = (p.p7 * eq137_e1744_d_n0);
        let eq137_e1745_d_n1: f64 = (p.p7 * eq137_e1744_d_n1);
        let eq137_e1745_d_n2: f64 = (p.p7 * eq137_e1744_d_n2);
        let eq137_e1745_d_n3: f64 = (p.p7 * eq137_e1744_d_n3);
        let eq137_e1745_d_n4: f64 = (p.p7 * eq137_e1744_d_n4);
        let eq137_e1745_d_n5: f64 = (p.p7 * eq137_e1744_d_n5);
        let eq137_e1745_d_n6: f64 = (p.p7 * eq137_e1744_d_n6);
        let eq137_e1745_d_n7: f64 = (p.p7 * eq137_e1744_d_n7);
        let eq137_e1745_d_n8: f64 = (p.p7 * eq137_e1744_d_n8);
        let eq137_e1745_d_n9: f64 = (p.p7 * eq137_e1744_d_n9);
        let eq137_e1745_d_n10: f64 = (p.p7 * eq137_e1744_d_n10);
        let eq137_e1745_d_n11: f64 = (p.p7 * eq137_e1744_d_n11);
        let eq137_e1745_d_n12: f64 = (p.p7 * eq137_e1744_d_n12);
        let eq137_e1745_d_n13: f64 = (p.p7 * eq137_e1744_d_n13);
        let eq137_e1745_d_n14: f64 = (p.p7 * eq137_e1744_d_n14);
        let eq137_e1745_d_n15: f64 = (p.p7 * eq137_e1744_d_n15);
        let eq137_e1745_d_n16: f64 = (p.p7 * eq137_e1744_d_n16);
        let eq137_e1745_d_n17: f64 = (p.p7 * eq137_e1744_d_n17);
        let eq137_e1745_d_n18: f64 = (p.p7 * eq137_e1744_d_n18);
        let eq137_e1745_d_n19: f64 = (p.p7 * eq137_e1744_d_n19);
        let eq137_e1745_d_n20: f64 = (p.p7 * eq137_e1744_d_n20);
        let eq137_e1745_d_n21: f64 = (p.p7 * eq137_e1744_d_n21);
        let eq137_e1745_d_n22: f64 = (p.p7 * eq137_e1744_d_n22);
        let eq137_e1745_d_b0: f64 = (p.p7 * eq137_e1744_d_b0);
        let eq137_e1745_d_b1: f64 = (p.p7 * eq137_e1744_d_b1);
        let eq137_e1745_d_b2: f64 = (p.p7 * eq137_e1744_d_b2);
        let eq137_e1745_d_b3: f64 = (p.p7 * eq137_e1744_d_b3);
        let eq137_e1745_d_b4: f64 = (p.p7 * eq137_e1744_d_b4);
        let eq137_e1745_d_b5: f64 = (p.p7 * eq137_e1744_d_b5);
        let eq137_e1745_d_b6: f64 = (p.p7 * eq137_e1744_d_b6);
        let eq137_e1745_d_b7: f64 = (p.p7 * eq137_e1744_d_b7);
        let eq137_e1745_d_b8: f64 = (p.p7 * eq137_e1744_d_b8);
        let eq137_e1745_d_b9: f64 = (p.p7 * eq137_e1744_d_b9);
        let eq137_e1745_d_b10: f64 = (p.p7 * eq137_e1744_d_b10);
        let eq137_e1745_d_b11: f64 = (p.p7 * eq137_e1744_d_b11);
        let eq137_e1745_d_b12: f64 = (p.p7 * eq137_e1744_d_b12);
        let eq137_e1745_d_b13: f64 = (p.p7 * eq137_e1744_d_b13);
        let eq137_e1745_d_b14: f64 = (p.p7 * eq137_e1744_d_b14);
        let eq137_e1745_d_b15: f64 = (p.p7 * eq137_e1744_d_b15);
        let eq137_e1745_d_b16: f64 = (p.p7 * eq137_e1744_d_b16);
        let eq137_e1745_d_b17: f64 = (p.p7 * eq137_e1744_d_b17);
        let eq137_e1745_d_b18: f64 = (p.p7 * eq137_e1744_d_b18);
        let eq137_e1745_d_b19: f64 = (p.p7 * eq137_e1744_d_b19);
        let eq137_e1745_d_b20: f64 = (p.p7 * eq137_e1744_d_b20);
        let eq137_e1745_d_b21: f64 = (p.p7 * eq137_e1744_d_b21);
        let eq137_e1745_d_b22: f64 = (p.p7 * eq137_e1744_d_b22);
        let eq137_e1745_d_b23: f64 = (p.p7 * eq137_e1744_d_b23);
        let eq137_e1745_d_b24: f64 = (p.p7 * eq137_e1744_d_b24);
        let eq137_e1745_d_b25: f64 = (p.p7 * eq137_e1744_d_b25);
        let eq137_e1745_d_b26: f64 = (p.p7 * eq137_e1744_d_b26);
        let eq137_e1745_d_b27: f64 = (p.p7 * eq137_e1744_d_b27);
        let eq137_e1745_d_b28: f64 = (p.p7 * eq137_e1744_d_b28);
        let eq137_e1745_d_b29: f64 = (p.p7 * eq137_e1744_d_b29);
        let eq137_e1745_d_b30: f64 = (p.p7 * eq137_e1744_d_b30);
        let eq137_e1745_d_b31: f64 = (p.p7 * eq137_e1744_d_b31);
        let eq137_e1745_d_b32: f64 = (p.p7 * eq137_e1744_d_b32);
        let eq137_e1745_d_b33: f64 = (p.p7 * eq137_e1744_d_b33);
        let eq137_e1745_d_b34: f64 = (p.p7 * eq137_e1744_d_b34);
        let eq137_e1745_d_b35: f64 = (p.p7 * eq137_e1744_d_b35);
        let eq137_e1745_d_b36: f64 = (p.p7 * eq137_e1744_d_b36);
        let eq137_e1745_d_b37: f64 = (p.p7 * eq137_e1744_d_b37);
        let eq137_e1745_d_b38: f64 = (p.p7 * eq137_e1744_d_b38);
        let eq137_e1745_d_b39: f64 = (p.p7 * eq137_e1744_d_b39);
        let eq137_e1745_d_b40: f64 = (p.p7 * eq137_e1744_d_b40);
        let eq137_e1745_d_b41: f64 = (p.p7 * eq137_e1744_d_b41);
        let eq137_e1745_d_b42: f64 = (p.p7 * eq137_e1744_d_b42);
        let eq137_e1745_d_b43: f64 = (p.p7 * eq137_e1744_d_b43);
        let eq137_e1745_d_b44: f64 = (p.p7 * eq137_e1744_d_b44);
        let eq137_e1745_d_b45: f64 = (p.p7 * eq137_e1744_d_b45);
        let eq137_e1745_d_b46: f64 = (p.p7 * eq137_e1744_d_b46);
        let eq137_e1745_d_b47: f64 = (p.p7 * eq137_e1744_d_b47);
        let eq137_e1745_d_b48: f64 = (p.p7 * eq137_e1744_d_b48);
        let eq137_e1745_d_b49: f64 = (p.p7 * eq137_e1744_d_b49);
        let eq137_e1745_d_b50: f64 = (p.p7 * eq137_e1744_d_b50);
        let eq137_e1745_d_b51: f64 = (p.p7 * eq137_e1744_d_b51);
        let eq137_e1745_d_b52: f64 = (p.p7 * eq137_e1744_d_b52);
        let eq137_e1745_d_b53: f64 = (p.p7 * eq137_e1744_d_b53);
        let eq137_e1745_d_b54: f64 = (p.p7 * eq137_e1744_d_b54);
        (eq137_e1745, eq137_e1745_d_n0, eq137_e1745_d_n1, eq137_e1745_d_n2, eq137_e1745_d_n3, eq137_e1745_d_n4, eq137_e1745_d_n5, eq137_e1745_d_n6, eq137_e1745_d_n7, eq137_e1745_d_n8, eq137_e1745_d_n9, eq137_e1745_d_n10, eq137_e1745_d_n11, eq137_e1745_d_n12, eq137_e1745_d_n13, eq137_e1745_d_n14, eq137_e1745_d_n15, eq137_e1745_d_n16, eq137_e1745_d_n17, eq137_e1745_d_n18, eq137_e1745_d_n19, eq137_e1745_d_n20, eq137_e1745_d_n21, eq137_e1745_d_n22, eq137_e1745_d_b0, eq137_e1745_d_b1, eq137_e1745_d_b2, eq137_e1745_d_b3, eq137_e1745_d_b4, eq137_e1745_d_b5, eq137_e1745_d_b6, eq137_e1745_d_b7, eq137_e1745_d_b8, eq137_e1745_d_b9, eq137_e1745_d_b10, eq137_e1745_d_b11, eq137_e1745_d_b12, eq137_e1745_d_b13, eq137_e1745_d_b14, eq137_e1745_d_b15, eq137_e1745_d_b16, eq137_e1745_d_b17, eq137_e1745_d_b18, eq137_e1745_d_b19, eq137_e1745_d_b20, eq137_e1745_d_b21, eq137_e1745_d_b22, eq137_e1745_d_b23, eq137_e1745_d_b24, eq137_e1745_d_b25, eq137_e1745_d_b26, eq137_e1745_d_b27, eq137_e1745_d_b28, eq137_e1745_d_b29, eq137_e1745_d_b30, eq137_e1745_d_b31, eq137_e1745_d_b32, eq137_e1745_d_b33, eq137_e1745_d_b34, eq137_e1745_d_b35, eq137_e1745_d_b36, eq137_e1745_d_b37, eq137_e1745_d_b38, eq137_e1745_d_b39, eq137_e1745_d_b40, eq137_e1745_d_b41, eq137_e1745_d_b42, eq137_e1745_d_b43, eq137_e1745_d_b44, eq137_e1745_d_b45, eq137_e1745_d_b46, eq137_e1745_d_b47, eq137_e1745_d_b48, eq137_e1745_d_b49, eq137_e1745_d_b50, eq137_e1745_d_b51, eq137_e1745_d_b52, eq137_e1745_d_b53, eq137_e1745_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_value: f64 = eq137_e1747;
        let eq137_node_derivatives: [f64; 23] = [eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22];
        let eq137_branch_derivatives: [f64; 55] = [eq137_e1747_d_b0, eq137_e1747_d_b1, eq137_e1747_d_b2, eq137_e1747_d_b3, eq137_e1747_d_b4, eq137_e1747_d_b5, eq137_e1747_d_b6, eq137_e1747_d_b7, eq137_e1747_d_b8, eq137_e1747_d_b9, eq137_e1747_d_b10, eq137_e1747_d_b11, eq137_e1747_d_b12, eq137_e1747_d_b13, eq137_e1747_d_b14, eq137_e1747_d_b15, eq137_e1747_d_b16, eq137_e1747_d_b17, eq137_e1747_d_b18, eq137_e1747_d_b19, eq137_e1747_d_b20, eq137_e1747_d_b21, eq137_e1747_d_b22, eq137_e1747_d_b23, eq137_e1747_d_b24, eq137_e1747_d_b25, eq137_e1747_d_b26, eq137_e1747_d_b27, eq137_e1747_d_b28, eq137_e1747_d_b29, eq137_e1747_d_b30, eq137_e1747_d_b31, eq137_e1747_d_b32, eq137_e1747_d_b33, eq137_e1747_d_b34, eq137_e1747_d_b35, eq137_e1747_d_b36, eq137_e1747_d_b37, eq137_e1747_d_b38, eq137_e1747_d_b39, eq137_e1747_d_b40, eq137_e1747_d_b41, eq137_e1747_d_b42, eq137_e1747_d_b43, eq137_e1747_d_b44, eq137_e1747_d_b45, eq137_e1747_d_b46, eq137_e1747_d_b47, eq137_e1747_d_b48, eq137_e1747_d_b49, eq137_e1747_d_b50, eq137_e1747_d_b51, eq137_e1747_d_b52, eq137_e1747_d_b53, eq137_e1747_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(19),
            multiplicity * (eq137_value),
            &eq137_node_derivatives,
            &eq137_branch_derivatives,
            multiplicity,
        );
        let (eq138_e1757, eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22, eq138_e1757_d_b0, eq138_e1757_d_b1, eq138_e1757_d_b2, eq138_e1757_d_b3, eq138_e1757_d_b4, eq138_e1757_d_b5, eq138_e1757_d_b6, eq138_e1757_d_b7, eq138_e1757_d_b8, eq138_e1757_d_b9, eq138_e1757_d_b10, eq138_e1757_d_b11, eq138_e1757_d_b12, eq138_e1757_d_b13, eq138_e1757_d_b14, eq138_e1757_d_b15, eq138_e1757_d_b16, eq138_e1757_d_b17, eq138_e1757_d_b18, eq138_e1757_d_b19, eq138_e1757_d_b20, eq138_e1757_d_b21, eq138_e1757_d_b22, eq138_e1757_d_b23, eq138_e1757_d_b24, eq138_e1757_d_b25, eq138_e1757_d_b26, eq138_e1757_d_b27, eq138_e1757_d_b28, eq138_e1757_d_b29, eq138_e1757_d_b30, eq138_e1757_d_b31, eq138_e1757_d_b32, eq138_e1757_d_b33, eq138_e1757_d_b34, eq138_e1757_d_b35, eq138_e1757_d_b36, eq138_e1757_d_b37, eq138_e1757_d_b38, eq138_e1757_d_b39, eq138_e1757_d_b40, eq138_e1757_d_b41, eq138_e1757_d_b42, eq138_e1757_d_b43, eq138_e1757_d_b44, eq138_e1757_d_b45, eq138_e1757_d_b46, eq138_e1757_d_b47, eq138_e1757_d_b48, eq138_e1757_d_b49, eq138_e1757_d_b50, eq138_e1757_d_b51, eq138_e1757_d_b52, eq138_e1757_d_b53, eq138_e1757_d_b54,) = {
    if ((!s.b[575]) && s.b[578]) {
        let eq138_e1754: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 37, s.v[241]);
        let eq138_e1755: f64 = (p.p7 * eq138_e1754);
        let eq138_e1755_d_n0: f64 = (p.p7 * (s.dn[241][0] * ddt_scale));
        let eq138_e1755_d_n1: f64 = (p.p7 * (s.dn[241][1] * ddt_scale));
        let eq138_e1755_d_n2: f64 = (p.p7 * (s.dn[241][2] * ddt_scale));
        let eq138_e1755_d_n3: f64 = (p.p7 * (s.dn[241][3] * ddt_scale));
        let eq138_e1755_d_n4: f64 = (p.p7 * (s.dn[241][4] * ddt_scale));
        let eq138_e1755_d_n5: f64 = (p.p7 * (s.dn[241][5] * ddt_scale));
        let eq138_e1755_d_n6: f64 = (p.p7 * (s.dn[241][6] * ddt_scale));
        let eq138_e1755_d_n7: f64 = (p.p7 * (s.dn[241][7] * ddt_scale));
        let eq138_e1755_d_n8: f64 = (p.p7 * (s.dn[241][8] * ddt_scale));
        let eq138_e1755_d_n9: f64 = (p.p7 * (s.dn[241][9] * ddt_scale));
        let eq138_e1755_d_n10: f64 = (p.p7 * (s.dn[241][10] * ddt_scale));
        let eq138_e1755_d_n11: f64 = (p.p7 * (s.dn[241][11] * ddt_scale));
        let eq138_e1755_d_n12: f64 = (p.p7 * (s.dn[241][12] * ddt_scale));
        let eq138_e1755_d_n13: f64 = (p.p7 * (s.dn[241][13] * ddt_scale));
        let eq138_e1755_d_n14: f64 = (p.p7 * (s.dn[241][14] * ddt_scale));
        let eq138_e1755_d_n15: f64 = (p.p7 * (s.dn[241][15] * ddt_scale));
        let eq138_e1755_d_n16: f64 = (p.p7 * (s.dn[241][16] * ddt_scale));
        let eq138_e1755_d_n17: f64 = (p.p7 * (s.dn[241][17] * ddt_scale));
        let eq138_e1755_d_n18: f64 = (p.p7 * (s.dn[241][18] * ddt_scale));
        let eq138_e1755_d_n19: f64 = (p.p7 * (s.dn[241][19] * ddt_scale));
        let eq138_e1755_d_n20: f64 = (p.p7 * (s.dn[241][20] * ddt_scale));
        let eq138_e1755_d_n21: f64 = (p.p7 * (s.dn[241][21] * ddt_scale));
        let eq138_e1755_d_n22: f64 = (p.p7 * (s.dn[241][22] * ddt_scale));
        let eq138_e1755_d_b0: f64 = (p.p7 * (s.db[241][0] * ddt_scale));
        let eq138_e1755_d_b1: f64 = (p.p7 * (s.db[241][1] * ddt_scale));
        let eq138_e1755_d_b2: f64 = (p.p7 * (s.db[241][2] * ddt_scale));
        let eq138_e1755_d_b3: f64 = (p.p7 * (s.db[241][3] * ddt_scale));
        let eq138_e1755_d_b4: f64 = (p.p7 * (s.db[241][4] * ddt_scale));
        let eq138_e1755_d_b5: f64 = (p.p7 * (s.db[241][5] * ddt_scale));
        let eq138_e1755_d_b6: f64 = (p.p7 * (s.db[241][6] * ddt_scale));
        let eq138_e1755_d_b7: f64 = (p.p7 * (s.db[241][7] * ddt_scale));
        let eq138_e1755_d_b8: f64 = (p.p7 * (s.db[241][8] * ddt_scale));
        let eq138_e1755_d_b9: f64 = (p.p7 * (s.db[241][9] * ddt_scale));
        let eq138_e1755_d_b10: f64 = (p.p7 * (s.db[241][10] * ddt_scale));
        let eq138_e1755_d_b11: f64 = (p.p7 * (s.db[241][11] * ddt_scale));
        let eq138_e1755_d_b12: f64 = (p.p7 * (s.db[241][12] * ddt_scale));
        let eq138_e1755_d_b13: f64 = (p.p7 * (s.db[241][13] * ddt_scale));
        let eq138_e1755_d_b14: f64 = (p.p7 * (s.db[241][14] * ddt_scale));
        let eq138_e1755_d_b15: f64 = (p.p7 * (s.db[241][15] * ddt_scale));
        let eq138_e1755_d_b16: f64 = (p.p7 * (s.db[241][16] * ddt_scale));
        let eq138_e1755_d_b17: f64 = (p.p7 * (s.db[241][17] * ddt_scale));
        let eq138_e1755_d_b18: f64 = (p.p7 * (s.db[241][18] * ddt_scale));
        let eq138_e1755_d_b19: f64 = (p.p7 * (s.db[241][19] * ddt_scale));
        let eq138_e1755_d_b20: f64 = (p.p7 * (s.db[241][20] * ddt_scale));
        let eq138_e1755_d_b21: f64 = (p.p7 * (s.db[241][21] * ddt_scale));
        let eq138_e1755_d_b22: f64 = (p.p7 * (s.db[241][22] * ddt_scale));
        let eq138_e1755_d_b23: f64 = (p.p7 * (s.db[241][23] * ddt_scale));
        let eq138_e1755_d_b24: f64 = (p.p7 * (s.db[241][24] * ddt_scale));
        let eq138_e1755_d_b25: f64 = (p.p7 * (s.db[241][25] * ddt_scale));
        let eq138_e1755_d_b26: f64 = (p.p7 * (s.db[241][26] * ddt_scale));
        let eq138_e1755_d_b27: f64 = (p.p7 * (s.db[241][27] * ddt_scale));
        let eq138_e1755_d_b28: f64 = (p.p7 * (s.db[241][28] * ddt_scale));
        let eq138_e1755_d_b29: f64 = (p.p7 * (s.db[241][29] * ddt_scale));
        let eq138_e1755_d_b30: f64 = (p.p7 * (s.db[241][30] * ddt_scale));
        let eq138_e1755_d_b31: f64 = (p.p7 * (s.db[241][31] * ddt_scale));
        let eq138_e1755_d_b32: f64 = (p.p7 * (s.db[241][32] * ddt_scale));
        let eq138_e1755_d_b33: f64 = (p.p7 * (s.db[241][33] * ddt_scale));
        let eq138_e1755_d_b34: f64 = (p.p7 * (s.db[241][34] * ddt_scale));
        let eq138_e1755_d_b35: f64 = (p.p7 * (s.db[241][35] * ddt_scale));
        let eq138_e1755_d_b36: f64 = (p.p7 * (s.db[241][36] * ddt_scale));
        let eq138_e1755_d_b37: f64 = (p.p7 * (s.db[241][37] * ddt_scale));
        let eq138_e1755_d_b38: f64 = (p.p7 * (s.db[241][38] * ddt_scale));
        let eq138_e1755_d_b39: f64 = (p.p7 * (s.db[241][39] * ddt_scale));
        let eq138_e1755_d_b40: f64 = (p.p7 * (s.db[241][40] * ddt_scale));
        let eq138_e1755_d_b41: f64 = (p.p7 * (s.db[241][41] * ddt_scale));
        let eq138_e1755_d_b42: f64 = (p.p7 * (s.db[241][42] * ddt_scale));
        let eq138_e1755_d_b43: f64 = (p.p7 * (s.db[241][43] * ddt_scale));
        let eq138_e1755_d_b44: f64 = (p.p7 * (s.db[241][44] * ddt_scale));
        let eq138_e1755_d_b45: f64 = (p.p7 * (s.db[241][45] * ddt_scale));
        let eq138_e1755_d_b46: f64 = (p.p7 * (s.db[241][46] * ddt_scale));
        let eq138_e1755_d_b47: f64 = (p.p7 * (s.db[241][47] * ddt_scale));
        let eq138_e1755_d_b48: f64 = (p.p7 * (s.db[241][48] * ddt_scale));
        let eq138_e1755_d_b49: f64 = (p.p7 * (s.db[241][49] * ddt_scale));
        let eq138_e1755_d_b50: f64 = (p.p7 * (s.db[241][50] * ddt_scale));
        let eq138_e1755_d_b51: f64 = (p.p7 * (s.db[241][51] * ddt_scale));
        let eq138_e1755_d_b52: f64 = (p.p7 * (s.db[241][52] * ddt_scale));
        let eq138_e1755_d_b53: f64 = (p.p7 * (s.db[241][53] * ddt_scale));
        let eq138_e1755_d_b54: f64 = (p.p7 * (s.db[241][54] * ddt_scale));
        (eq138_e1755, eq138_e1755_d_n0, eq138_e1755_d_n1, eq138_e1755_d_n2, eq138_e1755_d_n3, eq138_e1755_d_n4, eq138_e1755_d_n5, eq138_e1755_d_n6, eq138_e1755_d_n7, eq138_e1755_d_n8, eq138_e1755_d_n9, eq138_e1755_d_n10, eq138_e1755_d_n11, eq138_e1755_d_n12, eq138_e1755_d_n13, eq138_e1755_d_n14, eq138_e1755_d_n15, eq138_e1755_d_n16, eq138_e1755_d_n17, eq138_e1755_d_n18, eq138_e1755_d_n19, eq138_e1755_d_n20, eq138_e1755_d_n21, eq138_e1755_d_n22, eq138_e1755_d_b0, eq138_e1755_d_b1, eq138_e1755_d_b2, eq138_e1755_d_b3, eq138_e1755_d_b4, eq138_e1755_d_b5, eq138_e1755_d_b6, eq138_e1755_d_b7, eq138_e1755_d_b8, eq138_e1755_d_b9, eq138_e1755_d_b10, eq138_e1755_d_b11, eq138_e1755_d_b12, eq138_e1755_d_b13, eq138_e1755_d_b14, eq138_e1755_d_b15, eq138_e1755_d_b16, eq138_e1755_d_b17, eq138_e1755_d_b18, eq138_e1755_d_b19, eq138_e1755_d_b20, eq138_e1755_d_b21, eq138_e1755_d_b22, eq138_e1755_d_b23, eq138_e1755_d_b24, eq138_e1755_d_b25, eq138_e1755_d_b26, eq138_e1755_d_b27, eq138_e1755_d_b28, eq138_e1755_d_b29, eq138_e1755_d_b30, eq138_e1755_d_b31, eq138_e1755_d_b32, eq138_e1755_d_b33, eq138_e1755_d_b34, eq138_e1755_d_b35, eq138_e1755_d_b36, eq138_e1755_d_b37, eq138_e1755_d_b38, eq138_e1755_d_b39, eq138_e1755_d_b40, eq138_e1755_d_b41, eq138_e1755_d_b42, eq138_e1755_d_b43, eq138_e1755_d_b44, eq138_e1755_d_b45, eq138_e1755_d_b46, eq138_e1755_d_b47, eq138_e1755_d_b48, eq138_e1755_d_b49, eq138_e1755_d_b50, eq138_e1755_d_b51, eq138_e1755_d_b52, eq138_e1755_d_b53, eq138_e1755_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq138_value: f64 = eq138_e1757;
        let eq138_node_derivatives: [f64; 23] = [eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22];
        let eq138_branch_derivatives: [f64; 55] = [eq138_e1757_d_b0, eq138_e1757_d_b1, eq138_e1757_d_b2, eq138_e1757_d_b3, eq138_e1757_d_b4, eq138_e1757_d_b5, eq138_e1757_d_b6, eq138_e1757_d_b7, eq138_e1757_d_b8, eq138_e1757_d_b9, eq138_e1757_d_b10, eq138_e1757_d_b11, eq138_e1757_d_b12, eq138_e1757_d_b13, eq138_e1757_d_b14, eq138_e1757_d_b15, eq138_e1757_d_b16, eq138_e1757_d_b17, eq138_e1757_d_b18, eq138_e1757_d_b19, eq138_e1757_d_b20, eq138_e1757_d_b21, eq138_e1757_d_b22, eq138_e1757_d_b23, eq138_e1757_d_b24, eq138_e1757_d_b25, eq138_e1757_d_b26, eq138_e1757_d_b27, eq138_e1757_d_b28, eq138_e1757_d_b29, eq138_e1757_d_b30, eq138_e1757_d_b31, eq138_e1757_d_b32, eq138_e1757_d_b33, eq138_e1757_d_b34, eq138_e1757_d_b35, eq138_e1757_d_b36, eq138_e1757_d_b37, eq138_e1757_d_b38, eq138_e1757_d_b39, eq138_e1757_d_b40, eq138_e1757_d_b41, eq138_e1757_d_b42, eq138_e1757_d_b43, eq138_e1757_d_b44, eq138_e1757_d_b45, eq138_e1757_d_b46, eq138_e1757_d_b47, eq138_e1757_d_b48, eq138_e1757_d_b49, eq138_e1757_d_b50, eq138_e1757_d_b51, eq138_e1757_d_b52, eq138_e1757_d_b53, eq138_e1757_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq138_value),
            &eq138_node_derivatives,
            &eq138_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_22(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[240][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[240][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[240][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[240][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[240][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[240][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[240][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[240][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[240][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[240][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[240][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[240][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[240][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[240][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[240][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[240][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[240][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[240][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[240][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[240][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[240][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[240][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[240][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[240][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[240][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[240][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[240][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[240][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[240][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[240][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[240][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[240][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[240][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[240][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[240][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[240][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[240][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[240][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[240][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[240][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[240][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[240][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[240][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[240][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[240][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[240][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[240][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[240][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[240][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[240][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[240][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[240][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[240][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[240][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[240][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[240][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[240][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[240][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[240][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[240][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[240][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[240][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[240][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[240][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[240][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[240][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[240][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[240][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[240][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[240][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[240][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[240][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[240][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[240][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[240][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[240][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[240][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[240][54] * ddt_scale));
        let (eq139_e1769, eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22, eq139_e1769_d_b0, eq139_e1769_d_b1, eq139_e1769_d_b2, eq139_e1769_d_b3, eq139_e1769_d_b4, eq139_e1769_d_b5, eq139_e1769_d_b6, eq139_e1769_d_b7, eq139_e1769_d_b8, eq139_e1769_d_b9, eq139_e1769_d_b10, eq139_e1769_d_b11, eq139_e1769_d_b12, eq139_e1769_d_b13, eq139_e1769_d_b14, eq139_e1769_d_b15, eq139_e1769_d_b16, eq139_e1769_d_b17, eq139_e1769_d_b18, eq139_e1769_d_b19, eq139_e1769_d_b20, eq139_e1769_d_b21, eq139_e1769_d_b22, eq139_e1769_d_b23, eq139_e1769_d_b24, eq139_e1769_d_b25, eq139_e1769_d_b26, eq139_e1769_d_b27, eq139_e1769_d_b28, eq139_e1769_d_b29, eq139_e1769_d_b30, eq139_e1769_d_b31, eq139_e1769_d_b32, eq139_e1769_d_b33, eq139_e1769_d_b34, eq139_e1769_d_b35, eq139_e1769_d_b36, eq139_e1769_d_b37, eq139_e1769_d_b38, eq139_e1769_d_b39, eq139_e1769_d_b40, eq139_e1769_d_b41, eq139_e1769_d_b42, eq139_e1769_d_b43, eq139_e1769_d_b44, eq139_e1769_d_b45, eq139_e1769_d_b46, eq139_e1769_d_b47, eq139_e1769_d_b48, eq139_e1769_d_b49, eq139_e1769_d_b50, eq139_e1769_d_b51, eq139_e1769_d_b52, eq139_e1769_d_b53, eq139_e1769_d_b54,) = {
    if (((!s.b[575]) && s.b[578]) && s.b[579]) {
        let eq139_e1766: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 38, s.v[240]);
        let eq139_e1767: f64 = (p.p7 * eq139_e1766);
        (eq139_e1767, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq139_value: f64 = eq139_e1769;
        let eq139_node_derivatives: [f64; 23] = [eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22];
        let eq139_branch_derivatives: [f64; 55] = [eq139_e1769_d_b0, eq139_e1769_d_b1, eq139_e1769_d_b2, eq139_e1769_d_b3, eq139_e1769_d_b4, eq139_e1769_d_b5, eq139_e1769_d_b6, eq139_e1769_d_b7, eq139_e1769_d_b8, eq139_e1769_d_b9, eq139_e1769_d_b10, eq139_e1769_d_b11, eq139_e1769_d_b12, eq139_e1769_d_b13, eq139_e1769_d_b14, eq139_e1769_d_b15, eq139_e1769_d_b16, eq139_e1769_d_b17, eq139_e1769_d_b18, eq139_e1769_d_b19, eq139_e1769_d_b20, eq139_e1769_d_b21, eq139_e1769_d_b22, eq139_e1769_d_b23, eq139_e1769_d_b24, eq139_e1769_d_b25, eq139_e1769_d_b26, eq139_e1769_d_b27, eq139_e1769_d_b28, eq139_e1769_d_b29, eq139_e1769_d_b30, eq139_e1769_d_b31, eq139_e1769_d_b32, eq139_e1769_d_b33, eq139_e1769_d_b34, eq139_e1769_d_b35, eq139_e1769_d_b36, eq139_e1769_d_b37, eq139_e1769_d_b38, eq139_e1769_d_b39, eq139_e1769_d_b40, eq139_e1769_d_b41, eq139_e1769_d_b42, eq139_e1769_d_b43, eq139_e1769_d_b44, eq139_e1769_d_b45, eq139_e1769_d_b46, eq139_e1769_d_b47, eq139_e1769_d_b48, eq139_e1769_d_b49, eq139_e1769_d_b50, eq139_e1769_d_b51, eq139_e1769_d_b52, eq139_e1769_d_b53, eq139_e1769_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq139_value),
            &eq139_node_derivatives,
            &eq139_branch_derivatives,
            multiplicity,
        );
        let (eq140_e1783, eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22, eq140_e1783_d_b0, eq140_e1783_d_b1, eq140_e1783_d_b2, eq140_e1783_d_b3, eq140_e1783_d_b4, eq140_e1783_d_b5, eq140_e1783_d_b6, eq140_e1783_d_b7, eq140_e1783_d_b8, eq140_e1783_d_b9, eq140_e1783_d_b10, eq140_e1783_d_b11, eq140_e1783_d_b12, eq140_e1783_d_b13, eq140_e1783_d_b14, eq140_e1783_d_b15, eq140_e1783_d_b16, eq140_e1783_d_b17, eq140_e1783_d_b18, eq140_e1783_d_b19, eq140_e1783_d_b20, eq140_e1783_d_b21, eq140_e1783_d_b22, eq140_e1783_d_b23, eq140_e1783_d_b24, eq140_e1783_d_b25, eq140_e1783_d_b26, eq140_e1783_d_b27, eq140_e1783_d_b28, eq140_e1783_d_b29, eq140_e1783_d_b30, eq140_e1783_d_b31, eq140_e1783_d_b32, eq140_e1783_d_b33, eq140_e1783_d_b34, eq140_e1783_d_b35, eq140_e1783_d_b36, eq140_e1783_d_b37, eq140_e1783_d_b38, eq140_e1783_d_b39, eq140_e1783_d_b40, eq140_e1783_d_b41, eq140_e1783_d_b42, eq140_e1783_d_b43, eq140_e1783_d_b44, eq140_e1783_d_b45, eq140_e1783_d_b46, eq140_e1783_d_b47, eq140_e1783_d_b48, eq140_e1783_d_b49, eq140_e1783_d_b50, eq140_e1783_d_b51, eq140_e1783_d_b52, eq140_e1783_d_b53, eq140_e1783_d_b54,) = {
    if (((!s.b[575]) && s.b[578]) && s.b[579]) {
        let eq140_e1778: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 39, s.v[240]);
        let eq140_e1779: f64 = (p.p7 * eq140_e1778);
        let eq140_e1781: f64 = (eq140_e1779 * p.p246);
        let eq140_e1781_d_n0: f64 = (__rspice_deriv_cse_0 * p.p246);
        let eq140_e1781_d_n1: f64 = (__rspice_deriv_cse_1 * p.p246);
        let eq140_e1781_d_n2: f64 = (__rspice_deriv_cse_2 * p.p246);
        let eq140_e1781_d_n3: f64 = (__rspice_deriv_cse_3 * p.p246);
        let eq140_e1781_d_n4: f64 = (__rspice_deriv_cse_4 * p.p246);
        let eq140_e1781_d_n5: f64 = (__rspice_deriv_cse_5 * p.p246);
        let eq140_e1781_d_n6: f64 = (__rspice_deriv_cse_6 * p.p246);
        let eq140_e1781_d_n7: f64 = (__rspice_deriv_cse_7 * p.p246);
        let eq140_e1781_d_n8: f64 = (__rspice_deriv_cse_8 * p.p246);
        let eq140_e1781_d_n9: f64 = (__rspice_deriv_cse_9 * p.p246);
        let eq140_e1781_d_n10: f64 = (__rspice_deriv_cse_10 * p.p246);
        let eq140_e1781_d_n11: f64 = (__rspice_deriv_cse_11 * p.p246);
        let eq140_e1781_d_n12: f64 = (__rspice_deriv_cse_12 * p.p246);
        let eq140_e1781_d_n13: f64 = (__rspice_deriv_cse_13 * p.p246);
        let eq140_e1781_d_n14: f64 = (__rspice_deriv_cse_14 * p.p246);
        let eq140_e1781_d_n15: f64 = (__rspice_deriv_cse_15 * p.p246);
        let eq140_e1781_d_n16: f64 = (__rspice_deriv_cse_16 * p.p246);
        let eq140_e1781_d_n17: f64 = (__rspice_deriv_cse_17 * p.p246);
        let eq140_e1781_d_n18: f64 = (__rspice_deriv_cse_18 * p.p246);
        let eq140_e1781_d_n19: f64 = (__rspice_deriv_cse_19 * p.p246);
        let eq140_e1781_d_n20: f64 = (__rspice_deriv_cse_20 * p.p246);
        let eq140_e1781_d_n21: f64 = (__rspice_deriv_cse_21 * p.p246);
        let eq140_e1781_d_n22: f64 = (__rspice_deriv_cse_22 * p.p246);
        let eq140_e1781_d_b0: f64 = (__rspice_deriv_cse_23 * p.p246);
        let eq140_e1781_d_b1: f64 = (__rspice_deriv_cse_24 * p.p246);
        let eq140_e1781_d_b2: f64 = (__rspice_deriv_cse_25 * p.p246);
        let eq140_e1781_d_b3: f64 = (__rspice_deriv_cse_26 * p.p246);
        let eq140_e1781_d_b4: f64 = (__rspice_deriv_cse_27 * p.p246);
        let eq140_e1781_d_b5: f64 = (__rspice_deriv_cse_28 * p.p246);
        let eq140_e1781_d_b6: f64 = (__rspice_deriv_cse_29 * p.p246);
        let eq140_e1781_d_b7: f64 = (__rspice_deriv_cse_30 * p.p246);
        let eq140_e1781_d_b8: f64 = (__rspice_deriv_cse_31 * p.p246);
        let eq140_e1781_d_b9: f64 = (__rspice_deriv_cse_32 * p.p246);
        let eq140_e1781_d_b10: f64 = (__rspice_deriv_cse_33 * p.p246);
        let eq140_e1781_d_b11: f64 = (__rspice_deriv_cse_34 * p.p246);
        let eq140_e1781_d_b12: f64 = (__rspice_deriv_cse_35 * p.p246);
        let eq140_e1781_d_b13: f64 = (__rspice_deriv_cse_36 * p.p246);
        let eq140_e1781_d_b14: f64 = (__rspice_deriv_cse_37 * p.p246);
        let eq140_e1781_d_b15: f64 = (__rspice_deriv_cse_38 * p.p246);
        let eq140_e1781_d_b16: f64 = (__rspice_deriv_cse_39 * p.p246);
        let eq140_e1781_d_b17: f64 = (__rspice_deriv_cse_40 * p.p246);
        let eq140_e1781_d_b18: f64 = (__rspice_deriv_cse_41 * p.p246);
        let eq140_e1781_d_b19: f64 = (__rspice_deriv_cse_42 * p.p246);
        let eq140_e1781_d_b20: f64 = (__rspice_deriv_cse_43 * p.p246);
        let eq140_e1781_d_b21: f64 = (__rspice_deriv_cse_44 * p.p246);
        let eq140_e1781_d_b22: f64 = (__rspice_deriv_cse_45 * p.p246);
        let eq140_e1781_d_b23: f64 = (__rspice_deriv_cse_46 * p.p246);
        let eq140_e1781_d_b24: f64 = (__rspice_deriv_cse_47 * p.p246);
        let eq140_e1781_d_b25: f64 = (__rspice_deriv_cse_48 * p.p246);
        let eq140_e1781_d_b26: f64 = (__rspice_deriv_cse_49 * p.p246);
        let eq140_e1781_d_b27: f64 = (__rspice_deriv_cse_50 * p.p246);
        let eq140_e1781_d_b28: f64 = (__rspice_deriv_cse_51 * p.p246);
        let eq140_e1781_d_b29: f64 = (__rspice_deriv_cse_52 * p.p246);
        let eq140_e1781_d_b30: f64 = (__rspice_deriv_cse_53 * p.p246);
        let eq140_e1781_d_b31: f64 = (__rspice_deriv_cse_54 * p.p246);
        let eq140_e1781_d_b32: f64 = (__rspice_deriv_cse_55 * p.p246);
        let eq140_e1781_d_b33: f64 = (__rspice_deriv_cse_56 * p.p246);
        let eq140_e1781_d_b34: f64 = (__rspice_deriv_cse_57 * p.p246);
        let eq140_e1781_d_b35: f64 = (__rspice_deriv_cse_58 * p.p246);
        let eq140_e1781_d_b36: f64 = (__rspice_deriv_cse_59 * p.p246);
        let eq140_e1781_d_b37: f64 = (__rspice_deriv_cse_60 * p.p246);
        let eq140_e1781_d_b38: f64 = (__rspice_deriv_cse_61 * p.p246);
        let eq140_e1781_d_b39: f64 = (__rspice_deriv_cse_62 * p.p246);
        let eq140_e1781_d_b40: f64 = (__rspice_deriv_cse_63 * p.p246);
        let eq140_e1781_d_b41: f64 = (__rspice_deriv_cse_64 * p.p246);
        let eq140_e1781_d_b42: f64 = (__rspice_deriv_cse_65 * p.p246);
        let eq140_e1781_d_b43: f64 = (__rspice_deriv_cse_66 * p.p246);
        let eq140_e1781_d_b44: f64 = (__rspice_deriv_cse_67 * p.p246);
        let eq140_e1781_d_b45: f64 = (__rspice_deriv_cse_68 * p.p246);
        let eq140_e1781_d_b46: f64 = (__rspice_deriv_cse_69 * p.p246);
        let eq140_e1781_d_b47: f64 = (__rspice_deriv_cse_70 * p.p246);
        let eq140_e1781_d_b48: f64 = (__rspice_deriv_cse_71 * p.p246);
        let eq140_e1781_d_b49: f64 = (__rspice_deriv_cse_72 * p.p246);
        let eq140_e1781_d_b50: f64 = (__rspice_deriv_cse_73 * p.p246);
        let eq140_e1781_d_b51: f64 = (__rspice_deriv_cse_74 * p.p246);
        let eq140_e1781_d_b52: f64 = (__rspice_deriv_cse_75 * p.p246);
        let eq140_e1781_d_b53: f64 = (__rspice_deriv_cse_76 * p.p246);
        let eq140_e1781_d_b54: f64 = (__rspice_deriv_cse_77 * p.p246);
        (eq140_e1781, eq140_e1781_d_n0, eq140_e1781_d_n1, eq140_e1781_d_n2, eq140_e1781_d_n3, eq140_e1781_d_n4, eq140_e1781_d_n5, eq140_e1781_d_n6, eq140_e1781_d_n7, eq140_e1781_d_n8, eq140_e1781_d_n9, eq140_e1781_d_n10, eq140_e1781_d_n11, eq140_e1781_d_n12, eq140_e1781_d_n13, eq140_e1781_d_n14, eq140_e1781_d_n15, eq140_e1781_d_n16, eq140_e1781_d_n17, eq140_e1781_d_n18, eq140_e1781_d_n19, eq140_e1781_d_n20, eq140_e1781_d_n21, eq140_e1781_d_n22, eq140_e1781_d_b0, eq140_e1781_d_b1, eq140_e1781_d_b2, eq140_e1781_d_b3, eq140_e1781_d_b4, eq140_e1781_d_b5, eq140_e1781_d_b6, eq140_e1781_d_b7, eq140_e1781_d_b8, eq140_e1781_d_b9, eq140_e1781_d_b10, eq140_e1781_d_b11, eq140_e1781_d_b12, eq140_e1781_d_b13, eq140_e1781_d_b14, eq140_e1781_d_b15, eq140_e1781_d_b16, eq140_e1781_d_b17, eq140_e1781_d_b18, eq140_e1781_d_b19, eq140_e1781_d_b20, eq140_e1781_d_b21, eq140_e1781_d_b22, eq140_e1781_d_b23, eq140_e1781_d_b24, eq140_e1781_d_b25, eq140_e1781_d_b26, eq140_e1781_d_b27, eq140_e1781_d_b28, eq140_e1781_d_b29, eq140_e1781_d_b30, eq140_e1781_d_b31, eq140_e1781_d_b32, eq140_e1781_d_b33, eq140_e1781_d_b34, eq140_e1781_d_b35, eq140_e1781_d_b36, eq140_e1781_d_b37, eq140_e1781_d_b38, eq140_e1781_d_b39, eq140_e1781_d_b40, eq140_e1781_d_b41, eq140_e1781_d_b42, eq140_e1781_d_b43, eq140_e1781_d_b44, eq140_e1781_d_b45, eq140_e1781_d_b46, eq140_e1781_d_b47, eq140_e1781_d_b48, eq140_e1781_d_b49, eq140_e1781_d_b50, eq140_e1781_d_b51, eq140_e1781_d_b52, eq140_e1781_d_b53, eq140_e1781_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq140_value: f64 = eq140_e1783;
        let eq140_node_derivatives: [f64; 23] = [eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22];
        let eq140_branch_derivatives: [f64; 55] = [eq140_e1783_d_b0, eq140_e1783_d_b1, eq140_e1783_d_b2, eq140_e1783_d_b3, eq140_e1783_d_b4, eq140_e1783_d_b5, eq140_e1783_d_b6, eq140_e1783_d_b7, eq140_e1783_d_b8, eq140_e1783_d_b9, eq140_e1783_d_b10, eq140_e1783_d_b11, eq140_e1783_d_b12, eq140_e1783_d_b13, eq140_e1783_d_b14, eq140_e1783_d_b15, eq140_e1783_d_b16, eq140_e1783_d_b17, eq140_e1783_d_b18, eq140_e1783_d_b19, eq140_e1783_d_b20, eq140_e1783_d_b21, eq140_e1783_d_b22, eq140_e1783_d_b23, eq140_e1783_d_b24, eq140_e1783_d_b25, eq140_e1783_d_b26, eq140_e1783_d_b27, eq140_e1783_d_b28, eq140_e1783_d_b29, eq140_e1783_d_b30, eq140_e1783_d_b31, eq140_e1783_d_b32, eq140_e1783_d_b33, eq140_e1783_d_b34, eq140_e1783_d_b35, eq140_e1783_d_b36, eq140_e1783_d_b37, eq140_e1783_d_b38, eq140_e1783_d_b39, eq140_e1783_d_b40, eq140_e1783_d_b41, eq140_e1783_d_b42, eq140_e1783_d_b43, eq140_e1783_d_b44, eq140_e1783_d_b45, eq140_e1783_d_b46, eq140_e1783_d_b47, eq140_e1783_d_b48, eq140_e1783_d_b49, eq140_e1783_d_b50, eq140_e1783_d_b51, eq140_e1783_d_b52, eq140_e1783_d_b53, eq140_e1783_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq140_value),
            &eq140_node_derivatives,
            &eq140_branch_derivatives,
            multiplicity,
        );
        let (eq141_e1796, eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22, eq141_e1796_d_b0, eq141_e1796_d_b1, eq141_e1796_d_b2, eq141_e1796_d_b3, eq141_e1796_d_b4, eq141_e1796_d_b5, eq141_e1796_d_b6, eq141_e1796_d_b7, eq141_e1796_d_b8, eq141_e1796_d_b9, eq141_e1796_d_b10, eq141_e1796_d_b11, eq141_e1796_d_b12, eq141_e1796_d_b13, eq141_e1796_d_b14, eq141_e1796_d_b15, eq141_e1796_d_b16, eq141_e1796_d_b17, eq141_e1796_d_b18, eq141_e1796_d_b19, eq141_e1796_d_b20, eq141_e1796_d_b21, eq141_e1796_d_b22, eq141_e1796_d_b23, eq141_e1796_d_b24, eq141_e1796_d_b25, eq141_e1796_d_b26, eq141_e1796_d_b27, eq141_e1796_d_b28, eq141_e1796_d_b29, eq141_e1796_d_b30, eq141_e1796_d_b31, eq141_e1796_d_b32, eq141_e1796_d_b33, eq141_e1796_d_b34, eq141_e1796_d_b35, eq141_e1796_d_b36, eq141_e1796_d_b37, eq141_e1796_d_b38, eq141_e1796_d_b39, eq141_e1796_d_b40, eq141_e1796_d_b41, eq141_e1796_d_b42, eq141_e1796_d_b43, eq141_e1796_d_b44, eq141_e1796_d_b45, eq141_e1796_d_b46, eq141_e1796_d_b47, eq141_e1796_d_b48, eq141_e1796_d_b49, eq141_e1796_d_b50, eq141_e1796_d_b51, eq141_e1796_d_b52, eq141_e1796_d_b53, eq141_e1796_d_b54,) = {
    if (((!s.b[575]) && s.b[578]) && (!s.b[579])) {
        let eq141_e1793: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 40, s.v[240]);
        let eq141_e1794: f64 = (p.p7 * eq141_e1793);
        (eq141_e1794, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_value: f64 = eq141_e1796;
        let eq141_node_derivatives: [f64; 23] = [eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22];
        let eq141_branch_derivatives: [f64; 55] = [eq141_e1796_d_b0, eq141_e1796_d_b1, eq141_e1796_d_b2, eq141_e1796_d_b3, eq141_e1796_d_b4, eq141_e1796_d_b5, eq141_e1796_d_b6, eq141_e1796_d_b7, eq141_e1796_d_b8, eq141_e1796_d_b9, eq141_e1796_d_b10, eq141_e1796_d_b11, eq141_e1796_d_b12, eq141_e1796_d_b13, eq141_e1796_d_b14, eq141_e1796_d_b15, eq141_e1796_d_b16, eq141_e1796_d_b17, eq141_e1796_d_b18, eq141_e1796_d_b19, eq141_e1796_d_b20, eq141_e1796_d_b21, eq141_e1796_d_b22, eq141_e1796_d_b23, eq141_e1796_d_b24, eq141_e1796_d_b25, eq141_e1796_d_b26, eq141_e1796_d_b27, eq141_e1796_d_b28, eq141_e1796_d_b29, eq141_e1796_d_b30, eq141_e1796_d_b31, eq141_e1796_d_b32, eq141_e1796_d_b33, eq141_e1796_d_b34, eq141_e1796_d_b35, eq141_e1796_d_b36, eq141_e1796_d_b37, eq141_e1796_d_b38, eq141_e1796_d_b39, eq141_e1796_d_b40, eq141_e1796_d_b41, eq141_e1796_d_b42, eq141_e1796_d_b43, eq141_e1796_d_b44, eq141_e1796_d_b45, eq141_e1796_d_b46, eq141_e1796_d_b47, eq141_e1796_d_b48, eq141_e1796_d_b49, eq141_e1796_d_b50, eq141_e1796_d_b51, eq141_e1796_d_b52, eq141_e1796_d_b53, eq141_e1796_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq141_value),
            &eq141_node_derivatives,
            &eq141_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_23(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq142_e1811, eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22, eq142_e1811_d_b0, eq142_e1811_d_b1, eq142_e1811_d_b2, eq142_e1811_d_b3, eq142_e1811_d_b4, eq142_e1811_d_b5, eq142_e1811_d_b6, eq142_e1811_d_b7, eq142_e1811_d_b8, eq142_e1811_d_b9, eq142_e1811_d_b10, eq142_e1811_d_b11, eq142_e1811_d_b12, eq142_e1811_d_b13, eq142_e1811_d_b14, eq142_e1811_d_b15, eq142_e1811_d_b16, eq142_e1811_d_b17, eq142_e1811_d_b18, eq142_e1811_d_b19, eq142_e1811_d_b20, eq142_e1811_d_b21, eq142_e1811_d_b22, eq142_e1811_d_b23, eq142_e1811_d_b24, eq142_e1811_d_b25, eq142_e1811_d_b26, eq142_e1811_d_b27, eq142_e1811_d_b28, eq142_e1811_d_b29, eq142_e1811_d_b30, eq142_e1811_d_b31, eq142_e1811_d_b32, eq142_e1811_d_b33, eq142_e1811_d_b34, eq142_e1811_d_b35, eq142_e1811_d_b36, eq142_e1811_d_b37, eq142_e1811_d_b38, eq142_e1811_d_b39, eq142_e1811_d_b40, eq142_e1811_d_b41, eq142_e1811_d_b42, eq142_e1811_d_b43, eq142_e1811_d_b44, eq142_e1811_d_b45, eq142_e1811_d_b46, eq142_e1811_d_b47, eq142_e1811_d_b48, eq142_e1811_d_b49, eq142_e1811_d_b50, eq142_e1811_d_b51, eq142_e1811_d_b52, eq142_e1811_d_b53, eq142_e1811_d_b54,) = {
    if (((!s.b[575]) && s.b[578]) && (!s.b[579])) {
        let eq142_e1806: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 41, s.v[240]);
        let eq142_e1807: f64 = (p.p7 * eq142_e1806);
        let eq142_e1807_d_n0: f64 = (p.p7 * (s.dn[240][0] * ddt_scale));
        let eq142_e1807_d_n1: f64 = (p.p7 * (s.dn[240][1] * ddt_scale));
        let eq142_e1807_d_n2: f64 = (p.p7 * (s.dn[240][2] * ddt_scale));
        let eq142_e1807_d_n3: f64 = (p.p7 * (s.dn[240][3] * ddt_scale));
        let eq142_e1807_d_n4: f64 = (p.p7 * (s.dn[240][4] * ddt_scale));
        let eq142_e1807_d_n5: f64 = (p.p7 * (s.dn[240][5] * ddt_scale));
        let eq142_e1807_d_n6: f64 = (p.p7 * (s.dn[240][6] * ddt_scale));
        let eq142_e1807_d_n7: f64 = (p.p7 * (s.dn[240][7] * ddt_scale));
        let eq142_e1807_d_n8: f64 = (p.p7 * (s.dn[240][8] * ddt_scale));
        let eq142_e1807_d_n9: f64 = (p.p7 * (s.dn[240][9] * ddt_scale));
        let eq142_e1807_d_n10: f64 = (p.p7 * (s.dn[240][10] * ddt_scale));
        let eq142_e1807_d_n11: f64 = (p.p7 * (s.dn[240][11] * ddt_scale));
        let eq142_e1807_d_n12: f64 = (p.p7 * (s.dn[240][12] * ddt_scale));
        let eq142_e1807_d_n13: f64 = (p.p7 * (s.dn[240][13] * ddt_scale));
        let eq142_e1807_d_n14: f64 = (p.p7 * (s.dn[240][14] * ddt_scale));
        let eq142_e1807_d_n15: f64 = (p.p7 * (s.dn[240][15] * ddt_scale));
        let eq142_e1807_d_n16: f64 = (p.p7 * (s.dn[240][16] * ddt_scale));
        let eq142_e1807_d_n17: f64 = (p.p7 * (s.dn[240][17] * ddt_scale));
        let eq142_e1807_d_n18: f64 = (p.p7 * (s.dn[240][18] * ddt_scale));
        let eq142_e1807_d_n19: f64 = (p.p7 * (s.dn[240][19] * ddt_scale));
        let eq142_e1807_d_n20: f64 = (p.p7 * (s.dn[240][20] * ddt_scale));
        let eq142_e1807_d_n21: f64 = (p.p7 * (s.dn[240][21] * ddt_scale));
        let eq142_e1807_d_n22: f64 = (p.p7 * (s.dn[240][22] * ddt_scale));
        let eq142_e1807_d_b0: f64 = (p.p7 * (s.db[240][0] * ddt_scale));
        let eq142_e1807_d_b1: f64 = (p.p7 * (s.db[240][1] * ddt_scale));
        let eq142_e1807_d_b2: f64 = (p.p7 * (s.db[240][2] * ddt_scale));
        let eq142_e1807_d_b3: f64 = (p.p7 * (s.db[240][3] * ddt_scale));
        let eq142_e1807_d_b4: f64 = (p.p7 * (s.db[240][4] * ddt_scale));
        let eq142_e1807_d_b5: f64 = (p.p7 * (s.db[240][5] * ddt_scale));
        let eq142_e1807_d_b6: f64 = (p.p7 * (s.db[240][6] * ddt_scale));
        let eq142_e1807_d_b7: f64 = (p.p7 * (s.db[240][7] * ddt_scale));
        let eq142_e1807_d_b8: f64 = (p.p7 * (s.db[240][8] * ddt_scale));
        let eq142_e1807_d_b9: f64 = (p.p7 * (s.db[240][9] * ddt_scale));
        let eq142_e1807_d_b10: f64 = (p.p7 * (s.db[240][10] * ddt_scale));
        let eq142_e1807_d_b11: f64 = (p.p7 * (s.db[240][11] * ddt_scale));
        let eq142_e1807_d_b12: f64 = (p.p7 * (s.db[240][12] * ddt_scale));
        let eq142_e1807_d_b13: f64 = (p.p7 * (s.db[240][13] * ddt_scale));
        let eq142_e1807_d_b14: f64 = (p.p7 * (s.db[240][14] * ddt_scale));
        let eq142_e1807_d_b15: f64 = (p.p7 * (s.db[240][15] * ddt_scale));
        let eq142_e1807_d_b16: f64 = (p.p7 * (s.db[240][16] * ddt_scale));
        let eq142_e1807_d_b17: f64 = (p.p7 * (s.db[240][17] * ddt_scale));
        let eq142_e1807_d_b18: f64 = (p.p7 * (s.db[240][18] * ddt_scale));
        let eq142_e1807_d_b19: f64 = (p.p7 * (s.db[240][19] * ddt_scale));
        let eq142_e1807_d_b20: f64 = (p.p7 * (s.db[240][20] * ddt_scale));
        let eq142_e1807_d_b21: f64 = (p.p7 * (s.db[240][21] * ddt_scale));
        let eq142_e1807_d_b22: f64 = (p.p7 * (s.db[240][22] * ddt_scale));
        let eq142_e1807_d_b23: f64 = (p.p7 * (s.db[240][23] * ddt_scale));
        let eq142_e1807_d_b24: f64 = (p.p7 * (s.db[240][24] * ddt_scale));
        let eq142_e1807_d_b25: f64 = (p.p7 * (s.db[240][25] * ddt_scale));
        let eq142_e1807_d_b26: f64 = (p.p7 * (s.db[240][26] * ddt_scale));
        let eq142_e1807_d_b27: f64 = (p.p7 * (s.db[240][27] * ddt_scale));
        let eq142_e1807_d_b28: f64 = (p.p7 * (s.db[240][28] * ddt_scale));
        let eq142_e1807_d_b29: f64 = (p.p7 * (s.db[240][29] * ddt_scale));
        let eq142_e1807_d_b30: f64 = (p.p7 * (s.db[240][30] * ddt_scale));
        let eq142_e1807_d_b31: f64 = (p.p7 * (s.db[240][31] * ddt_scale));
        let eq142_e1807_d_b32: f64 = (p.p7 * (s.db[240][32] * ddt_scale));
        let eq142_e1807_d_b33: f64 = (p.p7 * (s.db[240][33] * ddt_scale));
        let eq142_e1807_d_b34: f64 = (p.p7 * (s.db[240][34] * ddt_scale));
        let eq142_e1807_d_b35: f64 = (p.p7 * (s.db[240][35] * ddt_scale));
        let eq142_e1807_d_b36: f64 = (p.p7 * (s.db[240][36] * ddt_scale));
        let eq142_e1807_d_b37: f64 = (p.p7 * (s.db[240][37] * ddt_scale));
        let eq142_e1807_d_b38: f64 = (p.p7 * (s.db[240][38] * ddt_scale));
        let eq142_e1807_d_b39: f64 = (p.p7 * (s.db[240][39] * ddt_scale));
        let eq142_e1807_d_b40: f64 = (p.p7 * (s.db[240][40] * ddt_scale));
        let eq142_e1807_d_b41: f64 = (p.p7 * (s.db[240][41] * ddt_scale));
        let eq142_e1807_d_b42: f64 = (p.p7 * (s.db[240][42] * ddt_scale));
        let eq142_e1807_d_b43: f64 = (p.p7 * (s.db[240][43] * ddt_scale));
        let eq142_e1807_d_b44: f64 = (p.p7 * (s.db[240][44] * ddt_scale));
        let eq142_e1807_d_b45: f64 = (p.p7 * (s.db[240][45] * ddt_scale));
        let eq142_e1807_d_b46: f64 = (p.p7 * (s.db[240][46] * ddt_scale));
        let eq142_e1807_d_b47: f64 = (p.p7 * (s.db[240][47] * ddt_scale));
        let eq142_e1807_d_b48: f64 = (p.p7 * (s.db[240][48] * ddt_scale));
        let eq142_e1807_d_b49: f64 = (p.p7 * (s.db[240][49] * ddt_scale));
        let eq142_e1807_d_b50: f64 = (p.p7 * (s.db[240][50] * ddt_scale));
        let eq142_e1807_d_b51: f64 = (p.p7 * (s.db[240][51] * ddt_scale));
        let eq142_e1807_d_b52: f64 = (p.p7 * (s.db[240][52] * ddt_scale));
        let eq142_e1807_d_b53: f64 = (p.p7 * (s.db[240][53] * ddt_scale));
        let eq142_e1807_d_b54: f64 = (p.p7 * (s.db[240][54] * ddt_scale));
        let eq142_e1809: f64 = (eq142_e1807 * p.p246);
        let eq142_e1809_d_n0: f64 = (eq142_e1807_d_n0 * p.p246);
        let eq142_e1809_d_n1: f64 = (eq142_e1807_d_n1 * p.p246);
        let eq142_e1809_d_n2: f64 = (eq142_e1807_d_n2 * p.p246);
        let eq142_e1809_d_n3: f64 = (eq142_e1807_d_n3 * p.p246);
        let eq142_e1809_d_n4: f64 = (eq142_e1807_d_n4 * p.p246);
        let eq142_e1809_d_n5: f64 = (eq142_e1807_d_n5 * p.p246);
        let eq142_e1809_d_n6: f64 = (eq142_e1807_d_n6 * p.p246);
        let eq142_e1809_d_n7: f64 = (eq142_e1807_d_n7 * p.p246);
        let eq142_e1809_d_n8: f64 = (eq142_e1807_d_n8 * p.p246);
        let eq142_e1809_d_n9: f64 = (eq142_e1807_d_n9 * p.p246);
        let eq142_e1809_d_n10: f64 = (eq142_e1807_d_n10 * p.p246);
        let eq142_e1809_d_n11: f64 = (eq142_e1807_d_n11 * p.p246);
        let eq142_e1809_d_n12: f64 = (eq142_e1807_d_n12 * p.p246);
        let eq142_e1809_d_n13: f64 = (eq142_e1807_d_n13 * p.p246);
        let eq142_e1809_d_n14: f64 = (eq142_e1807_d_n14 * p.p246);
        let eq142_e1809_d_n15: f64 = (eq142_e1807_d_n15 * p.p246);
        let eq142_e1809_d_n16: f64 = (eq142_e1807_d_n16 * p.p246);
        let eq142_e1809_d_n17: f64 = (eq142_e1807_d_n17 * p.p246);
        let eq142_e1809_d_n18: f64 = (eq142_e1807_d_n18 * p.p246);
        let eq142_e1809_d_n19: f64 = (eq142_e1807_d_n19 * p.p246);
        let eq142_e1809_d_n20: f64 = (eq142_e1807_d_n20 * p.p246);
        let eq142_e1809_d_n21: f64 = (eq142_e1807_d_n21 * p.p246);
        let eq142_e1809_d_n22: f64 = (eq142_e1807_d_n22 * p.p246);
        let eq142_e1809_d_b0: f64 = (eq142_e1807_d_b0 * p.p246);
        let eq142_e1809_d_b1: f64 = (eq142_e1807_d_b1 * p.p246);
        let eq142_e1809_d_b2: f64 = (eq142_e1807_d_b2 * p.p246);
        let eq142_e1809_d_b3: f64 = (eq142_e1807_d_b3 * p.p246);
        let eq142_e1809_d_b4: f64 = (eq142_e1807_d_b4 * p.p246);
        let eq142_e1809_d_b5: f64 = (eq142_e1807_d_b5 * p.p246);
        let eq142_e1809_d_b6: f64 = (eq142_e1807_d_b6 * p.p246);
        let eq142_e1809_d_b7: f64 = (eq142_e1807_d_b7 * p.p246);
        let eq142_e1809_d_b8: f64 = (eq142_e1807_d_b8 * p.p246);
        let eq142_e1809_d_b9: f64 = (eq142_e1807_d_b9 * p.p246);
        let eq142_e1809_d_b10: f64 = (eq142_e1807_d_b10 * p.p246);
        let eq142_e1809_d_b11: f64 = (eq142_e1807_d_b11 * p.p246);
        let eq142_e1809_d_b12: f64 = (eq142_e1807_d_b12 * p.p246);
        let eq142_e1809_d_b13: f64 = (eq142_e1807_d_b13 * p.p246);
        let eq142_e1809_d_b14: f64 = (eq142_e1807_d_b14 * p.p246);
        let eq142_e1809_d_b15: f64 = (eq142_e1807_d_b15 * p.p246);
        let eq142_e1809_d_b16: f64 = (eq142_e1807_d_b16 * p.p246);
        let eq142_e1809_d_b17: f64 = (eq142_e1807_d_b17 * p.p246);
        let eq142_e1809_d_b18: f64 = (eq142_e1807_d_b18 * p.p246);
        let eq142_e1809_d_b19: f64 = (eq142_e1807_d_b19 * p.p246);
        let eq142_e1809_d_b20: f64 = (eq142_e1807_d_b20 * p.p246);
        let eq142_e1809_d_b21: f64 = (eq142_e1807_d_b21 * p.p246);
        let eq142_e1809_d_b22: f64 = (eq142_e1807_d_b22 * p.p246);
        let eq142_e1809_d_b23: f64 = (eq142_e1807_d_b23 * p.p246);
        let eq142_e1809_d_b24: f64 = (eq142_e1807_d_b24 * p.p246);
        let eq142_e1809_d_b25: f64 = (eq142_e1807_d_b25 * p.p246);
        let eq142_e1809_d_b26: f64 = (eq142_e1807_d_b26 * p.p246);
        let eq142_e1809_d_b27: f64 = (eq142_e1807_d_b27 * p.p246);
        let eq142_e1809_d_b28: f64 = (eq142_e1807_d_b28 * p.p246);
        let eq142_e1809_d_b29: f64 = (eq142_e1807_d_b29 * p.p246);
        let eq142_e1809_d_b30: f64 = (eq142_e1807_d_b30 * p.p246);
        let eq142_e1809_d_b31: f64 = (eq142_e1807_d_b31 * p.p246);
        let eq142_e1809_d_b32: f64 = (eq142_e1807_d_b32 * p.p246);
        let eq142_e1809_d_b33: f64 = (eq142_e1807_d_b33 * p.p246);
        let eq142_e1809_d_b34: f64 = (eq142_e1807_d_b34 * p.p246);
        let eq142_e1809_d_b35: f64 = (eq142_e1807_d_b35 * p.p246);
        let eq142_e1809_d_b36: f64 = (eq142_e1807_d_b36 * p.p246);
        let eq142_e1809_d_b37: f64 = (eq142_e1807_d_b37 * p.p246);
        let eq142_e1809_d_b38: f64 = (eq142_e1807_d_b38 * p.p246);
        let eq142_e1809_d_b39: f64 = (eq142_e1807_d_b39 * p.p246);
        let eq142_e1809_d_b40: f64 = (eq142_e1807_d_b40 * p.p246);
        let eq142_e1809_d_b41: f64 = (eq142_e1807_d_b41 * p.p246);
        let eq142_e1809_d_b42: f64 = (eq142_e1807_d_b42 * p.p246);
        let eq142_e1809_d_b43: f64 = (eq142_e1807_d_b43 * p.p246);
        let eq142_e1809_d_b44: f64 = (eq142_e1807_d_b44 * p.p246);
        let eq142_e1809_d_b45: f64 = (eq142_e1807_d_b45 * p.p246);
        let eq142_e1809_d_b46: f64 = (eq142_e1807_d_b46 * p.p246);
        let eq142_e1809_d_b47: f64 = (eq142_e1807_d_b47 * p.p246);
        let eq142_e1809_d_b48: f64 = (eq142_e1807_d_b48 * p.p246);
        let eq142_e1809_d_b49: f64 = (eq142_e1807_d_b49 * p.p246);
        let eq142_e1809_d_b50: f64 = (eq142_e1807_d_b50 * p.p246);
        let eq142_e1809_d_b51: f64 = (eq142_e1807_d_b51 * p.p246);
        let eq142_e1809_d_b52: f64 = (eq142_e1807_d_b52 * p.p246);
        let eq142_e1809_d_b53: f64 = (eq142_e1807_d_b53 * p.p246);
        let eq142_e1809_d_b54: f64 = (eq142_e1807_d_b54 * p.p246);
        (eq142_e1809, eq142_e1809_d_n0, eq142_e1809_d_n1, eq142_e1809_d_n2, eq142_e1809_d_n3, eq142_e1809_d_n4, eq142_e1809_d_n5, eq142_e1809_d_n6, eq142_e1809_d_n7, eq142_e1809_d_n8, eq142_e1809_d_n9, eq142_e1809_d_n10, eq142_e1809_d_n11, eq142_e1809_d_n12, eq142_e1809_d_n13, eq142_e1809_d_n14, eq142_e1809_d_n15, eq142_e1809_d_n16, eq142_e1809_d_n17, eq142_e1809_d_n18, eq142_e1809_d_n19, eq142_e1809_d_n20, eq142_e1809_d_n21, eq142_e1809_d_n22, eq142_e1809_d_b0, eq142_e1809_d_b1, eq142_e1809_d_b2, eq142_e1809_d_b3, eq142_e1809_d_b4, eq142_e1809_d_b5, eq142_e1809_d_b6, eq142_e1809_d_b7, eq142_e1809_d_b8, eq142_e1809_d_b9, eq142_e1809_d_b10, eq142_e1809_d_b11, eq142_e1809_d_b12, eq142_e1809_d_b13, eq142_e1809_d_b14, eq142_e1809_d_b15, eq142_e1809_d_b16, eq142_e1809_d_b17, eq142_e1809_d_b18, eq142_e1809_d_b19, eq142_e1809_d_b20, eq142_e1809_d_b21, eq142_e1809_d_b22, eq142_e1809_d_b23, eq142_e1809_d_b24, eq142_e1809_d_b25, eq142_e1809_d_b26, eq142_e1809_d_b27, eq142_e1809_d_b28, eq142_e1809_d_b29, eq142_e1809_d_b30, eq142_e1809_d_b31, eq142_e1809_d_b32, eq142_e1809_d_b33, eq142_e1809_d_b34, eq142_e1809_d_b35, eq142_e1809_d_b36, eq142_e1809_d_b37, eq142_e1809_d_b38, eq142_e1809_d_b39, eq142_e1809_d_b40, eq142_e1809_d_b41, eq142_e1809_d_b42, eq142_e1809_d_b43, eq142_e1809_d_b44, eq142_e1809_d_b45, eq142_e1809_d_b46, eq142_e1809_d_b47, eq142_e1809_d_b48, eq142_e1809_d_b49, eq142_e1809_d_b50, eq142_e1809_d_b51, eq142_e1809_d_b52, eq142_e1809_d_b53, eq142_e1809_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_value: f64 = eq142_e1811;
        let eq142_node_derivatives: [f64; 23] = [eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22];
        let eq142_branch_derivatives: [f64; 55] = [eq142_e1811_d_b0, eq142_e1811_d_b1, eq142_e1811_d_b2, eq142_e1811_d_b3, eq142_e1811_d_b4, eq142_e1811_d_b5, eq142_e1811_d_b6, eq142_e1811_d_b7, eq142_e1811_d_b8, eq142_e1811_d_b9, eq142_e1811_d_b10, eq142_e1811_d_b11, eq142_e1811_d_b12, eq142_e1811_d_b13, eq142_e1811_d_b14, eq142_e1811_d_b15, eq142_e1811_d_b16, eq142_e1811_d_b17, eq142_e1811_d_b18, eq142_e1811_d_b19, eq142_e1811_d_b20, eq142_e1811_d_b21, eq142_e1811_d_b22, eq142_e1811_d_b23, eq142_e1811_d_b24, eq142_e1811_d_b25, eq142_e1811_d_b26, eq142_e1811_d_b27, eq142_e1811_d_b28, eq142_e1811_d_b29, eq142_e1811_d_b30, eq142_e1811_d_b31, eq142_e1811_d_b32, eq142_e1811_d_b33, eq142_e1811_d_b34, eq142_e1811_d_b35, eq142_e1811_d_b36, eq142_e1811_d_b37, eq142_e1811_d_b38, eq142_e1811_d_b39, eq142_e1811_d_b40, eq142_e1811_d_b41, eq142_e1811_d_b42, eq142_e1811_d_b43, eq142_e1811_d_b44, eq142_e1811_d_b45, eq142_e1811_d_b46, eq142_e1811_d_b47, eq142_e1811_d_b48, eq142_e1811_d_b49, eq142_e1811_d_b50, eq142_e1811_d_b51, eq142_e1811_d_b52, eq142_e1811_d_b53, eq142_e1811_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq142_value),
            &eq142_node_derivatives,
            &eq142_branch_derivatives,
            multiplicity,
        );
        let (eq143_e1823, eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22, eq143_e1823_d_b0, eq143_e1823_d_b1, eq143_e1823_d_b2, eq143_e1823_d_b3, eq143_e1823_d_b4, eq143_e1823_d_b5, eq143_e1823_d_b6, eq143_e1823_d_b7, eq143_e1823_d_b8, eq143_e1823_d_b9, eq143_e1823_d_b10, eq143_e1823_d_b11, eq143_e1823_d_b12, eq143_e1823_d_b13, eq143_e1823_d_b14, eq143_e1823_d_b15, eq143_e1823_d_b16, eq143_e1823_d_b17, eq143_e1823_d_b18, eq143_e1823_d_b19, eq143_e1823_d_b20, eq143_e1823_d_b21, eq143_e1823_d_b22, eq143_e1823_d_b23, eq143_e1823_d_b24, eq143_e1823_d_b25, eq143_e1823_d_b26, eq143_e1823_d_b27, eq143_e1823_d_b28, eq143_e1823_d_b29, eq143_e1823_d_b30, eq143_e1823_d_b31, eq143_e1823_d_b32, eq143_e1823_d_b33, eq143_e1823_d_b34, eq143_e1823_d_b35, eq143_e1823_d_b36, eq143_e1823_d_b37, eq143_e1823_d_b38, eq143_e1823_d_b39, eq143_e1823_d_b40, eq143_e1823_d_b41, eq143_e1823_d_b42, eq143_e1823_d_b43, eq143_e1823_d_b44, eq143_e1823_d_b45, eq143_e1823_d_b46, eq143_e1823_d_b47, eq143_e1823_d_b48, eq143_e1823_d_b49, eq143_e1823_d_b50, eq143_e1823_d_b51, eq143_e1823_d_b52, eq143_e1823_d_b53, eq143_e1823_d_b54,) = {
    if ((!s.b[575]) && s.b[578]) {
        let eq143_e1819: f64 = (p.p251 * s.v[240]);
        let eq143_e1820: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 42, eq143_e1819);
        let eq143_e1820_d_n0: f64 = ((p.p251 * s.dn[240][0]) * ddt_scale);
        let eq143_e1820_d_n1: f64 = ((p.p251 * s.dn[240][1]) * ddt_scale);
        let eq143_e1820_d_n2: f64 = ((p.p251 * s.dn[240][2]) * ddt_scale);
        let eq143_e1820_d_n3: f64 = ((p.p251 * s.dn[240][3]) * ddt_scale);
        let eq143_e1820_d_n4: f64 = ((p.p251 * s.dn[240][4]) * ddt_scale);
        let eq143_e1820_d_n5: f64 = ((p.p251 * s.dn[240][5]) * ddt_scale);
        let eq143_e1820_d_n6: f64 = ((p.p251 * s.dn[240][6]) * ddt_scale);
        let eq143_e1820_d_n7: f64 = ((p.p251 * s.dn[240][7]) * ddt_scale);
        let eq143_e1820_d_n8: f64 = ((p.p251 * s.dn[240][8]) * ddt_scale);
        let eq143_e1820_d_n9: f64 = ((p.p251 * s.dn[240][9]) * ddt_scale);
        let eq143_e1820_d_n10: f64 = ((p.p251 * s.dn[240][10]) * ddt_scale);
        let eq143_e1820_d_n11: f64 = ((p.p251 * s.dn[240][11]) * ddt_scale);
        let eq143_e1820_d_n12: f64 = ((p.p251 * s.dn[240][12]) * ddt_scale);
        let eq143_e1820_d_n13: f64 = ((p.p251 * s.dn[240][13]) * ddt_scale);
        let eq143_e1820_d_n14: f64 = ((p.p251 * s.dn[240][14]) * ddt_scale);
        let eq143_e1820_d_n15: f64 = ((p.p251 * s.dn[240][15]) * ddt_scale);
        let eq143_e1820_d_n16: f64 = ((p.p251 * s.dn[240][16]) * ddt_scale);
        let eq143_e1820_d_n17: f64 = ((p.p251 * s.dn[240][17]) * ddt_scale);
        let eq143_e1820_d_n18: f64 = ((p.p251 * s.dn[240][18]) * ddt_scale);
        let eq143_e1820_d_n19: f64 = ((p.p251 * s.dn[240][19]) * ddt_scale);
        let eq143_e1820_d_n20: f64 = ((p.p251 * s.dn[240][20]) * ddt_scale);
        let eq143_e1820_d_n21: f64 = ((p.p251 * s.dn[240][21]) * ddt_scale);
        let eq143_e1820_d_n22: f64 = ((p.p251 * s.dn[240][22]) * ddt_scale);
        let eq143_e1820_d_b0: f64 = ((p.p251 * s.db[240][0]) * ddt_scale);
        let eq143_e1820_d_b1: f64 = ((p.p251 * s.db[240][1]) * ddt_scale);
        let eq143_e1820_d_b2: f64 = ((p.p251 * s.db[240][2]) * ddt_scale);
        let eq143_e1820_d_b3: f64 = ((p.p251 * s.db[240][3]) * ddt_scale);
        let eq143_e1820_d_b4: f64 = ((p.p251 * s.db[240][4]) * ddt_scale);
        let eq143_e1820_d_b5: f64 = ((p.p251 * s.db[240][5]) * ddt_scale);
        let eq143_e1820_d_b6: f64 = ((p.p251 * s.db[240][6]) * ddt_scale);
        let eq143_e1820_d_b7: f64 = ((p.p251 * s.db[240][7]) * ddt_scale);
        let eq143_e1820_d_b8: f64 = ((p.p251 * s.db[240][8]) * ddt_scale);
        let eq143_e1820_d_b9: f64 = ((p.p251 * s.db[240][9]) * ddt_scale);
        let eq143_e1820_d_b10: f64 = ((p.p251 * s.db[240][10]) * ddt_scale);
        let eq143_e1820_d_b11: f64 = ((p.p251 * s.db[240][11]) * ddt_scale);
        let eq143_e1820_d_b12: f64 = ((p.p251 * s.db[240][12]) * ddt_scale);
        let eq143_e1820_d_b13: f64 = ((p.p251 * s.db[240][13]) * ddt_scale);
        let eq143_e1820_d_b14: f64 = ((p.p251 * s.db[240][14]) * ddt_scale);
        let eq143_e1820_d_b15: f64 = ((p.p251 * s.db[240][15]) * ddt_scale);
        let eq143_e1820_d_b16: f64 = ((p.p251 * s.db[240][16]) * ddt_scale);
        let eq143_e1820_d_b17: f64 = ((p.p251 * s.db[240][17]) * ddt_scale);
        let eq143_e1820_d_b18: f64 = ((p.p251 * s.db[240][18]) * ddt_scale);
        let eq143_e1820_d_b19: f64 = ((p.p251 * s.db[240][19]) * ddt_scale);
        let eq143_e1820_d_b20: f64 = ((p.p251 * s.db[240][20]) * ddt_scale);
        let eq143_e1820_d_b21: f64 = ((p.p251 * s.db[240][21]) * ddt_scale);
        let eq143_e1820_d_b22: f64 = ((p.p251 * s.db[240][22]) * ddt_scale);
        let eq143_e1820_d_b23: f64 = ((p.p251 * s.db[240][23]) * ddt_scale);
        let eq143_e1820_d_b24: f64 = ((p.p251 * s.db[240][24]) * ddt_scale);
        let eq143_e1820_d_b25: f64 = ((p.p251 * s.db[240][25]) * ddt_scale);
        let eq143_e1820_d_b26: f64 = ((p.p251 * s.db[240][26]) * ddt_scale);
        let eq143_e1820_d_b27: f64 = ((p.p251 * s.db[240][27]) * ddt_scale);
        let eq143_e1820_d_b28: f64 = ((p.p251 * s.db[240][28]) * ddt_scale);
        let eq143_e1820_d_b29: f64 = ((p.p251 * s.db[240][29]) * ddt_scale);
        let eq143_e1820_d_b30: f64 = ((p.p251 * s.db[240][30]) * ddt_scale);
        let eq143_e1820_d_b31: f64 = ((p.p251 * s.db[240][31]) * ddt_scale);
        let eq143_e1820_d_b32: f64 = ((p.p251 * s.db[240][32]) * ddt_scale);
        let eq143_e1820_d_b33: f64 = ((p.p251 * s.db[240][33]) * ddt_scale);
        let eq143_e1820_d_b34: f64 = ((p.p251 * s.db[240][34]) * ddt_scale);
        let eq143_e1820_d_b35: f64 = ((p.p251 * s.db[240][35]) * ddt_scale);
        let eq143_e1820_d_b36: f64 = ((p.p251 * s.db[240][36]) * ddt_scale);
        let eq143_e1820_d_b37: f64 = ((p.p251 * s.db[240][37]) * ddt_scale);
        let eq143_e1820_d_b38: f64 = ((p.p251 * s.db[240][38]) * ddt_scale);
        let eq143_e1820_d_b39: f64 = ((p.p251 * s.db[240][39]) * ddt_scale);
        let eq143_e1820_d_b40: f64 = ((p.p251 * s.db[240][40]) * ddt_scale);
        let eq143_e1820_d_b41: f64 = ((p.p251 * s.db[240][41]) * ddt_scale);
        let eq143_e1820_d_b42: f64 = ((p.p251 * s.db[240][42]) * ddt_scale);
        let eq143_e1820_d_b43: f64 = ((p.p251 * s.db[240][43]) * ddt_scale);
        let eq143_e1820_d_b44: f64 = ((p.p251 * s.db[240][44]) * ddt_scale);
        let eq143_e1820_d_b45: f64 = ((p.p251 * s.db[240][45]) * ddt_scale);
        let eq143_e1820_d_b46: f64 = ((p.p251 * s.db[240][46]) * ddt_scale);
        let eq143_e1820_d_b47: f64 = ((p.p251 * s.db[240][47]) * ddt_scale);
        let eq143_e1820_d_b48: f64 = ((p.p251 * s.db[240][48]) * ddt_scale);
        let eq143_e1820_d_b49: f64 = ((p.p251 * s.db[240][49]) * ddt_scale);
        let eq143_e1820_d_b50: f64 = ((p.p251 * s.db[240][50]) * ddt_scale);
        let eq143_e1820_d_b51: f64 = ((p.p251 * s.db[240][51]) * ddt_scale);
        let eq143_e1820_d_b52: f64 = ((p.p251 * s.db[240][52]) * ddt_scale);
        let eq143_e1820_d_b53: f64 = ((p.p251 * s.db[240][53]) * ddt_scale);
        let eq143_e1820_d_b54: f64 = ((p.p251 * s.db[240][54]) * ddt_scale);
        let eq143_e1821: f64 = (p.p7 * eq143_e1820);
        let eq143_e1821_d_n0: f64 = (p.p7 * eq143_e1820_d_n0);
        let eq143_e1821_d_n1: f64 = (p.p7 * eq143_e1820_d_n1);
        let eq143_e1821_d_n2: f64 = (p.p7 * eq143_e1820_d_n2);
        let eq143_e1821_d_n3: f64 = (p.p7 * eq143_e1820_d_n3);
        let eq143_e1821_d_n4: f64 = (p.p7 * eq143_e1820_d_n4);
        let eq143_e1821_d_n5: f64 = (p.p7 * eq143_e1820_d_n5);
        let eq143_e1821_d_n6: f64 = (p.p7 * eq143_e1820_d_n6);
        let eq143_e1821_d_n7: f64 = (p.p7 * eq143_e1820_d_n7);
        let eq143_e1821_d_n8: f64 = (p.p7 * eq143_e1820_d_n8);
        let eq143_e1821_d_n9: f64 = (p.p7 * eq143_e1820_d_n9);
        let eq143_e1821_d_n10: f64 = (p.p7 * eq143_e1820_d_n10);
        let eq143_e1821_d_n11: f64 = (p.p7 * eq143_e1820_d_n11);
        let eq143_e1821_d_n12: f64 = (p.p7 * eq143_e1820_d_n12);
        let eq143_e1821_d_n13: f64 = (p.p7 * eq143_e1820_d_n13);
        let eq143_e1821_d_n14: f64 = (p.p7 * eq143_e1820_d_n14);
        let eq143_e1821_d_n15: f64 = (p.p7 * eq143_e1820_d_n15);
        let eq143_e1821_d_n16: f64 = (p.p7 * eq143_e1820_d_n16);
        let eq143_e1821_d_n17: f64 = (p.p7 * eq143_e1820_d_n17);
        let eq143_e1821_d_n18: f64 = (p.p7 * eq143_e1820_d_n18);
        let eq143_e1821_d_n19: f64 = (p.p7 * eq143_e1820_d_n19);
        let eq143_e1821_d_n20: f64 = (p.p7 * eq143_e1820_d_n20);
        let eq143_e1821_d_n21: f64 = (p.p7 * eq143_e1820_d_n21);
        let eq143_e1821_d_n22: f64 = (p.p7 * eq143_e1820_d_n22);
        let eq143_e1821_d_b0: f64 = (p.p7 * eq143_e1820_d_b0);
        let eq143_e1821_d_b1: f64 = (p.p7 * eq143_e1820_d_b1);
        let eq143_e1821_d_b2: f64 = (p.p7 * eq143_e1820_d_b2);
        let eq143_e1821_d_b3: f64 = (p.p7 * eq143_e1820_d_b3);
        let eq143_e1821_d_b4: f64 = (p.p7 * eq143_e1820_d_b4);
        let eq143_e1821_d_b5: f64 = (p.p7 * eq143_e1820_d_b5);
        let eq143_e1821_d_b6: f64 = (p.p7 * eq143_e1820_d_b6);
        let eq143_e1821_d_b7: f64 = (p.p7 * eq143_e1820_d_b7);
        let eq143_e1821_d_b8: f64 = (p.p7 * eq143_e1820_d_b8);
        let eq143_e1821_d_b9: f64 = (p.p7 * eq143_e1820_d_b9);
        let eq143_e1821_d_b10: f64 = (p.p7 * eq143_e1820_d_b10);
        let eq143_e1821_d_b11: f64 = (p.p7 * eq143_e1820_d_b11);
        let eq143_e1821_d_b12: f64 = (p.p7 * eq143_e1820_d_b12);
        let eq143_e1821_d_b13: f64 = (p.p7 * eq143_e1820_d_b13);
        let eq143_e1821_d_b14: f64 = (p.p7 * eq143_e1820_d_b14);
        let eq143_e1821_d_b15: f64 = (p.p7 * eq143_e1820_d_b15);
        let eq143_e1821_d_b16: f64 = (p.p7 * eq143_e1820_d_b16);
        let eq143_e1821_d_b17: f64 = (p.p7 * eq143_e1820_d_b17);
        let eq143_e1821_d_b18: f64 = (p.p7 * eq143_e1820_d_b18);
        let eq143_e1821_d_b19: f64 = (p.p7 * eq143_e1820_d_b19);
        let eq143_e1821_d_b20: f64 = (p.p7 * eq143_e1820_d_b20);
        let eq143_e1821_d_b21: f64 = (p.p7 * eq143_e1820_d_b21);
        let eq143_e1821_d_b22: f64 = (p.p7 * eq143_e1820_d_b22);
        let eq143_e1821_d_b23: f64 = (p.p7 * eq143_e1820_d_b23);
        let eq143_e1821_d_b24: f64 = (p.p7 * eq143_e1820_d_b24);
        let eq143_e1821_d_b25: f64 = (p.p7 * eq143_e1820_d_b25);
        let eq143_e1821_d_b26: f64 = (p.p7 * eq143_e1820_d_b26);
        let eq143_e1821_d_b27: f64 = (p.p7 * eq143_e1820_d_b27);
        let eq143_e1821_d_b28: f64 = (p.p7 * eq143_e1820_d_b28);
        let eq143_e1821_d_b29: f64 = (p.p7 * eq143_e1820_d_b29);
        let eq143_e1821_d_b30: f64 = (p.p7 * eq143_e1820_d_b30);
        let eq143_e1821_d_b31: f64 = (p.p7 * eq143_e1820_d_b31);
        let eq143_e1821_d_b32: f64 = (p.p7 * eq143_e1820_d_b32);
        let eq143_e1821_d_b33: f64 = (p.p7 * eq143_e1820_d_b33);
        let eq143_e1821_d_b34: f64 = (p.p7 * eq143_e1820_d_b34);
        let eq143_e1821_d_b35: f64 = (p.p7 * eq143_e1820_d_b35);
        let eq143_e1821_d_b36: f64 = (p.p7 * eq143_e1820_d_b36);
        let eq143_e1821_d_b37: f64 = (p.p7 * eq143_e1820_d_b37);
        let eq143_e1821_d_b38: f64 = (p.p7 * eq143_e1820_d_b38);
        let eq143_e1821_d_b39: f64 = (p.p7 * eq143_e1820_d_b39);
        let eq143_e1821_d_b40: f64 = (p.p7 * eq143_e1820_d_b40);
        let eq143_e1821_d_b41: f64 = (p.p7 * eq143_e1820_d_b41);
        let eq143_e1821_d_b42: f64 = (p.p7 * eq143_e1820_d_b42);
        let eq143_e1821_d_b43: f64 = (p.p7 * eq143_e1820_d_b43);
        let eq143_e1821_d_b44: f64 = (p.p7 * eq143_e1820_d_b44);
        let eq143_e1821_d_b45: f64 = (p.p7 * eq143_e1820_d_b45);
        let eq143_e1821_d_b46: f64 = (p.p7 * eq143_e1820_d_b46);
        let eq143_e1821_d_b47: f64 = (p.p7 * eq143_e1820_d_b47);
        let eq143_e1821_d_b48: f64 = (p.p7 * eq143_e1820_d_b48);
        let eq143_e1821_d_b49: f64 = (p.p7 * eq143_e1820_d_b49);
        let eq143_e1821_d_b50: f64 = (p.p7 * eq143_e1820_d_b50);
        let eq143_e1821_d_b51: f64 = (p.p7 * eq143_e1820_d_b51);
        let eq143_e1821_d_b52: f64 = (p.p7 * eq143_e1820_d_b52);
        let eq143_e1821_d_b53: f64 = (p.p7 * eq143_e1820_d_b53);
        let eq143_e1821_d_b54: f64 = (p.p7 * eq143_e1820_d_b54);
        (eq143_e1821, eq143_e1821_d_n0, eq143_e1821_d_n1, eq143_e1821_d_n2, eq143_e1821_d_n3, eq143_e1821_d_n4, eq143_e1821_d_n5, eq143_e1821_d_n6, eq143_e1821_d_n7, eq143_e1821_d_n8, eq143_e1821_d_n9, eq143_e1821_d_n10, eq143_e1821_d_n11, eq143_e1821_d_n12, eq143_e1821_d_n13, eq143_e1821_d_n14, eq143_e1821_d_n15, eq143_e1821_d_n16, eq143_e1821_d_n17, eq143_e1821_d_n18, eq143_e1821_d_n19, eq143_e1821_d_n20, eq143_e1821_d_n21, eq143_e1821_d_n22, eq143_e1821_d_b0, eq143_e1821_d_b1, eq143_e1821_d_b2, eq143_e1821_d_b3, eq143_e1821_d_b4, eq143_e1821_d_b5, eq143_e1821_d_b6, eq143_e1821_d_b7, eq143_e1821_d_b8, eq143_e1821_d_b9, eq143_e1821_d_b10, eq143_e1821_d_b11, eq143_e1821_d_b12, eq143_e1821_d_b13, eq143_e1821_d_b14, eq143_e1821_d_b15, eq143_e1821_d_b16, eq143_e1821_d_b17, eq143_e1821_d_b18, eq143_e1821_d_b19, eq143_e1821_d_b20, eq143_e1821_d_b21, eq143_e1821_d_b22, eq143_e1821_d_b23, eq143_e1821_d_b24, eq143_e1821_d_b25, eq143_e1821_d_b26, eq143_e1821_d_b27, eq143_e1821_d_b28, eq143_e1821_d_b29, eq143_e1821_d_b30, eq143_e1821_d_b31, eq143_e1821_d_b32, eq143_e1821_d_b33, eq143_e1821_d_b34, eq143_e1821_d_b35, eq143_e1821_d_b36, eq143_e1821_d_b37, eq143_e1821_d_b38, eq143_e1821_d_b39, eq143_e1821_d_b40, eq143_e1821_d_b41, eq143_e1821_d_b42, eq143_e1821_d_b43, eq143_e1821_d_b44, eq143_e1821_d_b45, eq143_e1821_d_b46, eq143_e1821_d_b47, eq143_e1821_d_b48, eq143_e1821_d_b49, eq143_e1821_d_b50, eq143_e1821_d_b51, eq143_e1821_d_b52, eq143_e1821_d_b53, eq143_e1821_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq143_value: f64 = eq143_e1823;
        let eq143_node_derivatives: [f64; 23] = [eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22];
        let eq143_branch_derivatives: [f64; 55] = [eq143_e1823_d_b0, eq143_e1823_d_b1, eq143_e1823_d_b2, eq143_e1823_d_b3, eq143_e1823_d_b4, eq143_e1823_d_b5, eq143_e1823_d_b6, eq143_e1823_d_b7, eq143_e1823_d_b8, eq143_e1823_d_b9, eq143_e1823_d_b10, eq143_e1823_d_b11, eq143_e1823_d_b12, eq143_e1823_d_b13, eq143_e1823_d_b14, eq143_e1823_d_b15, eq143_e1823_d_b16, eq143_e1823_d_b17, eq143_e1823_d_b18, eq143_e1823_d_b19, eq143_e1823_d_b20, eq143_e1823_d_b21, eq143_e1823_d_b22, eq143_e1823_d_b23, eq143_e1823_d_b24, eq143_e1823_d_b25, eq143_e1823_d_b26, eq143_e1823_d_b27, eq143_e1823_d_b28, eq143_e1823_d_b29, eq143_e1823_d_b30, eq143_e1823_d_b31, eq143_e1823_d_b32, eq143_e1823_d_b33, eq143_e1823_d_b34, eq143_e1823_d_b35, eq143_e1823_d_b36, eq143_e1823_d_b37, eq143_e1823_d_b38, eq143_e1823_d_b39, eq143_e1823_d_b40, eq143_e1823_d_b41, eq143_e1823_d_b42, eq143_e1823_d_b43, eq143_e1823_d_b44, eq143_e1823_d_b45, eq143_e1823_d_b46, eq143_e1823_d_b47, eq143_e1823_d_b48, eq143_e1823_d_b49, eq143_e1823_d_b50, eq143_e1823_d_b51, eq143_e1823_d_b52, eq143_e1823_d_b53, eq143_e1823_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq143_value),
            &eq143_node_derivatives,
            &eq143_branch_derivatives,
            multiplicity,
        );
        let (eq144_e1832, eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22, eq144_e1832_d_b0, eq144_e1832_d_b1, eq144_e1832_d_b2, eq144_e1832_d_b3, eq144_e1832_d_b4, eq144_e1832_d_b5, eq144_e1832_d_b6, eq144_e1832_d_b7, eq144_e1832_d_b8, eq144_e1832_d_b9, eq144_e1832_d_b10, eq144_e1832_d_b11, eq144_e1832_d_b12, eq144_e1832_d_b13, eq144_e1832_d_b14, eq144_e1832_d_b15, eq144_e1832_d_b16, eq144_e1832_d_b17, eq144_e1832_d_b18, eq144_e1832_d_b19, eq144_e1832_d_b20, eq144_e1832_d_b21, eq144_e1832_d_b22, eq144_e1832_d_b23, eq144_e1832_d_b24, eq144_e1832_d_b25, eq144_e1832_d_b26, eq144_e1832_d_b27, eq144_e1832_d_b28, eq144_e1832_d_b29, eq144_e1832_d_b30, eq144_e1832_d_b31, eq144_e1832_d_b32, eq144_e1832_d_b33, eq144_e1832_d_b34, eq144_e1832_d_b35, eq144_e1832_d_b36, eq144_e1832_d_b37, eq144_e1832_d_b38, eq144_e1832_d_b39, eq144_e1832_d_b40, eq144_e1832_d_b41, eq144_e1832_d_b42, eq144_e1832_d_b43, eq144_e1832_d_b44, eq144_e1832_d_b45, eq144_e1832_d_b46, eq144_e1832_d_b47, eq144_e1832_d_b48, eq144_e1832_d_b49, eq144_e1832_d_b50, eq144_e1832_d_b51, eq144_e1832_d_b52, eq144_e1832_d_b53, eq144_e1832_d_b54,) = {
    if (s.b[580] && s.b[581]) {
        let eq144_e1829: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 43, s.v[253]);
        let eq144_e1830: f64 = (p.p7 * eq144_e1829);
        let eq144_e1830_d_n0: f64 = (p.p7 * (s.dn[253][0] * ddt_scale));
        let eq144_e1830_d_n1: f64 = (p.p7 * (s.dn[253][1] * ddt_scale));
        let eq144_e1830_d_n2: f64 = (p.p7 * (s.dn[253][2] * ddt_scale));
        let eq144_e1830_d_n3: f64 = (p.p7 * (s.dn[253][3] * ddt_scale));
        let eq144_e1830_d_n4: f64 = (p.p7 * (s.dn[253][4] * ddt_scale));
        let eq144_e1830_d_n5: f64 = (p.p7 * (s.dn[253][5] * ddt_scale));
        let eq144_e1830_d_n6: f64 = (p.p7 * (s.dn[253][6] * ddt_scale));
        let eq144_e1830_d_n7: f64 = (p.p7 * (s.dn[253][7] * ddt_scale));
        let eq144_e1830_d_n8: f64 = (p.p7 * (s.dn[253][8] * ddt_scale));
        let eq144_e1830_d_n9: f64 = (p.p7 * (s.dn[253][9] * ddt_scale));
        let eq144_e1830_d_n10: f64 = (p.p7 * (s.dn[253][10] * ddt_scale));
        let eq144_e1830_d_n11: f64 = (p.p7 * (s.dn[253][11] * ddt_scale));
        let eq144_e1830_d_n12: f64 = (p.p7 * (s.dn[253][12] * ddt_scale));
        let eq144_e1830_d_n13: f64 = (p.p7 * (s.dn[253][13] * ddt_scale));
        let eq144_e1830_d_n14: f64 = (p.p7 * (s.dn[253][14] * ddt_scale));
        let eq144_e1830_d_n15: f64 = (p.p7 * (s.dn[253][15] * ddt_scale));
        let eq144_e1830_d_n16: f64 = (p.p7 * (s.dn[253][16] * ddt_scale));
        let eq144_e1830_d_n17: f64 = (p.p7 * (s.dn[253][17] * ddt_scale));
        let eq144_e1830_d_n18: f64 = (p.p7 * (s.dn[253][18] * ddt_scale));
        let eq144_e1830_d_n19: f64 = (p.p7 * (s.dn[253][19] * ddt_scale));
        let eq144_e1830_d_n20: f64 = (p.p7 * (s.dn[253][20] * ddt_scale));
        let eq144_e1830_d_n21: f64 = (p.p7 * (s.dn[253][21] * ddt_scale));
        let eq144_e1830_d_n22: f64 = (p.p7 * (s.dn[253][22] * ddt_scale));
        let eq144_e1830_d_b0: f64 = (p.p7 * (s.db[253][0] * ddt_scale));
        let eq144_e1830_d_b1: f64 = (p.p7 * (s.db[253][1] * ddt_scale));
        let eq144_e1830_d_b2: f64 = (p.p7 * (s.db[253][2] * ddt_scale));
        let eq144_e1830_d_b3: f64 = (p.p7 * (s.db[253][3] * ddt_scale));
        let eq144_e1830_d_b4: f64 = (p.p7 * (s.db[253][4] * ddt_scale));
        let eq144_e1830_d_b5: f64 = (p.p7 * (s.db[253][5] * ddt_scale));
        let eq144_e1830_d_b6: f64 = (p.p7 * (s.db[253][6] * ddt_scale));
        let eq144_e1830_d_b7: f64 = (p.p7 * (s.db[253][7] * ddt_scale));
        let eq144_e1830_d_b8: f64 = (p.p7 * (s.db[253][8] * ddt_scale));
        let eq144_e1830_d_b9: f64 = (p.p7 * (s.db[253][9] * ddt_scale));
        let eq144_e1830_d_b10: f64 = (p.p7 * (s.db[253][10] * ddt_scale));
        let eq144_e1830_d_b11: f64 = (p.p7 * (s.db[253][11] * ddt_scale));
        let eq144_e1830_d_b12: f64 = (p.p7 * (s.db[253][12] * ddt_scale));
        let eq144_e1830_d_b13: f64 = (p.p7 * (s.db[253][13] * ddt_scale));
        let eq144_e1830_d_b14: f64 = (p.p7 * (s.db[253][14] * ddt_scale));
        let eq144_e1830_d_b15: f64 = (p.p7 * (s.db[253][15] * ddt_scale));
        let eq144_e1830_d_b16: f64 = (p.p7 * (s.db[253][16] * ddt_scale));
        let eq144_e1830_d_b17: f64 = (p.p7 * (s.db[253][17] * ddt_scale));
        let eq144_e1830_d_b18: f64 = (p.p7 * (s.db[253][18] * ddt_scale));
        let eq144_e1830_d_b19: f64 = (p.p7 * (s.db[253][19] * ddt_scale));
        let eq144_e1830_d_b20: f64 = (p.p7 * (s.db[253][20] * ddt_scale));
        let eq144_e1830_d_b21: f64 = (p.p7 * (s.db[253][21] * ddt_scale));
        let eq144_e1830_d_b22: f64 = (p.p7 * (s.db[253][22] * ddt_scale));
        let eq144_e1830_d_b23: f64 = (p.p7 * (s.db[253][23] * ddt_scale));
        let eq144_e1830_d_b24: f64 = (p.p7 * (s.db[253][24] * ddt_scale));
        let eq144_e1830_d_b25: f64 = (p.p7 * (s.db[253][25] * ddt_scale));
        let eq144_e1830_d_b26: f64 = (p.p7 * (s.db[253][26] * ddt_scale));
        let eq144_e1830_d_b27: f64 = (p.p7 * (s.db[253][27] * ddt_scale));
        let eq144_e1830_d_b28: f64 = (p.p7 * (s.db[253][28] * ddt_scale));
        let eq144_e1830_d_b29: f64 = (p.p7 * (s.db[253][29] * ddt_scale));
        let eq144_e1830_d_b30: f64 = (p.p7 * (s.db[253][30] * ddt_scale));
        let eq144_e1830_d_b31: f64 = (p.p7 * (s.db[253][31] * ddt_scale));
        let eq144_e1830_d_b32: f64 = (p.p7 * (s.db[253][32] * ddt_scale));
        let eq144_e1830_d_b33: f64 = (p.p7 * (s.db[253][33] * ddt_scale));
        let eq144_e1830_d_b34: f64 = (p.p7 * (s.db[253][34] * ddt_scale));
        let eq144_e1830_d_b35: f64 = (p.p7 * (s.db[253][35] * ddt_scale));
        let eq144_e1830_d_b36: f64 = (p.p7 * (s.db[253][36] * ddt_scale));
        let eq144_e1830_d_b37: f64 = (p.p7 * (s.db[253][37] * ddt_scale));
        let eq144_e1830_d_b38: f64 = (p.p7 * (s.db[253][38] * ddt_scale));
        let eq144_e1830_d_b39: f64 = (p.p7 * (s.db[253][39] * ddt_scale));
        let eq144_e1830_d_b40: f64 = (p.p7 * (s.db[253][40] * ddt_scale));
        let eq144_e1830_d_b41: f64 = (p.p7 * (s.db[253][41] * ddt_scale));
        let eq144_e1830_d_b42: f64 = (p.p7 * (s.db[253][42] * ddt_scale));
        let eq144_e1830_d_b43: f64 = (p.p7 * (s.db[253][43] * ddt_scale));
        let eq144_e1830_d_b44: f64 = (p.p7 * (s.db[253][44] * ddt_scale));
        let eq144_e1830_d_b45: f64 = (p.p7 * (s.db[253][45] * ddt_scale));
        let eq144_e1830_d_b46: f64 = (p.p7 * (s.db[253][46] * ddt_scale));
        let eq144_e1830_d_b47: f64 = (p.p7 * (s.db[253][47] * ddt_scale));
        let eq144_e1830_d_b48: f64 = (p.p7 * (s.db[253][48] * ddt_scale));
        let eq144_e1830_d_b49: f64 = (p.p7 * (s.db[253][49] * ddt_scale));
        let eq144_e1830_d_b50: f64 = (p.p7 * (s.db[253][50] * ddt_scale));
        let eq144_e1830_d_b51: f64 = (p.p7 * (s.db[253][51] * ddt_scale));
        let eq144_e1830_d_b52: f64 = (p.p7 * (s.db[253][52] * ddt_scale));
        let eq144_e1830_d_b53: f64 = (p.p7 * (s.db[253][53] * ddt_scale));
        let eq144_e1830_d_b54: f64 = (p.p7 * (s.db[253][54] * ddt_scale));
        (eq144_e1830, eq144_e1830_d_n0, eq144_e1830_d_n1, eq144_e1830_d_n2, eq144_e1830_d_n3, eq144_e1830_d_n4, eq144_e1830_d_n5, eq144_e1830_d_n6, eq144_e1830_d_n7, eq144_e1830_d_n8, eq144_e1830_d_n9, eq144_e1830_d_n10, eq144_e1830_d_n11, eq144_e1830_d_n12, eq144_e1830_d_n13, eq144_e1830_d_n14, eq144_e1830_d_n15, eq144_e1830_d_n16, eq144_e1830_d_n17, eq144_e1830_d_n18, eq144_e1830_d_n19, eq144_e1830_d_n20, eq144_e1830_d_n21, eq144_e1830_d_n22, eq144_e1830_d_b0, eq144_e1830_d_b1, eq144_e1830_d_b2, eq144_e1830_d_b3, eq144_e1830_d_b4, eq144_e1830_d_b5, eq144_e1830_d_b6, eq144_e1830_d_b7, eq144_e1830_d_b8, eq144_e1830_d_b9, eq144_e1830_d_b10, eq144_e1830_d_b11, eq144_e1830_d_b12, eq144_e1830_d_b13, eq144_e1830_d_b14, eq144_e1830_d_b15, eq144_e1830_d_b16, eq144_e1830_d_b17, eq144_e1830_d_b18, eq144_e1830_d_b19, eq144_e1830_d_b20, eq144_e1830_d_b21, eq144_e1830_d_b22, eq144_e1830_d_b23, eq144_e1830_d_b24, eq144_e1830_d_b25, eq144_e1830_d_b26, eq144_e1830_d_b27, eq144_e1830_d_b28, eq144_e1830_d_b29, eq144_e1830_d_b30, eq144_e1830_d_b31, eq144_e1830_d_b32, eq144_e1830_d_b33, eq144_e1830_d_b34, eq144_e1830_d_b35, eq144_e1830_d_b36, eq144_e1830_d_b37, eq144_e1830_d_b38, eq144_e1830_d_b39, eq144_e1830_d_b40, eq144_e1830_d_b41, eq144_e1830_d_b42, eq144_e1830_d_b43, eq144_e1830_d_b44, eq144_e1830_d_b45, eq144_e1830_d_b46, eq144_e1830_d_b47, eq144_e1830_d_b48, eq144_e1830_d_b49, eq144_e1830_d_b50, eq144_e1830_d_b51, eq144_e1830_d_b52, eq144_e1830_d_b53, eq144_e1830_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_value: f64 = eq144_e1832;
        let eq144_node_derivatives: [f64; 23] = [eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22];
        let eq144_branch_derivatives: [f64; 55] = [eq144_e1832_d_b0, eq144_e1832_d_b1, eq144_e1832_d_b2, eq144_e1832_d_b3, eq144_e1832_d_b4, eq144_e1832_d_b5, eq144_e1832_d_b6, eq144_e1832_d_b7, eq144_e1832_d_b8, eq144_e1832_d_b9, eq144_e1832_d_b10, eq144_e1832_d_b11, eq144_e1832_d_b12, eq144_e1832_d_b13, eq144_e1832_d_b14, eq144_e1832_d_b15, eq144_e1832_d_b16, eq144_e1832_d_b17, eq144_e1832_d_b18, eq144_e1832_d_b19, eq144_e1832_d_b20, eq144_e1832_d_b21, eq144_e1832_d_b22, eq144_e1832_d_b23, eq144_e1832_d_b24, eq144_e1832_d_b25, eq144_e1832_d_b26, eq144_e1832_d_b27, eq144_e1832_d_b28, eq144_e1832_d_b29, eq144_e1832_d_b30, eq144_e1832_d_b31, eq144_e1832_d_b32, eq144_e1832_d_b33, eq144_e1832_d_b34, eq144_e1832_d_b35, eq144_e1832_d_b36, eq144_e1832_d_b37, eq144_e1832_d_b38, eq144_e1832_d_b39, eq144_e1832_d_b40, eq144_e1832_d_b41, eq144_e1832_d_b42, eq144_e1832_d_b43, eq144_e1832_d_b44, eq144_e1832_d_b45, eq144_e1832_d_b46, eq144_e1832_d_b47, eq144_e1832_d_b48, eq144_e1832_d_b49, eq144_e1832_d_b50, eq144_e1832_d_b51, eq144_e1832_d_b52, eq144_e1832_d_b53, eq144_e1832_d_b54];
        stamper.stamp_current_dense_local(
            Some(16),
            Some(15),
            multiplicity * (eq144_value),
            &eq144_node_derivatives,
            &eq144_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_24(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[252][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[252][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[252][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[252][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[252][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[252][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[252][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[252][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[252][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[252][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[252][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[252][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[252][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[252][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[252][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[252][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[252][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[252][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[252][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[252][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[252][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[252][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[252][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[252][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[252][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[252][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[252][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[252][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[252][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[252][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[252][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[252][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[252][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[252][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[252][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[252][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[252][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[252][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[252][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[252][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[252][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[252][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[252][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[252][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[252][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[252][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[252][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[252][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[252][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[252][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[252][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[252][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[252][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[252][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[252][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[252][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[252][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[252][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[252][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[252][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[252][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[252][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[252][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[252][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[252][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[252][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[252][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[252][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[252][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[252][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[252][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[252][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[252][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[252][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[252][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[252][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[252][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[252][54] * ddt_scale));
        let (eq145_e1843, eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n10, eq145_e1843_d_n11, eq145_e1843_d_n12, eq145_e1843_d_n13, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22, eq145_e1843_d_b0, eq145_e1843_d_b1, eq145_e1843_d_b2, eq145_e1843_d_b3, eq145_e1843_d_b4, eq145_e1843_d_b5, eq145_e1843_d_b6, eq145_e1843_d_b7, eq145_e1843_d_b8, eq145_e1843_d_b9, eq145_e1843_d_b10, eq145_e1843_d_b11, eq145_e1843_d_b12, eq145_e1843_d_b13, eq145_e1843_d_b14, eq145_e1843_d_b15, eq145_e1843_d_b16, eq145_e1843_d_b17, eq145_e1843_d_b18, eq145_e1843_d_b19, eq145_e1843_d_b20, eq145_e1843_d_b21, eq145_e1843_d_b22, eq145_e1843_d_b23, eq145_e1843_d_b24, eq145_e1843_d_b25, eq145_e1843_d_b26, eq145_e1843_d_b27, eq145_e1843_d_b28, eq145_e1843_d_b29, eq145_e1843_d_b30, eq145_e1843_d_b31, eq145_e1843_d_b32, eq145_e1843_d_b33, eq145_e1843_d_b34, eq145_e1843_d_b35, eq145_e1843_d_b36, eq145_e1843_d_b37, eq145_e1843_d_b38, eq145_e1843_d_b39, eq145_e1843_d_b40, eq145_e1843_d_b41, eq145_e1843_d_b42, eq145_e1843_d_b43, eq145_e1843_d_b44, eq145_e1843_d_b45, eq145_e1843_d_b46, eq145_e1843_d_b47, eq145_e1843_d_b48, eq145_e1843_d_b49, eq145_e1843_d_b50, eq145_e1843_d_b51, eq145_e1843_d_b52, eq145_e1843_d_b53, eq145_e1843_d_b54,) = {
    if ((s.b[580] && s.b[581]) && s.b[582]) {
        let eq145_e1840: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 44, s.v[252]);
        let eq145_e1841: f64 = (p.p7 * eq145_e1840);
        (eq145_e1841, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq145_value: f64 = eq145_e1843;
        let eq145_node_derivatives: [f64; 23] = [eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n10, eq145_e1843_d_n11, eq145_e1843_d_n12, eq145_e1843_d_n13, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22];
        let eq145_branch_derivatives: [f64; 55] = [eq145_e1843_d_b0, eq145_e1843_d_b1, eq145_e1843_d_b2, eq145_e1843_d_b3, eq145_e1843_d_b4, eq145_e1843_d_b5, eq145_e1843_d_b6, eq145_e1843_d_b7, eq145_e1843_d_b8, eq145_e1843_d_b9, eq145_e1843_d_b10, eq145_e1843_d_b11, eq145_e1843_d_b12, eq145_e1843_d_b13, eq145_e1843_d_b14, eq145_e1843_d_b15, eq145_e1843_d_b16, eq145_e1843_d_b17, eq145_e1843_d_b18, eq145_e1843_d_b19, eq145_e1843_d_b20, eq145_e1843_d_b21, eq145_e1843_d_b22, eq145_e1843_d_b23, eq145_e1843_d_b24, eq145_e1843_d_b25, eq145_e1843_d_b26, eq145_e1843_d_b27, eq145_e1843_d_b28, eq145_e1843_d_b29, eq145_e1843_d_b30, eq145_e1843_d_b31, eq145_e1843_d_b32, eq145_e1843_d_b33, eq145_e1843_d_b34, eq145_e1843_d_b35, eq145_e1843_d_b36, eq145_e1843_d_b37, eq145_e1843_d_b38, eq145_e1843_d_b39, eq145_e1843_d_b40, eq145_e1843_d_b41, eq145_e1843_d_b42, eq145_e1843_d_b43, eq145_e1843_d_b44, eq145_e1843_d_b45, eq145_e1843_d_b46, eq145_e1843_d_b47, eq145_e1843_d_b48, eq145_e1843_d_b49, eq145_e1843_d_b50, eq145_e1843_d_b51, eq145_e1843_d_b52, eq145_e1843_d_b53, eq145_e1843_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(15),
            multiplicity * (eq145_value),
            &eq145_node_derivatives,
            &eq145_branch_derivatives,
            multiplicity,
        );
        let (eq146_e1856, eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n10, eq146_e1856_d_n11, eq146_e1856_d_n12, eq146_e1856_d_n13, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22, eq146_e1856_d_b0, eq146_e1856_d_b1, eq146_e1856_d_b2, eq146_e1856_d_b3, eq146_e1856_d_b4, eq146_e1856_d_b5, eq146_e1856_d_b6, eq146_e1856_d_b7, eq146_e1856_d_b8, eq146_e1856_d_b9, eq146_e1856_d_b10, eq146_e1856_d_b11, eq146_e1856_d_b12, eq146_e1856_d_b13, eq146_e1856_d_b14, eq146_e1856_d_b15, eq146_e1856_d_b16, eq146_e1856_d_b17, eq146_e1856_d_b18, eq146_e1856_d_b19, eq146_e1856_d_b20, eq146_e1856_d_b21, eq146_e1856_d_b22, eq146_e1856_d_b23, eq146_e1856_d_b24, eq146_e1856_d_b25, eq146_e1856_d_b26, eq146_e1856_d_b27, eq146_e1856_d_b28, eq146_e1856_d_b29, eq146_e1856_d_b30, eq146_e1856_d_b31, eq146_e1856_d_b32, eq146_e1856_d_b33, eq146_e1856_d_b34, eq146_e1856_d_b35, eq146_e1856_d_b36, eq146_e1856_d_b37, eq146_e1856_d_b38, eq146_e1856_d_b39, eq146_e1856_d_b40, eq146_e1856_d_b41, eq146_e1856_d_b42, eq146_e1856_d_b43, eq146_e1856_d_b44, eq146_e1856_d_b45, eq146_e1856_d_b46, eq146_e1856_d_b47, eq146_e1856_d_b48, eq146_e1856_d_b49, eq146_e1856_d_b50, eq146_e1856_d_b51, eq146_e1856_d_b52, eq146_e1856_d_b53, eq146_e1856_d_b54,) = {
    if ((s.b[580] && s.b[581]) && s.b[582]) {
        let eq146_e1851: f64 = (p.p7 * p.p247);
        let eq146_e1853: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 45, s.v[252]);
        let eq146_e1854: f64 = (eq146_e1851 * eq146_e1853);
        let eq146_e1854_d_n0: f64 = (eq146_e1851 * (s.dn[252][0] * ddt_scale));
        let eq146_e1854_d_n1: f64 = (eq146_e1851 * (s.dn[252][1] * ddt_scale));
        let eq146_e1854_d_n2: f64 = (eq146_e1851 * (s.dn[252][2] * ddt_scale));
        let eq146_e1854_d_n3: f64 = (eq146_e1851 * (s.dn[252][3] * ddt_scale));
        let eq146_e1854_d_n4: f64 = (eq146_e1851 * (s.dn[252][4] * ddt_scale));
        let eq146_e1854_d_n5: f64 = (eq146_e1851 * (s.dn[252][5] * ddt_scale));
        let eq146_e1854_d_n6: f64 = (eq146_e1851 * (s.dn[252][6] * ddt_scale));
        let eq146_e1854_d_n7: f64 = (eq146_e1851 * (s.dn[252][7] * ddt_scale));
        let eq146_e1854_d_n8: f64 = (eq146_e1851 * (s.dn[252][8] * ddt_scale));
        let eq146_e1854_d_n9: f64 = (eq146_e1851 * (s.dn[252][9] * ddt_scale));
        let eq146_e1854_d_n10: f64 = (eq146_e1851 * (s.dn[252][10] * ddt_scale));
        let eq146_e1854_d_n11: f64 = (eq146_e1851 * (s.dn[252][11] * ddt_scale));
        let eq146_e1854_d_n12: f64 = (eq146_e1851 * (s.dn[252][12] * ddt_scale));
        let eq146_e1854_d_n13: f64 = (eq146_e1851 * (s.dn[252][13] * ddt_scale));
        let eq146_e1854_d_n14: f64 = (eq146_e1851 * (s.dn[252][14] * ddt_scale));
        let eq146_e1854_d_n15: f64 = (eq146_e1851 * (s.dn[252][15] * ddt_scale));
        let eq146_e1854_d_n16: f64 = (eq146_e1851 * (s.dn[252][16] * ddt_scale));
        let eq146_e1854_d_n17: f64 = (eq146_e1851 * (s.dn[252][17] * ddt_scale));
        let eq146_e1854_d_n18: f64 = (eq146_e1851 * (s.dn[252][18] * ddt_scale));
        let eq146_e1854_d_n19: f64 = (eq146_e1851 * (s.dn[252][19] * ddt_scale));
        let eq146_e1854_d_n20: f64 = (eq146_e1851 * (s.dn[252][20] * ddt_scale));
        let eq146_e1854_d_n21: f64 = (eq146_e1851 * (s.dn[252][21] * ddt_scale));
        let eq146_e1854_d_n22: f64 = (eq146_e1851 * (s.dn[252][22] * ddt_scale));
        let eq146_e1854_d_b0: f64 = (eq146_e1851 * (s.db[252][0] * ddt_scale));
        let eq146_e1854_d_b1: f64 = (eq146_e1851 * (s.db[252][1] * ddt_scale));
        let eq146_e1854_d_b2: f64 = (eq146_e1851 * (s.db[252][2] * ddt_scale));
        let eq146_e1854_d_b3: f64 = (eq146_e1851 * (s.db[252][3] * ddt_scale));
        let eq146_e1854_d_b4: f64 = (eq146_e1851 * (s.db[252][4] * ddt_scale));
        let eq146_e1854_d_b5: f64 = (eq146_e1851 * (s.db[252][5] * ddt_scale));
        let eq146_e1854_d_b6: f64 = (eq146_e1851 * (s.db[252][6] * ddt_scale));
        let eq146_e1854_d_b7: f64 = (eq146_e1851 * (s.db[252][7] * ddt_scale));
        let eq146_e1854_d_b8: f64 = (eq146_e1851 * (s.db[252][8] * ddt_scale));
        let eq146_e1854_d_b9: f64 = (eq146_e1851 * (s.db[252][9] * ddt_scale));
        let eq146_e1854_d_b10: f64 = (eq146_e1851 * (s.db[252][10] * ddt_scale));
        let eq146_e1854_d_b11: f64 = (eq146_e1851 * (s.db[252][11] * ddt_scale));
        let eq146_e1854_d_b12: f64 = (eq146_e1851 * (s.db[252][12] * ddt_scale));
        let eq146_e1854_d_b13: f64 = (eq146_e1851 * (s.db[252][13] * ddt_scale));
        let eq146_e1854_d_b14: f64 = (eq146_e1851 * (s.db[252][14] * ddt_scale));
        let eq146_e1854_d_b15: f64 = (eq146_e1851 * (s.db[252][15] * ddt_scale));
        let eq146_e1854_d_b16: f64 = (eq146_e1851 * (s.db[252][16] * ddt_scale));
        let eq146_e1854_d_b17: f64 = (eq146_e1851 * (s.db[252][17] * ddt_scale));
        let eq146_e1854_d_b18: f64 = (eq146_e1851 * (s.db[252][18] * ddt_scale));
        let eq146_e1854_d_b19: f64 = (eq146_e1851 * (s.db[252][19] * ddt_scale));
        let eq146_e1854_d_b20: f64 = (eq146_e1851 * (s.db[252][20] * ddt_scale));
        let eq146_e1854_d_b21: f64 = (eq146_e1851 * (s.db[252][21] * ddt_scale));
        let eq146_e1854_d_b22: f64 = (eq146_e1851 * (s.db[252][22] * ddt_scale));
        let eq146_e1854_d_b23: f64 = (eq146_e1851 * (s.db[252][23] * ddt_scale));
        let eq146_e1854_d_b24: f64 = (eq146_e1851 * (s.db[252][24] * ddt_scale));
        let eq146_e1854_d_b25: f64 = (eq146_e1851 * (s.db[252][25] * ddt_scale));
        let eq146_e1854_d_b26: f64 = (eq146_e1851 * (s.db[252][26] * ddt_scale));
        let eq146_e1854_d_b27: f64 = (eq146_e1851 * (s.db[252][27] * ddt_scale));
        let eq146_e1854_d_b28: f64 = (eq146_e1851 * (s.db[252][28] * ddt_scale));
        let eq146_e1854_d_b29: f64 = (eq146_e1851 * (s.db[252][29] * ddt_scale));
        let eq146_e1854_d_b30: f64 = (eq146_e1851 * (s.db[252][30] * ddt_scale));
        let eq146_e1854_d_b31: f64 = (eq146_e1851 * (s.db[252][31] * ddt_scale));
        let eq146_e1854_d_b32: f64 = (eq146_e1851 * (s.db[252][32] * ddt_scale));
        let eq146_e1854_d_b33: f64 = (eq146_e1851 * (s.db[252][33] * ddt_scale));
        let eq146_e1854_d_b34: f64 = (eq146_e1851 * (s.db[252][34] * ddt_scale));
        let eq146_e1854_d_b35: f64 = (eq146_e1851 * (s.db[252][35] * ddt_scale));
        let eq146_e1854_d_b36: f64 = (eq146_e1851 * (s.db[252][36] * ddt_scale));
        let eq146_e1854_d_b37: f64 = (eq146_e1851 * (s.db[252][37] * ddt_scale));
        let eq146_e1854_d_b38: f64 = (eq146_e1851 * (s.db[252][38] * ddt_scale));
        let eq146_e1854_d_b39: f64 = (eq146_e1851 * (s.db[252][39] * ddt_scale));
        let eq146_e1854_d_b40: f64 = (eq146_e1851 * (s.db[252][40] * ddt_scale));
        let eq146_e1854_d_b41: f64 = (eq146_e1851 * (s.db[252][41] * ddt_scale));
        let eq146_e1854_d_b42: f64 = (eq146_e1851 * (s.db[252][42] * ddt_scale));
        let eq146_e1854_d_b43: f64 = (eq146_e1851 * (s.db[252][43] * ddt_scale));
        let eq146_e1854_d_b44: f64 = (eq146_e1851 * (s.db[252][44] * ddt_scale));
        let eq146_e1854_d_b45: f64 = (eq146_e1851 * (s.db[252][45] * ddt_scale));
        let eq146_e1854_d_b46: f64 = (eq146_e1851 * (s.db[252][46] * ddt_scale));
        let eq146_e1854_d_b47: f64 = (eq146_e1851 * (s.db[252][47] * ddt_scale));
        let eq146_e1854_d_b48: f64 = (eq146_e1851 * (s.db[252][48] * ddt_scale));
        let eq146_e1854_d_b49: f64 = (eq146_e1851 * (s.db[252][49] * ddt_scale));
        let eq146_e1854_d_b50: f64 = (eq146_e1851 * (s.db[252][50] * ddt_scale));
        let eq146_e1854_d_b51: f64 = (eq146_e1851 * (s.db[252][51] * ddt_scale));
        let eq146_e1854_d_b52: f64 = (eq146_e1851 * (s.db[252][52] * ddt_scale));
        let eq146_e1854_d_b53: f64 = (eq146_e1851 * (s.db[252][53] * ddt_scale));
        let eq146_e1854_d_b54: f64 = (eq146_e1851 * (s.db[252][54] * ddt_scale));
        (eq146_e1854, eq146_e1854_d_n0, eq146_e1854_d_n1, eq146_e1854_d_n2, eq146_e1854_d_n3, eq146_e1854_d_n4, eq146_e1854_d_n5, eq146_e1854_d_n6, eq146_e1854_d_n7, eq146_e1854_d_n8, eq146_e1854_d_n9, eq146_e1854_d_n10, eq146_e1854_d_n11, eq146_e1854_d_n12, eq146_e1854_d_n13, eq146_e1854_d_n14, eq146_e1854_d_n15, eq146_e1854_d_n16, eq146_e1854_d_n17, eq146_e1854_d_n18, eq146_e1854_d_n19, eq146_e1854_d_n20, eq146_e1854_d_n21, eq146_e1854_d_n22, eq146_e1854_d_b0, eq146_e1854_d_b1, eq146_e1854_d_b2, eq146_e1854_d_b3, eq146_e1854_d_b4, eq146_e1854_d_b5, eq146_e1854_d_b6, eq146_e1854_d_b7, eq146_e1854_d_b8, eq146_e1854_d_b9, eq146_e1854_d_b10, eq146_e1854_d_b11, eq146_e1854_d_b12, eq146_e1854_d_b13, eq146_e1854_d_b14, eq146_e1854_d_b15, eq146_e1854_d_b16, eq146_e1854_d_b17, eq146_e1854_d_b18, eq146_e1854_d_b19, eq146_e1854_d_b20, eq146_e1854_d_b21, eq146_e1854_d_b22, eq146_e1854_d_b23, eq146_e1854_d_b24, eq146_e1854_d_b25, eq146_e1854_d_b26, eq146_e1854_d_b27, eq146_e1854_d_b28, eq146_e1854_d_b29, eq146_e1854_d_b30, eq146_e1854_d_b31, eq146_e1854_d_b32, eq146_e1854_d_b33, eq146_e1854_d_b34, eq146_e1854_d_b35, eq146_e1854_d_b36, eq146_e1854_d_b37, eq146_e1854_d_b38, eq146_e1854_d_b39, eq146_e1854_d_b40, eq146_e1854_d_b41, eq146_e1854_d_b42, eq146_e1854_d_b43, eq146_e1854_d_b44, eq146_e1854_d_b45, eq146_e1854_d_b46, eq146_e1854_d_b47, eq146_e1854_d_b48, eq146_e1854_d_b49, eq146_e1854_d_b50, eq146_e1854_d_b51, eq146_e1854_d_b52, eq146_e1854_d_b53, eq146_e1854_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq146_value: f64 = eq146_e1856;
        let eq146_node_derivatives: [f64; 23] = [eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n10, eq146_e1856_d_n11, eq146_e1856_d_n12, eq146_e1856_d_n13, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22];
        let eq146_branch_derivatives: [f64; 55] = [eq146_e1856_d_b0, eq146_e1856_d_b1, eq146_e1856_d_b2, eq146_e1856_d_b3, eq146_e1856_d_b4, eq146_e1856_d_b5, eq146_e1856_d_b6, eq146_e1856_d_b7, eq146_e1856_d_b8, eq146_e1856_d_b9, eq146_e1856_d_b10, eq146_e1856_d_b11, eq146_e1856_d_b12, eq146_e1856_d_b13, eq146_e1856_d_b14, eq146_e1856_d_b15, eq146_e1856_d_b16, eq146_e1856_d_b17, eq146_e1856_d_b18, eq146_e1856_d_b19, eq146_e1856_d_b20, eq146_e1856_d_b21, eq146_e1856_d_b22, eq146_e1856_d_b23, eq146_e1856_d_b24, eq146_e1856_d_b25, eq146_e1856_d_b26, eq146_e1856_d_b27, eq146_e1856_d_b28, eq146_e1856_d_b29, eq146_e1856_d_b30, eq146_e1856_d_b31, eq146_e1856_d_b32, eq146_e1856_d_b33, eq146_e1856_d_b34, eq146_e1856_d_b35, eq146_e1856_d_b36, eq146_e1856_d_b37, eq146_e1856_d_b38, eq146_e1856_d_b39, eq146_e1856_d_b40, eq146_e1856_d_b41, eq146_e1856_d_b42, eq146_e1856_d_b43, eq146_e1856_d_b44, eq146_e1856_d_b45, eq146_e1856_d_b46, eq146_e1856_d_b47, eq146_e1856_d_b48, eq146_e1856_d_b49, eq146_e1856_d_b50, eq146_e1856_d_b51, eq146_e1856_d_b52, eq146_e1856_d_b53, eq146_e1856_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(15),
            multiplicity * (eq146_value),
            &eq146_node_derivatives,
            &eq146_branch_derivatives,
            multiplicity,
        );
        let (eq147_e1868, eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n10, eq147_e1868_d_n11, eq147_e1868_d_n12, eq147_e1868_d_n13, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22, eq147_e1868_d_b0, eq147_e1868_d_b1, eq147_e1868_d_b2, eq147_e1868_d_b3, eq147_e1868_d_b4, eq147_e1868_d_b5, eq147_e1868_d_b6, eq147_e1868_d_b7, eq147_e1868_d_b8, eq147_e1868_d_b9, eq147_e1868_d_b10, eq147_e1868_d_b11, eq147_e1868_d_b12, eq147_e1868_d_b13, eq147_e1868_d_b14, eq147_e1868_d_b15, eq147_e1868_d_b16, eq147_e1868_d_b17, eq147_e1868_d_b18, eq147_e1868_d_b19, eq147_e1868_d_b20, eq147_e1868_d_b21, eq147_e1868_d_b22, eq147_e1868_d_b23, eq147_e1868_d_b24, eq147_e1868_d_b25, eq147_e1868_d_b26, eq147_e1868_d_b27, eq147_e1868_d_b28, eq147_e1868_d_b29, eq147_e1868_d_b30, eq147_e1868_d_b31, eq147_e1868_d_b32, eq147_e1868_d_b33, eq147_e1868_d_b34, eq147_e1868_d_b35, eq147_e1868_d_b36, eq147_e1868_d_b37, eq147_e1868_d_b38, eq147_e1868_d_b39, eq147_e1868_d_b40, eq147_e1868_d_b41, eq147_e1868_d_b42, eq147_e1868_d_b43, eq147_e1868_d_b44, eq147_e1868_d_b45, eq147_e1868_d_b46, eq147_e1868_d_b47, eq147_e1868_d_b48, eq147_e1868_d_b49, eq147_e1868_d_b50, eq147_e1868_d_b51, eq147_e1868_d_b52, eq147_e1868_d_b53, eq147_e1868_d_b54,) = {
    if ((s.b[580] && s.b[581]) && (!s.b[582])) {
        let eq147_e1865: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 46, s.v[252]);
        let eq147_e1866: f64 = (p.p7 * eq147_e1865);
        (eq147_e1866, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_value: f64 = eq147_e1868;
        let eq147_node_derivatives: [f64; 23] = [eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n10, eq147_e1868_d_n11, eq147_e1868_d_n12, eq147_e1868_d_n13, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22];
        let eq147_branch_derivatives: [f64; 55] = [eq147_e1868_d_b0, eq147_e1868_d_b1, eq147_e1868_d_b2, eq147_e1868_d_b3, eq147_e1868_d_b4, eq147_e1868_d_b5, eq147_e1868_d_b6, eq147_e1868_d_b7, eq147_e1868_d_b8, eq147_e1868_d_b9, eq147_e1868_d_b10, eq147_e1868_d_b11, eq147_e1868_d_b12, eq147_e1868_d_b13, eq147_e1868_d_b14, eq147_e1868_d_b15, eq147_e1868_d_b16, eq147_e1868_d_b17, eq147_e1868_d_b18, eq147_e1868_d_b19, eq147_e1868_d_b20, eq147_e1868_d_b21, eq147_e1868_d_b22, eq147_e1868_d_b23, eq147_e1868_d_b24, eq147_e1868_d_b25, eq147_e1868_d_b26, eq147_e1868_d_b27, eq147_e1868_d_b28, eq147_e1868_d_b29, eq147_e1868_d_b30, eq147_e1868_d_b31, eq147_e1868_d_b32, eq147_e1868_d_b33, eq147_e1868_d_b34, eq147_e1868_d_b35, eq147_e1868_d_b36, eq147_e1868_d_b37, eq147_e1868_d_b38, eq147_e1868_d_b39, eq147_e1868_d_b40, eq147_e1868_d_b41, eq147_e1868_d_b42, eq147_e1868_d_b43, eq147_e1868_d_b44, eq147_e1868_d_b45, eq147_e1868_d_b46, eq147_e1868_d_b47, eq147_e1868_d_b48, eq147_e1868_d_b49, eq147_e1868_d_b50, eq147_e1868_d_b51, eq147_e1868_d_b52, eq147_e1868_d_b53, eq147_e1868_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(15),
            multiplicity * (eq147_value),
            &eq147_node_derivatives,
            &eq147_branch_derivatives,
            multiplicity,
        );
        let (eq148_e1882, eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n10, eq148_e1882_d_n11, eq148_e1882_d_n12, eq148_e1882_d_n13, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22, eq148_e1882_d_b0, eq148_e1882_d_b1, eq148_e1882_d_b2, eq148_e1882_d_b3, eq148_e1882_d_b4, eq148_e1882_d_b5, eq148_e1882_d_b6, eq148_e1882_d_b7, eq148_e1882_d_b8, eq148_e1882_d_b9, eq148_e1882_d_b10, eq148_e1882_d_b11, eq148_e1882_d_b12, eq148_e1882_d_b13, eq148_e1882_d_b14, eq148_e1882_d_b15, eq148_e1882_d_b16, eq148_e1882_d_b17, eq148_e1882_d_b18, eq148_e1882_d_b19, eq148_e1882_d_b20, eq148_e1882_d_b21, eq148_e1882_d_b22, eq148_e1882_d_b23, eq148_e1882_d_b24, eq148_e1882_d_b25, eq148_e1882_d_b26, eq148_e1882_d_b27, eq148_e1882_d_b28, eq148_e1882_d_b29, eq148_e1882_d_b30, eq148_e1882_d_b31, eq148_e1882_d_b32, eq148_e1882_d_b33, eq148_e1882_d_b34, eq148_e1882_d_b35, eq148_e1882_d_b36, eq148_e1882_d_b37, eq148_e1882_d_b38, eq148_e1882_d_b39, eq148_e1882_d_b40, eq148_e1882_d_b41, eq148_e1882_d_b42, eq148_e1882_d_b43, eq148_e1882_d_b44, eq148_e1882_d_b45, eq148_e1882_d_b46, eq148_e1882_d_b47, eq148_e1882_d_b48, eq148_e1882_d_b49, eq148_e1882_d_b50, eq148_e1882_d_b51, eq148_e1882_d_b52, eq148_e1882_d_b53, eq148_e1882_d_b54,) = {
    if ((s.b[580] && s.b[581]) && (!s.b[582])) {
        let eq148_e1877: f64 = (p.p7 * p.p247);
        let eq148_e1879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 47, s.v[252]);
        let eq148_e1880: f64 = (eq148_e1877 * eq148_e1879);
        let eq148_e1880_d_n0: f64 = (eq148_e1877 * (s.dn[252][0] * ddt_scale));
        let eq148_e1880_d_n1: f64 = (eq148_e1877 * (s.dn[252][1] * ddt_scale));
        let eq148_e1880_d_n2: f64 = (eq148_e1877 * (s.dn[252][2] * ddt_scale));
        let eq148_e1880_d_n3: f64 = (eq148_e1877 * (s.dn[252][3] * ddt_scale));
        let eq148_e1880_d_n4: f64 = (eq148_e1877 * (s.dn[252][4] * ddt_scale));
        let eq148_e1880_d_n5: f64 = (eq148_e1877 * (s.dn[252][5] * ddt_scale));
        let eq148_e1880_d_n6: f64 = (eq148_e1877 * (s.dn[252][6] * ddt_scale));
        let eq148_e1880_d_n7: f64 = (eq148_e1877 * (s.dn[252][7] * ddt_scale));
        let eq148_e1880_d_n8: f64 = (eq148_e1877 * (s.dn[252][8] * ddt_scale));
        let eq148_e1880_d_n9: f64 = (eq148_e1877 * (s.dn[252][9] * ddt_scale));
        let eq148_e1880_d_n10: f64 = (eq148_e1877 * (s.dn[252][10] * ddt_scale));
        let eq148_e1880_d_n11: f64 = (eq148_e1877 * (s.dn[252][11] * ddt_scale));
        let eq148_e1880_d_n12: f64 = (eq148_e1877 * (s.dn[252][12] * ddt_scale));
        let eq148_e1880_d_n13: f64 = (eq148_e1877 * (s.dn[252][13] * ddt_scale));
        let eq148_e1880_d_n14: f64 = (eq148_e1877 * (s.dn[252][14] * ddt_scale));
        let eq148_e1880_d_n15: f64 = (eq148_e1877 * (s.dn[252][15] * ddt_scale));
        let eq148_e1880_d_n16: f64 = (eq148_e1877 * (s.dn[252][16] * ddt_scale));
        let eq148_e1880_d_n17: f64 = (eq148_e1877 * (s.dn[252][17] * ddt_scale));
        let eq148_e1880_d_n18: f64 = (eq148_e1877 * (s.dn[252][18] * ddt_scale));
        let eq148_e1880_d_n19: f64 = (eq148_e1877 * (s.dn[252][19] * ddt_scale));
        let eq148_e1880_d_n20: f64 = (eq148_e1877 * (s.dn[252][20] * ddt_scale));
        let eq148_e1880_d_n21: f64 = (eq148_e1877 * (s.dn[252][21] * ddt_scale));
        let eq148_e1880_d_n22: f64 = (eq148_e1877 * (s.dn[252][22] * ddt_scale));
        let eq148_e1880_d_b0: f64 = (eq148_e1877 * (s.db[252][0] * ddt_scale));
        let eq148_e1880_d_b1: f64 = (eq148_e1877 * (s.db[252][1] * ddt_scale));
        let eq148_e1880_d_b2: f64 = (eq148_e1877 * (s.db[252][2] * ddt_scale));
        let eq148_e1880_d_b3: f64 = (eq148_e1877 * (s.db[252][3] * ddt_scale));
        let eq148_e1880_d_b4: f64 = (eq148_e1877 * (s.db[252][4] * ddt_scale));
        let eq148_e1880_d_b5: f64 = (eq148_e1877 * (s.db[252][5] * ddt_scale));
        let eq148_e1880_d_b6: f64 = (eq148_e1877 * (s.db[252][6] * ddt_scale));
        let eq148_e1880_d_b7: f64 = (eq148_e1877 * (s.db[252][7] * ddt_scale));
        let eq148_e1880_d_b8: f64 = (eq148_e1877 * (s.db[252][8] * ddt_scale));
        let eq148_e1880_d_b9: f64 = (eq148_e1877 * (s.db[252][9] * ddt_scale));
        let eq148_e1880_d_b10: f64 = (eq148_e1877 * (s.db[252][10] * ddt_scale));
        let eq148_e1880_d_b11: f64 = (eq148_e1877 * (s.db[252][11] * ddt_scale));
        let eq148_e1880_d_b12: f64 = (eq148_e1877 * (s.db[252][12] * ddt_scale));
        let eq148_e1880_d_b13: f64 = (eq148_e1877 * (s.db[252][13] * ddt_scale));
        let eq148_e1880_d_b14: f64 = (eq148_e1877 * (s.db[252][14] * ddt_scale));
        let eq148_e1880_d_b15: f64 = (eq148_e1877 * (s.db[252][15] * ddt_scale));
        let eq148_e1880_d_b16: f64 = (eq148_e1877 * (s.db[252][16] * ddt_scale));
        let eq148_e1880_d_b17: f64 = (eq148_e1877 * (s.db[252][17] * ddt_scale));
        let eq148_e1880_d_b18: f64 = (eq148_e1877 * (s.db[252][18] * ddt_scale));
        let eq148_e1880_d_b19: f64 = (eq148_e1877 * (s.db[252][19] * ddt_scale));
        let eq148_e1880_d_b20: f64 = (eq148_e1877 * (s.db[252][20] * ddt_scale));
        let eq148_e1880_d_b21: f64 = (eq148_e1877 * (s.db[252][21] * ddt_scale));
        let eq148_e1880_d_b22: f64 = (eq148_e1877 * (s.db[252][22] * ddt_scale));
        let eq148_e1880_d_b23: f64 = (eq148_e1877 * (s.db[252][23] * ddt_scale));
        let eq148_e1880_d_b24: f64 = (eq148_e1877 * (s.db[252][24] * ddt_scale));
        let eq148_e1880_d_b25: f64 = (eq148_e1877 * (s.db[252][25] * ddt_scale));
        let eq148_e1880_d_b26: f64 = (eq148_e1877 * (s.db[252][26] * ddt_scale));
        let eq148_e1880_d_b27: f64 = (eq148_e1877 * (s.db[252][27] * ddt_scale));
        let eq148_e1880_d_b28: f64 = (eq148_e1877 * (s.db[252][28] * ddt_scale));
        let eq148_e1880_d_b29: f64 = (eq148_e1877 * (s.db[252][29] * ddt_scale));
        let eq148_e1880_d_b30: f64 = (eq148_e1877 * (s.db[252][30] * ddt_scale));
        let eq148_e1880_d_b31: f64 = (eq148_e1877 * (s.db[252][31] * ddt_scale));
        let eq148_e1880_d_b32: f64 = (eq148_e1877 * (s.db[252][32] * ddt_scale));
        let eq148_e1880_d_b33: f64 = (eq148_e1877 * (s.db[252][33] * ddt_scale));
        let eq148_e1880_d_b34: f64 = (eq148_e1877 * (s.db[252][34] * ddt_scale));
        let eq148_e1880_d_b35: f64 = (eq148_e1877 * (s.db[252][35] * ddt_scale));
        let eq148_e1880_d_b36: f64 = (eq148_e1877 * (s.db[252][36] * ddt_scale));
        let eq148_e1880_d_b37: f64 = (eq148_e1877 * (s.db[252][37] * ddt_scale));
        let eq148_e1880_d_b38: f64 = (eq148_e1877 * (s.db[252][38] * ddt_scale));
        let eq148_e1880_d_b39: f64 = (eq148_e1877 * (s.db[252][39] * ddt_scale));
        let eq148_e1880_d_b40: f64 = (eq148_e1877 * (s.db[252][40] * ddt_scale));
        let eq148_e1880_d_b41: f64 = (eq148_e1877 * (s.db[252][41] * ddt_scale));
        let eq148_e1880_d_b42: f64 = (eq148_e1877 * (s.db[252][42] * ddt_scale));
        let eq148_e1880_d_b43: f64 = (eq148_e1877 * (s.db[252][43] * ddt_scale));
        let eq148_e1880_d_b44: f64 = (eq148_e1877 * (s.db[252][44] * ddt_scale));
        let eq148_e1880_d_b45: f64 = (eq148_e1877 * (s.db[252][45] * ddt_scale));
        let eq148_e1880_d_b46: f64 = (eq148_e1877 * (s.db[252][46] * ddt_scale));
        let eq148_e1880_d_b47: f64 = (eq148_e1877 * (s.db[252][47] * ddt_scale));
        let eq148_e1880_d_b48: f64 = (eq148_e1877 * (s.db[252][48] * ddt_scale));
        let eq148_e1880_d_b49: f64 = (eq148_e1877 * (s.db[252][49] * ddt_scale));
        let eq148_e1880_d_b50: f64 = (eq148_e1877 * (s.db[252][50] * ddt_scale));
        let eq148_e1880_d_b51: f64 = (eq148_e1877 * (s.db[252][51] * ddt_scale));
        let eq148_e1880_d_b52: f64 = (eq148_e1877 * (s.db[252][52] * ddt_scale));
        let eq148_e1880_d_b53: f64 = (eq148_e1877 * (s.db[252][53] * ddt_scale));
        let eq148_e1880_d_b54: f64 = (eq148_e1877 * (s.db[252][54] * ddt_scale));
        (eq148_e1880, eq148_e1880_d_n0, eq148_e1880_d_n1, eq148_e1880_d_n2, eq148_e1880_d_n3, eq148_e1880_d_n4, eq148_e1880_d_n5, eq148_e1880_d_n6, eq148_e1880_d_n7, eq148_e1880_d_n8, eq148_e1880_d_n9, eq148_e1880_d_n10, eq148_e1880_d_n11, eq148_e1880_d_n12, eq148_e1880_d_n13, eq148_e1880_d_n14, eq148_e1880_d_n15, eq148_e1880_d_n16, eq148_e1880_d_n17, eq148_e1880_d_n18, eq148_e1880_d_n19, eq148_e1880_d_n20, eq148_e1880_d_n21, eq148_e1880_d_n22, eq148_e1880_d_b0, eq148_e1880_d_b1, eq148_e1880_d_b2, eq148_e1880_d_b3, eq148_e1880_d_b4, eq148_e1880_d_b5, eq148_e1880_d_b6, eq148_e1880_d_b7, eq148_e1880_d_b8, eq148_e1880_d_b9, eq148_e1880_d_b10, eq148_e1880_d_b11, eq148_e1880_d_b12, eq148_e1880_d_b13, eq148_e1880_d_b14, eq148_e1880_d_b15, eq148_e1880_d_b16, eq148_e1880_d_b17, eq148_e1880_d_b18, eq148_e1880_d_b19, eq148_e1880_d_b20, eq148_e1880_d_b21, eq148_e1880_d_b22, eq148_e1880_d_b23, eq148_e1880_d_b24, eq148_e1880_d_b25, eq148_e1880_d_b26, eq148_e1880_d_b27, eq148_e1880_d_b28, eq148_e1880_d_b29, eq148_e1880_d_b30, eq148_e1880_d_b31, eq148_e1880_d_b32, eq148_e1880_d_b33, eq148_e1880_d_b34, eq148_e1880_d_b35, eq148_e1880_d_b36, eq148_e1880_d_b37, eq148_e1880_d_b38, eq148_e1880_d_b39, eq148_e1880_d_b40, eq148_e1880_d_b41, eq148_e1880_d_b42, eq148_e1880_d_b43, eq148_e1880_d_b44, eq148_e1880_d_b45, eq148_e1880_d_b46, eq148_e1880_d_b47, eq148_e1880_d_b48, eq148_e1880_d_b49, eq148_e1880_d_b50, eq148_e1880_d_b51, eq148_e1880_d_b52, eq148_e1880_d_b53, eq148_e1880_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_value: f64 = eq148_e1882;
        let eq148_node_derivatives: [f64; 23] = [eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n10, eq148_e1882_d_n11, eq148_e1882_d_n12, eq148_e1882_d_n13, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22];
        let eq148_branch_derivatives: [f64; 55] = [eq148_e1882_d_b0, eq148_e1882_d_b1, eq148_e1882_d_b2, eq148_e1882_d_b3, eq148_e1882_d_b4, eq148_e1882_d_b5, eq148_e1882_d_b6, eq148_e1882_d_b7, eq148_e1882_d_b8, eq148_e1882_d_b9, eq148_e1882_d_b10, eq148_e1882_d_b11, eq148_e1882_d_b12, eq148_e1882_d_b13, eq148_e1882_d_b14, eq148_e1882_d_b15, eq148_e1882_d_b16, eq148_e1882_d_b17, eq148_e1882_d_b18, eq148_e1882_d_b19, eq148_e1882_d_b20, eq148_e1882_d_b21, eq148_e1882_d_b22, eq148_e1882_d_b23, eq148_e1882_d_b24, eq148_e1882_d_b25, eq148_e1882_d_b26, eq148_e1882_d_b27, eq148_e1882_d_b28, eq148_e1882_d_b29, eq148_e1882_d_b30, eq148_e1882_d_b31, eq148_e1882_d_b32, eq148_e1882_d_b33, eq148_e1882_d_b34, eq148_e1882_d_b35, eq148_e1882_d_b36, eq148_e1882_d_b37, eq148_e1882_d_b38, eq148_e1882_d_b39, eq148_e1882_d_b40, eq148_e1882_d_b41, eq148_e1882_d_b42, eq148_e1882_d_b43, eq148_e1882_d_b44, eq148_e1882_d_b45, eq148_e1882_d_b46, eq148_e1882_d_b47, eq148_e1882_d_b48, eq148_e1882_d_b49, eq148_e1882_d_b50, eq148_e1882_d_b51, eq148_e1882_d_b52, eq148_e1882_d_b53, eq148_e1882_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(15),
            multiplicity * (eq148_value),
            &eq148_node_derivatives,
            &eq148_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_25(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq149_e1893, eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n10, eq149_e1893_d_n11, eq149_e1893_d_n12, eq149_e1893_d_n13, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22, eq149_e1893_d_b0, eq149_e1893_d_b1, eq149_e1893_d_b2, eq149_e1893_d_b3, eq149_e1893_d_b4, eq149_e1893_d_b5, eq149_e1893_d_b6, eq149_e1893_d_b7, eq149_e1893_d_b8, eq149_e1893_d_b9, eq149_e1893_d_b10, eq149_e1893_d_b11, eq149_e1893_d_b12, eq149_e1893_d_b13, eq149_e1893_d_b14, eq149_e1893_d_b15, eq149_e1893_d_b16, eq149_e1893_d_b17, eq149_e1893_d_b18, eq149_e1893_d_b19, eq149_e1893_d_b20, eq149_e1893_d_b21, eq149_e1893_d_b22, eq149_e1893_d_b23, eq149_e1893_d_b24, eq149_e1893_d_b25, eq149_e1893_d_b26, eq149_e1893_d_b27, eq149_e1893_d_b28, eq149_e1893_d_b29, eq149_e1893_d_b30, eq149_e1893_d_b31, eq149_e1893_d_b32, eq149_e1893_d_b33, eq149_e1893_d_b34, eq149_e1893_d_b35, eq149_e1893_d_b36, eq149_e1893_d_b37, eq149_e1893_d_b38, eq149_e1893_d_b39, eq149_e1893_d_b40, eq149_e1893_d_b41, eq149_e1893_d_b42, eq149_e1893_d_b43, eq149_e1893_d_b44, eq149_e1893_d_b45, eq149_e1893_d_b46, eq149_e1893_d_b47, eq149_e1893_d_b48, eq149_e1893_d_b49, eq149_e1893_d_b50, eq149_e1893_d_b51, eq149_e1893_d_b52, eq149_e1893_d_b53, eq149_e1893_d_b54,) = {
    if (s.b[580] && s.b[581]) {
        let eq149_e1889: f64 = (p.p252 * s.v[252]);
        let eq149_e1890: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 48, eq149_e1889);
        let eq149_e1890_d_n0: f64 = ((p.p252 * s.dn[252][0]) * ddt_scale);
        let eq149_e1890_d_n1: f64 = ((p.p252 * s.dn[252][1]) * ddt_scale);
        let eq149_e1890_d_n2: f64 = ((p.p252 * s.dn[252][2]) * ddt_scale);
        let eq149_e1890_d_n3: f64 = ((p.p252 * s.dn[252][3]) * ddt_scale);
        let eq149_e1890_d_n4: f64 = ((p.p252 * s.dn[252][4]) * ddt_scale);
        let eq149_e1890_d_n5: f64 = ((p.p252 * s.dn[252][5]) * ddt_scale);
        let eq149_e1890_d_n6: f64 = ((p.p252 * s.dn[252][6]) * ddt_scale);
        let eq149_e1890_d_n7: f64 = ((p.p252 * s.dn[252][7]) * ddt_scale);
        let eq149_e1890_d_n8: f64 = ((p.p252 * s.dn[252][8]) * ddt_scale);
        let eq149_e1890_d_n9: f64 = ((p.p252 * s.dn[252][9]) * ddt_scale);
        let eq149_e1890_d_n10: f64 = ((p.p252 * s.dn[252][10]) * ddt_scale);
        let eq149_e1890_d_n11: f64 = ((p.p252 * s.dn[252][11]) * ddt_scale);
        let eq149_e1890_d_n12: f64 = ((p.p252 * s.dn[252][12]) * ddt_scale);
        let eq149_e1890_d_n13: f64 = ((p.p252 * s.dn[252][13]) * ddt_scale);
        let eq149_e1890_d_n14: f64 = ((p.p252 * s.dn[252][14]) * ddt_scale);
        let eq149_e1890_d_n15: f64 = ((p.p252 * s.dn[252][15]) * ddt_scale);
        let eq149_e1890_d_n16: f64 = ((p.p252 * s.dn[252][16]) * ddt_scale);
        let eq149_e1890_d_n17: f64 = ((p.p252 * s.dn[252][17]) * ddt_scale);
        let eq149_e1890_d_n18: f64 = ((p.p252 * s.dn[252][18]) * ddt_scale);
        let eq149_e1890_d_n19: f64 = ((p.p252 * s.dn[252][19]) * ddt_scale);
        let eq149_e1890_d_n20: f64 = ((p.p252 * s.dn[252][20]) * ddt_scale);
        let eq149_e1890_d_n21: f64 = ((p.p252 * s.dn[252][21]) * ddt_scale);
        let eq149_e1890_d_n22: f64 = ((p.p252 * s.dn[252][22]) * ddt_scale);
        let eq149_e1890_d_b0: f64 = ((p.p252 * s.db[252][0]) * ddt_scale);
        let eq149_e1890_d_b1: f64 = ((p.p252 * s.db[252][1]) * ddt_scale);
        let eq149_e1890_d_b2: f64 = ((p.p252 * s.db[252][2]) * ddt_scale);
        let eq149_e1890_d_b3: f64 = ((p.p252 * s.db[252][3]) * ddt_scale);
        let eq149_e1890_d_b4: f64 = ((p.p252 * s.db[252][4]) * ddt_scale);
        let eq149_e1890_d_b5: f64 = ((p.p252 * s.db[252][5]) * ddt_scale);
        let eq149_e1890_d_b6: f64 = ((p.p252 * s.db[252][6]) * ddt_scale);
        let eq149_e1890_d_b7: f64 = ((p.p252 * s.db[252][7]) * ddt_scale);
        let eq149_e1890_d_b8: f64 = ((p.p252 * s.db[252][8]) * ddt_scale);
        let eq149_e1890_d_b9: f64 = ((p.p252 * s.db[252][9]) * ddt_scale);
        let eq149_e1890_d_b10: f64 = ((p.p252 * s.db[252][10]) * ddt_scale);
        let eq149_e1890_d_b11: f64 = ((p.p252 * s.db[252][11]) * ddt_scale);
        let eq149_e1890_d_b12: f64 = ((p.p252 * s.db[252][12]) * ddt_scale);
        let eq149_e1890_d_b13: f64 = ((p.p252 * s.db[252][13]) * ddt_scale);
        let eq149_e1890_d_b14: f64 = ((p.p252 * s.db[252][14]) * ddt_scale);
        let eq149_e1890_d_b15: f64 = ((p.p252 * s.db[252][15]) * ddt_scale);
        let eq149_e1890_d_b16: f64 = ((p.p252 * s.db[252][16]) * ddt_scale);
        let eq149_e1890_d_b17: f64 = ((p.p252 * s.db[252][17]) * ddt_scale);
        let eq149_e1890_d_b18: f64 = ((p.p252 * s.db[252][18]) * ddt_scale);
        let eq149_e1890_d_b19: f64 = ((p.p252 * s.db[252][19]) * ddt_scale);
        let eq149_e1890_d_b20: f64 = ((p.p252 * s.db[252][20]) * ddt_scale);
        let eq149_e1890_d_b21: f64 = ((p.p252 * s.db[252][21]) * ddt_scale);
        let eq149_e1890_d_b22: f64 = ((p.p252 * s.db[252][22]) * ddt_scale);
        let eq149_e1890_d_b23: f64 = ((p.p252 * s.db[252][23]) * ddt_scale);
        let eq149_e1890_d_b24: f64 = ((p.p252 * s.db[252][24]) * ddt_scale);
        let eq149_e1890_d_b25: f64 = ((p.p252 * s.db[252][25]) * ddt_scale);
        let eq149_e1890_d_b26: f64 = ((p.p252 * s.db[252][26]) * ddt_scale);
        let eq149_e1890_d_b27: f64 = ((p.p252 * s.db[252][27]) * ddt_scale);
        let eq149_e1890_d_b28: f64 = ((p.p252 * s.db[252][28]) * ddt_scale);
        let eq149_e1890_d_b29: f64 = ((p.p252 * s.db[252][29]) * ddt_scale);
        let eq149_e1890_d_b30: f64 = ((p.p252 * s.db[252][30]) * ddt_scale);
        let eq149_e1890_d_b31: f64 = ((p.p252 * s.db[252][31]) * ddt_scale);
        let eq149_e1890_d_b32: f64 = ((p.p252 * s.db[252][32]) * ddt_scale);
        let eq149_e1890_d_b33: f64 = ((p.p252 * s.db[252][33]) * ddt_scale);
        let eq149_e1890_d_b34: f64 = ((p.p252 * s.db[252][34]) * ddt_scale);
        let eq149_e1890_d_b35: f64 = ((p.p252 * s.db[252][35]) * ddt_scale);
        let eq149_e1890_d_b36: f64 = ((p.p252 * s.db[252][36]) * ddt_scale);
        let eq149_e1890_d_b37: f64 = ((p.p252 * s.db[252][37]) * ddt_scale);
        let eq149_e1890_d_b38: f64 = ((p.p252 * s.db[252][38]) * ddt_scale);
        let eq149_e1890_d_b39: f64 = ((p.p252 * s.db[252][39]) * ddt_scale);
        let eq149_e1890_d_b40: f64 = ((p.p252 * s.db[252][40]) * ddt_scale);
        let eq149_e1890_d_b41: f64 = ((p.p252 * s.db[252][41]) * ddt_scale);
        let eq149_e1890_d_b42: f64 = ((p.p252 * s.db[252][42]) * ddt_scale);
        let eq149_e1890_d_b43: f64 = ((p.p252 * s.db[252][43]) * ddt_scale);
        let eq149_e1890_d_b44: f64 = ((p.p252 * s.db[252][44]) * ddt_scale);
        let eq149_e1890_d_b45: f64 = ((p.p252 * s.db[252][45]) * ddt_scale);
        let eq149_e1890_d_b46: f64 = ((p.p252 * s.db[252][46]) * ddt_scale);
        let eq149_e1890_d_b47: f64 = ((p.p252 * s.db[252][47]) * ddt_scale);
        let eq149_e1890_d_b48: f64 = ((p.p252 * s.db[252][48]) * ddt_scale);
        let eq149_e1890_d_b49: f64 = ((p.p252 * s.db[252][49]) * ddt_scale);
        let eq149_e1890_d_b50: f64 = ((p.p252 * s.db[252][50]) * ddt_scale);
        let eq149_e1890_d_b51: f64 = ((p.p252 * s.db[252][51]) * ddt_scale);
        let eq149_e1890_d_b52: f64 = ((p.p252 * s.db[252][52]) * ddt_scale);
        let eq149_e1890_d_b53: f64 = ((p.p252 * s.db[252][53]) * ddt_scale);
        let eq149_e1890_d_b54: f64 = ((p.p252 * s.db[252][54]) * ddt_scale);
        let eq149_e1891: f64 = (p.p7 * eq149_e1890);
        let eq149_e1891_d_n0: f64 = (p.p7 * eq149_e1890_d_n0);
        let eq149_e1891_d_n1: f64 = (p.p7 * eq149_e1890_d_n1);
        let eq149_e1891_d_n2: f64 = (p.p7 * eq149_e1890_d_n2);
        let eq149_e1891_d_n3: f64 = (p.p7 * eq149_e1890_d_n3);
        let eq149_e1891_d_n4: f64 = (p.p7 * eq149_e1890_d_n4);
        let eq149_e1891_d_n5: f64 = (p.p7 * eq149_e1890_d_n5);
        let eq149_e1891_d_n6: f64 = (p.p7 * eq149_e1890_d_n6);
        let eq149_e1891_d_n7: f64 = (p.p7 * eq149_e1890_d_n7);
        let eq149_e1891_d_n8: f64 = (p.p7 * eq149_e1890_d_n8);
        let eq149_e1891_d_n9: f64 = (p.p7 * eq149_e1890_d_n9);
        let eq149_e1891_d_n10: f64 = (p.p7 * eq149_e1890_d_n10);
        let eq149_e1891_d_n11: f64 = (p.p7 * eq149_e1890_d_n11);
        let eq149_e1891_d_n12: f64 = (p.p7 * eq149_e1890_d_n12);
        let eq149_e1891_d_n13: f64 = (p.p7 * eq149_e1890_d_n13);
        let eq149_e1891_d_n14: f64 = (p.p7 * eq149_e1890_d_n14);
        let eq149_e1891_d_n15: f64 = (p.p7 * eq149_e1890_d_n15);
        let eq149_e1891_d_n16: f64 = (p.p7 * eq149_e1890_d_n16);
        let eq149_e1891_d_n17: f64 = (p.p7 * eq149_e1890_d_n17);
        let eq149_e1891_d_n18: f64 = (p.p7 * eq149_e1890_d_n18);
        let eq149_e1891_d_n19: f64 = (p.p7 * eq149_e1890_d_n19);
        let eq149_e1891_d_n20: f64 = (p.p7 * eq149_e1890_d_n20);
        let eq149_e1891_d_n21: f64 = (p.p7 * eq149_e1890_d_n21);
        let eq149_e1891_d_n22: f64 = (p.p7 * eq149_e1890_d_n22);
        let eq149_e1891_d_b0: f64 = (p.p7 * eq149_e1890_d_b0);
        let eq149_e1891_d_b1: f64 = (p.p7 * eq149_e1890_d_b1);
        let eq149_e1891_d_b2: f64 = (p.p7 * eq149_e1890_d_b2);
        let eq149_e1891_d_b3: f64 = (p.p7 * eq149_e1890_d_b3);
        let eq149_e1891_d_b4: f64 = (p.p7 * eq149_e1890_d_b4);
        let eq149_e1891_d_b5: f64 = (p.p7 * eq149_e1890_d_b5);
        let eq149_e1891_d_b6: f64 = (p.p7 * eq149_e1890_d_b6);
        let eq149_e1891_d_b7: f64 = (p.p7 * eq149_e1890_d_b7);
        let eq149_e1891_d_b8: f64 = (p.p7 * eq149_e1890_d_b8);
        let eq149_e1891_d_b9: f64 = (p.p7 * eq149_e1890_d_b9);
        let eq149_e1891_d_b10: f64 = (p.p7 * eq149_e1890_d_b10);
        let eq149_e1891_d_b11: f64 = (p.p7 * eq149_e1890_d_b11);
        let eq149_e1891_d_b12: f64 = (p.p7 * eq149_e1890_d_b12);
        let eq149_e1891_d_b13: f64 = (p.p7 * eq149_e1890_d_b13);
        let eq149_e1891_d_b14: f64 = (p.p7 * eq149_e1890_d_b14);
        let eq149_e1891_d_b15: f64 = (p.p7 * eq149_e1890_d_b15);
        let eq149_e1891_d_b16: f64 = (p.p7 * eq149_e1890_d_b16);
        let eq149_e1891_d_b17: f64 = (p.p7 * eq149_e1890_d_b17);
        let eq149_e1891_d_b18: f64 = (p.p7 * eq149_e1890_d_b18);
        let eq149_e1891_d_b19: f64 = (p.p7 * eq149_e1890_d_b19);
        let eq149_e1891_d_b20: f64 = (p.p7 * eq149_e1890_d_b20);
        let eq149_e1891_d_b21: f64 = (p.p7 * eq149_e1890_d_b21);
        let eq149_e1891_d_b22: f64 = (p.p7 * eq149_e1890_d_b22);
        let eq149_e1891_d_b23: f64 = (p.p7 * eq149_e1890_d_b23);
        let eq149_e1891_d_b24: f64 = (p.p7 * eq149_e1890_d_b24);
        let eq149_e1891_d_b25: f64 = (p.p7 * eq149_e1890_d_b25);
        let eq149_e1891_d_b26: f64 = (p.p7 * eq149_e1890_d_b26);
        let eq149_e1891_d_b27: f64 = (p.p7 * eq149_e1890_d_b27);
        let eq149_e1891_d_b28: f64 = (p.p7 * eq149_e1890_d_b28);
        let eq149_e1891_d_b29: f64 = (p.p7 * eq149_e1890_d_b29);
        let eq149_e1891_d_b30: f64 = (p.p7 * eq149_e1890_d_b30);
        let eq149_e1891_d_b31: f64 = (p.p7 * eq149_e1890_d_b31);
        let eq149_e1891_d_b32: f64 = (p.p7 * eq149_e1890_d_b32);
        let eq149_e1891_d_b33: f64 = (p.p7 * eq149_e1890_d_b33);
        let eq149_e1891_d_b34: f64 = (p.p7 * eq149_e1890_d_b34);
        let eq149_e1891_d_b35: f64 = (p.p7 * eq149_e1890_d_b35);
        let eq149_e1891_d_b36: f64 = (p.p7 * eq149_e1890_d_b36);
        let eq149_e1891_d_b37: f64 = (p.p7 * eq149_e1890_d_b37);
        let eq149_e1891_d_b38: f64 = (p.p7 * eq149_e1890_d_b38);
        let eq149_e1891_d_b39: f64 = (p.p7 * eq149_e1890_d_b39);
        let eq149_e1891_d_b40: f64 = (p.p7 * eq149_e1890_d_b40);
        let eq149_e1891_d_b41: f64 = (p.p7 * eq149_e1890_d_b41);
        let eq149_e1891_d_b42: f64 = (p.p7 * eq149_e1890_d_b42);
        let eq149_e1891_d_b43: f64 = (p.p7 * eq149_e1890_d_b43);
        let eq149_e1891_d_b44: f64 = (p.p7 * eq149_e1890_d_b44);
        let eq149_e1891_d_b45: f64 = (p.p7 * eq149_e1890_d_b45);
        let eq149_e1891_d_b46: f64 = (p.p7 * eq149_e1890_d_b46);
        let eq149_e1891_d_b47: f64 = (p.p7 * eq149_e1890_d_b47);
        let eq149_e1891_d_b48: f64 = (p.p7 * eq149_e1890_d_b48);
        let eq149_e1891_d_b49: f64 = (p.p7 * eq149_e1890_d_b49);
        let eq149_e1891_d_b50: f64 = (p.p7 * eq149_e1890_d_b50);
        let eq149_e1891_d_b51: f64 = (p.p7 * eq149_e1890_d_b51);
        let eq149_e1891_d_b52: f64 = (p.p7 * eq149_e1890_d_b52);
        let eq149_e1891_d_b53: f64 = (p.p7 * eq149_e1890_d_b53);
        let eq149_e1891_d_b54: f64 = (p.p7 * eq149_e1890_d_b54);
        (eq149_e1891, eq149_e1891_d_n0, eq149_e1891_d_n1, eq149_e1891_d_n2, eq149_e1891_d_n3, eq149_e1891_d_n4, eq149_e1891_d_n5, eq149_e1891_d_n6, eq149_e1891_d_n7, eq149_e1891_d_n8, eq149_e1891_d_n9, eq149_e1891_d_n10, eq149_e1891_d_n11, eq149_e1891_d_n12, eq149_e1891_d_n13, eq149_e1891_d_n14, eq149_e1891_d_n15, eq149_e1891_d_n16, eq149_e1891_d_n17, eq149_e1891_d_n18, eq149_e1891_d_n19, eq149_e1891_d_n20, eq149_e1891_d_n21, eq149_e1891_d_n22, eq149_e1891_d_b0, eq149_e1891_d_b1, eq149_e1891_d_b2, eq149_e1891_d_b3, eq149_e1891_d_b4, eq149_e1891_d_b5, eq149_e1891_d_b6, eq149_e1891_d_b7, eq149_e1891_d_b8, eq149_e1891_d_b9, eq149_e1891_d_b10, eq149_e1891_d_b11, eq149_e1891_d_b12, eq149_e1891_d_b13, eq149_e1891_d_b14, eq149_e1891_d_b15, eq149_e1891_d_b16, eq149_e1891_d_b17, eq149_e1891_d_b18, eq149_e1891_d_b19, eq149_e1891_d_b20, eq149_e1891_d_b21, eq149_e1891_d_b22, eq149_e1891_d_b23, eq149_e1891_d_b24, eq149_e1891_d_b25, eq149_e1891_d_b26, eq149_e1891_d_b27, eq149_e1891_d_b28, eq149_e1891_d_b29, eq149_e1891_d_b30, eq149_e1891_d_b31, eq149_e1891_d_b32, eq149_e1891_d_b33, eq149_e1891_d_b34, eq149_e1891_d_b35, eq149_e1891_d_b36, eq149_e1891_d_b37, eq149_e1891_d_b38, eq149_e1891_d_b39, eq149_e1891_d_b40, eq149_e1891_d_b41, eq149_e1891_d_b42, eq149_e1891_d_b43, eq149_e1891_d_b44, eq149_e1891_d_b45, eq149_e1891_d_b46, eq149_e1891_d_b47, eq149_e1891_d_b48, eq149_e1891_d_b49, eq149_e1891_d_b50, eq149_e1891_d_b51, eq149_e1891_d_b52, eq149_e1891_d_b53, eq149_e1891_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_value: f64 = eq149_e1893;
        let eq149_node_derivatives: [f64; 23] = [eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n10, eq149_e1893_d_n11, eq149_e1893_d_n12, eq149_e1893_d_n13, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22];
        let eq149_branch_derivatives: [f64; 55] = [eq149_e1893_d_b0, eq149_e1893_d_b1, eq149_e1893_d_b2, eq149_e1893_d_b3, eq149_e1893_d_b4, eq149_e1893_d_b5, eq149_e1893_d_b6, eq149_e1893_d_b7, eq149_e1893_d_b8, eq149_e1893_d_b9, eq149_e1893_d_b10, eq149_e1893_d_b11, eq149_e1893_d_b12, eq149_e1893_d_b13, eq149_e1893_d_b14, eq149_e1893_d_b15, eq149_e1893_d_b16, eq149_e1893_d_b17, eq149_e1893_d_b18, eq149_e1893_d_b19, eq149_e1893_d_b20, eq149_e1893_d_b21, eq149_e1893_d_b22, eq149_e1893_d_b23, eq149_e1893_d_b24, eq149_e1893_d_b25, eq149_e1893_d_b26, eq149_e1893_d_b27, eq149_e1893_d_b28, eq149_e1893_d_b29, eq149_e1893_d_b30, eq149_e1893_d_b31, eq149_e1893_d_b32, eq149_e1893_d_b33, eq149_e1893_d_b34, eq149_e1893_d_b35, eq149_e1893_d_b36, eq149_e1893_d_b37, eq149_e1893_d_b38, eq149_e1893_d_b39, eq149_e1893_d_b40, eq149_e1893_d_b41, eq149_e1893_d_b42, eq149_e1893_d_b43, eq149_e1893_d_b44, eq149_e1893_d_b45, eq149_e1893_d_b46, eq149_e1893_d_b47, eq149_e1893_d_b48, eq149_e1893_d_b49, eq149_e1893_d_b50, eq149_e1893_d_b51, eq149_e1893_d_b52, eq149_e1893_d_b53, eq149_e1893_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(15),
            multiplicity * (eq149_value),
            &eq149_node_derivatives,
            &eq149_branch_derivatives,
            multiplicity,
        );
        let (eq150_e1903, eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n10, eq150_e1903_d_n11, eq150_e1903_d_n12, eq150_e1903_d_n13, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22, eq150_e1903_d_b0, eq150_e1903_d_b1, eq150_e1903_d_b2, eq150_e1903_d_b3, eq150_e1903_d_b4, eq150_e1903_d_b5, eq150_e1903_d_b6, eq150_e1903_d_b7, eq150_e1903_d_b8, eq150_e1903_d_b9, eq150_e1903_d_b10, eq150_e1903_d_b11, eq150_e1903_d_b12, eq150_e1903_d_b13, eq150_e1903_d_b14, eq150_e1903_d_b15, eq150_e1903_d_b16, eq150_e1903_d_b17, eq150_e1903_d_b18, eq150_e1903_d_b19, eq150_e1903_d_b20, eq150_e1903_d_b21, eq150_e1903_d_b22, eq150_e1903_d_b23, eq150_e1903_d_b24, eq150_e1903_d_b25, eq150_e1903_d_b26, eq150_e1903_d_b27, eq150_e1903_d_b28, eq150_e1903_d_b29, eq150_e1903_d_b30, eq150_e1903_d_b31, eq150_e1903_d_b32, eq150_e1903_d_b33, eq150_e1903_d_b34, eq150_e1903_d_b35, eq150_e1903_d_b36, eq150_e1903_d_b37, eq150_e1903_d_b38, eq150_e1903_d_b39, eq150_e1903_d_b40, eq150_e1903_d_b41, eq150_e1903_d_b42, eq150_e1903_d_b43, eq150_e1903_d_b44, eq150_e1903_d_b45, eq150_e1903_d_b46, eq150_e1903_d_b47, eq150_e1903_d_b48, eq150_e1903_d_b49, eq150_e1903_d_b50, eq150_e1903_d_b51, eq150_e1903_d_b52, eq150_e1903_d_b53, eq150_e1903_d_b54,) = {
    if ((!s.b[580]) && s.b[583]) {
        let eq150_e1900: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 49, s.v[253]);
        let eq150_e1901: f64 = (p.p7 * eq150_e1900);
        let eq150_e1901_d_n0: f64 = (p.p7 * (s.dn[253][0] * ddt_scale));
        let eq150_e1901_d_n1: f64 = (p.p7 * (s.dn[253][1] * ddt_scale));
        let eq150_e1901_d_n2: f64 = (p.p7 * (s.dn[253][2] * ddt_scale));
        let eq150_e1901_d_n3: f64 = (p.p7 * (s.dn[253][3] * ddt_scale));
        let eq150_e1901_d_n4: f64 = (p.p7 * (s.dn[253][4] * ddt_scale));
        let eq150_e1901_d_n5: f64 = (p.p7 * (s.dn[253][5] * ddt_scale));
        let eq150_e1901_d_n6: f64 = (p.p7 * (s.dn[253][6] * ddt_scale));
        let eq150_e1901_d_n7: f64 = (p.p7 * (s.dn[253][7] * ddt_scale));
        let eq150_e1901_d_n8: f64 = (p.p7 * (s.dn[253][8] * ddt_scale));
        let eq150_e1901_d_n9: f64 = (p.p7 * (s.dn[253][9] * ddt_scale));
        let eq150_e1901_d_n10: f64 = (p.p7 * (s.dn[253][10] * ddt_scale));
        let eq150_e1901_d_n11: f64 = (p.p7 * (s.dn[253][11] * ddt_scale));
        let eq150_e1901_d_n12: f64 = (p.p7 * (s.dn[253][12] * ddt_scale));
        let eq150_e1901_d_n13: f64 = (p.p7 * (s.dn[253][13] * ddt_scale));
        let eq150_e1901_d_n14: f64 = (p.p7 * (s.dn[253][14] * ddt_scale));
        let eq150_e1901_d_n15: f64 = (p.p7 * (s.dn[253][15] * ddt_scale));
        let eq150_e1901_d_n16: f64 = (p.p7 * (s.dn[253][16] * ddt_scale));
        let eq150_e1901_d_n17: f64 = (p.p7 * (s.dn[253][17] * ddt_scale));
        let eq150_e1901_d_n18: f64 = (p.p7 * (s.dn[253][18] * ddt_scale));
        let eq150_e1901_d_n19: f64 = (p.p7 * (s.dn[253][19] * ddt_scale));
        let eq150_e1901_d_n20: f64 = (p.p7 * (s.dn[253][20] * ddt_scale));
        let eq150_e1901_d_n21: f64 = (p.p7 * (s.dn[253][21] * ddt_scale));
        let eq150_e1901_d_n22: f64 = (p.p7 * (s.dn[253][22] * ddt_scale));
        let eq150_e1901_d_b0: f64 = (p.p7 * (s.db[253][0] * ddt_scale));
        let eq150_e1901_d_b1: f64 = (p.p7 * (s.db[253][1] * ddt_scale));
        let eq150_e1901_d_b2: f64 = (p.p7 * (s.db[253][2] * ddt_scale));
        let eq150_e1901_d_b3: f64 = (p.p7 * (s.db[253][3] * ddt_scale));
        let eq150_e1901_d_b4: f64 = (p.p7 * (s.db[253][4] * ddt_scale));
        let eq150_e1901_d_b5: f64 = (p.p7 * (s.db[253][5] * ddt_scale));
        let eq150_e1901_d_b6: f64 = (p.p7 * (s.db[253][6] * ddt_scale));
        let eq150_e1901_d_b7: f64 = (p.p7 * (s.db[253][7] * ddt_scale));
        let eq150_e1901_d_b8: f64 = (p.p7 * (s.db[253][8] * ddt_scale));
        let eq150_e1901_d_b9: f64 = (p.p7 * (s.db[253][9] * ddt_scale));
        let eq150_e1901_d_b10: f64 = (p.p7 * (s.db[253][10] * ddt_scale));
        let eq150_e1901_d_b11: f64 = (p.p7 * (s.db[253][11] * ddt_scale));
        let eq150_e1901_d_b12: f64 = (p.p7 * (s.db[253][12] * ddt_scale));
        let eq150_e1901_d_b13: f64 = (p.p7 * (s.db[253][13] * ddt_scale));
        let eq150_e1901_d_b14: f64 = (p.p7 * (s.db[253][14] * ddt_scale));
        let eq150_e1901_d_b15: f64 = (p.p7 * (s.db[253][15] * ddt_scale));
        let eq150_e1901_d_b16: f64 = (p.p7 * (s.db[253][16] * ddt_scale));
        let eq150_e1901_d_b17: f64 = (p.p7 * (s.db[253][17] * ddt_scale));
        let eq150_e1901_d_b18: f64 = (p.p7 * (s.db[253][18] * ddt_scale));
        let eq150_e1901_d_b19: f64 = (p.p7 * (s.db[253][19] * ddt_scale));
        let eq150_e1901_d_b20: f64 = (p.p7 * (s.db[253][20] * ddt_scale));
        let eq150_e1901_d_b21: f64 = (p.p7 * (s.db[253][21] * ddt_scale));
        let eq150_e1901_d_b22: f64 = (p.p7 * (s.db[253][22] * ddt_scale));
        let eq150_e1901_d_b23: f64 = (p.p7 * (s.db[253][23] * ddt_scale));
        let eq150_e1901_d_b24: f64 = (p.p7 * (s.db[253][24] * ddt_scale));
        let eq150_e1901_d_b25: f64 = (p.p7 * (s.db[253][25] * ddt_scale));
        let eq150_e1901_d_b26: f64 = (p.p7 * (s.db[253][26] * ddt_scale));
        let eq150_e1901_d_b27: f64 = (p.p7 * (s.db[253][27] * ddt_scale));
        let eq150_e1901_d_b28: f64 = (p.p7 * (s.db[253][28] * ddt_scale));
        let eq150_e1901_d_b29: f64 = (p.p7 * (s.db[253][29] * ddt_scale));
        let eq150_e1901_d_b30: f64 = (p.p7 * (s.db[253][30] * ddt_scale));
        let eq150_e1901_d_b31: f64 = (p.p7 * (s.db[253][31] * ddt_scale));
        let eq150_e1901_d_b32: f64 = (p.p7 * (s.db[253][32] * ddt_scale));
        let eq150_e1901_d_b33: f64 = (p.p7 * (s.db[253][33] * ddt_scale));
        let eq150_e1901_d_b34: f64 = (p.p7 * (s.db[253][34] * ddt_scale));
        let eq150_e1901_d_b35: f64 = (p.p7 * (s.db[253][35] * ddt_scale));
        let eq150_e1901_d_b36: f64 = (p.p7 * (s.db[253][36] * ddt_scale));
        let eq150_e1901_d_b37: f64 = (p.p7 * (s.db[253][37] * ddt_scale));
        let eq150_e1901_d_b38: f64 = (p.p7 * (s.db[253][38] * ddt_scale));
        let eq150_e1901_d_b39: f64 = (p.p7 * (s.db[253][39] * ddt_scale));
        let eq150_e1901_d_b40: f64 = (p.p7 * (s.db[253][40] * ddt_scale));
        let eq150_e1901_d_b41: f64 = (p.p7 * (s.db[253][41] * ddt_scale));
        let eq150_e1901_d_b42: f64 = (p.p7 * (s.db[253][42] * ddt_scale));
        let eq150_e1901_d_b43: f64 = (p.p7 * (s.db[253][43] * ddt_scale));
        let eq150_e1901_d_b44: f64 = (p.p7 * (s.db[253][44] * ddt_scale));
        let eq150_e1901_d_b45: f64 = (p.p7 * (s.db[253][45] * ddt_scale));
        let eq150_e1901_d_b46: f64 = (p.p7 * (s.db[253][46] * ddt_scale));
        let eq150_e1901_d_b47: f64 = (p.p7 * (s.db[253][47] * ddt_scale));
        let eq150_e1901_d_b48: f64 = (p.p7 * (s.db[253][48] * ddt_scale));
        let eq150_e1901_d_b49: f64 = (p.p7 * (s.db[253][49] * ddt_scale));
        let eq150_e1901_d_b50: f64 = (p.p7 * (s.db[253][50] * ddt_scale));
        let eq150_e1901_d_b51: f64 = (p.p7 * (s.db[253][51] * ddt_scale));
        let eq150_e1901_d_b52: f64 = (p.p7 * (s.db[253][52] * ddt_scale));
        let eq150_e1901_d_b53: f64 = (p.p7 * (s.db[253][53] * ddt_scale));
        let eq150_e1901_d_b54: f64 = (p.p7 * (s.db[253][54] * ddt_scale));
        (eq150_e1901, eq150_e1901_d_n0, eq150_e1901_d_n1, eq150_e1901_d_n2, eq150_e1901_d_n3, eq150_e1901_d_n4, eq150_e1901_d_n5, eq150_e1901_d_n6, eq150_e1901_d_n7, eq150_e1901_d_n8, eq150_e1901_d_n9, eq150_e1901_d_n10, eq150_e1901_d_n11, eq150_e1901_d_n12, eq150_e1901_d_n13, eq150_e1901_d_n14, eq150_e1901_d_n15, eq150_e1901_d_n16, eq150_e1901_d_n17, eq150_e1901_d_n18, eq150_e1901_d_n19, eq150_e1901_d_n20, eq150_e1901_d_n21, eq150_e1901_d_n22, eq150_e1901_d_b0, eq150_e1901_d_b1, eq150_e1901_d_b2, eq150_e1901_d_b3, eq150_e1901_d_b4, eq150_e1901_d_b5, eq150_e1901_d_b6, eq150_e1901_d_b7, eq150_e1901_d_b8, eq150_e1901_d_b9, eq150_e1901_d_b10, eq150_e1901_d_b11, eq150_e1901_d_b12, eq150_e1901_d_b13, eq150_e1901_d_b14, eq150_e1901_d_b15, eq150_e1901_d_b16, eq150_e1901_d_b17, eq150_e1901_d_b18, eq150_e1901_d_b19, eq150_e1901_d_b20, eq150_e1901_d_b21, eq150_e1901_d_b22, eq150_e1901_d_b23, eq150_e1901_d_b24, eq150_e1901_d_b25, eq150_e1901_d_b26, eq150_e1901_d_b27, eq150_e1901_d_b28, eq150_e1901_d_b29, eq150_e1901_d_b30, eq150_e1901_d_b31, eq150_e1901_d_b32, eq150_e1901_d_b33, eq150_e1901_d_b34, eq150_e1901_d_b35, eq150_e1901_d_b36, eq150_e1901_d_b37, eq150_e1901_d_b38, eq150_e1901_d_b39, eq150_e1901_d_b40, eq150_e1901_d_b41, eq150_e1901_d_b42, eq150_e1901_d_b43, eq150_e1901_d_b44, eq150_e1901_d_b45, eq150_e1901_d_b46, eq150_e1901_d_b47, eq150_e1901_d_b48, eq150_e1901_d_b49, eq150_e1901_d_b50, eq150_e1901_d_b51, eq150_e1901_d_b52, eq150_e1901_d_b53, eq150_e1901_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_value: f64 = eq150_e1903;
        let eq150_node_derivatives: [f64; 23] = [eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n10, eq150_e1903_d_n11, eq150_e1903_d_n12, eq150_e1903_d_n13, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22];
        let eq150_branch_derivatives: [f64; 55] = [eq150_e1903_d_b0, eq150_e1903_d_b1, eq150_e1903_d_b2, eq150_e1903_d_b3, eq150_e1903_d_b4, eq150_e1903_d_b5, eq150_e1903_d_b6, eq150_e1903_d_b7, eq150_e1903_d_b8, eq150_e1903_d_b9, eq150_e1903_d_b10, eq150_e1903_d_b11, eq150_e1903_d_b12, eq150_e1903_d_b13, eq150_e1903_d_b14, eq150_e1903_d_b15, eq150_e1903_d_b16, eq150_e1903_d_b17, eq150_e1903_d_b18, eq150_e1903_d_b19, eq150_e1903_d_b20, eq150_e1903_d_b21, eq150_e1903_d_b22, eq150_e1903_d_b23, eq150_e1903_d_b24, eq150_e1903_d_b25, eq150_e1903_d_b26, eq150_e1903_d_b27, eq150_e1903_d_b28, eq150_e1903_d_b29, eq150_e1903_d_b30, eq150_e1903_d_b31, eq150_e1903_d_b32, eq150_e1903_d_b33, eq150_e1903_d_b34, eq150_e1903_d_b35, eq150_e1903_d_b36, eq150_e1903_d_b37, eq150_e1903_d_b38, eq150_e1903_d_b39, eq150_e1903_d_b40, eq150_e1903_d_b41, eq150_e1903_d_b42, eq150_e1903_d_b43, eq150_e1903_d_b44, eq150_e1903_d_b45, eq150_e1903_d_b46, eq150_e1903_d_b47, eq150_e1903_d_b48, eq150_e1903_d_b49, eq150_e1903_d_b50, eq150_e1903_d_b51, eq150_e1903_d_b52, eq150_e1903_d_b53, eq150_e1903_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq150_value),
            &eq150_node_derivatives,
            &eq150_branch_derivatives,
            multiplicity,
        );
        let (eq151_e1915, eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n10, eq151_e1915_d_n11, eq151_e1915_d_n12, eq151_e1915_d_n13, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22, eq151_e1915_d_b0, eq151_e1915_d_b1, eq151_e1915_d_b2, eq151_e1915_d_b3, eq151_e1915_d_b4, eq151_e1915_d_b5, eq151_e1915_d_b6, eq151_e1915_d_b7, eq151_e1915_d_b8, eq151_e1915_d_b9, eq151_e1915_d_b10, eq151_e1915_d_b11, eq151_e1915_d_b12, eq151_e1915_d_b13, eq151_e1915_d_b14, eq151_e1915_d_b15, eq151_e1915_d_b16, eq151_e1915_d_b17, eq151_e1915_d_b18, eq151_e1915_d_b19, eq151_e1915_d_b20, eq151_e1915_d_b21, eq151_e1915_d_b22, eq151_e1915_d_b23, eq151_e1915_d_b24, eq151_e1915_d_b25, eq151_e1915_d_b26, eq151_e1915_d_b27, eq151_e1915_d_b28, eq151_e1915_d_b29, eq151_e1915_d_b30, eq151_e1915_d_b31, eq151_e1915_d_b32, eq151_e1915_d_b33, eq151_e1915_d_b34, eq151_e1915_d_b35, eq151_e1915_d_b36, eq151_e1915_d_b37, eq151_e1915_d_b38, eq151_e1915_d_b39, eq151_e1915_d_b40, eq151_e1915_d_b41, eq151_e1915_d_b42, eq151_e1915_d_b43, eq151_e1915_d_b44, eq151_e1915_d_b45, eq151_e1915_d_b46, eq151_e1915_d_b47, eq151_e1915_d_b48, eq151_e1915_d_b49, eq151_e1915_d_b50, eq151_e1915_d_b51, eq151_e1915_d_b52, eq151_e1915_d_b53, eq151_e1915_d_b54,) = {
    if (((!s.b[580]) && s.b[583]) && s.b[584]) {
        let eq151_e1912: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 50, s.v[252]);
        let eq151_e1913: f64 = (p.p7 * eq151_e1912);
        let eq151_e1913_d_n0: f64 = (p.p7 * (s.dn[252][0] * ddt_scale));
        let eq151_e1913_d_n1: f64 = (p.p7 * (s.dn[252][1] * ddt_scale));
        let eq151_e1913_d_n2: f64 = (p.p7 * (s.dn[252][2] * ddt_scale));
        let eq151_e1913_d_n3: f64 = (p.p7 * (s.dn[252][3] * ddt_scale));
        let eq151_e1913_d_n4: f64 = (p.p7 * (s.dn[252][4] * ddt_scale));
        let eq151_e1913_d_n5: f64 = (p.p7 * (s.dn[252][5] * ddt_scale));
        let eq151_e1913_d_n6: f64 = (p.p7 * (s.dn[252][6] * ddt_scale));
        let eq151_e1913_d_n7: f64 = (p.p7 * (s.dn[252][7] * ddt_scale));
        let eq151_e1913_d_n8: f64 = (p.p7 * (s.dn[252][8] * ddt_scale));
        let eq151_e1913_d_n9: f64 = (p.p7 * (s.dn[252][9] * ddt_scale));
        let eq151_e1913_d_n10: f64 = (p.p7 * (s.dn[252][10] * ddt_scale));
        let eq151_e1913_d_n11: f64 = (p.p7 * (s.dn[252][11] * ddt_scale));
        let eq151_e1913_d_n12: f64 = (p.p7 * (s.dn[252][12] * ddt_scale));
        let eq151_e1913_d_n13: f64 = (p.p7 * (s.dn[252][13] * ddt_scale));
        let eq151_e1913_d_n14: f64 = (p.p7 * (s.dn[252][14] * ddt_scale));
        let eq151_e1913_d_n15: f64 = (p.p7 * (s.dn[252][15] * ddt_scale));
        let eq151_e1913_d_n16: f64 = (p.p7 * (s.dn[252][16] * ddt_scale));
        let eq151_e1913_d_n17: f64 = (p.p7 * (s.dn[252][17] * ddt_scale));
        let eq151_e1913_d_n18: f64 = (p.p7 * (s.dn[252][18] * ddt_scale));
        let eq151_e1913_d_n19: f64 = (p.p7 * (s.dn[252][19] * ddt_scale));
        let eq151_e1913_d_n20: f64 = (p.p7 * (s.dn[252][20] * ddt_scale));
        let eq151_e1913_d_n21: f64 = (p.p7 * (s.dn[252][21] * ddt_scale));
        let eq151_e1913_d_n22: f64 = (p.p7 * (s.dn[252][22] * ddt_scale));
        let eq151_e1913_d_b0: f64 = (p.p7 * (s.db[252][0] * ddt_scale));
        let eq151_e1913_d_b1: f64 = (p.p7 * (s.db[252][1] * ddt_scale));
        let eq151_e1913_d_b2: f64 = (p.p7 * (s.db[252][2] * ddt_scale));
        let eq151_e1913_d_b3: f64 = (p.p7 * (s.db[252][3] * ddt_scale));
        let eq151_e1913_d_b4: f64 = (p.p7 * (s.db[252][4] * ddt_scale));
        let eq151_e1913_d_b5: f64 = (p.p7 * (s.db[252][5] * ddt_scale));
        let eq151_e1913_d_b6: f64 = (p.p7 * (s.db[252][6] * ddt_scale));
        let eq151_e1913_d_b7: f64 = (p.p7 * (s.db[252][7] * ddt_scale));
        let eq151_e1913_d_b8: f64 = (p.p7 * (s.db[252][8] * ddt_scale));
        let eq151_e1913_d_b9: f64 = (p.p7 * (s.db[252][9] * ddt_scale));
        let eq151_e1913_d_b10: f64 = (p.p7 * (s.db[252][10] * ddt_scale));
        let eq151_e1913_d_b11: f64 = (p.p7 * (s.db[252][11] * ddt_scale));
        let eq151_e1913_d_b12: f64 = (p.p7 * (s.db[252][12] * ddt_scale));
        let eq151_e1913_d_b13: f64 = (p.p7 * (s.db[252][13] * ddt_scale));
        let eq151_e1913_d_b14: f64 = (p.p7 * (s.db[252][14] * ddt_scale));
        let eq151_e1913_d_b15: f64 = (p.p7 * (s.db[252][15] * ddt_scale));
        let eq151_e1913_d_b16: f64 = (p.p7 * (s.db[252][16] * ddt_scale));
        let eq151_e1913_d_b17: f64 = (p.p7 * (s.db[252][17] * ddt_scale));
        let eq151_e1913_d_b18: f64 = (p.p7 * (s.db[252][18] * ddt_scale));
        let eq151_e1913_d_b19: f64 = (p.p7 * (s.db[252][19] * ddt_scale));
        let eq151_e1913_d_b20: f64 = (p.p7 * (s.db[252][20] * ddt_scale));
        let eq151_e1913_d_b21: f64 = (p.p7 * (s.db[252][21] * ddt_scale));
        let eq151_e1913_d_b22: f64 = (p.p7 * (s.db[252][22] * ddt_scale));
        let eq151_e1913_d_b23: f64 = (p.p7 * (s.db[252][23] * ddt_scale));
        let eq151_e1913_d_b24: f64 = (p.p7 * (s.db[252][24] * ddt_scale));
        let eq151_e1913_d_b25: f64 = (p.p7 * (s.db[252][25] * ddt_scale));
        let eq151_e1913_d_b26: f64 = (p.p7 * (s.db[252][26] * ddt_scale));
        let eq151_e1913_d_b27: f64 = (p.p7 * (s.db[252][27] * ddt_scale));
        let eq151_e1913_d_b28: f64 = (p.p7 * (s.db[252][28] * ddt_scale));
        let eq151_e1913_d_b29: f64 = (p.p7 * (s.db[252][29] * ddt_scale));
        let eq151_e1913_d_b30: f64 = (p.p7 * (s.db[252][30] * ddt_scale));
        let eq151_e1913_d_b31: f64 = (p.p7 * (s.db[252][31] * ddt_scale));
        let eq151_e1913_d_b32: f64 = (p.p7 * (s.db[252][32] * ddt_scale));
        let eq151_e1913_d_b33: f64 = (p.p7 * (s.db[252][33] * ddt_scale));
        let eq151_e1913_d_b34: f64 = (p.p7 * (s.db[252][34] * ddt_scale));
        let eq151_e1913_d_b35: f64 = (p.p7 * (s.db[252][35] * ddt_scale));
        let eq151_e1913_d_b36: f64 = (p.p7 * (s.db[252][36] * ddt_scale));
        let eq151_e1913_d_b37: f64 = (p.p7 * (s.db[252][37] * ddt_scale));
        let eq151_e1913_d_b38: f64 = (p.p7 * (s.db[252][38] * ddt_scale));
        let eq151_e1913_d_b39: f64 = (p.p7 * (s.db[252][39] * ddt_scale));
        let eq151_e1913_d_b40: f64 = (p.p7 * (s.db[252][40] * ddt_scale));
        let eq151_e1913_d_b41: f64 = (p.p7 * (s.db[252][41] * ddt_scale));
        let eq151_e1913_d_b42: f64 = (p.p7 * (s.db[252][42] * ddt_scale));
        let eq151_e1913_d_b43: f64 = (p.p7 * (s.db[252][43] * ddt_scale));
        let eq151_e1913_d_b44: f64 = (p.p7 * (s.db[252][44] * ddt_scale));
        let eq151_e1913_d_b45: f64 = (p.p7 * (s.db[252][45] * ddt_scale));
        let eq151_e1913_d_b46: f64 = (p.p7 * (s.db[252][46] * ddt_scale));
        let eq151_e1913_d_b47: f64 = (p.p7 * (s.db[252][47] * ddt_scale));
        let eq151_e1913_d_b48: f64 = (p.p7 * (s.db[252][48] * ddt_scale));
        let eq151_e1913_d_b49: f64 = (p.p7 * (s.db[252][49] * ddt_scale));
        let eq151_e1913_d_b50: f64 = (p.p7 * (s.db[252][50] * ddt_scale));
        let eq151_e1913_d_b51: f64 = (p.p7 * (s.db[252][51] * ddt_scale));
        let eq151_e1913_d_b52: f64 = (p.p7 * (s.db[252][52] * ddt_scale));
        let eq151_e1913_d_b53: f64 = (p.p7 * (s.db[252][53] * ddt_scale));
        let eq151_e1913_d_b54: f64 = (p.p7 * (s.db[252][54] * ddt_scale));
        (eq151_e1913, eq151_e1913_d_n0, eq151_e1913_d_n1, eq151_e1913_d_n2, eq151_e1913_d_n3, eq151_e1913_d_n4, eq151_e1913_d_n5, eq151_e1913_d_n6, eq151_e1913_d_n7, eq151_e1913_d_n8, eq151_e1913_d_n9, eq151_e1913_d_n10, eq151_e1913_d_n11, eq151_e1913_d_n12, eq151_e1913_d_n13, eq151_e1913_d_n14, eq151_e1913_d_n15, eq151_e1913_d_n16, eq151_e1913_d_n17, eq151_e1913_d_n18, eq151_e1913_d_n19, eq151_e1913_d_n20, eq151_e1913_d_n21, eq151_e1913_d_n22, eq151_e1913_d_b0, eq151_e1913_d_b1, eq151_e1913_d_b2, eq151_e1913_d_b3, eq151_e1913_d_b4, eq151_e1913_d_b5, eq151_e1913_d_b6, eq151_e1913_d_b7, eq151_e1913_d_b8, eq151_e1913_d_b9, eq151_e1913_d_b10, eq151_e1913_d_b11, eq151_e1913_d_b12, eq151_e1913_d_b13, eq151_e1913_d_b14, eq151_e1913_d_b15, eq151_e1913_d_b16, eq151_e1913_d_b17, eq151_e1913_d_b18, eq151_e1913_d_b19, eq151_e1913_d_b20, eq151_e1913_d_b21, eq151_e1913_d_b22, eq151_e1913_d_b23, eq151_e1913_d_b24, eq151_e1913_d_b25, eq151_e1913_d_b26, eq151_e1913_d_b27, eq151_e1913_d_b28, eq151_e1913_d_b29, eq151_e1913_d_b30, eq151_e1913_d_b31, eq151_e1913_d_b32, eq151_e1913_d_b33, eq151_e1913_d_b34, eq151_e1913_d_b35, eq151_e1913_d_b36, eq151_e1913_d_b37, eq151_e1913_d_b38, eq151_e1913_d_b39, eq151_e1913_d_b40, eq151_e1913_d_b41, eq151_e1913_d_b42, eq151_e1913_d_b43, eq151_e1913_d_b44, eq151_e1913_d_b45, eq151_e1913_d_b46, eq151_e1913_d_b47, eq151_e1913_d_b48, eq151_e1913_d_b49, eq151_e1913_d_b50, eq151_e1913_d_b51, eq151_e1913_d_b52, eq151_e1913_d_b53, eq151_e1913_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_value: f64 = eq151_e1915;
        let eq151_node_derivatives: [f64; 23] = [eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n10, eq151_e1915_d_n11, eq151_e1915_d_n12, eq151_e1915_d_n13, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22];
        let eq151_branch_derivatives: [f64; 55] = [eq151_e1915_d_b0, eq151_e1915_d_b1, eq151_e1915_d_b2, eq151_e1915_d_b3, eq151_e1915_d_b4, eq151_e1915_d_b5, eq151_e1915_d_b6, eq151_e1915_d_b7, eq151_e1915_d_b8, eq151_e1915_d_b9, eq151_e1915_d_b10, eq151_e1915_d_b11, eq151_e1915_d_b12, eq151_e1915_d_b13, eq151_e1915_d_b14, eq151_e1915_d_b15, eq151_e1915_d_b16, eq151_e1915_d_b17, eq151_e1915_d_b18, eq151_e1915_d_b19, eq151_e1915_d_b20, eq151_e1915_d_b21, eq151_e1915_d_b22, eq151_e1915_d_b23, eq151_e1915_d_b24, eq151_e1915_d_b25, eq151_e1915_d_b26, eq151_e1915_d_b27, eq151_e1915_d_b28, eq151_e1915_d_b29, eq151_e1915_d_b30, eq151_e1915_d_b31, eq151_e1915_d_b32, eq151_e1915_d_b33, eq151_e1915_d_b34, eq151_e1915_d_b35, eq151_e1915_d_b36, eq151_e1915_d_b37, eq151_e1915_d_b38, eq151_e1915_d_b39, eq151_e1915_d_b40, eq151_e1915_d_b41, eq151_e1915_d_b42, eq151_e1915_d_b43, eq151_e1915_d_b44, eq151_e1915_d_b45, eq151_e1915_d_b46, eq151_e1915_d_b47, eq151_e1915_d_b48, eq151_e1915_d_b49, eq151_e1915_d_b50, eq151_e1915_d_b51, eq151_e1915_d_b52, eq151_e1915_d_b53, eq151_e1915_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq151_value),
            &eq151_node_derivatives,
            &eq151_branch_derivatives,
            multiplicity,
        );
        let (eq152_e1929, eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n10, eq152_e1929_d_n11, eq152_e1929_d_n12, eq152_e1929_d_n13, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22, eq152_e1929_d_b0, eq152_e1929_d_b1, eq152_e1929_d_b2, eq152_e1929_d_b3, eq152_e1929_d_b4, eq152_e1929_d_b5, eq152_e1929_d_b6, eq152_e1929_d_b7, eq152_e1929_d_b8, eq152_e1929_d_b9, eq152_e1929_d_b10, eq152_e1929_d_b11, eq152_e1929_d_b12, eq152_e1929_d_b13, eq152_e1929_d_b14, eq152_e1929_d_b15, eq152_e1929_d_b16, eq152_e1929_d_b17, eq152_e1929_d_b18, eq152_e1929_d_b19, eq152_e1929_d_b20, eq152_e1929_d_b21, eq152_e1929_d_b22, eq152_e1929_d_b23, eq152_e1929_d_b24, eq152_e1929_d_b25, eq152_e1929_d_b26, eq152_e1929_d_b27, eq152_e1929_d_b28, eq152_e1929_d_b29, eq152_e1929_d_b30, eq152_e1929_d_b31, eq152_e1929_d_b32, eq152_e1929_d_b33, eq152_e1929_d_b34, eq152_e1929_d_b35, eq152_e1929_d_b36, eq152_e1929_d_b37, eq152_e1929_d_b38, eq152_e1929_d_b39, eq152_e1929_d_b40, eq152_e1929_d_b41, eq152_e1929_d_b42, eq152_e1929_d_b43, eq152_e1929_d_b44, eq152_e1929_d_b45, eq152_e1929_d_b46, eq152_e1929_d_b47, eq152_e1929_d_b48, eq152_e1929_d_b49, eq152_e1929_d_b50, eq152_e1929_d_b51, eq152_e1929_d_b52, eq152_e1929_d_b53, eq152_e1929_d_b54,) = {
    if (((!s.b[580]) && s.b[583]) && s.b[584]) {
        let eq152_e1924: f64 = (p.p7 * p.p247);
        let eq152_e1926: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 51, s.v[252]);
        let eq152_e1927: f64 = (eq152_e1924 * eq152_e1926);
        let eq152_e1927_d_n0: f64 = (eq152_e1924 * (s.dn[252][0] * ddt_scale));
        let eq152_e1927_d_n1: f64 = (eq152_e1924 * (s.dn[252][1] * ddt_scale));
        let eq152_e1927_d_n2: f64 = (eq152_e1924 * (s.dn[252][2] * ddt_scale));
        let eq152_e1927_d_n3: f64 = (eq152_e1924 * (s.dn[252][3] * ddt_scale));
        let eq152_e1927_d_n4: f64 = (eq152_e1924 * (s.dn[252][4] * ddt_scale));
        let eq152_e1927_d_n5: f64 = (eq152_e1924 * (s.dn[252][5] * ddt_scale));
        let eq152_e1927_d_n6: f64 = (eq152_e1924 * (s.dn[252][6] * ddt_scale));
        let eq152_e1927_d_n7: f64 = (eq152_e1924 * (s.dn[252][7] * ddt_scale));
        let eq152_e1927_d_n8: f64 = (eq152_e1924 * (s.dn[252][8] * ddt_scale));
        let eq152_e1927_d_n9: f64 = (eq152_e1924 * (s.dn[252][9] * ddt_scale));
        let eq152_e1927_d_n10: f64 = (eq152_e1924 * (s.dn[252][10] * ddt_scale));
        let eq152_e1927_d_n11: f64 = (eq152_e1924 * (s.dn[252][11] * ddt_scale));
        let eq152_e1927_d_n12: f64 = (eq152_e1924 * (s.dn[252][12] * ddt_scale));
        let eq152_e1927_d_n13: f64 = (eq152_e1924 * (s.dn[252][13] * ddt_scale));
        let eq152_e1927_d_n14: f64 = (eq152_e1924 * (s.dn[252][14] * ddt_scale));
        let eq152_e1927_d_n15: f64 = (eq152_e1924 * (s.dn[252][15] * ddt_scale));
        let eq152_e1927_d_n16: f64 = (eq152_e1924 * (s.dn[252][16] * ddt_scale));
        let eq152_e1927_d_n17: f64 = (eq152_e1924 * (s.dn[252][17] * ddt_scale));
        let eq152_e1927_d_n18: f64 = (eq152_e1924 * (s.dn[252][18] * ddt_scale));
        let eq152_e1927_d_n19: f64 = (eq152_e1924 * (s.dn[252][19] * ddt_scale));
        let eq152_e1927_d_n20: f64 = (eq152_e1924 * (s.dn[252][20] * ddt_scale));
        let eq152_e1927_d_n21: f64 = (eq152_e1924 * (s.dn[252][21] * ddt_scale));
        let eq152_e1927_d_n22: f64 = (eq152_e1924 * (s.dn[252][22] * ddt_scale));
        let eq152_e1927_d_b0: f64 = (eq152_e1924 * (s.db[252][0] * ddt_scale));
        let eq152_e1927_d_b1: f64 = (eq152_e1924 * (s.db[252][1] * ddt_scale));
        let eq152_e1927_d_b2: f64 = (eq152_e1924 * (s.db[252][2] * ddt_scale));
        let eq152_e1927_d_b3: f64 = (eq152_e1924 * (s.db[252][3] * ddt_scale));
        let eq152_e1927_d_b4: f64 = (eq152_e1924 * (s.db[252][4] * ddt_scale));
        let eq152_e1927_d_b5: f64 = (eq152_e1924 * (s.db[252][5] * ddt_scale));
        let eq152_e1927_d_b6: f64 = (eq152_e1924 * (s.db[252][6] * ddt_scale));
        let eq152_e1927_d_b7: f64 = (eq152_e1924 * (s.db[252][7] * ddt_scale));
        let eq152_e1927_d_b8: f64 = (eq152_e1924 * (s.db[252][8] * ddt_scale));
        let eq152_e1927_d_b9: f64 = (eq152_e1924 * (s.db[252][9] * ddt_scale));
        let eq152_e1927_d_b10: f64 = (eq152_e1924 * (s.db[252][10] * ddt_scale));
        let eq152_e1927_d_b11: f64 = (eq152_e1924 * (s.db[252][11] * ddt_scale));
        let eq152_e1927_d_b12: f64 = (eq152_e1924 * (s.db[252][12] * ddt_scale));
        let eq152_e1927_d_b13: f64 = (eq152_e1924 * (s.db[252][13] * ddt_scale));
        let eq152_e1927_d_b14: f64 = (eq152_e1924 * (s.db[252][14] * ddt_scale));
        let eq152_e1927_d_b15: f64 = (eq152_e1924 * (s.db[252][15] * ddt_scale));
        let eq152_e1927_d_b16: f64 = (eq152_e1924 * (s.db[252][16] * ddt_scale));
        let eq152_e1927_d_b17: f64 = (eq152_e1924 * (s.db[252][17] * ddt_scale));
        let eq152_e1927_d_b18: f64 = (eq152_e1924 * (s.db[252][18] * ddt_scale));
        let eq152_e1927_d_b19: f64 = (eq152_e1924 * (s.db[252][19] * ddt_scale));
        let eq152_e1927_d_b20: f64 = (eq152_e1924 * (s.db[252][20] * ddt_scale));
        let eq152_e1927_d_b21: f64 = (eq152_e1924 * (s.db[252][21] * ddt_scale));
        let eq152_e1927_d_b22: f64 = (eq152_e1924 * (s.db[252][22] * ddt_scale));
        let eq152_e1927_d_b23: f64 = (eq152_e1924 * (s.db[252][23] * ddt_scale));
        let eq152_e1927_d_b24: f64 = (eq152_e1924 * (s.db[252][24] * ddt_scale));
        let eq152_e1927_d_b25: f64 = (eq152_e1924 * (s.db[252][25] * ddt_scale));
        let eq152_e1927_d_b26: f64 = (eq152_e1924 * (s.db[252][26] * ddt_scale));
        let eq152_e1927_d_b27: f64 = (eq152_e1924 * (s.db[252][27] * ddt_scale));
        let eq152_e1927_d_b28: f64 = (eq152_e1924 * (s.db[252][28] * ddt_scale));
        let eq152_e1927_d_b29: f64 = (eq152_e1924 * (s.db[252][29] * ddt_scale));
        let eq152_e1927_d_b30: f64 = (eq152_e1924 * (s.db[252][30] * ddt_scale));
        let eq152_e1927_d_b31: f64 = (eq152_e1924 * (s.db[252][31] * ddt_scale));
        let eq152_e1927_d_b32: f64 = (eq152_e1924 * (s.db[252][32] * ddt_scale));
        let eq152_e1927_d_b33: f64 = (eq152_e1924 * (s.db[252][33] * ddt_scale));
        let eq152_e1927_d_b34: f64 = (eq152_e1924 * (s.db[252][34] * ddt_scale));
        let eq152_e1927_d_b35: f64 = (eq152_e1924 * (s.db[252][35] * ddt_scale));
        let eq152_e1927_d_b36: f64 = (eq152_e1924 * (s.db[252][36] * ddt_scale));
        let eq152_e1927_d_b37: f64 = (eq152_e1924 * (s.db[252][37] * ddt_scale));
        let eq152_e1927_d_b38: f64 = (eq152_e1924 * (s.db[252][38] * ddt_scale));
        let eq152_e1927_d_b39: f64 = (eq152_e1924 * (s.db[252][39] * ddt_scale));
        let eq152_e1927_d_b40: f64 = (eq152_e1924 * (s.db[252][40] * ddt_scale));
        let eq152_e1927_d_b41: f64 = (eq152_e1924 * (s.db[252][41] * ddt_scale));
        let eq152_e1927_d_b42: f64 = (eq152_e1924 * (s.db[252][42] * ddt_scale));
        let eq152_e1927_d_b43: f64 = (eq152_e1924 * (s.db[252][43] * ddt_scale));
        let eq152_e1927_d_b44: f64 = (eq152_e1924 * (s.db[252][44] * ddt_scale));
        let eq152_e1927_d_b45: f64 = (eq152_e1924 * (s.db[252][45] * ddt_scale));
        let eq152_e1927_d_b46: f64 = (eq152_e1924 * (s.db[252][46] * ddt_scale));
        let eq152_e1927_d_b47: f64 = (eq152_e1924 * (s.db[252][47] * ddt_scale));
        let eq152_e1927_d_b48: f64 = (eq152_e1924 * (s.db[252][48] * ddt_scale));
        let eq152_e1927_d_b49: f64 = (eq152_e1924 * (s.db[252][49] * ddt_scale));
        let eq152_e1927_d_b50: f64 = (eq152_e1924 * (s.db[252][50] * ddt_scale));
        let eq152_e1927_d_b51: f64 = (eq152_e1924 * (s.db[252][51] * ddt_scale));
        let eq152_e1927_d_b52: f64 = (eq152_e1924 * (s.db[252][52] * ddt_scale));
        let eq152_e1927_d_b53: f64 = (eq152_e1924 * (s.db[252][53] * ddt_scale));
        let eq152_e1927_d_b54: f64 = (eq152_e1924 * (s.db[252][54] * ddt_scale));
        (eq152_e1927, eq152_e1927_d_n0, eq152_e1927_d_n1, eq152_e1927_d_n2, eq152_e1927_d_n3, eq152_e1927_d_n4, eq152_e1927_d_n5, eq152_e1927_d_n6, eq152_e1927_d_n7, eq152_e1927_d_n8, eq152_e1927_d_n9, eq152_e1927_d_n10, eq152_e1927_d_n11, eq152_e1927_d_n12, eq152_e1927_d_n13, eq152_e1927_d_n14, eq152_e1927_d_n15, eq152_e1927_d_n16, eq152_e1927_d_n17, eq152_e1927_d_n18, eq152_e1927_d_n19, eq152_e1927_d_n20, eq152_e1927_d_n21, eq152_e1927_d_n22, eq152_e1927_d_b0, eq152_e1927_d_b1, eq152_e1927_d_b2, eq152_e1927_d_b3, eq152_e1927_d_b4, eq152_e1927_d_b5, eq152_e1927_d_b6, eq152_e1927_d_b7, eq152_e1927_d_b8, eq152_e1927_d_b9, eq152_e1927_d_b10, eq152_e1927_d_b11, eq152_e1927_d_b12, eq152_e1927_d_b13, eq152_e1927_d_b14, eq152_e1927_d_b15, eq152_e1927_d_b16, eq152_e1927_d_b17, eq152_e1927_d_b18, eq152_e1927_d_b19, eq152_e1927_d_b20, eq152_e1927_d_b21, eq152_e1927_d_b22, eq152_e1927_d_b23, eq152_e1927_d_b24, eq152_e1927_d_b25, eq152_e1927_d_b26, eq152_e1927_d_b27, eq152_e1927_d_b28, eq152_e1927_d_b29, eq152_e1927_d_b30, eq152_e1927_d_b31, eq152_e1927_d_b32, eq152_e1927_d_b33, eq152_e1927_d_b34, eq152_e1927_d_b35, eq152_e1927_d_b36, eq152_e1927_d_b37, eq152_e1927_d_b38, eq152_e1927_d_b39, eq152_e1927_d_b40, eq152_e1927_d_b41, eq152_e1927_d_b42, eq152_e1927_d_b43, eq152_e1927_d_b44, eq152_e1927_d_b45, eq152_e1927_d_b46, eq152_e1927_d_b47, eq152_e1927_d_b48, eq152_e1927_d_b49, eq152_e1927_d_b50, eq152_e1927_d_b51, eq152_e1927_d_b52, eq152_e1927_d_b53, eq152_e1927_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_value: f64 = eq152_e1929;
        let eq152_node_derivatives: [f64; 23] = [eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n10, eq152_e1929_d_n11, eq152_e1929_d_n12, eq152_e1929_d_n13, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22];
        let eq152_branch_derivatives: [f64; 55] = [eq152_e1929_d_b0, eq152_e1929_d_b1, eq152_e1929_d_b2, eq152_e1929_d_b3, eq152_e1929_d_b4, eq152_e1929_d_b5, eq152_e1929_d_b6, eq152_e1929_d_b7, eq152_e1929_d_b8, eq152_e1929_d_b9, eq152_e1929_d_b10, eq152_e1929_d_b11, eq152_e1929_d_b12, eq152_e1929_d_b13, eq152_e1929_d_b14, eq152_e1929_d_b15, eq152_e1929_d_b16, eq152_e1929_d_b17, eq152_e1929_d_b18, eq152_e1929_d_b19, eq152_e1929_d_b20, eq152_e1929_d_b21, eq152_e1929_d_b22, eq152_e1929_d_b23, eq152_e1929_d_b24, eq152_e1929_d_b25, eq152_e1929_d_b26, eq152_e1929_d_b27, eq152_e1929_d_b28, eq152_e1929_d_b29, eq152_e1929_d_b30, eq152_e1929_d_b31, eq152_e1929_d_b32, eq152_e1929_d_b33, eq152_e1929_d_b34, eq152_e1929_d_b35, eq152_e1929_d_b36, eq152_e1929_d_b37, eq152_e1929_d_b38, eq152_e1929_d_b39, eq152_e1929_d_b40, eq152_e1929_d_b41, eq152_e1929_d_b42, eq152_e1929_d_b43, eq152_e1929_d_b44, eq152_e1929_d_b45, eq152_e1929_d_b46, eq152_e1929_d_b47, eq152_e1929_d_b48, eq152_e1929_d_b49, eq152_e1929_d_b50, eq152_e1929_d_b51, eq152_e1929_d_b52, eq152_e1929_d_b53, eq152_e1929_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq152_value),
            &eq152_node_derivatives,
            &eq152_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_26(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq153_e1942, eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n10, eq153_e1942_d_n11, eq153_e1942_d_n12, eq153_e1942_d_n13, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22, eq153_e1942_d_b0, eq153_e1942_d_b1, eq153_e1942_d_b2, eq153_e1942_d_b3, eq153_e1942_d_b4, eq153_e1942_d_b5, eq153_e1942_d_b6, eq153_e1942_d_b7, eq153_e1942_d_b8, eq153_e1942_d_b9, eq153_e1942_d_b10, eq153_e1942_d_b11, eq153_e1942_d_b12, eq153_e1942_d_b13, eq153_e1942_d_b14, eq153_e1942_d_b15, eq153_e1942_d_b16, eq153_e1942_d_b17, eq153_e1942_d_b18, eq153_e1942_d_b19, eq153_e1942_d_b20, eq153_e1942_d_b21, eq153_e1942_d_b22, eq153_e1942_d_b23, eq153_e1942_d_b24, eq153_e1942_d_b25, eq153_e1942_d_b26, eq153_e1942_d_b27, eq153_e1942_d_b28, eq153_e1942_d_b29, eq153_e1942_d_b30, eq153_e1942_d_b31, eq153_e1942_d_b32, eq153_e1942_d_b33, eq153_e1942_d_b34, eq153_e1942_d_b35, eq153_e1942_d_b36, eq153_e1942_d_b37, eq153_e1942_d_b38, eq153_e1942_d_b39, eq153_e1942_d_b40, eq153_e1942_d_b41, eq153_e1942_d_b42, eq153_e1942_d_b43, eq153_e1942_d_b44, eq153_e1942_d_b45, eq153_e1942_d_b46, eq153_e1942_d_b47, eq153_e1942_d_b48, eq153_e1942_d_b49, eq153_e1942_d_b50, eq153_e1942_d_b51, eq153_e1942_d_b52, eq153_e1942_d_b53, eq153_e1942_d_b54,) = {
    if (((!s.b[580]) && s.b[583]) && (!s.b[584])) {
        let eq153_e1939: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 52, s.v[252]);
        let eq153_e1940: f64 = (p.p7 * eq153_e1939);
        let eq153_e1940_d_n0: f64 = (p.p7 * (s.dn[252][0] * ddt_scale));
        let eq153_e1940_d_n1: f64 = (p.p7 * (s.dn[252][1] * ddt_scale));
        let eq153_e1940_d_n2: f64 = (p.p7 * (s.dn[252][2] * ddt_scale));
        let eq153_e1940_d_n3: f64 = (p.p7 * (s.dn[252][3] * ddt_scale));
        let eq153_e1940_d_n4: f64 = (p.p7 * (s.dn[252][4] * ddt_scale));
        let eq153_e1940_d_n5: f64 = (p.p7 * (s.dn[252][5] * ddt_scale));
        let eq153_e1940_d_n6: f64 = (p.p7 * (s.dn[252][6] * ddt_scale));
        let eq153_e1940_d_n7: f64 = (p.p7 * (s.dn[252][7] * ddt_scale));
        let eq153_e1940_d_n8: f64 = (p.p7 * (s.dn[252][8] * ddt_scale));
        let eq153_e1940_d_n9: f64 = (p.p7 * (s.dn[252][9] * ddt_scale));
        let eq153_e1940_d_n10: f64 = (p.p7 * (s.dn[252][10] * ddt_scale));
        let eq153_e1940_d_n11: f64 = (p.p7 * (s.dn[252][11] * ddt_scale));
        let eq153_e1940_d_n12: f64 = (p.p7 * (s.dn[252][12] * ddt_scale));
        let eq153_e1940_d_n13: f64 = (p.p7 * (s.dn[252][13] * ddt_scale));
        let eq153_e1940_d_n14: f64 = (p.p7 * (s.dn[252][14] * ddt_scale));
        let eq153_e1940_d_n15: f64 = (p.p7 * (s.dn[252][15] * ddt_scale));
        let eq153_e1940_d_n16: f64 = (p.p7 * (s.dn[252][16] * ddt_scale));
        let eq153_e1940_d_n17: f64 = (p.p7 * (s.dn[252][17] * ddt_scale));
        let eq153_e1940_d_n18: f64 = (p.p7 * (s.dn[252][18] * ddt_scale));
        let eq153_e1940_d_n19: f64 = (p.p7 * (s.dn[252][19] * ddt_scale));
        let eq153_e1940_d_n20: f64 = (p.p7 * (s.dn[252][20] * ddt_scale));
        let eq153_e1940_d_n21: f64 = (p.p7 * (s.dn[252][21] * ddt_scale));
        let eq153_e1940_d_n22: f64 = (p.p7 * (s.dn[252][22] * ddt_scale));
        let eq153_e1940_d_b0: f64 = (p.p7 * (s.db[252][0] * ddt_scale));
        let eq153_e1940_d_b1: f64 = (p.p7 * (s.db[252][1] * ddt_scale));
        let eq153_e1940_d_b2: f64 = (p.p7 * (s.db[252][2] * ddt_scale));
        let eq153_e1940_d_b3: f64 = (p.p7 * (s.db[252][3] * ddt_scale));
        let eq153_e1940_d_b4: f64 = (p.p7 * (s.db[252][4] * ddt_scale));
        let eq153_e1940_d_b5: f64 = (p.p7 * (s.db[252][5] * ddt_scale));
        let eq153_e1940_d_b6: f64 = (p.p7 * (s.db[252][6] * ddt_scale));
        let eq153_e1940_d_b7: f64 = (p.p7 * (s.db[252][7] * ddt_scale));
        let eq153_e1940_d_b8: f64 = (p.p7 * (s.db[252][8] * ddt_scale));
        let eq153_e1940_d_b9: f64 = (p.p7 * (s.db[252][9] * ddt_scale));
        let eq153_e1940_d_b10: f64 = (p.p7 * (s.db[252][10] * ddt_scale));
        let eq153_e1940_d_b11: f64 = (p.p7 * (s.db[252][11] * ddt_scale));
        let eq153_e1940_d_b12: f64 = (p.p7 * (s.db[252][12] * ddt_scale));
        let eq153_e1940_d_b13: f64 = (p.p7 * (s.db[252][13] * ddt_scale));
        let eq153_e1940_d_b14: f64 = (p.p7 * (s.db[252][14] * ddt_scale));
        let eq153_e1940_d_b15: f64 = (p.p7 * (s.db[252][15] * ddt_scale));
        let eq153_e1940_d_b16: f64 = (p.p7 * (s.db[252][16] * ddt_scale));
        let eq153_e1940_d_b17: f64 = (p.p7 * (s.db[252][17] * ddt_scale));
        let eq153_e1940_d_b18: f64 = (p.p7 * (s.db[252][18] * ddt_scale));
        let eq153_e1940_d_b19: f64 = (p.p7 * (s.db[252][19] * ddt_scale));
        let eq153_e1940_d_b20: f64 = (p.p7 * (s.db[252][20] * ddt_scale));
        let eq153_e1940_d_b21: f64 = (p.p7 * (s.db[252][21] * ddt_scale));
        let eq153_e1940_d_b22: f64 = (p.p7 * (s.db[252][22] * ddt_scale));
        let eq153_e1940_d_b23: f64 = (p.p7 * (s.db[252][23] * ddt_scale));
        let eq153_e1940_d_b24: f64 = (p.p7 * (s.db[252][24] * ddt_scale));
        let eq153_e1940_d_b25: f64 = (p.p7 * (s.db[252][25] * ddt_scale));
        let eq153_e1940_d_b26: f64 = (p.p7 * (s.db[252][26] * ddt_scale));
        let eq153_e1940_d_b27: f64 = (p.p7 * (s.db[252][27] * ddt_scale));
        let eq153_e1940_d_b28: f64 = (p.p7 * (s.db[252][28] * ddt_scale));
        let eq153_e1940_d_b29: f64 = (p.p7 * (s.db[252][29] * ddt_scale));
        let eq153_e1940_d_b30: f64 = (p.p7 * (s.db[252][30] * ddt_scale));
        let eq153_e1940_d_b31: f64 = (p.p7 * (s.db[252][31] * ddt_scale));
        let eq153_e1940_d_b32: f64 = (p.p7 * (s.db[252][32] * ddt_scale));
        let eq153_e1940_d_b33: f64 = (p.p7 * (s.db[252][33] * ddt_scale));
        let eq153_e1940_d_b34: f64 = (p.p7 * (s.db[252][34] * ddt_scale));
        let eq153_e1940_d_b35: f64 = (p.p7 * (s.db[252][35] * ddt_scale));
        let eq153_e1940_d_b36: f64 = (p.p7 * (s.db[252][36] * ddt_scale));
        let eq153_e1940_d_b37: f64 = (p.p7 * (s.db[252][37] * ddt_scale));
        let eq153_e1940_d_b38: f64 = (p.p7 * (s.db[252][38] * ddt_scale));
        let eq153_e1940_d_b39: f64 = (p.p7 * (s.db[252][39] * ddt_scale));
        let eq153_e1940_d_b40: f64 = (p.p7 * (s.db[252][40] * ddt_scale));
        let eq153_e1940_d_b41: f64 = (p.p7 * (s.db[252][41] * ddt_scale));
        let eq153_e1940_d_b42: f64 = (p.p7 * (s.db[252][42] * ddt_scale));
        let eq153_e1940_d_b43: f64 = (p.p7 * (s.db[252][43] * ddt_scale));
        let eq153_e1940_d_b44: f64 = (p.p7 * (s.db[252][44] * ddt_scale));
        let eq153_e1940_d_b45: f64 = (p.p7 * (s.db[252][45] * ddt_scale));
        let eq153_e1940_d_b46: f64 = (p.p7 * (s.db[252][46] * ddt_scale));
        let eq153_e1940_d_b47: f64 = (p.p7 * (s.db[252][47] * ddt_scale));
        let eq153_e1940_d_b48: f64 = (p.p7 * (s.db[252][48] * ddt_scale));
        let eq153_e1940_d_b49: f64 = (p.p7 * (s.db[252][49] * ddt_scale));
        let eq153_e1940_d_b50: f64 = (p.p7 * (s.db[252][50] * ddt_scale));
        let eq153_e1940_d_b51: f64 = (p.p7 * (s.db[252][51] * ddt_scale));
        let eq153_e1940_d_b52: f64 = (p.p7 * (s.db[252][52] * ddt_scale));
        let eq153_e1940_d_b53: f64 = (p.p7 * (s.db[252][53] * ddt_scale));
        let eq153_e1940_d_b54: f64 = (p.p7 * (s.db[252][54] * ddt_scale));
        (eq153_e1940, eq153_e1940_d_n0, eq153_e1940_d_n1, eq153_e1940_d_n2, eq153_e1940_d_n3, eq153_e1940_d_n4, eq153_e1940_d_n5, eq153_e1940_d_n6, eq153_e1940_d_n7, eq153_e1940_d_n8, eq153_e1940_d_n9, eq153_e1940_d_n10, eq153_e1940_d_n11, eq153_e1940_d_n12, eq153_e1940_d_n13, eq153_e1940_d_n14, eq153_e1940_d_n15, eq153_e1940_d_n16, eq153_e1940_d_n17, eq153_e1940_d_n18, eq153_e1940_d_n19, eq153_e1940_d_n20, eq153_e1940_d_n21, eq153_e1940_d_n22, eq153_e1940_d_b0, eq153_e1940_d_b1, eq153_e1940_d_b2, eq153_e1940_d_b3, eq153_e1940_d_b4, eq153_e1940_d_b5, eq153_e1940_d_b6, eq153_e1940_d_b7, eq153_e1940_d_b8, eq153_e1940_d_b9, eq153_e1940_d_b10, eq153_e1940_d_b11, eq153_e1940_d_b12, eq153_e1940_d_b13, eq153_e1940_d_b14, eq153_e1940_d_b15, eq153_e1940_d_b16, eq153_e1940_d_b17, eq153_e1940_d_b18, eq153_e1940_d_b19, eq153_e1940_d_b20, eq153_e1940_d_b21, eq153_e1940_d_b22, eq153_e1940_d_b23, eq153_e1940_d_b24, eq153_e1940_d_b25, eq153_e1940_d_b26, eq153_e1940_d_b27, eq153_e1940_d_b28, eq153_e1940_d_b29, eq153_e1940_d_b30, eq153_e1940_d_b31, eq153_e1940_d_b32, eq153_e1940_d_b33, eq153_e1940_d_b34, eq153_e1940_d_b35, eq153_e1940_d_b36, eq153_e1940_d_b37, eq153_e1940_d_b38, eq153_e1940_d_b39, eq153_e1940_d_b40, eq153_e1940_d_b41, eq153_e1940_d_b42, eq153_e1940_d_b43, eq153_e1940_d_b44, eq153_e1940_d_b45, eq153_e1940_d_b46, eq153_e1940_d_b47, eq153_e1940_d_b48, eq153_e1940_d_b49, eq153_e1940_d_b50, eq153_e1940_d_b51, eq153_e1940_d_b52, eq153_e1940_d_b53, eq153_e1940_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_value: f64 = eq153_e1942;
        let eq153_node_derivatives: [f64; 23] = [eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n10, eq153_e1942_d_n11, eq153_e1942_d_n12, eq153_e1942_d_n13, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22];
        let eq153_branch_derivatives: [f64; 55] = [eq153_e1942_d_b0, eq153_e1942_d_b1, eq153_e1942_d_b2, eq153_e1942_d_b3, eq153_e1942_d_b4, eq153_e1942_d_b5, eq153_e1942_d_b6, eq153_e1942_d_b7, eq153_e1942_d_b8, eq153_e1942_d_b9, eq153_e1942_d_b10, eq153_e1942_d_b11, eq153_e1942_d_b12, eq153_e1942_d_b13, eq153_e1942_d_b14, eq153_e1942_d_b15, eq153_e1942_d_b16, eq153_e1942_d_b17, eq153_e1942_d_b18, eq153_e1942_d_b19, eq153_e1942_d_b20, eq153_e1942_d_b21, eq153_e1942_d_b22, eq153_e1942_d_b23, eq153_e1942_d_b24, eq153_e1942_d_b25, eq153_e1942_d_b26, eq153_e1942_d_b27, eq153_e1942_d_b28, eq153_e1942_d_b29, eq153_e1942_d_b30, eq153_e1942_d_b31, eq153_e1942_d_b32, eq153_e1942_d_b33, eq153_e1942_d_b34, eq153_e1942_d_b35, eq153_e1942_d_b36, eq153_e1942_d_b37, eq153_e1942_d_b38, eq153_e1942_d_b39, eq153_e1942_d_b40, eq153_e1942_d_b41, eq153_e1942_d_b42, eq153_e1942_d_b43, eq153_e1942_d_b44, eq153_e1942_d_b45, eq153_e1942_d_b46, eq153_e1942_d_b47, eq153_e1942_d_b48, eq153_e1942_d_b49, eq153_e1942_d_b50, eq153_e1942_d_b51, eq153_e1942_d_b52, eq153_e1942_d_b53, eq153_e1942_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq153_value),
            &eq153_node_derivatives,
            &eq153_branch_derivatives,
            multiplicity,
        );
        let (eq154_e1957, eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n10, eq154_e1957_d_n11, eq154_e1957_d_n12, eq154_e1957_d_n13, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22, eq154_e1957_d_b0, eq154_e1957_d_b1, eq154_e1957_d_b2, eq154_e1957_d_b3, eq154_e1957_d_b4, eq154_e1957_d_b5, eq154_e1957_d_b6, eq154_e1957_d_b7, eq154_e1957_d_b8, eq154_e1957_d_b9, eq154_e1957_d_b10, eq154_e1957_d_b11, eq154_e1957_d_b12, eq154_e1957_d_b13, eq154_e1957_d_b14, eq154_e1957_d_b15, eq154_e1957_d_b16, eq154_e1957_d_b17, eq154_e1957_d_b18, eq154_e1957_d_b19, eq154_e1957_d_b20, eq154_e1957_d_b21, eq154_e1957_d_b22, eq154_e1957_d_b23, eq154_e1957_d_b24, eq154_e1957_d_b25, eq154_e1957_d_b26, eq154_e1957_d_b27, eq154_e1957_d_b28, eq154_e1957_d_b29, eq154_e1957_d_b30, eq154_e1957_d_b31, eq154_e1957_d_b32, eq154_e1957_d_b33, eq154_e1957_d_b34, eq154_e1957_d_b35, eq154_e1957_d_b36, eq154_e1957_d_b37, eq154_e1957_d_b38, eq154_e1957_d_b39, eq154_e1957_d_b40, eq154_e1957_d_b41, eq154_e1957_d_b42, eq154_e1957_d_b43, eq154_e1957_d_b44, eq154_e1957_d_b45, eq154_e1957_d_b46, eq154_e1957_d_b47, eq154_e1957_d_b48, eq154_e1957_d_b49, eq154_e1957_d_b50, eq154_e1957_d_b51, eq154_e1957_d_b52, eq154_e1957_d_b53, eq154_e1957_d_b54,) = {
    if (((!s.b[580]) && s.b[583]) && (!s.b[584])) {
        let eq154_e1952: f64 = (p.p7 * p.p247);
        let eq154_e1954: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 53, s.v[252]);
        let eq154_e1955: f64 = (eq154_e1952 * eq154_e1954);
        let eq154_e1955_d_n0: f64 = (eq154_e1952 * (s.dn[252][0] * ddt_scale));
        let eq154_e1955_d_n1: f64 = (eq154_e1952 * (s.dn[252][1] * ddt_scale));
        let eq154_e1955_d_n2: f64 = (eq154_e1952 * (s.dn[252][2] * ddt_scale));
        let eq154_e1955_d_n3: f64 = (eq154_e1952 * (s.dn[252][3] * ddt_scale));
        let eq154_e1955_d_n4: f64 = (eq154_e1952 * (s.dn[252][4] * ddt_scale));
        let eq154_e1955_d_n5: f64 = (eq154_e1952 * (s.dn[252][5] * ddt_scale));
        let eq154_e1955_d_n6: f64 = (eq154_e1952 * (s.dn[252][6] * ddt_scale));
        let eq154_e1955_d_n7: f64 = (eq154_e1952 * (s.dn[252][7] * ddt_scale));
        let eq154_e1955_d_n8: f64 = (eq154_e1952 * (s.dn[252][8] * ddt_scale));
        let eq154_e1955_d_n9: f64 = (eq154_e1952 * (s.dn[252][9] * ddt_scale));
        let eq154_e1955_d_n10: f64 = (eq154_e1952 * (s.dn[252][10] * ddt_scale));
        let eq154_e1955_d_n11: f64 = (eq154_e1952 * (s.dn[252][11] * ddt_scale));
        let eq154_e1955_d_n12: f64 = (eq154_e1952 * (s.dn[252][12] * ddt_scale));
        let eq154_e1955_d_n13: f64 = (eq154_e1952 * (s.dn[252][13] * ddt_scale));
        let eq154_e1955_d_n14: f64 = (eq154_e1952 * (s.dn[252][14] * ddt_scale));
        let eq154_e1955_d_n15: f64 = (eq154_e1952 * (s.dn[252][15] * ddt_scale));
        let eq154_e1955_d_n16: f64 = (eq154_e1952 * (s.dn[252][16] * ddt_scale));
        let eq154_e1955_d_n17: f64 = (eq154_e1952 * (s.dn[252][17] * ddt_scale));
        let eq154_e1955_d_n18: f64 = (eq154_e1952 * (s.dn[252][18] * ddt_scale));
        let eq154_e1955_d_n19: f64 = (eq154_e1952 * (s.dn[252][19] * ddt_scale));
        let eq154_e1955_d_n20: f64 = (eq154_e1952 * (s.dn[252][20] * ddt_scale));
        let eq154_e1955_d_n21: f64 = (eq154_e1952 * (s.dn[252][21] * ddt_scale));
        let eq154_e1955_d_n22: f64 = (eq154_e1952 * (s.dn[252][22] * ddt_scale));
        let eq154_e1955_d_b0: f64 = (eq154_e1952 * (s.db[252][0] * ddt_scale));
        let eq154_e1955_d_b1: f64 = (eq154_e1952 * (s.db[252][1] * ddt_scale));
        let eq154_e1955_d_b2: f64 = (eq154_e1952 * (s.db[252][2] * ddt_scale));
        let eq154_e1955_d_b3: f64 = (eq154_e1952 * (s.db[252][3] * ddt_scale));
        let eq154_e1955_d_b4: f64 = (eq154_e1952 * (s.db[252][4] * ddt_scale));
        let eq154_e1955_d_b5: f64 = (eq154_e1952 * (s.db[252][5] * ddt_scale));
        let eq154_e1955_d_b6: f64 = (eq154_e1952 * (s.db[252][6] * ddt_scale));
        let eq154_e1955_d_b7: f64 = (eq154_e1952 * (s.db[252][7] * ddt_scale));
        let eq154_e1955_d_b8: f64 = (eq154_e1952 * (s.db[252][8] * ddt_scale));
        let eq154_e1955_d_b9: f64 = (eq154_e1952 * (s.db[252][9] * ddt_scale));
        let eq154_e1955_d_b10: f64 = (eq154_e1952 * (s.db[252][10] * ddt_scale));
        let eq154_e1955_d_b11: f64 = (eq154_e1952 * (s.db[252][11] * ddt_scale));
        let eq154_e1955_d_b12: f64 = (eq154_e1952 * (s.db[252][12] * ddt_scale));
        let eq154_e1955_d_b13: f64 = (eq154_e1952 * (s.db[252][13] * ddt_scale));
        let eq154_e1955_d_b14: f64 = (eq154_e1952 * (s.db[252][14] * ddt_scale));
        let eq154_e1955_d_b15: f64 = (eq154_e1952 * (s.db[252][15] * ddt_scale));
        let eq154_e1955_d_b16: f64 = (eq154_e1952 * (s.db[252][16] * ddt_scale));
        let eq154_e1955_d_b17: f64 = (eq154_e1952 * (s.db[252][17] * ddt_scale));
        let eq154_e1955_d_b18: f64 = (eq154_e1952 * (s.db[252][18] * ddt_scale));
        let eq154_e1955_d_b19: f64 = (eq154_e1952 * (s.db[252][19] * ddt_scale));
        let eq154_e1955_d_b20: f64 = (eq154_e1952 * (s.db[252][20] * ddt_scale));
        let eq154_e1955_d_b21: f64 = (eq154_e1952 * (s.db[252][21] * ddt_scale));
        let eq154_e1955_d_b22: f64 = (eq154_e1952 * (s.db[252][22] * ddt_scale));
        let eq154_e1955_d_b23: f64 = (eq154_e1952 * (s.db[252][23] * ddt_scale));
        let eq154_e1955_d_b24: f64 = (eq154_e1952 * (s.db[252][24] * ddt_scale));
        let eq154_e1955_d_b25: f64 = (eq154_e1952 * (s.db[252][25] * ddt_scale));
        let eq154_e1955_d_b26: f64 = (eq154_e1952 * (s.db[252][26] * ddt_scale));
        let eq154_e1955_d_b27: f64 = (eq154_e1952 * (s.db[252][27] * ddt_scale));
        let eq154_e1955_d_b28: f64 = (eq154_e1952 * (s.db[252][28] * ddt_scale));
        let eq154_e1955_d_b29: f64 = (eq154_e1952 * (s.db[252][29] * ddt_scale));
        let eq154_e1955_d_b30: f64 = (eq154_e1952 * (s.db[252][30] * ddt_scale));
        let eq154_e1955_d_b31: f64 = (eq154_e1952 * (s.db[252][31] * ddt_scale));
        let eq154_e1955_d_b32: f64 = (eq154_e1952 * (s.db[252][32] * ddt_scale));
        let eq154_e1955_d_b33: f64 = (eq154_e1952 * (s.db[252][33] * ddt_scale));
        let eq154_e1955_d_b34: f64 = (eq154_e1952 * (s.db[252][34] * ddt_scale));
        let eq154_e1955_d_b35: f64 = (eq154_e1952 * (s.db[252][35] * ddt_scale));
        let eq154_e1955_d_b36: f64 = (eq154_e1952 * (s.db[252][36] * ddt_scale));
        let eq154_e1955_d_b37: f64 = (eq154_e1952 * (s.db[252][37] * ddt_scale));
        let eq154_e1955_d_b38: f64 = (eq154_e1952 * (s.db[252][38] * ddt_scale));
        let eq154_e1955_d_b39: f64 = (eq154_e1952 * (s.db[252][39] * ddt_scale));
        let eq154_e1955_d_b40: f64 = (eq154_e1952 * (s.db[252][40] * ddt_scale));
        let eq154_e1955_d_b41: f64 = (eq154_e1952 * (s.db[252][41] * ddt_scale));
        let eq154_e1955_d_b42: f64 = (eq154_e1952 * (s.db[252][42] * ddt_scale));
        let eq154_e1955_d_b43: f64 = (eq154_e1952 * (s.db[252][43] * ddt_scale));
        let eq154_e1955_d_b44: f64 = (eq154_e1952 * (s.db[252][44] * ddt_scale));
        let eq154_e1955_d_b45: f64 = (eq154_e1952 * (s.db[252][45] * ddt_scale));
        let eq154_e1955_d_b46: f64 = (eq154_e1952 * (s.db[252][46] * ddt_scale));
        let eq154_e1955_d_b47: f64 = (eq154_e1952 * (s.db[252][47] * ddt_scale));
        let eq154_e1955_d_b48: f64 = (eq154_e1952 * (s.db[252][48] * ddt_scale));
        let eq154_e1955_d_b49: f64 = (eq154_e1952 * (s.db[252][49] * ddt_scale));
        let eq154_e1955_d_b50: f64 = (eq154_e1952 * (s.db[252][50] * ddt_scale));
        let eq154_e1955_d_b51: f64 = (eq154_e1952 * (s.db[252][51] * ddt_scale));
        let eq154_e1955_d_b52: f64 = (eq154_e1952 * (s.db[252][52] * ddt_scale));
        let eq154_e1955_d_b53: f64 = (eq154_e1952 * (s.db[252][53] * ddt_scale));
        let eq154_e1955_d_b54: f64 = (eq154_e1952 * (s.db[252][54] * ddt_scale));
        (eq154_e1955, eq154_e1955_d_n0, eq154_e1955_d_n1, eq154_e1955_d_n2, eq154_e1955_d_n3, eq154_e1955_d_n4, eq154_e1955_d_n5, eq154_e1955_d_n6, eq154_e1955_d_n7, eq154_e1955_d_n8, eq154_e1955_d_n9, eq154_e1955_d_n10, eq154_e1955_d_n11, eq154_e1955_d_n12, eq154_e1955_d_n13, eq154_e1955_d_n14, eq154_e1955_d_n15, eq154_e1955_d_n16, eq154_e1955_d_n17, eq154_e1955_d_n18, eq154_e1955_d_n19, eq154_e1955_d_n20, eq154_e1955_d_n21, eq154_e1955_d_n22, eq154_e1955_d_b0, eq154_e1955_d_b1, eq154_e1955_d_b2, eq154_e1955_d_b3, eq154_e1955_d_b4, eq154_e1955_d_b5, eq154_e1955_d_b6, eq154_e1955_d_b7, eq154_e1955_d_b8, eq154_e1955_d_b9, eq154_e1955_d_b10, eq154_e1955_d_b11, eq154_e1955_d_b12, eq154_e1955_d_b13, eq154_e1955_d_b14, eq154_e1955_d_b15, eq154_e1955_d_b16, eq154_e1955_d_b17, eq154_e1955_d_b18, eq154_e1955_d_b19, eq154_e1955_d_b20, eq154_e1955_d_b21, eq154_e1955_d_b22, eq154_e1955_d_b23, eq154_e1955_d_b24, eq154_e1955_d_b25, eq154_e1955_d_b26, eq154_e1955_d_b27, eq154_e1955_d_b28, eq154_e1955_d_b29, eq154_e1955_d_b30, eq154_e1955_d_b31, eq154_e1955_d_b32, eq154_e1955_d_b33, eq154_e1955_d_b34, eq154_e1955_d_b35, eq154_e1955_d_b36, eq154_e1955_d_b37, eq154_e1955_d_b38, eq154_e1955_d_b39, eq154_e1955_d_b40, eq154_e1955_d_b41, eq154_e1955_d_b42, eq154_e1955_d_b43, eq154_e1955_d_b44, eq154_e1955_d_b45, eq154_e1955_d_b46, eq154_e1955_d_b47, eq154_e1955_d_b48, eq154_e1955_d_b49, eq154_e1955_d_b50, eq154_e1955_d_b51, eq154_e1955_d_b52, eq154_e1955_d_b53, eq154_e1955_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_value: f64 = eq154_e1957;
        let eq154_node_derivatives: [f64; 23] = [eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n10, eq154_e1957_d_n11, eq154_e1957_d_n12, eq154_e1957_d_n13, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22];
        let eq154_branch_derivatives: [f64; 55] = [eq154_e1957_d_b0, eq154_e1957_d_b1, eq154_e1957_d_b2, eq154_e1957_d_b3, eq154_e1957_d_b4, eq154_e1957_d_b5, eq154_e1957_d_b6, eq154_e1957_d_b7, eq154_e1957_d_b8, eq154_e1957_d_b9, eq154_e1957_d_b10, eq154_e1957_d_b11, eq154_e1957_d_b12, eq154_e1957_d_b13, eq154_e1957_d_b14, eq154_e1957_d_b15, eq154_e1957_d_b16, eq154_e1957_d_b17, eq154_e1957_d_b18, eq154_e1957_d_b19, eq154_e1957_d_b20, eq154_e1957_d_b21, eq154_e1957_d_b22, eq154_e1957_d_b23, eq154_e1957_d_b24, eq154_e1957_d_b25, eq154_e1957_d_b26, eq154_e1957_d_b27, eq154_e1957_d_b28, eq154_e1957_d_b29, eq154_e1957_d_b30, eq154_e1957_d_b31, eq154_e1957_d_b32, eq154_e1957_d_b33, eq154_e1957_d_b34, eq154_e1957_d_b35, eq154_e1957_d_b36, eq154_e1957_d_b37, eq154_e1957_d_b38, eq154_e1957_d_b39, eq154_e1957_d_b40, eq154_e1957_d_b41, eq154_e1957_d_b42, eq154_e1957_d_b43, eq154_e1957_d_b44, eq154_e1957_d_b45, eq154_e1957_d_b46, eq154_e1957_d_b47, eq154_e1957_d_b48, eq154_e1957_d_b49, eq154_e1957_d_b50, eq154_e1957_d_b51, eq154_e1957_d_b52, eq154_e1957_d_b53, eq154_e1957_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq154_value),
            &eq154_node_derivatives,
            &eq154_branch_derivatives,
            multiplicity,
        );
        let (eq155_e1969, eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n10, eq155_e1969_d_n11, eq155_e1969_d_n12, eq155_e1969_d_n13, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22, eq155_e1969_d_b0, eq155_e1969_d_b1, eq155_e1969_d_b2, eq155_e1969_d_b3, eq155_e1969_d_b4, eq155_e1969_d_b5, eq155_e1969_d_b6, eq155_e1969_d_b7, eq155_e1969_d_b8, eq155_e1969_d_b9, eq155_e1969_d_b10, eq155_e1969_d_b11, eq155_e1969_d_b12, eq155_e1969_d_b13, eq155_e1969_d_b14, eq155_e1969_d_b15, eq155_e1969_d_b16, eq155_e1969_d_b17, eq155_e1969_d_b18, eq155_e1969_d_b19, eq155_e1969_d_b20, eq155_e1969_d_b21, eq155_e1969_d_b22, eq155_e1969_d_b23, eq155_e1969_d_b24, eq155_e1969_d_b25, eq155_e1969_d_b26, eq155_e1969_d_b27, eq155_e1969_d_b28, eq155_e1969_d_b29, eq155_e1969_d_b30, eq155_e1969_d_b31, eq155_e1969_d_b32, eq155_e1969_d_b33, eq155_e1969_d_b34, eq155_e1969_d_b35, eq155_e1969_d_b36, eq155_e1969_d_b37, eq155_e1969_d_b38, eq155_e1969_d_b39, eq155_e1969_d_b40, eq155_e1969_d_b41, eq155_e1969_d_b42, eq155_e1969_d_b43, eq155_e1969_d_b44, eq155_e1969_d_b45, eq155_e1969_d_b46, eq155_e1969_d_b47, eq155_e1969_d_b48, eq155_e1969_d_b49, eq155_e1969_d_b50, eq155_e1969_d_b51, eq155_e1969_d_b52, eq155_e1969_d_b53, eq155_e1969_d_b54,) = {
    if ((!s.b[580]) && s.b[583]) {
        let eq155_e1965: f64 = (p.p252 * s.v[252]);
        let eq155_e1966: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 54, eq155_e1965);
        let eq155_e1966_d_n0: f64 = ((p.p252 * s.dn[252][0]) * ddt_scale);
        let eq155_e1966_d_n1: f64 = ((p.p252 * s.dn[252][1]) * ddt_scale);
        let eq155_e1966_d_n2: f64 = ((p.p252 * s.dn[252][2]) * ddt_scale);
        let eq155_e1966_d_n3: f64 = ((p.p252 * s.dn[252][3]) * ddt_scale);
        let eq155_e1966_d_n4: f64 = ((p.p252 * s.dn[252][4]) * ddt_scale);
        let eq155_e1966_d_n5: f64 = ((p.p252 * s.dn[252][5]) * ddt_scale);
        let eq155_e1966_d_n6: f64 = ((p.p252 * s.dn[252][6]) * ddt_scale);
        let eq155_e1966_d_n7: f64 = ((p.p252 * s.dn[252][7]) * ddt_scale);
        let eq155_e1966_d_n8: f64 = ((p.p252 * s.dn[252][8]) * ddt_scale);
        let eq155_e1966_d_n9: f64 = ((p.p252 * s.dn[252][9]) * ddt_scale);
        let eq155_e1966_d_n10: f64 = ((p.p252 * s.dn[252][10]) * ddt_scale);
        let eq155_e1966_d_n11: f64 = ((p.p252 * s.dn[252][11]) * ddt_scale);
        let eq155_e1966_d_n12: f64 = ((p.p252 * s.dn[252][12]) * ddt_scale);
        let eq155_e1966_d_n13: f64 = ((p.p252 * s.dn[252][13]) * ddt_scale);
        let eq155_e1966_d_n14: f64 = ((p.p252 * s.dn[252][14]) * ddt_scale);
        let eq155_e1966_d_n15: f64 = ((p.p252 * s.dn[252][15]) * ddt_scale);
        let eq155_e1966_d_n16: f64 = ((p.p252 * s.dn[252][16]) * ddt_scale);
        let eq155_e1966_d_n17: f64 = ((p.p252 * s.dn[252][17]) * ddt_scale);
        let eq155_e1966_d_n18: f64 = ((p.p252 * s.dn[252][18]) * ddt_scale);
        let eq155_e1966_d_n19: f64 = ((p.p252 * s.dn[252][19]) * ddt_scale);
        let eq155_e1966_d_n20: f64 = ((p.p252 * s.dn[252][20]) * ddt_scale);
        let eq155_e1966_d_n21: f64 = ((p.p252 * s.dn[252][21]) * ddt_scale);
        let eq155_e1966_d_n22: f64 = ((p.p252 * s.dn[252][22]) * ddt_scale);
        let eq155_e1966_d_b0: f64 = ((p.p252 * s.db[252][0]) * ddt_scale);
        let eq155_e1966_d_b1: f64 = ((p.p252 * s.db[252][1]) * ddt_scale);
        let eq155_e1966_d_b2: f64 = ((p.p252 * s.db[252][2]) * ddt_scale);
        let eq155_e1966_d_b3: f64 = ((p.p252 * s.db[252][3]) * ddt_scale);
        let eq155_e1966_d_b4: f64 = ((p.p252 * s.db[252][4]) * ddt_scale);
        let eq155_e1966_d_b5: f64 = ((p.p252 * s.db[252][5]) * ddt_scale);
        let eq155_e1966_d_b6: f64 = ((p.p252 * s.db[252][6]) * ddt_scale);
        let eq155_e1966_d_b7: f64 = ((p.p252 * s.db[252][7]) * ddt_scale);
        let eq155_e1966_d_b8: f64 = ((p.p252 * s.db[252][8]) * ddt_scale);
        let eq155_e1966_d_b9: f64 = ((p.p252 * s.db[252][9]) * ddt_scale);
        let eq155_e1966_d_b10: f64 = ((p.p252 * s.db[252][10]) * ddt_scale);
        let eq155_e1966_d_b11: f64 = ((p.p252 * s.db[252][11]) * ddt_scale);
        let eq155_e1966_d_b12: f64 = ((p.p252 * s.db[252][12]) * ddt_scale);
        let eq155_e1966_d_b13: f64 = ((p.p252 * s.db[252][13]) * ddt_scale);
        let eq155_e1966_d_b14: f64 = ((p.p252 * s.db[252][14]) * ddt_scale);
        let eq155_e1966_d_b15: f64 = ((p.p252 * s.db[252][15]) * ddt_scale);
        let eq155_e1966_d_b16: f64 = ((p.p252 * s.db[252][16]) * ddt_scale);
        let eq155_e1966_d_b17: f64 = ((p.p252 * s.db[252][17]) * ddt_scale);
        let eq155_e1966_d_b18: f64 = ((p.p252 * s.db[252][18]) * ddt_scale);
        let eq155_e1966_d_b19: f64 = ((p.p252 * s.db[252][19]) * ddt_scale);
        let eq155_e1966_d_b20: f64 = ((p.p252 * s.db[252][20]) * ddt_scale);
        let eq155_e1966_d_b21: f64 = ((p.p252 * s.db[252][21]) * ddt_scale);
        let eq155_e1966_d_b22: f64 = ((p.p252 * s.db[252][22]) * ddt_scale);
        let eq155_e1966_d_b23: f64 = ((p.p252 * s.db[252][23]) * ddt_scale);
        let eq155_e1966_d_b24: f64 = ((p.p252 * s.db[252][24]) * ddt_scale);
        let eq155_e1966_d_b25: f64 = ((p.p252 * s.db[252][25]) * ddt_scale);
        let eq155_e1966_d_b26: f64 = ((p.p252 * s.db[252][26]) * ddt_scale);
        let eq155_e1966_d_b27: f64 = ((p.p252 * s.db[252][27]) * ddt_scale);
        let eq155_e1966_d_b28: f64 = ((p.p252 * s.db[252][28]) * ddt_scale);
        let eq155_e1966_d_b29: f64 = ((p.p252 * s.db[252][29]) * ddt_scale);
        let eq155_e1966_d_b30: f64 = ((p.p252 * s.db[252][30]) * ddt_scale);
        let eq155_e1966_d_b31: f64 = ((p.p252 * s.db[252][31]) * ddt_scale);
        let eq155_e1966_d_b32: f64 = ((p.p252 * s.db[252][32]) * ddt_scale);
        let eq155_e1966_d_b33: f64 = ((p.p252 * s.db[252][33]) * ddt_scale);
        let eq155_e1966_d_b34: f64 = ((p.p252 * s.db[252][34]) * ddt_scale);
        let eq155_e1966_d_b35: f64 = ((p.p252 * s.db[252][35]) * ddt_scale);
        let eq155_e1966_d_b36: f64 = ((p.p252 * s.db[252][36]) * ddt_scale);
        let eq155_e1966_d_b37: f64 = ((p.p252 * s.db[252][37]) * ddt_scale);
        let eq155_e1966_d_b38: f64 = ((p.p252 * s.db[252][38]) * ddt_scale);
        let eq155_e1966_d_b39: f64 = ((p.p252 * s.db[252][39]) * ddt_scale);
        let eq155_e1966_d_b40: f64 = ((p.p252 * s.db[252][40]) * ddt_scale);
        let eq155_e1966_d_b41: f64 = ((p.p252 * s.db[252][41]) * ddt_scale);
        let eq155_e1966_d_b42: f64 = ((p.p252 * s.db[252][42]) * ddt_scale);
        let eq155_e1966_d_b43: f64 = ((p.p252 * s.db[252][43]) * ddt_scale);
        let eq155_e1966_d_b44: f64 = ((p.p252 * s.db[252][44]) * ddt_scale);
        let eq155_e1966_d_b45: f64 = ((p.p252 * s.db[252][45]) * ddt_scale);
        let eq155_e1966_d_b46: f64 = ((p.p252 * s.db[252][46]) * ddt_scale);
        let eq155_e1966_d_b47: f64 = ((p.p252 * s.db[252][47]) * ddt_scale);
        let eq155_e1966_d_b48: f64 = ((p.p252 * s.db[252][48]) * ddt_scale);
        let eq155_e1966_d_b49: f64 = ((p.p252 * s.db[252][49]) * ddt_scale);
        let eq155_e1966_d_b50: f64 = ((p.p252 * s.db[252][50]) * ddt_scale);
        let eq155_e1966_d_b51: f64 = ((p.p252 * s.db[252][51]) * ddt_scale);
        let eq155_e1966_d_b52: f64 = ((p.p252 * s.db[252][52]) * ddt_scale);
        let eq155_e1966_d_b53: f64 = ((p.p252 * s.db[252][53]) * ddt_scale);
        let eq155_e1966_d_b54: f64 = ((p.p252 * s.db[252][54]) * ddt_scale);
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
        let eq155_e1967_d_b0: f64 = (p.p7 * eq155_e1966_d_b0);
        let eq155_e1967_d_b1: f64 = (p.p7 * eq155_e1966_d_b1);
        let eq155_e1967_d_b2: f64 = (p.p7 * eq155_e1966_d_b2);
        let eq155_e1967_d_b3: f64 = (p.p7 * eq155_e1966_d_b3);
        let eq155_e1967_d_b4: f64 = (p.p7 * eq155_e1966_d_b4);
        let eq155_e1967_d_b5: f64 = (p.p7 * eq155_e1966_d_b5);
        let eq155_e1967_d_b6: f64 = (p.p7 * eq155_e1966_d_b6);
        let eq155_e1967_d_b7: f64 = (p.p7 * eq155_e1966_d_b7);
        let eq155_e1967_d_b8: f64 = (p.p7 * eq155_e1966_d_b8);
        let eq155_e1967_d_b9: f64 = (p.p7 * eq155_e1966_d_b9);
        let eq155_e1967_d_b10: f64 = (p.p7 * eq155_e1966_d_b10);
        let eq155_e1967_d_b11: f64 = (p.p7 * eq155_e1966_d_b11);
        let eq155_e1967_d_b12: f64 = (p.p7 * eq155_e1966_d_b12);
        let eq155_e1967_d_b13: f64 = (p.p7 * eq155_e1966_d_b13);
        let eq155_e1967_d_b14: f64 = (p.p7 * eq155_e1966_d_b14);
        let eq155_e1967_d_b15: f64 = (p.p7 * eq155_e1966_d_b15);
        let eq155_e1967_d_b16: f64 = (p.p7 * eq155_e1966_d_b16);
        let eq155_e1967_d_b17: f64 = (p.p7 * eq155_e1966_d_b17);
        let eq155_e1967_d_b18: f64 = (p.p7 * eq155_e1966_d_b18);
        let eq155_e1967_d_b19: f64 = (p.p7 * eq155_e1966_d_b19);
        let eq155_e1967_d_b20: f64 = (p.p7 * eq155_e1966_d_b20);
        let eq155_e1967_d_b21: f64 = (p.p7 * eq155_e1966_d_b21);
        let eq155_e1967_d_b22: f64 = (p.p7 * eq155_e1966_d_b22);
        let eq155_e1967_d_b23: f64 = (p.p7 * eq155_e1966_d_b23);
        let eq155_e1967_d_b24: f64 = (p.p7 * eq155_e1966_d_b24);
        let eq155_e1967_d_b25: f64 = (p.p7 * eq155_e1966_d_b25);
        let eq155_e1967_d_b26: f64 = (p.p7 * eq155_e1966_d_b26);
        let eq155_e1967_d_b27: f64 = (p.p7 * eq155_e1966_d_b27);
        let eq155_e1967_d_b28: f64 = (p.p7 * eq155_e1966_d_b28);
        let eq155_e1967_d_b29: f64 = (p.p7 * eq155_e1966_d_b29);
        let eq155_e1967_d_b30: f64 = (p.p7 * eq155_e1966_d_b30);
        let eq155_e1967_d_b31: f64 = (p.p7 * eq155_e1966_d_b31);
        let eq155_e1967_d_b32: f64 = (p.p7 * eq155_e1966_d_b32);
        let eq155_e1967_d_b33: f64 = (p.p7 * eq155_e1966_d_b33);
        let eq155_e1967_d_b34: f64 = (p.p7 * eq155_e1966_d_b34);
        let eq155_e1967_d_b35: f64 = (p.p7 * eq155_e1966_d_b35);
        let eq155_e1967_d_b36: f64 = (p.p7 * eq155_e1966_d_b36);
        let eq155_e1967_d_b37: f64 = (p.p7 * eq155_e1966_d_b37);
        let eq155_e1967_d_b38: f64 = (p.p7 * eq155_e1966_d_b38);
        let eq155_e1967_d_b39: f64 = (p.p7 * eq155_e1966_d_b39);
        let eq155_e1967_d_b40: f64 = (p.p7 * eq155_e1966_d_b40);
        let eq155_e1967_d_b41: f64 = (p.p7 * eq155_e1966_d_b41);
        let eq155_e1967_d_b42: f64 = (p.p7 * eq155_e1966_d_b42);
        let eq155_e1967_d_b43: f64 = (p.p7 * eq155_e1966_d_b43);
        let eq155_e1967_d_b44: f64 = (p.p7 * eq155_e1966_d_b44);
        let eq155_e1967_d_b45: f64 = (p.p7 * eq155_e1966_d_b45);
        let eq155_e1967_d_b46: f64 = (p.p7 * eq155_e1966_d_b46);
        let eq155_e1967_d_b47: f64 = (p.p7 * eq155_e1966_d_b47);
        let eq155_e1967_d_b48: f64 = (p.p7 * eq155_e1966_d_b48);
        let eq155_e1967_d_b49: f64 = (p.p7 * eq155_e1966_d_b49);
        let eq155_e1967_d_b50: f64 = (p.p7 * eq155_e1966_d_b50);
        let eq155_e1967_d_b51: f64 = (p.p7 * eq155_e1966_d_b51);
        let eq155_e1967_d_b52: f64 = (p.p7 * eq155_e1966_d_b52);
        let eq155_e1967_d_b53: f64 = (p.p7 * eq155_e1966_d_b53);
        let eq155_e1967_d_b54: f64 = (p.p7 * eq155_e1966_d_b54);
        (eq155_e1967, eq155_e1967_d_n0, eq155_e1967_d_n1, eq155_e1967_d_n2, eq155_e1967_d_n3, eq155_e1967_d_n4, eq155_e1967_d_n5, eq155_e1967_d_n6, eq155_e1967_d_n7, eq155_e1967_d_n8, eq155_e1967_d_n9, eq155_e1967_d_n10, eq155_e1967_d_n11, eq155_e1967_d_n12, eq155_e1967_d_n13, eq155_e1967_d_n14, eq155_e1967_d_n15, eq155_e1967_d_n16, eq155_e1967_d_n17, eq155_e1967_d_n18, eq155_e1967_d_n19, eq155_e1967_d_n20, eq155_e1967_d_n21, eq155_e1967_d_n22, eq155_e1967_d_b0, eq155_e1967_d_b1, eq155_e1967_d_b2, eq155_e1967_d_b3, eq155_e1967_d_b4, eq155_e1967_d_b5, eq155_e1967_d_b6, eq155_e1967_d_b7, eq155_e1967_d_b8, eq155_e1967_d_b9, eq155_e1967_d_b10, eq155_e1967_d_b11, eq155_e1967_d_b12, eq155_e1967_d_b13, eq155_e1967_d_b14, eq155_e1967_d_b15, eq155_e1967_d_b16, eq155_e1967_d_b17, eq155_e1967_d_b18, eq155_e1967_d_b19, eq155_e1967_d_b20, eq155_e1967_d_b21, eq155_e1967_d_b22, eq155_e1967_d_b23, eq155_e1967_d_b24, eq155_e1967_d_b25, eq155_e1967_d_b26, eq155_e1967_d_b27, eq155_e1967_d_b28, eq155_e1967_d_b29, eq155_e1967_d_b30, eq155_e1967_d_b31, eq155_e1967_d_b32, eq155_e1967_d_b33, eq155_e1967_d_b34, eq155_e1967_d_b35, eq155_e1967_d_b36, eq155_e1967_d_b37, eq155_e1967_d_b38, eq155_e1967_d_b39, eq155_e1967_d_b40, eq155_e1967_d_b41, eq155_e1967_d_b42, eq155_e1967_d_b43, eq155_e1967_d_b44, eq155_e1967_d_b45, eq155_e1967_d_b46, eq155_e1967_d_b47, eq155_e1967_d_b48, eq155_e1967_d_b49, eq155_e1967_d_b50, eq155_e1967_d_b51, eq155_e1967_d_b52, eq155_e1967_d_b53, eq155_e1967_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_value: f64 = eq155_e1969;
        let eq155_node_derivatives: [f64; 23] = [eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n10, eq155_e1969_d_n11, eq155_e1969_d_n12, eq155_e1969_d_n13, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22];
        let eq155_branch_derivatives: [f64; 55] = [eq155_e1969_d_b0, eq155_e1969_d_b1, eq155_e1969_d_b2, eq155_e1969_d_b3, eq155_e1969_d_b4, eq155_e1969_d_b5, eq155_e1969_d_b6, eq155_e1969_d_b7, eq155_e1969_d_b8, eq155_e1969_d_b9, eq155_e1969_d_b10, eq155_e1969_d_b11, eq155_e1969_d_b12, eq155_e1969_d_b13, eq155_e1969_d_b14, eq155_e1969_d_b15, eq155_e1969_d_b16, eq155_e1969_d_b17, eq155_e1969_d_b18, eq155_e1969_d_b19, eq155_e1969_d_b20, eq155_e1969_d_b21, eq155_e1969_d_b22, eq155_e1969_d_b23, eq155_e1969_d_b24, eq155_e1969_d_b25, eq155_e1969_d_b26, eq155_e1969_d_b27, eq155_e1969_d_b28, eq155_e1969_d_b29, eq155_e1969_d_b30, eq155_e1969_d_b31, eq155_e1969_d_b32, eq155_e1969_d_b33, eq155_e1969_d_b34, eq155_e1969_d_b35, eq155_e1969_d_b36, eq155_e1969_d_b37, eq155_e1969_d_b38, eq155_e1969_d_b39, eq155_e1969_d_b40, eq155_e1969_d_b41, eq155_e1969_d_b42, eq155_e1969_d_b43, eq155_e1969_d_b44, eq155_e1969_d_b45, eq155_e1969_d_b46, eq155_e1969_d_b47, eq155_e1969_d_b48, eq155_e1969_d_b49, eq155_e1969_d_b50, eq155_e1969_d_b51, eq155_e1969_d_b52, eq155_e1969_d_b53, eq155_e1969_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq155_value),
            &eq155_node_derivatives,
            &eq155_branch_derivatives,
            multiplicity,
        );
        let (eq156_e1978, eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n10, eq156_e1978_d_n11, eq156_e1978_d_n12, eq156_e1978_d_n13, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22, eq156_e1978_d_b0, eq156_e1978_d_b1, eq156_e1978_d_b2, eq156_e1978_d_b3, eq156_e1978_d_b4, eq156_e1978_d_b5, eq156_e1978_d_b6, eq156_e1978_d_b7, eq156_e1978_d_b8, eq156_e1978_d_b9, eq156_e1978_d_b10, eq156_e1978_d_b11, eq156_e1978_d_b12, eq156_e1978_d_b13, eq156_e1978_d_b14, eq156_e1978_d_b15, eq156_e1978_d_b16, eq156_e1978_d_b17, eq156_e1978_d_b18, eq156_e1978_d_b19, eq156_e1978_d_b20, eq156_e1978_d_b21, eq156_e1978_d_b22, eq156_e1978_d_b23, eq156_e1978_d_b24, eq156_e1978_d_b25, eq156_e1978_d_b26, eq156_e1978_d_b27, eq156_e1978_d_b28, eq156_e1978_d_b29, eq156_e1978_d_b30, eq156_e1978_d_b31, eq156_e1978_d_b32, eq156_e1978_d_b33, eq156_e1978_d_b34, eq156_e1978_d_b35, eq156_e1978_d_b36, eq156_e1978_d_b37, eq156_e1978_d_b38, eq156_e1978_d_b39, eq156_e1978_d_b40, eq156_e1978_d_b41, eq156_e1978_d_b42, eq156_e1978_d_b43, eq156_e1978_d_b44, eq156_e1978_d_b45, eq156_e1978_d_b46, eq156_e1978_d_b47, eq156_e1978_d_b48, eq156_e1978_d_b49, eq156_e1978_d_b50, eq156_e1978_d_b51, eq156_e1978_d_b52, eq156_e1978_d_b53, eq156_e1978_d_b54,) = {
    if (s.b[585] && s.b[586]) {
        let eq156_e1975: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 55, s.v[265]);
        let eq156_e1976: f64 = (p.p7 * eq156_e1975);
        let eq156_e1976_d_n0: f64 = (p.p7 * (s.dn[265][0] * ddt_scale));
        let eq156_e1976_d_n1: f64 = (p.p7 * (s.dn[265][1] * ddt_scale));
        let eq156_e1976_d_n2: f64 = (p.p7 * (s.dn[265][2] * ddt_scale));
        let eq156_e1976_d_n3: f64 = (p.p7 * (s.dn[265][3] * ddt_scale));
        let eq156_e1976_d_n4: f64 = (p.p7 * (s.dn[265][4] * ddt_scale));
        let eq156_e1976_d_n5: f64 = (p.p7 * (s.dn[265][5] * ddt_scale));
        let eq156_e1976_d_n6: f64 = (p.p7 * (s.dn[265][6] * ddt_scale));
        let eq156_e1976_d_n7: f64 = (p.p7 * (s.dn[265][7] * ddt_scale));
        let eq156_e1976_d_n8: f64 = (p.p7 * (s.dn[265][8] * ddt_scale));
        let eq156_e1976_d_n9: f64 = (p.p7 * (s.dn[265][9] * ddt_scale));
        let eq156_e1976_d_n10: f64 = (p.p7 * (s.dn[265][10] * ddt_scale));
        let eq156_e1976_d_n11: f64 = (p.p7 * (s.dn[265][11] * ddt_scale));
        let eq156_e1976_d_n12: f64 = (p.p7 * (s.dn[265][12] * ddt_scale));
        let eq156_e1976_d_n13: f64 = (p.p7 * (s.dn[265][13] * ddt_scale));
        let eq156_e1976_d_n14: f64 = (p.p7 * (s.dn[265][14] * ddt_scale));
        let eq156_e1976_d_n15: f64 = (p.p7 * (s.dn[265][15] * ddt_scale));
        let eq156_e1976_d_n16: f64 = (p.p7 * (s.dn[265][16] * ddt_scale));
        let eq156_e1976_d_n17: f64 = (p.p7 * (s.dn[265][17] * ddt_scale));
        let eq156_e1976_d_n18: f64 = (p.p7 * (s.dn[265][18] * ddt_scale));
        let eq156_e1976_d_n19: f64 = (p.p7 * (s.dn[265][19] * ddt_scale));
        let eq156_e1976_d_n20: f64 = (p.p7 * (s.dn[265][20] * ddt_scale));
        let eq156_e1976_d_n21: f64 = (p.p7 * (s.dn[265][21] * ddt_scale));
        let eq156_e1976_d_n22: f64 = (p.p7 * (s.dn[265][22] * ddt_scale));
        let eq156_e1976_d_b0: f64 = (p.p7 * (s.db[265][0] * ddt_scale));
        let eq156_e1976_d_b1: f64 = (p.p7 * (s.db[265][1] * ddt_scale));
        let eq156_e1976_d_b2: f64 = (p.p7 * (s.db[265][2] * ddt_scale));
        let eq156_e1976_d_b3: f64 = (p.p7 * (s.db[265][3] * ddt_scale));
        let eq156_e1976_d_b4: f64 = (p.p7 * (s.db[265][4] * ddt_scale));
        let eq156_e1976_d_b5: f64 = (p.p7 * (s.db[265][5] * ddt_scale));
        let eq156_e1976_d_b6: f64 = (p.p7 * (s.db[265][6] * ddt_scale));
        let eq156_e1976_d_b7: f64 = (p.p7 * (s.db[265][7] * ddt_scale));
        let eq156_e1976_d_b8: f64 = (p.p7 * (s.db[265][8] * ddt_scale));
        let eq156_e1976_d_b9: f64 = (p.p7 * (s.db[265][9] * ddt_scale));
        let eq156_e1976_d_b10: f64 = (p.p7 * (s.db[265][10] * ddt_scale));
        let eq156_e1976_d_b11: f64 = (p.p7 * (s.db[265][11] * ddt_scale));
        let eq156_e1976_d_b12: f64 = (p.p7 * (s.db[265][12] * ddt_scale));
        let eq156_e1976_d_b13: f64 = (p.p7 * (s.db[265][13] * ddt_scale));
        let eq156_e1976_d_b14: f64 = (p.p7 * (s.db[265][14] * ddt_scale));
        let eq156_e1976_d_b15: f64 = (p.p7 * (s.db[265][15] * ddt_scale));
        let eq156_e1976_d_b16: f64 = (p.p7 * (s.db[265][16] * ddt_scale));
        let eq156_e1976_d_b17: f64 = (p.p7 * (s.db[265][17] * ddt_scale));
        let eq156_e1976_d_b18: f64 = (p.p7 * (s.db[265][18] * ddt_scale));
        let eq156_e1976_d_b19: f64 = (p.p7 * (s.db[265][19] * ddt_scale));
        let eq156_e1976_d_b20: f64 = (p.p7 * (s.db[265][20] * ddt_scale));
        let eq156_e1976_d_b21: f64 = (p.p7 * (s.db[265][21] * ddt_scale));
        let eq156_e1976_d_b22: f64 = (p.p7 * (s.db[265][22] * ddt_scale));
        let eq156_e1976_d_b23: f64 = (p.p7 * (s.db[265][23] * ddt_scale));
        let eq156_e1976_d_b24: f64 = (p.p7 * (s.db[265][24] * ddt_scale));
        let eq156_e1976_d_b25: f64 = (p.p7 * (s.db[265][25] * ddt_scale));
        let eq156_e1976_d_b26: f64 = (p.p7 * (s.db[265][26] * ddt_scale));
        let eq156_e1976_d_b27: f64 = (p.p7 * (s.db[265][27] * ddt_scale));
        let eq156_e1976_d_b28: f64 = (p.p7 * (s.db[265][28] * ddt_scale));
        let eq156_e1976_d_b29: f64 = (p.p7 * (s.db[265][29] * ddt_scale));
        let eq156_e1976_d_b30: f64 = (p.p7 * (s.db[265][30] * ddt_scale));
        let eq156_e1976_d_b31: f64 = (p.p7 * (s.db[265][31] * ddt_scale));
        let eq156_e1976_d_b32: f64 = (p.p7 * (s.db[265][32] * ddt_scale));
        let eq156_e1976_d_b33: f64 = (p.p7 * (s.db[265][33] * ddt_scale));
        let eq156_e1976_d_b34: f64 = (p.p7 * (s.db[265][34] * ddt_scale));
        let eq156_e1976_d_b35: f64 = (p.p7 * (s.db[265][35] * ddt_scale));
        let eq156_e1976_d_b36: f64 = (p.p7 * (s.db[265][36] * ddt_scale));
        let eq156_e1976_d_b37: f64 = (p.p7 * (s.db[265][37] * ddt_scale));
        let eq156_e1976_d_b38: f64 = (p.p7 * (s.db[265][38] * ddt_scale));
        let eq156_e1976_d_b39: f64 = (p.p7 * (s.db[265][39] * ddt_scale));
        let eq156_e1976_d_b40: f64 = (p.p7 * (s.db[265][40] * ddt_scale));
        let eq156_e1976_d_b41: f64 = (p.p7 * (s.db[265][41] * ddt_scale));
        let eq156_e1976_d_b42: f64 = (p.p7 * (s.db[265][42] * ddt_scale));
        let eq156_e1976_d_b43: f64 = (p.p7 * (s.db[265][43] * ddt_scale));
        let eq156_e1976_d_b44: f64 = (p.p7 * (s.db[265][44] * ddt_scale));
        let eq156_e1976_d_b45: f64 = (p.p7 * (s.db[265][45] * ddt_scale));
        let eq156_e1976_d_b46: f64 = (p.p7 * (s.db[265][46] * ddt_scale));
        let eq156_e1976_d_b47: f64 = (p.p7 * (s.db[265][47] * ddt_scale));
        let eq156_e1976_d_b48: f64 = (p.p7 * (s.db[265][48] * ddt_scale));
        let eq156_e1976_d_b49: f64 = (p.p7 * (s.db[265][49] * ddt_scale));
        let eq156_e1976_d_b50: f64 = (p.p7 * (s.db[265][50] * ddt_scale));
        let eq156_e1976_d_b51: f64 = (p.p7 * (s.db[265][51] * ddt_scale));
        let eq156_e1976_d_b52: f64 = (p.p7 * (s.db[265][52] * ddt_scale));
        let eq156_e1976_d_b53: f64 = (p.p7 * (s.db[265][53] * ddt_scale));
        let eq156_e1976_d_b54: f64 = (p.p7 * (s.db[265][54] * ddt_scale));
        (eq156_e1976, eq156_e1976_d_n0, eq156_e1976_d_n1, eq156_e1976_d_n2, eq156_e1976_d_n3, eq156_e1976_d_n4, eq156_e1976_d_n5, eq156_e1976_d_n6, eq156_e1976_d_n7, eq156_e1976_d_n8, eq156_e1976_d_n9, eq156_e1976_d_n10, eq156_e1976_d_n11, eq156_e1976_d_n12, eq156_e1976_d_n13, eq156_e1976_d_n14, eq156_e1976_d_n15, eq156_e1976_d_n16, eq156_e1976_d_n17, eq156_e1976_d_n18, eq156_e1976_d_n19, eq156_e1976_d_n20, eq156_e1976_d_n21, eq156_e1976_d_n22, eq156_e1976_d_b0, eq156_e1976_d_b1, eq156_e1976_d_b2, eq156_e1976_d_b3, eq156_e1976_d_b4, eq156_e1976_d_b5, eq156_e1976_d_b6, eq156_e1976_d_b7, eq156_e1976_d_b8, eq156_e1976_d_b9, eq156_e1976_d_b10, eq156_e1976_d_b11, eq156_e1976_d_b12, eq156_e1976_d_b13, eq156_e1976_d_b14, eq156_e1976_d_b15, eq156_e1976_d_b16, eq156_e1976_d_b17, eq156_e1976_d_b18, eq156_e1976_d_b19, eq156_e1976_d_b20, eq156_e1976_d_b21, eq156_e1976_d_b22, eq156_e1976_d_b23, eq156_e1976_d_b24, eq156_e1976_d_b25, eq156_e1976_d_b26, eq156_e1976_d_b27, eq156_e1976_d_b28, eq156_e1976_d_b29, eq156_e1976_d_b30, eq156_e1976_d_b31, eq156_e1976_d_b32, eq156_e1976_d_b33, eq156_e1976_d_b34, eq156_e1976_d_b35, eq156_e1976_d_b36, eq156_e1976_d_b37, eq156_e1976_d_b38, eq156_e1976_d_b39, eq156_e1976_d_b40, eq156_e1976_d_b41, eq156_e1976_d_b42, eq156_e1976_d_b43, eq156_e1976_d_b44, eq156_e1976_d_b45, eq156_e1976_d_b46, eq156_e1976_d_b47, eq156_e1976_d_b48, eq156_e1976_d_b49, eq156_e1976_d_b50, eq156_e1976_d_b51, eq156_e1976_d_b52, eq156_e1976_d_b53, eq156_e1976_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_value: f64 = eq156_e1978;
        let eq156_node_derivatives: [f64; 23] = [eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n10, eq156_e1978_d_n11, eq156_e1978_d_n12, eq156_e1978_d_n13, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22];
        let eq156_branch_derivatives: [f64; 55] = [eq156_e1978_d_b0, eq156_e1978_d_b1, eq156_e1978_d_b2, eq156_e1978_d_b3, eq156_e1978_d_b4, eq156_e1978_d_b5, eq156_e1978_d_b6, eq156_e1978_d_b7, eq156_e1978_d_b8, eq156_e1978_d_b9, eq156_e1978_d_b10, eq156_e1978_d_b11, eq156_e1978_d_b12, eq156_e1978_d_b13, eq156_e1978_d_b14, eq156_e1978_d_b15, eq156_e1978_d_b16, eq156_e1978_d_b17, eq156_e1978_d_b18, eq156_e1978_d_b19, eq156_e1978_d_b20, eq156_e1978_d_b21, eq156_e1978_d_b22, eq156_e1978_d_b23, eq156_e1978_d_b24, eq156_e1978_d_b25, eq156_e1978_d_b26, eq156_e1978_d_b27, eq156_e1978_d_b28, eq156_e1978_d_b29, eq156_e1978_d_b30, eq156_e1978_d_b31, eq156_e1978_d_b32, eq156_e1978_d_b33, eq156_e1978_d_b34, eq156_e1978_d_b35, eq156_e1978_d_b36, eq156_e1978_d_b37, eq156_e1978_d_b38, eq156_e1978_d_b39, eq156_e1978_d_b40, eq156_e1978_d_b41, eq156_e1978_d_b42, eq156_e1978_d_b43, eq156_e1978_d_b44, eq156_e1978_d_b45, eq156_e1978_d_b46, eq156_e1978_d_b47, eq156_e1978_d_b48, eq156_e1978_d_b49, eq156_e1978_d_b50, eq156_e1978_d_b51, eq156_e1978_d_b52, eq156_e1978_d_b53, eq156_e1978_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(20),
            multiplicity * (eq156_value),
            &eq156_node_derivatives,
            &eq156_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_27(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[264][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[264][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[264][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[264][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[264][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[264][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[264][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[264][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[264][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[264][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[264][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[264][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[264][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[264][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[264][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[264][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[264][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[264][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[264][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[264][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[264][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[264][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[264][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[264][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[264][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[264][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[264][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[264][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[264][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[264][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[264][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[264][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[264][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[264][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[264][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[264][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[264][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[264][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[264][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[264][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[264][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[264][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[264][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[264][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[264][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[264][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[264][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[264][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[264][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[264][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[264][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[264][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[264][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[264][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[264][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[264][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[264][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[264][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[264][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[264][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[264][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[264][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[264][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[264][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[264][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[264][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[264][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[264][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[264][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[264][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[264][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[264][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[264][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[264][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[264][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[264][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[264][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[264][54] * ddt_scale));
        let (eq157_e1989, eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n10, eq157_e1989_d_n11, eq157_e1989_d_n12, eq157_e1989_d_n13, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22, eq157_e1989_d_b0, eq157_e1989_d_b1, eq157_e1989_d_b2, eq157_e1989_d_b3, eq157_e1989_d_b4, eq157_e1989_d_b5, eq157_e1989_d_b6, eq157_e1989_d_b7, eq157_e1989_d_b8, eq157_e1989_d_b9, eq157_e1989_d_b10, eq157_e1989_d_b11, eq157_e1989_d_b12, eq157_e1989_d_b13, eq157_e1989_d_b14, eq157_e1989_d_b15, eq157_e1989_d_b16, eq157_e1989_d_b17, eq157_e1989_d_b18, eq157_e1989_d_b19, eq157_e1989_d_b20, eq157_e1989_d_b21, eq157_e1989_d_b22, eq157_e1989_d_b23, eq157_e1989_d_b24, eq157_e1989_d_b25, eq157_e1989_d_b26, eq157_e1989_d_b27, eq157_e1989_d_b28, eq157_e1989_d_b29, eq157_e1989_d_b30, eq157_e1989_d_b31, eq157_e1989_d_b32, eq157_e1989_d_b33, eq157_e1989_d_b34, eq157_e1989_d_b35, eq157_e1989_d_b36, eq157_e1989_d_b37, eq157_e1989_d_b38, eq157_e1989_d_b39, eq157_e1989_d_b40, eq157_e1989_d_b41, eq157_e1989_d_b42, eq157_e1989_d_b43, eq157_e1989_d_b44, eq157_e1989_d_b45, eq157_e1989_d_b46, eq157_e1989_d_b47, eq157_e1989_d_b48, eq157_e1989_d_b49, eq157_e1989_d_b50, eq157_e1989_d_b51, eq157_e1989_d_b52, eq157_e1989_d_b53, eq157_e1989_d_b54,) = {
    if ((s.b[585] && s.b[586]) && s.b[587]) {
        let eq157_e1986: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 56, s.v[264]);
        let eq157_e1987: f64 = (p.p7 * eq157_e1986);
        (eq157_e1987, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_value: f64 = eq157_e1989;
        let eq157_node_derivatives: [f64; 23] = [eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n10, eq157_e1989_d_n11, eq157_e1989_d_n12, eq157_e1989_d_n13, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22];
        let eq157_branch_derivatives: [f64; 55] = [eq157_e1989_d_b0, eq157_e1989_d_b1, eq157_e1989_d_b2, eq157_e1989_d_b3, eq157_e1989_d_b4, eq157_e1989_d_b5, eq157_e1989_d_b6, eq157_e1989_d_b7, eq157_e1989_d_b8, eq157_e1989_d_b9, eq157_e1989_d_b10, eq157_e1989_d_b11, eq157_e1989_d_b12, eq157_e1989_d_b13, eq157_e1989_d_b14, eq157_e1989_d_b15, eq157_e1989_d_b16, eq157_e1989_d_b17, eq157_e1989_d_b18, eq157_e1989_d_b19, eq157_e1989_d_b20, eq157_e1989_d_b21, eq157_e1989_d_b22, eq157_e1989_d_b23, eq157_e1989_d_b24, eq157_e1989_d_b25, eq157_e1989_d_b26, eq157_e1989_d_b27, eq157_e1989_d_b28, eq157_e1989_d_b29, eq157_e1989_d_b30, eq157_e1989_d_b31, eq157_e1989_d_b32, eq157_e1989_d_b33, eq157_e1989_d_b34, eq157_e1989_d_b35, eq157_e1989_d_b36, eq157_e1989_d_b37, eq157_e1989_d_b38, eq157_e1989_d_b39, eq157_e1989_d_b40, eq157_e1989_d_b41, eq157_e1989_d_b42, eq157_e1989_d_b43, eq157_e1989_d_b44, eq157_e1989_d_b45, eq157_e1989_d_b46, eq157_e1989_d_b47, eq157_e1989_d_b48, eq157_e1989_d_b49, eq157_e1989_d_b50, eq157_e1989_d_b51, eq157_e1989_d_b52, eq157_e1989_d_b53, eq157_e1989_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(20),
            multiplicity * (eq157_value),
            &eq157_node_derivatives,
            &eq157_branch_derivatives,
            multiplicity,
        );
        let (eq158_e2002, eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n10, eq158_e2002_d_n11, eq158_e2002_d_n12, eq158_e2002_d_n13, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22, eq158_e2002_d_b0, eq158_e2002_d_b1, eq158_e2002_d_b2, eq158_e2002_d_b3, eq158_e2002_d_b4, eq158_e2002_d_b5, eq158_e2002_d_b6, eq158_e2002_d_b7, eq158_e2002_d_b8, eq158_e2002_d_b9, eq158_e2002_d_b10, eq158_e2002_d_b11, eq158_e2002_d_b12, eq158_e2002_d_b13, eq158_e2002_d_b14, eq158_e2002_d_b15, eq158_e2002_d_b16, eq158_e2002_d_b17, eq158_e2002_d_b18, eq158_e2002_d_b19, eq158_e2002_d_b20, eq158_e2002_d_b21, eq158_e2002_d_b22, eq158_e2002_d_b23, eq158_e2002_d_b24, eq158_e2002_d_b25, eq158_e2002_d_b26, eq158_e2002_d_b27, eq158_e2002_d_b28, eq158_e2002_d_b29, eq158_e2002_d_b30, eq158_e2002_d_b31, eq158_e2002_d_b32, eq158_e2002_d_b33, eq158_e2002_d_b34, eq158_e2002_d_b35, eq158_e2002_d_b36, eq158_e2002_d_b37, eq158_e2002_d_b38, eq158_e2002_d_b39, eq158_e2002_d_b40, eq158_e2002_d_b41, eq158_e2002_d_b42, eq158_e2002_d_b43, eq158_e2002_d_b44, eq158_e2002_d_b45, eq158_e2002_d_b46, eq158_e2002_d_b47, eq158_e2002_d_b48, eq158_e2002_d_b49, eq158_e2002_d_b50, eq158_e2002_d_b51, eq158_e2002_d_b52, eq158_e2002_d_b53, eq158_e2002_d_b54,) = {
    if ((s.b[585] && s.b[586]) && s.b[587]) {
        let eq158_e1997: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 57, s.v[264]);
        let eq158_e1998: f64 = (p.p7 * eq158_e1997);
        let eq158_e2000: f64 = (eq158_e1998 * p.p247);
        let eq158_e2000_d_n0: f64 = (__rspice_deriv_cse_0 * p.p247);
        let eq158_e2000_d_n1: f64 = (__rspice_deriv_cse_1 * p.p247);
        let eq158_e2000_d_n2: f64 = (__rspice_deriv_cse_2 * p.p247);
        let eq158_e2000_d_n3: f64 = (__rspice_deriv_cse_3 * p.p247);
        let eq158_e2000_d_n4: f64 = (__rspice_deriv_cse_4 * p.p247);
        let eq158_e2000_d_n5: f64 = (__rspice_deriv_cse_5 * p.p247);
        let eq158_e2000_d_n6: f64 = (__rspice_deriv_cse_6 * p.p247);
        let eq158_e2000_d_n7: f64 = (__rspice_deriv_cse_7 * p.p247);
        let eq158_e2000_d_n8: f64 = (__rspice_deriv_cse_8 * p.p247);
        let eq158_e2000_d_n9: f64 = (__rspice_deriv_cse_9 * p.p247);
        let eq158_e2000_d_n10: f64 = (__rspice_deriv_cse_10 * p.p247);
        let eq158_e2000_d_n11: f64 = (__rspice_deriv_cse_11 * p.p247);
        let eq158_e2000_d_n12: f64 = (__rspice_deriv_cse_12 * p.p247);
        let eq158_e2000_d_n13: f64 = (__rspice_deriv_cse_13 * p.p247);
        let eq158_e2000_d_n14: f64 = (__rspice_deriv_cse_14 * p.p247);
        let eq158_e2000_d_n15: f64 = (__rspice_deriv_cse_15 * p.p247);
        let eq158_e2000_d_n16: f64 = (__rspice_deriv_cse_16 * p.p247);
        let eq158_e2000_d_n17: f64 = (__rspice_deriv_cse_17 * p.p247);
        let eq158_e2000_d_n18: f64 = (__rspice_deriv_cse_18 * p.p247);
        let eq158_e2000_d_n19: f64 = (__rspice_deriv_cse_19 * p.p247);
        let eq158_e2000_d_n20: f64 = (__rspice_deriv_cse_20 * p.p247);
        let eq158_e2000_d_n21: f64 = (__rspice_deriv_cse_21 * p.p247);
        let eq158_e2000_d_n22: f64 = (__rspice_deriv_cse_22 * p.p247);
        let eq158_e2000_d_b0: f64 = (__rspice_deriv_cse_23 * p.p247);
        let eq158_e2000_d_b1: f64 = (__rspice_deriv_cse_24 * p.p247);
        let eq158_e2000_d_b2: f64 = (__rspice_deriv_cse_25 * p.p247);
        let eq158_e2000_d_b3: f64 = (__rspice_deriv_cse_26 * p.p247);
        let eq158_e2000_d_b4: f64 = (__rspice_deriv_cse_27 * p.p247);
        let eq158_e2000_d_b5: f64 = (__rspice_deriv_cse_28 * p.p247);
        let eq158_e2000_d_b6: f64 = (__rspice_deriv_cse_29 * p.p247);
        let eq158_e2000_d_b7: f64 = (__rspice_deriv_cse_30 * p.p247);
        let eq158_e2000_d_b8: f64 = (__rspice_deriv_cse_31 * p.p247);
        let eq158_e2000_d_b9: f64 = (__rspice_deriv_cse_32 * p.p247);
        let eq158_e2000_d_b10: f64 = (__rspice_deriv_cse_33 * p.p247);
        let eq158_e2000_d_b11: f64 = (__rspice_deriv_cse_34 * p.p247);
        let eq158_e2000_d_b12: f64 = (__rspice_deriv_cse_35 * p.p247);
        let eq158_e2000_d_b13: f64 = (__rspice_deriv_cse_36 * p.p247);
        let eq158_e2000_d_b14: f64 = (__rspice_deriv_cse_37 * p.p247);
        let eq158_e2000_d_b15: f64 = (__rspice_deriv_cse_38 * p.p247);
        let eq158_e2000_d_b16: f64 = (__rspice_deriv_cse_39 * p.p247);
        let eq158_e2000_d_b17: f64 = (__rspice_deriv_cse_40 * p.p247);
        let eq158_e2000_d_b18: f64 = (__rspice_deriv_cse_41 * p.p247);
        let eq158_e2000_d_b19: f64 = (__rspice_deriv_cse_42 * p.p247);
        let eq158_e2000_d_b20: f64 = (__rspice_deriv_cse_43 * p.p247);
        let eq158_e2000_d_b21: f64 = (__rspice_deriv_cse_44 * p.p247);
        let eq158_e2000_d_b22: f64 = (__rspice_deriv_cse_45 * p.p247);
        let eq158_e2000_d_b23: f64 = (__rspice_deriv_cse_46 * p.p247);
        let eq158_e2000_d_b24: f64 = (__rspice_deriv_cse_47 * p.p247);
        let eq158_e2000_d_b25: f64 = (__rspice_deriv_cse_48 * p.p247);
        let eq158_e2000_d_b26: f64 = (__rspice_deriv_cse_49 * p.p247);
        let eq158_e2000_d_b27: f64 = (__rspice_deriv_cse_50 * p.p247);
        let eq158_e2000_d_b28: f64 = (__rspice_deriv_cse_51 * p.p247);
        let eq158_e2000_d_b29: f64 = (__rspice_deriv_cse_52 * p.p247);
        let eq158_e2000_d_b30: f64 = (__rspice_deriv_cse_53 * p.p247);
        let eq158_e2000_d_b31: f64 = (__rspice_deriv_cse_54 * p.p247);
        let eq158_e2000_d_b32: f64 = (__rspice_deriv_cse_55 * p.p247);
        let eq158_e2000_d_b33: f64 = (__rspice_deriv_cse_56 * p.p247);
        let eq158_e2000_d_b34: f64 = (__rspice_deriv_cse_57 * p.p247);
        let eq158_e2000_d_b35: f64 = (__rspice_deriv_cse_58 * p.p247);
        let eq158_e2000_d_b36: f64 = (__rspice_deriv_cse_59 * p.p247);
        let eq158_e2000_d_b37: f64 = (__rspice_deriv_cse_60 * p.p247);
        let eq158_e2000_d_b38: f64 = (__rspice_deriv_cse_61 * p.p247);
        let eq158_e2000_d_b39: f64 = (__rspice_deriv_cse_62 * p.p247);
        let eq158_e2000_d_b40: f64 = (__rspice_deriv_cse_63 * p.p247);
        let eq158_e2000_d_b41: f64 = (__rspice_deriv_cse_64 * p.p247);
        let eq158_e2000_d_b42: f64 = (__rspice_deriv_cse_65 * p.p247);
        let eq158_e2000_d_b43: f64 = (__rspice_deriv_cse_66 * p.p247);
        let eq158_e2000_d_b44: f64 = (__rspice_deriv_cse_67 * p.p247);
        let eq158_e2000_d_b45: f64 = (__rspice_deriv_cse_68 * p.p247);
        let eq158_e2000_d_b46: f64 = (__rspice_deriv_cse_69 * p.p247);
        let eq158_e2000_d_b47: f64 = (__rspice_deriv_cse_70 * p.p247);
        let eq158_e2000_d_b48: f64 = (__rspice_deriv_cse_71 * p.p247);
        let eq158_e2000_d_b49: f64 = (__rspice_deriv_cse_72 * p.p247);
        let eq158_e2000_d_b50: f64 = (__rspice_deriv_cse_73 * p.p247);
        let eq158_e2000_d_b51: f64 = (__rspice_deriv_cse_74 * p.p247);
        let eq158_e2000_d_b52: f64 = (__rspice_deriv_cse_75 * p.p247);
        let eq158_e2000_d_b53: f64 = (__rspice_deriv_cse_76 * p.p247);
        let eq158_e2000_d_b54: f64 = (__rspice_deriv_cse_77 * p.p247);
        (eq158_e2000, eq158_e2000_d_n0, eq158_e2000_d_n1, eq158_e2000_d_n2, eq158_e2000_d_n3, eq158_e2000_d_n4, eq158_e2000_d_n5, eq158_e2000_d_n6, eq158_e2000_d_n7, eq158_e2000_d_n8, eq158_e2000_d_n9, eq158_e2000_d_n10, eq158_e2000_d_n11, eq158_e2000_d_n12, eq158_e2000_d_n13, eq158_e2000_d_n14, eq158_e2000_d_n15, eq158_e2000_d_n16, eq158_e2000_d_n17, eq158_e2000_d_n18, eq158_e2000_d_n19, eq158_e2000_d_n20, eq158_e2000_d_n21, eq158_e2000_d_n22, eq158_e2000_d_b0, eq158_e2000_d_b1, eq158_e2000_d_b2, eq158_e2000_d_b3, eq158_e2000_d_b4, eq158_e2000_d_b5, eq158_e2000_d_b6, eq158_e2000_d_b7, eq158_e2000_d_b8, eq158_e2000_d_b9, eq158_e2000_d_b10, eq158_e2000_d_b11, eq158_e2000_d_b12, eq158_e2000_d_b13, eq158_e2000_d_b14, eq158_e2000_d_b15, eq158_e2000_d_b16, eq158_e2000_d_b17, eq158_e2000_d_b18, eq158_e2000_d_b19, eq158_e2000_d_b20, eq158_e2000_d_b21, eq158_e2000_d_b22, eq158_e2000_d_b23, eq158_e2000_d_b24, eq158_e2000_d_b25, eq158_e2000_d_b26, eq158_e2000_d_b27, eq158_e2000_d_b28, eq158_e2000_d_b29, eq158_e2000_d_b30, eq158_e2000_d_b31, eq158_e2000_d_b32, eq158_e2000_d_b33, eq158_e2000_d_b34, eq158_e2000_d_b35, eq158_e2000_d_b36, eq158_e2000_d_b37, eq158_e2000_d_b38, eq158_e2000_d_b39, eq158_e2000_d_b40, eq158_e2000_d_b41, eq158_e2000_d_b42, eq158_e2000_d_b43, eq158_e2000_d_b44, eq158_e2000_d_b45, eq158_e2000_d_b46, eq158_e2000_d_b47, eq158_e2000_d_b48, eq158_e2000_d_b49, eq158_e2000_d_b50, eq158_e2000_d_b51, eq158_e2000_d_b52, eq158_e2000_d_b53, eq158_e2000_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq158_value: f64 = eq158_e2002;
        let eq158_node_derivatives: [f64; 23] = [eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n10, eq158_e2002_d_n11, eq158_e2002_d_n12, eq158_e2002_d_n13, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22];
        let eq158_branch_derivatives: [f64; 55] = [eq158_e2002_d_b0, eq158_e2002_d_b1, eq158_e2002_d_b2, eq158_e2002_d_b3, eq158_e2002_d_b4, eq158_e2002_d_b5, eq158_e2002_d_b6, eq158_e2002_d_b7, eq158_e2002_d_b8, eq158_e2002_d_b9, eq158_e2002_d_b10, eq158_e2002_d_b11, eq158_e2002_d_b12, eq158_e2002_d_b13, eq158_e2002_d_b14, eq158_e2002_d_b15, eq158_e2002_d_b16, eq158_e2002_d_b17, eq158_e2002_d_b18, eq158_e2002_d_b19, eq158_e2002_d_b20, eq158_e2002_d_b21, eq158_e2002_d_b22, eq158_e2002_d_b23, eq158_e2002_d_b24, eq158_e2002_d_b25, eq158_e2002_d_b26, eq158_e2002_d_b27, eq158_e2002_d_b28, eq158_e2002_d_b29, eq158_e2002_d_b30, eq158_e2002_d_b31, eq158_e2002_d_b32, eq158_e2002_d_b33, eq158_e2002_d_b34, eq158_e2002_d_b35, eq158_e2002_d_b36, eq158_e2002_d_b37, eq158_e2002_d_b38, eq158_e2002_d_b39, eq158_e2002_d_b40, eq158_e2002_d_b41, eq158_e2002_d_b42, eq158_e2002_d_b43, eq158_e2002_d_b44, eq158_e2002_d_b45, eq158_e2002_d_b46, eq158_e2002_d_b47, eq158_e2002_d_b48, eq158_e2002_d_b49, eq158_e2002_d_b50, eq158_e2002_d_b51, eq158_e2002_d_b52, eq158_e2002_d_b53, eq158_e2002_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(20),
            multiplicity * (eq158_value),
            &eq158_node_derivatives,
            &eq158_branch_derivatives,
            multiplicity,
        );
        let (eq159_e2014, eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n10, eq159_e2014_d_n11, eq159_e2014_d_n12, eq159_e2014_d_n13, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22, eq159_e2014_d_b0, eq159_e2014_d_b1, eq159_e2014_d_b2, eq159_e2014_d_b3, eq159_e2014_d_b4, eq159_e2014_d_b5, eq159_e2014_d_b6, eq159_e2014_d_b7, eq159_e2014_d_b8, eq159_e2014_d_b9, eq159_e2014_d_b10, eq159_e2014_d_b11, eq159_e2014_d_b12, eq159_e2014_d_b13, eq159_e2014_d_b14, eq159_e2014_d_b15, eq159_e2014_d_b16, eq159_e2014_d_b17, eq159_e2014_d_b18, eq159_e2014_d_b19, eq159_e2014_d_b20, eq159_e2014_d_b21, eq159_e2014_d_b22, eq159_e2014_d_b23, eq159_e2014_d_b24, eq159_e2014_d_b25, eq159_e2014_d_b26, eq159_e2014_d_b27, eq159_e2014_d_b28, eq159_e2014_d_b29, eq159_e2014_d_b30, eq159_e2014_d_b31, eq159_e2014_d_b32, eq159_e2014_d_b33, eq159_e2014_d_b34, eq159_e2014_d_b35, eq159_e2014_d_b36, eq159_e2014_d_b37, eq159_e2014_d_b38, eq159_e2014_d_b39, eq159_e2014_d_b40, eq159_e2014_d_b41, eq159_e2014_d_b42, eq159_e2014_d_b43, eq159_e2014_d_b44, eq159_e2014_d_b45, eq159_e2014_d_b46, eq159_e2014_d_b47, eq159_e2014_d_b48, eq159_e2014_d_b49, eq159_e2014_d_b50, eq159_e2014_d_b51, eq159_e2014_d_b52, eq159_e2014_d_b53, eq159_e2014_d_b54,) = {
    if ((s.b[585] && s.b[586]) && (!s.b[587])) {
        let eq159_e2011: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 58, s.v[264]);
        let eq159_e2012: f64 = (p.p7 * eq159_e2011);
        (eq159_e2012, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq159_value: f64 = eq159_e2014;
        let eq159_node_derivatives: [f64; 23] = [eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n10, eq159_e2014_d_n11, eq159_e2014_d_n12, eq159_e2014_d_n13, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22];
        let eq159_branch_derivatives: [f64; 55] = [eq159_e2014_d_b0, eq159_e2014_d_b1, eq159_e2014_d_b2, eq159_e2014_d_b3, eq159_e2014_d_b4, eq159_e2014_d_b5, eq159_e2014_d_b6, eq159_e2014_d_b7, eq159_e2014_d_b8, eq159_e2014_d_b9, eq159_e2014_d_b10, eq159_e2014_d_b11, eq159_e2014_d_b12, eq159_e2014_d_b13, eq159_e2014_d_b14, eq159_e2014_d_b15, eq159_e2014_d_b16, eq159_e2014_d_b17, eq159_e2014_d_b18, eq159_e2014_d_b19, eq159_e2014_d_b20, eq159_e2014_d_b21, eq159_e2014_d_b22, eq159_e2014_d_b23, eq159_e2014_d_b24, eq159_e2014_d_b25, eq159_e2014_d_b26, eq159_e2014_d_b27, eq159_e2014_d_b28, eq159_e2014_d_b29, eq159_e2014_d_b30, eq159_e2014_d_b31, eq159_e2014_d_b32, eq159_e2014_d_b33, eq159_e2014_d_b34, eq159_e2014_d_b35, eq159_e2014_d_b36, eq159_e2014_d_b37, eq159_e2014_d_b38, eq159_e2014_d_b39, eq159_e2014_d_b40, eq159_e2014_d_b41, eq159_e2014_d_b42, eq159_e2014_d_b43, eq159_e2014_d_b44, eq159_e2014_d_b45, eq159_e2014_d_b46, eq159_e2014_d_b47, eq159_e2014_d_b48, eq159_e2014_d_b49, eq159_e2014_d_b50, eq159_e2014_d_b51, eq159_e2014_d_b52, eq159_e2014_d_b53, eq159_e2014_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(20),
            multiplicity * (eq159_value),
            &eq159_node_derivatives,
            &eq159_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_28(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq160_e2028, eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n10, eq160_e2028_d_n11, eq160_e2028_d_n12, eq160_e2028_d_n13, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22, eq160_e2028_d_b0, eq160_e2028_d_b1, eq160_e2028_d_b2, eq160_e2028_d_b3, eq160_e2028_d_b4, eq160_e2028_d_b5, eq160_e2028_d_b6, eq160_e2028_d_b7, eq160_e2028_d_b8, eq160_e2028_d_b9, eq160_e2028_d_b10, eq160_e2028_d_b11, eq160_e2028_d_b12, eq160_e2028_d_b13, eq160_e2028_d_b14, eq160_e2028_d_b15, eq160_e2028_d_b16, eq160_e2028_d_b17, eq160_e2028_d_b18, eq160_e2028_d_b19, eq160_e2028_d_b20, eq160_e2028_d_b21, eq160_e2028_d_b22, eq160_e2028_d_b23, eq160_e2028_d_b24, eq160_e2028_d_b25, eq160_e2028_d_b26, eq160_e2028_d_b27, eq160_e2028_d_b28, eq160_e2028_d_b29, eq160_e2028_d_b30, eq160_e2028_d_b31, eq160_e2028_d_b32, eq160_e2028_d_b33, eq160_e2028_d_b34, eq160_e2028_d_b35, eq160_e2028_d_b36, eq160_e2028_d_b37, eq160_e2028_d_b38, eq160_e2028_d_b39, eq160_e2028_d_b40, eq160_e2028_d_b41, eq160_e2028_d_b42, eq160_e2028_d_b43, eq160_e2028_d_b44, eq160_e2028_d_b45, eq160_e2028_d_b46, eq160_e2028_d_b47, eq160_e2028_d_b48, eq160_e2028_d_b49, eq160_e2028_d_b50, eq160_e2028_d_b51, eq160_e2028_d_b52, eq160_e2028_d_b53, eq160_e2028_d_b54,) = {
    if ((s.b[585] && s.b[586]) && (!s.b[587])) {
        let eq160_e2023: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 59, s.v[264]);
        let eq160_e2024: f64 = (p.p7 * eq160_e2023);
        let eq160_e2024_d_n0: f64 = (p.p7 * (s.dn[264][0] * ddt_scale));
        let eq160_e2024_d_n1: f64 = (p.p7 * (s.dn[264][1] * ddt_scale));
        let eq160_e2024_d_n2: f64 = (p.p7 * (s.dn[264][2] * ddt_scale));
        let eq160_e2024_d_n3: f64 = (p.p7 * (s.dn[264][3] * ddt_scale));
        let eq160_e2024_d_n4: f64 = (p.p7 * (s.dn[264][4] * ddt_scale));
        let eq160_e2024_d_n5: f64 = (p.p7 * (s.dn[264][5] * ddt_scale));
        let eq160_e2024_d_n6: f64 = (p.p7 * (s.dn[264][6] * ddt_scale));
        let eq160_e2024_d_n7: f64 = (p.p7 * (s.dn[264][7] * ddt_scale));
        let eq160_e2024_d_n8: f64 = (p.p7 * (s.dn[264][8] * ddt_scale));
        let eq160_e2024_d_n9: f64 = (p.p7 * (s.dn[264][9] * ddt_scale));
        let eq160_e2024_d_n10: f64 = (p.p7 * (s.dn[264][10] * ddt_scale));
        let eq160_e2024_d_n11: f64 = (p.p7 * (s.dn[264][11] * ddt_scale));
        let eq160_e2024_d_n12: f64 = (p.p7 * (s.dn[264][12] * ddt_scale));
        let eq160_e2024_d_n13: f64 = (p.p7 * (s.dn[264][13] * ddt_scale));
        let eq160_e2024_d_n14: f64 = (p.p7 * (s.dn[264][14] * ddt_scale));
        let eq160_e2024_d_n15: f64 = (p.p7 * (s.dn[264][15] * ddt_scale));
        let eq160_e2024_d_n16: f64 = (p.p7 * (s.dn[264][16] * ddt_scale));
        let eq160_e2024_d_n17: f64 = (p.p7 * (s.dn[264][17] * ddt_scale));
        let eq160_e2024_d_n18: f64 = (p.p7 * (s.dn[264][18] * ddt_scale));
        let eq160_e2024_d_n19: f64 = (p.p7 * (s.dn[264][19] * ddt_scale));
        let eq160_e2024_d_n20: f64 = (p.p7 * (s.dn[264][20] * ddt_scale));
        let eq160_e2024_d_n21: f64 = (p.p7 * (s.dn[264][21] * ddt_scale));
        let eq160_e2024_d_n22: f64 = (p.p7 * (s.dn[264][22] * ddt_scale));
        let eq160_e2024_d_b0: f64 = (p.p7 * (s.db[264][0] * ddt_scale));
        let eq160_e2024_d_b1: f64 = (p.p7 * (s.db[264][1] * ddt_scale));
        let eq160_e2024_d_b2: f64 = (p.p7 * (s.db[264][2] * ddt_scale));
        let eq160_e2024_d_b3: f64 = (p.p7 * (s.db[264][3] * ddt_scale));
        let eq160_e2024_d_b4: f64 = (p.p7 * (s.db[264][4] * ddt_scale));
        let eq160_e2024_d_b5: f64 = (p.p7 * (s.db[264][5] * ddt_scale));
        let eq160_e2024_d_b6: f64 = (p.p7 * (s.db[264][6] * ddt_scale));
        let eq160_e2024_d_b7: f64 = (p.p7 * (s.db[264][7] * ddt_scale));
        let eq160_e2024_d_b8: f64 = (p.p7 * (s.db[264][8] * ddt_scale));
        let eq160_e2024_d_b9: f64 = (p.p7 * (s.db[264][9] * ddt_scale));
        let eq160_e2024_d_b10: f64 = (p.p7 * (s.db[264][10] * ddt_scale));
        let eq160_e2024_d_b11: f64 = (p.p7 * (s.db[264][11] * ddt_scale));
        let eq160_e2024_d_b12: f64 = (p.p7 * (s.db[264][12] * ddt_scale));
        let eq160_e2024_d_b13: f64 = (p.p7 * (s.db[264][13] * ddt_scale));
        let eq160_e2024_d_b14: f64 = (p.p7 * (s.db[264][14] * ddt_scale));
        let eq160_e2024_d_b15: f64 = (p.p7 * (s.db[264][15] * ddt_scale));
        let eq160_e2024_d_b16: f64 = (p.p7 * (s.db[264][16] * ddt_scale));
        let eq160_e2024_d_b17: f64 = (p.p7 * (s.db[264][17] * ddt_scale));
        let eq160_e2024_d_b18: f64 = (p.p7 * (s.db[264][18] * ddt_scale));
        let eq160_e2024_d_b19: f64 = (p.p7 * (s.db[264][19] * ddt_scale));
        let eq160_e2024_d_b20: f64 = (p.p7 * (s.db[264][20] * ddt_scale));
        let eq160_e2024_d_b21: f64 = (p.p7 * (s.db[264][21] * ddt_scale));
        let eq160_e2024_d_b22: f64 = (p.p7 * (s.db[264][22] * ddt_scale));
        let eq160_e2024_d_b23: f64 = (p.p7 * (s.db[264][23] * ddt_scale));
        let eq160_e2024_d_b24: f64 = (p.p7 * (s.db[264][24] * ddt_scale));
        let eq160_e2024_d_b25: f64 = (p.p7 * (s.db[264][25] * ddt_scale));
        let eq160_e2024_d_b26: f64 = (p.p7 * (s.db[264][26] * ddt_scale));
        let eq160_e2024_d_b27: f64 = (p.p7 * (s.db[264][27] * ddt_scale));
        let eq160_e2024_d_b28: f64 = (p.p7 * (s.db[264][28] * ddt_scale));
        let eq160_e2024_d_b29: f64 = (p.p7 * (s.db[264][29] * ddt_scale));
        let eq160_e2024_d_b30: f64 = (p.p7 * (s.db[264][30] * ddt_scale));
        let eq160_e2024_d_b31: f64 = (p.p7 * (s.db[264][31] * ddt_scale));
        let eq160_e2024_d_b32: f64 = (p.p7 * (s.db[264][32] * ddt_scale));
        let eq160_e2024_d_b33: f64 = (p.p7 * (s.db[264][33] * ddt_scale));
        let eq160_e2024_d_b34: f64 = (p.p7 * (s.db[264][34] * ddt_scale));
        let eq160_e2024_d_b35: f64 = (p.p7 * (s.db[264][35] * ddt_scale));
        let eq160_e2024_d_b36: f64 = (p.p7 * (s.db[264][36] * ddt_scale));
        let eq160_e2024_d_b37: f64 = (p.p7 * (s.db[264][37] * ddt_scale));
        let eq160_e2024_d_b38: f64 = (p.p7 * (s.db[264][38] * ddt_scale));
        let eq160_e2024_d_b39: f64 = (p.p7 * (s.db[264][39] * ddt_scale));
        let eq160_e2024_d_b40: f64 = (p.p7 * (s.db[264][40] * ddt_scale));
        let eq160_e2024_d_b41: f64 = (p.p7 * (s.db[264][41] * ddt_scale));
        let eq160_e2024_d_b42: f64 = (p.p7 * (s.db[264][42] * ddt_scale));
        let eq160_e2024_d_b43: f64 = (p.p7 * (s.db[264][43] * ddt_scale));
        let eq160_e2024_d_b44: f64 = (p.p7 * (s.db[264][44] * ddt_scale));
        let eq160_e2024_d_b45: f64 = (p.p7 * (s.db[264][45] * ddt_scale));
        let eq160_e2024_d_b46: f64 = (p.p7 * (s.db[264][46] * ddt_scale));
        let eq160_e2024_d_b47: f64 = (p.p7 * (s.db[264][47] * ddt_scale));
        let eq160_e2024_d_b48: f64 = (p.p7 * (s.db[264][48] * ddt_scale));
        let eq160_e2024_d_b49: f64 = (p.p7 * (s.db[264][49] * ddt_scale));
        let eq160_e2024_d_b50: f64 = (p.p7 * (s.db[264][50] * ddt_scale));
        let eq160_e2024_d_b51: f64 = (p.p7 * (s.db[264][51] * ddt_scale));
        let eq160_e2024_d_b52: f64 = (p.p7 * (s.db[264][52] * ddt_scale));
        let eq160_e2024_d_b53: f64 = (p.p7 * (s.db[264][53] * ddt_scale));
        let eq160_e2024_d_b54: f64 = (p.p7 * (s.db[264][54] * ddt_scale));
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
        let eq160_e2026_d_b0: f64 = (eq160_e2024_d_b0 * p.p247);
        let eq160_e2026_d_b1: f64 = (eq160_e2024_d_b1 * p.p247);
        let eq160_e2026_d_b2: f64 = (eq160_e2024_d_b2 * p.p247);
        let eq160_e2026_d_b3: f64 = (eq160_e2024_d_b3 * p.p247);
        let eq160_e2026_d_b4: f64 = (eq160_e2024_d_b4 * p.p247);
        let eq160_e2026_d_b5: f64 = (eq160_e2024_d_b5 * p.p247);
        let eq160_e2026_d_b6: f64 = (eq160_e2024_d_b6 * p.p247);
        let eq160_e2026_d_b7: f64 = (eq160_e2024_d_b7 * p.p247);
        let eq160_e2026_d_b8: f64 = (eq160_e2024_d_b8 * p.p247);
        let eq160_e2026_d_b9: f64 = (eq160_e2024_d_b9 * p.p247);
        let eq160_e2026_d_b10: f64 = (eq160_e2024_d_b10 * p.p247);
        let eq160_e2026_d_b11: f64 = (eq160_e2024_d_b11 * p.p247);
        let eq160_e2026_d_b12: f64 = (eq160_e2024_d_b12 * p.p247);
        let eq160_e2026_d_b13: f64 = (eq160_e2024_d_b13 * p.p247);
        let eq160_e2026_d_b14: f64 = (eq160_e2024_d_b14 * p.p247);
        let eq160_e2026_d_b15: f64 = (eq160_e2024_d_b15 * p.p247);
        let eq160_e2026_d_b16: f64 = (eq160_e2024_d_b16 * p.p247);
        let eq160_e2026_d_b17: f64 = (eq160_e2024_d_b17 * p.p247);
        let eq160_e2026_d_b18: f64 = (eq160_e2024_d_b18 * p.p247);
        let eq160_e2026_d_b19: f64 = (eq160_e2024_d_b19 * p.p247);
        let eq160_e2026_d_b20: f64 = (eq160_e2024_d_b20 * p.p247);
        let eq160_e2026_d_b21: f64 = (eq160_e2024_d_b21 * p.p247);
        let eq160_e2026_d_b22: f64 = (eq160_e2024_d_b22 * p.p247);
        let eq160_e2026_d_b23: f64 = (eq160_e2024_d_b23 * p.p247);
        let eq160_e2026_d_b24: f64 = (eq160_e2024_d_b24 * p.p247);
        let eq160_e2026_d_b25: f64 = (eq160_e2024_d_b25 * p.p247);
        let eq160_e2026_d_b26: f64 = (eq160_e2024_d_b26 * p.p247);
        let eq160_e2026_d_b27: f64 = (eq160_e2024_d_b27 * p.p247);
        let eq160_e2026_d_b28: f64 = (eq160_e2024_d_b28 * p.p247);
        let eq160_e2026_d_b29: f64 = (eq160_e2024_d_b29 * p.p247);
        let eq160_e2026_d_b30: f64 = (eq160_e2024_d_b30 * p.p247);
        let eq160_e2026_d_b31: f64 = (eq160_e2024_d_b31 * p.p247);
        let eq160_e2026_d_b32: f64 = (eq160_e2024_d_b32 * p.p247);
        let eq160_e2026_d_b33: f64 = (eq160_e2024_d_b33 * p.p247);
        let eq160_e2026_d_b34: f64 = (eq160_e2024_d_b34 * p.p247);
        let eq160_e2026_d_b35: f64 = (eq160_e2024_d_b35 * p.p247);
        let eq160_e2026_d_b36: f64 = (eq160_e2024_d_b36 * p.p247);
        let eq160_e2026_d_b37: f64 = (eq160_e2024_d_b37 * p.p247);
        let eq160_e2026_d_b38: f64 = (eq160_e2024_d_b38 * p.p247);
        let eq160_e2026_d_b39: f64 = (eq160_e2024_d_b39 * p.p247);
        let eq160_e2026_d_b40: f64 = (eq160_e2024_d_b40 * p.p247);
        let eq160_e2026_d_b41: f64 = (eq160_e2024_d_b41 * p.p247);
        let eq160_e2026_d_b42: f64 = (eq160_e2024_d_b42 * p.p247);
        let eq160_e2026_d_b43: f64 = (eq160_e2024_d_b43 * p.p247);
        let eq160_e2026_d_b44: f64 = (eq160_e2024_d_b44 * p.p247);
        let eq160_e2026_d_b45: f64 = (eq160_e2024_d_b45 * p.p247);
        let eq160_e2026_d_b46: f64 = (eq160_e2024_d_b46 * p.p247);
        let eq160_e2026_d_b47: f64 = (eq160_e2024_d_b47 * p.p247);
        let eq160_e2026_d_b48: f64 = (eq160_e2024_d_b48 * p.p247);
        let eq160_e2026_d_b49: f64 = (eq160_e2024_d_b49 * p.p247);
        let eq160_e2026_d_b50: f64 = (eq160_e2024_d_b50 * p.p247);
        let eq160_e2026_d_b51: f64 = (eq160_e2024_d_b51 * p.p247);
        let eq160_e2026_d_b52: f64 = (eq160_e2024_d_b52 * p.p247);
        let eq160_e2026_d_b53: f64 = (eq160_e2024_d_b53 * p.p247);
        let eq160_e2026_d_b54: f64 = (eq160_e2024_d_b54 * p.p247);
        (eq160_e2026, eq160_e2026_d_n0, eq160_e2026_d_n1, eq160_e2026_d_n2, eq160_e2026_d_n3, eq160_e2026_d_n4, eq160_e2026_d_n5, eq160_e2026_d_n6, eq160_e2026_d_n7, eq160_e2026_d_n8, eq160_e2026_d_n9, eq160_e2026_d_n10, eq160_e2026_d_n11, eq160_e2026_d_n12, eq160_e2026_d_n13, eq160_e2026_d_n14, eq160_e2026_d_n15, eq160_e2026_d_n16, eq160_e2026_d_n17, eq160_e2026_d_n18, eq160_e2026_d_n19, eq160_e2026_d_n20, eq160_e2026_d_n21, eq160_e2026_d_n22, eq160_e2026_d_b0, eq160_e2026_d_b1, eq160_e2026_d_b2, eq160_e2026_d_b3, eq160_e2026_d_b4, eq160_e2026_d_b5, eq160_e2026_d_b6, eq160_e2026_d_b7, eq160_e2026_d_b8, eq160_e2026_d_b9, eq160_e2026_d_b10, eq160_e2026_d_b11, eq160_e2026_d_b12, eq160_e2026_d_b13, eq160_e2026_d_b14, eq160_e2026_d_b15, eq160_e2026_d_b16, eq160_e2026_d_b17, eq160_e2026_d_b18, eq160_e2026_d_b19, eq160_e2026_d_b20, eq160_e2026_d_b21, eq160_e2026_d_b22, eq160_e2026_d_b23, eq160_e2026_d_b24, eq160_e2026_d_b25, eq160_e2026_d_b26, eq160_e2026_d_b27, eq160_e2026_d_b28, eq160_e2026_d_b29, eq160_e2026_d_b30, eq160_e2026_d_b31, eq160_e2026_d_b32, eq160_e2026_d_b33, eq160_e2026_d_b34, eq160_e2026_d_b35, eq160_e2026_d_b36, eq160_e2026_d_b37, eq160_e2026_d_b38, eq160_e2026_d_b39, eq160_e2026_d_b40, eq160_e2026_d_b41, eq160_e2026_d_b42, eq160_e2026_d_b43, eq160_e2026_d_b44, eq160_e2026_d_b45, eq160_e2026_d_b46, eq160_e2026_d_b47, eq160_e2026_d_b48, eq160_e2026_d_b49, eq160_e2026_d_b50, eq160_e2026_d_b51, eq160_e2026_d_b52, eq160_e2026_d_b53, eq160_e2026_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_value: f64 = eq160_e2028;
        let eq160_node_derivatives: [f64; 23] = [eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n10, eq160_e2028_d_n11, eq160_e2028_d_n12, eq160_e2028_d_n13, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22];
        let eq160_branch_derivatives: [f64; 55] = [eq160_e2028_d_b0, eq160_e2028_d_b1, eq160_e2028_d_b2, eq160_e2028_d_b3, eq160_e2028_d_b4, eq160_e2028_d_b5, eq160_e2028_d_b6, eq160_e2028_d_b7, eq160_e2028_d_b8, eq160_e2028_d_b9, eq160_e2028_d_b10, eq160_e2028_d_b11, eq160_e2028_d_b12, eq160_e2028_d_b13, eq160_e2028_d_b14, eq160_e2028_d_b15, eq160_e2028_d_b16, eq160_e2028_d_b17, eq160_e2028_d_b18, eq160_e2028_d_b19, eq160_e2028_d_b20, eq160_e2028_d_b21, eq160_e2028_d_b22, eq160_e2028_d_b23, eq160_e2028_d_b24, eq160_e2028_d_b25, eq160_e2028_d_b26, eq160_e2028_d_b27, eq160_e2028_d_b28, eq160_e2028_d_b29, eq160_e2028_d_b30, eq160_e2028_d_b31, eq160_e2028_d_b32, eq160_e2028_d_b33, eq160_e2028_d_b34, eq160_e2028_d_b35, eq160_e2028_d_b36, eq160_e2028_d_b37, eq160_e2028_d_b38, eq160_e2028_d_b39, eq160_e2028_d_b40, eq160_e2028_d_b41, eq160_e2028_d_b42, eq160_e2028_d_b43, eq160_e2028_d_b44, eq160_e2028_d_b45, eq160_e2028_d_b46, eq160_e2028_d_b47, eq160_e2028_d_b48, eq160_e2028_d_b49, eq160_e2028_d_b50, eq160_e2028_d_b51, eq160_e2028_d_b52, eq160_e2028_d_b53, eq160_e2028_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(20),
            multiplicity * (eq160_value),
            &eq160_node_derivatives,
            &eq160_branch_derivatives,
            multiplicity,
        );
        let (eq161_e2039, eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n10, eq161_e2039_d_n11, eq161_e2039_d_n12, eq161_e2039_d_n13, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22, eq161_e2039_d_b0, eq161_e2039_d_b1, eq161_e2039_d_b2, eq161_e2039_d_b3, eq161_e2039_d_b4, eq161_e2039_d_b5, eq161_e2039_d_b6, eq161_e2039_d_b7, eq161_e2039_d_b8, eq161_e2039_d_b9, eq161_e2039_d_b10, eq161_e2039_d_b11, eq161_e2039_d_b12, eq161_e2039_d_b13, eq161_e2039_d_b14, eq161_e2039_d_b15, eq161_e2039_d_b16, eq161_e2039_d_b17, eq161_e2039_d_b18, eq161_e2039_d_b19, eq161_e2039_d_b20, eq161_e2039_d_b21, eq161_e2039_d_b22, eq161_e2039_d_b23, eq161_e2039_d_b24, eq161_e2039_d_b25, eq161_e2039_d_b26, eq161_e2039_d_b27, eq161_e2039_d_b28, eq161_e2039_d_b29, eq161_e2039_d_b30, eq161_e2039_d_b31, eq161_e2039_d_b32, eq161_e2039_d_b33, eq161_e2039_d_b34, eq161_e2039_d_b35, eq161_e2039_d_b36, eq161_e2039_d_b37, eq161_e2039_d_b38, eq161_e2039_d_b39, eq161_e2039_d_b40, eq161_e2039_d_b41, eq161_e2039_d_b42, eq161_e2039_d_b43, eq161_e2039_d_b44, eq161_e2039_d_b45, eq161_e2039_d_b46, eq161_e2039_d_b47, eq161_e2039_d_b48, eq161_e2039_d_b49, eq161_e2039_d_b50, eq161_e2039_d_b51, eq161_e2039_d_b52, eq161_e2039_d_b53, eq161_e2039_d_b54,) = {
    if (s.b[585] && s.b[586]) {
        let eq161_e2035: f64 = (p.p252 * s.v[264]);
        let eq161_e2036: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 60, eq161_e2035);
        let eq161_e2036_d_n0: f64 = ((p.p252 * s.dn[264][0]) * ddt_scale);
        let eq161_e2036_d_n1: f64 = ((p.p252 * s.dn[264][1]) * ddt_scale);
        let eq161_e2036_d_n2: f64 = ((p.p252 * s.dn[264][2]) * ddt_scale);
        let eq161_e2036_d_n3: f64 = ((p.p252 * s.dn[264][3]) * ddt_scale);
        let eq161_e2036_d_n4: f64 = ((p.p252 * s.dn[264][4]) * ddt_scale);
        let eq161_e2036_d_n5: f64 = ((p.p252 * s.dn[264][5]) * ddt_scale);
        let eq161_e2036_d_n6: f64 = ((p.p252 * s.dn[264][6]) * ddt_scale);
        let eq161_e2036_d_n7: f64 = ((p.p252 * s.dn[264][7]) * ddt_scale);
        let eq161_e2036_d_n8: f64 = ((p.p252 * s.dn[264][8]) * ddt_scale);
        let eq161_e2036_d_n9: f64 = ((p.p252 * s.dn[264][9]) * ddt_scale);
        let eq161_e2036_d_n10: f64 = ((p.p252 * s.dn[264][10]) * ddt_scale);
        let eq161_e2036_d_n11: f64 = ((p.p252 * s.dn[264][11]) * ddt_scale);
        let eq161_e2036_d_n12: f64 = ((p.p252 * s.dn[264][12]) * ddt_scale);
        let eq161_e2036_d_n13: f64 = ((p.p252 * s.dn[264][13]) * ddt_scale);
        let eq161_e2036_d_n14: f64 = ((p.p252 * s.dn[264][14]) * ddt_scale);
        let eq161_e2036_d_n15: f64 = ((p.p252 * s.dn[264][15]) * ddt_scale);
        let eq161_e2036_d_n16: f64 = ((p.p252 * s.dn[264][16]) * ddt_scale);
        let eq161_e2036_d_n17: f64 = ((p.p252 * s.dn[264][17]) * ddt_scale);
        let eq161_e2036_d_n18: f64 = ((p.p252 * s.dn[264][18]) * ddt_scale);
        let eq161_e2036_d_n19: f64 = ((p.p252 * s.dn[264][19]) * ddt_scale);
        let eq161_e2036_d_n20: f64 = ((p.p252 * s.dn[264][20]) * ddt_scale);
        let eq161_e2036_d_n21: f64 = ((p.p252 * s.dn[264][21]) * ddt_scale);
        let eq161_e2036_d_n22: f64 = ((p.p252 * s.dn[264][22]) * ddt_scale);
        let eq161_e2036_d_b0: f64 = ((p.p252 * s.db[264][0]) * ddt_scale);
        let eq161_e2036_d_b1: f64 = ((p.p252 * s.db[264][1]) * ddt_scale);
        let eq161_e2036_d_b2: f64 = ((p.p252 * s.db[264][2]) * ddt_scale);
        let eq161_e2036_d_b3: f64 = ((p.p252 * s.db[264][3]) * ddt_scale);
        let eq161_e2036_d_b4: f64 = ((p.p252 * s.db[264][4]) * ddt_scale);
        let eq161_e2036_d_b5: f64 = ((p.p252 * s.db[264][5]) * ddt_scale);
        let eq161_e2036_d_b6: f64 = ((p.p252 * s.db[264][6]) * ddt_scale);
        let eq161_e2036_d_b7: f64 = ((p.p252 * s.db[264][7]) * ddt_scale);
        let eq161_e2036_d_b8: f64 = ((p.p252 * s.db[264][8]) * ddt_scale);
        let eq161_e2036_d_b9: f64 = ((p.p252 * s.db[264][9]) * ddt_scale);
        let eq161_e2036_d_b10: f64 = ((p.p252 * s.db[264][10]) * ddt_scale);
        let eq161_e2036_d_b11: f64 = ((p.p252 * s.db[264][11]) * ddt_scale);
        let eq161_e2036_d_b12: f64 = ((p.p252 * s.db[264][12]) * ddt_scale);
        let eq161_e2036_d_b13: f64 = ((p.p252 * s.db[264][13]) * ddt_scale);
        let eq161_e2036_d_b14: f64 = ((p.p252 * s.db[264][14]) * ddt_scale);
        let eq161_e2036_d_b15: f64 = ((p.p252 * s.db[264][15]) * ddt_scale);
        let eq161_e2036_d_b16: f64 = ((p.p252 * s.db[264][16]) * ddt_scale);
        let eq161_e2036_d_b17: f64 = ((p.p252 * s.db[264][17]) * ddt_scale);
        let eq161_e2036_d_b18: f64 = ((p.p252 * s.db[264][18]) * ddt_scale);
        let eq161_e2036_d_b19: f64 = ((p.p252 * s.db[264][19]) * ddt_scale);
        let eq161_e2036_d_b20: f64 = ((p.p252 * s.db[264][20]) * ddt_scale);
        let eq161_e2036_d_b21: f64 = ((p.p252 * s.db[264][21]) * ddt_scale);
        let eq161_e2036_d_b22: f64 = ((p.p252 * s.db[264][22]) * ddt_scale);
        let eq161_e2036_d_b23: f64 = ((p.p252 * s.db[264][23]) * ddt_scale);
        let eq161_e2036_d_b24: f64 = ((p.p252 * s.db[264][24]) * ddt_scale);
        let eq161_e2036_d_b25: f64 = ((p.p252 * s.db[264][25]) * ddt_scale);
        let eq161_e2036_d_b26: f64 = ((p.p252 * s.db[264][26]) * ddt_scale);
        let eq161_e2036_d_b27: f64 = ((p.p252 * s.db[264][27]) * ddt_scale);
        let eq161_e2036_d_b28: f64 = ((p.p252 * s.db[264][28]) * ddt_scale);
        let eq161_e2036_d_b29: f64 = ((p.p252 * s.db[264][29]) * ddt_scale);
        let eq161_e2036_d_b30: f64 = ((p.p252 * s.db[264][30]) * ddt_scale);
        let eq161_e2036_d_b31: f64 = ((p.p252 * s.db[264][31]) * ddt_scale);
        let eq161_e2036_d_b32: f64 = ((p.p252 * s.db[264][32]) * ddt_scale);
        let eq161_e2036_d_b33: f64 = ((p.p252 * s.db[264][33]) * ddt_scale);
        let eq161_e2036_d_b34: f64 = ((p.p252 * s.db[264][34]) * ddt_scale);
        let eq161_e2036_d_b35: f64 = ((p.p252 * s.db[264][35]) * ddt_scale);
        let eq161_e2036_d_b36: f64 = ((p.p252 * s.db[264][36]) * ddt_scale);
        let eq161_e2036_d_b37: f64 = ((p.p252 * s.db[264][37]) * ddt_scale);
        let eq161_e2036_d_b38: f64 = ((p.p252 * s.db[264][38]) * ddt_scale);
        let eq161_e2036_d_b39: f64 = ((p.p252 * s.db[264][39]) * ddt_scale);
        let eq161_e2036_d_b40: f64 = ((p.p252 * s.db[264][40]) * ddt_scale);
        let eq161_e2036_d_b41: f64 = ((p.p252 * s.db[264][41]) * ddt_scale);
        let eq161_e2036_d_b42: f64 = ((p.p252 * s.db[264][42]) * ddt_scale);
        let eq161_e2036_d_b43: f64 = ((p.p252 * s.db[264][43]) * ddt_scale);
        let eq161_e2036_d_b44: f64 = ((p.p252 * s.db[264][44]) * ddt_scale);
        let eq161_e2036_d_b45: f64 = ((p.p252 * s.db[264][45]) * ddt_scale);
        let eq161_e2036_d_b46: f64 = ((p.p252 * s.db[264][46]) * ddt_scale);
        let eq161_e2036_d_b47: f64 = ((p.p252 * s.db[264][47]) * ddt_scale);
        let eq161_e2036_d_b48: f64 = ((p.p252 * s.db[264][48]) * ddt_scale);
        let eq161_e2036_d_b49: f64 = ((p.p252 * s.db[264][49]) * ddt_scale);
        let eq161_e2036_d_b50: f64 = ((p.p252 * s.db[264][50]) * ddt_scale);
        let eq161_e2036_d_b51: f64 = ((p.p252 * s.db[264][51]) * ddt_scale);
        let eq161_e2036_d_b52: f64 = ((p.p252 * s.db[264][52]) * ddt_scale);
        let eq161_e2036_d_b53: f64 = ((p.p252 * s.db[264][53]) * ddt_scale);
        let eq161_e2036_d_b54: f64 = ((p.p252 * s.db[264][54]) * ddt_scale);
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
        let eq161_e2037_d_b0: f64 = (p.p7 * eq161_e2036_d_b0);
        let eq161_e2037_d_b1: f64 = (p.p7 * eq161_e2036_d_b1);
        let eq161_e2037_d_b2: f64 = (p.p7 * eq161_e2036_d_b2);
        let eq161_e2037_d_b3: f64 = (p.p7 * eq161_e2036_d_b3);
        let eq161_e2037_d_b4: f64 = (p.p7 * eq161_e2036_d_b4);
        let eq161_e2037_d_b5: f64 = (p.p7 * eq161_e2036_d_b5);
        let eq161_e2037_d_b6: f64 = (p.p7 * eq161_e2036_d_b6);
        let eq161_e2037_d_b7: f64 = (p.p7 * eq161_e2036_d_b7);
        let eq161_e2037_d_b8: f64 = (p.p7 * eq161_e2036_d_b8);
        let eq161_e2037_d_b9: f64 = (p.p7 * eq161_e2036_d_b9);
        let eq161_e2037_d_b10: f64 = (p.p7 * eq161_e2036_d_b10);
        let eq161_e2037_d_b11: f64 = (p.p7 * eq161_e2036_d_b11);
        let eq161_e2037_d_b12: f64 = (p.p7 * eq161_e2036_d_b12);
        let eq161_e2037_d_b13: f64 = (p.p7 * eq161_e2036_d_b13);
        let eq161_e2037_d_b14: f64 = (p.p7 * eq161_e2036_d_b14);
        let eq161_e2037_d_b15: f64 = (p.p7 * eq161_e2036_d_b15);
        let eq161_e2037_d_b16: f64 = (p.p7 * eq161_e2036_d_b16);
        let eq161_e2037_d_b17: f64 = (p.p7 * eq161_e2036_d_b17);
        let eq161_e2037_d_b18: f64 = (p.p7 * eq161_e2036_d_b18);
        let eq161_e2037_d_b19: f64 = (p.p7 * eq161_e2036_d_b19);
        let eq161_e2037_d_b20: f64 = (p.p7 * eq161_e2036_d_b20);
        let eq161_e2037_d_b21: f64 = (p.p7 * eq161_e2036_d_b21);
        let eq161_e2037_d_b22: f64 = (p.p7 * eq161_e2036_d_b22);
        let eq161_e2037_d_b23: f64 = (p.p7 * eq161_e2036_d_b23);
        let eq161_e2037_d_b24: f64 = (p.p7 * eq161_e2036_d_b24);
        let eq161_e2037_d_b25: f64 = (p.p7 * eq161_e2036_d_b25);
        let eq161_e2037_d_b26: f64 = (p.p7 * eq161_e2036_d_b26);
        let eq161_e2037_d_b27: f64 = (p.p7 * eq161_e2036_d_b27);
        let eq161_e2037_d_b28: f64 = (p.p7 * eq161_e2036_d_b28);
        let eq161_e2037_d_b29: f64 = (p.p7 * eq161_e2036_d_b29);
        let eq161_e2037_d_b30: f64 = (p.p7 * eq161_e2036_d_b30);
        let eq161_e2037_d_b31: f64 = (p.p7 * eq161_e2036_d_b31);
        let eq161_e2037_d_b32: f64 = (p.p7 * eq161_e2036_d_b32);
        let eq161_e2037_d_b33: f64 = (p.p7 * eq161_e2036_d_b33);
        let eq161_e2037_d_b34: f64 = (p.p7 * eq161_e2036_d_b34);
        let eq161_e2037_d_b35: f64 = (p.p7 * eq161_e2036_d_b35);
        let eq161_e2037_d_b36: f64 = (p.p7 * eq161_e2036_d_b36);
        let eq161_e2037_d_b37: f64 = (p.p7 * eq161_e2036_d_b37);
        let eq161_e2037_d_b38: f64 = (p.p7 * eq161_e2036_d_b38);
        let eq161_e2037_d_b39: f64 = (p.p7 * eq161_e2036_d_b39);
        let eq161_e2037_d_b40: f64 = (p.p7 * eq161_e2036_d_b40);
        let eq161_e2037_d_b41: f64 = (p.p7 * eq161_e2036_d_b41);
        let eq161_e2037_d_b42: f64 = (p.p7 * eq161_e2036_d_b42);
        let eq161_e2037_d_b43: f64 = (p.p7 * eq161_e2036_d_b43);
        let eq161_e2037_d_b44: f64 = (p.p7 * eq161_e2036_d_b44);
        let eq161_e2037_d_b45: f64 = (p.p7 * eq161_e2036_d_b45);
        let eq161_e2037_d_b46: f64 = (p.p7 * eq161_e2036_d_b46);
        let eq161_e2037_d_b47: f64 = (p.p7 * eq161_e2036_d_b47);
        let eq161_e2037_d_b48: f64 = (p.p7 * eq161_e2036_d_b48);
        let eq161_e2037_d_b49: f64 = (p.p7 * eq161_e2036_d_b49);
        let eq161_e2037_d_b50: f64 = (p.p7 * eq161_e2036_d_b50);
        let eq161_e2037_d_b51: f64 = (p.p7 * eq161_e2036_d_b51);
        let eq161_e2037_d_b52: f64 = (p.p7 * eq161_e2036_d_b52);
        let eq161_e2037_d_b53: f64 = (p.p7 * eq161_e2036_d_b53);
        let eq161_e2037_d_b54: f64 = (p.p7 * eq161_e2036_d_b54);
        (eq161_e2037, eq161_e2037_d_n0, eq161_e2037_d_n1, eq161_e2037_d_n2, eq161_e2037_d_n3, eq161_e2037_d_n4, eq161_e2037_d_n5, eq161_e2037_d_n6, eq161_e2037_d_n7, eq161_e2037_d_n8, eq161_e2037_d_n9, eq161_e2037_d_n10, eq161_e2037_d_n11, eq161_e2037_d_n12, eq161_e2037_d_n13, eq161_e2037_d_n14, eq161_e2037_d_n15, eq161_e2037_d_n16, eq161_e2037_d_n17, eq161_e2037_d_n18, eq161_e2037_d_n19, eq161_e2037_d_n20, eq161_e2037_d_n21, eq161_e2037_d_n22, eq161_e2037_d_b0, eq161_e2037_d_b1, eq161_e2037_d_b2, eq161_e2037_d_b3, eq161_e2037_d_b4, eq161_e2037_d_b5, eq161_e2037_d_b6, eq161_e2037_d_b7, eq161_e2037_d_b8, eq161_e2037_d_b9, eq161_e2037_d_b10, eq161_e2037_d_b11, eq161_e2037_d_b12, eq161_e2037_d_b13, eq161_e2037_d_b14, eq161_e2037_d_b15, eq161_e2037_d_b16, eq161_e2037_d_b17, eq161_e2037_d_b18, eq161_e2037_d_b19, eq161_e2037_d_b20, eq161_e2037_d_b21, eq161_e2037_d_b22, eq161_e2037_d_b23, eq161_e2037_d_b24, eq161_e2037_d_b25, eq161_e2037_d_b26, eq161_e2037_d_b27, eq161_e2037_d_b28, eq161_e2037_d_b29, eq161_e2037_d_b30, eq161_e2037_d_b31, eq161_e2037_d_b32, eq161_e2037_d_b33, eq161_e2037_d_b34, eq161_e2037_d_b35, eq161_e2037_d_b36, eq161_e2037_d_b37, eq161_e2037_d_b38, eq161_e2037_d_b39, eq161_e2037_d_b40, eq161_e2037_d_b41, eq161_e2037_d_b42, eq161_e2037_d_b43, eq161_e2037_d_b44, eq161_e2037_d_b45, eq161_e2037_d_b46, eq161_e2037_d_b47, eq161_e2037_d_b48, eq161_e2037_d_b49, eq161_e2037_d_b50, eq161_e2037_d_b51, eq161_e2037_d_b52, eq161_e2037_d_b53, eq161_e2037_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_value: f64 = eq161_e2039;
        let eq161_node_derivatives: [f64; 23] = [eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n10, eq161_e2039_d_n11, eq161_e2039_d_n12, eq161_e2039_d_n13, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22];
        let eq161_branch_derivatives: [f64; 55] = [eq161_e2039_d_b0, eq161_e2039_d_b1, eq161_e2039_d_b2, eq161_e2039_d_b3, eq161_e2039_d_b4, eq161_e2039_d_b5, eq161_e2039_d_b6, eq161_e2039_d_b7, eq161_e2039_d_b8, eq161_e2039_d_b9, eq161_e2039_d_b10, eq161_e2039_d_b11, eq161_e2039_d_b12, eq161_e2039_d_b13, eq161_e2039_d_b14, eq161_e2039_d_b15, eq161_e2039_d_b16, eq161_e2039_d_b17, eq161_e2039_d_b18, eq161_e2039_d_b19, eq161_e2039_d_b20, eq161_e2039_d_b21, eq161_e2039_d_b22, eq161_e2039_d_b23, eq161_e2039_d_b24, eq161_e2039_d_b25, eq161_e2039_d_b26, eq161_e2039_d_b27, eq161_e2039_d_b28, eq161_e2039_d_b29, eq161_e2039_d_b30, eq161_e2039_d_b31, eq161_e2039_d_b32, eq161_e2039_d_b33, eq161_e2039_d_b34, eq161_e2039_d_b35, eq161_e2039_d_b36, eq161_e2039_d_b37, eq161_e2039_d_b38, eq161_e2039_d_b39, eq161_e2039_d_b40, eq161_e2039_d_b41, eq161_e2039_d_b42, eq161_e2039_d_b43, eq161_e2039_d_b44, eq161_e2039_d_b45, eq161_e2039_d_b46, eq161_e2039_d_b47, eq161_e2039_d_b48, eq161_e2039_d_b49, eq161_e2039_d_b50, eq161_e2039_d_b51, eq161_e2039_d_b52, eq161_e2039_d_b53, eq161_e2039_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(20),
            multiplicity * (eq161_value),
            &eq161_node_derivatives,
            &eq161_branch_derivatives,
            multiplicity,
        );
        let (eq162_e2049, eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n10, eq162_e2049_d_n11, eq162_e2049_d_n12, eq162_e2049_d_n13, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22, eq162_e2049_d_b0, eq162_e2049_d_b1, eq162_e2049_d_b2, eq162_e2049_d_b3, eq162_e2049_d_b4, eq162_e2049_d_b5, eq162_e2049_d_b6, eq162_e2049_d_b7, eq162_e2049_d_b8, eq162_e2049_d_b9, eq162_e2049_d_b10, eq162_e2049_d_b11, eq162_e2049_d_b12, eq162_e2049_d_b13, eq162_e2049_d_b14, eq162_e2049_d_b15, eq162_e2049_d_b16, eq162_e2049_d_b17, eq162_e2049_d_b18, eq162_e2049_d_b19, eq162_e2049_d_b20, eq162_e2049_d_b21, eq162_e2049_d_b22, eq162_e2049_d_b23, eq162_e2049_d_b24, eq162_e2049_d_b25, eq162_e2049_d_b26, eq162_e2049_d_b27, eq162_e2049_d_b28, eq162_e2049_d_b29, eq162_e2049_d_b30, eq162_e2049_d_b31, eq162_e2049_d_b32, eq162_e2049_d_b33, eq162_e2049_d_b34, eq162_e2049_d_b35, eq162_e2049_d_b36, eq162_e2049_d_b37, eq162_e2049_d_b38, eq162_e2049_d_b39, eq162_e2049_d_b40, eq162_e2049_d_b41, eq162_e2049_d_b42, eq162_e2049_d_b43, eq162_e2049_d_b44, eq162_e2049_d_b45, eq162_e2049_d_b46, eq162_e2049_d_b47, eq162_e2049_d_b48, eq162_e2049_d_b49, eq162_e2049_d_b50, eq162_e2049_d_b51, eq162_e2049_d_b52, eq162_e2049_d_b53, eq162_e2049_d_b54,) = {
    if ((!s.b[585]) && s.b[588]) {
        let eq162_e2046: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 61, s.v[265]);
        let eq162_e2047: f64 = (p.p7 * eq162_e2046);
        let eq162_e2047_d_n0: f64 = (p.p7 * (s.dn[265][0] * ddt_scale));
        let eq162_e2047_d_n1: f64 = (p.p7 * (s.dn[265][1] * ddt_scale));
        let eq162_e2047_d_n2: f64 = (p.p7 * (s.dn[265][2] * ddt_scale));
        let eq162_e2047_d_n3: f64 = (p.p7 * (s.dn[265][3] * ddt_scale));
        let eq162_e2047_d_n4: f64 = (p.p7 * (s.dn[265][4] * ddt_scale));
        let eq162_e2047_d_n5: f64 = (p.p7 * (s.dn[265][5] * ddt_scale));
        let eq162_e2047_d_n6: f64 = (p.p7 * (s.dn[265][6] * ddt_scale));
        let eq162_e2047_d_n7: f64 = (p.p7 * (s.dn[265][7] * ddt_scale));
        let eq162_e2047_d_n8: f64 = (p.p7 * (s.dn[265][8] * ddt_scale));
        let eq162_e2047_d_n9: f64 = (p.p7 * (s.dn[265][9] * ddt_scale));
        let eq162_e2047_d_n10: f64 = (p.p7 * (s.dn[265][10] * ddt_scale));
        let eq162_e2047_d_n11: f64 = (p.p7 * (s.dn[265][11] * ddt_scale));
        let eq162_e2047_d_n12: f64 = (p.p7 * (s.dn[265][12] * ddt_scale));
        let eq162_e2047_d_n13: f64 = (p.p7 * (s.dn[265][13] * ddt_scale));
        let eq162_e2047_d_n14: f64 = (p.p7 * (s.dn[265][14] * ddt_scale));
        let eq162_e2047_d_n15: f64 = (p.p7 * (s.dn[265][15] * ddt_scale));
        let eq162_e2047_d_n16: f64 = (p.p7 * (s.dn[265][16] * ddt_scale));
        let eq162_e2047_d_n17: f64 = (p.p7 * (s.dn[265][17] * ddt_scale));
        let eq162_e2047_d_n18: f64 = (p.p7 * (s.dn[265][18] * ddt_scale));
        let eq162_e2047_d_n19: f64 = (p.p7 * (s.dn[265][19] * ddt_scale));
        let eq162_e2047_d_n20: f64 = (p.p7 * (s.dn[265][20] * ddt_scale));
        let eq162_e2047_d_n21: f64 = (p.p7 * (s.dn[265][21] * ddt_scale));
        let eq162_e2047_d_n22: f64 = (p.p7 * (s.dn[265][22] * ddt_scale));
        let eq162_e2047_d_b0: f64 = (p.p7 * (s.db[265][0] * ddt_scale));
        let eq162_e2047_d_b1: f64 = (p.p7 * (s.db[265][1] * ddt_scale));
        let eq162_e2047_d_b2: f64 = (p.p7 * (s.db[265][2] * ddt_scale));
        let eq162_e2047_d_b3: f64 = (p.p7 * (s.db[265][3] * ddt_scale));
        let eq162_e2047_d_b4: f64 = (p.p7 * (s.db[265][4] * ddt_scale));
        let eq162_e2047_d_b5: f64 = (p.p7 * (s.db[265][5] * ddt_scale));
        let eq162_e2047_d_b6: f64 = (p.p7 * (s.db[265][6] * ddt_scale));
        let eq162_e2047_d_b7: f64 = (p.p7 * (s.db[265][7] * ddt_scale));
        let eq162_e2047_d_b8: f64 = (p.p7 * (s.db[265][8] * ddt_scale));
        let eq162_e2047_d_b9: f64 = (p.p7 * (s.db[265][9] * ddt_scale));
        let eq162_e2047_d_b10: f64 = (p.p7 * (s.db[265][10] * ddt_scale));
        let eq162_e2047_d_b11: f64 = (p.p7 * (s.db[265][11] * ddt_scale));
        let eq162_e2047_d_b12: f64 = (p.p7 * (s.db[265][12] * ddt_scale));
        let eq162_e2047_d_b13: f64 = (p.p7 * (s.db[265][13] * ddt_scale));
        let eq162_e2047_d_b14: f64 = (p.p7 * (s.db[265][14] * ddt_scale));
        let eq162_e2047_d_b15: f64 = (p.p7 * (s.db[265][15] * ddt_scale));
        let eq162_e2047_d_b16: f64 = (p.p7 * (s.db[265][16] * ddt_scale));
        let eq162_e2047_d_b17: f64 = (p.p7 * (s.db[265][17] * ddt_scale));
        let eq162_e2047_d_b18: f64 = (p.p7 * (s.db[265][18] * ddt_scale));
        let eq162_e2047_d_b19: f64 = (p.p7 * (s.db[265][19] * ddt_scale));
        let eq162_e2047_d_b20: f64 = (p.p7 * (s.db[265][20] * ddt_scale));
        let eq162_e2047_d_b21: f64 = (p.p7 * (s.db[265][21] * ddt_scale));
        let eq162_e2047_d_b22: f64 = (p.p7 * (s.db[265][22] * ddt_scale));
        let eq162_e2047_d_b23: f64 = (p.p7 * (s.db[265][23] * ddt_scale));
        let eq162_e2047_d_b24: f64 = (p.p7 * (s.db[265][24] * ddt_scale));
        let eq162_e2047_d_b25: f64 = (p.p7 * (s.db[265][25] * ddt_scale));
        let eq162_e2047_d_b26: f64 = (p.p7 * (s.db[265][26] * ddt_scale));
        let eq162_e2047_d_b27: f64 = (p.p7 * (s.db[265][27] * ddt_scale));
        let eq162_e2047_d_b28: f64 = (p.p7 * (s.db[265][28] * ddt_scale));
        let eq162_e2047_d_b29: f64 = (p.p7 * (s.db[265][29] * ddt_scale));
        let eq162_e2047_d_b30: f64 = (p.p7 * (s.db[265][30] * ddt_scale));
        let eq162_e2047_d_b31: f64 = (p.p7 * (s.db[265][31] * ddt_scale));
        let eq162_e2047_d_b32: f64 = (p.p7 * (s.db[265][32] * ddt_scale));
        let eq162_e2047_d_b33: f64 = (p.p7 * (s.db[265][33] * ddt_scale));
        let eq162_e2047_d_b34: f64 = (p.p7 * (s.db[265][34] * ddt_scale));
        let eq162_e2047_d_b35: f64 = (p.p7 * (s.db[265][35] * ddt_scale));
        let eq162_e2047_d_b36: f64 = (p.p7 * (s.db[265][36] * ddt_scale));
        let eq162_e2047_d_b37: f64 = (p.p7 * (s.db[265][37] * ddt_scale));
        let eq162_e2047_d_b38: f64 = (p.p7 * (s.db[265][38] * ddt_scale));
        let eq162_e2047_d_b39: f64 = (p.p7 * (s.db[265][39] * ddt_scale));
        let eq162_e2047_d_b40: f64 = (p.p7 * (s.db[265][40] * ddt_scale));
        let eq162_e2047_d_b41: f64 = (p.p7 * (s.db[265][41] * ddt_scale));
        let eq162_e2047_d_b42: f64 = (p.p7 * (s.db[265][42] * ddt_scale));
        let eq162_e2047_d_b43: f64 = (p.p7 * (s.db[265][43] * ddt_scale));
        let eq162_e2047_d_b44: f64 = (p.p7 * (s.db[265][44] * ddt_scale));
        let eq162_e2047_d_b45: f64 = (p.p7 * (s.db[265][45] * ddt_scale));
        let eq162_e2047_d_b46: f64 = (p.p7 * (s.db[265][46] * ddt_scale));
        let eq162_e2047_d_b47: f64 = (p.p7 * (s.db[265][47] * ddt_scale));
        let eq162_e2047_d_b48: f64 = (p.p7 * (s.db[265][48] * ddt_scale));
        let eq162_e2047_d_b49: f64 = (p.p7 * (s.db[265][49] * ddt_scale));
        let eq162_e2047_d_b50: f64 = (p.p7 * (s.db[265][50] * ddt_scale));
        let eq162_e2047_d_b51: f64 = (p.p7 * (s.db[265][51] * ddt_scale));
        let eq162_e2047_d_b52: f64 = (p.p7 * (s.db[265][52] * ddt_scale));
        let eq162_e2047_d_b53: f64 = (p.p7 * (s.db[265][53] * ddt_scale));
        let eq162_e2047_d_b54: f64 = (p.p7 * (s.db[265][54] * ddt_scale));
        (eq162_e2047, eq162_e2047_d_n0, eq162_e2047_d_n1, eq162_e2047_d_n2, eq162_e2047_d_n3, eq162_e2047_d_n4, eq162_e2047_d_n5, eq162_e2047_d_n6, eq162_e2047_d_n7, eq162_e2047_d_n8, eq162_e2047_d_n9, eq162_e2047_d_n10, eq162_e2047_d_n11, eq162_e2047_d_n12, eq162_e2047_d_n13, eq162_e2047_d_n14, eq162_e2047_d_n15, eq162_e2047_d_n16, eq162_e2047_d_n17, eq162_e2047_d_n18, eq162_e2047_d_n19, eq162_e2047_d_n20, eq162_e2047_d_n21, eq162_e2047_d_n22, eq162_e2047_d_b0, eq162_e2047_d_b1, eq162_e2047_d_b2, eq162_e2047_d_b3, eq162_e2047_d_b4, eq162_e2047_d_b5, eq162_e2047_d_b6, eq162_e2047_d_b7, eq162_e2047_d_b8, eq162_e2047_d_b9, eq162_e2047_d_b10, eq162_e2047_d_b11, eq162_e2047_d_b12, eq162_e2047_d_b13, eq162_e2047_d_b14, eq162_e2047_d_b15, eq162_e2047_d_b16, eq162_e2047_d_b17, eq162_e2047_d_b18, eq162_e2047_d_b19, eq162_e2047_d_b20, eq162_e2047_d_b21, eq162_e2047_d_b22, eq162_e2047_d_b23, eq162_e2047_d_b24, eq162_e2047_d_b25, eq162_e2047_d_b26, eq162_e2047_d_b27, eq162_e2047_d_b28, eq162_e2047_d_b29, eq162_e2047_d_b30, eq162_e2047_d_b31, eq162_e2047_d_b32, eq162_e2047_d_b33, eq162_e2047_d_b34, eq162_e2047_d_b35, eq162_e2047_d_b36, eq162_e2047_d_b37, eq162_e2047_d_b38, eq162_e2047_d_b39, eq162_e2047_d_b40, eq162_e2047_d_b41, eq162_e2047_d_b42, eq162_e2047_d_b43, eq162_e2047_d_b44, eq162_e2047_d_b45, eq162_e2047_d_b46, eq162_e2047_d_b47, eq162_e2047_d_b48, eq162_e2047_d_b49, eq162_e2047_d_b50, eq162_e2047_d_b51, eq162_e2047_d_b52, eq162_e2047_d_b53, eq162_e2047_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_value: f64 = eq162_e2049;
        let eq162_node_derivatives: [f64; 23] = [eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n10, eq162_e2049_d_n11, eq162_e2049_d_n12, eq162_e2049_d_n13, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22];
        let eq162_branch_derivatives: [f64; 55] = [eq162_e2049_d_b0, eq162_e2049_d_b1, eq162_e2049_d_b2, eq162_e2049_d_b3, eq162_e2049_d_b4, eq162_e2049_d_b5, eq162_e2049_d_b6, eq162_e2049_d_b7, eq162_e2049_d_b8, eq162_e2049_d_b9, eq162_e2049_d_b10, eq162_e2049_d_b11, eq162_e2049_d_b12, eq162_e2049_d_b13, eq162_e2049_d_b14, eq162_e2049_d_b15, eq162_e2049_d_b16, eq162_e2049_d_b17, eq162_e2049_d_b18, eq162_e2049_d_b19, eq162_e2049_d_b20, eq162_e2049_d_b21, eq162_e2049_d_b22, eq162_e2049_d_b23, eq162_e2049_d_b24, eq162_e2049_d_b25, eq162_e2049_d_b26, eq162_e2049_d_b27, eq162_e2049_d_b28, eq162_e2049_d_b29, eq162_e2049_d_b30, eq162_e2049_d_b31, eq162_e2049_d_b32, eq162_e2049_d_b33, eq162_e2049_d_b34, eq162_e2049_d_b35, eq162_e2049_d_b36, eq162_e2049_d_b37, eq162_e2049_d_b38, eq162_e2049_d_b39, eq162_e2049_d_b40, eq162_e2049_d_b41, eq162_e2049_d_b42, eq162_e2049_d_b43, eq162_e2049_d_b44, eq162_e2049_d_b45, eq162_e2049_d_b46, eq162_e2049_d_b47, eq162_e2049_d_b48, eq162_e2049_d_b49, eq162_e2049_d_b50, eq162_e2049_d_b51, eq162_e2049_d_b52, eq162_e2049_d_b53, eq162_e2049_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq162_value),
            &eq162_node_derivatives,
            &eq162_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_29(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[264][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[264][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[264][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[264][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[264][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[264][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[264][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[264][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[264][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[264][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[264][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[264][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[264][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[264][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[264][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[264][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[264][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[264][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[264][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[264][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[264][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[264][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[264][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[264][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[264][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[264][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[264][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[264][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[264][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[264][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[264][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[264][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[264][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[264][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[264][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[264][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[264][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[264][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[264][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[264][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[264][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[264][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[264][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[264][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[264][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[264][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[264][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[264][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[264][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[264][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[264][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[264][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[264][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[264][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[264][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[264][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[264][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[264][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[264][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[264][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[264][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[264][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[264][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[264][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[264][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[264][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[264][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[264][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[264][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[264][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[264][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[264][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[264][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[264][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[264][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[264][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[264][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[264][54] * ddt_scale));
        let (eq163_e2061, eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n10, eq163_e2061_d_n11, eq163_e2061_d_n12, eq163_e2061_d_n13, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22, eq163_e2061_d_b0, eq163_e2061_d_b1, eq163_e2061_d_b2, eq163_e2061_d_b3, eq163_e2061_d_b4, eq163_e2061_d_b5, eq163_e2061_d_b6, eq163_e2061_d_b7, eq163_e2061_d_b8, eq163_e2061_d_b9, eq163_e2061_d_b10, eq163_e2061_d_b11, eq163_e2061_d_b12, eq163_e2061_d_b13, eq163_e2061_d_b14, eq163_e2061_d_b15, eq163_e2061_d_b16, eq163_e2061_d_b17, eq163_e2061_d_b18, eq163_e2061_d_b19, eq163_e2061_d_b20, eq163_e2061_d_b21, eq163_e2061_d_b22, eq163_e2061_d_b23, eq163_e2061_d_b24, eq163_e2061_d_b25, eq163_e2061_d_b26, eq163_e2061_d_b27, eq163_e2061_d_b28, eq163_e2061_d_b29, eq163_e2061_d_b30, eq163_e2061_d_b31, eq163_e2061_d_b32, eq163_e2061_d_b33, eq163_e2061_d_b34, eq163_e2061_d_b35, eq163_e2061_d_b36, eq163_e2061_d_b37, eq163_e2061_d_b38, eq163_e2061_d_b39, eq163_e2061_d_b40, eq163_e2061_d_b41, eq163_e2061_d_b42, eq163_e2061_d_b43, eq163_e2061_d_b44, eq163_e2061_d_b45, eq163_e2061_d_b46, eq163_e2061_d_b47, eq163_e2061_d_b48, eq163_e2061_d_b49, eq163_e2061_d_b50, eq163_e2061_d_b51, eq163_e2061_d_b52, eq163_e2061_d_b53, eq163_e2061_d_b54,) = {
    if (((!s.b[585]) && s.b[588]) && s.b[589]) {
        let eq163_e2058: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 62, s.v[264]);
        let eq163_e2059: f64 = (p.p7 * eq163_e2058);
        (eq163_e2059, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_value: f64 = eq163_e2061;
        let eq163_node_derivatives: [f64; 23] = [eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n10, eq163_e2061_d_n11, eq163_e2061_d_n12, eq163_e2061_d_n13, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22];
        let eq163_branch_derivatives: [f64; 55] = [eq163_e2061_d_b0, eq163_e2061_d_b1, eq163_e2061_d_b2, eq163_e2061_d_b3, eq163_e2061_d_b4, eq163_e2061_d_b5, eq163_e2061_d_b6, eq163_e2061_d_b7, eq163_e2061_d_b8, eq163_e2061_d_b9, eq163_e2061_d_b10, eq163_e2061_d_b11, eq163_e2061_d_b12, eq163_e2061_d_b13, eq163_e2061_d_b14, eq163_e2061_d_b15, eq163_e2061_d_b16, eq163_e2061_d_b17, eq163_e2061_d_b18, eq163_e2061_d_b19, eq163_e2061_d_b20, eq163_e2061_d_b21, eq163_e2061_d_b22, eq163_e2061_d_b23, eq163_e2061_d_b24, eq163_e2061_d_b25, eq163_e2061_d_b26, eq163_e2061_d_b27, eq163_e2061_d_b28, eq163_e2061_d_b29, eq163_e2061_d_b30, eq163_e2061_d_b31, eq163_e2061_d_b32, eq163_e2061_d_b33, eq163_e2061_d_b34, eq163_e2061_d_b35, eq163_e2061_d_b36, eq163_e2061_d_b37, eq163_e2061_d_b38, eq163_e2061_d_b39, eq163_e2061_d_b40, eq163_e2061_d_b41, eq163_e2061_d_b42, eq163_e2061_d_b43, eq163_e2061_d_b44, eq163_e2061_d_b45, eq163_e2061_d_b46, eq163_e2061_d_b47, eq163_e2061_d_b48, eq163_e2061_d_b49, eq163_e2061_d_b50, eq163_e2061_d_b51, eq163_e2061_d_b52, eq163_e2061_d_b53, eq163_e2061_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq163_value),
            &eq163_node_derivatives,
            &eq163_branch_derivatives,
            multiplicity,
        );
        let (eq164_e2075, eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n10, eq164_e2075_d_n11, eq164_e2075_d_n12, eq164_e2075_d_n13, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22, eq164_e2075_d_b0, eq164_e2075_d_b1, eq164_e2075_d_b2, eq164_e2075_d_b3, eq164_e2075_d_b4, eq164_e2075_d_b5, eq164_e2075_d_b6, eq164_e2075_d_b7, eq164_e2075_d_b8, eq164_e2075_d_b9, eq164_e2075_d_b10, eq164_e2075_d_b11, eq164_e2075_d_b12, eq164_e2075_d_b13, eq164_e2075_d_b14, eq164_e2075_d_b15, eq164_e2075_d_b16, eq164_e2075_d_b17, eq164_e2075_d_b18, eq164_e2075_d_b19, eq164_e2075_d_b20, eq164_e2075_d_b21, eq164_e2075_d_b22, eq164_e2075_d_b23, eq164_e2075_d_b24, eq164_e2075_d_b25, eq164_e2075_d_b26, eq164_e2075_d_b27, eq164_e2075_d_b28, eq164_e2075_d_b29, eq164_e2075_d_b30, eq164_e2075_d_b31, eq164_e2075_d_b32, eq164_e2075_d_b33, eq164_e2075_d_b34, eq164_e2075_d_b35, eq164_e2075_d_b36, eq164_e2075_d_b37, eq164_e2075_d_b38, eq164_e2075_d_b39, eq164_e2075_d_b40, eq164_e2075_d_b41, eq164_e2075_d_b42, eq164_e2075_d_b43, eq164_e2075_d_b44, eq164_e2075_d_b45, eq164_e2075_d_b46, eq164_e2075_d_b47, eq164_e2075_d_b48, eq164_e2075_d_b49, eq164_e2075_d_b50, eq164_e2075_d_b51, eq164_e2075_d_b52, eq164_e2075_d_b53, eq164_e2075_d_b54,) = {
    if (((!s.b[585]) && s.b[588]) && s.b[589]) {
        let eq164_e2070: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 63, s.v[264]);
        let eq164_e2071: f64 = (p.p7 * eq164_e2070);
        let eq164_e2073: f64 = (eq164_e2071 * p.p247);
        let eq164_e2073_d_n0: f64 = (__rspice_deriv_cse_0 * p.p247);
        let eq164_e2073_d_n1: f64 = (__rspice_deriv_cse_1 * p.p247);
        let eq164_e2073_d_n2: f64 = (__rspice_deriv_cse_2 * p.p247);
        let eq164_e2073_d_n3: f64 = (__rspice_deriv_cse_3 * p.p247);
        let eq164_e2073_d_n4: f64 = (__rspice_deriv_cse_4 * p.p247);
        let eq164_e2073_d_n5: f64 = (__rspice_deriv_cse_5 * p.p247);
        let eq164_e2073_d_n6: f64 = (__rspice_deriv_cse_6 * p.p247);
        let eq164_e2073_d_n7: f64 = (__rspice_deriv_cse_7 * p.p247);
        let eq164_e2073_d_n8: f64 = (__rspice_deriv_cse_8 * p.p247);
        let eq164_e2073_d_n9: f64 = (__rspice_deriv_cse_9 * p.p247);
        let eq164_e2073_d_n10: f64 = (__rspice_deriv_cse_10 * p.p247);
        let eq164_e2073_d_n11: f64 = (__rspice_deriv_cse_11 * p.p247);
        let eq164_e2073_d_n12: f64 = (__rspice_deriv_cse_12 * p.p247);
        let eq164_e2073_d_n13: f64 = (__rspice_deriv_cse_13 * p.p247);
        let eq164_e2073_d_n14: f64 = (__rspice_deriv_cse_14 * p.p247);
        let eq164_e2073_d_n15: f64 = (__rspice_deriv_cse_15 * p.p247);
        let eq164_e2073_d_n16: f64 = (__rspice_deriv_cse_16 * p.p247);
        let eq164_e2073_d_n17: f64 = (__rspice_deriv_cse_17 * p.p247);
        let eq164_e2073_d_n18: f64 = (__rspice_deriv_cse_18 * p.p247);
        let eq164_e2073_d_n19: f64 = (__rspice_deriv_cse_19 * p.p247);
        let eq164_e2073_d_n20: f64 = (__rspice_deriv_cse_20 * p.p247);
        let eq164_e2073_d_n21: f64 = (__rspice_deriv_cse_21 * p.p247);
        let eq164_e2073_d_n22: f64 = (__rspice_deriv_cse_22 * p.p247);
        let eq164_e2073_d_b0: f64 = (__rspice_deriv_cse_23 * p.p247);
        let eq164_e2073_d_b1: f64 = (__rspice_deriv_cse_24 * p.p247);
        let eq164_e2073_d_b2: f64 = (__rspice_deriv_cse_25 * p.p247);
        let eq164_e2073_d_b3: f64 = (__rspice_deriv_cse_26 * p.p247);
        let eq164_e2073_d_b4: f64 = (__rspice_deriv_cse_27 * p.p247);
        let eq164_e2073_d_b5: f64 = (__rspice_deriv_cse_28 * p.p247);
        let eq164_e2073_d_b6: f64 = (__rspice_deriv_cse_29 * p.p247);
        let eq164_e2073_d_b7: f64 = (__rspice_deriv_cse_30 * p.p247);
        let eq164_e2073_d_b8: f64 = (__rspice_deriv_cse_31 * p.p247);
        let eq164_e2073_d_b9: f64 = (__rspice_deriv_cse_32 * p.p247);
        let eq164_e2073_d_b10: f64 = (__rspice_deriv_cse_33 * p.p247);
        let eq164_e2073_d_b11: f64 = (__rspice_deriv_cse_34 * p.p247);
        let eq164_e2073_d_b12: f64 = (__rspice_deriv_cse_35 * p.p247);
        let eq164_e2073_d_b13: f64 = (__rspice_deriv_cse_36 * p.p247);
        let eq164_e2073_d_b14: f64 = (__rspice_deriv_cse_37 * p.p247);
        let eq164_e2073_d_b15: f64 = (__rspice_deriv_cse_38 * p.p247);
        let eq164_e2073_d_b16: f64 = (__rspice_deriv_cse_39 * p.p247);
        let eq164_e2073_d_b17: f64 = (__rspice_deriv_cse_40 * p.p247);
        let eq164_e2073_d_b18: f64 = (__rspice_deriv_cse_41 * p.p247);
        let eq164_e2073_d_b19: f64 = (__rspice_deriv_cse_42 * p.p247);
        let eq164_e2073_d_b20: f64 = (__rspice_deriv_cse_43 * p.p247);
        let eq164_e2073_d_b21: f64 = (__rspice_deriv_cse_44 * p.p247);
        let eq164_e2073_d_b22: f64 = (__rspice_deriv_cse_45 * p.p247);
        let eq164_e2073_d_b23: f64 = (__rspice_deriv_cse_46 * p.p247);
        let eq164_e2073_d_b24: f64 = (__rspice_deriv_cse_47 * p.p247);
        let eq164_e2073_d_b25: f64 = (__rspice_deriv_cse_48 * p.p247);
        let eq164_e2073_d_b26: f64 = (__rspice_deriv_cse_49 * p.p247);
        let eq164_e2073_d_b27: f64 = (__rspice_deriv_cse_50 * p.p247);
        let eq164_e2073_d_b28: f64 = (__rspice_deriv_cse_51 * p.p247);
        let eq164_e2073_d_b29: f64 = (__rspice_deriv_cse_52 * p.p247);
        let eq164_e2073_d_b30: f64 = (__rspice_deriv_cse_53 * p.p247);
        let eq164_e2073_d_b31: f64 = (__rspice_deriv_cse_54 * p.p247);
        let eq164_e2073_d_b32: f64 = (__rspice_deriv_cse_55 * p.p247);
        let eq164_e2073_d_b33: f64 = (__rspice_deriv_cse_56 * p.p247);
        let eq164_e2073_d_b34: f64 = (__rspice_deriv_cse_57 * p.p247);
        let eq164_e2073_d_b35: f64 = (__rspice_deriv_cse_58 * p.p247);
        let eq164_e2073_d_b36: f64 = (__rspice_deriv_cse_59 * p.p247);
        let eq164_e2073_d_b37: f64 = (__rspice_deriv_cse_60 * p.p247);
        let eq164_e2073_d_b38: f64 = (__rspice_deriv_cse_61 * p.p247);
        let eq164_e2073_d_b39: f64 = (__rspice_deriv_cse_62 * p.p247);
        let eq164_e2073_d_b40: f64 = (__rspice_deriv_cse_63 * p.p247);
        let eq164_e2073_d_b41: f64 = (__rspice_deriv_cse_64 * p.p247);
        let eq164_e2073_d_b42: f64 = (__rspice_deriv_cse_65 * p.p247);
        let eq164_e2073_d_b43: f64 = (__rspice_deriv_cse_66 * p.p247);
        let eq164_e2073_d_b44: f64 = (__rspice_deriv_cse_67 * p.p247);
        let eq164_e2073_d_b45: f64 = (__rspice_deriv_cse_68 * p.p247);
        let eq164_e2073_d_b46: f64 = (__rspice_deriv_cse_69 * p.p247);
        let eq164_e2073_d_b47: f64 = (__rspice_deriv_cse_70 * p.p247);
        let eq164_e2073_d_b48: f64 = (__rspice_deriv_cse_71 * p.p247);
        let eq164_e2073_d_b49: f64 = (__rspice_deriv_cse_72 * p.p247);
        let eq164_e2073_d_b50: f64 = (__rspice_deriv_cse_73 * p.p247);
        let eq164_e2073_d_b51: f64 = (__rspice_deriv_cse_74 * p.p247);
        let eq164_e2073_d_b52: f64 = (__rspice_deriv_cse_75 * p.p247);
        let eq164_e2073_d_b53: f64 = (__rspice_deriv_cse_76 * p.p247);
        let eq164_e2073_d_b54: f64 = (__rspice_deriv_cse_77 * p.p247);
        (eq164_e2073, eq164_e2073_d_n0, eq164_e2073_d_n1, eq164_e2073_d_n2, eq164_e2073_d_n3, eq164_e2073_d_n4, eq164_e2073_d_n5, eq164_e2073_d_n6, eq164_e2073_d_n7, eq164_e2073_d_n8, eq164_e2073_d_n9, eq164_e2073_d_n10, eq164_e2073_d_n11, eq164_e2073_d_n12, eq164_e2073_d_n13, eq164_e2073_d_n14, eq164_e2073_d_n15, eq164_e2073_d_n16, eq164_e2073_d_n17, eq164_e2073_d_n18, eq164_e2073_d_n19, eq164_e2073_d_n20, eq164_e2073_d_n21, eq164_e2073_d_n22, eq164_e2073_d_b0, eq164_e2073_d_b1, eq164_e2073_d_b2, eq164_e2073_d_b3, eq164_e2073_d_b4, eq164_e2073_d_b5, eq164_e2073_d_b6, eq164_e2073_d_b7, eq164_e2073_d_b8, eq164_e2073_d_b9, eq164_e2073_d_b10, eq164_e2073_d_b11, eq164_e2073_d_b12, eq164_e2073_d_b13, eq164_e2073_d_b14, eq164_e2073_d_b15, eq164_e2073_d_b16, eq164_e2073_d_b17, eq164_e2073_d_b18, eq164_e2073_d_b19, eq164_e2073_d_b20, eq164_e2073_d_b21, eq164_e2073_d_b22, eq164_e2073_d_b23, eq164_e2073_d_b24, eq164_e2073_d_b25, eq164_e2073_d_b26, eq164_e2073_d_b27, eq164_e2073_d_b28, eq164_e2073_d_b29, eq164_e2073_d_b30, eq164_e2073_d_b31, eq164_e2073_d_b32, eq164_e2073_d_b33, eq164_e2073_d_b34, eq164_e2073_d_b35, eq164_e2073_d_b36, eq164_e2073_d_b37, eq164_e2073_d_b38, eq164_e2073_d_b39, eq164_e2073_d_b40, eq164_e2073_d_b41, eq164_e2073_d_b42, eq164_e2073_d_b43, eq164_e2073_d_b44, eq164_e2073_d_b45, eq164_e2073_d_b46, eq164_e2073_d_b47, eq164_e2073_d_b48, eq164_e2073_d_b49, eq164_e2073_d_b50, eq164_e2073_d_b51, eq164_e2073_d_b52, eq164_e2073_d_b53, eq164_e2073_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_value: f64 = eq164_e2075;
        let eq164_node_derivatives: [f64; 23] = [eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n10, eq164_e2075_d_n11, eq164_e2075_d_n12, eq164_e2075_d_n13, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22];
        let eq164_branch_derivatives: [f64; 55] = [eq164_e2075_d_b0, eq164_e2075_d_b1, eq164_e2075_d_b2, eq164_e2075_d_b3, eq164_e2075_d_b4, eq164_e2075_d_b5, eq164_e2075_d_b6, eq164_e2075_d_b7, eq164_e2075_d_b8, eq164_e2075_d_b9, eq164_e2075_d_b10, eq164_e2075_d_b11, eq164_e2075_d_b12, eq164_e2075_d_b13, eq164_e2075_d_b14, eq164_e2075_d_b15, eq164_e2075_d_b16, eq164_e2075_d_b17, eq164_e2075_d_b18, eq164_e2075_d_b19, eq164_e2075_d_b20, eq164_e2075_d_b21, eq164_e2075_d_b22, eq164_e2075_d_b23, eq164_e2075_d_b24, eq164_e2075_d_b25, eq164_e2075_d_b26, eq164_e2075_d_b27, eq164_e2075_d_b28, eq164_e2075_d_b29, eq164_e2075_d_b30, eq164_e2075_d_b31, eq164_e2075_d_b32, eq164_e2075_d_b33, eq164_e2075_d_b34, eq164_e2075_d_b35, eq164_e2075_d_b36, eq164_e2075_d_b37, eq164_e2075_d_b38, eq164_e2075_d_b39, eq164_e2075_d_b40, eq164_e2075_d_b41, eq164_e2075_d_b42, eq164_e2075_d_b43, eq164_e2075_d_b44, eq164_e2075_d_b45, eq164_e2075_d_b46, eq164_e2075_d_b47, eq164_e2075_d_b48, eq164_e2075_d_b49, eq164_e2075_d_b50, eq164_e2075_d_b51, eq164_e2075_d_b52, eq164_e2075_d_b53, eq164_e2075_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq164_value),
            &eq164_node_derivatives,
            &eq164_branch_derivatives,
            multiplicity,
        );
        let (eq165_e2088, eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n10, eq165_e2088_d_n11, eq165_e2088_d_n12, eq165_e2088_d_n13, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22, eq165_e2088_d_b0, eq165_e2088_d_b1, eq165_e2088_d_b2, eq165_e2088_d_b3, eq165_e2088_d_b4, eq165_e2088_d_b5, eq165_e2088_d_b6, eq165_e2088_d_b7, eq165_e2088_d_b8, eq165_e2088_d_b9, eq165_e2088_d_b10, eq165_e2088_d_b11, eq165_e2088_d_b12, eq165_e2088_d_b13, eq165_e2088_d_b14, eq165_e2088_d_b15, eq165_e2088_d_b16, eq165_e2088_d_b17, eq165_e2088_d_b18, eq165_e2088_d_b19, eq165_e2088_d_b20, eq165_e2088_d_b21, eq165_e2088_d_b22, eq165_e2088_d_b23, eq165_e2088_d_b24, eq165_e2088_d_b25, eq165_e2088_d_b26, eq165_e2088_d_b27, eq165_e2088_d_b28, eq165_e2088_d_b29, eq165_e2088_d_b30, eq165_e2088_d_b31, eq165_e2088_d_b32, eq165_e2088_d_b33, eq165_e2088_d_b34, eq165_e2088_d_b35, eq165_e2088_d_b36, eq165_e2088_d_b37, eq165_e2088_d_b38, eq165_e2088_d_b39, eq165_e2088_d_b40, eq165_e2088_d_b41, eq165_e2088_d_b42, eq165_e2088_d_b43, eq165_e2088_d_b44, eq165_e2088_d_b45, eq165_e2088_d_b46, eq165_e2088_d_b47, eq165_e2088_d_b48, eq165_e2088_d_b49, eq165_e2088_d_b50, eq165_e2088_d_b51, eq165_e2088_d_b52, eq165_e2088_d_b53, eq165_e2088_d_b54,) = {
    if (((!s.b[585]) && s.b[588]) && (!s.b[589])) {
        let eq165_e2085: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 64, s.v[264]);
        let eq165_e2086: f64 = (p.p7 * eq165_e2085);
        (eq165_e2086, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq165_value: f64 = eq165_e2088;
        let eq165_node_derivatives: [f64; 23] = [eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n10, eq165_e2088_d_n11, eq165_e2088_d_n12, eq165_e2088_d_n13, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22];
        let eq165_branch_derivatives: [f64; 55] = [eq165_e2088_d_b0, eq165_e2088_d_b1, eq165_e2088_d_b2, eq165_e2088_d_b3, eq165_e2088_d_b4, eq165_e2088_d_b5, eq165_e2088_d_b6, eq165_e2088_d_b7, eq165_e2088_d_b8, eq165_e2088_d_b9, eq165_e2088_d_b10, eq165_e2088_d_b11, eq165_e2088_d_b12, eq165_e2088_d_b13, eq165_e2088_d_b14, eq165_e2088_d_b15, eq165_e2088_d_b16, eq165_e2088_d_b17, eq165_e2088_d_b18, eq165_e2088_d_b19, eq165_e2088_d_b20, eq165_e2088_d_b21, eq165_e2088_d_b22, eq165_e2088_d_b23, eq165_e2088_d_b24, eq165_e2088_d_b25, eq165_e2088_d_b26, eq165_e2088_d_b27, eq165_e2088_d_b28, eq165_e2088_d_b29, eq165_e2088_d_b30, eq165_e2088_d_b31, eq165_e2088_d_b32, eq165_e2088_d_b33, eq165_e2088_d_b34, eq165_e2088_d_b35, eq165_e2088_d_b36, eq165_e2088_d_b37, eq165_e2088_d_b38, eq165_e2088_d_b39, eq165_e2088_d_b40, eq165_e2088_d_b41, eq165_e2088_d_b42, eq165_e2088_d_b43, eq165_e2088_d_b44, eq165_e2088_d_b45, eq165_e2088_d_b46, eq165_e2088_d_b47, eq165_e2088_d_b48, eq165_e2088_d_b49, eq165_e2088_d_b50, eq165_e2088_d_b51, eq165_e2088_d_b52, eq165_e2088_d_b53, eq165_e2088_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq165_value),
            &eq165_node_derivatives,
            &eq165_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_30(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq166_e2103, eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n10, eq166_e2103_d_n11, eq166_e2103_d_n12, eq166_e2103_d_n13, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22, eq166_e2103_d_b0, eq166_e2103_d_b1, eq166_e2103_d_b2, eq166_e2103_d_b3, eq166_e2103_d_b4, eq166_e2103_d_b5, eq166_e2103_d_b6, eq166_e2103_d_b7, eq166_e2103_d_b8, eq166_e2103_d_b9, eq166_e2103_d_b10, eq166_e2103_d_b11, eq166_e2103_d_b12, eq166_e2103_d_b13, eq166_e2103_d_b14, eq166_e2103_d_b15, eq166_e2103_d_b16, eq166_e2103_d_b17, eq166_e2103_d_b18, eq166_e2103_d_b19, eq166_e2103_d_b20, eq166_e2103_d_b21, eq166_e2103_d_b22, eq166_e2103_d_b23, eq166_e2103_d_b24, eq166_e2103_d_b25, eq166_e2103_d_b26, eq166_e2103_d_b27, eq166_e2103_d_b28, eq166_e2103_d_b29, eq166_e2103_d_b30, eq166_e2103_d_b31, eq166_e2103_d_b32, eq166_e2103_d_b33, eq166_e2103_d_b34, eq166_e2103_d_b35, eq166_e2103_d_b36, eq166_e2103_d_b37, eq166_e2103_d_b38, eq166_e2103_d_b39, eq166_e2103_d_b40, eq166_e2103_d_b41, eq166_e2103_d_b42, eq166_e2103_d_b43, eq166_e2103_d_b44, eq166_e2103_d_b45, eq166_e2103_d_b46, eq166_e2103_d_b47, eq166_e2103_d_b48, eq166_e2103_d_b49, eq166_e2103_d_b50, eq166_e2103_d_b51, eq166_e2103_d_b52, eq166_e2103_d_b53, eq166_e2103_d_b54,) = {
    if (((!s.b[585]) && s.b[588]) && (!s.b[589])) {
        let eq166_e2098: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 65, s.v[264]);
        let eq166_e2099: f64 = (p.p7 * eq166_e2098);
        let eq166_e2099_d_n0: f64 = (p.p7 * (s.dn[264][0] * ddt_scale));
        let eq166_e2099_d_n1: f64 = (p.p7 * (s.dn[264][1] * ddt_scale));
        let eq166_e2099_d_n2: f64 = (p.p7 * (s.dn[264][2] * ddt_scale));
        let eq166_e2099_d_n3: f64 = (p.p7 * (s.dn[264][3] * ddt_scale));
        let eq166_e2099_d_n4: f64 = (p.p7 * (s.dn[264][4] * ddt_scale));
        let eq166_e2099_d_n5: f64 = (p.p7 * (s.dn[264][5] * ddt_scale));
        let eq166_e2099_d_n6: f64 = (p.p7 * (s.dn[264][6] * ddt_scale));
        let eq166_e2099_d_n7: f64 = (p.p7 * (s.dn[264][7] * ddt_scale));
        let eq166_e2099_d_n8: f64 = (p.p7 * (s.dn[264][8] * ddt_scale));
        let eq166_e2099_d_n9: f64 = (p.p7 * (s.dn[264][9] * ddt_scale));
        let eq166_e2099_d_n10: f64 = (p.p7 * (s.dn[264][10] * ddt_scale));
        let eq166_e2099_d_n11: f64 = (p.p7 * (s.dn[264][11] * ddt_scale));
        let eq166_e2099_d_n12: f64 = (p.p7 * (s.dn[264][12] * ddt_scale));
        let eq166_e2099_d_n13: f64 = (p.p7 * (s.dn[264][13] * ddt_scale));
        let eq166_e2099_d_n14: f64 = (p.p7 * (s.dn[264][14] * ddt_scale));
        let eq166_e2099_d_n15: f64 = (p.p7 * (s.dn[264][15] * ddt_scale));
        let eq166_e2099_d_n16: f64 = (p.p7 * (s.dn[264][16] * ddt_scale));
        let eq166_e2099_d_n17: f64 = (p.p7 * (s.dn[264][17] * ddt_scale));
        let eq166_e2099_d_n18: f64 = (p.p7 * (s.dn[264][18] * ddt_scale));
        let eq166_e2099_d_n19: f64 = (p.p7 * (s.dn[264][19] * ddt_scale));
        let eq166_e2099_d_n20: f64 = (p.p7 * (s.dn[264][20] * ddt_scale));
        let eq166_e2099_d_n21: f64 = (p.p7 * (s.dn[264][21] * ddt_scale));
        let eq166_e2099_d_n22: f64 = (p.p7 * (s.dn[264][22] * ddt_scale));
        let eq166_e2099_d_b0: f64 = (p.p7 * (s.db[264][0] * ddt_scale));
        let eq166_e2099_d_b1: f64 = (p.p7 * (s.db[264][1] * ddt_scale));
        let eq166_e2099_d_b2: f64 = (p.p7 * (s.db[264][2] * ddt_scale));
        let eq166_e2099_d_b3: f64 = (p.p7 * (s.db[264][3] * ddt_scale));
        let eq166_e2099_d_b4: f64 = (p.p7 * (s.db[264][4] * ddt_scale));
        let eq166_e2099_d_b5: f64 = (p.p7 * (s.db[264][5] * ddt_scale));
        let eq166_e2099_d_b6: f64 = (p.p7 * (s.db[264][6] * ddt_scale));
        let eq166_e2099_d_b7: f64 = (p.p7 * (s.db[264][7] * ddt_scale));
        let eq166_e2099_d_b8: f64 = (p.p7 * (s.db[264][8] * ddt_scale));
        let eq166_e2099_d_b9: f64 = (p.p7 * (s.db[264][9] * ddt_scale));
        let eq166_e2099_d_b10: f64 = (p.p7 * (s.db[264][10] * ddt_scale));
        let eq166_e2099_d_b11: f64 = (p.p7 * (s.db[264][11] * ddt_scale));
        let eq166_e2099_d_b12: f64 = (p.p7 * (s.db[264][12] * ddt_scale));
        let eq166_e2099_d_b13: f64 = (p.p7 * (s.db[264][13] * ddt_scale));
        let eq166_e2099_d_b14: f64 = (p.p7 * (s.db[264][14] * ddt_scale));
        let eq166_e2099_d_b15: f64 = (p.p7 * (s.db[264][15] * ddt_scale));
        let eq166_e2099_d_b16: f64 = (p.p7 * (s.db[264][16] * ddt_scale));
        let eq166_e2099_d_b17: f64 = (p.p7 * (s.db[264][17] * ddt_scale));
        let eq166_e2099_d_b18: f64 = (p.p7 * (s.db[264][18] * ddt_scale));
        let eq166_e2099_d_b19: f64 = (p.p7 * (s.db[264][19] * ddt_scale));
        let eq166_e2099_d_b20: f64 = (p.p7 * (s.db[264][20] * ddt_scale));
        let eq166_e2099_d_b21: f64 = (p.p7 * (s.db[264][21] * ddt_scale));
        let eq166_e2099_d_b22: f64 = (p.p7 * (s.db[264][22] * ddt_scale));
        let eq166_e2099_d_b23: f64 = (p.p7 * (s.db[264][23] * ddt_scale));
        let eq166_e2099_d_b24: f64 = (p.p7 * (s.db[264][24] * ddt_scale));
        let eq166_e2099_d_b25: f64 = (p.p7 * (s.db[264][25] * ddt_scale));
        let eq166_e2099_d_b26: f64 = (p.p7 * (s.db[264][26] * ddt_scale));
        let eq166_e2099_d_b27: f64 = (p.p7 * (s.db[264][27] * ddt_scale));
        let eq166_e2099_d_b28: f64 = (p.p7 * (s.db[264][28] * ddt_scale));
        let eq166_e2099_d_b29: f64 = (p.p7 * (s.db[264][29] * ddt_scale));
        let eq166_e2099_d_b30: f64 = (p.p7 * (s.db[264][30] * ddt_scale));
        let eq166_e2099_d_b31: f64 = (p.p7 * (s.db[264][31] * ddt_scale));
        let eq166_e2099_d_b32: f64 = (p.p7 * (s.db[264][32] * ddt_scale));
        let eq166_e2099_d_b33: f64 = (p.p7 * (s.db[264][33] * ddt_scale));
        let eq166_e2099_d_b34: f64 = (p.p7 * (s.db[264][34] * ddt_scale));
        let eq166_e2099_d_b35: f64 = (p.p7 * (s.db[264][35] * ddt_scale));
        let eq166_e2099_d_b36: f64 = (p.p7 * (s.db[264][36] * ddt_scale));
        let eq166_e2099_d_b37: f64 = (p.p7 * (s.db[264][37] * ddt_scale));
        let eq166_e2099_d_b38: f64 = (p.p7 * (s.db[264][38] * ddt_scale));
        let eq166_e2099_d_b39: f64 = (p.p7 * (s.db[264][39] * ddt_scale));
        let eq166_e2099_d_b40: f64 = (p.p7 * (s.db[264][40] * ddt_scale));
        let eq166_e2099_d_b41: f64 = (p.p7 * (s.db[264][41] * ddt_scale));
        let eq166_e2099_d_b42: f64 = (p.p7 * (s.db[264][42] * ddt_scale));
        let eq166_e2099_d_b43: f64 = (p.p7 * (s.db[264][43] * ddt_scale));
        let eq166_e2099_d_b44: f64 = (p.p7 * (s.db[264][44] * ddt_scale));
        let eq166_e2099_d_b45: f64 = (p.p7 * (s.db[264][45] * ddt_scale));
        let eq166_e2099_d_b46: f64 = (p.p7 * (s.db[264][46] * ddt_scale));
        let eq166_e2099_d_b47: f64 = (p.p7 * (s.db[264][47] * ddt_scale));
        let eq166_e2099_d_b48: f64 = (p.p7 * (s.db[264][48] * ddt_scale));
        let eq166_e2099_d_b49: f64 = (p.p7 * (s.db[264][49] * ddt_scale));
        let eq166_e2099_d_b50: f64 = (p.p7 * (s.db[264][50] * ddt_scale));
        let eq166_e2099_d_b51: f64 = (p.p7 * (s.db[264][51] * ddt_scale));
        let eq166_e2099_d_b52: f64 = (p.p7 * (s.db[264][52] * ddt_scale));
        let eq166_e2099_d_b53: f64 = (p.p7 * (s.db[264][53] * ddt_scale));
        let eq166_e2099_d_b54: f64 = (p.p7 * (s.db[264][54] * ddt_scale));
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
        let eq166_e2101_d_b0: f64 = (eq166_e2099_d_b0 * p.p247);
        let eq166_e2101_d_b1: f64 = (eq166_e2099_d_b1 * p.p247);
        let eq166_e2101_d_b2: f64 = (eq166_e2099_d_b2 * p.p247);
        let eq166_e2101_d_b3: f64 = (eq166_e2099_d_b3 * p.p247);
        let eq166_e2101_d_b4: f64 = (eq166_e2099_d_b4 * p.p247);
        let eq166_e2101_d_b5: f64 = (eq166_e2099_d_b5 * p.p247);
        let eq166_e2101_d_b6: f64 = (eq166_e2099_d_b6 * p.p247);
        let eq166_e2101_d_b7: f64 = (eq166_e2099_d_b7 * p.p247);
        let eq166_e2101_d_b8: f64 = (eq166_e2099_d_b8 * p.p247);
        let eq166_e2101_d_b9: f64 = (eq166_e2099_d_b9 * p.p247);
        let eq166_e2101_d_b10: f64 = (eq166_e2099_d_b10 * p.p247);
        let eq166_e2101_d_b11: f64 = (eq166_e2099_d_b11 * p.p247);
        let eq166_e2101_d_b12: f64 = (eq166_e2099_d_b12 * p.p247);
        let eq166_e2101_d_b13: f64 = (eq166_e2099_d_b13 * p.p247);
        let eq166_e2101_d_b14: f64 = (eq166_e2099_d_b14 * p.p247);
        let eq166_e2101_d_b15: f64 = (eq166_e2099_d_b15 * p.p247);
        let eq166_e2101_d_b16: f64 = (eq166_e2099_d_b16 * p.p247);
        let eq166_e2101_d_b17: f64 = (eq166_e2099_d_b17 * p.p247);
        let eq166_e2101_d_b18: f64 = (eq166_e2099_d_b18 * p.p247);
        let eq166_e2101_d_b19: f64 = (eq166_e2099_d_b19 * p.p247);
        let eq166_e2101_d_b20: f64 = (eq166_e2099_d_b20 * p.p247);
        let eq166_e2101_d_b21: f64 = (eq166_e2099_d_b21 * p.p247);
        let eq166_e2101_d_b22: f64 = (eq166_e2099_d_b22 * p.p247);
        let eq166_e2101_d_b23: f64 = (eq166_e2099_d_b23 * p.p247);
        let eq166_e2101_d_b24: f64 = (eq166_e2099_d_b24 * p.p247);
        let eq166_e2101_d_b25: f64 = (eq166_e2099_d_b25 * p.p247);
        let eq166_e2101_d_b26: f64 = (eq166_e2099_d_b26 * p.p247);
        let eq166_e2101_d_b27: f64 = (eq166_e2099_d_b27 * p.p247);
        let eq166_e2101_d_b28: f64 = (eq166_e2099_d_b28 * p.p247);
        let eq166_e2101_d_b29: f64 = (eq166_e2099_d_b29 * p.p247);
        let eq166_e2101_d_b30: f64 = (eq166_e2099_d_b30 * p.p247);
        let eq166_e2101_d_b31: f64 = (eq166_e2099_d_b31 * p.p247);
        let eq166_e2101_d_b32: f64 = (eq166_e2099_d_b32 * p.p247);
        let eq166_e2101_d_b33: f64 = (eq166_e2099_d_b33 * p.p247);
        let eq166_e2101_d_b34: f64 = (eq166_e2099_d_b34 * p.p247);
        let eq166_e2101_d_b35: f64 = (eq166_e2099_d_b35 * p.p247);
        let eq166_e2101_d_b36: f64 = (eq166_e2099_d_b36 * p.p247);
        let eq166_e2101_d_b37: f64 = (eq166_e2099_d_b37 * p.p247);
        let eq166_e2101_d_b38: f64 = (eq166_e2099_d_b38 * p.p247);
        let eq166_e2101_d_b39: f64 = (eq166_e2099_d_b39 * p.p247);
        let eq166_e2101_d_b40: f64 = (eq166_e2099_d_b40 * p.p247);
        let eq166_e2101_d_b41: f64 = (eq166_e2099_d_b41 * p.p247);
        let eq166_e2101_d_b42: f64 = (eq166_e2099_d_b42 * p.p247);
        let eq166_e2101_d_b43: f64 = (eq166_e2099_d_b43 * p.p247);
        let eq166_e2101_d_b44: f64 = (eq166_e2099_d_b44 * p.p247);
        let eq166_e2101_d_b45: f64 = (eq166_e2099_d_b45 * p.p247);
        let eq166_e2101_d_b46: f64 = (eq166_e2099_d_b46 * p.p247);
        let eq166_e2101_d_b47: f64 = (eq166_e2099_d_b47 * p.p247);
        let eq166_e2101_d_b48: f64 = (eq166_e2099_d_b48 * p.p247);
        let eq166_e2101_d_b49: f64 = (eq166_e2099_d_b49 * p.p247);
        let eq166_e2101_d_b50: f64 = (eq166_e2099_d_b50 * p.p247);
        let eq166_e2101_d_b51: f64 = (eq166_e2099_d_b51 * p.p247);
        let eq166_e2101_d_b52: f64 = (eq166_e2099_d_b52 * p.p247);
        let eq166_e2101_d_b53: f64 = (eq166_e2099_d_b53 * p.p247);
        let eq166_e2101_d_b54: f64 = (eq166_e2099_d_b54 * p.p247);
        (eq166_e2101, eq166_e2101_d_n0, eq166_e2101_d_n1, eq166_e2101_d_n2, eq166_e2101_d_n3, eq166_e2101_d_n4, eq166_e2101_d_n5, eq166_e2101_d_n6, eq166_e2101_d_n7, eq166_e2101_d_n8, eq166_e2101_d_n9, eq166_e2101_d_n10, eq166_e2101_d_n11, eq166_e2101_d_n12, eq166_e2101_d_n13, eq166_e2101_d_n14, eq166_e2101_d_n15, eq166_e2101_d_n16, eq166_e2101_d_n17, eq166_e2101_d_n18, eq166_e2101_d_n19, eq166_e2101_d_n20, eq166_e2101_d_n21, eq166_e2101_d_n22, eq166_e2101_d_b0, eq166_e2101_d_b1, eq166_e2101_d_b2, eq166_e2101_d_b3, eq166_e2101_d_b4, eq166_e2101_d_b5, eq166_e2101_d_b6, eq166_e2101_d_b7, eq166_e2101_d_b8, eq166_e2101_d_b9, eq166_e2101_d_b10, eq166_e2101_d_b11, eq166_e2101_d_b12, eq166_e2101_d_b13, eq166_e2101_d_b14, eq166_e2101_d_b15, eq166_e2101_d_b16, eq166_e2101_d_b17, eq166_e2101_d_b18, eq166_e2101_d_b19, eq166_e2101_d_b20, eq166_e2101_d_b21, eq166_e2101_d_b22, eq166_e2101_d_b23, eq166_e2101_d_b24, eq166_e2101_d_b25, eq166_e2101_d_b26, eq166_e2101_d_b27, eq166_e2101_d_b28, eq166_e2101_d_b29, eq166_e2101_d_b30, eq166_e2101_d_b31, eq166_e2101_d_b32, eq166_e2101_d_b33, eq166_e2101_d_b34, eq166_e2101_d_b35, eq166_e2101_d_b36, eq166_e2101_d_b37, eq166_e2101_d_b38, eq166_e2101_d_b39, eq166_e2101_d_b40, eq166_e2101_d_b41, eq166_e2101_d_b42, eq166_e2101_d_b43, eq166_e2101_d_b44, eq166_e2101_d_b45, eq166_e2101_d_b46, eq166_e2101_d_b47, eq166_e2101_d_b48, eq166_e2101_d_b49, eq166_e2101_d_b50, eq166_e2101_d_b51, eq166_e2101_d_b52, eq166_e2101_d_b53, eq166_e2101_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_value: f64 = eq166_e2103;
        let eq166_node_derivatives: [f64; 23] = [eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n10, eq166_e2103_d_n11, eq166_e2103_d_n12, eq166_e2103_d_n13, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22];
        let eq166_branch_derivatives: [f64; 55] = [eq166_e2103_d_b0, eq166_e2103_d_b1, eq166_e2103_d_b2, eq166_e2103_d_b3, eq166_e2103_d_b4, eq166_e2103_d_b5, eq166_e2103_d_b6, eq166_e2103_d_b7, eq166_e2103_d_b8, eq166_e2103_d_b9, eq166_e2103_d_b10, eq166_e2103_d_b11, eq166_e2103_d_b12, eq166_e2103_d_b13, eq166_e2103_d_b14, eq166_e2103_d_b15, eq166_e2103_d_b16, eq166_e2103_d_b17, eq166_e2103_d_b18, eq166_e2103_d_b19, eq166_e2103_d_b20, eq166_e2103_d_b21, eq166_e2103_d_b22, eq166_e2103_d_b23, eq166_e2103_d_b24, eq166_e2103_d_b25, eq166_e2103_d_b26, eq166_e2103_d_b27, eq166_e2103_d_b28, eq166_e2103_d_b29, eq166_e2103_d_b30, eq166_e2103_d_b31, eq166_e2103_d_b32, eq166_e2103_d_b33, eq166_e2103_d_b34, eq166_e2103_d_b35, eq166_e2103_d_b36, eq166_e2103_d_b37, eq166_e2103_d_b38, eq166_e2103_d_b39, eq166_e2103_d_b40, eq166_e2103_d_b41, eq166_e2103_d_b42, eq166_e2103_d_b43, eq166_e2103_d_b44, eq166_e2103_d_b45, eq166_e2103_d_b46, eq166_e2103_d_b47, eq166_e2103_d_b48, eq166_e2103_d_b49, eq166_e2103_d_b50, eq166_e2103_d_b51, eq166_e2103_d_b52, eq166_e2103_d_b53, eq166_e2103_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq166_value),
            &eq166_node_derivatives,
            &eq166_branch_derivatives,
            multiplicity,
        );
        let (eq167_e2115, eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n10, eq167_e2115_d_n11, eq167_e2115_d_n12, eq167_e2115_d_n13, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22, eq167_e2115_d_b0, eq167_e2115_d_b1, eq167_e2115_d_b2, eq167_e2115_d_b3, eq167_e2115_d_b4, eq167_e2115_d_b5, eq167_e2115_d_b6, eq167_e2115_d_b7, eq167_e2115_d_b8, eq167_e2115_d_b9, eq167_e2115_d_b10, eq167_e2115_d_b11, eq167_e2115_d_b12, eq167_e2115_d_b13, eq167_e2115_d_b14, eq167_e2115_d_b15, eq167_e2115_d_b16, eq167_e2115_d_b17, eq167_e2115_d_b18, eq167_e2115_d_b19, eq167_e2115_d_b20, eq167_e2115_d_b21, eq167_e2115_d_b22, eq167_e2115_d_b23, eq167_e2115_d_b24, eq167_e2115_d_b25, eq167_e2115_d_b26, eq167_e2115_d_b27, eq167_e2115_d_b28, eq167_e2115_d_b29, eq167_e2115_d_b30, eq167_e2115_d_b31, eq167_e2115_d_b32, eq167_e2115_d_b33, eq167_e2115_d_b34, eq167_e2115_d_b35, eq167_e2115_d_b36, eq167_e2115_d_b37, eq167_e2115_d_b38, eq167_e2115_d_b39, eq167_e2115_d_b40, eq167_e2115_d_b41, eq167_e2115_d_b42, eq167_e2115_d_b43, eq167_e2115_d_b44, eq167_e2115_d_b45, eq167_e2115_d_b46, eq167_e2115_d_b47, eq167_e2115_d_b48, eq167_e2115_d_b49, eq167_e2115_d_b50, eq167_e2115_d_b51, eq167_e2115_d_b52, eq167_e2115_d_b53, eq167_e2115_d_b54,) = {
    if ((!s.b[585]) && s.b[588]) {
        let eq167_e2111: f64 = (p.p252 * s.v[264]);
        let eq167_e2112: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 66, eq167_e2111);
        let eq167_e2112_d_n0: f64 = ((p.p252 * s.dn[264][0]) * ddt_scale);
        let eq167_e2112_d_n1: f64 = ((p.p252 * s.dn[264][1]) * ddt_scale);
        let eq167_e2112_d_n2: f64 = ((p.p252 * s.dn[264][2]) * ddt_scale);
        let eq167_e2112_d_n3: f64 = ((p.p252 * s.dn[264][3]) * ddt_scale);
        let eq167_e2112_d_n4: f64 = ((p.p252 * s.dn[264][4]) * ddt_scale);
        let eq167_e2112_d_n5: f64 = ((p.p252 * s.dn[264][5]) * ddt_scale);
        let eq167_e2112_d_n6: f64 = ((p.p252 * s.dn[264][6]) * ddt_scale);
        let eq167_e2112_d_n7: f64 = ((p.p252 * s.dn[264][7]) * ddt_scale);
        let eq167_e2112_d_n8: f64 = ((p.p252 * s.dn[264][8]) * ddt_scale);
        let eq167_e2112_d_n9: f64 = ((p.p252 * s.dn[264][9]) * ddt_scale);
        let eq167_e2112_d_n10: f64 = ((p.p252 * s.dn[264][10]) * ddt_scale);
        let eq167_e2112_d_n11: f64 = ((p.p252 * s.dn[264][11]) * ddt_scale);
        let eq167_e2112_d_n12: f64 = ((p.p252 * s.dn[264][12]) * ddt_scale);
        let eq167_e2112_d_n13: f64 = ((p.p252 * s.dn[264][13]) * ddt_scale);
        let eq167_e2112_d_n14: f64 = ((p.p252 * s.dn[264][14]) * ddt_scale);
        let eq167_e2112_d_n15: f64 = ((p.p252 * s.dn[264][15]) * ddt_scale);
        let eq167_e2112_d_n16: f64 = ((p.p252 * s.dn[264][16]) * ddt_scale);
        let eq167_e2112_d_n17: f64 = ((p.p252 * s.dn[264][17]) * ddt_scale);
        let eq167_e2112_d_n18: f64 = ((p.p252 * s.dn[264][18]) * ddt_scale);
        let eq167_e2112_d_n19: f64 = ((p.p252 * s.dn[264][19]) * ddt_scale);
        let eq167_e2112_d_n20: f64 = ((p.p252 * s.dn[264][20]) * ddt_scale);
        let eq167_e2112_d_n21: f64 = ((p.p252 * s.dn[264][21]) * ddt_scale);
        let eq167_e2112_d_n22: f64 = ((p.p252 * s.dn[264][22]) * ddt_scale);
        let eq167_e2112_d_b0: f64 = ((p.p252 * s.db[264][0]) * ddt_scale);
        let eq167_e2112_d_b1: f64 = ((p.p252 * s.db[264][1]) * ddt_scale);
        let eq167_e2112_d_b2: f64 = ((p.p252 * s.db[264][2]) * ddt_scale);
        let eq167_e2112_d_b3: f64 = ((p.p252 * s.db[264][3]) * ddt_scale);
        let eq167_e2112_d_b4: f64 = ((p.p252 * s.db[264][4]) * ddt_scale);
        let eq167_e2112_d_b5: f64 = ((p.p252 * s.db[264][5]) * ddt_scale);
        let eq167_e2112_d_b6: f64 = ((p.p252 * s.db[264][6]) * ddt_scale);
        let eq167_e2112_d_b7: f64 = ((p.p252 * s.db[264][7]) * ddt_scale);
        let eq167_e2112_d_b8: f64 = ((p.p252 * s.db[264][8]) * ddt_scale);
        let eq167_e2112_d_b9: f64 = ((p.p252 * s.db[264][9]) * ddt_scale);
        let eq167_e2112_d_b10: f64 = ((p.p252 * s.db[264][10]) * ddt_scale);
        let eq167_e2112_d_b11: f64 = ((p.p252 * s.db[264][11]) * ddt_scale);
        let eq167_e2112_d_b12: f64 = ((p.p252 * s.db[264][12]) * ddt_scale);
        let eq167_e2112_d_b13: f64 = ((p.p252 * s.db[264][13]) * ddt_scale);
        let eq167_e2112_d_b14: f64 = ((p.p252 * s.db[264][14]) * ddt_scale);
        let eq167_e2112_d_b15: f64 = ((p.p252 * s.db[264][15]) * ddt_scale);
        let eq167_e2112_d_b16: f64 = ((p.p252 * s.db[264][16]) * ddt_scale);
        let eq167_e2112_d_b17: f64 = ((p.p252 * s.db[264][17]) * ddt_scale);
        let eq167_e2112_d_b18: f64 = ((p.p252 * s.db[264][18]) * ddt_scale);
        let eq167_e2112_d_b19: f64 = ((p.p252 * s.db[264][19]) * ddt_scale);
        let eq167_e2112_d_b20: f64 = ((p.p252 * s.db[264][20]) * ddt_scale);
        let eq167_e2112_d_b21: f64 = ((p.p252 * s.db[264][21]) * ddt_scale);
        let eq167_e2112_d_b22: f64 = ((p.p252 * s.db[264][22]) * ddt_scale);
        let eq167_e2112_d_b23: f64 = ((p.p252 * s.db[264][23]) * ddt_scale);
        let eq167_e2112_d_b24: f64 = ((p.p252 * s.db[264][24]) * ddt_scale);
        let eq167_e2112_d_b25: f64 = ((p.p252 * s.db[264][25]) * ddt_scale);
        let eq167_e2112_d_b26: f64 = ((p.p252 * s.db[264][26]) * ddt_scale);
        let eq167_e2112_d_b27: f64 = ((p.p252 * s.db[264][27]) * ddt_scale);
        let eq167_e2112_d_b28: f64 = ((p.p252 * s.db[264][28]) * ddt_scale);
        let eq167_e2112_d_b29: f64 = ((p.p252 * s.db[264][29]) * ddt_scale);
        let eq167_e2112_d_b30: f64 = ((p.p252 * s.db[264][30]) * ddt_scale);
        let eq167_e2112_d_b31: f64 = ((p.p252 * s.db[264][31]) * ddt_scale);
        let eq167_e2112_d_b32: f64 = ((p.p252 * s.db[264][32]) * ddt_scale);
        let eq167_e2112_d_b33: f64 = ((p.p252 * s.db[264][33]) * ddt_scale);
        let eq167_e2112_d_b34: f64 = ((p.p252 * s.db[264][34]) * ddt_scale);
        let eq167_e2112_d_b35: f64 = ((p.p252 * s.db[264][35]) * ddt_scale);
        let eq167_e2112_d_b36: f64 = ((p.p252 * s.db[264][36]) * ddt_scale);
        let eq167_e2112_d_b37: f64 = ((p.p252 * s.db[264][37]) * ddt_scale);
        let eq167_e2112_d_b38: f64 = ((p.p252 * s.db[264][38]) * ddt_scale);
        let eq167_e2112_d_b39: f64 = ((p.p252 * s.db[264][39]) * ddt_scale);
        let eq167_e2112_d_b40: f64 = ((p.p252 * s.db[264][40]) * ddt_scale);
        let eq167_e2112_d_b41: f64 = ((p.p252 * s.db[264][41]) * ddt_scale);
        let eq167_e2112_d_b42: f64 = ((p.p252 * s.db[264][42]) * ddt_scale);
        let eq167_e2112_d_b43: f64 = ((p.p252 * s.db[264][43]) * ddt_scale);
        let eq167_e2112_d_b44: f64 = ((p.p252 * s.db[264][44]) * ddt_scale);
        let eq167_e2112_d_b45: f64 = ((p.p252 * s.db[264][45]) * ddt_scale);
        let eq167_e2112_d_b46: f64 = ((p.p252 * s.db[264][46]) * ddt_scale);
        let eq167_e2112_d_b47: f64 = ((p.p252 * s.db[264][47]) * ddt_scale);
        let eq167_e2112_d_b48: f64 = ((p.p252 * s.db[264][48]) * ddt_scale);
        let eq167_e2112_d_b49: f64 = ((p.p252 * s.db[264][49]) * ddt_scale);
        let eq167_e2112_d_b50: f64 = ((p.p252 * s.db[264][50]) * ddt_scale);
        let eq167_e2112_d_b51: f64 = ((p.p252 * s.db[264][51]) * ddt_scale);
        let eq167_e2112_d_b52: f64 = ((p.p252 * s.db[264][52]) * ddt_scale);
        let eq167_e2112_d_b53: f64 = ((p.p252 * s.db[264][53]) * ddt_scale);
        let eq167_e2112_d_b54: f64 = ((p.p252 * s.db[264][54]) * ddt_scale);
        let eq167_e2113: f64 = (p.p7 * eq167_e2112);
        let eq167_e2113_d_n0: f64 = (p.p7 * eq167_e2112_d_n0);
        let eq167_e2113_d_n1: f64 = (p.p7 * eq167_e2112_d_n1);
        let eq167_e2113_d_n2: f64 = (p.p7 * eq167_e2112_d_n2);
        let eq167_e2113_d_n3: f64 = (p.p7 * eq167_e2112_d_n3);
        let eq167_e2113_d_n4: f64 = (p.p7 * eq167_e2112_d_n4);
        let eq167_e2113_d_n5: f64 = (p.p7 * eq167_e2112_d_n5);
        let eq167_e2113_d_n6: f64 = (p.p7 * eq167_e2112_d_n6);
        let eq167_e2113_d_n7: f64 = (p.p7 * eq167_e2112_d_n7);
        let eq167_e2113_d_n8: f64 = (p.p7 * eq167_e2112_d_n8);
        let eq167_e2113_d_n9: f64 = (p.p7 * eq167_e2112_d_n9);
        let eq167_e2113_d_n10: f64 = (p.p7 * eq167_e2112_d_n10);
        let eq167_e2113_d_n11: f64 = (p.p7 * eq167_e2112_d_n11);
        let eq167_e2113_d_n12: f64 = (p.p7 * eq167_e2112_d_n12);
        let eq167_e2113_d_n13: f64 = (p.p7 * eq167_e2112_d_n13);
        let eq167_e2113_d_n14: f64 = (p.p7 * eq167_e2112_d_n14);
        let eq167_e2113_d_n15: f64 = (p.p7 * eq167_e2112_d_n15);
        let eq167_e2113_d_n16: f64 = (p.p7 * eq167_e2112_d_n16);
        let eq167_e2113_d_n17: f64 = (p.p7 * eq167_e2112_d_n17);
        let eq167_e2113_d_n18: f64 = (p.p7 * eq167_e2112_d_n18);
        let eq167_e2113_d_n19: f64 = (p.p7 * eq167_e2112_d_n19);
        let eq167_e2113_d_n20: f64 = (p.p7 * eq167_e2112_d_n20);
        let eq167_e2113_d_n21: f64 = (p.p7 * eq167_e2112_d_n21);
        let eq167_e2113_d_n22: f64 = (p.p7 * eq167_e2112_d_n22);
        let eq167_e2113_d_b0: f64 = (p.p7 * eq167_e2112_d_b0);
        let eq167_e2113_d_b1: f64 = (p.p7 * eq167_e2112_d_b1);
        let eq167_e2113_d_b2: f64 = (p.p7 * eq167_e2112_d_b2);
        let eq167_e2113_d_b3: f64 = (p.p7 * eq167_e2112_d_b3);
        let eq167_e2113_d_b4: f64 = (p.p7 * eq167_e2112_d_b4);
        let eq167_e2113_d_b5: f64 = (p.p7 * eq167_e2112_d_b5);
        let eq167_e2113_d_b6: f64 = (p.p7 * eq167_e2112_d_b6);
        let eq167_e2113_d_b7: f64 = (p.p7 * eq167_e2112_d_b7);
        let eq167_e2113_d_b8: f64 = (p.p7 * eq167_e2112_d_b8);
        let eq167_e2113_d_b9: f64 = (p.p7 * eq167_e2112_d_b9);
        let eq167_e2113_d_b10: f64 = (p.p7 * eq167_e2112_d_b10);
        let eq167_e2113_d_b11: f64 = (p.p7 * eq167_e2112_d_b11);
        let eq167_e2113_d_b12: f64 = (p.p7 * eq167_e2112_d_b12);
        let eq167_e2113_d_b13: f64 = (p.p7 * eq167_e2112_d_b13);
        let eq167_e2113_d_b14: f64 = (p.p7 * eq167_e2112_d_b14);
        let eq167_e2113_d_b15: f64 = (p.p7 * eq167_e2112_d_b15);
        let eq167_e2113_d_b16: f64 = (p.p7 * eq167_e2112_d_b16);
        let eq167_e2113_d_b17: f64 = (p.p7 * eq167_e2112_d_b17);
        let eq167_e2113_d_b18: f64 = (p.p7 * eq167_e2112_d_b18);
        let eq167_e2113_d_b19: f64 = (p.p7 * eq167_e2112_d_b19);
        let eq167_e2113_d_b20: f64 = (p.p7 * eq167_e2112_d_b20);
        let eq167_e2113_d_b21: f64 = (p.p7 * eq167_e2112_d_b21);
        let eq167_e2113_d_b22: f64 = (p.p7 * eq167_e2112_d_b22);
        let eq167_e2113_d_b23: f64 = (p.p7 * eq167_e2112_d_b23);
        let eq167_e2113_d_b24: f64 = (p.p7 * eq167_e2112_d_b24);
        let eq167_e2113_d_b25: f64 = (p.p7 * eq167_e2112_d_b25);
        let eq167_e2113_d_b26: f64 = (p.p7 * eq167_e2112_d_b26);
        let eq167_e2113_d_b27: f64 = (p.p7 * eq167_e2112_d_b27);
        let eq167_e2113_d_b28: f64 = (p.p7 * eq167_e2112_d_b28);
        let eq167_e2113_d_b29: f64 = (p.p7 * eq167_e2112_d_b29);
        let eq167_e2113_d_b30: f64 = (p.p7 * eq167_e2112_d_b30);
        let eq167_e2113_d_b31: f64 = (p.p7 * eq167_e2112_d_b31);
        let eq167_e2113_d_b32: f64 = (p.p7 * eq167_e2112_d_b32);
        let eq167_e2113_d_b33: f64 = (p.p7 * eq167_e2112_d_b33);
        let eq167_e2113_d_b34: f64 = (p.p7 * eq167_e2112_d_b34);
        let eq167_e2113_d_b35: f64 = (p.p7 * eq167_e2112_d_b35);
        let eq167_e2113_d_b36: f64 = (p.p7 * eq167_e2112_d_b36);
        let eq167_e2113_d_b37: f64 = (p.p7 * eq167_e2112_d_b37);
        let eq167_e2113_d_b38: f64 = (p.p7 * eq167_e2112_d_b38);
        let eq167_e2113_d_b39: f64 = (p.p7 * eq167_e2112_d_b39);
        let eq167_e2113_d_b40: f64 = (p.p7 * eq167_e2112_d_b40);
        let eq167_e2113_d_b41: f64 = (p.p7 * eq167_e2112_d_b41);
        let eq167_e2113_d_b42: f64 = (p.p7 * eq167_e2112_d_b42);
        let eq167_e2113_d_b43: f64 = (p.p7 * eq167_e2112_d_b43);
        let eq167_e2113_d_b44: f64 = (p.p7 * eq167_e2112_d_b44);
        let eq167_e2113_d_b45: f64 = (p.p7 * eq167_e2112_d_b45);
        let eq167_e2113_d_b46: f64 = (p.p7 * eq167_e2112_d_b46);
        let eq167_e2113_d_b47: f64 = (p.p7 * eq167_e2112_d_b47);
        let eq167_e2113_d_b48: f64 = (p.p7 * eq167_e2112_d_b48);
        let eq167_e2113_d_b49: f64 = (p.p7 * eq167_e2112_d_b49);
        let eq167_e2113_d_b50: f64 = (p.p7 * eq167_e2112_d_b50);
        let eq167_e2113_d_b51: f64 = (p.p7 * eq167_e2112_d_b51);
        let eq167_e2113_d_b52: f64 = (p.p7 * eq167_e2112_d_b52);
        let eq167_e2113_d_b53: f64 = (p.p7 * eq167_e2112_d_b53);
        let eq167_e2113_d_b54: f64 = (p.p7 * eq167_e2112_d_b54);
        (eq167_e2113, eq167_e2113_d_n0, eq167_e2113_d_n1, eq167_e2113_d_n2, eq167_e2113_d_n3, eq167_e2113_d_n4, eq167_e2113_d_n5, eq167_e2113_d_n6, eq167_e2113_d_n7, eq167_e2113_d_n8, eq167_e2113_d_n9, eq167_e2113_d_n10, eq167_e2113_d_n11, eq167_e2113_d_n12, eq167_e2113_d_n13, eq167_e2113_d_n14, eq167_e2113_d_n15, eq167_e2113_d_n16, eq167_e2113_d_n17, eq167_e2113_d_n18, eq167_e2113_d_n19, eq167_e2113_d_n20, eq167_e2113_d_n21, eq167_e2113_d_n22, eq167_e2113_d_b0, eq167_e2113_d_b1, eq167_e2113_d_b2, eq167_e2113_d_b3, eq167_e2113_d_b4, eq167_e2113_d_b5, eq167_e2113_d_b6, eq167_e2113_d_b7, eq167_e2113_d_b8, eq167_e2113_d_b9, eq167_e2113_d_b10, eq167_e2113_d_b11, eq167_e2113_d_b12, eq167_e2113_d_b13, eq167_e2113_d_b14, eq167_e2113_d_b15, eq167_e2113_d_b16, eq167_e2113_d_b17, eq167_e2113_d_b18, eq167_e2113_d_b19, eq167_e2113_d_b20, eq167_e2113_d_b21, eq167_e2113_d_b22, eq167_e2113_d_b23, eq167_e2113_d_b24, eq167_e2113_d_b25, eq167_e2113_d_b26, eq167_e2113_d_b27, eq167_e2113_d_b28, eq167_e2113_d_b29, eq167_e2113_d_b30, eq167_e2113_d_b31, eq167_e2113_d_b32, eq167_e2113_d_b33, eq167_e2113_d_b34, eq167_e2113_d_b35, eq167_e2113_d_b36, eq167_e2113_d_b37, eq167_e2113_d_b38, eq167_e2113_d_b39, eq167_e2113_d_b40, eq167_e2113_d_b41, eq167_e2113_d_b42, eq167_e2113_d_b43, eq167_e2113_d_b44, eq167_e2113_d_b45, eq167_e2113_d_b46, eq167_e2113_d_b47, eq167_e2113_d_b48, eq167_e2113_d_b49, eq167_e2113_d_b50, eq167_e2113_d_b51, eq167_e2113_d_b52, eq167_e2113_d_b53, eq167_e2113_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq167_value: f64 = eq167_e2115;
        let eq167_node_derivatives: [f64; 23] = [eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n10, eq167_e2115_d_n11, eq167_e2115_d_n12, eq167_e2115_d_n13, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22];
        let eq167_branch_derivatives: [f64; 55] = [eq167_e2115_d_b0, eq167_e2115_d_b1, eq167_e2115_d_b2, eq167_e2115_d_b3, eq167_e2115_d_b4, eq167_e2115_d_b5, eq167_e2115_d_b6, eq167_e2115_d_b7, eq167_e2115_d_b8, eq167_e2115_d_b9, eq167_e2115_d_b10, eq167_e2115_d_b11, eq167_e2115_d_b12, eq167_e2115_d_b13, eq167_e2115_d_b14, eq167_e2115_d_b15, eq167_e2115_d_b16, eq167_e2115_d_b17, eq167_e2115_d_b18, eq167_e2115_d_b19, eq167_e2115_d_b20, eq167_e2115_d_b21, eq167_e2115_d_b22, eq167_e2115_d_b23, eq167_e2115_d_b24, eq167_e2115_d_b25, eq167_e2115_d_b26, eq167_e2115_d_b27, eq167_e2115_d_b28, eq167_e2115_d_b29, eq167_e2115_d_b30, eq167_e2115_d_b31, eq167_e2115_d_b32, eq167_e2115_d_b33, eq167_e2115_d_b34, eq167_e2115_d_b35, eq167_e2115_d_b36, eq167_e2115_d_b37, eq167_e2115_d_b38, eq167_e2115_d_b39, eq167_e2115_d_b40, eq167_e2115_d_b41, eq167_e2115_d_b42, eq167_e2115_d_b43, eq167_e2115_d_b44, eq167_e2115_d_b45, eq167_e2115_d_b46, eq167_e2115_d_b47, eq167_e2115_d_b48, eq167_e2115_d_b49, eq167_e2115_d_b50, eq167_e2115_d_b51, eq167_e2115_d_b52, eq167_e2115_d_b53, eq167_e2115_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq167_value),
            &eq167_node_derivatives,
            &eq167_branch_derivatives,
            multiplicity,
        );
        let (eq168_e2124, eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n10, eq168_e2124_d_n11, eq168_e2124_d_n12, eq168_e2124_d_n13, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22, eq168_e2124_d_b0, eq168_e2124_d_b1, eq168_e2124_d_b2, eq168_e2124_d_b3, eq168_e2124_d_b4, eq168_e2124_d_b5, eq168_e2124_d_b6, eq168_e2124_d_b7, eq168_e2124_d_b8, eq168_e2124_d_b9, eq168_e2124_d_b10, eq168_e2124_d_b11, eq168_e2124_d_b12, eq168_e2124_d_b13, eq168_e2124_d_b14, eq168_e2124_d_b15, eq168_e2124_d_b16, eq168_e2124_d_b17, eq168_e2124_d_b18, eq168_e2124_d_b19, eq168_e2124_d_b20, eq168_e2124_d_b21, eq168_e2124_d_b22, eq168_e2124_d_b23, eq168_e2124_d_b24, eq168_e2124_d_b25, eq168_e2124_d_b26, eq168_e2124_d_b27, eq168_e2124_d_b28, eq168_e2124_d_b29, eq168_e2124_d_b30, eq168_e2124_d_b31, eq168_e2124_d_b32, eq168_e2124_d_b33, eq168_e2124_d_b34, eq168_e2124_d_b35, eq168_e2124_d_b36, eq168_e2124_d_b37, eq168_e2124_d_b38, eq168_e2124_d_b39, eq168_e2124_d_b40, eq168_e2124_d_b41, eq168_e2124_d_b42, eq168_e2124_d_b43, eq168_e2124_d_b44, eq168_e2124_d_b45, eq168_e2124_d_b46, eq168_e2124_d_b47, eq168_e2124_d_b48, eq168_e2124_d_b49, eq168_e2124_d_b50, eq168_e2124_d_b51, eq168_e2124_d_b52, eq168_e2124_d_b53, eq168_e2124_d_b54,) = {
    if (s.b[590] && s.b[591]) {
        let eq168_e2121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 67, s.v[277]);
        let eq168_e2122: f64 = (p.p7 * eq168_e2121);
        let eq168_e2122_d_n0: f64 = (p.p7 * (s.dn[277][0] * ddt_scale));
        let eq168_e2122_d_n1: f64 = (p.p7 * (s.dn[277][1] * ddt_scale));
        let eq168_e2122_d_n2: f64 = (p.p7 * (s.dn[277][2] * ddt_scale));
        let eq168_e2122_d_n3: f64 = (p.p7 * (s.dn[277][3] * ddt_scale));
        let eq168_e2122_d_n4: f64 = (p.p7 * (s.dn[277][4] * ddt_scale));
        let eq168_e2122_d_n5: f64 = (p.p7 * (s.dn[277][5] * ddt_scale));
        let eq168_e2122_d_n6: f64 = (p.p7 * (s.dn[277][6] * ddt_scale));
        let eq168_e2122_d_n7: f64 = (p.p7 * (s.dn[277][7] * ddt_scale));
        let eq168_e2122_d_n8: f64 = (p.p7 * (s.dn[277][8] * ddt_scale));
        let eq168_e2122_d_n9: f64 = (p.p7 * (s.dn[277][9] * ddt_scale));
        let eq168_e2122_d_n10: f64 = (p.p7 * (s.dn[277][10] * ddt_scale));
        let eq168_e2122_d_n11: f64 = (p.p7 * (s.dn[277][11] * ddt_scale));
        let eq168_e2122_d_n12: f64 = (p.p7 * (s.dn[277][12] * ddt_scale));
        let eq168_e2122_d_n13: f64 = (p.p7 * (s.dn[277][13] * ddt_scale));
        let eq168_e2122_d_n14: f64 = (p.p7 * (s.dn[277][14] * ddt_scale));
        let eq168_e2122_d_n15: f64 = (p.p7 * (s.dn[277][15] * ddt_scale));
        let eq168_e2122_d_n16: f64 = (p.p7 * (s.dn[277][16] * ddt_scale));
        let eq168_e2122_d_n17: f64 = (p.p7 * (s.dn[277][17] * ddt_scale));
        let eq168_e2122_d_n18: f64 = (p.p7 * (s.dn[277][18] * ddt_scale));
        let eq168_e2122_d_n19: f64 = (p.p7 * (s.dn[277][19] * ddt_scale));
        let eq168_e2122_d_n20: f64 = (p.p7 * (s.dn[277][20] * ddt_scale));
        let eq168_e2122_d_n21: f64 = (p.p7 * (s.dn[277][21] * ddt_scale));
        let eq168_e2122_d_n22: f64 = (p.p7 * (s.dn[277][22] * ddt_scale));
        let eq168_e2122_d_b0: f64 = (p.p7 * (s.db[277][0] * ddt_scale));
        let eq168_e2122_d_b1: f64 = (p.p7 * (s.db[277][1] * ddt_scale));
        let eq168_e2122_d_b2: f64 = (p.p7 * (s.db[277][2] * ddt_scale));
        let eq168_e2122_d_b3: f64 = (p.p7 * (s.db[277][3] * ddt_scale));
        let eq168_e2122_d_b4: f64 = (p.p7 * (s.db[277][4] * ddt_scale));
        let eq168_e2122_d_b5: f64 = (p.p7 * (s.db[277][5] * ddt_scale));
        let eq168_e2122_d_b6: f64 = (p.p7 * (s.db[277][6] * ddt_scale));
        let eq168_e2122_d_b7: f64 = (p.p7 * (s.db[277][7] * ddt_scale));
        let eq168_e2122_d_b8: f64 = (p.p7 * (s.db[277][8] * ddt_scale));
        let eq168_e2122_d_b9: f64 = (p.p7 * (s.db[277][9] * ddt_scale));
        let eq168_e2122_d_b10: f64 = (p.p7 * (s.db[277][10] * ddt_scale));
        let eq168_e2122_d_b11: f64 = (p.p7 * (s.db[277][11] * ddt_scale));
        let eq168_e2122_d_b12: f64 = (p.p7 * (s.db[277][12] * ddt_scale));
        let eq168_e2122_d_b13: f64 = (p.p7 * (s.db[277][13] * ddt_scale));
        let eq168_e2122_d_b14: f64 = (p.p7 * (s.db[277][14] * ddt_scale));
        let eq168_e2122_d_b15: f64 = (p.p7 * (s.db[277][15] * ddt_scale));
        let eq168_e2122_d_b16: f64 = (p.p7 * (s.db[277][16] * ddt_scale));
        let eq168_e2122_d_b17: f64 = (p.p7 * (s.db[277][17] * ddt_scale));
        let eq168_e2122_d_b18: f64 = (p.p7 * (s.db[277][18] * ddt_scale));
        let eq168_e2122_d_b19: f64 = (p.p7 * (s.db[277][19] * ddt_scale));
        let eq168_e2122_d_b20: f64 = (p.p7 * (s.db[277][20] * ddt_scale));
        let eq168_e2122_d_b21: f64 = (p.p7 * (s.db[277][21] * ddt_scale));
        let eq168_e2122_d_b22: f64 = (p.p7 * (s.db[277][22] * ddt_scale));
        let eq168_e2122_d_b23: f64 = (p.p7 * (s.db[277][23] * ddt_scale));
        let eq168_e2122_d_b24: f64 = (p.p7 * (s.db[277][24] * ddt_scale));
        let eq168_e2122_d_b25: f64 = (p.p7 * (s.db[277][25] * ddt_scale));
        let eq168_e2122_d_b26: f64 = (p.p7 * (s.db[277][26] * ddt_scale));
        let eq168_e2122_d_b27: f64 = (p.p7 * (s.db[277][27] * ddt_scale));
        let eq168_e2122_d_b28: f64 = (p.p7 * (s.db[277][28] * ddt_scale));
        let eq168_e2122_d_b29: f64 = (p.p7 * (s.db[277][29] * ddt_scale));
        let eq168_e2122_d_b30: f64 = (p.p7 * (s.db[277][30] * ddt_scale));
        let eq168_e2122_d_b31: f64 = (p.p7 * (s.db[277][31] * ddt_scale));
        let eq168_e2122_d_b32: f64 = (p.p7 * (s.db[277][32] * ddt_scale));
        let eq168_e2122_d_b33: f64 = (p.p7 * (s.db[277][33] * ddt_scale));
        let eq168_e2122_d_b34: f64 = (p.p7 * (s.db[277][34] * ddt_scale));
        let eq168_e2122_d_b35: f64 = (p.p7 * (s.db[277][35] * ddt_scale));
        let eq168_e2122_d_b36: f64 = (p.p7 * (s.db[277][36] * ddt_scale));
        let eq168_e2122_d_b37: f64 = (p.p7 * (s.db[277][37] * ddt_scale));
        let eq168_e2122_d_b38: f64 = (p.p7 * (s.db[277][38] * ddt_scale));
        let eq168_e2122_d_b39: f64 = (p.p7 * (s.db[277][39] * ddt_scale));
        let eq168_e2122_d_b40: f64 = (p.p7 * (s.db[277][40] * ddt_scale));
        let eq168_e2122_d_b41: f64 = (p.p7 * (s.db[277][41] * ddt_scale));
        let eq168_e2122_d_b42: f64 = (p.p7 * (s.db[277][42] * ddt_scale));
        let eq168_e2122_d_b43: f64 = (p.p7 * (s.db[277][43] * ddt_scale));
        let eq168_e2122_d_b44: f64 = (p.p7 * (s.db[277][44] * ddt_scale));
        let eq168_e2122_d_b45: f64 = (p.p7 * (s.db[277][45] * ddt_scale));
        let eq168_e2122_d_b46: f64 = (p.p7 * (s.db[277][46] * ddt_scale));
        let eq168_e2122_d_b47: f64 = (p.p7 * (s.db[277][47] * ddt_scale));
        let eq168_e2122_d_b48: f64 = (p.p7 * (s.db[277][48] * ddt_scale));
        let eq168_e2122_d_b49: f64 = (p.p7 * (s.db[277][49] * ddt_scale));
        let eq168_e2122_d_b50: f64 = (p.p7 * (s.db[277][50] * ddt_scale));
        let eq168_e2122_d_b51: f64 = (p.p7 * (s.db[277][51] * ddt_scale));
        let eq168_e2122_d_b52: f64 = (p.p7 * (s.db[277][52] * ddt_scale));
        let eq168_e2122_d_b53: f64 = (p.p7 * (s.db[277][53] * ddt_scale));
        let eq168_e2122_d_b54: f64 = (p.p7 * (s.db[277][54] * ddt_scale));
        (eq168_e2122, eq168_e2122_d_n0, eq168_e2122_d_n1, eq168_e2122_d_n2, eq168_e2122_d_n3, eq168_e2122_d_n4, eq168_e2122_d_n5, eq168_e2122_d_n6, eq168_e2122_d_n7, eq168_e2122_d_n8, eq168_e2122_d_n9, eq168_e2122_d_n10, eq168_e2122_d_n11, eq168_e2122_d_n12, eq168_e2122_d_n13, eq168_e2122_d_n14, eq168_e2122_d_n15, eq168_e2122_d_n16, eq168_e2122_d_n17, eq168_e2122_d_n18, eq168_e2122_d_n19, eq168_e2122_d_n20, eq168_e2122_d_n21, eq168_e2122_d_n22, eq168_e2122_d_b0, eq168_e2122_d_b1, eq168_e2122_d_b2, eq168_e2122_d_b3, eq168_e2122_d_b4, eq168_e2122_d_b5, eq168_e2122_d_b6, eq168_e2122_d_b7, eq168_e2122_d_b8, eq168_e2122_d_b9, eq168_e2122_d_b10, eq168_e2122_d_b11, eq168_e2122_d_b12, eq168_e2122_d_b13, eq168_e2122_d_b14, eq168_e2122_d_b15, eq168_e2122_d_b16, eq168_e2122_d_b17, eq168_e2122_d_b18, eq168_e2122_d_b19, eq168_e2122_d_b20, eq168_e2122_d_b21, eq168_e2122_d_b22, eq168_e2122_d_b23, eq168_e2122_d_b24, eq168_e2122_d_b25, eq168_e2122_d_b26, eq168_e2122_d_b27, eq168_e2122_d_b28, eq168_e2122_d_b29, eq168_e2122_d_b30, eq168_e2122_d_b31, eq168_e2122_d_b32, eq168_e2122_d_b33, eq168_e2122_d_b34, eq168_e2122_d_b35, eq168_e2122_d_b36, eq168_e2122_d_b37, eq168_e2122_d_b38, eq168_e2122_d_b39, eq168_e2122_d_b40, eq168_e2122_d_b41, eq168_e2122_d_b42, eq168_e2122_d_b43, eq168_e2122_d_b44, eq168_e2122_d_b45, eq168_e2122_d_b46, eq168_e2122_d_b47, eq168_e2122_d_b48, eq168_e2122_d_b49, eq168_e2122_d_b50, eq168_e2122_d_b51, eq168_e2122_d_b52, eq168_e2122_d_b53, eq168_e2122_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq168_value: f64 = eq168_e2124;
        let eq168_node_derivatives: [f64; 23] = [eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n10, eq168_e2124_d_n11, eq168_e2124_d_n12, eq168_e2124_d_n13, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22];
        let eq168_branch_derivatives: [f64; 55] = [eq168_e2124_d_b0, eq168_e2124_d_b1, eq168_e2124_d_b2, eq168_e2124_d_b3, eq168_e2124_d_b4, eq168_e2124_d_b5, eq168_e2124_d_b6, eq168_e2124_d_b7, eq168_e2124_d_b8, eq168_e2124_d_b9, eq168_e2124_d_b10, eq168_e2124_d_b11, eq168_e2124_d_b12, eq168_e2124_d_b13, eq168_e2124_d_b14, eq168_e2124_d_b15, eq168_e2124_d_b16, eq168_e2124_d_b17, eq168_e2124_d_b18, eq168_e2124_d_b19, eq168_e2124_d_b20, eq168_e2124_d_b21, eq168_e2124_d_b22, eq168_e2124_d_b23, eq168_e2124_d_b24, eq168_e2124_d_b25, eq168_e2124_d_b26, eq168_e2124_d_b27, eq168_e2124_d_b28, eq168_e2124_d_b29, eq168_e2124_d_b30, eq168_e2124_d_b31, eq168_e2124_d_b32, eq168_e2124_d_b33, eq168_e2124_d_b34, eq168_e2124_d_b35, eq168_e2124_d_b36, eq168_e2124_d_b37, eq168_e2124_d_b38, eq168_e2124_d_b39, eq168_e2124_d_b40, eq168_e2124_d_b41, eq168_e2124_d_b42, eq168_e2124_d_b43, eq168_e2124_d_b44, eq168_e2124_d_b45, eq168_e2124_d_b46, eq168_e2124_d_b47, eq168_e2124_d_b48, eq168_e2124_d_b49, eq168_e2124_d_b50, eq168_e2124_d_b51, eq168_e2124_d_b52, eq168_e2124_d_b53, eq168_e2124_d_b54];
        stamper.stamp_current_dense_local(
            Some(17),
            Some(16),
            multiplicity * (eq168_value),
            &eq168_node_derivatives,
            &eq168_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_31(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[276][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[276][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[276][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[276][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[276][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[276][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[276][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[276][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[276][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[276][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[276][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[276][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[276][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[276][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[276][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[276][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[276][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[276][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[276][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[276][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[276][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[276][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[276][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[276][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[276][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[276][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[276][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[276][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[276][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[276][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[276][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[276][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[276][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[276][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[276][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[276][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[276][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[276][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[276][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[276][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[276][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[276][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[276][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[276][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[276][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[276][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[276][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[276][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[276][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[276][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[276][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[276][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[276][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[276][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[276][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[276][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[276][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[276][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[276][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[276][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[276][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[276][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[276][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[276][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[276][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[276][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[276][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[276][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[276][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[276][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[276][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[276][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[276][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[276][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[276][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[276][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[276][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[276][54] * ddt_scale));
        let (eq169_e2135, eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, eq169_e2135_d_n10, eq169_e2135_d_n11, eq169_e2135_d_n12, eq169_e2135_d_n13, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22, eq169_e2135_d_b0, eq169_e2135_d_b1, eq169_e2135_d_b2, eq169_e2135_d_b3, eq169_e2135_d_b4, eq169_e2135_d_b5, eq169_e2135_d_b6, eq169_e2135_d_b7, eq169_e2135_d_b8, eq169_e2135_d_b9, eq169_e2135_d_b10, eq169_e2135_d_b11, eq169_e2135_d_b12, eq169_e2135_d_b13, eq169_e2135_d_b14, eq169_e2135_d_b15, eq169_e2135_d_b16, eq169_e2135_d_b17, eq169_e2135_d_b18, eq169_e2135_d_b19, eq169_e2135_d_b20, eq169_e2135_d_b21, eq169_e2135_d_b22, eq169_e2135_d_b23, eq169_e2135_d_b24, eq169_e2135_d_b25, eq169_e2135_d_b26, eq169_e2135_d_b27, eq169_e2135_d_b28, eq169_e2135_d_b29, eq169_e2135_d_b30, eq169_e2135_d_b31, eq169_e2135_d_b32, eq169_e2135_d_b33, eq169_e2135_d_b34, eq169_e2135_d_b35, eq169_e2135_d_b36, eq169_e2135_d_b37, eq169_e2135_d_b38, eq169_e2135_d_b39, eq169_e2135_d_b40, eq169_e2135_d_b41, eq169_e2135_d_b42, eq169_e2135_d_b43, eq169_e2135_d_b44, eq169_e2135_d_b45, eq169_e2135_d_b46, eq169_e2135_d_b47, eq169_e2135_d_b48, eq169_e2135_d_b49, eq169_e2135_d_b50, eq169_e2135_d_b51, eq169_e2135_d_b52, eq169_e2135_d_b53, eq169_e2135_d_b54,) = {
    if ((s.b[590] && s.b[591]) && s.b[592]) {
        let eq169_e2132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 68, s.v[276]);
        let eq169_e2133: f64 = (p.p7 * eq169_e2132);
        (eq169_e2133, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq169_value: f64 = eq169_e2135;
        let eq169_node_derivatives: [f64; 23] = [eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, eq169_e2135_d_n10, eq169_e2135_d_n11, eq169_e2135_d_n12, eq169_e2135_d_n13, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22];
        let eq169_branch_derivatives: [f64; 55] = [eq169_e2135_d_b0, eq169_e2135_d_b1, eq169_e2135_d_b2, eq169_e2135_d_b3, eq169_e2135_d_b4, eq169_e2135_d_b5, eq169_e2135_d_b6, eq169_e2135_d_b7, eq169_e2135_d_b8, eq169_e2135_d_b9, eq169_e2135_d_b10, eq169_e2135_d_b11, eq169_e2135_d_b12, eq169_e2135_d_b13, eq169_e2135_d_b14, eq169_e2135_d_b15, eq169_e2135_d_b16, eq169_e2135_d_b17, eq169_e2135_d_b18, eq169_e2135_d_b19, eq169_e2135_d_b20, eq169_e2135_d_b21, eq169_e2135_d_b22, eq169_e2135_d_b23, eq169_e2135_d_b24, eq169_e2135_d_b25, eq169_e2135_d_b26, eq169_e2135_d_b27, eq169_e2135_d_b28, eq169_e2135_d_b29, eq169_e2135_d_b30, eq169_e2135_d_b31, eq169_e2135_d_b32, eq169_e2135_d_b33, eq169_e2135_d_b34, eq169_e2135_d_b35, eq169_e2135_d_b36, eq169_e2135_d_b37, eq169_e2135_d_b38, eq169_e2135_d_b39, eq169_e2135_d_b40, eq169_e2135_d_b41, eq169_e2135_d_b42, eq169_e2135_d_b43, eq169_e2135_d_b44, eq169_e2135_d_b45, eq169_e2135_d_b46, eq169_e2135_d_b47, eq169_e2135_d_b48, eq169_e2135_d_b49, eq169_e2135_d_b50, eq169_e2135_d_b51, eq169_e2135_d_b52, eq169_e2135_d_b53, eq169_e2135_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(16),
            multiplicity * (eq169_value),
            &eq169_node_derivatives,
            &eq169_branch_derivatives,
            multiplicity,
        );
        let (eq170_e2148, eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, eq170_e2148_d_n10, eq170_e2148_d_n11, eq170_e2148_d_n12, eq170_e2148_d_n13, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22, eq170_e2148_d_b0, eq170_e2148_d_b1, eq170_e2148_d_b2, eq170_e2148_d_b3, eq170_e2148_d_b4, eq170_e2148_d_b5, eq170_e2148_d_b6, eq170_e2148_d_b7, eq170_e2148_d_b8, eq170_e2148_d_b9, eq170_e2148_d_b10, eq170_e2148_d_b11, eq170_e2148_d_b12, eq170_e2148_d_b13, eq170_e2148_d_b14, eq170_e2148_d_b15, eq170_e2148_d_b16, eq170_e2148_d_b17, eq170_e2148_d_b18, eq170_e2148_d_b19, eq170_e2148_d_b20, eq170_e2148_d_b21, eq170_e2148_d_b22, eq170_e2148_d_b23, eq170_e2148_d_b24, eq170_e2148_d_b25, eq170_e2148_d_b26, eq170_e2148_d_b27, eq170_e2148_d_b28, eq170_e2148_d_b29, eq170_e2148_d_b30, eq170_e2148_d_b31, eq170_e2148_d_b32, eq170_e2148_d_b33, eq170_e2148_d_b34, eq170_e2148_d_b35, eq170_e2148_d_b36, eq170_e2148_d_b37, eq170_e2148_d_b38, eq170_e2148_d_b39, eq170_e2148_d_b40, eq170_e2148_d_b41, eq170_e2148_d_b42, eq170_e2148_d_b43, eq170_e2148_d_b44, eq170_e2148_d_b45, eq170_e2148_d_b46, eq170_e2148_d_b47, eq170_e2148_d_b48, eq170_e2148_d_b49, eq170_e2148_d_b50, eq170_e2148_d_b51, eq170_e2148_d_b52, eq170_e2148_d_b53, eq170_e2148_d_b54,) = {
    if ((s.b[590] && s.b[591]) && s.b[592]) {
        let eq170_e2143: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 69, s.v[276]);
        let eq170_e2144: f64 = (p.p7 * eq170_e2143);
        let eq170_e2146: f64 = (eq170_e2144 * p.p248);
        let eq170_e2146_d_n0: f64 = (__rspice_deriv_cse_0 * p.p248);
        let eq170_e2146_d_n1: f64 = (__rspice_deriv_cse_1 * p.p248);
        let eq170_e2146_d_n2: f64 = (__rspice_deriv_cse_2 * p.p248);
        let eq170_e2146_d_n3: f64 = (__rspice_deriv_cse_3 * p.p248);
        let eq170_e2146_d_n4: f64 = (__rspice_deriv_cse_4 * p.p248);
        let eq170_e2146_d_n5: f64 = (__rspice_deriv_cse_5 * p.p248);
        let eq170_e2146_d_n6: f64 = (__rspice_deriv_cse_6 * p.p248);
        let eq170_e2146_d_n7: f64 = (__rspice_deriv_cse_7 * p.p248);
        let eq170_e2146_d_n8: f64 = (__rspice_deriv_cse_8 * p.p248);
        let eq170_e2146_d_n9: f64 = (__rspice_deriv_cse_9 * p.p248);
        let eq170_e2146_d_n10: f64 = (__rspice_deriv_cse_10 * p.p248);
        let eq170_e2146_d_n11: f64 = (__rspice_deriv_cse_11 * p.p248);
        let eq170_e2146_d_n12: f64 = (__rspice_deriv_cse_12 * p.p248);
        let eq170_e2146_d_n13: f64 = (__rspice_deriv_cse_13 * p.p248);
        let eq170_e2146_d_n14: f64 = (__rspice_deriv_cse_14 * p.p248);
        let eq170_e2146_d_n15: f64 = (__rspice_deriv_cse_15 * p.p248);
        let eq170_e2146_d_n16: f64 = (__rspice_deriv_cse_16 * p.p248);
        let eq170_e2146_d_n17: f64 = (__rspice_deriv_cse_17 * p.p248);
        let eq170_e2146_d_n18: f64 = (__rspice_deriv_cse_18 * p.p248);
        let eq170_e2146_d_n19: f64 = (__rspice_deriv_cse_19 * p.p248);
        let eq170_e2146_d_n20: f64 = (__rspice_deriv_cse_20 * p.p248);
        let eq170_e2146_d_n21: f64 = (__rspice_deriv_cse_21 * p.p248);
        let eq170_e2146_d_n22: f64 = (__rspice_deriv_cse_22 * p.p248);
        let eq170_e2146_d_b0: f64 = (__rspice_deriv_cse_23 * p.p248);
        let eq170_e2146_d_b1: f64 = (__rspice_deriv_cse_24 * p.p248);
        let eq170_e2146_d_b2: f64 = (__rspice_deriv_cse_25 * p.p248);
        let eq170_e2146_d_b3: f64 = (__rspice_deriv_cse_26 * p.p248);
        let eq170_e2146_d_b4: f64 = (__rspice_deriv_cse_27 * p.p248);
        let eq170_e2146_d_b5: f64 = (__rspice_deriv_cse_28 * p.p248);
        let eq170_e2146_d_b6: f64 = (__rspice_deriv_cse_29 * p.p248);
        let eq170_e2146_d_b7: f64 = (__rspice_deriv_cse_30 * p.p248);
        let eq170_e2146_d_b8: f64 = (__rspice_deriv_cse_31 * p.p248);
        let eq170_e2146_d_b9: f64 = (__rspice_deriv_cse_32 * p.p248);
        let eq170_e2146_d_b10: f64 = (__rspice_deriv_cse_33 * p.p248);
        let eq170_e2146_d_b11: f64 = (__rspice_deriv_cse_34 * p.p248);
        let eq170_e2146_d_b12: f64 = (__rspice_deriv_cse_35 * p.p248);
        let eq170_e2146_d_b13: f64 = (__rspice_deriv_cse_36 * p.p248);
        let eq170_e2146_d_b14: f64 = (__rspice_deriv_cse_37 * p.p248);
        let eq170_e2146_d_b15: f64 = (__rspice_deriv_cse_38 * p.p248);
        let eq170_e2146_d_b16: f64 = (__rspice_deriv_cse_39 * p.p248);
        let eq170_e2146_d_b17: f64 = (__rspice_deriv_cse_40 * p.p248);
        let eq170_e2146_d_b18: f64 = (__rspice_deriv_cse_41 * p.p248);
        let eq170_e2146_d_b19: f64 = (__rspice_deriv_cse_42 * p.p248);
        let eq170_e2146_d_b20: f64 = (__rspice_deriv_cse_43 * p.p248);
        let eq170_e2146_d_b21: f64 = (__rspice_deriv_cse_44 * p.p248);
        let eq170_e2146_d_b22: f64 = (__rspice_deriv_cse_45 * p.p248);
        let eq170_e2146_d_b23: f64 = (__rspice_deriv_cse_46 * p.p248);
        let eq170_e2146_d_b24: f64 = (__rspice_deriv_cse_47 * p.p248);
        let eq170_e2146_d_b25: f64 = (__rspice_deriv_cse_48 * p.p248);
        let eq170_e2146_d_b26: f64 = (__rspice_deriv_cse_49 * p.p248);
        let eq170_e2146_d_b27: f64 = (__rspice_deriv_cse_50 * p.p248);
        let eq170_e2146_d_b28: f64 = (__rspice_deriv_cse_51 * p.p248);
        let eq170_e2146_d_b29: f64 = (__rspice_deriv_cse_52 * p.p248);
        let eq170_e2146_d_b30: f64 = (__rspice_deriv_cse_53 * p.p248);
        let eq170_e2146_d_b31: f64 = (__rspice_deriv_cse_54 * p.p248);
        let eq170_e2146_d_b32: f64 = (__rspice_deriv_cse_55 * p.p248);
        let eq170_e2146_d_b33: f64 = (__rspice_deriv_cse_56 * p.p248);
        let eq170_e2146_d_b34: f64 = (__rspice_deriv_cse_57 * p.p248);
        let eq170_e2146_d_b35: f64 = (__rspice_deriv_cse_58 * p.p248);
        let eq170_e2146_d_b36: f64 = (__rspice_deriv_cse_59 * p.p248);
        let eq170_e2146_d_b37: f64 = (__rspice_deriv_cse_60 * p.p248);
        let eq170_e2146_d_b38: f64 = (__rspice_deriv_cse_61 * p.p248);
        let eq170_e2146_d_b39: f64 = (__rspice_deriv_cse_62 * p.p248);
        let eq170_e2146_d_b40: f64 = (__rspice_deriv_cse_63 * p.p248);
        let eq170_e2146_d_b41: f64 = (__rspice_deriv_cse_64 * p.p248);
        let eq170_e2146_d_b42: f64 = (__rspice_deriv_cse_65 * p.p248);
        let eq170_e2146_d_b43: f64 = (__rspice_deriv_cse_66 * p.p248);
        let eq170_e2146_d_b44: f64 = (__rspice_deriv_cse_67 * p.p248);
        let eq170_e2146_d_b45: f64 = (__rspice_deriv_cse_68 * p.p248);
        let eq170_e2146_d_b46: f64 = (__rspice_deriv_cse_69 * p.p248);
        let eq170_e2146_d_b47: f64 = (__rspice_deriv_cse_70 * p.p248);
        let eq170_e2146_d_b48: f64 = (__rspice_deriv_cse_71 * p.p248);
        let eq170_e2146_d_b49: f64 = (__rspice_deriv_cse_72 * p.p248);
        let eq170_e2146_d_b50: f64 = (__rspice_deriv_cse_73 * p.p248);
        let eq170_e2146_d_b51: f64 = (__rspice_deriv_cse_74 * p.p248);
        let eq170_e2146_d_b52: f64 = (__rspice_deriv_cse_75 * p.p248);
        let eq170_e2146_d_b53: f64 = (__rspice_deriv_cse_76 * p.p248);
        let eq170_e2146_d_b54: f64 = (__rspice_deriv_cse_77 * p.p248);
        (eq170_e2146, eq170_e2146_d_n0, eq170_e2146_d_n1, eq170_e2146_d_n2, eq170_e2146_d_n3, eq170_e2146_d_n4, eq170_e2146_d_n5, eq170_e2146_d_n6, eq170_e2146_d_n7, eq170_e2146_d_n8, eq170_e2146_d_n9, eq170_e2146_d_n10, eq170_e2146_d_n11, eq170_e2146_d_n12, eq170_e2146_d_n13, eq170_e2146_d_n14, eq170_e2146_d_n15, eq170_e2146_d_n16, eq170_e2146_d_n17, eq170_e2146_d_n18, eq170_e2146_d_n19, eq170_e2146_d_n20, eq170_e2146_d_n21, eq170_e2146_d_n22, eq170_e2146_d_b0, eq170_e2146_d_b1, eq170_e2146_d_b2, eq170_e2146_d_b3, eq170_e2146_d_b4, eq170_e2146_d_b5, eq170_e2146_d_b6, eq170_e2146_d_b7, eq170_e2146_d_b8, eq170_e2146_d_b9, eq170_e2146_d_b10, eq170_e2146_d_b11, eq170_e2146_d_b12, eq170_e2146_d_b13, eq170_e2146_d_b14, eq170_e2146_d_b15, eq170_e2146_d_b16, eq170_e2146_d_b17, eq170_e2146_d_b18, eq170_e2146_d_b19, eq170_e2146_d_b20, eq170_e2146_d_b21, eq170_e2146_d_b22, eq170_e2146_d_b23, eq170_e2146_d_b24, eq170_e2146_d_b25, eq170_e2146_d_b26, eq170_e2146_d_b27, eq170_e2146_d_b28, eq170_e2146_d_b29, eq170_e2146_d_b30, eq170_e2146_d_b31, eq170_e2146_d_b32, eq170_e2146_d_b33, eq170_e2146_d_b34, eq170_e2146_d_b35, eq170_e2146_d_b36, eq170_e2146_d_b37, eq170_e2146_d_b38, eq170_e2146_d_b39, eq170_e2146_d_b40, eq170_e2146_d_b41, eq170_e2146_d_b42, eq170_e2146_d_b43, eq170_e2146_d_b44, eq170_e2146_d_b45, eq170_e2146_d_b46, eq170_e2146_d_b47, eq170_e2146_d_b48, eq170_e2146_d_b49, eq170_e2146_d_b50, eq170_e2146_d_b51, eq170_e2146_d_b52, eq170_e2146_d_b53, eq170_e2146_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq170_value: f64 = eq170_e2148;
        let eq170_node_derivatives: [f64; 23] = [eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, eq170_e2148_d_n10, eq170_e2148_d_n11, eq170_e2148_d_n12, eq170_e2148_d_n13, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22];
        let eq170_branch_derivatives: [f64; 55] = [eq170_e2148_d_b0, eq170_e2148_d_b1, eq170_e2148_d_b2, eq170_e2148_d_b3, eq170_e2148_d_b4, eq170_e2148_d_b5, eq170_e2148_d_b6, eq170_e2148_d_b7, eq170_e2148_d_b8, eq170_e2148_d_b9, eq170_e2148_d_b10, eq170_e2148_d_b11, eq170_e2148_d_b12, eq170_e2148_d_b13, eq170_e2148_d_b14, eq170_e2148_d_b15, eq170_e2148_d_b16, eq170_e2148_d_b17, eq170_e2148_d_b18, eq170_e2148_d_b19, eq170_e2148_d_b20, eq170_e2148_d_b21, eq170_e2148_d_b22, eq170_e2148_d_b23, eq170_e2148_d_b24, eq170_e2148_d_b25, eq170_e2148_d_b26, eq170_e2148_d_b27, eq170_e2148_d_b28, eq170_e2148_d_b29, eq170_e2148_d_b30, eq170_e2148_d_b31, eq170_e2148_d_b32, eq170_e2148_d_b33, eq170_e2148_d_b34, eq170_e2148_d_b35, eq170_e2148_d_b36, eq170_e2148_d_b37, eq170_e2148_d_b38, eq170_e2148_d_b39, eq170_e2148_d_b40, eq170_e2148_d_b41, eq170_e2148_d_b42, eq170_e2148_d_b43, eq170_e2148_d_b44, eq170_e2148_d_b45, eq170_e2148_d_b46, eq170_e2148_d_b47, eq170_e2148_d_b48, eq170_e2148_d_b49, eq170_e2148_d_b50, eq170_e2148_d_b51, eq170_e2148_d_b52, eq170_e2148_d_b53, eq170_e2148_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(16),
            multiplicity * (eq170_value),
            &eq170_node_derivatives,
            &eq170_branch_derivatives,
            multiplicity,
        );
        let (eq171_e2160, eq171_e2160_d_n0, eq171_e2160_d_n1, eq171_e2160_d_n2, eq171_e2160_d_n3, eq171_e2160_d_n4, eq171_e2160_d_n5, eq171_e2160_d_n6, eq171_e2160_d_n7, eq171_e2160_d_n8, eq171_e2160_d_n9, eq171_e2160_d_n10, eq171_e2160_d_n11, eq171_e2160_d_n12, eq171_e2160_d_n13, eq171_e2160_d_n14, eq171_e2160_d_n15, eq171_e2160_d_n16, eq171_e2160_d_n17, eq171_e2160_d_n18, eq171_e2160_d_n19, eq171_e2160_d_n20, eq171_e2160_d_n21, eq171_e2160_d_n22, eq171_e2160_d_b0, eq171_e2160_d_b1, eq171_e2160_d_b2, eq171_e2160_d_b3, eq171_e2160_d_b4, eq171_e2160_d_b5, eq171_e2160_d_b6, eq171_e2160_d_b7, eq171_e2160_d_b8, eq171_e2160_d_b9, eq171_e2160_d_b10, eq171_e2160_d_b11, eq171_e2160_d_b12, eq171_e2160_d_b13, eq171_e2160_d_b14, eq171_e2160_d_b15, eq171_e2160_d_b16, eq171_e2160_d_b17, eq171_e2160_d_b18, eq171_e2160_d_b19, eq171_e2160_d_b20, eq171_e2160_d_b21, eq171_e2160_d_b22, eq171_e2160_d_b23, eq171_e2160_d_b24, eq171_e2160_d_b25, eq171_e2160_d_b26, eq171_e2160_d_b27, eq171_e2160_d_b28, eq171_e2160_d_b29, eq171_e2160_d_b30, eq171_e2160_d_b31, eq171_e2160_d_b32, eq171_e2160_d_b33, eq171_e2160_d_b34, eq171_e2160_d_b35, eq171_e2160_d_b36, eq171_e2160_d_b37, eq171_e2160_d_b38, eq171_e2160_d_b39, eq171_e2160_d_b40, eq171_e2160_d_b41, eq171_e2160_d_b42, eq171_e2160_d_b43, eq171_e2160_d_b44, eq171_e2160_d_b45, eq171_e2160_d_b46, eq171_e2160_d_b47, eq171_e2160_d_b48, eq171_e2160_d_b49, eq171_e2160_d_b50, eq171_e2160_d_b51, eq171_e2160_d_b52, eq171_e2160_d_b53, eq171_e2160_d_b54,) = {
    if ((s.b[590] && s.b[591]) && (!s.b[592])) {
        let eq171_e2157: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 70, s.v[276]);
        let eq171_e2158: f64 = (p.p7 * eq171_e2157);
        (eq171_e2158, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq171_value: f64 = eq171_e2160;
        let eq171_node_derivatives: [f64; 23] = [eq171_e2160_d_n0, eq171_e2160_d_n1, eq171_e2160_d_n2, eq171_e2160_d_n3, eq171_e2160_d_n4, eq171_e2160_d_n5, eq171_e2160_d_n6, eq171_e2160_d_n7, eq171_e2160_d_n8, eq171_e2160_d_n9, eq171_e2160_d_n10, eq171_e2160_d_n11, eq171_e2160_d_n12, eq171_e2160_d_n13, eq171_e2160_d_n14, eq171_e2160_d_n15, eq171_e2160_d_n16, eq171_e2160_d_n17, eq171_e2160_d_n18, eq171_e2160_d_n19, eq171_e2160_d_n20, eq171_e2160_d_n21, eq171_e2160_d_n22];
        let eq171_branch_derivatives: [f64; 55] = [eq171_e2160_d_b0, eq171_e2160_d_b1, eq171_e2160_d_b2, eq171_e2160_d_b3, eq171_e2160_d_b4, eq171_e2160_d_b5, eq171_e2160_d_b6, eq171_e2160_d_b7, eq171_e2160_d_b8, eq171_e2160_d_b9, eq171_e2160_d_b10, eq171_e2160_d_b11, eq171_e2160_d_b12, eq171_e2160_d_b13, eq171_e2160_d_b14, eq171_e2160_d_b15, eq171_e2160_d_b16, eq171_e2160_d_b17, eq171_e2160_d_b18, eq171_e2160_d_b19, eq171_e2160_d_b20, eq171_e2160_d_b21, eq171_e2160_d_b22, eq171_e2160_d_b23, eq171_e2160_d_b24, eq171_e2160_d_b25, eq171_e2160_d_b26, eq171_e2160_d_b27, eq171_e2160_d_b28, eq171_e2160_d_b29, eq171_e2160_d_b30, eq171_e2160_d_b31, eq171_e2160_d_b32, eq171_e2160_d_b33, eq171_e2160_d_b34, eq171_e2160_d_b35, eq171_e2160_d_b36, eq171_e2160_d_b37, eq171_e2160_d_b38, eq171_e2160_d_b39, eq171_e2160_d_b40, eq171_e2160_d_b41, eq171_e2160_d_b42, eq171_e2160_d_b43, eq171_e2160_d_b44, eq171_e2160_d_b45, eq171_e2160_d_b46, eq171_e2160_d_b47, eq171_e2160_d_b48, eq171_e2160_d_b49, eq171_e2160_d_b50, eq171_e2160_d_b51, eq171_e2160_d_b52, eq171_e2160_d_b53, eq171_e2160_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(16),
            multiplicity * (eq171_value),
            &eq171_node_derivatives,
            &eq171_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_32(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq172_e2174, eq172_e2174_d_n0, eq172_e2174_d_n1, eq172_e2174_d_n2, eq172_e2174_d_n3, eq172_e2174_d_n4, eq172_e2174_d_n5, eq172_e2174_d_n6, eq172_e2174_d_n7, eq172_e2174_d_n8, eq172_e2174_d_n9, eq172_e2174_d_n10, eq172_e2174_d_n11, eq172_e2174_d_n12, eq172_e2174_d_n13, eq172_e2174_d_n14, eq172_e2174_d_n15, eq172_e2174_d_n16, eq172_e2174_d_n17, eq172_e2174_d_n18, eq172_e2174_d_n19, eq172_e2174_d_n20, eq172_e2174_d_n21, eq172_e2174_d_n22, eq172_e2174_d_b0, eq172_e2174_d_b1, eq172_e2174_d_b2, eq172_e2174_d_b3, eq172_e2174_d_b4, eq172_e2174_d_b5, eq172_e2174_d_b6, eq172_e2174_d_b7, eq172_e2174_d_b8, eq172_e2174_d_b9, eq172_e2174_d_b10, eq172_e2174_d_b11, eq172_e2174_d_b12, eq172_e2174_d_b13, eq172_e2174_d_b14, eq172_e2174_d_b15, eq172_e2174_d_b16, eq172_e2174_d_b17, eq172_e2174_d_b18, eq172_e2174_d_b19, eq172_e2174_d_b20, eq172_e2174_d_b21, eq172_e2174_d_b22, eq172_e2174_d_b23, eq172_e2174_d_b24, eq172_e2174_d_b25, eq172_e2174_d_b26, eq172_e2174_d_b27, eq172_e2174_d_b28, eq172_e2174_d_b29, eq172_e2174_d_b30, eq172_e2174_d_b31, eq172_e2174_d_b32, eq172_e2174_d_b33, eq172_e2174_d_b34, eq172_e2174_d_b35, eq172_e2174_d_b36, eq172_e2174_d_b37, eq172_e2174_d_b38, eq172_e2174_d_b39, eq172_e2174_d_b40, eq172_e2174_d_b41, eq172_e2174_d_b42, eq172_e2174_d_b43, eq172_e2174_d_b44, eq172_e2174_d_b45, eq172_e2174_d_b46, eq172_e2174_d_b47, eq172_e2174_d_b48, eq172_e2174_d_b49, eq172_e2174_d_b50, eq172_e2174_d_b51, eq172_e2174_d_b52, eq172_e2174_d_b53, eq172_e2174_d_b54,) = {
    if ((s.b[590] && s.b[591]) && (!s.b[592])) {
        let eq172_e2169: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 71, s.v[276]);
        let eq172_e2170: f64 = (p.p7 * eq172_e2169);
        let eq172_e2170_d_n0: f64 = (p.p7 * (s.dn[276][0] * ddt_scale));
        let eq172_e2170_d_n1: f64 = (p.p7 * (s.dn[276][1] * ddt_scale));
        let eq172_e2170_d_n2: f64 = (p.p7 * (s.dn[276][2] * ddt_scale));
        let eq172_e2170_d_n3: f64 = (p.p7 * (s.dn[276][3] * ddt_scale));
        let eq172_e2170_d_n4: f64 = (p.p7 * (s.dn[276][4] * ddt_scale));
        let eq172_e2170_d_n5: f64 = (p.p7 * (s.dn[276][5] * ddt_scale));
        let eq172_e2170_d_n6: f64 = (p.p7 * (s.dn[276][6] * ddt_scale));
        let eq172_e2170_d_n7: f64 = (p.p7 * (s.dn[276][7] * ddt_scale));
        let eq172_e2170_d_n8: f64 = (p.p7 * (s.dn[276][8] * ddt_scale));
        let eq172_e2170_d_n9: f64 = (p.p7 * (s.dn[276][9] * ddt_scale));
        let eq172_e2170_d_n10: f64 = (p.p7 * (s.dn[276][10] * ddt_scale));
        let eq172_e2170_d_n11: f64 = (p.p7 * (s.dn[276][11] * ddt_scale));
        let eq172_e2170_d_n12: f64 = (p.p7 * (s.dn[276][12] * ddt_scale));
        let eq172_e2170_d_n13: f64 = (p.p7 * (s.dn[276][13] * ddt_scale));
        let eq172_e2170_d_n14: f64 = (p.p7 * (s.dn[276][14] * ddt_scale));
        let eq172_e2170_d_n15: f64 = (p.p7 * (s.dn[276][15] * ddt_scale));
        let eq172_e2170_d_n16: f64 = (p.p7 * (s.dn[276][16] * ddt_scale));
        let eq172_e2170_d_n17: f64 = (p.p7 * (s.dn[276][17] * ddt_scale));
        let eq172_e2170_d_n18: f64 = (p.p7 * (s.dn[276][18] * ddt_scale));
        let eq172_e2170_d_n19: f64 = (p.p7 * (s.dn[276][19] * ddt_scale));
        let eq172_e2170_d_n20: f64 = (p.p7 * (s.dn[276][20] * ddt_scale));
        let eq172_e2170_d_n21: f64 = (p.p7 * (s.dn[276][21] * ddt_scale));
        let eq172_e2170_d_n22: f64 = (p.p7 * (s.dn[276][22] * ddt_scale));
        let eq172_e2170_d_b0: f64 = (p.p7 * (s.db[276][0] * ddt_scale));
        let eq172_e2170_d_b1: f64 = (p.p7 * (s.db[276][1] * ddt_scale));
        let eq172_e2170_d_b2: f64 = (p.p7 * (s.db[276][2] * ddt_scale));
        let eq172_e2170_d_b3: f64 = (p.p7 * (s.db[276][3] * ddt_scale));
        let eq172_e2170_d_b4: f64 = (p.p7 * (s.db[276][4] * ddt_scale));
        let eq172_e2170_d_b5: f64 = (p.p7 * (s.db[276][5] * ddt_scale));
        let eq172_e2170_d_b6: f64 = (p.p7 * (s.db[276][6] * ddt_scale));
        let eq172_e2170_d_b7: f64 = (p.p7 * (s.db[276][7] * ddt_scale));
        let eq172_e2170_d_b8: f64 = (p.p7 * (s.db[276][8] * ddt_scale));
        let eq172_e2170_d_b9: f64 = (p.p7 * (s.db[276][9] * ddt_scale));
        let eq172_e2170_d_b10: f64 = (p.p7 * (s.db[276][10] * ddt_scale));
        let eq172_e2170_d_b11: f64 = (p.p7 * (s.db[276][11] * ddt_scale));
        let eq172_e2170_d_b12: f64 = (p.p7 * (s.db[276][12] * ddt_scale));
        let eq172_e2170_d_b13: f64 = (p.p7 * (s.db[276][13] * ddt_scale));
        let eq172_e2170_d_b14: f64 = (p.p7 * (s.db[276][14] * ddt_scale));
        let eq172_e2170_d_b15: f64 = (p.p7 * (s.db[276][15] * ddt_scale));
        let eq172_e2170_d_b16: f64 = (p.p7 * (s.db[276][16] * ddt_scale));
        let eq172_e2170_d_b17: f64 = (p.p7 * (s.db[276][17] * ddt_scale));
        let eq172_e2170_d_b18: f64 = (p.p7 * (s.db[276][18] * ddt_scale));
        let eq172_e2170_d_b19: f64 = (p.p7 * (s.db[276][19] * ddt_scale));
        let eq172_e2170_d_b20: f64 = (p.p7 * (s.db[276][20] * ddt_scale));
        let eq172_e2170_d_b21: f64 = (p.p7 * (s.db[276][21] * ddt_scale));
        let eq172_e2170_d_b22: f64 = (p.p7 * (s.db[276][22] * ddt_scale));
        let eq172_e2170_d_b23: f64 = (p.p7 * (s.db[276][23] * ddt_scale));
        let eq172_e2170_d_b24: f64 = (p.p7 * (s.db[276][24] * ddt_scale));
        let eq172_e2170_d_b25: f64 = (p.p7 * (s.db[276][25] * ddt_scale));
        let eq172_e2170_d_b26: f64 = (p.p7 * (s.db[276][26] * ddt_scale));
        let eq172_e2170_d_b27: f64 = (p.p7 * (s.db[276][27] * ddt_scale));
        let eq172_e2170_d_b28: f64 = (p.p7 * (s.db[276][28] * ddt_scale));
        let eq172_e2170_d_b29: f64 = (p.p7 * (s.db[276][29] * ddt_scale));
        let eq172_e2170_d_b30: f64 = (p.p7 * (s.db[276][30] * ddt_scale));
        let eq172_e2170_d_b31: f64 = (p.p7 * (s.db[276][31] * ddt_scale));
        let eq172_e2170_d_b32: f64 = (p.p7 * (s.db[276][32] * ddt_scale));
        let eq172_e2170_d_b33: f64 = (p.p7 * (s.db[276][33] * ddt_scale));
        let eq172_e2170_d_b34: f64 = (p.p7 * (s.db[276][34] * ddt_scale));
        let eq172_e2170_d_b35: f64 = (p.p7 * (s.db[276][35] * ddt_scale));
        let eq172_e2170_d_b36: f64 = (p.p7 * (s.db[276][36] * ddt_scale));
        let eq172_e2170_d_b37: f64 = (p.p7 * (s.db[276][37] * ddt_scale));
        let eq172_e2170_d_b38: f64 = (p.p7 * (s.db[276][38] * ddt_scale));
        let eq172_e2170_d_b39: f64 = (p.p7 * (s.db[276][39] * ddt_scale));
        let eq172_e2170_d_b40: f64 = (p.p7 * (s.db[276][40] * ddt_scale));
        let eq172_e2170_d_b41: f64 = (p.p7 * (s.db[276][41] * ddt_scale));
        let eq172_e2170_d_b42: f64 = (p.p7 * (s.db[276][42] * ddt_scale));
        let eq172_e2170_d_b43: f64 = (p.p7 * (s.db[276][43] * ddt_scale));
        let eq172_e2170_d_b44: f64 = (p.p7 * (s.db[276][44] * ddt_scale));
        let eq172_e2170_d_b45: f64 = (p.p7 * (s.db[276][45] * ddt_scale));
        let eq172_e2170_d_b46: f64 = (p.p7 * (s.db[276][46] * ddt_scale));
        let eq172_e2170_d_b47: f64 = (p.p7 * (s.db[276][47] * ddt_scale));
        let eq172_e2170_d_b48: f64 = (p.p7 * (s.db[276][48] * ddt_scale));
        let eq172_e2170_d_b49: f64 = (p.p7 * (s.db[276][49] * ddt_scale));
        let eq172_e2170_d_b50: f64 = (p.p7 * (s.db[276][50] * ddt_scale));
        let eq172_e2170_d_b51: f64 = (p.p7 * (s.db[276][51] * ddt_scale));
        let eq172_e2170_d_b52: f64 = (p.p7 * (s.db[276][52] * ddt_scale));
        let eq172_e2170_d_b53: f64 = (p.p7 * (s.db[276][53] * ddt_scale));
        let eq172_e2170_d_b54: f64 = (p.p7 * (s.db[276][54] * ddt_scale));
        let eq172_e2172: f64 = (eq172_e2170 * p.p248);
        let eq172_e2172_d_n0: f64 = (eq172_e2170_d_n0 * p.p248);
        let eq172_e2172_d_n1: f64 = (eq172_e2170_d_n1 * p.p248);
        let eq172_e2172_d_n2: f64 = (eq172_e2170_d_n2 * p.p248);
        let eq172_e2172_d_n3: f64 = (eq172_e2170_d_n3 * p.p248);
        let eq172_e2172_d_n4: f64 = (eq172_e2170_d_n4 * p.p248);
        let eq172_e2172_d_n5: f64 = (eq172_e2170_d_n5 * p.p248);
        let eq172_e2172_d_n6: f64 = (eq172_e2170_d_n6 * p.p248);
        let eq172_e2172_d_n7: f64 = (eq172_e2170_d_n7 * p.p248);
        let eq172_e2172_d_n8: f64 = (eq172_e2170_d_n8 * p.p248);
        let eq172_e2172_d_n9: f64 = (eq172_e2170_d_n9 * p.p248);
        let eq172_e2172_d_n10: f64 = (eq172_e2170_d_n10 * p.p248);
        let eq172_e2172_d_n11: f64 = (eq172_e2170_d_n11 * p.p248);
        let eq172_e2172_d_n12: f64 = (eq172_e2170_d_n12 * p.p248);
        let eq172_e2172_d_n13: f64 = (eq172_e2170_d_n13 * p.p248);
        let eq172_e2172_d_n14: f64 = (eq172_e2170_d_n14 * p.p248);
        let eq172_e2172_d_n15: f64 = (eq172_e2170_d_n15 * p.p248);
        let eq172_e2172_d_n16: f64 = (eq172_e2170_d_n16 * p.p248);
        let eq172_e2172_d_n17: f64 = (eq172_e2170_d_n17 * p.p248);
        let eq172_e2172_d_n18: f64 = (eq172_e2170_d_n18 * p.p248);
        let eq172_e2172_d_n19: f64 = (eq172_e2170_d_n19 * p.p248);
        let eq172_e2172_d_n20: f64 = (eq172_e2170_d_n20 * p.p248);
        let eq172_e2172_d_n21: f64 = (eq172_e2170_d_n21 * p.p248);
        let eq172_e2172_d_n22: f64 = (eq172_e2170_d_n22 * p.p248);
        let eq172_e2172_d_b0: f64 = (eq172_e2170_d_b0 * p.p248);
        let eq172_e2172_d_b1: f64 = (eq172_e2170_d_b1 * p.p248);
        let eq172_e2172_d_b2: f64 = (eq172_e2170_d_b2 * p.p248);
        let eq172_e2172_d_b3: f64 = (eq172_e2170_d_b3 * p.p248);
        let eq172_e2172_d_b4: f64 = (eq172_e2170_d_b4 * p.p248);
        let eq172_e2172_d_b5: f64 = (eq172_e2170_d_b5 * p.p248);
        let eq172_e2172_d_b6: f64 = (eq172_e2170_d_b6 * p.p248);
        let eq172_e2172_d_b7: f64 = (eq172_e2170_d_b7 * p.p248);
        let eq172_e2172_d_b8: f64 = (eq172_e2170_d_b8 * p.p248);
        let eq172_e2172_d_b9: f64 = (eq172_e2170_d_b9 * p.p248);
        let eq172_e2172_d_b10: f64 = (eq172_e2170_d_b10 * p.p248);
        let eq172_e2172_d_b11: f64 = (eq172_e2170_d_b11 * p.p248);
        let eq172_e2172_d_b12: f64 = (eq172_e2170_d_b12 * p.p248);
        let eq172_e2172_d_b13: f64 = (eq172_e2170_d_b13 * p.p248);
        let eq172_e2172_d_b14: f64 = (eq172_e2170_d_b14 * p.p248);
        let eq172_e2172_d_b15: f64 = (eq172_e2170_d_b15 * p.p248);
        let eq172_e2172_d_b16: f64 = (eq172_e2170_d_b16 * p.p248);
        let eq172_e2172_d_b17: f64 = (eq172_e2170_d_b17 * p.p248);
        let eq172_e2172_d_b18: f64 = (eq172_e2170_d_b18 * p.p248);
        let eq172_e2172_d_b19: f64 = (eq172_e2170_d_b19 * p.p248);
        let eq172_e2172_d_b20: f64 = (eq172_e2170_d_b20 * p.p248);
        let eq172_e2172_d_b21: f64 = (eq172_e2170_d_b21 * p.p248);
        let eq172_e2172_d_b22: f64 = (eq172_e2170_d_b22 * p.p248);
        let eq172_e2172_d_b23: f64 = (eq172_e2170_d_b23 * p.p248);
        let eq172_e2172_d_b24: f64 = (eq172_e2170_d_b24 * p.p248);
        let eq172_e2172_d_b25: f64 = (eq172_e2170_d_b25 * p.p248);
        let eq172_e2172_d_b26: f64 = (eq172_e2170_d_b26 * p.p248);
        let eq172_e2172_d_b27: f64 = (eq172_e2170_d_b27 * p.p248);
        let eq172_e2172_d_b28: f64 = (eq172_e2170_d_b28 * p.p248);
        let eq172_e2172_d_b29: f64 = (eq172_e2170_d_b29 * p.p248);
        let eq172_e2172_d_b30: f64 = (eq172_e2170_d_b30 * p.p248);
        let eq172_e2172_d_b31: f64 = (eq172_e2170_d_b31 * p.p248);
        let eq172_e2172_d_b32: f64 = (eq172_e2170_d_b32 * p.p248);
        let eq172_e2172_d_b33: f64 = (eq172_e2170_d_b33 * p.p248);
        let eq172_e2172_d_b34: f64 = (eq172_e2170_d_b34 * p.p248);
        let eq172_e2172_d_b35: f64 = (eq172_e2170_d_b35 * p.p248);
        let eq172_e2172_d_b36: f64 = (eq172_e2170_d_b36 * p.p248);
        let eq172_e2172_d_b37: f64 = (eq172_e2170_d_b37 * p.p248);
        let eq172_e2172_d_b38: f64 = (eq172_e2170_d_b38 * p.p248);
        let eq172_e2172_d_b39: f64 = (eq172_e2170_d_b39 * p.p248);
        let eq172_e2172_d_b40: f64 = (eq172_e2170_d_b40 * p.p248);
        let eq172_e2172_d_b41: f64 = (eq172_e2170_d_b41 * p.p248);
        let eq172_e2172_d_b42: f64 = (eq172_e2170_d_b42 * p.p248);
        let eq172_e2172_d_b43: f64 = (eq172_e2170_d_b43 * p.p248);
        let eq172_e2172_d_b44: f64 = (eq172_e2170_d_b44 * p.p248);
        let eq172_e2172_d_b45: f64 = (eq172_e2170_d_b45 * p.p248);
        let eq172_e2172_d_b46: f64 = (eq172_e2170_d_b46 * p.p248);
        let eq172_e2172_d_b47: f64 = (eq172_e2170_d_b47 * p.p248);
        let eq172_e2172_d_b48: f64 = (eq172_e2170_d_b48 * p.p248);
        let eq172_e2172_d_b49: f64 = (eq172_e2170_d_b49 * p.p248);
        let eq172_e2172_d_b50: f64 = (eq172_e2170_d_b50 * p.p248);
        let eq172_e2172_d_b51: f64 = (eq172_e2170_d_b51 * p.p248);
        let eq172_e2172_d_b52: f64 = (eq172_e2170_d_b52 * p.p248);
        let eq172_e2172_d_b53: f64 = (eq172_e2170_d_b53 * p.p248);
        let eq172_e2172_d_b54: f64 = (eq172_e2170_d_b54 * p.p248);
        (eq172_e2172, eq172_e2172_d_n0, eq172_e2172_d_n1, eq172_e2172_d_n2, eq172_e2172_d_n3, eq172_e2172_d_n4, eq172_e2172_d_n5, eq172_e2172_d_n6, eq172_e2172_d_n7, eq172_e2172_d_n8, eq172_e2172_d_n9, eq172_e2172_d_n10, eq172_e2172_d_n11, eq172_e2172_d_n12, eq172_e2172_d_n13, eq172_e2172_d_n14, eq172_e2172_d_n15, eq172_e2172_d_n16, eq172_e2172_d_n17, eq172_e2172_d_n18, eq172_e2172_d_n19, eq172_e2172_d_n20, eq172_e2172_d_n21, eq172_e2172_d_n22, eq172_e2172_d_b0, eq172_e2172_d_b1, eq172_e2172_d_b2, eq172_e2172_d_b3, eq172_e2172_d_b4, eq172_e2172_d_b5, eq172_e2172_d_b6, eq172_e2172_d_b7, eq172_e2172_d_b8, eq172_e2172_d_b9, eq172_e2172_d_b10, eq172_e2172_d_b11, eq172_e2172_d_b12, eq172_e2172_d_b13, eq172_e2172_d_b14, eq172_e2172_d_b15, eq172_e2172_d_b16, eq172_e2172_d_b17, eq172_e2172_d_b18, eq172_e2172_d_b19, eq172_e2172_d_b20, eq172_e2172_d_b21, eq172_e2172_d_b22, eq172_e2172_d_b23, eq172_e2172_d_b24, eq172_e2172_d_b25, eq172_e2172_d_b26, eq172_e2172_d_b27, eq172_e2172_d_b28, eq172_e2172_d_b29, eq172_e2172_d_b30, eq172_e2172_d_b31, eq172_e2172_d_b32, eq172_e2172_d_b33, eq172_e2172_d_b34, eq172_e2172_d_b35, eq172_e2172_d_b36, eq172_e2172_d_b37, eq172_e2172_d_b38, eq172_e2172_d_b39, eq172_e2172_d_b40, eq172_e2172_d_b41, eq172_e2172_d_b42, eq172_e2172_d_b43, eq172_e2172_d_b44, eq172_e2172_d_b45, eq172_e2172_d_b46, eq172_e2172_d_b47, eq172_e2172_d_b48, eq172_e2172_d_b49, eq172_e2172_d_b50, eq172_e2172_d_b51, eq172_e2172_d_b52, eq172_e2172_d_b53, eq172_e2172_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq172_value: f64 = eq172_e2174;
        let eq172_node_derivatives: [f64; 23] = [eq172_e2174_d_n0, eq172_e2174_d_n1, eq172_e2174_d_n2, eq172_e2174_d_n3, eq172_e2174_d_n4, eq172_e2174_d_n5, eq172_e2174_d_n6, eq172_e2174_d_n7, eq172_e2174_d_n8, eq172_e2174_d_n9, eq172_e2174_d_n10, eq172_e2174_d_n11, eq172_e2174_d_n12, eq172_e2174_d_n13, eq172_e2174_d_n14, eq172_e2174_d_n15, eq172_e2174_d_n16, eq172_e2174_d_n17, eq172_e2174_d_n18, eq172_e2174_d_n19, eq172_e2174_d_n20, eq172_e2174_d_n21, eq172_e2174_d_n22];
        let eq172_branch_derivatives: [f64; 55] = [eq172_e2174_d_b0, eq172_e2174_d_b1, eq172_e2174_d_b2, eq172_e2174_d_b3, eq172_e2174_d_b4, eq172_e2174_d_b5, eq172_e2174_d_b6, eq172_e2174_d_b7, eq172_e2174_d_b8, eq172_e2174_d_b9, eq172_e2174_d_b10, eq172_e2174_d_b11, eq172_e2174_d_b12, eq172_e2174_d_b13, eq172_e2174_d_b14, eq172_e2174_d_b15, eq172_e2174_d_b16, eq172_e2174_d_b17, eq172_e2174_d_b18, eq172_e2174_d_b19, eq172_e2174_d_b20, eq172_e2174_d_b21, eq172_e2174_d_b22, eq172_e2174_d_b23, eq172_e2174_d_b24, eq172_e2174_d_b25, eq172_e2174_d_b26, eq172_e2174_d_b27, eq172_e2174_d_b28, eq172_e2174_d_b29, eq172_e2174_d_b30, eq172_e2174_d_b31, eq172_e2174_d_b32, eq172_e2174_d_b33, eq172_e2174_d_b34, eq172_e2174_d_b35, eq172_e2174_d_b36, eq172_e2174_d_b37, eq172_e2174_d_b38, eq172_e2174_d_b39, eq172_e2174_d_b40, eq172_e2174_d_b41, eq172_e2174_d_b42, eq172_e2174_d_b43, eq172_e2174_d_b44, eq172_e2174_d_b45, eq172_e2174_d_b46, eq172_e2174_d_b47, eq172_e2174_d_b48, eq172_e2174_d_b49, eq172_e2174_d_b50, eq172_e2174_d_b51, eq172_e2174_d_b52, eq172_e2174_d_b53, eq172_e2174_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(16),
            multiplicity * (eq172_value),
            &eq172_node_derivatives,
            &eq172_branch_derivatives,
            multiplicity,
        );
        let (eq173_e2185, eq173_e2185_d_n0, eq173_e2185_d_n1, eq173_e2185_d_n2, eq173_e2185_d_n3, eq173_e2185_d_n4, eq173_e2185_d_n5, eq173_e2185_d_n6, eq173_e2185_d_n7, eq173_e2185_d_n8, eq173_e2185_d_n9, eq173_e2185_d_n10, eq173_e2185_d_n11, eq173_e2185_d_n12, eq173_e2185_d_n13, eq173_e2185_d_n14, eq173_e2185_d_n15, eq173_e2185_d_n16, eq173_e2185_d_n17, eq173_e2185_d_n18, eq173_e2185_d_n19, eq173_e2185_d_n20, eq173_e2185_d_n21, eq173_e2185_d_n22, eq173_e2185_d_b0, eq173_e2185_d_b1, eq173_e2185_d_b2, eq173_e2185_d_b3, eq173_e2185_d_b4, eq173_e2185_d_b5, eq173_e2185_d_b6, eq173_e2185_d_b7, eq173_e2185_d_b8, eq173_e2185_d_b9, eq173_e2185_d_b10, eq173_e2185_d_b11, eq173_e2185_d_b12, eq173_e2185_d_b13, eq173_e2185_d_b14, eq173_e2185_d_b15, eq173_e2185_d_b16, eq173_e2185_d_b17, eq173_e2185_d_b18, eq173_e2185_d_b19, eq173_e2185_d_b20, eq173_e2185_d_b21, eq173_e2185_d_b22, eq173_e2185_d_b23, eq173_e2185_d_b24, eq173_e2185_d_b25, eq173_e2185_d_b26, eq173_e2185_d_b27, eq173_e2185_d_b28, eq173_e2185_d_b29, eq173_e2185_d_b30, eq173_e2185_d_b31, eq173_e2185_d_b32, eq173_e2185_d_b33, eq173_e2185_d_b34, eq173_e2185_d_b35, eq173_e2185_d_b36, eq173_e2185_d_b37, eq173_e2185_d_b38, eq173_e2185_d_b39, eq173_e2185_d_b40, eq173_e2185_d_b41, eq173_e2185_d_b42, eq173_e2185_d_b43, eq173_e2185_d_b44, eq173_e2185_d_b45, eq173_e2185_d_b46, eq173_e2185_d_b47, eq173_e2185_d_b48, eq173_e2185_d_b49, eq173_e2185_d_b50, eq173_e2185_d_b51, eq173_e2185_d_b52, eq173_e2185_d_b53, eq173_e2185_d_b54,) = {
    if (s.b[590] && s.b[591]) {
        let eq173_e2181: f64 = (p.p253 * s.v[276]);
        let eq173_e2182: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 72, eq173_e2181);
        let eq173_e2182_d_n0: f64 = ((p.p253 * s.dn[276][0]) * ddt_scale);
        let eq173_e2182_d_n1: f64 = ((p.p253 * s.dn[276][1]) * ddt_scale);
        let eq173_e2182_d_n2: f64 = ((p.p253 * s.dn[276][2]) * ddt_scale);
        let eq173_e2182_d_n3: f64 = ((p.p253 * s.dn[276][3]) * ddt_scale);
        let eq173_e2182_d_n4: f64 = ((p.p253 * s.dn[276][4]) * ddt_scale);
        let eq173_e2182_d_n5: f64 = ((p.p253 * s.dn[276][5]) * ddt_scale);
        let eq173_e2182_d_n6: f64 = ((p.p253 * s.dn[276][6]) * ddt_scale);
        let eq173_e2182_d_n7: f64 = ((p.p253 * s.dn[276][7]) * ddt_scale);
        let eq173_e2182_d_n8: f64 = ((p.p253 * s.dn[276][8]) * ddt_scale);
        let eq173_e2182_d_n9: f64 = ((p.p253 * s.dn[276][9]) * ddt_scale);
        let eq173_e2182_d_n10: f64 = ((p.p253 * s.dn[276][10]) * ddt_scale);
        let eq173_e2182_d_n11: f64 = ((p.p253 * s.dn[276][11]) * ddt_scale);
        let eq173_e2182_d_n12: f64 = ((p.p253 * s.dn[276][12]) * ddt_scale);
        let eq173_e2182_d_n13: f64 = ((p.p253 * s.dn[276][13]) * ddt_scale);
        let eq173_e2182_d_n14: f64 = ((p.p253 * s.dn[276][14]) * ddt_scale);
        let eq173_e2182_d_n15: f64 = ((p.p253 * s.dn[276][15]) * ddt_scale);
        let eq173_e2182_d_n16: f64 = ((p.p253 * s.dn[276][16]) * ddt_scale);
        let eq173_e2182_d_n17: f64 = ((p.p253 * s.dn[276][17]) * ddt_scale);
        let eq173_e2182_d_n18: f64 = ((p.p253 * s.dn[276][18]) * ddt_scale);
        let eq173_e2182_d_n19: f64 = ((p.p253 * s.dn[276][19]) * ddt_scale);
        let eq173_e2182_d_n20: f64 = ((p.p253 * s.dn[276][20]) * ddt_scale);
        let eq173_e2182_d_n21: f64 = ((p.p253 * s.dn[276][21]) * ddt_scale);
        let eq173_e2182_d_n22: f64 = ((p.p253 * s.dn[276][22]) * ddt_scale);
        let eq173_e2182_d_b0: f64 = ((p.p253 * s.db[276][0]) * ddt_scale);
        let eq173_e2182_d_b1: f64 = ((p.p253 * s.db[276][1]) * ddt_scale);
        let eq173_e2182_d_b2: f64 = ((p.p253 * s.db[276][2]) * ddt_scale);
        let eq173_e2182_d_b3: f64 = ((p.p253 * s.db[276][3]) * ddt_scale);
        let eq173_e2182_d_b4: f64 = ((p.p253 * s.db[276][4]) * ddt_scale);
        let eq173_e2182_d_b5: f64 = ((p.p253 * s.db[276][5]) * ddt_scale);
        let eq173_e2182_d_b6: f64 = ((p.p253 * s.db[276][6]) * ddt_scale);
        let eq173_e2182_d_b7: f64 = ((p.p253 * s.db[276][7]) * ddt_scale);
        let eq173_e2182_d_b8: f64 = ((p.p253 * s.db[276][8]) * ddt_scale);
        let eq173_e2182_d_b9: f64 = ((p.p253 * s.db[276][9]) * ddt_scale);
        let eq173_e2182_d_b10: f64 = ((p.p253 * s.db[276][10]) * ddt_scale);
        let eq173_e2182_d_b11: f64 = ((p.p253 * s.db[276][11]) * ddt_scale);
        let eq173_e2182_d_b12: f64 = ((p.p253 * s.db[276][12]) * ddt_scale);
        let eq173_e2182_d_b13: f64 = ((p.p253 * s.db[276][13]) * ddt_scale);
        let eq173_e2182_d_b14: f64 = ((p.p253 * s.db[276][14]) * ddt_scale);
        let eq173_e2182_d_b15: f64 = ((p.p253 * s.db[276][15]) * ddt_scale);
        let eq173_e2182_d_b16: f64 = ((p.p253 * s.db[276][16]) * ddt_scale);
        let eq173_e2182_d_b17: f64 = ((p.p253 * s.db[276][17]) * ddt_scale);
        let eq173_e2182_d_b18: f64 = ((p.p253 * s.db[276][18]) * ddt_scale);
        let eq173_e2182_d_b19: f64 = ((p.p253 * s.db[276][19]) * ddt_scale);
        let eq173_e2182_d_b20: f64 = ((p.p253 * s.db[276][20]) * ddt_scale);
        let eq173_e2182_d_b21: f64 = ((p.p253 * s.db[276][21]) * ddt_scale);
        let eq173_e2182_d_b22: f64 = ((p.p253 * s.db[276][22]) * ddt_scale);
        let eq173_e2182_d_b23: f64 = ((p.p253 * s.db[276][23]) * ddt_scale);
        let eq173_e2182_d_b24: f64 = ((p.p253 * s.db[276][24]) * ddt_scale);
        let eq173_e2182_d_b25: f64 = ((p.p253 * s.db[276][25]) * ddt_scale);
        let eq173_e2182_d_b26: f64 = ((p.p253 * s.db[276][26]) * ddt_scale);
        let eq173_e2182_d_b27: f64 = ((p.p253 * s.db[276][27]) * ddt_scale);
        let eq173_e2182_d_b28: f64 = ((p.p253 * s.db[276][28]) * ddt_scale);
        let eq173_e2182_d_b29: f64 = ((p.p253 * s.db[276][29]) * ddt_scale);
        let eq173_e2182_d_b30: f64 = ((p.p253 * s.db[276][30]) * ddt_scale);
        let eq173_e2182_d_b31: f64 = ((p.p253 * s.db[276][31]) * ddt_scale);
        let eq173_e2182_d_b32: f64 = ((p.p253 * s.db[276][32]) * ddt_scale);
        let eq173_e2182_d_b33: f64 = ((p.p253 * s.db[276][33]) * ddt_scale);
        let eq173_e2182_d_b34: f64 = ((p.p253 * s.db[276][34]) * ddt_scale);
        let eq173_e2182_d_b35: f64 = ((p.p253 * s.db[276][35]) * ddt_scale);
        let eq173_e2182_d_b36: f64 = ((p.p253 * s.db[276][36]) * ddt_scale);
        let eq173_e2182_d_b37: f64 = ((p.p253 * s.db[276][37]) * ddt_scale);
        let eq173_e2182_d_b38: f64 = ((p.p253 * s.db[276][38]) * ddt_scale);
        let eq173_e2182_d_b39: f64 = ((p.p253 * s.db[276][39]) * ddt_scale);
        let eq173_e2182_d_b40: f64 = ((p.p253 * s.db[276][40]) * ddt_scale);
        let eq173_e2182_d_b41: f64 = ((p.p253 * s.db[276][41]) * ddt_scale);
        let eq173_e2182_d_b42: f64 = ((p.p253 * s.db[276][42]) * ddt_scale);
        let eq173_e2182_d_b43: f64 = ((p.p253 * s.db[276][43]) * ddt_scale);
        let eq173_e2182_d_b44: f64 = ((p.p253 * s.db[276][44]) * ddt_scale);
        let eq173_e2182_d_b45: f64 = ((p.p253 * s.db[276][45]) * ddt_scale);
        let eq173_e2182_d_b46: f64 = ((p.p253 * s.db[276][46]) * ddt_scale);
        let eq173_e2182_d_b47: f64 = ((p.p253 * s.db[276][47]) * ddt_scale);
        let eq173_e2182_d_b48: f64 = ((p.p253 * s.db[276][48]) * ddt_scale);
        let eq173_e2182_d_b49: f64 = ((p.p253 * s.db[276][49]) * ddt_scale);
        let eq173_e2182_d_b50: f64 = ((p.p253 * s.db[276][50]) * ddt_scale);
        let eq173_e2182_d_b51: f64 = ((p.p253 * s.db[276][51]) * ddt_scale);
        let eq173_e2182_d_b52: f64 = ((p.p253 * s.db[276][52]) * ddt_scale);
        let eq173_e2182_d_b53: f64 = ((p.p253 * s.db[276][53]) * ddt_scale);
        let eq173_e2182_d_b54: f64 = ((p.p253 * s.db[276][54]) * ddt_scale);
        let eq173_e2183: f64 = (p.p7 * eq173_e2182);
        let eq173_e2183_d_n0: f64 = (p.p7 * eq173_e2182_d_n0);
        let eq173_e2183_d_n1: f64 = (p.p7 * eq173_e2182_d_n1);
        let eq173_e2183_d_n2: f64 = (p.p7 * eq173_e2182_d_n2);
        let eq173_e2183_d_n3: f64 = (p.p7 * eq173_e2182_d_n3);
        let eq173_e2183_d_n4: f64 = (p.p7 * eq173_e2182_d_n4);
        let eq173_e2183_d_n5: f64 = (p.p7 * eq173_e2182_d_n5);
        let eq173_e2183_d_n6: f64 = (p.p7 * eq173_e2182_d_n6);
        let eq173_e2183_d_n7: f64 = (p.p7 * eq173_e2182_d_n7);
        let eq173_e2183_d_n8: f64 = (p.p7 * eq173_e2182_d_n8);
        let eq173_e2183_d_n9: f64 = (p.p7 * eq173_e2182_d_n9);
        let eq173_e2183_d_n10: f64 = (p.p7 * eq173_e2182_d_n10);
        let eq173_e2183_d_n11: f64 = (p.p7 * eq173_e2182_d_n11);
        let eq173_e2183_d_n12: f64 = (p.p7 * eq173_e2182_d_n12);
        let eq173_e2183_d_n13: f64 = (p.p7 * eq173_e2182_d_n13);
        let eq173_e2183_d_n14: f64 = (p.p7 * eq173_e2182_d_n14);
        let eq173_e2183_d_n15: f64 = (p.p7 * eq173_e2182_d_n15);
        let eq173_e2183_d_n16: f64 = (p.p7 * eq173_e2182_d_n16);
        let eq173_e2183_d_n17: f64 = (p.p7 * eq173_e2182_d_n17);
        let eq173_e2183_d_n18: f64 = (p.p7 * eq173_e2182_d_n18);
        let eq173_e2183_d_n19: f64 = (p.p7 * eq173_e2182_d_n19);
        let eq173_e2183_d_n20: f64 = (p.p7 * eq173_e2182_d_n20);
        let eq173_e2183_d_n21: f64 = (p.p7 * eq173_e2182_d_n21);
        let eq173_e2183_d_n22: f64 = (p.p7 * eq173_e2182_d_n22);
        let eq173_e2183_d_b0: f64 = (p.p7 * eq173_e2182_d_b0);
        let eq173_e2183_d_b1: f64 = (p.p7 * eq173_e2182_d_b1);
        let eq173_e2183_d_b2: f64 = (p.p7 * eq173_e2182_d_b2);
        let eq173_e2183_d_b3: f64 = (p.p7 * eq173_e2182_d_b3);
        let eq173_e2183_d_b4: f64 = (p.p7 * eq173_e2182_d_b4);
        let eq173_e2183_d_b5: f64 = (p.p7 * eq173_e2182_d_b5);
        let eq173_e2183_d_b6: f64 = (p.p7 * eq173_e2182_d_b6);
        let eq173_e2183_d_b7: f64 = (p.p7 * eq173_e2182_d_b7);
        let eq173_e2183_d_b8: f64 = (p.p7 * eq173_e2182_d_b8);
        let eq173_e2183_d_b9: f64 = (p.p7 * eq173_e2182_d_b9);
        let eq173_e2183_d_b10: f64 = (p.p7 * eq173_e2182_d_b10);
        let eq173_e2183_d_b11: f64 = (p.p7 * eq173_e2182_d_b11);
        let eq173_e2183_d_b12: f64 = (p.p7 * eq173_e2182_d_b12);
        let eq173_e2183_d_b13: f64 = (p.p7 * eq173_e2182_d_b13);
        let eq173_e2183_d_b14: f64 = (p.p7 * eq173_e2182_d_b14);
        let eq173_e2183_d_b15: f64 = (p.p7 * eq173_e2182_d_b15);
        let eq173_e2183_d_b16: f64 = (p.p7 * eq173_e2182_d_b16);
        let eq173_e2183_d_b17: f64 = (p.p7 * eq173_e2182_d_b17);
        let eq173_e2183_d_b18: f64 = (p.p7 * eq173_e2182_d_b18);
        let eq173_e2183_d_b19: f64 = (p.p7 * eq173_e2182_d_b19);
        let eq173_e2183_d_b20: f64 = (p.p7 * eq173_e2182_d_b20);
        let eq173_e2183_d_b21: f64 = (p.p7 * eq173_e2182_d_b21);
        let eq173_e2183_d_b22: f64 = (p.p7 * eq173_e2182_d_b22);
        let eq173_e2183_d_b23: f64 = (p.p7 * eq173_e2182_d_b23);
        let eq173_e2183_d_b24: f64 = (p.p7 * eq173_e2182_d_b24);
        let eq173_e2183_d_b25: f64 = (p.p7 * eq173_e2182_d_b25);
        let eq173_e2183_d_b26: f64 = (p.p7 * eq173_e2182_d_b26);
        let eq173_e2183_d_b27: f64 = (p.p7 * eq173_e2182_d_b27);
        let eq173_e2183_d_b28: f64 = (p.p7 * eq173_e2182_d_b28);
        let eq173_e2183_d_b29: f64 = (p.p7 * eq173_e2182_d_b29);
        let eq173_e2183_d_b30: f64 = (p.p7 * eq173_e2182_d_b30);
        let eq173_e2183_d_b31: f64 = (p.p7 * eq173_e2182_d_b31);
        let eq173_e2183_d_b32: f64 = (p.p7 * eq173_e2182_d_b32);
        let eq173_e2183_d_b33: f64 = (p.p7 * eq173_e2182_d_b33);
        let eq173_e2183_d_b34: f64 = (p.p7 * eq173_e2182_d_b34);
        let eq173_e2183_d_b35: f64 = (p.p7 * eq173_e2182_d_b35);
        let eq173_e2183_d_b36: f64 = (p.p7 * eq173_e2182_d_b36);
        let eq173_e2183_d_b37: f64 = (p.p7 * eq173_e2182_d_b37);
        let eq173_e2183_d_b38: f64 = (p.p7 * eq173_e2182_d_b38);
        let eq173_e2183_d_b39: f64 = (p.p7 * eq173_e2182_d_b39);
        let eq173_e2183_d_b40: f64 = (p.p7 * eq173_e2182_d_b40);
        let eq173_e2183_d_b41: f64 = (p.p7 * eq173_e2182_d_b41);
        let eq173_e2183_d_b42: f64 = (p.p7 * eq173_e2182_d_b42);
        let eq173_e2183_d_b43: f64 = (p.p7 * eq173_e2182_d_b43);
        let eq173_e2183_d_b44: f64 = (p.p7 * eq173_e2182_d_b44);
        let eq173_e2183_d_b45: f64 = (p.p7 * eq173_e2182_d_b45);
        let eq173_e2183_d_b46: f64 = (p.p7 * eq173_e2182_d_b46);
        let eq173_e2183_d_b47: f64 = (p.p7 * eq173_e2182_d_b47);
        let eq173_e2183_d_b48: f64 = (p.p7 * eq173_e2182_d_b48);
        let eq173_e2183_d_b49: f64 = (p.p7 * eq173_e2182_d_b49);
        let eq173_e2183_d_b50: f64 = (p.p7 * eq173_e2182_d_b50);
        let eq173_e2183_d_b51: f64 = (p.p7 * eq173_e2182_d_b51);
        let eq173_e2183_d_b52: f64 = (p.p7 * eq173_e2182_d_b52);
        let eq173_e2183_d_b53: f64 = (p.p7 * eq173_e2182_d_b53);
        let eq173_e2183_d_b54: f64 = (p.p7 * eq173_e2182_d_b54);
        (eq173_e2183, eq173_e2183_d_n0, eq173_e2183_d_n1, eq173_e2183_d_n2, eq173_e2183_d_n3, eq173_e2183_d_n4, eq173_e2183_d_n5, eq173_e2183_d_n6, eq173_e2183_d_n7, eq173_e2183_d_n8, eq173_e2183_d_n9, eq173_e2183_d_n10, eq173_e2183_d_n11, eq173_e2183_d_n12, eq173_e2183_d_n13, eq173_e2183_d_n14, eq173_e2183_d_n15, eq173_e2183_d_n16, eq173_e2183_d_n17, eq173_e2183_d_n18, eq173_e2183_d_n19, eq173_e2183_d_n20, eq173_e2183_d_n21, eq173_e2183_d_n22, eq173_e2183_d_b0, eq173_e2183_d_b1, eq173_e2183_d_b2, eq173_e2183_d_b3, eq173_e2183_d_b4, eq173_e2183_d_b5, eq173_e2183_d_b6, eq173_e2183_d_b7, eq173_e2183_d_b8, eq173_e2183_d_b9, eq173_e2183_d_b10, eq173_e2183_d_b11, eq173_e2183_d_b12, eq173_e2183_d_b13, eq173_e2183_d_b14, eq173_e2183_d_b15, eq173_e2183_d_b16, eq173_e2183_d_b17, eq173_e2183_d_b18, eq173_e2183_d_b19, eq173_e2183_d_b20, eq173_e2183_d_b21, eq173_e2183_d_b22, eq173_e2183_d_b23, eq173_e2183_d_b24, eq173_e2183_d_b25, eq173_e2183_d_b26, eq173_e2183_d_b27, eq173_e2183_d_b28, eq173_e2183_d_b29, eq173_e2183_d_b30, eq173_e2183_d_b31, eq173_e2183_d_b32, eq173_e2183_d_b33, eq173_e2183_d_b34, eq173_e2183_d_b35, eq173_e2183_d_b36, eq173_e2183_d_b37, eq173_e2183_d_b38, eq173_e2183_d_b39, eq173_e2183_d_b40, eq173_e2183_d_b41, eq173_e2183_d_b42, eq173_e2183_d_b43, eq173_e2183_d_b44, eq173_e2183_d_b45, eq173_e2183_d_b46, eq173_e2183_d_b47, eq173_e2183_d_b48, eq173_e2183_d_b49, eq173_e2183_d_b50, eq173_e2183_d_b51, eq173_e2183_d_b52, eq173_e2183_d_b53, eq173_e2183_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq173_value: f64 = eq173_e2185;
        let eq173_node_derivatives: [f64; 23] = [eq173_e2185_d_n0, eq173_e2185_d_n1, eq173_e2185_d_n2, eq173_e2185_d_n3, eq173_e2185_d_n4, eq173_e2185_d_n5, eq173_e2185_d_n6, eq173_e2185_d_n7, eq173_e2185_d_n8, eq173_e2185_d_n9, eq173_e2185_d_n10, eq173_e2185_d_n11, eq173_e2185_d_n12, eq173_e2185_d_n13, eq173_e2185_d_n14, eq173_e2185_d_n15, eq173_e2185_d_n16, eq173_e2185_d_n17, eq173_e2185_d_n18, eq173_e2185_d_n19, eq173_e2185_d_n20, eq173_e2185_d_n21, eq173_e2185_d_n22];
        let eq173_branch_derivatives: [f64; 55] = [eq173_e2185_d_b0, eq173_e2185_d_b1, eq173_e2185_d_b2, eq173_e2185_d_b3, eq173_e2185_d_b4, eq173_e2185_d_b5, eq173_e2185_d_b6, eq173_e2185_d_b7, eq173_e2185_d_b8, eq173_e2185_d_b9, eq173_e2185_d_b10, eq173_e2185_d_b11, eq173_e2185_d_b12, eq173_e2185_d_b13, eq173_e2185_d_b14, eq173_e2185_d_b15, eq173_e2185_d_b16, eq173_e2185_d_b17, eq173_e2185_d_b18, eq173_e2185_d_b19, eq173_e2185_d_b20, eq173_e2185_d_b21, eq173_e2185_d_b22, eq173_e2185_d_b23, eq173_e2185_d_b24, eq173_e2185_d_b25, eq173_e2185_d_b26, eq173_e2185_d_b27, eq173_e2185_d_b28, eq173_e2185_d_b29, eq173_e2185_d_b30, eq173_e2185_d_b31, eq173_e2185_d_b32, eq173_e2185_d_b33, eq173_e2185_d_b34, eq173_e2185_d_b35, eq173_e2185_d_b36, eq173_e2185_d_b37, eq173_e2185_d_b38, eq173_e2185_d_b39, eq173_e2185_d_b40, eq173_e2185_d_b41, eq173_e2185_d_b42, eq173_e2185_d_b43, eq173_e2185_d_b44, eq173_e2185_d_b45, eq173_e2185_d_b46, eq173_e2185_d_b47, eq173_e2185_d_b48, eq173_e2185_d_b49, eq173_e2185_d_b50, eq173_e2185_d_b51, eq173_e2185_d_b52, eq173_e2185_d_b53, eq173_e2185_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(16),
            multiplicity * (eq173_value),
            &eq173_node_derivatives,
            &eq173_branch_derivatives,
            multiplicity,
        );
        let (eq174_e2195, eq174_e2195_d_n0, eq174_e2195_d_n1, eq174_e2195_d_n2, eq174_e2195_d_n3, eq174_e2195_d_n4, eq174_e2195_d_n5, eq174_e2195_d_n6, eq174_e2195_d_n7, eq174_e2195_d_n8, eq174_e2195_d_n9, eq174_e2195_d_n10, eq174_e2195_d_n11, eq174_e2195_d_n12, eq174_e2195_d_n13, eq174_e2195_d_n14, eq174_e2195_d_n15, eq174_e2195_d_n16, eq174_e2195_d_n17, eq174_e2195_d_n18, eq174_e2195_d_n19, eq174_e2195_d_n20, eq174_e2195_d_n21, eq174_e2195_d_n22, eq174_e2195_d_b0, eq174_e2195_d_b1, eq174_e2195_d_b2, eq174_e2195_d_b3, eq174_e2195_d_b4, eq174_e2195_d_b5, eq174_e2195_d_b6, eq174_e2195_d_b7, eq174_e2195_d_b8, eq174_e2195_d_b9, eq174_e2195_d_b10, eq174_e2195_d_b11, eq174_e2195_d_b12, eq174_e2195_d_b13, eq174_e2195_d_b14, eq174_e2195_d_b15, eq174_e2195_d_b16, eq174_e2195_d_b17, eq174_e2195_d_b18, eq174_e2195_d_b19, eq174_e2195_d_b20, eq174_e2195_d_b21, eq174_e2195_d_b22, eq174_e2195_d_b23, eq174_e2195_d_b24, eq174_e2195_d_b25, eq174_e2195_d_b26, eq174_e2195_d_b27, eq174_e2195_d_b28, eq174_e2195_d_b29, eq174_e2195_d_b30, eq174_e2195_d_b31, eq174_e2195_d_b32, eq174_e2195_d_b33, eq174_e2195_d_b34, eq174_e2195_d_b35, eq174_e2195_d_b36, eq174_e2195_d_b37, eq174_e2195_d_b38, eq174_e2195_d_b39, eq174_e2195_d_b40, eq174_e2195_d_b41, eq174_e2195_d_b42, eq174_e2195_d_b43, eq174_e2195_d_b44, eq174_e2195_d_b45, eq174_e2195_d_b46, eq174_e2195_d_b47, eq174_e2195_d_b48, eq174_e2195_d_b49, eq174_e2195_d_b50, eq174_e2195_d_b51, eq174_e2195_d_b52, eq174_e2195_d_b53, eq174_e2195_d_b54,) = {
    if ((!s.b[590]) && s.b[593]) {
        let eq174_e2192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 73, s.v[277]);
        let eq174_e2193: f64 = (p.p7 * eq174_e2192);
        let eq174_e2193_d_n0: f64 = (p.p7 * (s.dn[277][0] * ddt_scale));
        let eq174_e2193_d_n1: f64 = (p.p7 * (s.dn[277][1] * ddt_scale));
        let eq174_e2193_d_n2: f64 = (p.p7 * (s.dn[277][2] * ddt_scale));
        let eq174_e2193_d_n3: f64 = (p.p7 * (s.dn[277][3] * ddt_scale));
        let eq174_e2193_d_n4: f64 = (p.p7 * (s.dn[277][4] * ddt_scale));
        let eq174_e2193_d_n5: f64 = (p.p7 * (s.dn[277][5] * ddt_scale));
        let eq174_e2193_d_n6: f64 = (p.p7 * (s.dn[277][6] * ddt_scale));
        let eq174_e2193_d_n7: f64 = (p.p7 * (s.dn[277][7] * ddt_scale));
        let eq174_e2193_d_n8: f64 = (p.p7 * (s.dn[277][8] * ddt_scale));
        let eq174_e2193_d_n9: f64 = (p.p7 * (s.dn[277][9] * ddt_scale));
        let eq174_e2193_d_n10: f64 = (p.p7 * (s.dn[277][10] * ddt_scale));
        let eq174_e2193_d_n11: f64 = (p.p7 * (s.dn[277][11] * ddt_scale));
        let eq174_e2193_d_n12: f64 = (p.p7 * (s.dn[277][12] * ddt_scale));
        let eq174_e2193_d_n13: f64 = (p.p7 * (s.dn[277][13] * ddt_scale));
        let eq174_e2193_d_n14: f64 = (p.p7 * (s.dn[277][14] * ddt_scale));
        let eq174_e2193_d_n15: f64 = (p.p7 * (s.dn[277][15] * ddt_scale));
        let eq174_e2193_d_n16: f64 = (p.p7 * (s.dn[277][16] * ddt_scale));
        let eq174_e2193_d_n17: f64 = (p.p7 * (s.dn[277][17] * ddt_scale));
        let eq174_e2193_d_n18: f64 = (p.p7 * (s.dn[277][18] * ddt_scale));
        let eq174_e2193_d_n19: f64 = (p.p7 * (s.dn[277][19] * ddt_scale));
        let eq174_e2193_d_n20: f64 = (p.p7 * (s.dn[277][20] * ddt_scale));
        let eq174_e2193_d_n21: f64 = (p.p7 * (s.dn[277][21] * ddt_scale));
        let eq174_e2193_d_n22: f64 = (p.p7 * (s.dn[277][22] * ddt_scale));
        let eq174_e2193_d_b0: f64 = (p.p7 * (s.db[277][0] * ddt_scale));
        let eq174_e2193_d_b1: f64 = (p.p7 * (s.db[277][1] * ddt_scale));
        let eq174_e2193_d_b2: f64 = (p.p7 * (s.db[277][2] * ddt_scale));
        let eq174_e2193_d_b3: f64 = (p.p7 * (s.db[277][3] * ddt_scale));
        let eq174_e2193_d_b4: f64 = (p.p7 * (s.db[277][4] * ddt_scale));
        let eq174_e2193_d_b5: f64 = (p.p7 * (s.db[277][5] * ddt_scale));
        let eq174_e2193_d_b6: f64 = (p.p7 * (s.db[277][6] * ddt_scale));
        let eq174_e2193_d_b7: f64 = (p.p7 * (s.db[277][7] * ddt_scale));
        let eq174_e2193_d_b8: f64 = (p.p7 * (s.db[277][8] * ddt_scale));
        let eq174_e2193_d_b9: f64 = (p.p7 * (s.db[277][9] * ddt_scale));
        let eq174_e2193_d_b10: f64 = (p.p7 * (s.db[277][10] * ddt_scale));
        let eq174_e2193_d_b11: f64 = (p.p7 * (s.db[277][11] * ddt_scale));
        let eq174_e2193_d_b12: f64 = (p.p7 * (s.db[277][12] * ddt_scale));
        let eq174_e2193_d_b13: f64 = (p.p7 * (s.db[277][13] * ddt_scale));
        let eq174_e2193_d_b14: f64 = (p.p7 * (s.db[277][14] * ddt_scale));
        let eq174_e2193_d_b15: f64 = (p.p7 * (s.db[277][15] * ddt_scale));
        let eq174_e2193_d_b16: f64 = (p.p7 * (s.db[277][16] * ddt_scale));
        let eq174_e2193_d_b17: f64 = (p.p7 * (s.db[277][17] * ddt_scale));
        let eq174_e2193_d_b18: f64 = (p.p7 * (s.db[277][18] * ddt_scale));
        let eq174_e2193_d_b19: f64 = (p.p7 * (s.db[277][19] * ddt_scale));
        let eq174_e2193_d_b20: f64 = (p.p7 * (s.db[277][20] * ddt_scale));
        let eq174_e2193_d_b21: f64 = (p.p7 * (s.db[277][21] * ddt_scale));
        let eq174_e2193_d_b22: f64 = (p.p7 * (s.db[277][22] * ddt_scale));
        let eq174_e2193_d_b23: f64 = (p.p7 * (s.db[277][23] * ddt_scale));
        let eq174_e2193_d_b24: f64 = (p.p7 * (s.db[277][24] * ddt_scale));
        let eq174_e2193_d_b25: f64 = (p.p7 * (s.db[277][25] * ddt_scale));
        let eq174_e2193_d_b26: f64 = (p.p7 * (s.db[277][26] * ddt_scale));
        let eq174_e2193_d_b27: f64 = (p.p7 * (s.db[277][27] * ddt_scale));
        let eq174_e2193_d_b28: f64 = (p.p7 * (s.db[277][28] * ddt_scale));
        let eq174_e2193_d_b29: f64 = (p.p7 * (s.db[277][29] * ddt_scale));
        let eq174_e2193_d_b30: f64 = (p.p7 * (s.db[277][30] * ddt_scale));
        let eq174_e2193_d_b31: f64 = (p.p7 * (s.db[277][31] * ddt_scale));
        let eq174_e2193_d_b32: f64 = (p.p7 * (s.db[277][32] * ddt_scale));
        let eq174_e2193_d_b33: f64 = (p.p7 * (s.db[277][33] * ddt_scale));
        let eq174_e2193_d_b34: f64 = (p.p7 * (s.db[277][34] * ddt_scale));
        let eq174_e2193_d_b35: f64 = (p.p7 * (s.db[277][35] * ddt_scale));
        let eq174_e2193_d_b36: f64 = (p.p7 * (s.db[277][36] * ddt_scale));
        let eq174_e2193_d_b37: f64 = (p.p7 * (s.db[277][37] * ddt_scale));
        let eq174_e2193_d_b38: f64 = (p.p7 * (s.db[277][38] * ddt_scale));
        let eq174_e2193_d_b39: f64 = (p.p7 * (s.db[277][39] * ddt_scale));
        let eq174_e2193_d_b40: f64 = (p.p7 * (s.db[277][40] * ddt_scale));
        let eq174_e2193_d_b41: f64 = (p.p7 * (s.db[277][41] * ddt_scale));
        let eq174_e2193_d_b42: f64 = (p.p7 * (s.db[277][42] * ddt_scale));
        let eq174_e2193_d_b43: f64 = (p.p7 * (s.db[277][43] * ddt_scale));
        let eq174_e2193_d_b44: f64 = (p.p7 * (s.db[277][44] * ddt_scale));
        let eq174_e2193_d_b45: f64 = (p.p7 * (s.db[277][45] * ddt_scale));
        let eq174_e2193_d_b46: f64 = (p.p7 * (s.db[277][46] * ddt_scale));
        let eq174_e2193_d_b47: f64 = (p.p7 * (s.db[277][47] * ddt_scale));
        let eq174_e2193_d_b48: f64 = (p.p7 * (s.db[277][48] * ddt_scale));
        let eq174_e2193_d_b49: f64 = (p.p7 * (s.db[277][49] * ddt_scale));
        let eq174_e2193_d_b50: f64 = (p.p7 * (s.db[277][50] * ddt_scale));
        let eq174_e2193_d_b51: f64 = (p.p7 * (s.db[277][51] * ddt_scale));
        let eq174_e2193_d_b52: f64 = (p.p7 * (s.db[277][52] * ddt_scale));
        let eq174_e2193_d_b53: f64 = (p.p7 * (s.db[277][53] * ddt_scale));
        let eq174_e2193_d_b54: f64 = (p.p7 * (s.db[277][54] * ddt_scale));
        (eq174_e2193, eq174_e2193_d_n0, eq174_e2193_d_n1, eq174_e2193_d_n2, eq174_e2193_d_n3, eq174_e2193_d_n4, eq174_e2193_d_n5, eq174_e2193_d_n6, eq174_e2193_d_n7, eq174_e2193_d_n8, eq174_e2193_d_n9, eq174_e2193_d_n10, eq174_e2193_d_n11, eq174_e2193_d_n12, eq174_e2193_d_n13, eq174_e2193_d_n14, eq174_e2193_d_n15, eq174_e2193_d_n16, eq174_e2193_d_n17, eq174_e2193_d_n18, eq174_e2193_d_n19, eq174_e2193_d_n20, eq174_e2193_d_n21, eq174_e2193_d_n22, eq174_e2193_d_b0, eq174_e2193_d_b1, eq174_e2193_d_b2, eq174_e2193_d_b3, eq174_e2193_d_b4, eq174_e2193_d_b5, eq174_e2193_d_b6, eq174_e2193_d_b7, eq174_e2193_d_b8, eq174_e2193_d_b9, eq174_e2193_d_b10, eq174_e2193_d_b11, eq174_e2193_d_b12, eq174_e2193_d_b13, eq174_e2193_d_b14, eq174_e2193_d_b15, eq174_e2193_d_b16, eq174_e2193_d_b17, eq174_e2193_d_b18, eq174_e2193_d_b19, eq174_e2193_d_b20, eq174_e2193_d_b21, eq174_e2193_d_b22, eq174_e2193_d_b23, eq174_e2193_d_b24, eq174_e2193_d_b25, eq174_e2193_d_b26, eq174_e2193_d_b27, eq174_e2193_d_b28, eq174_e2193_d_b29, eq174_e2193_d_b30, eq174_e2193_d_b31, eq174_e2193_d_b32, eq174_e2193_d_b33, eq174_e2193_d_b34, eq174_e2193_d_b35, eq174_e2193_d_b36, eq174_e2193_d_b37, eq174_e2193_d_b38, eq174_e2193_d_b39, eq174_e2193_d_b40, eq174_e2193_d_b41, eq174_e2193_d_b42, eq174_e2193_d_b43, eq174_e2193_d_b44, eq174_e2193_d_b45, eq174_e2193_d_b46, eq174_e2193_d_b47, eq174_e2193_d_b48, eq174_e2193_d_b49, eq174_e2193_d_b50, eq174_e2193_d_b51, eq174_e2193_d_b52, eq174_e2193_d_b53, eq174_e2193_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq174_value: f64 = eq174_e2195;
        let eq174_node_derivatives: [f64; 23] = [eq174_e2195_d_n0, eq174_e2195_d_n1, eq174_e2195_d_n2, eq174_e2195_d_n3, eq174_e2195_d_n4, eq174_e2195_d_n5, eq174_e2195_d_n6, eq174_e2195_d_n7, eq174_e2195_d_n8, eq174_e2195_d_n9, eq174_e2195_d_n10, eq174_e2195_d_n11, eq174_e2195_d_n12, eq174_e2195_d_n13, eq174_e2195_d_n14, eq174_e2195_d_n15, eq174_e2195_d_n16, eq174_e2195_d_n17, eq174_e2195_d_n18, eq174_e2195_d_n19, eq174_e2195_d_n20, eq174_e2195_d_n21, eq174_e2195_d_n22];
        let eq174_branch_derivatives: [f64; 55] = [eq174_e2195_d_b0, eq174_e2195_d_b1, eq174_e2195_d_b2, eq174_e2195_d_b3, eq174_e2195_d_b4, eq174_e2195_d_b5, eq174_e2195_d_b6, eq174_e2195_d_b7, eq174_e2195_d_b8, eq174_e2195_d_b9, eq174_e2195_d_b10, eq174_e2195_d_b11, eq174_e2195_d_b12, eq174_e2195_d_b13, eq174_e2195_d_b14, eq174_e2195_d_b15, eq174_e2195_d_b16, eq174_e2195_d_b17, eq174_e2195_d_b18, eq174_e2195_d_b19, eq174_e2195_d_b20, eq174_e2195_d_b21, eq174_e2195_d_b22, eq174_e2195_d_b23, eq174_e2195_d_b24, eq174_e2195_d_b25, eq174_e2195_d_b26, eq174_e2195_d_b27, eq174_e2195_d_b28, eq174_e2195_d_b29, eq174_e2195_d_b30, eq174_e2195_d_b31, eq174_e2195_d_b32, eq174_e2195_d_b33, eq174_e2195_d_b34, eq174_e2195_d_b35, eq174_e2195_d_b36, eq174_e2195_d_b37, eq174_e2195_d_b38, eq174_e2195_d_b39, eq174_e2195_d_b40, eq174_e2195_d_b41, eq174_e2195_d_b42, eq174_e2195_d_b43, eq174_e2195_d_b44, eq174_e2195_d_b45, eq174_e2195_d_b46, eq174_e2195_d_b47, eq174_e2195_d_b48, eq174_e2195_d_b49, eq174_e2195_d_b50, eq174_e2195_d_b51, eq174_e2195_d_b52, eq174_e2195_d_b53, eq174_e2195_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq174_value),
            &eq174_node_derivatives,
            &eq174_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_33(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[276][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[276][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[276][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[276][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[276][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[276][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[276][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[276][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[276][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[276][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[276][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[276][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[276][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[276][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[276][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[276][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[276][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[276][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[276][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[276][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[276][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[276][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[276][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[276][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[276][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[276][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[276][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[276][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[276][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[276][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[276][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[276][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[276][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[276][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[276][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[276][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[276][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[276][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[276][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[276][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[276][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[276][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[276][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[276][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[276][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[276][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[276][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[276][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[276][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[276][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[276][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[276][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[276][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[276][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[276][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[276][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[276][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[276][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[276][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[276][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[276][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[276][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[276][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[276][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[276][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[276][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[276][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[276][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[276][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[276][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[276][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[276][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[276][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[276][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[276][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[276][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[276][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[276][54] * ddt_scale));
        let (eq175_e2207, eq175_e2207_d_n0, eq175_e2207_d_n1, eq175_e2207_d_n2, eq175_e2207_d_n3, eq175_e2207_d_n4, eq175_e2207_d_n5, eq175_e2207_d_n6, eq175_e2207_d_n7, eq175_e2207_d_n8, eq175_e2207_d_n9, eq175_e2207_d_n10, eq175_e2207_d_n11, eq175_e2207_d_n12, eq175_e2207_d_n13, eq175_e2207_d_n14, eq175_e2207_d_n15, eq175_e2207_d_n16, eq175_e2207_d_n17, eq175_e2207_d_n18, eq175_e2207_d_n19, eq175_e2207_d_n20, eq175_e2207_d_n21, eq175_e2207_d_n22, eq175_e2207_d_b0, eq175_e2207_d_b1, eq175_e2207_d_b2, eq175_e2207_d_b3, eq175_e2207_d_b4, eq175_e2207_d_b5, eq175_e2207_d_b6, eq175_e2207_d_b7, eq175_e2207_d_b8, eq175_e2207_d_b9, eq175_e2207_d_b10, eq175_e2207_d_b11, eq175_e2207_d_b12, eq175_e2207_d_b13, eq175_e2207_d_b14, eq175_e2207_d_b15, eq175_e2207_d_b16, eq175_e2207_d_b17, eq175_e2207_d_b18, eq175_e2207_d_b19, eq175_e2207_d_b20, eq175_e2207_d_b21, eq175_e2207_d_b22, eq175_e2207_d_b23, eq175_e2207_d_b24, eq175_e2207_d_b25, eq175_e2207_d_b26, eq175_e2207_d_b27, eq175_e2207_d_b28, eq175_e2207_d_b29, eq175_e2207_d_b30, eq175_e2207_d_b31, eq175_e2207_d_b32, eq175_e2207_d_b33, eq175_e2207_d_b34, eq175_e2207_d_b35, eq175_e2207_d_b36, eq175_e2207_d_b37, eq175_e2207_d_b38, eq175_e2207_d_b39, eq175_e2207_d_b40, eq175_e2207_d_b41, eq175_e2207_d_b42, eq175_e2207_d_b43, eq175_e2207_d_b44, eq175_e2207_d_b45, eq175_e2207_d_b46, eq175_e2207_d_b47, eq175_e2207_d_b48, eq175_e2207_d_b49, eq175_e2207_d_b50, eq175_e2207_d_b51, eq175_e2207_d_b52, eq175_e2207_d_b53, eq175_e2207_d_b54,) = {
    if (((!s.b[590]) && s.b[593]) && s.b[594]) {
        let eq175_e2204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 74, s.v[276]);
        let eq175_e2205: f64 = (p.p7 * eq175_e2204);
        (eq175_e2205, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq175_value: f64 = eq175_e2207;
        let eq175_node_derivatives: [f64; 23] = [eq175_e2207_d_n0, eq175_e2207_d_n1, eq175_e2207_d_n2, eq175_e2207_d_n3, eq175_e2207_d_n4, eq175_e2207_d_n5, eq175_e2207_d_n6, eq175_e2207_d_n7, eq175_e2207_d_n8, eq175_e2207_d_n9, eq175_e2207_d_n10, eq175_e2207_d_n11, eq175_e2207_d_n12, eq175_e2207_d_n13, eq175_e2207_d_n14, eq175_e2207_d_n15, eq175_e2207_d_n16, eq175_e2207_d_n17, eq175_e2207_d_n18, eq175_e2207_d_n19, eq175_e2207_d_n20, eq175_e2207_d_n21, eq175_e2207_d_n22];
        let eq175_branch_derivatives: [f64; 55] = [eq175_e2207_d_b0, eq175_e2207_d_b1, eq175_e2207_d_b2, eq175_e2207_d_b3, eq175_e2207_d_b4, eq175_e2207_d_b5, eq175_e2207_d_b6, eq175_e2207_d_b7, eq175_e2207_d_b8, eq175_e2207_d_b9, eq175_e2207_d_b10, eq175_e2207_d_b11, eq175_e2207_d_b12, eq175_e2207_d_b13, eq175_e2207_d_b14, eq175_e2207_d_b15, eq175_e2207_d_b16, eq175_e2207_d_b17, eq175_e2207_d_b18, eq175_e2207_d_b19, eq175_e2207_d_b20, eq175_e2207_d_b21, eq175_e2207_d_b22, eq175_e2207_d_b23, eq175_e2207_d_b24, eq175_e2207_d_b25, eq175_e2207_d_b26, eq175_e2207_d_b27, eq175_e2207_d_b28, eq175_e2207_d_b29, eq175_e2207_d_b30, eq175_e2207_d_b31, eq175_e2207_d_b32, eq175_e2207_d_b33, eq175_e2207_d_b34, eq175_e2207_d_b35, eq175_e2207_d_b36, eq175_e2207_d_b37, eq175_e2207_d_b38, eq175_e2207_d_b39, eq175_e2207_d_b40, eq175_e2207_d_b41, eq175_e2207_d_b42, eq175_e2207_d_b43, eq175_e2207_d_b44, eq175_e2207_d_b45, eq175_e2207_d_b46, eq175_e2207_d_b47, eq175_e2207_d_b48, eq175_e2207_d_b49, eq175_e2207_d_b50, eq175_e2207_d_b51, eq175_e2207_d_b52, eq175_e2207_d_b53, eq175_e2207_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq175_value),
            &eq175_node_derivatives,
            &eq175_branch_derivatives,
            multiplicity,
        );
        let (eq176_e2221, eq176_e2221_d_n0, eq176_e2221_d_n1, eq176_e2221_d_n2, eq176_e2221_d_n3, eq176_e2221_d_n4, eq176_e2221_d_n5, eq176_e2221_d_n6, eq176_e2221_d_n7, eq176_e2221_d_n8, eq176_e2221_d_n9, eq176_e2221_d_n10, eq176_e2221_d_n11, eq176_e2221_d_n12, eq176_e2221_d_n13, eq176_e2221_d_n14, eq176_e2221_d_n15, eq176_e2221_d_n16, eq176_e2221_d_n17, eq176_e2221_d_n18, eq176_e2221_d_n19, eq176_e2221_d_n20, eq176_e2221_d_n21, eq176_e2221_d_n22, eq176_e2221_d_b0, eq176_e2221_d_b1, eq176_e2221_d_b2, eq176_e2221_d_b3, eq176_e2221_d_b4, eq176_e2221_d_b5, eq176_e2221_d_b6, eq176_e2221_d_b7, eq176_e2221_d_b8, eq176_e2221_d_b9, eq176_e2221_d_b10, eq176_e2221_d_b11, eq176_e2221_d_b12, eq176_e2221_d_b13, eq176_e2221_d_b14, eq176_e2221_d_b15, eq176_e2221_d_b16, eq176_e2221_d_b17, eq176_e2221_d_b18, eq176_e2221_d_b19, eq176_e2221_d_b20, eq176_e2221_d_b21, eq176_e2221_d_b22, eq176_e2221_d_b23, eq176_e2221_d_b24, eq176_e2221_d_b25, eq176_e2221_d_b26, eq176_e2221_d_b27, eq176_e2221_d_b28, eq176_e2221_d_b29, eq176_e2221_d_b30, eq176_e2221_d_b31, eq176_e2221_d_b32, eq176_e2221_d_b33, eq176_e2221_d_b34, eq176_e2221_d_b35, eq176_e2221_d_b36, eq176_e2221_d_b37, eq176_e2221_d_b38, eq176_e2221_d_b39, eq176_e2221_d_b40, eq176_e2221_d_b41, eq176_e2221_d_b42, eq176_e2221_d_b43, eq176_e2221_d_b44, eq176_e2221_d_b45, eq176_e2221_d_b46, eq176_e2221_d_b47, eq176_e2221_d_b48, eq176_e2221_d_b49, eq176_e2221_d_b50, eq176_e2221_d_b51, eq176_e2221_d_b52, eq176_e2221_d_b53, eq176_e2221_d_b54,) = {
    if (((!s.b[590]) && s.b[593]) && s.b[594]) {
        let eq176_e2216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 75, s.v[276]);
        let eq176_e2217: f64 = (p.p7 * eq176_e2216);
        let eq176_e2219: f64 = (eq176_e2217 * p.p248);
        let eq176_e2219_d_n0: f64 = (__rspice_deriv_cse_0 * p.p248);
        let eq176_e2219_d_n1: f64 = (__rspice_deriv_cse_1 * p.p248);
        let eq176_e2219_d_n2: f64 = (__rspice_deriv_cse_2 * p.p248);
        let eq176_e2219_d_n3: f64 = (__rspice_deriv_cse_3 * p.p248);
        let eq176_e2219_d_n4: f64 = (__rspice_deriv_cse_4 * p.p248);
        let eq176_e2219_d_n5: f64 = (__rspice_deriv_cse_5 * p.p248);
        let eq176_e2219_d_n6: f64 = (__rspice_deriv_cse_6 * p.p248);
        let eq176_e2219_d_n7: f64 = (__rspice_deriv_cse_7 * p.p248);
        let eq176_e2219_d_n8: f64 = (__rspice_deriv_cse_8 * p.p248);
        let eq176_e2219_d_n9: f64 = (__rspice_deriv_cse_9 * p.p248);
        let eq176_e2219_d_n10: f64 = (__rspice_deriv_cse_10 * p.p248);
        let eq176_e2219_d_n11: f64 = (__rspice_deriv_cse_11 * p.p248);
        let eq176_e2219_d_n12: f64 = (__rspice_deriv_cse_12 * p.p248);
        let eq176_e2219_d_n13: f64 = (__rspice_deriv_cse_13 * p.p248);
        let eq176_e2219_d_n14: f64 = (__rspice_deriv_cse_14 * p.p248);
        let eq176_e2219_d_n15: f64 = (__rspice_deriv_cse_15 * p.p248);
        let eq176_e2219_d_n16: f64 = (__rspice_deriv_cse_16 * p.p248);
        let eq176_e2219_d_n17: f64 = (__rspice_deriv_cse_17 * p.p248);
        let eq176_e2219_d_n18: f64 = (__rspice_deriv_cse_18 * p.p248);
        let eq176_e2219_d_n19: f64 = (__rspice_deriv_cse_19 * p.p248);
        let eq176_e2219_d_n20: f64 = (__rspice_deriv_cse_20 * p.p248);
        let eq176_e2219_d_n21: f64 = (__rspice_deriv_cse_21 * p.p248);
        let eq176_e2219_d_n22: f64 = (__rspice_deriv_cse_22 * p.p248);
        let eq176_e2219_d_b0: f64 = (__rspice_deriv_cse_23 * p.p248);
        let eq176_e2219_d_b1: f64 = (__rspice_deriv_cse_24 * p.p248);
        let eq176_e2219_d_b2: f64 = (__rspice_deriv_cse_25 * p.p248);
        let eq176_e2219_d_b3: f64 = (__rspice_deriv_cse_26 * p.p248);
        let eq176_e2219_d_b4: f64 = (__rspice_deriv_cse_27 * p.p248);
        let eq176_e2219_d_b5: f64 = (__rspice_deriv_cse_28 * p.p248);
        let eq176_e2219_d_b6: f64 = (__rspice_deriv_cse_29 * p.p248);
        let eq176_e2219_d_b7: f64 = (__rspice_deriv_cse_30 * p.p248);
        let eq176_e2219_d_b8: f64 = (__rspice_deriv_cse_31 * p.p248);
        let eq176_e2219_d_b9: f64 = (__rspice_deriv_cse_32 * p.p248);
        let eq176_e2219_d_b10: f64 = (__rspice_deriv_cse_33 * p.p248);
        let eq176_e2219_d_b11: f64 = (__rspice_deriv_cse_34 * p.p248);
        let eq176_e2219_d_b12: f64 = (__rspice_deriv_cse_35 * p.p248);
        let eq176_e2219_d_b13: f64 = (__rspice_deriv_cse_36 * p.p248);
        let eq176_e2219_d_b14: f64 = (__rspice_deriv_cse_37 * p.p248);
        let eq176_e2219_d_b15: f64 = (__rspice_deriv_cse_38 * p.p248);
        let eq176_e2219_d_b16: f64 = (__rspice_deriv_cse_39 * p.p248);
        let eq176_e2219_d_b17: f64 = (__rspice_deriv_cse_40 * p.p248);
        let eq176_e2219_d_b18: f64 = (__rspice_deriv_cse_41 * p.p248);
        let eq176_e2219_d_b19: f64 = (__rspice_deriv_cse_42 * p.p248);
        let eq176_e2219_d_b20: f64 = (__rspice_deriv_cse_43 * p.p248);
        let eq176_e2219_d_b21: f64 = (__rspice_deriv_cse_44 * p.p248);
        let eq176_e2219_d_b22: f64 = (__rspice_deriv_cse_45 * p.p248);
        let eq176_e2219_d_b23: f64 = (__rspice_deriv_cse_46 * p.p248);
        let eq176_e2219_d_b24: f64 = (__rspice_deriv_cse_47 * p.p248);
        let eq176_e2219_d_b25: f64 = (__rspice_deriv_cse_48 * p.p248);
        let eq176_e2219_d_b26: f64 = (__rspice_deriv_cse_49 * p.p248);
        let eq176_e2219_d_b27: f64 = (__rspice_deriv_cse_50 * p.p248);
        let eq176_e2219_d_b28: f64 = (__rspice_deriv_cse_51 * p.p248);
        let eq176_e2219_d_b29: f64 = (__rspice_deriv_cse_52 * p.p248);
        let eq176_e2219_d_b30: f64 = (__rspice_deriv_cse_53 * p.p248);
        let eq176_e2219_d_b31: f64 = (__rspice_deriv_cse_54 * p.p248);
        let eq176_e2219_d_b32: f64 = (__rspice_deriv_cse_55 * p.p248);
        let eq176_e2219_d_b33: f64 = (__rspice_deriv_cse_56 * p.p248);
        let eq176_e2219_d_b34: f64 = (__rspice_deriv_cse_57 * p.p248);
        let eq176_e2219_d_b35: f64 = (__rspice_deriv_cse_58 * p.p248);
        let eq176_e2219_d_b36: f64 = (__rspice_deriv_cse_59 * p.p248);
        let eq176_e2219_d_b37: f64 = (__rspice_deriv_cse_60 * p.p248);
        let eq176_e2219_d_b38: f64 = (__rspice_deriv_cse_61 * p.p248);
        let eq176_e2219_d_b39: f64 = (__rspice_deriv_cse_62 * p.p248);
        let eq176_e2219_d_b40: f64 = (__rspice_deriv_cse_63 * p.p248);
        let eq176_e2219_d_b41: f64 = (__rspice_deriv_cse_64 * p.p248);
        let eq176_e2219_d_b42: f64 = (__rspice_deriv_cse_65 * p.p248);
        let eq176_e2219_d_b43: f64 = (__rspice_deriv_cse_66 * p.p248);
        let eq176_e2219_d_b44: f64 = (__rspice_deriv_cse_67 * p.p248);
        let eq176_e2219_d_b45: f64 = (__rspice_deriv_cse_68 * p.p248);
        let eq176_e2219_d_b46: f64 = (__rspice_deriv_cse_69 * p.p248);
        let eq176_e2219_d_b47: f64 = (__rspice_deriv_cse_70 * p.p248);
        let eq176_e2219_d_b48: f64 = (__rspice_deriv_cse_71 * p.p248);
        let eq176_e2219_d_b49: f64 = (__rspice_deriv_cse_72 * p.p248);
        let eq176_e2219_d_b50: f64 = (__rspice_deriv_cse_73 * p.p248);
        let eq176_e2219_d_b51: f64 = (__rspice_deriv_cse_74 * p.p248);
        let eq176_e2219_d_b52: f64 = (__rspice_deriv_cse_75 * p.p248);
        let eq176_e2219_d_b53: f64 = (__rspice_deriv_cse_76 * p.p248);
        let eq176_e2219_d_b54: f64 = (__rspice_deriv_cse_77 * p.p248);
        (eq176_e2219, eq176_e2219_d_n0, eq176_e2219_d_n1, eq176_e2219_d_n2, eq176_e2219_d_n3, eq176_e2219_d_n4, eq176_e2219_d_n5, eq176_e2219_d_n6, eq176_e2219_d_n7, eq176_e2219_d_n8, eq176_e2219_d_n9, eq176_e2219_d_n10, eq176_e2219_d_n11, eq176_e2219_d_n12, eq176_e2219_d_n13, eq176_e2219_d_n14, eq176_e2219_d_n15, eq176_e2219_d_n16, eq176_e2219_d_n17, eq176_e2219_d_n18, eq176_e2219_d_n19, eq176_e2219_d_n20, eq176_e2219_d_n21, eq176_e2219_d_n22, eq176_e2219_d_b0, eq176_e2219_d_b1, eq176_e2219_d_b2, eq176_e2219_d_b3, eq176_e2219_d_b4, eq176_e2219_d_b5, eq176_e2219_d_b6, eq176_e2219_d_b7, eq176_e2219_d_b8, eq176_e2219_d_b9, eq176_e2219_d_b10, eq176_e2219_d_b11, eq176_e2219_d_b12, eq176_e2219_d_b13, eq176_e2219_d_b14, eq176_e2219_d_b15, eq176_e2219_d_b16, eq176_e2219_d_b17, eq176_e2219_d_b18, eq176_e2219_d_b19, eq176_e2219_d_b20, eq176_e2219_d_b21, eq176_e2219_d_b22, eq176_e2219_d_b23, eq176_e2219_d_b24, eq176_e2219_d_b25, eq176_e2219_d_b26, eq176_e2219_d_b27, eq176_e2219_d_b28, eq176_e2219_d_b29, eq176_e2219_d_b30, eq176_e2219_d_b31, eq176_e2219_d_b32, eq176_e2219_d_b33, eq176_e2219_d_b34, eq176_e2219_d_b35, eq176_e2219_d_b36, eq176_e2219_d_b37, eq176_e2219_d_b38, eq176_e2219_d_b39, eq176_e2219_d_b40, eq176_e2219_d_b41, eq176_e2219_d_b42, eq176_e2219_d_b43, eq176_e2219_d_b44, eq176_e2219_d_b45, eq176_e2219_d_b46, eq176_e2219_d_b47, eq176_e2219_d_b48, eq176_e2219_d_b49, eq176_e2219_d_b50, eq176_e2219_d_b51, eq176_e2219_d_b52, eq176_e2219_d_b53, eq176_e2219_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq176_value: f64 = eq176_e2221;
        let eq176_node_derivatives: [f64; 23] = [eq176_e2221_d_n0, eq176_e2221_d_n1, eq176_e2221_d_n2, eq176_e2221_d_n3, eq176_e2221_d_n4, eq176_e2221_d_n5, eq176_e2221_d_n6, eq176_e2221_d_n7, eq176_e2221_d_n8, eq176_e2221_d_n9, eq176_e2221_d_n10, eq176_e2221_d_n11, eq176_e2221_d_n12, eq176_e2221_d_n13, eq176_e2221_d_n14, eq176_e2221_d_n15, eq176_e2221_d_n16, eq176_e2221_d_n17, eq176_e2221_d_n18, eq176_e2221_d_n19, eq176_e2221_d_n20, eq176_e2221_d_n21, eq176_e2221_d_n22];
        let eq176_branch_derivatives: [f64; 55] = [eq176_e2221_d_b0, eq176_e2221_d_b1, eq176_e2221_d_b2, eq176_e2221_d_b3, eq176_e2221_d_b4, eq176_e2221_d_b5, eq176_e2221_d_b6, eq176_e2221_d_b7, eq176_e2221_d_b8, eq176_e2221_d_b9, eq176_e2221_d_b10, eq176_e2221_d_b11, eq176_e2221_d_b12, eq176_e2221_d_b13, eq176_e2221_d_b14, eq176_e2221_d_b15, eq176_e2221_d_b16, eq176_e2221_d_b17, eq176_e2221_d_b18, eq176_e2221_d_b19, eq176_e2221_d_b20, eq176_e2221_d_b21, eq176_e2221_d_b22, eq176_e2221_d_b23, eq176_e2221_d_b24, eq176_e2221_d_b25, eq176_e2221_d_b26, eq176_e2221_d_b27, eq176_e2221_d_b28, eq176_e2221_d_b29, eq176_e2221_d_b30, eq176_e2221_d_b31, eq176_e2221_d_b32, eq176_e2221_d_b33, eq176_e2221_d_b34, eq176_e2221_d_b35, eq176_e2221_d_b36, eq176_e2221_d_b37, eq176_e2221_d_b38, eq176_e2221_d_b39, eq176_e2221_d_b40, eq176_e2221_d_b41, eq176_e2221_d_b42, eq176_e2221_d_b43, eq176_e2221_d_b44, eq176_e2221_d_b45, eq176_e2221_d_b46, eq176_e2221_d_b47, eq176_e2221_d_b48, eq176_e2221_d_b49, eq176_e2221_d_b50, eq176_e2221_d_b51, eq176_e2221_d_b52, eq176_e2221_d_b53, eq176_e2221_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq176_value),
            &eq176_node_derivatives,
            &eq176_branch_derivatives,
            multiplicity,
        );
        let (eq177_e2234, eq177_e2234_d_n0, eq177_e2234_d_n1, eq177_e2234_d_n2, eq177_e2234_d_n3, eq177_e2234_d_n4, eq177_e2234_d_n5, eq177_e2234_d_n6, eq177_e2234_d_n7, eq177_e2234_d_n8, eq177_e2234_d_n9, eq177_e2234_d_n10, eq177_e2234_d_n11, eq177_e2234_d_n12, eq177_e2234_d_n13, eq177_e2234_d_n14, eq177_e2234_d_n15, eq177_e2234_d_n16, eq177_e2234_d_n17, eq177_e2234_d_n18, eq177_e2234_d_n19, eq177_e2234_d_n20, eq177_e2234_d_n21, eq177_e2234_d_n22, eq177_e2234_d_b0, eq177_e2234_d_b1, eq177_e2234_d_b2, eq177_e2234_d_b3, eq177_e2234_d_b4, eq177_e2234_d_b5, eq177_e2234_d_b6, eq177_e2234_d_b7, eq177_e2234_d_b8, eq177_e2234_d_b9, eq177_e2234_d_b10, eq177_e2234_d_b11, eq177_e2234_d_b12, eq177_e2234_d_b13, eq177_e2234_d_b14, eq177_e2234_d_b15, eq177_e2234_d_b16, eq177_e2234_d_b17, eq177_e2234_d_b18, eq177_e2234_d_b19, eq177_e2234_d_b20, eq177_e2234_d_b21, eq177_e2234_d_b22, eq177_e2234_d_b23, eq177_e2234_d_b24, eq177_e2234_d_b25, eq177_e2234_d_b26, eq177_e2234_d_b27, eq177_e2234_d_b28, eq177_e2234_d_b29, eq177_e2234_d_b30, eq177_e2234_d_b31, eq177_e2234_d_b32, eq177_e2234_d_b33, eq177_e2234_d_b34, eq177_e2234_d_b35, eq177_e2234_d_b36, eq177_e2234_d_b37, eq177_e2234_d_b38, eq177_e2234_d_b39, eq177_e2234_d_b40, eq177_e2234_d_b41, eq177_e2234_d_b42, eq177_e2234_d_b43, eq177_e2234_d_b44, eq177_e2234_d_b45, eq177_e2234_d_b46, eq177_e2234_d_b47, eq177_e2234_d_b48, eq177_e2234_d_b49, eq177_e2234_d_b50, eq177_e2234_d_b51, eq177_e2234_d_b52, eq177_e2234_d_b53, eq177_e2234_d_b54,) = {
    if (((!s.b[590]) && s.b[593]) && (!s.b[594])) {
        let eq177_e2231: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 76, s.v[276]);
        let eq177_e2232: f64 = (p.p7 * eq177_e2231);
        (eq177_e2232, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq177_value: f64 = eq177_e2234;
        let eq177_node_derivatives: [f64; 23] = [eq177_e2234_d_n0, eq177_e2234_d_n1, eq177_e2234_d_n2, eq177_e2234_d_n3, eq177_e2234_d_n4, eq177_e2234_d_n5, eq177_e2234_d_n6, eq177_e2234_d_n7, eq177_e2234_d_n8, eq177_e2234_d_n9, eq177_e2234_d_n10, eq177_e2234_d_n11, eq177_e2234_d_n12, eq177_e2234_d_n13, eq177_e2234_d_n14, eq177_e2234_d_n15, eq177_e2234_d_n16, eq177_e2234_d_n17, eq177_e2234_d_n18, eq177_e2234_d_n19, eq177_e2234_d_n20, eq177_e2234_d_n21, eq177_e2234_d_n22];
        let eq177_branch_derivatives: [f64; 55] = [eq177_e2234_d_b0, eq177_e2234_d_b1, eq177_e2234_d_b2, eq177_e2234_d_b3, eq177_e2234_d_b4, eq177_e2234_d_b5, eq177_e2234_d_b6, eq177_e2234_d_b7, eq177_e2234_d_b8, eq177_e2234_d_b9, eq177_e2234_d_b10, eq177_e2234_d_b11, eq177_e2234_d_b12, eq177_e2234_d_b13, eq177_e2234_d_b14, eq177_e2234_d_b15, eq177_e2234_d_b16, eq177_e2234_d_b17, eq177_e2234_d_b18, eq177_e2234_d_b19, eq177_e2234_d_b20, eq177_e2234_d_b21, eq177_e2234_d_b22, eq177_e2234_d_b23, eq177_e2234_d_b24, eq177_e2234_d_b25, eq177_e2234_d_b26, eq177_e2234_d_b27, eq177_e2234_d_b28, eq177_e2234_d_b29, eq177_e2234_d_b30, eq177_e2234_d_b31, eq177_e2234_d_b32, eq177_e2234_d_b33, eq177_e2234_d_b34, eq177_e2234_d_b35, eq177_e2234_d_b36, eq177_e2234_d_b37, eq177_e2234_d_b38, eq177_e2234_d_b39, eq177_e2234_d_b40, eq177_e2234_d_b41, eq177_e2234_d_b42, eq177_e2234_d_b43, eq177_e2234_d_b44, eq177_e2234_d_b45, eq177_e2234_d_b46, eq177_e2234_d_b47, eq177_e2234_d_b48, eq177_e2234_d_b49, eq177_e2234_d_b50, eq177_e2234_d_b51, eq177_e2234_d_b52, eq177_e2234_d_b53, eq177_e2234_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq177_value),
            &eq177_node_derivatives,
            &eq177_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_34(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq178_e2249, eq178_e2249_d_n0, eq178_e2249_d_n1, eq178_e2249_d_n2, eq178_e2249_d_n3, eq178_e2249_d_n4, eq178_e2249_d_n5, eq178_e2249_d_n6, eq178_e2249_d_n7, eq178_e2249_d_n8, eq178_e2249_d_n9, eq178_e2249_d_n10, eq178_e2249_d_n11, eq178_e2249_d_n12, eq178_e2249_d_n13, eq178_e2249_d_n14, eq178_e2249_d_n15, eq178_e2249_d_n16, eq178_e2249_d_n17, eq178_e2249_d_n18, eq178_e2249_d_n19, eq178_e2249_d_n20, eq178_e2249_d_n21, eq178_e2249_d_n22, eq178_e2249_d_b0, eq178_e2249_d_b1, eq178_e2249_d_b2, eq178_e2249_d_b3, eq178_e2249_d_b4, eq178_e2249_d_b5, eq178_e2249_d_b6, eq178_e2249_d_b7, eq178_e2249_d_b8, eq178_e2249_d_b9, eq178_e2249_d_b10, eq178_e2249_d_b11, eq178_e2249_d_b12, eq178_e2249_d_b13, eq178_e2249_d_b14, eq178_e2249_d_b15, eq178_e2249_d_b16, eq178_e2249_d_b17, eq178_e2249_d_b18, eq178_e2249_d_b19, eq178_e2249_d_b20, eq178_e2249_d_b21, eq178_e2249_d_b22, eq178_e2249_d_b23, eq178_e2249_d_b24, eq178_e2249_d_b25, eq178_e2249_d_b26, eq178_e2249_d_b27, eq178_e2249_d_b28, eq178_e2249_d_b29, eq178_e2249_d_b30, eq178_e2249_d_b31, eq178_e2249_d_b32, eq178_e2249_d_b33, eq178_e2249_d_b34, eq178_e2249_d_b35, eq178_e2249_d_b36, eq178_e2249_d_b37, eq178_e2249_d_b38, eq178_e2249_d_b39, eq178_e2249_d_b40, eq178_e2249_d_b41, eq178_e2249_d_b42, eq178_e2249_d_b43, eq178_e2249_d_b44, eq178_e2249_d_b45, eq178_e2249_d_b46, eq178_e2249_d_b47, eq178_e2249_d_b48, eq178_e2249_d_b49, eq178_e2249_d_b50, eq178_e2249_d_b51, eq178_e2249_d_b52, eq178_e2249_d_b53, eq178_e2249_d_b54,) = {
    if (((!s.b[590]) && s.b[593]) && (!s.b[594])) {
        let eq178_e2244: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 77, s.v[276]);
        let eq178_e2245: f64 = (p.p7 * eq178_e2244);
        let eq178_e2245_d_n0: f64 = (p.p7 * (s.dn[276][0] * ddt_scale));
        let eq178_e2245_d_n1: f64 = (p.p7 * (s.dn[276][1] * ddt_scale));
        let eq178_e2245_d_n2: f64 = (p.p7 * (s.dn[276][2] * ddt_scale));
        let eq178_e2245_d_n3: f64 = (p.p7 * (s.dn[276][3] * ddt_scale));
        let eq178_e2245_d_n4: f64 = (p.p7 * (s.dn[276][4] * ddt_scale));
        let eq178_e2245_d_n5: f64 = (p.p7 * (s.dn[276][5] * ddt_scale));
        let eq178_e2245_d_n6: f64 = (p.p7 * (s.dn[276][6] * ddt_scale));
        let eq178_e2245_d_n7: f64 = (p.p7 * (s.dn[276][7] * ddt_scale));
        let eq178_e2245_d_n8: f64 = (p.p7 * (s.dn[276][8] * ddt_scale));
        let eq178_e2245_d_n9: f64 = (p.p7 * (s.dn[276][9] * ddt_scale));
        let eq178_e2245_d_n10: f64 = (p.p7 * (s.dn[276][10] * ddt_scale));
        let eq178_e2245_d_n11: f64 = (p.p7 * (s.dn[276][11] * ddt_scale));
        let eq178_e2245_d_n12: f64 = (p.p7 * (s.dn[276][12] * ddt_scale));
        let eq178_e2245_d_n13: f64 = (p.p7 * (s.dn[276][13] * ddt_scale));
        let eq178_e2245_d_n14: f64 = (p.p7 * (s.dn[276][14] * ddt_scale));
        let eq178_e2245_d_n15: f64 = (p.p7 * (s.dn[276][15] * ddt_scale));
        let eq178_e2245_d_n16: f64 = (p.p7 * (s.dn[276][16] * ddt_scale));
        let eq178_e2245_d_n17: f64 = (p.p7 * (s.dn[276][17] * ddt_scale));
        let eq178_e2245_d_n18: f64 = (p.p7 * (s.dn[276][18] * ddt_scale));
        let eq178_e2245_d_n19: f64 = (p.p7 * (s.dn[276][19] * ddt_scale));
        let eq178_e2245_d_n20: f64 = (p.p7 * (s.dn[276][20] * ddt_scale));
        let eq178_e2245_d_n21: f64 = (p.p7 * (s.dn[276][21] * ddt_scale));
        let eq178_e2245_d_n22: f64 = (p.p7 * (s.dn[276][22] * ddt_scale));
        let eq178_e2245_d_b0: f64 = (p.p7 * (s.db[276][0] * ddt_scale));
        let eq178_e2245_d_b1: f64 = (p.p7 * (s.db[276][1] * ddt_scale));
        let eq178_e2245_d_b2: f64 = (p.p7 * (s.db[276][2] * ddt_scale));
        let eq178_e2245_d_b3: f64 = (p.p7 * (s.db[276][3] * ddt_scale));
        let eq178_e2245_d_b4: f64 = (p.p7 * (s.db[276][4] * ddt_scale));
        let eq178_e2245_d_b5: f64 = (p.p7 * (s.db[276][5] * ddt_scale));
        let eq178_e2245_d_b6: f64 = (p.p7 * (s.db[276][6] * ddt_scale));
        let eq178_e2245_d_b7: f64 = (p.p7 * (s.db[276][7] * ddt_scale));
        let eq178_e2245_d_b8: f64 = (p.p7 * (s.db[276][8] * ddt_scale));
        let eq178_e2245_d_b9: f64 = (p.p7 * (s.db[276][9] * ddt_scale));
        let eq178_e2245_d_b10: f64 = (p.p7 * (s.db[276][10] * ddt_scale));
        let eq178_e2245_d_b11: f64 = (p.p7 * (s.db[276][11] * ddt_scale));
        let eq178_e2245_d_b12: f64 = (p.p7 * (s.db[276][12] * ddt_scale));
        let eq178_e2245_d_b13: f64 = (p.p7 * (s.db[276][13] * ddt_scale));
        let eq178_e2245_d_b14: f64 = (p.p7 * (s.db[276][14] * ddt_scale));
        let eq178_e2245_d_b15: f64 = (p.p7 * (s.db[276][15] * ddt_scale));
        let eq178_e2245_d_b16: f64 = (p.p7 * (s.db[276][16] * ddt_scale));
        let eq178_e2245_d_b17: f64 = (p.p7 * (s.db[276][17] * ddt_scale));
        let eq178_e2245_d_b18: f64 = (p.p7 * (s.db[276][18] * ddt_scale));
        let eq178_e2245_d_b19: f64 = (p.p7 * (s.db[276][19] * ddt_scale));
        let eq178_e2245_d_b20: f64 = (p.p7 * (s.db[276][20] * ddt_scale));
        let eq178_e2245_d_b21: f64 = (p.p7 * (s.db[276][21] * ddt_scale));
        let eq178_e2245_d_b22: f64 = (p.p7 * (s.db[276][22] * ddt_scale));
        let eq178_e2245_d_b23: f64 = (p.p7 * (s.db[276][23] * ddt_scale));
        let eq178_e2245_d_b24: f64 = (p.p7 * (s.db[276][24] * ddt_scale));
        let eq178_e2245_d_b25: f64 = (p.p7 * (s.db[276][25] * ddt_scale));
        let eq178_e2245_d_b26: f64 = (p.p7 * (s.db[276][26] * ddt_scale));
        let eq178_e2245_d_b27: f64 = (p.p7 * (s.db[276][27] * ddt_scale));
        let eq178_e2245_d_b28: f64 = (p.p7 * (s.db[276][28] * ddt_scale));
        let eq178_e2245_d_b29: f64 = (p.p7 * (s.db[276][29] * ddt_scale));
        let eq178_e2245_d_b30: f64 = (p.p7 * (s.db[276][30] * ddt_scale));
        let eq178_e2245_d_b31: f64 = (p.p7 * (s.db[276][31] * ddt_scale));
        let eq178_e2245_d_b32: f64 = (p.p7 * (s.db[276][32] * ddt_scale));
        let eq178_e2245_d_b33: f64 = (p.p7 * (s.db[276][33] * ddt_scale));
        let eq178_e2245_d_b34: f64 = (p.p7 * (s.db[276][34] * ddt_scale));
        let eq178_e2245_d_b35: f64 = (p.p7 * (s.db[276][35] * ddt_scale));
        let eq178_e2245_d_b36: f64 = (p.p7 * (s.db[276][36] * ddt_scale));
        let eq178_e2245_d_b37: f64 = (p.p7 * (s.db[276][37] * ddt_scale));
        let eq178_e2245_d_b38: f64 = (p.p7 * (s.db[276][38] * ddt_scale));
        let eq178_e2245_d_b39: f64 = (p.p7 * (s.db[276][39] * ddt_scale));
        let eq178_e2245_d_b40: f64 = (p.p7 * (s.db[276][40] * ddt_scale));
        let eq178_e2245_d_b41: f64 = (p.p7 * (s.db[276][41] * ddt_scale));
        let eq178_e2245_d_b42: f64 = (p.p7 * (s.db[276][42] * ddt_scale));
        let eq178_e2245_d_b43: f64 = (p.p7 * (s.db[276][43] * ddt_scale));
        let eq178_e2245_d_b44: f64 = (p.p7 * (s.db[276][44] * ddt_scale));
        let eq178_e2245_d_b45: f64 = (p.p7 * (s.db[276][45] * ddt_scale));
        let eq178_e2245_d_b46: f64 = (p.p7 * (s.db[276][46] * ddt_scale));
        let eq178_e2245_d_b47: f64 = (p.p7 * (s.db[276][47] * ddt_scale));
        let eq178_e2245_d_b48: f64 = (p.p7 * (s.db[276][48] * ddt_scale));
        let eq178_e2245_d_b49: f64 = (p.p7 * (s.db[276][49] * ddt_scale));
        let eq178_e2245_d_b50: f64 = (p.p7 * (s.db[276][50] * ddt_scale));
        let eq178_e2245_d_b51: f64 = (p.p7 * (s.db[276][51] * ddt_scale));
        let eq178_e2245_d_b52: f64 = (p.p7 * (s.db[276][52] * ddt_scale));
        let eq178_e2245_d_b53: f64 = (p.p7 * (s.db[276][53] * ddt_scale));
        let eq178_e2245_d_b54: f64 = (p.p7 * (s.db[276][54] * ddt_scale));
        let eq178_e2247: f64 = (eq178_e2245 * p.p248);
        let eq178_e2247_d_n0: f64 = (eq178_e2245_d_n0 * p.p248);
        let eq178_e2247_d_n1: f64 = (eq178_e2245_d_n1 * p.p248);
        let eq178_e2247_d_n2: f64 = (eq178_e2245_d_n2 * p.p248);
        let eq178_e2247_d_n3: f64 = (eq178_e2245_d_n3 * p.p248);
        let eq178_e2247_d_n4: f64 = (eq178_e2245_d_n4 * p.p248);
        let eq178_e2247_d_n5: f64 = (eq178_e2245_d_n5 * p.p248);
        let eq178_e2247_d_n6: f64 = (eq178_e2245_d_n6 * p.p248);
        let eq178_e2247_d_n7: f64 = (eq178_e2245_d_n7 * p.p248);
        let eq178_e2247_d_n8: f64 = (eq178_e2245_d_n8 * p.p248);
        let eq178_e2247_d_n9: f64 = (eq178_e2245_d_n9 * p.p248);
        let eq178_e2247_d_n10: f64 = (eq178_e2245_d_n10 * p.p248);
        let eq178_e2247_d_n11: f64 = (eq178_e2245_d_n11 * p.p248);
        let eq178_e2247_d_n12: f64 = (eq178_e2245_d_n12 * p.p248);
        let eq178_e2247_d_n13: f64 = (eq178_e2245_d_n13 * p.p248);
        let eq178_e2247_d_n14: f64 = (eq178_e2245_d_n14 * p.p248);
        let eq178_e2247_d_n15: f64 = (eq178_e2245_d_n15 * p.p248);
        let eq178_e2247_d_n16: f64 = (eq178_e2245_d_n16 * p.p248);
        let eq178_e2247_d_n17: f64 = (eq178_e2245_d_n17 * p.p248);
        let eq178_e2247_d_n18: f64 = (eq178_e2245_d_n18 * p.p248);
        let eq178_e2247_d_n19: f64 = (eq178_e2245_d_n19 * p.p248);
        let eq178_e2247_d_n20: f64 = (eq178_e2245_d_n20 * p.p248);
        let eq178_e2247_d_n21: f64 = (eq178_e2245_d_n21 * p.p248);
        let eq178_e2247_d_n22: f64 = (eq178_e2245_d_n22 * p.p248);
        let eq178_e2247_d_b0: f64 = (eq178_e2245_d_b0 * p.p248);
        let eq178_e2247_d_b1: f64 = (eq178_e2245_d_b1 * p.p248);
        let eq178_e2247_d_b2: f64 = (eq178_e2245_d_b2 * p.p248);
        let eq178_e2247_d_b3: f64 = (eq178_e2245_d_b3 * p.p248);
        let eq178_e2247_d_b4: f64 = (eq178_e2245_d_b4 * p.p248);
        let eq178_e2247_d_b5: f64 = (eq178_e2245_d_b5 * p.p248);
        let eq178_e2247_d_b6: f64 = (eq178_e2245_d_b6 * p.p248);
        let eq178_e2247_d_b7: f64 = (eq178_e2245_d_b7 * p.p248);
        let eq178_e2247_d_b8: f64 = (eq178_e2245_d_b8 * p.p248);
        let eq178_e2247_d_b9: f64 = (eq178_e2245_d_b9 * p.p248);
        let eq178_e2247_d_b10: f64 = (eq178_e2245_d_b10 * p.p248);
        let eq178_e2247_d_b11: f64 = (eq178_e2245_d_b11 * p.p248);
        let eq178_e2247_d_b12: f64 = (eq178_e2245_d_b12 * p.p248);
        let eq178_e2247_d_b13: f64 = (eq178_e2245_d_b13 * p.p248);
        let eq178_e2247_d_b14: f64 = (eq178_e2245_d_b14 * p.p248);
        let eq178_e2247_d_b15: f64 = (eq178_e2245_d_b15 * p.p248);
        let eq178_e2247_d_b16: f64 = (eq178_e2245_d_b16 * p.p248);
        let eq178_e2247_d_b17: f64 = (eq178_e2245_d_b17 * p.p248);
        let eq178_e2247_d_b18: f64 = (eq178_e2245_d_b18 * p.p248);
        let eq178_e2247_d_b19: f64 = (eq178_e2245_d_b19 * p.p248);
        let eq178_e2247_d_b20: f64 = (eq178_e2245_d_b20 * p.p248);
        let eq178_e2247_d_b21: f64 = (eq178_e2245_d_b21 * p.p248);
        let eq178_e2247_d_b22: f64 = (eq178_e2245_d_b22 * p.p248);
        let eq178_e2247_d_b23: f64 = (eq178_e2245_d_b23 * p.p248);
        let eq178_e2247_d_b24: f64 = (eq178_e2245_d_b24 * p.p248);
        let eq178_e2247_d_b25: f64 = (eq178_e2245_d_b25 * p.p248);
        let eq178_e2247_d_b26: f64 = (eq178_e2245_d_b26 * p.p248);
        let eq178_e2247_d_b27: f64 = (eq178_e2245_d_b27 * p.p248);
        let eq178_e2247_d_b28: f64 = (eq178_e2245_d_b28 * p.p248);
        let eq178_e2247_d_b29: f64 = (eq178_e2245_d_b29 * p.p248);
        let eq178_e2247_d_b30: f64 = (eq178_e2245_d_b30 * p.p248);
        let eq178_e2247_d_b31: f64 = (eq178_e2245_d_b31 * p.p248);
        let eq178_e2247_d_b32: f64 = (eq178_e2245_d_b32 * p.p248);
        let eq178_e2247_d_b33: f64 = (eq178_e2245_d_b33 * p.p248);
        let eq178_e2247_d_b34: f64 = (eq178_e2245_d_b34 * p.p248);
        let eq178_e2247_d_b35: f64 = (eq178_e2245_d_b35 * p.p248);
        let eq178_e2247_d_b36: f64 = (eq178_e2245_d_b36 * p.p248);
        let eq178_e2247_d_b37: f64 = (eq178_e2245_d_b37 * p.p248);
        let eq178_e2247_d_b38: f64 = (eq178_e2245_d_b38 * p.p248);
        let eq178_e2247_d_b39: f64 = (eq178_e2245_d_b39 * p.p248);
        let eq178_e2247_d_b40: f64 = (eq178_e2245_d_b40 * p.p248);
        let eq178_e2247_d_b41: f64 = (eq178_e2245_d_b41 * p.p248);
        let eq178_e2247_d_b42: f64 = (eq178_e2245_d_b42 * p.p248);
        let eq178_e2247_d_b43: f64 = (eq178_e2245_d_b43 * p.p248);
        let eq178_e2247_d_b44: f64 = (eq178_e2245_d_b44 * p.p248);
        let eq178_e2247_d_b45: f64 = (eq178_e2245_d_b45 * p.p248);
        let eq178_e2247_d_b46: f64 = (eq178_e2245_d_b46 * p.p248);
        let eq178_e2247_d_b47: f64 = (eq178_e2245_d_b47 * p.p248);
        let eq178_e2247_d_b48: f64 = (eq178_e2245_d_b48 * p.p248);
        let eq178_e2247_d_b49: f64 = (eq178_e2245_d_b49 * p.p248);
        let eq178_e2247_d_b50: f64 = (eq178_e2245_d_b50 * p.p248);
        let eq178_e2247_d_b51: f64 = (eq178_e2245_d_b51 * p.p248);
        let eq178_e2247_d_b52: f64 = (eq178_e2245_d_b52 * p.p248);
        let eq178_e2247_d_b53: f64 = (eq178_e2245_d_b53 * p.p248);
        let eq178_e2247_d_b54: f64 = (eq178_e2245_d_b54 * p.p248);
        (eq178_e2247, eq178_e2247_d_n0, eq178_e2247_d_n1, eq178_e2247_d_n2, eq178_e2247_d_n3, eq178_e2247_d_n4, eq178_e2247_d_n5, eq178_e2247_d_n6, eq178_e2247_d_n7, eq178_e2247_d_n8, eq178_e2247_d_n9, eq178_e2247_d_n10, eq178_e2247_d_n11, eq178_e2247_d_n12, eq178_e2247_d_n13, eq178_e2247_d_n14, eq178_e2247_d_n15, eq178_e2247_d_n16, eq178_e2247_d_n17, eq178_e2247_d_n18, eq178_e2247_d_n19, eq178_e2247_d_n20, eq178_e2247_d_n21, eq178_e2247_d_n22, eq178_e2247_d_b0, eq178_e2247_d_b1, eq178_e2247_d_b2, eq178_e2247_d_b3, eq178_e2247_d_b4, eq178_e2247_d_b5, eq178_e2247_d_b6, eq178_e2247_d_b7, eq178_e2247_d_b8, eq178_e2247_d_b9, eq178_e2247_d_b10, eq178_e2247_d_b11, eq178_e2247_d_b12, eq178_e2247_d_b13, eq178_e2247_d_b14, eq178_e2247_d_b15, eq178_e2247_d_b16, eq178_e2247_d_b17, eq178_e2247_d_b18, eq178_e2247_d_b19, eq178_e2247_d_b20, eq178_e2247_d_b21, eq178_e2247_d_b22, eq178_e2247_d_b23, eq178_e2247_d_b24, eq178_e2247_d_b25, eq178_e2247_d_b26, eq178_e2247_d_b27, eq178_e2247_d_b28, eq178_e2247_d_b29, eq178_e2247_d_b30, eq178_e2247_d_b31, eq178_e2247_d_b32, eq178_e2247_d_b33, eq178_e2247_d_b34, eq178_e2247_d_b35, eq178_e2247_d_b36, eq178_e2247_d_b37, eq178_e2247_d_b38, eq178_e2247_d_b39, eq178_e2247_d_b40, eq178_e2247_d_b41, eq178_e2247_d_b42, eq178_e2247_d_b43, eq178_e2247_d_b44, eq178_e2247_d_b45, eq178_e2247_d_b46, eq178_e2247_d_b47, eq178_e2247_d_b48, eq178_e2247_d_b49, eq178_e2247_d_b50, eq178_e2247_d_b51, eq178_e2247_d_b52, eq178_e2247_d_b53, eq178_e2247_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq178_value: f64 = eq178_e2249;
        let eq178_node_derivatives: [f64; 23] = [eq178_e2249_d_n0, eq178_e2249_d_n1, eq178_e2249_d_n2, eq178_e2249_d_n3, eq178_e2249_d_n4, eq178_e2249_d_n5, eq178_e2249_d_n6, eq178_e2249_d_n7, eq178_e2249_d_n8, eq178_e2249_d_n9, eq178_e2249_d_n10, eq178_e2249_d_n11, eq178_e2249_d_n12, eq178_e2249_d_n13, eq178_e2249_d_n14, eq178_e2249_d_n15, eq178_e2249_d_n16, eq178_e2249_d_n17, eq178_e2249_d_n18, eq178_e2249_d_n19, eq178_e2249_d_n20, eq178_e2249_d_n21, eq178_e2249_d_n22];
        let eq178_branch_derivatives: [f64; 55] = [eq178_e2249_d_b0, eq178_e2249_d_b1, eq178_e2249_d_b2, eq178_e2249_d_b3, eq178_e2249_d_b4, eq178_e2249_d_b5, eq178_e2249_d_b6, eq178_e2249_d_b7, eq178_e2249_d_b8, eq178_e2249_d_b9, eq178_e2249_d_b10, eq178_e2249_d_b11, eq178_e2249_d_b12, eq178_e2249_d_b13, eq178_e2249_d_b14, eq178_e2249_d_b15, eq178_e2249_d_b16, eq178_e2249_d_b17, eq178_e2249_d_b18, eq178_e2249_d_b19, eq178_e2249_d_b20, eq178_e2249_d_b21, eq178_e2249_d_b22, eq178_e2249_d_b23, eq178_e2249_d_b24, eq178_e2249_d_b25, eq178_e2249_d_b26, eq178_e2249_d_b27, eq178_e2249_d_b28, eq178_e2249_d_b29, eq178_e2249_d_b30, eq178_e2249_d_b31, eq178_e2249_d_b32, eq178_e2249_d_b33, eq178_e2249_d_b34, eq178_e2249_d_b35, eq178_e2249_d_b36, eq178_e2249_d_b37, eq178_e2249_d_b38, eq178_e2249_d_b39, eq178_e2249_d_b40, eq178_e2249_d_b41, eq178_e2249_d_b42, eq178_e2249_d_b43, eq178_e2249_d_b44, eq178_e2249_d_b45, eq178_e2249_d_b46, eq178_e2249_d_b47, eq178_e2249_d_b48, eq178_e2249_d_b49, eq178_e2249_d_b50, eq178_e2249_d_b51, eq178_e2249_d_b52, eq178_e2249_d_b53, eq178_e2249_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq178_value),
            &eq178_node_derivatives,
            &eq178_branch_derivatives,
            multiplicity,
        );
        let (eq179_e2261, eq179_e2261_d_n0, eq179_e2261_d_n1, eq179_e2261_d_n2, eq179_e2261_d_n3, eq179_e2261_d_n4, eq179_e2261_d_n5, eq179_e2261_d_n6, eq179_e2261_d_n7, eq179_e2261_d_n8, eq179_e2261_d_n9, eq179_e2261_d_n10, eq179_e2261_d_n11, eq179_e2261_d_n12, eq179_e2261_d_n13, eq179_e2261_d_n14, eq179_e2261_d_n15, eq179_e2261_d_n16, eq179_e2261_d_n17, eq179_e2261_d_n18, eq179_e2261_d_n19, eq179_e2261_d_n20, eq179_e2261_d_n21, eq179_e2261_d_n22, eq179_e2261_d_b0, eq179_e2261_d_b1, eq179_e2261_d_b2, eq179_e2261_d_b3, eq179_e2261_d_b4, eq179_e2261_d_b5, eq179_e2261_d_b6, eq179_e2261_d_b7, eq179_e2261_d_b8, eq179_e2261_d_b9, eq179_e2261_d_b10, eq179_e2261_d_b11, eq179_e2261_d_b12, eq179_e2261_d_b13, eq179_e2261_d_b14, eq179_e2261_d_b15, eq179_e2261_d_b16, eq179_e2261_d_b17, eq179_e2261_d_b18, eq179_e2261_d_b19, eq179_e2261_d_b20, eq179_e2261_d_b21, eq179_e2261_d_b22, eq179_e2261_d_b23, eq179_e2261_d_b24, eq179_e2261_d_b25, eq179_e2261_d_b26, eq179_e2261_d_b27, eq179_e2261_d_b28, eq179_e2261_d_b29, eq179_e2261_d_b30, eq179_e2261_d_b31, eq179_e2261_d_b32, eq179_e2261_d_b33, eq179_e2261_d_b34, eq179_e2261_d_b35, eq179_e2261_d_b36, eq179_e2261_d_b37, eq179_e2261_d_b38, eq179_e2261_d_b39, eq179_e2261_d_b40, eq179_e2261_d_b41, eq179_e2261_d_b42, eq179_e2261_d_b43, eq179_e2261_d_b44, eq179_e2261_d_b45, eq179_e2261_d_b46, eq179_e2261_d_b47, eq179_e2261_d_b48, eq179_e2261_d_b49, eq179_e2261_d_b50, eq179_e2261_d_b51, eq179_e2261_d_b52, eq179_e2261_d_b53, eq179_e2261_d_b54,) = {
    if ((!s.b[590]) && s.b[593]) {
        let eq179_e2257: f64 = (p.p253 * s.v[276]);
        let eq179_e2258: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 78, eq179_e2257);
        let eq179_e2258_d_n0: f64 = ((p.p253 * s.dn[276][0]) * ddt_scale);
        let eq179_e2258_d_n1: f64 = ((p.p253 * s.dn[276][1]) * ddt_scale);
        let eq179_e2258_d_n2: f64 = ((p.p253 * s.dn[276][2]) * ddt_scale);
        let eq179_e2258_d_n3: f64 = ((p.p253 * s.dn[276][3]) * ddt_scale);
        let eq179_e2258_d_n4: f64 = ((p.p253 * s.dn[276][4]) * ddt_scale);
        let eq179_e2258_d_n5: f64 = ((p.p253 * s.dn[276][5]) * ddt_scale);
        let eq179_e2258_d_n6: f64 = ((p.p253 * s.dn[276][6]) * ddt_scale);
        let eq179_e2258_d_n7: f64 = ((p.p253 * s.dn[276][7]) * ddt_scale);
        let eq179_e2258_d_n8: f64 = ((p.p253 * s.dn[276][8]) * ddt_scale);
        let eq179_e2258_d_n9: f64 = ((p.p253 * s.dn[276][9]) * ddt_scale);
        let eq179_e2258_d_n10: f64 = ((p.p253 * s.dn[276][10]) * ddt_scale);
        let eq179_e2258_d_n11: f64 = ((p.p253 * s.dn[276][11]) * ddt_scale);
        let eq179_e2258_d_n12: f64 = ((p.p253 * s.dn[276][12]) * ddt_scale);
        let eq179_e2258_d_n13: f64 = ((p.p253 * s.dn[276][13]) * ddt_scale);
        let eq179_e2258_d_n14: f64 = ((p.p253 * s.dn[276][14]) * ddt_scale);
        let eq179_e2258_d_n15: f64 = ((p.p253 * s.dn[276][15]) * ddt_scale);
        let eq179_e2258_d_n16: f64 = ((p.p253 * s.dn[276][16]) * ddt_scale);
        let eq179_e2258_d_n17: f64 = ((p.p253 * s.dn[276][17]) * ddt_scale);
        let eq179_e2258_d_n18: f64 = ((p.p253 * s.dn[276][18]) * ddt_scale);
        let eq179_e2258_d_n19: f64 = ((p.p253 * s.dn[276][19]) * ddt_scale);
        let eq179_e2258_d_n20: f64 = ((p.p253 * s.dn[276][20]) * ddt_scale);
        let eq179_e2258_d_n21: f64 = ((p.p253 * s.dn[276][21]) * ddt_scale);
        let eq179_e2258_d_n22: f64 = ((p.p253 * s.dn[276][22]) * ddt_scale);
        let eq179_e2258_d_b0: f64 = ((p.p253 * s.db[276][0]) * ddt_scale);
        let eq179_e2258_d_b1: f64 = ((p.p253 * s.db[276][1]) * ddt_scale);
        let eq179_e2258_d_b2: f64 = ((p.p253 * s.db[276][2]) * ddt_scale);
        let eq179_e2258_d_b3: f64 = ((p.p253 * s.db[276][3]) * ddt_scale);
        let eq179_e2258_d_b4: f64 = ((p.p253 * s.db[276][4]) * ddt_scale);
        let eq179_e2258_d_b5: f64 = ((p.p253 * s.db[276][5]) * ddt_scale);
        let eq179_e2258_d_b6: f64 = ((p.p253 * s.db[276][6]) * ddt_scale);
        let eq179_e2258_d_b7: f64 = ((p.p253 * s.db[276][7]) * ddt_scale);
        let eq179_e2258_d_b8: f64 = ((p.p253 * s.db[276][8]) * ddt_scale);
        let eq179_e2258_d_b9: f64 = ((p.p253 * s.db[276][9]) * ddt_scale);
        let eq179_e2258_d_b10: f64 = ((p.p253 * s.db[276][10]) * ddt_scale);
        let eq179_e2258_d_b11: f64 = ((p.p253 * s.db[276][11]) * ddt_scale);
        let eq179_e2258_d_b12: f64 = ((p.p253 * s.db[276][12]) * ddt_scale);
        let eq179_e2258_d_b13: f64 = ((p.p253 * s.db[276][13]) * ddt_scale);
        let eq179_e2258_d_b14: f64 = ((p.p253 * s.db[276][14]) * ddt_scale);
        let eq179_e2258_d_b15: f64 = ((p.p253 * s.db[276][15]) * ddt_scale);
        let eq179_e2258_d_b16: f64 = ((p.p253 * s.db[276][16]) * ddt_scale);
        let eq179_e2258_d_b17: f64 = ((p.p253 * s.db[276][17]) * ddt_scale);
        let eq179_e2258_d_b18: f64 = ((p.p253 * s.db[276][18]) * ddt_scale);
        let eq179_e2258_d_b19: f64 = ((p.p253 * s.db[276][19]) * ddt_scale);
        let eq179_e2258_d_b20: f64 = ((p.p253 * s.db[276][20]) * ddt_scale);
        let eq179_e2258_d_b21: f64 = ((p.p253 * s.db[276][21]) * ddt_scale);
        let eq179_e2258_d_b22: f64 = ((p.p253 * s.db[276][22]) * ddt_scale);
        let eq179_e2258_d_b23: f64 = ((p.p253 * s.db[276][23]) * ddt_scale);
        let eq179_e2258_d_b24: f64 = ((p.p253 * s.db[276][24]) * ddt_scale);
        let eq179_e2258_d_b25: f64 = ((p.p253 * s.db[276][25]) * ddt_scale);
        let eq179_e2258_d_b26: f64 = ((p.p253 * s.db[276][26]) * ddt_scale);
        let eq179_e2258_d_b27: f64 = ((p.p253 * s.db[276][27]) * ddt_scale);
        let eq179_e2258_d_b28: f64 = ((p.p253 * s.db[276][28]) * ddt_scale);
        let eq179_e2258_d_b29: f64 = ((p.p253 * s.db[276][29]) * ddt_scale);
        let eq179_e2258_d_b30: f64 = ((p.p253 * s.db[276][30]) * ddt_scale);
        let eq179_e2258_d_b31: f64 = ((p.p253 * s.db[276][31]) * ddt_scale);
        let eq179_e2258_d_b32: f64 = ((p.p253 * s.db[276][32]) * ddt_scale);
        let eq179_e2258_d_b33: f64 = ((p.p253 * s.db[276][33]) * ddt_scale);
        let eq179_e2258_d_b34: f64 = ((p.p253 * s.db[276][34]) * ddt_scale);
        let eq179_e2258_d_b35: f64 = ((p.p253 * s.db[276][35]) * ddt_scale);
        let eq179_e2258_d_b36: f64 = ((p.p253 * s.db[276][36]) * ddt_scale);
        let eq179_e2258_d_b37: f64 = ((p.p253 * s.db[276][37]) * ddt_scale);
        let eq179_e2258_d_b38: f64 = ((p.p253 * s.db[276][38]) * ddt_scale);
        let eq179_e2258_d_b39: f64 = ((p.p253 * s.db[276][39]) * ddt_scale);
        let eq179_e2258_d_b40: f64 = ((p.p253 * s.db[276][40]) * ddt_scale);
        let eq179_e2258_d_b41: f64 = ((p.p253 * s.db[276][41]) * ddt_scale);
        let eq179_e2258_d_b42: f64 = ((p.p253 * s.db[276][42]) * ddt_scale);
        let eq179_e2258_d_b43: f64 = ((p.p253 * s.db[276][43]) * ddt_scale);
        let eq179_e2258_d_b44: f64 = ((p.p253 * s.db[276][44]) * ddt_scale);
        let eq179_e2258_d_b45: f64 = ((p.p253 * s.db[276][45]) * ddt_scale);
        let eq179_e2258_d_b46: f64 = ((p.p253 * s.db[276][46]) * ddt_scale);
        let eq179_e2258_d_b47: f64 = ((p.p253 * s.db[276][47]) * ddt_scale);
        let eq179_e2258_d_b48: f64 = ((p.p253 * s.db[276][48]) * ddt_scale);
        let eq179_e2258_d_b49: f64 = ((p.p253 * s.db[276][49]) * ddt_scale);
        let eq179_e2258_d_b50: f64 = ((p.p253 * s.db[276][50]) * ddt_scale);
        let eq179_e2258_d_b51: f64 = ((p.p253 * s.db[276][51]) * ddt_scale);
        let eq179_e2258_d_b52: f64 = ((p.p253 * s.db[276][52]) * ddt_scale);
        let eq179_e2258_d_b53: f64 = ((p.p253 * s.db[276][53]) * ddt_scale);
        let eq179_e2258_d_b54: f64 = ((p.p253 * s.db[276][54]) * ddt_scale);
        let eq179_e2259: f64 = (p.p7 * eq179_e2258);
        let eq179_e2259_d_n0: f64 = (p.p7 * eq179_e2258_d_n0);
        let eq179_e2259_d_n1: f64 = (p.p7 * eq179_e2258_d_n1);
        let eq179_e2259_d_n2: f64 = (p.p7 * eq179_e2258_d_n2);
        let eq179_e2259_d_n3: f64 = (p.p7 * eq179_e2258_d_n3);
        let eq179_e2259_d_n4: f64 = (p.p7 * eq179_e2258_d_n4);
        let eq179_e2259_d_n5: f64 = (p.p7 * eq179_e2258_d_n5);
        let eq179_e2259_d_n6: f64 = (p.p7 * eq179_e2258_d_n6);
        let eq179_e2259_d_n7: f64 = (p.p7 * eq179_e2258_d_n7);
        let eq179_e2259_d_n8: f64 = (p.p7 * eq179_e2258_d_n8);
        let eq179_e2259_d_n9: f64 = (p.p7 * eq179_e2258_d_n9);
        let eq179_e2259_d_n10: f64 = (p.p7 * eq179_e2258_d_n10);
        let eq179_e2259_d_n11: f64 = (p.p7 * eq179_e2258_d_n11);
        let eq179_e2259_d_n12: f64 = (p.p7 * eq179_e2258_d_n12);
        let eq179_e2259_d_n13: f64 = (p.p7 * eq179_e2258_d_n13);
        let eq179_e2259_d_n14: f64 = (p.p7 * eq179_e2258_d_n14);
        let eq179_e2259_d_n15: f64 = (p.p7 * eq179_e2258_d_n15);
        let eq179_e2259_d_n16: f64 = (p.p7 * eq179_e2258_d_n16);
        let eq179_e2259_d_n17: f64 = (p.p7 * eq179_e2258_d_n17);
        let eq179_e2259_d_n18: f64 = (p.p7 * eq179_e2258_d_n18);
        let eq179_e2259_d_n19: f64 = (p.p7 * eq179_e2258_d_n19);
        let eq179_e2259_d_n20: f64 = (p.p7 * eq179_e2258_d_n20);
        let eq179_e2259_d_n21: f64 = (p.p7 * eq179_e2258_d_n21);
        let eq179_e2259_d_n22: f64 = (p.p7 * eq179_e2258_d_n22);
        let eq179_e2259_d_b0: f64 = (p.p7 * eq179_e2258_d_b0);
        let eq179_e2259_d_b1: f64 = (p.p7 * eq179_e2258_d_b1);
        let eq179_e2259_d_b2: f64 = (p.p7 * eq179_e2258_d_b2);
        let eq179_e2259_d_b3: f64 = (p.p7 * eq179_e2258_d_b3);
        let eq179_e2259_d_b4: f64 = (p.p7 * eq179_e2258_d_b4);
        let eq179_e2259_d_b5: f64 = (p.p7 * eq179_e2258_d_b5);
        let eq179_e2259_d_b6: f64 = (p.p7 * eq179_e2258_d_b6);
        let eq179_e2259_d_b7: f64 = (p.p7 * eq179_e2258_d_b7);
        let eq179_e2259_d_b8: f64 = (p.p7 * eq179_e2258_d_b8);
        let eq179_e2259_d_b9: f64 = (p.p7 * eq179_e2258_d_b9);
        let eq179_e2259_d_b10: f64 = (p.p7 * eq179_e2258_d_b10);
        let eq179_e2259_d_b11: f64 = (p.p7 * eq179_e2258_d_b11);
        let eq179_e2259_d_b12: f64 = (p.p7 * eq179_e2258_d_b12);
        let eq179_e2259_d_b13: f64 = (p.p7 * eq179_e2258_d_b13);
        let eq179_e2259_d_b14: f64 = (p.p7 * eq179_e2258_d_b14);
        let eq179_e2259_d_b15: f64 = (p.p7 * eq179_e2258_d_b15);
        let eq179_e2259_d_b16: f64 = (p.p7 * eq179_e2258_d_b16);
        let eq179_e2259_d_b17: f64 = (p.p7 * eq179_e2258_d_b17);
        let eq179_e2259_d_b18: f64 = (p.p7 * eq179_e2258_d_b18);
        let eq179_e2259_d_b19: f64 = (p.p7 * eq179_e2258_d_b19);
        let eq179_e2259_d_b20: f64 = (p.p7 * eq179_e2258_d_b20);
        let eq179_e2259_d_b21: f64 = (p.p7 * eq179_e2258_d_b21);
        let eq179_e2259_d_b22: f64 = (p.p7 * eq179_e2258_d_b22);
        let eq179_e2259_d_b23: f64 = (p.p7 * eq179_e2258_d_b23);
        let eq179_e2259_d_b24: f64 = (p.p7 * eq179_e2258_d_b24);
        let eq179_e2259_d_b25: f64 = (p.p7 * eq179_e2258_d_b25);
        let eq179_e2259_d_b26: f64 = (p.p7 * eq179_e2258_d_b26);
        let eq179_e2259_d_b27: f64 = (p.p7 * eq179_e2258_d_b27);
        let eq179_e2259_d_b28: f64 = (p.p7 * eq179_e2258_d_b28);
        let eq179_e2259_d_b29: f64 = (p.p7 * eq179_e2258_d_b29);
        let eq179_e2259_d_b30: f64 = (p.p7 * eq179_e2258_d_b30);
        let eq179_e2259_d_b31: f64 = (p.p7 * eq179_e2258_d_b31);
        let eq179_e2259_d_b32: f64 = (p.p7 * eq179_e2258_d_b32);
        let eq179_e2259_d_b33: f64 = (p.p7 * eq179_e2258_d_b33);
        let eq179_e2259_d_b34: f64 = (p.p7 * eq179_e2258_d_b34);
        let eq179_e2259_d_b35: f64 = (p.p7 * eq179_e2258_d_b35);
        let eq179_e2259_d_b36: f64 = (p.p7 * eq179_e2258_d_b36);
        let eq179_e2259_d_b37: f64 = (p.p7 * eq179_e2258_d_b37);
        let eq179_e2259_d_b38: f64 = (p.p7 * eq179_e2258_d_b38);
        let eq179_e2259_d_b39: f64 = (p.p7 * eq179_e2258_d_b39);
        let eq179_e2259_d_b40: f64 = (p.p7 * eq179_e2258_d_b40);
        let eq179_e2259_d_b41: f64 = (p.p7 * eq179_e2258_d_b41);
        let eq179_e2259_d_b42: f64 = (p.p7 * eq179_e2258_d_b42);
        let eq179_e2259_d_b43: f64 = (p.p7 * eq179_e2258_d_b43);
        let eq179_e2259_d_b44: f64 = (p.p7 * eq179_e2258_d_b44);
        let eq179_e2259_d_b45: f64 = (p.p7 * eq179_e2258_d_b45);
        let eq179_e2259_d_b46: f64 = (p.p7 * eq179_e2258_d_b46);
        let eq179_e2259_d_b47: f64 = (p.p7 * eq179_e2258_d_b47);
        let eq179_e2259_d_b48: f64 = (p.p7 * eq179_e2258_d_b48);
        let eq179_e2259_d_b49: f64 = (p.p7 * eq179_e2258_d_b49);
        let eq179_e2259_d_b50: f64 = (p.p7 * eq179_e2258_d_b50);
        let eq179_e2259_d_b51: f64 = (p.p7 * eq179_e2258_d_b51);
        let eq179_e2259_d_b52: f64 = (p.p7 * eq179_e2258_d_b52);
        let eq179_e2259_d_b53: f64 = (p.p7 * eq179_e2258_d_b53);
        let eq179_e2259_d_b54: f64 = (p.p7 * eq179_e2258_d_b54);
        (eq179_e2259, eq179_e2259_d_n0, eq179_e2259_d_n1, eq179_e2259_d_n2, eq179_e2259_d_n3, eq179_e2259_d_n4, eq179_e2259_d_n5, eq179_e2259_d_n6, eq179_e2259_d_n7, eq179_e2259_d_n8, eq179_e2259_d_n9, eq179_e2259_d_n10, eq179_e2259_d_n11, eq179_e2259_d_n12, eq179_e2259_d_n13, eq179_e2259_d_n14, eq179_e2259_d_n15, eq179_e2259_d_n16, eq179_e2259_d_n17, eq179_e2259_d_n18, eq179_e2259_d_n19, eq179_e2259_d_n20, eq179_e2259_d_n21, eq179_e2259_d_n22, eq179_e2259_d_b0, eq179_e2259_d_b1, eq179_e2259_d_b2, eq179_e2259_d_b3, eq179_e2259_d_b4, eq179_e2259_d_b5, eq179_e2259_d_b6, eq179_e2259_d_b7, eq179_e2259_d_b8, eq179_e2259_d_b9, eq179_e2259_d_b10, eq179_e2259_d_b11, eq179_e2259_d_b12, eq179_e2259_d_b13, eq179_e2259_d_b14, eq179_e2259_d_b15, eq179_e2259_d_b16, eq179_e2259_d_b17, eq179_e2259_d_b18, eq179_e2259_d_b19, eq179_e2259_d_b20, eq179_e2259_d_b21, eq179_e2259_d_b22, eq179_e2259_d_b23, eq179_e2259_d_b24, eq179_e2259_d_b25, eq179_e2259_d_b26, eq179_e2259_d_b27, eq179_e2259_d_b28, eq179_e2259_d_b29, eq179_e2259_d_b30, eq179_e2259_d_b31, eq179_e2259_d_b32, eq179_e2259_d_b33, eq179_e2259_d_b34, eq179_e2259_d_b35, eq179_e2259_d_b36, eq179_e2259_d_b37, eq179_e2259_d_b38, eq179_e2259_d_b39, eq179_e2259_d_b40, eq179_e2259_d_b41, eq179_e2259_d_b42, eq179_e2259_d_b43, eq179_e2259_d_b44, eq179_e2259_d_b45, eq179_e2259_d_b46, eq179_e2259_d_b47, eq179_e2259_d_b48, eq179_e2259_d_b49, eq179_e2259_d_b50, eq179_e2259_d_b51, eq179_e2259_d_b52, eq179_e2259_d_b53, eq179_e2259_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq179_value: f64 = eq179_e2261;
        let eq179_node_derivatives: [f64; 23] = [eq179_e2261_d_n0, eq179_e2261_d_n1, eq179_e2261_d_n2, eq179_e2261_d_n3, eq179_e2261_d_n4, eq179_e2261_d_n5, eq179_e2261_d_n6, eq179_e2261_d_n7, eq179_e2261_d_n8, eq179_e2261_d_n9, eq179_e2261_d_n10, eq179_e2261_d_n11, eq179_e2261_d_n12, eq179_e2261_d_n13, eq179_e2261_d_n14, eq179_e2261_d_n15, eq179_e2261_d_n16, eq179_e2261_d_n17, eq179_e2261_d_n18, eq179_e2261_d_n19, eq179_e2261_d_n20, eq179_e2261_d_n21, eq179_e2261_d_n22];
        let eq179_branch_derivatives: [f64; 55] = [eq179_e2261_d_b0, eq179_e2261_d_b1, eq179_e2261_d_b2, eq179_e2261_d_b3, eq179_e2261_d_b4, eq179_e2261_d_b5, eq179_e2261_d_b6, eq179_e2261_d_b7, eq179_e2261_d_b8, eq179_e2261_d_b9, eq179_e2261_d_b10, eq179_e2261_d_b11, eq179_e2261_d_b12, eq179_e2261_d_b13, eq179_e2261_d_b14, eq179_e2261_d_b15, eq179_e2261_d_b16, eq179_e2261_d_b17, eq179_e2261_d_b18, eq179_e2261_d_b19, eq179_e2261_d_b20, eq179_e2261_d_b21, eq179_e2261_d_b22, eq179_e2261_d_b23, eq179_e2261_d_b24, eq179_e2261_d_b25, eq179_e2261_d_b26, eq179_e2261_d_b27, eq179_e2261_d_b28, eq179_e2261_d_b29, eq179_e2261_d_b30, eq179_e2261_d_b31, eq179_e2261_d_b32, eq179_e2261_d_b33, eq179_e2261_d_b34, eq179_e2261_d_b35, eq179_e2261_d_b36, eq179_e2261_d_b37, eq179_e2261_d_b38, eq179_e2261_d_b39, eq179_e2261_d_b40, eq179_e2261_d_b41, eq179_e2261_d_b42, eq179_e2261_d_b43, eq179_e2261_d_b44, eq179_e2261_d_b45, eq179_e2261_d_b46, eq179_e2261_d_b47, eq179_e2261_d_b48, eq179_e2261_d_b49, eq179_e2261_d_b50, eq179_e2261_d_b51, eq179_e2261_d_b52, eq179_e2261_d_b53, eq179_e2261_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq179_value),
            &eq179_node_derivatives,
            &eq179_branch_derivatives,
            multiplicity,
        );
        let (eq180_e2270, eq180_e2270_d_n0, eq180_e2270_d_n1, eq180_e2270_d_n2, eq180_e2270_d_n3, eq180_e2270_d_n4, eq180_e2270_d_n5, eq180_e2270_d_n6, eq180_e2270_d_n7, eq180_e2270_d_n8, eq180_e2270_d_n9, eq180_e2270_d_n10, eq180_e2270_d_n11, eq180_e2270_d_n12, eq180_e2270_d_n13, eq180_e2270_d_n14, eq180_e2270_d_n15, eq180_e2270_d_n16, eq180_e2270_d_n17, eq180_e2270_d_n18, eq180_e2270_d_n19, eq180_e2270_d_n20, eq180_e2270_d_n21, eq180_e2270_d_n22, eq180_e2270_d_b0, eq180_e2270_d_b1, eq180_e2270_d_b2, eq180_e2270_d_b3, eq180_e2270_d_b4, eq180_e2270_d_b5, eq180_e2270_d_b6, eq180_e2270_d_b7, eq180_e2270_d_b8, eq180_e2270_d_b9, eq180_e2270_d_b10, eq180_e2270_d_b11, eq180_e2270_d_b12, eq180_e2270_d_b13, eq180_e2270_d_b14, eq180_e2270_d_b15, eq180_e2270_d_b16, eq180_e2270_d_b17, eq180_e2270_d_b18, eq180_e2270_d_b19, eq180_e2270_d_b20, eq180_e2270_d_b21, eq180_e2270_d_b22, eq180_e2270_d_b23, eq180_e2270_d_b24, eq180_e2270_d_b25, eq180_e2270_d_b26, eq180_e2270_d_b27, eq180_e2270_d_b28, eq180_e2270_d_b29, eq180_e2270_d_b30, eq180_e2270_d_b31, eq180_e2270_d_b32, eq180_e2270_d_b33, eq180_e2270_d_b34, eq180_e2270_d_b35, eq180_e2270_d_b36, eq180_e2270_d_b37, eq180_e2270_d_b38, eq180_e2270_d_b39, eq180_e2270_d_b40, eq180_e2270_d_b41, eq180_e2270_d_b42, eq180_e2270_d_b43, eq180_e2270_d_b44, eq180_e2270_d_b45, eq180_e2270_d_b46, eq180_e2270_d_b47, eq180_e2270_d_b48, eq180_e2270_d_b49, eq180_e2270_d_b50, eq180_e2270_d_b51, eq180_e2270_d_b52, eq180_e2270_d_b53, eq180_e2270_d_b54,) = {
    if (s.b[595] && s.b[596]) {
        let eq180_e2267: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 79, s.v[289]);
        let eq180_e2268: f64 = (p.p7 * eq180_e2267);
        let eq180_e2268_d_n0: f64 = (p.p7 * (s.dn[289][0] * ddt_scale));
        let eq180_e2268_d_n1: f64 = (p.p7 * (s.dn[289][1] * ddt_scale));
        let eq180_e2268_d_n2: f64 = (p.p7 * (s.dn[289][2] * ddt_scale));
        let eq180_e2268_d_n3: f64 = (p.p7 * (s.dn[289][3] * ddt_scale));
        let eq180_e2268_d_n4: f64 = (p.p7 * (s.dn[289][4] * ddt_scale));
        let eq180_e2268_d_n5: f64 = (p.p7 * (s.dn[289][5] * ddt_scale));
        let eq180_e2268_d_n6: f64 = (p.p7 * (s.dn[289][6] * ddt_scale));
        let eq180_e2268_d_n7: f64 = (p.p7 * (s.dn[289][7] * ddt_scale));
        let eq180_e2268_d_n8: f64 = (p.p7 * (s.dn[289][8] * ddt_scale));
        let eq180_e2268_d_n9: f64 = (p.p7 * (s.dn[289][9] * ddt_scale));
        let eq180_e2268_d_n10: f64 = (p.p7 * (s.dn[289][10] * ddt_scale));
        let eq180_e2268_d_n11: f64 = (p.p7 * (s.dn[289][11] * ddt_scale));
        let eq180_e2268_d_n12: f64 = (p.p7 * (s.dn[289][12] * ddt_scale));
        let eq180_e2268_d_n13: f64 = (p.p7 * (s.dn[289][13] * ddt_scale));
        let eq180_e2268_d_n14: f64 = (p.p7 * (s.dn[289][14] * ddt_scale));
        let eq180_e2268_d_n15: f64 = (p.p7 * (s.dn[289][15] * ddt_scale));
        let eq180_e2268_d_n16: f64 = (p.p7 * (s.dn[289][16] * ddt_scale));
        let eq180_e2268_d_n17: f64 = (p.p7 * (s.dn[289][17] * ddt_scale));
        let eq180_e2268_d_n18: f64 = (p.p7 * (s.dn[289][18] * ddt_scale));
        let eq180_e2268_d_n19: f64 = (p.p7 * (s.dn[289][19] * ddt_scale));
        let eq180_e2268_d_n20: f64 = (p.p7 * (s.dn[289][20] * ddt_scale));
        let eq180_e2268_d_n21: f64 = (p.p7 * (s.dn[289][21] * ddt_scale));
        let eq180_e2268_d_n22: f64 = (p.p7 * (s.dn[289][22] * ddt_scale));
        let eq180_e2268_d_b0: f64 = (p.p7 * (s.db[289][0] * ddt_scale));
        let eq180_e2268_d_b1: f64 = (p.p7 * (s.db[289][1] * ddt_scale));
        let eq180_e2268_d_b2: f64 = (p.p7 * (s.db[289][2] * ddt_scale));
        let eq180_e2268_d_b3: f64 = (p.p7 * (s.db[289][3] * ddt_scale));
        let eq180_e2268_d_b4: f64 = (p.p7 * (s.db[289][4] * ddt_scale));
        let eq180_e2268_d_b5: f64 = (p.p7 * (s.db[289][5] * ddt_scale));
        let eq180_e2268_d_b6: f64 = (p.p7 * (s.db[289][6] * ddt_scale));
        let eq180_e2268_d_b7: f64 = (p.p7 * (s.db[289][7] * ddt_scale));
        let eq180_e2268_d_b8: f64 = (p.p7 * (s.db[289][8] * ddt_scale));
        let eq180_e2268_d_b9: f64 = (p.p7 * (s.db[289][9] * ddt_scale));
        let eq180_e2268_d_b10: f64 = (p.p7 * (s.db[289][10] * ddt_scale));
        let eq180_e2268_d_b11: f64 = (p.p7 * (s.db[289][11] * ddt_scale));
        let eq180_e2268_d_b12: f64 = (p.p7 * (s.db[289][12] * ddt_scale));
        let eq180_e2268_d_b13: f64 = (p.p7 * (s.db[289][13] * ddt_scale));
        let eq180_e2268_d_b14: f64 = (p.p7 * (s.db[289][14] * ddt_scale));
        let eq180_e2268_d_b15: f64 = (p.p7 * (s.db[289][15] * ddt_scale));
        let eq180_e2268_d_b16: f64 = (p.p7 * (s.db[289][16] * ddt_scale));
        let eq180_e2268_d_b17: f64 = (p.p7 * (s.db[289][17] * ddt_scale));
        let eq180_e2268_d_b18: f64 = (p.p7 * (s.db[289][18] * ddt_scale));
        let eq180_e2268_d_b19: f64 = (p.p7 * (s.db[289][19] * ddt_scale));
        let eq180_e2268_d_b20: f64 = (p.p7 * (s.db[289][20] * ddt_scale));
        let eq180_e2268_d_b21: f64 = (p.p7 * (s.db[289][21] * ddt_scale));
        let eq180_e2268_d_b22: f64 = (p.p7 * (s.db[289][22] * ddt_scale));
        let eq180_e2268_d_b23: f64 = (p.p7 * (s.db[289][23] * ddt_scale));
        let eq180_e2268_d_b24: f64 = (p.p7 * (s.db[289][24] * ddt_scale));
        let eq180_e2268_d_b25: f64 = (p.p7 * (s.db[289][25] * ddt_scale));
        let eq180_e2268_d_b26: f64 = (p.p7 * (s.db[289][26] * ddt_scale));
        let eq180_e2268_d_b27: f64 = (p.p7 * (s.db[289][27] * ddt_scale));
        let eq180_e2268_d_b28: f64 = (p.p7 * (s.db[289][28] * ddt_scale));
        let eq180_e2268_d_b29: f64 = (p.p7 * (s.db[289][29] * ddt_scale));
        let eq180_e2268_d_b30: f64 = (p.p7 * (s.db[289][30] * ddt_scale));
        let eq180_e2268_d_b31: f64 = (p.p7 * (s.db[289][31] * ddt_scale));
        let eq180_e2268_d_b32: f64 = (p.p7 * (s.db[289][32] * ddt_scale));
        let eq180_e2268_d_b33: f64 = (p.p7 * (s.db[289][33] * ddt_scale));
        let eq180_e2268_d_b34: f64 = (p.p7 * (s.db[289][34] * ddt_scale));
        let eq180_e2268_d_b35: f64 = (p.p7 * (s.db[289][35] * ddt_scale));
        let eq180_e2268_d_b36: f64 = (p.p7 * (s.db[289][36] * ddt_scale));
        let eq180_e2268_d_b37: f64 = (p.p7 * (s.db[289][37] * ddt_scale));
        let eq180_e2268_d_b38: f64 = (p.p7 * (s.db[289][38] * ddt_scale));
        let eq180_e2268_d_b39: f64 = (p.p7 * (s.db[289][39] * ddt_scale));
        let eq180_e2268_d_b40: f64 = (p.p7 * (s.db[289][40] * ddt_scale));
        let eq180_e2268_d_b41: f64 = (p.p7 * (s.db[289][41] * ddt_scale));
        let eq180_e2268_d_b42: f64 = (p.p7 * (s.db[289][42] * ddt_scale));
        let eq180_e2268_d_b43: f64 = (p.p7 * (s.db[289][43] * ddt_scale));
        let eq180_e2268_d_b44: f64 = (p.p7 * (s.db[289][44] * ddt_scale));
        let eq180_e2268_d_b45: f64 = (p.p7 * (s.db[289][45] * ddt_scale));
        let eq180_e2268_d_b46: f64 = (p.p7 * (s.db[289][46] * ddt_scale));
        let eq180_e2268_d_b47: f64 = (p.p7 * (s.db[289][47] * ddt_scale));
        let eq180_e2268_d_b48: f64 = (p.p7 * (s.db[289][48] * ddt_scale));
        let eq180_e2268_d_b49: f64 = (p.p7 * (s.db[289][49] * ddt_scale));
        let eq180_e2268_d_b50: f64 = (p.p7 * (s.db[289][50] * ddt_scale));
        let eq180_e2268_d_b51: f64 = (p.p7 * (s.db[289][51] * ddt_scale));
        let eq180_e2268_d_b52: f64 = (p.p7 * (s.db[289][52] * ddt_scale));
        let eq180_e2268_d_b53: f64 = (p.p7 * (s.db[289][53] * ddt_scale));
        let eq180_e2268_d_b54: f64 = (p.p7 * (s.db[289][54] * ddt_scale));
        (eq180_e2268, eq180_e2268_d_n0, eq180_e2268_d_n1, eq180_e2268_d_n2, eq180_e2268_d_n3, eq180_e2268_d_n4, eq180_e2268_d_n5, eq180_e2268_d_n6, eq180_e2268_d_n7, eq180_e2268_d_n8, eq180_e2268_d_n9, eq180_e2268_d_n10, eq180_e2268_d_n11, eq180_e2268_d_n12, eq180_e2268_d_n13, eq180_e2268_d_n14, eq180_e2268_d_n15, eq180_e2268_d_n16, eq180_e2268_d_n17, eq180_e2268_d_n18, eq180_e2268_d_n19, eq180_e2268_d_n20, eq180_e2268_d_n21, eq180_e2268_d_n22, eq180_e2268_d_b0, eq180_e2268_d_b1, eq180_e2268_d_b2, eq180_e2268_d_b3, eq180_e2268_d_b4, eq180_e2268_d_b5, eq180_e2268_d_b6, eq180_e2268_d_b7, eq180_e2268_d_b8, eq180_e2268_d_b9, eq180_e2268_d_b10, eq180_e2268_d_b11, eq180_e2268_d_b12, eq180_e2268_d_b13, eq180_e2268_d_b14, eq180_e2268_d_b15, eq180_e2268_d_b16, eq180_e2268_d_b17, eq180_e2268_d_b18, eq180_e2268_d_b19, eq180_e2268_d_b20, eq180_e2268_d_b21, eq180_e2268_d_b22, eq180_e2268_d_b23, eq180_e2268_d_b24, eq180_e2268_d_b25, eq180_e2268_d_b26, eq180_e2268_d_b27, eq180_e2268_d_b28, eq180_e2268_d_b29, eq180_e2268_d_b30, eq180_e2268_d_b31, eq180_e2268_d_b32, eq180_e2268_d_b33, eq180_e2268_d_b34, eq180_e2268_d_b35, eq180_e2268_d_b36, eq180_e2268_d_b37, eq180_e2268_d_b38, eq180_e2268_d_b39, eq180_e2268_d_b40, eq180_e2268_d_b41, eq180_e2268_d_b42, eq180_e2268_d_b43, eq180_e2268_d_b44, eq180_e2268_d_b45, eq180_e2268_d_b46, eq180_e2268_d_b47, eq180_e2268_d_b48, eq180_e2268_d_b49, eq180_e2268_d_b50, eq180_e2268_d_b51, eq180_e2268_d_b52, eq180_e2268_d_b53, eq180_e2268_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq180_value: f64 = eq180_e2270;
        let eq180_node_derivatives: [f64; 23] = [eq180_e2270_d_n0, eq180_e2270_d_n1, eq180_e2270_d_n2, eq180_e2270_d_n3, eq180_e2270_d_n4, eq180_e2270_d_n5, eq180_e2270_d_n6, eq180_e2270_d_n7, eq180_e2270_d_n8, eq180_e2270_d_n9, eq180_e2270_d_n10, eq180_e2270_d_n11, eq180_e2270_d_n12, eq180_e2270_d_n13, eq180_e2270_d_n14, eq180_e2270_d_n15, eq180_e2270_d_n16, eq180_e2270_d_n17, eq180_e2270_d_n18, eq180_e2270_d_n19, eq180_e2270_d_n20, eq180_e2270_d_n21, eq180_e2270_d_n22];
        let eq180_branch_derivatives: [f64; 55] = [eq180_e2270_d_b0, eq180_e2270_d_b1, eq180_e2270_d_b2, eq180_e2270_d_b3, eq180_e2270_d_b4, eq180_e2270_d_b5, eq180_e2270_d_b6, eq180_e2270_d_b7, eq180_e2270_d_b8, eq180_e2270_d_b9, eq180_e2270_d_b10, eq180_e2270_d_b11, eq180_e2270_d_b12, eq180_e2270_d_b13, eq180_e2270_d_b14, eq180_e2270_d_b15, eq180_e2270_d_b16, eq180_e2270_d_b17, eq180_e2270_d_b18, eq180_e2270_d_b19, eq180_e2270_d_b20, eq180_e2270_d_b21, eq180_e2270_d_b22, eq180_e2270_d_b23, eq180_e2270_d_b24, eq180_e2270_d_b25, eq180_e2270_d_b26, eq180_e2270_d_b27, eq180_e2270_d_b28, eq180_e2270_d_b29, eq180_e2270_d_b30, eq180_e2270_d_b31, eq180_e2270_d_b32, eq180_e2270_d_b33, eq180_e2270_d_b34, eq180_e2270_d_b35, eq180_e2270_d_b36, eq180_e2270_d_b37, eq180_e2270_d_b38, eq180_e2270_d_b39, eq180_e2270_d_b40, eq180_e2270_d_b41, eq180_e2270_d_b42, eq180_e2270_d_b43, eq180_e2270_d_b44, eq180_e2270_d_b45, eq180_e2270_d_b46, eq180_e2270_d_b47, eq180_e2270_d_b48, eq180_e2270_d_b49, eq180_e2270_d_b50, eq180_e2270_d_b51, eq180_e2270_d_b52, eq180_e2270_d_b53, eq180_e2270_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(21),
            multiplicity * (eq180_value),
            &eq180_node_derivatives,
            &eq180_branch_derivatives,
            multiplicity,
        );
    }
}
