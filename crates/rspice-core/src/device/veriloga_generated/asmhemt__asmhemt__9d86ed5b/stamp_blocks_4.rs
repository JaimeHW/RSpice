#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_en1: f64,
        var_en1_db0: f64,
        var_en1_db1: f64,
        var_en1_db10: f64,
        var_en1_db11: f64,
        var_en1_db12: f64,
        var_en1_db13: f64,
        var_en1_db14: f64,
        var_en1_db15: f64,
        var_en1_db16: f64,
        var_en1_db17: f64,
        var_en1_db18: f64,
        var_en1_db19: f64,
        var_en1_db2: f64,
        var_en1_db20: f64,
        var_en1_db21: f64,
        var_en1_db22: f64,
        var_en1_db23: f64,
        var_en1_db24: f64,
        var_en1_db25: f64,
        var_en1_db26: f64,
        var_en1_db27: f64,
        var_en1_db28: f64,
        var_en1_db29: f64,
        var_en1_db3: f64,
        var_en1_db30: f64,
        var_en1_db31: f64,
        var_en1_db32: f64,
        var_en1_db33: f64,
        var_en1_db34: f64,
        var_en1_db35: f64,
        var_en1_db36: f64,
        var_en1_db37: f64,
        var_en1_db38: f64,
        var_en1_db39: f64,
        var_en1_db4: f64,
        var_en1_db40: f64,
        var_en1_db41: f64,
        var_en1_db42: f64,
        var_en1_db43: f64,
        var_en1_db44: f64,
        var_en1_db45: f64,
        var_en1_db46: f64,
        var_en1_db47: f64,
        var_en1_db48: f64,
        var_en1_db49: f64,
        var_en1_db5: f64,
        var_en1_db50: f64,
        var_en1_db51: f64,
        var_en1_db52: f64,
        var_en1_db53: f64,
        var_en1_db54: f64,
        var_en1_db6: f64,
        var_en1_db7: f64,
        var_en1_db8: f64,
        var_en1_db9: f64,
        var_en1_dn0: f64,
        var_en1_dn1: f64,
        var_en1_dn10: f64,
        var_en1_dn11: f64,
        var_en1_dn12: f64,
        var_en1_dn13: f64,
        var_en1_dn14: f64,
        var_en1_dn15: f64,
        var_en1_dn16: f64,
        var_en1_dn17: f64,
        var_en1_dn18: f64,
        var_en1_dn19: f64,
        var_en1_dn2: f64,
        var_en1_dn20: f64,
        var_en1_dn21: f64,
        var_en1_dn22: f64,
        var_en1_dn3: f64,
        var_en1_dn4: f64,
        var_en1_dn5: f64,
        var_en1_dn6: f64,
        var_en1_dn7: f64,
        var_en1_dn8: f64,
        var_en1_dn9: f64,
        var_guard353: f64,
        var_guard354: f64,
        var_guard355: f64,
        var_guard356: f64,
        var_guard357: f64,
        var_guard358: f64,
        var_phiyn: f64,
        var_phiyn_db0: f64,
        var_phiyn_db1: f64,
        var_phiyn_db10: f64,
        var_phiyn_db11: f64,
        var_phiyn_db12: f64,
        var_phiyn_db13: f64,
        var_phiyn_db14: f64,
        var_phiyn_db15: f64,
        var_phiyn_db16: f64,
        var_phiyn_db17: f64,
        var_phiyn_db18: f64,
        var_phiyn_db19: f64,
        var_phiyn_db2: f64,
        var_phiyn_db20: f64,
        var_phiyn_db21: f64,
        var_phiyn_db22: f64,
        var_phiyn_db23: f64,
        var_phiyn_db24: f64,
        var_phiyn_db25: f64,
        var_phiyn_db26: f64,
        var_phiyn_db27: f64,
        var_phiyn_db28: f64,
        var_phiyn_db29: f64,
        var_phiyn_db3: f64,
        var_phiyn_db30: f64,
        var_phiyn_db31: f64,
        var_phiyn_db32: f64,
        var_phiyn_db33: f64,
        var_phiyn_db34: f64,
        var_phiyn_db35: f64,
        var_phiyn_db36: f64,
        var_phiyn_db37: f64,
        var_phiyn_db38: f64,
        var_phiyn_db39: f64,
        var_phiyn_db4: f64,
        var_phiyn_db40: f64,
        var_phiyn_db41: f64,
        var_phiyn_db42: f64,
        var_phiyn_db43: f64,
        var_phiyn_db44: f64,
        var_phiyn_db45: f64,
        var_phiyn_db46: f64,
        var_phiyn_db47: f64,
        var_phiyn_db48: f64,
        var_phiyn_db49: f64,
        var_phiyn_db5: f64,
        var_phiyn_db50: f64,
        var_phiyn_db51: f64,
        var_phiyn_db52: f64,
        var_phiyn_db53: f64,
        var_phiyn_db54: f64,
        var_phiyn_db6: f64,
        var_phiyn_db7: f64,
        var_phiyn_db8: f64,
        var_phiyn_db9: f64,
        var_phiyn_dn0: f64,
        var_phiyn_dn1: f64,
        var_phiyn_dn10: f64,
        var_phiyn_dn11: f64,
        var_phiyn_dn12: f64,
        var_phiyn_dn13: f64,
        var_phiyn_dn14: f64,
        var_phiyn_dn15: f64,
        var_phiyn_dn16: f64,
        var_phiyn_dn17: f64,
        var_phiyn_dn18: f64,
        var_phiyn_dn19: f64,
        var_phiyn_dn2: f64,
        var_phiyn_dn20: f64,
        var_phiyn_dn21: f64,
        var_phiyn_dn22: f64,
        var_phiyn_dn3: f64,
        var_phiyn_dn4: f64,
        var_phiyn_dn5: f64,
        var_phiyn_dn6: f64,
        var_phiyn_dn7: f64,
        var_phiyn_dn8: f64,
        var_phiyn_dn9: f64,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq44_e815, eq44_e815_d_n0, eq44_e815_d_n1, eq44_e815_d_n2, eq44_e815_d_n3, eq44_e815_d_n4, eq44_e815_d_n5, eq44_e815_d_n6, eq44_e815_d_n7, eq44_e815_d_n8, eq44_e815_d_n9, eq44_e815_d_n10, eq44_e815_d_n11, eq44_e815_d_n12, eq44_e815_d_n13, eq44_e815_d_n14, eq44_e815_d_n15, eq44_e815_d_n16, eq44_e815_d_n17, eq44_e815_d_n18, eq44_e815_d_n19, eq44_e815_d_n20, eq44_e815_d_n21, eq44_e815_d_n22, eq44_e815_d_b0, eq44_e815_d_b1, eq44_e815_d_b2, eq44_e815_d_b3, eq44_e815_d_b4, eq44_e815_d_b5, eq44_e815_d_b6, eq44_e815_d_b7, eq44_e815_d_b8, eq44_e815_d_b9, eq44_e815_d_b10, eq44_e815_d_b11, eq44_e815_d_b12, eq44_e815_d_b13, eq44_e815_d_b14, eq44_e815_d_b15, eq44_e815_d_b16, eq44_e815_d_b17, eq44_e815_d_b18, eq44_e815_d_b19, eq44_e815_d_b20, eq44_e815_d_b21, eq44_e815_d_b22, eq44_e815_d_b23, eq44_e815_d_b24, eq44_e815_d_b25, eq44_e815_d_b26, eq44_e815_d_b27, eq44_e815_d_b28, eq44_e815_d_b29, eq44_e815_d_b30, eq44_e815_d_b31, eq44_e815_d_b32, eq44_e815_d_b33, eq44_e815_d_b34, eq44_e815_d_b35, eq44_e815_d_b36, eq44_e815_d_b37, eq44_e815_d_b38, eq44_e815_d_b39, eq44_e815_d_b40, eq44_e815_d_b41, eq44_e815_d_b42, eq44_e815_d_b43, eq44_e815_d_b44, eq44_e815_d_b45, eq44_e815_d_b46, eq44_e815_d_b47, eq44_e815_d_b48, eq44_e815_d_b49, eq44_e815_d_b50, eq44_e815_d_b51, eq44_e815_d_b52, eq44_e815_d_b53, eq44_e815_d_b54,) = {
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq44_e798: f64 = (-p.p144);
        let eq44_e800: f64 = (eq44_e798 * var_en1);
        let eq44_e800_d_n0: f64 = (eq44_e798 * var_en1_dn0);
        let eq44_e800_d_n1: f64 = (eq44_e798 * var_en1_dn1);
        let eq44_e800_d_n2: f64 = (eq44_e798 * var_en1_dn2);
        let eq44_e800_d_n3: f64 = (eq44_e798 * var_en1_dn3);
        let eq44_e800_d_n4: f64 = (eq44_e798 * var_en1_dn4);
        let eq44_e800_d_n5: f64 = (eq44_e798 * var_en1_dn5);
        let eq44_e800_d_n6: f64 = (eq44_e798 * var_en1_dn6);
        let eq44_e800_d_n7: f64 = (eq44_e798 * var_en1_dn7);
        let eq44_e800_d_n8: f64 = (eq44_e798 * var_en1_dn8);
        let eq44_e800_d_n9: f64 = (eq44_e798 * var_en1_dn9);
        let eq44_e800_d_n10: f64 = (eq44_e798 * var_en1_dn10);
        let eq44_e800_d_n11: f64 = (eq44_e798 * var_en1_dn11);
        let eq44_e800_d_n12: f64 = (eq44_e798 * var_en1_dn12);
        let eq44_e800_d_n13: f64 = (eq44_e798 * var_en1_dn13);
        let eq44_e800_d_n14: f64 = (eq44_e798 * var_en1_dn14);
        let eq44_e800_d_n15: f64 = (eq44_e798 * var_en1_dn15);
        let eq44_e800_d_n16: f64 = (eq44_e798 * var_en1_dn16);
        let eq44_e800_d_n17: f64 = (eq44_e798 * var_en1_dn17);
        let eq44_e800_d_n18: f64 = (eq44_e798 * var_en1_dn18);
        let eq44_e800_d_n19: f64 = (eq44_e798 * var_en1_dn19);
        let eq44_e800_d_n20: f64 = (eq44_e798 * var_en1_dn20);
        let eq44_e800_d_n21: f64 = (eq44_e798 * var_en1_dn21);
        let eq44_e800_d_n22: f64 = (eq44_e798 * var_en1_dn22);
        let eq44_e800_d_b0: f64 = (eq44_e798 * var_en1_db0);
        let eq44_e800_d_b1: f64 = (eq44_e798 * var_en1_db1);
        let eq44_e800_d_b2: f64 = (eq44_e798 * var_en1_db2);
        let eq44_e800_d_b3: f64 = (eq44_e798 * var_en1_db3);
        let eq44_e800_d_b4: f64 = (eq44_e798 * var_en1_db4);
        let eq44_e800_d_b5: f64 = (eq44_e798 * var_en1_db5);
        let eq44_e800_d_b6: f64 = (eq44_e798 * var_en1_db6);
        let eq44_e800_d_b7: f64 = (eq44_e798 * var_en1_db7);
        let eq44_e800_d_b8: f64 = (eq44_e798 * var_en1_db8);
        let eq44_e800_d_b9: f64 = (eq44_e798 * var_en1_db9);
        let eq44_e800_d_b10: f64 = (eq44_e798 * var_en1_db10);
        let eq44_e800_d_b11: f64 = (eq44_e798 * var_en1_db11);
        let eq44_e800_d_b12: f64 = (eq44_e798 * var_en1_db12);
        let eq44_e800_d_b13: f64 = (eq44_e798 * var_en1_db13);
        let eq44_e800_d_b14: f64 = (eq44_e798 * var_en1_db14);
        let eq44_e800_d_b15: f64 = (eq44_e798 * var_en1_db15);
        let eq44_e800_d_b16: f64 = (eq44_e798 * var_en1_db16);
        let eq44_e800_d_b17: f64 = (eq44_e798 * var_en1_db17);
        let eq44_e800_d_b18: f64 = (eq44_e798 * var_en1_db18);
        let eq44_e800_d_b19: f64 = (eq44_e798 * var_en1_db19);
        let eq44_e800_d_b20: f64 = (eq44_e798 * var_en1_db20);
        let eq44_e800_d_b21: f64 = (eq44_e798 * var_en1_db21);
        let eq44_e800_d_b22: f64 = (eq44_e798 * var_en1_db22);
        let eq44_e800_d_b23: f64 = (eq44_e798 * var_en1_db23);
        let eq44_e800_d_b24: f64 = (eq44_e798 * var_en1_db24);
        let eq44_e800_d_b25: f64 = (eq44_e798 * var_en1_db25);
        let eq44_e800_d_b26: f64 = (eq44_e798 * var_en1_db26);
        let eq44_e800_d_b27: f64 = (eq44_e798 * var_en1_db27);
        let eq44_e800_d_b28: f64 = (eq44_e798 * var_en1_db28);
        let eq44_e800_d_b29: f64 = (eq44_e798 * var_en1_db29);
        let eq44_e800_d_b30: f64 = (eq44_e798 * var_en1_db30);
        let eq44_e800_d_b31: f64 = (eq44_e798 * var_en1_db31);
        let eq44_e800_d_b32: f64 = (eq44_e798 * var_en1_db32);
        let eq44_e800_d_b33: f64 = (eq44_e798 * var_en1_db33);
        let eq44_e800_d_b34: f64 = (eq44_e798 * var_en1_db34);
        let eq44_e800_d_b35: f64 = (eq44_e798 * var_en1_db35);
        let eq44_e800_d_b36: f64 = (eq44_e798 * var_en1_db36);
        let eq44_e800_d_b37: f64 = (eq44_e798 * var_en1_db37);
        let eq44_e800_d_b38: f64 = (eq44_e798 * var_en1_db38);
        let eq44_e800_d_b39: f64 = (eq44_e798 * var_en1_db39);
        let eq44_e800_d_b40: f64 = (eq44_e798 * var_en1_db40);
        let eq44_e800_d_b41: f64 = (eq44_e798 * var_en1_db41);
        let eq44_e800_d_b42: f64 = (eq44_e798 * var_en1_db42);
        let eq44_e800_d_b43: f64 = (eq44_e798 * var_en1_db43);
        let eq44_e800_d_b44: f64 = (eq44_e798 * var_en1_db44);
        let eq44_e800_d_b45: f64 = (eq44_e798 * var_en1_db45);
        let eq44_e800_d_b46: f64 = (eq44_e798 * var_en1_db46);
        let eq44_e800_d_b47: f64 = (eq44_e798 * var_en1_db47);
        let eq44_e800_d_b48: f64 = (eq44_e798 * var_en1_db48);
        let eq44_e800_d_b49: f64 = (eq44_e798 * var_en1_db49);
        let eq44_e800_d_b50: f64 = (eq44_e798 * var_en1_db50);
        let eq44_e800_d_b51: f64 = (eq44_e798 * var_en1_db51);
        let eq44_e800_d_b52: f64 = (eq44_e798 * var_en1_db52);
        let eq44_e800_d_b53: f64 = (eq44_e798 * var_en1_db53);
        let eq44_e800_d_b54: f64 = (eq44_e798 * var_en1_db54);
        let eq44_e803: f64 = (p.p145 - (nv6 - 0.0));
        let eq44_e804: f64 = (eq44_e800 * eq44_e803);
        let eq44_e804_d_n0: f64 = (eq44_e800_d_n0 * eq44_e803);
        let eq44_e804_d_n1: f64 = (eq44_e800_d_n1 * eq44_e803);
        let eq44_e804_d_n2: f64 = (eq44_e800_d_n2 * eq44_e803);
        let eq44_e804_d_n3: f64 = (eq44_e800_d_n3 * eq44_e803);
        let eq44_e804_d_n4: f64 = (eq44_e800_d_n4 * eq44_e803);
        let eq44_e804_d_n5: f64 = (eq44_e800_d_n5 * eq44_e803);
        let eq44_e804_d_n6: f64 = ((eq44_e800_d_n6 * eq44_e803) + (eq44_e800 * (-1.0)));
        let eq44_e804_d_n7: f64 = (eq44_e800_d_n7 * eq44_e803);
        let eq44_e804_d_n8: f64 = (eq44_e800_d_n8 * eq44_e803);
        let eq44_e804_d_n9: f64 = (eq44_e800_d_n9 * eq44_e803);
        let eq44_e804_d_n10: f64 = (eq44_e800_d_n10 * eq44_e803);
        let eq44_e804_d_n11: f64 = (eq44_e800_d_n11 * eq44_e803);
        let eq44_e804_d_n12: f64 = (eq44_e800_d_n12 * eq44_e803);
        let eq44_e804_d_n13: f64 = (eq44_e800_d_n13 * eq44_e803);
        let eq44_e804_d_n14: f64 = (eq44_e800_d_n14 * eq44_e803);
        let eq44_e804_d_n15: f64 = (eq44_e800_d_n15 * eq44_e803);
        let eq44_e804_d_n16: f64 = (eq44_e800_d_n16 * eq44_e803);
        let eq44_e804_d_n17: f64 = (eq44_e800_d_n17 * eq44_e803);
        let eq44_e804_d_n18: f64 = (eq44_e800_d_n18 * eq44_e803);
        let eq44_e804_d_n19: f64 = (eq44_e800_d_n19 * eq44_e803);
        let eq44_e804_d_n20: f64 = (eq44_e800_d_n20 * eq44_e803);
        let eq44_e804_d_n21: f64 = (eq44_e800_d_n21 * eq44_e803);
        let eq44_e804_d_n22: f64 = (eq44_e800_d_n22 * eq44_e803);
        let eq44_e804_d_b0: f64 = (eq44_e800_d_b0 * eq44_e803);
        let eq44_e804_d_b1: f64 = (eq44_e800_d_b1 * eq44_e803);
        let eq44_e804_d_b2: f64 = (eq44_e800_d_b2 * eq44_e803);
        let eq44_e804_d_b3: f64 = (eq44_e800_d_b3 * eq44_e803);
        let eq44_e804_d_b4: f64 = (eq44_e800_d_b4 * eq44_e803);
        let eq44_e804_d_b5: f64 = (eq44_e800_d_b5 * eq44_e803);
        let eq44_e804_d_b6: f64 = (eq44_e800_d_b6 * eq44_e803);
        let eq44_e804_d_b7: f64 = (eq44_e800_d_b7 * eq44_e803);
        let eq44_e804_d_b8: f64 = (eq44_e800_d_b8 * eq44_e803);
        let eq44_e804_d_b9: f64 = (eq44_e800_d_b9 * eq44_e803);
        let eq44_e804_d_b10: f64 = (eq44_e800_d_b10 * eq44_e803);
        let eq44_e804_d_b11: f64 = (eq44_e800_d_b11 * eq44_e803);
        let eq44_e804_d_b12: f64 = (eq44_e800_d_b12 * eq44_e803);
        let eq44_e804_d_b13: f64 = (eq44_e800_d_b13 * eq44_e803);
        let eq44_e804_d_b14: f64 = (eq44_e800_d_b14 * eq44_e803);
        let eq44_e804_d_b15: f64 = (eq44_e800_d_b15 * eq44_e803);
        let eq44_e804_d_b16: f64 = (eq44_e800_d_b16 * eq44_e803);
        let eq44_e804_d_b17: f64 = (eq44_e800_d_b17 * eq44_e803);
        let eq44_e804_d_b18: f64 = (eq44_e800_d_b18 * eq44_e803);
        let eq44_e804_d_b19: f64 = (eq44_e800_d_b19 * eq44_e803);
        let eq44_e804_d_b20: f64 = (eq44_e800_d_b20 * eq44_e803);
        let eq44_e804_d_b21: f64 = (eq44_e800_d_b21 * eq44_e803);
        let eq44_e804_d_b22: f64 = (eq44_e800_d_b22 * eq44_e803);
        let eq44_e804_d_b23: f64 = (eq44_e800_d_b23 * eq44_e803);
        let eq44_e804_d_b24: f64 = (eq44_e800_d_b24 * eq44_e803);
        let eq44_e804_d_b25: f64 = (eq44_e800_d_b25 * eq44_e803);
        let eq44_e804_d_b26: f64 = (eq44_e800_d_b26 * eq44_e803);
        let eq44_e804_d_b27: f64 = (eq44_e800_d_b27 * eq44_e803);
        let eq44_e804_d_b28: f64 = (eq44_e800_d_b28 * eq44_e803);
        let eq44_e804_d_b29: f64 = (eq44_e800_d_b29 * eq44_e803);
        let eq44_e804_d_b30: f64 = (eq44_e800_d_b30 * eq44_e803);
        let eq44_e804_d_b31: f64 = (eq44_e800_d_b31 * eq44_e803);
        let eq44_e804_d_b32: f64 = (eq44_e800_d_b32 * eq44_e803);
        let eq44_e804_d_b33: f64 = (eq44_e800_d_b33 * eq44_e803);
        let eq44_e804_d_b34: f64 = (eq44_e800_d_b34 * eq44_e803);
        let eq44_e804_d_b35: f64 = (eq44_e800_d_b35 * eq44_e803);
        let eq44_e804_d_b36: f64 = (eq44_e800_d_b36 * eq44_e803);
        let eq44_e804_d_b37: f64 = (eq44_e800_d_b37 * eq44_e803);
        let eq44_e804_d_b38: f64 = (eq44_e800_d_b38 * eq44_e803);
        let eq44_e804_d_b39: f64 = (eq44_e800_d_b39 * eq44_e803);
        let eq44_e804_d_b40: f64 = (eq44_e800_d_b40 * eq44_e803);
        let eq44_e804_d_b41: f64 = (eq44_e800_d_b41 * eq44_e803);
        let eq44_e804_d_b42: f64 = (eq44_e800_d_b42 * eq44_e803);
        let eq44_e804_d_b43: f64 = (eq44_e800_d_b43 * eq44_e803);
        let eq44_e804_d_b44: f64 = (eq44_e800_d_b44 * eq44_e803);
        let eq44_e804_d_b45: f64 = (eq44_e800_d_b45 * eq44_e803);
        let eq44_e804_d_b46: f64 = (eq44_e800_d_b46 * eq44_e803);
        let eq44_e804_d_b47: f64 = (eq44_e800_d_b47 * eq44_e803);
        let eq44_e804_d_b48: f64 = (eq44_e800_d_b48 * eq44_e803);
        let eq44_e804_d_b49: f64 = (eq44_e800_d_b49 * eq44_e803);
        let eq44_e804_d_b50: f64 = (eq44_e800_d_b50 * eq44_e803);
        let eq44_e804_d_b51: f64 = (eq44_e800_d_b51 * eq44_e803);
        let eq44_e804_d_b52: f64 = (eq44_e800_d_b52 * eq44_e803);
        let eq44_e804_d_b53: f64 = (eq44_e800_d_b53 * eq44_e803);
        let eq44_e804_d_b54: f64 = (eq44_e800_d_b54 * eq44_e803);
        let eq44_e807: f64 = (2.0 * var_phiyn);
        let eq44_e807_d_n0: f64 = (2.0 * var_phiyn_dn0);
        let eq44_e807_d_n1: f64 = (2.0 * var_phiyn_dn1);
        let eq44_e807_d_n2: f64 = (2.0 * var_phiyn_dn2);
        let eq44_e807_d_n3: f64 = (2.0 * var_phiyn_dn3);
        let eq44_e807_d_n4: f64 = (2.0 * var_phiyn_dn4);
        let eq44_e807_d_n5: f64 = (2.0 * var_phiyn_dn5);
        let eq44_e807_d_n6: f64 = (2.0 * var_phiyn_dn6);
        let eq44_e807_d_n7: f64 = (2.0 * var_phiyn_dn7);
        let eq44_e807_d_n8: f64 = (2.0 * var_phiyn_dn8);
        let eq44_e807_d_n9: f64 = (2.0 * var_phiyn_dn9);
        let eq44_e807_d_n10: f64 = (2.0 * var_phiyn_dn10);
        let eq44_e807_d_n11: f64 = (2.0 * var_phiyn_dn11);
        let eq44_e807_d_n12: f64 = (2.0 * var_phiyn_dn12);
        let eq44_e807_d_n13: f64 = (2.0 * var_phiyn_dn13);
        let eq44_e807_d_n14: f64 = (2.0 * var_phiyn_dn14);
        let eq44_e807_d_n15: f64 = (2.0 * var_phiyn_dn15);
        let eq44_e807_d_n16: f64 = (2.0 * var_phiyn_dn16);
        let eq44_e807_d_n17: f64 = (2.0 * var_phiyn_dn17);
        let eq44_e807_d_n18: f64 = (2.0 * var_phiyn_dn18);
        let eq44_e807_d_n19: f64 = (2.0 * var_phiyn_dn19);
        let eq44_e807_d_n20: f64 = (2.0 * var_phiyn_dn20);
        let eq44_e807_d_n21: f64 = (2.0 * var_phiyn_dn21);
        let eq44_e807_d_n22: f64 = (2.0 * var_phiyn_dn22);
        let eq44_e807_d_b0: f64 = (2.0 * var_phiyn_db0);
        let eq44_e807_d_b1: f64 = (2.0 * var_phiyn_db1);
        let eq44_e807_d_b2: f64 = (2.0 * var_phiyn_db2);
        let eq44_e807_d_b3: f64 = (2.0 * var_phiyn_db3);
        let eq44_e807_d_b4: f64 = (2.0 * var_phiyn_db4);
        let eq44_e807_d_b5: f64 = (2.0 * var_phiyn_db5);
        let eq44_e807_d_b6: f64 = (2.0 * var_phiyn_db6);
        let eq44_e807_d_b7: f64 = (2.0 * var_phiyn_db7);
        let eq44_e807_d_b8: f64 = (2.0 * var_phiyn_db8);
        let eq44_e807_d_b9: f64 = (2.0 * var_phiyn_db9);
        let eq44_e807_d_b10: f64 = (2.0 * var_phiyn_db10);
        let eq44_e807_d_b11: f64 = (2.0 * var_phiyn_db11);
        let eq44_e807_d_b12: f64 = (2.0 * var_phiyn_db12);
        let eq44_e807_d_b13: f64 = (2.0 * var_phiyn_db13);
        let eq44_e807_d_b14: f64 = (2.0 * var_phiyn_db14);
        let eq44_e807_d_b15: f64 = (2.0 * var_phiyn_db15);
        let eq44_e807_d_b16: f64 = (2.0 * var_phiyn_db16);
        let eq44_e807_d_b17: f64 = (2.0 * var_phiyn_db17);
        let eq44_e807_d_b18: f64 = (2.0 * var_phiyn_db18);
        let eq44_e807_d_b19: f64 = (2.0 * var_phiyn_db19);
        let eq44_e807_d_b20: f64 = (2.0 * var_phiyn_db20);
        let eq44_e807_d_b21: f64 = (2.0 * var_phiyn_db21);
        let eq44_e807_d_b22: f64 = (2.0 * var_phiyn_db22);
        let eq44_e807_d_b23: f64 = (2.0 * var_phiyn_db23);
        let eq44_e807_d_b24: f64 = (2.0 * var_phiyn_db24);
        let eq44_e807_d_b25: f64 = (2.0 * var_phiyn_db25);
        let eq44_e807_d_b26: f64 = (2.0 * var_phiyn_db26);
        let eq44_e807_d_b27: f64 = (2.0 * var_phiyn_db27);
        let eq44_e807_d_b28: f64 = (2.0 * var_phiyn_db28);
        let eq44_e807_d_b29: f64 = (2.0 * var_phiyn_db29);
        let eq44_e807_d_b30: f64 = (2.0 * var_phiyn_db30);
        let eq44_e807_d_b31: f64 = (2.0 * var_phiyn_db31);
        let eq44_e807_d_b32: f64 = (2.0 * var_phiyn_db32);
        let eq44_e807_d_b33: f64 = (2.0 * var_phiyn_db33);
        let eq44_e807_d_b34: f64 = (2.0 * var_phiyn_db34);
        let eq44_e807_d_b35: f64 = (2.0 * var_phiyn_db35);
        let eq44_e807_d_b36: f64 = (2.0 * var_phiyn_db36);
        let eq44_e807_d_b37: f64 = (2.0 * var_phiyn_db37);
        let eq44_e807_d_b38: f64 = (2.0 * var_phiyn_db38);
        let eq44_e807_d_b39: f64 = (2.0 * var_phiyn_db39);
        let eq44_e807_d_b40: f64 = (2.0 * var_phiyn_db40);
        let eq44_e807_d_b41: f64 = (2.0 * var_phiyn_db41);
        let eq44_e807_d_b42: f64 = (2.0 * var_phiyn_db42);
        let eq44_e807_d_b43: f64 = (2.0 * var_phiyn_db43);
        let eq44_e807_d_b44: f64 = (2.0 * var_phiyn_db44);
        let eq44_e807_d_b45: f64 = (2.0 * var_phiyn_db45);
        let eq44_e807_d_b46: f64 = (2.0 * var_phiyn_db46);
        let eq44_e807_d_b47: f64 = (2.0 * var_phiyn_db47);
        let eq44_e807_d_b48: f64 = (2.0 * var_phiyn_db48);
        let eq44_e807_d_b49: f64 = (2.0 * var_phiyn_db49);
        let eq44_e807_d_b50: f64 = (2.0 * var_phiyn_db50);
        let eq44_e807_d_b51: f64 = (2.0 * var_phiyn_db51);
        let eq44_e807_d_b52: f64 = (2.0 * var_phiyn_db52);
        let eq44_e807_d_b53: f64 = (2.0 * var_phiyn_db53);
        let eq44_e807_d_b54: f64 = (2.0 * var_phiyn_db54);
        let eq44_e808: f64 = (eq44_e807).exp();
        let eq44_e808_d_n0: f64 = (eq44_e808 * eq44_e807_d_n0);
        let eq44_e808_d_n1: f64 = (eq44_e808 * eq44_e807_d_n1);
        let eq44_e808_d_n2: f64 = (eq44_e808 * eq44_e807_d_n2);
        let eq44_e808_d_n3: f64 = (eq44_e808 * eq44_e807_d_n3);
        let eq44_e808_d_n4: f64 = (eq44_e808 * eq44_e807_d_n4);
        let eq44_e808_d_n5: f64 = (eq44_e808 * eq44_e807_d_n5);
        let eq44_e808_d_n6: f64 = (eq44_e808 * eq44_e807_d_n6);
        let eq44_e808_d_n7: f64 = (eq44_e808 * eq44_e807_d_n7);
        let eq44_e808_d_n8: f64 = (eq44_e808 * eq44_e807_d_n8);
        let eq44_e808_d_n9: f64 = (eq44_e808 * eq44_e807_d_n9);
        let eq44_e808_d_n10: f64 = (eq44_e808 * eq44_e807_d_n10);
        let eq44_e808_d_n11: f64 = (eq44_e808 * eq44_e807_d_n11);
        let eq44_e808_d_n12: f64 = (eq44_e808 * eq44_e807_d_n12);
        let eq44_e808_d_n13: f64 = (eq44_e808 * eq44_e807_d_n13);
        let eq44_e808_d_n14: f64 = (eq44_e808 * eq44_e807_d_n14);
        let eq44_e808_d_n15: f64 = (eq44_e808 * eq44_e807_d_n15);
        let eq44_e808_d_n16: f64 = (eq44_e808 * eq44_e807_d_n16);
        let eq44_e808_d_n17: f64 = (eq44_e808 * eq44_e807_d_n17);
        let eq44_e808_d_n18: f64 = (eq44_e808 * eq44_e807_d_n18);
        let eq44_e808_d_n19: f64 = (eq44_e808 * eq44_e807_d_n19);
        let eq44_e808_d_n20: f64 = (eq44_e808 * eq44_e807_d_n20);
        let eq44_e808_d_n21: f64 = (eq44_e808 * eq44_e807_d_n21);
        let eq44_e808_d_n22: f64 = (eq44_e808 * eq44_e807_d_n22);
        let eq44_e808_d_b0: f64 = (eq44_e808 * eq44_e807_d_b0);
        let eq44_e808_d_b1: f64 = (eq44_e808 * eq44_e807_d_b1);
        let eq44_e808_d_b2: f64 = (eq44_e808 * eq44_e807_d_b2);
        let eq44_e808_d_b3: f64 = (eq44_e808 * eq44_e807_d_b3);
        let eq44_e808_d_b4: f64 = (eq44_e808 * eq44_e807_d_b4);
        let eq44_e808_d_b5: f64 = (eq44_e808 * eq44_e807_d_b5);
        let eq44_e808_d_b6: f64 = (eq44_e808 * eq44_e807_d_b6);
        let eq44_e808_d_b7: f64 = (eq44_e808 * eq44_e807_d_b7);
        let eq44_e808_d_b8: f64 = (eq44_e808 * eq44_e807_d_b8);
        let eq44_e808_d_b9: f64 = (eq44_e808 * eq44_e807_d_b9);
        let eq44_e808_d_b10: f64 = (eq44_e808 * eq44_e807_d_b10);
        let eq44_e808_d_b11: f64 = (eq44_e808 * eq44_e807_d_b11);
        let eq44_e808_d_b12: f64 = (eq44_e808 * eq44_e807_d_b12);
        let eq44_e808_d_b13: f64 = (eq44_e808 * eq44_e807_d_b13);
        let eq44_e808_d_b14: f64 = (eq44_e808 * eq44_e807_d_b14);
        let eq44_e808_d_b15: f64 = (eq44_e808 * eq44_e807_d_b15);
        let eq44_e808_d_b16: f64 = (eq44_e808 * eq44_e807_d_b16);
        let eq44_e808_d_b17: f64 = (eq44_e808 * eq44_e807_d_b17);
        let eq44_e808_d_b18: f64 = (eq44_e808 * eq44_e807_d_b18);
        let eq44_e808_d_b19: f64 = (eq44_e808 * eq44_e807_d_b19);
        let eq44_e808_d_b20: f64 = (eq44_e808 * eq44_e807_d_b20);
        let eq44_e808_d_b21: f64 = (eq44_e808 * eq44_e807_d_b21);
        let eq44_e808_d_b22: f64 = (eq44_e808 * eq44_e807_d_b22);
        let eq44_e808_d_b23: f64 = (eq44_e808 * eq44_e807_d_b23);
        let eq44_e808_d_b24: f64 = (eq44_e808 * eq44_e807_d_b24);
        let eq44_e808_d_b25: f64 = (eq44_e808 * eq44_e807_d_b25);
        let eq44_e808_d_b26: f64 = (eq44_e808 * eq44_e807_d_b26);
        let eq44_e808_d_b27: f64 = (eq44_e808 * eq44_e807_d_b27);
        let eq44_e808_d_b28: f64 = (eq44_e808 * eq44_e807_d_b28);
        let eq44_e808_d_b29: f64 = (eq44_e808 * eq44_e807_d_b29);
        let eq44_e808_d_b30: f64 = (eq44_e808 * eq44_e807_d_b30);
        let eq44_e808_d_b31: f64 = (eq44_e808 * eq44_e807_d_b31);
        let eq44_e808_d_b32: f64 = (eq44_e808 * eq44_e807_d_b32);
        let eq44_e808_d_b33: f64 = (eq44_e808 * eq44_e807_d_b33);
        let eq44_e808_d_b34: f64 = (eq44_e808 * eq44_e807_d_b34);
        let eq44_e808_d_b35: f64 = (eq44_e808 * eq44_e807_d_b35);
        let eq44_e808_d_b36: f64 = (eq44_e808 * eq44_e807_d_b36);
        let eq44_e808_d_b37: f64 = (eq44_e808 * eq44_e807_d_b37);
        let eq44_e808_d_b38: f64 = (eq44_e808 * eq44_e807_d_b38);
        let eq44_e808_d_b39: f64 = (eq44_e808 * eq44_e807_d_b39);
        let eq44_e808_d_b40: f64 = (eq44_e808 * eq44_e807_d_b40);
        let eq44_e808_d_b41: f64 = (eq44_e808 * eq44_e807_d_b41);
        let eq44_e808_d_b42: f64 = (eq44_e808 * eq44_e807_d_b42);
        let eq44_e808_d_b43: f64 = (eq44_e808 * eq44_e807_d_b43);
        let eq44_e808_d_b44: f64 = (eq44_e808 * eq44_e807_d_b44);
        let eq44_e808_d_b45: f64 = (eq44_e808 * eq44_e807_d_b45);
        let eq44_e808_d_b46: f64 = (eq44_e808 * eq44_e807_d_b46);
        let eq44_e808_d_b47: f64 = (eq44_e808 * eq44_e807_d_b47);
        let eq44_e808_d_b48: f64 = (eq44_e808 * eq44_e807_d_b48);
        let eq44_e808_d_b49: f64 = (eq44_e808 * eq44_e807_d_b49);
        let eq44_e808_d_b50: f64 = (eq44_e808 * eq44_e807_d_b50);
        let eq44_e808_d_b51: f64 = (eq44_e808 * eq44_e807_d_b51);
        let eq44_e808_d_b52: f64 = (eq44_e808 * eq44_e807_d_b52);
        let eq44_e808_d_b53: f64 = (eq44_e808 * eq44_e807_d_b53);
        let eq44_e808_d_b54: f64 = (eq44_e808 * eq44_e807_d_b54);
        let eq44_e810: f64 = (eq44_e808 - 1.0);
        let eq44_e811: f64 = (eq44_e804 * eq44_e810);
        let eq44_e811_d_n0: f64 = ((eq44_e804_d_n0 * eq44_e810) + (eq44_e804 * eq44_e808_d_n0));
        let eq44_e811_d_n1: f64 = ((eq44_e804_d_n1 * eq44_e810) + (eq44_e804 * eq44_e808_d_n1));
        let eq44_e811_d_n2: f64 = ((eq44_e804_d_n2 * eq44_e810) + (eq44_e804 * eq44_e808_d_n2));
        let eq44_e811_d_n3: f64 = ((eq44_e804_d_n3 * eq44_e810) + (eq44_e804 * eq44_e808_d_n3));
        let eq44_e811_d_n4: f64 = ((eq44_e804_d_n4 * eq44_e810) + (eq44_e804 * eq44_e808_d_n4));
        let eq44_e811_d_n5: f64 = ((eq44_e804_d_n5 * eq44_e810) + (eq44_e804 * eq44_e808_d_n5));
        let eq44_e811_d_n6: f64 = ((eq44_e804_d_n6 * eq44_e810) + (eq44_e804 * eq44_e808_d_n6));
        let eq44_e811_d_n7: f64 = ((eq44_e804_d_n7 * eq44_e810) + (eq44_e804 * eq44_e808_d_n7));
        let eq44_e811_d_n8: f64 = ((eq44_e804_d_n8 * eq44_e810) + (eq44_e804 * eq44_e808_d_n8));
        let eq44_e811_d_n9: f64 = ((eq44_e804_d_n9 * eq44_e810) + (eq44_e804 * eq44_e808_d_n9));
        let eq44_e811_d_n10: f64 = ((eq44_e804_d_n10 * eq44_e810) + (eq44_e804 * eq44_e808_d_n10));
        let eq44_e811_d_n11: f64 = ((eq44_e804_d_n11 * eq44_e810) + (eq44_e804 * eq44_e808_d_n11));
        let eq44_e811_d_n12: f64 = ((eq44_e804_d_n12 * eq44_e810) + (eq44_e804 * eq44_e808_d_n12));
        let eq44_e811_d_n13: f64 = ((eq44_e804_d_n13 * eq44_e810) + (eq44_e804 * eq44_e808_d_n13));
        let eq44_e811_d_n14: f64 = ((eq44_e804_d_n14 * eq44_e810) + (eq44_e804 * eq44_e808_d_n14));
        let eq44_e811_d_n15: f64 = ((eq44_e804_d_n15 * eq44_e810) + (eq44_e804 * eq44_e808_d_n15));
        let eq44_e811_d_n16: f64 = ((eq44_e804_d_n16 * eq44_e810) + (eq44_e804 * eq44_e808_d_n16));
        let eq44_e811_d_n17: f64 = ((eq44_e804_d_n17 * eq44_e810) + (eq44_e804 * eq44_e808_d_n17));
        let eq44_e811_d_n18: f64 = ((eq44_e804_d_n18 * eq44_e810) + (eq44_e804 * eq44_e808_d_n18));
        let eq44_e811_d_n19: f64 = ((eq44_e804_d_n19 * eq44_e810) + (eq44_e804 * eq44_e808_d_n19));
        let eq44_e811_d_n20: f64 = ((eq44_e804_d_n20 * eq44_e810) + (eq44_e804 * eq44_e808_d_n20));
        let eq44_e811_d_n21: f64 = ((eq44_e804_d_n21 * eq44_e810) + (eq44_e804 * eq44_e808_d_n21));
        let eq44_e811_d_n22: f64 = ((eq44_e804_d_n22 * eq44_e810) + (eq44_e804 * eq44_e808_d_n22));
        let eq44_e811_d_b0: f64 = ((eq44_e804_d_b0 * eq44_e810) + (eq44_e804 * eq44_e808_d_b0));
        let eq44_e811_d_b1: f64 = ((eq44_e804_d_b1 * eq44_e810) + (eq44_e804 * eq44_e808_d_b1));
        let eq44_e811_d_b2: f64 = ((eq44_e804_d_b2 * eq44_e810) + (eq44_e804 * eq44_e808_d_b2));
        let eq44_e811_d_b3: f64 = ((eq44_e804_d_b3 * eq44_e810) + (eq44_e804 * eq44_e808_d_b3));
        let eq44_e811_d_b4: f64 = ((eq44_e804_d_b4 * eq44_e810) + (eq44_e804 * eq44_e808_d_b4));
        let eq44_e811_d_b5: f64 = ((eq44_e804_d_b5 * eq44_e810) + (eq44_e804 * eq44_e808_d_b5));
        let eq44_e811_d_b6: f64 = ((eq44_e804_d_b6 * eq44_e810) + (eq44_e804 * eq44_e808_d_b6));
        let eq44_e811_d_b7: f64 = ((eq44_e804_d_b7 * eq44_e810) + (eq44_e804 * eq44_e808_d_b7));
        let eq44_e811_d_b8: f64 = ((eq44_e804_d_b8 * eq44_e810) + (eq44_e804 * eq44_e808_d_b8));
        let eq44_e811_d_b9: f64 = ((eq44_e804_d_b9 * eq44_e810) + (eq44_e804 * eq44_e808_d_b9));
        let eq44_e811_d_b10: f64 = ((eq44_e804_d_b10 * eq44_e810) + (eq44_e804 * eq44_e808_d_b10));
        let eq44_e811_d_b11: f64 = ((eq44_e804_d_b11 * eq44_e810) + (eq44_e804 * eq44_e808_d_b11));
        let eq44_e811_d_b12: f64 = ((eq44_e804_d_b12 * eq44_e810) + (eq44_e804 * eq44_e808_d_b12));
        let eq44_e811_d_b13: f64 = ((eq44_e804_d_b13 * eq44_e810) + (eq44_e804 * eq44_e808_d_b13));
        let eq44_e811_d_b14: f64 = ((eq44_e804_d_b14 * eq44_e810) + (eq44_e804 * eq44_e808_d_b14));
        let eq44_e811_d_b15: f64 = ((eq44_e804_d_b15 * eq44_e810) + (eq44_e804 * eq44_e808_d_b15));
        let eq44_e811_d_b16: f64 = ((eq44_e804_d_b16 * eq44_e810) + (eq44_e804 * eq44_e808_d_b16));
        let eq44_e811_d_b17: f64 = ((eq44_e804_d_b17 * eq44_e810) + (eq44_e804 * eq44_e808_d_b17));
        let eq44_e811_d_b18: f64 = ((eq44_e804_d_b18 * eq44_e810) + (eq44_e804 * eq44_e808_d_b18));
        let eq44_e811_d_b19: f64 = ((eq44_e804_d_b19 * eq44_e810) + (eq44_e804 * eq44_e808_d_b19));
        let eq44_e811_d_b20: f64 = ((eq44_e804_d_b20 * eq44_e810) + (eq44_e804 * eq44_e808_d_b20));
        let eq44_e811_d_b21: f64 = ((eq44_e804_d_b21 * eq44_e810) + (eq44_e804 * eq44_e808_d_b21));
        let eq44_e811_d_b22: f64 = ((eq44_e804_d_b22 * eq44_e810) + (eq44_e804 * eq44_e808_d_b22));
        let eq44_e811_d_b23: f64 = ((eq44_e804_d_b23 * eq44_e810) + (eq44_e804 * eq44_e808_d_b23));
        let eq44_e811_d_b24: f64 = ((eq44_e804_d_b24 * eq44_e810) + (eq44_e804 * eq44_e808_d_b24));
        let eq44_e811_d_b25: f64 = ((eq44_e804_d_b25 * eq44_e810) + (eq44_e804 * eq44_e808_d_b25));
        let eq44_e811_d_b26: f64 = ((eq44_e804_d_b26 * eq44_e810) + (eq44_e804 * eq44_e808_d_b26));
        let eq44_e811_d_b27: f64 = ((eq44_e804_d_b27 * eq44_e810) + (eq44_e804 * eq44_e808_d_b27));
        let eq44_e811_d_b28: f64 = ((eq44_e804_d_b28 * eq44_e810) + (eq44_e804 * eq44_e808_d_b28));
        let eq44_e811_d_b29: f64 = ((eq44_e804_d_b29 * eq44_e810) + (eq44_e804 * eq44_e808_d_b29));
        let eq44_e811_d_b30: f64 = ((eq44_e804_d_b30 * eq44_e810) + (eq44_e804 * eq44_e808_d_b30));
        let eq44_e811_d_b31: f64 = ((eq44_e804_d_b31 * eq44_e810) + (eq44_e804 * eq44_e808_d_b31));
        let eq44_e811_d_b32: f64 = ((eq44_e804_d_b32 * eq44_e810) + (eq44_e804 * eq44_e808_d_b32));
        let eq44_e811_d_b33: f64 = ((eq44_e804_d_b33 * eq44_e810) + (eq44_e804 * eq44_e808_d_b33));
        let eq44_e811_d_b34: f64 = ((eq44_e804_d_b34 * eq44_e810) + (eq44_e804 * eq44_e808_d_b34));
        let eq44_e811_d_b35: f64 = ((eq44_e804_d_b35 * eq44_e810) + (eq44_e804 * eq44_e808_d_b35));
        let eq44_e811_d_b36: f64 = ((eq44_e804_d_b36 * eq44_e810) + (eq44_e804 * eq44_e808_d_b36));
        let eq44_e811_d_b37: f64 = ((eq44_e804_d_b37 * eq44_e810) + (eq44_e804 * eq44_e808_d_b37));
        let eq44_e811_d_b38: f64 = ((eq44_e804_d_b38 * eq44_e810) + (eq44_e804 * eq44_e808_d_b38));
        let eq44_e811_d_b39: f64 = ((eq44_e804_d_b39 * eq44_e810) + (eq44_e804 * eq44_e808_d_b39));
        let eq44_e811_d_b40: f64 = ((eq44_e804_d_b40 * eq44_e810) + (eq44_e804 * eq44_e808_d_b40));
        let eq44_e811_d_b41: f64 = ((eq44_e804_d_b41 * eq44_e810) + (eq44_e804 * eq44_e808_d_b41));
        let eq44_e811_d_b42: f64 = ((eq44_e804_d_b42 * eq44_e810) + (eq44_e804 * eq44_e808_d_b42));
        let eq44_e811_d_b43: f64 = ((eq44_e804_d_b43 * eq44_e810) + (eq44_e804 * eq44_e808_d_b43));
        let eq44_e811_d_b44: f64 = ((eq44_e804_d_b44 * eq44_e810) + (eq44_e804 * eq44_e808_d_b44));
        let eq44_e811_d_b45: f64 = ((eq44_e804_d_b45 * eq44_e810) + (eq44_e804 * eq44_e808_d_b45));
        let eq44_e811_d_b46: f64 = ((eq44_e804_d_b46 * eq44_e810) + (eq44_e804 * eq44_e808_d_b46));
        let eq44_e811_d_b47: f64 = ((eq44_e804_d_b47 * eq44_e810) + (eq44_e804 * eq44_e808_d_b47));
        let eq44_e811_d_b48: f64 = ((eq44_e804_d_b48 * eq44_e810) + (eq44_e804 * eq44_e808_d_b48));
        let eq44_e811_d_b49: f64 = ((eq44_e804_d_b49 * eq44_e810) + (eq44_e804 * eq44_e808_d_b49));
        let eq44_e811_d_b50: f64 = ((eq44_e804_d_b50 * eq44_e810) + (eq44_e804 * eq44_e808_d_b50));
        let eq44_e811_d_b51: f64 = ((eq44_e804_d_b51 * eq44_e810) + (eq44_e804 * eq44_e808_d_b51));
        let eq44_e811_d_b52: f64 = ((eq44_e804_d_b52 * eq44_e810) + (eq44_e804 * eq44_e808_d_b52));
        let eq44_e811_d_b53: f64 = ((eq44_e804_d_b53 * eq44_e810) + (eq44_e804 * eq44_e808_d_b53));
        let eq44_e811_d_b54: f64 = ((eq44_e804_d_b54 * eq44_e810) + (eq44_e804 * eq44_e808_d_b54));
        let eq44_e813: f64 = (eq44_e811 * 0.5);
        let eq44_e813_d_n0: f64 = (eq44_e811_d_n0 * 0.5);
        let eq44_e813_d_n1: f64 = (eq44_e811_d_n1 * 0.5);
        let eq44_e813_d_n2: f64 = (eq44_e811_d_n2 * 0.5);
        let eq44_e813_d_n3: f64 = (eq44_e811_d_n3 * 0.5);
        let eq44_e813_d_n4: f64 = (eq44_e811_d_n4 * 0.5);
        let eq44_e813_d_n5: f64 = (eq44_e811_d_n5 * 0.5);
        let eq44_e813_d_n6: f64 = (eq44_e811_d_n6 * 0.5);
        let eq44_e813_d_n7: f64 = (eq44_e811_d_n7 * 0.5);
        let eq44_e813_d_n8: f64 = (eq44_e811_d_n8 * 0.5);
        let eq44_e813_d_n9: f64 = (eq44_e811_d_n9 * 0.5);
        let eq44_e813_d_n10: f64 = (eq44_e811_d_n10 * 0.5);
        let eq44_e813_d_n11: f64 = (eq44_e811_d_n11 * 0.5);
        let eq44_e813_d_n12: f64 = (eq44_e811_d_n12 * 0.5);
        let eq44_e813_d_n13: f64 = (eq44_e811_d_n13 * 0.5);
        let eq44_e813_d_n14: f64 = (eq44_e811_d_n14 * 0.5);
        let eq44_e813_d_n15: f64 = (eq44_e811_d_n15 * 0.5);
        let eq44_e813_d_n16: f64 = (eq44_e811_d_n16 * 0.5);
        let eq44_e813_d_n17: f64 = (eq44_e811_d_n17 * 0.5);
        let eq44_e813_d_n18: f64 = (eq44_e811_d_n18 * 0.5);
        let eq44_e813_d_n19: f64 = (eq44_e811_d_n19 * 0.5);
        let eq44_e813_d_n20: f64 = (eq44_e811_d_n20 * 0.5);
        let eq44_e813_d_n21: f64 = (eq44_e811_d_n21 * 0.5);
        let eq44_e813_d_n22: f64 = (eq44_e811_d_n22 * 0.5);
        let eq44_e813_d_b0: f64 = (eq44_e811_d_b0 * 0.5);
        let eq44_e813_d_b1: f64 = (eq44_e811_d_b1 * 0.5);
        let eq44_e813_d_b2: f64 = (eq44_e811_d_b2 * 0.5);
        let eq44_e813_d_b3: f64 = (eq44_e811_d_b3 * 0.5);
        let eq44_e813_d_b4: f64 = (eq44_e811_d_b4 * 0.5);
        let eq44_e813_d_b5: f64 = (eq44_e811_d_b5 * 0.5);
        let eq44_e813_d_b6: f64 = (eq44_e811_d_b6 * 0.5);
        let eq44_e813_d_b7: f64 = (eq44_e811_d_b7 * 0.5);
        let eq44_e813_d_b8: f64 = (eq44_e811_d_b8 * 0.5);
        let eq44_e813_d_b9: f64 = (eq44_e811_d_b9 * 0.5);
        let eq44_e813_d_b10: f64 = (eq44_e811_d_b10 * 0.5);
        let eq44_e813_d_b11: f64 = (eq44_e811_d_b11 * 0.5);
        let eq44_e813_d_b12: f64 = (eq44_e811_d_b12 * 0.5);
        let eq44_e813_d_b13: f64 = (eq44_e811_d_b13 * 0.5);
        let eq44_e813_d_b14: f64 = (eq44_e811_d_b14 * 0.5);
        let eq44_e813_d_b15: f64 = (eq44_e811_d_b15 * 0.5);
        let eq44_e813_d_b16: f64 = (eq44_e811_d_b16 * 0.5);
        let eq44_e813_d_b17: f64 = (eq44_e811_d_b17 * 0.5);
        let eq44_e813_d_b18: f64 = (eq44_e811_d_b18 * 0.5);
        let eq44_e813_d_b19: f64 = (eq44_e811_d_b19 * 0.5);
        let eq44_e813_d_b20: f64 = (eq44_e811_d_b20 * 0.5);
        let eq44_e813_d_b21: f64 = (eq44_e811_d_b21 * 0.5);
        let eq44_e813_d_b22: f64 = (eq44_e811_d_b22 * 0.5);
        let eq44_e813_d_b23: f64 = (eq44_e811_d_b23 * 0.5);
        let eq44_e813_d_b24: f64 = (eq44_e811_d_b24 * 0.5);
        let eq44_e813_d_b25: f64 = (eq44_e811_d_b25 * 0.5);
        let eq44_e813_d_b26: f64 = (eq44_e811_d_b26 * 0.5);
        let eq44_e813_d_b27: f64 = (eq44_e811_d_b27 * 0.5);
        let eq44_e813_d_b28: f64 = (eq44_e811_d_b28 * 0.5);
        let eq44_e813_d_b29: f64 = (eq44_e811_d_b29 * 0.5);
        let eq44_e813_d_b30: f64 = (eq44_e811_d_b30 * 0.5);
        let eq44_e813_d_b31: f64 = (eq44_e811_d_b31 * 0.5);
        let eq44_e813_d_b32: f64 = (eq44_e811_d_b32 * 0.5);
        let eq44_e813_d_b33: f64 = (eq44_e811_d_b33 * 0.5);
        let eq44_e813_d_b34: f64 = (eq44_e811_d_b34 * 0.5);
        let eq44_e813_d_b35: f64 = (eq44_e811_d_b35 * 0.5);
        let eq44_e813_d_b36: f64 = (eq44_e811_d_b36 * 0.5);
        let eq44_e813_d_b37: f64 = (eq44_e811_d_b37 * 0.5);
        let eq44_e813_d_b38: f64 = (eq44_e811_d_b38 * 0.5);
        let eq44_e813_d_b39: f64 = (eq44_e811_d_b39 * 0.5);
        let eq44_e813_d_b40: f64 = (eq44_e811_d_b40 * 0.5);
        let eq44_e813_d_b41: f64 = (eq44_e811_d_b41 * 0.5);
        let eq44_e813_d_b42: f64 = (eq44_e811_d_b42 * 0.5);
        let eq44_e813_d_b43: f64 = (eq44_e811_d_b43 * 0.5);
        let eq44_e813_d_b44: f64 = (eq44_e811_d_b44 * 0.5);
        let eq44_e813_d_b45: f64 = (eq44_e811_d_b45 * 0.5);
        let eq44_e813_d_b46: f64 = (eq44_e811_d_b46 * 0.5);
        let eq44_e813_d_b47: f64 = (eq44_e811_d_b47 * 0.5);
        let eq44_e813_d_b48: f64 = (eq44_e811_d_b48 * 0.5);
        let eq44_e813_d_b49: f64 = (eq44_e811_d_b49 * 0.5);
        let eq44_e813_d_b50: f64 = (eq44_e811_d_b50 * 0.5);
        let eq44_e813_d_b51: f64 = (eq44_e811_d_b51 * 0.5);
        let eq44_e813_d_b52: f64 = (eq44_e811_d_b52 * 0.5);
        let eq44_e813_d_b53: f64 = (eq44_e811_d_b53 * 0.5);
        let eq44_e813_d_b54: f64 = (eq44_e811_d_b54 * 0.5);
        (eq44_e813, eq44_e813_d_n0, eq44_e813_d_n1, eq44_e813_d_n2, eq44_e813_d_n3, eq44_e813_d_n4, eq44_e813_d_n5, eq44_e813_d_n6, eq44_e813_d_n7, eq44_e813_d_n8, eq44_e813_d_n9, eq44_e813_d_n10, eq44_e813_d_n11, eq44_e813_d_n12, eq44_e813_d_n13, eq44_e813_d_n14, eq44_e813_d_n15, eq44_e813_d_n16, eq44_e813_d_n17, eq44_e813_d_n18, eq44_e813_d_n19, eq44_e813_d_n20, eq44_e813_d_n21, eq44_e813_d_n22, eq44_e813_d_b0, eq44_e813_d_b1, eq44_e813_d_b2, eq44_e813_d_b3, eq44_e813_d_b4, eq44_e813_d_b5, eq44_e813_d_b6, eq44_e813_d_b7, eq44_e813_d_b8, eq44_e813_d_b9, eq44_e813_d_b10, eq44_e813_d_b11, eq44_e813_d_b12, eq44_e813_d_b13, eq44_e813_d_b14, eq44_e813_d_b15, eq44_e813_d_b16, eq44_e813_d_b17, eq44_e813_d_b18, eq44_e813_d_b19, eq44_e813_d_b20, eq44_e813_d_b21, eq44_e813_d_b22, eq44_e813_d_b23, eq44_e813_d_b24, eq44_e813_d_b25, eq44_e813_d_b26, eq44_e813_d_b27, eq44_e813_d_b28, eq44_e813_d_b29, eq44_e813_d_b30, eq44_e813_d_b31, eq44_e813_d_b32, eq44_e813_d_b33, eq44_e813_d_b34, eq44_e813_d_b35, eq44_e813_d_b36, eq44_e813_d_b37, eq44_e813_d_b38, eq44_e813_d_b39, eq44_e813_d_b40, eq44_e813_d_b41, eq44_e813_d_b42, eq44_e813_d_b43, eq44_e813_d_b44, eq44_e813_d_b45, eq44_e813_d_b46, eq44_e813_d_b47, eq44_e813_d_b48, eq44_e813_d_b49, eq44_e813_d_b50, eq44_e813_d_b51, eq44_e813_d_b52, eq44_e813_d_b53, eq44_e813_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e815;
        let eq44_node_derivatives: [f64; 23] = [eq44_e815_d_n0, eq44_e815_d_n1, eq44_e815_d_n2, eq44_e815_d_n3, eq44_e815_d_n4, eq44_e815_d_n5, eq44_e815_d_n6, eq44_e815_d_n7, eq44_e815_d_n8, eq44_e815_d_n9, eq44_e815_d_n10, eq44_e815_d_n11, eq44_e815_d_n12, eq44_e815_d_n13, eq44_e815_d_n14, eq44_e815_d_n15, eq44_e815_d_n16, eq44_e815_d_n17, eq44_e815_d_n18, eq44_e815_d_n19, eq44_e815_d_n20, eq44_e815_d_n21, eq44_e815_d_n22];
        let eq44_branch_derivatives: [f64; 55] = [eq44_e815_d_b0, eq44_e815_d_b1, eq44_e815_d_b2, eq44_e815_d_b3, eq44_e815_d_b4, eq44_e815_d_b5, eq44_e815_d_b6, eq44_e815_d_b7, eq44_e815_d_b8, eq44_e815_d_b9, eq44_e815_d_b10, eq44_e815_d_b11, eq44_e815_d_b12, eq44_e815_d_b13, eq44_e815_d_b14, eq44_e815_d_b15, eq44_e815_d_b16, eq44_e815_d_b17, eq44_e815_d_b18, eq44_e815_d_b19, eq44_e815_d_b20, eq44_e815_d_b21, eq44_e815_d_b22, eq44_e815_d_b23, eq44_e815_d_b24, eq44_e815_d_b25, eq44_e815_d_b26, eq44_e815_d_b27, eq44_e815_d_b28, eq44_e815_d_b29, eq44_e815_d_b30, eq44_e815_d_b31, eq44_e815_d_b32, eq44_e815_d_b33, eq44_e815_d_b34, eq44_e815_d_b35, eq44_e815_d_b36, eq44_e815_d_b37, eq44_e815_d_b38, eq44_e815_d_b39, eq44_e815_d_b40, eq44_e815_d_b41, eq44_e815_d_b42, eq44_e815_d_b43, eq44_e815_d_b44, eq44_e815_d_b45, eq44_e815_d_b46, eq44_e815_d_b47, eq44_e815_d_b48, eq44_e815_d_b49, eq44_e815_d_b50, eq44_e815_d_b51, eq44_e815_d_b52, eq44_e815_d_b53, eq44_e815_d_b54];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
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
        var_en1: f64,
        var_en1_db0: f64,
        var_en1_db1: f64,
        var_en1_db10: f64,
        var_en1_db11: f64,
        var_en1_db12: f64,
        var_en1_db13: f64,
        var_en1_db14: f64,
        var_en1_db15: f64,
        var_en1_db16: f64,
        var_en1_db17: f64,
        var_en1_db18: f64,
        var_en1_db19: f64,
        var_en1_db2: f64,
        var_en1_db20: f64,
        var_en1_db21: f64,
        var_en1_db22: f64,
        var_en1_db23: f64,
        var_en1_db24: f64,
        var_en1_db25: f64,
        var_en1_db26: f64,
        var_en1_db27: f64,
        var_en1_db28: f64,
        var_en1_db29: f64,
        var_en1_db3: f64,
        var_en1_db30: f64,
        var_en1_db31: f64,
        var_en1_db32: f64,
        var_en1_db33: f64,
        var_en1_db34: f64,
        var_en1_db35: f64,
        var_en1_db36: f64,
        var_en1_db37: f64,
        var_en1_db38: f64,
        var_en1_db39: f64,
        var_en1_db4: f64,
        var_en1_db40: f64,
        var_en1_db41: f64,
        var_en1_db42: f64,
        var_en1_db43: f64,
        var_en1_db44: f64,
        var_en1_db45: f64,
        var_en1_db46: f64,
        var_en1_db47: f64,
        var_en1_db48: f64,
        var_en1_db49: f64,
        var_en1_db5: f64,
        var_en1_db50: f64,
        var_en1_db51: f64,
        var_en1_db52: f64,
        var_en1_db53: f64,
        var_en1_db54: f64,
        var_en1_db6: f64,
        var_en1_db7: f64,
        var_en1_db8: f64,
        var_en1_db9: f64,
        var_en1_dn0: f64,
        var_en1_dn1: f64,
        var_en1_dn10: f64,
        var_en1_dn11: f64,
        var_en1_dn12: f64,
        var_en1_dn13: f64,
        var_en1_dn14: f64,
        var_en1_dn15: f64,
        var_en1_dn16: f64,
        var_en1_dn17: f64,
        var_en1_dn18: f64,
        var_en1_dn19: f64,
        var_en1_dn2: f64,
        var_en1_dn20: f64,
        var_en1_dn21: f64,
        var_en1_dn22: f64,
        var_en1_dn3: f64,
        var_en1_dn4: f64,
        var_en1_dn5: f64,
        var_en1_dn6: f64,
        var_en1_dn7: f64,
        var_en1_dn8: f64,
        var_en1_dn9: f64,
        var_guard353: f64,
        var_guard354: f64,
        var_guard355: f64,
        var_guard356: f64,
        var_guard357: f64,
        var_guard358: f64,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq45_e834, eq45_e834_d_n0, eq45_e834_d_n1, eq45_e834_d_n2, eq45_e834_d_n3, eq45_e834_d_n4, eq45_e834_d_n5, eq45_e834_d_n6, eq45_e834_d_n7, eq45_e834_d_n8, eq45_e834_d_n9, eq45_e834_d_n10, eq45_e834_d_n11, eq45_e834_d_n12, eq45_e834_d_n13, eq45_e834_d_n14, eq45_e834_d_n15, eq45_e834_d_n16, eq45_e834_d_n17, eq45_e834_d_n18, eq45_e834_d_n19, eq45_e834_d_n20, eq45_e834_d_n21, eq45_e834_d_n22, eq45_e834_d_b0, eq45_e834_d_b1, eq45_e834_d_b2, eq45_e834_d_b3, eq45_e834_d_b4, eq45_e834_d_b5, eq45_e834_d_b6, eq45_e834_d_b7, eq45_e834_d_b8, eq45_e834_d_b9, eq45_e834_d_b10, eq45_e834_d_b11, eq45_e834_d_b12, eq45_e834_d_b13, eq45_e834_d_b14, eq45_e834_d_b15, eq45_e834_d_b16, eq45_e834_d_b17, eq45_e834_d_b18, eq45_e834_d_b19, eq45_e834_d_b20, eq45_e834_d_b21, eq45_e834_d_b22, eq45_e834_d_b23, eq45_e834_d_b24, eq45_e834_d_b25, eq45_e834_d_b26, eq45_e834_d_b27, eq45_e834_d_b28, eq45_e834_d_b29, eq45_e834_d_b30, eq45_e834_d_b31, eq45_e834_d_b32, eq45_e834_d_b33, eq45_e834_d_b34, eq45_e834_d_b35, eq45_e834_d_b36, eq45_e834_d_b37, eq45_e834_d_b38, eq45_e834_d_b39, eq45_e834_d_b40, eq45_e834_d_b41, eq45_e834_d_b42, eq45_e834_d_b43, eq45_e834_d_b44, eq45_e834_d_b45, eq45_e834_d_b46, eq45_e834_d_b47, eq45_e834_d_b48, eq45_e834_d_b49, eq45_e834_d_b50, eq45_e834_d_b51, eq45_e834_d_b52, eq45_e834_d_b53, eq45_e834_d_b54,) = {
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq45_e830: f64 = (p.p144 * var_en1);
        let eq45_e830_d_n0: f64 = (p.p144 * var_en1_dn0);
        let eq45_e830_d_n1: f64 = (p.p144 * var_en1_dn1);
        let eq45_e830_d_n2: f64 = (p.p144 * var_en1_dn2);
        let eq45_e830_d_n3: f64 = (p.p144 * var_en1_dn3);
        let eq45_e830_d_n4: f64 = (p.p144 * var_en1_dn4);
        let eq45_e830_d_n5: f64 = (p.p144 * var_en1_dn5);
        let eq45_e830_d_n6: f64 = (p.p144 * var_en1_dn6);
        let eq45_e830_d_n7: f64 = (p.p144 * var_en1_dn7);
        let eq45_e830_d_n8: f64 = (p.p144 * var_en1_dn8);
        let eq45_e830_d_n9: f64 = (p.p144 * var_en1_dn9);
        let eq45_e830_d_n10: f64 = (p.p144 * var_en1_dn10);
        let eq45_e830_d_n11: f64 = (p.p144 * var_en1_dn11);
        let eq45_e830_d_n12: f64 = (p.p144 * var_en1_dn12);
        let eq45_e830_d_n13: f64 = (p.p144 * var_en1_dn13);
        let eq45_e830_d_n14: f64 = (p.p144 * var_en1_dn14);
        let eq45_e830_d_n15: f64 = (p.p144 * var_en1_dn15);
        let eq45_e830_d_n16: f64 = (p.p144 * var_en1_dn16);
        let eq45_e830_d_n17: f64 = (p.p144 * var_en1_dn17);
        let eq45_e830_d_n18: f64 = (p.p144 * var_en1_dn18);
        let eq45_e830_d_n19: f64 = (p.p144 * var_en1_dn19);
        let eq45_e830_d_n20: f64 = (p.p144 * var_en1_dn20);
        let eq45_e830_d_n21: f64 = (p.p144 * var_en1_dn21);
        let eq45_e830_d_n22: f64 = (p.p144 * var_en1_dn22);
        let eq45_e830_d_b0: f64 = (p.p144 * var_en1_db0);
        let eq45_e830_d_b1: f64 = (p.p144 * var_en1_db1);
        let eq45_e830_d_b2: f64 = (p.p144 * var_en1_db2);
        let eq45_e830_d_b3: f64 = (p.p144 * var_en1_db3);
        let eq45_e830_d_b4: f64 = (p.p144 * var_en1_db4);
        let eq45_e830_d_b5: f64 = (p.p144 * var_en1_db5);
        let eq45_e830_d_b6: f64 = (p.p144 * var_en1_db6);
        let eq45_e830_d_b7: f64 = (p.p144 * var_en1_db7);
        let eq45_e830_d_b8: f64 = (p.p144 * var_en1_db8);
        let eq45_e830_d_b9: f64 = (p.p144 * var_en1_db9);
        let eq45_e830_d_b10: f64 = (p.p144 * var_en1_db10);
        let eq45_e830_d_b11: f64 = (p.p144 * var_en1_db11);
        let eq45_e830_d_b12: f64 = (p.p144 * var_en1_db12);
        let eq45_e830_d_b13: f64 = (p.p144 * var_en1_db13);
        let eq45_e830_d_b14: f64 = (p.p144 * var_en1_db14);
        let eq45_e830_d_b15: f64 = (p.p144 * var_en1_db15);
        let eq45_e830_d_b16: f64 = (p.p144 * var_en1_db16);
        let eq45_e830_d_b17: f64 = (p.p144 * var_en1_db17);
        let eq45_e830_d_b18: f64 = (p.p144 * var_en1_db18);
        let eq45_e830_d_b19: f64 = (p.p144 * var_en1_db19);
        let eq45_e830_d_b20: f64 = (p.p144 * var_en1_db20);
        let eq45_e830_d_b21: f64 = (p.p144 * var_en1_db21);
        let eq45_e830_d_b22: f64 = (p.p144 * var_en1_db22);
        let eq45_e830_d_b23: f64 = (p.p144 * var_en1_db23);
        let eq45_e830_d_b24: f64 = (p.p144 * var_en1_db24);
        let eq45_e830_d_b25: f64 = (p.p144 * var_en1_db25);
        let eq45_e830_d_b26: f64 = (p.p144 * var_en1_db26);
        let eq45_e830_d_b27: f64 = (p.p144 * var_en1_db27);
        let eq45_e830_d_b28: f64 = (p.p144 * var_en1_db28);
        let eq45_e830_d_b29: f64 = (p.p144 * var_en1_db29);
        let eq45_e830_d_b30: f64 = (p.p144 * var_en1_db30);
        let eq45_e830_d_b31: f64 = (p.p144 * var_en1_db31);
        let eq45_e830_d_b32: f64 = (p.p144 * var_en1_db32);
        let eq45_e830_d_b33: f64 = (p.p144 * var_en1_db33);
        let eq45_e830_d_b34: f64 = (p.p144 * var_en1_db34);
        let eq45_e830_d_b35: f64 = (p.p144 * var_en1_db35);
        let eq45_e830_d_b36: f64 = (p.p144 * var_en1_db36);
        let eq45_e830_d_b37: f64 = (p.p144 * var_en1_db37);
        let eq45_e830_d_b38: f64 = (p.p144 * var_en1_db38);
        let eq45_e830_d_b39: f64 = (p.p144 * var_en1_db39);
        let eq45_e830_d_b40: f64 = (p.p144 * var_en1_db40);
        let eq45_e830_d_b41: f64 = (p.p144 * var_en1_db41);
        let eq45_e830_d_b42: f64 = (p.p144 * var_en1_db42);
        let eq45_e830_d_b43: f64 = (p.p144 * var_en1_db43);
        let eq45_e830_d_b44: f64 = (p.p144 * var_en1_db44);
        let eq45_e830_d_b45: f64 = (p.p144 * var_en1_db45);
        let eq45_e830_d_b46: f64 = (p.p144 * var_en1_db46);
        let eq45_e830_d_b47: f64 = (p.p144 * var_en1_db47);
        let eq45_e830_d_b48: f64 = (p.p144 * var_en1_db48);
        let eq45_e830_d_b49: f64 = (p.p144 * var_en1_db49);
        let eq45_e830_d_b50: f64 = (p.p144 * var_en1_db50);
        let eq45_e830_d_b51: f64 = (p.p144 * var_en1_db51);
        let eq45_e830_d_b52: f64 = (p.p144 * var_en1_db52);
        let eq45_e830_d_b53: f64 = (p.p144 * var_en1_db53);
        let eq45_e830_d_b54: f64 = (p.p144 * var_en1_db54);
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
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq46_e849: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (nv6 - 0.0));
        let eq46_e850: f64 = (p.p144 * eq46_e849);
        (eq46_e850, (p.p144 * ddt_scale),)
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
        let eq51_e915: f64 = (p.p6 * s.v[41]);
        let eq51_e917: f64 = (eq51_e915 * s.v[94]);
        let eq51_e917_d_n0: f64 = (((p.p6 * s.dn[41][0]) * s.v[94]) + (eq51_e915 * s.dn[94][0]));
        let eq51_e917_d_n1: f64 = (((p.p6 * s.dn[41][1]) * s.v[94]) + (eq51_e915 * s.dn[94][1]));
        let eq51_e917_d_n2: f64 = (((p.p6 * s.dn[41][2]) * s.v[94]) + (eq51_e915 * s.dn[94][2]));
        let eq51_e917_d_n3: f64 = (((p.p6 * s.dn[41][3]) * s.v[94]) + (eq51_e915 * s.dn[94][3]));
        let eq51_e917_d_n4: f64 = (((p.p6 * s.dn[41][4]) * s.v[94]) + (eq51_e915 * s.dn[94][4]));
        let eq51_e917_d_n5: f64 = (((p.p6 * s.dn[41][5]) * s.v[94]) + (eq51_e915 * s.dn[94][5]));
        let eq51_e917_d_n6: f64 = (((p.p6 * s.dn[41][6]) * s.v[94]) + (eq51_e915 * s.dn[94][6]));
        let eq51_e917_d_n7: f64 = (((p.p6 * s.dn[41][7]) * s.v[94]) + (eq51_e915 * s.dn[94][7]));
        let eq51_e917_d_n8: f64 = (((p.p6 * s.dn[41][8]) * s.v[94]) + (eq51_e915 * s.dn[94][8]));
        let eq51_e917_d_n9: f64 = (((p.p6 * s.dn[41][9]) * s.v[94]) + (eq51_e915 * s.dn[94][9]));
        let eq51_e917_d_n10: f64 = (((p.p6 * s.dn[41][10]) * s.v[94]) + (eq51_e915 * s.dn[94][10]));
        let eq51_e917_d_n11: f64 = (((p.p6 * s.dn[41][11]) * s.v[94]) + (eq51_e915 * s.dn[94][11]));
        let eq51_e917_d_n12: f64 = (((p.p6 * s.dn[41][12]) * s.v[94]) + (eq51_e915 * s.dn[94][12]));
        let eq51_e917_d_n13: f64 = (((p.p6 * s.dn[41][13]) * s.v[94]) + (eq51_e915 * s.dn[94][13]));
        let eq51_e917_d_n14: f64 = (((p.p6 * s.dn[41][14]) * s.v[94]) + (eq51_e915 * s.dn[94][14]));
        let eq51_e917_d_n15: f64 = (((p.p6 * s.dn[41][15]) * s.v[94]) + (eq51_e915 * s.dn[94][15]));
        let eq51_e917_d_n16: f64 = (((p.p6 * s.dn[41][16]) * s.v[94]) + (eq51_e915 * s.dn[94][16]));
        let eq51_e917_d_n17: f64 = (((p.p6 * s.dn[41][17]) * s.v[94]) + (eq51_e915 * s.dn[94][17]));
        let eq51_e917_d_n18: f64 = (((p.p6 * s.dn[41][18]) * s.v[94]) + (eq51_e915 * s.dn[94][18]));
        let eq51_e917_d_n19: f64 = (((p.p6 * s.dn[41][19]) * s.v[94]) + (eq51_e915 * s.dn[94][19]));
        let eq51_e917_d_n20: f64 = (((p.p6 * s.dn[41][20]) * s.v[94]) + (eq51_e915 * s.dn[94][20]));
        let eq51_e917_d_n21: f64 = (((p.p6 * s.dn[41][21]) * s.v[94]) + (eq51_e915 * s.dn[94][21]));
        let eq51_e917_d_n22: f64 = (((p.p6 * s.dn[41][22]) * s.v[94]) + (eq51_e915 * s.dn[94][22]));
        let eq51_e917_d_b0: f64 = (((p.p6 * s.db[41][0]) * s.v[94]) + (eq51_e915 * s.db[94][0]));
        let eq51_e917_d_b1: f64 = (((p.p6 * s.db[41][1]) * s.v[94]) + (eq51_e915 * s.db[94][1]));
        let eq51_e917_d_b2: f64 = (((p.p6 * s.db[41][2]) * s.v[94]) + (eq51_e915 * s.db[94][2]));
        let eq51_e917_d_b3: f64 = (((p.p6 * s.db[41][3]) * s.v[94]) + (eq51_e915 * s.db[94][3]));
        let eq51_e917_d_b4: f64 = (((p.p6 * s.db[41][4]) * s.v[94]) + (eq51_e915 * s.db[94][4]));
        let eq51_e917_d_b5: f64 = (((p.p6 * s.db[41][5]) * s.v[94]) + (eq51_e915 * s.db[94][5]));
        let eq51_e917_d_b6: f64 = (((p.p6 * s.db[41][6]) * s.v[94]) + (eq51_e915 * s.db[94][6]));
        let eq51_e917_d_b7: f64 = (((p.p6 * s.db[41][7]) * s.v[94]) + (eq51_e915 * s.db[94][7]));
        let eq51_e917_d_b8: f64 = (((p.p6 * s.db[41][8]) * s.v[94]) + (eq51_e915 * s.db[94][8]));
        let eq51_e917_d_b9: f64 = (((p.p6 * s.db[41][9]) * s.v[94]) + (eq51_e915 * s.db[94][9]));
        let eq51_e917_d_b10: f64 = (((p.p6 * s.db[41][10]) * s.v[94]) + (eq51_e915 * s.db[94][10]));
        let eq51_e917_d_b11: f64 = (((p.p6 * s.db[41][11]) * s.v[94]) + (eq51_e915 * s.db[94][11]));
        let eq51_e917_d_b12: f64 = (((p.p6 * s.db[41][12]) * s.v[94]) + (eq51_e915 * s.db[94][12]));
        let eq51_e917_d_b13: f64 = (((p.p6 * s.db[41][13]) * s.v[94]) + (eq51_e915 * s.db[94][13]));
        let eq51_e917_d_b14: f64 = (((p.p6 * s.db[41][14]) * s.v[94]) + (eq51_e915 * s.db[94][14]));
        let eq51_e917_d_b15: f64 = (((p.p6 * s.db[41][15]) * s.v[94]) + (eq51_e915 * s.db[94][15]));
        let eq51_e917_d_b16: f64 = (((p.p6 * s.db[41][16]) * s.v[94]) + (eq51_e915 * s.db[94][16]));
        let eq51_e917_d_b17: f64 = (((p.p6 * s.db[41][17]) * s.v[94]) + (eq51_e915 * s.db[94][17]));
        let eq51_e917_d_b18: f64 = (((p.p6 * s.db[41][18]) * s.v[94]) + (eq51_e915 * s.db[94][18]));
        let eq51_e917_d_b19: f64 = (((p.p6 * s.db[41][19]) * s.v[94]) + (eq51_e915 * s.db[94][19]));
        let eq51_e917_d_b20: f64 = (((p.p6 * s.db[41][20]) * s.v[94]) + (eq51_e915 * s.db[94][20]));
        let eq51_e917_d_b21: f64 = (((p.p6 * s.db[41][21]) * s.v[94]) + (eq51_e915 * s.db[94][21]));
        let eq51_e917_d_b22: f64 = (((p.p6 * s.db[41][22]) * s.v[94]) + (eq51_e915 * s.db[94][22]));
        let eq51_e917_d_b23: f64 = (((p.p6 * s.db[41][23]) * s.v[94]) + (eq51_e915 * s.db[94][23]));
        let eq51_e917_d_b24: f64 = (((p.p6 * s.db[41][24]) * s.v[94]) + (eq51_e915 * s.db[94][24]));
        let eq51_e917_d_b25: f64 = (((p.p6 * s.db[41][25]) * s.v[94]) + (eq51_e915 * s.db[94][25]));
        let eq51_e917_d_b26: f64 = (((p.p6 * s.db[41][26]) * s.v[94]) + (eq51_e915 * s.db[94][26]));
        let eq51_e917_d_b27: f64 = (((p.p6 * s.db[41][27]) * s.v[94]) + (eq51_e915 * s.db[94][27]));
        let eq51_e917_d_b28: f64 = (((p.p6 * s.db[41][28]) * s.v[94]) + (eq51_e915 * s.db[94][28]));
        let eq51_e917_d_b29: f64 = (((p.p6 * s.db[41][29]) * s.v[94]) + (eq51_e915 * s.db[94][29]));
        let eq51_e917_d_b30: f64 = (((p.p6 * s.db[41][30]) * s.v[94]) + (eq51_e915 * s.db[94][30]));
        let eq51_e917_d_b31: f64 = (((p.p6 * s.db[41][31]) * s.v[94]) + (eq51_e915 * s.db[94][31]));
        let eq51_e917_d_b32: f64 = (((p.p6 * s.db[41][32]) * s.v[94]) + (eq51_e915 * s.db[94][32]));
        let eq51_e917_d_b33: f64 = (((p.p6 * s.db[41][33]) * s.v[94]) + (eq51_e915 * s.db[94][33]));
        let eq51_e917_d_b34: f64 = (((p.p6 * s.db[41][34]) * s.v[94]) + (eq51_e915 * s.db[94][34]));
        let eq51_e917_d_b35: f64 = (((p.p6 * s.db[41][35]) * s.v[94]) + (eq51_e915 * s.db[94][35]));
        let eq51_e917_d_b36: f64 = (((p.p6 * s.db[41][36]) * s.v[94]) + (eq51_e915 * s.db[94][36]));
        let eq51_e917_d_b37: f64 = (((p.p6 * s.db[41][37]) * s.v[94]) + (eq51_e915 * s.db[94][37]));
        let eq51_e917_d_b38: f64 = (((p.p6 * s.db[41][38]) * s.v[94]) + (eq51_e915 * s.db[94][38]));
        let eq51_e917_d_b39: f64 = (((p.p6 * s.db[41][39]) * s.v[94]) + (eq51_e915 * s.db[94][39]));
        let eq51_e917_d_b40: f64 = (((p.p6 * s.db[41][40]) * s.v[94]) + (eq51_e915 * s.db[94][40]));
        let eq51_e917_d_b41: f64 = (((p.p6 * s.db[41][41]) * s.v[94]) + (eq51_e915 * s.db[94][41]));
        let eq51_e917_d_b42: f64 = (((p.p6 * s.db[41][42]) * s.v[94]) + (eq51_e915 * s.db[94][42]));
        let eq51_e917_d_b43: f64 = (((p.p6 * s.db[41][43]) * s.v[94]) + (eq51_e915 * s.db[94][43]));
        let eq51_e917_d_b44: f64 = (((p.p6 * s.db[41][44]) * s.v[94]) + (eq51_e915 * s.db[94][44]));
        let eq51_e917_d_b45: f64 = (((p.p6 * s.db[41][45]) * s.v[94]) + (eq51_e915 * s.db[94][45]));
        let eq51_e917_d_b46: f64 = (((p.p6 * s.db[41][46]) * s.v[94]) + (eq51_e915 * s.db[94][46]));
        let eq51_e917_d_b47: f64 = (((p.p6 * s.db[41][47]) * s.v[94]) + (eq51_e915 * s.db[94][47]));
        let eq51_e917_d_b48: f64 = (((p.p6 * s.db[41][48]) * s.v[94]) + (eq51_e915 * s.db[94][48]));
        let eq51_e917_d_b49: f64 = (((p.p6 * s.db[41][49]) * s.v[94]) + (eq51_e915 * s.db[94][49]));
        let eq51_e917_d_b50: f64 = (((p.p6 * s.db[41][50]) * s.v[94]) + (eq51_e915 * s.db[94][50]));
        let eq51_e917_d_b51: f64 = (((p.p6 * s.db[41][51]) * s.v[94]) + (eq51_e915 * s.db[94][51]));
        let eq51_e917_d_b52: f64 = (((p.p6 * s.db[41][52]) * s.v[94]) + (eq51_e915 * s.db[94][52]));
        let eq51_e917_d_b53: f64 = (((p.p6 * s.db[41][53]) * s.v[94]) + (eq51_e915 * s.db[94][53]));
        let eq51_e917_d_b54: f64 = (((p.p6 * s.db[41][54]) * s.v[94]) + (eq51_e915 * s.db[94][54]));
        let eq51_e920: f64 = (p.p6 * s.v[379]);
        let eq51_e922: f64 = (eq51_e920 * (nv7 - nv8));
        let eq51_e922_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv7 - nv8));
        let eq51_e922_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv7 - nv8));
        let eq51_e922_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv7 - nv8));
        let eq51_e922_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv7 - nv8));
        let eq51_e922_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv7 - nv8));
        let eq51_e922_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv7 - nv8));
        let eq51_e922_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv7 - nv8));
        let eq51_e922_d_n7: f64 = (((p.p6 * s.dn[379][7]) * (nv7 - nv8)) + eq51_e920);
        let eq51_e922_d_n8: f64 = (((p.p6 * s.dn[379][8]) * (nv7 - nv8)) + (-eq51_e920));
        let eq51_e922_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv7 - nv8));
        let eq51_e922_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv7 - nv8));
        let eq51_e922_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv7 - nv8));
        let eq51_e922_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv7 - nv8));
        let eq51_e922_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv7 - nv8));
        let eq51_e922_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv7 - nv8));
        let eq51_e922_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv7 - nv8));
        let eq51_e922_d_n16: f64 = ((p.p6 * s.dn[379][16]) * (nv7 - nv8));
        let eq51_e922_d_n17: f64 = ((p.p6 * s.dn[379][17]) * (nv7 - nv8));
        let eq51_e922_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv7 - nv8));
        let eq51_e922_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv7 - nv8));
        let eq51_e922_d_n20: f64 = ((p.p6 * s.dn[379][20]) * (nv7 - nv8));
        let eq51_e922_d_n21: f64 = ((p.p6 * s.dn[379][21]) * (nv7 - nv8));
        let eq51_e922_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv7 - nv8));
        let eq51_e922_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv7 - nv8));
        let eq51_e922_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv7 - nv8));
        let eq51_e922_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv7 - nv8));
        let eq51_e922_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv7 - nv8));
        let eq51_e922_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv7 - nv8));
        let eq51_e922_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv7 - nv8));
        let eq51_e922_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv7 - nv8));
        let eq51_e922_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv7 - nv8));
        let eq51_e922_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv7 - nv8));
        let eq51_e922_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv7 - nv8));
        let eq51_e922_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv7 - nv8));
        let eq51_e922_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv7 - nv8));
        let eq51_e922_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv7 - nv8));
        let eq51_e922_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv7 - nv8));
        let eq51_e922_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv7 - nv8));
        let eq51_e922_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv7 - nv8));
        let eq51_e922_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv7 - nv8));
        let eq51_e922_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv7 - nv8));
        let eq51_e922_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv7 - nv8));
        let eq51_e922_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv7 - nv8));
        let eq51_e922_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv7 - nv8));
        let eq51_e922_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv7 - nv8));
        let eq51_e922_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv7 - nv8));
        let eq51_e922_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv7 - nv8));
        let eq51_e922_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv7 - nv8));
        let eq51_e922_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv7 - nv8));
        let eq51_e922_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv7 - nv8));
        let eq51_e922_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv7 - nv8));
        let eq51_e922_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv7 - nv8));
        let eq51_e922_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv7 - nv8));
        let eq51_e922_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv7 - nv8));
        let eq51_e922_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv7 - nv8));
        let eq51_e922_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv7 - nv8));
        let eq51_e922_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv7 - nv8));
        let eq51_e922_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv7 - nv8));
        let eq51_e922_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv7 - nv8));
        let eq51_e922_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv7 - nv8));
        let eq51_e922_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv7 - nv8));
        let eq51_e922_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv7 - nv8));
        let eq51_e922_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv7 - nv8));
        let eq51_e922_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv7 - nv8));
        let eq51_e922_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv7 - nv8));
        let eq51_e922_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv7 - nv8));
        let eq51_e922_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv7 - nv8));
        let eq51_e922_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv7 - nv8));
        let eq51_e922_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv7 - nv8));
        let eq51_e922_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv7 - nv8));
        let eq51_e922_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv7 - nv8));
        let eq51_e922_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv7 - nv8));
        let eq51_e922_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv7 - nv8));
        let eq51_e922_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv7 - nv8));
        let eq51_e922_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv7 - nv8));
        let eq51_e922_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv7 - nv8));
        let eq51_e922_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv7 - nv8));
        let eq51_e922_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv7 - nv8));
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

    pub(super) fn stamp_transient_equations_block_5(
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let eq52_e926: f64 = (p.p6 * s.v[41]);
        let eq52_e929: f64 = (p.p4 * p.p5);
        let eq52_e931: f64 = (eq52_e929 * s.v[332]);
        let eq52_e932: f64 = (eq52_e926 * eq52_e931);
        let eq52_e932_d_n0: f64 = (((p.p6 * s.dn[41][0]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][0])));
        let eq52_e932_d_n1: f64 = (((p.p6 * s.dn[41][1]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][1])));
        let eq52_e932_d_n2: f64 = (((p.p6 * s.dn[41][2]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][2])));
        let eq52_e932_d_n3: f64 = (((p.p6 * s.dn[41][3]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][3])));
        let eq52_e932_d_n4: f64 = (((p.p6 * s.dn[41][4]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][4])));
        let eq52_e932_d_n5: f64 = (((p.p6 * s.dn[41][5]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][5])));
        let eq52_e932_d_n6: f64 = (((p.p6 * s.dn[41][6]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][6])));
        let eq52_e932_d_n7: f64 = (((p.p6 * s.dn[41][7]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][7])));
        let eq52_e932_d_n8: f64 = (((p.p6 * s.dn[41][8]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][8])));
        let eq52_e932_d_n9: f64 = (((p.p6 * s.dn[41][9]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][9])));
        let eq52_e932_d_n10: f64 = (((p.p6 * s.dn[41][10]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][10])));
        let eq52_e932_d_n11: f64 = (((p.p6 * s.dn[41][11]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][11])));
        let eq52_e932_d_n12: f64 = (((p.p6 * s.dn[41][12]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][12])));
        let eq52_e932_d_n13: f64 = (((p.p6 * s.dn[41][13]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][13])));
        let eq52_e932_d_n14: f64 = (((p.p6 * s.dn[41][14]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][14])));
        let eq52_e932_d_n15: f64 = (((p.p6 * s.dn[41][15]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][15])));
        let eq52_e932_d_n16: f64 = (((p.p6 * s.dn[41][16]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][16])));
        let eq52_e932_d_n17: f64 = (((p.p6 * s.dn[41][17]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][17])));
        let eq52_e932_d_n18: f64 = (((p.p6 * s.dn[41][18]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][18])));
        let eq52_e932_d_n19: f64 = (((p.p6 * s.dn[41][19]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][19])));
        let eq52_e932_d_n20: f64 = (((p.p6 * s.dn[41][20]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][20])));
        let eq52_e932_d_n21: f64 = (((p.p6 * s.dn[41][21]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][21])));
        let eq52_e932_d_n22: f64 = (((p.p6 * s.dn[41][22]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.dn[332][22])));
        let eq52_e932_d_b0: f64 = (((p.p6 * s.db[41][0]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][0])));
        let eq52_e932_d_b1: f64 = (((p.p6 * s.db[41][1]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][1])));
        let eq52_e932_d_b2: f64 = (((p.p6 * s.db[41][2]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][2])));
        let eq52_e932_d_b3: f64 = (((p.p6 * s.db[41][3]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][3])));
        let eq52_e932_d_b4: f64 = (((p.p6 * s.db[41][4]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][4])));
        let eq52_e932_d_b5: f64 = (((p.p6 * s.db[41][5]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][5])));
        let eq52_e932_d_b6: f64 = (((p.p6 * s.db[41][6]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][6])));
        let eq52_e932_d_b7: f64 = (((p.p6 * s.db[41][7]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][7])));
        let eq52_e932_d_b8: f64 = (((p.p6 * s.db[41][8]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][8])));
        let eq52_e932_d_b9: f64 = (((p.p6 * s.db[41][9]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][9])));
        let eq52_e932_d_b10: f64 = (((p.p6 * s.db[41][10]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][10])));
        let eq52_e932_d_b11: f64 = (((p.p6 * s.db[41][11]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][11])));
        let eq52_e932_d_b12: f64 = (((p.p6 * s.db[41][12]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][12])));
        let eq52_e932_d_b13: f64 = (((p.p6 * s.db[41][13]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][13])));
        let eq52_e932_d_b14: f64 = (((p.p6 * s.db[41][14]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][14])));
        let eq52_e932_d_b15: f64 = (((p.p6 * s.db[41][15]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][15])));
        let eq52_e932_d_b16: f64 = (((p.p6 * s.db[41][16]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][16])));
        let eq52_e932_d_b17: f64 = (((p.p6 * s.db[41][17]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][17])));
        let eq52_e932_d_b18: f64 = (((p.p6 * s.db[41][18]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][18])));
        let eq52_e932_d_b19: f64 = (((p.p6 * s.db[41][19]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][19])));
        let eq52_e932_d_b20: f64 = (((p.p6 * s.db[41][20]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][20])));
        let eq52_e932_d_b21: f64 = (((p.p6 * s.db[41][21]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][21])));
        let eq52_e932_d_b22: f64 = (((p.p6 * s.db[41][22]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][22])));
        let eq52_e932_d_b23: f64 = (((p.p6 * s.db[41][23]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][23])));
        let eq52_e932_d_b24: f64 = (((p.p6 * s.db[41][24]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][24])));
        let eq52_e932_d_b25: f64 = (((p.p6 * s.db[41][25]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][25])));
        let eq52_e932_d_b26: f64 = (((p.p6 * s.db[41][26]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][26])));
        let eq52_e932_d_b27: f64 = (((p.p6 * s.db[41][27]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][27])));
        let eq52_e932_d_b28: f64 = (((p.p6 * s.db[41][28]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][28])));
        let eq52_e932_d_b29: f64 = (((p.p6 * s.db[41][29]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][29])));
        let eq52_e932_d_b30: f64 = (((p.p6 * s.db[41][30]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][30])));
        let eq52_e932_d_b31: f64 = (((p.p6 * s.db[41][31]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][31])));
        let eq52_e932_d_b32: f64 = (((p.p6 * s.db[41][32]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][32])));
        let eq52_e932_d_b33: f64 = (((p.p6 * s.db[41][33]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][33])));
        let eq52_e932_d_b34: f64 = (((p.p6 * s.db[41][34]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][34])));
        let eq52_e932_d_b35: f64 = (((p.p6 * s.db[41][35]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][35])));
        let eq52_e932_d_b36: f64 = (((p.p6 * s.db[41][36]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][36])));
        let eq52_e932_d_b37: f64 = (((p.p6 * s.db[41][37]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][37])));
        let eq52_e932_d_b38: f64 = (((p.p6 * s.db[41][38]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][38])));
        let eq52_e932_d_b39: f64 = (((p.p6 * s.db[41][39]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][39])));
        let eq52_e932_d_b40: f64 = (((p.p6 * s.db[41][40]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][40])));
        let eq52_e932_d_b41: f64 = (((p.p6 * s.db[41][41]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][41])));
        let eq52_e932_d_b42: f64 = (((p.p6 * s.db[41][42]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][42])));
        let eq52_e932_d_b43: f64 = (((p.p6 * s.db[41][43]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][43])));
        let eq52_e932_d_b44: f64 = (((p.p6 * s.db[41][44]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][44])));
        let eq52_e932_d_b45: f64 = (((p.p6 * s.db[41][45]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][45])));
        let eq52_e932_d_b46: f64 = (((p.p6 * s.db[41][46]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][46])));
        let eq52_e932_d_b47: f64 = (((p.p6 * s.db[41][47]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][47])));
        let eq52_e932_d_b48: f64 = (((p.p6 * s.db[41][48]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][48])));
        let eq52_e932_d_b49: f64 = (((p.p6 * s.db[41][49]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][49])));
        let eq52_e932_d_b50: f64 = (((p.p6 * s.db[41][50]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][50])));
        let eq52_e932_d_b51: f64 = (((p.p6 * s.db[41][51]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][51])));
        let eq52_e932_d_b52: f64 = (((p.p6 * s.db[41][52]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][52])));
        let eq52_e932_d_b53: f64 = (((p.p6 * s.db[41][53]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][53])));
        let eq52_e932_d_b54: f64 = (((p.p6 * s.db[41][54]) * eq52_e931) + (eq52_e926 * (eq52_e929 * s.db[332][54])));
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
        (eq53_e936, (p.p6 * s.dn[206][0]), (p.p6 * s.dn[206][1]), (p.p6 * s.dn[206][2]), (p.p6 * s.dn[206][3]), (p.p6 * s.dn[206][4]), (p.p6 * s.dn[206][5]), (p.p6 * s.dn[206][6]), (p.p6 * s.dn[206][7]), (p.p6 * s.dn[206][8]), (p.p6 * s.dn[206][9]), (p.p6 * s.dn[206][10]), (p.p6 * s.dn[206][11]), (p.p6 * s.dn[206][12]), (p.p6 * s.dn[206][13]), (p.p6 * s.dn[206][14]), (p.p6 * s.dn[206][15]), (p.p6 * s.dn[206][16]), (p.p6 * s.dn[206][17]), (p.p6 * s.dn[206][18]), (p.p6 * s.dn[206][19]), (p.p6 * s.dn[206][20]), (p.p6 * s.dn[206][21]), (p.p6 * s.dn[206][22]), (p.p6 * s.db[206][0]), (p.p6 * s.db[206][1]), (p.p6 * s.db[206][2]), (p.p6 * s.db[206][3]), (p.p6 * s.db[206][4]), (p.p6 * s.db[206][5]), (p.p6 * s.db[206][6]), (p.p6 * s.db[206][7]), (p.p6 * s.db[206][8]), (p.p6 * s.db[206][9]), (p.p6 * s.db[206][10]), (p.p6 * s.db[206][11]), (p.p6 * s.db[206][12]), (p.p6 * s.db[206][13]), (p.p6 * s.db[206][14]), (p.p6 * s.db[206][15]), (p.p6 * s.db[206][16]), (p.p6 * s.db[206][17]), (p.p6 * s.db[206][18]), (p.p6 * s.db[206][19]), (p.p6 * s.db[206][20]), (p.p6 * s.db[206][21]), (p.p6 * s.db[206][22]), (p.p6 * s.db[206][23]), (p.p6 * s.db[206][24]), (p.p6 * s.db[206][25]), (p.p6 * s.db[206][26]), (p.p6 * s.db[206][27]), (p.p6 * s.db[206][28]), (p.p6 * s.db[206][29]), (p.p6 * s.db[206][30]), (p.p6 * s.db[206][31]), (p.p6 * s.db[206][32]), (p.p6 * s.db[206][33]), (p.p6 * s.db[206][34]), (p.p6 * s.db[206][35]), (p.p6 * s.db[206][36]), (p.p6 * s.db[206][37]), (p.p6 * s.db[206][38]), (p.p6 * s.db[206][39]), (p.p6 * s.db[206][40]), (p.p6 * s.db[206][41]), (p.p6 * s.db[206][42]), (p.p6 * s.db[206][43]), (p.p6 * s.db[206][44]), (p.p6 * s.db[206][45]), (p.p6 * s.db[206][46]), (p.p6 * s.db[206][47]), (p.p6 * s.db[206][48]), (p.p6 * s.db[206][49]), (p.p6 * s.db[206][50]), (p.p6 * s.db[206][51]), (p.p6 * s.db[206][52]), (p.p6 * s.db[206][53]), (p.p6 * s.db[206][54]),)
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
        (eq54_e942, (p.p6 * s.dn[207][0]), (p.p6 * s.dn[207][1]), (p.p6 * s.dn[207][2]), (p.p6 * s.dn[207][3]), (p.p6 * s.dn[207][4]), (p.p6 * s.dn[207][5]), (p.p6 * s.dn[207][6]), (p.p6 * s.dn[207][7]), (p.p6 * s.dn[207][8]), (p.p6 * s.dn[207][9]), (p.p6 * s.dn[207][10]), (p.p6 * s.dn[207][11]), (p.p6 * s.dn[207][12]), (p.p6 * s.dn[207][13]), (p.p6 * s.dn[207][14]), (p.p6 * s.dn[207][15]), (p.p6 * s.dn[207][16]), (p.p6 * s.dn[207][17]), (p.p6 * s.dn[207][18]), (p.p6 * s.dn[207][19]), (p.p6 * s.dn[207][20]), (p.p6 * s.dn[207][21]), (p.p6 * s.dn[207][22]), (p.p6 * s.db[207][0]), (p.p6 * s.db[207][1]), (p.p6 * s.db[207][2]), (p.p6 * s.db[207][3]), (p.p6 * s.db[207][4]), (p.p6 * s.db[207][5]), (p.p6 * s.db[207][6]), (p.p6 * s.db[207][7]), (p.p6 * s.db[207][8]), (p.p6 * s.db[207][9]), (p.p6 * s.db[207][10]), (p.p6 * s.db[207][11]), (p.p6 * s.db[207][12]), (p.p6 * s.db[207][13]), (p.p6 * s.db[207][14]), (p.p6 * s.db[207][15]), (p.p6 * s.db[207][16]), (p.p6 * s.db[207][17]), (p.p6 * s.db[207][18]), (p.p6 * s.db[207][19]), (p.p6 * s.db[207][20]), (p.p6 * s.db[207][21]), (p.p6 * s.db[207][22]), (p.p6 * s.db[207][23]), (p.p6 * s.db[207][24]), (p.p6 * s.db[207][25]), (p.p6 * s.db[207][26]), (p.p6 * s.db[207][27]), (p.p6 * s.db[207][28]), (p.p6 * s.db[207][29]), (p.p6 * s.db[207][30]), (p.p6 * s.db[207][31]), (p.p6 * s.db[207][32]), (p.p6 * s.db[207][33]), (p.p6 * s.db[207][34]), (p.p6 * s.db[207][35]), (p.p6 * s.db[207][36]), (p.p6 * s.db[207][37]), (p.p6 * s.db[207][38]), (p.p6 * s.db[207][39]), (p.p6 * s.db[207][40]), (p.p6 * s.db[207][41]), (p.p6 * s.db[207][42]), (p.p6 * s.db[207][43]), (p.p6 * s.db[207][44]), (p.p6 * s.db[207][45]), (p.p6 * s.db[207][46]), (p.p6 * s.db[207][47]), (p.p6 * s.db[207][48]), (p.p6 * s.db[207][49]), (p.p6 * s.db[207][50]), (p.p6 * s.db[207][51]), (p.p6 * s.db[207][52]), (p.p6 * s.db[207][53]), (p.p6 * s.db[207][54]),)
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
        let (eq55_e957, eq55_e957_d_n0, eq55_e957_d_n1, eq55_e957_d_n2, eq55_e957_d_n3, eq55_e957_d_n4, eq55_e957_d_n5, eq55_e957_d_n6, eq55_e957_d_n7, eq55_e957_d_n8, eq55_e957_d_n9, eq55_e957_d_n10, eq55_e957_d_n11, eq55_e957_d_n12, eq55_e957_d_n13, eq55_e957_d_n14, eq55_e957_d_n15, eq55_e957_d_n16, eq55_e957_d_n17, eq55_e957_d_n18, eq55_e957_d_n19, eq55_e957_d_n20, eq55_e957_d_n21, eq55_e957_d_n22, eq55_e957_d_b0, eq55_e957_d_b1, eq55_e957_d_b2, eq55_e957_d_b3, eq55_e957_d_b4, eq55_e957_d_b5, eq55_e957_d_b6, eq55_e957_d_b7, eq55_e957_d_b8, eq55_e957_d_b9, eq55_e957_d_b10, eq55_e957_d_b11, eq55_e957_d_b12, eq55_e957_d_b13, eq55_e957_d_b14, eq55_e957_d_b15, eq55_e957_d_b16, eq55_e957_d_b17, eq55_e957_d_b18, eq55_e957_d_b19, eq55_e957_d_b20, eq55_e957_d_b21, eq55_e957_d_b22, eq55_e957_d_b23, eq55_e957_d_b24, eq55_e957_d_b25, eq55_e957_d_b26, eq55_e957_d_b27, eq55_e957_d_b28, eq55_e957_d_b29, eq55_e957_d_b30, eq55_e957_d_b31, eq55_e957_d_b32, eq55_e957_d_b33, eq55_e957_d_b34, eq55_e957_d_b35, eq55_e957_d_b36, eq55_e957_d_b37, eq55_e957_d_b38, eq55_e957_d_b39, eq55_e957_d_b40, eq55_e957_d_b41, eq55_e957_d_b42, eq55_e957_d_b43, eq55_e957_d_b44, eq55_e957_d_b45, eq55_e957_d_b46, eq55_e957_d_b47, eq55_e957_d_b48, eq55_e957_d_b49, eq55_e957_d_b50, eq55_e957_d_b51, eq55_e957_d_b52, eq55_e957_d_b53, eq55_e957_d_b54,) = {
    if (!s.b[423]) {
        let eq55_e951: f64 = 0.0;
        let eq55_e953: f64 = (eq55_e951 * (nv9 - nv8));
        let eq55_e954: f64 = (s.v[206] + eq55_e953);
        let eq55_e954_d_n8: f64 = (s.dn[206][8] + (-eq55_e951));
        let eq55_e954_d_n9: f64 = (s.dn[206][9] + eq55_e951);
        let eq55_e955: f64 = (p.p6 * eq55_e954);
        let eq55_e955_d_n8: f64 = (p.p6 * eq55_e954_d_n8);
        let eq55_e955_d_n9: f64 = (p.p6 * eq55_e954_d_n9);
        (eq55_e955, (p.p6 * s.dn[206][0]), (p.p6 * s.dn[206][1]), (p.p6 * s.dn[206][2]), (p.p6 * s.dn[206][3]), (p.p6 * s.dn[206][4]), (p.p6 * s.dn[206][5]), (p.p6 * s.dn[206][6]), (p.p6 * s.dn[206][7]), eq55_e955_d_n8, eq55_e955_d_n9, (p.p6 * s.dn[206][10]), (p.p6 * s.dn[206][11]), (p.p6 * s.dn[206][12]), (p.p6 * s.dn[206][13]), (p.p6 * s.dn[206][14]), (p.p6 * s.dn[206][15]), (p.p6 * s.dn[206][16]), (p.p6 * s.dn[206][17]), (p.p6 * s.dn[206][18]), (p.p6 * s.dn[206][19]), (p.p6 * s.dn[206][20]), (p.p6 * s.dn[206][21]), (p.p6 * s.dn[206][22]), (p.p6 * s.db[206][0]), (p.p6 * s.db[206][1]), (p.p6 * s.db[206][2]), (p.p6 * s.db[206][3]), (p.p6 * s.db[206][4]), (p.p6 * s.db[206][5]), (p.p6 * s.db[206][6]), (p.p6 * s.db[206][7]), (p.p6 * s.db[206][8]), (p.p6 * s.db[206][9]), (p.p6 * s.db[206][10]), (p.p6 * s.db[206][11]), (p.p6 * s.db[206][12]), (p.p6 * s.db[206][13]), (p.p6 * s.db[206][14]), (p.p6 * s.db[206][15]), (p.p6 * s.db[206][16]), (p.p6 * s.db[206][17]), (p.p6 * s.db[206][18]), (p.p6 * s.db[206][19]), (p.p6 * s.db[206][20]), (p.p6 * s.db[206][21]), (p.p6 * s.db[206][22]), (p.p6 * s.db[206][23]), (p.p6 * s.db[206][24]), (p.p6 * s.db[206][25]), (p.p6 * s.db[206][26]), (p.p6 * s.db[206][27]), (p.p6 * s.db[206][28]), (p.p6 * s.db[206][29]), (p.p6 * s.db[206][30]), (p.p6 * s.db[206][31]), (p.p6 * s.db[206][32]), (p.p6 * s.db[206][33]), (p.p6 * s.db[206][34]), (p.p6 * s.db[206][35]), (p.p6 * s.db[206][36]), (p.p6 * s.db[206][37]), (p.p6 * s.db[206][38]), (p.p6 * s.db[206][39]), (p.p6 * s.db[206][40]), (p.p6 * s.db[206][41]), (p.p6 * s.db[206][42]), (p.p6 * s.db[206][43]), (p.p6 * s.db[206][44]), (p.p6 * s.db[206][45]), (p.p6 * s.db[206][46]), (p.p6 * s.db[206][47]), (p.p6 * s.db[206][48]), (p.p6 * s.db[206][49]), (p.p6 * s.db[206][50]), (p.p6 * s.db[206][51]), (p.p6 * s.db[206][52]), (p.p6 * s.db[206][53]), (p.p6 * s.db[206][54]),)
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
        let eq56_e967: f64 = (s.v[207] + eq56_e966);
        let eq56_e967_d_n7: f64 = (s.dn[207][7] + (-eq56_e964));
        let eq56_e967_d_n9: f64 = (s.dn[207][9] + eq56_e964);
        let eq56_e968: f64 = (p.p6 * eq56_e967);
        let eq56_e968_d_n7: f64 = (p.p6 * eq56_e967_d_n7);
        let eq56_e968_d_n9: f64 = (p.p6 * eq56_e967_d_n9);
        (eq56_e968, (p.p6 * s.dn[207][0]), (p.p6 * s.dn[207][1]), (p.p6 * s.dn[207][2]), (p.p6 * s.dn[207][3]), (p.p6 * s.dn[207][4]), (p.p6 * s.dn[207][5]), (p.p6 * s.dn[207][6]), eq56_e968_d_n7, (p.p6 * s.dn[207][8]), eq56_e968_d_n9, (p.p6 * s.dn[207][10]), (p.p6 * s.dn[207][11]), (p.p6 * s.dn[207][12]), (p.p6 * s.dn[207][13]), (p.p6 * s.dn[207][14]), (p.p6 * s.dn[207][15]), (p.p6 * s.dn[207][16]), (p.p6 * s.dn[207][17]), (p.p6 * s.dn[207][18]), (p.p6 * s.dn[207][19]), (p.p6 * s.dn[207][20]), (p.p6 * s.dn[207][21]), (p.p6 * s.dn[207][22]), (p.p6 * s.db[207][0]), (p.p6 * s.db[207][1]), (p.p6 * s.db[207][2]), (p.p6 * s.db[207][3]), (p.p6 * s.db[207][4]), (p.p6 * s.db[207][5]), (p.p6 * s.db[207][6]), (p.p6 * s.db[207][7]), (p.p6 * s.db[207][8]), (p.p6 * s.db[207][9]), (p.p6 * s.db[207][10]), (p.p6 * s.db[207][11]), (p.p6 * s.db[207][12]), (p.p6 * s.db[207][13]), (p.p6 * s.db[207][14]), (p.p6 * s.db[207][15]), (p.p6 * s.db[207][16]), (p.p6 * s.db[207][17]), (p.p6 * s.db[207][18]), (p.p6 * s.db[207][19]), (p.p6 * s.db[207][20]), (p.p6 * s.db[207][21]), (p.p6 * s.db[207][22]), (p.p6 * s.db[207][23]), (p.p6 * s.db[207][24]), (p.p6 * s.db[207][25]), (p.p6 * s.db[207][26]), (p.p6 * s.db[207][27]), (p.p6 * s.db[207][28]), (p.p6 * s.db[207][29]), (p.p6 * s.db[207][30]), (p.p6 * s.db[207][31]), (p.p6 * s.db[207][32]), (p.p6 * s.db[207][33]), (p.p6 * s.db[207][34]), (p.p6 * s.db[207][35]), (p.p6 * s.db[207][36]), (p.p6 * s.db[207][37]), (p.p6 * s.db[207][38]), (p.p6 * s.db[207][39]), (p.p6 * s.db[207][40]), (p.p6 * s.db[207][41]), (p.p6 * s.db[207][42]), (p.p6 * s.db[207][43]), (p.p6 * s.db[207][44]), (p.p6 * s.db[207][45]), (p.p6 * s.db[207][46]), (p.p6 * s.db[207][47]), (p.p6 * s.db[207][48]), (p.p6 * s.db[207][49]), (p.p6 * s.db[207][50]), (p.p6 * s.db[207][51]), (p.p6 * s.db[207][52]), (p.p6 * s.db[207][53]), (p.p6 * s.db[207][54]),)
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
        let eq57_e978: f64 = (eq57_e976 * (nv0 - nv18));
        let eq57_e978_d_n0: f64 = (((p.p6 * s.dn[142][0]) * (nv0 - nv18)) + eq57_e976);
        let eq57_e978_d_n1: f64 = ((p.p6 * s.dn[142][1]) * (nv0 - nv18));
        let eq57_e978_d_n2: f64 = ((p.p6 * s.dn[142][2]) * (nv0 - nv18));
        let eq57_e978_d_n3: f64 = ((p.p6 * s.dn[142][3]) * (nv0 - nv18));
        let eq57_e978_d_n4: f64 = ((p.p6 * s.dn[142][4]) * (nv0 - nv18));
        let eq57_e978_d_n5: f64 = ((p.p6 * s.dn[142][5]) * (nv0 - nv18));
        let eq57_e978_d_n6: f64 = ((p.p6 * s.dn[142][6]) * (nv0 - nv18));
        let eq57_e978_d_n7: f64 = ((p.p6 * s.dn[142][7]) * (nv0 - nv18));
        let eq57_e978_d_n8: f64 = ((p.p6 * s.dn[142][8]) * (nv0 - nv18));
        let eq57_e978_d_n9: f64 = ((p.p6 * s.dn[142][9]) * (nv0 - nv18));
        let eq57_e978_d_n10: f64 = ((p.p6 * s.dn[142][10]) * (nv0 - nv18));
        let eq57_e978_d_n11: f64 = ((p.p6 * s.dn[142][11]) * (nv0 - nv18));
        let eq57_e978_d_n12: f64 = ((p.p6 * s.dn[142][12]) * (nv0 - nv18));
        let eq57_e978_d_n13: f64 = ((p.p6 * s.dn[142][13]) * (nv0 - nv18));
        let eq57_e978_d_n14: f64 = ((p.p6 * s.dn[142][14]) * (nv0 - nv18));
        let eq57_e978_d_n15: f64 = ((p.p6 * s.dn[142][15]) * (nv0 - nv18));
        let eq57_e978_d_n16: f64 = ((p.p6 * s.dn[142][16]) * (nv0 - nv18));
        let eq57_e978_d_n17: f64 = ((p.p6 * s.dn[142][17]) * (nv0 - nv18));
        let eq57_e978_d_n18: f64 = (((p.p6 * s.dn[142][18]) * (nv0 - nv18)) + (-eq57_e976));
        let eq57_e978_d_n19: f64 = ((p.p6 * s.dn[142][19]) * (nv0 - nv18));
        let eq57_e978_d_n20: f64 = ((p.p6 * s.dn[142][20]) * (nv0 - nv18));
        let eq57_e978_d_n21: f64 = ((p.p6 * s.dn[142][21]) * (nv0 - nv18));
        let eq57_e978_d_n22: f64 = ((p.p6 * s.dn[142][22]) * (nv0 - nv18));
        let eq57_e978_d_b0: f64 = ((p.p6 * s.db[142][0]) * (nv0 - nv18));
        let eq57_e978_d_b1: f64 = ((p.p6 * s.db[142][1]) * (nv0 - nv18));
        let eq57_e978_d_b2: f64 = ((p.p6 * s.db[142][2]) * (nv0 - nv18));
        let eq57_e978_d_b3: f64 = ((p.p6 * s.db[142][3]) * (nv0 - nv18));
        let eq57_e978_d_b4: f64 = ((p.p6 * s.db[142][4]) * (nv0 - nv18));
        let eq57_e978_d_b5: f64 = ((p.p6 * s.db[142][5]) * (nv0 - nv18));
        let eq57_e978_d_b6: f64 = ((p.p6 * s.db[142][6]) * (nv0 - nv18));
        let eq57_e978_d_b7: f64 = ((p.p6 * s.db[142][7]) * (nv0 - nv18));
        let eq57_e978_d_b8: f64 = ((p.p6 * s.db[142][8]) * (nv0 - nv18));
        let eq57_e978_d_b9: f64 = ((p.p6 * s.db[142][9]) * (nv0 - nv18));
        let eq57_e978_d_b10: f64 = ((p.p6 * s.db[142][10]) * (nv0 - nv18));
        let eq57_e978_d_b11: f64 = ((p.p6 * s.db[142][11]) * (nv0 - nv18));
        let eq57_e978_d_b12: f64 = ((p.p6 * s.db[142][12]) * (nv0 - nv18));
        let eq57_e978_d_b13: f64 = ((p.p6 * s.db[142][13]) * (nv0 - nv18));
        let eq57_e978_d_b14: f64 = ((p.p6 * s.db[142][14]) * (nv0 - nv18));
        let eq57_e978_d_b15: f64 = ((p.p6 * s.db[142][15]) * (nv0 - nv18));
        let eq57_e978_d_b16: f64 = ((p.p6 * s.db[142][16]) * (nv0 - nv18));
        let eq57_e978_d_b17: f64 = ((p.p6 * s.db[142][17]) * (nv0 - nv18));
        let eq57_e978_d_b18: f64 = ((p.p6 * s.db[142][18]) * (nv0 - nv18));
        let eq57_e978_d_b19: f64 = ((p.p6 * s.db[142][19]) * (nv0 - nv18));
        let eq57_e978_d_b20: f64 = ((p.p6 * s.db[142][20]) * (nv0 - nv18));
        let eq57_e978_d_b21: f64 = ((p.p6 * s.db[142][21]) * (nv0 - nv18));
        let eq57_e978_d_b22: f64 = ((p.p6 * s.db[142][22]) * (nv0 - nv18));
        let eq57_e978_d_b23: f64 = ((p.p6 * s.db[142][23]) * (nv0 - nv18));
        let eq57_e978_d_b24: f64 = ((p.p6 * s.db[142][24]) * (nv0 - nv18));
        let eq57_e978_d_b25: f64 = ((p.p6 * s.db[142][25]) * (nv0 - nv18));
        let eq57_e978_d_b26: f64 = ((p.p6 * s.db[142][26]) * (nv0 - nv18));
        let eq57_e978_d_b27: f64 = ((p.p6 * s.db[142][27]) * (nv0 - nv18));
        let eq57_e978_d_b28: f64 = ((p.p6 * s.db[142][28]) * (nv0 - nv18));
        let eq57_e978_d_b29: f64 = ((p.p6 * s.db[142][29]) * (nv0 - nv18));
        let eq57_e978_d_b30: f64 = ((p.p6 * s.db[142][30]) * (nv0 - nv18));
        let eq57_e978_d_b31: f64 = ((p.p6 * s.db[142][31]) * (nv0 - nv18));
        let eq57_e978_d_b32: f64 = ((p.p6 * s.db[142][32]) * (nv0 - nv18));
        let eq57_e978_d_b33: f64 = ((p.p6 * s.db[142][33]) * (nv0 - nv18));
        let eq57_e978_d_b34: f64 = ((p.p6 * s.db[142][34]) * (nv0 - nv18));
        let eq57_e978_d_b35: f64 = ((p.p6 * s.db[142][35]) * (nv0 - nv18));
        let eq57_e978_d_b36: f64 = ((p.p6 * s.db[142][36]) * (nv0 - nv18));
        let eq57_e978_d_b37: f64 = ((p.p6 * s.db[142][37]) * (nv0 - nv18));
        let eq57_e978_d_b38: f64 = ((p.p6 * s.db[142][38]) * (nv0 - nv18));
        let eq57_e978_d_b39: f64 = ((p.p6 * s.db[142][39]) * (nv0 - nv18));
        let eq57_e978_d_b40: f64 = ((p.p6 * s.db[142][40]) * (nv0 - nv18));
        let eq57_e978_d_b41: f64 = ((p.p6 * s.db[142][41]) * (nv0 - nv18));
        let eq57_e978_d_b42: f64 = ((p.p6 * s.db[142][42]) * (nv0 - nv18));
        let eq57_e978_d_b43: f64 = ((p.p6 * s.db[142][43]) * (nv0 - nv18));
        let eq57_e978_d_b44: f64 = ((p.p6 * s.db[142][44]) * (nv0 - nv18));
        let eq57_e978_d_b45: f64 = ((p.p6 * s.db[142][45]) * (nv0 - nv18));
        let eq57_e978_d_b46: f64 = ((p.p6 * s.db[142][46]) * (nv0 - nv18));
        let eq57_e978_d_b47: f64 = ((p.p6 * s.db[142][47]) * (nv0 - nv18));
        let eq57_e978_d_b48: f64 = ((p.p6 * s.db[142][48]) * (nv0 - nv18));
        let eq57_e978_d_b49: f64 = ((p.p6 * s.db[142][49]) * (nv0 - nv18));
        let eq57_e978_d_b50: f64 = ((p.p6 * s.db[142][50]) * (nv0 - nv18));
        let eq57_e978_d_b51: f64 = ((p.p6 * s.db[142][51]) * (nv0 - nv18));
        let eq57_e978_d_b52: f64 = ((p.p6 * s.db[142][52]) * (nv0 - nv18));
        let eq57_e978_d_b53: f64 = ((p.p6 * s.db[142][53]) * (nv0 - nv18));
        let eq57_e978_d_b54: f64 = ((p.p6 * s.db[142][54]) * (nv0 - nv18));
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
        let (eq58_e990, eq58_e990_d_n0, eq58_e990_d_n1, eq58_e990_d_n2, eq58_e990_d_n3, eq58_e990_d_n4, eq58_e990_d_n5, eq58_e990_d_n6, eq58_e990_d_n7, eq58_e990_d_n8, eq58_e990_d_n9, eq58_e990_d_n10, eq58_e990_d_n11, eq58_e990_d_n12, eq58_e990_d_n13, eq58_e990_d_n14, eq58_e990_d_n15, eq58_e990_d_n16, eq58_e990_d_n17, eq58_e990_d_n18, eq58_e990_d_n19, eq58_e990_d_n20, eq58_e990_d_n21, eq58_e990_d_n22, eq58_e990_d_b0, eq58_e990_d_b1, eq58_e990_d_b2, eq58_e990_d_b3, eq58_e990_d_b4, eq58_e990_d_b5, eq58_e990_d_b6, eq58_e990_d_b7, eq58_e990_d_b8, eq58_e990_d_b9, eq58_e990_d_b10, eq58_e990_d_b11, eq58_e990_d_b12, eq58_e990_d_b13, eq58_e990_d_b14, eq58_e990_d_b15, eq58_e990_d_b16, eq58_e990_d_b17, eq58_e990_d_b18, eq58_e990_d_b19, eq58_e990_d_b20, eq58_e990_d_b21, eq58_e990_d_b22, eq58_e990_d_b23, eq58_e990_d_b24, eq58_e990_d_b25, eq58_e990_d_b26, eq58_e990_d_b27, eq58_e990_d_b28, eq58_e990_d_b29, eq58_e990_d_b30, eq58_e990_d_b31, eq58_e990_d_b32, eq58_e990_d_b33, eq58_e990_d_b34, eq58_e990_d_b35, eq58_e990_d_b36, eq58_e990_d_b37, eq58_e990_d_b38, eq58_e990_d_b39, eq58_e990_d_b40, eq58_e990_d_b41, eq58_e990_d_b42, eq58_e990_d_b43, eq58_e990_d_b44, eq58_e990_d_b45, eq58_e990_d_b46, eq58_e990_d_b47, eq58_e990_d_b48, eq58_e990_d_b49, eq58_e990_d_b50, eq58_e990_d_b51, eq58_e990_d_b52, eq58_e990_d_b53, eq58_e990_d_b54,) = {
    if (s.b[424] && s.b[427]) {
        let eq58_e986: f64 = (p.p6 * s.v[143]);
        let eq58_e988: f64 = (eq58_e986 * (nv22 - nv2));
        let eq58_e988_d_n0: f64 = ((p.p6 * s.dn[143][0]) * (nv22 - nv2));
        let eq58_e988_d_n1: f64 = ((p.p6 * s.dn[143][1]) * (nv22 - nv2));
        let eq58_e988_d_n2: f64 = (((p.p6 * s.dn[143][2]) * (nv22 - nv2)) + (-eq58_e986));
        let eq58_e988_d_n3: f64 = ((p.p6 * s.dn[143][3]) * (nv22 - nv2));
        let eq58_e988_d_n4: f64 = ((p.p6 * s.dn[143][4]) * (nv22 - nv2));
        let eq58_e988_d_n5: f64 = ((p.p6 * s.dn[143][5]) * (nv22 - nv2));
        let eq58_e988_d_n6: f64 = ((p.p6 * s.dn[143][6]) * (nv22 - nv2));
        let eq58_e988_d_n7: f64 = ((p.p6 * s.dn[143][7]) * (nv22 - nv2));
        let eq58_e988_d_n8: f64 = ((p.p6 * s.dn[143][8]) * (nv22 - nv2));
        let eq58_e988_d_n9: f64 = ((p.p6 * s.dn[143][9]) * (nv22 - nv2));
        let eq58_e988_d_n10: f64 = ((p.p6 * s.dn[143][10]) * (nv22 - nv2));
        let eq58_e988_d_n11: f64 = ((p.p6 * s.dn[143][11]) * (nv22 - nv2));
        let eq58_e988_d_n12: f64 = ((p.p6 * s.dn[143][12]) * (nv22 - nv2));
        let eq58_e988_d_n13: f64 = ((p.p6 * s.dn[143][13]) * (nv22 - nv2));
        let eq58_e988_d_n14: f64 = ((p.p6 * s.dn[143][14]) * (nv22 - nv2));
        let eq58_e988_d_n15: f64 = ((p.p6 * s.dn[143][15]) * (nv22 - nv2));
        let eq58_e988_d_n16: f64 = ((p.p6 * s.dn[143][16]) * (nv22 - nv2));
        let eq58_e988_d_n17: f64 = ((p.p6 * s.dn[143][17]) * (nv22 - nv2));
        let eq58_e988_d_n18: f64 = ((p.p6 * s.dn[143][18]) * (nv22 - nv2));
        let eq58_e988_d_n19: f64 = ((p.p6 * s.dn[143][19]) * (nv22 - nv2));
        let eq58_e988_d_n20: f64 = ((p.p6 * s.dn[143][20]) * (nv22 - nv2));
        let eq58_e988_d_n21: f64 = ((p.p6 * s.dn[143][21]) * (nv22 - nv2));
        let eq58_e988_d_n22: f64 = (((p.p6 * s.dn[143][22]) * (nv22 - nv2)) + eq58_e986);
        let eq58_e988_d_b0: f64 = ((p.p6 * s.db[143][0]) * (nv22 - nv2));
        let eq58_e988_d_b1: f64 = ((p.p6 * s.db[143][1]) * (nv22 - nv2));
        let eq58_e988_d_b2: f64 = ((p.p6 * s.db[143][2]) * (nv22 - nv2));
        let eq58_e988_d_b3: f64 = ((p.p6 * s.db[143][3]) * (nv22 - nv2));
        let eq58_e988_d_b4: f64 = ((p.p6 * s.db[143][4]) * (nv22 - nv2));
        let eq58_e988_d_b5: f64 = ((p.p6 * s.db[143][5]) * (nv22 - nv2));
        let eq58_e988_d_b6: f64 = ((p.p6 * s.db[143][6]) * (nv22 - nv2));
        let eq58_e988_d_b7: f64 = ((p.p6 * s.db[143][7]) * (nv22 - nv2));
        let eq58_e988_d_b8: f64 = ((p.p6 * s.db[143][8]) * (nv22 - nv2));
        let eq58_e988_d_b9: f64 = ((p.p6 * s.db[143][9]) * (nv22 - nv2));
        let eq58_e988_d_b10: f64 = ((p.p6 * s.db[143][10]) * (nv22 - nv2));
        let eq58_e988_d_b11: f64 = ((p.p6 * s.db[143][11]) * (nv22 - nv2));
        let eq58_e988_d_b12: f64 = ((p.p6 * s.db[143][12]) * (nv22 - nv2));
        let eq58_e988_d_b13: f64 = ((p.p6 * s.db[143][13]) * (nv22 - nv2));
        let eq58_e988_d_b14: f64 = ((p.p6 * s.db[143][14]) * (nv22 - nv2));
        let eq58_e988_d_b15: f64 = ((p.p6 * s.db[143][15]) * (nv22 - nv2));
        let eq58_e988_d_b16: f64 = ((p.p6 * s.db[143][16]) * (nv22 - nv2));
        let eq58_e988_d_b17: f64 = ((p.p6 * s.db[143][17]) * (nv22 - nv2));
        let eq58_e988_d_b18: f64 = ((p.p6 * s.db[143][18]) * (nv22 - nv2));
        let eq58_e988_d_b19: f64 = ((p.p6 * s.db[143][19]) * (nv22 - nv2));
        let eq58_e988_d_b20: f64 = ((p.p6 * s.db[143][20]) * (nv22 - nv2));
        let eq58_e988_d_b21: f64 = ((p.p6 * s.db[143][21]) * (nv22 - nv2));
        let eq58_e988_d_b22: f64 = ((p.p6 * s.db[143][22]) * (nv22 - nv2));
        let eq58_e988_d_b23: f64 = ((p.p6 * s.db[143][23]) * (nv22 - nv2));
        let eq58_e988_d_b24: f64 = ((p.p6 * s.db[143][24]) * (nv22 - nv2));
        let eq58_e988_d_b25: f64 = ((p.p6 * s.db[143][25]) * (nv22 - nv2));
        let eq58_e988_d_b26: f64 = ((p.p6 * s.db[143][26]) * (nv22 - nv2));
        let eq58_e988_d_b27: f64 = ((p.p6 * s.db[143][27]) * (nv22 - nv2));
        let eq58_e988_d_b28: f64 = ((p.p6 * s.db[143][28]) * (nv22 - nv2));
        let eq58_e988_d_b29: f64 = ((p.p6 * s.db[143][29]) * (nv22 - nv2));
        let eq58_e988_d_b30: f64 = ((p.p6 * s.db[143][30]) * (nv22 - nv2));
        let eq58_e988_d_b31: f64 = ((p.p6 * s.db[143][31]) * (nv22 - nv2));
        let eq58_e988_d_b32: f64 = ((p.p6 * s.db[143][32]) * (nv22 - nv2));
        let eq58_e988_d_b33: f64 = ((p.p6 * s.db[143][33]) * (nv22 - nv2));
        let eq58_e988_d_b34: f64 = ((p.p6 * s.db[143][34]) * (nv22 - nv2));
        let eq58_e988_d_b35: f64 = ((p.p6 * s.db[143][35]) * (nv22 - nv2));
        let eq58_e988_d_b36: f64 = ((p.p6 * s.db[143][36]) * (nv22 - nv2));
        let eq58_e988_d_b37: f64 = ((p.p6 * s.db[143][37]) * (nv22 - nv2));
        let eq58_e988_d_b38: f64 = ((p.p6 * s.db[143][38]) * (nv22 - nv2));
        let eq58_e988_d_b39: f64 = ((p.p6 * s.db[143][39]) * (nv22 - nv2));
        let eq58_e988_d_b40: f64 = ((p.p6 * s.db[143][40]) * (nv22 - nv2));
        let eq58_e988_d_b41: f64 = ((p.p6 * s.db[143][41]) * (nv22 - nv2));
        let eq58_e988_d_b42: f64 = ((p.p6 * s.db[143][42]) * (nv22 - nv2));
        let eq58_e988_d_b43: f64 = ((p.p6 * s.db[143][43]) * (nv22 - nv2));
        let eq58_e988_d_b44: f64 = ((p.p6 * s.db[143][44]) * (nv22 - nv2));
        let eq58_e988_d_b45: f64 = ((p.p6 * s.db[143][45]) * (nv22 - nv2));
        let eq58_e988_d_b46: f64 = ((p.p6 * s.db[143][46]) * (nv22 - nv2));
        let eq58_e988_d_b47: f64 = ((p.p6 * s.db[143][47]) * (nv22 - nv2));
        let eq58_e988_d_b48: f64 = ((p.p6 * s.db[143][48]) * (nv22 - nv2));
        let eq58_e988_d_b49: f64 = ((p.p6 * s.db[143][49]) * (nv22 - nv2));
        let eq58_e988_d_b50: f64 = ((p.p6 * s.db[143][50]) * (nv22 - nv2));
        let eq58_e988_d_b51: f64 = ((p.p6 * s.db[143][51]) * (nv22 - nv2));
        let eq58_e988_d_b52: f64 = ((p.p6 * s.db[143][52]) * (nv22 - nv2));
        let eq58_e988_d_b53: f64 = ((p.p6 * s.db[143][53]) * (nv22 - nv2));
        let eq58_e988_d_b54: f64 = ((p.p6 * s.db[143][54]) * (nv22 - nv2));
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
        let eq59_e999: f64 = (eq59_e997 * (nv0 - nv7));
        let eq59_e999_d_n0: f64 = (((p.p6 * s.dn[142][0]) * (nv0 - nv7)) + eq59_e997);
        let eq59_e999_d_n1: f64 = ((p.p6 * s.dn[142][1]) * (nv0 - nv7));
        let eq59_e999_d_n2: f64 = ((p.p6 * s.dn[142][2]) * (nv0 - nv7));
        let eq59_e999_d_n3: f64 = ((p.p6 * s.dn[142][3]) * (nv0 - nv7));
        let eq59_e999_d_n4: f64 = ((p.p6 * s.dn[142][4]) * (nv0 - nv7));
        let eq59_e999_d_n5: f64 = ((p.p6 * s.dn[142][5]) * (nv0 - nv7));
        let eq59_e999_d_n6: f64 = ((p.p6 * s.dn[142][6]) * (nv0 - nv7));
        let eq59_e999_d_n7: f64 = (((p.p6 * s.dn[142][7]) * (nv0 - nv7)) + (-eq59_e997));
        let eq59_e999_d_n8: f64 = ((p.p6 * s.dn[142][8]) * (nv0 - nv7));
        let eq59_e999_d_n9: f64 = ((p.p6 * s.dn[142][9]) * (nv0 - nv7));
        let eq59_e999_d_n10: f64 = ((p.p6 * s.dn[142][10]) * (nv0 - nv7));
        let eq59_e999_d_n11: f64 = ((p.p6 * s.dn[142][11]) * (nv0 - nv7));
        let eq59_e999_d_n12: f64 = ((p.p6 * s.dn[142][12]) * (nv0 - nv7));
        let eq59_e999_d_n13: f64 = ((p.p6 * s.dn[142][13]) * (nv0 - nv7));
        let eq59_e999_d_n14: f64 = ((p.p6 * s.dn[142][14]) * (nv0 - nv7));
        let eq59_e999_d_n15: f64 = ((p.p6 * s.dn[142][15]) * (nv0 - nv7));
        let eq59_e999_d_n16: f64 = ((p.p6 * s.dn[142][16]) * (nv0 - nv7));
        let eq59_e999_d_n17: f64 = ((p.p6 * s.dn[142][17]) * (nv0 - nv7));
        let eq59_e999_d_n18: f64 = ((p.p6 * s.dn[142][18]) * (nv0 - nv7));
        let eq59_e999_d_n19: f64 = ((p.p6 * s.dn[142][19]) * (nv0 - nv7));
        let eq59_e999_d_n20: f64 = ((p.p6 * s.dn[142][20]) * (nv0 - nv7));
        let eq59_e999_d_n21: f64 = ((p.p6 * s.dn[142][21]) * (nv0 - nv7));
        let eq59_e999_d_n22: f64 = ((p.p6 * s.dn[142][22]) * (nv0 - nv7));
        let eq59_e999_d_b0: f64 = ((p.p6 * s.db[142][0]) * (nv0 - nv7));
        let eq59_e999_d_b1: f64 = ((p.p6 * s.db[142][1]) * (nv0 - nv7));
        let eq59_e999_d_b2: f64 = ((p.p6 * s.db[142][2]) * (nv0 - nv7));
        let eq59_e999_d_b3: f64 = ((p.p6 * s.db[142][3]) * (nv0 - nv7));
        let eq59_e999_d_b4: f64 = ((p.p6 * s.db[142][4]) * (nv0 - nv7));
        let eq59_e999_d_b5: f64 = ((p.p6 * s.db[142][5]) * (nv0 - nv7));
        let eq59_e999_d_b6: f64 = ((p.p6 * s.db[142][6]) * (nv0 - nv7));
        let eq59_e999_d_b7: f64 = ((p.p6 * s.db[142][7]) * (nv0 - nv7));
        let eq59_e999_d_b8: f64 = ((p.p6 * s.db[142][8]) * (nv0 - nv7));
        let eq59_e999_d_b9: f64 = ((p.p6 * s.db[142][9]) * (nv0 - nv7));
        let eq59_e999_d_b10: f64 = ((p.p6 * s.db[142][10]) * (nv0 - nv7));
        let eq59_e999_d_b11: f64 = ((p.p6 * s.db[142][11]) * (nv0 - nv7));
        let eq59_e999_d_b12: f64 = ((p.p6 * s.db[142][12]) * (nv0 - nv7));
        let eq59_e999_d_b13: f64 = ((p.p6 * s.db[142][13]) * (nv0 - nv7));
        let eq59_e999_d_b14: f64 = ((p.p6 * s.db[142][14]) * (nv0 - nv7));
        let eq59_e999_d_b15: f64 = ((p.p6 * s.db[142][15]) * (nv0 - nv7));
        let eq59_e999_d_b16: f64 = ((p.p6 * s.db[142][16]) * (nv0 - nv7));
        let eq59_e999_d_b17: f64 = ((p.p6 * s.db[142][17]) * (nv0 - nv7));
        let eq59_e999_d_b18: f64 = ((p.p6 * s.db[142][18]) * (nv0 - nv7));
        let eq59_e999_d_b19: f64 = ((p.p6 * s.db[142][19]) * (nv0 - nv7));
        let eq59_e999_d_b20: f64 = ((p.p6 * s.db[142][20]) * (nv0 - nv7));
        let eq59_e999_d_b21: f64 = ((p.p6 * s.db[142][21]) * (nv0 - nv7));
        let eq59_e999_d_b22: f64 = ((p.p6 * s.db[142][22]) * (nv0 - nv7));
        let eq59_e999_d_b23: f64 = ((p.p6 * s.db[142][23]) * (nv0 - nv7));
        let eq59_e999_d_b24: f64 = ((p.p6 * s.db[142][24]) * (nv0 - nv7));
        let eq59_e999_d_b25: f64 = ((p.p6 * s.db[142][25]) * (nv0 - nv7));
        let eq59_e999_d_b26: f64 = ((p.p6 * s.db[142][26]) * (nv0 - nv7));
        let eq59_e999_d_b27: f64 = ((p.p6 * s.db[142][27]) * (nv0 - nv7));
        let eq59_e999_d_b28: f64 = ((p.p6 * s.db[142][28]) * (nv0 - nv7));
        let eq59_e999_d_b29: f64 = ((p.p6 * s.db[142][29]) * (nv0 - nv7));
        let eq59_e999_d_b30: f64 = ((p.p6 * s.db[142][30]) * (nv0 - nv7));
        let eq59_e999_d_b31: f64 = ((p.p6 * s.db[142][31]) * (nv0 - nv7));
        let eq59_e999_d_b32: f64 = ((p.p6 * s.db[142][32]) * (nv0 - nv7));
        let eq59_e999_d_b33: f64 = ((p.p6 * s.db[142][33]) * (nv0 - nv7));
        let eq59_e999_d_b34: f64 = ((p.p6 * s.db[142][34]) * (nv0 - nv7));
        let eq59_e999_d_b35: f64 = ((p.p6 * s.db[142][35]) * (nv0 - nv7));
        let eq59_e999_d_b36: f64 = ((p.p6 * s.db[142][36]) * (nv0 - nv7));
        let eq59_e999_d_b37: f64 = ((p.p6 * s.db[142][37]) * (nv0 - nv7));
        let eq59_e999_d_b38: f64 = ((p.p6 * s.db[142][38]) * (nv0 - nv7));
        let eq59_e999_d_b39: f64 = ((p.p6 * s.db[142][39]) * (nv0 - nv7));
        let eq59_e999_d_b40: f64 = ((p.p6 * s.db[142][40]) * (nv0 - nv7));
        let eq59_e999_d_b41: f64 = ((p.p6 * s.db[142][41]) * (nv0 - nv7));
        let eq59_e999_d_b42: f64 = ((p.p6 * s.db[142][42]) * (nv0 - nv7));
        let eq59_e999_d_b43: f64 = ((p.p6 * s.db[142][43]) * (nv0 - nv7));
        let eq59_e999_d_b44: f64 = ((p.p6 * s.db[142][44]) * (nv0 - nv7));
        let eq59_e999_d_b45: f64 = ((p.p6 * s.db[142][45]) * (nv0 - nv7));
        let eq59_e999_d_b46: f64 = ((p.p6 * s.db[142][46]) * (nv0 - nv7));
        let eq59_e999_d_b47: f64 = ((p.p6 * s.db[142][47]) * (nv0 - nv7));
        let eq59_e999_d_b48: f64 = ((p.p6 * s.db[142][48]) * (nv0 - nv7));
        let eq59_e999_d_b49: f64 = ((p.p6 * s.db[142][49]) * (nv0 - nv7));
        let eq59_e999_d_b50: f64 = ((p.p6 * s.db[142][50]) * (nv0 - nv7));
        let eq59_e999_d_b51: f64 = ((p.p6 * s.db[142][51]) * (nv0 - nv7));
        let eq59_e999_d_b52: f64 = ((p.p6 * s.db[142][52]) * (nv0 - nv7));
        let eq59_e999_d_b53: f64 = ((p.p6 * s.db[142][53]) * (nv0 - nv7));
        let eq59_e999_d_b54: f64 = ((p.p6 * s.db[142][54]) * (nv0 - nv7));
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

    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq60_e1012, eq60_e1012_d_n0, eq60_e1012_d_n1, eq60_e1012_d_n2, eq60_e1012_d_n3, eq60_e1012_d_n4, eq60_e1012_d_n5, eq60_e1012_d_n6, eq60_e1012_d_n7, eq60_e1012_d_n8, eq60_e1012_d_n9, eq60_e1012_d_n10, eq60_e1012_d_n11, eq60_e1012_d_n12, eq60_e1012_d_n13, eq60_e1012_d_n14, eq60_e1012_d_n15, eq60_e1012_d_n16, eq60_e1012_d_n17, eq60_e1012_d_n18, eq60_e1012_d_n19, eq60_e1012_d_n20, eq60_e1012_d_n21, eq60_e1012_d_n22, eq60_e1012_d_b0, eq60_e1012_d_b1, eq60_e1012_d_b2, eq60_e1012_d_b3, eq60_e1012_d_b4, eq60_e1012_d_b5, eq60_e1012_d_b6, eq60_e1012_d_b7, eq60_e1012_d_b8, eq60_e1012_d_b9, eq60_e1012_d_b10, eq60_e1012_d_b11, eq60_e1012_d_b12, eq60_e1012_d_b13, eq60_e1012_d_b14, eq60_e1012_d_b15, eq60_e1012_d_b16, eq60_e1012_d_b17, eq60_e1012_d_b18, eq60_e1012_d_b19, eq60_e1012_d_b20, eq60_e1012_d_b21, eq60_e1012_d_b22, eq60_e1012_d_b23, eq60_e1012_d_b24, eq60_e1012_d_b25, eq60_e1012_d_b26, eq60_e1012_d_b27, eq60_e1012_d_b28, eq60_e1012_d_b29, eq60_e1012_d_b30, eq60_e1012_d_b31, eq60_e1012_d_b32, eq60_e1012_d_b33, eq60_e1012_d_b34, eq60_e1012_d_b35, eq60_e1012_d_b36, eq60_e1012_d_b37, eq60_e1012_d_b38, eq60_e1012_d_b39, eq60_e1012_d_b40, eq60_e1012_d_b41, eq60_e1012_d_b42, eq60_e1012_d_b43, eq60_e1012_d_b44, eq60_e1012_d_b45, eq60_e1012_d_b46, eq60_e1012_d_b47, eq60_e1012_d_b48, eq60_e1012_d_b49, eq60_e1012_d_b50, eq60_e1012_d_b51, eq60_e1012_d_b52, eq60_e1012_d_b53, eq60_e1012_d_b54,) = {
    if (s.b[424] && (!s.b[427])) {
        let eq60_e1008: f64 = (p.p6 * s.v[143]);
        let eq60_e1010: f64 = (eq60_e1008 * (nv8 - nv2));
        let eq60_e1010_d_n0: f64 = ((p.p6 * s.dn[143][0]) * (nv8 - nv2));
        let eq60_e1010_d_n1: f64 = ((p.p6 * s.dn[143][1]) * (nv8 - nv2));
        let eq60_e1010_d_n2: f64 = (((p.p6 * s.dn[143][2]) * (nv8 - nv2)) + (-eq60_e1008));
        let eq60_e1010_d_n3: f64 = ((p.p6 * s.dn[143][3]) * (nv8 - nv2));
        let eq60_e1010_d_n4: f64 = ((p.p6 * s.dn[143][4]) * (nv8 - nv2));
        let eq60_e1010_d_n5: f64 = ((p.p6 * s.dn[143][5]) * (nv8 - nv2));
        let eq60_e1010_d_n6: f64 = ((p.p6 * s.dn[143][6]) * (nv8 - nv2));
        let eq60_e1010_d_n7: f64 = ((p.p6 * s.dn[143][7]) * (nv8 - nv2));
        let eq60_e1010_d_n8: f64 = (((p.p6 * s.dn[143][8]) * (nv8 - nv2)) + eq60_e1008);
        let eq60_e1010_d_n9: f64 = ((p.p6 * s.dn[143][9]) * (nv8 - nv2));
        let eq60_e1010_d_n10: f64 = ((p.p6 * s.dn[143][10]) * (nv8 - nv2));
        let eq60_e1010_d_n11: f64 = ((p.p6 * s.dn[143][11]) * (nv8 - nv2));
        let eq60_e1010_d_n12: f64 = ((p.p6 * s.dn[143][12]) * (nv8 - nv2));
        let eq60_e1010_d_n13: f64 = ((p.p6 * s.dn[143][13]) * (nv8 - nv2));
        let eq60_e1010_d_n14: f64 = ((p.p6 * s.dn[143][14]) * (nv8 - nv2));
        let eq60_e1010_d_n15: f64 = ((p.p6 * s.dn[143][15]) * (nv8 - nv2));
        let eq60_e1010_d_n16: f64 = ((p.p6 * s.dn[143][16]) * (nv8 - nv2));
        let eq60_e1010_d_n17: f64 = ((p.p6 * s.dn[143][17]) * (nv8 - nv2));
        let eq60_e1010_d_n18: f64 = ((p.p6 * s.dn[143][18]) * (nv8 - nv2));
        let eq60_e1010_d_n19: f64 = ((p.p6 * s.dn[143][19]) * (nv8 - nv2));
        let eq60_e1010_d_n20: f64 = ((p.p6 * s.dn[143][20]) * (nv8 - nv2));
        let eq60_e1010_d_n21: f64 = ((p.p6 * s.dn[143][21]) * (nv8 - nv2));
        let eq60_e1010_d_n22: f64 = ((p.p6 * s.dn[143][22]) * (nv8 - nv2));
        let eq60_e1010_d_b0: f64 = ((p.p6 * s.db[143][0]) * (nv8 - nv2));
        let eq60_e1010_d_b1: f64 = ((p.p6 * s.db[143][1]) * (nv8 - nv2));
        let eq60_e1010_d_b2: f64 = ((p.p6 * s.db[143][2]) * (nv8 - nv2));
        let eq60_e1010_d_b3: f64 = ((p.p6 * s.db[143][3]) * (nv8 - nv2));
        let eq60_e1010_d_b4: f64 = ((p.p6 * s.db[143][4]) * (nv8 - nv2));
        let eq60_e1010_d_b5: f64 = ((p.p6 * s.db[143][5]) * (nv8 - nv2));
        let eq60_e1010_d_b6: f64 = ((p.p6 * s.db[143][6]) * (nv8 - nv2));
        let eq60_e1010_d_b7: f64 = ((p.p6 * s.db[143][7]) * (nv8 - nv2));
        let eq60_e1010_d_b8: f64 = ((p.p6 * s.db[143][8]) * (nv8 - nv2));
        let eq60_e1010_d_b9: f64 = ((p.p6 * s.db[143][9]) * (nv8 - nv2));
        let eq60_e1010_d_b10: f64 = ((p.p6 * s.db[143][10]) * (nv8 - nv2));
        let eq60_e1010_d_b11: f64 = ((p.p6 * s.db[143][11]) * (nv8 - nv2));
        let eq60_e1010_d_b12: f64 = ((p.p6 * s.db[143][12]) * (nv8 - nv2));
        let eq60_e1010_d_b13: f64 = ((p.p6 * s.db[143][13]) * (nv8 - nv2));
        let eq60_e1010_d_b14: f64 = ((p.p6 * s.db[143][14]) * (nv8 - nv2));
        let eq60_e1010_d_b15: f64 = ((p.p6 * s.db[143][15]) * (nv8 - nv2));
        let eq60_e1010_d_b16: f64 = ((p.p6 * s.db[143][16]) * (nv8 - nv2));
        let eq60_e1010_d_b17: f64 = ((p.p6 * s.db[143][17]) * (nv8 - nv2));
        let eq60_e1010_d_b18: f64 = ((p.p6 * s.db[143][18]) * (nv8 - nv2));
        let eq60_e1010_d_b19: f64 = ((p.p6 * s.db[143][19]) * (nv8 - nv2));
        let eq60_e1010_d_b20: f64 = ((p.p6 * s.db[143][20]) * (nv8 - nv2));
        let eq60_e1010_d_b21: f64 = ((p.p6 * s.db[143][21]) * (nv8 - nv2));
        let eq60_e1010_d_b22: f64 = ((p.p6 * s.db[143][22]) * (nv8 - nv2));
        let eq60_e1010_d_b23: f64 = ((p.p6 * s.db[143][23]) * (nv8 - nv2));
        let eq60_e1010_d_b24: f64 = ((p.p6 * s.db[143][24]) * (nv8 - nv2));
        let eq60_e1010_d_b25: f64 = ((p.p6 * s.db[143][25]) * (nv8 - nv2));
        let eq60_e1010_d_b26: f64 = ((p.p6 * s.db[143][26]) * (nv8 - nv2));
        let eq60_e1010_d_b27: f64 = ((p.p6 * s.db[143][27]) * (nv8 - nv2));
        let eq60_e1010_d_b28: f64 = ((p.p6 * s.db[143][28]) * (nv8 - nv2));
        let eq60_e1010_d_b29: f64 = ((p.p6 * s.db[143][29]) * (nv8 - nv2));
        let eq60_e1010_d_b30: f64 = ((p.p6 * s.db[143][30]) * (nv8 - nv2));
        let eq60_e1010_d_b31: f64 = ((p.p6 * s.db[143][31]) * (nv8 - nv2));
        let eq60_e1010_d_b32: f64 = ((p.p6 * s.db[143][32]) * (nv8 - nv2));
        let eq60_e1010_d_b33: f64 = ((p.p6 * s.db[143][33]) * (nv8 - nv2));
        let eq60_e1010_d_b34: f64 = ((p.p6 * s.db[143][34]) * (nv8 - nv2));
        let eq60_e1010_d_b35: f64 = ((p.p6 * s.db[143][35]) * (nv8 - nv2));
        let eq60_e1010_d_b36: f64 = ((p.p6 * s.db[143][36]) * (nv8 - nv2));
        let eq60_e1010_d_b37: f64 = ((p.p6 * s.db[143][37]) * (nv8 - nv2));
        let eq60_e1010_d_b38: f64 = ((p.p6 * s.db[143][38]) * (nv8 - nv2));
        let eq60_e1010_d_b39: f64 = ((p.p6 * s.db[143][39]) * (nv8 - nv2));
        let eq60_e1010_d_b40: f64 = ((p.p6 * s.db[143][40]) * (nv8 - nv2));
        let eq60_e1010_d_b41: f64 = ((p.p6 * s.db[143][41]) * (nv8 - nv2));
        let eq60_e1010_d_b42: f64 = ((p.p6 * s.db[143][42]) * (nv8 - nv2));
        let eq60_e1010_d_b43: f64 = ((p.p6 * s.db[143][43]) * (nv8 - nv2));
        let eq60_e1010_d_b44: f64 = ((p.p6 * s.db[143][44]) * (nv8 - nv2));
        let eq60_e1010_d_b45: f64 = ((p.p6 * s.db[143][45]) * (nv8 - nv2));
        let eq60_e1010_d_b46: f64 = ((p.p6 * s.db[143][46]) * (nv8 - nv2));
        let eq60_e1010_d_b47: f64 = ((p.p6 * s.db[143][47]) * (nv8 - nv2));
        let eq60_e1010_d_b48: f64 = ((p.p6 * s.db[143][48]) * (nv8 - nv2));
        let eq60_e1010_d_b49: f64 = ((p.p6 * s.db[143][49]) * (nv8 - nv2));
        let eq60_e1010_d_b50: f64 = ((p.p6 * s.db[143][50]) * (nv8 - nv2));
        let eq60_e1010_d_b51: f64 = ((p.p6 * s.db[143][51]) * (nv8 - nv2));
        let eq60_e1010_d_b52: f64 = ((p.p6 * s.db[143][52]) * (nv8 - nv2));
        let eq60_e1010_d_b53: f64 = ((p.p6 * s.db[143][53]) * (nv8 - nv2));
        let eq60_e1010_d_b54: f64 = ((p.p6 * s.db[143][54]) * (nv8 - nv2));
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
        let (eq72_e1166, eq72_e1166_d_n0, eq72_e1166_d_n1, eq72_e1166_d_n2, eq72_e1166_d_n3, eq72_e1166_d_n4, eq72_e1166_d_n5, eq72_e1166_d_n6, eq72_e1166_d_n7, eq72_e1166_d_n8, eq72_e1166_d_n9, eq72_e1166_d_n10, eq72_e1166_d_n11, eq72_e1166_d_n12, eq72_e1166_d_n13, eq72_e1166_d_n14, eq72_e1166_d_n15, eq72_e1166_d_n16, eq72_e1166_d_n17, eq72_e1166_d_n18, eq72_e1166_d_n19, eq72_e1166_d_n20, eq72_e1166_d_n21, eq72_e1166_d_n22, eq72_e1166_d_b0, eq72_e1166_d_b1, eq72_e1166_d_b2, eq72_e1166_d_b3, eq72_e1166_d_b4, eq72_e1166_d_b5, eq72_e1166_d_b6, eq72_e1166_d_b7, eq72_e1166_d_b8, eq72_e1166_d_b9, eq72_e1166_d_b10, eq72_e1166_d_b11, eq72_e1166_d_b12, eq72_e1166_d_b13, eq72_e1166_d_b14, eq72_e1166_d_b15, eq72_e1166_d_b16, eq72_e1166_d_b17, eq72_e1166_d_b18, eq72_e1166_d_b19, eq72_e1166_d_b20, eq72_e1166_d_b21, eq72_e1166_d_b22, eq72_e1166_d_b23, eq72_e1166_d_b24, eq72_e1166_d_b25, eq72_e1166_d_b26, eq72_e1166_d_b27, eq72_e1166_d_b28, eq72_e1166_d_b29, eq72_e1166_d_b30, eq72_e1166_d_b31, eq72_e1166_d_b32, eq72_e1166_d_b33, eq72_e1166_d_b34, eq72_e1166_d_b35, eq72_e1166_d_b36, eq72_e1166_d_b37, eq72_e1166_d_b38, eq72_e1166_d_b39, eq72_e1166_d_b40, eq72_e1166_d_b41, eq72_e1166_d_b42, eq72_e1166_d_b43, eq72_e1166_d_b44, eq72_e1166_d_b45, eq72_e1166_d_b46, eq72_e1166_d_b47, eq72_e1166_d_b48, eq72_e1166_d_b49, eq72_e1166_d_b50, eq72_e1166_d_b51, eq72_e1166_d_b52, eq72_e1166_d_b53, eq72_e1166_d_b54,) = {
    if (s.b[433] && s.b[434]) {
        let eq72_e1156: f64 = (p.p6 * s.v[48]);
        let eq72_e1158: f64 = (eq72_e1156 * s.v[233]);
        let eq72_e1158_d_n0: f64 = (((p.p6 * s.dn[48][0]) * s.v[233]) + (eq72_e1156 * s.dn[233][0]));
        let eq72_e1158_d_n1: f64 = (((p.p6 * s.dn[48][1]) * s.v[233]) + (eq72_e1156 * s.dn[233][1]));
        let eq72_e1158_d_n2: f64 = (((p.p6 * s.dn[48][2]) * s.v[233]) + (eq72_e1156 * s.dn[233][2]));
        let eq72_e1158_d_n3: f64 = (((p.p6 * s.dn[48][3]) * s.v[233]) + (eq72_e1156 * s.dn[233][3]));
        let eq72_e1158_d_n4: f64 = (((p.p6 * s.dn[48][4]) * s.v[233]) + (eq72_e1156 * s.dn[233][4]));
        let eq72_e1158_d_n5: f64 = (((p.p6 * s.dn[48][5]) * s.v[233]) + (eq72_e1156 * s.dn[233][5]));
        let eq72_e1158_d_n6: f64 = (((p.p6 * s.dn[48][6]) * s.v[233]) + (eq72_e1156 * s.dn[233][6]));
        let eq72_e1158_d_n7: f64 = (((p.p6 * s.dn[48][7]) * s.v[233]) + (eq72_e1156 * s.dn[233][7]));
        let eq72_e1158_d_n8: f64 = (((p.p6 * s.dn[48][8]) * s.v[233]) + (eq72_e1156 * s.dn[233][8]));
        let eq72_e1158_d_n9: f64 = (((p.p6 * s.dn[48][9]) * s.v[233]) + (eq72_e1156 * s.dn[233][9]));
        let eq72_e1158_d_n10: f64 = (((p.p6 * s.dn[48][10]) * s.v[233]) + (eq72_e1156 * s.dn[233][10]));
        let eq72_e1158_d_n11: f64 = (((p.p6 * s.dn[48][11]) * s.v[233]) + (eq72_e1156 * s.dn[233][11]));
        let eq72_e1158_d_n12: f64 = (((p.p6 * s.dn[48][12]) * s.v[233]) + (eq72_e1156 * s.dn[233][12]));
        let eq72_e1158_d_n13: f64 = (((p.p6 * s.dn[48][13]) * s.v[233]) + (eq72_e1156 * s.dn[233][13]));
        let eq72_e1158_d_n14: f64 = (((p.p6 * s.dn[48][14]) * s.v[233]) + (eq72_e1156 * s.dn[233][14]));
        let eq72_e1158_d_n15: f64 = (((p.p6 * s.dn[48][15]) * s.v[233]) + (eq72_e1156 * s.dn[233][15]));
        let eq72_e1158_d_n16: f64 = (((p.p6 * s.dn[48][16]) * s.v[233]) + (eq72_e1156 * s.dn[233][16]));
        let eq72_e1158_d_n17: f64 = (((p.p6 * s.dn[48][17]) * s.v[233]) + (eq72_e1156 * s.dn[233][17]));
        let eq72_e1158_d_n18: f64 = (((p.p6 * s.dn[48][18]) * s.v[233]) + (eq72_e1156 * s.dn[233][18]));
        let eq72_e1158_d_n19: f64 = (((p.p6 * s.dn[48][19]) * s.v[233]) + (eq72_e1156 * s.dn[233][19]));
        let eq72_e1158_d_n20: f64 = (((p.p6 * s.dn[48][20]) * s.v[233]) + (eq72_e1156 * s.dn[233][20]));
        let eq72_e1158_d_n21: f64 = (((p.p6 * s.dn[48][21]) * s.v[233]) + (eq72_e1156 * s.dn[233][21]));
        let eq72_e1158_d_n22: f64 = (((p.p6 * s.dn[48][22]) * s.v[233]) + (eq72_e1156 * s.dn[233][22]));
        let eq72_e1158_d_b0: f64 = (((p.p6 * s.db[48][0]) * s.v[233]) + (eq72_e1156 * s.db[233][0]));
        let eq72_e1158_d_b1: f64 = (((p.p6 * s.db[48][1]) * s.v[233]) + (eq72_e1156 * s.db[233][1]));
        let eq72_e1158_d_b2: f64 = (((p.p6 * s.db[48][2]) * s.v[233]) + (eq72_e1156 * s.db[233][2]));
        let eq72_e1158_d_b3: f64 = (((p.p6 * s.db[48][3]) * s.v[233]) + (eq72_e1156 * s.db[233][3]));
        let eq72_e1158_d_b4: f64 = (((p.p6 * s.db[48][4]) * s.v[233]) + (eq72_e1156 * s.db[233][4]));
        let eq72_e1158_d_b5: f64 = (((p.p6 * s.db[48][5]) * s.v[233]) + (eq72_e1156 * s.db[233][5]));
        let eq72_e1158_d_b6: f64 = (((p.p6 * s.db[48][6]) * s.v[233]) + (eq72_e1156 * s.db[233][6]));
        let eq72_e1158_d_b7: f64 = (((p.p6 * s.db[48][7]) * s.v[233]) + (eq72_e1156 * s.db[233][7]));
        let eq72_e1158_d_b8: f64 = (((p.p6 * s.db[48][8]) * s.v[233]) + (eq72_e1156 * s.db[233][8]));
        let eq72_e1158_d_b9: f64 = (((p.p6 * s.db[48][9]) * s.v[233]) + (eq72_e1156 * s.db[233][9]));
        let eq72_e1158_d_b10: f64 = (((p.p6 * s.db[48][10]) * s.v[233]) + (eq72_e1156 * s.db[233][10]));
        let eq72_e1158_d_b11: f64 = (((p.p6 * s.db[48][11]) * s.v[233]) + (eq72_e1156 * s.db[233][11]));
        let eq72_e1158_d_b12: f64 = (((p.p6 * s.db[48][12]) * s.v[233]) + (eq72_e1156 * s.db[233][12]));
        let eq72_e1158_d_b13: f64 = (((p.p6 * s.db[48][13]) * s.v[233]) + (eq72_e1156 * s.db[233][13]));
        let eq72_e1158_d_b14: f64 = (((p.p6 * s.db[48][14]) * s.v[233]) + (eq72_e1156 * s.db[233][14]));
        let eq72_e1158_d_b15: f64 = (((p.p6 * s.db[48][15]) * s.v[233]) + (eq72_e1156 * s.db[233][15]));
        let eq72_e1158_d_b16: f64 = (((p.p6 * s.db[48][16]) * s.v[233]) + (eq72_e1156 * s.db[233][16]));
        let eq72_e1158_d_b17: f64 = (((p.p6 * s.db[48][17]) * s.v[233]) + (eq72_e1156 * s.db[233][17]));
        let eq72_e1158_d_b18: f64 = (((p.p6 * s.db[48][18]) * s.v[233]) + (eq72_e1156 * s.db[233][18]));
        let eq72_e1158_d_b19: f64 = (((p.p6 * s.db[48][19]) * s.v[233]) + (eq72_e1156 * s.db[233][19]));
        let eq72_e1158_d_b20: f64 = (((p.p6 * s.db[48][20]) * s.v[233]) + (eq72_e1156 * s.db[233][20]));
        let eq72_e1158_d_b21: f64 = (((p.p6 * s.db[48][21]) * s.v[233]) + (eq72_e1156 * s.db[233][21]));
        let eq72_e1158_d_b22: f64 = (((p.p6 * s.db[48][22]) * s.v[233]) + (eq72_e1156 * s.db[233][22]));
        let eq72_e1158_d_b23: f64 = (((p.p6 * s.db[48][23]) * s.v[233]) + (eq72_e1156 * s.db[233][23]));
        let eq72_e1158_d_b24: f64 = (((p.p6 * s.db[48][24]) * s.v[233]) + (eq72_e1156 * s.db[233][24]));
        let eq72_e1158_d_b25: f64 = (((p.p6 * s.db[48][25]) * s.v[233]) + (eq72_e1156 * s.db[233][25]));
        let eq72_e1158_d_b26: f64 = (((p.p6 * s.db[48][26]) * s.v[233]) + (eq72_e1156 * s.db[233][26]));
        let eq72_e1158_d_b27: f64 = (((p.p6 * s.db[48][27]) * s.v[233]) + (eq72_e1156 * s.db[233][27]));
        let eq72_e1158_d_b28: f64 = (((p.p6 * s.db[48][28]) * s.v[233]) + (eq72_e1156 * s.db[233][28]));
        let eq72_e1158_d_b29: f64 = (((p.p6 * s.db[48][29]) * s.v[233]) + (eq72_e1156 * s.db[233][29]));
        let eq72_e1158_d_b30: f64 = (((p.p6 * s.db[48][30]) * s.v[233]) + (eq72_e1156 * s.db[233][30]));
        let eq72_e1158_d_b31: f64 = (((p.p6 * s.db[48][31]) * s.v[233]) + (eq72_e1156 * s.db[233][31]));
        let eq72_e1158_d_b32: f64 = (((p.p6 * s.db[48][32]) * s.v[233]) + (eq72_e1156 * s.db[233][32]));
        let eq72_e1158_d_b33: f64 = (((p.p6 * s.db[48][33]) * s.v[233]) + (eq72_e1156 * s.db[233][33]));
        let eq72_e1158_d_b34: f64 = (((p.p6 * s.db[48][34]) * s.v[233]) + (eq72_e1156 * s.db[233][34]));
        let eq72_e1158_d_b35: f64 = (((p.p6 * s.db[48][35]) * s.v[233]) + (eq72_e1156 * s.db[233][35]));
        let eq72_e1158_d_b36: f64 = (((p.p6 * s.db[48][36]) * s.v[233]) + (eq72_e1156 * s.db[233][36]));
        let eq72_e1158_d_b37: f64 = (((p.p6 * s.db[48][37]) * s.v[233]) + (eq72_e1156 * s.db[233][37]));
        let eq72_e1158_d_b38: f64 = (((p.p6 * s.db[48][38]) * s.v[233]) + (eq72_e1156 * s.db[233][38]));
        let eq72_e1158_d_b39: f64 = (((p.p6 * s.db[48][39]) * s.v[233]) + (eq72_e1156 * s.db[233][39]));
        let eq72_e1158_d_b40: f64 = (((p.p6 * s.db[48][40]) * s.v[233]) + (eq72_e1156 * s.db[233][40]));
        let eq72_e1158_d_b41: f64 = (((p.p6 * s.db[48][41]) * s.v[233]) + (eq72_e1156 * s.db[233][41]));
        let eq72_e1158_d_b42: f64 = (((p.p6 * s.db[48][42]) * s.v[233]) + (eq72_e1156 * s.db[233][42]));
        let eq72_e1158_d_b43: f64 = (((p.p6 * s.db[48][43]) * s.v[233]) + (eq72_e1156 * s.db[233][43]));
        let eq72_e1158_d_b44: f64 = (((p.p6 * s.db[48][44]) * s.v[233]) + (eq72_e1156 * s.db[233][44]));
        let eq72_e1158_d_b45: f64 = (((p.p6 * s.db[48][45]) * s.v[233]) + (eq72_e1156 * s.db[233][45]));
        let eq72_e1158_d_b46: f64 = (((p.p6 * s.db[48][46]) * s.v[233]) + (eq72_e1156 * s.db[233][46]));
        let eq72_e1158_d_b47: f64 = (((p.p6 * s.db[48][47]) * s.v[233]) + (eq72_e1156 * s.db[233][47]));
        let eq72_e1158_d_b48: f64 = (((p.p6 * s.db[48][48]) * s.v[233]) + (eq72_e1156 * s.db[233][48]));
        let eq72_e1158_d_b49: f64 = (((p.p6 * s.db[48][49]) * s.v[233]) + (eq72_e1156 * s.db[233][49]));
        let eq72_e1158_d_b50: f64 = (((p.p6 * s.db[48][50]) * s.v[233]) + (eq72_e1156 * s.db[233][50]));
        let eq72_e1158_d_b51: f64 = (((p.p6 * s.db[48][51]) * s.v[233]) + (eq72_e1156 * s.db[233][51]));
        let eq72_e1158_d_b52: f64 = (((p.p6 * s.db[48][52]) * s.v[233]) + (eq72_e1156 * s.db[233][52]));
        let eq72_e1158_d_b53: f64 = (((p.p6 * s.db[48][53]) * s.v[233]) + (eq72_e1156 * s.db[233][53]));
        let eq72_e1158_d_b54: f64 = (((p.p6 * s.db[48][54]) * s.v[233]) + (eq72_e1156 * s.db[233][54]));
        let eq72_e1161: f64 = (p.p6 * s.v[379]);
        let eq72_e1163: f64 = (eq72_e1161 * (nv15 - nv7));
        let eq72_e1163_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv15 - nv7));
        let eq72_e1163_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv15 - nv7));
        let eq72_e1163_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv15 - nv7));
        let eq72_e1163_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv15 - nv7));
        let eq72_e1163_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv15 - nv7));
        let eq72_e1163_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv15 - nv7));
        let eq72_e1163_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv15 - nv7));
        let eq72_e1163_d_n7: f64 = (((p.p6 * s.dn[379][7]) * (nv15 - nv7)) + (-eq72_e1161));
        let eq72_e1163_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv15 - nv7));
        let eq72_e1163_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv15 - nv7));
        let eq72_e1163_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv15 - nv7));
        let eq72_e1163_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv15 - nv7));
        let eq72_e1163_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv15 - nv7));
        let eq72_e1163_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv15 - nv7));
        let eq72_e1163_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv15 - nv7));
        let eq72_e1163_d_n15: f64 = (((p.p6 * s.dn[379][15]) * (nv15 - nv7)) + eq72_e1161);
        let eq72_e1163_d_n16: f64 = ((p.p6 * s.dn[379][16]) * (nv15 - nv7));
        let eq72_e1163_d_n17: f64 = ((p.p6 * s.dn[379][17]) * (nv15 - nv7));
        let eq72_e1163_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv15 - nv7));
        let eq72_e1163_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv15 - nv7));
        let eq72_e1163_d_n20: f64 = ((p.p6 * s.dn[379][20]) * (nv15 - nv7));
        let eq72_e1163_d_n21: f64 = ((p.p6 * s.dn[379][21]) * (nv15 - nv7));
        let eq72_e1163_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv15 - nv7));
        let eq72_e1163_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv15 - nv7));
        let eq72_e1163_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv15 - nv7));
        let eq72_e1163_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv15 - nv7));
        let eq72_e1163_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv15 - nv7));
        let eq72_e1163_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv15 - nv7));
        let eq72_e1163_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv15 - nv7));
        let eq72_e1163_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv15 - nv7));
        let eq72_e1163_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv15 - nv7));
        let eq72_e1163_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv15 - nv7));
        let eq72_e1163_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv15 - nv7));
        let eq72_e1163_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv15 - nv7));
        let eq72_e1163_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv15 - nv7));
        let eq72_e1163_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv15 - nv7));
        let eq72_e1163_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv15 - nv7));
        let eq72_e1163_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv15 - nv7));
        let eq72_e1163_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv15 - nv7));
        let eq72_e1163_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv15 - nv7));
        let eq72_e1163_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv15 - nv7));
        let eq72_e1163_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv15 - nv7));
        let eq72_e1163_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv15 - nv7));
        let eq72_e1163_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv15 - nv7));
        let eq72_e1163_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv15 - nv7));
        let eq72_e1163_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv15 - nv7));
        let eq72_e1163_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv15 - nv7));
        let eq72_e1163_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv15 - nv7));
        let eq72_e1163_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv15 - nv7));
        let eq72_e1163_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv15 - nv7));
        let eq72_e1163_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv15 - nv7));
        let eq72_e1163_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv15 - nv7));
        let eq72_e1163_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv15 - nv7));
        let eq72_e1163_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv15 - nv7));
        let eq72_e1163_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv15 - nv7));
        let eq72_e1163_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv15 - nv7));
        let eq72_e1163_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv15 - nv7));
        let eq72_e1163_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv15 - nv7));
        let eq72_e1163_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv15 - nv7));
        let eq72_e1163_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv15 - nv7));
        let eq72_e1163_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv15 - nv7));
        let eq72_e1163_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv15 - nv7));
        let eq72_e1163_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv15 - nv7));
        let eq72_e1163_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv15 - nv7));
        let eq72_e1163_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv15 - nv7));
        let eq72_e1163_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv15 - nv7));
        let eq72_e1163_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv15 - nv7));
        let eq72_e1163_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv15 - nv7));
        let eq72_e1163_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv15 - nv7));
        let eq72_e1163_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv15 - nv7));
        let eq72_e1163_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv15 - nv7));
        let eq72_e1163_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv15 - nv7));
        let eq72_e1163_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv15 - nv7));
        let eq72_e1163_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv15 - nv7));
        let eq72_e1163_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv15 - nv7));
        let eq72_e1163_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv15 - nv7));
        let eq72_e1163_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv15 - nv7));
        let eq72_e1163_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv15 - nv7));
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
    }

    pub(super) fn stamp_transient_equations_block_7(
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
        let eq75_e1186: f64 = (eq75_e1184 * s.v[245]);
        let eq75_e1186_d_n0: f64 = (((p.p6 * s.dn[52][0]) * s.v[245]) + (eq75_e1184 * s.dn[245][0]));
        let eq75_e1186_d_n1: f64 = (((p.p6 * s.dn[52][1]) * s.v[245]) + (eq75_e1184 * s.dn[245][1]));
        let eq75_e1186_d_n2: f64 = (((p.p6 * s.dn[52][2]) * s.v[245]) + (eq75_e1184 * s.dn[245][2]));
        let eq75_e1186_d_n3: f64 = (((p.p6 * s.dn[52][3]) * s.v[245]) + (eq75_e1184 * s.dn[245][3]));
        let eq75_e1186_d_n4: f64 = (((p.p6 * s.dn[52][4]) * s.v[245]) + (eq75_e1184 * s.dn[245][4]));
        let eq75_e1186_d_n5: f64 = (((p.p6 * s.dn[52][5]) * s.v[245]) + (eq75_e1184 * s.dn[245][5]));
        let eq75_e1186_d_n6: f64 = (((p.p6 * s.dn[52][6]) * s.v[245]) + (eq75_e1184 * s.dn[245][6]));
        let eq75_e1186_d_n7: f64 = (((p.p6 * s.dn[52][7]) * s.v[245]) + (eq75_e1184 * s.dn[245][7]));
        let eq75_e1186_d_n8: f64 = (((p.p6 * s.dn[52][8]) * s.v[245]) + (eq75_e1184 * s.dn[245][8]));
        let eq75_e1186_d_n9: f64 = (((p.p6 * s.dn[52][9]) * s.v[245]) + (eq75_e1184 * s.dn[245][9]));
        let eq75_e1186_d_n10: f64 = (((p.p6 * s.dn[52][10]) * s.v[245]) + (eq75_e1184 * s.dn[245][10]));
        let eq75_e1186_d_n11: f64 = (((p.p6 * s.dn[52][11]) * s.v[245]) + (eq75_e1184 * s.dn[245][11]));
        let eq75_e1186_d_n12: f64 = (((p.p6 * s.dn[52][12]) * s.v[245]) + (eq75_e1184 * s.dn[245][12]));
        let eq75_e1186_d_n13: f64 = (((p.p6 * s.dn[52][13]) * s.v[245]) + (eq75_e1184 * s.dn[245][13]));
        let eq75_e1186_d_n14: f64 = (((p.p6 * s.dn[52][14]) * s.v[245]) + (eq75_e1184 * s.dn[245][14]));
        let eq75_e1186_d_n15: f64 = (((p.p6 * s.dn[52][15]) * s.v[245]) + (eq75_e1184 * s.dn[245][15]));
        let eq75_e1186_d_n16: f64 = (((p.p6 * s.dn[52][16]) * s.v[245]) + (eq75_e1184 * s.dn[245][16]));
        let eq75_e1186_d_n17: f64 = (((p.p6 * s.dn[52][17]) * s.v[245]) + (eq75_e1184 * s.dn[245][17]));
        let eq75_e1186_d_n18: f64 = (((p.p6 * s.dn[52][18]) * s.v[245]) + (eq75_e1184 * s.dn[245][18]));
        let eq75_e1186_d_n19: f64 = (((p.p6 * s.dn[52][19]) * s.v[245]) + (eq75_e1184 * s.dn[245][19]));
        let eq75_e1186_d_n20: f64 = (((p.p6 * s.dn[52][20]) * s.v[245]) + (eq75_e1184 * s.dn[245][20]));
        let eq75_e1186_d_n21: f64 = (((p.p6 * s.dn[52][21]) * s.v[245]) + (eq75_e1184 * s.dn[245][21]));
        let eq75_e1186_d_n22: f64 = (((p.p6 * s.dn[52][22]) * s.v[245]) + (eq75_e1184 * s.dn[245][22]));
        let eq75_e1186_d_b0: f64 = (((p.p6 * s.db[52][0]) * s.v[245]) + (eq75_e1184 * s.db[245][0]));
        let eq75_e1186_d_b1: f64 = (((p.p6 * s.db[52][1]) * s.v[245]) + (eq75_e1184 * s.db[245][1]));
        let eq75_e1186_d_b2: f64 = (((p.p6 * s.db[52][2]) * s.v[245]) + (eq75_e1184 * s.db[245][2]));
        let eq75_e1186_d_b3: f64 = (((p.p6 * s.db[52][3]) * s.v[245]) + (eq75_e1184 * s.db[245][3]));
        let eq75_e1186_d_b4: f64 = (((p.p6 * s.db[52][4]) * s.v[245]) + (eq75_e1184 * s.db[245][4]));
        let eq75_e1186_d_b5: f64 = (((p.p6 * s.db[52][5]) * s.v[245]) + (eq75_e1184 * s.db[245][5]));
        let eq75_e1186_d_b6: f64 = (((p.p6 * s.db[52][6]) * s.v[245]) + (eq75_e1184 * s.db[245][6]));
        let eq75_e1186_d_b7: f64 = (((p.p6 * s.db[52][7]) * s.v[245]) + (eq75_e1184 * s.db[245][7]));
        let eq75_e1186_d_b8: f64 = (((p.p6 * s.db[52][8]) * s.v[245]) + (eq75_e1184 * s.db[245][8]));
        let eq75_e1186_d_b9: f64 = (((p.p6 * s.db[52][9]) * s.v[245]) + (eq75_e1184 * s.db[245][9]));
        let eq75_e1186_d_b10: f64 = (((p.p6 * s.db[52][10]) * s.v[245]) + (eq75_e1184 * s.db[245][10]));
        let eq75_e1186_d_b11: f64 = (((p.p6 * s.db[52][11]) * s.v[245]) + (eq75_e1184 * s.db[245][11]));
        let eq75_e1186_d_b12: f64 = (((p.p6 * s.db[52][12]) * s.v[245]) + (eq75_e1184 * s.db[245][12]));
        let eq75_e1186_d_b13: f64 = (((p.p6 * s.db[52][13]) * s.v[245]) + (eq75_e1184 * s.db[245][13]));
        let eq75_e1186_d_b14: f64 = (((p.p6 * s.db[52][14]) * s.v[245]) + (eq75_e1184 * s.db[245][14]));
        let eq75_e1186_d_b15: f64 = (((p.p6 * s.db[52][15]) * s.v[245]) + (eq75_e1184 * s.db[245][15]));
        let eq75_e1186_d_b16: f64 = (((p.p6 * s.db[52][16]) * s.v[245]) + (eq75_e1184 * s.db[245][16]));
        let eq75_e1186_d_b17: f64 = (((p.p6 * s.db[52][17]) * s.v[245]) + (eq75_e1184 * s.db[245][17]));
        let eq75_e1186_d_b18: f64 = (((p.p6 * s.db[52][18]) * s.v[245]) + (eq75_e1184 * s.db[245][18]));
        let eq75_e1186_d_b19: f64 = (((p.p6 * s.db[52][19]) * s.v[245]) + (eq75_e1184 * s.db[245][19]));
        let eq75_e1186_d_b20: f64 = (((p.p6 * s.db[52][20]) * s.v[245]) + (eq75_e1184 * s.db[245][20]));
        let eq75_e1186_d_b21: f64 = (((p.p6 * s.db[52][21]) * s.v[245]) + (eq75_e1184 * s.db[245][21]));
        let eq75_e1186_d_b22: f64 = (((p.p6 * s.db[52][22]) * s.v[245]) + (eq75_e1184 * s.db[245][22]));
        let eq75_e1186_d_b23: f64 = (((p.p6 * s.db[52][23]) * s.v[245]) + (eq75_e1184 * s.db[245][23]));
        let eq75_e1186_d_b24: f64 = (((p.p6 * s.db[52][24]) * s.v[245]) + (eq75_e1184 * s.db[245][24]));
        let eq75_e1186_d_b25: f64 = (((p.p6 * s.db[52][25]) * s.v[245]) + (eq75_e1184 * s.db[245][25]));
        let eq75_e1186_d_b26: f64 = (((p.p6 * s.db[52][26]) * s.v[245]) + (eq75_e1184 * s.db[245][26]));
        let eq75_e1186_d_b27: f64 = (((p.p6 * s.db[52][27]) * s.v[245]) + (eq75_e1184 * s.db[245][27]));
        let eq75_e1186_d_b28: f64 = (((p.p6 * s.db[52][28]) * s.v[245]) + (eq75_e1184 * s.db[245][28]));
        let eq75_e1186_d_b29: f64 = (((p.p6 * s.db[52][29]) * s.v[245]) + (eq75_e1184 * s.db[245][29]));
        let eq75_e1186_d_b30: f64 = (((p.p6 * s.db[52][30]) * s.v[245]) + (eq75_e1184 * s.db[245][30]));
        let eq75_e1186_d_b31: f64 = (((p.p6 * s.db[52][31]) * s.v[245]) + (eq75_e1184 * s.db[245][31]));
        let eq75_e1186_d_b32: f64 = (((p.p6 * s.db[52][32]) * s.v[245]) + (eq75_e1184 * s.db[245][32]));
        let eq75_e1186_d_b33: f64 = (((p.p6 * s.db[52][33]) * s.v[245]) + (eq75_e1184 * s.db[245][33]));
        let eq75_e1186_d_b34: f64 = (((p.p6 * s.db[52][34]) * s.v[245]) + (eq75_e1184 * s.db[245][34]));
        let eq75_e1186_d_b35: f64 = (((p.p6 * s.db[52][35]) * s.v[245]) + (eq75_e1184 * s.db[245][35]));
        let eq75_e1186_d_b36: f64 = (((p.p6 * s.db[52][36]) * s.v[245]) + (eq75_e1184 * s.db[245][36]));
        let eq75_e1186_d_b37: f64 = (((p.p6 * s.db[52][37]) * s.v[245]) + (eq75_e1184 * s.db[245][37]));
        let eq75_e1186_d_b38: f64 = (((p.p6 * s.db[52][38]) * s.v[245]) + (eq75_e1184 * s.db[245][38]));
        let eq75_e1186_d_b39: f64 = (((p.p6 * s.db[52][39]) * s.v[245]) + (eq75_e1184 * s.db[245][39]));
        let eq75_e1186_d_b40: f64 = (((p.p6 * s.db[52][40]) * s.v[245]) + (eq75_e1184 * s.db[245][40]));
        let eq75_e1186_d_b41: f64 = (((p.p6 * s.db[52][41]) * s.v[245]) + (eq75_e1184 * s.db[245][41]));
        let eq75_e1186_d_b42: f64 = (((p.p6 * s.db[52][42]) * s.v[245]) + (eq75_e1184 * s.db[245][42]));
        let eq75_e1186_d_b43: f64 = (((p.p6 * s.db[52][43]) * s.v[245]) + (eq75_e1184 * s.db[245][43]));
        let eq75_e1186_d_b44: f64 = (((p.p6 * s.db[52][44]) * s.v[245]) + (eq75_e1184 * s.db[245][44]));
        let eq75_e1186_d_b45: f64 = (((p.p6 * s.db[52][45]) * s.v[245]) + (eq75_e1184 * s.db[245][45]));
        let eq75_e1186_d_b46: f64 = (((p.p6 * s.db[52][46]) * s.v[245]) + (eq75_e1184 * s.db[245][46]));
        let eq75_e1186_d_b47: f64 = (((p.p6 * s.db[52][47]) * s.v[245]) + (eq75_e1184 * s.db[245][47]));
        let eq75_e1186_d_b48: f64 = (((p.p6 * s.db[52][48]) * s.v[245]) + (eq75_e1184 * s.db[245][48]));
        let eq75_e1186_d_b49: f64 = (((p.p6 * s.db[52][49]) * s.v[245]) + (eq75_e1184 * s.db[245][49]));
        let eq75_e1186_d_b50: f64 = (((p.p6 * s.db[52][50]) * s.v[245]) + (eq75_e1184 * s.db[245][50]));
        let eq75_e1186_d_b51: f64 = (((p.p6 * s.db[52][51]) * s.v[245]) + (eq75_e1184 * s.db[245][51]));
        let eq75_e1186_d_b52: f64 = (((p.p6 * s.db[52][52]) * s.v[245]) + (eq75_e1184 * s.db[245][52]));
        let eq75_e1186_d_b53: f64 = (((p.p6 * s.db[52][53]) * s.v[245]) + (eq75_e1184 * s.db[245][53]));
        let eq75_e1186_d_b54: f64 = (((p.p6 * s.db[52][54]) * s.v[245]) + (eq75_e1184 * s.db[245][54]));
        let eq75_e1189: f64 = (p.p6 * s.v[379]);
        let eq75_e1191: f64 = (eq75_e1189 * (nv8 - nv19));
        let eq75_e1191_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv8 - nv19));
        let eq75_e1191_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv8 - nv19));
        let eq75_e1191_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv8 - nv19));
        let eq75_e1191_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv8 - nv19));
        let eq75_e1191_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv8 - nv19));
        let eq75_e1191_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv8 - nv19));
        let eq75_e1191_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv8 - nv19));
        let eq75_e1191_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv8 - nv19));
        let eq75_e1191_d_n8: f64 = (((p.p6 * s.dn[379][8]) * (nv8 - nv19)) + eq75_e1189);
        let eq75_e1191_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv8 - nv19));
        let eq75_e1191_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv8 - nv19));
        let eq75_e1191_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv8 - nv19));
        let eq75_e1191_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv8 - nv19));
        let eq75_e1191_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv8 - nv19));
        let eq75_e1191_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv8 - nv19));
        let eq75_e1191_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv8 - nv19));
        let eq75_e1191_d_n16: f64 = ((p.p6 * s.dn[379][16]) * (nv8 - nv19));
        let eq75_e1191_d_n17: f64 = ((p.p6 * s.dn[379][17]) * (nv8 - nv19));
        let eq75_e1191_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv8 - nv19));
        let eq75_e1191_d_n19: f64 = (((p.p6 * s.dn[379][19]) * (nv8 - nv19)) + (-eq75_e1189));
        let eq75_e1191_d_n20: f64 = ((p.p6 * s.dn[379][20]) * (nv8 - nv19));
        let eq75_e1191_d_n21: f64 = ((p.p6 * s.dn[379][21]) * (nv8 - nv19));
        let eq75_e1191_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv8 - nv19));
        let eq75_e1191_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv8 - nv19));
        let eq75_e1191_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv8 - nv19));
        let eq75_e1191_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv8 - nv19));
        let eq75_e1191_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv8 - nv19));
        let eq75_e1191_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv8 - nv19));
        let eq75_e1191_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv8 - nv19));
        let eq75_e1191_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv8 - nv19));
        let eq75_e1191_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv8 - nv19));
        let eq75_e1191_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv8 - nv19));
        let eq75_e1191_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv8 - nv19));
        let eq75_e1191_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv8 - nv19));
        let eq75_e1191_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv8 - nv19));
        let eq75_e1191_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv8 - nv19));
        let eq75_e1191_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv8 - nv19));
        let eq75_e1191_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv8 - nv19));
        let eq75_e1191_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv8 - nv19));
        let eq75_e1191_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv8 - nv19));
        let eq75_e1191_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv8 - nv19));
        let eq75_e1191_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv8 - nv19));
        let eq75_e1191_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv8 - nv19));
        let eq75_e1191_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv8 - nv19));
        let eq75_e1191_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv8 - nv19));
        let eq75_e1191_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv8 - nv19));
        let eq75_e1191_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv8 - nv19));
        let eq75_e1191_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv8 - nv19));
        let eq75_e1191_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv8 - nv19));
        let eq75_e1191_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv8 - nv19));
        let eq75_e1191_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv8 - nv19));
        let eq75_e1191_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv8 - nv19));
        let eq75_e1191_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv8 - nv19));
        let eq75_e1191_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv8 - nv19));
        let eq75_e1191_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv8 - nv19));
        let eq75_e1191_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv8 - nv19));
        let eq75_e1191_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv8 - nv19));
        let eq75_e1191_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv8 - nv19));
        let eq75_e1191_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv8 - nv19));
        let eq75_e1191_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv8 - nv19));
        let eq75_e1191_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv8 - nv19));
        let eq75_e1191_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv8 - nv19));
        let eq75_e1191_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv8 - nv19));
        let eq75_e1191_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv8 - nv19));
        let eq75_e1191_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv8 - nv19));
        let eq75_e1191_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv8 - nv19));
        let eq75_e1191_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv8 - nv19));
        let eq75_e1191_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv8 - nv19));
        let eq75_e1191_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv8 - nv19));
        let eq75_e1191_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv8 - nv19));
        let eq75_e1191_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv8 - nv19));
        let eq75_e1191_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv8 - nv19));
        let eq75_e1191_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv8 - nv19));
        let eq75_e1191_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv8 - nv19));
        let eq75_e1191_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv8 - nv19));
        let eq75_e1191_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv8 - nv19));
        let eq75_e1191_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv8 - nv19));
        let eq75_e1191_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv8 - nv19));
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
    }

    pub(super) fn stamp_transient_equations_block_8(
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
        let eq79_e1222: f64 = (eq79_e1220 * s.v[257]);
        let eq79_e1222_d_n0: f64 = (((p.p6 * s.dn[56][0]) * s.v[257]) + (eq79_e1220 * s.dn[257][0]));
        let eq79_e1222_d_n1: f64 = (((p.p6 * s.dn[56][1]) * s.v[257]) + (eq79_e1220 * s.dn[257][1]));
        let eq79_e1222_d_n2: f64 = (((p.p6 * s.dn[56][2]) * s.v[257]) + (eq79_e1220 * s.dn[257][2]));
        let eq79_e1222_d_n3: f64 = (((p.p6 * s.dn[56][3]) * s.v[257]) + (eq79_e1220 * s.dn[257][3]));
        let eq79_e1222_d_n4: f64 = (((p.p6 * s.dn[56][4]) * s.v[257]) + (eq79_e1220 * s.dn[257][4]));
        let eq79_e1222_d_n5: f64 = (((p.p6 * s.dn[56][5]) * s.v[257]) + (eq79_e1220 * s.dn[257][5]));
        let eq79_e1222_d_n6: f64 = (((p.p6 * s.dn[56][6]) * s.v[257]) + (eq79_e1220 * s.dn[257][6]));
        let eq79_e1222_d_n7: f64 = (((p.p6 * s.dn[56][7]) * s.v[257]) + (eq79_e1220 * s.dn[257][7]));
        let eq79_e1222_d_n8: f64 = (((p.p6 * s.dn[56][8]) * s.v[257]) + (eq79_e1220 * s.dn[257][8]));
        let eq79_e1222_d_n9: f64 = (((p.p6 * s.dn[56][9]) * s.v[257]) + (eq79_e1220 * s.dn[257][9]));
        let eq79_e1222_d_n10: f64 = (((p.p6 * s.dn[56][10]) * s.v[257]) + (eq79_e1220 * s.dn[257][10]));
        let eq79_e1222_d_n11: f64 = (((p.p6 * s.dn[56][11]) * s.v[257]) + (eq79_e1220 * s.dn[257][11]));
        let eq79_e1222_d_n12: f64 = (((p.p6 * s.dn[56][12]) * s.v[257]) + (eq79_e1220 * s.dn[257][12]));
        let eq79_e1222_d_n13: f64 = (((p.p6 * s.dn[56][13]) * s.v[257]) + (eq79_e1220 * s.dn[257][13]));
        let eq79_e1222_d_n14: f64 = (((p.p6 * s.dn[56][14]) * s.v[257]) + (eq79_e1220 * s.dn[257][14]));
        let eq79_e1222_d_n15: f64 = (((p.p6 * s.dn[56][15]) * s.v[257]) + (eq79_e1220 * s.dn[257][15]));
        let eq79_e1222_d_n16: f64 = (((p.p6 * s.dn[56][16]) * s.v[257]) + (eq79_e1220 * s.dn[257][16]));
        let eq79_e1222_d_n17: f64 = (((p.p6 * s.dn[56][17]) * s.v[257]) + (eq79_e1220 * s.dn[257][17]));
        let eq79_e1222_d_n18: f64 = (((p.p6 * s.dn[56][18]) * s.v[257]) + (eq79_e1220 * s.dn[257][18]));
        let eq79_e1222_d_n19: f64 = (((p.p6 * s.dn[56][19]) * s.v[257]) + (eq79_e1220 * s.dn[257][19]));
        let eq79_e1222_d_n20: f64 = (((p.p6 * s.dn[56][20]) * s.v[257]) + (eq79_e1220 * s.dn[257][20]));
        let eq79_e1222_d_n21: f64 = (((p.p6 * s.dn[56][21]) * s.v[257]) + (eq79_e1220 * s.dn[257][21]));
        let eq79_e1222_d_n22: f64 = (((p.p6 * s.dn[56][22]) * s.v[257]) + (eq79_e1220 * s.dn[257][22]));
        let eq79_e1222_d_b0: f64 = (((p.p6 * s.db[56][0]) * s.v[257]) + (eq79_e1220 * s.db[257][0]));
        let eq79_e1222_d_b1: f64 = (((p.p6 * s.db[56][1]) * s.v[257]) + (eq79_e1220 * s.db[257][1]));
        let eq79_e1222_d_b2: f64 = (((p.p6 * s.db[56][2]) * s.v[257]) + (eq79_e1220 * s.db[257][2]));
        let eq79_e1222_d_b3: f64 = (((p.p6 * s.db[56][3]) * s.v[257]) + (eq79_e1220 * s.db[257][3]));
        let eq79_e1222_d_b4: f64 = (((p.p6 * s.db[56][4]) * s.v[257]) + (eq79_e1220 * s.db[257][4]));
        let eq79_e1222_d_b5: f64 = (((p.p6 * s.db[56][5]) * s.v[257]) + (eq79_e1220 * s.db[257][5]));
        let eq79_e1222_d_b6: f64 = (((p.p6 * s.db[56][6]) * s.v[257]) + (eq79_e1220 * s.db[257][6]));
        let eq79_e1222_d_b7: f64 = (((p.p6 * s.db[56][7]) * s.v[257]) + (eq79_e1220 * s.db[257][7]));
        let eq79_e1222_d_b8: f64 = (((p.p6 * s.db[56][8]) * s.v[257]) + (eq79_e1220 * s.db[257][8]));
        let eq79_e1222_d_b9: f64 = (((p.p6 * s.db[56][9]) * s.v[257]) + (eq79_e1220 * s.db[257][9]));
        let eq79_e1222_d_b10: f64 = (((p.p6 * s.db[56][10]) * s.v[257]) + (eq79_e1220 * s.db[257][10]));
        let eq79_e1222_d_b11: f64 = (((p.p6 * s.db[56][11]) * s.v[257]) + (eq79_e1220 * s.db[257][11]));
        let eq79_e1222_d_b12: f64 = (((p.p6 * s.db[56][12]) * s.v[257]) + (eq79_e1220 * s.db[257][12]));
        let eq79_e1222_d_b13: f64 = (((p.p6 * s.db[56][13]) * s.v[257]) + (eq79_e1220 * s.db[257][13]));
        let eq79_e1222_d_b14: f64 = (((p.p6 * s.db[56][14]) * s.v[257]) + (eq79_e1220 * s.db[257][14]));
        let eq79_e1222_d_b15: f64 = (((p.p6 * s.db[56][15]) * s.v[257]) + (eq79_e1220 * s.db[257][15]));
        let eq79_e1222_d_b16: f64 = (((p.p6 * s.db[56][16]) * s.v[257]) + (eq79_e1220 * s.db[257][16]));
        let eq79_e1222_d_b17: f64 = (((p.p6 * s.db[56][17]) * s.v[257]) + (eq79_e1220 * s.db[257][17]));
        let eq79_e1222_d_b18: f64 = (((p.p6 * s.db[56][18]) * s.v[257]) + (eq79_e1220 * s.db[257][18]));
        let eq79_e1222_d_b19: f64 = (((p.p6 * s.db[56][19]) * s.v[257]) + (eq79_e1220 * s.db[257][19]));
        let eq79_e1222_d_b20: f64 = (((p.p6 * s.db[56][20]) * s.v[257]) + (eq79_e1220 * s.db[257][20]));
        let eq79_e1222_d_b21: f64 = (((p.p6 * s.db[56][21]) * s.v[257]) + (eq79_e1220 * s.db[257][21]));
        let eq79_e1222_d_b22: f64 = (((p.p6 * s.db[56][22]) * s.v[257]) + (eq79_e1220 * s.db[257][22]));
        let eq79_e1222_d_b23: f64 = (((p.p6 * s.db[56][23]) * s.v[257]) + (eq79_e1220 * s.db[257][23]));
        let eq79_e1222_d_b24: f64 = (((p.p6 * s.db[56][24]) * s.v[257]) + (eq79_e1220 * s.db[257][24]));
        let eq79_e1222_d_b25: f64 = (((p.p6 * s.db[56][25]) * s.v[257]) + (eq79_e1220 * s.db[257][25]));
        let eq79_e1222_d_b26: f64 = (((p.p6 * s.db[56][26]) * s.v[257]) + (eq79_e1220 * s.db[257][26]));
        let eq79_e1222_d_b27: f64 = (((p.p6 * s.db[56][27]) * s.v[257]) + (eq79_e1220 * s.db[257][27]));
        let eq79_e1222_d_b28: f64 = (((p.p6 * s.db[56][28]) * s.v[257]) + (eq79_e1220 * s.db[257][28]));
        let eq79_e1222_d_b29: f64 = (((p.p6 * s.db[56][29]) * s.v[257]) + (eq79_e1220 * s.db[257][29]));
        let eq79_e1222_d_b30: f64 = (((p.p6 * s.db[56][30]) * s.v[257]) + (eq79_e1220 * s.db[257][30]));
        let eq79_e1222_d_b31: f64 = (((p.p6 * s.db[56][31]) * s.v[257]) + (eq79_e1220 * s.db[257][31]));
        let eq79_e1222_d_b32: f64 = (((p.p6 * s.db[56][32]) * s.v[257]) + (eq79_e1220 * s.db[257][32]));
        let eq79_e1222_d_b33: f64 = (((p.p6 * s.db[56][33]) * s.v[257]) + (eq79_e1220 * s.db[257][33]));
        let eq79_e1222_d_b34: f64 = (((p.p6 * s.db[56][34]) * s.v[257]) + (eq79_e1220 * s.db[257][34]));
        let eq79_e1222_d_b35: f64 = (((p.p6 * s.db[56][35]) * s.v[257]) + (eq79_e1220 * s.db[257][35]));
        let eq79_e1222_d_b36: f64 = (((p.p6 * s.db[56][36]) * s.v[257]) + (eq79_e1220 * s.db[257][36]));
        let eq79_e1222_d_b37: f64 = (((p.p6 * s.db[56][37]) * s.v[257]) + (eq79_e1220 * s.db[257][37]));
        let eq79_e1222_d_b38: f64 = (((p.p6 * s.db[56][38]) * s.v[257]) + (eq79_e1220 * s.db[257][38]));
        let eq79_e1222_d_b39: f64 = (((p.p6 * s.db[56][39]) * s.v[257]) + (eq79_e1220 * s.db[257][39]));
        let eq79_e1222_d_b40: f64 = (((p.p6 * s.db[56][40]) * s.v[257]) + (eq79_e1220 * s.db[257][40]));
        let eq79_e1222_d_b41: f64 = (((p.p6 * s.db[56][41]) * s.v[257]) + (eq79_e1220 * s.db[257][41]));
        let eq79_e1222_d_b42: f64 = (((p.p6 * s.db[56][42]) * s.v[257]) + (eq79_e1220 * s.db[257][42]));
        let eq79_e1222_d_b43: f64 = (((p.p6 * s.db[56][43]) * s.v[257]) + (eq79_e1220 * s.db[257][43]));
        let eq79_e1222_d_b44: f64 = (((p.p6 * s.db[56][44]) * s.v[257]) + (eq79_e1220 * s.db[257][44]));
        let eq79_e1222_d_b45: f64 = (((p.p6 * s.db[56][45]) * s.v[257]) + (eq79_e1220 * s.db[257][45]));
        let eq79_e1222_d_b46: f64 = (((p.p6 * s.db[56][46]) * s.v[257]) + (eq79_e1220 * s.db[257][46]));
        let eq79_e1222_d_b47: f64 = (((p.p6 * s.db[56][47]) * s.v[257]) + (eq79_e1220 * s.db[257][47]));
        let eq79_e1222_d_b48: f64 = (((p.p6 * s.db[56][48]) * s.v[257]) + (eq79_e1220 * s.db[257][48]));
        let eq79_e1222_d_b49: f64 = (((p.p6 * s.db[56][49]) * s.v[257]) + (eq79_e1220 * s.db[257][49]));
        let eq79_e1222_d_b50: f64 = (((p.p6 * s.db[56][50]) * s.v[257]) + (eq79_e1220 * s.db[257][50]));
        let eq79_e1222_d_b51: f64 = (((p.p6 * s.db[56][51]) * s.v[257]) + (eq79_e1220 * s.db[257][51]));
        let eq79_e1222_d_b52: f64 = (((p.p6 * s.db[56][52]) * s.v[257]) + (eq79_e1220 * s.db[257][52]));
        let eq79_e1222_d_b53: f64 = (((p.p6 * s.db[56][53]) * s.v[257]) + (eq79_e1220 * s.db[257][53]));
        let eq79_e1222_d_b54: f64 = (((p.p6 * s.db[56][54]) * s.v[257]) + (eq79_e1220 * s.db[257][54]));
        let eq79_e1225: f64 = (p.p6 * s.v[379]);
        let eq79_e1227: f64 = (eq79_e1225 * (nv16 - nv15));
        let eq79_e1227_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv16 - nv15));
        let eq79_e1227_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv16 - nv15));
        let eq79_e1227_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv16 - nv15));
        let eq79_e1227_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv16 - nv15));
        let eq79_e1227_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv16 - nv15));
        let eq79_e1227_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv16 - nv15));
        let eq79_e1227_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv16 - nv15));
        let eq79_e1227_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv16 - nv15));
        let eq79_e1227_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv16 - nv15));
        let eq79_e1227_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv16 - nv15));
        let eq79_e1227_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv16 - nv15));
        let eq79_e1227_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv16 - nv15));
        let eq79_e1227_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv16 - nv15));
        let eq79_e1227_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv16 - nv15));
        let eq79_e1227_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv16 - nv15));
        let eq79_e1227_d_n15: f64 = (((p.p6 * s.dn[379][15]) * (nv16 - nv15)) + (-eq79_e1225));
        let eq79_e1227_d_n16: f64 = (((p.p6 * s.dn[379][16]) * (nv16 - nv15)) + eq79_e1225);
        let eq79_e1227_d_n17: f64 = ((p.p6 * s.dn[379][17]) * (nv16 - nv15));
        let eq79_e1227_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv16 - nv15));
        let eq79_e1227_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv16 - nv15));
        let eq79_e1227_d_n20: f64 = ((p.p6 * s.dn[379][20]) * (nv16 - nv15));
        let eq79_e1227_d_n21: f64 = ((p.p6 * s.dn[379][21]) * (nv16 - nv15));
        let eq79_e1227_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv16 - nv15));
        let eq79_e1227_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv16 - nv15));
        let eq79_e1227_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv16 - nv15));
        let eq79_e1227_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv16 - nv15));
        let eq79_e1227_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv16 - nv15));
        let eq79_e1227_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv16 - nv15));
        let eq79_e1227_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv16 - nv15));
        let eq79_e1227_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv16 - nv15));
        let eq79_e1227_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv16 - nv15));
        let eq79_e1227_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv16 - nv15));
        let eq79_e1227_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv16 - nv15));
        let eq79_e1227_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv16 - nv15));
        let eq79_e1227_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv16 - nv15));
        let eq79_e1227_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv16 - nv15));
        let eq79_e1227_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv16 - nv15));
        let eq79_e1227_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv16 - nv15));
        let eq79_e1227_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv16 - nv15));
        let eq79_e1227_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv16 - nv15));
        let eq79_e1227_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv16 - nv15));
        let eq79_e1227_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv16 - nv15));
        let eq79_e1227_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv16 - nv15));
        let eq79_e1227_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv16 - nv15));
        let eq79_e1227_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv16 - nv15));
        let eq79_e1227_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv16 - nv15));
        let eq79_e1227_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv16 - nv15));
        let eq79_e1227_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv16 - nv15));
        let eq79_e1227_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv16 - nv15));
        let eq79_e1227_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv16 - nv15));
        let eq79_e1227_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv16 - nv15));
        let eq79_e1227_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv16 - nv15));
        let eq79_e1227_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv16 - nv15));
        let eq79_e1227_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv16 - nv15));
        let eq79_e1227_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv16 - nv15));
        let eq79_e1227_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv16 - nv15));
        let eq79_e1227_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv16 - nv15));
        let eq79_e1227_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv16 - nv15));
        let eq79_e1227_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv16 - nv15));
        let eq79_e1227_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv16 - nv15));
        let eq79_e1227_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv16 - nv15));
        let eq79_e1227_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv16 - nv15));
        let eq79_e1227_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv16 - nv15));
        let eq79_e1227_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv16 - nv15));
        let eq79_e1227_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv16 - nv15));
        let eq79_e1227_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv16 - nv15));
        let eq79_e1227_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv16 - nv15));
        let eq79_e1227_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv16 - nv15));
        let eq79_e1227_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv16 - nv15));
        let eq79_e1227_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv16 - nv15));
        let eq79_e1227_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv16 - nv15));
        let eq79_e1227_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv16 - nv15));
        let eq79_e1227_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv16 - nv15));
        let eq79_e1227_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv16 - nv15));
        let eq79_e1227_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv16 - nv15));
        let eq79_e1227_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv16 - nv15));
        let eq79_e1227_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv16 - nv15));
        let eq79_e1227_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv16 - nv15));
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
    }

    pub(super) fn stamp_transient_equations_block_9(
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
        let eq82_e1250: f64 = (eq82_e1248 * s.v[269]);
        let eq82_e1250_d_n0: f64 = (((p.p6 * s.dn[60][0]) * s.v[269]) + (eq82_e1248 * s.dn[269][0]));
        let eq82_e1250_d_n1: f64 = (((p.p6 * s.dn[60][1]) * s.v[269]) + (eq82_e1248 * s.dn[269][1]));
        let eq82_e1250_d_n2: f64 = (((p.p6 * s.dn[60][2]) * s.v[269]) + (eq82_e1248 * s.dn[269][2]));
        let eq82_e1250_d_n3: f64 = (((p.p6 * s.dn[60][3]) * s.v[269]) + (eq82_e1248 * s.dn[269][3]));
        let eq82_e1250_d_n4: f64 = (((p.p6 * s.dn[60][4]) * s.v[269]) + (eq82_e1248 * s.dn[269][4]));
        let eq82_e1250_d_n5: f64 = (((p.p6 * s.dn[60][5]) * s.v[269]) + (eq82_e1248 * s.dn[269][5]));
        let eq82_e1250_d_n6: f64 = (((p.p6 * s.dn[60][6]) * s.v[269]) + (eq82_e1248 * s.dn[269][6]));
        let eq82_e1250_d_n7: f64 = (((p.p6 * s.dn[60][7]) * s.v[269]) + (eq82_e1248 * s.dn[269][7]));
        let eq82_e1250_d_n8: f64 = (((p.p6 * s.dn[60][8]) * s.v[269]) + (eq82_e1248 * s.dn[269][8]));
        let eq82_e1250_d_n9: f64 = (((p.p6 * s.dn[60][9]) * s.v[269]) + (eq82_e1248 * s.dn[269][9]));
        let eq82_e1250_d_n10: f64 = (((p.p6 * s.dn[60][10]) * s.v[269]) + (eq82_e1248 * s.dn[269][10]));
        let eq82_e1250_d_n11: f64 = (((p.p6 * s.dn[60][11]) * s.v[269]) + (eq82_e1248 * s.dn[269][11]));
        let eq82_e1250_d_n12: f64 = (((p.p6 * s.dn[60][12]) * s.v[269]) + (eq82_e1248 * s.dn[269][12]));
        let eq82_e1250_d_n13: f64 = (((p.p6 * s.dn[60][13]) * s.v[269]) + (eq82_e1248 * s.dn[269][13]));
        let eq82_e1250_d_n14: f64 = (((p.p6 * s.dn[60][14]) * s.v[269]) + (eq82_e1248 * s.dn[269][14]));
        let eq82_e1250_d_n15: f64 = (((p.p6 * s.dn[60][15]) * s.v[269]) + (eq82_e1248 * s.dn[269][15]));
        let eq82_e1250_d_n16: f64 = (((p.p6 * s.dn[60][16]) * s.v[269]) + (eq82_e1248 * s.dn[269][16]));
        let eq82_e1250_d_n17: f64 = (((p.p6 * s.dn[60][17]) * s.v[269]) + (eq82_e1248 * s.dn[269][17]));
        let eq82_e1250_d_n18: f64 = (((p.p6 * s.dn[60][18]) * s.v[269]) + (eq82_e1248 * s.dn[269][18]));
        let eq82_e1250_d_n19: f64 = (((p.p6 * s.dn[60][19]) * s.v[269]) + (eq82_e1248 * s.dn[269][19]));
        let eq82_e1250_d_n20: f64 = (((p.p6 * s.dn[60][20]) * s.v[269]) + (eq82_e1248 * s.dn[269][20]));
        let eq82_e1250_d_n21: f64 = (((p.p6 * s.dn[60][21]) * s.v[269]) + (eq82_e1248 * s.dn[269][21]));
        let eq82_e1250_d_n22: f64 = (((p.p6 * s.dn[60][22]) * s.v[269]) + (eq82_e1248 * s.dn[269][22]));
        let eq82_e1250_d_b0: f64 = (((p.p6 * s.db[60][0]) * s.v[269]) + (eq82_e1248 * s.db[269][0]));
        let eq82_e1250_d_b1: f64 = (((p.p6 * s.db[60][1]) * s.v[269]) + (eq82_e1248 * s.db[269][1]));
        let eq82_e1250_d_b2: f64 = (((p.p6 * s.db[60][2]) * s.v[269]) + (eq82_e1248 * s.db[269][2]));
        let eq82_e1250_d_b3: f64 = (((p.p6 * s.db[60][3]) * s.v[269]) + (eq82_e1248 * s.db[269][3]));
        let eq82_e1250_d_b4: f64 = (((p.p6 * s.db[60][4]) * s.v[269]) + (eq82_e1248 * s.db[269][4]));
        let eq82_e1250_d_b5: f64 = (((p.p6 * s.db[60][5]) * s.v[269]) + (eq82_e1248 * s.db[269][5]));
        let eq82_e1250_d_b6: f64 = (((p.p6 * s.db[60][6]) * s.v[269]) + (eq82_e1248 * s.db[269][6]));
        let eq82_e1250_d_b7: f64 = (((p.p6 * s.db[60][7]) * s.v[269]) + (eq82_e1248 * s.db[269][7]));
        let eq82_e1250_d_b8: f64 = (((p.p6 * s.db[60][8]) * s.v[269]) + (eq82_e1248 * s.db[269][8]));
        let eq82_e1250_d_b9: f64 = (((p.p6 * s.db[60][9]) * s.v[269]) + (eq82_e1248 * s.db[269][9]));
        let eq82_e1250_d_b10: f64 = (((p.p6 * s.db[60][10]) * s.v[269]) + (eq82_e1248 * s.db[269][10]));
        let eq82_e1250_d_b11: f64 = (((p.p6 * s.db[60][11]) * s.v[269]) + (eq82_e1248 * s.db[269][11]));
        let eq82_e1250_d_b12: f64 = (((p.p6 * s.db[60][12]) * s.v[269]) + (eq82_e1248 * s.db[269][12]));
        let eq82_e1250_d_b13: f64 = (((p.p6 * s.db[60][13]) * s.v[269]) + (eq82_e1248 * s.db[269][13]));
        let eq82_e1250_d_b14: f64 = (((p.p6 * s.db[60][14]) * s.v[269]) + (eq82_e1248 * s.db[269][14]));
        let eq82_e1250_d_b15: f64 = (((p.p6 * s.db[60][15]) * s.v[269]) + (eq82_e1248 * s.db[269][15]));
        let eq82_e1250_d_b16: f64 = (((p.p6 * s.db[60][16]) * s.v[269]) + (eq82_e1248 * s.db[269][16]));
        let eq82_e1250_d_b17: f64 = (((p.p6 * s.db[60][17]) * s.v[269]) + (eq82_e1248 * s.db[269][17]));
        let eq82_e1250_d_b18: f64 = (((p.p6 * s.db[60][18]) * s.v[269]) + (eq82_e1248 * s.db[269][18]));
        let eq82_e1250_d_b19: f64 = (((p.p6 * s.db[60][19]) * s.v[269]) + (eq82_e1248 * s.db[269][19]));
        let eq82_e1250_d_b20: f64 = (((p.p6 * s.db[60][20]) * s.v[269]) + (eq82_e1248 * s.db[269][20]));
        let eq82_e1250_d_b21: f64 = (((p.p6 * s.db[60][21]) * s.v[269]) + (eq82_e1248 * s.db[269][21]));
        let eq82_e1250_d_b22: f64 = (((p.p6 * s.db[60][22]) * s.v[269]) + (eq82_e1248 * s.db[269][22]));
        let eq82_e1250_d_b23: f64 = (((p.p6 * s.db[60][23]) * s.v[269]) + (eq82_e1248 * s.db[269][23]));
        let eq82_e1250_d_b24: f64 = (((p.p6 * s.db[60][24]) * s.v[269]) + (eq82_e1248 * s.db[269][24]));
        let eq82_e1250_d_b25: f64 = (((p.p6 * s.db[60][25]) * s.v[269]) + (eq82_e1248 * s.db[269][25]));
        let eq82_e1250_d_b26: f64 = (((p.p6 * s.db[60][26]) * s.v[269]) + (eq82_e1248 * s.db[269][26]));
        let eq82_e1250_d_b27: f64 = (((p.p6 * s.db[60][27]) * s.v[269]) + (eq82_e1248 * s.db[269][27]));
        let eq82_e1250_d_b28: f64 = (((p.p6 * s.db[60][28]) * s.v[269]) + (eq82_e1248 * s.db[269][28]));
        let eq82_e1250_d_b29: f64 = (((p.p6 * s.db[60][29]) * s.v[269]) + (eq82_e1248 * s.db[269][29]));
        let eq82_e1250_d_b30: f64 = (((p.p6 * s.db[60][30]) * s.v[269]) + (eq82_e1248 * s.db[269][30]));
        let eq82_e1250_d_b31: f64 = (((p.p6 * s.db[60][31]) * s.v[269]) + (eq82_e1248 * s.db[269][31]));
        let eq82_e1250_d_b32: f64 = (((p.p6 * s.db[60][32]) * s.v[269]) + (eq82_e1248 * s.db[269][32]));
        let eq82_e1250_d_b33: f64 = (((p.p6 * s.db[60][33]) * s.v[269]) + (eq82_e1248 * s.db[269][33]));
        let eq82_e1250_d_b34: f64 = (((p.p6 * s.db[60][34]) * s.v[269]) + (eq82_e1248 * s.db[269][34]));
        let eq82_e1250_d_b35: f64 = (((p.p6 * s.db[60][35]) * s.v[269]) + (eq82_e1248 * s.db[269][35]));
        let eq82_e1250_d_b36: f64 = (((p.p6 * s.db[60][36]) * s.v[269]) + (eq82_e1248 * s.db[269][36]));
        let eq82_e1250_d_b37: f64 = (((p.p6 * s.db[60][37]) * s.v[269]) + (eq82_e1248 * s.db[269][37]));
        let eq82_e1250_d_b38: f64 = (((p.p6 * s.db[60][38]) * s.v[269]) + (eq82_e1248 * s.db[269][38]));
        let eq82_e1250_d_b39: f64 = (((p.p6 * s.db[60][39]) * s.v[269]) + (eq82_e1248 * s.db[269][39]));
        let eq82_e1250_d_b40: f64 = (((p.p6 * s.db[60][40]) * s.v[269]) + (eq82_e1248 * s.db[269][40]));
        let eq82_e1250_d_b41: f64 = (((p.p6 * s.db[60][41]) * s.v[269]) + (eq82_e1248 * s.db[269][41]));
        let eq82_e1250_d_b42: f64 = (((p.p6 * s.db[60][42]) * s.v[269]) + (eq82_e1248 * s.db[269][42]));
        let eq82_e1250_d_b43: f64 = (((p.p6 * s.db[60][43]) * s.v[269]) + (eq82_e1248 * s.db[269][43]));
        let eq82_e1250_d_b44: f64 = (((p.p6 * s.db[60][44]) * s.v[269]) + (eq82_e1248 * s.db[269][44]));
        let eq82_e1250_d_b45: f64 = (((p.p6 * s.db[60][45]) * s.v[269]) + (eq82_e1248 * s.db[269][45]));
        let eq82_e1250_d_b46: f64 = (((p.p6 * s.db[60][46]) * s.v[269]) + (eq82_e1248 * s.db[269][46]));
        let eq82_e1250_d_b47: f64 = (((p.p6 * s.db[60][47]) * s.v[269]) + (eq82_e1248 * s.db[269][47]));
        let eq82_e1250_d_b48: f64 = (((p.p6 * s.db[60][48]) * s.v[269]) + (eq82_e1248 * s.db[269][48]));
        let eq82_e1250_d_b49: f64 = (((p.p6 * s.db[60][49]) * s.v[269]) + (eq82_e1248 * s.db[269][49]));
        let eq82_e1250_d_b50: f64 = (((p.p6 * s.db[60][50]) * s.v[269]) + (eq82_e1248 * s.db[269][50]));
        let eq82_e1250_d_b51: f64 = (((p.p6 * s.db[60][51]) * s.v[269]) + (eq82_e1248 * s.db[269][51]));
        let eq82_e1250_d_b52: f64 = (((p.p6 * s.db[60][52]) * s.v[269]) + (eq82_e1248 * s.db[269][52]));
        let eq82_e1250_d_b53: f64 = (((p.p6 * s.db[60][53]) * s.v[269]) + (eq82_e1248 * s.db[269][53]));
        let eq82_e1250_d_b54: f64 = (((p.p6 * s.db[60][54]) * s.v[269]) + (eq82_e1248 * s.db[269][54]));
        let eq82_e1253: f64 = (p.p6 * s.v[379]);
        let eq82_e1255: f64 = (eq82_e1253 * (nv19 - nv20));
        let eq82_e1255_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv19 - nv20));
        let eq82_e1255_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv19 - nv20));
        let eq82_e1255_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv19 - nv20));
        let eq82_e1255_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv19 - nv20));
        let eq82_e1255_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv19 - nv20));
        let eq82_e1255_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv19 - nv20));
        let eq82_e1255_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv19 - nv20));
        let eq82_e1255_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv19 - nv20));
        let eq82_e1255_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv19 - nv20));
        let eq82_e1255_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv19 - nv20));
        let eq82_e1255_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv19 - nv20));
        let eq82_e1255_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv19 - nv20));
        let eq82_e1255_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv19 - nv20));
        let eq82_e1255_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv19 - nv20));
        let eq82_e1255_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv19 - nv20));
        let eq82_e1255_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv19 - nv20));
        let eq82_e1255_d_n16: f64 = ((p.p6 * s.dn[379][16]) * (nv19 - nv20));
        let eq82_e1255_d_n17: f64 = ((p.p6 * s.dn[379][17]) * (nv19 - nv20));
        let eq82_e1255_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv19 - nv20));
        let eq82_e1255_d_n19: f64 = (((p.p6 * s.dn[379][19]) * (nv19 - nv20)) + eq82_e1253);
        let eq82_e1255_d_n20: f64 = (((p.p6 * s.dn[379][20]) * (nv19 - nv20)) + (-eq82_e1253));
        let eq82_e1255_d_n21: f64 = ((p.p6 * s.dn[379][21]) * (nv19 - nv20));
        let eq82_e1255_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv19 - nv20));
        let eq82_e1255_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv19 - nv20));
        let eq82_e1255_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv19 - nv20));
        let eq82_e1255_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv19 - nv20));
        let eq82_e1255_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv19 - nv20));
        let eq82_e1255_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv19 - nv20));
        let eq82_e1255_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv19 - nv20));
        let eq82_e1255_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv19 - nv20));
        let eq82_e1255_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv19 - nv20));
        let eq82_e1255_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv19 - nv20));
        let eq82_e1255_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv19 - nv20));
        let eq82_e1255_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv19 - nv20));
        let eq82_e1255_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv19 - nv20));
        let eq82_e1255_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv19 - nv20));
        let eq82_e1255_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv19 - nv20));
        let eq82_e1255_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv19 - nv20));
        let eq82_e1255_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv19 - nv20));
        let eq82_e1255_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv19 - nv20));
        let eq82_e1255_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv19 - nv20));
        let eq82_e1255_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv19 - nv20));
        let eq82_e1255_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv19 - nv20));
        let eq82_e1255_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv19 - nv20));
        let eq82_e1255_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv19 - nv20));
        let eq82_e1255_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv19 - nv20));
        let eq82_e1255_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv19 - nv20));
        let eq82_e1255_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv19 - nv20));
        let eq82_e1255_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv19 - nv20));
        let eq82_e1255_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv19 - nv20));
        let eq82_e1255_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv19 - nv20));
        let eq82_e1255_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv19 - nv20));
        let eq82_e1255_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv19 - nv20));
        let eq82_e1255_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv19 - nv20));
        let eq82_e1255_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv19 - nv20));
        let eq82_e1255_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv19 - nv20));
        let eq82_e1255_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv19 - nv20));
        let eq82_e1255_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv19 - nv20));
        let eq82_e1255_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv19 - nv20));
        let eq82_e1255_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv19 - nv20));
        let eq82_e1255_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv19 - nv20));
        let eq82_e1255_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv19 - nv20));
        let eq82_e1255_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv19 - nv20));
        let eq82_e1255_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv19 - nv20));
        let eq82_e1255_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv19 - nv20));
        let eq82_e1255_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv19 - nv20));
        let eq82_e1255_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv19 - nv20));
        let eq82_e1255_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv19 - nv20));
        let eq82_e1255_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv19 - nv20));
        let eq82_e1255_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv19 - nv20));
        let eq82_e1255_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv19 - nv20));
        let eq82_e1255_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv19 - nv20));
        let eq82_e1255_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv19 - nv20));
        let eq82_e1255_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv19 - nv20));
        let eq82_e1255_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv19 - nv20));
        let eq82_e1255_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv19 - nv20));
        let eq82_e1255_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv19 - nv20));
        let eq82_e1255_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv19 - nv20));
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
    }

    pub(super) fn stamp_transient_equations_block_10(
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
        let eq86_e1286: f64 = (eq86_e1284 * s.v[281]);
        let eq86_e1286_d_n0: f64 = (((p.p6 * s.dn[64][0]) * s.v[281]) + (eq86_e1284 * s.dn[281][0]));
        let eq86_e1286_d_n1: f64 = (((p.p6 * s.dn[64][1]) * s.v[281]) + (eq86_e1284 * s.dn[281][1]));
        let eq86_e1286_d_n2: f64 = (((p.p6 * s.dn[64][2]) * s.v[281]) + (eq86_e1284 * s.dn[281][2]));
        let eq86_e1286_d_n3: f64 = (((p.p6 * s.dn[64][3]) * s.v[281]) + (eq86_e1284 * s.dn[281][3]));
        let eq86_e1286_d_n4: f64 = (((p.p6 * s.dn[64][4]) * s.v[281]) + (eq86_e1284 * s.dn[281][4]));
        let eq86_e1286_d_n5: f64 = (((p.p6 * s.dn[64][5]) * s.v[281]) + (eq86_e1284 * s.dn[281][5]));
        let eq86_e1286_d_n6: f64 = (((p.p6 * s.dn[64][6]) * s.v[281]) + (eq86_e1284 * s.dn[281][6]));
        let eq86_e1286_d_n7: f64 = (((p.p6 * s.dn[64][7]) * s.v[281]) + (eq86_e1284 * s.dn[281][7]));
        let eq86_e1286_d_n8: f64 = (((p.p6 * s.dn[64][8]) * s.v[281]) + (eq86_e1284 * s.dn[281][8]));
        let eq86_e1286_d_n9: f64 = (((p.p6 * s.dn[64][9]) * s.v[281]) + (eq86_e1284 * s.dn[281][9]));
        let eq86_e1286_d_n10: f64 = (((p.p6 * s.dn[64][10]) * s.v[281]) + (eq86_e1284 * s.dn[281][10]));
        let eq86_e1286_d_n11: f64 = (((p.p6 * s.dn[64][11]) * s.v[281]) + (eq86_e1284 * s.dn[281][11]));
        let eq86_e1286_d_n12: f64 = (((p.p6 * s.dn[64][12]) * s.v[281]) + (eq86_e1284 * s.dn[281][12]));
        let eq86_e1286_d_n13: f64 = (((p.p6 * s.dn[64][13]) * s.v[281]) + (eq86_e1284 * s.dn[281][13]));
        let eq86_e1286_d_n14: f64 = (((p.p6 * s.dn[64][14]) * s.v[281]) + (eq86_e1284 * s.dn[281][14]));
        let eq86_e1286_d_n15: f64 = (((p.p6 * s.dn[64][15]) * s.v[281]) + (eq86_e1284 * s.dn[281][15]));
        let eq86_e1286_d_n16: f64 = (((p.p6 * s.dn[64][16]) * s.v[281]) + (eq86_e1284 * s.dn[281][16]));
        let eq86_e1286_d_n17: f64 = (((p.p6 * s.dn[64][17]) * s.v[281]) + (eq86_e1284 * s.dn[281][17]));
        let eq86_e1286_d_n18: f64 = (((p.p6 * s.dn[64][18]) * s.v[281]) + (eq86_e1284 * s.dn[281][18]));
        let eq86_e1286_d_n19: f64 = (((p.p6 * s.dn[64][19]) * s.v[281]) + (eq86_e1284 * s.dn[281][19]));
        let eq86_e1286_d_n20: f64 = (((p.p6 * s.dn[64][20]) * s.v[281]) + (eq86_e1284 * s.dn[281][20]));
        let eq86_e1286_d_n21: f64 = (((p.p6 * s.dn[64][21]) * s.v[281]) + (eq86_e1284 * s.dn[281][21]));
        let eq86_e1286_d_n22: f64 = (((p.p6 * s.dn[64][22]) * s.v[281]) + (eq86_e1284 * s.dn[281][22]));
        let eq86_e1286_d_b0: f64 = (((p.p6 * s.db[64][0]) * s.v[281]) + (eq86_e1284 * s.db[281][0]));
        let eq86_e1286_d_b1: f64 = (((p.p6 * s.db[64][1]) * s.v[281]) + (eq86_e1284 * s.db[281][1]));
        let eq86_e1286_d_b2: f64 = (((p.p6 * s.db[64][2]) * s.v[281]) + (eq86_e1284 * s.db[281][2]));
        let eq86_e1286_d_b3: f64 = (((p.p6 * s.db[64][3]) * s.v[281]) + (eq86_e1284 * s.db[281][3]));
        let eq86_e1286_d_b4: f64 = (((p.p6 * s.db[64][4]) * s.v[281]) + (eq86_e1284 * s.db[281][4]));
        let eq86_e1286_d_b5: f64 = (((p.p6 * s.db[64][5]) * s.v[281]) + (eq86_e1284 * s.db[281][5]));
        let eq86_e1286_d_b6: f64 = (((p.p6 * s.db[64][6]) * s.v[281]) + (eq86_e1284 * s.db[281][6]));
        let eq86_e1286_d_b7: f64 = (((p.p6 * s.db[64][7]) * s.v[281]) + (eq86_e1284 * s.db[281][7]));
        let eq86_e1286_d_b8: f64 = (((p.p6 * s.db[64][8]) * s.v[281]) + (eq86_e1284 * s.db[281][8]));
        let eq86_e1286_d_b9: f64 = (((p.p6 * s.db[64][9]) * s.v[281]) + (eq86_e1284 * s.db[281][9]));
        let eq86_e1286_d_b10: f64 = (((p.p6 * s.db[64][10]) * s.v[281]) + (eq86_e1284 * s.db[281][10]));
        let eq86_e1286_d_b11: f64 = (((p.p6 * s.db[64][11]) * s.v[281]) + (eq86_e1284 * s.db[281][11]));
        let eq86_e1286_d_b12: f64 = (((p.p6 * s.db[64][12]) * s.v[281]) + (eq86_e1284 * s.db[281][12]));
        let eq86_e1286_d_b13: f64 = (((p.p6 * s.db[64][13]) * s.v[281]) + (eq86_e1284 * s.db[281][13]));
        let eq86_e1286_d_b14: f64 = (((p.p6 * s.db[64][14]) * s.v[281]) + (eq86_e1284 * s.db[281][14]));
        let eq86_e1286_d_b15: f64 = (((p.p6 * s.db[64][15]) * s.v[281]) + (eq86_e1284 * s.db[281][15]));
        let eq86_e1286_d_b16: f64 = (((p.p6 * s.db[64][16]) * s.v[281]) + (eq86_e1284 * s.db[281][16]));
        let eq86_e1286_d_b17: f64 = (((p.p6 * s.db[64][17]) * s.v[281]) + (eq86_e1284 * s.db[281][17]));
        let eq86_e1286_d_b18: f64 = (((p.p6 * s.db[64][18]) * s.v[281]) + (eq86_e1284 * s.db[281][18]));
        let eq86_e1286_d_b19: f64 = (((p.p6 * s.db[64][19]) * s.v[281]) + (eq86_e1284 * s.db[281][19]));
        let eq86_e1286_d_b20: f64 = (((p.p6 * s.db[64][20]) * s.v[281]) + (eq86_e1284 * s.db[281][20]));
        let eq86_e1286_d_b21: f64 = (((p.p6 * s.db[64][21]) * s.v[281]) + (eq86_e1284 * s.db[281][21]));
        let eq86_e1286_d_b22: f64 = (((p.p6 * s.db[64][22]) * s.v[281]) + (eq86_e1284 * s.db[281][22]));
        let eq86_e1286_d_b23: f64 = (((p.p6 * s.db[64][23]) * s.v[281]) + (eq86_e1284 * s.db[281][23]));
        let eq86_e1286_d_b24: f64 = (((p.p6 * s.db[64][24]) * s.v[281]) + (eq86_e1284 * s.db[281][24]));
        let eq86_e1286_d_b25: f64 = (((p.p6 * s.db[64][25]) * s.v[281]) + (eq86_e1284 * s.db[281][25]));
        let eq86_e1286_d_b26: f64 = (((p.p6 * s.db[64][26]) * s.v[281]) + (eq86_e1284 * s.db[281][26]));
        let eq86_e1286_d_b27: f64 = (((p.p6 * s.db[64][27]) * s.v[281]) + (eq86_e1284 * s.db[281][27]));
        let eq86_e1286_d_b28: f64 = (((p.p6 * s.db[64][28]) * s.v[281]) + (eq86_e1284 * s.db[281][28]));
        let eq86_e1286_d_b29: f64 = (((p.p6 * s.db[64][29]) * s.v[281]) + (eq86_e1284 * s.db[281][29]));
        let eq86_e1286_d_b30: f64 = (((p.p6 * s.db[64][30]) * s.v[281]) + (eq86_e1284 * s.db[281][30]));
        let eq86_e1286_d_b31: f64 = (((p.p6 * s.db[64][31]) * s.v[281]) + (eq86_e1284 * s.db[281][31]));
        let eq86_e1286_d_b32: f64 = (((p.p6 * s.db[64][32]) * s.v[281]) + (eq86_e1284 * s.db[281][32]));
        let eq86_e1286_d_b33: f64 = (((p.p6 * s.db[64][33]) * s.v[281]) + (eq86_e1284 * s.db[281][33]));
        let eq86_e1286_d_b34: f64 = (((p.p6 * s.db[64][34]) * s.v[281]) + (eq86_e1284 * s.db[281][34]));
        let eq86_e1286_d_b35: f64 = (((p.p6 * s.db[64][35]) * s.v[281]) + (eq86_e1284 * s.db[281][35]));
        let eq86_e1286_d_b36: f64 = (((p.p6 * s.db[64][36]) * s.v[281]) + (eq86_e1284 * s.db[281][36]));
        let eq86_e1286_d_b37: f64 = (((p.p6 * s.db[64][37]) * s.v[281]) + (eq86_e1284 * s.db[281][37]));
        let eq86_e1286_d_b38: f64 = (((p.p6 * s.db[64][38]) * s.v[281]) + (eq86_e1284 * s.db[281][38]));
        let eq86_e1286_d_b39: f64 = (((p.p6 * s.db[64][39]) * s.v[281]) + (eq86_e1284 * s.db[281][39]));
        let eq86_e1286_d_b40: f64 = (((p.p6 * s.db[64][40]) * s.v[281]) + (eq86_e1284 * s.db[281][40]));
        let eq86_e1286_d_b41: f64 = (((p.p6 * s.db[64][41]) * s.v[281]) + (eq86_e1284 * s.db[281][41]));
        let eq86_e1286_d_b42: f64 = (((p.p6 * s.db[64][42]) * s.v[281]) + (eq86_e1284 * s.db[281][42]));
        let eq86_e1286_d_b43: f64 = (((p.p6 * s.db[64][43]) * s.v[281]) + (eq86_e1284 * s.db[281][43]));
        let eq86_e1286_d_b44: f64 = (((p.p6 * s.db[64][44]) * s.v[281]) + (eq86_e1284 * s.db[281][44]));
        let eq86_e1286_d_b45: f64 = (((p.p6 * s.db[64][45]) * s.v[281]) + (eq86_e1284 * s.db[281][45]));
        let eq86_e1286_d_b46: f64 = (((p.p6 * s.db[64][46]) * s.v[281]) + (eq86_e1284 * s.db[281][46]));
        let eq86_e1286_d_b47: f64 = (((p.p6 * s.db[64][47]) * s.v[281]) + (eq86_e1284 * s.db[281][47]));
        let eq86_e1286_d_b48: f64 = (((p.p6 * s.db[64][48]) * s.v[281]) + (eq86_e1284 * s.db[281][48]));
        let eq86_e1286_d_b49: f64 = (((p.p6 * s.db[64][49]) * s.v[281]) + (eq86_e1284 * s.db[281][49]));
        let eq86_e1286_d_b50: f64 = (((p.p6 * s.db[64][50]) * s.v[281]) + (eq86_e1284 * s.db[281][50]));
        let eq86_e1286_d_b51: f64 = (((p.p6 * s.db[64][51]) * s.v[281]) + (eq86_e1284 * s.db[281][51]));
        let eq86_e1286_d_b52: f64 = (((p.p6 * s.db[64][52]) * s.v[281]) + (eq86_e1284 * s.db[281][52]));
        let eq86_e1286_d_b53: f64 = (((p.p6 * s.db[64][53]) * s.v[281]) + (eq86_e1284 * s.db[281][53]));
        let eq86_e1286_d_b54: f64 = (((p.p6 * s.db[64][54]) * s.v[281]) + (eq86_e1284 * s.db[281][54]));
        let eq86_e1289: f64 = (p.p6 * s.v[379]);
        let eq86_e1291: f64 = (eq86_e1289 * (nv17 - nv16));
        let eq86_e1291_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv17 - nv16));
        let eq86_e1291_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv17 - nv16));
        let eq86_e1291_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv17 - nv16));
        let eq86_e1291_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv17 - nv16));
        let eq86_e1291_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv17 - nv16));
        let eq86_e1291_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv17 - nv16));
        let eq86_e1291_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv17 - nv16));
        let eq86_e1291_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv17 - nv16));
        let eq86_e1291_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv17 - nv16));
        let eq86_e1291_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv17 - nv16));
        let eq86_e1291_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv17 - nv16));
        let eq86_e1291_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv17 - nv16));
        let eq86_e1291_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv17 - nv16));
        let eq86_e1291_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv17 - nv16));
        let eq86_e1291_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv17 - nv16));
        let eq86_e1291_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv17 - nv16));
        let eq86_e1291_d_n16: f64 = (((p.p6 * s.dn[379][16]) * (nv17 - nv16)) + (-eq86_e1289));
        let eq86_e1291_d_n17: f64 = (((p.p6 * s.dn[379][17]) * (nv17 - nv16)) + eq86_e1289);
        let eq86_e1291_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv17 - nv16));
        let eq86_e1291_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv17 - nv16));
        let eq86_e1291_d_n20: f64 = ((p.p6 * s.dn[379][20]) * (nv17 - nv16));
        let eq86_e1291_d_n21: f64 = ((p.p6 * s.dn[379][21]) * (nv17 - nv16));
        let eq86_e1291_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv17 - nv16));
        let eq86_e1291_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv17 - nv16));
        let eq86_e1291_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv17 - nv16));
        let eq86_e1291_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv17 - nv16));
        let eq86_e1291_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv17 - nv16));
        let eq86_e1291_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv17 - nv16));
        let eq86_e1291_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv17 - nv16));
        let eq86_e1291_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv17 - nv16));
        let eq86_e1291_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv17 - nv16));
        let eq86_e1291_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv17 - nv16));
        let eq86_e1291_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv17 - nv16));
        let eq86_e1291_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv17 - nv16));
        let eq86_e1291_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv17 - nv16));
        let eq86_e1291_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv17 - nv16));
        let eq86_e1291_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv17 - nv16));
        let eq86_e1291_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv17 - nv16));
        let eq86_e1291_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv17 - nv16));
        let eq86_e1291_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv17 - nv16));
        let eq86_e1291_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv17 - nv16));
        let eq86_e1291_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv17 - nv16));
        let eq86_e1291_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv17 - nv16));
        let eq86_e1291_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv17 - nv16));
        let eq86_e1291_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv17 - nv16));
        let eq86_e1291_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv17 - nv16));
        let eq86_e1291_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv17 - nv16));
        let eq86_e1291_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv17 - nv16));
        let eq86_e1291_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv17 - nv16));
        let eq86_e1291_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv17 - nv16));
        let eq86_e1291_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv17 - nv16));
        let eq86_e1291_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv17 - nv16));
        let eq86_e1291_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv17 - nv16));
        let eq86_e1291_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv17 - nv16));
        let eq86_e1291_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv17 - nv16));
        let eq86_e1291_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv17 - nv16));
        let eq86_e1291_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv17 - nv16));
        let eq86_e1291_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv17 - nv16));
        let eq86_e1291_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv17 - nv16));
        let eq86_e1291_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv17 - nv16));
        let eq86_e1291_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv17 - nv16));
        let eq86_e1291_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv17 - nv16));
        let eq86_e1291_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv17 - nv16));
        let eq86_e1291_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv17 - nv16));
        let eq86_e1291_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv17 - nv16));
        let eq86_e1291_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv17 - nv16));
        let eq86_e1291_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv17 - nv16));
        let eq86_e1291_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv17 - nv16));
        let eq86_e1291_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv17 - nv16));
        let eq86_e1291_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv17 - nv16));
        let eq86_e1291_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv17 - nv16));
        let eq86_e1291_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv17 - nv16));
        let eq86_e1291_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv17 - nv16));
        let eq86_e1291_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv17 - nv16));
        let eq86_e1291_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv17 - nv16));
        let eq86_e1291_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv17 - nv16));
        let eq86_e1291_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv17 - nv16));
        let eq86_e1291_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv17 - nv16));
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
    }

    pub(super) fn stamp_transient_equations_block_11(
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
        let eq89_e1314: f64 = (eq89_e1312 * s.v[293]);
        let eq89_e1314_d_n0: f64 = (((p.p6 * s.dn[68][0]) * s.v[293]) + (eq89_e1312 * s.dn[293][0]));
        let eq89_e1314_d_n1: f64 = (((p.p6 * s.dn[68][1]) * s.v[293]) + (eq89_e1312 * s.dn[293][1]));
        let eq89_e1314_d_n2: f64 = (((p.p6 * s.dn[68][2]) * s.v[293]) + (eq89_e1312 * s.dn[293][2]));
        let eq89_e1314_d_n3: f64 = (((p.p6 * s.dn[68][3]) * s.v[293]) + (eq89_e1312 * s.dn[293][3]));
        let eq89_e1314_d_n4: f64 = (((p.p6 * s.dn[68][4]) * s.v[293]) + (eq89_e1312 * s.dn[293][4]));
        let eq89_e1314_d_n5: f64 = (((p.p6 * s.dn[68][5]) * s.v[293]) + (eq89_e1312 * s.dn[293][5]));
        let eq89_e1314_d_n6: f64 = (((p.p6 * s.dn[68][6]) * s.v[293]) + (eq89_e1312 * s.dn[293][6]));
        let eq89_e1314_d_n7: f64 = (((p.p6 * s.dn[68][7]) * s.v[293]) + (eq89_e1312 * s.dn[293][7]));
        let eq89_e1314_d_n8: f64 = (((p.p6 * s.dn[68][8]) * s.v[293]) + (eq89_e1312 * s.dn[293][8]));
        let eq89_e1314_d_n9: f64 = (((p.p6 * s.dn[68][9]) * s.v[293]) + (eq89_e1312 * s.dn[293][9]));
        let eq89_e1314_d_n10: f64 = (((p.p6 * s.dn[68][10]) * s.v[293]) + (eq89_e1312 * s.dn[293][10]));
        let eq89_e1314_d_n11: f64 = (((p.p6 * s.dn[68][11]) * s.v[293]) + (eq89_e1312 * s.dn[293][11]));
        let eq89_e1314_d_n12: f64 = (((p.p6 * s.dn[68][12]) * s.v[293]) + (eq89_e1312 * s.dn[293][12]));
        let eq89_e1314_d_n13: f64 = (((p.p6 * s.dn[68][13]) * s.v[293]) + (eq89_e1312 * s.dn[293][13]));
        let eq89_e1314_d_n14: f64 = (((p.p6 * s.dn[68][14]) * s.v[293]) + (eq89_e1312 * s.dn[293][14]));
        let eq89_e1314_d_n15: f64 = (((p.p6 * s.dn[68][15]) * s.v[293]) + (eq89_e1312 * s.dn[293][15]));
        let eq89_e1314_d_n16: f64 = (((p.p6 * s.dn[68][16]) * s.v[293]) + (eq89_e1312 * s.dn[293][16]));
        let eq89_e1314_d_n17: f64 = (((p.p6 * s.dn[68][17]) * s.v[293]) + (eq89_e1312 * s.dn[293][17]));
        let eq89_e1314_d_n18: f64 = (((p.p6 * s.dn[68][18]) * s.v[293]) + (eq89_e1312 * s.dn[293][18]));
        let eq89_e1314_d_n19: f64 = (((p.p6 * s.dn[68][19]) * s.v[293]) + (eq89_e1312 * s.dn[293][19]));
        let eq89_e1314_d_n20: f64 = (((p.p6 * s.dn[68][20]) * s.v[293]) + (eq89_e1312 * s.dn[293][20]));
        let eq89_e1314_d_n21: f64 = (((p.p6 * s.dn[68][21]) * s.v[293]) + (eq89_e1312 * s.dn[293][21]));
        let eq89_e1314_d_n22: f64 = (((p.p6 * s.dn[68][22]) * s.v[293]) + (eq89_e1312 * s.dn[293][22]));
        let eq89_e1314_d_b0: f64 = (((p.p6 * s.db[68][0]) * s.v[293]) + (eq89_e1312 * s.db[293][0]));
        let eq89_e1314_d_b1: f64 = (((p.p6 * s.db[68][1]) * s.v[293]) + (eq89_e1312 * s.db[293][1]));
        let eq89_e1314_d_b2: f64 = (((p.p6 * s.db[68][2]) * s.v[293]) + (eq89_e1312 * s.db[293][2]));
        let eq89_e1314_d_b3: f64 = (((p.p6 * s.db[68][3]) * s.v[293]) + (eq89_e1312 * s.db[293][3]));
        let eq89_e1314_d_b4: f64 = (((p.p6 * s.db[68][4]) * s.v[293]) + (eq89_e1312 * s.db[293][4]));
        let eq89_e1314_d_b5: f64 = (((p.p6 * s.db[68][5]) * s.v[293]) + (eq89_e1312 * s.db[293][5]));
        let eq89_e1314_d_b6: f64 = (((p.p6 * s.db[68][6]) * s.v[293]) + (eq89_e1312 * s.db[293][6]));
        let eq89_e1314_d_b7: f64 = (((p.p6 * s.db[68][7]) * s.v[293]) + (eq89_e1312 * s.db[293][7]));
        let eq89_e1314_d_b8: f64 = (((p.p6 * s.db[68][8]) * s.v[293]) + (eq89_e1312 * s.db[293][8]));
        let eq89_e1314_d_b9: f64 = (((p.p6 * s.db[68][9]) * s.v[293]) + (eq89_e1312 * s.db[293][9]));
        let eq89_e1314_d_b10: f64 = (((p.p6 * s.db[68][10]) * s.v[293]) + (eq89_e1312 * s.db[293][10]));
        let eq89_e1314_d_b11: f64 = (((p.p6 * s.db[68][11]) * s.v[293]) + (eq89_e1312 * s.db[293][11]));
        let eq89_e1314_d_b12: f64 = (((p.p6 * s.db[68][12]) * s.v[293]) + (eq89_e1312 * s.db[293][12]));
        let eq89_e1314_d_b13: f64 = (((p.p6 * s.db[68][13]) * s.v[293]) + (eq89_e1312 * s.db[293][13]));
        let eq89_e1314_d_b14: f64 = (((p.p6 * s.db[68][14]) * s.v[293]) + (eq89_e1312 * s.db[293][14]));
        let eq89_e1314_d_b15: f64 = (((p.p6 * s.db[68][15]) * s.v[293]) + (eq89_e1312 * s.db[293][15]));
        let eq89_e1314_d_b16: f64 = (((p.p6 * s.db[68][16]) * s.v[293]) + (eq89_e1312 * s.db[293][16]));
        let eq89_e1314_d_b17: f64 = (((p.p6 * s.db[68][17]) * s.v[293]) + (eq89_e1312 * s.db[293][17]));
        let eq89_e1314_d_b18: f64 = (((p.p6 * s.db[68][18]) * s.v[293]) + (eq89_e1312 * s.db[293][18]));
        let eq89_e1314_d_b19: f64 = (((p.p6 * s.db[68][19]) * s.v[293]) + (eq89_e1312 * s.db[293][19]));
        let eq89_e1314_d_b20: f64 = (((p.p6 * s.db[68][20]) * s.v[293]) + (eq89_e1312 * s.db[293][20]));
        let eq89_e1314_d_b21: f64 = (((p.p6 * s.db[68][21]) * s.v[293]) + (eq89_e1312 * s.db[293][21]));
        let eq89_e1314_d_b22: f64 = (((p.p6 * s.db[68][22]) * s.v[293]) + (eq89_e1312 * s.db[293][22]));
        let eq89_e1314_d_b23: f64 = (((p.p6 * s.db[68][23]) * s.v[293]) + (eq89_e1312 * s.db[293][23]));
        let eq89_e1314_d_b24: f64 = (((p.p6 * s.db[68][24]) * s.v[293]) + (eq89_e1312 * s.db[293][24]));
        let eq89_e1314_d_b25: f64 = (((p.p6 * s.db[68][25]) * s.v[293]) + (eq89_e1312 * s.db[293][25]));
        let eq89_e1314_d_b26: f64 = (((p.p6 * s.db[68][26]) * s.v[293]) + (eq89_e1312 * s.db[293][26]));
        let eq89_e1314_d_b27: f64 = (((p.p6 * s.db[68][27]) * s.v[293]) + (eq89_e1312 * s.db[293][27]));
        let eq89_e1314_d_b28: f64 = (((p.p6 * s.db[68][28]) * s.v[293]) + (eq89_e1312 * s.db[293][28]));
        let eq89_e1314_d_b29: f64 = (((p.p6 * s.db[68][29]) * s.v[293]) + (eq89_e1312 * s.db[293][29]));
        let eq89_e1314_d_b30: f64 = (((p.p6 * s.db[68][30]) * s.v[293]) + (eq89_e1312 * s.db[293][30]));
        let eq89_e1314_d_b31: f64 = (((p.p6 * s.db[68][31]) * s.v[293]) + (eq89_e1312 * s.db[293][31]));
        let eq89_e1314_d_b32: f64 = (((p.p6 * s.db[68][32]) * s.v[293]) + (eq89_e1312 * s.db[293][32]));
        let eq89_e1314_d_b33: f64 = (((p.p6 * s.db[68][33]) * s.v[293]) + (eq89_e1312 * s.db[293][33]));
        let eq89_e1314_d_b34: f64 = (((p.p6 * s.db[68][34]) * s.v[293]) + (eq89_e1312 * s.db[293][34]));
        let eq89_e1314_d_b35: f64 = (((p.p6 * s.db[68][35]) * s.v[293]) + (eq89_e1312 * s.db[293][35]));
        let eq89_e1314_d_b36: f64 = (((p.p6 * s.db[68][36]) * s.v[293]) + (eq89_e1312 * s.db[293][36]));
        let eq89_e1314_d_b37: f64 = (((p.p6 * s.db[68][37]) * s.v[293]) + (eq89_e1312 * s.db[293][37]));
        let eq89_e1314_d_b38: f64 = (((p.p6 * s.db[68][38]) * s.v[293]) + (eq89_e1312 * s.db[293][38]));
        let eq89_e1314_d_b39: f64 = (((p.p6 * s.db[68][39]) * s.v[293]) + (eq89_e1312 * s.db[293][39]));
        let eq89_e1314_d_b40: f64 = (((p.p6 * s.db[68][40]) * s.v[293]) + (eq89_e1312 * s.db[293][40]));
        let eq89_e1314_d_b41: f64 = (((p.p6 * s.db[68][41]) * s.v[293]) + (eq89_e1312 * s.db[293][41]));
        let eq89_e1314_d_b42: f64 = (((p.p6 * s.db[68][42]) * s.v[293]) + (eq89_e1312 * s.db[293][42]));
        let eq89_e1314_d_b43: f64 = (((p.p6 * s.db[68][43]) * s.v[293]) + (eq89_e1312 * s.db[293][43]));
        let eq89_e1314_d_b44: f64 = (((p.p6 * s.db[68][44]) * s.v[293]) + (eq89_e1312 * s.db[293][44]));
        let eq89_e1314_d_b45: f64 = (((p.p6 * s.db[68][45]) * s.v[293]) + (eq89_e1312 * s.db[293][45]));
        let eq89_e1314_d_b46: f64 = (((p.p6 * s.db[68][46]) * s.v[293]) + (eq89_e1312 * s.db[293][46]));
        let eq89_e1314_d_b47: f64 = (((p.p6 * s.db[68][47]) * s.v[293]) + (eq89_e1312 * s.db[293][47]));
        let eq89_e1314_d_b48: f64 = (((p.p6 * s.db[68][48]) * s.v[293]) + (eq89_e1312 * s.db[293][48]));
        let eq89_e1314_d_b49: f64 = (((p.p6 * s.db[68][49]) * s.v[293]) + (eq89_e1312 * s.db[293][49]));
        let eq89_e1314_d_b50: f64 = (((p.p6 * s.db[68][50]) * s.v[293]) + (eq89_e1312 * s.db[293][50]));
        let eq89_e1314_d_b51: f64 = (((p.p6 * s.db[68][51]) * s.v[293]) + (eq89_e1312 * s.db[293][51]));
        let eq89_e1314_d_b52: f64 = (((p.p6 * s.db[68][52]) * s.v[293]) + (eq89_e1312 * s.db[293][52]));
        let eq89_e1314_d_b53: f64 = (((p.p6 * s.db[68][53]) * s.v[293]) + (eq89_e1312 * s.db[293][53]));
        let eq89_e1314_d_b54: f64 = (((p.p6 * s.db[68][54]) * s.v[293]) + (eq89_e1312 * s.db[293][54]));
        let eq89_e1317: f64 = (p.p6 * s.v[379]);
        let eq89_e1319: f64 = (eq89_e1317 * (nv20 - nv21));
        let eq89_e1319_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv20 - nv21));
        let eq89_e1319_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv20 - nv21));
        let eq89_e1319_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv20 - nv21));
        let eq89_e1319_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv20 - nv21));
        let eq89_e1319_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv20 - nv21));
        let eq89_e1319_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv20 - nv21));
        let eq89_e1319_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv20 - nv21));
        let eq89_e1319_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv20 - nv21));
        let eq89_e1319_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv20 - nv21));
        let eq89_e1319_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv20 - nv21));
        let eq89_e1319_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv20 - nv21));
        let eq89_e1319_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv20 - nv21));
        let eq89_e1319_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv20 - nv21));
        let eq89_e1319_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv20 - nv21));
        let eq89_e1319_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv20 - nv21));
        let eq89_e1319_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv20 - nv21));
        let eq89_e1319_d_n16: f64 = ((p.p6 * s.dn[379][16]) * (nv20 - nv21));
        let eq89_e1319_d_n17: f64 = ((p.p6 * s.dn[379][17]) * (nv20 - nv21));
        let eq89_e1319_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv20 - nv21));
        let eq89_e1319_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv20 - nv21));
        let eq89_e1319_d_n20: f64 = (((p.p6 * s.dn[379][20]) * (nv20 - nv21)) + eq89_e1317);
        let eq89_e1319_d_n21: f64 = (((p.p6 * s.dn[379][21]) * (nv20 - nv21)) + (-eq89_e1317));
        let eq89_e1319_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv20 - nv21));
        let eq89_e1319_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv20 - nv21));
        let eq89_e1319_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv20 - nv21));
        let eq89_e1319_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv20 - nv21));
        let eq89_e1319_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv20 - nv21));
        let eq89_e1319_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv20 - nv21));
        let eq89_e1319_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv20 - nv21));
        let eq89_e1319_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv20 - nv21));
        let eq89_e1319_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv20 - nv21));
        let eq89_e1319_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv20 - nv21));
        let eq89_e1319_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv20 - nv21));
        let eq89_e1319_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv20 - nv21));
        let eq89_e1319_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv20 - nv21));
        let eq89_e1319_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv20 - nv21));
        let eq89_e1319_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv20 - nv21));
        let eq89_e1319_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv20 - nv21));
        let eq89_e1319_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv20 - nv21));
        let eq89_e1319_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv20 - nv21));
        let eq89_e1319_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv20 - nv21));
        let eq89_e1319_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv20 - nv21));
        let eq89_e1319_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv20 - nv21));
        let eq89_e1319_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv20 - nv21));
        let eq89_e1319_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv20 - nv21));
        let eq89_e1319_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv20 - nv21));
        let eq89_e1319_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv20 - nv21));
        let eq89_e1319_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv20 - nv21));
        let eq89_e1319_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv20 - nv21));
        let eq89_e1319_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv20 - nv21));
        let eq89_e1319_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv20 - nv21));
        let eq89_e1319_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv20 - nv21));
        let eq89_e1319_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv20 - nv21));
        let eq89_e1319_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv20 - nv21));
        let eq89_e1319_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv20 - nv21));
        let eq89_e1319_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv20 - nv21));
        let eq89_e1319_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv20 - nv21));
        let eq89_e1319_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv20 - nv21));
        let eq89_e1319_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv20 - nv21));
        let eq89_e1319_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv20 - nv21));
        let eq89_e1319_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv20 - nv21));
        let eq89_e1319_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv20 - nv21));
        let eq89_e1319_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv20 - nv21));
        let eq89_e1319_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv20 - nv21));
        let eq89_e1319_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv20 - nv21));
        let eq89_e1319_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv20 - nv21));
        let eq89_e1319_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv20 - nv21));
        let eq89_e1319_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv20 - nv21));
        let eq89_e1319_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv20 - nv21));
        let eq89_e1319_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv20 - nv21));
        let eq89_e1319_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv20 - nv21));
        let eq89_e1319_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv20 - nv21));
        let eq89_e1319_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv20 - nv21));
        let eq89_e1319_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv20 - nv21));
        let eq89_e1319_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv20 - nv21));
        let eq89_e1319_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv20 - nv21));
        let eq89_e1319_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv20 - nv21));
        let eq89_e1319_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv20 - nv21));
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
    }

    pub(super) fn stamp_transient_equations_block_12(
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
        let eq93_e1350: f64 = (eq93_e1348 * s.v[305]);
        let eq93_e1350_d_n0: f64 = (((p.p6 * s.dn[72][0]) * s.v[305]) + (eq93_e1348 * s.dn[305][0]));
        let eq93_e1350_d_n1: f64 = (((p.p6 * s.dn[72][1]) * s.v[305]) + (eq93_e1348 * s.dn[305][1]));
        let eq93_e1350_d_n2: f64 = (((p.p6 * s.dn[72][2]) * s.v[305]) + (eq93_e1348 * s.dn[305][2]));
        let eq93_e1350_d_n3: f64 = (((p.p6 * s.dn[72][3]) * s.v[305]) + (eq93_e1348 * s.dn[305][3]));
        let eq93_e1350_d_n4: f64 = (((p.p6 * s.dn[72][4]) * s.v[305]) + (eq93_e1348 * s.dn[305][4]));
        let eq93_e1350_d_n5: f64 = (((p.p6 * s.dn[72][5]) * s.v[305]) + (eq93_e1348 * s.dn[305][5]));
        let eq93_e1350_d_n6: f64 = (((p.p6 * s.dn[72][6]) * s.v[305]) + (eq93_e1348 * s.dn[305][6]));
        let eq93_e1350_d_n7: f64 = (((p.p6 * s.dn[72][7]) * s.v[305]) + (eq93_e1348 * s.dn[305][7]));
        let eq93_e1350_d_n8: f64 = (((p.p6 * s.dn[72][8]) * s.v[305]) + (eq93_e1348 * s.dn[305][8]));
        let eq93_e1350_d_n9: f64 = (((p.p6 * s.dn[72][9]) * s.v[305]) + (eq93_e1348 * s.dn[305][9]));
        let eq93_e1350_d_n10: f64 = (((p.p6 * s.dn[72][10]) * s.v[305]) + (eq93_e1348 * s.dn[305][10]));
        let eq93_e1350_d_n11: f64 = (((p.p6 * s.dn[72][11]) * s.v[305]) + (eq93_e1348 * s.dn[305][11]));
        let eq93_e1350_d_n12: f64 = (((p.p6 * s.dn[72][12]) * s.v[305]) + (eq93_e1348 * s.dn[305][12]));
        let eq93_e1350_d_n13: f64 = (((p.p6 * s.dn[72][13]) * s.v[305]) + (eq93_e1348 * s.dn[305][13]));
        let eq93_e1350_d_n14: f64 = (((p.p6 * s.dn[72][14]) * s.v[305]) + (eq93_e1348 * s.dn[305][14]));
        let eq93_e1350_d_n15: f64 = (((p.p6 * s.dn[72][15]) * s.v[305]) + (eq93_e1348 * s.dn[305][15]));
        let eq93_e1350_d_n16: f64 = (((p.p6 * s.dn[72][16]) * s.v[305]) + (eq93_e1348 * s.dn[305][16]));
        let eq93_e1350_d_n17: f64 = (((p.p6 * s.dn[72][17]) * s.v[305]) + (eq93_e1348 * s.dn[305][17]));
        let eq93_e1350_d_n18: f64 = (((p.p6 * s.dn[72][18]) * s.v[305]) + (eq93_e1348 * s.dn[305][18]));
        let eq93_e1350_d_n19: f64 = (((p.p6 * s.dn[72][19]) * s.v[305]) + (eq93_e1348 * s.dn[305][19]));
        let eq93_e1350_d_n20: f64 = (((p.p6 * s.dn[72][20]) * s.v[305]) + (eq93_e1348 * s.dn[305][20]));
        let eq93_e1350_d_n21: f64 = (((p.p6 * s.dn[72][21]) * s.v[305]) + (eq93_e1348 * s.dn[305][21]));
        let eq93_e1350_d_n22: f64 = (((p.p6 * s.dn[72][22]) * s.v[305]) + (eq93_e1348 * s.dn[305][22]));
        let eq93_e1350_d_b0: f64 = (((p.p6 * s.db[72][0]) * s.v[305]) + (eq93_e1348 * s.db[305][0]));
        let eq93_e1350_d_b1: f64 = (((p.p6 * s.db[72][1]) * s.v[305]) + (eq93_e1348 * s.db[305][1]));
        let eq93_e1350_d_b2: f64 = (((p.p6 * s.db[72][2]) * s.v[305]) + (eq93_e1348 * s.db[305][2]));
        let eq93_e1350_d_b3: f64 = (((p.p6 * s.db[72][3]) * s.v[305]) + (eq93_e1348 * s.db[305][3]));
        let eq93_e1350_d_b4: f64 = (((p.p6 * s.db[72][4]) * s.v[305]) + (eq93_e1348 * s.db[305][4]));
        let eq93_e1350_d_b5: f64 = (((p.p6 * s.db[72][5]) * s.v[305]) + (eq93_e1348 * s.db[305][5]));
        let eq93_e1350_d_b6: f64 = (((p.p6 * s.db[72][6]) * s.v[305]) + (eq93_e1348 * s.db[305][6]));
        let eq93_e1350_d_b7: f64 = (((p.p6 * s.db[72][7]) * s.v[305]) + (eq93_e1348 * s.db[305][7]));
        let eq93_e1350_d_b8: f64 = (((p.p6 * s.db[72][8]) * s.v[305]) + (eq93_e1348 * s.db[305][8]));
        let eq93_e1350_d_b9: f64 = (((p.p6 * s.db[72][9]) * s.v[305]) + (eq93_e1348 * s.db[305][9]));
        let eq93_e1350_d_b10: f64 = (((p.p6 * s.db[72][10]) * s.v[305]) + (eq93_e1348 * s.db[305][10]));
        let eq93_e1350_d_b11: f64 = (((p.p6 * s.db[72][11]) * s.v[305]) + (eq93_e1348 * s.db[305][11]));
        let eq93_e1350_d_b12: f64 = (((p.p6 * s.db[72][12]) * s.v[305]) + (eq93_e1348 * s.db[305][12]));
        let eq93_e1350_d_b13: f64 = (((p.p6 * s.db[72][13]) * s.v[305]) + (eq93_e1348 * s.db[305][13]));
        let eq93_e1350_d_b14: f64 = (((p.p6 * s.db[72][14]) * s.v[305]) + (eq93_e1348 * s.db[305][14]));
        let eq93_e1350_d_b15: f64 = (((p.p6 * s.db[72][15]) * s.v[305]) + (eq93_e1348 * s.db[305][15]));
        let eq93_e1350_d_b16: f64 = (((p.p6 * s.db[72][16]) * s.v[305]) + (eq93_e1348 * s.db[305][16]));
        let eq93_e1350_d_b17: f64 = (((p.p6 * s.db[72][17]) * s.v[305]) + (eq93_e1348 * s.db[305][17]));
        let eq93_e1350_d_b18: f64 = (((p.p6 * s.db[72][18]) * s.v[305]) + (eq93_e1348 * s.db[305][18]));
        let eq93_e1350_d_b19: f64 = (((p.p6 * s.db[72][19]) * s.v[305]) + (eq93_e1348 * s.db[305][19]));
        let eq93_e1350_d_b20: f64 = (((p.p6 * s.db[72][20]) * s.v[305]) + (eq93_e1348 * s.db[305][20]));
        let eq93_e1350_d_b21: f64 = (((p.p6 * s.db[72][21]) * s.v[305]) + (eq93_e1348 * s.db[305][21]));
        let eq93_e1350_d_b22: f64 = (((p.p6 * s.db[72][22]) * s.v[305]) + (eq93_e1348 * s.db[305][22]));
        let eq93_e1350_d_b23: f64 = (((p.p6 * s.db[72][23]) * s.v[305]) + (eq93_e1348 * s.db[305][23]));
        let eq93_e1350_d_b24: f64 = (((p.p6 * s.db[72][24]) * s.v[305]) + (eq93_e1348 * s.db[305][24]));
        let eq93_e1350_d_b25: f64 = (((p.p6 * s.db[72][25]) * s.v[305]) + (eq93_e1348 * s.db[305][25]));
        let eq93_e1350_d_b26: f64 = (((p.p6 * s.db[72][26]) * s.v[305]) + (eq93_e1348 * s.db[305][26]));
        let eq93_e1350_d_b27: f64 = (((p.p6 * s.db[72][27]) * s.v[305]) + (eq93_e1348 * s.db[305][27]));
        let eq93_e1350_d_b28: f64 = (((p.p6 * s.db[72][28]) * s.v[305]) + (eq93_e1348 * s.db[305][28]));
        let eq93_e1350_d_b29: f64 = (((p.p6 * s.db[72][29]) * s.v[305]) + (eq93_e1348 * s.db[305][29]));
        let eq93_e1350_d_b30: f64 = (((p.p6 * s.db[72][30]) * s.v[305]) + (eq93_e1348 * s.db[305][30]));
        let eq93_e1350_d_b31: f64 = (((p.p6 * s.db[72][31]) * s.v[305]) + (eq93_e1348 * s.db[305][31]));
        let eq93_e1350_d_b32: f64 = (((p.p6 * s.db[72][32]) * s.v[305]) + (eq93_e1348 * s.db[305][32]));
        let eq93_e1350_d_b33: f64 = (((p.p6 * s.db[72][33]) * s.v[305]) + (eq93_e1348 * s.db[305][33]));
        let eq93_e1350_d_b34: f64 = (((p.p6 * s.db[72][34]) * s.v[305]) + (eq93_e1348 * s.db[305][34]));
        let eq93_e1350_d_b35: f64 = (((p.p6 * s.db[72][35]) * s.v[305]) + (eq93_e1348 * s.db[305][35]));
        let eq93_e1350_d_b36: f64 = (((p.p6 * s.db[72][36]) * s.v[305]) + (eq93_e1348 * s.db[305][36]));
        let eq93_e1350_d_b37: f64 = (((p.p6 * s.db[72][37]) * s.v[305]) + (eq93_e1348 * s.db[305][37]));
        let eq93_e1350_d_b38: f64 = (((p.p6 * s.db[72][38]) * s.v[305]) + (eq93_e1348 * s.db[305][38]));
        let eq93_e1350_d_b39: f64 = (((p.p6 * s.db[72][39]) * s.v[305]) + (eq93_e1348 * s.db[305][39]));
        let eq93_e1350_d_b40: f64 = (((p.p6 * s.db[72][40]) * s.v[305]) + (eq93_e1348 * s.db[305][40]));
        let eq93_e1350_d_b41: f64 = (((p.p6 * s.db[72][41]) * s.v[305]) + (eq93_e1348 * s.db[305][41]));
        let eq93_e1350_d_b42: f64 = (((p.p6 * s.db[72][42]) * s.v[305]) + (eq93_e1348 * s.db[305][42]));
        let eq93_e1350_d_b43: f64 = (((p.p6 * s.db[72][43]) * s.v[305]) + (eq93_e1348 * s.db[305][43]));
        let eq93_e1350_d_b44: f64 = (((p.p6 * s.db[72][44]) * s.v[305]) + (eq93_e1348 * s.db[305][44]));
        let eq93_e1350_d_b45: f64 = (((p.p6 * s.db[72][45]) * s.v[305]) + (eq93_e1348 * s.db[305][45]));
        let eq93_e1350_d_b46: f64 = (((p.p6 * s.db[72][46]) * s.v[305]) + (eq93_e1348 * s.db[305][46]));
        let eq93_e1350_d_b47: f64 = (((p.p6 * s.db[72][47]) * s.v[305]) + (eq93_e1348 * s.db[305][47]));
        let eq93_e1350_d_b48: f64 = (((p.p6 * s.db[72][48]) * s.v[305]) + (eq93_e1348 * s.db[305][48]));
        let eq93_e1350_d_b49: f64 = (((p.p6 * s.db[72][49]) * s.v[305]) + (eq93_e1348 * s.db[305][49]));
        let eq93_e1350_d_b50: f64 = (((p.p6 * s.db[72][50]) * s.v[305]) + (eq93_e1348 * s.db[305][50]));
        let eq93_e1350_d_b51: f64 = (((p.p6 * s.db[72][51]) * s.v[305]) + (eq93_e1348 * s.db[305][51]));
        let eq93_e1350_d_b52: f64 = (((p.p6 * s.db[72][52]) * s.v[305]) + (eq93_e1348 * s.db[305][52]));
        let eq93_e1350_d_b53: f64 = (((p.p6 * s.db[72][53]) * s.v[305]) + (eq93_e1348 * s.db[305][53]));
        let eq93_e1350_d_b54: f64 = (((p.p6 * s.db[72][54]) * s.v[305]) + (eq93_e1348 * s.db[305][54]));
        let eq93_e1353: f64 = (p.p6 * s.v[379]);
        let eq93_e1355: f64 = (eq93_e1353 * (nv18 - nv17));
        let eq93_e1355_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv18 - nv17));
        let eq93_e1355_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv18 - nv17));
        let eq93_e1355_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv18 - nv17));
        let eq93_e1355_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv18 - nv17));
        let eq93_e1355_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv18 - nv17));
        let eq93_e1355_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv18 - nv17));
        let eq93_e1355_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv18 - nv17));
        let eq93_e1355_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv18 - nv17));
        let eq93_e1355_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv18 - nv17));
        let eq93_e1355_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv18 - nv17));
        let eq93_e1355_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv18 - nv17));
        let eq93_e1355_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv18 - nv17));
        let eq93_e1355_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv18 - nv17));
        let eq93_e1355_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv18 - nv17));
        let eq93_e1355_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv18 - nv17));
        let eq93_e1355_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv18 - nv17));
        let eq93_e1355_d_n16: f64 = ((p.p6 * s.dn[379][16]) * (nv18 - nv17));
        let eq93_e1355_d_n17: f64 = (((p.p6 * s.dn[379][17]) * (nv18 - nv17)) + (-eq93_e1353));
        let eq93_e1355_d_n18: f64 = (((p.p6 * s.dn[379][18]) * (nv18 - nv17)) + eq93_e1353);
        let eq93_e1355_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv18 - nv17));
        let eq93_e1355_d_n20: f64 = ((p.p6 * s.dn[379][20]) * (nv18 - nv17));
        let eq93_e1355_d_n21: f64 = ((p.p6 * s.dn[379][21]) * (nv18 - nv17));
        let eq93_e1355_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv18 - nv17));
        let eq93_e1355_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv18 - nv17));
        let eq93_e1355_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv18 - nv17));
        let eq93_e1355_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv18 - nv17));
        let eq93_e1355_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv18 - nv17));
        let eq93_e1355_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv18 - nv17));
        let eq93_e1355_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv18 - nv17));
        let eq93_e1355_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv18 - nv17));
        let eq93_e1355_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv18 - nv17));
        let eq93_e1355_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv18 - nv17));
        let eq93_e1355_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv18 - nv17));
        let eq93_e1355_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv18 - nv17));
        let eq93_e1355_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv18 - nv17));
        let eq93_e1355_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv18 - nv17));
        let eq93_e1355_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv18 - nv17));
        let eq93_e1355_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv18 - nv17));
        let eq93_e1355_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv18 - nv17));
        let eq93_e1355_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv18 - nv17));
        let eq93_e1355_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv18 - nv17));
        let eq93_e1355_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv18 - nv17));
        let eq93_e1355_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv18 - nv17));
        let eq93_e1355_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv18 - nv17));
        let eq93_e1355_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv18 - nv17));
        let eq93_e1355_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv18 - nv17));
        let eq93_e1355_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv18 - nv17));
        let eq93_e1355_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv18 - nv17));
        let eq93_e1355_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv18 - nv17));
        let eq93_e1355_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv18 - nv17));
        let eq93_e1355_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv18 - nv17));
        let eq93_e1355_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv18 - nv17));
        let eq93_e1355_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv18 - nv17));
        let eq93_e1355_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv18 - nv17));
        let eq93_e1355_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv18 - nv17));
        let eq93_e1355_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv18 - nv17));
        let eq93_e1355_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv18 - nv17));
        let eq93_e1355_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv18 - nv17));
        let eq93_e1355_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv18 - nv17));
        let eq93_e1355_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv18 - nv17));
        let eq93_e1355_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv18 - nv17));
        let eq93_e1355_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv18 - nv17));
        let eq93_e1355_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv18 - nv17));
        let eq93_e1355_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv18 - nv17));
        let eq93_e1355_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv18 - nv17));
        let eq93_e1355_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv18 - nv17));
        let eq93_e1355_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv18 - nv17));
        let eq93_e1355_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv18 - nv17));
        let eq93_e1355_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv18 - nv17));
        let eq93_e1355_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv18 - nv17));
        let eq93_e1355_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv18 - nv17));
        let eq93_e1355_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv18 - nv17));
        let eq93_e1355_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv18 - nv17));
        let eq93_e1355_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv18 - nv17));
        let eq93_e1355_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv18 - nv17));
        let eq93_e1355_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv18 - nv17));
        let eq93_e1355_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv18 - nv17));
        let eq93_e1355_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv18 - nv17));
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
    }

    pub(super) fn stamp_transient_equations_block_13(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
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
        let nv21 = ctx.node_voltage(nodes[21]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let (eq96_e1386, eq96_e1386_d_n0, eq96_e1386_d_n1, eq96_e1386_d_n2, eq96_e1386_d_n3, eq96_e1386_d_n4, eq96_e1386_d_n5, eq96_e1386_d_n6, eq96_e1386_d_n7, eq96_e1386_d_n8, eq96_e1386_d_n9, eq96_e1386_d_n10, eq96_e1386_d_n11, eq96_e1386_d_n12, eq96_e1386_d_n13, eq96_e1386_d_n14, eq96_e1386_d_n15, eq96_e1386_d_n16, eq96_e1386_d_n17, eq96_e1386_d_n18, eq96_e1386_d_n19, eq96_e1386_d_n20, eq96_e1386_d_n21, eq96_e1386_d_n22, eq96_e1386_d_b0, eq96_e1386_d_b1, eq96_e1386_d_b2, eq96_e1386_d_b3, eq96_e1386_d_b4, eq96_e1386_d_b5, eq96_e1386_d_b6, eq96_e1386_d_b7, eq96_e1386_d_b8, eq96_e1386_d_b9, eq96_e1386_d_b10, eq96_e1386_d_b11, eq96_e1386_d_b12, eq96_e1386_d_b13, eq96_e1386_d_b14, eq96_e1386_d_b15, eq96_e1386_d_b16, eq96_e1386_d_b17, eq96_e1386_d_b18, eq96_e1386_d_b19, eq96_e1386_d_b20, eq96_e1386_d_b21, eq96_e1386_d_b22, eq96_e1386_d_b23, eq96_e1386_d_b24, eq96_e1386_d_b25, eq96_e1386_d_b26, eq96_e1386_d_b27, eq96_e1386_d_b28, eq96_e1386_d_b29, eq96_e1386_d_b30, eq96_e1386_d_b31, eq96_e1386_d_b32, eq96_e1386_d_b33, eq96_e1386_d_b34, eq96_e1386_d_b35, eq96_e1386_d_b36, eq96_e1386_d_b37, eq96_e1386_d_b38, eq96_e1386_d_b39, eq96_e1386_d_b40, eq96_e1386_d_b41, eq96_e1386_d_b42, eq96_e1386_d_b43, eq96_e1386_d_b44, eq96_e1386_d_b45, eq96_e1386_d_b46, eq96_e1386_d_b47, eq96_e1386_d_b48, eq96_e1386_d_b49, eq96_e1386_d_b50, eq96_e1386_d_b51, eq96_e1386_d_b52, eq96_e1386_d_b53, eq96_e1386_d_b54,) = {
    if (s.b[538] && s.b[539]) {
        let eq96_e1376: f64 = (p.p6 * s.v[76]);
        let eq96_e1378: f64 = (eq96_e1376 * s.v[317]);
        let eq96_e1378_d_n0: f64 = (((p.p6 * s.dn[76][0]) * s.v[317]) + (eq96_e1376 * s.dn[317][0]));
        let eq96_e1378_d_n1: f64 = (((p.p6 * s.dn[76][1]) * s.v[317]) + (eq96_e1376 * s.dn[317][1]));
        let eq96_e1378_d_n2: f64 = (((p.p6 * s.dn[76][2]) * s.v[317]) + (eq96_e1376 * s.dn[317][2]));
        let eq96_e1378_d_n3: f64 = (((p.p6 * s.dn[76][3]) * s.v[317]) + (eq96_e1376 * s.dn[317][3]));
        let eq96_e1378_d_n4: f64 = (((p.p6 * s.dn[76][4]) * s.v[317]) + (eq96_e1376 * s.dn[317][4]));
        let eq96_e1378_d_n5: f64 = (((p.p6 * s.dn[76][5]) * s.v[317]) + (eq96_e1376 * s.dn[317][5]));
        let eq96_e1378_d_n6: f64 = (((p.p6 * s.dn[76][6]) * s.v[317]) + (eq96_e1376 * s.dn[317][6]));
        let eq96_e1378_d_n7: f64 = (((p.p6 * s.dn[76][7]) * s.v[317]) + (eq96_e1376 * s.dn[317][7]));
        let eq96_e1378_d_n8: f64 = (((p.p6 * s.dn[76][8]) * s.v[317]) + (eq96_e1376 * s.dn[317][8]));
        let eq96_e1378_d_n9: f64 = (((p.p6 * s.dn[76][9]) * s.v[317]) + (eq96_e1376 * s.dn[317][9]));
        let eq96_e1378_d_n10: f64 = (((p.p6 * s.dn[76][10]) * s.v[317]) + (eq96_e1376 * s.dn[317][10]));
        let eq96_e1378_d_n11: f64 = (((p.p6 * s.dn[76][11]) * s.v[317]) + (eq96_e1376 * s.dn[317][11]));
        let eq96_e1378_d_n12: f64 = (((p.p6 * s.dn[76][12]) * s.v[317]) + (eq96_e1376 * s.dn[317][12]));
        let eq96_e1378_d_n13: f64 = (((p.p6 * s.dn[76][13]) * s.v[317]) + (eq96_e1376 * s.dn[317][13]));
        let eq96_e1378_d_n14: f64 = (((p.p6 * s.dn[76][14]) * s.v[317]) + (eq96_e1376 * s.dn[317][14]));
        let eq96_e1378_d_n15: f64 = (((p.p6 * s.dn[76][15]) * s.v[317]) + (eq96_e1376 * s.dn[317][15]));
        let eq96_e1378_d_n16: f64 = (((p.p6 * s.dn[76][16]) * s.v[317]) + (eq96_e1376 * s.dn[317][16]));
        let eq96_e1378_d_n17: f64 = (((p.p6 * s.dn[76][17]) * s.v[317]) + (eq96_e1376 * s.dn[317][17]));
        let eq96_e1378_d_n18: f64 = (((p.p6 * s.dn[76][18]) * s.v[317]) + (eq96_e1376 * s.dn[317][18]));
        let eq96_e1378_d_n19: f64 = (((p.p6 * s.dn[76][19]) * s.v[317]) + (eq96_e1376 * s.dn[317][19]));
        let eq96_e1378_d_n20: f64 = (((p.p6 * s.dn[76][20]) * s.v[317]) + (eq96_e1376 * s.dn[317][20]));
        let eq96_e1378_d_n21: f64 = (((p.p6 * s.dn[76][21]) * s.v[317]) + (eq96_e1376 * s.dn[317][21]));
        let eq96_e1378_d_n22: f64 = (((p.p6 * s.dn[76][22]) * s.v[317]) + (eq96_e1376 * s.dn[317][22]));
        let eq96_e1378_d_b0: f64 = (((p.p6 * s.db[76][0]) * s.v[317]) + (eq96_e1376 * s.db[317][0]));
        let eq96_e1378_d_b1: f64 = (((p.p6 * s.db[76][1]) * s.v[317]) + (eq96_e1376 * s.db[317][1]));
        let eq96_e1378_d_b2: f64 = (((p.p6 * s.db[76][2]) * s.v[317]) + (eq96_e1376 * s.db[317][2]));
        let eq96_e1378_d_b3: f64 = (((p.p6 * s.db[76][3]) * s.v[317]) + (eq96_e1376 * s.db[317][3]));
        let eq96_e1378_d_b4: f64 = (((p.p6 * s.db[76][4]) * s.v[317]) + (eq96_e1376 * s.db[317][4]));
        let eq96_e1378_d_b5: f64 = (((p.p6 * s.db[76][5]) * s.v[317]) + (eq96_e1376 * s.db[317][5]));
        let eq96_e1378_d_b6: f64 = (((p.p6 * s.db[76][6]) * s.v[317]) + (eq96_e1376 * s.db[317][6]));
        let eq96_e1378_d_b7: f64 = (((p.p6 * s.db[76][7]) * s.v[317]) + (eq96_e1376 * s.db[317][7]));
        let eq96_e1378_d_b8: f64 = (((p.p6 * s.db[76][8]) * s.v[317]) + (eq96_e1376 * s.db[317][8]));
        let eq96_e1378_d_b9: f64 = (((p.p6 * s.db[76][9]) * s.v[317]) + (eq96_e1376 * s.db[317][9]));
        let eq96_e1378_d_b10: f64 = (((p.p6 * s.db[76][10]) * s.v[317]) + (eq96_e1376 * s.db[317][10]));
        let eq96_e1378_d_b11: f64 = (((p.p6 * s.db[76][11]) * s.v[317]) + (eq96_e1376 * s.db[317][11]));
        let eq96_e1378_d_b12: f64 = (((p.p6 * s.db[76][12]) * s.v[317]) + (eq96_e1376 * s.db[317][12]));
        let eq96_e1378_d_b13: f64 = (((p.p6 * s.db[76][13]) * s.v[317]) + (eq96_e1376 * s.db[317][13]));
        let eq96_e1378_d_b14: f64 = (((p.p6 * s.db[76][14]) * s.v[317]) + (eq96_e1376 * s.db[317][14]));
        let eq96_e1378_d_b15: f64 = (((p.p6 * s.db[76][15]) * s.v[317]) + (eq96_e1376 * s.db[317][15]));
        let eq96_e1378_d_b16: f64 = (((p.p6 * s.db[76][16]) * s.v[317]) + (eq96_e1376 * s.db[317][16]));
        let eq96_e1378_d_b17: f64 = (((p.p6 * s.db[76][17]) * s.v[317]) + (eq96_e1376 * s.db[317][17]));
        let eq96_e1378_d_b18: f64 = (((p.p6 * s.db[76][18]) * s.v[317]) + (eq96_e1376 * s.db[317][18]));
        let eq96_e1378_d_b19: f64 = (((p.p6 * s.db[76][19]) * s.v[317]) + (eq96_e1376 * s.db[317][19]));
        let eq96_e1378_d_b20: f64 = (((p.p6 * s.db[76][20]) * s.v[317]) + (eq96_e1376 * s.db[317][20]));
        let eq96_e1378_d_b21: f64 = (((p.p6 * s.db[76][21]) * s.v[317]) + (eq96_e1376 * s.db[317][21]));
        let eq96_e1378_d_b22: f64 = (((p.p6 * s.db[76][22]) * s.v[317]) + (eq96_e1376 * s.db[317][22]));
        let eq96_e1378_d_b23: f64 = (((p.p6 * s.db[76][23]) * s.v[317]) + (eq96_e1376 * s.db[317][23]));
        let eq96_e1378_d_b24: f64 = (((p.p6 * s.db[76][24]) * s.v[317]) + (eq96_e1376 * s.db[317][24]));
        let eq96_e1378_d_b25: f64 = (((p.p6 * s.db[76][25]) * s.v[317]) + (eq96_e1376 * s.db[317][25]));
        let eq96_e1378_d_b26: f64 = (((p.p6 * s.db[76][26]) * s.v[317]) + (eq96_e1376 * s.db[317][26]));
        let eq96_e1378_d_b27: f64 = (((p.p6 * s.db[76][27]) * s.v[317]) + (eq96_e1376 * s.db[317][27]));
        let eq96_e1378_d_b28: f64 = (((p.p6 * s.db[76][28]) * s.v[317]) + (eq96_e1376 * s.db[317][28]));
        let eq96_e1378_d_b29: f64 = (((p.p6 * s.db[76][29]) * s.v[317]) + (eq96_e1376 * s.db[317][29]));
        let eq96_e1378_d_b30: f64 = (((p.p6 * s.db[76][30]) * s.v[317]) + (eq96_e1376 * s.db[317][30]));
        let eq96_e1378_d_b31: f64 = (((p.p6 * s.db[76][31]) * s.v[317]) + (eq96_e1376 * s.db[317][31]));
        let eq96_e1378_d_b32: f64 = (((p.p6 * s.db[76][32]) * s.v[317]) + (eq96_e1376 * s.db[317][32]));
        let eq96_e1378_d_b33: f64 = (((p.p6 * s.db[76][33]) * s.v[317]) + (eq96_e1376 * s.db[317][33]));
        let eq96_e1378_d_b34: f64 = (((p.p6 * s.db[76][34]) * s.v[317]) + (eq96_e1376 * s.db[317][34]));
        let eq96_e1378_d_b35: f64 = (((p.p6 * s.db[76][35]) * s.v[317]) + (eq96_e1376 * s.db[317][35]));
        let eq96_e1378_d_b36: f64 = (((p.p6 * s.db[76][36]) * s.v[317]) + (eq96_e1376 * s.db[317][36]));
        let eq96_e1378_d_b37: f64 = (((p.p6 * s.db[76][37]) * s.v[317]) + (eq96_e1376 * s.db[317][37]));
        let eq96_e1378_d_b38: f64 = (((p.p6 * s.db[76][38]) * s.v[317]) + (eq96_e1376 * s.db[317][38]));
        let eq96_e1378_d_b39: f64 = (((p.p6 * s.db[76][39]) * s.v[317]) + (eq96_e1376 * s.db[317][39]));
        let eq96_e1378_d_b40: f64 = (((p.p6 * s.db[76][40]) * s.v[317]) + (eq96_e1376 * s.db[317][40]));
        let eq96_e1378_d_b41: f64 = (((p.p6 * s.db[76][41]) * s.v[317]) + (eq96_e1376 * s.db[317][41]));
        let eq96_e1378_d_b42: f64 = (((p.p6 * s.db[76][42]) * s.v[317]) + (eq96_e1376 * s.db[317][42]));
        let eq96_e1378_d_b43: f64 = (((p.p6 * s.db[76][43]) * s.v[317]) + (eq96_e1376 * s.db[317][43]));
        let eq96_e1378_d_b44: f64 = (((p.p6 * s.db[76][44]) * s.v[317]) + (eq96_e1376 * s.db[317][44]));
        let eq96_e1378_d_b45: f64 = (((p.p6 * s.db[76][45]) * s.v[317]) + (eq96_e1376 * s.db[317][45]));
        let eq96_e1378_d_b46: f64 = (((p.p6 * s.db[76][46]) * s.v[317]) + (eq96_e1376 * s.db[317][46]));
        let eq96_e1378_d_b47: f64 = (((p.p6 * s.db[76][47]) * s.v[317]) + (eq96_e1376 * s.db[317][47]));
        let eq96_e1378_d_b48: f64 = (((p.p6 * s.db[76][48]) * s.v[317]) + (eq96_e1376 * s.db[317][48]));
        let eq96_e1378_d_b49: f64 = (((p.p6 * s.db[76][49]) * s.v[317]) + (eq96_e1376 * s.db[317][49]));
        let eq96_e1378_d_b50: f64 = (((p.p6 * s.db[76][50]) * s.v[317]) + (eq96_e1376 * s.db[317][50]));
        let eq96_e1378_d_b51: f64 = (((p.p6 * s.db[76][51]) * s.v[317]) + (eq96_e1376 * s.db[317][51]));
        let eq96_e1378_d_b52: f64 = (((p.p6 * s.db[76][52]) * s.v[317]) + (eq96_e1376 * s.db[317][52]));
        let eq96_e1378_d_b53: f64 = (((p.p6 * s.db[76][53]) * s.v[317]) + (eq96_e1376 * s.db[317][53]));
        let eq96_e1378_d_b54: f64 = (((p.p6 * s.db[76][54]) * s.v[317]) + (eq96_e1376 * s.db[317][54]));
        let eq96_e1381: f64 = (p.p6 * s.v[379]);
        let eq96_e1383: f64 = (eq96_e1381 * (nv21 - nv22));
        let eq96_e1383_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv21 - nv22));
        let eq96_e1383_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv21 - nv22));
        let eq96_e1383_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv21 - nv22));
        let eq96_e1383_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv21 - nv22));
        let eq96_e1383_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv21 - nv22));
        let eq96_e1383_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv21 - nv22));
        let eq96_e1383_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv21 - nv22));
        let eq96_e1383_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv21 - nv22));
        let eq96_e1383_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv21 - nv22));
        let eq96_e1383_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv21 - nv22));
        let eq96_e1383_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv21 - nv22));
        let eq96_e1383_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv21 - nv22));
        let eq96_e1383_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv21 - nv22));
        let eq96_e1383_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv21 - nv22));
        let eq96_e1383_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv21 - nv22));
        let eq96_e1383_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv21 - nv22));
        let eq96_e1383_d_n16: f64 = ((p.p6 * s.dn[379][16]) * (nv21 - nv22));
        let eq96_e1383_d_n17: f64 = ((p.p6 * s.dn[379][17]) * (nv21 - nv22));
        let eq96_e1383_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv21 - nv22));
        let eq96_e1383_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv21 - nv22));
        let eq96_e1383_d_n20: f64 = ((p.p6 * s.dn[379][20]) * (nv21 - nv22));
        let eq96_e1383_d_n21: f64 = (((p.p6 * s.dn[379][21]) * (nv21 - nv22)) + eq96_e1381);
        let eq96_e1383_d_n22: f64 = (((p.p6 * s.dn[379][22]) * (nv21 - nv22)) + (-eq96_e1381));
        let eq96_e1383_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv21 - nv22));
        let eq96_e1383_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv21 - nv22));
        let eq96_e1383_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv21 - nv22));
        let eq96_e1383_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv21 - nv22));
        let eq96_e1383_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv21 - nv22));
        let eq96_e1383_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv21 - nv22));
        let eq96_e1383_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv21 - nv22));
        let eq96_e1383_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv21 - nv22));
        let eq96_e1383_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv21 - nv22));
        let eq96_e1383_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv21 - nv22));
        let eq96_e1383_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv21 - nv22));
        let eq96_e1383_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv21 - nv22));
        let eq96_e1383_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv21 - nv22));
        let eq96_e1383_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv21 - nv22));
        let eq96_e1383_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv21 - nv22));
        let eq96_e1383_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv21 - nv22));
        let eq96_e1383_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv21 - nv22));
        let eq96_e1383_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv21 - nv22));
        let eq96_e1383_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv21 - nv22));
        let eq96_e1383_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv21 - nv22));
        let eq96_e1383_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv21 - nv22));
        let eq96_e1383_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv21 - nv22));
        let eq96_e1383_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv21 - nv22));
        let eq96_e1383_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv21 - nv22));
        let eq96_e1383_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv21 - nv22));
        let eq96_e1383_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv21 - nv22));
        let eq96_e1383_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv21 - nv22));
        let eq96_e1383_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv21 - nv22));
        let eq96_e1383_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv21 - nv22));
        let eq96_e1383_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv21 - nv22));
        let eq96_e1383_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv21 - nv22));
        let eq96_e1383_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv21 - nv22));
        let eq96_e1383_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv21 - nv22));
        let eq96_e1383_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv21 - nv22));
        let eq96_e1383_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv21 - nv22));
        let eq96_e1383_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv21 - nv22));
        let eq96_e1383_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv21 - nv22));
        let eq96_e1383_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv21 - nv22));
        let eq96_e1383_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv21 - nv22));
        let eq96_e1383_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv21 - nv22));
        let eq96_e1383_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv21 - nv22));
        let eq96_e1383_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv21 - nv22));
        let eq96_e1383_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv21 - nv22));
        let eq96_e1383_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv21 - nv22));
        let eq96_e1383_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv21 - nv22));
        let eq96_e1383_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv21 - nv22));
        let eq96_e1383_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv21 - nv22));
        let eq96_e1383_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv21 - nv22));
        let eq96_e1383_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv21 - nv22));
        let eq96_e1383_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv21 - nv22));
        let eq96_e1383_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv21 - nv22));
        let eq96_e1383_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv21 - nv22));
        let eq96_e1383_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv21 - nv22));
        let eq96_e1383_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv21 - nv22));
        let eq96_e1383_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv21 - nv22));
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
        let eq106_e1459: f64 = (p.p6 * s.v[369]);
        let eq106_value: f64 = eq106_e1459;
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * (eq106_value),
            &s.dn[369],
            &s.db[369],
            (multiplicity) * (p.p6),
        );
        let eq107_e1462: f64 = (p.p6 * s.v[370]);
        let eq107_value: f64 = eq107_e1462;
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * (eq107_value),
            &s.dn[370],
            &s.db[370],
            (multiplicity) * (p.p6),
        );
        let eq109_e1474: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, s.v[165]);
        let eq109_e1475: f64 = (p.p7 * eq109_e1474);
        let eq109_e1475_d_n0: f64 = (p.p7 * (s.dn[165][0] * ddt_scale));
        let eq109_e1475_d_n1: f64 = (p.p7 * (s.dn[165][1] * ddt_scale));
        let eq109_e1475_d_n2: f64 = (p.p7 * (s.dn[165][2] * ddt_scale));
        let eq109_e1475_d_n3: f64 = (p.p7 * (s.dn[165][3] * ddt_scale));
        let eq109_e1475_d_n4: f64 = (p.p7 * (s.dn[165][4] * ddt_scale));
        let eq109_e1475_d_n5: f64 = (p.p7 * (s.dn[165][5] * ddt_scale));
        let eq109_e1475_d_n6: f64 = (p.p7 * (s.dn[165][6] * ddt_scale));
        let eq109_e1475_d_n7: f64 = (p.p7 * (s.dn[165][7] * ddt_scale));
        let eq109_e1475_d_n8: f64 = (p.p7 * (s.dn[165][8] * ddt_scale));
        let eq109_e1475_d_n9: f64 = (p.p7 * (s.dn[165][9] * ddt_scale));
        let eq109_e1475_d_n10: f64 = (p.p7 * (s.dn[165][10] * ddt_scale));
        let eq109_e1475_d_n11: f64 = (p.p7 * (s.dn[165][11] * ddt_scale));
        let eq109_e1475_d_n12: f64 = (p.p7 * (s.dn[165][12] * ddt_scale));
        let eq109_e1475_d_n13: f64 = (p.p7 * (s.dn[165][13] * ddt_scale));
        let eq109_e1475_d_n14: f64 = (p.p7 * (s.dn[165][14] * ddt_scale));
        let eq109_e1475_d_n15: f64 = (p.p7 * (s.dn[165][15] * ddt_scale));
        let eq109_e1475_d_n16: f64 = (p.p7 * (s.dn[165][16] * ddt_scale));
        let eq109_e1475_d_n17: f64 = (p.p7 * (s.dn[165][17] * ddt_scale));
        let eq109_e1475_d_n18: f64 = (p.p7 * (s.dn[165][18] * ddt_scale));
        let eq109_e1475_d_n19: f64 = (p.p7 * (s.dn[165][19] * ddt_scale));
        let eq109_e1475_d_n20: f64 = (p.p7 * (s.dn[165][20] * ddt_scale));
        let eq109_e1475_d_n21: f64 = (p.p7 * (s.dn[165][21] * ddt_scale));
        let eq109_e1475_d_n22: f64 = (p.p7 * (s.dn[165][22] * ddt_scale));
        let eq109_e1475_d_b0: f64 = (p.p7 * (s.db[165][0] * ddt_scale));
        let eq109_e1475_d_b1: f64 = (p.p7 * (s.db[165][1] * ddt_scale));
        let eq109_e1475_d_b2: f64 = (p.p7 * (s.db[165][2] * ddt_scale));
        let eq109_e1475_d_b3: f64 = (p.p7 * (s.db[165][3] * ddt_scale));
        let eq109_e1475_d_b4: f64 = (p.p7 * (s.db[165][4] * ddt_scale));
        let eq109_e1475_d_b5: f64 = (p.p7 * (s.db[165][5] * ddt_scale));
        let eq109_e1475_d_b6: f64 = (p.p7 * (s.db[165][6] * ddt_scale));
        let eq109_e1475_d_b7: f64 = (p.p7 * (s.db[165][7] * ddt_scale));
        let eq109_e1475_d_b8: f64 = (p.p7 * (s.db[165][8] * ddt_scale));
        let eq109_e1475_d_b9: f64 = (p.p7 * (s.db[165][9] * ddt_scale));
        let eq109_e1475_d_b10: f64 = (p.p7 * (s.db[165][10] * ddt_scale));
        let eq109_e1475_d_b11: f64 = (p.p7 * (s.db[165][11] * ddt_scale));
        let eq109_e1475_d_b12: f64 = (p.p7 * (s.db[165][12] * ddt_scale));
        let eq109_e1475_d_b13: f64 = (p.p7 * (s.db[165][13] * ddt_scale));
        let eq109_e1475_d_b14: f64 = (p.p7 * (s.db[165][14] * ddt_scale));
        let eq109_e1475_d_b15: f64 = (p.p7 * (s.db[165][15] * ddt_scale));
        let eq109_e1475_d_b16: f64 = (p.p7 * (s.db[165][16] * ddt_scale));
        let eq109_e1475_d_b17: f64 = (p.p7 * (s.db[165][17] * ddt_scale));
        let eq109_e1475_d_b18: f64 = (p.p7 * (s.db[165][18] * ddt_scale));
        let eq109_e1475_d_b19: f64 = (p.p7 * (s.db[165][19] * ddt_scale));
        let eq109_e1475_d_b20: f64 = (p.p7 * (s.db[165][20] * ddt_scale));
        let eq109_e1475_d_b21: f64 = (p.p7 * (s.db[165][21] * ddt_scale));
        let eq109_e1475_d_b22: f64 = (p.p7 * (s.db[165][22] * ddt_scale));
        let eq109_e1475_d_b23: f64 = (p.p7 * (s.db[165][23] * ddt_scale));
        let eq109_e1475_d_b24: f64 = (p.p7 * (s.db[165][24] * ddt_scale));
        let eq109_e1475_d_b25: f64 = (p.p7 * (s.db[165][25] * ddt_scale));
        let eq109_e1475_d_b26: f64 = (p.p7 * (s.db[165][26] * ddt_scale));
        let eq109_e1475_d_b27: f64 = (p.p7 * (s.db[165][27] * ddt_scale));
        let eq109_e1475_d_b28: f64 = (p.p7 * (s.db[165][28] * ddt_scale));
        let eq109_e1475_d_b29: f64 = (p.p7 * (s.db[165][29] * ddt_scale));
        let eq109_e1475_d_b30: f64 = (p.p7 * (s.db[165][30] * ddt_scale));
        let eq109_e1475_d_b31: f64 = (p.p7 * (s.db[165][31] * ddt_scale));
        let eq109_e1475_d_b32: f64 = (p.p7 * (s.db[165][32] * ddt_scale));
        let eq109_e1475_d_b33: f64 = (p.p7 * (s.db[165][33] * ddt_scale));
        let eq109_e1475_d_b34: f64 = (p.p7 * (s.db[165][34] * ddt_scale));
        let eq109_e1475_d_b35: f64 = (p.p7 * (s.db[165][35] * ddt_scale));
        let eq109_e1475_d_b36: f64 = (p.p7 * (s.db[165][36] * ddt_scale));
        let eq109_e1475_d_b37: f64 = (p.p7 * (s.db[165][37] * ddt_scale));
        let eq109_e1475_d_b38: f64 = (p.p7 * (s.db[165][38] * ddt_scale));
        let eq109_e1475_d_b39: f64 = (p.p7 * (s.db[165][39] * ddt_scale));
        let eq109_e1475_d_b40: f64 = (p.p7 * (s.db[165][40] * ddt_scale));
        let eq109_e1475_d_b41: f64 = (p.p7 * (s.db[165][41] * ddt_scale));
        let eq109_e1475_d_b42: f64 = (p.p7 * (s.db[165][42] * ddt_scale));
        let eq109_e1475_d_b43: f64 = (p.p7 * (s.db[165][43] * ddt_scale));
        let eq109_e1475_d_b44: f64 = (p.p7 * (s.db[165][44] * ddt_scale));
        let eq109_e1475_d_b45: f64 = (p.p7 * (s.db[165][45] * ddt_scale));
        let eq109_e1475_d_b46: f64 = (p.p7 * (s.db[165][46] * ddt_scale));
        let eq109_e1475_d_b47: f64 = (p.p7 * (s.db[165][47] * ddt_scale));
        let eq109_e1475_d_b48: f64 = (p.p7 * (s.db[165][48] * ddt_scale));
        let eq109_e1475_d_b49: f64 = (p.p7 * (s.db[165][49] * ddt_scale));
        let eq109_e1475_d_b50: f64 = (p.p7 * (s.db[165][50] * ddt_scale));
        let eq109_e1475_d_b51: f64 = (p.p7 * (s.db[165][51] * ddt_scale));
        let eq109_e1475_d_b52: f64 = (p.p7 * (s.db[165][52] * ddt_scale));
        let eq109_e1475_d_b53: f64 = (p.p7 * (s.db[165][53] * ddt_scale));
        let eq109_e1475_d_b54: f64 = (p.p7 * (s.db[165][54] * ddt_scale));
        let eq109_value: f64 = eq109_e1475;
        let eq109_node_derivatives: [f64; 23] = [eq109_e1475_d_n0, eq109_e1475_d_n1, eq109_e1475_d_n2, eq109_e1475_d_n3, eq109_e1475_d_n4, eq109_e1475_d_n5, eq109_e1475_d_n6, eq109_e1475_d_n7, eq109_e1475_d_n8, eq109_e1475_d_n9, eq109_e1475_d_n10, eq109_e1475_d_n11, eq109_e1475_d_n12, eq109_e1475_d_n13, eq109_e1475_d_n14, eq109_e1475_d_n15, eq109_e1475_d_n16, eq109_e1475_d_n17, eq109_e1475_d_n18, eq109_e1475_d_n19, eq109_e1475_d_n20, eq109_e1475_d_n21, eq109_e1475_d_n22];
        let eq109_branch_derivatives: [f64; 55] = [eq109_e1475_d_b0, eq109_e1475_d_b1, eq109_e1475_d_b2, eq109_e1475_d_b3, eq109_e1475_d_b4, eq109_e1475_d_b5, eq109_e1475_d_b6, eq109_e1475_d_b7, eq109_e1475_d_b8, eq109_e1475_d_b9, eq109_e1475_d_b10, eq109_e1475_d_b11, eq109_e1475_d_b12, eq109_e1475_d_b13, eq109_e1475_d_b14, eq109_e1475_d_b15, eq109_e1475_d_b16, eq109_e1475_d_b17, eq109_e1475_d_b18, eq109_e1475_d_b19, eq109_e1475_d_b20, eq109_e1475_d_b21, eq109_e1475_d_b22, eq109_e1475_d_b23, eq109_e1475_d_b24, eq109_e1475_d_b25, eq109_e1475_d_b26, eq109_e1475_d_b27, eq109_e1475_d_b28, eq109_e1475_d_b29, eq109_e1475_d_b30, eq109_e1475_d_b31, eq109_e1475_d_b32, eq109_e1475_d_b33, eq109_e1475_d_b34, eq109_e1475_d_b35, eq109_e1475_d_b36, eq109_e1475_d_b37, eq109_e1475_d_b38, eq109_e1475_d_b39, eq109_e1475_d_b40, eq109_e1475_d_b41, eq109_e1475_d_b42, eq109_e1475_d_b43, eq109_e1475_d_b44, eq109_e1475_d_b45, eq109_e1475_d_b46, eq109_e1475_d_b47, eq109_e1475_d_b48, eq109_e1475_d_b49, eq109_e1475_d_b50, eq109_e1475_d_b51, eq109_e1475_d_b52, eq109_e1475_d_b53, eq109_e1475_d_b54];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq109_value),
            &eq109_node_derivatives,
            &eq109_branch_derivatives,
            multiplicity,
        );
        let eq110_e1478: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, s.v[161]);
        let eq110_e1479: f64 = (p.p7 * eq110_e1478);
        let eq110_e1479_d_n0: f64 = (p.p7 * (s.dn[161][0] * ddt_scale));
        let eq110_e1479_d_n1: f64 = (p.p7 * (s.dn[161][1] * ddt_scale));
        let eq110_e1479_d_n2: f64 = (p.p7 * (s.dn[161][2] * ddt_scale));
        let eq110_e1479_d_n3: f64 = (p.p7 * (s.dn[161][3] * ddt_scale));
        let eq110_e1479_d_n4: f64 = (p.p7 * (s.dn[161][4] * ddt_scale));
        let eq110_e1479_d_n5: f64 = (p.p7 * (s.dn[161][5] * ddt_scale));
        let eq110_e1479_d_n6: f64 = (p.p7 * (s.dn[161][6] * ddt_scale));
        let eq110_e1479_d_n7: f64 = (p.p7 * (s.dn[161][7] * ddt_scale));
        let eq110_e1479_d_n8: f64 = (p.p7 * (s.dn[161][8] * ddt_scale));
        let eq110_e1479_d_n9: f64 = (p.p7 * (s.dn[161][9] * ddt_scale));
        let eq110_e1479_d_n10: f64 = (p.p7 * (s.dn[161][10] * ddt_scale));
        let eq110_e1479_d_n11: f64 = (p.p7 * (s.dn[161][11] * ddt_scale));
        let eq110_e1479_d_n12: f64 = (p.p7 * (s.dn[161][12] * ddt_scale));
        let eq110_e1479_d_n13: f64 = (p.p7 * (s.dn[161][13] * ddt_scale));
        let eq110_e1479_d_n14: f64 = (p.p7 * (s.dn[161][14] * ddt_scale));
        let eq110_e1479_d_n15: f64 = (p.p7 * (s.dn[161][15] * ddt_scale));
        let eq110_e1479_d_n16: f64 = (p.p7 * (s.dn[161][16] * ddt_scale));
        let eq110_e1479_d_n17: f64 = (p.p7 * (s.dn[161][17] * ddt_scale));
        let eq110_e1479_d_n18: f64 = (p.p7 * (s.dn[161][18] * ddt_scale));
        let eq110_e1479_d_n19: f64 = (p.p7 * (s.dn[161][19] * ddt_scale));
        let eq110_e1479_d_n20: f64 = (p.p7 * (s.dn[161][20] * ddt_scale));
        let eq110_e1479_d_n21: f64 = (p.p7 * (s.dn[161][21] * ddt_scale));
        let eq110_e1479_d_n22: f64 = (p.p7 * (s.dn[161][22] * ddt_scale));
        let eq110_e1479_d_b0: f64 = (p.p7 * (s.db[161][0] * ddt_scale));
        let eq110_e1479_d_b1: f64 = (p.p7 * (s.db[161][1] * ddt_scale));
        let eq110_e1479_d_b2: f64 = (p.p7 * (s.db[161][2] * ddt_scale));
        let eq110_e1479_d_b3: f64 = (p.p7 * (s.db[161][3] * ddt_scale));
        let eq110_e1479_d_b4: f64 = (p.p7 * (s.db[161][4] * ddt_scale));
        let eq110_e1479_d_b5: f64 = (p.p7 * (s.db[161][5] * ddt_scale));
        let eq110_e1479_d_b6: f64 = (p.p7 * (s.db[161][6] * ddt_scale));
        let eq110_e1479_d_b7: f64 = (p.p7 * (s.db[161][7] * ddt_scale));
        let eq110_e1479_d_b8: f64 = (p.p7 * (s.db[161][8] * ddt_scale));
        let eq110_e1479_d_b9: f64 = (p.p7 * (s.db[161][9] * ddt_scale));
        let eq110_e1479_d_b10: f64 = (p.p7 * (s.db[161][10] * ddt_scale));
        let eq110_e1479_d_b11: f64 = (p.p7 * (s.db[161][11] * ddt_scale));
        let eq110_e1479_d_b12: f64 = (p.p7 * (s.db[161][12] * ddt_scale));
        let eq110_e1479_d_b13: f64 = (p.p7 * (s.db[161][13] * ddt_scale));
        let eq110_e1479_d_b14: f64 = (p.p7 * (s.db[161][14] * ddt_scale));
        let eq110_e1479_d_b15: f64 = (p.p7 * (s.db[161][15] * ddt_scale));
        let eq110_e1479_d_b16: f64 = (p.p7 * (s.db[161][16] * ddt_scale));
        let eq110_e1479_d_b17: f64 = (p.p7 * (s.db[161][17] * ddt_scale));
        let eq110_e1479_d_b18: f64 = (p.p7 * (s.db[161][18] * ddt_scale));
        let eq110_e1479_d_b19: f64 = (p.p7 * (s.db[161][19] * ddt_scale));
        let eq110_e1479_d_b20: f64 = (p.p7 * (s.db[161][20] * ddt_scale));
        let eq110_e1479_d_b21: f64 = (p.p7 * (s.db[161][21] * ddt_scale));
        let eq110_e1479_d_b22: f64 = (p.p7 * (s.db[161][22] * ddt_scale));
        let eq110_e1479_d_b23: f64 = (p.p7 * (s.db[161][23] * ddt_scale));
        let eq110_e1479_d_b24: f64 = (p.p7 * (s.db[161][24] * ddt_scale));
        let eq110_e1479_d_b25: f64 = (p.p7 * (s.db[161][25] * ddt_scale));
        let eq110_e1479_d_b26: f64 = (p.p7 * (s.db[161][26] * ddt_scale));
        let eq110_e1479_d_b27: f64 = (p.p7 * (s.db[161][27] * ddt_scale));
        let eq110_e1479_d_b28: f64 = (p.p7 * (s.db[161][28] * ddt_scale));
        let eq110_e1479_d_b29: f64 = (p.p7 * (s.db[161][29] * ddt_scale));
        let eq110_e1479_d_b30: f64 = (p.p7 * (s.db[161][30] * ddt_scale));
        let eq110_e1479_d_b31: f64 = (p.p7 * (s.db[161][31] * ddt_scale));
        let eq110_e1479_d_b32: f64 = (p.p7 * (s.db[161][32] * ddt_scale));
        let eq110_e1479_d_b33: f64 = (p.p7 * (s.db[161][33] * ddt_scale));
        let eq110_e1479_d_b34: f64 = (p.p7 * (s.db[161][34] * ddt_scale));
        let eq110_e1479_d_b35: f64 = (p.p7 * (s.db[161][35] * ddt_scale));
        let eq110_e1479_d_b36: f64 = (p.p7 * (s.db[161][36] * ddt_scale));
        let eq110_e1479_d_b37: f64 = (p.p7 * (s.db[161][37] * ddt_scale));
        let eq110_e1479_d_b38: f64 = (p.p7 * (s.db[161][38] * ddt_scale));
        let eq110_e1479_d_b39: f64 = (p.p7 * (s.db[161][39] * ddt_scale));
        let eq110_e1479_d_b40: f64 = (p.p7 * (s.db[161][40] * ddt_scale));
        let eq110_e1479_d_b41: f64 = (p.p7 * (s.db[161][41] * ddt_scale));
        let eq110_e1479_d_b42: f64 = (p.p7 * (s.db[161][42] * ddt_scale));
        let eq110_e1479_d_b43: f64 = (p.p7 * (s.db[161][43] * ddt_scale));
        let eq110_e1479_d_b44: f64 = (p.p7 * (s.db[161][44] * ddt_scale));
        let eq110_e1479_d_b45: f64 = (p.p7 * (s.db[161][45] * ddt_scale));
        let eq110_e1479_d_b46: f64 = (p.p7 * (s.db[161][46] * ddt_scale));
        let eq110_e1479_d_b47: f64 = (p.p7 * (s.db[161][47] * ddt_scale));
        let eq110_e1479_d_b48: f64 = (p.p7 * (s.db[161][48] * ddt_scale));
        let eq110_e1479_d_b49: f64 = (p.p7 * (s.db[161][49] * ddt_scale));
        let eq110_e1479_d_b50: f64 = (p.p7 * (s.db[161][50] * ddt_scale));
        let eq110_e1479_d_b51: f64 = (p.p7 * (s.db[161][51] * ddt_scale));
        let eq110_e1479_d_b52: f64 = (p.p7 * (s.db[161][52] * ddt_scale));
        let eq110_e1479_d_b53: f64 = (p.p7 * (s.db[161][53] * ddt_scale));
        let eq110_e1479_d_b54: f64 = (p.p7 * (s.db[161][54] * ddt_scale));
        let eq110_value: f64 = eq110_e1479;
        let eq110_node_derivatives: [f64; 23] = [eq110_e1479_d_n0, eq110_e1479_d_n1, eq110_e1479_d_n2, eq110_e1479_d_n3, eq110_e1479_d_n4, eq110_e1479_d_n5, eq110_e1479_d_n6, eq110_e1479_d_n7, eq110_e1479_d_n8, eq110_e1479_d_n9, eq110_e1479_d_n10, eq110_e1479_d_n11, eq110_e1479_d_n12, eq110_e1479_d_n13, eq110_e1479_d_n14, eq110_e1479_d_n15, eq110_e1479_d_n16, eq110_e1479_d_n17, eq110_e1479_d_n18, eq110_e1479_d_n19, eq110_e1479_d_n20, eq110_e1479_d_n21, eq110_e1479_d_n22];
        let eq110_branch_derivatives: [f64; 55] = [eq110_e1479_d_b0, eq110_e1479_d_b1, eq110_e1479_d_b2, eq110_e1479_d_b3, eq110_e1479_d_b4, eq110_e1479_d_b5, eq110_e1479_d_b6, eq110_e1479_d_b7, eq110_e1479_d_b8, eq110_e1479_d_b9, eq110_e1479_d_b10, eq110_e1479_d_b11, eq110_e1479_d_b12, eq110_e1479_d_b13, eq110_e1479_d_b14, eq110_e1479_d_b15, eq110_e1479_d_b16, eq110_e1479_d_b17, eq110_e1479_d_b18, eq110_e1479_d_b19, eq110_e1479_d_b20, eq110_e1479_d_b21, eq110_e1479_d_b22, eq110_e1479_d_b23, eq110_e1479_d_b24, eq110_e1479_d_b25, eq110_e1479_d_b26, eq110_e1479_d_b27, eq110_e1479_d_b28, eq110_e1479_d_b29, eq110_e1479_d_b30, eq110_e1479_d_b31, eq110_e1479_d_b32, eq110_e1479_d_b33, eq110_e1479_d_b34, eq110_e1479_d_b35, eq110_e1479_d_b36, eq110_e1479_d_b37, eq110_e1479_d_b38, eq110_e1479_d_b39, eq110_e1479_d_b40, eq110_e1479_d_b41, eq110_e1479_d_b42, eq110_e1479_d_b43, eq110_e1479_d_b44, eq110_e1479_d_b45, eq110_e1479_d_b46, eq110_e1479_d_b47, eq110_e1479_d_b48, eq110_e1479_d_b49, eq110_e1479_d_b50, eq110_e1479_d_b51, eq110_e1479_d_b52, eq110_e1479_d_b53, eq110_e1479_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq110_value),
            &eq110_node_derivatives,
            &eq110_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_14(
        stamper: &mut GeneratedStamper<'_>,
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
        var_guard535: f64,
        var_qdov: f64,
        var_qdov_db0: f64,
        var_qdov_db1: f64,
        var_qdov_db10: f64,
        var_qdov_db11: f64,
        var_qdov_db12: f64,
        var_qdov_db13: f64,
        var_qdov_db14: f64,
        var_qdov_db15: f64,
        var_qdov_db16: f64,
        var_qdov_db17: f64,
        var_qdov_db18: f64,
        var_qdov_db19: f64,
        var_qdov_db2: f64,
        var_qdov_db20: f64,
        var_qdov_db21: f64,
        var_qdov_db22: f64,
        var_qdov_db23: f64,
        var_qdov_db24: f64,
        var_qdov_db25: f64,
        var_qdov_db26: f64,
        var_qdov_db27: f64,
        var_qdov_db28: f64,
        var_qdov_db29: f64,
        var_qdov_db3: f64,
        var_qdov_db30: f64,
        var_qdov_db31: f64,
        var_qdov_db32: f64,
        var_qdov_db33: f64,
        var_qdov_db34: f64,
        var_qdov_db35: f64,
        var_qdov_db36: f64,
        var_qdov_db37: f64,
        var_qdov_db38: f64,
        var_qdov_db39: f64,
        var_qdov_db4: f64,
        var_qdov_db40: f64,
        var_qdov_db41: f64,
        var_qdov_db42: f64,
        var_qdov_db43: f64,
        var_qdov_db44: f64,
        var_qdov_db45: f64,
        var_qdov_db46: f64,
        var_qdov_db47: f64,
        var_qdov_db48: f64,
        var_qdov_db49: f64,
        var_qdov_db5: f64,
        var_qdov_db50: f64,
        var_qdov_db51: f64,
        var_qdov_db52: f64,
        var_qdov_db53: f64,
        var_qdov_db54: f64,
        var_qdov_db6: f64,
        var_qdov_db7: f64,
        var_qdov_db8: f64,
        var_qdov_db9: f64,
        var_qdov_dn0: f64,
        var_qdov_dn1: f64,
        var_qdov_dn10: f64,
        var_qdov_dn11: f64,
        var_qdov_dn12: f64,
        var_qdov_dn13: f64,
        var_qdov_dn14: f64,
        var_qdov_dn15: f64,
        var_qdov_dn16: f64,
        var_qdov_dn17: f64,
        var_qdov_dn18: f64,
        var_qdov_dn19: f64,
        var_qdov_dn2: f64,
        var_qdov_dn20: f64,
        var_qdov_dn21: f64,
        var_qdov_dn22: f64,
        var_qdov_dn3: f64,
        var_qdov_dn4: f64,
        var_qdov_dn5: f64,
        var_qdov_dn6: f64,
        var_qdov_dn7: f64,
        var_qdov_dn8: f64,
        var_qdov_dn9: f64,
        var_qdsov: f64,
        var_qdsov_db0: f64,
        var_qdsov_db1: f64,
        var_qdsov_db10: f64,
        var_qdsov_db11: f64,
        var_qdsov_db12: f64,
        var_qdsov_db13: f64,
        var_qdsov_db14: f64,
        var_qdsov_db15: f64,
        var_qdsov_db16: f64,
        var_qdsov_db17: f64,
        var_qdsov_db18: f64,
        var_qdsov_db19: f64,
        var_qdsov_db2: f64,
        var_qdsov_db20: f64,
        var_qdsov_db21: f64,
        var_qdsov_db22: f64,
        var_qdsov_db23: f64,
        var_qdsov_db24: f64,
        var_qdsov_db25: f64,
        var_qdsov_db26: f64,
        var_qdsov_db27: f64,
        var_qdsov_db28: f64,
        var_qdsov_db29: f64,
        var_qdsov_db3: f64,
        var_qdsov_db30: f64,
        var_qdsov_db31: f64,
        var_qdsov_db32: f64,
        var_qdsov_db33: f64,
        var_qdsov_db34: f64,
        var_qdsov_db35: f64,
        var_qdsov_db36: f64,
        var_qdsov_db37: f64,
        var_qdsov_db38: f64,
        var_qdsov_db39: f64,
        var_qdsov_db4: f64,
        var_qdsov_db40: f64,
        var_qdsov_db41: f64,
        var_qdsov_db42: f64,
        var_qdsov_db43: f64,
        var_qdsov_db44: f64,
        var_qdsov_db45: f64,
        var_qdsov_db46: f64,
        var_qdsov_db47: f64,
        var_qdsov_db48: f64,
        var_qdsov_db49: f64,
        var_qdsov_db5: f64,
        var_qdsov_db50: f64,
        var_qdsov_db51: f64,
        var_qdsov_db52: f64,
        var_qdsov_db53: f64,
        var_qdsov_db54: f64,
        var_qdsov_db6: f64,
        var_qdsov_db7: f64,
        var_qdsov_db8: f64,
        var_qdsov_db9: f64,
        var_qdsov_dn0: f64,
        var_qdsov_dn1: f64,
        var_qdsov_dn10: f64,
        var_qdsov_dn11: f64,
        var_qdsov_dn12: f64,
        var_qdsov_dn13: f64,
        var_qdsov_dn14: f64,
        var_qdsov_dn15: f64,
        var_qdsov_dn16: f64,
        var_qdsov_dn17: f64,
        var_qdsov_dn18: f64,
        var_qdsov_dn19: f64,
        var_qdsov_dn2: f64,
        var_qdsov_dn20: f64,
        var_qdsov_dn21: f64,
        var_qdsov_dn22: f64,
        var_qdsov_dn3: f64,
        var_qdsov_dn4: f64,
        var_qdsov_dn5: f64,
        var_qdsov_dn6: f64,
        var_qdsov_dn7: f64,
        var_qdsov_dn8: f64,
        var_qdsov_dn9: f64,
        var_qsov: f64,
        var_qsov_db0: f64,
        var_qsov_db1: f64,
        var_qsov_db10: f64,
        var_qsov_db11: f64,
        var_qsov_db12: f64,
        var_qsov_db13: f64,
        var_qsov_db14: f64,
        var_qsov_db15: f64,
        var_qsov_db16: f64,
        var_qsov_db17: f64,
        var_qsov_db18: f64,
        var_qsov_db19: f64,
        var_qsov_db2: f64,
        var_qsov_db20: f64,
        var_qsov_db21: f64,
        var_qsov_db22: f64,
        var_qsov_db23: f64,
        var_qsov_db24: f64,
        var_qsov_db25: f64,
        var_qsov_db26: f64,
        var_qsov_db27: f64,
        var_qsov_db28: f64,
        var_qsov_db29: f64,
        var_qsov_db3: f64,
        var_qsov_db30: f64,
        var_qsov_db31: f64,
        var_qsov_db32: f64,
        var_qsov_db33: f64,
        var_qsov_db34: f64,
        var_qsov_db35: f64,
        var_qsov_db36: f64,
        var_qsov_db37: f64,
        var_qsov_db38: f64,
        var_qsov_db39: f64,
        var_qsov_db4: f64,
        var_qsov_db40: f64,
        var_qsov_db41: f64,
        var_qsov_db42: f64,
        var_qsov_db43: f64,
        var_qsov_db44: f64,
        var_qsov_db45: f64,
        var_qsov_db46: f64,
        var_qsov_db47: f64,
        var_qsov_db48: f64,
        var_qsov_db49: f64,
        var_qsov_db5: f64,
        var_qsov_db50: f64,
        var_qsov_db51: f64,
        var_qsov_db52: f64,
        var_qsov_db53: f64,
        var_qsov_db54: f64,
        var_qsov_db6: f64,
        var_qsov_db7: f64,
        var_qsov_db8: f64,
        var_qsov_db9: f64,
        var_qsov_dn0: f64,
        var_qsov_dn1: f64,
        var_qsov_dn10: f64,
        var_qsov_dn11: f64,
        var_qsov_dn12: f64,
        var_qsov_dn13: f64,
        var_qsov_dn14: f64,
        var_qsov_dn15: f64,
        var_qsov_dn16: f64,
        var_qsov_dn17: f64,
        var_qsov_dn18: f64,
        var_qsov_dn19: f64,
        var_qsov_dn2: f64,
        var_qsov_dn20: f64,
        var_qsov_dn21: f64,
        var_qsov_dn22: f64,
        var_qsov_dn3: f64,
        var_qsov_dn4: f64,
        var_qsov_dn5: f64,
        var_qsov_dn6: f64,
        var_qsov_dn7: f64,
        var_qsov_dn8: f64,
        var_qsov_dn9: f64,
    ) {
        let (eq111_e1486, eq111_e1486_d_n0, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n3, eq111_e1486_d_n4, eq111_e1486_d_n5, eq111_e1486_d_n6, eq111_e1486_d_n7, eq111_e1486_d_n8, eq111_e1486_d_n9, eq111_e1486_d_n10, eq111_e1486_d_n11, eq111_e1486_d_n12, eq111_e1486_d_n13, eq111_e1486_d_n14, eq111_e1486_d_n15, eq111_e1486_d_n16, eq111_e1486_d_n17, eq111_e1486_d_n18, eq111_e1486_d_n19, eq111_e1486_d_n20, eq111_e1486_d_n21, eq111_e1486_d_n22, eq111_e1486_d_b0, eq111_e1486_d_b1, eq111_e1486_d_b2, eq111_e1486_d_b3, eq111_e1486_d_b4, eq111_e1486_d_b5, eq111_e1486_d_b6, eq111_e1486_d_b7, eq111_e1486_d_b8, eq111_e1486_d_b9, eq111_e1486_d_b10, eq111_e1486_d_b11, eq111_e1486_d_b12, eq111_e1486_d_b13, eq111_e1486_d_b14, eq111_e1486_d_b15, eq111_e1486_d_b16, eq111_e1486_d_b17, eq111_e1486_d_b18, eq111_e1486_d_b19, eq111_e1486_d_b20, eq111_e1486_d_b21, eq111_e1486_d_b22, eq111_e1486_d_b23, eq111_e1486_d_b24, eq111_e1486_d_b25, eq111_e1486_d_b26, eq111_e1486_d_b27, eq111_e1486_d_b28, eq111_e1486_d_b29, eq111_e1486_d_b30, eq111_e1486_d_b31, eq111_e1486_d_b32, eq111_e1486_d_b33, eq111_e1486_d_b34, eq111_e1486_d_b35, eq111_e1486_d_b36, eq111_e1486_d_b37, eq111_e1486_d_b38, eq111_e1486_d_b39, eq111_e1486_d_b40, eq111_e1486_d_b41, eq111_e1486_d_b42, eq111_e1486_d_b43, eq111_e1486_d_b44, eq111_e1486_d_b45, eq111_e1486_d_b46, eq111_e1486_d_b47, eq111_e1486_d_b48, eq111_e1486_d_b49, eq111_e1486_d_b50, eq111_e1486_d_b51, eq111_e1486_d_b52, eq111_e1486_d_b53, eq111_e1486_d_b54,) = {
    if (var_guard535 != 0.0) {
        let eq111_e1483: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, var_qsov);
        let eq111_e1484: f64 = (p.p7 * eq111_e1483);
        let eq111_e1484_d_n0: f64 = (p.p7 * (var_qsov_dn0 * ddt_scale));
        let eq111_e1484_d_n1: f64 = (p.p7 * (var_qsov_dn1 * ddt_scale));
        let eq111_e1484_d_n2: f64 = (p.p7 * (var_qsov_dn2 * ddt_scale));
        let eq111_e1484_d_n3: f64 = (p.p7 * (var_qsov_dn3 * ddt_scale));
        let eq111_e1484_d_n4: f64 = (p.p7 * (var_qsov_dn4 * ddt_scale));
        let eq111_e1484_d_n5: f64 = (p.p7 * (var_qsov_dn5 * ddt_scale));
        let eq111_e1484_d_n6: f64 = (p.p7 * (var_qsov_dn6 * ddt_scale));
        let eq111_e1484_d_n7: f64 = (p.p7 * (var_qsov_dn7 * ddt_scale));
        let eq111_e1484_d_n8: f64 = (p.p7 * (var_qsov_dn8 * ddt_scale));
        let eq111_e1484_d_n9: f64 = (p.p7 * (var_qsov_dn9 * ddt_scale));
        let eq111_e1484_d_n10: f64 = (p.p7 * (var_qsov_dn10 * ddt_scale));
        let eq111_e1484_d_n11: f64 = (p.p7 * (var_qsov_dn11 * ddt_scale));
        let eq111_e1484_d_n12: f64 = (p.p7 * (var_qsov_dn12 * ddt_scale));
        let eq111_e1484_d_n13: f64 = (p.p7 * (var_qsov_dn13 * ddt_scale));
        let eq111_e1484_d_n14: f64 = (p.p7 * (var_qsov_dn14 * ddt_scale));
        let eq111_e1484_d_n15: f64 = (p.p7 * (var_qsov_dn15 * ddt_scale));
        let eq111_e1484_d_n16: f64 = (p.p7 * (var_qsov_dn16 * ddt_scale));
        let eq111_e1484_d_n17: f64 = (p.p7 * (var_qsov_dn17 * ddt_scale));
        let eq111_e1484_d_n18: f64 = (p.p7 * (var_qsov_dn18 * ddt_scale));
        let eq111_e1484_d_n19: f64 = (p.p7 * (var_qsov_dn19 * ddt_scale));
        let eq111_e1484_d_n20: f64 = (p.p7 * (var_qsov_dn20 * ddt_scale));
        let eq111_e1484_d_n21: f64 = (p.p7 * (var_qsov_dn21 * ddt_scale));
        let eq111_e1484_d_n22: f64 = (p.p7 * (var_qsov_dn22 * ddt_scale));
        let eq111_e1484_d_b0: f64 = (p.p7 * (var_qsov_db0 * ddt_scale));
        let eq111_e1484_d_b1: f64 = (p.p7 * (var_qsov_db1 * ddt_scale));
        let eq111_e1484_d_b2: f64 = (p.p7 * (var_qsov_db2 * ddt_scale));
        let eq111_e1484_d_b3: f64 = (p.p7 * (var_qsov_db3 * ddt_scale));
        let eq111_e1484_d_b4: f64 = (p.p7 * (var_qsov_db4 * ddt_scale));
        let eq111_e1484_d_b5: f64 = (p.p7 * (var_qsov_db5 * ddt_scale));
        let eq111_e1484_d_b6: f64 = (p.p7 * (var_qsov_db6 * ddt_scale));
        let eq111_e1484_d_b7: f64 = (p.p7 * (var_qsov_db7 * ddt_scale));
        let eq111_e1484_d_b8: f64 = (p.p7 * (var_qsov_db8 * ddt_scale));
        let eq111_e1484_d_b9: f64 = (p.p7 * (var_qsov_db9 * ddt_scale));
        let eq111_e1484_d_b10: f64 = (p.p7 * (var_qsov_db10 * ddt_scale));
        let eq111_e1484_d_b11: f64 = (p.p7 * (var_qsov_db11 * ddt_scale));
        let eq111_e1484_d_b12: f64 = (p.p7 * (var_qsov_db12 * ddt_scale));
        let eq111_e1484_d_b13: f64 = (p.p7 * (var_qsov_db13 * ddt_scale));
        let eq111_e1484_d_b14: f64 = (p.p7 * (var_qsov_db14 * ddt_scale));
        let eq111_e1484_d_b15: f64 = (p.p7 * (var_qsov_db15 * ddt_scale));
        let eq111_e1484_d_b16: f64 = (p.p7 * (var_qsov_db16 * ddt_scale));
        let eq111_e1484_d_b17: f64 = (p.p7 * (var_qsov_db17 * ddt_scale));
        let eq111_e1484_d_b18: f64 = (p.p7 * (var_qsov_db18 * ddt_scale));
        let eq111_e1484_d_b19: f64 = (p.p7 * (var_qsov_db19 * ddt_scale));
        let eq111_e1484_d_b20: f64 = (p.p7 * (var_qsov_db20 * ddt_scale));
        let eq111_e1484_d_b21: f64 = (p.p7 * (var_qsov_db21 * ddt_scale));
        let eq111_e1484_d_b22: f64 = (p.p7 * (var_qsov_db22 * ddt_scale));
        let eq111_e1484_d_b23: f64 = (p.p7 * (var_qsov_db23 * ddt_scale));
        let eq111_e1484_d_b24: f64 = (p.p7 * (var_qsov_db24 * ddt_scale));
        let eq111_e1484_d_b25: f64 = (p.p7 * (var_qsov_db25 * ddt_scale));
        let eq111_e1484_d_b26: f64 = (p.p7 * (var_qsov_db26 * ddt_scale));
        let eq111_e1484_d_b27: f64 = (p.p7 * (var_qsov_db27 * ddt_scale));
        let eq111_e1484_d_b28: f64 = (p.p7 * (var_qsov_db28 * ddt_scale));
        let eq111_e1484_d_b29: f64 = (p.p7 * (var_qsov_db29 * ddt_scale));
        let eq111_e1484_d_b30: f64 = (p.p7 * (var_qsov_db30 * ddt_scale));
        let eq111_e1484_d_b31: f64 = (p.p7 * (var_qsov_db31 * ddt_scale));
        let eq111_e1484_d_b32: f64 = (p.p7 * (var_qsov_db32 * ddt_scale));
        let eq111_e1484_d_b33: f64 = (p.p7 * (var_qsov_db33 * ddt_scale));
        let eq111_e1484_d_b34: f64 = (p.p7 * (var_qsov_db34 * ddt_scale));
        let eq111_e1484_d_b35: f64 = (p.p7 * (var_qsov_db35 * ddt_scale));
        let eq111_e1484_d_b36: f64 = (p.p7 * (var_qsov_db36 * ddt_scale));
        let eq111_e1484_d_b37: f64 = (p.p7 * (var_qsov_db37 * ddt_scale));
        let eq111_e1484_d_b38: f64 = (p.p7 * (var_qsov_db38 * ddt_scale));
        let eq111_e1484_d_b39: f64 = (p.p7 * (var_qsov_db39 * ddt_scale));
        let eq111_e1484_d_b40: f64 = (p.p7 * (var_qsov_db40 * ddt_scale));
        let eq111_e1484_d_b41: f64 = (p.p7 * (var_qsov_db41 * ddt_scale));
        let eq111_e1484_d_b42: f64 = (p.p7 * (var_qsov_db42 * ddt_scale));
        let eq111_e1484_d_b43: f64 = (p.p7 * (var_qsov_db43 * ddt_scale));
        let eq111_e1484_d_b44: f64 = (p.p7 * (var_qsov_db44 * ddt_scale));
        let eq111_e1484_d_b45: f64 = (p.p7 * (var_qsov_db45 * ddt_scale));
        let eq111_e1484_d_b46: f64 = (p.p7 * (var_qsov_db46 * ddt_scale));
        let eq111_e1484_d_b47: f64 = (p.p7 * (var_qsov_db47 * ddt_scale));
        let eq111_e1484_d_b48: f64 = (p.p7 * (var_qsov_db48 * ddt_scale));
        let eq111_e1484_d_b49: f64 = (p.p7 * (var_qsov_db49 * ddt_scale));
        let eq111_e1484_d_b50: f64 = (p.p7 * (var_qsov_db50 * ddt_scale));
        let eq111_e1484_d_b51: f64 = (p.p7 * (var_qsov_db51 * ddt_scale));
        let eq111_e1484_d_b52: f64 = (p.p7 * (var_qsov_db52 * ddt_scale));
        let eq111_e1484_d_b53: f64 = (p.p7 * (var_qsov_db53 * ddt_scale));
        let eq111_e1484_d_b54: f64 = (p.p7 * (var_qsov_db54 * ddt_scale));
        (eq111_e1484, eq111_e1484_d_n0, eq111_e1484_d_n1, eq111_e1484_d_n2, eq111_e1484_d_n3, eq111_e1484_d_n4, eq111_e1484_d_n5, eq111_e1484_d_n6, eq111_e1484_d_n7, eq111_e1484_d_n8, eq111_e1484_d_n9, eq111_e1484_d_n10, eq111_e1484_d_n11, eq111_e1484_d_n12, eq111_e1484_d_n13, eq111_e1484_d_n14, eq111_e1484_d_n15, eq111_e1484_d_n16, eq111_e1484_d_n17, eq111_e1484_d_n18, eq111_e1484_d_n19, eq111_e1484_d_n20, eq111_e1484_d_n21, eq111_e1484_d_n22, eq111_e1484_d_b0, eq111_e1484_d_b1, eq111_e1484_d_b2, eq111_e1484_d_b3, eq111_e1484_d_b4, eq111_e1484_d_b5, eq111_e1484_d_b6, eq111_e1484_d_b7, eq111_e1484_d_b8, eq111_e1484_d_b9, eq111_e1484_d_b10, eq111_e1484_d_b11, eq111_e1484_d_b12, eq111_e1484_d_b13, eq111_e1484_d_b14, eq111_e1484_d_b15, eq111_e1484_d_b16, eq111_e1484_d_b17, eq111_e1484_d_b18, eq111_e1484_d_b19, eq111_e1484_d_b20, eq111_e1484_d_b21, eq111_e1484_d_b22, eq111_e1484_d_b23, eq111_e1484_d_b24, eq111_e1484_d_b25, eq111_e1484_d_b26, eq111_e1484_d_b27, eq111_e1484_d_b28, eq111_e1484_d_b29, eq111_e1484_d_b30, eq111_e1484_d_b31, eq111_e1484_d_b32, eq111_e1484_d_b33, eq111_e1484_d_b34, eq111_e1484_d_b35, eq111_e1484_d_b36, eq111_e1484_d_b37, eq111_e1484_d_b38, eq111_e1484_d_b39, eq111_e1484_d_b40, eq111_e1484_d_b41, eq111_e1484_d_b42, eq111_e1484_d_b43, eq111_e1484_d_b44, eq111_e1484_d_b45, eq111_e1484_d_b46, eq111_e1484_d_b47, eq111_e1484_d_b48, eq111_e1484_d_b49, eq111_e1484_d_b50, eq111_e1484_d_b51, eq111_e1484_d_b52, eq111_e1484_d_b53, eq111_e1484_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e1486;
        let eq111_node_derivatives: [f64; 23] = [eq111_e1486_d_n0, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n3, eq111_e1486_d_n4, eq111_e1486_d_n5, eq111_e1486_d_n6, eq111_e1486_d_n7, eq111_e1486_d_n8, eq111_e1486_d_n9, eq111_e1486_d_n10, eq111_e1486_d_n11, eq111_e1486_d_n12, eq111_e1486_d_n13, eq111_e1486_d_n14, eq111_e1486_d_n15, eq111_e1486_d_n16, eq111_e1486_d_n17, eq111_e1486_d_n18, eq111_e1486_d_n19, eq111_e1486_d_n20, eq111_e1486_d_n21, eq111_e1486_d_n22];
        let eq111_branch_derivatives: [f64; 55] = [eq111_e1486_d_b0, eq111_e1486_d_b1, eq111_e1486_d_b2, eq111_e1486_d_b3, eq111_e1486_d_b4, eq111_e1486_d_b5, eq111_e1486_d_b6, eq111_e1486_d_b7, eq111_e1486_d_b8, eq111_e1486_d_b9, eq111_e1486_d_b10, eq111_e1486_d_b11, eq111_e1486_d_b12, eq111_e1486_d_b13, eq111_e1486_d_b14, eq111_e1486_d_b15, eq111_e1486_d_b16, eq111_e1486_d_b17, eq111_e1486_d_b18, eq111_e1486_d_b19, eq111_e1486_d_b20, eq111_e1486_d_b21, eq111_e1486_d_b22, eq111_e1486_d_b23, eq111_e1486_d_b24, eq111_e1486_d_b25, eq111_e1486_d_b26, eq111_e1486_d_b27, eq111_e1486_d_b28, eq111_e1486_d_b29, eq111_e1486_d_b30, eq111_e1486_d_b31, eq111_e1486_d_b32, eq111_e1486_d_b33, eq111_e1486_d_b34, eq111_e1486_d_b35, eq111_e1486_d_b36, eq111_e1486_d_b37, eq111_e1486_d_b38, eq111_e1486_d_b39, eq111_e1486_d_b40, eq111_e1486_d_b41, eq111_e1486_d_b42, eq111_e1486_d_b43, eq111_e1486_d_b44, eq111_e1486_d_b45, eq111_e1486_d_b46, eq111_e1486_d_b47, eq111_e1486_d_b48, eq111_e1486_d_b49, eq111_e1486_d_b50, eq111_e1486_d_b51, eq111_e1486_d_b52, eq111_e1486_d_b53, eq111_e1486_d_b54];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq111_value),
            &eq111_node_derivatives,
            &eq111_branch_derivatives,
            multiplicity,
        );
        let (eq112_e1493, eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n3, eq112_e1493_d_n4, eq112_e1493_d_n5, eq112_e1493_d_n6, eq112_e1493_d_n7, eq112_e1493_d_n8, eq112_e1493_d_n9, eq112_e1493_d_n10, eq112_e1493_d_n11, eq112_e1493_d_n12, eq112_e1493_d_n13, eq112_e1493_d_n14, eq112_e1493_d_n15, eq112_e1493_d_n16, eq112_e1493_d_n17, eq112_e1493_d_n18, eq112_e1493_d_n19, eq112_e1493_d_n20, eq112_e1493_d_n21, eq112_e1493_d_n22, eq112_e1493_d_b0, eq112_e1493_d_b1, eq112_e1493_d_b2, eq112_e1493_d_b3, eq112_e1493_d_b4, eq112_e1493_d_b5, eq112_e1493_d_b6, eq112_e1493_d_b7, eq112_e1493_d_b8, eq112_e1493_d_b9, eq112_e1493_d_b10, eq112_e1493_d_b11, eq112_e1493_d_b12, eq112_e1493_d_b13, eq112_e1493_d_b14, eq112_e1493_d_b15, eq112_e1493_d_b16, eq112_e1493_d_b17, eq112_e1493_d_b18, eq112_e1493_d_b19, eq112_e1493_d_b20, eq112_e1493_d_b21, eq112_e1493_d_b22, eq112_e1493_d_b23, eq112_e1493_d_b24, eq112_e1493_d_b25, eq112_e1493_d_b26, eq112_e1493_d_b27, eq112_e1493_d_b28, eq112_e1493_d_b29, eq112_e1493_d_b30, eq112_e1493_d_b31, eq112_e1493_d_b32, eq112_e1493_d_b33, eq112_e1493_d_b34, eq112_e1493_d_b35, eq112_e1493_d_b36, eq112_e1493_d_b37, eq112_e1493_d_b38, eq112_e1493_d_b39, eq112_e1493_d_b40, eq112_e1493_d_b41, eq112_e1493_d_b42, eq112_e1493_d_b43, eq112_e1493_d_b44, eq112_e1493_d_b45, eq112_e1493_d_b46, eq112_e1493_d_b47, eq112_e1493_d_b48, eq112_e1493_d_b49, eq112_e1493_d_b50, eq112_e1493_d_b51, eq112_e1493_d_b52, eq112_e1493_d_b53, eq112_e1493_d_b54,) = {
    if (var_guard535 != 0.0) {
        let eq112_e1490: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, var_qdov);
        let eq112_e1491: f64 = (p.p7 * eq112_e1490);
        let eq112_e1491_d_n0: f64 = (p.p7 * (var_qdov_dn0 * ddt_scale));
        let eq112_e1491_d_n1: f64 = (p.p7 * (var_qdov_dn1 * ddt_scale));
        let eq112_e1491_d_n2: f64 = (p.p7 * (var_qdov_dn2 * ddt_scale));
        let eq112_e1491_d_n3: f64 = (p.p7 * (var_qdov_dn3 * ddt_scale));
        let eq112_e1491_d_n4: f64 = (p.p7 * (var_qdov_dn4 * ddt_scale));
        let eq112_e1491_d_n5: f64 = (p.p7 * (var_qdov_dn5 * ddt_scale));
        let eq112_e1491_d_n6: f64 = (p.p7 * (var_qdov_dn6 * ddt_scale));
        let eq112_e1491_d_n7: f64 = (p.p7 * (var_qdov_dn7 * ddt_scale));
        let eq112_e1491_d_n8: f64 = (p.p7 * (var_qdov_dn8 * ddt_scale));
        let eq112_e1491_d_n9: f64 = (p.p7 * (var_qdov_dn9 * ddt_scale));
        let eq112_e1491_d_n10: f64 = (p.p7 * (var_qdov_dn10 * ddt_scale));
        let eq112_e1491_d_n11: f64 = (p.p7 * (var_qdov_dn11 * ddt_scale));
        let eq112_e1491_d_n12: f64 = (p.p7 * (var_qdov_dn12 * ddt_scale));
        let eq112_e1491_d_n13: f64 = (p.p7 * (var_qdov_dn13 * ddt_scale));
        let eq112_e1491_d_n14: f64 = (p.p7 * (var_qdov_dn14 * ddt_scale));
        let eq112_e1491_d_n15: f64 = (p.p7 * (var_qdov_dn15 * ddt_scale));
        let eq112_e1491_d_n16: f64 = (p.p7 * (var_qdov_dn16 * ddt_scale));
        let eq112_e1491_d_n17: f64 = (p.p7 * (var_qdov_dn17 * ddt_scale));
        let eq112_e1491_d_n18: f64 = (p.p7 * (var_qdov_dn18 * ddt_scale));
        let eq112_e1491_d_n19: f64 = (p.p7 * (var_qdov_dn19 * ddt_scale));
        let eq112_e1491_d_n20: f64 = (p.p7 * (var_qdov_dn20 * ddt_scale));
        let eq112_e1491_d_n21: f64 = (p.p7 * (var_qdov_dn21 * ddt_scale));
        let eq112_e1491_d_n22: f64 = (p.p7 * (var_qdov_dn22 * ddt_scale));
        let eq112_e1491_d_b0: f64 = (p.p7 * (var_qdov_db0 * ddt_scale));
        let eq112_e1491_d_b1: f64 = (p.p7 * (var_qdov_db1 * ddt_scale));
        let eq112_e1491_d_b2: f64 = (p.p7 * (var_qdov_db2 * ddt_scale));
        let eq112_e1491_d_b3: f64 = (p.p7 * (var_qdov_db3 * ddt_scale));
        let eq112_e1491_d_b4: f64 = (p.p7 * (var_qdov_db4 * ddt_scale));
        let eq112_e1491_d_b5: f64 = (p.p7 * (var_qdov_db5 * ddt_scale));
        let eq112_e1491_d_b6: f64 = (p.p7 * (var_qdov_db6 * ddt_scale));
        let eq112_e1491_d_b7: f64 = (p.p7 * (var_qdov_db7 * ddt_scale));
        let eq112_e1491_d_b8: f64 = (p.p7 * (var_qdov_db8 * ddt_scale));
        let eq112_e1491_d_b9: f64 = (p.p7 * (var_qdov_db9 * ddt_scale));
        let eq112_e1491_d_b10: f64 = (p.p7 * (var_qdov_db10 * ddt_scale));
        let eq112_e1491_d_b11: f64 = (p.p7 * (var_qdov_db11 * ddt_scale));
        let eq112_e1491_d_b12: f64 = (p.p7 * (var_qdov_db12 * ddt_scale));
        let eq112_e1491_d_b13: f64 = (p.p7 * (var_qdov_db13 * ddt_scale));
        let eq112_e1491_d_b14: f64 = (p.p7 * (var_qdov_db14 * ddt_scale));
        let eq112_e1491_d_b15: f64 = (p.p7 * (var_qdov_db15 * ddt_scale));
        let eq112_e1491_d_b16: f64 = (p.p7 * (var_qdov_db16 * ddt_scale));
        let eq112_e1491_d_b17: f64 = (p.p7 * (var_qdov_db17 * ddt_scale));
        let eq112_e1491_d_b18: f64 = (p.p7 * (var_qdov_db18 * ddt_scale));
        let eq112_e1491_d_b19: f64 = (p.p7 * (var_qdov_db19 * ddt_scale));
        let eq112_e1491_d_b20: f64 = (p.p7 * (var_qdov_db20 * ddt_scale));
        let eq112_e1491_d_b21: f64 = (p.p7 * (var_qdov_db21 * ddt_scale));
        let eq112_e1491_d_b22: f64 = (p.p7 * (var_qdov_db22 * ddt_scale));
        let eq112_e1491_d_b23: f64 = (p.p7 * (var_qdov_db23 * ddt_scale));
        let eq112_e1491_d_b24: f64 = (p.p7 * (var_qdov_db24 * ddt_scale));
        let eq112_e1491_d_b25: f64 = (p.p7 * (var_qdov_db25 * ddt_scale));
        let eq112_e1491_d_b26: f64 = (p.p7 * (var_qdov_db26 * ddt_scale));
        let eq112_e1491_d_b27: f64 = (p.p7 * (var_qdov_db27 * ddt_scale));
        let eq112_e1491_d_b28: f64 = (p.p7 * (var_qdov_db28 * ddt_scale));
        let eq112_e1491_d_b29: f64 = (p.p7 * (var_qdov_db29 * ddt_scale));
        let eq112_e1491_d_b30: f64 = (p.p7 * (var_qdov_db30 * ddt_scale));
        let eq112_e1491_d_b31: f64 = (p.p7 * (var_qdov_db31 * ddt_scale));
        let eq112_e1491_d_b32: f64 = (p.p7 * (var_qdov_db32 * ddt_scale));
        let eq112_e1491_d_b33: f64 = (p.p7 * (var_qdov_db33 * ddt_scale));
        let eq112_e1491_d_b34: f64 = (p.p7 * (var_qdov_db34 * ddt_scale));
        let eq112_e1491_d_b35: f64 = (p.p7 * (var_qdov_db35 * ddt_scale));
        let eq112_e1491_d_b36: f64 = (p.p7 * (var_qdov_db36 * ddt_scale));
        let eq112_e1491_d_b37: f64 = (p.p7 * (var_qdov_db37 * ddt_scale));
        let eq112_e1491_d_b38: f64 = (p.p7 * (var_qdov_db38 * ddt_scale));
        let eq112_e1491_d_b39: f64 = (p.p7 * (var_qdov_db39 * ddt_scale));
        let eq112_e1491_d_b40: f64 = (p.p7 * (var_qdov_db40 * ddt_scale));
        let eq112_e1491_d_b41: f64 = (p.p7 * (var_qdov_db41 * ddt_scale));
        let eq112_e1491_d_b42: f64 = (p.p7 * (var_qdov_db42 * ddt_scale));
        let eq112_e1491_d_b43: f64 = (p.p7 * (var_qdov_db43 * ddt_scale));
        let eq112_e1491_d_b44: f64 = (p.p7 * (var_qdov_db44 * ddt_scale));
        let eq112_e1491_d_b45: f64 = (p.p7 * (var_qdov_db45 * ddt_scale));
        let eq112_e1491_d_b46: f64 = (p.p7 * (var_qdov_db46 * ddt_scale));
        let eq112_e1491_d_b47: f64 = (p.p7 * (var_qdov_db47 * ddt_scale));
        let eq112_e1491_d_b48: f64 = (p.p7 * (var_qdov_db48 * ddt_scale));
        let eq112_e1491_d_b49: f64 = (p.p7 * (var_qdov_db49 * ddt_scale));
        let eq112_e1491_d_b50: f64 = (p.p7 * (var_qdov_db50 * ddt_scale));
        let eq112_e1491_d_b51: f64 = (p.p7 * (var_qdov_db51 * ddt_scale));
        let eq112_e1491_d_b52: f64 = (p.p7 * (var_qdov_db52 * ddt_scale));
        let eq112_e1491_d_b53: f64 = (p.p7 * (var_qdov_db53 * ddt_scale));
        let eq112_e1491_d_b54: f64 = (p.p7 * (var_qdov_db54 * ddt_scale));
        (eq112_e1491, eq112_e1491_d_n0, eq112_e1491_d_n1, eq112_e1491_d_n2, eq112_e1491_d_n3, eq112_e1491_d_n4, eq112_e1491_d_n5, eq112_e1491_d_n6, eq112_e1491_d_n7, eq112_e1491_d_n8, eq112_e1491_d_n9, eq112_e1491_d_n10, eq112_e1491_d_n11, eq112_e1491_d_n12, eq112_e1491_d_n13, eq112_e1491_d_n14, eq112_e1491_d_n15, eq112_e1491_d_n16, eq112_e1491_d_n17, eq112_e1491_d_n18, eq112_e1491_d_n19, eq112_e1491_d_n20, eq112_e1491_d_n21, eq112_e1491_d_n22, eq112_e1491_d_b0, eq112_e1491_d_b1, eq112_e1491_d_b2, eq112_e1491_d_b3, eq112_e1491_d_b4, eq112_e1491_d_b5, eq112_e1491_d_b6, eq112_e1491_d_b7, eq112_e1491_d_b8, eq112_e1491_d_b9, eq112_e1491_d_b10, eq112_e1491_d_b11, eq112_e1491_d_b12, eq112_e1491_d_b13, eq112_e1491_d_b14, eq112_e1491_d_b15, eq112_e1491_d_b16, eq112_e1491_d_b17, eq112_e1491_d_b18, eq112_e1491_d_b19, eq112_e1491_d_b20, eq112_e1491_d_b21, eq112_e1491_d_b22, eq112_e1491_d_b23, eq112_e1491_d_b24, eq112_e1491_d_b25, eq112_e1491_d_b26, eq112_e1491_d_b27, eq112_e1491_d_b28, eq112_e1491_d_b29, eq112_e1491_d_b30, eq112_e1491_d_b31, eq112_e1491_d_b32, eq112_e1491_d_b33, eq112_e1491_d_b34, eq112_e1491_d_b35, eq112_e1491_d_b36, eq112_e1491_d_b37, eq112_e1491_d_b38, eq112_e1491_d_b39, eq112_e1491_d_b40, eq112_e1491_d_b41, eq112_e1491_d_b42, eq112_e1491_d_b43, eq112_e1491_d_b44, eq112_e1491_d_b45, eq112_e1491_d_b46, eq112_e1491_d_b47, eq112_e1491_d_b48, eq112_e1491_d_b49, eq112_e1491_d_b50, eq112_e1491_d_b51, eq112_e1491_d_b52, eq112_e1491_d_b53, eq112_e1491_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_value: f64 = eq112_e1493;
        let eq112_node_derivatives: [f64; 23] = [eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n3, eq112_e1493_d_n4, eq112_e1493_d_n5, eq112_e1493_d_n6, eq112_e1493_d_n7, eq112_e1493_d_n8, eq112_e1493_d_n9, eq112_e1493_d_n10, eq112_e1493_d_n11, eq112_e1493_d_n12, eq112_e1493_d_n13, eq112_e1493_d_n14, eq112_e1493_d_n15, eq112_e1493_d_n16, eq112_e1493_d_n17, eq112_e1493_d_n18, eq112_e1493_d_n19, eq112_e1493_d_n20, eq112_e1493_d_n21, eq112_e1493_d_n22];
        let eq112_branch_derivatives: [f64; 55] = [eq112_e1493_d_b0, eq112_e1493_d_b1, eq112_e1493_d_b2, eq112_e1493_d_b3, eq112_e1493_d_b4, eq112_e1493_d_b5, eq112_e1493_d_b6, eq112_e1493_d_b7, eq112_e1493_d_b8, eq112_e1493_d_b9, eq112_e1493_d_b10, eq112_e1493_d_b11, eq112_e1493_d_b12, eq112_e1493_d_b13, eq112_e1493_d_b14, eq112_e1493_d_b15, eq112_e1493_d_b16, eq112_e1493_d_b17, eq112_e1493_d_b18, eq112_e1493_d_b19, eq112_e1493_d_b20, eq112_e1493_d_b21, eq112_e1493_d_b22, eq112_e1493_d_b23, eq112_e1493_d_b24, eq112_e1493_d_b25, eq112_e1493_d_b26, eq112_e1493_d_b27, eq112_e1493_d_b28, eq112_e1493_d_b29, eq112_e1493_d_b30, eq112_e1493_d_b31, eq112_e1493_d_b32, eq112_e1493_d_b33, eq112_e1493_d_b34, eq112_e1493_d_b35, eq112_e1493_d_b36, eq112_e1493_d_b37, eq112_e1493_d_b38, eq112_e1493_d_b39, eq112_e1493_d_b40, eq112_e1493_d_b41, eq112_e1493_d_b42, eq112_e1493_d_b43, eq112_e1493_d_b44, eq112_e1493_d_b45, eq112_e1493_d_b46, eq112_e1493_d_b47, eq112_e1493_d_b48, eq112_e1493_d_b49, eq112_e1493_d_b50, eq112_e1493_d_b51, eq112_e1493_d_b52, eq112_e1493_d_b53, eq112_e1493_d_b54];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq112_value),
            &eq112_node_derivatives,
            &eq112_branch_derivatives,
            multiplicity,
        );
        let (eq113_e1501, eq113_e1501_d_n0, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n3, eq113_e1501_d_n4, eq113_e1501_d_n5, eq113_e1501_d_n6, eq113_e1501_d_n7, eq113_e1501_d_n8, eq113_e1501_d_n9, eq113_e1501_d_n10, eq113_e1501_d_n11, eq113_e1501_d_n12, eq113_e1501_d_n13, eq113_e1501_d_n14, eq113_e1501_d_n15, eq113_e1501_d_n16, eq113_e1501_d_n17, eq113_e1501_d_n18, eq113_e1501_d_n19, eq113_e1501_d_n20, eq113_e1501_d_n21, eq113_e1501_d_n22, eq113_e1501_d_b0, eq113_e1501_d_b1, eq113_e1501_d_b2, eq113_e1501_d_b3, eq113_e1501_d_b4, eq113_e1501_d_b5, eq113_e1501_d_b6, eq113_e1501_d_b7, eq113_e1501_d_b8, eq113_e1501_d_b9, eq113_e1501_d_b10, eq113_e1501_d_b11, eq113_e1501_d_b12, eq113_e1501_d_b13, eq113_e1501_d_b14, eq113_e1501_d_b15, eq113_e1501_d_b16, eq113_e1501_d_b17, eq113_e1501_d_b18, eq113_e1501_d_b19, eq113_e1501_d_b20, eq113_e1501_d_b21, eq113_e1501_d_b22, eq113_e1501_d_b23, eq113_e1501_d_b24, eq113_e1501_d_b25, eq113_e1501_d_b26, eq113_e1501_d_b27, eq113_e1501_d_b28, eq113_e1501_d_b29, eq113_e1501_d_b30, eq113_e1501_d_b31, eq113_e1501_d_b32, eq113_e1501_d_b33, eq113_e1501_d_b34, eq113_e1501_d_b35, eq113_e1501_d_b36, eq113_e1501_d_b37, eq113_e1501_d_b38, eq113_e1501_d_b39, eq113_e1501_d_b40, eq113_e1501_d_b41, eq113_e1501_d_b42, eq113_e1501_d_b43, eq113_e1501_d_b44, eq113_e1501_d_b45, eq113_e1501_d_b46, eq113_e1501_d_b47, eq113_e1501_d_b48, eq113_e1501_d_b49, eq113_e1501_d_b50, eq113_e1501_d_b51, eq113_e1501_d_b52, eq113_e1501_d_b53, eq113_e1501_d_b54,) = {
    if (var_guard535 == 0.0) {
        let eq113_e1498: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, var_qsov);
        let eq113_e1499: f64 = (p.p7 * eq113_e1498);
        let eq113_e1499_d_n0: f64 = (p.p7 * (var_qsov_dn0 * ddt_scale));
        let eq113_e1499_d_n1: f64 = (p.p7 * (var_qsov_dn1 * ddt_scale));
        let eq113_e1499_d_n2: f64 = (p.p7 * (var_qsov_dn2 * ddt_scale));
        let eq113_e1499_d_n3: f64 = (p.p7 * (var_qsov_dn3 * ddt_scale));
        let eq113_e1499_d_n4: f64 = (p.p7 * (var_qsov_dn4 * ddt_scale));
        let eq113_e1499_d_n5: f64 = (p.p7 * (var_qsov_dn5 * ddt_scale));
        let eq113_e1499_d_n6: f64 = (p.p7 * (var_qsov_dn6 * ddt_scale));
        let eq113_e1499_d_n7: f64 = (p.p7 * (var_qsov_dn7 * ddt_scale));
        let eq113_e1499_d_n8: f64 = (p.p7 * (var_qsov_dn8 * ddt_scale));
        let eq113_e1499_d_n9: f64 = (p.p7 * (var_qsov_dn9 * ddt_scale));
        let eq113_e1499_d_n10: f64 = (p.p7 * (var_qsov_dn10 * ddt_scale));
        let eq113_e1499_d_n11: f64 = (p.p7 * (var_qsov_dn11 * ddt_scale));
        let eq113_e1499_d_n12: f64 = (p.p7 * (var_qsov_dn12 * ddt_scale));
        let eq113_e1499_d_n13: f64 = (p.p7 * (var_qsov_dn13 * ddt_scale));
        let eq113_e1499_d_n14: f64 = (p.p7 * (var_qsov_dn14 * ddt_scale));
        let eq113_e1499_d_n15: f64 = (p.p7 * (var_qsov_dn15 * ddt_scale));
        let eq113_e1499_d_n16: f64 = (p.p7 * (var_qsov_dn16 * ddt_scale));
        let eq113_e1499_d_n17: f64 = (p.p7 * (var_qsov_dn17 * ddt_scale));
        let eq113_e1499_d_n18: f64 = (p.p7 * (var_qsov_dn18 * ddt_scale));
        let eq113_e1499_d_n19: f64 = (p.p7 * (var_qsov_dn19 * ddt_scale));
        let eq113_e1499_d_n20: f64 = (p.p7 * (var_qsov_dn20 * ddt_scale));
        let eq113_e1499_d_n21: f64 = (p.p7 * (var_qsov_dn21 * ddt_scale));
        let eq113_e1499_d_n22: f64 = (p.p7 * (var_qsov_dn22 * ddt_scale));
        let eq113_e1499_d_b0: f64 = (p.p7 * (var_qsov_db0 * ddt_scale));
        let eq113_e1499_d_b1: f64 = (p.p7 * (var_qsov_db1 * ddt_scale));
        let eq113_e1499_d_b2: f64 = (p.p7 * (var_qsov_db2 * ddt_scale));
        let eq113_e1499_d_b3: f64 = (p.p7 * (var_qsov_db3 * ddt_scale));
        let eq113_e1499_d_b4: f64 = (p.p7 * (var_qsov_db4 * ddt_scale));
        let eq113_e1499_d_b5: f64 = (p.p7 * (var_qsov_db5 * ddt_scale));
        let eq113_e1499_d_b6: f64 = (p.p7 * (var_qsov_db6 * ddt_scale));
        let eq113_e1499_d_b7: f64 = (p.p7 * (var_qsov_db7 * ddt_scale));
        let eq113_e1499_d_b8: f64 = (p.p7 * (var_qsov_db8 * ddt_scale));
        let eq113_e1499_d_b9: f64 = (p.p7 * (var_qsov_db9 * ddt_scale));
        let eq113_e1499_d_b10: f64 = (p.p7 * (var_qsov_db10 * ddt_scale));
        let eq113_e1499_d_b11: f64 = (p.p7 * (var_qsov_db11 * ddt_scale));
        let eq113_e1499_d_b12: f64 = (p.p7 * (var_qsov_db12 * ddt_scale));
        let eq113_e1499_d_b13: f64 = (p.p7 * (var_qsov_db13 * ddt_scale));
        let eq113_e1499_d_b14: f64 = (p.p7 * (var_qsov_db14 * ddt_scale));
        let eq113_e1499_d_b15: f64 = (p.p7 * (var_qsov_db15 * ddt_scale));
        let eq113_e1499_d_b16: f64 = (p.p7 * (var_qsov_db16 * ddt_scale));
        let eq113_e1499_d_b17: f64 = (p.p7 * (var_qsov_db17 * ddt_scale));
        let eq113_e1499_d_b18: f64 = (p.p7 * (var_qsov_db18 * ddt_scale));
        let eq113_e1499_d_b19: f64 = (p.p7 * (var_qsov_db19 * ddt_scale));
        let eq113_e1499_d_b20: f64 = (p.p7 * (var_qsov_db20 * ddt_scale));
        let eq113_e1499_d_b21: f64 = (p.p7 * (var_qsov_db21 * ddt_scale));
        let eq113_e1499_d_b22: f64 = (p.p7 * (var_qsov_db22 * ddt_scale));
        let eq113_e1499_d_b23: f64 = (p.p7 * (var_qsov_db23 * ddt_scale));
        let eq113_e1499_d_b24: f64 = (p.p7 * (var_qsov_db24 * ddt_scale));
        let eq113_e1499_d_b25: f64 = (p.p7 * (var_qsov_db25 * ddt_scale));
        let eq113_e1499_d_b26: f64 = (p.p7 * (var_qsov_db26 * ddt_scale));
        let eq113_e1499_d_b27: f64 = (p.p7 * (var_qsov_db27 * ddt_scale));
        let eq113_e1499_d_b28: f64 = (p.p7 * (var_qsov_db28 * ddt_scale));
        let eq113_e1499_d_b29: f64 = (p.p7 * (var_qsov_db29 * ddt_scale));
        let eq113_e1499_d_b30: f64 = (p.p7 * (var_qsov_db30 * ddt_scale));
        let eq113_e1499_d_b31: f64 = (p.p7 * (var_qsov_db31 * ddt_scale));
        let eq113_e1499_d_b32: f64 = (p.p7 * (var_qsov_db32 * ddt_scale));
        let eq113_e1499_d_b33: f64 = (p.p7 * (var_qsov_db33 * ddt_scale));
        let eq113_e1499_d_b34: f64 = (p.p7 * (var_qsov_db34 * ddt_scale));
        let eq113_e1499_d_b35: f64 = (p.p7 * (var_qsov_db35 * ddt_scale));
        let eq113_e1499_d_b36: f64 = (p.p7 * (var_qsov_db36 * ddt_scale));
        let eq113_e1499_d_b37: f64 = (p.p7 * (var_qsov_db37 * ddt_scale));
        let eq113_e1499_d_b38: f64 = (p.p7 * (var_qsov_db38 * ddt_scale));
        let eq113_e1499_d_b39: f64 = (p.p7 * (var_qsov_db39 * ddt_scale));
        let eq113_e1499_d_b40: f64 = (p.p7 * (var_qsov_db40 * ddt_scale));
        let eq113_e1499_d_b41: f64 = (p.p7 * (var_qsov_db41 * ddt_scale));
        let eq113_e1499_d_b42: f64 = (p.p7 * (var_qsov_db42 * ddt_scale));
        let eq113_e1499_d_b43: f64 = (p.p7 * (var_qsov_db43 * ddt_scale));
        let eq113_e1499_d_b44: f64 = (p.p7 * (var_qsov_db44 * ddt_scale));
        let eq113_e1499_d_b45: f64 = (p.p7 * (var_qsov_db45 * ddt_scale));
        let eq113_e1499_d_b46: f64 = (p.p7 * (var_qsov_db46 * ddt_scale));
        let eq113_e1499_d_b47: f64 = (p.p7 * (var_qsov_db47 * ddt_scale));
        let eq113_e1499_d_b48: f64 = (p.p7 * (var_qsov_db48 * ddt_scale));
        let eq113_e1499_d_b49: f64 = (p.p7 * (var_qsov_db49 * ddt_scale));
        let eq113_e1499_d_b50: f64 = (p.p7 * (var_qsov_db50 * ddt_scale));
        let eq113_e1499_d_b51: f64 = (p.p7 * (var_qsov_db51 * ddt_scale));
        let eq113_e1499_d_b52: f64 = (p.p7 * (var_qsov_db52 * ddt_scale));
        let eq113_e1499_d_b53: f64 = (p.p7 * (var_qsov_db53 * ddt_scale));
        let eq113_e1499_d_b54: f64 = (p.p7 * (var_qsov_db54 * ddt_scale));
        (eq113_e1499, eq113_e1499_d_n0, eq113_e1499_d_n1, eq113_e1499_d_n2, eq113_e1499_d_n3, eq113_e1499_d_n4, eq113_e1499_d_n5, eq113_e1499_d_n6, eq113_e1499_d_n7, eq113_e1499_d_n8, eq113_e1499_d_n9, eq113_e1499_d_n10, eq113_e1499_d_n11, eq113_e1499_d_n12, eq113_e1499_d_n13, eq113_e1499_d_n14, eq113_e1499_d_n15, eq113_e1499_d_n16, eq113_e1499_d_n17, eq113_e1499_d_n18, eq113_e1499_d_n19, eq113_e1499_d_n20, eq113_e1499_d_n21, eq113_e1499_d_n22, eq113_e1499_d_b0, eq113_e1499_d_b1, eq113_e1499_d_b2, eq113_e1499_d_b3, eq113_e1499_d_b4, eq113_e1499_d_b5, eq113_e1499_d_b6, eq113_e1499_d_b7, eq113_e1499_d_b8, eq113_e1499_d_b9, eq113_e1499_d_b10, eq113_e1499_d_b11, eq113_e1499_d_b12, eq113_e1499_d_b13, eq113_e1499_d_b14, eq113_e1499_d_b15, eq113_e1499_d_b16, eq113_e1499_d_b17, eq113_e1499_d_b18, eq113_e1499_d_b19, eq113_e1499_d_b20, eq113_e1499_d_b21, eq113_e1499_d_b22, eq113_e1499_d_b23, eq113_e1499_d_b24, eq113_e1499_d_b25, eq113_e1499_d_b26, eq113_e1499_d_b27, eq113_e1499_d_b28, eq113_e1499_d_b29, eq113_e1499_d_b30, eq113_e1499_d_b31, eq113_e1499_d_b32, eq113_e1499_d_b33, eq113_e1499_d_b34, eq113_e1499_d_b35, eq113_e1499_d_b36, eq113_e1499_d_b37, eq113_e1499_d_b38, eq113_e1499_d_b39, eq113_e1499_d_b40, eq113_e1499_d_b41, eq113_e1499_d_b42, eq113_e1499_d_b43, eq113_e1499_d_b44, eq113_e1499_d_b45, eq113_e1499_d_b46, eq113_e1499_d_b47, eq113_e1499_d_b48, eq113_e1499_d_b49, eq113_e1499_d_b50, eq113_e1499_d_b51, eq113_e1499_d_b52, eq113_e1499_d_b53, eq113_e1499_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_value: f64 = eq113_e1501;
        let eq113_node_derivatives: [f64; 23] = [eq113_e1501_d_n0, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n3, eq113_e1501_d_n4, eq113_e1501_d_n5, eq113_e1501_d_n6, eq113_e1501_d_n7, eq113_e1501_d_n8, eq113_e1501_d_n9, eq113_e1501_d_n10, eq113_e1501_d_n11, eq113_e1501_d_n12, eq113_e1501_d_n13, eq113_e1501_d_n14, eq113_e1501_d_n15, eq113_e1501_d_n16, eq113_e1501_d_n17, eq113_e1501_d_n18, eq113_e1501_d_n19, eq113_e1501_d_n20, eq113_e1501_d_n21, eq113_e1501_d_n22];
        let eq113_branch_derivatives: [f64; 55] = [eq113_e1501_d_b0, eq113_e1501_d_b1, eq113_e1501_d_b2, eq113_e1501_d_b3, eq113_e1501_d_b4, eq113_e1501_d_b5, eq113_e1501_d_b6, eq113_e1501_d_b7, eq113_e1501_d_b8, eq113_e1501_d_b9, eq113_e1501_d_b10, eq113_e1501_d_b11, eq113_e1501_d_b12, eq113_e1501_d_b13, eq113_e1501_d_b14, eq113_e1501_d_b15, eq113_e1501_d_b16, eq113_e1501_d_b17, eq113_e1501_d_b18, eq113_e1501_d_b19, eq113_e1501_d_b20, eq113_e1501_d_b21, eq113_e1501_d_b22, eq113_e1501_d_b23, eq113_e1501_d_b24, eq113_e1501_d_b25, eq113_e1501_d_b26, eq113_e1501_d_b27, eq113_e1501_d_b28, eq113_e1501_d_b29, eq113_e1501_d_b30, eq113_e1501_d_b31, eq113_e1501_d_b32, eq113_e1501_d_b33, eq113_e1501_d_b34, eq113_e1501_d_b35, eq113_e1501_d_b36, eq113_e1501_d_b37, eq113_e1501_d_b38, eq113_e1501_d_b39, eq113_e1501_d_b40, eq113_e1501_d_b41, eq113_e1501_d_b42, eq113_e1501_d_b43, eq113_e1501_d_b44, eq113_e1501_d_b45, eq113_e1501_d_b46, eq113_e1501_d_b47, eq113_e1501_d_b48, eq113_e1501_d_b49, eq113_e1501_d_b50, eq113_e1501_d_b51, eq113_e1501_d_b52, eq113_e1501_d_b53, eq113_e1501_d_b54];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq113_value),
            &eq113_node_derivatives,
            &eq113_branch_derivatives,
            multiplicity,
        );
        let (eq114_e1509, eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n3, eq114_e1509_d_n4, eq114_e1509_d_n5, eq114_e1509_d_n6, eq114_e1509_d_n7, eq114_e1509_d_n8, eq114_e1509_d_n9, eq114_e1509_d_n10, eq114_e1509_d_n11, eq114_e1509_d_n12, eq114_e1509_d_n13, eq114_e1509_d_n14, eq114_e1509_d_n15, eq114_e1509_d_n16, eq114_e1509_d_n17, eq114_e1509_d_n18, eq114_e1509_d_n19, eq114_e1509_d_n20, eq114_e1509_d_n21, eq114_e1509_d_n22, eq114_e1509_d_b0, eq114_e1509_d_b1, eq114_e1509_d_b2, eq114_e1509_d_b3, eq114_e1509_d_b4, eq114_e1509_d_b5, eq114_e1509_d_b6, eq114_e1509_d_b7, eq114_e1509_d_b8, eq114_e1509_d_b9, eq114_e1509_d_b10, eq114_e1509_d_b11, eq114_e1509_d_b12, eq114_e1509_d_b13, eq114_e1509_d_b14, eq114_e1509_d_b15, eq114_e1509_d_b16, eq114_e1509_d_b17, eq114_e1509_d_b18, eq114_e1509_d_b19, eq114_e1509_d_b20, eq114_e1509_d_b21, eq114_e1509_d_b22, eq114_e1509_d_b23, eq114_e1509_d_b24, eq114_e1509_d_b25, eq114_e1509_d_b26, eq114_e1509_d_b27, eq114_e1509_d_b28, eq114_e1509_d_b29, eq114_e1509_d_b30, eq114_e1509_d_b31, eq114_e1509_d_b32, eq114_e1509_d_b33, eq114_e1509_d_b34, eq114_e1509_d_b35, eq114_e1509_d_b36, eq114_e1509_d_b37, eq114_e1509_d_b38, eq114_e1509_d_b39, eq114_e1509_d_b40, eq114_e1509_d_b41, eq114_e1509_d_b42, eq114_e1509_d_b43, eq114_e1509_d_b44, eq114_e1509_d_b45, eq114_e1509_d_b46, eq114_e1509_d_b47, eq114_e1509_d_b48, eq114_e1509_d_b49, eq114_e1509_d_b50, eq114_e1509_d_b51, eq114_e1509_d_b52, eq114_e1509_d_b53, eq114_e1509_d_b54,) = {
    if (var_guard535 == 0.0) {
        let eq114_e1506: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, var_qdov);
        let eq114_e1507: f64 = (p.p7 * eq114_e1506);
        let eq114_e1507_d_n0: f64 = (p.p7 * (var_qdov_dn0 * ddt_scale));
        let eq114_e1507_d_n1: f64 = (p.p7 * (var_qdov_dn1 * ddt_scale));
        let eq114_e1507_d_n2: f64 = (p.p7 * (var_qdov_dn2 * ddt_scale));
        let eq114_e1507_d_n3: f64 = (p.p7 * (var_qdov_dn3 * ddt_scale));
        let eq114_e1507_d_n4: f64 = (p.p7 * (var_qdov_dn4 * ddt_scale));
        let eq114_e1507_d_n5: f64 = (p.p7 * (var_qdov_dn5 * ddt_scale));
        let eq114_e1507_d_n6: f64 = (p.p7 * (var_qdov_dn6 * ddt_scale));
        let eq114_e1507_d_n7: f64 = (p.p7 * (var_qdov_dn7 * ddt_scale));
        let eq114_e1507_d_n8: f64 = (p.p7 * (var_qdov_dn8 * ddt_scale));
        let eq114_e1507_d_n9: f64 = (p.p7 * (var_qdov_dn9 * ddt_scale));
        let eq114_e1507_d_n10: f64 = (p.p7 * (var_qdov_dn10 * ddt_scale));
        let eq114_e1507_d_n11: f64 = (p.p7 * (var_qdov_dn11 * ddt_scale));
        let eq114_e1507_d_n12: f64 = (p.p7 * (var_qdov_dn12 * ddt_scale));
        let eq114_e1507_d_n13: f64 = (p.p7 * (var_qdov_dn13 * ddt_scale));
        let eq114_e1507_d_n14: f64 = (p.p7 * (var_qdov_dn14 * ddt_scale));
        let eq114_e1507_d_n15: f64 = (p.p7 * (var_qdov_dn15 * ddt_scale));
        let eq114_e1507_d_n16: f64 = (p.p7 * (var_qdov_dn16 * ddt_scale));
        let eq114_e1507_d_n17: f64 = (p.p7 * (var_qdov_dn17 * ddt_scale));
        let eq114_e1507_d_n18: f64 = (p.p7 * (var_qdov_dn18 * ddt_scale));
        let eq114_e1507_d_n19: f64 = (p.p7 * (var_qdov_dn19 * ddt_scale));
        let eq114_e1507_d_n20: f64 = (p.p7 * (var_qdov_dn20 * ddt_scale));
        let eq114_e1507_d_n21: f64 = (p.p7 * (var_qdov_dn21 * ddt_scale));
        let eq114_e1507_d_n22: f64 = (p.p7 * (var_qdov_dn22 * ddt_scale));
        let eq114_e1507_d_b0: f64 = (p.p7 * (var_qdov_db0 * ddt_scale));
        let eq114_e1507_d_b1: f64 = (p.p7 * (var_qdov_db1 * ddt_scale));
        let eq114_e1507_d_b2: f64 = (p.p7 * (var_qdov_db2 * ddt_scale));
        let eq114_e1507_d_b3: f64 = (p.p7 * (var_qdov_db3 * ddt_scale));
        let eq114_e1507_d_b4: f64 = (p.p7 * (var_qdov_db4 * ddt_scale));
        let eq114_e1507_d_b5: f64 = (p.p7 * (var_qdov_db5 * ddt_scale));
        let eq114_e1507_d_b6: f64 = (p.p7 * (var_qdov_db6 * ddt_scale));
        let eq114_e1507_d_b7: f64 = (p.p7 * (var_qdov_db7 * ddt_scale));
        let eq114_e1507_d_b8: f64 = (p.p7 * (var_qdov_db8 * ddt_scale));
        let eq114_e1507_d_b9: f64 = (p.p7 * (var_qdov_db9 * ddt_scale));
        let eq114_e1507_d_b10: f64 = (p.p7 * (var_qdov_db10 * ddt_scale));
        let eq114_e1507_d_b11: f64 = (p.p7 * (var_qdov_db11 * ddt_scale));
        let eq114_e1507_d_b12: f64 = (p.p7 * (var_qdov_db12 * ddt_scale));
        let eq114_e1507_d_b13: f64 = (p.p7 * (var_qdov_db13 * ddt_scale));
        let eq114_e1507_d_b14: f64 = (p.p7 * (var_qdov_db14 * ddt_scale));
        let eq114_e1507_d_b15: f64 = (p.p7 * (var_qdov_db15 * ddt_scale));
        let eq114_e1507_d_b16: f64 = (p.p7 * (var_qdov_db16 * ddt_scale));
        let eq114_e1507_d_b17: f64 = (p.p7 * (var_qdov_db17 * ddt_scale));
        let eq114_e1507_d_b18: f64 = (p.p7 * (var_qdov_db18 * ddt_scale));
        let eq114_e1507_d_b19: f64 = (p.p7 * (var_qdov_db19 * ddt_scale));
        let eq114_e1507_d_b20: f64 = (p.p7 * (var_qdov_db20 * ddt_scale));
        let eq114_e1507_d_b21: f64 = (p.p7 * (var_qdov_db21 * ddt_scale));
        let eq114_e1507_d_b22: f64 = (p.p7 * (var_qdov_db22 * ddt_scale));
        let eq114_e1507_d_b23: f64 = (p.p7 * (var_qdov_db23 * ddt_scale));
        let eq114_e1507_d_b24: f64 = (p.p7 * (var_qdov_db24 * ddt_scale));
        let eq114_e1507_d_b25: f64 = (p.p7 * (var_qdov_db25 * ddt_scale));
        let eq114_e1507_d_b26: f64 = (p.p7 * (var_qdov_db26 * ddt_scale));
        let eq114_e1507_d_b27: f64 = (p.p7 * (var_qdov_db27 * ddt_scale));
        let eq114_e1507_d_b28: f64 = (p.p7 * (var_qdov_db28 * ddt_scale));
        let eq114_e1507_d_b29: f64 = (p.p7 * (var_qdov_db29 * ddt_scale));
        let eq114_e1507_d_b30: f64 = (p.p7 * (var_qdov_db30 * ddt_scale));
        let eq114_e1507_d_b31: f64 = (p.p7 * (var_qdov_db31 * ddt_scale));
        let eq114_e1507_d_b32: f64 = (p.p7 * (var_qdov_db32 * ddt_scale));
        let eq114_e1507_d_b33: f64 = (p.p7 * (var_qdov_db33 * ddt_scale));
        let eq114_e1507_d_b34: f64 = (p.p7 * (var_qdov_db34 * ddt_scale));
        let eq114_e1507_d_b35: f64 = (p.p7 * (var_qdov_db35 * ddt_scale));
        let eq114_e1507_d_b36: f64 = (p.p7 * (var_qdov_db36 * ddt_scale));
        let eq114_e1507_d_b37: f64 = (p.p7 * (var_qdov_db37 * ddt_scale));
        let eq114_e1507_d_b38: f64 = (p.p7 * (var_qdov_db38 * ddt_scale));
        let eq114_e1507_d_b39: f64 = (p.p7 * (var_qdov_db39 * ddt_scale));
        let eq114_e1507_d_b40: f64 = (p.p7 * (var_qdov_db40 * ddt_scale));
        let eq114_e1507_d_b41: f64 = (p.p7 * (var_qdov_db41 * ddt_scale));
        let eq114_e1507_d_b42: f64 = (p.p7 * (var_qdov_db42 * ddt_scale));
        let eq114_e1507_d_b43: f64 = (p.p7 * (var_qdov_db43 * ddt_scale));
        let eq114_e1507_d_b44: f64 = (p.p7 * (var_qdov_db44 * ddt_scale));
        let eq114_e1507_d_b45: f64 = (p.p7 * (var_qdov_db45 * ddt_scale));
        let eq114_e1507_d_b46: f64 = (p.p7 * (var_qdov_db46 * ddt_scale));
        let eq114_e1507_d_b47: f64 = (p.p7 * (var_qdov_db47 * ddt_scale));
        let eq114_e1507_d_b48: f64 = (p.p7 * (var_qdov_db48 * ddt_scale));
        let eq114_e1507_d_b49: f64 = (p.p7 * (var_qdov_db49 * ddt_scale));
        let eq114_e1507_d_b50: f64 = (p.p7 * (var_qdov_db50 * ddt_scale));
        let eq114_e1507_d_b51: f64 = (p.p7 * (var_qdov_db51 * ddt_scale));
        let eq114_e1507_d_b52: f64 = (p.p7 * (var_qdov_db52 * ddt_scale));
        let eq114_e1507_d_b53: f64 = (p.p7 * (var_qdov_db53 * ddt_scale));
        let eq114_e1507_d_b54: f64 = (p.p7 * (var_qdov_db54 * ddt_scale));
        (eq114_e1507, eq114_e1507_d_n0, eq114_e1507_d_n1, eq114_e1507_d_n2, eq114_e1507_d_n3, eq114_e1507_d_n4, eq114_e1507_d_n5, eq114_e1507_d_n6, eq114_e1507_d_n7, eq114_e1507_d_n8, eq114_e1507_d_n9, eq114_e1507_d_n10, eq114_e1507_d_n11, eq114_e1507_d_n12, eq114_e1507_d_n13, eq114_e1507_d_n14, eq114_e1507_d_n15, eq114_e1507_d_n16, eq114_e1507_d_n17, eq114_e1507_d_n18, eq114_e1507_d_n19, eq114_e1507_d_n20, eq114_e1507_d_n21, eq114_e1507_d_n22, eq114_e1507_d_b0, eq114_e1507_d_b1, eq114_e1507_d_b2, eq114_e1507_d_b3, eq114_e1507_d_b4, eq114_e1507_d_b5, eq114_e1507_d_b6, eq114_e1507_d_b7, eq114_e1507_d_b8, eq114_e1507_d_b9, eq114_e1507_d_b10, eq114_e1507_d_b11, eq114_e1507_d_b12, eq114_e1507_d_b13, eq114_e1507_d_b14, eq114_e1507_d_b15, eq114_e1507_d_b16, eq114_e1507_d_b17, eq114_e1507_d_b18, eq114_e1507_d_b19, eq114_e1507_d_b20, eq114_e1507_d_b21, eq114_e1507_d_b22, eq114_e1507_d_b23, eq114_e1507_d_b24, eq114_e1507_d_b25, eq114_e1507_d_b26, eq114_e1507_d_b27, eq114_e1507_d_b28, eq114_e1507_d_b29, eq114_e1507_d_b30, eq114_e1507_d_b31, eq114_e1507_d_b32, eq114_e1507_d_b33, eq114_e1507_d_b34, eq114_e1507_d_b35, eq114_e1507_d_b36, eq114_e1507_d_b37, eq114_e1507_d_b38, eq114_e1507_d_b39, eq114_e1507_d_b40, eq114_e1507_d_b41, eq114_e1507_d_b42, eq114_e1507_d_b43, eq114_e1507_d_b44, eq114_e1507_d_b45, eq114_e1507_d_b46, eq114_e1507_d_b47, eq114_e1507_d_b48, eq114_e1507_d_b49, eq114_e1507_d_b50, eq114_e1507_d_b51, eq114_e1507_d_b52, eq114_e1507_d_b53, eq114_e1507_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq114_value: f64 = eq114_e1509;
        let eq114_node_derivatives: [f64; 23] = [eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n3, eq114_e1509_d_n4, eq114_e1509_d_n5, eq114_e1509_d_n6, eq114_e1509_d_n7, eq114_e1509_d_n8, eq114_e1509_d_n9, eq114_e1509_d_n10, eq114_e1509_d_n11, eq114_e1509_d_n12, eq114_e1509_d_n13, eq114_e1509_d_n14, eq114_e1509_d_n15, eq114_e1509_d_n16, eq114_e1509_d_n17, eq114_e1509_d_n18, eq114_e1509_d_n19, eq114_e1509_d_n20, eq114_e1509_d_n21, eq114_e1509_d_n22];
        let eq114_branch_derivatives: [f64; 55] = [eq114_e1509_d_b0, eq114_e1509_d_b1, eq114_e1509_d_b2, eq114_e1509_d_b3, eq114_e1509_d_b4, eq114_e1509_d_b5, eq114_e1509_d_b6, eq114_e1509_d_b7, eq114_e1509_d_b8, eq114_e1509_d_b9, eq114_e1509_d_b10, eq114_e1509_d_b11, eq114_e1509_d_b12, eq114_e1509_d_b13, eq114_e1509_d_b14, eq114_e1509_d_b15, eq114_e1509_d_b16, eq114_e1509_d_b17, eq114_e1509_d_b18, eq114_e1509_d_b19, eq114_e1509_d_b20, eq114_e1509_d_b21, eq114_e1509_d_b22, eq114_e1509_d_b23, eq114_e1509_d_b24, eq114_e1509_d_b25, eq114_e1509_d_b26, eq114_e1509_d_b27, eq114_e1509_d_b28, eq114_e1509_d_b29, eq114_e1509_d_b30, eq114_e1509_d_b31, eq114_e1509_d_b32, eq114_e1509_d_b33, eq114_e1509_d_b34, eq114_e1509_d_b35, eq114_e1509_d_b36, eq114_e1509_d_b37, eq114_e1509_d_b38, eq114_e1509_d_b39, eq114_e1509_d_b40, eq114_e1509_d_b41, eq114_e1509_d_b42, eq114_e1509_d_b43, eq114_e1509_d_b44, eq114_e1509_d_b45, eq114_e1509_d_b46, eq114_e1509_d_b47, eq114_e1509_d_b48, eq114_e1509_d_b49, eq114_e1509_d_b50, eq114_e1509_d_b51, eq114_e1509_d_b52, eq114_e1509_d_b53, eq114_e1509_d_b54];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq114_value),
            &eq114_node_derivatives,
            &eq114_branch_derivatives,
            multiplicity,
        );
        let eq115_e1512: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, var_qdsov);
        let eq115_e1513: f64 = (p.p7 * eq115_e1512);
        let eq115_e1513_d_n0: f64 = (p.p7 * (var_qdsov_dn0 * ddt_scale));
        let eq115_e1513_d_n1: f64 = (p.p7 * (var_qdsov_dn1 * ddt_scale));
        let eq115_e1513_d_n2: f64 = (p.p7 * (var_qdsov_dn2 * ddt_scale));
        let eq115_e1513_d_n3: f64 = (p.p7 * (var_qdsov_dn3 * ddt_scale));
        let eq115_e1513_d_n4: f64 = (p.p7 * (var_qdsov_dn4 * ddt_scale));
        let eq115_e1513_d_n5: f64 = (p.p7 * (var_qdsov_dn5 * ddt_scale));
        let eq115_e1513_d_n6: f64 = (p.p7 * (var_qdsov_dn6 * ddt_scale));
        let eq115_e1513_d_n7: f64 = (p.p7 * (var_qdsov_dn7 * ddt_scale));
        let eq115_e1513_d_n8: f64 = (p.p7 * (var_qdsov_dn8 * ddt_scale));
        let eq115_e1513_d_n9: f64 = (p.p7 * (var_qdsov_dn9 * ddt_scale));
        let eq115_e1513_d_n10: f64 = (p.p7 * (var_qdsov_dn10 * ddt_scale));
        let eq115_e1513_d_n11: f64 = (p.p7 * (var_qdsov_dn11 * ddt_scale));
        let eq115_e1513_d_n12: f64 = (p.p7 * (var_qdsov_dn12 * ddt_scale));
        let eq115_e1513_d_n13: f64 = (p.p7 * (var_qdsov_dn13 * ddt_scale));
        let eq115_e1513_d_n14: f64 = (p.p7 * (var_qdsov_dn14 * ddt_scale));
        let eq115_e1513_d_n15: f64 = (p.p7 * (var_qdsov_dn15 * ddt_scale));
        let eq115_e1513_d_n16: f64 = (p.p7 * (var_qdsov_dn16 * ddt_scale));
        let eq115_e1513_d_n17: f64 = (p.p7 * (var_qdsov_dn17 * ddt_scale));
        let eq115_e1513_d_n18: f64 = (p.p7 * (var_qdsov_dn18 * ddt_scale));
        let eq115_e1513_d_n19: f64 = (p.p7 * (var_qdsov_dn19 * ddt_scale));
        let eq115_e1513_d_n20: f64 = (p.p7 * (var_qdsov_dn20 * ddt_scale));
        let eq115_e1513_d_n21: f64 = (p.p7 * (var_qdsov_dn21 * ddt_scale));
        let eq115_e1513_d_n22: f64 = (p.p7 * (var_qdsov_dn22 * ddt_scale));
        let eq115_e1513_d_b0: f64 = (p.p7 * (var_qdsov_db0 * ddt_scale));
        let eq115_e1513_d_b1: f64 = (p.p7 * (var_qdsov_db1 * ddt_scale));
        let eq115_e1513_d_b2: f64 = (p.p7 * (var_qdsov_db2 * ddt_scale));
        let eq115_e1513_d_b3: f64 = (p.p7 * (var_qdsov_db3 * ddt_scale));
        let eq115_e1513_d_b4: f64 = (p.p7 * (var_qdsov_db4 * ddt_scale));
        let eq115_e1513_d_b5: f64 = (p.p7 * (var_qdsov_db5 * ddt_scale));
        let eq115_e1513_d_b6: f64 = (p.p7 * (var_qdsov_db6 * ddt_scale));
        let eq115_e1513_d_b7: f64 = (p.p7 * (var_qdsov_db7 * ddt_scale));
        let eq115_e1513_d_b8: f64 = (p.p7 * (var_qdsov_db8 * ddt_scale));
        let eq115_e1513_d_b9: f64 = (p.p7 * (var_qdsov_db9 * ddt_scale));
        let eq115_e1513_d_b10: f64 = (p.p7 * (var_qdsov_db10 * ddt_scale));
        let eq115_e1513_d_b11: f64 = (p.p7 * (var_qdsov_db11 * ddt_scale));
        let eq115_e1513_d_b12: f64 = (p.p7 * (var_qdsov_db12 * ddt_scale));
        let eq115_e1513_d_b13: f64 = (p.p7 * (var_qdsov_db13 * ddt_scale));
        let eq115_e1513_d_b14: f64 = (p.p7 * (var_qdsov_db14 * ddt_scale));
        let eq115_e1513_d_b15: f64 = (p.p7 * (var_qdsov_db15 * ddt_scale));
        let eq115_e1513_d_b16: f64 = (p.p7 * (var_qdsov_db16 * ddt_scale));
        let eq115_e1513_d_b17: f64 = (p.p7 * (var_qdsov_db17 * ddt_scale));
        let eq115_e1513_d_b18: f64 = (p.p7 * (var_qdsov_db18 * ddt_scale));
        let eq115_e1513_d_b19: f64 = (p.p7 * (var_qdsov_db19 * ddt_scale));
        let eq115_e1513_d_b20: f64 = (p.p7 * (var_qdsov_db20 * ddt_scale));
        let eq115_e1513_d_b21: f64 = (p.p7 * (var_qdsov_db21 * ddt_scale));
        let eq115_e1513_d_b22: f64 = (p.p7 * (var_qdsov_db22 * ddt_scale));
        let eq115_e1513_d_b23: f64 = (p.p7 * (var_qdsov_db23 * ddt_scale));
        let eq115_e1513_d_b24: f64 = (p.p7 * (var_qdsov_db24 * ddt_scale));
        let eq115_e1513_d_b25: f64 = (p.p7 * (var_qdsov_db25 * ddt_scale));
        let eq115_e1513_d_b26: f64 = (p.p7 * (var_qdsov_db26 * ddt_scale));
        let eq115_e1513_d_b27: f64 = (p.p7 * (var_qdsov_db27 * ddt_scale));
        let eq115_e1513_d_b28: f64 = (p.p7 * (var_qdsov_db28 * ddt_scale));
        let eq115_e1513_d_b29: f64 = (p.p7 * (var_qdsov_db29 * ddt_scale));
        let eq115_e1513_d_b30: f64 = (p.p7 * (var_qdsov_db30 * ddt_scale));
        let eq115_e1513_d_b31: f64 = (p.p7 * (var_qdsov_db31 * ddt_scale));
        let eq115_e1513_d_b32: f64 = (p.p7 * (var_qdsov_db32 * ddt_scale));
        let eq115_e1513_d_b33: f64 = (p.p7 * (var_qdsov_db33 * ddt_scale));
        let eq115_e1513_d_b34: f64 = (p.p7 * (var_qdsov_db34 * ddt_scale));
        let eq115_e1513_d_b35: f64 = (p.p7 * (var_qdsov_db35 * ddt_scale));
        let eq115_e1513_d_b36: f64 = (p.p7 * (var_qdsov_db36 * ddt_scale));
        let eq115_e1513_d_b37: f64 = (p.p7 * (var_qdsov_db37 * ddt_scale));
        let eq115_e1513_d_b38: f64 = (p.p7 * (var_qdsov_db38 * ddt_scale));
        let eq115_e1513_d_b39: f64 = (p.p7 * (var_qdsov_db39 * ddt_scale));
        let eq115_e1513_d_b40: f64 = (p.p7 * (var_qdsov_db40 * ddt_scale));
        let eq115_e1513_d_b41: f64 = (p.p7 * (var_qdsov_db41 * ddt_scale));
        let eq115_e1513_d_b42: f64 = (p.p7 * (var_qdsov_db42 * ddt_scale));
        let eq115_e1513_d_b43: f64 = (p.p7 * (var_qdsov_db43 * ddt_scale));
        let eq115_e1513_d_b44: f64 = (p.p7 * (var_qdsov_db44 * ddt_scale));
        let eq115_e1513_d_b45: f64 = (p.p7 * (var_qdsov_db45 * ddt_scale));
        let eq115_e1513_d_b46: f64 = (p.p7 * (var_qdsov_db46 * ddt_scale));
        let eq115_e1513_d_b47: f64 = (p.p7 * (var_qdsov_db47 * ddt_scale));
        let eq115_e1513_d_b48: f64 = (p.p7 * (var_qdsov_db48 * ddt_scale));
        let eq115_e1513_d_b49: f64 = (p.p7 * (var_qdsov_db49 * ddt_scale));
        let eq115_e1513_d_b50: f64 = (p.p7 * (var_qdsov_db50 * ddt_scale));
        let eq115_e1513_d_b51: f64 = (p.p7 * (var_qdsov_db51 * ddt_scale));
        let eq115_e1513_d_b52: f64 = (p.p7 * (var_qdsov_db52 * ddt_scale));
        let eq115_e1513_d_b53: f64 = (p.p7 * (var_qdsov_db53 * ddt_scale));
        let eq115_e1513_d_b54: f64 = (p.p7 * (var_qdsov_db54 * ddt_scale));
        let eq115_value: f64 = eq115_e1513;
        let eq115_node_derivatives: [f64; 23] = [eq115_e1513_d_n0, eq115_e1513_d_n1, eq115_e1513_d_n2, eq115_e1513_d_n3, eq115_e1513_d_n4, eq115_e1513_d_n5, eq115_e1513_d_n6, eq115_e1513_d_n7, eq115_e1513_d_n8, eq115_e1513_d_n9, eq115_e1513_d_n10, eq115_e1513_d_n11, eq115_e1513_d_n12, eq115_e1513_d_n13, eq115_e1513_d_n14, eq115_e1513_d_n15, eq115_e1513_d_n16, eq115_e1513_d_n17, eq115_e1513_d_n18, eq115_e1513_d_n19, eq115_e1513_d_n20, eq115_e1513_d_n21, eq115_e1513_d_n22];
        let eq115_branch_derivatives: [f64; 55] = [eq115_e1513_d_b0, eq115_e1513_d_b1, eq115_e1513_d_b2, eq115_e1513_d_b3, eq115_e1513_d_b4, eq115_e1513_d_b5, eq115_e1513_d_b6, eq115_e1513_d_b7, eq115_e1513_d_b8, eq115_e1513_d_b9, eq115_e1513_d_b10, eq115_e1513_d_b11, eq115_e1513_d_b12, eq115_e1513_d_b13, eq115_e1513_d_b14, eq115_e1513_d_b15, eq115_e1513_d_b16, eq115_e1513_d_b17, eq115_e1513_d_b18, eq115_e1513_d_b19, eq115_e1513_d_b20, eq115_e1513_d_b21, eq115_e1513_d_b22, eq115_e1513_d_b23, eq115_e1513_d_b24, eq115_e1513_d_b25, eq115_e1513_d_b26, eq115_e1513_d_b27, eq115_e1513_d_b28, eq115_e1513_d_b29, eq115_e1513_d_b30, eq115_e1513_d_b31, eq115_e1513_d_b32, eq115_e1513_d_b33, eq115_e1513_d_b34, eq115_e1513_d_b35, eq115_e1513_d_b36, eq115_e1513_d_b37, eq115_e1513_d_b38, eq115_e1513_d_b39, eq115_e1513_d_b40, eq115_e1513_d_b41, eq115_e1513_d_b42, eq115_e1513_d_b43, eq115_e1513_d_b44, eq115_e1513_d_b45, eq115_e1513_d_b46, eq115_e1513_d_b47, eq115_e1513_d_b48, eq115_e1513_d_b49, eq115_e1513_d_b50, eq115_e1513_d_b51, eq115_e1513_d_b52, eq115_e1513_d_b53, eq115_e1513_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq115_value),
            &eq115_node_derivatives,
            &eq115_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_15(
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
        var_qbdov: f64,
        var_qbdov_db0: f64,
        var_qbdov_db1: f64,
        var_qbdov_db10: f64,
        var_qbdov_db11: f64,
        var_qbdov_db12: f64,
        var_qbdov_db13: f64,
        var_qbdov_db14: f64,
        var_qbdov_db15: f64,
        var_qbdov_db16: f64,
        var_qbdov_db17: f64,
        var_qbdov_db18: f64,
        var_qbdov_db19: f64,
        var_qbdov_db2: f64,
        var_qbdov_db20: f64,
        var_qbdov_db21: f64,
        var_qbdov_db22: f64,
        var_qbdov_db23: f64,
        var_qbdov_db24: f64,
        var_qbdov_db25: f64,
        var_qbdov_db26: f64,
        var_qbdov_db27: f64,
        var_qbdov_db28: f64,
        var_qbdov_db29: f64,
        var_qbdov_db3: f64,
        var_qbdov_db30: f64,
        var_qbdov_db31: f64,
        var_qbdov_db32: f64,
        var_qbdov_db33: f64,
        var_qbdov_db34: f64,
        var_qbdov_db35: f64,
        var_qbdov_db36: f64,
        var_qbdov_db37: f64,
        var_qbdov_db38: f64,
        var_qbdov_db39: f64,
        var_qbdov_db4: f64,
        var_qbdov_db40: f64,
        var_qbdov_db41: f64,
        var_qbdov_db42: f64,
        var_qbdov_db43: f64,
        var_qbdov_db44: f64,
        var_qbdov_db45: f64,
        var_qbdov_db46: f64,
        var_qbdov_db47: f64,
        var_qbdov_db48: f64,
        var_qbdov_db49: f64,
        var_qbdov_db5: f64,
        var_qbdov_db50: f64,
        var_qbdov_db51: f64,
        var_qbdov_db52: f64,
        var_qbdov_db53: f64,
        var_qbdov_db54: f64,
        var_qbdov_db6: f64,
        var_qbdov_db7: f64,
        var_qbdov_db8: f64,
        var_qbdov_db9: f64,
        var_qbdov_dn0: f64,
        var_qbdov_dn1: f64,
        var_qbdov_dn10: f64,
        var_qbdov_dn11: f64,
        var_qbdov_dn12: f64,
        var_qbdov_dn13: f64,
        var_qbdov_dn14: f64,
        var_qbdov_dn15: f64,
        var_qbdov_dn16: f64,
        var_qbdov_dn17: f64,
        var_qbdov_dn18: f64,
        var_qbdov_dn19: f64,
        var_qbdov_dn2: f64,
        var_qbdov_dn20: f64,
        var_qbdov_dn21: f64,
        var_qbdov_dn22: f64,
        var_qbdov_dn3: f64,
        var_qbdov_dn4: f64,
        var_qbdov_dn5: f64,
        var_qbdov_dn6: f64,
        var_qbdov_dn7: f64,
        var_qbdov_dn8: f64,
        var_qbdov_dn9: f64,
        var_qbgov: f64,
        var_qbgov_db0: f64,
        var_qbgov_db1: f64,
        var_qbgov_db10: f64,
        var_qbgov_db11: f64,
        var_qbgov_db12: f64,
        var_qbgov_db13: f64,
        var_qbgov_db14: f64,
        var_qbgov_db15: f64,
        var_qbgov_db16: f64,
        var_qbgov_db17: f64,
        var_qbgov_db18: f64,
        var_qbgov_db19: f64,
        var_qbgov_db2: f64,
        var_qbgov_db20: f64,
        var_qbgov_db21: f64,
        var_qbgov_db22: f64,
        var_qbgov_db23: f64,
        var_qbgov_db24: f64,
        var_qbgov_db25: f64,
        var_qbgov_db26: f64,
        var_qbgov_db27: f64,
        var_qbgov_db28: f64,
        var_qbgov_db29: f64,
        var_qbgov_db3: f64,
        var_qbgov_db30: f64,
        var_qbgov_db31: f64,
        var_qbgov_db32: f64,
        var_qbgov_db33: f64,
        var_qbgov_db34: f64,
        var_qbgov_db35: f64,
        var_qbgov_db36: f64,
        var_qbgov_db37: f64,
        var_qbgov_db38: f64,
        var_qbgov_db39: f64,
        var_qbgov_db4: f64,
        var_qbgov_db40: f64,
        var_qbgov_db41: f64,
        var_qbgov_db42: f64,
        var_qbgov_db43: f64,
        var_qbgov_db44: f64,
        var_qbgov_db45: f64,
        var_qbgov_db46: f64,
        var_qbgov_db47: f64,
        var_qbgov_db48: f64,
        var_qbgov_db49: f64,
        var_qbgov_db5: f64,
        var_qbgov_db50: f64,
        var_qbgov_db51: f64,
        var_qbgov_db52: f64,
        var_qbgov_db53: f64,
        var_qbgov_db54: f64,
        var_qbgov_db6: f64,
        var_qbgov_db7: f64,
        var_qbgov_db8: f64,
        var_qbgov_db9: f64,
        var_qbgov_dn0: f64,
        var_qbgov_dn1: f64,
        var_qbgov_dn10: f64,
        var_qbgov_dn11: f64,
        var_qbgov_dn12: f64,
        var_qbgov_dn13: f64,
        var_qbgov_dn14: f64,
        var_qbgov_dn15: f64,
        var_qbgov_dn16: f64,
        var_qbgov_dn17: f64,
        var_qbgov_dn18: f64,
        var_qbgov_dn19: f64,
        var_qbgov_dn2: f64,
        var_qbgov_dn20: f64,
        var_qbgov_dn21: f64,
        var_qbgov_dn22: f64,
        var_qbgov_dn3: f64,
        var_qbgov_dn4: f64,
        var_qbgov_dn5: f64,
        var_qbgov_dn6: f64,
        var_qbgov_dn7: f64,
        var_qbgov_dn8: f64,
        var_qbgov_dn9: f64,
        var_qbsov: f64,
        var_qbsov_db0: f64,
        var_qbsov_db1: f64,
        var_qbsov_db10: f64,
        var_qbsov_db11: f64,
        var_qbsov_db12: f64,
        var_qbsov_db13: f64,
        var_qbsov_db14: f64,
        var_qbsov_db15: f64,
        var_qbsov_db16: f64,
        var_qbsov_db17: f64,
        var_qbsov_db18: f64,
        var_qbsov_db19: f64,
        var_qbsov_db2: f64,
        var_qbsov_db20: f64,
        var_qbsov_db21: f64,
        var_qbsov_db22: f64,
        var_qbsov_db23: f64,
        var_qbsov_db24: f64,
        var_qbsov_db25: f64,
        var_qbsov_db26: f64,
        var_qbsov_db27: f64,
        var_qbsov_db28: f64,
        var_qbsov_db29: f64,
        var_qbsov_db3: f64,
        var_qbsov_db30: f64,
        var_qbsov_db31: f64,
        var_qbsov_db32: f64,
        var_qbsov_db33: f64,
        var_qbsov_db34: f64,
        var_qbsov_db35: f64,
        var_qbsov_db36: f64,
        var_qbsov_db37: f64,
        var_qbsov_db38: f64,
        var_qbsov_db39: f64,
        var_qbsov_db4: f64,
        var_qbsov_db40: f64,
        var_qbsov_db41: f64,
        var_qbsov_db42: f64,
        var_qbsov_db43: f64,
        var_qbsov_db44: f64,
        var_qbsov_db45: f64,
        var_qbsov_db46: f64,
        var_qbsov_db47: f64,
        var_qbsov_db48: f64,
        var_qbsov_db49: f64,
        var_qbsov_db5: f64,
        var_qbsov_db50: f64,
        var_qbsov_db51: f64,
        var_qbsov_db52: f64,
        var_qbsov_db53: f64,
        var_qbsov_db54: f64,
        var_qbsov_db6: f64,
        var_qbsov_db7: f64,
        var_qbsov_db8: f64,
        var_qbsov_db9: f64,
        var_qbsov_dn0: f64,
        var_qbsov_dn1: f64,
        var_qbsov_dn10: f64,
        var_qbsov_dn11: f64,
        var_qbsov_dn12: f64,
        var_qbsov_dn13: f64,
        var_qbsov_dn14: f64,
        var_qbsov_dn15: f64,
        var_qbsov_dn16: f64,
        var_qbsov_dn17: f64,
        var_qbsov_dn18: f64,
        var_qbsov_dn19: f64,
        var_qbsov_dn2: f64,
        var_qbsov_dn20: f64,
        var_qbsov_dn21: f64,
        var_qbsov_dn22: f64,
        var_qbsov_dn3: f64,
        var_qbsov_dn4: f64,
        var_qbsov_dn5: f64,
        var_qbsov_dn6: f64,
        var_qbsov_dn7: f64,
        var_qbsov_dn8: f64,
        var_qbsov_dn9: f64,
    ) {
        let eq116_e1516: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, var_qbdov);
        let eq116_e1517: f64 = (p.p7 * eq116_e1516);
        let eq116_e1517_d_n0: f64 = (p.p7 * (var_qbdov_dn0 * ddt_scale));
        let eq116_e1517_d_n1: f64 = (p.p7 * (var_qbdov_dn1 * ddt_scale));
        let eq116_e1517_d_n2: f64 = (p.p7 * (var_qbdov_dn2 * ddt_scale));
        let eq116_e1517_d_n3: f64 = (p.p7 * (var_qbdov_dn3 * ddt_scale));
        let eq116_e1517_d_n4: f64 = (p.p7 * (var_qbdov_dn4 * ddt_scale));
        let eq116_e1517_d_n5: f64 = (p.p7 * (var_qbdov_dn5 * ddt_scale));
        let eq116_e1517_d_n6: f64 = (p.p7 * (var_qbdov_dn6 * ddt_scale));
        let eq116_e1517_d_n7: f64 = (p.p7 * (var_qbdov_dn7 * ddt_scale));
        let eq116_e1517_d_n8: f64 = (p.p7 * (var_qbdov_dn8 * ddt_scale));
        let eq116_e1517_d_n9: f64 = (p.p7 * (var_qbdov_dn9 * ddt_scale));
        let eq116_e1517_d_n10: f64 = (p.p7 * (var_qbdov_dn10 * ddt_scale));
        let eq116_e1517_d_n11: f64 = (p.p7 * (var_qbdov_dn11 * ddt_scale));
        let eq116_e1517_d_n12: f64 = (p.p7 * (var_qbdov_dn12 * ddt_scale));
        let eq116_e1517_d_n13: f64 = (p.p7 * (var_qbdov_dn13 * ddt_scale));
        let eq116_e1517_d_n14: f64 = (p.p7 * (var_qbdov_dn14 * ddt_scale));
        let eq116_e1517_d_n15: f64 = (p.p7 * (var_qbdov_dn15 * ddt_scale));
        let eq116_e1517_d_n16: f64 = (p.p7 * (var_qbdov_dn16 * ddt_scale));
        let eq116_e1517_d_n17: f64 = (p.p7 * (var_qbdov_dn17 * ddt_scale));
        let eq116_e1517_d_n18: f64 = (p.p7 * (var_qbdov_dn18 * ddt_scale));
        let eq116_e1517_d_n19: f64 = (p.p7 * (var_qbdov_dn19 * ddt_scale));
        let eq116_e1517_d_n20: f64 = (p.p7 * (var_qbdov_dn20 * ddt_scale));
        let eq116_e1517_d_n21: f64 = (p.p7 * (var_qbdov_dn21 * ddt_scale));
        let eq116_e1517_d_n22: f64 = (p.p7 * (var_qbdov_dn22 * ddt_scale));
        let eq116_e1517_d_b0: f64 = (p.p7 * (var_qbdov_db0 * ddt_scale));
        let eq116_e1517_d_b1: f64 = (p.p7 * (var_qbdov_db1 * ddt_scale));
        let eq116_e1517_d_b2: f64 = (p.p7 * (var_qbdov_db2 * ddt_scale));
        let eq116_e1517_d_b3: f64 = (p.p7 * (var_qbdov_db3 * ddt_scale));
        let eq116_e1517_d_b4: f64 = (p.p7 * (var_qbdov_db4 * ddt_scale));
        let eq116_e1517_d_b5: f64 = (p.p7 * (var_qbdov_db5 * ddt_scale));
        let eq116_e1517_d_b6: f64 = (p.p7 * (var_qbdov_db6 * ddt_scale));
        let eq116_e1517_d_b7: f64 = (p.p7 * (var_qbdov_db7 * ddt_scale));
        let eq116_e1517_d_b8: f64 = (p.p7 * (var_qbdov_db8 * ddt_scale));
        let eq116_e1517_d_b9: f64 = (p.p7 * (var_qbdov_db9 * ddt_scale));
        let eq116_e1517_d_b10: f64 = (p.p7 * (var_qbdov_db10 * ddt_scale));
        let eq116_e1517_d_b11: f64 = (p.p7 * (var_qbdov_db11 * ddt_scale));
        let eq116_e1517_d_b12: f64 = (p.p7 * (var_qbdov_db12 * ddt_scale));
        let eq116_e1517_d_b13: f64 = (p.p7 * (var_qbdov_db13 * ddt_scale));
        let eq116_e1517_d_b14: f64 = (p.p7 * (var_qbdov_db14 * ddt_scale));
        let eq116_e1517_d_b15: f64 = (p.p7 * (var_qbdov_db15 * ddt_scale));
        let eq116_e1517_d_b16: f64 = (p.p7 * (var_qbdov_db16 * ddt_scale));
        let eq116_e1517_d_b17: f64 = (p.p7 * (var_qbdov_db17 * ddt_scale));
        let eq116_e1517_d_b18: f64 = (p.p7 * (var_qbdov_db18 * ddt_scale));
        let eq116_e1517_d_b19: f64 = (p.p7 * (var_qbdov_db19 * ddt_scale));
        let eq116_e1517_d_b20: f64 = (p.p7 * (var_qbdov_db20 * ddt_scale));
        let eq116_e1517_d_b21: f64 = (p.p7 * (var_qbdov_db21 * ddt_scale));
        let eq116_e1517_d_b22: f64 = (p.p7 * (var_qbdov_db22 * ddt_scale));
        let eq116_e1517_d_b23: f64 = (p.p7 * (var_qbdov_db23 * ddt_scale));
        let eq116_e1517_d_b24: f64 = (p.p7 * (var_qbdov_db24 * ddt_scale));
        let eq116_e1517_d_b25: f64 = (p.p7 * (var_qbdov_db25 * ddt_scale));
        let eq116_e1517_d_b26: f64 = (p.p7 * (var_qbdov_db26 * ddt_scale));
        let eq116_e1517_d_b27: f64 = (p.p7 * (var_qbdov_db27 * ddt_scale));
        let eq116_e1517_d_b28: f64 = (p.p7 * (var_qbdov_db28 * ddt_scale));
        let eq116_e1517_d_b29: f64 = (p.p7 * (var_qbdov_db29 * ddt_scale));
        let eq116_e1517_d_b30: f64 = (p.p7 * (var_qbdov_db30 * ddt_scale));
        let eq116_e1517_d_b31: f64 = (p.p7 * (var_qbdov_db31 * ddt_scale));
        let eq116_e1517_d_b32: f64 = (p.p7 * (var_qbdov_db32 * ddt_scale));
        let eq116_e1517_d_b33: f64 = (p.p7 * (var_qbdov_db33 * ddt_scale));
        let eq116_e1517_d_b34: f64 = (p.p7 * (var_qbdov_db34 * ddt_scale));
        let eq116_e1517_d_b35: f64 = (p.p7 * (var_qbdov_db35 * ddt_scale));
        let eq116_e1517_d_b36: f64 = (p.p7 * (var_qbdov_db36 * ddt_scale));
        let eq116_e1517_d_b37: f64 = (p.p7 * (var_qbdov_db37 * ddt_scale));
        let eq116_e1517_d_b38: f64 = (p.p7 * (var_qbdov_db38 * ddt_scale));
        let eq116_e1517_d_b39: f64 = (p.p7 * (var_qbdov_db39 * ddt_scale));
        let eq116_e1517_d_b40: f64 = (p.p7 * (var_qbdov_db40 * ddt_scale));
        let eq116_e1517_d_b41: f64 = (p.p7 * (var_qbdov_db41 * ddt_scale));
        let eq116_e1517_d_b42: f64 = (p.p7 * (var_qbdov_db42 * ddt_scale));
        let eq116_e1517_d_b43: f64 = (p.p7 * (var_qbdov_db43 * ddt_scale));
        let eq116_e1517_d_b44: f64 = (p.p7 * (var_qbdov_db44 * ddt_scale));
        let eq116_e1517_d_b45: f64 = (p.p7 * (var_qbdov_db45 * ddt_scale));
        let eq116_e1517_d_b46: f64 = (p.p7 * (var_qbdov_db46 * ddt_scale));
        let eq116_e1517_d_b47: f64 = (p.p7 * (var_qbdov_db47 * ddt_scale));
        let eq116_e1517_d_b48: f64 = (p.p7 * (var_qbdov_db48 * ddt_scale));
        let eq116_e1517_d_b49: f64 = (p.p7 * (var_qbdov_db49 * ddt_scale));
        let eq116_e1517_d_b50: f64 = (p.p7 * (var_qbdov_db50 * ddt_scale));
        let eq116_e1517_d_b51: f64 = (p.p7 * (var_qbdov_db51 * ddt_scale));
        let eq116_e1517_d_b52: f64 = (p.p7 * (var_qbdov_db52 * ddt_scale));
        let eq116_e1517_d_b53: f64 = (p.p7 * (var_qbdov_db53 * ddt_scale));
        let eq116_e1517_d_b54: f64 = (p.p7 * (var_qbdov_db54 * ddt_scale));
        let eq116_value: f64 = eq116_e1517;
        let eq116_node_derivatives: [f64; 23] = [eq116_e1517_d_n0, eq116_e1517_d_n1, eq116_e1517_d_n2, eq116_e1517_d_n3, eq116_e1517_d_n4, eq116_e1517_d_n5, eq116_e1517_d_n6, eq116_e1517_d_n7, eq116_e1517_d_n8, eq116_e1517_d_n9, eq116_e1517_d_n10, eq116_e1517_d_n11, eq116_e1517_d_n12, eq116_e1517_d_n13, eq116_e1517_d_n14, eq116_e1517_d_n15, eq116_e1517_d_n16, eq116_e1517_d_n17, eq116_e1517_d_n18, eq116_e1517_d_n19, eq116_e1517_d_n20, eq116_e1517_d_n21, eq116_e1517_d_n22];
        let eq116_branch_derivatives: [f64; 55] = [eq116_e1517_d_b0, eq116_e1517_d_b1, eq116_e1517_d_b2, eq116_e1517_d_b3, eq116_e1517_d_b4, eq116_e1517_d_b5, eq116_e1517_d_b6, eq116_e1517_d_b7, eq116_e1517_d_b8, eq116_e1517_d_b9, eq116_e1517_d_b10, eq116_e1517_d_b11, eq116_e1517_d_b12, eq116_e1517_d_b13, eq116_e1517_d_b14, eq116_e1517_d_b15, eq116_e1517_d_b16, eq116_e1517_d_b17, eq116_e1517_d_b18, eq116_e1517_d_b19, eq116_e1517_d_b20, eq116_e1517_d_b21, eq116_e1517_d_b22, eq116_e1517_d_b23, eq116_e1517_d_b24, eq116_e1517_d_b25, eq116_e1517_d_b26, eq116_e1517_d_b27, eq116_e1517_d_b28, eq116_e1517_d_b29, eq116_e1517_d_b30, eq116_e1517_d_b31, eq116_e1517_d_b32, eq116_e1517_d_b33, eq116_e1517_d_b34, eq116_e1517_d_b35, eq116_e1517_d_b36, eq116_e1517_d_b37, eq116_e1517_d_b38, eq116_e1517_d_b39, eq116_e1517_d_b40, eq116_e1517_d_b41, eq116_e1517_d_b42, eq116_e1517_d_b43, eq116_e1517_d_b44, eq116_e1517_d_b45, eq116_e1517_d_b46, eq116_e1517_d_b47, eq116_e1517_d_b48, eq116_e1517_d_b49, eq116_e1517_d_b50, eq116_e1517_d_b51, eq116_e1517_d_b52, eq116_e1517_d_b53, eq116_e1517_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(0),
            multiplicity * (eq116_value),
            &eq116_node_derivatives,
            &eq116_branch_derivatives,
            multiplicity,
        );
        let eq117_e1520: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, var_qbsov);
        let eq117_e1521: f64 = (p.p7 * eq117_e1520);
        let eq117_e1521_d_n0: f64 = (p.p7 * (var_qbsov_dn0 * ddt_scale));
        let eq117_e1521_d_n1: f64 = (p.p7 * (var_qbsov_dn1 * ddt_scale));
        let eq117_e1521_d_n2: f64 = (p.p7 * (var_qbsov_dn2 * ddt_scale));
        let eq117_e1521_d_n3: f64 = (p.p7 * (var_qbsov_dn3 * ddt_scale));
        let eq117_e1521_d_n4: f64 = (p.p7 * (var_qbsov_dn4 * ddt_scale));
        let eq117_e1521_d_n5: f64 = (p.p7 * (var_qbsov_dn5 * ddt_scale));
        let eq117_e1521_d_n6: f64 = (p.p7 * (var_qbsov_dn6 * ddt_scale));
        let eq117_e1521_d_n7: f64 = (p.p7 * (var_qbsov_dn7 * ddt_scale));
        let eq117_e1521_d_n8: f64 = (p.p7 * (var_qbsov_dn8 * ddt_scale));
        let eq117_e1521_d_n9: f64 = (p.p7 * (var_qbsov_dn9 * ddt_scale));
        let eq117_e1521_d_n10: f64 = (p.p7 * (var_qbsov_dn10 * ddt_scale));
        let eq117_e1521_d_n11: f64 = (p.p7 * (var_qbsov_dn11 * ddt_scale));
        let eq117_e1521_d_n12: f64 = (p.p7 * (var_qbsov_dn12 * ddt_scale));
        let eq117_e1521_d_n13: f64 = (p.p7 * (var_qbsov_dn13 * ddt_scale));
        let eq117_e1521_d_n14: f64 = (p.p7 * (var_qbsov_dn14 * ddt_scale));
        let eq117_e1521_d_n15: f64 = (p.p7 * (var_qbsov_dn15 * ddt_scale));
        let eq117_e1521_d_n16: f64 = (p.p7 * (var_qbsov_dn16 * ddt_scale));
        let eq117_e1521_d_n17: f64 = (p.p7 * (var_qbsov_dn17 * ddt_scale));
        let eq117_e1521_d_n18: f64 = (p.p7 * (var_qbsov_dn18 * ddt_scale));
        let eq117_e1521_d_n19: f64 = (p.p7 * (var_qbsov_dn19 * ddt_scale));
        let eq117_e1521_d_n20: f64 = (p.p7 * (var_qbsov_dn20 * ddt_scale));
        let eq117_e1521_d_n21: f64 = (p.p7 * (var_qbsov_dn21 * ddt_scale));
        let eq117_e1521_d_n22: f64 = (p.p7 * (var_qbsov_dn22 * ddt_scale));
        let eq117_e1521_d_b0: f64 = (p.p7 * (var_qbsov_db0 * ddt_scale));
        let eq117_e1521_d_b1: f64 = (p.p7 * (var_qbsov_db1 * ddt_scale));
        let eq117_e1521_d_b2: f64 = (p.p7 * (var_qbsov_db2 * ddt_scale));
        let eq117_e1521_d_b3: f64 = (p.p7 * (var_qbsov_db3 * ddt_scale));
        let eq117_e1521_d_b4: f64 = (p.p7 * (var_qbsov_db4 * ddt_scale));
        let eq117_e1521_d_b5: f64 = (p.p7 * (var_qbsov_db5 * ddt_scale));
        let eq117_e1521_d_b6: f64 = (p.p7 * (var_qbsov_db6 * ddt_scale));
        let eq117_e1521_d_b7: f64 = (p.p7 * (var_qbsov_db7 * ddt_scale));
        let eq117_e1521_d_b8: f64 = (p.p7 * (var_qbsov_db8 * ddt_scale));
        let eq117_e1521_d_b9: f64 = (p.p7 * (var_qbsov_db9 * ddt_scale));
        let eq117_e1521_d_b10: f64 = (p.p7 * (var_qbsov_db10 * ddt_scale));
        let eq117_e1521_d_b11: f64 = (p.p7 * (var_qbsov_db11 * ddt_scale));
        let eq117_e1521_d_b12: f64 = (p.p7 * (var_qbsov_db12 * ddt_scale));
        let eq117_e1521_d_b13: f64 = (p.p7 * (var_qbsov_db13 * ddt_scale));
        let eq117_e1521_d_b14: f64 = (p.p7 * (var_qbsov_db14 * ddt_scale));
        let eq117_e1521_d_b15: f64 = (p.p7 * (var_qbsov_db15 * ddt_scale));
        let eq117_e1521_d_b16: f64 = (p.p7 * (var_qbsov_db16 * ddt_scale));
        let eq117_e1521_d_b17: f64 = (p.p7 * (var_qbsov_db17 * ddt_scale));
        let eq117_e1521_d_b18: f64 = (p.p7 * (var_qbsov_db18 * ddt_scale));
        let eq117_e1521_d_b19: f64 = (p.p7 * (var_qbsov_db19 * ddt_scale));
        let eq117_e1521_d_b20: f64 = (p.p7 * (var_qbsov_db20 * ddt_scale));
        let eq117_e1521_d_b21: f64 = (p.p7 * (var_qbsov_db21 * ddt_scale));
        let eq117_e1521_d_b22: f64 = (p.p7 * (var_qbsov_db22 * ddt_scale));
        let eq117_e1521_d_b23: f64 = (p.p7 * (var_qbsov_db23 * ddt_scale));
        let eq117_e1521_d_b24: f64 = (p.p7 * (var_qbsov_db24 * ddt_scale));
        let eq117_e1521_d_b25: f64 = (p.p7 * (var_qbsov_db25 * ddt_scale));
        let eq117_e1521_d_b26: f64 = (p.p7 * (var_qbsov_db26 * ddt_scale));
        let eq117_e1521_d_b27: f64 = (p.p7 * (var_qbsov_db27 * ddt_scale));
        let eq117_e1521_d_b28: f64 = (p.p7 * (var_qbsov_db28 * ddt_scale));
        let eq117_e1521_d_b29: f64 = (p.p7 * (var_qbsov_db29 * ddt_scale));
        let eq117_e1521_d_b30: f64 = (p.p7 * (var_qbsov_db30 * ddt_scale));
        let eq117_e1521_d_b31: f64 = (p.p7 * (var_qbsov_db31 * ddt_scale));
        let eq117_e1521_d_b32: f64 = (p.p7 * (var_qbsov_db32 * ddt_scale));
        let eq117_e1521_d_b33: f64 = (p.p7 * (var_qbsov_db33 * ddt_scale));
        let eq117_e1521_d_b34: f64 = (p.p7 * (var_qbsov_db34 * ddt_scale));
        let eq117_e1521_d_b35: f64 = (p.p7 * (var_qbsov_db35 * ddt_scale));
        let eq117_e1521_d_b36: f64 = (p.p7 * (var_qbsov_db36 * ddt_scale));
        let eq117_e1521_d_b37: f64 = (p.p7 * (var_qbsov_db37 * ddt_scale));
        let eq117_e1521_d_b38: f64 = (p.p7 * (var_qbsov_db38 * ddt_scale));
        let eq117_e1521_d_b39: f64 = (p.p7 * (var_qbsov_db39 * ddt_scale));
        let eq117_e1521_d_b40: f64 = (p.p7 * (var_qbsov_db40 * ddt_scale));
        let eq117_e1521_d_b41: f64 = (p.p7 * (var_qbsov_db41 * ddt_scale));
        let eq117_e1521_d_b42: f64 = (p.p7 * (var_qbsov_db42 * ddt_scale));
        let eq117_e1521_d_b43: f64 = (p.p7 * (var_qbsov_db43 * ddt_scale));
        let eq117_e1521_d_b44: f64 = (p.p7 * (var_qbsov_db44 * ddt_scale));
        let eq117_e1521_d_b45: f64 = (p.p7 * (var_qbsov_db45 * ddt_scale));
        let eq117_e1521_d_b46: f64 = (p.p7 * (var_qbsov_db46 * ddt_scale));
        let eq117_e1521_d_b47: f64 = (p.p7 * (var_qbsov_db47 * ddt_scale));
        let eq117_e1521_d_b48: f64 = (p.p7 * (var_qbsov_db48 * ddt_scale));
        let eq117_e1521_d_b49: f64 = (p.p7 * (var_qbsov_db49 * ddt_scale));
        let eq117_e1521_d_b50: f64 = (p.p7 * (var_qbsov_db50 * ddt_scale));
        let eq117_e1521_d_b51: f64 = (p.p7 * (var_qbsov_db51 * ddt_scale));
        let eq117_e1521_d_b52: f64 = (p.p7 * (var_qbsov_db52 * ddt_scale));
        let eq117_e1521_d_b53: f64 = (p.p7 * (var_qbsov_db53 * ddt_scale));
        let eq117_e1521_d_b54: f64 = (p.p7 * (var_qbsov_db54 * ddt_scale));
        let eq117_value: f64 = eq117_e1521;
        let eq117_node_derivatives: [f64; 23] = [eq117_e1521_d_n0, eq117_e1521_d_n1, eq117_e1521_d_n2, eq117_e1521_d_n3, eq117_e1521_d_n4, eq117_e1521_d_n5, eq117_e1521_d_n6, eq117_e1521_d_n7, eq117_e1521_d_n8, eq117_e1521_d_n9, eq117_e1521_d_n10, eq117_e1521_d_n11, eq117_e1521_d_n12, eq117_e1521_d_n13, eq117_e1521_d_n14, eq117_e1521_d_n15, eq117_e1521_d_n16, eq117_e1521_d_n17, eq117_e1521_d_n18, eq117_e1521_d_n19, eq117_e1521_d_n20, eq117_e1521_d_n21, eq117_e1521_d_n22];
        let eq117_branch_derivatives: [f64; 55] = [eq117_e1521_d_b0, eq117_e1521_d_b1, eq117_e1521_d_b2, eq117_e1521_d_b3, eq117_e1521_d_b4, eq117_e1521_d_b5, eq117_e1521_d_b6, eq117_e1521_d_b7, eq117_e1521_d_b8, eq117_e1521_d_b9, eq117_e1521_d_b10, eq117_e1521_d_b11, eq117_e1521_d_b12, eq117_e1521_d_b13, eq117_e1521_d_b14, eq117_e1521_d_b15, eq117_e1521_d_b16, eq117_e1521_d_b17, eq117_e1521_d_b18, eq117_e1521_d_b19, eq117_e1521_d_b20, eq117_e1521_d_b21, eq117_e1521_d_b22, eq117_e1521_d_b23, eq117_e1521_d_b24, eq117_e1521_d_b25, eq117_e1521_d_b26, eq117_e1521_d_b27, eq117_e1521_d_b28, eq117_e1521_d_b29, eq117_e1521_d_b30, eq117_e1521_d_b31, eq117_e1521_d_b32, eq117_e1521_d_b33, eq117_e1521_d_b34, eq117_e1521_d_b35, eq117_e1521_d_b36, eq117_e1521_d_b37, eq117_e1521_d_b38, eq117_e1521_d_b39, eq117_e1521_d_b40, eq117_e1521_d_b41, eq117_e1521_d_b42, eq117_e1521_d_b43, eq117_e1521_d_b44, eq117_e1521_d_b45, eq117_e1521_d_b46, eq117_e1521_d_b47, eq117_e1521_d_b48, eq117_e1521_d_b49, eq117_e1521_d_b50, eq117_e1521_d_b51, eq117_e1521_d_b52, eq117_e1521_d_b53, eq117_e1521_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(2),
            multiplicity * (eq117_value),
            &eq117_node_derivatives,
            &eq117_branch_derivatives,
            multiplicity,
        );
        let eq118_e1524: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, var_qbgov);
        let eq118_e1525: f64 = (p.p7 * eq118_e1524);
        let eq118_e1525_d_n0: f64 = (p.p7 * (var_qbgov_dn0 * ddt_scale));
        let eq118_e1525_d_n1: f64 = (p.p7 * (var_qbgov_dn1 * ddt_scale));
        let eq118_e1525_d_n2: f64 = (p.p7 * (var_qbgov_dn2 * ddt_scale));
        let eq118_e1525_d_n3: f64 = (p.p7 * (var_qbgov_dn3 * ddt_scale));
        let eq118_e1525_d_n4: f64 = (p.p7 * (var_qbgov_dn4 * ddt_scale));
        let eq118_e1525_d_n5: f64 = (p.p7 * (var_qbgov_dn5 * ddt_scale));
        let eq118_e1525_d_n6: f64 = (p.p7 * (var_qbgov_dn6 * ddt_scale));
        let eq118_e1525_d_n7: f64 = (p.p7 * (var_qbgov_dn7 * ddt_scale));
        let eq118_e1525_d_n8: f64 = (p.p7 * (var_qbgov_dn8 * ddt_scale));
        let eq118_e1525_d_n9: f64 = (p.p7 * (var_qbgov_dn9 * ddt_scale));
        let eq118_e1525_d_n10: f64 = (p.p7 * (var_qbgov_dn10 * ddt_scale));
        let eq118_e1525_d_n11: f64 = (p.p7 * (var_qbgov_dn11 * ddt_scale));
        let eq118_e1525_d_n12: f64 = (p.p7 * (var_qbgov_dn12 * ddt_scale));
        let eq118_e1525_d_n13: f64 = (p.p7 * (var_qbgov_dn13 * ddt_scale));
        let eq118_e1525_d_n14: f64 = (p.p7 * (var_qbgov_dn14 * ddt_scale));
        let eq118_e1525_d_n15: f64 = (p.p7 * (var_qbgov_dn15 * ddt_scale));
        let eq118_e1525_d_n16: f64 = (p.p7 * (var_qbgov_dn16 * ddt_scale));
        let eq118_e1525_d_n17: f64 = (p.p7 * (var_qbgov_dn17 * ddt_scale));
        let eq118_e1525_d_n18: f64 = (p.p7 * (var_qbgov_dn18 * ddt_scale));
        let eq118_e1525_d_n19: f64 = (p.p7 * (var_qbgov_dn19 * ddt_scale));
        let eq118_e1525_d_n20: f64 = (p.p7 * (var_qbgov_dn20 * ddt_scale));
        let eq118_e1525_d_n21: f64 = (p.p7 * (var_qbgov_dn21 * ddt_scale));
        let eq118_e1525_d_n22: f64 = (p.p7 * (var_qbgov_dn22 * ddt_scale));
        let eq118_e1525_d_b0: f64 = (p.p7 * (var_qbgov_db0 * ddt_scale));
        let eq118_e1525_d_b1: f64 = (p.p7 * (var_qbgov_db1 * ddt_scale));
        let eq118_e1525_d_b2: f64 = (p.p7 * (var_qbgov_db2 * ddt_scale));
        let eq118_e1525_d_b3: f64 = (p.p7 * (var_qbgov_db3 * ddt_scale));
        let eq118_e1525_d_b4: f64 = (p.p7 * (var_qbgov_db4 * ddt_scale));
        let eq118_e1525_d_b5: f64 = (p.p7 * (var_qbgov_db5 * ddt_scale));
        let eq118_e1525_d_b6: f64 = (p.p7 * (var_qbgov_db6 * ddt_scale));
        let eq118_e1525_d_b7: f64 = (p.p7 * (var_qbgov_db7 * ddt_scale));
        let eq118_e1525_d_b8: f64 = (p.p7 * (var_qbgov_db8 * ddt_scale));
        let eq118_e1525_d_b9: f64 = (p.p7 * (var_qbgov_db9 * ddt_scale));
        let eq118_e1525_d_b10: f64 = (p.p7 * (var_qbgov_db10 * ddt_scale));
        let eq118_e1525_d_b11: f64 = (p.p7 * (var_qbgov_db11 * ddt_scale));
        let eq118_e1525_d_b12: f64 = (p.p7 * (var_qbgov_db12 * ddt_scale));
        let eq118_e1525_d_b13: f64 = (p.p7 * (var_qbgov_db13 * ddt_scale));
        let eq118_e1525_d_b14: f64 = (p.p7 * (var_qbgov_db14 * ddt_scale));
        let eq118_e1525_d_b15: f64 = (p.p7 * (var_qbgov_db15 * ddt_scale));
        let eq118_e1525_d_b16: f64 = (p.p7 * (var_qbgov_db16 * ddt_scale));
        let eq118_e1525_d_b17: f64 = (p.p7 * (var_qbgov_db17 * ddt_scale));
        let eq118_e1525_d_b18: f64 = (p.p7 * (var_qbgov_db18 * ddt_scale));
        let eq118_e1525_d_b19: f64 = (p.p7 * (var_qbgov_db19 * ddt_scale));
        let eq118_e1525_d_b20: f64 = (p.p7 * (var_qbgov_db20 * ddt_scale));
        let eq118_e1525_d_b21: f64 = (p.p7 * (var_qbgov_db21 * ddt_scale));
        let eq118_e1525_d_b22: f64 = (p.p7 * (var_qbgov_db22 * ddt_scale));
        let eq118_e1525_d_b23: f64 = (p.p7 * (var_qbgov_db23 * ddt_scale));
        let eq118_e1525_d_b24: f64 = (p.p7 * (var_qbgov_db24 * ddt_scale));
        let eq118_e1525_d_b25: f64 = (p.p7 * (var_qbgov_db25 * ddt_scale));
        let eq118_e1525_d_b26: f64 = (p.p7 * (var_qbgov_db26 * ddt_scale));
        let eq118_e1525_d_b27: f64 = (p.p7 * (var_qbgov_db27 * ddt_scale));
        let eq118_e1525_d_b28: f64 = (p.p7 * (var_qbgov_db28 * ddt_scale));
        let eq118_e1525_d_b29: f64 = (p.p7 * (var_qbgov_db29 * ddt_scale));
        let eq118_e1525_d_b30: f64 = (p.p7 * (var_qbgov_db30 * ddt_scale));
        let eq118_e1525_d_b31: f64 = (p.p7 * (var_qbgov_db31 * ddt_scale));
        let eq118_e1525_d_b32: f64 = (p.p7 * (var_qbgov_db32 * ddt_scale));
        let eq118_e1525_d_b33: f64 = (p.p7 * (var_qbgov_db33 * ddt_scale));
        let eq118_e1525_d_b34: f64 = (p.p7 * (var_qbgov_db34 * ddt_scale));
        let eq118_e1525_d_b35: f64 = (p.p7 * (var_qbgov_db35 * ddt_scale));
        let eq118_e1525_d_b36: f64 = (p.p7 * (var_qbgov_db36 * ddt_scale));
        let eq118_e1525_d_b37: f64 = (p.p7 * (var_qbgov_db37 * ddt_scale));
        let eq118_e1525_d_b38: f64 = (p.p7 * (var_qbgov_db38 * ddt_scale));
        let eq118_e1525_d_b39: f64 = (p.p7 * (var_qbgov_db39 * ddt_scale));
        let eq118_e1525_d_b40: f64 = (p.p7 * (var_qbgov_db40 * ddt_scale));
        let eq118_e1525_d_b41: f64 = (p.p7 * (var_qbgov_db41 * ddt_scale));
        let eq118_e1525_d_b42: f64 = (p.p7 * (var_qbgov_db42 * ddt_scale));
        let eq118_e1525_d_b43: f64 = (p.p7 * (var_qbgov_db43 * ddt_scale));
        let eq118_e1525_d_b44: f64 = (p.p7 * (var_qbgov_db44 * ddt_scale));
        let eq118_e1525_d_b45: f64 = (p.p7 * (var_qbgov_db45 * ddt_scale));
        let eq118_e1525_d_b46: f64 = (p.p7 * (var_qbgov_db46 * ddt_scale));
        let eq118_e1525_d_b47: f64 = (p.p7 * (var_qbgov_db47 * ddt_scale));
        let eq118_e1525_d_b48: f64 = (p.p7 * (var_qbgov_db48 * ddt_scale));
        let eq118_e1525_d_b49: f64 = (p.p7 * (var_qbgov_db49 * ddt_scale));
        let eq118_e1525_d_b50: f64 = (p.p7 * (var_qbgov_db50 * ddt_scale));
        let eq118_e1525_d_b51: f64 = (p.p7 * (var_qbgov_db51 * ddt_scale));
        let eq118_e1525_d_b52: f64 = (p.p7 * (var_qbgov_db52 * ddt_scale));
        let eq118_e1525_d_b53: f64 = (p.p7 * (var_qbgov_db53 * ddt_scale));
        let eq118_e1525_d_b54: f64 = (p.p7 * (var_qbgov_db54 * ddt_scale));
        let eq118_value: f64 = eq118_e1525;
        let eq118_node_derivatives: [f64; 23] = [eq118_e1525_d_n0, eq118_e1525_d_n1, eq118_e1525_d_n2, eq118_e1525_d_n3, eq118_e1525_d_n4, eq118_e1525_d_n5, eq118_e1525_d_n6, eq118_e1525_d_n7, eq118_e1525_d_n8, eq118_e1525_d_n9, eq118_e1525_d_n10, eq118_e1525_d_n11, eq118_e1525_d_n12, eq118_e1525_d_n13, eq118_e1525_d_n14, eq118_e1525_d_n15, eq118_e1525_d_n16, eq118_e1525_d_n17, eq118_e1525_d_n18, eq118_e1525_d_n19, eq118_e1525_d_n20, eq118_e1525_d_n21, eq118_e1525_d_n22];
        let eq118_branch_derivatives: [f64; 55] = [eq118_e1525_d_b0, eq118_e1525_d_b1, eq118_e1525_d_b2, eq118_e1525_d_b3, eq118_e1525_d_b4, eq118_e1525_d_b5, eq118_e1525_d_b6, eq118_e1525_d_b7, eq118_e1525_d_b8, eq118_e1525_d_b9, eq118_e1525_d_b10, eq118_e1525_d_b11, eq118_e1525_d_b12, eq118_e1525_d_b13, eq118_e1525_d_b14, eq118_e1525_d_b15, eq118_e1525_d_b16, eq118_e1525_d_b17, eq118_e1525_d_b18, eq118_e1525_d_b19, eq118_e1525_d_b20, eq118_e1525_d_b21, eq118_e1525_d_b22, eq118_e1525_d_b23, eq118_e1525_d_b24, eq118_e1525_d_b25, eq118_e1525_d_b26, eq118_e1525_d_b27, eq118_e1525_d_b28, eq118_e1525_d_b29, eq118_e1525_d_b30, eq118_e1525_d_b31, eq118_e1525_d_b32, eq118_e1525_d_b33, eq118_e1525_d_b34, eq118_e1525_d_b35, eq118_e1525_d_b36, eq118_e1525_d_b37, eq118_e1525_d_b38, eq118_e1525_d_b39, eq118_e1525_d_b40, eq118_e1525_d_b41, eq118_e1525_d_b42, eq118_e1525_d_b43, eq118_e1525_d_b44, eq118_e1525_d_b45, eq118_e1525_d_b46, eq118_e1525_d_b47, eq118_e1525_d_b48, eq118_e1525_d_b49, eq118_e1525_d_b50, eq118_e1525_d_b51, eq118_e1525_d_b52, eq118_e1525_d_b53, eq118_e1525_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(1),
            multiplicity * (eq118_value),
            &eq118_node_derivatives,
            &eq118_branch_derivatives,
            multiplicity,
        );
        let eq119_e1529: f64 = (p.p250 * s.v[161]);
        let eq119_e1530: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq119_e1529);
        let eq119_e1530_d_n0: f64 = ((p.p250 * s.dn[161][0]) * ddt_scale);
        let eq119_e1530_d_n1: f64 = ((p.p250 * s.dn[161][1]) * ddt_scale);
        let eq119_e1530_d_n2: f64 = ((p.p250 * s.dn[161][2]) * ddt_scale);
        let eq119_e1530_d_n3: f64 = ((p.p250 * s.dn[161][3]) * ddt_scale);
        let eq119_e1530_d_n4: f64 = ((p.p250 * s.dn[161][4]) * ddt_scale);
        let eq119_e1530_d_n5: f64 = ((p.p250 * s.dn[161][5]) * ddt_scale);
        let eq119_e1530_d_n6: f64 = ((p.p250 * s.dn[161][6]) * ddt_scale);
        let eq119_e1530_d_n7: f64 = ((p.p250 * s.dn[161][7]) * ddt_scale);
        let eq119_e1530_d_n8: f64 = ((p.p250 * s.dn[161][8]) * ddt_scale);
        let eq119_e1530_d_n9: f64 = ((p.p250 * s.dn[161][9]) * ddt_scale);
        let eq119_e1530_d_n10: f64 = ((p.p250 * s.dn[161][10]) * ddt_scale);
        let eq119_e1530_d_n11: f64 = ((p.p250 * s.dn[161][11]) * ddt_scale);
        let eq119_e1530_d_n12: f64 = ((p.p250 * s.dn[161][12]) * ddt_scale);
        let eq119_e1530_d_n13: f64 = ((p.p250 * s.dn[161][13]) * ddt_scale);
        let eq119_e1530_d_n14: f64 = ((p.p250 * s.dn[161][14]) * ddt_scale);
        let eq119_e1530_d_n15: f64 = ((p.p250 * s.dn[161][15]) * ddt_scale);
        let eq119_e1530_d_n16: f64 = ((p.p250 * s.dn[161][16]) * ddt_scale);
        let eq119_e1530_d_n17: f64 = ((p.p250 * s.dn[161][17]) * ddt_scale);
        let eq119_e1530_d_n18: f64 = ((p.p250 * s.dn[161][18]) * ddt_scale);
        let eq119_e1530_d_n19: f64 = ((p.p250 * s.dn[161][19]) * ddt_scale);
        let eq119_e1530_d_n20: f64 = ((p.p250 * s.dn[161][20]) * ddt_scale);
        let eq119_e1530_d_n21: f64 = ((p.p250 * s.dn[161][21]) * ddt_scale);
        let eq119_e1530_d_n22: f64 = ((p.p250 * s.dn[161][22]) * ddt_scale);
        let eq119_e1530_d_b0: f64 = ((p.p250 * s.db[161][0]) * ddt_scale);
        let eq119_e1530_d_b1: f64 = ((p.p250 * s.db[161][1]) * ddt_scale);
        let eq119_e1530_d_b2: f64 = ((p.p250 * s.db[161][2]) * ddt_scale);
        let eq119_e1530_d_b3: f64 = ((p.p250 * s.db[161][3]) * ddt_scale);
        let eq119_e1530_d_b4: f64 = ((p.p250 * s.db[161][4]) * ddt_scale);
        let eq119_e1530_d_b5: f64 = ((p.p250 * s.db[161][5]) * ddt_scale);
        let eq119_e1530_d_b6: f64 = ((p.p250 * s.db[161][6]) * ddt_scale);
        let eq119_e1530_d_b7: f64 = ((p.p250 * s.db[161][7]) * ddt_scale);
        let eq119_e1530_d_b8: f64 = ((p.p250 * s.db[161][8]) * ddt_scale);
        let eq119_e1530_d_b9: f64 = ((p.p250 * s.db[161][9]) * ddt_scale);
        let eq119_e1530_d_b10: f64 = ((p.p250 * s.db[161][10]) * ddt_scale);
        let eq119_e1530_d_b11: f64 = ((p.p250 * s.db[161][11]) * ddt_scale);
        let eq119_e1530_d_b12: f64 = ((p.p250 * s.db[161][12]) * ddt_scale);
        let eq119_e1530_d_b13: f64 = ((p.p250 * s.db[161][13]) * ddt_scale);
        let eq119_e1530_d_b14: f64 = ((p.p250 * s.db[161][14]) * ddt_scale);
        let eq119_e1530_d_b15: f64 = ((p.p250 * s.db[161][15]) * ddt_scale);
        let eq119_e1530_d_b16: f64 = ((p.p250 * s.db[161][16]) * ddt_scale);
        let eq119_e1530_d_b17: f64 = ((p.p250 * s.db[161][17]) * ddt_scale);
        let eq119_e1530_d_b18: f64 = ((p.p250 * s.db[161][18]) * ddt_scale);
        let eq119_e1530_d_b19: f64 = ((p.p250 * s.db[161][19]) * ddt_scale);
        let eq119_e1530_d_b20: f64 = ((p.p250 * s.db[161][20]) * ddt_scale);
        let eq119_e1530_d_b21: f64 = ((p.p250 * s.db[161][21]) * ddt_scale);
        let eq119_e1530_d_b22: f64 = ((p.p250 * s.db[161][22]) * ddt_scale);
        let eq119_e1530_d_b23: f64 = ((p.p250 * s.db[161][23]) * ddt_scale);
        let eq119_e1530_d_b24: f64 = ((p.p250 * s.db[161][24]) * ddt_scale);
        let eq119_e1530_d_b25: f64 = ((p.p250 * s.db[161][25]) * ddt_scale);
        let eq119_e1530_d_b26: f64 = ((p.p250 * s.db[161][26]) * ddt_scale);
        let eq119_e1530_d_b27: f64 = ((p.p250 * s.db[161][27]) * ddt_scale);
        let eq119_e1530_d_b28: f64 = ((p.p250 * s.db[161][28]) * ddt_scale);
        let eq119_e1530_d_b29: f64 = ((p.p250 * s.db[161][29]) * ddt_scale);
        let eq119_e1530_d_b30: f64 = ((p.p250 * s.db[161][30]) * ddt_scale);
        let eq119_e1530_d_b31: f64 = ((p.p250 * s.db[161][31]) * ddt_scale);
        let eq119_e1530_d_b32: f64 = ((p.p250 * s.db[161][32]) * ddt_scale);
        let eq119_e1530_d_b33: f64 = ((p.p250 * s.db[161][33]) * ddt_scale);
        let eq119_e1530_d_b34: f64 = ((p.p250 * s.db[161][34]) * ddt_scale);
        let eq119_e1530_d_b35: f64 = ((p.p250 * s.db[161][35]) * ddt_scale);
        let eq119_e1530_d_b36: f64 = ((p.p250 * s.db[161][36]) * ddt_scale);
        let eq119_e1530_d_b37: f64 = ((p.p250 * s.db[161][37]) * ddt_scale);
        let eq119_e1530_d_b38: f64 = ((p.p250 * s.db[161][38]) * ddt_scale);
        let eq119_e1530_d_b39: f64 = ((p.p250 * s.db[161][39]) * ddt_scale);
        let eq119_e1530_d_b40: f64 = ((p.p250 * s.db[161][40]) * ddt_scale);
        let eq119_e1530_d_b41: f64 = ((p.p250 * s.db[161][41]) * ddt_scale);
        let eq119_e1530_d_b42: f64 = ((p.p250 * s.db[161][42]) * ddt_scale);
        let eq119_e1530_d_b43: f64 = ((p.p250 * s.db[161][43]) * ddt_scale);
        let eq119_e1530_d_b44: f64 = ((p.p250 * s.db[161][44]) * ddt_scale);
        let eq119_e1530_d_b45: f64 = ((p.p250 * s.db[161][45]) * ddt_scale);
        let eq119_e1530_d_b46: f64 = ((p.p250 * s.db[161][46]) * ddt_scale);
        let eq119_e1530_d_b47: f64 = ((p.p250 * s.db[161][47]) * ddt_scale);
        let eq119_e1530_d_b48: f64 = ((p.p250 * s.db[161][48]) * ddt_scale);
        let eq119_e1530_d_b49: f64 = ((p.p250 * s.db[161][49]) * ddt_scale);
        let eq119_e1530_d_b50: f64 = ((p.p250 * s.db[161][50]) * ddt_scale);
        let eq119_e1530_d_b51: f64 = ((p.p250 * s.db[161][51]) * ddt_scale);
        let eq119_e1530_d_b52: f64 = ((p.p250 * s.db[161][52]) * ddt_scale);
        let eq119_e1530_d_b53: f64 = ((p.p250 * s.db[161][53]) * ddt_scale);
        let eq119_e1530_d_b54: f64 = ((p.p250 * s.db[161][54]) * ddt_scale);
        let eq119_e1531: f64 = (p.p7 * eq119_e1530);
        let eq119_e1531_d_n0: f64 = (p.p7 * eq119_e1530_d_n0);
        let eq119_e1531_d_n1: f64 = (p.p7 * eq119_e1530_d_n1);
        let eq119_e1531_d_n2: f64 = (p.p7 * eq119_e1530_d_n2);
        let eq119_e1531_d_n3: f64 = (p.p7 * eq119_e1530_d_n3);
        let eq119_e1531_d_n4: f64 = (p.p7 * eq119_e1530_d_n4);
        let eq119_e1531_d_n5: f64 = (p.p7 * eq119_e1530_d_n5);
        let eq119_e1531_d_n6: f64 = (p.p7 * eq119_e1530_d_n6);
        let eq119_e1531_d_n7: f64 = (p.p7 * eq119_e1530_d_n7);
        let eq119_e1531_d_n8: f64 = (p.p7 * eq119_e1530_d_n8);
        let eq119_e1531_d_n9: f64 = (p.p7 * eq119_e1530_d_n9);
        let eq119_e1531_d_n10: f64 = (p.p7 * eq119_e1530_d_n10);
        let eq119_e1531_d_n11: f64 = (p.p7 * eq119_e1530_d_n11);
        let eq119_e1531_d_n12: f64 = (p.p7 * eq119_e1530_d_n12);
        let eq119_e1531_d_n13: f64 = (p.p7 * eq119_e1530_d_n13);
        let eq119_e1531_d_n14: f64 = (p.p7 * eq119_e1530_d_n14);
        let eq119_e1531_d_n15: f64 = (p.p7 * eq119_e1530_d_n15);
        let eq119_e1531_d_n16: f64 = (p.p7 * eq119_e1530_d_n16);
        let eq119_e1531_d_n17: f64 = (p.p7 * eq119_e1530_d_n17);
        let eq119_e1531_d_n18: f64 = (p.p7 * eq119_e1530_d_n18);
        let eq119_e1531_d_n19: f64 = (p.p7 * eq119_e1530_d_n19);
        let eq119_e1531_d_n20: f64 = (p.p7 * eq119_e1530_d_n20);
        let eq119_e1531_d_n21: f64 = (p.p7 * eq119_e1530_d_n21);
        let eq119_e1531_d_n22: f64 = (p.p7 * eq119_e1530_d_n22);
        let eq119_e1531_d_b0: f64 = (p.p7 * eq119_e1530_d_b0);
        let eq119_e1531_d_b1: f64 = (p.p7 * eq119_e1530_d_b1);
        let eq119_e1531_d_b2: f64 = (p.p7 * eq119_e1530_d_b2);
        let eq119_e1531_d_b3: f64 = (p.p7 * eq119_e1530_d_b3);
        let eq119_e1531_d_b4: f64 = (p.p7 * eq119_e1530_d_b4);
        let eq119_e1531_d_b5: f64 = (p.p7 * eq119_e1530_d_b5);
        let eq119_e1531_d_b6: f64 = (p.p7 * eq119_e1530_d_b6);
        let eq119_e1531_d_b7: f64 = (p.p7 * eq119_e1530_d_b7);
        let eq119_e1531_d_b8: f64 = (p.p7 * eq119_e1530_d_b8);
        let eq119_e1531_d_b9: f64 = (p.p7 * eq119_e1530_d_b9);
        let eq119_e1531_d_b10: f64 = (p.p7 * eq119_e1530_d_b10);
        let eq119_e1531_d_b11: f64 = (p.p7 * eq119_e1530_d_b11);
        let eq119_e1531_d_b12: f64 = (p.p7 * eq119_e1530_d_b12);
        let eq119_e1531_d_b13: f64 = (p.p7 * eq119_e1530_d_b13);
        let eq119_e1531_d_b14: f64 = (p.p7 * eq119_e1530_d_b14);
        let eq119_e1531_d_b15: f64 = (p.p7 * eq119_e1530_d_b15);
        let eq119_e1531_d_b16: f64 = (p.p7 * eq119_e1530_d_b16);
        let eq119_e1531_d_b17: f64 = (p.p7 * eq119_e1530_d_b17);
        let eq119_e1531_d_b18: f64 = (p.p7 * eq119_e1530_d_b18);
        let eq119_e1531_d_b19: f64 = (p.p7 * eq119_e1530_d_b19);
        let eq119_e1531_d_b20: f64 = (p.p7 * eq119_e1530_d_b20);
        let eq119_e1531_d_b21: f64 = (p.p7 * eq119_e1530_d_b21);
        let eq119_e1531_d_b22: f64 = (p.p7 * eq119_e1530_d_b22);
        let eq119_e1531_d_b23: f64 = (p.p7 * eq119_e1530_d_b23);
        let eq119_e1531_d_b24: f64 = (p.p7 * eq119_e1530_d_b24);
        let eq119_e1531_d_b25: f64 = (p.p7 * eq119_e1530_d_b25);
        let eq119_e1531_d_b26: f64 = (p.p7 * eq119_e1530_d_b26);
        let eq119_e1531_d_b27: f64 = (p.p7 * eq119_e1530_d_b27);
        let eq119_e1531_d_b28: f64 = (p.p7 * eq119_e1530_d_b28);
        let eq119_e1531_d_b29: f64 = (p.p7 * eq119_e1530_d_b29);
        let eq119_e1531_d_b30: f64 = (p.p7 * eq119_e1530_d_b30);
        let eq119_e1531_d_b31: f64 = (p.p7 * eq119_e1530_d_b31);
        let eq119_e1531_d_b32: f64 = (p.p7 * eq119_e1530_d_b32);
        let eq119_e1531_d_b33: f64 = (p.p7 * eq119_e1530_d_b33);
        let eq119_e1531_d_b34: f64 = (p.p7 * eq119_e1530_d_b34);
        let eq119_e1531_d_b35: f64 = (p.p7 * eq119_e1530_d_b35);
        let eq119_e1531_d_b36: f64 = (p.p7 * eq119_e1530_d_b36);
        let eq119_e1531_d_b37: f64 = (p.p7 * eq119_e1530_d_b37);
        let eq119_e1531_d_b38: f64 = (p.p7 * eq119_e1530_d_b38);
        let eq119_e1531_d_b39: f64 = (p.p7 * eq119_e1530_d_b39);
        let eq119_e1531_d_b40: f64 = (p.p7 * eq119_e1530_d_b40);
        let eq119_e1531_d_b41: f64 = (p.p7 * eq119_e1530_d_b41);
        let eq119_e1531_d_b42: f64 = (p.p7 * eq119_e1530_d_b42);
        let eq119_e1531_d_b43: f64 = (p.p7 * eq119_e1530_d_b43);
        let eq119_e1531_d_b44: f64 = (p.p7 * eq119_e1530_d_b44);
        let eq119_e1531_d_b45: f64 = (p.p7 * eq119_e1530_d_b45);
        let eq119_e1531_d_b46: f64 = (p.p7 * eq119_e1530_d_b46);
        let eq119_e1531_d_b47: f64 = (p.p7 * eq119_e1530_d_b47);
        let eq119_e1531_d_b48: f64 = (p.p7 * eq119_e1530_d_b48);
        let eq119_e1531_d_b49: f64 = (p.p7 * eq119_e1530_d_b49);
        let eq119_e1531_d_b50: f64 = (p.p7 * eq119_e1530_d_b50);
        let eq119_e1531_d_b51: f64 = (p.p7 * eq119_e1530_d_b51);
        let eq119_e1531_d_b52: f64 = (p.p7 * eq119_e1530_d_b52);
        let eq119_e1531_d_b53: f64 = (p.p7 * eq119_e1530_d_b53);
        let eq119_e1531_d_b54: f64 = (p.p7 * eq119_e1530_d_b54);
        let eq119_value: f64 = eq119_e1531;
        let eq119_node_derivatives: [f64; 23] = [eq119_e1531_d_n0, eq119_e1531_d_n1, eq119_e1531_d_n2, eq119_e1531_d_n3, eq119_e1531_d_n4, eq119_e1531_d_n5, eq119_e1531_d_n6, eq119_e1531_d_n7, eq119_e1531_d_n8, eq119_e1531_d_n9, eq119_e1531_d_n10, eq119_e1531_d_n11, eq119_e1531_d_n12, eq119_e1531_d_n13, eq119_e1531_d_n14, eq119_e1531_d_n15, eq119_e1531_d_n16, eq119_e1531_d_n17, eq119_e1531_d_n18, eq119_e1531_d_n19, eq119_e1531_d_n20, eq119_e1531_d_n21, eq119_e1531_d_n22];
        let eq119_branch_derivatives: [f64; 55] = [eq119_e1531_d_b0, eq119_e1531_d_b1, eq119_e1531_d_b2, eq119_e1531_d_b3, eq119_e1531_d_b4, eq119_e1531_d_b5, eq119_e1531_d_b6, eq119_e1531_d_b7, eq119_e1531_d_b8, eq119_e1531_d_b9, eq119_e1531_d_b10, eq119_e1531_d_b11, eq119_e1531_d_b12, eq119_e1531_d_b13, eq119_e1531_d_b14, eq119_e1531_d_b15, eq119_e1531_d_b16, eq119_e1531_d_b17, eq119_e1531_d_b18, eq119_e1531_d_b19, eq119_e1531_d_b20, eq119_e1531_d_b21, eq119_e1531_d_b22, eq119_e1531_d_b23, eq119_e1531_d_b24, eq119_e1531_d_b25, eq119_e1531_d_b26, eq119_e1531_d_b27, eq119_e1531_d_b28, eq119_e1531_d_b29, eq119_e1531_d_b30, eq119_e1531_d_b31, eq119_e1531_d_b32, eq119_e1531_d_b33, eq119_e1531_d_b34, eq119_e1531_d_b35, eq119_e1531_d_b36, eq119_e1531_d_b37, eq119_e1531_d_b38, eq119_e1531_d_b39, eq119_e1531_d_b40, eq119_e1531_d_b41, eq119_e1531_d_b42, eq119_e1531_d_b43, eq119_e1531_d_b44, eq119_e1531_d_b45, eq119_e1531_d_b46, eq119_e1531_d_b47, eq119_e1531_d_b48, eq119_e1531_d_b49, eq119_e1531_d_b50, eq119_e1531_d_b51, eq119_e1531_d_b52, eq119_e1531_d_b53, eq119_e1531_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq119_value),
            &eq119_node_derivatives,
            &eq119_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_16(
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[228][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[228][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[228][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[228][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[228][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[228][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[228][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[228][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[228][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[228][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[228][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[228][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[228][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[228][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[228][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[228][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[228][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[228][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[228][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[228][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[228][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[228][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[228][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[228][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[228][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[228][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[228][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[228][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[228][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[228][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[228][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[228][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[228][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[228][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[228][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[228][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[228][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[228][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[228][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[228][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[228][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[228][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[228][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[228][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[228][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[228][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[228][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[228][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[228][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[228][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[228][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[228][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[228][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[228][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[228][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[228][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[228][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[228][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[228][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[228][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[228][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[228][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[228][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[228][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[228][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[228][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[228][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[228][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[228][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[228][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[228][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[228][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[228][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[228][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[228][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[228][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[228][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[228][54] * ddt_scale));
        let (eq120_e1540, eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22, eq120_e1540_d_b0, eq120_e1540_d_b1, eq120_e1540_d_b2, eq120_e1540_d_b3, eq120_e1540_d_b4, eq120_e1540_d_b5, eq120_e1540_d_b6, eq120_e1540_d_b7, eq120_e1540_d_b8, eq120_e1540_d_b9, eq120_e1540_d_b10, eq120_e1540_d_b11, eq120_e1540_d_b12, eq120_e1540_d_b13, eq120_e1540_d_b14, eq120_e1540_d_b15, eq120_e1540_d_b16, eq120_e1540_d_b17, eq120_e1540_d_b18, eq120_e1540_d_b19, eq120_e1540_d_b20, eq120_e1540_d_b21, eq120_e1540_d_b22, eq120_e1540_d_b23, eq120_e1540_d_b24, eq120_e1540_d_b25, eq120_e1540_d_b26, eq120_e1540_d_b27, eq120_e1540_d_b28, eq120_e1540_d_b29, eq120_e1540_d_b30, eq120_e1540_d_b31, eq120_e1540_d_b32, eq120_e1540_d_b33, eq120_e1540_d_b34, eq120_e1540_d_b35, eq120_e1540_d_b36, eq120_e1540_d_b37, eq120_e1540_d_b38, eq120_e1540_d_b39, eq120_e1540_d_b40, eq120_e1540_d_b41, eq120_e1540_d_b42, eq120_e1540_d_b43, eq120_e1540_d_b44, eq120_e1540_d_b45, eq120_e1540_d_b46, eq120_e1540_d_b47, eq120_e1540_d_b48, eq120_e1540_d_b49, eq120_e1540_d_b50, eq120_e1540_d_b51, eq120_e1540_d_b52, eq120_e1540_d_b53, eq120_e1540_d_b54,) = {
    if (s.b[570] && s.b[571]) {
        let eq120_e1537: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, s.v[229]);
        let eq120_e1538: f64 = (p.p7 * eq120_e1537);
        let eq120_e1538_d_n0: f64 = (p.p7 * (s.dn[229][0] * ddt_scale));
        let eq120_e1538_d_n1: f64 = (p.p7 * (s.dn[229][1] * ddt_scale));
        let eq120_e1538_d_n2: f64 = (p.p7 * (s.dn[229][2] * ddt_scale));
        let eq120_e1538_d_n3: f64 = (p.p7 * (s.dn[229][3] * ddt_scale));
        let eq120_e1538_d_n4: f64 = (p.p7 * (s.dn[229][4] * ddt_scale));
        let eq120_e1538_d_n5: f64 = (p.p7 * (s.dn[229][5] * ddt_scale));
        let eq120_e1538_d_n6: f64 = (p.p7 * (s.dn[229][6] * ddt_scale));
        let eq120_e1538_d_n7: f64 = (p.p7 * (s.dn[229][7] * ddt_scale));
        let eq120_e1538_d_n8: f64 = (p.p7 * (s.dn[229][8] * ddt_scale));
        let eq120_e1538_d_n9: f64 = (p.p7 * (s.dn[229][9] * ddt_scale));
        let eq120_e1538_d_n10: f64 = (p.p7 * (s.dn[229][10] * ddt_scale));
        let eq120_e1538_d_n11: f64 = (p.p7 * (s.dn[229][11] * ddt_scale));
        let eq120_e1538_d_n12: f64 = (p.p7 * (s.dn[229][12] * ddt_scale));
        let eq120_e1538_d_n13: f64 = (p.p7 * (s.dn[229][13] * ddt_scale));
        let eq120_e1538_d_n14: f64 = (p.p7 * (s.dn[229][14] * ddt_scale));
        let eq120_e1538_d_n15: f64 = (p.p7 * (s.dn[229][15] * ddt_scale));
        let eq120_e1538_d_n16: f64 = (p.p7 * (s.dn[229][16] * ddt_scale));
        let eq120_e1538_d_n17: f64 = (p.p7 * (s.dn[229][17] * ddt_scale));
        let eq120_e1538_d_n18: f64 = (p.p7 * (s.dn[229][18] * ddt_scale));
        let eq120_e1538_d_n19: f64 = (p.p7 * (s.dn[229][19] * ddt_scale));
        let eq120_e1538_d_n20: f64 = (p.p7 * (s.dn[229][20] * ddt_scale));
        let eq120_e1538_d_n21: f64 = (p.p7 * (s.dn[229][21] * ddt_scale));
        let eq120_e1538_d_n22: f64 = (p.p7 * (s.dn[229][22] * ddt_scale));
        let eq120_e1538_d_b0: f64 = (p.p7 * (s.db[229][0] * ddt_scale));
        let eq120_e1538_d_b1: f64 = (p.p7 * (s.db[229][1] * ddt_scale));
        let eq120_e1538_d_b2: f64 = (p.p7 * (s.db[229][2] * ddt_scale));
        let eq120_e1538_d_b3: f64 = (p.p7 * (s.db[229][3] * ddt_scale));
        let eq120_e1538_d_b4: f64 = (p.p7 * (s.db[229][4] * ddt_scale));
        let eq120_e1538_d_b5: f64 = (p.p7 * (s.db[229][5] * ddt_scale));
        let eq120_e1538_d_b6: f64 = (p.p7 * (s.db[229][6] * ddt_scale));
        let eq120_e1538_d_b7: f64 = (p.p7 * (s.db[229][7] * ddt_scale));
        let eq120_e1538_d_b8: f64 = (p.p7 * (s.db[229][8] * ddt_scale));
        let eq120_e1538_d_b9: f64 = (p.p7 * (s.db[229][9] * ddt_scale));
        let eq120_e1538_d_b10: f64 = (p.p7 * (s.db[229][10] * ddt_scale));
        let eq120_e1538_d_b11: f64 = (p.p7 * (s.db[229][11] * ddt_scale));
        let eq120_e1538_d_b12: f64 = (p.p7 * (s.db[229][12] * ddt_scale));
        let eq120_e1538_d_b13: f64 = (p.p7 * (s.db[229][13] * ddt_scale));
        let eq120_e1538_d_b14: f64 = (p.p7 * (s.db[229][14] * ddt_scale));
        let eq120_e1538_d_b15: f64 = (p.p7 * (s.db[229][15] * ddt_scale));
        let eq120_e1538_d_b16: f64 = (p.p7 * (s.db[229][16] * ddt_scale));
        let eq120_e1538_d_b17: f64 = (p.p7 * (s.db[229][17] * ddt_scale));
        let eq120_e1538_d_b18: f64 = (p.p7 * (s.db[229][18] * ddt_scale));
        let eq120_e1538_d_b19: f64 = (p.p7 * (s.db[229][19] * ddt_scale));
        let eq120_e1538_d_b20: f64 = (p.p7 * (s.db[229][20] * ddt_scale));
        let eq120_e1538_d_b21: f64 = (p.p7 * (s.db[229][21] * ddt_scale));
        let eq120_e1538_d_b22: f64 = (p.p7 * (s.db[229][22] * ddt_scale));
        let eq120_e1538_d_b23: f64 = (p.p7 * (s.db[229][23] * ddt_scale));
        let eq120_e1538_d_b24: f64 = (p.p7 * (s.db[229][24] * ddt_scale));
        let eq120_e1538_d_b25: f64 = (p.p7 * (s.db[229][25] * ddt_scale));
        let eq120_e1538_d_b26: f64 = (p.p7 * (s.db[229][26] * ddt_scale));
        let eq120_e1538_d_b27: f64 = (p.p7 * (s.db[229][27] * ddt_scale));
        let eq120_e1538_d_b28: f64 = (p.p7 * (s.db[229][28] * ddt_scale));
        let eq120_e1538_d_b29: f64 = (p.p7 * (s.db[229][29] * ddt_scale));
        let eq120_e1538_d_b30: f64 = (p.p7 * (s.db[229][30] * ddt_scale));
        let eq120_e1538_d_b31: f64 = (p.p7 * (s.db[229][31] * ddt_scale));
        let eq120_e1538_d_b32: f64 = (p.p7 * (s.db[229][32] * ddt_scale));
        let eq120_e1538_d_b33: f64 = (p.p7 * (s.db[229][33] * ddt_scale));
        let eq120_e1538_d_b34: f64 = (p.p7 * (s.db[229][34] * ddt_scale));
        let eq120_e1538_d_b35: f64 = (p.p7 * (s.db[229][35] * ddt_scale));
        let eq120_e1538_d_b36: f64 = (p.p7 * (s.db[229][36] * ddt_scale));
        let eq120_e1538_d_b37: f64 = (p.p7 * (s.db[229][37] * ddt_scale));
        let eq120_e1538_d_b38: f64 = (p.p7 * (s.db[229][38] * ddt_scale));
        let eq120_e1538_d_b39: f64 = (p.p7 * (s.db[229][39] * ddt_scale));
        let eq120_e1538_d_b40: f64 = (p.p7 * (s.db[229][40] * ddt_scale));
        let eq120_e1538_d_b41: f64 = (p.p7 * (s.db[229][41] * ddt_scale));
        let eq120_e1538_d_b42: f64 = (p.p7 * (s.db[229][42] * ddt_scale));
        let eq120_e1538_d_b43: f64 = (p.p7 * (s.db[229][43] * ddt_scale));
        let eq120_e1538_d_b44: f64 = (p.p7 * (s.db[229][44] * ddt_scale));
        let eq120_e1538_d_b45: f64 = (p.p7 * (s.db[229][45] * ddt_scale));
        let eq120_e1538_d_b46: f64 = (p.p7 * (s.db[229][46] * ddt_scale));
        let eq120_e1538_d_b47: f64 = (p.p7 * (s.db[229][47] * ddt_scale));
        let eq120_e1538_d_b48: f64 = (p.p7 * (s.db[229][48] * ddt_scale));
        let eq120_e1538_d_b49: f64 = (p.p7 * (s.db[229][49] * ddt_scale));
        let eq120_e1538_d_b50: f64 = (p.p7 * (s.db[229][50] * ddt_scale));
        let eq120_e1538_d_b51: f64 = (p.p7 * (s.db[229][51] * ddt_scale));
        let eq120_e1538_d_b52: f64 = (p.p7 * (s.db[229][52] * ddt_scale));
        let eq120_e1538_d_b53: f64 = (p.p7 * (s.db[229][53] * ddt_scale));
        let eq120_e1538_d_b54: f64 = (p.p7 * (s.db[229][54] * ddt_scale));
        (eq120_e1538, eq120_e1538_d_n0, eq120_e1538_d_n1, eq120_e1538_d_n2, eq120_e1538_d_n3, eq120_e1538_d_n4, eq120_e1538_d_n5, eq120_e1538_d_n6, eq120_e1538_d_n7, eq120_e1538_d_n8, eq120_e1538_d_n9, eq120_e1538_d_n10, eq120_e1538_d_n11, eq120_e1538_d_n12, eq120_e1538_d_n13, eq120_e1538_d_n14, eq120_e1538_d_n15, eq120_e1538_d_n16, eq120_e1538_d_n17, eq120_e1538_d_n18, eq120_e1538_d_n19, eq120_e1538_d_n20, eq120_e1538_d_n21, eq120_e1538_d_n22, eq120_e1538_d_b0, eq120_e1538_d_b1, eq120_e1538_d_b2, eq120_e1538_d_b3, eq120_e1538_d_b4, eq120_e1538_d_b5, eq120_e1538_d_b6, eq120_e1538_d_b7, eq120_e1538_d_b8, eq120_e1538_d_b9, eq120_e1538_d_b10, eq120_e1538_d_b11, eq120_e1538_d_b12, eq120_e1538_d_b13, eq120_e1538_d_b14, eq120_e1538_d_b15, eq120_e1538_d_b16, eq120_e1538_d_b17, eq120_e1538_d_b18, eq120_e1538_d_b19, eq120_e1538_d_b20, eq120_e1538_d_b21, eq120_e1538_d_b22, eq120_e1538_d_b23, eq120_e1538_d_b24, eq120_e1538_d_b25, eq120_e1538_d_b26, eq120_e1538_d_b27, eq120_e1538_d_b28, eq120_e1538_d_b29, eq120_e1538_d_b30, eq120_e1538_d_b31, eq120_e1538_d_b32, eq120_e1538_d_b33, eq120_e1538_d_b34, eq120_e1538_d_b35, eq120_e1538_d_b36, eq120_e1538_d_b37, eq120_e1538_d_b38, eq120_e1538_d_b39, eq120_e1538_d_b40, eq120_e1538_d_b41, eq120_e1538_d_b42, eq120_e1538_d_b43, eq120_e1538_d_b44, eq120_e1538_d_b45, eq120_e1538_d_b46, eq120_e1538_d_b47, eq120_e1538_d_b48, eq120_e1538_d_b49, eq120_e1538_d_b50, eq120_e1538_d_b51, eq120_e1538_d_b52, eq120_e1538_d_b53, eq120_e1538_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq120_value: f64 = eq120_e1540;
        let eq120_node_derivatives: [f64; 23] = [eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22];
        let eq120_branch_derivatives: [f64; 55] = [eq120_e1540_d_b0, eq120_e1540_d_b1, eq120_e1540_d_b2, eq120_e1540_d_b3, eq120_e1540_d_b4, eq120_e1540_d_b5, eq120_e1540_d_b6, eq120_e1540_d_b7, eq120_e1540_d_b8, eq120_e1540_d_b9, eq120_e1540_d_b10, eq120_e1540_d_b11, eq120_e1540_d_b12, eq120_e1540_d_b13, eq120_e1540_d_b14, eq120_e1540_d_b15, eq120_e1540_d_b16, eq120_e1540_d_b17, eq120_e1540_d_b18, eq120_e1540_d_b19, eq120_e1540_d_b20, eq120_e1540_d_b21, eq120_e1540_d_b22, eq120_e1540_d_b23, eq120_e1540_d_b24, eq120_e1540_d_b25, eq120_e1540_d_b26, eq120_e1540_d_b27, eq120_e1540_d_b28, eq120_e1540_d_b29, eq120_e1540_d_b30, eq120_e1540_d_b31, eq120_e1540_d_b32, eq120_e1540_d_b33, eq120_e1540_d_b34, eq120_e1540_d_b35, eq120_e1540_d_b36, eq120_e1540_d_b37, eq120_e1540_d_b38, eq120_e1540_d_b39, eq120_e1540_d_b40, eq120_e1540_d_b41, eq120_e1540_d_b42, eq120_e1540_d_b43, eq120_e1540_d_b44, eq120_e1540_d_b45, eq120_e1540_d_b46, eq120_e1540_d_b47, eq120_e1540_d_b48, eq120_e1540_d_b49, eq120_e1540_d_b50, eq120_e1540_d_b51, eq120_e1540_d_b52, eq120_e1540_d_b53, eq120_e1540_d_b54];
        stamper.stamp_current_dense_local(
            Some(15),
            Some(7),
            multiplicity * (eq120_value),
            &eq120_node_derivatives,
            &eq120_branch_derivatives,
            multiplicity,
        );
        let (eq121_e1551, eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22, eq121_e1551_d_b0, eq121_e1551_d_b1, eq121_e1551_d_b2, eq121_e1551_d_b3, eq121_e1551_d_b4, eq121_e1551_d_b5, eq121_e1551_d_b6, eq121_e1551_d_b7, eq121_e1551_d_b8, eq121_e1551_d_b9, eq121_e1551_d_b10, eq121_e1551_d_b11, eq121_e1551_d_b12, eq121_e1551_d_b13, eq121_e1551_d_b14, eq121_e1551_d_b15, eq121_e1551_d_b16, eq121_e1551_d_b17, eq121_e1551_d_b18, eq121_e1551_d_b19, eq121_e1551_d_b20, eq121_e1551_d_b21, eq121_e1551_d_b22, eq121_e1551_d_b23, eq121_e1551_d_b24, eq121_e1551_d_b25, eq121_e1551_d_b26, eq121_e1551_d_b27, eq121_e1551_d_b28, eq121_e1551_d_b29, eq121_e1551_d_b30, eq121_e1551_d_b31, eq121_e1551_d_b32, eq121_e1551_d_b33, eq121_e1551_d_b34, eq121_e1551_d_b35, eq121_e1551_d_b36, eq121_e1551_d_b37, eq121_e1551_d_b38, eq121_e1551_d_b39, eq121_e1551_d_b40, eq121_e1551_d_b41, eq121_e1551_d_b42, eq121_e1551_d_b43, eq121_e1551_d_b44, eq121_e1551_d_b45, eq121_e1551_d_b46, eq121_e1551_d_b47, eq121_e1551_d_b48, eq121_e1551_d_b49, eq121_e1551_d_b50, eq121_e1551_d_b51, eq121_e1551_d_b52, eq121_e1551_d_b53, eq121_e1551_d_b54,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq121_e1548: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, s.v[228]);
        let eq121_e1549: f64 = (p.p7 * eq121_e1548);
        (eq121_e1549, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq121_value: f64 = eq121_e1551;
        let eq121_node_derivatives: [f64; 23] = [eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22];
        let eq121_branch_derivatives: [f64; 55] = [eq121_e1551_d_b0, eq121_e1551_d_b1, eq121_e1551_d_b2, eq121_e1551_d_b3, eq121_e1551_d_b4, eq121_e1551_d_b5, eq121_e1551_d_b6, eq121_e1551_d_b7, eq121_e1551_d_b8, eq121_e1551_d_b9, eq121_e1551_d_b10, eq121_e1551_d_b11, eq121_e1551_d_b12, eq121_e1551_d_b13, eq121_e1551_d_b14, eq121_e1551_d_b15, eq121_e1551_d_b16, eq121_e1551_d_b17, eq121_e1551_d_b18, eq121_e1551_d_b19, eq121_e1551_d_b20, eq121_e1551_d_b21, eq121_e1551_d_b22, eq121_e1551_d_b23, eq121_e1551_d_b24, eq121_e1551_d_b25, eq121_e1551_d_b26, eq121_e1551_d_b27, eq121_e1551_d_b28, eq121_e1551_d_b29, eq121_e1551_d_b30, eq121_e1551_d_b31, eq121_e1551_d_b32, eq121_e1551_d_b33, eq121_e1551_d_b34, eq121_e1551_d_b35, eq121_e1551_d_b36, eq121_e1551_d_b37, eq121_e1551_d_b38, eq121_e1551_d_b39, eq121_e1551_d_b40, eq121_e1551_d_b41, eq121_e1551_d_b42, eq121_e1551_d_b43, eq121_e1551_d_b44, eq121_e1551_d_b45, eq121_e1551_d_b46, eq121_e1551_d_b47, eq121_e1551_d_b48, eq121_e1551_d_b49, eq121_e1551_d_b50, eq121_e1551_d_b51, eq121_e1551_d_b52, eq121_e1551_d_b53, eq121_e1551_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq121_value),
            &eq121_node_derivatives,
            &eq121_branch_derivatives,
            multiplicity,
        );
        let (eq122_e1564, eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22, eq122_e1564_d_b0, eq122_e1564_d_b1, eq122_e1564_d_b2, eq122_e1564_d_b3, eq122_e1564_d_b4, eq122_e1564_d_b5, eq122_e1564_d_b6, eq122_e1564_d_b7, eq122_e1564_d_b8, eq122_e1564_d_b9, eq122_e1564_d_b10, eq122_e1564_d_b11, eq122_e1564_d_b12, eq122_e1564_d_b13, eq122_e1564_d_b14, eq122_e1564_d_b15, eq122_e1564_d_b16, eq122_e1564_d_b17, eq122_e1564_d_b18, eq122_e1564_d_b19, eq122_e1564_d_b20, eq122_e1564_d_b21, eq122_e1564_d_b22, eq122_e1564_d_b23, eq122_e1564_d_b24, eq122_e1564_d_b25, eq122_e1564_d_b26, eq122_e1564_d_b27, eq122_e1564_d_b28, eq122_e1564_d_b29, eq122_e1564_d_b30, eq122_e1564_d_b31, eq122_e1564_d_b32, eq122_e1564_d_b33, eq122_e1564_d_b34, eq122_e1564_d_b35, eq122_e1564_d_b36, eq122_e1564_d_b37, eq122_e1564_d_b38, eq122_e1564_d_b39, eq122_e1564_d_b40, eq122_e1564_d_b41, eq122_e1564_d_b42, eq122_e1564_d_b43, eq122_e1564_d_b44, eq122_e1564_d_b45, eq122_e1564_d_b46, eq122_e1564_d_b47, eq122_e1564_d_b48, eq122_e1564_d_b49, eq122_e1564_d_b50, eq122_e1564_d_b51, eq122_e1564_d_b52, eq122_e1564_d_b53, eq122_e1564_d_b54,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq122_e1559: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, s.v[228]);
        let eq122_e1560: f64 = (p.p7 * eq122_e1559);
        let eq122_e1562: f64 = (eq122_e1560 * p.p246);
        let eq122_e1562_d_n0: f64 = (__rspice_deriv_cse_0 * p.p246);
        let eq122_e1562_d_n1: f64 = (__rspice_deriv_cse_1 * p.p246);
        let eq122_e1562_d_n2: f64 = (__rspice_deriv_cse_2 * p.p246);
        let eq122_e1562_d_n3: f64 = (__rspice_deriv_cse_3 * p.p246);
        let eq122_e1562_d_n4: f64 = (__rspice_deriv_cse_4 * p.p246);
        let eq122_e1562_d_n5: f64 = (__rspice_deriv_cse_5 * p.p246);
        let eq122_e1562_d_n6: f64 = (__rspice_deriv_cse_6 * p.p246);
        let eq122_e1562_d_n7: f64 = (__rspice_deriv_cse_7 * p.p246);
        let eq122_e1562_d_n8: f64 = (__rspice_deriv_cse_8 * p.p246);
        let eq122_e1562_d_n9: f64 = (__rspice_deriv_cse_9 * p.p246);
        let eq122_e1562_d_n10: f64 = (__rspice_deriv_cse_10 * p.p246);
        let eq122_e1562_d_n11: f64 = (__rspice_deriv_cse_11 * p.p246);
        let eq122_e1562_d_n12: f64 = (__rspice_deriv_cse_12 * p.p246);
        let eq122_e1562_d_n13: f64 = (__rspice_deriv_cse_13 * p.p246);
        let eq122_e1562_d_n14: f64 = (__rspice_deriv_cse_14 * p.p246);
        let eq122_e1562_d_n15: f64 = (__rspice_deriv_cse_15 * p.p246);
        let eq122_e1562_d_n16: f64 = (__rspice_deriv_cse_16 * p.p246);
        let eq122_e1562_d_n17: f64 = (__rspice_deriv_cse_17 * p.p246);
        let eq122_e1562_d_n18: f64 = (__rspice_deriv_cse_18 * p.p246);
        let eq122_e1562_d_n19: f64 = (__rspice_deriv_cse_19 * p.p246);
        let eq122_e1562_d_n20: f64 = (__rspice_deriv_cse_20 * p.p246);
        let eq122_e1562_d_n21: f64 = (__rspice_deriv_cse_21 * p.p246);
        let eq122_e1562_d_n22: f64 = (__rspice_deriv_cse_22 * p.p246);
        let eq122_e1562_d_b0: f64 = (__rspice_deriv_cse_23 * p.p246);
        let eq122_e1562_d_b1: f64 = (__rspice_deriv_cse_24 * p.p246);
        let eq122_e1562_d_b2: f64 = (__rspice_deriv_cse_25 * p.p246);
        let eq122_e1562_d_b3: f64 = (__rspice_deriv_cse_26 * p.p246);
        let eq122_e1562_d_b4: f64 = (__rspice_deriv_cse_27 * p.p246);
        let eq122_e1562_d_b5: f64 = (__rspice_deriv_cse_28 * p.p246);
        let eq122_e1562_d_b6: f64 = (__rspice_deriv_cse_29 * p.p246);
        let eq122_e1562_d_b7: f64 = (__rspice_deriv_cse_30 * p.p246);
        let eq122_e1562_d_b8: f64 = (__rspice_deriv_cse_31 * p.p246);
        let eq122_e1562_d_b9: f64 = (__rspice_deriv_cse_32 * p.p246);
        let eq122_e1562_d_b10: f64 = (__rspice_deriv_cse_33 * p.p246);
        let eq122_e1562_d_b11: f64 = (__rspice_deriv_cse_34 * p.p246);
        let eq122_e1562_d_b12: f64 = (__rspice_deriv_cse_35 * p.p246);
        let eq122_e1562_d_b13: f64 = (__rspice_deriv_cse_36 * p.p246);
        let eq122_e1562_d_b14: f64 = (__rspice_deriv_cse_37 * p.p246);
        let eq122_e1562_d_b15: f64 = (__rspice_deriv_cse_38 * p.p246);
        let eq122_e1562_d_b16: f64 = (__rspice_deriv_cse_39 * p.p246);
        let eq122_e1562_d_b17: f64 = (__rspice_deriv_cse_40 * p.p246);
        let eq122_e1562_d_b18: f64 = (__rspice_deriv_cse_41 * p.p246);
        let eq122_e1562_d_b19: f64 = (__rspice_deriv_cse_42 * p.p246);
        let eq122_e1562_d_b20: f64 = (__rspice_deriv_cse_43 * p.p246);
        let eq122_e1562_d_b21: f64 = (__rspice_deriv_cse_44 * p.p246);
        let eq122_e1562_d_b22: f64 = (__rspice_deriv_cse_45 * p.p246);
        let eq122_e1562_d_b23: f64 = (__rspice_deriv_cse_46 * p.p246);
        let eq122_e1562_d_b24: f64 = (__rspice_deriv_cse_47 * p.p246);
        let eq122_e1562_d_b25: f64 = (__rspice_deriv_cse_48 * p.p246);
        let eq122_e1562_d_b26: f64 = (__rspice_deriv_cse_49 * p.p246);
        let eq122_e1562_d_b27: f64 = (__rspice_deriv_cse_50 * p.p246);
        let eq122_e1562_d_b28: f64 = (__rspice_deriv_cse_51 * p.p246);
        let eq122_e1562_d_b29: f64 = (__rspice_deriv_cse_52 * p.p246);
        let eq122_e1562_d_b30: f64 = (__rspice_deriv_cse_53 * p.p246);
        let eq122_e1562_d_b31: f64 = (__rspice_deriv_cse_54 * p.p246);
        let eq122_e1562_d_b32: f64 = (__rspice_deriv_cse_55 * p.p246);
        let eq122_e1562_d_b33: f64 = (__rspice_deriv_cse_56 * p.p246);
        let eq122_e1562_d_b34: f64 = (__rspice_deriv_cse_57 * p.p246);
        let eq122_e1562_d_b35: f64 = (__rspice_deriv_cse_58 * p.p246);
        let eq122_e1562_d_b36: f64 = (__rspice_deriv_cse_59 * p.p246);
        let eq122_e1562_d_b37: f64 = (__rspice_deriv_cse_60 * p.p246);
        let eq122_e1562_d_b38: f64 = (__rspice_deriv_cse_61 * p.p246);
        let eq122_e1562_d_b39: f64 = (__rspice_deriv_cse_62 * p.p246);
        let eq122_e1562_d_b40: f64 = (__rspice_deriv_cse_63 * p.p246);
        let eq122_e1562_d_b41: f64 = (__rspice_deriv_cse_64 * p.p246);
        let eq122_e1562_d_b42: f64 = (__rspice_deriv_cse_65 * p.p246);
        let eq122_e1562_d_b43: f64 = (__rspice_deriv_cse_66 * p.p246);
        let eq122_e1562_d_b44: f64 = (__rspice_deriv_cse_67 * p.p246);
        let eq122_e1562_d_b45: f64 = (__rspice_deriv_cse_68 * p.p246);
        let eq122_e1562_d_b46: f64 = (__rspice_deriv_cse_69 * p.p246);
        let eq122_e1562_d_b47: f64 = (__rspice_deriv_cse_70 * p.p246);
        let eq122_e1562_d_b48: f64 = (__rspice_deriv_cse_71 * p.p246);
        let eq122_e1562_d_b49: f64 = (__rspice_deriv_cse_72 * p.p246);
        let eq122_e1562_d_b50: f64 = (__rspice_deriv_cse_73 * p.p246);
        let eq122_e1562_d_b51: f64 = (__rspice_deriv_cse_74 * p.p246);
        let eq122_e1562_d_b52: f64 = (__rspice_deriv_cse_75 * p.p246);
        let eq122_e1562_d_b53: f64 = (__rspice_deriv_cse_76 * p.p246);
        let eq122_e1562_d_b54: f64 = (__rspice_deriv_cse_77 * p.p246);
        (eq122_e1562, eq122_e1562_d_n0, eq122_e1562_d_n1, eq122_e1562_d_n2, eq122_e1562_d_n3, eq122_e1562_d_n4, eq122_e1562_d_n5, eq122_e1562_d_n6, eq122_e1562_d_n7, eq122_e1562_d_n8, eq122_e1562_d_n9, eq122_e1562_d_n10, eq122_e1562_d_n11, eq122_e1562_d_n12, eq122_e1562_d_n13, eq122_e1562_d_n14, eq122_e1562_d_n15, eq122_e1562_d_n16, eq122_e1562_d_n17, eq122_e1562_d_n18, eq122_e1562_d_n19, eq122_e1562_d_n20, eq122_e1562_d_n21, eq122_e1562_d_n22, eq122_e1562_d_b0, eq122_e1562_d_b1, eq122_e1562_d_b2, eq122_e1562_d_b3, eq122_e1562_d_b4, eq122_e1562_d_b5, eq122_e1562_d_b6, eq122_e1562_d_b7, eq122_e1562_d_b8, eq122_e1562_d_b9, eq122_e1562_d_b10, eq122_e1562_d_b11, eq122_e1562_d_b12, eq122_e1562_d_b13, eq122_e1562_d_b14, eq122_e1562_d_b15, eq122_e1562_d_b16, eq122_e1562_d_b17, eq122_e1562_d_b18, eq122_e1562_d_b19, eq122_e1562_d_b20, eq122_e1562_d_b21, eq122_e1562_d_b22, eq122_e1562_d_b23, eq122_e1562_d_b24, eq122_e1562_d_b25, eq122_e1562_d_b26, eq122_e1562_d_b27, eq122_e1562_d_b28, eq122_e1562_d_b29, eq122_e1562_d_b30, eq122_e1562_d_b31, eq122_e1562_d_b32, eq122_e1562_d_b33, eq122_e1562_d_b34, eq122_e1562_d_b35, eq122_e1562_d_b36, eq122_e1562_d_b37, eq122_e1562_d_b38, eq122_e1562_d_b39, eq122_e1562_d_b40, eq122_e1562_d_b41, eq122_e1562_d_b42, eq122_e1562_d_b43, eq122_e1562_d_b44, eq122_e1562_d_b45, eq122_e1562_d_b46, eq122_e1562_d_b47, eq122_e1562_d_b48, eq122_e1562_d_b49, eq122_e1562_d_b50, eq122_e1562_d_b51, eq122_e1562_d_b52, eq122_e1562_d_b53, eq122_e1562_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_value: f64 = eq122_e1564;
        let eq122_node_derivatives: [f64; 23] = [eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22];
        let eq122_branch_derivatives: [f64; 55] = [eq122_e1564_d_b0, eq122_e1564_d_b1, eq122_e1564_d_b2, eq122_e1564_d_b3, eq122_e1564_d_b4, eq122_e1564_d_b5, eq122_e1564_d_b6, eq122_e1564_d_b7, eq122_e1564_d_b8, eq122_e1564_d_b9, eq122_e1564_d_b10, eq122_e1564_d_b11, eq122_e1564_d_b12, eq122_e1564_d_b13, eq122_e1564_d_b14, eq122_e1564_d_b15, eq122_e1564_d_b16, eq122_e1564_d_b17, eq122_e1564_d_b18, eq122_e1564_d_b19, eq122_e1564_d_b20, eq122_e1564_d_b21, eq122_e1564_d_b22, eq122_e1564_d_b23, eq122_e1564_d_b24, eq122_e1564_d_b25, eq122_e1564_d_b26, eq122_e1564_d_b27, eq122_e1564_d_b28, eq122_e1564_d_b29, eq122_e1564_d_b30, eq122_e1564_d_b31, eq122_e1564_d_b32, eq122_e1564_d_b33, eq122_e1564_d_b34, eq122_e1564_d_b35, eq122_e1564_d_b36, eq122_e1564_d_b37, eq122_e1564_d_b38, eq122_e1564_d_b39, eq122_e1564_d_b40, eq122_e1564_d_b41, eq122_e1564_d_b42, eq122_e1564_d_b43, eq122_e1564_d_b44, eq122_e1564_d_b45, eq122_e1564_d_b46, eq122_e1564_d_b47, eq122_e1564_d_b48, eq122_e1564_d_b49, eq122_e1564_d_b50, eq122_e1564_d_b51, eq122_e1564_d_b52, eq122_e1564_d_b53, eq122_e1564_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq122_value),
            &eq122_node_derivatives,
            &eq122_branch_derivatives,
            multiplicity,
        );
        let (eq123_e1576, eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22, eq123_e1576_d_b0, eq123_e1576_d_b1, eq123_e1576_d_b2, eq123_e1576_d_b3, eq123_e1576_d_b4, eq123_e1576_d_b5, eq123_e1576_d_b6, eq123_e1576_d_b7, eq123_e1576_d_b8, eq123_e1576_d_b9, eq123_e1576_d_b10, eq123_e1576_d_b11, eq123_e1576_d_b12, eq123_e1576_d_b13, eq123_e1576_d_b14, eq123_e1576_d_b15, eq123_e1576_d_b16, eq123_e1576_d_b17, eq123_e1576_d_b18, eq123_e1576_d_b19, eq123_e1576_d_b20, eq123_e1576_d_b21, eq123_e1576_d_b22, eq123_e1576_d_b23, eq123_e1576_d_b24, eq123_e1576_d_b25, eq123_e1576_d_b26, eq123_e1576_d_b27, eq123_e1576_d_b28, eq123_e1576_d_b29, eq123_e1576_d_b30, eq123_e1576_d_b31, eq123_e1576_d_b32, eq123_e1576_d_b33, eq123_e1576_d_b34, eq123_e1576_d_b35, eq123_e1576_d_b36, eq123_e1576_d_b37, eq123_e1576_d_b38, eq123_e1576_d_b39, eq123_e1576_d_b40, eq123_e1576_d_b41, eq123_e1576_d_b42, eq123_e1576_d_b43, eq123_e1576_d_b44, eq123_e1576_d_b45, eq123_e1576_d_b46, eq123_e1576_d_b47, eq123_e1576_d_b48, eq123_e1576_d_b49, eq123_e1576_d_b50, eq123_e1576_d_b51, eq123_e1576_d_b52, eq123_e1576_d_b53, eq123_e1576_d_b54,) = {
    if ((s.b[570] && s.b[571]) && (!s.b[572])) {
        let eq123_e1573: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, s.v[228]);
        let eq123_e1574: f64 = (p.p7 * eq123_e1573);
        (eq123_e1574, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq123_value: f64 = eq123_e1576;
        let eq123_node_derivatives: [f64; 23] = [eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22];
        let eq123_branch_derivatives: [f64; 55] = [eq123_e1576_d_b0, eq123_e1576_d_b1, eq123_e1576_d_b2, eq123_e1576_d_b3, eq123_e1576_d_b4, eq123_e1576_d_b5, eq123_e1576_d_b6, eq123_e1576_d_b7, eq123_e1576_d_b8, eq123_e1576_d_b9, eq123_e1576_d_b10, eq123_e1576_d_b11, eq123_e1576_d_b12, eq123_e1576_d_b13, eq123_e1576_d_b14, eq123_e1576_d_b15, eq123_e1576_d_b16, eq123_e1576_d_b17, eq123_e1576_d_b18, eq123_e1576_d_b19, eq123_e1576_d_b20, eq123_e1576_d_b21, eq123_e1576_d_b22, eq123_e1576_d_b23, eq123_e1576_d_b24, eq123_e1576_d_b25, eq123_e1576_d_b26, eq123_e1576_d_b27, eq123_e1576_d_b28, eq123_e1576_d_b29, eq123_e1576_d_b30, eq123_e1576_d_b31, eq123_e1576_d_b32, eq123_e1576_d_b33, eq123_e1576_d_b34, eq123_e1576_d_b35, eq123_e1576_d_b36, eq123_e1576_d_b37, eq123_e1576_d_b38, eq123_e1576_d_b39, eq123_e1576_d_b40, eq123_e1576_d_b41, eq123_e1576_d_b42, eq123_e1576_d_b43, eq123_e1576_d_b44, eq123_e1576_d_b45, eq123_e1576_d_b46, eq123_e1576_d_b47, eq123_e1576_d_b48, eq123_e1576_d_b49, eq123_e1576_d_b50, eq123_e1576_d_b51, eq123_e1576_d_b52, eq123_e1576_d_b53, eq123_e1576_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq123_value),
            &eq123_node_derivatives,
            &eq123_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_17(
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
        let (eq124_e1590, eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22, eq124_e1590_d_b0, eq124_e1590_d_b1, eq124_e1590_d_b2, eq124_e1590_d_b3, eq124_e1590_d_b4, eq124_e1590_d_b5, eq124_e1590_d_b6, eq124_e1590_d_b7, eq124_e1590_d_b8, eq124_e1590_d_b9, eq124_e1590_d_b10, eq124_e1590_d_b11, eq124_e1590_d_b12, eq124_e1590_d_b13, eq124_e1590_d_b14, eq124_e1590_d_b15, eq124_e1590_d_b16, eq124_e1590_d_b17, eq124_e1590_d_b18, eq124_e1590_d_b19, eq124_e1590_d_b20, eq124_e1590_d_b21, eq124_e1590_d_b22, eq124_e1590_d_b23, eq124_e1590_d_b24, eq124_e1590_d_b25, eq124_e1590_d_b26, eq124_e1590_d_b27, eq124_e1590_d_b28, eq124_e1590_d_b29, eq124_e1590_d_b30, eq124_e1590_d_b31, eq124_e1590_d_b32, eq124_e1590_d_b33, eq124_e1590_d_b34, eq124_e1590_d_b35, eq124_e1590_d_b36, eq124_e1590_d_b37, eq124_e1590_d_b38, eq124_e1590_d_b39, eq124_e1590_d_b40, eq124_e1590_d_b41, eq124_e1590_d_b42, eq124_e1590_d_b43, eq124_e1590_d_b44, eq124_e1590_d_b45, eq124_e1590_d_b46, eq124_e1590_d_b47, eq124_e1590_d_b48, eq124_e1590_d_b49, eq124_e1590_d_b50, eq124_e1590_d_b51, eq124_e1590_d_b52, eq124_e1590_d_b53, eq124_e1590_d_b54,) = {
    if ((s.b[570] && s.b[571]) && (!s.b[572])) {
        let eq124_e1585: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, s.v[228]);
        let eq124_e1586: f64 = (p.p7 * eq124_e1585);
        let eq124_e1586_d_n0: f64 = (p.p7 * (s.dn[228][0] * ddt_scale));
        let eq124_e1586_d_n1: f64 = (p.p7 * (s.dn[228][1] * ddt_scale));
        let eq124_e1586_d_n2: f64 = (p.p7 * (s.dn[228][2] * ddt_scale));
        let eq124_e1586_d_n3: f64 = (p.p7 * (s.dn[228][3] * ddt_scale));
        let eq124_e1586_d_n4: f64 = (p.p7 * (s.dn[228][4] * ddt_scale));
        let eq124_e1586_d_n5: f64 = (p.p7 * (s.dn[228][5] * ddt_scale));
        let eq124_e1586_d_n6: f64 = (p.p7 * (s.dn[228][6] * ddt_scale));
        let eq124_e1586_d_n7: f64 = (p.p7 * (s.dn[228][7] * ddt_scale));
        let eq124_e1586_d_n8: f64 = (p.p7 * (s.dn[228][8] * ddt_scale));
        let eq124_e1586_d_n9: f64 = (p.p7 * (s.dn[228][9] * ddt_scale));
        let eq124_e1586_d_n10: f64 = (p.p7 * (s.dn[228][10] * ddt_scale));
        let eq124_e1586_d_n11: f64 = (p.p7 * (s.dn[228][11] * ddt_scale));
        let eq124_e1586_d_n12: f64 = (p.p7 * (s.dn[228][12] * ddt_scale));
        let eq124_e1586_d_n13: f64 = (p.p7 * (s.dn[228][13] * ddt_scale));
        let eq124_e1586_d_n14: f64 = (p.p7 * (s.dn[228][14] * ddt_scale));
        let eq124_e1586_d_n15: f64 = (p.p7 * (s.dn[228][15] * ddt_scale));
        let eq124_e1586_d_n16: f64 = (p.p7 * (s.dn[228][16] * ddt_scale));
        let eq124_e1586_d_n17: f64 = (p.p7 * (s.dn[228][17] * ddt_scale));
        let eq124_e1586_d_n18: f64 = (p.p7 * (s.dn[228][18] * ddt_scale));
        let eq124_e1586_d_n19: f64 = (p.p7 * (s.dn[228][19] * ddt_scale));
        let eq124_e1586_d_n20: f64 = (p.p7 * (s.dn[228][20] * ddt_scale));
        let eq124_e1586_d_n21: f64 = (p.p7 * (s.dn[228][21] * ddt_scale));
        let eq124_e1586_d_n22: f64 = (p.p7 * (s.dn[228][22] * ddt_scale));
        let eq124_e1586_d_b0: f64 = (p.p7 * (s.db[228][0] * ddt_scale));
        let eq124_e1586_d_b1: f64 = (p.p7 * (s.db[228][1] * ddt_scale));
        let eq124_e1586_d_b2: f64 = (p.p7 * (s.db[228][2] * ddt_scale));
        let eq124_e1586_d_b3: f64 = (p.p7 * (s.db[228][3] * ddt_scale));
        let eq124_e1586_d_b4: f64 = (p.p7 * (s.db[228][4] * ddt_scale));
        let eq124_e1586_d_b5: f64 = (p.p7 * (s.db[228][5] * ddt_scale));
        let eq124_e1586_d_b6: f64 = (p.p7 * (s.db[228][6] * ddt_scale));
        let eq124_e1586_d_b7: f64 = (p.p7 * (s.db[228][7] * ddt_scale));
        let eq124_e1586_d_b8: f64 = (p.p7 * (s.db[228][8] * ddt_scale));
        let eq124_e1586_d_b9: f64 = (p.p7 * (s.db[228][9] * ddt_scale));
        let eq124_e1586_d_b10: f64 = (p.p7 * (s.db[228][10] * ddt_scale));
        let eq124_e1586_d_b11: f64 = (p.p7 * (s.db[228][11] * ddt_scale));
        let eq124_e1586_d_b12: f64 = (p.p7 * (s.db[228][12] * ddt_scale));
        let eq124_e1586_d_b13: f64 = (p.p7 * (s.db[228][13] * ddt_scale));
        let eq124_e1586_d_b14: f64 = (p.p7 * (s.db[228][14] * ddt_scale));
        let eq124_e1586_d_b15: f64 = (p.p7 * (s.db[228][15] * ddt_scale));
        let eq124_e1586_d_b16: f64 = (p.p7 * (s.db[228][16] * ddt_scale));
        let eq124_e1586_d_b17: f64 = (p.p7 * (s.db[228][17] * ddt_scale));
        let eq124_e1586_d_b18: f64 = (p.p7 * (s.db[228][18] * ddt_scale));
        let eq124_e1586_d_b19: f64 = (p.p7 * (s.db[228][19] * ddt_scale));
        let eq124_e1586_d_b20: f64 = (p.p7 * (s.db[228][20] * ddt_scale));
        let eq124_e1586_d_b21: f64 = (p.p7 * (s.db[228][21] * ddt_scale));
        let eq124_e1586_d_b22: f64 = (p.p7 * (s.db[228][22] * ddt_scale));
        let eq124_e1586_d_b23: f64 = (p.p7 * (s.db[228][23] * ddt_scale));
        let eq124_e1586_d_b24: f64 = (p.p7 * (s.db[228][24] * ddt_scale));
        let eq124_e1586_d_b25: f64 = (p.p7 * (s.db[228][25] * ddt_scale));
        let eq124_e1586_d_b26: f64 = (p.p7 * (s.db[228][26] * ddt_scale));
        let eq124_e1586_d_b27: f64 = (p.p7 * (s.db[228][27] * ddt_scale));
        let eq124_e1586_d_b28: f64 = (p.p7 * (s.db[228][28] * ddt_scale));
        let eq124_e1586_d_b29: f64 = (p.p7 * (s.db[228][29] * ddt_scale));
        let eq124_e1586_d_b30: f64 = (p.p7 * (s.db[228][30] * ddt_scale));
        let eq124_e1586_d_b31: f64 = (p.p7 * (s.db[228][31] * ddt_scale));
        let eq124_e1586_d_b32: f64 = (p.p7 * (s.db[228][32] * ddt_scale));
        let eq124_e1586_d_b33: f64 = (p.p7 * (s.db[228][33] * ddt_scale));
        let eq124_e1586_d_b34: f64 = (p.p7 * (s.db[228][34] * ddt_scale));
        let eq124_e1586_d_b35: f64 = (p.p7 * (s.db[228][35] * ddt_scale));
        let eq124_e1586_d_b36: f64 = (p.p7 * (s.db[228][36] * ddt_scale));
        let eq124_e1586_d_b37: f64 = (p.p7 * (s.db[228][37] * ddt_scale));
        let eq124_e1586_d_b38: f64 = (p.p7 * (s.db[228][38] * ddt_scale));
        let eq124_e1586_d_b39: f64 = (p.p7 * (s.db[228][39] * ddt_scale));
        let eq124_e1586_d_b40: f64 = (p.p7 * (s.db[228][40] * ddt_scale));
        let eq124_e1586_d_b41: f64 = (p.p7 * (s.db[228][41] * ddt_scale));
        let eq124_e1586_d_b42: f64 = (p.p7 * (s.db[228][42] * ddt_scale));
        let eq124_e1586_d_b43: f64 = (p.p7 * (s.db[228][43] * ddt_scale));
        let eq124_e1586_d_b44: f64 = (p.p7 * (s.db[228][44] * ddt_scale));
        let eq124_e1586_d_b45: f64 = (p.p7 * (s.db[228][45] * ddt_scale));
        let eq124_e1586_d_b46: f64 = (p.p7 * (s.db[228][46] * ddt_scale));
        let eq124_e1586_d_b47: f64 = (p.p7 * (s.db[228][47] * ddt_scale));
        let eq124_e1586_d_b48: f64 = (p.p7 * (s.db[228][48] * ddt_scale));
        let eq124_e1586_d_b49: f64 = (p.p7 * (s.db[228][49] * ddt_scale));
        let eq124_e1586_d_b50: f64 = (p.p7 * (s.db[228][50] * ddt_scale));
        let eq124_e1586_d_b51: f64 = (p.p7 * (s.db[228][51] * ddt_scale));
        let eq124_e1586_d_b52: f64 = (p.p7 * (s.db[228][52] * ddt_scale));
        let eq124_e1586_d_b53: f64 = (p.p7 * (s.db[228][53] * ddt_scale));
        let eq124_e1586_d_b54: f64 = (p.p7 * (s.db[228][54] * ddt_scale));
        let eq124_e1588: f64 = (eq124_e1586 * p.p246);
        let eq124_e1588_d_n0: f64 = (eq124_e1586_d_n0 * p.p246);
        let eq124_e1588_d_n1: f64 = (eq124_e1586_d_n1 * p.p246);
        let eq124_e1588_d_n2: f64 = (eq124_e1586_d_n2 * p.p246);
        let eq124_e1588_d_n3: f64 = (eq124_e1586_d_n3 * p.p246);
        let eq124_e1588_d_n4: f64 = (eq124_e1586_d_n4 * p.p246);
        let eq124_e1588_d_n5: f64 = (eq124_e1586_d_n5 * p.p246);
        let eq124_e1588_d_n6: f64 = (eq124_e1586_d_n6 * p.p246);
        let eq124_e1588_d_n7: f64 = (eq124_e1586_d_n7 * p.p246);
        let eq124_e1588_d_n8: f64 = (eq124_e1586_d_n8 * p.p246);
        let eq124_e1588_d_n9: f64 = (eq124_e1586_d_n9 * p.p246);
        let eq124_e1588_d_n10: f64 = (eq124_e1586_d_n10 * p.p246);
        let eq124_e1588_d_n11: f64 = (eq124_e1586_d_n11 * p.p246);
        let eq124_e1588_d_n12: f64 = (eq124_e1586_d_n12 * p.p246);
        let eq124_e1588_d_n13: f64 = (eq124_e1586_d_n13 * p.p246);
        let eq124_e1588_d_n14: f64 = (eq124_e1586_d_n14 * p.p246);
        let eq124_e1588_d_n15: f64 = (eq124_e1586_d_n15 * p.p246);
        let eq124_e1588_d_n16: f64 = (eq124_e1586_d_n16 * p.p246);
        let eq124_e1588_d_n17: f64 = (eq124_e1586_d_n17 * p.p246);
        let eq124_e1588_d_n18: f64 = (eq124_e1586_d_n18 * p.p246);
        let eq124_e1588_d_n19: f64 = (eq124_e1586_d_n19 * p.p246);
        let eq124_e1588_d_n20: f64 = (eq124_e1586_d_n20 * p.p246);
        let eq124_e1588_d_n21: f64 = (eq124_e1586_d_n21 * p.p246);
        let eq124_e1588_d_n22: f64 = (eq124_e1586_d_n22 * p.p246);
        let eq124_e1588_d_b0: f64 = (eq124_e1586_d_b0 * p.p246);
        let eq124_e1588_d_b1: f64 = (eq124_e1586_d_b1 * p.p246);
        let eq124_e1588_d_b2: f64 = (eq124_e1586_d_b2 * p.p246);
        let eq124_e1588_d_b3: f64 = (eq124_e1586_d_b3 * p.p246);
        let eq124_e1588_d_b4: f64 = (eq124_e1586_d_b4 * p.p246);
        let eq124_e1588_d_b5: f64 = (eq124_e1586_d_b5 * p.p246);
        let eq124_e1588_d_b6: f64 = (eq124_e1586_d_b6 * p.p246);
        let eq124_e1588_d_b7: f64 = (eq124_e1586_d_b7 * p.p246);
        let eq124_e1588_d_b8: f64 = (eq124_e1586_d_b8 * p.p246);
        let eq124_e1588_d_b9: f64 = (eq124_e1586_d_b9 * p.p246);
        let eq124_e1588_d_b10: f64 = (eq124_e1586_d_b10 * p.p246);
        let eq124_e1588_d_b11: f64 = (eq124_e1586_d_b11 * p.p246);
        let eq124_e1588_d_b12: f64 = (eq124_e1586_d_b12 * p.p246);
        let eq124_e1588_d_b13: f64 = (eq124_e1586_d_b13 * p.p246);
        let eq124_e1588_d_b14: f64 = (eq124_e1586_d_b14 * p.p246);
        let eq124_e1588_d_b15: f64 = (eq124_e1586_d_b15 * p.p246);
        let eq124_e1588_d_b16: f64 = (eq124_e1586_d_b16 * p.p246);
        let eq124_e1588_d_b17: f64 = (eq124_e1586_d_b17 * p.p246);
        let eq124_e1588_d_b18: f64 = (eq124_e1586_d_b18 * p.p246);
        let eq124_e1588_d_b19: f64 = (eq124_e1586_d_b19 * p.p246);
        let eq124_e1588_d_b20: f64 = (eq124_e1586_d_b20 * p.p246);
        let eq124_e1588_d_b21: f64 = (eq124_e1586_d_b21 * p.p246);
        let eq124_e1588_d_b22: f64 = (eq124_e1586_d_b22 * p.p246);
        let eq124_e1588_d_b23: f64 = (eq124_e1586_d_b23 * p.p246);
        let eq124_e1588_d_b24: f64 = (eq124_e1586_d_b24 * p.p246);
        let eq124_e1588_d_b25: f64 = (eq124_e1586_d_b25 * p.p246);
        let eq124_e1588_d_b26: f64 = (eq124_e1586_d_b26 * p.p246);
        let eq124_e1588_d_b27: f64 = (eq124_e1586_d_b27 * p.p246);
        let eq124_e1588_d_b28: f64 = (eq124_e1586_d_b28 * p.p246);
        let eq124_e1588_d_b29: f64 = (eq124_e1586_d_b29 * p.p246);
        let eq124_e1588_d_b30: f64 = (eq124_e1586_d_b30 * p.p246);
        let eq124_e1588_d_b31: f64 = (eq124_e1586_d_b31 * p.p246);
        let eq124_e1588_d_b32: f64 = (eq124_e1586_d_b32 * p.p246);
        let eq124_e1588_d_b33: f64 = (eq124_e1586_d_b33 * p.p246);
        let eq124_e1588_d_b34: f64 = (eq124_e1586_d_b34 * p.p246);
        let eq124_e1588_d_b35: f64 = (eq124_e1586_d_b35 * p.p246);
        let eq124_e1588_d_b36: f64 = (eq124_e1586_d_b36 * p.p246);
        let eq124_e1588_d_b37: f64 = (eq124_e1586_d_b37 * p.p246);
        let eq124_e1588_d_b38: f64 = (eq124_e1586_d_b38 * p.p246);
        let eq124_e1588_d_b39: f64 = (eq124_e1586_d_b39 * p.p246);
        let eq124_e1588_d_b40: f64 = (eq124_e1586_d_b40 * p.p246);
        let eq124_e1588_d_b41: f64 = (eq124_e1586_d_b41 * p.p246);
        let eq124_e1588_d_b42: f64 = (eq124_e1586_d_b42 * p.p246);
        let eq124_e1588_d_b43: f64 = (eq124_e1586_d_b43 * p.p246);
        let eq124_e1588_d_b44: f64 = (eq124_e1586_d_b44 * p.p246);
        let eq124_e1588_d_b45: f64 = (eq124_e1586_d_b45 * p.p246);
        let eq124_e1588_d_b46: f64 = (eq124_e1586_d_b46 * p.p246);
        let eq124_e1588_d_b47: f64 = (eq124_e1586_d_b47 * p.p246);
        let eq124_e1588_d_b48: f64 = (eq124_e1586_d_b48 * p.p246);
        let eq124_e1588_d_b49: f64 = (eq124_e1586_d_b49 * p.p246);
        let eq124_e1588_d_b50: f64 = (eq124_e1586_d_b50 * p.p246);
        let eq124_e1588_d_b51: f64 = (eq124_e1586_d_b51 * p.p246);
        let eq124_e1588_d_b52: f64 = (eq124_e1586_d_b52 * p.p246);
        let eq124_e1588_d_b53: f64 = (eq124_e1586_d_b53 * p.p246);
        let eq124_e1588_d_b54: f64 = (eq124_e1586_d_b54 * p.p246);
        (eq124_e1588, eq124_e1588_d_n0, eq124_e1588_d_n1, eq124_e1588_d_n2, eq124_e1588_d_n3, eq124_e1588_d_n4, eq124_e1588_d_n5, eq124_e1588_d_n6, eq124_e1588_d_n7, eq124_e1588_d_n8, eq124_e1588_d_n9, eq124_e1588_d_n10, eq124_e1588_d_n11, eq124_e1588_d_n12, eq124_e1588_d_n13, eq124_e1588_d_n14, eq124_e1588_d_n15, eq124_e1588_d_n16, eq124_e1588_d_n17, eq124_e1588_d_n18, eq124_e1588_d_n19, eq124_e1588_d_n20, eq124_e1588_d_n21, eq124_e1588_d_n22, eq124_e1588_d_b0, eq124_e1588_d_b1, eq124_e1588_d_b2, eq124_e1588_d_b3, eq124_e1588_d_b4, eq124_e1588_d_b5, eq124_e1588_d_b6, eq124_e1588_d_b7, eq124_e1588_d_b8, eq124_e1588_d_b9, eq124_e1588_d_b10, eq124_e1588_d_b11, eq124_e1588_d_b12, eq124_e1588_d_b13, eq124_e1588_d_b14, eq124_e1588_d_b15, eq124_e1588_d_b16, eq124_e1588_d_b17, eq124_e1588_d_b18, eq124_e1588_d_b19, eq124_e1588_d_b20, eq124_e1588_d_b21, eq124_e1588_d_b22, eq124_e1588_d_b23, eq124_e1588_d_b24, eq124_e1588_d_b25, eq124_e1588_d_b26, eq124_e1588_d_b27, eq124_e1588_d_b28, eq124_e1588_d_b29, eq124_e1588_d_b30, eq124_e1588_d_b31, eq124_e1588_d_b32, eq124_e1588_d_b33, eq124_e1588_d_b34, eq124_e1588_d_b35, eq124_e1588_d_b36, eq124_e1588_d_b37, eq124_e1588_d_b38, eq124_e1588_d_b39, eq124_e1588_d_b40, eq124_e1588_d_b41, eq124_e1588_d_b42, eq124_e1588_d_b43, eq124_e1588_d_b44, eq124_e1588_d_b45, eq124_e1588_d_b46, eq124_e1588_d_b47, eq124_e1588_d_b48, eq124_e1588_d_b49, eq124_e1588_d_b50, eq124_e1588_d_b51, eq124_e1588_d_b52, eq124_e1588_d_b53, eq124_e1588_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_value: f64 = eq124_e1590;
        let eq124_node_derivatives: [f64; 23] = [eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22];
        let eq124_branch_derivatives: [f64; 55] = [eq124_e1590_d_b0, eq124_e1590_d_b1, eq124_e1590_d_b2, eq124_e1590_d_b3, eq124_e1590_d_b4, eq124_e1590_d_b5, eq124_e1590_d_b6, eq124_e1590_d_b7, eq124_e1590_d_b8, eq124_e1590_d_b9, eq124_e1590_d_b10, eq124_e1590_d_b11, eq124_e1590_d_b12, eq124_e1590_d_b13, eq124_e1590_d_b14, eq124_e1590_d_b15, eq124_e1590_d_b16, eq124_e1590_d_b17, eq124_e1590_d_b18, eq124_e1590_d_b19, eq124_e1590_d_b20, eq124_e1590_d_b21, eq124_e1590_d_b22, eq124_e1590_d_b23, eq124_e1590_d_b24, eq124_e1590_d_b25, eq124_e1590_d_b26, eq124_e1590_d_b27, eq124_e1590_d_b28, eq124_e1590_d_b29, eq124_e1590_d_b30, eq124_e1590_d_b31, eq124_e1590_d_b32, eq124_e1590_d_b33, eq124_e1590_d_b34, eq124_e1590_d_b35, eq124_e1590_d_b36, eq124_e1590_d_b37, eq124_e1590_d_b38, eq124_e1590_d_b39, eq124_e1590_d_b40, eq124_e1590_d_b41, eq124_e1590_d_b42, eq124_e1590_d_b43, eq124_e1590_d_b44, eq124_e1590_d_b45, eq124_e1590_d_b46, eq124_e1590_d_b47, eq124_e1590_d_b48, eq124_e1590_d_b49, eq124_e1590_d_b50, eq124_e1590_d_b51, eq124_e1590_d_b52, eq124_e1590_d_b53, eq124_e1590_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq124_value),
            &eq124_node_derivatives,
            &eq124_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1601, eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22, eq125_e1601_d_b0, eq125_e1601_d_b1, eq125_e1601_d_b2, eq125_e1601_d_b3, eq125_e1601_d_b4, eq125_e1601_d_b5, eq125_e1601_d_b6, eq125_e1601_d_b7, eq125_e1601_d_b8, eq125_e1601_d_b9, eq125_e1601_d_b10, eq125_e1601_d_b11, eq125_e1601_d_b12, eq125_e1601_d_b13, eq125_e1601_d_b14, eq125_e1601_d_b15, eq125_e1601_d_b16, eq125_e1601_d_b17, eq125_e1601_d_b18, eq125_e1601_d_b19, eq125_e1601_d_b20, eq125_e1601_d_b21, eq125_e1601_d_b22, eq125_e1601_d_b23, eq125_e1601_d_b24, eq125_e1601_d_b25, eq125_e1601_d_b26, eq125_e1601_d_b27, eq125_e1601_d_b28, eq125_e1601_d_b29, eq125_e1601_d_b30, eq125_e1601_d_b31, eq125_e1601_d_b32, eq125_e1601_d_b33, eq125_e1601_d_b34, eq125_e1601_d_b35, eq125_e1601_d_b36, eq125_e1601_d_b37, eq125_e1601_d_b38, eq125_e1601_d_b39, eq125_e1601_d_b40, eq125_e1601_d_b41, eq125_e1601_d_b42, eq125_e1601_d_b43, eq125_e1601_d_b44, eq125_e1601_d_b45, eq125_e1601_d_b46, eq125_e1601_d_b47, eq125_e1601_d_b48, eq125_e1601_d_b49, eq125_e1601_d_b50, eq125_e1601_d_b51, eq125_e1601_d_b52, eq125_e1601_d_b53, eq125_e1601_d_b54,) = {
    if (s.b[570] && s.b[571]) {
        let eq125_e1597: f64 = (p.p251 * s.v[228]);
        let eq125_e1598: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 24, eq125_e1597);
        let eq125_e1598_d_n0: f64 = ((p.p251 * s.dn[228][0]) * ddt_scale);
        let eq125_e1598_d_n1: f64 = ((p.p251 * s.dn[228][1]) * ddt_scale);
        let eq125_e1598_d_n2: f64 = ((p.p251 * s.dn[228][2]) * ddt_scale);
        let eq125_e1598_d_n3: f64 = ((p.p251 * s.dn[228][3]) * ddt_scale);
        let eq125_e1598_d_n4: f64 = ((p.p251 * s.dn[228][4]) * ddt_scale);
        let eq125_e1598_d_n5: f64 = ((p.p251 * s.dn[228][5]) * ddt_scale);
        let eq125_e1598_d_n6: f64 = ((p.p251 * s.dn[228][6]) * ddt_scale);
        let eq125_e1598_d_n7: f64 = ((p.p251 * s.dn[228][7]) * ddt_scale);
        let eq125_e1598_d_n8: f64 = ((p.p251 * s.dn[228][8]) * ddt_scale);
        let eq125_e1598_d_n9: f64 = ((p.p251 * s.dn[228][9]) * ddt_scale);
        let eq125_e1598_d_n10: f64 = ((p.p251 * s.dn[228][10]) * ddt_scale);
        let eq125_e1598_d_n11: f64 = ((p.p251 * s.dn[228][11]) * ddt_scale);
        let eq125_e1598_d_n12: f64 = ((p.p251 * s.dn[228][12]) * ddt_scale);
        let eq125_e1598_d_n13: f64 = ((p.p251 * s.dn[228][13]) * ddt_scale);
        let eq125_e1598_d_n14: f64 = ((p.p251 * s.dn[228][14]) * ddt_scale);
        let eq125_e1598_d_n15: f64 = ((p.p251 * s.dn[228][15]) * ddt_scale);
        let eq125_e1598_d_n16: f64 = ((p.p251 * s.dn[228][16]) * ddt_scale);
        let eq125_e1598_d_n17: f64 = ((p.p251 * s.dn[228][17]) * ddt_scale);
        let eq125_e1598_d_n18: f64 = ((p.p251 * s.dn[228][18]) * ddt_scale);
        let eq125_e1598_d_n19: f64 = ((p.p251 * s.dn[228][19]) * ddt_scale);
        let eq125_e1598_d_n20: f64 = ((p.p251 * s.dn[228][20]) * ddt_scale);
        let eq125_e1598_d_n21: f64 = ((p.p251 * s.dn[228][21]) * ddt_scale);
        let eq125_e1598_d_n22: f64 = ((p.p251 * s.dn[228][22]) * ddt_scale);
        let eq125_e1598_d_b0: f64 = ((p.p251 * s.db[228][0]) * ddt_scale);
        let eq125_e1598_d_b1: f64 = ((p.p251 * s.db[228][1]) * ddt_scale);
        let eq125_e1598_d_b2: f64 = ((p.p251 * s.db[228][2]) * ddt_scale);
        let eq125_e1598_d_b3: f64 = ((p.p251 * s.db[228][3]) * ddt_scale);
        let eq125_e1598_d_b4: f64 = ((p.p251 * s.db[228][4]) * ddt_scale);
        let eq125_e1598_d_b5: f64 = ((p.p251 * s.db[228][5]) * ddt_scale);
        let eq125_e1598_d_b6: f64 = ((p.p251 * s.db[228][6]) * ddt_scale);
        let eq125_e1598_d_b7: f64 = ((p.p251 * s.db[228][7]) * ddt_scale);
        let eq125_e1598_d_b8: f64 = ((p.p251 * s.db[228][8]) * ddt_scale);
        let eq125_e1598_d_b9: f64 = ((p.p251 * s.db[228][9]) * ddt_scale);
        let eq125_e1598_d_b10: f64 = ((p.p251 * s.db[228][10]) * ddt_scale);
        let eq125_e1598_d_b11: f64 = ((p.p251 * s.db[228][11]) * ddt_scale);
        let eq125_e1598_d_b12: f64 = ((p.p251 * s.db[228][12]) * ddt_scale);
        let eq125_e1598_d_b13: f64 = ((p.p251 * s.db[228][13]) * ddt_scale);
        let eq125_e1598_d_b14: f64 = ((p.p251 * s.db[228][14]) * ddt_scale);
        let eq125_e1598_d_b15: f64 = ((p.p251 * s.db[228][15]) * ddt_scale);
        let eq125_e1598_d_b16: f64 = ((p.p251 * s.db[228][16]) * ddt_scale);
        let eq125_e1598_d_b17: f64 = ((p.p251 * s.db[228][17]) * ddt_scale);
        let eq125_e1598_d_b18: f64 = ((p.p251 * s.db[228][18]) * ddt_scale);
        let eq125_e1598_d_b19: f64 = ((p.p251 * s.db[228][19]) * ddt_scale);
        let eq125_e1598_d_b20: f64 = ((p.p251 * s.db[228][20]) * ddt_scale);
        let eq125_e1598_d_b21: f64 = ((p.p251 * s.db[228][21]) * ddt_scale);
        let eq125_e1598_d_b22: f64 = ((p.p251 * s.db[228][22]) * ddt_scale);
        let eq125_e1598_d_b23: f64 = ((p.p251 * s.db[228][23]) * ddt_scale);
        let eq125_e1598_d_b24: f64 = ((p.p251 * s.db[228][24]) * ddt_scale);
        let eq125_e1598_d_b25: f64 = ((p.p251 * s.db[228][25]) * ddt_scale);
        let eq125_e1598_d_b26: f64 = ((p.p251 * s.db[228][26]) * ddt_scale);
        let eq125_e1598_d_b27: f64 = ((p.p251 * s.db[228][27]) * ddt_scale);
        let eq125_e1598_d_b28: f64 = ((p.p251 * s.db[228][28]) * ddt_scale);
        let eq125_e1598_d_b29: f64 = ((p.p251 * s.db[228][29]) * ddt_scale);
        let eq125_e1598_d_b30: f64 = ((p.p251 * s.db[228][30]) * ddt_scale);
        let eq125_e1598_d_b31: f64 = ((p.p251 * s.db[228][31]) * ddt_scale);
        let eq125_e1598_d_b32: f64 = ((p.p251 * s.db[228][32]) * ddt_scale);
        let eq125_e1598_d_b33: f64 = ((p.p251 * s.db[228][33]) * ddt_scale);
        let eq125_e1598_d_b34: f64 = ((p.p251 * s.db[228][34]) * ddt_scale);
        let eq125_e1598_d_b35: f64 = ((p.p251 * s.db[228][35]) * ddt_scale);
        let eq125_e1598_d_b36: f64 = ((p.p251 * s.db[228][36]) * ddt_scale);
        let eq125_e1598_d_b37: f64 = ((p.p251 * s.db[228][37]) * ddt_scale);
        let eq125_e1598_d_b38: f64 = ((p.p251 * s.db[228][38]) * ddt_scale);
        let eq125_e1598_d_b39: f64 = ((p.p251 * s.db[228][39]) * ddt_scale);
        let eq125_e1598_d_b40: f64 = ((p.p251 * s.db[228][40]) * ddt_scale);
        let eq125_e1598_d_b41: f64 = ((p.p251 * s.db[228][41]) * ddt_scale);
        let eq125_e1598_d_b42: f64 = ((p.p251 * s.db[228][42]) * ddt_scale);
        let eq125_e1598_d_b43: f64 = ((p.p251 * s.db[228][43]) * ddt_scale);
        let eq125_e1598_d_b44: f64 = ((p.p251 * s.db[228][44]) * ddt_scale);
        let eq125_e1598_d_b45: f64 = ((p.p251 * s.db[228][45]) * ddt_scale);
        let eq125_e1598_d_b46: f64 = ((p.p251 * s.db[228][46]) * ddt_scale);
        let eq125_e1598_d_b47: f64 = ((p.p251 * s.db[228][47]) * ddt_scale);
        let eq125_e1598_d_b48: f64 = ((p.p251 * s.db[228][48]) * ddt_scale);
        let eq125_e1598_d_b49: f64 = ((p.p251 * s.db[228][49]) * ddt_scale);
        let eq125_e1598_d_b50: f64 = ((p.p251 * s.db[228][50]) * ddt_scale);
        let eq125_e1598_d_b51: f64 = ((p.p251 * s.db[228][51]) * ddt_scale);
        let eq125_e1598_d_b52: f64 = ((p.p251 * s.db[228][52]) * ddt_scale);
        let eq125_e1598_d_b53: f64 = ((p.p251 * s.db[228][53]) * ddt_scale);
        let eq125_e1598_d_b54: f64 = ((p.p251 * s.db[228][54]) * ddt_scale);
        let eq125_e1599: f64 = (p.p7 * eq125_e1598);
        let eq125_e1599_d_n0: f64 = (p.p7 * eq125_e1598_d_n0);
        let eq125_e1599_d_n1: f64 = (p.p7 * eq125_e1598_d_n1);
        let eq125_e1599_d_n2: f64 = (p.p7 * eq125_e1598_d_n2);
        let eq125_e1599_d_n3: f64 = (p.p7 * eq125_e1598_d_n3);
        let eq125_e1599_d_n4: f64 = (p.p7 * eq125_e1598_d_n4);
        let eq125_e1599_d_n5: f64 = (p.p7 * eq125_e1598_d_n5);
        let eq125_e1599_d_n6: f64 = (p.p7 * eq125_e1598_d_n6);
        let eq125_e1599_d_n7: f64 = (p.p7 * eq125_e1598_d_n7);
        let eq125_e1599_d_n8: f64 = (p.p7 * eq125_e1598_d_n8);
        let eq125_e1599_d_n9: f64 = (p.p7 * eq125_e1598_d_n9);
        let eq125_e1599_d_n10: f64 = (p.p7 * eq125_e1598_d_n10);
        let eq125_e1599_d_n11: f64 = (p.p7 * eq125_e1598_d_n11);
        let eq125_e1599_d_n12: f64 = (p.p7 * eq125_e1598_d_n12);
        let eq125_e1599_d_n13: f64 = (p.p7 * eq125_e1598_d_n13);
        let eq125_e1599_d_n14: f64 = (p.p7 * eq125_e1598_d_n14);
        let eq125_e1599_d_n15: f64 = (p.p7 * eq125_e1598_d_n15);
        let eq125_e1599_d_n16: f64 = (p.p7 * eq125_e1598_d_n16);
        let eq125_e1599_d_n17: f64 = (p.p7 * eq125_e1598_d_n17);
        let eq125_e1599_d_n18: f64 = (p.p7 * eq125_e1598_d_n18);
        let eq125_e1599_d_n19: f64 = (p.p7 * eq125_e1598_d_n19);
        let eq125_e1599_d_n20: f64 = (p.p7 * eq125_e1598_d_n20);
        let eq125_e1599_d_n21: f64 = (p.p7 * eq125_e1598_d_n21);
        let eq125_e1599_d_n22: f64 = (p.p7 * eq125_e1598_d_n22);
        let eq125_e1599_d_b0: f64 = (p.p7 * eq125_e1598_d_b0);
        let eq125_e1599_d_b1: f64 = (p.p7 * eq125_e1598_d_b1);
        let eq125_e1599_d_b2: f64 = (p.p7 * eq125_e1598_d_b2);
        let eq125_e1599_d_b3: f64 = (p.p7 * eq125_e1598_d_b3);
        let eq125_e1599_d_b4: f64 = (p.p7 * eq125_e1598_d_b4);
        let eq125_e1599_d_b5: f64 = (p.p7 * eq125_e1598_d_b5);
        let eq125_e1599_d_b6: f64 = (p.p7 * eq125_e1598_d_b6);
        let eq125_e1599_d_b7: f64 = (p.p7 * eq125_e1598_d_b7);
        let eq125_e1599_d_b8: f64 = (p.p7 * eq125_e1598_d_b8);
        let eq125_e1599_d_b9: f64 = (p.p7 * eq125_e1598_d_b9);
        let eq125_e1599_d_b10: f64 = (p.p7 * eq125_e1598_d_b10);
        let eq125_e1599_d_b11: f64 = (p.p7 * eq125_e1598_d_b11);
        let eq125_e1599_d_b12: f64 = (p.p7 * eq125_e1598_d_b12);
        let eq125_e1599_d_b13: f64 = (p.p7 * eq125_e1598_d_b13);
        let eq125_e1599_d_b14: f64 = (p.p7 * eq125_e1598_d_b14);
        let eq125_e1599_d_b15: f64 = (p.p7 * eq125_e1598_d_b15);
        let eq125_e1599_d_b16: f64 = (p.p7 * eq125_e1598_d_b16);
        let eq125_e1599_d_b17: f64 = (p.p7 * eq125_e1598_d_b17);
        let eq125_e1599_d_b18: f64 = (p.p7 * eq125_e1598_d_b18);
        let eq125_e1599_d_b19: f64 = (p.p7 * eq125_e1598_d_b19);
        let eq125_e1599_d_b20: f64 = (p.p7 * eq125_e1598_d_b20);
        let eq125_e1599_d_b21: f64 = (p.p7 * eq125_e1598_d_b21);
        let eq125_e1599_d_b22: f64 = (p.p7 * eq125_e1598_d_b22);
        let eq125_e1599_d_b23: f64 = (p.p7 * eq125_e1598_d_b23);
        let eq125_e1599_d_b24: f64 = (p.p7 * eq125_e1598_d_b24);
        let eq125_e1599_d_b25: f64 = (p.p7 * eq125_e1598_d_b25);
        let eq125_e1599_d_b26: f64 = (p.p7 * eq125_e1598_d_b26);
        let eq125_e1599_d_b27: f64 = (p.p7 * eq125_e1598_d_b27);
        let eq125_e1599_d_b28: f64 = (p.p7 * eq125_e1598_d_b28);
        let eq125_e1599_d_b29: f64 = (p.p7 * eq125_e1598_d_b29);
        let eq125_e1599_d_b30: f64 = (p.p7 * eq125_e1598_d_b30);
        let eq125_e1599_d_b31: f64 = (p.p7 * eq125_e1598_d_b31);
        let eq125_e1599_d_b32: f64 = (p.p7 * eq125_e1598_d_b32);
        let eq125_e1599_d_b33: f64 = (p.p7 * eq125_e1598_d_b33);
        let eq125_e1599_d_b34: f64 = (p.p7 * eq125_e1598_d_b34);
        let eq125_e1599_d_b35: f64 = (p.p7 * eq125_e1598_d_b35);
        let eq125_e1599_d_b36: f64 = (p.p7 * eq125_e1598_d_b36);
        let eq125_e1599_d_b37: f64 = (p.p7 * eq125_e1598_d_b37);
        let eq125_e1599_d_b38: f64 = (p.p7 * eq125_e1598_d_b38);
        let eq125_e1599_d_b39: f64 = (p.p7 * eq125_e1598_d_b39);
        let eq125_e1599_d_b40: f64 = (p.p7 * eq125_e1598_d_b40);
        let eq125_e1599_d_b41: f64 = (p.p7 * eq125_e1598_d_b41);
        let eq125_e1599_d_b42: f64 = (p.p7 * eq125_e1598_d_b42);
        let eq125_e1599_d_b43: f64 = (p.p7 * eq125_e1598_d_b43);
        let eq125_e1599_d_b44: f64 = (p.p7 * eq125_e1598_d_b44);
        let eq125_e1599_d_b45: f64 = (p.p7 * eq125_e1598_d_b45);
        let eq125_e1599_d_b46: f64 = (p.p7 * eq125_e1598_d_b46);
        let eq125_e1599_d_b47: f64 = (p.p7 * eq125_e1598_d_b47);
        let eq125_e1599_d_b48: f64 = (p.p7 * eq125_e1598_d_b48);
        let eq125_e1599_d_b49: f64 = (p.p7 * eq125_e1598_d_b49);
        let eq125_e1599_d_b50: f64 = (p.p7 * eq125_e1598_d_b50);
        let eq125_e1599_d_b51: f64 = (p.p7 * eq125_e1598_d_b51);
        let eq125_e1599_d_b52: f64 = (p.p7 * eq125_e1598_d_b52);
        let eq125_e1599_d_b53: f64 = (p.p7 * eq125_e1598_d_b53);
        let eq125_e1599_d_b54: f64 = (p.p7 * eq125_e1598_d_b54);
        (eq125_e1599, eq125_e1599_d_n0, eq125_e1599_d_n1, eq125_e1599_d_n2, eq125_e1599_d_n3, eq125_e1599_d_n4, eq125_e1599_d_n5, eq125_e1599_d_n6, eq125_e1599_d_n7, eq125_e1599_d_n8, eq125_e1599_d_n9, eq125_e1599_d_n10, eq125_e1599_d_n11, eq125_e1599_d_n12, eq125_e1599_d_n13, eq125_e1599_d_n14, eq125_e1599_d_n15, eq125_e1599_d_n16, eq125_e1599_d_n17, eq125_e1599_d_n18, eq125_e1599_d_n19, eq125_e1599_d_n20, eq125_e1599_d_n21, eq125_e1599_d_n22, eq125_e1599_d_b0, eq125_e1599_d_b1, eq125_e1599_d_b2, eq125_e1599_d_b3, eq125_e1599_d_b4, eq125_e1599_d_b5, eq125_e1599_d_b6, eq125_e1599_d_b7, eq125_e1599_d_b8, eq125_e1599_d_b9, eq125_e1599_d_b10, eq125_e1599_d_b11, eq125_e1599_d_b12, eq125_e1599_d_b13, eq125_e1599_d_b14, eq125_e1599_d_b15, eq125_e1599_d_b16, eq125_e1599_d_b17, eq125_e1599_d_b18, eq125_e1599_d_b19, eq125_e1599_d_b20, eq125_e1599_d_b21, eq125_e1599_d_b22, eq125_e1599_d_b23, eq125_e1599_d_b24, eq125_e1599_d_b25, eq125_e1599_d_b26, eq125_e1599_d_b27, eq125_e1599_d_b28, eq125_e1599_d_b29, eq125_e1599_d_b30, eq125_e1599_d_b31, eq125_e1599_d_b32, eq125_e1599_d_b33, eq125_e1599_d_b34, eq125_e1599_d_b35, eq125_e1599_d_b36, eq125_e1599_d_b37, eq125_e1599_d_b38, eq125_e1599_d_b39, eq125_e1599_d_b40, eq125_e1599_d_b41, eq125_e1599_d_b42, eq125_e1599_d_b43, eq125_e1599_d_b44, eq125_e1599_d_b45, eq125_e1599_d_b46, eq125_e1599_d_b47, eq125_e1599_d_b48, eq125_e1599_d_b49, eq125_e1599_d_b50, eq125_e1599_d_b51, eq125_e1599_d_b52, eq125_e1599_d_b53, eq125_e1599_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_value: f64 = eq125_e1601;
        let eq125_node_derivatives: [f64; 23] = [eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22];
        let eq125_branch_derivatives: [f64; 55] = [eq125_e1601_d_b0, eq125_e1601_d_b1, eq125_e1601_d_b2, eq125_e1601_d_b3, eq125_e1601_d_b4, eq125_e1601_d_b5, eq125_e1601_d_b6, eq125_e1601_d_b7, eq125_e1601_d_b8, eq125_e1601_d_b9, eq125_e1601_d_b10, eq125_e1601_d_b11, eq125_e1601_d_b12, eq125_e1601_d_b13, eq125_e1601_d_b14, eq125_e1601_d_b15, eq125_e1601_d_b16, eq125_e1601_d_b17, eq125_e1601_d_b18, eq125_e1601_d_b19, eq125_e1601_d_b20, eq125_e1601_d_b21, eq125_e1601_d_b22, eq125_e1601_d_b23, eq125_e1601_d_b24, eq125_e1601_d_b25, eq125_e1601_d_b26, eq125_e1601_d_b27, eq125_e1601_d_b28, eq125_e1601_d_b29, eq125_e1601_d_b30, eq125_e1601_d_b31, eq125_e1601_d_b32, eq125_e1601_d_b33, eq125_e1601_d_b34, eq125_e1601_d_b35, eq125_e1601_d_b36, eq125_e1601_d_b37, eq125_e1601_d_b38, eq125_e1601_d_b39, eq125_e1601_d_b40, eq125_e1601_d_b41, eq125_e1601_d_b42, eq125_e1601_d_b43, eq125_e1601_d_b44, eq125_e1601_d_b45, eq125_e1601_d_b46, eq125_e1601_d_b47, eq125_e1601_d_b48, eq125_e1601_d_b49, eq125_e1601_d_b50, eq125_e1601_d_b51, eq125_e1601_d_b52, eq125_e1601_d_b53, eq125_e1601_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq125_value),
            &eq125_node_derivatives,
            &eq125_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1611, eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22, eq126_e1611_d_b0, eq126_e1611_d_b1, eq126_e1611_d_b2, eq126_e1611_d_b3, eq126_e1611_d_b4, eq126_e1611_d_b5, eq126_e1611_d_b6, eq126_e1611_d_b7, eq126_e1611_d_b8, eq126_e1611_d_b9, eq126_e1611_d_b10, eq126_e1611_d_b11, eq126_e1611_d_b12, eq126_e1611_d_b13, eq126_e1611_d_b14, eq126_e1611_d_b15, eq126_e1611_d_b16, eq126_e1611_d_b17, eq126_e1611_d_b18, eq126_e1611_d_b19, eq126_e1611_d_b20, eq126_e1611_d_b21, eq126_e1611_d_b22, eq126_e1611_d_b23, eq126_e1611_d_b24, eq126_e1611_d_b25, eq126_e1611_d_b26, eq126_e1611_d_b27, eq126_e1611_d_b28, eq126_e1611_d_b29, eq126_e1611_d_b30, eq126_e1611_d_b31, eq126_e1611_d_b32, eq126_e1611_d_b33, eq126_e1611_d_b34, eq126_e1611_d_b35, eq126_e1611_d_b36, eq126_e1611_d_b37, eq126_e1611_d_b38, eq126_e1611_d_b39, eq126_e1611_d_b40, eq126_e1611_d_b41, eq126_e1611_d_b42, eq126_e1611_d_b43, eq126_e1611_d_b44, eq126_e1611_d_b45, eq126_e1611_d_b46, eq126_e1611_d_b47, eq126_e1611_d_b48, eq126_e1611_d_b49, eq126_e1611_d_b50, eq126_e1611_d_b51, eq126_e1611_d_b52, eq126_e1611_d_b53, eq126_e1611_d_b54,) = {
    if ((!s.b[570]) && s.b[573]) {
        let eq126_e1608: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 25, s.v[229]);
        let eq126_e1609: f64 = (p.p7 * eq126_e1608);
        let eq126_e1609_d_n0: f64 = (p.p7 * (s.dn[229][0] * ddt_scale));
        let eq126_e1609_d_n1: f64 = (p.p7 * (s.dn[229][1] * ddt_scale));
        let eq126_e1609_d_n2: f64 = (p.p7 * (s.dn[229][2] * ddt_scale));
        let eq126_e1609_d_n3: f64 = (p.p7 * (s.dn[229][3] * ddt_scale));
        let eq126_e1609_d_n4: f64 = (p.p7 * (s.dn[229][4] * ddt_scale));
        let eq126_e1609_d_n5: f64 = (p.p7 * (s.dn[229][5] * ddt_scale));
        let eq126_e1609_d_n6: f64 = (p.p7 * (s.dn[229][6] * ddt_scale));
        let eq126_e1609_d_n7: f64 = (p.p7 * (s.dn[229][7] * ddt_scale));
        let eq126_e1609_d_n8: f64 = (p.p7 * (s.dn[229][8] * ddt_scale));
        let eq126_e1609_d_n9: f64 = (p.p7 * (s.dn[229][9] * ddt_scale));
        let eq126_e1609_d_n10: f64 = (p.p7 * (s.dn[229][10] * ddt_scale));
        let eq126_e1609_d_n11: f64 = (p.p7 * (s.dn[229][11] * ddt_scale));
        let eq126_e1609_d_n12: f64 = (p.p7 * (s.dn[229][12] * ddt_scale));
        let eq126_e1609_d_n13: f64 = (p.p7 * (s.dn[229][13] * ddt_scale));
        let eq126_e1609_d_n14: f64 = (p.p7 * (s.dn[229][14] * ddt_scale));
        let eq126_e1609_d_n15: f64 = (p.p7 * (s.dn[229][15] * ddt_scale));
        let eq126_e1609_d_n16: f64 = (p.p7 * (s.dn[229][16] * ddt_scale));
        let eq126_e1609_d_n17: f64 = (p.p7 * (s.dn[229][17] * ddt_scale));
        let eq126_e1609_d_n18: f64 = (p.p7 * (s.dn[229][18] * ddt_scale));
        let eq126_e1609_d_n19: f64 = (p.p7 * (s.dn[229][19] * ddt_scale));
        let eq126_e1609_d_n20: f64 = (p.p7 * (s.dn[229][20] * ddt_scale));
        let eq126_e1609_d_n21: f64 = (p.p7 * (s.dn[229][21] * ddt_scale));
        let eq126_e1609_d_n22: f64 = (p.p7 * (s.dn[229][22] * ddt_scale));
        let eq126_e1609_d_b0: f64 = (p.p7 * (s.db[229][0] * ddt_scale));
        let eq126_e1609_d_b1: f64 = (p.p7 * (s.db[229][1] * ddt_scale));
        let eq126_e1609_d_b2: f64 = (p.p7 * (s.db[229][2] * ddt_scale));
        let eq126_e1609_d_b3: f64 = (p.p7 * (s.db[229][3] * ddt_scale));
        let eq126_e1609_d_b4: f64 = (p.p7 * (s.db[229][4] * ddt_scale));
        let eq126_e1609_d_b5: f64 = (p.p7 * (s.db[229][5] * ddt_scale));
        let eq126_e1609_d_b6: f64 = (p.p7 * (s.db[229][6] * ddt_scale));
        let eq126_e1609_d_b7: f64 = (p.p7 * (s.db[229][7] * ddt_scale));
        let eq126_e1609_d_b8: f64 = (p.p7 * (s.db[229][8] * ddt_scale));
        let eq126_e1609_d_b9: f64 = (p.p7 * (s.db[229][9] * ddt_scale));
        let eq126_e1609_d_b10: f64 = (p.p7 * (s.db[229][10] * ddt_scale));
        let eq126_e1609_d_b11: f64 = (p.p7 * (s.db[229][11] * ddt_scale));
        let eq126_e1609_d_b12: f64 = (p.p7 * (s.db[229][12] * ddt_scale));
        let eq126_e1609_d_b13: f64 = (p.p7 * (s.db[229][13] * ddt_scale));
        let eq126_e1609_d_b14: f64 = (p.p7 * (s.db[229][14] * ddt_scale));
        let eq126_e1609_d_b15: f64 = (p.p7 * (s.db[229][15] * ddt_scale));
        let eq126_e1609_d_b16: f64 = (p.p7 * (s.db[229][16] * ddt_scale));
        let eq126_e1609_d_b17: f64 = (p.p7 * (s.db[229][17] * ddt_scale));
        let eq126_e1609_d_b18: f64 = (p.p7 * (s.db[229][18] * ddt_scale));
        let eq126_e1609_d_b19: f64 = (p.p7 * (s.db[229][19] * ddt_scale));
        let eq126_e1609_d_b20: f64 = (p.p7 * (s.db[229][20] * ddt_scale));
        let eq126_e1609_d_b21: f64 = (p.p7 * (s.db[229][21] * ddt_scale));
        let eq126_e1609_d_b22: f64 = (p.p7 * (s.db[229][22] * ddt_scale));
        let eq126_e1609_d_b23: f64 = (p.p7 * (s.db[229][23] * ddt_scale));
        let eq126_e1609_d_b24: f64 = (p.p7 * (s.db[229][24] * ddt_scale));
        let eq126_e1609_d_b25: f64 = (p.p7 * (s.db[229][25] * ddt_scale));
        let eq126_e1609_d_b26: f64 = (p.p7 * (s.db[229][26] * ddt_scale));
        let eq126_e1609_d_b27: f64 = (p.p7 * (s.db[229][27] * ddt_scale));
        let eq126_e1609_d_b28: f64 = (p.p7 * (s.db[229][28] * ddt_scale));
        let eq126_e1609_d_b29: f64 = (p.p7 * (s.db[229][29] * ddt_scale));
        let eq126_e1609_d_b30: f64 = (p.p7 * (s.db[229][30] * ddt_scale));
        let eq126_e1609_d_b31: f64 = (p.p7 * (s.db[229][31] * ddt_scale));
        let eq126_e1609_d_b32: f64 = (p.p7 * (s.db[229][32] * ddt_scale));
        let eq126_e1609_d_b33: f64 = (p.p7 * (s.db[229][33] * ddt_scale));
        let eq126_e1609_d_b34: f64 = (p.p7 * (s.db[229][34] * ddt_scale));
        let eq126_e1609_d_b35: f64 = (p.p7 * (s.db[229][35] * ddt_scale));
        let eq126_e1609_d_b36: f64 = (p.p7 * (s.db[229][36] * ddt_scale));
        let eq126_e1609_d_b37: f64 = (p.p7 * (s.db[229][37] * ddt_scale));
        let eq126_e1609_d_b38: f64 = (p.p7 * (s.db[229][38] * ddt_scale));
        let eq126_e1609_d_b39: f64 = (p.p7 * (s.db[229][39] * ddt_scale));
        let eq126_e1609_d_b40: f64 = (p.p7 * (s.db[229][40] * ddt_scale));
        let eq126_e1609_d_b41: f64 = (p.p7 * (s.db[229][41] * ddt_scale));
        let eq126_e1609_d_b42: f64 = (p.p7 * (s.db[229][42] * ddt_scale));
        let eq126_e1609_d_b43: f64 = (p.p7 * (s.db[229][43] * ddt_scale));
        let eq126_e1609_d_b44: f64 = (p.p7 * (s.db[229][44] * ddt_scale));
        let eq126_e1609_d_b45: f64 = (p.p7 * (s.db[229][45] * ddt_scale));
        let eq126_e1609_d_b46: f64 = (p.p7 * (s.db[229][46] * ddt_scale));
        let eq126_e1609_d_b47: f64 = (p.p7 * (s.db[229][47] * ddt_scale));
        let eq126_e1609_d_b48: f64 = (p.p7 * (s.db[229][48] * ddt_scale));
        let eq126_e1609_d_b49: f64 = (p.p7 * (s.db[229][49] * ddt_scale));
        let eq126_e1609_d_b50: f64 = (p.p7 * (s.db[229][50] * ddt_scale));
        let eq126_e1609_d_b51: f64 = (p.p7 * (s.db[229][51] * ddt_scale));
        let eq126_e1609_d_b52: f64 = (p.p7 * (s.db[229][52] * ddt_scale));
        let eq126_e1609_d_b53: f64 = (p.p7 * (s.db[229][53] * ddt_scale));
        let eq126_e1609_d_b54: f64 = (p.p7 * (s.db[229][54] * ddt_scale));
        (eq126_e1609, eq126_e1609_d_n0, eq126_e1609_d_n1, eq126_e1609_d_n2, eq126_e1609_d_n3, eq126_e1609_d_n4, eq126_e1609_d_n5, eq126_e1609_d_n6, eq126_e1609_d_n7, eq126_e1609_d_n8, eq126_e1609_d_n9, eq126_e1609_d_n10, eq126_e1609_d_n11, eq126_e1609_d_n12, eq126_e1609_d_n13, eq126_e1609_d_n14, eq126_e1609_d_n15, eq126_e1609_d_n16, eq126_e1609_d_n17, eq126_e1609_d_n18, eq126_e1609_d_n19, eq126_e1609_d_n20, eq126_e1609_d_n21, eq126_e1609_d_n22, eq126_e1609_d_b0, eq126_e1609_d_b1, eq126_e1609_d_b2, eq126_e1609_d_b3, eq126_e1609_d_b4, eq126_e1609_d_b5, eq126_e1609_d_b6, eq126_e1609_d_b7, eq126_e1609_d_b8, eq126_e1609_d_b9, eq126_e1609_d_b10, eq126_e1609_d_b11, eq126_e1609_d_b12, eq126_e1609_d_b13, eq126_e1609_d_b14, eq126_e1609_d_b15, eq126_e1609_d_b16, eq126_e1609_d_b17, eq126_e1609_d_b18, eq126_e1609_d_b19, eq126_e1609_d_b20, eq126_e1609_d_b21, eq126_e1609_d_b22, eq126_e1609_d_b23, eq126_e1609_d_b24, eq126_e1609_d_b25, eq126_e1609_d_b26, eq126_e1609_d_b27, eq126_e1609_d_b28, eq126_e1609_d_b29, eq126_e1609_d_b30, eq126_e1609_d_b31, eq126_e1609_d_b32, eq126_e1609_d_b33, eq126_e1609_d_b34, eq126_e1609_d_b35, eq126_e1609_d_b36, eq126_e1609_d_b37, eq126_e1609_d_b38, eq126_e1609_d_b39, eq126_e1609_d_b40, eq126_e1609_d_b41, eq126_e1609_d_b42, eq126_e1609_d_b43, eq126_e1609_d_b44, eq126_e1609_d_b45, eq126_e1609_d_b46, eq126_e1609_d_b47, eq126_e1609_d_b48, eq126_e1609_d_b49, eq126_e1609_d_b50, eq126_e1609_d_b51, eq126_e1609_d_b52, eq126_e1609_d_b53, eq126_e1609_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_value: f64 = eq126_e1611;
        let eq126_node_derivatives: [f64; 23] = [eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22];
        let eq126_branch_derivatives: [f64; 55] = [eq126_e1611_d_b0, eq126_e1611_d_b1, eq126_e1611_d_b2, eq126_e1611_d_b3, eq126_e1611_d_b4, eq126_e1611_d_b5, eq126_e1611_d_b6, eq126_e1611_d_b7, eq126_e1611_d_b8, eq126_e1611_d_b9, eq126_e1611_d_b10, eq126_e1611_d_b11, eq126_e1611_d_b12, eq126_e1611_d_b13, eq126_e1611_d_b14, eq126_e1611_d_b15, eq126_e1611_d_b16, eq126_e1611_d_b17, eq126_e1611_d_b18, eq126_e1611_d_b19, eq126_e1611_d_b20, eq126_e1611_d_b21, eq126_e1611_d_b22, eq126_e1611_d_b23, eq126_e1611_d_b24, eq126_e1611_d_b25, eq126_e1611_d_b26, eq126_e1611_d_b27, eq126_e1611_d_b28, eq126_e1611_d_b29, eq126_e1611_d_b30, eq126_e1611_d_b31, eq126_e1611_d_b32, eq126_e1611_d_b33, eq126_e1611_d_b34, eq126_e1611_d_b35, eq126_e1611_d_b36, eq126_e1611_d_b37, eq126_e1611_d_b38, eq126_e1611_d_b39, eq126_e1611_d_b40, eq126_e1611_d_b41, eq126_e1611_d_b42, eq126_e1611_d_b43, eq126_e1611_d_b44, eq126_e1611_d_b45, eq126_e1611_d_b46, eq126_e1611_d_b47, eq126_e1611_d_b48, eq126_e1611_d_b49, eq126_e1611_d_b50, eq126_e1611_d_b51, eq126_e1611_d_b52, eq126_e1611_d_b53, eq126_e1611_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq126_value),
            &eq126_node_derivatives,
            &eq126_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_18(
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[228][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[228][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[228][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[228][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[228][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[228][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[228][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[228][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[228][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[228][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[228][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[228][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[228][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[228][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[228][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[228][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[228][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[228][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[228][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[228][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[228][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[228][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[228][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[228][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[228][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[228][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[228][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[228][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[228][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[228][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[228][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[228][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[228][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[228][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[228][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[228][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[228][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[228][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[228][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[228][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[228][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[228][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[228][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[228][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[228][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[228][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[228][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[228][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[228][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[228][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[228][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[228][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[228][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[228][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[228][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[228][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[228][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[228][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[228][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[228][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[228][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[228][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[228][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[228][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[228][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[228][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[228][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[228][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[228][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[228][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[228][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[228][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[228][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[228][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[228][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[228][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[228][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[228][54] * ddt_scale));
        let (eq127_e1623, eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22, eq127_e1623_d_b0, eq127_e1623_d_b1, eq127_e1623_d_b2, eq127_e1623_d_b3, eq127_e1623_d_b4, eq127_e1623_d_b5, eq127_e1623_d_b6, eq127_e1623_d_b7, eq127_e1623_d_b8, eq127_e1623_d_b9, eq127_e1623_d_b10, eq127_e1623_d_b11, eq127_e1623_d_b12, eq127_e1623_d_b13, eq127_e1623_d_b14, eq127_e1623_d_b15, eq127_e1623_d_b16, eq127_e1623_d_b17, eq127_e1623_d_b18, eq127_e1623_d_b19, eq127_e1623_d_b20, eq127_e1623_d_b21, eq127_e1623_d_b22, eq127_e1623_d_b23, eq127_e1623_d_b24, eq127_e1623_d_b25, eq127_e1623_d_b26, eq127_e1623_d_b27, eq127_e1623_d_b28, eq127_e1623_d_b29, eq127_e1623_d_b30, eq127_e1623_d_b31, eq127_e1623_d_b32, eq127_e1623_d_b33, eq127_e1623_d_b34, eq127_e1623_d_b35, eq127_e1623_d_b36, eq127_e1623_d_b37, eq127_e1623_d_b38, eq127_e1623_d_b39, eq127_e1623_d_b40, eq127_e1623_d_b41, eq127_e1623_d_b42, eq127_e1623_d_b43, eq127_e1623_d_b44, eq127_e1623_d_b45, eq127_e1623_d_b46, eq127_e1623_d_b47, eq127_e1623_d_b48, eq127_e1623_d_b49, eq127_e1623_d_b50, eq127_e1623_d_b51, eq127_e1623_d_b52, eq127_e1623_d_b53, eq127_e1623_d_b54,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq127_e1620: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 26, s.v[228]);
        let eq127_e1621: f64 = (p.p7 * eq127_e1620);
        (eq127_e1621, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq127_value: f64 = eq127_e1623;
        let eq127_node_derivatives: [f64; 23] = [eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22];
        let eq127_branch_derivatives: [f64; 55] = [eq127_e1623_d_b0, eq127_e1623_d_b1, eq127_e1623_d_b2, eq127_e1623_d_b3, eq127_e1623_d_b4, eq127_e1623_d_b5, eq127_e1623_d_b6, eq127_e1623_d_b7, eq127_e1623_d_b8, eq127_e1623_d_b9, eq127_e1623_d_b10, eq127_e1623_d_b11, eq127_e1623_d_b12, eq127_e1623_d_b13, eq127_e1623_d_b14, eq127_e1623_d_b15, eq127_e1623_d_b16, eq127_e1623_d_b17, eq127_e1623_d_b18, eq127_e1623_d_b19, eq127_e1623_d_b20, eq127_e1623_d_b21, eq127_e1623_d_b22, eq127_e1623_d_b23, eq127_e1623_d_b24, eq127_e1623_d_b25, eq127_e1623_d_b26, eq127_e1623_d_b27, eq127_e1623_d_b28, eq127_e1623_d_b29, eq127_e1623_d_b30, eq127_e1623_d_b31, eq127_e1623_d_b32, eq127_e1623_d_b33, eq127_e1623_d_b34, eq127_e1623_d_b35, eq127_e1623_d_b36, eq127_e1623_d_b37, eq127_e1623_d_b38, eq127_e1623_d_b39, eq127_e1623_d_b40, eq127_e1623_d_b41, eq127_e1623_d_b42, eq127_e1623_d_b43, eq127_e1623_d_b44, eq127_e1623_d_b45, eq127_e1623_d_b46, eq127_e1623_d_b47, eq127_e1623_d_b48, eq127_e1623_d_b49, eq127_e1623_d_b50, eq127_e1623_d_b51, eq127_e1623_d_b52, eq127_e1623_d_b53, eq127_e1623_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq127_value),
            &eq127_node_derivatives,
            &eq127_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1637, eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22, eq128_e1637_d_b0, eq128_e1637_d_b1, eq128_e1637_d_b2, eq128_e1637_d_b3, eq128_e1637_d_b4, eq128_e1637_d_b5, eq128_e1637_d_b6, eq128_e1637_d_b7, eq128_e1637_d_b8, eq128_e1637_d_b9, eq128_e1637_d_b10, eq128_e1637_d_b11, eq128_e1637_d_b12, eq128_e1637_d_b13, eq128_e1637_d_b14, eq128_e1637_d_b15, eq128_e1637_d_b16, eq128_e1637_d_b17, eq128_e1637_d_b18, eq128_e1637_d_b19, eq128_e1637_d_b20, eq128_e1637_d_b21, eq128_e1637_d_b22, eq128_e1637_d_b23, eq128_e1637_d_b24, eq128_e1637_d_b25, eq128_e1637_d_b26, eq128_e1637_d_b27, eq128_e1637_d_b28, eq128_e1637_d_b29, eq128_e1637_d_b30, eq128_e1637_d_b31, eq128_e1637_d_b32, eq128_e1637_d_b33, eq128_e1637_d_b34, eq128_e1637_d_b35, eq128_e1637_d_b36, eq128_e1637_d_b37, eq128_e1637_d_b38, eq128_e1637_d_b39, eq128_e1637_d_b40, eq128_e1637_d_b41, eq128_e1637_d_b42, eq128_e1637_d_b43, eq128_e1637_d_b44, eq128_e1637_d_b45, eq128_e1637_d_b46, eq128_e1637_d_b47, eq128_e1637_d_b48, eq128_e1637_d_b49, eq128_e1637_d_b50, eq128_e1637_d_b51, eq128_e1637_d_b52, eq128_e1637_d_b53, eq128_e1637_d_b54,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq128_e1632: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 27, s.v[228]);
        let eq128_e1633: f64 = (p.p7 * eq128_e1632);
        let eq128_e1635: f64 = (eq128_e1633 * p.p246);
        let eq128_e1635_d_n0: f64 = (__rspice_deriv_cse_0 * p.p246);
        let eq128_e1635_d_n1: f64 = (__rspice_deriv_cse_1 * p.p246);
        let eq128_e1635_d_n2: f64 = (__rspice_deriv_cse_2 * p.p246);
        let eq128_e1635_d_n3: f64 = (__rspice_deriv_cse_3 * p.p246);
        let eq128_e1635_d_n4: f64 = (__rspice_deriv_cse_4 * p.p246);
        let eq128_e1635_d_n5: f64 = (__rspice_deriv_cse_5 * p.p246);
        let eq128_e1635_d_n6: f64 = (__rspice_deriv_cse_6 * p.p246);
        let eq128_e1635_d_n7: f64 = (__rspice_deriv_cse_7 * p.p246);
        let eq128_e1635_d_n8: f64 = (__rspice_deriv_cse_8 * p.p246);
        let eq128_e1635_d_n9: f64 = (__rspice_deriv_cse_9 * p.p246);
        let eq128_e1635_d_n10: f64 = (__rspice_deriv_cse_10 * p.p246);
        let eq128_e1635_d_n11: f64 = (__rspice_deriv_cse_11 * p.p246);
        let eq128_e1635_d_n12: f64 = (__rspice_deriv_cse_12 * p.p246);
        let eq128_e1635_d_n13: f64 = (__rspice_deriv_cse_13 * p.p246);
        let eq128_e1635_d_n14: f64 = (__rspice_deriv_cse_14 * p.p246);
        let eq128_e1635_d_n15: f64 = (__rspice_deriv_cse_15 * p.p246);
        let eq128_e1635_d_n16: f64 = (__rspice_deriv_cse_16 * p.p246);
        let eq128_e1635_d_n17: f64 = (__rspice_deriv_cse_17 * p.p246);
        let eq128_e1635_d_n18: f64 = (__rspice_deriv_cse_18 * p.p246);
        let eq128_e1635_d_n19: f64 = (__rspice_deriv_cse_19 * p.p246);
        let eq128_e1635_d_n20: f64 = (__rspice_deriv_cse_20 * p.p246);
        let eq128_e1635_d_n21: f64 = (__rspice_deriv_cse_21 * p.p246);
        let eq128_e1635_d_n22: f64 = (__rspice_deriv_cse_22 * p.p246);
        let eq128_e1635_d_b0: f64 = (__rspice_deriv_cse_23 * p.p246);
        let eq128_e1635_d_b1: f64 = (__rspice_deriv_cse_24 * p.p246);
        let eq128_e1635_d_b2: f64 = (__rspice_deriv_cse_25 * p.p246);
        let eq128_e1635_d_b3: f64 = (__rspice_deriv_cse_26 * p.p246);
        let eq128_e1635_d_b4: f64 = (__rspice_deriv_cse_27 * p.p246);
        let eq128_e1635_d_b5: f64 = (__rspice_deriv_cse_28 * p.p246);
        let eq128_e1635_d_b6: f64 = (__rspice_deriv_cse_29 * p.p246);
        let eq128_e1635_d_b7: f64 = (__rspice_deriv_cse_30 * p.p246);
        let eq128_e1635_d_b8: f64 = (__rspice_deriv_cse_31 * p.p246);
        let eq128_e1635_d_b9: f64 = (__rspice_deriv_cse_32 * p.p246);
        let eq128_e1635_d_b10: f64 = (__rspice_deriv_cse_33 * p.p246);
        let eq128_e1635_d_b11: f64 = (__rspice_deriv_cse_34 * p.p246);
        let eq128_e1635_d_b12: f64 = (__rspice_deriv_cse_35 * p.p246);
        let eq128_e1635_d_b13: f64 = (__rspice_deriv_cse_36 * p.p246);
        let eq128_e1635_d_b14: f64 = (__rspice_deriv_cse_37 * p.p246);
        let eq128_e1635_d_b15: f64 = (__rspice_deriv_cse_38 * p.p246);
        let eq128_e1635_d_b16: f64 = (__rspice_deriv_cse_39 * p.p246);
        let eq128_e1635_d_b17: f64 = (__rspice_deriv_cse_40 * p.p246);
        let eq128_e1635_d_b18: f64 = (__rspice_deriv_cse_41 * p.p246);
        let eq128_e1635_d_b19: f64 = (__rspice_deriv_cse_42 * p.p246);
        let eq128_e1635_d_b20: f64 = (__rspice_deriv_cse_43 * p.p246);
        let eq128_e1635_d_b21: f64 = (__rspice_deriv_cse_44 * p.p246);
        let eq128_e1635_d_b22: f64 = (__rspice_deriv_cse_45 * p.p246);
        let eq128_e1635_d_b23: f64 = (__rspice_deriv_cse_46 * p.p246);
        let eq128_e1635_d_b24: f64 = (__rspice_deriv_cse_47 * p.p246);
        let eq128_e1635_d_b25: f64 = (__rspice_deriv_cse_48 * p.p246);
        let eq128_e1635_d_b26: f64 = (__rspice_deriv_cse_49 * p.p246);
        let eq128_e1635_d_b27: f64 = (__rspice_deriv_cse_50 * p.p246);
        let eq128_e1635_d_b28: f64 = (__rspice_deriv_cse_51 * p.p246);
        let eq128_e1635_d_b29: f64 = (__rspice_deriv_cse_52 * p.p246);
        let eq128_e1635_d_b30: f64 = (__rspice_deriv_cse_53 * p.p246);
        let eq128_e1635_d_b31: f64 = (__rspice_deriv_cse_54 * p.p246);
        let eq128_e1635_d_b32: f64 = (__rspice_deriv_cse_55 * p.p246);
        let eq128_e1635_d_b33: f64 = (__rspice_deriv_cse_56 * p.p246);
        let eq128_e1635_d_b34: f64 = (__rspice_deriv_cse_57 * p.p246);
        let eq128_e1635_d_b35: f64 = (__rspice_deriv_cse_58 * p.p246);
        let eq128_e1635_d_b36: f64 = (__rspice_deriv_cse_59 * p.p246);
        let eq128_e1635_d_b37: f64 = (__rspice_deriv_cse_60 * p.p246);
        let eq128_e1635_d_b38: f64 = (__rspice_deriv_cse_61 * p.p246);
        let eq128_e1635_d_b39: f64 = (__rspice_deriv_cse_62 * p.p246);
        let eq128_e1635_d_b40: f64 = (__rspice_deriv_cse_63 * p.p246);
        let eq128_e1635_d_b41: f64 = (__rspice_deriv_cse_64 * p.p246);
        let eq128_e1635_d_b42: f64 = (__rspice_deriv_cse_65 * p.p246);
        let eq128_e1635_d_b43: f64 = (__rspice_deriv_cse_66 * p.p246);
        let eq128_e1635_d_b44: f64 = (__rspice_deriv_cse_67 * p.p246);
        let eq128_e1635_d_b45: f64 = (__rspice_deriv_cse_68 * p.p246);
        let eq128_e1635_d_b46: f64 = (__rspice_deriv_cse_69 * p.p246);
        let eq128_e1635_d_b47: f64 = (__rspice_deriv_cse_70 * p.p246);
        let eq128_e1635_d_b48: f64 = (__rspice_deriv_cse_71 * p.p246);
        let eq128_e1635_d_b49: f64 = (__rspice_deriv_cse_72 * p.p246);
        let eq128_e1635_d_b50: f64 = (__rspice_deriv_cse_73 * p.p246);
        let eq128_e1635_d_b51: f64 = (__rspice_deriv_cse_74 * p.p246);
        let eq128_e1635_d_b52: f64 = (__rspice_deriv_cse_75 * p.p246);
        let eq128_e1635_d_b53: f64 = (__rspice_deriv_cse_76 * p.p246);
        let eq128_e1635_d_b54: f64 = (__rspice_deriv_cse_77 * p.p246);
        (eq128_e1635, eq128_e1635_d_n0, eq128_e1635_d_n1, eq128_e1635_d_n2, eq128_e1635_d_n3, eq128_e1635_d_n4, eq128_e1635_d_n5, eq128_e1635_d_n6, eq128_e1635_d_n7, eq128_e1635_d_n8, eq128_e1635_d_n9, eq128_e1635_d_n10, eq128_e1635_d_n11, eq128_e1635_d_n12, eq128_e1635_d_n13, eq128_e1635_d_n14, eq128_e1635_d_n15, eq128_e1635_d_n16, eq128_e1635_d_n17, eq128_e1635_d_n18, eq128_e1635_d_n19, eq128_e1635_d_n20, eq128_e1635_d_n21, eq128_e1635_d_n22, eq128_e1635_d_b0, eq128_e1635_d_b1, eq128_e1635_d_b2, eq128_e1635_d_b3, eq128_e1635_d_b4, eq128_e1635_d_b5, eq128_e1635_d_b6, eq128_e1635_d_b7, eq128_e1635_d_b8, eq128_e1635_d_b9, eq128_e1635_d_b10, eq128_e1635_d_b11, eq128_e1635_d_b12, eq128_e1635_d_b13, eq128_e1635_d_b14, eq128_e1635_d_b15, eq128_e1635_d_b16, eq128_e1635_d_b17, eq128_e1635_d_b18, eq128_e1635_d_b19, eq128_e1635_d_b20, eq128_e1635_d_b21, eq128_e1635_d_b22, eq128_e1635_d_b23, eq128_e1635_d_b24, eq128_e1635_d_b25, eq128_e1635_d_b26, eq128_e1635_d_b27, eq128_e1635_d_b28, eq128_e1635_d_b29, eq128_e1635_d_b30, eq128_e1635_d_b31, eq128_e1635_d_b32, eq128_e1635_d_b33, eq128_e1635_d_b34, eq128_e1635_d_b35, eq128_e1635_d_b36, eq128_e1635_d_b37, eq128_e1635_d_b38, eq128_e1635_d_b39, eq128_e1635_d_b40, eq128_e1635_d_b41, eq128_e1635_d_b42, eq128_e1635_d_b43, eq128_e1635_d_b44, eq128_e1635_d_b45, eq128_e1635_d_b46, eq128_e1635_d_b47, eq128_e1635_d_b48, eq128_e1635_d_b49, eq128_e1635_d_b50, eq128_e1635_d_b51, eq128_e1635_d_b52, eq128_e1635_d_b53, eq128_e1635_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_value: f64 = eq128_e1637;
        let eq128_node_derivatives: [f64; 23] = [eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22];
        let eq128_branch_derivatives: [f64; 55] = [eq128_e1637_d_b0, eq128_e1637_d_b1, eq128_e1637_d_b2, eq128_e1637_d_b3, eq128_e1637_d_b4, eq128_e1637_d_b5, eq128_e1637_d_b6, eq128_e1637_d_b7, eq128_e1637_d_b8, eq128_e1637_d_b9, eq128_e1637_d_b10, eq128_e1637_d_b11, eq128_e1637_d_b12, eq128_e1637_d_b13, eq128_e1637_d_b14, eq128_e1637_d_b15, eq128_e1637_d_b16, eq128_e1637_d_b17, eq128_e1637_d_b18, eq128_e1637_d_b19, eq128_e1637_d_b20, eq128_e1637_d_b21, eq128_e1637_d_b22, eq128_e1637_d_b23, eq128_e1637_d_b24, eq128_e1637_d_b25, eq128_e1637_d_b26, eq128_e1637_d_b27, eq128_e1637_d_b28, eq128_e1637_d_b29, eq128_e1637_d_b30, eq128_e1637_d_b31, eq128_e1637_d_b32, eq128_e1637_d_b33, eq128_e1637_d_b34, eq128_e1637_d_b35, eq128_e1637_d_b36, eq128_e1637_d_b37, eq128_e1637_d_b38, eq128_e1637_d_b39, eq128_e1637_d_b40, eq128_e1637_d_b41, eq128_e1637_d_b42, eq128_e1637_d_b43, eq128_e1637_d_b44, eq128_e1637_d_b45, eq128_e1637_d_b46, eq128_e1637_d_b47, eq128_e1637_d_b48, eq128_e1637_d_b49, eq128_e1637_d_b50, eq128_e1637_d_b51, eq128_e1637_d_b52, eq128_e1637_d_b53, eq128_e1637_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq128_value),
            &eq128_node_derivatives,
            &eq128_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1650, eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22, eq129_e1650_d_b0, eq129_e1650_d_b1, eq129_e1650_d_b2, eq129_e1650_d_b3, eq129_e1650_d_b4, eq129_e1650_d_b5, eq129_e1650_d_b6, eq129_e1650_d_b7, eq129_e1650_d_b8, eq129_e1650_d_b9, eq129_e1650_d_b10, eq129_e1650_d_b11, eq129_e1650_d_b12, eq129_e1650_d_b13, eq129_e1650_d_b14, eq129_e1650_d_b15, eq129_e1650_d_b16, eq129_e1650_d_b17, eq129_e1650_d_b18, eq129_e1650_d_b19, eq129_e1650_d_b20, eq129_e1650_d_b21, eq129_e1650_d_b22, eq129_e1650_d_b23, eq129_e1650_d_b24, eq129_e1650_d_b25, eq129_e1650_d_b26, eq129_e1650_d_b27, eq129_e1650_d_b28, eq129_e1650_d_b29, eq129_e1650_d_b30, eq129_e1650_d_b31, eq129_e1650_d_b32, eq129_e1650_d_b33, eq129_e1650_d_b34, eq129_e1650_d_b35, eq129_e1650_d_b36, eq129_e1650_d_b37, eq129_e1650_d_b38, eq129_e1650_d_b39, eq129_e1650_d_b40, eq129_e1650_d_b41, eq129_e1650_d_b42, eq129_e1650_d_b43, eq129_e1650_d_b44, eq129_e1650_d_b45, eq129_e1650_d_b46, eq129_e1650_d_b47, eq129_e1650_d_b48, eq129_e1650_d_b49, eq129_e1650_d_b50, eq129_e1650_d_b51, eq129_e1650_d_b52, eq129_e1650_d_b53, eq129_e1650_d_b54,) = {
    if (((!s.b[570]) && s.b[573]) && (!s.b[574])) {
        let eq129_e1647: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 28, s.v[228]);
        let eq129_e1648: f64 = (p.p7 * eq129_e1647);
        (eq129_e1648, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_value: f64 = eq129_e1650;
        let eq129_node_derivatives: [f64; 23] = [eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22];
        let eq129_branch_derivatives: [f64; 55] = [eq129_e1650_d_b0, eq129_e1650_d_b1, eq129_e1650_d_b2, eq129_e1650_d_b3, eq129_e1650_d_b4, eq129_e1650_d_b5, eq129_e1650_d_b6, eq129_e1650_d_b7, eq129_e1650_d_b8, eq129_e1650_d_b9, eq129_e1650_d_b10, eq129_e1650_d_b11, eq129_e1650_d_b12, eq129_e1650_d_b13, eq129_e1650_d_b14, eq129_e1650_d_b15, eq129_e1650_d_b16, eq129_e1650_d_b17, eq129_e1650_d_b18, eq129_e1650_d_b19, eq129_e1650_d_b20, eq129_e1650_d_b21, eq129_e1650_d_b22, eq129_e1650_d_b23, eq129_e1650_d_b24, eq129_e1650_d_b25, eq129_e1650_d_b26, eq129_e1650_d_b27, eq129_e1650_d_b28, eq129_e1650_d_b29, eq129_e1650_d_b30, eq129_e1650_d_b31, eq129_e1650_d_b32, eq129_e1650_d_b33, eq129_e1650_d_b34, eq129_e1650_d_b35, eq129_e1650_d_b36, eq129_e1650_d_b37, eq129_e1650_d_b38, eq129_e1650_d_b39, eq129_e1650_d_b40, eq129_e1650_d_b41, eq129_e1650_d_b42, eq129_e1650_d_b43, eq129_e1650_d_b44, eq129_e1650_d_b45, eq129_e1650_d_b46, eq129_e1650_d_b47, eq129_e1650_d_b48, eq129_e1650_d_b49, eq129_e1650_d_b50, eq129_e1650_d_b51, eq129_e1650_d_b52, eq129_e1650_d_b53, eq129_e1650_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq129_value),
            &eq129_node_derivatives,
            &eq129_branch_derivatives,
            multiplicity,
        );
    }
}
