#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_35(
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[288][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[288][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[288][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[288][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[288][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[288][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[288][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[288][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[288][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[288][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[288][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[288][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[288][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[288][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[288][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[288][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[288][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[288][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[288][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[288][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[288][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[288][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[288][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[288][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[288][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[288][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[288][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[288][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[288][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[288][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[288][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[288][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[288][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[288][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[288][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[288][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[288][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[288][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[288][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[288][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[288][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[288][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[288][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[288][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[288][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[288][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[288][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[288][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[288][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[288][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[288][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[288][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[288][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[288][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[288][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[288][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[288][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[288][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[288][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[288][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[288][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[288][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[288][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[288][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[288][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[288][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[288][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[288][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[288][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[288][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[288][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[288][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[288][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[288][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[288][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[288][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[288][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[288][54] * ddt_scale));
        let (eq181_e2281, eq181_e2281_d_n0, eq181_e2281_d_n1, eq181_e2281_d_n2, eq181_e2281_d_n3, eq181_e2281_d_n4, eq181_e2281_d_n5, eq181_e2281_d_n6, eq181_e2281_d_n7, eq181_e2281_d_n8, eq181_e2281_d_n9, eq181_e2281_d_n10, eq181_e2281_d_n11, eq181_e2281_d_n12, eq181_e2281_d_n13, eq181_e2281_d_n14, eq181_e2281_d_n15, eq181_e2281_d_n16, eq181_e2281_d_n17, eq181_e2281_d_n18, eq181_e2281_d_n19, eq181_e2281_d_n20, eq181_e2281_d_n21, eq181_e2281_d_n22, eq181_e2281_d_b0, eq181_e2281_d_b1, eq181_e2281_d_b2, eq181_e2281_d_b3, eq181_e2281_d_b4, eq181_e2281_d_b5, eq181_e2281_d_b6, eq181_e2281_d_b7, eq181_e2281_d_b8, eq181_e2281_d_b9, eq181_e2281_d_b10, eq181_e2281_d_b11, eq181_e2281_d_b12, eq181_e2281_d_b13, eq181_e2281_d_b14, eq181_e2281_d_b15, eq181_e2281_d_b16, eq181_e2281_d_b17, eq181_e2281_d_b18, eq181_e2281_d_b19, eq181_e2281_d_b20, eq181_e2281_d_b21, eq181_e2281_d_b22, eq181_e2281_d_b23, eq181_e2281_d_b24, eq181_e2281_d_b25, eq181_e2281_d_b26, eq181_e2281_d_b27, eq181_e2281_d_b28, eq181_e2281_d_b29, eq181_e2281_d_b30, eq181_e2281_d_b31, eq181_e2281_d_b32, eq181_e2281_d_b33, eq181_e2281_d_b34, eq181_e2281_d_b35, eq181_e2281_d_b36, eq181_e2281_d_b37, eq181_e2281_d_b38, eq181_e2281_d_b39, eq181_e2281_d_b40, eq181_e2281_d_b41, eq181_e2281_d_b42, eq181_e2281_d_b43, eq181_e2281_d_b44, eq181_e2281_d_b45, eq181_e2281_d_b46, eq181_e2281_d_b47, eq181_e2281_d_b48, eq181_e2281_d_b49, eq181_e2281_d_b50, eq181_e2281_d_b51, eq181_e2281_d_b52, eq181_e2281_d_b53, eq181_e2281_d_b54,) = {
    if ((s.b[595] && s.b[596]) && s.b[597]) {
        let eq181_e2278: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 80, s.v[288]);
        let eq181_e2279: f64 = (p.p7 * eq181_e2278);
        (eq181_e2279, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq181_value: f64 = eq181_e2281;
        let eq181_node_derivatives: [f64; 23] = [eq181_e2281_d_n0, eq181_e2281_d_n1, eq181_e2281_d_n2, eq181_e2281_d_n3, eq181_e2281_d_n4, eq181_e2281_d_n5, eq181_e2281_d_n6, eq181_e2281_d_n7, eq181_e2281_d_n8, eq181_e2281_d_n9, eq181_e2281_d_n10, eq181_e2281_d_n11, eq181_e2281_d_n12, eq181_e2281_d_n13, eq181_e2281_d_n14, eq181_e2281_d_n15, eq181_e2281_d_n16, eq181_e2281_d_n17, eq181_e2281_d_n18, eq181_e2281_d_n19, eq181_e2281_d_n20, eq181_e2281_d_n21, eq181_e2281_d_n22];
        let eq181_branch_derivatives: [f64; 55] = [eq181_e2281_d_b0, eq181_e2281_d_b1, eq181_e2281_d_b2, eq181_e2281_d_b3, eq181_e2281_d_b4, eq181_e2281_d_b5, eq181_e2281_d_b6, eq181_e2281_d_b7, eq181_e2281_d_b8, eq181_e2281_d_b9, eq181_e2281_d_b10, eq181_e2281_d_b11, eq181_e2281_d_b12, eq181_e2281_d_b13, eq181_e2281_d_b14, eq181_e2281_d_b15, eq181_e2281_d_b16, eq181_e2281_d_b17, eq181_e2281_d_b18, eq181_e2281_d_b19, eq181_e2281_d_b20, eq181_e2281_d_b21, eq181_e2281_d_b22, eq181_e2281_d_b23, eq181_e2281_d_b24, eq181_e2281_d_b25, eq181_e2281_d_b26, eq181_e2281_d_b27, eq181_e2281_d_b28, eq181_e2281_d_b29, eq181_e2281_d_b30, eq181_e2281_d_b31, eq181_e2281_d_b32, eq181_e2281_d_b33, eq181_e2281_d_b34, eq181_e2281_d_b35, eq181_e2281_d_b36, eq181_e2281_d_b37, eq181_e2281_d_b38, eq181_e2281_d_b39, eq181_e2281_d_b40, eq181_e2281_d_b41, eq181_e2281_d_b42, eq181_e2281_d_b43, eq181_e2281_d_b44, eq181_e2281_d_b45, eq181_e2281_d_b46, eq181_e2281_d_b47, eq181_e2281_d_b48, eq181_e2281_d_b49, eq181_e2281_d_b50, eq181_e2281_d_b51, eq181_e2281_d_b52, eq181_e2281_d_b53, eq181_e2281_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(21),
            multiplicity * (eq181_value),
            &eq181_node_derivatives,
            &eq181_branch_derivatives,
            multiplicity,
        );
        let (eq182_e2294, eq182_e2294_d_n0, eq182_e2294_d_n1, eq182_e2294_d_n2, eq182_e2294_d_n3, eq182_e2294_d_n4, eq182_e2294_d_n5, eq182_e2294_d_n6, eq182_e2294_d_n7, eq182_e2294_d_n8, eq182_e2294_d_n9, eq182_e2294_d_n10, eq182_e2294_d_n11, eq182_e2294_d_n12, eq182_e2294_d_n13, eq182_e2294_d_n14, eq182_e2294_d_n15, eq182_e2294_d_n16, eq182_e2294_d_n17, eq182_e2294_d_n18, eq182_e2294_d_n19, eq182_e2294_d_n20, eq182_e2294_d_n21, eq182_e2294_d_n22, eq182_e2294_d_b0, eq182_e2294_d_b1, eq182_e2294_d_b2, eq182_e2294_d_b3, eq182_e2294_d_b4, eq182_e2294_d_b5, eq182_e2294_d_b6, eq182_e2294_d_b7, eq182_e2294_d_b8, eq182_e2294_d_b9, eq182_e2294_d_b10, eq182_e2294_d_b11, eq182_e2294_d_b12, eq182_e2294_d_b13, eq182_e2294_d_b14, eq182_e2294_d_b15, eq182_e2294_d_b16, eq182_e2294_d_b17, eq182_e2294_d_b18, eq182_e2294_d_b19, eq182_e2294_d_b20, eq182_e2294_d_b21, eq182_e2294_d_b22, eq182_e2294_d_b23, eq182_e2294_d_b24, eq182_e2294_d_b25, eq182_e2294_d_b26, eq182_e2294_d_b27, eq182_e2294_d_b28, eq182_e2294_d_b29, eq182_e2294_d_b30, eq182_e2294_d_b31, eq182_e2294_d_b32, eq182_e2294_d_b33, eq182_e2294_d_b34, eq182_e2294_d_b35, eq182_e2294_d_b36, eq182_e2294_d_b37, eq182_e2294_d_b38, eq182_e2294_d_b39, eq182_e2294_d_b40, eq182_e2294_d_b41, eq182_e2294_d_b42, eq182_e2294_d_b43, eq182_e2294_d_b44, eq182_e2294_d_b45, eq182_e2294_d_b46, eq182_e2294_d_b47, eq182_e2294_d_b48, eq182_e2294_d_b49, eq182_e2294_d_b50, eq182_e2294_d_b51, eq182_e2294_d_b52, eq182_e2294_d_b53, eq182_e2294_d_b54,) = {
    if ((s.b[595] && s.b[596]) && s.b[597]) {
        let eq182_e2289: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 81, s.v[288]);
        let eq182_e2290: f64 = (p.p7 * eq182_e2289);
        let eq182_e2292: f64 = (eq182_e2290 * p.p248);
        let eq182_e2292_d_n0: f64 = (__rspice_deriv_cse_0 * p.p248);
        let eq182_e2292_d_n1: f64 = (__rspice_deriv_cse_1 * p.p248);
        let eq182_e2292_d_n2: f64 = (__rspice_deriv_cse_2 * p.p248);
        let eq182_e2292_d_n3: f64 = (__rspice_deriv_cse_3 * p.p248);
        let eq182_e2292_d_n4: f64 = (__rspice_deriv_cse_4 * p.p248);
        let eq182_e2292_d_n5: f64 = (__rspice_deriv_cse_5 * p.p248);
        let eq182_e2292_d_n6: f64 = (__rspice_deriv_cse_6 * p.p248);
        let eq182_e2292_d_n7: f64 = (__rspice_deriv_cse_7 * p.p248);
        let eq182_e2292_d_n8: f64 = (__rspice_deriv_cse_8 * p.p248);
        let eq182_e2292_d_n9: f64 = (__rspice_deriv_cse_9 * p.p248);
        let eq182_e2292_d_n10: f64 = (__rspice_deriv_cse_10 * p.p248);
        let eq182_e2292_d_n11: f64 = (__rspice_deriv_cse_11 * p.p248);
        let eq182_e2292_d_n12: f64 = (__rspice_deriv_cse_12 * p.p248);
        let eq182_e2292_d_n13: f64 = (__rspice_deriv_cse_13 * p.p248);
        let eq182_e2292_d_n14: f64 = (__rspice_deriv_cse_14 * p.p248);
        let eq182_e2292_d_n15: f64 = (__rspice_deriv_cse_15 * p.p248);
        let eq182_e2292_d_n16: f64 = (__rspice_deriv_cse_16 * p.p248);
        let eq182_e2292_d_n17: f64 = (__rspice_deriv_cse_17 * p.p248);
        let eq182_e2292_d_n18: f64 = (__rspice_deriv_cse_18 * p.p248);
        let eq182_e2292_d_n19: f64 = (__rspice_deriv_cse_19 * p.p248);
        let eq182_e2292_d_n20: f64 = (__rspice_deriv_cse_20 * p.p248);
        let eq182_e2292_d_n21: f64 = (__rspice_deriv_cse_21 * p.p248);
        let eq182_e2292_d_n22: f64 = (__rspice_deriv_cse_22 * p.p248);
        let eq182_e2292_d_b0: f64 = (__rspice_deriv_cse_23 * p.p248);
        let eq182_e2292_d_b1: f64 = (__rspice_deriv_cse_24 * p.p248);
        let eq182_e2292_d_b2: f64 = (__rspice_deriv_cse_25 * p.p248);
        let eq182_e2292_d_b3: f64 = (__rspice_deriv_cse_26 * p.p248);
        let eq182_e2292_d_b4: f64 = (__rspice_deriv_cse_27 * p.p248);
        let eq182_e2292_d_b5: f64 = (__rspice_deriv_cse_28 * p.p248);
        let eq182_e2292_d_b6: f64 = (__rspice_deriv_cse_29 * p.p248);
        let eq182_e2292_d_b7: f64 = (__rspice_deriv_cse_30 * p.p248);
        let eq182_e2292_d_b8: f64 = (__rspice_deriv_cse_31 * p.p248);
        let eq182_e2292_d_b9: f64 = (__rspice_deriv_cse_32 * p.p248);
        let eq182_e2292_d_b10: f64 = (__rspice_deriv_cse_33 * p.p248);
        let eq182_e2292_d_b11: f64 = (__rspice_deriv_cse_34 * p.p248);
        let eq182_e2292_d_b12: f64 = (__rspice_deriv_cse_35 * p.p248);
        let eq182_e2292_d_b13: f64 = (__rspice_deriv_cse_36 * p.p248);
        let eq182_e2292_d_b14: f64 = (__rspice_deriv_cse_37 * p.p248);
        let eq182_e2292_d_b15: f64 = (__rspice_deriv_cse_38 * p.p248);
        let eq182_e2292_d_b16: f64 = (__rspice_deriv_cse_39 * p.p248);
        let eq182_e2292_d_b17: f64 = (__rspice_deriv_cse_40 * p.p248);
        let eq182_e2292_d_b18: f64 = (__rspice_deriv_cse_41 * p.p248);
        let eq182_e2292_d_b19: f64 = (__rspice_deriv_cse_42 * p.p248);
        let eq182_e2292_d_b20: f64 = (__rspice_deriv_cse_43 * p.p248);
        let eq182_e2292_d_b21: f64 = (__rspice_deriv_cse_44 * p.p248);
        let eq182_e2292_d_b22: f64 = (__rspice_deriv_cse_45 * p.p248);
        let eq182_e2292_d_b23: f64 = (__rspice_deriv_cse_46 * p.p248);
        let eq182_e2292_d_b24: f64 = (__rspice_deriv_cse_47 * p.p248);
        let eq182_e2292_d_b25: f64 = (__rspice_deriv_cse_48 * p.p248);
        let eq182_e2292_d_b26: f64 = (__rspice_deriv_cse_49 * p.p248);
        let eq182_e2292_d_b27: f64 = (__rspice_deriv_cse_50 * p.p248);
        let eq182_e2292_d_b28: f64 = (__rspice_deriv_cse_51 * p.p248);
        let eq182_e2292_d_b29: f64 = (__rspice_deriv_cse_52 * p.p248);
        let eq182_e2292_d_b30: f64 = (__rspice_deriv_cse_53 * p.p248);
        let eq182_e2292_d_b31: f64 = (__rspice_deriv_cse_54 * p.p248);
        let eq182_e2292_d_b32: f64 = (__rspice_deriv_cse_55 * p.p248);
        let eq182_e2292_d_b33: f64 = (__rspice_deriv_cse_56 * p.p248);
        let eq182_e2292_d_b34: f64 = (__rspice_deriv_cse_57 * p.p248);
        let eq182_e2292_d_b35: f64 = (__rspice_deriv_cse_58 * p.p248);
        let eq182_e2292_d_b36: f64 = (__rspice_deriv_cse_59 * p.p248);
        let eq182_e2292_d_b37: f64 = (__rspice_deriv_cse_60 * p.p248);
        let eq182_e2292_d_b38: f64 = (__rspice_deriv_cse_61 * p.p248);
        let eq182_e2292_d_b39: f64 = (__rspice_deriv_cse_62 * p.p248);
        let eq182_e2292_d_b40: f64 = (__rspice_deriv_cse_63 * p.p248);
        let eq182_e2292_d_b41: f64 = (__rspice_deriv_cse_64 * p.p248);
        let eq182_e2292_d_b42: f64 = (__rspice_deriv_cse_65 * p.p248);
        let eq182_e2292_d_b43: f64 = (__rspice_deriv_cse_66 * p.p248);
        let eq182_e2292_d_b44: f64 = (__rspice_deriv_cse_67 * p.p248);
        let eq182_e2292_d_b45: f64 = (__rspice_deriv_cse_68 * p.p248);
        let eq182_e2292_d_b46: f64 = (__rspice_deriv_cse_69 * p.p248);
        let eq182_e2292_d_b47: f64 = (__rspice_deriv_cse_70 * p.p248);
        let eq182_e2292_d_b48: f64 = (__rspice_deriv_cse_71 * p.p248);
        let eq182_e2292_d_b49: f64 = (__rspice_deriv_cse_72 * p.p248);
        let eq182_e2292_d_b50: f64 = (__rspice_deriv_cse_73 * p.p248);
        let eq182_e2292_d_b51: f64 = (__rspice_deriv_cse_74 * p.p248);
        let eq182_e2292_d_b52: f64 = (__rspice_deriv_cse_75 * p.p248);
        let eq182_e2292_d_b53: f64 = (__rspice_deriv_cse_76 * p.p248);
        let eq182_e2292_d_b54: f64 = (__rspice_deriv_cse_77 * p.p248);
        (eq182_e2292, eq182_e2292_d_n0, eq182_e2292_d_n1, eq182_e2292_d_n2, eq182_e2292_d_n3, eq182_e2292_d_n4, eq182_e2292_d_n5, eq182_e2292_d_n6, eq182_e2292_d_n7, eq182_e2292_d_n8, eq182_e2292_d_n9, eq182_e2292_d_n10, eq182_e2292_d_n11, eq182_e2292_d_n12, eq182_e2292_d_n13, eq182_e2292_d_n14, eq182_e2292_d_n15, eq182_e2292_d_n16, eq182_e2292_d_n17, eq182_e2292_d_n18, eq182_e2292_d_n19, eq182_e2292_d_n20, eq182_e2292_d_n21, eq182_e2292_d_n22, eq182_e2292_d_b0, eq182_e2292_d_b1, eq182_e2292_d_b2, eq182_e2292_d_b3, eq182_e2292_d_b4, eq182_e2292_d_b5, eq182_e2292_d_b6, eq182_e2292_d_b7, eq182_e2292_d_b8, eq182_e2292_d_b9, eq182_e2292_d_b10, eq182_e2292_d_b11, eq182_e2292_d_b12, eq182_e2292_d_b13, eq182_e2292_d_b14, eq182_e2292_d_b15, eq182_e2292_d_b16, eq182_e2292_d_b17, eq182_e2292_d_b18, eq182_e2292_d_b19, eq182_e2292_d_b20, eq182_e2292_d_b21, eq182_e2292_d_b22, eq182_e2292_d_b23, eq182_e2292_d_b24, eq182_e2292_d_b25, eq182_e2292_d_b26, eq182_e2292_d_b27, eq182_e2292_d_b28, eq182_e2292_d_b29, eq182_e2292_d_b30, eq182_e2292_d_b31, eq182_e2292_d_b32, eq182_e2292_d_b33, eq182_e2292_d_b34, eq182_e2292_d_b35, eq182_e2292_d_b36, eq182_e2292_d_b37, eq182_e2292_d_b38, eq182_e2292_d_b39, eq182_e2292_d_b40, eq182_e2292_d_b41, eq182_e2292_d_b42, eq182_e2292_d_b43, eq182_e2292_d_b44, eq182_e2292_d_b45, eq182_e2292_d_b46, eq182_e2292_d_b47, eq182_e2292_d_b48, eq182_e2292_d_b49, eq182_e2292_d_b50, eq182_e2292_d_b51, eq182_e2292_d_b52, eq182_e2292_d_b53, eq182_e2292_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq182_value: f64 = eq182_e2294;
        let eq182_node_derivatives: [f64; 23] = [eq182_e2294_d_n0, eq182_e2294_d_n1, eq182_e2294_d_n2, eq182_e2294_d_n3, eq182_e2294_d_n4, eq182_e2294_d_n5, eq182_e2294_d_n6, eq182_e2294_d_n7, eq182_e2294_d_n8, eq182_e2294_d_n9, eq182_e2294_d_n10, eq182_e2294_d_n11, eq182_e2294_d_n12, eq182_e2294_d_n13, eq182_e2294_d_n14, eq182_e2294_d_n15, eq182_e2294_d_n16, eq182_e2294_d_n17, eq182_e2294_d_n18, eq182_e2294_d_n19, eq182_e2294_d_n20, eq182_e2294_d_n21, eq182_e2294_d_n22];
        let eq182_branch_derivatives: [f64; 55] = [eq182_e2294_d_b0, eq182_e2294_d_b1, eq182_e2294_d_b2, eq182_e2294_d_b3, eq182_e2294_d_b4, eq182_e2294_d_b5, eq182_e2294_d_b6, eq182_e2294_d_b7, eq182_e2294_d_b8, eq182_e2294_d_b9, eq182_e2294_d_b10, eq182_e2294_d_b11, eq182_e2294_d_b12, eq182_e2294_d_b13, eq182_e2294_d_b14, eq182_e2294_d_b15, eq182_e2294_d_b16, eq182_e2294_d_b17, eq182_e2294_d_b18, eq182_e2294_d_b19, eq182_e2294_d_b20, eq182_e2294_d_b21, eq182_e2294_d_b22, eq182_e2294_d_b23, eq182_e2294_d_b24, eq182_e2294_d_b25, eq182_e2294_d_b26, eq182_e2294_d_b27, eq182_e2294_d_b28, eq182_e2294_d_b29, eq182_e2294_d_b30, eq182_e2294_d_b31, eq182_e2294_d_b32, eq182_e2294_d_b33, eq182_e2294_d_b34, eq182_e2294_d_b35, eq182_e2294_d_b36, eq182_e2294_d_b37, eq182_e2294_d_b38, eq182_e2294_d_b39, eq182_e2294_d_b40, eq182_e2294_d_b41, eq182_e2294_d_b42, eq182_e2294_d_b43, eq182_e2294_d_b44, eq182_e2294_d_b45, eq182_e2294_d_b46, eq182_e2294_d_b47, eq182_e2294_d_b48, eq182_e2294_d_b49, eq182_e2294_d_b50, eq182_e2294_d_b51, eq182_e2294_d_b52, eq182_e2294_d_b53, eq182_e2294_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(21),
            multiplicity * (eq182_value),
            &eq182_node_derivatives,
            &eq182_branch_derivatives,
            multiplicity,
        );
        let (eq183_e2306, eq183_e2306_d_n0, eq183_e2306_d_n1, eq183_e2306_d_n2, eq183_e2306_d_n3, eq183_e2306_d_n4, eq183_e2306_d_n5, eq183_e2306_d_n6, eq183_e2306_d_n7, eq183_e2306_d_n8, eq183_e2306_d_n9, eq183_e2306_d_n10, eq183_e2306_d_n11, eq183_e2306_d_n12, eq183_e2306_d_n13, eq183_e2306_d_n14, eq183_e2306_d_n15, eq183_e2306_d_n16, eq183_e2306_d_n17, eq183_e2306_d_n18, eq183_e2306_d_n19, eq183_e2306_d_n20, eq183_e2306_d_n21, eq183_e2306_d_n22, eq183_e2306_d_b0, eq183_e2306_d_b1, eq183_e2306_d_b2, eq183_e2306_d_b3, eq183_e2306_d_b4, eq183_e2306_d_b5, eq183_e2306_d_b6, eq183_e2306_d_b7, eq183_e2306_d_b8, eq183_e2306_d_b9, eq183_e2306_d_b10, eq183_e2306_d_b11, eq183_e2306_d_b12, eq183_e2306_d_b13, eq183_e2306_d_b14, eq183_e2306_d_b15, eq183_e2306_d_b16, eq183_e2306_d_b17, eq183_e2306_d_b18, eq183_e2306_d_b19, eq183_e2306_d_b20, eq183_e2306_d_b21, eq183_e2306_d_b22, eq183_e2306_d_b23, eq183_e2306_d_b24, eq183_e2306_d_b25, eq183_e2306_d_b26, eq183_e2306_d_b27, eq183_e2306_d_b28, eq183_e2306_d_b29, eq183_e2306_d_b30, eq183_e2306_d_b31, eq183_e2306_d_b32, eq183_e2306_d_b33, eq183_e2306_d_b34, eq183_e2306_d_b35, eq183_e2306_d_b36, eq183_e2306_d_b37, eq183_e2306_d_b38, eq183_e2306_d_b39, eq183_e2306_d_b40, eq183_e2306_d_b41, eq183_e2306_d_b42, eq183_e2306_d_b43, eq183_e2306_d_b44, eq183_e2306_d_b45, eq183_e2306_d_b46, eq183_e2306_d_b47, eq183_e2306_d_b48, eq183_e2306_d_b49, eq183_e2306_d_b50, eq183_e2306_d_b51, eq183_e2306_d_b52, eq183_e2306_d_b53, eq183_e2306_d_b54,) = {
    if ((s.b[595] && s.b[596]) && (!s.b[597])) {
        let eq183_e2303: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 82, s.v[288]);
        let eq183_e2304: f64 = (p.p7 * eq183_e2303);
        (eq183_e2304, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq183_value: f64 = eq183_e2306;
        let eq183_node_derivatives: [f64; 23] = [eq183_e2306_d_n0, eq183_e2306_d_n1, eq183_e2306_d_n2, eq183_e2306_d_n3, eq183_e2306_d_n4, eq183_e2306_d_n5, eq183_e2306_d_n6, eq183_e2306_d_n7, eq183_e2306_d_n8, eq183_e2306_d_n9, eq183_e2306_d_n10, eq183_e2306_d_n11, eq183_e2306_d_n12, eq183_e2306_d_n13, eq183_e2306_d_n14, eq183_e2306_d_n15, eq183_e2306_d_n16, eq183_e2306_d_n17, eq183_e2306_d_n18, eq183_e2306_d_n19, eq183_e2306_d_n20, eq183_e2306_d_n21, eq183_e2306_d_n22];
        let eq183_branch_derivatives: [f64; 55] = [eq183_e2306_d_b0, eq183_e2306_d_b1, eq183_e2306_d_b2, eq183_e2306_d_b3, eq183_e2306_d_b4, eq183_e2306_d_b5, eq183_e2306_d_b6, eq183_e2306_d_b7, eq183_e2306_d_b8, eq183_e2306_d_b9, eq183_e2306_d_b10, eq183_e2306_d_b11, eq183_e2306_d_b12, eq183_e2306_d_b13, eq183_e2306_d_b14, eq183_e2306_d_b15, eq183_e2306_d_b16, eq183_e2306_d_b17, eq183_e2306_d_b18, eq183_e2306_d_b19, eq183_e2306_d_b20, eq183_e2306_d_b21, eq183_e2306_d_b22, eq183_e2306_d_b23, eq183_e2306_d_b24, eq183_e2306_d_b25, eq183_e2306_d_b26, eq183_e2306_d_b27, eq183_e2306_d_b28, eq183_e2306_d_b29, eq183_e2306_d_b30, eq183_e2306_d_b31, eq183_e2306_d_b32, eq183_e2306_d_b33, eq183_e2306_d_b34, eq183_e2306_d_b35, eq183_e2306_d_b36, eq183_e2306_d_b37, eq183_e2306_d_b38, eq183_e2306_d_b39, eq183_e2306_d_b40, eq183_e2306_d_b41, eq183_e2306_d_b42, eq183_e2306_d_b43, eq183_e2306_d_b44, eq183_e2306_d_b45, eq183_e2306_d_b46, eq183_e2306_d_b47, eq183_e2306_d_b48, eq183_e2306_d_b49, eq183_e2306_d_b50, eq183_e2306_d_b51, eq183_e2306_d_b52, eq183_e2306_d_b53, eq183_e2306_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(21),
            multiplicity * (eq183_value),
            &eq183_node_derivatives,
            &eq183_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_36(
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
        let (eq184_e2320, eq184_e2320_d_n0, eq184_e2320_d_n1, eq184_e2320_d_n2, eq184_e2320_d_n3, eq184_e2320_d_n4, eq184_e2320_d_n5, eq184_e2320_d_n6, eq184_e2320_d_n7, eq184_e2320_d_n8, eq184_e2320_d_n9, eq184_e2320_d_n10, eq184_e2320_d_n11, eq184_e2320_d_n12, eq184_e2320_d_n13, eq184_e2320_d_n14, eq184_e2320_d_n15, eq184_e2320_d_n16, eq184_e2320_d_n17, eq184_e2320_d_n18, eq184_e2320_d_n19, eq184_e2320_d_n20, eq184_e2320_d_n21, eq184_e2320_d_n22, eq184_e2320_d_b0, eq184_e2320_d_b1, eq184_e2320_d_b2, eq184_e2320_d_b3, eq184_e2320_d_b4, eq184_e2320_d_b5, eq184_e2320_d_b6, eq184_e2320_d_b7, eq184_e2320_d_b8, eq184_e2320_d_b9, eq184_e2320_d_b10, eq184_e2320_d_b11, eq184_e2320_d_b12, eq184_e2320_d_b13, eq184_e2320_d_b14, eq184_e2320_d_b15, eq184_e2320_d_b16, eq184_e2320_d_b17, eq184_e2320_d_b18, eq184_e2320_d_b19, eq184_e2320_d_b20, eq184_e2320_d_b21, eq184_e2320_d_b22, eq184_e2320_d_b23, eq184_e2320_d_b24, eq184_e2320_d_b25, eq184_e2320_d_b26, eq184_e2320_d_b27, eq184_e2320_d_b28, eq184_e2320_d_b29, eq184_e2320_d_b30, eq184_e2320_d_b31, eq184_e2320_d_b32, eq184_e2320_d_b33, eq184_e2320_d_b34, eq184_e2320_d_b35, eq184_e2320_d_b36, eq184_e2320_d_b37, eq184_e2320_d_b38, eq184_e2320_d_b39, eq184_e2320_d_b40, eq184_e2320_d_b41, eq184_e2320_d_b42, eq184_e2320_d_b43, eq184_e2320_d_b44, eq184_e2320_d_b45, eq184_e2320_d_b46, eq184_e2320_d_b47, eq184_e2320_d_b48, eq184_e2320_d_b49, eq184_e2320_d_b50, eq184_e2320_d_b51, eq184_e2320_d_b52, eq184_e2320_d_b53, eq184_e2320_d_b54,) = {
    if ((s.b[595] && s.b[596]) && (!s.b[597])) {
        let eq184_e2315: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 83, s.v[288]);
        let eq184_e2316: f64 = (p.p7 * eq184_e2315);
        let eq184_e2316_d_n0: f64 = (p.p7 * (s.dn[288][0] * ddt_scale));
        let eq184_e2316_d_n1: f64 = (p.p7 * (s.dn[288][1] * ddt_scale));
        let eq184_e2316_d_n2: f64 = (p.p7 * (s.dn[288][2] * ddt_scale));
        let eq184_e2316_d_n3: f64 = (p.p7 * (s.dn[288][3] * ddt_scale));
        let eq184_e2316_d_n4: f64 = (p.p7 * (s.dn[288][4] * ddt_scale));
        let eq184_e2316_d_n5: f64 = (p.p7 * (s.dn[288][5] * ddt_scale));
        let eq184_e2316_d_n6: f64 = (p.p7 * (s.dn[288][6] * ddt_scale));
        let eq184_e2316_d_n7: f64 = (p.p7 * (s.dn[288][7] * ddt_scale));
        let eq184_e2316_d_n8: f64 = (p.p7 * (s.dn[288][8] * ddt_scale));
        let eq184_e2316_d_n9: f64 = (p.p7 * (s.dn[288][9] * ddt_scale));
        let eq184_e2316_d_n10: f64 = (p.p7 * (s.dn[288][10] * ddt_scale));
        let eq184_e2316_d_n11: f64 = (p.p7 * (s.dn[288][11] * ddt_scale));
        let eq184_e2316_d_n12: f64 = (p.p7 * (s.dn[288][12] * ddt_scale));
        let eq184_e2316_d_n13: f64 = (p.p7 * (s.dn[288][13] * ddt_scale));
        let eq184_e2316_d_n14: f64 = (p.p7 * (s.dn[288][14] * ddt_scale));
        let eq184_e2316_d_n15: f64 = (p.p7 * (s.dn[288][15] * ddt_scale));
        let eq184_e2316_d_n16: f64 = (p.p7 * (s.dn[288][16] * ddt_scale));
        let eq184_e2316_d_n17: f64 = (p.p7 * (s.dn[288][17] * ddt_scale));
        let eq184_e2316_d_n18: f64 = (p.p7 * (s.dn[288][18] * ddt_scale));
        let eq184_e2316_d_n19: f64 = (p.p7 * (s.dn[288][19] * ddt_scale));
        let eq184_e2316_d_n20: f64 = (p.p7 * (s.dn[288][20] * ddt_scale));
        let eq184_e2316_d_n21: f64 = (p.p7 * (s.dn[288][21] * ddt_scale));
        let eq184_e2316_d_n22: f64 = (p.p7 * (s.dn[288][22] * ddt_scale));
        let eq184_e2316_d_b0: f64 = (p.p7 * (s.db[288][0] * ddt_scale));
        let eq184_e2316_d_b1: f64 = (p.p7 * (s.db[288][1] * ddt_scale));
        let eq184_e2316_d_b2: f64 = (p.p7 * (s.db[288][2] * ddt_scale));
        let eq184_e2316_d_b3: f64 = (p.p7 * (s.db[288][3] * ddt_scale));
        let eq184_e2316_d_b4: f64 = (p.p7 * (s.db[288][4] * ddt_scale));
        let eq184_e2316_d_b5: f64 = (p.p7 * (s.db[288][5] * ddt_scale));
        let eq184_e2316_d_b6: f64 = (p.p7 * (s.db[288][6] * ddt_scale));
        let eq184_e2316_d_b7: f64 = (p.p7 * (s.db[288][7] * ddt_scale));
        let eq184_e2316_d_b8: f64 = (p.p7 * (s.db[288][8] * ddt_scale));
        let eq184_e2316_d_b9: f64 = (p.p7 * (s.db[288][9] * ddt_scale));
        let eq184_e2316_d_b10: f64 = (p.p7 * (s.db[288][10] * ddt_scale));
        let eq184_e2316_d_b11: f64 = (p.p7 * (s.db[288][11] * ddt_scale));
        let eq184_e2316_d_b12: f64 = (p.p7 * (s.db[288][12] * ddt_scale));
        let eq184_e2316_d_b13: f64 = (p.p7 * (s.db[288][13] * ddt_scale));
        let eq184_e2316_d_b14: f64 = (p.p7 * (s.db[288][14] * ddt_scale));
        let eq184_e2316_d_b15: f64 = (p.p7 * (s.db[288][15] * ddt_scale));
        let eq184_e2316_d_b16: f64 = (p.p7 * (s.db[288][16] * ddt_scale));
        let eq184_e2316_d_b17: f64 = (p.p7 * (s.db[288][17] * ddt_scale));
        let eq184_e2316_d_b18: f64 = (p.p7 * (s.db[288][18] * ddt_scale));
        let eq184_e2316_d_b19: f64 = (p.p7 * (s.db[288][19] * ddt_scale));
        let eq184_e2316_d_b20: f64 = (p.p7 * (s.db[288][20] * ddt_scale));
        let eq184_e2316_d_b21: f64 = (p.p7 * (s.db[288][21] * ddt_scale));
        let eq184_e2316_d_b22: f64 = (p.p7 * (s.db[288][22] * ddt_scale));
        let eq184_e2316_d_b23: f64 = (p.p7 * (s.db[288][23] * ddt_scale));
        let eq184_e2316_d_b24: f64 = (p.p7 * (s.db[288][24] * ddt_scale));
        let eq184_e2316_d_b25: f64 = (p.p7 * (s.db[288][25] * ddt_scale));
        let eq184_e2316_d_b26: f64 = (p.p7 * (s.db[288][26] * ddt_scale));
        let eq184_e2316_d_b27: f64 = (p.p7 * (s.db[288][27] * ddt_scale));
        let eq184_e2316_d_b28: f64 = (p.p7 * (s.db[288][28] * ddt_scale));
        let eq184_e2316_d_b29: f64 = (p.p7 * (s.db[288][29] * ddt_scale));
        let eq184_e2316_d_b30: f64 = (p.p7 * (s.db[288][30] * ddt_scale));
        let eq184_e2316_d_b31: f64 = (p.p7 * (s.db[288][31] * ddt_scale));
        let eq184_e2316_d_b32: f64 = (p.p7 * (s.db[288][32] * ddt_scale));
        let eq184_e2316_d_b33: f64 = (p.p7 * (s.db[288][33] * ddt_scale));
        let eq184_e2316_d_b34: f64 = (p.p7 * (s.db[288][34] * ddt_scale));
        let eq184_e2316_d_b35: f64 = (p.p7 * (s.db[288][35] * ddt_scale));
        let eq184_e2316_d_b36: f64 = (p.p7 * (s.db[288][36] * ddt_scale));
        let eq184_e2316_d_b37: f64 = (p.p7 * (s.db[288][37] * ddt_scale));
        let eq184_e2316_d_b38: f64 = (p.p7 * (s.db[288][38] * ddt_scale));
        let eq184_e2316_d_b39: f64 = (p.p7 * (s.db[288][39] * ddt_scale));
        let eq184_e2316_d_b40: f64 = (p.p7 * (s.db[288][40] * ddt_scale));
        let eq184_e2316_d_b41: f64 = (p.p7 * (s.db[288][41] * ddt_scale));
        let eq184_e2316_d_b42: f64 = (p.p7 * (s.db[288][42] * ddt_scale));
        let eq184_e2316_d_b43: f64 = (p.p7 * (s.db[288][43] * ddt_scale));
        let eq184_e2316_d_b44: f64 = (p.p7 * (s.db[288][44] * ddt_scale));
        let eq184_e2316_d_b45: f64 = (p.p7 * (s.db[288][45] * ddt_scale));
        let eq184_e2316_d_b46: f64 = (p.p7 * (s.db[288][46] * ddt_scale));
        let eq184_e2316_d_b47: f64 = (p.p7 * (s.db[288][47] * ddt_scale));
        let eq184_e2316_d_b48: f64 = (p.p7 * (s.db[288][48] * ddt_scale));
        let eq184_e2316_d_b49: f64 = (p.p7 * (s.db[288][49] * ddt_scale));
        let eq184_e2316_d_b50: f64 = (p.p7 * (s.db[288][50] * ddt_scale));
        let eq184_e2316_d_b51: f64 = (p.p7 * (s.db[288][51] * ddt_scale));
        let eq184_e2316_d_b52: f64 = (p.p7 * (s.db[288][52] * ddt_scale));
        let eq184_e2316_d_b53: f64 = (p.p7 * (s.db[288][53] * ddt_scale));
        let eq184_e2316_d_b54: f64 = (p.p7 * (s.db[288][54] * ddt_scale));
        let eq184_e2318: f64 = (eq184_e2316 * p.p248);
        let eq184_e2318_d_n0: f64 = (eq184_e2316_d_n0 * p.p248);
        let eq184_e2318_d_n1: f64 = (eq184_e2316_d_n1 * p.p248);
        let eq184_e2318_d_n2: f64 = (eq184_e2316_d_n2 * p.p248);
        let eq184_e2318_d_n3: f64 = (eq184_e2316_d_n3 * p.p248);
        let eq184_e2318_d_n4: f64 = (eq184_e2316_d_n4 * p.p248);
        let eq184_e2318_d_n5: f64 = (eq184_e2316_d_n5 * p.p248);
        let eq184_e2318_d_n6: f64 = (eq184_e2316_d_n6 * p.p248);
        let eq184_e2318_d_n7: f64 = (eq184_e2316_d_n7 * p.p248);
        let eq184_e2318_d_n8: f64 = (eq184_e2316_d_n8 * p.p248);
        let eq184_e2318_d_n9: f64 = (eq184_e2316_d_n9 * p.p248);
        let eq184_e2318_d_n10: f64 = (eq184_e2316_d_n10 * p.p248);
        let eq184_e2318_d_n11: f64 = (eq184_e2316_d_n11 * p.p248);
        let eq184_e2318_d_n12: f64 = (eq184_e2316_d_n12 * p.p248);
        let eq184_e2318_d_n13: f64 = (eq184_e2316_d_n13 * p.p248);
        let eq184_e2318_d_n14: f64 = (eq184_e2316_d_n14 * p.p248);
        let eq184_e2318_d_n15: f64 = (eq184_e2316_d_n15 * p.p248);
        let eq184_e2318_d_n16: f64 = (eq184_e2316_d_n16 * p.p248);
        let eq184_e2318_d_n17: f64 = (eq184_e2316_d_n17 * p.p248);
        let eq184_e2318_d_n18: f64 = (eq184_e2316_d_n18 * p.p248);
        let eq184_e2318_d_n19: f64 = (eq184_e2316_d_n19 * p.p248);
        let eq184_e2318_d_n20: f64 = (eq184_e2316_d_n20 * p.p248);
        let eq184_e2318_d_n21: f64 = (eq184_e2316_d_n21 * p.p248);
        let eq184_e2318_d_n22: f64 = (eq184_e2316_d_n22 * p.p248);
        let eq184_e2318_d_b0: f64 = (eq184_e2316_d_b0 * p.p248);
        let eq184_e2318_d_b1: f64 = (eq184_e2316_d_b1 * p.p248);
        let eq184_e2318_d_b2: f64 = (eq184_e2316_d_b2 * p.p248);
        let eq184_e2318_d_b3: f64 = (eq184_e2316_d_b3 * p.p248);
        let eq184_e2318_d_b4: f64 = (eq184_e2316_d_b4 * p.p248);
        let eq184_e2318_d_b5: f64 = (eq184_e2316_d_b5 * p.p248);
        let eq184_e2318_d_b6: f64 = (eq184_e2316_d_b6 * p.p248);
        let eq184_e2318_d_b7: f64 = (eq184_e2316_d_b7 * p.p248);
        let eq184_e2318_d_b8: f64 = (eq184_e2316_d_b8 * p.p248);
        let eq184_e2318_d_b9: f64 = (eq184_e2316_d_b9 * p.p248);
        let eq184_e2318_d_b10: f64 = (eq184_e2316_d_b10 * p.p248);
        let eq184_e2318_d_b11: f64 = (eq184_e2316_d_b11 * p.p248);
        let eq184_e2318_d_b12: f64 = (eq184_e2316_d_b12 * p.p248);
        let eq184_e2318_d_b13: f64 = (eq184_e2316_d_b13 * p.p248);
        let eq184_e2318_d_b14: f64 = (eq184_e2316_d_b14 * p.p248);
        let eq184_e2318_d_b15: f64 = (eq184_e2316_d_b15 * p.p248);
        let eq184_e2318_d_b16: f64 = (eq184_e2316_d_b16 * p.p248);
        let eq184_e2318_d_b17: f64 = (eq184_e2316_d_b17 * p.p248);
        let eq184_e2318_d_b18: f64 = (eq184_e2316_d_b18 * p.p248);
        let eq184_e2318_d_b19: f64 = (eq184_e2316_d_b19 * p.p248);
        let eq184_e2318_d_b20: f64 = (eq184_e2316_d_b20 * p.p248);
        let eq184_e2318_d_b21: f64 = (eq184_e2316_d_b21 * p.p248);
        let eq184_e2318_d_b22: f64 = (eq184_e2316_d_b22 * p.p248);
        let eq184_e2318_d_b23: f64 = (eq184_e2316_d_b23 * p.p248);
        let eq184_e2318_d_b24: f64 = (eq184_e2316_d_b24 * p.p248);
        let eq184_e2318_d_b25: f64 = (eq184_e2316_d_b25 * p.p248);
        let eq184_e2318_d_b26: f64 = (eq184_e2316_d_b26 * p.p248);
        let eq184_e2318_d_b27: f64 = (eq184_e2316_d_b27 * p.p248);
        let eq184_e2318_d_b28: f64 = (eq184_e2316_d_b28 * p.p248);
        let eq184_e2318_d_b29: f64 = (eq184_e2316_d_b29 * p.p248);
        let eq184_e2318_d_b30: f64 = (eq184_e2316_d_b30 * p.p248);
        let eq184_e2318_d_b31: f64 = (eq184_e2316_d_b31 * p.p248);
        let eq184_e2318_d_b32: f64 = (eq184_e2316_d_b32 * p.p248);
        let eq184_e2318_d_b33: f64 = (eq184_e2316_d_b33 * p.p248);
        let eq184_e2318_d_b34: f64 = (eq184_e2316_d_b34 * p.p248);
        let eq184_e2318_d_b35: f64 = (eq184_e2316_d_b35 * p.p248);
        let eq184_e2318_d_b36: f64 = (eq184_e2316_d_b36 * p.p248);
        let eq184_e2318_d_b37: f64 = (eq184_e2316_d_b37 * p.p248);
        let eq184_e2318_d_b38: f64 = (eq184_e2316_d_b38 * p.p248);
        let eq184_e2318_d_b39: f64 = (eq184_e2316_d_b39 * p.p248);
        let eq184_e2318_d_b40: f64 = (eq184_e2316_d_b40 * p.p248);
        let eq184_e2318_d_b41: f64 = (eq184_e2316_d_b41 * p.p248);
        let eq184_e2318_d_b42: f64 = (eq184_e2316_d_b42 * p.p248);
        let eq184_e2318_d_b43: f64 = (eq184_e2316_d_b43 * p.p248);
        let eq184_e2318_d_b44: f64 = (eq184_e2316_d_b44 * p.p248);
        let eq184_e2318_d_b45: f64 = (eq184_e2316_d_b45 * p.p248);
        let eq184_e2318_d_b46: f64 = (eq184_e2316_d_b46 * p.p248);
        let eq184_e2318_d_b47: f64 = (eq184_e2316_d_b47 * p.p248);
        let eq184_e2318_d_b48: f64 = (eq184_e2316_d_b48 * p.p248);
        let eq184_e2318_d_b49: f64 = (eq184_e2316_d_b49 * p.p248);
        let eq184_e2318_d_b50: f64 = (eq184_e2316_d_b50 * p.p248);
        let eq184_e2318_d_b51: f64 = (eq184_e2316_d_b51 * p.p248);
        let eq184_e2318_d_b52: f64 = (eq184_e2316_d_b52 * p.p248);
        let eq184_e2318_d_b53: f64 = (eq184_e2316_d_b53 * p.p248);
        let eq184_e2318_d_b54: f64 = (eq184_e2316_d_b54 * p.p248);
        (eq184_e2318, eq184_e2318_d_n0, eq184_e2318_d_n1, eq184_e2318_d_n2, eq184_e2318_d_n3, eq184_e2318_d_n4, eq184_e2318_d_n5, eq184_e2318_d_n6, eq184_e2318_d_n7, eq184_e2318_d_n8, eq184_e2318_d_n9, eq184_e2318_d_n10, eq184_e2318_d_n11, eq184_e2318_d_n12, eq184_e2318_d_n13, eq184_e2318_d_n14, eq184_e2318_d_n15, eq184_e2318_d_n16, eq184_e2318_d_n17, eq184_e2318_d_n18, eq184_e2318_d_n19, eq184_e2318_d_n20, eq184_e2318_d_n21, eq184_e2318_d_n22, eq184_e2318_d_b0, eq184_e2318_d_b1, eq184_e2318_d_b2, eq184_e2318_d_b3, eq184_e2318_d_b4, eq184_e2318_d_b5, eq184_e2318_d_b6, eq184_e2318_d_b7, eq184_e2318_d_b8, eq184_e2318_d_b9, eq184_e2318_d_b10, eq184_e2318_d_b11, eq184_e2318_d_b12, eq184_e2318_d_b13, eq184_e2318_d_b14, eq184_e2318_d_b15, eq184_e2318_d_b16, eq184_e2318_d_b17, eq184_e2318_d_b18, eq184_e2318_d_b19, eq184_e2318_d_b20, eq184_e2318_d_b21, eq184_e2318_d_b22, eq184_e2318_d_b23, eq184_e2318_d_b24, eq184_e2318_d_b25, eq184_e2318_d_b26, eq184_e2318_d_b27, eq184_e2318_d_b28, eq184_e2318_d_b29, eq184_e2318_d_b30, eq184_e2318_d_b31, eq184_e2318_d_b32, eq184_e2318_d_b33, eq184_e2318_d_b34, eq184_e2318_d_b35, eq184_e2318_d_b36, eq184_e2318_d_b37, eq184_e2318_d_b38, eq184_e2318_d_b39, eq184_e2318_d_b40, eq184_e2318_d_b41, eq184_e2318_d_b42, eq184_e2318_d_b43, eq184_e2318_d_b44, eq184_e2318_d_b45, eq184_e2318_d_b46, eq184_e2318_d_b47, eq184_e2318_d_b48, eq184_e2318_d_b49, eq184_e2318_d_b50, eq184_e2318_d_b51, eq184_e2318_d_b52, eq184_e2318_d_b53, eq184_e2318_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq184_value: f64 = eq184_e2320;
        let eq184_node_derivatives: [f64; 23] = [eq184_e2320_d_n0, eq184_e2320_d_n1, eq184_e2320_d_n2, eq184_e2320_d_n3, eq184_e2320_d_n4, eq184_e2320_d_n5, eq184_e2320_d_n6, eq184_e2320_d_n7, eq184_e2320_d_n8, eq184_e2320_d_n9, eq184_e2320_d_n10, eq184_e2320_d_n11, eq184_e2320_d_n12, eq184_e2320_d_n13, eq184_e2320_d_n14, eq184_e2320_d_n15, eq184_e2320_d_n16, eq184_e2320_d_n17, eq184_e2320_d_n18, eq184_e2320_d_n19, eq184_e2320_d_n20, eq184_e2320_d_n21, eq184_e2320_d_n22];
        let eq184_branch_derivatives: [f64; 55] = [eq184_e2320_d_b0, eq184_e2320_d_b1, eq184_e2320_d_b2, eq184_e2320_d_b3, eq184_e2320_d_b4, eq184_e2320_d_b5, eq184_e2320_d_b6, eq184_e2320_d_b7, eq184_e2320_d_b8, eq184_e2320_d_b9, eq184_e2320_d_b10, eq184_e2320_d_b11, eq184_e2320_d_b12, eq184_e2320_d_b13, eq184_e2320_d_b14, eq184_e2320_d_b15, eq184_e2320_d_b16, eq184_e2320_d_b17, eq184_e2320_d_b18, eq184_e2320_d_b19, eq184_e2320_d_b20, eq184_e2320_d_b21, eq184_e2320_d_b22, eq184_e2320_d_b23, eq184_e2320_d_b24, eq184_e2320_d_b25, eq184_e2320_d_b26, eq184_e2320_d_b27, eq184_e2320_d_b28, eq184_e2320_d_b29, eq184_e2320_d_b30, eq184_e2320_d_b31, eq184_e2320_d_b32, eq184_e2320_d_b33, eq184_e2320_d_b34, eq184_e2320_d_b35, eq184_e2320_d_b36, eq184_e2320_d_b37, eq184_e2320_d_b38, eq184_e2320_d_b39, eq184_e2320_d_b40, eq184_e2320_d_b41, eq184_e2320_d_b42, eq184_e2320_d_b43, eq184_e2320_d_b44, eq184_e2320_d_b45, eq184_e2320_d_b46, eq184_e2320_d_b47, eq184_e2320_d_b48, eq184_e2320_d_b49, eq184_e2320_d_b50, eq184_e2320_d_b51, eq184_e2320_d_b52, eq184_e2320_d_b53, eq184_e2320_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(21),
            multiplicity * (eq184_value),
            &eq184_node_derivatives,
            &eq184_branch_derivatives,
            multiplicity,
        );
        let (eq185_e2331, eq185_e2331_d_n0, eq185_e2331_d_n1, eq185_e2331_d_n2, eq185_e2331_d_n3, eq185_e2331_d_n4, eq185_e2331_d_n5, eq185_e2331_d_n6, eq185_e2331_d_n7, eq185_e2331_d_n8, eq185_e2331_d_n9, eq185_e2331_d_n10, eq185_e2331_d_n11, eq185_e2331_d_n12, eq185_e2331_d_n13, eq185_e2331_d_n14, eq185_e2331_d_n15, eq185_e2331_d_n16, eq185_e2331_d_n17, eq185_e2331_d_n18, eq185_e2331_d_n19, eq185_e2331_d_n20, eq185_e2331_d_n21, eq185_e2331_d_n22, eq185_e2331_d_b0, eq185_e2331_d_b1, eq185_e2331_d_b2, eq185_e2331_d_b3, eq185_e2331_d_b4, eq185_e2331_d_b5, eq185_e2331_d_b6, eq185_e2331_d_b7, eq185_e2331_d_b8, eq185_e2331_d_b9, eq185_e2331_d_b10, eq185_e2331_d_b11, eq185_e2331_d_b12, eq185_e2331_d_b13, eq185_e2331_d_b14, eq185_e2331_d_b15, eq185_e2331_d_b16, eq185_e2331_d_b17, eq185_e2331_d_b18, eq185_e2331_d_b19, eq185_e2331_d_b20, eq185_e2331_d_b21, eq185_e2331_d_b22, eq185_e2331_d_b23, eq185_e2331_d_b24, eq185_e2331_d_b25, eq185_e2331_d_b26, eq185_e2331_d_b27, eq185_e2331_d_b28, eq185_e2331_d_b29, eq185_e2331_d_b30, eq185_e2331_d_b31, eq185_e2331_d_b32, eq185_e2331_d_b33, eq185_e2331_d_b34, eq185_e2331_d_b35, eq185_e2331_d_b36, eq185_e2331_d_b37, eq185_e2331_d_b38, eq185_e2331_d_b39, eq185_e2331_d_b40, eq185_e2331_d_b41, eq185_e2331_d_b42, eq185_e2331_d_b43, eq185_e2331_d_b44, eq185_e2331_d_b45, eq185_e2331_d_b46, eq185_e2331_d_b47, eq185_e2331_d_b48, eq185_e2331_d_b49, eq185_e2331_d_b50, eq185_e2331_d_b51, eq185_e2331_d_b52, eq185_e2331_d_b53, eq185_e2331_d_b54,) = {
    if (s.b[595] && s.b[596]) {
        let eq185_e2327: f64 = (p.p253 * s.v[288]);
        let eq185_e2328: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 84, eq185_e2327);
        let eq185_e2328_d_n0: f64 = ((p.p253 * s.dn[288][0]) * ddt_scale);
        let eq185_e2328_d_n1: f64 = ((p.p253 * s.dn[288][1]) * ddt_scale);
        let eq185_e2328_d_n2: f64 = ((p.p253 * s.dn[288][2]) * ddt_scale);
        let eq185_e2328_d_n3: f64 = ((p.p253 * s.dn[288][3]) * ddt_scale);
        let eq185_e2328_d_n4: f64 = ((p.p253 * s.dn[288][4]) * ddt_scale);
        let eq185_e2328_d_n5: f64 = ((p.p253 * s.dn[288][5]) * ddt_scale);
        let eq185_e2328_d_n6: f64 = ((p.p253 * s.dn[288][6]) * ddt_scale);
        let eq185_e2328_d_n7: f64 = ((p.p253 * s.dn[288][7]) * ddt_scale);
        let eq185_e2328_d_n8: f64 = ((p.p253 * s.dn[288][8]) * ddt_scale);
        let eq185_e2328_d_n9: f64 = ((p.p253 * s.dn[288][9]) * ddt_scale);
        let eq185_e2328_d_n10: f64 = ((p.p253 * s.dn[288][10]) * ddt_scale);
        let eq185_e2328_d_n11: f64 = ((p.p253 * s.dn[288][11]) * ddt_scale);
        let eq185_e2328_d_n12: f64 = ((p.p253 * s.dn[288][12]) * ddt_scale);
        let eq185_e2328_d_n13: f64 = ((p.p253 * s.dn[288][13]) * ddt_scale);
        let eq185_e2328_d_n14: f64 = ((p.p253 * s.dn[288][14]) * ddt_scale);
        let eq185_e2328_d_n15: f64 = ((p.p253 * s.dn[288][15]) * ddt_scale);
        let eq185_e2328_d_n16: f64 = ((p.p253 * s.dn[288][16]) * ddt_scale);
        let eq185_e2328_d_n17: f64 = ((p.p253 * s.dn[288][17]) * ddt_scale);
        let eq185_e2328_d_n18: f64 = ((p.p253 * s.dn[288][18]) * ddt_scale);
        let eq185_e2328_d_n19: f64 = ((p.p253 * s.dn[288][19]) * ddt_scale);
        let eq185_e2328_d_n20: f64 = ((p.p253 * s.dn[288][20]) * ddt_scale);
        let eq185_e2328_d_n21: f64 = ((p.p253 * s.dn[288][21]) * ddt_scale);
        let eq185_e2328_d_n22: f64 = ((p.p253 * s.dn[288][22]) * ddt_scale);
        let eq185_e2328_d_b0: f64 = ((p.p253 * s.db[288][0]) * ddt_scale);
        let eq185_e2328_d_b1: f64 = ((p.p253 * s.db[288][1]) * ddt_scale);
        let eq185_e2328_d_b2: f64 = ((p.p253 * s.db[288][2]) * ddt_scale);
        let eq185_e2328_d_b3: f64 = ((p.p253 * s.db[288][3]) * ddt_scale);
        let eq185_e2328_d_b4: f64 = ((p.p253 * s.db[288][4]) * ddt_scale);
        let eq185_e2328_d_b5: f64 = ((p.p253 * s.db[288][5]) * ddt_scale);
        let eq185_e2328_d_b6: f64 = ((p.p253 * s.db[288][6]) * ddt_scale);
        let eq185_e2328_d_b7: f64 = ((p.p253 * s.db[288][7]) * ddt_scale);
        let eq185_e2328_d_b8: f64 = ((p.p253 * s.db[288][8]) * ddt_scale);
        let eq185_e2328_d_b9: f64 = ((p.p253 * s.db[288][9]) * ddt_scale);
        let eq185_e2328_d_b10: f64 = ((p.p253 * s.db[288][10]) * ddt_scale);
        let eq185_e2328_d_b11: f64 = ((p.p253 * s.db[288][11]) * ddt_scale);
        let eq185_e2328_d_b12: f64 = ((p.p253 * s.db[288][12]) * ddt_scale);
        let eq185_e2328_d_b13: f64 = ((p.p253 * s.db[288][13]) * ddt_scale);
        let eq185_e2328_d_b14: f64 = ((p.p253 * s.db[288][14]) * ddt_scale);
        let eq185_e2328_d_b15: f64 = ((p.p253 * s.db[288][15]) * ddt_scale);
        let eq185_e2328_d_b16: f64 = ((p.p253 * s.db[288][16]) * ddt_scale);
        let eq185_e2328_d_b17: f64 = ((p.p253 * s.db[288][17]) * ddt_scale);
        let eq185_e2328_d_b18: f64 = ((p.p253 * s.db[288][18]) * ddt_scale);
        let eq185_e2328_d_b19: f64 = ((p.p253 * s.db[288][19]) * ddt_scale);
        let eq185_e2328_d_b20: f64 = ((p.p253 * s.db[288][20]) * ddt_scale);
        let eq185_e2328_d_b21: f64 = ((p.p253 * s.db[288][21]) * ddt_scale);
        let eq185_e2328_d_b22: f64 = ((p.p253 * s.db[288][22]) * ddt_scale);
        let eq185_e2328_d_b23: f64 = ((p.p253 * s.db[288][23]) * ddt_scale);
        let eq185_e2328_d_b24: f64 = ((p.p253 * s.db[288][24]) * ddt_scale);
        let eq185_e2328_d_b25: f64 = ((p.p253 * s.db[288][25]) * ddt_scale);
        let eq185_e2328_d_b26: f64 = ((p.p253 * s.db[288][26]) * ddt_scale);
        let eq185_e2328_d_b27: f64 = ((p.p253 * s.db[288][27]) * ddt_scale);
        let eq185_e2328_d_b28: f64 = ((p.p253 * s.db[288][28]) * ddt_scale);
        let eq185_e2328_d_b29: f64 = ((p.p253 * s.db[288][29]) * ddt_scale);
        let eq185_e2328_d_b30: f64 = ((p.p253 * s.db[288][30]) * ddt_scale);
        let eq185_e2328_d_b31: f64 = ((p.p253 * s.db[288][31]) * ddt_scale);
        let eq185_e2328_d_b32: f64 = ((p.p253 * s.db[288][32]) * ddt_scale);
        let eq185_e2328_d_b33: f64 = ((p.p253 * s.db[288][33]) * ddt_scale);
        let eq185_e2328_d_b34: f64 = ((p.p253 * s.db[288][34]) * ddt_scale);
        let eq185_e2328_d_b35: f64 = ((p.p253 * s.db[288][35]) * ddt_scale);
        let eq185_e2328_d_b36: f64 = ((p.p253 * s.db[288][36]) * ddt_scale);
        let eq185_e2328_d_b37: f64 = ((p.p253 * s.db[288][37]) * ddt_scale);
        let eq185_e2328_d_b38: f64 = ((p.p253 * s.db[288][38]) * ddt_scale);
        let eq185_e2328_d_b39: f64 = ((p.p253 * s.db[288][39]) * ddt_scale);
        let eq185_e2328_d_b40: f64 = ((p.p253 * s.db[288][40]) * ddt_scale);
        let eq185_e2328_d_b41: f64 = ((p.p253 * s.db[288][41]) * ddt_scale);
        let eq185_e2328_d_b42: f64 = ((p.p253 * s.db[288][42]) * ddt_scale);
        let eq185_e2328_d_b43: f64 = ((p.p253 * s.db[288][43]) * ddt_scale);
        let eq185_e2328_d_b44: f64 = ((p.p253 * s.db[288][44]) * ddt_scale);
        let eq185_e2328_d_b45: f64 = ((p.p253 * s.db[288][45]) * ddt_scale);
        let eq185_e2328_d_b46: f64 = ((p.p253 * s.db[288][46]) * ddt_scale);
        let eq185_e2328_d_b47: f64 = ((p.p253 * s.db[288][47]) * ddt_scale);
        let eq185_e2328_d_b48: f64 = ((p.p253 * s.db[288][48]) * ddt_scale);
        let eq185_e2328_d_b49: f64 = ((p.p253 * s.db[288][49]) * ddt_scale);
        let eq185_e2328_d_b50: f64 = ((p.p253 * s.db[288][50]) * ddt_scale);
        let eq185_e2328_d_b51: f64 = ((p.p253 * s.db[288][51]) * ddt_scale);
        let eq185_e2328_d_b52: f64 = ((p.p253 * s.db[288][52]) * ddt_scale);
        let eq185_e2328_d_b53: f64 = ((p.p253 * s.db[288][53]) * ddt_scale);
        let eq185_e2328_d_b54: f64 = ((p.p253 * s.db[288][54]) * ddt_scale);
        let eq185_e2329: f64 = (p.p7 * eq185_e2328);
        let eq185_e2329_d_n0: f64 = (p.p7 * eq185_e2328_d_n0);
        let eq185_e2329_d_n1: f64 = (p.p7 * eq185_e2328_d_n1);
        let eq185_e2329_d_n2: f64 = (p.p7 * eq185_e2328_d_n2);
        let eq185_e2329_d_n3: f64 = (p.p7 * eq185_e2328_d_n3);
        let eq185_e2329_d_n4: f64 = (p.p7 * eq185_e2328_d_n4);
        let eq185_e2329_d_n5: f64 = (p.p7 * eq185_e2328_d_n5);
        let eq185_e2329_d_n6: f64 = (p.p7 * eq185_e2328_d_n6);
        let eq185_e2329_d_n7: f64 = (p.p7 * eq185_e2328_d_n7);
        let eq185_e2329_d_n8: f64 = (p.p7 * eq185_e2328_d_n8);
        let eq185_e2329_d_n9: f64 = (p.p7 * eq185_e2328_d_n9);
        let eq185_e2329_d_n10: f64 = (p.p7 * eq185_e2328_d_n10);
        let eq185_e2329_d_n11: f64 = (p.p7 * eq185_e2328_d_n11);
        let eq185_e2329_d_n12: f64 = (p.p7 * eq185_e2328_d_n12);
        let eq185_e2329_d_n13: f64 = (p.p7 * eq185_e2328_d_n13);
        let eq185_e2329_d_n14: f64 = (p.p7 * eq185_e2328_d_n14);
        let eq185_e2329_d_n15: f64 = (p.p7 * eq185_e2328_d_n15);
        let eq185_e2329_d_n16: f64 = (p.p7 * eq185_e2328_d_n16);
        let eq185_e2329_d_n17: f64 = (p.p7 * eq185_e2328_d_n17);
        let eq185_e2329_d_n18: f64 = (p.p7 * eq185_e2328_d_n18);
        let eq185_e2329_d_n19: f64 = (p.p7 * eq185_e2328_d_n19);
        let eq185_e2329_d_n20: f64 = (p.p7 * eq185_e2328_d_n20);
        let eq185_e2329_d_n21: f64 = (p.p7 * eq185_e2328_d_n21);
        let eq185_e2329_d_n22: f64 = (p.p7 * eq185_e2328_d_n22);
        let eq185_e2329_d_b0: f64 = (p.p7 * eq185_e2328_d_b0);
        let eq185_e2329_d_b1: f64 = (p.p7 * eq185_e2328_d_b1);
        let eq185_e2329_d_b2: f64 = (p.p7 * eq185_e2328_d_b2);
        let eq185_e2329_d_b3: f64 = (p.p7 * eq185_e2328_d_b3);
        let eq185_e2329_d_b4: f64 = (p.p7 * eq185_e2328_d_b4);
        let eq185_e2329_d_b5: f64 = (p.p7 * eq185_e2328_d_b5);
        let eq185_e2329_d_b6: f64 = (p.p7 * eq185_e2328_d_b6);
        let eq185_e2329_d_b7: f64 = (p.p7 * eq185_e2328_d_b7);
        let eq185_e2329_d_b8: f64 = (p.p7 * eq185_e2328_d_b8);
        let eq185_e2329_d_b9: f64 = (p.p7 * eq185_e2328_d_b9);
        let eq185_e2329_d_b10: f64 = (p.p7 * eq185_e2328_d_b10);
        let eq185_e2329_d_b11: f64 = (p.p7 * eq185_e2328_d_b11);
        let eq185_e2329_d_b12: f64 = (p.p7 * eq185_e2328_d_b12);
        let eq185_e2329_d_b13: f64 = (p.p7 * eq185_e2328_d_b13);
        let eq185_e2329_d_b14: f64 = (p.p7 * eq185_e2328_d_b14);
        let eq185_e2329_d_b15: f64 = (p.p7 * eq185_e2328_d_b15);
        let eq185_e2329_d_b16: f64 = (p.p7 * eq185_e2328_d_b16);
        let eq185_e2329_d_b17: f64 = (p.p7 * eq185_e2328_d_b17);
        let eq185_e2329_d_b18: f64 = (p.p7 * eq185_e2328_d_b18);
        let eq185_e2329_d_b19: f64 = (p.p7 * eq185_e2328_d_b19);
        let eq185_e2329_d_b20: f64 = (p.p7 * eq185_e2328_d_b20);
        let eq185_e2329_d_b21: f64 = (p.p7 * eq185_e2328_d_b21);
        let eq185_e2329_d_b22: f64 = (p.p7 * eq185_e2328_d_b22);
        let eq185_e2329_d_b23: f64 = (p.p7 * eq185_e2328_d_b23);
        let eq185_e2329_d_b24: f64 = (p.p7 * eq185_e2328_d_b24);
        let eq185_e2329_d_b25: f64 = (p.p7 * eq185_e2328_d_b25);
        let eq185_e2329_d_b26: f64 = (p.p7 * eq185_e2328_d_b26);
        let eq185_e2329_d_b27: f64 = (p.p7 * eq185_e2328_d_b27);
        let eq185_e2329_d_b28: f64 = (p.p7 * eq185_e2328_d_b28);
        let eq185_e2329_d_b29: f64 = (p.p7 * eq185_e2328_d_b29);
        let eq185_e2329_d_b30: f64 = (p.p7 * eq185_e2328_d_b30);
        let eq185_e2329_d_b31: f64 = (p.p7 * eq185_e2328_d_b31);
        let eq185_e2329_d_b32: f64 = (p.p7 * eq185_e2328_d_b32);
        let eq185_e2329_d_b33: f64 = (p.p7 * eq185_e2328_d_b33);
        let eq185_e2329_d_b34: f64 = (p.p7 * eq185_e2328_d_b34);
        let eq185_e2329_d_b35: f64 = (p.p7 * eq185_e2328_d_b35);
        let eq185_e2329_d_b36: f64 = (p.p7 * eq185_e2328_d_b36);
        let eq185_e2329_d_b37: f64 = (p.p7 * eq185_e2328_d_b37);
        let eq185_e2329_d_b38: f64 = (p.p7 * eq185_e2328_d_b38);
        let eq185_e2329_d_b39: f64 = (p.p7 * eq185_e2328_d_b39);
        let eq185_e2329_d_b40: f64 = (p.p7 * eq185_e2328_d_b40);
        let eq185_e2329_d_b41: f64 = (p.p7 * eq185_e2328_d_b41);
        let eq185_e2329_d_b42: f64 = (p.p7 * eq185_e2328_d_b42);
        let eq185_e2329_d_b43: f64 = (p.p7 * eq185_e2328_d_b43);
        let eq185_e2329_d_b44: f64 = (p.p7 * eq185_e2328_d_b44);
        let eq185_e2329_d_b45: f64 = (p.p7 * eq185_e2328_d_b45);
        let eq185_e2329_d_b46: f64 = (p.p7 * eq185_e2328_d_b46);
        let eq185_e2329_d_b47: f64 = (p.p7 * eq185_e2328_d_b47);
        let eq185_e2329_d_b48: f64 = (p.p7 * eq185_e2328_d_b48);
        let eq185_e2329_d_b49: f64 = (p.p7 * eq185_e2328_d_b49);
        let eq185_e2329_d_b50: f64 = (p.p7 * eq185_e2328_d_b50);
        let eq185_e2329_d_b51: f64 = (p.p7 * eq185_e2328_d_b51);
        let eq185_e2329_d_b52: f64 = (p.p7 * eq185_e2328_d_b52);
        let eq185_e2329_d_b53: f64 = (p.p7 * eq185_e2328_d_b53);
        let eq185_e2329_d_b54: f64 = (p.p7 * eq185_e2328_d_b54);
        (eq185_e2329, eq185_e2329_d_n0, eq185_e2329_d_n1, eq185_e2329_d_n2, eq185_e2329_d_n3, eq185_e2329_d_n4, eq185_e2329_d_n5, eq185_e2329_d_n6, eq185_e2329_d_n7, eq185_e2329_d_n8, eq185_e2329_d_n9, eq185_e2329_d_n10, eq185_e2329_d_n11, eq185_e2329_d_n12, eq185_e2329_d_n13, eq185_e2329_d_n14, eq185_e2329_d_n15, eq185_e2329_d_n16, eq185_e2329_d_n17, eq185_e2329_d_n18, eq185_e2329_d_n19, eq185_e2329_d_n20, eq185_e2329_d_n21, eq185_e2329_d_n22, eq185_e2329_d_b0, eq185_e2329_d_b1, eq185_e2329_d_b2, eq185_e2329_d_b3, eq185_e2329_d_b4, eq185_e2329_d_b5, eq185_e2329_d_b6, eq185_e2329_d_b7, eq185_e2329_d_b8, eq185_e2329_d_b9, eq185_e2329_d_b10, eq185_e2329_d_b11, eq185_e2329_d_b12, eq185_e2329_d_b13, eq185_e2329_d_b14, eq185_e2329_d_b15, eq185_e2329_d_b16, eq185_e2329_d_b17, eq185_e2329_d_b18, eq185_e2329_d_b19, eq185_e2329_d_b20, eq185_e2329_d_b21, eq185_e2329_d_b22, eq185_e2329_d_b23, eq185_e2329_d_b24, eq185_e2329_d_b25, eq185_e2329_d_b26, eq185_e2329_d_b27, eq185_e2329_d_b28, eq185_e2329_d_b29, eq185_e2329_d_b30, eq185_e2329_d_b31, eq185_e2329_d_b32, eq185_e2329_d_b33, eq185_e2329_d_b34, eq185_e2329_d_b35, eq185_e2329_d_b36, eq185_e2329_d_b37, eq185_e2329_d_b38, eq185_e2329_d_b39, eq185_e2329_d_b40, eq185_e2329_d_b41, eq185_e2329_d_b42, eq185_e2329_d_b43, eq185_e2329_d_b44, eq185_e2329_d_b45, eq185_e2329_d_b46, eq185_e2329_d_b47, eq185_e2329_d_b48, eq185_e2329_d_b49, eq185_e2329_d_b50, eq185_e2329_d_b51, eq185_e2329_d_b52, eq185_e2329_d_b53, eq185_e2329_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq185_value: f64 = eq185_e2331;
        let eq185_node_derivatives: [f64; 23] = [eq185_e2331_d_n0, eq185_e2331_d_n1, eq185_e2331_d_n2, eq185_e2331_d_n3, eq185_e2331_d_n4, eq185_e2331_d_n5, eq185_e2331_d_n6, eq185_e2331_d_n7, eq185_e2331_d_n8, eq185_e2331_d_n9, eq185_e2331_d_n10, eq185_e2331_d_n11, eq185_e2331_d_n12, eq185_e2331_d_n13, eq185_e2331_d_n14, eq185_e2331_d_n15, eq185_e2331_d_n16, eq185_e2331_d_n17, eq185_e2331_d_n18, eq185_e2331_d_n19, eq185_e2331_d_n20, eq185_e2331_d_n21, eq185_e2331_d_n22];
        let eq185_branch_derivatives: [f64; 55] = [eq185_e2331_d_b0, eq185_e2331_d_b1, eq185_e2331_d_b2, eq185_e2331_d_b3, eq185_e2331_d_b4, eq185_e2331_d_b5, eq185_e2331_d_b6, eq185_e2331_d_b7, eq185_e2331_d_b8, eq185_e2331_d_b9, eq185_e2331_d_b10, eq185_e2331_d_b11, eq185_e2331_d_b12, eq185_e2331_d_b13, eq185_e2331_d_b14, eq185_e2331_d_b15, eq185_e2331_d_b16, eq185_e2331_d_b17, eq185_e2331_d_b18, eq185_e2331_d_b19, eq185_e2331_d_b20, eq185_e2331_d_b21, eq185_e2331_d_b22, eq185_e2331_d_b23, eq185_e2331_d_b24, eq185_e2331_d_b25, eq185_e2331_d_b26, eq185_e2331_d_b27, eq185_e2331_d_b28, eq185_e2331_d_b29, eq185_e2331_d_b30, eq185_e2331_d_b31, eq185_e2331_d_b32, eq185_e2331_d_b33, eq185_e2331_d_b34, eq185_e2331_d_b35, eq185_e2331_d_b36, eq185_e2331_d_b37, eq185_e2331_d_b38, eq185_e2331_d_b39, eq185_e2331_d_b40, eq185_e2331_d_b41, eq185_e2331_d_b42, eq185_e2331_d_b43, eq185_e2331_d_b44, eq185_e2331_d_b45, eq185_e2331_d_b46, eq185_e2331_d_b47, eq185_e2331_d_b48, eq185_e2331_d_b49, eq185_e2331_d_b50, eq185_e2331_d_b51, eq185_e2331_d_b52, eq185_e2331_d_b53, eq185_e2331_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(21),
            multiplicity * (eq185_value),
            &eq185_node_derivatives,
            &eq185_branch_derivatives,
            multiplicity,
        );
        let (eq186_e2341, eq186_e2341_d_n0, eq186_e2341_d_n1, eq186_e2341_d_n2, eq186_e2341_d_n3, eq186_e2341_d_n4, eq186_e2341_d_n5, eq186_e2341_d_n6, eq186_e2341_d_n7, eq186_e2341_d_n8, eq186_e2341_d_n9, eq186_e2341_d_n10, eq186_e2341_d_n11, eq186_e2341_d_n12, eq186_e2341_d_n13, eq186_e2341_d_n14, eq186_e2341_d_n15, eq186_e2341_d_n16, eq186_e2341_d_n17, eq186_e2341_d_n18, eq186_e2341_d_n19, eq186_e2341_d_n20, eq186_e2341_d_n21, eq186_e2341_d_n22, eq186_e2341_d_b0, eq186_e2341_d_b1, eq186_e2341_d_b2, eq186_e2341_d_b3, eq186_e2341_d_b4, eq186_e2341_d_b5, eq186_e2341_d_b6, eq186_e2341_d_b7, eq186_e2341_d_b8, eq186_e2341_d_b9, eq186_e2341_d_b10, eq186_e2341_d_b11, eq186_e2341_d_b12, eq186_e2341_d_b13, eq186_e2341_d_b14, eq186_e2341_d_b15, eq186_e2341_d_b16, eq186_e2341_d_b17, eq186_e2341_d_b18, eq186_e2341_d_b19, eq186_e2341_d_b20, eq186_e2341_d_b21, eq186_e2341_d_b22, eq186_e2341_d_b23, eq186_e2341_d_b24, eq186_e2341_d_b25, eq186_e2341_d_b26, eq186_e2341_d_b27, eq186_e2341_d_b28, eq186_e2341_d_b29, eq186_e2341_d_b30, eq186_e2341_d_b31, eq186_e2341_d_b32, eq186_e2341_d_b33, eq186_e2341_d_b34, eq186_e2341_d_b35, eq186_e2341_d_b36, eq186_e2341_d_b37, eq186_e2341_d_b38, eq186_e2341_d_b39, eq186_e2341_d_b40, eq186_e2341_d_b41, eq186_e2341_d_b42, eq186_e2341_d_b43, eq186_e2341_d_b44, eq186_e2341_d_b45, eq186_e2341_d_b46, eq186_e2341_d_b47, eq186_e2341_d_b48, eq186_e2341_d_b49, eq186_e2341_d_b50, eq186_e2341_d_b51, eq186_e2341_d_b52, eq186_e2341_d_b53, eq186_e2341_d_b54,) = {
    if ((!s.b[595]) && s.b[598]) {
        let eq186_e2338: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 85, s.v[289]);
        let eq186_e2339: f64 = (p.p7 * eq186_e2338);
        let eq186_e2339_d_n0: f64 = (p.p7 * (s.dn[289][0] * ddt_scale));
        let eq186_e2339_d_n1: f64 = (p.p7 * (s.dn[289][1] * ddt_scale));
        let eq186_e2339_d_n2: f64 = (p.p7 * (s.dn[289][2] * ddt_scale));
        let eq186_e2339_d_n3: f64 = (p.p7 * (s.dn[289][3] * ddt_scale));
        let eq186_e2339_d_n4: f64 = (p.p7 * (s.dn[289][4] * ddt_scale));
        let eq186_e2339_d_n5: f64 = (p.p7 * (s.dn[289][5] * ddt_scale));
        let eq186_e2339_d_n6: f64 = (p.p7 * (s.dn[289][6] * ddt_scale));
        let eq186_e2339_d_n7: f64 = (p.p7 * (s.dn[289][7] * ddt_scale));
        let eq186_e2339_d_n8: f64 = (p.p7 * (s.dn[289][8] * ddt_scale));
        let eq186_e2339_d_n9: f64 = (p.p7 * (s.dn[289][9] * ddt_scale));
        let eq186_e2339_d_n10: f64 = (p.p7 * (s.dn[289][10] * ddt_scale));
        let eq186_e2339_d_n11: f64 = (p.p7 * (s.dn[289][11] * ddt_scale));
        let eq186_e2339_d_n12: f64 = (p.p7 * (s.dn[289][12] * ddt_scale));
        let eq186_e2339_d_n13: f64 = (p.p7 * (s.dn[289][13] * ddt_scale));
        let eq186_e2339_d_n14: f64 = (p.p7 * (s.dn[289][14] * ddt_scale));
        let eq186_e2339_d_n15: f64 = (p.p7 * (s.dn[289][15] * ddt_scale));
        let eq186_e2339_d_n16: f64 = (p.p7 * (s.dn[289][16] * ddt_scale));
        let eq186_e2339_d_n17: f64 = (p.p7 * (s.dn[289][17] * ddt_scale));
        let eq186_e2339_d_n18: f64 = (p.p7 * (s.dn[289][18] * ddt_scale));
        let eq186_e2339_d_n19: f64 = (p.p7 * (s.dn[289][19] * ddt_scale));
        let eq186_e2339_d_n20: f64 = (p.p7 * (s.dn[289][20] * ddt_scale));
        let eq186_e2339_d_n21: f64 = (p.p7 * (s.dn[289][21] * ddt_scale));
        let eq186_e2339_d_n22: f64 = (p.p7 * (s.dn[289][22] * ddt_scale));
        let eq186_e2339_d_b0: f64 = (p.p7 * (s.db[289][0] * ddt_scale));
        let eq186_e2339_d_b1: f64 = (p.p7 * (s.db[289][1] * ddt_scale));
        let eq186_e2339_d_b2: f64 = (p.p7 * (s.db[289][2] * ddt_scale));
        let eq186_e2339_d_b3: f64 = (p.p7 * (s.db[289][3] * ddt_scale));
        let eq186_e2339_d_b4: f64 = (p.p7 * (s.db[289][4] * ddt_scale));
        let eq186_e2339_d_b5: f64 = (p.p7 * (s.db[289][5] * ddt_scale));
        let eq186_e2339_d_b6: f64 = (p.p7 * (s.db[289][6] * ddt_scale));
        let eq186_e2339_d_b7: f64 = (p.p7 * (s.db[289][7] * ddt_scale));
        let eq186_e2339_d_b8: f64 = (p.p7 * (s.db[289][8] * ddt_scale));
        let eq186_e2339_d_b9: f64 = (p.p7 * (s.db[289][9] * ddt_scale));
        let eq186_e2339_d_b10: f64 = (p.p7 * (s.db[289][10] * ddt_scale));
        let eq186_e2339_d_b11: f64 = (p.p7 * (s.db[289][11] * ddt_scale));
        let eq186_e2339_d_b12: f64 = (p.p7 * (s.db[289][12] * ddt_scale));
        let eq186_e2339_d_b13: f64 = (p.p7 * (s.db[289][13] * ddt_scale));
        let eq186_e2339_d_b14: f64 = (p.p7 * (s.db[289][14] * ddt_scale));
        let eq186_e2339_d_b15: f64 = (p.p7 * (s.db[289][15] * ddt_scale));
        let eq186_e2339_d_b16: f64 = (p.p7 * (s.db[289][16] * ddt_scale));
        let eq186_e2339_d_b17: f64 = (p.p7 * (s.db[289][17] * ddt_scale));
        let eq186_e2339_d_b18: f64 = (p.p7 * (s.db[289][18] * ddt_scale));
        let eq186_e2339_d_b19: f64 = (p.p7 * (s.db[289][19] * ddt_scale));
        let eq186_e2339_d_b20: f64 = (p.p7 * (s.db[289][20] * ddt_scale));
        let eq186_e2339_d_b21: f64 = (p.p7 * (s.db[289][21] * ddt_scale));
        let eq186_e2339_d_b22: f64 = (p.p7 * (s.db[289][22] * ddt_scale));
        let eq186_e2339_d_b23: f64 = (p.p7 * (s.db[289][23] * ddt_scale));
        let eq186_e2339_d_b24: f64 = (p.p7 * (s.db[289][24] * ddt_scale));
        let eq186_e2339_d_b25: f64 = (p.p7 * (s.db[289][25] * ddt_scale));
        let eq186_e2339_d_b26: f64 = (p.p7 * (s.db[289][26] * ddt_scale));
        let eq186_e2339_d_b27: f64 = (p.p7 * (s.db[289][27] * ddt_scale));
        let eq186_e2339_d_b28: f64 = (p.p7 * (s.db[289][28] * ddt_scale));
        let eq186_e2339_d_b29: f64 = (p.p7 * (s.db[289][29] * ddt_scale));
        let eq186_e2339_d_b30: f64 = (p.p7 * (s.db[289][30] * ddt_scale));
        let eq186_e2339_d_b31: f64 = (p.p7 * (s.db[289][31] * ddt_scale));
        let eq186_e2339_d_b32: f64 = (p.p7 * (s.db[289][32] * ddt_scale));
        let eq186_e2339_d_b33: f64 = (p.p7 * (s.db[289][33] * ddt_scale));
        let eq186_e2339_d_b34: f64 = (p.p7 * (s.db[289][34] * ddt_scale));
        let eq186_e2339_d_b35: f64 = (p.p7 * (s.db[289][35] * ddt_scale));
        let eq186_e2339_d_b36: f64 = (p.p7 * (s.db[289][36] * ddt_scale));
        let eq186_e2339_d_b37: f64 = (p.p7 * (s.db[289][37] * ddt_scale));
        let eq186_e2339_d_b38: f64 = (p.p7 * (s.db[289][38] * ddt_scale));
        let eq186_e2339_d_b39: f64 = (p.p7 * (s.db[289][39] * ddt_scale));
        let eq186_e2339_d_b40: f64 = (p.p7 * (s.db[289][40] * ddt_scale));
        let eq186_e2339_d_b41: f64 = (p.p7 * (s.db[289][41] * ddt_scale));
        let eq186_e2339_d_b42: f64 = (p.p7 * (s.db[289][42] * ddt_scale));
        let eq186_e2339_d_b43: f64 = (p.p7 * (s.db[289][43] * ddt_scale));
        let eq186_e2339_d_b44: f64 = (p.p7 * (s.db[289][44] * ddt_scale));
        let eq186_e2339_d_b45: f64 = (p.p7 * (s.db[289][45] * ddt_scale));
        let eq186_e2339_d_b46: f64 = (p.p7 * (s.db[289][46] * ddt_scale));
        let eq186_e2339_d_b47: f64 = (p.p7 * (s.db[289][47] * ddt_scale));
        let eq186_e2339_d_b48: f64 = (p.p7 * (s.db[289][48] * ddt_scale));
        let eq186_e2339_d_b49: f64 = (p.p7 * (s.db[289][49] * ddt_scale));
        let eq186_e2339_d_b50: f64 = (p.p7 * (s.db[289][50] * ddt_scale));
        let eq186_e2339_d_b51: f64 = (p.p7 * (s.db[289][51] * ddt_scale));
        let eq186_e2339_d_b52: f64 = (p.p7 * (s.db[289][52] * ddt_scale));
        let eq186_e2339_d_b53: f64 = (p.p7 * (s.db[289][53] * ddt_scale));
        let eq186_e2339_d_b54: f64 = (p.p7 * (s.db[289][54] * ddt_scale));
        (eq186_e2339, eq186_e2339_d_n0, eq186_e2339_d_n1, eq186_e2339_d_n2, eq186_e2339_d_n3, eq186_e2339_d_n4, eq186_e2339_d_n5, eq186_e2339_d_n6, eq186_e2339_d_n7, eq186_e2339_d_n8, eq186_e2339_d_n9, eq186_e2339_d_n10, eq186_e2339_d_n11, eq186_e2339_d_n12, eq186_e2339_d_n13, eq186_e2339_d_n14, eq186_e2339_d_n15, eq186_e2339_d_n16, eq186_e2339_d_n17, eq186_e2339_d_n18, eq186_e2339_d_n19, eq186_e2339_d_n20, eq186_e2339_d_n21, eq186_e2339_d_n22, eq186_e2339_d_b0, eq186_e2339_d_b1, eq186_e2339_d_b2, eq186_e2339_d_b3, eq186_e2339_d_b4, eq186_e2339_d_b5, eq186_e2339_d_b6, eq186_e2339_d_b7, eq186_e2339_d_b8, eq186_e2339_d_b9, eq186_e2339_d_b10, eq186_e2339_d_b11, eq186_e2339_d_b12, eq186_e2339_d_b13, eq186_e2339_d_b14, eq186_e2339_d_b15, eq186_e2339_d_b16, eq186_e2339_d_b17, eq186_e2339_d_b18, eq186_e2339_d_b19, eq186_e2339_d_b20, eq186_e2339_d_b21, eq186_e2339_d_b22, eq186_e2339_d_b23, eq186_e2339_d_b24, eq186_e2339_d_b25, eq186_e2339_d_b26, eq186_e2339_d_b27, eq186_e2339_d_b28, eq186_e2339_d_b29, eq186_e2339_d_b30, eq186_e2339_d_b31, eq186_e2339_d_b32, eq186_e2339_d_b33, eq186_e2339_d_b34, eq186_e2339_d_b35, eq186_e2339_d_b36, eq186_e2339_d_b37, eq186_e2339_d_b38, eq186_e2339_d_b39, eq186_e2339_d_b40, eq186_e2339_d_b41, eq186_e2339_d_b42, eq186_e2339_d_b43, eq186_e2339_d_b44, eq186_e2339_d_b45, eq186_e2339_d_b46, eq186_e2339_d_b47, eq186_e2339_d_b48, eq186_e2339_d_b49, eq186_e2339_d_b50, eq186_e2339_d_b51, eq186_e2339_d_b52, eq186_e2339_d_b53, eq186_e2339_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq186_value: f64 = eq186_e2341;
        let eq186_node_derivatives: [f64; 23] = [eq186_e2341_d_n0, eq186_e2341_d_n1, eq186_e2341_d_n2, eq186_e2341_d_n3, eq186_e2341_d_n4, eq186_e2341_d_n5, eq186_e2341_d_n6, eq186_e2341_d_n7, eq186_e2341_d_n8, eq186_e2341_d_n9, eq186_e2341_d_n10, eq186_e2341_d_n11, eq186_e2341_d_n12, eq186_e2341_d_n13, eq186_e2341_d_n14, eq186_e2341_d_n15, eq186_e2341_d_n16, eq186_e2341_d_n17, eq186_e2341_d_n18, eq186_e2341_d_n19, eq186_e2341_d_n20, eq186_e2341_d_n21, eq186_e2341_d_n22];
        let eq186_branch_derivatives: [f64; 55] = [eq186_e2341_d_b0, eq186_e2341_d_b1, eq186_e2341_d_b2, eq186_e2341_d_b3, eq186_e2341_d_b4, eq186_e2341_d_b5, eq186_e2341_d_b6, eq186_e2341_d_b7, eq186_e2341_d_b8, eq186_e2341_d_b9, eq186_e2341_d_b10, eq186_e2341_d_b11, eq186_e2341_d_b12, eq186_e2341_d_b13, eq186_e2341_d_b14, eq186_e2341_d_b15, eq186_e2341_d_b16, eq186_e2341_d_b17, eq186_e2341_d_b18, eq186_e2341_d_b19, eq186_e2341_d_b20, eq186_e2341_d_b21, eq186_e2341_d_b22, eq186_e2341_d_b23, eq186_e2341_d_b24, eq186_e2341_d_b25, eq186_e2341_d_b26, eq186_e2341_d_b27, eq186_e2341_d_b28, eq186_e2341_d_b29, eq186_e2341_d_b30, eq186_e2341_d_b31, eq186_e2341_d_b32, eq186_e2341_d_b33, eq186_e2341_d_b34, eq186_e2341_d_b35, eq186_e2341_d_b36, eq186_e2341_d_b37, eq186_e2341_d_b38, eq186_e2341_d_b39, eq186_e2341_d_b40, eq186_e2341_d_b41, eq186_e2341_d_b42, eq186_e2341_d_b43, eq186_e2341_d_b44, eq186_e2341_d_b45, eq186_e2341_d_b46, eq186_e2341_d_b47, eq186_e2341_d_b48, eq186_e2341_d_b49, eq186_e2341_d_b50, eq186_e2341_d_b51, eq186_e2341_d_b52, eq186_e2341_d_b53, eq186_e2341_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq186_value),
            &eq186_node_derivatives,
            &eq186_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_37(
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[288][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[288][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[288][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[288][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[288][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[288][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[288][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[288][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[288][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[288][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[288][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[288][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[288][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[288][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[288][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[288][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[288][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[288][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[288][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[288][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[288][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[288][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[288][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[288][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[288][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[288][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[288][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[288][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[288][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[288][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[288][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[288][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[288][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[288][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[288][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[288][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[288][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[288][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[288][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[288][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[288][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[288][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[288][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[288][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[288][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[288][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[288][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[288][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[288][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[288][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[288][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[288][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[288][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[288][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[288][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[288][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[288][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[288][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[288][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[288][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[288][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[288][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[288][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[288][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[288][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[288][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[288][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[288][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[288][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[288][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[288][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[288][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[288][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[288][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[288][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[288][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[288][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[288][54] * ddt_scale));
        let (eq187_e2353, eq187_e2353_d_n0, eq187_e2353_d_n1, eq187_e2353_d_n2, eq187_e2353_d_n3, eq187_e2353_d_n4, eq187_e2353_d_n5, eq187_e2353_d_n6, eq187_e2353_d_n7, eq187_e2353_d_n8, eq187_e2353_d_n9, eq187_e2353_d_n10, eq187_e2353_d_n11, eq187_e2353_d_n12, eq187_e2353_d_n13, eq187_e2353_d_n14, eq187_e2353_d_n15, eq187_e2353_d_n16, eq187_e2353_d_n17, eq187_e2353_d_n18, eq187_e2353_d_n19, eq187_e2353_d_n20, eq187_e2353_d_n21, eq187_e2353_d_n22, eq187_e2353_d_b0, eq187_e2353_d_b1, eq187_e2353_d_b2, eq187_e2353_d_b3, eq187_e2353_d_b4, eq187_e2353_d_b5, eq187_e2353_d_b6, eq187_e2353_d_b7, eq187_e2353_d_b8, eq187_e2353_d_b9, eq187_e2353_d_b10, eq187_e2353_d_b11, eq187_e2353_d_b12, eq187_e2353_d_b13, eq187_e2353_d_b14, eq187_e2353_d_b15, eq187_e2353_d_b16, eq187_e2353_d_b17, eq187_e2353_d_b18, eq187_e2353_d_b19, eq187_e2353_d_b20, eq187_e2353_d_b21, eq187_e2353_d_b22, eq187_e2353_d_b23, eq187_e2353_d_b24, eq187_e2353_d_b25, eq187_e2353_d_b26, eq187_e2353_d_b27, eq187_e2353_d_b28, eq187_e2353_d_b29, eq187_e2353_d_b30, eq187_e2353_d_b31, eq187_e2353_d_b32, eq187_e2353_d_b33, eq187_e2353_d_b34, eq187_e2353_d_b35, eq187_e2353_d_b36, eq187_e2353_d_b37, eq187_e2353_d_b38, eq187_e2353_d_b39, eq187_e2353_d_b40, eq187_e2353_d_b41, eq187_e2353_d_b42, eq187_e2353_d_b43, eq187_e2353_d_b44, eq187_e2353_d_b45, eq187_e2353_d_b46, eq187_e2353_d_b47, eq187_e2353_d_b48, eq187_e2353_d_b49, eq187_e2353_d_b50, eq187_e2353_d_b51, eq187_e2353_d_b52, eq187_e2353_d_b53, eq187_e2353_d_b54,) = {
    if (((!s.b[595]) && s.b[598]) && s.b[599]) {
        let eq187_e2350: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 86, s.v[288]);
        let eq187_e2351: f64 = (p.p7 * eq187_e2350);
        (eq187_e2351, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq187_value: f64 = eq187_e2353;
        let eq187_node_derivatives: [f64; 23] = [eq187_e2353_d_n0, eq187_e2353_d_n1, eq187_e2353_d_n2, eq187_e2353_d_n3, eq187_e2353_d_n4, eq187_e2353_d_n5, eq187_e2353_d_n6, eq187_e2353_d_n7, eq187_e2353_d_n8, eq187_e2353_d_n9, eq187_e2353_d_n10, eq187_e2353_d_n11, eq187_e2353_d_n12, eq187_e2353_d_n13, eq187_e2353_d_n14, eq187_e2353_d_n15, eq187_e2353_d_n16, eq187_e2353_d_n17, eq187_e2353_d_n18, eq187_e2353_d_n19, eq187_e2353_d_n20, eq187_e2353_d_n21, eq187_e2353_d_n22];
        let eq187_branch_derivatives: [f64; 55] = [eq187_e2353_d_b0, eq187_e2353_d_b1, eq187_e2353_d_b2, eq187_e2353_d_b3, eq187_e2353_d_b4, eq187_e2353_d_b5, eq187_e2353_d_b6, eq187_e2353_d_b7, eq187_e2353_d_b8, eq187_e2353_d_b9, eq187_e2353_d_b10, eq187_e2353_d_b11, eq187_e2353_d_b12, eq187_e2353_d_b13, eq187_e2353_d_b14, eq187_e2353_d_b15, eq187_e2353_d_b16, eq187_e2353_d_b17, eq187_e2353_d_b18, eq187_e2353_d_b19, eq187_e2353_d_b20, eq187_e2353_d_b21, eq187_e2353_d_b22, eq187_e2353_d_b23, eq187_e2353_d_b24, eq187_e2353_d_b25, eq187_e2353_d_b26, eq187_e2353_d_b27, eq187_e2353_d_b28, eq187_e2353_d_b29, eq187_e2353_d_b30, eq187_e2353_d_b31, eq187_e2353_d_b32, eq187_e2353_d_b33, eq187_e2353_d_b34, eq187_e2353_d_b35, eq187_e2353_d_b36, eq187_e2353_d_b37, eq187_e2353_d_b38, eq187_e2353_d_b39, eq187_e2353_d_b40, eq187_e2353_d_b41, eq187_e2353_d_b42, eq187_e2353_d_b43, eq187_e2353_d_b44, eq187_e2353_d_b45, eq187_e2353_d_b46, eq187_e2353_d_b47, eq187_e2353_d_b48, eq187_e2353_d_b49, eq187_e2353_d_b50, eq187_e2353_d_b51, eq187_e2353_d_b52, eq187_e2353_d_b53, eq187_e2353_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq187_value),
            &eq187_node_derivatives,
            &eq187_branch_derivatives,
            multiplicity,
        );
        let (eq188_e2367, eq188_e2367_d_n0, eq188_e2367_d_n1, eq188_e2367_d_n2, eq188_e2367_d_n3, eq188_e2367_d_n4, eq188_e2367_d_n5, eq188_e2367_d_n6, eq188_e2367_d_n7, eq188_e2367_d_n8, eq188_e2367_d_n9, eq188_e2367_d_n10, eq188_e2367_d_n11, eq188_e2367_d_n12, eq188_e2367_d_n13, eq188_e2367_d_n14, eq188_e2367_d_n15, eq188_e2367_d_n16, eq188_e2367_d_n17, eq188_e2367_d_n18, eq188_e2367_d_n19, eq188_e2367_d_n20, eq188_e2367_d_n21, eq188_e2367_d_n22, eq188_e2367_d_b0, eq188_e2367_d_b1, eq188_e2367_d_b2, eq188_e2367_d_b3, eq188_e2367_d_b4, eq188_e2367_d_b5, eq188_e2367_d_b6, eq188_e2367_d_b7, eq188_e2367_d_b8, eq188_e2367_d_b9, eq188_e2367_d_b10, eq188_e2367_d_b11, eq188_e2367_d_b12, eq188_e2367_d_b13, eq188_e2367_d_b14, eq188_e2367_d_b15, eq188_e2367_d_b16, eq188_e2367_d_b17, eq188_e2367_d_b18, eq188_e2367_d_b19, eq188_e2367_d_b20, eq188_e2367_d_b21, eq188_e2367_d_b22, eq188_e2367_d_b23, eq188_e2367_d_b24, eq188_e2367_d_b25, eq188_e2367_d_b26, eq188_e2367_d_b27, eq188_e2367_d_b28, eq188_e2367_d_b29, eq188_e2367_d_b30, eq188_e2367_d_b31, eq188_e2367_d_b32, eq188_e2367_d_b33, eq188_e2367_d_b34, eq188_e2367_d_b35, eq188_e2367_d_b36, eq188_e2367_d_b37, eq188_e2367_d_b38, eq188_e2367_d_b39, eq188_e2367_d_b40, eq188_e2367_d_b41, eq188_e2367_d_b42, eq188_e2367_d_b43, eq188_e2367_d_b44, eq188_e2367_d_b45, eq188_e2367_d_b46, eq188_e2367_d_b47, eq188_e2367_d_b48, eq188_e2367_d_b49, eq188_e2367_d_b50, eq188_e2367_d_b51, eq188_e2367_d_b52, eq188_e2367_d_b53, eq188_e2367_d_b54,) = {
    if (((!s.b[595]) && s.b[598]) && s.b[599]) {
        let eq188_e2362: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 87, s.v[288]);
        let eq188_e2363: f64 = (p.p7 * eq188_e2362);
        let eq188_e2365: f64 = (eq188_e2363 * p.p248);
        let eq188_e2365_d_n0: f64 = (__rspice_deriv_cse_0 * p.p248);
        let eq188_e2365_d_n1: f64 = (__rspice_deriv_cse_1 * p.p248);
        let eq188_e2365_d_n2: f64 = (__rspice_deriv_cse_2 * p.p248);
        let eq188_e2365_d_n3: f64 = (__rspice_deriv_cse_3 * p.p248);
        let eq188_e2365_d_n4: f64 = (__rspice_deriv_cse_4 * p.p248);
        let eq188_e2365_d_n5: f64 = (__rspice_deriv_cse_5 * p.p248);
        let eq188_e2365_d_n6: f64 = (__rspice_deriv_cse_6 * p.p248);
        let eq188_e2365_d_n7: f64 = (__rspice_deriv_cse_7 * p.p248);
        let eq188_e2365_d_n8: f64 = (__rspice_deriv_cse_8 * p.p248);
        let eq188_e2365_d_n9: f64 = (__rspice_deriv_cse_9 * p.p248);
        let eq188_e2365_d_n10: f64 = (__rspice_deriv_cse_10 * p.p248);
        let eq188_e2365_d_n11: f64 = (__rspice_deriv_cse_11 * p.p248);
        let eq188_e2365_d_n12: f64 = (__rspice_deriv_cse_12 * p.p248);
        let eq188_e2365_d_n13: f64 = (__rspice_deriv_cse_13 * p.p248);
        let eq188_e2365_d_n14: f64 = (__rspice_deriv_cse_14 * p.p248);
        let eq188_e2365_d_n15: f64 = (__rspice_deriv_cse_15 * p.p248);
        let eq188_e2365_d_n16: f64 = (__rspice_deriv_cse_16 * p.p248);
        let eq188_e2365_d_n17: f64 = (__rspice_deriv_cse_17 * p.p248);
        let eq188_e2365_d_n18: f64 = (__rspice_deriv_cse_18 * p.p248);
        let eq188_e2365_d_n19: f64 = (__rspice_deriv_cse_19 * p.p248);
        let eq188_e2365_d_n20: f64 = (__rspice_deriv_cse_20 * p.p248);
        let eq188_e2365_d_n21: f64 = (__rspice_deriv_cse_21 * p.p248);
        let eq188_e2365_d_n22: f64 = (__rspice_deriv_cse_22 * p.p248);
        let eq188_e2365_d_b0: f64 = (__rspice_deriv_cse_23 * p.p248);
        let eq188_e2365_d_b1: f64 = (__rspice_deriv_cse_24 * p.p248);
        let eq188_e2365_d_b2: f64 = (__rspice_deriv_cse_25 * p.p248);
        let eq188_e2365_d_b3: f64 = (__rspice_deriv_cse_26 * p.p248);
        let eq188_e2365_d_b4: f64 = (__rspice_deriv_cse_27 * p.p248);
        let eq188_e2365_d_b5: f64 = (__rspice_deriv_cse_28 * p.p248);
        let eq188_e2365_d_b6: f64 = (__rspice_deriv_cse_29 * p.p248);
        let eq188_e2365_d_b7: f64 = (__rspice_deriv_cse_30 * p.p248);
        let eq188_e2365_d_b8: f64 = (__rspice_deriv_cse_31 * p.p248);
        let eq188_e2365_d_b9: f64 = (__rspice_deriv_cse_32 * p.p248);
        let eq188_e2365_d_b10: f64 = (__rspice_deriv_cse_33 * p.p248);
        let eq188_e2365_d_b11: f64 = (__rspice_deriv_cse_34 * p.p248);
        let eq188_e2365_d_b12: f64 = (__rspice_deriv_cse_35 * p.p248);
        let eq188_e2365_d_b13: f64 = (__rspice_deriv_cse_36 * p.p248);
        let eq188_e2365_d_b14: f64 = (__rspice_deriv_cse_37 * p.p248);
        let eq188_e2365_d_b15: f64 = (__rspice_deriv_cse_38 * p.p248);
        let eq188_e2365_d_b16: f64 = (__rspice_deriv_cse_39 * p.p248);
        let eq188_e2365_d_b17: f64 = (__rspice_deriv_cse_40 * p.p248);
        let eq188_e2365_d_b18: f64 = (__rspice_deriv_cse_41 * p.p248);
        let eq188_e2365_d_b19: f64 = (__rspice_deriv_cse_42 * p.p248);
        let eq188_e2365_d_b20: f64 = (__rspice_deriv_cse_43 * p.p248);
        let eq188_e2365_d_b21: f64 = (__rspice_deriv_cse_44 * p.p248);
        let eq188_e2365_d_b22: f64 = (__rspice_deriv_cse_45 * p.p248);
        let eq188_e2365_d_b23: f64 = (__rspice_deriv_cse_46 * p.p248);
        let eq188_e2365_d_b24: f64 = (__rspice_deriv_cse_47 * p.p248);
        let eq188_e2365_d_b25: f64 = (__rspice_deriv_cse_48 * p.p248);
        let eq188_e2365_d_b26: f64 = (__rspice_deriv_cse_49 * p.p248);
        let eq188_e2365_d_b27: f64 = (__rspice_deriv_cse_50 * p.p248);
        let eq188_e2365_d_b28: f64 = (__rspice_deriv_cse_51 * p.p248);
        let eq188_e2365_d_b29: f64 = (__rspice_deriv_cse_52 * p.p248);
        let eq188_e2365_d_b30: f64 = (__rspice_deriv_cse_53 * p.p248);
        let eq188_e2365_d_b31: f64 = (__rspice_deriv_cse_54 * p.p248);
        let eq188_e2365_d_b32: f64 = (__rspice_deriv_cse_55 * p.p248);
        let eq188_e2365_d_b33: f64 = (__rspice_deriv_cse_56 * p.p248);
        let eq188_e2365_d_b34: f64 = (__rspice_deriv_cse_57 * p.p248);
        let eq188_e2365_d_b35: f64 = (__rspice_deriv_cse_58 * p.p248);
        let eq188_e2365_d_b36: f64 = (__rspice_deriv_cse_59 * p.p248);
        let eq188_e2365_d_b37: f64 = (__rspice_deriv_cse_60 * p.p248);
        let eq188_e2365_d_b38: f64 = (__rspice_deriv_cse_61 * p.p248);
        let eq188_e2365_d_b39: f64 = (__rspice_deriv_cse_62 * p.p248);
        let eq188_e2365_d_b40: f64 = (__rspice_deriv_cse_63 * p.p248);
        let eq188_e2365_d_b41: f64 = (__rspice_deriv_cse_64 * p.p248);
        let eq188_e2365_d_b42: f64 = (__rspice_deriv_cse_65 * p.p248);
        let eq188_e2365_d_b43: f64 = (__rspice_deriv_cse_66 * p.p248);
        let eq188_e2365_d_b44: f64 = (__rspice_deriv_cse_67 * p.p248);
        let eq188_e2365_d_b45: f64 = (__rspice_deriv_cse_68 * p.p248);
        let eq188_e2365_d_b46: f64 = (__rspice_deriv_cse_69 * p.p248);
        let eq188_e2365_d_b47: f64 = (__rspice_deriv_cse_70 * p.p248);
        let eq188_e2365_d_b48: f64 = (__rspice_deriv_cse_71 * p.p248);
        let eq188_e2365_d_b49: f64 = (__rspice_deriv_cse_72 * p.p248);
        let eq188_e2365_d_b50: f64 = (__rspice_deriv_cse_73 * p.p248);
        let eq188_e2365_d_b51: f64 = (__rspice_deriv_cse_74 * p.p248);
        let eq188_e2365_d_b52: f64 = (__rspice_deriv_cse_75 * p.p248);
        let eq188_e2365_d_b53: f64 = (__rspice_deriv_cse_76 * p.p248);
        let eq188_e2365_d_b54: f64 = (__rspice_deriv_cse_77 * p.p248);
        (eq188_e2365, eq188_e2365_d_n0, eq188_e2365_d_n1, eq188_e2365_d_n2, eq188_e2365_d_n3, eq188_e2365_d_n4, eq188_e2365_d_n5, eq188_e2365_d_n6, eq188_e2365_d_n7, eq188_e2365_d_n8, eq188_e2365_d_n9, eq188_e2365_d_n10, eq188_e2365_d_n11, eq188_e2365_d_n12, eq188_e2365_d_n13, eq188_e2365_d_n14, eq188_e2365_d_n15, eq188_e2365_d_n16, eq188_e2365_d_n17, eq188_e2365_d_n18, eq188_e2365_d_n19, eq188_e2365_d_n20, eq188_e2365_d_n21, eq188_e2365_d_n22, eq188_e2365_d_b0, eq188_e2365_d_b1, eq188_e2365_d_b2, eq188_e2365_d_b3, eq188_e2365_d_b4, eq188_e2365_d_b5, eq188_e2365_d_b6, eq188_e2365_d_b7, eq188_e2365_d_b8, eq188_e2365_d_b9, eq188_e2365_d_b10, eq188_e2365_d_b11, eq188_e2365_d_b12, eq188_e2365_d_b13, eq188_e2365_d_b14, eq188_e2365_d_b15, eq188_e2365_d_b16, eq188_e2365_d_b17, eq188_e2365_d_b18, eq188_e2365_d_b19, eq188_e2365_d_b20, eq188_e2365_d_b21, eq188_e2365_d_b22, eq188_e2365_d_b23, eq188_e2365_d_b24, eq188_e2365_d_b25, eq188_e2365_d_b26, eq188_e2365_d_b27, eq188_e2365_d_b28, eq188_e2365_d_b29, eq188_e2365_d_b30, eq188_e2365_d_b31, eq188_e2365_d_b32, eq188_e2365_d_b33, eq188_e2365_d_b34, eq188_e2365_d_b35, eq188_e2365_d_b36, eq188_e2365_d_b37, eq188_e2365_d_b38, eq188_e2365_d_b39, eq188_e2365_d_b40, eq188_e2365_d_b41, eq188_e2365_d_b42, eq188_e2365_d_b43, eq188_e2365_d_b44, eq188_e2365_d_b45, eq188_e2365_d_b46, eq188_e2365_d_b47, eq188_e2365_d_b48, eq188_e2365_d_b49, eq188_e2365_d_b50, eq188_e2365_d_b51, eq188_e2365_d_b52, eq188_e2365_d_b53, eq188_e2365_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq188_value: f64 = eq188_e2367;
        let eq188_node_derivatives: [f64; 23] = [eq188_e2367_d_n0, eq188_e2367_d_n1, eq188_e2367_d_n2, eq188_e2367_d_n3, eq188_e2367_d_n4, eq188_e2367_d_n5, eq188_e2367_d_n6, eq188_e2367_d_n7, eq188_e2367_d_n8, eq188_e2367_d_n9, eq188_e2367_d_n10, eq188_e2367_d_n11, eq188_e2367_d_n12, eq188_e2367_d_n13, eq188_e2367_d_n14, eq188_e2367_d_n15, eq188_e2367_d_n16, eq188_e2367_d_n17, eq188_e2367_d_n18, eq188_e2367_d_n19, eq188_e2367_d_n20, eq188_e2367_d_n21, eq188_e2367_d_n22];
        let eq188_branch_derivatives: [f64; 55] = [eq188_e2367_d_b0, eq188_e2367_d_b1, eq188_e2367_d_b2, eq188_e2367_d_b3, eq188_e2367_d_b4, eq188_e2367_d_b5, eq188_e2367_d_b6, eq188_e2367_d_b7, eq188_e2367_d_b8, eq188_e2367_d_b9, eq188_e2367_d_b10, eq188_e2367_d_b11, eq188_e2367_d_b12, eq188_e2367_d_b13, eq188_e2367_d_b14, eq188_e2367_d_b15, eq188_e2367_d_b16, eq188_e2367_d_b17, eq188_e2367_d_b18, eq188_e2367_d_b19, eq188_e2367_d_b20, eq188_e2367_d_b21, eq188_e2367_d_b22, eq188_e2367_d_b23, eq188_e2367_d_b24, eq188_e2367_d_b25, eq188_e2367_d_b26, eq188_e2367_d_b27, eq188_e2367_d_b28, eq188_e2367_d_b29, eq188_e2367_d_b30, eq188_e2367_d_b31, eq188_e2367_d_b32, eq188_e2367_d_b33, eq188_e2367_d_b34, eq188_e2367_d_b35, eq188_e2367_d_b36, eq188_e2367_d_b37, eq188_e2367_d_b38, eq188_e2367_d_b39, eq188_e2367_d_b40, eq188_e2367_d_b41, eq188_e2367_d_b42, eq188_e2367_d_b43, eq188_e2367_d_b44, eq188_e2367_d_b45, eq188_e2367_d_b46, eq188_e2367_d_b47, eq188_e2367_d_b48, eq188_e2367_d_b49, eq188_e2367_d_b50, eq188_e2367_d_b51, eq188_e2367_d_b52, eq188_e2367_d_b53, eq188_e2367_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq188_value),
            &eq188_node_derivatives,
            &eq188_branch_derivatives,
            multiplicity,
        );
        let (eq189_e2380, eq189_e2380_d_n0, eq189_e2380_d_n1, eq189_e2380_d_n2, eq189_e2380_d_n3, eq189_e2380_d_n4, eq189_e2380_d_n5, eq189_e2380_d_n6, eq189_e2380_d_n7, eq189_e2380_d_n8, eq189_e2380_d_n9, eq189_e2380_d_n10, eq189_e2380_d_n11, eq189_e2380_d_n12, eq189_e2380_d_n13, eq189_e2380_d_n14, eq189_e2380_d_n15, eq189_e2380_d_n16, eq189_e2380_d_n17, eq189_e2380_d_n18, eq189_e2380_d_n19, eq189_e2380_d_n20, eq189_e2380_d_n21, eq189_e2380_d_n22, eq189_e2380_d_b0, eq189_e2380_d_b1, eq189_e2380_d_b2, eq189_e2380_d_b3, eq189_e2380_d_b4, eq189_e2380_d_b5, eq189_e2380_d_b6, eq189_e2380_d_b7, eq189_e2380_d_b8, eq189_e2380_d_b9, eq189_e2380_d_b10, eq189_e2380_d_b11, eq189_e2380_d_b12, eq189_e2380_d_b13, eq189_e2380_d_b14, eq189_e2380_d_b15, eq189_e2380_d_b16, eq189_e2380_d_b17, eq189_e2380_d_b18, eq189_e2380_d_b19, eq189_e2380_d_b20, eq189_e2380_d_b21, eq189_e2380_d_b22, eq189_e2380_d_b23, eq189_e2380_d_b24, eq189_e2380_d_b25, eq189_e2380_d_b26, eq189_e2380_d_b27, eq189_e2380_d_b28, eq189_e2380_d_b29, eq189_e2380_d_b30, eq189_e2380_d_b31, eq189_e2380_d_b32, eq189_e2380_d_b33, eq189_e2380_d_b34, eq189_e2380_d_b35, eq189_e2380_d_b36, eq189_e2380_d_b37, eq189_e2380_d_b38, eq189_e2380_d_b39, eq189_e2380_d_b40, eq189_e2380_d_b41, eq189_e2380_d_b42, eq189_e2380_d_b43, eq189_e2380_d_b44, eq189_e2380_d_b45, eq189_e2380_d_b46, eq189_e2380_d_b47, eq189_e2380_d_b48, eq189_e2380_d_b49, eq189_e2380_d_b50, eq189_e2380_d_b51, eq189_e2380_d_b52, eq189_e2380_d_b53, eq189_e2380_d_b54,) = {
    if (((!s.b[595]) && s.b[598]) && (!s.b[599])) {
        let eq189_e2377: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 88, s.v[288]);
        let eq189_e2378: f64 = (p.p7 * eq189_e2377);
        (eq189_e2378, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq189_value: f64 = eq189_e2380;
        let eq189_node_derivatives: [f64; 23] = [eq189_e2380_d_n0, eq189_e2380_d_n1, eq189_e2380_d_n2, eq189_e2380_d_n3, eq189_e2380_d_n4, eq189_e2380_d_n5, eq189_e2380_d_n6, eq189_e2380_d_n7, eq189_e2380_d_n8, eq189_e2380_d_n9, eq189_e2380_d_n10, eq189_e2380_d_n11, eq189_e2380_d_n12, eq189_e2380_d_n13, eq189_e2380_d_n14, eq189_e2380_d_n15, eq189_e2380_d_n16, eq189_e2380_d_n17, eq189_e2380_d_n18, eq189_e2380_d_n19, eq189_e2380_d_n20, eq189_e2380_d_n21, eq189_e2380_d_n22];
        let eq189_branch_derivatives: [f64; 55] = [eq189_e2380_d_b0, eq189_e2380_d_b1, eq189_e2380_d_b2, eq189_e2380_d_b3, eq189_e2380_d_b4, eq189_e2380_d_b5, eq189_e2380_d_b6, eq189_e2380_d_b7, eq189_e2380_d_b8, eq189_e2380_d_b9, eq189_e2380_d_b10, eq189_e2380_d_b11, eq189_e2380_d_b12, eq189_e2380_d_b13, eq189_e2380_d_b14, eq189_e2380_d_b15, eq189_e2380_d_b16, eq189_e2380_d_b17, eq189_e2380_d_b18, eq189_e2380_d_b19, eq189_e2380_d_b20, eq189_e2380_d_b21, eq189_e2380_d_b22, eq189_e2380_d_b23, eq189_e2380_d_b24, eq189_e2380_d_b25, eq189_e2380_d_b26, eq189_e2380_d_b27, eq189_e2380_d_b28, eq189_e2380_d_b29, eq189_e2380_d_b30, eq189_e2380_d_b31, eq189_e2380_d_b32, eq189_e2380_d_b33, eq189_e2380_d_b34, eq189_e2380_d_b35, eq189_e2380_d_b36, eq189_e2380_d_b37, eq189_e2380_d_b38, eq189_e2380_d_b39, eq189_e2380_d_b40, eq189_e2380_d_b41, eq189_e2380_d_b42, eq189_e2380_d_b43, eq189_e2380_d_b44, eq189_e2380_d_b45, eq189_e2380_d_b46, eq189_e2380_d_b47, eq189_e2380_d_b48, eq189_e2380_d_b49, eq189_e2380_d_b50, eq189_e2380_d_b51, eq189_e2380_d_b52, eq189_e2380_d_b53, eq189_e2380_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq189_value),
            &eq189_node_derivatives,
            &eq189_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_38(
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
        let (eq190_e2395, eq190_e2395_d_n0, eq190_e2395_d_n1, eq190_e2395_d_n2, eq190_e2395_d_n3, eq190_e2395_d_n4, eq190_e2395_d_n5, eq190_e2395_d_n6, eq190_e2395_d_n7, eq190_e2395_d_n8, eq190_e2395_d_n9, eq190_e2395_d_n10, eq190_e2395_d_n11, eq190_e2395_d_n12, eq190_e2395_d_n13, eq190_e2395_d_n14, eq190_e2395_d_n15, eq190_e2395_d_n16, eq190_e2395_d_n17, eq190_e2395_d_n18, eq190_e2395_d_n19, eq190_e2395_d_n20, eq190_e2395_d_n21, eq190_e2395_d_n22, eq190_e2395_d_b0, eq190_e2395_d_b1, eq190_e2395_d_b2, eq190_e2395_d_b3, eq190_e2395_d_b4, eq190_e2395_d_b5, eq190_e2395_d_b6, eq190_e2395_d_b7, eq190_e2395_d_b8, eq190_e2395_d_b9, eq190_e2395_d_b10, eq190_e2395_d_b11, eq190_e2395_d_b12, eq190_e2395_d_b13, eq190_e2395_d_b14, eq190_e2395_d_b15, eq190_e2395_d_b16, eq190_e2395_d_b17, eq190_e2395_d_b18, eq190_e2395_d_b19, eq190_e2395_d_b20, eq190_e2395_d_b21, eq190_e2395_d_b22, eq190_e2395_d_b23, eq190_e2395_d_b24, eq190_e2395_d_b25, eq190_e2395_d_b26, eq190_e2395_d_b27, eq190_e2395_d_b28, eq190_e2395_d_b29, eq190_e2395_d_b30, eq190_e2395_d_b31, eq190_e2395_d_b32, eq190_e2395_d_b33, eq190_e2395_d_b34, eq190_e2395_d_b35, eq190_e2395_d_b36, eq190_e2395_d_b37, eq190_e2395_d_b38, eq190_e2395_d_b39, eq190_e2395_d_b40, eq190_e2395_d_b41, eq190_e2395_d_b42, eq190_e2395_d_b43, eq190_e2395_d_b44, eq190_e2395_d_b45, eq190_e2395_d_b46, eq190_e2395_d_b47, eq190_e2395_d_b48, eq190_e2395_d_b49, eq190_e2395_d_b50, eq190_e2395_d_b51, eq190_e2395_d_b52, eq190_e2395_d_b53, eq190_e2395_d_b54,) = {
    if (((!s.b[595]) && s.b[598]) && (!s.b[599])) {
        let eq190_e2390: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 89, s.v[288]);
        let eq190_e2391: f64 = (p.p7 * eq190_e2390);
        let eq190_e2391_d_n0: f64 = (p.p7 * (s.dn[288][0] * ddt_scale));
        let eq190_e2391_d_n1: f64 = (p.p7 * (s.dn[288][1] * ddt_scale));
        let eq190_e2391_d_n2: f64 = (p.p7 * (s.dn[288][2] * ddt_scale));
        let eq190_e2391_d_n3: f64 = (p.p7 * (s.dn[288][3] * ddt_scale));
        let eq190_e2391_d_n4: f64 = (p.p7 * (s.dn[288][4] * ddt_scale));
        let eq190_e2391_d_n5: f64 = (p.p7 * (s.dn[288][5] * ddt_scale));
        let eq190_e2391_d_n6: f64 = (p.p7 * (s.dn[288][6] * ddt_scale));
        let eq190_e2391_d_n7: f64 = (p.p7 * (s.dn[288][7] * ddt_scale));
        let eq190_e2391_d_n8: f64 = (p.p7 * (s.dn[288][8] * ddt_scale));
        let eq190_e2391_d_n9: f64 = (p.p7 * (s.dn[288][9] * ddt_scale));
        let eq190_e2391_d_n10: f64 = (p.p7 * (s.dn[288][10] * ddt_scale));
        let eq190_e2391_d_n11: f64 = (p.p7 * (s.dn[288][11] * ddt_scale));
        let eq190_e2391_d_n12: f64 = (p.p7 * (s.dn[288][12] * ddt_scale));
        let eq190_e2391_d_n13: f64 = (p.p7 * (s.dn[288][13] * ddt_scale));
        let eq190_e2391_d_n14: f64 = (p.p7 * (s.dn[288][14] * ddt_scale));
        let eq190_e2391_d_n15: f64 = (p.p7 * (s.dn[288][15] * ddt_scale));
        let eq190_e2391_d_n16: f64 = (p.p7 * (s.dn[288][16] * ddt_scale));
        let eq190_e2391_d_n17: f64 = (p.p7 * (s.dn[288][17] * ddt_scale));
        let eq190_e2391_d_n18: f64 = (p.p7 * (s.dn[288][18] * ddt_scale));
        let eq190_e2391_d_n19: f64 = (p.p7 * (s.dn[288][19] * ddt_scale));
        let eq190_e2391_d_n20: f64 = (p.p7 * (s.dn[288][20] * ddt_scale));
        let eq190_e2391_d_n21: f64 = (p.p7 * (s.dn[288][21] * ddt_scale));
        let eq190_e2391_d_n22: f64 = (p.p7 * (s.dn[288][22] * ddt_scale));
        let eq190_e2391_d_b0: f64 = (p.p7 * (s.db[288][0] * ddt_scale));
        let eq190_e2391_d_b1: f64 = (p.p7 * (s.db[288][1] * ddt_scale));
        let eq190_e2391_d_b2: f64 = (p.p7 * (s.db[288][2] * ddt_scale));
        let eq190_e2391_d_b3: f64 = (p.p7 * (s.db[288][3] * ddt_scale));
        let eq190_e2391_d_b4: f64 = (p.p7 * (s.db[288][4] * ddt_scale));
        let eq190_e2391_d_b5: f64 = (p.p7 * (s.db[288][5] * ddt_scale));
        let eq190_e2391_d_b6: f64 = (p.p7 * (s.db[288][6] * ddt_scale));
        let eq190_e2391_d_b7: f64 = (p.p7 * (s.db[288][7] * ddt_scale));
        let eq190_e2391_d_b8: f64 = (p.p7 * (s.db[288][8] * ddt_scale));
        let eq190_e2391_d_b9: f64 = (p.p7 * (s.db[288][9] * ddt_scale));
        let eq190_e2391_d_b10: f64 = (p.p7 * (s.db[288][10] * ddt_scale));
        let eq190_e2391_d_b11: f64 = (p.p7 * (s.db[288][11] * ddt_scale));
        let eq190_e2391_d_b12: f64 = (p.p7 * (s.db[288][12] * ddt_scale));
        let eq190_e2391_d_b13: f64 = (p.p7 * (s.db[288][13] * ddt_scale));
        let eq190_e2391_d_b14: f64 = (p.p7 * (s.db[288][14] * ddt_scale));
        let eq190_e2391_d_b15: f64 = (p.p7 * (s.db[288][15] * ddt_scale));
        let eq190_e2391_d_b16: f64 = (p.p7 * (s.db[288][16] * ddt_scale));
        let eq190_e2391_d_b17: f64 = (p.p7 * (s.db[288][17] * ddt_scale));
        let eq190_e2391_d_b18: f64 = (p.p7 * (s.db[288][18] * ddt_scale));
        let eq190_e2391_d_b19: f64 = (p.p7 * (s.db[288][19] * ddt_scale));
        let eq190_e2391_d_b20: f64 = (p.p7 * (s.db[288][20] * ddt_scale));
        let eq190_e2391_d_b21: f64 = (p.p7 * (s.db[288][21] * ddt_scale));
        let eq190_e2391_d_b22: f64 = (p.p7 * (s.db[288][22] * ddt_scale));
        let eq190_e2391_d_b23: f64 = (p.p7 * (s.db[288][23] * ddt_scale));
        let eq190_e2391_d_b24: f64 = (p.p7 * (s.db[288][24] * ddt_scale));
        let eq190_e2391_d_b25: f64 = (p.p7 * (s.db[288][25] * ddt_scale));
        let eq190_e2391_d_b26: f64 = (p.p7 * (s.db[288][26] * ddt_scale));
        let eq190_e2391_d_b27: f64 = (p.p7 * (s.db[288][27] * ddt_scale));
        let eq190_e2391_d_b28: f64 = (p.p7 * (s.db[288][28] * ddt_scale));
        let eq190_e2391_d_b29: f64 = (p.p7 * (s.db[288][29] * ddt_scale));
        let eq190_e2391_d_b30: f64 = (p.p7 * (s.db[288][30] * ddt_scale));
        let eq190_e2391_d_b31: f64 = (p.p7 * (s.db[288][31] * ddt_scale));
        let eq190_e2391_d_b32: f64 = (p.p7 * (s.db[288][32] * ddt_scale));
        let eq190_e2391_d_b33: f64 = (p.p7 * (s.db[288][33] * ddt_scale));
        let eq190_e2391_d_b34: f64 = (p.p7 * (s.db[288][34] * ddt_scale));
        let eq190_e2391_d_b35: f64 = (p.p7 * (s.db[288][35] * ddt_scale));
        let eq190_e2391_d_b36: f64 = (p.p7 * (s.db[288][36] * ddt_scale));
        let eq190_e2391_d_b37: f64 = (p.p7 * (s.db[288][37] * ddt_scale));
        let eq190_e2391_d_b38: f64 = (p.p7 * (s.db[288][38] * ddt_scale));
        let eq190_e2391_d_b39: f64 = (p.p7 * (s.db[288][39] * ddt_scale));
        let eq190_e2391_d_b40: f64 = (p.p7 * (s.db[288][40] * ddt_scale));
        let eq190_e2391_d_b41: f64 = (p.p7 * (s.db[288][41] * ddt_scale));
        let eq190_e2391_d_b42: f64 = (p.p7 * (s.db[288][42] * ddt_scale));
        let eq190_e2391_d_b43: f64 = (p.p7 * (s.db[288][43] * ddt_scale));
        let eq190_e2391_d_b44: f64 = (p.p7 * (s.db[288][44] * ddt_scale));
        let eq190_e2391_d_b45: f64 = (p.p7 * (s.db[288][45] * ddt_scale));
        let eq190_e2391_d_b46: f64 = (p.p7 * (s.db[288][46] * ddt_scale));
        let eq190_e2391_d_b47: f64 = (p.p7 * (s.db[288][47] * ddt_scale));
        let eq190_e2391_d_b48: f64 = (p.p7 * (s.db[288][48] * ddt_scale));
        let eq190_e2391_d_b49: f64 = (p.p7 * (s.db[288][49] * ddt_scale));
        let eq190_e2391_d_b50: f64 = (p.p7 * (s.db[288][50] * ddt_scale));
        let eq190_e2391_d_b51: f64 = (p.p7 * (s.db[288][51] * ddt_scale));
        let eq190_e2391_d_b52: f64 = (p.p7 * (s.db[288][52] * ddt_scale));
        let eq190_e2391_d_b53: f64 = (p.p7 * (s.db[288][53] * ddt_scale));
        let eq190_e2391_d_b54: f64 = (p.p7 * (s.db[288][54] * ddt_scale));
        let eq190_e2393: f64 = (eq190_e2391 * p.p248);
        let eq190_e2393_d_n0: f64 = (eq190_e2391_d_n0 * p.p248);
        let eq190_e2393_d_n1: f64 = (eq190_e2391_d_n1 * p.p248);
        let eq190_e2393_d_n2: f64 = (eq190_e2391_d_n2 * p.p248);
        let eq190_e2393_d_n3: f64 = (eq190_e2391_d_n3 * p.p248);
        let eq190_e2393_d_n4: f64 = (eq190_e2391_d_n4 * p.p248);
        let eq190_e2393_d_n5: f64 = (eq190_e2391_d_n5 * p.p248);
        let eq190_e2393_d_n6: f64 = (eq190_e2391_d_n6 * p.p248);
        let eq190_e2393_d_n7: f64 = (eq190_e2391_d_n7 * p.p248);
        let eq190_e2393_d_n8: f64 = (eq190_e2391_d_n8 * p.p248);
        let eq190_e2393_d_n9: f64 = (eq190_e2391_d_n9 * p.p248);
        let eq190_e2393_d_n10: f64 = (eq190_e2391_d_n10 * p.p248);
        let eq190_e2393_d_n11: f64 = (eq190_e2391_d_n11 * p.p248);
        let eq190_e2393_d_n12: f64 = (eq190_e2391_d_n12 * p.p248);
        let eq190_e2393_d_n13: f64 = (eq190_e2391_d_n13 * p.p248);
        let eq190_e2393_d_n14: f64 = (eq190_e2391_d_n14 * p.p248);
        let eq190_e2393_d_n15: f64 = (eq190_e2391_d_n15 * p.p248);
        let eq190_e2393_d_n16: f64 = (eq190_e2391_d_n16 * p.p248);
        let eq190_e2393_d_n17: f64 = (eq190_e2391_d_n17 * p.p248);
        let eq190_e2393_d_n18: f64 = (eq190_e2391_d_n18 * p.p248);
        let eq190_e2393_d_n19: f64 = (eq190_e2391_d_n19 * p.p248);
        let eq190_e2393_d_n20: f64 = (eq190_e2391_d_n20 * p.p248);
        let eq190_e2393_d_n21: f64 = (eq190_e2391_d_n21 * p.p248);
        let eq190_e2393_d_n22: f64 = (eq190_e2391_d_n22 * p.p248);
        let eq190_e2393_d_b0: f64 = (eq190_e2391_d_b0 * p.p248);
        let eq190_e2393_d_b1: f64 = (eq190_e2391_d_b1 * p.p248);
        let eq190_e2393_d_b2: f64 = (eq190_e2391_d_b2 * p.p248);
        let eq190_e2393_d_b3: f64 = (eq190_e2391_d_b3 * p.p248);
        let eq190_e2393_d_b4: f64 = (eq190_e2391_d_b4 * p.p248);
        let eq190_e2393_d_b5: f64 = (eq190_e2391_d_b5 * p.p248);
        let eq190_e2393_d_b6: f64 = (eq190_e2391_d_b6 * p.p248);
        let eq190_e2393_d_b7: f64 = (eq190_e2391_d_b7 * p.p248);
        let eq190_e2393_d_b8: f64 = (eq190_e2391_d_b8 * p.p248);
        let eq190_e2393_d_b9: f64 = (eq190_e2391_d_b9 * p.p248);
        let eq190_e2393_d_b10: f64 = (eq190_e2391_d_b10 * p.p248);
        let eq190_e2393_d_b11: f64 = (eq190_e2391_d_b11 * p.p248);
        let eq190_e2393_d_b12: f64 = (eq190_e2391_d_b12 * p.p248);
        let eq190_e2393_d_b13: f64 = (eq190_e2391_d_b13 * p.p248);
        let eq190_e2393_d_b14: f64 = (eq190_e2391_d_b14 * p.p248);
        let eq190_e2393_d_b15: f64 = (eq190_e2391_d_b15 * p.p248);
        let eq190_e2393_d_b16: f64 = (eq190_e2391_d_b16 * p.p248);
        let eq190_e2393_d_b17: f64 = (eq190_e2391_d_b17 * p.p248);
        let eq190_e2393_d_b18: f64 = (eq190_e2391_d_b18 * p.p248);
        let eq190_e2393_d_b19: f64 = (eq190_e2391_d_b19 * p.p248);
        let eq190_e2393_d_b20: f64 = (eq190_e2391_d_b20 * p.p248);
        let eq190_e2393_d_b21: f64 = (eq190_e2391_d_b21 * p.p248);
        let eq190_e2393_d_b22: f64 = (eq190_e2391_d_b22 * p.p248);
        let eq190_e2393_d_b23: f64 = (eq190_e2391_d_b23 * p.p248);
        let eq190_e2393_d_b24: f64 = (eq190_e2391_d_b24 * p.p248);
        let eq190_e2393_d_b25: f64 = (eq190_e2391_d_b25 * p.p248);
        let eq190_e2393_d_b26: f64 = (eq190_e2391_d_b26 * p.p248);
        let eq190_e2393_d_b27: f64 = (eq190_e2391_d_b27 * p.p248);
        let eq190_e2393_d_b28: f64 = (eq190_e2391_d_b28 * p.p248);
        let eq190_e2393_d_b29: f64 = (eq190_e2391_d_b29 * p.p248);
        let eq190_e2393_d_b30: f64 = (eq190_e2391_d_b30 * p.p248);
        let eq190_e2393_d_b31: f64 = (eq190_e2391_d_b31 * p.p248);
        let eq190_e2393_d_b32: f64 = (eq190_e2391_d_b32 * p.p248);
        let eq190_e2393_d_b33: f64 = (eq190_e2391_d_b33 * p.p248);
        let eq190_e2393_d_b34: f64 = (eq190_e2391_d_b34 * p.p248);
        let eq190_e2393_d_b35: f64 = (eq190_e2391_d_b35 * p.p248);
        let eq190_e2393_d_b36: f64 = (eq190_e2391_d_b36 * p.p248);
        let eq190_e2393_d_b37: f64 = (eq190_e2391_d_b37 * p.p248);
        let eq190_e2393_d_b38: f64 = (eq190_e2391_d_b38 * p.p248);
        let eq190_e2393_d_b39: f64 = (eq190_e2391_d_b39 * p.p248);
        let eq190_e2393_d_b40: f64 = (eq190_e2391_d_b40 * p.p248);
        let eq190_e2393_d_b41: f64 = (eq190_e2391_d_b41 * p.p248);
        let eq190_e2393_d_b42: f64 = (eq190_e2391_d_b42 * p.p248);
        let eq190_e2393_d_b43: f64 = (eq190_e2391_d_b43 * p.p248);
        let eq190_e2393_d_b44: f64 = (eq190_e2391_d_b44 * p.p248);
        let eq190_e2393_d_b45: f64 = (eq190_e2391_d_b45 * p.p248);
        let eq190_e2393_d_b46: f64 = (eq190_e2391_d_b46 * p.p248);
        let eq190_e2393_d_b47: f64 = (eq190_e2391_d_b47 * p.p248);
        let eq190_e2393_d_b48: f64 = (eq190_e2391_d_b48 * p.p248);
        let eq190_e2393_d_b49: f64 = (eq190_e2391_d_b49 * p.p248);
        let eq190_e2393_d_b50: f64 = (eq190_e2391_d_b50 * p.p248);
        let eq190_e2393_d_b51: f64 = (eq190_e2391_d_b51 * p.p248);
        let eq190_e2393_d_b52: f64 = (eq190_e2391_d_b52 * p.p248);
        let eq190_e2393_d_b53: f64 = (eq190_e2391_d_b53 * p.p248);
        let eq190_e2393_d_b54: f64 = (eq190_e2391_d_b54 * p.p248);
        (eq190_e2393, eq190_e2393_d_n0, eq190_e2393_d_n1, eq190_e2393_d_n2, eq190_e2393_d_n3, eq190_e2393_d_n4, eq190_e2393_d_n5, eq190_e2393_d_n6, eq190_e2393_d_n7, eq190_e2393_d_n8, eq190_e2393_d_n9, eq190_e2393_d_n10, eq190_e2393_d_n11, eq190_e2393_d_n12, eq190_e2393_d_n13, eq190_e2393_d_n14, eq190_e2393_d_n15, eq190_e2393_d_n16, eq190_e2393_d_n17, eq190_e2393_d_n18, eq190_e2393_d_n19, eq190_e2393_d_n20, eq190_e2393_d_n21, eq190_e2393_d_n22, eq190_e2393_d_b0, eq190_e2393_d_b1, eq190_e2393_d_b2, eq190_e2393_d_b3, eq190_e2393_d_b4, eq190_e2393_d_b5, eq190_e2393_d_b6, eq190_e2393_d_b7, eq190_e2393_d_b8, eq190_e2393_d_b9, eq190_e2393_d_b10, eq190_e2393_d_b11, eq190_e2393_d_b12, eq190_e2393_d_b13, eq190_e2393_d_b14, eq190_e2393_d_b15, eq190_e2393_d_b16, eq190_e2393_d_b17, eq190_e2393_d_b18, eq190_e2393_d_b19, eq190_e2393_d_b20, eq190_e2393_d_b21, eq190_e2393_d_b22, eq190_e2393_d_b23, eq190_e2393_d_b24, eq190_e2393_d_b25, eq190_e2393_d_b26, eq190_e2393_d_b27, eq190_e2393_d_b28, eq190_e2393_d_b29, eq190_e2393_d_b30, eq190_e2393_d_b31, eq190_e2393_d_b32, eq190_e2393_d_b33, eq190_e2393_d_b34, eq190_e2393_d_b35, eq190_e2393_d_b36, eq190_e2393_d_b37, eq190_e2393_d_b38, eq190_e2393_d_b39, eq190_e2393_d_b40, eq190_e2393_d_b41, eq190_e2393_d_b42, eq190_e2393_d_b43, eq190_e2393_d_b44, eq190_e2393_d_b45, eq190_e2393_d_b46, eq190_e2393_d_b47, eq190_e2393_d_b48, eq190_e2393_d_b49, eq190_e2393_d_b50, eq190_e2393_d_b51, eq190_e2393_d_b52, eq190_e2393_d_b53, eq190_e2393_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq190_value: f64 = eq190_e2395;
        let eq190_node_derivatives: [f64; 23] = [eq190_e2395_d_n0, eq190_e2395_d_n1, eq190_e2395_d_n2, eq190_e2395_d_n3, eq190_e2395_d_n4, eq190_e2395_d_n5, eq190_e2395_d_n6, eq190_e2395_d_n7, eq190_e2395_d_n8, eq190_e2395_d_n9, eq190_e2395_d_n10, eq190_e2395_d_n11, eq190_e2395_d_n12, eq190_e2395_d_n13, eq190_e2395_d_n14, eq190_e2395_d_n15, eq190_e2395_d_n16, eq190_e2395_d_n17, eq190_e2395_d_n18, eq190_e2395_d_n19, eq190_e2395_d_n20, eq190_e2395_d_n21, eq190_e2395_d_n22];
        let eq190_branch_derivatives: [f64; 55] = [eq190_e2395_d_b0, eq190_e2395_d_b1, eq190_e2395_d_b2, eq190_e2395_d_b3, eq190_e2395_d_b4, eq190_e2395_d_b5, eq190_e2395_d_b6, eq190_e2395_d_b7, eq190_e2395_d_b8, eq190_e2395_d_b9, eq190_e2395_d_b10, eq190_e2395_d_b11, eq190_e2395_d_b12, eq190_e2395_d_b13, eq190_e2395_d_b14, eq190_e2395_d_b15, eq190_e2395_d_b16, eq190_e2395_d_b17, eq190_e2395_d_b18, eq190_e2395_d_b19, eq190_e2395_d_b20, eq190_e2395_d_b21, eq190_e2395_d_b22, eq190_e2395_d_b23, eq190_e2395_d_b24, eq190_e2395_d_b25, eq190_e2395_d_b26, eq190_e2395_d_b27, eq190_e2395_d_b28, eq190_e2395_d_b29, eq190_e2395_d_b30, eq190_e2395_d_b31, eq190_e2395_d_b32, eq190_e2395_d_b33, eq190_e2395_d_b34, eq190_e2395_d_b35, eq190_e2395_d_b36, eq190_e2395_d_b37, eq190_e2395_d_b38, eq190_e2395_d_b39, eq190_e2395_d_b40, eq190_e2395_d_b41, eq190_e2395_d_b42, eq190_e2395_d_b43, eq190_e2395_d_b44, eq190_e2395_d_b45, eq190_e2395_d_b46, eq190_e2395_d_b47, eq190_e2395_d_b48, eq190_e2395_d_b49, eq190_e2395_d_b50, eq190_e2395_d_b51, eq190_e2395_d_b52, eq190_e2395_d_b53, eq190_e2395_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq190_value),
            &eq190_node_derivatives,
            &eq190_branch_derivatives,
            multiplicity,
        );
        let (eq191_e2407, eq191_e2407_d_n0, eq191_e2407_d_n1, eq191_e2407_d_n2, eq191_e2407_d_n3, eq191_e2407_d_n4, eq191_e2407_d_n5, eq191_e2407_d_n6, eq191_e2407_d_n7, eq191_e2407_d_n8, eq191_e2407_d_n9, eq191_e2407_d_n10, eq191_e2407_d_n11, eq191_e2407_d_n12, eq191_e2407_d_n13, eq191_e2407_d_n14, eq191_e2407_d_n15, eq191_e2407_d_n16, eq191_e2407_d_n17, eq191_e2407_d_n18, eq191_e2407_d_n19, eq191_e2407_d_n20, eq191_e2407_d_n21, eq191_e2407_d_n22, eq191_e2407_d_b0, eq191_e2407_d_b1, eq191_e2407_d_b2, eq191_e2407_d_b3, eq191_e2407_d_b4, eq191_e2407_d_b5, eq191_e2407_d_b6, eq191_e2407_d_b7, eq191_e2407_d_b8, eq191_e2407_d_b9, eq191_e2407_d_b10, eq191_e2407_d_b11, eq191_e2407_d_b12, eq191_e2407_d_b13, eq191_e2407_d_b14, eq191_e2407_d_b15, eq191_e2407_d_b16, eq191_e2407_d_b17, eq191_e2407_d_b18, eq191_e2407_d_b19, eq191_e2407_d_b20, eq191_e2407_d_b21, eq191_e2407_d_b22, eq191_e2407_d_b23, eq191_e2407_d_b24, eq191_e2407_d_b25, eq191_e2407_d_b26, eq191_e2407_d_b27, eq191_e2407_d_b28, eq191_e2407_d_b29, eq191_e2407_d_b30, eq191_e2407_d_b31, eq191_e2407_d_b32, eq191_e2407_d_b33, eq191_e2407_d_b34, eq191_e2407_d_b35, eq191_e2407_d_b36, eq191_e2407_d_b37, eq191_e2407_d_b38, eq191_e2407_d_b39, eq191_e2407_d_b40, eq191_e2407_d_b41, eq191_e2407_d_b42, eq191_e2407_d_b43, eq191_e2407_d_b44, eq191_e2407_d_b45, eq191_e2407_d_b46, eq191_e2407_d_b47, eq191_e2407_d_b48, eq191_e2407_d_b49, eq191_e2407_d_b50, eq191_e2407_d_b51, eq191_e2407_d_b52, eq191_e2407_d_b53, eq191_e2407_d_b54,) = {
    if ((!s.b[595]) && s.b[598]) {
        let eq191_e2403: f64 = (p.p253 * s.v[288]);
        let eq191_e2404: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 90, eq191_e2403);
        let eq191_e2404_d_n0: f64 = ((p.p253 * s.dn[288][0]) * ddt_scale);
        let eq191_e2404_d_n1: f64 = ((p.p253 * s.dn[288][1]) * ddt_scale);
        let eq191_e2404_d_n2: f64 = ((p.p253 * s.dn[288][2]) * ddt_scale);
        let eq191_e2404_d_n3: f64 = ((p.p253 * s.dn[288][3]) * ddt_scale);
        let eq191_e2404_d_n4: f64 = ((p.p253 * s.dn[288][4]) * ddt_scale);
        let eq191_e2404_d_n5: f64 = ((p.p253 * s.dn[288][5]) * ddt_scale);
        let eq191_e2404_d_n6: f64 = ((p.p253 * s.dn[288][6]) * ddt_scale);
        let eq191_e2404_d_n7: f64 = ((p.p253 * s.dn[288][7]) * ddt_scale);
        let eq191_e2404_d_n8: f64 = ((p.p253 * s.dn[288][8]) * ddt_scale);
        let eq191_e2404_d_n9: f64 = ((p.p253 * s.dn[288][9]) * ddt_scale);
        let eq191_e2404_d_n10: f64 = ((p.p253 * s.dn[288][10]) * ddt_scale);
        let eq191_e2404_d_n11: f64 = ((p.p253 * s.dn[288][11]) * ddt_scale);
        let eq191_e2404_d_n12: f64 = ((p.p253 * s.dn[288][12]) * ddt_scale);
        let eq191_e2404_d_n13: f64 = ((p.p253 * s.dn[288][13]) * ddt_scale);
        let eq191_e2404_d_n14: f64 = ((p.p253 * s.dn[288][14]) * ddt_scale);
        let eq191_e2404_d_n15: f64 = ((p.p253 * s.dn[288][15]) * ddt_scale);
        let eq191_e2404_d_n16: f64 = ((p.p253 * s.dn[288][16]) * ddt_scale);
        let eq191_e2404_d_n17: f64 = ((p.p253 * s.dn[288][17]) * ddt_scale);
        let eq191_e2404_d_n18: f64 = ((p.p253 * s.dn[288][18]) * ddt_scale);
        let eq191_e2404_d_n19: f64 = ((p.p253 * s.dn[288][19]) * ddt_scale);
        let eq191_e2404_d_n20: f64 = ((p.p253 * s.dn[288][20]) * ddt_scale);
        let eq191_e2404_d_n21: f64 = ((p.p253 * s.dn[288][21]) * ddt_scale);
        let eq191_e2404_d_n22: f64 = ((p.p253 * s.dn[288][22]) * ddt_scale);
        let eq191_e2404_d_b0: f64 = ((p.p253 * s.db[288][0]) * ddt_scale);
        let eq191_e2404_d_b1: f64 = ((p.p253 * s.db[288][1]) * ddt_scale);
        let eq191_e2404_d_b2: f64 = ((p.p253 * s.db[288][2]) * ddt_scale);
        let eq191_e2404_d_b3: f64 = ((p.p253 * s.db[288][3]) * ddt_scale);
        let eq191_e2404_d_b4: f64 = ((p.p253 * s.db[288][4]) * ddt_scale);
        let eq191_e2404_d_b5: f64 = ((p.p253 * s.db[288][5]) * ddt_scale);
        let eq191_e2404_d_b6: f64 = ((p.p253 * s.db[288][6]) * ddt_scale);
        let eq191_e2404_d_b7: f64 = ((p.p253 * s.db[288][7]) * ddt_scale);
        let eq191_e2404_d_b8: f64 = ((p.p253 * s.db[288][8]) * ddt_scale);
        let eq191_e2404_d_b9: f64 = ((p.p253 * s.db[288][9]) * ddt_scale);
        let eq191_e2404_d_b10: f64 = ((p.p253 * s.db[288][10]) * ddt_scale);
        let eq191_e2404_d_b11: f64 = ((p.p253 * s.db[288][11]) * ddt_scale);
        let eq191_e2404_d_b12: f64 = ((p.p253 * s.db[288][12]) * ddt_scale);
        let eq191_e2404_d_b13: f64 = ((p.p253 * s.db[288][13]) * ddt_scale);
        let eq191_e2404_d_b14: f64 = ((p.p253 * s.db[288][14]) * ddt_scale);
        let eq191_e2404_d_b15: f64 = ((p.p253 * s.db[288][15]) * ddt_scale);
        let eq191_e2404_d_b16: f64 = ((p.p253 * s.db[288][16]) * ddt_scale);
        let eq191_e2404_d_b17: f64 = ((p.p253 * s.db[288][17]) * ddt_scale);
        let eq191_e2404_d_b18: f64 = ((p.p253 * s.db[288][18]) * ddt_scale);
        let eq191_e2404_d_b19: f64 = ((p.p253 * s.db[288][19]) * ddt_scale);
        let eq191_e2404_d_b20: f64 = ((p.p253 * s.db[288][20]) * ddt_scale);
        let eq191_e2404_d_b21: f64 = ((p.p253 * s.db[288][21]) * ddt_scale);
        let eq191_e2404_d_b22: f64 = ((p.p253 * s.db[288][22]) * ddt_scale);
        let eq191_e2404_d_b23: f64 = ((p.p253 * s.db[288][23]) * ddt_scale);
        let eq191_e2404_d_b24: f64 = ((p.p253 * s.db[288][24]) * ddt_scale);
        let eq191_e2404_d_b25: f64 = ((p.p253 * s.db[288][25]) * ddt_scale);
        let eq191_e2404_d_b26: f64 = ((p.p253 * s.db[288][26]) * ddt_scale);
        let eq191_e2404_d_b27: f64 = ((p.p253 * s.db[288][27]) * ddt_scale);
        let eq191_e2404_d_b28: f64 = ((p.p253 * s.db[288][28]) * ddt_scale);
        let eq191_e2404_d_b29: f64 = ((p.p253 * s.db[288][29]) * ddt_scale);
        let eq191_e2404_d_b30: f64 = ((p.p253 * s.db[288][30]) * ddt_scale);
        let eq191_e2404_d_b31: f64 = ((p.p253 * s.db[288][31]) * ddt_scale);
        let eq191_e2404_d_b32: f64 = ((p.p253 * s.db[288][32]) * ddt_scale);
        let eq191_e2404_d_b33: f64 = ((p.p253 * s.db[288][33]) * ddt_scale);
        let eq191_e2404_d_b34: f64 = ((p.p253 * s.db[288][34]) * ddt_scale);
        let eq191_e2404_d_b35: f64 = ((p.p253 * s.db[288][35]) * ddt_scale);
        let eq191_e2404_d_b36: f64 = ((p.p253 * s.db[288][36]) * ddt_scale);
        let eq191_e2404_d_b37: f64 = ((p.p253 * s.db[288][37]) * ddt_scale);
        let eq191_e2404_d_b38: f64 = ((p.p253 * s.db[288][38]) * ddt_scale);
        let eq191_e2404_d_b39: f64 = ((p.p253 * s.db[288][39]) * ddt_scale);
        let eq191_e2404_d_b40: f64 = ((p.p253 * s.db[288][40]) * ddt_scale);
        let eq191_e2404_d_b41: f64 = ((p.p253 * s.db[288][41]) * ddt_scale);
        let eq191_e2404_d_b42: f64 = ((p.p253 * s.db[288][42]) * ddt_scale);
        let eq191_e2404_d_b43: f64 = ((p.p253 * s.db[288][43]) * ddt_scale);
        let eq191_e2404_d_b44: f64 = ((p.p253 * s.db[288][44]) * ddt_scale);
        let eq191_e2404_d_b45: f64 = ((p.p253 * s.db[288][45]) * ddt_scale);
        let eq191_e2404_d_b46: f64 = ((p.p253 * s.db[288][46]) * ddt_scale);
        let eq191_e2404_d_b47: f64 = ((p.p253 * s.db[288][47]) * ddt_scale);
        let eq191_e2404_d_b48: f64 = ((p.p253 * s.db[288][48]) * ddt_scale);
        let eq191_e2404_d_b49: f64 = ((p.p253 * s.db[288][49]) * ddt_scale);
        let eq191_e2404_d_b50: f64 = ((p.p253 * s.db[288][50]) * ddt_scale);
        let eq191_e2404_d_b51: f64 = ((p.p253 * s.db[288][51]) * ddt_scale);
        let eq191_e2404_d_b52: f64 = ((p.p253 * s.db[288][52]) * ddt_scale);
        let eq191_e2404_d_b53: f64 = ((p.p253 * s.db[288][53]) * ddt_scale);
        let eq191_e2404_d_b54: f64 = ((p.p253 * s.db[288][54]) * ddt_scale);
        let eq191_e2405: f64 = (p.p7 * eq191_e2404);
        let eq191_e2405_d_n0: f64 = (p.p7 * eq191_e2404_d_n0);
        let eq191_e2405_d_n1: f64 = (p.p7 * eq191_e2404_d_n1);
        let eq191_e2405_d_n2: f64 = (p.p7 * eq191_e2404_d_n2);
        let eq191_e2405_d_n3: f64 = (p.p7 * eq191_e2404_d_n3);
        let eq191_e2405_d_n4: f64 = (p.p7 * eq191_e2404_d_n4);
        let eq191_e2405_d_n5: f64 = (p.p7 * eq191_e2404_d_n5);
        let eq191_e2405_d_n6: f64 = (p.p7 * eq191_e2404_d_n6);
        let eq191_e2405_d_n7: f64 = (p.p7 * eq191_e2404_d_n7);
        let eq191_e2405_d_n8: f64 = (p.p7 * eq191_e2404_d_n8);
        let eq191_e2405_d_n9: f64 = (p.p7 * eq191_e2404_d_n9);
        let eq191_e2405_d_n10: f64 = (p.p7 * eq191_e2404_d_n10);
        let eq191_e2405_d_n11: f64 = (p.p7 * eq191_e2404_d_n11);
        let eq191_e2405_d_n12: f64 = (p.p7 * eq191_e2404_d_n12);
        let eq191_e2405_d_n13: f64 = (p.p7 * eq191_e2404_d_n13);
        let eq191_e2405_d_n14: f64 = (p.p7 * eq191_e2404_d_n14);
        let eq191_e2405_d_n15: f64 = (p.p7 * eq191_e2404_d_n15);
        let eq191_e2405_d_n16: f64 = (p.p7 * eq191_e2404_d_n16);
        let eq191_e2405_d_n17: f64 = (p.p7 * eq191_e2404_d_n17);
        let eq191_e2405_d_n18: f64 = (p.p7 * eq191_e2404_d_n18);
        let eq191_e2405_d_n19: f64 = (p.p7 * eq191_e2404_d_n19);
        let eq191_e2405_d_n20: f64 = (p.p7 * eq191_e2404_d_n20);
        let eq191_e2405_d_n21: f64 = (p.p7 * eq191_e2404_d_n21);
        let eq191_e2405_d_n22: f64 = (p.p7 * eq191_e2404_d_n22);
        let eq191_e2405_d_b0: f64 = (p.p7 * eq191_e2404_d_b0);
        let eq191_e2405_d_b1: f64 = (p.p7 * eq191_e2404_d_b1);
        let eq191_e2405_d_b2: f64 = (p.p7 * eq191_e2404_d_b2);
        let eq191_e2405_d_b3: f64 = (p.p7 * eq191_e2404_d_b3);
        let eq191_e2405_d_b4: f64 = (p.p7 * eq191_e2404_d_b4);
        let eq191_e2405_d_b5: f64 = (p.p7 * eq191_e2404_d_b5);
        let eq191_e2405_d_b6: f64 = (p.p7 * eq191_e2404_d_b6);
        let eq191_e2405_d_b7: f64 = (p.p7 * eq191_e2404_d_b7);
        let eq191_e2405_d_b8: f64 = (p.p7 * eq191_e2404_d_b8);
        let eq191_e2405_d_b9: f64 = (p.p7 * eq191_e2404_d_b9);
        let eq191_e2405_d_b10: f64 = (p.p7 * eq191_e2404_d_b10);
        let eq191_e2405_d_b11: f64 = (p.p7 * eq191_e2404_d_b11);
        let eq191_e2405_d_b12: f64 = (p.p7 * eq191_e2404_d_b12);
        let eq191_e2405_d_b13: f64 = (p.p7 * eq191_e2404_d_b13);
        let eq191_e2405_d_b14: f64 = (p.p7 * eq191_e2404_d_b14);
        let eq191_e2405_d_b15: f64 = (p.p7 * eq191_e2404_d_b15);
        let eq191_e2405_d_b16: f64 = (p.p7 * eq191_e2404_d_b16);
        let eq191_e2405_d_b17: f64 = (p.p7 * eq191_e2404_d_b17);
        let eq191_e2405_d_b18: f64 = (p.p7 * eq191_e2404_d_b18);
        let eq191_e2405_d_b19: f64 = (p.p7 * eq191_e2404_d_b19);
        let eq191_e2405_d_b20: f64 = (p.p7 * eq191_e2404_d_b20);
        let eq191_e2405_d_b21: f64 = (p.p7 * eq191_e2404_d_b21);
        let eq191_e2405_d_b22: f64 = (p.p7 * eq191_e2404_d_b22);
        let eq191_e2405_d_b23: f64 = (p.p7 * eq191_e2404_d_b23);
        let eq191_e2405_d_b24: f64 = (p.p7 * eq191_e2404_d_b24);
        let eq191_e2405_d_b25: f64 = (p.p7 * eq191_e2404_d_b25);
        let eq191_e2405_d_b26: f64 = (p.p7 * eq191_e2404_d_b26);
        let eq191_e2405_d_b27: f64 = (p.p7 * eq191_e2404_d_b27);
        let eq191_e2405_d_b28: f64 = (p.p7 * eq191_e2404_d_b28);
        let eq191_e2405_d_b29: f64 = (p.p7 * eq191_e2404_d_b29);
        let eq191_e2405_d_b30: f64 = (p.p7 * eq191_e2404_d_b30);
        let eq191_e2405_d_b31: f64 = (p.p7 * eq191_e2404_d_b31);
        let eq191_e2405_d_b32: f64 = (p.p7 * eq191_e2404_d_b32);
        let eq191_e2405_d_b33: f64 = (p.p7 * eq191_e2404_d_b33);
        let eq191_e2405_d_b34: f64 = (p.p7 * eq191_e2404_d_b34);
        let eq191_e2405_d_b35: f64 = (p.p7 * eq191_e2404_d_b35);
        let eq191_e2405_d_b36: f64 = (p.p7 * eq191_e2404_d_b36);
        let eq191_e2405_d_b37: f64 = (p.p7 * eq191_e2404_d_b37);
        let eq191_e2405_d_b38: f64 = (p.p7 * eq191_e2404_d_b38);
        let eq191_e2405_d_b39: f64 = (p.p7 * eq191_e2404_d_b39);
        let eq191_e2405_d_b40: f64 = (p.p7 * eq191_e2404_d_b40);
        let eq191_e2405_d_b41: f64 = (p.p7 * eq191_e2404_d_b41);
        let eq191_e2405_d_b42: f64 = (p.p7 * eq191_e2404_d_b42);
        let eq191_e2405_d_b43: f64 = (p.p7 * eq191_e2404_d_b43);
        let eq191_e2405_d_b44: f64 = (p.p7 * eq191_e2404_d_b44);
        let eq191_e2405_d_b45: f64 = (p.p7 * eq191_e2404_d_b45);
        let eq191_e2405_d_b46: f64 = (p.p7 * eq191_e2404_d_b46);
        let eq191_e2405_d_b47: f64 = (p.p7 * eq191_e2404_d_b47);
        let eq191_e2405_d_b48: f64 = (p.p7 * eq191_e2404_d_b48);
        let eq191_e2405_d_b49: f64 = (p.p7 * eq191_e2404_d_b49);
        let eq191_e2405_d_b50: f64 = (p.p7 * eq191_e2404_d_b50);
        let eq191_e2405_d_b51: f64 = (p.p7 * eq191_e2404_d_b51);
        let eq191_e2405_d_b52: f64 = (p.p7 * eq191_e2404_d_b52);
        let eq191_e2405_d_b53: f64 = (p.p7 * eq191_e2404_d_b53);
        let eq191_e2405_d_b54: f64 = (p.p7 * eq191_e2404_d_b54);
        (eq191_e2405, eq191_e2405_d_n0, eq191_e2405_d_n1, eq191_e2405_d_n2, eq191_e2405_d_n3, eq191_e2405_d_n4, eq191_e2405_d_n5, eq191_e2405_d_n6, eq191_e2405_d_n7, eq191_e2405_d_n8, eq191_e2405_d_n9, eq191_e2405_d_n10, eq191_e2405_d_n11, eq191_e2405_d_n12, eq191_e2405_d_n13, eq191_e2405_d_n14, eq191_e2405_d_n15, eq191_e2405_d_n16, eq191_e2405_d_n17, eq191_e2405_d_n18, eq191_e2405_d_n19, eq191_e2405_d_n20, eq191_e2405_d_n21, eq191_e2405_d_n22, eq191_e2405_d_b0, eq191_e2405_d_b1, eq191_e2405_d_b2, eq191_e2405_d_b3, eq191_e2405_d_b4, eq191_e2405_d_b5, eq191_e2405_d_b6, eq191_e2405_d_b7, eq191_e2405_d_b8, eq191_e2405_d_b9, eq191_e2405_d_b10, eq191_e2405_d_b11, eq191_e2405_d_b12, eq191_e2405_d_b13, eq191_e2405_d_b14, eq191_e2405_d_b15, eq191_e2405_d_b16, eq191_e2405_d_b17, eq191_e2405_d_b18, eq191_e2405_d_b19, eq191_e2405_d_b20, eq191_e2405_d_b21, eq191_e2405_d_b22, eq191_e2405_d_b23, eq191_e2405_d_b24, eq191_e2405_d_b25, eq191_e2405_d_b26, eq191_e2405_d_b27, eq191_e2405_d_b28, eq191_e2405_d_b29, eq191_e2405_d_b30, eq191_e2405_d_b31, eq191_e2405_d_b32, eq191_e2405_d_b33, eq191_e2405_d_b34, eq191_e2405_d_b35, eq191_e2405_d_b36, eq191_e2405_d_b37, eq191_e2405_d_b38, eq191_e2405_d_b39, eq191_e2405_d_b40, eq191_e2405_d_b41, eq191_e2405_d_b42, eq191_e2405_d_b43, eq191_e2405_d_b44, eq191_e2405_d_b45, eq191_e2405_d_b46, eq191_e2405_d_b47, eq191_e2405_d_b48, eq191_e2405_d_b49, eq191_e2405_d_b50, eq191_e2405_d_b51, eq191_e2405_d_b52, eq191_e2405_d_b53, eq191_e2405_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq191_value: f64 = eq191_e2407;
        let eq191_node_derivatives: [f64; 23] = [eq191_e2407_d_n0, eq191_e2407_d_n1, eq191_e2407_d_n2, eq191_e2407_d_n3, eq191_e2407_d_n4, eq191_e2407_d_n5, eq191_e2407_d_n6, eq191_e2407_d_n7, eq191_e2407_d_n8, eq191_e2407_d_n9, eq191_e2407_d_n10, eq191_e2407_d_n11, eq191_e2407_d_n12, eq191_e2407_d_n13, eq191_e2407_d_n14, eq191_e2407_d_n15, eq191_e2407_d_n16, eq191_e2407_d_n17, eq191_e2407_d_n18, eq191_e2407_d_n19, eq191_e2407_d_n20, eq191_e2407_d_n21, eq191_e2407_d_n22];
        let eq191_branch_derivatives: [f64; 55] = [eq191_e2407_d_b0, eq191_e2407_d_b1, eq191_e2407_d_b2, eq191_e2407_d_b3, eq191_e2407_d_b4, eq191_e2407_d_b5, eq191_e2407_d_b6, eq191_e2407_d_b7, eq191_e2407_d_b8, eq191_e2407_d_b9, eq191_e2407_d_b10, eq191_e2407_d_b11, eq191_e2407_d_b12, eq191_e2407_d_b13, eq191_e2407_d_b14, eq191_e2407_d_b15, eq191_e2407_d_b16, eq191_e2407_d_b17, eq191_e2407_d_b18, eq191_e2407_d_b19, eq191_e2407_d_b20, eq191_e2407_d_b21, eq191_e2407_d_b22, eq191_e2407_d_b23, eq191_e2407_d_b24, eq191_e2407_d_b25, eq191_e2407_d_b26, eq191_e2407_d_b27, eq191_e2407_d_b28, eq191_e2407_d_b29, eq191_e2407_d_b30, eq191_e2407_d_b31, eq191_e2407_d_b32, eq191_e2407_d_b33, eq191_e2407_d_b34, eq191_e2407_d_b35, eq191_e2407_d_b36, eq191_e2407_d_b37, eq191_e2407_d_b38, eq191_e2407_d_b39, eq191_e2407_d_b40, eq191_e2407_d_b41, eq191_e2407_d_b42, eq191_e2407_d_b43, eq191_e2407_d_b44, eq191_e2407_d_b45, eq191_e2407_d_b46, eq191_e2407_d_b47, eq191_e2407_d_b48, eq191_e2407_d_b49, eq191_e2407_d_b50, eq191_e2407_d_b51, eq191_e2407_d_b52, eq191_e2407_d_b53, eq191_e2407_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq191_value),
            &eq191_node_derivatives,
            &eq191_branch_derivatives,
            multiplicity,
        );
        let (eq192_e2416, eq192_e2416_d_n0, eq192_e2416_d_n1, eq192_e2416_d_n2, eq192_e2416_d_n3, eq192_e2416_d_n4, eq192_e2416_d_n5, eq192_e2416_d_n6, eq192_e2416_d_n7, eq192_e2416_d_n8, eq192_e2416_d_n9, eq192_e2416_d_n10, eq192_e2416_d_n11, eq192_e2416_d_n12, eq192_e2416_d_n13, eq192_e2416_d_n14, eq192_e2416_d_n15, eq192_e2416_d_n16, eq192_e2416_d_n17, eq192_e2416_d_n18, eq192_e2416_d_n19, eq192_e2416_d_n20, eq192_e2416_d_n21, eq192_e2416_d_n22, eq192_e2416_d_b0, eq192_e2416_d_b1, eq192_e2416_d_b2, eq192_e2416_d_b3, eq192_e2416_d_b4, eq192_e2416_d_b5, eq192_e2416_d_b6, eq192_e2416_d_b7, eq192_e2416_d_b8, eq192_e2416_d_b9, eq192_e2416_d_b10, eq192_e2416_d_b11, eq192_e2416_d_b12, eq192_e2416_d_b13, eq192_e2416_d_b14, eq192_e2416_d_b15, eq192_e2416_d_b16, eq192_e2416_d_b17, eq192_e2416_d_b18, eq192_e2416_d_b19, eq192_e2416_d_b20, eq192_e2416_d_b21, eq192_e2416_d_b22, eq192_e2416_d_b23, eq192_e2416_d_b24, eq192_e2416_d_b25, eq192_e2416_d_b26, eq192_e2416_d_b27, eq192_e2416_d_b28, eq192_e2416_d_b29, eq192_e2416_d_b30, eq192_e2416_d_b31, eq192_e2416_d_b32, eq192_e2416_d_b33, eq192_e2416_d_b34, eq192_e2416_d_b35, eq192_e2416_d_b36, eq192_e2416_d_b37, eq192_e2416_d_b38, eq192_e2416_d_b39, eq192_e2416_d_b40, eq192_e2416_d_b41, eq192_e2416_d_b42, eq192_e2416_d_b43, eq192_e2416_d_b44, eq192_e2416_d_b45, eq192_e2416_d_b46, eq192_e2416_d_b47, eq192_e2416_d_b48, eq192_e2416_d_b49, eq192_e2416_d_b50, eq192_e2416_d_b51, eq192_e2416_d_b52, eq192_e2416_d_b53, eq192_e2416_d_b54,) = {
    if (s.b[600] && s.b[601]) {
        let eq192_e2413: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 91, s.v[301]);
        let eq192_e2414: f64 = (p.p7 * eq192_e2413);
        let eq192_e2414_d_n0: f64 = (p.p7 * (s.dn[301][0] * ddt_scale));
        let eq192_e2414_d_n1: f64 = (p.p7 * (s.dn[301][1] * ddt_scale));
        let eq192_e2414_d_n2: f64 = (p.p7 * (s.dn[301][2] * ddt_scale));
        let eq192_e2414_d_n3: f64 = (p.p7 * (s.dn[301][3] * ddt_scale));
        let eq192_e2414_d_n4: f64 = (p.p7 * (s.dn[301][4] * ddt_scale));
        let eq192_e2414_d_n5: f64 = (p.p7 * (s.dn[301][5] * ddt_scale));
        let eq192_e2414_d_n6: f64 = (p.p7 * (s.dn[301][6] * ddt_scale));
        let eq192_e2414_d_n7: f64 = (p.p7 * (s.dn[301][7] * ddt_scale));
        let eq192_e2414_d_n8: f64 = (p.p7 * (s.dn[301][8] * ddt_scale));
        let eq192_e2414_d_n9: f64 = (p.p7 * (s.dn[301][9] * ddt_scale));
        let eq192_e2414_d_n10: f64 = (p.p7 * (s.dn[301][10] * ddt_scale));
        let eq192_e2414_d_n11: f64 = (p.p7 * (s.dn[301][11] * ddt_scale));
        let eq192_e2414_d_n12: f64 = (p.p7 * (s.dn[301][12] * ddt_scale));
        let eq192_e2414_d_n13: f64 = (p.p7 * (s.dn[301][13] * ddt_scale));
        let eq192_e2414_d_n14: f64 = (p.p7 * (s.dn[301][14] * ddt_scale));
        let eq192_e2414_d_n15: f64 = (p.p7 * (s.dn[301][15] * ddt_scale));
        let eq192_e2414_d_n16: f64 = (p.p7 * (s.dn[301][16] * ddt_scale));
        let eq192_e2414_d_n17: f64 = (p.p7 * (s.dn[301][17] * ddt_scale));
        let eq192_e2414_d_n18: f64 = (p.p7 * (s.dn[301][18] * ddt_scale));
        let eq192_e2414_d_n19: f64 = (p.p7 * (s.dn[301][19] * ddt_scale));
        let eq192_e2414_d_n20: f64 = (p.p7 * (s.dn[301][20] * ddt_scale));
        let eq192_e2414_d_n21: f64 = (p.p7 * (s.dn[301][21] * ddt_scale));
        let eq192_e2414_d_n22: f64 = (p.p7 * (s.dn[301][22] * ddt_scale));
        let eq192_e2414_d_b0: f64 = (p.p7 * (s.db[301][0] * ddt_scale));
        let eq192_e2414_d_b1: f64 = (p.p7 * (s.db[301][1] * ddt_scale));
        let eq192_e2414_d_b2: f64 = (p.p7 * (s.db[301][2] * ddt_scale));
        let eq192_e2414_d_b3: f64 = (p.p7 * (s.db[301][3] * ddt_scale));
        let eq192_e2414_d_b4: f64 = (p.p7 * (s.db[301][4] * ddt_scale));
        let eq192_e2414_d_b5: f64 = (p.p7 * (s.db[301][5] * ddt_scale));
        let eq192_e2414_d_b6: f64 = (p.p7 * (s.db[301][6] * ddt_scale));
        let eq192_e2414_d_b7: f64 = (p.p7 * (s.db[301][7] * ddt_scale));
        let eq192_e2414_d_b8: f64 = (p.p7 * (s.db[301][8] * ddt_scale));
        let eq192_e2414_d_b9: f64 = (p.p7 * (s.db[301][9] * ddt_scale));
        let eq192_e2414_d_b10: f64 = (p.p7 * (s.db[301][10] * ddt_scale));
        let eq192_e2414_d_b11: f64 = (p.p7 * (s.db[301][11] * ddt_scale));
        let eq192_e2414_d_b12: f64 = (p.p7 * (s.db[301][12] * ddt_scale));
        let eq192_e2414_d_b13: f64 = (p.p7 * (s.db[301][13] * ddt_scale));
        let eq192_e2414_d_b14: f64 = (p.p7 * (s.db[301][14] * ddt_scale));
        let eq192_e2414_d_b15: f64 = (p.p7 * (s.db[301][15] * ddt_scale));
        let eq192_e2414_d_b16: f64 = (p.p7 * (s.db[301][16] * ddt_scale));
        let eq192_e2414_d_b17: f64 = (p.p7 * (s.db[301][17] * ddt_scale));
        let eq192_e2414_d_b18: f64 = (p.p7 * (s.db[301][18] * ddt_scale));
        let eq192_e2414_d_b19: f64 = (p.p7 * (s.db[301][19] * ddt_scale));
        let eq192_e2414_d_b20: f64 = (p.p7 * (s.db[301][20] * ddt_scale));
        let eq192_e2414_d_b21: f64 = (p.p7 * (s.db[301][21] * ddt_scale));
        let eq192_e2414_d_b22: f64 = (p.p7 * (s.db[301][22] * ddt_scale));
        let eq192_e2414_d_b23: f64 = (p.p7 * (s.db[301][23] * ddt_scale));
        let eq192_e2414_d_b24: f64 = (p.p7 * (s.db[301][24] * ddt_scale));
        let eq192_e2414_d_b25: f64 = (p.p7 * (s.db[301][25] * ddt_scale));
        let eq192_e2414_d_b26: f64 = (p.p7 * (s.db[301][26] * ddt_scale));
        let eq192_e2414_d_b27: f64 = (p.p7 * (s.db[301][27] * ddt_scale));
        let eq192_e2414_d_b28: f64 = (p.p7 * (s.db[301][28] * ddt_scale));
        let eq192_e2414_d_b29: f64 = (p.p7 * (s.db[301][29] * ddt_scale));
        let eq192_e2414_d_b30: f64 = (p.p7 * (s.db[301][30] * ddt_scale));
        let eq192_e2414_d_b31: f64 = (p.p7 * (s.db[301][31] * ddt_scale));
        let eq192_e2414_d_b32: f64 = (p.p7 * (s.db[301][32] * ddt_scale));
        let eq192_e2414_d_b33: f64 = (p.p7 * (s.db[301][33] * ddt_scale));
        let eq192_e2414_d_b34: f64 = (p.p7 * (s.db[301][34] * ddt_scale));
        let eq192_e2414_d_b35: f64 = (p.p7 * (s.db[301][35] * ddt_scale));
        let eq192_e2414_d_b36: f64 = (p.p7 * (s.db[301][36] * ddt_scale));
        let eq192_e2414_d_b37: f64 = (p.p7 * (s.db[301][37] * ddt_scale));
        let eq192_e2414_d_b38: f64 = (p.p7 * (s.db[301][38] * ddt_scale));
        let eq192_e2414_d_b39: f64 = (p.p7 * (s.db[301][39] * ddt_scale));
        let eq192_e2414_d_b40: f64 = (p.p7 * (s.db[301][40] * ddt_scale));
        let eq192_e2414_d_b41: f64 = (p.p7 * (s.db[301][41] * ddt_scale));
        let eq192_e2414_d_b42: f64 = (p.p7 * (s.db[301][42] * ddt_scale));
        let eq192_e2414_d_b43: f64 = (p.p7 * (s.db[301][43] * ddt_scale));
        let eq192_e2414_d_b44: f64 = (p.p7 * (s.db[301][44] * ddt_scale));
        let eq192_e2414_d_b45: f64 = (p.p7 * (s.db[301][45] * ddt_scale));
        let eq192_e2414_d_b46: f64 = (p.p7 * (s.db[301][46] * ddt_scale));
        let eq192_e2414_d_b47: f64 = (p.p7 * (s.db[301][47] * ddt_scale));
        let eq192_e2414_d_b48: f64 = (p.p7 * (s.db[301][48] * ddt_scale));
        let eq192_e2414_d_b49: f64 = (p.p7 * (s.db[301][49] * ddt_scale));
        let eq192_e2414_d_b50: f64 = (p.p7 * (s.db[301][50] * ddt_scale));
        let eq192_e2414_d_b51: f64 = (p.p7 * (s.db[301][51] * ddt_scale));
        let eq192_e2414_d_b52: f64 = (p.p7 * (s.db[301][52] * ddt_scale));
        let eq192_e2414_d_b53: f64 = (p.p7 * (s.db[301][53] * ddt_scale));
        let eq192_e2414_d_b54: f64 = (p.p7 * (s.db[301][54] * ddt_scale));
        (eq192_e2414, eq192_e2414_d_n0, eq192_e2414_d_n1, eq192_e2414_d_n2, eq192_e2414_d_n3, eq192_e2414_d_n4, eq192_e2414_d_n5, eq192_e2414_d_n6, eq192_e2414_d_n7, eq192_e2414_d_n8, eq192_e2414_d_n9, eq192_e2414_d_n10, eq192_e2414_d_n11, eq192_e2414_d_n12, eq192_e2414_d_n13, eq192_e2414_d_n14, eq192_e2414_d_n15, eq192_e2414_d_n16, eq192_e2414_d_n17, eq192_e2414_d_n18, eq192_e2414_d_n19, eq192_e2414_d_n20, eq192_e2414_d_n21, eq192_e2414_d_n22, eq192_e2414_d_b0, eq192_e2414_d_b1, eq192_e2414_d_b2, eq192_e2414_d_b3, eq192_e2414_d_b4, eq192_e2414_d_b5, eq192_e2414_d_b6, eq192_e2414_d_b7, eq192_e2414_d_b8, eq192_e2414_d_b9, eq192_e2414_d_b10, eq192_e2414_d_b11, eq192_e2414_d_b12, eq192_e2414_d_b13, eq192_e2414_d_b14, eq192_e2414_d_b15, eq192_e2414_d_b16, eq192_e2414_d_b17, eq192_e2414_d_b18, eq192_e2414_d_b19, eq192_e2414_d_b20, eq192_e2414_d_b21, eq192_e2414_d_b22, eq192_e2414_d_b23, eq192_e2414_d_b24, eq192_e2414_d_b25, eq192_e2414_d_b26, eq192_e2414_d_b27, eq192_e2414_d_b28, eq192_e2414_d_b29, eq192_e2414_d_b30, eq192_e2414_d_b31, eq192_e2414_d_b32, eq192_e2414_d_b33, eq192_e2414_d_b34, eq192_e2414_d_b35, eq192_e2414_d_b36, eq192_e2414_d_b37, eq192_e2414_d_b38, eq192_e2414_d_b39, eq192_e2414_d_b40, eq192_e2414_d_b41, eq192_e2414_d_b42, eq192_e2414_d_b43, eq192_e2414_d_b44, eq192_e2414_d_b45, eq192_e2414_d_b46, eq192_e2414_d_b47, eq192_e2414_d_b48, eq192_e2414_d_b49, eq192_e2414_d_b50, eq192_e2414_d_b51, eq192_e2414_d_b52, eq192_e2414_d_b53, eq192_e2414_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq192_value: f64 = eq192_e2416;
        let eq192_node_derivatives: [f64; 23] = [eq192_e2416_d_n0, eq192_e2416_d_n1, eq192_e2416_d_n2, eq192_e2416_d_n3, eq192_e2416_d_n4, eq192_e2416_d_n5, eq192_e2416_d_n6, eq192_e2416_d_n7, eq192_e2416_d_n8, eq192_e2416_d_n9, eq192_e2416_d_n10, eq192_e2416_d_n11, eq192_e2416_d_n12, eq192_e2416_d_n13, eq192_e2416_d_n14, eq192_e2416_d_n15, eq192_e2416_d_n16, eq192_e2416_d_n17, eq192_e2416_d_n18, eq192_e2416_d_n19, eq192_e2416_d_n20, eq192_e2416_d_n21, eq192_e2416_d_n22];
        let eq192_branch_derivatives: [f64; 55] = [eq192_e2416_d_b0, eq192_e2416_d_b1, eq192_e2416_d_b2, eq192_e2416_d_b3, eq192_e2416_d_b4, eq192_e2416_d_b5, eq192_e2416_d_b6, eq192_e2416_d_b7, eq192_e2416_d_b8, eq192_e2416_d_b9, eq192_e2416_d_b10, eq192_e2416_d_b11, eq192_e2416_d_b12, eq192_e2416_d_b13, eq192_e2416_d_b14, eq192_e2416_d_b15, eq192_e2416_d_b16, eq192_e2416_d_b17, eq192_e2416_d_b18, eq192_e2416_d_b19, eq192_e2416_d_b20, eq192_e2416_d_b21, eq192_e2416_d_b22, eq192_e2416_d_b23, eq192_e2416_d_b24, eq192_e2416_d_b25, eq192_e2416_d_b26, eq192_e2416_d_b27, eq192_e2416_d_b28, eq192_e2416_d_b29, eq192_e2416_d_b30, eq192_e2416_d_b31, eq192_e2416_d_b32, eq192_e2416_d_b33, eq192_e2416_d_b34, eq192_e2416_d_b35, eq192_e2416_d_b36, eq192_e2416_d_b37, eq192_e2416_d_b38, eq192_e2416_d_b39, eq192_e2416_d_b40, eq192_e2416_d_b41, eq192_e2416_d_b42, eq192_e2416_d_b43, eq192_e2416_d_b44, eq192_e2416_d_b45, eq192_e2416_d_b46, eq192_e2416_d_b47, eq192_e2416_d_b48, eq192_e2416_d_b49, eq192_e2416_d_b50, eq192_e2416_d_b51, eq192_e2416_d_b52, eq192_e2416_d_b53, eq192_e2416_d_b54];
        stamper.stamp_current_dense_local(
            Some(18),
            Some(17),
            multiplicity * (eq192_value),
            &eq192_node_derivatives,
            &eq192_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_39(
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[300][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[300][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[300][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[300][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[300][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[300][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[300][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[300][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[300][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[300][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[300][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[300][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[300][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[300][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[300][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[300][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[300][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[300][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[300][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[300][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[300][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[300][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[300][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[300][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[300][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[300][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[300][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[300][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[300][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[300][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[300][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[300][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[300][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[300][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[300][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[300][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[300][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[300][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[300][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[300][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[300][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[300][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[300][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[300][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[300][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[300][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[300][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[300][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[300][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[300][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[300][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[300][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[300][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[300][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[300][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[300][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[300][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[300][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[300][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[300][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[300][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[300][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[300][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[300][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[300][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[300][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[300][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[300][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[300][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[300][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[300][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[300][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[300][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[300][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[300][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[300][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[300][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[300][54] * ddt_scale));
        let (eq193_e2427, eq193_e2427_d_n0, eq193_e2427_d_n1, eq193_e2427_d_n2, eq193_e2427_d_n3, eq193_e2427_d_n4, eq193_e2427_d_n5, eq193_e2427_d_n6, eq193_e2427_d_n7, eq193_e2427_d_n8, eq193_e2427_d_n9, eq193_e2427_d_n10, eq193_e2427_d_n11, eq193_e2427_d_n12, eq193_e2427_d_n13, eq193_e2427_d_n14, eq193_e2427_d_n15, eq193_e2427_d_n16, eq193_e2427_d_n17, eq193_e2427_d_n18, eq193_e2427_d_n19, eq193_e2427_d_n20, eq193_e2427_d_n21, eq193_e2427_d_n22, eq193_e2427_d_b0, eq193_e2427_d_b1, eq193_e2427_d_b2, eq193_e2427_d_b3, eq193_e2427_d_b4, eq193_e2427_d_b5, eq193_e2427_d_b6, eq193_e2427_d_b7, eq193_e2427_d_b8, eq193_e2427_d_b9, eq193_e2427_d_b10, eq193_e2427_d_b11, eq193_e2427_d_b12, eq193_e2427_d_b13, eq193_e2427_d_b14, eq193_e2427_d_b15, eq193_e2427_d_b16, eq193_e2427_d_b17, eq193_e2427_d_b18, eq193_e2427_d_b19, eq193_e2427_d_b20, eq193_e2427_d_b21, eq193_e2427_d_b22, eq193_e2427_d_b23, eq193_e2427_d_b24, eq193_e2427_d_b25, eq193_e2427_d_b26, eq193_e2427_d_b27, eq193_e2427_d_b28, eq193_e2427_d_b29, eq193_e2427_d_b30, eq193_e2427_d_b31, eq193_e2427_d_b32, eq193_e2427_d_b33, eq193_e2427_d_b34, eq193_e2427_d_b35, eq193_e2427_d_b36, eq193_e2427_d_b37, eq193_e2427_d_b38, eq193_e2427_d_b39, eq193_e2427_d_b40, eq193_e2427_d_b41, eq193_e2427_d_b42, eq193_e2427_d_b43, eq193_e2427_d_b44, eq193_e2427_d_b45, eq193_e2427_d_b46, eq193_e2427_d_b47, eq193_e2427_d_b48, eq193_e2427_d_b49, eq193_e2427_d_b50, eq193_e2427_d_b51, eq193_e2427_d_b52, eq193_e2427_d_b53, eq193_e2427_d_b54,) = {
    if ((s.b[600] && s.b[601]) && s.b[602]) {
        let eq193_e2424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 92, s.v[300]);
        let eq193_e2425: f64 = (p.p7 * eq193_e2424);
        (eq193_e2425, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq193_value: f64 = eq193_e2427;
        let eq193_node_derivatives: [f64; 23] = [eq193_e2427_d_n0, eq193_e2427_d_n1, eq193_e2427_d_n2, eq193_e2427_d_n3, eq193_e2427_d_n4, eq193_e2427_d_n5, eq193_e2427_d_n6, eq193_e2427_d_n7, eq193_e2427_d_n8, eq193_e2427_d_n9, eq193_e2427_d_n10, eq193_e2427_d_n11, eq193_e2427_d_n12, eq193_e2427_d_n13, eq193_e2427_d_n14, eq193_e2427_d_n15, eq193_e2427_d_n16, eq193_e2427_d_n17, eq193_e2427_d_n18, eq193_e2427_d_n19, eq193_e2427_d_n20, eq193_e2427_d_n21, eq193_e2427_d_n22];
        let eq193_branch_derivatives: [f64; 55] = [eq193_e2427_d_b0, eq193_e2427_d_b1, eq193_e2427_d_b2, eq193_e2427_d_b3, eq193_e2427_d_b4, eq193_e2427_d_b5, eq193_e2427_d_b6, eq193_e2427_d_b7, eq193_e2427_d_b8, eq193_e2427_d_b9, eq193_e2427_d_b10, eq193_e2427_d_b11, eq193_e2427_d_b12, eq193_e2427_d_b13, eq193_e2427_d_b14, eq193_e2427_d_b15, eq193_e2427_d_b16, eq193_e2427_d_b17, eq193_e2427_d_b18, eq193_e2427_d_b19, eq193_e2427_d_b20, eq193_e2427_d_b21, eq193_e2427_d_b22, eq193_e2427_d_b23, eq193_e2427_d_b24, eq193_e2427_d_b25, eq193_e2427_d_b26, eq193_e2427_d_b27, eq193_e2427_d_b28, eq193_e2427_d_b29, eq193_e2427_d_b30, eq193_e2427_d_b31, eq193_e2427_d_b32, eq193_e2427_d_b33, eq193_e2427_d_b34, eq193_e2427_d_b35, eq193_e2427_d_b36, eq193_e2427_d_b37, eq193_e2427_d_b38, eq193_e2427_d_b39, eq193_e2427_d_b40, eq193_e2427_d_b41, eq193_e2427_d_b42, eq193_e2427_d_b43, eq193_e2427_d_b44, eq193_e2427_d_b45, eq193_e2427_d_b46, eq193_e2427_d_b47, eq193_e2427_d_b48, eq193_e2427_d_b49, eq193_e2427_d_b50, eq193_e2427_d_b51, eq193_e2427_d_b52, eq193_e2427_d_b53, eq193_e2427_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(17),
            multiplicity * (eq193_value),
            &eq193_node_derivatives,
            &eq193_branch_derivatives,
            multiplicity,
        );
        let (eq194_e2440, eq194_e2440_d_n0, eq194_e2440_d_n1, eq194_e2440_d_n2, eq194_e2440_d_n3, eq194_e2440_d_n4, eq194_e2440_d_n5, eq194_e2440_d_n6, eq194_e2440_d_n7, eq194_e2440_d_n8, eq194_e2440_d_n9, eq194_e2440_d_n10, eq194_e2440_d_n11, eq194_e2440_d_n12, eq194_e2440_d_n13, eq194_e2440_d_n14, eq194_e2440_d_n15, eq194_e2440_d_n16, eq194_e2440_d_n17, eq194_e2440_d_n18, eq194_e2440_d_n19, eq194_e2440_d_n20, eq194_e2440_d_n21, eq194_e2440_d_n22, eq194_e2440_d_b0, eq194_e2440_d_b1, eq194_e2440_d_b2, eq194_e2440_d_b3, eq194_e2440_d_b4, eq194_e2440_d_b5, eq194_e2440_d_b6, eq194_e2440_d_b7, eq194_e2440_d_b8, eq194_e2440_d_b9, eq194_e2440_d_b10, eq194_e2440_d_b11, eq194_e2440_d_b12, eq194_e2440_d_b13, eq194_e2440_d_b14, eq194_e2440_d_b15, eq194_e2440_d_b16, eq194_e2440_d_b17, eq194_e2440_d_b18, eq194_e2440_d_b19, eq194_e2440_d_b20, eq194_e2440_d_b21, eq194_e2440_d_b22, eq194_e2440_d_b23, eq194_e2440_d_b24, eq194_e2440_d_b25, eq194_e2440_d_b26, eq194_e2440_d_b27, eq194_e2440_d_b28, eq194_e2440_d_b29, eq194_e2440_d_b30, eq194_e2440_d_b31, eq194_e2440_d_b32, eq194_e2440_d_b33, eq194_e2440_d_b34, eq194_e2440_d_b35, eq194_e2440_d_b36, eq194_e2440_d_b37, eq194_e2440_d_b38, eq194_e2440_d_b39, eq194_e2440_d_b40, eq194_e2440_d_b41, eq194_e2440_d_b42, eq194_e2440_d_b43, eq194_e2440_d_b44, eq194_e2440_d_b45, eq194_e2440_d_b46, eq194_e2440_d_b47, eq194_e2440_d_b48, eq194_e2440_d_b49, eq194_e2440_d_b50, eq194_e2440_d_b51, eq194_e2440_d_b52, eq194_e2440_d_b53, eq194_e2440_d_b54,) = {
    if ((s.b[600] && s.b[601]) && s.b[602]) {
        let eq194_e2435: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 93, s.v[300]);
        let eq194_e2436: f64 = (p.p7 * eq194_e2435);
        let eq194_e2438: f64 = (eq194_e2436 * p.p249);
        let eq194_e2438_d_n0: f64 = (__rspice_deriv_cse_0 * p.p249);
        let eq194_e2438_d_n1: f64 = (__rspice_deriv_cse_1 * p.p249);
        let eq194_e2438_d_n2: f64 = (__rspice_deriv_cse_2 * p.p249);
        let eq194_e2438_d_n3: f64 = (__rspice_deriv_cse_3 * p.p249);
        let eq194_e2438_d_n4: f64 = (__rspice_deriv_cse_4 * p.p249);
        let eq194_e2438_d_n5: f64 = (__rspice_deriv_cse_5 * p.p249);
        let eq194_e2438_d_n6: f64 = (__rspice_deriv_cse_6 * p.p249);
        let eq194_e2438_d_n7: f64 = (__rspice_deriv_cse_7 * p.p249);
        let eq194_e2438_d_n8: f64 = (__rspice_deriv_cse_8 * p.p249);
        let eq194_e2438_d_n9: f64 = (__rspice_deriv_cse_9 * p.p249);
        let eq194_e2438_d_n10: f64 = (__rspice_deriv_cse_10 * p.p249);
        let eq194_e2438_d_n11: f64 = (__rspice_deriv_cse_11 * p.p249);
        let eq194_e2438_d_n12: f64 = (__rspice_deriv_cse_12 * p.p249);
        let eq194_e2438_d_n13: f64 = (__rspice_deriv_cse_13 * p.p249);
        let eq194_e2438_d_n14: f64 = (__rspice_deriv_cse_14 * p.p249);
        let eq194_e2438_d_n15: f64 = (__rspice_deriv_cse_15 * p.p249);
        let eq194_e2438_d_n16: f64 = (__rspice_deriv_cse_16 * p.p249);
        let eq194_e2438_d_n17: f64 = (__rspice_deriv_cse_17 * p.p249);
        let eq194_e2438_d_n18: f64 = (__rspice_deriv_cse_18 * p.p249);
        let eq194_e2438_d_n19: f64 = (__rspice_deriv_cse_19 * p.p249);
        let eq194_e2438_d_n20: f64 = (__rspice_deriv_cse_20 * p.p249);
        let eq194_e2438_d_n21: f64 = (__rspice_deriv_cse_21 * p.p249);
        let eq194_e2438_d_n22: f64 = (__rspice_deriv_cse_22 * p.p249);
        let eq194_e2438_d_b0: f64 = (__rspice_deriv_cse_23 * p.p249);
        let eq194_e2438_d_b1: f64 = (__rspice_deriv_cse_24 * p.p249);
        let eq194_e2438_d_b2: f64 = (__rspice_deriv_cse_25 * p.p249);
        let eq194_e2438_d_b3: f64 = (__rspice_deriv_cse_26 * p.p249);
        let eq194_e2438_d_b4: f64 = (__rspice_deriv_cse_27 * p.p249);
        let eq194_e2438_d_b5: f64 = (__rspice_deriv_cse_28 * p.p249);
        let eq194_e2438_d_b6: f64 = (__rspice_deriv_cse_29 * p.p249);
        let eq194_e2438_d_b7: f64 = (__rspice_deriv_cse_30 * p.p249);
        let eq194_e2438_d_b8: f64 = (__rspice_deriv_cse_31 * p.p249);
        let eq194_e2438_d_b9: f64 = (__rspice_deriv_cse_32 * p.p249);
        let eq194_e2438_d_b10: f64 = (__rspice_deriv_cse_33 * p.p249);
        let eq194_e2438_d_b11: f64 = (__rspice_deriv_cse_34 * p.p249);
        let eq194_e2438_d_b12: f64 = (__rspice_deriv_cse_35 * p.p249);
        let eq194_e2438_d_b13: f64 = (__rspice_deriv_cse_36 * p.p249);
        let eq194_e2438_d_b14: f64 = (__rspice_deriv_cse_37 * p.p249);
        let eq194_e2438_d_b15: f64 = (__rspice_deriv_cse_38 * p.p249);
        let eq194_e2438_d_b16: f64 = (__rspice_deriv_cse_39 * p.p249);
        let eq194_e2438_d_b17: f64 = (__rspice_deriv_cse_40 * p.p249);
        let eq194_e2438_d_b18: f64 = (__rspice_deriv_cse_41 * p.p249);
        let eq194_e2438_d_b19: f64 = (__rspice_deriv_cse_42 * p.p249);
        let eq194_e2438_d_b20: f64 = (__rspice_deriv_cse_43 * p.p249);
        let eq194_e2438_d_b21: f64 = (__rspice_deriv_cse_44 * p.p249);
        let eq194_e2438_d_b22: f64 = (__rspice_deriv_cse_45 * p.p249);
        let eq194_e2438_d_b23: f64 = (__rspice_deriv_cse_46 * p.p249);
        let eq194_e2438_d_b24: f64 = (__rspice_deriv_cse_47 * p.p249);
        let eq194_e2438_d_b25: f64 = (__rspice_deriv_cse_48 * p.p249);
        let eq194_e2438_d_b26: f64 = (__rspice_deriv_cse_49 * p.p249);
        let eq194_e2438_d_b27: f64 = (__rspice_deriv_cse_50 * p.p249);
        let eq194_e2438_d_b28: f64 = (__rspice_deriv_cse_51 * p.p249);
        let eq194_e2438_d_b29: f64 = (__rspice_deriv_cse_52 * p.p249);
        let eq194_e2438_d_b30: f64 = (__rspice_deriv_cse_53 * p.p249);
        let eq194_e2438_d_b31: f64 = (__rspice_deriv_cse_54 * p.p249);
        let eq194_e2438_d_b32: f64 = (__rspice_deriv_cse_55 * p.p249);
        let eq194_e2438_d_b33: f64 = (__rspice_deriv_cse_56 * p.p249);
        let eq194_e2438_d_b34: f64 = (__rspice_deriv_cse_57 * p.p249);
        let eq194_e2438_d_b35: f64 = (__rspice_deriv_cse_58 * p.p249);
        let eq194_e2438_d_b36: f64 = (__rspice_deriv_cse_59 * p.p249);
        let eq194_e2438_d_b37: f64 = (__rspice_deriv_cse_60 * p.p249);
        let eq194_e2438_d_b38: f64 = (__rspice_deriv_cse_61 * p.p249);
        let eq194_e2438_d_b39: f64 = (__rspice_deriv_cse_62 * p.p249);
        let eq194_e2438_d_b40: f64 = (__rspice_deriv_cse_63 * p.p249);
        let eq194_e2438_d_b41: f64 = (__rspice_deriv_cse_64 * p.p249);
        let eq194_e2438_d_b42: f64 = (__rspice_deriv_cse_65 * p.p249);
        let eq194_e2438_d_b43: f64 = (__rspice_deriv_cse_66 * p.p249);
        let eq194_e2438_d_b44: f64 = (__rspice_deriv_cse_67 * p.p249);
        let eq194_e2438_d_b45: f64 = (__rspice_deriv_cse_68 * p.p249);
        let eq194_e2438_d_b46: f64 = (__rspice_deriv_cse_69 * p.p249);
        let eq194_e2438_d_b47: f64 = (__rspice_deriv_cse_70 * p.p249);
        let eq194_e2438_d_b48: f64 = (__rspice_deriv_cse_71 * p.p249);
        let eq194_e2438_d_b49: f64 = (__rspice_deriv_cse_72 * p.p249);
        let eq194_e2438_d_b50: f64 = (__rspice_deriv_cse_73 * p.p249);
        let eq194_e2438_d_b51: f64 = (__rspice_deriv_cse_74 * p.p249);
        let eq194_e2438_d_b52: f64 = (__rspice_deriv_cse_75 * p.p249);
        let eq194_e2438_d_b53: f64 = (__rspice_deriv_cse_76 * p.p249);
        let eq194_e2438_d_b54: f64 = (__rspice_deriv_cse_77 * p.p249);
        (eq194_e2438, eq194_e2438_d_n0, eq194_e2438_d_n1, eq194_e2438_d_n2, eq194_e2438_d_n3, eq194_e2438_d_n4, eq194_e2438_d_n5, eq194_e2438_d_n6, eq194_e2438_d_n7, eq194_e2438_d_n8, eq194_e2438_d_n9, eq194_e2438_d_n10, eq194_e2438_d_n11, eq194_e2438_d_n12, eq194_e2438_d_n13, eq194_e2438_d_n14, eq194_e2438_d_n15, eq194_e2438_d_n16, eq194_e2438_d_n17, eq194_e2438_d_n18, eq194_e2438_d_n19, eq194_e2438_d_n20, eq194_e2438_d_n21, eq194_e2438_d_n22, eq194_e2438_d_b0, eq194_e2438_d_b1, eq194_e2438_d_b2, eq194_e2438_d_b3, eq194_e2438_d_b4, eq194_e2438_d_b5, eq194_e2438_d_b6, eq194_e2438_d_b7, eq194_e2438_d_b8, eq194_e2438_d_b9, eq194_e2438_d_b10, eq194_e2438_d_b11, eq194_e2438_d_b12, eq194_e2438_d_b13, eq194_e2438_d_b14, eq194_e2438_d_b15, eq194_e2438_d_b16, eq194_e2438_d_b17, eq194_e2438_d_b18, eq194_e2438_d_b19, eq194_e2438_d_b20, eq194_e2438_d_b21, eq194_e2438_d_b22, eq194_e2438_d_b23, eq194_e2438_d_b24, eq194_e2438_d_b25, eq194_e2438_d_b26, eq194_e2438_d_b27, eq194_e2438_d_b28, eq194_e2438_d_b29, eq194_e2438_d_b30, eq194_e2438_d_b31, eq194_e2438_d_b32, eq194_e2438_d_b33, eq194_e2438_d_b34, eq194_e2438_d_b35, eq194_e2438_d_b36, eq194_e2438_d_b37, eq194_e2438_d_b38, eq194_e2438_d_b39, eq194_e2438_d_b40, eq194_e2438_d_b41, eq194_e2438_d_b42, eq194_e2438_d_b43, eq194_e2438_d_b44, eq194_e2438_d_b45, eq194_e2438_d_b46, eq194_e2438_d_b47, eq194_e2438_d_b48, eq194_e2438_d_b49, eq194_e2438_d_b50, eq194_e2438_d_b51, eq194_e2438_d_b52, eq194_e2438_d_b53, eq194_e2438_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq194_value: f64 = eq194_e2440;
        let eq194_node_derivatives: [f64; 23] = [eq194_e2440_d_n0, eq194_e2440_d_n1, eq194_e2440_d_n2, eq194_e2440_d_n3, eq194_e2440_d_n4, eq194_e2440_d_n5, eq194_e2440_d_n6, eq194_e2440_d_n7, eq194_e2440_d_n8, eq194_e2440_d_n9, eq194_e2440_d_n10, eq194_e2440_d_n11, eq194_e2440_d_n12, eq194_e2440_d_n13, eq194_e2440_d_n14, eq194_e2440_d_n15, eq194_e2440_d_n16, eq194_e2440_d_n17, eq194_e2440_d_n18, eq194_e2440_d_n19, eq194_e2440_d_n20, eq194_e2440_d_n21, eq194_e2440_d_n22];
        let eq194_branch_derivatives: [f64; 55] = [eq194_e2440_d_b0, eq194_e2440_d_b1, eq194_e2440_d_b2, eq194_e2440_d_b3, eq194_e2440_d_b4, eq194_e2440_d_b5, eq194_e2440_d_b6, eq194_e2440_d_b7, eq194_e2440_d_b8, eq194_e2440_d_b9, eq194_e2440_d_b10, eq194_e2440_d_b11, eq194_e2440_d_b12, eq194_e2440_d_b13, eq194_e2440_d_b14, eq194_e2440_d_b15, eq194_e2440_d_b16, eq194_e2440_d_b17, eq194_e2440_d_b18, eq194_e2440_d_b19, eq194_e2440_d_b20, eq194_e2440_d_b21, eq194_e2440_d_b22, eq194_e2440_d_b23, eq194_e2440_d_b24, eq194_e2440_d_b25, eq194_e2440_d_b26, eq194_e2440_d_b27, eq194_e2440_d_b28, eq194_e2440_d_b29, eq194_e2440_d_b30, eq194_e2440_d_b31, eq194_e2440_d_b32, eq194_e2440_d_b33, eq194_e2440_d_b34, eq194_e2440_d_b35, eq194_e2440_d_b36, eq194_e2440_d_b37, eq194_e2440_d_b38, eq194_e2440_d_b39, eq194_e2440_d_b40, eq194_e2440_d_b41, eq194_e2440_d_b42, eq194_e2440_d_b43, eq194_e2440_d_b44, eq194_e2440_d_b45, eq194_e2440_d_b46, eq194_e2440_d_b47, eq194_e2440_d_b48, eq194_e2440_d_b49, eq194_e2440_d_b50, eq194_e2440_d_b51, eq194_e2440_d_b52, eq194_e2440_d_b53, eq194_e2440_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(17),
            multiplicity * (eq194_value),
            &eq194_node_derivatives,
            &eq194_branch_derivatives,
            multiplicity,
        );
        let (eq195_e2452, eq195_e2452_d_n0, eq195_e2452_d_n1, eq195_e2452_d_n2, eq195_e2452_d_n3, eq195_e2452_d_n4, eq195_e2452_d_n5, eq195_e2452_d_n6, eq195_e2452_d_n7, eq195_e2452_d_n8, eq195_e2452_d_n9, eq195_e2452_d_n10, eq195_e2452_d_n11, eq195_e2452_d_n12, eq195_e2452_d_n13, eq195_e2452_d_n14, eq195_e2452_d_n15, eq195_e2452_d_n16, eq195_e2452_d_n17, eq195_e2452_d_n18, eq195_e2452_d_n19, eq195_e2452_d_n20, eq195_e2452_d_n21, eq195_e2452_d_n22, eq195_e2452_d_b0, eq195_e2452_d_b1, eq195_e2452_d_b2, eq195_e2452_d_b3, eq195_e2452_d_b4, eq195_e2452_d_b5, eq195_e2452_d_b6, eq195_e2452_d_b7, eq195_e2452_d_b8, eq195_e2452_d_b9, eq195_e2452_d_b10, eq195_e2452_d_b11, eq195_e2452_d_b12, eq195_e2452_d_b13, eq195_e2452_d_b14, eq195_e2452_d_b15, eq195_e2452_d_b16, eq195_e2452_d_b17, eq195_e2452_d_b18, eq195_e2452_d_b19, eq195_e2452_d_b20, eq195_e2452_d_b21, eq195_e2452_d_b22, eq195_e2452_d_b23, eq195_e2452_d_b24, eq195_e2452_d_b25, eq195_e2452_d_b26, eq195_e2452_d_b27, eq195_e2452_d_b28, eq195_e2452_d_b29, eq195_e2452_d_b30, eq195_e2452_d_b31, eq195_e2452_d_b32, eq195_e2452_d_b33, eq195_e2452_d_b34, eq195_e2452_d_b35, eq195_e2452_d_b36, eq195_e2452_d_b37, eq195_e2452_d_b38, eq195_e2452_d_b39, eq195_e2452_d_b40, eq195_e2452_d_b41, eq195_e2452_d_b42, eq195_e2452_d_b43, eq195_e2452_d_b44, eq195_e2452_d_b45, eq195_e2452_d_b46, eq195_e2452_d_b47, eq195_e2452_d_b48, eq195_e2452_d_b49, eq195_e2452_d_b50, eq195_e2452_d_b51, eq195_e2452_d_b52, eq195_e2452_d_b53, eq195_e2452_d_b54,) = {
    if ((s.b[600] && s.b[601]) && (!s.b[602])) {
        let eq195_e2449: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 94, s.v[300]);
        let eq195_e2450: f64 = (p.p7 * eq195_e2449);
        (eq195_e2450, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq195_value: f64 = eq195_e2452;
        let eq195_node_derivatives: [f64; 23] = [eq195_e2452_d_n0, eq195_e2452_d_n1, eq195_e2452_d_n2, eq195_e2452_d_n3, eq195_e2452_d_n4, eq195_e2452_d_n5, eq195_e2452_d_n6, eq195_e2452_d_n7, eq195_e2452_d_n8, eq195_e2452_d_n9, eq195_e2452_d_n10, eq195_e2452_d_n11, eq195_e2452_d_n12, eq195_e2452_d_n13, eq195_e2452_d_n14, eq195_e2452_d_n15, eq195_e2452_d_n16, eq195_e2452_d_n17, eq195_e2452_d_n18, eq195_e2452_d_n19, eq195_e2452_d_n20, eq195_e2452_d_n21, eq195_e2452_d_n22];
        let eq195_branch_derivatives: [f64; 55] = [eq195_e2452_d_b0, eq195_e2452_d_b1, eq195_e2452_d_b2, eq195_e2452_d_b3, eq195_e2452_d_b4, eq195_e2452_d_b5, eq195_e2452_d_b6, eq195_e2452_d_b7, eq195_e2452_d_b8, eq195_e2452_d_b9, eq195_e2452_d_b10, eq195_e2452_d_b11, eq195_e2452_d_b12, eq195_e2452_d_b13, eq195_e2452_d_b14, eq195_e2452_d_b15, eq195_e2452_d_b16, eq195_e2452_d_b17, eq195_e2452_d_b18, eq195_e2452_d_b19, eq195_e2452_d_b20, eq195_e2452_d_b21, eq195_e2452_d_b22, eq195_e2452_d_b23, eq195_e2452_d_b24, eq195_e2452_d_b25, eq195_e2452_d_b26, eq195_e2452_d_b27, eq195_e2452_d_b28, eq195_e2452_d_b29, eq195_e2452_d_b30, eq195_e2452_d_b31, eq195_e2452_d_b32, eq195_e2452_d_b33, eq195_e2452_d_b34, eq195_e2452_d_b35, eq195_e2452_d_b36, eq195_e2452_d_b37, eq195_e2452_d_b38, eq195_e2452_d_b39, eq195_e2452_d_b40, eq195_e2452_d_b41, eq195_e2452_d_b42, eq195_e2452_d_b43, eq195_e2452_d_b44, eq195_e2452_d_b45, eq195_e2452_d_b46, eq195_e2452_d_b47, eq195_e2452_d_b48, eq195_e2452_d_b49, eq195_e2452_d_b50, eq195_e2452_d_b51, eq195_e2452_d_b52, eq195_e2452_d_b53, eq195_e2452_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(17),
            multiplicity * (eq195_value),
            &eq195_node_derivatives,
            &eq195_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_40(
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
        let (eq196_e2466, eq196_e2466_d_n0, eq196_e2466_d_n1, eq196_e2466_d_n2, eq196_e2466_d_n3, eq196_e2466_d_n4, eq196_e2466_d_n5, eq196_e2466_d_n6, eq196_e2466_d_n7, eq196_e2466_d_n8, eq196_e2466_d_n9, eq196_e2466_d_n10, eq196_e2466_d_n11, eq196_e2466_d_n12, eq196_e2466_d_n13, eq196_e2466_d_n14, eq196_e2466_d_n15, eq196_e2466_d_n16, eq196_e2466_d_n17, eq196_e2466_d_n18, eq196_e2466_d_n19, eq196_e2466_d_n20, eq196_e2466_d_n21, eq196_e2466_d_n22, eq196_e2466_d_b0, eq196_e2466_d_b1, eq196_e2466_d_b2, eq196_e2466_d_b3, eq196_e2466_d_b4, eq196_e2466_d_b5, eq196_e2466_d_b6, eq196_e2466_d_b7, eq196_e2466_d_b8, eq196_e2466_d_b9, eq196_e2466_d_b10, eq196_e2466_d_b11, eq196_e2466_d_b12, eq196_e2466_d_b13, eq196_e2466_d_b14, eq196_e2466_d_b15, eq196_e2466_d_b16, eq196_e2466_d_b17, eq196_e2466_d_b18, eq196_e2466_d_b19, eq196_e2466_d_b20, eq196_e2466_d_b21, eq196_e2466_d_b22, eq196_e2466_d_b23, eq196_e2466_d_b24, eq196_e2466_d_b25, eq196_e2466_d_b26, eq196_e2466_d_b27, eq196_e2466_d_b28, eq196_e2466_d_b29, eq196_e2466_d_b30, eq196_e2466_d_b31, eq196_e2466_d_b32, eq196_e2466_d_b33, eq196_e2466_d_b34, eq196_e2466_d_b35, eq196_e2466_d_b36, eq196_e2466_d_b37, eq196_e2466_d_b38, eq196_e2466_d_b39, eq196_e2466_d_b40, eq196_e2466_d_b41, eq196_e2466_d_b42, eq196_e2466_d_b43, eq196_e2466_d_b44, eq196_e2466_d_b45, eq196_e2466_d_b46, eq196_e2466_d_b47, eq196_e2466_d_b48, eq196_e2466_d_b49, eq196_e2466_d_b50, eq196_e2466_d_b51, eq196_e2466_d_b52, eq196_e2466_d_b53, eq196_e2466_d_b54,) = {
    if ((s.b[600] && s.b[601]) && (!s.b[602])) {
        let eq196_e2461: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 95, s.v[300]);
        let eq196_e2462: f64 = (p.p7 * eq196_e2461);
        let eq196_e2462_d_n0: f64 = (p.p7 * (s.dn[300][0] * ddt_scale));
        let eq196_e2462_d_n1: f64 = (p.p7 * (s.dn[300][1] * ddt_scale));
        let eq196_e2462_d_n2: f64 = (p.p7 * (s.dn[300][2] * ddt_scale));
        let eq196_e2462_d_n3: f64 = (p.p7 * (s.dn[300][3] * ddt_scale));
        let eq196_e2462_d_n4: f64 = (p.p7 * (s.dn[300][4] * ddt_scale));
        let eq196_e2462_d_n5: f64 = (p.p7 * (s.dn[300][5] * ddt_scale));
        let eq196_e2462_d_n6: f64 = (p.p7 * (s.dn[300][6] * ddt_scale));
        let eq196_e2462_d_n7: f64 = (p.p7 * (s.dn[300][7] * ddt_scale));
        let eq196_e2462_d_n8: f64 = (p.p7 * (s.dn[300][8] * ddt_scale));
        let eq196_e2462_d_n9: f64 = (p.p7 * (s.dn[300][9] * ddt_scale));
        let eq196_e2462_d_n10: f64 = (p.p7 * (s.dn[300][10] * ddt_scale));
        let eq196_e2462_d_n11: f64 = (p.p7 * (s.dn[300][11] * ddt_scale));
        let eq196_e2462_d_n12: f64 = (p.p7 * (s.dn[300][12] * ddt_scale));
        let eq196_e2462_d_n13: f64 = (p.p7 * (s.dn[300][13] * ddt_scale));
        let eq196_e2462_d_n14: f64 = (p.p7 * (s.dn[300][14] * ddt_scale));
        let eq196_e2462_d_n15: f64 = (p.p7 * (s.dn[300][15] * ddt_scale));
        let eq196_e2462_d_n16: f64 = (p.p7 * (s.dn[300][16] * ddt_scale));
        let eq196_e2462_d_n17: f64 = (p.p7 * (s.dn[300][17] * ddt_scale));
        let eq196_e2462_d_n18: f64 = (p.p7 * (s.dn[300][18] * ddt_scale));
        let eq196_e2462_d_n19: f64 = (p.p7 * (s.dn[300][19] * ddt_scale));
        let eq196_e2462_d_n20: f64 = (p.p7 * (s.dn[300][20] * ddt_scale));
        let eq196_e2462_d_n21: f64 = (p.p7 * (s.dn[300][21] * ddt_scale));
        let eq196_e2462_d_n22: f64 = (p.p7 * (s.dn[300][22] * ddt_scale));
        let eq196_e2462_d_b0: f64 = (p.p7 * (s.db[300][0] * ddt_scale));
        let eq196_e2462_d_b1: f64 = (p.p7 * (s.db[300][1] * ddt_scale));
        let eq196_e2462_d_b2: f64 = (p.p7 * (s.db[300][2] * ddt_scale));
        let eq196_e2462_d_b3: f64 = (p.p7 * (s.db[300][3] * ddt_scale));
        let eq196_e2462_d_b4: f64 = (p.p7 * (s.db[300][4] * ddt_scale));
        let eq196_e2462_d_b5: f64 = (p.p7 * (s.db[300][5] * ddt_scale));
        let eq196_e2462_d_b6: f64 = (p.p7 * (s.db[300][6] * ddt_scale));
        let eq196_e2462_d_b7: f64 = (p.p7 * (s.db[300][7] * ddt_scale));
        let eq196_e2462_d_b8: f64 = (p.p7 * (s.db[300][8] * ddt_scale));
        let eq196_e2462_d_b9: f64 = (p.p7 * (s.db[300][9] * ddt_scale));
        let eq196_e2462_d_b10: f64 = (p.p7 * (s.db[300][10] * ddt_scale));
        let eq196_e2462_d_b11: f64 = (p.p7 * (s.db[300][11] * ddt_scale));
        let eq196_e2462_d_b12: f64 = (p.p7 * (s.db[300][12] * ddt_scale));
        let eq196_e2462_d_b13: f64 = (p.p7 * (s.db[300][13] * ddt_scale));
        let eq196_e2462_d_b14: f64 = (p.p7 * (s.db[300][14] * ddt_scale));
        let eq196_e2462_d_b15: f64 = (p.p7 * (s.db[300][15] * ddt_scale));
        let eq196_e2462_d_b16: f64 = (p.p7 * (s.db[300][16] * ddt_scale));
        let eq196_e2462_d_b17: f64 = (p.p7 * (s.db[300][17] * ddt_scale));
        let eq196_e2462_d_b18: f64 = (p.p7 * (s.db[300][18] * ddt_scale));
        let eq196_e2462_d_b19: f64 = (p.p7 * (s.db[300][19] * ddt_scale));
        let eq196_e2462_d_b20: f64 = (p.p7 * (s.db[300][20] * ddt_scale));
        let eq196_e2462_d_b21: f64 = (p.p7 * (s.db[300][21] * ddt_scale));
        let eq196_e2462_d_b22: f64 = (p.p7 * (s.db[300][22] * ddt_scale));
        let eq196_e2462_d_b23: f64 = (p.p7 * (s.db[300][23] * ddt_scale));
        let eq196_e2462_d_b24: f64 = (p.p7 * (s.db[300][24] * ddt_scale));
        let eq196_e2462_d_b25: f64 = (p.p7 * (s.db[300][25] * ddt_scale));
        let eq196_e2462_d_b26: f64 = (p.p7 * (s.db[300][26] * ddt_scale));
        let eq196_e2462_d_b27: f64 = (p.p7 * (s.db[300][27] * ddt_scale));
        let eq196_e2462_d_b28: f64 = (p.p7 * (s.db[300][28] * ddt_scale));
        let eq196_e2462_d_b29: f64 = (p.p7 * (s.db[300][29] * ddt_scale));
        let eq196_e2462_d_b30: f64 = (p.p7 * (s.db[300][30] * ddt_scale));
        let eq196_e2462_d_b31: f64 = (p.p7 * (s.db[300][31] * ddt_scale));
        let eq196_e2462_d_b32: f64 = (p.p7 * (s.db[300][32] * ddt_scale));
        let eq196_e2462_d_b33: f64 = (p.p7 * (s.db[300][33] * ddt_scale));
        let eq196_e2462_d_b34: f64 = (p.p7 * (s.db[300][34] * ddt_scale));
        let eq196_e2462_d_b35: f64 = (p.p7 * (s.db[300][35] * ddt_scale));
        let eq196_e2462_d_b36: f64 = (p.p7 * (s.db[300][36] * ddt_scale));
        let eq196_e2462_d_b37: f64 = (p.p7 * (s.db[300][37] * ddt_scale));
        let eq196_e2462_d_b38: f64 = (p.p7 * (s.db[300][38] * ddt_scale));
        let eq196_e2462_d_b39: f64 = (p.p7 * (s.db[300][39] * ddt_scale));
        let eq196_e2462_d_b40: f64 = (p.p7 * (s.db[300][40] * ddt_scale));
        let eq196_e2462_d_b41: f64 = (p.p7 * (s.db[300][41] * ddt_scale));
        let eq196_e2462_d_b42: f64 = (p.p7 * (s.db[300][42] * ddt_scale));
        let eq196_e2462_d_b43: f64 = (p.p7 * (s.db[300][43] * ddt_scale));
        let eq196_e2462_d_b44: f64 = (p.p7 * (s.db[300][44] * ddt_scale));
        let eq196_e2462_d_b45: f64 = (p.p7 * (s.db[300][45] * ddt_scale));
        let eq196_e2462_d_b46: f64 = (p.p7 * (s.db[300][46] * ddt_scale));
        let eq196_e2462_d_b47: f64 = (p.p7 * (s.db[300][47] * ddt_scale));
        let eq196_e2462_d_b48: f64 = (p.p7 * (s.db[300][48] * ddt_scale));
        let eq196_e2462_d_b49: f64 = (p.p7 * (s.db[300][49] * ddt_scale));
        let eq196_e2462_d_b50: f64 = (p.p7 * (s.db[300][50] * ddt_scale));
        let eq196_e2462_d_b51: f64 = (p.p7 * (s.db[300][51] * ddt_scale));
        let eq196_e2462_d_b52: f64 = (p.p7 * (s.db[300][52] * ddt_scale));
        let eq196_e2462_d_b53: f64 = (p.p7 * (s.db[300][53] * ddt_scale));
        let eq196_e2462_d_b54: f64 = (p.p7 * (s.db[300][54] * ddt_scale));
        let eq196_e2464: f64 = (eq196_e2462 * p.p249);
        let eq196_e2464_d_n0: f64 = (eq196_e2462_d_n0 * p.p249);
        let eq196_e2464_d_n1: f64 = (eq196_e2462_d_n1 * p.p249);
        let eq196_e2464_d_n2: f64 = (eq196_e2462_d_n2 * p.p249);
        let eq196_e2464_d_n3: f64 = (eq196_e2462_d_n3 * p.p249);
        let eq196_e2464_d_n4: f64 = (eq196_e2462_d_n4 * p.p249);
        let eq196_e2464_d_n5: f64 = (eq196_e2462_d_n5 * p.p249);
        let eq196_e2464_d_n6: f64 = (eq196_e2462_d_n6 * p.p249);
        let eq196_e2464_d_n7: f64 = (eq196_e2462_d_n7 * p.p249);
        let eq196_e2464_d_n8: f64 = (eq196_e2462_d_n8 * p.p249);
        let eq196_e2464_d_n9: f64 = (eq196_e2462_d_n9 * p.p249);
        let eq196_e2464_d_n10: f64 = (eq196_e2462_d_n10 * p.p249);
        let eq196_e2464_d_n11: f64 = (eq196_e2462_d_n11 * p.p249);
        let eq196_e2464_d_n12: f64 = (eq196_e2462_d_n12 * p.p249);
        let eq196_e2464_d_n13: f64 = (eq196_e2462_d_n13 * p.p249);
        let eq196_e2464_d_n14: f64 = (eq196_e2462_d_n14 * p.p249);
        let eq196_e2464_d_n15: f64 = (eq196_e2462_d_n15 * p.p249);
        let eq196_e2464_d_n16: f64 = (eq196_e2462_d_n16 * p.p249);
        let eq196_e2464_d_n17: f64 = (eq196_e2462_d_n17 * p.p249);
        let eq196_e2464_d_n18: f64 = (eq196_e2462_d_n18 * p.p249);
        let eq196_e2464_d_n19: f64 = (eq196_e2462_d_n19 * p.p249);
        let eq196_e2464_d_n20: f64 = (eq196_e2462_d_n20 * p.p249);
        let eq196_e2464_d_n21: f64 = (eq196_e2462_d_n21 * p.p249);
        let eq196_e2464_d_n22: f64 = (eq196_e2462_d_n22 * p.p249);
        let eq196_e2464_d_b0: f64 = (eq196_e2462_d_b0 * p.p249);
        let eq196_e2464_d_b1: f64 = (eq196_e2462_d_b1 * p.p249);
        let eq196_e2464_d_b2: f64 = (eq196_e2462_d_b2 * p.p249);
        let eq196_e2464_d_b3: f64 = (eq196_e2462_d_b3 * p.p249);
        let eq196_e2464_d_b4: f64 = (eq196_e2462_d_b4 * p.p249);
        let eq196_e2464_d_b5: f64 = (eq196_e2462_d_b5 * p.p249);
        let eq196_e2464_d_b6: f64 = (eq196_e2462_d_b6 * p.p249);
        let eq196_e2464_d_b7: f64 = (eq196_e2462_d_b7 * p.p249);
        let eq196_e2464_d_b8: f64 = (eq196_e2462_d_b8 * p.p249);
        let eq196_e2464_d_b9: f64 = (eq196_e2462_d_b9 * p.p249);
        let eq196_e2464_d_b10: f64 = (eq196_e2462_d_b10 * p.p249);
        let eq196_e2464_d_b11: f64 = (eq196_e2462_d_b11 * p.p249);
        let eq196_e2464_d_b12: f64 = (eq196_e2462_d_b12 * p.p249);
        let eq196_e2464_d_b13: f64 = (eq196_e2462_d_b13 * p.p249);
        let eq196_e2464_d_b14: f64 = (eq196_e2462_d_b14 * p.p249);
        let eq196_e2464_d_b15: f64 = (eq196_e2462_d_b15 * p.p249);
        let eq196_e2464_d_b16: f64 = (eq196_e2462_d_b16 * p.p249);
        let eq196_e2464_d_b17: f64 = (eq196_e2462_d_b17 * p.p249);
        let eq196_e2464_d_b18: f64 = (eq196_e2462_d_b18 * p.p249);
        let eq196_e2464_d_b19: f64 = (eq196_e2462_d_b19 * p.p249);
        let eq196_e2464_d_b20: f64 = (eq196_e2462_d_b20 * p.p249);
        let eq196_e2464_d_b21: f64 = (eq196_e2462_d_b21 * p.p249);
        let eq196_e2464_d_b22: f64 = (eq196_e2462_d_b22 * p.p249);
        let eq196_e2464_d_b23: f64 = (eq196_e2462_d_b23 * p.p249);
        let eq196_e2464_d_b24: f64 = (eq196_e2462_d_b24 * p.p249);
        let eq196_e2464_d_b25: f64 = (eq196_e2462_d_b25 * p.p249);
        let eq196_e2464_d_b26: f64 = (eq196_e2462_d_b26 * p.p249);
        let eq196_e2464_d_b27: f64 = (eq196_e2462_d_b27 * p.p249);
        let eq196_e2464_d_b28: f64 = (eq196_e2462_d_b28 * p.p249);
        let eq196_e2464_d_b29: f64 = (eq196_e2462_d_b29 * p.p249);
        let eq196_e2464_d_b30: f64 = (eq196_e2462_d_b30 * p.p249);
        let eq196_e2464_d_b31: f64 = (eq196_e2462_d_b31 * p.p249);
        let eq196_e2464_d_b32: f64 = (eq196_e2462_d_b32 * p.p249);
        let eq196_e2464_d_b33: f64 = (eq196_e2462_d_b33 * p.p249);
        let eq196_e2464_d_b34: f64 = (eq196_e2462_d_b34 * p.p249);
        let eq196_e2464_d_b35: f64 = (eq196_e2462_d_b35 * p.p249);
        let eq196_e2464_d_b36: f64 = (eq196_e2462_d_b36 * p.p249);
        let eq196_e2464_d_b37: f64 = (eq196_e2462_d_b37 * p.p249);
        let eq196_e2464_d_b38: f64 = (eq196_e2462_d_b38 * p.p249);
        let eq196_e2464_d_b39: f64 = (eq196_e2462_d_b39 * p.p249);
        let eq196_e2464_d_b40: f64 = (eq196_e2462_d_b40 * p.p249);
        let eq196_e2464_d_b41: f64 = (eq196_e2462_d_b41 * p.p249);
        let eq196_e2464_d_b42: f64 = (eq196_e2462_d_b42 * p.p249);
        let eq196_e2464_d_b43: f64 = (eq196_e2462_d_b43 * p.p249);
        let eq196_e2464_d_b44: f64 = (eq196_e2462_d_b44 * p.p249);
        let eq196_e2464_d_b45: f64 = (eq196_e2462_d_b45 * p.p249);
        let eq196_e2464_d_b46: f64 = (eq196_e2462_d_b46 * p.p249);
        let eq196_e2464_d_b47: f64 = (eq196_e2462_d_b47 * p.p249);
        let eq196_e2464_d_b48: f64 = (eq196_e2462_d_b48 * p.p249);
        let eq196_e2464_d_b49: f64 = (eq196_e2462_d_b49 * p.p249);
        let eq196_e2464_d_b50: f64 = (eq196_e2462_d_b50 * p.p249);
        let eq196_e2464_d_b51: f64 = (eq196_e2462_d_b51 * p.p249);
        let eq196_e2464_d_b52: f64 = (eq196_e2462_d_b52 * p.p249);
        let eq196_e2464_d_b53: f64 = (eq196_e2462_d_b53 * p.p249);
        let eq196_e2464_d_b54: f64 = (eq196_e2462_d_b54 * p.p249);
        (eq196_e2464, eq196_e2464_d_n0, eq196_e2464_d_n1, eq196_e2464_d_n2, eq196_e2464_d_n3, eq196_e2464_d_n4, eq196_e2464_d_n5, eq196_e2464_d_n6, eq196_e2464_d_n7, eq196_e2464_d_n8, eq196_e2464_d_n9, eq196_e2464_d_n10, eq196_e2464_d_n11, eq196_e2464_d_n12, eq196_e2464_d_n13, eq196_e2464_d_n14, eq196_e2464_d_n15, eq196_e2464_d_n16, eq196_e2464_d_n17, eq196_e2464_d_n18, eq196_e2464_d_n19, eq196_e2464_d_n20, eq196_e2464_d_n21, eq196_e2464_d_n22, eq196_e2464_d_b0, eq196_e2464_d_b1, eq196_e2464_d_b2, eq196_e2464_d_b3, eq196_e2464_d_b4, eq196_e2464_d_b5, eq196_e2464_d_b6, eq196_e2464_d_b7, eq196_e2464_d_b8, eq196_e2464_d_b9, eq196_e2464_d_b10, eq196_e2464_d_b11, eq196_e2464_d_b12, eq196_e2464_d_b13, eq196_e2464_d_b14, eq196_e2464_d_b15, eq196_e2464_d_b16, eq196_e2464_d_b17, eq196_e2464_d_b18, eq196_e2464_d_b19, eq196_e2464_d_b20, eq196_e2464_d_b21, eq196_e2464_d_b22, eq196_e2464_d_b23, eq196_e2464_d_b24, eq196_e2464_d_b25, eq196_e2464_d_b26, eq196_e2464_d_b27, eq196_e2464_d_b28, eq196_e2464_d_b29, eq196_e2464_d_b30, eq196_e2464_d_b31, eq196_e2464_d_b32, eq196_e2464_d_b33, eq196_e2464_d_b34, eq196_e2464_d_b35, eq196_e2464_d_b36, eq196_e2464_d_b37, eq196_e2464_d_b38, eq196_e2464_d_b39, eq196_e2464_d_b40, eq196_e2464_d_b41, eq196_e2464_d_b42, eq196_e2464_d_b43, eq196_e2464_d_b44, eq196_e2464_d_b45, eq196_e2464_d_b46, eq196_e2464_d_b47, eq196_e2464_d_b48, eq196_e2464_d_b49, eq196_e2464_d_b50, eq196_e2464_d_b51, eq196_e2464_d_b52, eq196_e2464_d_b53, eq196_e2464_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq196_value: f64 = eq196_e2466;
        let eq196_node_derivatives: [f64; 23] = [eq196_e2466_d_n0, eq196_e2466_d_n1, eq196_e2466_d_n2, eq196_e2466_d_n3, eq196_e2466_d_n4, eq196_e2466_d_n5, eq196_e2466_d_n6, eq196_e2466_d_n7, eq196_e2466_d_n8, eq196_e2466_d_n9, eq196_e2466_d_n10, eq196_e2466_d_n11, eq196_e2466_d_n12, eq196_e2466_d_n13, eq196_e2466_d_n14, eq196_e2466_d_n15, eq196_e2466_d_n16, eq196_e2466_d_n17, eq196_e2466_d_n18, eq196_e2466_d_n19, eq196_e2466_d_n20, eq196_e2466_d_n21, eq196_e2466_d_n22];
        let eq196_branch_derivatives: [f64; 55] = [eq196_e2466_d_b0, eq196_e2466_d_b1, eq196_e2466_d_b2, eq196_e2466_d_b3, eq196_e2466_d_b4, eq196_e2466_d_b5, eq196_e2466_d_b6, eq196_e2466_d_b7, eq196_e2466_d_b8, eq196_e2466_d_b9, eq196_e2466_d_b10, eq196_e2466_d_b11, eq196_e2466_d_b12, eq196_e2466_d_b13, eq196_e2466_d_b14, eq196_e2466_d_b15, eq196_e2466_d_b16, eq196_e2466_d_b17, eq196_e2466_d_b18, eq196_e2466_d_b19, eq196_e2466_d_b20, eq196_e2466_d_b21, eq196_e2466_d_b22, eq196_e2466_d_b23, eq196_e2466_d_b24, eq196_e2466_d_b25, eq196_e2466_d_b26, eq196_e2466_d_b27, eq196_e2466_d_b28, eq196_e2466_d_b29, eq196_e2466_d_b30, eq196_e2466_d_b31, eq196_e2466_d_b32, eq196_e2466_d_b33, eq196_e2466_d_b34, eq196_e2466_d_b35, eq196_e2466_d_b36, eq196_e2466_d_b37, eq196_e2466_d_b38, eq196_e2466_d_b39, eq196_e2466_d_b40, eq196_e2466_d_b41, eq196_e2466_d_b42, eq196_e2466_d_b43, eq196_e2466_d_b44, eq196_e2466_d_b45, eq196_e2466_d_b46, eq196_e2466_d_b47, eq196_e2466_d_b48, eq196_e2466_d_b49, eq196_e2466_d_b50, eq196_e2466_d_b51, eq196_e2466_d_b52, eq196_e2466_d_b53, eq196_e2466_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(17),
            multiplicity * (eq196_value),
            &eq196_node_derivatives,
            &eq196_branch_derivatives,
            multiplicity,
        );
        let (eq197_e2477, eq197_e2477_d_n0, eq197_e2477_d_n1, eq197_e2477_d_n2, eq197_e2477_d_n3, eq197_e2477_d_n4, eq197_e2477_d_n5, eq197_e2477_d_n6, eq197_e2477_d_n7, eq197_e2477_d_n8, eq197_e2477_d_n9, eq197_e2477_d_n10, eq197_e2477_d_n11, eq197_e2477_d_n12, eq197_e2477_d_n13, eq197_e2477_d_n14, eq197_e2477_d_n15, eq197_e2477_d_n16, eq197_e2477_d_n17, eq197_e2477_d_n18, eq197_e2477_d_n19, eq197_e2477_d_n20, eq197_e2477_d_n21, eq197_e2477_d_n22, eq197_e2477_d_b0, eq197_e2477_d_b1, eq197_e2477_d_b2, eq197_e2477_d_b3, eq197_e2477_d_b4, eq197_e2477_d_b5, eq197_e2477_d_b6, eq197_e2477_d_b7, eq197_e2477_d_b8, eq197_e2477_d_b9, eq197_e2477_d_b10, eq197_e2477_d_b11, eq197_e2477_d_b12, eq197_e2477_d_b13, eq197_e2477_d_b14, eq197_e2477_d_b15, eq197_e2477_d_b16, eq197_e2477_d_b17, eq197_e2477_d_b18, eq197_e2477_d_b19, eq197_e2477_d_b20, eq197_e2477_d_b21, eq197_e2477_d_b22, eq197_e2477_d_b23, eq197_e2477_d_b24, eq197_e2477_d_b25, eq197_e2477_d_b26, eq197_e2477_d_b27, eq197_e2477_d_b28, eq197_e2477_d_b29, eq197_e2477_d_b30, eq197_e2477_d_b31, eq197_e2477_d_b32, eq197_e2477_d_b33, eq197_e2477_d_b34, eq197_e2477_d_b35, eq197_e2477_d_b36, eq197_e2477_d_b37, eq197_e2477_d_b38, eq197_e2477_d_b39, eq197_e2477_d_b40, eq197_e2477_d_b41, eq197_e2477_d_b42, eq197_e2477_d_b43, eq197_e2477_d_b44, eq197_e2477_d_b45, eq197_e2477_d_b46, eq197_e2477_d_b47, eq197_e2477_d_b48, eq197_e2477_d_b49, eq197_e2477_d_b50, eq197_e2477_d_b51, eq197_e2477_d_b52, eq197_e2477_d_b53, eq197_e2477_d_b54,) = {
    if (s.b[600] && s.b[601]) {
        let eq197_e2473: f64 = (p.p254 * s.v[300]);
        let eq197_e2474: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 96, eq197_e2473);
        let eq197_e2474_d_n0: f64 = ((p.p254 * s.dn[300][0]) * ddt_scale);
        let eq197_e2474_d_n1: f64 = ((p.p254 * s.dn[300][1]) * ddt_scale);
        let eq197_e2474_d_n2: f64 = ((p.p254 * s.dn[300][2]) * ddt_scale);
        let eq197_e2474_d_n3: f64 = ((p.p254 * s.dn[300][3]) * ddt_scale);
        let eq197_e2474_d_n4: f64 = ((p.p254 * s.dn[300][4]) * ddt_scale);
        let eq197_e2474_d_n5: f64 = ((p.p254 * s.dn[300][5]) * ddt_scale);
        let eq197_e2474_d_n6: f64 = ((p.p254 * s.dn[300][6]) * ddt_scale);
        let eq197_e2474_d_n7: f64 = ((p.p254 * s.dn[300][7]) * ddt_scale);
        let eq197_e2474_d_n8: f64 = ((p.p254 * s.dn[300][8]) * ddt_scale);
        let eq197_e2474_d_n9: f64 = ((p.p254 * s.dn[300][9]) * ddt_scale);
        let eq197_e2474_d_n10: f64 = ((p.p254 * s.dn[300][10]) * ddt_scale);
        let eq197_e2474_d_n11: f64 = ((p.p254 * s.dn[300][11]) * ddt_scale);
        let eq197_e2474_d_n12: f64 = ((p.p254 * s.dn[300][12]) * ddt_scale);
        let eq197_e2474_d_n13: f64 = ((p.p254 * s.dn[300][13]) * ddt_scale);
        let eq197_e2474_d_n14: f64 = ((p.p254 * s.dn[300][14]) * ddt_scale);
        let eq197_e2474_d_n15: f64 = ((p.p254 * s.dn[300][15]) * ddt_scale);
        let eq197_e2474_d_n16: f64 = ((p.p254 * s.dn[300][16]) * ddt_scale);
        let eq197_e2474_d_n17: f64 = ((p.p254 * s.dn[300][17]) * ddt_scale);
        let eq197_e2474_d_n18: f64 = ((p.p254 * s.dn[300][18]) * ddt_scale);
        let eq197_e2474_d_n19: f64 = ((p.p254 * s.dn[300][19]) * ddt_scale);
        let eq197_e2474_d_n20: f64 = ((p.p254 * s.dn[300][20]) * ddt_scale);
        let eq197_e2474_d_n21: f64 = ((p.p254 * s.dn[300][21]) * ddt_scale);
        let eq197_e2474_d_n22: f64 = ((p.p254 * s.dn[300][22]) * ddt_scale);
        let eq197_e2474_d_b0: f64 = ((p.p254 * s.db[300][0]) * ddt_scale);
        let eq197_e2474_d_b1: f64 = ((p.p254 * s.db[300][1]) * ddt_scale);
        let eq197_e2474_d_b2: f64 = ((p.p254 * s.db[300][2]) * ddt_scale);
        let eq197_e2474_d_b3: f64 = ((p.p254 * s.db[300][3]) * ddt_scale);
        let eq197_e2474_d_b4: f64 = ((p.p254 * s.db[300][4]) * ddt_scale);
        let eq197_e2474_d_b5: f64 = ((p.p254 * s.db[300][5]) * ddt_scale);
        let eq197_e2474_d_b6: f64 = ((p.p254 * s.db[300][6]) * ddt_scale);
        let eq197_e2474_d_b7: f64 = ((p.p254 * s.db[300][7]) * ddt_scale);
        let eq197_e2474_d_b8: f64 = ((p.p254 * s.db[300][8]) * ddt_scale);
        let eq197_e2474_d_b9: f64 = ((p.p254 * s.db[300][9]) * ddt_scale);
        let eq197_e2474_d_b10: f64 = ((p.p254 * s.db[300][10]) * ddt_scale);
        let eq197_e2474_d_b11: f64 = ((p.p254 * s.db[300][11]) * ddt_scale);
        let eq197_e2474_d_b12: f64 = ((p.p254 * s.db[300][12]) * ddt_scale);
        let eq197_e2474_d_b13: f64 = ((p.p254 * s.db[300][13]) * ddt_scale);
        let eq197_e2474_d_b14: f64 = ((p.p254 * s.db[300][14]) * ddt_scale);
        let eq197_e2474_d_b15: f64 = ((p.p254 * s.db[300][15]) * ddt_scale);
        let eq197_e2474_d_b16: f64 = ((p.p254 * s.db[300][16]) * ddt_scale);
        let eq197_e2474_d_b17: f64 = ((p.p254 * s.db[300][17]) * ddt_scale);
        let eq197_e2474_d_b18: f64 = ((p.p254 * s.db[300][18]) * ddt_scale);
        let eq197_e2474_d_b19: f64 = ((p.p254 * s.db[300][19]) * ddt_scale);
        let eq197_e2474_d_b20: f64 = ((p.p254 * s.db[300][20]) * ddt_scale);
        let eq197_e2474_d_b21: f64 = ((p.p254 * s.db[300][21]) * ddt_scale);
        let eq197_e2474_d_b22: f64 = ((p.p254 * s.db[300][22]) * ddt_scale);
        let eq197_e2474_d_b23: f64 = ((p.p254 * s.db[300][23]) * ddt_scale);
        let eq197_e2474_d_b24: f64 = ((p.p254 * s.db[300][24]) * ddt_scale);
        let eq197_e2474_d_b25: f64 = ((p.p254 * s.db[300][25]) * ddt_scale);
        let eq197_e2474_d_b26: f64 = ((p.p254 * s.db[300][26]) * ddt_scale);
        let eq197_e2474_d_b27: f64 = ((p.p254 * s.db[300][27]) * ddt_scale);
        let eq197_e2474_d_b28: f64 = ((p.p254 * s.db[300][28]) * ddt_scale);
        let eq197_e2474_d_b29: f64 = ((p.p254 * s.db[300][29]) * ddt_scale);
        let eq197_e2474_d_b30: f64 = ((p.p254 * s.db[300][30]) * ddt_scale);
        let eq197_e2474_d_b31: f64 = ((p.p254 * s.db[300][31]) * ddt_scale);
        let eq197_e2474_d_b32: f64 = ((p.p254 * s.db[300][32]) * ddt_scale);
        let eq197_e2474_d_b33: f64 = ((p.p254 * s.db[300][33]) * ddt_scale);
        let eq197_e2474_d_b34: f64 = ((p.p254 * s.db[300][34]) * ddt_scale);
        let eq197_e2474_d_b35: f64 = ((p.p254 * s.db[300][35]) * ddt_scale);
        let eq197_e2474_d_b36: f64 = ((p.p254 * s.db[300][36]) * ddt_scale);
        let eq197_e2474_d_b37: f64 = ((p.p254 * s.db[300][37]) * ddt_scale);
        let eq197_e2474_d_b38: f64 = ((p.p254 * s.db[300][38]) * ddt_scale);
        let eq197_e2474_d_b39: f64 = ((p.p254 * s.db[300][39]) * ddt_scale);
        let eq197_e2474_d_b40: f64 = ((p.p254 * s.db[300][40]) * ddt_scale);
        let eq197_e2474_d_b41: f64 = ((p.p254 * s.db[300][41]) * ddt_scale);
        let eq197_e2474_d_b42: f64 = ((p.p254 * s.db[300][42]) * ddt_scale);
        let eq197_e2474_d_b43: f64 = ((p.p254 * s.db[300][43]) * ddt_scale);
        let eq197_e2474_d_b44: f64 = ((p.p254 * s.db[300][44]) * ddt_scale);
        let eq197_e2474_d_b45: f64 = ((p.p254 * s.db[300][45]) * ddt_scale);
        let eq197_e2474_d_b46: f64 = ((p.p254 * s.db[300][46]) * ddt_scale);
        let eq197_e2474_d_b47: f64 = ((p.p254 * s.db[300][47]) * ddt_scale);
        let eq197_e2474_d_b48: f64 = ((p.p254 * s.db[300][48]) * ddt_scale);
        let eq197_e2474_d_b49: f64 = ((p.p254 * s.db[300][49]) * ddt_scale);
        let eq197_e2474_d_b50: f64 = ((p.p254 * s.db[300][50]) * ddt_scale);
        let eq197_e2474_d_b51: f64 = ((p.p254 * s.db[300][51]) * ddt_scale);
        let eq197_e2474_d_b52: f64 = ((p.p254 * s.db[300][52]) * ddt_scale);
        let eq197_e2474_d_b53: f64 = ((p.p254 * s.db[300][53]) * ddt_scale);
        let eq197_e2474_d_b54: f64 = ((p.p254 * s.db[300][54]) * ddt_scale);
        let eq197_e2475: f64 = (p.p7 * eq197_e2474);
        let eq197_e2475_d_n0: f64 = (p.p7 * eq197_e2474_d_n0);
        let eq197_e2475_d_n1: f64 = (p.p7 * eq197_e2474_d_n1);
        let eq197_e2475_d_n2: f64 = (p.p7 * eq197_e2474_d_n2);
        let eq197_e2475_d_n3: f64 = (p.p7 * eq197_e2474_d_n3);
        let eq197_e2475_d_n4: f64 = (p.p7 * eq197_e2474_d_n4);
        let eq197_e2475_d_n5: f64 = (p.p7 * eq197_e2474_d_n5);
        let eq197_e2475_d_n6: f64 = (p.p7 * eq197_e2474_d_n6);
        let eq197_e2475_d_n7: f64 = (p.p7 * eq197_e2474_d_n7);
        let eq197_e2475_d_n8: f64 = (p.p7 * eq197_e2474_d_n8);
        let eq197_e2475_d_n9: f64 = (p.p7 * eq197_e2474_d_n9);
        let eq197_e2475_d_n10: f64 = (p.p7 * eq197_e2474_d_n10);
        let eq197_e2475_d_n11: f64 = (p.p7 * eq197_e2474_d_n11);
        let eq197_e2475_d_n12: f64 = (p.p7 * eq197_e2474_d_n12);
        let eq197_e2475_d_n13: f64 = (p.p7 * eq197_e2474_d_n13);
        let eq197_e2475_d_n14: f64 = (p.p7 * eq197_e2474_d_n14);
        let eq197_e2475_d_n15: f64 = (p.p7 * eq197_e2474_d_n15);
        let eq197_e2475_d_n16: f64 = (p.p7 * eq197_e2474_d_n16);
        let eq197_e2475_d_n17: f64 = (p.p7 * eq197_e2474_d_n17);
        let eq197_e2475_d_n18: f64 = (p.p7 * eq197_e2474_d_n18);
        let eq197_e2475_d_n19: f64 = (p.p7 * eq197_e2474_d_n19);
        let eq197_e2475_d_n20: f64 = (p.p7 * eq197_e2474_d_n20);
        let eq197_e2475_d_n21: f64 = (p.p7 * eq197_e2474_d_n21);
        let eq197_e2475_d_n22: f64 = (p.p7 * eq197_e2474_d_n22);
        let eq197_e2475_d_b0: f64 = (p.p7 * eq197_e2474_d_b0);
        let eq197_e2475_d_b1: f64 = (p.p7 * eq197_e2474_d_b1);
        let eq197_e2475_d_b2: f64 = (p.p7 * eq197_e2474_d_b2);
        let eq197_e2475_d_b3: f64 = (p.p7 * eq197_e2474_d_b3);
        let eq197_e2475_d_b4: f64 = (p.p7 * eq197_e2474_d_b4);
        let eq197_e2475_d_b5: f64 = (p.p7 * eq197_e2474_d_b5);
        let eq197_e2475_d_b6: f64 = (p.p7 * eq197_e2474_d_b6);
        let eq197_e2475_d_b7: f64 = (p.p7 * eq197_e2474_d_b7);
        let eq197_e2475_d_b8: f64 = (p.p7 * eq197_e2474_d_b8);
        let eq197_e2475_d_b9: f64 = (p.p7 * eq197_e2474_d_b9);
        let eq197_e2475_d_b10: f64 = (p.p7 * eq197_e2474_d_b10);
        let eq197_e2475_d_b11: f64 = (p.p7 * eq197_e2474_d_b11);
        let eq197_e2475_d_b12: f64 = (p.p7 * eq197_e2474_d_b12);
        let eq197_e2475_d_b13: f64 = (p.p7 * eq197_e2474_d_b13);
        let eq197_e2475_d_b14: f64 = (p.p7 * eq197_e2474_d_b14);
        let eq197_e2475_d_b15: f64 = (p.p7 * eq197_e2474_d_b15);
        let eq197_e2475_d_b16: f64 = (p.p7 * eq197_e2474_d_b16);
        let eq197_e2475_d_b17: f64 = (p.p7 * eq197_e2474_d_b17);
        let eq197_e2475_d_b18: f64 = (p.p7 * eq197_e2474_d_b18);
        let eq197_e2475_d_b19: f64 = (p.p7 * eq197_e2474_d_b19);
        let eq197_e2475_d_b20: f64 = (p.p7 * eq197_e2474_d_b20);
        let eq197_e2475_d_b21: f64 = (p.p7 * eq197_e2474_d_b21);
        let eq197_e2475_d_b22: f64 = (p.p7 * eq197_e2474_d_b22);
        let eq197_e2475_d_b23: f64 = (p.p7 * eq197_e2474_d_b23);
        let eq197_e2475_d_b24: f64 = (p.p7 * eq197_e2474_d_b24);
        let eq197_e2475_d_b25: f64 = (p.p7 * eq197_e2474_d_b25);
        let eq197_e2475_d_b26: f64 = (p.p7 * eq197_e2474_d_b26);
        let eq197_e2475_d_b27: f64 = (p.p7 * eq197_e2474_d_b27);
        let eq197_e2475_d_b28: f64 = (p.p7 * eq197_e2474_d_b28);
        let eq197_e2475_d_b29: f64 = (p.p7 * eq197_e2474_d_b29);
        let eq197_e2475_d_b30: f64 = (p.p7 * eq197_e2474_d_b30);
        let eq197_e2475_d_b31: f64 = (p.p7 * eq197_e2474_d_b31);
        let eq197_e2475_d_b32: f64 = (p.p7 * eq197_e2474_d_b32);
        let eq197_e2475_d_b33: f64 = (p.p7 * eq197_e2474_d_b33);
        let eq197_e2475_d_b34: f64 = (p.p7 * eq197_e2474_d_b34);
        let eq197_e2475_d_b35: f64 = (p.p7 * eq197_e2474_d_b35);
        let eq197_e2475_d_b36: f64 = (p.p7 * eq197_e2474_d_b36);
        let eq197_e2475_d_b37: f64 = (p.p7 * eq197_e2474_d_b37);
        let eq197_e2475_d_b38: f64 = (p.p7 * eq197_e2474_d_b38);
        let eq197_e2475_d_b39: f64 = (p.p7 * eq197_e2474_d_b39);
        let eq197_e2475_d_b40: f64 = (p.p7 * eq197_e2474_d_b40);
        let eq197_e2475_d_b41: f64 = (p.p7 * eq197_e2474_d_b41);
        let eq197_e2475_d_b42: f64 = (p.p7 * eq197_e2474_d_b42);
        let eq197_e2475_d_b43: f64 = (p.p7 * eq197_e2474_d_b43);
        let eq197_e2475_d_b44: f64 = (p.p7 * eq197_e2474_d_b44);
        let eq197_e2475_d_b45: f64 = (p.p7 * eq197_e2474_d_b45);
        let eq197_e2475_d_b46: f64 = (p.p7 * eq197_e2474_d_b46);
        let eq197_e2475_d_b47: f64 = (p.p7 * eq197_e2474_d_b47);
        let eq197_e2475_d_b48: f64 = (p.p7 * eq197_e2474_d_b48);
        let eq197_e2475_d_b49: f64 = (p.p7 * eq197_e2474_d_b49);
        let eq197_e2475_d_b50: f64 = (p.p7 * eq197_e2474_d_b50);
        let eq197_e2475_d_b51: f64 = (p.p7 * eq197_e2474_d_b51);
        let eq197_e2475_d_b52: f64 = (p.p7 * eq197_e2474_d_b52);
        let eq197_e2475_d_b53: f64 = (p.p7 * eq197_e2474_d_b53);
        let eq197_e2475_d_b54: f64 = (p.p7 * eq197_e2474_d_b54);
        (eq197_e2475, eq197_e2475_d_n0, eq197_e2475_d_n1, eq197_e2475_d_n2, eq197_e2475_d_n3, eq197_e2475_d_n4, eq197_e2475_d_n5, eq197_e2475_d_n6, eq197_e2475_d_n7, eq197_e2475_d_n8, eq197_e2475_d_n9, eq197_e2475_d_n10, eq197_e2475_d_n11, eq197_e2475_d_n12, eq197_e2475_d_n13, eq197_e2475_d_n14, eq197_e2475_d_n15, eq197_e2475_d_n16, eq197_e2475_d_n17, eq197_e2475_d_n18, eq197_e2475_d_n19, eq197_e2475_d_n20, eq197_e2475_d_n21, eq197_e2475_d_n22, eq197_e2475_d_b0, eq197_e2475_d_b1, eq197_e2475_d_b2, eq197_e2475_d_b3, eq197_e2475_d_b4, eq197_e2475_d_b5, eq197_e2475_d_b6, eq197_e2475_d_b7, eq197_e2475_d_b8, eq197_e2475_d_b9, eq197_e2475_d_b10, eq197_e2475_d_b11, eq197_e2475_d_b12, eq197_e2475_d_b13, eq197_e2475_d_b14, eq197_e2475_d_b15, eq197_e2475_d_b16, eq197_e2475_d_b17, eq197_e2475_d_b18, eq197_e2475_d_b19, eq197_e2475_d_b20, eq197_e2475_d_b21, eq197_e2475_d_b22, eq197_e2475_d_b23, eq197_e2475_d_b24, eq197_e2475_d_b25, eq197_e2475_d_b26, eq197_e2475_d_b27, eq197_e2475_d_b28, eq197_e2475_d_b29, eq197_e2475_d_b30, eq197_e2475_d_b31, eq197_e2475_d_b32, eq197_e2475_d_b33, eq197_e2475_d_b34, eq197_e2475_d_b35, eq197_e2475_d_b36, eq197_e2475_d_b37, eq197_e2475_d_b38, eq197_e2475_d_b39, eq197_e2475_d_b40, eq197_e2475_d_b41, eq197_e2475_d_b42, eq197_e2475_d_b43, eq197_e2475_d_b44, eq197_e2475_d_b45, eq197_e2475_d_b46, eq197_e2475_d_b47, eq197_e2475_d_b48, eq197_e2475_d_b49, eq197_e2475_d_b50, eq197_e2475_d_b51, eq197_e2475_d_b52, eq197_e2475_d_b53, eq197_e2475_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq197_value: f64 = eq197_e2477;
        let eq197_node_derivatives: [f64; 23] = [eq197_e2477_d_n0, eq197_e2477_d_n1, eq197_e2477_d_n2, eq197_e2477_d_n3, eq197_e2477_d_n4, eq197_e2477_d_n5, eq197_e2477_d_n6, eq197_e2477_d_n7, eq197_e2477_d_n8, eq197_e2477_d_n9, eq197_e2477_d_n10, eq197_e2477_d_n11, eq197_e2477_d_n12, eq197_e2477_d_n13, eq197_e2477_d_n14, eq197_e2477_d_n15, eq197_e2477_d_n16, eq197_e2477_d_n17, eq197_e2477_d_n18, eq197_e2477_d_n19, eq197_e2477_d_n20, eq197_e2477_d_n21, eq197_e2477_d_n22];
        let eq197_branch_derivatives: [f64; 55] = [eq197_e2477_d_b0, eq197_e2477_d_b1, eq197_e2477_d_b2, eq197_e2477_d_b3, eq197_e2477_d_b4, eq197_e2477_d_b5, eq197_e2477_d_b6, eq197_e2477_d_b7, eq197_e2477_d_b8, eq197_e2477_d_b9, eq197_e2477_d_b10, eq197_e2477_d_b11, eq197_e2477_d_b12, eq197_e2477_d_b13, eq197_e2477_d_b14, eq197_e2477_d_b15, eq197_e2477_d_b16, eq197_e2477_d_b17, eq197_e2477_d_b18, eq197_e2477_d_b19, eq197_e2477_d_b20, eq197_e2477_d_b21, eq197_e2477_d_b22, eq197_e2477_d_b23, eq197_e2477_d_b24, eq197_e2477_d_b25, eq197_e2477_d_b26, eq197_e2477_d_b27, eq197_e2477_d_b28, eq197_e2477_d_b29, eq197_e2477_d_b30, eq197_e2477_d_b31, eq197_e2477_d_b32, eq197_e2477_d_b33, eq197_e2477_d_b34, eq197_e2477_d_b35, eq197_e2477_d_b36, eq197_e2477_d_b37, eq197_e2477_d_b38, eq197_e2477_d_b39, eq197_e2477_d_b40, eq197_e2477_d_b41, eq197_e2477_d_b42, eq197_e2477_d_b43, eq197_e2477_d_b44, eq197_e2477_d_b45, eq197_e2477_d_b46, eq197_e2477_d_b47, eq197_e2477_d_b48, eq197_e2477_d_b49, eq197_e2477_d_b50, eq197_e2477_d_b51, eq197_e2477_d_b52, eq197_e2477_d_b53, eq197_e2477_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(17),
            multiplicity * (eq197_value),
            &eq197_node_derivatives,
            &eq197_branch_derivatives,
            multiplicity,
        );
        let (eq198_e2487, eq198_e2487_d_n0, eq198_e2487_d_n1, eq198_e2487_d_n2, eq198_e2487_d_n3, eq198_e2487_d_n4, eq198_e2487_d_n5, eq198_e2487_d_n6, eq198_e2487_d_n7, eq198_e2487_d_n8, eq198_e2487_d_n9, eq198_e2487_d_n10, eq198_e2487_d_n11, eq198_e2487_d_n12, eq198_e2487_d_n13, eq198_e2487_d_n14, eq198_e2487_d_n15, eq198_e2487_d_n16, eq198_e2487_d_n17, eq198_e2487_d_n18, eq198_e2487_d_n19, eq198_e2487_d_n20, eq198_e2487_d_n21, eq198_e2487_d_n22, eq198_e2487_d_b0, eq198_e2487_d_b1, eq198_e2487_d_b2, eq198_e2487_d_b3, eq198_e2487_d_b4, eq198_e2487_d_b5, eq198_e2487_d_b6, eq198_e2487_d_b7, eq198_e2487_d_b8, eq198_e2487_d_b9, eq198_e2487_d_b10, eq198_e2487_d_b11, eq198_e2487_d_b12, eq198_e2487_d_b13, eq198_e2487_d_b14, eq198_e2487_d_b15, eq198_e2487_d_b16, eq198_e2487_d_b17, eq198_e2487_d_b18, eq198_e2487_d_b19, eq198_e2487_d_b20, eq198_e2487_d_b21, eq198_e2487_d_b22, eq198_e2487_d_b23, eq198_e2487_d_b24, eq198_e2487_d_b25, eq198_e2487_d_b26, eq198_e2487_d_b27, eq198_e2487_d_b28, eq198_e2487_d_b29, eq198_e2487_d_b30, eq198_e2487_d_b31, eq198_e2487_d_b32, eq198_e2487_d_b33, eq198_e2487_d_b34, eq198_e2487_d_b35, eq198_e2487_d_b36, eq198_e2487_d_b37, eq198_e2487_d_b38, eq198_e2487_d_b39, eq198_e2487_d_b40, eq198_e2487_d_b41, eq198_e2487_d_b42, eq198_e2487_d_b43, eq198_e2487_d_b44, eq198_e2487_d_b45, eq198_e2487_d_b46, eq198_e2487_d_b47, eq198_e2487_d_b48, eq198_e2487_d_b49, eq198_e2487_d_b50, eq198_e2487_d_b51, eq198_e2487_d_b52, eq198_e2487_d_b53, eq198_e2487_d_b54,) = {
    if ((!s.b[600]) && s.b[603]) {
        let eq198_e2484: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 97, s.v[301]);
        let eq198_e2485: f64 = (p.p7 * eq198_e2484);
        let eq198_e2485_d_n0: f64 = (p.p7 * (s.dn[301][0] * ddt_scale));
        let eq198_e2485_d_n1: f64 = (p.p7 * (s.dn[301][1] * ddt_scale));
        let eq198_e2485_d_n2: f64 = (p.p7 * (s.dn[301][2] * ddt_scale));
        let eq198_e2485_d_n3: f64 = (p.p7 * (s.dn[301][3] * ddt_scale));
        let eq198_e2485_d_n4: f64 = (p.p7 * (s.dn[301][4] * ddt_scale));
        let eq198_e2485_d_n5: f64 = (p.p7 * (s.dn[301][5] * ddt_scale));
        let eq198_e2485_d_n6: f64 = (p.p7 * (s.dn[301][6] * ddt_scale));
        let eq198_e2485_d_n7: f64 = (p.p7 * (s.dn[301][7] * ddt_scale));
        let eq198_e2485_d_n8: f64 = (p.p7 * (s.dn[301][8] * ddt_scale));
        let eq198_e2485_d_n9: f64 = (p.p7 * (s.dn[301][9] * ddt_scale));
        let eq198_e2485_d_n10: f64 = (p.p7 * (s.dn[301][10] * ddt_scale));
        let eq198_e2485_d_n11: f64 = (p.p7 * (s.dn[301][11] * ddt_scale));
        let eq198_e2485_d_n12: f64 = (p.p7 * (s.dn[301][12] * ddt_scale));
        let eq198_e2485_d_n13: f64 = (p.p7 * (s.dn[301][13] * ddt_scale));
        let eq198_e2485_d_n14: f64 = (p.p7 * (s.dn[301][14] * ddt_scale));
        let eq198_e2485_d_n15: f64 = (p.p7 * (s.dn[301][15] * ddt_scale));
        let eq198_e2485_d_n16: f64 = (p.p7 * (s.dn[301][16] * ddt_scale));
        let eq198_e2485_d_n17: f64 = (p.p7 * (s.dn[301][17] * ddt_scale));
        let eq198_e2485_d_n18: f64 = (p.p7 * (s.dn[301][18] * ddt_scale));
        let eq198_e2485_d_n19: f64 = (p.p7 * (s.dn[301][19] * ddt_scale));
        let eq198_e2485_d_n20: f64 = (p.p7 * (s.dn[301][20] * ddt_scale));
        let eq198_e2485_d_n21: f64 = (p.p7 * (s.dn[301][21] * ddt_scale));
        let eq198_e2485_d_n22: f64 = (p.p7 * (s.dn[301][22] * ddt_scale));
        let eq198_e2485_d_b0: f64 = (p.p7 * (s.db[301][0] * ddt_scale));
        let eq198_e2485_d_b1: f64 = (p.p7 * (s.db[301][1] * ddt_scale));
        let eq198_e2485_d_b2: f64 = (p.p7 * (s.db[301][2] * ddt_scale));
        let eq198_e2485_d_b3: f64 = (p.p7 * (s.db[301][3] * ddt_scale));
        let eq198_e2485_d_b4: f64 = (p.p7 * (s.db[301][4] * ddt_scale));
        let eq198_e2485_d_b5: f64 = (p.p7 * (s.db[301][5] * ddt_scale));
        let eq198_e2485_d_b6: f64 = (p.p7 * (s.db[301][6] * ddt_scale));
        let eq198_e2485_d_b7: f64 = (p.p7 * (s.db[301][7] * ddt_scale));
        let eq198_e2485_d_b8: f64 = (p.p7 * (s.db[301][8] * ddt_scale));
        let eq198_e2485_d_b9: f64 = (p.p7 * (s.db[301][9] * ddt_scale));
        let eq198_e2485_d_b10: f64 = (p.p7 * (s.db[301][10] * ddt_scale));
        let eq198_e2485_d_b11: f64 = (p.p7 * (s.db[301][11] * ddt_scale));
        let eq198_e2485_d_b12: f64 = (p.p7 * (s.db[301][12] * ddt_scale));
        let eq198_e2485_d_b13: f64 = (p.p7 * (s.db[301][13] * ddt_scale));
        let eq198_e2485_d_b14: f64 = (p.p7 * (s.db[301][14] * ddt_scale));
        let eq198_e2485_d_b15: f64 = (p.p7 * (s.db[301][15] * ddt_scale));
        let eq198_e2485_d_b16: f64 = (p.p7 * (s.db[301][16] * ddt_scale));
        let eq198_e2485_d_b17: f64 = (p.p7 * (s.db[301][17] * ddt_scale));
        let eq198_e2485_d_b18: f64 = (p.p7 * (s.db[301][18] * ddt_scale));
        let eq198_e2485_d_b19: f64 = (p.p7 * (s.db[301][19] * ddt_scale));
        let eq198_e2485_d_b20: f64 = (p.p7 * (s.db[301][20] * ddt_scale));
        let eq198_e2485_d_b21: f64 = (p.p7 * (s.db[301][21] * ddt_scale));
        let eq198_e2485_d_b22: f64 = (p.p7 * (s.db[301][22] * ddt_scale));
        let eq198_e2485_d_b23: f64 = (p.p7 * (s.db[301][23] * ddt_scale));
        let eq198_e2485_d_b24: f64 = (p.p7 * (s.db[301][24] * ddt_scale));
        let eq198_e2485_d_b25: f64 = (p.p7 * (s.db[301][25] * ddt_scale));
        let eq198_e2485_d_b26: f64 = (p.p7 * (s.db[301][26] * ddt_scale));
        let eq198_e2485_d_b27: f64 = (p.p7 * (s.db[301][27] * ddt_scale));
        let eq198_e2485_d_b28: f64 = (p.p7 * (s.db[301][28] * ddt_scale));
        let eq198_e2485_d_b29: f64 = (p.p7 * (s.db[301][29] * ddt_scale));
        let eq198_e2485_d_b30: f64 = (p.p7 * (s.db[301][30] * ddt_scale));
        let eq198_e2485_d_b31: f64 = (p.p7 * (s.db[301][31] * ddt_scale));
        let eq198_e2485_d_b32: f64 = (p.p7 * (s.db[301][32] * ddt_scale));
        let eq198_e2485_d_b33: f64 = (p.p7 * (s.db[301][33] * ddt_scale));
        let eq198_e2485_d_b34: f64 = (p.p7 * (s.db[301][34] * ddt_scale));
        let eq198_e2485_d_b35: f64 = (p.p7 * (s.db[301][35] * ddt_scale));
        let eq198_e2485_d_b36: f64 = (p.p7 * (s.db[301][36] * ddt_scale));
        let eq198_e2485_d_b37: f64 = (p.p7 * (s.db[301][37] * ddt_scale));
        let eq198_e2485_d_b38: f64 = (p.p7 * (s.db[301][38] * ddt_scale));
        let eq198_e2485_d_b39: f64 = (p.p7 * (s.db[301][39] * ddt_scale));
        let eq198_e2485_d_b40: f64 = (p.p7 * (s.db[301][40] * ddt_scale));
        let eq198_e2485_d_b41: f64 = (p.p7 * (s.db[301][41] * ddt_scale));
        let eq198_e2485_d_b42: f64 = (p.p7 * (s.db[301][42] * ddt_scale));
        let eq198_e2485_d_b43: f64 = (p.p7 * (s.db[301][43] * ddt_scale));
        let eq198_e2485_d_b44: f64 = (p.p7 * (s.db[301][44] * ddt_scale));
        let eq198_e2485_d_b45: f64 = (p.p7 * (s.db[301][45] * ddt_scale));
        let eq198_e2485_d_b46: f64 = (p.p7 * (s.db[301][46] * ddt_scale));
        let eq198_e2485_d_b47: f64 = (p.p7 * (s.db[301][47] * ddt_scale));
        let eq198_e2485_d_b48: f64 = (p.p7 * (s.db[301][48] * ddt_scale));
        let eq198_e2485_d_b49: f64 = (p.p7 * (s.db[301][49] * ddt_scale));
        let eq198_e2485_d_b50: f64 = (p.p7 * (s.db[301][50] * ddt_scale));
        let eq198_e2485_d_b51: f64 = (p.p7 * (s.db[301][51] * ddt_scale));
        let eq198_e2485_d_b52: f64 = (p.p7 * (s.db[301][52] * ddt_scale));
        let eq198_e2485_d_b53: f64 = (p.p7 * (s.db[301][53] * ddt_scale));
        let eq198_e2485_d_b54: f64 = (p.p7 * (s.db[301][54] * ddt_scale));
        (eq198_e2485, eq198_e2485_d_n0, eq198_e2485_d_n1, eq198_e2485_d_n2, eq198_e2485_d_n3, eq198_e2485_d_n4, eq198_e2485_d_n5, eq198_e2485_d_n6, eq198_e2485_d_n7, eq198_e2485_d_n8, eq198_e2485_d_n9, eq198_e2485_d_n10, eq198_e2485_d_n11, eq198_e2485_d_n12, eq198_e2485_d_n13, eq198_e2485_d_n14, eq198_e2485_d_n15, eq198_e2485_d_n16, eq198_e2485_d_n17, eq198_e2485_d_n18, eq198_e2485_d_n19, eq198_e2485_d_n20, eq198_e2485_d_n21, eq198_e2485_d_n22, eq198_e2485_d_b0, eq198_e2485_d_b1, eq198_e2485_d_b2, eq198_e2485_d_b3, eq198_e2485_d_b4, eq198_e2485_d_b5, eq198_e2485_d_b6, eq198_e2485_d_b7, eq198_e2485_d_b8, eq198_e2485_d_b9, eq198_e2485_d_b10, eq198_e2485_d_b11, eq198_e2485_d_b12, eq198_e2485_d_b13, eq198_e2485_d_b14, eq198_e2485_d_b15, eq198_e2485_d_b16, eq198_e2485_d_b17, eq198_e2485_d_b18, eq198_e2485_d_b19, eq198_e2485_d_b20, eq198_e2485_d_b21, eq198_e2485_d_b22, eq198_e2485_d_b23, eq198_e2485_d_b24, eq198_e2485_d_b25, eq198_e2485_d_b26, eq198_e2485_d_b27, eq198_e2485_d_b28, eq198_e2485_d_b29, eq198_e2485_d_b30, eq198_e2485_d_b31, eq198_e2485_d_b32, eq198_e2485_d_b33, eq198_e2485_d_b34, eq198_e2485_d_b35, eq198_e2485_d_b36, eq198_e2485_d_b37, eq198_e2485_d_b38, eq198_e2485_d_b39, eq198_e2485_d_b40, eq198_e2485_d_b41, eq198_e2485_d_b42, eq198_e2485_d_b43, eq198_e2485_d_b44, eq198_e2485_d_b45, eq198_e2485_d_b46, eq198_e2485_d_b47, eq198_e2485_d_b48, eq198_e2485_d_b49, eq198_e2485_d_b50, eq198_e2485_d_b51, eq198_e2485_d_b52, eq198_e2485_d_b53, eq198_e2485_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq198_value: f64 = eq198_e2487;
        let eq198_node_derivatives: [f64; 23] = [eq198_e2487_d_n0, eq198_e2487_d_n1, eq198_e2487_d_n2, eq198_e2487_d_n3, eq198_e2487_d_n4, eq198_e2487_d_n5, eq198_e2487_d_n6, eq198_e2487_d_n7, eq198_e2487_d_n8, eq198_e2487_d_n9, eq198_e2487_d_n10, eq198_e2487_d_n11, eq198_e2487_d_n12, eq198_e2487_d_n13, eq198_e2487_d_n14, eq198_e2487_d_n15, eq198_e2487_d_n16, eq198_e2487_d_n17, eq198_e2487_d_n18, eq198_e2487_d_n19, eq198_e2487_d_n20, eq198_e2487_d_n21, eq198_e2487_d_n22];
        let eq198_branch_derivatives: [f64; 55] = [eq198_e2487_d_b0, eq198_e2487_d_b1, eq198_e2487_d_b2, eq198_e2487_d_b3, eq198_e2487_d_b4, eq198_e2487_d_b5, eq198_e2487_d_b6, eq198_e2487_d_b7, eq198_e2487_d_b8, eq198_e2487_d_b9, eq198_e2487_d_b10, eq198_e2487_d_b11, eq198_e2487_d_b12, eq198_e2487_d_b13, eq198_e2487_d_b14, eq198_e2487_d_b15, eq198_e2487_d_b16, eq198_e2487_d_b17, eq198_e2487_d_b18, eq198_e2487_d_b19, eq198_e2487_d_b20, eq198_e2487_d_b21, eq198_e2487_d_b22, eq198_e2487_d_b23, eq198_e2487_d_b24, eq198_e2487_d_b25, eq198_e2487_d_b26, eq198_e2487_d_b27, eq198_e2487_d_b28, eq198_e2487_d_b29, eq198_e2487_d_b30, eq198_e2487_d_b31, eq198_e2487_d_b32, eq198_e2487_d_b33, eq198_e2487_d_b34, eq198_e2487_d_b35, eq198_e2487_d_b36, eq198_e2487_d_b37, eq198_e2487_d_b38, eq198_e2487_d_b39, eq198_e2487_d_b40, eq198_e2487_d_b41, eq198_e2487_d_b42, eq198_e2487_d_b43, eq198_e2487_d_b44, eq198_e2487_d_b45, eq198_e2487_d_b46, eq198_e2487_d_b47, eq198_e2487_d_b48, eq198_e2487_d_b49, eq198_e2487_d_b50, eq198_e2487_d_b51, eq198_e2487_d_b52, eq198_e2487_d_b53, eq198_e2487_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq198_value),
            &eq198_node_derivatives,
            &eq198_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_41(
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[300][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[300][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[300][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[300][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[300][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[300][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[300][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[300][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[300][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[300][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[300][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[300][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[300][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[300][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[300][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[300][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[300][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[300][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[300][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[300][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[300][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[300][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[300][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[300][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[300][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[300][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[300][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[300][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[300][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[300][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[300][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[300][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[300][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[300][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[300][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[300][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[300][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[300][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[300][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[300][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[300][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[300][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[300][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[300][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[300][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[300][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[300][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[300][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[300][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[300][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[300][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[300][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[300][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[300][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[300][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[300][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[300][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[300][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[300][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[300][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[300][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[300][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[300][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[300][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[300][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[300][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[300][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[300][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[300][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[300][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[300][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[300][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[300][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[300][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[300][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[300][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[300][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[300][54] * ddt_scale));
        let (eq199_e2499, eq199_e2499_d_n0, eq199_e2499_d_n1, eq199_e2499_d_n2, eq199_e2499_d_n3, eq199_e2499_d_n4, eq199_e2499_d_n5, eq199_e2499_d_n6, eq199_e2499_d_n7, eq199_e2499_d_n8, eq199_e2499_d_n9, eq199_e2499_d_n10, eq199_e2499_d_n11, eq199_e2499_d_n12, eq199_e2499_d_n13, eq199_e2499_d_n14, eq199_e2499_d_n15, eq199_e2499_d_n16, eq199_e2499_d_n17, eq199_e2499_d_n18, eq199_e2499_d_n19, eq199_e2499_d_n20, eq199_e2499_d_n21, eq199_e2499_d_n22, eq199_e2499_d_b0, eq199_e2499_d_b1, eq199_e2499_d_b2, eq199_e2499_d_b3, eq199_e2499_d_b4, eq199_e2499_d_b5, eq199_e2499_d_b6, eq199_e2499_d_b7, eq199_e2499_d_b8, eq199_e2499_d_b9, eq199_e2499_d_b10, eq199_e2499_d_b11, eq199_e2499_d_b12, eq199_e2499_d_b13, eq199_e2499_d_b14, eq199_e2499_d_b15, eq199_e2499_d_b16, eq199_e2499_d_b17, eq199_e2499_d_b18, eq199_e2499_d_b19, eq199_e2499_d_b20, eq199_e2499_d_b21, eq199_e2499_d_b22, eq199_e2499_d_b23, eq199_e2499_d_b24, eq199_e2499_d_b25, eq199_e2499_d_b26, eq199_e2499_d_b27, eq199_e2499_d_b28, eq199_e2499_d_b29, eq199_e2499_d_b30, eq199_e2499_d_b31, eq199_e2499_d_b32, eq199_e2499_d_b33, eq199_e2499_d_b34, eq199_e2499_d_b35, eq199_e2499_d_b36, eq199_e2499_d_b37, eq199_e2499_d_b38, eq199_e2499_d_b39, eq199_e2499_d_b40, eq199_e2499_d_b41, eq199_e2499_d_b42, eq199_e2499_d_b43, eq199_e2499_d_b44, eq199_e2499_d_b45, eq199_e2499_d_b46, eq199_e2499_d_b47, eq199_e2499_d_b48, eq199_e2499_d_b49, eq199_e2499_d_b50, eq199_e2499_d_b51, eq199_e2499_d_b52, eq199_e2499_d_b53, eq199_e2499_d_b54,) = {
    if (((!s.b[600]) && s.b[603]) && s.b[604]) {
        let eq199_e2496: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 98, s.v[300]);
        let eq199_e2497: f64 = (p.p7 * eq199_e2496);
        (eq199_e2497, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq199_value: f64 = eq199_e2499;
        let eq199_node_derivatives: [f64; 23] = [eq199_e2499_d_n0, eq199_e2499_d_n1, eq199_e2499_d_n2, eq199_e2499_d_n3, eq199_e2499_d_n4, eq199_e2499_d_n5, eq199_e2499_d_n6, eq199_e2499_d_n7, eq199_e2499_d_n8, eq199_e2499_d_n9, eq199_e2499_d_n10, eq199_e2499_d_n11, eq199_e2499_d_n12, eq199_e2499_d_n13, eq199_e2499_d_n14, eq199_e2499_d_n15, eq199_e2499_d_n16, eq199_e2499_d_n17, eq199_e2499_d_n18, eq199_e2499_d_n19, eq199_e2499_d_n20, eq199_e2499_d_n21, eq199_e2499_d_n22];
        let eq199_branch_derivatives: [f64; 55] = [eq199_e2499_d_b0, eq199_e2499_d_b1, eq199_e2499_d_b2, eq199_e2499_d_b3, eq199_e2499_d_b4, eq199_e2499_d_b5, eq199_e2499_d_b6, eq199_e2499_d_b7, eq199_e2499_d_b8, eq199_e2499_d_b9, eq199_e2499_d_b10, eq199_e2499_d_b11, eq199_e2499_d_b12, eq199_e2499_d_b13, eq199_e2499_d_b14, eq199_e2499_d_b15, eq199_e2499_d_b16, eq199_e2499_d_b17, eq199_e2499_d_b18, eq199_e2499_d_b19, eq199_e2499_d_b20, eq199_e2499_d_b21, eq199_e2499_d_b22, eq199_e2499_d_b23, eq199_e2499_d_b24, eq199_e2499_d_b25, eq199_e2499_d_b26, eq199_e2499_d_b27, eq199_e2499_d_b28, eq199_e2499_d_b29, eq199_e2499_d_b30, eq199_e2499_d_b31, eq199_e2499_d_b32, eq199_e2499_d_b33, eq199_e2499_d_b34, eq199_e2499_d_b35, eq199_e2499_d_b36, eq199_e2499_d_b37, eq199_e2499_d_b38, eq199_e2499_d_b39, eq199_e2499_d_b40, eq199_e2499_d_b41, eq199_e2499_d_b42, eq199_e2499_d_b43, eq199_e2499_d_b44, eq199_e2499_d_b45, eq199_e2499_d_b46, eq199_e2499_d_b47, eq199_e2499_d_b48, eq199_e2499_d_b49, eq199_e2499_d_b50, eq199_e2499_d_b51, eq199_e2499_d_b52, eq199_e2499_d_b53, eq199_e2499_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq199_value),
            &eq199_node_derivatives,
            &eq199_branch_derivatives,
            multiplicity,
        );
        let (eq200_e2513, eq200_e2513_d_n0, eq200_e2513_d_n1, eq200_e2513_d_n2, eq200_e2513_d_n3, eq200_e2513_d_n4, eq200_e2513_d_n5, eq200_e2513_d_n6, eq200_e2513_d_n7, eq200_e2513_d_n8, eq200_e2513_d_n9, eq200_e2513_d_n10, eq200_e2513_d_n11, eq200_e2513_d_n12, eq200_e2513_d_n13, eq200_e2513_d_n14, eq200_e2513_d_n15, eq200_e2513_d_n16, eq200_e2513_d_n17, eq200_e2513_d_n18, eq200_e2513_d_n19, eq200_e2513_d_n20, eq200_e2513_d_n21, eq200_e2513_d_n22, eq200_e2513_d_b0, eq200_e2513_d_b1, eq200_e2513_d_b2, eq200_e2513_d_b3, eq200_e2513_d_b4, eq200_e2513_d_b5, eq200_e2513_d_b6, eq200_e2513_d_b7, eq200_e2513_d_b8, eq200_e2513_d_b9, eq200_e2513_d_b10, eq200_e2513_d_b11, eq200_e2513_d_b12, eq200_e2513_d_b13, eq200_e2513_d_b14, eq200_e2513_d_b15, eq200_e2513_d_b16, eq200_e2513_d_b17, eq200_e2513_d_b18, eq200_e2513_d_b19, eq200_e2513_d_b20, eq200_e2513_d_b21, eq200_e2513_d_b22, eq200_e2513_d_b23, eq200_e2513_d_b24, eq200_e2513_d_b25, eq200_e2513_d_b26, eq200_e2513_d_b27, eq200_e2513_d_b28, eq200_e2513_d_b29, eq200_e2513_d_b30, eq200_e2513_d_b31, eq200_e2513_d_b32, eq200_e2513_d_b33, eq200_e2513_d_b34, eq200_e2513_d_b35, eq200_e2513_d_b36, eq200_e2513_d_b37, eq200_e2513_d_b38, eq200_e2513_d_b39, eq200_e2513_d_b40, eq200_e2513_d_b41, eq200_e2513_d_b42, eq200_e2513_d_b43, eq200_e2513_d_b44, eq200_e2513_d_b45, eq200_e2513_d_b46, eq200_e2513_d_b47, eq200_e2513_d_b48, eq200_e2513_d_b49, eq200_e2513_d_b50, eq200_e2513_d_b51, eq200_e2513_d_b52, eq200_e2513_d_b53, eq200_e2513_d_b54,) = {
    if (((!s.b[600]) && s.b[603]) && s.b[604]) {
        let eq200_e2508: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 99, s.v[300]);
        let eq200_e2509: f64 = (p.p7 * eq200_e2508);
        let eq200_e2511: f64 = (eq200_e2509 * p.p249);
        let eq200_e2511_d_n0: f64 = (__rspice_deriv_cse_0 * p.p249);
        let eq200_e2511_d_n1: f64 = (__rspice_deriv_cse_1 * p.p249);
        let eq200_e2511_d_n2: f64 = (__rspice_deriv_cse_2 * p.p249);
        let eq200_e2511_d_n3: f64 = (__rspice_deriv_cse_3 * p.p249);
        let eq200_e2511_d_n4: f64 = (__rspice_deriv_cse_4 * p.p249);
        let eq200_e2511_d_n5: f64 = (__rspice_deriv_cse_5 * p.p249);
        let eq200_e2511_d_n6: f64 = (__rspice_deriv_cse_6 * p.p249);
        let eq200_e2511_d_n7: f64 = (__rspice_deriv_cse_7 * p.p249);
        let eq200_e2511_d_n8: f64 = (__rspice_deriv_cse_8 * p.p249);
        let eq200_e2511_d_n9: f64 = (__rspice_deriv_cse_9 * p.p249);
        let eq200_e2511_d_n10: f64 = (__rspice_deriv_cse_10 * p.p249);
        let eq200_e2511_d_n11: f64 = (__rspice_deriv_cse_11 * p.p249);
        let eq200_e2511_d_n12: f64 = (__rspice_deriv_cse_12 * p.p249);
        let eq200_e2511_d_n13: f64 = (__rspice_deriv_cse_13 * p.p249);
        let eq200_e2511_d_n14: f64 = (__rspice_deriv_cse_14 * p.p249);
        let eq200_e2511_d_n15: f64 = (__rspice_deriv_cse_15 * p.p249);
        let eq200_e2511_d_n16: f64 = (__rspice_deriv_cse_16 * p.p249);
        let eq200_e2511_d_n17: f64 = (__rspice_deriv_cse_17 * p.p249);
        let eq200_e2511_d_n18: f64 = (__rspice_deriv_cse_18 * p.p249);
        let eq200_e2511_d_n19: f64 = (__rspice_deriv_cse_19 * p.p249);
        let eq200_e2511_d_n20: f64 = (__rspice_deriv_cse_20 * p.p249);
        let eq200_e2511_d_n21: f64 = (__rspice_deriv_cse_21 * p.p249);
        let eq200_e2511_d_n22: f64 = (__rspice_deriv_cse_22 * p.p249);
        let eq200_e2511_d_b0: f64 = (__rspice_deriv_cse_23 * p.p249);
        let eq200_e2511_d_b1: f64 = (__rspice_deriv_cse_24 * p.p249);
        let eq200_e2511_d_b2: f64 = (__rspice_deriv_cse_25 * p.p249);
        let eq200_e2511_d_b3: f64 = (__rspice_deriv_cse_26 * p.p249);
        let eq200_e2511_d_b4: f64 = (__rspice_deriv_cse_27 * p.p249);
        let eq200_e2511_d_b5: f64 = (__rspice_deriv_cse_28 * p.p249);
        let eq200_e2511_d_b6: f64 = (__rspice_deriv_cse_29 * p.p249);
        let eq200_e2511_d_b7: f64 = (__rspice_deriv_cse_30 * p.p249);
        let eq200_e2511_d_b8: f64 = (__rspice_deriv_cse_31 * p.p249);
        let eq200_e2511_d_b9: f64 = (__rspice_deriv_cse_32 * p.p249);
        let eq200_e2511_d_b10: f64 = (__rspice_deriv_cse_33 * p.p249);
        let eq200_e2511_d_b11: f64 = (__rspice_deriv_cse_34 * p.p249);
        let eq200_e2511_d_b12: f64 = (__rspice_deriv_cse_35 * p.p249);
        let eq200_e2511_d_b13: f64 = (__rspice_deriv_cse_36 * p.p249);
        let eq200_e2511_d_b14: f64 = (__rspice_deriv_cse_37 * p.p249);
        let eq200_e2511_d_b15: f64 = (__rspice_deriv_cse_38 * p.p249);
        let eq200_e2511_d_b16: f64 = (__rspice_deriv_cse_39 * p.p249);
        let eq200_e2511_d_b17: f64 = (__rspice_deriv_cse_40 * p.p249);
        let eq200_e2511_d_b18: f64 = (__rspice_deriv_cse_41 * p.p249);
        let eq200_e2511_d_b19: f64 = (__rspice_deriv_cse_42 * p.p249);
        let eq200_e2511_d_b20: f64 = (__rspice_deriv_cse_43 * p.p249);
        let eq200_e2511_d_b21: f64 = (__rspice_deriv_cse_44 * p.p249);
        let eq200_e2511_d_b22: f64 = (__rspice_deriv_cse_45 * p.p249);
        let eq200_e2511_d_b23: f64 = (__rspice_deriv_cse_46 * p.p249);
        let eq200_e2511_d_b24: f64 = (__rspice_deriv_cse_47 * p.p249);
        let eq200_e2511_d_b25: f64 = (__rspice_deriv_cse_48 * p.p249);
        let eq200_e2511_d_b26: f64 = (__rspice_deriv_cse_49 * p.p249);
        let eq200_e2511_d_b27: f64 = (__rspice_deriv_cse_50 * p.p249);
        let eq200_e2511_d_b28: f64 = (__rspice_deriv_cse_51 * p.p249);
        let eq200_e2511_d_b29: f64 = (__rspice_deriv_cse_52 * p.p249);
        let eq200_e2511_d_b30: f64 = (__rspice_deriv_cse_53 * p.p249);
        let eq200_e2511_d_b31: f64 = (__rspice_deriv_cse_54 * p.p249);
        let eq200_e2511_d_b32: f64 = (__rspice_deriv_cse_55 * p.p249);
        let eq200_e2511_d_b33: f64 = (__rspice_deriv_cse_56 * p.p249);
        let eq200_e2511_d_b34: f64 = (__rspice_deriv_cse_57 * p.p249);
        let eq200_e2511_d_b35: f64 = (__rspice_deriv_cse_58 * p.p249);
        let eq200_e2511_d_b36: f64 = (__rspice_deriv_cse_59 * p.p249);
        let eq200_e2511_d_b37: f64 = (__rspice_deriv_cse_60 * p.p249);
        let eq200_e2511_d_b38: f64 = (__rspice_deriv_cse_61 * p.p249);
        let eq200_e2511_d_b39: f64 = (__rspice_deriv_cse_62 * p.p249);
        let eq200_e2511_d_b40: f64 = (__rspice_deriv_cse_63 * p.p249);
        let eq200_e2511_d_b41: f64 = (__rspice_deriv_cse_64 * p.p249);
        let eq200_e2511_d_b42: f64 = (__rspice_deriv_cse_65 * p.p249);
        let eq200_e2511_d_b43: f64 = (__rspice_deriv_cse_66 * p.p249);
        let eq200_e2511_d_b44: f64 = (__rspice_deriv_cse_67 * p.p249);
        let eq200_e2511_d_b45: f64 = (__rspice_deriv_cse_68 * p.p249);
        let eq200_e2511_d_b46: f64 = (__rspice_deriv_cse_69 * p.p249);
        let eq200_e2511_d_b47: f64 = (__rspice_deriv_cse_70 * p.p249);
        let eq200_e2511_d_b48: f64 = (__rspice_deriv_cse_71 * p.p249);
        let eq200_e2511_d_b49: f64 = (__rspice_deriv_cse_72 * p.p249);
        let eq200_e2511_d_b50: f64 = (__rspice_deriv_cse_73 * p.p249);
        let eq200_e2511_d_b51: f64 = (__rspice_deriv_cse_74 * p.p249);
        let eq200_e2511_d_b52: f64 = (__rspice_deriv_cse_75 * p.p249);
        let eq200_e2511_d_b53: f64 = (__rspice_deriv_cse_76 * p.p249);
        let eq200_e2511_d_b54: f64 = (__rspice_deriv_cse_77 * p.p249);
        (eq200_e2511, eq200_e2511_d_n0, eq200_e2511_d_n1, eq200_e2511_d_n2, eq200_e2511_d_n3, eq200_e2511_d_n4, eq200_e2511_d_n5, eq200_e2511_d_n6, eq200_e2511_d_n7, eq200_e2511_d_n8, eq200_e2511_d_n9, eq200_e2511_d_n10, eq200_e2511_d_n11, eq200_e2511_d_n12, eq200_e2511_d_n13, eq200_e2511_d_n14, eq200_e2511_d_n15, eq200_e2511_d_n16, eq200_e2511_d_n17, eq200_e2511_d_n18, eq200_e2511_d_n19, eq200_e2511_d_n20, eq200_e2511_d_n21, eq200_e2511_d_n22, eq200_e2511_d_b0, eq200_e2511_d_b1, eq200_e2511_d_b2, eq200_e2511_d_b3, eq200_e2511_d_b4, eq200_e2511_d_b5, eq200_e2511_d_b6, eq200_e2511_d_b7, eq200_e2511_d_b8, eq200_e2511_d_b9, eq200_e2511_d_b10, eq200_e2511_d_b11, eq200_e2511_d_b12, eq200_e2511_d_b13, eq200_e2511_d_b14, eq200_e2511_d_b15, eq200_e2511_d_b16, eq200_e2511_d_b17, eq200_e2511_d_b18, eq200_e2511_d_b19, eq200_e2511_d_b20, eq200_e2511_d_b21, eq200_e2511_d_b22, eq200_e2511_d_b23, eq200_e2511_d_b24, eq200_e2511_d_b25, eq200_e2511_d_b26, eq200_e2511_d_b27, eq200_e2511_d_b28, eq200_e2511_d_b29, eq200_e2511_d_b30, eq200_e2511_d_b31, eq200_e2511_d_b32, eq200_e2511_d_b33, eq200_e2511_d_b34, eq200_e2511_d_b35, eq200_e2511_d_b36, eq200_e2511_d_b37, eq200_e2511_d_b38, eq200_e2511_d_b39, eq200_e2511_d_b40, eq200_e2511_d_b41, eq200_e2511_d_b42, eq200_e2511_d_b43, eq200_e2511_d_b44, eq200_e2511_d_b45, eq200_e2511_d_b46, eq200_e2511_d_b47, eq200_e2511_d_b48, eq200_e2511_d_b49, eq200_e2511_d_b50, eq200_e2511_d_b51, eq200_e2511_d_b52, eq200_e2511_d_b53, eq200_e2511_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq200_value: f64 = eq200_e2513;
        let eq200_node_derivatives: [f64; 23] = [eq200_e2513_d_n0, eq200_e2513_d_n1, eq200_e2513_d_n2, eq200_e2513_d_n3, eq200_e2513_d_n4, eq200_e2513_d_n5, eq200_e2513_d_n6, eq200_e2513_d_n7, eq200_e2513_d_n8, eq200_e2513_d_n9, eq200_e2513_d_n10, eq200_e2513_d_n11, eq200_e2513_d_n12, eq200_e2513_d_n13, eq200_e2513_d_n14, eq200_e2513_d_n15, eq200_e2513_d_n16, eq200_e2513_d_n17, eq200_e2513_d_n18, eq200_e2513_d_n19, eq200_e2513_d_n20, eq200_e2513_d_n21, eq200_e2513_d_n22];
        let eq200_branch_derivatives: [f64; 55] = [eq200_e2513_d_b0, eq200_e2513_d_b1, eq200_e2513_d_b2, eq200_e2513_d_b3, eq200_e2513_d_b4, eq200_e2513_d_b5, eq200_e2513_d_b6, eq200_e2513_d_b7, eq200_e2513_d_b8, eq200_e2513_d_b9, eq200_e2513_d_b10, eq200_e2513_d_b11, eq200_e2513_d_b12, eq200_e2513_d_b13, eq200_e2513_d_b14, eq200_e2513_d_b15, eq200_e2513_d_b16, eq200_e2513_d_b17, eq200_e2513_d_b18, eq200_e2513_d_b19, eq200_e2513_d_b20, eq200_e2513_d_b21, eq200_e2513_d_b22, eq200_e2513_d_b23, eq200_e2513_d_b24, eq200_e2513_d_b25, eq200_e2513_d_b26, eq200_e2513_d_b27, eq200_e2513_d_b28, eq200_e2513_d_b29, eq200_e2513_d_b30, eq200_e2513_d_b31, eq200_e2513_d_b32, eq200_e2513_d_b33, eq200_e2513_d_b34, eq200_e2513_d_b35, eq200_e2513_d_b36, eq200_e2513_d_b37, eq200_e2513_d_b38, eq200_e2513_d_b39, eq200_e2513_d_b40, eq200_e2513_d_b41, eq200_e2513_d_b42, eq200_e2513_d_b43, eq200_e2513_d_b44, eq200_e2513_d_b45, eq200_e2513_d_b46, eq200_e2513_d_b47, eq200_e2513_d_b48, eq200_e2513_d_b49, eq200_e2513_d_b50, eq200_e2513_d_b51, eq200_e2513_d_b52, eq200_e2513_d_b53, eq200_e2513_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq200_value),
            &eq200_node_derivatives,
            &eq200_branch_derivatives,
            multiplicity,
        );
        let (eq201_e2526, eq201_e2526_d_n0, eq201_e2526_d_n1, eq201_e2526_d_n2, eq201_e2526_d_n3, eq201_e2526_d_n4, eq201_e2526_d_n5, eq201_e2526_d_n6, eq201_e2526_d_n7, eq201_e2526_d_n8, eq201_e2526_d_n9, eq201_e2526_d_n10, eq201_e2526_d_n11, eq201_e2526_d_n12, eq201_e2526_d_n13, eq201_e2526_d_n14, eq201_e2526_d_n15, eq201_e2526_d_n16, eq201_e2526_d_n17, eq201_e2526_d_n18, eq201_e2526_d_n19, eq201_e2526_d_n20, eq201_e2526_d_n21, eq201_e2526_d_n22, eq201_e2526_d_b0, eq201_e2526_d_b1, eq201_e2526_d_b2, eq201_e2526_d_b3, eq201_e2526_d_b4, eq201_e2526_d_b5, eq201_e2526_d_b6, eq201_e2526_d_b7, eq201_e2526_d_b8, eq201_e2526_d_b9, eq201_e2526_d_b10, eq201_e2526_d_b11, eq201_e2526_d_b12, eq201_e2526_d_b13, eq201_e2526_d_b14, eq201_e2526_d_b15, eq201_e2526_d_b16, eq201_e2526_d_b17, eq201_e2526_d_b18, eq201_e2526_d_b19, eq201_e2526_d_b20, eq201_e2526_d_b21, eq201_e2526_d_b22, eq201_e2526_d_b23, eq201_e2526_d_b24, eq201_e2526_d_b25, eq201_e2526_d_b26, eq201_e2526_d_b27, eq201_e2526_d_b28, eq201_e2526_d_b29, eq201_e2526_d_b30, eq201_e2526_d_b31, eq201_e2526_d_b32, eq201_e2526_d_b33, eq201_e2526_d_b34, eq201_e2526_d_b35, eq201_e2526_d_b36, eq201_e2526_d_b37, eq201_e2526_d_b38, eq201_e2526_d_b39, eq201_e2526_d_b40, eq201_e2526_d_b41, eq201_e2526_d_b42, eq201_e2526_d_b43, eq201_e2526_d_b44, eq201_e2526_d_b45, eq201_e2526_d_b46, eq201_e2526_d_b47, eq201_e2526_d_b48, eq201_e2526_d_b49, eq201_e2526_d_b50, eq201_e2526_d_b51, eq201_e2526_d_b52, eq201_e2526_d_b53, eq201_e2526_d_b54,) = {
    if (((!s.b[600]) && s.b[603]) && (!s.b[604])) {
        let eq201_e2523: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 100, s.v[300]);
        let eq201_e2524: f64 = (p.p7 * eq201_e2523);
        (eq201_e2524, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq201_value: f64 = eq201_e2526;
        let eq201_node_derivatives: [f64; 23] = [eq201_e2526_d_n0, eq201_e2526_d_n1, eq201_e2526_d_n2, eq201_e2526_d_n3, eq201_e2526_d_n4, eq201_e2526_d_n5, eq201_e2526_d_n6, eq201_e2526_d_n7, eq201_e2526_d_n8, eq201_e2526_d_n9, eq201_e2526_d_n10, eq201_e2526_d_n11, eq201_e2526_d_n12, eq201_e2526_d_n13, eq201_e2526_d_n14, eq201_e2526_d_n15, eq201_e2526_d_n16, eq201_e2526_d_n17, eq201_e2526_d_n18, eq201_e2526_d_n19, eq201_e2526_d_n20, eq201_e2526_d_n21, eq201_e2526_d_n22];
        let eq201_branch_derivatives: [f64; 55] = [eq201_e2526_d_b0, eq201_e2526_d_b1, eq201_e2526_d_b2, eq201_e2526_d_b3, eq201_e2526_d_b4, eq201_e2526_d_b5, eq201_e2526_d_b6, eq201_e2526_d_b7, eq201_e2526_d_b8, eq201_e2526_d_b9, eq201_e2526_d_b10, eq201_e2526_d_b11, eq201_e2526_d_b12, eq201_e2526_d_b13, eq201_e2526_d_b14, eq201_e2526_d_b15, eq201_e2526_d_b16, eq201_e2526_d_b17, eq201_e2526_d_b18, eq201_e2526_d_b19, eq201_e2526_d_b20, eq201_e2526_d_b21, eq201_e2526_d_b22, eq201_e2526_d_b23, eq201_e2526_d_b24, eq201_e2526_d_b25, eq201_e2526_d_b26, eq201_e2526_d_b27, eq201_e2526_d_b28, eq201_e2526_d_b29, eq201_e2526_d_b30, eq201_e2526_d_b31, eq201_e2526_d_b32, eq201_e2526_d_b33, eq201_e2526_d_b34, eq201_e2526_d_b35, eq201_e2526_d_b36, eq201_e2526_d_b37, eq201_e2526_d_b38, eq201_e2526_d_b39, eq201_e2526_d_b40, eq201_e2526_d_b41, eq201_e2526_d_b42, eq201_e2526_d_b43, eq201_e2526_d_b44, eq201_e2526_d_b45, eq201_e2526_d_b46, eq201_e2526_d_b47, eq201_e2526_d_b48, eq201_e2526_d_b49, eq201_e2526_d_b50, eq201_e2526_d_b51, eq201_e2526_d_b52, eq201_e2526_d_b53, eq201_e2526_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq201_value),
            &eq201_node_derivatives,
            &eq201_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_42(
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
        let (eq202_e2541, eq202_e2541_d_n0, eq202_e2541_d_n1, eq202_e2541_d_n2, eq202_e2541_d_n3, eq202_e2541_d_n4, eq202_e2541_d_n5, eq202_e2541_d_n6, eq202_e2541_d_n7, eq202_e2541_d_n8, eq202_e2541_d_n9, eq202_e2541_d_n10, eq202_e2541_d_n11, eq202_e2541_d_n12, eq202_e2541_d_n13, eq202_e2541_d_n14, eq202_e2541_d_n15, eq202_e2541_d_n16, eq202_e2541_d_n17, eq202_e2541_d_n18, eq202_e2541_d_n19, eq202_e2541_d_n20, eq202_e2541_d_n21, eq202_e2541_d_n22, eq202_e2541_d_b0, eq202_e2541_d_b1, eq202_e2541_d_b2, eq202_e2541_d_b3, eq202_e2541_d_b4, eq202_e2541_d_b5, eq202_e2541_d_b6, eq202_e2541_d_b7, eq202_e2541_d_b8, eq202_e2541_d_b9, eq202_e2541_d_b10, eq202_e2541_d_b11, eq202_e2541_d_b12, eq202_e2541_d_b13, eq202_e2541_d_b14, eq202_e2541_d_b15, eq202_e2541_d_b16, eq202_e2541_d_b17, eq202_e2541_d_b18, eq202_e2541_d_b19, eq202_e2541_d_b20, eq202_e2541_d_b21, eq202_e2541_d_b22, eq202_e2541_d_b23, eq202_e2541_d_b24, eq202_e2541_d_b25, eq202_e2541_d_b26, eq202_e2541_d_b27, eq202_e2541_d_b28, eq202_e2541_d_b29, eq202_e2541_d_b30, eq202_e2541_d_b31, eq202_e2541_d_b32, eq202_e2541_d_b33, eq202_e2541_d_b34, eq202_e2541_d_b35, eq202_e2541_d_b36, eq202_e2541_d_b37, eq202_e2541_d_b38, eq202_e2541_d_b39, eq202_e2541_d_b40, eq202_e2541_d_b41, eq202_e2541_d_b42, eq202_e2541_d_b43, eq202_e2541_d_b44, eq202_e2541_d_b45, eq202_e2541_d_b46, eq202_e2541_d_b47, eq202_e2541_d_b48, eq202_e2541_d_b49, eq202_e2541_d_b50, eq202_e2541_d_b51, eq202_e2541_d_b52, eq202_e2541_d_b53, eq202_e2541_d_b54,) = {
    if (((!s.b[600]) && s.b[603]) && (!s.b[604])) {
        let eq202_e2536: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 101, s.v[300]);
        let eq202_e2537: f64 = (p.p7 * eq202_e2536);
        let eq202_e2537_d_n0: f64 = (p.p7 * (s.dn[300][0] * ddt_scale));
        let eq202_e2537_d_n1: f64 = (p.p7 * (s.dn[300][1] * ddt_scale));
        let eq202_e2537_d_n2: f64 = (p.p7 * (s.dn[300][2] * ddt_scale));
        let eq202_e2537_d_n3: f64 = (p.p7 * (s.dn[300][3] * ddt_scale));
        let eq202_e2537_d_n4: f64 = (p.p7 * (s.dn[300][4] * ddt_scale));
        let eq202_e2537_d_n5: f64 = (p.p7 * (s.dn[300][5] * ddt_scale));
        let eq202_e2537_d_n6: f64 = (p.p7 * (s.dn[300][6] * ddt_scale));
        let eq202_e2537_d_n7: f64 = (p.p7 * (s.dn[300][7] * ddt_scale));
        let eq202_e2537_d_n8: f64 = (p.p7 * (s.dn[300][8] * ddt_scale));
        let eq202_e2537_d_n9: f64 = (p.p7 * (s.dn[300][9] * ddt_scale));
        let eq202_e2537_d_n10: f64 = (p.p7 * (s.dn[300][10] * ddt_scale));
        let eq202_e2537_d_n11: f64 = (p.p7 * (s.dn[300][11] * ddt_scale));
        let eq202_e2537_d_n12: f64 = (p.p7 * (s.dn[300][12] * ddt_scale));
        let eq202_e2537_d_n13: f64 = (p.p7 * (s.dn[300][13] * ddt_scale));
        let eq202_e2537_d_n14: f64 = (p.p7 * (s.dn[300][14] * ddt_scale));
        let eq202_e2537_d_n15: f64 = (p.p7 * (s.dn[300][15] * ddt_scale));
        let eq202_e2537_d_n16: f64 = (p.p7 * (s.dn[300][16] * ddt_scale));
        let eq202_e2537_d_n17: f64 = (p.p7 * (s.dn[300][17] * ddt_scale));
        let eq202_e2537_d_n18: f64 = (p.p7 * (s.dn[300][18] * ddt_scale));
        let eq202_e2537_d_n19: f64 = (p.p7 * (s.dn[300][19] * ddt_scale));
        let eq202_e2537_d_n20: f64 = (p.p7 * (s.dn[300][20] * ddt_scale));
        let eq202_e2537_d_n21: f64 = (p.p7 * (s.dn[300][21] * ddt_scale));
        let eq202_e2537_d_n22: f64 = (p.p7 * (s.dn[300][22] * ddt_scale));
        let eq202_e2537_d_b0: f64 = (p.p7 * (s.db[300][0] * ddt_scale));
        let eq202_e2537_d_b1: f64 = (p.p7 * (s.db[300][1] * ddt_scale));
        let eq202_e2537_d_b2: f64 = (p.p7 * (s.db[300][2] * ddt_scale));
        let eq202_e2537_d_b3: f64 = (p.p7 * (s.db[300][3] * ddt_scale));
        let eq202_e2537_d_b4: f64 = (p.p7 * (s.db[300][4] * ddt_scale));
        let eq202_e2537_d_b5: f64 = (p.p7 * (s.db[300][5] * ddt_scale));
        let eq202_e2537_d_b6: f64 = (p.p7 * (s.db[300][6] * ddt_scale));
        let eq202_e2537_d_b7: f64 = (p.p7 * (s.db[300][7] * ddt_scale));
        let eq202_e2537_d_b8: f64 = (p.p7 * (s.db[300][8] * ddt_scale));
        let eq202_e2537_d_b9: f64 = (p.p7 * (s.db[300][9] * ddt_scale));
        let eq202_e2537_d_b10: f64 = (p.p7 * (s.db[300][10] * ddt_scale));
        let eq202_e2537_d_b11: f64 = (p.p7 * (s.db[300][11] * ddt_scale));
        let eq202_e2537_d_b12: f64 = (p.p7 * (s.db[300][12] * ddt_scale));
        let eq202_e2537_d_b13: f64 = (p.p7 * (s.db[300][13] * ddt_scale));
        let eq202_e2537_d_b14: f64 = (p.p7 * (s.db[300][14] * ddt_scale));
        let eq202_e2537_d_b15: f64 = (p.p7 * (s.db[300][15] * ddt_scale));
        let eq202_e2537_d_b16: f64 = (p.p7 * (s.db[300][16] * ddt_scale));
        let eq202_e2537_d_b17: f64 = (p.p7 * (s.db[300][17] * ddt_scale));
        let eq202_e2537_d_b18: f64 = (p.p7 * (s.db[300][18] * ddt_scale));
        let eq202_e2537_d_b19: f64 = (p.p7 * (s.db[300][19] * ddt_scale));
        let eq202_e2537_d_b20: f64 = (p.p7 * (s.db[300][20] * ddt_scale));
        let eq202_e2537_d_b21: f64 = (p.p7 * (s.db[300][21] * ddt_scale));
        let eq202_e2537_d_b22: f64 = (p.p7 * (s.db[300][22] * ddt_scale));
        let eq202_e2537_d_b23: f64 = (p.p7 * (s.db[300][23] * ddt_scale));
        let eq202_e2537_d_b24: f64 = (p.p7 * (s.db[300][24] * ddt_scale));
        let eq202_e2537_d_b25: f64 = (p.p7 * (s.db[300][25] * ddt_scale));
        let eq202_e2537_d_b26: f64 = (p.p7 * (s.db[300][26] * ddt_scale));
        let eq202_e2537_d_b27: f64 = (p.p7 * (s.db[300][27] * ddt_scale));
        let eq202_e2537_d_b28: f64 = (p.p7 * (s.db[300][28] * ddt_scale));
        let eq202_e2537_d_b29: f64 = (p.p7 * (s.db[300][29] * ddt_scale));
        let eq202_e2537_d_b30: f64 = (p.p7 * (s.db[300][30] * ddt_scale));
        let eq202_e2537_d_b31: f64 = (p.p7 * (s.db[300][31] * ddt_scale));
        let eq202_e2537_d_b32: f64 = (p.p7 * (s.db[300][32] * ddt_scale));
        let eq202_e2537_d_b33: f64 = (p.p7 * (s.db[300][33] * ddt_scale));
        let eq202_e2537_d_b34: f64 = (p.p7 * (s.db[300][34] * ddt_scale));
        let eq202_e2537_d_b35: f64 = (p.p7 * (s.db[300][35] * ddt_scale));
        let eq202_e2537_d_b36: f64 = (p.p7 * (s.db[300][36] * ddt_scale));
        let eq202_e2537_d_b37: f64 = (p.p7 * (s.db[300][37] * ddt_scale));
        let eq202_e2537_d_b38: f64 = (p.p7 * (s.db[300][38] * ddt_scale));
        let eq202_e2537_d_b39: f64 = (p.p7 * (s.db[300][39] * ddt_scale));
        let eq202_e2537_d_b40: f64 = (p.p7 * (s.db[300][40] * ddt_scale));
        let eq202_e2537_d_b41: f64 = (p.p7 * (s.db[300][41] * ddt_scale));
        let eq202_e2537_d_b42: f64 = (p.p7 * (s.db[300][42] * ddt_scale));
        let eq202_e2537_d_b43: f64 = (p.p7 * (s.db[300][43] * ddt_scale));
        let eq202_e2537_d_b44: f64 = (p.p7 * (s.db[300][44] * ddt_scale));
        let eq202_e2537_d_b45: f64 = (p.p7 * (s.db[300][45] * ddt_scale));
        let eq202_e2537_d_b46: f64 = (p.p7 * (s.db[300][46] * ddt_scale));
        let eq202_e2537_d_b47: f64 = (p.p7 * (s.db[300][47] * ddt_scale));
        let eq202_e2537_d_b48: f64 = (p.p7 * (s.db[300][48] * ddt_scale));
        let eq202_e2537_d_b49: f64 = (p.p7 * (s.db[300][49] * ddt_scale));
        let eq202_e2537_d_b50: f64 = (p.p7 * (s.db[300][50] * ddt_scale));
        let eq202_e2537_d_b51: f64 = (p.p7 * (s.db[300][51] * ddt_scale));
        let eq202_e2537_d_b52: f64 = (p.p7 * (s.db[300][52] * ddt_scale));
        let eq202_e2537_d_b53: f64 = (p.p7 * (s.db[300][53] * ddt_scale));
        let eq202_e2537_d_b54: f64 = (p.p7 * (s.db[300][54] * ddt_scale));
        let eq202_e2539: f64 = (eq202_e2537 * p.p249);
        let eq202_e2539_d_n0: f64 = (eq202_e2537_d_n0 * p.p249);
        let eq202_e2539_d_n1: f64 = (eq202_e2537_d_n1 * p.p249);
        let eq202_e2539_d_n2: f64 = (eq202_e2537_d_n2 * p.p249);
        let eq202_e2539_d_n3: f64 = (eq202_e2537_d_n3 * p.p249);
        let eq202_e2539_d_n4: f64 = (eq202_e2537_d_n4 * p.p249);
        let eq202_e2539_d_n5: f64 = (eq202_e2537_d_n5 * p.p249);
        let eq202_e2539_d_n6: f64 = (eq202_e2537_d_n6 * p.p249);
        let eq202_e2539_d_n7: f64 = (eq202_e2537_d_n7 * p.p249);
        let eq202_e2539_d_n8: f64 = (eq202_e2537_d_n8 * p.p249);
        let eq202_e2539_d_n9: f64 = (eq202_e2537_d_n9 * p.p249);
        let eq202_e2539_d_n10: f64 = (eq202_e2537_d_n10 * p.p249);
        let eq202_e2539_d_n11: f64 = (eq202_e2537_d_n11 * p.p249);
        let eq202_e2539_d_n12: f64 = (eq202_e2537_d_n12 * p.p249);
        let eq202_e2539_d_n13: f64 = (eq202_e2537_d_n13 * p.p249);
        let eq202_e2539_d_n14: f64 = (eq202_e2537_d_n14 * p.p249);
        let eq202_e2539_d_n15: f64 = (eq202_e2537_d_n15 * p.p249);
        let eq202_e2539_d_n16: f64 = (eq202_e2537_d_n16 * p.p249);
        let eq202_e2539_d_n17: f64 = (eq202_e2537_d_n17 * p.p249);
        let eq202_e2539_d_n18: f64 = (eq202_e2537_d_n18 * p.p249);
        let eq202_e2539_d_n19: f64 = (eq202_e2537_d_n19 * p.p249);
        let eq202_e2539_d_n20: f64 = (eq202_e2537_d_n20 * p.p249);
        let eq202_e2539_d_n21: f64 = (eq202_e2537_d_n21 * p.p249);
        let eq202_e2539_d_n22: f64 = (eq202_e2537_d_n22 * p.p249);
        let eq202_e2539_d_b0: f64 = (eq202_e2537_d_b0 * p.p249);
        let eq202_e2539_d_b1: f64 = (eq202_e2537_d_b1 * p.p249);
        let eq202_e2539_d_b2: f64 = (eq202_e2537_d_b2 * p.p249);
        let eq202_e2539_d_b3: f64 = (eq202_e2537_d_b3 * p.p249);
        let eq202_e2539_d_b4: f64 = (eq202_e2537_d_b4 * p.p249);
        let eq202_e2539_d_b5: f64 = (eq202_e2537_d_b5 * p.p249);
        let eq202_e2539_d_b6: f64 = (eq202_e2537_d_b6 * p.p249);
        let eq202_e2539_d_b7: f64 = (eq202_e2537_d_b7 * p.p249);
        let eq202_e2539_d_b8: f64 = (eq202_e2537_d_b8 * p.p249);
        let eq202_e2539_d_b9: f64 = (eq202_e2537_d_b9 * p.p249);
        let eq202_e2539_d_b10: f64 = (eq202_e2537_d_b10 * p.p249);
        let eq202_e2539_d_b11: f64 = (eq202_e2537_d_b11 * p.p249);
        let eq202_e2539_d_b12: f64 = (eq202_e2537_d_b12 * p.p249);
        let eq202_e2539_d_b13: f64 = (eq202_e2537_d_b13 * p.p249);
        let eq202_e2539_d_b14: f64 = (eq202_e2537_d_b14 * p.p249);
        let eq202_e2539_d_b15: f64 = (eq202_e2537_d_b15 * p.p249);
        let eq202_e2539_d_b16: f64 = (eq202_e2537_d_b16 * p.p249);
        let eq202_e2539_d_b17: f64 = (eq202_e2537_d_b17 * p.p249);
        let eq202_e2539_d_b18: f64 = (eq202_e2537_d_b18 * p.p249);
        let eq202_e2539_d_b19: f64 = (eq202_e2537_d_b19 * p.p249);
        let eq202_e2539_d_b20: f64 = (eq202_e2537_d_b20 * p.p249);
        let eq202_e2539_d_b21: f64 = (eq202_e2537_d_b21 * p.p249);
        let eq202_e2539_d_b22: f64 = (eq202_e2537_d_b22 * p.p249);
        let eq202_e2539_d_b23: f64 = (eq202_e2537_d_b23 * p.p249);
        let eq202_e2539_d_b24: f64 = (eq202_e2537_d_b24 * p.p249);
        let eq202_e2539_d_b25: f64 = (eq202_e2537_d_b25 * p.p249);
        let eq202_e2539_d_b26: f64 = (eq202_e2537_d_b26 * p.p249);
        let eq202_e2539_d_b27: f64 = (eq202_e2537_d_b27 * p.p249);
        let eq202_e2539_d_b28: f64 = (eq202_e2537_d_b28 * p.p249);
        let eq202_e2539_d_b29: f64 = (eq202_e2537_d_b29 * p.p249);
        let eq202_e2539_d_b30: f64 = (eq202_e2537_d_b30 * p.p249);
        let eq202_e2539_d_b31: f64 = (eq202_e2537_d_b31 * p.p249);
        let eq202_e2539_d_b32: f64 = (eq202_e2537_d_b32 * p.p249);
        let eq202_e2539_d_b33: f64 = (eq202_e2537_d_b33 * p.p249);
        let eq202_e2539_d_b34: f64 = (eq202_e2537_d_b34 * p.p249);
        let eq202_e2539_d_b35: f64 = (eq202_e2537_d_b35 * p.p249);
        let eq202_e2539_d_b36: f64 = (eq202_e2537_d_b36 * p.p249);
        let eq202_e2539_d_b37: f64 = (eq202_e2537_d_b37 * p.p249);
        let eq202_e2539_d_b38: f64 = (eq202_e2537_d_b38 * p.p249);
        let eq202_e2539_d_b39: f64 = (eq202_e2537_d_b39 * p.p249);
        let eq202_e2539_d_b40: f64 = (eq202_e2537_d_b40 * p.p249);
        let eq202_e2539_d_b41: f64 = (eq202_e2537_d_b41 * p.p249);
        let eq202_e2539_d_b42: f64 = (eq202_e2537_d_b42 * p.p249);
        let eq202_e2539_d_b43: f64 = (eq202_e2537_d_b43 * p.p249);
        let eq202_e2539_d_b44: f64 = (eq202_e2537_d_b44 * p.p249);
        let eq202_e2539_d_b45: f64 = (eq202_e2537_d_b45 * p.p249);
        let eq202_e2539_d_b46: f64 = (eq202_e2537_d_b46 * p.p249);
        let eq202_e2539_d_b47: f64 = (eq202_e2537_d_b47 * p.p249);
        let eq202_e2539_d_b48: f64 = (eq202_e2537_d_b48 * p.p249);
        let eq202_e2539_d_b49: f64 = (eq202_e2537_d_b49 * p.p249);
        let eq202_e2539_d_b50: f64 = (eq202_e2537_d_b50 * p.p249);
        let eq202_e2539_d_b51: f64 = (eq202_e2537_d_b51 * p.p249);
        let eq202_e2539_d_b52: f64 = (eq202_e2537_d_b52 * p.p249);
        let eq202_e2539_d_b53: f64 = (eq202_e2537_d_b53 * p.p249);
        let eq202_e2539_d_b54: f64 = (eq202_e2537_d_b54 * p.p249);
        (eq202_e2539, eq202_e2539_d_n0, eq202_e2539_d_n1, eq202_e2539_d_n2, eq202_e2539_d_n3, eq202_e2539_d_n4, eq202_e2539_d_n5, eq202_e2539_d_n6, eq202_e2539_d_n7, eq202_e2539_d_n8, eq202_e2539_d_n9, eq202_e2539_d_n10, eq202_e2539_d_n11, eq202_e2539_d_n12, eq202_e2539_d_n13, eq202_e2539_d_n14, eq202_e2539_d_n15, eq202_e2539_d_n16, eq202_e2539_d_n17, eq202_e2539_d_n18, eq202_e2539_d_n19, eq202_e2539_d_n20, eq202_e2539_d_n21, eq202_e2539_d_n22, eq202_e2539_d_b0, eq202_e2539_d_b1, eq202_e2539_d_b2, eq202_e2539_d_b3, eq202_e2539_d_b4, eq202_e2539_d_b5, eq202_e2539_d_b6, eq202_e2539_d_b7, eq202_e2539_d_b8, eq202_e2539_d_b9, eq202_e2539_d_b10, eq202_e2539_d_b11, eq202_e2539_d_b12, eq202_e2539_d_b13, eq202_e2539_d_b14, eq202_e2539_d_b15, eq202_e2539_d_b16, eq202_e2539_d_b17, eq202_e2539_d_b18, eq202_e2539_d_b19, eq202_e2539_d_b20, eq202_e2539_d_b21, eq202_e2539_d_b22, eq202_e2539_d_b23, eq202_e2539_d_b24, eq202_e2539_d_b25, eq202_e2539_d_b26, eq202_e2539_d_b27, eq202_e2539_d_b28, eq202_e2539_d_b29, eq202_e2539_d_b30, eq202_e2539_d_b31, eq202_e2539_d_b32, eq202_e2539_d_b33, eq202_e2539_d_b34, eq202_e2539_d_b35, eq202_e2539_d_b36, eq202_e2539_d_b37, eq202_e2539_d_b38, eq202_e2539_d_b39, eq202_e2539_d_b40, eq202_e2539_d_b41, eq202_e2539_d_b42, eq202_e2539_d_b43, eq202_e2539_d_b44, eq202_e2539_d_b45, eq202_e2539_d_b46, eq202_e2539_d_b47, eq202_e2539_d_b48, eq202_e2539_d_b49, eq202_e2539_d_b50, eq202_e2539_d_b51, eq202_e2539_d_b52, eq202_e2539_d_b53, eq202_e2539_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq202_value: f64 = eq202_e2541;
        let eq202_node_derivatives: [f64; 23] = [eq202_e2541_d_n0, eq202_e2541_d_n1, eq202_e2541_d_n2, eq202_e2541_d_n3, eq202_e2541_d_n4, eq202_e2541_d_n5, eq202_e2541_d_n6, eq202_e2541_d_n7, eq202_e2541_d_n8, eq202_e2541_d_n9, eq202_e2541_d_n10, eq202_e2541_d_n11, eq202_e2541_d_n12, eq202_e2541_d_n13, eq202_e2541_d_n14, eq202_e2541_d_n15, eq202_e2541_d_n16, eq202_e2541_d_n17, eq202_e2541_d_n18, eq202_e2541_d_n19, eq202_e2541_d_n20, eq202_e2541_d_n21, eq202_e2541_d_n22];
        let eq202_branch_derivatives: [f64; 55] = [eq202_e2541_d_b0, eq202_e2541_d_b1, eq202_e2541_d_b2, eq202_e2541_d_b3, eq202_e2541_d_b4, eq202_e2541_d_b5, eq202_e2541_d_b6, eq202_e2541_d_b7, eq202_e2541_d_b8, eq202_e2541_d_b9, eq202_e2541_d_b10, eq202_e2541_d_b11, eq202_e2541_d_b12, eq202_e2541_d_b13, eq202_e2541_d_b14, eq202_e2541_d_b15, eq202_e2541_d_b16, eq202_e2541_d_b17, eq202_e2541_d_b18, eq202_e2541_d_b19, eq202_e2541_d_b20, eq202_e2541_d_b21, eq202_e2541_d_b22, eq202_e2541_d_b23, eq202_e2541_d_b24, eq202_e2541_d_b25, eq202_e2541_d_b26, eq202_e2541_d_b27, eq202_e2541_d_b28, eq202_e2541_d_b29, eq202_e2541_d_b30, eq202_e2541_d_b31, eq202_e2541_d_b32, eq202_e2541_d_b33, eq202_e2541_d_b34, eq202_e2541_d_b35, eq202_e2541_d_b36, eq202_e2541_d_b37, eq202_e2541_d_b38, eq202_e2541_d_b39, eq202_e2541_d_b40, eq202_e2541_d_b41, eq202_e2541_d_b42, eq202_e2541_d_b43, eq202_e2541_d_b44, eq202_e2541_d_b45, eq202_e2541_d_b46, eq202_e2541_d_b47, eq202_e2541_d_b48, eq202_e2541_d_b49, eq202_e2541_d_b50, eq202_e2541_d_b51, eq202_e2541_d_b52, eq202_e2541_d_b53, eq202_e2541_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq202_value),
            &eq202_node_derivatives,
            &eq202_branch_derivatives,
            multiplicity,
        );
        let (eq203_e2553, eq203_e2553_d_n0, eq203_e2553_d_n1, eq203_e2553_d_n2, eq203_e2553_d_n3, eq203_e2553_d_n4, eq203_e2553_d_n5, eq203_e2553_d_n6, eq203_e2553_d_n7, eq203_e2553_d_n8, eq203_e2553_d_n9, eq203_e2553_d_n10, eq203_e2553_d_n11, eq203_e2553_d_n12, eq203_e2553_d_n13, eq203_e2553_d_n14, eq203_e2553_d_n15, eq203_e2553_d_n16, eq203_e2553_d_n17, eq203_e2553_d_n18, eq203_e2553_d_n19, eq203_e2553_d_n20, eq203_e2553_d_n21, eq203_e2553_d_n22, eq203_e2553_d_b0, eq203_e2553_d_b1, eq203_e2553_d_b2, eq203_e2553_d_b3, eq203_e2553_d_b4, eq203_e2553_d_b5, eq203_e2553_d_b6, eq203_e2553_d_b7, eq203_e2553_d_b8, eq203_e2553_d_b9, eq203_e2553_d_b10, eq203_e2553_d_b11, eq203_e2553_d_b12, eq203_e2553_d_b13, eq203_e2553_d_b14, eq203_e2553_d_b15, eq203_e2553_d_b16, eq203_e2553_d_b17, eq203_e2553_d_b18, eq203_e2553_d_b19, eq203_e2553_d_b20, eq203_e2553_d_b21, eq203_e2553_d_b22, eq203_e2553_d_b23, eq203_e2553_d_b24, eq203_e2553_d_b25, eq203_e2553_d_b26, eq203_e2553_d_b27, eq203_e2553_d_b28, eq203_e2553_d_b29, eq203_e2553_d_b30, eq203_e2553_d_b31, eq203_e2553_d_b32, eq203_e2553_d_b33, eq203_e2553_d_b34, eq203_e2553_d_b35, eq203_e2553_d_b36, eq203_e2553_d_b37, eq203_e2553_d_b38, eq203_e2553_d_b39, eq203_e2553_d_b40, eq203_e2553_d_b41, eq203_e2553_d_b42, eq203_e2553_d_b43, eq203_e2553_d_b44, eq203_e2553_d_b45, eq203_e2553_d_b46, eq203_e2553_d_b47, eq203_e2553_d_b48, eq203_e2553_d_b49, eq203_e2553_d_b50, eq203_e2553_d_b51, eq203_e2553_d_b52, eq203_e2553_d_b53, eq203_e2553_d_b54,) = {
    if ((!s.b[600]) && s.b[603]) {
        let eq203_e2549: f64 = (p.p254 * s.v[300]);
        let eq203_e2550: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 102, eq203_e2549);
        let eq203_e2550_d_n0: f64 = ((p.p254 * s.dn[300][0]) * ddt_scale);
        let eq203_e2550_d_n1: f64 = ((p.p254 * s.dn[300][1]) * ddt_scale);
        let eq203_e2550_d_n2: f64 = ((p.p254 * s.dn[300][2]) * ddt_scale);
        let eq203_e2550_d_n3: f64 = ((p.p254 * s.dn[300][3]) * ddt_scale);
        let eq203_e2550_d_n4: f64 = ((p.p254 * s.dn[300][4]) * ddt_scale);
        let eq203_e2550_d_n5: f64 = ((p.p254 * s.dn[300][5]) * ddt_scale);
        let eq203_e2550_d_n6: f64 = ((p.p254 * s.dn[300][6]) * ddt_scale);
        let eq203_e2550_d_n7: f64 = ((p.p254 * s.dn[300][7]) * ddt_scale);
        let eq203_e2550_d_n8: f64 = ((p.p254 * s.dn[300][8]) * ddt_scale);
        let eq203_e2550_d_n9: f64 = ((p.p254 * s.dn[300][9]) * ddt_scale);
        let eq203_e2550_d_n10: f64 = ((p.p254 * s.dn[300][10]) * ddt_scale);
        let eq203_e2550_d_n11: f64 = ((p.p254 * s.dn[300][11]) * ddt_scale);
        let eq203_e2550_d_n12: f64 = ((p.p254 * s.dn[300][12]) * ddt_scale);
        let eq203_e2550_d_n13: f64 = ((p.p254 * s.dn[300][13]) * ddt_scale);
        let eq203_e2550_d_n14: f64 = ((p.p254 * s.dn[300][14]) * ddt_scale);
        let eq203_e2550_d_n15: f64 = ((p.p254 * s.dn[300][15]) * ddt_scale);
        let eq203_e2550_d_n16: f64 = ((p.p254 * s.dn[300][16]) * ddt_scale);
        let eq203_e2550_d_n17: f64 = ((p.p254 * s.dn[300][17]) * ddt_scale);
        let eq203_e2550_d_n18: f64 = ((p.p254 * s.dn[300][18]) * ddt_scale);
        let eq203_e2550_d_n19: f64 = ((p.p254 * s.dn[300][19]) * ddt_scale);
        let eq203_e2550_d_n20: f64 = ((p.p254 * s.dn[300][20]) * ddt_scale);
        let eq203_e2550_d_n21: f64 = ((p.p254 * s.dn[300][21]) * ddt_scale);
        let eq203_e2550_d_n22: f64 = ((p.p254 * s.dn[300][22]) * ddt_scale);
        let eq203_e2550_d_b0: f64 = ((p.p254 * s.db[300][0]) * ddt_scale);
        let eq203_e2550_d_b1: f64 = ((p.p254 * s.db[300][1]) * ddt_scale);
        let eq203_e2550_d_b2: f64 = ((p.p254 * s.db[300][2]) * ddt_scale);
        let eq203_e2550_d_b3: f64 = ((p.p254 * s.db[300][3]) * ddt_scale);
        let eq203_e2550_d_b4: f64 = ((p.p254 * s.db[300][4]) * ddt_scale);
        let eq203_e2550_d_b5: f64 = ((p.p254 * s.db[300][5]) * ddt_scale);
        let eq203_e2550_d_b6: f64 = ((p.p254 * s.db[300][6]) * ddt_scale);
        let eq203_e2550_d_b7: f64 = ((p.p254 * s.db[300][7]) * ddt_scale);
        let eq203_e2550_d_b8: f64 = ((p.p254 * s.db[300][8]) * ddt_scale);
        let eq203_e2550_d_b9: f64 = ((p.p254 * s.db[300][9]) * ddt_scale);
        let eq203_e2550_d_b10: f64 = ((p.p254 * s.db[300][10]) * ddt_scale);
        let eq203_e2550_d_b11: f64 = ((p.p254 * s.db[300][11]) * ddt_scale);
        let eq203_e2550_d_b12: f64 = ((p.p254 * s.db[300][12]) * ddt_scale);
        let eq203_e2550_d_b13: f64 = ((p.p254 * s.db[300][13]) * ddt_scale);
        let eq203_e2550_d_b14: f64 = ((p.p254 * s.db[300][14]) * ddt_scale);
        let eq203_e2550_d_b15: f64 = ((p.p254 * s.db[300][15]) * ddt_scale);
        let eq203_e2550_d_b16: f64 = ((p.p254 * s.db[300][16]) * ddt_scale);
        let eq203_e2550_d_b17: f64 = ((p.p254 * s.db[300][17]) * ddt_scale);
        let eq203_e2550_d_b18: f64 = ((p.p254 * s.db[300][18]) * ddt_scale);
        let eq203_e2550_d_b19: f64 = ((p.p254 * s.db[300][19]) * ddt_scale);
        let eq203_e2550_d_b20: f64 = ((p.p254 * s.db[300][20]) * ddt_scale);
        let eq203_e2550_d_b21: f64 = ((p.p254 * s.db[300][21]) * ddt_scale);
        let eq203_e2550_d_b22: f64 = ((p.p254 * s.db[300][22]) * ddt_scale);
        let eq203_e2550_d_b23: f64 = ((p.p254 * s.db[300][23]) * ddt_scale);
        let eq203_e2550_d_b24: f64 = ((p.p254 * s.db[300][24]) * ddt_scale);
        let eq203_e2550_d_b25: f64 = ((p.p254 * s.db[300][25]) * ddt_scale);
        let eq203_e2550_d_b26: f64 = ((p.p254 * s.db[300][26]) * ddt_scale);
        let eq203_e2550_d_b27: f64 = ((p.p254 * s.db[300][27]) * ddt_scale);
        let eq203_e2550_d_b28: f64 = ((p.p254 * s.db[300][28]) * ddt_scale);
        let eq203_e2550_d_b29: f64 = ((p.p254 * s.db[300][29]) * ddt_scale);
        let eq203_e2550_d_b30: f64 = ((p.p254 * s.db[300][30]) * ddt_scale);
        let eq203_e2550_d_b31: f64 = ((p.p254 * s.db[300][31]) * ddt_scale);
        let eq203_e2550_d_b32: f64 = ((p.p254 * s.db[300][32]) * ddt_scale);
        let eq203_e2550_d_b33: f64 = ((p.p254 * s.db[300][33]) * ddt_scale);
        let eq203_e2550_d_b34: f64 = ((p.p254 * s.db[300][34]) * ddt_scale);
        let eq203_e2550_d_b35: f64 = ((p.p254 * s.db[300][35]) * ddt_scale);
        let eq203_e2550_d_b36: f64 = ((p.p254 * s.db[300][36]) * ddt_scale);
        let eq203_e2550_d_b37: f64 = ((p.p254 * s.db[300][37]) * ddt_scale);
        let eq203_e2550_d_b38: f64 = ((p.p254 * s.db[300][38]) * ddt_scale);
        let eq203_e2550_d_b39: f64 = ((p.p254 * s.db[300][39]) * ddt_scale);
        let eq203_e2550_d_b40: f64 = ((p.p254 * s.db[300][40]) * ddt_scale);
        let eq203_e2550_d_b41: f64 = ((p.p254 * s.db[300][41]) * ddt_scale);
        let eq203_e2550_d_b42: f64 = ((p.p254 * s.db[300][42]) * ddt_scale);
        let eq203_e2550_d_b43: f64 = ((p.p254 * s.db[300][43]) * ddt_scale);
        let eq203_e2550_d_b44: f64 = ((p.p254 * s.db[300][44]) * ddt_scale);
        let eq203_e2550_d_b45: f64 = ((p.p254 * s.db[300][45]) * ddt_scale);
        let eq203_e2550_d_b46: f64 = ((p.p254 * s.db[300][46]) * ddt_scale);
        let eq203_e2550_d_b47: f64 = ((p.p254 * s.db[300][47]) * ddt_scale);
        let eq203_e2550_d_b48: f64 = ((p.p254 * s.db[300][48]) * ddt_scale);
        let eq203_e2550_d_b49: f64 = ((p.p254 * s.db[300][49]) * ddt_scale);
        let eq203_e2550_d_b50: f64 = ((p.p254 * s.db[300][50]) * ddt_scale);
        let eq203_e2550_d_b51: f64 = ((p.p254 * s.db[300][51]) * ddt_scale);
        let eq203_e2550_d_b52: f64 = ((p.p254 * s.db[300][52]) * ddt_scale);
        let eq203_e2550_d_b53: f64 = ((p.p254 * s.db[300][53]) * ddt_scale);
        let eq203_e2550_d_b54: f64 = ((p.p254 * s.db[300][54]) * ddt_scale);
        let eq203_e2551: f64 = (p.p7 * eq203_e2550);
        let eq203_e2551_d_n0: f64 = (p.p7 * eq203_e2550_d_n0);
        let eq203_e2551_d_n1: f64 = (p.p7 * eq203_e2550_d_n1);
        let eq203_e2551_d_n2: f64 = (p.p7 * eq203_e2550_d_n2);
        let eq203_e2551_d_n3: f64 = (p.p7 * eq203_e2550_d_n3);
        let eq203_e2551_d_n4: f64 = (p.p7 * eq203_e2550_d_n4);
        let eq203_e2551_d_n5: f64 = (p.p7 * eq203_e2550_d_n5);
        let eq203_e2551_d_n6: f64 = (p.p7 * eq203_e2550_d_n6);
        let eq203_e2551_d_n7: f64 = (p.p7 * eq203_e2550_d_n7);
        let eq203_e2551_d_n8: f64 = (p.p7 * eq203_e2550_d_n8);
        let eq203_e2551_d_n9: f64 = (p.p7 * eq203_e2550_d_n9);
        let eq203_e2551_d_n10: f64 = (p.p7 * eq203_e2550_d_n10);
        let eq203_e2551_d_n11: f64 = (p.p7 * eq203_e2550_d_n11);
        let eq203_e2551_d_n12: f64 = (p.p7 * eq203_e2550_d_n12);
        let eq203_e2551_d_n13: f64 = (p.p7 * eq203_e2550_d_n13);
        let eq203_e2551_d_n14: f64 = (p.p7 * eq203_e2550_d_n14);
        let eq203_e2551_d_n15: f64 = (p.p7 * eq203_e2550_d_n15);
        let eq203_e2551_d_n16: f64 = (p.p7 * eq203_e2550_d_n16);
        let eq203_e2551_d_n17: f64 = (p.p7 * eq203_e2550_d_n17);
        let eq203_e2551_d_n18: f64 = (p.p7 * eq203_e2550_d_n18);
        let eq203_e2551_d_n19: f64 = (p.p7 * eq203_e2550_d_n19);
        let eq203_e2551_d_n20: f64 = (p.p7 * eq203_e2550_d_n20);
        let eq203_e2551_d_n21: f64 = (p.p7 * eq203_e2550_d_n21);
        let eq203_e2551_d_n22: f64 = (p.p7 * eq203_e2550_d_n22);
        let eq203_e2551_d_b0: f64 = (p.p7 * eq203_e2550_d_b0);
        let eq203_e2551_d_b1: f64 = (p.p7 * eq203_e2550_d_b1);
        let eq203_e2551_d_b2: f64 = (p.p7 * eq203_e2550_d_b2);
        let eq203_e2551_d_b3: f64 = (p.p7 * eq203_e2550_d_b3);
        let eq203_e2551_d_b4: f64 = (p.p7 * eq203_e2550_d_b4);
        let eq203_e2551_d_b5: f64 = (p.p7 * eq203_e2550_d_b5);
        let eq203_e2551_d_b6: f64 = (p.p7 * eq203_e2550_d_b6);
        let eq203_e2551_d_b7: f64 = (p.p7 * eq203_e2550_d_b7);
        let eq203_e2551_d_b8: f64 = (p.p7 * eq203_e2550_d_b8);
        let eq203_e2551_d_b9: f64 = (p.p7 * eq203_e2550_d_b9);
        let eq203_e2551_d_b10: f64 = (p.p7 * eq203_e2550_d_b10);
        let eq203_e2551_d_b11: f64 = (p.p7 * eq203_e2550_d_b11);
        let eq203_e2551_d_b12: f64 = (p.p7 * eq203_e2550_d_b12);
        let eq203_e2551_d_b13: f64 = (p.p7 * eq203_e2550_d_b13);
        let eq203_e2551_d_b14: f64 = (p.p7 * eq203_e2550_d_b14);
        let eq203_e2551_d_b15: f64 = (p.p7 * eq203_e2550_d_b15);
        let eq203_e2551_d_b16: f64 = (p.p7 * eq203_e2550_d_b16);
        let eq203_e2551_d_b17: f64 = (p.p7 * eq203_e2550_d_b17);
        let eq203_e2551_d_b18: f64 = (p.p7 * eq203_e2550_d_b18);
        let eq203_e2551_d_b19: f64 = (p.p7 * eq203_e2550_d_b19);
        let eq203_e2551_d_b20: f64 = (p.p7 * eq203_e2550_d_b20);
        let eq203_e2551_d_b21: f64 = (p.p7 * eq203_e2550_d_b21);
        let eq203_e2551_d_b22: f64 = (p.p7 * eq203_e2550_d_b22);
        let eq203_e2551_d_b23: f64 = (p.p7 * eq203_e2550_d_b23);
        let eq203_e2551_d_b24: f64 = (p.p7 * eq203_e2550_d_b24);
        let eq203_e2551_d_b25: f64 = (p.p7 * eq203_e2550_d_b25);
        let eq203_e2551_d_b26: f64 = (p.p7 * eq203_e2550_d_b26);
        let eq203_e2551_d_b27: f64 = (p.p7 * eq203_e2550_d_b27);
        let eq203_e2551_d_b28: f64 = (p.p7 * eq203_e2550_d_b28);
        let eq203_e2551_d_b29: f64 = (p.p7 * eq203_e2550_d_b29);
        let eq203_e2551_d_b30: f64 = (p.p7 * eq203_e2550_d_b30);
        let eq203_e2551_d_b31: f64 = (p.p7 * eq203_e2550_d_b31);
        let eq203_e2551_d_b32: f64 = (p.p7 * eq203_e2550_d_b32);
        let eq203_e2551_d_b33: f64 = (p.p7 * eq203_e2550_d_b33);
        let eq203_e2551_d_b34: f64 = (p.p7 * eq203_e2550_d_b34);
        let eq203_e2551_d_b35: f64 = (p.p7 * eq203_e2550_d_b35);
        let eq203_e2551_d_b36: f64 = (p.p7 * eq203_e2550_d_b36);
        let eq203_e2551_d_b37: f64 = (p.p7 * eq203_e2550_d_b37);
        let eq203_e2551_d_b38: f64 = (p.p7 * eq203_e2550_d_b38);
        let eq203_e2551_d_b39: f64 = (p.p7 * eq203_e2550_d_b39);
        let eq203_e2551_d_b40: f64 = (p.p7 * eq203_e2550_d_b40);
        let eq203_e2551_d_b41: f64 = (p.p7 * eq203_e2550_d_b41);
        let eq203_e2551_d_b42: f64 = (p.p7 * eq203_e2550_d_b42);
        let eq203_e2551_d_b43: f64 = (p.p7 * eq203_e2550_d_b43);
        let eq203_e2551_d_b44: f64 = (p.p7 * eq203_e2550_d_b44);
        let eq203_e2551_d_b45: f64 = (p.p7 * eq203_e2550_d_b45);
        let eq203_e2551_d_b46: f64 = (p.p7 * eq203_e2550_d_b46);
        let eq203_e2551_d_b47: f64 = (p.p7 * eq203_e2550_d_b47);
        let eq203_e2551_d_b48: f64 = (p.p7 * eq203_e2550_d_b48);
        let eq203_e2551_d_b49: f64 = (p.p7 * eq203_e2550_d_b49);
        let eq203_e2551_d_b50: f64 = (p.p7 * eq203_e2550_d_b50);
        let eq203_e2551_d_b51: f64 = (p.p7 * eq203_e2550_d_b51);
        let eq203_e2551_d_b52: f64 = (p.p7 * eq203_e2550_d_b52);
        let eq203_e2551_d_b53: f64 = (p.p7 * eq203_e2550_d_b53);
        let eq203_e2551_d_b54: f64 = (p.p7 * eq203_e2550_d_b54);
        (eq203_e2551, eq203_e2551_d_n0, eq203_e2551_d_n1, eq203_e2551_d_n2, eq203_e2551_d_n3, eq203_e2551_d_n4, eq203_e2551_d_n5, eq203_e2551_d_n6, eq203_e2551_d_n7, eq203_e2551_d_n8, eq203_e2551_d_n9, eq203_e2551_d_n10, eq203_e2551_d_n11, eq203_e2551_d_n12, eq203_e2551_d_n13, eq203_e2551_d_n14, eq203_e2551_d_n15, eq203_e2551_d_n16, eq203_e2551_d_n17, eq203_e2551_d_n18, eq203_e2551_d_n19, eq203_e2551_d_n20, eq203_e2551_d_n21, eq203_e2551_d_n22, eq203_e2551_d_b0, eq203_e2551_d_b1, eq203_e2551_d_b2, eq203_e2551_d_b3, eq203_e2551_d_b4, eq203_e2551_d_b5, eq203_e2551_d_b6, eq203_e2551_d_b7, eq203_e2551_d_b8, eq203_e2551_d_b9, eq203_e2551_d_b10, eq203_e2551_d_b11, eq203_e2551_d_b12, eq203_e2551_d_b13, eq203_e2551_d_b14, eq203_e2551_d_b15, eq203_e2551_d_b16, eq203_e2551_d_b17, eq203_e2551_d_b18, eq203_e2551_d_b19, eq203_e2551_d_b20, eq203_e2551_d_b21, eq203_e2551_d_b22, eq203_e2551_d_b23, eq203_e2551_d_b24, eq203_e2551_d_b25, eq203_e2551_d_b26, eq203_e2551_d_b27, eq203_e2551_d_b28, eq203_e2551_d_b29, eq203_e2551_d_b30, eq203_e2551_d_b31, eq203_e2551_d_b32, eq203_e2551_d_b33, eq203_e2551_d_b34, eq203_e2551_d_b35, eq203_e2551_d_b36, eq203_e2551_d_b37, eq203_e2551_d_b38, eq203_e2551_d_b39, eq203_e2551_d_b40, eq203_e2551_d_b41, eq203_e2551_d_b42, eq203_e2551_d_b43, eq203_e2551_d_b44, eq203_e2551_d_b45, eq203_e2551_d_b46, eq203_e2551_d_b47, eq203_e2551_d_b48, eq203_e2551_d_b49, eq203_e2551_d_b50, eq203_e2551_d_b51, eq203_e2551_d_b52, eq203_e2551_d_b53, eq203_e2551_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq203_value: f64 = eq203_e2553;
        let eq203_node_derivatives: [f64; 23] = [eq203_e2553_d_n0, eq203_e2553_d_n1, eq203_e2553_d_n2, eq203_e2553_d_n3, eq203_e2553_d_n4, eq203_e2553_d_n5, eq203_e2553_d_n6, eq203_e2553_d_n7, eq203_e2553_d_n8, eq203_e2553_d_n9, eq203_e2553_d_n10, eq203_e2553_d_n11, eq203_e2553_d_n12, eq203_e2553_d_n13, eq203_e2553_d_n14, eq203_e2553_d_n15, eq203_e2553_d_n16, eq203_e2553_d_n17, eq203_e2553_d_n18, eq203_e2553_d_n19, eq203_e2553_d_n20, eq203_e2553_d_n21, eq203_e2553_d_n22];
        let eq203_branch_derivatives: [f64; 55] = [eq203_e2553_d_b0, eq203_e2553_d_b1, eq203_e2553_d_b2, eq203_e2553_d_b3, eq203_e2553_d_b4, eq203_e2553_d_b5, eq203_e2553_d_b6, eq203_e2553_d_b7, eq203_e2553_d_b8, eq203_e2553_d_b9, eq203_e2553_d_b10, eq203_e2553_d_b11, eq203_e2553_d_b12, eq203_e2553_d_b13, eq203_e2553_d_b14, eq203_e2553_d_b15, eq203_e2553_d_b16, eq203_e2553_d_b17, eq203_e2553_d_b18, eq203_e2553_d_b19, eq203_e2553_d_b20, eq203_e2553_d_b21, eq203_e2553_d_b22, eq203_e2553_d_b23, eq203_e2553_d_b24, eq203_e2553_d_b25, eq203_e2553_d_b26, eq203_e2553_d_b27, eq203_e2553_d_b28, eq203_e2553_d_b29, eq203_e2553_d_b30, eq203_e2553_d_b31, eq203_e2553_d_b32, eq203_e2553_d_b33, eq203_e2553_d_b34, eq203_e2553_d_b35, eq203_e2553_d_b36, eq203_e2553_d_b37, eq203_e2553_d_b38, eq203_e2553_d_b39, eq203_e2553_d_b40, eq203_e2553_d_b41, eq203_e2553_d_b42, eq203_e2553_d_b43, eq203_e2553_d_b44, eq203_e2553_d_b45, eq203_e2553_d_b46, eq203_e2553_d_b47, eq203_e2553_d_b48, eq203_e2553_d_b49, eq203_e2553_d_b50, eq203_e2553_d_b51, eq203_e2553_d_b52, eq203_e2553_d_b53, eq203_e2553_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq203_value),
            &eq203_node_derivatives,
            &eq203_branch_derivatives,
            multiplicity,
        );
        let (eq204_e2562, eq204_e2562_d_n0, eq204_e2562_d_n1, eq204_e2562_d_n2, eq204_e2562_d_n3, eq204_e2562_d_n4, eq204_e2562_d_n5, eq204_e2562_d_n6, eq204_e2562_d_n7, eq204_e2562_d_n8, eq204_e2562_d_n9, eq204_e2562_d_n10, eq204_e2562_d_n11, eq204_e2562_d_n12, eq204_e2562_d_n13, eq204_e2562_d_n14, eq204_e2562_d_n15, eq204_e2562_d_n16, eq204_e2562_d_n17, eq204_e2562_d_n18, eq204_e2562_d_n19, eq204_e2562_d_n20, eq204_e2562_d_n21, eq204_e2562_d_n22, eq204_e2562_d_b0, eq204_e2562_d_b1, eq204_e2562_d_b2, eq204_e2562_d_b3, eq204_e2562_d_b4, eq204_e2562_d_b5, eq204_e2562_d_b6, eq204_e2562_d_b7, eq204_e2562_d_b8, eq204_e2562_d_b9, eq204_e2562_d_b10, eq204_e2562_d_b11, eq204_e2562_d_b12, eq204_e2562_d_b13, eq204_e2562_d_b14, eq204_e2562_d_b15, eq204_e2562_d_b16, eq204_e2562_d_b17, eq204_e2562_d_b18, eq204_e2562_d_b19, eq204_e2562_d_b20, eq204_e2562_d_b21, eq204_e2562_d_b22, eq204_e2562_d_b23, eq204_e2562_d_b24, eq204_e2562_d_b25, eq204_e2562_d_b26, eq204_e2562_d_b27, eq204_e2562_d_b28, eq204_e2562_d_b29, eq204_e2562_d_b30, eq204_e2562_d_b31, eq204_e2562_d_b32, eq204_e2562_d_b33, eq204_e2562_d_b34, eq204_e2562_d_b35, eq204_e2562_d_b36, eq204_e2562_d_b37, eq204_e2562_d_b38, eq204_e2562_d_b39, eq204_e2562_d_b40, eq204_e2562_d_b41, eq204_e2562_d_b42, eq204_e2562_d_b43, eq204_e2562_d_b44, eq204_e2562_d_b45, eq204_e2562_d_b46, eq204_e2562_d_b47, eq204_e2562_d_b48, eq204_e2562_d_b49, eq204_e2562_d_b50, eq204_e2562_d_b51, eq204_e2562_d_b52, eq204_e2562_d_b53, eq204_e2562_d_b54,) = {
    if (s.b[605] && s.b[606]) {
        let eq204_e2559: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 103, s.v[313]);
        let eq204_e2560: f64 = (p.p7 * eq204_e2559);
        let eq204_e2560_d_n0: f64 = (p.p7 * (s.dn[313][0] * ddt_scale));
        let eq204_e2560_d_n1: f64 = (p.p7 * (s.dn[313][1] * ddt_scale));
        let eq204_e2560_d_n2: f64 = (p.p7 * (s.dn[313][2] * ddt_scale));
        let eq204_e2560_d_n3: f64 = (p.p7 * (s.dn[313][3] * ddt_scale));
        let eq204_e2560_d_n4: f64 = (p.p7 * (s.dn[313][4] * ddt_scale));
        let eq204_e2560_d_n5: f64 = (p.p7 * (s.dn[313][5] * ddt_scale));
        let eq204_e2560_d_n6: f64 = (p.p7 * (s.dn[313][6] * ddt_scale));
        let eq204_e2560_d_n7: f64 = (p.p7 * (s.dn[313][7] * ddt_scale));
        let eq204_e2560_d_n8: f64 = (p.p7 * (s.dn[313][8] * ddt_scale));
        let eq204_e2560_d_n9: f64 = (p.p7 * (s.dn[313][9] * ddt_scale));
        let eq204_e2560_d_n10: f64 = (p.p7 * (s.dn[313][10] * ddt_scale));
        let eq204_e2560_d_n11: f64 = (p.p7 * (s.dn[313][11] * ddt_scale));
        let eq204_e2560_d_n12: f64 = (p.p7 * (s.dn[313][12] * ddt_scale));
        let eq204_e2560_d_n13: f64 = (p.p7 * (s.dn[313][13] * ddt_scale));
        let eq204_e2560_d_n14: f64 = (p.p7 * (s.dn[313][14] * ddt_scale));
        let eq204_e2560_d_n15: f64 = (p.p7 * (s.dn[313][15] * ddt_scale));
        let eq204_e2560_d_n16: f64 = (p.p7 * (s.dn[313][16] * ddt_scale));
        let eq204_e2560_d_n17: f64 = (p.p7 * (s.dn[313][17] * ddt_scale));
        let eq204_e2560_d_n18: f64 = (p.p7 * (s.dn[313][18] * ddt_scale));
        let eq204_e2560_d_n19: f64 = (p.p7 * (s.dn[313][19] * ddt_scale));
        let eq204_e2560_d_n20: f64 = (p.p7 * (s.dn[313][20] * ddt_scale));
        let eq204_e2560_d_n21: f64 = (p.p7 * (s.dn[313][21] * ddt_scale));
        let eq204_e2560_d_n22: f64 = (p.p7 * (s.dn[313][22] * ddt_scale));
        let eq204_e2560_d_b0: f64 = (p.p7 * (s.db[313][0] * ddt_scale));
        let eq204_e2560_d_b1: f64 = (p.p7 * (s.db[313][1] * ddt_scale));
        let eq204_e2560_d_b2: f64 = (p.p7 * (s.db[313][2] * ddt_scale));
        let eq204_e2560_d_b3: f64 = (p.p7 * (s.db[313][3] * ddt_scale));
        let eq204_e2560_d_b4: f64 = (p.p7 * (s.db[313][4] * ddt_scale));
        let eq204_e2560_d_b5: f64 = (p.p7 * (s.db[313][5] * ddt_scale));
        let eq204_e2560_d_b6: f64 = (p.p7 * (s.db[313][6] * ddt_scale));
        let eq204_e2560_d_b7: f64 = (p.p7 * (s.db[313][7] * ddt_scale));
        let eq204_e2560_d_b8: f64 = (p.p7 * (s.db[313][8] * ddt_scale));
        let eq204_e2560_d_b9: f64 = (p.p7 * (s.db[313][9] * ddt_scale));
        let eq204_e2560_d_b10: f64 = (p.p7 * (s.db[313][10] * ddt_scale));
        let eq204_e2560_d_b11: f64 = (p.p7 * (s.db[313][11] * ddt_scale));
        let eq204_e2560_d_b12: f64 = (p.p7 * (s.db[313][12] * ddt_scale));
        let eq204_e2560_d_b13: f64 = (p.p7 * (s.db[313][13] * ddt_scale));
        let eq204_e2560_d_b14: f64 = (p.p7 * (s.db[313][14] * ddt_scale));
        let eq204_e2560_d_b15: f64 = (p.p7 * (s.db[313][15] * ddt_scale));
        let eq204_e2560_d_b16: f64 = (p.p7 * (s.db[313][16] * ddt_scale));
        let eq204_e2560_d_b17: f64 = (p.p7 * (s.db[313][17] * ddt_scale));
        let eq204_e2560_d_b18: f64 = (p.p7 * (s.db[313][18] * ddt_scale));
        let eq204_e2560_d_b19: f64 = (p.p7 * (s.db[313][19] * ddt_scale));
        let eq204_e2560_d_b20: f64 = (p.p7 * (s.db[313][20] * ddt_scale));
        let eq204_e2560_d_b21: f64 = (p.p7 * (s.db[313][21] * ddt_scale));
        let eq204_e2560_d_b22: f64 = (p.p7 * (s.db[313][22] * ddt_scale));
        let eq204_e2560_d_b23: f64 = (p.p7 * (s.db[313][23] * ddt_scale));
        let eq204_e2560_d_b24: f64 = (p.p7 * (s.db[313][24] * ddt_scale));
        let eq204_e2560_d_b25: f64 = (p.p7 * (s.db[313][25] * ddt_scale));
        let eq204_e2560_d_b26: f64 = (p.p7 * (s.db[313][26] * ddt_scale));
        let eq204_e2560_d_b27: f64 = (p.p7 * (s.db[313][27] * ddt_scale));
        let eq204_e2560_d_b28: f64 = (p.p7 * (s.db[313][28] * ddt_scale));
        let eq204_e2560_d_b29: f64 = (p.p7 * (s.db[313][29] * ddt_scale));
        let eq204_e2560_d_b30: f64 = (p.p7 * (s.db[313][30] * ddt_scale));
        let eq204_e2560_d_b31: f64 = (p.p7 * (s.db[313][31] * ddt_scale));
        let eq204_e2560_d_b32: f64 = (p.p7 * (s.db[313][32] * ddt_scale));
        let eq204_e2560_d_b33: f64 = (p.p7 * (s.db[313][33] * ddt_scale));
        let eq204_e2560_d_b34: f64 = (p.p7 * (s.db[313][34] * ddt_scale));
        let eq204_e2560_d_b35: f64 = (p.p7 * (s.db[313][35] * ddt_scale));
        let eq204_e2560_d_b36: f64 = (p.p7 * (s.db[313][36] * ddt_scale));
        let eq204_e2560_d_b37: f64 = (p.p7 * (s.db[313][37] * ddt_scale));
        let eq204_e2560_d_b38: f64 = (p.p7 * (s.db[313][38] * ddt_scale));
        let eq204_e2560_d_b39: f64 = (p.p7 * (s.db[313][39] * ddt_scale));
        let eq204_e2560_d_b40: f64 = (p.p7 * (s.db[313][40] * ddt_scale));
        let eq204_e2560_d_b41: f64 = (p.p7 * (s.db[313][41] * ddt_scale));
        let eq204_e2560_d_b42: f64 = (p.p7 * (s.db[313][42] * ddt_scale));
        let eq204_e2560_d_b43: f64 = (p.p7 * (s.db[313][43] * ddt_scale));
        let eq204_e2560_d_b44: f64 = (p.p7 * (s.db[313][44] * ddt_scale));
        let eq204_e2560_d_b45: f64 = (p.p7 * (s.db[313][45] * ddt_scale));
        let eq204_e2560_d_b46: f64 = (p.p7 * (s.db[313][46] * ddt_scale));
        let eq204_e2560_d_b47: f64 = (p.p7 * (s.db[313][47] * ddt_scale));
        let eq204_e2560_d_b48: f64 = (p.p7 * (s.db[313][48] * ddt_scale));
        let eq204_e2560_d_b49: f64 = (p.p7 * (s.db[313][49] * ddt_scale));
        let eq204_e2560_d_b50: f64 = (p.p7 * (s.db[313][50] * ddt_scale));
        let eq204_e2560_d_b51: f64 = (p.p7 * (s.db[313][51] * ddt_scale));
        let eq204_e2560_d_b52: f64 = (p.p7 * (s.db[313][52] * ddt_scale));
        let eq204_e2560_d_b53: f64 = (p.p7 * (s.db[313][53] * ddt_scale));
        let eq204_e2560_d_b54: f64 = (p.p7 * (s.db[313][54] * ddt_scale));
        (eq204_e2560, eq204_e2560_d_n0, eq204_e2560_d_n1, eq204_e2560_d_n2, eq204_e2560_d_n3, eq204_e2560_d_n4, eq204_e2560_d_n5, eq204_e2560_d_n6, eq204_e2560_d_n7, eq204_e2560_d_n8, eq204_e2560_d_n9, eq204_e2560_d_n10, eq204_e2560_d_n11, eq204_e2560_d_n12, eq204_e2560_d_n13, eq204_e2560_d_n14, eq204_e2560_d_n15, eq204_e2560_d_n16, eq204_e2560_d_n17, eq204_e2560_d_n18, eq204_e2560_d_n19, eq204_e2560_d_n20, eq204_e2560_d_n21, eq204_e2560_d_n22, eq204_e2560_d_b0, eq204_e2560_d_b1, eq204_e2560_d_b2, eq204_e2560_d_b3, eq204_e2560_d_b4, eq204_e2560_d_b5, eq204_e2560_d_b6, eq204_e2560_d_b7, eq204_e2560_d_b8, eq204_e2560_d_b9, eq204_e2560_d_b10, eq204_e2560_d_b11, eq204_e2560_d_b12, eq204_e2560_d_b13, eq204_e2560_d_b14, eq204_e2560_d_b15, eq204_e2560_d_b16, eq204_e2560_d_b17, eq204_e2560_d_b18, eq204_e2560_d_b19, eq204_e2560_d_b20, eq204_e2560_d_b21, eq204_e2560_d_b22, eq204_e2560_d_b23, eq204_e2560_d_b24, eq204_e2560_d_b25, eq204_e2560_d_b26, eq204_e2560_d_b27, eq204_e2560_d_b28, eq204_e2560_d_b29, eq204_e2560_d_b30, eq204_e2560_d_b31, eq204_e2560_d_b32, eq204_e2560_d_b33, eq204_e2560_d_b34, eq204_e2560_d_b35, eq204_e2560_d_b36, eq204_e2560_d_b37, eq204_e2560_d_b38, eq204_e2560_d_b39, eq204_e2560_d_b40, eq204_e2560_d_b41, eq204_e2560_d_b42, eq204_e2560_d_b43, eq204_e2560_d_b44, eq204_e2560_d_b45, eq204_e2560_d_b46, eq204_e2560_d_b47, eq204_e2560_d_b48, eq204_e2560_d_b49, eq204_e2560_d_b50, eq204_e2560_d_b51, eq204_e2560_d_b52, eq204_e2560_d_b53, eq204_e2560_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq204_value: f64 = eq204_e2562;
        let eq204_node_derivatives: [f64; 23] = [eq204_e2562_d_n0, eq204_e2562_d_n1, eq204_e2562_d_n2, eq204_e2562_d_n3, eq204_e2562_d_n4, eq204_e2562_d_n5, eq204_e2562_d_n6, eq204_e2562_d_n7, eq204_e2562_d_n8, eq204_e2562_d_n9, eq204_e2562_d_n10, eq204_e2562_d_n11, eq204_e2562_d_n12, eq204_e2562_d_n13, eq204_e2562_d_n14, eq204_e2562_d_n15, eq204_e2562_d_n16, eq204_e2562_d_n17, eq204_e2562_d_n18, eq204_e2562_d_n19, eq204_e2562_d_n20, eq204_e2562_d_n21, eq204_e2562_d_n22];
        let eq204_branch_derivatives: [f64; 55] = [eq204_e2562_d_b0, eq204_e2562_d_b1, eq204_e2562_d_b2, eq204_e2562_d_b3, eq204_e2562_d_b4, eq204_e2562_d_b5, eq204_e2562_d_b6, eq204_e2562_d_b7, eq204_e2562_d_b8, eq204_e2562_d_b9, eq204_e2562_d_b10, eq204_e2562_d_b11, eq204_e2562_d_b12, eq204_e2562_d_b13, eq204_e2562_d_b14, eq204_e2562_d_b15, eq204_e2562_d_b16, eq204_e2562_d_b17, eq204_e2562_d_b18, eq204_e2562_d_b19, eq204_e2562_d_b20, eq204_e2562_d_b21, eq204_e2562_d_b22, eq204_e2562_d_b23, eq204_e2562_d_b24, eq204_e2562_d_b25, eq204_e2562_d_b26, eq204_e2562_d_b27, eq204_e2562_d_b28, eq204_e2562_d_b29, eq204_e2562_d_b30, eq204_e2562_d_b31, eq204_e2562_d_b32, eq204_e2562_d_b33, eq204_e2562_d_b34, eq204_e2562_d_b35, eq204_e2562_d_b36, eq204_e2562_d_b37, eq204_e2562_d_b38, eq204_e2562_d_b39, eq204_e2562_d_b40, eq204_e2562_d_b41, eq204_e2562_d_b42, eq204_e2562_d_b43, eq204_e2562_d_b44, eq204_e2562_d_b45, eq204_e2562_d_b46, eq204_e2562_d_b47, eq204_e2562_d_b48, eq204_e2562_d_b49, eq204_e2562_d_b50, eq204_e2562_d_b51, eq204_e2562_d_b52, eq204_e2562_d_b53, eq204_e2562_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(22),
            multiplicity * (eq204_value),
            &eq204_node_derivatives,
            &eq204_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_43(
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[312][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[312][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[312][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[312][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[312][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[312][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[312][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[312][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[312][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[312][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[312][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[312][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[312][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[312][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[312][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[312][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[312][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[312][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[312][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[312][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[312][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[312][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[312][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[312][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[312][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[312][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[312][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[312][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[312][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[312][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[312][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[312][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[312][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[312][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[312][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[312][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[312][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[312][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[312][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[312][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[312][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[312][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[312][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[312][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[312][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[312][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[312][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[312][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[312][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[312][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[312][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[312][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[312][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[312][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[312][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[312][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[312][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[312][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[312][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[312][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[312][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[312][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[312][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[312][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[312][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[312][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[312][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[312][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[312][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[312][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[312][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[312][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[312][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[312][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[312][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[312][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[312][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[312][54] * ddt_scale));
        let (eq205_e2573, eq205_e2573_d_n0, eq205_e2573_d_n1, eq205_e2573_d_n2, eq205_e2573_d_n3, eq205_e2573_d_n4, eq205_e2573_d_n5, eq205_e2573_d_n6, eq205_e2573_d_n7, eq205_e2573_d_n8, eq205_e2573_d_n9, eq205_e2573_d_n10, eq205_e2573_d_n11, eq205_e2573_d_n12, eq205_e2573_d_n13, eq205_e2573_d_n14, eq205_e2573_d_n15, eq205_e2573_d_n16, eq205_e2573_d_n17, eq205_e2573_d_n18, eq205_e2573_d_n19, eq205_e2573_d_n20, eq205_e2573_d_n21, eq205_e2573_d_n22, eq205_e2573_d_b0, eq205_e2573_d_b1, eq205_e2573_d_b2, eq205_e2573_d_b3, eq205_e2573_d_b4, eq205_e2573_d_b5, eq205_e2573_d_b6, eq205_e2573_d_b7, eq205_e2573_d_b8, eq205_e2573_d_b9, eq205_e2573_d_b10, eq205_e2573_d_b11, eq205_e2573_d_b12, eq205_e2573_d_b13, eq205_e2573_d_b14, eq205_e2573_d_b15, eq205_e2573_d_b16, eq205_e2573_d_b17, eq205_e2573_d_b18, eq205_e2573_d_b19, eq205_e2573_d_b20, eq205_e2573_d_b21, eq205_e2573_d_b22, eq205_e2573_d_b23, eq205_e2573_d_b24, eq205_e2573_d_b25, eq205_e2573_d_b26, eq205_e2573_d_b27, eq205_e2573_d_b28, eq205_e2573_d_b29, eq205_e2573_d_b30, eq205_e2573_d_b31, eq205_e2573_d_b32, eq205_e2573_d_b33, eq205_e2573_d_b34, eq205_e2573_d_b35, eq205_e2573_d_b36, eq205_e2573_d_b37, eq205_e2573_d_b38, eq205_e2573_d_b39, eq205_e2573_d_b40, eq205_e2573_d_b41, eq205_e2573_d_b42, eq205_e2573_d_b43, eq205_e2573_d_b44, eq205_e2573_d_b45, eq205_e2573_d_b46, eq205_e2573_d_b47, eq205_e2573_d_b48, eq205_e2573_d_b49, eq205_e2573_d_b50, eq205_e2573_d_b51, eq205_e2573_d_b52, eq205_e2573_d_b53, eq205_e2573_d_b54,) = {
    if ((s.b[605] && s.b[606]) && s.b[607]) {
        let eq205_e2570: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 104, s.v[312]);
        let eq205_e2571: f64 = (p.p7 * eq205_e2570);
        (eq205_e2571, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq205_value: f64 = eq205_e2573;
        let eq205_node_derivatives: [f64; 23] = [eq205_e2573_d_n0, eq205_e2573_d_n1, eq205_e2573_d_n2, eq205_e2573_d_n3, eq205_e2573_d_n4, eq205_e2573_d_n5, eq205_e2573_d_n6, eq205_e2573_d_n7, eq205_e2573_d_n8, eq205_e2573_d_n9, eq205_e2573_d_n10, eq205_e2573_d_n11, eq205_e2573_d_n12, eq205_e2573_d_n13, eq205_e2573_d_n14, eq205_e2573_d_n15, eq205_e2573_d_n16, eq205_e2573_d_n17, eq205_e2573_d_n18, eq205_e2573_d_n19, eq205_e2573_d_n20, eq205_e2573_d_n21, eq205_e2573_d_n22];
        let eq205_branch_derivatives: [f64; 55] = [eq205_e2573_d_b0, eq205_e2573_d_b1, eq205_e2573_d_b2, eq205_e2573_d_b3, eq205_e2573_d_b4, eq205_e2573_d_b5, eq205_e2573_d_b6, eq205_e2573_d_b7, eq205_e2573_d_b8, eq205_e2573_d_b9, eq205_e2573_d_b10, eq205_e2573_d_b11, eq205_e2573_d_b12, eq205_e2573_d_b13, eq205_e2573_d_b14, eq205_e2573_d_b15, eq205_e2573_d_b16, eq205_e2573_d_b17, eq205_e2573_d_b18, eq205_e2573_d_b19, eq205_e2573_d_b20, eq205_e2573_d_b21, eq205_e2573_d_b22, eq205_e2573_d_b23, eq205_e2573_d_b24, eq205_e2573_d_b25, eq205_e2573_d_b26, eq205_e2573_d_b27, eq205_e2573_d_b28, eq205_e2573_d_b29, eq205_e2573_d_b30, eq205_e2573_d_b31, eq205_e2573_d_b32, eq205_e2573_d_b33, eq205_e2573_d_b34, eq205_e2573_d_b35, eq205_e2573_d_b36, eq205_e2573_d_b37, eq205_e2573_d_b38, eq205_e2573_d_b39, eq205_e2573_d_b40, eq205_e2573_d_b41, eq205_e2573_d_b42, eq205_e2573_d_b43, eq205_e2573_d_b44, eq205_e2573_d_b45, eq205_e2573_d_b46, eq205_e2573_d_b47, eq205_e2573_d_b48, eq205_e2573_d_b49, eq205_e2573_d_b50, eq205_e2573_d_b51, eq205_e2573_d_b52, eq205_e2573_d_b53, eq205_e2573_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(22),
            multiplicity * (eq205_value),
            &eq205_node_derivatives,
            &eq205_branch_derivatives,
            multiplicity,
        );
        let (eq206_e2586, eq206_e2586_d_n0, eq206_e2586_d_n1, eq206_e2586_d_n2, eq206_e2586_d_n3, eq206_e2586_d_n4, eq206_e2586_d_n5, eq206_e2586_d_n6, eq206_e2586_d_n7, eq206_e2586_d_n8, eq206_e2586_d_n9, eq206_e2586_d_n10, eq206_e2586_d_n11, eq206_e2586_d_n12, eq206_e2586_d_n13, eq206_e2586_d_n14, eq206_e2586_d_n15, eq206_e2586_d_n16, eq206_e2586_d_n17, eq206_e2586_d_n18, eq206_e2586_d_n19, eq206_e2586_d_n20, eq206_e2586_d_n21, eq206_e2586_d_n22, eq206_e2586_d_b0, eq206_e2586_d_b1, eq206_e2586_d_b2, eq206_e2586_d_b3, eq206_e2586_d_b4, eq206_e2586_d_b5, eq206_e2586_d_b6, eq206_e2586_d_b7, eq206_e2586_d_b8, eq206_e2586_d_b9, eq206_e2586_d_b10, eq206_e2586_d_b11, eq206_e2586_d_b12, eq206_e2586_d_b13, eq206_e2586_d_b14, eq206_e2586_d_b15, eq206_e2586_d_b16, eq206_e2586_d_b17, eq206_e2586_d_b18, eq206_e2586_d_b19, eq206_e2586_d_b20, eq206_e2586_d_b21, eq206_e2586_d_b22, eq206_e2586_d_b23, eq206_e2586_d_b24, eq206_e2586_d_b25, eq206_e2586_d_b26, eq206_e2586_d_b27, eq206_e2586_d_b28, eq206_e2586_d_b29, eq206_e2586_d_b30, eq206_e2586_d_b31, eq206_e2586_d_b32, eq206_e2586_d_b33, eq206_e2586_d_b34, eq206_e2586_d_b35, eq206_e2586_d_b36, eq206_e2586_d_b37, eq206_e2586_d_b38, eq206_e2586_d_b39, eq206_e2586_d_b40, eq206_e2586_d_b41, eq206_e2586_d_b42, eq206_e2586_d_b43, eq206_e2586_d_b44, eq206_e2586_d_b45, eq206_e2586_d_b46, eq206_e2586_d_b47, eq206_e2586_d_b48, eq206_e2586_d_b49, eq206_e2586_d_b50, eq206_e2586_d_b51, eq206_e2586_d_b52, eq206_e2586_d_b53, eq206_e2586_d_b54,) = {
    if ((s.b[605] && s.b[606]) && s.b[607]) {
        let eq206_e2581: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 105, s.v[312]);
        let eq206_e2582: f64 = (p.p7 * eq206_e2581);
        let eq206_e2584: f64 = (eq206_e2582 * p.p249);
        let eq206_e2584_d_n0: f64 = (__rspice_deriv_cse_0 * p.p249);
        let eq206_e2584_d_n1: f64 = (__rspice_deriv_cse_1 * p.p249);
        let eq206_e2584_d_n2: f64 = (__rspice_deriv_cse_2 * p.p249);
        let eq206_e2584_d_n3: f64 = (__rspice_deriv_cse_3 * p.p249);
        let eq206_e2584_d_n4: f64 = (__rspice_deriv_cse_4 * p.p249);
        let eq206_e2584_d_n5: f64 = (__rspice_deriv_cse_5 * p.p249);
        let eq206_e2584_d_n6: f64 = (__rspice_deriv_cse_6 * p.p249);
        let eq206_e2584_d_n7: f64 = (__rspice_deriv_cse_7 * p.p249);
        let eq206_e2584_d_n8: f64 = (__rspice_deriv_cse_8 * p.p249);
        let eq206_e2584_d_n9: f64 = (__rspice_deriv_cse_9 * p.p249);
        let eq206_e2584_d_n10: f64 = (__rspice_deriv_cse_10 * p.p249);
        let eq206_e2584_d_n11: f64 = (__rspice_deriv_cse_11 * p.p249);
        let eq206_e2584_d_n12: f64 = (__rspice_deriv_cse_12 * p.p249);
        let eq206_e2584_d_n13: f64 = (__rspice_deriv_cse_13 * p.p249);
        let eq206_e2584_d_n14: f64 = (__rspice_deriv_cse_14 * p.p249);
        let eq206_e2584_d_n15: f64 = (__rspice_deriv_cse_15 * p.p249);
        let eq206_e2584_d_n16: f64 = (__rspice_deriv_cse_16 * p.p249);
        let eq206_e2584_d_n17: f64 = (__rspice_deriv_cse_17 * p.p249);
        let eq206_e2584_d_n18: f64 = (__rspice_deriv_cse_18 * p.p249);
        let eq206_e2584_d_n19: f64 = (__rspice_deriv_cse_19 * p.p249);
        let eq206_e2584_d_n20: f64 = (__rspice_deriv_cse_20 * p.p249);
        let eq206_e2584_d_n21: f64 = (__rspice_deriv_cse_21 * p.p249);
        let eq206_e2584_d_n22: f64 = (__rspice_deriv_cse_22 * p.p249);
        let eq206_e2584_d_b0: f64 = (__rspice_deriv_cse_23 * p.p249);
        let eq206_e2584_d_b1: f64 = (__rspice_deriv_cse_24 * p.p249);
        let eq206_e2584_d_b2: f64 = (__rspice_deriv_cse_25 * p.p249);
        let eq206_e2584_d_b3: f64 = (__rspice_deriv_cse_26 * p.p249);
        let eq206_e2584_d_b4: f64 = (__rspice_deriv_cse_27 * p.p249);
        let eq206_e2584_d_b5: f64 = (__rspice_deriv_cse_28 * p.p249);
        let eq206_e2584_d_b6: f64 = (__rspice_deriv_cse_29 * p.p249);
        let eq206_e2584_d_b7: f64 = (__rspice_deriv_cse_30 * p.p249);
        let eq206_e2584_d_b8: f64 = (__rspice_deriv_cse_31 * p.p249);
        let eq206_e2584_d_b9: f64 = (__rspice_deriv_cse_32 * p.p249);
        let eq206_e2584_d_b10: f64 = (__rspice_deriv_cse_33 * p.p249);
        let eq206_e2584_d_b11: f64 = (__rspice_deriv_cse_34 * p.p249);
        let eq206_e2584_d_b12: f64 = (__rspice_deriv_cse_35 * p.p249);
        let eq206_e2584_d_b13: f64 = (__rspice_deriv_cse_36 * p.p249);
        let eq206_e2584_d_b14: f64 = (__rspice_deriv_cse_37 * p.p249);
        let eq206_e2584_d_b15: f64 = (__rspice_deriv_cse_38 * p.p249);
        let eq206_e2584_d_b16: f64 = (__rspice_deriv_cse_39 * p.p249);
        let eq206_e2584_d_b17: f64 = (__rspice_deriv_cse_40 * p.p249);
        let eq206_e2584_d_b18: f64 = (__rspice_deriv_cse_41 * p.p249);
        let eq206_e2584_d_b19: f64 = (__rspice_deriv_cse_42 * p.p249);
        let eq206_e2584_d_b20: f64 = (__rspice_deriv_cse_43 * p.p249);
        let eq206_e2584_d_b21: f64 = (__rspice_deriv_cse_44 * p.p249);
        let eq206_e2584_d_b22: f64 = (__rspice_deriv_cse_45 * p.p249);
        let eq206_e2584_d_b23: f64 = (__rspice_deriv_cse_46 * p.p249);
        let eq206_e2584_d_b24: f64 = (__rspice_deriv_cse_47 * p.p249);
        let eq206_e2584_d_b25: f64 = (__rspice_deriv_cse_48 * p.p249);
        let eq206_e2584_d_b26: f64 = (__rspice_deriv_cse_49 * p.p249);
        let eq206_e2584_d_b27: f64 = (__rspice_deriv_cse_50 * p.p249);
        let eq206_e2584_d_b28: f64 = (__rspice_deriv_cse_51 * p.p249);
        let eq206_e2584_d_b29: f64 = (__rspice_deriv_cse_52 * p.p249);
        let eq206_e2584_d_b30: f64 = (__rspice_deriv_cse_53 * p.p249);
        let eq206_e2584_d_b31: f64 = (__rspice_deriv_cse_54 * p.p249);
        let eq206_e2584_d_b32: f64 = (__rspice_deriv_cse_55 * p.p249);
        let eq206_e2584_d_b33: f64 = (__rspice_deriv_cse_56 * p.p249);
        let eq206_e2584_d_b34: f64 = (__rspice_deriv_cse_57 * p.p249);
        let eq206_e2584_d_b35: f64 = (__rspice_deriv_cse_58 * p.p249);
        let eq206_e2584_d_b36: f64 = (__rspice_deriv_cse_59 * p.p249);
        let eq206_e2584_d_b37: f64 = (__rspice_deriv_cse_60 * p.p249);
        let eq206_e2584_d_b38: f64 = (__rspice_deriv_cse_61 * p.p249);
        let eq206_e2584_d_b39: f64 = (__rspice_deriv_cse_62 * p.p249);
        let eq206_e2584_d_b40: f64 = (__rspice_deriv_cse_63 * p.p249);
        let eq206_e2584_d_b41: f64 = (__rspice_deriv_cse_64 * p.p249);
        let eq206_e2584_d_b42: f64 = (__rspice_deriv_cse_65 * p.p249);
        let eq206_e2584_d_b43: f64 = (__rspice_deriv_cse_66 * p.p249);
        let eq206_e2584_d_b44: f64 = (__rspice_deriv_cse_67 * p.p249);
        let eq206_e2584_d_b45: f64 = (__rspice_deriv_cse_68 * p.p249);
        let eq206_e2584_d_b46: f64 = (__rspice_deriv_cse_69 * p.p249);
        let eq206_e2584_d_b47: f64 = (__rspice_deriv_cse_70 * p.p249);
        let eq206_e2584_d_b48: f64 = (__rspice_deriv_cse_71 * p.p249);
        let eq206_e2584_d_b49: f64 = (__rspice_deriv_cse_72 * p.p249);
        let eq206_e2584_d_b50: f64 = (__rspice_deriv_cse_73 * p.p249);
        let eq206_e2584_d_b51: f64 = (__rspice_deriv_cse_74 * p.p249);
        let eq206_e2584_d_b52: f64 = (__rspice_deriv_cse_75 * p.p249);
        let eq206_e2584_d_b53: f64 = (__rspice_deriv_cse_76 * p.p249);
        let eq206_e2584_d_b54: f64 = (__rspice_deriv_cse_77 * p.p249);
        (eq206_e2584, eq206_e2584_d_n0, eq206_e2584_d_n1, eq206_e2584_d_n2, eq206_e2584_d_n3, eq206_e2584_d_n4, eq206_e2584_d_n5, eq206_e2584_d_n6, eq206_e2584_d_n7, eq206_e2584_d_n8, eq206_e2584_d_n9, eq206_e2584_d_n10, eq206_e2584_d_n11, eq206_e2584_d_n12, eq206_e2584_d_n13, eq206_e2584_d_n14, eq206_e2584_d_n15, eq206_e2584_d_n16, eq206_e2584_d_n17, eq206_e2584_d_n18, eq206_e2584_d_n19, eq206_e2584_d_n20, eq206_e2584_d_n21, eq206_e2584_d_n22, eq206_e2584_d_b0, eq206_e2584_d_b1, eq206_e2584_d_b2, eq206_e2584_d_b3, eq206_e2584_d_b4, eq206_e2584_d_b5, eq206_e2584_d_b6, eq206_e2584_d_b7, eq206_e2584_d_b8, eq206_e2584_d_b9, eq206_e2584_d_b10, eq206_e2584_d_b11, eq206_e2584_d_b12, eq206_e2584_d_b13, eq206_e2584_d_b14, eq206_e2584_d_b15, eq206_e2584_d_b16, eq206_e2584_d_b17, eq206_e2584_d_b18, eq206_e2584_d_b19, eq206_e2584_d_b20, eq206_e2584_d_b21, eq206_e2584_d_b22, eq206_e2584_d_b23, eq206_e2584_d_b24, eq206_e2584_d_b25, eq206_e2584_d_b26, eq206_e2584_d_b27, eq206_e2584_d_b28, eq206_e2584_d_b29, eq206_e2584_d_b30, eq206_e2584_d_b31, eq206_e2584_d_b32, eq206_e2584_d_b33, eq206_e2584_d_b34, eq206_e2584_d_b35, eq206_e2584_d_b36, eq206_e2584_d_b37, eq206_e2584_d_b38, eq206_e2584_d_b39, eq206_e2584_d_b40, eq206_e2584_d_b41, eq206_e2584_d_b42, eq206_e2584_d_b43, eq206_e2584_d_b44, eq206_e2584_d_b45, eq206_e2584_d_b46, eq206_e2584_d_b47, eq206_e2584_d_b48, eq206_e2584_d_b49, eq206_e2584_d_b50, eq206_e2584_d_b51, eq206_e2584_d_b52, eq206_e2584_d_b53, eq206_e2584_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq206_value: f64 = eq206_e2586;
        let eq206_node_derivatives: [f64; 23] = [eq206_e2586_d_n0, eq206_e2586_d_n1, eq206_e2586_d_n2, eq206_e2586_d_n3, eq206_e2586_d_n4, eq206_e2586_d_n5, eq206_e2586_d_n6, eq206_e2586_d_n7, eq206_e2586_d_n8, eq206_e2586_d_n9, eq206_e2586_d_n10, eq206_e2586_d_n11, eq206_e2586_d_n12, eq206_e2586_d_n13, eq206_e2586_d_n14, eq206_e2586_d_n15, eq206_e2586_d_n16, eq206_e2586_d_n17, eq206_e2586_d_n18, eq206_e2586_d_n19, eq206_e2586_d_n20, eq206_e2586_d_n21, eq206_e2586_d_n22];
        let eq206_branch_derivatives: [f64; 55] = [eq206_e2586_d_b0, eq206_e2586_d_b1, eq206_e2586_d_b2, eq206_e2586_d_b3, eq206_e2586_d_b4, eq206_e2586_d_b5, eq206_e2586_d_b6, eq206_e2586_d_b7, eq206_e2586_d_b8, eq206_e2586_d_b9, eq206_e2586_d_b10, eq206_e2586_d_b11, eq206_e2586_d_b12, eq206_e2586_d_b13, eq206_e2586_d_b14, eq206_e2586_d_b15, eq206_e2586_d_b16, eq206_e2586_d_b17, eq206_e2586_d_b18, eq206_e2586_d_b19, eq206_e2586_d_b20, eq206_e2586_d_b21, eq206_e2586_d_b22, eq206_e2586_d_b23, eq206_e2586_d_b24, eq206_e2586_d_b25, eq206_e2586_d_b26, eq206_e2586_d_b27, eq206_e2586_d_b28, eq206_e2586_d_b29, eq206_e2586_d_b30, eq206_e2586_d_b31, eq206_e2586_d_b32, eq206_e2586_d_b33, eq206_e2586_d_b34, eq206_e2586_d_b35, eq206_e2586_d_b36, eq206_e2586_d_b37, eq206_e2586_d_b38, eq206_e2586_d_b39, eq206_e2586_d_b40, eq206_e2586_d_b41, eq206_e2586_d_b42, eq206_e2586_d_b43, eq206_e2586_d_b44, eq206_e2586_d_b45, eq206_e2586_d_b46, eq206_e2586_d_b47, eq206_e2586_d_b48, eq206_e2586_d_b49, eq206_e2586_d_b50, eq206_e2586_d_b51, eq206_e2586_d_b52, eq206_e2586_d_b53, eq206_e2586_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(22),
            multiplicity * (eq206_value),
            &eq206_node_derivatives,
            &eq206_branch_derivatives,
            multiplicity,
        );
        let (eq207_e2598, eq207_e2598_d_n0, eq207_e2598_d_n1, eq207_e2598_d_n2, eq207_e2598_d_n3, eq207_e2598_d_n4, eq207_e2598_d_n5, eq207_e2598_d_n6, eq207_e2598_d_n7, eq207_e2598_d_n8, eq207_e2598_d_n9, eq207_e2598_d_n10, eq207_e2598_d_n11, eq207_e2598_d_n12, eq207_e2598_d_n13, eq207_e2598_d_n14, eq207_e2598_d_n15, eq207_e2598_d_n16, eq207_e2598_d_n17, eq207_e2598_d_n18, eq207_e2598_d_n19, eq207_e2598_d_n20, eq207_e2598_d_n21, eq207_e2598_d_n22, eq207_e2598_d_b0, eq207_e2598_d_b1, eq207_e2598_d_b2, eq207_e2598_d_b3, eq207_e2598_d_b4, eq207_e2598_d_b5, eq207_e2598_d_b6, eq207_e2598_d_b7, eq207_e2598_d_b8, eq207_e2598_d_b9, eq207_e2598_d_b10, eq207_e2598_d_b11, eq207_e2598_d_b12, eq207_e2598_d_b13, eq207_e2598_d_b14, eq207_e2598_d_b15, eq207_e2598_d_b16, eq207_e2598_d_b17, eq207_e2598_d_b18, eq207_e2598_d_b19, eq207_e2598_d_b20, eq207_e2598_d_b21, eq207_e2598_d_b22, eq207_e2598_d_b23, eq207_e2598_d_b24, eq207_e2598_d_b25, eq207_e2598_d_b26, eq207_e2598_d_b27, eq207_e2598_d_b28, eq207_e2598_d_b29, eq207_e2598_d_b30, eq207_e2598_d_b31, eq207_e2598_d_b32, eq207_e2598_d_b33, eq207_e2598_d_b34, eq207_e2598_d_b35, eq207_e2598_d_b36, eq207_e2598_d_b37, eq207_e2598_d_b38, eq207_e2598_d_b39, eq207_e2598_d_b40, eq207_e2598_d_b41, eq207_e2598_d_b42, eq207_e2598_d_b43, eq207_e2598_d_b44, eq207_e2598_d_b45, eq207_e2598_d_b46, eq207_e2598_d_b47, eq207_e2598_d_b48, eq207_e2598_d_b49, eq207_e2598_d_b50, eq207_e2598_d_b51, eq207_e2598_d_b52, eq207_e2598_d_b53, eq207_e2598_d_b54,) = {
    if ((s.b[605] && s.b[606]) && (!s.b[607])) {
        let eq207_e2595: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 106, s.v[312]);
        let eq207_e2596: f64 = (p.p7 * eq207_e2595);
        (eq207_e2596, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq207_value: f64 = eq207_e2598;
        let eq207_node_derivatives: [f64; 23] = [eq207_e2598_d_n0, eq207_e2598_d_n1, eq207_e2598_d_n2, eq207_e2598_d_n3, eq207_e2598_d_n4, eq207_e2598_d_n5, eq207_e2598_d_n6, eq207_e2598_d_n7, eq207_e2598_d_n8, eq207_e2598_d_n9, eq207_e2598_d_n10, eq207_e2598_d_n11, eq207_e2598_d_n12, eq207_e2598_d_n13, eq207_e2598_d_n14, eq207_e2598_d_n15, eq207_e2598_d_n16, eq207_e2598_d_n17, eq207_e2598_d_n18, eq207_e2598_d_n19, eq207_e2598_d_n20, eq207_e2598_d_n21, eq207_e2598_d_n22];
        let eq207_branch_derivatives: [f64; 55] = [eq207_e2598_d_b0, eq207_e2598_d_b1, eq207_e2598_d_b2, eq207_e2598_d_b3, eq207_e2598_d_b4, eq207_e2598_d_b5, eq207_e2598_d_b6, eq207_e2598_d_b7, eq207_e2598_d_b8, eq207_e2598_d_b9, eq207_e2598_d_b10, eq207_e2598_d_b11, eq207_e2598_d_b12, eq207_e2598_d_b13, eq207_e2598_d_b14, eq207_e2598_d_b15, eq207_e2598_d_b16, eq207_e2598_d_b17, eq207_e2598_d_b18, eq207_e2598_d_b19, eq207_e2598_d_b20, eq207_e2598_d_b21, eq207_e2598_d_b22, eq207_e2598_d_b23, eq207_e2598_d_b24, eq207_e2598_d_b25, eq207_e2598_d_b26, eq207_e2598_d_b27, eq207_e2598_d_b28, eq207_e2598_d_b29, eq207_e2598_d_b30, eq207_e2598_d_b31, eq207_e2598_d_b32, eq207_e2598_d_b33, eq207_e2598_d_b34, eq207_e2598_d_b35, eq207_e2598_d_b36, eq207_e2598_d_b37, eq207_e2598_d_b38, eq207_e2598_d_b39, eq207_e2598_d_b40, eq207_e2598_d_b41, eq207_e2598_d_b42, eq207_e2598_d_b43, eq207_e2598_d_b44, eq207_e2598_d_b45, eq207_e2598_d_b46, eq207_e2598_d_b47, eq207_e2598_d_b48, eq207_e2598_d_b49, eq207_e2598_d_b50, eq207_e2598_d_b51, eq207_e2598_d_b52, eq207_e2598_d_b53, eq207_e2598_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(22),
            multiplicity * (eq207_value),
            &eq207_node_derivatives,
            &eq207_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_44(
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
        let (eq208_e2612, eq208_e2612_d_n0, eq208_e2612_d_n1, eq208_e2612_d_n2, eq208_e2612_d_n3, eq208_e2612_d_n4, eq208_e2612_d_n5, eq208_e2612_d_n6, eq208_e2612_d_n7, eq208_e2612_d_n8, eq208_e2612_d_n9, eq208_e2612_d_n10, eq208_e2612_d_n11, eq208_e2612_d_n12, eq208_e2612_d_n13, eq208_e2612_d_n14, eq208_e2612_d_n15, eq208_e2612_d_n16, eq208_e2612_d_n17, eq208_e2612_d_n18, eq208_e2612_d_n19, eq208_e2612_d_n20, eq208_e2612_d_n21, eq208_e2612_d_n22, eq208_e2612_d_b0, eq208_e2612_d_b1, eq208_e2612_d_b2, eq208_e2612_d_b3, eq208_e2612_d_b4, eq208_e2612_d_b5, eq208_e2612_d_b6, eq208_e2612_d_b7, eq208_e2612_d_b8, eq208_e2612_d_b9, eq208_e2612_d_b10, eq208_e2612_d_b11, eq208_e2612_d_b12, eq208_e2612_d_b13, eq208_e2612_d_b14, eq208_e2612_d_b15, eq208_e2612_d_b16, eq208_e2612_d_b17, eq208_e2612_d_b18, eq208_e2612_d_b19, eq208_e2612_d_b20, eq208_e2612_d_b21, eq208_e2612_d_b22, eq208_e2612_d_b23, eq208_e2612_d_b24, eq208_e2612_d_b25, eq208_e2612_d_b26, eq208_e2612_d_b27, eq208_e2612_d_b28, eq208_e2612_d_b29, eq208_e2612_d_b30, eq208_e2612_d_b31, eq208_e2612_d_b32, eq208_e2612_d_b33, eq208_e2612_d_b34, eq208_e2612_d_b35, eq208_e2612_d_b36, eq208_e2612_d_b37, eq208_e2612_d_b38, eq208_e2612_d_b39, eq208_e2612_d_b40, eq208_e2612_d_b41, eq208_e2612_d_b42, eq208_e2612_d_b43, eq208_e2612_d_b44, eq208_e2612_d_b45, eq208_e2612_d_b46, eq208_e2612_d_b47, eq208_e2612_d_b48, eq208_e2612_d_b49, eq208_e2612_d_b50, eq208_e2612_d_b51, eq208_e2612_d_b52, eq208_e2612_d_b53, eq208_e2612_d_b54,) = {
    if ((s.b[605] && s.b[606]) && (!s.b[607])) {
        let eq208_e2607: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 107, s.v[312]);
        let eq208_e2608: f64 = (p.p7 * eq208_e2607);
        let eq208_e2608_d_n0: f64 = (p.p7 * (s.dn[312][0] * ddt_scale));
        let eq208_e2608_d_n1: f64 = (p.p7 * (s.dn[312][1] * ddt_scale));
        let eq208_e2608_d_n2: f64 = (p.p7 * (s.dn[312][2] * ddt_scale));
        let eq208_e2608_d_n3: f64 = (p.p7 * (s.dn[312][3] * ddt_scale));
        let eq208_e2608_d_n4: f64 = (p.p7 * (s.dn[312][4] * ddt_scale));
        let eq208_e2608_d_n5: f64 = (p.p7 * (s.dn[312][5] * ddt_scale));
        let eq208_e2608_d_n6: f64 = (p.p7 * (s.dn[312][6] * ddt_scale));
        let eq208_e2608_d_n7: f64 = (p.p7 * (s.dn[312][7] * ddt_scale));
        let eq208_e2608_d_n8: f64 = (p.p7 * (s.dn[312][8] * ddt_scale));
        let eq208_e2608_d_n9: f64 = (p.p7 * (s.dn[312][9] * ddt_scale));
        let eq208_e2608_d_n10: f64 = (p.p7 * (s.dn[312][10] * ddt_scale));
        let eq208_e2608_d_n11: f64 = (p.p7 * (s.dn[312][11] * ddt_scale));
        let eq208_e2608_d_n12: f64 = (p.p7 * (s.dn[312][12] * ddt_scale));
        let eq208_e2608_d_n13: f64 = (p.p7 * (s.dn[312][13] * ddt_scale));
        let eq208_e2608_d_n14: f64 = (p.p7 * (s.dn[312][14] * ddt_scale));
        let eq208_e2608_d_n15: f64 = (p.p7 * (s.dn[312][15] * ddt_scale));
        let eq208_e2608_d_n16: f64 = (p.p7 * (s.dn[312][16] * ddt_scale));
        let eq208_e2608_d_n17: f64 = (p.p7 * (s.dn[312][17] * ddt_scale));
        let eq208_e2608_d_n18: f64 = (p.p7 * (s.dn[312][18] * ddt_scale));
        let eq208_e2608_d_n19: f64 = (p.p7 * (s.dn[312][19] * ddt_scale));
        let eq208_e2608_d_n20: f64 = (p.p7 * (s.dn[312][20] * ddt_scale));
        let eq208_e2608_d_n21: f64 = (p.p7 * (s.dn[312][21] * ddt_scale));
        let eq208_e2608_d_n22: f64 = (p.p7 * (s.dn[312][22] * ddt_scale));
        let eq208_e2608_d_b0: f64 = (p.p7 * (s.db[312][0] * ddt_scale));
        let eq208_e2608_d_b1: f64 = (p.p7 * (s.db[312][1] * ddt_scale));
        let eq208_e2608_d_b2: f64 = (p.p7 * (s.db[312][2] * ddt_scale));
        let eq208_e2608_d_b3: f64 = (p.p7 * (s.db[312][3] * ddt_scale));
        let eq208_e2608_d_b4: f64 = (p.p7 * (s.db[312][4] * ddt_scale));
        let eq208_e2608_d_b5: f64 = (p.p7 * (s.db[312][5] * ddt_scale));
        let eq208_e2608_d_b6: f64 = (p.p7 * (s.db[312][6] * ddt_scale));
        let eq208_e2608_d_b7: f64 = (p.p7 * (s.db[312][7] * ddt_scale));
        let eq208_e2608_d_b8: f64 = (p.p7 * (s.db[312][8] * ddt_scale));
        let eq208_e2608_d_b9: f64 = (p.p7 * (s.db[312][9] * ddt_scale));
        let eq208_e2608_d_b10: f64 = (p.p7 * (s.db[312][10] * ddt_scale));
        let eq208_e2608_d_b11: f64 = (p.p7 * (s.db[312][11] * ddt_scale));
        let eq208_e2608_d_b12: f64 = (p.p7 * (s.db[312][12] * ddt_scale));
        let eq208_e2608_d_b13: f64 = (p.p7 * (s.db[312][13] * ddt_scale));
        let eq208_e2608_d_b14: f64 = (p.p7 * (s.db[312][14] * ddt_scale));
        let eq208_e2608_d_b15: f64 = (p.p7 * (s.db[312][15] * ddt_scale));
        let eq208_e2608_d_b16: f64 = (p.p7 * (s.db[312][16] * ddt_scale));
        let eq208_e2608_d_b17: f64 = (p.p7 * (s.db[312][17] * ddt_scale));
        let eq208_e2608_d_b18: f64 = (p.p7 * (s.db[312][18] * ddt_scale));
        let eq208_e2608_d_b19: f64 = (p.p7 * (s.db[312][19] * ddt_scale));
        let eq208_e2608_d_b20: f64 = (p.p7 * (s.db[312][20] * ddt_scale));
        let eq208_e2608_d_b21: f64 = (p.p7 * (s.db[312][21] * ddt_scale));
        let eq208_e2608_d_b22: f64 = (p.p7 * (s.db[312][22] * ddt_scale));
        let eq208_e2608_d_b23: f64 = (p.p7 * (s.db[312][23] * ddt_scale));
        let eq208_e2608_d_b24: f64 = (p.p7 * (s.db[312][24] * ddt_scale));
        let eq208_e2608_d_b25: f64 = (p.p7 * (s.db[312][25] * ddt_scale));
        let eq208_e2608_d_b26: f64 = (p.p7 * (s.db[312][26] * ddt_scale));
        let eq208_e2608_d_b27: f64 = (p.p7 * (s.db[312][27] * ddt_scale));
        let eq208_e2608_d_b28: f64 = (p.p7 * (s.db[312][28] * ddt_scale));
        let eq208_e2608_d_b29: f64 = (p.p7 * (s.db[312][29] * ddt_scale));
        let eq208_e2608_d_b30: f64 = (p.p7 * (s.db[312][30] * ddt_scale));
        let eq208_e2608_d_b31: f64 = (p.p7 * (s.db[312][31] * ddt_scale));
        let eq208_e2608_d_b32: f64 = (p.p7 * (s.db[312][32] * ddt_scale));
        let eq208_e2608_d_b33: f64 = (p.p7 * (s.db[312][33] * ddt_scale));
        let eq208_e2608_d_b34: f64 = (p.p7 * (s.db[312][34] * ddt_scale));
        let eq208_e2608_d_b35: f64 = (p.p7 * (s.db[312][35] * ddt_scale));
        let eq208_e2608_d_b36: f64 = (p.p7 * (s.db[312][36] * ddt_scale));
        let eq208_e2608_d_b37: f64 = (p.p7 * (s.db[312][37] * ddt_scale));
        let eq208_e2608_d_b38: f64 = (p.p7 * (s.db[312][38] * ddt_scale));
        let eq208_e2608_d_b39: f64 = (p.p7 * (s.db[312][39] * ddt_scale));
        let eq208_e2608_d_b40: f64 = (p.p7 * (s.db[312][40] * ddt_scale));
        let eq208_e2608_d_b41: f64 = (p.p7 * (s.db[312][41] * ddt_scale));
        let eq208_e2608_d_b42: f64 = (p.p7 * (s.db[312][42] * ddt_scale));
        let eq208_e2608_d_b43: f64 = (p.p7 * (s.db[312][43] * ddt_scale));
        let eq208_e2608_d_b44: f64 = (p.p7 * (s.db[312][44] * ddt_scale));
        let eq208_e2608_d_b45: f64 = (p.p7 * (s.db[312][45] * ddt_scale));
        let eq208_e2608_d_b46: f64 = (p.p7 * (s.db[312][46] * ddt_scale));
        let eq208_e2608_d_b47: f64 = (p.p7 * (s.db[312][47] * ddt_scale));
        let eq208_e2608_d_b48: f64 = (p.p7 * (s.db[312][48] * ddt_scale));
        let eq208_e2608_d_b49: f64 = (p.p7 * (s.db[312][49] * ddt_scale));
        let eq208_e2608_d_b50: f64 = (p.p7 * (s.db[312][50] * ddt_scale));
        let eq208_e2608_d_b51: f64 = (p.p7 * (s.db[312][51] * ddt_scale));
        let eq208_e2608_d_b52: f64 = (p.p7 * (s.db[312][52] * ddt_scale));
        let eq208_e2608_d_b53: f64 = (p.p7 * (s.db[312][53] * ddt_scale));
        let eq208_e2608_d_b54: f64 = (p.p7 * (s.db[312][54] * ddt_scale));
        let eq208_e2610: f64 = (eq208_e2608 * p.p249);
        let eq208_e2610_d_n0: f64 = (eq208_e2608_d_n0 * p.p249);
        let eq208_e2610_d_n1: f64 = (eq208_e2608_d_n1 * p.p249);
        let eq208_e2610_d_n2: f64 = (eq208_e2608_d_n2 * p.p249);
        let eq208_e2610_d_n3: f64 = (eq208_e2608_d_n3 * p.p249);
        let eq208_e2610_d_n4: f64 = (eq208_e2608_d_n4 * p.p249);
        let eq208_e2610_d_n5: f64 = (eq208_e2608_d_n5 * p.p249);
        let eq208_e2610_d_n6: f64 = (eq208_e2608_d_n6 * p.p249);
        let eq208_e2610_d_n7: f64 = (eq208_e2608_d_n7 * p.p249);
        let eq208_e2610_d_n8: f64 = (eq208_e2608_d_n8 * p.p249);
        let eq208_e2610_d_n9: f64 = (eq208_e2608_d_n9 * p.p249);
        let eq208_e2610_d_n10: f64 = (eq208_e2608_d_n10 * p.p249);
        let eq208_e2610_d_n11: f64 = (eq208_e2608_d_n11 * p.p249);
        let eq208_e2610_d_n12: f64 = (eq208_e2608_d_n12 * p.p249);
        let eq208_e2610_d_n13: f64 = (eq208_e2608_d_n13 * p.p249);
        let eq208_e2610_d_n14: f64 = (eq208_e2608_d_n14 * p.p249);
        let eq208_e2610_d_n15: f64 = (eq208_e2608_d_n15 * p.p249);
        let eq208_e2610_d_n16: f64 = (eq208_e2608_d_n16 * p.p249);
        let eq208_e2610_d_n17: f64 = (eq208_e2608_d_n17 * p.p249);
        let eq208_e2610_d_n18: f64 = (eq208_e2608_d_n18 * p.p249);
        let eq208_e2610_d_n19: f64 = (eq208_e2608_d_n19 * p.p249);
        let eq208_e2610_d_n20: f64 = (eq208_e2608_d_n20 * p.p249);
        let eq208_e2610_d_n21: f64 = (eq208_e2608_d_n21 * p.p249);
        let eq208_e2610_d_n22: f64 = (eq208_e2608_d_n22 * p.p249);
        let eq208_e2610_d_b0: f64 = (eq208_e2608_d_b0 * p.p249);
        let eq208_e2610_d_b1: f64 = (eq208_e2608_d_b1 * p.p249);
        let eq208_e2610_d_b2: f64 = (eq208_e2608_d_b2 * p.p249);
        let eq208_e2610_d_b3: f64 = (eq208_e2608_d_b3 * p.p249);
        let eq208_e2610_d_b4: f64 = (eq208_e2608_d_b4 * p.p249);
        let eq208_e2610_d_b5: f64 = (eq208_e2608_d_b5 * p.p249);
        let eq208_e2610_d_b6: f64 = (eq208_e2608_d_b6 * p.p249);
        let eq208_e2610_d_b7: f64 = (eq208_e2608_d_b7 * p.p249);
        let eq208_e2610_d_b8: f64 = (eq208_e2608_d_b8 * p.p249);
        let eq208_e2610_d_b9: f64 = (eq208_e2608_d_b9 * p.p249);
        let eq208_e2610_d_b10: f64 = (eq208_e2608_d_b10 * p.p249);
        let eq208_e2610_d_b11: f64 = (eq208_e2608_d_b11 * p.p249);
        let eq208_e2610_d_b12: f64 = (eq208_e2608_d_b12 * p.p249);
        let eq208_e2610_d_b13: f64 = (eq208_e2608_d_b13 * p.p249);
        let eq208_e2610_d_b14: f64 = (eq208_e2608_d_b14 * p.p249);
        let eq208_e2610_d_b15: f64 = (eq208_e2608_d_b15 * p.p249);
        let eq208_e2610_d_b16: f64 = (eq208_e2608_d_b16 * p.p249);
        let eq208_e2610_d_b17: f64 = (eq208_e2608_d_b17 * p.p249);
        let eq208_e2610_d_b18: f64 = (eq208_e2608_d_b18 * p.p249);
        let eq208_e2610_d_b19: f64 = (eq208_e2608_d_b19 * p.p249);
        let eq208_e2610_d_b20: f64 = (eq208_e2608_d_b20 * p.p249);
        let eq208_e2610_d_b21: f64 = (eq208_e2608_d_b21 * p.p249);
        let eq208_e2610_d_b22: f64 = (eq208_e2608_d_b22 * p.p249);
        let eq208_e2610_d_b23: f64 = (eq208_e2608_d_b23 * p.p249);
        let eq208_e2610_d_b24: f64 = (eq208_e2608_d_b24 * p.p249);
        let eq208_e2610_d_b25: f64 = (eq208_e2608_d_b25 * p.p249);
        let eq208_e2610_d_b26: f64 = (eq208_e2608_d_b26 * p.p249);
        let eq208_e2610_d_b27: f64 = (eq208_e2608_d_b27 * p.p249);
        let eq208_e2610_d_b28: f64 = (eq208_e2608_d_b28 * p.p249);
        let eq208_e2610_d_b29: f64 = (eq208_e2608_d_b29 * p.p249);
        let eq208_e2610_d_b30: f64 = (eq208_e2608_d_b30 * p.p249);
        let eq208_e2610_d_b31: f64 = (eq208_e2608_d_b31 * p.p249);
        let eq208_e2610_d_b32: f64 = (eq208_e2608_d_b32 * p.p249);
        let eq208_e2610_d_b33: f64 = (eq208_e2608_d_b33 * p.p249);
        let eq208_e2610_d_b34: f64 = (eq208_e2608_d_b34 * p.p249);
        let eq208_e2610_d_b35: f64 = (eq208_e2608_d_b35 * p.p249);
        let eq208_e2610_d_b36: f64 = (eq208_e2608_d_b36 * p.p249);
        let eq208_e2610_d_b37: f64 = (eq208_e2608_d_b37 * p.p249);
        let eq208_e2610_d_b38: f64 = (eq208_e2608_d_b38 * p.p249);
        let eq208_e2610_d_b39: f64 = (eq208_e2608_d_b39 * p.p249);
        let eq208_e2610_d_b40: f64 = (eq208_e2608_d_b40 * p.p249);
        let eq208_e2610_d_b41: f64 = (eq208_e2608_d_b41 * p.p249);
        let eq208_e2610_d_b42: f64 = (eq208_e2608_d_b42 * p.p249);
        let eq208_e2610_d_b43: f64 = (eq208_e2608_d_b43 * p.p249);
        let eq208_e2610_d_b44: f64 = (eq208_e2608_d_b44 * p.p249);
        let eq208_e2610_d_b45: f64 = (eq208_e2608_d_b45 * p.p249);
        let eq208_e2610_d_b46: f64 = (eq208_e2608_d_b46 * p.p249);
        let eq208_e2610_d_b47: f64 = (eq208_e2608_d_b47 * p.p249);
        let eq208_e2610_d_b48: f64 = (eq208_e2608_d_b48 * p.p249);
        let eq208_e2610_d_b49: f64 = (eq208_e2608_d_b49 * p.p249);
        let eq208_e2610_d_b50: f64 = (eq208_e2608_d_b50 * p.p249);
        let eq208_e2610_d_b51: f64 = (eq208_e2608_d_b51 * p.p249);
        let eq208_e2610_d_b52: f64 = (eq208_e2608_d_b52 * p.p249);
        let eq208_e2610_d_b53: f64 = (eq208_e2608_d_b53 * p.p249);
        let eq208_e2610_d_b54: f64 = (eq208_e2608_d_b54 * p.p249);
        (eq208_e2610, eq208_e2610_d_n0, eq208_e2610_d_n1, eq208_e2610_d_n2, eq208_e2610_d_n3, eq208_e2610_d_n4, eq208_e2610_d_n5, eq208_e2610_d_n6, eq208_e2610_d_n7, eq208_e2610_d_n8, eq208_e2610_d_n9, eq208_e2610_d_n10, eq208_e2610_d_n11, eq208_e2610_d_n12, eq208_e2610_d_n13, eq208_e2610_d_n14, eq208_e2610_d_n15, eq208_e2610_d_n16, eq208_e2610_d_n17, eq208_e2610_d_n18, eq208_e2610_d_n19, eq208_e2610_d_n20, eq208_e2610_d_n21, eq208_e2610_d_n22, eq208_e2610_d_b0, eq208_e2610_d_b1, eq208_e2610_d_b2, eq208_e2610_d_b3, eq208_e2610_d_b4, eq208_e2610_d_b5, eq208_e2610_d_b6, eq208_e2610_d_b7, eq208_e2610_d_b8, eq208_e2610_d_b9, eq208_e2610_d_b10, eq208_e2610_d_b11, eq208_e2610_d_b12, eq208_e2610_d_b13, eq208_e2610_d_b14, eq208_e2610_d_b15, eq208_e2610_d_b16, eq208_e2610_d_b17, eq208_e2610_d_b18, eq208_e2610_d_b19, eq208_e2610_d_b20, eq208_e2610_d_b21, eq208_e2610_d_b22, eq208_e2610_d_b23, eq208_e2610_d_b24, eq208_e2610_d_b25, eq208_e2610_d_b26, eq208_e2610_d_b27, eq208_e2610_d_b28, eq208_e2610_d_b29, eq208_e2610_d_b30, eq208_e2610_d_b31, eq208_e2610_d_b32, eq208_e2610_d_b33, eq208_e2610_d_b34, eq208_e2610_d_b35, eq208_e2610_d_b36, eq208_e2610_d_b37, eq208_e2610_d_b38, eq208_e2610_d_b39, eq208_e2610_d_b40, eq208_e2610_d_b41, eq208_e2610_d_b42, eq208_e2610_d_b43, eq208_e2610_d_b44, eq208_e2610_d_b45, eq208_e2610_d_b46, eq208_e2610_d_b47, eq208_e2610_d_b48, eq208_e2610_d_b49, eq208_e2610_d_b50, eq208_e2610_d_b51, eq208_e2610_d_b52, eq208_e2610_d_b53, eq208_e2610_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq208_value: f64 = eq208_e2612;
        let eq208_node_derivatives: [f64; 23] = [eq208_e2612_d_n0, eq208_e2612_d_n1, eq208_e2612_d_n2, eq208_e2612_d_n3, eq208_e2612_d_n4, eq208_e2612_d_n5, eq208_e2612_d_n6, eq208_e2612_d_n7, eq208_e2612_d_n8, eq208_e2612_d_n9, eq208_e2612_d_n10, eq208_e2612_d_n11, eq208_e2612_d_n12, eq208_e2612_d_n13, eq208_e2612_d_n14, eq208_e2612_d_n15, eq208_e2612_d_n16, eq208_e2612_d_n17, eq208_e2612_d_n18, eq208_e2612_d_n19, eq208_e2612_d_n20, eq208_e2612_d_n21, eq208_e2612_d_n22];
        let eq208_branch_derivatives: [f64; 55] = [eq208_e2612_d_b0, eq208_e2612_d_b1, eq208_e2612_d_b2, eq208_e2612_d_b3, eq208_e2612_d_b4, eq208_e2612_d_b5, eq208_e2612_d_b6, eq208_e2612_d_b7, eq208_e2612_d_b8, eq208_e2612_d_b9, eq208_e2612_d_b10, eq208_e2612_d_b11, eq208_e2612_d_b12, eq208_e2612_d_b13, eq208_e2612_d_b14, eq208_e2612_d_b15, eq208_e2612_d_b16, eq208_e2612_d_b17, eq208_e2612_d_b18, eq208_e2612_d_b19, eq208_e2612_d_b20, eq208_e2612_d_b21, eq208_e2612_d_b22, eq208_e2612_d_b23, eq208_e2612_d_b24, eq208_e2612_d_b25, eq208_e2612_d_b26, eq208_e2612_d_b27, eq208_e2612_d_b28, eq208_e2612_d_b29, eq208_e2612_d_b30, eq208_e2612_d_b31, eq208_e2612_d_b32, eq208_e2612_d_b33, eq208_e2612_d_b34, eq208_e2612_d_b35, eq208_e2612_d_b36, eq208_e2612_d_b37, eq208_e2612_d_b38, eq208_e2612_d_b39, eq208_e2612_d_b40, eq208_e2612_d_b41, eq208_e2612_d_b42, eq208_e2612_d_b43, eq208_e2612_d_b44, eq208_e2612_d_b45, eq208_e2612_d_b46, eq208_e2612_d_b47, eq208_e2612_d_b48, eq208_e2612_d_b49, eq208_e2612_d_b50, eq208_e2612_d_b51, eq208_e2612_d_b52, eq208_e2612_d_b53, eq208_e2612_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(22),
            multiplicity * (eq208_value),
            &eq208_node_derivatives,
            &eq208_branch_derivatives,
            multiplicity,
        );
        let (eq209_e2623, eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, eq209_e2623_d_n10, eq209_e2623_d_n11, eq209_e2623_d_n12, eq209_e2623_d_n13, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22, eq209_e2623_d_b0, eq209_e2623_d_b1, eq209_e2623_d_b2, eq209_e2623_d_b3, eq209_e2623_d_b4, eq209_e2623_d_b5, eq209_e2623_d_b6, eq209_e2623_d_b7, eq209_e2623_d_b8, eq209_e2623_d_b9, eq209_e2623_d_b10, eq209_e2623_d_b11, eq209_e2623_d_b12, eq209_e2623_d_b13, eq209_e2623_d_b14, eq209_e2623_d_b15, eq209_e2623_d_b16, eq209_e2623_d_b17, eq209_e2623_d_b18, eq209_e2623_d_b19, eq209_e2623_d_b20, eq209_e2623_d_b21, eq209_e2623_d_b22, eq209_e2623_d_b23, eq209_e2623_d_b24, eq209_e2623_d_b25, eq209_e2623_d_b26, eq209_e2623_d_b27, eq209_e2623_d_b28, eq209_e2623_d_b29, eq209_e2623_d_b30, eq209_e2623_d_b31, eq209_e2623_d_b32, eq209_e2623_d_b33, eq209_e2623_d_b34, eq209_e2623_d_b35, eq209_e2623_d_b36, eq209_e2623_d_b37, eq209_e2623_d_b38, eq209_e2623_d_b39, eq209_e2623_d_b40, eq209_e2623_d_b41, eq209_e2623_d_b42, eq209_e2623_d_b43, eq209_e2623_d_b44, eq209_e2623_d_b45, eq209_e2623_d_b46, eq209_e2623_d_b47, eq209_e2623_d_b48, eq209_e2623_d_b49, eq209_e2623_d_b50, eq209_e2623_d_b51, eq209_e2623_d_b52, eq209_e2623_d_b53, eq209_e2623_d_b54,) = {
    if (s.b[605] && s.b[606]) {
        let eq209_e2619: f64 = (p.p254 * s.v[312]);
        let eq209_e2620: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 108, eq209_e2619);
        let eq209_e2620_d_n0: f64 = ((p.p254 * s.dn[312][0]) * ddt_scale);
        let eq209_e2620_d_n1: f64 = ((p.p254 * s.dn[312][1]) * ddt_scale);
        let eq209_e2620_d_n2: f64 = ((p.p254 * s.dn[312][2]) * ddt_scale);
        let eq209_e2620_d_n3: f64 = ((p.p254 * s.dn[312][3]) * ddt_scale);
        let eq209_e2620_d_n4: f64 = ((p.p254 * s.dn[312][4]) * ddt_scale);
        let eq209_e2620_d_n5: f64 = ((p.p254 * s.dn[312][5]) * ddt_scale);
        let eq209_e2620_d_n6: f64 = ((p.p254 * s.dn[312][6]) * ddt_scale);
        let eq209_e2620_d_n7: f64 = ((p.p254 * s.dn[312][7]) * ddt_scale);
        let eq209_e2620_d_n8: f64 = ((p.p254 * s.dn[312][8]) * ddt_scale);
        let eq209_e2620_d_n9: f64 = ((p.p254 * s.dn[312][9]) * ddt_scale);
        let eq209_e2620_d_n10: f64 = ((p.p254 * s.dn[312][10]) * ddt_scale);
        let eq209_e2620_d_n11: f64 = ((p.p254 * s.dn[312][11]) * ddt_scale);
        let eq209_e2620_d_n12: f64 = ((p.p254 * s.dn[312][12]) * ddt_scale);
        let eq209_e2620_d_n13: f64 = ((p.p254 * s.dn[312][13]) * ddt_scale);
        let eq209_e2620_d_n14: f64 = ((p.p254 * s.dn[312][14]) * ddt_scale);
        let eq209_e2620_d_n15: f64 = ((p.p254 * s.dn[312][15]) * ddt_scale);
        let eq209_e2620_d_n16: f64 = ((p.p254 * s.dn[312][16]) * ddt_scale);
        let eq209_e2620_d_n17: f64 = ((p.p254 * s.dn[312][17]) * ddt_scale);
        let eq209_e2620_d_n18: f64 = ((p.p254 * s.dn[312][18]) * ddt_scale);
        let eq209_e2620_d_n19: f64 = ((p.p254 * s.dn[312][19]) * ddt_scale);
        let eq209_e2620_d_n20: f64 = ((p.p254 * s.dn[312][20]) * ddt_scale);
        let eq209_e2620_d_n21: f64 = ((p.p254 * s.dn[312][21]) * ddt_scale);
        let eq209_e2620_d_n22: f64 = ((p.p254 * s.dn[312][22]) * ddt_scale);
        let eq209_e2620_d_b0: f64 = ((p.p254 * s.db[312][0]) * ddt_scale);
        let eq209_e2620_d_b1: f64 = ((p.p254 * s.db[312][1]) * ddt_scale);
        let eq209_e2620_d_b2: f64 = ((p.p254 * s.db[312][2]) * ddt_scale);
        let eq209_e2620_d_b3: f64 = ((p.p254 * s.db[312][3]) * ddt_scale);
        let eq209_e2620_d_b4: f64 = ((p.p254 * s.db[312][4]) * ddt_scale);
        let eq209_e2620_d_b5: f64 = ((p.p254 * s.db[312][5]) * ddt_scale);
        let eq209_e2620_d_b6: f64 = ((p.p254 * s.db[312][6]) * ddt_scale);
        let eq209_e2620_d_b7: f64 = ((p.p254 * s.db[312][7]) * ddt_scale);
        let eq209_e2620_d_b8: f64 = ((p.p254 * s.db[312][8]) * ddt_scale);
        let eq209_e2620_d_b9: f64 = ((p.p254 * s.db[312][9]) * ddt_scale);
        let eq209_e2620_d_b10: f64 = ((p.p254 * s.db[312][10]) * ddt_scale);
        let eq209_e2620_d_b11: f64 = ((p.p254 * s.db[312][11]) * ddt_scale);
        let eq209_e2620_d_b12: f64 = ((p.p254 * s.db[312][12]) * ddt_scale);
        let eq209_e2620_d_b13: f64 = ((p.p254 * s.db[312][13]) * ddt_scale);
        let eq209_e2620_d_b14: f64 = ((p.p254 * s.db[312][14]) * ddt_scale);
        let eq209_e2620_d_b15: f64 = ((p.p254 * s.db[312][15]) * ddt_scale);
        let eq209_e2620_d_b16: f64 = ((p.p254 * s.db[312][16]) * ddt_scale);
        let eq209_e2620_d_b17: f64 = ((p.p254 * s.db[312][17]) * ddt_scale);
        let eq209_e2620_d_b18: f64 = ((p.p254 * s.db[312][18]) * ddt_scale);
        let eq209_e2620_d_b19: f64 = ((p.p254 * s.db[312][19]) * ddt_scale);
        let eq209_e2620_d_b20: f64 = ((p.p254 * s.db[312][20]) * ddt_scale);
        let eq209_e2620_d_b21: f64 = ((p.p254 * s.db[312][21]) * ddt_scale);
        let eq209_e2620_d_b22: f64 = ((p.p254 * s.db[312][22]) * ddt_scale);
        let eq209_e2620_d_b23: f64 = ((p.p254 * s.db[312][23]) * ddt_scale);
        let eq209_e2620_d_b24: f64 = ((p.p254 * s.db[312][24]) * ddt_scale);
        let eq209_e2620_d_b25: f64 = ((p.p254 * s.db[312][25]) * ddt_scale);
        let eq209_e2620_d_b26: f64 = ((p.p254 * s.db[312][26]) * ddt_scale);
        let eq209_e2620_d_b27: f64 = ((p.p254 * s.db[312][27]) * ddt_scale);
        let eq209_e2620_d_b28: f64 = ((p.p254 * s.db[312][28]) * ddt_scale);
        let eq209_e2620_d_b29: f64 = ((p.p254 * s.db[312][29]) * ddt_scale);
        let eq209_e2620_d_b30: f64 = ((p.p254 * s.db[312][30]) * ddt_scale);
        let eq209_e2620_d_b31: f64 = ((p.p254 * s.db[312][31]) * ddt_scale);
        let eq209_e2620_d_b32: f64 = ((p.p254 * s.db[312][32]) * ddt_scale);
        let eq209_e2620_d_b33: f64 = ((p.p254 * s.db[312][33]) * ddt_scale);
        let eq209_e2620_d_b34: f64 = ((p.p254 * s.db[312][34]) * ddt_scale);
        let eq209_e2620_d_b35: f64 = ((p.p254 * s.db[312][35]) * ddt_scale);
        let eq209_e2620_d_b36: f64 = ((p.p254 * s.db[312][36]) * ddt_scale);
        let eq209_e2620_d_b37: f64 = ((p.p254 * s.db[312][37]) * ddt_scale);
        let eq209_e2620_d_b38: f64 = ((p.p254 * s.db[312][38]) * ddt_scale);
        let eq209_e2620_d_b39: f64 = ((p.p254 * s.db[312][39]) * ddt_scale);
        let eq209_e2620_d_b40: f64 = ((p.p254 * s.db[312][40]) * ddt_scale);
        let eq209_e2620_d_b41: f64 = ((p.p254 * s.db[312][41]) * ddt_scale);
        let eq209_e2620_d_b42: f64 = ((p.p254 * s.db[312][42]) * ddt_scale);
        let eq209_e2620_d_b43: f64 = ((p.p254 * s.db[312][43]) * ddt_scale);
        let eq209_e2620_d_b44: f64 = ((p.p254 * s.db[312][44]) * ddt_scale);
        let eq209_e2620_d_b45: f64 = ((p.p254 * s.db[312][45]) * ddt_scale);
        let eq209_e2620_d_b46: f64 = ((p.p254 * s.db[312][46]) * ddt_scale);
        let eq209_e2620_d_b47: f64 = ((p.p254 * s.db[312][47]) * ddt_scale);
        let eq209_e2620_d_b48: f64 = ((p.p254 * s.db[312][48]) * ddt_scale);
        let eq209_e2620_d_b49: f64 = ((p.p254 * s.db[312][49]) * ddt_scale);
        let eq209_e2620_d_b50: f64 = ((p.p254 * s.db[312][50]) * ddt_scale);
        let eq209_e2620_d_b51: f64 = ((p.p254 * s.db[312][51]) * ddt_scale);
        let eq209_e2620_d_b52: f64 = ((p.p254 * s.db[312][52]) * ddt_scale);
        let eq209_e2620_d_b53: f64 = ((p.p254 * s.db[312][53]) * ddt_scale);
        let eq209_e2620_d_b54: f64 = ((p.p254 * s.db[312][54]) * ddt_scale);
        let eq209_e2621: f64 = (p.p7 * eq209_e2620);
        let eq209_e2621_d_n0: f64 = (p.p7 * eq209_e2620_d_n0);
        let eq209_e2621_d_n1: f64 = (p.p7 * eq209_e2620_d_n1);
        let eq209_e2621_d_n2: f64 = (p.p7 * eq209_e2620_d_n2);
        let eq209_e2621_d_n3: f64 = (p.p7 * eq209_e2620_d_n3);
        let eq209_e2621_d_n4: f64 = (p.p7 * eq209_e2620_d_n4);
        let eq209_e2621_d_n5: f64 = (p.p7 * eq209_e2620_d_n5);
        let eq209_e2621_d_n6: f64 = (p.p7 * eq209_e2620_d_n6);
        let eq209_e2621_d_n7: f64 = (p.p7 * eq209_e2620_d_n7);
        let eq209_e2621_d_n8: f64 = (p.p7 * eq209_e2620_d_n8);
        let eq209_e2621_d_n9: f64 = (p.p7 * eq209_e2620_d_n9);
        let eq209_e2621_d_n10: f64 = (p.p7 * eq209_e2620_d_n10);
        let eq209_e2621_d_n11: f64 = (p.p7 * eq209_e2620_d_n11);
        let eq209_e2621_d_n12: f64 = (p.p7 * eq209_e2620_d_n12);
        let eq209_e2621_d_n13: f64 = (p.p7 * eq209_e2620_d_n13);
        let eq209_e2621_d_n14: f64 = (p.p7 * eq209_e2620_d_n14);
        let eq209_e2621_d_n15: f64 = (p.p7 * eq209_e2620_d_n15);
        let eq209_e2621_d_n16: f64 = (p.p7 * eq209_e2620_d_n16);
        let eq209_e2621_d_n17: f64 = (p.p7 * eq209_e2620_d_n17);
        let eq209_e2621_d_n18: f64 = (p.p7 * eq209_e2620_d_n18);
        let eq209_e2621_d_n19: f64 = (p.p7 * eq209_e2620_d_n19);
        let eq209_e2621_d_n20: f64 = (p.p7 * eq209_e2620_d_n20);
        let eq209_e2621_d_n21: f64 = (p.p7 * eq209_e2620_d_n21);
        let eq209_e2621_d_n22: f64 = (p.p7 * eq209_e2620_d_n22);
        let eq209_e2621_d_b0: f64 = (p.p7 * eq209_e2620_d_b0);
        let eq209_e2621_d_b1: f64 = (p.p7 * eq209_e2620_d_b1);
        let eq209_e2621_d_b2: f64 = (p.p7 * eq209_e2620_d_b2);
        let eq209_e2621_d_b3: f64 = (p.p7 * eq209_e2620_d_b3);
        let eq209_e2621_d_b4: f64 = (p.p7 * eq209_e2620_d_b4);
        let eq209_e2621_d_b5: f64 = (p.p7 * eq209_e2620_d_b5);
        let eq209_e2621_d_b6: f64 = (p.p7 * eq209_e2620_d_b6);
        let eq209_e2621_d_b7: f64 = (p.p7 * eq209_e2620_d_b7);
        let eq209_e2621_d_b8: f64 = (p.p7 * eq209_e2620_d_b8);
        let eq209_e2621_d_b9: f64 = (p.p7 * eq209_e2620_d_b9);
        let eq209_e2621_d_b10: f64 = (p.p7 * eq209_e2620_d_b10);
        let eq209_e2621_d_b11: f64 = (p.p7 * eq209_e2620_d_b11);
        let eq209_e2621_d_b12: f64 = (p.p7 * eq209_e2620_d_b12);
        let eq209_e2621_d_b13: f64 = (p.p7 * eq209_e2620_d_b13);
        let eq209_e2621_d_b14: f64 = (p.p7 * eq209_e2620_d_b14);
        let eq209_e2621_d_b15: f64 = (p.p7 * eq209_e2620_d_b15);
        let eq209_e2621_d_b16: f64 = (p.p7 * eq209_e2620_d_b16);
        let eq209_e2621_d_b17: f64 = (p.p7 * eq209_e2620_d_b17);
        let eq209_e2621_d_b18: f64 = (p.p7 * eq209_e2620_d_b18);
        let eq209_e2621_d_b19: f64 = (p.p7 * eq209_e2620_d_b19);
        let eq209_e2621_d_b20: f64 = (p.p7 * eq209_e2620_d_b20);
        let eq209_e2621_d_b21: f64 = (p.p7 * eq209_e2620_d_b21);
        let eq209_e2621_d_b22: f64 = (p.p7 * eq209_e2620_d_b22);
        let eq209_e2621_d_b23: f64 = (p.p7 * eq209_e2620_d_b23);
        let eq209_e2621_d_b24: f64 = (p.p7 * eq209_e2620_d_b24);
        let eq209_e2621_d_b25: f64 = (p.p7 * eq209_e2620_d_b25);
        let eq209_e2621_d_b26: f64 = (p.p7 * eq209_e2620_d_b26);
        let eq209_e2621_d_b27: f64 = (p.p7 * eq209_e2620_d_b27);
        let eq209_e2621_d_b28: f64 = (p.p7 * eq209_e2620_d_b28);
        let eq209_e2621_d_b29: f64 = (p.p7 * eq209_e2620_d_b29);
        let eq209_e2621_d_b30: f64 = (p.p7 * eq209_e2620_d_b30);
        let eq209_e2621_d_b31: f64 = (p.p7 * eq209_e2620_d_b31);
        let eq209_e2621_d_b32: f64 = (p.p7 * eq209_e2620_d_b32);
        let eq209_e2621_d_b33: f64 = (p.p7 * eq209_e2620_d_b33);
        let eq209_e2621_d_b34: f64 = (p.p7 * eq209_e2620_d_b34);
        let eq209_e2621_d_b35: f64 = (p.p7 * eq209_e2620_d_b35);
        let eq209_e2621_d_b36: f64 = (p.p7 * eq209_e2620_d_b36);
        let eq209_e2621_d_b37: f64 = (p.p7 * eq209_e2620_d_b37);
        let eq209_e2621_d_b38: f64 = (p.p7 * eq209_e2620_d_b38);
        let eq209_e2621_d_b39: f64 = (p.p7 * eq209_e2620_d_b39);
        let eq209_e2621_d_b40: f64 = (p.p7 * eq209_e2620_d_b40);
        let eq209_e2621_d_b41: f64 = (p.p7 * eq209_e2620_d_b41);
        let eq209_e2621_d_b42: f64 = (p.p7 * eq209_e2620_d_b42);
        let eq209_e2621_d_b43: f64 = (p.p7 * eq209_e2620_d_b43);
        let eq209_e2621_d_b44: f64 = (p.p7 * eq209_e2620_d_b44);
        let eq209_e2621_d_b45: f64 = (p.p7 * eq209_e2620_d_b45);
        let eq209_e2621_d_b46: f64 = (p.p7 * eq209_e2620_d_b46);
        let eq209_e2621_d_b47: f64 = (p.p7 * eq209_e2620_d_b47);
        let eq209_e2621_d_b48: f64 = (p.p7 * eq209_e2620_d_b48);
        let eq209_e2621_d_b49: f64 = (p.p7 * eq209_e2620_d_b49);
        let eq209_e2621_d_b50: f64 = (p.p7 * eq209_e2620_d_b50);
        let eq209_e2621_d_b51: f64 = (p.p7 * eq209_e2620_d_b51);
        let eq209_e2621_d_b52: f64 = (p.p7 * eq209_e2620_d_b52);
        let eq209_e2621_d_b53: f64 = (p.p7 * eq209_e2620_d_b53);
        let eq209_e2621_d_b54: f64 = (p.p7 * eq209_e2620_d_b54);
        (eq209_e2621, eq209_e2621_d_n0, eq209_e2621_d_n1, eq209_e2621_d_n2, eq209_e2621_d_n3, eq209_e2621_d_n4, eq209_e2621_d_n5, eq209_e2621_d_n6, eq209_e2621_d_n7, eq209_e2621_d_n8, eq209_e2621_d_n9, eq209_e2621_d_n10, eq209_e2621_d_n11, eq209_e2621_d_n12, eq209_e2621_d_n13, eq209_e2621_d_n14, eq209_e2621_d_n15, eq209_e2621_d_n16, eq209_e2621_d_n17, eq209_e2621_d_n18, eq209_e2621_d_n19, eq209_e2621_d_n20, eq209_e2621_d_n21, eq209_e2621_d_n22, eq209_e2621_d_b0, eq209_e2621_d_b1, eq209_e2621_d_b2, eq209_e2621_d_b3, eq209_e2621_d_b4, eq209_e2621_d_b5, eq209_e2621_d_b6, eq209_e2621_d_b7, eq209_e2621_d_b8, eq209_e2621_d_b9, eq209_e2621_d_b10, eq209_e2621_d_b11, eq209_e2621_d_b12, eq209_e2621_d_b13, eq209_e2621_d_b14, eq209_e2621_d_b15, eq209_e2621_d_b16, eq209_e2621_d_b17, eq209_e2621_d_b18, eq209_e2621_d_b19, eq209_e2621_d_b20, eq209_e2621_d_b21, eq209_e2621_d_b22, eq209_e2621_d_b23, eq209_e2621_d_b24, eq209_e2621_d_b25, eq209_e2621_d_b26, eq209_e2621_d_b27, eq209_e2621_d_b28, eq209_e2621_d_b29, eq209_e2621_d_b30, eq209_e2621_d_b31, eq209_e2621_d_b32, eq209_e2621_d_b33, eq209_e2621_d_b34, eq209_e2621_d_b35, eq209_e2621_d_b36, eq209_e2621_d_b37, eq209_e2621_d_b38, eq209_e2621_d_b39, eq209_e2621_d_b40, eq209_e2621_d_b41, eq209_e2621_d_b42, eq209_e2621_d_b43, eq209_e2621_d_b44, eq209_e2621_d_b45, eq209_e2621_d_b46, eq209_e2621_d_b47, eq209_e2621_d_b48, eq209_e2621_d_b49, eq209_e2621_d_b50, eq209_e2621_d_b51, eq209_e2621_d_b52, eq209_e2621_d_b53, eq209_e2621_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq209_value: f64 = eq209_e2623;
        let eq209_node_derivatives: [f64; 23] = [eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, eq209_e2623_d_n10, eq209_e2623_d_n11, eq209_e2623_d_n12, eq209_e2623_d_n13, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22];
        let eq209_branch_derivatives: [f64; 55] = [eq209_e2623_d_b0, eq209_e2623_d_b1, eq209_e2623_d_b2, eq209_e2623_d_b3, eq209_e2623_d_b4, eq209_e2623_d_b5, eq209_e2623_d_b6, eq209_e2623_d_b7, eq209_e2623_d_b8, eq209_e2623_d_b9, eq209_e2623_d_b10, eq209_e2623_d_b11, eq209_e2623_d_b12, eq209_e2623_d_b13, eq209_e2623_d_b14, eq209_e2623_d_b15, eq209_e2623_d_b16, eq209_e2623_d_b17, eq209_e2623_d_b18, eq209_e2623_d_b19, eq209_e2623_d_b20, eq209_e2623_d_b21, eq209_e2623_d_b22, eq209_e2623_d_b23, eq209_e2623_d_b24, eq209_e2623_d_b25, eq209_e2623_d_b26, eq209_e2623_d_b27, eq209_e2623_d_b28, eq209_e2623_d_b29, eq209_e2623_d_b30, eq209_e2623_d_b31, eq209_e2623_d_b32, eq209_e2623_d_b33, eq209_e2623_d_b34, eq209_e2623_d_b35, eq209_e2623_d_b36, eq209_e2623_d_b37, eq209_e2623_d_b38, eq209_e2623_d_b39, eq209_e2623_d_b40, eq209_e2623_d_b41, eq209_e2623_d_b42, eq209_e2623_d_b43, eq209_e2623_d_b44, eq209_e2623_d_b45, eq209_e2623_d_b46, eq209_e2623_d_b47, eq209_e2623_d_b48, eq209_e2623_d_b49, eq209_e2623_d_b50, eq209_e2623_d_b51, eq209_e2623_d_b52, eq209_e2623_d_b53, eq209_e2623_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(22),
            multiplicity * (eq209_value),
            &eq209_node_derivatives,
            &eq209_branch_derivatives,
            multiplicity,
        );
        let (eq210_e2633, eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, eq210_e2633_d_n10, eq210_e2633_d_n11, eq210_e2633_d_n12, eq210_e2633_d_n13, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22, eq210_e2633_d_b0, eq210_e2633_d_b1, eq210_e2633_d_b2, eq210_e2633_d_b3, eq210_e2633_d_b4, eq210_e2633_d_b5, eq210_e2633_d_b6, eq210_e2633_d_b7, eq210_e2633_d_b8, eq210_e2633_d_b9, eq210_e2633_d_b10, eq210_e2633_d_b11, eq210_e2633_d_b12, eq210_e2633_d_b13, eq210_e2633_d_b14, eq210_e2633_d_b15, eq210_e2633_d_b16, eq210_e2633_d_b17, eq210_e2633_d_b18, eq210_e2633_d_b19, eq210_e2633_d_b20, eq210_e2633_d_b21, eq210_e2633_d_b22, eq210_e2633_d_b23, eq210_e2633_d_b24, eq210_e2633_d_b25, eq210_e2633_d_b26, eq210_e2633_d_b27, eq210_e2633_d_b28, eq210_e2633_d_b29, eq210_e2633_d_b30, eq210_e2633_d_b31, eq210_e2633_d_b32, eq210_e2633_d_b33, eq210_e2633_d_b34, eq210_e2633_d_b35, eq210_e2633_d_b36, eq210_e2633_d_b37, eq210_e2633_d_b38, eq210_e2633_d_b39, eq210_e2633_d_b40, eq210_e2633_d_b41, eq210_e2633_d_b42, eq210_e2633_d_b43, eq210_e2633_d_b44, eq210_e2633_d_b45, eq210_e2633_d_b46, eq210_e2633_d_b47, eq210_e2633_d_b48, eq210_e2633_d_b49, eq210_e2633_d_b50, eq210_e2633_d_b51, eq210_e2633_d_b52, eq210_e2633_d_b53, eq210_e2633_d_b54,) = {
    if ((!s.b[605]) && s.b[608]) {
        let eq210_e2630: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 109, s.v[313]);
        let eq210_e2631: f64 = (p.p7 * eq210_e2630);
        let eq210_e2631_d_n0: f64 = (p.p7 * (s.dn[313][0] * ddt_scale));
        let eq210_e2631_d_n1: f64 = (p.p7 * (s.dn[313][1] * ddt_scale));
        let eq210_e2631_d_n2: f64 = (p.p7 * (s.dn[313][2] * ddt_scale));
        let eq210_e2631_d_n3: f64 = (p.p7 * (s.dn[313][3] * ddt_scale));
        let eq210_e2631_d_n4: f64 = (p.p7 * (s.dn[313][4] * ddt_scale));
        let eq210_e2631_d_n5: f64 = (p.p7 * (s.dn[313][5] * ddt_scale));
        let eq210_e2631_d_n6: f64 = (p.p7 * (s.dn[313][6] * ddt_scale));
        let eq210_e2631_d_n7: f64 = (p.p7 * (s.dn[313][7] * ddt_scale));
        let eq210_e2631_d_n8: f64 = (p.p7 * (s.dn[313][8] * ddt_scale));
        let eq210_e2631_d_n9: f64 = (p.p7 * (s.dn[313][9] * ddt_scale));
        let eq210_e2631_d_n10: f64 = (p.p7 * (s.dn[313][10] * ddt_scale));
        let eq210_e2631_d_n11: f64 = (p.p7 * (s.dn[313][11] * ddt_scale));
        let eq210_e2631_d_n12: f64 = (p.p7 * (s.dn[313][12] * ddt_scale));
        let eq210_e2631_d_n13: f64 = (p.p7 * (s.dn[313][13] * ddt_scale));
        let eq210_e2631_d_n14: f64 = (p.p7 * (s.dn[313][14] * ddt_scale));
        let eq210_e2631_d_n15: f64 = (p.p7 * (s.dn[313][15] * ddt_scale));
        let eq210_e2631_d_n16: f64 = (p.p7 * (s.dn[313][16] * ddt_scale));
        let eq210_e2631_d_n17: f64 = (p.p7 * (s.dn[313][17] * ddt_scale));
        let eq210_e2631_d_n18: f64 = (p.p7 * (s.dn[313][18] * ddt_scale));
        let eq210_e2631_d_n19: f64 = (p.p7 * (s.dn[313][19] * ddt_scale));
        let eq210_e2631_d_n20: f64 = (p.p7 * (s.dn[313][20] * ddt_scale));
        let eq210_e2631_d_n21: f64 = (p.p7 * (s.dn[313][21] * ddt_scale));
        let eq210_e2631_d_n22: f64 = (p.p7 * (s.dn[313][22] * ddt_scale));
        let eq210_e2631_d_b0: f64 = (p.p7 * (s.db[313][0] * ddt_scale));
        let eq210_e2631_d_b1: f64 = (p.p7 * (s.db[313][1] * ddt_scale));
        let eq210_e2631_d_b2: f64 = (p.p7 * (s.db[313][2] * ddt_scale));
        let eq210_e2631_d_b3: f64 = (p.p7 * (s.db[313][3] * ddt_scale));
        let eq210_e2631_d_b4: f64 = (p.p7 * (s.db[313][4] * ddt_scale));
        let eq210_e2631_d_b5: f64 = (p.p7 * (s.db[313][5] * ddt_scale));
        let eq210_e2631_d_b6: f64 = (p.p7 * (s.db[313][6] * ddt_scale));
        let eq210_e2631_d_b7: f64 = (p.p7 * (s.db[313][7] * ddt_scale));
        let eq210_e2631_d_b8: f64 = (p.p7 * (s.db[313][8] * ddt_scale));
        let eq210_e2631_d_b9: f64 = (p.p7 * (s.db[313][9] * ddt_scale));
        let eq210_e2631_d_b10: f64 = (p.p7 * (s.db[313][10] * ddt_scale));
        let eq210_e2631_d_b11: f64 = (p.p7 * (s.db[313][11] * ddt_scale));
        let eq210_e2631_d_b12: f64 = (p.p7 * (s.db[313][12] * ddt_scale));
        let eq210_e2631_d_b13: f64 = (p.p7 * (s.db[313][13] * ddt_scale));
        let eq210_e2631_d_b14: f64 = (p.p7 * (s.db[313][14] * ddt_scale));
        let eq210_e2631_d_b15: f64 = (p.p7 * (s.db[313][15] * ddt_scale));
        let eq210_e2631_d_b16: f64 = (p.p7 * (s.db[313][16] * ddt_scale));
        let eq210_e2631_d_b17: f64 = (p.p7 * (s.db[313][17] * ddt_scale));
        let eq210_e2631_d_b18: f64 = (p.p7 * (s.db[313][18] * ddt_scale));
        let eq210_e2631_d_b19: f64 = (p.p7 * (s.db[313][19] * ddt_scale));
        let eq210_e2631_d_b20: f64 = (p.p7 * (s.db[313][20] * ddt_scale));
        let eq210_e2631_d_b21: f64 = (p.p7 * (s.db[313][21] * ddt_scale));
        let eq210_e2631_d_b22: f64 = (p.p7 * (s.db[313][22] * ddt_scale));
        let eq210_e2631_d_b23: f64 = (p.p7 * (s.db[313][23] * ddt_scale));
        let eq210_e2631_d_b24: f64 = (p.p7 * (s.db[313][24] * ddt_scale));
        let eq210_e2631_d_b25: f64 = (p.p7 * (s.db[313][25] * ddt_scale));
        let eq210_e2631_d_b26: f64 = (p.p7 * (s.db[313][26] * ddt_scale));
        let eq210_e2631_d_b27: f64 = (p.p7 * (s.db[313][27] * ddt_scale));
        let eq210_e2631_d_b28: f64 = (p.p7 * (s.db[313][28] * ddt_scale));
        let eq210_e2631_d_b29: f64 = (p.p7 * (s.db[313][29] * ddt_scale));
        let eq210_e2631_d_b30: f64 = (p.p7 * (s.db[313][30] * ddt_scale));
        let eq210_e2631_d_b31: f64 = (p.p7 * (s.db[313][31] * ddt_scale));
        let eq210_e2631_d_b32: f64 = (p.p7 * (s.db[313][32] * ddt_scale));
        let eq210_e2631_d_b33: f64 = (p.p7 * (s.db[313][33] * ddt_scale));
        let eq210_e2631_d_b34: f64 = (p.p7 * (s.db[313][34] * ddt_scale));
        let eq210_e2631_d_b35: f64 = (p.p7 * (s.db[313][35] * ddt_scale));
        let eq210_e2631_d_b36: f64 = (p.p7 * (s.db[313][36] * ddt_scale));
        let eq210_e2631_d_b37: f64 = (p.p7 * (s.db[313][37] * ddt_scale));
        let eq210_e2631_d_b38: f64 = (p.p7 * (s.db[313][38] * ddt_scale));
        let eq210_e2631_d_b39: f64 = (p.p7 * (s.db[313][39] * ddt_scale));
        let eq210_e2631_d_b40: f64 = (p.p7 * (s.db[313][40] * ddt_scale));
        let eq210_e2631_d_b41: f64 = (p.p7 * (s.db[313][41] * ddt_scale));
        let eq210_e2631_d_b42: f64 = (p.p7 * (s.db[313][42] * ddt_scale));
        let eq210_e2631_d_b43: f64 = (p.p7 * (s.db[313][43] * ddt_scale));
        let eq210_e2631_d_b44: f64 = (p.p7 * (s.db[313][44] * ddt_scale));
        let eq210_e2631_d_b45: f64 = (p.p7 * (s.db[313][45] * ddt_scale));
        let eq210_e2631_d_b46: f64 = (p.p7 * (s.db[313][46] * ddt_scale));
        let eq210_e2631_d_b47: f64 = (p.p7 * (s.db[313][47] * ddt_scale));
        let eq210_e2631_d_b48: f64 = (p.p7 * (s.db[313][48] * ddt_scale));
        let eq210_e2631_d_b49: f64 = (p.p7 * (s.db[313][49] * ddt_scale));
        let eq210_e2631_d_b50: f64 = (p.p7 * (s.db[313][50] * ddt_scale));
        let eq210_e2631_d_b51: f64 = (p.p7 * (s.db[313][51] * ddt_scale));
        let eq210_e2631_d_b52: f64 = (p.p7 * (s.db[313][52] * ddt_scale));
        let eq210_e2631_d_b53: f64 = (p.p7 * (s.db[313][53] * ddt_scale));
        let eq210_e2631_d_b54: f64 = (p.p7 * (s.db[313][54] * ddt_scale));
        (eq210_e2631, eq210_e2631_d_n0, eq210_e2631_d_n1, eq210_e2631_d_n2, eq210_e2631_d_n3, eq210_e2631_d_n4, eq210_e2631_d_n5, eq210_e2631_d_n6, eq210_e2631_d_n7, eq210_e2631_d_n8, eq210_e2631_d_n9, eq210_e2631_d_n10, eq210_e2631_d_n11, eq210_e2631_d_n12, eq210_e2631_d_n13, eq210_e2631_d_n14, eq210_e2631_d_n15, eq210_e2631_d_n16, eq210_e2631_d_n17, eq210_e2631_d_n18, eq210_e2631_d_n19, eq210_e2631_d_n20, eq210_e2631_d_n21, eq210_e2631_d_n22, eq210_e2631_d_b0, eq210_e2631_d_b1, eq210_e2631_d_b2, eq210_e2631_d_b3, eq210_e2631_d_b4, eq210_e2631_d_b5, eq210_e2631_d_b6, eq210_e2631_d_b7, eq210_e2631_d_b8, eq210_e2631_d_b9, eq210_e2631_d_b10, eq210_e2631_d_b11, eq210_e2631_d_b12, eq210_e2631_d_b13, eq210_e2631_d_b14, eq210_e2631_d_b15, eq210_e2631_d_b16, eq210_e2631_d_b17, eq210_e2631_d_b18, eq210_e2631_d_b19, eq210_e2631_d_b20, eq210_e2631_d_b21, eq210_e2631_d_b22, eq210_e2631_d_b23, eq210_e2631_d_b24, eq210_e2631_d_b25, eq210_e2631_d_b26, eq210_e2631_d_b27, eq210_e2631_d_b28, eq210_e2631_d_b29, eq210_e2631_d_b30, eq210_e2631_d_b31, eq210_e2631_d_b32, eq210_e2631_d_b33, eq210_e2631_d_b34, eq210_e2631_d_b35, eq210_e2631_d_b36, eq210_e2631_d_b37, eq210_e2631_d_b38, eq210_e2631_d_b39, eq210_e2631_d_b40, eq210_e2631_d_b41, eq210_e2631_d_b42, eq210_e2631_d_b43, eq210_e2631_d_b44, eq210_e2631_d_b45, eq210_e2631_d_b46, eq210_e2631_d_b47, eq210_e2631_d_b48, eq210_e2631_d_b49, eq210_e2631_d_b50, eq210_e2631_d_b51, eq210_e2631_d_b52, eq210_e2631_d_b53, eq210_e2631_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq210_value: f64 = eq210_e2633;
        let eq210_node_derivatives: [f64; 23] = [eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, eq210_e2633_d_n10, eq210_e2633_d_n11, eq210_e2633_d_n12, eq210_e2633_d_n13, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22];
        let eq210_branch_derivatives: [f64; 55] = [eq210_e2633_d_b0, eq210_e2633_d_b1, eq210_e2633_d_b2, eq210_e2633_d_b3, eq210_e2633_d_b4, eq210_e2633_d_b5, eq210_e2633_d_b6, eq210_e2633_d_b7, eq210_e2633_d_b8, eq210_e2633_d_b9, eq210_e2633_d_b10, eq210_e2633_d_b11, eq210_e2633_d_b12, eq210_e2633_d_b13, eq210_e2633_d_b14, eq210_e2633_d_b15, eq210_e2633_d_b16, eq210_e2633_d_b17, eq210_e2633_d_b18, eq210_e2633_d_b19, eq210_e2633_d_b20, eq210_e2633_d_b21, eq210_e2633_d_b22, eq210_e2633_d_b23, eq210_e2633_d_b24, eq210_e2633_d_b25, eq210_e2633_d_b26, eq210_e2633_d_b27, eq210_e2633_d_b28, eq210_e2633_d_b29, eq210_e2633_d_b30, eq210_e2633_d_b31, eq210_e2633_d_b32, eq210_e2633_d_b33, eq210_e2633_d_b34, eq210_e2633_d_b35, eq210_e2633_d_b36, eq210_e2633_d_b37, eq210_e2633_d_b38, eq210_e2633_d_b39, eq210_e2633_d_b40, eq210_e2633_d_b41, eq210_e2633_d_b42, eq210_e2633_d_b43, eq210_e2633_d_b44, eq210_e2633_d_b45, eq210_e2633_d_b46, eq210_e2633_d_b47, eq210_e2633_d_b48, eq210_e2633_d_b49, eq210_e2633_d_b50, eq210_e2633_d_b51, eq210_e2633_d_b52, eq210_e2633_d_b53, eq210_e2633_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq210_value),
            &eq210_node_derivatives,
            &eq210_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_45(
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[312][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[312][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[312][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[312][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[312][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[312][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[312][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[312][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[312][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[312][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[312][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[312][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[312][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[312][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[312][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[312][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[312][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[312][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[312][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[312][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[312][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[312][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[312][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[312][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[312][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[312][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[312][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[312][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[312][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[312][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[312][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[312][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[312][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[312][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[312][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[312][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[312][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[312][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[312][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[312][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[312][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[312][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[312][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[312][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[312][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[312][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[312][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[312][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[312][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[312][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[312][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[312][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[312][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[312][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[312][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[312][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[312][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[312][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[312][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[312][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[312][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[312][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[312][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[312][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[312][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[312][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[312][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[312][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[312][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[312][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[312][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[312][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[312][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[312][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[312][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[312][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[312][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[312][54] * ddt_scale));
        let (eq211_e2645, eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, eq211_e2645_d_n10, eq211_e2645_d_n11, eq211_e2645_d_n12, eq211_e2645_d_n13, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22, eq211_e2645_d_b0, eq211_e2645_d_b1, eq211_e2645_d_b2, eq211_e2645_d_b3, eq211_e2645_d_b4, eq211_e2645_d_b5, eq211_e2645_d_b6, eq211_e2645_d_b7, eq211_e2645_d_b8, eq211_e2645_d_b9, eq211_e2645_d_b10, eq211_e2645_d_b11, eq211_e2645_d_b12, eq211_e2645_d_b13, eq211_e2645_d_b14, eq211_e2645_d_b15, eq211_e2645_d_b16, eq211_e2645_d_b17, eq211_e2645_d_b18, eq211_e2645_d_b19, eq211_e2645_d_b20, eq211_e2645_d_b21, eq211_e2645_d_b22, eq211_e2645_d_b23, eq211_e2645_d_b24, eq211_e2645_d_b25, eq211_e2645_d_b26, eq211_e2645_d_b27, eq211_e2645_d_b28, eq211_e2645_d_b29, eq211_e2645_d_b30, eq211_e2645_d_b31, eq211_e2645_d_b32, eq211_e2645_d_b33, eq211_e2645_d_b34, eq211_e2645_d_b35, eq211_e2645_d_b36, eq211_e2645_d_b37, eq211_e2645_d_b38, eq211_e2645_d_b39, eq211_e2645_d_b40, eq211_e2645_d_b41, eq211_e2645_d_b42, eq211_e2645_d_b43, eq211_e2645_d_b44, eq211_e2645_d_b45, eq211_e2645_d_b46, eq211_e2645_d_b47, eq211_e2645_d_b48, eq211_e2645_d_b49, eq211_e2645_d_b50, eq211_e2645_d_b51, eq211_e2645_d_b52, eq211_e2645_d_b53, eq211_e2645_d_b54,) = {
    if (((!s.b[605]) && s.b[608]) && s.b[609]) {
        let eq211_e2642: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 110, s.v[312]);
        let eq211_e2643: f64 = (p.p7 * eq211_e2642);
        (eq211_e2643, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq211_value: f64 = eq211_e2645;
        let eq211_node_derivatives: [f64; 23] = [eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, eq211_e2645_d_n10, eq211_e2645_d_n11, eq211_e2645_d_n12, eq211_e2645_d_n13, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22];
        let eq211_branch_derivatives: [f64; 55] = [eq211_e2645_d_b0, eq211_e2645_d_b1, eq211_e2645_d_b2, eq211_e2645_d_b3, eq211_e2645_d_b4, eq211_e2645_d_b5, eq211_e2645_d_b6, eq211_e2645_d_b7, eq211_e2645_d_b8, eq211_e2645_d_b9, eq211_e2645_d_b10, eq211_e2645_d_b11, eq211_e2645_d_b12, eq211_e2645_d_b13, eq211_e2645_d_b14, eq211_e2645_d_b15, eq211_e2645_d_b16, eq211_e2645_d_b17, eq211_e2645_d_b18, eq211_e2645_d_b19, eq211_e2645_d_b20, eq211_e2645_d_b21, eq211_e2645_d_b22, eq211_e2645_d_b23, eq211_e2645_d_b24, eq211_e2645_d_b25, eq211_e2645_d_b26, eq211_e2645_d_b27, eq211_e2645_d_b28, eq211_e2645_d_b29, eq211_e2645_d_b30, eq211_e2645_d_b31, eq211_e2645_d_b32, eq211_e2645_d_b33, eq211_e2645_d_b34, eq211_e2645_d_b35, eq211_e2645_d_b36, eq211_e2645_d_b37, eq211_e2645_d_b38, eq211_e2645_d_b39, eq211_e2645_d_b40, eq211_e2645_d_b41, eq211_e2645_d_b42, eq211_e2645_d_b43, eq211_e2645_d_b44, eq211_e2645_d_b45, eq211_e2645_d_b46, eq211_e2645_d_b47, eq211_e2645_d_b48, eq211_e2645_d_b49, eq211_e2645_d_b50, eq211_e2645_d_b51, eq211_e2645_d_b52, eq211_e2645_d_b53, eq211_e2645_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq211_value),
            &eq211_node_derivatives,
            &eq211_branch_derivatives,
            multiplicity,
        );
        let (eq212_e2659, eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, eq212_e2659_d_n10, eq212_e2659_d_n11, eq212_e2659_d_n12, eq212_e2659_d_n13, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22, eq212_e2659_d_b0, eq212_e2659_d_b1, eq212_e2659_d_b2, eq212_e2659_d_b3, eq212_e2659_d_b4, eq212_e2659_d_b5, eq212_e2659_d_b6, eq212_e2659_d_b7, eq212_e2659_d_b8, eq212_e2659_d_b9, eq212_e2659_d_b10, eq212_e2659_d_b11, eq212_e2659_d_b12, eq212_e2659_d_b13, eq212_e2659_d_b14, eq212_e2659_d_b15, eq212_e2659_d_b16, eq212_e2659_d_b17, eq212_e2659_d_b18, eq212_e2659_d_b19, eq212_e2659_d_b20, eq212_e2659_d_b21, eq212_e2659_d_b22, eq212_e2659_d_b23, eq212_e2659_d_b24, eq212_e2659_d_b25, eq212_e2659_d_b26, eq212_e2659_d_b27, eq212_e2659_d_b28, eq212_e2659_d_b29, eq212_e2659_d_b30, eq212_e2659_d_b31, eq212_e2659_d_b32, eq212_e2659_d_b33, eq212_e2659_d_b34, eq212_e2659_d_b35, eq212_e2659_d_b36, eq212_e2659_d_b37, eq212_e2659_d_b38, eq212_e2659_d_b39, eq212_e2659_d_b40, eq212_e2659_d_b41, eq212_e2659_d_b42, eq212_e2659_d_b43, eq212_e2659_d_b44, eq212_e2659_d_b45, eq212_e2659_d_b46, eq212_e2659_d_b47, eq212_e2659_d_b48, eq212_e2659_d_b49, eq212_e2659_d_b50, eq212_e2659_d_b51, eq212_e2659_d_b52, eq212_e2659_d_b53, eq212_e2659_d_b54,) = {
    if (((!s.b[605]) && s.b[608]) && s.b[609]) {
        let eq212_e2654: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 111, s.v[312]);
        let eq212_e2655: f64 = (p.p7 * eq212_e2654);
        let eq212_e2657: f64 = (eq212_e2655 * p.p249);
        let eq212_e2657_d_n0: f64 = (__rspice_deriv_cse_0 * p.p249);
        let eq212_e2657_d_n1: f64 = (__rspice_deriv_cse_1 * p.p249);
        let eq212_e2657_d_n2: f64 = (__rspice_deriv_cse_2 * p.p249);
        let eq212_e2657_d_n3: f64 = (__rspice_deriv_cse_3 * p.p249);
        let eq212_e2657_d_n4: f64 = (__rspice_deriv_cse_4 * p.p249);
        let eq212_e2657_d_n5: f64 = (__rspice_deriv_cse_5 * p.p249);
        let eq212_e2657_d_n6: f64 = (__rspice_deriv_cse_6 * p.p249);
        let eq212_e2657_d_n7: f64 = (__rspice_deriv_cse_7 * p.p249);
        let eq212_e2657_d_n8: f64 = (__rspice_deriv_cse_8 * p.p249);
        let eq212_e2657_d_n9: f64 = (__rspice_deriv_cse_9 * p.p249);
        let eq212_e2657_d_n10: f64 = (__rspice_deriv_cse_10 * p.p249);
        let eq212_e2657_d_n11: f64 = (__rspice_deriv_cse_11 * p.p249);
        let eq212_e2657_d_n12: f64 = (__rspice_deriv_cse_12 * p.p249);
        let eq212_e2657_d_n13: f64 = (__rspice_deriv_cse_13 * p.p249);
        let eq212_e2657_d_n14: f64 = (__rspice_deriv_cse_14 * p.p249);
        let eq212_e2657_d_n15: f64 = (__rspice_deriv_cse_15 * p.p249);
        let eq212_e2657_d_n16: f64 = (__rspice_deriv_cse_16 * p.p249);
        let eq212_e2657_d_n17: f64 = (__rspice_deriv_cse_17 * p.p249);
        let eq212_e2657_d_n18: f64 = (__rspice_deriv_cse_18 * p.p249);
        let eq212_e2657_d_n19: f64 = (__rspice_deriv_cse_19 * p.p249);
        let eq212_e2657_d_n20: f64 = (__rspice_deriv_cse_20 * p.p249);
        let eq212_e2657_d_n21: f64 = (__rspice_deriv_cse_21 * p.p249);
        let eq212_e2657_d_n22: f64 = (__rspice_deriv_cse_22 * p.p249);
        let eq212_e2657_d_b0: f64 = (__rspice_deriv_cse_23 * p.p249);
        let eq212_e2657_d_b1: f64 = (__rspice_deriv_cse_24 * p.p249);
        let eq212_e2657_d_b2: f64 = (__rspice_deriv_cse_25 * p.p249);
        let eq212_e2657_d_b3: f64 = (__rspice_deriv_cse_26 * p.p249);
        let eq212_e2657_d_b4: f64 = (__rspice_deriv_cse_27 * p.p249);
        let eq212_e2657_d_b5: f64 = (__rspice_deriv_cse_28 * p.p249);
        let eq212_e2657_d_b6: f64 = (__rspice_deriv_cse_29 * p.p249);
        let eq212_e2657_d_b7: f64 = (__rspice_deriv_cse_30 * p.p249);
        let eq212_e2657_d_b8: f64 = (__rspice_deriv_cse_31 * p.p249);
        let eq212_e2657_d_b9: f64 = (__rspice_deriv_cse_32 * p.p249);
        let eq212_e2657_d_b10: f64 = (__rspice_deriv_cse_33 * p.p249);
        let eq212_e2657_d_b11: f64 = (__rspice_deriv_cse_34 * p.p249);
        let eq212_e2657_d_b12: f64 = (__rspice_deriv_cse_35 * p.p249);
        let eq212_e2657_d_b13: f64 = (__rspice_deriv_cse_36 * p.p249);
        let eq212_e2657_d_b14: f64 = (__rspice_deriv_cse_37 * p.p249);
        let eq212_e2657_d_b15: f64 = (__rspice_deriv_cse_38 * p.p249);
        let eq212_e2657_d_b16: f64 = (__rspice_deriv_cse_39 * p.p249);
        let eq212_e2657_d_b17: f64 = (__rspice_deriv_cse_40 * p.p249);
        let eq212_e2657_d_b18: f64 = (__rspice_deriv_cse_41 * p.p249);
        let eq212_e2657_d_b19: f64 = (__rspice_deriv_cse_42 * p.p249);
        let eq212_e2657_d_b20: f64 = (__rspice_deriv_cse_43 * p.p249);
        let eq212_e2657_d_b21: f64 = (__rspice_deriv_cse_44 * p.p249);
        let eq212_e2657_d_b22: f64 = (__rspice_deriv_cse_45 * p.p249);
        let eq212_e2657_d_b23: f64 = (__rspice_deriv_cse_46 * p.p249);
        let eq212_e2657_d_b24: f64 = (__rspice_deriv_cse_47 * p.p249);
        let eq212_e2657_d_b25: f64 = (__rspice_deriv_cse_48 * p.p249);
        let eq212_e2657_d_b26: f64 = (__rspice_deriv_cse_49 * p.p249);
        let eq212_e2657_d_b27: f64 = (__rspice_deriv_cse_50 * p.p249);
        let eq212_e2657_d_b28: f64 = (__rspice_deriv_cse_51 * p.p249);
        let eq212_e2657_d_b29: f64 = (__rspice_deriv_cse_52 * p.p249);
        let eq212_e2657_d_b30: f64 = (__rspice_deriv_cse_53 * p.p249);
        let eq212_e2657_d_b31: f64 = (__rspice_deriv_cse_54 * p.p249);
        let eq212_e2657_d_b32: f64 = (__rspice_deriv_cse_55 * p.p249);
        let eq212_e2657_d_b33: f64 = (__rspice_deriv_cse_56 * p.p249);
        let eq212_e2657_d_b34: f64 = (__rspice_deriv_cse_57 * p.p249);
        let eq212_e2657_d_b35: f64 = (__rspice_deriv_cse_58 * p.p249);
        let eq212_e2657_d_b36: f64 = (__rspice_deriv_cse_59 * p.p249);
        let eq212_e2657_d_b37: f64 = (__rspice_deriv_cse_60 * p.p249);
        let eq212_e2657_d_b38: f64 = (__rspice_deriv_cse_61 * p.p249);
        let eq212_e2657_d_b39: f64 = (__rspice_deriv_cse_62 * p.p249);
        let eq212_e2657_d_b40: f64 = (__rspice_deriv_cse_63 * p.p249);
        let eq212_e2657_d_b41: f64 = (__rspice_deriv_cse_64 * p.p249);
        let eq212_e2657_d_b42: f64 = (__rspice_deriv_cse_65 * p.p249);
        let eq212_e2657_d_b43: f64 = (__rspice_deriv_cse_66 * p.p249);
        let eq212_e2657_d_b44: f64 = (__rspice_deriv_cse_67 * p.p249);
        let eq212_e2657_d_b45: f64 = (__rspice_deriv_cse_68 * p.p249);
        let eq212_e2657_d_b46: f64 = (__rspice_deriv_cse_69 * p.p249);
        let eq212_e2657_d_b47: f64 = (__rspice_deriv_cse_70 * p.p249);
        let eq212_e2657_d_b48: f64 = (__rspice_deriv_cse_71 * p.p249);
        let eq212_e2657_d_b49: f64 = (__rspice_deriv_cse_72 * p.p249);
        let eq212_e2657_d_b50: f64 = (__rspice_deriv_cse_73 * p.p249);
        let eq212_e2657_d_b51: f64 = (__rspice_deriv_cse_74 * p.p249);
        let eq212_e2657_d_b52: f64 = (__rspice_deriv_cse_75 * p.p249);
        let eq212_e2657_d_b53: f64 = (__rspice_deriv_cse_76 * p.p249);
        let eq212_e2657_d_b54: f64 = (__rspice_deriv_cse_77 * p.p249);
        (eq212_e2657, eq212_e2657_d_n0, eq212_e2657_d_n1, eq212_e2657_d_n2, eq212_e2657_d_n3, eq212_e2657_d_n4, eq212_e2657_d_n5, eq212_e2657_d_n6, eq212_e2657_d_n7, eq212_e2657_d_n8, eq212_e2657_d_n9, eq212_e2657_d_n10, eq212_e2657_d_n11, eq212_e2657_d_n12, eq212_e2657_d_n13, eq212_e2657_d_n14, eq212_e2657_d_n15, eq212_e2657_d_n16, eq212_e2657_d_n17, eq212_e2657_d_n18, eq212_e2657_d_n19, eq212_e2657_d_n20, eq212_e2657_d_n21, eq212_e2657_d_n22, eq212_e2657_d_b0, eq212_e2657_d_b1, eq212_e2657_d_b2, eq212_e2657_d_b3, eq212_e2657_d_b4, eq212_e2657_d_b5, eq212_e2657_d_b6, eq212_e2657_d_b7, eq212_e2657_d_b8, eq212_e2657_d_b9, eq212_e2657_d_b10, eq212_e2657_d_b11, eq212_e2657_d_b12, eq212_e2657_d_b13, eq212_e2657_d_b14, eq212_e2657_d_b15, eq212_e2657_d_b16, eq212_e2657_d_b17, eq212_e2657_d_b18, eq212_e2657_d_b19, eq212_e2657_d_b20, eq212_e2657_d_b21, eq212_e2657_d_b22, eq212_e2657_d_b23, eq212_e2657_d_b24, eq212_e2657_d_b25, eq212_e2657_d_b26, eq212_e2657_d_b27, eq212_e2657_d_b28, eq212_e2657_d_b29, eq212_e2657_d_b30, eq212_e2657_d_b31, eq212_e2657_d_b32, eq212_e2657_d_b33, eq212_e2657_d_b34, eq212_e2657_d_b35, eq212_e2657_d_b36, eq212_e2657_d_b37, eq212_e2657_d_b38, eq212_e2657_d_b39, eq212_e2657_d_b40, eq212_e2657_d_b41, eq212_e2657_d_b42, eq212_e2657_d_b43, eq212_e2657_d_b44, eq212_e2657_d_b45, eq212_e2657_d_b46, eq212_e2657_d_b47, eq212_e2657_d_b48, eq212_e2657_d_b49, eq212_e2657_d_b50, eq212_e2657_d_b51, eq212_e2657_d_b52, eq212_e2657_d_b53, eq212_e2657_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq212_value: f64 = eq212_e2659;
        let eq212_node_derivatives: [f64; 23] = [eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, eq212_e2659_d_n10, eq212_e2659_d_n11, eq212_e2659_d_n12, eq212_e2659_d_n13, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22];
        let eq212_branch_derivatives: [f64; 55] = [eq212_e2659_d_b0, eq212_e2659_d_b1, eq212_e2659_d_b2, eq212_e2659_d_b3, eq212_e2659_d_b4, eq212_e2659_d_b5, eq212_e2659_d_b6, eq212_e2659_d_b7, eq212_e2659_d_b8, eq212_e2659_d_b9, eq212_e2659_d_b10, eq212_e2659_d_b11, eq212_e2659_d_b12, eq212_e2659_d_b13, eq212_e2659_d_b14, eq212_e2659_d_b15, eq212_e2659_d_b16, eq212_e2659_d_b17, eq212_e2659_d_b18, eq212_e2659_d_b19, eq212_e2659_d_b20, eq212_e2659_d_b21, eq212_e2659_d_b22, eq212_e2659_d_b23, eq212_e2659_d_b24, eq212_e2659_d_b25, eq212_e2659_d_b26, eq212_e2659_d_b27, eq212_e2659_d_b28, eq212_e2659_d_b29, eq212_e2659_d_b30, eq212_e2659_d_b31, eq212_e2659_d_b32, eq212_e2659_d_b33, eq212_e2659_d_b34, eq212_e2659_d_b35, eq212_e2659_d_b36, eq212_e2659_d_b37, eq212_e2659_d_b38, eq212_e2659_d_b39, eq212_e2659_d_b40, eq212_e2659_d_b41, eq212_e2659_d_b42, eq212_e2659_d_b43, eq212_e2659_d_b44, eq212_e2659_d_b45, eq212_e2659_d_b46, eq212_e2659_d_b47, eq212_e2659_d_b48, eq212_e2659_d_b49, eq212_e2659_d_b50, eq212_e2659_d_b51, eq212_e2659_d_b52, eq212_e2659_d_b53, eq212_e2659_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq212_value),
            &eq212_node_derivatives,
            &eq212_branch_derivatives,
            multiplicity,
        );
        let (eq213_e2672, eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, eq213_e2672_d_n10, eq213_e2672_d_n11, eq213_e2672_d_n12, eq213_e2672_d_n13, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22, eq213_e2672_d_b0, eq213_e2672_d_b1, eq213_e2672_d_b2, eq213_e2672_d_b3, eq213_e2672_d_b4, eq213_e2672_d_b5, eq213_e2672_d_b6, eq213_e2672_d_b7, eq213_e2672_d_b8, eq213_e2672_d_b9, eq213_e2672_d_b10, eq213_e2672_d_b11, eq213_e2672_d_b12, eq213_e2672_d_b13, eq213_e2672_d_b14, eq213_e2672_d_b15, eq213_e2672_d_b16, eq213_e2672_d_b17, eq213_e2672_d_b18, eq213_e2672_d_b19, eq213_e2672_d_b20, eq213_e2672_d_b21, eq213_e2672_d_b22, eq213_e2672_d_b23, eq213_e2672_d_b24, eq213_e2672_d_b25, eq213_e2672_d_b26, eq213_e2672_d_b27, eq213_e2672_d_b28, eq213_e2672_d_b29, eq213_e2672_d_b30, eq213_e2672_d_b31, eq213_e2672_d_b32, eq213_e2672_d_b33, eq213_e2672_d_b34, eq213_e2672_d_b35, eq213_e2672_d_b36, eq213_e2672_d_b37, eq213_e2672_d_b38, eq213_e2672_d_b39, eq213_e2672_d_b40, eq213_e2672_d_b41, eq213_e2672_d_b42, eq213_e2672_d_b43, eq213_e2672_d_b44, eq213_e2672_d_b45, eq213_e2672_d_b46, eq213_e2672_d_b47, eq213_e2672_d_b48, eq213_e2672_d_b49, eq213_e2672_d_b50, eq213_e2672_d_b51, eq213_e2672_d_b52, eq213_e2672_d_b53, eq213_e2672_d_b54,) = {
    if (((!s.b[605]) && s.b[608]) && (!s.b[609])) {
        let eq213_e2669: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 112, s.v[312]);
        let eq213_e2670: f64 = (p.p7 * eq213_e2669);
        (eq213_e2670, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq213_value: f64 = eq213_e2672;
        let eq213_node_derivatives: [f64; 23] = [eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, eq213_e2672_d_n10, eq213_e2672_d_n11, eq213_e2672_d_n12, eq213_e2672_d_n13, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22];
        let eq213_branch_derivatives: [f64; 55] = [eq213_e2672_d_b0, eq213_e2672_d_b1, eq213_e2672_d_b2, eq213_e2672_d_b3, eq213_e2672_d_b4, eq213_e2672_d_b5, eq213_e2672_d_b6, eq213_e2672_d_b7, eq213_e2672_d_b8, eq213_e2672_d_b9, eq213_e2672_d_b10, eq213_e2672_d_b11, eq213_e2672_d_b12, eq213_e2672_d_b13, eq213_e2672_d_b14, eq213_e2672_d_b15, eq213_e2672_d_b16, eq213_e2672_d_b17, eq213_e2672_d_b18, eq213_e2672_d_b19, eq213_e2672_d_b20, eq213_e2672_d_b21, eq213_e2672_d_b22, eq213_e2672_d_b23, eq213_e2672_d_b24, eq213_e2672_d_b25, eq213_e2672_d_b26, eq213_e2672_d_b27, eq213_e2672_d_b28, eq213_e2672_d_b29, eq213_e2672_d_b30, eq213_e2672_d_b31, eq213_e2672_d_b32, eq213_e2672_d_b33, eq213_e2672_d_b34, eq213_e2672_d_b35, eq213_e2672_d_b36, eq213_e2672_d_b37, eq213_e2672_d_b38, eq213_e2672_d_b39, eq213_e2672_d_b40, eq213_e2672_d_b41, eq213_e2672_d_b42, eq213_e2672_d_b43, eq213_e2672_d_b44, eq213_e2672_d_b45, eq213_e2672_d_b46, eq213_e2672_d_b47, eq213_e2672_d_b48, eq213_e2672_d_b49, eq213_e2672_d_b50, eq213_e2672_d_b51, eq213_e2672_d_b52, eq213_e2672_d_b53, eq213_e2672_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq213_value),
            &eq213_node_derivatives,
            &eq213_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_46(
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
        var_qfr: f64,
        var_qfr_db0: f64,
        var_qfr_db1: f64,
        var_qfr_db10: f64,
        var_qfr_db11: f64,
        var_qfr_db12: f64,
        var_qfr_db13: f64,
        var_qfr_db14: f64,
        var_qfr_db15: f64,
        var_qfr_db16: f64,
        var_qfr_db17: f64,
        var_qfr_db18: f64,
        var_qfr_db19: f64,
        var_qfr_db2: f64,
        var_qfr_db20: f64,
        var_qfr_db21: f64,
        var_qfr_db22: f64,
        var_qfr_db23: f64,
        var_qfr_db24: f64,
        var_qfr_db25: f64,
        var_qfr_db26: f64,
        var_qfr_db27: f64,
        var_qfr_db28: f64,
        var_qfr_db29: f64,
        var_qfr_db3: f64,
        var_qfr_db30: f64,
        var_qfr_db31: f64,
        var_qfr_db32: f64,
        var_qfr_db33: f64,
        var_qfr_db34: f64,
        var_qfr_db35: f64,
        var_qfr_db36: f64,
        var_qfr_db37: f64,
        var_qfr_db38: f64,
        var_qfr_db39: f64,
        var_qfr_db4: f64,
        var_qfr_db40: f64,
        var_qfr_db41: f64,
        var_qfr_db42: f64,
        var_qfr_db43: f64,
        var_qfr_db44: f64,
        var_qfr_db45: f64,
        var_qfr_db46: f64,
        var_qfr_db47: f64,
        var_qfr_db48: f64,
        var_qfr_db49: f64,
        var_qfr_db5: f64,
        var_qfr_db50: f64,
        var_qfr_db51: f64,
        var_qfr_db52: f64,
        var_qfr_db53: f64,
        var_qfr_db54: f64,
        var_qfr_db6: f64,
        var_qfr_db7: f64,
        var_qfr_db8: f64,
        var_qfr_db9: f64,
        var_qfr_dn0: f64,
        var_qfr_dn1: f64,
        var_qfr_dn10: f64,
        var_qfr_dn11: f64,
        var_qfr_dn12: f64,
        var_qfr_dn13: f64,
        var_qfr_dn14: f64,
        var_qfr_dn15: f64,
        var_qfr_dn16: f64,
        var_qfr_dn17: f64,
        var_qfr_dn18: f64,
        var_qfr_dn19: f64,
        var_qfr_dn2: f64,
        var_qfr_dn20: f64,
        var_qfr_dn21: f64,
        var_qfr_dn22: f64,
        var_qfr_dn3: f64,
        var_qfr_dn4: f64,
        var_qfr_dn5: f64,
        var_qfr_dn6: f64,
        var_qfr_dn7: f64,
        var_qfr_dn8: f64,
        var_qfr_dn9: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq214_e2687, eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n10, eq214_e2687_d_n11, eq214_e2687_d_n12, eq214_e2687_d_n13, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22, eq214_e2687_d_b0, eq214_e2687_d_b1, eq214_e2687_d_b2, eq214_e2687_d_b3, eq214_e2687_d_b4, eq214_e2687_d_b5, eq214_e2687_d_b6, eq214_e2687_d_b7, eq214_e2687_d_b8, eq214_e2687_d_b9, eq214_e2687_d_b10, eq214_e2687_d_b11, eq214_e2687_d_b12, eq214_e2687_d_b13, eq214_e2687_d_b14, eq214_e2687_d_b15, eq214_e2687_d_b16, eq214_e2687_d_b17, eq214_e2687_d_b18, eq214_e2687_d_b19, eq214_e2687_d_b20, eq214_e2687_d_b21, eq214_e2687_d_b22, eq214_e2687_d_b23, eq214_e2687_d_b24, eq214_e2687_d_b25, eq214_e2687_d_b26, eq214_e2687_d_b27, eq214_e2687_d_b28, eq214_e2687_d_b29, eq214_e2687_d_b30, eq214_e2687_d_b31, eq214_e2687_d_b32, eq214_e2687_d_b33, eq214_e2687_d_b34, eq214_e2687_d_b35, eq214_e2687_d_b36, eq214_e2687_d_b37, eq214_e2687_d_b38, eq214_e2687_d_b39, eq214_e2687_d_b40, eq214_e2687_d_b41, eq214_e2687_d_b42, eq214_e2687_d_b43, eq214_e2687_d_b44, eq214_e2687_d_b45, eq214_e2687_d_b46, eq214_e2687_d_b47, eq214_e2687_d_b48, eq214_e2687_d_b49, eq214_e2687_d_b50, eq214_e2687_d_b51, eq214_e2687_d_b52, eq214_e2687_d_b53, eq214_e2687_d_b54,) = {
    if (((!s.b[605]) && s.b[608]) && (!s.b[609])) {
        let eq214_e2682: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 113, s.v[312]);
        let eq214_e2683: f64 = (p.p7 * eq214_e2682);
        let eq214_e2683_d_n0: f64 = (p.p7 * (s.dn[312][0] * ddt_scale));
        let eq214_e2683_d_n1: f64 = (p.p7 * (s.dn[312][1] * ddt_scale));
        let eq214_e2683_d_n2: f64 = (p.p7 * (s.dn[312][2] * ddt_scale));
        let eq214_e2683_d_n3: f64 = (p.p7 * (s.dn[312][3] * ddt_scale));
        let eq214_e2683_d_n4: f64 = (p.p7 * (s.dn[312][4] * ddt_scale));
        let eq214_e2683_d_n5: f64 = (p.p7 * (s.dn[312][5] * ddt_scale));
        let eq214_e2683_d_n6: f64 = (p.p7 * (s.dn[312][6] * ddt_scale));
        let eq214_e2683_d_n7: f64 = (p.p7 * (s.dn[312][7] * ddt_scale));
        let eq214_e2683_d_n8: f64 = (p.p7 * (s.dn[312][8] * ddt_scale));
        let eq214_e2683_d_n9: f64 = (p.p7 * (s.dn[312][9] * ddt_scale));
        let eq214_e2683_d_n10: f64 = (p.p7 * (s.dn[312][10] * ddt_scale));
        let eq214_e2683_d_n11: f64 = (p.p7 * (s.dn[312][11] * ddt_scale));
        let eq214_e2683_d_n12: f64 = (p.p7 * (s.dn[312][12] * ddt_scale));
        let eq214_e2683_d_n13: f64 = (p.p7 * (s.dn[312][13] * ddt_scale));
        let eq214_e2683_d_n14: f64 = (p.p7 * (s.dn[312][14] * ddt_scale));
        let eq214_e2683_d_n15: f64 = (p.p7 * (s.dn[312][15] * ddt_scale));
        let eq214_e2683_d_n16: f64 = (p.p7 * (s.dn[312][16] * ddt_scale));
        let eq214_e2683_d_n17: f64 = (p.p7 * (s.dn[312][17] * ddt_scale));
        let eq214_e2683_d_n18: f64 = (p.p7 * (s.dn[312][18] * ddt_scale));
        let eq214_e2683_d_n19: f64 = (p.p7 * (s.dn[312][19] * ddt_scale));
        let eq214_e2683_d_n20: f64 = (p.p7 * (s.dn[312][20] * ddt_scale));
        let eq214_e2683_d_n21: f64 = (p.p7 * (s.dn[312][21] * ddt_scale));
        let eq214_e2683_d_n22: f64 = (p.p7 * (s.dn[312][22] * ddt_scale));
        let eq214_e2683_d_b0: f64 = (p.p7 * (s.db[312][0] * ddt_scale));
        let eq214_e2683_d_b1: f64 = (p.p7 * (s.db[312][1] * ddt_scale));
        let eq214_e2683_d_b2: f64 = (p.p7 * (s.db[312][2] * ddt_scale));
        let eq214_e2683_d_b3: f64 = (p.p7 * (s.db[312][3] * ddt_scale));
        let eq214_e2683_d_b4: f64 = (p.p7 * (s.db[312][4] * ddt_scale));
        let eq214_e2683_d_b5: f64 = (p.p7 * (s.db[312][5] * ddt_scale));
        let eq214_e2683_d_b6: f64 = (p.p7 * (s.db[312][6] * ddt_scale));
        let eq214_e2683_d_b7: f64 = (p.p7 * (s.db[312][7] * ddt_scale));
        let eq214_e2683_d_b8: f64 = (p.p7 * (s.db[312][8] * ddt_scale));
        let eq214_e2683_d_b9: f64 = (p.p7 * (s.db[312][9] * ddt_scale));
        let eq214_e2683_d_b10: f64 = (p.p7 * (s.db[312][10] * ddt_scale));
        let eq214_e2683_d_b11: f64 = (p.p7 * (s.db[312][11] * ddt_scale));
        let eq214_e2683_d_b12: f64 = (p.p7 * (s.db[312][12] * ddt_scale));
        let eq214_e2683_d_b13: f64 = (p.p7 * (s.db[312][13] * ddt_scale));
        let eq214_e2683_d_b14: f64 = (p.p7 * (s.db[312][14] * ddt_scale));
        let eq214_e2683_d_b15: f64 = (p.p7 * (s.db[312][15] * ddt_scale));
        let eq214_e2683_d_b16: f64 = (p.p7 * (s.db[312][16] * ddt_scale));
        let eq214_e2683_d_b17: f64 = (p.p7 * (s.db[312][17] * ddt_scale));
        let eq214_e2683_d_b18: f64 = (p.p7 * (s.db[312][18] * ddt_scale));
        let eq214_e2683_d_b19: f64 = (p.p7 * (s.db[312][19] * ddt_scale));
        let eq214_e2683_d_b20: f64 = (p.p7 * (s.db[312][20] * ddt_scale));
        let eq214_e2683_d_b21: f64 = (p.p7 * (s.db[312][21] * ddt_scale));
        let eq214_e2683_d_b22: f64 = (p.p7 * (s.db[312][22] * ddt_scale));
        let eq214_e2683_d_b23: f64 = (p.p7 * (s.db[312][23] * ddt_scale));
        let eq214_e2683_d_b24: f64 = (p.p7 * (s.db[312][24] * ddt_scale));
        let eq214_e2683_d_b25: f64 = (p.p7 * (s.db[312][25] * ddt_scale));
        let eq214_e2683_d_b26: f64 = (p.p7 * (s.db[312][26] * ddt_scale));
        let eq214_e2683_d_b27: f64 = (p.p7 * (s.db[312][27] * ddt_scale));
        let eq214_e2683_d_b28: f64 = (p.p7 * (s.db[312][28] * ddt_scale));
        let eq214_e2683_d_b29: f64 = (p.p7 * (s.db[312][29] * ddt_scale));
        let eq214_e2683_d_b30: f64 = (p.p7 * (s.db[312][30] * ddt_scale));
        let eq214_e2683_d_b31: f64 = (p.p7 * (s.db[312][31] * ddt_scale));
        let eq214_e2683_d_b32: f64 = (p.p7 * (s.db[312][32] * ddt_scale));
        let eq214_e2683_d_b33: f64 = (p.p7 * (s.db[312][33] * ddt_scale));
        let eq214_e2683_d_b34: f64 = (p.p7 * (s.db[312][34] * ddt_scale));
        let eq214_e2683_d_b35: f64 = (p.p7 * (s.db[312][35] * ddt_scale));
        let eq214_e2683_d_b36: f64 = (p.p7 * (s.db[312][36] * ddt_scale));
        let eq214_e2683_d_b37: f64 = (p.p7 * (s.db[312][37] * ddt_scale));
        let eq214_e2683_d_b38: f64 = (p.p7 * (s.db[312][38] * ddt_scale));
        let eq214_e2683_d_b39: f64 = (p.p7 * (s.db[312][39] * ddt_scale));
        let eq214_e2683_d_b40: f64 = (p.p7 * (s.db[312][40] * ddt_scale));
        let eq214_e2683_d_b41: f64 = (p.p7 * (s.db[312][41] * ddt_scale));
        let eq214_e2683_d_b42: f64 = (p.p7 * (s.db[312][42] * ddt_scale));
        let eq214_e2683_d_b43: f64 = (p.p7 * (s.db[312][43] * ddt_scale));
        let eq214_e2683_d_b44: f64 = (p.p7 * (s.db[312][44] * ddt_scale));
        let eq214_e2683_d_b45: f64 = (p.p7 * (s.db[312][45] * ddt_scale));
        let eq214_e2683_d_b46: f64 = (p.p7 * (s.db[312][46] * ddt_scale));
        let eq214_e2683_d_b47: f64 = (p.p7 * (s.db[312][47] * ddt_scale));
        let eq214_e2683_d_b48: f64 = (p.p7 * (s.db[312][48] * ddt_scale));
        let eq214_e2683_d_b49: f64 = (p.p7 * (s.db[312][49] * ddt_scale));
        let eq214_e2683_d_b50: f64 = (p.p7 * (s.db[312][50] * ddt_scale));
        let eq214_e2683_d_b51: f64 = (p.p7 * (s.db[312][51] * ddt_scale));
        let eq214_e2683_d_b52: f64 = (p.p7 * (s.db[312][52] * ddt_scale));
        let eq214_e2683_d_b53: f64 = (p.p7 * (s.db[312][53] * ddt_scale));
        let eq214_e2683_d_b54: f64 = (p.p7 * (s.db[312][54] * ddt_scale));
        let eq214_e2685: f64 = (eq214_e2683 * p.p249);
        let eq214_e2685_d_n0: f64 = (eq214_e2683_d_n0 * p.p249);
        let eq214_e2685_d_n1: f64 = (eq214_e2683_d_n1 * p.p249);
        let eq214_e2685_d_n2: f64 = (eq214_e2683_d_n2 * p.p249);
        let eq214_e2685_d_n3: f64 = (eq214_e2683_d_n3 * p.p249);
        let eq214_e2685_d_n4: f64 = (eq214_e2683_d_n4 * p.p249);
        let eq214_e2685_d_n5: f64 = (eq214_e2683_d_n5 * p.p249);
        let eq214_e2685_d_n6: f64 = (eq214_e2683_d_n6 * p.p249);
        let eq214_e2685_d_n7: f64 = (eq214_e2683_d_n7 * p.p249);
        let eq214_e2685_d_n8: f64 = (eq214_e2683_d_n8 * p.p249);
        let eq214_e2685_d_n9: f64 = (eq214_e2683_d_n9 * p.p249);
        let eq214_e2685_d_n10: f64 = (eq214_e2683_d_n10 * p.p249);
        let eq214_e2685_d_n11: f64 = (eq214_e2683_d_n11 * p.p249);
        let eq214_e2685_d_n12: f64 = (eq214_e2683_d_n12 * p.p249);
        let eq214_e2685_d_n13: f64 = (eq214_e2683_d_n13 * p.p249);
        let eq214_e2685_d_n14: f64 = (eq214_e2683_d_n14 * p.p249);
        let eq214_e2685_d_n15: f64 = (eq214_e2683_d_n15 * p.p249);
        let eq214_e2685_d_n16: f64 = (eq214_e2683_d_n16 * p.p249);
        let eq214_e2685_d_n17: f64 = (eq214_e2683_d_n17 * p.p249);
        let eq214_e2685_d_n18: f64 = (eq214_e2683_d_n18 * p.p249);
        let eq214_e2685_d_n19: f64 = (eq214_e2683_d_n19 * p.p249);
        let eq214_e2685_d_n20: f64 = (eq214_e2683_d_n20 * p.p249);
        let eq214_e2685_d_n21: f64 = (eq214_e2683_d_n21 * p.p249);
        let eq214_e2685_d_n22: f64 = (eq214_e2683_d_n22 * p.p249);
        let eq214_e2685_d_b0: f64 = (eq214_e2683_d_b0 * p.p249);
        let eq214_e2685_d_b1: f64 = (eq214_e2683_d_b1 * p.p249);
        let eq214_e2685_d_b2: f64 = (eq214_e2683_d_b2 * p.p249);
        let eq214_e2685_d_b3: f64 = (eq214_e2683_d_b3 * p.p249);
        let eq214_e2685_d_b4: f64 = (eq214_e2683_d_b4 * p.p249);
        let eq214_e2685_d_b5: f64 = (eq214_e2683_d_b5 * p.p249);
        let eq214_e2685_d_b6: f64 = (eq214_e2683_d_b6 * p.p249);
        let eq214_e2685_d_b7: f64 = (eq214_e2683_d_b7 * p.p249);
        let eq214_e2685_d_b8: f64 = (eq214_e2683_d_b8 * p.p249);
        let eq214_e2685_d_b9: f64 = (eq214_e2683_d_b9 * p.p249);
        let eq214_e2685_d_b10: f64 = (eq214_e2683_d_b10 * p.p249);
        let eq214_e2685_d_b11: f64 = (eq214_e2683_d_b11 * p.p249);
        let eq214_e2685_d_b12: f64 = (eq214_e2683_d_b12 * p.p249);
        let eq214_e2685_d_b13: f64 = (eq214_e2683_d_b13 * p.p249);
        let eq214_e2685_d_b14: f64 = (eq214_e2683_d_b14 * p.p249);
        let eq214_e2685_d_b15: f64 = (eq214_e2683_d_b15 * p.p249);
        let eq214_e2685_d_b16: f64 = (eq214_e2683_d_b16 * p.p249);
        let eq214_e2685_d_b17: f64 = (eq214_e2683_d_b17 * p.p249);
        let eq214_e2685_d_b18: f64 = (eq214_e2683_d_b18 * p.p249);
        let eq214_e2685_d_b19: f64 = (eq214_e2683_d_b19 * p.p249);
        let eq214_e2685_d_b20: f64 = (eq214_e2683_d_b20 * p.p249);
        let eq214_e2685_d_b21: f64 = (eq214_e2683_d_b21 * p.p249);
        let eq214_e2685_d_b22: f64 = (eq214_e2683_d_b22 * p.p249);
        let eq214_e2685_d_b23: f64 = (eq214_e2683_d_b23 * p.p249);
        let eq214_e2685_d_b24: f64 = (eq214_e2683_d_b24 * p.p249);
        let eq214_e2685_d_b25: f64 = (eq214_e2683_d_b25 * p.p249);
        let eq214_e2685_d_b26: f64 = (eq214_e2683_d_b26 * p.p249);
        let eq214_e2685_d_b27: f64 = (eq214_e2683_d_b27 * p.p249);
        let eq214_e2685_d_b28: f64 = (eq214_e2683_d_b28 * p.p249);
        let eq214_e2685_d_b29: f64 = (eq214_e2683_d_b29 * p.p249);
        let eq214_e2685_d_b30: f64 = (eq214_e2683_d_b30 * p.p249);
        let eq214_e2685_d_b31: f64 = (eq214_e2683_d_b31 * p.p249);
        let eq214_e2685_d_b32: f64 = (eq214_e2683_d_b32 * p.p249);
        let eq214_e2685_d_b33: f64 = (eq214_e2683_d_b33 * p.p249);
        let eq214_e2685_d_b34: f64 = (eq214_e2683_d_b34 * p.p249);
        let eq214_e2685_d_b35: f64 = (eq214_e2683_d_b35 * p.p249);
        let eq214_e2685_d_b36: f64 = (eq214_e2683_d_b36 * p.p249);
        let eq214_e2685_d_b37: f64 = (eq214_e2683_d_b37 * p.p249);
        let eq214_e2685_d_b38: f64 = (eq214_e2683_d_b38 * p.p249);
        let eq214_e2685_d_b39: f64 = (eq214_e2683_d_b39 * p.p249);
        let eq214_e2685_d_b40: f64 = (eq214_e2683_d_b40 * p.p249);
        let eq214_e2685_d_b41: f64 = (eq214_e2683_d_b41 * p.p249);
        let eq214_e2685_d_b42: f64 = (eq214_e2683_d_b42 * p.p249);
        let eq214_e2685_d_b43: f64 = (eq214_e2683_d_b43 * p.p249);
        let eq214_e2685_d_b44: f64 = (eq214_e2683_d_b44 * p.p249);
        let eq214_e2685_d_b45: f64 = (eq214_e2683_d_b45 * p.p249);
        let eq214_e2685_d_b46: f64 = (eq214_e2683_d_b46 * p.p249);
        let eq214_e2685_d_b47: f64 = (eq214_e2683_d_b47 * p.p249);
        let eq214_e2685_d_b48: f64 = (eq214_e2683_d_b48 * p.p249);
        let eq214_e2685_d_b49: f64 = (eq214_e2683_d_b49 * p.p249);
        let eq214_e2685_d_b50: f64 = (eq214_e2683_d_b50 * p.p249);
        let eq214_e2685_d_b51: f64 = (eq214_e2683_d_b51 * p.p249);
        let eq214_e2685_d_b52: f64 = (eq214_e2683_d_b52 * p.p249);
        let eq214_e2685_d_b53: f64 = (eq214_e2683_d_b53 * p.p249);
        let eq214_e2685_d_b54: f64 = (eq214_e2683_d_b54 * p.p249);
        (eq214_e2685, eq214_e2685_d_n0, eq214_e2685_d_n1, eq214_e2685_d_n2, eq214_e2685_d_n3, eq214_e2685_d_n4, eq214_e2685_d_n5, eq214_e2685_d_n6, eq214_e2685_d_n7, eq214_e2685_d_n8, eq214_e2685_d_n9, eq214_e2685_d_n10, eq214_e2685_d_n11, eq214_e2685_d_n12, eq214_e2685_d_n13, eq214_e2685_d_n14, eq214_e2685_d_n15, eq214_e2685_d_n16, eq214_e2685_d_n17, eq214_e2685_d_n18, eq214_e2685_d_n19, eq214_e2685_d_n20, eq214_e2685_d_n21, eq214_e2685_d_n22, eq214_e2685_d_b0, eq214_e2685_d_b1, eq214_e2685_d_b2, eq214_e2685_d_b3, eq214_e2685_d_b4, eq214_e2685_d_b5, eq214_e2685_d_b6, eq214_e2685_d_b7, eq214_e2685_d_b8, eq214_e2685_d_b9, eq214_e2685_d_b10, eq214_e2685_d_b11, eq214_e2685_d_b12, eq214_e2685_d_b13, eq214_e2685_d_b14, eq214_e2685_d_b15, eq214_e2685_d_b16, eq214_e2685_d_b17, eq214_e2685_d_b18, eq214_e2685_d_b19, eq214_e2685_d_b20, eq214_e2685_d_b21, eq214_e2685_d_b22, eq214_e2685_d_b23, eq214_e2685_d_b24, eq214_e2685_d_b25, eq214_e2685_d_b26, eq214_e2685_d_b27, eq214_e2685_d_b28, eq214_e2685_d_b29, eq214_e2685_d_b30, eq214_e2685_d_b31, eq214_e2685_d_b32, eq214_e2685_d_b33, eq214_e2685_d_b34, eq214_e2685_d_b35, eq214_e2685_d_b36, eq214_e2685_d_b37, eq214_e2685_d_b38, eq214_e2685_d_b39, eq214_e2685_d_b40, eq214_e2685_d_b41, eq214_e2685_d_b42, eq214_e2685_d_b43, eq214_e2685_d_b44, eq214_e2685_d_b45, eq214_e2685_d_b46, eq214_e2685_d_b47, eq214_e2685_d_b48, eq214_e2685_d_b49, eq214_e2685_d_b50, eq214_e2685_d_b51, eq214_e2685_d_b52, eq214_e2685_d_b53, eq214_e2685_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq214_value: f64 = eq214_e2687;
        let eq214_node_derivatives: [f64; 23] = [eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n10, eq214_e2687_d_n11, eq214_e2687_d_n12, eq214_e2687_d_n13, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22];
        let eq214_branch_derivatives: [f64; 55] = [eq214_e2687_d_b0, eq214_e2687_d_b1, eq214_e2687_d_b2, eq214_e2687_d_b3, eq214_e2687_d_b4, eq214_e2687_d_b5, eq214_e2687_d_b6, eq214_e2687_d_b7, eq214_e2687_d_b8, eq214_e2687_d_b9, eq214_e2687_d_b10, eq214_e2687_d_b11, eq214_e2687_d_b12, eq214_e2687_d_b13, eq214_e2687_d_b14, eq214_e2687_d_b15, eq214_e2687_d_b16, eq214_e2687_d_b17, eq214_e2687_d_b18, eq214_e2687_d_b19, eq214_e2687_d_b20, eq214_e2687_d_b21, eq214_e2687_d_b22, eq214_e2687_d_b23, eq214_e2687_d_b24, eq214_e2687_d_b25, eq214_e2687_d_b26, eq214_e2687_d_b27, eq214_e2687_d_b28, eq214_e2687_d_b29, eq214_e2687_d_b30, eq214_e2687_d_b31, eq214_e2687_d_b32, eq214_e2687_d_b33, eq214_e2687_d_b34, eq214_e2687_d_b35, eq214_e2687_d_b36, eq214_e2687_d_b37, eq214_e2687_d_b38, eq214_e2687_d_b39, eq214_e2687_d_b40, eq214_e2687_d_b41, eq214_e2687_d_b42, eq214_e2687_d_b43, eq214_e2687_d_b44, eq214_e2687_d_b45, eq214_e2687_d_b46, eq214_e2687_d_b47, eq214_e2687_d_b48, eq214_e2687_d_b49, eq214_e2687_d_b50, eq214_e2687_d_b51, eq214_e2687_d_b52, eq214_e2687_d_b53, eq214_e2687_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq214_value),
            &eq214_node_derivatives,
            &eq214_branch_derivatives,
            multiplicity,
        );
        let (eq215_e2699, eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n10, eq215_e2699_d_n11, eq215_e2699_d_n12, eq215_e2699_d_n13, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22, eq215_e2699_d_b0, eq215_e2699_d_b1, eq215_e2699_d_b2, eq215_e2699_d_b3, eq215_e2699_d_b4, eq215_e2699_d_b5, eq215_e2699_d_b6, eq215_e2699_d_b7, eq215_e2699_d_b8, eq215_e2699_d_b9, eq215_e2699_d_b10, eq215_e2699_d_b11, eq215_e2699_d_b12, eq215_e2699_d_b13, eq215_e2699_d_b14, eq215_e2699_d_b15, eq215_e2699_d_b16, eq215_e2699_d_b17, eq215_e2699_d_b18, eq215_e2699_d_b19, eq215_e2699_d_b20, eq215_e2699_d_b21, eq215_e2699_d_b22, eq215_e2699_d_b23, eq215_e2699_d_b24, eq215_e2699_d_b25, eq215_e2699_d_b26, eq215_e2699_d_b27, eq215_e2699_d_b28, eq215_e2699_d_b29, eq215_e2699_d_b30, eq215_e2699_d_b31, eq215_e2699_d_b32, eq215_e2699_d_b33, eq215_e2699_d_b34, eq215_e2699_d_b35, eq215_e2699_d_b36, eq215_e2699_d_b37, eq215_e2699_d_b38, eq215_e2699_d_b39, eq215_e2699_d_b40, eq215_e2699_d_b41, eq215_e2699_d_b42, eq215_e2699_d_b43, eq215_e2699_d_b44, eq215_e2699_d_b45, eq215_e2699_d_b46, eq215_e2699_d_b47, eq215_e2699_d_b48, eq215_e2699_d_b49, eq215_e2699_d_b50, eq215_e2699_d_b51, eq215_e2699_d_b52, eq215_e2699_d_b53, eq215_e2699_d_b54,) = {
    if ((!s.b[605]) && s.b[608]) {
        let eq215_e2695: f64 = (p.p254 * s.v[312]);
        let eq215_e2696: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 114, eq215_e2695);
        let eq215_e2696_d_n0: f64 = ((p.p254 * s.dn[312][0]) * ddt_scale);
        let eq215_e2696_d_n1: f64 = ((p.p254 * s.dn[312][1]) * ddt_scale);
        let eq215_e2696_d_n2: f64 = ((p.p254 * s.dn[312][2]) * ddt_scale);
        let eq215_e2696_d_n3: f64 = ((p.p254 * s.dn[312][3]) * ddt_scale);
        let eq215_e2696_d_n4: f64 = ((p.p254 * s.dn[312][4]) * ddt_scale);
        let eq215_e2696_d_n5: f64 = ((p.p254 * s.dn[312][5]) * ddt_scale);
        let eq215_e2696_d_n6: f64 = ((p.p254 * s.dn[312][6]) * ddt_scale);
        let eq215_e2696_d_n7: f64 = ((p.p254 * s.dn[312][7]) * ddt_scale);
        let eq215_e2696_d_n8: f64 = ((p.p254 * s.dn[312][8]) * ddt_scale);
        let eq215_e2696_d_n9: f64 = ((p.p254 * s.dn[312][9]) * ddt_scale);
        let eq215_e2696_d_n10: f64 = ((p.p254 * s.dn[312][10]) * ddt_scale);
        let eq215_e2696_d_n11: f64 = ((p.p254 * s.dn[312][11]) * ddt_scale);
        let eq215_e2696_d_n12: f64 = ((p.p254 * s.dn[312][12]) * ddt_scale);
        let eq215_e2696_d_n13: f64 = ((p.p254 * s.dn[312][13]) * ddt_scale);
        let eq215_e2696_d_n14: f64 = ((p.p254 * s.dn[312][14]) * ddt_scale);
        let eq215_e2696_d_n15: f64 = ((p.p254 * s.dn[312][15]) * ddt_scale);
        let eq215_e2696_d_n16: f64 = ((p.p254 * s.dn[312][16]) * ddt_scale);
        let eq215_e2696_d_n17: f64 = ((p.p254 * s.dn[312][17]) * ddt_scale);
        let eq215_e2696_d_n18: f64 = ((p.p254 * s.dn[312][18]) * ddt_scale);
        let eq215_e2696_d_n19: f64 = ((p.p254 * s.dn[312][19]) * ddt_scale);
        let eq215_e2696_d_n20: f64 = ((p.p254 * s.dn[312][20]) * ddt_scale);
        let eq215_e2696_d_n21: f64 = ((p.p254 * s.dn[312][21]) * ddt_scale);
        let eq215_e2696_d_n22: f64 = ((p.p254 * s.dn[312][22]) * ddt_scale);
        let eq215_e2696_d_b0: f64 = ((p.p254 * s.db[312][0]) * ddt_scale);
        let eq215_e2696_d_b1: f64 = ((p.p254 * s.db[312][1]) * ddt_scale);
        let eq215_e2696_d_b2: f64 = ((p.p254 * s.db[312][2]) * ddt_scale);
        let eq215_e2696_d_b3: f64 = ((p.p254 * s.db[312][3]) * ddt_scale);
        let eq215_e2696_d_b4: f64 = ((p.p254 * s.db[312][4]) * ddt_scale);
        let eq215_e2696_d_b5: f64 = ((p.p254 * s.db[312][5]) * ddt_scale);
        let eq215_e2696_d_b6: f64 = ((p.p254 * s.db[312][6]) * ddt_scale);
        let eq215_e2696_d_b7: f64 = ((p.p254 * s.db[312][7]) * ddt_scale);
        let eq215_e2696_d_b8: f64 = ((p.p254 * s.db[312][8]) * ddt_scale);
        let eq215_e2696_d_b9: f64 = ((p.p254 * s.db[312][9]) * ddt_scale);
        let eq215_e2696_d_b10: f64 = ((p.p254 * s.db[312][10]) * ddt_scale);
        let eq215_e2696_d_b11: f64 = ((p.p254 * s.db[312][11]) * ddt_scale);
        let eq215_e2696_d_b12: f64 = ((p.p254 * s.db[312][12]) * ddt_scale);
        let eq215_e2696_d_b13: f64 = ((p.p254 * s.db[312][13]) * ddt_scale);
        let eq215_e2696_d_b14: f64 = ((p.p254 * s.db[312][14]) * ddt_scale);
        let eq215_e2696_d_b15: f64 = ((p.p254 * s.db[312][15]) * ddt_scale);
        let eq215_e2696_d_b16: f64 = ((p.p254 * s.db[312][16]) * ddt_scale);
        let eq215_e2696_d_b17: f64 = ((p.p254 * s.db[312][17]) * ddt_scale);
        let eq215_e2696_d_b18: f64 = ((p.p254 * s.db[312][18]) * ddt_scale);
        let eq215_e2696_d_b19: f64 = ((p.p254 * s.db[312][19]) * ddt_scale);
        let eq215_e2696_d_b20: f64 = ((p.p254 * s.db[312][20]) * ddt_scale);
        let eq215_e2696_d_b21: f64 = ((p.p254 * s.db[312][21]) * ddt_scale);
        let eq215_e2696_d_b22: f64 = ((p.p254 * s.db[312][22]) * ddt_scale);
        let eq215_e2696_d_b23: f64 = ((p.p254 * s.db[312][23]) * ddt_scale);
        let eq215_e2696_d_b24: f64 = ((p.p254 * s.db[312][24]) * ddt_scale);
        let eq215_e2696_d_b25: f64 = ((p.p254 * s.db[312][25]) * ddt_scale);
        let eq215_e2696_d_b26: f64 = ((p.p254 * s.db[312][26]) * ddt_scale);
        let eq215_e2696_d_b27: f64 = ((p.p254 * s.db[312][27]) * ddt_scale);
        let eq215_e2696_d_b28: f64 = ((p.p254 * s.db[312][28]) * ddt_scale);
        let eq215_e2696_d_b29: f64 = ((p.p254 * s.db[312][29]) * ddt_scale);
        let eq215_e2696_d_b30: f64 = ((p.p254 * s.db[312][30]) * ddt_scale);
        let eq215_e2696_d_b31: f64 = ((p.p254 * s.db[312][31]) * ddt_scale);
        let eq215_e2696_d_b32: f64 = ((p.p254 * s.db[312][32]) * ddt_scale);
        let eq215_e2696_d_b33: f64 = ((p.p254 * s.db[312][33]) * ddt_scale);
        let eq215_e2696_d_b34: f64 = ((p.p254 * s.db[312][34]) * ddt_scale);
        let eq215_e2696_d_b35: f64 = ((p.p254 * s.db[312][35]) * ddt_scale);
        let eq215_e2696_d_b36: f64 = ((p.p254 * s.db[312][36]) * ddt_scale);
        let eq215_e2696_d_b37: f64 = ((p.p254 * s.db[312][37]) * ddt_scale);
        let eq215_e2696_d_b38: f64 = ((p.p254 * s.db[312][38]) * ddt_scale);
        let eq215_e2696_d_b39: f64 = ((p.p254 * s.db[312][39]) * ddt_scale);
        let eq215_e2696_d_b40: f64 = ((p.p254 * s.db[312][40]) * ddt_scale);
        let eq215_e2696_d_b41: f64 = ((p.p254 * s.db[312][41]) * ddt_scale);
        let eq215_e2696_d_b42: f64 = ((p.p254 * s.db[312][42]) * ddt_scale);
        let eq215_e2696_d_b43: f64 = ((p.p254 * s.db[312][43]) * ddt_scale);
        let eq215_e2696_d_b44: f64 = ((p.p254 * s.db[312][44]) * ddt_scale);
        let eq215_e2696_d_b45: f64 = ((p.p254 * s.db[312][45]) * ddt_scale);
        let eq215_e2696_d_b46: f64 = ((p.p254 * s.db[312][46]) * ddt_scale);
        let eq215_e2696_d_b47: f64 = ((p.p254 * s.db[312][47]) * ddt_scale);
        let eq215_e2696_d_b48: f64 = ((p.p254 * s.db[312][48]) * ddt_scale);
        let eq215_e2696_d_b49: f64 = ((p.p254 * s.db[312][49]) * ddt_scale);
        let eq215_e2696_d_b50: f64 = ((p.p254 * s.db[312][50]) * ddt_scale);
        let eq215_e2696_d_b51: f64 = ((p.p254 * s.db[312][51]) * ddt_scale);
        let eq215_e2696_d_b52: f64 = ((p.p254 * s.db[312][52]) * ddt_scale);
        let eq215_e2696_d_b53: f64 = ((p.p254 * s.db[312][53]) * ddt_scale);
        let eq215_e2696_d_b54: f64 = ((p.p254 * s.db[312][54]) * ddt_scale);
        let eq215_e2697: f64 = (p.p7 * eq215_e2696);
        let eq215_e2697_d_n0: f64 = (p.p7 * eq215_e2696_d_n0);
        let eq215_e2697_d_n1: f64 = (p.p7 * eq215_e2696_d_n1);
        let eq215_e2697_d_n2: f64 = (p.p7 * eq215_e2696_d_n2);
        let eq215_e2697_d_n3: f64 = (p.p7 * eq215_e2696_d_n3);
        let eq215_e2697_d_n4: f64 = (p.p7 * eq215_e2696_d_n4);
        let eq215_e2697_d_n5: f64 = (p.p7 * eq215_e2696_d_n5);
        let eq215_e2697_d_n6: f64 = (p.p7 * eq215_e2696_d_n6);
        let eq215_e2697_d_n7: f64 = (p.p7 * eq215_e2696_d_n7);
        let eq215_e2697_d_n8: f64 = (p.p7 * eq215_e2696_d_n8);
        let eq215_e2697_d_n9: f64 = (p.p7 * eq215_e2696_d_n9);
        let eq215_e2697_d_n10: f64 = (p.p7 * eq215_e2696_d_n10);
        let eq215_e2697_d_n11: f64 = (p.p7 * eq215_e2696_d_n11);
        let eq215_e2697_d_n12: f64 = (p.p7 * eq215_e2696_d_n12);
        let eq215_e2697_d_n13: f64 = (p.p7 * eq215_e2696_d_n13);
        let eq215_e2697_d_n14: f64 = (p.p7 * eq215_e2696_d_n14);
        let eq215_e2697_d_n15: f64 = (p.p7 * eq215_e2696_d_n15);
        let eq215_e2697_d_n16: f64 = (p.p7 * eq215_e2696_d_n16);
        let eq215_e2697_d_n17: f64 = (p.p7 * eq215_e2696_d_n17);
        let eq215_e2697_d_n18: f64 = (p.p7 * eq215_e2696_d_n18);
        let eq215_e2697_d_n19: f64 = (p.p7 * eq215_e2696_d_n19);
        let eq215_e2697_d_n20: f64 = (p.p7 * eq215_e2696_d_n20);
        let eq215_e2697_d_n21: f64 = (p.p7 * eq215_e2696_d_n21);
        let eq215_e2697_d_n22: f64 = (p.p7 * eq215_e2696_d_n22);
        let eq215_e2697_d_b0: f64 = (p.p7 * eq215_e2696_d_b0);
        let eq215_e2697_d_b1: f64 = (p.p7 * eq215_e2696_d_b1);
        let eq215_e2697_d_b2: f64 = (p.p7 * eq215_e2696_d_b2);
        let eq215_e2697_d_b3: f64 = (p.p7 * eq215_e2696_d_b3);
        let eq215_e2697_d_b4: f64 = (p.p7 * eq215_e2696_d_b4);
        let eq215_e2697_d_b5: f64 = (p.p7 * eq215_e2696_d_b5);
        let eq215_e2697_d_b6: f64 = (p.p7 * eq215_e2696_d_b6);
        let eq215_e2697_d_b7: f64 = (p.p7 * eq215_e2696_d_b7);
        let eq215_e2697_d_b8: f64 = (p.p7 * eq215_e2696_d_b8);
        let eq215_e2697_d_b9: f64 = (p.p7 * eq215_e2696_d_b9);
        let eq215_e2697_d_b10: f64 = (p.p7 * eq215_e2696_d_b10);
        let eq215_e2697_d_b11: f64 = (p.p7 * eq215_e2696_d_b11);
        let eq215_e2697_d_b12: f64 = (p.p7 * eq215_e2696_d_b12);
        let eq215_e2697_d_b13: f64 = (p.p7 * eq215_e2696_d_b13);
        let eq215_e2697_d_b14: f64 = (p.p7 * eq215_e2696_d_b14);
        let eq215_e2697_d_b15: f64 = (p.p7 * eq215_e2696_d_b15);
        let eq215_e2697_d_b16: f64 = (p.p7 * eq215_e2696_d_b16);
        let eq215_e2697_d_b17: f64 = (p.p7 * eq215_e2696_d_b17);
        let eq215_e2697_d_b18: f64 = (p.p7 * eq215_e2696_d_b18);
        let eq215_e2697_d_b19: f64 = (p.p7 * eq215_e2696_d_b19);
        let eq215_e2697_d_b20: f64 = (p.p7 * eq215_e2696_d_b20);
        let eq215_e2697_d_b21: f64 = (p.p7 * eq215_e2696_d_b21);
        let eq215_e2697_d_b22: f64 = (p.p7 * eq215_e2696_d_b22);
        let eq215_e2697_d_b23: f64 = (p.p7 * eq215_e2696_d_b23);
        let eq215_e2697_d_b24: f64 = (p.p7 * eq215_e2696_d_b24);
        let eq215_e2697_d_b25: f64 = (p.p7 * eq215_e2696_d_b25);
        let eq215_e2697_d_b26: f64 = (p.p7 * eq215_e2696_d_b26);
        let eq215_e2697_d_b27: f64 = (p.p7 * eq215_e2696_d_b27);
        let eq215_e2697_d_b28: f64 = (p.p7 * eq215_e2696_d_b28);
        let eq215_e2697_d_b29: f64 = (p.p7 * eq215_e2696_d_b29);
        let eq215_e2697_d_b30: f64 = (p.p7 * eq215_e2696_d_b30);
        let eq215_e2697_d_b31: f64 = (p.p7 * eq215_e2696_d_b31);
        let eq215_e2697_d_b32: f64 = (p.p7 * eq215_e2696_d_b32);
        let eq215_e2697_d_b33: f64 = (p.p7 * eq215_e2696_d_b33);
        let eq215_e2697_d_b34: f64 = (p.p7 * eq215_e2696_d_b34);
        let eq215_e2697_d_b35: f64 = (p.p7 * eq215_e2696_d_b35);
        let eq215_e2697_d_b36: f64 = (p.p7 * eq215_e2696_d_b36);
        let eq215_e2697_d_b37: f64 = (p.p7 * eq215_e2696_d_b37);
        let eq215_e2697_d_b38: f64 = (p.p7 * eq215_e2696_d_b38);
        let eq215_e2697_d_b39: f64 = (p.p7 * eq215_e2696_d_b39);
        let eq215_e2697_d_b40: f64 = (p.p7 * eq215_e2696_d_b40);
        let eq215_e2697_d_b41: f64 = (p.p7 * eq215_e2696_d_b41);
        let eq215_e2697_d_b42: f64 = (p.p7 * eq215_e2696_d_b42);
        let eq215_e2697_d_b43: f64 = (p.p7 * eq215_e2696_d_b43);
        let eq215_e2697_d_b44: f64 = (p.p7 * eq215_e2696_d_b44);
        let eq215_e2697_d_b45: f64 = (p.p7 * eq215_e2696_d_b45);
        let eq215_e2697_d_b46: f64 = (p.p7 * eq215_e2696_d_b46);
        let eq215_e2697_d_b47: f64 = (p.p7 * eq215_e2696_d_b47);
        let eq215_e2697_d_b48: f64 = (p.p7 * eq215_e2696_d_b48);
        let eq215_e2697_d_b49: f64 = (p.p7 * eq215_e2696_d_b49);
        let eq215_e2697_d_b50: f64 = (p.p7 * eq215_e2696_d_b50);
        let eq215_e2697_d_b51: f64 = (p.p7 * eq215_e2696_d_b51);
        let eq215_e2697_d_b52: f64 = (p.p7 * eq215_e2696_d_b52);
        let eq215_e2697_d_b53: f64 = (p.p7 * eq215_e2696_d_b53);
        let eq215_e2697_d_b54: f64 = (p.p7 * eq215_e2696_d_b54);
        (eq215_e2697, eq215_e2697_d_n0, eq215_e2697_d_n1, eq215_e2697_d_n2, eq215_e2697_d_n3, eq215_e2697_d_n4, eq215_e2697_d_n5, eq215_e2697_d_n6, eq215_e2697_d_n7, eq215_e2697_d_n8, eq215_e2697_d_n9, eq215_e2697_d_n10, eq215_e2697_d_n11, eq215_e2697_d_n12, eq215_e2697_d_n13, eq215_e2697_d_n14, eq215_e2697_d_n15, eq215_e2697_d_n16, eq215_e2697_d_n17, eq215_e2697_d_n18, eq215_e2697_d_n19, eq215_e2697_d_n20, eq215_e2697_d_n21, eq215_e2697_d_n22, eq215_e2697_d_b0, eq215_e2697_d_b1, eq215_e2697_d_b2, eq215_e2697_d_b3, eq215_e2697_d_b4, eq215_e2697_d_b5, eq215_e2697_d_b6, eq215_e2697_d_b7, eq215_e2697_d_b8, eq215_e2697_d_b9, eq215_e2697_d_b10, eq215_e2697_d_b11, eq215_e2697_d_b12, eq215_e2697_d_b13, eq215_e2697_d_b14, eq215_e2697_d_b15, eq215_e2697_d_b16, eq215_e2697_d_b17, eq215_e2697_d_b18, eq215_e2697_d_b19, eq215_e2697_d_b20, eq215_e2697_d_b21, eq215_e2697_d_b22, eq215_e2697_d_b23, eq215_e2697_d_b24, eq215_e2697_d_b25, eq215_e2697_d_b26, eq215_e2697_d_b27, eq215_e2697_d_b28, eq215_e2697_d_b29, eq215_e2697_d_b30, eq215_e2697_d_b31, eq215_e2697_d_b32, eq215_e2697_d_b33, eq215_e2697_d_b34, eq215_e2697_d_b35, eq215_e2697_d_b36, eq215_e2697_d_b37, eq215_e2697_d_b38, eq215_e2697_d_b39, eq215_e2697_d_b40, eq215_e2697_d_b41, eq215_e2697_d_b42, eq215_e2697_d_b43, eq215_e2697_d_b44, eq215_e2697_d_b45, eq215_e2697_d_b46, eq215_e2697_d_b47, eq215_e2697_d_b48, eq215_e2697_d_b49, eq215_e2697_d_b50, eq215_e2697_d_b51, eq215_e2697_d_b52, eq215_e2697_d_b53, eq215_e2697_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq215_value: f64 = eq215_e2699;
        let eq215_node_derivatives: [f64; 23] = [eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n10, eq215_e2699_d_n11, eq215_e2699_d_n12, eq215_e2699_d_n13, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22];
        let eq215_branch_derivatives: [f64; 55] = [eq215_e2699_d_b0, eq215_e2699_d_b1, eq215_e2699_d_b2, eq215_e2699_d_b3, eq215_e2699_d_b4, eq215_e2699_d_b5, eq215_e2699_d_b6, eq215_e2699_d_b7, eq215_e2699_d_b8, eq215_e2699_d_b9, eq215_e2699_d_b10, eq215_e2699_d_b11, eq215_e2699_d_b12, eq215_e2699_d_b13, eq215_e2699_d_b14, eq215_e2699_d_b15, eq215_e2699_d_b16, eq215_e2699_d_b17, eq215_e2699_d_b18, eq215_e2699_d_b19, eq215_e2699_d_b20, eq215_e2699_d_b21, eq215_e2699_d_b22, eq215_e2699_d_b23, eq215_e2699_d_b24, eq215_e2699_d_b25, eq215_e2699_d_b26, eq215_e2699_d_b27, eq215_e2699_d_b28, eq215_e2699_d_b29, eq215_e2699_d_b30, eq215_e2699_d_b31, eq215_e2699_d_b32, eq215_e2699_d_b33, eq215_e2699_d_b34, eq215_e2699_d_b35, eq215_e2699_d_b36, eq215_e2699_d_b37, eq215_e2699_d_b38, eq215_e2699_d_b39, eq215_e2699_d_b40, eq215_e2699_d_b41, eq215_e2699_d_b42, eq215_e2699_d_b43, eq215_e2699_d_b44, eq215_e2699_d_b45, eq215_e2699_d_b46, eq215_e2699_d_b47, eq215_e2699_d_b48, eq215_e2699_d_b49, eq215_e2699_d_b50, eq215_e2699_d_b51, eq215_e2699_d_b52, eq215_e2699_d_b53, eq215_e2699_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq215_value),
            &eq215_node_derivatives,
            &eq215_branch_derivatives,
            multiplicity,
        );
        let eq216_e2702: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 115, var_qfr);
        let eq216_e2703: f64 = (p.p7 * eq216_e2702);
        let eq216_e2703_d_n0: f64 = (p.p7 * (var_qfr_dn0 * ddt_scale));
        let eq216_e2703_d_n1: f64 = (p.p7 * (var_qfr_dn1 * ddt_scale));
        let eq216_e2703_d_n2: f64 = (p.p7 * (var_qfr_dn2 * ddt_scale));
        let eq216_e2703_d_n3: f64 = (p.p7 * (var_qfr_dn3 * ddt_scale));
        let eq216_e2703_d_n4: f64 = (p.p7 * (var_qfr_dn4 * ddt_scale));
        let eq216_e2703_d_n5: f64 = (p.p7 * (var_qfr_dn5 * ddt_scale));
        let eq216_e2703_d_n6: f64 = (p.p7 * (var_qfr_dn6 * ddt_scale));
        let eq216_e2703_d_n7: f64 = (p.p7 * (var_qfr_dn7 * ddt_scale));
        let eq216_e2703_d_n8: f64 = (p.p7 * (var_qfr_dn8 * ddt_scale));
        let eq216_e2703_d_n9: f64 = (p.p7 * (var_qfr_dn9 * ddt_scale));
        let eq216_e2703_d_n10: f64 = (p.p7 * (var_qfr_dn10 * ddt_scale));
        let eq216_e2703_d_n11: f64 = (p.p7 * (var_qfr_dn11 * ddt_scale));
        let eq216_e2703_d_n12: f64 = (p.p7 * (var_qfr_dn12 * ddt_scale));
        let eq216_e2703_d_n13: f64 = (p.p7 * (var_qfr_dn13 * ddt_scale));
        let eq216_e2703_d_n14: f64 = (p.p7 * (var_qfr_dn14 * ddt_scale));
        let eq216_e2703_d_n15: f64 = (p.p7 * (var_qfr_dn15 * ddt_scale));
        let eq216_e2703_d_n16: f64 = (p.p7 * (var_qfr_dn16 * ddt_scale));
        let eq216_e2703_d_n17: f64 = (p.p7 * (var_qfr_dn17 * ddt_scale));
        let eq216_e2703_d_n18: f64 = (p.p7 * (var_qfr_dn18 * ddt_scale));
        let eq216_e2703_d_n19: f64 = (p.p7 * (var_qfr_dn19 * ddt_scale));
        let eq216_e2703_d_n20: f64 = (p.p7 * (var_qfr_dn20 * ddt_scale));
        let eq216_e2703_d_n21: f64 = (p.p7 * (var_qfr_dn21 * ddt_scale));
        let eq216_e2703_d_n22: f64 = (p.p7 * (var_qfr_dn22 * ddt_scale));
        let eq216_e2703_d_b0: f64 = (p.p7 * (var_qfr_db0 * ddt_scale));
        let eq216_e2703_d_b1: f64 = (p.p7 * (var_qfr_db1 * ddt_scale));
        let eq216_e2703_d_b2: f64 = (p.p7 * (var_qfr_db2 * ddt_scale));
        let eq216_e2703_d_b3: f64 = (p.p7 * (var_qfr_db3 * ddt_scale));
        let eq216_e2703_d_b4: f64 = (p.p7 * (var_qfr_db4 * ddt_scale));
        let eq216_e2703_d_b5: f64 = (p.p7 * (var_qfr_db5 * ddt_scale));
        let eq216_e2703_d_b6: f64 = (p.p7 * (var_qfr_db6 * ddt_scale));
        let eq216_e2703_d_b7: f64 = (p.p7 * (var_qfr_db7 * ddt_scale));
        let eq216_e2703_d_b8: f64 = (p.p7 * (var_qfr_db8 * ddt_scale));
        let eq216_e2703_d_b9: f64 = (p.p7 * (var_qfr_db9 * ddt_scale));
        let eq216_e2703_d_b10: f64 = (p.p7 * (var_qfr_db10 * ddt_scale));
        let eq216_e2703_d_b11: f64 = (p.p7 * (var_qfr_db11 * ddt_scale));
        let eq216_e2703_d_b12: f64 = (p.p7 * (var_qfr_db12 * ddt_scale));
        let eq216_e2703_d_b13: f64 = (p.p7 * (var_qfr_db13 * ddt_scale));
        let eq216_e2703_d_b14: f64 = (p.p7 * (var_qfr_db14 * ddt_scale));
        let eq216_e2703_d_b15: f64 = (p.p7 * (var_qfr_db15 * ddt_scale));
        let eq216_e2703_d_b16: f64 = (p.p7 * (var_qfr_db16 * ddt_scale));
        let eq216_e2703_d_b17: f64 = (p.p7 * (var_qfr_db17 * ddt_scale));
        let eq216_e2703_d_b18: f64 = (p.p7 * (var_qfr_db18 * ddt_scale));
        let eq216_e2703_d_b19: f64 = (p.p7 * (var_qfr_db19 * ddt_scale));
        let eq216_e2703_d_b20: f64 = (p.p7 * (var_qfr_db20 * ddt_scale));
        let eq216_e2703_d_b21: f64 = (p.p7 * (var_qfr_db21 * ddt_scale));
        let eq216_e2703_d_b22: f64 = (p.p7 * (var_qfr_db22 * ddt_scale));
        let eq216_e2703_d_b23: f64 = (p.p7 * (var_qfr_db23 * ddt_scale));
        let eq216_e2703_d_b24: f64 = (p.p7 * (var_qfr_db24 * ddt_scale));
        let eq216_e2703_d_b25: f64 = (p.p7 * (var_qfr_db25 * ddt_scale));
        let eq216_e2703_d_b26: f64 = (p.p7 * (var_qfr_db26 * ddt_scale));
        let eq216_e2703_d_b27: f64 = (p.p7 * (var_qfr_db27 * ddt_scale));
        let eq216_e2703_d_b28: f64 = (p.p7 * (var_qfr_db28 * ddt_scale));
        let eq216_e2703_d_b29: f64 = (p.p7 * (var_qfr_db29 * ddt_scale));
        let eq216_e2703_d_b30: f64 = (p.p7 * (var_qfr_db30 * ddt_scale));
        let eq216_e2703_d_b31: f64 = (p.p7 * (var_qfr_db31 * ddt_scale));
        let eq216_e2703_d_b32: f64 = (p.p7 * (var_qfr_db32 * ddt_scale));
        let eq216_e2703_d_b33: f64 = (p.p7 * (var_qfr_db33 * ddt_scale));
        let eq216_e2703_d_b34: f64 = (p.p7 * (var_qfr_db34 * ddt_scale));
        let eq216_e2703_d_b35: f64 = (p.p7 * (var_qfr_db35 * ddt_scale));
        let eq216_e2703_d_b36: f64 = (p.p7 * (var_qfr_db36 * ddt_scale));
        let eq216_e2703_d_b37: f64 = (p.p7 * (var_qfr_db37 * ddt_scale));
        let eq216_e2703_d_b38: f64 = (p.p7 * (var_qfr_db38 * ddt_scale));
        let eq216_e2703_d_b39: f64 = (p.p7 * (var_qfr_db39 * ddt_scale));
        let eq216_e2703_d_b40: f64 = (p.p7 * (var_qfr_db40 * ddt_scale));
        let eq216_e2703_d_b41: f64 = (p.p7 * (var_qfr_db41 * ddt_scale));
        let eq216_e2703_d_b42: f64 = (p.p7 * (var_qfr_db42 * ddt_scale));
        let eq216_e2703_d_b43: f64 = (p.p7 * (var_qfr_db43 * ddt_scale));
        let eq216_e2703_d_b44: f64 = (p.p7 * (var_qfr_db44 * ddt_scale));
        let eq216_e2703_d_b45: f64 = (p.p7 * (var_qfr_db45 * ddt_scale));
        let eq216_e2703_d_b46: f64 = (p.p7 * (var_qfr_db46 * ddt_scale));
        let eq216_e2703_d_b47: f64 = (p.p7 * (var_qfr_db47 * ddt_scale));
        let eq216_e2703_d_b48: f64 = (p.p7 * (var_qfr_db48 * ddt_scale));
        let eq216_e2703_d_b49: f64 = (p.p7 * (var_qfr_db49 * ddt_scale));
        let eq216_e2703_d_b50: f64 = (p.p7 * (var_qfr_db50 * ddt_scale));
        let eq216_e2703_d_b51: f64 = (p.p7 * (var_qfr_db51 * ddt_scale));
        let eq216_e2703_d_b52: f64 = (p.p7 * (var_qfr_db52 * ddt_scale));
        let eq216_e2703_d_b53: f64 = (p.p7 * (var_qfr_db53 * ddt_scale));
        let eq216_e2703_d_b54: f64 = (p.p7 * (var_qfr_db54 * ddt_scale));
        let eq216_value: f64 = eq216_e2703;
        let eq216_node_derivatives: [f64; 23] = [eq216_e2703_d_n0, eq216_e2703_d_n1, eq216_e2703_d_n2, eq216_e2703_d_n3, eq216_e2703_d_n4, eq216_e2703_d_n5, eq216_e2703_d_n6, eq216_e2703_d_n7, eq216_e2703_d_n8, eq216_e2703_d_n9, eq216_e2703_d_n10, eq216_e2703_d_n11, eq216_e2703_d_n12, eq216_e2703_d_n13, eq216_e2703_d_n14, eq216_e2703_d_n15, eq216_e2703_d_n16, eq216_e2703_d_n17, eq216_e2703_d_n18, eq216_e2703_d_n19, eq216_e2703_d_n20, eq216_e2703_d_n21, eq216_e2703_d_n22];
        let eq216_branch_derivatives: [f64; 55] = [eq216_e2703_d_b0, eq216_e2703_d_b1, eq216_e2703_d_b2, eq216_e2703_d_b3, eq216_e2703_d_b4, eq216_e2703_d_b5, eq216_e2703_d_b6, eq216_e2703_d_b7, eq216_e2703_d_b8, eq216_e2703_d_b9, eq216_e2703_d_b10, eq216_e2703_d_b11, eq216_e2703_d_b12, eq216_e2703_d_b13, eq216_e2703_d_b14, eq216_e2703_d_b15, eq216_e2703_d_b16, eq216_e2703_d_b17, eq216_e2703_d_b18, eq216_e2703_d_b19, eq216_e2703_d_b20, eq216_e2703_d_b21, eq216_e2703_d_b22, eq216_e2703_d_b23, eq216_e2703_d_b24, eq216_e2703_d_b25, eq216_e2703_d_b26, eq216_e2703_d_b27, eq216_e2703_d_b28, eq216_e2703_d_b29, eq216_e2703_d_b30, eq216_e2703_d_b31, eq216_e2703_d_b32, eq216_e2703_d_b33, eq216_e2703_d_b34, eq216_e2703_d_b35, eq216_e2703_d_b36, eq216_e2703_d_b37, eq216_e2703_d_b38, eq216_e2703_d_b39, eq216_e2703_d_b40, eq216_e2703_d_b41, eq216_e2703_d_b42, eq216_e2703_d_b43, eq216_e2703_d_b44, eq216_e2703_d_b45, eq216_e2703_d_b46, eq216_e2703_d_b47, eq216_e2703_d_b48, eq216_e2703_d_b49, eq216_e2703_d_b50, eq216_e2703_d_b51, eq216_e2703_d_b52, eq216_e2703_d_b53, eq216_e2703_d_b54];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq216_value),
            &eq216_node_derivatives,
            &eq216_branch_derivatives,
            multiplicity,
        );
        let eq217_e2707: f64 = (p.p4 * p.p5);
        let eq217_e2709: f64 = (eq217_e2707 * p.p220);
        let eq217_e2711: f64 = (eq217_e2709 * (nv1 - nv2));
        let eq217_e2712: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 116, eq217_e2711);
        let eq217_e2713: f64 = (p.p7 * eq217_e2712);
        let eq217_e2713_d_n1: f64 = (p.p7 * (eq217_e2709 * ddt_scale));
        let eq217_e2713_d_n2: f64 = (p.p7 * ((-eq217_e2709) * ddt_scale));
        let eq217_value: f64 = eq217_e2713;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (eq217_value),
            1,
            multiplicity * (eq217_e2713_d_n1),
            2,
            multiplicity * (eq217_e2713_d_n2),
        );
    }

    pub(super) fn stamp_transient_equations_block_47(
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
        var_guard576: f64,
        var_qfr3: f64,
        var_qfr3_db0: f64,
        var_qfr3_db1: f64,
        var_qfr3_db10: f64,
        var_qfr3_db11: f64,
        var_qfr3_db12: f64,
        var_qfr3_db13: f64,
        var_qfr3_db14: f64,
        var_qfr3_db15: f64,
        var_qfr3_db16: f64,
        var_qfr3_db17: f64,
        var_qfr3_db18: f64,
        var_qfr3_db19: f64,
        var_qfr3_db2: f64,
        var_qfr3_db20: f64,
        var_qfr3_db21: f64,
        var_qfr3_db22: f64,
        var_qfr3_db23: f64,
        var_qfr3_db24: f64,
        var_qfr3_db25: f64,
        var_qfr3_db26: f64,
        var_qfr3_db27: f64,
        var_qfr3_db28: f64,
        var_qfr3_db29: f64,
        var_qfr3_db3: f64,
        var_qfr3_db30: f64,
        var_qfr3_db31: f64,
        var_qfr3_db32: f64,
        var_qfr3_db33: f64,
        var_qfr3_db34: f64,
        var_qfr3_db35: f64,
        var_qfr3_db36: f64,
        var_qfr3_db37: f64,
        var_qfr3_db38: f64,
        var_qfr3_db39: f64,
        var_qfr3_db4: f64,
        var_qfr3_db40: f64,
        var_qfr3_db41: f64,
        var_qfr3_db42: f64,
        var_qfr3_db43: f64,
        var_qfr3_db44: f64,
        var_qfr3_db45: f64,
        var_qfr3_db46: f64,
        var_qfr3_db47: f64,
        var_qfr3_db48: f64,
        var_qfr3_db49: f64,
        var_qfr3_db5: f64,
        var_qfr3_db50: f64,
        var_qfr3_db51: f64,
        var_qfr3_db52: f64,
        var_qfr3_db53: f64,
        var_qfr3_db54: f64,
        var_qfr3_db6: f64,
        var_qfr3_db7: f64,
        var_qfr3_db8: f64,
        var_qfr3_db9: f64,
        var_qfr3_dn0: f64,
        var_qfr3_dn1: f64,
        var_qfr3_dn10: f64,
        var_qfr3_dn11: f64,
        var_qfr3_dn12: f64,
        var_qfr3_dn13: f64,
        var_qfr3_dn14: f64,
        var_qfr3_dn15: f64,
        var_qfr3_dn16: f64,
        var_qfr3_dn17: f64,
        var_qfr3_dn18: f64,
        var_qfr3_dn19: f64,
        var_qfr3_dn2: f64,
        var_qfr3_dn20: f64,
        var_qfr3_dn21: f64,
        var_qfr3_dn22: f64,
        var_qfr3_dn3: f64,
        var_qfr3_dn4: f64,
        var_qfr3_dn5: f64,
        var_qfr3_dn6: f64,
        var_qfr3_dn7: f64,
        var_qfr3_dn8: f64,
        var_qfr3_dn9: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq218_e2716: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 117, s.v[196]);
        let eq218_e2717: f64 = (p.p7 * eq218_e2716);
        let eq218_e2717_d_n0: f64 = (p.p7 * (s.dn[196][0] * ddt_scale));
        let eq218_e2717_d_n1: f64 = (p.p7 * (s.dn[196][1] * ddt_scale));
        let eq218_e2717_d_n2: f64 = (p.p7 * (s.dn[196][2] * ddt_scale));
        let eq218_e2717_d_n3: f64 = (p.p7 * (s.dn[196][3] * ddt_scale));
        let eq218_e2717_d_n4: f64 = (p.p7 * (s.dn[196][4] * ddt_scale));
        let eq218_e2717_d_n5: f64 = (p.p7 * (s.dn[196][5] * ddt_scale));
        let eq218_e2717_d_n6: f64 = (p.p7 * (s.dn[196][6] * ddt_scale));
        let eq218_e2717_d_n7: f64 = (p.p7 * (s.dn[196][7] * ddt_scale));
        let eq218_e2717_d_n8: f64 = (p.p7 * (s.dn[196][8] * ddt_scale));
        let eq218_e2717_d_n9: f64 = (p.p7 * (s.dn[196][9] * ddt_scale));
        let eq218_e2717_d_n10: f64 = (p.p7 * (s.dn[196][10] * ddt_scale));
        let eq218_e2717_d_n11: f64 = (p.p7 * (s.dn[196][11] * ddt_scale));
        let eq218_e2717_d_n12: f64 = (p.p7 * (s.dn[196][12] * ddt_scale));
        let eq218_e2717_d_n13: f64 = (p.p7 * (s.dn[196][13] * ddt_scale));
        let eq218_e2717_d_n14: f64 = (p.p7 * (s.dn[196][14] * ddt_scale));
        let eq218_e2717_d_n15: f64 = (p.p7 * (s.dn[196][15] * ddt_scale));
        let eq218_e2717_d_n16: f64 = (p.p7 * (s.dn[196][16] * ddt_scale));
        let eq218_e2717_d_n17: f64 = (p.p7 * (s.dn[196][17] * ddt_scale));
        let eq218_e2717_d_n18: f64 = (p.p7 * (s.dn[196][18] * ddt_scale));
        let eq218_e2717_d_n19: f64 = (p.p7 * (s.dn[196][19] * ddt_scale));
        let eq218_e2717_d_n20: f64 = (p.p7 * (s.dn[196][20] * ddt_scale));
        let eq218_e2717_d_n21: f64 = (p.p7 * (s.dn[196][21] * ddt_scale));
        let eq218_e2717_d_n22: f64 = (p.p7 * (s.dn[196][22] * ddt_scale));
        let eq218_e2717_d_b0: f64 = (p.p7 * (s.db[196][0] * ddt_scale));
        let eq218_e2717_d_b1: f64 = (p.p7 * (s.db[196][1] * ddt_scale));
        let eq218_e2717_d_b2: f64 = (p.p7 * (s.db[196][2] * ddt_scale));
        let eq218_e2717_d_b3: f64 = (p.p7 * (s.db[196][3] * ddt_scale));
        let eq218_e2717_d_b4: f64 = (p.p7 * (s.db[196][4] * ddt_scale));
        let eq218_e2717_d_b5: f64 = (p.p7 * (s.db[196][5] * ddt_scale));
        let eq218_e2717_d_b6: f64 = (p.p7 * (s.db[196][6] * ddt_scale));
        let eq218_e2717_d_b7: f64 = (p.p7 * (s.db[196][7] * ddt_scale));
        let eq218_e2717_d_b8: f64 = (p.p7 * (s.db[196][8] * ddt_scale));
        let eq218_e2717_d_b9: f64 = (p.p7 * (s.db[196][9] * ddt_scale));
        let eq218_e2717_d_b10: f64 = (p.p7 * (s.db[196][10] * ddt_scale));
        let eq218_e2717_d_b11: f64 = (p.p7 * (s.db[196][11] * ddt_scale));
        let eq218_e2717_d_b12: f64 = (p.p7 * (s.db[196][12] * ddt_scale));
        let eq218_e2717_d_b13: f64 = (p.p7 * (s.db[196][13] * ddt_scale));
        let eq218_e2717_d_b14: f64 = (p.p7 * (s.db[196][14] * ddt_scale));
        let eq218_e2717_d_b15: f64 = (p.p7 * (s.db[196][15] * ddt_scale));
        let eq218_e2717_d_b16: f64 = (p.p7 * (s.db[196][16] * ddt_scale));
        let eq218_e2717_d_b17: f64 = (p.p7 * (s.db[196][17] * ddt_scale));
        let eq218_e2717_d_b18: f64 = (p.p7 * (s.db[196][18] * ddt_scale));
        let eq218_e2717_d_b19: f64 = (p.p7 * (s.db[196][19] * ddt_scale));
        let eq218_e2717_d_b20: f64 = (p.p7 * (s.db[196][20] * ddt_scale));
        let eq218_e2717_d_b21: f64 = (p.p7 * (s.db[196][21] * ddt_scale));
        let eq218_e2717_d_b22: f64 = (p.p7 * (s.db[196][22] * ddt_scale));
        let eq218_e2717_d_b23: f64 = (p.p7 * (s.db[196][23] * ddt_scale));
        let eq218_e2717_d_b24: f64 = (p.p7 * (s.db[196][24] * ddt_scale));
        let eq218_e2717_d_b25: f64 = (p.p7 * (s.db[196][25] * ddt_scale));
        let eq218_e2717_d_b26: f64 = (p.p7 * (s.db[196][26] * ddt_scale));
        let eq218_e2717_d_b27: f64 = (p.p7 * (s.db[196][27] * ddt_scale));
        let eq218_e2717_d_b28: f64 = (p.p7 * (s.db[196][28] * ddt_scale));
        let eq218_e2717_d_b29: f64 = (p.p7 * (s.db[196][29] * ddt_scale));
        let eq218_e2717_d_b30: f64 = (p.p7 * (s.db[196][30] * ddt_scale));
        let eq218_e2717_d_b31: f64 = (p.p7 * (s.db[196][31] * ddt_scale));
        let eq218_e2717_d_b32: f64 = (p.p7 * (s.db[196][32] * ddt_scale));
        let eq218_e2717_d_b33: f64 = (p.p7 * (s.db[196][33] * ddt_scale));
        let eq218_e2717_d_b34: f64 = (p.p7 * (s.db[196][34] * ddt_scale));
        let eq218_e2717_d_b35: f64 = (p.p7 * (s.db[196][35] * ddt_scale));
        let eq218_e2717_d_b36: f64 = (p.p7 * (s.db[196][36] * ddt_scale));
        let eq218_e2717_d_b37: f64 = (p.p7 * (s.db[196][37] * ddt_scale));
        let eq218_e2717_d_b38: f64 = (p.p7 * (s.db[196][38] * ddt_scale));
        let eq218_e2717_d_b39: f64 = (p.p7 * (s.db[196][39] * ddt_scale));
        let eq218_e2717_d_b40: f64 = (p.p7 * (s.db[196][40] * ddt_scale));
        let eq218_e2717_d_b41: f64 = (p.p7 * (s.db[196][41] * ddt_scale));
        let eq218_e2717_d_b42: f64 = (p.p7 * (s.db[196][42] * ddt_scale));
        let eq218_e2717_d_b43: f64 = (p.p7 * (s.db[196][43] * ddt_scale));
        let eq218_e2717_d_b44: f64 = (p.p7 * (s.db[196][44] * ddt_scale));
        let eq218_e2717_d_b45: f64 = (p.p7 * (s.db[196][45] * ddt_scale));
        let eq218_e2717_d_b46: f64 = (p.p7 * (s.db[196][46] * ddt_scale));
        let eq218_e2717_d_b47: f64 = (p.p7 * (s.db[196][47] * ddt_scale));
        let eq218_e2717_d_b48: f64 = (p.p7 * (s.db[196][48] * ddt_scale));
        let eq218_e2717_d_b49: f64 = (p.p7 * (s.db[196][49] * ddt_scale));
        let eq218_e2717_d_b50: f64 = (p.p7 * (s.db[196][50] * ddt_scale));
        let eq218_e2717_d_b51: f64 = (p.p7 * (s.db[196][51] * ddt_scale));
        let eq218_e2717_d_b52: f64 = (p.p7 * (s.db[196][52] * ddt_scale));
        let eq218_e2717_d_b53: f64 = (p.p7 * (s.db[196][53] * ddt_scale));
        let eq218_e2717_d_b54: f64 = (p.p7 * (s.db[196][54] * ddt_scale));
        let eq218_value: f64 = eq218_e2717;
        let eq218_node_derivatives: [f64; 23] = [eq218_e2717_d_n0, eq218_e2717_d_n1, eq218_e2717_d_n2, eq218_e2717_d_n3, eq218_e2717_d_n4, eq218_e2717_d_n5, eq218_e2717_d_n6, eq218_e2717_d_n7, eq218_e2717_d_n8, eq218_e2717_d_n9, eq218_e2717_d_n10, eq218_e2717_d_n11, eq218_e2717_d_n12, eq218_e2717_d_n13, eq218_e2717_d_n14, eq218_e2717_d_n15, eq218_e2717_d_n16, eq218_e2717_d_n17, eq218_e2717_d_n18, eq218_e2717_d_n19, eq218_e2717_d_n20, eq218_e2717_d_n21, eq218_e2717_d_n22];
        let eq218_branch_derivatives: [f64; 55] = [eq218_e2717_d_b0, eq218_e2717_d_b1, eq218_e2717_d_b2, eq218_e2717_d_b3, eq218_e2717_d_b4, eq218_e2717_d_b5, eq218_e2717_d_b6, eq218_e2717_d_b7, eq218_e2717_d_b8, eq218_e2717_d_b9, eq218_e2717_d_b10, eq218_e2717_d_b11, eq218_e2717_d_b12, eq218_e2717_d_b13, eq218_e2717_d_b14, eq218_e2717_d_b15, eq218_e2717_d_b16, eq218_e2717_d_b17, eq218_e2717_d_b18, eq218_e2717_d_b19, eq218_e2717_d_b20, eq218_e2717_d_b21, eq218_e2717_d_b22, eq218_e2717_d_b23, eq218_e2717_d_b24, eq218_e2717_d_b25, eq218_e2717_d_b26, eq218_e2717_d_b27, eq218_e2717_d_b28, eq218_e2717_d_b29, eq218_e2717_d_b30, eq218_e2717_d_b31, eq218_e2717_d_b32, eq218_e2717_d_b33, eq218_e2717_d_b34, eq218_e2717_d_b35, eq218_e2717_d_b36, eq218_e2717_d_b37, eq218_e2717_d_b38, eq218_e2717_d_b39, eq218_e2717_d_b40, eq218_e2717_d_b41, eq218_e2717_d_b42, eq218_e2717_d_b43, eq218_e2717_d_b44, eq218_e2717_d_b45, eq218_e2717_d_b46, eq218_e2717_d_b47, eq218_e2717_d_b48, eq218_e2717_d_b49, eq218_e2717_d_b50, eq218_e2717_d_b51, eq218_e2717_d_b52, eq218_e2717_d_b53, eq218_e2717_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq218_value),
            &eq218_node_derivatives,
            &eq218_branch_derivatives,
            multiplicity,
        );
        let eq219_e2720: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 118, var_qfr3);
        let eq219_e2721: f64 = (p.p7 * eq219_e2720);
        let eq219_e2721_d_n0: f64 = (p.p7 * (var_qfr3_dn0 * ddt_scale));
        let eq219_e2721_d_n1: f64 = (p.p7 * (var_qfr3_dn1 * ddt_scale));
        let eq219_e2721_d_n2: f64 = (p.p7 * (var_qfr3_dn2 * ddt_scale));
        let eq219_e2721_d_n3: f64 = (p.p7 * (var_qfr3_dn3 * ddt_scale));
        let eq219_e2721_d_n4: f64 = (p.p7 * (var_qfr3_dn4 * ddt_scale));
        let eq219_e2721_d_n5: f64 = (p.p7 * (var_qfr3_dn5 * ddt_scale));
        let eq219_e2721_d_n6: f64 = (p.p7 * (var_qfr3_dn6 * ddt_scale));
        let eq219_e2721_d_n7: f64 = (p.p7 * (var_qfr3_dn7 * ddt_scale));
        let eq219_e2721_d_n8: f64 = (p.p7 * (var_qfr3_dn8 * ddt_scale));
        let eq219_e2721_d_n9: f64 = (p.p7 * (var_qfr3_dn9 * ddt_scale));
        let eq219_e2721_d_n10: f64 = (p.p7 * (var_qfr3_dn10 * ddt_scale));
        let eq219_e2721_d_n11: f64 = (p.p7 * (var_qfr3_dn11 * ddt_scale));
        let eq219_e2721_d_n12: f64 = (p.p7 * (var_qfr3_dn12 * ddt_scale));
        let eq219_e2721_d_n13: f64 = (p.p7 * (var_qfr3_dn13 * ddt_scale));
        let eq219_e2721_d_n14: f64 = (p.p7 * (var_qfr3_dn14 * ddt_scale));
        let eq219_e2721_d_n15: f64 = (p.p7 * (var_qfr3_dn15 * ddt_scale));
        let eq219_e2721_d_n16: f64 = (p.p7 * (var_qfr3_dn16 * ddt_scale));
        let eq219_e2721_d_n17: f64 = (p.p7 * (var_qfr3_dn17 * ddt_scale));
        let eq219_e2721_d_n18: f64 = (p.p7 * (var_qfr3_dn18 * ddt_scale));
        let eq219_e2721_d_n19: f64 = (p.p7 * (var_qfr3_dn19 * ddt_scale));
        let eq219_e2721_d_n20: f64 = (p.p7 * (var_qfr3_dn20 * ddt_scale));
        let eq219_e2721_d_n21: f64 = (p.p7 * (var_qfr3_dn21 * ddt_scale));
        let eq219_e2721_d_n22: f64 = (p.p7 * (var_qfr3_dn22 * ddt_scale));
        let eq219_e2721_d_b0: f64 = (p.p7 * (var_qfr3_db0 * ddt_scale));
        let eq219_e2721_d_b1: f64 = (p.p7 * (var_qfr3_db1 * ddt_scale));
        let eq219_e2721_d_b2: f64 = (p.p7 * (var_qfr3_db2 * ddt_scale));
        let eq219_e2721_d_b3: f64 = (p.p7 * (var_qfr3_db3 * ddt_scale));
        let eq219_e2721_d_b4: f64 = (p.p7 * (var_qfr3_db4 * ddt_scale));
        let eq219_e2721_d_b5: f64 = (p.p7 * (var_qfr3_db5 * ddt_scale));
        let eq219_e2721_d_b6: f64 = (p.p7 * (var_qfr3_db6 * ddt_scale));
        let eq219_e2721_d_b7: f64 = (p.p7 * (var_qfr3_db7 * ddt_scale));
        let eq219_e2721_d_b8: f64 = (p.p7 * (var_qfr3_db8 * ddt_scale));
        let eq219_e2721_d_b9: f64 = (p.p7 * (var_qfr3_db9 * ddt_scale));
        let eq219_e2721_d_b10: f64 = (p.p7 * (var_qfr3_db10 * ddt_scale));
        let eq219_e2721_d_b11: f64 = (p.p7 * (var_qfr3_db11 * ddt_scale));
        let eq219_e2721_d_b12: f64 = (p.p7 * (var_qfr3_db12 * ddt_scale));
        let eq219_e2721_d_b13: f64 = (p.p7 * (var_qfr3_db13 * ddt_scale));
        let eq219_e2721_d_b14: f64 = (p.p7 * (var_qfr3_db14 * ddt_scale));
        let eq219_e2721_d_b15: f64 = (p.p7 * (var_qfr3_db15 * ddt_scale));
        let eq219_e2721_d_b16: f64 = (p.p7 * (var_qfr3_db16 * ddt_scale));
        let eq219_e2721_d_b17: f64 = (p.p7 * (var_qfr3_db17 * ddt_scale));
        let eq219_e2721_d_b18: f64 = (p.p7 * (var_qfr3_db18 * ddt_scale));
        let eq219_e2721_d_b19: f64 = (p.p7 * (var_qfr3_db19 * ddt_scale));
        let eq219_e2721_d_b20: f64 = (p.p7 * (var_qfr3_db20 * ddt_scale));
        let eq219_e2721_d_b21: f64 = (p.p7 * (var_qfr3_db21 * ddt_scale));
        let eq219_e2721_d_b22: f64 = (p.p7 * (var_qfr3_db22 * ddt_scale));
        let eq219_e2721_d_b23: f64 = (p.p7 * (var_qfr3_db23 * ddt_scale));
        let eq219_e2721_d_b24: f64 = (p.p7 * (var_qfr3_db24 * ddt_scale));
        let eq219_e2721_d_b25: f64 = (p.p7 * (var_qfr3_db25 * ddt_scale));
        let eq219_e2721_d_b26: f64 = (p.p7 * (var_qfr3_db26 * ddt_scale));
        let eq219_e2721_d_b27: f64 = (p.p7 * (var_qfr3_db27 * ddt_scale));
        let eq219_e2721_d_b28: f64 = (p.p7 * (var_qfr3_db28 * ddt_scale));
        let eq219_e2721_d_b29: f64 = (p.p7 * (var_qfr3_db29 * ddt_scale));
        let eq219_e2721_d_b30: f64 = (p.p7 * (var_qfr3_db30 * ddt_scale));
        let eq219_e2721_d_b31: f64 = (p.p7 * (var_qfr3_db31 * ddt_scale));
        let eq219_e2721_d_b32: f64 = (p.p7 * (var_qfr3_db32 * ddt_scale));
        let eq219_e2721_d_b33: f64 = (p.p7 * (var_qfr3_db33 * ddt_scale));
        let eq219_e2721_d_b34: f64 = (p.p7 * (var_qfr3_db34 * ddt_scale));
        let eq219_e2721_d_b35: f64 = (p.p7 * (var_qfr3_db35 * ddt_scale));
        let eq219_e2721_d_b36: f64 = (p.p7 * (var_qfr3_db36 * ddt_scale));
        let eq219_e2721_d_b37: f64 = (p.p7 * (var_qfr3_db37 * ddt_scale));
        let eq219_e2721_d_b38: f64 = (p.p7 * (var_qfr3_db38 * ddt_scale));
        let eq219_e2721_d_b39: f64 = (p.p7 * (var_qfr3_db39 * ddt_scale));
        let eq219_e2721_d_b40: f64 = (p.p7 * (var_qfr3_db40 * ddt_scale));
        let eq219_e2721_d_b41: f64 = (p.p7 * (var_qfr3_db41 * ddt_scale));
        let eq219_e2721_d_b42: f64 = (p.p7 * (var_qfr3_db42 * ddt_scale));
        let eq219_e2721_d_b43: f64 = (p.p7 * (var_qfr3_db43 * ddt_scale));
        let eq219_e2721_d_b44: f64 = (p.p7 * (var_qfr3_db44 * ddt_scale));
        let eq219_e2721_d_b45: f64 = (p.p7 * (var_qfr3_db45 * ddt_scale));
        let eq219_e2721_d_b46: f64 = (p.p7 * (var_qfr3_db46 * ddt_scale));
        let eq219_e2721_d_b47: f64 = (p.p7 * (var_qfr3_db47 * ddt_scale));
        let eq219_e2721_d_b48: f64 = (p.p7 * (var_qfr3_db48 * ddt_scale));
        let eq219_e2721_d_b49: f64 = (p.p7 * (var_qfr3_db49 * ddt_scale));
        let eq219_e2721_d_b50: f64 = (p.p7 * (var_qfr3_db50 * ddt_scale));
        let eq219_e2721_d_b51: f64 = (p.p7 * (var_qfr3_db51 * ddt_scale));
        let eq219_e2721_d_b52: f64 = (p.p7 * (var_qfr3_db52 * ddt_scale));
        let eq219_e2721_d_b53: f64 = (p.p7 * (var_qfr3_db53 * ddt_scale));
        let eq219_e2721_d_b54: f64 = (p.p7 * (var_qfr3_db54 * ddt_scale));
        let eq219_value: f64 = eq219_e2721;
        let eq219_node_derivatives: [f64; 23] = [eq219_e2721_d_n0, eq219_e2721_d_n1, eq219_e2721_d_n2, eq219_e2721_d_n3, eq219_e2721_d_n4, eq219_e2721_d_n5, eq219_e2721_d_n6, eq219_e2721_d_n7, eq219_e2721_d_n8, eq219_e2721_d_n9, eq219_e2721_d_n10, eq219_e2721_d_n11, eq219_e2721_d_n12, eq219_e2721_d_n13, eq219_e2721_d_n14, eq219_e2721_d_n15, eq219_e2721_d_n16, eq219_e2721_d_n17, eq219_e2721_d_n18, eq219_e2721_d_n19, eq219_e2721_d_n20, eq219_e2721_d_n21, eq219_e2721_d_n22];
        let eq219_branch_derivatives: [f64; 55] = [eq219_e2721_d_b0, eq219_e2721_d_b1, eq219_e2721_d_b2, eq219_e2721_d_b3, eq219_e2721_d_b4, eq219_e2721_d_b5, eq219_e2721_d_b6, eq219_e2721_d_b7, eq219_e2721_d_b8, eq219_e2721_d_b9, eq219_e2721_d_b10, eq219_e2721_d_b11, eq219_e2721_d_b12, eq219_e2721_d_b13, eq219_e2721_d_b14, eq219_e2721_d_b15, eq219_e2721_d_b16, eq219_e2721_d_b17, eq219_e2721_d_b18, eq219_e2721_d_b19, eq219_e2721_d_b20, eq219_e2721_d_b21, eq219_e2721_d_b22, eq219_e2721_d_b23, eq219_e2721_d_b24, eq219_e2721_d_b25, eq219_e2721_d_b26, eq219_e2721_d_b27, eq219_e2721_d_b28, eq219_e2721_d_b29, eq219_e2721_d_b30, eq219_e2721_d_b31, eq219_e2721_d_b32, eq219_e2721_d_b33, eq219_e2721_d_b34, eq219_e2721_d_b35, eq219_e2721_d_b36, eq219_e2721_d_b37, eq219_e2721_d_b38, eq219_e2721_d_b39, eq219_e2721_d_b40, eq219_e2721_d_b41, eq219_e2721_d_b42, eq219_e2721_d_b43, eq219_e2721_d_b44, eq219_e2721_d_b45, eq219_e2721_d_b46, eq219_e2721_d_b47, eq219_e2721_d_b48, eq219_e2721_d_b49, eq219_e2721_d_b50, eq219_e2721_d_b51, eq219_e2721_d_b52, eq219_e2721_d_b53, eq219_e2721_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(0),
            multiplicity * (eq219_value),
            &eq219_node_derivatives,
            &eq219_branch_derivatives,
            multiplicity,
        );
        let eq220_e2724: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 119, s.v[194]);
        let eq220_e2725: f64 = (p.p7 * eq220_e2724);
        let eq220_e2725_d_n0: f64 = (p.p7 * (s.dn[194][0] * ddt_scale));
        let eq220_e2725_d_n1: f64 = (p.p7 * (s.dn[194][1] * ddt_scale));
        let eq220_e2725_d_n2: f64 = (p.p7 * (s.dn[194][2] * ddt_scale));
        let eq220_e2725_d_n3: f64 = (p.p7 * (s.dn[194][3] * ddt_scale));
        let eq220_e2725_d_n4: f64 = (p.p7 * (s.dn[194][4] * ddt_scale));
        let eq220_e2725_d_n5: f64 = (p.p7 * (s.dn[194][5] * ddt_scale));
        let eq220_e2725_d_n6: f64 = (p.p7 * (s.dn[194][6] * ddt_scale));
        let eq220_e2725_d_n7: f64 = (p.p7 * (s.dn[194][7] * ddt_scale));
        let eq220_e2725_d_n8: f64 = (p.p7 * (s.dn[194][8] * ddt_scale));
        let eq220_e2725_d_n9: f64 = (p.p7 * (s.dn[194][9] * ddt_scale));
        let eq220_e2725_d_n10: f64 = (p.p7 * (s.dn[194][10] * ddt_scale));
        let eq220_e2725_d_n11: f64 = (p.p7 * (s.dn[194][11] * ddt_scale));
        let eq220_e2725_d_n12: f64 = (p.p7 * (s.dn[194][12] * ddt_scale));
        let eq220_e2725_d_n13: f64 = (p.p7 * (s.dn[194][13] * ddt_scale));
        let eq220_e2725_d_n14: f64 = (p.p7 * (s.dn[194][14] * ddt_scale));
        let eq220_e2725_d_n15: f64 = (p.p7 * (s.dn[194][15] * ddt_scale));
        let eq220_e2725_d_n16: f64 = (p.p7 * (s.dn[194][16] * ddt_scale));
        let eq220_e2725_d_n17: f64 = (p.p7 * (s.dn[194][17] * ddt_scale));
        let eq220_e2725_d_n18: f64 = (p.p7 * (s.dn[194][18] * ddt_scale));
        let eq220_e2725_d_n19: f64 = (p.p7 * (s.dn[194][19] * ddt_scale));
        let eq220_e2725_d_n20: f64 = (p.p7 * (s.dn[194][20] * ddt_scale));
        let eq220_e2725_d_n21: f64 = (p.p7 * (s.dn[194][21] * ddt_scale));
        let eq220_e2725_d_n22: f64 = (p.p7 * (s.dn[194][22] * ddt_scale));
        let eq220_e2725_d_b0: f64 = (p.p7 * (s.db[194][0] * ddt_scale));
        let eq220_e2725_d_b1: f64 = (p.p7 * (s.db[194][1] * ddt_scale));
        let eq220_e2725_d_b2: f64 = (p.p7 * (s.db[194][2] * ddt_scale));
        let eq220_e2725_d_b3: f64 = (p.p7 * (s.db[194][3] * ddt_scale));
        let eq220_e2725_d_b4: f64 = (p.p7 * (s.db[194][4] * ddt_scale));
        let eq220_e2725_d_b5: f64 = (p.p7 * (s.db[194][5] * ddt_scale));
        let eq220_e2725_d_b6: f64 = (p.p7 * (s.db[194][6] * ddt_scale));
        let eq220_e2725_d_b7: f64 = (p.p7 * (s.db[194][7] * ddt_scale));
        let eq220_e2725_d_b8: f64 = (p.p7 * (s.db[194][8] * ddt_scale));
        let eq220_e2725_d_b9: f64 = (p.p7 * (s.db[194][9] * ddt_scale));
        let eq220_e2725_d_b10: f64 = (p.p7 * (s.db[194][10] * ddt_scale));
        let eq220_e2725_d_b11: f64 = (p.p7 * (s.db[194][11] * ddt_scale));
        let eq220_e2725_d_b12: f64 = (p.p7 * (s.db[194][12] * ddt_scale));
        let eq220_e2725_d_b13: f64 = (p.p7 * (s.db[194][13] * ddt_scale));
        let eq220_e2725_d_b14: f64 = (p.p7 * (s.db[194][14] * ddt_scale));
        let eq220_e2725_d_b15: f64 = (p.p7 * (s.db[194][15] * ddt_scale));
        let eq220_e2725_d_b16: f64 = (p.p7 * (s.db[194][16] * ddt_scale));
        let eq220_e2725_d_b17: f64 = (p.p7 * (s.db[194][17] * ddt_scale));
        let eq220_e2725_d_b18: f64 = (p.p7 * (s.db[194][18] * ddt_scale));
        let eq220_e2725_d_b19: f64 = (p.p7 * (s.db[194][19] * ddt_scale));
        let eq220_e2725_d_b20: f64 = (p.p7 * (s.db[194][20] * ddt_scale));
        let eq220_e2725_d_b21: f64 = (p.p7 * (s.db[194][21] * ddt_scale));
        let eq220_e2725_d_b22: f64 = (p.p7 * (s.db[194][22] * ddt_scale));
        let eq220_e2725_d_b23: f64 = (p.p7 * (s.db[194][23] * ddt_scale));
        let eq220_e2725_d_b24: f64 = (p.p7 * (s.db[194][24] * ddt_scale));
        let eq220_e2725_d_b25: f64 = (p.p7 * (s.db[194][25] * ddt_scale));
        let eq220_e2725_d_b26: f64 = (p.p7 * (s.db[194][26] * ddt_scale));
        let eq220_e2725_d_b27: f64 = (p.p7 * (s.db[194][27] * ddt_scale));
        let eq220_e2725_d_b28: f64 = (p.p7 * (s.db[194][28] * ddt_scale));
        let eq220_e2725_d_b29: f64 = (p.p7 * (s.db[194][29] * ddt_scale));
        let eq220_e2725_d_b30: f64 = (p.p7 * (s.db[194][30] * ddt_scale));
        let eq220_e2725_d_b31: f64 = (p.p7 * (s.db[194][31] * ddt_scale));
        let eq220_e2725_d_b32: f64 = (p.p7 * (s.db[194][32] * ddt_scale));
        let eq220_e2725_d_b33: f64 = (p.p7 * (s.db[194][33] * ddt_scale));
        let eq220_e2725_d_b34: f64 = (p.p7 * (s.db[194][34] * ddt_scale));
        let eq220_e2725_d_b35: f64 = (p.p7 * (s.db[194][35] * ddt_scale));
        let eq220_e2725_d_b36: f64 = (p.p7 * (s.db[194][36] * ddt_scale));
        let eq220_e2725_d_b37: f64 = (p.p7 * (s.db[194][37] * ddt_scale));
        let eq220_e2725_d_b38: f64 = (p.p7 * (s.db[194][38] * ddt_scale));
        let eq220_e2725_d_b39: f64 = (p.p7 * (s.db[194][39] * ddt_scale));
        let eq220_e2725_d_b40: f64 = (p.p7 * (s.db[194][40] * ddt_scale));
        let eq220_e2725_d_b41: f64 = (p.p7 * (s.db[194][41] * ddt_scale));
        let eq220_e2725_d_b42: f64 = (p.p7 * (s.db[194][42] * ddt_scale));
        let eq220_e2725_d_b43: f64 = (p.p7 * (s.db[194][43] * ddt_scale));
        let eq220_e2725_d_b44: f64 = (p.p7 * (s.db[194][44] * ddt_scale));
        let eq220_e2725_d_b45: f64 = (p.p7 * (s.db[194][45] * ddt_scale));
        let eq220_e2725_d_b46: f64 = (p.p7 * (s.db[194][46] * ddt_scale));
        let eq220_e2725_d_b47: f64 = (p.p7 * (s.db[194][47] * ddt_scale));
        let eq220_e2725_d_b48: f64 = (p.p7 * (s.db[194][48] * ddt_scale));
        let eq220_e2725_d_b49: f64 = (p.p7 * (s.db[194][49] * ddt_scale));
        let eq220_e2725_d_b50: f64 = (p.p7 * (s.db[194][50] * ddt_scale));
        let eq220_e2725_d_b51: f64 = (p.p7 * (s.db[194][51] * ddt_scale));
        let eq220_e2725_d_b52: f64 = (p.p7 * (s.db[194][52] * ddt_scale));
        let eq220_e2725_d_b53: f64 = (p.p7 * (s.db[194][53] * ddt_scale));
        let eq220_e2725_d_b54: f64 = (p.p7 * (s.db[194][54] * ddt_scale));
        let eq220_value: f64 = eq220_e2725;
        let eq220_node_derivatives: [f64; 23] = [eq220_e2725_d_n0, eq220_e2725_d_n1, eq220_e2725_d_n2, eq220_e2725_d_n3, eq220_e2725_d_n4, eq220_e2725_d_n5, eq220_e2725_d_n6, eq220_e2725_d_n7, eq220_e2725_d_n8, eq220_e2725_d_n9, eq220_e2725_d_n10, eq220_e2725_d_n11, eq220_e2725_d_n12, eq220_e2725_d_n13, eq220_e2725_d_n14, eq220_e2725_d_n15, eq220_e2725_d_n16, eq220_e2725_d_n17, eq220_e2725_d_n18, eq220_e2725_d_n19, eq220_e2725_d_n20, eq220_e2725_d_n21, eq220_e2725_d_n22];
        let eq220_branch_derivatives: [f64; 55] = [eq220_e2725_d_b0, eq220_e2725_d_b1, eq220_e2725_d_b2, eq220_e2725_d_b3, eq220_e2725_d_b4, eq220_e2725_d_b5, eq220_e2725_d_b6, eq220_e2725_d_b7, eq220_e2725_d_b8, eq220_e2725_d_b9, eq220_e2725_d_b10, eq220_e2725_d_b11, eq220_e2725_d_b12, eq220_e2725_d_b13, eq220_e2725_d_b14, eq220_e2725_d_b15, eq220_e2725_d_b16, eq220_e2725_d_b17, eq220_e2725_d_b18, eq220_e2725_d_b19, eq220_e2725_d_b20, eq220_e2725_d_b21, eq220_e2725_d_b22, eq220_e2725_d_b23, eq220_e2725_d_b24, eq220_e2725_d_b25, eq220_e2725_d_b26, eq220_e2725_d_b27, eq220_e2725_d_b28, eq220_e2725_d_b29, eq220_e2725_d_b30, eq220_e2725_d_b31, eq220_e2725_d_b32, eq220_e2725_d_b33, eq220_e2725_d_b34, eq220_e2725_d_b35, eq220_e2725_d_b36, eq220_e2725_d_b37, eq220_e2725_d_b38, eq220_e2725_d_b39, eq220_e2725_d_b40, eq220_e2725_d_b41, eq220_e2725_d_b42, eq220_e2725_d_b43, eq220_e2725_d_b44, eq220_e2725_d_b45, eq220_e2725_d_b46, eq220_e2725_d_b47, eq220_e2725_d_b48, eq220_e2725_d_b49, eq220_e2725_d_b50, eq220_e2725_d_b51, eq220_e2725_d_b52, eq220_e2725_d_b53, eq220_e2725_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(0),
            multiplicity * (eq220_value),
            &eq220_node_derivatives,
            &eq220_branch_derivatives,
            multiplicity,
        );
        let eq221_ad: A = {
    if (s.v[610] != 0.0) {
        A::add_scaled_value_products(A::add_scaled_products3(s.ad_value(94), s.ad_value(38), (-1.0), s.ad_value(233), s.ad_value(231), (-1.0), s.ad_value(257), s.ad_value(255), (-1.0)), 1.0, s.ad_value(281), s.ad_value(279), (-1.0), s.ad_value(305), s.ad_value(303), (-1.0))
    } else {
        A::constant(0.0)
    }
};
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * eq221_ad.value,
            &eq221_ad.dn,
            &eq221_ad.db,
            multiplicity,
        );
        let (eq222_e2764, eq222_e2764_d_n4,) = {
    if (var_guard576 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p32;
        let eq222_e2762: f64 = ((nv4 - 0.0) * __rspice_inv_cse_0);
        let eq222_e2762_d_n4: f64 = (1.0 * __rspice_inv_cse_0);
        (eq222_e2762, eq222_e2762_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq222_value: f64 = eq222_e2764;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq222_value),
            4,
            multiplicity * (eq222_e2764_d_n4),
        );
        let (eq223_e2771, eq223_e2771_d_n4,) = {
    if (var_guard576 != 0.0) {
        let eq223_e2768: f64 = ((nv4 - 0.0) * p.p33);
        let eq223_e2769: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 120, eq223_e2768);
        (eq223_e2769, (p.p33 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq223_value: f64 = eq223_e2771;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq223_value),
            4,
            multiplicity * (eq223_e2771_d_n4),
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq9_e355, eq9_e355_d_n5, eq9_e355_q,) = {
    if (s.b[388] && (!s.b[387])) {
        let eq9_e352_q: f64 = (nv5 - 0.0);
        let eq9_e353: f64 = (p.p97 * (nv5 - 0.0));
        let eq9_e353_q: f64 = (p.p97 * eq9_e352_q);
        (eq9_e353, p.p97, eq9_e353_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq9_e355_d_n5),
        );
        let (eq17_e427, eq17_e427_d_n5, eq17_e427_q,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq17_e424_q: f64 = (nv5 - 0.0);
        let eq17_e425: f64 = (p.p110 * (nv5 - 0.0));
        let eq17_e425_q: f64 = (p.p110 * eq17_e424_q);
        (eq17_e425, p.p110, eq17_e425_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq17_e427_d_n5),
        );
        let (eq20_e462, eq20_e462_d_n6, eq20_e462_q,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq20_e459_q: f64 = (nv6 - 0.0);
        let eq20_e460: f64 = (p.p111 * (nv6 - 0.0));
        let eq20_e460_q: f64 = (p.p111 * eq20_e459_q);
        (eq20_e460, p.p111, eq20_e460_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * (eq20_e462_d_n6),
        );
        let (eq27_e539, eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n10, eq27_e539_d_n11, eq27_e539_d_n12, eq27_e539_d_n13, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22, eq27_e539_d_b0, eq27_e539_d_b1, eq27_e539_d_b2, eq27_e539_d_b3, eq27_e539_d_b4, eq27_e539_d_b5, eq27_e539_d_b6, eq27_e539_d_b7, eq27_e539_d_b8, eq27_e539_d_b9, eq27_e539_d_b10, eq27_e539_d_b11, eq27_e539_d_b12, eq27_e539_d_b13, eq27_e539_d_b14, eq27_e539_d_b15, eq27_e539_d_b16, eq27_e539_d_b17, eq27_e539_d_b18, eq27_e539_d_b19, eq27_e539_d_b20, eq27_e539_d_b21, eq27_e539_d_b22, eq27_e539_d_b23, eq27_e539_d_b24, eq27_e539_d_b25, eq27_e539_d_b26, eq27_e539_d_b27, eq27_e539_d_b28, eq27_e539_d_b29, eq27_e539_d_b30, eq27_e539_d_b31, eq27_e539_d_b32, eq27_e539_d_b33, eq27_e539_d_b34, eq27_e539_d_b35, eq27_e539_d_b36, eq27_e539_d_b37, eq27_e539_d_b38, eq27_e539_d_b39, eq27_e539_d_b40, eq27_e539_d_b41, eq27_e539_d_b42, eq27_e539_d_b43, eq27_e539_d_b44, eq27_e539_d_b45, eq27_e539_d_b46, eq27_e539_d_b47, eq27_e539_d_b48, eq27_e539_d_b49, eq27_e539_d_b50, eq27_e539_d_b51, eq27_e539_d_b52, eq27_e539_d_b53, eq27_e539_d_b54, eq27_e539_q, eq27_e539_q_d_n0, eq27_e539_q_d_n1, eq27_e539_q_d_n2, eq27_e539_q_d_n3, eq27_e539_q_d_n4, eq27_e539_q_d_n5, eq27_e539_q_d_n6, eq27_e539_q_d_n7, eq27_e539_q_d_n8, eq27_e539_q_d_n9, eq27_e539_q_d_n10, eq27_e539_q_d_n11, eq27_e539_q_d_n12, eq27_e539_q_d_n13, eq27_e539_q_d_n14, eq27_e539_q_d_n15, eq27_e539_q_d_n16, eq27_e539_q_d_n17, eq27_e539_q_d_n18, eq27_e539_q_d_n19, eq27_e539_q_d_n20, eq27_e539_q_d_n21, eq27_e539_q_d_n22, eq27_e539_q_d_b0, eq27_e539_q_d_b1, eq27_e539_q_d_b2, eq27_e539_q_d_b3, eq27_e539_q_d_b4, eq27_e539_q_d_b5, eq27_e539_q_d_b6, eq27_e539_q_d_b7, eq27_e539_q_d_b8, eq27_e539_q_d_b9, eq27_e539_q_d_b10, eq27_e539_q_d_b11, eq27_e539_q_d_b12, eq27_e539_q_d_b13, eq27_e539_q_d_b14, eq27_e539_q_d_b15, eq27_e539_q_d_b16, eq27_e539_q_d_b17, eq27_e539_q_d_b18, eq27_e539_q_d_b19, eq27_e539_q_d_b20, eq27_e539_q_d_b21, eq27_e539_q_d_b22, eq27_e539_q_d_b23, eq27_e539_q_d_b24, eq27_e539_q_d_b25, eq27_e539_q_d_b26, eq27_e539_q_d_b27, eq27_e539_q_d_b28, eq27_e539_q_d_b29, eq27_e539_q_d_b30, eq27_e539_q_d_b31, eq27_e539_q_d_b32, eq27_e539_q_d_b33, eq27_e539_q_d_b34, eq27_e539_q_d_b35, eq27_e539_q_d_b36, eq27_e539_q_d_b37, eq27_e539_q_d_b38, eq27_e539_q_d_b39, eq27_e539_q_d_b40, eq27_e539_q_d_b41, eq27_e539_q_d_b42, eq27_e539_q_d_b43, eq27_e539_q_d_b44, eq27_e539_q_d_b45, eq27_e539_q_d_b46, eq27_e539_q_d_b47, eq27_e539_q_d_b48, eq27_e539_q_d_b49, eq27_e539_q_d_b50, eq27_e539_q_d_b51, eq27_e539_q_d_b52, eq27_e539_q_d_b53, eq27_e539_q_d_b54,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        let eq27_e536_q: f64 = (nv5 - 0.0);
        let eq27_e537: f64 = (s.v[149] * (nv5 - 0.0));
        let eq27_e537_d_n0: f64 = (s.dn[149][0] * (nv5 - 0.0));
        let eq27_e537_d_n1: f64 = (s.dn[149][1] * (nv5 - 0.0));
        let eq27_e537_d_n2: f64 = (s.dn[149][2] * (nv5 - 0.0));
        let eq27_e537_d_n3: f64 = (s.dn[149][3] * (nv5 - 0.0));
        let eq27_e537_d_n4: f64 = (s.dn[149][4] * (nv5 - 0.0));
        let eq27_e537_d_n5: f64 = ((s.dn[149][5] * (nv5 - 0.0)) + s.v[149]);
        let eq27_e537_d_n6: f64 = (s.dn[149][6] * (nv5 - 0.0));
        let eq27_e537_d_n7: f64 = (s.dn[149][7] * (nv5 - 0.0));
        let eq27_e537_d_n8: f64 = (s.dn[149][8] * (nv5 - 0.0));
        let eq27_e537_d_n9: f64 = (s.dn[149][9] * (nv5 - 0.0));
        let eq27_e537_d_n10: f64 = (s.dn[149][10] * (nv5 - 0.0));
        let eq27_e537_d_n11: f64 = (s.dn[149][11] * (nv5 - 0.0));
        let eq27_e537_d_n12: f64 = (s.dn[149][12] * (nv5 - 0.0));
        let eq27_e537_d_n13: f64 = (s.dn[149][13] * (nv5 - 0.0));
        let eq27_e537_d_n14: f64 = (s.dn[149][14] * (nv5 - 0.0));
        let eq27_e537_d_n15: f64 = (s.dn[149][15] * (nv5 - 0.0));
        let eq27_e537_d_n16: f64 = (s.dn[149][16] * (nv5 - 0.0));
        let eq27_e537_d_n17: f64 = (s.dn[149][17] * (nv5 - 0.0));
        let eq27_e537_d_n18: f64 = (s.dn[149][18] * (nv5 - 0.0));
        let eq27_e537_d_n19: f64 = (s.dn[149][19] * (nv5 - 0.0));
        let eq27_e537_d_n20: f64 = (s.dn[149][20] * (nv5 - 0.0));
        let eq27_e537_d_n21: f64 = (s.dn[149][21] * (nv5 - 0.0));
        let eq27_e537_d_n22: f64 = (s.dn[149][22] * (nv5 - 0.0));
        let eq27_e537_d_b0: f64 = (s.db[149][0] * (nv5 - 0.0));
        let eq27_e537_d_b1: f64 = (s.db[149][1] * (nv5 - 0.0));
        let eq27_e537_d_b2: f64 = (s.db[149][2] * (nv5 - 0.0));
        let eq27_e537_d_b3: f64 = (s.db[149][3] * (nv5 - 0.0));
        let eq27_e537_d_b4: f64 = (s.db[149][4] * (nv5 - 0.0));
        let eq27_e537_d_b5: f64 = (s.db[149][5] * (nv5 - 0.0));
        let eq27_e537_d_b6: f64 = (s.db[149][6] * (nv5 - 0.0));
        let eq27_e537_d_b7: f64 = (s.db[149][7] * (nv5 - 0.0));
        let eq27_e537_d_b8: f64 = (s.db[149][8] * (nv5 - 0.0));
        let eq27_e537_d_b9: f64 = (s.db[149][9] * (nv5 - 0.0));
        let eq27_e537_d_b10: f64 = (s.db[149][10] * (nv5 - 0.0));
        let eq27_e537_d_b11: f64 = (s.db[149][11] * (nv5 - 0.0));
        let eq27_e537_d_b12: f64 = (s.db[149][12] * (nv5 - 0.0));
        let eq27_e537_d_b13: f64 = (s.db[149][13] * (nv5 - 0.0));
        let eq27_e537_d_b14: f64 = (s.db[149][14] * (nv5 - 0.0));
        let eq27_e537_d_b15: f64 = (s.db[149][15] * (nv5 - 0.0));
        let eq27_e537_d_b16: f64 = (s.db[149][16] * (nv5 - 0.0));
        let eq27_e537_d_b17: f64 = (s.db[149][17] * (nv5 - 0.0));
        let eq27_e537_d_b18: f64 = (s.db[149][18] * (nv5 - 0.0));
        let eq27_e537_d_b19: f64 = (s.db[149][19] * (nv5 - 0.0));
        let eq27_e537_d_b20: f64 = (s.db[149][20] * (nv5 - 0.0));
        let eq27_e537_d_b21: f64 = (s.db[149][21] * (nv5 - 0.0));
        let eq27_e537_d_b22: f64 = (s.db[149][22] * (nv5 - 0.0));
        let eq27_e537_d_b23: f64 = (s.db[149][23] * (nv5 - 0.0));
        let eq27_e537_d_b24: f64 = (s.db[149][24] * (nv5 - 0.0));
        let eq27_e537_d_b25: f64 = (s.db[149][25] * (nv5 - 0.0));
        let eq27_e537_d_b26: f64 = (s.db[149][26] * (nv5 - 0.0));
        let eq27_e537_d_b27: f64 = (s.db[149][27] * (nv5 - 0.0));
        let eq27_e537_d_b28: f64 = (s.db[149][28] * (nv5 - 0.0));
        let eq27_e537_d_b29: f64 = (s.db[149][29] * (nv5 - 0.0));
        let eq27_e537_d_b30: f64 = (s.db[149][30] * (nv5 - 0.0));
        let eq27_e537_d_b31: f64 = (s.db[149][31] * (nv5 - 0.0));
        let eq27_e537_d_b32: f64 = (s.db[149][32] * (nv5 - 0.0));
        let eq27_e537_d_b33: f64 = (s.db[149][33] * (nv5 - 0.0));
        let eq27_e537_d_b34: f64 = (s.db[149][34] * (nv5 - 0.0));
        let eq27_e537_d_b35: f64 = (s.db[149][35] * (nv5 - 0.0));
        let eq27_e537_d_b36: f64 = (s.db[149][36] * (nv5 - 0.0));
        let eq27_e537_d_b37: f64 = (s.db[149][37] * (nv5 - 0.0));
        let eq27_e537_d_b38: f64 = (s.db[149][38] * (nv5 - 0.0));
        let eq27_e537_d_b39: f64 = (s.db[149][39] * (nv5 - 0.0));
        let eq27_e537_d_b40: f64 = (s.db[149][40] * (nv5 - 0.0));
        let eq27_e537_d_b41: f64 = (s.db[149][41] * (nv5 - 0.0));
        let eq27_e537_d_b42: f64 = (s.db[149][42] * (nv5 - 0.0));
        let eq27_e537_d_b43: f64 = (s.db[149][43] * (nv5 - 0.0));
        let eq27_e537_d_b44: f64 = (s.db[149][44] * (nv5 - 0.0));
        let eq27_e537_d_b45: f64 = (s.db[149][45] * (nv5 - 0.0));
        let eq27_e537_d_b46: f64 = (s.db[149][46] * (nv5 - 0.0));
        let eq27_e537_d_b47: f64 = (s.db[149][47] * (nv5 - 0.0));
        let eq27_e537_d_b48: f64 = (s.db[149][48] * (nv5 - 0.0));
        let eq27_e537_d_b49: f64 = (s.db[149][49] * (nv5 - 0.0));
        let eq27_e537_d_b50: f64 = (s.db[149][50] * (nv5 - 0.0));
        let eq27_e537_d_b51: f64 = (s.db[149][51] * (nv5 - 0.0));
        let eq27_e537_d_b52: f64 = (s.db[149][52] * (nv5 - 0.0));
        let eq27_e537_d_b53: f64 = (s.db[149][53] * (nv5 - 0.0));
        let eq27_e537_d_b54: f64 = (s.db[149][54] * (nv5 - 0.0));
        let eq27_e537_q: f64 = (s.v[149] * eq27_e536_q);
        let eq27_e537_q_d_n5: f64 = ((s.dn[149][5] * eq27_e536_q) + s.v[149]);
        (eq27_e537, eq27_e537_d_n0, eq27_e537_d_n1, eq27_e537_d_n2, eq27_e537_d_n3, eq27_e537_d_n4, eq27_e537_d_n5, eq27_e537_d_n6, eq27_e537_d_n7, eq27_e537_d_n8, eq27_e537_d_n9, eq27_e537_d_n10, eq27_e537_d_n11, eq27_e537_d_n12, eq27_e537_d_n13, eq27_e537_d_n14, eq27_e537_d_n15, eq27_e537_d_n16, eq27_e537_d_n17, eq27_e537_d_n18, eq27_e537_d_n19, eq27_e537_d_n20, eq27_e537_d_n21, eq27_e537_d_n22, eq27_e537_d_b0, eq27_e537_d_b1, eq27_e537_d_b2, eq27_e537_d_b3, eq27_e537_d_b4, eq27_e537_d_b5, eq27_e537_d_b6, eq27_e537_d_b7, eq27_e537_d_b8, eq27_e537_d_b9, eq27_e537_d_b10, eq27_e537_d_b11, eq27_e537_d_b12, eq27_e537_d_b13, eq27_e537_d_b14, eq27_e537_d_b15, eq27_e537_d_b16, eq27_e537_d_b17, eq27_e537_d_b18, eq27_e537_d_b19, eq27_e537_d_b20, eq27_e537_d_b21, eq27_e537_d_b22, eq27_e537_d_b23, eq27_e537_d_b24, eq27_e537_d_b25, eq27_e537_d_b26, eq27_e537_d_b27, eq27_e537_d_b28, eq27_e537_d_b29, eq27_e537_d_b30, eq27_e537_d_b31, eq27_e537_d_b32, eq27_e537_d_b33, eq27_e537_d_b34, eq27_e537_d_b35, eq27_e537_d_b36, eq27_e537_d_b37, eq27_e537_d_b38, eq27_e537_d_b39, eq27_e537_d_b40, eq27_e537_d_b41, eq27_e537_d_b42, eq27_e537_d_b43, eq27_e537_d_b44, eq27_e537_d_b45, eq27_e537_d_b46, eq27_e537_d_b47, eq27_e537_d_b48, eq27_e537_d_b49, eq27_e537_d_b50, eq27_e537_d_b51, eq27_e537_d_b52, eq27_e537_d_b53, eq27_e537_d_b54, eq27_e537_q, (s.dn[149][0] * eq27_e536_q), (s.dn[149][1] * eq27_e536_q), (s.dn[149][2] * eq27_e536_q), (s.dn[149][3] * eq27_e536_q), (s.dn[149][4] * eq27_e536_q), eq27_e537_q_d_n5, (s.dn[149][6] * eq27_e536_q), (s.dn[149][7] * eq27_e536_q), (s.dn[149][8] * eq27_e536_q), (s.dn[149][9] * eq27_e536_q), (s.dn[149][10] * eq27_e536_q), (s.dn[149][11] * eq27_e536_q), (s.dn[149][12] * eq27_e536_q), (s.dn[149][13] * eq27_e536_q), (s.dn[149][14] * eq27_e536_q), (s.dn[149][15] * eq27_e536_q), (s.dn[149][16] * eq27_e536_q), (s.dn[149][17] * eq27_e536_q), (s.dn[149][18] * eq27_e536_q), (s.dn[149][19] * eq27_e536_q), (s.dn[149][20] * eq27_e536_q), (s.dn[149][21] * eq27_e536_q), (s.dn[149][22] * eq27_e536_q), (s.db[149][0] * eq27_e536_q), (s.db[149][1] * eq27_e536_q), (s.db[149][2] * eq27_e536_q), (s.db[149][3] * eq27_e536_q), (s.db[149][4] * eq27_e536_q), (s.db[149][5] * eq27_e536_q), (s.db[149][6] * eq27_e536_q), (s.db[149][7] * eq27_e536_q), (s.db[149][8] * eq27_e536_q), (s.db[149][9] * eq27_e536_q), (s.db[149][10] * eq27_e536_q), (s.db[149][11] * eq27_e536_q), (s.db[149][12] * eq27_e536_q), (s.db[149][13] * eq27_e536_q), (s.db[149][14] * eq27_e536_q), (s.db[149][15] * eq27_e536_q), (s.db[149][16] * eq27_e536_q), (s.db[149][17] * eq27_e536_q), (s.db[149][18] * eq27_e536_q), (s.db[149][19] * eq27_e536_q), (s.db[149][20] * eq27_e536_q), (s.db[149][21] * eq27_e536_q), (s.db[149][22] * eq27_e536_q), (s.db[149][23] * eq27_e536_q), (s.db[149][24] * eq27_e536_q), (s.db[149][25] * eq27_e536_q), (s.db[149][26] * eq27_e536_q), (s.db[149][27] * eq27_e536_q), (s.db[149][28] * eq27_e536_q), (s.db[149][29] * eq27_e536_q), (s.db[149][30] * eq27_e536_q), (s.db[149][31] * eq27_e536_q), (s.db[149][32] * eq27_e536_q), (s.db[149][33] * eq27_e536_q), (s.db[149][34] * eq27_e536_q), (s.db[149][35] * eq27_e536_q), (s.db[149][36] * eq27_e536_q), (s.db[149][37] * eq27_e536_q), (s.db[149][38] * eq27_e536_q), (s.db[149][39] * eq27_e536_q), (s.db[149][40] * eq27_e536_q), (s.db[149][41] * eq27_e536_q), (s.db[149][42] * eq27_e536_q), (s.db[149][43] * eq27_e536_q), (s.db[149][44] * eq27_e536_q), (s.db[149][45] * eq27_e536_q), (s.db[149][46] * eq27_e536_q), (s.db[149][47] * eq27_e536_q), (s.db[149][48] * eq27_e536_q), (s.db[149][49] * eq27_e536_q), (s.db[149][50] * eq27_e536_q), (s.db[149][51] * eq27_e536_q), (s.db[149][52] * eq27_e536_q), (s.db[149][53] * eq27_e536_q), (s.db[149][54] * eq27_e536_q),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_reactive_node_derivatives: [f64; 23] = [eq27_e539_q_d_n0, eq27_e539_q_d_n1, eq27_e539_q_d_n2, eq27_e539_q_d_n3, eq27_e539_q_d_n4, eq27_e539_q_d_n5, eq27_e539_q_d_n6, eq27_e539_q_d_n7, eq27_e539_q_d_n8, eq27_e539_q_d_n9, eq27_e539_q_d_n10, eq27_e539_q_d_n11, eq27_e539_q_d_n12, eq27_e539_q_d_n13, eq27_e539_q_d_n14, eq27_e539_q_d_n15, eq27_e539_q_d_n16, eq27_e539_q_d_n17, eq27_e539_q_d_n18, eq27_e539_q_d_n19, eq27_e539_q_d_n20, eq27_e539_q_d_n21, eq27_e539_q_d_n22];
        let eq27_reactive_branch_derivatives: [f64; 55] = [eq27_e539_q_d_b0, eq27_e539_q_d_b1, eq27_e539_q_d_b2, eq27_e539_q_d_b3, eq27_e539_q_d_b4, eq27_e539_q_d_b5, eq27_e539_q_d_b6, eq27_e539_q_d_b7, eq27_e539_q_d_b8, eq27_e539_q_d_b9, eq27_e539_q_d_b10, eq27_e539_q_d_b11, eq27_e539_q_d_b12, eq27_e539_q_d_b13, eq27_e539_q_d_b14, eq27_e539_q_d_b15, eq27_e539_q_d_b16, eq27_e539_q_d_b17, eq27_e539_q_d_b18, eq27_e539_q_d_b19, eq27_e539_q_d_b20, eq27_e539_q_d_b21, eq27_e539_q_d_b22, eq27_e539_q_d_b23, eq27_e539_q_d_b24, eq27_e539_q_d_b25, eq27_e539_q_d_b26, eq27_e539_q_d_b27, eq27_e539_q_d_b28, eq27_e539_q_d_b29, eq27_e539_q_d_b30, eq27_e539_q_d_b31, eq27_e539_q_d_b32, eq27_e539_q_d_b33, eq27_e539_q_d_b34, eq27_e539_q_d_b35, eq27_e539_q_d_b36, eq27_e539_q_d_b37, eq27_e539_q_d_b38, eq27_e539_q_d_b39, eq27_e539_q_d_b40, eq27_e539_q_d_b41, eq27_e539_q_d_b42, eq27_e539_q_d_b43, eq27_e539_q_d_b44, eq27_e539_q_d_b45, eq27_e539_q_d_b46, eq27_e539_q_d_b47, eq27_e539_q_d_b48, eq27_e539_q_d_b49, eq27_e539_q_d_b50, eq27_e539_q_d_b51, eq27_e539_q_d_b52, eq27_e539_q_d_b53, eq27_e539_q_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq37_e668, eq37_e668_d_n12, eq37_e668_q, eq37_e668_q_d_n12,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        let eq37_e661_q: f64 = (nv12 - 0.0);
        let eq37_e662: f64 = (p.p97 * (nv12 - 0.0));
        let eq37_e662_q: f64 = (p.p97 * eq37_e661_q);
        let eq37_e665: f64 = (1e-12 * (nv12 - 0.0));
        let eq37_e666: f64 = (eq37_e662 + eq37_e665);
        let eq37_e666_d_n12: f64 = (p.p97 + 1e-12);
        let eq37_e666_q: f64 = eq37_e662_q;
        (eq37_e666, eq37_e666_d_n12, eq37_e666_q, p.p97,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (eq37_e668_q_d_n12),
        );
        let (eq40_e716, eq40_e716_d_n14, eq40_e716_q, eq40_e716_q_d_n14,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        let eq40_e709_q: f64 = (nv14 - 0.0);
        let eq40_e710: f64 = (p.p83 * (nv14 - 0.0));
        let eq40_e710_q: f64 = (p.p83 * eq40_e709_q);
        let eq40_e713: f64 = (1e-12 * (nv14 - 0.0));
        let eq40_e714: f64 = (eq40_e710 + eq40_e713);
        let eq40_e714_d_n14: f64 = (p.p83 + 1e-12);
        let eq40_e714_q: f64 = eq40_e710_q;
        (eq40_e714, eq40_e714_d_n14, eq40_e714_q, p.p83,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[14]),
            None,
            nodes[14],
            multiplicity * (eq40_e716_q_d_n14),
        );
        let (eq43_e784, eq43_e784_d_n5, eq43_e784_q,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq43_e781_q: f64 = (nv5 - 0.0);
        let eq43_e782: f64 = (p.p135 * (nv5 - 0.0));
        let eq43_e782_q: f64 = (p.p135 * eq43_e781_q);
        (eq43_e782, p.p135, eq43_e782_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq43_e784_d_n5),
        );
        let (eq46_e852, eq46_e852_d_n6, eq46_e852_q,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq46_e849_q: f64 = (nv6 - 0.0);
        let eq46_e850: f64 = (p.p144 * (nv6 - 0.0));
        let eq46_e850_q: f64 = (p.p144 * eq46_e849_q);
        (eq46_e850, p.p144, eq46_e850_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * (eq46_e852_d_n6),
        );
        let eq109_e1474_q: f64 = s.v[165];
        let eq109_e1475: f64 = (p.p7 * s.v[165]);
        let eq109_e1475_q: f64 = (p.p7 * eq109_e1474_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &s.dn[165],
            branches,
            &s.db[165],
            (multiplicity) * (p.p7),
        );
        let eq110_e1478_q: f64 = s.v[161];
        let eq110_e1479: f64 = (p.p7 * s.v[161]);
        let eq110_e1479_q: f64 = (p.p7 * eq110_e1478_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &s.dn[161],
            branches,
            &s.db[161],
            (multiplicity) * (p.p7),
        );
        let (eq111_e1486, eq111_e1486_d_n0, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n3, eq111_e1486_d_n4, eq111_e1486_d_n5, eq111_e1486_d_n6, eq111_e1486_d_n7, eq111_e1486_d_n8, eq111_e1486_d_n9, eq111_e1486_d_n10, eq111_e1486_d_n11, eq111_e1486_d_n12, eq111_e1486_d_n13, eq111_e1486_d_n14, eq111_e1486_d_n15, eq111_e1486_d_n16, eq111_e1486_d_n17, eq111_e1486_d_n18, eq111_e1486_d_n19, eq111_e1486_d_n20, eq111_e1486_d_n21, eq111_e1486_d_n22, eq111_e1486_d_b0, eq111_e1486_d_b1, eq111_e1486_d_b2, eq111_e1486_d_b3, eq111_e1486_d_b4, eq111_e1486_d_b5, eq111_e1486_d_b6, eq111_e1486_d_b7, eq111_e1486_d_b8, eq111_e1486_d_b9, eq111_e1486_d_b10, eq111_e1486_d_b11, eq111_e1486_d_b12, eq111_e1486_d_b13, eq111_e1486_d_b14, eq111_e1486_d_b15, eq111_e1486_d_b16, eq111_e1486_d_b17, eq111_e1486_d_b18, eq111_e1486_d_b19, eq111_e1486_d_b20, eq111_e1486_d_b21, eq111_e1486_d_b22, eq111_e1486_d_b23, eq111_e1486_d_b24, eq111_e1486_d_b25, eq111_e1486_d_b26, eq111_e1486_d_b27, eq111_e1486_d_b28, eq111_e1486_d_b29, eq111_e1486_d_b30, eq111_e1486_d_b31, eq111_e1486_d_b32, eq111_e1486_d_b33, eq111_e1486_d_b34, eq111_e1486_d_b35, eq111_e1486_d_b36, eq111_e1486_d_b37, eq111_e1486_d_b38, eq111_e1486_d_b39, eq111_e1486_d_b40, eq111_e1486_d_b41, eq111_e1486_d_b42, eq111_e1486_d_b43, eq111_e1486_d_b44, eq111_e1486_d_b45, eq111_e1486_d_b46, eq111_e1486_d_b47, eq111_e1486_d_b48, eq111_e1486_d_b49, eq111_e1486_d_b50, eq111_e1486_d_b51, eq111_e1486_d_b52, eq111_e1486_d_b53, eq111_e1486_d_b54, eq111_e1486_q,) = {
    if s.b[569] {
        let eq111_e1483_q: f64 = s.v[162];
        let eq111_e1484: f64 = (p.p7 * s.v[162]);
        let eq111_e1484_q: f64 = (p.p7 * eq111_e1483_q);
        (eq111_e1484, (p.p7 * s.dn[162][0]), (p.p7 * s.dn[162][1]), (p.p7 * s.dn[162][2]), (p.p7 * s.dn[162][3]), (p.p7 * s.dn[162][4]), (p.p7 * s.dn[162][5]), (p.p7 * s.dn[162][6]), (p.p7 * s.dn[162][7]), (p.p7 * s.dn[162][8]), (p.p7 * s.dn[162][9]), (p.p7 * s.dn[162][10]), (p.p7 * s.dn[162][11]), (p.p7 * s.dn[162][12]), (p.p7 * s.dn[162][13]), (p.p7 * s.dn[162][14]), (p.p7 * s.dn[162][15]), (p.p7 * s.dn[162][16]), (p.p7 * s.dn[162][17]), (p.p7 * s.dn[162][18]), (p.p7 * s.dn[162][19]), (p.p7 * s.dn[162][20]), (p.p7 * s.dn[162][21]), (p.p7 * s.dn[162][22]), (p.p7 * s.db[162][0]), (p.p7 * s.db[162][1]), (p.p7 * s.db[162][2]), (p.p7 * s.db[162][3]), (p.p7 * s.db[162][4]), (p.p7 * s.db[162][5]), (p.p7 * s.db[162][6]), (p.p7 * s.db[162][7]), (p.p7 * s.db[162][8]), (p.p7 * s.db[162][9]), (p.p7 * s.db[162][10]), (p.p7 * s.db[162][11]), (p.p7 * s.db[162][12]), (p.p7 * s.db[162][13]), (p.p7 * s.db[162][14]), (p.p7 * s.db[162][15]), (p.p7 * s.db[162][16]), (p.p7 * s.db[162][17]), (p.p7 * s.db[162][18]), (p.p7 * s.db[162][19]), (p.p7 * s.db[162][20]), (p.p7 * s.db[162][21]), (p.p7 * s.db[162][22]), (p.p7 * s.db[162][23]), (p.p7 * s.db[162][24]), (p.p7 * s.db[162][25]), (p.p7 * s.db[162][26]), (p.p7 * s.db[162][27]), (p.p7 * s.db[162][28]), (p.p7 * s.db[162][29]), (p.p7 * s.db[162][30]), (p.p7 * s.db[162][31]), (p.p7 * s.db[162][32]), (p.p7 * s.db[162][33]), (p.p7 * s.db[162][34]), (p.p7 * s.db[162][35]), (p.p7 * s.db[162][36]), (p.p7 * s.db[162][37]), (p.p7 * s.db[162][38]), (p.p7 * s.db[162][39]), (p.p7 * s.db[162][40]), (p.p7 * s.db[162][41]), (p.p7 * s.db[162][42]), (p.p7 * s.db[162][43]), (p.p7 * s.db[162][44]), (p.p7 * s.db[162][45]), (p.p7 * s.db[162][46]), (p.p7 * s.db[162][47]), (p.p7 * s.db[162][48]), (p.p7 * s.db[162][49]), (p.p7 * s.db[162][50]), (p.p7 * s.db[162][51]), (p.p7 * s.db[162][52]), (p.p7 * s.db[162][53]), (p.p7 * s.db[162][54]), eq111_e1484_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_reactive_node_derivatives: [f64; 23] = [eq111_e1486_d_n0, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n3, eq111_e1486_d_n4, eq111_e1486_d_n5, eq111_e1486_d_n6, eq111_e1486_d_n7, eq111_e1486_d_n8, eq111_e1486_d_n9, eq111_e1486_d_n10, eq111_e1486_d_n11, eq111_e1486_d_n12, eq111_e1486_d_n13, eq111_e1486_d_n14, eq111_e1486_d_n15, eq111_e1486_d_n16, eq111_e1486_d_n17, eq111_e1486_d_n18, eq111_e1486_d_n19, eq111_e1486_d_n20, eq111_e1486_d_n21, eq111_e1486_d_n22];
        let eq111_reactive_branch_derivatives: [f64; 55] = [eq111_e1486_d_b0, eq111_e1486_d_b1, eq111_e1486_d_b2, eq111_e1486_d_b3, eq111_e1486_d_b4, eq111_e1486_d_b5, eq111_e1486_d_b6, eq111_e1486_d_b7, eq111_e1486_d_b8, eq111_e1486_d_b9, eq111_e1486_d_b10, eq111_e1486_d_b11, eq111_e1486_d_b12, eq111_e1486_d_b13, eq111_e1486_d_b14, eq111_e1486_d_b15, eq111_e1486_d_b16, eq111_e1486_d_b17, eq111_e1486_d_b18, eq111_e1486_d_b19, eq111_e1486_d_b20, eq111_e1486_d_b21, eq111_e1486_d_b22, eq111_e1486_d_b23, eq111_e1486_d_b24, eq111_e1486_d_b25, eq111_e1486_d_b26, eq111_e1486_d_b27, eq111_e1486_d_b28, eq111_e1486_d_b29, eq111_e1486_d_b30, eq111_e1486_d_b31, eq111_e1486_d_b32, eq111_e1486_d_b33, eq111_e1486_d_b34, eq111_e1486_d_b35, eq111_e1486_d_b36, eq111_e1486_d_b37, eq111_e1486_d_b38, eq111_e1486_d_b39, eq111_e1486_d_b40, eq111_e1486_d_b41, eq111_e1486_d_b42, eq111_e1486_d_b43, eq111_e1486_d_b44, eq111_e1486_d_b45, eq111_e1486_d_b46, eq111_e1486_d_b47, eq111_e1486_d_b48, eq111_e1486_d_b49, eq111_e1486_d_b50, eq111_e1486_d_b51, eq111_e1486_d_b52, eq111_e1486_d_b53, eq111_e1486_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            nodes,
            &eq111_reactive_node_derivatives,
            branches,
            &eq111_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq112_e1493, eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n3, eq112_e1493_d_n4, eq112_e1493_d_n5, eq112_e1493_d_n6, eq112_e1493_d_n7, eq112_e1493_d_n8, eq112_e1493_d_n9, eq112_e1493_d_n10, eq112_e1493_d_n11, eq112_e1493_d_n12, eq112_e1493_d_n13, eq112_e1493_d_n14, eq112_e1493_d_n15, eq112_e1493_d_n16, eq112_e1493_d_n17, eq112_e1493_d_n18, eq112_e1493_d_n19, eq112_e1493_d_n20, eq112_e1493_d_n21, eq112_e1493_d_n22, eq112_e1493_d_b0, eq112_e1493_d_b1, eq112_e1493_d_b2, eq112_e1493_d_b3, eq112_e1493_d_b4, eq112_e1493_d_b5, eq112_e1493_d_b6, eq112_e1493_d_b7, eq112_e1493_d_b8, eq112_e1493_d_b9, eq112_e1493_d_b10, eq112_e1493_d_b11, eq112_e1493_d_b12, eq112_e1493_d_b13, eq112_e1493_d_b14, eq112_e1493_d_b15, eq112_e1493_d_b16, eq112_e1493_d_b17, eq112_e1493_d_b18, eq112_e1493_d_b19, eq112_e1493_d_b20, eq112_e1493_d_b21, eq112_e1493_d_b22, eq112_e1493_d_b23, eq112_e1493_d_b24, eq112_e1493_d_b25, eq112_e1493_d_b26, eq112_e1493_d_b27, eq112_e1493_d_b28, eq112_e1493_d_b29, eq112_e1493_d_b30, eq112_e1493_d_b31, eq112_e1493_d_b32, eq112_e1493_d_b33, eq112_e1493_d_b34, eq112_e1493_d_b35, eq112_e1493_d_b36, eq112_e1493_d_b37, eq112_e1493_d_b38, eq112_e1493_d_b39, eq112_e1493_d_b40, eq112_e1493_d_b41, eq112_e1493_d_b42, eq112_e1493_d_b43, eq112_e1493_d_b44, eq112_e1493_d_b45, eq112_e1493_d_b46, eq112_e1493_d_b47, eq112_e1493_d_b48, eq112_e1493_d_b49, eq112_e1493_d_b50, eq112_e1493_d_b51, eq112_e1493_d_b52, eq112_e1493_d_b53, eq112_e1493_d_b54, eq112_e1493_q,) = {
    if s.b[569] {
        let eq112_e1490_q: f64 = s.v[163];
        let eq112_e1491: f64 = (p.p7 * s.v[163]);
        let eq112_e1491_q: f64 = (p.p7 * eq112_e1490_q);
        (eq112_e1491, (p.p7 * s.dn[163][0]), (p.p7 * s.dn[163][1]), (p.p7 * s.dn[163][2]), (p.p7 * s.dn[163][3]), (p.p7 * s.dn[163][4]), (p.p7 * s.dn[163][5]), (p.p7 * s.dn[163][6]), (p.p7 * s.dn[163][7]), (p.p7 * s.dn[163][8]), (p.p7 * s.dn[163][9]), (p.p7 * s.dn[163][10]), (p.p7 * s.dn[163][11]), (p.p7 * s.dn[163][12]), (p.p7 * s.dn[163][13]), (p.p7 * s.dn[163][14]), (p.p7 * s.dn[163][15]), (p.p7 * s.dn[163][16]), (p.p7 * s.dn[163][17]), (p.p7 * s.dn[163][18]), (p.p7 * s.dn[163][19]), (p.p7 * s.dn[163][20]), (p.p7 * s.dn[163][21]), (p.p7 * s.dn[163][22]), (p.p7 * s.db[163][0]), (p.p7 * s.db[163][1]), (p.p7 * s.db[163][2]), (p.p7 * s.db[163][3]), (p.p7 * s.db[163][4]), (p.p7 * s.db[163][5]), (p.p7 * s.db[163][6]), (p.p7 * s.db[163][7]), (p.p7 * s.db[163][8]), (p.p7 * s.db[163][9]), (p.p7 * s.db[163][10]), (p.p7 * s.db[163][11]), (p.p7 * s.db[163][12]), (p.p7 * s.db[163][13]), (p.p7 * s.db[163][14]), (p.p7 * s.db[163][15]), (p.p7 * s.db[163][16]), (p.p7 * s.db[163][17]), (p.p7 * s.db[163][18]), (p.p7 * s.db[163][19]), (p.p7 * s.db[163][20]), (p.p7 * s.db[163][21]), (p.p7 * s.db[163][22]), (p.p7 * s.db[163][23]), (p.p7 * s.db[163][24]), (p.p7 * s.db[163][25]), (p.p7 * s.db[163][26]), (p.p7 * s.db[163][27]), (p.p7 * s.db[163][28]), (p.p7 * s.db[163][29]), (p.p7 * s.db[163][30]), (p.p7 * s.db[163][31]), (p.p7 * s.db[163][32]), (p.p7 * s.db[163][33]), (p.p7 * s.db[163][34]), (p.p7 * s.db[163][35]), (p.p7 * s.db[163][36]), (p.p7 * s.db[163][37]), (p.p7 * s.db[163][38]), (p.p7 * s.db[163][39]), (p.p7 * s.db[163][40]), (p.p7 * s.db[163][41]), (p.p7 * s.db[163][42]), (p.p7 * s.db[163][43]), (p.p7 * s.db[163][44]), (p.p7 * s.db[163][45]), (p.p7 * s.db[163][46]), (p.p7 * s.db[163][47]), (p.p7 * s.db[163][48]), (p.p7 * s.db[163][49]), (p.p7 * s.db[163][50]), (p.p7 * s.db[163][51]), (p.p7 * s.db[163][52]), (p.p7 * s.db[163][53]), (p.p7 * s.db[163][54]), eq112_e1491_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_reactive_node_derivatives: [f64; 23] = [eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n3, eq112_e1493_d_n4, eq112_e1493_d_n5, eq112_e1493_d_n6, eq112_e1493_d_n7, eq112_e1493_d_n8, eq112_e1493_d_n9, eq112_e1493_d_n10, eq112_e1493_d_n11, eq112_e1493_d_n12, eq112_e1493_d_n13, eq112_e1493_d_n14, eq112_e1493_d_n15, eq112_e1493_d_n16, eq112_e1493_d_n17, eq112_e1493_d_n18, eq112_e1493_d_n19, eq112_e1493_d_n20, eq112_e1493_d_n21, eq112_e1493_d_n22];
        let eq112_reactive_branch_derivatives: [f64; 55] = [eq112_e1493_d_b0, eq112_e1493_d_b1, eq112_e1493_d_b2, eq112_e1493_d_b3, eq112_e1493_d_b4, eq112_e1493_d_b5, eq112_e1493_d_b6, eq112_e1493_d_b7, eq112_e1493_d_b8, eq112_e1493_d_b9, eq112_e1493_d_b10, eq112_e1493_d_b11, eq112_e1493_d_b12, eq112_e1493_d_b13, eq112_e1493_d_b14, eq112_e1493_d_b15, eq112_e1493_d_b16, eq112_e1493_d_b17, eq112_e1493_d_b18, eq112_e1493_d_b19, eq112_e1493_d_b20, eq112_e1493_d_b21, eq112_e1493_d_b22, eq112_e1493_d_b23, eq112_e1493_d_b24, eq112_e1493_d_b25, eq112_e1493_d_b26, eq112_e1493_d_b27, eq112_e1493_d_b28, eq112_e1493_d_b29, eq112_e1493_d_b30, eq112_e1493_d_b31, eq112_e1493_d_b32, eq112_e1493_d_b33, eq112_e1493_d_b34, eq112_e1493_d_b35, eq112_e1493_d_b36, eq112_e1493_d_b37, eq112_e1493_d_b38, eq112_e1493_d_b39, eq112_e1493_d_b40, eq112_e1493_d_b41, eq112_e1493_d_b42, eq112_e1493_d_b43, eq112_e1493_d_b44, eq112_e1493_d_b45, eq112_e1493_d_b46, eq112_e1493_d_b47, eq112_e1493_d_b48, eq112_e1493_d_b49, eq112_e1493_d_b50, eq112_e1493_d_b51, eq112_e1493_d_b52, eq112_e1493_d_b53, eq112_e1493_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            nodes,
            &eq112_reactive_node_derivatives,
            branches,
            &eq112_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq113_e1501, eq113_e1501_d_n0, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n3, eq113_e1501_d_n4, eq113_e1501_d_n5, eq113_e1501_d_n6, eq113_e1501_d_n7, eq113_e1501_d_n8, eq113_e1501_d_n9, eq113_e1501_d_n10, eq113_e1501_d_n11, eq113_e1501_d_n12, eq113_e1501_d_n13, eq113_e1501_d_n14, eq113_e1501_d_n15, eq113_e1501_d_n16, eq113_e1501_d_n17, eq113_e1501_d_n18, eq113_e1501_d_n19, eq113_e1501_d_n20, eq113_e1501_d_n21, eq113_e1501_d_n22, eq113_e1501_d_b0, eq113_e1501_d_b1, eq113_e1501_d_b2, eq113_e1501_d_b3, eq113_e1501_d_b4, eq113_e1501_d_b5, eq113_e1501_d_b6, eq113_e1501_d_b7, eq113_e1501_d_b8, eq113_e1501_d_b9, eq113_e1501_d_b10, eq113_e1501_d_b11, eq113_e1501_d_b12, eq113_e1501_d_b13, eq113_e1501_d_b14, eq113_e1501_d_b15, eq113_e1501_d_b16, eq113_e1501_d_b17, eq113_e1501_d_b18, eq113_e1501_d_b19, eq113_e1501_d_b20, eq113_e1501_d_b21, eq113_e1501_d_b22, eq113_e1501_d_b23, eq113_e1501_d_b24, eq113_e1501_d_b25, eq113_e1501_d_b26, eq113_e1501_d_b27, eq113_e1501_d_b28, eq113_e1501_d_b29, eq113_e1501_d_b30, eq113_e1501_d_b31, eq113_e1501_d_b32, eq113_e1501_d_b33, eq113_e1501_d_b34, eq113_e1501_d_b35, eq113_e1501_d_b36, eq113_e1501_d_b37, eq113_e1501_d_b38, eq113_e1501_d_b39, eq113_e1501_d_b40, eq113_e1501_d_b41, eq113_e1501_d_b42, eq113_e1501_d_b43, eq113_e1501_d_b44, eq113_e1501_d_b45, eq113_e1501_d_b46, eq113_e1501_d_b47, eq113_e1501_d_b48, eq113_e1501_d_b49, eq113_e1501_d_b50, eq113_e1501_d_b51, eq113_e1501_d_b52, eq113_e1501_d_b53, eq113_e1501_d_b54, eq113_e1501_q,) = {
    if (!s.b[569]) {
        let eq113_e1498_q: f64 = s.v[162];
        let eq113_e1499: f64 = (p.p7 * s.v[162]);
        let eq113_e1499_q: f64 = (p.p7 * eq113_e1498_q);
        (eq113_e1499, (p.p7 * s.dn[162][0]), (p.p7 * s.dn[162][1]), (p.p7 * s.dn[162][2]), (p.p7 * s.dn[162][3]), (p.p7 * s.dn[162][4]), (p.p7 * s.dn[162][5]), (p.p7 * s.dn[162][6]), (p.p7 * s.dn[162][7]), (p.p7 * s.dn[162][8]), (p.p7 * s.dn[162][9]), (p.p7 * s.dn[162][10]), (p.p7 * s.dn[162][11]), (p.p7 * s.dn[162][12]), (p.p7 * s.dn[162][13]), (p.p7 * s.dn[162][14]), (p.p7 * s.dn[162][15]), (p.p7 * s.dn[162][16]), (p.p7 * s.dn[162][17]), (p.p7 * s.dn[162][18]), (p.p7 * s.dn[162][19]), (p.p7 * s.dn[162][20]), (p.p7 * s.dn[162][21]), (p.p7 * s.dn[162][22]), (p.p7 * s.db[162][0]), (p.p7 * s.db[162][1]), (p.p7 * s.db[162][2]), (p.p7 * s.db[162][3]), (p.p7 * s.db[162][4]), (p.p7 * s.db[162][5]), (p.p7 * s.db[162][6]), (p.p7 * s.db[162][7]), (p.p7 * s.db[162][8]), (p.p7 * s.db[162][9]), (p.p7 * s.db[162][10]), (p.p7 * s.db[162][11]), (p.p7 * s.db[162][12]), (p.p7 * s.db[162][13]), (p.p7 * s.db[162][14]), (p.p7 * s.db[162][15]), (p.p7 * s.db[162][16]), (p.p7 * s.db[162][17]), (p.p7 * s.db[162][18]), (p.p7 * s.db[162][19]), (p.p7 * s.db[162][20]), (p.p7 * s.db[162][21]), (p.p7 * s.db[162][22]), (p.p7 * s.db[162][23]), (p.p7 * s.db[162][24]), (p.p7 * s.db[162][25]), (p.p7 * s.db[162][26]), (p.p7 * s.db[162][27]), (p.p7 * s.db[162][28]), (p.p7 * s.db[162][29]), (p.p7 * s.db[162][30]), (p.p7 * s.db[162][31]), (p.p7 * s.db[162][32]), (p.p7 * s.db[162][33]), (p.p7 * s.db[162][34]), (p.p7 * s.db[162][35]), (p.p7 * s.db[162][36]), (p.p7 * s.db[162][37]), (p.p7 * s.db[162][38]), (p.p7 * s.db[162][39]), (p.p7 * s.db[162][40]), (p.p7 * s.db[162][41]), (p.p7 * s.db[162][42]), (p.p7 * s.db[162][43]), (p.p7 * s.db[162][44]), (p.p7 * s.db[162][45]), (p.p7 * s.db[162][46]), (p.p7 * s.db[162][47]), (p.p7 * s.db[162][48]), (p.p7 * s.db[162][49]), (p.p7 * s.db[162][50]), (p.p7 * s.db[162][51]), (p.p7 * s.db[162][52]), (p.p7 * s.db[162][53]), (p.p7 * s.db[162][54]), eq113_e1499_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_reactive_node_derivatives: [f64; 23] = [eq113_e1501_d_n0, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n3, eq113_e1501_d_n4, eq113_e1501_d_n5, eq113_e1501_d_n6, eq113_e1501_d_n7, eq113_e1501_d_n8, eq113_e1501_d_n9, eq113_e1501_d_n10, eq113_e1501_d_n11, eq113_e1501_d_n12, eq113_e1501_d_n13, eq113_e1501_d_n14, eq113_e1501_d_n15, eq113_e1501_d_n16, eq113_e1501_d_n17, eq113_e1501_d_n18, eq113_e1501_d_n19, eq113_e1501_d_n20, eq113_e1501_d_n21, eq113_e1501_d_n22];
        let eq113_reactive_branch_derivatives: [f64; 55] = [eq113_e1501_d_b0, eq113_e1501_d_b1, eq113_e1501_d_b2, eq113_e1501_d_b3, eq113_e1501_d_b4, eq113_e1501_d_b5, eq113_e1501_d_b6, eq113_e1501_d_b7, eq113_e1501_d_b8, eq113_e1501_d_b9, eq113_e1501_d_b10, eq113_e1501_d_b11, eq113_e1501_d_b12, eq113_e1501_d_b13, eq113_e1501_d_b14, eq113_e1501_d_b15, eq113_e1501_d_b16, eq113_e1501_d_b17, eq113_e1501_d_b18, eq113_e1501_d_b19, eq113_e1501_d_b20, eq113_e1501_d_b21, eq113_e1501_d_b22, eq113_e1501_d_b23, eq113_e1501_d_b24, eq113_e1501_d_b25, eq113_e1501_d_b26, eq113_e1501_d_b27, eq113_e1501_d_b28, eq113_e1501_d_b29, eq113_e1501_d_b30, eq113_e1501_d_b31, eq113_e1501_d_b32, eq113_e1501_d_b33, eq113_e1501_d_b34, eq113_e1501_d_b35, eq113_e1501_d_b36, eq113_e1501_d_b37, eq113_e1501_d_b38, eq113_e1501_d_b39, eq113_e1501_d_b40, eq113_e1501_d_b41, eq113_e1501_d_b42, eq113_e1501_d_b43, eq113_e1501_d_b44, eq113_e1501_d_b45, eq113_e1501_d_b46, eq113_e1501_d_b47, eq113_e1501_d_b48, eq113_e1501_d_b49, eq113_e1501_d_b50, eq113_e1501_d_b51, eq113_e1501_d_b52, eq113_e1501_d_b53, eq113_e1501_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq113_reactive_node_derivatives,
            branches,
            &eq113_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq114_e1509, eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n3, eq114_e1509_d_n4, eq114_e1509_d_n5, eq114_e1509_d_n6, eq114_e1509_d_n7, eq114_e1509_d_n8, eq114_e1509_d_n9, eq114_e1509_d_n10, eq114_e1509_d_n11, eq114_e1509_d_n12, eq114_e1509_d_n13, eq114_e1509_d_n14, eq114_e1509_d_n15, eq114_e1509_d_n16, eq114_e1509_d_n17, eq114_e1509_d_n18, eq114_e1509_d_n19, eq114_e1509_d_n20, eq114_e1509_d_n21, eq114_e1509_d_n22, eq114_e1509_d_b0, eq114_e1509_d_b1, eq114_e1509_d_b2, eq114_e1509_d_b3, eq114_e1509_d_b4, eq114_e1509_d_b5, eq114_e1509_d_b6, eq114_e1509_d_b7, eq114_e1509_d_b8, eq114_e1509_d_b9, eq114_e1509_d_b10, eq114_e1509_d_b11, eq114_e1509_d_b12, eq114_e1509_d_b13, eq114_e1509_d_b14, eq114_e1509_d_b15, eq114_e1509_d_b16, eq114_e1509_d_b17, eq114_e1509_d_b18, eq114_e1509_d_b19, eq114_e1509_d_b20, eq114_e1509_d_b21, eq114_e1509_d_b22, eq114_e1509_d_b23, eq114_e1509_d_b24, eq114_e1509_d_b25, eq114_e1509_d_b26, eq114_e1509_d_b27, eq114_e1509_d_b28, eq114_e1509_d_b29, eq114_e1509_d_b30, eq114_e1509_d_b31, eq114_e1509_d_b32, eq114_e1509_d_b33, eq114_e1509_d_b34, eq114_e1509_d_b35, eq114_e1509_d_b36, eq114_e1509_d_b37, eq114_e1509_d_b38, eq114_e1509_d_b39, eq114_e1509_d_b40, eq114_e1509_d_b41, eq114_e1509_d_b42, eq114_e1509_d_b43, eq114_e1509_d_b44, eq114_e1509_d_b45, eq114_e1509_d_b46, eq114_e1509_d_b47, eq114_e1509_d_b48, eq114_e1509_d_b49, eq114_e1509_d_b50, eq114_e1509_d_b51, eq114_e1509_d_b52, eq114_e1509_d_b53, eq114_e1509_d_b54, eq114_e1509_q,) = {
    if (!s.b[569]) {
        let eq114_e1506_q: f64 = s.v[163];
        let eq114_e1507: f64 = (p.p7 * s.v[163]);
        let eq114_e1507_q: f64 = (p.p7 * eq114_e1506_q);
        (eq114_e1507, (p.p7 * s.dn[163][0]), (p.p7 * s.dn[163][1]), (p.p7 * s.dn[163][2]), (p.p7 * s.dn[163][3]), (p.p7 * s.dn[163][4]), (p.p7 * s.dn[163][5]), (p.p7 * s.dn[163][6]), (p.p7 * s.dn[163][7]), (p.p7 * s.dn[163][8]), (p.p7 * s.dn[163][9]), (p.p7 * s.dn[163][10]), (p.p7 * s.dn[163][11]), (p.p7 * s.dn[163][12]), (p.p7 * s.dn[163][13]), (p.p7 * s.dn[163][14]), (p.p7 * s.dn[163][15]), (p.p7 * s.dn[163][16]), (p.p7 * s.dn[163][17]), (p.p7 * s.dn[163][18]), (p.p7 * s.dn[163][19]), (p.p7 * s.dn[163][20]), (p.p7 * s.dn[163][21]), (p.p7 * s.dn[163][22]), (p.p7 * s.db[163][0]), (p.p7 * s.db[163][1]), (p.p7 * s.db[163][2]), (p.p7 * s.db[163][3]), (p.p7 * s.db[163][4]), (p.p7 * s.db[163][5]), (p.p7 * s.db[163][6]), (p.p7 * s.db[163][7]), (p.p7 * s.db[163][8]), (p.p7 * s.db[163][9]), (p.p7 * s.db[163][10]), (p.p7 * s.db[163][11]), (p.p7 * s.db[163][12]), (p.p7 * s.db[163][13]), (p.p7 * s.db[163][14]), (p.p7 * s.db[163][15]), (p.p7 * s.db[163][16]), (p.p7 * s.db[163][17]), (p.p7 * s.db[163][18]), (p.p7 * s.db[163][19]), (p.p7 * s.db[163][20]), (p.p7 * s.db[163][21]), (p.p7 * s.db[163][22]), (p.p7 * s.db[163][23]), (p.p7 * s.db[163][24]), (p.p7 * s.db[163][25]), (p.p7 * s.db[163][26]), (p.p7 * s.db[163][27]), (p.p7 * s.db[163][28]), (p.p7 * s.db[163][29]), (p.p7 * s.db[163][30]), (p.p7 * s.db[163][31]), (p.p7 * s.db[163][32]), (p.p7 * s.db[163][33]), (p.p7 * s.db[163][34]), (p.p7 * s.db[163][35]), (p.p7 * s.db[163][36]), (p.p7 * s.db[163][37]), (p.p7 * s.db[163][38]), (p.p7 * s.db[163][39]), (p.p7 * s.db[163][40]), (p.p7 * s.db[163][41]), (p.p7 * s.db[163][42]), (p.p7 * s.db[163][43]), (p.p7 * s.db[163][44]), (p.p7 * s.db[163][45]), (p.p7 * s.db[163][46]), (p.p7 * s.db[163][47]), (p.p7 * s.db[163][48]), (p.p7 * s.db[163][49]), (p.p7 * s.db[163][50]), (p.p7 * s.db[163][51]), (p.p7 * s.db[163][52]), (p.p7 * s.db[163][53]), (p.p7 * s.db[163][54]), eq114_e1507_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq114_reactive_node_derivatives: [f64; 23] = [eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n3, eq114_e1509_d_n4, eq114_e1509_d_n5, eq114_e1509_d_n6, eq114_e1509_d_n7, eq114_e1509_d_n8, eq114_e1509_d_n9, eq114_e1509_d_n10, eq114_e1509_d_n11, eq114_e1509_d_n12, eq114_e1509_d_n13, eq114_e1509_d_n14, eq114_e1509_d_n15, eq114_e1509_d_n16, eq114_e1509_d_n17, eq114_e1509_d_n18, eq114_e1509_d_n19, eq114_e1509_d_n20, eq114_e1509_d_n21, eq114_e1509_d_n22];
        let eq114_reactive_branch_derivatives: [f64; 55] = [eq114_e1509_d_b0, eq114_e1509_d_b1, eq114_e1509_d_b2, eq114_e1509_d_b3, eq114_e1509_d_b4, eq114_e1509_d_b5, eq114_e1509_d_b6, eq114_e1509_d_b7, eq114_e1509_d_b8, eq114_e1509_d_b9, eq114_e1509_d_b10, eq114_e1509_d_b11, eq114_e1509_d_b12, eq114_e1509_d_b13, eq114_e1509_d_b14, eq114_e1509_d_b15, eq114_e1509_d_b16, eq114_e1509_d_b17, eq114_e1509_d_b18, eq114_e1509_d_b19, eq114_e1509_d_b20, eq114_e1509_d_b21, eq114_e1509_d_b22, eq114_e1509_d_b23, eq114_e1509_d_b24, eq114_e1509_d_b25, eq114_e1509_d_b26, eq114_e1509_d_b27, eq114_e1509_d_b28, eq114_e1509_d_b29, eq114_e1509_d_b30, eq114_e1509_d_b31, eq114_e1509_d_b32, eq114_e1509_d_b33, eq114_e1509_d_b34, eq114_e1509_d_b35, eq114_e1509_d_b36, eq114_e1509_d_b37, eq114_e1509_d_b38, eq114_e1509_d_b39, eq114_e1509_d_b40, eq114_e1509_d_b41, eq114_e1509_d_b42, eq114_e1509_d_b43, eq114_e1509_d_b44, eq114_e1509_d_b45, eq114_e1509_d_b46, eq114_e1509_d_b47, eq114_e1509_d_b48, eq114_e1509_d_b49, eq114_e1509_d_b50, eq114_e1509_d_b51, eq114_e1509_d_b52, eq114_e1509_d_b53, eq114_e1509_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq114_reactive_node_derivatives,
            branches,
            &eq114_reactive_branch_derivatives,
            multiplicity,
        );
        let eq115_e1512_q: f64 = s.v[164];
        let eq115_e1513: f64 = (p.p7 * s.v[164]);
        let eq115_e1513_q: f64 = (p.p7 * eq115_e1512_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &s.dn[164],
            branches,
            &s.db[164],
            (multiplicity) * (p.p7),
        );
        let eq116_e1516_q: f64 = s.v[219];
        let eq116_e1517: f64 = (p.p7 * s.v[219]);
        let eq116_e1517_q: f64 = (p.p7 * eq116_e1516_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            nodes,
            &s.dn[219],
            branches,
            &s.db[219],
            (multiplicity) * (p.p7),
        );
        let eq117_e1520_q: f64 = s.v[220];
        let eq117_e1521: f64 = (p.p7 * s.v[220]);
        let eq117_e1521_q: f64 = (p.p7 * eq117_e1520_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[2]),
            nodes,
            &s.dn[220],
            branches,
            &s.db[220],
            (multiplicity) * (p.p7),
        );
        let eq118_e1524_q: f64 = s.v[221];
        let eq118_e1525: f64 = (p.p7 * s.v[221]);
        let eq118_e1525_q: f64 = (p.p7 * eq118_e1524_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[1]),
            nodes,
            &s.dn[221],
            branches,
            &s.db[221],
            (multiplicity) * (p.p7),
        );
        let eq119_e1529: f64 = (p.p250 * s.v[161]);
        let eq119_e1530_q: f64 = eq119_e1529;
        let eq119_e1531: f64 = (p.p7 * eq119_e1529);
        let eq119_e1531_d_n0: f64 = (p.p7 * (p.p250 * s.dn[161][0]));
        let eq119_e1531_d_n1: f64 = (p.p7 * (p.p250 * s.dn[161][1]));
        let eq119_e1531_d_n2: f64 = (p.p7 * (p.p250 * s.dn[161][2]));
        let eq119_e1531_d_n3: f64 = (p.p7 * (p.p250 * s.dn[161][3]));
        let eq119_e1531_d_n4: f64 = (p.p7 * (p.p250 * s.dn[161][4]));
        let eq119_e1531_d_n5: f64 = (p.p7 * (p.p250 * s.dn[161][5]));
        let eq119_e1531_d_n6: f64 = (p.p7 * (p.p250 * s.dn[161][6]));
        let eq119_e1531_d_n7: f64 = (p.p7 * (p.p250 * s.dn[161][7]));
        let eq119_e1531_d_n8: f64 = (p.p7 * (p.p250 * s.dn[161][8]));
        let eq119_e1531_d_n9: f64 = (p.p7 * (p.p250 * s.dn[161][9]));
        let eq119_e1531_d_n10: f64 = (p.p7 * (p.p250 * s.dn[161][10]));
        let eq119_e1531_d_n11: f64 = (p.p7 * (p.p250 * s.dn[161][11]));
        let eq119_e1531_d_n12: f64 = (p.p7 * (p.p250 * s.dn[161][12]));
        let eq119_e1531_d_n13: f64 = (p.p7 * (p.p250 * s.dn[161][13]));
        let eq119_e1531_d_n14: f64 = (p.p7 * (p.p250 * s.dn[161][14]));
        let eq119_e1531_d_n15: f64 = (p.p7 * (p.p250 * s.dn[161][15]));
        let eq119_e1531_d_n16: f64 = (p.p7 * (p.p250 * s.dn[161][16]));
        let eq119_e1531_d_n17: f64 = (p.p7 * (p.p250 * s.dn[161][17]));
        let eq119_e1531_d_n18: f64 = (p.p7 * (p.p250 * s.dn[161][18]));
        let eq119_e1531_d_n19: f64 = (p.p7 * (p.p250 * s.dn[161][19]));
        let eq119_e1531_d_n20: f64 = (p.p7 * (p.p250 * s.dn[161][20]));
        let eq119_e1531_d_n21: f64 = (p.p7 * (p.p250 * s.dn[161][21]));
        let eq119_e1531_d_n22: f64 = (p.p7 * (p.p250 * s.dn[161][22]));
        let eq119_e1531_d_b0: f64 = (p.p7 * (p.p250 * s.db[161][0]));
        let eq119_e1531_d_b1: f64 = (p.p7 * (p.p250 * s.db[161][1]));
        let eq119_e1531_d_b2: f64 = (p.p7 * (p.p250 * s.db[161][2]));
        let eq119_e1531_d_b3: f64 = (p.p7 * (p.p250 * s.db[161][3]));
        let eq119_e1531_d_b4: f64 = (p.p7 * (p.p250 * s.db[161][4]));
        let eq119_e1531_d_b5: f64 = (p.p7 * (p.p250 * s.db[161][5]));
        let eq119_e1531_d_b6: f64 = (p.p7 * (p.p250 * s.db[161][6]));
        let eq119_e1531_d_b7: f64 = (p.p7 * (p.p250 * s.db[161][7]));
        let eq119_e1531_d_b8: f64 = (p.p7 * (p.p250 * s.db[161][8]));
        let eq119_e1531_d_b9: f64 = (p.p7 * (p.p250 * s.db[161][9]));
        let eq119_e1531_d_b10: f64 = (p.p7 * (p.p250 * s.db[161][10]));
        let eq119_e1531_d_b11: f64 = (p.p7 * (p.p250 * s.db[161][11]));
        let eq119_e1531_d_b12: f64 = (p.p7 * (p.p250 * s.db[161][12]));
        let eq119_e1531_d_b13: f64 = (p.p7 * (p.p250 * s.db[161][13]));
        let eq119_e1531_d_b14: f64 = (p.p7 * (p.p250 * s.db[161][14]));
        let eq119_e1531_d_b15: f64 = (p.p7 * (p.p250 * s.db[161][15]));
        let eq119_e1531_d_b16: f64 = (p.p7 * (p.p250 * s.db[161][16]));
        let eq119_e1531_d_b17: f64 = (p.p7 * (p.p250 * s.db[161][17]));
        let eq119_e1531_d_b18: f64 = (p.p7 * (p.p250 * s.db[161][18]));
        let eq119_e1531_d_b19: f64 = (p.p7 * (p.p250 * s.db[161][19]));
        let eq119_e1531_d_b20: f64 = (p.p7 * (p.p250 * s.db[161][20]));
        let eq119_e1531_d_b21: f64 = (p.p7 * (p.p250 * s.db[161][21]));
        let eq119_e1531_d_b22: f64 = (p.p7 * (p.p250 * s.db[161][22]));
        let eq119_e1531_d_b23: f64 = (p.p7 * (p.p250 * s.db[161][23]));
        let eq119_e1531_d_b24: f64 = (p.p7 * (p.p250 * s.db[161][24]));
        let eq119_e1531_d_b25: f64 = (p.p7 * (p.p250 * s.db[161][25]));
        let eq119_e1531_d_b26: f64 = (p.p7 * (p.p250 * s.db[161][26]));
        let eq119_e1531_d_b27: f64 = (p.p7 * (p.p250 * s.db[161][27]));
        let eq119_e1531_d_b28: f64 = (p.p7 * (p.p250 * s.db[161][28]));
        let eq119_e1531_d_b29: f64 = (p.p7 * (p.p250 * s.db[161][29]));
        let eq119_e1531_d_b30: f64 = (p.p7 * (p.p250 * s.db[161][30]));
        let eq119_e1531_d_b31: f64 = (p.p7 * (p.p250 * s.db[161][31]));
        let eq119_e1531_d_b32: f64 = (p.p7 * (p.p250 * s.db[161][32]));
        let eq119_e1531_d_b33: f64 = (p.p7 * (p.p250 * s.db[161][33]));
        let eq119_e1531_d_b34: f64 = (p.p7 * (p.p250 * s.db[161][34]));
        let eq119_e1531_d_b35: f64 = (p.p7 * (p.p250 * s.db[161][35]));
        let eq119_e1531_d_b36: f64 = (p.p7 * (p.p250 * s.db[161][36]));
        let eq119_e1531_d_b37: f64 = (p.p7 * (p.p250 * s.db[161][37]));
        let eq119_e1531_d_b38: f64 = (p.p7 * (p.p250 * s.db[161][38]));
        let eq119_e1531_d_b39: f64 = (p.p7 * (p.p250 * s.db[161][39]));
        let eq119_e1531_d_b40: f64 = (p.p7 * (p.p250 * s.db[161][40]));
        let eq119_e1531_d_b41: f64 = (p.p7 * (p.p250 * s.db[161][41]));
        let eq119_e1531_d_b42: f64 = (p.p7 * (p.p250 * s.db[161][42]));
        let eq119_e1531_d_b43: f64 = (p.p7 * (p.p250 * s.db[161][43]));
        let eq119_e1531_d_b44: f64 = (p.p7 * (p.p250 * s.db[161][44]));
        let eq119_e1531_d_b45: f64 = (p.p7 * (p.p250 * s.db[161][45]));
        let eq119_e1531_d_b46: f64 = (p.p7 * (p.p250 * s.db[161][46]));
        let eq119_e1531_d_b47: f64 = (p.p7 * (p.p250 * s.db[161][47]));
        let eq119_e1531_d_b48: f64 = (p.p7 * (p.p250 * s.db[161][48]));
        let eq119_e1531_d_b49: f64 = (p.p7 * (p.p250 * s.db[161][49]));
        let eq119_e1531_d_b50: f64 = (p.p7 * (p.p250 * s.db[161][50]));
        let eq119_e1531_d_b51: f64 = (p.p7 * (p.p250 * s.db[161][51]));
        let eq119_e1531_d_b52: f64 = (p.p7 * (p.p250 * s.db[161][52]));
        let eq119_e1531_d_b53: f64 = (p.p7 * (p.p250 * s.db[161][53]));
        let eq119_e1531_d_b54: f64 = (p.p7 * (p.p250 * s.db[161][54]));
        let eq119_e1531_q: f64 = (p.p7 * eq119_e1530_q);
        let eq119_reactive_node_derivatives: [f64; 23] = [eq119_e1531_d_n0, eq119_e1531_d_n1, eq119_e1531_d_n2, eq119_e1531_d_n3, eq119_e1531_d_n4, eq119_e1531_d_n5, eq119_e1531_d_n6, eq119_e1531_d_n7, eq119_e1531_d_n8, eq119_e1531_d_n9, eq119_e1531_d_n10, eq119_e1531_d_n11, eq119_e1531_d_n12, eq119_e1531_d_n13, eq119_e1531_d_n14, eq119_e1531_d_n15, eq119_e1531_d_n16, eq119_e1531_d_n17, eq119_e1531_d_n18, eq119_e1531_d_n19, eq119_e1531_d_n20, eq119_e1531_d_n21, eq119_e1531_d_n22];
        let eq119_reactive_branch_derivatives: [f64; 55] = [eq119_e1531_d_b0, eq119_e1531_d_b1, eq119_e1531_d_b2, eq119_e1531_d_b3, eq119_e1531_d_b4, eq119_e1531_d_b5, eq119_e1531_d_b6, eq119_e1531_d_b7, eq119_e1531_d_b8, eq119_e1531_d_b9, eq119_e1531_d_b10, eq119_e1531_d_b11, eq119_e1531_d_b12, eq119_e1531_d_b13, eq119_e1531_d_b14, eq119_e1531_d_b15, eq119_e1531_d_b16, eq119_e1531_d_b17, eq119_e1531_d_b18, eq119_e1531_d_b19, eq119_e1531_d_b20, eq119_e1531_d_b21, eq119_e1531_d_b22, eq119_e1531_d_b23, eq119_e1531_d_b24, eq119_e1531_d_b25, eq119_e1531_d_b26, eq119_e1531_d_b27, eq119_e1531_d_b28, eq119_e1531_d_b29, eq119_e1531_d_b30, eq119_e1531_d_b31, eq119_e1531_d_b32, eq119_e1531_d_b33, eq119_e1531_d_b34, eq119_e1531_d_b35, eq119_e1531_d_b36, eq119_e1531_d_b37, eq119_e1531_d_b38, eq119_e1531_d_b39, eq119_e1531_d_b40, eq119_e1531_d_b41, eq119_e1531_d_b42, eq119_e1531_d_b43, eq119_e1531_d_b44, eq119_e1531_d_b45, eq119_e1531_d_b46, eq119_e1531_d_b47, eq119_e1531_d_b48, eq119_e1531_d_b49, eq119_e1531_d_b50, eq119_e1531_d_b51, eq119_e1531_d_b52, eq119_e1531_d_b53, eq119_e1531_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq119_reactive_node_derivatives,
            branches,
            &eq119_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq120_e1540, eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22, eq120_e1540_d_b0, eq120_e1540_d_b1, eq120_e1540_d_b2, eq120_e1540_d_b3, eq120_e1540_d_b4, eq120_e1540_d_b5, eq120_e1540_d_b6, eq120_e1540_d_b7, eq120_e1540_d_b8, eq120_e1540_d_b9, eq120_e1540_d_b10, eq120_e1540_d_b11, eq120_e1540_d_b12, eq120_e1540_d_b13, eq120_e1540_d_b14, eq120_e1540_d_b15, eq120_e1540_d_b16, eq120_e1540_d_b17, eq120_e1540_d_b18, eq120_e1540_d_b19, eq120_e1540_d_b20, eq120_e1540_d_b21, eq120_e1540_d_b22, eq120_e1540_d_b23, eq120_e1540_d_b24, eq120_e1540_d_b25, eq120_e1540_d_b26, eq120_e1540_d_b27, eq120_e1540_d_b28, eq120_e1540_d_b29, eq120_e1540_d_b30, eq120_e1540_d_b31, eq120_e1540_d_b32, eq120_e1540_d_b33, eq120_e1540_d_b34, eq120_e1540_d_b35, eq120_e1540_d_b36, eq120_e1540_d_b37, eq120_e1540_d_b38, eq120_e1540_d_b39, eq120_e1540_d_b40, eq120_e1540_d_b41, eq120_e1540_d_b42, eq120_e1540_d_b43, eq120_e1540_d_b44, eq120_e1540_d_b45, eq120_e1540_d_b46, eq120_e1540_d_b47, eq120_e1540_d_b48, eq120_e1540_d_b49, eq120_e1540_d_b50, eq120_e1540_d_b51, eq120_e1540_d_b52, eq120_e1540_d_b53, eq120_e1540_d_b54, eq120_e1540_q,) = {
    if (s.b[570] && s.b[571]) {
        let eq120_e1537_q: f64 = s.v[229];
        let eq120_e1538: f64 = (p.p7 * s.v[229]);
        let eq120_e1538_q: f64 = (p.p7 * eq120_e1537_q);
        (eq120_e1538, (p.p7 * s.dn[229][0]), (p.p7 * s.dn[229][1]), (p.p7 * s.dn[229][2]), (p.p7 * s.dn[229][3]), (p.p7 * s.dn[229][4]), (p.p7 * s.dn[229][5]), (p.p7 * s.dn[229][6]), (p.p7 * s.dn[229][7]), (p.p7 * s.dn[229][8]), (p.p7 * s.dn[229][9]), (p.p7 * s.dn[229][10]), (p.p7 * s.dn[229][11]), (p.p7 * s.dn[229][12]), (p.p7 * s.dn[229][13]), (p.p7 * s.dn[229][14]), (p.p7 * s.dn[229][15]), (p.p7 * s.dn[229][16]), (p.p7 * s.dn[229][17]), (p.p7 * s.dn[229][18]), (p.p7 * s.dn[229][19]), (p.p7 * s.dn[229][20]), (p.p7 * s.dn[229][21]), (p.p7 * s.dn[229][22]), (p.p7 * s.db[229][0]), (p.p7 * s.db[229][1]), (p.p7 * s.db[229][2]), (p.p7 * s.db[229][3]), (p.p7 * s.db[229][4]), (p.p7 * s.db[229][5]), (p.p7 * s.db[229][6]), (p.p7 * s.db[229][7]), (p.p7 * s.db[229][8]), (p.p7 * s.db[229][9]), (p.p7 * s.db[229][10]), (p.p7 * s.db[229][11]), (p.p7 * s.db[229][12]), (p.p7 * s.db[229][13]), (p.p7 * s.db[229][14]), (p.p7 * s.db[229][15]), (p.p7 * s.db[229][16]), (p.p7 * s.db[229][17]), (p.p7 * s.db[229][18]), (p.p7 * s.db[229][19]), (p.p7 * s.db[229][20]), (p.p7 * s.db[229][21]), (p.p7 * s.db[229][22]), (p.p7 * s.db[229][23]), (p.p7 * s.db[229][24]), (p.p7 * s.db[229][25]), (p.p7 * s.db[229][26]), (p.p7 * s.db[229][27]), (p.p7 * s.db[229][28]), (p.p7 * s.db[229][29]), (p.p7 * s.db[229][30]), (p.p7 * s.db[229][31]), (p.p7 * s.db[229][32]), (p.p7 * s.db[229][33]), (p.p7 * s.db[229][34]), (p.p7 * s.db[229][35]), (p.p7 * s.db[229][36]), (p.p7 * s.db[229][37]), (p.p7 * s.db[229][38]), (p.p7 * s.db[229][39]), (p.p7 * s.db[229][40]), (p.p7 * s.db[229][41]), (p.p7 * s.db[229][42]), (p.p7 * s.db[229][43]), (p.p7 * s.db[229][44]), (p.p7 * s.db[229][45]), (p.p7 * s.db[229][46]), (p.p7 * s.db[229][47]), (p.p7 * s.db[229][48]), (p.p7 * s.db[229][49]), (p.p7 * s.db[229][50]), (p.p7 * s.db[229][51]), (p.p7 * s.db[229][52]), (p.p7 * s.db[229][53]), (p.p7 * s.db[229][54]), eq120_e1538_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq120_reactive_node_derivatives: [f64; 23] = [eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22];
        let eq120_reactive_branch_derivatives: [f64; 55] = [eq120_e1540_d_b0, eq120_e1540_d_b1, eq120_e1540_d_b2, eq120_e1540_d_b3, eq120_e1540_d_b4, eq120_e1540_d_b5, eq120_e1540_d_b6, eq120_e1540_d_b7, eq120_e1540_d_b8, eq120_e1540_d_b9, eq120_e1540_d_b10, eq120_e1540_d_b11, eq120_e1540_d_b12, eq120_e1540_d_b13, eq120_e1540_d_b14, eq120_e1540_d_b15, eq120_e1540_d_b16, eq120_e1540_d_b17, eq120_e1540_d_b18, eq120_e1540_d_b19, eq120_e1540_d_b20, eq120_e1540_d_b21, eq120_e1540_d_b22, eq120_e1540_d_b23, eq120_e1540_d_b24, eq120_e1540_d_b25, eq120_e1540_d_b26, eq120_e1540_d_b27, eq120_e1540_d_b28, eq120_e1540_d_b29, eq120_e1540_d_b30, eq120_e1540_d_b31, eq120_e1540_d_b32, eq120_e1540_d_b33, eq120_e1540_d_b34, eq120_e1540_d_b35, eq120_e1540_d_b36, eq120_e1540_d_b37, eq120_e1540_d_b38, eq120_e1540_d_b39, eq120_e1540_d_b40, eq120_e1540_d_b41, eq120_e1540_d_b42, eq120_e1540_d_b43, eq120_e1540_d_b44, eq120_e1540_d_b45, eq120_e1540_d_b46, eq120_e1540_d_b47, eq120_e1540_d_b48, eq120_e1540_d_b49, eq120_e1540_d_b50, eq120_e1540_d_b51, eq120_e1540_d_b52, eq120_e1540_d_b53, eq120_e1540_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            Some(nodes[7]),
            nodes,
            &eq120_reactive_node_derivatives,
            branches,
            &eq120_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq121_e1551, eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22, eq121_e1551_d_b0, eq121_e1551_d_b1, eq121_e1551_d_b2, eq121_e1551_d_b3, eq121_e1551_d_b4, eq121_e1551_d_b5, eq121_e1551_d_b6, eq121_e1551_d_b7, eq121_e1551_d_b8, eq121_e1551_d_b9, eq121_e1551_d_b10, eq121_e1551_d_b11, eq121_e1551_d_b12, eq121_e1551_d_b13, eq121_e1551_d_b14, eq121_e1551_d_b15, eq121_e1551_d_b16, eq121_e1551_d_b17, eq121_e1551_d_b18, eq121_e1551_d_b19, eq121_e1551_d_b20, eq121_e1551_d_b21, eq121_e1551_d_b22, eq121_e1551_d_b23, eq121_e1551_d_b24, eq121_e1551_d_b25, eq121_e1551_d_b26, eq121_e1551_d_b27, eq121_e1551_d_b28, eq121_e1551_d_b29, eq121_e1551_d_b30, eq121_e1551_d_b31, eq121_e1551_d_b32, eq121_e1551_d_b33, eq121_e1551_d_b34, eq121_e1551_d_b35, eq121_e1551_d_b36, eq121_e1551_d_b37, eq121_e1551_d_b38, eq121_e1551_d_b39, eq121_e1551_d_b40, eq121_e1551_d_b41, eq121_e1551_d_b42, eq121_e1551_d_b43, eq121_e1551_d_b44, eq121_e1551_d_b45, eq121_e1551_d_b46, eq121_e1551_d_b47, eq121_e1551_d_b48, eq121_e1551_d_b49, eq121_e1551_d_b50, eq121_e1551_d_b51, eq121_e1551_d_b52, eq121_e1551_d_b53, eq121_e1551_d_b54, eq121_e1551_q,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq121_e1548_q: f64 = s.v[228];
        let eq121_e1549: f64 = (p.p7 * s.v[228]);
        let eq121_e1549_q: f64 = (p.p7 * eq121_e1548_q);
        (eq121_e1549, (p.p7 * s.dn[228][0]), (p.p7 * s.dn[228][1]), (p.p7 * s.dn[228][2]), (p.p7 * s.dn[228][3]), (p.p7 * s.dn[228][4]), (p.p7 * s.dn[228][5]), (p.p7 * s.dn[228][6]), (p.p7 * s.dn[228][7]), (p.p7 * s.dn[228][8]), (p.p7 * s.dn[228][9]), (p.p7 * s.dn[228][10]), (p.p7 * s.dn[228][11]), (p.p7 * s.dn[228][12]), (p.p7 * s.dn[228][13]), (p.p7 * s.dn[228][14]), (p.p7 * s.dn[228][15]), (p.p7 * s.dn[228][16]), (p.p7 * s.dn[228][17]), (p.p7 * s.dn[228][18]), (p.p7 * s.dn[228][19]), (p.p7 * s.dn[228][20]), (p.p7 * s.dn[228][21]), (p.p7 * s.dn[228][22]), (p.p7 * s.db[228][0]), (p.p7 * s.db[228][1]), (p.p7 * s.db[228][2]), (p.p7 * s.db[228][3]), (p.p7 * s.db[228][4]), (p.p7 * s.db[228][5]), (p.p7 * s.db[228][6]), (p.p7 * s.db[228][7]), (p.p7 * s.db[228][8]), (p.p7 * s.db[228][9]), (p.p7 * s.db[228][10]), (p.p7 * s.db[228][11]), (p.p7 * s.db[228][12]), (p.p7 * s.db[228][13]), (p.p7 * s.db[228][14]), (p.p7 * s.db[228][15]), (p.p7 * s.db[228][16]), (p.p7 * s.db[228][17]), (p.p7 * s.db[228][18]), (p.p7 * s.db[228][19]), (p.p7 * s.db[228][20]), (p.p7 * s.db[228][21]), (p.p7 * s.db[228][22]), (p.p7 * s.db[228][23]), (p.p7 * s.db[228][24]), (p.p7 * s.db[228][25]), (p.p7 * s.db[228][26]), (p.p7 * s.db[228][27]), (p.p7 * s.db[228][28]), (p.p7 * s.db[228][29]), (p.p7 * s.db[228][30]), (p.p7 * s.db[228][31]), (p.p7 * s.db[228][32]), (p.p7 * s.db[228][33]), (p.p7 * s.db[228][34]), (p.p7 * s.db[228][35]), (p.p7 * s.db[228][36]), (p.p7 * s.db[228][37]), (p.p7 * s.db[228][38]), (p.p7 * s.db[228][39]), (p.p7 * s.db[228][40]), (p.p7 * s.db[228][41]), (p.p7 * s.db[228][42]), (p.p7 * s.db[228][43]), (p.p7 * s.db[228][44]), (p.p7 * s.db[228][45]), (p.p7 * s.db[228][46]), (p.p7 * s.db[228][47]), (p.p7 * s.db[228][48]), (p.p7 * s.db[228][49]), (p.p7 * s.db[228][50]), (p.p7 * s.db[228][51]), (p.p7 * s.db[228][52]), (p.p7 * s.db[228][53]), (p.p7 * s.db[228][54]), eq121_e1549_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq121_reactive_node_derivatives: [f64; 23] = [eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22];
        let eq121_reactive_branch_derivatives: [f64; 55] = [eq121_e1551_d_b0, eq121_e1551_d_b1, eq121_e1551_d_b2, eq121_e1551_d_b3, eq121_e1551_d_b4, eq121_e1551_d_b5, eq121_e1551_d_b6, eq121_e1551_d_b7, eq121_e1551_d_b8, eq121_e1551_d_b9, eq121_e1551_d_b10, eq121_e1551_d_b11, eq121_e1551_d_b12, eq121_e1551_d_b13, eq121_e1551_d_b14, eq121_e1551_d_b15, eq121_e1551_d_b16, eq121_e1551_d_b17, eq121_e1551_d_b18, eq121_e1551_d_b19, eq121_e1551_d_b20, eq121_e1551_d_b21, eq121_e1551_d_b22, eq121_e1551_d_b23, eq121_e1551_d_b24, eq121_e1551_d_b25, eq121_e1551_d_b26, eq121_e1551_d_b27, eq121_e1551_d_b28, eq121_e1551_d_b29, eq121_e1551_d_b30, eq121_e1551_d_b31, eq121_e1551_d_b32, eq121_e1551_d_b33, eq121_e1551_d_b34, eq121_e1551_d_b35, eq121_e1551_d_b36, eq121_e1551_d_b37, eq121_e1551_d_b38, eq121_e1551_d_b39, eq121_e1551_d_b40, eq121_e1551_d_b41, eq121_e1551_d_b42, eq121_e1551_d_b43, eq121_e1551_d_b44, eq121_e1551_d_b45, eq121_e1551_d_b46, eq121_e1551_d_b47, eq121_e1551_d_b48, eq121_e1551_d_b49, eq121_e1551_d_b50, eq121_e1551_d_b51, eq121_e1551_d_b52, eq121_e1551_d_b53, eq121_e1551_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq121_reactive_node_derivatives,
            branches,
            &eq121_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = ((p.p7 * s.dn[228][0]) * p.p246);
        let __rspice_deriv_cse_1: f64 = ((p.p7 * s.dn[228][1]) * p.p246);
        let __rspice_deriv_cse_2: f64 = ((p.p7 * s.dn[228][2]) * p.p246);
        let __rspice_deriv_cse_3: f64 = ((p.p7 * s.dn[228][3]) * p.p246);
        let __rspice_deriv_cse_4: f64 = ((p.p7 * s.dn[228][4]) * p.p246);
        let __rspice_deriv_cse_5: f64 = ((p.p7 * s.dn[228][5]) * p.p246);
        let __rspice_deriv_cse_6: f64 = ((p.p7 * s.dn[228][6]) * p.p246);
        let __rspice_deriv_cse_7: f64 = ((p.p7 * s.dn[228][7]) * p.p246);
        let __rspice_deriv_cse_8: f64 = ((p.p7 * s.dn[228][8]) * p.p246);
        let __rspice_deriv_cse_9: f64 = ((p.p7 * s.dn[228][9]) * p.p246);
        let __rspice_deriv_cse_10: f64 = ((p.p7 * s.dn[228][10]) * p.p246);
        let __rspice_deriv_cse_11: f64 = ((p.p7 * s.dn[228][11]) * p.p246);
        let __rspice_deriv_cse_12: f64 = ((p.p7 * s.dn[228][12]) * p.p246);
        let __rspice_deriv_cse_13: f64 = ((p.p7 * s.dn[228][13]) * p.p246);
        let __rspice_deriv_cse_14: f64 = ((p.p7 * s.dn[228][14]) * p.p246);
        let __rspice_deriv_cse_15: f64 = ((p.p7 * s.dn[228][15]) * p.p246);
        let __rspice_deriv_cse_16: f64 = ((p.p7 * s.dn[228][16]) * p.p246);
        let __rspice_deriv_cse_17: f64 = ((p.p7 * s.dn[228][17]) * p.p246);
        let __rspice_deriv_cse_18: f64 = ((p.p7 * s.dn[228][18]) * p.p246);
        let __rspice_deriv_cse_19: f64 = ((p.p7 * s.dn[228][19]) * p.p246);
        let __rspice_deriv_cse_20: f64 = ((p.p7 * s.dn[228][20]) * p.p246);
        let __rspice_deriv_cse_21: f64 = ((p.p7 * s.dn[228][21]) * p.p246);
        let __rspice_deriv_cse_22: f64 = ((p.p7 * s.dn[228][22]) * p.p246);
        let __rspice_deriv_cse_23: f64 = ((p.p7 * s.db[228][0]) * p.p246);
        let __rspice_deriv_cse_24: f64 = ((p.p7 * s.db[228][1]) * p.p246);
        let __rspice_deriv_cse_25: f64 = ((p.p7 * s.db[228][2]) * p.p246);
        let __rspice_deriv_cse_26: f64 = ((p.p7 * s.db[228][3]) * p.p246);
        let __rspice_deriv_cse_27: f64 = ((p.p7 * s.db[228][4]) * p.p246);
        let __rspice_deriv_cse_28: f64 = ((p.p7 * s.db[228][5]) * p.p246);
        let __rspice_deriv_cse_29: f64 = ((p.p7 * s.db[228][6]) * p.p246);
        let __rspice_deriv_cse_30: f64 = ((p.p7 * s.db[228][7]) * p.p246);
        let __rspice_deriv_cse_31: f64 = ((p.p7 * s.db[228][8]) * p.p246);
        let __rspice_deriv_cse_32: f64 = ((p.p7 * s.db[228][9]) * p.p246);
        let __rspice_deriv_cse_33: f64 = ((p.p7 * s.db[228][10]) * p.p246);
        let __rspice_deriv_cse_34: f64 = ((p.p7 * s.db[228][11]) * p.p246);
        let __rspice_deriv_cse_35: f64 = ((p.p7 * s.db[228][12]) * p.p246);
        let __rspice_deriv_cse_36: f64 = ((p.p7 * s.db[228][13]) * p.p246);
        let __rspice_deriv_cse_37: f64 = ((p.p7 * s.db[228][14]) * p.p246);
        let __rspice_deriv_cse_38: f64 = ((p.p7 * s.db[228][15]) * p.p246);
        let __rspice_deriv_cse_39: f64 = ((p.p7 * s.db[228][16]) * p.p246);
        let __rspice_deriv_cse_40: f64 = ((p.p7 * s.db[228][17]) * p.p246);
        let __rspice_deriv_cse_41: f64 = ((p.p7 * s.db[228][18]) * p.p246);
        let __rspice_deriv_cse_42: f64 = ((p.p7 * s.db[228][19]) * p.p246);
        let __rspice_deriv_cse_43: f64 = ((p.p7 * s.db[228][20]) * p.p246);
        let __rspice_deriv_cse_44: f64 = ((p.p7 * s.db[228][21]) * p.p246);
        let __rspice_deriv_cse_45: f64 = ((p.p7 * s.db[228][22]) * p.p246);
        let __rspice_deriv_cse_46: f64 = ((p.p7 * s.db[228][23]) * p.p246);
        let __rspice_deriv_cse_47: f64 = ((p.p7 * s.db[228][24]) * p.p246);
        let __rspice_deriv_cse_48: f64 = ((p.p7 * s.db[228][25]) * p.p246);
        let __rspice_deriv_cse_49: f64 = ((p.p7 * s.db[228][26]) * p.p246);
        let __rspice_deriv_cse_50: f64 = ((p.p7 * s.db[228][27]) * p.p246);
        let __rspice_deriv_cse_51: f64 = ((p.p7 * s.db[228][28]) * p.p246);
        let __rspice_deriv_cse_52: f64 = ((p.p7 * s.db[228][29]) * p.p246);
        let __rspice_deriv_cse_53: f64 = ((p.p7 * s.db[228][30]) * p.p246);
        let __rspice_deriv_cse_54: f64 = ((p.p7 * s.db[228][31]) * p.p246);
        let __rspice_deriv_cse_55: f64 = ((p.p7 * s.db[228][32]) * p.p246);
        let __rspice_deriv_cse_56: f64 = ((p.p7 * s.db[228][33]) * p.p246);
        let __rspice_deriv_cse_57: f64 = ((p.p7 * s.db[228][34]) * p.p246);
        let __rspice_deriv_cse_58: f64 = ((p.p7 * s.db[228][35]) * p.p246);
        let __rspice_deriv_cse_59: f64 = ((p.p7 * s.db[228][36]) * p.p246);
        let __rspice_deriv_cse_60: f64 = ((p.p7 * s.db[228][37]) * p.p246);
        let __rspice_deriv_cse_61: f64 = ((p.p7 * s.db[228][38]) * p.p246);
        let __rspice_deriv_cse_62: f64 = ((p.p7 * s.db[228][39]) * p.p246);
        let __rspice_deriv_cse_63: f64 = ((p.p7 * s.db[228][40]) * p.p246);
        let __rspice_deriv_cse_64: f64 = ((p.p7 * s.db[228][41]) * p.p246);
        let __rspice_deriv_cse_65: f64 = ((p.p7 * s.db[228][42]) * p.p246);
        let __rspice_deriv_cse_66: f64 = ((p.p7 * s.db[228][43]) * p.p246);
        let __rspice_deriv_cse_67: f64 = ((p.p7 * s.db[228][44]) * p.p246);
        let __rspice_deriv_cse_68: f64 = ((p.p7 * s.db[228][45]) * p.p246);
        let __rspice_deriv_cse_69: f64 = ((p.p7 * s.db[228][46]) * p.p246);
        let __rspice_deriv_cse_70: f64 = ((p.p7 * s.db[228][47]) * p.p246);
        let __rspice_deriv_cse_71: f64 = ((p.p7 * s.db[228][48]) * p.p246);
        let __rspice_deriv_cse_72: f64 = ((p.p7 * s.db[228][49]) * p.p246);
        let __rspice_deriv_cse_73: f64 = ((p.p7 * s.db[228][50]) * p.p246);
        let __rspice_deriv_cse_74: f64 = ((p.p7 * s.db[228][51]) * p.p246);
        let __rspice_deriv_cse_75: f64 = ((p.p7 * s.db[228][52]) * p.p246);
        let __rspice_deriv_cse_76: f64 = ((p.p7 * s.db[228][53]) * p.p246);
        let __rspice_deriv_cse_77: f64 = ((p.p7 * s.db[228][54]) * p.p246);
        let (eq122_e1564, eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22, eq122_e1564_d_b0, eq122_e1564_d_b1, eq122_e1564_d_b2, eq122_e1564_d_b3, eq122_e1564_d_b4, eq122_e1564_d_b5, eq122_e1564_d_b6, eq122_e1564_d_b7, eq122_e1564_d_b8, eq122_e1564_d_b9, eq122_e1564_d_b10, eq122_e1564_d_b11, eq122_e1564_d_b12, eq122_e1564_d_b13, eq122_e1564_d_b14, eq122_e1564_d_b15, eq122_e1564_d_b16, eq122_e1564_d_b17, eq122_e1564_d_b18, eq122_e1564_d_b19, eq122_e1564_d_b20, eq122_e1564_d_b21, eq122_e1564_d_b22, eq122_e1564_d_b23, eq122_e1564_d_b24, eq122_e1564_d_b25, eq122_e1564_d_b26, eq122_e1564_d_b27, eq122_e1564_d_b28, eq122_e1564_d_b29, eq122_e1564_d_b30, eq122_e1564_d_b31, eq122_e1564_d_b32, eq122_e1564_d_b33, eq122_e1564_d_b34, eq122_e1564_d_b35, eq122_e1564_d_b36, eq122_e1564_d_b37, eq122_e1564_d_b38, eq122_e1564_d_b39, eq122_e1564_d_b40, eq122_e1564_d_b41, eq122_e1564_d_b42, eq122_e1564_d_b43, eq122_e1564_d_b44, eq122_e1564_d_b45, eq122_e1564_d_b46, eq122_e1564_d_b47, eq122_e1564_d_b48, eq122_e1564_d_b49, eq122_e1564_d_b50, eq122_e1564_d_b51, eq122_e1564_d_b52, eq122_e1564_d_b53, eq122_e1564_d_b54, eq122_e1564_q,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq122_e1559_q: f64 = s.v[228];
        let eq122_e1560: f64 = (p.p7 * s.v[228]);
        let eq122_e1560_q: f64 = (p.p7 * eq122_e1559_q);
        let eq122_e1562: f64 = (eq122_e1560 * p.p246);
        let eq122_e1562_q: f64 = (eq122_e1560_q * p.p246);
        (eq122_e1562, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq122_e1562_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_reactive_node_derivatives: [f64; 23] = [eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22];
        let eq122_reactive_branch_derivatives: [f64; 55] = [eq122_e1564_d_b0, eq122_e1564_d_b1, eq122_e1564_d_b2, eq122_e1564_d_b3, eq122_e1564_d_b4, eq122_e1564_d_b5, eq122_e1564_d_b6, eq122_e1564_d_b7, eq122_e1564_d_b8, eq122_e1564_d_b9, eq122_e1564_d_b10, eq122_e1564_d_b11, eq122_e1564_d_b12, eq122_e1564_d_b13, eq122_e1564_d_b14, eq122_e1564_d_b15, eq122_e1564_d_b16, eq122_e1564_d_b17, eq122_e1564_d_b18, eq122_e1564_d_b19, eq122_e1564_d_b20, eq122_e1564_d_b21, eq122_e1564_d_b22, eq122_e1564_d_b23, eq122_e1564_d_b24, eq122_e1564_d_b25, eq122_e1564_d_b26, eq122_e1564_d_b27, eq122_e1564_d_b28, eq122_e1564_d_b29, eq122_e1564_d_b30, eq122_e1564_d_b31, eq122_e1564_d_b32, eq122_e1564_d_b33, eq122_e1564_d_b34, eq122_e1564_d_b35, eq122_e1564_d_b36, eq122_e1564_d_b37, eq122_e1564_d_b38, eq122_e1564_d_b39, eq122_e1564_d_b40, eq122_e1564_d_b41, eq122_e1564_d_b42, eq122_e1564_d_b43, eq122_e1564_d_b44, eq122_e1564_d_b45, eq122_e1564_d_b46, eq122_e1564_d_b47, eq122_e1564_d_b48, eq122_e1564_d_b49, eq122_e1564_d_b50, eq122_e1564_d_b51, eq122_e1564_d_b52, eq122_e1564_d_b53, eq122_e1564_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq122_reactive_node_derivatives,
            branches,
            &eq122_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq123_e1576, eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22, eq123_e1576_d_b0, eq123_e1576_d_b1, eq123_e1576_d_b2, eq123_e1576_d_b3, eq123_e1576_d_b4, eq123_e1576_d_b5, eq123_e1576_d_b6, eq123_e1576_d_b7, eq123_e1576_d_b8, eq123_e1576_d_b9, eq123_e1576_d_b10, eq123_e1576_d_b11, eq123_e1576_d_b12, eq123_e1576_d_b13, eq123_e1576_d_b14, eq123_e1576_d_b15, eq123_e1576_d_b16, eq123_e1576_d_b17, eq123_e1576_d_b18, eq123_e1576_d_b19, eq123_e1576_d_b20, eq123_e1576_d_b21, eq123_e1576_d_b22, eq123_e1576_d_b23, eq123_e1576_d_b24, eq123_e1576_d_b25, eq123_e1576_d_b26, eq123_e1576_d_b27, eq123_e1576_d_b28, eq123_e1576_d_b29, eq123_e1576_d_b30, eq123_e1576_d_b31, eq123_e1576_d_b32, eq123_e1576_d_b33, eq123_e1576_d_b34, eq123_e1576_d_b35, eq123_e1576_d_b36, eq123_e1576_d_b37, eq123_e1576_d_b38, eq123_e1576_d_b39, eq123_e1576_d_b40, eq123_e1576_d_b41, eq123_e1576_d_b42, eq123_e1576_d_b43, eq123_e1576_d_b44, eq123_e1576_d_b45, eq123_e1576_d_b46, eq123_e1576_d_b47, eq123_e1576_d_b48, eq123_e1576_d_b49, eq123_e1576_d_b50, eq123_e1576_d_b51, eq123_e1576_d_b52, eq123_e1576_d_b53, eq123_e1576_d_b54, eq123_e1576_q,) = {
    if ((s.b[570] && s.b[571]) && (!s.b[572])) {
        let eq123_e1573_q: f64 = s.v[228];
        let eq123_e1574: f64 = (p.p7 * s.v[228]);
        let eq123_e1574_q: f64 = (p.p7 * eq123_e1573_q);
        (eq123_e1574, (p.p7 * s.dn[228][0]), (p.p7 * s.dn[228][1]), (p.p7 * s.dn[228][2]), (p.p7 * s.dn[228][3]), (p.p7 * s.dn[228][4]), (p.p7 * s.dn[228][5]), (p.p7 * s.dn[228][6]), (p.p7 * s.dn[228][7]), (p.p7 * s.dn[228][8]), (p.p7 * s.dn[228][9]), (p.p7 * s.dn[228][10]), (p.p7 * s.dn[228][11]), (p.p7 * s.dn[228][12]), (p.p7 * s.dn[228][13]), (p.p7 * s.dn[228][14]), (p.p7 * s.dn[228][15]), (p.p7 * s.dn[228][16]), (p.p7 * s.dn[228][17]), (p.p7 * s.dn[228][18]), (p.p7 * s.dn[228][19]), (p.p7 * s.dn[228][20]), (p.p7 * s.dn[228][21]), (p.p7 * s.dn[228][22]), (p.p7 * s.db[228][0]), (p.p7 * s.db[228][1]), (p.p7 * s.db[228][2]), (p.p7 * s.db[228][3]), (p.p7 * s.db[228][4]), (p.p7 * s.db[228][5]), (p.p7 * s.db[228][6]), (p.p7 * s.db[228][7]), (p.p7 * s.db[228][8]), (p.p7 * s.db[228][9]), (p.p7 * s.db[228][10]), (p.p7 * s.db[228][11]), (p.p7 * s.db[228][12]), (p.p7 * s.db[228][13]), (p.p7 * s.db[228][14]), (p.p7 * s.db[228][15]), (p.p7 * s.db[228][16]), (p.p7 * s.db[228][17]), (p.p7 * s.db[228][18]), (p.p7 * s.db[228][19]), (p.p7 * s.db[228][20]), (p.p7 * s.db[228][21]), (p.p7 * s.db[228][22]), (p.p7 * s.db[228][23]), (p.p7 * s.db[228][24]), (p.p7 * s.db[228][25]), (p.p7 * s.db[228][26]), (p.p7 * s.db[228][27]), (p.p7 * s.db[228][28]), (p.p7 * s.db[228][29]), (p.p7 * s.db[228][30]), (p.p7 * s.db[228][31]), (p.p7 * s.db[228][32]), (p.p7 * s.db[228][33]), (p.p7 * s.db[228][34]), (p.p7 * s.db[228][35]), (p.p7 * s.db[228][36]), (p.p7 * s.db[228][37]), (p.p7 * s.db[228][38]), (p.p7 * s.db[228][39]), (p.p7 * s.db[228][40]), (p.p7 * s.db[228][41]), (p.p7 * s.db[228][42]), (p.p7 * s.db[228][43]), (p.p7 * s.db[228][44]), (p.p7 * s.db[228][45]), (p.p7 * s.db[228][46]), (p.p7 * s.db[228][47]), (p.p7 * s.db[228][48]), (p.p7 * s.db[228][49]), (p.p7 * s.db[228][50]), (p.p7 * s.db[228][51]), (p.p7 * s.db[228][52]), (p.p7 * s.db[228][53]), (p.p7 * s.db[228][54]), eq123_e1574_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq123_reactive_node_derivatives: [f64; 23] = [eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22];
        let eq123_reactive_branch_derivatives: [f64; 55] = [eq123_e1576_d_b0, eq123_e1576_d_b1, eq123_e1576_d_b2, eq123_e1576_d_b3, eq123_e1576_d_b4, eq123_e1576_d_b5, eq123_e1576_d_b6, eq123_e1576_d_b7, eq123_e1576_d_b8, eq123_e1576_d_b9, eq123_e1576_d_b10, eq123_e1576_d_b11, eq123_e1576_d_b12, eq123_e1576_d_b13, eq123_e1576_d_b14, eq123_e1576_d_b15, eq123_e1576_d_b16, eq123_e1576_d_b17, eq123_e1576_d_b18, eq123_e1576_d_b19, eq123_e1576_d_b20, eq123_e1576_d_b21, eq123_e1576_d_b22, eq123_e1576_d_b23, eq123_e1576_d_b24, eq123_e1576_d_b25, eq123_e1576_d_b26, eq123_e1576_d_b27, eq123_e1576_d_b28, eq123_e1576_d_b29, eq123_e1576_d_b30, eq123_e1576_d_b31, eq123_e1576_d_b32, eq123_e1576_d_b33, eq123_e1576_d_b34, eq123_e1576_d_b35, eq123_e1576_d_b36, eq123_e1576_d_b37, eq123_e1576_d_b38, eq123_e1576_d_b39, eq123_e1576_d_b40, eq123_e1576_d_b41, eq123_e1576_d_b42, eq123_e1576_d_b43, eq123_e1576_d_b44, eq123_e1576_d_b45, eq123_e1576_d_b46, eq123_e1576_d_b47, eq123_e1576_d_b48, eq123_e1576_d_b49, eq123_e1576_d_b50, eq123_e1576_d_b51, eq123_e1576_d_b52, eq123_e1576_d_b53, eq123_e1576_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq123_reactive_node_derivatives,
            branches,
            &eq123_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq124_e1590, eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22, eq124_e1590_d_b0, eq124_e1590_d_b1, eq124_e1590_d_b2, eq124_e1590_d_b3, eq124_e1590_d_b4, eq124_e1590_d_b5, eq124_e1590_d_b6, eq124_e1590_d_b7, eq124_e1590_d_b8, eq124_e1590_d_b9, eq124_e1590_d_b10, eq124_e1590_d_b11, eq124_e1590_d_b12, eq124_e1590_d_b13, eq124_e1590_d_b14, eq124_e1590_d_b15, eq124_e1590_d_b16, eq124_e1590_d_b17, eq124_e1590_d_b18, eq124_e1590_d_b19, eq124_e1590_d_b20, eq124_e1590_d_b21, eq124_e1590_d_b22, eq124_e1590_d_b23, eq124_e1590_d_b24, eq124_e1590_d_b25, eq124_e1590_d_b26, eq124_e1590_d_b27, eq124_e1590_d_b28, eq124_e1590_d_b29, eq124_e1590_d_b30, eq124_e1590_d_b31, eq124_e1590_d_b32, eq124_e1590_d_b33, eq124_e1590_d_b34, eq124_e1590_d_b35, eq124_e1590_d_b36, eq124_e1590_d_b37, eq124_e1590_d_b38, eq124_e1590_d_b39, eq124_e1590_d_b40, eq124_e1590_d_b41, eq124_e1590_d_b42, eq124_e1590_d_b43, eq124_e1590_d_b44, eq124_e1590_d_b45, eq124_e1590_d_b46, eq124_e1590_d_b47, eq124_e1590_d_b48, eq124_e1590_d_b49, eq124_e1590_d_b50, eq124_e1590_d_b51, eq124_e1590_d_b52, eq124_e1590_d_b53, eq124_e1590_d_b54, eq124_e1590_q,) = {
    if ((s.b[570] && s.b[571]) && (!s.b[572])) {
        let eq124_e1585_q: f64 = s.v[228];
        let eq124_e1586: f64 = (p.p7 * s.v[228]);
        let eq124_e1586_q: f64 = (p.p7 * eq124_e1585_q);
        let eq124_e1588: f64 = (eq124_e1586 * p.p246);
        let eq124_e1588_q: f64 = (eq124_e1586_q * p.p246);
        (eq124_e1588, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq124_e1588_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_reactive_node_derivatives: [f64; 23] = [eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22];
        let eq124_reactive_branch_derivatives: [f64; 55] = [eq124_e1590_d_b0, eq124_e1590_d_b1, eq124_e1590_d_b2, eq124_e1590_d_b3, eq124_e1590_d_b4, eq124_e1590_d_b5, eq124_e1590_d_b6, eq124_e1590_d_b7, eq124_e1590_d_b8, eq124_e1590_d_b9, eq124_e1590_d_b10, eq124_e1590_d_b11, eq124_e1590_d_b12, eq124_e1590_d_b13, eq124_e1590_d_b14, eq124_e1590_d_b15, eq124_e1590_d_b16, eq124_e1590_d_b17, eq124_e1590_d_b18, eq124_e1590_d_b19, eq124_e1590_d_b20, eq124_e1590_d_b21, eq124_e1590_d_b22, eq124_e1590_d_b23, eq124_e1590_d_b24, eq124_e1590_d_b25, eq124_e1590_d_b26, eq124_e1590_d_b27, eq124_e1590_d_b28, eq124_e1590_d_b29, eq124_e1590_d_b30, eq124_e1590_d_b31, eq124_e1590_d_b32, eq124_e1590_d_b33, eq124_e1590_d_b34, eq124_e1590_d_b35, eq124_e1590_d_b36, eq124_e1590_d_b37, eq124_e1590_d_b38, eq124_e1590_d_b39, eq124_e1590_d_b40, eq124_e1590_d_b41, eq124_e1590_d_b42, eq124_e1590_d_b43, eq124_e1590_d_b44, eq124_e1590_d_b45, eq124_e1590_d_b46, eq124_e1590_d_b47, eq124_e1590_d_b48, eq124_e1590_d_b49, eq124_e1590_d_b50, eq124_e1590_d_b51, eq124_e1590_d_b52, eq124_e1590_d_b53, eq124_e1590_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq124_reactive_node_derivatives,
            branches,
            &eq124_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1601, eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22, eq125_e1601_d_b0, eq125_e1601_d_b1, eq125_e1601_d_b2, eq125_e1601_d_b3, eq125_e1601_d_b4, eq125_e1601_d_b5, eq125_e1601_d_b6, eq125_e1601_d_b7, eq125_e1601_d_b8, eq125_e1601_d_b9, eq125_e1601_d_b10, eq125_e1601_d_b11, eq125_e1601_d_b12, eq125_e1601_d_b13, eq125_e1601_d_b14, eq125_e1601_d_b15, eq125_e1601_d_b16, eq125_e1601_d_b17, eq125_e1601_d_b18, eq125_e1601_d_b19, eq125_e1601_d_b20, eq125_e1601_d_b21, eq125_e1601_d_b22, eq125_e1601_d_b23, eq125_e1601_d_b24, eq125_e1601_d_b25, eq125_e1601_d_b26, eq125_e1601_d_b27, eq125_e1601_d_b28, eq125_e1601_d_b29, eq125_e1601_d_b30, eq125_e1601_d_b31, eq125_e1601_d_b32, eq125_e1601_d_b33, eq125_e1601_d_b34, eq125_e1601_d_b35, eq125_e1601_d_b36, eq125_e1601_d_b37, eq125_e1601_d_b38, eq125_e1601_d_b39, eq125_e1601_d_b40, eq125_e1601_d_b41, eq125_e1601_d_b42, eq125_e1601_d_b43, eq125_e1601_d_b44, eq125_e1601_d_b45, eq125_e1601_d_b46, eq125_e1601_d_b47, eq125_e1601_d_b48, eq125_e1601_d_b49, eq125_e1601_d_b50, eq125_e1601_d_b51, eq125_e1601_d_b52, eq125_e1601_d_b53, eq125_e1601_d_b54, eq125_e1601_q,) = {
    if (s.b[570] && s.b[571]) {
        let eq125_e1597: f64 = (p.p251 * s.v[228]);
        let eq125_e1598_q: f64 = eq125_e1597;
        let eq125_e1599: f64 = (p.p7 * eq125_e1597);
        let eq125_e1599_d_n0: f64 = (p.p7 * (p.p251 * s.dn[228][0]));
        let eq125_e1599_d_n1: f64 = (p.p7 * (p.p251 * s.dn[228][1]));
        let eq125_e1599_d_n2: f64 = (p.p7 * (p.p251 * s.dn[228][2]));
        let eq125_e1599_d_n3: f64 = (p.p7 * (p.p251 * s.dn[228][3]));
        let eq125_e1599_d_n4: f64 = (p.p7 * (p.p251 * s.dn[228][4]));
        let eq125_e1599_d_n5: f64 = (p.p7 * (p.p251 * s.dn[228][5]));
        let eq125_e1599_d_n6: f64 = (p.p7 * (p.p251 * s.dn[228][6]));
        let eq125_e1599_d_n7: f64 = (p.p7 * (p.p251 * s.dn[228][7]));
        let eq125_e1599_d_n8: f64 = (p.p7 * (p.p251 * s.dn[228][8]));
        let eq125_e1599_d_n9: f64 = (p.p7 * (p.p251 * s.dn[228][9]));
        let eq125_e1599_d_n10: f64 = (p.p7 * (p.p251 * s.dn[228][10]));
        let eq125_e1599_d_n11: f64 = (p.p7 * (p.p251 * s.dn[228][11]));
        let eq125_e1599_d_n12: f64 = (p.p7 * (p.p251 * s.dn[228][12]));
        let eq125_e1599_d_n13: f64 = (p.p7 * (p.p251 * s.dn[228][13]));
        let eq125_e1599_d_n14: f64 = (p.p7 * (p.p251 * s.dn[228][14]));
        let eq125_e1599_d_n15: f64 = (p.p7 * (p.p251 * s.dn[228][15]));
        let eq125_e1599_d_n16: f64 = (p.p7 * (p.p251 * s.dn[228][16]));
        let eq125_e1599_d_n17: f64 = (p.p7 * (p.p251 * s.dn[228][17]));
        let eq125_e1599_d_n18: f64 = (p.p7 * (p.p251 * s.dn[228][18]));
        let eq125_e1599_d_n19: f64 = (p.p7 * (p.p251 * s.dn[228][19]));
        let eq125_e1599_d_n20: f64 = (p.p7 * (p.p251 * s.dn[228][20]));
        let eq125_e1599_d_n21: f64 = (p.p7 * (p.p251 * s.dn[228][21]));
        let eq125_e1599_d_n22: f64 = (p.p7 * (p.p251 * s.dn[228][22]));
        let eq125_e1599_d_b0: f64 = (p.p7 * (p.p251 * s.db[228][0]));
        let eq125_e1599_d_b1: f64 = (p.p7 * (p.p251 * s.db[228][1]));
        let eq125_e1599_d_b2: f64 = (p.p7 * (p.p251 * s.db[228][2]));
        let eq125_e1599_d_b3: f64 = (p.p7 * (p.p251 * s.db[228][3]));
        let eq125_e1599_d_b4: f64 = (p.p7 * (p.p251 * s.db[228][4]));
        let eq125_e1599_d_b5: f64 = (p.p7 * (p.p251 * s.db[228][5]));
        let eq125_e1599_d_b6: f64 = (p.p7 * (p.p251 * s.db[228][6]));
        let eq125_e1599_d_b7: f64 = (p.p7 * (p.p251 * s.db[228][7]));
        let eq125_e1599_d_b8: f64 = (p.p7 * (p.p251 * s.db[228][8]));
        let eq125_e1599_d_b9: f64 = (p.p7 * (p.p251 * s.db[228][9]));
        let eq125_e1599_d_b10: f64 = (p.p7 * (p.p251 * s.db[228][10]));
        let eq125_e1599_d_b11: f64 = (p.p7 * (p.p251 * s.db[228][11]));
        let eq125_e1599_d_b12: f64 = (p.p7 * (p.p251 * s.db[228][12]));
        let eq125_e1599_d_b13: f64 = (p.p7 * (p.p251 * s.db[228][13]));
        let eq125_e1599_d_b14: f64 = (p.p7 * (p.p251 * s.db[228][14]));
        let eq125_e1599_d_b15: f64 = (p.p7 * (p.p251 * s.db[228][15]));
        let eq125_e1599_d_b16: f64 = (p.p7 * (p.p251 * s.db[228][16]));
        let eq125_e1599_d_b17: f64 = (p.p7 * (p.p251 * s.db[228][17]));
        let eq125_e1599_d_b18: f64 = (p.p7 * (p.p251 * s.db[228][18]));
        let eq125_e1599_d_b19: f64 = (p.p7 * (p.p251 * s.db[228][19]));
        let eq125_e1599_d_b20: f64 = (p.p7 * (p.p251 * s.db[228][20]));
        let eq125_e1599_d_b21: f64 = (p.p7 * (p.p251 * s.db[228][21]));
        let eq125_e1599_d_b22: f64 = (p.p7 * (p.p251 * s.db[228][22]));
        let eq125_e1599_d_b23: f64 = (p.p7 * (p.p251 * s.db[228][23]));
        let eq125_e1599_d_b24: f64 = (p.p7 * (p.p251 * s.db[228][24]));
        let eq125_e1599_d_b25: f64 = (p.p7 * (p.p251 * s.db[228][25]));
        let eq125_e1599_d_b26: f64 = (p.p7 * (p.p251 * s.db[228][26]));
        let eq125_e1599_d_b27: f64 = (p.p7 * (p.p251 * s.db[228][27]));
        let eq125_e1599_d_b28: f64 = (p.p7 * (p.p251 * s.db[228][28]));
        let eq125_e1599_d_b29: f64 = (p.p7 * (p.p251 * s.db[228][29]));
        let eq125_e1599_d_b30: f64 = (p.p7 * (p.p251 * s.db[228][30]));
        let eq125_e1599_d_b31: f64 = (p.p7 * (p.p251 * s.db[228][31]));
        let eq125_e1599_d_b32: f64 = (p.p7 * (p.p251 * s.db[228][32]));
        let eq125_e1599_d_b33: f64 = (p.p7 * (p.p251 * s.db[228][33]));
        let eq125_e1599_d_b34: f64 = (p.p7 * (p.p251 * s.db[228][34]));
        let eq125_e1599_d_b35: f64 = (p.p7 * (p.p251 * s.db[228][35]));
        let eq125_e1599_d_b36: f64 = (p.p7 * (p.p251 * s.db[228][36]));
        let eq125_e1599_d_b37: f64 = (p.p7 * (p.p251 * s.db[228][37]));
        let eq125_e1599_d_b38: f64 = (p.p7 * (p.p251 * s.db[228][38]));
        let eq125_e1599_d_b39: f64 = (p.p7 * (p.p251 * s.db[228][39]));
        let eq125_e1599_d_b40: f64 = (p.p7 * (p.p251 * s.db[228][40]));
        let eq125_e1599_d_b41: f64 = (p.p7 * (p.p251 * s.db[228][41]));
        let eq125_e1599_d_b42: f64 = (p.p7 * (p.p251 * s.db[228][42]));
        let eq125_e1599_d_b43: f64 = (p.p7 * (p.p251 * s.db[228][43]));
        let eq125_e1599_d_b44: f64 = (p.p7 * (p.p251 * s.db[228][44]));
        let eq125_e1599_d_b45: f64 = (p.p7 * (p.p251 * s.db[228][45]));
        let eq125_e1599_d_b46: f64 = (p.p7 * (p.p251 * s.db[228][46]));
        let eq125_e1599_d_b47: f64 = (p.p7 * (p.p251 * s.db[228][47]));
        let eq125_e1599_d_b48: f64 = (p.p7 * (p.p251 * s.db[228][48]));
        let eq125_e1599_d_b49: f64 = (p.p7 * (p.p251 * s.db[228][49]));
        let eq125_e1599_d_b50: f64 = (p.p7 * (p.p251 * s.db[228][50]));
        let eq125_e1599_d_b51: f64 = (p.p7 * (p.p251 * s.db[228][51]));
        let eq125_e1599_d_b52: f64 = (p.p7 * (p.p251 * s.db[228][52]));
        let eq125_e1599_d_b53: f64 = (p.p7 * (p.p251 * s.db[228][53]));
        let eq125_e1599_d_b54: f64 = (p.p7 * (p.p251 * s.db[228][54]));
        let eq125_e1599_q: f64 = (p.p7 * eq125_e1598_q);
        (eq125_e1599, eq125_e1599_d_n0, eq125_e1599_d_n1, eq125_e1599_d_n2, eq125_e1599_d_n3, eq125_e1599_d_n4, eq125_e1599_d_n5, eq125_e1599_d_n6, eq125_e1599_d_n7, eq125_e1599_d_n8, eq125_e1599_d_n9, eq125_e1599_d_n10, eq125_e1599_d_n11, eq125_e1599_d_n12, eq125_e1599_d_n13, eq125_e1599_d_n14, eq125_e1599_d_n15, eq125_e1599_d_n16, eq125_e1599_d_n17, eq125_e1599_d_n18, eq125_e1599_d_n19, eq125_e1599_d_n20, eq125_e1599_d_n21, eq125_e1599_d_n22, eq125_e1599_d_b0, eq125_e1599_d_b1, eq125_e1599_d_b2, eq125_e1599_d_b3, eq125_e1599_d_b4, eq125_e1599_d_b5, eq125_e1599_d_b6, eq125_e1599_d_b7, eq125_e1599_d_b8, eq125_e1599_d_b9, eq125_e1599_d_b10, eq125_e1599_d_b11, eq125_e1599_d_b12, eq125_e1599_d_b13, eq125_e1599_d_b14, eq125_e1599_d_b15, eq125_e1599_d_b16, eq125_e1599_d_b17, eq125_e1599_d_b18, eq125_e1599_d_b19, eq125_e1599_d_b20, eq125_e1599_d_b21, eq125_e1599_d_b22, eq125_e1599_d_b23, eq125_e1599_d_b24, eq125_e1599_d_b25, eq125_e1599_d_b26, eq125_e1599_d_b27, eq125_e1599_d_b28, eq125_e1599_d_b29, eq125_e1599_d_b30, eq125_e1599_d_b31, eq125_e1599_d_b32, eq125_e1599_d_b33, eq125_e1599_d_b34, eq125_e1599_d_b35, eq125_e1599_d_b36, eq125_e1599_d_b37, eq125_e1599_d_b38, eq125_e1599_d_b39, eq125_e1599_d_b40, eq125_e1599_d_b41, eq125_e1599_d_b42, eq125_e1599_d_b43, eq125_e1599_d_b44, eq125_e1599_d_b45, eq125_e1599_d_b46, eq125_e1599_d_b47, eq125_e1599_d_b48, eq125_e1599_d_b49, eq125_e1599_d_b50, eq125_e1599_d_b51, eq125_e1599_d_b52, eq125_e1599_d_b53, eq125_e1599_d_b54, eq125_e1599_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_reactive_node_derivatives: [f64; 23] = [eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22];
        let eq125_reactive_branch_derivatives: [f64; 55] = [eq125_e1601_d_b0, eq125_e1601_d_b1, eq125_e1601_d_b2, eq125_e1601_d_b3, eq125_e1601_d_b4, eq125_e1601_d_b5, eq125_e1601_d_b6, eq125_e1601_d_b7, eq125_e1601_d_b8, eq125_e1601_d_b9, eq125_e1601_d_b10, eq125_e1601_d_b11, eq125_e1601_d_b12, eq125_e1601_d_b13, eq125_e1601_d_b14, eq125_e1601_d_b15, eq125_e1601_d_b16, eq125_e1601_d_b17, eq125_e1601_d_b18, eq125_e1601_d_b19, eq125_e1601_d_b20, eq125_e1601_d_b21, eq125_e1601_d_b22, eq125_e1601_d_b23, eq125_e1601_d_b24, eq125_e1601_d_b25, eq125_e1601_d_b26, eq125_e1601_d_b27, eq125_e1601_d_b28, eq125_e1601_d_b29, eq125_e1601_d_b30, eq125_e1601_d_b31, eq125_e1601_d_b32, eq125_e1601_d_b33, eq125_e1601_d_b34, eq125_e1601_d_b35, eq125_e1601_d_b36, eq125_e1601_d_b37, eq125_e1601_d_b38, eq125_e1601_d_b39, eq125_e1601_d_b40, eq125_e1601_d_b41, eq125_e1601_d_b42, eq125_e1601_d_b43, eq125_e1601_d_b44, eq125_e1601_d_b45, eq125_e1601_d_b46, eq125_e1601_d_b47, eq125_e1601_d_b48, eq125_e1601_d_b49, eq125_e1601_d_b50, eq125_e1601_d_b51, eq125_e1601_d_b52, eq125_e1601_d_b53, eq125_e1601_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq125_reactive_node_derivatives,
            branches,
            &eq125_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1611, eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22, eq126_e1611_d_b0, eq126_e1611_d_b1, eq126_e1611_d_b2, eq126_e1611_d_b3, eq126_e1611_d_b4, eq126_e1611_d_b5, eq126_e1611_d_b6, eq126_e1611_d_b7, eq126_e1611_d_b8, eq126_e1611_d_b9, eq126_e1611_d_b10, eq126_e1611_d_b11, eq126_e1611_d_b12, eq126_e1611_d_b13, eq126_e1611_d_b14, eq126_e1611_d_b15, eq126_e1611_d_b16, eq126_e1611_d_b17, eq126_e1611_d_b18, eq126_e1611_d_b19, eq126_e1611_d_b20, eq126_e1611_d_b21, eq126_e1611_d_b22, eq126_e1611_d_b23, eq126_e1611_d_b24, eq126_e1611_d_b25, eq126_e1611_d_b26, eq126_e1611_d_b27, eq126_e1611_d_b28, eq126_e1611_d_b29, eq126_e1611_d_b30, eq126_e1611_d_b31, eq126_e1611_d_b32, eq126_e1611_d_b33, eq126_e1611_d_b34, eq126_e1611_d_b35, eq126_e1611_d_b36, eq126_e1611_d_b37, eq126_e1611_d_b38, eq126_e1611_d_b39, eq126_e1611_d_b40, eq126_e1611_d_b41, eq126_e1611_d_b42, eq126_e1611_d_b43, eq126_e1611_d_b44, eq126_e1611_d_b45, eq126_e1611_d_b46, eq126_e1611_d_b47, eq126_e1611_d_b48, eq126_e1611_d_b49, eq126_e1611_d_b50, eq126_e1611_d_b51, eq126_e1611_d_b52, eq126_e1611_d_b53, eq126_e1611_d_b54, eq126_e1611_q,) = {
    if ((!s.b[570]) && s.b[573]) {
        let eq126_e1608_q: f64 = s.v[229];
        let eq126_e1609: f64 = (p.p7 * s.v[229]);
        let eq126_e1609_q: f64 = (p.p7 * eq126_e1608_q);
        (eq126_e1609, (p.p7 * s.dn[229][0]), (p.p7 * s.dn[229][1]), (p.p7 * s.dn[229][2]), (p.p7 * s.dn[229][3]), (p.p7 * s.dn[229][4]), (p.p7 * s.dn[229][5]), (p.p7 * s.dn[229][6]), (p.p7 * s.dn[229][7]), (p.p7 * s.dn[229][8]), (p.p7 * s.dn[229][9]), (p.p7 * s.dn[229][10]), (p.p7 * s.dn[229][11]), (p.p7 * s.dn[229][12]), (p.p7 * s.dn[229][13]), (p.p7 * s.dn[229][14]), (p.p7 * s.dn[229][15]), (p.p7 * s.dn[229][16]), (p.p7 * s.dn[229][17]), (p.p7 * s.dn[229][18]), (p.p7 * s.dn[229][19]), (p.p7 * s.dn[229][20]), (p.p7 * s.dn[229][21]), (p.p7 * s.dn[229][22]), (p.p7 * s.db[229][0]), (p.p7 * s.db[229][1]), (p.p7 * s.db[229][2]), (p.p7 * s.db[229][3]), (p.p7 * s.db[229][4]), (p.p7 * s.db[229][5]), (p.p7 * s.db[229][6]), (p.p7 * s.db[229][7]), (p.p7 * s.db[229][8]), (p.p7 * s.db[229][9]), (p.p7 * s.db[229][10]), (p.p7 * s.db[229][11]), (p.p7 * s.db[229][12]), (p.p7 * s.db[229][13]), (p.p7 * s.db[229][14]), (p.p7 * s.db[229][15]), (p.p7 * s.db[229][16]), (p.p7 * s.db[229][17]), (p.p7 * s.db[229][18]), (p.p7 * s.db[229][19]), (p.p7 * s.db[229][20]), (p.p7 * s.db[229][21]), (p.p7 * s.db[229][22]), (p.p7 * s.db[229][23]), (p.p7 * s.db[229][24]), (p.p7 * s.db[229][25]), (p.p7 * s.db[229][26]), (p.p7 * s.db[229][27]), (p.p7 * s.db[229][28]), (p.p7 * s.db[229][29]), (p.p7 * s.db[229][30]), (p.p7 * s.db[229][31]), (p.p7 * s.db[229][32]), (p.p7 * s.db[229][33]), (p.p7 * s.db[229][34]), (p.p7 * s.db[229][35]), (p.p7 * s.db[229][36]), (p.p7 * s.db[229][37]), (p.p7 * s.db[229][38]), (p.p7 * s.db[229][39]), (p.p7 * s.db[229][40]), (p.p7 * s.db[229][41]), (p.p7 * s.db[229][42]), (p.p7 * s.db[229][43]), (p.p7 * s.db[229][44]), (p.p7 * s.db[229][45]), (p.p7 * s.db[229][46]), (p.p7 * s.db[229][47]), (p.p7 * s.db[229][48]), (p.p7 * s.db[229][49]), (p.p7 * s.db[229][50]), (p.p7 * s.db[229][51]), (p.p7 * s.db[229][52]), (p.p7 * s.db[229][53]), (p.p7 * s.db[229][54]), eq126_e1609_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_reactive_node_derivatives: [f64; 23] = [eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22];
        let eq126_reactive_branch_derivatives: [f64; 55] = [eq126_e1611_d_b0, eq126_e1611_d_b1, eq126_e1611_d_b2, eq126_e1611_d_b3, eq126_e1611_d_b4, eq126_e1611_d_b5, eq126_e1611_d_b6, eq126_e1611_d_b7, eq126_e1611_d_b8, eq126_e1611_d_b9, eq126_e1611_d_b10, eq126_e1611_d_b11, eq126_e1611_d_b12, eq126_e1611_d_b13, eq126_e1611_d_b14, eq126_e1611_d_b15, eq126_e1611_d_b16, eq126_e1611_d_b17, eq126_e1611_d_b18, eq126_e1611_d_b19, eq126_e1611_d_b20, eq126_e1611_d_b21, eq126_e1611_d_b22, eq126_e1611_d_b23, eq126_e1611_d_b24, eq126_e1611_d_b25, eq126_e1611_d_b26, eq126_e1611_d_b27, eq126_e1611_d_b28, eq126_e1611_d_b29, eq126_e1611_d_b30, eq126_e1611_d_b31, eq126_e1611_d_b32, eq126_e1611_d_b33, eq126_e1611_d_b34, eq126_e1611_d_b35, eq126_e1611_d_b36, eq126_e1611_d_b37, eq126_e1611_d_b38, eq126_e1611_d_b39, eq126_e1611_d_b40, eq126_e1611_d_b41, eq126_e1611_d_b42, eq126_e1611_d_b43, eq126_e1611_d_b44, eq126_e1611_d_b45, eq126_e1611_d_b46, eq126_e1611_d_b47, eq126_e1611_d_b48, eq126_e1611_d_b49, eq126_e1611_d_b50, eq126_e1611_d_b51, eq126_e1611_d_b52, eq126_e1611_d_b53, eq126_e1611_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq126_reactive_node_derivatives,
            branches,
            &eq126_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq127_e1623, eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22, eq127_e1623_d_b0, eq127_e1623_d_b1, eq127_e1623_d_b2, eq127_e1623_d_b3, eq127_e1623_d_b4, eq127_e1623_d_b5, eq127_e1623_d_b6, eq127_e1623_d_b7, eq127_e1623_d_b8, eq127_e1623_d_b9, eq127_e1623_d_b10, eq127_e1623_d_b11, eq127_e1623_d_b12, eq127_e1623_d_b13, eq127_e1623_d_b14, eq127_e1623_d_b15, eq127_e1623_d_b16, eq127_e1623_d_b17, eq127_e1623_d_b18, eq127_e1623_d_b19, eq127_e1623_d_b20, eq127_e1623_d_b21, eq127_e1623_d_b22, eq127_e1623_d_b23, eq127_e1623_d_b24, eq127_e1623_d_b25, eq127_e1623_d_b26, eq127_e1623_d_b27, eq127_e1623_d_b28, eq127_e1623_d_b29, eq127_e1623_d_b30, eq127_e1623_d_b31, eq127_e1623_d_b32, eq127_e1623_d_b33, eq127_e1623_d_b34, eq127_e1623_d_b35, eq127_e1623_d_b36, eq127_e1623_d_b37, eq127_e1623_d_b38, eq127_e1623_d_b39, eq127_e1623_d_b40, eq127_e1623_d_b41, eq127_e1623_d_b42, eq127_e1623_d_b43, eq127_e1623_d_b44, eq127_e1623_d_b45, eq127_e1623_d_b46, eq127_e1623_d_b47, eq127_e1623_d_b48, eq127_e1623_d_b49, eq127_e1623_d_b50, eq127_e1623_d_b51, eq127_e1623_d_b52, eq127_e1623_d_b53, eq127_e1623_d_b54, eq127_e1623_q,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq127_e1620_q: f64 = s.v[228];
        let eq127_e1621: f64 = (p.p7 * s.v[228]);
        let eq127_e1621_q: f64 = (p.p7 * eq127_e1620_q);
        (eq127_e1621, (p.p7 * s.dn[228][0]), (p.p7 * s.dn[228][1]), (p.p7 * s.dn[228][2]), (p.p7 * s.dn[228][3]), (p.p7 * s.dn[228][4]), (p.p7 * s.dn[228][5]), (p.p7 * s.dn[228][6]), (p.p7 * s.dn[228][7]), (p.p7 * s.dn[228][8]), (p.p7 * s.dn[228][9]), (p.p7 * s.dn[228][10]), (p.p7 * s.dn[228][11]), (p.p7 * s.dn[228][12]), (p.p7 * s.dn[228][13]), (p.p7 * s.dn[228][14]), (p.p7 * s.dn[228][15]), (p.p7 * s.dn[228][16]), (p.p7 * s.dn[228][17]), (p.p7 * s.dn[228][18]), (p.p7 * s.dn[228][19]), (p.p7 * s.dn[228][20]), (p.p7 * s.dn[228][21]), (p.p7 * s.dn[228][22]), (p.p7 * s.db[228][0]), (p.p7 * s.db[228][1]), (p.p7 * s.db[228][2]), (p.p7 * s.db[228][3]), (p.p7 * s.db[228][4]), (p.p7 * s.db[228][5]), (p.p7 * s.db[228][6]), (p.p7 * s.db[228][7]), (p.p7 * s.db[228][8]), (p.p7 * s.db[228][9]), (p.p7 * s.db[228][10]), (p.p7 * s.db[228][11]), (p.p7 * s.db[228][12]), (p.p7 * s.db[228][13]), (p.p7 * s.db[228][14]), (p.p7 * s.db[228][15]), (p.p7 * s.db[228][16]), (p.p7 * s.db[228][17]), (p.p7 * s.db[228][18]), (p.p7 * s.db[228][19]), (p.p7 * s.db[228][20]), (p.p7 * s.db[228][21]), (p.p7 * s.db[228][22]), (p.p7 * s.db[228][23]), (p.p7 * s.db[228][24]), (p.p7 * s.db[228][25]), (p.p7 * s.db[228][26]), (p.p7 * s.db[228][27]), (p.p7 * s.db[228][28]), (p.p7 * s.db[228][29]), (p.p7 * s.db[228][30]), (p.p7 * s.db[228][31]), (p.p7 * s.db[228][32]), (p.p7 * s.db[228][33]), (p.p7 * s.db[228][34]), (p.p7 * s.db[228][35]), (p.p7 * s.db[228][36]), (p.p7 * s.db[228][37]), (p.p7 * s.db[228][38]), (p.p7 * s.db[228][39]), (p.p7 * s.db[228][40]), (p.p7 * s.db[228][41]), (p.p7 * s.db[228][42]), (p.p7 * s.db[228][43]), (p.p7 * s.db[228][44]), (p.p7 * s.db[228][45]), (p.p7 * s.db[228][46]), (p.p7 * s.db[228][47]), (p.p7 * s.db[228][48]), (p.p7 * s.db[228][49]), (p.p7 * s.db[228][50]), (p.p7 * s.db[228][51]), (p.p7 * s.db[228][52]), (p.p7 * s.db[228][53]), (p.p7 * s.db[228][54]), eq127_e1621_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq127_reactive_node_derivatives: [f64; 23] = [eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22];
        let eq127_reactive_branch_derivatives: [f64; 55] = [eq127_e1623_d_b0, eq127_e1623_d_b1, eq127_e1623_d_b2, eq127_e1623_d_b3, eq127_e1623_d_b4, eq127_e1623_d_b5, eq127_e1623_d_b6, eq127_e1623_d_b7, eq127_e1623_d_b8, eq127_e1623_d_b9, eq127_e1623_d_b10, eq127_e1623_d_b11, eq127_e1623_d_b12, eq127_e1623_d_b13, eq127_e1623_d_b14, eq127_e1623_d_b15, eq127_e1623_d_b16, eq127_e1623_d_b17, eq127_e1623_d_b18, eq127_e1623_d_b19, eq127_e1623_d_b20, eq127_e1623_d_b21, eq127_e1623_d_b22, eq127_e1623_d_b23, eq127_e1623_d_b24, eq127_e1623_d_b25, eq127_e1623_d_b26, eq127_e1623_d_b27, eq127_e1623_d_b28, eq127_e1623_d_b29, eq127_e1623_d_b30, eq127_e1623_d_b31, eq127_e1623_d_b32, eq127_e1623_d_b33, eq127_e1623_d_b34, eq127_e1623_d_b35, eq127_e1623_d_b36, eq127_e1623_d_b37, eq127_e1623_d_b38, eq127_e1623_d_b39, eq127_e1623_d_b40, eq127_e1623_d_b41, eq127_e1623_d_b42, eq127_e1623_d_b43, eq127_e1623_d_b44, eq127_e1623_d_b45, eq127_e1623_d_b46, eq127_e1623_d_b47, eq127_e1623_d_b48, eq127_e1623_d_b49, eq127_e1623_d_b50, eq127_e1623_d_b51, eq127_e1623_d_b52, eq127_e1623_d_b53, eq127_e1623_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq127_reactive_node_derivatives,
            branches,
            &eq127_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1637, eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22, eq128_e1637_d_b0, eq128_e1637_d_b1, eq128_e1637_d_b2, eq128_e1637_d_b3, eq128_e1637_d_b4, eq128_e1637_d_b5, eq128_e1637_d_b6, eq128_e1637_d_b7, eq128_e1637_d_b8, eq128_e1637_d_b9, eq128_e1637_d_b10, eq128_e1637_d_b11, eq128_e1637_d_b12, eq128_e1637_d_b13, eq128_e1637_d_b14, eq128_e1637_d_b15, eq128_e1637_d_b16, eq128_e1637_d_b17, eq128_e1637_d_b18, eq128_e1637_d_b19, eq128_e1637_d_b20, eq128_e1637_d_b21, eq128_e1637_d_b22, eq128_e1637_d_b23, eq128_e1637_d_b24, eq128_e1637_d_b25, eq128_e1637_d_b26, eq128_e1637_d_b27, eq128_e1637_d_b28, eq128_e1637_d_b29, eq128_e1637_d_b30, eq128_e1637_d_b31, eq128_e1637_d_b32, eq128_e1637_d_b33, eq128_e1637_d_b34, eq128_e1637_d_b35, eq128_e1637_d_b36, eq128_e1637_d_b37, eq128_e1637_d_b38, eq128_e1637_d_b39, eq128_e1637_d_b40, eq128_e1637_d_b41, eq128_e1637_d_b42, eq128_e1637_d_b43, eq128_e1637_d_b44, eq128_e1637_d_b45, eq128_e1637_d_b46, eq128_e1637_d_b47, eq128_e1637_d_b48, eq128_e1637_d_b49, eq128_e1637_d_b50, eq128_e1637_d_b51, eq128_e1637_d_b52, eq128_e1637_d_b53, eq128_e1637_d_b54, eq128_e1637_q,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq128_e1632_q: f64 = s.v[228];
        let eq128_e1633: f64 = (p.p7 * s.v[228]);
        let eq128_e1633_q: f64 = (p.p7 * eq128_e1632_q);
        let eq128_e1635: f64 = (eq128_e1633 * p.p246);
        let eq128_e1635_q: f64 = (eq128_e1633_q * p.p246);
        (eq128_e1635, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq128_e1635_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_reactive_node_derivatives: [f64; 23] = [eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22];
        let eq128_reactive_branch_derivatives: [f64; 55] = [eq128_e1637_d_b0, eq128_e1637_d_b1, eq128_e1637_d_b2, eq128_e1637_d_b3, eq128_e1637_d_b4, eq128_e1637_d_b5, eq128_e1637_d_b6, eq128_e1637_d_b7, eq128_e1637_d_b8, eq128_e1637_d_b9, eq128_e1637_d_b10, eq128_e1637_d_b11, eq128_e1637_d_b12, eq128_e1637_d_b13, eq128_e1637_d_b14, eq128_e1637_d_b15, eq128_e1637_d_b16, eq128_e1637_d_b17, eq128_e1637_d_b18, eq128_e1637_d_b19, eq128_e1637_d_b20, eq128_e1637_d_b21, eq128_e1637_d_b22, eq128_e1637_d_b23, eq128_e1637_d_b24, eq128_e1637_d_b25, eq128_e1637_d_b26, eq128_e1637_d_b27, eq128_e1637_d_b28, eq128_e1637_d_b29, eq128_e1637_d_b30, eq128_e1637_d_b31, eq128_e1637_d_b32, eq128_e1637_d_b33, eq128_e1637_d_b34, eq128_e1637_d_b35, eq128_e1637_d_b36, eq128_e1637_d_b37, eq128_e1637_d_b38, eq128_e1637_d_b39, eq128_e1637_d_b40, eq128_e1637_d_b41, eq128_e1637_d_b42, eq128_e1637_d_b43, eq128_e1637_d_b44, eq128_e1637_d_b45, eq128_e1637_d_b46, eq128_e1637_d_b47, eq128_e1637_d_b48, eq128_e1637_d_b49, eq128_e1637_d_b50, eq128_e1637_d_b51, eq128_e1637_d_b52, eq128_e1637_d_b53, eq128_e1637_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq128_reactive_node_derivatives,
            branches,
            &eq128_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1650, eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22, eq129_e1650_d_b0, eq129_e1650_d_b1, eq129_e1650_d_b2, eq129_e1650_d_b3, eq129_e1650_d_b4, eq129_e1650_d_b5, eq129_e1650_d_b6, eq129_e1650_d_b7, eq129_e1650_d_b8, eq129_e1650_d_b9, eq129_e1650_d_b10, eq129_e1650_d_b11, eq129_e1650_d_b12, eq129_e1650_d_b13, eq129_e1650_d_b14, eq129_e1650_d_b15, eq129_e1650_d_b16, eq129_e1650_d_b17, eq129_e1650_d_b18, eq129_e1650_d_b19, eq129_e1650_d_b20, eq129_e1650_d_b21, eq129_e1650_d_b22, eq129_e1650_d_b23, eq129_e1650_d_b24, eq129_e1650_d_b25, eq129_e1650_d_b26, eq129_e1650_d_b27, eq129_e1650_d_b28, eq129_e1650_d_b29, eq129_e1650_d_b30, eq129_e1650_d_b31, eq129_e1650_d_b32, eq129_e1650_d_b33, eq129_e1650_d_b34, eq129_e1650_d_b35, eq129_e1650_d_b36, eq129_e1650_d_b37, eq129_e1650_d_b38, eq129_e1650_d_b39, eq129_e1650_d_b40, eq129_e1650_d_b41, eq129_e1650_d_b42, eq129_e1650_d_b43, eq129_e1650_d_b44, eq129_e1650_d_b45, eq129_e1650_d_b46, eq129_e1650_d_b47, eq129_e1650_d_b48, eq129_e1650_d_b49, eq129_e1650_d_b50, eq129_e1650_d_b51, eq129_e1650_d_b52, eq129_e1650_d_b53, eq129_e1650_d_b54, eq129_e1650_q,) = {
    if (((!s.b[570]) && s.b[573]) && (!s.b[574])) {
        let eq129_e1647_q: f64 = s.v[228];
        let eq129_e1648: f64 = (p.p7 * s.v[228]);
        let eq129_e1648_q: f64 = (p.p7 * eq129_e1647_q);
        (eq129_e1648, (p.p7 * s.dn[228][0]), (p.p7 * s.dn[228][1]), (p.p7 * s.dn[228][2]), (p.p7 * s.dn[228][3]), (p.p7 * s.dn[228][4]), (p.p7 * s.dn[228][5]), (p.p7 * s.dn[228][6]), (p.p7 * s.dn[228][7]), (p.p7 * s.dn[228][8]), (p.p7 * s.dn[228][9]), (p.p7 * s.dn[228][10]), (p.p7 * s.dn[228][11]), (p.p7 * s.dn[228][12]), (p.p7 * s.dn[228][13]), (p.p7 * s.dn[228][14]), (p.p7 * s.dn[228][15]), (p.p7 * s.dn[228][16]), (p.p7 * s.dn[228][17]), (p.p7 * s.dn[228][18]), (p.p7 * s.dn[228][19]), (p.p7 * s.dn[228][20]), (p.p7 * s.dn[228][21]), (p.p7 * s.dn[228][22]), (p.p7 * s.db[228][0]), (p.p7 * s.db[228][1]), (p.p7 * s.db[228][2]), (p.p7 * s.db[228][3]), (p.p7 * s.db[228][4]), (p.p7 * s.db[228][5]), (p.p7 * s.db[228][6]), (p.p7 * s.db[228][7]), (p.p7 * s.db[228][8]), (p.p7 * s.db[228][9]), (p.p7 * s.db[228][10]), (p.p7 * s.db[228][11]), (p.p7 * s.db[228][12]), (p.p7 * s.db[228][13]), (p.p7 * s.db[228][14]), (p.p7 * s.db[228][15]), (p.p7 * s.db[228][16]), (p.p7 * s.db[228][17]), (p.p7 * s.db[228][18]), (p.p7 * s.db[228][19]), (p.p7 * s.db[228][20]), (p.p7 * s.db[228][21]), (p.p7 * s.db[228][22]), (p.p7 * s.db[228][23]), (p.p7 * s.db[228][24]), (p.p7 * s.db[228][25]), (p.p7 * s.db[228][26]), (p.p7 * s.db[228][27]), (p.p7 * s.db[228][28]), (p.p7 * s.db[228][29]), (p.p7 * s.db[228][30]), (p.p7 * s.db[228][31]), (p.p7 * s.db[228][32]), (p.p7 * s.db[228][33]), (p.p7 * s.db[228][34]), (p.p7 * s.db[228][35]), (p.p7 * s.db[228][36]), (p.p7 * s.db[228][37]), (p.p7 * s.db[228][38]), (p.p7 * s.db[228][39]), (p.p7 * s.db[228][40]), (p.p7 * s.db[228][41]), (p.p7 * s.db[228][42]), (p.p7 * s.db[228][43]), (p.p7 * s.db[228][44]), (p.p7 * s.db[228][45]), (p.p7 * s.db[228][46]), (p.p7 * s.db[228][47]), (p.p7 * s.db[228][48]), (p.p7 * s.db[228][49]), (p.p7 * s.db[228][50]), (p.p7 * s.db[228][51]), (p.p7 * s.db[228][52]), (p.p7 * s.db[228][53]), (p.p7 * s.db[228][54]), eq129_e1648_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_reactive_node_derivatives: [f64; 23] = [eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22];
        let eq129_reactive_branch_derivatives: [f64; 55] = [eq129_e1650_d_b0, eq129_e1650_d_b1, eq129_e1650_d_b2, eq129_e1650_d_b3, eq129_e1650_d_b4, eq129_e1650_d_b5, eq129_e1650_d_b6, eq129_e1650_d_b7, eq129_e1650_d_b8, eq129_e1650_d_b9, eq129_e1650_d_b10, eq129_e1650_d_b11, eq129_e1650_d_b12, eq129_e1650_d_b13, eq129_e1650_d_b14, eq129_e1650_d_b15, eq129_e1650_d_b16, eq129_e1650_d_b17, eq129_e1650_d_b18, eq129_e1650_d_b19, eq129_e1650_d_b20, eq129_e1650_d_b21, eq129_e1650_d_b22, eq129_e1650_d_b23, eq129_e1650_d_b24, eq129_e1650_d_b25, eq129_e1650_d_b26, eq129_e1650_d_b27, eq129_e1650_d_b28, eq129_e1650_d_b29, eq129_e1650_d_b30, eq129_e1650_d_b31, eq129_e1650_d_b32, eq129_e1650_d_b33, eq129_e1650_d_b34, eq129_e1650_d_b35, eq129_e1650_d_b36, eq129_e1650_d_b37, eq129_e1650_d_b38, eq129_e1650_d_b39, eq129_e1650_d_b40, eq129_e1650_d_b41, eq129_e1650_d_b42, eq129_e1650_d_b43, eq129_e1650_d_b44, eq129_e1650_d_b45, eq129_e1650_d_b46, eq129_e1650_d_b47, eq129_e1650_d_b48, eq129_e1650_d_b49, eq129_e1650_d_b50, eq129_e1650_d_b51, eq129_e1650_d_b52, eq129_e1650_d_b53, eq129_e1650_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq129_reactive_node_derivatives,
            branches,
            &eq129_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = ((p.p7 * s.dn[240][0]) * p.p246);
        let __rspice_deriv_cse_1: f64 = ((p.p7 * s.dn[240][1]) * p.p246);
        let __rspice_deriv_cse_2: f64 = ((p.p7 * s.dn[240][2]) * p.p246);
        let __rspice_deriv_cse_3: f64 = ((p.p7 * s.dn[240][3]) * p.p246);
        let __rspice_deriv_cse_4: f64 = ((p.p7 * s.dn[240][4]) * p.p246);
        let __rspice_deriv_cse_5: f64 = ((p.p7 * s.dn[240][5]) * p.p246);
        let __rspice_deriv_cse_6: f64 = ((p.p7 * s.dn[240][6]) * p.p246);
        let __rspice_deriv_cse_7: f64 = ((p.p7 * s.dn[240][7]) * p.p246);
        let __rspice_deriv_cse_8: f64 = ((p.p7 * s.dn[240][8]) * p.p246);
        let __rspice_deriv_cse_9: f64 = ((p.p7 * s.dn[240][9]) * p.p246);
        let __rspice_deriv_cse_10: f64 = ((p.p7 * s.dn[240][10]) * p.p246);
        let __rspice_deriv_cse_11: f64 = ((p.p7 * s.dn[240][11]) * p.p246);
        let __rspice_deriv_cse_12: f64 = ((p.p7 * s.dn[240][12]) * p.p246);
        let __rspice_deriv_cse_13: f64 = ((p.p7 * s.dn[240][13]) * p.p246);
        let __rspice_deriv_cse_14: f64 = ((p.p7 * s.dn[240][14]) * p.p246);
        let __rspice_deriv_cse_15: f64 = ((p.p7 * s.dn[240][15]) * p.p246);
        let __rspice_deriv_cse_16: f64 = ((p.p7 * s.dn[240][16]) * p.p246);
        let __rspice_deriv_cse_17: f64 = ((p.p7 * s.dn[240][17]) * p.p246);
        let __rspice_deriv_cse_18: f64 = ((p.p7 * s.dn[240][18]) * p.p246);
        let __rspice_deriv_cse_19: f64 = ((p.p7 * s.dn[240][19]) * p.p246);
        let __rspice_deriv_cse_20: f64 = ((p.p7 * s.dn[240][20]) * p.p246);
        let __rspice_deriv_cse_21: f64 = ((p.p7 * s.dn[240][21]) * p.p246);
        let __rspice_deriv_cse_22: f64 = ((p.p7 * s.dn[240][22]) * p.p246);
        let __rspice_deriv_cse_23: f64 = ((p.p7 * s.db[240][0]) * p.p246);
        let __rspice_deriv_cse_24: f64 = ((p.p7 * s.db[240][1]) * p.p246);
        let __rspice_deriv_cse_25: f64 = ((p.p7 * s.db[240][2]) * p.p246);
        let __rspice_deriv_cse_26: f64 = ((p.p7 * s.db[240][3]) * p.p246);
        let __rspice_deriv_cse_27: f64 = ((p.p7 * s.db[240][4]) * p.p246);
        let __rspice_deriv_cse_28: f64 = ((p.p7 * s.db[240][5]) * p.p246);
        let __rspice_deriv_cse_29: f64 = ((p.p7 * s.db[240][6]) * p.p246);
        let __rspice_deriv_cse_30: f64 = ((p.p7 * s.db[240][7]) * p.p246);
        let __rspice_deriv_cse_31: f64 = ((p.p7 * s.db[240][8]) * p.p246);
        let __rspice_deriv_cse_32: f64 = ((p.p7 * s.db[240][9]) * p.p246);
        let __rspice_deriv_cse_33: f64 = ((p.p7 * s.db[240][10]) * p.p246);
        let __rspice_deriv_cse_34: f64 = ((p.p7 * s.db[240][11]) * p.p246);
        let __rspice_deriv_cse_35: f64 = ((p.p7 * s.db[240][12]) * p.p246);
        let __rspice_deriv_cse_36: f64 = ((p.p7 * s.db[240][13]) * p.p246);
        let __rspice_deriv_cse_37: f64 = ((p.p7 * s.db[240][14]) * p.p246);
        let __rspice_deriv_cse_38: f64 = ((p.p7 * s.db[240][15]) * p.p246);
        let __rspice_deriv_cse_39: f64 = ((p.p7 * s.db[240][16]) * p.p246);
        let __rspice_deriv_cse_40: f64 = ((p.p7 * s.db[240][17]) * p.p246);
        let __rspice_deriv_cse_41: f64 = ((p.p7 * s.db[240][18]) * p.p246);
        let __rspice_deriv_cse_42: f64 = ((p.p7 * s.db[240][19]) * p.p246);
        let __rspice_deriv_cse_43: f64 = ((p.p7 * s.db[240][20]) * p.p246);
        let __rspice_deriv_cse_44: f64 = ((p.p7 * s.db[240][21]) * p.p246);
        let __rspice_deriv_cse_45: f64 = ((p.p7 * s.db[240][22]) * p.p246);
        let __rspice_deriv_cse_46: f64 = ((p.p7 * s.db[240][23]) * p.p246);
        let __rspice_deriv_cse_47: f64 = ((p.p7 * s.db[240][24]) * p.p246);
        let __rspice_deriv_cse_48: f64 = ((p.p7 * s.db[240][25]) * p.p246);
        let __rspice_deriv_cse_49: f64 = ((p.p7 * s.db[240][26]) * p.p246);
        let __rspice_deriv_cse_50: f64 = ((p.p7 * s.db[240][27]) * p.p246);
        let __rspice_deriv_cse_51: f64 = ((p.p7 * s.db[240][28]) * p.p246);
        let __rspice_deriv_cse_52: f64 = ((p.p7 * s.db[240][29]) * p.p246);
        let __rspice_deriv_cse_53: f64 = ((p.p7 * s.db[240][30]) * p.p246);
        let __rspice_deriv_cse_54: f64 = ((p.p7 * s.db[240][31]) * p.p246);
        let __rspice_deriv_cse_55: f64 = ((p.p7 * s.db[240][32]) * p.p246);
        let __rspice_deriv_cse_56: f64 = ((p.p7 * s.db[240][33]) * p.p246);
        let __rspice_deriv_cse_57: f64 = ((p.p7 * s.db[240][34]) * p.p246);
        let __rspice_deriv_cse_58: f64 = ((p.p7 * s.db[240][35]) * p.p246);
        let __rspice_deriv_cse_59: f64 = ((p.p7 * s.db[240][36]) * p.p246);
        let __rspice_deriv_cse_60: f64 = ((p.p7 * s.db[240][37]) * p.p246);
        let __rspice_deriv_cse_61: f64 = ((p.p7 * s.db[240][38]) * p.p246);
        let __rspice_deriv_cse_62: f64 = ((p.p7 * s.db[240][39]) * p.p246);
        let __rspice_deriv_cse_63: f64 = ((p.p7 * s.db[240][40]) * p.p246);
        let __rspice_deriv_cse_64: f64 = ((p.p7 * s.db[240][41]) * p.p246);
        let __rspice_deriv_cse_65: f64 = ((p.p7 * s.db[240][42]) * p.p246);
        let __rspice_deriv_cse_66: f64 = ((p.p7 * s.db[240][43]) * p.p246);
        let __rspice_deriv_cse_67: f64 = ((p.p7 * s.db[240][44]) * p.p246);
        let __rspice_deriv_cse_68: f64 = ((p.p7 * s.db[240][45]) * p.p246);
        let __rspice_deriv_cse_69: f64 = ((p.p7 * s.db[240][46]) * p.p246);
        let __rspice_deriv_cse_70: f64 = ((p.p7 * s.db[240][47]) * p.p246);
        let __rspice_deriv_cse_71: f64 = ((p.p7 * s.db[240][48]) * p.p246);
        let __rspice_deriv_cse_72: f64 = ((p.p7 * s.db[240][49]) * p.p246);
        let __rspice_deriv_cse_73: f64 = ((p.p7 * s.db[240][50]) * p.p246);
        let __rspice_deriv_cse_74: f64 = ((p.p7 * s.db[240][51]) * p.p246);
        let __rspice_deriv_cse_75: f64 = ((p.p7 * s.db[240][52]) * p.p246);
        let __rspice_deriv_cse_76: f64 = ((p.p7 * s.db[240][53]) * p.p246);
        let __rspice_deriv_cse_77: f64 = ((p.p7 * s.db[240][54]) * p.p246);
        let (eq130_e1665, eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22, eq130_e1665_d_b0, eq130_e1665_d_b1, eq130_e1665_d_b2, eq130_e1665_d_b3, eq130_e1665_d_b4, eq130_e1665_d_b5, eq130_e1665_d_b6, eq130_e1665_d_b7, eq130_e1665_d_b8, eq130_e1665_d_b9, eq130_e1665_d_b10, eq130_e1665_d_b11, eq130_e1665_d_b12, eq130_e1665_d_b13, eq130_e1665_d_b14, eq130_e1665_d_b15, eq130_e1665_d_b16, eq130_e1665_d_b17, eq130_e1665_d_b18, eq130_e1665_d_b19, eq130_e1665_d_b20, eq130_e1665_d_b21, eq130_e1665_d_b22, eq130_e1665_d_b23, eq130_e1665_d_b24, eq130_e1665_d_b25, eq130_e1665_d_b26, eq130_e1665_d_b27, eq130_e1665_d_b28, eq130_e1665_d_b29, eq130_e1665_d_b30, eq130_e1665_d_b31, eq130_e1665_d_b32, eq130_e1665_d_b33, eq130_e1665_d_b34, eq130_e1665_d_b35, eq130_e1665_d_b36, eq130_e1665_d_b37, eq130_e1665_d_b38, eq130_e1665_d_b39, eq130_e1665_d_b40, eq130_e1665_d_b41, eq130_e1665_d_b42, eq130_e1665_d_b43, eq130_e1665_d_b44, eq130_e1665_d_b45, eq130_e1665_d_b46, eq130_e1665_d_b47, eq130_e1665_d_b48, eq130_e1665_d_b49, eq130_e1665_d_b50, eq130_e1665_d_b51, eq130_e1665_d_b52, eq130_e1665_d_b53, eq130_e1665_d_b54, eq130_e1665_q,) = {
    if (((!s.b[570]) && s.b[573]) && (!s.b[574])) {
        let eq130_e1660_q: f64 = s.v[228];
        let eq130_e1661: f64 = (p.p7 * s.v[228]);
        let eq130_e1661_q: f64 = (p.p7 * eq130_e1660_q);
        let eq130_e1663: f64 = (eq130_e1661 * p.p246);
        let eq130_e1663_d_n0: f64 = ((p.p7 * s.dn[228][0]) * p.p246);
        let eq130_e1663_d_n1: f64 = ((p.p7 * s.dn[228][1]) * p.p246);
        let eq130_e1663_d_n2: f64 = ((p.p7 * s.dn[228][2]) * p.p246);
        let eq130_e1663_d_n3: f64 = ((p.p7 * s.dn[228][3]) * p.p246);
        let eq130_e1663_d_n4: f64 = ((p.p7 * s.dn[228][4]) * p.p246);
        let eq130_e1663_d_n5: f64 = ((p.p7 * s.dn[228][5]) * p.p246);
        let eq130_e1663_d_n6: f64 = ((p.p7 * s.dn[228][6]) * p.p246);
        let eq130_e1663_d_n7: f64 = ((p.p7 * s.dn[228][7]) * p.p246);
        let eq130_e1663_d_n8: f64 = ((p.p7 * s.dn[228][8]) * p.p246);
        let eq130_e1663_d_n9: f64 = ((p.p7 * s.dn[228][9]) * p.p246);
        let eq130_e1663_d_n10: f64 = ((p.p7 * s.dn[228][10]) * p.p246);
        let eq130_e1663_d_n11: f64 = ((p.p7 * s.dn[228][11]) * p.p246);
        let eq130_e1663_d_n12: f64 = ((p.p7 * s.dn[228][12]) * p.p246);
        let eq130_e1663_d_n13: f64 = ((p.p7 * s.dn[228][13]) * p.p246);
        let eq130_e1663_d_n14: f64 = ((p.p7 * s.dn[228][14]) * p.p246);
        let eq130_e1663_d_n15: f64 = ((p.p7 * s.dn[228][15]) * p.p246);
        let eq130_e1663_d_n16: f64 = ((p.p7 * s.dn[228][16]) * p.p246);
        let eq130_e1663_d_n17: f64 = ((p.p7 * s.dn[228][17]) * p.p246);
        let eq130_e1663_d_n18: f64 = ((p.p7 * s.dn[228][18]) * p.p246);
        let eq130_e1663_d_n19: f64 = ((p.p7 * s.dn[228][19]) * p.p246);
        let eq130_e1663_d_n20: f64 = ((p.p7 * s.dn[228][20]) * p.p246);
        let eq130_e1663_d_n21: f64 = ((p.p7 * s.dn[228][21]) * p.p246);
        let eq130_e1663_d_n22: f64 = ((p.p7 * s.dn[228][22]) * p.p246);
        let eq130_e1663_d_b0: f64 = ((p.p7 * s.db[228][0]) * p.p246);
        let eq130_e1663_d_b1: f64 = ((p.p7 * s.db[228][1]) * p.p246);
        let eq130_e1663_d_b2: f64 = ((p.p7 * s.db[228][2]) * p.p246);
        let eq130_e1663_d_b3: f64 = ((p.p7 * s.db[228][3]) * p.p246);
        let eq130_e1663_d_b4: f64 = ((p.p7 * s.db[228][4]) * p.p246);
        let eq130_e1663_d_b5: f64 = ((p.p7 * s.db[228][5]) * p.p246);
        let eq130_e1663_d_b6: f64 = ((p.p7 * s.db[228][6]) * p.p246);
        let eq130_e1663_d_b7: f64 = ((p.p7 * s.db[228][7]) * p.p246);
        let eq130_e1663_d_b8: f64 = ((p.p7 * s.db[228][8]) * p.p246);
        let eq130_e1663_d_b9: f64 = ((p.p7 * s.db[228][9]) * p.p246);
        let eq130_e1663_d_b10: f64 = ((p.p7 * s.db[228][10]) * p.p246);
        let eq130_e1663_d_b11: f64 = ((p.p7 * s.db[228][11]) * p.p246);
        let eq130_e1663_d_b12: f64 = ((p.p7 * s.db[228][12]) * p.p246);
        let eq130_e1663_d_b13: f64 = ((p.p7 * s.db[228][13]) * p.p246);
        let eq130_e1663_d_b14: f64 = ((p.p7 * s.db[228][14]) * p.p246);
        let eq130_e1663_d_b15: f64 = ((p.p7 * s.db[228][15]) * p.p246);
        let eq130_e1663_d_b16: f64 = ((p.p7 * s.db[228][16]) * p.p246);
        let eq130_e1663_d_b17: f64 = ((p.p7 * s.db[228][17]) * p.p246);
        let eq130_e1663_d_b18: f64 = ((p.p7 * s.db[228][18]) * p.p246);
        let eq130_e1663_d_b19: f64 = ((p.p7 * s.db[228][19]) * p.p246);
        let eq130_e1663_d_b20: f64 = ((p.p7 * s.db[228][20]) * p.p246);
        let eq130_e1663_d_b21: f64 = ((p.p7 * s.db[228][21]) * p.p246);
        let eq130_e1663_d_b22: f64 = ((p.p7 * s.db[228][22]) * p.p246);
        let eq130_e1663_d_b23: f64 = ((p.p7 * s.db[228][23]) * p.p246);
        let eq130_e1663_d_b24: f64 = ((p.p7 * s.db[228][24]) * p.p246);
        let eq130_e1663_d_b25: f64 = ((p.p7 * s.db[228][25]) * p.p246);
        let eq130_e1663_d_b26: f64 = ((p.p7 * s.db[228][26]) * p.p246);
        let eq130_e1663_d_b27: f64 = ((p.p7 * s.db[228][27]) * p.p246);
        let eq130_e1663_d_b28: f64 = ((p.p7 * s.db[228][28]) * p.p246);
        let eq130_e1663_d_b29: f64 = ((p.p7 * s.db[228][29]) * p.p246);
        let eq130_e1663_d_b30: f64 = ((p.p7 * s.db[228][30]) * p.p246);
        let eq130_e1663_d_b31: f64 = ((p.p7 * s.db[228][31]) * p.p246);
        let eq130_e1663_d_b32: f64 = ((p.p7 * s.db[228][32]) * p.p246);
        let eq130_e1663_d_b33: f64 = ((p.p7 * s.db[228][33]) * p.p246);
        let eq130_e1663_d_b34: f64 = ((p.p7 * s.db[228][34]) * p.p246);
        let eq130_e1663_d_b35: f64 = ((p.p7 * s.db[228][35]) * p.p246);
        let eq130_e1663_d_b36: f64 = ((p.p7 * s.db[228][36]) * p.p246);
        let eq130_e1663_d_b37: f64 = ((p.p7 * s.db[228][37]) * p.p246);
        let eq130_e1663_d_b38: f64 = ((p.p7 * s.db[228][38]) * p.p246);
        let eq130_e1663_d_b39: f64 = ((p.p7 * s.db[228][39]) * p.p246);
        let eq130_e1663_d_b40: f64 = ((p.p7 * s.db[228][40]) * p.p246);
        let eq130_e1663_d_b41: f64 = ((p.p7 * s.db[228][41]) * p.p246);
        let eq130_e1663_d_b42: f64 = ((p.p7 * s.db[228][42]) * p.p246);
        let eq130_e1663_d_b43: f64 = ((p.p7 * s.db[228][43]) * p.p246);
        let eq130_e1663_d_b44: f64 = ((p.p7 * s.db[228][44]) * p.p246);
        let eq130_e1663_d_b45: f64 = ((p.p7 * s.db[228][45]) * p.p246);
        let eq130_e1663_d_b46: f64 = ((p.p7 * s.db[228][46]) * p.p246);
        let eq130_e1663_d_b47: f64 = ((p.p7 * s.db[228][47]) * p.p246);
        let eq130_e1663_d_b48: f64 = ((p.p7 * s.db[228][48]) * p.p246);
        let eq130_e1663_d_b49: f64 = ((p.p7 * s.db[228][49]) * p.p246);
        let eq130_e1663_d_b50: f64 = ((p.p7 * s.db[228][50]) * p.p246);
        let eq130_e1663_d_b51: f64 = ((p.p7 * s.db[228][51]) * p.p246);
        let eq130_e1663_d_b52: f64 = ((p.p7 * s.db[228][52]) * p.p246);
        let eq130_e1663_d_b53: f64 = ((p.p7 * s.db[228][53]) * p.p246);
        let eq130_e1663_d_b54: f64 = ((p.p7 * s.db[228][54]) * p.p246);
        let eq130_e1663_q: f64 = (eq130_e1661_q * p.p246);
        (eq130_e1663, eq130_e1663_d_n0, eq130_e1663_d_n1, eq130_e1663_d_n2, eq130_e1663_d_n3, eq130_e1663_d_n4, eq130_e1663_d_n5, eq130_e1663_d_n6, eq130_e1663_d_n7, eq130_e1663_d_n8, eq130_e1663_d_n9, eq130_e1663_d_n10, eq130_e1663_d_n11, eq130_e1663_d_n12, eq130_e1663_d_n13, eq130_e1663_d_n14, eq130_e1663_d_n15, eq130_e1663_d_n16, eq130_e1663_d_n17, eq130_e1663_d_n18, eq130_e1663_d_n19, eq130_e1663_d_n20, eq130_e1663_d_n21, eq130_e1663_d_n22, eq130_e1663_d_b0, eq130_e1663_d_b1, eq130_e1663_d_b2, eq130_e1663_d_b3, eq130_e1663_d_b4, eq130_e1663_d_b5, eq130_e1663_d_b6, eq130_e1663_d_b7, eq130_e1663_d_b8, eq130_e1663_d_b9, eq130_e1663_d_b10, eq130_e1663_d_b11, eq130_e1663_d_b12, eq130_e1663_d_b13, eq130_e1663_d_b14, eq130_e1663_d_b15, eq130_e1663_d_b16, eq130_e1663_d_b17, eq130_e1663_d_b18, eq130_e1663_d_b19, eq130_e1663_d_b20, eq130_e1663_d_b21, eq130_e1663_d_b22, eq130_e1663_d_b23, eq130_e1663_d_b24, eq130_e1663_d_b25, eq130_e1663_d_b26, eq130_e1663_d_b27, eq130_e1663_d_b28, eq130_e1663_d_b29, eq130_e1663_d_b30, eq130_e1663_d_b31, eq130_e1663_d_b32, eq130_e1663_d_b33, eq130_e1663_d_b34, eq130_e1663_d_b35, eq130_e1663_d_b36, eq130_e1663_d_b37, eq130_e1663_d_b38, eq130_e1663_d_b39, eq130_e1663_d_b40, eq130_e1663_d_b41, eq130_e1663_d_b42, eq130_e1663_d_b43, eq130_e1663_d_b44, eq130_e1663_d_b45, eq130_e1663_d_b46, eq130_e1663_d_b47, eq130_e1663_d_b48, eq130_e1663_d_b49, eq130_e1663_d_b50, eq130_e1663_d_b51, eq130_e1663_d_b52, eq130_e1663_d_b53, eq130_e1663_d_b54, eq130_e1663_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_reactive_node_derivatives: [f64; 23] = [eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22];
        let eq130_reactive_branch_derivatives: [f64; 55] = [eq130_e1665_d_b0, eq130_e1665_d_b1, eq130_e1665_d_b2, eq130_e1665_d_b3, eq130_e1665_d_b4, eq130_e1665_d_b5, eq130_e1665_d_b6, eq130_e1665_d_b7, eq130_e1665_d_b8, eq130_e1665_d_b9, eq130_e1665_d_b10, eq130_e1665_d_b11, eq130_e1665_d_b12, eq130_e1665_d_b13, eq130_e1665_d_b14, eq130_e1665_d_b15, eq130_e1665_d_b16, eq130_e1665_d_b17, eq130_e1665_d_b18, eq130_e1665_d_b19, eq130_e1665_d_b20, eq130_e1665_d_b21, eq130_e1665_d_b22, eq130_e1665_d_b23, eq130_e1665_d_b24, eq130_e1665_d_b25, eq130_e1665_d_b26, eq130_e1665_d_b27, eq130_e1665_d_b28, eq130_e1665_d_b29, eq130_e1665_d_b30, eq130_e1665_d_b31, eq130_e1665_d_b32, eq130_e1665_d_b33, eq130_e1665_d_b34, eq130_e1665_d_b35, eq130_e1665_d_b36, eq130_e1665_d_b37, eq130_e1665_d_b38, eq130_e1665_d_b39, eq130_e1665_d_b40, eq130_e1665_d_b41, eq130_e1665_d_b42, eq130_e1665_d_b43, eq130_e1665_d_b44, eq130_e1665_d_b45, eq130_e1665_d_b46, eq130_e1665_d_b47, eq130_e1665_d_b48, eq130_e1665_d_b49, eq130_e1665_d_b50, eq130_e1665_d_b51, eq130_e1665_d_b52, eq130_e1665_d_b53, eq130_e1665_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq130_reactive_node_derivatives,
            branches,
            &eq130_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1677, eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22, eq131_e1677_d_b0, eq131_e1677_d_b1, eq131_e1677_d_b2, eq131_e1677_d_b3, eq131_e1677_d_b4, eq131_e1677_d_b5, eq131_e1677_d_b6, eq131_e1677_d_b7, eq131_e1677_d_b8, eq131_e1677_d_b9, eq131_e1677_d_b10, eq131_e1677_d_b11, eq131_e1677_d_b12, eq131_e1677_d_b13, eq131_e1677_d_b14, eq131_e1677_d_b15, eq131_e1677_d_b16, eq131_e1677_d_b17, eq131_e1677_d_b18, eq131_e1677_d_b19, eq131_e1677_d_b20, eq131_e1677_d_b21, eq131_e1677_d_b22, eq131_e1677_d_b23, eq131_e1677_d_b24, eq131_e1677_d_b25, eq131_e1677_d_b26, eq131_e1677_d_b27, eq131_e1677_d_b28, eq131_e1677_d_b29, eq131_e1677_d_b30, eq131_e1677_d_b31, eq131_e1677_d_b32, eq131_e1677_d_b33, eq131_e1677_d_b34, eq131_e1677_d_b35, eq131_e1677_d_b36, eq131_e1677_d_b37, eq131_e1677_d_b38, eq131_e1677_d_b39, eq131_e1677_d_b40, eq131_e1677_d_b41, eq131_e1677_d_b42, eq131_e1677_d_b43, eq131_e1677_d_b44, eq131_e1677_d_b45, eq131_e1677_d_b46, eq131_e1677_d_b47, eq131_e1677_d_b48, eq131_e1677_d_b49, eq131_e1677_d_b50, eq131_e1677_d_b51, eq131_e1677_d_b52, eq131_e1677_d_b53, eq131_e1677_d_b54, eq131_e1677_q,) = {
    if ((!s.b[570]) && s.b[573]) {
        let eq131_e1673: f64 = (p.p251 * s.v[228]);
        let eq131_e1674_q: f64 = eq131_e1673;
        let eq131_e1675: f64 = (p.p7 * eq131_e1673);
        let eq131_e1675_d_n0: f64 = (p.p7 * (p.p251 * s.dn[228][0]));
        let eq131_e1675_d_n1: f64 = (p.p7 * (p.p251 * s.dn[228][1]));
        let eq131_e1675_d_n2: f64 = (p.p7 * (p.p251 * s.dn[228][2]));
        let eq131_e1675_d_n3: f64 = (p.p7 * (p.p251 * s.dn[228][3]));
        let eq131_e1675_d_n4: f64 = (p.p7 * (p.p251 * s.dn[228][4]));
        let eq131_e1675_d_n5: f64 = (p.p7 * (p.p251 * s.dn[228][5]));
        let eq131_e1675_d_n6: f64 = (p.p7 * (p.p251 * s.dn[228][6]));
        let eq131_e1675_d_n7: f64 = (p.p7 * (p.p251 * s.dn[228][7]));
        let eq131_e1675_d_n8: f64 = (p.p7 * (p.p251 * s.dn[228][8]));
        let eq131_e1675_d_n9: f64 = (p.p7 * (p.p251 * s.dn[228][9]));
        let eq131_e1675_d_n10: f64 = (p.p7 * (p.p251 * s.dn[228][10]));
        let eq131_e1675_d_n11: f64 = (p.p7 * (p.p251 * s.dn[228][11]));
        let eq131_e1675_d_n12: f64 = (p.p7 * (p.p251 * s.dn[228][12]));
        let eq131_e1675_d_n13: f64 = (p.p7 * (p.p251 * s.dn[228][13]));
        let eq131_e1675_d_n14: f64 = (p.p7 * (p.p251 * s.dn[228][14]));
        let eq131_e1675_d_n15: f64 = (p.p7 * (p.p251 * s.dn[228][15]));
        let eq131_e1675_d_n16: f64 = (p.p7 * (p.p251 * s.dn[228][16]));
        let eq131_e1675_d_n17: f64 = (p.p7 * (p.p251 * s.dn[228][17]));
        let eq131_e1675_d_n18: f64 = (p.p7 * (p.p251 * s.dn[228][18]));
        let eq131_e1675_d_n19: f64 = (p.p7 * (p.p251 * s.dn[228][19]));
        let eq131_e1675_d_n20: f64 = (p.p7 * (p.p251 * s.dn[228][20]));
        let eq131_e1675_d_n21: f64 = (p.p7 * (p.p251 * s.dn[228][21]));
        let eq131_e1675_d_n22: f64 = (p.p7 * (p.p251 * s.dn[228][22]));
        let eq131_e1675_d_b0: f64 = (p.p7 * (p.p251 * s.db[228][0]));
        let eq131_e1675_d_b1: f64 = (p.p7 * (p.p251 * s.db[228][1]));
        let eq131_e1675_d_b2: f64 = (p.p7 * (p.p251 * s.db[228][2]));
        let eq131_e1675_d_b3: f64 = (p.p7 * (p.p251 * s.db[228][3]));
        let eq131_e1675_d_b4: f64 = (p.p7 * (p.p251 * s.db[228][4]));
        let eq131_e1675_d_b5: f64 = (p.p7 * (p.p251 * s.db[228][5]));
        let eq131_e1675_d_b6: f64 = (p.p7 * (p.p251 * s.db[228][6]));
        let eq131_e1675_d_b7: f64 = (p.p7 * (p.p251 * s.db[228][7]));
        let eq131_e1675_d_b8: f64 = (p.p7 * (p.p251 * s.db[228][8]));
        let eq131_e1675_d_b9: f64 = (p.p7 * (p.p251 * s.db[228][9]));
        let eq131_e1675_d_b10: f64 = (p.p7 * (p.p251 * s.db[228][10]));
        let eq131_e1675_d_b11: f64 = (p.p7 * (p.p251 * s.db[228][11]));
        let eq131_e1675_d_b12: f64 = (p.p7 * (p.p251 * s.db[228][12]));
        let eq131_e1675_d_b13: f64 = (p.p7 * (p.p251 * s.db[228][13]));
        let eq131_e1675_d_b14: f64 = (p.p7 * (p.p251 * s.db[228][14]));
        let eq131_e1675_d_b15: f64 = (p.p7 * (p.p251 * s.db[228][15]));
        let eq131_e1675_d_b16: f64 = (p.p7 * (p.p251 * s.db[228][16]));
        let eq131_e1675_d_b17: f64 = (p.p7 * (p.p251 * s.db[228][17]));
        let eq131_e1675_d_b18: f64 = (p.p7 * (p.p251 * s.db[228][18]));
        let eq131_e1675_d_b19: f64 = (p.p7 * (p.p251 * s.db[228][19]));
        let eq131_e1675_d_b20: f64 = (p.p7 * (p.p251 * s.db[228][20]));
        let eq131_e1675_d_b21: f64 = (p.p7 * (p.p251 * s.db[228][21]));
        let eq131_e1675_d_b22: f64 = (p.p7 * (p.p251 * s.db[228][22]));
        let eq131_e1675_d_b23: f64 = (p.p7 * (p.p251 * s.db[228][23]));
        let eq131_e1675_d_b24: f64 = (p.p7 * (p.p251 * s.db[228][24]));
        let eq131_e1675_d_b25: f64 = (p.p7 * (p.p251 * s.db[228][25]));
        let eq131_e1675_d_b26: f64 = (p.p7 * (p.p251 * s.db[228][26]));
        let eq131_e1675_d_b27: f64 = (p.p7 * (p.p251 * s.db[228][27]));
        let eq131_e1675_d_b28: f64 = (p.p7 * (p.p251 * s.db[228][28]));
        let eq131_e1675_d_b29: f64 = (p.p7 * (p.p251 * s.db[228][29]));
        let eq131_e1675_d_b30: f64 = (p.p7 * (p.p251 * s.db[228][30]));
        let eq131_e1675_d_b31: f64 = (p.p7 * (p.p251 * s.db[228][31]));
        let eq131_e1675_d_b32: f64 = (p.p7 * (p.p251 * s.db[228][32]));
        let eq131_e1675_d_b33: f64 = (p.p7 * (p.p251 * s.db[228][33]));
        let eq131_e1675_d_b34: f64 = (p.p7 * (p.p251 * s.db[228][34]));
        let eq131_e1675_d_b35: f64 = (p.p7 * (p.p251 * s.db[228][35]));
        let eq131_e1675_d_b36: f64 = (p.p7 * (p.p251 * s.db[228][36]));
        let eq131_e1675_d_b37: f64 = (p.p7 * (p.p251 * s.db[228][37]));
        let eq131_e1675_d_b38: f64 = (p.p7 * (p.p251 * s.db[228][38]));
        let eq131_e1675_d_b39: f64 = (p.p7 * (p.p251 * s.db[228][39]));
        let eq131_e1675_d_b40: f64 = (p.p7 * (p.p251 * s.db[228][40]));
        let eq131_e1675_d_b41: f64 = (p.p7 * (p.p251 * s.db[228][41]));
        let eq131_e1675_d_b42: f64 = (p.p7 * (p.p251 * s.db[228][42]));
        let eq131_e1675_d_b43: f64 = (p.p7 * (p.p251 * s.db[228][43]));
        let eq131_e1675_d_b44: f64 = (p.p7 * (p.p251 * s.db[228][44]));
        let eq131_e1675_d_b45: f64 = (p.p7 * (p.p251 * s.db[228][45]));
        let eq131_e1675_d_b46: f64 = (p.p7 * (p.p251 * s.db[228][46]));
        let eq131_e1675_d_b47: f64 = (p.p7 * (p.p251 * s.db[228][47]));
        let eq131_e1675_d_b48: f64 = (p.p7 * (p.p251 * s.db[228][48]));
        let eq131_e1675_d_b49: f64 = (p.p7 * (p.p251 * s.db[228][49]));
        let eq131_e1675_d_b50: f64 = (p.p7 * (p.p251 * s.db[228][50]));
        let eq131_e1675_d_b51: f64 = (p.p7 * (p.p251 * s.db[228][51]));
        let eq131_e1675_d_b52: f64 = (p.p7 * (p.p251 * s.db[228][52]));
        let eq131_e1675_d_b53: f64 = (p.p7 * (p.p251 * s.db[228][53]));
        let eq131_e1675_d_b54: f64 = (p.p7 * (p.p251 * s.db[228][54]));
        let eq131_e1675_q: f64 = (p.p7 * eq131_e1674_q);
        (eq131_e1675, eq131_e1675_d_n0, eq131_e1675_d_n1, eq131_e1675_d_n2, eq131_e1675_d_n3, eq131_e1675_d_n4, eq131_e1675_d_n5, eq131_e1675_d_n6, eq131_e1675_d_n7, eq131_e1675_d_n8, eq131_e1675_d_n9, eq131_e1675_d_n10, eq131_e1675_d_n11, eq131_e1675_d_n12, eq131_e1675_d_n13, eq131_e1675_d_n14, eq131_e1675_d_n15, eq131_e1675_d_n16, eq131_e1675_d_n17, eq131_e1675_d_n18, eq131_e1675_d_n19, eq131_e1675_d_n20, eq131_e1675_d_n21, eq131_e1675_d_n22, eq131_e1675_d_b0, eq131_e1675_d_b1, eq131_e1675_d_b2, eq131_e1675_d_b3, eq131_e1675_d_b4, eq131_e1675_d_b5, eq131_e1675_d_b6, eq131_e1675_d_b7, eq131_e1675_d_b8, eq131_e1675_d_b9, eq131_e1675_d_b10, eq131_e1675_d_b11, eq131_e1675_d_b12, eq131_e1675_d_b13, eq131_e1675_d_b14, eq131_e1675_d_b15, eq131_e1675_d_b16, eq131_e1675_d_b17, eq131_e1675_d_b18, eq131_e1675_d_b19, eq131_e1675_d_b20, eq131_e1675_d_b21, eq131_e1675_d_b22, eq131_e1675_d_b23, eq131_e1675_d_b24, eq131_e1675_d_b25, eq131_e1675_d_b26, eq131_e1675_d_b27, eq131_e1675_d_b28, eq131_e1675_d_b29, eq131_e1675_d_b30, eq131_e1675_d_b31, eq131_e1675_d_b32, eq131_e1675_d_b33, eq131_e1675_d_b34, eq131_e1675_d_b35, eq131_e1675_d_b36, eq131_e1675_d_b37, eq131_e1675_d_b38, eq131_e1675_d_b39, eq131_e1675_d_b40, eq131_e1675_d_b41, eq131_e1675_d_b42, eq131_e1675_d_b43, eq131_e1675_d_b44, eq131_e1675_d_b45, eq131_e1675_d_b46, eq131_e1675_d_b47, eq131_e1675_d_b48, eq131_e1675_d_b49, eq131_e1675_d_b50, eq131_e1675_d_b51, eq131_e1675_d_b52, eq131_e1675_d_b53, eq131_e1675_d_b54, eq131_e1675_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_reactive_node_derivatives: [f64; 23] = [eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22];
        let eq131_reactive_branch_derivatives: [f64; 55] = [eq131_e1677_d_b0, eq131_e1677_d_b1, eq131_e1677_d_b2, eq131_e1677_d_b3, eq131_e1677_d_b4, eq131_e1677_d_b5, eq131_e1677_d_b6, eq131_e1677_d_b7, eq131_e1677_d_b8, eq131_e1677_d_b9, eq131_e1677_d_b10, eq131_e1677_d_b11, eq131_e1677_d_b12, eq131_e1677_d_b13, eq131_e1677_d_b14, eq131_e1677_d_b15, eq131_e1677_d_b16, eq131_e1677_d_b17, eq131_e1677_d_b18, eq131_e1677_d_b19, eq131_e1677_d_b20, eq131_e1677_d_b21, eq131_e1677_d_b22, eq131_e1677_d_b23, eq131_e1677_d_b24, eq131_e1677_d_b25, eq131_e1677_d_b26, eq131_e1677_d_b27, eq131_e1677_d_b28, eq131_e1677_d_b29, eq131_e1677_d_b30, eq131_e1677_d_b31, eq131_e1677_d_b32, eq131_e1677_d_b33, eq131_e1677_d_b34, eq131_e1677_d_b35, eq131_e1677_d_b36, eq131_e1677_d_b37, eq131_e1677_d_b38, eq131_e1677_d_b39, eq131_e1677_d_b40, eq131_e1677_d_b41, eq131_e1677_d_b42, eq131_e1677_d_b43, eq131_e1677_d_b44, eq131_e1677_d_b45, eq131_e1677_d_b46, eq131_e1677_d_b47, eq131_e1677_d_b48, eq131_e1677_d_b49, eq131_e1677_d_b50, eq131_e1677_d_b51, eq131_e1677_d_b52, eq131_e1677_d_b53, eq131_e1677_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq131_reactive_node_derivatives,
            branches,
            &eq131_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq132_e1686, eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22, eq132_e1686_d_b0, eq132_e1686_d_b1, eq132_e1686_d_b2, eq132_e1686_d_b3, eq132_e1686_d_b4, eq132_e1686_d_b5, eq132_e1686_d_b6, eq132_e1686_d_b7, eq132_e1686_d_b8, eq132_e1686_d_b9, eq132_e1686_d_b10, eq132_e1686_d_b11, eq132_e1686_d_b12, eq132_e1686_d_b13, eq132_e1686_d_b14, eq132_e1686_d_b15, eq132_e1686_d_b16, eq132_e1686_d_b17, eq132_e1686_d_b18, eq132_e1686_d_b19, eq132_e1686_d_b20, eq132_e1686_d_b21, eq132_e1686_d_b22, eq132_e1686_d_b23, eq132_e1686_d_b24, eq132_e1686_d_b25, eq132_e1686_d_b26, eq132_e1686_d_b27, eq132_e1686_d_b28, eq132_e1686_d_b29, eq132_e1686_d_b30, eq132_e1686_d_b31, eq132_e1686_d_b32, eq132_e1686_d_b33, eq132_e1686_d_b34, eq132_e1686_d_b35, eq132_e1686_d_b36, eq132_e1686_d_b37, eq132_e1686_d_b38, eq132_e1686_d_b39, eq132_e1686_d_b40, eq132_e1686_d_b41, eq132_e1686_d_b42, eq132_e1686_d_b43, eq132_e1686_d_b44, eq132_e1686_d_b45, eq132_e1686_d_b46, eq132_e1686_d_b47, eq132_e1686_d_b48, eq132_e1686_d_b49, eq132_e1686_d_b50, eq132_e1686_d_b51, eq132_e1686_d_b52, eq132_e1686_d_b53, eq132_e1686_d_b54, eq132_e1686_q,) = {
    if (s.b[575] && s.b[576]) {
        let eq132_e1683_q: f64 = s.v[241];
        let eq132_e1684: f64 = (p.p7 * s.v[241]);
        let eq132_e1684_q: f64 = (p.p7 * eq132_e1683_q);
        (eq132_e1684, (p.p7 * s.dn[241][0]), (p.p7 * s.dn[241][1]), (p.p7 * s.dn[241][2]), (p.p7 * s.dn[241][3]), (p.p7 * s.dn[241][4]), (p.p7 * s.dn[241][5]), (p.p7 * s.dn[241][6]), (p.p7 * s.dn[241][7]), (p.p7 * s.dn[241][8]), (p.p7 * s.dn[241][9]), (p.p7 * s.dn[241][10]), (p.p7 * s.dn[241][11]), (p.p7 * s.dn[241][12]), (p.p7 * s.dn[241][13]), (p.p7 * s.dn[241][14]), (p.p7 * s.dn[241][15]), (p.p7 * s.dn[241][16]), (p.p7 * s.dn[241][17]), (p.p7 * s.dn[241][18]), (p.p7 * s.dn[241][19]), (p.p7 * s.dn[241][20]), (p.p7 * s.dn[241][21]), (p.p7 * s.dn[241][22]), (p.p7 * s.db[241][0]), (p.p7 * s.db[241][1]), (p.p7 * s.db[241][2]), (p.p7 * s.db[241][3]), (p.p7 * s.db[241][4]), (p.p7 * s.db[241][5]), (p.p7 * s.db[241][6]), (p.p7 * s.db[241][7]), (p.p7 * s.db[241][8]), (p.p7 * s.db[241][9]), (p.p7 * s.db[241][10]), (p.p7 * s.db[241][11]), (p.p7 * s.db[241][12]), (p.p7 * s.db[241][13]), (p.p7 * s.db[241][14]), (p.p7 * s.db[241][15]), (p.p7 * s.db[241][16]), (p.p7 * s.db[241][17]), (p.p7 * s.db[241][18]), (p.p7 * s.db[241][19]), (p.p7 * s.db[241][20]), (p.p7 * s.db[241][21]), (p.p7 * s.db[241][22]), (p.p7 * s.db[241][23]), (p.p7 * s.db[241][24]), (p.p7 * s.db[241][25]), (p.p7 * s.db[241][26]), (p.p7 * s.db[241][27]), (p.p7 * s.db[241][28]), (p.p7 * s.db[241][29]), (p.p7 * s.db[241][30]), (p.p7 * s.db[241][31]), (p.p7 * s.db[241][32]), (p.p7 * s.db[241][33]), (p.p7 * s.db[241][34]), (p.p7 * s.db[241][35]), (p.p7 * s.db[241][36]), (p.p7 * s.db[241][37]), (p.p7 * s.db[241][38]), (p.p7 * s.db[241][39]), (p.p7 * s.db[241][40]), (p.p7 * s.db[241][41]), (p.p7 * s.db[241][42]), (p.p7 * s.db[241][43]), (p.p7 * s.db[241][44]), (p.p7 * s.db[241][45]), (p.p7 * s.db[241][46]), (p.p7 * s.db[241][47]), (p.p7 * s.db[241][48]), (p.p7 * s.db[241][49]), (p.p7 * s.db[241][50]), (p.p7 * s.db[241][51]), (p.p7 * s.db[241][52]), (p.p7 * s.db[241][53]), (p.p7 * s.db[241][54]), eq132_e1684_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq132_reactive_node_derivatives: [f64; 23] = [eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22];
        let eq132_reactive_branch_derivatives: [f64; 55] = [eq132_e1686_d_b0, eq132_e1686_d_b1, eq132_e1686_d_b2, eq132_e1686_d_b3, eq132_e1686_d_b4, eq132_e1686_d_b5, eq132_e1686_d_b6, eq132_e1686_d_b7, eq132_e1686_d_b8, eq132_e1686_d_b9, eq132_e1686_d_b10, eq132_e1686_d_b11, eq132_e1686_d_b12, eq132_e1686_d_b13, eq132_e1686_d_b14, eq132_e1686_d_b15, eq132_e1686_d_b16, eq132_e1686_d_b17, eq132_e1686_d_b18, eq132_e1686_d_b19, eq132_e1686_d_b20, eq132_e1686_d_b21, eq132_e1686_d_b22, eq132_e1686_d_b23, eq132_e1686_d_b24, eq132_e1686_d_b25, eq132_e1686_d_b26, eq132_e1686_d_b27, eq132_e1686_d_b28, eq132_e1686_d_b29, eq132_e1686_d_b30, eq132_e1686_d_b31, eq132_e1686_d_b32, eq132_e1686_d_b33, eq132_e1686_d_b34, eq132_e1686_d_b35, eq132_e1686_d_b36, eq132_e1686_d_b37, eq132_e1686_d_b38, eq132_e1686_d_b39, eq132_e1686_d_b40, eq132_e1686_d_b41, eq132_e1686_d_b42, eq132_e1686_d_b43, eq132_e1686_d_b44, eq132_e1686_d_b45, eq132_e1686_d_b46, eq132_e1686_d_b47, eq132_e1686_d_b48, eq132_e1686_d_b49, eq132_e1686_d_b50, eq132_e1686_d_b51, eq132_e1686_d_b52, eq132_e1686_d_b53, eq132_e1686_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[19]),
            nodes,
            &eq132_reactive_node_derivatives,
            branches,
            &eq132_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq133_e1697, eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22, eq133_e1697_d_b0, eq133_e1697_d_b1, eq133_e1697_d_b2, eq133_e1697_d_b3, eq133_e1697_d_b4, eq133_e1697_d_b5, eq133_e1697_d_b6, eq133_e1697_d_b7, eq133_e1697_d_b8, eq133_e1697_d_b9, eq133_e1697_d_b10, eq133_e1697_d_b11, eq133_e1697_d_b12, eq133_e1697_d_b13, eq133_e1697_d_b14, eq133_e1697_d_b15, eq133_e1697_d_b16, eq133_e1697_d_b17, eq133_e1697_d_b18, eq133_e1697_d_b19, eq133_e1697_d_b20, eq133_e1697_d_b21, eq133_e1697_d_b22, eq133_e1697_d_b23, eq133_e1697_d_b24, eq133_e1697_d_b25, eq133_e1697_d_b26, eq133_e1697_d_b27, eq133_e1697_d_b28, eq133_e1697_d_b29, eq133_e1697_d_b30, eq133_e1697_d_b31, eq133_e1697_d_b32, eq133_e1697_d_b33, eq133_e1697_d_b34, eq133_e1697_d_b35, eq133_e1697_d_b36, eq133_e1697_d_b37, eq133_e1697_d_b38, eq133_e1697_d_b39, eq133_e1697_d_b40, eq133_e1697_d_b41, eq133_e1697_d_b42, eq133_e1697_d_b43, eq133_e1697_d_b44, eq133_e1697_d_b45, eq133_e1697_d_b46, eq133_e1697_d_b47, eq133_e1697_d_b48, eq133_e1697_d_b49, eq133_e1697_d_b50, eq133_e1697_d_b51, eq133_e1697_d_b52, eq133_e1697_d_b53, eq133_e1697_d_b54, eq133_e1697_q,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq133_e1694_q: f64 = s.v[240];
        let eq133_e1695: f64 = (p.p7 * s.v[240]);
        let eq133_e1695_q: f64 = (p.p7 * eq133_e1694_q);
        (eq133_e1695, (p.p7 * s.dn[240][0]), (p.p7 * s.dn[240][1]), (p.p7 * s.dn[240][2]), (p.p7 * s.dn[240][3]), (p.p7 * s.dn[240][4]), (p.p7 * s.dn[240][5]), (p.p7 * s.dn[240][6]), (p.p7 * s.dn[240][7]), (p.p7 * s.dn[240][8]), (p.p7 * s.dn[240][9]), (p.p7 * s.dn[240][10]), (p.p7 * s.dn[240][11]), (p.p7 * s.dn[240][12]), (p.p7 * s.dn[240][13]), (p.p7 * s.dn[240][14]), (p.p7 * s.dn[240][15]), (p.p7 * s.dn[240][16]), (p.p7 * s.dn[240][17]), (p.p7 * s.dn[240][18]), (p.p7 * s.dn[240][19]), (p.p7 * s.dn[240][20]), (p.p7 * s.dn[240][21]), (p.p7 * s.dn[240][22]), (p.p7 * s.db[240][0]), (p.p7 * s.db[240][1]), (p.p7 * s.db[240][2]), (p.p7 * s.db[240][3]), (p.p7 * s.db[240][4]), (p.p7 * s.db[240][5]), (p.p7 * s.db[240][6]), (p.p7 * s.db[240][7]), (p.p7 * s.db[240][8]), (p.p7 * s.db[240][9]), (p.p7 * s.db[240][10]), (p.p7 * s.db[240][11]), (p.p7 * s.db[240][12]), (p.p7 * s.db[240][13]), (p.p7 * s.db[240][14]), (p.p7 * s.db[240][15]), (p.p7 * s.db[240][16]), (p.p7 * s.db[240][17]), (p.p7 * s.db[240][18]), (p.p7 * s.db[240][19]), (p.p7 * s.db[240][20]), (p.p7 * s.db[240][21]), (p.p7 * s.db[240][22]), (p.p7 * s.db[240][23]), (p.p7 * s.db[240][24]), (p.p7 * s.db[240][25]), (p.p7 * s.db[240][26]), (p.p7 * s.db[240][27]), (p.p7 * s.db[240][28]), (p.p7 * s.db[240][29]), (p.p7 * s.db[240][30]), (p.p7 * s.db[240][31]), (p.p7 * s.db[240][32]), (p.p7 * s.db[240][33]), (p.p7 * s.db[240][34]), (p.p7 * s.db[240][35]), (p.p7 * s.db[240][36]), (p.p7 * s.db[240][37]), (p.p7 * s.db[240][38]), (p.p7 * s.db[240][39]), (p.p7 * s.db[240][40]), (p.p7 * s.db[240][41]), (p.p7 * s.db[240][42]), (p.p7 * s.db[240][43]), (p.p7 * s.db[240][44]), (p.p7 * s.db[240][45]), (p.p7 * s.db[240][46]), (p.p7 * s.db[240][47]), (p.p7 * s.db[240][48]), (p.p7 * s.db[240][49]), (p.p7 * s.db[240][50]), (p.p7 * s.db[240][51]), (p.p7 * s.db[240][52]), (p.p7 * s.db[240][53]), (p.p7 * s.db[240][54]), eq133_e1695_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq133_reactive_node_derivatives: [f64; 23] = [eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22];
        let eq133_reactive_branch_derivatives: [f64; 55] = [eq133_e1697_d_b0, eq133_e1697_d_b1, eq133_e1697_d_b2, eq133_e1697_d_b3, eq133_e1697_d_b4, eq133_e1697_d_b5, eq133_e1697_d_b6, eq133_e1697_d_b7, eq133_e1697_d_b8, eq133_e1697_d_b9, eq133_e1697_d_b10, eq133_e1697_d_b11, eq133_e1697_d_b12, eq133_e1697_d_b13, eq133_e1697_d_b14, eq133_e1697_d_b15, eq133_e1697_d_b16, eq133_e1697_d_b17, eq133_e1697_d_b18, eq133_e1697_d_b19, eq133_e1697_d_b20, eq133_e1697_d_b21, eq133_e1697_d_b22, eq133_e1697_d_b23, eq133_e1697_d_b24, eq133_e1697_d_b25, eq133_e1697_d_b26, eq133_e1697_d_b27, eq133_e1697_d_b28, eq133_e1697_d_b29, eq133_e1697_d_b30, eq133_e1697_d_b31, eq133_e1697_d_b32, eq133_e1697_d_b33, eq133_e1697_d_b34, eq133_e1697_d_b35, eq133_e1697_d_b36, eq133_e1697_d_b37, eq133_e1697_d_b38, eq133_e1697_d_b39, eq133_e1697_d_b40, eq133_e1697_d_b41, eq133_e1697_d_b42, eq133_e1697_d_b43, eq133_e1697_d_b44, eq133_e1697_d_b45, eq133_e1697_d_b46, eq133_e1697_d_b47, eq133_e1697_d_b48, eq133_e1697_d_b49, eq133_e1697_d_b50, eq133_e1697_d_b51, eq133_e1697_d_b52, eq133_e1697_d_b53, eq133_e1697_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            nodes,
            &eq133_reactive_node_derivatives,
            branches,
            &eq133_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq134_e1710, eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22, eq134_e1710_d_b0, eq134_e1710_d_b1, eq134_e1710_d_b2, eq134_e1710_d_b3, eq134_e1710_d_b4, eq134_e1710_d_b5, eq134_e1710_d_b6, eq134_e1710_d_b7, eq134_e1710_d_b8, eq134_e1710_d_b9, eq134_e1710_d_b10, eq134_e1710_d_b11, eq134_e1710_d_b12, eq134_e1710_d_b13, eq134_e1710_d_b14, eq134_e1710_d_b15, eq134_e1710_d_b16, eq134_e1710_d_b17, eq134_e1710_d_b18, eq134_e1710_d_b19, eq134_e1710_d_b20, eq134_e1710_d_b21, eq134_e1710_d_b22, eq134_e1710_d_b23, eq134_e1710_d_b24, eq134_e1710_d_b25, eq134_e1710_d_b26, eq134_e1710_d_b27, eq134_e1710_d_b28, eq134_e1710_d_b29, eq134_e1710_d_b30, eq134_e1710_d_b31, eq134_e1710_d_b32, eq134_e1710_d_b33, eq134_e1710_d_b34, eq134_e1710_d_b35, eq134_e1710_d_b36, eq134_e1710_d_b37, eq134_e1710_d_b38, eq134_e1710_d_b39, eq134_e1710_d_b40, eq134_e1710_d_b41, eq134_e1710_d_b42, eq134_e1710_d_b43, eq134_e1710_d_b44, eq134_e1710_d_b45, eq134_e1710_d_b46, eq134_e1710_d_b47, eq134_e1710_d_b48, eq134_e1710_d_b49, eq134_e1710_d_b50, eq134_e1710_d_b51, eq134_e1710_d_b52, eq134_e1710_d_b53, eq134_e1710_d_b54, eq134_e1710_q,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq134_e1705_q: f64 = s.v[240];
        let eq134_e1706: f64 = (p.p7 * s.v[240]);
        let eq134_e1706_q: f64 = (p.p7 * eq134_e1705_q);
        let eq134_e1708: f64 = (eq134_e1706 * p.p246);
        let eq134_e1708_q: f64 = (eq134_e1706_q * p.p246);
        (eq134_e1708, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq134_e1708_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq134_reactive_node_derivatives: [f64; 23] = [eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22];
        let eq134_reactive_branch_derivatives: [f64; 55] = [eq134_e1710_d_b0, eq134_e1710_d_b1, eq134_e1710_d_b2, eq134_e1710_d_b3, eq134_e1710_d_b4, eq134_e1710_d_b5, eq134_e1710_d_b6, eq134_e1710_d_b7, eq134_e1710_d_b8, eq134_e1710_d_b9, eq134_e1710_d_b10, eq134_e1710_d_b11, eq134_e1710_d_b12, eq134_e1710_d_b13, eq134_e1710_d_b14, eq134_e1710_d_b15, eq134_e1710_d_b16, eq134_e1710_d_b17, eq134_e1710_d_b18, eq134_e1710_d_b19, eq134_e1710_d_b20, eq134_e1710_d_b21, eq134_e1710_d_b22, eq134_e1710_d_b23, eq134_e1710_d_b24, eq134_e1710_d_b25, eq134_e1710_d_b26, eq134_e1710_d_b27, eq134_e1710_d_b28, eq134_e1710_d_b29, eq134_e1710_d_b30, eq134_e1710_d_b31, eq134_e1710_d_b32, eq134_e1710_d_b33, eq134_e1710_d_b34, eq134_e1710_d_b35, eq134_e1710_d_b36, eq134_e1710_d_b37, eq134_e1710_d_b38, eq134_e1710_d_b39, eq134_e1710_d_b40, eq134_e1710_d_b41, eq134_e1710_d_b42, eq134_e1710_d_b43, eq134_e1710_d_b44, eq134_e1710_d_b45, eq134_e1710_d_b46, eq134_e1710_d_b47, eq134_e1710_d_b48, eq134_e1710_d_b49, eq134_e1710_d_b50, eq134_e1710_d_b51, eq134_e1710_d_b52, eq134_e1710_d_b53, eq134_e1710_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            nodes,
            &eq134_reactive_node_derivatives,
            branches,
            &eq134_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq135_e1722, eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22, eq135_e1722_d_b0, eq135_e1722_d_b1, eq135_e1722_d_b2, eq135_e1722_d_b3, eq135_e1722_d_b4, eq135_e1722_d_b5, eq135_e1722_d_b6, eq135_e1722_d_b7, eq135_e1722_d_b8, eq135_e1722_d_b9, eq135_e1722_d_b10, eq135_e1722_d_b11, eq135_e1722_d_b12, eq135_e1722_d_b13, eq135_e1722_d_b14, eq135_e1722_d_b15, eq135_e1722_d_b16, eq135_e1722_d_b17, eq135_e1722_d_b18, eq135_e1722_d_b19, eq135_e1722_d_b20, eq135_e1722_d_b21, eq135_e1722_d_b22, eq135_e1722_d_b23, eq135_e1722_d_b24, eq135_e1722_d_b25, eq135_e1722_d_b26, eq135_e1722_d_b27, eq135_e1722_d_b28, eq135_e1722_d_b29, eq135_e1722_d_b30, eq135_e1722_d_b31, eq135_e1722_d_b32, eq135_e1722_d_b33, eq135_e1722_d_b34, eq135_e1722_d_b35, eq135_e1722_d_b36, eq135_e1722_d_b37, eq135_e1722_d_b38, eq135_e1722_d_b39, eq135_e1722_d_b40, eq135_e1722_d_b41, eq135_e1722_d_b42, eq135_e1722_d_b43, eq135_e1722_d_b44, eq135_e1722_d_b45, eq135_e1722_d_b46, eq135_e1722_d_b47, eq135_e1722_d_b48, eq135_e1722_d_b49, eq135_e1722_d_b50, eq135_e1722_d_b51, eq135_e1722_d_b52, eq135_e1722_d_b53, eq135_e1722_d_b54, eq135_e1722_q,) = {
    if ((s.b[575] && s.b[576]) && (!s.b[577])) {
        let eq135_e1719_q: f64 = s.v[240];
        let eq135_e1720: f64 = (p.p7 * s.v[240]);
        let eq135_e1720_q: f64 = (p.p7 * eq135_e1719_q);
        (eq135_e1720, (p.p7 * s.dn[240][0]), (p.p7 * s.dn[240][1]), (p.p7 * s.dn[240][2]), (p.p7 * s.dn[240][3]), (p.p7 * s.dn[240][4]), (p.p7 * s.dn[240][5]), (p.p7 * s.dn[240][6]), (p.p7 * s.dn[240][7]), (p.p7 * s.dn[240][8]), (p.p7 * s.dn[240][9]), (p.p7 * s.dn[240][10]), (p.p7 * s.dn[240][11]), (p.p7 * s.dn[240][12]), (p.p7 * s.dn[240][13]), (p.p7 * s.dn[240][14]), (p.p7 * s.dn[240][15]), (p.p7 * s.dn[240][16]), (p.p7 * s.dn[240][17]), (p.p7 * s.dn[240][18]), (p.p7 * s.dn[240][19]), (p.p7 * s.dn[240][20]), (p.p7 * s.dn[240][21]), (p.p7 * s.dn[240][22]), (p.p7 * s.db[240][0]), (p.p7 * s.db[240][1]), (p.p7 * s.db[240][2]), (p.p7 * s.db[240][3]), (p.p7 * s.db[240][4]), (p.p7 * s.db[240][5]), (p.p7 * s.db[240][6]), (p.p7 * s.db[240][7]), (p.p7 * s.db[240][8]), (p.p7 * s.db[240][9]), (p.p7 * s.db[240][10]), (p.p7 * s.db[240][11]), (p.p7 * s.db[240][12]), (p.p7 * s.db[240][13]), (p.p7 * s.db[240][14]), (p.p7 * s.db[240][15]), (p.p7 * s.db[240][16]), (p.p7 * s.db[240][17]), (p.p7 * s.db[240][18]), (p.p7 * s.db[240][19]), (p.p7 * s.db[240][20]), (p.p7 * s.db[240][21]), (p.p7 * s.db[240][22]), (p.p7 * s.db[240][23]), (p.p7 * s.db[240][24]), (p.p7 * s.db[240][25]), (p.p7 * s.db[240][26]), (p.p7 * s.db[240][27]), (p.p7 * s.db[240][28]), (p.p7 * s.db[240][29]), (p.p7 * s.db[240][30]), (p.p7 * s.db[240][31]), (p.p7 * s.db[240][32]), (p.p7 * s.db[240][33]), (p.p7 * s.db[240][34]), (p.p7 * s.db[240][35]), (p.p7 * s.db[240][36]), (p.p7 * s.db[240][37]), (p.p7 * s.db[240][38]), (p.p7 * s.db[240][39]), (p.p7 * s.db[240][40]), (p.p7 * s.db[240][41]), (p.p7 * s.db[240][42]), (p.p7 * s.db[240][43]), (p.p7 * s.db[240][44]), (p.p7 * s.db[240][45]), (p.p7 * s.db[240][46]), (p.p7 * s.db[240][47]), (p.p7 * s.db[240][48]), (p.p7 * s.db[240][49]), (p.p7 * s.db[240][50]), (p.p7 * s.db[240][51]), (p.p7 * s.db[240][52]), (p.p7 * s.db[240][53]), (p.p7 * s.db[240][54]), eq135_e1720_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_reactive_node_derivatives: [f64; 23] = [eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22];
        let eq135_reactive_branch_derivatives: [f64; 55] = [eq135_e1722_d_b0, eq135_e1722_d_b1, eq135_e1722_d_b2, eq135_e1722_d_b3, eq135_e1722_d_b4, eq135_e1722_d_b5, eq135_e1722_d_b6, eq135_e1722_d_b7, eq135_e1722_d_b8, eq135_e1722_d_b9, eq135_e1722_d_b10, eq135_e1722_d_b11, eq135_e1722_d_b12, eq135_e1722_d_b13, eq135_e1722_d_b14, eq135_e1722_d_b15, eq135_e1722_d_b16, eq135_e1722_d_b17, eq135_e1722_d_b18, eq135_e1722_d_b19, eq135_e1722_d_b20, eq135_e1722_d_b21, eq135_e1722_d_b22, eq135_e1722_d_b23, eq135_e1722_d_b24, eq135_e1722_d_b25, eq135_e1722_d_b26, eq135_e1722_d_b27, eq135_e1722_d_b28, eq135_e1722_d_b29, eq135_e1722_d_b30, eq135_e1722_d_b31, eq135_e1722_d_b32, eq135_e1722_d_b33, eq135_e1722_d_b34, eq135_e1722_d_b35, eq135_e1722_d_b36, eq135_e1722_d_b37, eq135_e1722_d_b38, eq135_e1722_d_b39, eq135_e1722_d_b40, eq135_e1722_d_b41, eq135_e1722_d_b42, eq135_e1722_d_b43, eq135_e1722_d_b44, eq135_e1722_d_b45, eq135_e1722_d_b46, eq135_e1722_d_b47, eq135_e1722_d_b48, eq135_e1722_d_b49, eq135_e1722_d_b50, eq135_e1722_d_b51, eq135_e1722_d_b52, eq135_e1722_d_b53, eq135_e1722_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            nodes,
            &eq135_reactive_node_derivatives,
            branches,
            &eq135_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq136_e1736, eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22, eq136_e1736_d_b0, eq136_e1736_d_b1, eq136_e1736_d_b2, eq136_e1736_d_b3, eq136_e1736_d_b4, eq136_e1736_d_b5, eq136_e1736_d_b6, eq136_e1736_d_b7, eq136_e1736_d_b8, eq136_e1736_d_b9, eq136_e1736_d_b10, eq136_e1736_d_b11, eq136_e1736_d_b12, eq136_e1736_d_b13, eq136_e1736_d_b14, eq136_e1736_d_b15, eq136_e1736_d_b16, eq136_e1736_d_b17, eq136_e1736_d_b18, eq136_e1736_d_b19, eq136_e1736_d_b20, eq136_e1736_d_b21, eq136_e1736_d_b22, eq136_e1736_d_b23, eq136_e1736_d_b24, eq136_e1736_d_b25, eq136_e1736_d_b26, eq136_e1736_d_b27, eq136_e1736_d_b28, eq136_e1736_d_b29, eq136_e1736_d_b30, eq136_e1736_d_b31, eq136_e1736_d_b32, eq136_e1736_d_b33, eq136_e1736_d_b34, eq136_e1736_d_b35, eq136_e1736_d_b36, eq136_e1736_d_b37, eq136_e1736_d_b38, eq136_e1736_d_b39, eq136_e1736_d_b40, eq136_e1736_d_b41, eq136_e1736_d_b42, eq136_e1736_d_b43, eq136_e1736_d_b44, eq136_e1736_d_b45, eq136_e1736_d_b46, eq136_e1736_d_b47, eq136_e1736_d_b48, eq136_e1736_d_b49, eq136_e1736_d_b50, eq136_e1736_d_b51, eq136_e1736_d_b52, eq136_e1736_d_b53, eq136_e1736_d_b54, eq136_e1736_q,) = {
    if ((s.b[575] && s.b[576]) && (!s.b[577])) {
        let eq136_e1731_q: f64 = s.v[240];
        let eq136_e1732: f64 = (p.p7 * s.v[240]);
        let eq136_e1732_q: f64 = (p.p7 * eq136_e1731_q);
        let eq136_e1734: f64 = (eq136_e1732 * p.p246);
        let eq136_e1734_q: f64 = (eq136_e1732_q * p.p246);
        (eq136_e1734, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq136_e1734_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq136_reactive_node_derivatives: [f64; 23] = [eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22];
        let eq136_reactive_branch_derivatives: [f64; 55] = [eq136_e1736_d_b0, eq136_e1736_d_b1, eq136_e1736_d_b2, eq136_e1736_d_b3, eq136_e1736_d_b4, eq136_e1736_d_b5, eq136_e1736_d_b6, eq136_e1736_d_b7, eq136_e1736_d_b8, eq136_e1736_d_b9, eq136_e1736_d_b10, eq136_e1736_d_b11, eq136_e1736_d_b12, eq136_e1736_d_b13, eq136_e1736_d_b14, eq136_e1736_d_b15, eq136_e1736_d_b16, eq136_e1736_d_b17, eq136_e1736_d_b18, eq136_e1736_d_b19, eq136_e1736_d_b20, eq136_e1736_d_b21, eq136_e1736_d_b22, eq136_e1736_d_b23, eq136_e1736_d_b24, eq136_e1736_d_b25, eq136_e1736_d_b26, eq136_e1736_d_b27, eq136_e1736_d_b28, eq136_e1736_d_b29, eq136_e1736_d_b30, eq136_e1736_d_b31, eq136_e1736_d_b32, eq136_e1736_d_b33, eq136_e1736_d_b34, eq136_e1736_d_b35, eq136_e1736_d_b36, eq136_e1736_d_b37, eq136_e1736_d_b38, eq136_e1736_d_b39, eq136_e1736_d_b40, eq136_e1736_d_b41, eq136_e1736_d_b42, eq136_e1736_d_b43, eq136_e1736_d_b44, eq136_e1736_d_b45, eq136_e1736_d_b46, eq136_e1736_d_b47, eq136_e1736_d_b48, eq136_e1736_d_b49, eq136_e1736_d_b50, eq136_e1736_d_b51, eq136_e1736_d_b52, eq136_e1736_d_b53, eq136_e1736_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            nodes,
            &eq136_reactive_node_derivatives,
            branches,
            &eq136_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
