#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6880_e8307, assign6880_e8307_d_n2, assign6880_e8307_d_n4, assign6880_e8307_d_n7, assign6880_e8307_d_n15, assign6880_e8307_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6880_e8303: f64 = (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0);
        let assign6880_e8305: f64 = (assign6880_e8303 + 1e-57);
        (assign6880_e8305, ((locals.var_fn61_calc_iq__qs2_dn2 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0_dn2)), ((locals.var_fn61_calc_iq__qs2_dn4 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0_dn4)), ((locals.var_fn61_calc_iq__qs2_dn7 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0_dn7)), ((locals.var_fn61_calc_iq__qs2_dn15 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0_dn15)), ((locals.var_fn61_calc_iq__qs2_dn16 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qs3, locals.var_fn61_calc_iq__qs3_dn2, locals.var_fn61_calc_iq__qs3_dn4, locals.var_fn61_calc_iq__qs3_dn7, locals.var_fn61_calc_iq__qs3_dn15, locals.var_fn61_calc_iq__qs3_dn16,)
    }
};
        locals.var_fn61_calc_iq__qs3 = assign6880_e8307;
        locals.var_fn61_calc_iq__qs3_dn2 = assign6880_e8307_d_n2;
        locals.var_fn61_calc_iq__qs3_dn4 = assign6880_e8307_d_n4;
        locals.var_fn61_calc_iq__qs3_dn7 = assign6880_e8307_d_n7;
        locals.var_fn61_calc_iq__qs3_dn15 = assign6880_e8307_d_n15;
        locals.var_fn61_calc_iq__qs3_dn16 = assign6880_e8307_d_n16;

        let (assign6890_e8315, assign6890_e8315_d_n2, assign6890_e8315_d_n4, assign6890_e8315_d_n7, assign6890_e8315_d_n15, assign6890_e8315_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6890_e8311: f64 = (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0);
        let assign6890_e8313: f64 = (assign6890_e8311 + 1e-38);
        (assign6890_e8313, ((locals.var_fn61_calc_iq__qinvd0_dn2 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0_dn2)), ((locals.var_fn61_calc_iq__qinvd0_dn4 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0_dn4)), ((locals.var_fn61_calc_iq__qinvd0_dn7 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0_dn7)), ((locals.var_fn61_calc_iq__qinvd0_dn15 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0_dn15)), ((locals.var_fn61_calc_iq__qinvd0_dn16 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qd2, locals.var_fn61_calc_iq__qd2_dn2, locals.var_fn61_calc_iq__qd2_dn4, locals.var_fn61_calc_iq__qd2_dn7, locals.var_fn61_calc_iq__qd2_dn15, locals.var_fn61_calc_iq__qd2_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd2 = assign6890_e8315;
        locals.var_fn61_calc_iq__qd2_dn2 = assign6890_e8315_d_n2;
        locals.var_fn61_calc_iq__qd2_dn4 = assign6890_e8315_d_n4;
        locals.var_fn61_calc_iq__qd2_dn7 = assign6890_e8315_d_n7;
        locals.var_fn61_calc_iq__qd2_dn15 = assign6890_e8315_d_n15;
        locals.var_fn61_calc_iq__qd2_dn16 = assign6890_e8315_d_n16;

        let (assign6900_e8323, assign6900_e8323_d_n2, assign6900_e8323_d_n4, assign6900_e8323_d_n7, assign6900_e8323_d_n15, assign6900_e8323_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6900_e8319: f64 = (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0);
        let assign6900_e8321: f64 = (assign6900_e8319 + 1e-57);
        (assign6900_e8321, ((locals.var_fn61_calc_iq__qd2_dn2 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0_dn2)), ((locals.var_fn61_calc_iq__qd2_dn4 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0_dn4)), ((locals.var_fn61_calc_iq__qd2_dn7 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0_dn7)), ((locals.var_fn61_calc_iq__qd2_dn15 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0_dn15)), ((locals.var_fn61_calc_iq__qd2_dn16 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qd3, locals.var_fn61_calc_iq__qd3_dn2, locals.var_fn61_calc_iq__qd3_dn4, locals.var_fn61_calc_iq__qd3_dn7, locals.var_fn61_calc_iq__qd3_dn15, locals.var_fn61_calc_iq__qd3_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd3 = assign6900_e8323;
        locals.var_fn61_calc_iq__qd3_dn2 = assign6900_e8323_d_n2;
        locals.var_fn61_calc_iq__qd3_dn4 = assign6900_e8323_d_n4;
        locals.var_fn61_calc_iq__qd3_dn7 = assign6900_e8323_d_n7;
        locals.var_fn61_calc_iq__qd3_dn15 = assign6900_e8323_d_n15;
        locals.var_fn61_calc_iq__qd3_dn16 = assign6900_e8323_d_n16;

        let (assign6910_e8331, assign6910_e8331_d_n2, assign6910_e8331_d_n4, assign6910_e8331_d_n7, assign6910_e8331_d_n15, assign6910_e8331_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6910_e8327: f64 = (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0);
        let assign6910_e8329: f64 = (assign6910_e8327 + 1e-38);
        (assign6910_e8329, ((locals.var_fn61_calc_iq__qinvs0_dn2 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0_dn2)), ((locals.var_fn61_calc_iq__qinvs0_dn4 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0_dn4)), ((locals.var_fn61_calc_iq__qinvs0_dn7 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0_dn7)), ((locals.var_fn61_calc_iq__qinvs0_dn15 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0_dn15)), ((locals.var_fn61_calc_iq__qinvs0_dn16 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qsqd, locals.var_fn61_calc_iq__qsqd_dn2, locals.var_fn61_calc_iq__qsqd_dn4, locals.var_fn61_calc_iq__qsqd_dn7, locals.var_fn61_calc_iq__qsqd_dn15, locals.var_fn61_calc_iq__qsqd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qsqd = assign6910_e8331;
        locals.var_fn61_calc_iq__qsqd_dn2 = assign6910_e8331_d_n2;
        locals.var_fn61_calc_iq__qsqd_dn4 = assign6910_e8331_d_n4;
        locals.var_fn61_calc_iq__qsqd_dn7 = assign6910_e8331_d_n7;
        locals.var_fn61_calc_iq__qsqd_dn15 = assign6910_e8331_d_n15;
        locals.var_fn61_calc_iq__qsqd_dn16 = assign6910_e8331_d_n16;

        let (assign6920_e8349, assign6920_e8349_d_n2, assign6920_e8349_d_n4, assign6920_e8349_d_n7, assign6920_e8349_d_n15, assign6920_e8349_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6920_e8335: f64 = (2.0 / 3.0);
        let assign6920_e8338: f64 = (locals.var_fn61_calc_iq__qs2 + locals.var_fn61_calc_iq__qd2);
        let assign6920_e8340: f64 = (assign6920_e8338 + locals.var_fn61_calc_iq__qsqd);
        let assign6920_e8341: f64 = (assign6920_e8335 * assign6920_e8340);
        let assign6920_e8344: f64 = (locals.var_fn61_calc_iq__qinvs0 + locals.var_fn61_calc_iq__qinvd0);
        let assign6920_e8346: f64 = (assign6920_e8344 + 2e-19);
        let assign6920_e8347: f64 = (assign6920_e8341 / assign6920_e8346);
        (assign6920_e8347, ((((assign6920_e8335 * ((locals.var_fn61_calc_iq__qs2_dn2 + locals.var_fn61_calc_iq__qd2_dn2) + locals.var_fn61_calc_iq__qsqd_dn2)) * assign6920_e8346) - (assign6920_e8341 * (locals.var_fn61_calc_iq__qinvs0_dn2 + locals.var_fn61_calc_iq__qinvd0_dn2))) / (assign6920_e8346 * assign6920_e8346)), ((((assign6920_e8335 * ((locals.var_fn61_calc_iq__qs2_dn4 + locals.var_fn61_calc_iq__qd2_dn4) + locals.var_fn61_calc_iq__qsqd_dn4)) * assign6920_e8346) - (assign6920_e8341 * (locals.var_fn61_calc_iq__qinvs0_dn4 + locals.var_fn61_calc_iq__qinvd0_dn4))) / (assign6920_e8346 * assign6920_e8346)), ((((assign6920_e8335 * ((locals.var_fn61_calc_iq__qs2_dn7 + locals.var_fn61_calc_iq__qd2_dn7) + locals.var_fn61_calc_iq__qsqd_dn7)) * assign6920_e8346) - (assign6920_e8341 * (locals.var_fn61_calc_iq__qinvs0_dn7 + locals.var_fn61_calc_iq__qinvd0_dn7))) / (assign6920_e8346 * assign6920_e8346)), ((((assign6920_e8335 * ((locals.var_fn61_calc_iq__qs2_dn15 + locals.var_fn61_calc_iq__qd2_dn15) + locals.var_fn61_calc_iq__qsqd_dn15)) * assign6920_e8346) - (assign6920_e8341 * (locals.var_fn61_calc_iq__qinvs0_dn15 + locals.var_fn61_calc_iq__qinvd0_dn15))) / (assign6920_e8346 * assign6920_e8346)), ((((assign6920_e8335 * ((locals.var_fn61_calc_iq__qs2_dn16 + locals.var_fn61_calc_iq__qd2_dn16) + locals.var_fn61_calc_iq__qsqd_dn16)) * assign6920_e8346) - (assign6920_e8341 * (locals.var_fn61_calc_iq__qinvs0_dn16 + locals.var_fn61_calc_iq__qinvd0_dn16))) / (assign6920_e8346 * assign6920_e8346)),)
    } else {
        (locals.var_fn61_calc_iq__qinvdd, locals.var_fn61_calc_iq__qinvdd_dn2, locals.var_fn61_calc_iq__qinvdd_dn4, locals.var_fn61_calc_iq__qinvdd_dn7, locals.var_fn61_calc_iq__qinvdd_dn15, locals.var_fn61_calc_iq__qinvdd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvdd = assign6920_e8349;
        locals.var_fn61_calc_iq__qinvdd_dn2 = assign6920_e8349_d_n2;
        locals.var_fn61_calc_iq__qinvdd_dn4 = assign6920_e8349_d_n4;
        locals.var_fn61_calc_iq__qinvdd_dn7 = assign6920_e8349_d_n7;
        locals.var_fn61_calc_iq__qinvdd_dn15 = assign6920_e8349_d_n15;
        locals.var_fn61_calc_iq__qinvdd_dn16 = assign6920_e8349_d_n16;

        let (assign6930_e8383, assign6930_e8383_d_n2, assign6930_e8383_d_n4, assign6930_e8383_d_n7, assign6930_e8383_d_n15, assign6930_e8383_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6930_e8354: f64 = (2.0 * locals.var_fn61_calc_iq__qs3);
        let assign6930_e8357: f64 = (3.0 * locals.var_fn61_calc_iq__qd3);
        let assign6930_e8358: f64 = (assign6930_e8354 + assign6930_e8357);
        let assign6930_e8361: f64 = (4.0 * locals.var_fn61_calc_iq__qs2);
        let assign6930_e8363: f64 = (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0);
        let assign6930_e8364: f64 = (assign6930_e8358 + assign6930_e8363);
        let assign6930_e8367: f64 = (6.0 * locals.var_fn61_calc_iq__qd2);
        let assign6930_e8369: f64 = (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0);
        let assign6930_e8370: f64 = (assign6930_e8364 + assign6930_e8369);
        let assign6930_e8371: f64 = (2.0 * assign6930_e8370);
        let assign6930_e8375: f64 = (locals.var_fn61_calc_iq__qs2 + locals.var_fn61_calc_iq__qd2);
        let assign6930_e8378: f64 = (2.0 * locals.var_fn61_calc_iq__qsqd);
        let assign6930_e8379: f64 = (assign6930_e8375 + assign6930_e8378);
        let assign6930_e8380: f64 = (15.0 * assign6930_e8379);
        let assign6930_e8381: f64 = (assign6930_e8371 / assign6930_e8380);
        (assign6930_e8381, ((((2.0 * ((((2.0 * locals.var_fn61_calc_iq__qs3_dn2) + (3.0 * locals.var_fn61_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn61_calc_iq__qs2_dn2) * locals.var_fn61_calc_iq__qinvd0) + (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn61_calc_iq__qd2_dn2) * locals.var_fn61_calc_iq__qinvs0) + (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0_dn2)))) * assign6930_e8380) - (assign6930_e8371 * (15.0 * ((locals.var_fn61_calc_iq__qs2_dn2 + locals.var_fn61_calc_iq__qd2_dn2) + (2.0 * locals.var_fn61_calc_iq__qsqd_dn2))))) / (assign6930_e8380 * assign6930_e8380)), ((((2.0 * ((((2.0 * locals.var_fn61_calc_iq__qs3_dn4) + (3.0 * locals.var_fn61_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn61_calc_iq__qs2_dn4) * locals.var_fn61_calc_iq__qinvd0) + (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn61_calc_iq__qd2_dn4) * locals.var_fn61_calc_iq__qinvs0) + (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0_dn4)))) * assign6930_e8380) - (assign6930_e8371 * (15.0 * ((locals.var_fn61_calc_iq__qs2_dn4 + locals.var_fn61_calc_iq__qd2_dn4) + (2.0 * locals.var_fn61_calc_iq__qsqd_dn4))))) / (assign6930_e8380 * assign6930_e8380)), ((((2.0 * ((((2.0 * locals.var_fn61_calc_iq__qs3_dn7) + (3.0 * locals.var_fn61_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn61_calc_iq__qs2_dn7) * locals.var_fn61_calc_iq__qinvd0) + (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn61_calc_iq__qd2_dn7) * locals.var_fn61_calc_iq__qinvs0) + (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0_dn7)))) * assign6930_e8380) - (assign6930_e8371 * (15.0 * ((locals.var_fn61_calc_iq__qs2_dn7 + locals.var_fn61_calc_iq__qd2_dn7) + (2.0 * locals.var_fn61_calc_iq__qsqd_dn7))))) / (assign6930_e8380 * assign6930_e8380)), ((((2.0 * ((((2.0 * locals.var_fn61_calc_iq__qs3_dn15) + (3.0 * locals.var_fn61_calc_iq__qd3_dn15)) + (((4.0 * locals.var_fn61_calc_iq__qs2_dn15) * locals.var_fn61_calc_iq__qinvd0) + (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0_dn15))) + (((6.0 * locals.var_fn61_calc_iq__qd2_dn15) * locals.var_fn61_calc_iq__qinvs0) + (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0_dn15)))) * assign6930_e8380) - (assign6930_e8371 * (15.0 * ((locals.var_fn61_calc_iq__qs2_dn15 + locals.var_fn61_calc_iq__qd2_dn15) + (2.0 * locals.var_fn61_calc_iq__qsqd_dn15))))) / (assign6930_e8380 * assign6930_e8380)), ((((2.0 * ((((2.0 * locals.var_fn61_calc_iq__qs3_dn16) + (3.0 * locals.var_fn61_calc_iq__qd3_dn16)) + (((4.0 * locals.var_fn61_calc_iq__qs2_dn16) * locals.var_fn61_calc_iq__qinvd0) + (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0_dn16))) + (((6.0 * locals.var_fn61_calc_iq__qd2_dn16) * locals.var_fn61_calc_iq__qinvs0) + (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0_dn16)))) * assign6930_e8380) - (assign6930_e8371 * (15.0 * ((locals.var_fn61_calc_iq__qs2_dn16 + locals.var_fn61_calc_iq__qd2_dn16) + (2.0 * locals.var_fn61_calc_iq__qsqd_dn16))))) / (assign6930_e8380 * assign6930_e8380)),)
    } else {
        (locals.var_fn61_calc_iq__qd1, locals.var_fn61_calc_iq__qd1_dn2, locals.var_fn61_calc_iq__qd1_dn4, locals.var_fn61_calc_iq__qd1_dn7, locals.var_fn61_calc_iq__qd1_dn15, locals.var_fn61_calc_iq__qd1_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd1 = assign6930_e8383;
        locals.var_fn61_calc_iq__qd1_dn2 = assign6930_e8383_d_n2;
        locals.var_fn61_calc_iq__qd1_dn4 = assign6930_e8383_d_n4;
        locals.var_fn61_calc_iq__qd1_dn7 = assign6930_e8383_d_n7;
        locals.var_fn61_calc_iq__qd1_dn15 = assign6930_e8383_d_n15;
        locals.var_fn61_calc_iq__qd1_dn16 = assign6930_e8383_d_n16;

        let (assign6940_e8389, assign6940_e8389_d_n2, assign6940_e8389_d_n4, assign6940_e8389_d_n7, assign6940_e8389_d_n15, assign6940_e8389_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6940_e8387: f64 = (locals.var_fn61_calc_iq__qinvdd - locals.var_fn61_calc_iq__qd1);
        (assign6940_e8387, (locals.var_fn61_calc_iq__qinvdd_dn2 - locals.var_fn61_calc_iq__qd1_dn2), (locals.var_fn61_calc_iq__qinvdd_dn4 - locals.var_fn61_calc_iq__qd1_dn4), (locals.var_fn61_calc_iq__qinvdd_dn7 - locals.var_fn61_calc_iq__qd1_dn7), (locals.var_fn61_calc_iq__qinvdd_dn15 - locals.var_fn61_calc_iq__qd1_dn15), (locals.var_fn61_calc_iq__qinvdd_dn16 - locals.var_fn61_calc_iq__qd1_dn16),)
    } else {
        (locals.var_fn61_calc_iq__qs, locals.var_fn61_calc_iq__qs_dn2, locals.var_fn61_calc_iq__qs_dn4, locals.var_fn61_calc_iq__qs_dn7, locals.var_fn61_calc_iq__qs_dn15, locals.var_fn61_calc_iq__qs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qs = assign6940_e8389;
        locals.var_fn61_calc_iq__qs_dn2 = assign6940_e8389_d_n2;
        locals.var_fn61_calc_iq__qs_dn4 = assign6940_e8389_d_n4;
        locals.var_fn61_calc_iq__qs_dn7 = assign6940_e8389_d_n7;
        locals.var_fn61_calc_iq__qs_dn15 = assign6940_e8389_d_n15;
        locals.var_fn61_calc_iq__qs_dn16 = assign6940_e8389_d_n16;

        let (assign6950_e8393, assign6950_e8393_d_n2, assign6950_e8393_d_n4, assign6950_e8393_d_n7, assign6950_e8393_d_n15, assign6950_e8393_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qd1, locals.var_fn61_calc_iq__qd1_dn2, locals.var_fn61_calc_iq__qd1_dn4, locals.var_fn61_calc_iq__qd1_dn7, locals.var_fn61_calc_iq__qd1_dn15, locals.var_fn61_calc_iq__qd1_dn16,)
    } else {
        (locals.var_fn61_calc_iq__qd, locals.var_fn61_calc_iq__qd_dn2, locals.var_fn61_calc_iq__qd_dn4, locals.var_fn61_calc_iq__qd_dn7, locals.var_fn61_calc_iq__qd_dn15, locals.var_fn61_calc_iq__qd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd = assign6950_e8393;
        locals.var_fn61_calc_iq__qd_dn2 = assign6950_e8393_d_n2;
        locals.var_fn61_calc_iq__qd_dn4 = assign6950_e8393_d_n4;
        locals.var_fn61_calc_iq__qd_dn7 = assign6950_e8393_d_n7;
        locals.var_fn61_calc_iq__qd_dn15 = assign6950_e8393_d_n15;
        locals.var_fn61_calc_iq__qd_dn16 = assign6950_e8393_d_n16;

        let (assign6960_e8407, assign6960_e8407_d_n2, assign6960_e8407_d_n4, assign6960_e8407_d_n7, assign6960_e8407_d_n15, assign6960_e8407_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6960_e8397: f64 = (locals.var_fn61_calc_iq__w * locals.var_fn61_calc_iq__ngf);
        let assign6960_e8399: f64 = (assign6960_e8397 * locals.var_fn61_calc_iq__lin);
        let assign6960_e8401: f64 = (assign6960_e8399 * locals.var_fn61_calc_iq__type);
        let assign6960_e8403: f64 = (assign6960_e8401 * locals.var_fn61_calc_iq__qs);
        let assign6960_e8405: f64 = (assign6960_e8403 * locals.var_fn61_calc_iq__trapfracdl);
        (assign6960_e8405, ((assign6960_e8401 * locals.var_fn61_calc_iq__qs_dn2) * locals.var_fn61_calc_iq__trapfracdl), ((assign6960_e8401 * locals.var_fn61_calc_iq__qs_dn4) * locals.var_fn61_calc_iq__trapfracdl), ((assign6960_e8401 * locals.var_fn61_calc_iq__qs_dn7) * locals.var_fn61_calc_iq__trapfracdl), ((assign6960_e8401 * locals.var_fn61_calc_iq__qs_dn15) * locals.var_fn61_calc_iq__trapfracdl), ((assign6960_e8401 * locals.var_fn61_calc_iq__qs_dn16) * locals.var_fn61_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn61_calc_iq__qgsout, locals.var_fn61_calc_iq__qgsout_dn2, locals.var_fn61_calc_iq__qgsout_dn4, locals.var_fn61_calc_iq__qgsout_dn7, locals.var_fn61_calc_iq__qgsout_dn15, locals.var_fn61_calc_iq__qgsout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qgsout = assign6960_e8407;
        locals.var_fn61_calc_iq__qgsout_dn2 = assign6960_e8407_d_n2;
        locals.var_fn61_calc_iq__qgsout_dn4 = assign6960_e8407_d_n4;
        locals.var_fn61_calc_iq__qgsout_dn7 = assign6960_e8407_d_n7;
        locals.var_fn61_calc_iq__qgsout_dn15 = assign6960_e8407_d_n15;
        locals.var_fn61_calc_iq__qgsout_dn16 = assign6960_e8407_d_n16;

        let (assign6970_e8421, assign6970_e8421_d_n2, assign6970_e8421_d_n4, assign6970_e8421_d_n7, assign6970_e8421_d_n15, assign6970_e8421_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6970_e8411: f64 = (locals.var_fn61_calc_iq__w * locals.var_fn61_calc_iq__ngf);
        let assign6970_e8413: f64 = (assign6970_e8411 * locals.var_fn61_calc_iq__lin);
        let assign6970_e8415: f64 = (assign6970_e8413 * locals.var_fn61_calc_iq__type);
        let assign6970_e8417: f64 = (assign6970_e8415 * locals.var_fn61_calc_iq__qd);
        let assign6970_e8419: f64 = (assign6970_e8417 * locals.var_fn61_calc_iq__trapfracdl);
        (assign6970_e8419, ((assign6970_e8415 * locals.var_fn61_calc_iq__qd_dn2) * locals.var_fn61_calc_iq__trapfracdl), ((assign6970_e8415 * locals.var_fn61_calc_iq__qd_dn4) * locals.var_fn61_calc_iq__trapfracdl), ((assign6970_e8415 * locals.var_fn61_calc_iq__qd_dn7) * locals.var_fn61_calc_iq__trapfracdl), ((assign6970_e8415 * locals.var_fn61_calc_iq__qd_dn15) * locals.var_fn61_calc_iq__trapfracdl), ((assign6970_e8415 * locals.var_fn61_calc_iq__qd_dn16) * locals.var_fn61_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn61_calc_iq__qgdout, locals.var_fn61_calc_iq__qgdout_dn2, locals.var_fn61_calc_iq__qgdout_dn4, locals.var_fn61_calc_iq__qgdout_dn7, locals.var_fn61_calc_iq__qgdout_dn15, locals.var_fn61_calc_iq__qgdout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qgdout = assign6970_e8421;
        locals.var_fn61_calc_iq__qgdout_dn2 = assign6970_e8421_d_n2;
        locals.var_fn61_calc_iq__qgdout_dn4 = assign6970_e8421_d_n4;
        locals.var_fn61_calc_iq__qgdout_dn7 = assign6970_e8421_d_n7;
        locals.var_fn61_calc_iq__qgdout_dn15 = assign6970_e8421_d_n15;
        locals.var_fn61_calc_iq__qgdout_dn16 = assign6970_e8421_d_n16;

        let assign6980_e8424: f64 = if locals.var_fn61_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign6980_e8424;

        let (assign6990_e8440, assign6990_e8440_d_n2, assign6990_e8440_d_n4, assign6990_e8440_d_n7, assign6990_e8440_d_n15,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign6990_e8432: f64 = (p.p51 * 0.5);
        let assign6990_e8434: f64 = (assign6990_e8432 * locals.var_fn61_calc_iq__alpha_phit);
        let assign6990_e8435: f64 = (locals.var_fn61_calc_iq__vtof - assign6990_e8434);
        let assign6990_e8436: f64 = (locals.var_fn61_calc_iq__vcin - assign6990_e8435);
        let assign6990_e8438: f64 = (assign6990_e8436 / locals.var_fn61_calc_iq__two_n_phit0);
        (assign6990_e8438, (locals.var_fn61_calc_iq__vcin_dn2 / locals.var_fn61_calc_iq__two_n_phit0), ((((-(locals.var_fn61_calc_iq__vtof_dn4 - (assign6990_e8432 * locals.var_fn61_calc_iq__alpha_phit_dn4))) * locals.var_fn61_calc_iq__two_n_phit0) - (assign6990_e8436 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) / (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__two_n_phit0)), (locals.var_fn61_calc_iq__vcin_dn7 / locals.var_fn61_calc_iq__two_n_phit0), (locals.var_fn61_calc_iq__vcin_dn15 / locals.var_fn61_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn61_calc_iq__etac, locals.var_fn61_calc_iq__etac_dn2, locals.var_fn61_calc_iq__etac_dn4, locals.var_fn61_calc_iq__etac_dn7, locals.var_fn61_calc_iq__etac_dn15,)
    }
};
        locals.var_fn61_calc_iq__etac = assign6990_e8440;
        locals.var_fn61_calc_iq__etac_dn2 = assign6990_e8440_d_n2;
        locals.var_fn61_calc_iq__etac_dn4 = assign6990_e8440_d_n4;
        locals.var_fn61_calc_iq__etac_dn7 = assign6990_e8440_d_n7;
        locals.var_fn61_calc_iq__etac_dn15 = assign6990_e8440_d_n15;

        let assign7000_e8443: f64 = if locals.var_fn61_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign7000_e8443;

        let (assign7010_e8451, assign7010_e8451_d_n2, assign7010_e8451_d_n3, assign7010_e8451_d_n4, assign7010_e8451_d_n7, assign7010_e8451_d_n15, assign7010_e8451_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard88 != 0.0)) {
        (locals.var_fn61_calc_iq__etac, locals.var_fn61_calc_iq__etac_dn2, 0.0, locals.var_fn61_calc_iq__etac_dn4, locals.var_fn61_calc_iq__etac_dn7, locals.var_fn61_calc_iq__etac_dn15, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7010_e8451;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7010_e8451_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7010_e8451_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7010_e8451_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7010_e8451_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7010_e8451_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7010_e8451_d_n16;

        let assign7020_e8454: f64 = (-50.0);
        let assign7020_e8455: f64 = if locals.var_fn61_calc_iq__etac < assign7020_e8454 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign7020_e8455;

        let (assign7030_e8467, assign7030_e8467_d_n2, assign7030_e8467_d_n3, assign7030_e8467_d_n4, assign7030_e8467_d_n7, assign7030_e8467_d_n15, assign7030_e8467_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign7030_e8465: f64 = (locals.var_fn61_calc_iq__etac).exp();
        (assign7030_e8465, (assign7030_e8465 * locals.var_fn61_calc_iq__etac_dn2), 0.0, (assign7030_e8465 * locals.var_fn61_calc_iq__etac_dn4), (assign7030_e8465 * locals.var_fn61_calc_iq__etac_dn7), (assign7030_e8465 * locals.var_fn61_calc_iq__etac_dn15), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7030_e8467;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7030_e8467_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7030_e8467_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7030_e8467_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7030_e8467_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7030_e8467_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7030_e8467_d_n16;

        let (assign7040_e8483, assign7040_e8483_d_n2, assign7040_e8483_d_n3, assign7040_e8483_d_n4, assign7040_e8483_d_n7, assign7040_e8483_d_n15, assign7040_e8483_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard89 == 0.0)) {
        let assign7040_e8479: f64 = (locals.var_fn61_calc_iq__etac).exp();
        let assign7040_e8480: f64 = (1.0 + assign7040_e8479);
        let assign7040_e8481: f64 = (assign7040_e8480).ln();
        (assign7040_e8481, ((assign7040_e8479 * locals.var_fn61_calc_iq__etac_dn2) / assign7040_e8480), 0.0, ((assign7040_e8479 * locals.var_fn61_calc_iq__etac_dn4) / assign7040_e8480), ((assign7040_e8479 * locals.var_fn61_calc_iq__etac_dn7) / assign7040_e8480), ((assign7040_e8479 * locals.var_fn61_calc_iq__etac_dn15) / assign7040_e8480), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7040_e8483;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7040_e8483_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7040_e8483_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7040_e8483_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7040_e8483_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7040_e8483_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7040_e8483_d_n16;

        let (assign7050_e8501, assign7050_e8501_d_n2, assign7050_e8501_d_n3, assign7050_e8501_d_n4, assign7050_e8501_d_n7, assign7050_e8501_d_n15, assign7050_e8501_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7050_e8489: f64 = (locals.var_fn61_calc_iq__w * locals.var_fn61_calc_iq__ngf);
        let assign7050_e8491: f64 = (assign7050_e8489 * locals.var_fn61_calc_iq__type);
        let assign7050_e8493: f64 = (assign7050_e8491 * locals.var_fn61_calc_iq__cc);
        let assign7050_e8495: f64 = (assign7050_e8493 * locals.var_fn61_calc_iq__two_n_phit0);
        let assign7050_e8497: f64 = (assign7050_e8495 * locals.var_fn61_calc_iq__exparg);
        let assign7050_e8499: f64 = (assign7050_e8497 * locals.var_fn61_calc_iq__trapfracdl);
        (assign7050_e8499, ((assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn2) * locals.var_fn61_calc_iq__trapfracdl), ((assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn3) * locals.var_fn61_calc_iq__trapfracdl), ((((((assign7050_e8491 * locals.var_fn61_calc_iq__cc_dn4) * locals.var_fn61_calc_iq__two_n_phit0) + (assign7050_e8493 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) * locals.var_fn61_calc_iq__exparg) + (assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn4)) * locals.var_fn61_calc_iq__trapfracdl), ((assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn7) * locals.var_fn61_calc_iq__trapfracdl), ((assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn15) * locals.var_fn61_calc_iq__trapfracdl), ((assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn16) * locals.var_fn61_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn61_calc_iq__qcout, locals.var_fn61_calc_iq__qcout_dn2, locals.var_fn61_calc_iq__qcout_dn3, locals.var_fn61_calc_iq__qcout_dn4, locals.var_fn61_calc_iq__qcout_dn7, locals.var_fn61_calc_iq__qcout_dn15, locals.var_fn61_calc_iq__qcout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qcout = assign7050_e8501;
        locals.var_fn61_calc_iq__qcout_dn2 = assign7050_e8501_d_n2;
        locals.var_fn61_calc_iq__qcout_dn3 = assign7050_e8501_d_n3;
        locals.var_fn61_calc_iq__qcout_dn4 = assign7050_e8501_d_n4;
        locals.var_fn61_calc_iq__qcout_dn7 = assign7050_e8501_d_n7;
        locals.var_fn61_calc_iq__qcout_dn15 = assign7050_e8501_d_n15;
        locals.var_fn61_calc_iq__qcout_dn16 = assign7050_e8501_d_n16;

        let (assign7060_e8517, assign7060_e8517_d_n3, assign7060_e8517_d_n4, assign7060_e8517_d_n15,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7060_e8509: f64 = (p.p51 * 0.5);
        let assign7060_e8511: f64 = (assign7060_e8509 * locals.var_fn61_calc_iq__alpha_phit);
        let assign7060_e8512: f64 = (locals.var_fn61_calc_iq__vtof - assign7060_e8511);
        let assign7060_e8513: f64 = (locals.var_fn61_calc_iq__vbin - assign7060_e8512);
        let assign7060_e8515: f64 = (assign7060_e8513 / locals.var_fn61_calc_iq__two_n_phit0);
        (assign7060_e8515, (locals.var_fn61_calc_iq__vbin_dn3 / locals.var_fn61_calc_iq__two_n_phit0), ((((-(locals.var_fn61_calc_iq__vtof_dn4 - (assign7060_e8509 * locals.var_fn61_calc_iq__alpha_phit_dn4))) * locals.var_fn61_calc_iq__two_n_phit0) - (assign7060_e8513 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) / (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__two_n_phit0)), (locals.var_fn61_calc_iq__vbin_dn15 / locals.var_fn61_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn61_calc_iq__etab, locals.var_fn61_calc_iq__etab_dn3, locals.var_fn61_calc_iq__etab_dn4, locals.var_fn61_calc_iq__etab_dn15,)
    }
};
        locals.var_fn61_calc_iq__etab = assign7060_e8517;
        locals.var_fn61_calc_iq__etab_dn3 = assign7060_e8517_d_n3;
        locals.var_fn61_calc_iq__etab_dn4 = assign7060_e8517_d_n4;
        locals.var_fn61_calc_iq__etab_dn15 = assign7060_e8517_d_n15;

        let assign7070_e8520: f64 = if locals.var_fn61_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign7070_e8520;

        let (assign7080_e8528, assign7080_e8528_d_n2, assign7080_e8528_d_n3, assign7080_e8528_d_n4, assign7080_e8528_d_n7, assign7080_e8528_d_n15, assign7080_e8528_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard90 != 0.0)) {
        (locals.var_fn61_calc_iq__etab, 0.0, locals.var_fn61_calc_iq__etab_dn3, locals.var_fn61_calc_iq__etab_dn4, 0.0, locals.var_fn61_calc_iq__etab_dn15, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7080_e8528;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7080_e8528_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7080_e8528_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7080_e8528_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7080_e8528_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7080_e8528_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7080_e8528_d_n16;

        let assign7090_e8531: f64 = (-50.0);
        let assign7090_e8532: f64 = if locals.var_fn61_calc_iq__etab < assign7090_e8531 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign7090_e8532;

        let (assign7100_e8544, assign7100_e8544_d_n2, assign7100_e8544_d_n3, assign7100_e8544_d_n4, assign7100_e8544_d_n7, assign7100_e8544_d_n15, assign7100_e8544_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign7100_e8542: f64 = (locals.var_fn61_calc_iq__etab).exp();
        (assign7100_e8542, 0.0, (assign7100_e8542 * locals.var_fn61_calc_iq__etab_dn3), (assign7100_e8542 * locals.var_fn61_calc_iq__etab_dn4), 0.0, (assign7100_e8542 * locals.var_fn61_calc_iq__etab_dn15), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7100_e8544;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7100_e8544_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7100_e8544_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7100_e8544_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7100_e8544_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7100_e8544_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7100_e8544_d_n16;

        let (assign7110_e8560, assign7110_e8560_d_n2, assign7110_e8560_d_n3, assign7110_e8560_d_n4, assign7110_e8560_d_n7, assign7110_e8560_d_n15, assign7110_e8560_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard91 == 0.0)) {
        let assign7110_e8556: f64 = (locals.var_fn61_calc_iq__etab).exp();
        let assign7110_e8557: f64 = (1.0 + assign7110_e8556);
        let assign7110_e8558: f64 = (assign7110_e8557).ln();
        (assign7110_e8558, 0.0, ((assign7110_e8556 * locals.var_fn61_calc_iq__etab_dn3) / assign7110_e8557), ((assign7110_e8556 * locals.var_fn61_calc_iq__etab_dn4) / assign7110_e8557), 0.0, ((assign7110_e8556 * locals.var_fn61_calc_iq__etab_dn15) / assign7110_e8557), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7110_e8560;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7110_e8560_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7110_e8560_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7110_e8560_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7110_e8560_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7110_e8560_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7110_e8560_d_n16;

        let (assign7120_e8578, assign7120_e8578_d_n2, assign7120_e8578_d_n3, assign7120_e8578_d_n4, assign7120_e8578_d_n7, assign7120_e8578_d_n15, assign7120_e8578_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7120_e8566: f64 = (locals.var_fn61_calc_iq__w * locals.var_fn61_calc_iq__ngf);
        let assign7120_e8568: f64 = (assign7120_e8566 * locals.var_fn61_calc_iq__type);
        let assign7120_e8570: f64 = (assign7120_e8568 * locals.var_fn61_calc_iq__cb);
        let assign7120_e8572: f64 = (assign7120_e8570 * locals.var_fn61_calc_iq__two_n_phit0);
        let assign7120_e8574: f64 = (assign7120_e8572 * locals.var_fn61_calc_iq__exparg);
        let assign7120_e8576: f64 = (assign7120_e8574 * locals.var_fn61_calc_iq__trapfracdl);
        (assign7120_e8576, ((assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn2) * locals.var_fn61_calc_iq__trapfracdl), ((assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn3) * locals.var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * locals.var_fn61_calc_iq__cb_dn4) * locals.var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) * locals.var_fn61_calc_iq__exparg) + (assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn4)) * locals.var_fn61_calc_iq__trapfracdl), ((assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn7) * locals.var_fn61_calc_iq__trapfracdl), ((assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn15) * locals.var_fn61_calc_iq__trapfracdl), ((assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn16) * locals.var_fn61_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn61_calc_iq__qbout, locals.var_fn61_calc_iq__qbout_dn2, locals.var_fn61_calc_iq__qbout_dn3, locals.var_fn61_calc_iq__qbout_dn4, locals.var_fn61_calc_iq__qbout_dn7, locals.var_fn61_calc_iq__qbout_dn15, locals.var_fn61_calc_iq__qbout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qbout = assign7120_e8578;
        locals.var_fn61_calc_iq__qbout_dn2 = assign7120_e8578_d_n2;
        locals.var_fn61_calc_iq__qbout_dn3 = assign7120_e8578_d_n3;
        locals.var_fn61_calc_iq__qbout_dn4 = assign7120_e8578_d_n4;
        locals.var_fn61_calc_iq__qbout_dn7 = assign7120_e8578_d_n7;
        locals.var_fn61_calc_iq__qbout_dn15 = assign7120_e8578_d_n15;
        locals.var_fn61_calc_iq__qbout_dn16 = assign7120_e8578_d_n16;

        let (assign7130_e8585, assign7130_e8585_d_n2, assign7130_e8585_d_n3, assign7130_e8585_d_n4, assign7130_e8585_d_n7, assign7130_e8585_d_n15, assign7130_e8585_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qcout, locals.var_fn61_calc_iq__qcout_dn2, locals.var_fn61_calc_iq__qcout_dn3, locals.var_fn61_calc_iq__qcout_dn4, locals.var_fn61_calc_iq__qcout_dn7, locals.var_fn61_calc_iq__qcout_dn15, locals.var_fn61_calc_iq__qcout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qcout = assign7130_e8585;
        locals.var_fn61_calc_iq__qcout_dn2 = assign7130_e8585_d_n2;
        locals.var_fn61_calc_iq__qcout_dn3 = assign7130_e8585_d_n3;
        locals.var_fn61_calc_iq__qcout_dn4 = assign7130_e8585_d_n4;
        locals.var_fn61_calc_iq__qcout_dn7 = assign7130_e8585_d_n7;
        locals.var_fn61_calc_iq__qcout_dn15 = assign7130_e8585_d_n15;
        locals.var_fn61_calc_iq__qcout_dn16 = assign7130_e8585_d_n16;

        let (assign7140_e8592, assign7140_e8592_d_n2, assign7140_e8592_d_n3, assign7140_e8592_d_n4, assign7140_e8592_d_n7, assign7140_e8592_d_n15, assign7140_e8592_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qbout, locals.var_fn61_calc_iq__qbout_dn2, locals.var_fn61_calc_iq__qbout_dn3, locals.var_fn61_calc_iq__qbout_dn4, locals.var_fn61_calc_iq__qbout_dn7, locals.var_fn61_calc_iq__qbout_dn15, locals.var_fn61_calc_iq__qbout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qbout = assign7140_e8592;
        locals.var_fn61_calc_iq__qbout_dn2 = assign7140_e8592_d_n2;
        locals.var_fn61_calc_iq__qbout_dn3 = assign7140_e8592_d_n3;
        locals.var_fn61_calc_iq__qbout_dn4 = assign7140_e8592_d_n4;
        locals.var_fn61_calc_iq__qbout_dn7 = assign7140_e8592_d_n7;
        locals.var_fn61_calc_iq__qbout_dn15 = assign7140_e8592_d_n15;
        locals.var_fn61_calc_iq__qbout_dn16 = assign7140_e8592_d_n16;

        let assign7150_e8595: f64 = if locals.var_fn61_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign7150_e8595;

        let (assign7160_e8611, assign7160_e8611_d_n2, assign7160_e8611_d_n4, assign7160_e8611_d_n7, assign7160_e8611_d_n15,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign7160_e8603: f64 = (p.p51 * 0.5);
        let assign7160_e8605: f64 = (assign7160_e8603 * locals.var_fn61_calc_iq__alpha_phit);
        let assign7160_e8606: f64 = (locals.var_fn61_calc_iq__vtof - assign7160_e8605);
        let assign7160_e8607: f64 = (locals.var_fn61_calc_iq__vgsin - assign7160_e8606);
        let assign7160_e8609: f64 = (assign7160_e8607 / locals.var_fn61_calc_iq__two_n_phit0);
        (assign7160_e8609, (locals.var_fn61_calc_iq__vgsin_dn2 / locals.var_fn61_calc_iq__two_n_phit0), ((((-(locals.var_fn61_calc_iq__vtof_dn4 - (assign7160_e8603 * locals.var_fn61_calc_iq__alpha_phit_dn4))) * locals.var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) / (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__two_n_phit0)), (locals.var_fn61_calc_iq__vgsin_dn7 / locals.var_fn61_calc_iq__two_n_phit0), (locals.var_fn61_calc_iq__vgsin_dn15 / locals.var_fn61_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn61_calc_iq__etags, locals.var_fn61_calc_iq__etags_dn2, locals.var_fn61_calc_iq__etags_dn4, locals.var_fn61_calc_iq__etags_dn7, locals.var_fn61_calc_iq__etags_dn15,)
    }
};
        locals.var_fn61_calc_iq__etags = assign7160_e8611;
        locals.var_fn61_calc_iq__etags_dn2 = assign7160_e8611_d_n2;
        locals.var_fn61_calc_iq__etags_dn4 = assign7160_e8611_d_n4;
        locals.var_fn61_calc_iq__etags_dn7 = assign7160_e8611_d_n7;
        locals.var_fn61_calc_iq__etags_dn15 = assign7160_e8611_d_n15;

        let assign7170_e8614: f64 = if locals.var_fn61_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign7170_e8614;

        let (assign7180_e8622, assign7180_e8622_d_n2, assign7180_e8622_d_n3, assign7180_e8622_d_n4, assign7180_e8622_d_n7, assign7180_e8622_d_n15, assign7180_e8622_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard92 != 0.0)) && (locals.var_guard93 != 0.0)) {
        (locals.var_fn61_calc_iq__etags, locals.var_fn61_calc_iq__etags_dn2, 0.0, locals.var_fn61_calc_iq__etags_dn4, locals.var_fn61_calc_iq__etags_dn7, locals.var_fn61_calc_iq__etags_dn15, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7180_e8622;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7180_e8622_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7180_e8622_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7180_e8622_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7180_e8622_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7180_e8622_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7180_e8622_d_n16;

        let assign7190_e8625: f64 = (-50.0);
        let assign7190_e8626: f64 = if locals.var_fn61_calc_iq__etags < assign7190_e8625 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign7190_e8626;

        let (assign7200_e8638, assign7200_e8638_d_n2, assign7200_e8638_d_n3, assign7200_e8638_d_n4, assign7200_e8638_d_n7, assign7200_e8638_d_n15, assign7200_e8638_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard92 != 0.0)) && (locals.var_guard93 == 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign7200_e8636: f64 = (locals.var_fn61_calc_iq__etags).exp();
        (assign7200_e8636, (assign7200_e8636 * locals.var_fn61_calc_iq__etags_dn2), 0.0, (assign7200_e8636 * locals.var_fn61_calc_iq__etags_dn4), (assign7200_e8636 * locals.var_fn61_calc_iq__etags_dn7), (assign7200_e8636 * locals.var_fn61_calc_iq__etags_dn15), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7200_e8638;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7200_e8638_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7200_e8638_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7200_e8638_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7200_e8638_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7200_e8638_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7200_e8638_d_n16;

        let (assign7210_e8654, assign7210_e8654_d_n2, assign7210_e8654_d_n3, assign7210_e8654_d_n4, assign7210_e8654_d_n7, assign7210_e8654_d_n15, assign7210_e8654_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard92 != 0.0)) && (locals.var_guard93 == 0.0)) && (locals.var_guard94 == 0.0)) {
        let assign7210_e8650: f64 = (locals.var_fn61_calc_iq__etags).exp();
        let assign7210_e8651: f64 = (1.0 + assign7210_e8650);
        let assign7210_e8652: f64 = (assign7210_e8651).ln();
        (assign7210_e8652, ((assign7210_e8650 * locals.var_fn61_calc_iq__etags_dn2) / assign7210_e8651), 0.0, ((assign7210_e8650 * locals.var_fn61_calc_iq__etags_dn4) / assign7210_e8651), ((assign7210_e8650 * locals.var_fn61_calc_iq__etags_dn7) / assign7210_e8651), ((assign7210_e8650 * locals.var_fn61_calc_iq__etags_dn15) / assign7210_e8651), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7210_e8654;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7210_e8654_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7210_e8654_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7210_e8654_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7210_e8654_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7210_e8654_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7210_e8654_d_n16;

        let (assign7220_e8672, assign7220_e8672_d_n2, assign7220_e8672_d_n3, assign7220_e8672_d_n4, assign7220_e8672_d_n7, assign7220_e8672_d_n15, assign7220_e8672_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign7220_e8660: f64 = (locals.var_fn61_calc_iq__w * locals.var_fn61_calc_iq__ngf);
        let assign7220_e8662: f64 = (assign7220_e8660 * locals.var_fn61_calc_iq__type);
        let assign7220_e8664: f64 = (assign7220_e8662 * locals.var_fn61_calc_iq__cs);
        let assign7220_e8666: f64 = (assign7220_e8664 * locals.var_fn61_calc_iq__two_n_phit0);
        let assign7220_e8668: f64 = (assign7220_e8666 * locals.var_fn61_calc_iq__exparg);
        let assign7220_e8670: f64 = (assign7220_e8668 * locals.var_fn61_calc_iq__trapfracdl);
        (assign7220_e8670, ((assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn2) * locals.var_fn61_calc_iq__trapfracdl), ((assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn3) * locals.var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * locals.var_fn61_calc_iq__two_n_phit0_dn4) * locals.var_fn61_calc_iq__exparg) + (assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn4)) * locals.var_fn61_calc_iq__trapfracdl), ((assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn7) * locals.var_fn61_calc_iq__trapfracdl), ((assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn15) * locals.var_fn61_calc_iq__trapfracdl), ((assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn16) * locals.var_fn61_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn61_calc_iq__qsout, locals.var_fn61_calc_iq__qsout_dn2, locals.var_fn61_calc_iq__qsout_dn3, locals.var_fn61_calc_iq__qsout_dn4, locals.var_fn61_calc_iq__qsout_dn7, locals.var_fn61_calc_iq__qsout_dn15, locals.var_fn61_calc_iq__qsout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qsout = assign7220_e8672;
        locals.var_fn61_calc_iq__qsout_dn2 = assign7220_e8672_d_n2;
        locals.var_fn61_calc_iq__qsout_dn3 = assign7220_e8672_d_n3;
        locals.var_fn61_calc_iq__qsout_dn4 = assign7220_e8672_d_n4;
        locals.var_fn61_calc_iq__qsout_dn7 = assign7220_e8672_d_n7;
        locals.var_fn61_calc_iq__qsout_dn15 = assign7220_e8672_d_n15;
        locals.var_fn61_calc_iq__qsout_dn16 = assign7220_e8672_d_n16;

    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7230_e8679, assign7230_e8679_d_n2, assign7230_e8679_d_n3, assign7230_e8679_d_n4, assign7230_e8679_d_n7, assign7230_e8679_d_n15, assign7230_e8679_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard92 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qsout, locals.var_fn61_calc_iq__qsout_dn2, locals.var_fn61_calc_iq__qsout_dn3, locals.var_fn61_calc_iq__qsout_dn4, locals.var_fn61_calc_iq__qsout_dn7, locals.var_fn61_calc_iq__qsout_dn15, locals.var_fn61_calc_iq__qsout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qsout = assign7230_e8679;
        locals.var_fn61_calc_iq__qsout_dn2 = assign7230_e8679_d_n2;
        locals.var_fn61_calc_iq__qsout_dn3 = assign7230_e8679_d_n3;
        locals.var_fn61_calc_iq__qsout_dn4 = assign7230_e8679_d_n4;
        locals.var_fn61_calc_iq__qsout_dn7 = assign7230_e8679_d_n7;
        locals.var_fn61_calc_iq__qsout_dn15 = assign7230_e8679_d_n15;
        locals.var_fn61_calc_iq__qsout_dn16 = assign7230_e8679_d_n16;

        let (assign7260_e8691, assign7260_e8691_d_n2, assign7260_e8691_d_n4, assign7260_e8691_d_n7, assign7260_e8691_d_n15, assign7260_e8691_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qgsout, locals.var_fn61_calc_iq__qgsout_dn2, locals.var_fn61_calc_iq__qgsout_dn4, locals.var_fn61_calc_iq__qgsout_dn7, locals.var_fn61_calc_iq__qgsout_dn15, locals.var_fn61_calc_iq__qgsout_dn16,)
    } else {
        (locals.var_qgsfp3, locals.var_qgsfp3_dn2, locals.var_qgsfp3_dn4, locals.var_qgsfp3_dn7, locals.var_qgsfp3_dn15, locals.var_qgsfp3_dn16,)
    }
};
        locals.var_qgsfp3 = assign7260_e8691;
        locals.var_qgsfp3_dn2 = assign7260_e8691_d_n2;
        locals.var_qgsfp3_dn4 = assign7260_e8691_d_n4;
        locals.var_qgsfp3_dn7 = assign7260_e8691_d_n7;
        locals.var_qgsfp3_dn15 = assign7260_e8691_d_n15;
        locals.var_qgsfp3_dn16 = assign7260_e8691_d_n16;

        let (assign7270_e8695, assign7270_e8695_d_n2, assign7270_e8695_d_n4, assign7270_e8695_d_n7, assign7270_e8695_d_n15, assign7270_e8695_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qgdout, locals.var_fn61_calc_iq__qgdout_dn2, locals.var_fn61_calc_iq__qgdout_dn4, locals.var_fn61_calc_iq__qgdout_dn7, locals.var_fn61_calc_iq__qgdout_dn15, locals.var_fn61_calc_iq__qgdout_dn16,)
    } else {
        (locals.var_qgdfp3, locals.var_qgdfp3_dn2, locals.var_qgdfp3_dn4, locals.var_qgdfp3_dn7, locals.var_qgdfp3_dn15, locals.var_qgdfp3_dn16,)
    }
};
        locals.var_qgdfp3 = assign7270_e8695;
        locals.var_qgdfp3_dn2 = assign7270_e8695_d_n2;
        locals.var_qgdfp3_dn4 = assign7270_e8695_d_n4;
        locals.var_qgdfp3_dn7 = assign7270_e8695_d_n7;
        locals.var_qgdfp3_dn15 = assign7270_e8695_d_n15;
        locals.var_qgdfp3_dn16 = assign7270_e8695_d_n16;

        let (assign7280_e8699, assign7280_e8699_d_n2, assign7280_e8699_d_n3, assign7280_e8699_d_n4, assign7280_e8699_d_n7, assign7280_e8699_d_n15, assign7280_e8699_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qcout, locals.var_fn61_calc_iq__qcout_dn2, locals.var_fn61_calc_iq__qcout_dn3, locals.var_fn61_calc_iq__qcout_dn4, locals.var_fn61_calc_iq__qcout_dn7, locals.var_fn61_calc_iq__qcout_dn15, locals.var_fn61_calc_iq__qcout_dn16,)
    } else {
        (locals.var_qcfp3, locals.var_qcfp3_dn2, locals.var_qcfp3_dn3, locals.var_qcfp3_dn4, locals.var_qcfp3_dn7, locals.var_qcfp3_dn15, locals.var_qcfp3_dn16,)
    }
};
        locals.var_qcfp3 = assign7280_e8699;
        locals.var_qcfp3_dn2 = assign7280_e8699_d_n2;
        locals.var_qcfp3_dn3 = assign7280_e8699_d_n3;
        locals.var_qcfp3_dn4 = assign7280_e8699_d_n4;
        locals.var_qcfp3_dn7 = assign7280_e8699_d_n7;
        locals.var_qcfp3_dn15 = assign7280_e8699_d_n15;
        locals.var_qcfp3_dn16 = assign7280_e8699_d_n16;

        let (assign7290_e8703, assign7290_e8703_d_n2, assign7290_e8703_d_n3, assign7290_e8703_d_n4, assign7290_e8703_d_n7, assign7290_e8703_d_n15, assign7290_e8703_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qbout, locals.var_fn61_calc_iq__qbout_dn2, locals.var_fn61_calc_iq__qbout_dn3, locals.var_fn61_calc_iq__qbout_dn4, locals.var_fn61_calc_iq__qbout_dn7, locals.var_fn61_calc_iq__qbout_dn15, locals.var_fn61_calc_iq__qbout_dn16,)
    } else {
        (locals.var_qbfp3, locals.var_qbfp3_dn2, locals.var_qbfp3_dn3, locals.var_qbfp3_dn4, locals.var_qbfp3_dn7, locals.var_qbfp3_dn15, locals.var_qbfp3_dn16,)
    }
};
        locals.var_qbfp3 = assign7290_e8703;
        locals.var_qbfp3_dn2 = assign7290_e8703_d_n2;
        locals.var_qbfp3_dn3 = assign7290_e8703_d_n3;
        locals.var_qbfp3_dn4 = assign7290_e8703_d_n4;
        locals.var_qbfp3_dn7 = assign7290_e8703_d_n7;
        locals.var_qbfp3_dn15 = assign7290_e8703_d_n15;
        locals.var_qbfp3_dn16 = assign7290_e8703_d_n16;

        let (assign7300_e8707, assign7300_e8707_d_n2, assign7300_e8707_d_n3, assign7300_e8707_d_n4, assign7300_e8707_d_n7, assign7300_e8707_d_n15, assign7300_e8707_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qsout, locals.var_fn61_calc_iq__qsout_dn2, locals.var_fn61_calc_iq__qsout_dn3, locals.var_fn61_calc_iq__qsout_dn4, locals.var_fn61_calc_iq__qsout_dn7, locals.var_fn61_calc_iq__qsout_dn15, locals.var_fn61_calc_iq__qsout_dn16,)
    } else {
        (locals.var_qsfp3, locals.var_qsfp3_dn2, locals.var_qsfp3_dn3, locals.var_qsfp3_dn4, locals.var_qsfp3_dn7, locals.var_qsfp3_dn15, locals.var_qsfp3_dn16,)
    }
};
        locals.var_qsfp3 = assign7300_e8707;
        locals.var_qsfp3_dn2 = assign7300_e8707_d_n2;
        locals.var_qsfp3_dn3 = assign7300_e8707_d_n3;
        locals.var_qsfp3_dn4 = assign7300_e8707_d_n4;
        locals.var_qsfp3_dn7 = assign7300_e8707_d_n7;
        locals.var_qsfp3_dn15 = assign7300_e8707_d_n15;
        locals.var_qsfp3_dn16 = assign7300_e8707_d_n16;

        let assign7340_e8722: f64 = if p.p210 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign7340_e8722;

        locals.var_qgsfp2 = 0.0;
        locals.var_qgsfp2_dn2 = 0.0;
        locals.var_qgsfp2_dn4 = 0.0;
        locals.var_qgsfp2_dn7 = 0.0;
        locals.var_qgsfp2_dn14 = 0.0;
        locals.var_qgsfp2_dn15 = 0.0;

        locals.var_qgdfp2 = 0.0;
        locals.var_qgdfp2_dn2 = 0.0;
        locals.var_qgdfp2_dn4 = 0.0;
        locals.var_qgdfp2_dn7 = 0.0;
        locals.var_qgdfp2_dn14 = 0.0;
        locals.var_qgdfp2_dn15 = 0.0;

        locals.var_qcfp2 = 0.0;
        locals.var_qcfp2_dn2 = 0.0;
        locals.var_qcfp2_dn3 = 0.0;
        locals.var_qcfp2_dn4 = 0.0;
        locals.var_qcfp2_dn7 = 0.0;
        locals.var_qcfp2_dn14 = 0.0;
        locals.var_qcfp2_dn15 = 0.0;

        locals.var_qbfp2 = 0.0;
        locals.var_qbfp2_dn2 = 0.0;
        locals.var_qbfp2_dn3 = 0.0;
        locals.var_qbfp2_dn4 = 0.0;
        locals.var_qbfp2_dn7 = 0.0;
        locals.var_qbfp2_dn14 = 0.0;
        locals.var_qbfp2_dn15 = 0.0;

        locals.var_qsfp2 = 0.0;
        locals.var_qsfp2_dn2 = 0.0;
        locals.var_qsfp2_dn3 = 0.0;
        locals.var_qsfp2_dn4 = 0.0;
        locals.var_qsfp2_dn7 = 0.0;
        locals.var_qsfp2_dn14 = 0.0;
        locals.var_qsfp2_dn15 = 0.0;

        let assign7430_e8733: f64 = if p.p189 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign7430_e8733;

        let (assign7460_e8745, assign7460_e8745_d_n2, assign7460_e8745_d_n4, assign7460_e8745_d_n7, assign7460_e8745_d_n14, assign7460_e8745_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qgsout, locals.var_fn97_calc_iq__qgsout_dn2, locals.var_fn97_calc_iq__qgsout_dn4, locals.var_fn97_calc_iq__qgsout_dn7, locals.var_fn97_calc_iq__qgsout_dn14, locals.var_fn97_calc_iq__qgsout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qgsout = assign7460_e8745;
        locals.var_fn97_calc_iq__qgsout_dn2 = assign7460_e8745_d_n2;
        locals.var_fn97_calc_iq__qgsout_dn4 = assign7460_e8745_d_n4;
        locals.var_fn97_calc_iq__qgsout_dn7 = assign7460_e8745_d_n7;
        locals.var_fn97_calc_iq__qgsout_dn14 = assign7460_e8745_d_n14;
        locals.var_fn97_calc_iq__qgsout_dn15 = assign7460_e8745_d_n15;

        let (assign7470_e8749, assign7470_e8749_d_n2, assign7470_e8749_d_n4, assign7470_e8749_d_n7, assign7470_e8749_d_n14, assign7470_e8749_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qgdout, locals.var_fn97_calc_iq__qgdout_dn2, locals.var_fn97_calc_iq__qgdout_dn4, locals.var_fn97_calc_iq__qgdout_dn7, locals.var_fn97_calc_iq__qgdout_dn14, locals.var_fn97_calc_iq__qgdout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qgdout = assign7470_e8749;
        locals.var_fn97_calc_iq__qgdout_dn2 = assign7470_e8749_d_n2;
        locals.var_fn97_calc_iq__qgdout_dn4 = assign7470_e8749_d_n4;
        locals.var_fn97_calc_iq__qgdout_dn7 = assign7470_e8749_d_n7;
        locals.var_fn97_calc_iq__qgdout_dn14 = assign7470_e8749_d_n14;
        locals.var_fn97_calc_iq__qgdout_dn15 = assign7470_e8749_d_n15;

        let (assign7480_e8753, assign7480_e8753_d_n2, assign7480_e8753_d_n3, assign7480_e8753_d_n4, assign7480_e8753_d_n7, assign7480_e8753_d_n14, assign7480_e8753_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qcout, locals.var_fn97_calc_iq__qcout_dn2, locals.var_fn97_calc_iq__qcout_dn3, locals.var_fn97_calc_iq__qcout_dn4, locals.var_fn97_calc_iq__qcout_dn7, locals.var_fn97_calc_iq__qcout_dn14, locals.var_fn97_calc_iq__qcout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qcout = assign7480_e8753;
        locals.var_fn97_calc_iq__qcout_dn2 = assign7480_e8753_d_n2;
        locals.var_fn97_calc_iq__qcout_dn3 = assign7480_e8753_d_n3;
        locals.var_fn97_calc_iq__qcout_dn4 = assign7480_e8753_d_n4;
        locals.var_fn97_calc_iq__qcout_dn7 = assign7480_e8753_d_n7;
        locals.var_fn97_calc_iq__qcout_dn14 = assign7480_e8753_d_n14;
        locals.var_fn97_calc_iq__qcout_dn15 = assign7480_e8753_d_n15;

        let (assign7490_e8757, assign7490_e8757_d_n2, assign7490_e8757_d_n3, assign7490_e8757_d_n4, assign7490_e8757_d_n7, assign7490_e8757_d_n14, assign7490_e8757_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qbout, locals.var_fn97_calc_iq__qbout_dn2, locals.var_fn97_calc_iq__qbout_dn3, locals.var_fn97_calc_iq__qbout_dn4, locals.var_fn97_calc_iq__qbout_dn7, locals.var_fn97_calc_iq__qbout_dn14, locals.var_fn97_calc_iq__qbout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qbout = assign7490_e8757;
        locals.var_fn97_calc_iq__qbout_dn2 = assign7490_e8757_d_n2;
        locals.var_fn97_calc_iq__qbout_dn3 = assign7490_e8757_d_n3;
        locals.var_fn97_calc_iq__qbout_dn4 = assign7490_e8757_d_n4;
        locals.var_fn97_calc_iq__qbout_dn7 = assign7490_e8757_d_n7;
        locals.var_fn97_calc_iq__qbout_dn14 = assign7490_e8757_d_n14;
        locals.var_fn97_calc_iq__qbout_dn15 = assign7490_e8757_d_n15;

        let (assign7500_e8761, assign7500_e8761_d_n2, assign7500_e8761_d_n3, assign7500_e8761_d_n4, assign7500_e8761_d_n7, assign7500_e8761_d_n14, assign7500_e8761_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qsout, locals.var_fn97_calc_iq__qsout_dn2, locals.var_fn97_calc_iq__qsout_dn3, locals.var_fn97_calc_iq__qsout_dn4, locals.var_fn97_calc_iq__qsout_dn7, locals.var_fn97_calc_iq__qsout_dn14, locals.var_fn97_calc_iq__qsout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qsout = assign7500_e8761;
        locals.var_fn97_calc_iq__qsout_dn2 = assign7500_e8761_d_n2;
        locals.var_fn97_calc_iq__qsout_dn3 = assign7500_e8761_d_n3;
        locals.var_fn97_calc_iq__qsout_dn4 = assign7500_e8761_d_n4;
        locals.var_fn97_calc_iq__qsout_dn7 = assign7500_e8761_d_n7;
        locals.var_fn97_calc_iq__qsout_dn14 = assign7500_e8761_d_n14;
        locals.var_fn97_calc_iq__qsout_dn15 = assign7500_e8761_d_n15;

        let (assign7510_e8765, assign7510_e8765_d_n4, assign7510_e8765_d_n14, assign7510_e8765_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vtdibl, locals.var_fn97_calc_iq__vtdibl_dn4, locals.var_fn97_calc_iq__vtdibl_dn14, locals.var_fn97_calc_iq__vtdibl_dn15,)
    }
};
        locals.var_fn97_calc_iq__vtdibl = assign7510_e8765;
        locals.var_fn97_calc_iq__vtdibl_dn4 = assign7510_e8765_d_n4;
        locals.var_fn97_calc_iq__vtdibl_dn14 = assign7510_e8765_d_n14;
        locals.var_fn97_calc_iq__vtdibl_dn15 = assign7510_e8765_d_n15;

        let (assign7520_e8769, assign7520_e8769_d_n2, assign7520_e8769_d_n3, assign7520_e8769_d_n4, assign7520_e8769_d_n7, assign7520_e8769_d_n14, assign7520_e8769_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsat1, locals.var_fn97_calc_iq__vdsat1_dn2, locals.var_fn97_calc_iq__vdsat1_dn3, locals.var_fn97_calc_iq__vdsat1_dn4, locals.var_fn97_calc_iq__vdsat1_dn7, locals.var_fn97_calc_iq__vdsat1_dn14, locals.var_fn97_calc_iq__vdsat1_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat1 = assign7520_e8769;
        locals.var_fn97_calc_iq__vdsat1_dn2 = assign7520_e8769_d_n2;
        locals.var_fn97_calc_iq__vdsat1_dn3 = assign7520_e8769_d_n3;
        locals.var_fn97_calc_iq__vdsat1_dn4 = assign7520_e8769_d_n4;
        locals.var_fn97_calc_iq__vdsat1_dn7 = assign7520_e8769_d_n7;
        locals.var_fn97_calc_iq__vdsat1_dn14 = assign7520_e8769_d_n14;
        locals.var_fn97_calc_iq__vdsat1_dn15 = assign7520_e8769_d_n15;

        let (assign7530_e8773, assign7530_e8773_d_n2, assign7530_e8773_d_n7, assign7530_e8773_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_vgsfp2, locals.var_vgsfp2_dn2, locals.var_vgsfp2_dn7, locals.var_vgsfp2_dn14,)
    } else {
        (locals.var_fn97_calc_iq__vgsin, locals.var_fn97_calc_iq__vgsin_dn2, locals.var_fn97_calc_iq__vgsin_dn7, locals.var_fn97_calc_iq__vgsin_dn14,)
    }
};
        locals.var_fn97_calc_iq__vgsin = assign7530_e8773;
        locals.var_fn97_calc_iq__vgsin_dn2 = assign7530_e8773_d_n2;
        locals.var_fn97_calc_iq__vgsin_dn7 = assign7530_e8773_d_n7;
        locals.var_fn97_calc_iq__vgsin_dn14 = assign7530_e8773_d_n14;

        let (assign7540_e8777, assign7540_e8777_d_n14, assign7540_e8777_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_vdsfp2, locals.var_vdsfp2_dn14, locals.var_vdsfp2_dn15,)
    } else {
        (locals.var_fn97_calc_iq__vdsin, locals.var_fn97_calc_iq__vdsin_dn14, locals.var_fn97_calc_iq__vdsin_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsin = assign7540_e8777;
        locals.var_fn97_calc_iq__vdsin_dn14 = assign7540_e8777_d_n14;
        locals.var_fn97_calc_iq__vdsin_dn15 = assign7540_e8777_d_n15;

        let (assign7550_e8781,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p195,)
    } else {
        (locals.var_fn97_calc_iq__qcbflag,)
    }
};
        locals.var_fn97_calc_iq__qcbflag = assign7550_e8781;

        let (assign7560_e8785, assign7560_e8785_d_n2, assign7560_e8785_d_n7, assign7560_e8785_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_vcfp2, locals.var_vcfp2_dn2, locals.var_vcfp2_dn7, locals.var_vcfp2_dn14,)
    } else {
        (locals.var_fn97_calc_iq__vcin, locals.var_fn97_calc_iq__vcin_dn2, locals.var_fn97_calc_iq__vcin_dn7, locals.var_fn97_calc_iq__vcin_dn14,)
    }
};
        locals.var_fn97_calc_iq__vcin = assign7560_e8785;
        locals.var_fn97_calc_iq__vcin_dn2 = assign7560_e8785_d_n2;
        locals.var_fn97_calc_iq__vcin_dn7 = assign7560_e8785_d_n7;
        locals.var_fn97_calc_iq__vcin_dn14 = assign7560_e8785_d_n14;

        let (assign7570_e8789, assign7570_e8789_d_n3, assign7570_e8789_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_vbfp2, locals.var_vbfp2_dn3, locals.var_vbfp2_dn14,)
    } else {
        (locals.var_fn97_calc_iq__vbin, locals.var_fn97_calc_iq__vbin_dn3, locals.var_fn97_calc_iq__vbin_dn14,)
    }
};
        locals.var_fn97_calc_iq__vbin = assign7570_e8789;
        locals.var_fn97_calc_iq__vbin_dn3 = assign7570_e8789_d_n3;
        locals.var_fn97_calc_iq__vbin_dn14 = assign7570_e8789_d_n14;

        let (assign7580_e8793,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p193,)
    } else {
        (locals.var_fn97_calc_iq__qgsflag,)
    }
};
        locals.var_fn97_calc_iq__qgsflag = assign7580_e8793;

        let (assign7590_e8797, assign7590_e8797_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn97_calc_iq__tambin, locals.var_fn97_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn97_calc_iq__tambin = assign7590_e8797;
        locals.var_fn97_calc_iq__tambin_dn4 = assign7590_e8797_d_n4;

        let (assign7600_e8801,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn97_calc_iq__tnomin,)
    }
};
        locals.var_fn97_calc_iq__tnomin = assign7600_e8801;

        let (assign7610_e8805, assign7610_e8805_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn97_calc_iq__phitin, locals.var_fn97_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn97_calc_iq__phitin = assign7610_e8805;
        locals.var_fn97_calc_iq__phitin_dn4 = assign7610_e8805_d_n4;

        let (assign7620_e8809,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn97_calc_iq__w,)
    }
};
        locals.var_fn97_calc_iq__w = assign7620_e8809;

        let (assign7630_e8813,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p189,)
    } else {
        (locals.var_fn97_calc_iq__lin,)
    }
};
        locals.var_fn97_calc_iq__lin = assign7630_e8813;

        let (assign7640_e8817, assign7640_e8817_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_cgfp2t, locals.var_cgfp2t_dn4,)
    } else {
        (locals.var_fn97_calc_iq__cgin, locals.var_fn97_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn97_calc_iq__cgin = assign7640_e8817;
        locals.var_fn97_calc_iq__cgin_dn4 = assign7640_e8817_d_n4;

        let (assign7650_e8821,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p194,)
    } else {
        (locals.var_fn97_calc_iq__cs,)
    }
};
        locals.var_fn97_calc_iq__cs = assign7650_e8821;

        let (assign7660_e8825, assign7660_e8825_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_ccfp2t, locals.var_ccfp2t_dn4,)
    } else {
        (locals.var_fn97_calc_iq__cc, locals.var_fn97_calc_iq__cc_dn4,)
    }
};
        locals.var_fn97_calc_iq__cc = assign7660_e8825;
        locals.var_fn97_calc_iq__cc_dn4 = assign7660_e8825_d_n4;

        let (assign7670_e8829, assign7670_e8829_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_cbfp2t, locals.var_cbfp2t_dn4,)
    } else {
        (locals.var_fn97_calc_iq__cb, locals.var_fn97_calc_iq__cb_dn4,)
    }
};
        locals.var_fn97_calc_iq__cb = assign7670_e8829;
        locals.var_fn97_calc_iq__cb_dn4 = assign7670_e8829_d_n4;

        let (assign7680_e8833,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p190,)
    } else {
        (locals.var_fn97_calc_iq__vto,)
    }
};
        locals.var_fn97_calc_iq__vto = assign7680_e8833;

        let (assign7690_e8837,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p204,)
    } else {
        (locals.var_fn97_calc_iq__ss,)
    }
};
        locals.var_fn97_calc_iq__ss = assign7690_e8837;

        let (assign7700_e8841,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p203,)
    } else {
        (locals.var_fn97_calc_iq__delta1,)
    }
};
        locals.var_fn97_calc_iq__delta1 = assign7700_e8841;

        let (assign7710_e8845,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn97_calc_iq__delta2,)
    }
};
        locals.var_fn97_calc_iq__delta2 = assign7710_e8845;

        let (assign7720_e8849,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p205,)
    } else {
        (locals.var_fn97_calc_iq__nd,)
    }
};
        locals.var_fn97_calc_iq__nd = assign7720_e8849;

        let (assign7730_e8853,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p209,)
    } else {
        (locals.var_fn97_calc_iq__alpha,)
    }
};
        locals.var_fn97_calc_iq__alpha = assign7730_e8853;

        let (assign7740_e8857,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p200,)
    } else {
        (locals.var_fn97_calc_iq__vel0,)
    }
};
        locals.var_fn97_calc_iq__vel0 = assign7740_e8857;

        let (assign7750_e8861,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p201,)
    } else {
        (locals.var_fn97_calc_iq__mu0,)
    }
};
        locals.var_fn97_calc_iq__mu0 = assign7750_e8861;

        let (assign7760_e8865,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p202,)
    } else {
        (locals.var_fn97_calc_iq__beta,)
    }
};
        locals.var_fn97_calc_iq__beta = assign7760_e8865;

        let (assign7770_e8869,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p208,)
    } else {
        (locals.var_fn97_calc_iq__mtheta,)
    }
};
        locals.var_fn97_calc_iq__mtheta = assign7770_e8869;

        let (assign7780_e8873,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p207,)
    } else {
        (locals.var_fn97_calc_iq__vtheta,)
    }
};
        locals.var_fn97_calc_iq__vtheta = assign7780_e8873;

        let (assign7790_e8877,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p206,)
    } else {
        (locals.var_fn97_calc_iq__vtzeta,)
    }
};
        locals.var_fn97_calc_iq__vtzeta = assign7790_e8877;

        let (assign7800_e8881,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn97_calc_iq__dibsat,)
    }
};
        locals.var_fn97_calc_iq__dibsat = assign7800_e8881;

        let (assign7810_e8885,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn97_calc_iq__epsilon,)
    }
};
        locals.var_fn97_calc_iq__epsilon = assign7810_e8885;

    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7820_e8889,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn97_calc_iq__vzeta,)
    }
};
        locals.var_fn97_calc_iq__vzeta = assign7820_e8889;

        let (assign7830_e8893,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn97_calc_iq__lambda,)
    }
};
        locals.var_fn97_calc_iq__lambda = assign7830_e8893;

        let (assign7840_e8897,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn97_calc_iq__ngf,)
    }
};
        locals.var_fn97_calc_iq__ngf = assign7840_e8897;

        let (assign7850_e8901,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn97_calc_iq__type,)
    }
};
        locals.var_fn97_calc_iq__type = assign7850_e8901;

        let (assign7860_e8905,) = {
    if (locals.var_guard96 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn97_calc_iq__trapfracdl,)
    }
};
        locals.var_fn97_calc_iq__trapfracdl = assign7860_e8905;

        let (assign7870_e8909, assign7870_e8909_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__alpha_phit, locals.var_fn97_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn97_calc_iq__alpha_phit = assign7870_e8909;
        locals.var_fn97_calc_iq__alpha_phit_dn4 = assign7870_e8909_d_n4;

        let (assign7880_e8913, assign7880_e8913_d_n14, assign7880_e8913_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__delta, locals.var_fn97_calc_iq__delta_dn14, locals.var_fn97_calc_iq__delta_dn15,)
    }
};
        locals.var_fn97_calc_iq__delta = assign7880_e8913;
        locals.var_fn97_calc_iq__delta_dn14 = assign7880_e8913_d_n14;
        locals.var_fn97_calc_iq__delta_dn15 = assign7880_e8913_d_n15;

        let (assign7890_e8917, assign7890_e8917_d_n4, assign7890_e8917_d_n14, assign7890_e8917_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__n, locals.var_fn97_calc_iq__n_dn4, locals.var_fn97_calc_iq__n_dn14, locals.var_fn97_calc_iq__n_dn15,)
    }
};
        locals.var_fn97_calc_iq__n = assign7890_e8917;
        locals.var_fn97_calc_iq__n_dn4 = assign7890_e8917_d_n4;
        locals.var_fn97_calc_iq__n_dn14 = assign7890_e8917_d_n14;
        locals.var_fn97_calc_iq__n_dn15 = assign7890_e8917_d_n15;

        let (assign7900_e8921, assign7900_e8921_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vtof, locals.var_fn97_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn97_calc_iq__vtof = assign7900_e8921;
        locals.var_fn97_calc_iq__vtof_dn4 = assign7900_e8921_d_n4;

        let (assign7910_e8925, assign7910_e8925_d_n14, assign7910_e8925_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vsatdibl, locals.var_fn97_calc_iq__vsatdibl_dn14, locals.var_fn97_calc_iq__vsatdibl_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsatdibl = assign7910_e8925;
        locals.var_fn97_calc_iq__vsatdibl_dn14 = assign7910_e8925_d_n14;
        locals.var_fn97_calc_iq__vsatdibl_dn15 = assign7910_e8925_d_n15;

        let (assign7920_e8929, assign7920_e8929_d_n2, assign7920_e8929_d_n3, assign7920_e8929_d_n4, assign7920_e8929_d_n7, assign7920_e8929_d_n14, assign7920_e8929_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs, locals.var_fn97_calc_iq__ffs_dn2, locals.var_fn97_calc_iq__ffs_dn3, locals.var_fn97_calc_iq__ffs_dn4, locals.var_fn97_calc_iq__ffs_dn7, locals.var_fn97_calc_iq__ffs_dn14, locals.var_fn97_calc_iq__ffs_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs = assign7920_e8929;
        locals.var_fn97_calc_iq__ffs_dn2 = assign7920_e8929_d_n2;
        locals.var_fn97_calc_iq__ffs_dn3 = assign7920_e8929_d_n3;
        locals.var_fn97_calc_iq__ffs_dn4 = assign7920_e8929_d_n4;
        locals.var_fn97_calc_iq__ffs_dn7 = assign7920_e8929_d_n7;
        locals.var_fn97_calc_iq__ffs_dn14 = assign7920_e8929_d_n14;
        locals.var_fn97_calc_iq__ffs_dn15 = assign7920_e8929_d_n15;

        let (assign7930_e8933, assign7930_e8933_d_n4, assign7930_e8933_d_n14, assign7930_e8933_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__two_n_phit, locals.var_fn97_calc_iq__two_n_phit_dn4, locals.var_fn97_calc_iq__two_n_phit_dn14, locals.var_fn97_calc_iq__two_n_phit_dn15,)
    }
};
        locals.var_fn97_calc_iq__two_n_phit = assign7930_e8933;
        locals.var_fn97_calc_iq__two_n_phit_dn4 = assign7930_e8933_d_n4;
        locals.var_fn97_calc_iq__two_n_phit_dn14 = assign7930_e8933_d_n14;
        locals.var_fn97_calc_iq__two_n_phit_dn15 = assign7930_e8933_d_n15;

        let (assign7940_e8937, assign7940_e8937_d_n4, assign7940_e8937_d_n14, assign7940_e8937_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qref, locals.var_fn97_calc_iq__qref_dn4, locals.var_fn97_calc_iq__qref_dn14, locals.var_fn97_calc_iq__qref_dn15,)
    }
};
        locals.var_fn97_calc_iq__qref = assign7940_e8937;
        locals.var_fn97_calc_iq__qref_dn4 = assign7940_e8937_d_n4;
        locals.var_fn97_calc_iq__qref_dn14 = assign7940_e8937_d_n14;
        locals.var_fn97_calc_iq__qref_dn15 = assign7940_e8937_d_n15;

        let (assign7950_e8941, assign7950_e8941_d_n2, assign7950_e8941_d_n3, assign7950_e8941_d_n4, assign7950_e8941_d_n7, assign7950_e8941_d_n14, assign7950_e8941_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etas, locals.var_fn97_calc_iq__etas_dn2, locals.var_fn97_calc_iq__etas_dn3, locals.var_fn97_calc_iq__etas_dn4, locals.var_fn97_calc_iq__etas_dn7, locals.var_fn97_calc_iq__etas_dn14, locals.var_fn97_calc_iq__etas_dn15,)
    }
};
        locals.var_fn97_calc_iq__etas = assign7950_e8941;
        locals.var_fn97_calc_iq__etas_dn2 = assign7950_e8941_d_n2;
        locals.var_fn97_calc_iq__etas_dn3 = assign7950_e8941_d_n3;
        locals.var_fn97_calc_iq__etas_dn4 = assign7950_e8941_d_n4;
        locals.var_fn97_calc_iq__etas_dn7 = assign7950_e8941_d_n7;
        locals.var_fn97_calc_iq__etas_dn14 = assign7950_e8941_d_n14;
        locals.var_fn97_calc_iq__etas_dn15 = assign7950_e8941_d_n15;

        let (assign7960_e8945, assign7960_e8945_d_n2, assign7960_e8945_d_n3, assign7960_e8945_d_n4, assign7960_e8945_d_n7, assign7960_e8945_d_n14, assign7960_e8945_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvs, locals.var_fn97_calc_iq__qinvs_dn2, locals.var_fn97_calc_iq__qinvs_dn3, locals.var_fn97_calc_iq__qinvs_dn4, locals.var_fn97_calc_iq__qinvs_dn7, locals.var_fn97_calc_iq__qinvs_dn14, locals.var_fn97_calc_iq__qinvs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs = assign7960_e8945;
        locals.var_fn97_calc_iq__qinvs_dn2 = assign7960_e8945_d_n2;
        locals.var_fn97_calc_iq__qinvs_dn3 = assign7960_e8945_d_n3;
        locals.var_fn97_calc_iq__qinvs_dn4 = assign7960_e8945_d_n4;
        locals.var_fn97_calc_iq__qinvs_dn7 = assign7960_e8945_d_n7;
        locals.var_fn97_calc_iq__qinvs_dn14 = assign7960_e8945_d_n14;
        locals.var_fn97_calc_iq__qinvs_dn15 = assign7960_e8945_d_n15;

        let (assign7970_e8949, assign7970_e8949_d_n2, assign7970_e8949_d_n3, assign7970_e8949_d_n4, assign7970_e8949_d_n7, assign7970_e8949_d_n14, assign7970_e8949_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__muf, locals.var_fn97_calc_iq__muf_dn2, locals.var_fn97_calc_iq__muf_dn3, locals.var_fn97_calc_iq__muf_dn4, locals.var_fn97_calc_iq__muf_dn7, locals.var_fn97_calc_iq__muf_dn14, locals.var_fn97_calc_iq__muf_dn15,)
    }
};
        locals.var_fn97_calc_iq__muf = assign7970_e8949;
        locals.var_fn97_calc_iq__muf_dn2 = assign7970_e8949_d_n2;
        locals.var_fn97_calc_iq__muf_dn3 = assign7970_e8949_d_n3;
        locals.var_fn97_calc_iq__muf_dn4 = assign7970_e8949_d_n4;
        locals.var_fn97_calc_iq__muf_dn7 = assign7970_e8949_d_n7;
        locals.var_fn97_calc_iq__muf_dn14 = assign7970_e8949_d_n14;
        locals.var_fn97_calc_iq__muf_dn15 = assign7970_e8949_d_n15;

        let (assign7980_e8953, assign7980_e8953_d_n2, assign7980_e8953_d_n3, assign7980_e8953_d_n4, assign7980_e8953_d_n7, assign7980_e8953_d_n14, assign7980_e8953_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vx, locals.var_fn97_calc_iq__vx_dn2, locals.var_fn97_calc_iq__vx_dn3, locals.var_fn97_calc_iq__vx_dn4, locals.var_fn97_calc_iq__vx_dn7, locals.var_fn97_calc_iq__vx_dn14, locals.var_fn97_calc_iq__vx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vx = assign7980_e8953;
        locals.var_fn97_calc_iq__vx_dn2 = assign7980_e8953_d_n2;
        locals.var_fn97_calc_iq__vx_dn3 = assign7980_e8953_d_n3;
        locals.var_fn97_calc_iq__vx_dn4 = assign7980_e8953_d_n4;
        locals.var_fn97_calc_iq__vx_dn7 = assign7980_e8953_d_n7;
        locals.var_fn97_calc_iq__vx_dn14 = assign7980_e8953_d_n14;
        locals.var_fn97_calc_iq__vx_dn15 = assign7980_e8953_d_n15;

        let (assign8000_e8961, assign8000_e8961_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__n0, locals.var_fn97_calc_iq__n0_dn4,)
    }
};
        locals.var_fn97_calc_iq__n0 = assign8000_e8961;
        locals.var_fn97_calc_iq__n0_dn4 = assign8000_e8961_d_n4;

        let (assign8010_e8965, assign8010_e8965_d_n2, assign8010_e8965_d_n4, assign8010_e8965_d_n7, assign8010_e8965_d_n14, assign8010_e8965_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs0, locals.var_fn97_calc_iq__ffs0_dn2, locals.var_fn97_calc_iq__ffs0_dn4, locals.var_fn97_calc_iq__ffs0_dn7, locals.var_fn97_calc_iq__ffs0_dn14, locals.var_fn97_calc_iq__ffs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs0 = assign8010_e8965;
        locals.var_fn97_calc_iq__ffs0_dn2 = assign8010_e8965_d_n2;
        locals.var_fn97_calc_iq__ffs0_dn4 = assign8010_e8965_d_n4;
        locals.var_fn97_calc_iq__ffs0_dn7 = assign8010_e8965_d_n7;
        locals.var_fn97_calc_iq__ffs0_dn14 = assign8010_e8965_d_n14;
        locals.var_fn97_calc_iq__ffs0_dn15 = assign8010_e8965_d_n15;

        let (assign8020_e8969, assign8020_e8969_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__two_n_phit0, locals.var_fn97_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn97_calc_iq__two_n_phit0 = assign8020_e8969;
        locals.var_fn97_calc_iq__two_n_phit0_dn4 = assign8020_e8969_d_n4;

        let (assign8030_e8973, assign8030_e8973_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qref0, locals.var_fn97_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn97_calc_iq__qref0 = assign8030_e8973;
        locals.var_fn97_calc_iq__qref0_dn4 = assign8030_e8973_d_n4;

        let (assign8040_e8977, assign8040_e8977_d_n2, assign8040_e8977_d_n4, assign8040_e8977_d_n7, assign8040_e8977_d_n14, assign8040_e8977_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etas0, locals.var_fn97_calc_iq__etas0_dn2, locals.var_fn97_calc_iq__etas0_dn4, locals.var_fn97_calc_iq__etas0_dn7, locals.var_fn97_calc_iq__etas0_dn14, locals.var_fn97_calc_iq__etas0_dn15,)
    }
};
        locals.var_fn97_calc_iq__etas0 = assign8040_e8977;
        locals.var_fn97_calc_iq__etas0_dn2 = assign8040_e8977_d_n2;
        locals.var_fn97_calc_iq__etas0_dn4 = assign8040_e8977_d_n4;
        locals.var_fn97_calc_iq__etas0_dn7 = assign8040_e8977_d_n7;
        locals.var_fn97_calc_iq__etas0_dn14 = assign8040_e8977_d_n14;
        locals.var_fn97_calc_iq__etas0_dn15 = assign8040_e8977_d_n15;

        let (assign8050_e8981, assign8050_e8981_d_n2, assign8050_e8981_d_n4, assign8050_e8981_d_n7, assign8050_e8981_d_n14, assign8050_e8981_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvs0, locals.var_fn97_calc_iq__qinvs0_dn2, locals.var_fn97_calc_iq__qinvs0_dn4, locals.var_fn97_calc_iq__qinvs0_dn7, locals.var_fn97_calc_iq__qinvs0_dn14, locals.var_fn97_calc_iq__qinvs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs0 = assign8050_e8981;
        locals.var_fn97_calc_iq__qinvs0_dn2 = assign8050_e8981_d_n2;
        locals.var_fn97_calc_iq__qinvs0_dn4 = assign8050_e8981_d_n4;
        locals.var_fn97_calc_iq__qinvs0_dn7 = assign8050_e8981_d_n7;
        locals.var_fn97_calc_iq__qinvs0_dn14 = assign8050_e8981_d_n14;
        locals.var_fn97_calc_iq__qinvs0_dn15 = assign8050_e8981_d_n15;

        let (assign8060_e8985, assign8060_e8985_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__muf0, locals.var_fn97_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn97_calc_iq__muf0 = assign8060_e8985;
        locals.var_fn97_calc_iq__muf0_dn4 = assign8060_e8985_d_n4;

        let (assign8070_e8989, assign8070_e8989_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vx0, locals.var_fn97_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn97_calc_iq__vx0 = assign8070_e8989;
        locals.var_fn97_calc_iq__vx0_dn4 = assign8070_e8989_d_n4;

        let (assign8080_e8993, assign8080_e8993_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__tfacmobin, locals.var_fn97_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn97_calc_iq__tfacmobin = assign8080_e8993;
        locals.var_fn97_calc_iq__tfacmobin_dn4 = assign8080_e8993_d_n4;

        let (assign8090_e8997, assign8090_e8997_d_n2, assign8090_e8997_d_n3, assign8090_e8997_d_n4, assign8090_e8997_d_n7, assign8090_e8997_d_n14, assign8090_e8997_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff, locals.var_fn97_calc_iq__ff_dn2, locals.var_fn97_calc_iq__ff_dn3, locals.var_fn97_calc_iq__ff_dn4, locals.var_fn97_calc_iq__ff_dn7, locals.var_fn97_calc_iq__ff_dn14, locals.var_fn97_calc_iq__ff_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff = assign8090_e8997;
        locals.var_fn97_calc_iq__ff_dn2 = assign8090_e8997_d_n2;
        locals.var_fn97_calc_iq__ff_dn3 = assign8090_e8997_d_n3;
        locals.var_fn97_calc_iq__ff_dn4 = assign8090_e8997_d_n4;
        locals.var_fn97_calc_iq__ff_dn7 = assign8090_e8997_d_n7;
        locals.var_fn97_calc_iq__ff_dn14 = assign8090_e8997_d_n14;
        locals.var_fn97_calc_iq__ff_dn15 = assign8090_e8997_d_n15;

        let (assign8100_e9001, assign8100_e9001_d_n2, assign8100_e9001_d_n3, assign8100_e9001_d_n4, assign8100_e9001_d_n7, assign8100_e9001_d_n14, assign8100_e9001_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__eta, locals.var_fn97_calc_iq__eta_dn2, locals.var_fn97_calc_iq__eta_dn3, locals.var_fn97_calc_iq__eta_dn4, locals.var_fn97_calc_iq__eta_dn7, locals.var_fn97_calc_iq__eta_dn14, locals.var_fn97_calc_iq__eta_dn15,)
    }
};
        locals.var_fn97_calc_iq__eta = assign8100_e9001;
        locals.var_fn97_calc_iq__eta_dn2 = assign8100_e9001_d_n2;
        locals.var_fn97_calc_iq__eta_dn3 = assign8100_e9001_d_n3;
        locals.var_fn97_calc_iq__eta_dn4 = assign8100_e9001_d_n4;
        locals.var_fn97_calc_iq__eta_dn7 = assign8100_e9001_d_n7;
        locals.var_fn97_calc_iq__eta_dn14 = assign8100_e9001_d_n14;
        locals.var_fn97_calc_iq__eta_dn15 = assign8100_e9001_d_n15;

        let (assign8110_e9005, assign8110_e9005_d_n2, assign8110_e9005_d_n3, assign8110_e9005_d_n4, assign8110_e9005_d_n7, assign8110_e9005_d_n14, assign8110_e9005_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvv, locals.var_fn97_calc_iq__qinvv_dn2, locals.var_fn97_calc_iq__qinvv_dn3, locals.var_fn97_calc_iq__qinvv_dn4, locals.var_fn97_calc_iq__qinvv_dn7, locals.var_fn97_calc_iq__qinvv_dn14, locals.var_fn97_calc_iq__qinvv_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv = assign8110_e9005;
        locals.var_fn97_calc_iq__qinvv_dn2 = assign8110_e9005_d_n2;
        locals.var_fn97_calc_iq__qinvv_dn3 = assign8110_e9005_d_n3;
        locals.var_fn97_calc_iq__qinvv_dn4 = assign8110_e9005_d_n4;
        locals.var_fn97_calc_iq__qinvv_dn7 = assign8110_e9005_d_n7;
        locals.var_fn97_calc_iq__qinvv_dn14 = assign8110_e9005_d_n14;
        locals.var_fn97_calc_iq__qinvv_dn15 = assign8110_e9005_d_n15;

        let (assign8120_e9009, assign8120_e9009_d_n2, assign8120_e9009_d_n4, assign8120_e9009_d_n7, assign8120_e9009_d_n14, assign8120_e9009_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff0, locals.var_fn97_calc_iq__ff0_dn2, locals.var_fn97_calc_iq__ff0_dn4, locals.var_fn97_calc_iq__ff0_dn7, locals.var_fn97_calc_iq__ff0_dn14, locals.var_fn97_calc_iq__ff0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff0 = assign8120_e9009;
        locals.var_fn97_calc_iq__ff0_dn2 = assign8120_e9009_d_n2;
        locals.var_fn97_calc_iq__ff0_dn4 = assign8120_e9009_d_n4;
        locals.var_fn97_calc_iq__ff0_dn7 = assign8120_e9009_d_n7;
        locals.var_fn97_calc_iq__ff0_dn14 = assign8120_e9009_d_n14;
        locals.var_fn97_calc_iq__ff0_dn15 = assign8120_e9009_d_n15;

        let (assign8130_e9013, assign8130_e9013_d_n2, assign8130_e9013_d_n4, assign8130_e9013_d_n7, assign8130_e9013_d_n14, assign8130_e9013_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__eta0, locals.var_fn97_calc_iq__eta0_dn2, locals.var_fn97_calc_iq__eta0_dn4, locals.var_fn97_calc_iq__eta0_dn7, locals.var_fn97_calc_iq__eta0_dn14, locals.var_fn97_calc_iq__eta0_dn15,)
    }
};
        locals.var_fn97_calc_iq__eta0 = assign8130_e9013;
        locals.var_fn97_calc_iq__eta0_dn2 = assign8130_e9013_d_n2;
        locals.var_fn97_calc_iq__eta0_dn4 = assign8130_e9013_d_n4;
        locals.var_fn97_calc_iq__eta0_dn7 = assign8130_e9013_d_n7;
        locals.var_fn97_calc_iq__eta0_dn14 = assign8130_e9013_d_n14;
        locals.var_fn97_calc_iq__eta0_dn15 = assign8130_e9013_d_n15;

        let (assign8140_e9017, assign8140_e9017_d_n2, assign8140_e9017_d_n4, assign8140_e9017_d_n7, assign8140_e9017_d_n14, assign8140_e9017_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvv0, locals.var_fn97_calc_iq__qinvv0_dn2, locals.var_fn97_calc_iq__qinvv0_dn4, locals.var_fn97_calc_iq__qinvv0_dn7, locals.var_fn97_calc_iq__qinvv0_dn14, locals.var_fn97_calc_iq__qinvv0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv0 = assign8140_e9017;
        locals.var_fn97_calc_iq__qinvv0_dn2 = assign8140_e9017_d_n2;
        locals.var_fn97_calc_iq__qinvv0_dn4 = assign8140_e9017_d_n4;
        locals.var_fn97_calc_iq__qinvv0_dn7 = assign8140_e9017_d_n7;
        locals.var_fn97_calc_iq__qinvv0_dn14 = assign8140_e9017_d_n14;
        locals.var_fn97_calc_iq__qinvv0_dn15 = assign8140_e9017_d_n15;

        let (assign8150_e9021, assign8150_e9021_d_n2, assign8150_e9021_d_n3, assign8150_e9021_d_n4, assign8150_e9021_d_n7, assign8150_e9021_d_n14, assign8150_e9021_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsats, locals.var_fn97_calc_iq__vdsats_dn2, locals.var_fn97_calc_iq__vdsats_dn3, locals.var_fn97_calc_iq__vdsats_dn4, locals.var_fn97_calc_iq__vdsats_dn7, locals.var_fn97_calc_iq__vdsats_dn14, locals.var_fn97_calc_iq__vdsats_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats = assign8150_e9021;
        locals.var_fn97_calc_iq__vdsats_dn2 = assign8150_e9021_d_n2;
        locals.var_fn97_calc_iq__vdsats_dn3 = assign8150_e9021_d_n3;
        locals.var_fn97_calc_iq__vdsats_dn4 = assign8150_e9021_d_n4;
        locals.var_fn97_calc_iq__vdsats_dn7 = assign8150_e9021_d_n7;
        locals.var_fn97_calc_iq__vdsats_dn14 = assign8150_e9021_d_n14;
        locals.var_fn97_calc_iq__vdsats_dn15 = assign8150_e9021_d_n15;

        let (assign8160_e9025, assign8160_e9025_d_n2, assign8160_e9025_d_n3, assign8160_e9025_d_n4, assign8160_e9025_d_n7, assign8160_e9025_d_n14, assign8160_e9025_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsats1, locals.var_fn97_calc_iq__vdsats1_dn2, locals.var_fn97_calc_iq__vdsats1_dn3, locals.var_fn97_calc_iq__vdsats1_dn4, locals.var_fn97_calc_iq__vdsats1_dn7, locals.var_fn97_calc_iq__vdsats1_dn14, locals.var_fn97_calc_iq__vdsats1_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats1 = assign8160_e9025;
        locals.var_fn97_calc_iq__vdsats1_dn2 = assign8160_e9025_d_n2;
        locals.var_fn97_calc_iq__vdsats1_dn3 = assign8160_e9025_d_n3;
        locals.var_fn97_calc_iq__vdsats1_dn4 = assign8160_e9025_d_n4;
        locals.var_fn97_calc_iq__vdsats1_dn7 = assign8160_e9025_d_n7;
        locals.var_fn97_calc_iq__vdsats1_dn14 = assign8160_e9025_d_n14;
        locals.var_fn97_calc_iq__vdsats1_dn15 = assign8160_e9025_d_n15;

        let (assign8170_e9029, assign8170_e9029_d_n2, assign8170_e9029_d_n3, assign8170_e9029_d_n4, assign8170_e9029_d_n7, assign8170_e9029_d_n14, assign8170_e9029_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsat, locals.var_fn97_calc_iq__vdsat_dn2, locals.var_fn97_calc_iq__vdsat_dn3, locals.var_fn97_calc_iq__vdsat_dn4, locals.var_fn97_calc_iq__vdsat_dn7, locals.var_fn97_calc_iq__vdsat_dn14, locals.var_fn97_calc_iq__vdsat_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat = assign8170_e9029;
        locals.var_fn97_calc_iq__vdsat_dn2 = assign8170_e9029_d_n2;
        locals.var_fn97_calc_iq__vdsat_dn3 = assign8170_e9029_d_n3;
        locals.var_fn97_calc_iq__vdsat_dn4 = assign8170_e9029_d_n4;
        locals.var_fn97_calc_iq__vdsat_dn7 = assign8170_e9029_d_n7;
        locals.var_fn97_calc_iq__vdsat_dn14 = assign8170_e9029_d_n14;
        locals.var_fn97_calc_iq__vdsat_dn15 = assign8170_e9029_d_n15;

        let (assign8180_e9033, assign8180_e9033_d_n2, assign8180_e9033_d_n3, assign8180_e9033_d_n4, assign8180_e9033_d_n7, assign8180_e9033_d_n14, assign8180_e9033_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__fsd, locals.var_fn97_calc_iq__fsd_dn2, locals.var_fn97_calc_iq__fsd_dn3, locals.var_fn97_calc_iq__fsd_dn4, locals.var_fn97_calc_iq__fsd_dn7, locals.var_fn97_calc_iq__fsd_dn14, locals.var_fn97_calc_iq__fsd_dn15,)
    }
};
        locals.var_fn97_calc_iq__fsd = assign8180_e9033;
        locals.var_fn97_calc_iq__fsd_dn2 = assign8180_e9033_d_n2;
        locals.var_fn97_calc_iq__fsd_dn3 = assign8180_e9033_d_n3;
        locals.var_fn97_calc_iq__fsd_dn4 = assign8180_e9033_d_n4;
        locals.var_fn97_calc_iq__fsd_dn7 = assign8180_e9033_d_n7;
        locals.var_fn97_calc_iq__fsd_dn14 = assign8180_e9033_d_n14;
        locals.var_fn97_calc_iq__fsd_dn15 = assign8180_e9033_d_n15;

        let (assign8190_e9037, assign8190_e9037_d_n2, assign8190_e9037_d_n3, assign8190_e9037_d_n4, assign8190_e9037_d_n7, assign8190_e9037_d_n14, assign8190_e9037_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdx, locals.var_fn97_calc_iq__vdx_dn2, locals.var_fn97_calc_iq__vdx_dn3, locals.var_fn97_calc_iq__vdx_dn4, locals.var_fn97_calc_iq__vdx_dn7, locals.var_fn97_calc_iq__vdx_dn14, locals.var_fn97_calc_iq__vdx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdx = assign8190_e9037;
        locals.var_fn97_calc_iq__vdx_dn2 = assign8190_e9037_d_n2;
        locals.var_fn97_calc_iq__vdx_dn3 = assign8190_e9037_d_n3;
        locals.var_fn97_calc_iq__vdx_dn4 = assign8190_e9037_d_n4;
        locals.var_fn97_calc_iq__vdx_dn7 = assign8190_e9037_d_n7;
        locals.var_fn97_calc_iq__vdx_dn14 = assign8190_e9037_d_n14;
        locals.var_fn97_calc_iq__vdx_dn15 = assign8190_e9037_d_n15;

        let (assign8200_e9041, assign8200_e9041_d_n2, assign8200_e9041_d_n3, assign8200_e9041_d_n4, assign8200_e9041_d_n7, assign8200_e9041_d_n14, assign8200_e9041_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__fds, locals.var_fn97_calc_iq__fds_dn2, locals.var_fn97_calc_iq__fds_dn3, locals.var_fn97_calc_iq__fds_dn4, locals.var_fn97_calc_iq__fds_dn7, locals.var_fn97_calc_iq__fds_dn14, locals.var_fn97_calc_iq__fds_dn15,)
    }
};
        locals.var_fn97_calc_iq__fds = assign8200_e9041;
        locals.var_fn97_calc_iq__fds_dn2 = assign8200_e9041_d_n2;
        locals.var_fn97_calc_iq__fds_dn3 = assign8200_e9041_d_n3;
        locals.var_fn97_calc_iq__fds_dn4 = assign8200_e9041_d_n4;
        locals.var_fn97_calc_iq__fds_dn7 = assign8200_e9041_d_n7;
        locals.var_fn97_calc_iq__fds_dn14 = assign8200_e9041_d_n14;
        locals.var_fn97_calc_iq__fds_dn15 = assign8200_e9041_d_n15;

        let (assign8210_e9045, assign8210_e9045_d_n2, assign8210_e9045_d_n3, assign8210_e9045_d_n4, assign8210_e9045_d_n7, assign8210_e9045_d_n14, assign8210_e9045_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vsx, locals.var_fn97_calc_iq__vsx_dn2, locals.var_fn97_calc_iq__vsx_dn3, locals.var_fn97_calc_iq__vsx_dn4, locals.var_fn97_calc_iq__vsx_dn7, locals.var_fn97_calc_iq__vsx_dn14, locals.var_fn97_calc_iq__vsx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsx = assign8210_e9045;
        locals.var_fn97_calc_iq__vsx_dn2 = assign8210_e9045_d_n2;
        locals.var_fn97_calc_iq__vsx_dn3 = assign8210_e9045_d_n3;
        locals.var_fn97_calc_iq__vsx_dn4 = assign8210_e9045_d_n4;
        locals.var_fn97_calc_iq__vsx_dn7 = assign8210_e9045_d_n7;
        locals.var_fn97_calc_iq__vsx_dn14 = assign8210_e9045_d_n14;
        locals.var_fn97_calc_iq__vsx_dn15 = assign8210_e9045_d_n15;

        let (assign8220_e9049, assign8220_e9049_d_n2, assign8220_e9049_d_n3, assign8220_e9049_d_n4, assign8220_e9049_d_n7, assign8220_e9049_d_n14, assign8220_e9049_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd, locals.var_fn97_calc_iq__ffd_dn2, locals.var_fn97_calc_iq__ffd_dn3, locals.var_fn97_calc_iq__ffd_dn4, locals.var_fn97_calc_iq__ffd_dn7, locals.var_fn97_calc_iq__ffd_dn14, locals.var_fn97_calc_iq__ffd_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd = assign8220_e9049;
        locals.var_fn97_calc_iq__ffd_dn2 = assign8220_e9049_d_n2;
        locals.var_fn97_calc_iq__ffd_dn3 = assign8220_e9049_d_n3;
        locals.var_fn97_calc_iq__ffd_dn4 = assign8220_e9049_d_n4;
        locals.var_fn97_calc_iq__ffd_dn7 = assign8220_e9049_d_n7;
        locals.var_fn97_calc_iq__ffd_dn14 = assign8220_e9049_d_n14;
        locals.var_fn97_calc_iq__ffd_dn15 = assign8220_e9049_d_n15;

    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8230_e9053, assign8230_e9053_d_n2, assign8230_e9053_d_n3, assign8230_e9053_d_n4, assign8230_e9053_d_n7, assign8230_e9053_d_n14, assign8230_e9053_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etad, locals.var_fn97_calc_iq__etad_dn2, locals.var_fn97_calc_iq__etad_dn3, locals.var_fn97_calc_iq__etad_dn4, locals.var_fn97_calc_iq__etad_dn7, locals.var_fn97_calc_iq__etad_dn14, locals.var_fn97_calc_iq__etad_dn15,)
    }
};
        locals.var_fn97_calc_iq__etad = assign8230_e9053;
        locals.var_fn97_calc_iq__etad_dn2 = assign8230_e9053_d_n2;
        locals.var_fn97_calc_iq__etad_dn3 = assign8230_e9053_d_n3;
        locals.var_fn97_calc_iq__etad_dn4 = assign8230_e9053_d_n4;
        locals.var_fn97_calc_iq__etad_dn7 = assign8230_e9053_d_n7;
        locals.var_fn97_calc_iq__etad_dn14 = assign8230_e9053_d_n14;
        locals.var_fn97_calc_iq__etad_dn15 = assign8230_e9053_d_n15;

        let (assign8240_e9057, assign8240_e9057_d_n2, assign8240_e9057_d_n3, assign8240_e9057_d_n4, assign8240_e9057_d_n7, assign8240_e9057_d_n14, assign8240_e9057_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvd, locals.var_fn97_calc_iq__qinvd_dn2, locals.var_fn97_calc_iq__qinvd_dn3, locals.var_fn97_calc_iq__qinvd_dn4, locals.var_fn97_calc_iq__qinvd_dn7, locals.var_fn97_calc_iq__qinvd_dn14, locals.var_fn97_calc_iq__qinvd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd = assign8240_e9057;
        locals.var_fn97_calc_iq__qinvd_dn2 = assign8240_e9057_d_n2;
        locals.var_fn97_calc_iq__qinvd_dn3 = assign8240_e9057_d_n3;
        locals.var_fn97_calc_iq__qinvd_dn4 = assign8240_e9057_d_n4;
        locals.var_fn97_calc_iq__qinvd_dn7 = assign8240_e9057_d_n7;
        locals.var_fn97_calc_iq__qinvd_dn14 = assign8240_e9057_d_n14;
        locals.var_fn97_calc_iq__qinvd_dn15 = assign8240_e9057_d_n15;

        let (assign8250_e9061, assign8250_e9061_d_n2, assign8250_e9061_d_n3, assign8250_e9061_d_n4, assign8250_e9061_d_n7, assign8250_e9061_d_n14, assign8250_e9061_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsc, locals.var_fn97_calc_iq__vdsc_dn2, locals.var_fn97_calc_iq__vdsc_dn3, locals.var_fn97_calc_iq__vdsc_dn4, locals.var_fn97_calc_iq__vdsc_dn7, locals.var_fn97_calc_iq__vdsc_dn14, locals.var_fn97_calc_iq__vdsc_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsc = assign8250_e9061;
        locals.var_fn97_calc_iq__vdsc_dn2 = assign8250_e9061_d_n2;
        locals.var_fn97_calc_iq__vdsc_dn3 = assign8250_e9061_d_n3;
        locals.var_fn97_calc_iq__vdsc_dn4 = assign8250_e9061_d_n4;
        locals.var_fn97_calc_iq__vdsc_dn7 = assign8250_e9061_d_n7;
        locals.var_fn97_calc_iq__vdsc_dn14 = assign8250_e9061_d_n14;
        locals.var_fn97_calc_iq__vdsc_dn15 = assign8250_e9061_d_n15;

        let (assign8280_e9073, assign8280_e9073_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsats0, locals.var_fn97_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn97_calc_iq__vdsats0 = assign8280_e9073;
        locals.var_fn97_calc_iq__vdsats0_dn4 = assign8280_e9073_d_n4;

        let (assign8290_e9077, assign8290_e9077_d_n2, assign8290_e9077_d_n4, assign8290_e9077_d_n7, assign8290_e9077_d_n14, assign8290_e9077_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsats10, locals.var_fn97_calc_iq__vdsats10_dn2, locals.var_fn97_calc_iq__vdsats10_dn4, locals.var_fn97_calc_iq__vdsats10_dn7, locals.var_fn97_calc_iq__vdsats10_dn14, locals.var_fn97_calc_iq__vdsats10_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats10 = assign8290_e9077;
        locals.var_fn97_calc_iq__vdsats10_dn2 = assign8290_e9077_d_n2;
        locals.var_fn97_calc_iq__vdsats10_dn4 = assign8290_e9077_d_n4;
        locals.var_fn97_calc_iq__vdsats10_dn7 = assign8290_e9077_d_n7;
        locals.var_fn97_calc_iq__vdsats10_dn14 = assign8290_e9077_d_n14;
        locals.var_fn97_calc_iq__vdsats10_dn15 = assign8290_e9077_d_n15;

        let (assign8300_e9081, assign8300_e9081_d_n2, assign8300_e9081_d_n4, assign8300_e9081_d_n7, assign8300_e9081_d_n14, assign8300_e9081_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsat10, locals.var_fn97_calc_iq__vdsat10_dn2, locals.var_fn97_calc_iq__vdsat10_dn4, locals.var_fn97_calc_iq__vdsat10_dn7, locals.var_fn97_calc_iq__vdsat10_dn14, locals.var_fn97_calc_iq__vdsat10_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat10 = assign8300_e9081;
        locals.var_fn97_calc_iq__vdsat10_dn2 = assign8300_e9081_d_n2;
        locals.var_fn97_calc_iq__vdsat10_dn4 = assign8300_e9081_d_n4;
        locals.var_fn97_calc_iq__vdsat10_dn7 = assign8300_e9081_d_n7;
        locals.var_fn97_calc_iq__vdsat10_dn14 = assign8300_e9081_d_n14;
        locals.var_fn97_calc_iq__vdsat10_dn15 = assign8300_e9081_d_n15;

        let (assign8310_e9085, assign8310_e9085_d_n2, assign8310_e9085_d_n4, assign8310_e9085_d_n7, assign8310_e9085_d_n14, assign8310_e9085_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__fsd0, locals.var_fn97_calc_iq__fsd0_dn2, locals.var_fn97_calc_iq__fsd0_dn4, locals.var_fn97_calc_iq__fsd0_dn7, locals.var_fn97_calc_iq__fsd0_dn14, locals.var_fn97_calc_iq__fsd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__fsd0 = assign8310_e9085;
        locals.var_fn97_calc_iq__fsd0_dn2 = assign8310_e9085_d_n2;
        locals.var_fn97_calc_iq__fsd0_dn4 = assign8310_e9085_d_n4;
        locals.var_fn97_calc_iq__fsd0_dn7 = assign8310_e9085_d_n7;
        locals.var_fn97_calc_iq__fsd0_dn14 = assign8310_e9085_d_n14;
        locals.var_fn97_calc_iq__fsd0_dn15 = assign8310_e9085_d_n15;

        let (assign8320_e9089, assign8320_e9089_d_n2, assign8320_e9089_d_n4, assign8320_e9089_d_n7, assign8320_e9089_d_n14, assign8320_e9089_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdx0, locals.var_fn97_calc_iq__vdx0_dn2, locals.var_fn97_calc_iq__vdx0_dn4, locals.var_fn97_calc_iq__vdx0_dn7, locals.var_fn97_calc_iq__vdx0_dn14, locals.var_fn97_calc_iq__vdx0_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdx0 = assign8320_e9089;
        locals.var_fn97_calc_iq__vdx0_dn2 = assign8320_e9089_d_n2;
        locals.var_fn97_calc_iq__vdx0_dn4 = assign8320_e9089_d_n4;
        locals.var_fn97_calc_iq__vdx0_dn7 = assign8320_e9089_d_n7;
        locals.var_fn97_calc_iq__vdx0_dn14 = assign8320_e9089_d_n14;
        locals.var_fn97_calc_iq__vdx0_dn15 = assign8320_e9089_d_n15;

        let (assign8330_e9093, assign8330_e9093_d_n2, assign8330_e9093_d_n4, assign8330_e9093_d_n7, assign8330_e9093_d_n14, assign8330_e9093_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__fds0, locals.var_fn97_calc_iq__fds0_dn2, locals.var_fn97_calc_iq__fds0_dn4, locals.var_fn97_calc_iq__fds0_dn7, locals.var_fn97_calc_iq__fds0_dn14, locals.var_fn97_calc_iq__fds0_dn15,)
    }
};
        locals.var_fn97_calc_iq__fds0 = assign8330_e9093;
        locals.var_fn97_calc_iq__fds0_dn2 = assign8330_e9093_d_n2;
        locals.var_fn97_calc_iq__fds0_dn4 = assign8330_e9093_d_n4;
        locals.var_fn97_calc_iq__fds0_dn7 = assign8330_e9093_d_n7;
        locals.var_fn97_calc_iq__fds0_dn14 = assign8330_e9093_d_n14;
        locals.var_fn97_calc_iq__fds0_dn15 = assign8330_e9093_d_n15;

        let (assign8340_e9097, assign8340_e9097_d_n2, assign8340_e9097_d_n4, assign8340_e9097_d_n7, assign8340_e9097_d_n14, assign8340_e9097_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vsx0, locals.var_fn97_calc_iq__vsx0_dn2, locals.var_fn97_calc_iq__vsx0_dn4, locals.var_fn97_calc_iq__vsx0_dn7, locals.var_fn97_calc_iq__vsx0_dn14, locals.var_fn97_calc_iq__vsx0_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsx0 = assign8340_e9097;
        locals.var_fn97_calc_iq__vsx0_dn2 = assign8340_e9097_d_n2;
        locals.var_fn97_calc_iq__vsx0_dn4 = assign8340_e9097_d_n4;
        locals.var_fn97_calc_iq__vsx0_dn7 = assign8340_e9097_d_n7;
        locals.var_fn97_calc_iq__vsx0_dn14 = assign8340_e9097_d_n14;
        locals.var_fn97_calc_iq__vsx0_dn15 = assign8340_e9097_d_n15;

        let (assign8350_e9101, assign8350_e9101_d_n2, assign8350_e9101_d_n4, assign8350_e9101_d_n7, assign8350_e9101_d_n14, assign8350_e9101_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd0, locals.var_fn97_calc_iq__ffd0_dn2, locals.var_fn97_calc_iq__ffd0_dn4, locals.var_fn97_calc_iq__ffd0_dn7, locals.var_fn97_calc_iq__ffd0_dn14, locals.var_fn97_calc_iq__ffd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd0 = assign8350_e9101;
        locals.var_fn97_calc_iq__ffd0_dn2 = assign8350_e9101_d_n2;
        locals.var_fn97_calc_iq__ffd0_dn4 = assign8350_e9101_d_n4;
        locals.var_fn97_calc_iq__ffd0_dn7 = assign8350_e9101_d_n7;
        locals.var_fn97_calc_iq__ffd0_dn14 = assign8350_e9101_d_n14;
        locals.var_fn97_calc_iq__ffd0_dn15 = assign8350_e9101_d_n15;

        let (assign8360_e9105, assign8360_e9105_d_n2, assign8360_e9105_d_n4, assign8360_e9105_d_n7, assign8360_e9105_d_n14, assign8360_e9105_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etad0, locals.var_fn97_calc_iq__etad0_dn2, locals.var_fn97_calc_iq__etad0_dn4, locals.var_fn97_calc_iq__etad0_dn7, locals.var_fn97_calc_iq__etad0_dn14, locals.var_fn97_calc_iq__etad0_dn15,)
    }
};
        locals.var_fn97_calc_iq__etad0 = assign8360_e9105;
        locals.var_fn97_calc_iq__etad0_dn2 = assign8360_e9105_d_n2;
        locals.var_fn97_calc_iq__etad0_dn4 = assign8360_e9105_d_n4;
        locals.var_fn97_calc_iq__etad0_dn7 = assign8360_e9105_d_n7;
        locals.var_fn97_calc_iq__etad0_dn14 = assign8360_e9105_d_n14;
        locals.var_fn97_calc_iq__etad0_dn15 = assign8360_e9105_d_n15;

        let (assign8370_e9109, assign8370_e9109_d_n2, assign8370_e9109_d_n4, assign8370_e9109_d_n7, assign8370_e9109_d_n14, assign8370_e9109_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvd0, locals.var_fn97_calc_iq__qinvd0_dn2, locals.var_fn97_calc_iq__qinvd0_dn4, locals.var_fn97_calc_iq__qinvd0_dn7, locals.var_fn97_calc_iq__qinvd0_dn14, locals.var_fn97_calc_iq__qinvd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd0 = assign8370_e9109;
        locals.var_fn97_calc_iq__qinvd0_dn2 = assign8370_e9109_d_n2;
        locals.var_fn97_calc_iq__qinvd0_dn4 = assign8370_e9109_d_n4;
        locals.var_fn97_calc_iq__qinvd0_dn7 = assign8370_e9109_d_n7;
        locals.var_fn97_calc_iq__qinvd0_dn14 = assign8370_e9109_d_n14;
        locals.var_fn97_calc_iq__qinvd0_dn15 = assign8370_e9109_d_n15;

        let (assign8380_e9113, assign8380_e9113_d_n2, assign8380_e9113_d_n4, assign8380_e9113_d_n7, assign8380_e9113_d_n14, assign8380_e9113_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qs2, locals.var_fn97_calc_iq__qs2_dn2, locals.var_fn97_calc_iq__qs2_dn4, locals.var_fn97_calc_iq__qs2_dn7, locals.var_fn97_calc_iq__qs2_dn14, locals.var_fn97_calc_iq__qs2_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs2 = assign8380_e9113;
        locals.var_fn97_calc_iq__qs2_dn2 = assign8380_e9113_d_n2;
        locals.var_fn97_calc_iq__qs2_dn4 = assign8380_e9113_d_n4;
        locals.var_fn97_calc_iq__qs2_dn7 = assign8380_e9113_d_n7;
        locals.var_fn97_calc_iq__qs2_dn14 = assign8380_e9113_d_n14;
        locals.var_fn97_calc_iq__qs2_dn15 = assign8380_e9113_d_n15;

        let (assign8390_e9117, assign8390_e9117_d_n2, assign8390_e9117_d_n4, assign8390_e9117_d_n7, assign8390_e9117_d_n14, assign8390_e9117_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qs3, locals.var_fn97_calc_iq__qs3_dn2, locals.var_fn97_calc_iq__qs3_dn4, locals.var_fn97_calc_iq__qs3_dn7, locals.var_fn97_calc_iq__qs3_dn14, locals.var_fn97_calc_iq__qs3_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs3 = assign8390_e9117;
        locals.var_fn97_calc_iq__qs3_dn2 = assign8390_e9117_d_n2;
        locals.var_fn97_calc_iq__qs3_dn4 = assign8390_e9117_d_n4;
        locals.var_fn97_calc_iq__qs3_dn7 = assign8390_e9117_d_n7;
        locals.var_fn97_calc_iq__qs3_dn14 = assign8390_e9117_d_n14;
        locals.var_fn97_calc_iq__qs3_dn15 = assign8390_e9117_d_n15;

        let (assign8400_e9121, assign8400_e9121_d_n2, assign8400_e9121_d_n4, assign8400_e9121_d_n7, assign8400_e9121_d_n14, assign8400_e9121_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qd2, locals.var_fn97_calc_iq__qd2_dn2, locals.var_fn97_calc_iq__qd2_dn4, locals.var_fn97_calc_iq__qd2_dn7, locals.var_fn97_calc_iq__qd2_dn14, locals.var_fn97_calc_iq__qd2_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd2 = assign8400_e9121;
        locals.var_fn97_calc_iq__qd2_dn2 = assign8400_e9121_d_n2;
        locals.var_fn97_calc_iq__qd2_dn4 = assign8400_e9121_d_n4;
        locals.var_fn97_calc_iq__qd2_dn7 = assign8400_e9121_d_n7;
        locals.var_fn97_calc_iq__qd2_dn14 = assign8400_e9121_d_n14;
        locals.var_fn97_calc_iq__qd2_dn15 = assign8400_e9121_d_n15;

        let (assign8410_e9125, assign8410_e9125_d_n2, assign8410_e9125_d_n4, assign8410_e9125_d_n7, assign8410_e9125_d_n14, assign8410_e9125_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qd3, locals.var_fn97_calc_iq__qd3_dn2, locals.var_fn97_calc_iq__qd3_dn4, locals.var_fn97_calc_iq__qd3_dn7, locals.var_fn97_calc_iq__qd3_dn14, locals.var_fn97_calc_iq__qd3_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd3 = assign8410_e9125;
        locals.var_fn97_calc_iq__qd3_dn2 = assign8410_e9125_d_n2;
        locals.var_fn97_calc_iq__qd3_dn4 = assign8410_e9125_d_n4;
        locals.var_fn97_calc_iq__qd3_dn7 = assign8410_e9125_d_n7;
        locals.var_fn97_calc_iq__qd3_dn14 = assign8410_e9125_d_n14;
        locals.var_fn97_calc_iq__qd3_dn15 = assign8410_e9125_d_n15;

        let (assign8420_e9129, assign8420_e9129_d_n2, assign8420_e9129_d_n4, assign8420_e9129_d_n7, assign8420_e9129_d_n14, assign8420_e9129_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qsqd, locals.var_fn97_calc_iq__qsqd_dn2, locals.var_fn97_calc_iq__qsqd_dn4, locals.var_fn97_calc_iq__qsqd_dn7, locals.var_fn97_calc_iq__qsqd_dn14, locals.var_fn97_calc_iq__qsqd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qsqd = assign8420_e9129;
        locals.var_fn97_calc_iq__qsqd_dn2 = assign8420_e9129_d_n2;
        locals.var_fn97_calc_iq__qsqd_dn4 = assign8420_e9129_d_n4;
        locals.var_fn97_calc_iq__qsqd_dn7 = assign8420_e9129_d_n7;
        locals.var_fn97_calc_iq__qsqd_dn14 = assign8420_e9129_d_n14;
        locals.var_fn97_calc_iq__qsqd_dn15 = assign8420_e9129_d_n15;

        let (assign8430_e9133, assign8430_e9133_d_n2, assign8430_e9133_d_n4, assign8430_e9133_d_n7, assign8430_e9133_d_n14, assign8430_e9133_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvdd, locals.var_fn97_calc_iq__qinvdd_dn2, locals.var_fn97_calc_iq__qinvdd_dn4, locals.var_fn97_calc_iq__qinvdd_dn7, locals.var_fn97_calc_iq__qinvdd_dn14, locals.var_fn97_calc_iq__qinvdd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvdd = assign8430_e9133;
        locals.var_fn97_calc_iq__qinvdd_dn2 = assign8430_e9133_d_n2;
        locals.var_fn97_calc_iq__qinvdd_dn4 = assign8430_e9133_d_n4;
        locals.var_fn97_calc_iq__qinvdd_dn7 = assign8430_e9133_d_n7;
        locals.var_fn97_calc_iq__qinvdd_dn14 = assign8430_e9133_d_n14;
        locals.var_fn97_calc_iq__qinvdd_dn15 = assign8430_e9133_d_n15;

        let (assign8440_e9137, assign8440_e9137_d_n2, assign8440_e9137_d_n4, assign8440_e9137_d_n7, assign8440_e9137_d_n14, assign8440_e9137_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qd1, locals.var_fn97_calc_iq__qd1_dn2, locals.var_fn97_calc_iq__qd1_dn4, locals.var_fn97_calc_iq__qd1_dn7, locals.var_fn97_calc_iq__qd1_dn14, locals.var_fn97_calc_iq__qd1_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd1 = assign8440_e9137;
        locals.var_fn97_calc_iq__qd1_dn2 = assign8440_e9137_d_n2;
        locals.var_fn97_calc_iq__qd1_dn4 = assign8440_e9137_d_n4;
        locals.var_fn97_calc_iq__qd1_dn7 = assign8440_e9137_d_n7;
        locals.var_fn97_calc_iq__qd1_dn14 = assign8440_e9137_d_n14;
        locals.var_fn97_calc_iq__qd1_dn15 = assign8440_e9137_d_n15;

        let (assign8450_e9141, assign8450_e9141_d_n2, assign8450_e9141_d_n4, assign8450_e9141_d_n7, assign8450_e9141_d_n14, assign8450_e9141_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qs, locals.var_fn97_calc_iq__qs_dn2, locals.var_fn97_calc_iq__qs_dn4, locals.var_fn97_calc_iq__qs_dn7, locals.var_fn97_calc_iq__qs_dn14, locals.var_fn97_calc_iq__qs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs = assign8450_e9141;
        locals.var_fn97_calc_iq__qs_dn2 = assign8450_e9141_d_n2;
        locals.var_fn97_calc_iq__qs_dn4 = assign8450_e9141_d_n4;
        locals.var_fn97_calc_iq__qs_dn7 = assign8450_e9141_d_n7;
        locals.var_fn97_calc_iq__qs_dn14 = assign8450_e9141_d_n14;
        locals.var_fn97_calc_iq__qs_dn15 = assign8450_e9141_d_n15;

        let (assign8460_e9145, assign8460_e9145_d_n2, assign8460_e9145_d_n4, assign8460_e9145_d_n7, assign8460_e9145_d_n14, assign8460_e9145_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qd, locals.var_fn97_calc_iq__qd_dn2, locals.var_fn97_calc_iq__qd_dn4, locals.var_fn97_calc_iq__qd_dn7, locals.var_fn97_calc_iq__qd_dn14, locals.var_fn97_calc_iq__qd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd = assign8460_e9145;
        locals.var_fn97_calc_iq__qd_dn2 = assign8460_e9145_d_n2;
        locals.var_fn97_calc_iq__qd_dn4 = assign8460_e9145_d_n4;
        locals.var_fn97_calc_iq__qd_dn7 = assign8460_e9145_d_n7;
        locals.var_fn97_calc_iq__qd_dn14 = assign8460_e9145_d_n14;
        locals.var_fn97_calc_iq__qd_dn15 = assign8460_e9145_d_n15;

        let (assign8470_e9149, assign8470_e9149_d_n2, assign8470_e9149_d_n4, assign8470_e9149_d_n7, assign8470_e9149_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etac, locals.var_fn97_calc_iq__etac_dn2, locals.var_fn97_calc_iq__etac_dn4, locals.var_fn97_calc_iq__etac_dn7, locals.var_fn97_calc_iq__etac_dn14,)
    }
};
        locals.var_fn97_calc_iq__etac = assign8470_e9149;
        locals.var_fn97_calc_iq__etac_dn2 = assign8470_e9149_d_n2;
        locals.var_fn97_calc_iq__etac_dn4 = assign8470_e9149_d_n4;
        locals.var_fn97_calc_iq__etac_dn7 = assign8470_e9149_d_n7;
        locals.var_fn97_calc_iq__etac_dn14 = assign8470_e9149_d_n14;

        let (assign8480_e9153, assign8480_e9153_d_n3, assign8480_e9153_d_n4, assign8480_e9153_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etab, locals.var_fn97_calc_iq__etab_dn3, locals.var_fn97_calc_iq__etab_dn4, locals.var_fn97_calc_iq__etab_dn14,)
    }
};
        locals.var_fn97_calc_iq__etab = assign8480_e9153;
        locals.var_fn97_calc_iq__etab_dn3 = assign8480_e9153_d_n3;
        locals.var_fn97_calc_iq__etab_dn4 = assign8480_e9153_d_n4;
        locals.var_fn97_calc_iq__etab_dn14 = assign8480_e9153_d_n14;

        let (assign8490_e9157, assign8490_e9157_d_n2, assign8490_e9157_d_n4, assign8490_e9157_d_n7, assign8490_e9157_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etags, locals.var_fn97_calc_iq__etags_dn2, locals.var_fn97_calc_iq__etags_dn4, locals.var_fn97_calc_iq__etags_dn7, locals.var_fn97_calc_iq__etags_dn14,)
    }
};
        locals.var_fn97_calc_iq__etags = assign8490_e9157;
        locals.var_fn97_calc_iq__etags_dn2 = assign8490_e9157_d_n2;
        locals.var_fn97_calc_iq__etags_dn4 = assign8490_e9157_d_n4;
        locals.var_fn97_calc_iq__etags_dn7 = assign8490_e9157_d_n7;
        locals.var_fn97_calc_iq__etags_dn14 = assign8490_e9157_d_n14;

        let (assign8500_e9161, assign8500_e9161_d_n2, assign8500_e9161_d_n3, assign8500_e9161_d_n4, assign8500_e9161_d_n7, assign8500_e9161_d_n14, assign8500_e9161_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign8500_e9161;
        locals.var_fn97_calc_iq__exparg_dn2 = assign8500_e9161_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign8500_e9161_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign8500_e9161_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign8500_e9161_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign8500_e9161_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign8500_e9161_d_n15;

        let (assign8510_e9165, assign8510_e9165_d_n2, assign8510_e9165_d_n3, assign8510_e9165_d_n4, assign8510_e9165_d_n7, assign8510_e9165_d_n14, assign8510_e9165_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__myarg, locals.var_fn97_calc_iq__myarg_dn2, locals.var_fn97_calc_iq__myarg_dn3, locals.var_fn97_calc_iq__myarg_dn4, locals.var_fn97_calc_iq__myarg_dn7, locals.var_fn97_calc_iq__myarg_dn14, locals.var_fn97_calc_iq__myarg_dn15,)
    }
};
        locals.var_fn97_calc_iq__myarg = assign8510_e9165;
        locals.var_fn97_calc_iq__myarg_dn2 = assign8510_e9165_d_n2;
        locals.var_fn97_calc_iq__myarg_dn3 = assign8510_e9165_d_n3;
        locals.var_fn97_calc_iq__myarg_dn4 = assign8510_e9165_d_n4;
        locals.var_fn97_calc_iq__myarg_dn7 = assign8510_e9165_d_n7;
        locals.var_fn97_calc_iq__myarg_dn14 = assign8510_e9165_d_n14;
        locals.var_fn97_calc_iq__myarg_dn15 = assign8510_e9165_d_n15;

        let (assign8520_e9169, assign8520_e9169_d_n14, assign8520_e9169_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__absvdsin, locals.var_fn97_calc_iq__absvdsin_dn14, locals.var_fn97_calc_iq__absvdsin_dn15,)
    }
};
        locals.var_fn97_calc_iq__absvdsin = assign8520_e9169;
        locals.var_fn97_calc_iq__absvdsin_dn14 = assign8520_e9169_d_n14;
        locals.var_fn97_calc_iq__absvdsin_dn15 = assign8520_e9169_d_n15;

        let (assign8530_e9173, assign8530_e9173_d_n2, assign8530_e9173_d_n7, assign8530_e9173_d_n14, assign8530_e9173_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vgdin, locals.var_fn97_calc_iq__vgdin_dn2, locals.var_fn97_calc_iq__vgdin_dn7, locals.var_fn97_calc_iq__vgdin_dn14, locals.var_fn97_calc_iq__vgdin_dn15,)
    }
};
        locals.var_fn97_calc_iq__vgdin = assign8530_e9173;
        locals.var_fn97_calc_iq__vgdin_dn2 = assign8530_e9173_d_n2;
        locals.var_fn97_calc_iq__vgdin_dn7 = assign8530_e9173_d_n7;
        locals.var_fn97_calc_iq__vgdin_dn14 = assign8530_e9173_d_n14;
        locals.var_fn97_calc_iq__vgdin_dn15 = assign8530_e9173_d_n15;

        let (assign8540_e9177, assign8540_e9177_d_n2, assign8540_e9177_d_n4, assign8540_e9177_d_n7, assign8540_e9177_d_n14, assign8540_e9177_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg0, locals.var_fn97_calc_iq__exparg0_dn2, locals.var_fn97_calc_iq__exparg0_dn4, locals.var_fn97_calc_iq__exparg0_dn7, locals.var_fn97_calc_iq__exparg0_dn14, locals.var_fn97_calc_iq__exparg0_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg0 = assign8540_e9177;
        locals.var_fn97_calc_iq__exparg0_dn2 = assign8540_e9177_d_n2;
        locals.var_fn97_calc_iq__exparg0_dn4 = assign8540_e9177_d_n4;
        locals.var_fn97_calc_iq__exparg0_dn7 = assign8540_e9177_d_n7;
        locals.var_fn97_calc_iq__exparg0_dn14 = assign8540_e9177_d_n14;
        locals.var_fn97_calc_iq__exparg0_dn15 = assign8540_e9177_d_n15;

        let (assign8550_e9181, assign8550_e9181_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__myarg0, locals.var_fn97_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn97_calc_iq__myarg0 = assign8550_e9181;
        locals.var_fn97_calc_iq__myarg0_dn4 = assign8550_e9181_d_n4;

        let (assign8560_e9208, assign8560_e9208_d_n14, assign8560_e9208_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign8560_e9206, assign8560_e9206_d_n14, assign8560_e9206_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign8560_e9190: f64 = (0.001 / p.p53);
                let assign8560_e9192: f64 = (assign8560_e9190 * locals.var_fn97_calc_iq__vdsin);
                let assign8560_e9193: f64 = (assign8560_e9192).tanh();
                let assign8560_e9194: f64 = (locals.var_fn97_calc_iq__vdsin * assign8560_e9193);
                (assign8560_e9194, ((locals.var_fn97_calc_iq__vdsin_dn14 * assign8560_e9193) + (locals.var_fn97_calc_iq__vdsin * ((assign8560_e9190 * locals.var_fn97_calc_iq__vdsin_dn14) / ((assign8560_e9192).cosh() * (assign8560_e9192).cosh())))), ((locals.var_fn97_calc_iq__vdsin_dn15 * assign8560_e9193) + (locals.var_fn97_calc_iq__vdsin * ((assign8560_e9190 * locals.var_fn97_calc_iq__vdsin_dn15) / ((assign8560_e9192).cosh() * (assign8560_e9192).cosh())))),)
            } else {
                let (assign8560_e9205, assign8560_e9205_d_n14, assign8560_e9205_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign8560_e9200: f64 = (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsin);
                        let assign8560_e9202: f64 = (assign8560_e9200 + p.p53);
                        let assign8560_e9203: f64 = (assign8560_e9202).sqrt();
                        (assign8560_e9203, (((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsin) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsin_dn14)) / (2.0 * assign8560_e9203)), (((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsin) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsin_dn15)) / (2.0 * assign8560_e9203)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign8560_e9205, assign8560_e9205_d_n14, assign8560_e9205_d_n15,)
            }
        };
        (assign8560_e9206, assign8560_e9206_d_n14, assign8560_e9206_d_n15,)
    } else {
        (locals.var_fn97_calc_iq__absvdsin, locals.var_fn97_calc_iq__absvdsin_dn14, locals.var_fn97_calc_iq__absvdsin_dn15,)
    }
};
        locals.var_fn97_calc_iq__absvdsin = assign8560_e9208;
        locals.var_fn97_calc_iq__absvdsin_dn14 = assign8560_e9208_d_n14;
        locals.var_fn97_calc_iq__absvdsin_dn15 = assign8560_e9208_d_n15;

        let (assign8570_e9214, assign8570_e9214_d_n2, assign8570_e9214_d_n7, assign8570_e9214_d_n14, assign8570_e9214_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8570_e9212: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vdsin);
        (assign8570_e9212, locals.var_fn97_calc_iq__vgsin_dn2, locals.var_fn97_calc_iq__vgsin_dn7, (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vdsin_dn14), (-locals.var_fn97_calc_iq__vdsin_dn15),)
    } else {
        (locals.var_fn97_calc_iq__vgdin, locals.var_fn97_calc_iq__vgdin_dn2, locals.var_fn97_calc_iq__vgdin_dn7, locals.var_fn97_calc_iq__vgdin_dn14, locals.var_fn97_calc_iq__vgdin_dn15,)
    }
};
        locals.var_fn97_calc_iq__vgdin = assign8570_e9214;
        locals.var_fn97_calc_iq__vgdin_dn2 = assign8570_e9214_d_n2;
        locals.var_fn97_calc_iq__vgdin_dn7 = assign8570_e9214_d_n7;
        locals.var_fn97_calc_iq__vgdin_dn14 = assign8570_e9214_d_n14;
        locals.var_fn97_calc_iq__vgdin_dn15 = assign8570_e9214_d_n15;

        let (assign8580_e9220, assign8580_e9220_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8580_e9218: f64 = (locals.var_fn97_calc_iq__alpha * locals.var_fn97_calc_iq__phitin);
        (assign8580_e9218, (locals.var_fn97_calc_iq__alpha * locals.var_fn97_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn97_calc_iq__alpha_phit, locals.var_fn97_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn97_calc_iq__alpha_phit = assign8580_e9220;
        locals.var_fn97_calc_iq__alpha_phit_dn4 = assign8580_e9220_d_n4;

        let (assign8590_e9232, assign8590_e9232_d_n4, assign8590_e9232_d_n14, assign8590_e9232_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8590_e9225: f64 = (2.302585092994046 * locals.var_fn97_calc_iq__phitin);
        let assign8590_e9226: f64 = (locals.var_fn97_calc_iq__ss / assign8590_e9225);
        let assign8590_e9229: f64 = (locals.var_fn97_calc_iq__nd * locals.var_fn97_calc_iq__absvdsin);
        let assign8590_e9230: f64 = (assign8590_e9226 + assign8590_e9229);
        (assign8590_e9230, (-((locals.var_fn97_calc_iq__ss * (2.302585092994046 * locals.var_fn97_calc_iq__phitin_dn4)) / (assign8590_e9225 * assign8590_e9225))), (locals.var_fn97_calc_iq__nd * locals.var_fn97_calc_iq__absvdsin_dn14), (locals.var_fn97_calc_iq__nd * locals.var_fn97_calc_iq__absvdsin_dn15),)
    } else {
        (locals.var_fn97_calc_iq__n, locals.var_fn97_calc_iq__n_dn4, locals.var_fn97_calc_iq__n_dn14, locals.var_fn97_calc_iq__n_dn15,)
    }
};
        locals.var_fn97_calc_iq__n = assign8590_e9232;
        locals.var_fn97_calc_iq__n_dn4 = assign8590_e9232_d_n4;
        locals.var_fn97_calc_iq__n_dn14 = assign8590_e9232_d_n14;
        locals.var_fn97_calc_iq__n_dn15 = assign8590_e9232_d_n15;

        let (assign8600_e9242, assign8600_e9242_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8600_e9238: f64 = (locals.var_fn97_calc_iq__tambin - locals.var_fn97_calc_iq__tnomin);
        let assign8600_e9239: f64 = (locals.var_fn97_calc_iq__vtzeta * assign8600_e9238);
        let assign8600_e9240: f64 = (locals.var_fn97_calc_iq__vto + assign8600_e9239);
        (assign8600_e9240, (locals.var_fn97_calc_iq__vtzeta * locals.var_fn97_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn97_calc_iq__vtof, locals.var_fn97_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn97_calc_iq__vtof = assign8600_e9242;
        locals.var_fn97_calc_iq__vtof_dn4 = assign8600_e9242_d_n4;

    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8610_e9250, assign8610_e9250_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8610_e9246: f64 = (locals.var_fn97_calc_iq__tambin / locals.var_fn97_calc_iq__tnomin);
        let assign8610_e9248: f64 = (assign8610_e9246).powf(locals.var_fn97_calc_iq__epsilon);
        (assign8610_e9248, if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn97_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__epsilon * ((assign8610_e9246).powf(locals.var_fn97_calc_iq__epsilon - 1.0) * (locals.var_fn97_calc_iq__tambin_dn4 / locals.var_fn97_calc_iq__tnomin))) } } else { (assign8610_e9248 * (locals.var_fn97_calc_iq__epsilon * ((locals.var_fn97_calc_iq__tambin_dn4 / locals.var_fn97_calc_iq__tnomin) / assign8610_e9246))) },)
    } else {
        (locals.var_fn97_calc_iq__tfacmobin, locals.var_fn97_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn97_calc_iq__tfacmobin = assign8610_e9250;
        locals.var_fn97_calc_iq__tfacmobin_dn4 = assign8610_e9250_d_n4;

        let assign8620_e9253: f64 = if locals.var_fn97_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign8620_e9253;

        let (assign8630_e9271, assign8630_e9271_d_n14, assign8630_e9271_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard98 != 0.0)) {
        let assign8630_e9261: f64 = (locals.var_fn97_calc_iq__absvdsin / locals.var_fn97_calc_iq__dibsat);
        let assign8630_e9263: f64 = (assign8630_e9261).powf(locals.var_fn97_calc_iq__beta);
        let assign8630_e9264: f64 = (1.0 + assign8630_e9263);
        let assign8630_e9267: f64 = (1.0 / locals.var_fn97_calc_iq__beta);
        let assign8630_e9268: f64 = (assign8630_e9264).powf(assign8630_e9267);
        let assign8630_e9269: f64 = (locals.var_fn97_calc_iq__absvdsin / assign8630_e9268);
        (assign8630_e9269, (((locals.var_fn97_calc_iq__absvdsin_dn14 * assign8630_e9268) - (locals.var_fn97_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign8630_e9267) as f64).is_finite() && ((assign8630_e9267) as f64).fract() == 0.0 { if assign8630_e9267 == 0.0 { 0.0 } else { (assign8630_e9267 * ((assign8630_e9264).powf(assign8630_e9267 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8630_e9261).powf(locals.var_fn97_calc_iq__beta - 1.0) * (locals.var_fn97_calc_iq__absvdsin_dn14 / locals.var_fn97_calc_iq__dibsat))) } } else { (assign8630_e9263 * (locals.var_fn97_calc_iq__beta * ((locals.var_fn97_calc_iq__absvdsin_dn14 / locals.var_fn97_calc_iq__dibsat) / assign8630_e9261))) })) } } else { (assign8630_e9268 * (assign8630_e9267 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8630_e9261).powf(locals.var_fn97_calc_iq__beta - 1.0) * (locals.var_fn97_calc_iq__absvdsin_dn14 / locals.var_fn97_calc_iq__dibsat))) } } else { (assign8630_e9263 * (locals.var_fn97_calc_iq__beta * ((locals.var_fn97_calc_iq__absvdsin_dn14 / locals.var_fn97_calc_iq__dibsat) / assign8630_e9261))) } / assign8630_e9264))) })) / (assign8630_e9268 * assign8630_e9268)), (((locals.var_fn97_calc_iq__absvdsin_dn15 * assign8630_e9268) - (locals.var_fn97_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign8630_e9267) as f64).is_finite() && ((assign8630_e9267) as f64).fract() == 0.0 { if assign8630_e9267 == 0.0 { 0.0 } else { (assign8630_e9267 * ((assign8630_e9264).powf(assign8630_e9267 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8630_e9261).powf(locals.var_fn97_calc_iq__beta - 1.0) * (locals.var_fn97_calc_iq__absvdsin_dn15 / locals.var_fn97_calc_iq__dibsat))) } } else { (assign8630_e9263 * (locals.var_fn97_calc_iq__beta * ((locals.var_fn97_calc_iq__absvdsin_dn15 / locals.var_fn97_calc_iq__dibsat) / assign8630_e9261))) })) } } else { (assign8630_e9268 * (assign8630_e9267 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8630_e9261).powf(locals.var_fn97_calc_iq__beta - 1.0) * (locals.var_fn97_calc_iq__absvdsin_dn15 / locals.var_fn97_calc_iq__dibsat))) } } else { (assign8630_e9263 * (locals.var_fn97_calc_iq__beta * ((locals.var_fn97_calc_iq__absvdsin_dn15 / locals.var_fn97_calc_iq__dibsat) / assign8630_e9261))) } / assign8630_e9264))) })) / (assign8630_e9268 * assign8630_e9268)),)
    } else {
        (locals.var_fn97_calc_iq__vsatdibl, locals.var_fn97_calc_iq__vsatdibl_dn14, locals.var_fn97_calc_iq__vsatdibl_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsatdibl = assign8630_e9271;
        locals.var_fn97_calc_iq__vsatdibl_dn14 = assign8630_e9271_d_n14;
        locals.var_fn97_calc_iq__vsatdibl_dn15 = assign8630_e9271_d_n15;

        let (assign8640_e9278, assign8640_e9278_d_n14, assign8640_e9278_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard98 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vsatdibl, locals.var_fn97_calc_iq__vsatdibl_dn14, locals.var_fn97_calc_iq__vsatdibl_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsatdibl = assign8640_e9278;
        locals.var_fn97_calc_iq__vsatdibl_dn14 = assign8640_e9278_d_n14;
        locals.var_fn97_calc_iq__vsatdibl_dn15 = assign8640_e9278_d_n15;

        let (assign8650_e9288, assign8650_e9288_d_n14, assign8650_e9288_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8650_e9283: f64 = (locals.var_fn97_calc_iq__vsatdibl * locals.var_fn97_calc_iq__delta2);
        let assign8650_e9284: f64 = (locals.var_fn97_calc_iq__delta1 - assign8650_e9283);
        let assign8650_e9286: f64 = (assign8650_e9284 * locals.var_fn97_calc_iq__absvdsin);
        (assign8650_e9286, (((-(locals.var_fn97_calc_iq__vsatdibl_dn14 * locals.var_fn97_calc_iq__delta2)) * locals.var_fn97_calc_iq__absvdsin) + (assign8650_e9284 * locals.var_fn97_calc_iq__absvdsin_dn14)), (((-(locals.var_fn97_calc_iq__vsatdibl_dn15 * locals.var_fn97_calc_iq__delta2)) * locals.var_fn97_calc_iq__absvdsin) + (assign8650_e9284 * locals.var_fn97_calc_iq__absvdsin_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__delta, locals.var_fn97_calc_iq__delta_dn14, locals.var_fn97_calc_iq__delta_dn15,)
    }
};
        locals.var_fn97_calc_iq__delta = assign8650_e9288;
        locals.var_fn97_calc_iq__delta_dn14 = assign8650_e9288_d_n14;
        locals.var_fn97_calc_iq__delta_dn15 = assign8650_e9288_d_n15;

        let (assign8660_e9294, assign8660_e9294_d_n4, assign8660_e9294_d_n14, assign8660_e9294_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8660_e9292: f64 = (locals.var_fn97_calc_iq__vtof - locals.var_fn97_calc_iq__delta);
        (assign8660_e9292, locals.var_fn97_calc_iq__vtof_dn4, (-locals.var_fn97_calc_iq__delta_dn14), (-locals.var_fn97_calc_iq__delta_dn15),)
    } else {
        (locals.var_fn97_calc_iq__vtdibl, locals.var_fn97_calc_iq__vtdibl_dn4, locals.var_fn97_calc_iq__vtdibl_dn14, locals.var_fn97_calc_iq__vtdibl_dn15,)
    }
};
        locals.var_fn97_calc_iq__vtdibl = assign8660_e9294;
        locals.var_fn97_calc_iq__vtdibl_dn4 = assign8660_e9294_d_n4;
        locals.var_fn97_calc_iq__vtdibl_dn14 = assign8660_e9294_d_n14;
        locals.var_fn97_calc_iq__vtdibl_dn15 = assign8660_e9294_d_n15;

        let (assign8670_e9302, assign8670_e9302_d_n4, assign8670_e9302_d_n14, assign8670_e9302_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8670_e9298: f64 = (2.0 * locals.var_fn97_calc_iq__n);
        let assign8670_e9300: f64 = (assign8670_e9298 * locals.var_fn97_calc_iq__phitin);
        (assign8670_e9300, (((2.0 * locals.var_fn97_calc_iq__n_dn4) * locals.var_fn97_calc_iq__phitin) + (assign8670_e9298 * locals.var_fn97_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn97_calc_iq__n_dn14) * locals.var_fn97_calc_iq__phitin), ((2.0 * locals.var_fn97_calc_iq__n_dn15) * locals.var_fn97_calc_iq__phitin),)
    } else {
        (locals.var_fn97_calc_iq__two_n_phit, locals.var_fn97_calc_iq__two_n_phit_dn4, locals.var_fn97_calc_iq__two_n_phit_dn14, locals.var_fn97_calc_iq__two_n_phit_dn15,)
    }
};
        locals.var_fn97_calc_iq__two_n_phit = assign8670_e9302;
        locals.var_fn97_calc_iq__two_n_phit_dn4 = assign8670_e9302_d_n4;
        locals.var_fn97_calc_iq__two_n_phit_dn14 = assign8670_e9302_d_n14;
        locals.var_fn97_calc_iq__two_n_phit_dn15 = assign8670_e9302_d_n15;

        let (assign8680_e9308, assign8680_e9308_d_n4, assign8680_e9308_d_n14, assign8680_e9308_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8680_e9306: f64 = (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit);
        (assign8680_e9306, ((locals.var_fn97_calc_iq__cgin_dn4 * locals.var_fn97_calc_iq__two_n_phit) + (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit_dn4)), (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit_dn14), (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit_dn15),)
    } else {
        (locals.var_fn97_calc_iq__qref, locals.var_fn97_calc_iq__qref_dn4, locals.var_fn97_calc_iq__qref_dn14, locals.var_fn97_calc_iq__qref_dn15,)
    }
};
        locals.var_fn97_calc_iq__qref = assign8680_e9308;
        locals.var_fn97_calc_iq__qref_dn4 = assign8680_e9308_d_n4;
        locals.var_fn97_calc_iq__qref_dn14 = assign8680_e9308_d_n14;
        locals.var_fn97_calc_iq__qref_dn15 = assign8680_e9308_d_n15;

        let (assign8690_e9318, assign8690_e9318_d_n2, assign8690_e9318_d_n3, assign8690_e9318_d_n4, assign8690_e9318_d_n7, assign8690_e9318_d_n14, assign8690_e9318_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8690_e9313: f64 = (p.p51 * locals.var_fn97_calc_iq__alpha_phit);
        let assign8690_e9315: f64 = (assign8690_e9313 / 2.0);
        let assign8690_e9316: f64 = (locals.var_fn97_calc_iq__vtdibl - assign8690_e9315);
        (assign8690_e9316, 0.0, 0.0, (locals.var_fn97_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn97_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn97_calc_iq__vtdibl_dn14, locals.var_fn97_calc_iq__vtdibl_dn15,)
    } else {
        (locals.var_fn97_calc_iq__myarg, locals.var_fn97_calc_iq__myarg_dn2, locals.var_fn97_calc_iq__myarg_dn3, locals.var_fn97_calc_iq__myarg_dn4, locals.var_fn97_calc_iq__myarg_dn7, locals.var_fn97_calc_iq__myarg_dn14, locals.var_fn97_calc_iq__myarg_dn15,)
    }
};
        locals.var_fn97_calc_iq__myarg = assign8690_e9318;
        locals.var_fn97_calc_iq__myarg_dn2 = assign8690_e9318_d_n2;
        locals.var_fn97_calc_iq__myarg_dn3 = assign8690_e9318_d_n3;
        locals.var_fn97_calc_iq__myarg_dn4 = assign8690_e9318_d_n4;
        locals.var_fn97_calc_iq__myarg_dn7 = assign8690_e9318_d_n7;
        locals.var_fn97_calc_iq__myarg_dn14 = assign8690_e9318_d_n14;
        locals.var_fn97_calc_iq__myarg_dn15 = assign8690_e9318_d_n15;

        let (assign8700_e9369, assign8700_e9369_d_n2, assign8700_e9369_d_n3, assign8700_e9369_d_n4, assign8700_e9369_d_n7, assign8700_e9369_d_n14, assign8700_e9369_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign8700_e9363, assign8700_e9363_d_n2, assign8700_e9363_d_n7, assign8700_e9363_d_n14, assign8700_e9363_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign8700_e9327: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                let assign8700_e9330: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign8700_e9333: f64 = (0.001 / p.p53);
                let assign8700_e9336: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign8700_e9337: f64 = (assign8700_e9333 * assign8700_e9336);
                let assign8700_e9338: f64 = (assign8700_e9337).tanh();
                let assign8700_e9339: f64 = (assign8700_e9330 * assign8700_e9338);
                let assign8700_e9340: f64 = (assign8700_e9327 + assign8700_e9339);
                let assign8700_e9341: f64 = (0.5 * assign8700_e9340);
                (assign8700_e9341, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign8700_e9338) + (assign8700_e9330 * ((assign8700_e9333 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2)) / ((assign8700_e9337).cosh() * (assign8700_e9337).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign8700_e9338) + (assign8700_e9330 * ((assign8700_e9333 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7)) / ((assign8700_e9337).cosh() * (assign8700_e9337).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + (((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign8700_e9338) + (assign8700_e9330 * ((assign8700_e9333 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14)) / ((assign8700_e9337).cosh() * (assign8700_e9337).cosh())))))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + (((-locals.var_fn97_calc_iq__vgdin_dn15) * assign8700_e9338) + (assign8700_e9330 * ((assign8700_e9333 * (-locals.var_fn97_calc_iq__vgdin_dn15)) / ((assign8700_e9337).cosh() * (assign8700_e9337).cosh())))))),)
            } else {
                let (assign8700_e9362, assign8700_e9362_d_n2, assign8700_e9362_d_n7, assign8700_e9362_d_n14, assign8700_e9362_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign8700_e9348: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                        let assign8700_e9351: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign8700_e9354: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign8700_e9355: f64 = (assign8700_e9351 * assign8700_e9354);
                        let assign8700_e9357: f64 = (assign8700_e9355 + p.p53);
                        let assign8700_e9358: f64 = (assign8700_e9357).sqrt();
                        let assign8700_e9359: f64 = (assign8700_e9348 + assign8700_e9358);
                        let assign8700_e9360: f64 = (0.5 * assign8700_e9359);
                        (assign8700_e9360, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + ((((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign8700_e9354) + (assign8700_e9351 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2))) / (2.0 * assign8700_e9358)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + ((((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign8700_e9354) + (assign8700_e9351 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7))) / (2.0 * assign8700_e9358)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + ((((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign8700_e9354) + (assign8700_e9351 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14))) / (2.0 * assign8700_e9358)))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + ((((-locals.var_fn97_calc_iq__vgdin_dn15) * assign8700_e9354) + (assign8700_e9351 * (-locals.var_fn97_calc_iq__vgdin_dn15))) / (2.0 * assign8700_e9358)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign8700_e9362, assign8700_e9362_d_n2, assign8700_e9362_d_n7, assign8700_e9362_d_n14, assign8700_e9362_d_n15,)
            }
        };
        let assign8700_e9365: f64 = (assign8700_e9363 - locals.var_fn97_calc_iq__myarg);
        let assign8700_e9367: f64 = (assign8700_e9365 / locals.var_fn97_calc_iq__alpha_phit);
        (assign8700_e9367, ((assign8700_e9363_d_n2 - locals.var_fn97_calc_iq__myarg_dn2) / locals.var_fn97_calc_iq__alpha_phit), ((-locals.var_fn97_calc_iq__myarg_dn3) / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign8700_e9365 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), ((assign8700_e9363_d_n7 - locals.var_fn97_calc_iq__myarg_dn7) / locals.var_fn97_calc_iq__alpha_phit), ((assign8700_e9363_d_n14 - locals.var_fn97_calc_iq__myarg_dn14) / locals.var_fn97_calc_iq__alpha_phit), ((assign8700_e9363_d_n15 - locals.var_fn97_calc_iq__myarg_dn15) / locals.var_fn97_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign8700_e9369;
        locals.var_fn97_calc_iq__exparg_dn2 = assign8700_e9369_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign8700_e9369_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign8700_e9369_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign8700_e9369_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign8700_e9369_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign8700_e9369_d_n15;

        let assign8710_e9372: f64 = if locals.var_fn97_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign8710_e9372;

        let (assign8720_e9378, assign8720_e9378_d_n2, assign8720_e9378_d_n3, assign8720_e9378_d_n4, assign8720_e9378_d_n7, assign8720_e9378_d_n14, assign8720_e9378_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard99 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff, locals.var_fn97_calc_iq__ff_dn2, locals.var_fn97_calc_iq__ff_dn3, locals.var_fn97_calc_iq__ff_dn4, locals.var_fn97_calc_iq__ff_dn7, locals.var_fn97_calc_iq__ff_dn14, locals.var_fn97_calc_iq__ff_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff = assign8720_e9378;
        locals.var_fn97_calc_iq__ff_dn2 = assign8720_e9378_d_n2;
        locals.var_fn97_calc_iq__ff_dn3 = assign8720_e9378_d_n3;
        locals.var_fn97_calc_iq__ff_dn4 = assign8720_e9378_d_n4;
        locals.var_fn97_calc_iq__ff_dn7 = assign8720_e9378_d_n7;
        locals.var_fn97_calc_iq__ff_dn14 = assign8720_e9378_d_n14;
        locals.var_fn97_calc_iq__ff_dn15 = assign8720_e9378_d_n15;

        let assign8730_e9381: f64 = (-50.0);
        let assign8730_e9382: f64 = if locals.var_fn97_calc_iq__exparg < assign8730_e9381 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign8730_e9382;

        let (assign8740_e9391, assign8740_e9391_d_n2, assign8740_e9391_d_n3, assign8740_e9391_d_n4, assign8740_e9391_d_n7, assign8740_e9391_d_n14, assign8740_e9391_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard99 == 0.0)) && (locals.var_guard100 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff, locals.var_fn97_calc_iq__ff_dn2, locals.var_fn97_calc_iq__ff_dn3, locals.var_fn97_calc_iq__ff_dn4, locals.var_fn97_calc_iq__ff_dn7, locals.var_fn97_calc_iq__ff_dn14, locals.var_fn97_calc_iq__ff_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff = assign8740_e9391;
        locals.var_fn97_calc_iq__ff_dn2 = assign8740_e9391_d_n2;
        locals.var_fn97_calc_iq__ff_dn3 = assign8740_e9391_d_n3;
        locals.var_fn97_calc_iq__ff_dn4 = assign8740_e9391_d_n4;
        locals.var_fn97_calc_iq__ff_dn7 = assign8740_e9391_d_n7;
        locals.var_fn97_calc_iq__ff_dn14 = assign8740_e9391_d_n14;
        locals.var_fn97_calc_iq__ff_dn15 = assign8740_e9391_d_n15;

        let (assign8750_e9406, assign8750_e9406_d_n2, assign8750_e9406_d_n3, assign8750_e9406_d_n4, assign8750_e9406_d_n7, assign8750_e9406_d_n14, assign8750_e9406_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard99 == 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign8750_e9402: f64 = (locals.var_fn97_calc_iq__exparg).exp();
        let assign8750_e9403: f64 = (1.0 + assign8750_e9402);
        let assign8750_e9404: f64 = (1.0 / assign8750_e9403);
        (assign8750_e9404, (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn2) / (assign8750_e9403 * assign8750_e9403))), (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn3) / (assign8750_e9403 * assign8750_e9403))), (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn4) / (assign8750_e9403 * assign8750_e9403))), (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn7) / (assign8750_e9403 * assign8750_e9403))), (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn14) / (assign8750_e9403 * assign8750_e9403))), (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn15) / (assign8750_e9403 * assign8750_e9403))),)
    } else {
        (locals.var_fn97_calc_iq__ff, locals.var_fn97_calc_iq__ff_dn2, locals.var_fn97_calc_iq__ff_dn3, locals.var_fn97_calc_iq__ff_dn4, locals.var_fn97_calc_iq__ff_dn7, locals.var_fn97_calc_iq__ff_dn14, locals.var_fn97_calc_iq__ff_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff = assign8750_e9406;
        locals.var_fn97_calc_iq__ff_dn2 = assign8750_e9406_d_n2;
        locals.var_fn97_calc_iq__ff_dn3 = assign8750_e9406_d_n3;
        locals.var_fn97_calc_iq__ff_dn4 = assign8750_e9406_d_n4;
        locals.var_fn97_calc_iq__ff_dn7 = assign8750_e9406_d_n7;
        locals.var_fn97_calc_iq__ff_dn14 = assign8750_e9406_d_n14;
        locals.var_fn97_calc_iq__ff_dn15 = assign8750_e9406_d_n15;

        let (assign8760_e9465, assign8760_e9465_d_n2, assign8760_e9465_d_n3, assign8760_e9465_d_n4, assign8760_e9465_d_n7, assign8760_e9465_d_n14, assign8760_e9465_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign8760_e9451, assign8760_e9451_d_n2, assign8760_e9451_d_n7, assign8760_e9451_d_n14, assign8760_e9451_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign8760_e9415: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                let assign8760_e9418: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign8760_e9421: f64 = (0.001 / p.p53);
                let assign8760_e9424: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign8760_e9425: f64 = (assign8760_e9421 * assign8760_e9424);
                let assign8760_e9426: f64 = (assign8760_e9425).tanh();
                let assign8760_e9427: f64 = (assign8760_e9418 * assign8760_e9426);
                let assign8760_e9428: f64 = (assign8760_e9415 + assign8760_e9427);
                let assign8760_e9429: f64 = (0.5 * assign8760_e9428);
                (assign8760_e9429, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign8760_e9426) + (assign8760_e9418 * ((assign8760_e9421 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2)) / ((assign8760_e9425).cosh() * (assign8760_e9425).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign8760_e9426) + (assign8760_e9418 * ((assign8760_e9421 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7)) / ((assign8760_e9425).cosh() * (assign8760_e9425).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + (((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign8760_e9426) + (assign8760_e9418 * ((assign8760_e9421 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14)) / ((assign8760_e9425).cosh() * (assign8760_e9425).cosh())))))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + (((-locals.var_fn97_calc_iq__vgdin_dn15) * assign8760_e9426) + (assign8760_e9418 * ((assign8760_e9421 * (-locals.var_fn97_calc_iq__vgdin_dn15)) / ((assign8760_e9425).cosh() * (assign8760_e9425).cosh())))))),)
            } else {
                let (assign8760_e9450, assign8760_e9450_d_n2, assign8760_e9450_d_n7, assign8760_e9450_d_n14, assign8760_e9450_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign8760_e9436: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                        let assign8760_e9439: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign8760_e9442: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign8760_e9443: f64 = (assign8760_e9439 * assign8760_e9442);
                        let assign8760_e9445: f64 = (assign8760_e9443 + p.p53);
                        let assign8760_e9446: f64 = (assign8760_e9445).sqrt();
                        let assign8760_e9447: f64 = (assign8760_e9436 + assign8760_e9446);
                        let assign8760_e9448: f64 = (0.5 * assign8760_e9447);
                        (assign8760_e9448, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + ((((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign8760_e9442) + (assign8760_e9439 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2))) / (2.0 * assign8760_e9446)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + ((((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign8760_e9442) + (assign8760_e9439 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7))) / (2.0 * assign8760_e9446)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + ((((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign8760_e9442) + (assign8760_e9439 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14))) / (2.0 * assign8760_e9446)))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + ((((-locals.var_fn97_calc_iq__vgdin_dn15) * assign8760_e9442) + (assign8760_e9439 * (-locals.var_fn97_calc_iq__vgdin_dn15))) / (2.0 * assign8760_e9446)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign8760_e9450, assign8760_e9450_d_n2, assign8760_e9450_d_n7, assign8760_e9450_d_n14, assign8760_e9450_d_n15,)
            }
        };
        let assign8760_e9455: f64 = (p.p51 * 0.1);
        let assign8760_e9457: f64 = (assign8760_e9455 * locals.var_fn97_calc_iq__alpha_phit);
        let assign8760_e9459: f64 = (assign8760_e9457 * locals.var_fn97_calc_iq__ff);
        let assign8760_e9460: f64 = (locals.var_fn97_calc_iq__vtdibl - assign8760_e9459);
        let assign8760_e9461: f64 = (assign8760_e9451 - assign8760_e9460);
        let assign8760_e9463: f64 = (assign8760_e9461 / locals.var_fn97_calc_iq__two_n_phit);
        (assign8760_e9463, ((assign8760_e9451_d_n2 - (-(assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn2))) / locals.var_fn97_calc_iq__two_n_phit), ((-(-(assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn3))) / locals.var_fn97_calc_iq__two_n_phit), ((((-(locals.var_fn97_calc_iq__vtdibl_dn4 - (((assign8760_e9455 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ff) + (assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn4)))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8760_e9461 * locals.var_fn97_calc_iq__two_n_phit_dn4)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), ((assign8760_e9451_d_n7 - (-(assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn7))) / locals.var_fn97_calc_iq__two_n_phit), ((((assign8760_e9451_d_n14 - (locals.var_fn97_calc_iq__vtdibl_dn14 - (assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn14))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8760_e9461 * locals.var_fn97_calc_iq__two_n_phit_dn14)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), ((((assign8760_e9451_d_n15 - (locals.var_fn97_calc_iq__vtdibl_dn15 - (assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn15))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8760_e9461 * locals.var_fn97_calc_iq__two_n_phit_dn15)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn97_calc_iq__eta, locals.var_fn97_calc_iq__eta_dn2, locals.var_fn97_calc_iq__eta_dn3, locals.var_fn97_calc_iq__eta_dn4, locals.var_fn97_calc_iq__eta_dn7, locals.var_fn97_calc_iq__eta_dn14, locals.var_fn97_calc_iq__eta_dn15,)
    }
};
        locals.var_fn97_calc_iq__eta = assign8760_e9465;
        locals.var_fn97_calc_iq__eta_dn2 = assign8760_e9465_d_n2;
        locals.var_fn97_calc_iq__eta_dn3 = assign8760_e9465_d_n3;
        locals.var_fn97_calc_iq__eta_dn4 = assign8760_e9465_d_n4;
        locals.var_fn97_calc_iq__eta_dn7 = assign8760_e9465_d_n7;
        locals.var_fn97_calc_iq__eta_dn14 = assign8760_e9465_d_n14;
        locals.var_fn97_calc_iq__eta_dn15 = assign8760_e9465_d_n15;

        let assign8770_e9468: f64 = if locals.var_fn97_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign8770_e9468;

        let (assign8780_e9476, assign8780_e9476_d_n2, assign8780_e9476_d_n3, assign8780_e9476_d_n4, assign8780_e9476_d_n7, assign8780_e9476_d_n14, assign8780_e9476_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard101 != 0.0)) {
        let assign8780_e9474: f64 = (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta);
        (assign8780_e9474, (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn2), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn3), ((locals.var_fn97_calc_iq__qref_dn4 * locals.var_fn97_calc_iq__eta) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn4)), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn7), ((locals.var_fn97_calc_iq__qref_dn14 * locals.var_fn97_calc_iq__eta) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn14)), ((locals.var_fn97_calc_iq__qref_dn15 * locals.var_fn97_calc_iq__eta) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvv, locals.var_fn97_calc_iq__qinvv_dn2, locals.var_fn97_calc_iq__qinvv_dn3, locals.var_fn97_calc_iq__qinvv_dn4, locals.var_fn97_calc_iq__qinvv_dn7, locals.var_fn97_calc_iq__qinvv_dn14, locals.var_fn97_calc_iq__qinvv_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv = assign8780_e9476;
        locals.var_fn97_calc_iq__qinvv_dn2 = assign8780_e9476_d_n2;
        locals.var_fn97_calc_iq__qinvv_dn3 = assign8780_e9476_d_n3;
        locals.var_fn97_calc_iq__qinvv_dn4 = assign8780_e9476_d_n4;
        locals.var_fn97_calc_iq__qinvv_dn7 = assign8780_e9476_d_n7;
        locals.var_fn97_calc_iq__qinvv_dn14 = assign8780_e9476_d_n14;
        locals.var_fn97_calc_iq__qinvv_dn15 = assign8780_e9476_d_n15;

        let assign8790_e9479: f64 = (-50.0);
        let assign8790_e9480: f64 = if locals.var_fn97_calc_iq__eta < assign8790_e9479 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign8790_e9480;

        let (assign8800_e9492, assign8800_e9492_d_n2, assign8800_e9492_d_n3, assign8800_e9492_d_n4, assign8800_e9492_d_n7, assign8800_e9492_d_n14, assign8800_e9492_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard101 == 0.0)) && (locals.var_guard102 != 0.0)) {
        let assign8800_e9489: f64 = (locals.var_fn97_calc_iq__eta).exp();
        let assign8800_e9490: f64 = (locals.var_fn97_calc_iq__qref * assign8800_e9489);
        (assign8800_e9490, (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn2)), (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn3)), ((locals.var_fn97_calc_iq__qref_dn4 * assign8800_e9489) + (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn4))), (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn7)), ((locals.var_fn97_calc_iq__qref_dn14 * assign8800_e9489) + (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn14))), ((locals.var_fn97_calc_iq__qref_dn15 * assign8800_e9489) + (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn15))),)
    } else {
        (locals.var_fn97_calc_iq__qinvv, locals.var_fn97_calc_iq__qinvv_dn2, locals.var_fn97_calc_iq__qinvv_dn3, locals.var_fn97_calc_iq__qinvv_dn4, locals.var_fn97_calc_iq__qinvv_dn7, locals.var_fn97_calc_iq__qinvv_dn14, locals.var_fn97_calc_iq__qinvv_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv = assign8800_e9492;
        locals.var_fn97_calc_iq__qinvv_dn2 = assign8800_e9492_d_n2;
        locals.var_fn97_calc_iq__qinvv_dn3 = assign8800_e9492_d_n3;
        locals.var_fn97_calc_iq__qinvv_dn4 = assign8800_e9492_d_n4;
        locals.var_fn97_calc_iq__qinvv_dn7 = assign8800_e9492_d_n7;
        locals.var_fn97_calc_iq__qinvv_dn14 = assign8800_e9492_d_n14;
        locals.var_fn97_calc_iq__qinvv_dn15 = assign8800_e9492_d_n15;

        let (assign8810_e9508, assign8810_e9508_d_n2, assign8810_e9508_d_n3, assign8810_e9508_d_n4, assign8810_e9508_d_n7, assign8810_e9508_d_n14, assign8810_e9508_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard101 == 0.0)) && (locals.var_guard102 == 0.0)) {
        let assign8810_e9503: f64 = (locals.var_fn97_calc_iq__eta).exp();
        let assign8810_e9504: f64 = (1.0 + assign8810_e9503);
        let assign8810_e9505: f64 = (assign8810_e9504).ln();
        let assign8810_e9506: f64 = (locals.var_fn97_calc_iq__qref * assign8810_e9505);
        (assign8810_e9506, (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn2) / assign8810_e9504)), (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn3) / assign8810_e9504)), ((locals.var_fn97_calc_iq__qref_dn4 * assign8810_e9505) + (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn4) / assign8810_e9504))), (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn7) / assign8810_e9504)), ((locals.var_fn97_calc_iq__qref_dn14 * assign8810_e9505) + (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn14) / assign8810_e9504))), ((locals.var_fn97_calc_iq__qref_dn15 * assign8810_e9505) + (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn15) / assign8810_e9504))),)
    } else {
        (locals.var_fn97_calc_iq__qinvv, locals.var_fn97_calc_iq__qinvv_dn2, locals.var_fn97_calc_iq__qinvv_dn3, locals.var_fn97_calc_iq__qinvv_dn4, locals.var_fn97_calc_iq__qinvv_dn7, locals.var_fn97_calc_iq__qinvv_dn14, locals.var_fn97_calc_iq__qinvv_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv = assign8810_e9508;
        locals.var_fn97_calc_iq__qinvv_dn2 = assign8810_e9508_d_n2;
        locals.var_fn97_calc_iq__qinvv_dn3 = assign8810_e9508_d_n3;
        locals.var_fn97_calc_iq__qinvv_dn4 = assign8810_e9508_d_n4;
        locals.var_fn97_calc_iq__qinvv_dn7 = assign8810_e9508_d_n7;
        locals.var_fn97_calc_iq__qinvv_dn14 = assign8810_e9508_d_n14;
        locals.var_fn97_calc_iq__qinvv_dn15 = assign8810_e9508_d_n15;

        let (assign8820_e9522, assign8820_e9522_d_n2, assign8820_e9522_d_n3, assign8820_e9522_d_n4, assign8820_e9522_d_n7, assign8820_e9522_d_n14, assign8820_e9522_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8820_e9515: f64 = (locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv);
        let assign8820_e9517: f64 = (assign8820_e9515 / locals.var_fn97_calc_iq__cgin);
        let assign8820_e9518: f64 = (1.0 + assign8820_e9517);
        let assign8820_e9519: f64 = (locals.var_fn97_calc_iq__tfacmobin * assign8820_e9518);
        let assign8820_e9520: f64 = (locals.var_fn97_calc_iq__mu0 / assign8820_e9519);
        (assign8820_e9520, (-((locals.var_fn97_calc_iq__mu0 * (locals.var_fn97_calc_iq__tfacmobin * ((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn2) / locals.var_fn97_calc_iq__cgin))) / (assign8820_e9519 * assign8820_e9519))), (-((locals.var_fn97_calc_iq__mu0 * (locals.var_fn97_calc_iq__tfacmobin * ((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn3) / locals.var_fn97_calc_iq__cgin))) / (assign8820_e9519 * assign8820_e9519))), (-((locals.var_fn97_calc_iq__mu0 * ((locals.var_fn97_calc_iq__tfacmobin_dn4 * assign8820_e9518) + (locals.var_fn97_calc_iq__tfacmobin * ((((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn4) * locals.var_fn97_calc_iq__cgin) - (assign8820_e9515 * locals.var_fn97_calc_iq__cgin_dn4)) / (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__cgin))))) / (assign8820_e9519 * assign8820_e9519))), (-((locals.var_fn97_calc_iq__mu0 * (locals.var_fn97_calc_iq__tfacmobin * ((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn7) / locals.var_fn97_calc_iq__cgin))) / (assign8820_e9519 * assign8820_e9519))), (-((locals.var_fn97_calc_iq__mu0 * (locals.var_fn97_calc_iq__tfacmobin * ((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn14) / locals.var_fn97_calc_iq__cgin))) / (assign8820_e9519 * assign8820_e9519))), (-((locals.var_fn97_calc_iq__mu0 * (locals.var_fn97_calc_iq__tfacmobin * ((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn15) / locals.var_fn97_calc_iq__cgin))) / (assign8820_e9519 * assign8820_e9519))),)
    } else {
        (locals.var_fn97_calc_iq__muf, locals.var_fn97_calc_iq__muf_dn2, locals.var_fn97_calc_iq__muf_dn3, locals.var_fn97_calc_iq__muf_dn4, locals.var_fn97_calc_iq__muf_dn7, locals.var_fn97_calc_iq__muf_dn14, locals.var_fn97_calc_iq__muf_dn15,)
    }
};
        locals.var_fn97_calc_iq__muf = assign8820_e9522;
        locals.var_fn97_calc_iq__muf_dn2 = assign8820_e9522_d_n2;
        locals.var_fn97_calc_iq__muf_dn3 = assign8820_e9522_d_n3;
        locals.var_fn97_calc_iq__muf_dn4 = assign8820_e9522_d_n4;
        locals.var_fn97_calc_iq__muf_dn7 = assign8820_e9522_d_n7;
        locals.var_fn97_calc_iq__muf_dn14 = assign8820_e9522_d_n14;
        locals.var_fn97_calc_iq__muf_dn15 = assign8820_e9522_d_n15;

        let (assign8830_e9554, assign8830_e9554_d_n2, assign8830_e9554_d_n3, assign8830_e9554_d_n4, assign8830_e9554_d_n7, assign8830_e9554_d_n14, assign8830_e9554_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8830_e9528: f64 = (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tnomin);
        let assign8830_e9529: f64 = (1.0 + assign8830_e9528);
        let assign8830_e9533: f64 = (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tambin);
        let assign8830_e9534: f64 = (1.0 + assign8830_e9533);
        let assign8830_e9535: f64 = (assign8830_e9529 / assign8830_e9534);
        let assign8830_e9536: f64 = (locals.var_fn97_calc_iq__vel0 * assign8830_e9535);
        let assign8830_e9540: f64 = (locals.var_fn97_calc_iq__lambda * locals.var_fn97_calc_iq__absvdsin);
        let assign8830_e9542: f64 = (assign8830_e9540 / locals.var_fn97_calc_iq__lin);
        let assign8830_e9543: f64 = (1.0 + assign8830_e9542);
        let assign8830_e9544: f64 = (assign8830_e9536 * assign8830_e9543);
        let assign8830_e9548: f64 = (locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv);
        let assign8830_e9550: f64 = (assign8830_e9548 / locals.var_fn97_calc_iq__cgin);
        let assign8830_e9551: f64 = (1.0 + assign8830_e9550);
        let assign8830_e9552: f64 = (assign8830_e9544 / assign8830_e9551);
        (assign8830_e9552, (-((assign8830_e9544 * ((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn2) / locals.var_fn97_calc_iq__cgin)) / (assign8830_e9551 * assign8830_e9551))), (-((assign8830_e9544 * ((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn3) / locals.var_fn97_calc_iq__cgin)) / (assign8830_e9551 * assign8830_e9551))), (((((locals.var_fn97_calc_iq__vel0 * (-((assign8830_e9529 * (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tambin_dn4)) / (assign8830_e9534 * assign8830_e9534)))) * assign8830_e9543) * assign8830_e9551) - (assign8830_e9544 * ((((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn4) * locals.var_fn97_calc_iq__cgin) - (assign8830_e9548 * locals.var_fn97_calc_iq__cgin_dn4)) / (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__cgin)))) / (assign8830_e9551 * assign8830_e9551)), (-((assign8830_e9544 * ((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn7) / locals.var_fn97_calc_iq__cgin)) / (assign8830_e9551 * assign8830_e9551))), ((((assign8830_e9536 * ((locals.var_fn97_calc_iq__lambda * locals.var_fn97_calc_iq__absvdsin_dn14) / locals.var_fn97_calc_iq__lin)) * assign8830_e9551) - (assign8830_e9544 * ((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn14) / locals.var_fn97_calc_iq__cgin))) / (assign8830_e9551 * assign8830_e9551)), ((((assign8830_e9536 * ((locals.var_fn97_calc_iq__lambda * locals.var_fn97_calc_iq__absvdsin_dn15) / locals.var_fn97_calc_iq__lin)) * assign8830_e9551) - (assign8830_e9544 * ((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn15) / locals.var_fn97_calc_iq__cgin))) / (assign8830_e9551 * assign8830_e9551)),)
    } else {
        (locals.var_fn97_calc_iq__vx, locals.var_fn97_calc_iq__vx_dn2, locals.var_fn97_calc_iq__vx_dn3, locals.var_fn97_calc_iq__vx_dn4, locals.var_fn97_calc_iq__vx_dn7, locals.var_fn97_calc_iq__vx_dn14, locals.var_fn97_calc_iq__vx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vx = assign8830_e9554;
        locals.var_fn97_calc_iq__vx_dn2 = assign8830_e9554_d_n2;
        locals.var_fn97_calc_iq__vx_dn3 = assign8830_e9554_d_n3;
        locals.var_fn97_calc_iq__vx_dn4 = assign8830_e9554_d_n4;
        locals.var_fn97_calc_iq__vx_dn7 = assign8830_e9554_d_n7;
        locals.var_fn97_calc_iq__vx_dn14 = assign8830_e9554_d_n14;
        locals.var_fn97_calc_iq__vx_dn15 = assign8830_e9554_d_n15;

        let (assign8850_e9580, assign8850_e9580_d_n2, assign8850_e9580_d_n3, assign8850_e9580_d_n4, assign8850_e9580_d_n7, assign8850_e9580_d_n14, assign8850_e9580_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8850_e9576: f64 = (locals.var_fn97_calc_iq__vx * locals.var_fn97_calc_iq__lin);
        let assign8850_e9578: f64 = (assign8850_e9576 / locals.var_fn97_calc_iq__muf);
        (assign8850_e9578, ((((locals.var_fn97_calc_iq__vx_dn2 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn2)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)), ((((locals.var_fn97_calc_iq__vx_dn3 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn3)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)), ((((locals.var_fn97_calc_iq__vx_dn4 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn4)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)), ((((locals.var_fn97_calc_iq__vx_dn7 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn7)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)), ((((locals.var_fn97_calc_iq__vx_dn14 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn14)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)), ((((locals.var_fn97_calc_iq__vx_dn15 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn15)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)),)
    } else {
        (locals.var_fn97_calc_iq__vdsats, locals.var_fn97_calc_iq__vdsats_dn2, locals.var_fn97_calc_iq__vdsats_dn3, locals.var_fn97_calc_iq__vdsats_dn4, locals.var_fn97_calc_iq__vdsats_dn7, locals.var_fn97_calc_iq__vdsats_dn14, locals.var_fn97_calc_iq__vdsats_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats = assign8850_e9580;
        locals.var_fn97_calc_iq__vdsats_dn2 = assign8850_e9580_d_n2;
        locals.var_fn97_calc_iq__vdsats_dn3 = assign8850_e9580_d_n3;
        locals.var_fn97_calc_iq__vdsats_dn4 = assign8850_e9580_d_n4;
        locals.var_fn97_calc_iq__vdsats_dn7 = assign8850_e9580_d_n7;
        locals.var_fn97_calc_iq__vdsats_dn14 = assign8850_e9580_d_n14;
        locals.var_fn97_calc_iq__vdsats_dn15 = assign8850_e9580_d_n15;

        let (assign8860_e9597, assign8860_e9597_d_n2, assign8860_e9597_d_n3, assign8860_e9597_d_n4, assign8860_e9597_d_n7, assign8860_e9597_d_n14, assign8860_e9597_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8860_e9586: f64 = (2.0 * locals.var_fn97_calc_iq__qinvv);
        let assign8860_e9588: f64 = (assign8860_e9586 / locals.var_fn97_calc_iq__cgin);
        let assign8860_e9590: f64 = (assign8860_e9588 / locals.var_fn97_calc_iq__vdsats);
        let assign8860_e9591: f64 = (1.0 + assign8860_e9590);
        let assign8860_e9592: f64 = (assign8860_e9591).sqrt();
        let assign8860_e9593: f64 = (locals.var_fn97_calc_iq__vdsats * assign8860_e9592);
        let assign8860_e9595: f64 = (assign8860_e9593 - locals.var_fn97_calc_iq__vdsats);
        (assign8860_e9595, (((locals.var_fn97_calc_iq__vdsats_dn2 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn2) / locals.var_fn97_calc_iq__cgin) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn2)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn2), (((locals.var_fn97_calc_iq__vdsats_dn3 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn3) / locals.var_fn97_calc_iq__cgin) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn3)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn3), (((locals.var_fn97_calc_iq__vdsats_dn4 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn4) * locals.var_fn97_calc_iq__cgin) - (assign8860_e9586 * locals.var_fn97_calc_iq__cgin_dn4)) / (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__cgin)) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn4)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn4), (((locals.var_fn97_calc_iq__vdsats_dn7 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn7) / locals.var_fn97_calc_iq__cgin) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn7)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn7), (((locals.var_fn97_calc_iq__vdsats_dn14 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn14) / locals.var_fn97_calc_iq__cgin) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn14)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn14), (((locals.var_fn97_calc_iq__vdsats_dn15 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn15) / locals.var_fn97_calc_iq__cgin) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn15)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn15),)
    } else {
        (locals.var_fn97_calc_iq__vdsats1, locals.var_fn97_calc_iq__vdsats1_dn2, locals.var_fn97_calc_iq__vdsats1_dn3, locals.var_fn97_calc_iq__vdsats1_dn4, locals.var_fn97_calc_iq__vdsats1_dn7, locals.var_fn97_calc_iq__vdsats1_dn14, locals.var_fn97_calc_iq__vdsats1_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats1 = assign8860_e9597;
        locals.var_fn97_calc_iq__vdsats1_dn2 = assign8860_e9597_d_n2;
        locals.var_fn97_calc_iq__vdsats1_dn3 = assign8860_e9597_d_n3;
        locals.var_fn97_calc_iq__vdsats1_dn4 = assign8860_e9597_d_n4;
        locals.var_fn97_calc_iq__vdsats1_dn7 = assign8860_e9597_d_n7;
        locals.var_fn97_calc_iq__vdsats1_dn14 = assign8860_e9597_d_n14;
        locals.var_fn97_calc_iq__vdsats1_dn15 = assign8860_e9597_d_n15;

        let (assign8870_e9609, assign8870_e9609_d_n2, assign8870_e9609_d_n3, assign8870_e9609_d_n4, assign8870_e9609_d_n7, assign8870_e9609_d_n14, assign8870_e9609_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8870_e9602: f64 = (1.0 - locals.var_fn97_calc_iq__ff);
        let assign8870_e9603: f64 = (locals.var_fn97_calc_iq__vdsats * assign8870_e9602);
        let assign8870_e9606: f64 = (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff);
        let assign8870_e9607: f64 = (assign8870_e9603 + assign8870_e9606);
        (assign8870_e9607, (((locals.var_fn97_calc_iq__vdsats_dn2 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn2))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn2)), (((locals.var_fn97_calc_iq__vdsats_dn3 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn3))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn3)), (((locals.var_fn97_calc_iq__vdsats_dn4 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn4))) + ((locals.var_fn97_calc_iq__two_n_phit_dn4 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn4))), (((locals.var_fn97_calc_iq__vdsats_dn7 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn7))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn7)), (((locals.var_fn97_calc_iq__vdsats_dn14 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn14))) + ((locals.var_fn97_calc_iq__two_n_phit_dn14 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn14))), (((locals.var_fn97_calc_iq__vdsats_dn15 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn15))) + ((locals.var_fn97_calc_iq__two_n_phit_dn15 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn15))),)
    } else {
        (locals.var_fn97_calc_iq__vdsat, locals.var_fn97_calc_iq__vdsat_dn2, locals.var_fn97_calc_iq__vdsat_dn3, locals.var_fn97_calc_iq__vdsat_dn4, locals.var_fn97_calc_iq__vdsat_dn7, locals.var_fn97_calc_iq__vdsat_dn14, locals.var_fn97_calc_iq__vdsat_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat = assign8870_e9609;
        locals.var_fn97_calc_iq__vdsat_dn2 = assign8870_e9609_d_n2;
        locals.var_fn97_calc_iq__vdsat_dn3 = assign8870_e9609_d_n3;
        locals.var_fn97_calc_iq__vdsat_dn4 = assign8870_e9609_d_n4;
        locals.var_fn97_calc_iq__vdsat_dn7 = assign8870_e9609_d_n7;
        locals.var_fn97_calc_iq__vdsat_dn14 = assign8870_e9609_d_n14;
        locals.var_fn97_calc_iq__vdsat_dn15 = assign8870_e9609_d_n15;

        let (assign8880_e9621, assign8880_e9621_d_n2, assign8880_e9621_d_n3, assign8880_e9621_d_n4, assign8880_e9621_d_n7, assign8880_e9621_d_n14, assign8880_e9621_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8880_e9614: f64 = (1.0 - locals.var_fn97_calc_iq__ff);
        let assign8880_e9615: f64 = (locals.var_fn97_calc_iq__vdsats1 * assign8880_e9614);
        let assign8880_e9618: f64 = (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff);
        let assign8880_e9619: f64 = (assign8880_e9615 + assign8880_e9618);
        (assign8880_e9619, (((locals.var_fn97_calc_iq__vdsats1_dn2 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn2))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn2)), (((locals.var_fn97_calc_iq__vdsats1_dn3 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn3))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn3)), (((locals.var_fn97_calc_iq__vdsats1_dn4 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn4))) + ((locals.var_fn97_calc_iq__two_n_phit_dn4 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn4))), (((locals.var_fn97_calc_iq__vdsats1_dn7 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn7))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn7)), (((locals.var_fn97_calc_iq__vdsats1_dn14 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn14))) + ((locals.var_fn97_calc_iq__two_n_phit_dn14 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn14))), (((locals.var_fn97_calc_iq__vdsats1_dn15 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn15))) + ((locals.var_fn97_calc_iq__two_n_phit_dn15 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn15))),)
    } else {
        (locals.var_fn97_calc_iq__vdsat1, locals.var_fn97_calc_iq__vdsat1_dn2, locals.var_fn97_calc_iq__vdsat1_dn3, locals.var_fn97_calc_iq__vdsat1_dn4, locals.var_fn97_calc_iq__vdsat1_dn7, locals.var_fn97_calc_iq__vdsat1_dn14, locals.var_fn97_calc_iq__vdsat1_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat1 = assign8880_e9621;
        locals.var_fn97_calc_iq__vdsat1_dn2 = assign8880_e9621_d_n2;
        locals.var_fn97_calc_iq__vdsat1_dn3 = assign8880_e9621_d_n3;
        locals.var_fn97_calc_iq__vdsat1_dn4 = assign8880_e9621_d_n4;
        locals.var_fn97_calc_iq__vdsat1_dn7 = assign8880_e9621_d_n7;
        locals.var_fn97_calc_iq__vdsat1_dn14 = assign8880_e9621_d_n14;
        locals.var_fn97_calc_iq__vdsat1_dn15 = assign8880_e9621_d_n15;

        let (assign8890_e9690, assign8890_e9690_d_n2, assign8890_e9690_d_n3, assign8890_e9690_d_n4, assign8890_e9690_d_n7, assign8890_e9690_d_n14, assign8890_e9690_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign8890_e9680, assign8890_e9680_d_n2, assign8890_e9680_d_n3, assign8890_e9680_d_n4, assign8890_e9680_d_n7, assign8890_e9680_d_n14, assign8890_e9680_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign8890_e9633: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                let assign8890_e9634: f64 = assign8890_e9633;
                let assign8890_e9638: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                let assign8890_e9639: f64 = (-assign8890_e9638);
                let assign8890_e9642: f64 = (0.001 / p.p53);
                let assign8890_e9646: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                let assign8890_e9647: f64 = (-assign8890_e9646);
                let assign8890_e9648: f64 = (assign8890_e9642 * assign8890_e9647);
                let assign8890_e9649: f64 = (assign8890_e9648).tanh();
                let assign8890_e9650: f64 = (assign8890_e9639 * assign8890_e9649);
                let assign8890_e9651: f64 = (assign8890_e9634 + assign8890_e9650);
                let assign8890_e9652: f64 = (0.5 * assign8890_e9651);
                (assign8890_e9652, (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + (((-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + (((-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))),)
            } else {
                let (assign8890_e9679, assign8890_e9679_d_n2, assign8890_e9679_d_n3, assign8890_e9679_d_n4, assign8890_e9679_d_n7, assign8890_e9679_d_n14, assign8890_e9679_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign8890_e9660: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                        let assign8890_e9661: f64 = assign8890_e9660;
                        let assign8890_e9665: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                        let assign8890_e9666: f64 = (-assign8890_e9665);
                        let assign8890_e9670: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                        let assign8890_e9671: f64 = (-assign8890_e9670);
                        let assign8890_e9672: f64 = (assign8890_e9666 * assign8890_e9671);
                        let assign8890_e9674: f64 = (assign8890_e9672 + p.p53);
                        let assign8890_e9675: f64 = (assign8890_e9674).sqrt();
                        let assign8890_e9676: f64 = (assign8890_e9661 + assign8890_e9675);
                        let assign8890_e9677: f64 = (0.5 * assign8890_e9676);
                        (assign8890_e9677, (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9671) + (assign8890_e9666 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8890_e9675)))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9671) + (assign8890_e9666 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8890_e9675)))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9671) + (assign8890_e9666 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8890_e9675)))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9671) + (assign8890_e9666 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8890_e9675)))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + ((((-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8890_e9671) + (assign8890_e9666 * (-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / (2.0 * assign8890_e9675)))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + ((((-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8890_e9671) + (assign8890_e9666 * (-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / (2.0 * assign8890_e9675)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign8890_e9679, assign8890_e9679_d_n2, assign8890_e9679_d_n3, assign8890_e9679_d_n4, assign8890_e9679_d_n7, assign8890_e9679_d_n14, assign8890_e9679_d_n15,)
            }
        };
        let assign8890_e9682: f64 = (assign8890_e9680).powf(locals.var_fn97_calc_iq__beta);
        let assign8890_e9683: f64 = (1.0 + assign8890_e9682);
        let assign8890_e9686: f64 = (1.0 / locals.var_fn97_calc_iq__beta);
        let assign8890_e9687: f64 = (assign8890_e9683).powf(assign8890_e9686);
        let assign8890_e9688: f64 = (1.0 / assign8890_e9687);
        (assign8890_e9688, (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n2)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n2 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n2)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n2 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))), (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n3)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n3 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n3)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n3 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))), (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n4)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n4 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n4)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n4 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))), (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n7)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n7 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n7)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n7 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))), (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n14)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n14 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n14)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n14 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))), (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n15)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n15 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n15)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n15 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))),)
    } else {
        (locals.var_fn97_calc_iq__fsd, locals.var_fn97_calc_iq__fsd_dn2, locals.var_fn97_calc_iq__fsd_dn3, locals.var_fn97_calc_iq__fsd_dn4, locals.var_fn97_calc_iq__fsd_dn7, locals.var_fn97_calc_iq__fsd_dn14, locals.var_fn97_calc_iq__fsd_dn15,)
    }
};
        locals.var_fn97_calc_iq__fsd = assign8890_e9690;
        locals.var_fn97_calc_iq__fsd_dn2 = assign8890_e9690_d_n2;
        locals.var_fn97_calc_iq__fsd_dn3 = assign8890_e9690_d_n3;
        locals.var_fn97_calc_iq__fsd_dn4 = assign8890_e9690_d_n4;
        locals.var_fn97_calc_iq__fsd_dn7 = assign8890_e9690_d_n7;
        locals.var_fn97_calc_iq__fsd_dn14 = assign8890_e9690_d_n14;
        locals.var_fn97_calc_iq__fsd_dn15 = assign8890_e9690_d_n15;

    }

    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8900_e9696, assign8900_e9696_d_n2, assign8900_e9696_d_n3, assign8900_e9696_d_n4, assign8900_e9696_d_n7, assign8900_e9696_d_n14, assign8900_e9696_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8900_e9694: f64 = (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd);
        (assign8900_e9694, (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn2), (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn3), (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn4), (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn7), ((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__fsd) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn14)), ((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__fsd) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__vdx, locals.var_fn97_calc_iq__vdx_dn2, locals.var_fn97_calc_iq__vdx_dn3, locals.var_fn97_calc_iq__vdx_dn4, locals.var_fn97_calc_iq__vdx_dn7, locals.var_fn97_calc_iq__vdx_dn14, locals.var_fn97_calc_iq__vdx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdx = assign8900_e9696;
        locals.var_fn97_calc_iq__vdx_dn2 = assign8900_e9696_d_n2;
        locals.var_fn97_calc_iq__vdx_dn3 = assign8900_e9696_d_n3;
        locals.var_fn97_calc_iq__vdx_dn4 = assign8900_e9696_d_n4;
        locals.var_fn97_calc_iq__vdx_dn7 = assign8900_e9696_d_n7;
        locals.var_fn97_calc_iq__vdx_dn14 = assign8900_e9696_d_n14;
        locals.var_fn97_calc_iq__vdx_dn15 = assign8900_e9696_d_n15;

        let (assign8910_e9771, assign8910_e9771_d_n2, assign8910_e9771_d_n3, assign8910_e9771_d_n4, assign8910_e9771_d_n7, assign8910_e9771_d_n14, assign8910_e9771_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign8910_e9761, assign8910_e9761_d_n2, assign8910_e9761_d_n3, assign8910_e9761_d_n4, assign8910_e9761_d_n7, assign8910_e9761_d_n14, assign8910_e9761_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign8910_e9707: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign8910_e9709: f64 = (assign8910_e9707 / locals.var_fn97_calc_iq__vdsat1);
                let assign8910_e9710: f64 = assign8910_e9709;
                let assign8910_e9713: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign8910_e9715: f64 = (assign8910_e9713 / locals.var_fn97_calc_iq__vdsat1);
                let assign8910_e9716: f64 = (-assign8910_e9715);
                let assign8910_e9719: f64 = (0.001 / p.p53);
                let assign8910_e9722: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign8910_e9724: f64 = (assign8910_e9722 / locals.var_fn97_calc_iq__vdsat1);
                let assign8910_e9725: f64 = (-assign8910_e9724);
                let assign8910_e9726: f64 = (assign8910_e9719 * assign8910_e9725);
                let assign8910_e9727: f64 = (assign8910_e9726).tanh();
                let assign8910_e9728: f64 = (assign8910_e9716 * assign8910_e9727);
                let assign8910_e9729: f64 = (assign8910_e9710 + assign8910_e9728);
                let assign8910_e9730: f64 = (0.5 * assign8910_e9729);
                (assign8910_e9730, (0.5 * ((-((assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-(-((assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))), (0.5 * ((-((assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-(-((assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))), (0.5 * ((-((assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-(-((assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))), (0.5 * ((-((assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-(-((assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + (((-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + (((-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))),)
            } else {
                let (assign8910_e9760, assign8910_e9760_d_n2, assign8910_e9760_d_n3, assign8910_e9760_d_n4, assign8910_e9760_d_n7, assign8910_e9760_d_n14, assign8910_e9760_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign8910_e9737: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign8910_e9739: f64 = (assign8910_e9737 / locals.var_fn97_calc_iq__vdsat1);
                        let assign8910_e9740: f64 = assign8910_e9739;
                        let assign8910_e9743: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign8910_e9745: f64 = (assign8910_e9743 / locals.var_fn97_calc_iq__vdsat1);
                        let assign8910_e9746: f64 = (-assign8910_e9745);
                        let assign8910_e9749: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign8910_e9751: f64 = (assign8910_e9749 / locals.var_fn97_calc_iq__vdsat1);
                        let assign8910_e9752: f64 = (-assign8910_e9751);
                        let assign8910_e9753: f64 = (assign8910_e9746 * assign8910_e9752);
                        let assign8910_e9755: f64 = (assign8910_e9753 + p.p53);
                        let assign8910_e9756: f64 = (assign8910_e9755).sqrt();
                        let assign8910_e9757: f64 = (assign8910_e9740 + assign8910_e9756);
                        let assign8910_e9758: f64 = (0.5 * assign8910_e9757);
                        (assign8910_e9758, (0.5 * ((-((assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9752) + (assign8910_e9746 * (-(-((assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8910_e9756)))), (0.5 * ((-((assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9752) + (assign8910_e9746 * (-(-((assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8910_e9756)))), (0.5 * ((-((assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9752) + (assign8910_e9746 * (-(-((assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8910_e9756)))), (0.5 * ((-((assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9752) + (assign8910_e9746 * (-(-((assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8910_e9756)))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + ((((-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8910_e9752) + (assign8910_e9746 * (-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / (2.0 * assign8910_e9756)))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + ((((-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8910_e9752) + (assign8910_e9746 * (-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / (2.0 * assign8910_e9756)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign8910_e9760, assign8910_e9760_d_n2, assign8910_e9760_d_n3, assign8910_e9760_d_n4, assign8910_e9760_d_n7, assign8910_e9760_d_n14, assign8910_e9760_d_n15,)
            }
        };
        let assign8910_e9763: f64 = (assign8910_e9761).powf(locals.var_fn97_calc_iq__beta);
        let assign8910_e9764: f64 = (1.0 + assign8910_e9763);
        let assign8910_e9767: f64 = (1.0 / locals.var_fn97_calc_iq__beta);
        let assign8910_e9768: f64 = (assign8910_e9764).powf(assign8910_e9767);
        let assign8910_e9769: f64 = (1.0 / assign8910_e9768);
        (assign8910_e9769, (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n2)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n2 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n2)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n2 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))), (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n3)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n3 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n3)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n3 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))), (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n4)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n4 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n4)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n4 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))), (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n7)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n7 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n7)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n7 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))), (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n14)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n14 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n14)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n14 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))), (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n15)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n15 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n15)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n15 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))),)
    } else {
        (locals.var_fn97_calc_iq__fds, locals.var_fn97_calc_iq__fds_dn2, locals.var_fn97_calc_iq__fds_dn3, locals.var_fn97_calc_iq__fds_dn4, locals.var_fn97_calc_iq__fds_dn7, locals.var_fn97_calc_iq__fds_dn14, locals.var_fn97_calc_iq__fds_dn15,)
    }
};
        locals.var_fn97_calc_iq__fds = assign8910_e9771;
        locals.var_fn97_calc_iq__fds_dn2 = assign8910_e9771_d_n2;
        locals.var_fn97_calc_iq__fds_dn3 = assign8910_e9771_d_n3;
        locals.var_fn97_calc_iq__fds_dn4 = assign8910_e9771_d_n4;
        locals.var_fn97_calc_iq__fds_dn7 = assign8910_e9771_d_n7;
        locals.var_fn97_calc_iq__fds_dn14 = assign8910_e9771_d_n14;
        locals.var_fn97_calc_iq__fds_dn15 = assign8910_e9771_d_n15;

        let (assign8920_e9778, assign8920_e9778_d_n2, assign8920_e9778_d_n3, assign8920_e9778_d_n4, assign8920_e9778_d_n7, assign8920_e9778_d_n14, assign8920_e9778_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8920_e9774: f64 = (-locals.var_fn97_calc_iq__vdsin);
        let assign8920_e9776: f64 = (assign8920_e9774 * locals.var_fn97_calc_iq__fds);
        (assign8920_e9776, (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn2), (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn3), (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn4), (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn7), (((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__fds) + (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn14)), (((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__fds) + (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__vsx, locals.var_fn97_calc_iq__vsx_dn2, locals.var_fn97_calc_iq__vsx_dn3, locals.var_fn97_calc_iq__vsx_dn4, locals.var_fn97_calc_iq__vsx_dn7, locals.var_fn97_calc_iq__vsx_dn14, locals.var_fn97_calc_iq__vsx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsx = assign8920_e9778;
        locals.var_fn97_calc_iq__vsx_dn2 = assign8920_e9778_d_n2;
        locals.var_fn97_calc_iq__vsx_dn3 = assign8920_e9778_d_n3;
        locals.var_fn97_calc_iq__vsx_dn4 = assign8920_e9778_d_n4;
        locals.var_fn97_calc_iq__vsx_dn7 = assign8920_e9778_d_n7;
        locals.var_fn97_calc_iq__vsx_dn14 = assign8920_e9778_d_n14;
        locals.var_fn97_calc_iq__vsx_dn15 = assign8920_e9778_d_n15;

        let (assign8930_e9786, assign8930_e9786_d_n2, assign8930_e9786_d_n3, assign8930_e9786_d_n4, assign8930_e9786_d_n7, assign8930_e9786_d_n14, assign8930_e9786_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8930_e9782: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__myarg);
        let assign8930_e9784: f64 = (assign8930_e9782 / locals.var_fn97_calc_iq__alpha_phit);
        (assign8930_e9784, ((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__myarg_dn2) / locals.var_fn97_calc_iq__alpha_phit), ((-locals.var_fn97_calc_iq__myarg_dn3) / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign8930_e9782 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), ((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__myarg_dn7) / locals.var_fn97_calc_iq__alpha_phit), ((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__myarg_dn14) / locals.var_fn97_calc_iq__alpha_phit), ((-locals.var_fn97_calc_iq__myarg_dn15) / locals.var_fn97_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign8930_e9786;
        locals.var_fn97_calc_iq__exparg_dn2 = assign8930_e9786_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign8930_e9786_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign8930_e9786_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign8930_e9786_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign8930_e9786_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign8930_e9786_d_n15;

        let assign8940_e9789: f64 = if locals.var_fn97_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard103 = assign8940_e9789;

        let (assign8950_e9795, assign8950_e9795_d_n2, assign8950_e9795_d_n3, assign8950_e9795_d_n4, assign8950_e9795_d_n7, assign8950_e9795_d_n14, assign8950_e9795_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard103 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs, locals.var_fn97_calc_iq__ffs_dn2, locals.var_fn97_calc_iq__ffs_dn3, locals.var_fn97_calc_iq__ffs_dn4, locals.var_fn97_calc_iq__ffs_dn7, locals.var_fn97_calc_iq__ffs_dn14, locals.var_fn97_calc_iq__ffs_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs = assign8950_e9795;
        locals.var_fn97_calc_iq__ffs_dn2 = assign8950_e9795_d_n2;
        locals.var_fn97_calc_iq__ffs_dn3 = assign8950_e9795_d_n3;
        locals.var_fn97_calc_iq__ffs_dn4 = assign8950_e9795_d_n4;
        locals.var_fn97_calc_iq__ffs_dn7 = assign8950_e9795_d_n7;
        locals.var_fn97_calc_iq__ffs_dn14 = assign8950_e9795_d_n14;
        locals.var_fn97_calc_iq__ffs_dn15 = assign8950_e9795_d_n15;

        let assign8960_e9798: f64 = (-50.0);
        let assign8960_e9799: f64 = if locals.var_fn97_calc_iq__exparg < assign8960_e9798 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign8960_e9799;

        let (assign8970_e9808, assign8970_e9808_d_n2, assign8970_e9808_d_n3, assign8970_e9808_d_n4, assign8970_e9808_d_n7, assign8970_e9808_d_n14, assign8970_e9808_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard103 == 0.0)) && (locals.var_guard104 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs, locals.var_fn97_calc_iq__ffs_dn2, locals.var_fn97_calc_iq__ffs_dn3, locals.var_fn97_calc_iq__ffs_dn4, locals.var_fn97_calc_iq__ffs_dn7, locals.var_fn97_calc_iq__ffs_dn14, locals.var_fn97_calc_iq__ffs_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs = assign8970_e9808;
        locals.var_fn97_calc_iq__ffs_dn2 = assign8970_e9808_d_n2;
        locals.var_fn97_calc_iq__ffs_dn3 = assign8970_e9808_d_n3;
        locals.var_fn97_calc_iq__ffs_dn4 = assign8970_e9808_d_n4;
        locals.var_fn97_calc_iq__ffs_dn7 = assign8970_e9808_d_n7;
        locals.var_fn97_calc_iq__ffs_dn14 = assign8970_e9808_d_n14;
        locals.var_fn97_calc_iq__ffs_dn15 = assign8970_e9808_d_n15;

        let (assign8980_e9823, assign8980_e9823_d_n2, assign8980_e9823_d_n3, assign8980_e9823_d_n4, assign8980_e9823_d_n7, assign8980_e9823_d_n14, assign8980_e9823_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard103 == 0.0)) && (locals.var_guard104 == 0.0)) {
        let assign8980_e9819: f64 = (locals.var_fn97_calc_iq__exparg).exp();
        let assign8980_e9820: f64 = (1.0 + assign8980_e9819);
        let assign8980_e9821: f64 = (1.0 / assign8980_e9820);
        (assign8980_e9821, (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn2) / (assign8980_e9820 * assign8980_e9820))), (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn3) / (assign8980_e9820 * assign8980_e9820))), (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn4) / (assign8980_e9820 * assign8980_e9820))), (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn7) / (assign8980_e9820 * assign8980_e9820))), (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn14) / (assign8980_e9820 * assign8980_e9820))), (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn15) / (assign8980_e9820 * assign8980_e9820))),)
    } else {
        (locals.var_fn97_calc_iq__ffs, locals.var_fn97_calc_iq__ffs_dn2, locals.var_fn97_calc_iq__ffs_dn3, locals.var_fn97_calc_iq__ffs_dn4, locals.var_fn97_calc_iq__ffs_dn7, locals.var_fn97_calc_iq__ffs_dn14, locals.var_fn97_calc_iq__ffs_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs = assign8980_e9823;
        locals.var_fn97_calc_iq__ffs_dn2 = assign8980_e9823_d_n2;
        locals.var_fn97_calc_iq__ffs_dn3 = assign8980_e9823_d_n3;
        locals.var_fn97_calc_iq__ffs_dn4 = assign8980_e9823_d_n4;
        locals.var_fn97_calc_iq__ffs_dn7 = assign8980_e9823_d_n7;
        locals.var_fn97_calc_iq__ffs_dn14 = assign8980_e9823_d_n14;
        locals.var_fn97_calc_iq__ffs_dn15 = assign8980_e9823_d_n15;

        let (assign8990_e9841, assign8990_e9841_d_n2, assign8990_e9841_d_n3, assign8990_e9841_d_n4, assign8990_e9841_d_n7, assign8990_e9841_d_n14, assign8990_e9841_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8990_e9827: f64 = (locals.var_fn97_calc_iq__vgdin - locals.var_fn97_calc_iq__vsx);
        let assign8990_e9831: f64 = (p.p51 * 0.1);
        let assign8990_e9833: f64 = (assign8990_e9831 * locals.var_fn97_calc_iq__alpha_phit);
        let assign8990_e9835: f64 = (assign8990_e9833 * locals.var_fn97_calc_iq__ffs);
        let assign8990_e9836: f64 = (locals.var_fn97_calc_iq__vtdibl - assign8990_e9835);
        let assign8990_e9837: f64 = (assign8990_e9827 - assign8990_e9836);
        let assign8990_e9839: f64 = (assign8990_e9837 / locals.var_fn97_calc_iq__two_n_phit);
        (assign8990_e9839, (((locals.var_fn97_calc_iq__vgdin_dn2 - locals.var_fn97_calc_iq__vsx_dn2) - (-(assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn2))) / locals.var_fn97_calc_iq__two_n_phit), (((-locals.var_fn97_calc_iq__vsx_dn3) - (-(assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn3))) / locals.var_fn97_calc_iq__two_n_phit), (((((-locals.var_fn97_calc_iq__vsx_dn4) - (locals.var_fn97_calc_iq__vtdibl_dn4 - (((assign8990_e9831 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ffs) + (assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn4)))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8990_e9837 * locals.var_fn97_calc_iq__two_n_phit_dn4)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), (((locals.var_fn97_calc_iq__vgdin_dn7 - locals.var_fn97_calc_iq__vsx_dn7) - (-(assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn7))) / locals.var_fn97_calc_iq__two_n_phit), (((((locals.var_fn97_calc_iq__vgdin_dn14 - locals.var_fn97_calc_iq__vsx_dn14) - (locals.var_fn97_calc_iq__vtdibl_dn14 - (assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn14))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8990_e9837 * locals.var_fn97_calc_iq__two_n_phit_dn14)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), (((((locals.var_fn97_calc_iq__vgdin_dn15 - locals.var_fn97_calc_iq__vsx_dn15) - (locals.var_fn97_calc_iq__vtdibl_dn15 - (assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn15))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8990_e9837 * locals.var_fn97_calc_iq__two_n_phit_dn15)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn97_calc_iq__etas, locals.var_fn97_calc_iq__etas_dn2, locals.var_fn97_calc_iq__etas_dn3, locals.var_fn97_calc_iq__etas_dn4, locals.var_fn97_calc_iq__etas_dn7, locals.var_fn97_calc_iq__etas_dn14, locals.var_fn97_calc_iq__etas_dn15,)
    }
};
        locals.var_fn97_calc_iq__etas = assign8990_e9841;
        locals.var_fn97_calc_iq__etas_dn2 = assign8990_e9841_d_n2;
        locals.var_fn97_calc_iq__etas_dn3 = assign8990_e9841_d_n3;
        locals.var_fn97_calc_iq__etas_dn4 = assign8990_e9841_d_n4;
        locals.var_fn97_calc_iq__etas_dn7 = assign8990_e9841_d_n7;
        locals.var_fn97_calc_iq__etas_dn14 = assign8990_e9841_d_n14;
        locals.var_fn97_calc_iq__etas_dn15 = assign8990_e9841_d_n15;

        let assign9000_e9844: f64 = if locals.var_fn97_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign9000_e9844;

        let (assign9010_e9852, assign9010_e9852_d_n2, assign9010_e9852_d_n3, assign9010_e9852_d_n4, assign9010_e9852_d_n7, assign9010_e9852_d_n14, assign9010_e9852_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard105 != 0.0)) {
        let assign9010_e9850: f64 = (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas);
        (assign9010_e9850, (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn2), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn3), ((locals.var_fn97_calc_iq__qref_dn4 * locals.var_fn97_calc_iq__etas) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn4)), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn7), ((locals.var_fn97_calc_iq__qref_dn14 * locals.var_fn97_calc_iq__etas) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn14)), ((locals.var_fn97_calc_iq__qref_dn15 * locals.var_fn97_calc_iq__etas) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvs, locals.var_fn97_calc_iq__qinvs_dn2, locals.var_fn97_calc_iq__qinvs_dn3, locals.var_fn97_calc_iq__qinvs_dn4, locals.var_fn97_calc_iq__qinvs_dn7, locals.var_fn97_calc_iq__qinvs_dn14, locals.var_fn97_calc_iq__qinvs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs = assign9010_e9852;
        locals.var_fn97_calc_iq__qinvs_dn2 = assign9010_e9852_d_n2;
        locals.var_fn97_calc_iq__qinvs_dn3 = assign9010_e9852_d_n3;
        locals.var_fn97_calc_iq__qinvs_dn4 = assign9010_e9852_d_n4;
        locals.var_fn97_calc_iq__qinvs_dn7 = assign9010_e9852_d_n7;
        locals.var_fn97_calc_iq__qinvs_dn14 = assign9010_e9852_d_n14;
        locals.var_fn97_calc_iq__qinvs_dn15 = assign9010_e9852_d_n15;

        let assign9020_e9855: f64 = (-50.0);
        let assign9020_e9856: f64 = if locals.var_fn97_calc_iq__etas < assign9020_e9855 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign9020_e9856;

        let (assign9030_e9868, assign9030_e9868_d_n2, assign9030_e9868_d_n3, assign9030_e9868_d_n4, assign9030_e9868_d_n7, assign9030_e9868_d_n14, assign9030_e9868_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard105 == 0.0)) && (locals.var_guard106 != 0.0)) {
        let assign9030_e9865: f64 = (locals.var_fn97_calc_iq__etas).exp();
        let assign9030_e9866: f64 = (locals.var_fn97_calc_iq__qref * assign9030_e9865);
        (assign9030_e9866, (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn2)), (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn3)), ((locals.var_fn97_calc_iq__qref_dn4 * assign9030_e9865) + (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn4))), (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn7)), ((locals.var_fn97_calc_iq__qref_dn14 * assign9030_e9865) + (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn14))), ((locals.var_fn97_calc_iq__qref_dn15 * assign9030_e9865) + (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn15))),)
    } else {
        (locals.var_fn97_calc_iq__qinvs, locals.var_fn97_calc_iq__qinvs_dn2, locals.var_fn97_calc_iq__qinvs_dn3, locals.var_fn97_calc_iq__qinvs_dn4, locals.var_fn97_calc_iq__qinvs_dn7, locals.var_fn97_calc_iq__qinvs_dn14, locals.var_fn97_calc_iq__qinvs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs = assign9030_e9868;
        locals.var_fn97_calc_iq__qinvs_dn2 = assign9030_e9868_d_n2;
        locals.var_fn97_calc_iq__qinvs_dn3 = assign9030_e9868_d_n3;
        locals.var_fn97_calc_iq__qinvs_dn4 = assign9030_e9868_d_n4;
        locals.var_fn97_calc_iq__qinvs_dn7 = assign9030_e9868_d_n7;
        locals.var_fn97_calc_iq__qinvs_dn14 = assign9030_e9868_d_n14;
        locals.var_fn97_calc_iq__qinvs_dn15 = assign9030_e9868_d_n15;

        let (assign9040_e9884, assign9040_e9884_d_n2, assign9040_e9884_d_n3, assign9040_e9884_d_n4, assign9040_e9884_d_n7, assign9040_e9884_d_n14, assign9040_e9884_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard105 == 0.0)) && (locals.var_guard106 == 0.0)) {
        let assign9040_e9879: f64 = (locals.var_fn97_calc_iq__etas).exp();
        let assign9040_e9880: f64 = (1.0 + assign9040_e9879);
        let assign9040_e9881: f64 = (assign9040_e9880).ln();
        let assign9040_e9882: f64 = (locals.var_fn97_calc_iq__qref * assign9040_e9881);
        (assign9040_e9882, (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn2) / assign9040_e9880)), (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn3) / assign9040_e9880)), ((locals.var_fn97_calc_iq__qref_dn4 * assign9040_e9881) + (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn4) / assign9040_e9880))), (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn7) / assign9040_e9880)), ((locals.var_fn97_calc_iq__qref_dn14 * assign9040_e9881) + (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn14) / assign9040_e9880))), ((locals.var_fn97_calc_iq__qref_dn15 * assign9040_e9881) + (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn15) / assign9040_e9880))),)
    } else {
        (locals.var_fn97_calc_iq__qinvs, locals.var_fn97_calc_iq__qinvs_dn2, locals.var_fn97_calc_iq__qinvs_dn3, locals.var_fn97_calc_iq__qinvs_dn4, locals.var_fn97_calc_iq__qinvs_dn7, locals.var_fn97_calc_iq__qinvs_dn14, locals.var_fn97_calc_iq__qinvs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs = assign9040_e9884;
        locals.var_fn97_calc_iq__qinvs_dn2 = assign9040_e9884_d_n2;
        locals.var_fn97_calc_iq__qinvs_dn3 = assign9040_e9884_d_n3;
        locals.var_fn97_calc_iq__qinvs_dn4 = assign9040_e9884_d_n4;
        locals.var_fn97_calc_iq__qinvs_dn7 = assign9040_e9884_d_n7;
        locals.var_fn97_calc_iq__qinvs_dn14 = assign9040_e9884_d_n14;
        locals.var_fn97_calc_iq__qinvs_dn15 = assign9040_e9884_d_n15;

        let (assign9050_e9892, assign9050_e9892_d_n2, assign9050_e9892_d_n3, assign9050_e9892_d_n4, assign9050_e9892_d_n7, assign9050_e9892_d_n14, assign9050_e9892_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9050_e9888: f64 = (locals.var_fn97_calc_iq__vgdin - locals.var_fn97_calc_iq__myarg);
        let assign9050_e9890: f64 = (assign9050_e9888 / locals.var_fn97_calc_iq__alpha_phit);
        (assign9050_e9890, ((locals.var_fn97_calc_iq__vgdin_dn2 - locals.var_fn97_calc_iq__myarg_dn2) / locals.var_fn97_calc_iq__alpha_phit), ((-locals.var_fn97_calc_iq__myarg_dn3) / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign9050_e9888 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), ((locals.var_fn97_calc_iq__vgdin_dn7 - locals.var_fn97_calc_iq__myarg_dn7) / locals.var_fn97_calc_iq__alpha_phit), ((locals.var_fn97_calc_iq__vgdin_dn14 - locals.var_fn97_calc_iq__myarg_dn14) / locals.var_fn97_calc_iq__alpha_phit), ((locals.var_fn97_calc_iq__vgdin_dn15 - locals.var_fn97_calc_iq__myarg_dn15) / locals.var_fn97_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9050_e9892;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9050_e9892_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9050_e9892_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9050_e9892_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9050_e9892_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9050_e9892_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9050_e9892_d_n15;

        let assign9060_e9895: f64 = if locals.var_fn97_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign9060_e9895;

        let (assign9070_e9901, assign9070_e9901_d_n2, assign9070_e9901_d_n3, assign9070_e9901_d_n4, assign9070_e9901_d_n7, assign9070_e9901_d_n14, assign9070_e9901_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard107 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd, locals.var_fn97_calc_iq__ffd_dn2, locals.var_fn97_calc_iq__ffd_dn3, locals.var_fn97_calc_iq__ffd_dn4, locals.var_fn97_calc_iq__ffd_dn7, locals.var_fn97_calc_iq__ffd_dn14, locals.var_fn97_calc_iq__ffd_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd = assign9070_e9901;
        locals.var_fn97_calc_iq__ffd_dn2 = assign9070_e9901_d_n2;
        locals.var_fn97_calc_iq__ffd_dn3 = assign9070_e9901_d_n3;
        locals.var_fn97_calc_iq__ffd_dn4 = assign9070_e9901_d_n4;
        locals.var_fn97_calc_iq__ffd_dn7 = assign9070_e9901_d_n7;
        locals.var_fn97_calc_iq__ffd_dn14 = assign9070_e9901_d_n14;
        locals.var_fn97_calc_iq__ffd_dn15 = assign9070_e9901_d_n15;

        let assign9080_e9904: f64 = (-50.0);
        let assign9080_e9905: f64 = if locals.var_fn97_calc_iq__exparg < assign9080_e9904 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign9080_e9905;

        let (assign9090_e9914, assign9090_e9914_d_n2, assign9090_e9914_d_n3, assign9090_e9914_d_n4, assign9090_e9914_d_n7, assign9090_e9914_d_n14, assign9090_e9914_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard107 == 0.0)) && (locals.var_guard108 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd, locals.var_fn97_calc_iq__ffd_dn2, locals.var_fn97_calc_iq__ffd_dn3, locals.var_fn97_calc_iq__ffd_dn4, locals.var_fn97_calc_iq__ffd_dn7, locals.var_fn97_calc_iq__ffd_dn14, locals.var_fn97_calc_iq__ffd_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd = assign9090_e9914;
        locals.var_fn97_calc_iq__ffd_dn2 = assign9090_e9914_d_n2;
        locals.var_fn97_calc_iq__ffd_dn3 = assign9090_e9914_d_n3;
        locals.var_fn97_calc_iq__ffd_dn4 = assign9090_e9914_d_n4;
        locals.var_fn97_calc_iq__ffd_dn7 = assign9090_e9914_d_n7;
        locals.var_fn97_calc_iq__ffd_dn14 = assign9090_e9914_d_n14;
        locals.var_fn97_calc_iq__ffd_dn15 = assign9090_e9914_d_n15;

        let (assign9100_e9929, assign9100_e9929_d_n2, assign9100_e9929_d_n3, assign9100_e9929_d_n4, assign9100_e9929_d_n7, assign9100_e9929_d_n14, assign9100_e9929_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard107 == 0.0)) && (locals.var_guard108 == 0.0)) {
        let assign9100_e9925: f64 = (locals.var_fn97_calc_iq__exparg).exp();
        let assign9100_e9926: f64 = (1.0 + assign9100_e9925);
        let assign9100_e9927: f64 = (1.0 / assign9100_e9926);
        (assign9100_e9927, (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn2) / (assign9100_e9926 * assign9100_e9926))), (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn3) / (assign9100_e9926 * assign9100_e9926))), (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn4) / (assign9100_e9926 * assign9100_e9926))), (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn7) / (assign9100_e9926 * assign9100_e9926))), (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn14) / (assign9100_e9926 * assign9100_e9926))), (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn15) / (assign9100_e9926 * assign9100_e9926))),)
    } else {
        (locals.var_fn97_calc_iq__ffd, locals.var_fn97_calc_iq__ffd_dn2, locals.var_fn97_calc_iq__ffd_dn3, locals.var_fn97_calc_iq__ffd_dn4, locals.var_fn97_calc_iq__ffd_dn7, locals.var_fn97_calc_iq__ffd_dn14, locals.var_fn97_calc_iq__ffd_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd = assign9100_e9929;
        locals.var_fn97_calc_iq__ffd_dn2 = assign9100_e9929_d_n2;
        locals.var_fn97_calc_iq__ffd_dn3 = assign9100_e9929_d_n3;
        locals.var_fn97_calc_iq__ffd_dn4 = assign9100_e9929_d_n4;
        locals.var_fn97_calc_iq__ffd_dn7 = assign9100_e9929_d_n7;
        locals.var_fn97_calc_iq__ffd_dn14 = assign9100_e9929_d_n14;
        locals.var_fn97_calc_iq__ffd_dn15 = assign9100_e9929_d_n15;

        let (assign9110_e9947, assign9110_e9947_d_n2, assign9110_e9947_d_n3, assign9110_e9947_d_n4, assign9110_e9947_d_n7, assign9110_e9947_d_n14, assign9110_e9947_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9110_e9933: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vdx);
        let assign9110_e9937: f64 = (p.p51 * 0.1);
        let assign9110_e9939: f64 = (assign9110_e9937 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9110_e9941: f64 = (assign9110_e9939 * locals.var_fn97_calc_iq__ffd);
        let assign9110_e9942: f64 = (locals.var_fn97_calc_iq__vtdibl - assign9110_e9941);
        let assign9110_e9943: f64 = (assign9110_e9933 - assign9110_e9942);
        let assign9110_e9945: f64 = (assign9110_e9943 / locals.var_fn97_calc_iq__two_n_phit);
        (assign9110_e9945, (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vdx_dn2) - (-(assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn2))) / locals.var_fn97_calc_iq__two_n_phit), (((-locals.var_fn97_calc_iq__vdx_dn3) - (-(assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn3))) / locals.var_fn97_calc_iq__two_n_phit), (((((-locals.var_fn97_calc_iq__vdx_dn4) - (locals.var_fn97_calc_iq__vtdibl_dn4 - (((assign9110_e9937 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ffd) + (assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn4)))) * locals.var_fn97_calc_iq__two_n_phit) - (assign9110_e9943 * locals.var_fn97_calc_iq__two_n_phit_dn4)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vdx_dn7) - (-(assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn7))) / locals.var_fn97_calc_iq__two_n_phit), (((((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vdx_dn14) - (locals.var_fn97_calc_iq__vtdibl_dn14 - (assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn14))) * locals.var_fn97_calc_iq__two_n_phit) - (assign9110_e9943 * locals.var_fn97_calc_iq__two_n_phit_dn14)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), (((((-locals.var_fn97_calc_iq__vdx_dn15) - (locals.var_fn97_calc_iq__vtdibl_dn15 - (assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn15))) * locals.var_fn97_calc_iq__two_n_phit) - (assign9110_e9943 * locals.var_fn97_calc_iq__two_n_phit_dn15)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn97_calc_iq__etad, locals.var_fn97_calc_iq__etad_dn2, locals.var_fn97_calc_iq__etad_dn3, locals.var_fn97_calc_iq__etad_dn4, locals.var_fn97_calc_iq__etad_dn7, locals.var_fn97_calc_iq__etad_dn14, locals.var_fn97_calc_iq__etad_dn15,)
    }
};
        locals.var_fn97_calc_iq__etad = assign9110_e9947;
        locals.var_fn97_calc_iq__etad_dn2 = assign9110_e9947_d_n2;
        locals.var_fn97_calc_iq__etad_dn3 = assign9110_e9947_d_n3;
        locals.var_fn97_calc_iq__etad_dn4 = assign9110_e9947_d_n4;
        locals.var_fn97_calc_iq__etad_dn7 = assign9110_e9947_d_n7;
        locals.var_fn97_calc_iq__etad_dn14 = assign9110_e9947_d_n14;
        locals.var_fn97_calc_iq__etad_dn15 = assign9110_e9947_d_n15;

        let assign9120_e9950: f64 = if locals.var_fn97_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign9120_e9950;

        let (assign9130_e9958, assign9130_e9958_d_n2, assign9130_e9958_d_n3, assign9130_e9958_d_n4, assign9130_e9958_d_n7, assign9130_e9958_d_n14, assign9130_e9958_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard109 != 0.0)) {
        let assign9130_e9956: f64 = (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad);
        (assign9130_e9956, (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn2), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn3), ((locals.var_fn97_calc_iq__qref_dn4 * locals.var_fn97_calc_iq__etad) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn4)), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn7), ((locals.var_fn97_calc_iq__qref_dn14 * locals.var_fn97_calc_iq__etad) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn14)), ((locals.var_fn97_calc_iq__qref_dn15 * locals.var_fn97_calc_iq__etad) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvd, locals.var_fn97_calc_iq__qinvd_dn2, locals.var_fn97_calc_iq__qinvd_dn3, locals.var_fn97_calc_iq__qinvd_dn4, locals.var_fn97_calc_iq__qinvd_dn7, locals.var_fn97_calc_iq__qinvd_dn14, locals.var_fn97_calc_iq__qinvd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd = assign9130_e9958;
        locals.var_fn97_calc_iq__qinvd_dn2 = assign9130_e9958_d_n2;
        locals.var_fn97_calc_iq__qinvd_dn3 = assign9130_e9958_d_n3;
        locals.var_fn97_calc_iq__qinvd_dn4 = assign9130_e9958_d_n4;
        locals.var_fn97_calc_iq__qinvd_dn7 = assign9130_e9958_d_n7;
        locals.var_fn97_calc_iq__qinvd_dn14 = assign9130_e9958_d_n14;
        locals.var_fn97_calc_iq__qinvd_dn15 = assign9130_e9958_d_n15;

        let assign9140_e9961: f64 = (-50.0);
        let assign9140_e9962: f64 = if locals.var_fn97_calc_iq__etad < assign9140_e9961 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign9140_e9962;

        let (assign9150_e9974, assign9150_e9974_d_n2, assign9150_e9974_d_n3, assign9150_e9974_d_n4, assign9150_e9974_d_n7, assign9150_e9974_d_n14, assign9150_e9974_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard109 == 0.0)) && (locals.var_guard110 != 0.0)) {
        let assign9150_e9971: f64 = (locals.var_fn97_calc_iq__etad).exp();
        let assign9150_e9972: f64 = (locals.var_fn97_calc_iq__qref * assign9150_e9971);
        (assign9150_e9972, (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn2)), (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn3)), ((locals.var_fn97_calc_iq__qref_dn4 * assign9150_e9971) + (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn4))), (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn7)), ((locals.var_fn97_calc_iq__qref_dn14 * assign9150_e9971) + (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn14))), ((locals.var_fn97_calc_iq__qref_dn15 * assign9150_e9971) + (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn15))),)
    } else {
        (locals.var_fn97_calc_iq__qinvd, locals.var_fn97_calc_iq__qinvd_dn2, locals.var_fn97_calc_iq__qinvd_dn3, locals.var_fn97_calc_iq__qinvd_dn4, locals.var_fn97_calc_iq__qinvd_dn7, locals.var_fn97_calc_iq__qinvd_dn14, locals.var_fn97_calc_iq__qinvd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd = assign9150_e9974;
        locals.var_fn97_calc_iq__qinvd_dn2 = assign9150_e9974_d_n2;
        locals.var_fn97_calc_iq__qinvd_dn3 = assign9150_e9974_d_n3;
        locals.var_fn97_calc_iq__qinvd_dn4 = assign9150_e9974_d_n4;
        locals.var_fn97_calc_iq__qinvd_dn7 = assign9150_e9974_d_n7;
        locals.var_fn97_calc_iq__qinvd_dn14 = assign9150_e9974_d_n14;
        locals.var_fn97_calc_iq__qinvd_dn15 = assign9150_e9974_d_n15;

        let (assign9160_e9990, assign9160_e9990_d_n2, assign9160_e9990_d_n3, assign9160_e9990_d_n4, assign9160_e9990_d_n7, assign9160_e9990_d_n14, assign9160_e9990_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard109 == 0.0)) && (locals.var_guard110 == 0.0)) {
        let assign9160_e9985: f64 = (locals.var_fn97_calc_iq__etad).exp();
        let assign9160_e9986: f64 = (1.0 + assign9160_e9985);
        let assign9160_e9987: f64 = (assign9160_e9986).ln();
        let assign9160_e9988: f64 = (locals.var_fn97_calc_iq__qref * assign9160_e9987);
        (assign9160_e9988, (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn2) / assign9160_e9986)), (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn3) / assign9160_e9986)), ((locals.var_fn97_calc_iq__qref_dn4 * assign9160_e9987) + (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn4) / assign9160_e9986))), (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn7) / assign9160_e9986)), ((locals.var_fn97_calc_iq__qref_dn14 * assign9160_e9987) + (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn14) / assign9160_e9986))), ((locals.var_fn97_calc_iq__qref_dn15 * assign9160_e9987) + (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn15) / assign9160_e9986))),)
    } else {
        (locals.var_fn97_calc_iq__qinvd, locals.var_fn97_calc_iq__qinvd_dn2, locals.var_fn97_calc_iq__qinvd_dn3, locals.var_fn97_calc_iq__qinvd_dn4, locals.var_fn97_calc_iq__qinvd_dn7, locals.var_fn97_calc_iq__qinvd_dn14, locals.var_fn97_calc_iq__qinvd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd = assign9160_e9990;
        locals.var_fn97_calc_iq__qinvd_dn2 = assign9160_e9990_d_n2;
        locals.var_fn97_calc_iq__qinvd_dn3 = assign9160_e9990_d_n3;
        locals.var_fn97_calc_iq__qinvd_dn4 = assign9160_e9990_d_n4;
        locals.var_fn97_calc_iq__qinvd_dn7 = assign9160_e9990_d_n7;
        locals.var_fn97_calc_iq__qinvd_dn14 = assign9160_e9990_d_n14;
        locals.var_fn97_calc_iq__qinvd_dn15 = assign9160_e9990_d_n15;

        let (assign9170_e9998, assign9170_e9998_d_n2, assign9170_e9998_d_n3, assign9170_e9998_d_n4, assign9170_e9998_d_n7, assign9170_e9998_d_n14, assign9170_e9998_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9170_e9994: f64 = (locals.var_fn97_calc_iq__qinvs - locals.var_fn97_calc_iq__qinvd);
        let assign9170_e9996: f64 = (assign9170_e9994 / locals.var_fn97_calc_iq__cgin);
        (assign9170_e9996, ((locals.var_fn97_calc_iq__qinvs_dn2 - locals.var_fn97_calc_iq__qinvd_dn2) / locals.var_fn97_calc_iq__cgin), ((locals.var_fn97_calc_iq__qinvs_dn3 - locals.var_fn97_calc_iq__qinvd_dn3) / locals.var_fn97_calc_iq__cgin), ((((locals.var_fn97_calc_iq__qinvs_dn4 - locals.var_fn97_calc_iq__qinvd_dn4) * locals.var_fn97_calc_iq__cgin) - (assign9170_e9994 * locals.var_fn97_calc_iq__cgin_dn4)) / (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__cgin)), ((locals.var_fn97_calc_iq__qinvs_dn7 - locals.var_fn97_calc_iq__qinvd_dn7) / locals.var_fn97_calc_iq__cgin), ((locals.var_fn97_calc_iq__qinvs_dn14 - locals.var_fn97_calc_iq__qinvd_dn14) / locals.var_fn97_calc_iq__cgin), ((locals.var_fn97_calc_iq__qinvs_dn15 - locals.var_fn97_calc_iq__qinvd_dn15) / locals.var_fn97_calc_iq__cgin),)
    } else {
        (locals.var_fn97_calc_iq__vdsc, locals.var_fn97_calc_iq__vdsc_dn2, locals.var_fn97_calc_iq__vdsc_dn3, locals.var_fn97_calc_iq__vdsc_dn4, locals.var_fn97_calc_iq__vdsc_dn7, locals.var_fn97_calc_iq__vdsc_dn14, locals.var_fn97_calc_iq__vdsc_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsc = assign9170_e9998;
        locals.var_fn97_calc_iq__vdsc_dn2 = assign9170_e9998_d_n2;
        locals.var_fn97_calc_iq__vdsc_dn3 = assign9170_e9998_d_n3;
        locals.var_fn97_calc_iq__vdsc_dn4 = assign9170_e9998_d_n4;
        locals.var_fn97_calc_iq__vdsc_dn7 = assign9170_e9998_d_n7;
        locals.var_fn97_calc_iq__vdsc_dn14 = assign9170_e9998_d_n14;
        locals.var_fn97_calc_iq__vdsc_dn15 = assign9170_e9998_d_n15;

        let (assign9180_e10004, assign9180_e10004_d_n2, assign9180_e10004_d_n3, assign9180_e10004_d_n4, assign9180_e10004_d_n7, assign9180_e10004_d_n14, assign9180_e10004_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9180_e10002: f64 = (locals.var_fn97_calc_iq__vdsc / locals.var_fn97_calc_iq__vdsat);
        (assign9180_e10002, (((locals.var_fn97_calc_iq__vdsc_dn2 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn2)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)), (((locals.var_fn97_calc_iq__vdsc_dn3 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn3)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)), (((locals.var_fn97_calc_iq__vdsc_dn4 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn4)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)), (((locals.var_fn97_calc_iq__vdsc_dn7 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn7)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)), (((locals.var_fn97_calc_iq__vdsc_dn14 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn14)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)), (((locals.var_fn97_calc_iq__vdsc_dn15 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn15)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)),)
    } else {
        (locals.var_fn97_calc_iq__myarg, locals.var_fn97_calc_iq__myarg_dn2, locals.var_fn97_calc_iq__myarg_dn3, locals.var_fn97_calc_iq__myarg_dn4, locals.var_fn97_calc_iq__myarg_dn7, locals.var_fn97_calc_iq__myarg_dn14, locals.var_fn97_calc_iq__myarg_dn15,)
    }
};
        locals.var_fn97_calc_iq__myarg = assign9180_e10004;
        locals.var_fn97_calc_iq__myarg_dn2 = assign9180_e10004_d_n2;
        locals.var_fn97_calc_iq__myarg_dn3 = assign9180_e10004_d_n3;
        locals.var_fn97_calc_iq__myarg_dn4 = assign9180_e10004_d_n4;
        locals.var_fn97_calc_iq__myarg_dn7 = assign9180_e10004_d_n7;
        locals.var_fn97_calc_iq__myarg_dn14 = assign9180_e10004_d_n14;
        locals.var_fn97_calc_iq__myarg_dn15 = assign9180_e10004_d_n15;

        let (assign9220_e10073, assign9220_e10073_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9220_e10070: f64 = (2.302585092994046 * locals.var_fn97_calc_iq__phitin);
        let assign9220_e10071: f64 = (locals.var_fn97_calc_iq__ss / assign9220_e10070);
        (assign9220_e10071, (-((locals.var_fn97_calc_iq__ss * (2.302585092994046 * locals.var_fn97_calc_iq__phitin_dn4)) / (assign9220_e10070 * assign9220_e10070))),)
    } else {
        (locals.var_fn97_calc_iq__n0, locals.var_fn97_calc_iq__n0_dn4,)
    }
};
        locals.var_fn97_calc_iq__n0 = assign9220_e10073;
        locals.var_fn97_calc_iq__n0_dn4 = assign9220_e10073_d_n4;

        let (assign9230_e10081, assign9230_e10081_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9230_e10077: f64 = (2.0 * locals.var_fn97_calc_iq__n0);
        let assign9230_e10079: f64 = (assign9230_e10077 * locals.var_fn97_calc_iq__phitin);
        (assign9230_e10079, (((2.0 * locals.var_fn97_calc_iq__n0_dn4) * locals.var_fn97_calc_iq__phitin) + (assign9230_e10077 * locals.var_fn97_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn97_calc_iq__two_n_phit0, locals.var_fn97_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn97_calc_iq__two_n_phit0 = assign9230_e10081;
        locals.var_fn97_calc_iq__two_n_phit0_dn4 = assign9230_e10081_d_n4;

        let (assign9240_e10087, assign9240_e10087_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9240_e10085: f64 = (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit0);
        (assign9240_e10085, ((locals.var_fn97_calc_iq__cgin_dn4 * locals.var_fn97_calc_iq__two_n_phit0) + (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn97_calc_iq__qref0, locals.var_fn97_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn97_calc_iq__qref0 = assign9240_e10087;
        locals.var_fn97_calc_iq__qref0_dn4 = assign9240_e10087_d_n4;

        let (assign9250_e10097, assign9250_e10097_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9250_e10092: f64 = (p.p51 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9250_e10094: f64 = (assign9250_e10092 / 2.0);
        let assign9250_e10095: f64 = (locals.var_fn97_calc_iq__vtof - assign9250_e10094);
        (assign9250_e10095, (locals.var_fn97_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn97_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn97_calc_iq__myarg0, locals.var_fn97_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn97_calc_iq__myarg0 = assign9250_e10097;
        locals.var_fn97_calc_iq__myarg0_dn4 = assign9250_e10097_d_n4;

    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9260_e10148, assign9260_e10148_d_n2, assign9260_e10148_d_n4, assign9260_e10148_d_n7, assign9260_e10148_d_n14, assign9260_e10148_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign9260_e10142, assign9260_e10142_d_n2, assign9260_e10142_d_n7, assign9260_e10142_d_n14, assign9260_e10142_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign9260_e10106: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                let assign9260_e10109: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign9260_e10112: f64 = (0.001 / p.p53);
                let assign9260_e10115: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign9260_e10116: f64 = (assign9260_e10112 * assign9260_e10115);
                let assign9260_e10117: f64 = (assign9260_e10116).tanh();
                let assign9260_e10118: f64 = (assign9260_e10109 * assign9260_e10117);
                let assign9260_e10119: f64 = (assign9260_e10106 + assign9260_e10118);
                let assign9260_e10120: f64 = (0.5 * assign9260_e10119);
                (assign9260_e10120, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign9260_e10117) + (assign9260_e10109 * ((assign9260_e10112 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2)) / ((assign9260_e10116).cosh() * (assign9260_e10116).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign9260_e10117) + (assign9260_e10109 * ((assign9260_e10112 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7)) / ((assign9260_e10116).cosh() * (assign9260_e10116).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + (((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign9260_e10117) + (assign9260_e10109 * ((assign9260_e10112 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14)) / ((assign9260_e10116).cosh() * (assign9260_e10116).cosh())))))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + (((-locals.var_fn97_calc_iq__vgdin_dn15) * assign9260_e10117) + (assign9260_e10109 * ((assign9260_e10112 * (-locals.var_fn97_calc_iq__vgdin_dn15)) / ((assign9260_e10116).cosh() * (assign9260_e10116).cosh())))))),)
            } else {
                let (assign9260_e10141, assign9260_e10141_d_n2, assign9260_e10141_d_n7, assign9260_e10141_d_n14, assign9260_e10141_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign9260_e10127: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                        let assign9260_e10130: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign9260_e10133: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign9260_e10134: f64 = (assign9260_e10130 * assign9260_e10133);
                        let assign9260_e10136: f64 = (assign9260_e10134 + p.p53);
                        let assign9260_e10137: f64 = (assign9260_e10136).sqrt();
                        let assign9260_e10138: f64 = (assign9260_e10127 + assign9260_e10137);
                        let assign9260_e10139: f64 = (0.5 * assign9260_e10138);
                        (assign9260_e10139, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + ((((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign9260_e10133) + (assign9260_e10130 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2))) / (2.0 * assign9260_e10137)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + ((((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign9260_e10133) + (assign9260_e10130 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7))) / (2.0 * assign9260_e10137)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + ((((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign9260_e10133) + (assign9260_e10130 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14))) / (2.0 * assign9260_e10137)))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + ((((-locals.var_fn97_calc_iq__vgdin_dn15) * assign9260_e10133) + (assign9260_e10130 * (-locals.var_fn97_calc_iq__vgdin_dn15))) / (2.0 * assign9260_e10137)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign9260_e10141, assign9260_e10141_d_n2, assign9260_e10141_d_n7, assign9260_e10141_d_n14, assign9260_e10141_d_n15,)
            }
        };
        let assign9260_e10144: f64 = (assign9260_e10142 - locals.var_fn97_calc_iq__myarg0);
        let assign9260_e10146: f64 = (assign9260_e10144 / locals.var_fn97_calc_iq__alpha_phit);
        (assign9260_e10146, (assign9260_e10142_d_n2 / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg0_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign9260_e10144 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), (assign9260_e10142_d_n7 / locals.var_fn97_calc_iq__alpha_phit), (assign9260_e10142_d_n14 / locals.var_fn97_calc_iq__alpha_phit), (assign9260_e10142_d_n15 / locals.var_fn97_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn97_calc_iq__exparg0, locals.var_fn97_calc_iq__exparg0_dn2, locals.var_fn97_calc_iq__exparg0_dn4, locals.var_fn97_calc_iq__exparg0_dn7, locals.var_fn97_calc_iq__exparg0_dn14, locals.var_fn97_calc_iq__exparg0_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg0 = assign9260_e10148;
        locals.var_fn97_calc_iq__exparg0_dn2 = assign9260_e10148_d_n2;
        locals.var_fn97_calc_iq__exparg0_dn4 = assign9260_e10148_d_n4;
        locals.var_fn97_calc_iq__exparg0_dn7 = assign9260_e10148_d_n7;
        locals.var_fn97_calc_iq__exparg0_dn14 = assign9260_e10148_d_n14;
        locals.var_fn97_calc_iq__exparg0_dn15 = assign9260_e10148_d_n15;

        let assign9270_e10151: f64 = if locals.var_fn97_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign9270_e10151;

        let (assign9280_e10157, assign9280_e10157_d_n2, assign9280_e10157_d_n4, assign9280_e10157_d_n7, assign9280_e10157_d_n14, assign9280_e10157_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard111 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff0, locals.var_fn97_calc_iq__ff0_dn2, locals.var_fn97_calc_iq__ff0_dn4, locals.var_fn97_calc_iq__ff0_dn7, locals.var_fn97_calc_iq__ff0_dn14, locals.var_fn97_calc_iq__ff0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff0 = assign9280_e10157;
        locals.var_fn97_calc_iq__ff0_dn2 = assign9280_e10157_d_n2;
        locals.var_fn97_calc_iq__ff0_dn4 = assign9280_e10157_d_n4;
        locals.var_fn97_calc_iq__ff0_dn7 = assign9280_e10157_d_n7;
        locals.var_fn97_calc_iq__ff0_dn14 = assign9280_e10157_d_n14;
        locals.var_fn97_calc_iq__ff0_dn15 = assign9280_e10157_d_n15;

        let assign9290_e10160: f64 = (-50.0);
        let assign9290_e10161: f64 = if locals.var_fn97_calc_iq__exparg0 < assign9290_e10160 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign9290_e10161;

        let (assign9300_e10170, assign9300_e10170_d_n2, assign9300_e10170_d_n4, assign9300_e10170_d_n7, assign9300_e10170_d_n14, assign9300_e10170_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard111 == 0.0)) && (locals.var_guard112 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff0, locals.var_fn97_calc_iq__ff0_dn2, locals.var_fn97_calc_iq__ff0_dn4, locals.var_fn97_calc_iq__ff0_dn7, locals.var_fn97_calc_iq__ff0_dn14, locals.var_fn97_calc_iq__ff0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff0 = assign9300_e10170;
        locals.var_fn97_calc_iq__ff0_dn2 = assign9300_e10170_d_n2;
        locals.var_fn97_calc_iq__ff0_dn4 = assign9300_e10170_d_n4;
        locals.var_fn97_calc_iq__ff0_dn7 = assign9300_e10170_d_n7;
        locals.var_fn97_calc_iq__ff0_dn14 = assign9300_e10170_d_n14;
        locals.var_fn97_calc_iq__ff0_dn15 = assign9300_e10170_d_n15;

        let (assign9310_e10185, assign9310_e10185_d_n2, assign9310_e10185_d_n4, assign9310_e10185_d_n7, assign9310_e10185_d_n14, assign9310_e10185_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard111 == 0.0)) && (locals.var_guard112 == 0.0)) {
        let assign9310_e10181: f64 = (locals.var_fn97_calc_iq__exparg0).exp();
        let assign9310_e10182: f64 = (1.0 + assign9310_e10181);
        let assign9310_e10183: f64 = (1.0 / assign9310_e10182);
        (assign9310_e10183, (-((assign9310_e10181 * locals.var_fn97_calc_iq__exparg0_dn2) / (assign9310_e10182 * assign9310_e10182))), (-((assign9310_e10181 * locals.var_fn97_calc_iq__exparg0_dn4) / (assign9310_e10182 * assign9310_e10182))), (-((assign9310_e10181 * locals.var_fn97_calc_iq__exparg0_dn7) / (assign9310_e10182 * assign9310_e10182))), (-((assign9310_e10181 * locals.var_fn97_calc_iq__exparg0_dn14) / (assign9310_e10182 * assign9310_e10182))), (-((assign9310_e10181 * locals.var_fn97_calc_iq__exparg0_dn15) / (assign9310_e10182 * assign9310_e10182))),)
    } else {
        (locals.var_fn97_calc_iq__ff0, locals.var_fn97_calc_iq__ff0_dn2, locals.var_fn97_calc_iq__ff0_dn4, locals.var_fn97_calc_iq__ff0_dn7, locals.var_fn97_calc_iq__ff0_dn14, locals.var_fn97_calc_iq__ff0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff0 = assign9310_e10185;
        locals.var_fn97_calc_iq__ff0_dn2 = assign9310_e10185_d_n2;
        locals.var_fn97_calc_iq__ff0_dn4 = assign9310_e10185_d_n4;
        locals.var_fn97_calc_iq__ff0_dn7 = assign9310_e10185_d_n7;
        locals.var_fn97_calc_iq__ff0_dn14 = assign9310_e10185_d_n14;
        locals.var_fn97_calc_iq__ff0_dn15 = assign9310_e10185_d_n15;

        let (assign9320_e10244, assign9320_e10244_d_n2, assign9320_e10244_d_n4, assign9320_e10244_d_n7, assign9320_e10244_d_n14, assign9320_e10244_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign9320_e10230, assign9320_e10230_d_n2, assign9320_e10230_d_n7, assign9320_e10230_d_n14, assign9320_e10230_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign9320_e10194: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                let assign9320_e10197: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign9320_e10200: f64 = (0.001 / p.p53);
                let assign9320_e10203: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign9320_e10204: f64 = (assign9320_e10200 * assign9320_e10203);
                let assign9320_e10205: f64 = (assign9320_e10204).tanh();
                let assign9320_e10206: f64 = (assign9320_e10197 * assign9320_e10205);
                let assign9320_e10207: f64 = (assign9320_e10194 + assign9320_e10206);
                let assign9320_e10208: f64 = (0.5 * assign9320_e10207);
                (assign9320_e10208, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign9320_e10205) + (assign9320_e10197 * ((assign9320_e10200 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2)) / ((assign9320_e10204).cosh() * (assign9320_e10204).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign9320_e10205) + (assign9320_e10197 * ((assign9320_e10200 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7)) / ((assign9320_e10204).cosh() * (assign9320_e10204).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + (((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign9320_e10205) + (assign9320_e10197 * ((assign9320_e10200 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14)) / ((assign9320_e10204).cosh() * (assign9320_e10204).cosh())))))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + (((-locals.var_fn97_calc_iq__vgdin_dn15) * assign9320_e10205) + (assign9320_e10197 * ((assign9320_e10200 * (-locals.var_fn97_calc_iq__vgdin_dn15)) / ((assign9320_e10204).cosh() * (assign9320_e10204).cosh())))))),)
            } else {
                let (assign9320_e10229, assign9320_e10229_d_n2, assign9320_e10229_d_n7, assign9320_e10229_d_n14, assign9320_e10229_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign9320_e10215: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                        let assign9320_e10218: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign9320_e10221: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign9320_e10222: f64 = (assign9320_e10218 * assign9320_e10221);
                        let assign9320_e10224: f64 = (assign9320_e10222 + p.p53);
                        let assign9320_e10225: f64 = (assign9320_e10224).sqrt();
                        let assign9320_e10226: f64 = (assign9320_e10215 + assign9320_e10225);
                        let assign9320_e10227: f64 = (0.5 * assign9320_e10226);
                        (assign9320_e10227, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + ((((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign9320_e10221) + (assign9320_e10218 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2))) / (2.0 * assign9320_e10225)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + ((((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign9320_e10221) + (assign9320_e10218 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7))) / (2.0 * assign9320_e10225)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + ((((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign9320_e10221) + (assign9320_e10218 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14))) / (2.0 * assign9320_e10225)))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + ((((-locals.var_fn97_calc_iq__vgdin_dn15) * assign9320_e10221) + (assign9320_e10218 * (-locals.var_fn97_calc_iq__vgdin_dn15))) / (2.0 * assign9320_e10225)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign9320_e10229, assign9320_e10229_d_n2, assign9320_e10229_d_n7, assign9320_e10229_d_n14, assign9320_e10229_d_n15,)
            }
        };
        let assign9320_e10234: f64 = (p.p51 * 0.1);
        let assign9320_e10236: f64 = (assign9320_e10234 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9320_e10238: f64 = (assign9320_e10236 * locals.var_fn97_calc_iq__ff0);
        let assign9320_e10239: f64 = (locals.var_fn97_calc_iq__vtof - assign9320_e10238);
        let assign9320_e10240: f64 = (assign9320_e10230 - assign9320_e10239);
        let assign9320_e10242: f64 = (assign9320_e10240 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign9320_e10242, ((assign9320_e10230_d_n2 - (-(assign9320_e10236 * locals.var_fn97_calc_iq__ff0_dn2))) / locals.var_fn97_calc_iq__two_n_phit0), ((((-(locals.var_fn97_calc_iq__vtof_dn4 - (((assign9320_e10234 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ff0) + (assign9320_e10236 * locals.var_fn97_calc_iq__ff0_dn4)))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign9320_e10240 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), ((assign9320_e10230_d_n7 - (-(assign9320_e10236 * locals.var_fn97_calc_iq__ff0_dn7))) / locals.var_fn97_calc_iq__two_n_phit0), ((assign9320_e10230_d_n14 - (-(assign9320_e10236 * locals.var_fn97_calc_iq__ff0_dn14))) / locals.var_fn97_calc_iq__two_n_phit0), ((assign9320_e10230_d_n15 - (-(assign9320_e10236 * locals.var_fn97_calc_iq__ff0_dn15))) / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__eta0, locals.var_fn97_calc_iq__eta0_dn2, locals.var_fn97_calc_iq__eta0_dn4, locals.var_fn97_calc_iq__eta0_dn7, locals.var_fn97_calc_iq__eta0_dn14, locals.var_fn97_calc_iq__eta0_dn15,)
    }
};
        locals.var_fn97_calc_iq__eta0 = assign9320_e10244;
        locals.var_fn97_calc_iq__eta0_dn2 = assign9320_e10244_d_n2;
        locals.var_fn97_calc_iq__eta0_dn4 = assign9320_e10244_d_n4;
        locals.var_fn97_calc_iq__eta0_dn7 = assign9320_e10244_d_n7;
        locals.var_fn97_calc_iq__eta0_dn14 = assign9320_e10244_d_n14;
        locals.var_fn97_calc_iq__eta0_dn15 = assign9320_e10244_d_n15;

        let assign9330_e10247: f64 = if locals.var_fn97_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign9330_e10247;

        let (assign9340_e10255, assign9340_e10255_d_n2, assign9340_e10255_d_n4, assign9340_e10255_d_n7, assign9340_e10255_d_n14, assign9340_e10255_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign9340_e10253: f64 = (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0);
        (assign9340_e10253, (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0_dn2), ((locals.var_fn97_calc_iq__qref0_dn4 * locals.var_fn97_calc_iq__eta0) + (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0_dn4)), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0_dn7), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0_dn14), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0_dn15),)
    } else {
        (locals.var_fn97_calc_iq__qinvv0, locals.var_fn97_calc_iq__qinvv0_dn2, locals.var_fn97_calc_iq__qinvv0_dn4, locals.var_fn97_calc_iq__qinvv0_dn7, locals.var_fn97_calc_iq__qinvv0_dn14, locals.var_fn97_calc_iq__qinvv0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv0 = assign9340_e10255;
        locals.var_fn97_calc_iq__qinvv0_dn2 = assign9340_e10255_d_n2;
        locals.var_fn97_calc_iq__qinvv0_dn4 = assign9340_e10255_d_n4;
        locals.var_fn97_calc_iq__qinvv0_dn7 = assign9340_e10255_d_n7;
        locals.var_fn97_calc_iq__qinvv0_dn14 = assign9340_e10255_d_n14;
        locals.var_fn97_calc_iq__qinvv0_dn15 = assign9340_e10255_d_n15;

        let assign9350_e10258: f64 = (-50.0);
        let assign9350_e10259: f64 = if locals.var_fn97_calc_iq__eta0 < assign9350_e10258 { 1.0 } else { 0.0 };
        locals.var_guard114 = assign9350_e10259;

        let (assign9360_e10271, assign9360_e10271_d_n2, assign9360_e10271_d_n4, assign9360_e10271_d_n7, assign9360_e10271_d_n14, assign9360_e10271_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard113 == 0.0)) && (locals.var_guard114 != 0.0)) {
        let assign9360_e10268: f64 = (locals.var_fn97_calc_iq__eta0).exp();
        let assign9360_e10269: f64 = (locals.var_fn97_calc_iq__qref0 * assign9360_e10268);
        (assign9360_e10269, (locals.var_fn97_calc_iq__qref0 * (assign9360_e10268 * locals.var_fn97_calc_iq__eta0_dn2)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9360_e10268) + (locals.var_fn97_calc_iq__qref0 * (assign9360_e10268 * locals.var_fn97_calc_iq__eta0_dn4))), (locals.var_fn97_calc_iq__qref0 * (assign9360_e10268 * locals.var_fn97_calc_iq__eta0_dn7)), (locals.var_fn97_calc_iq__qref0 * (assign9360_e10268 * locals.var_fn97_calc_iq__eta0_dn14)), (locals.var_fn97_calc_iq__qref0 * (assign9360_e10268 * locals.var_fn97_calc_iq__eta0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvv0, locals.var_fn97_calc_iq__qinvv0_dn2, locals.var_fn97_calc_iq__qinvv0_dn4, locals.var_fn97_calc_iq__qinvv0_dn7, locals.var_fn97_calc_iq__qinvv0_dn14, locals.var_fn97_calc_iq__qinvv0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv0 = assign9360_e10271;
        locals.var_fn97_calc_iq__qinvv0_dn2 = assign9360_e10271_d_n2;
        locals.var_fn97_calc_iq__qinvv0_dn4 = assign9360_e10271_d_n4;
        locals.var_fn97_calc_iq__qinvv0_dn7 = assign9360_e10271_d_n7;
        locals.var_fn97_calc_iq__qinvv0_dn14 = assign9360_e10271_d_n14;
        locals.var_fn97_calc_iq__qinvv0_dn15 = assign9360_e10271_d_n15;

        let (assign9370_e10287, assign9370_e10287_d_n2, assign9370_e10287_d_n4, assign9370_e10287_d_n7, assign9370_e10287_d_n14, assign9370_e10287_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard113 == 0.0)) && (locals.var_guard114 == 0.0)) {
        let assign9370_e10282: f64 = (locals.var_fn97_calc_iq__eta0).exp();
        let assign9370_e10283: f64 = (1.0 + assign9370_e10282);
        let assign9370_e10284: f64 = (assign9370_e10283).ln();
        let assign9370_e10285: f64 = (locals.var_fn97_calc_iq__qref0 * assign9370_e10284);
        (assign9370_e10285, (locals.var_fn97_calc_iq__qref0 * ((assign9370_e10282 * locals.var_fn97_calc_iq__eta0_dn2) / assign9370_e10283)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9370_e10284) + (locals.var_fn97_calc_iq__qref0 * ((assign9370_e10282 * locals.var_fn97_calc_iq__eta0_dn4) / assign9370_e10283))), (locals.var_fn97_calc_iq__qref0 * ((assign9370_e10282 * locals.var_fn97_calc_iq__eta0_dn7) / assign9370_e10283)), (locals.var_fn97_calc_iq__qref0 * ((assign9370_e10282 * locals.var_fn97_calc_iq__eta0_dn14) / assign9370_e10283)), (locals.var_fn97_calc_iq__qref0 * ((assign9370_e10282 * locals.var_fn97_calc_iq__eta0_dn15) / assign9370_e10283)),)
    } else {
        (locals.var_fn97_calc_iq__qinvv0, locals.var_fn97_calc_iq__qinvv0_dn2, locals.var_fn97_calc_iq__qinvv0_dn4, locals.var_fn97_calc_iq__qinvv0_dn7, locals.var_fn97_calc_iq__qinvv0_dn14, locals.var_fn97_calc_iq__qinvv0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv0 = assign9370_e10287;
        locals.var_fn97_calc_iq__qinvv0_dn2 = assign9370_e10287_d_n2;
        locals.var_fn97_calc_iq__qinvv0_dn4 = assign9370_e10287_d_n4;
        locals.var_fn97_calc_iq__qinvv0_dn7 = assign9370_e10287_d_n7;
        locals.var_fn97_calc_iq__qinvv0_dn14 = assign9370_e10287_d_n14;
        locals.var_fn97_calc_iq__qinvv0_dn15 = assign9370_e10287_d_n15;

        let (assign9380_e10293, assign9380_e10293_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9380_e10291: f64 = (locals.var_fn97_calc_iq__mu0 / locals.var_fn97_calc_iq__tfacmobin);
        (assign9380_e10291, (-((locals.var_fn97_calc_iq__mu0 * locals.var_fn97_calc_iq__tfacmobin_dn4) / (locals.var_fn97_calc_iq__tfacmobin * locals.var_fn97_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn97_calc_iq__muf0, locals.var_fn97_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn97_calc_iq__muf0 = assign9380_e10293;
        locals.var_fn97_calc_iq__muf0_dn4 = assign9380_e10293_d_n4;

        let (assign9390_e10309, assign9390_e10309_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9390_e10299: f64 = (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tnomin);
        let assign9390_e10300: f64 = (1.0 + assign9390_e10299);
        let assign9390_e10304: f64 = (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tambin);
        let assign9390_e10305: f64 = (1.0 + assign9390_e10304);
        let assign9390_e10306: f64 = (assign9390_e10300 / assign9390_e10305);
        let assign9390_e10307: f64 = (locals.var_fn97_calc_iq__vel0 * assign9390_e10306);
        (assign9390_e10307, (locals.var_fn97_calc_iq__vel0 * (-((assign9390_e10300 * (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tambin_dn4)) / (assign9390_e10305 * assign9390_e10305)))),)
    } else {
        (locals.var_fn97_calc_iq__vx0, locals.var_fn97_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn97_calc_iq__vx0 = assign9390_e10309;
        locals.var_fn97_calc_iq__vx0_dn4 = assign9390_e10309_d_n4;

        let (assign9400_e10317, assign9400_e10317_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9400_e10313: f64 = (locals.var_fn97_calc_iq__vx0 * locals.var_fn97_calc_iq__lin);
        let assign9400_e10315: f64 = (assign9400_e10313 / locals.var_fn97_calc_iq__muf0);
        (assign9400_e10315, ((((locals.var_fn97_calc_iq__vx0_dn4 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf0) - (assign9400_e10313 * locals.var_fn97_calc_iq__muf0_dn4)) / (locals.var_fn97_calc_iq__muf0 * locals.var_fn97_calc_iq__muf0)),)
    } else {
        (locals.var_fn97_calc_iq__vdsats0, locals.var_fn97_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn97_calc_iq__vdsats0 = assign9400_e10317;
        locals.var_fn97_calc_iq__vdsats0_dn4 = assign9400_e10317_d_n4;

        let (assign9410_e10334, assign9410_e10334_d_n2, assign9410_e10334_d_n4, assign9410_e10334_d_n7, assign9410_e10334_d_n14, assign9410_e10334_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9410_e10323: f64 = (2.0 * locals.var_fn97_calc_iq__qinvv0);
        let assign9410_e10325: f64 = (assign9410_e10323 / locals.var_fn97_calc_iq__cgin);
        let assign9410_e10327: f64 = (assign9410_e10325 / locals.var_fn97_calc_iq__vdsats0);
        let assign9410_e10328: f64 = (1.0 + assign9410_e10327);
        let assign9410_e10329: f64 = (assign9410_e10328).sqrt();
        let assign9410_e10330: f64 = (locals.var_fn97_calc_iq__vdsats0 * assign9410_e10329);
        let assign9410_e10332: f64 = (assign9410_e10330 - locals.var_fn97_calc_iq__vdsats0);
        (assign9410_e10332, (locals.var_fn97_calc_iq__vdsats0 * ((((2.0 * locals.var_fn97_calc_iq__qinvv0_dn2) / locals.var_fn97_calc_iq__cgin) / locals.var_fn97_calc_iq__vdsats0) / (2.0 * assign9410_e10329))), (((locals.var_fn97_calc_iq__vdsats0_dn4 * assign9410_e10329) + (locals.var_fn97_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn97_calc_iq__qinvv0_dn4) * locals.var_fn97_calc_iq__cgin) - (assign9410_e10323 * locals.var_fn97_calc_iq__cgin_dn4)) / (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__cgin)) * locals.var_fn97_calc_iq__vdsats0) - (assign9410_e10325 * locals.var_fn97_calc_iq__vdsats0_dn4)) / (locals.var_fn97_calc_iq__vdsats0 * locals.var_fn97_calc_iq__vdsats0)) / (2.0 * assign9410_e10329)))) - locals.var_fn97_calc_iq__vdsats0_dn4), (locals.var_fn97_calc_iq__vdsats0 * ((((2.0 * locals.var_fn97_calc_iq__qinvv0_dn7) / locals.var_fn97_calc_iq__cgin) / locals.var_fn97_calc_iq__vdsats0) / (2.0 * assign9410_e10329))), (locals.var_fn97_calc_iq__vdsats0 * ((((2.0 * locals.var_fn97_calc_iq__qinvv0_dn14) / locals.var_fn97_calc_iq__cgin) / locals.var_fn97_calc_iq__vdsats0) / (2.0 * assign9410_e10329))), (locals.var_fn97_calc_iq__vdsats0 * ((((2.0 * locals.var_fn97_calc_iq__qinvv0_dn15) / locals.var_fn97_calc_iq__cgin) / locals.var_fn97_calc_iq__vdsats0) / (2.0 * assign9410_e10329))),)
    } else {
        (locals.var_fn97_calc_iq__vdsats10, locals.var_fn97_calc_iq__vdsats10_dn2, locals.var_fn97_calc_iq__vdsats10_dn4, locals.var_fn97_calc_iq__vdsats10_dn7, locals.var_fn97_calc_iq__vdsats10_dn14, locals.var_fn97_calc_iq__vdsats10_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats10 = assign9410_e10334;
        locals.var_fn97_calc_iq__vdsats10_dn2 = assign9410_e10334_d_n2;
        locals.var_fn97_calc_iq__vdsats10_dn4 = assign9410_e10334_d_n4;
        locals.var_fn97_calc_iq__vdsats10_dn7 = assign9410_e10334_d_n7;
        locals.var_fn97_calc_iq__vdsats10_dn14 = assign9410_e10334_d_n14;
        locals.var_fn97_calc_iq__vdsats10_dn15 = assign9410_e10334_d_n15;

        let (assign9420_e10346, assign9420_e10346_d_n2, assign9420_e10346_d_n4, assign9420_e10346_d_n7, assign9420_e10346_d_n14, assign9420_e10346_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9420_e10339: f64 = (1.0 - locals.var_fn97_calc_iq__ff0);
        let assign9420_e10340: f64 = (locals.var_fn97_calc_iq__vdsats10 * assign9420_e10339);
        let assign9420_e10343: f64 = (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0);
        let assign9420_e10344: f64 = (assign9420_e10340 + assign9420_e10343);
        (assign9420_e10344, (((locals.var_fn97_calc_iq__vdsats10_dn2 * assign9420_e10339) + (locals.var_fn97_calc_iq__vdsats10 * (-locals.var_fn97_calc_iq__ff0_dn2))) + (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0_dn2)), (((locals.var_fn97_calc_iq__vdsats10_dn4 * assign9420_e10339) + (locals.var_fn97_calc_iq__vdsats10 * (-locals.var_fn97_calc_iq__ff0_dn4))) + ((locals.var_fn97_calc_iq__two_n_phit0_dn4 * locals.var_fn97_calc_iq__ff0) + (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0_dn4))), (((locals.var_fn97_calc_iq__vdsats10_dn7 * assign9420_e10339) + (locals.var_fn97_calc_iq__vdsats10 * (-locals.var_fn97_calc_iq__ff0_dn7))) + (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0_dn7)), (((locals.var_fn97_calc_iq__vdsats10_dn14 * assign9420_e10339) + (locals.var_fn97_calc_iq__vdsats10 * (-locals.var_fn97_calc_iq__ff0_dn14))) + (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0_dn14)), (((locals.var_fn97_calc_iq__vdsats10_dn15 * assign9420_e10339) + (locals.var_fn97_calc_iq__vdsats10 * (-locals.var_fn97_calc_iq__ff0_dn15))) + (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__vdsat10, locals.var_fn97_calc_iq__vdsat10_dn2, locals.var_fn97_calc_iq__vdsat10_dn4, locals.var_fn97_calc_iq__vdsat10_dn7, locals.var_fn97_calc_iq__vdsat10_dn14, locals.var_fn97_calc_iq__vdsat10_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat10 = assign9420_e10346;
        locals.var_fn97_calc_iq__vdsat10_dn2 = assign9420_e10346_d_n2;
        locals.var_fn97_calc_iq__vdsat10_dn4 = assign9420_e10346_d_n4;
        locals.var_fn97_calc_iq__vdsat10_dn7 = assign9420_e10346_d_n7;
        locals.var_fn97_calc_iq__vdsat10_dn14 = assign9420_e10346_d_n14;
        locals.var_fn97_calc_iq__vdsat10_dn15 = assign9420_e10346_d_n15;

        let (assign9430_e10415, assign9430_e10415_d_n2, assign9430_e10415_d_n4, assign9430_e10415_d_n7, assign9430_e10415_d_n14, assign9430_e10415_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign9430_e10405, assign9430_e10405_d_n2, assign9430_e10405_d_n4, assign9430_e10405_d_n7, assign9430_e10405_d_n14, assign9430_e10405_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign9430_e10358: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                let assign9430_e10359: f64 = assign9430_e10358;
                let assign9430_e10363: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                let assign9430_e10364: f64 = (-assign9430_e10363);
                let assign9430_e10367: f64 = (0.001 / p.p53);
                let assign9430_e10371: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                let assign9430_e10372: f64 = (-assign9430_e10371);
                let assign9430_e10373: f64 = (assign9430_e10367 * assign9430_e10372);
                let assign9430_e10374: f64 = (assign9430_e10373).tanh();
                let assign9430_e10375: f64 = (assign9430_e10364 * assign9430_e10374);
                let assign9430_e10376: f64 = (assign9430_e10359 + assign9430_e10375);
                let assign9430_e10377: f64 = (0.5 * assign9430_e10376);
                (assign9430_e10377, (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10374) + (assign9430_e10364 * ((assign9430_e10367 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9430_e10373).cosh() * (assign9430_e10373).cosh())))))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10374) + (assign9430_e10364 * ((assign9430_e10367 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9430_e10373).cosh() * (assign9430_e10373).cosh())))))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10374) + (assign9430_e10364 * ((assign9430_e10367 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9430_e10373).cosh() * (assign9430_e10373).cosh())))))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + (((-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9430_e10374) + (assign9430_e10364 * ((assign9430_e10367 * (-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) / ((assign9430_e10373).cosh() * (assign9430_e10373).cosh())))))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + (((-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9430_e10374) + (assign9430_e10364 * ((assign9430_e10367 * (-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) / ((assign9430_e10373).cosh() * (assign9430_e10373).cosh())))))),)
            } else {
                let (assign9430_e10404, assign9430_e10404_d_n2, assign9430_e10404_d_n4, assign9430_e10404_d_n7, assign9430_e10404_d_n14, assign9430_e10404_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign9430_e10385: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                        let assign9430_e10386: f64 = assign9430_e10385;
                        let assign9430_e10390: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                        let assign9430_e10391: f64 = (-assign9430_e10390);
                        let assign9430_e10395: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                        let assign9430_e10396: f64 = (-assign9430_e10395);
                        let assign9430_e10397: f64 = (assign9430_e10391 * assign9430_e10396);
                        let assign9430_e10399: f64 = (assign9430_e10397 + p.p53);
                        let assign9430_e10400: f64 = (assign9430_e10399).sqrt();
                        let assign9430_e10401: f64 = (assign9430_e10386 + assign9430_e10400);
                        let assign9430_e10402: f64 = (0.5 * assign9430_e10401);
                        (assign9430_e10402, (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10396) + (assign9430_e10391 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9430_e10400)))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10396) + (assign9430_e10391 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9430_e10400)))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10396) + (assign9430_e10391 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9430_e10400)))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + ((((-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9430_e10396) + (assign9430_e10391 * (-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / (2.0 * assign9430_e10400)))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + ((((-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9430_e10396) + (assign9430_e10391 * (-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / (2.0 * assign9430_e10400)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign9430_e10404, assign9430_e10404_d_n2, assign9430_e10404_d_n4, assign9430_e10404_d_n7, assign9430_e10404_d_n14, assign9430_e10404_d_n15,)
            }
        };
        let assign9430_e10407: f64 = (assign9430_e10405).powf(locals.var_fn97_calc_iq__beta);
        let assign9430_e10408: f64 = (1.0 + assign9430_e10407);
        let assign9430_e10411: f64 = (1.0 / locals.var_fn97_calc_iq__beta);
        let assign9430_e10412: f64 = (assign9430_e10408).powf(assign9430_e10411);
        let assign9430_e10413: f64 = (1.0 / assign9430_e10412);
        (assign9430_e10413, (-(if 0.0 == 0.0 && ((assign9430_e10411) as f64).is_finite() && ((assign9430_e10411) as f64).fract() == 0.0 { if assign9430_e10411 == 0.0 { 0.0 } else { (assign9430_e10411 * ((assign9430_e10408).powf(assign9430_e10411 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n2)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n2 / assign9430_e10405))) })) } } else { (assign9430_e10412 * (assign9430_e10411 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n2)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n2 / assign9430_e10405))) } / assign9430_e10408))) } / (assign9430_e10412 * assign9430_e10412))), (-(if 0.0 == 0.0 && ((assign9430_e10411) as f64).is_finite() && ((assign9430_e10411) as f64).fract() == 0.0 { if assign9430_e10411 == 0.0 { 0.0 } else { (assign9430_e10411 * ((assign9430_e10408).powf(assign9430_e10411 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n4)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n4 / assign9430_e10405))) })) } } else { (assign9430_e10412 * (assign9430_e10411 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n4)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n4 / assign9430_e10405))) } / assign9430_e10408))) } / (assign9430_e10412 * assign9430_e10412))), (-(if 0.0 == 0.0 && ((assign9430_e10411) as f64).is_finite() && ((assign9430_e10411) as f64).fract() == 0.0 { if assign9430_e10411 == 0.0 { 0.0 } else { (assign9430_e10411 * ((assign9430_e10408).powf(assign9430_e10411 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n7)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n7 / assign9430_e10405))) })) } } else { (assign9430_e10412 * (assign9430_e10411 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n7)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n7 / assign9430_e10405))) } / assign9430_e10408))) } / (assign9430_e10412 * assign9430_e10412))), (-(if 0.0 == 0.0 && ((assign9430_e10411) as f64).is_finite() && ((assign9430_e10411) as f64).fract() == 0.0 { if assign9430_e10411 == 0.0 { 0.0 } else { (assign9430_e10411 * ((assign9430_e10408).powf(assign9430_e10411 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n14)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n14 / assign9430_e10405))) })) } } else { (assign9430_e10412 * (assign9430_e10411 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n14)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n14 / assign9430_e10405))) } / assign9430_e10408))) } / (assign9430_e10412 * assign9430_e10412))), (-(if 0.0 == 0.0 && ((assign9430_e10411) as f64).is_finite() && ((assign9430_e10411) as f64).fract() == 0.0 { if assign9430_e10411 == 0.0 { 0.0 } else { (assign9430_e10411 * ((assign9430_e10408).powf(assign9430_e10411 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n15)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n15 / assign9430_e10405))) })) } } else { (assign9430_e10412 * (assign9430_e10411 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n15)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n15 / assign9430_e10405))) } / assign9430_e10408))) } / (assign9430_e10412 * assign9430_e10412))),)
    } else {
        (locals.var_fn97_calc_iq__fsd0, locals.var_fn97_calc_iq__fsd0_dn2, locals.var_fn97_calc_iq__fsd0_dn4, locals.var_fn97_calc_iq__fsd0_dn7, locals.var_fn97_calc_iq__fsd0_dn14, locals.var_fn97_calc_iq__fsd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__fsd0 = assign9430_e10415;
        locals.var_fn97_calc_iq__fsd0_dn2 = assign9430_e10415_d_n2;
        locals.var_fn97_calc_iq__fsd0_dn4 = assign9430_e10415_d_n4;
        locals.var_fn97_calc_iq__fsd0_dn7 = assign9430_e10415_d_n7;
        locals.var_fn97_calc_iq__fsd0_dn14 = assign9430_e10415_d_n14;
        locals.var_fn97_calc_iq__fsd0_dn15 = assign9430_e10415_d_n15;

        let (assign9440_e10421, assign9440_e10421_d_n2, assign9440_e10421_d_n4, assign9440_e10421_d_n7, assign9440_e10421_d_n14, assign9440_e10421_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9440_e10419: f64 = (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0);
        (assign9440_e10419, (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0_dn2), (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0_dn4), (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0_dn7), ((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__fsd0) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0_dn14)), ((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__fsd0) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__vdx0, locals.var_fn97_calc_iq__vdx0_dn2, locals.var_fn97_calc_iq__vdx0_dn4, locals.var_fn97_calc_iq__vdx0_dn7, locals.var_fn97_calc_iq__vdx0_dn14, locals.var_fn97_calc_iq__vdx0_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdx0 = assign9440_e10421;
        locals.var_fn97_calc_iq__vdx0_dn2 = assign9440_e10421_d_n2;
        locals.var_fn97_calc_iq__vdx0_dn4 = assign9440_e10421_d_n4;
        locals.var_fn97_calc_iq__vdx0_dn7 = assign9440_e10421_d_n7;
        locals.var_fn97_calc_iq__vdx0_dn14 = assign9440_e10421_d_n14;
        locals.var_fn97_calc_iq__vdx0_dn15 = assign9440_e10421_d_n15;

        let (assign9450_e10496, assign9450_e10496_d_n2, assign9450_e10496_d_n4, assign9450_e10496_d_n7, assign9450_e10496_d_n14, assign9450_e10496_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign9450_e10486, assign9450_e10486_d_n2, assign9450_e10486_d_n4, assign9450_e10486_d_n7, assign9450_e10486_d_n14, assign9450_e10486_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign9450_e10432: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign9450_e10434: f64 = (assign9450_e10432 / locals.var_fn97_calc_iq__vdsat10);
                let assign9450_e10435: f64 = assign9450_e10434;
                let assign9450_e10438: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign9450_e10440: f64 = (assign9450_e10438 / locals.var_fn97_calc_iq__vdsat10);
                let assign9450_e10441: f64 = (-assign9450_e10440);
                let assign9450_e10444: f64 = (0.001 / p.p53);
                let assign9450_e10447: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign9450_e10449: f64 = (assign9450_e10447 / locals.var_fn97_calc_iq__vdsat10);
                let assign9450_e10450: f64 = (-assign9450_e10449);
                let assign9450_e10451: f64 = (assign9450_e10444 * assign9450_e10450);
                let assign9450_e10452: f64 = (assign9450_e10451).tanh();
                let assign9450_e10453: f64 = (assign9450_e10441 * assign9450_e10452);
                let assign9450_e10454: f64 = (assign9450_e10435 + assign9450_e10453);
                let assign9450_e10455: f64 = (0.5 * assign9450_e10454);
                (assign9450_e10455, (0.5 * ((-((assign9450_e10432 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((assign9450_e10438 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10452) + (assign9450_e10441 * ((assign9450_e10444 * (-(-((assign9450_e10447 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9450_e10451).cosh() * (assign9450_e10451).cosh())))))), (0.5 * ((-((assign9450_e10432 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((assign9450_e10438 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10452) + (assign9450_e10441 * ((assign9450_e10444 * (-(-((assign9450_e10447 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9450_e10451).cosh() * (assign9450_e10451).cosh())))))), (0.5 * ((-((assign9450_e10432 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((assign9450_e10438 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10452) + (assign9450_e10441 * ((assign9450_e10444 * (-(-((assign9450_e10447 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9450_e10451).cosh() * (assign9450_e10451).cosh())))))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10432 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + (((-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10438 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9450_e10452) + (assign9450_e10441 * ((assign9450_e10444 * (-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10447 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) / ((assign9450_e10451).cosh() * (assign9450_e10451).cosh())))))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10432 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + (((-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10438 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9450_e10452) + (assign9450_e10441 * ((assign9450_e10444 * (-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10447 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) / ((assign9450_e10451).cosh() * (assign9450_e10451).cosh())))))),)
            } else {
                let (assign9450_e10485, assign9450_e10485_d_n2, assign9450_e10485_d_n4, assign9450_e10485_d_n7, assign9450_e10485_d_n14, assign9450_e10485_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign9450_e10462: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign9450_e10464: f64 = (assign9450_e10462 / locals.var_fn97_calc_iq__vdsat10);
                        let assign9450_e10465: f64 = assign9450_e10464;
                        let assign9450_e10468: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign9450_e10470: f64 = (assign9450_e10468 / locals.var_fn97_calc_iq__vdsat10);
                        let assign9450_e10471: f64 = (-assign9450_e10470);
                        let assign9450_e10474: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign9450_e10476: f64 = (assign9450_e10474 / locals.var_fn97_calc_iq__vdsat10);
                        let assign9450_e10477: f64 = (-assign9450_e10476);
                        let assign9450_e10478: f64 = (assign9450_e10471 * assign9450_e10477);
                        let assign9450_e10480: f64 = (assign9450_e10478 + p.p53);
                        let assign9450_e10481: f64 = (assign9450_e10480).sqrt();
                        let assign9450_e10482: f64 = (assign9450_e10465 + assign9450_e10481);
                        let assign9450_e10483: f64 = (0.5 * assign9450_e10482);
                        (assign9450_e10483, (0.5 * ((-((assign9450_e10462 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((assign9450_e10468 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10477) + (assign9450_e10471 * (-(-((assign9450_e10474 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9450_e10481)))), (0.5 * ((-((assign9450_e10462 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((assign9450_e10468 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10477) + (assign9450_e10471 * (-(-((assign9450_e10474 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9450_e10481)))), (0.5 * ((-((assign9450_e10462 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((assign9450_e10468 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10477) + (assign9450_e10471 * (-(-((assign9450_e10474 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9450_e10481)))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10462 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + ((((-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10468 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9450_e10477) + (assign9450_e10471 * (-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10474 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / (2.0 * assign9450_e10481)))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10462 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + ((((-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10468 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9450_e10477) + (assign9450_e10471 * (-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10474 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / (2.0 * assign9450_e10481)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign9450_e10485, assign9450_e10485_d_n2, assign9450_e10485_d_n4, assign9450_e10485_d_n7, assign9450_e10485_d_n14, assign9450_e10485_d_n15,)
            }
        };
        let assign9450_e10488: f64 = (assign9450_e10486).powf(locals.var_fn97_calc_iq__beta);
        let assign9450_e10489: f64 = (1.0 + assign9450_e10488);
        let assign9450_e10492: f64 = (1.0 / locals.var_fn97_calc_iq__beta);
        let assign9450_e10493: f64 = (assign9450_e10489).powf(assign9450_e10492);
        let assign9450_e10494: f64 = (1.0 / assign9450_e10493);
        (assign9450_e10494, (-(if 0.0 == 0.0 && ((assign9450_e10492) as f64).is_finite() && ((assign9450_e10492) as f64).fract() == 0.0 { if assign9450_e10492 == 0.0 { 0.0 } else { (assign9450_e10492 * ((assign9450_e10489).powf(assign9450_e10492 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n2)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n2 / assign9450_e10486))) })) } } else { (assign9450_e10493 * (assign9450_e10492 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n2)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n2 / assign9450_e10486))) } / assign9450_e10489))) } / (assign9450_e10493 * assign9450_e10493))), (-(if 0.0 == 0.0 && ((assign9450_e10492) as f64).is_finite() && ((assign9450_e10492) as f64).fract() == 0.0 { if assign9450_e10492 == 0.0 { 0.0 } else { (assign9450_e10492 * ((assign9450_e10489).powf(assign9450_e10492 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n4)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n4 / assign9450_e10486))) })) } } else { (assign9450_e10493 * (assign9450_e10492 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n4)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n4 / assign9450_e10486))) } / assign9450_e10489))) } / (assign9450_e10493 * assign9450_e10493))), (-(if 0.0 == 0.0 && ((assign9450_e10492) as f64).is_finite() && ((assign9450_e10492) as f64).fract() == 0.0 { if assign9450_e10492 == 0.0 { 0.0 } else { (assign9450_e10492 * ((assign9450_e10489).powf(assign9450_e10492 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n7)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n7 / assign9450_e10486))) })) } } else { (assign9450_e10493 * (assign9450_e10492 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n7)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n7 / assign9450_e10486))) } / assign9450_e10489))) } / (assign9450_e10493 * assign9450_e10493))), (-(if 0.0 == 0.0 && ((assign9450_e10492) as f64).is_finite() && ((assign9450_e10492) as f64).fract() == 0.0 { if assign9450_e10492 == 0.0 { 0.0 } else { (assign9450_e10492 * ((assign9450_e10489).powf(assign9450_e10492 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n14)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n14 / assign9450_e10486))) })) } } else { (assign9450_e10493 * (assign9450_e10492 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n14)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n14 / assign9450_e10486))) } / assign9450_e10489))) } / (assign9450_e10493 * assign9450_e10493))), (-(if 0.0 == 0.0 && ((assign9450_e10492) as f64).is_finite() && ((assign9450_e10492) as f64).fract() == 0.0 { if assign9450_e10492 == 0.0 { 0.0 } else { (assign9450_e10492 * ((assign9450_e10489).powf(assign9450_e10492 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n15)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n15 / assign9450_e10486))) })) } } else { (assign9450_e10493 * (assign9450_e10492 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n15)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n15 / assign9450_e10486))) } / assign9450_e10489))) } / (assign9450_e10493 * assign9450_e10493))),)
    } else {
        (locals.var_fn97_calc_iq__fds0, locals.var_fn97_calc_iq__fds0_dn2, locals.var_fn97_calc_iq__fds0_dn4, locals.var_fn97_calc_iq__fds0_dn7, locals.var_fn97_calc_iq__fds0_dn14, locals.var_fn97_calc_iq__fds0_dn15,)
    }
};
        locals.var_fn97_calc_iq__fds0 = assign9450_e10496;
        locals.var_fn97_calc_iq__fds0_dn2 = assign9450_e10496_d_n2;
        locals.var_fn97_calc_iq__fds0_dn4 = assign9450_e10496_d_n4;
        locals.var_fn97_calc_iq__fds0_dn7 = assign9450_e10496_d_n7;
        locals.var_fn97_calc_iq__fds0_dn14 = assign9450_e10496_d_n14;
        locals.var_fn97_calc_iq__fds0_dn15 = assign9450_e10496_d_n15;

        let (assign9460_e10503, assign9460_e10503_d_n2, assign9460_e10503_d_n4, assign9460_e10503_d_n7, assign9460_e10503_d_n14, assign9460_e10503_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9460_e10499: f64 = (-locals.var_fn97_calc_iq__vdsin);
        let assign9460_e10501: f64 = (assign9460_e10499 * locals.var_fn97_calc_iq__fds0);
        (assign9460_e10501, (assign9460_e10499 * locals.var_fn97_calc_iq__fds0_dn2), (assign9460_e10499 * locals.var_fn97_calc_iq__fds0_dn4), (assign9460_e10499 * locals.var_fn97_calc_iq__fds0_dn7), (((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__fds0) + (assign9460_e10499 * locals.var_fn97_calc_iq__fds0_dn14)), (((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__fds0) + (assign9460_e10499 * locals.var_fn97_calc_iq__fds0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__vsx0, locals.var_fn97_calc_iq__vsx0_dn2, locals.var_fn97_calc_iq__vsx0_dn4, locals.var_fn97_calc_iq__vsx0_dn7, locals.var_fn97_calc_iq__vsx0_dn14, locals.var_fn97_calc_iq__vsx0_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsx0 = assign9460_e10503;
        locals.var_fn97_calc_iq__vsx0_dn2 = assign9460_e10503_d_n2;
        locals.var_fn97_calc_iq__vsx0_dn4 = assign9460_e10503_d_n4;
        locals.var_fn97_calc_iq__vsx0_dn7 = assign9460_e10503_d_n7;
        locals.var_fn97_calc_iq__vsx0_dn14 = assign9460_e10503_d_n14;
        locals.var_fn97_calc_iq__vsx0_dn15 = assign9460_e10503_d_n15;

        let (assign9470_e10511, assign9470_e10511_d_n2, assign9470_e10511_d_n4, assign9470_e10511_d_n7, assign9470_e10511_d_n14, assign9470_e10511_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9470_e10507: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__myarg0);
        let assign9470_e10509: f64 = (assign9470_e10507 / locals.var_fn97_calc_iq__alpha_phit);
        (assign9470_e10509, (locals.var_fn97_calc_iq__vgsin_dn2 / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg0_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign9470_e10507 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), (locals.var_fn97_calc_iq__vgsin_dn7 / locals.var_fn97_calc_iq__alpha_phit), (locals.var_fn97_calc_iq__vgsin_dn14 / locals.var_fn97_calc_iq__alpha_phit), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg0, locals.var_fn97_calc_iq__exparg0_dn2, locals.var_fn97_calc_iq__exparg0_dn4, locals.var_fn97_calc_iq__exparg0_dn7, locals.var_fn97_calc_iq__exparg0_dn14, locals.var_fn97_calc_iq__exparg0_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg0 = assign9470_e10511;
        locals.var_fn97_calc_iq__exparg0_dn2 = assign9470_e10511_d_n2;
        locals.var_fn97_calc_iq__exparg0_dn4 = assign9470_e10511_d_n4;
        locals.var_fn97_calc_iq__exparg0_dn7 = assign9470_e10511_d_n7;
        locals.var_fn97_calc_iq__exparg0_dn14 = assign9470_e10511_d_n14;
        locals.var_fn97_calc_iq__exparg0_dn15 = assign9470_e10511_d_n15;

        let assign9480_e10514: f64 = if locals.var_fn97_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign9480_e10514;

        let (assign9490_e10520, assign9490_e10520_d_n2, assign9490_e10520_d_n4, assign9490_e10520_d_n7, assign9490_e10520_d_n14, assign9490_e10520_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard115 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs0, locals.var_fn97_calc_iq__ffs0_dn2, locals.var_fn97_calc_iq__ffs0_dn4, locals.var_fn97_calc_iq__ffs0_dn7, locals.var_fn97_calc_iq__ffs0_dn14, locals.var_fn97_calc_iq__ffs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs0 = assign9490_e10520;
        locals.var_fn97_calc_iq__ffs0_dn2 = assign9490_e10520_d_n2;
        locals.var_fn97_calc_iq__ffs0_dn4 = assign9490_e10520_d_n4;
        locals.var_fn97_calc_iq__ffs0_dn7 = assign9490_e10520_d_n7;
        locals.var_fn97_calc_iq__ffs0_dn14 = assign9490_e10520_d_n14;
        locals.var_fn97_calc_iq__ffs0_dn15 = assign9490_e10520_d_n15;

        let assign9500_e10523: f64 = (-50.0);
        let assign9500_e10524: f64 = if locals.var_fn97_calc_iq__exparg0 < assign9500_e10523 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign9500_e10524;

        let (assign9510_e10533, assign9510_e10533_d_n2, assign9510_e10533_d_n4, assign9510_e10533_d_n7, assign9510_e10533_d_n14, assign9510_e10533_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard115 == 0.0)) && (locals.var_guard116 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs0, locals.var_fn97_calc_iq__ffs0_dn2, locals.var_fn97_calc_iq__ffs0_dn4, locals.var_fn97_calc_iq__ffs0_dn7, locals.var_fn97_calc_iq__ffs0_dn14, locals.var_fn97_calc_iq__ffs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs0 = assign9510_e10533;
        locals.var_fn97_calc_iq__ffs0_dn2 = assign9510_e10533_d_n2;
        locals.var_fn97_calc_iq__ffs0_dn4 = assign9510_e10533_d_n4;
        locals.var_fn97_calc_iq__ffs0_dn7 = assign9510_e10533_d_n7;
        locals.var_fn97_calc_iq__ffs0_dn14 = assign9510_e10533_d_n14;
        locals.var_fn97_calc_iq__ffs0_dn15 = assign9510_e10533_d_n15;

        let (assign9520_e10548, assign9520_e10548_d_n2, assign9520_e10548_d_n4, assign9520_e10548_d_n7, assign9520_e10548_d_n14, assign9520_e10548_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard115 == 0.0)) && (locals.var_guard116 == 0.0)) {
        let assign9520_e10544: f64 = (locals.var_fn97_calc_iq__exparg0).exp();
        let assign9520_e10545: f64 = (1.0 + assign9520_e10544);
        let assign9520_e10546: f64 = (1.0 / assign9520_e10545);
        (assign9520_e10546, (-((assign9520_e10544 * locals.var_fn97_calc_iq__exparg0_dn2) / (assign9520_e10545 * assign9520_e10545))), (-((assign9520_e10544 * locals.var_fn97_calc_iq__exparg0_dn4) / (assign9520_e10545 * assign9520_e10545))), (-((assign9520_e10544 * locals.var_fn97_calc_iq__exparg0_dn7) / (assign9520_e10545 * assign9520_e10545))), (-((assign9520_e10544 * locals.var_fn97_calc_iq__exparg0_dn14) / (assign9520_e10545 * assign9520_e10545))), (-((assign9520_e10544 * locals.var_fn97_calc_iq__exparg0_dn15) / (assign9520_e10545 * assign9520_e10545))),)
    } else {
        (locals.var_fn97_calc_iq__ffs0, locals.var_fn97_calc_iq__ffs0_dn2, locals.var_fn97_calc_iq__ffs0_dn4, locals.var_fn97_calc_iq__ffs0_dn7, locals.var_fn97_calc_iq__ffs0_dn14, locals.var_fn97_calc_iq__ffs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs0 = assign9520_e10548;
        locals.var_fn97_calc_iq__ffs0_dn2 = assign9520_e10548_d_n2;
        locals.var_fn97_calc_iq__ffs0_dn4 = assign9520_e10548_d_n4;
        locals.var_fn97_calc_iq__ffs0_dn7 = assign9520_e10548_d_n7;
        locals.var_fn97_calc_iq__ffs0_dn14 = assign9520_e10548_d_n14;
        locals.var_fn97_calc_iq__ffs0_dn15 = assign9520_e10548_d_n15;

    }

    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9530_e10566, assign9530_e10566_d_n2, assign9530_e10566_d_n4, assign9530_e10566_d_n7, assign9530_e10566_d_n14, assign9530_e10566_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9530_e10552: f64 = (locals.var_fn97_calc_iq__vgdin - locals.var_fn97_calc_iq__vsx0);
        let assign9530_e10556: f64 = (p.p51 * 0.1);
        let assign9530_e10558: f64 = (assign9530_e10556 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9530_e10560: f64 = (assign9530_e10558 * locals.var_fn97_calc_iq__ffs0);
        let assign9530_e10561: f64 = (locals.var_fn97_calc_iq__vtof - assign9530_e10560);
        let assign9530_e10562: f64 = (assign9530_e10552 - assign9530_e10561);
        let assign9530_e10564: f64 = (assign9530_e10562 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign9530_e10564, (((locals.var_fn97_calc_iq__vgdin_dn2 - locals.var_fn97_calc_iq__vsx0_dn2) - (-(assign9530_e10558 * locals.var_fn97_calc_iq__ffs0_dn2))) / locals.var_fn97_calc_iq__two_n_phit0), (((((-locals.var_fn97_calc_iq__vsx0_dn4) - (locals.var_fn97_calc_iq__vtof_dn4 - (((assign9530_e10556 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ffs0) + (assign9530_e10558 * locals.var_fn97_calc_iq__ffs0_dn4)))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign9530_e10562 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), (((locals.var_fn97_calc_iq__vgdin_dn7 - locals.var_fn97_calc_iq__vsx0_dn7) - (-(assign9530_e10558 * locals.var_fn97_calc_iq__ffs0_dn7))) / locals.var_fn97_calc_iq__two_n_phit0), (((locals.var_fn97_calc_iq__vgdin_dn14 - locals.var_fn97_calc_iq__vsx0_dn14) - (-(assign9530_e10558 * locals.var_fn97_calc_iq__ffs0_dn14))) / locals.var_fn97_calc_iq__two_n_phit0), (((locals.var_fn97_calc_iq__vgdin_dn15 - locals.var_fn97_calc_iq__vsx0_dn15) - (-(assign9530_e10558 * locals.var_fn97_calc_iq__ffs0_dn15))) / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__etas0, locals.var_fn97_calc_iq__etas0_dn2, locals.var_fn97_calc_iq__etas0_dn4, locals.var_fn97_calc_iq__etas0_dn7, locals.var_fn97_calc_iq__etas0_dn14, locals.var_fn97_calc_iq__etas0_dn15,)
    }
};
        locals.var_fn97_calc_iq__etas0 = assign9530_e10566;
        locals.var_fn97_calc_iq__etas0_dn2 = assign9530_e10566_d_n2;
        locals.var_fn97_calc_iq__etas0_dn4 = assign9530_e10566_d_n4;
        locals.var_fn97_calc_iq__etas0_dn7 = assign9530_e10566_d_n7;
        locals.var_fn97_calc_iq__etas0_dn14 = assign9530_e10566_d_n14;
        locals.var_fn97_calc_iq__etas0_dn15 = assign9530_e10566_d_n15;

        let assign9540_e10569: f64 = if locals.var_fn97_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign9540_e10569;

        let (assign9550_e10577, assign9550_e10577_d_n2, assign9550_e10577_d_n4, assign9550_e10577_d_n7, assign9550_e10577_d_n14, assign9550_e10577_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard117 != 0.0)) {
        let assign9550_e10575: f64 = (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0);
        (assign9550_e10575, (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0_dn2), ((locals.var_fn97_calc_iq__qref0_dn4 * locals.var_fn97_calc_iq__etas0) + (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0_dn4)), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0_dn7), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0_dn14), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0_dn15),)
    } else {
        (locals.var_fn97_calc_iq__qinvs0, locals.var_fn97_calc_iq__qinvs0_dn2, locals.var_fn97_calc_iq__qinvs0_dn4, locals.var_fn97_calc_iq__qinvs0_dn7, locals.var_fn97_calc_iq__qinvs0_dn14, locals.var_fn97_calc_iq__qinvs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs0 = assign9550_e10577;
        locals.var_fn97_calc_iq__qinvs0_dn2 = assign9550_e10577_d_n2;
        locals.var_fn97_calc_iq__qinvs0_dn4 = assign9550_e10577_d_n4;
        locals.var_fn97_calc_iq__qinvs0_dn7 = assign9550_e10577_d_n7;
        locals.var_fn97_calc_iq__qinvs0_dn14 = assign9550_e10577_d_n14;
        locals.var_fn97_calc_iq__qinvs0_dn15 = assign9550_e10577_d_n15;

        let assign9560_e10580: f64 = (-50.0);
        let assign9560_e10581: f64 = if locals.var_fn97_calc_iq__etas0 < assign9560_e10580 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign9560_e10581;

        let (assign9570_e10593, assign9570_e10593_d_n2, assign9570_e10593_d_n4, assign9570_e10593_d_n7, assign9570_e10593_d_n14, assign9570_e10593_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard117 == 0.0)) && (locals.var_guard118 != 0.0)) {
        let assign9570_e10590: f64 = (locals.var_fn97_calc_iq__etas0).exp();
        let assign9570_e10591: f64 = (locals.var_fn97_calc_iq__qref0 * assign9570_e10590);
        (assign9570_e10591, (locals.var_fn97_calc_iq__qref0 * (assign9570_e10590 * locals.var_fn97_calc_iq__etas0_dn2)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9570_e10590) + (locals.var_fn97_calc_iq__qref0 * (assign9570_e10590 * locals.var_fn97_calc_iq__etas0_dn4))), (locals.var_fn97_calc_iq__qref0 * (assign9570_e10590 * locals.var_fn97_calc_iq__etas0_dn7)), (locals.var_fn97_calc_iq__qref0 * (assign9570_e10590 * locals.var_fn97_calc_iq__etas0_dn14)), (locals.var_fn97_calc_iq__qref0 * (assign9570_e10590 * locals.var_fn97_calc_iq__etas0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvs0, locals.var_fn97_calc_iq__qinvs0_dn2, locals.var_fn97_calc_iq__qinvs0_dn4, locals.var_fn97_calc_iq__qinvs0_dn7, locals.var_fn97_calc_iq__qinvs0_dn14, locals.var_fn97_calc_iq__qinvs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs0 = assign9570_e10593;
        locals.var_fn97_calc_iq__qinvs0_dn2 = assign9570_e10593_d_n2;
        locals.var_fn97_calc_iq__qinvs0_dn4 = assign9570_e10593_d_n4;
        locals.var_fn97_calc_iq__qinvs0_dn7 = assign9570_e10593_d_n7;
        locals.var_fn97_calc_iq__qinvs0_dn14 = assign9570_e10593_d_n14;
        locals.var_fn97_calc_iq__qinvs0_dn15 = assign9570_e10593_d_n15;

        let (assign9580_e10609, assign9580_e10609_d_n2, assign9580_e10609_d_n4, assign9580_e10609_d_n7, assign9580_e10609_d_n14, assign9580_e10609_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard117 == 0.0)) && (locals.var_guard118 == 0.0)) {
        let assign9580_e10604: f64 = (locals.var_fn97_calc_iq__etas0).exp();
        let assign9580_e10605: f64 = (1.0 + assign9580_e10604);
        let assign9580_e10606: f64 = (assign9580_e10605).ln();
        let assign9580_e10607: f64 = (locals.var_fn97_calc_iq__qref0 * assign9580_e10606);
        (assign9580_e10607, (locals.var_fn97_calc_iq__qref0 * ((assign9580_e10604 * locals.var_fn97_calc_iq__etas0_dn2) / assign9580_e10605)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9580_e10606) + (locals.var_fn97_calc_iq__qref0 * ((assign9580_e10604 * locals.var_fn97_calc_iq__etas0_dn4) / assign9580_e10605))), (locals.var_fn97_calc_iq__qref0 * ((assign9580_e10604 * locals.var_fn97_calc_iq__etas0_dn7) / assign9580_e10605)), (locals.var_fn97_calc_iq__qref0 * ((assign9580_e10604 * locals.var_fn97_calc_iq__etas0_dn14) / assign9580_e10605)), (locals.var_fn97_calc_iq__qref0 * ((assign9580_e10604 * locals.var_fn97_calc_iq__etas0_dn15) / assign9580_e10605)),)
    } else {
        (locals.var_fn97_calc_iq__qinvs0, locals.var_fn97_calc_iq__qinvs0_dn2, locals.var_fn97_calc_iq__qinvs0_dn4, locals.var_fn97_calc_iq__qinvs0_dn7, locals.var_fn97_calc_iq__qinvs0_dn14, locals.var_fn97_calc_iq__qinvs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs0 = assign9580_e10609;
        locals.var_fn97_calc_iq__qinvs0_dn2 = assign9580_e10609_d_n2;
        locals.var_fn97_calc_iq__qinvs0_dn4 = assign9580_e10609_d_n4;
        locals.var_fn97_calc_iq__qinvs0_dn7 = assign9580_e10609_d_n7;
        locals.var_fn97_calc_iq__qinvs0_dn14 = assign9580_e10609_d_n14;
        locals.var_fn97_calc_iq__qinvs0_dn15 = assign9580_e10609_d_n15;

        let (assign9590_e10617, assign9590_e10617_d_n2, assign9590_e10617_d_n4, assign9590_e10617_d_n7, assign9590_e10617_d_n14, assign9590_e10617_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9590_e10613: f64 = (locals.var_fn97_calc_iq__vgdin - locals.var_fn97_calc_iq__myarg0);
        let assign9590_e10615: f64 = (assign9590_e10613 / locals.var_fn97_calc_iq__alpha_phit);
        (assign9590_e10615, (locals.var_fn97_calc_iq__vgdin_dn2 / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg0_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign9590_e10613 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), (locals.var_fn97_calc_iq__vgdin_dn7 / locals.var_fn97_calc_iq__alpha_phit), (locals.var_fn97_calc_iq__vgdin_dn14 / locals.var_fn97_calc_iq__alpha_phit), (locals.var_fn97_calc_iq__vgdin_dn15 / locals.var_fn97_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn97_calc_iq__exparg0, locals.var_fn97_calc_iq__exparg0_dn2, locals.var_fn97_calc_iq__exparg0_dn4, locals.var_fn97_calc_iq__exparg0_dn7, locals.var_fn97_calc_iq__exparg0_dn14, locals.var_fn97_calc_iq__exparg0_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg0 = assign9590_e10617;
        locals.var_fn97_calc_iq__exparg0_dn2 = assign9590_e10617_d_n2;
        locals.var_fn97_calc_iq__exparg0_dn4 = assign9590_e10617_d_n4;
        locals.var_fn97_calc_iq__exparg0_dn7 = assign9590_e10617_d_n7;
        locals.var_fn97_calc_iq__exparg0_dn14 = assign9590_e10617_d_n14;
        locals.var_fn97_calc_iq__exparg0_dn15 = assign9590_e10617_d_n15;

        let assign9600_e10620: f64 = if locals.var_fn97_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign9600_e10620;

        let (assign9610_e10626, assign9610_e10626_d_n2, assign9610_e10626_d_n4, assign9610_e10626_d_n7, assign9610_e10626_d_n14, assign9610_e10626_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard119 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd0, locals.var_fn97_calc_iq__ffd0_dn2, locals.var_fn97_calc_iq__ffd0_dn4, locals.var_fn97_calc_iq__ffd0_dn7, locals.var_fn97_calc_iq__ffd0_dn14, locals.var_fn97_calc_iq__ffd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd0 = assign9610_e10626;
        locals.var_fn97_calc_iq__ffd0_dn2 = assign9610_e10626_d_n2;
        locals.var_fn97_calc_iq__ffd0_dn4 = assign9610_e10626_d_n4;
        locals.var_fn97_calc_iq__ffd0_dn7 = assign9610_e10626_d_n7;
        locals.var_fn97_calc_iq__ffd0_dn14 = assign9610_e10626_d_n14;
        locals.var_fn97_calc_iq__ffd0_dn15 = assign9610_e10626_d_n15;

        let assign9620_e10629: f64 = (-50.0);
        let assign9620_e10630: f64 = if locals.var_fn97_calc_iq__exparg0 < assign9620_e10629 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign9620_e10630;

        let (assign9630_e10639, assign9630_e10639_d_n2, assign9630_e10639_d_n4, assign9630_e10639_d_n7, assign9630_e10639_d_n14, assign9630_e10639_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard119 == 0.0)) && (locals.var_guard120 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd0, locals.var_fn97_calc_iq__ffd0_dn2, locals.var_fn97_calc_iq__ffd0_dn4, locals.var_fn97_calc_iq__ffd0_dn7, locals.var_fn97_calc_iq__ffd0_dn14, locals.var_fn97_calc_iq__ffd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd0 = assign9630_e10639;
        locals.var_fn97_calc_iq__ffd0_dn2 = assign9630_e10639_d_n2;
        locals.var_fn97_calc_iq__ffd0_dn4 = assign9630_e10639_d_n4;
        locals.var_fn97_calc_iq__ffd0_dn7 = assign9630_e10639_d_n7;
        locals.var_fn97_calc_iq__ffd0_dn14 = assign9630_e10639_d_n14;
        locals.var_fn97_calc_iq__ffd0_dn15 = assign9630_e10639_d_n15;

        let (assign9640_e10654, assign9640_e10654_d_n2, assign9640_e10654_d_n4, assign9640_e10654_d_n7, assign9640_e10654_d_n14, assign9640_e10654_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard119 == 0.0)) && (locals.var_guard120 == 0.0)) {
        let assign9640_e10650: f64 = (locals.var_fn97_calc_iq__exparg0).exp();
        let assign9640_e10651: f64 = (1.0 + assign9640_e10650);
        let assign9640_e10652: f64 = (1.0 / assign9640_e10651);
        (assign9640_e10652, (-((assign9640_e10650 * locals.var_fn97_calc_iq__exparg0_dn2) / (assign9640_e10651 * assign9640_e10651))), (-((assign9640_e10650 * locals.var_fn97_calc_iq__exparg0_dn4) / (assign9640_e10651 * assign9640_e10651))), (-((assign9640_e10650 * locals.var_fn97_calc_iq__exparg0_dn7) / (assign9640_e10651 * assign9640_e10651))), (-((assign9640_e10650 * locals.var_fn97_calc_iq__exparg0_dn14) / (assign9640_e10651 * assign9640_e10651))), (-((assign9640_e10650 * locals.var_fn97_calc_iq__exparg0_dn15) / (assign9640_e10651 * assign9640_e10651))),)
    } else {
        (locals.var_fn97_calc_iq__ffd0, locals.var_fn97_calc_iq__ffd0_dn2, locals.var_fn97_calc_iq__ffd0_dn4, locals.var_fn97_calc_iq__ffd0_dn7, locals.var_fn97_calc_iq__ffd0_dn14, locals.var_fn97_calc_iq__ffd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd0 = assign9640_e10654;
        locals.var_fn97_calc_iq__ffd0_dn2 = assign9640_e10654_d_n2;
        locals.var_fn97_calc_iq__ffd0_dn4 = assign9640_e10654_d_n4;
        locals.var_fn97_calc_iq__ffd0_dn7 = assign9640_e10654_d_n7;
        locals.var_fn97_calc_iq__ffd0_dn14 = assign9640_e10654_d_n14;
        locals.var_fn97_calc_iq__ffd0_dn15 = assign9640_e10654_d_n15;

        let (assign9650_e10672, assign9650_e10672_d_n2, assign9650_e10672_d_n4, assign9650_e10672_d_n7, assign9650_e10672_d_n14, assign9650_e10672_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9650_e10658: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vdx0);
        let assign9650_e10662: f64 = (p.p51 * 0.1);
        let assign9650_e10664: f64 = (assign9650_e10662 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9650_e10666: f64 = (assign9650_e10664 * locals.var_fn97_calc_iq__ffd0);
        let assign9650_e10667: f64 = (locals.var_fn97_calc_iq__vtof - assign9650_e10666);
        let assign9650_e10668: f64 = (assign9650_e10658 - assign9650_e10667);
        let assign9650_e10670: f64 = (assign9650_e10668 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign9650_e10670, (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vdx0_dn2) - (-(assign9650_e10664 * locals.var_fn97_calc_iq__ffd0_dn2))) / locals.var_fn97_calc_iq__two_n_phit0), (((((-locals.var_fn97_calc_iq__vdx0_dn4) - (locals.var_fn97_calc_iq__vtof_dn4 - (((assign9650_e10662 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ffd0) + (assign9650_e10664 * locals.var_fn97_calc_iq__ffd0_dn4)))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign9650_e10668 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vdx0_dn7) - (-(assign9650_e10664 * locals.var_fn97_calc_iq__ffd0_dn7))) / locals.var_fn97_calc_iq__two_n_phit0), (((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vdx0_dn14) - (-(assign9650_e10664 * locals.var_fn97_calc_iq__ffd0_dn14))) / locals.var_fn97_calc_iq__two_n_phit0), (((-locals.var_fn97_calc_iq__vdx0_dn15) - (-(assign9650_e10664 * locals.var_fn97_calc_iq__ffd0_dn15))) / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__etad0, locals.var_fn97_calc_iq__etad0_dn2, locals.var_fn97_calc_iq__etad0_dn4, locals.var_fn97_calc_iq__etad0_dn7, locals.var_fn97_calc_iq__etad0_dn14, locals.var_fn97_calc_iq__etad0_dn15,)
    }
};
        locals.var_fn97_calc_iq__etad0 = assign9650_e10672;
        locals.var_fn97_calc_iq__etad0_dn2 = assign9650_e10672_d_n2;
        locals.var_fn97_calc_iq__etad0_dn4 = assign9650_e10672_d_n4;
        locals.var_fn97_calc_iq__etad0_dn7 = assign9650_e10672_d_n7;
        locals.var_fn97_calc_iq__etad0_dn14 = assign9650_e10672_d_n14;
        locals.var_fn97_calc_iq__etad0_dn15 = assign9650_e10672_d_n15;

        let assign9660_e10675: f64 = if locals.var_fn97_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign9660_e10675;

        let (assign9670_e10683, assign9670_e10683_d_n2, assign9670_e10683_d_n4, assign9670_e10683_d_n7, assign9670_e10683_d_n14, assign9670_e10683_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard121 != 0.0)) {
        let assign9670_e10681: f64 = (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0);
        (assign9670_e10681, (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0_dn2), ((locals.var_fn97_calc_iq__qref0_dn4 * locals.var_fn97_calc_iq__etad0) + (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0_dn4)), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0_dn7), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0_dn14), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0_dn15),)
    } else {
        (locals.var_fn97_calc_iq__qinvd0, locals.var_fn97_calc_iq__qinvd0_dn2, locals.var_fn97_calc_iq__qinvd0_dn4, locals.var_fn97_calc_iq__qinvd0_dn7, locals.var_fn97_calc_iq__qinvd0_dn14, locals.var_fn97_calc_iq__qinvd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd0 = assign9670_e10683;
        locals.var_fn97_calc_iq__qinvd0_dn2 = assign9670_e10683_d_n2;
        locals.var_fn97_calc_iq__qinvd0_dn4 = assign9670_e10683_d_n4;
        locals.var_fn97_calc_iq__qinvd0_dn7 = assign9670_e10683_d_n7;
        locals.var_fn97_calc_iq__qinvd0_dn14 = assign9670_e10683_d_n14;
        locals.var_fn97_calc_iq__qinvd0_dn15 = assign9670_e10683_d_n15;

        let assign9680_e10686: f64 = (-50.0);
        let assign9680_e10687: f64 = if locals.var_fn97_calc_iq__etad0 < assign9680_e10686 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign9680_e10687;

        let (assign9690_e10699, assign9690_e10699_d_n2, assign9690_e10699_d_n4, assign9690_e10699_d_n7, assign9690_e10699_d_n14, assign9690_e10699_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard121 == 0.0)) && (locals.var_guard122 != 0.0)) {
        let assign9690_e10696: f64 = (locals.var_fn97_calc_iq__etad0).exp();
        let assign9690_e10697: f64 = (locals.var_fn97_calc_iq__qref0 * assign9690_e10696);
        (assign9690_e10697, (locals.var_fn97_calc_iq__qref0 * (assign9690_e10696 * locals.var_fn97_calc_iq__etad0_dn2)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9690_e10696) + (locals.var_fn97_calc_iq__qref0 * (assign9690_e10696 * locals.var_fn97_calc_iq__etad0_dn4))), (locals.var_fn97_calc_iq__qref0 * (assign9690_e10696 * locals.var_fn97_calc_iq__etad0_dn7)), (locals.var_fn97_calc_iq__qref0 * (assign9690_e10696 * locals.var_fn97_calc_iq__etad0_dn14)), (locals.var_fn97_calc_iq__qref0 * (assign9690_e10696 * locals.var_fn97_calc_iq__etad0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvd0, locals.var_fn97_calc_iq__qinvd0_dn2, locals.var_fn97_calc_iq__qinvd0_dn4, locals.var_fn97_calc_iq__qinvd0_dn7, locals.var_fn97_calc_iq__qinvd0_dn14, locals.var_fn97_calc_iq__qinvd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd0 = assign9690_e10699;
        locals.var_fn97_calc_iq__qinvd0_dn2 = assign9690_e10699_d_n2;
        locals.var_fn97_calc_iq__qinvd0_dn4 = assign9690_e10699_d_n4;
        locals.var_fn97_calc_iq__qinvd0_dn7 = assign9690_e10699_d_n7;
        locals.var_fn97_calc_iq__qinvd0_dn14 = assign9690_e10699_d_n14;
        locals.var_fn97_calc_iq__qinvd0_dn15 = assign9690_e10699_d_n15;

        let (assign9700_e10715, assign9700_e10715_d_n2, assign9700_e10715_d_n4, assign9700_e10715_d_n7, assign9700_e10715_d_n14, assign9700_e10715_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard121 == 0.0)) && (locals.var_guard122 == 0.0)) {
        let assign9700_e10710: f64 = (locals.var_fn97_calc_iq__etad0).exp();
        let assign9700_e10711: f64 = (1.0 + assign9700_e10710);
        let assign9700_e10712: f64 = (assign9700_e10711).ln();
        let assign9700_e10713: f64 = (locals.var_fn97_calc_iq__qref0 * assign9700_e10712);
        (assign9700_e10713, (locals.var_fn97_calc_iq__qref0 * ((assign9700_e10710 * locals.var_fn97_calc_iq__etad0_dn2) / assign9700_e10711)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9700_e10712) + (locals.var_fn97_calc_iq__qref0 * ((assign9700_e10710 * locals.var_fn97_calc_iq__etad0_dn4) / assign9700_e10711))), (locals.var_fn97_calc_iq__qref0 * ((assign9700_e10710 * locals.var_fn97_calc_iq__etad0_dn7) / assign9700_e10711)), (locals.var_fn97_calc_iq__qref0 * ((assign9700_e10710 * locals.var_fn97_calc_iq__etad0_dn14) / assign9700_e10711)), (locals.var_fn97_calc_iq__qref0 * ((assign9700_e10710 * locals.var_fn97_calc_iq__etad0_dn15) / assign9700_e10711)),)
    } else {
        (locals.var_fn97_calc_iq__qinvd0, locals.var_fn97_calc_iq__qinvd0_dn2, locals.var_fn97_calc_iq__qinvd0_dn4, locals.var_fn97_calc_iq__qinvd0_dn7, locals.var_fn97_calc_iq__qinvd0_dn14, locals.var_fn97_calc_iq__qinvd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd0 = assign9700_e10715;
        locals.var_fn97_calc_iq__qinvd0_dn2 = assign9700_e10715_d_n2;
        locals.var_fn97_calc_iq__qinvd0_dn4 = assign9700_e10715_d_n4;
        locals.var_fn97_calc_iq__qinvd0_dn7 = assign9700_e10715_d_n7;
        locals.var_fn97_calc_iq__qinvd0_dn14 = assign9700_e10715_d_n14;
        locals.var_fn97_calc_iq__qinvd0_dn15 = assign9700_e10715_d_n15;

        let (assign9710_e10723, assign9710_e10723_d_n2, assign9710_e10723_d_n4, assign9710_e10723_d_n7, assign9710_e10723_d_n14, assign9710_e10723_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9710_e10719: f64 = (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0);
        let assign9710_e10721: f64 = (assign9710_e10719 + 1e-38);
        (assign9710_e10721, ((locals.var_fn97_calc_iq__qinvs0_dn2 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0_dn2)), ((locals.var_fn97_calc_iq__qinvs0_dn4 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0_dn4)), ((locals.var_fn97_calc_iq__qinvs0_dn7 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0_dn7)), ((locals.var_fn97_calc_iq__qinvs0_dn14 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0_dn14)), ((locals.var_fn97_calc_iq__qinvs0_dn15 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qs2, locals.var_fn97_calc_iq__qs2_dn2, locals.var_fn97_calc_iq__qs2_dn4, locals.var_fn97_calc_iq__qs2_dn7, locals.var_fn97_calc_iq__qs2_dn14, locals.var_fn97_calc_iq__qs2_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs2 = assign9710_e10723;
        locals.var_fn97_calc_iq__qs2_dn2 = assign9710_e10723_d_n2;
        locals.var_fn97_calc_iq__qs2_dn4 = assign9710_e10723_d_n4;
        locals.var_fn97_calc_iq__qs2_dn7 = assign9710_e10723_d_n7;
        locals.var_fn97_calc_iq__qs2_dn14 = assign9710_e10723_d_n14;
        locals.var_fn97_calc_iq__qs2_dn15 = assign9710_e10723_d_n15;

        let (assign9720_e10731, assign9720_e10731_d_n2, assign9720_e10731_d_n4, assign9720_e10731_d_n7, assign9720_e10731_d_n14, assign9720_e10731_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9720_e10727: f64 = (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0);
        let assign9720_e10729: f64 = (assign9720_e10727 + 1e-57);
        (assign9720_e10729, ((locals.var_fn97_calc_iq__qs2_dn2 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0_dn2)), ((locals.var_fn97_calc_iq__qs2_dn4 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0_dn4)), ((locals.var_fn97_calc_iq__qs2_dn7 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0_dn7)), ((locals.var_fn97_calc_iq__qs2_dn14 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0_dn14)), ((locals.var_fn97_calc_iq__qs2_dn15 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qs3, locals.var_fn97_calc_iq__qs3_dn2, locals.var_fn97_calc_iq__qs3_dn4, locals.var_fn97_calc_iq__qs3_dn7, locals.var_fn97_calc_iq__qs3_dn14, locals.var_fn97_calc_iq__qs3_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs3 = assign9720_e10731;
        locals.var_fn97_calc_iq__qs3_dn2 = assign9720_e10731_d_n2;
        locals.var_fn97_calc_iq__qs3_dn4 = assign9720_e10731_d_n4;
        locals.var_fn97_calc_iq__qs3_dn7 = assign9720_e10731_d_n7;
        locals.var_fn97_calc_iq__qs3_dn14 = assign9720_e10731_d_n14;
        locals.var_fn97_calc_iq__qs3_dn15 = assign9720_e10731_d_n15;

        let (assign9730_e10739, assign9730_e10739_d_n2, assign9730_e10739_d_n4, assign9730_e10739_d_n7, assign9730_e10739_d_n14, assign9730_e10739_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9730_e10735: f64 = (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0);
        let assign9730_e10737: f64 = (assign9730_e10735 + 1e-38);
        (assign9730_e10737, ((locals.var_fn97_calc_iq__qinvd0_dn2 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0_dn2)), ((locals.var_fn97_calc_iq__qinvd0_dn4 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0_dn4)), ((locals.var_fn97_calc_iq__qinvd0_dn7 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0_dn7)), ((locals.var_fn97_calc_iq__qinvd0_dn14 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0_dn14)), ((locals.var_fn97_calc_iq__qinvd0_dn15 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qd2, locals.var_fn97_calc_iq__qd2_dn2, locals.var_fn97_calc_iq__qd2_dn4, locals.var_fn97_calc_iq__qd2_dn7, locals.var_fn97_calc_iq__qd2_dn14, locals.var_fn97_calc_iq__qd2_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd2 = assign9730_e10739;
        locals.var_fn97_calc_iq__qd2_dn2 = assign9730_e10739_d_n2;
        locals.var_fn97_calc_iq__qd2_dn4 = assign9730_e10739_d_n4;
        locals.var_fn97_calc_iq__qd2_dn7 = assign9730_e10739_d_n7;
        locals.var_fn97_calc_iq__qd2_dn14 = assign9730_e10739_d_n14;
        locals.var_fn97_calc_iq__qd2_dn15 = assign9730_e10739_d_n15;

        let (assign9740_e10747, assign9740_e10747_d_n2, assign9740_e10747_d_n4, assign9740_e10747_d_n7, assign9740_e10747_d_n14, assign9740_e10747_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9740_e10743: f64 = (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0);
        let assign9740_e10745: f64 = (assign9740_e10743 + 1e-57);
        (assign9740_e10745, ((locals.var_fn97_calc_iq__qd2_dn2 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0_dn2)), ((locals.var_fn97_calc_iq__qd2_dn4 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0_dn4)), ((locals.var_fn97_calc_iq__qd2_dn7 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0_dn7)), ((locals.var_fn97_calc_iq__qd2_dn14 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0_dn14)), ((locals.var_fn97_calc_iq__qd2_dn15 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qd3, locals.var_fn97_calc_iq__qd3_dn2, locals.var_fn97_calc_iq__qd3_dn4, locals.var_fn97_calc_iq__qd3_dn7, locals.var_fn97_calc_iq__qd3_dn14, locals.var_fn97_calc_iq__qd3_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd3 = assign9740_e10747;
        locals.var_fn97_calc_iq__qd3_dn2 = assign9740_e10747_d_n2;
        locals.var_fn97_calc_iq__qd3_dn4 = assign9740_e10747_d_n4;
        locals.var_fn97_calc_iq__qd3_dn7 = assign9740_e10747_d_n7;
        locals.var_fn97_calc_iq__qd3_dn14 = assign9740_e10747_d_n14;
        locals.var_fn97_calc_iq__qd3_dn15 = assign9740_e10747_d_n15;

        let (assign9750_e10755, assign9750_e10755_d_n2, assign9750_e10755_d_n4, assign9750_e10755_d_n7, assign9750_e10755_d_n14, assign9750_e10755_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9750_e10751: f64 = (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0);
        let assign9750_e10753: f64 = (assign9750_e10751 + 1e-38);
        (assign9750_e10753, ((locals.var_fn97_calc_iq__qinvs0_dn2 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0_dn2)), ((locals.var_fn97_calc_iq__qinvs0_dn4 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0_dn4)), ((locals.var_fn97_calc_iq__qinvs0_dn7 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0_dn7)), ((locals.var_fn97_calc_iq__qinvs0_dn14 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0_dn14)), ((locals.var_fn97_calc_iq__qinvs0_dn15 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qsqd, locals.var_fn97_calc_iq__qsqd_dn2, locals.var_fn97_calc_iq__qsqd_dn4, locals.var_fn97_calc_iq__qsqd_dn7, locals.var_fn97_calc_iq__qsqd_dn14, locals.var_fn97_calc_iq__qsqd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qsqd = assign9750_e10755;
        locals.var_fn97_calc_iq__qsqd_dn2 = assign9750_e10755_d_n2;
        locals.var_fn97_calc_iq__qsqd_dn4 = assign9750_e10755_d_n4;
        locals.var_fn97_calc_iq__qsqd_dn7 = assign9750_e10755_d_n7;
        locals.var_fn97_calc_iq__qsqd_dn14 = assign9750_e10755_d_n14;
        locals.var_fn97_calc_iq__qsqd_dn15 = assign9750_e10755_d_n15;

        let (assign9760_e10773, assign9760_e10773_d_n2, assign9760_e10773_d_n4, assign9760_e10773_d_n7, assign9760_e10773_d_n14, assign9760_e10773_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9760_e10759: f64 = (2.0 / 3.0);
        let assign9760_e10762: f64 = (locals.var_fn97_calc_iq__qs2 + locals.var_fn97_calc_iq__qd2);
        let assign9760_e10764: f64 = (assign9760_e10762 + locals.var_fn97_calc_iq__qsqd);
        let assign9760_e10765: f64 = (assign9760_e10759 * assign9760_e10764);
        let assign9760_e10768: f64 = (locals.var_fn97_calc_iq__qinvs0 + locals.var_fn97_calc_iq__qinvd0);
        let assign9760_e10770: f64 = (assign9760_e10768 + 2e-19);
        let assign9760_e10771: f64 = (assign9760_e10765 / assign9760_e10770);
        (assign9760_e10771, ((((assign9760_e10759 * ((locals.var_fn97_calc_iq__qs2_dn2 + locals.var_fn97_calc_iq__qd2_dn2) + locals.var_fn97_calc_iq__qsqd_dn2)) * assign9760_e10770) - (assign9760_e10765 * (locals.var_fn97_calc_iq__qinvs0_dn2 + locals.var_fn97_calc_iq__qinvd0_dn2))) / (assign9760_e10770 * assign9760_e10770)), ((((assign9760_e10759 * ((locals.var_fn97_calc_iq__qs2_dn4 + locals.var_fn97_calc_iq__qd2_dn4) + locals.var_fn97_calc_iq__qsqd_dn4)) * assign9760_e10770) - (assign9760_e10765 * (locals.var_fn97_calc_iq__qinvs0_dn4 + locals.var_fn97_calc_iq__qinvd0_dn4))) / (assign9760_e10770 * assign9760_e10770)), ((((assign9760_e10759 * ((locals.var_fn97_calc_iq__qs2_dn7 + locals.var_fn97_calc_iq__qd2_dn7) + locals.var_fn97_calc_iq__qsqd_dn7)) * assign9760_e10770) - (assign9760_e10765 * (locals.var_fn97_calc_iq__qinvs0_dn7 + locals.var_fn97_calc_iq__qinvd0_dn7))) / (assign9760_e10770 * assign9760_e10770)), ((((assign9760_e10759 * ((locals.var_fn97_calc_iq__qs2_dn14 + locals.var_fn97_calc_iq__qd2_dn14) + locals.var_fn97_calc_iq__qsqd_dn14)) * assign9760_e10770) - (assign9760_e10765 * (locals.var_fn97_calc_iq__qinvs0_dn14 + locals.var_fn97_calc_iq__qinvd0_dn14))) / (assign9760_e10770 * assign9760_e10770)), ((((assign9760_e10759 * ((locals.var_fn97_calc_iq__qs2_dn15 + locals.var_fn97_calc_iq__qd2_dn15) + locals.var_fn97_calc_iq__qsqd_dn15)) * assign9760_e10770) - (assign9760_e10765 * (locals.var_fn97_calc_iq__qinvs0_dn15 + locals.var_fn97_calc_iq__qinvd0_dn15))) / (assign9760_e10770 * assign9760_e10770)),)
    } else {
        (locals.var_fn97_calc_iq__qinvdd, locals.var_fn97_calc_iq__qinvdd_dn2, locals.var_fn97_calc_iq__qinvdd_dn4, locals.var_fn97_calc_iq__qinvdd_dn7, locals.var_fn97_calc_iq__qinvdd_dn14, locals.var_fn97_calc_iq__qinvdd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvdd = assign9760_e10773;
        locals.var_fn97_calc_iq__qinvdd_dn2 = assign9760_e10773_d_n2;
        locals.var_fn97_calc_iq__qinvdd_dn4 = assign9760_e10773_d_n4;
        locals.var_fn97_calc_iq__qinvdd_dn7 = assign9760_e10773_d_n7;
        locals.var_fn97_calc_iq__qinvdd_dn14 = assign9760_e10773_d_n14;
        locals.var_fn97_calc_iq__qinvdd_dn15 = assign9760_e10773_d_n15;

        let (assign9770_e10807, assign9770_e10807_d_n2, assign9770_e10807_d_n4, assign9770_e10807_d_n7, assign9770_e10807_d_n14, assign9770_e10807_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9770_e10778: f64 = (2.0 * locals.var_fn97_calc_iq__qs3);
        let assign9770_e10781: f64 = (3.0 * locals.var_fn97_calc_iq__qd3);
        let assign9770_e10782: f64 = (assign9770_e10778 + assign9770_e10781);
        let assign9770_e10785: f64 = (4.0 * locals.var_fn97_calc_iq__qs2);
        let assign9770_e10787: f64 = (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0);
        let assign9770_e10788: f64 = (assign9770_e10782 + assign9770_e10787);
        let assign9770_e10791: f64 = (6.0 * locals.var_fn97_calc_iq__qd2);
        let assign9770_e10793: f64 = (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0);
        let assign9770_e10794: f64 = (assign9770_e10788 + assign9770_e10793);
        let assign9770_e10795: f64 = (2.0 * assign9770_e10794);
        let assign9770_e10799: f64 = (locals.var_fn97_calc_iq__qs2 + locals.var_fn97_calc_iq__qd2);
        let assign9770_e10802: f64 = (2.0 * locals.var_fn97_calc_iq__qsqd);
        let assign9770_e10803: f64 = (assign9770_e10799 + assign9770_e10802);
        let assign9770_e10804: f64 = (15.0 * assign9770_e10803);
        let assign9770_e10805: f64 = (assign9770_e10795 / assign9770_e10804);
        (assign9770_e10805, ((((2.0 * ((((2.0 * locals.var_fn97_calc_iq__qs3_dn2) + (3.0 * locals.var_fn97_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn97_calc_iq__qs2_dn2) * locals.var_fn97_calc_iq__qinvd0) + (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn97_calc_iq__qd2_dn2) * locals.var_fn97_calc_iq__qinvs0) + (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0_dn2)))) * assign9770_e10804) - (assign9770_e10795 * (15.0 * ((locals.var_fn97_calc_iq__qs2_dn2 + locals.var_fn97_calc_iq__qd2_dn2) + (2.0 * locals.var_fn97_calc_iq__qsqd_dn2))))) / (assign9770_e10804 * assign9770_e10804)), ((((2.0 * ((((2.0 * locals.var_fn97_calc_iq__qs3_dn4) + (3.0 * locals.var_fn97_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn97_calc_iq__qs2_dn4) * locals.var_fn97_calc_iq__qinvd0) + (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn97_calc_iq__qd2_dn4) * locals.var_fn97_calc_iq__qinvs0) + (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0_dn4)))) * assign9770_e10804) - (assign9770_e10795 * (15.0 * ((locals.var_fn97_calc_iq__qs2_dn4 + locals.var_fn97_calc_iq__qd2_dn4) + (2.0 * locals.var_fn97_calc_iq__qsqd_dn4))))) / (assign9770_e10804 * assign9770_e10804)), ((((2.0 * ((((2.0 * locals.var_fn97_calc_iq__qs3_dn7) + (3.0 * locals.var_fn97_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn97_calc_iq__qs2_dn7) * locals.var_fn97_calc_iq__qinvd0) + (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn97_calc_iq__qd2_dn7) * locals.var_fn97_calc_iq__qinvs0) + (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0_dn7)))) * assign9770_e10804) - (assign9770_e10795 * (15.0 * ((locals.var_fn97_calc_iq__qs2_dn7 + locals.var_fn97_calc_iq__qd2_dn7) + (2.0 * locals.var_fn97_calc_iq__qsqd_dn7))))) / (assign9770_e10804 * assign9770_e10804)), ((((2.0 * ((((2.0 * locals.var_fn97_calc_iq__qs3_dn14) + (3.0 * locals.var_fn97_calc_iq__qd3_dn14)) + (((4.0 * locals.var_fn97_calc_iq__qs2_dn14) * locals.var_fn97_calc_iq__qinvd0) + (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0_dn14))) + (((6.0 * locals.var_fn97_calc_iq__qd2_dn14) * locals.var_fn97_calc_iq__qinvs0) + (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0_dn14)))) * assign9770_e10804) - (assign9770_e10795 * (15.0 * ((locals.var_fn97_calc_iq__qs2_dn14 + locals.var_fn97_calc_iq__qd2_dn14) + (2.0 * locals.var_fn97_calc_iq__qsqd_dn14))))) / (assign9770_e10804 * assign9770_e10804)), ((((2.0 * ((((2.0 * locals.var_fn97_calc_iq__qs3_dn15) + (3.0 * locals.var_fn97_calc_iq__qd3_dn15)) + (((4.0 * locals.var_fn97_calc_iq__qs2_dn15) * locals.var_fn97_calc_iq__qinvd0) + (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0_dn15))) + (((6.0 * locals.var_fn97_calc_iq__qd2_dn15) * locals.var_fn97_calc_iq__qinvs0) + (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0_dn15)))) * assign9770_e10804) - (assign9770_e10795 * (15.0 * ((locals.var_fn97_calc_iq__qs2_dn15 + locals.var_fn97_calc_iq__qd2_dn15) + (2.0 * locals.var_fn97_calc_iq__qsqd_dn15))))) / (assign9770_e10804 * assign9770_e10804)),)
    } else {
        (locals.var_fn97_calc_iq__qd1, locals.var_fn97_calc_iq__qd1_dn2, locals.var_fn97_calc_iq__qd1_dn4, locals.var_fn97_calc_iq__qd1_dn7, locals.var_fn97_calc_iq__qd1_dn14, locals.var_fn97_calc_iq__qd1_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd1 = assign9770_e10807;
        locals.var_fn97_calc_iq__qd1_dn2 = assign9770_e10807_d_n2;
        locals.var_fn97_calc_iq__qd1_dn4 = assign9770_e10807_d_n4;
        locals.var_fn97_calc_iq__qd1_dn7 = assign9770_e10807_d_n7;
        locals.var_fn97_calc_iq__qd1_dn14 = assign9770_e10807_d_n14;
        locals.var_fn97_calc_iq__qd1_dn15 = assign9770_e10807_d_n15;

        let (assign9780_e10813, assign9780_e10813_d_n2, assign9780_e10813_d_n4, assign9780_e10813_d_n7, assign9780_e10813_d_n14, assign9780_e10813_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9780_e10811: f64 = (locals.var_fn97_calc_iq__qinvdd - locals.var_fn97_calc_iq__qd1);
        (assign9780_e10811, (locals.var_fn97_calc_iq__qinvdd_dn2 - locals.var_fn97_calc_iq__qd1_dn2), (locals.var_fn97_calc_iq__qinvdd_dn4 - locals.var_fn97_calc_iq__qd1_dn4), (locals.var_fn97_calc_iq__qinvdd_dn7 - locals.var_fn97_calc_iq__qd1_dn7), (locals.var_fn97_calc_iq__qinvdd_dn14 - locals.var_fn97_calc_iq__qd1_dn14), (locals.var_fn97_calc_iq__qinvdd_dn15 - locals.var_fn97_calc_iq__qd1_dn15),)
    } else {
        (locals.var_fn97_calc_iq__qs, locals.var_fn97_calc_iq__qs_dn2, locals.var_fn97_calc_iq__qs_dn4, locals.var_fn97_calc_iq__qs_dn7, locals.var_fn97_calc_iq__qs_dn14, locals.var_fn97_calc_iq__qs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs = assign9780_e10813;
        locals.var_fn97_calc_iq__qs_dn2 = assign9780_e10813_d_n2;
        locals.var_fn97_calc_iq__qs_dn4 = assign9780_e10813_d_n4;
        locals.var_fn97_calc_iq__qs_dn7 = assign9780_e10813_d_n7;
        locals.var_fn97_calc_iq__qs_dn14 = assign9780_e10813_d_n14;
        locals.var_fn97_calc_iq__qs_dn15 = assign9780_e10813_d_n15;

        let (assign9790_e10817, assign9790_e10817_d_n2, assign9790_e10817_d_n4, assign9790_e10817_d_n7, assign9790_e10817_d_n14, assign9790_e10817_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qd1, locals.var_fn97_calc_iq__qd1_dn2, locals.var_fn97_calc_iq__qd1_dn4, locals.var_fn97_calc_iq__qd1_dn7, locals.var_fn97_calc_iq__qd1_dn14, locals.var_fn97_calc_iq__qd1_dn15,)
    } else {
        (locals.var_fn97_calc_iq__qd, locals.var_fn97_calc_iq__qd_dn2, locals.var_fn97_calc_iq__qd_dn4, locals.var_fn97_calc_iq__qd_dn7, locals.var_fn97_calc_iq__qd_dn14, locals.var_fn97_calc_iq__qd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd = assign9790_e10817;
        locals.var_fn97_calc_iq__qd_dn2 = assign9790_e10817_d_n2;
        locals.var_fn97_calc_iq__qd_dn4 = assign9790_e10817_d_n4;
        locals.var_fn97_calc_iq__qd_dn7 = assign9790_e10817_d_n7;
        locals.var_fn97_calc_iq__qd_dn14 = assign9790_e10817_d_n14;
        locals.var_fn97_calc_iq__qd_dn15 = assign9790_e10817_d_n15;

        let (assign9800_e10831, assign9800_e10831_d_n2, assign9800_e10831_d_n4, assign9800_e10831_d_n7, assign9800_e10831_d_n14, assign9800_e10831_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9800_e10821: f64 = (locals.var_fn97_calc_iq__w * locals.var_fn97_calc_iq__ngf);
        let assign9800_e10823: f64 = (assign9800_e10821 * locals.var_fn97_calc_iq__lin);
        let assign9800_e10825: f64 = (assign9800_e10823 * locals.var_fn97_calc_iq__type);
        let assign9800_e10827: f64 = (assign9800_e10825 * locals.var_fn97_calc_iq__qs);
        let assign9800_e10829: f64 = (assign9800_e10827 * locals.var_fn97_calc_iq__trapfracdl);
        (assign9800_e10829, ((assign9800_e10825 * locals.var_fn97_calc_iq__qs_dn2) * locals.var_fn97_calc_iq__trapfracdl), ((assign9800_e10825 * locals.var_fn97_calc_iq__qs_dn4) * locals.var_fn97_calc_iq__trapfracdl), ((assign9800_e10825 * locals.var_fn97_calc_iq__qs_dn7) * locals.var_fn97_calc_iq__trapfracdl), ((assign9800_e10825 * locals.var_fn97_calc_iq__qs_dn14) * locals.var_fn97_calc_iq__trapfracdl), ((assign9800_e10825 * locals.var_fn97_calc_iq__qs_dn15) * locals.var_fn97_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn97_calc_iq__qgsout, locals.var_fn97_calc_iq__qgsout_dn2, locals.var_fn97_calc_iq__qgsout_dn4, locals.var_fn97_calc_iq__qgsout_dn7, locals.var_fn97_calc_iq__qgsout_dn14, locals.var_fn97_calc_iq__qgsout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qgsout = assign9800_e10831;
        locals.var_fn97_calc_iq__qgsout_dn2 = assign9800_e10831_d_n2;
        locals.var_fn97_calc_iq__qgsout_dn4 = assign9800_e10831_d_n4;
        locals.var_fn97_calc_iq__qgsout_dn7 = assign9800_e10831_d_n7;
        locals.var_fn97_calc_iq__qgsout_dn14 = assign9800_e10831_d_n14;
        locals.var_fn97_calc_iq__qgsout_dn15 = assign9800_e10831_d_n15;

        let (assign9810_e10845, assign9810_e10845_d_n2, assign9810_e10845_d_n4, assign9810_e10845_d_n7, assign9810_e10845_d_n14, assign9810_e10845_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9810_e10835: f64 = (locals.var_fn97_calc_iq__w * locals.var_fn97_calc_iq__ngf);
        let assign9810_e10837: f64 = (assign9810_e10835 * locals.var_fn97_calc_iq__lin);
        let assign9810_e10839: f64 = (assign9810_e10837 * locals.var_fn97_calc_iq__type);
        let assign9810_e10841: f64 = (assign9810_e10839 * locals.var_fn97_calc_iq__qd);
        let assign9810_e10843: f64 = (assign9810_e10841 * locals.var_fn97_calc_iq__trapfracdl);
        (assign9810_e10843, ((assign9810_e10839 * locals.var_fn97_calc_iq__qd_dn2) * locals.var_fn97_calc_iq__trapfracdl), ((assign9810_e10839 * locals.var_fn97_calc_iq__qd_dn4) * locals.var_fn97_calc_iq__trapfracdl), ((assign9810_e10839 * locals.var_fn97_calc_iq__qd_dn7) * locals.var_fn97_calc_iq__trapfracdl), ((assign9810_e10839 * locals.var_fn97_calc_iq__qd_dn14) * locals.var_fn97_calc_iq__trapfracdl), ((assign9810_e10839 * locals.var_fn97_calc_iq__qd_dn15) * locals.var_fn97_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn97_calc_iq__qgdout, locals.var_fn97_calc_iq__qgdout_dn2, locals.var_fn97_calc_iq__qgdout_dn4, locals.var_fn97_calc_iq__qgdout_dn7, locals.var_fn97_calc_iq__qgdout_dn14, locals.var_fn97_calc_iq__qgdout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qgdout = assign9810_e10845;
        locals.var_fn97_calc_iq__qgdout_dn2 = assign9810_e10845_d_n2;
        locals.var_fn97_calc_iq__qgdout_dn4 = assign9810_e10845_d_n4;
        locals.var_fn97_calc_iq__qgdout_dn7 = assign9810_e10845_d_n7;
        locals.var_fn97_calc_iq__qgdout_dn14 = assign9810_e10845_d_n14;
        locals.var_fn97_calc_iq__qgdout_dn15 = assign9810_e10845_d_n15;

        let assign9820_e10848: f64 = if locals.var_fn97_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign9820_e10848;

        let (assign9830_e10864, assign9830_e10864_d_n2, assign9830_e10864_d_n4, assign9830_e10864_d_n7, assign9830_e10864_d_n14,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) {
        let assign9830_e10856: f64 = (p.p51 * 0.5);
        let assign9830_e10858: f64 = (assign9830_e10856 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9830_e10859: f64 = (locals.var_fn97_calc_iq__vtof - assign9830_e10858);
        let assign9830_e10860: f64 = (locals.var_fn97_calc_iq__vcin - assign9830_e10859);
        let assign9830_e10862: f64 = (assign9830_e10860 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign9830_e10862, (locals.var_fn97_calc_iq__vcin_dn2 / locals.var_fn97_calc_iq__two_n_phit0), ((((-(locals.var_fn97_calc_iq__vtof_dn4 - (assign9830_e10856 * locals.var_fn97_calc_iq__alpha_phit_dn4))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign9830_e10860 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), (locals.var_fn97_calc_iq__vcin_dn7 / locals.var_fn97_calc_iq__two_n_phit0), (locals.var_fn97_calc_iq__vcin_dn14 / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__etac, locals.var_fn97_calc_iq__etac_dn2, locals.var_fn97_calc_iq__etac_dn4, locals.var_fn97_calc_iq__etac_dn7, locals.var_fn97_calc_iq__etac_dn14,)
    }
};
        locals.var_fn97_calc_iq__etac = assign9830_e10864;
        locals.var_fn97_calc_iq__etac_dn2 = assign9830_e10864_d_n2;
        locals.var_fn97_calc_iq__etac_dn4 = assign9830_e10864_d_n4;
        locals.var_fn97_calc_iq__etac_dn7 = assign9830_e10864_d_n7;
        locals.var_fn97_calc_iq__etac_dn14 = assign9830_e10864_d_n14;

        let assign9840_e10867: f64 = if locals.var_fn97_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign9840_e10867;

        let (assign9850_e10875, assign9850_e10875_d_n2, assign9850_e10875_d_n3, assign9850_e10875_d_n4, assign9850_e10875_d_n7, assign9850_e10875_d_n14, assign9850_e10875_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard124 != 0.0)) {
        (locals.var_fn97_calc_iq__etac, locals.var_fn97_calc_iq__etac_dn2, 0.0, locals.var_fn97_calc_iq__etac_dn4, locals.var_fn97_calc_iq__etac_dn7, locals.var_fn97_calc_iq__etac_dn14, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9850_e10875;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9850_e10875_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9850_e10875_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9850_e10875_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9850_e10875_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9850_e10875_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9850_e10875_d_n15;

        let assign9860_e10878: f64 = (-50.0);
        let assign9860_e10879: f64 = if locals.var_fn97_calc_iq__etac < assign9860_e10878 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign9860_e10879;

        let (assign9870_e10891, assign9870_e10891_d_n2, assign9870_e10891_d_n3, assign9870_e10891_d_n4, assign9870_e10891_d_n7, assign9870_e10891_d_n14, assign9870_e10891_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard124 == 0.0)) && (locals.var_guard125 != 0.0)) {
        let assign9870_e10889: f64 = (locals.var_fn97_calc_iq__etac).exp();
        (assign9870_e10889, (assign9870_e10889 * locals.var_fn97_calc_iq__etac_dn2), 0.0, (assign9870_e10889 * locals.var_fn97_calc_iq__etac_dn4), (assign9870_e10889 * locals.var_fn97_calc_iq__etac_dn7), (assign9870_e10889 * locals.var_fn97_calc_iq__etac_dn14), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9870_e10891;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9870_e10891_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9870_e10891_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9870_e10891_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9870_e10891_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9870_e10891_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9870_e10891_d_n15;

        let (assign9880_e10907, assign9880_e10907_d_n2, assign9880_e10907_d_n3, assign9880_e10907_d_n4, assign9880_e10907_d_n7, assign9880_e10907_d_n14, assign9880_e10907_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard124 == 0.0)) && (locals.var_guard125 == 0.0)) {
        let assign9880_e10903: f64 = (locals.var_fn97_calc_iq__etac).exp();
        let assign9880_e10904: f64 = (1.0 + assign9880_e10903);
        let assign9880_e10905: f64 = (assign9880_e10904).ln();
        (assign9880_e10905, ((assign9880_e10903 * locals.var_fn97_calc_iq__etac_dn2) / assign9880_e10904), 0.0, ((assign9880_e10903 * locals.var_fn97_calc_iq__etac_dn4) / assign9880_e10904), ((assign9880_e10903 * locals.var_fn97_calc_iq__etac_dn7) / assign9880_e10904), ((assign9880_e10903 * locals.var_fn97_calc_iq__etac_dn14) / assign9880_e10904), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9880_e10907;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9880_e10907_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9880_e10907_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9880_e10907_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9880_e10907_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9880_e10907_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9880_e10907_d_n15;

    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9890_e10925, assign9890_e10925_d_n2, assign9890_e10925_d_n3, assign9890_e10925_d_n4, assign9890_e10925_d_n7, assign9890_e10925_d_n14, assign9890_e10925_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) {
        let assign9890_e10913: f64 = (locals.var_fn97_calc_iq__w * locals.var_fn97_calc_iq__ngf);
        let assign9890_e10915: f64 = (assign9890_e10913 * locals.var_fn97_calc_iq__type);
        let assign9890_e10917: f64 = (assign9890_e10915 * locals.var_fn97_calc_iq__cc);
        let assign9890_e10919: f64 = (assign9890_e10917 * locals.var_fn97_calc_iq__two_n_phit0);
        let assign9890_e10921: f64 = (assign9890_e10919 * locals.var_fn97_calc_iq__exparg);
        let assign9890_e10923: f64 = (assign9890_e10921 * locals.var_fn97_calc_iq__trapfracdl);
        (assign9890_e10923, ((assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn2) * locals.var_fn97_calc_iq__trapfracdl), ((assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn3) * locals.var_fn97_calc_iq__trapfracdl), ((((((assign9890_e10915 * locals.var_fn97_calc_iq__cc_dn4) * locals.var_fn97_calc_iq__two_n_phit0) + (assign9890_e10917 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) * locals.var_fn97_calc_iq__exparg) + (assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn4)) * locals.var_fn97_calc_iq__trapfracdl), ((assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn7) * locals.var_fn97_calc_iq__trapfracdl), ((assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn14) * locals.var_fn97_calc_iq__trapfracdl), ((assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn15) * locals.var_fn97_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn97_calc_iq__qcout, locals.var_fn97_calc_iq__qcout_dn2, locals.var_fn97_calc_iq__qcout_dn3, locals.var_fn97_calc_iq__qcout_dn4, locals.var_fn97_calc_iq__qcout_dn7, locals.var_fn97_calc_iq__qcout_dn14, locals.var_fn97_calc_iq__qcout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qcout = assign9890_e10925;
        locals.var_fn97_calc_iq__qcout_dn2 = assign9890_e10925_d_n2;
        locals.var_fn97_calc_iq__qcout_dn3 = assign9890_e10925_d_n3;
        locals.var_fn97_calc_iq__qcout_dn4 = assign9890_e10925_d_n4;
        locals.var_fn97_calc_iq__qcout_dn7 = assign9890_e10925_d_n7;
        locals.var_fn97_calc_iq__qcout_dn14 = assign9890_e10925_d_n14;
        locals.var_fn97_calc_iq__qcout_dn15 = assign9890_e10925_d_n15;

        let (assign9900_e10941, assign9900_e10941_d_n3, assign9900_e10941_d_n4, assign9900_e10941_d_n14,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) {
        let assign9900_e10933: f64 = (p.p51 * 0.5);
        let assign9900_e10935: f64 = (assign9900_e10933 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9900_e10936: f64 = (locals.var_fn97_calc_iq__vtof - assign9900_e10935);
        let assign9900_e10937: f64 = (locals.var_fn97_calc_iq__vbin - assign9900_e10936);
        let assign9900_e10939: f64 = (assign9900_e10937 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign9900_e10939, (locals.var_fn97_calc_iq__vbin_dn3 / locals.var_fn97_calc_iq__two_n_phit0), ((((-(locals.var_fn97_calc_iq__vtof_dn4 - (assign9900_e10933 * locals.var_fn97_calc_iq__alpha_phit_dn4))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign9900_e10937 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), (locals.var_fn97_calc_iq__vbin_dn14 / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__etab, locals.var_fn97_calc_iq__etab_dn3, locals.var_fn97_calc_iq__etab_dn4, locals.var_fn97_calc_iq__etab_dn14,)
    }
};
        locals.var_fn97_calc_iq__etab = assign9900_e10941;
        locals.var_fn97_calc_iq__etab_dn3 = assign9900_e10941_d_n3;
        locals.var_fn97_calc_iq__etab_dn4 = assign9900_e10941_d_n4;
        locals.var_fn97_calc_iq__etab_dn14 = assign9900_e10941_d_n14;

        let assign9910_e10944: f64 = if locals.var_fn97_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign9910_e10944;

        let (assign9920_e10952, assign9920_e10952_d_n2, assign9920_e10952_d_n3, assign9920_e10952_d_n4, assign9920_e10952_d_n7, assign9920_e10952_d_n14, assign9920_e10952_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard126 != 0.0)) {
        (locals.var_fn97_calc_iq__etab, 0.0, locals.var_fn97_calc_iq__etab_dn3, locals.var_fn97_calc_iq__etab_dn4, 0.0, locals.var_fn97_calc_iq__etab_dn14, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9920_e10952;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9920_e10952_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9920_e10952_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9920_e10952_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9920_e10952_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9920_e10952_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9920_e10952_d_n15;

        let assign9930_e10955: f64 = (-50.0);
        let assign9930_e10956: f64 = if locals.var_fn97_calc_iq__etab < assign9930_e10955 { 1.0 } else { 0.0 };
        locals.var_guard127 = assign9930_e10956;

        let (assign9940_e10968, assign9940_e10968_d_n2, assign9940_e10968_d_n3, assign9940_e10968_d_n4, assign9940_e10968_d_n7, assign9940_e10968_d_n14, assign9940_e10968_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard126 == 0.0)) && (locals.var_guard127 != 0.0)) {
        let assign9940_e10966: f64 = (locals.var_fn97_calc_iq__etab).exp();
        (assign9940_e10966, 0.0, (assign9940_e10966 * locals.var_fn97_calc_iq__etab_dn3), (assign9940_e10966 * locals.var_fn97_calc_iq__etab_dn4), 0.0, (assign9940_e10966 * locals.var_fn97_calc_iq__etab_dn14), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9940_e10968;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9940_e10968_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9940_e10968_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9940_e10968_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9940_e10968_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9940_e10968_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9940_e10968_d_n15;

        let (assign9950_e10984, assign9950_e10984_d_n2, assign9950_e10984_d_n3, assign9950_e10984_d_n4, assign9950_e10984_d_n7, assign9950_e10984_d_n14, assign9950_e10984_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard126 == 0.0)) && (locals.var_guard127 == 0.0)) {
        let assign9950_e10980: f64 = (locals.var_fn97_calc_iq__etab).exp();
        let assign9950_e10981: f64 = (1.0 + assign9950_e10980);
        let assign9950_e10982: f64 = (assign9950_e10981).ln();
        (assign9950_e10982, 0.0, ((assign9950_e10980 * locals.var_fn97_calc_iq__etab_dn3) / assign9950_e10981), ((assign9950_e10980 * locals.var_fn97_calc_iq__etab_dn4) / assign9950_e10981), 0.0, ((assign9950_e10980 * locals.var_fn97_calc_iq__etab_dn14) / assign9950_e10981), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9950_e10984;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9950_e10984_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9950_e10984_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9950_e10984_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9950_e10984_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9950_e10984_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9950_e10984_d_n15;

        let (assign9960_e11002, assign9960_e11002_d_n2, assign9960_e11002_d_n3, assign9960_e11002_d_n4, assign9960_e11002_d_n7, assign9960_e11002_d_n14, assign9960_e11002_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) {
        let assign9960_e10990: f64 = (locals.var_fn97_calc_iq__w * locals.var_fn97_calc_iq__ngf);
        let assign9960_e10992: f64 = (assign9960_e10990 * locals.var_fn97_calc_iq__type);
        let assign9960_e10994: f64 = (assign9960_e10992 * locals.var_fn97_calc_iq__cb);
        let assign9960_e10996: f64 = (assign9960_e10994 * locals.var_fn97_calc_iq__two_n_phit0);
        let assign9960_e10998: f64 = (assign9960_e10996 * locals.var_fn97_calc_iq__exparg);
        let assign9960_e11000: f64 = (assign9960_e10998 * locals.var_fn97_calc_iq__trapfracdl);
        (assign9960_e11000, ((assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn2) * locals.var_fn97_calc_iq__trapfracdl), ((assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn3) * locals.var_fn97_calc_iq__trapfracdl), ((((((assign9960_e10992 * locals.var_fn97_calc_iq__cb_dn4) * locals.var_fn97_calc_iq__two_n_phit0) + (assign9960_e10994 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) * locals.var_fn97_calc_iq__exparg) + (assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn4)) * locals.var_fn97_calc_iq__trapfracdl), ((assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn7) * locals.var_fn97_calc_iq__trapfracdl), ((assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn14) * locals.var_fn97_calc_iq__trapfracdl), ((assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn15) * locals.var_fn97_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn97_calc_iq__qbout, locals.var_fn97_calc_iq__qbout_dn2, locals.var_fn97_calc_iq__qbout_dn3, locals.var_fn97_calc_iq__qbout_dn4, locals.var_fn97_calc_iq__qbout_dn7, locals.var_fn97_calc_iq__qbout_dn14, locals.var_fn97_calc_iq__qbout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qbout = assign9960_e11002;
        locals.var_fn97_calc_iq__qbout_dn2 = assign9960_e11002_d_n2;
        locals.var_fn97_calc_iq__qbout_dn3 = assign9960_e11002_d_n3;
        locals.var_fn97_calc_iq__qbout_dn4 = assign9960_e11002_d_n4;
        locals.var_fn97_calc_iq__qbout_dn7 = assign9960_e11002_d_n7;
        locals.var_fn97_calc_iq__qbout_dn14 = assign9960_e11002_d_n14;
        locals.var_fn97_calc_iq__qbout_dn15 = assign9960_e11002_d_n15;

        let (assign9970_e11009, assign9970_e11009_d_n2, assign9970_e11009_d_n3, assign9970_e11009_d_n4, assign9970_e11009_d_n7, assign9970_e11009_d_n14, assign9970_e11009_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qcout, locals.var_fn97_calc_iq__qcout_dn2, locals.var_fn97_calc_iq__qcout_dn3, locals.var_fn97_calc_iq__qcout_dn4, locals.var_fn97_calc_iq__qcout_dn7, locals.var_fn97_calc_iq__qcout_dn14, locals.var_fn97_calc_iq__qcout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qcout = assign9970_e11009;
        locals.var_fn97_calc_iq__qcout_dn2 = assign9970_e11009_d_n2;
        locals.var_fn97_calc_iq__qcout_dn3 = assign9970_e11009_d_n3;
        locals.var_fn97_calc_iq__qcout_dn4 = assign9970_e11009_d_n4;
        locals.var_fn97_calc_iq__qcout_dn7 = assign9970_e11009_d_n7;
        locals.var_fn97_calc_iq__qcout_dn14 = assign9970_e11009_d_n14;
        locals.var_fn97_calc_iq__qcout_dn15 = assign9970_e11009_d_n15;

        let (assign9980_e11016, assign9980_e11016_d_n2, assign9980_e11016_d_n3, assign9980_e11016_d_n4, assign9980_e11016_d_n7, assign9980_e11016_d_n14, assign9980_e11016_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qbout, locals.var_fn97_calc_iq__qbout_dn2, locals.var_fn97_calc_iq__qbout_dn3, locals.var_fn97_calc_iq__qbout_dn4, locals.var_fn97_calc_iq__qbout_dn7, locals.var_fn97_calc_iq__qbout_dn14, locals.var_fn97_calc_iq__qbout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qbout = assign9980_e11016;
        locals.var_fn97_calc_iq__qbout_dn2 = assign9980_e11016_d_n2;
        locals.var_fn97_calc_iq__qbout_dn3 = assign9980_e11016_d_n3;
        locals.var_fn97_calc_iq__qbout_dn4 = assign9980_e11016_d_n4;
        locals.var_fn97_calc_iq__qbout_dn7 = assign9980_e11016_d_n7;
        locals.var_fn97_calc_iq__qbout_dn14 = assign9980_e11016_d_n14;
        locals.var_fn97_calc_iq__qbout_dn15 = assign9980_e11016_d_n15;

        let assign9990_e11019: f64 = if locals.var_fn97_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign9990_e11019;

        let (assign10000_e11035, assign10000_e11035_d_n2, assign10000_e11035_d_n4, assign10000_e11035_d_n7, assign10000_e11035_d_n14,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard128 != 0.0)) {
        let assign10000_e11027: f64 = (p.p51 * 0.5);
        let assign10000_e11029: f64 = (assign10000_e11027 * locals.var_fn97_calc_iq__alpha_phit);
        let assign10000_e11030: f64 = (locals.var_fn97_calc_iq__vtof - assign10000_e11029);
        let assign10000_e11031: f64 = (locals.var_fn97_calc_iq__vgsin - assign10000_e11030);
        let assign10000_e11033: f64 = (assign10000_e11031 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign10000_e11033, (locals.var_fn97_calc_iq__vgsin_dn2 / locals.var_fn97_calc_iq__two_n_phit0), ((((-(locals.var_fn97_calc_iq__vtof_dn4 - (assign10000_e11027 * locals.var_fn97_calc_iq__alpha_phit_dn4))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign10000_e11031 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), (locals.var_fn97_calc_iq__vgsin_dn7 / locals.var_fn97_calc_iq__two_n_phit0), (locals.var_fn97_calc_iq__vgsin_dn14 / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__etags, locals.var_fn97_calc_iq__etags_dn2, locals.var_fn97_calc_iq__etags_dn4, locals.var_fn97_calc_iq__etags_dn7, locals.var_fn97_calc_iq__etags_dn14,)
    }
};
        locals.var_fn97_calc_iq__etags = assign10000_e11035;
        locals.var_fn97_calc_iq__etags_dn2 = assign10000_e11035_d_n2;
        locals.var_fn97_calc_iq__etags_dn4 = assign10000_e11035_d_n4;
        locals.var_fn97_calc_iq__etags_dn7 = assign10000_e11035_d_n7;
        locals.var_fn97_calc_iq__etags_dn14 = assign10000_e11035_d_n14;

        let assign10010_e11038: f64 = if locals.var_fn97_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard129 = assign10010_e11038;

        let (assign10020_e11046, assign10020_e11046_d_n2, assign10020_e11046_d_n3, assign10020_e11046_d_n4, assign10020_e11046_d_n7, assign10020_e11046_d_n14, assign10020_e11046_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 != 0.0)) {
        (locals.var_fn97_calc_iq__etags, locals.var_fn97_calc_iq__etags_dn2, 0.0, locals.var_fn97_calc_iq__etags_dn4, locals.var_fn97_calc_iq__etags_dn7, locals.var_fn97_calc_iq__etags_dn14, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign10020_e11046;
        locals.var_fn97_calc_iq__exparg_dn2 = assign10020_e11046_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign10020_e11046_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign10020_e11046_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign10020_e11046_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign10020_e11046_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign10020_e11046_d_n15;

        let assign10030_e11049: f64 = (-50.0);
        let assign10030_e11050: f64 = if locals.var_fn97_calc_iq__etags < assign10030_e11049 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign10030_e11050;

        let (assign10040_e11062, assign10040_e11062_d_n2, assign10040_e11062_d_n3, assign10040_e11062_d_n4, assign10040_e11062_d_n7, assign10040_e11062_d_n14, assign10040_e11062_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) && (locals.var_guard130 != 0.0)) {
        let assign10040_e11060: f64 = (locals.var_fn97_calc_iq__etags).exp();
        (assign10040_e11060, (assign10040_e11060 * locals.var_fn97_calc_iq__etags_dn2), 0.0, (assign10040_e11060 * locals.var_fn97_calc_iq__etags_dn4), (assign10040_e11060 * locals.var_fn97_calc_iq__etags_dn7), (assign10040_e11060 * locals.var_fn97_calc_iq__etags_dn14), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign10040_e11062;
        locals.var_fn97_calc_iq__exparg_dn2 = assign10040_e11062_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign10040_e11062_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign10040_e11062_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign10040_e11062_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign10040_e11062_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign10040_e11062_d_n15;

        let (assign10050_e11078, assign10050_e11078_d_n2, assign10050_e11078_d_n3, assign10050_e11078_d_n4, assign10050_e11078_d_n7, assign10050_e11078_d_n14, assign10050_e11078_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) && (locals.var_guard130 == 0.0)) {
        let assign10050_e11074: f64 = (locals.var_fn97_calc_iq__etags).exp();
        let assign10050_e11075: f64 = (1.0 + assign10050_e11074);
        let assign10050_e11076: f64 = (assign10050_e11075).ln();
        (assign10050_e11076, ((assign10050_e11074 * locals.var_fn97_calc_iq__etags_dn2) / assign10050_e11075), 0.0, ((assign10050_e11074 * locals.var_fn97_calc_iq__etags_dn4) / assign10050_e11075), ((assign10050_e11074 * locals.var_fn97_calc_iq__etags_dn7) / assign10050_e11075), ((assign10050_e11074 * locals.var_fn97_calc_iq__etags_dn14) / assign10050_e11075), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign10050_e11078;
        locals.var_fn97_calc_iq__exparg_dn2 = assign10050_e11078_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign10050_e11078_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign10050_e11078_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign10050_e11078_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign10050_e11078_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign10050_e11078_d_n15;

        let (assign10060_e11096, assign10060_e11096_d_n2, assign10060_e11096_d_n3, assign10060_e11096_d_n4, assign10060_e11096_d_n7, assign10060_e11096_d_n14, assign10060_e11096_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard128 != 0.0)) {
        let assign10060_e11084: f64 = (locals.var_fn97_calc_iq__w * locals.var_fn97_calc_iq__ngf);
        let assign10060_e11086: f64 = (assign10060_e11084 * locals.var_fn97_calc_iq__type);
        let assign10060_e11088: f64 = (assign10060_e11086 * locals.var_fn97_calc_iq__cs);
        let assign10060_e11090: f64 = (assign10060_e11088 * locals.var_fn97_calc_iq__two_n_phit0);
        let assign10060_e11092: f64 = (assign10060_e11090 * locals.var_fn97_calc_iq__exparg);
        let assign10060_e11094: f64 = (assign10060_e11092 * locals.var_fn97_calc_iq__trapfracdl);
        (assign10060_e11094, ((assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn2) * locals.var_fn97_calc_iq__trapfracdl), ((assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn3) * locals.var_fn97_calc_iq__trapfracdl), ((((assign10060_e11088 * locals.var_fn97_calc_iq__two_n_phit0_dn4) * locals.var_fn97_calc_iq__exparg) + (assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn4)) * locals.var_fn97_calc_iq__trapfracdl), ((assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn7) * locals.var_fn97_calc_iq__trapfracdl), ((assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn14) * locals.var_fn97_calc_iq__trapfracdl), ((assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn15) * locals.var_fn97_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn97_calc_iq__qsout, locals.var_fn97_calc_iq__qsout_dn2, locals.var_fn97_calc_iq__qsout_dn3, locals.var_fn97_calc_iq__qsout_dn4, locals.var_fn97_calc_iq__qsout_dn7, locals.var_fn97_calc_iq__qsout_dn14, locals.var_fn97_calc_iq__qsout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qsout = assign10060_e11096;
        locals.var_fn97_calc_iq__qsout_dn2 = assign10060_e11096_d_n2;
        locals.var_fn97_calc_iq__qsout_dn3 = assign10060_e11096_d_n3;
        locals.var_fn97_calc_iq__qsout_dn4 = assign10060_e11096_d_n4;
        locals.var_fn97_calc_iq__qsout_dn7 = assign10060_e11096_d_n7;
        locals.var_fn97_calc_iq__qsout_dn14 = assign10060_e11096_d_n14;
        locals.var_fn97_calc_iq__qsout_dn15 = assign10060_e11096_d_n15;

        let (assign10070_e11103, assign10070_e11103_d_n2, assign10070_e11103_d_n3, assign10070_e11103_d_n4, assign10070_e11103_d_n7, assign10070_e11103_d_n14, assign10070_e11103_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard128 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qsout, locals.var_fn97_calc_iq__qsout_dn2, locals.var_fn97_calc_iq__qsout_dn3, locals.var_fn97_calc_iq__qsout_dn4, locals.var_fn97_calc_iq__qsout_dn7, locals.var_fn97_calc_iq__qsout_dn14, locals.var_fn97_calc_iq__qsout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qsout = assign10070_e11103;
        locals.var_fn97_calc_iq__qsout_dn2 = assign10070_e11103_d_n2;
        locals.var_fn97_calc_iq__qsout_dn3 = assign10070_e11103_d_n3;
        locals.var_fn97_calc_iq__qsout_dn4 = assign10070_e11103_d_n4;
        locals.var_fn97_calc_iq__qsout_dn7 = assign10070_e11103_d_n7;
        locals.var_fn97_calc_iq__qsout_dn14 = assign10070_e11103_d_n14;
        locals.var_fn97_calc_iq__qsout_dn15 = assign10070_e11103_d_n15;

        let (assign10100_e11115, assign10100_e11115_d_n2, assign10100_e11115_d_n4, assign10100_e11115_d_n7, assign10100_e11115_d_n14, assign10100_e11115_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qgsout, locals.var_fn97_calc_iq__qgsout_dn2, locals.var_fn97_calc_iq__qgsout_dn4, locals.var_fn97_calc_iq__qgsout_dn7, locals.var_fn97_calc_iq__qgsout_dn14, locals.var_fn97_calc_iq__qgsout_dn15,)
    } else {
        (locals.var_qgsfp2, locals.var_qgsfp2_dn2, locals.var_qgsfp2_dn4, locals.var_qgsfp2_dn7, locals.var_qgsfp2_dn14, locals.var_qgsfp2_dn15,)
    }
};
        locals.var_qgsfp2 = assign10100_e11115;
        locals.var_qgsfp2_dn2 = assign10100_e11115_d_n2;
        locals.var_qgsfp2_dn4 = assign10100_e11115_d_n4;
        locals.var_qgsfp2_dn7 = assign10100_e11115_d_n7;
        locals.var_qgsfp2_dn14 = assign10100_e11115_d_n14;
        locals.var_qgsfp2_dn15 = assign10100_e11115_d_n15;

        let (assign10110_e11119, assign10110_e11119_d_n2, assign10110_e11119_d_n4, assign10110_e11119_d_n7, assign10110_e11119_d_n14, assign10110_e11119_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qgdout, locals.var_fn97_calc_iq__qgdout_dn2, locals.var_fn97_calc_iq__qgdout_dn4, locals.var_fn97_calc_iq__qgdout_dn7, locals.var_fn97_calc_iq__qgdout_dn14, locals.var_fn97_calc_iq__qgdout_dn15,)
    } else {
        (locals.var_qgdfp2, locals.var_qgdfp2_dn2, locals.var_qgdfp2_dn4, locals.var_qgdfp2_dn7, locals.var_qgdfp2_dn14, locals.var_qgdfp2_dn15,)
    }
};
        locals.var_qgdfp2 = assign10110_e11119;
        locals.var_qgdfp2_dn2 = assign10110_e11119_d_n2;
        locals.var_qgdfp2_dn4 = assign10110_e11119_d_n4;
        locals.var_qgdfp2_dn7 = assign10110_e11119_d_n7;
        locals.var_qgdfp2_dn14 = assign10110_e11119_d_n14;
        locals.var_qgdfp2_dn15 = assign10110_e11119_d_n15;

        let (assign10120_e11123, assign10120_e11123_d_n2, assign10120_e11123_d_n3, assign10120_e11123_d_n4, assign10120_e11123_d_n7, assign10120_e11123_d_n14, assign10120_e11123_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qcout, locals.var_fn97_calc_iq__qcout_dn2, locals.var_fn97_calc_iq__qcout_dn3, locals.var_fn97_calc_iq__qcout_dn4, locals.var_fn97_calc_iq__qcout_dn7, locals.var_fn97_calc_iq__qcout_dn14, locals.var_fn97_calc_iq__qcout_dn15,)
    } else {
        (locals.var_qcfp2, locals.var_qcfp2_dn2, locals.var_qcfp2_dn3, locals.var_qcfp2_dn4, locals.var_qcfp2_dn7, locals.var_qcfp2_dn14, locals.var_qcfp2_dn15,)
    }
};
        locals.var_qcfp2 = assign10120_e11123;
        locals.var_qcfp2_dn2 = assign10120_e11123_d_n2;
        locals.var_qcfp2_dn3 = assign10120_e11123_d_n3;
        locals.var_qcfp2_dn4 = assign10120_e11123_d_n4;
        locals.var_qcfp2_dn7 = assign10120_e11123_d_n7;
        locals.var_qcfp2_dn14 = assign10120_e11123_d_n14;
        locals.var_qcfp2_dn15 = assign10120_e11123_d_n15;

        let (assign10130_e11127, assign10130_e11127_d_n2, assign10130_e11127_d_n3, assign10130_e11127_d_n4, assign10130_e11127_d_n7, assign10130_e11127_d_n14, assign10130_e11127_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qbout, locals.var_fn97_calc_iq__qbout_dn2, locals.var_fn97_calc_iq__qbout_dn3, locals.var_fn97_calc_iq__qbout_dn4, locals.var_fn97_calc_iq__qbout_dn7, locals.var_fn97_calc_iq__qbout_dn14, locals.var_fn97_calc_iq__qbout_dn15,)
    } else {
        (locals.var_qbfp2, locals.var_qbfp2_dn2, locals.var_qbfp2_dn3, locals.var_qbfp2_dn4, locals.var_qbfp2_dn7, locals.var_qbfp2_dn14, locals.var_qbfp2_dn15,)
    }
};
        locals.var_qbfp2 = assign10130_e11127;
        locals.var_qbfp2_dn2 = assign10130_e11127_d_n2;
        locals.var_qbfp2_dn3 = assign10130_e11127_d_n3;
        locals.var_qbfp2_dn4 = assign10130_e11127_d_n4;
        locals.var_qbfp2_dn7 = assign10130_e11127_d_n7;
        locals.var_qbfp2_dn14 = assign10130_e11127_d_n14;
        locals.var_qbfp2_dn15 = assign10130_e11127_d_n15;

        let (assign10140_e11131, assign10140_e11131_d_n2, assign10140_e11131_d_n3, assign10140_e11131_d_n4, assign10140_e11131_d_n7, assign10140_e11131_d_n14, assign10140_e11131_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qsout, locals.var_fn97_calc_iq__qsout_dn2, locals.var_fn97_calc_iq__qsout_dn3, locals.var_fn97_calc_iq__qsout_dn4, locals.var_fn97_calc_iq__qsout_dn7, locals.var_fn97_calc_iq__qsout_dn14, locals.var_fn97_calc_iq__qsout_dn15,)
    } else {
        (locals.var_qsfp2, locals.var_qsfp2_dn2, locals.var_qsfp2_dn3, locals.var_qsfp2_dn4, locals.var_qsfp2_dn7, locals.var_qsfp2_dn14, locals.var_qsfp2_dn15,)
    }
};
        locals.var_qsfp2 = assign10140_e11131;
        locals.var_qsfp2_dn2 = assign10140_e11131_d_n2;
        locals.var_qsfp2_dn3 = assign10140_e11131_d_n3;
        locals.var_qsfp2_dn4 = assign10140_e11131_d_n4;
        locals.var_qsfp2_dn7 = assign10140_e11131_d_n7;
        locals.var_qsfp2_dn14 = assign10140_e11131_d_n14;
        locals.var_qsfp2_dn15 = assign10140_e11131_d_n15;

        let assign10180_e11146: f64 = if p.p188 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign10180_e11146;

        locals.var_qgsfp1 = 0.0;
        locals.var_qgsfp1_dn2 = 0.0;
        locals.var_qgsfp1_dn4 = 0.0;
        locals.var_qgsfp1_dn5 = 0.0;
        locals.var_qgsfp1_dn7 = 0.0;
        locals.var_qgsfp1_dn14 = 0.0;

        locals.var_qgdfp1 = 0.0;
        locals.var_qgdfp1_dn2 = 0.0;
        locals.var_qgdfp1_dn4 = 0.0;
        locals.var_qgdfp1_dn5 = 0.0;
        locals.var_qgdfp1_dn7 = 0.0;
        locals.var_qgdfp1_dn14 = 0.0;

        locals.var_qcfp1 = 0.0;
        locals.var_qcfp1_dn2 = 0.0;
        locals.var_qcfp1_dn3 = 0.0;
        locals.var_qcfp1_dn4 = 0.0;
        locals.var_qcfp1_dn5 = 0.0;
        locals.var_qcfp1_dn7 = 0.0;
        locals.var_qcfp1_dn14 = 0.0;

        locals.var_qbfp1 = 0.0;
        locals.var_qbfp1_dn2 = 0.0;
        locals.var_qbfp1_dn3 = 0.0;
        locals.var_qbfp1_dn4 = 0.0;
        locals.var_qbfp1_dn5 = 0.0;
        locals.var_qbfp1_dn7 = 0.0;
        locals.var_qbfp1_dn14 = 0.0;

        locals.var_qsfp1 = 0.0;
        locals.var_qsfp1_dn2 = 0.0;
        locals.var_qsfp1_dn3 = 0.0;
        locals.var_qsfp1_dn4 = 0.0;
        locals.var_qsfp1_dn5 = 0.0;
        locals.var_qsfp1_dn7 = 0.0;
        locals.var_qsfp1_dn14 = 0.0;

        let assign10270_e11157: f64 = if p.p167 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard132 = assign10270_e11157;

        let (assign10300_e11169, assign10300_e11169_d_n2, assign10300_e11169_d_n4, assign10300_e11169_d_n5, assign10300_e11169_d_n7, assign10300_e11169_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qgsout, locals.var_fn133_calc_iq__qgsout_dn2, locals.var_fn133_calc_iq__qgsout_dn4, locals.var_fn133_calc_iq__qgsout_dn5, locals.var_fn133_calc_iq__qgsout_dn7, locals.var_fn133_calc_iq__qgsout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qgsout = assign10300_e11169;
        locals.var_fn133_calc_iq__qgsout_dn2 = assign10300_e11169_d_n2;
        locals.var_fn133_calc_iq__qgsout_dn4 = assign10300_e11169_d_n4;
        locals.var_fn133_calc_iq__qgsout_dn5 = assign10300_e11169_d_n5;
        locals.var_fn133_calc_iq__qgsout_dn7 = assign10300_e11169_d_n7;
        locals.var_fn133_calc_iq__qgsout_dn14 = assign10300_e11169_d_n14;

        let (assign10310_e11173, assign10310_e11173_d_n2, assign10310_e11173_d_n4, assign10310_e11173_d_n5, assign10310_e11173_d_n7, assign10310_e11173_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qgdout, locals.var_fn133_calc_iq__qgdout_dn2, locals.var_fn133_calc_iq__qgdout_dn4, locals.var_fn133_calc_iq__qgdout_dn5, locals.var_fn133_calc_iq__qgdout_dn7, locals.var_fn133_calc_iq__qgdout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qgdout = assign10310_e11173;
        locals.var_fn133_calc_iq__qgdout_dn2 = assign10310_e11173_d_n2;
        locals.var_fn133_calc_iq__qgdout_dn4 = assign10310_e11173_d_n4;
        locals.var_fn133_calc_iq__qgdout_dn5 = assign10310_e11173_d_n5;
        locals.var_fn133_calc_iq__qgdout_dn7 = assign10310_e11173_d_n7;
        locals.var_fn133_calc_iq__qgdout_dn14 = assign10310_e11173_d_n14;

        let (assign10320_e11177, assign10320_e11177_d_n2, assign10320_e11177_d_n3, assign10320_e11177_d_n4, assign10320_e11177_d_n5, assign10320_e11177_d_n7, assign10320_e11177_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qcout, locals.var_fn133_calc_iq__qcout_dn2, locals.var_fn133_calc_iq__qcout_dn3, locals.var_fn133_calc_iq__qcout_dn4, locals.var_fn133_calc_iq__qcout_dn5, locals.var_fn133_calc_iq__qcout_dn7, locals.var_fn133_calc_iq__qcout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qcout = assign10320_e11177;
        locals.var_fn133_calc_iq__qcout_dn2 = assign10320_e11177_d_n2;
        locals.var_fn133_calc_iq__qcout_dn3 = assign10320_e11177_d_n3;
        locals.var_fn133_calc_iq__qcout_dn4 = assign10320_e11177_d_n4;
        locals.var_fn133_calc_iq__qcout_dn5 = assign10320_e11177_d_n5;
        locals.var_fn133_calc_iq__qcout_dn7 = assign10320_e11177_d_n7;
        locals.var_fn133_calc_iq__qcout_dn14 = assign10320_e11177_d_n14;

        let (assign10330_e11181, assign10330_e11181_d_n2, assign10330_e11181_d_n3, assign10330_e11181_d_n4, assign10330_e11181_d_n5, assign10330_e11181_d_n7, assign10330_e11181_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qbout, locals.var_fn133_calc_iq__qbout_dn2, locals.var_fn133_calc_iq__qbout_dn3, locals.var_fn133_calc_iq__qbout_dn4, locals.var_fn133_calc_iq__qbout_dn5, locals.var_fn133_calc_iq__qbout_dn7, locals.var_fn133_calc_iq__qbout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qbout = assign10330_e11181;
        locals.var_fn133_calc_iq__qbout_dn2 = assign10330_e11181_d_n2;
        locals.var_fn133_calc_iq__qbout_dn3 = assign10330_e11181_d_n3;
        locals.var_fn133_calc_iq__qbout_dn4 = assign10330_e11181_d_n4;
        locals.var_fn133_calc_iq__qbout_dn5 = assign10330_e11181_d_n5;
        locals.var_fn133_calc_iq__qbout_dn7 = assign10330_e11181_d_n7;
        locals.var_fn133_calc_iq__qbout_dn14 = assign10330_e11181_d_n14;

        let (assign10340_e11185, assign10340_e11185_d_n2, assign10340_e11185_d_n3, assign10340_e11185_d_n4, assign10340_e11185_d_n5, assign10340_e11185_d_n7, assign10340_e11185_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qsout, locals.var_fn133_calc_iq__qsout_dn2, locals.var_fn133_calc_iq__qsout_dn3, locals.var_fn133_calc_iq__qsout_dn4, locals.var_fn133_calc_iq__qsout_dn5, locals.var_fn133_calc_iq__qsout_dn7, locals.var_fn133_calc_iq__qsout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qsout = assign10340_e11185;
        locals.var_fn133_calc_iq__qsout_dn2 = assign10340_e11185_d_n2;
        locals.var_fn133_calc_iq__qsout_dn3 = assign10340_e11185_d_n3;
        locals.var_fn133_calc_iq__qsout_dn4 = assign10340_e11185_d_n4;
        locals.var_fn133_calc_iq__qsout_dn5 = assign10340_e11185_d_n5;
        locals.var_fn133_calc_iq__qsout_dn7 = assign10340_e11185_d_n7;
        locals.var_fn133_calc_iq__qsout_dn14 = assign10340_e11185_d_n14;

        let (assign10350_e11189, assign10350_e11189_d_n4, assign10350_e11189_d_n5, assign10350_e11189_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vtdibl, locals.var_fn133_calc_iq__vtdibl_dn4, locals.var_fn133_calc_iq__vtdibl_dn5, locals.var_fn133_calc_iq__vtdibl_dn14,)
    }
};
        locals.var_fn133_calc_iq__vtdibl = assign10350_e11189;
        locals.var_fn133_calc_iq__vtdibl_dn4 = assign10350_e11189_d_n4;
        locals.var_fn133_calc_iq__vtdibl_dn5 = assign10350_e11189_d_n5;
        locals.var_fn133_calc_iq__vtdibl_dn14 = assign10350_e11189_d_n14;

        let (assign10360_e11193, assign10360_e11193_d_n2, assign10360_e11193_d_n3, assign10360_e11193_d_n4, assign10360_e11193_d_n5, assign10360_e11193_d_n7, assign10360_e11193_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vdsat1, locals.var_fn133_calc_iq__vdsat1_dn2, locals.var_fn133_calc_iq__vdsat1_dn3, locals.var_fn133_calc_iq__vdsat1_dn4, locals.var_fn133_calc_iq__vdsat1_dn5, locals.var_fn133_calc_iq__vdsat1_dn7, locals.var_fn133_calc_iq__vdsat1_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsat1 = assign10360_e11193;
        locals.var_fn133_calc_iq__vdsat1_dn2 = assign10360_e11193_d_n2;
        locals.var_fn133_calc_iq__vdsat1_dn3 = assign10360_e11193_d_n3;
        locals.var_fn133_calc_iq__vdsat1_dn4 = assign10360_e11193_d_n4;
        locals.var_fn133_calc_iq__vdsat1_dn5 = assign10360_e11193_d_n5;
        locals.var_fn133_calc_iq__vdsat1_dn7 = assign10360_e11193_d_n7;
        locals.var_fn133_calc_iq__vdsat1_dn14 = assign10360_e11193_d_n14;

        let (assign10370_e11197, assign10370_e11197_d_n2, assign10370_e11197_d_n5, assign10370_e11197_d_n7,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_vgsfp1, locals.var_vgsfp1_dn2, locals.var_vgsfp1_dn5, locals.var_vgsfp1_dn7,)
    } else {
        (locals.var_fn133_calc_iq__vgsin, locals.var_fn133_calc_iq__vgsin_dn2, locals.var_fn133_calc_iq__vgsin_dn5, locals.var_fn133_calc_iq__vgsin_dn7,)
    }
};
        locals.var_fn133_calc_iq__vgsin = assign10370_e11197;
        locals.var_fn133_calc_iq__vgsin_dn2 = assign10370_e11197_d_n2;
        locals.var_fn133_calc_iq__vgsin_dn5 = assign10370_e11197_d_n5;
        locals.var_fn133_calc_iq__vgsin_dn7 = assign10370_e11197_d_n7;

        let (assign10380_e11201, assign10380_e11201_d_n5, assign10380_e11201_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_vdsfp1, locals.var_vdsfp1_dn5, locals.var_vdsfp1_dn14,)
    } else {
        (locals.var_fn133_calc_iq__vdsin, locals.var_fn133_calc_iq__vdsin_dn5, locals.var_fn133_calc_iq__vdsin_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsin = assign10380_e11201;
        locals.var_fn133_calc_iq__vdsin_dn5 = assign10380_e11201_d_n5;
        locals.var_fn133_calc_iq__vdsin_dn14 = assign10380_e11201_d_n14;

        let (assign10390_e11205,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p173,)
    } else {
        (locals.var_fn133_calc_iq__qcbflag,)
    }
};
        locals.var_fn133_calc_iq__qcbflag = assign10390_e11205;

    }

    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10400_e11209, assign10400_e11209_d_n2, assign10400_e11209_d_n5, assign10400_e11209_d_n7,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_vcfp1, locals.var_vcfp1_dn2, locals.var_vcfp1_dn5, locals.var_vcfp1_dn7,)
    } else {
        (locals.var_fn133_calc_iq__vcin, locals.var_fn133_calc_iq__vcin_dn2, locals.var_fn133_calc_iq__vcin_dn5, locals.var_fn133_calc_iq__vcin_dn7,)
    }
};
        locals.var_fn133_calc_iq__vcin = assign10400_e11209;
        locals.var_fn133_calc_iq__vcin_dn2 = assign10400_e11209_d_n2;
        locals.var_fn133_calc_iq__vcin_dn5 = assign10400_e11209_d_n5;
        locals.var_fn133_calc_iq__vcin_dn7 = assign10400_e11209_d_n7;

        let (assign10410_e11213, assign10410_e11213_d_n3, assign10410_e11213_d_n5,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_vbfp1, locals.var_vbfp1_dn3, locals.var_vbfp1_dn5,)
    } else {
        (locals.var_fn133_calc_iq__vbin, locals.var_fn133_calc_iq__vbin_dn3, locals.var_fn133_calc_iq__vbin_dn5,)
    }
};
        locals.var_fn133_calc_iq__vbin = assign10410_e11213;
        locals.var_fn133_calc_iq__vbin_dn3 = assign10410_e11213_d_n3;
        locals.var_fn133_calc_iq__vbin_dn5 = assign10410_e11213_d_n5;

        let (assign10420_e11217,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p171,)
    } else {
        (locals.var_fn133_calc_iq__qgsflag,)
    }
};
        locals.var_fn133_calc_iq__qgsflag = assign10420_e11217;

        let (assign10430_e11221, assign10430_e11221_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn133_calc_iq__tambin, locals.var_fn133_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn133_calc_iq__tambin = assign10430_e11221;
        locals.var_fn133_calc_iq__tambin_dn4 = assign10430_e11221_d_n4;

        let (assign10440_e11225,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn133_calc_iq__tnomin,)
    }
};
        locals.var_fn133_calc_iq__tnomin = assign10440_e11225;

        let (assign10450_e11229, assign10450_e11229_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn133_calc_iq__phitin, locals.var_fn133_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn133_calc_iq__phitin = assign10450_e11229;
        locals.var_fn133_calc_iq__phitin_dn4 = assign10450_e11229_d_n4;

        let (assign10460_e11233,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn133_calc_iq__w,)
    }
};
        locals.var_fn133_calc_iq__w = assign10460_e11233;

        let (assign10470_e11237,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p167,)
    } else {
        (locals.var_fn133_calc_iq__lin,)
    }
};
        locals.var_fn133_calc_iq__lin = assign10470_e11237;

        let (assign10480_e11241, assign10480_e11241_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_cgfp1t, locals.var_cgfp1t_dn4,)
    } else {
        (locals.var_fn133_calc_iq__cgin, locals.var_fn133_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn133_calc_iq__cgin = assign10480_e11241;
        locals.var_fn133_calc_iq__cgin_dn4 = assign10480_e11241_d_n4;

        let (assign10490_e11245,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p172,)
    } else {
        (locals.var_fn133_calc_iq__cs,)
    }
};
        locals.var_fn133_calc_iq__cs = assign10490_e11245;

        let (assign10500_e11249, assign10500_e11249_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_ccfp1t, locals.var_ccfp1t_dn4,)
    } else {
        (locals.var_fn133_calc_iq__cc, locals.var_fn133_calc_iq__cc_dn4,)
    }
};
        locals.var_fn133_calc_iq__cc = assign10500_e11249;
        locals.var_fn133_calc_iq__cc_dn4 = assign10500_e11249_d_n4;

        let (assign10510_e11253, assign10510_e11253_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_cbfp1t, locals.var_cbfp1t_dn4,)
    } else {
        (locals.var_fn133_calc_iq__cb, locals.var_fn133_calc_iq__cb_dn4,)
    }
};
        locals.var_fn133_calc_iq__cb = assign10510_e11253;
        locals.var_fn133_calc_iq__cb_dn4 = assign10510_e11253_d_n4;

        let (assign10520_e11257,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p168,)
    } else {
        (locals.var_fn133_calc_iq__vto,)
    }
};
        locals.var_fn133_calc_iq__vto = assign10520_e11257;

        let (assign10530_e11261,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p182,)
    } else {
        (locals.var_fn133_calc_iq__ss,)
    }
};
        locals.var_fn133_calc_iq__ss = assign10530_e11261;

        let (assign10540_e11265,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p181,)
    } else {
        (locals.var_fn133_calc_iq__delta1,)
    }
};
        locals.var_fn133_calc_iq__delta1 = assign10540_e11265;

        let (assign10550_e11269,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn133_calc_iq__delta2,)
    }
};
        locals.var_fn133_calc_iq__delta2 = assign10550_e11269;

        let (assign10560_e11273,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p183,)
    } else {
        (locals.var_fn133_calc_iq__nd,)
    }
};
        locals.var_fn133_calc_iq__nd = assign10560_e11273;

        let (assign10570_e11277,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p187,)
    } else {
        (locals.var_fn133_calc_iq__alpha,)
    }
};
        locals.var_fn133_calc_iq__alpha = assign10570_e11277;

        let (assign10580_e11281,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p178,)
    } else {
        (locals.var_fn133_calc_iq__vel0,)
    }
};
        locals.var_fn133_calc_iq__vel0 = assign10580_e11281;

        let (assign10590_e11285,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p179,)
    } else {
        (locals.var_fn133_calc_iq__mu0,)
    }
};
        locals.var_fn133_calc_iq__mu0 = assign10590_e11285;

        let (assign10600_e11289,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p180,)
    } else {
        (locals.var_fn133_calc_iq__beta,)
    }
};
        locals.var_fn133_calc_iq__beta = assign10600_e11289;

        let (assign10610_e11293,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p186,)
    } else {
        (locals.var_fn133_calc_iq__mtheta,)
    }
};
        locals.var_fn133_calc_iq__mtheta = assign10610_e11293;

        let (assign10620_e11297,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p185,)
    } else {
        (locals.var_fn133_calc_iq__vtheta,)
    }
};
        locals.var_fn133_calc_iq__vtheta = assign10620_e11297;

        let (assign10630_e11301,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p184,)
    } else {
        (locals.var_fn133_calc_iq__vtzeta,)
    }
};
        locals.var_fn133_calc_iq__vtzeta = assign10630_e11301;

        let (assign10640_e11305,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn133_calc_iq__dibsat,)
    }
};
        locals.var_fn133_calc_iq__dibsat = assign10640_e11305;

        let (assign10650_e11309,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn133_calc_iq__epsilon,)
    }
};
        locals.var_fn133_calc_iq__epsilon = assign10650_e11309;

        let (assign10660_e11313,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn133_calc_iq__vzeta,)
    }
};
        locals.var_fn133_calc_iq__vzeta = assign10660_e11313;

        let (assign10670_e11317,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn133_calc_iq__lambda,)
    }
};
        locals.var_fn133_calc_iq__lambda = assign10670_e11317;

        let (assign10680_e11321,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn133_calc_iq__ngf,)
    }
};
        locals.var_fn133_calc_iq__ngf = assign10680_e11321;

        let (assign10690_e11325,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn133_calc_iq__type,)
    }
};
        locals.var_fn133_calc_iq__type = assign10690_e11325;

        let (assign10700_e11329,) = {
    if (locals.var_guard132 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn133_calc_iq__trapfracdl,)
    }
};
        locals.var_fn133_calc_iq__trapfracdl = assign10700_e11329;

        let (assign10710_e11333, assign10710_e11333_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__alpha_phit, locals.var_fn133_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn133_calc_iq__alpha_phit = assign10710_e11333;
        locals.var_fn133_calc_iq__alpha_phit_dn4 = assign10710_e11333_d_n4;

        let (assign10720_e11337, assign10720_e11337_d_n5, assign10720_e11337_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__delta, locals.var_fn133_calc_iq__delta_dn5, locals.var_fn133_calc_iq__delta_dn14,)
    }
};
        locals.var_fn133_calc_iq__delta = assign10720_e11337;
        locals.var_fn133_calc_iq__delta_dn5 = assign10720_e11337_d_n5;
        locals.var_fn133_calc_iq__delta_dn14 = assign10720_e11337_d_n14;

        let (assign10730_e11341, assign10730_e11341_d_n4, assign10730_e11341_d_n5, assign10730_e11341_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__n, locals.var_fn133_calc_iq__n_dn4, locals.var_fn133_calc_iq__n_dn5, locals.var_fn133_calc_iq__n_dn14,)
    }
};
        locals.var_fn133_calc_iq__n = assign10730_e11341;
        locals.var_fn133_calc_iq__n_dn4 = assign10730_e11341_d_n4;
        locals.var_fn133_calc_iq__n_dn5 = assign10730_e11341_d_n5;
        locals.var_fn133_calc_iq__n_dn14 = assign10730_e11341_d_n14;

        let (assign10740_e11345, assign10740_e11345_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vtof, locals.var_fn133_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn133_calc_iq__vtof = assign10740_e11345;
        locals.var_fn133_calc_iq__vtof_dn4 = assign10740_e11345_d_n4;

        let (assign10750_e11349, assign10750_e11349_d_n5, assign10750_e11349_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vsatdibl, locals.var_fn133_calc_iq__vsatdibl_dn5, locals.var_fn133_calc_iq__vsatdibl_dn14,)
    }
};
        locals.var_fn133_calc_iq__vsatdibl = assign10750_e11349;
        locals.var_fn133_calc_iq__vsatdibl_dn5 = assign10750_e11349_d_n5;
        locals.var_fn133_calc_iq__vsatdibl_dn14 = assign10750_e11349_d_n14;

        let (assign10760_e11353, assign10760_e11353_d_n2, assign10760_e11353_d_n3, assign10760_e11353_d_n4, assign10760_e11353_d_n5, assign10760_e11353_d_n7, assign10760_e11353_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffs, locals.var_fn133_calc_iq__ffs_dn2, locals.var_fn133_calc_iq__ffs_dn3, locals.var_fn133_calc_iq__ffs_dn4, locals.var_fn133_calc_iq__ffs_dn5, locals.var_fn133_calc_iq__ffs_dn7, locals.var_fn133_calc_iq__ffs_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs = assign10760_e11353;
        locals.var_fn133_calc_iq__ffs_dn2 = assign10760_e11353_d_n2;
        locals.var_fn133_calc_iq__ffs_dn3 = assign10760_e11353_d_n3;
        locals.var_fn133_calc_iq__ffs_dn4 = assign10760_e11353_d_n4;
        locals.var_fn133_calc_iq__ffs_dn5 = assign10760_e11353_d_n5;
        locals.var_fn133_calc_iq__ffs_dn7 = assign10760_e11353_d_n7;
        locals.var_fn133_calc_iq__ffs_dn14 = assign10760_e11353_d_n14;

        let (assign10770_e11357, assign10770_e11357_d_n4, assign10770_e11357_d_n5, assign10770_e11357_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__two_n_phit, locals.var_fn133_calc_iq__two_n_phit_dn4, locals.var_fn133_calc_iq__two_n_phit_dn5, locals.var_fn133_calc_iq__two_n_phit_dn14,)
    }
};
        locals.var_fn133_calc_iq__two_n_phit = assign10770_e11357;
        locals.var_fn133_calc_iq__two_n_phit_dn4 = assign10770_e11357_d_n4;
        locals.var_fn133_calc_iq__two_n_phit_dn5 = assign10770_e11357_d_n5;
        locals.var_fn133_calc_iq__two_n_phit_dn14 = assign10770_e11357_d_n14;

        let (assign10780_e11361, assign10780_e11361_d_n4, assign10780_e11361_d_n5, assign10780_e11361_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qref, locals.var_fn133_calc_iq__qref_dn4, locals.var_fn133_calc_iq__qref_dn5, locals.var_fn133_calc_iq__qref_dn14,)
    }
};
        locals.var_fn133_calc_iq__qref = assign10780_e11361;
        locals.var_fn133_calc_iq__qref_dn4 = assign10780_e11361_d_n4;
        locals.var_fn133_calc_iq__qref_dn5 = assign10780_e11361_d_n5;
        locals.var_fn133_calc_iq__qref_dn14 = assign10780_e11361_d_n14;

        let (assign10790_e11365, assign10790_e11365_d_n2, assign10790_e11365_d_n3, assign10790_e11365_d_n4, assign10790_e11365_d_n5, assign10790_e11365_d_n7, assign10790_e11365_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__etas, locals.var_fn133_calc_iq__etas_dn2, locals.var_fn133_calc_iq__etas_dn3, locals.var_fn133_calc_iq__etas_dn4, locals.var_fn133_calc_iq__etas_dn5, locals.var_fn133_calc_iq__etas_dn7, locals.var_fn133_calc_iq__etas_dn14,)
    }
};
        locals.var_fn133_calc_iq__etas = assign10790_e11365;
        locals.var_fn133_calc_iq__etas_dn2 = assign10790_e11365_d_n2;
        locals.var_fn133_calc_iq__etas_dn3 = assign10790_e11365_d_n3;
        locals.var_fn133_calc_iq__etas_dn4 = assign10790_e11365_d_n4;
        locals.var_fn133_calc_iq__etas_dn5 = assign10790_e11365_d_n5;
        locals.var_fn133_calc_iq__etas_dn7 = assign10790_e11365_d_n7;
        locals.var_fn133_calc_iq__etas_dn14 = assign10790_e11365_d_n14;

        let (assign10800_e11369, assign10800_e11369_d_n2, assign10800_e11369_d_n3, assign10800_e11369_d_n4, assign10800_e11369_d_n5, assign10800_e11369_d_n7, assign10800_e11369_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qinvs, locals.var_fn133_calc_iq__qinvs_dn2, locals.var_fn133_calc_iq__qinvs_dn3, locals.var_fn133_calc_iq__qinvs_dn4, locals.var_fn133_calc_iq__qinvs_dn5, locals.var_fn133_calc_iq__qinvs_dn7, locals.var_fn133_calc_iq__qinvs_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs = assign10800_e11369;
        locals.var_fn133_calc_iq__qinvs_dn2 = assign10800_e11369_d_n2;
        locals.var_fn133_calc_iq__qinvs_dn3 = assign10800_e11369_d_n3;
        locals.var_fn133_calc_iq__qinvs_dn4 = assign10800_e11369_d_n4;
        locals.var_fn133_calc_iq__qinvs_dn5 = assign10800_e11369_d_n5;
        locals.var_fn133_calc_iq__qinvs_dn7 = assign10800_e11369_d_n7;
        locals.var_fn133_calc_iq__qinvs_dn14 = assign10800_e11369_d_n14;

        let (assign10810_e11373, assign10810_e11373_d_n2, assign10810_e11373_d_n3, assign10810_e11373_d_n4, assign10810_e11373_d_n5, assign10810_e11373_d_n7, assign10810_e11373_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__muf, locals.var_fn133_calc_iq__muf_dn2, locals.var_fn133_calc_iq__muf_dn3, locals.var_fn133_calc_iq__muf_dn4, locals.var_fn133_calc_iq__muf_dn5, locals.var_fn133_calc_iq__muf_dn7, locals.var_fn133_calc_iq__muf_dn14,)
    }
};
        locals.var_fn133_calc_iq__muf = assign10810_e11373;
        locals.var_fn133_calc_iq__muf_dn2 = assign10810_e11373_d_n2;
        locals.var_fn133_calc_iq__muf_dn3 = assign10810_e11373_d_n3;
        locals.var_fn133_calc_iq__muf_dn4 = assign10810_e11373_d_n4;
        locals.var_fn133_calc_iq__muf_dn5 = assign10810_e11373_d_n5;
        locals.var_fn133_calc_iq__muf_dn7 = assign10810_e11373_d_n7;
        locals.var_fn133_calc_iq__muf_dn14 = assign10810_e11373_d_n14;

        let (assign10820_e11377, assign10820_e11377_d_n2, assign10820_e11377_d_n3, assign10820_e11377_d_n4, assign10820_e11377_d_n5, assign10820_e11377_d_n7, assign10820_e11377_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vx, locals.var_fn133_calc_iq__vx_dn2, locals.var_fn133_calc_iq__vx_dn3, locals.var_fn133_calc_iq__vx_dn4, locals.var_fn133_calc_iq__vx_dn5, locals.var_fn133_calc_iq__vx_dn7, locals.var_fn133_calc_iq__vx_dn14,)
    }
};
        locals.var_fn133_calc_iq__vx = assign10820_e11377;
        locals.var_fn133_calc_iq__vx_dn2 = assign10820_e11377_d_n2;
        locals.var_fn133_calc_iq__vx_dn3 = assign10820_e11377_d_n3;
        locals.var_fn133_calc_iq__vx_dn4 = assign10820_e11377_d_n4;
        locals.var_fn133_calc_iq__vx_dn5 = assign10820_e11377_d_n5;
        locals.var_fn133_calc_iq__vx_dn7 = assign10820_e11377_d_n7;
        locals.var_fn133_calc_iq__vx_dn14 = assign10820_e11377_d_n14;

        let (assign10840_e11385, assign10840_e11385_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__n0, locals.var_fn133_calc_iq__n0_dn4,)
    }
};
        locals.var_fn133_calc_iq__n0 = assign10840_e11385;
        locals.var_fn133_calc_iq__n0_dn4 = assign10840_e11385_d_n4;

        let (assign10850_e11389, assign10850_e11389_d_n2, assign10850_e11389_d_n4, assign10850_e11389_d_n5, assign10850_e11389_d_n7, assign10850_e11389_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffs0, locals.var_fn133_calc_iq__ffs0_dn2, locals.var_fn133_calc_iq__ffs0_dn4, locals.var_fn133_calc_iq__ffs0_dn5, locals.var_fn133_calc_iq__ffs0_dn7, locals.var_fn133_calc_iq__ffs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs0 = assign10850_e11389;
        locals.var_fn133_calc_iq__ffs0_dn2 = assign10850_e11389_d_n2;
        locals.var_fn133_calc_iq__ffs0_dn4 = assign10850_e11389_d_n4;
        locals.var_fn133_calc_iq__ffs0_dn5 = assign10850_e11389_d_n5;
        locals.var_fn133_calc_iq__ffs0_dn7 = assign10850_e11389_d_n7;
        locals.var_fn133_calc_iq__ffs0_dn14 = assign10850_e11389_d_n14;

        let (assign10860_e11393, assign10860_e11393_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__two_n_phit0, locals.var_fn133_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn133_calc_iq__two_n_phit0 = assign10860_e11393;
        locals.var_fn133_calc_iq__two_n_phit0_dn4 = assign10860_e11393_d_n4;

        let (assign10870_e11397, assign10870_e11397_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qref0, locals.var_fn133_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn133_calc_iq__qref0 = assign10870_e11397;
        locals.var_fn133_calc_iq__qref0_dn4 = assign10870_e11397_d_n4;

        let (assign10880_e11401, assign10880_e11401_d_n2, assign10880_e11401_d_n4, assign10880_e11401_d_n5, assign10880_e11401_d_n7, assign10880_e11401_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__etas0, locals.var_fn133_calc_iq__etas0_dn2, locals.var_fn133_calc_iq__etas0_dn4, locals.var_fn133_calc_iq__etas0_dn5, locals.var_fn133_calc_iq__etas0_dn7, locals.var_fn133_calc_iq__etas0_dn14,)
    }
};
        locals.var_fn133_calc_iq__etas0 = assign10880_e11401;
        locals.var_fn133_calc_iq__etas0_dn2 = assign10880_e11401_d_n2;
        locals.var_fn133_calc_iq__etas0_dn4 = assign10880_e11401_d_n4;
        locals.var_fn133_calc_iq__etas0_dn5 = assign10880_e11401_d_n5;
        locals.var_fn133_calc_iq__etas0_dn7 = assign10880_e11401_d_n7;
        locals.var_fn133_calc_iq__etas0_dn14 = assign10880_e11401_d_n14;

    }

    pub(super) fn stamp_transient_block_26(
        locals: &mut StampLocals,
    ) {
        let (assign10890_e11405, assign10890_e11405_d_n2, assign10890_e11405_d_n4, assign10890_e11405_d_n5, assign10890_e11405_d_n7, assign10890_e11405_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qinvs0, locals.var_fn133_calc_iq__qinvs0_dn2, locals.var_fn133_calc_iq__qinvs0_dn4, locals.var_fn133_calc_iq__qinvs0_dn5, locals.var_fn133_calc_iq__qinvs0_dn7, locals.var_fn133_calc_iq__qinvs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs0 = assign10890_e11405;
        locals.var_fn133_calc_iq__qinvs0_dn2 = assign10890_e11405_d_n2;
        locals.var_fn133_calc_iq__qinvs0_dn4 = assign10890_e11405_d_n4;
        locals.var_fn133_calc_iq__qinvs0_dn5 = assign10890_e11405_d_n5;
        locals.var_fn133_calc_iq__qinvs0_dn7 = assign10890_e11405_d_n7;
        locals.var_fn133_calc_iq__qinvs0_dn14 = assign10890_e11405_d_n14;

        let (assign10900_e11409, assign10900_e11409_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__muf0, locals.var_fn133_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn133_calc_iq__muf0 = assign10900_e11409;
        locals.var_fn133_calc_iq__muf0_dn4 = assign10900_e11409_d_n4;

        let (assign10910_e11413, assign10910_e11413_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vx0, locals.var_fn133_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn133_calc_iq__vx0 = assign10910_e11413;
        locals.var_fn133_calc_iq__vx0_dn4 = assign10910_e11413_d_n4;

        let (assign10920_e11417, assign10920_e11417_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__tfacmobin, locals.var_fn133_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn133_calc_iq__tfacmobin = assign10920_e11417;
        locals.var_fn133_calc_iq__tfacmobin_dn4 = assign10920_e11417_d_n4;

        let (assign10930_e11421, assign10930_e11421_d_n2, assign10930_e11421_d_n3, assign10930_e11421_d_n4, assign10930_e11421_d_n5, assign10930_e11421_d_n7, assign10930_e11421_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ff, locals.var_fn133_calc_iq__ff_dn2, locals.var_fn133_calc_iq__ff_dn3, locals.var_fn133_calc_iq__ff_dn4, locals.var_fn133_calc_iq__ff_dn5, locals.var_fn133_calc_iq__ff_dn7, locals.var_fn133_calc_iq__ff_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff = assign10930_e11421;
        locals.var_fn133_calc_iq__ff_dn2 = assign10930_e11421_d_n2;
        locals.var_fn133_calc_iq__ff_dn3 = assign10930_e11421_d_n3;
        locals.var_fn133_calc_iq__ff_dn4 = assign10930_e11421_d_n4;
        locals.var_fn133_calc_iq__ff_dn5 = assign10930_e11421_d_n5;
        locals.var_fn133_calc_iq__ff_dn7 = assign10930_e11421_d_n7;
        locals.var_fn133_calc_iq__ff_dn14 = assign10930_e11421_d_n14;

        let (assign10940_e11425, assign10940_e11425_d_n2, assign10940_e11425_d_n3, assign10940_e11425_d_n4, assign10940_e11425_d_n5, assign10940_e11425_d_n7, assign10940_e11425_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__eta, locals.var_fn133_calc_iq__eta_dn2, locals.var_fn133_calc_iq__eta_dn3, locals.var_fn133_calc_iq__eta_dn4, locals.var_fn133_calc_iq__eta_dn5, locals.var_fn133_calc_iq__eta_dn7, locals.var_fn133_calc_iq__eta_dn14,)
    }
};
        locals.var_fn133_calc_iq__eta = assign10940_e11425;
        locals.var_fn133_calc_iq__eta_dn2 = assign10940_e11425_d_n2;
        locals.var_fn133_calc_iq__eta_dn3 = assign10940_e11425_d_n3;
        locals.var_fn133_calc_iq__eta_dn4 = assign10940_e11425_d_n4;
        locals.var_fn133_calc_iq__eta_dn5 = assign10940_e11425_d_n5;
        locals.var_fn133_calc_iq__eta_dn7 = assign10940_e11425_d_n7;
        locals.var_fn133_calc_iq__eta_dn14 = assign10940_e11425_d_n14;

        let (assign10950_e11429, assign10950_e11429_d_n2, assign10950_e11429_d_n3, assign10950_e11429_d_n4, assign10950_e11429_d_n5, assign10950_e11429_d_n7, assign10950_e11429_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qinvv, locals.var_fn133_calc_iq__qinvv_dn2, locals.var_fn133_calc_iq__qinvv_dn3, locals.var_fn133_calc_iq__qinvv_dn4, locals.var_fn133_calc_iq__qinvv_dn5, locals.var_fn133_calc_iq__qinvv_dn7, locals.var_fn133_calc_iq__qinvv_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvv = assign10950_e11429;
        locals.var_fn133_calc_iq__qinvv_dn2 = assign10950_e11429_d_n2;
        locals.var_fn133_calc_iq__qinvv_dn3 = assign10950_e11429_d_n3;
        locals.var_fn133_calc_iq__qinvv_dn4 = assign10950_e11429_d_n4;
        locals.var_fn133_calc_iq__qinvv_dn5 = assign10950_e11429_d_n5;
        locals.var_fn133_calc_iq__qinvv_dn7 = assign10950_e11429_d_n7;
        locals.var_fn133_calc_iq__qinvv_dn14 = assign10950_e11429_d_n14;

        let (assign10960_e11433, assign10960_e11433_d_n2, assign10960_e11433_d_n4, assign10960_e11433_d_n5, assign10960_e11433_d_n7, assign10960_e11433_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ff0, locals.var_fn133_calc_iq__ff0_dn2, locals.var_fn133_calc_iq__ff0_dn4, locals.var_fn133_calc_iq__ff0_dn5, locals.var_fn133_calc_iq__ff0_dn7, locals.var_fn133_calc_iq__ff0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff0 = assign10960_e11433;
        locals.var_fn133_calc_iq__ff0_dn2 = assign10960_e11433_d_n2;
        locals.var_fn133_calc_iq__ff0_dn4 = assign10960_e11433_d_n4;
        locals.var_fn133_calc_iq__ff0_dn5 = assign10960_e11433_d_n5;
        locals.var_fn133_calc_iq__ff0_dn7 = assign10960_e11433_d_n7;
        locals.var_fn133_calc_iq__ff0_dn14 = assign10960_e11433_d_n14;

        let (assign10970_e11437, assign10970_e11437_d_n2, assign10970_e11437_d_n4, assign10970_e11437_d_n5, assign10970_e11437_d_n7, assign10970_e11437_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__eta0, locals.var_fn133_calc_iq__eta0_dn2, locals.var_fn133_calc_iq__eta0_dn4, locals.var_fn133_calc_iq__eta0_dn5, locals.var_fn133_calc_iq__eta0_dn7, locals.var_fn133_calc_iq__eta0_dn14,)
    }
};
        locals.var_fn133_calc_iq__eta0 = assign10970_e11437;
        locals.var_fn133_calc_iq__eta0_dn2 = assign10970_e11437_d_n2;
        locals.var_fn133_calc_iq__eta0_dn4 = assign10970_e11437_d_n4;
        locals.var_fn133_calc_iq__eta0_dn5 = assign10970_e11437_d_n5;
        locals.var_fn133_calc_iq__eta0_dn7 = assign10970_e11437_d_n7;
        locals.var_fn133_calc_iq__eta0_dn14 = assign10970_e11437_d_n14;

        let (assign10980_e11441, assign10980_e11441_d_n2, assign10980_e11441_d_n4, assign10980_e11441_d_n5, assign10980_e11441_d_n7, assign10980_e11441_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qinvv0, locals.var_fn133_calc_iq__qinvv0_dn2, locals.var_fn133_calc_iq__qinvv0_dn4, locals.var_fn133_calc_iq__qinvv0_dn5, locals.var_fn133_calc_iq__qinvv0_dn7, locals.var_fn133_calc_iq__qinvv0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvv0 = assign10980_e11441;
        locals.var_fn133_calc_iq__qinvv0_dn2 = assign10980_e11441_d_n2;
        locals.var_fn133_calc_iq__qinvv0_dn4 = assign10980_e11441_d_n4;
        locals.var_fn133_calc_iq__qinvv0_dn5 = assign10980_e11441_d_n5;
        locals.var_fn133_calc_iq__qinvv0_dn7 = assign10980_e11441_d_n7;
        locals.var_fn133_calc_iq__qinvv0_dn14 = assign10980_e11441_d_n14;

        let (assign10990_e11445, assign10990_e11445_d_n2, assign10990_e11445_d_n3, assign10990_e11445_d_n4, assign10990_e11445_d_n5, assign10990_e11445_d_n7, assign10990_e11445_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vdsats, locals.var_fn133_calc_iq__vdsats_dn2, locals.var_fn133_calc_iq__vdsats_dn3, locals.var_fn133_calc_iq__vdsats_dn4, locals.var_fn133_calc_iq__vdsats_dn5, locals.var_fn133_calc_iq__vdsats_dn7, locals.var_fn133_calc_iq__vdsats_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsats = assign10990_e11445;
        locals.var_fn133_calc_iq__vdsats_dn2 = assign10990_e11445_d_n2;
        locals.var_fn133_calc_iq__vdsats_dn3 = assign10990_e11445_d_n3;
        locals.var_fn133_calc_iq__vdsats_dn4 = assign10990_e11445_d_n4;
        locals.var_fn133_calc_iq__vdsats_dn5 = assign10990_e11445_d_n5;
        locals.var_fn133_calc_iq__vdsats_dn7 = assign10990_e11445_d_n7;
        locals.var_fn133_calc_iq__vdsats_dn14 = assign10990_e11445_d_n14;

        let (assign11000_e11449, assign11000_e11449_d_n2, assign11000_e11449_d_n3, assign11000_e11449_d_n4, assign11000_e11449_d_n5, assign11000_e11449_d_n7, assign11000_e11449_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vdsats1, locals.var_fn133_calc_iq__vdsats1_dn2, locals.var_fn133_calc_iq__vdsats1_dn3, locals.var_fn133_calc_iq__vdsats1_dn4, locals.var_fn133_calc_iq__vdsats1_dn5, locals.var_fn133_calc_iq__vdsats1_dn7, locals.var_fn133_calc_iq__vdsats1_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsats1 = assign11000_e11449;
        locals.var_fn133_calc_iq__vdsats1_dn2 = assign11000_e11449_d_n2;
        locals.var_fn133_calc_iq__vdsats1_dn3 = assign11000_e11449_d_n3;
        locals.var_fn133_calc_iq__vdsats1_dn4 = assign11000_e11449_d_n4;
        locals.var_fn133_calc_iq__vdsats1_dn5 = assign11000_e11449_d_n5;
        locals.var_fn133_calc_iq__vdsats1_dn7 = assign11000_e11449_d_n7;
        locals.var_fn133_calc_iq__vdsats1_dn14 = assign11000_e11449_d_n14;

        let (assign11010_e11453, assign11010_e11453_d_n2, assign11010_e11453_d_n3, assign11010_e11453_d_n4, assign11010_e11453_d_n5, assign11010_e11453_d_n7, assign11010_e11453_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vdsat, locals.var_fn133_calc_iq__vdsat_dn2, locals.var_fn133_calc_iq__vdsat_dn3, locals.var_fn133_calc_iq__vdsat_dn4, locals.var_fn133_calc_iq__vdsat_dn5, locals.var_fn133_calc_iq__vdsat_dn7, locals.var_fn133_calc_iq__vdsat_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsat = assign11010_e11453;
        locals.var_fn133_calc_iq__vdsat_dn2 = assign11010_e11453_d_n2;
        locals.var_fn133_calc_iq__vdsat_dn3 = assign11010_e11453_d_n3;
        locals.var_fn133_calc_iq__vdsat_dn4 = assign11010_e11453_d_n4;
        locals.var_fn133_calc_iq__vdsat_dn5 = assign11010_e11453_d_n5;
        locals.var_fn133_calc_iq__vdsat_dn7 = assign11010_e11453_d_n7;
        locals.var_fn133_calc_iq__vdsat_dn14 = assign11010_e11453_d_n14;

        let (assign11020_e11457, assign11020_e11457_d_n2, assign11020_e11457_d_n3, assign11020_e11457_d_n4, assign11020_e11457_d_n5, assign11020_e11457_d_n7, assign11020_e11457_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__fsd, locals.var_fn133_calc_iq__fsd_dn2, locals.var_fn133_calc_iq__fsd_dn3, locals.var_fn133_calc_iq__fsd_dn4, locals.var_fn133_calc_iq__fsd_dn5, locals.var_fn133_calc_iq__fsd_dn7, locals.var_fn133_calc_iq__fsd_dn14,)
    }
};
        locals.var_fn133_calc_iq__fsd = assign11020_e11457;
        locals.var_fn133_calc_iq__fsd_dn2 = assign11020_e11457_d_n2;
        locals.var_fn133_calc_iq__fsd_dn3 = assign11020_e11457_d_n3;
        locals.var_fn133_calc_iq__fsd_dn4 = assign11020_e11457_d_n4;
        locals.var_fn133_calc_iq__fsd_dn5 = assign11020_e11457_d_n5;
        locals.var_fn133_calc_iq__fsd_dn7 = assign11020_e11457_d_n7;
        locals.var_fn133_calc_iq__fsd_dn14 = assign11020_e11457_d_n14;

        let (assign11030_e11461, assign11030_e11461_d_n2, assign11030_e11461_d_n3, assign11030_e11461_d_n4, assign11030_e11461_d_n5, assign11030_e11461_d_n7, assign11030_e11461_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vdx, locals.var_fn133_calc_iq__vdx_dn2, locals.var_fn133_calc_iq__vdx_dn3, locals.var_fn133_calc_iq__vdx_dn4, locals.var_fn133_calc_iq__vdx_dn5, locals.var_fn133_calc_iq__vdx_dn7, locals.var_fn133_calc_iq__vdx_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdx = assign11030_e11461;
        locals.var_fn133_calc_iq__vdx_dn2 = assign11030_e11461_d_n2;
        locals.var_fn133_calc_iq__vdx_dn3 = assign11030_e11461_d_n3;
        locals.var_fn133_calc_iq__vdx_dn4 = assign11030_e11461_d_n4;
        locals.var_fn133_calc_iq__vdx_dn5 = assign11030_e11461_d_n5;
        locals.var_fn133_calc_iq__vdx_dn7 = assign11030_e11461_d_n7;
        locals.var_fn133_calc_iq__vdx_dn14 = assign11030_e11461_d_n14;

        let (assign11040_e11465, assign11040_e11465_d_n2, assign11040_e11465_d_n3, assign11040_e11465_d_n4, assign11040_e11465_d_n5, assign11040_e11465_d_n7, assign11040_e11465_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__fds, locals.var_fn133_calc_iq__fds_dn2, locals.var_fn133_calc_iq__fds_dn3, locals.var_fn133_calc_iq__fds_dn4, locals.var_fn133_calc_iq__fds_dn5, locals.var_fn133_calc_iq__fds_dn7, locals.var_fn133_calc_iq__fds_dn14,)
    }
};
        locals.var_fn133_calc_iq__fds = assign11040_e11465;
        locals.var_fn133_calc_iq__fds_dn2 = assign11040_e11465_d_n2;
        locals.var_fn133_calc_iq__fds_dn3 = assign11040_e11465_d_n3;
        locals.var_fn133_calc_iq__fds_dn4 = assign11040_e11465_d_n4;
        locals.var_fn133_calc_iq__fds_dn5 = assign11040_e11465_d_n5;
        locals.var_fn133_calc_iq__fds_dn7 = assign11040_e11465_d_n7;
        locals.var_fn133_calc_iq__fds_dn14 = assign11040_e11465_d_n14;

        let (assign11050_e11469, assign11050_e11469_d_n2, assign11050_e11469_d_n3, assign11050_e11469_d_n4, assign11050_e11469_d_n5, assign11050_e11469_d_n7, assign11050_e11469_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vsx, locals.var_fn133_calc_iq__vsx_dn2, locals.var_fn133_calc_iq__vsx_dn3, locals.var_fn133_calc_iq__vsx_dn4, locals.var_fn133_calc_iq__vsx_dn5, locals.var_fn133_calc_iq__vsx_dn7, locals.var_fn133_calc_iq__vsx_dn14,)
    }
};
        locals.var_fn133_calc_iq__vsx = assign11050_e11469;
        locals.var_fn133_calc_iq__vsx_dn2 = assign11050_e11469_d_n2;
        locals.var_fn133_calc_iq__vsx_dn3 = assign11050_e11469_d_n3;
        locals.var_fn133_calc_iq__vsx_dn4 = assign11050_e11469_d_n4;
        locals.var_fn133_calc_iq__vsx_dn5 = assign11050_e11469_d_n5;
        locals.var_fn133_calc_iq__vsx_dn7 = assign11050_e11469_d_n7;
        locals.var_fn133_calc_iq__vsx_dn14 = assign11050_e11469_d_n14;

        let (assign11060_e11473, assign11060_e11473_d_n2, assign11060_e11473_d_n3, assign11060_e11473_d_n4, assign11060_e11473_d_n5, assign11060_e11473_d_n7, assign11060_e11473_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffd, locals.var_fn133_calc_iq__ffd_dn2, locals.var_fn133_calc_iq__ffd_dn3, locals.var_fn133_calc_iq__ffd_dn4, locals.var_fn133_calc_iq__ffd_dn5, locals.var_fn133_calc_iq__ffd_dn7, locals.var_fn133_calc_iq__ffd_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffd = assign11060_e11473;
        locals.var_fn133_calc_iq__ffd_dn2 = assign11060_e11473_d_n2;
        locals.var_fn133_calc_iq__ffd_dn3 = assign11060_e11473_d_n3;
        locals.var_fn133_calc_iq__ffd_dn4 = assign11060_e11473_d_n4;
        locals.var_fn133_calc_iq__ffd_dn5 = assign11060_e11473_d_n5;
        locals.var_fn133_calc_iq__ffd_dn7 = assign11060_e11473_d_n7;
        locals.var_fn133_calc_iq__ffd_dn14 = assign11060_e11473_d_n14;

        let (assign11070_e11477, assign11070_e11477_d_n2, assign11070_e11477_d_n3, assign11070_e11477_d_n4, assign11070_e11477_d_n5, assign11070_e11477_d_n7, assign11070_e11477_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__etad, locals.var_fn133_calc_iq__etad_dn2, locals.var_fn133_calc_iq__etad_dn3, locals.var_fn133_calc_iq__etad_dn4, locals.var_fn133_calc_iq__etad_dn5, locals.var_fn133_calc_iq__etad_dn7, locals.var_fn133_calc_iq__etad_dn14,)
    }
};
        locals.var_fn133_calc_iq__etad = assign11070_e11477;
        locals.var_fn133_calc_iq__etad_dn2 = assign11070_e11477_d_n2;
        locals.var_fn133_calc_iq__etad_dn3 = assign11070_e11477_d_n3;
        locals.var_fn133_calc_iq__etad_dn4 = assign11070_e11477_d_n4;
        locals.var_fn133_calc_iq__etad_dn5 = assign11070_e11477_d_n5;
        locals.var_fn133_calc_iq__etad_dn7 = assign11070_e11477_d_n7;
        locals.var_fn133_calc_iq__etad_dn14 = assign11070_e11477_d_n14;

        let (assign11080_e11481, assign11080_e11481_d_n2, assign11080_e11481_d_n3, assign11080_e11481_d_n4, assign11080_e11481_d_n5, assign11080_e11481_d_n7, assign11080_e11481_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qinvd, locals.var_fn133_calc_iq__qinvd_dn2, locals.var_fn133_calc_iq__qinvd_dn3, locals.var_fn133_calc_iq__qinvd_dn4, locals.var_fn133_calc_iq__qinvd_dn5, locals.var_fn133_calc_iq__qinvd_dn7, locals.var_fn133_calc_iq__qinvd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd = assign11080_e11481;
        locals.var_fn133_calc_iq__qinvd_dn2 = assign11080_e11481_d_n2;
        locals.var_fn133_calc_iq__qinvd_dn3 = assign11080_e11481_d_n3;
        locals.var_fn133_calc_iq__qinvd_dn4 = assign11080_e11481_d_n4;
        locals.var_fn133_calc_iq__qinvd_dn5 = assign11080_e11481_d_n5;
        locals.var_fn133_calc_iq__qinvd_dn7 = assign11080_e11481_d_n7;
        locals.var_fn133_calc_iq__qinvd_dn14 = assign11080_e11481_d_n14;

        let (assign11090_e11485, assign11090_e11485_d_n2, assign11090_e11485_d_n3, assign11090_e11485_d_n4, assign11090_e11485_d_n5, assign11090_e11485_d_n7, assign11090_e11485_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vdsc, locals.var_fn133_calc_iq__vdsc_dn2, locals.var_fn133_calc_iq__vdsc_dn3, locals.var_fn133_calc_iq__vdsc_dn4, locals.var_fn133_calc_iq__vdsc_dn5, locals.var_fn133_calc_iq__vdsc_dn7, locals.var_fn133_calc_iq__vdsc_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsc = assign11090_e11485;
        locals.var_fn133_calc_iq__vdsc_dn2 = assign11090_e11485_d_n2;
        locals.var_fn133_calc_iq__vdsc_dn3 = assign11090_e11485_d_n3;
        locals.var_fn133_calc_iq__vdsc_dn4 = assign11090_e11485_d_n4;
        locals.var_fn133_calc_iq__vdsc_dn5 = assign11090_e11485_d_n5;
        locals.var_fn133_calc_iq__vdsc_dn7 = assign11090_e11485_d_n7;
        locals.var_fn133_calc_iq__vdsc_dn14 = assign11090_e11485_d_n14;

        let (assign11120_e11497, assign11120_e11497_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vdsats0, locals.var_fn133_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn133_calc_iq__vdsats0 = assign11120_e11497;
        locals.var_fn133_calc_iq__vdsats0_dn4 = assign11120_e11497_d_n4;

        let (assign11130_e11501, assign11130_e11501_d_n2, assign11130_e11501_d_n4, assign11130_e11501_d_n5, assign11130_e11501_d_n7, assign11130_e11501_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vdsats10, locals.var_fn133_calc_iq__vdsats10_dn2, locals.var_fn133_calc_iq__vdsats10_dn4, locals.var_fn133_calc_iq__vdsats10_dn5, locals.var_fn133_calc_iq__vdsats10_dn7, locals.var_fn133_calc_iq__vdsats10_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsats10 = assign11130_e11501;
        locals.var_fn133_calc_iq__vdsats10_dn2 = assign11130_e11501_d_n2;
        locals.var_fn133_calc_iq__vdsats10_dn4 = assign11130_e11501_d_n4;
        locals.var_fn133_calc_iq__vdsats10_dn5 = assign11130_e11501_d_n5;
        locals.var_fn133_calc_iq__vdsats10_dn7 = assign11130_e11501_d_n7;
        locals.var_fn133_calc_iq__vdsats10_dn14 = assign11130_e11501_d_n14;

        let (assign11140_e11505, assign11140_e11505_d_n2, assign11140_e11505_d_n4, assign11140_e11505_d_n5, assign11140_e11505_d_n7, assign11140_e11505_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vdsat10, locals.var_fn133_calc_iq__vdsat10_dn2, locals.var_fn133_calc_iq__vdsat10_dn4, locals.var_fn133_calc_iq__vdsat10_dn5, locals.var_fn133_calc_iq__vdsat10_dn7, locals.var_fn133_calc_iq__vdsat10_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsat10 = assign11140_e11505;
        locals.var_fn133_calc_iq__vdsat10_dn2 = assign11140_e11505_d_n2;
        locals.var_fn133_calc_iq__vdsat10_dn4 = assign11140_e11505_d_n4;
        locals.var_fn133_calc_iq__vdsat10_dn5 = assign11140_e11505_d_n5;
        locals.var_fn133_calc_iq__vdsat10_dn7 = assign11140_e11505_d_n7;
        locals.var_fn133_calc_iq__vdsat10_dn14 = assign11140_e11505_d_n14;

        let (assign11150_e11509, assign11150_e11509_d_n2, assign11150_e11509_d_n4, assign11150_e11509_d_n5, assign11150_e11509_d_n7, assign11150_e11509_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__fsd0, locals.var_fn133_calc_iq__fsd0_dn2, locals.var_fn133_calc_iq__fsd0_dn4, locals.var_fn133_calc_iq__fsd0_dn5, locals.var_fn133_calc_iq__fsd0_dn7, locals.var_fn133_calc_iq__fsd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__fsd0 = assign11150_e11509;
        locals.var_fn133_calc_iq__fsd0_dn2 = assign11150_e11509_d_n2;
        locals.var_fn133_calc_iq__fsd0_dn4 = assign11150_e11509_d_n4;
        locals.var_fn133_calc_iq__fsd0_dn5 = assign11150_e11509_d_n5;
        locals.var_fn133_calc_iq__fsd0_dn7 = assign11150_e11509_d_n7;
        locals.var_fn133_calc_iq__fsd0_dn14 = assign11150_e11509_d_n14;

        let (assign11160_e11513, assign11160_e11513_d_n2, assign11160_e11513_d_n4, assign11160_e11513_d_n5, assign11160_e11513_d_n7, assign11160_e11513_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vdx0, locals.var_fn133_calc_iq__vdx0_dn2, locals.var_fn133_calc_iq__vdx0_dn4, locals.var_fn133_calc_iq__vdx0_dn5, locals.var_fn133_calc_iq__vdx0_dn7, locals.var_fn133_calc_iq__vdx0_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdx0 = assign11160_e11513;
        locals.var_fn133_calc_iq__vdx0_dn2 = assign11160_e11513_d_n2;
        locals.var_fn133_calc_iq__vdx0_dn4 = assign11160_e11513_d_n4;
        locals.var_fn133_calc_iq__vdx0_dn5 = assign11160_e11513_d_n5;
        locals.var_fn133_calc_iq__vdx0_dn7 = assign11160_e11513_d_n7;
        locals.var_fn133_calc_iq__vdx0_dn14 = assign11160_e11513_d_n14;

        let (assign11170_e11517, assign11170_e11517_d_n2, assign11170_e11517_d_n4, assign11170_e11517_d_n5, assign11170_e11517_d_n7, assign11170_e11517_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__fds0, locals.var_fn133_calc_iq__fds0_dn2, locals.var_fn133_calc_iq__fds0_dn4, locals.var_fn133_calc_iq__fds0_dn5, locals.var_fn133_calc_iq__fds0_dn7, locals.var_fn133_calc_iq__fds0_dn14,)
    }
};
        locals.var_fn133_calc_iq__fds0 = assign11170_e11517;
        locals.var_fn133_calc_iq__fds0_dn2 = assign11170_e11517_d_n2;
        locals.var_fn133_calc_iq__fds0_dn4 = assign11170_e11517_d_n4;
        locals.var_fn133_calc_iq__fds0_dn5 = assign11170_e11517_d_n5;
        locals.var_fn133_calc_iq__fds0_dn7 = assign11170_e11517_d_n7;
        locals.var_fn133_calc_iq__fds0_dn14 = assign11170_e11517_d_n14;

        let (assign11180_e11521, assign11180_e11521_d_n2, assign11180_e11521_d_n4, assign11180_e11521_d_n5, assign11180_e11521_d_n7, assign11180_e11521_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vsx0, locals.var_fn133_calc_iq__vsx0_dn2, locals.var_fn133_calc_iq__vsx0_dn4, locals.var_fn133_calc_iq__vsx0_dn5, locals.var_fn133_calc_iq__vsx0_dn7, locals.var_fn133_calc_iq__vsx0_dn14,)
    }
};
        locals.var_fn133_calc_iq__vsx0 = assign11180_e11521;
        locals.var_fn133_calc_iq__vsx0_dn2 = assign11180_e11521_d_n2;
        locals.var_fn133_calc_iq__vsx0_dn4 = assign11180_e11521_d_n4;
        locals.var_fn133_calc_iq__vsx0_dn5 = assign11180_e11521_d_n5;
        locals.var_fn133_calc_iq__vsx0_dn7 = assign11180_e11521_d_n7;
        locals.var_fn133_calc_iq__vsx0_dn14 = assign11180_e11521_d_n14;

        let (assign11190_e11525, assign11190_e11525_d_n2, assign11190_e11525_d_n4, assign11190_e11525_d_n5, assign11190_e11525_d_n7, assign11190_e11525_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffd0, locals.var_fn133_calc_iq__ffd0_dn2, locals.var_fn133_calc_iq__ffd0_dn4, locals.var_fn133_calc_iq__ffd0_dn5, locals.var_fn133_calc_iq__ffd0_dn7, locals.var_fn133_calc_iq__ffd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffd0 = assign11190_e11525;
        locals.var_fn133_calc_iq__ffd0_dn2 = assign11190_e11525_d_n2;
        locals.var_fn133_calc_iq__ffd0_dn4 = assign11190_e11525_d_n4;
        locals.var_fn133_calc_iq__ffd0_dn5 = assign11190_e11525_d_n5;
        locals.var_fn133_calc_iq__ffd0_dn7 = assign11190_e11525_d_n7;
        locals.var_fn133_calc_iq__ffd0_dn14 = assign11190_e11525_d_n14;

        let (assign11200_e11529, assign11200_e11529_d_n2, assign11200_e11529_d_n4, assign11200_e11529_d_n5, assign11200_e11529_d_n7, assign11200_e11529_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__etad0, locals.var_fn133_calc_iq__etad0_dn2, locals.var_fn133_calc_iq__etad0_dn4, locals.var_fn133_calc_iq__etad0_dn5, locals.var_fn133_calc_iq__etad0_dn7, locals.var_fn133_calc_iq__etad0_dn14,)
    }
};
        locals.var_fn133_calc_iq__etad0 = assign11200_e11529;
        locals.var_fn133_calc_iq__etad0_dn2 = assign11200_e11529_d_n2;
        locals.var_fn133_calc_iq__etad0_dn4 = assign11200_e11529_d_n4;
        locals.var_fn133_calc_iq__etad0_dn5 = assign11200_e11529_d_n5;
        locals.var_fn133_calc_iq__etad0_dn7 = assign11200_e11529_d_n7;
        locals.var_fn133_calc_iq__etad0_dn14 = assign11200_e11529_d_n14;

        let (assign11210_e11533, assign11210_e11533_d_n2, assign11210_e11533_d_n4, assign11210_e11533_d_n5, assign11210_e11533_d_n7, assign11210_e11533_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qinvd0, locals.var_fn133_calc_iq__qinvd0_dn2, locals.var_fn133_calc_iq__qinvd0_dn4, locals.var_fn133_calc_iq__qinvd0_dn5, locals.var_fn133_calc_iq__qinvd0_dn7, locals.var_fn133_calc_iq__qinvd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd0 = assign11210_e11533;
        locals.var_fn133_calc_iq__qinvd0_dn2 = assign11210_e11533_d_n2;
        locals.var_fn133_calc_iq__qinvd0_dn4 = assign11210_e11533_d_n4;
        locals.var_fn133_calc_iq__qinvd0_dn5 = assign11210_e11533_d_n5;
        locals.var_fn133_calc_iq__qinvd0_dn7 = assign11210_e11533_d_n7;
        locals.var_fn133_calc_iq__qinvd0_dn14 = assign11210_e11533_d_n14;

        let (assign11220_e11537, assign11220_e11537_d_n2, assign11220_e11537_d_n4, assign11220_e11537_d_n5, assign11220_e11537_d_n7, assign11220_e11537_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qs2, locals.var_fn133_calc_iq__qs2_dn2, locals.var_fn133_calc_iq__qs2_dn4, locals.var_fn133_calc_iq__qs2_dn5, locals.var_fn133_calc_iq__qs2_dn7, locals.var_fn133_calc_iq__qs2_dn14,)
    }
};
        locals.var_fn133_calc_iq__qs2 = assign11220_e11537;
        locals.var_fn133_calc_iq__qs2_dn2 = assign11220_e11537_d_n2;
        locals.var_fn133_calc_iq__qs2_dn4 = assign11220_e11537_d_n4;
        locals.var_fn133_calc_iq__qs2_dn5 = assign11220_e11537_d_n5;
        locals.var_fn133_calc_iq__qs2_dn7 = assign11220_e11537_d_n7;
        locals.var_fn133_calc_iq__qs2_dn14 = assign11220_e11537_d_n14;

        let (assign11230_e11541, assign11230_e11541_d_n2, assign11230_e11541_d_n4, assign11230_e11541_d_n5, assign11230_e11541_d_n7, assign11230_e11541_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qs3, locals.var_fn133_calc_iq__qs3_dn2, locals.var_fn133_calc_iq__qs3_dn4, locals.var_fn133_calc_iq__qs3_dn5, locals.var_fn133_calc_iq__qs3_dn7, locals.var_fn133_calc_iq__qs3_dn14,)
    }
};
        locals.var_fn133_calc_iq__qs3 = assign11230_e11541;
        locals.var_fn133_calc_iq__qs3_dn2 = assign11230_e11541_d_n2;
        locals.var_fn133_calc_iq__qs3_dn4 = assign11230_e11541_d_n4;
        locals.var_fn133_calc_iq__qs3_dn5 = assign11230_e11541_d_n5;
        locals.var_fn133_calc_iq__qs3_dn7 = assign11230_e11541_d_n7;
        locals.var_fn133_calc_iq__qs3_dn14 = assign11230_e11541_d_n14;

        let (assign11240_e11545, assign11240_e11545_d_n2, assign11240_e11545_d_n4, assign11240_e11545_d_n5, assign11240_e11545_d_n7, assign11240_e11545_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qd2, locals.var_fn133_calc_iq__qd2_dn2, locals.var_fn133_calc_iq__qd2_dn4, locals.var_fn133_calc_iq__qd2_dn5, locals.var_fn133_calc_iq__qd2_dn7, locals.var_fn133_calc_iq__qd2_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd2 = assign11240_e11545;
        locals.var_fn133_calc_iq__qd2_dn2 = assign11240_e11545_d_n2;
        locals.var_fn133_calc_iq__qd2_dn4 = assign11240_e11545_d_n4;
        locals.var_fn133_calc_iq__qd2_dn5 = assign11240_e11545_d_n5;
        locals.var_fn133_calc_iq__qd2_dn7 = assign11240_e11545_d_n7;
        locals.var_fn133_calc_iq__qd2_dn14 = assign11240_e11545_d_n14;

        let (assign11250_e11549, assign11250_e11549_d_n2, assign11250_e11549_d_n4, assign11250_e11549_d_n5, assign11250_e11549_d_n7, assign11250_e11549_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qd3, locals.var_fn133_calc_iq__qd3_dn2, locals.var_fn133_calc_iq__qd3_dn4, locals.var_fn133_calc_iq__qd3_dn5, locals.var_fn133_calc_iq__qd3_dn7, locals.var_fn133_calc_iq__qd3_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd3 = assign11250_e11549;
        locals.var_fn133_calc_iq__qd3_dn2 = assign11250_e11549_d_n2;
        locals.var_fn133_calc_iq__qd3_dn4 = assign11250_e11549_d_n4;
        locals.var_fn133_calc_iq__qd3_dn5 = assign11250_e11549_d_n5;
        locals.var_fn133_calc_iq__qd3_dn7 = assign11250_e11549_d_n7;
        locals.var_fn133_calc_iq__qd3_dn14 = assign11250_e11549_d_n14;

        let (assign11260_e11553, assign11260_e11553_d_n2, assign11260_e11553_d_n4, assign11260_e11553_d_n5, assign11260_e11553_d_n7, assign11260_e11553_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qsqd, locals.var_fn133_calc_iq__qsqd_dn2, locals.var_fn133_calc_iq__qsqd_dn4, locals.var_fn133_calc_iq__qsqd_dn5, locals.var_fn133_calc_iq__qsqd_dn7, locals.var_fn133_calc_iq__qsqd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qsqd = assign11260_e11553;
        locals.var_fn133_calc_iq__qsqd_dn2 = assign11260_e11553_d_n2;
        locals.var_fn133_calc_iq__qsqd_dn4 = assign11260_e11553_d_n4;
        locals.var_fn133_calc_iq__qsqd_dn5 = assign11260_e11553_d_n5;
        locals.var_fn133_calc_iq__qsqd_dn7 = assign11260_e11553_d_n7;
        locals.var_fn133_calc_iq__qsqd_dn14 = assign11260_e11553_d_n14;

    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11270_e11557, assign11270_e11557_d_n2, assign11270_e11557_d_n4, assign11270_e11557_d_n5, assign11270_e11557_d_n7, assign11270_e11557_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qinvdd, locals.var_fn133_calc_iq__qinvdd_dn2, locals.var_fn133_calc_iq__qinvdd_dn4, locals.var_fn133_calc_iq__qinvdd_dn5, locals.var_fn133_calc_iq__qinvdd_dn7, locals.var_fn133_calc_iq__qinvdd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvdd = assign11270_e11557;
        locals.var_fn133_calc_iq__qinvdd_dn2 = assign11270_e11557_d_n2;
        locals.var_fn133_calc_iq__qinvdd_dn4 = assign11270_e11557_d_n4;
        locals.var_fn133_calc_iq__qinvdd_dn5 = assign11270_e11557_d_n5;
        locals.var_fn133_calc_iq__qinvdd_dn7 = assign11270_e11557_d_n7;
        locals.var_fn133_calc_iq__qinvdd_dn14 = assign11270_e11557_d_n14;

        let (assign11280_e11561, assign11280_e11561_d_n2, assign11280_e11561_d_n4, assign11280_e11561_d_n5, assign11280_e11561_d_n7, assign11280_e11561_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qd1, locals.var_fn133_calc_iq__qd1_dn2, locals.var_fn133_calc_iq__qd1_dn4, locals.var_fn133_calc_iq__qd1_dn5, locals.var_fn133_calc_iq__qd1_dn7, locals.var_fn133_calc_iq__qd1_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd1 = assign11280_e11561;
        locals.var_fn133_calc_iq__qd1_dn2 = assign11280_e11561_d_n2;
        locals.var_fn133_calc_iq__qd1_dn4 = assign11280_e11561_d_n4;
        locals.var_fn133_calc_iq__qd1_dn5 = assign11280_e11561_d_n5;
        locals.var_fn133_calc_iq__qd1_dn7 = assign11280_e11561_d_n7;
        locals.var_fn133_calc_iq__qd1_dn14 = assign11280_e11561_d_n14;

        let (assign11290_e11565, assign11290_e11565_d_n2, assign11290_e11565_d_n4, assign11290_e11565_d_n5, assign11290_e11565_d_n7, assign11290_e11565_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qs, locals.var_fn133_calc_iq__qs_dn2, locals.var_fn133_calc_iq__qs_dn4, locals.var_fn133_calc_iq__qs_dn5, locals.var_fn133_calc_iq__qs_dn7, locals.var_fn133_calc_iq__qs_dn14,)
    }
};
        locals.var_fn133_calc_iq__qs = assign11290_e11565;
        locals.var_fn133_calc_iq__qs_dn2 = assign11290_e11565_d_n2;
        locals.var_fn133_calc_iq__qs_dn4 = assign11290_e11565_d_n4;
        locals.var_fn133_calc_iq__qs_dn5 = assign11290_e11565_d_n5;
        locals.var_fn133_calc_iq__qs_dn7 = assign11290_e11565_d_n7;
        locals.var_fn133_calc_iq__qs_dn14 = assign11290_e11565_d_n14;

        let (assign11300_e11569, assign11300_e11569_d_n2, assign11300_e11569_d_n4, assign11300_e11569_d_n5, assign11300_e11569_d_n7, assign11300_e11569_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qd, locals.var_fn133_calc_iq__qd_dn2, locals.var_fn133_calc_iq__qd_dn4, locals.var_fn133_calc_iq__qd_dn5, locals.var_fn133_calc_iq__qd_dn7, locals.var_fn133_calc_iq__qd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd = assign11300_e11569;
        locals.var_fn133_calc_iq__qd_dn2 = assign11300_e11569_d_n2;
        locals.var_fn133_calc_iq__qd_dn4 = assign11300_e11569_d_n4;
        locals.var_fn133_calc_iq__qd_dn5 = assign11300_e11569_d_n5;
        locals.var_fn133_calc_iq__qd_dn7 = assign11300_e11569_d_n7;
        locals.var_fn133_calc_iq__qd_dn14 = assign11300_e11569_d_n14;

        let (assign11310_e11573, assign11310_e11573_d_n2, assign11310_e11573_d_n4, assign11310_e11573_d_n5, assign11310_e11573_d_n7,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__etac, locals.var_fn133_calc_iq__etac_dn2, locals.var_fn133_calc_iq__etac_dn4, locals.var_fn133_calc_iq__etac_dn5, locals.var_fn133_calc_iq__etac_dn7,)
    }
};
        locals.var_fn133_calc_iq__etac = assign11310_e11573;
        locals.var_fn133_calc_iq__etac_dn2 = assign11310_e11573_d_n2;
        locals.var_fn133_calc_iq__etac_dn4 = assign11310_e11573_d_n4;
        locals.var_fn133_calc_iq__etac_dn5 = assign11310_e11573_d_n5;
        locals.var_fn133_calc_iq__etac_dn7 = assign11310_e11573_d_n7;

        let (assign11320_e11577, assign11320_e11577_d_n3, assign11320_e11577_d_n4, assign11320_e11577_d_n5,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__etab, locals.var_fn133_calc_iq__etab_dn3, locals.var_fn133_calc_iq__etab_dn4, locals.var_fn133_calc_iq__etab_dn5,)
    }
};
        locals.var_fn133_calc_iq__etab = assign11320_e11577;
        locals.var_fn133_calc_iq__etab_dn3 = assign11320_e11577_d_n3;
        locals.var_fn133_calc_iq__etab_dn4 = assign11320_e11577_d_n4;
        locals.var_fn133_calc_iq__etab_dn5 = assign11320_e11577_d_n5;

        let (assign11330_e11581, assign11330_e11581_d_n2, assign11330_e11581_d_n4, assign11330_e11581_d_n5, assign11330_e11581_d_n7,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__etags, locals.var_fn133_calc_iq__etags_dn2, locals.var_fn133_calc_iq__etags_dn4, locals.var_fn133_calc_iq__etags_dn5, locals.var_fn133_calc_iq__etags_dn7,)
    }
};
        locals.var_fn133_calc_iq__etags = assign11330_e11581;
        locals.var_fn133_calc_iq__etags_dn2 = assign11330_e11581_d_n2;
        locals.var_fn133_calc_iq__etags_dn4 = assign11330_e11581_d_n4;
        locals.var_fn133_calc_iq__etags_dn5 = assign11330_e11581_d_n5;
        locals.var_fn133_calc_iq__etags_dn7 = assign11330_e11581_d_n7;

        let (assign11340_e11585, assign11340_e11585_d_n2, assign11340_e11585_d_n3, assign11340_e11585_d_n4, assign11340_e11585_d_n5, assign11340_e11585_d_n7, assign11340_e11585_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign11340_e11585;
        locals.var_fn133_calc_iq__exparg_dn2 = assign11340_e11585_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign11340_e11585_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign11340_e11585_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign11340_e11585_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign11340_e11585_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign11340_e11585_d_n14;

        let (assign11350_e11589, assign11350_e11589_d_n2, assign11350_e11589_d_n3, assign11350_e11589_d_n4, assign11350_e11589_d_n5, assign11350_e11589_d_n7, assign11350_e11589_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__myarg, locals.var_fn133_calc_iq__myarg_dn2, locals.var_fn133_calc_iq__myarg_dn3, locals.var_fn133_calc_iq__myarg_dn4, locals.var_fn133_calc_iq__myarg_dn5, locals.var_fn133_calc_iq__myarg_dn7, locals.var_fn133_calc_iq__myarg_dn14,)
    }
};
        locals.var_fn133_calc_iq__myarg = assign11350_e11589;
        locals.var_fn133_calc_iq__myarg_dn2 = assign11350_e11589_d_n2;
        locals.var_fn133_calc_iq__myarg_dn3 = assign11350_e11589_d_n3;
        locals.var_fn133_calc_iq__myarg_dn4 = assign11350_e11589_d_n4;
        locals.var_fn133_calc_iq__myarg_dn5 = assign11350_e11589_d_n5;
        locals.var_fn133_calc_iq__myarg_dn7 = assign11350_e11589_d_n7;
        locals.var_fn133_calc_iq__myarg_dn14 = assign11350_e11589_d_n14;

        let (assign11360_e11593, assign11360_e11593_d_n5, assign11360_e11593_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__absvdsin, locals.var_fn133_calc_iq__absvdsin_dn5, locals.var_fn133_calc_iq__absvdsin_dn14,)
    }
};
        locals.var_fn133_calc_iq__absvdsin = assign11360_e11593;
        locals.var_fn133_calc_iq__absvdsin_dn5 = assign11360_e11593_d_n5;
        locals.var_fn133_calc_iq__absvdsin_dn14 = assign11360_e11593_d_n14;

        let (assign11370_e11597, assign11370_e11597_d_n2, assign11370_e11597_d_n5, assign11370_e11597_d_n7, assign11370_e11597_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vgdin, locals.var_fn133_calc_iq__vgdin_dn2, locals.var_fn133_calc_iq__vgdin_dn5, locals.var_fn133_calc_iq__vgdin_dn7, locals.var_fn133_calc_iq__vgdin_dn14,)
    }
};
        locals.var_fn133_calc_iq__vgdin = assign11370_e11597;
        locals.var_fn133_calc_iq__vgdin_dn2 = assign11370_e11597_d_n2;
        locals.var_fn133_calc_iq__vgdin_dn5 = assign11370_e11597_d_n5;
        locals.var_fn133_calc_iq__vgdin_dn7 = assign11370_e11597_d_n7;
        locals.var_fn133_calc_iq__vgdin_dn14 = assign11370_e11597_d_n14;

        let (assign11380_e11601, assign11380_e11601_d_n2, assign11380_e11601_d_n4, assign11380_e11601_d_n5, assign11380_e11601_d_n7, assign11380_e11601_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg0, locals.var_fn133_calc_iq__exparg0_dn2, locals.var_fn133_calc_iq__exparg0_dn4, locals.var_fn133_calc_iq__exparg0_dn5, locals.var_fn133_calc_iq__exparg0_dn7, locals.var_fn133_calc_iq__exparg0_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg0 = assign11380_e11601;
        locals.var_fn133_calc_iq__exparg0_dn2 = assign11380_e11601_d_n2;
        locals.var_fn133_calc_iq__exparg0_dn4 = assign11380_e11601_d_n4;
        locals.var_fn133_calc_iq__exparg0_dn5 = assign11380_e11601_d_n5;
        locals.var_fn133_calc_iq__exparg0_dn7 = assign11380_e11601_d_n7;
        locals.var_fn133_calc_iq__exparg0_dn14 = assign11380_e11601_d_n14;

        let (assign11390_e11605, assign11390_e11605_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__myarg0, locals.var_fn133_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn133_calc_iq__myarg0 = assign11390_e11605;
        locals.var_fn133_calc_iq__myarg0_dn4 = assign11390_e11605_d_n4;

        let (assign11400_e11632, assign11400_e11632_d_n5, assign11400_e11632_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign11400_e11630, assign11400_e11630_d_n5, assign11400_e11630_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign11400_e11614: f64 = (0.001 / p.p53);
                let assign11400_e11616: f64 = (assign11400_e11614 * locals.var_fn133_calc_iq__vdsin);
                let assign11400_e11617: f64 = (assign11400_e11616).tanh();
                let assign11400_e11618: f64 = (locals.var_fn133_calc_iq__vdsin * assign11400_e11617);
                (assign11400_e11618, ((locals.var_fn133_calc_iq__vdsin_dn5 * assign11400_e11617) + (locals.var_fn133_calc_iq__vdsin * ((assign11400_e11614 * locals.var_fn133_calc_iq__vdsin_dn5) / ((assign11400_e11616).cosh() * (assign11400_e11616).cosh())))), ((locals.var_fn133_calc_iq__vdsin_dn14 * assign11400_e11617) + (locals.var_fn133_calc_iq__vdsin * ((assign11400_e11614 * locals.var_fn133_calc_iq__vdsin_dn14) / ((assign11400_e11616).cosh() * (assign11400_e11616).cosh())))),)
            } else {
                let (assign11400_e11629, assign11400_e11629_d_n5, assign11400_e11629_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign11400_e11624: f64 = (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsin);
                        let assign11400_e11626: f64 = (assign11400_e11624 + p.p53);
                        let assign11400_e11627: f64 = (assign11400_e11626).sqrt();
                        (assign11400_e11627, (((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsin) + (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsin_dn5)) / (2.0 * assign11400_e11627)), (((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsin) + (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsin_dn14)) / (2.0 * assign11400_e11627)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign11400_e11629, assign11400_e11629_d_n5, assign11400_e11629_d_n14,)
            }
        };
        (assign11400_e11630, assign11400_e11630_d_n5, assign11400_e11630_d_n14,)
    } else {
        (locals.var_fn133_calc_iq__absvdsin, locals.var_fn133_calc_iq__absvdsin_dn5, locals.var_fn133_calc_iq__absvdsin_dn14,)
    }
};
        locals.var_fn133_calc_iq__absvdsin = assign11400_e11632;
        locals.var_fn133_calc_iq__absvdsin_dn5 = assign11400_e11632_d_n5;
        locals.var_fn133_calc_iq__absvdsin_dn14 = assign11400_e11632_d_n14;

        let (assign11410_e11638, assign11410_e11638_d_n2, assign11410_e11638_d_n5, assign11410_e11638_d_n7, assign11410_e11638_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11410_e11636: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vdsin);
        (assign11410_e11636, locals.var_fn133_calc_iq__vgsin_dn2, (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vdsin_dn5), locals.var_fn133_calc_iq__vgsin_dn7, (-locals.var_fn133_calc_iq__vdsin_dn14),)
    } else {
        (locals.var_fn133_calc_iq__vgdin, locals.var_fn133_calc_iq__vgdin_dn2, locals.var_fn133_calc_iq__vgdin_dn5, locals.var_fn133_calc_iq__vgdin_dn7, locals.var_fn133_calc_iq__vgdin_dn14,)
    }
};
        locals.var_fn133_calc_iq__vgdin = assign11410_e11638;
        locals.var_fn133_calc_iq__vgdin_dn2 = assign11410_e11638_d_n2;
        locals.var_fn133_calc_iq__vgdin_dn5 = assign11410_e11638_d_n5;
        locals.var_fn133_calc_iq__vgdin_dn7 = assign11410_e11638_d_n7;
        locals.var_fn133_calc_iq__vgdin_dn14 = assign11410_e11638_d_n14;

        let (assign11420_e11644, assign11420_e11644_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11420_e11642: f64 = (locals.var_fn133_calc_iq__alpha * locals.var_fn133_calc_iq__phitin);
        (assign11420_e11642, (locals.var_fn133_calc_iq__alpha * locals.var_fn133_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn133_calc_iq__alpha_phit, locals.var_fn133_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn133_calc_iq__alpha_phit = assign11420_e11644;
        locals.var_fn133_calc_iq__alpha_phit_dn4 = assign11420_e11644_d_n4;

        let (assign11430_e11656, assign11430_e11656_d_n4, assign11430_e11656_d_n5, assign11430_e11656_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11430_e11649: f64 = (2.302585092994046 * locals.var_fn133_calc_iq__phitin);
        let assign11430_e11650: f64 = (locals.var_fn133_calc_iq__ss / assign11430_e11649);
        let assign11430_e11653: f64 = (locals.var_fn133_calc_iq__nd * locals.var_fn133_calc_iq__absvdsin);
        let assign11430_e11654: f64 = (assign11430_e11650 + assign11430_e11653);
        (assign11430_e11654, (-((locals.var_fn133_calc_iq__ss * (2.302585092994046 * locals.var_fn133_calc_iq__phitin_dn4)) / (assign11430_e11649 * assign11430_e11649))), (locals.var_fn133_calc_iq__nd * locals.var_fn133_calc_iq__absvdsin_dn5), (locals.var_fn133_calc_iq__nd * locals.var_fn133_calc_iq__absvdsin_dn14),)
    } else {
        (locals.var_fn133_calc_iq__n, locals.var_fn133_calc_iq__n_dn4, locals.var_fn133_calc_iq__n_dn5, locals.var_fn133_calc_iq__n_dn14,)
    }
};
        locals.var_fn133_calc_iq__n = assign11430_e11656;
        locals.var_fn133_calc_iq__n_dn4 = assign11430_e11656_d_n4;
        locals.var_fn133_calc_iq__n_dn5 = assign11430_e11656_d_n5;
        locals.var_fn133_calc_iq__n_dn14 = assign11430_e11656_d_n14;

        let (assign11440_e11666, assign11440_e11666_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11440_e11662: f64 = (locals.var_fn133_calc_iq__tambin - locals.var_fn133_calc_iq__tnomin);
        let assign11440_e11663: f64 = (locals.var_fn133_calc_iq__vtzeta * assign11440_e11662);
        let assign11440_e11664: f64 = (locals.var_fn133_calc_iq__vto + assign11440_e11663);
        (assign11440_e11664, (locals.var_fn133_calc_iq__vtzeta * locals.var_fn133_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn133_calc_iq__vtof, locals.var_fn133_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn133_calc_iq__vtof = assign11440_e11666;
        locals.var_fn133_calc_iq__vtof_dn4 = assign11440_e11666_d_n4;

        let (assign11450_e11674, assign11450_e11674_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11450_e11670: f64 = (locals.var_fn133_calc_iq__tambin / locals.var_fn133_calc_iq__tnomin);
        let assign11450_e11672: f64 = (assign11450_e11670).powf(locals.var_fn133_calc_iq__epsilon);
        (assign11450_e11672, if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn133_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__epsilon * ((assign11450_e11670).powf(locals.var_fn133_calc_iq__epsilon - 1.0) * (locals.var_fn133_calc_iq__tambin_dn4 / locals.var_fn133_calc_iq__tnomin))) } } else { (assign11450_e11672 * (locals.var_fn133_calc_iq__epsilon * ((locals.var_fn133_calc_iq__tambin_dn4 / locals.var_fn133_calc_iq__tnomin) / assign11450_e11670))) },)
    } else {
        (locals.var_fn133_calc_iq__tfacmobin, locals.var_fn133_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn133_calc_iq__tfacmobin = assign11450_e11674;
        locals.var_fn133_calc_iq__tfacmobin_dn4 = assign11450_e11674_d_n4;

        let assign11460_e11677: f64 = if locals.var_fn133_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign11460_e11677;

        let (assign11470_e11695, assign11470_e11695_d_n5, assign11470_e11695_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign11470_e11685: f64 = (locals.var_fn133_calc_iq__absvdsin / locals.var_fn133_calc_iq__dibsat);
        let assign11470_e11687: f64 = (assign11470_e11685).powf(locals.var_fn133_calc_iq__beta);
        let assign11470_e11688: f64 = (1.0 + assign11470_e11687);
        let assign11470_e11691: f64 = (1.0 / locals.var_fn133_calc_iq__beta);
        let assign11470_e11692: f64 = (assign11470_e11688).powf(assign11470_e11691);
        let assign11470_e11693: f64 = (locals.var_fn133_calc_iq__absvdsin / assign11470_e11692);
        (assign11470_e11693, (((locals.var_fn133_calc_iq__absvdsin_dn5 * assign11470_e11692) - (locals.var_fn133_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign11470_e11691) as f64).is_finite() && ((assign11470_e11691) as f64).fract() == 0.0 { if assign11470_e11691 == 0.0 { 0.0 } else { (assign11470_e11691 * ((assign11470_e11688).powf(assign11470_e11691 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11470_e11685).powf(locals.var_fn133_calc_iq__beta - 1.0) * (locals.var_fn133_calc_iq__absvdsin_dn5 / locals.var_fn133_calc_iq__dibsat))) } } else { (assign11470_e11687 * (locals.var_fn133_calc_iq__beta * ((locals.var_fn133_calc_iq__absvdsin_dn5 / locals.var_fn133_calc_iq__dibsat) / assign11470_e11685))) })) } } else { (assign11470_e11692 * (assign11470_e11691 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11470_e11685).powf(locals.var_fn133_calc_iq__beta - 1.0) * (locals.var_fn133_calc_iq__absvdsin_dn5 / locals.var_fn133_calc_iq__dibsat))) } } else { (assign11470_e11687 * (locals.var_fn133_calc_iq__beta * ((locals.var_fn133_calc_iq__absvdsin_dn5 / locals.var_fn133_calc_iq__dibsat) / assign11470_e11685))) } / assign11470_e11688))) })) / (assign11470_e11692 * assign11470_e11692)), (((locals.var_fn133_calc_iq__absvdsin_dn14 * assign11470_e11692) - (locals.var_fn133_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign11470_e11691) as f64).is_finite() && ((assign11470_e11691) as f64).fract() == 0.0 { if assign11470_e11691 == 0.0 { 0.0 } else { (assign11470_e11691 * ((assign11470_e11688).powf(assign11470_e11691 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11470_e11685).powf(locals.var_fn133_calc_iq__beta - 1.0) * (locals.var_fn133_calc_iq__absvdsin_dn14 / locals.var_fn133_calc_iq__dibsat))) } } else { (assign11470_e11687 * (locals.var_fn133_calc_iq__beta * ((locals.var_fn133_calc_iq__absvdsin_dn14 / locals.var_fn133_calc_iq__dibsat) / assign11470_e11685))) })) } } else { (assign11470_e11692 * (assign11470_e11691 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11470_e11685).powf(locals.var_fn133_calc_iq__beta - 1.0) * (locals.var_fn133_calc_iq__absvdsin_dn14 / locals.var_fn133_calc_iq__dibsat))) } } else { (assign11470_e11687 * (locals.var_fn133_calc_iq__beta * ((locals.var_fn133_calc_iq__absvdsin_dn14 / locals.var_fn133_calc_iq__dibsat) / assign11470_e11685))) } / assign11470_e11688))) })) / (assign11470_e11692 * assign11470_e11692)),)
    } else {
        (locals.var_fn133_calc_iq__vsatdibl, locals.var_fn133_calc_iq__vsatdibl_dn5, locals.var_fn133_calc_iq__vsatdibl_dn14,)
    }
};
        locals.var_fn133_calc_iq__vsatdibl = assign11470_e11695;
        locals.var_fn133_calc_iq__vsatdibl_dn5 = assign11470_e11695_d_n5;
        locals.var_fn133_calc_iq__vsatdibl_dn14 = assign11470_e11695_d_n14;

        let (assign11480_e11702, assign11480_e11702_d_n5, assign11480_e11702_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard134 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vsatdibl, locals.var_fn133_calc_iq__vsatdibl_dn5, locals.var_fn133_calc_iq__vsatdibl_dn14,)
    }
};
        locals.var_fn133_calc_iq__vsatdibl = assign11480_e11702;
        locals.var_fn133_calc_iq__vsatdibl_dn5 = assign11480_e11702_d_n5;
        locals.var_fn133_calc_iq__vsatdibl_dn14 = assign11480_e11702_d_n14;

        let (assign11490_e11712, assign11490_e11712_d_n5, assign11490_e11712_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11490_e11707: f64 = (locals.var_fn133_calc_iq__vsatdibl * locals.var_fn133_calc_iq__delta2);
        let assign11490_e11708: f64 = (locals.var_fn133_calc_iq__delta1 - assign11490_e11707);
        let assign11490_e11710: f64 = (assign11490_e11708 * locals.var_fn133_calc_iq__absvdsin);
        (assign11490_e11710, (((-(locals.var_fn133_calc_iq__vsatdibl_dn5 * locals.var_fn133_calc_iq__delta2)) * locals.var_fn133_calc_iq__absvdsin) + (assign11490_e11708 * locals.var_fn133_calc_iq__absvdsin_dn5)), (((-(locals.var_fn133_calc_iq__vsatdibl_dn14 * locals.var_fn133_calc_iq__delta2)) * locals.var_fn133_calc_iq__absvdsin) + (assign11490_e11708 * locals.var_fn133_calc_iq__absvdsin_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__delta, locals.var_fn133_calc_iq__delta_dn5, locals.var_fn133_calc_iq__delta_dn14,)
    }
};
        locals.var_fn133_calc_iq__delta = assign11490_e11712;
        locals.var_fn133_calc_iq__delta_dn5 = assign11490_e11712_d_n5;
        locals.var_fn133_calc_iq__delta_dn14 = assign11490_e11712_d_n14;

        let (assign11500_e11718, assign11500_e11718_d_n4, assign11500_e11718_d_n5, assign11500_e11718_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11500_e11716: f64 = (locals.var_fn133_calc_iq__vtof - locals.var_fn133_calc_iq__delta);
        (assign11500_e11716, locals.var_fn133_calc_iq__vtof_dn4, (-locals.var_fn133_calc_iq__delta_dn5), (-locals.var_fn133_calc_iq__delta_dn14),)
    } else {
        (locals.var_fn133_calc_iq__vtdibl, locals.var_fn133_calc_iq__vtdibl_dn4, locals.var_fn133_calc_iq__vtdibl_dn5, locals.var_fn133_calc_iq__vtdibl_dn14,)
    }
};
        locals.var_fn133_calc_iq__vtdibl = assign11500_e11718;
        locals.var_fn133_calc_iq__vtdibl_dn4 = assign11500_e11718_d_n4;
        locals.var_fn133_calc_iq__vtdibl_dn5 = assign11500_e11718_d_n5;
        locals.var_fn133_calc_iq__vtdibl_dn14 = assign11500_e11718_d_n14;

        let (assign11510_e11726, assign11510_e11726_d_n4, assign11510_e11726_d_n5, assign11510_e11726_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11510_e11722: f64 = (2.0 * locals.var_fn133_calc_iq__n);
        let assign11510_e11724: f64 = (assign11510_e11722 * locals.var_fn133_calc_iq__phitin);
        (assign11510_e11724, (((2.0 * locals.var_fn133_calc_iq__n_dn4) * locals.var_fn133_calc_iq__phitin) + (assign11510_e11722 * locals.var_fn133_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn133_calc_iq__n_dn5) * locals.var_fn133_calc_iq__phitin), ((2.0 * locals.var_fn133_calc_iq__n_dn14) * locals.var_fn133_calc_iq__phitin),)
    } else {
        (locals.var_fn133_calc_iq__two_n_phit, locals.var_fn133_calc_iq__two_n_phit_dn4, locals.var_fn133_calc_iq__two_n_phit_dn5, locals.var_fn133_calc_iq__two_n_phit_dn14,)
    }
};
        locals.var_fn133_calc_iq__two_n_phit = assign11510_e11726;
        locals.var_fn133_calc_iq__two_n_phit_dn4 = assign11510_e11726_d_n4;
        locals.var_fn133_calc_iq__two_n_phit_dn5 = assign11510_e11726_d_n5;
        locals.var_fn133_calc_iq__two_n_phit_dn14 = assign11510_e11726_d_n14;

        let (assign11520_e11732, assign11520_e11732_d_n4, assign11520_e11732_d_n5, assign11520_e11732_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11520_e11730: f64 = (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__two_n_phit);
        (assign11520_e11730, ((locals.var_fn133_calc_iq__cgin_dn4 * locals.var_fn133_calc_iq__two_n_phit) + (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__two_n_phit_dn4)), (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__two_n_phit_dn5), (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__two_n_phit_dn14),)
    } else {
        (locals.var_fn133_calc_iq__qref, locals.var_fn133_calc_iq__qref_dn4, locals.var_fn133_calc_iq__qref_dn5, locals.var_fn133_calc_iq__qref_dn14,)
    }
};
        locals.var_fn133_calc_iq__qref = assign11520_e11732;
        locals.var_fn133_calc_iq__qref_dn4 = assign11520_e11732_d_n4;
        locals.var_fn133_calc_iq__qref_dn5 = assign11520_e11732_d_n5;
        locals.var_fn133_calc_iq__qref_dn14 = assign11520_e11732_d_n14;

        let (assign11530_e11742, assign11530_e11742_d_n2, assign11530_e11742_d_n3, assign11530_e11742_d_n4, assign11530_e11742_d_n5, assign11530_e11742_d_n7, assign11530_e11742_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11530_e11737: f64 = (p.p51 * locals.var_fn133_calc_iq__alpha_phit);
        let assign11530_e11739: f64 = (assign11530_e11737 / 2.0);
        let assign11530_e11740: f64 = (locals.var_fn133_calc_iq__vtdibl - assign11530_e11739);
        (assign11530_e11740, 0.0, 0.0, (locals.var_fn133_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn133_calc_iq__alpha_phit_dn4) / 2.0)), locals.var_fn133_calc_iq__vtdibl_dn5, 0.0, locals.var_fn133_calc_iq__vtdibl_dn14,)
    } else {
        (locals.var_fn133_calc_iq__myarg, locals.var_fn133_calc_iq__myarg_dn2, locals.var_fn133_calc_iq__myarg_dn3, locals.var_fn133_calc_iq__myarg_dn4, locals.var_fn133_calc_iq__myarg_dn5, locals.var_fn133_calc_iq__myarg_dn7, locals.var_fn133_calc_iq__myarg_dn14,)
    }
};
        locals.var_fn133_calc_iq__myarg = assign11530_e11742;
        locals.var_fn133_calc_iq__myarg_dn2 = assign11530_e11742_d_n2;
        locals.var_fn133_calc_iq__myarg_dn3 = assign11530_e11742_d_n3;
        locals.var_fn133_calc_iq__myarg_dn4 = assign11530_e11742_d_n4;
        locals.var_fn133_calc_iq__myarg_dn5 = assign11530_e11742_d_n5;
        locals.var_fn133_calc_iq__myarg_dn7 = assign11530_e11742_d_n7;
        locals.var_fn133_calc_iq__myarg_dn14 = assign11530_e11742_d_n14;

        let (assign11540_e11793, assign11540_e11793_d_n2, assign11540_e11793_d_n3, assign11540_e11793_d_n4, assign11540_e11793_d_n5, assign11540_e11793_d_n7, assign11540_e11793_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign11540_e11787, assign11540_e11787_d_n2, assign11540_e11787_d_n5, assign11540_e11787_d_n7, assign11540_e11787_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign11540_e11751: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                let assign11540_e11754: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign11540_e11757: f64 = (0.001 / p.p53);
                let assign11540_e11760: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign11540_e11761: f64 = (assign11540_e11757 * assign11540_e11760);
                let assign11540_e11762: f64 = (assign11540_e11761).tanh();
                let assign11540_e11763: f64 = (assign11540_e11754 * assign11540_e11762);
                let assign11540_e11764: f64 = (assign11540_e11751 + assign11540_e11763);
                let assign11540_e11765: f64 = (0.5 * assign11540_e11764);
                (assign11540_e11765, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + (((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign11540_e11762) + (assign11540_e11754 * ((assign11540_e11757 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2)) / ((assign11540_e11761).cosh() * (assign11540_e11761).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + (((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign11540_e11762) + (assign11540_e11754 * ((assign11540_e11757 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5)) / ((assign11540_e11761).cosh() * (assign11540_e11761).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + (((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign11540_e11762) + (assign11540_e11754 * ((assign11540_e11757 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7)) / ((assign11540_e11761).cosh() * (assign11540_e11761).cosh())))))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + (((-locals.var_fn133_calc_iq__vgdin_dn14) * assign11540_e11762) + (assign11540_e11754 * ((assign11540_e11757 * (-locals.var_fn133_calc_iq__vgdin_dn14)) / ((assign11540_e11761).cosh() * (assign11540_e11761).cosh())))))),)
            } else {
                let (assign11540_e11786, assign11540_e11786_d_n2, assign11540_e11786_d_n5, assign11540_e11786_d_n7, assign11540_e11786_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign11540_e11772: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                        let assign11540_e11775: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign11540_e11778: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign11540_e11779: f64 = (assign11540_e11775 * assign11540_e11778);
                        let assign11540_e11781: f64 = (assign11540_e11779 + p.p53);
                        let assign11540_e11782: f64 = (assign11540_e11781).sqrt();
                        let assign11540_e11783: f64 = (assign11540_e11772 + assign11540_e11782);
                        let assign11540_e11784: f64 = (0.5 * assign11540_e11783);
                        (assign11540_e11784, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + ((((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign11540_e11778) + (assign11540_e11775 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2))) / (2.0 * assign11540_e11782)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + ((((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign11540_e11778) + (assign11540_e11775 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5))) / (2.0 * assign11540_e11782)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + ((((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign11540_e11778) + (assign11540_e11775 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7))) / (2.0 * assign11540_e11782)))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + ((((-locals.var_fn133_calc_iq__vgdin_dn14) * assign11540_e11778) + (assign11540_e11775 * (-locals.var_fn133_calc_iq__vgdin_dn14))) / (2.0 * assign11540_e11782)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign11540_e11786, assign11540_e11786_d_n2, assign11540_e11786_d_n5, assign11540_e11786_d_n7, assign11540_e11786_d_n14,)
            }
        };
        let assign11540_e11789: f64 = (assign11540_e11787 - locals.var_fn133_calc_iq__myarg);
        let assign11540_e11791: f64 = (assign11540_e11789 / locals.var_fn133_calc_iq__alpha_phit);
        (assign11540_e11791, ((assign11540_e11787_d_n2 - locals.var_fn133_calc_iq__myarg_dn2) / locals.var_fn133_calc_iq__alpha_phit), ((-locals.var_fn133_calc_iq__myarg_dn3) / locals.var_fn133_calc_iq__alpha_phit), ((((-locals.var_fn133_calc_iq__myarg_dn4) * locals.var_fn133_calc_iq__alpha_phit) - (assign11540_e11789 * locals.var_fn133_calc_iq__alpha_phit_dn4)) / (locals.var_fn133_calc_iq__alpha_phit * locals.var_fn133_calc_iq__alpha_phit)), ((assign11540_e11787_d_n5 - locals.var_fn133_calc_iq__myarg_dn5) / locals.var_fn133_calc_iq__alpha_phit), ((assign11540_e11787_d_n7 - locals.var_fn133_calc_iq__myarg_dn7) / locals.var_fn133_calc_iq__alpha_phit), ((assign11540_e11787_d_n14 - locals.var_fn133_calc_iq__myarg_dn14) / locals.var_fn133_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign11540_e11793;
        locals.var_fn133_calc_iq__exparg_dn2 = assign11540_e11793_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign11540_e11793_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign11540_e11793_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign11540_e11793_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign11540_e11793_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign11540_e11793_d_n14;

        let assign11550_e11796: f64 = if locals.var_fn133_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard135 = assign11550_e11796;

        let (assign11560_e11802, assign11560_e11802_d_n2, assign11560_e11802_d_n3, assign11560_e11802_d_n4, assign11560_e11802_d_n5, assign11560_e11802_d_n7, assign11560_e11802_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard135 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ff, locals.var_fn133_calc_iq__ff_dn2, locals.var_fn133_calc_iq__ff_dn3, locals.var_fn133_calc_iq__ff_dn4, locals.var_fn133_calc_iq__ff_dn5, locals.var_fn133_calc_iq__ff_dn7, locals.var_fn133_calc_iq__ff_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff = assign11560_e11802;
        locals.var_fn133_calc_iq__ff_dn2 = assign11560_e11802_d_n2;
        locals.var_fn133_calc_iq__ff_dn3 = assign11560_e11802_d_n3;
        locals.var_fn133_calc_iq__ff_dn4 = assign11560_e11802_d_n4;
        locals.var_fn133_calc_iq__ff_dn5 = assign11560_e11802_d_n5;
        locals.var_fn133_calc_iq__ff_dn7 = assign11560_e11802_d_n7;
        locals.var_fn133_calc_iq__ff_dn14 = assign11560_e11802_d_n14;

        let assign11570_e11805: f64 = (-50.0);
        let assign11570_e11806: f64 = if locals.var_fn133_calc_iq__exparg < assign11570_e11805 { 1.0 } else { 0.0 };
        locals.var_guard136 = assign11570_e11806;

        let (assign11580_e11815, assign11580_e11815_d_n2, assign11580_e11815_d_n3, assign11580_e11815_d_n4, assign11580_e11815_d_n5, assign11580_e11815_d_n7, assign11580_e11815_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard135 == 0.0)) && (locals.var_guard136 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ff, locals.var_fn133_calc_iq__ff_dn2, locals.var_fn133_calc_iq__ff_dn3, locals.var_fn133_calc_iq__ff_dn4, locals.var_fn133_calc_iq__ff_dn5, locals.var_fn133_calc_iq__ff_dn7, locals.var_fn133_calc_iq__ff_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff = assign11580_e11815;
        locals.var_fn133_calc_iq__ff_dn2 = assign11580_e11815_d_n2;
        locals.var_fn133_calc_iq__ff_dn3 = assign11580_e11815_d_n3;
        locals.var_fn133_calc_iq__ff_dn4 = assign11580_e11815_d_n4;
        locals.var_fn133_calc_iq__ff_dn5 = assign11580_e11815_d_n5;
        locals.var_fn133_calc_iq__ff_dn7 = assign11580_e11815_d_n7;
        locals.var_fn133_calc_iq__ff_dn14 = assign11580_e11815_d_n14;

        let (assign11590_e11830, assign11590_e11830_d_n2, assign11590_e11830_d_n3, assign11590_e11830_d_n4, assign11590_e11830_d_n5, assign11590_e11830_d_n7, assign11590_e11830_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard135 == 0.0)) && (locals.var_guard136 == 0.0)) {
        let assign11590_e11826: f64 = (locals.var_fn133_calc_iq__exparg).exp();
        let assign11590_e11827: f64 = (1.0 + assign11590_e11826);
        let assign11590_e11828: f64 = (1.0 / assign11590_e11827);
        (assign11590_e11828, (-((assign11590_e11826 * locals.var_fn133_calc_iq__exparg_dn2) / (assign11590_e11827 * assign11590_e11827))), (-((assign11590_e11826 * locals.var_fn133_calc_iq__exparg_dn3) / (assign11590_e11827 * assign11590_e11827))), (-((assign11590_e11826 * locals.var_fn133_calc_iq__exparg_dn4) / (assign11590_e11827 * assign11590_e11827))), (-((assign11590_e11826 * locals.var_fn133_calc_iq__exparg_dn5) / (assign11590_e11827 * assign11590_e11827))), (-((assign11590_e11826 * locals.var_fn133_calc_iq__exparg_dn7) / (assign11590_e11827 * assign11590_e11827))), (-((assign11590_e11826 * locals.var_fn133_calc_iq__exparg_dn14) / (assign11590_e11827 * assign11590_e11827))),)
    } else {
        (locals.var_fn133_calc_iq__ff, locals.var_fn133_calc_iq__ff_dn2, locals.var_fn133_calc_iq__ff_dn3, locals.var_fn133_calc_iq__ff_dn4, locals.var_fn133_calc_iq__ff_dn5, locals.var_fn133_calc_iq__ff_dn7, locals.var_fn133_calc_iq__ff_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff = assign11590_e11830;
        locals.var_fn133_calc_iq__ff_dn2 = assign11590_e11830_d_n2;
        locals.var_fn133_calc_iq__ff_dn3 = assign11590_e11830_d_n3;
        locals.var_fn133_calc_iq__ff_dn4 = assign11590_e11830_d_n4;
        locals.var_fn133_calc_iq__ff_dn5 = assign11590_e11830_d_n5;
        locals.var_fn133_calc_iq__ff_dn7 = assign11590_e11830_d_n7;
        locals.var_fn133_calc_iq__ff_dn14 = assign11590_e11830_d_n14;

    }

    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11600_e11889, assign11600_e11889_d_n2, assign11600_e11889_d_n3, assign11600_e11889_d_n4, assign11600_e11889_d_n5, assign11600_e11889_d_n7, assign11600_e11889_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign11600_e11875, assign11600_e11875_d_n2, assign11600_e11875_d_n5, assign11600_e11875_d_n7, assign11600_e11875_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign11600_e11839: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                let assign11600_e11842: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign11600_e11845: f64 = (0.001 / p.p53);
                let assign11600_e11848: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign11600_e11849: f64 = (assign11600_e11845 * assign11600_e11848);
                let assign11600_e11850: f64 = (assign11600_e11849).tanh();
                let assign11600_e11851: f64 = (assign11600_e11842 * assign11600_e11850);
                let assign11600_e11852: f64 = (assign11600_e11839 + assign11600_e11851);
                let assign11600_e11853: f64 = (0.5 * assign11600_e11852);
                (assign11600_e11853, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + (((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign11600_e11850) + (assign11600_e11842 * ((assign11600_e11845 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2)) / ((assign11600_e11849).cosh() * (assign11600_e11849).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + (((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign11600_e11850) + (assign11600_e11842 * ((assign11600_e11845 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5)) / ((assign11600_e11849).cosh() * (assign11600_e11849).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + (((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign11600_e11850) + (assign11600_e11842 * ((assign11600_e11845 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7)) / ((assign11600_e11849).cosh() * (assign11600_e11849).cosh())))))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + (((-locals.var_fn133_calc_iq__vgdin_dn14) * assign11600_e11850) + (assign11600_e11842 * ((assign11600_e11845 * (-locals.var_fn133_calc_iq__vgdin_dn14)) / ((assign11600_e11849).cosh() * (assign11600_e11849).cosh())))))),)
            } else {
                let (assign11600_e11874, assign11600_e11874_d_n2, assign11600_e11874_d_n5, assign11600_e11874_d_n7, assign11600_e11874_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign11600_e11860: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                        let assign11600_e11863: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign11600_e11866: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign11600_e11867: f64 = (assign11600_e11863 * assign11600_e11866);
                        let assign11600_e11869: f64 = (assign11600_e11867 + p.p53);
                        let assign11600_e11870: f64 = (assign11600_e11869).sqrt();
                        let assign11600_e11871: f64 = (assign11600_e11860 + assign11600_e11870);
                        let assign11600_e11872: f64 = (0.5 * assign11600_e11871);
                        (assign11600_e11872, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + ((((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign11600_e11866) + (assign11600_e11863 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2))) / (2.0 * assign11600_e11870)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + ((((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign11600_e11866) + (assign11600_e11863 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5))) / (2.0 * assign11600_e11870)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + ((((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign11600_e11866) + (assign11600_e11863 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7))) / (2.0 * assign11600_e11870)))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + ((((-locals.var_fn133_calc_iq__vgdin_dn14) * assign11600_e11866) + (assign11600_e11863 * (-locals.var_fn133_calc_iq__vgdin_dn14))) / (2.0 * assign11600_e11870)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign11600_e11874, assign11600_e11874_d_n2, assign11600_e11874_d_n5, assign11600_e11874_d_n7, assign11600_e11874_d_n14,)
            }
        };
        let assign11600_e11879: f64 = (p.p51 * 0.1);
        let assign11600_e11881: f64 = (assign11600_e11879 * locals.var_fn133_calc_iq__alpha_phit);
        let assign11600_e11883: f64 = (assign11600_e11881 * locals.var_fn133_calc_iq__ff);
        let assign11600_e11884: f64 = (locals.var_fn133_calc_iq__vtdibl - assign11600_e11883);
        let assign11600_e11885: f64 = (assign11600_e11875 - assign11600_e11884);
        let assign11600_e11887: f64 = (assign11600_e11885 / locals.var_fn133_calc_iq__two_n_phit);
        (assign11600_e11887, ((assign11600_e11875_d_n2 - (-(assign11600_e11881 * locals.var_fn133_calc_iq__ff_dn2))) / locals.var_fn133_calc_iq__two_n_phit), ((-(-(assign11600_e11881 * locals.var_fn133_calc_iq__ff_dn3))) / locals.var_fn133_calc_iq__two_n_phit), ((((-(locals.var_fn133_calc_iq__vtdibl_dn4 - (((assign11600_e11879 * locals.var_fn133_calc_iq__alpha_phit_dn4) * locals.var_fn133_calc_iq__ff) + (assign11600_e11881 * locals.var_fn133_calc_iq__ff_dn4)))) * locals.var_fn133_calc_iq__two_n_phit) - (assign11600_e11885 * locals.var_fn133_calc_iq__two_n_phit_dn4)) / (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__two_n_phit)), ((((assign11600_e11875_d_n5 - (locals.var_fn133_calc_iq__vtdibl_dn5 - (assign11600_e11881 * locals.var_fn133_calc_iq__ff_dn5))) * locals.var_fn133_calc_iq__two_n_phit) - (assign11600_e11885 * locals.var_fn133_calc_iq__two_n_phit_dn5)) / (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__two_n_phit)), ((assign11600_e11875_d_n7 - (-(assign11600_e11881 * locals.var_fn133_calc_iq__ff_dn7))) / locals.var_fn133_calc_iq__two_n_phit), ((((assign11600_e11875_d_n14 - (locals.var_fn133_calc_iq__vtdibl_dn14 - (assign11600_e11881 * locals.var_fn133_calc_iq__ff_dn14))) * locals.var_fn133_calc_iq__two_n_phit) - (assign11600_e11885 * locals.var_fn133_calc_iq__two_n_phit_dn14)) / (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn133_calc_iq__eta, locals.var_fn133_calc_iq__eta_dn2, locals.var_fn133_calc_iq__eta_dn3, locals.var_fn133_calc_iq__eta_dn4, locals.var_fn133_calc_iq__eta_dn5, locals.var_fn133_calc_iq__eta_dn7, locals.var_fn133_calc_iq__eta_dn14,)
    }
};
        locals.var_fn133_calc_iq__eta = assign11600_e11889;
        locals.var_fn133_calc_iq__eta_dn2 = assign11600_e11889_d_n2;
        locals.var_fn133_calc_iq__eta_dn3 = assign11600_e11889_d_n3;
        locals.var_fn133_calc_iq__eta_dn4 = assign11600_e11889_d_n4;
        locals.var_fn133_calc_iq__eta_dn5 = assign11600_e11889_d_n5;
        locals.var_fn133_calc_iq__eta_dn7 = assign11600_e11889_d_n7;
        locals.var_fn133_calc_iq__eta_dn14 = assign11600_e11889_d_n14;

        let assign11610_e11892: f64 = if locals.var_fn133_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard137 = assign11610_e11892;

        let (assign11620_e11900, assign11620_e11900_d_n2, assign11620_e11900_d_n3, assign11620_e11900_d_n4, assign11620_e11900_d_n5, assign11620_e11900_d_n7, assign11620_e11900_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard137 != 0.0)) {
        let assign11620_e11898: f64 = (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__eta);
        (assign11620_e11898, (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__eta_dn2), (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__eta_dn3), ((locals.var_fn133_calc_iq__qref_dn4 * locals.var_fn133_calc_iq__eta) + (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__eta_dn4)), ((locals.var_fn133_calc_iq__qref_dn5 * locals.var_fn133_calc_iq__eta) + (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__eta_dn5)), (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__eta_dn7), ((locals.var_fn133_calc_iq__qref_dn14 * locals.var_fn133_calc_iq__eta) + (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__eta_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qinvv, locals.var_fn133_calc_iq__qinvv_dn2, locals.var_fn133_calc_iq__qinvv_dn3, locals.var_fn133_calc_iq__qinvv_dn4, locals.var_fn133_calc_iq__qinvv_dn5, locals.var_fn133_calc_iq__qinvv_dn7, locals.var_fn133_calc_iq__qinvv_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvv = assign11620_e11900;
        locals.var_fn133_calc_iq__qinvv_dn2 = assign11620_e11900_d_n2;
        locals.var_fn133_calc_iq__qinvv_dn3 = assign11620_e11900_d_n3;
        locals.var_fn133_calc_iq__qinvv_dn4 = assign11620_e11900_d_n4;
        locals.var_fn133_calc_iq__qinvv_dn5 = assign11620_e11900_d_n5;
        locals.var_fn133_calc_iq__qinvv_dn7 = assign11620_e11900_d_n7;
        locals.var_fn133_calc_iq__qinvv_dn14 = assign11620_e11900_d_n14;

        let assign11630_e11903: f64 = (-50.0);
        let assign11630_e11904: f64 = if locals.var_fn133_calc_iq__eta < assign11630_e11903 { 1.0 } else { 0.0 };
        locals.var_guard138 = assign11630_e11904;

        let (assign11640_e11916, assign11640_e11916_d_n2, assign11640_e11916_d_n3, assign11640_e11916_d_n4, assign11640_e11916_d_n5, assign11640_e11916_d_n7, assign11640_e11916_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) {
        let assign11640_e11913: f64 = (locals.var_fn133_calc_iq__eta).exp();
        let assign11640_e11914: f64 = (locals.var_fn133_calc_iq__qref * assign11640_e11913);
        (assign11640_e11914, (locals.var_fn133_calc_iq__qref * (assign11640_e11913 * locals.var_fn133_calc_iq__eta_dn2)), (locals.var_fn133_calc_iq__qref * (assign11640_e11913 * locals.var_fn133_calc_iq__eta_dn3)), ((locals.var_fn133_calc_iq__qref_dn4 * assign11640_e11913) + (locals.var_fn133_calc_iq__qref * (assign11640_e11913 * locals.var_fn133_calc_iq__eta_dn4))), ((locals.var_fn133_calc_iq__qref_dn5 * assign11640_e11913) + (locals.var_fn133_calc_iq__qref * (assign11640_e11913 * locals.var_fn133_calc_iq__eta_dn5))), (locals.var_fn133_calc_iq__qref * (assign11640_e11913 * locals.var_fn133_calc_iq__eta_dn7)), ((locals.var_fn133_calc_iq__qref_dn14 * assign11640_e11913) + (locals.var_fn133_calc_iq__qref * (assign11640_e11913 * locals.var_fn133_calc_iq__eta_dn14))),)
    } else {
        (locals.var_fn133_calc_iq__qinvv, locals.var_fn133_calc_iq__qinvv_dn2, locals.var_fn133_calc_iq__qinvv_dn3, locals.var_fn133_calc_iq__qinvv_dn4, locals.var_fn133_calc_iq__qinvv_dn5, locals.var_fn133_calc_iq__qinvv_dn7, locals.var_fn133_calc_iq__qinvv_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvv = assign11640_e11916;
        locals.var_fn133_calc_iq__qinvv_dn2 = assign11640_e11916_d_n2;
        locals.var_fn133_calc_iq__qinvv_dn3 = assign11640_e11916_d_n3;
        locals.var_fn133_calc_iq__qinvv_dn4 = assign11640_e11916_d_n4;
        locals.var_fn133_calc_iq__qinvv_dn5 = assign11640_e11916_d_n5;
        locals.var_fn133_calc_iq__qinvv_dn7 = assign11640_e11916_d_n7;
        locals.var_fn133_calc_iq__qinvv_dn14 = assign11640_e11916_d_n14;

        let (assign11650_e11932, assign11650_e11932_d_n2, assign11650_e11932_d_n3, assign11650_e11932_d_n4, assign11650_e11932_d_n5, assign11650_e11932_d_n7, assign11650_e11932_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 == 0.0)) {
        let assign11650_e11927: f64 = (locals.var_fn133_calc_iq__eta).exp();
        let assign11650_e11928: f64 = (1.0 + assign11650_e11927);
        let assign11650_e11929: f64 = (assign11650_e11928).ln();
        let assign11650_e11930: f64 = (locals.var_fn133_calc_iq__qref * assign11650_e11929);
        (assign11650_e11930, (locals.var_fn133_calc_iq__qref * ((assign11650_e11927 * locals.var_fn133_calc_iq__eta_dn2) / assign11650_e11928)), (locals.var_fn133_calc_iq__qref * ((assign11650_e11927 * locals.var_fn133_calc_iq__eta_dn3) / assign11650_e11928)), ((locals.var_fn133_calc_iq__qref_dn4 * assign11650_e11929) + (locals.var_fn133_calc_iq__qref * ((assign11650_e11927 * locals.var_fn133_calc_iq__eta_dn4) / assign11650_e11928))), ((locals.var_fn133_calc_iq__qref_dn5 * assign11650_e11929) + (locals.var_fn133_calc_iq__qref * ((assign11650_e11927 * locals.var_fn133_calc_iq__eta_dn5) / assign11650_e11928))), (locals.var_fn133_calc_iq__qref * ((assign11650_e11927 * locals.var_fn133_calc_iq__eta_dn7) / assign11650_e11928)), ((locals.var_fn133_calc_iq__qref_dn14 * assign11650_e11929) + (locals.var_fn133_calc_iq__qref * ((assign11650_e11927 * locals.var_fn133_calc_iq__eta_dn14) / assign11650_e11928))),)
    } else {
        (locals.var_fn133_calc_iq__qinvv, locals.var_fn133_calc_iq__qinvv_dn2, locals.var_fn133_calc_iq__qinvv_dn3, locals.var_fn133_calc_iq__qinvv_dn4, locals.var_fn133_calc_iq__qinvv_dn5, locals.var_fn133_calc_iq__qinvv_dn7, locals.var_fn133_calc_iq__qinvv_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvv = assign11650_e11932;
        locals.var_fn133_calc_iq__qinvv_dn2 = assign11650_e11932_d_n2;
        locals.var_fn133_calc_iq__qinvv_dn3 = assign11650_e11932_d_n3;
        locals.var_fn133_calc_iq__qinvv_dn4 = assign11650_e11932_d_n4;
        locals.var_fn133_calc_iq__qinvv_dn5 = assign11650_e11932_d_n5;
        locals.var_fn133_calc_iq__qinvv_dn7 = assign11650_e11932_d_n7;
        locals.var_fn133_calc_iq__qinvv_dn14 = assign11650_e11932_d_n14;

        let (assign11660_e11946, assign11660_e11946_d_n2, assign11660_e11946_d_n3, assign11660_e11946_d_n4, assign11660_e11946_d_n5, assign11660_e11946_d_n7, assign11660_e11946_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11660_e11939: f64 = (locals.var_fn133_calc_iq__mtheta * locals.var_fn133_calc_iq__qinvv);
        let assign11660_e11941: f64 = (assign11660_e11939 / locals.var_fn133_calc_iq__cgin);
        let assign11660_e11942: f64 = (1.0 + assign11660_e11941);
        let assign11660_e11943: f64 = (locals.var_fn133_calc_iq__tfacmobin * assign11660_e11942);
        let assign11660_e11944: f64 = (locals.var_fn133_calc_iq__mu0 / assign11660_e11943);
        (assign11660_e11944, (-((locals.var_fn133_calc_iq__mu0 * (locals.var_fn133_calc_iq__tfacmobin * ((locals.var_fn133_calc_iq__mtheta * locals.var_fn133_calc_iq__qinvv_dn2) / locals.var_fn133_calc_iq__cgin))) / (assign11660_e11943 * assign11660_e11943))), (-((locals.var_fn133_calc_iq__mu0 * (locals.var_fn133_calc_iq__tfacmobin * ((locals.var_fn133_calc_iq__mtheta * locals.var_fn133_calc_iq__qinvv_dn3) / locals.var_fn133_calc_iq__cgin))) / (assign11660_e11943 * assign11660_e11943))), (-((locals.var_fn133_calc_iq__mu0 * ((locals.var_fn133_calc_iq__tfacmobin_dn4 * assign11660_e11942) + (locals.var_fn133_calc_iq__tfacmobin * ((((locals.var_fn133_calc_iq__mtheta * locals.var_fn133_calc_iq__qinvv_dn4) * locals.var_fn133_calc_iq__cgin) - (assign11660_e11939 * locals.var_fn133_calc_iq__cgin_dn4)) / (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__cgin))))) / (assign11660_e11943 * assign11660_e11943))), (-((locals.var_fn133_calc_iq__mu0 * (locals.var_fn133_calc_iq__tfacmobin * ((locals.var_fn133_calc_iq__mtheta * locals.var_fn133_calc_iq__qinvv_dn5) / locals.var_fn133_calc_iq__cgin))) / (assign11660_e11943 * assign11660_e11943))), (-((locals.var_fn133_calc_iq__mu0 * (locals.var_fn133_calc_iq__tfacmobin * ((locals.var_fn133_calc_iq__mtheta * locals.var_fn133_calc_iq__qinvv_dn7) / locals.var_fn133_calc_iq__cgin))) / (assign11660_e11943 * assign11660_e11943))), (-((locals.var_fn133_calc_iq__mu0 * (locals.var_fn133_calc_iq__tfacmobin * ((locals.var_fn133_calc_iq__mtheta * locals.var_fn133_calc_iq__qinvv_dn14) / locals.var_fn133_calc_iq__cgin))) / (assign11660_e11943 * assign11660_e11943))),)
    } else {
        (locals.var_fn133_calc_iq__muf, locals.var_fn133_calc_iq__muf_dn2, locals.var_fn133_calc_iq__muf_dn3, locals.var_fn133_calc_iq__muf_dn4, locals.var_fn133_calc_iq__muf_dn5, locals.var_fn133_calc_iq__muf_dn7, locals.var_fn133_calc_iq__muf_dn14,)
    }
};
        locals.var_fn133_calc_iq__muf = assign11660_e11946;
        locals.var_fn133_calc_iq__muf_dn2 = assign11660_e11946_d_n2;
        locals.var_fn133_calc_iq__muf_dn3 = assign11660_e11946_d_n3;
        locals.var_fn133_calc_iq__muf_dn4 = assign11660_e11946_d_n4;
        locals.var_fn133_calc_iq__muf_dn5 = assign11660_e11946_d_n5;
        locals.var_fn133_calc_iq__muf_dn7 = assign11660_e11946_d_n7;
        locals.var_fn133_calc_iq__muf_dn14 = assign11660_e11946_d_n14;

        let (assign11670_e11978, assign11670_e11978_d_n2, assign11670_e11978_d_n3, assign11670_e11978_d_n4, assign11670_e11978_d_n5, assign11670_e11978_d_n7, assign11670_e11978_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11670_e11952: f64 = (locals.var_fn133_calc_iq__vzeta * locals.var_fn133_calc_iq__tnomin);
        let assign11670_e11953: f64 = (1.0 + assign11670_e11952);
        let assign11670_e11957: f64 = (locals.var_fn133_calc_iq__vzeta * locals.var_fn133_calc_iq__tambin);
        let assign11670_e11958: f64 = (1.0 + assign11670_e11957);
        let assign11670_e11959: f64 = (assign11670_e11953 / assign11670_e11958);
        let assign11670_e11960: f64 = (locals.var_fn133_calc_iq__vel0 * assign11670_e11959);
        let assign11670_e11964: f64 = (locals.var_fn133_calc_iq__lambda * locals.var_fn133_calc_iq__absvdsin);
        let assign11670_e11966: f64 = (assign11670_e11964 / locals.var_fn133_calc_iq__lin);
        let assign11670_e11967: f64 = (1.0 + assign11670_e11966);
        let assign11670_e11968: f64 = (assign11670_e11960 * assign11670_e11967);
        let assign11670_e11972: f64 = (locals.var_fn133_calc_iq__vtheta * locals.var_fn133_calc_iq__qinvv);
        let assign11670_e11974: f64 = (assign11670_e11972 / locals.var_fn133_calc_iq__cgin);
        let assign11670_e11975: f64 = (1.0 + assign11670_e11974);
        let assign11670_e11976: f64 = (assign11670_e11968 / assign11670_e11975);
        (assign11670_e11976, (-((assign11670_e11968 * ((locals.var_fn133_calc_iq__vtheta * locals.var_fn133_calc_iq__qinvv_dn2) / locals.var_fn133_calc_iq__cgin)) / (assign11670_e11975 * assign11670_e11975))), (-((assign11670_e11968 * ((locals.var_fn133_calc_iq__vtheta * locals.var_fn133_calc_iq__qinvv_dn3) / locals.var_fn133_calc_iq__cgin)) / (assign11670_e11975 * assign11670_e11975))), (((((locals.var_fn133_calc_iq__vel0 * (-((assign11670_e11953 * (locals.var_fn133_calc_iq__vzeta * locals.var_fn133_calc_iq__tambin_dn4)) / (assign11670_e11958 * assign11670_e11958)))) * assign11670_e11967) * assign11670_e11975) - (assign11670_e11968 * ((((locals.var_fn133_calc_iq__vtheta * locals.var_fn133_calc_iq__qinvv_dn4) * locals.var_fn133_calc_iq__cgin) - (assign11670_e11972 * locals.var_fn133_calc_iq__cgin_dn4)) / (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__cgin)))) / (assign11670_e11975 * assign11670_e11975)), ((((assign11670_e11960 * ((locals.var_fn133_calc_iq__lambda * locals.var_fn133_calc_iq__absvdsin_dn5) / locals.var_fn133_calc_iq__lin)) * assign11670_e11975) - (assign11670_e11968 * ((locals.var_fn133_calc_iq__vtheta * locals.var_fn133_calc_iq__qinvv_dn5) / locals.var_fn133_calc_iq__cgin))) / (assign11670_e11975 * assign11670_e11975)), (-((assign11670_e11968 * ((locals.var_fn133_calc_iq__vtheta * locals.var_fn133_calc_iq__qinvv_dn7) / locals.var_fn133_calc_iq__cgin)) / (assign11670_e11975 * assign11670_e11975))), ((((assign11670_e11960 * ((locals.var_fn133_calc_iq__lambda * locals.var_fn133_calc_iq__absvdsin_dn14) / locals.var_fn133_calc_iq__lin)) * assign11670_e11975) - (assign11670_e11968 * ((locals.var_fn133_calc_iq__vtheta * locals.var_fn133_calc_iq__qinvv_dn14) / locals.var_fn133_calc_iq__cgin))) / (assign11670_e11975 * assign11670_e11975)),)
    } else {
        (locals.var_fn133_calc_iq__vx, locals.var_fn133_calc_iq__vx_dn2, locals.var_fn133_calc_iq__vx_dn3, locals.var_fn133_calc_iq__vx_dn4, locals.var_fn133_calc_iq__vx_dn5, locals.var_fn133_calc_iq__vx_dn7, locals.var_fn133_calc_iq__vx_dn14,)
    }
};
        locals.var_fn133_calc_iq__vx = assign11670_e11978;
        locals.var_fn133_calc_iq__vx_dn2 = assign11670_e11978_d_n2;
        locals.var_fn133_calc_iq__vx_dn3 = assign11670_e11978_d_n3;
        locals.var_fn133_calc_iq__vx_dn4 = assign11670_e11978_d_n4;
        locals.var_fn133_calc_iq__vx_dn5 = assign11670_e11978_d_n5;
        locals.var_fn133_calc_iq__vx_dn7 = assign11670_e11978_d_n7;
        locals.var_fn133_calc_iq__vx_dn14 = assign11670_e11978_d_n14;

        let (assign11690_e12004, assign11690_e12004_d_n2, assign11690_e12004_d_n3, assign11690_e12004_d_n4, assign11690_e12004_d_n5, assign11690_e12004_d_n7, assign11690_e12004_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11690_e12000: f64 = (locals.var_fn133_calc_iq__vx * locals.var_fn133_calc_iq__lin);
        let assign11690_e12002: f64 = (assign11690_e12000 / locals.var_fn133_calc_iq__muf);
        (assign11690_e12002, ((((locals.var_fn133_calc_iq__vx_dn2 * locals.var_fn133_calc_iq__lin) * locals.var_fn133_calc_iq__muf) - (assign11690_e12000 * locals.var_fn133_calc_iq__muf_dn2)) / (locals.var_fn133_calc_iq__muf * locals.var_fn133_calc_iq__muf)), ((((locals.var_fn133_calc_iq__vx_dn3 * locals.var_fn133_calc_iq__lin) * locals.var_fn133_calc_iq__muf) - (assign11690_e12000 * locals.var_fn133_calc_iq__muf_dn3)) / (locals.var_fn133_calc_iq__muf * locals.var_fn133_calc_iq__muf)), ((((locals.var_fn133_calc_iq__vx_dn4 * locals.var_fn133_calc_iq__lin) * locals.var_fn133_calc_iq__muf) - (assign11690_e12000 * locals.var_fn133_calc_iq__muf_dn4)) / (locals.var_fn133_calc_iq__muf * locals.var_fn133_calc_iq__muf)), ((((locals.var_fn133_calc_iq__vx_dn5 * locals.var_fn133_calc_iq__lin) * locals.var_fn133_calc_iq__muf) - (assign11690_e12000 * locals.var_fn133_calc_iq__muf_dn5)) / (locals.var_fn133_calc_iq__muf * locals.var_fn133_calc_iq__muf)), ((((locals.var_fn133_calc_iq__vx_dn7 * locals.var_fn133_calc_iq__lin) * locals.var_fn133_calc_iq__muf) - (assign11690_e12000 * locals.var_fn133_calc_iq__muf_dn7)) / (locals.var_fn133_calc_iq__muf * locals.var_fn133_calc_iq__muf)), ((((locals.var_fn133_calc_iq__vx_dn14 * locals.var_fn133_calc_iq__lin) * locals.var_fn133_calc_iq__muf) - (assign11690_e12000 * locals.var_fn133_calc_iq__muf_dn14)) / (locals.var_fn133_calc_iq__muf * locals.var_fn133_calc_iq__muf)),)
    } else {
        (locals.var_fn133_calc_iq__vdsats, locals.var_fn133_calc_iq__vdsats_dn2, locals.var_fn133_calc_iq__vdsats_dn3, locals.var_fn133_calc_iq__vdsats_dn4, locals.var_fn133_calc_iq__vdsats_dn5, locals.var_fn133_calc_iq__vdsats_dn7, locals.var_fn133_calc_iq__vdsats_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsats = assign11690_e12004;
        locals.var_fn133_calc_iq__vdsats_dn2 = assign11690_e12004_d_n2;
        locals.var_fn133_calc_iq__vdsats_dn3 = assign11690_e12004_d_n3;
        locals.var_fn133_calc_iq__vdsats_dn4 = assign11690_e12004_d_n4;
        locals.var_fn133_calc_iq__vdsats_dn5 = assign11690_e12004_d_n5;
        locals.var_fn133_calc_iq__vdsats_dn7 = assign11690_e12004_d_n7;
        locals.var_fn133_calc_iq__vdsats_dn14 = assign11690_e12004_d_n14;

        let (assign11700_e12021, assign11700_e12021_d_n2, assign11700_e12021_d_n3, assign11700_e12021_d_n4, assign11700_e12021_d_n5, assign11700_e12021_d_n7, assign11700_e12021_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11700_e12010: f64 = (2.0 * locals.var_fn133_calc_iq__qinvv);
        let assign11700_e12012: f64 = (assign11700_e12010 / locals.var_fn133_calc_iq__cgin);
        let assign11700_e12014: f64 = (assign11700_e12012 / locals.var_fn133_calc_iq__vdsats);
        let assign11700_e12015: f64 = (1.0 + assign11700_e12014);
        let assign11700_e12016: f64 = (assign11700_e12015).sqrt();
        let assign11700_e12017: f64 = (locals.var_fn133_calc_iq__vdsats * assign11700_e12016);
        let assign11700_e12019: f64 = (assign11700_e12017 - locals.var_fn133_calc_iq__vdsats);
        (assign11700_e12019, (((locals.var_fn133_calc_iq__vdsats_dn2 * assign11700_e12016) + (locals.var_fn133_calc_iq__vdsats * ((((((2.0 * locals.var_fn133_calc_iq__qinvv_dn2) / locals.var_fn133_calc_iq__cgin) * locals.var_fn133_calc_iq__vdsats) - (assign11700_e12012 * locals.var_fn133_calc_iq__vdsats_dn2)) / (locals.var_fn133_calc_iq__vdsats * locals.var_fn133_calc_iq__vdsats)) / (2.0 * assign11700_e12016)))) - locals.var_fn133_calc_iq__vdsats_dn2), (((locals.var_fn133_calc_iq__vdsats_dn3 * assign11700_e12016) + (locals.var_fn133_calc_iq__vdsats * ((((((2.0 * locals.var_fn133_calc_iq__qinvv_dn3) / locals.var_fn133_calc_iq__cgin) * locals.var_fn133_calc_iq__vdsats) - (assign11700_e12012 * locals.var_fn133_calc_iq__vdsats_dn3)) / (locals.var_fn133_calc_iq__vdsats * locals.var_fn133_calc_iq__vdsats)) / (2.0 * assign11700_e12016)))) - locals.var_fn133_calc_iq__vdsats_dn3), (((locals.var_fn133_calc_iq__vdsats_dn4 * assign11700_e12016) + (locals.var_fn133_calc_iq__vdsats * ((((((((2.0 * locals.var_fn133_calc_iq__qinvv_dn4) * locals.var_fn133_calc_iq__cgin) - (assign11700_e12010 * locals.var_fn133_calc_iq__cgin_dn4)) / (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__cgin)) * locals.var_fn133_calc_iq__vdsats) - (assign11700_e12012 * locals.var_fn133_calc_iq__vdsats_dn4)) / (locals.var_fn133_calc_iq__vdsats * locals.var_fn133_calc_iq__vdsats)) / (2.0 * assign11700_e12016)))) - locals.var_fn133_calc_iq__vdsats_dn4), (((locals.var_fn133_calc_iq__vdsats_dn5 * assign11700_e12016) + (locals.var_fn133_calc_iq__vdsats * ((((((2.0 * locals.var_fn133_calc_iq__qinvv_dn5) / locals.var_fn133_calc_iq__cgin) * locals.var_fn133_calc_iq__vdsats) - (assign11700_e12012 * locals.var_fn133_calc_iq__vdsats_dn5)) / (locals.var_fn133_calc_iq__vdsats * locals.var_fn133_calc_iq__vdsats)) / (2.0 * assign11700_e12016)))) - locals.var_fn133_calc_iq__vdsats_dn5), (((locals.var_fn133_calc_iq__vdsats_dn7 * assign11700_e12016) + (locals.var_fn133_calc_iq__vdsats * ((((((2.0 * locals.var_fn133_calc_iq__qinvv_dn7) / locals.var_fn133_calc_iq__cgin) * locals.var_fn133_calc_iq__vdsats) - (assign11700_e12012 * locals.var_fn133_calc_iq__vdsats_dn7)) / (locals.var_fn133_calc_iq__vdsats * locals.var_fn133_calc_iq__vdsats)) / (2.0 * assign11700_e12016)))) - locals.var_fn133_calc_iq__vdsats_dn7), (((locals.var_fn133_calc_iq__vdsats_dn14 * assign11700_e12016) + (locals.var_fn133_calc_iq__vdsats * ((((((2.0 * locals.var_fn133_calc_iq__qinvv_dn14) / locals.var_fn133_calc_iq__cgin) * locals.var_fn133_calc_iq__vdsats) - (assign11700_e12012 * locals.var_fn133_calc_iq__vdsats_dn14)) / (locals.var_fn133_calc_iq__vdsats * locals.var_fn133_calc_iq__vdsats)) / (2.0 * assign11700_e12016)))) - locals.var_fn133_calc_iq__vdsats_dn14),)
    } else {
        (locals.var_fn133_calc_iq__vdsats1, locals.var_fn133_calc_iq__vdsats1_dn2, locals.var_fn133_calc_iq__vdsats1_dn3, locals.var_fn133_calc_iq__vdsats1_dn4, locals.var_fn133_calc_iq__vdsats1_dn5, locals.var_fn133_calc_iq__vdsats1_dn7, locals.var_fn133_calc_iq__vdsats1_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsats1 = assign11700_e12021;
        locals.var_fn133_calc_iq__vdsats1_dn2 = assign11700_e12021_d_n2;
        locals.var_fn133_calc_iq__vdsats1_dn3 = assign11700_e12021_d_n3;
        locals.var_fn133_calc_iq__vdsats1_dn4 = assign11700_e12021_d_n4;
        locals.var_fn133_calc_iq__vdsats1_dn5 = assign11700_e12021_d_n5;
        locals.var_fn133_calc_iq__vdsats1_dn7 = assign11700_e12021_d_n7;
        locals.var_fn133_calc_iq__vdsats1_dn14 = assign11700_e12021_d_n14;

        let (assign11710_e12033, assign11710_e12033_d_n2, assign11710_e12033_d_n3, assign11710_e12033_d_n4, assign11710_e12033_d_n5, assign11710_e12033_d_n7, assign11710_e12033_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11710_e12026: f64 = (1.0 - locals.var_fn133_calc_iq__ff);
        let assign11710_e12027: f64 = (locals.var_fn133_calc_iq__vdsats * assign11710_e12026);
        let assign11710_e12030: f64 = (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff);
        let assign11710_e12031: f64 = (assign11710_e12027 + assign11710_e12030);
        (assign11710_e12031, (((locals.var_fn133_calc_iq__vdsats_dn2 * assign11710_e12026) + (locals.var_fn133_calc_iq__vdsats * (-locals.var_fn133_calc_iq__ff_dn2))) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn2)), (((locals.var_fn133_calc_iq__vdsats_dn3 * assign11710_e12026) + (locals.var_fn133_calc_iq__vdsats * (-locals.var_fn133_calc_iq__ff_dn3))) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn3)), (((locals.var_fn133_calc_iq__vdsats_dn4 * assign11710_e12026) + (locals.var_fn133_calc_iq__vdsats * (-locals.var_fn133_calc_iq__ff_dn4))) + ((locals.var_fn133_calc_iq__two_n_phit_dn4 * locals.var_fn133_calc_iq__ff) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn4))), (((locals.var_fn133_calc_iq__vdsats_dn5 * assign11710_e12026) + (locals.var_fn133_calc_iq__vdsats * (-locals.var_fn133_calc_iq__ff_dn5))) + ((locals.var_fn133_calc_iq__two_n_phit_dn5 * locals.var_fn133_calc_iq__ff) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn5))), (((locals.var_fn133_calc_iq__vdsats_dn7 * assign11710_e12026) + (locals.var_fn133_calc_iq__vdsats * (-locals.var_fn133_calc_iq__ff_dn7))) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn7)), (((locals.var_fn133_calc_iq__vdsats_dn14 * assign11710_e12026) + (locals.var_fn133_calc_iq__vdsats * (-locals.var_fn133_calc_iq__ff_dn14))) + ((locals.var_fn133_calc_iq__two_n_phit_dn14 * locals.var_fn133_calc_iq__ff) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn14))),)
    } else {
        (locals.var_fn133_calc_iq__vdsat, locals.var_fn133_calc_iq__vdsat_dn2, locals.var_fn133_calc_iq__vdsat_dn3, locals.var_fn133_calc_iq__vdsat_dn4, locals.var_fn133_calc_iq__vdsat_dn5, locals.var_fn133_calc_iq__vdsat_dn7, locals.var_fn133_calc_iq__vdsat_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsat = assign11710_e12033;
        locals.var_fn133_calc_iq__vdsat_dn2 = assign11710_e12033_d_n2;
        locals.var_fn133_calc_iq__vdsat_dn3 = assign11710_e12033_d_n3;
        locals.var_fn133_calc_iq__vdsat_dn4 = assign11710_e12033_d_n4;
        locals.var_fn133_calc_iq__vdsat_dn5 = assign11710_e12033_d_n5;
        locals.var_fn133_calc_iq__vdsat_dn7 = assign11710_e12033_d_n7;
        locals.var_fn133_calc_iq__vdsat_dn14 = assign11710_e12033_d_n14;

        let (assign11720_e12045, assign11720_e12045_d_n2, assign11720_e12045_d_n3, assign11720_e12045_d_n4, assign11720_e12045_d_n5, assign11720_e12045_d_n7, assign11720_e12045_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11720_e12038: f64 = (1.0 - locals.var_fn133_calc_iq__ff);
        let assign11720_e12039: f64 = (locals.var_fn133_calc_iq__vdsats1 * assign11720_e12038);
        let assign11720_e12042: f64 = (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff);
        let assign11720_e12043: f64 = (assign11720_e12039 + assign11720_e12042);
        (assign11720_e12043, (((locals.var_fn133_calc_iq__vdsats1_dn2 * assign11720_e12038) + (locals.var_fn133_calc_iq__vdsats1 * (-locals.var_fn133_calc_iq__ff_dn2))) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn2)), (((locals.var_fn133_calc_iq__vdsats1_dn3 * assign11720_e12038) + (locals.var_fn133_calc_iq__vdsats1 * (-locals.var_fn133_calc_iq__ff_dn3))) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn3)), (((locals.var_fn133_calc_iq__vdsats1_dn4 * assign11720_e12038) + (locals.var_fn133_calc_iq__vdsats1 * (-locals.var_fn133_calc_iq__ff_dn4))) + ((locals.var_fn133_calc_iq__two_n_phit_dn4 * locals.var_fn133_calc_iq__ff) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn4))), (((locals.var_fn133_calc_iq__vdsats1_dn5 * assign11720_e12038) + (locals.var_fn133_calc_iq__vdsats1 * (-locals.var_fn133_calc_iq__ff_dn5))) + ((locals.var_fn133_calc_iq__two_n_phit_dn5 * locals.var_fn133_calc_iq__ff) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn5))), (((locals.var_fn133_calc_iq__vdsats1_dn7 * assign11720_e12038) + (locals.var_fn133_calc_iq__vdsats1 * (-locals.var_fn133_calc_iq__ff_dn7))) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn7)), (((locals.var_fn133_calc_iq__vdsats1_dn14 * assign11720_e12038) + (locals.var_fn133_calc_iq__vdsats1 * (-locals.var_fn133_calc_iq__ff_dn14))) + ((locals.var_fn133_calc_iq__two_n_phit_dn14 * locals.var_fn133_calc_iq__ff) + (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__ff_dn14))),)
    } else {
        (locals.var_fn133_calc_iq__vdsat1, locals.var_fn133_calc_iq__vdsat1_dn2, locals.var_fn133_calc_iq__vdsat1_dn3, locals.var_fn133_calc_iq__vdsat1_dn4, locals.var_fn133_calc_iq__vdsat1_dn5, locals.var_fn133_calc_iq__vdsat1_dn7, locals.var_fn133_calc_iq__vdsat1_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsat1 = assign11720_e12045;
        locals.var_fn133_calc_iq__vdsat1_dn2 = assign11720_e12045_d_n2;
        locals.var_fn133_calc_iq__vdsat1_dn3 = assign11720_e12045_d_n3;
        locals.var_fn133_calc_iq__vdsat1_dn4 = assign11720_e12045_d_n4;
        locals.var_fn133_calc_iq__vdsat1_dn5 = assign11720_e12045_d_n5;
        locals.var_fn133_calc_iq__vdsat1_dn7 = assign11720_e12045_d_n7;
        locals.var_fn133_calc_iq__vdsat1_dn14 = assign11720_e12045_d_n14;

        let (assign11730_e12114, assign11730_e12114_d_n2, assign11730_e12114_d_n3, assign11730_e12114_d_n4, assign11730_e12114_d_n5, assign11730_e12114_d_n7, assign11730_e12114_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign11730_e12104, assign11730_e12104_d_n2, assign11730_e12104_d_n3, assign11730_e12104_d_n4, assign11730_e12104_d_n5, assign11730_e12104_d_n7, assign11730_e12104_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign11730_e12057: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat1);
                let assign11730_e12058: f64 = assign11730_e12057;
                let assign11730_e12062: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat1);
                let assign11730_e12063: f64 = (-assign11730_e12062);
                let assign11730_e12066: f64 = (0.001 / p.p53);
                let assign11730_e12070: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat1);
                let assign11730_e12071: f64 = (-assign11730_e12070);
                let assign11730_e12072: f64 = (assign11730_e12066 * assign11730_e12071);
                let assign11730_e12073: f64 = (assign11730_e12072).tanh();
                let assign11730_e12074: f64 = (assign11730_e12063 * assign11730_e12073);
                let assign11730_e12075: f64 = (assign11730_e12058 + assign11730_e12074);
                let assign11730_e12076: f64 = (0.5 * assign11730_e12075);
                (assign11730_e12076, (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + (((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11730_e12073) + (assign11730_e12063 * ((assign11730_e12066 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / ((assign11730_e12072).cosh() * (assign11730_e12072).cosh())))))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + (((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11730_e12073) + (assign11730_e12063 * ((assign11730_e12066 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / ((assign11730_e12072).cosh() * (assign11730_e12072).cosh())))))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + (((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11730_e12073) + (assign11730_e12063 * ((assign11730_e12066 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / ((assign11730_e12072).cosh() * (assign11730_e12072).cosh())))))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)) + (((-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) * assign11730_e12073) + (assign11730_e12063 * ((assign11730_e12066 * (-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) / ((assign11730_e12072).cosh() * (assign11730_e12072).cosh())))))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + (((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11730_e12073) + (assign11730_e12063 * ((assign11730_e12066 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / ((assign11730_e12072).cosh() * (assign11730_e12072).cosh())))))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)) + (((-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) * assign11730_e12073) + (assign11730_e12063 * ((assign11730_e12066 * (-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) / ((assign11730_e12072).cosh() * (assign11730_e12072).cosh())))))),)
            } else {
                let (assign11730_e12103, assign11730_e12103_d_n2, assign11730_e12103_d_n3, assign11730_e12103_d_n4, assign11730_e12103_d_n5, assign11730_e12103_d_n7, assign11730_e12103_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign11730_e12084: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat1);
                        let assign11730_e12085: f64 = assign11730_e12084;
                        let assign11730_e12089: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat1);
                        let assign11730_e12090: f64 = (-assign11730_e12089);
                        let assign11730_e12094: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat1);
                        let assign11730_e12095: f64 = (-assign11730_e12094);
                        let assign11730_e12096: f64 = (assign11730_e12090 * assign11730_e12095);
                        let assign11730_e12098: f64 = (assign11730_e12096 + p.p53);
                        let assign11730_e12099: f64 = (assign11730_e12098).sqrt();
                        let assign11730_e12100: f64 = (assign11730_e12085 + assign11730_e12099);
                        let assign11730_e12101: f64 = (0.5 * assign11730_e12100);
                        (assign11730_e12101, (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + ((((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11730_e12095) + (assign11730_e12090 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))))) / (2.0 * assign11730_e12099)))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + ((((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11730_e12095) + (assign11730_e12090 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))))) / (2.0 * assign11730_e12099)))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + ((((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11730_e12095) + (assign11730_e12090 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))))) / (2.0 * assign11730_e12099)))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)) + ((((-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) * assign11730_e12095) + (assign11730_e12090 * (-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / (2.0 * assign11730_e12099)))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + ((((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11730_e12095) + (assign11730_e12090 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))))) / (2.0 * assign11730_e12099)))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)) + ((((-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) * assign11730_e12095) + (assign11730_e12090 * (-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat1) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / (2.0 * assign11730_e12099)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign11730_e12103, assign11730_e12103_d_n2, assign11730_e12103_d_n3, assign11730_e12103_d_n4, assign11730_e12103_d_n5, assign11730_e12103_d_n7, assign11730_e12103_d_n14,)
            }
        };
        let assign11730_e12106: f64 = (assign11730_e12104).powf(locals.var_fn133_calc_iq__beta);
        let assign11730_e12107: f64 = (1.0 + assign11730_e12106);
        let assign11730_e12110: f64 = (1.0 / locals.var_fn133_calc_iq__beta);
        let assign11730_e12111: f64 = (assign11730_e12107).powf(assign11730_e12110);
        let assign11730_e12112: f64 = (1.0 / assign11730_e12111);
        (assign11730_e12112, (-(if 0.0 == 0.0 && ((assign11730_e12110) as f64).is_finite() && ((assign11730_e12110) as f64).fract() == 0.0 { if assign11730_e12110 == 0.0 { 0.0 } else { (assign11730_e12110 * ((assign11730_e12107).powf(assign11730_e12110 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n2)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n2 / assign11730_e12104))) })) } } else { (assign11730_e12111 * (assign11730_e12110 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n2)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n2 / assign11730_e12104))) } / assign11730_e12107))) } / (assign11730_e12111 * assign11730_e12111))), (-(if 0.0 == 0.0 && ((assign11730_e12110) as f64).is_finite() && ((assign11730_e12110) as f64).fract() == 0.0 { if assign11730_e12110 == 0.0 { 0.0 } else { (assign11730_e12110 * ((assign11730_e12107).powf(assign11730_e12110 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n3)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n3 / assign11730_e12104))) })) } } else { (assign11730_e12111 * (assign11730_e12110 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n3)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n3 / assign11730_e12104))) } / assign11730_e12107))) } / (assign11730_e12111 * assign11730_e12111))), (-(if 0.0 == 0.0 && ((assign11730_e12110) as f64).is_finite() && ((assign11730_e12110) as f64).fract() == 0.0 { if assign11730_e12110 == 0.0 { 0.0 } else { (assign11730_e12110 * ((assign11730_e12107).powf(assign11730_e12110 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n4)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n4 / assign11730_e12104))) })) } } else { (assign11730_e12111 * (assign11730_e12110 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n4)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n4 / assign11730_e12104))) } / assign11730_e12107))) } / (assign11730_e12111 * assign11730_e12111))), (-(if 0.0 == 0.0 && ((assign11730_e12110) as f64).is_finite() && ((assign11730_e12110) as f64).fract() == 0.0 { if assign11730_e12110 == 0.0 { 0.0 } else { (assign11730_e12110 * ((assign11730_e12107).powf(assign11730_e12110 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n5)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n5 / assign11730_e12104))) })) } } else { (assign11730_e12111 * (assign11730_e12110 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n5)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n5 / assign11730_e12104))) } / assign11730_e12107))) } / (assign11730_e12111 * assign11730_e12111))), (-(if 0.0 == 0.0 && ((assign11730_e12110) as f64).is_finite() && ((assign11730_e12110) as f64).fract() == 0.0 { if assign11730_e12110 == 0.0 { 0.0 } else { (assign11730_e12110 * ((assign11730_e12107).powf(assign11730_e12110 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n7)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n7 / assign11730_e12104))) })) } } else { (assign11730_e12111 * (assign11730_e12110 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n7)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n7 / assign11730_e12104))) } / assign11730_e12107))) } / (assign11730_e12111 * assign11730_e12111))), (-(if 0.0 == 0.0 && ((assign11730_e12110) as f64).is_finite() && ((assign11730_e12110) as f64).fract() == 0.0 { if assign11730_e12110 == 0.0 { 0.0 } else { (assign11730_e12110 * ((assign11730_e12107).powf(assign11730_e12110 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n14)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n14 / assign11730_e12104))) })) } } else { (assign11730_e12111 * (assign11730_e12110 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11730_e12104).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11730_e12104_d_n14)) } } else { (assign11730_e12106 * (locals.var_fn133_calc_iq__beta * (assign11730_e12104_d_n14 / assign11730_e12104))) } / assign11730_e12107))) } / (assign11730_e12111 * assign11730_e12111))),)
    } else {
        (locals.var_fn133_calc_iq__fsd, locals.var_fn133_calc_iq__fsd_dn2, locals.var_fn133_calc_iq__fsd_dn3, locals.var_fn133_calc_iq__fsd_dn4, locals.var_fn133_calc_iq__fsd_dn5, locals.var_fn133_calc_iq__fsd_dn7, locals.var_fn133_calc_iq__fsd_dn14,)
    }
};
        locals.var_fn133_calc_iq__fsd = assign11730_e12114;
        locals.var_fn133_calc_iq__fsd_dn2 = assign11730_e12114_d_n2;
        locals.var_fn133_calc_iq__fsd_dn3 = assign11730_e12114_d_n3;
        locals.var_fn133_calc_iq__fsd_dn4 = assign11730_e12114_d_n4;
        locals.var_fn133_calc_iq__fsd_dn5 = assign11730_e12114_d_n5;
        locals.var_fn133_calc_iq__fsd_dn7 = assign11730_e12114_d_n7;
        locals.var_fn133_calc_iq__fsd_dn14 = assign11730_e12114_d_n14;

        let (assign11740_e12120, assign11740_e12120_d_n2, assign11740_e12120_d_n3, assign11740_e12120_d_n4, assign11740_e12120_d_n5, assign11740_e12120_d_n7, assign11740_e12120_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11740_e12118: f64 = (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd);
        (assign11740_e12118, (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd_dn2), (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd_dn3), (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd_dn4), ((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__fsd) + (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd_dn5)), (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd_dn7), ((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__fsd) + (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__vdx, locals.var_fn133_calc_iq__vdx_dn2, locals.var_fn133_calc_iq__vdx_dn3, locals.var_fn133_calc_iq__vdx_dn4, locals.var_fn133_calc_iq__vdx_dn5, locals.var_fn133_calc_iq__vdx_dn7, locals.var_fn133_calc_iq__vdx_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdx = assign11740_e12120;
        locals.var_fn133_calc_iq__vdx_dn2 = assign11740_e12120_d_n2;
        locals.var_fn133_calc_iq__vdx_dn3 = assign11740_e12120_d_n3;
        locals.var_fn133_calc_iq__vdx_dn4 = assign11740_e12120_d_n4;
        locals.var_fn133_calc_iq__vdx_dn5 = assign11740_e12120_d_n5;
        locals.var_fn133_calc_iq__vdx_dn7 = assign11740_e12120_d_n7;
        locals.var_fn133_calc_iq__vdx_dn14 = assign11740_e12120_d_n14;

        let (assign11750_e12195, assign11750_e12195_d_n2, assign11750_e12195_d_n3, assign11750_e12195_d_n4, assign11750_e12195_d_n5, assign11750_e12195_d_n7, assign11750_e12195_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign11750_e12185, assign11750_e12185_d_n2, assign11750_e12185_d_n3, assign11750_e12185_d_n4, assign11750_e12185_d_n5, assign11750_e12185_d_n7, assign11750_e12185_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign11750_e12131: f64 = (-locals.var_fn133_calc_iq__vdsin);
                let assign11750_e12133: f64 = (assign11750_e12131 / locals.var_fn133_calc_iq__vdsat1);
                let assign11750_e12134: f64 = assign11750_e12133;
                let assign11750_e12137: f64 = (-locals.var_fn133_calc_iq__vdsin);
                let assign11750_e12139: f64 = (assign11750_e12137 / locals.var_fn133_calc_iq__vdsat1);
                let assign11750_e12140: f64 = (-assign11750_e12139);
                let assign11750_e12143: f64 = (0.001 / p.p53);
                let assign11750_e12146: f64 = (-locals.var_fn133_calc_iq__vdsin);
                let assign11750_e12148: f64 = (assign11750_e12146 / locals.var_fn133_calc_iq__vdsat1);
                let assign11750_e12149: f64 = (-assign11750_e12148);
                let assign11750_e12150: f64 = (assign11750_e12143 * assign11750_e12149);
                let assign11750_e12151: f64 = (assign11750_e12150).tanh();
                let assign11750_e12152: f64 = (assign11750_e12140 * assign11750_e12151);
                let assign11750_e12153: f64 = (assign11750_e12134 + assign11750_e12152);
                let assign11750_e12154: f64 = (0.5 * assign11750_e12153);
                (assign11750_e12154, (0.5 * ((-((assign11750_e12131 * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + (((-(-((assign11750_e12137 * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11750_e12151) + (assign11750_e12140 * ((assign11750_e12143 * (-(-((assign11750_e12146 * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / ((assign11750_e12150).cosh() * (assign11750_e12150).cosh())))))), (0.5 * ((-((assign11750_e12131 * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + (((-(-((assign11750_e12137 * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11750_e12151) + (assign11750_e12140 * ((assign11750_e12143 * (-(-((assign11750_e12146 * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / ((assign11750_e12150).cosh() * (assign11750_e12150).cosh())))))), (0.5 * ((-((assign11750_e12131 * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + (((-(-((assign11750_e12137 * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11750_e12151) + (assign11750_e12140 * ((assign11750_e12143 * (-(-((assign11750_e12146 * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / ((assign11750_e12150).cosh() * (assign11750_e12150).cosh())))))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12131 * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)) + (((-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12137 * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) * assign11750_e12151) + (assign11750_e12140 * ((assign11750_e12143 * (-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12146 * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) / ((assign11750_e12150).cosh() * (assign11750_e12150).cosh())))))), (0.5 * ((-((assign11750_e12131 * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + (((-(-((assign11750_e12137 * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11750_e12151) + (assign11750_e12140 * ((assign11750_e12143 * (-(-((assign11750_e12146 * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / ((assign11750_e12150).cosh() * (assign11750_e12150).cosh())))))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12131 * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)) + (((-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12137 * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) * assign11750_e12151) + (assign11750_e12140 * ((assign11750_e12143 * (-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12146 * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) / ((assign11750_e12150).cosh() * (assign11750_e12150).cosh())))))),)
            } else {
                let (assign11750_e12184, assign11750_e12184_d_n2, assign11750_e12184_d_n3, assign11750_e12184_d_n4, assign11750_e12184_d_n5, assign11750_e12184_d_n7, assign11750_e12184_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign11750_e12161: f64 = (-locals.var_fn133_calc_iq__vdsin);
                        let assign11750_e12163: f64 = (assign11750_e12161 / locals.var_fn133_calc_iq__vdsat1);
                        let assign11750_e12164: f64 = assign11750_e12163;
                        let assign11750_e12167: f64 = (-locals.var_fn133_calc_iq__vdsin);
                        let assign11750_e12169: f64 = (assign11750_e12167 / locals.var_fn133_calc_iq__vdsat1);
                        let assign11750_e12170: f64 = (-assign11750_e12169);
                        let assign11750_e12173: f64 = (-locals.var_fn133_calc_iq__vdsin);
                        let assign11750_e12175: f64 = (assign11750_e12173 / locals.var_fn133_calc_iq__vdsat1);
                        let assign11750_e12176: f64 = (-assign11750_e12175);
                        let assign11750_e12177: f64 = (assign11750_e12170 * assign11750_e12176);
                        let assign11750_e12179: f64 = (assign11750_e12177 + p.p53);
                        let assign11750_e12180: f64 = (assign11750_e12179).sqrt();
                        let assign11750_e12181: f64 = (assign11750_e12164 + assign11750_e12180);
                        let assign11750_e12182: f64 = (0.5 * assign11750_e12181);
                        (assign11750_e12182, (0.5 * ((-((assign11750_e12161 * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + ((((-(-((assign11750_e12167 * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11750_e12176) + (assign11750_e12170 * (-(-((assign11750_e12173 * locals.var_fn133_calc_iq__vdsat1_dn2) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))))) / (2.0 * assign11750_e12180)))), (0.5 * ((-((assign11750_e12161 * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + ((((-(-((assign11750_e12167 * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11750_e12176) + (assign11750_e12170 * (-(-((assign11750_e12173 * locals.var_fn133_calc_iq__vdsat1_dn3) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))))) / (2.0 * assign11750_e12180)))), (0.5 * ((-((assign11750_e12161 * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + ((((-(-((assign11750_e12167 * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11750_e12176) + (assign11750_e12170 * (-(-((assign11750_e12173 * locals.var_fn133_calc_iq__vdsat1_dn4) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))))) / (2.0 * assign11750_e12180)))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12161 * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)) + ((((-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12167 * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) * assign11750_e12176) + (assign11750_e12170 * (-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12173 * locals.var_fn133_calc_iq__vdsat1_dn5)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / (2.0 * assign11750_e12180)))), (0.5 * ((-((assign11750_e12161 * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) + ((((-(-((assign11750_e12167 * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))) * assign11750_e12176) + (assign11750_e12170 * (-(-((assign11750_e12173 * locals.var_fn133_calc_iq__vdsat1_dn7) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)))))) / (2.0 * assign11750_e12180)))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12161 * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1)) + ((((-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12167 * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))) * assign11750_e12176) + (assign11750_e12170 * (-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat1) - (assign11750_e12173 * locals.var_fn133_calc_iq__vdsat1_dn14)) / (locals.var_fn133_calc_iq__vdsat1 * locals.var_fn133_calc_iq__vdsat1))))) / (2.0 * assign11750_e12180)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign11750_e12184, assign11750_e12184_d_n2, assign11750_e12184_d_n3, assign11750_e12184_d_n4, assign11750_e12184_d_n5, assign11750_e12184_d_n7, assign11750_e12184_d_n14,)
            }
        };
        let assign11750_e12187: f64 = (assign11750_e12185).powf(locals.var_fn133_calc_iq__beta);
        let assign11750_e12188: f64 = (1.0 + assign11750_e12187);
        let assign11750_e12191: f64 = (1.0 / locals.var_fn133_calc_iq__beta);
        let assign11750_e12192: f64 = (assign11750_e12188).powf(assign11750_e12191);
        let assign11750_e12193: f64 = (1.0 / assign11750_e12192);
        (assign11750_e12193, (-(if 0.0 == 0.0 && ((assign11750_e12191) as f64).is_finite() && ((assign11750_e12191) as f64).fract() == 0.0 { if assign11750_e12191 == 0.0 { 0.0 } else { (assign11750_e12191 * ((assign11750_e12188).powf(assign11750_e12191 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n2)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n2 / assign11750_e12185))) })) } } else { (assign11750_e12192 * (assign11750_e12191 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n2)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n2 / assign11750_e12185))) } / assign11750_e12188))) } / (assign11750_e12192 * assign11750_e12192))), (-(if 0.0 == 0.0 && ((assign11750_e12191) as f64).is_finite() && ((assign11750_e12191) as f64).fract() == 0.0 { if assign11750_e12191 == 0.0 { 0.0 } else { (assign11750_e12191 * ((assign11750_e12188).powf(assign11750_e12191 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n3)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n3 / assign11750_e12185))) })) } } else { (assign11750_e12192 * (assign11750_e12191 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n3)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n3 / assign11750_e12185))) } / assign11750_e12188))) } / (assign11750_e12192 * assign11750_e12192))), (-(if 0.0 == 0.0 && ((assign11750_e12191) as f64).is_finite() && ((assign11750_e12191) as f64).fract() == 0.0 { if assign11750_e12191 == 0.0 { 0.0 } else { (assign11750_e12191 * ((assign11750_e12188).powf(assign11750_e12191 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n4)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n4 / assign11750_e12185))) })) } } else { (assign11750_e12192 * (assign11750_e12191 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n4)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n4 / assign11750_e12185))) } / assign11750_e12188))) } / (assign11750_e12192 * assign11750_e12192))), (-(if 0.0 == 0.0 && ((assign11750_e12191) as f64).is_finite() && ((assign11750_e12191) as f64).fract() == 0.0 { if assign11750_e12191 == 0.0 { 0.0 } else { (assign11750_e12191 * ((assign11750_e12188).powf(assign11750_e12191 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n5)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n5 / assign11750_e12185))) })) } } else { (assign11750_e12192 * (assign11750_e12191 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n5)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n5 / assign11750_e12185))) } / assign11750_e12188))) } / (assign11750_e12192 * assign11750_e12192))), (-(if 0.0 == 0.0 && ((assign11750_e12191) as f64).is_finite() && ((assign11750_e12191) as f64).fract() == 0.0 { if assign11750_e12191 == 0.0 { 0.0 } else { (assign11750_e12191 * ((assign11750_e12188).powf(assign11750_e12191 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n7)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n7 / assign11750_e12185))) })) } } else { (assign11750_e12192 * (assign11750_e12191 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n7)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n7 / assign11750_e12185))) } / assign11750_e12188))) } / (assign11750_e12192 * assign11750_e12192))), (-(if 0.0 == 0.0 && ((assign11750_e12191) as f64).is_finite() && ((assign11750_e12191) as f64).fract() == 0.0 { if assign11750_e12191 == 0.0 { 0.0 } else { (assign11750_e12191 * ((assign11750_e12188).powf(assign11750_e12191 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n14)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n14 / assign11750_e12185))) })) } } else { (assign11750_e12192 * (assign11750_e12191 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign11750_e12185).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign11750_e12185_d_n14)) } } else { (assign11750_e12187 * (locals.var_fn133_calc_iq__beta * (assign11750_e12185_d_n14 / assign11750_e12185))) } / assign11750_e12188))) } / (assign11750_e12192 * assign11750_e12192))),)
    } else {
        (locals.var_fn133_calc_iq__fds, locals.var_fn133_calc_iq__fds_dn2, locals.var_fn133_calc_iq__fds_dn3, locals.var_fn133_calc_iq__fds_dn4, locals.var_fn133_calc_iq__fds_dn5, locals.var_fn133_calc_iq__fds_dn7, locals.var_fn133_calc_iq__fds_dn14,)
    }
};
        locals.var_fn133_calc_iq__fds = assign11750_e12195;
        locals.var_fn133_calc_iq__fds_dn2 = assign11750_e12195_d_n2;
        locals.var_fn133_calc_iq__fds_dn3 = assign11750_e12195_d_n3;
        locals.var_fn133_calc_iq__fds_dn4 = assign11750_e12195_d_n4;
        locals.var_fn133_calc_iq__fds_dn5 = assign11750_e12195_d_n5;
        locals.var_fn133_calc_iq__fds_dn7 = assign11750_e12195_d_n7;
        locals.var_fn133_calc_iq__fds_dn14 = assign11750_e12195_d_n14;

        let (assign11760_e12202, assign11760_e12202_d_n2, assign11760_e12202_d_n3, assign11760_e12202_d_n4, assign11760_e12202_d_n5, assign11760_e12202_d_n7, assign11760_e12202_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11760_e12198: f64 = (-locals.var_fn133_calc_iq__vdsin);
        let assign11760_e12200: f64 = (assign11760_e12198 * locals.var_fn133_calc_iq__fds);
        (assign11760_e12200, (assign11760_e12198 * locals.var_fn133_calc_iq__fds_dn2), (assign11760_e12198 * locals.var_fn133_calc_iq__fds_dn3), (assign11760_e12198 * locals.var_fn133_calc_iq__fds_dn4), (((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__fds) + (assign11760_e12198 * locals.var_fn133_calc_iq__fds_dn5)), (assign11760_e12198 * locals.var_fn133_calc_iq__fds_dn7), (((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__fds) + (assign11760_e12198 * locals.var_fn133_calc_iq__fds_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__vsx, locals.var_fn133_calc_iq__vsx_dn2, locals.var_fn133_calc_iq__vsx_dn3, locals.var_fn133_calc_iq__vsx_dn4, locals.var_fn133_calc_iq__vsx_dn5, locals.var_fn133_calc_iq__vsx_dn7, locals.var_fn133_calc_iq__vsx_dn14,)
    }
};
        locals.var_fn133_calc_iq__vsx = assign11760_e12202;
        locals.var_fn133_calc_iq__vsx_dn2 = assign11760_e12202_d_n2;
        locals.var_fn133_calc_iq__vsx_dn3 = assign11760_e12202_d_n3;
        locals.var_fn133_calc_iq__vsx_dn4 = assign11760_e12202_d_n4;
        locals.var_fn133_calc_iq__vsx_dn5 = assign11760_e12202_d_n5;
        locals.var_fn133_calc_iq__vsx_dn7 = assign11760_e12202_d_n7;
        locals.var_fn133_calc_iq__vsx_dn14 = assign11760_e12202_d_n14;

        let (assign11770_e12210, assign11770_e12210_d_n2, assign11770_e12210_d_n3, assign11770_e12210_d_n4, assign11770_e12210_d_n5, assign11770_e12210_d_n7, assign11770_e12210_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11770_e12206: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__myarg);
        let assign11770_e12208: f64 = (assign11770_e12206 / locals.var_fn133_calc_iq__alpha_phit);
        (assign11770_e12208, ((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__myarg_dn2) / locals.var_fn133_calc_iq__alpha_phit), ((-locals.var_fn133_calc_iq__myarg_dn3) / locals.var_fn133_calc_iq__alpha_phit), ((((-locals.var_fn133_calc_iq__myarg_dn4) * locals.var_fn133_calc_iq__alpha_phit) - (assign11770_e12206 * locals.var_fn133_calc_iq__alpha_phit_dn4)) / (locals.var_fn133_calc_iq__alpha_phit * locals.var_fn133_calc_iq__alpha_phit)), ((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__myarg_dn5) / locals.var_fn133_calc_iq__alpha_phit), ((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__myarg_dn7) / locals.var_fn133_calc_iq__alpha_phit), ((-locals.var_fn133_calc_iq__myarg_dn14) / locals.var_fn133_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign11770_e12210;
        locals.var_fn133_calc_iq__exparg_dn2 = assign11770_e12210_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign11770_e12210_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign11770_e12210_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign11770_e12210_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign11770_e12210_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign11770_e12210_d_n14;

        let assign11780_e12213: f64 = if locals.var_fn133_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign11780_e12213;

        let (assign11790_e12219, assign11790_e12219_d_n2, assign11790_e12219_d_n3, assign11790_e12219_d_n4, assign11790_e12219_d_n5, assign11790_e12219_d_n7, assign11790_e12219_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard139 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffs, locals.var_fn133_calc_iq__ffs_dn2, locals.var_fn133_calc_iq__ffs_dn3, locals.var_fn133_calc_iq__ffs_dn4, locals.var_fn133_calc_iq__ffs_dn5, locals.var_fn133_calc_iq__ffs_dn7, locals.var_fn133_calc_iq__ffs_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs = assign11790_e12219;
        locals.var_fn133_calc_iq__ffs_dn2 = assign11790_e12219_d_n2;
        locals.var_fn133_calc_iq__ffs_dn3 = assign11790_e12219_d_n3;
        locals.var_fn133_calc_iq__ffs_dn4 = assign11790_e12219_d_n4;
        locals.var_fn133_calc_iq__ffs_dn5 = assign11790_e12219_d_n5;
        locals.var_fn133_calc_iq__ffs_dn7 = assign11790_e12219_d_n7;
        locals.var_fn133_calc_iq__ffs_dn14 = assign11790_e12219_d_n14;

        let assign11800_e12222: f64 = (-50.0);
        let assign11800_e12223: f64 = if locals.var_fn133_calc_iq__exparg < assign11800_e12222 { 1.0 } else { 0.0 };
        locals.var_guard140 = assign11800_e12223;

        let (assign11810_e12232, assign11810_e12232_d_n2, assign11810_e12232_d_n3, assign11810_e12232_d_n4, assign11810_e12232_d_n5, assign11810_e12232_d_n7, assign11810_e12232_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard139 == 0.0)) && (locals.var_guard140 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffs, locals.var_fn133_calc_iq__ffs_dn2, locals.var_fn133_calc_iq__ffs_dn3, locals.var_fn133_calc_iq__ffs_dn4, locals.var_fn133_calc_iq__ffs_dn5, locals.var_fn133_calc_iq__ffs_dn7, locals.var_fn133_calc_iq__ffs_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs = assign11810_e12232;
        locals.var_fn133_calc_iq__ffs_dn2 = assign11810_e12232_d_n2;
        locals.var_fn133_calc_iq__ffs_dn3 = assign11810_e12232_d_n3;
        locals.var_fn133_calc_iq__ffs_dn4 = assign11810_e12232_d_n4;
        locals.var_fn133_calc_iq__ffs_dn5 = assign11810_e12232_d_n5;
        locals.var_fn133_calc_iq__ffs_dn7 = assign11810_e12232_d_n7;
        locals.var_fn133_calc_iq__ffs_dn14 = assign11810_e12232_d_n14;

        let (assign11820_e12247, assign11820_e12247_d_n2, assign11820_e12247_d_n3, assign11820_e12247_d_n4, assign11820_e12247_d_n5, assign11820_e12247_d_n7, assign11820_e12247_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard139 == 0.0)) && (locals.var_guard140 == 0.0)) {
        let assign11820_e12243: f64 = (locals.var_fn133_calc_iq__exparg).exp();
        let assign11820_e12244: f64 = (1.0 + assign11820_e12243);
        let assign11820_e12245: f64 = (1.0 / assign11820_e12244);
        (assign11820_e12245, (-((assign11820_e12243 * locals.var_fn133_calc_iq__exparg_dn2) / (assign11820_e12244 * assign11820_e12244))), (-((assign11820_e12243 * locals.var_fn133_calc_iq__exparg_dn3) / (assign11820_e12244 * assign11820_e12244))), (-((assign11820_e12243 * locals.var_fn133_calc_iq__exparg_dn4) / (assign11820_e12244 * assign11820_e12244))), (-((assign11820_e12243 * locals.var_fn133_calc_iq__exparg_dn5) / (assign11820_e12244 * assign11820_e12244))), (-((assign11820_e12243 * locals.var_fn133_calc_iq__exparg_dn7) / (assign11820_e12244 * assign11820_e12244))), (-((assign11820_e12243 * locals.var_fn133_calc_iq__exparg_dn14) / (assign11820_e12244 * assign11820_e12244))),)
    } else {
        (locals.var_fn133_calc_iq__ffs, locals.var_fn133_calc_iq__ffs_dn2, locals.var_fn133_calc_iq__ffs_dn3, locals.var_fn133_calc_iq__ffs_dn4, locals.var_fn133_calc_iq__ffs_dn5, locals.var_fn133_calc_iq__ffs_dn7, locals.var_fn133_calc_iq__ffs_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs = assign11820_e12247;
        locals.var_fn133_calc_iq__ffs_dn2 = assign11820_e12247_d_n2;
        locals.var_fn133_calc_iq__ffs_dn3 = assign11820_e12247_d_n3;
        locals.var_fn133_calc_iq__ffs_dn4 = assign11820_e12247_d_n4;
        locals.var_fn133_calc_iq__ffs_dn5 = assign11820_e12247_d_n5;
        locals.var_fn133_calc_iq__ffs_dn7 = assign11820_e12247_d_n7;
        locals.var_fn133_calc_iq__ffs_dn14 = assign11820_e12247_d_n14;

        let (assign11830_e12265, assign11830_e12265_d_n2, assign11830_e12265_d_n3, assign11830_e12265_d_n4, assign11830_e12265_d_n5, assign11830_e12265_d_n7, assign11830_e12265_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11830_e12251: f64 = (locals.var_fn133_calc_iq__vgdin - locals.var_fn133_calc_iq__vsx);
        let assign11830_e12255: f64 = (p.p51 * 0.1);
        let assign11830_e12257: f64 = (assign11830_e12255 * locals.var_fn133_calc_iq__alpha_phit);
        let assign11830_e12259: f64 = (assign11830_e12257 * locals.var_fn133_calc_iq__ffs);
        let assign11830_e12260: f64 = (locals.var_fn133_calc_iq__vtdibl - assign11830_e12259);
        let assign11830_e12261: f64 = (assign11830_e12251 - assign11830_e12260);
        let assign11830_e12263: f64 = (assign11830_e12261 / locals.var_fn133_calc_iq__two_n_phit);
        (assign11830_e12263, (((locals.var_fn133_calc_iq__vgdin_dn2 - locals.var_fn133_calc_iq__vsx_dn2) - (-(assign11830_e12257 * locals.var_fn133_calc_iq__ffs_dn2))) / locals.var_fn133_calc_iq__two_n_phit), (((-locals.var_fn133_calc_iq__vsx_dn3) - (-(assign11830_e12257 * locals.var_fn133_calc_iq__ffs_dn3))) / locals.var_fn133_calc_iq__two_n_phit), (((((-locals.var_fn133_calc_iq__vsx_dn4) - (locals.var_fn133_calc_iq__vtdibl_dn4 - (((assign11830_e12255 * locals.var_fn133_calc_iq__alpha_phit_dn4) * locals.var_fn133_calc_iq__ffs) + (assign11830_e12257 * locals.var_fn133_calc_iq__ffs_dn4)))) * locals.var_fn133_calc_iq__two_n_phit) - (assign11830_e12261 * locals.var_fn133_calc_iq__two_n_phit_dn4)) / (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__two_n_phit)), (((((locals.var_fn133_calc_iq__vgdin_dn5 - locals.var_fn133_calc_iq__vsx_dn5) - (locals.var_fn133_calc_iq__vtdibl_dn5 - (assign11830_e12257 * locals.var_fn133_calc_iq__ffs_dn5))) * locals.var_fn133_calc_iq__two_n_phit) - (assign11830_e12261 * locals.var_fn133_calc_iq__two_n_phit_dn5)) / (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__two_n_phit)), (((locals.var_fn133_calc_iq__vgdin_dn7 - locals.var_fn133_calc_iq__vsx_dn7) - (-(assign11830_e12257 * locals.var_fn133_calc_iq__ffs_dn7))) / locals.var_fn133_calc_iq__two_n_phit), (((((locals.var_fn133_calc_iq__vgdin_dn14 - locals.var_fn133_calc_iq__vsx_dn14) - (locals.var_fn133_calc_iq__vtdibl_dn14 - (assign11830_e12257 * locals.var_fn133_calc_iq__ffs_dn14))) * locals.var_fn133_calc_iq__two_n_phit) - (assign11830_e12261 * locals.var_fn133_calc_iq__two_n_phit_dn14)) / (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn133_calc_iq__etas, locals.var_fn133_calc_iq__etas_dn2, locals.var_fn133_calc_iq__etas_dn3, locals.var_fn133_calc_iq__etas_dn4, locals.var_fn133_calc_iq__etas_dn5, locals.var_fn133_calc_iq__etas_dn7, locals.var_fn133_calc_iq__etas_dn14,)
    }
};
        locals.var_fn133_calc_iq__etas = assign11830_e12265;
        locals.var_fn133_calc_iq__etas_dn2 = assign11830_e12265_d_n2;
        locals.var_fn133_calc_iq__etas_dn3 = assign11830_e12265_d_n3;
        locals.var_fn133_calc_iq__etas_dn4 = assign11830_e12265_d_n4;
        locals.var_fn133_calc_iq__etas_dn5 = assign11830_e12265_d_n5;
        locals.var_fn133_calc_iq__etas_dn7 = assign11830_e12265_d_n7;
        locals.var_fn133_calc_iq__etas_dn14 = assign11830_e12265_d_n14;

        let assign11840_e12268: f64 = if locals.var_fn133_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard141 = assign11840_e12268;

        let (assign11850_e12276, assign11850_e12276_d_n2, assign11850_e12276_d_n3, assign11850_e12276_d_n4, assign11850_e12276_d_n5, assign11850_e12276_d_n7, assign11850_e12276_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard141 != 0.0)) {
        let assign11850_e12274: f64 = (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etas);
        (assign11850_e12274, (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etas_dn2), (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etas_dn3), ((locals.var_fn133_calc_iq__qref_dn4 * locals.var_fn133_calc_iq__etas) + (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etas_dn4)), ((locals.var_fn133_calc_iq__qref_dn5 * locals.var_fn133_calc_iq__etas) + (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etas_dn5)), (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etas_dn7), ((locals.var_fn133_calc_iq__qref_dn14 * locals.var_fn133_calc_iq__etas) + (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etas_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qinvs, locals.var_fn133_calc_iq__qinvs_dn2, locals.var_fn133_calc_iq__qinvs_dn3, locals.var_fn133_calc_iq__qinvs_dn4, locals.var_fn133_calc_iq__qinvs_dn5, locals.var_fn133_calc_iq__qinvs_dn7, locals.var_fn133_calc_iq__qinvs_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs = assign11850_e12276;
        locals.var_fn133_calc_iq__qinvs_dn2 = assign11850_e12276_d_n2;
        locals.var_fn133_calc_iq__qinvs_dn3 = assign11850_e12276_d_n3;
        locals.var_fn133_calc_iq__qinvs_dn4 = assign11850_e12276_d_n4;
        locals.var_fn133_calc_iq__qinvs_dn5 = assign11850_e12276_d_n5;
        locals.var_fn133_calc_iq__qinvs_dn7 = assign11850_e12276_d_n7;
        locals.var_fn133_calc_iq__qinvs_dn14 = assign11850_e12276_d_n14;

        let assign11860_e12279: f64 = (-50.0);
        let assign11860_e12280: f64 = if locals.var_fn133_calc_iq__etas < assign11860_e12279 { 1.0 } else { 0.0 };
        locals.var_guard142 = assign11860_e12280;

    }

    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11870_e12292, assign11870_e12292_d_n2, assign11870_e12292_d_n3, assign11870_e12292_d_n4, assign11870_e12292_d_n5, assign11870_e12292_d_n7, assign11870_e12292_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard141 == 0.0)) && (locals.var_guard142 != 0.0)) {
        let assign11870_e12289: f64 = (locals.var_fn133_calc_iq__etas).exp();
        let assign11870_e12290: f64 = (locals.var_fn133_calc_iq__qref * assign11870_e12289);
        (assign11870_e12290, (locals.var_fn133_calc_iq__qref * (assign11870_e12289 * locals.var_fn133_calc_iq__etas_dn2)), (locals.var_fn133_calc_iq__qref * (assign11870_e12289 * locals.var_fn133_calc_iq__etas_dn3)), ((locals.var_fn133_calc_iq__qref_dn4 * assign11870_e12289) + (locals.var_fn133_calc_iq__qref * (assign11870_e12289 * locals.var_fn133_calc_iq__etas_dn4))), ((locals.var_fn133_calc_iq__qref_dn5 * assign11870_e12289) + (locals.var_fn133_calc_iq__qref * (assign11870_e12289 * locals.var_fn133_calc_iq__etas_dn5))), (locals.var_fn133_calc_iq__qref * (assign11870_e12289 * locals.var_fn133_calc_iq__etas_dn7)), ((locals.var_fn133_calc_iq__qref_dn14 * assign11870_e12289) + (locals.var_fn133_calc_iq__qref * (assign11870_e12289 * locals.var_fn133_calc_iq__etas_dn14))),)
    } else {
        (locals.var_fn133_calc_iq__qinvs, locals.var_fn133_calc_iq__qinvs_dn2, locals.var_fn133_calc_iq__qinvs_dn3, locals.var_fn133_calc_iq__qinvs_dn4, locals.var_fn133_calc_iq__qinvs_dn5, locals.var_fn133_calc_iq__qinvs_dn7, locals.var_fn133_calc_iq__qinvs_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs = assign11870_e12292;
        locals.var_fn133_calc_iq__qinvs_dn2 = assign11870_e12292_d_n2;
        locals.var_fn133_calc_iq__qinvs_dn3 = assign11870_e12292_d_n3;
        locals.var_fn133_calc_iq__qinvs_dn4 = assign11870_e12292_d_n4;
        locals.var_fn133_calc_iq__qinvs_dn5 = assign11870_e12292_d_n5;
        locals.var_fn133_calc_iq__qinvs_dn7 = assign11870_e12292_d_n7;
        locals.var_fn133_calc_iq__qinvs_dn14 = assign11870_e12292_d_n14;

        let (assign11880_e12308, assign11880_e12308_d_n2, assign11880_e12308_d_n3, assign11880_e12308_d_n4, assign11880_e12308_d_n5, assign11880_e12308_d_n7, assign11880_e12308_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard141 == 0.0)) && (locals.var_guard142 == 0.0)) {
        let assign11880_e12303: f64 = (locals.var_fn133_calc_iq__etas).exp();
        let assign11880_e12304: f64 = (1.0 + assign11880_e12303);
        let assign11880_e12305: f64 = (assign11880_e12304).ln();
        let assign11880_e12306: f64 = (locals.var_fn133_calc_iq__qref * assign11880_e12305);
        (assign11880_e12306, (locals.var_fn133_calc_iq__qref * ((assign11880_e12303 * locals.var_fn133_calc_iq__etas_dn2) / assign11880_e12304)), (locals.var_fn133_calc_iq__qref * ((assign11880_e12303 * locals.var_fn133_calc_iq__etas_dn3) / assign11880_e12304)), ((locals.var_fn133_calc_iq__qref_dn4 * assign11880_e12305) + (locals.var_fn133_calc_iq__qref * ((assign11880_e12303 * locals.var_fn133_calc_iq__etas_dn4) / assign11880_e12304))), ((locals.var_fn133_calc_iq__qref_dn5 * assign11880_e12305) + (locals.var_fn133_calc_iq__qref * ((assign11880_e12303 * locals.var_fn133_calc_iq__etas_dn5) / assign11880_e12304))), (locals.var_fn133_calc_iq__qref * ((assign11880_e12303 * locals.var_fn133_calc_iq__etas_dn7) / assign11880_e12304)), ((locals.var_fn133_calc_iq__qref_dn14 * assign11880_e12305) + (locals.var_fn133_calc_iq__qref * ((assign11880_e12303 * locals.var_fn133_calc_iq__etas_dn14) / assign11880_e12304))),)
    } else {
        (locals.var_fn133_calc_iq__qinvs, locals.var_fn133_calc_iq__qinvs_dn2, locals.var_fn133_calc_iq__qinvs_dn3, locals.var_fn133_calc_iq__qinvs_dn4, locals.var_fn133_calc_iq__qinvs_dn5, locals.var_fn133_calc_iq__qinvs_dn7, locals.var_fn133_calc_iq__qinvs_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs = assign11880_e12308;
        locals.var_fn133_calc_iq__qinvs_dn2 = assign11880_e12308_d_n2;
        locals.var_fn133_calc_iq__qinvs_dn3 = assign11880_e12308_d_n3;
        locals.var_fn133_calc_iq__qinvs_dn4 = assign11880_e12308_d_n4;
        locals.var_fn133_calc_iq__qinvs_dn5 = assign11880_e12308_d_n5;
        locals.var_fn133_calc_iq__qinvs_dn7 = assign11880_e12308_d_n7;
        locals.var_fn133_calc_iq__qinvs_dn14 = assign11880_e12308_d_n14;

        let (assign11890_e12316, assign11890_e12316_d_n2, assign11890_e12316_d_n3, assign11890_e12316_d_n4, assign11890_e12316_d_n5, assign11890_e12316_d_n7, assign11890_e12316_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11890_e12312: f64 = (locals.var_fn133_calc_iq__vgdin - locals.var_fn133_calc_iq__myarg);
        let assign11890_e12314: f64 = (assign11890_e12312 / locals.var_fn133_calc_iq__alpha_phit);
        (assign11890_e12314, ((locals.var_fn133_calc_iq__vgdin_dn2 - locals.var_fn133_calc_iq__myarg_dn2) / locals.var_fn133_calc_iq__alpha_phit), ((-locals.var_fn133_calc_iq__myarg_dn3) / locals.var_fn133_calc_iq__alpha_phit), ((((-locals.var_fn133_calc_iq__myarg_dn4) * locals.var_fn133_calc_iq__alpha_phit) - (assign11890_e12312 * locals.var_fn133_calc_iq__alpha_phit_dn4)) / (locals.var_fn133_calc_iq__alpha_phit * locals.var_fn133_calc_iq__alpha_phit)), ((locals.var_fn133_calc_iq__vgdin_dn5 - locals.var_fn133_calc_iq__myarg_dn5) / locals.var_fn133_calc_iq__alpha_phit), ((locals.var_fn133_calc_iq__vgdin_dn7 - locals.var_fn133_calc_iq__myarg_dn7) / locals.var_fn133_calc_iq__alpha_phit), ((locals.var_fn133_calc_iq__vgdin_dn14 - locals.var_fn133_calc_iq__myarg_dn14) / locals.var_fn133_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign11890_e12316;
        locals.var_fn133_calc_iq__exparg_dn2 = assign11890_e12316_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign11890_e12316_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign11890_e12316_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign11890_e12316_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign11890_e12316_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign11890_e12316_d_n14;

        let assign11900_e12319: f64 = if locals.var_fn133_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign11900_e12319;

        let (assign11910_e12325, assign11910_e12325_d_n2, assign11910_e12325_d_n3, assign11910_e12325_d_n4, assign11910_e12325_d_n5, assign11910_e12325_d_n7, assign11910_e12325_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard143 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffd, locals.var_fn133_calc_iq__ffd_dn2, locals.var_fn133_calc_iq__ffd_dn3, locals.var_fn133_calc_iq__ffd_dn4, locals.var_fn133_calc_iq__ffd_dn5, locals.var_fn133_calc_iq__ffd_dn7, locals.var_fn133_calc_iq__ffd_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffd = assign11910_e12325;
        locals.var_fn133_calc_iq__ffd_dn2 = assign11910_e12325_d_n2;
        locals.var_fn133_calc_iq__ffd_dn3 = assign11910_e12325_d_n3;
        locals.var_fn133_calc_iq__ffd_dn4 = assign11910_e12325_d_n4;
        locals.var_fn133_calc_iq__ffd_dn5 = assign11910_e12325_d_n5;
        locals.var_fn133_calc_iq__ffd_dn7 = assign11910_e12325_d_n7;
        locals.var_fn133_calc_iq__ffd_dn14 = assign11910_e12325_d_n14;

        let assign11920_e12328: f64 = (-50.0);
        let assign11920_e12329: f64 = if locals.var_fn133_calc_iq__exparg < assign11920_e12328 { 1.0 } else { 0.0 };
        locals.var_guard144 = assign11920_e12329;

        let (assign11930_e12338, assign11930_e12338_d_n2, assign11930_e12338_d_n3, assign11930_e12338_d_n4, assign11930_e12338_d_n5, assign11930_e12338_d_n7, assign11930_e12338_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard143 == 0.0)) && (locals.var_guard144 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffd, locals.var_fn133_calc_iq__ffd_dn2, locals.var_fn133_calc_iq__ffd_dn3, locals.var_fn133_calc_iq__ffd_dn4, locals.var_fn133_calc_iq__ffd_dn5, locals.var_fn133_calc_iq__ffd_dn7, locals.var_fn133_calc_iq__ffd_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffd = assign11930_e12338;
        locals.var_fn133_calc_iq__ffd_dn2 = assign11930_e12338_d_n2;
        locals.var_fn133_calc_iq__ffd_dn3 = assign11930_e12338_d_n3;
        locals.var_fn133_calc_iq__ffd_dn4 = assign11930_e12338_d_n4;
        locals.var_fn133_calc_iq__ffd_dn5 = assign11930_e12338_d_n5;
        locals.var_fn133_calc_iq__ffd_dn7 = assign11930_e12338_d_n7;
        locals.var_fn133_calc_iq__ffd_dn14 = assign11930_e12338_d_n14;

        let (assign11940_e12353, assign11940_e12353_d_n2, assign11940_e12353_d_n3, assign11940_e12353_d_n4, assign11940_e12353_d_n5, assign11940_e12353_d_n7, assign11940_e12353_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard143 == 0.0)) && (locals.var_guard144 == 0.0)) {
        let assign11940_e12349: f64 = (locals.var_fn133_calc_iq__exparg).exp();
        let assign11940_e12350: f64 = (1.0 + assign11940_e12349);
        let assign11940_e12351: f64 = (1.0 / assign11940_e12350);
        (assign11940_e12351, (-((assign11940_e12349 * locals.var_fn133_calc_iq__exparg_dn2) / (assign11940_e12350 * assign11940_e12350))), (-((assign11940_e12349 * locals.var_fn133_calc_iq__exparg_dn3) / (assign11940_e12350 * assign11940_e12350))), (-((assign11940_e12349 * locals.var_fn133_calc_iq__exparg_dn4) / (assign11940_e12350 * assign11940_e12350))), (-((assign11940_e12349 * locals.var_fn133_calc_iq__exparg_dn5) / (assign11940_e12350 * assign11940_e12350))), (-((assign11940_e12349 * locals.var_fn133_calc_iq__exparg_dn7) / (assign11940_e12350 * assign11940_e12350))), (-((assign11940_e12349 * locals.var_fn133_calc_iq__exparg_dn14) / (assign11940_e12350 * assign11940_e12350))),)
    } else {
        (locals.var_fn133_calc_iq__ffd, locals.var_fn133_calc_iq__ffd_dn2, locals.var_fn133_calc_iq__ffd_dn3, locals.var_fn133_calc_iq__ffd_dn4, locals.var_fn133_calc_iq__ffd_dn5, locals.var_fn133_calc_iq__ffd_dn7, locals.var_fn133_calc_iq__ffd_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffd = assign11940_e12353;
        locals.var_fn133_calc_iq__ffd_dn2 = assign11940_e12353_d_n2;
        locals.var_fn133_calc_iq__ffd_dn3 = assign11940_e12353_d_n3;
        locals.var_fn133_calc_iq__ffd_dn4 = assign11940_e12353_d_n4;
        locals.var_fn133_calc_iq__ffd_dn5 = assign11940_e12353_d_n5;
        locals.var_fn133_calc_iq__ffd_dn7 = assign11940_e12353_d_n7;
        locals.var_fn133_calc_iq__ffd_dn14 = assign11940_e12353_d_n14;

        let (assign11950_e12371, assign11950_e12371_d_n2, assign11950_e12371_d_n3, assign11950_e12371_d_n4, assign11950_e12371_d_n5, assign11950_e12371_d_n7, assign11950_e12371_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign11950_e12357: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vdx);
        let assign11950_e12361: f64 = (p.p51 * 0.1);
        let assign11950_e12363: f64 = (assign11950_e12361 * locals.var_fn133_calc_iq__alpha_phit);
        let assign11950_e12365: f64 = (assign11950_e12363 * locals.var_fn133_calc_iq__ffd);
        let assign11950_e12366: f64 = (locals.var_fn133_calc_iq__vtdibl - assign11950_e12365);
        let assign11950_e12367: f64 = (assign11950_e12357 - assign11950_e12366);
        let assign11950_e12369: f64 = (assign11950_e12367 / locals.var_fn133_calc_iq__two_n_phit);
        (assign11950_e12369, (((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vdx_dn2) - (-(assign11950_e12363 * locals.var_fn133_calc_iq__ffd_dn2))) / locals.var_fn133_calc_iq__two_n_phit), (((-locals.var_fn133_calc_iq__vdx_dn3) - (-(assign11950_e12363 * locals.var_fn133_calc_iq__ffd_dn3))) / locals.var_fn133_calc_iq__two_n_phit), (((((-locals.var_fn133_calc_iq__vdx_dn4) - (locals.var_fn133_calc_iq__vtdibl_dn4 - (((assign11950_e12361 * locals.var_fn133_calc_iq__alpha_phit_dn4) * locals.var_fn133_calc_iq__ffd) + (assign11950_e12363 * locals.var_fn133_calc_iq__ffd_dn4)))) * locals.var_fn133_calc_iq__two_n_phit) - (assign11950_e12367 * locals.var_fn133_calc_iq__two_n_phit_dn4)) / (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__two_n_phit)), (((((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vdx_dn5) - (locals.var_fn133_calc_iq__vtdibl_dn5 - (assign11950_e12363 * locals.var_fn133_calc_iq__ffd_dn5))) * locals.var_fn133_calc_iq__two_n_phit) - (assign11950_e12367 * locals.var_fn133_calc_iq__two_n_phit_dn5)) / (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__two_n_phit)), (((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vdx_dn7) - (-(assign11950_e12363 * locals.var_fn133_calc_iq__ffd_dn7))) / locals.var_fn133_calc_iq__two_n_phit), (((((-locals.var_fn133_calc_iq__vdx_dn14) - (locals.var_fn133_calc_iq__vtdibl_dn14 - (assign11950_e12363 * locals.var_fn133_calc_iq__ffd_dn14))) * locals.var_fn133_calc_iq__two_n_phit) - (assign11950_e12367 * locals.var_fn133_calc_iq__two_n_phit_dn14)) / (locals.var_fn133_calc_iq__two_n_phit * locals.var_fn133_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn133_calc_iq__etad, locals.var_fn133_calc_iq__etad_dn2, locals.var_fn133_calc_iq__etad_dn3, locals.var_fn133_calc_iq__etad_dn4, locals.var_fn133_calc_iq__etad_dn5, locals.var_fn133_calc_iq__etad_dn7, locals.var_fn133_calc_iq__etad_dn14,)
    }
};
        locals.var_fn133_calc_iq__etad = assign11950_e12371;
        locals.var_fn133_calc_iq__etad_dn2 = assign11950_e12371_d_n2;
        locals.var_fn133_calc_iq__etad_dn3 = assign11950_e12371_d_n3;
        locals.var_fn133_calc_iq__etad_dn4 = assign11950_e12371_d_n4;
        locals.var_fn133_calc_iq__etad_dn5 = assign11950_e12371_d_n5;
        locals.var_fn133_calc_iq__etad_dn7 = assign11950_e12371_d_n7;
        locals.var_fn133_calc_iq__etad_dn14 = assign11950_e12371_d_n14;

        let assign11960_e12374: f64 = if locals.var_fn133_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard145 = assign11960_e12374;

        let (assign11970_e12382, assign11970_e12382_d_n2, assign11970_e12382_d_n3, assign11970_e12382_d_n4, assign11970_e12382_d_n5, assign11970_e12382_d_n7, assign11970_e12382_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard145 != 0.0)) {
        let assign11970_e12380: f64 = (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etad);
        (assign11970_e12380, (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etad_dn2), (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etad_dn3), ((locals.var_fn133_calc_iq__qref_dn4 * locals.var_fn133_calc_iq__etad) + (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etad_dn4)), ((locals.var_fn133_calc_iq__qref_dn5 * locals.var_fn133_calc_iq__etad) + (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etad_dn5)), (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etad_dn7), ((locals.var_fn133_calc_iq__qref_dn14 * locals.var_fn133_calc_iq__etad) + (locals.var_fn133_calc_iq__qref * locals.var_fn133_calc_iq__etad_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qinvd, locals.var_fn133_calc_iq__qinvd_dn2, locals.var_fn133_calc_iq__qinvd_dn3, locals.var_fn133_calc_iq__qinvd_dn4, locals.var_fn133_calc_iq__qinvd_dn5, locals.var_fn133_calc_iq__qinvd_dn7, locals.var_fn133_calc_iq__qinvd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd = assign11970_e12382;
        locals.var_fn133_calc_iq__qinvd_dn2 = assign11970_e12382_d_n2;
        locals.var_fn133_calc_iq__qinvd_dn3 = assign11970_e12382_d_n3;
        locals.var_fn133_calc_iq__qinvd_dn4 = assign11970_e12382_d_n4;
        locals.var_fn133_calc_iq__qinvd_dn5 = assign11970_e12382_d_n5;
        locals.var_fn133_calc_iq__qinvd_dn7 = assign11970_e12382_d_n7;
        locals.var_fn133_calc_iq__qinvd_dn14 = assign11970_e12382_d_n14;

        let assign11980_e12385: f64 = (-50.0);
        let assign11980_e12386: f64 = if locals.var_fn133_calc_iq__etad < assign11980_e12385 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign11980_e12386;

        let (assign11990_e12398, assign11990_e12398_d_n2, assign11990_e12398_d_n3, assign11990_e12398_d_n4, assign11990_e12398_d_n5, assign11990_e12398_d_n7, assign11990_e12398_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard145 == 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign11990_e12395: f64 = (locals.var_fn133_calc_iq__etad).exp();
        let assign11990_e12396: f64 = (locals.var_fn133_calc_iq__qref * assign11990_e12395);
        (assign11990_e12396, (locals.var_fn133_calc_iq__qref * (assign11990_e12395 * locals.var_fn133_calc_iq__etad_dn2)), (locals.var_fn133_calc_iq__qref * (assign11990_e12395 * locals.var_fn133_calc_iq__etad_dn3)), ((locals.var_fn133_calc_iq__qref_dn4 * assign11990_e12395) + (locals.var_fn133_calc_iq__qref * (assign11990_e12395 * locals.var_fn133_calc_iq__etad_dn4))), ((locals.var_fn133_calc_iq__qref_dn5 * assign11990_e12395) + (locals.var_fn133_calc_iq__qref * (assign11990_e12395 * locals.var_fn133_calc_iq__etad_dn5))), (locals.var_fn133_calc_iq__qref * (assign11990_e12395 * locals.var_fn133_calc_iq__etad_dn7)), ((locals.var_fn133_calc_iq__qref_dn14 * assign11990_e12395) + (locals.var_fn133_calc_iq__qref * (assign11990_e12395 * locals.var_fn133_calc_iq__etad_dn14))),)
    } else {
        (locals.var_fn133_calc_iq__qinvd, locals.var_fn133_calc_iq__qinvd_dn2, locals.var_fn133_calc_iq__qinvd_dn3, locals.var_fn133_calc_iq__qinvd_dn4, locals.var_fn133_calc_iq__qinvd_dn5, locals.var_fn133_calc_iq__qinvd_dn7, locals.var_fn133_calc_iq__qinvd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd = assign11990_e12398;
        locals.var_fn133_calc_iq__qinvd_dn2 = assign11990_e12398_d_n2;
        locals.var_fn133_calc_iq__qinvd_dn3 = assign11990_e12398_d_n3;
        locals.var_fn133_calc_iq__qinvd_dn4 = assign11990_e12398_d_n4;
        locals.var_fn133_calc_iq__qinvd_dn5 = assign11990_e12398_d_n5;
        locals.var_fn133_calc_iq__qinvd_dn7 = assign11990_e12398_d_n7;
        locals.var_fn133_calc_iq__qinvd_dn14 = assign11990_e12398_d_n14;

        let (assign12000_e12414, assign12000_e12414_d_n2, assign12000_e12414_d_n3, assign12000_e12414_d_n4, assign12000_e12414_d_n5, assign12000_e12414_d_n7, assign12000_e12414_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard145 == 0.0)) && (locals.var_guard146 == 0.0)) {
        let assign12000_e12409: f64 = (locals.var_fn133_calc_iq__etad).exp();
        let assign12000_e12410: f64 = (1.0 + assign12000_e12409);
        let assign12000_e12411: f64 = (assign12000_e12410).ln();
        let assign12000_e12412: f64 = (locals.var_fn133_calc_iq__qref * assign12000_e12411);
        (assign12000_e12412, (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn2) / assign12000_e12410)), (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn3) / assign12000_e12410)), ((locals.var_fn133_calc_iq__qref_dn4 * assign12000_e12411) + (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn4) / assign12000_e12410))), ((locals.var_fn133_calc_iq__qref_dn5 * assign12000_e12411) + (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn5) / assign12000_e12410))), (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn7) / assign12000_e12410)), ((locals.var_fn133_calc_iq__qref_dn14 * assign12000_e12411) + (locals.var_fn133_calc_iq__qref * ((assign12000_e12409 * locals.var_fn133_calc_iq__etad_dn14) / assign12000_e12410))),)
    } else {
        (locals.var_fn133_calc_iq__qinvd, locals.var_fn133_calc_iq__qinvd_dn2, locals.var_fn133_calc_iq__qinvd_dn3, locals.var_fn133_calc_iq__qinvd_dn4, locals.var_fn133_calc_iq__qinvd_dn5, locals.var_fn133_calc_iq__qinvd_dn7, locals.var_fn133_calc_iq__qinvd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd = assign12000_e12414;
        locals.var_fn133_calc_iq__qinvd_dn2 = assign12000_e12414_d_n2;
        locals.var_fn133_calc_iq__qinvd_dn3 = assign12000_e12414_d_n3;
        locals.var_fn133_calc_iq__qinvd_dn4 = assign12000_e12414_d_n4;
        locals.var_fn133_calc_iq__qinvd_dn5 = assign12000_e12414_d_n5;
        locals.var_fn133_calc_iq__qinvd_dn7 = assign12000_e12414_d_n7;
        locals.var_fn133_calc_iq__qinvd_dn14 = assign12000_e12414_d_n14;

        let (assign12010_e12422, assign12010_e12422_d_n2, assign12010_e12422_d_n3, assign12010_e12422_d_n4, assign12010_e12422_d_n5, assign12010_e12422_d_n7, assign12010_e12422_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12010_e12418: f64 = (locals.var_fn133_calc_iq__qinvs - locals.var_fn133_calc_iq__qinvd);
        let assign12010_e12420: f64 = (assign12010_e12418 / locals.var_fn133_calc_iq__cgin);
        (assign12010_e12420, ((locals.var_fn133_calc_iq__qinvs_dn2 - locals.var_fn133_calc_iq__qinvd_dn2) / locals.var_fn133_calc_iq__cgin), ((locals.var_fn133_calc_iq__qinvs_dn3 - locals.var_fn133_calc_iq__qinvd_dn3) / locals.var_fn133_calc_iq__cgin), ((((locals.var_fn133_calc_iq__qinvs_dn4 - locals.var_fn133_calc_iq__qinvd_dn4) * locals.var_fn133_calc_iq__cgin) - (assign12010_e12418 * locals.var_fn133_calc_iq__cgin_dn4)) / (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__cgin)), ((locals.var_fn133_calc_iq__qinvs_dn5 - locals.var_fn133_calc_iq__qinvd_dn5) / locals.var_fn133_calc_iq__cgin), ((locals.var_fn133_calc_iq__qinvs_dn7 - locals.var_fn133_calc_iq__qinvd_dn7) / locals.var_fn133_calc_iq__cgin), ((locals.var_fn133_calc_iq__qinvs_dn14 - locals.var_fn133_calc_iq__qinvd_dn14) / locals.var_fn133_calc_iq__cgin),)
    } else {
        (locals.var_fn133_calc_iq__vdsc, locals.var_fn133_calc_iq__vdsc_dn2, locals.var_fn133_calc_iq__vdsc_dn3, locals.var_fn133_calc_iq__vdsc_dn4, locals.var_fn133_calc_iq__vdsc_dn5, locals.var_fn133_calc_iq__vdsc_dn7, locals.var_fn133_calc_iq__vdsc_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsc = assign12010_e12422;
        locals.var_fn133_calc_iq__vdsc_dn2 = assign12010_e12422_d_n2;
        locals.var_fn133_calc_iq__vdsc_dn3 = assign12010_e12422_d_n3;
        locals.var_fn133_calc_iq__vdsc_dn4 = assign12010_e12422_d_n4;
        locals.var_fn133_calc_iq__vdsc_dn5 = assign12010_e12422_d_n5;
        locals.var_fn133_calc_iq__vdsc_dn7 = assign12010_e12422_d_n7;
        locals.var_fn133_calc_iq__vdsc_dn14 = assign12010_e12422_d_n14;

        let (assign12020_e12428, assign12020_e12428_d_n2, assign12020_e12428_d_n3, assign12020_e12428_d_n4, assign12020_e12428_d_n5, assign12020_e12428_d_n7, assign12020_e12428_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12020_e12426: f64 = (locals.var_fn133_calc_iq__vdsc / locals.var_fn133_calc_iq__vdsat);
        (assign12020_e12426, (((locals.var_fn133_calc_iq__vdsc_dn2 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn2)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)), (((locals.var_fn133_calc_iq__vdsc_dn3 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn3)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)), (((locals.var_fn133_calc_iq__vdsc_dn4 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn4)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)), (((locals.var_fn133_calc_iq__vdsc_dn5 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn5)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)), (((locals.var_fn133_calc_iq__vdsc_dn7 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn7)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)), (((locals.var_fn133_calc_iq__vdsc_dn14 * locals.var_fn133_calc_iq__vdsat) - (locals.var_fn133_calc_iq__vdsc * locals.var_fn133_calc_iq__vdsat_dn14)) / (locals.var_fn133_calc_iq__vdsat * locals.var_fn133_calc_iq__vdsat)),)
    } else {
        (locals.var_fn133_calc_iq__myarg, locals.var_fn133_calc_iq__myarg_dn2, locals.var_fn133_calc_iq__myarg_dn3, locals.var_fn133_calc_iq__myarg_dn4, locals.var_fn133_calc_iq__myarg_dn5, locals.var_fn133_calc_iq__myarg_dn7, locals.var_fn133_calc_iq__myarg_dn14,)
    }
};
        locals.var_fn133_calc_iq__myarg = assign12020_e12428;
        locals.var_fn133_calc_iq__myarg_dn2 = assign12020_e12428_d_n2;
        locals.var_fn133_calc_iq__myarg_dn3 = assign12020_e12428_d_n3;
        locals.var_fn133_calc_iq__myarg_dn4 = assign12020_e12428_d_n4;
        locals.var_fn133_calc_iq__myarg_dn5 = assign12020_e12428_d_n5;
        locals.var_fn133_calc_iq__myarg_dn7 = assign12020_e12428_d_n7;
        locals.var_fn133_calc_iq__myarg_dn14 = assign12020_e12428_d_n14;

        let (assign12060_e12497, assign12060_e12497_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12060_e12494: f64 = (2.302585092994046 * locals.var_fn133_calc_iq__phitin);
        let assign12060_e12495: f64 = (locals.var_fn133_calc_iq__ss / assign12060_e12494);
        (assign12060_e12495, (-((locals.var_fn133_calc_iq__ss * (2.302585092994046 * locals.var_fn133_calc_iq__phitin_dn4)) / (assign12060_e12494 * assign12060_e12494))),)
    } else {
        (locals.var_fn133_calc_iq__n0, locals.var_fn133_calc_iq__n0_dn4,)
    }
};
        locals.var_fn133_calc_iq__n0 = assign12060_e12497;
        locals.var_fn133_calc_iq__n0_dn4 = assign12060_e12497_d_n4;

        let (assign12070_e12505, assign12070_e12505_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12070_e12501: f64 = (2.0 * locals.var_fn133_calc_iq__n0);
        let assign12070_e12503: f64 = (assign12070_e12501 * locals.var_fn133_calc_iq__phitin);
        (assign12070_e12503, (((2.0 * locals.var_fn133_calc_iq__n0_dn4) * locals.var_fn133_calc_iq__phitin) + (assign12070_e12501 * locals.var_fn133_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn133_calc_iq__two_n_phit0, locals.var_fn133_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn133_calc_iq__two_n_phit0 = assign12070_e12505;
        locals.var_fn133_calc_iq__two_n_phit0_dn4 = assign12070_e12505_d_n4;

        let (assign12080_e12511, assign12080_e12511_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12080_e12509: f64 = (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__two_n_phit0);
        (assign12080_e12509, ((locals.var_fn133_calc_iq__cgin_dn4 * locals.var_fn133_calc_iq__two_n_phit0) + (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn133_calc_iq__qref0, locals.var_fn133_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn133_calc_iq__qref0 = assign12080_e12511;
        locals.var_fn133_calc_iq__qref0_dn4 = assign12080_e12511_d_n4;

        let (assign12090_e12521, assign12090_e12521_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12090_e12516: f64 = (p.p51 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12090_e12518: f64 = (assign12090_e12516 / 2.0);
        let assign12090_e12519: f64 = (locals.var_fn133_calc_iq__vtof - assign12090_e12518);
        (assign12090_e12519, (locals.var_fn133_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn133_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn133_calc_iq__myarg0, locals.var_fn133_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn133_calc_iq__myarg0 = assign12090_e12521;
        locals.var_fn133_calc_iq__myarg0_dn4 = assign12090_e12521_d_n4;

        let (assign12100_e12572, assign12100_e12572_d_n2, assign12100_e12572_d_n4, assign12100_e12572_d_n5, assign12100_e12572_d_n7, assign12100_e12572_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign12100_e12566, assign12100_e12566_d_n2, assign12100_e12566_d_n5, assign12100_e12566_d_n7, assign12100_e12566_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign12100_e12530: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                let assign12100_e12533: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign12100_e12536: f64 = (0.001 / p.p53);
                let assign12100_e12539: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign12100_e12540: f64 = (assign12100_e12536 * assign12100_e12539);
                let assign12100_e12541: f64 = (assign12100_e12540).tanh();
                let assign12100_e12542: f64 = (assign12100_e12533 * assign12100_e12541);
                let assign12100_e12543: f64 = (assign12100_e12530 + assign12100_e12542);
                let assign12100_e12544: f64 = (0.5 * assign12100_e12543);
                (assign12100_e12544, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + (((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign12100_e12541) + (assign12100_e12533 * ((assign12100_e12536 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2)) / ((assign12100_e12540).cosh() * (assign12100_e12540).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + (((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign12100_e12541) + (assign12100_e12533 * ((assign12100_e12536 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5)) / ((assign12100_e12540).cosh() * (assign12100_e12540).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + (((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign12100_e12541) + (assign12100_e12533 * ((assign12100_e12536 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7)) / ((assign12100_e12540).cosh() * (assign12100_e12540).cosh())))))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + (((-locals.var_fn133_calc_iq__vgdin_dn14) * assign12100_e12541) + (assign12100_e12533 * ((assign12100_e12536 * (-locals.var_fn133_calc_iq__vgdin_dn14)) / ((assign12100_e12540).cosh() * (assign12100_e12540).cosh())))))),)
            } else {
                let (assign12100_e12565, assign12100_e12565_d_n2, assign12100_e12565_d_n5, assign12100_e12565_d_n7, assign12100_e12565_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign12100_e12551: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                        let assign12100_e12554: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign12100_e12557: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign12100_e12558: f64 = (assign12100_e12554 * assign12100_e12557);
                        let assign12100_e12560: f64 = (assign12100_e12558 + p.p53);
                        let assign12100_e12561: f64 = (assign12100_e12560).sqrt();
                        let assign12100_e12562: f64 = (assign12100_e12551 + assign12100_e12561);
                        let assign12100_e12563: f64 = (0.5 * assign12100_e12562);
                        (assign12100_e12563, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + ((((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign12100_e12557) + (assign12100_e12554 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2))) / (2.0 * assign12100_e12561)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + ((((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign12100_e12557) + (assign12100_e12554 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5))) / (2.0 * assign12100_e12561)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + ((((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign12100_e12557) + (assign12100_e12554 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7))) / (2.0 * assign12100_e12561)))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + ((((-locals.var_fn133_calc_iq__vgdin_dn14) * assign12100_e12557) + (assign12100_e12554 * (-locals.var_fn133_calc_iq__vgdin_dn14))) / (2.0 * assign12100_e12561)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12100_e12565, assign12100_e12565_d_n2, assign12100_e12565_d_n5, assign12100_e12565_d_n7, assign12100_e12565_d_n14,)
            }
        };
        let assign12100_e12568: f64 = (assign12100_e12566 - locals.var_fn133_calc_iq__myarg0);
        let assign12100_e12570: f64 = (assign12100_e12568 / locals.var_fn133_calc_iq__alpha_phit);
        (assign12100_e12570, (assign12100_e12566_d_n2 / locals.var_fn133_calc_iq__alpha_phit), ((((-locals.var_fn133_calc_iq__myarg0_dn4) * locals.var_fn133_calc_iq__alpha_phit) - (assign12100_e12568 * locals.var_fn133_calc_iq__alpha_phit_dn4)) / (locals.var_fn133_calc_iq__alpha_phit * locals.var_fn133_calc_iq__alpha_phit)), (assign12100_e12566_d_n5 / locals.var_fn133_calc_iq__alpha_phit), (assign12100_e12566_d_n7 / locals.var_fn133_calc_iq__alpha_phit), (assign12100_e12566_d_n14 / locals.var_fn133_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn133_calc_iq__exparg0, locals.var_fn133_calc_iq__exparg0_dn2, locals.var_fn133_calc_iq__exparg0_dn4, locals.var_fn133_calc_iq__exparg0_dn5, locals.var_fn133_calc_iq__exparg0_dn7, locals.var_fn133_calc_iq__exparg0_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg0 = assign12100_e12572;
        locals.var_fn133_calc_iq__exparg0_dn2 = assign12100_e12572_d_n2;
        locals.var_fn133_calc_iq__exparg0_dn4 = assign12100_e12572_d_n4;
        locals.var_fn133_calc_iq__exparg0_dn5 = assign12100_e12572_d_n5;
        locals.var_fn133_calc_iq__exparg0_dn7 = assign12100_e12572_d_n7;
        locals.var_fn133_calc_iq__exparg0_dn14 = assign12100_e12572_d_n14;

        let assign12110_e12575: f64 = if locals.var_fn133_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign12110_e12575;

        let (assign12120_e12581, assign12120_e12581_d_n2, assign12120_e12581_d_n4, assign12120_e12581_d_n5, assign12120_e12581_d_n7, assign12120_e12581_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard147 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ff0, locals.var_fn133_calc_iq__ff0_dn2, locals.var_fn133_calc_iq__ff0_dn4, locals.var_fn133_calc_iq__ff0_dn5, locals.var_fn133_calc_iq__ff0_dn7, locals.var_fn133_calc_iq__ff0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff0 = assign12120_e12581;
        locals.var_fn133_calc_iq__ff0_dn2 = assign12120_e12581_d_n2;
        locals.var_fn133_calc_iq__ff0_dn4 = assign12120_e12581_d_n4;
        locals.var_fn133_calc_iq__ff0_dn5 = assign12120_e12581_d_n5;
        locals.var_fn133_calc_iq__ff0_dn7 = assign12120_e12581_d_n7;
        locals.var_fn133_calc_iq__ff0_dn14 = assign12120_e12581_d_n14;

        let assign12130_e12584: f64 = (-50.0);
        let assign12130_e12585: f64 = if locals.var_fn133_calc_iq__exparg0 < assign12130_e12584 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign12130_e12585;

        let (assign12140_e12594, assign12140_e12594_d_n2, assign12140_e12594_d_n4, assign12140_e12594_d_n5, assign12140_e12594_d_n7, assign12140_e12594_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ff0, locals.var_fn133_calc_iq__ff0_dn2, locals.var_fn133_calc_iq__ff0_dn4, locals.var_fn133_calc_iq__ff0_dn5, locals.var_fn133_calc_iq__ff0_dn7, locals.var_fn133_calc_iq__ff0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff0 = assign12140_e12594;
        locals.var_fn133_calc_iq__ff0_dn2 = assign12140_e12594_d_n2;
        locals.var_fn133_calc_iq__ff0_dn4 = assign12140_e12594_d_n4;
        locals.var_fn133_calc_iq__ff0_dn5 = assign12140_e12594_d_n5;
        locals.var_fn133_calc_iq__ff0_dn7 = assign12140_e12594_d_n7;
        locals.var_fn133_calc_iq__ff0_dn14 = assign12140_e12594_d_n14;

        let (assign12150_e12609, assign12150_e12609_d_n2, assign12150_e12609_d_n4, assign12150_e12609_d_n5, assign12150_e12609_d_n7, assign12150_e12609_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 == 0.0)) {
        let assign12150_e12605: f64 = (locals.var_fn133_calc_iq__exparg0).exp();
        let assign12150_e12606: f64 = (1.0 + assign12150_e12605);
        let assign12150_e12607: f64 = (1.0 / assign12150_e12606);
        (assign12150_e12607, (-((assign12150_e12605 * locals.var_fn133_calc_iq__exparg0_dn2) / (assign12150_e12606 * assign12150_e12606))), (-((assign12150_e12605 * locals.var_fn133_calc_iq__exparg0_dn4) / (assign12150_e12606 * assign12150_e12606))), (-((assign12150_e12605 * locals.var_fn133_calc_iq__exparg0_dn5) / (assign12150_e12606 * assign12150_e12606))), (-((assign12150_e12605 * locals.var_fn133_calc_iq__exparg0_dn7) / (assign12150_e12606 * assign12150_e12606))), (-((assign12150_e12605 * locals.var_fn133_calc_iq__exparg0_dn14) / (assign12150_e12606 * assign12150_e12606))),)
    } else {
        (locals.var_fn133_calc_iq__ff0, locals.var_fn133_calc_iq__ff0_dn2, locals.var_fn133_calc_iq__ff0_dn4, locals.var_fn133_calc_iq__ff0_dn5, locals.var_fn133_calc_iq__ff0_dn7, locals.var_fn133_calc_iq__ff0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff0 = assign12150_e12609;
        locals.var_fn133_calc_iq__ff0_dn2 = assign12150_e12609_d_n2;
        locals.var_fn133_calc_iq__ff0_dn4 = assign12150_e12609_d_n4;
        locals.var_fn133_calc_iq__ff0_dn5 = assign12150_e12609_d_n5;
        locals.var_fn133_calc_iq__ff0_dn7 = assign12150_e12609_d_n7;
        locals.var_fn133_calc_iq__ff0_dn14 = assign12150_e12609_d_n14;

        let (assign12160_e12668, assign12160_e12668_d_n2, assign12160_e12668_d_n4, assign12160_e12668_d_n5, assign12160_e12668_d_n7, assign12160_e12668_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign12160_e12654, assign12160_e12654_d_n2, assign12160_e12654_d_n5, assign12160_e12654_d_n7, assign12160_e12654_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign12160_e12618: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                let assign12160_e12621: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign12160_e12624: f64 = (0.001 / p.p53);
                let assign12160_e12627: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                let assign12160_e12628: f64 = (assign12160_e12624 * assign12160_e12627);
                let assign12160_e12629: f64 = (assign12160_e12628).tanh();
                let assign12160_e12630: f64 = (assign12160_e12621 * assign12160_e12629);
                let assign12160_e12631: f64 = (assign12160_e12618 + assign12160_e12630);
                let assign12160_e12632: f64 = (0.5 * assign12160_e12631);
                (assign12160_e12632, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + (((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign12160_e12629) + (assign12160_e12621 * ((assign12160_e12624 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2)) / ((assign12160_e12628).cosh() * (assign12160_e12628).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + (((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign12160_e12629) + (assign12160_e12621 * ((assign12160_e12624 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5)) / ((assign12160_e12628).cosh() * (assign12160_e12628).cosh())))))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + (((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign12160_e12629) + (assign12160_e12621 * ((assign12160_e12624 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7)) / ((assign12160_e12628).cosh() * (assign12160_e12628).cosh())))))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + (((-locals.var_fn133_calc_iq__vgdin_dn14) * assign12160_e12629) + (assign12160_e12621 * ((assign12160_e12624 * (-locals.var_fn133_calc_iq__vgdin_dn14)) / ((assign12160_e12628).cosh() * (assign12160_e12628).cosh())))))),)
            } else {
                let (assign12160_e12653, assign12160_e12653_d_n2, assign12160_e12653_d_n5, assign12160_e12653_d_n7, assign12160_e12653_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign12160_e12639: f64 = (locals.var_fn133_calc_iq__vgsin + locals.var_fn133_calc_iq__vgdin);
                        let assign12160_e12642: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign12160_e12645: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vgdin);
                        let assign12160_e12646: f64 = (assign12160_e12642 * assign12160_e12645);
                        let assign12160_e12648: f64 = (assign12160_e12646 + p.p53);
                        let assign12160_e12649: f64 = (assign12160_e12648).sqrt();
                        let assign12160_e12650: f64 = (assign12160_e12639 + assign12160_e12649);
                        let assign12160_e12651: f64 = (0.5 * assign12160_e12650);
                        (assign12160_e12651, (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn2 + locals.var_fn133_calc_iq__vgdin_dn2) + ((((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2) * assign12160_e12645) + (assign12160_e12642 * (locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vgdin_dn2))) / (2.0 * assign12160_e12649)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn5 + locals.var_fn133_calc_iq__vgdin_dn5) + ((((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5) * assign12160_e12645) + (assign12160_e12642 * (locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vgdin_dn5))) / (2.0 * assign12160_e12649)))), (0.5 * ((locals.var_fn133_calc_iq__vgsin_dn7 + locals.var_fn133_calc_iq__vgdin_dn7) + ((((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7) * assign12160_e12645) + (assign12160_e12642 * (locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vgdin_dn7))) / (2.0 * assign12160_e12649)))), (0.5 * (locals.var_fn133_calc_iq__vgdin_dn14 + ((((-locals.var_fn133_calc_iq__vgdin_dn14) * assign12160_e12645) + (assign12160_e12642 * (-locals.var_fn133_calc_iq__vgdin_dn14))) / (2.0 * assign12160_e12649)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12160_e12653, assign12160_e12653_d_n2, assign12160_e12653_d_n5, assign12160_e12653_d_n7, assign12160_e12653_d_n14,)
            }
        };
        let assign12160_e12658: f64 = (p.p51 * 0.1);
        let assign12160_e12660: f64 = (assign12160_e12658 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12160_e12662: f64 = (assign12160_e12660 * locals.var_fn133_calc_iq__ff0);
        let assign12160_e12663: f64 = (locals.var_fn133_calc_iq__vtof - assign12160_e12662);
        let assign12160_e12664: f64 = (assign12160_e12654 - assign12160_e12663);
        let assign12160_e12666: f64 = (assign12160_e12664 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12160_e12666, ((assign12160_e12654_d_n2 - (-(assign12160_e12660 * locals.var_fn133_calc_iq__ff0_dn2))) / locals.var_fn133_calc_iq__two_n_phit0), ((((-(locals.var_fn133_calc_iq__vtof_dn4 - (((assign12160_e12658 * locals.var_fn133_calc_iq__alpha_phit_dn4) * locals.var_fn133_calc_iq__ff0) + (assign12160_e12660 * locals.var_fn133_calc_iq__ff0_dn4)))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12160_e12664 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), ((assign12160_e12654_d_n5 - (-(assign12160_e12660 * locals.var_fn133_calc_iq__ff0_dn5))) / locals.var_fn133_calc_iq__two_n_phit0), ((assign12160_e12654_d_n7 - (-(assign12160_e12660 * locals.var_fn133_calc_iq__ff0_dn7))) / locals.var_fn133_calc_iq__two_n_phit0), ((assign12160_e12654_d_n14 - (-(assign12160_e12660 * locals.var_fn133_calc_iq__ff0_dn14))) / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__eta0, locals.var_fn133_calc_iq__eta0_dn2, locals.var_fn133_calc_iq__eta0_dn4, locals.var_fn133_calc_iq__eta0_dn5, locals.var_fn133_calc_iq__eta0_dn7, locals.var_fn133_calc_iq__eta0_dn14,)
    }
};
        locals.var_fn133_calc_iq__eta0 = assign12160_e12668;
        locals.var_fn133_calc_iq__eta0_dn2 = assign12160_e12668_d_n2;
        locals.var_fn133_calc_iq__eta0_dn4 = assign12160_e12668_d_n4;
        locals.var_fn133_calc_iq__eta0_dn5 = assign12160_e12668_d_n5;
        locals.var_fn133_calc_iq__eta0_dn7 = assign12160_e12668_d_n7;
        locals.var_fn133_calc_iq__eta0_dn14 = assign12160_e12668_d_n14;

        let assign12170_e12671: f64 = if locals.var_fn133_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign12170_e12671;

        let (assign12180_e12679, assign12180_e12679_d_n2, assign12180_e12679_d_n4, assign12180_e12679_d_n5, assign12180_e12679_d_n7, assign12180_e12679_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard149 != 0.0)) {
        let assign12180_e12677: f64 = (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0);
        (assign12180_e12677, (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0_dn2), ((locals.var_fn133_calc_iq__qref0_dn4 * locals.var_fn133_calc_iq__eta0) + (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0_dn4)), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0_dn5), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0_dn7), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__eta0_dn14),)
    } else {
        (locals.var_fn133_calc_iq__qinvv0, locals.var_fn133_calc_iq__qinvv0_dn2, locals.var_fn133_calc_iq__qinvv0_dn4, locals.var_fn133_calc_iq__qinvv0_dn5, locals.var_fn133_calc_iq__qinvv0_dn7, locals.var_fn133_calc_iq__qinvv0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvv0 = assign12180_e12679;
        locals.var_fn133_calc_iq__qinvv0_dn2 = assign12180_e12679_d_n2;
        locals.var_fn133_calc_iq__qinvv0_dn4 = assign12180_e12679_d_n4;
        locals.var_fn133_calc_iq__qinvv0_dn5 = assign12180_e12679_d_n5;
        locals.var_fn133_calc_iq__qinvv0_dn7 = assign12180_e12679_d_n7;
        locals.var_fn133_calc_iq__qinvv0_dn14 = assign12180_e12679_d_n14;

        let assign12190_e12682: f64 = (-50.0);
        let assign12190_e12683: f64 = if locals.var_fn133_calc_iq__eta0 < assign12190_e12682 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign12190_e12683;

        let (assign12200_e12695, assign12200_e12695_d_n2, assign12200_e12695_d_n4, assign12200_e12695_d_n5, assign12200_e12695_d_n7, assign12200_e12695_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard149 == 0.0)) && (locals.var_guard150 != 0.0)) {
        let assign12200_e12692: f64 = (locals.var_fn133_calc_iq__eta0).exp();
        let assign12200_e12693: f64 = (locals.var_fn133_calc_iq__qref0 * assign12200_e12692);
        (assign12200_e12693, (locals.var_fn133_calc_iq__qref0 * (assign12200_e12692 * locals.var_fn133_calc_iq__eta0_dn2)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12200_e12692) + (locals.var_fn133_calc_iq__qref0 * (assign12200_e12692 * locals.var_fn133_calc_iq__eta0_dn4))), (locals.var_fn133_calc_iq__qref0 * (assign12200_e12692 * locals.var_fn133_calc_iq__eta0_dn5)), (locals.var_fn133_calc_iq__qref0 * (assign12200_e12692 * locals.var_fn133_calc_iq__eta0_dn7)), (locals.var_fn133_calc_iq__qref0 * (assign12200_e12692 * locals.var_fn133_calc_iq__eta0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qinvv0, locals.var_fn133_calc_iq__qinvv0_dn2, locals.var_fn133_calc_iq__qinvv0_dn4, locals.var_fn133_calc_iq__qinvv0_dn5, locals.var_fn133_calc_iq__qinvv0_dn7, locals.var_fn133_calc_iq__qinvv0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvv0 = assign12200_e12695;
        locals.var_fn133_calc_iq__qinvv0_dn2 = assign12200_e12695_d_n2;
        locals.var_fn133_calc_iq__qinvv0_dn4 = assign12200_e12695_d_n4;
        locals.var_fn133_calc_iq__qinvv0_dn5 = assign12200_e12695_d_n5;
        locals.var_fn133_calc_iq__qinvv0_dn7 = assign12200_e12695_d_n7;
        locals.var_fn133_calc_iq__qinvv0_dn14 = assign12200_e12695_d_n14;

        let (assign12210_e12711, assign12210_e12711_d_n2, assign12210_e12711_d_n4, assign12210_e12711_d_n5, assign12210_e12711_d_n7, assign12210_e12711_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard149 == 0.0)) && (locals.var_guard150 == 0.0)) {
        let assign12210_e12706: f64 = (locals.var_fn133_calc_iq__eta0).exp();
        let assign12210_e12707: f64 = (1.0 + assign12210_e12706);
        let assign12210_e12708: f64 = (assign12210_e12707).ln();
        let assign12210_e12709: f64 = (locals.var_fn133_calc_iq__qref0 * assign12210_e12708);
        (assign12210_e12709, (locals.var_fn133_calc_iq__qref0 * ((assign12210_e12706 * locals.var_fn133_calc_iq__eta0_dn2) / assign12210_e12707)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12210_e12708) + (locals.var_fn133_calc_iq__qref0 * ((assign12210_e12706 * locals.var_fn133_calc_iq__eta0_dn4) / assign12210_e12707))), (locals.var_fn133_calc_iq__qref0 * ((assign12210_e12706 * locals.var_fn133_calc_iq__eta0_dn5) / assign12210_e12707)), (locals.var_fn133_calc_iq__qref0 * ((assign12210_e12706 * locals.var_fn133_calc_iq__eta0_dn7) / assign12210_e12707)), (locals.var_fn133_calc_iq__qref0 * ((assign12210_e12706 * locals.var_fn133_calc_iq__eta0_dn14) / assign12210_e12707)),)
    } else {
        (locals.var_fn133_calc_iq__qinvv0, locals.var_fn133_calc_iq__qinvv0_dn2, locals.var_fn133_calc_iq__qinvv0_dn4, locals.var_fn133_calc_iq__qinvv0_dn5, locals.var_fn133_calc_iq__qinvv0_dn7, locals.var_fn133_calc_iq__qinvv0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvv0 = assign12210_e12711;
        locals.var_fn133_calc_iq__qinvv0_dn2 = assign12210_e12711_d_n2;
        locals.var_fn133_calc_iq__qinvv0_dn4 = assign12210_e12711_d_n4;
        locals.var_fn133_calc_iq__qinvv0_dn5 = assign12210_e12711_d_n5;
        locals.var_fn133_calc_iq__qinvv0_dn7 = assign12210_e12711_d_n7;
        locals.var_fn133_calc_iq__qinvv0_dn14 = assign12210_e12711_d_n14;

        let (assign12220_e12717, assign12220_e12717_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12220_e12715: f64 = (locals.var_fn133_calc_iq__mu0 / locals.var_fn133_calc_iq__tfacmobin);
        (assign12220_e12715, (-((locals.var_fn133_calc_iq__mu0 * locals.var_fn133_calc_iq__tfacmobin_dn4) / (locals.var_fn133_calc_iq__tfacmobin * locals.var_fn133_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn133_calc_iq__muf0, locals.var_fn133_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn133_calc_iq__muf0 = assign12220_e12717;
        locals.var_fn133_calc_iq__muf0_dn4 = assign12220_e12717_d_n4;

        let (assign12230_e12733, assign12230_e12733_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12230_e12723: f64 = (locals.var_fn133_calc_iq__vzeta * locals.var_fn133_calc_iq__tnomin);
        let assign12230_e12724: f64 = (1.0 + assign12230_e12723);
        let assign12230_e12728: f64 = (locals.var_fn133_calc_iq__vzeta * locals.var_fn133_calc_iq__tambin);
        let assign12230_e12729: f64 = (1.0 + assign12230_e12728);
        let assign12230_e12730: f64 = (assign12230_e12724 / assign12230_e12729);
        let assign12230_e12731: f64 = (locals.var_fn133_calc_iq__vel0 * assign12230_e12730);
        (assign12230_e12731, (locals.var_fn133_calc_iq__vel0 * (-((assign12230_e12724 * (locals.var_fn133_calc_iq__vzeta * locals.var_fn133_calc_iq__tambin_dn4)) / (assign12230_e12729 * assign12230_e12729)))),)
    } else {
        (locals.var_fn133_calc_iq__vx0, locals.var_fn133_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn133_calc_iq__vx0 = assign12230_e12733;
        locals.var_fn133_calc_iq__vx0_dn4 = assign12230_e12733_d_n4;

    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12240_e12741, assign12240_e12741_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12240_e12737: f64 = (locals.var_fn133_calc_iq__vx0 * locals.var_fn133_calc_iq__lin);
        let assign12240_e12739: f64 = (assign12240_e12737 / locals.var_fn133_calc_iq__muf0);
        (assign12240_e12739, ((((locals.var_fn133_calc_iq__vx0_dn4 * locals.var_fn133_calc_iq__lin) * locals.var_fn133_calc_iq__muf0) - (assign12240_e12737 * locals.var_fn133_calc_iq__muf0_dn4)) / (locals.var_fn133_calc_iq__muf0 * locals.var_fn133_calc_iq__muf0)),)
    } else {
        (locals.var_fn133_calc_iq__vdsats0, locals.var_fn133_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn133_calc_iq__vdsats0 = assign12240_e12741;
        locals.var_fn133_calc_iq__vdsats0_dn4 = assign12240_e12741_d_n4;

        let (assign12250_e12758, assign12250_e12758_d_n2, assign12250_e12758_d_n4, assign12250_e12758_d_n5, assign12250_e12758_d_n7, assign12250_e12758_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12250_e12747: f64 = (2.0 * locals.var_fn133_calc_iq__qinvv0);
        let assign12250_e12749: f64 = (assign12250_e12747 / locals.var_fn133_calc_iq__cgin);
        let assign12250_e12751: f64 = (assign12250_e12749 / locals.var_fn133_calc_iq__vdsats0);
        let assign12250_e12752: f64 = (1.0 + assign12250_e12751);
        let assign12250_e12753: f64 = (assign12250_e12752).sqrt();
        let assign12250_e12754: f64 = (locals.var_fn133_calc_iq__vdsats0 * assign12250_e12753);
        let assign12250_e12756: f64 = (assign12250_e12754 - locals.var_fn133_calc_iq__vdsats0);
        (assign12250_e12756, (locals.var_fn133_calc_iq__vdsats0 * ((((2.0 * locals.var_fn133_calc_iq__qinvv0_dn2) / locals.var_fn133_calc_iq__cgin) / locals.var_fn133_calc_iq__vdsats0) / (2.0 * assign12250_e12753))), (((locals.var_fn133_calc_iq__vdsats0_dn4 * assign12250_e12753) + (locals.var_fn133_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn133_calc_iq__qinvv0_dn4) * locals.var_fn133_calc_iq__cgin) - (assign12250_e12747 * locals.var_fn133_calc_iq__cgin_dn4)) / (locals.var_fn133_calc_iq__cgin * locals.var_fn133_calc_iq__cgin)) * locals.var_fn133_calc_iq__vdsats0) - (assign12250_e12749 * locals.var_fn133_calc_iq__vdsats0_dn4)) / (locals.var_fn133_calc_iq__vdsats0 * locals.var_fn133_calc_iq__vdsats0)) / (2.0 * assign12250_e12753)))) - locals.var_fn133_calc_iq__vdsats0_dn4), (locals.var_fn133_calc_iq__vdsats0 * ((((2.0 * locals.var_fn133_calc_iq__qinvv0_dn5) / locals.var_fn133_calc_iq__cgin) / locals.var_fn133_calc_iq__vdsats0) / (2.0 * assign12250_e12753))), (locals.var_fn133_calc_iq__vdsats0 * ((((2.0 * locals.var_fn133_calc_iq__qinvv0_dn7) / locals.var_fn133_calc_iq__cgin) / locals.var_fn133_calc_iq__vdsats0) / (2.0 * assign12250_e12753))), (locals.var_fn133_calc_iq__vdsats0 * ((((2.0 * locals.var_fn133_calc_iq__qinvv0_dn14) / locals.var_fn133_calc_iq__cgin) / locals.var_fn133_calc_iq__vdsats0) / (2.0 * assign12250_e12753))),)
    } else {
        (locals.var_fn133_calc_iq__vdsats10, locals.var_fn133_calc_iq__vdsats10_dn2, locals.var_fn133_calc_iq__vdsats10_dn4, locals.var_fn133_calc_iq__vdsats10_dn5, locals.var_fn133_calc_iq__vdsats10_dn7, locals.var_fn133_calc_iq__vdsats10_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsats10 = assign12250_e12758;
        locals.var_fn133_calc_iq__vdsats10_dn2 = assign12250_e12758_d_n2;
        locals.var_fn133_calc_iq__vdsats10_dn4 = assign12250_e12758_d_n4;
        locals.var_fn133_calc_iq__vdsats10_dn5 = assign12250_e12758_d_n5;
        locals.var_fn133_calc_iq__vdsats10_dn7 = assign12250_e12758_d_n7;
        locals.var_fn133_calc_iq__vdsats10_dn14 = assign12250_e12758_d_n14;

        let (assign12260_e12770, assign12260_e12770_d_n2, assign12260_e12770_d_n4, assign12260_e12770_d_n5, assign12260_e12770_d_n7, assign12260_e12770_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12260_e12763: f64 = (1.0 - locals.var_fn133_calc_iq__ff0);
        let assign12260_e12764: f64 = (locals.var_fn133_calc_iq__vdsats10 * assign12260_e12763);
        let assign12260_e12767: f64 = (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0);
        let assign12260_e12768: f64 = (assign12260_e12764 + assign12260_e12767);
        (assign12260_e12768, (((locals.var_fn133_calc_iq__vdsats10_dn2 * assign12260_e12763) + (locals.var_fn133_calc_iq__vdsats10 * (-locals.var_fn133_calc_iq__ff0_dn2))) + (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0_dn2)), (((locals.var_fn133_calc_iq__vdsats10_dn4 * assign12260_e12763) + (locals.var_fn133_calc_iq__vdsats10 * (-locals.var_fn133_calc_iq__ff0_dn4))) + ((locals.var_fn133_calc_iq__two_n_phit0_dn4 * locals.var_fn133_calc_iq__ff0) + (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0_dn4))), (((locals.var_fn133_calc_iq__vdsats10_dn5 * assign12260_e12763) + (locals.var_fn133_calc_iq__vdsats10 * (-locals.var_fn133_calc_iq__ff0_dn5))) + (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0_dn5)), (((locals.var_fn133_calc_iq__vdsats10_dn7 * assign12260_e12763) + (locals.var_fn133_calc_iq__vdsats10 * (-locals.var_fn133_calc_iq__ff0_dn7))) + (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0_dn7)), (((locals.var_fn133_calc_iq__vdsats10_dn14 * assign12260_e12763) + (locals.var_fn133_calc_iq__vdsats10 * (-locals.var_fn133_calc_iq__ff0_dn14))) + (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__ff0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__vdsat10, locals.var_fn133_calc_iq__vdsat10_dn2, locals.var_fn133_calc_iq__vdsat10_dn4, locals.var_fn133_calc_iq__vdsat10_dn5, locals.var_fn133_calc_iq__vdsat10_dn7, locals.var_fn133_calc_iq__vdsat10_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsat10 = assign12260_e12770;
        locals.var_fn133_calc_iq__vdsat10_dn2 = assign12260_e12770_d_n2;
        locals.var_fn133_calc_iq__vdsat10_dn4 = assign12260_e12770_d_n4;
        locals.var_fn133_calc_iq__vdsat10_dn5 = assign12260_e12770_d_n5;
        locals.var_fn133_calc_iq__vdsat10_dn7 = assign12260_e12770_d_n7;
        locals.var_fn133_calc_iq__vdsat10_dn14 = assign12260_e12770_d_n14;

        let (assign12270_e12839, assign12270_e12839_d_n2, assign12270_e12839_d_n4, assign12270_e12839_d_n5, assign12270_e12839_d_n7, assign12270_e12839_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign12270_e12829, assign12270_e12829_d_n2, assign12270_e12829_d_n4, assign12270_e12829_d_n5, assign12270_e12829_d_n7, assign12270_e12829_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign12270_e12782: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                let assign12270_e12783: f64 = assign12270_e12782;
                let assign12270_e12787: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                let assign12270_e12788: f64 = (-assign12270_e12787);
                let assign12270_e12791: f64 = (0.001 / p.p53);
                let assign12270_e12795: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                let assign12270_e12796: f64 = (-assign12270_e12795);
                let assign12270_e12797: f64 = (assign12270_e12791 * assign12270_e12796);
                let assign12270_e12798: f64 = (assign12270_e12797).tanh();
                let assign12270_e12799: f64 = (assign12270_e12788 * assign12270_e12798);
                let assign12270_e12800: f64 = (assign12270_e12783 + assign12270_e12799);
                let assign12270_e12801: f64 = (0.5 * assign12270_e12800);
                (assign12270_e12801, (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12798) + (assign12270_e12788 * ((assign12270_e12791 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12270_e12797).cosh() * (assign12270_e12797).cosh())))))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12798) + (assign12270_e12788 * ((assign12270_e12791 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12270_e12797).cosh() * (assign12270_e12797).cosh())))))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + (((-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12270_e12798) + (assign12270_e12788 * ((assign12270_e12791 * (-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) / ((assign12270_e12797).cosh() * (assign12270_e12797).cosh())))))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12798) + (assign12270_e12788 * ((assign12270_e12791 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12270_e12797).cosh() * (assign12270_e12797).cosh())))))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + (((-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12270_e12798) + (assign12270_e12788 * ((assign12270_e12791 * (-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) / ((assign12270_e12797).cosh() * (assign12270_e12797).cosh())))))),)
            } else {
                let (assign12270_e12828, assign12270_e12828_d_n2, assign12270_e12828_d_n4, assign12270_e12828_d_n5, assign12270_e12828_d_n7, assign12270_e12828_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign12270_e12809: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                        let assign12270_e12810: f64 = assign12270_e12809;
                        let assign12270_e12814: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                        let assign12270_e12815: f64 = (-assign12270_e12814);
                        let assign12270_e12819: f64 = (locals.var_fn133_calc_iq__vdsin / locals.var_fn133_calc_iq__vdsat10);
                        let assign12270_e12820: f64 = (-assign12270_e12819);
                        let assign12270_e12821: f64 = (assign12270_e12815 * assign12270_e12820);
                        let assign12270_e12823: f64 = (assign12270_e12821 + p.p53);
                        let assign12270_e12824: f64 = (assign12270_e12823).sqrt();
                        let assign12270_e12825: f64 = (assign12270_e12810 + assign12270_e12824);
                        let assign12270_e12826: f64 = (0.5 * assign12270_e12825);
                        (assign12270_e12826, (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12820) + (assign12270_e12815 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12270_e12824)))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12820) + (assign12270_e12815 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12270_e12824)))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + ((((-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12270_e12820) + (assign12270_e12815 * (-(((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / (2.0 * assign12270_e12824)))), (0.5 * ((-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12270_e12820) + (assign12270_e12815 * (-(-((locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12270_e12824)))), (0.5 * ((((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + ((((-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12270_e12820) + (assign12270_e12815 * (-(((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__vdsat10) - (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / (2.0 * assign12270_e12824)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12270_e12828, assign12270_e12828_d_n2, assign12270_e12828_d_n4, assign12270_e12828_d_n5, assign12270_e12828_d_n7, assign12270_e12828_d_n14,)
            }
        };
        let assign12270_e12831: f64 = (assign12270_e12829).powf(locals.var_fn133_calc_iq__beta);
        let assign12270_e12832: f64 = (1.0 + assign12270_e12831);
        let assign12270_e12835: f64 = (1.0 / locals.var_fn133_calc_iq__beta);
        let assign12270_e12836: f64 = (assign12270_e12832).powf(assign12270_e12835);
        let assign12270_e12837: f64 = (1.0 / assign12270_e12836);
        (assign12270_e12837, (-(if 0.0 == 0.0 && ((assign12270_e12835) as f64).is_finite() && ((assign12270_e12835) as f64).fract() == 0.0 { if assign12270_e12835 == 0.0 { 0.0 } else { (assign12270_e12835 * ((assign12270_e12832).powf(assign12270_e12835 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n2)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n2 / assign12270_e12829))) })) } } else { (assign12270_e12836 * (assign12270_e12835 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n2)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n2 / assign12270_e12829))) } / assign12270_e12832))) } / (assign12270_e12836 * assign12270_e12836))), (-(if 0.0 == 0.0 && ((assign12270_e12835) as f64).is_finite() && ((assign12270_e12835) as f64).fract() == 0.0 { if assign12270_e12835 == 0.0 { 0.0 } else { (assign12270_e12835 * ((assign12270_e12832).powf(assign12270_e12835 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n4)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n4 / assign12270_e12829))) })) } } else { (assign12270_e12836 * (assign12270_e12835 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n4)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n4 / assign12270_e12829))) } / assign12270_e12832))) } / (assign12270_e12836 * assign12270_e12836))), (-(if 0.0 == 0.0 && ((assign12270_e12835) as f64).is_finite() && ((assign12270_e12835) as f64).fract() == 0.0 { if assign12270_e12835 == 0.0 { 0.0 } else { (assign12270_e12835 * ((assign12270_e12832).powf(assign12270_e12835 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n5)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n5 / assign12270_e12829))) })) } } else { (assign12270_e12836 * (assign12270_e12835 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n5)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n5 / assign12270_e12829))) } / assign12270_e12832))) } / (assign12270_e12836 * assign12270_e12836))), (-(if 0.0 == 0.0 && ((assign12270_e12835) as f64).is_finite() && ((assign12270_e12835) as f64).fract() == 0.0 { if assign12270_e12835 == 0.0 { 0.0 } else { (assign12270_e12835 * ((assign12270_e12832).powf(assign12270_e12835 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n7)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n7 / assign12270_e12829))) })) } } else { (assign12270_e12836 * (assign12270_e12835 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n7)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n7 / assign12270_e12829))) } / assign12270_e12832))) } / (assign12270_e12836 * assign12270_e12836))), (-(if 0.0 == 0.0 && ((assign12270_e12835) as f64).is_finite() && ((assign12270_e12835) as f64).fract() == 0.0 { if assign12270_e12835 == 0.0 { 0.0 } else { (assign12270_e12835 * ((assign12270_e12832).powf(assign12270_e12835 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n14)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n14 / assign12270_e12829))) })) } } else { (assign12270_e12836 * (assign12270_e12835 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12270_e12829).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12270_e12829_d_n14)) } } else { (assign12270_e12831 * (locals.var_fn133_calc_iq__beta * (assign12270_e12829_d_n14 / assign12270_e12829))) } / assign12270_e12832))) } / (assign12270_e12836 * assign12270_e12836))),)
    } else {
        (locals.var_fn133_calc_iq__fsd0, locals.var_fn133_calc_iq__fsd0_dn2, locals.var_fn133_calc_iq__fsd0_dn4, locals.var_fn133_calc_iq__fsd0_dn5, locals.var_fn133_calc_iq__fsd0_dn7, locals.var_fn133_calc_iq__fsd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__fsd0 = assign12270_e12839;
        locals.var_fn133_calc_iq__fsd0_dn2 = assign12270_e12839_d_n2;
        locals.var_fn133_calc_iq__fsd0_dn4 = assign12270_e12839_d_n4;
        locals.var_fn133_calc_iq__fsd0_dn5 = assign12270_e12839_d_n5;
        locals.var_fn133_calc_iq__fsd0_dn7 = assign12270_e12839_d_n7;
        locals.var_fn133_calc_iq__fsd0_dn14 = assign12270_e12839_d_n14;

        let (assign12280_e12845, assign12280_e12845_d_n2, assign12280_e12845_d_n4, assign12280_e12845_d_n5, assign12280_e12845_d_n7, assign12280_e12845_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12280_e12843: f64 = (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0);
        (assign12280_e12843, (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0_dn2), (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0_dn4), ((locals.var_fn133_calc_iq__vdsin_dn5 * locals.var_fn133_calc_iq__fsd0) + (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0_dn5)), (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0_dn7), ((locals.var_fn133_calc_iq__vdsin_dn14 * locals.var_fn133_calc_iq__fsd0) + (locals.var_fn133_calc_iq__vdsin * locals.var_fn133_calc_iq__fsd0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__vdx0, locals.var_fn133_calc_iq__vdx0_dn2, locals.var_fn133_calc_iq__vdx0_dn4, locals.var_fn133_calc_iq__vdx0_dn5, locals.var_fn133_calc_iq__vdx0_dn7, locals.var_fn133_calc_iq__vdx0_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdx0 = assign12280_e12845;
        locals.var_fn133_calc_iq__vdx0_dn2 = assign12280_e12845_d_n2;
        locals.var_fn133_calc_iq__vdx0_dn4 = assign12280_e12845_d_n4;
        locals.var_fn133_calc_iq__vdx0_dn5 = assign12280_e12845_d_n5;
        locals.var_fn133_calc_iq__vdx0_dn7 = assign12280_e12845_d_n7;
        locals.var_fn133_calc_iq__vdx0_dn14 = assign12280_e12845_d_n14;

        let (assign12290_e12920, assign12290_e12920_d_n2, assign12290_e12920_d_n4, assign12290_e12920_d_n5, assign12290_e12920_d_n7, assign12290_e12920_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let (assign12290_e12910, assign12290_e12910_d_n2, assign12290_e12910_d_n4, assign12290_e12910_d_n5, assign12290_e12910_d_n7, assign12290_e12910_d_n14,) = {
            if (p.p52 != 0.0) {
                let assign12290_e12856: f64 = (-locals.var_fn133_calc_iq__vdsin);
                let assign12290_e12858: f64 = (assign12290_e12856 / locals.var_fn133_calc_iq__vdsat10);
                let assign12290_e12859: f64 = assign12290_e12858;
                let assign12290_e12862: f64 = (-locals.var_fn133_calc_iq__vdsin);
                let assign12290_e12864: f64 = (assign12290_e12862 / locals.var_fn133_calc_iq__vdsat10);
                let assign12290_e12865: f64 = (-assign12290_e12864);
                let assign12290_e12868: f64 = (0.001 / p.p53);
                let assign12290_e12871: f64 = (-locals.var_fn133_calc_iq__vdsin);
                let assign12290_e12873: f64 = (assign12290_e12871 / locals.var_fn133_calc_iq__vdsat10);
                let assign12290_e12874: f64 = (-assign12290_e12873);
                let assign12290_e12875: f64 = (assign12290_e12868 * assign12290_e12874);
                let assign12290_e12876: f64 = (assign12290_e12875).tanh();
                let assign12290_e12877: f64 = (assign12290_e12865 * assign12290_e12876);
                let assign12290_e12878: f64 = (assign12290_e12859 + assign12290_e12877);
                let assign12290_e12879: f64 = (0.5 * assign12290_e12878);
                (assign12290_e12879, (0.5 * ((-((assign12290_e12856 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((assign12290_e12862 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12876) + (assign12290_e12865 * ((assign12290_e12868 * (-(-((assign12290_e12871 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12290_e12875).cosh() * (assign12290_e12875).cosh())))))), (0.5 * ((-((assign12290_e12856 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((assign12290_e12862 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12876) + (assign12290_e12865 * ((assign12290_e12868 * (-(-((assign12290_e12871 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12290_e12875).cosh() * (assign12290_e12875).cosh())))))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12856 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + (((-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12862 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12290_e12876) + (assign12290_e12865 * ((assign12290_e12868 * (-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12871 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) / ((assign12290_e12875).cosh() * (assign12290_e12875).cosh())))))), (0.5 * ((-((assign12290_e12856 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + (((-(-((assign12290_e12862 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12876) + (assign12290_e12865 * ((assign12290_e12868 * (-(-((assign12290_e12871 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / ((assign12290_e12875).cosh() * (assign12290_e12875).cosh())))))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12856 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + (((-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12862 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12290_e12876) + (assign12290_e12865 * ((assign12290_e12868 * (-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12871 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) / ((assign12290_e12875).cosh() * (assign12290_e12875).cosh())))))),)
            } else {
                let (assign12290_e12909, assign12290_e12909_d_n2, assign12290_e12909_d_n4, assign12290_e12909_d_n5, assign12290_e12909_d_n7, assign12290_e12909_d_n14,) = {
                    if (p.p52 == 0.0) {
                        let assign12290_e12886: f64 = (-locals.var_fn133_calc_iq__vdsin);
                        let assign12290_e12888: f64 = (assign12290_e12886 / locals.var_fn133_calc_iq__vdsat10);
                        let assign12290_e12889: f64 = assign12290_e12888;
                        let assign12290_e12892: f64 = (-locals.var_fn133_calc_iq__vdsin);
                        let assign12290_e12894: f64 = (assign12290_e12892 / locals.var_fn133_calc_iq__vdsat10);
                        let assign12290_e12895: f64 = (-assign12290_e12894);
                        let assign12290_e12898: f64 = (-locals.var_fn133_calc_iq__vdsin);
                        let assign12290_e12900: f64 = (assign12290_e12898 / locals.var_fn133_calc_iq__vdsat10);
                        let assign12290_e12901: f64 = (-assign12290_e12900);
                        let assign12290_e12902: f64 = (assign12290_e12895 * assign12290_e12901);
                        let assign12290_e12904: f64 = (assign12290_e12902 + p.p53);
                        let assign12290_e12905: f64 = (assign12290_e12904).sqrt();
                        let assign12290_e12906: f64 = (assign12290_e12889 + assign12290_e12905);
                        let assign12290_e12907: f64 = (0.5 * assign12290_e12906);
                        (assign12290_e12907, (0.5 * ((-((assign12290_e12886 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((assign12290_e12892 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12901) + (assign12290_e12895 * (-(-((assign12290_e12898 * locals.var_fn133_calc_iq__vdsat10_dn2) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12290_e12905)))), (0.5 * ((-((assign12290_e12886 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((assign12290_e12892 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12901) + (assign12290_e12895 * (-(-((assign12290_e12898 * locals.var_fn133_calc_iq__vdsat10_dn4) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12290_e12905)))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12886 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + ((((-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12892 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12290_e12901) + (assign12290_e12895 * (-((((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12898 * locals.var_fn133_calc_iq__vdsat10_dn5)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / (2.0 * assign12290_e12905)))), (0.5 * ((-((assign12290_e12886 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) + ((((-(-((assign12290_e12892 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))) * assign12290_e12901) + (assign12290_e12895 * (-(-((assign12290_e12898 * locals.var_fn133_calc_iq__vdsat10_dn7) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)))))) / (2.0 * assign12290_e12905)))), (0.5 * (((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12886 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10)) + ((((-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12892 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))) * assign12290_e12901) + (assign12290_e12895 * (-((((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__vdsat10) - (assign12290_e12898 * locals.var_fn133_calc_iq__vdsat10_dn14)) / (locals.var_fn133_calc_iq__vdsat10 * locals.var_fn133_calc_iq__vdsat10))))) / (2.0 * assign12290_e12905)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12290_e12909, assign12290_e12909_d_n2, assign12290_e12909_d_n4, assign12290_e12909_d_n5, assign12290_e12909_d_n7, assign12290_e12909_d_n14,)
            }
        };
        let assign12290_e12912: f64 = (assign12290_e12910).powf(locals.var_fn133_calc_iq__beta);
        let assign12290_e12913: f64 = (1.0 + assign12290_e12912);
        let assign12290_e12916: f64 = (1.0 / locals.var_fn133_calc_iq__beta);
        let assign12290_e12917: f64 = (assign12290_e12913).powf(assign12290_e12916);
        let assign12290_e12918: f64 = (1.0 / assign12290_e12917);
        (assign12290_e12918, (-(if 0.0 == 0.0 && ((assign12290_e12916) as f64).is_finite() && ((assign12290_e12916) as f64).fract() == 0.0 { if assign12290_e12916 == 0.0 { 0.0 } else { (assign12290_e12916 * ((assign12290_e12913).powf(assign12290_e12916 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n2)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n2 / assign12290_e12910))) })) } } else { (assign12290_e12917 * (assign12290_e12916 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n2)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n2 / assign12290_e12910))) } / assign12290_e12913))) } / (assign12290_e12917 * assign12290_e12917))), (-(if 0.0 == 0.0 && ((assign12290_e12916) as f64).is_finite() && ((assign12290_e12916) as f64).fract() == 0.0 { if assign12290_e12916 == 0.0 { 0.0 } else { (assign12290_e12916 * ((assign12290_e12913).powf(assign12290_e12916 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n4)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n4 / assign12290_e12910))) })) } } else { (assign12290_e12917 * (assign12290_e12916 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n4)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n4 / assign12290_e12910))) } / assign12290_e12913))) } / (assign12290_e12917 * assign12290_e12917))), (-(if 0.0 == 0.0 && ((assign12290_e12916) as f64).is_finite() && ((assign12290_e12916) as f64).fract() == 0.0 { if assign12290_e12916 == 0.0 { 0.0 } else { (assign12290_e12916 * ((assign12290_e12913).powf(assign12290_e12916 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n5)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n5 / assign12290_e12910))) })) } } else { (assign12290_e12917 * (assign12290_e12916 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n5)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n5 / assign12290_e12910))) } / assign12290_e12913))) } / (assign12290_e12917 * assign12290_e12917))), (-(if 0.0 == 0.0 && ((assign12290_e12916) as f64).is_finite() && ((assign12290_e12916) as f64).fract() == 0.0 { if assign12290_e12916 == 0.0 { 0.0 } else { (assign12290_e12916 * ((assign12290_e12913).powf(assign12290_e12916 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n7)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n7 / assign12290_e12910))) })) } } else { (assign12290_e12917 * (assign12290_e12916 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n7)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n7 / assign12290_e12910))) } / assign12290_e12913))) } / (assign12290_e12917 * assign12290_e12917))), (-(if 0.0 == 0.0 && ((assign12290_e12916) as f64).is_finite() && ((assign12290_e12916) as f64).fract() == 0.0 { if assign12290_e12916 == 0.0 { 0.0 } else { (assign12290_e12916 * ((assign12290_e12913).powf(assign12290_e12916 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n14)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n14 / assign12290_e12910))) })) } } else { (assign12290_e12917 * (assign12290_e12916 * (if 0.0 == 0.0 && ((locals.var_fn133_calc_iq__beta) as f64).is_finite() && ((locals.var_fn133_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn133_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn133_calc_iq__beta * ((assign12290_e12910).powf(locals.var_fn133_calc_iq__beta - 1.0) * assign12290_e12910_d_n14)) } } else { (assign12290_e12912 * (locals.var_fn133_calc_iq__beta * (assign12290_e12910_d_n14 / assign12290_e12910))) } / assign12290_e12913))) } / (assign12290_e12917 * assign12290_e12917))),)
    } else {
        (locals.var_fn133_calc_iq__fds0, locals.var_fn133_calc_iq__fds0_dn2, locals.var_fn133_calc_iq__fds0_dn4, locals.var_fn133_calc_iq__fds0_dn5, locals.var_fn133_calc_iq__fds0_dn7, locals.var_fn133_calc_iq__fds0_dn14,)
    }
};
        locals.var_fn133_calc_iq__fds0 = assign12290_e12920;
        locals.var_fn133_calc_iq__fds0_dn2 = assign12290_e12920_d_n2;
        locals.var_fn133_calc_iq__fds0_dn4 = assign12290_e12920_d_n4;
        locals.var_fn133_calc_iq__fds0_dn5 = assign12290_e12920_d_n5;
        locals.var_fn133_calc_iq__fds0_dn7 = assign12290_e12920_d_n7;
        locals.var_fn133_calc_iq__fds0_dn14 = assign12290_e12920_d_n14;

        let (assign12300_e12927, assign12300_e12927_d_n2, assign12300_e12927_d_n4, assign12300_e12927_d_n5, assign12300_e12927_d_n7, assign12300_e12927_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12300_e12923: f64 = (-locals.var_fn133_calc_iq__vdsin);
        let assign12300_e12925: f64 = (assign12300_e12923 * locals.var_fn133_calc_iq__fds0);
        (assign12300_e12925, (assign12300_e12923 * locals.var_fn133_calc_iq__fds0_dn2), (assign12300_e12923 * locals.var_fn133_calc_iq__fds0_dn4), (((-locals.var_fn133_calc_iq__vdsin_dn5) * locals.var_fn133_calc_iq__fds0) + (assign12300_e12923 * locals.var_fn133_calc_iq__fds0_dn5)), (assign12300_e12923 * locals.var_fn133_calc_iq__fds0_dn7), (((-locals.var_fn133_calc_iq__vdsin_dn14) * locals.var_fn133_calc_iq__fds0) + (assign12300_e12923 * locals.var_fn133_calc_iq__fds0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__vsx0, locals.var_fn133_calc_iq__vsx0_dn2, locals.var_fn133_calc_iq__vsx0_dn4, locals.var_fn133_calc_iq__vsx0_dn5, locals.var_fn133_calc_iq__vsx0_dn7, locals.var_fn133_calc_iq__vsx0_dn14,)
    }
};
        locals.var_fn133_calc_iq__vsx0 = assign12300_e12927;
        locals.var_fn133_calc_iq__vsx0_dn2 = assign12300_e12927_d_n2;
        locals.var_fn133_calc_iq__vsx0_dn4 = assign12300_e12927_d_n4;
        locals.var_fn133_calc_iq__vsx0_dn5 = assign12300_e12927_d_n5;
        locals.var_fn133_calc_iq__vsx0_dn7 = assign12300_e12927_d_n7;
        locals.var_fn133_calc_iq__vsx0_dn14 = assign12300_e12927_d_n14;

        let (assign12310_e12935, assign12310_e12935_d_n2, assign12310_e12935_d_n4, assign12310_e12935_d_n5, assign12310_e12935_d_n7, assign12310_e12935_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12310_e12931: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__myarg0);
        let assign12310_e12933: f64 = (assign12310_e12931 / locals.var_fn133_calc_iq__alpha_phit);
        (assign12310_e12933, (locals.var_fn133_calc_iq__vgsin_dn2 / locals.var_fn133_calc_iq__alpha_phit), ((((-locals.var_fn133_calc_iq__myarg0_dn4) * locals.var_fn133_calc_iq__alpha_phit) - (assign12310_e12931 * locals.var_fn133_calc_iq__alpha_phit_dn4)) / (locals.var_fn133_calc_iq__alpha_phit * locals.var_fn133_calc_iq__alpha_phit)), (locals.var_fn133_calc_iq__vgsin_dn5 / locals.var_fn133_calc_iq__alpha_phit), (locals.var_fn133_calc_iq__vgsin_dn7 / locals.var_fn133_calc_iq__alpha_phit), 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg0, locals.var_fn133_calc_iq__exparg0_dn2, locals.var_fn133_calc_iq__exparg0_dn4, locals.var_fn133_calc_iq__exparg0_dn5, locals.var_fn133_calc_iq__exparg0_dn7, locals.var_fn133_calc_iq__exparg0_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg0 = assign12310_e12935;
        locals.var_fn133_calc_iq__exparg0_dn2 = assign12310_e12935_d_n2;
        locals.var_fn133_calc_iq__exparg0_dn4 = assign12310_e12935_d_n4;
        locals.var_fn133_calc_iq__exparg0_dn5 = assign12310_e12935_d_n5;
        locals.var_fn133_calc_iq__exparg0_dn7 = assign12310_e12935_d_n7;
        locals.var_fn133_calc_iq__exparg0_dn14 = assign12310_e12935_d_n14;

        let assign12320_e12938: f64 = if locals.var_fn133_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign12320_e12938;

        let (assign12330_e12944, assign12330_e12944_d_n2, assign12330_e12944_d_n4, assign12330_e12944_d_n5, assign12330_e12944_d_n7, assign12330_e12944_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard151 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffs0, locals.var_fn133_calc_iq__ffs0_dn2, locals.var_fn133_calc_iq__ffs0_dn4, locals.var_fn133_calc_iq__ffs0_dn5, locals.var_fn133_calc_iq__ffs0_dn7, locals.var_fn133_calc_iq__ffs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs0 = assign12330_e12944;
        locals.var_fn133_calc_iq__ffs0_dn2 = assign12330_e12944_d_n2;
        locals.var_fn133_calc_iq__ffs0_dn4 = assign12330_e12944_d_n4;
        locals.var_fn133_calc_iq__ffs0_dn5 = assign12330_e12944_d_n5;
        locals.var_fn133_calc_iq__ffs0_dn7 = assign12330_e12944_d_n7;
        locals.var_fn133_calc_iq__ffs0_dn14 = assign12330_e12944_d_n14;

        let assign12340_e12947: f64 = (-50.0);
        let assign12340_e12948: f64 = if locals.var_fn133_calc_iq__exparg0 < assign12340_e12947 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign12340_e12948;

        let (assign12350_e12957, assign12350_e12957_d_n2, assign12350_e12957_d_n4, assign12350_e12957_d_n5, assign12350_e12957_d_n7, assign12350_e12957_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard151 == 0.0)) && (locals.var_guard152 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffs0, locals.var_fn133_calc_iq__ffs0_dn2, locals.var_fn133_calc_iq__ffs0_dn4, locals.var_fn133_calc_iq__ffs0_dn5, locals.var_fn133_calc_iq__ffs0_dn7, locals.var_fn133_calc_iq__ffs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs0 = assign12350_e12957;
        locals.var_fn133_calc_iq__ffs0_dn2 = assign12350_e12957_d_n2;
        locals.var_fn133_calc_iq__ffs0_dn4 = assign12350_e12957_d_n4;
        locals.var_fn133_calc_iq__ffs0_dn5 = assign12350_e12957_d_n5;
        locals.var_fn133_calc_iq__ffs0_dn7 = assign12350_e12957_d_n7;
        locals.var_fn133_calc_iq__ffs0_dn14 = assign12350_e12957_d_n14;

        let (assign12360_e12972, assign12360_e12972_d_n2, assign12360_e12972_d_n4, assign12360_e12972_d_n5, assign12360_e12972_d_n7, assign12360_e12972_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard151 == 0.0)) && (locals.var_guard152 == 0.0)) {
        let assign12360_e12968: f64 = (locals.var_fn133_calc_iq__exparg0).exp();
        let assign12360_e12969: f64 = (1.0 + assign12360_e12968);
        let assign12360_e12970: f64 = (1.0 / assign12360_e12969);
        (assign12360_e12970, (-((assign12360_e12968 * locals.var_fn133_calc_iq__exparg0_dn2) / (assign12360_e12969 * assign12360_e12969))), (-((assign12360_e12968 * locals.var_fn133_calc_iq__exparg0_dn4) / (assign12360_e12969 * assign12360_e12969))), (-((assign12360_e12968 * locals.var_fn133_calc_iq__exparg0_dn5) / (assign12360_e12969 * assign12360_e12969))), (-((assign12360_e12968 * locals.var_fn133_calc_iq__exparg0_dn7) / (assign12360_e12969 * assign12360_e12969))), (-((assign12360_e12968 * locals.var_fn133_calc_iq__exparg0_dn14) / (assign12360_e12969 * assign12360_e12969))),)
    } else {
        (locals.var_fn133_calc_iq__ffs0, locals.var_fn133_calc_iq__ffs0_dn2, locals.var_fn133_calc_iq__ffs0_dn4, locals.var_fn133_calc_iq__ffs0_dn5, locals.var_fn133_calc_iq__ffs0_dn7, locals.var_fn133_calc_iq__ffs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs0 = assign12360_e12972;
        locals.var_fn133_calc_iq__ffs0_dn2 = assign12360_e12972_d_n2;
        locals.var_fn133_calc_iq__ffs0_dn4 = assign12360_e12972_d_n4;
        locals.var_fn133_calc_iq__ffs0_dn5 = assign12360_e12972_d_n5;
        locals.var_fn133_calc_iq__ffs0_dn7 = assign12360_e12972_d_n7;
        locals.var_fn133_calc_iq__ffs0_dn14 = assign12360_e12972_d_n14;

        let (assign12370_e12990, assign12370_e12990_d_n2, assign12370_e12990_d_n4, assign12370_e12990_d_n5, assign12370_e12990_d_n7, assign12370_e12990_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12370_e12976: f64 = (locals.var_fn133_calc_iq__vgdin - locals.var_fn133_calc_iq__vsx0);
        let assign12370_e12980: f64 = (p.p51 * 0.1);
        let assign12370_e12982: f64 = (assign12370_e12980 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12370_e12984: f64 = (assign12370_e12982 * locals.var_fn133_calc_iq__ffs0);
        let assign12370_e12985: f64 = (locals.var_fn133_calc_iq__vtof - assign12370_e12984);
        let assign12370_e12986: f64 = (assign12370_e12976 - assign12370_e12985);
        let assign12370_e12988: f64 = (assign12370_e12986 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12370_e12988, (((locals.var_fn133_calc_iq__vgdin_dn2 - locals.var_fn133_calc_iq__vsx0_dn2) - (-(assign12370_e12982 * locals.var_fn133_calc_iq__ffs0_dn2))) / locals.var_fn133_calc_iq__two_n_phit0), (((((-locals.var_fn133_calc_iq__vsx0_dn4) - (locals.var_fn133_calc_iq__vtof_dn4 - (((assign12370_e12980 * locals.var_fn133_calc_iq__alpha_phit_dn4) * locals.var_fn133_calc_iq__ffs0) + (assign12370_e12982 * locals.var_fn133_calc_iq__ffs0_dn4)))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12370_e12986 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), (((locals.var_fn133_calc_iq__vgdin_dn5 - locals.var_fn133_calc_iq__vsx0_dn5) - (-(assign12370_e12982 * locals.var_fn133_calc_iq__ffs0_dn5))) / locals.var_fn133_calc_iq__two_n_phit0), (((locals.var_fn133_calc_iq__vgdin_dn7 - locals.var_fn133_calc_iq__vsx0_dn7) - (-(assign12370_e12982 * locals.var_fn133_calc_iq__ffs0_dn7))) / locals.var_fn133_calc_iq__two_n_phit0), (((locals.var_fn133_calc_iq__vgdin_dn14 - locals.var_fn133_calc_iq__vsx0_dn14) - (-(assign12370_e12982 * locals.var_fn133_calc_iq__ffs0_dn14))) / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__etas0, locals.var_fn133_calc_iq__etas0_dn2, locals.var_fn133_calc_iq__etas0_dn4, locals.var_fn133_calc_iq__etas0_dn5, locals.var_fn133_calc_iq__etas0_dn7, locals.var_fn133_calc_iq__etas0_dn14,)
    }
};
        locals.var_fn133_calc_iq__etas0 = assign12370_e12990;
        locals.var_fn133_calc_iq__etas0_dn2 = assign12370_e12990_d_n2;
        locals.var_fn133_calc_iq__etas0_dn4 = assign12370_e12990_d_n4;
        locals.var_fn133_calc_iq__etas0_dn5 = assign12370_e12990_d_n5;
        locals.var_fn133_calc_iq__etas0_dn7 = assign12370_e12990_d_n7;
        locals.var_fn133_calc_iq__etas0_dn14 = assign12370_e12990_d_n14;

        let assign12380_e12993: f64 = if locals.var_fn133_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign12380_e12993;

        let (assign12390_e13001, assign12390_e13001_d_n2, assign12390_e13001_d_n4, assign12390_e13001_d_n5, assign12390_e13001_d_n7, assign12390_e13001_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard153 != 0.0)) {
        let assign12390_e12999: f64 = (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0);
        (assign12390_e12999, (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0_dn2), ((locals.var_fn133_calc_iq__qref0_dn4 * locals.var_fn133_calc_iq__etas0) + (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0_dn4)), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0_dn5), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0_dn7), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etas0_dn14),)
    } else {
        (locals.var_fn133_calc_iq__qinvs0, locals.var_fn133_calc_iq__qinvs0_dn2, locals.var_fn133_calc_iq__qinvs0_dn4, locals.var_fn133_calc_iq__qinvs0_dn5, locals.var_fn133_calc_iq__qinvs0_dn7, locals.var_fn133_calc_iq__qinvs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs0 = assign12390_e13001;
        locals.var_fn133_calc_iq__qinvs0_dn2 = assign12390_e13001_d_n2;
        locals.var_fn133_calc_iq__qinvs0_dn4 = assign12390_e13001_d_n4;
        locals.var_fn133_calc_iq__qinvs0_dn5 = assign12390_e13001_d_n5;
        locals.var_fn133_calc_iq__qinvs0_dn7 = assign12390_e13001_d_n7;
        locals.var_fn133_calc_iq__qinvs0_dn14 = assign12390_e13001_d_n14;

        let assign12400_e13004: f64 = (-50.0);
        let assign12400_e13005: f64 = if locals.var_fn133_calc_iq__etas0 < assign12400_e13004 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign12400_e13005;

        let (assign12410_e13017, assign12410_e13017_d_n2, assign12410_e13017_d_n4, assign12410_e13017_d_n5, assign12410_e13017_d_n7, assign12410_e13017_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard153 == 0.0)) && (locals.var_guard154 != 0.0)) {
        let assign12410_e13014: f64 = (locals.var_fn133_calc_iq__etas0).exp();
        let assign12410_e13015: f64 = (locals.var_fn133_calc_iq__qref0 * assign12410_e13014);
        (assign12410_e13015, (locals.var_fn133_calc_iq__qref0 * (assign12410_e13014 * locals.var_fn133_calc_iq__etas0_dn2)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12410_e13014) + (locals.var_fn133_calc_iq__qref0 * (assign12410_e13014 * locals.var_fn133_calc_iq__etas0_dn4))), (locals.var_fn133_calc_iq__qref0 * (assign12410_e13014 * locals.var_fn133_calc_iq__etas0_dn5)), (locals.var_fn133_calc_iq__qref0 * (assign12410_e13014 * locals.var_fn133_calc_iq__etas0_dn7)), (locals.var_fn133_calc_iq__qref0 * (assign12410_e13014 * locals.var_fn133_calc_iq__etas0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qinvs0, locals.var_fn133_calc_iq__qinvs0_dn2, locals.var_fn133_calc_iq__qinvs0_dn4, locals.var_fn133_calc_iq__qinvs0_dn5, locals.var_fn133_calc_iq__qinvs0_dn7, locals.var_fn133_calc_iq__qinvs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs0 = assign12410_e13017;
        locals.var_fn133_calc_iq__qinvs0_dn2 = assign12410_e13017_d_n2;
        locals.var_fn133_calc_iq__qinvs0_dn4 = assign12410_e13017_d_n4;
        locals.var_fn133_calc_iq__qinvs0_dn5 = assign12410_e13017_d_n5;
        locals.var_fn133_calc_iq__qinvs0_dn7 = assign12410_e13017_d_n7;
        locals.var_fn133_calc_iq__qinvs0_dn14 = assign12410_e13017_d_n14;

        let (assign12420_e13033, assign12420_e13033_d_n2, assign12420_e13033_d_n4, assign12420_e13033_d_n5, assign12420_e13033_d_n7, assign12420_e13033_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard153 == 0.0)) && (locals.var_guard154 == 0.0)) {
        let assign12420_e13028: f64 = (locals.var_fn133_calc_iq__etas0).exp();
        let assign12420_e13029: f64 = (1.0 + assign12420_e13028);
        let assign12420_e13030: f64 = (assign12420_e13029).ln();
        let assign12420_e13031: f64 = (locals.var_fn133_calc_iq__qref0 * assign12420_e13030);
        (assign12420_e13031, (locals.var_fn133_calc_iq__qref0 * ((assign12420_e13028 * locals.var_fn133_calc_iq__etas0_dn2) / assign12420_e13029)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12420_e13030) + (locals.var_fn133_calc_iq__qref0 * ((assign12420_e13028 * locals.var_fn133_calc_iq__etas0_dn4) / assign12420_e13029))), (locals.var_fn133_calc_iq__qref0 * ((assign12420_e13028 * locals.var_fn133_calc_iq__etas0_dn5) / assign12420_e13029)), (locals.var_fn133_calc_iq__qref0 * ((assign12420_e13028 * locals.var_fn133_calc_iq__etas0_dn7) / assign12420_e13029)), (locals.var_fn133_calc_iq__qref0 * ((assign12420_e13028 * locals.var_fn133_calc_iq__etas0_dn14) / assign12420_e13029)),)
    } else {
        (locals.var_fn133_calc_iq__qinvs0, locals.var_fn133_calc_iq__qinvs0_dn2, locals.var_fn133_calc_iq__qinvs0_dn4, locals.var_fn133_calc_iq__qinvs0_dn5, locals.var_fn133_calc_iq__qinvs0_dn7, locals.var_fn133_calc_iq__qinvs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs0 = assign12420_e13033;
        locals.var_fn133_calc_iq__qinvs0_dn2 = assign12420_e13033_d_n2;
        locals.var_fn133_calc_iq__qinvs0_dn4 = assign12420_e13033_d_n4;
        locals.var_fn133_calc_iq__qinvs0_dn5 = assign12420_e13033_d_n5;
        locals.var_fn133_calc_iq__qinvs0_dn7 = assign12420_e13033_d_n7;
        locals.var_fn133_calc_iq__qinvs0_dn14 = assign12420_e13033_d_n14;

        let (assign12430_e13041, assign12430_e13041_d_n2, assign12430_e13041_d_n4, assign12430_e13041_d_n5, assign12430_e13041_d_n7, assign12430_e13041_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12430_e13037: f64 = (locals.var_fn133_calc_iq__vgdin - locals.var_fn133_calc_iq__myarg0);
        let assign12430_e13039: f64 = (assign12430_e13037 / locals.var_fn133_calc_iq__alpha_phit);
        (assign12430_e13039, (locals.var_fn133_calc_iq__vgdin_dn2 / locals.var_fn133_calc_iq__alpha_phit), ((((-locals.var_fn133_calc_iq__myarg0_dn4) * locals.var_fn133_calc_iq__alpha_phit) - (assign12430_e13037 * locals.var_fn133_calc_iq__alpha_phit_dn4)) / (locals.var_fn133_calc_iq__alpha_phit * locals.var_fn133_calc_iq__alpha_phit)), (locals.var_fn133_calc_iq__vgdin_dn5 / locals.var_fn133_calc_iq__alpha_phit), (locals.var_fn133_calc_iq__vgdin_dn7 / locals.var_fn133_calc_iq__alpha_phit), (locals.var_fn133_calc_iq__vgdin_dn14 / locals.var_fn133_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn133_calc_iq__exparg0, locals.var_fn133_calc_iq__exparg0_dn2, locals.var_fn133_calc_iq__exparg0_dn4, locals.var_fn133_calc_iq__exparg0_dn5, locals.var_fn133_calc_iq__exparg0_dn7, locals.var_fn133_calc_iq__exparg0_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg0 = assign12430_e13041;
        locals.var_fn133_calc_iq__exparg0_dn2 = assign12430_e13041_d_n2;
        locals.var_fn133_calc_iq__exparg0_dn4 = assign12430_e13041_d_n4;
        locals.var_fn133_calc_iq__exparg0_dn5 = assign12430_e13041_d_n5;
        locals.var_fn133_calc_iq__exparg0_dn7 = assign12430_e13041_d_n7;
        locals.var_fn133_calc_iq__exparg0_dn14 = assign12430_e13041_d_n14;

        let assign12440_e13044: f64 = if locals.var_fn133_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign12440_e13044;

        let (assign12450_e13050, assign12450_e13050_d_n2, assign12450_e13050_d_n4, assign12450_e13050_d_n5, assign12450_e13050_d_n7, assign12450_e13050_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard155 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffd0, locals.var_fn133_calc_iq__ffd0_dn2, locals.var_fn133_calc_iq__ffd0_dn4, locals.var_fn133_calc_iq__ffd0_dn5, locals.var_fn133_calc_iq__ffd0_dn7, locals.var_fn133_calc_iq__ffd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffd0 = assign12450_e13050;
        locals.var_fn133_calc_iq__ffd0_dn2 = assign12450_e13050_d_n2;
        locals.var_fn133_calc_iq__ffd0_dn4 = assign12450_e13050_d_n4;
        locals.var_fn133_calc_iq__ffd0_dn5 = assign12450_e13050_d_n5;
        locals.var_fn133_calc_iq__ffd0_dn7 = assign12450_e13050_d_n7;
        locals.var_fn133_calc_iq__ffd0_dn14 = assign12450_e13050_d_n14;

        let assign12460_e13053: f64 = (-50.0);
        let assign12460_e13054: f64 = if locals.var_fn133_calc_iq__exparg0 < assign12460_e13053 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign12460_e13054;

        let (assign12470_e13063, assign12470_e13063_d_n2, assign12470_e13063_d_n4, assign12470_e13063_d_n5, assign12470_e13063_d_n7, assign12470_e13063_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffd0, locals.var_fn133_calc_iq__ffd0_dn2, locals.var_fn133_calc_iq__ffd0_dn4, locals.var_fn133_calc_iq__ffd0_dn5, locals.var_fn133_calc_iq__ffd0_dn7, locals.var_fn133_calc_iq__ffd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffd0 = assign12470_e13063;
        locals.var_fn133_calc_iq__ffd0_dn2 = assign12470_e13063_d_n2;
        locals.var_fn133_calc_iq__ffd0_dn4 = assign12470_e13063_d_n4;
        locals.var_fn133_calc_iq__ffd0_dn5 = assign12470_e13063_d_n5;
        locals.var_fn133_calc_iq__ffd0_dn7 = assign12470_e13063_d_n7;
        locals.var_fn133_calc_iq__ffd0_dn14 = assign12470_e13063_d_n14;

        let (assign12480_e13078, assign12480_e13078_d_n2, assign12480_e13078_d_n4, assign12480_e13078_d_n5, assign12480_e13078_d_n7, assign12480_e13078_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 == 0.0)) {
        let assign12480_e13074: f64 = (locals.var_fn133_calc_iq__exparg0).exp();
        let assign12480_e13075: f64 = (1.0 + assign12480_e13074);
        let assign12480_e13076: f64 = (1.0 / assign12480_e13075);
        (assign12480_e13076, (-((assign12480_e13074 * locals.var_fn133_calc_iq__exparg0_dn2) / (assign12480_e13075 * assign12480_e13075))), (-((assign12480_e13074 * locals.var_fn133_calc_iq__exparg0_dn4) / (assign12480_e13075 * assign12480_e13075))), (-((assign12480_e13074 * locals.var_fn133_calc_iq__exparg0_dn5) / (assign12480_e13075 * assign12480_e13075))), (-((assign12480_e13074 * locals.var_fn133_calc_iq__exparg0_dn7) / (assign12480_e13075 * assign12480_e13075))), (-((assign12480_e13074 * locals.var_fn133_calc_iq__exparg0_dn14) / (assign12480_e13075 * assign12480_e13075))),)
    } else {
        (locals.var_fn133_calc_iq__ffd0, locals.var_fn133_calc_iq__ffd0_dn2, locals.var_fn133_calc_iq__ffd0_dn4, locals.var_fn133_calc_iq__ffd0_dn5, locals.var_fn133_calc_iq__ffd0_dn7, locals.var_fn133_calc_iq__ffd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffd0 = assign12480_e13078;
        locals.var_fn133_calc_iq__ffd0_dn2 = assign12480_e13078_d_n2;
        locals.var_fn133_calc_iq__ffd0_dn4 = assign12480_e13078_d_n4;
        locals.var_fn133_calc_iq__ffd0_dn5 = assign12480_e13078_d_n5;
        locals.var_fn133_calc_iq__ffd0_dn7 = assign12480_e13078_d_n7;
        locals.var_fn133_calc_iq__ffd0_dn14 = assign12480_e13078_d_n14;

        let (assign12490_e13096, assign12490_e13096_d_n2, assign12490_e13096_d_n4, assign12490_e13096_d_n5, assign12490_e13096_d_n7, assign12490_e13096_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12490_e13082: f64 = (locals.var_fn133_calc_iq__vgsin - locals.var_fn133_calc_iq__vdx0);
        let assign12490_e13086: f64 = (p.p51 * 0.1);
        let assign12490_e13088: f64 = (assign12490_e13086 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12490_e13090: f64 = (assign12490_e13088 * locals.var_fn133_calc_iq__ffd0);
        let assign12490_e13091: f64 = (locals.var_fn133_calc_iq__vtof - assign12490_e13090);
        let assign12490_e13092: f64 = (assign12490_e13082 - assign12490_e13091);
        let assign12490_e13094: f64 = (assign12490_e13092 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12490_e13094, (((locals.var_fn133_calc_iq__vgsin_dn2 - locals.var_fn133_calc_iq__vdx0_dn2) - (-(assign12490_e13088 * locals.var_fn133_calc_iq__ffd0_dn2))) / locals.var_fn133_calc_iq__two_n_phit0), (((((-locals.var_fn133_calc_iq__vdx0_dn4) - (locals.var_fn133_calc_iq__vtof_dn4 - (((assign12490_e13086 * locals.var_fn133_calc_iq__alpha_phit_dn4) * locals.var_fn133_calc_iq__ffd0) + (assign12490_e13088 * locals.var_fn133_calc_iq__ffd0_dn4)))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12490_e13092 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), (((locals.var_fn133_calc_iq__vgsin_dn5 - locals.var_fn133_calc_iq__vdx0_dn5) - (-(assign12490_e13088 * locals.var_fn133_calc_iq__ffd0_dn5))) / locals.var_fn133_calc_iq__two_n_phit0), (((locals.var_fn133_calc_iq__vgsin_dn7 - locals.var_fn133_calc_iq__vdx0_dn7) - (-(assign12490_e13088 * locals.var_fn133_calc_iq__ffd0_dn7))) / locals.var_fn133_calc_iq__two_n_phit0), (((-locals.var_fn133_calc_iq__vdx0_dn14) - (-(assign12490_e13088 * locals.var_fn133_calc_iq__ffd0_dn14))) / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__etad0, locals.var_fn133_calc_iq__etad0_dn2, locals.var_fn133_calc_iq__etad0_dn4, locals.var_fn133_calc_iq__etad0_dn5, locals.var_fn133_calc_iq__etad0_dn7, locals.var_fn133_calc_iq__etad0_dn14,)
    }
};
        locals.var_fn133_calc_iq__etad0 = assign12490_e13096;
        locals.var_fn133_calc_iq__etad0_dn2 = assign12490_e13096_d_n2;
        locals.var_fn133_calc_iq__etad0_dn4 = assign12490_e13096_d_n4;
        locals.var_fn133_calc_iq__etad0_dn5 = assign12490_e13096_d_n5;
        locals.var_fn133_calc_iq__etad0_dn7 = assign12490_e13096_d_n7;
        locals.var_fn133_calc_iq__etad0_dn14 = assign12490_e13096_d_n14;

        let assign12500_e13099: f64 = if locals.var_fn133_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign12500_e13099;

        let (assign12510_e13107, assign12510_e13107_d_n2, assign12510_e13107_d_n4, assign12510_e13107_d_n5, assign12510_e13107_d_n7, assign12510_e13107_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard157 != 0.0)) {
        let assign12510_e13105: f64 = (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0);
        (assign12510_e13105, (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0_dn2), ((locals.var_fn133_calc_iq__qref0_dn4 * locals.var_fn133_calc_iq__etad0) + (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0_dn4)), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0_dn5), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0_dn7), (locals.var_fn133_calc_iq__qref0 * locals.var_fn133_calc_iq__etad0_dn14),)
    } else {
        (locals.var_fn133_calc_iq__qinvd0, locals.var_fn133_calc_iq__qinvd0_dn2, locals.var_fn133_calc_iq__qinvd0_dn4, locals.var_fn133_calc_iq__qinvd0_dn5, locals.var_fn133_calc_iq__qinvd0_dn7, locals.var_fn133_calc_iq__qinvd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd0 = assign12510_e13107;
        locals.var_fn133_calc_iq__qinvd0_dn2 = assign12510_e13107_d_n2;
        locals.var_fn133_calc_iq__qinvd0_dn4 = assign12510_e13107_d_n4;
        locals.var_fn133_calc_iq__qinvd0_dn5 = assign12510_e13107_d_n5;
        locals.var_fn133_calc_iq__qinvd0_dn7 = assign12510_e13107_d_n7;
        locals.var_fn133_calc_iq__qinvd0_dn14 = assign12510_e13107_d_n14;

        let assign12520_e13110: f64 = (-50.0);
        let assign12520_e13111: f64 = if locals.var_fn133_calc_iq__etad0 < assign12520_e13110 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign12520_e13111;

        let (assign12530_e13123, assign12530_e13123_d_n2, assign12530_e13123_d_n4, assign12530_e13123_d_n5, assign12530_e13123_d_n7, assign12530_e13123_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard157 == 0.0)) && (locals.var_guard158 != 0.0)) {
        let assign12530_e13120: f64 = (locals.var_fn133_calc_iq__etad0).exp();
        let assign12530_e13121: f64 = (locals.var_fn133_calc_iq__qref0 * assign12530_e13120);
        (assign12530_e13121, (locals.var_fn133_calc_iq__qref0 * (assign12530_e13120 * locals.var_fn133_calc_iq__etad0_dn2)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12530_e13120) + (locals.var_fn133_calc_iq__qref0 * (assign12530_e13120 * locals.var_fn133_calc_iq__etad0_dn4))), (locals.var_fn133_calc_iq__qref0 * (assign12530_e13120 * locals.var_fn133_calc_iq__etad0_dn5)), (locals.var_fn133_calc_iq__qref0 * (assign12530_e13120 * locals.var_fn133_calc_iq__etad0_dn7)), (locals.var_fn133_calc_iq__qref0 * (assign12530_e13120 * locals.var_fn133_calc_iq__etad0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qinvd0, locals.var_fn133_calc_iq__qinvd0_dn2, locals.var_fn133_calc_iq__qinvd0_dn4, locals.var_fn133_calc_iq__qinvd0_dn5, locals.var_fn133_calc_iq__qinvd0_dn7, locals.var_fn133_calc_iq__qinvd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd0 = assign12530_e13123;
        locals.var_fn133_calc_iq__qinvd0_dn2 = assign12530_e13123_d_n2;
        locals.var_fn133_calc_iq__qinvd0_dn4 = assign12530_e13123_d_n4;
        locals.var_fn133_calc_iq__qinvd0_dn5 = assign12530_e13123_d_n5;
        locals.var_fn133_calc_iq__qinvd0_dn7 = assign12530_e13123_d_n7;
        locals.var_fn133_calc_iq__qinvd0_dn14 = assign12530_e13123_d_n14;

        let (assign12540_e13139, assign12540_e13139_d_n2, assign12540_e13139_d_n4, assign12540_e13139_d_n5, assign12540_e13139_d_n7, assign12540_e13139_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard157 == 0.0)) && (locals.var_guard158 == 0.0)) {
        let assign12540_e13134: f64 = (locals.var_fn133_calc_iq__etad0).exp();
        let assign12540_e13135: f64 = (1.0 + assign12540_e13134);
        let assign12540_e13136: f64 = (assign12540_e13135).ln();
        let assign12540_e13137: f64 = (locals.var_fn133_calc_iq__qref0 * assign12540_e13136);
        (assign12540_e13137, (locals.var_fn133_calc_iq__qref0 * ((assign12540_e13134 * locals.var_fn133_calc_iq__etad0_dn2) / assign12540_e13135)), ((locals.var_fn133_calc_iq__qref0_dn4 * assign12540_e13136) + (locals.var_fn133_calc_iq__qref0 * ((assign12540_e13134 * locals.var_fn133_calc_iq__etad0_dn4) / assign12540_e13135))), (locals.var_fn133_calc_iq__qref0 * ((assign12540_e13134 * locals.var_fn133_calc_iq__etad0_dn5) / assign12540_e13135)), (locals.var_fn133_calc_iq__qref0 * ((assign12540_e13134 * locals.var_fn133_calc_iq__etad0_dn7) / assign12540_e13135)), (locals.var_fn133_calc_iq__qref0 * ((assign12540_e13134 * locals.var_fn133_calc_iq__etad0_dn14) / assign12540_e13135)),)
    } else {
        (locals.var_fn133_calc_iq__qinvd0, locals.var_fn133_calc_iq__qinvd0_dn2, locals.var_fn133_calc_iq__qinvd0_dn4, locals.var_fn133_calc_iq__qinvd0_dn5, locals.var_fn133_calc_iq__qinvd0_dn7, locals.var_fn133_calc_iq__qinvd0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvd0 = assign12540_e13139;
        locals.var_fn133_calc_iq__qinvd0_dn2 = assign12540_e13139_d_n2;
        locals.var_fn133_calc_iq__qinvd0_dn4 = assign12540_e13139_d_n4;
        locals.var_fn133_calc_iq__qinvd0_dn5 = assign12540_e13139_d_n5;
        locals.var_fn133_calc_iq__qinvd0_dn7 = assign12540_e13139_d_n7;
        locals.var_fn133_calc_iq__qinvd0_dn14 = assign12540_e13139_d_n14;

        let (assign12550_e13147, assign12550_e13147_d_n2, assign12550_e13147_d_n4, assign12550_e13147_d_n5, assign12550_e13147_d_n7, assign12550_e13147_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12550_e13143: f64 = (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0);
        let assign12550_e13145: f64 = (assign12550_e13143 + 1e-38);
        (assign12550_e13145, ((locals.var_fn133_calc_iq__qinvs0_dn2 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0_dn2)), ((locals.var_fn133_calc_iq__qinvs0_dn4 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0_dn4)), ((locals.var_fn133_calc_iq__qinvs0_dn5 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0_dn5)), ((locals.var_fn133_calc_iq__qinvs0_dn7 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0_dn7)), ((locals.var_fn133_calc_iq__qinvs0_dn14 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvs0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qs2, locals.var_fn133_calc_iq__qs2_dn2, locals.var_fn133_calc_iq__qs2_dn4, locals.var_fn133_calc_iq__qs2_dn5, locals.var_fn133_calc_iq__qs2_dn7, locals.var_fn133_calc_iq__qs2_dn14,)
    }
};
        locals.var_fn133_calc_iq__qs2 = assign12550_e13147;
        locals.var_fn133_calc_iq__qs2_dn2 = assign12550_e13147_d_n2;
        locals.var_fn133_calc_iq__qs2_dn4 = assign12550_e13147_d_n4;
        locals.var_fn133_calc_iq__qs2_dn5 = assign12550_e13147_d_n5;
        locals.var_fn133_calc_iq__qs2_dn7 = assign12550_e13147_d_n7;
        locals.var_fn133_calc_iq__qs2_dn14 = assign12550_e13147_d_n14;

    }

    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12560_e13155, assign12560_e13155_d_n2, assign12560_e13155_d_n4, assign12560_e13155_d_n5, assign12560_e13155_d_n7, assign12560_e13155_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12560_e13151: f64 = (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0);
        let assign12560_e13153: f64 = (assign12560_e13151 + 1e-57);
        (assign12560_e13153, ((locals.var_fn133_calc_iq__qs2_dn2 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0_dn2)), ((locals.var_fn133_calc_iq__qs2_dn4 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0_dn4)), ((locals.var_fn133_calc_iq__qs2_dn5 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0_dn5)), ((locals.var_fn133_calc_iq__qs2_dn7 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0_dn7)), ((locals.var_fn133_calc_iq__qs2_dn14 * locals.var_fn133_calc_iq__qinvs0) + (locals.var_fn133_calc_iq__qs2 * locals.var_fn133_calc_iq__qinvs0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qs3, locals.var_fn133_calc_iq__qs3_dn2, locals.var_fn133_calc_iq__qs3_dn4, locals.var_fn133_calc_iq__qs3_dn5, locals.var_fn133_calc_iq__qs3_dn7, locals.var_fn133_calc_iq__qs3_dn14,)
    }
};
        locals.var_fn133_calc_iq__qs3 = assign12560_e13155;
        locals.var_fn133_calc_iq__qs3_dn2 = assign12560_e13155_d_n2;
        locals.var_fn133_calc_iq__qs3_dn4 = assign12560_e13155_d_n4;
        locals.var_fn133_calc_iq__qs3_dn5 = assign12560_e13155_d_n5;
        locals.var_fn133_calc_iq__qs3_dn7 = assign12560_e13155_d_n7;
        locals.var_fn133_calc_iq__qs3_dn14 = assign12560_e13155_d_n14;

        let (assign12570_e13163, assign12570_e13163_d_n2, assign12570_e13163_d_n4, assign12570_e13163_d_n5, assign12570_e13163_d_n7, assign12570_e13163_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12570_e13159: f64 = (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0);
        let assign12570_e13161: f64 = (assign12570_e13159 + 1e-38);
        (assign12570_e13161, ((locals.var_fn133_calc_iq__qinvd0_dn2 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0_dn2)), ((locals.var_fn133_calc_iq__qinvd0_dn4 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0_dn4)), ((locals.var_fn133_calc_iq__qinvd0_dn5 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0_dn5)), ((locals.var_fn133_calc_iq__qinvd0_dn7 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0_dn7)), ((locals.var_fn133_calc_iq__qinvd0_dn14 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvd0 * locals.var_fn133_calc_iq__qinvd0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qd2, locals.var_fn133_calc_iq__qd2_dn2, locals.var_fn133_calc_iq__qd2_dn4, locals.var_fn133_calc_iq__qd2_dn5, locals.var_fn133_calc_iq__qd2_dn7, locals.var_fn133_calc_iq__qd2_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd2 = assign12570_e13163;
        locals.var_fn133_calc_iq__qd2_dn2 = assign12570_e13163_d_n2;
        locals.var_fn133_calc_iq__qd2_dn4 = assign12570_e13163_d_n4;
        locals.var_fn133_calc_iq__qd2_dn5 = assign12570_e13163_d_n5;
        locals.var_fn133_calc_iq__qd2_dn7 = assign12570_e13163_d_n7;
        locals.var_fn133_calc_iq__qd2_dn14 = assign12570_e13163_d_n14;

        let (assign12580_e13171, assign12580_e13171_d_n2, assign12580_e13171_d_n4, assign12580_e13171_d_n5, assign12580_e13171_d_n7, assign12580_e13171_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12580_e13167: f64 = (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0);
        let assign12580_e13169: f64 = (assign12580_e13167 + 1e-57);
        (assign12580_e13169, ((locals.var_fn133_calc_iq__qd2_dn2 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0_dn2)), ((locals.var_fn133_calc_iq__qd2_dn4 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0_dn4)), ((locals.var_fn133_calc_iq__qd2_dn5 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0_dn5)), ((locals.var_fn133_calc_iq__qd2_dn7 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0_dn7)), ((locals.var_fn133_calc_iq__qd2_dn14 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qd2 * locals.var_fn133_calc_iq__qinvd0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qd3, locals.var_fn133_calc_iq__qd3_dn2, locals.var_fn133_calc_iq__qd3_dn4, locals.var_fn133_calc_iq__qd3_dn5, locals.var_fn133_calc_iq__qd3_dn7, locals.var_fn133_calc_iq__qd3_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd3 = assign12580_e13171;
        locals.var_fn133_calc_iq__qd3_dn2 = assign12580_e13171_d_n2;
        locals.var_fn133_calc_iq__qd3_dn4 = assign12580_e13171_d_n4;
        locals.var_fn133_calc_iq__qd3_dn5 = assign12580_e13171_d_n5;
        locals.var_fn133_calc_iq__qd3_dn7 = assign12580_e13171_d_n7;
        locals.var_fn133_calc_iq__qd3_dn14 = assign12580_e13171_d_n14;

        let (assign12590_e13179, assign12590_e13179_d_n2, assign12590_e13179_d_n4, assign12590_e13179_d_n5, assign12590_e13179_d_n7, assign12590_e13179_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12590_e13175: f64 = (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0);
        let assign12590_e13177: f64 = (assign12590_e13175 + 1e-38);
        (assign12590_e13177, ((locals.var_fn133_calc_iq__qinvs0_dn2 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0_dn2)), ((locals.var_fn133_calc_iq__qinvs0_dn4 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0_dn4)), ((locals.var_fn133_calc_iq__qinvs0_dn5 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0_dn5)), ((locals.var_fn133_calc_iq__qinvs0_dn7 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0_dn7)), ((locals.var_fn133_calc_iq__qinvs0_dn14 * locals.var_fn133_calc_iq__qinvd0) + (locals.var_fn133_calc_iq__qinvs0 * locals.var_fn133_calc_iq__qinvd0_dn14)),)
    } else {
        (locals.var_fn133_calc_iq__qsqd, locals.var_fn133_calc_iq__qsqd_dn2, locals.var_fn133_calc_iq__qsqd_dn4, locals.var_fn133_calc_iq__qsqd_dn5, locals.var_fn133_calc_iq__qsqd_dn7, locals.var_fn133_calc_iq__qsqd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qsqd = assign12590_e13179;
        locals.var_fn133_calc_iq__qsqd_dn2 = assign12590_e13179_d_n2;
        locals.var_fn133_calc_iq__qsqd_dn4 = assign12590_e13179_d_n4;
        locals.var_fn133_calc_iq__qsqd_dn5 = assign12590_e13179_d_n5;
        locals.var_fn133_calc_iq__qsqd_dn7 = assign12590_e13179_d_n7;
        locals.var_fn133_calc_iq__qsqd_dn14 = assign12590_e13179_d_n14;

        let (assign12600_e13197, assign12600_e13197_d_n2, assign12600_e13197_d_n4, assign12600_e13197_d_n5, assign12600_e13197_d_n7, assign12600_e13197_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12600_e13183: f64 = (2.0 / 3.0);
        let assign12600_e13186: f64 = (locals.var_fn133_calc_iq__qs2 + locals.var_fn133_calc_iq__qd2);
        let assign12600_e13188: f64 = (assign12600_e13186 + locals.var_fn133_calc_iq__qsqd);
        let assign12600_e13189: f64 = (assign12600_e13183 * assign12600_e13188);
        let assign12600_e13192: f64 = (locals.var_fn133_calc_iq__qinvs0 + locals.var_fn133_calc_iq__qinvd0);
        let assign12600_e13194: f64 = (assign12600_e13192 + 2e-19);
        let assign12600_e13195: f64 = (assign12600_e13189 / assign12600_e13194);
        (assign12600_e13195, ((((assign12600_e13183 * ((locals.var_fn133_calc_iq__qs2_dn2 + locals.var_fn133_calc_iq__qd2_dn2) + locals.var_fn133_calc_iq__qsqd_dn2)) * assign12600_e13194) - (assign12600_e13189 * (locals.var_fn133_calc_iq__qinvs0_dn2 + locals.var_fn133_calc_iq__qinvd0_dn2))) / (assign12600_e13194 * assign12600_e13194)), ((((assign12600_e13183 * ((locals.var_fn133_calc_iq__qs2_dn4 + locals.var_fn133_calc_iq__qd2_dn4) + locals.var_fn133_calc_iq__qsqd_dn4)) * assign12600_e13194) - (assign12600_e13189 * (locals.var_fn133_calc_iq__qinvs0_dn4 + locals.var_fn133_calc_iq__qinvd0_dn4))) / (assign12600_e13194 * assign12600_e13194)), ((((assign12600_e13183 * ((locals.var_fn133_calc_iq__qs2_dn5 + locals.var_fn133_calc_iq__qd2_dn5) + locals.var_fn133_calc_iq__qsqd_dn5)) * assign12600_e13194) - (assign12600_e13189 * (locals.var_fn133_calc_iq__qinvs0_dn5 + locals.var_fn133_calc_iq__qinvd0_dn5))) / (assign12600_e13194 * assign12600_e13194)), ((((assign12600_e13183 * ((locals.var_fn133_calc_iq__qs2_dn7 + locals.var_fn133_calc_iq__qd2_dn7) + locals.var_fn133_calc_iq__qsqd_dn7)) * assign12600_e13194) - (assign12600_e13189 * (locals.var_fn133_calc_iq__qinvs0_dn7 + locals.var_fn133_calc_iq__qinvd0_dn7))) / (assign12600_e13194 * assign12600_e13194)), ((((assign12600_e13183 * ((locals.var_fn133_calc_iq__qs2_dn14 + locals.var_fn133_calc_iq__qd2_dn14) + locals.var_fn133_calc_iq__qsqd_dn14)) * assign12600_e13194) - (assign12600_e13189 * (locals.var_fn133_calc_iq__qinvs0_dn14 + locals.var_fn133_calc_iq__qinvd0_dn14))) / (assign12600_e13194 * assign12600_e13194)),)
    } else {
        (locals.var_fn133_calc_iq__qinvdd, locals.var_fn133_calc_iq__qinvdd_dn2, locals.var_fn133_calc_iq__qinvdd_dn4, locals.var_fn133_calc_iq__qinvdd_dn5, locals.var_fn133_calc_iq__qinvdd_dn7, locals.var_fn133_calc_iq__qinvdd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvdd = assign12600_e13197;
        locals.var_fn133_calc_iq__qinvdd_dn2 = assign12600_e13197_d_n2;
        locals.var_fn133_calc_iq__qinvdd_dn4 = assign12600_e13197_d_n4;
        locals.var_fn133_calc_iq__qinvdd_dn5 = assign12600_e13197_d_n5;
        locals.var_fn133_calc_iq__qinvdd_dn7 = assign12600_e13197_d_n7;
        locals.var_fn133_calc_iq__qinvdd_dn14 = assign12600_e13197_d_n14;

        let (assign12610_e13231, assign12610_e13231_d_n2, assign12610_e13231_d_n4, assign12610_e13231_d_n5, assign12610_e13231_d_n7, assign12610_e13231_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12610_e13202: f64 = (2.0 * locals.var_fn133_calc_iq__qs3);
        let assign12610_e13205: f64 = (3.0 * locals.var_fn133_calc_iq__qd3);
        let assign12610_e13206: f64 = (assign12610_e13202 + assign12610_e13205);
        let assign12610_e13209: f64 = (4.0 * locals.var_fn133_calc_iq__qs2);
        let assign12610_e13211: f64 = (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0);
        let assign12610_e13212: f64 = (assign12610_e13206 + assign12610_e13211);
        let assign12610_e13215: f64 = (6.0 * locals.var_fn133_calc_iq__qd2);
        let assign12610_e13217: f64 = (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0);
        let assign12610_e13218: f64 = (assign12610_e13212 + assign12610_e13217);
        let assign12610_e13219: f64 = (2.0 * assign12610_e13218);
        let assign12610_e13223: f64 = (locals.var_fn133_calc_iq__qs2 + locals.var_fn133_calc_iq__qd2);
        let assign12610_e13226: f64 = (2.0 * locals.var_fn133_calc_iq__qsqd);
        let assign12610_e13227: f64 = (assign12610_e13223 + assign12610_e13226);
        let assign12610_e13228: f64 = (15.0 * assign12610_e13227);
        let assign12610_e13229: f64 = (assign12610_e13219 / assign12610_e13228);
        (assign12610_e13229, ((((2.0 * ((((2.0 * locals.var_fn133_calc_iq__qs3_dn2) + (3.0 * locals.var_fn133_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn133_calc_iq__qs2_dn2) * locals.var_fn133_calc_iq__qinvd0) + (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn133_calc_iq__qd2_dn2) * locals.var_fn133_calc_iq__qinvs0) + (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0_dn2)))) * assign12610_e13228) - (assign12610_e13219 * (15.0 * ((locals.var_fn133_calc_iq__qs2_dn2 + locals.var_fn133_calc_iq__qd2_dn2) + (2.0 * locals.var_fn133_calc_iq__qsqd_dn2))))) / (assign12610_e13228 * assign12610_e13228)), ((((2.0 * ((((2.0 * locals.var_fn133_calc_iq__qs3_dn4) + (3.0 * locals.var_fn133_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn133_calc_iq__qs2_dn4) * locals.var_fn133_calc_iq__qinvd0) + (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn133_calc_iq__qd2_dn4) * locals.var_fn133_calc_iq__qinvs0) + (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0_dn4)))) * assign12610_e13228) - (assign12610_e13219 * (15.0 * ((locals.var_fn133_calc_iq__qs2_dn4 + locals.var_fn133_calc_iq__qd2_dn4) + (2.0 * locals.var_fn133_calc_iq__qsqd_dn4))))) / (assign12610_e13228 * assign12610_e13228)), ((((2.0 * ((((2.0 * locals.var_fn133_calc_iq__qs3_dn5) + (3.0 * locals.var_fn133_calc_iq__qd3_dn5)) + (((4.0 * locals.var_fn133_calc_iq__qs2_dn5) * locals.var_fn133_calc_iq__qinvd0) + (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0_dn5))) + (((6.0 * locals.var_fn133_calc_iq__qd2_dn5) * locals.var_fn133_calc_iq__qinvs0) + (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0_dn5)))) * assign12610_e13228) - (assign12610_e13219 * (15.0 * ((locals.var_fn133_calc_iq__qs2_dn5 + locals.var_fn133_calc_iq__qd2_dn5) + (2.0 * locals.var_fn133_calc_iq__qsqd_dn5))))) / (assign12610_e13228 * assign12610_e13228)), ((((2.0 * ((((2.0 * locals.var_fn133_calc_iq__qs3_dn7) + (3.0 * locals.var_fn133_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn133_calc_iq__qs2_dn7) * locals.var_fn133_calc_iq__qinvd0) + (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn133_calc_iq__qd2_dn7) * locals.var_fn133_calc_iq__qinvs0) + (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0_dn7)))) * assign12610_e13228) - (assign12610_e13219 * (15.0 * ((locals.var_fn133_calc_iq__qs2_dn7 + locals.var_fn133_calc_iq__qd2_dn7) + (2.0 * locals.var_fn133_calc_iq__qsqd_dn7))))) / (assign12610_e13228 * assign12610_e13228)), ((((2.0 * ((((2.0 * locals.var_fn133_calc_iq__qs3_dn14) + (3.0 * locals.var_fn133_calc_iq__qd3_dn14)) + (((4.0 * locals.var_fn133_calc_iq__qs2_dn14) * locals.var_fn133_calc_iq__qinvd0) + (assign12610_e13209 * locals.var_fn133_calc_iq__qinvd0_dn14))) + (((6.0 * locals.var_fn133_calc_iq__qd2_dn14) * locals.var_fn133_calc_iq__qinvs0) + (assign12610_e13215 * locals.var_fn133_calc_iq__qinvs0_dn14)))) * assign12610_e13228) - (assign12610_e13219 * (15.0 * ((locals.var_fn133_calc_iq__qs2_dn14 + locals.var_fn133_calc_iq__qd2_dn14) + (2.0 * locals.var_fn133_calc_iq__qsqd_dn14))))) / (assign12610_e13228 * assign12610_e13228)),)
    } else {
        (locals.var_fn133_calc_iq__qd1, locals.var_fn133_calc_iq__qd1_dn2, locals.var_fn133_calc_iq__qd1_dn4, locals.var_fn133_calc_iq__qd1_dn5, locals.var_fn133_calc_iq__qd1_dn7, locals.var_fn133_calc_iq__qd1_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd1 = assign12610_e13231;
        locals.var_fn133_calc_iq__qd1_dn2 = assign12610_e13231_d_n2;
        locals.var_fn133_calc_iq__qd1_dn4 = assign12610_e13231_d_n4;
        locals.var_fn133_calc_iq__qd1_dn5 = assign12610_e13231_d_n5;
        locals.var_fn133_calc_iq__qd1_dn7 = assign12610_e13231_d_n7;
        locals.var_fn133_calc_iq__qd1_dn14 = assign12610_e13231_d_n14;

        let (assign12620_e13237, assign12620_e13237_d_n2, assign12620_e13237_d_n4, assign12620_e13237_d_n5, assign12620_e13237_d_n7, assign12620_e13237_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12620_e13235: f64 = (locals.var_fn133_calc_iq__qinvdd - locals.var_fn133_calc_iq__qd1);
        (assign12620_e13235, (locals.var_fn133_calc_iq__qinvdd_dn2 - locals.var_fn133_calc_iq__qd1_dn2), (locals.var_fn133_calc_iq__qinvdd_dn4 - locals.var_fn133_calc_iq__qd1_dn4), (locals.var_fn133_calc_iq__qinvdd_dn5 - locals.var_fn133_calc_iq__qd1_dn5), (locals.var_fn133_calc_iq__qinvdd_dn7 - locals.var_fn133_calc_iq__qd1_dn7), (locals.var_fn133_calc_iq__qinvdd_dn14 - locals.var_fn133_calc_iq__qd1_dn14),)
    } else {
        (locals.var_fn133_calc_iq__qs, locals.var_fn133_calc_iq__qs_dn2, locals.var_fn133_calc_iq__qs_dn4, locals.var_fn133_calc_iq__qs_dn5, locals.var_fn133_calc_iq__qs_dn7, locals.var_fn133_calc_iq__qs_dn14,)
    }
};
        locals.var_fn133_calc_iq__qs = assign12620_e13237;
        locals.var_fn133_calc_iq__qs_dn2 = assign12620_e13237_d_n2;
        locals.var_fn133_calc_iq__qs_dn4 = assign12620_e13237_d_n4;
        locals.var_fn133_calc_iq__qs_dn5 = assign12620_e13237_d_n5;
        locals.var_fn133_calc_iq__qs_dn7 = assign12620_e13237_d_n7;
        locals.var_fn133_calc_iq__qs_dn14 = assign12620_e13237_d_n14;

        let (assign12630_e13241, assign12630_e13241_d_n2, assign12630_e13241_d_n4, assign12630_e13241_d_n5, assign12630_e13241_d_n7, assign12630_e13241_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qd1, locals.var_fn133_calc_iq__qd1_dn2, locals.var_fn133_calc_iq__qd1_dn4, locals.var_fn133_calc_iq__qd1_dn5, locals.var_fn133_calc_iq__qd1_dn7, locals.var_fn133_calc_iq__qd1_dn14,)
    } else {
        (locals.var_fn133_calc_iq__qd, locals.var_fn133_calc_iq__qd_dn2, locals.var_fn133_calc_iq__qd_dn4, locals.var_fn133_calc_iq__qd_dn5, locals.var_fn133_calc_iq__qd_dn7, locals.var_fn133_calc_iq__qd_dn14,)
    }
};
        locals.var_fn133_calc_iq__qd = assign12630_e13241;
        locals.var_fn133_calc_iq__qd_dn2 = assign12630_e13241_d_n2;
        locals.var_fn133_calc_iq__qd_dn4 = assign12630_e13241_d_n4;
        locals.var_fn133_calc_iq__qd_dn5 = assign12630_e13241_d_n5;
        locals.var_fn133_calc_iq__qd_dn7 = assign12630_e13241_d_n7;
        locals.var_fn133_calc_iq__qd_dn14 = assign12630_e13241_d_n14;

        let (assign12640_e13255, assign12640_e13255_d_n2, assign12640_e13255_d_n4, assign12640_e13255_d_n5, assign12640_e13255_d_n7, assign12640_e13255_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12640_e13245: f64 = (locals.var_fn133_calc_iq__w * locals.var_fn133_calc_iq__ngf);
        let assign12640_e13247: f64 = (assign12640_e13245 * locals.var_fn133_calc_iq__lin);
        let assign12640_e13249: f64 = (assign12640_e13247 * locals.var_fn133_calc_iq__type);
        let assign12640_e13251: f64 = (assign12640_e13249 * locals.var_fn133_calc_iq__qs);
        let assign12640_e13253: f64 = (assign12640_e13251 * locals.var_fn133_calc_iq__trapfracdl);
        (assign12640_e13253, ((assign12640_e13249 * locals.var_fn133_calc_iq__qs_dn2) * locals.var_fn133_calc_iq__trapfracdl), ((assign12640_e13249 * locals.var_fn133_calc_iq__qs_dn4) * locals.var_fn133_calc_iq__trapfracdl), ((assign12640_e13249 * locals.var_fn133_calc_iq__qs_dn5) * locals.var_fn133_calc_iq__trapfracdl), ((assign12640_e13249 * locals.var_fn133_calc_iq__qs_dn7) * locals.var_fn133_calc_iq__trapfracdl), ((assign12640_e13249 * locals.var_fn133_calc_iq__qs_dn14) * locals.var_fn133_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn133_calc_iq__qgsout, locals.var_fn133_calc_iq__qgsout_dn2, locals.var_fn133_calc_iq__qgsout_dn4, locals.var_fn133_calc_iq__qgsout_dn5, locals.var_fn133_calc_iq__qgsout_dn7, locals.var_fn133_calc_iq__qgsout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qgsout = assign12640_e13255;
        locals.var_fn133_calc_iq__qgsout_dn2 = assign12640_e13255_d_n2;
        locals.var_fn133_calc_iq__qgsout_dn4 = assign12640_e13255_d_n4;
        locals.var_fn133_calc_iq__qgsout_dn5 = assign12640_e13255_d_n5;
        locals.var_fn133_calc_iq__qgsout_dn7 = assign12640_e13255_d_n7;
        locals.var_fn133_calc_iq__qgsout_dn14 = assign12640_e13255_d_n14;

        let (assign12650_e13269, assign12650_e13269_d_n2, assign12650_e13269_d_n4, assign12650_e13269_d_n5, assign12650_e13269_d_n7, assign12650_e13269_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        let assign12650_e13259: f64 = (locals.var_fn133_calc_iq__w * locals.var_fn133_calc_iq__ngf);
        let assign12650_e13261: f64 = (assign12650_e13259 * locals.var_fn133_calc_iq__lin);
        let assign12650_e13263: f64 = (assign12650_e13261 * locals.var_fn133_calc_iq__type);
        let assign12650_e13265: f64 = (assign12650_e13263 * locals.var_fn133_calc_iq__qd);
        let assign12650_e13267: f64 = (assign12650_e13265 * locals.var_fn133_calc_iq__trapfracdl);
        (assign12650_e13267, ((assign12650_e13263 * locals.var_fn133_calc_iq__qd_dn2) * locals.var_fn133_calc_iq__trapfracdl), ((assign12650_e13263 * locals.var_fn133_calc_iq__qd_dn4) * locals.var_fn133_calc_iq__trapfracdl), ((assign12650_e13263 * locals.var_fn133_calc_iq__qd_dn5) * locals.var_fn133_calc_iq__trapfracdl), ((assign12650_e13263 * locals.var_fn133_calc_iq__qd_dn7) * locals.var_fn133_calc_iq__trapfracdl), ((assign12650_e13263 * locals.var_fn133_calc_iq__qd_dn14) * locals.var_fn133_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn133_calc_iq__qgdout, locals.var_fn133_calc_iq__qgdout_dn2, locals.var_fn133_calc_iq__qgdout_dn4, locals.var_fn133_calc_iq__qgdout_dn5, locals.var_fn133_calc_iq__qgdout_dn7, locals.var_fn133_calc_iq__qgdout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qgdout = assign12650_e13269;
        locals.var_fn133_calc_iq__qgdout_dn2 = assign12650_e13269_d_n2;
        locals.var_fn133_calc_iq__qgdout_dn4 = assign12650_e13269_d_n4;
        locals.var_fn133_calc_iq__qgdout_dn5 = assign12650_e13269_d_n5;
        locals.var_fn133_calc_iq__qgdout_dn7 = assign12650_e13269_d_n7;
        locals.var_fn133_calc_iq__qgdout_dn14 = assign12650_e13269_d_n14;

        let assign12660_e13272: f64 = if locals.var_fn133_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign12660_e13272;

        let (assign12670_e13288, assign12670_e13288_d_n2, assign12670_e13288_d_n4, assign12670_e13288_d_n5, assign12670_e13288_d_n7,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) {
        let assign12670_e13280: f64 = (p.p51 * 0.5);
        let assign12670_e13282: f64 = (assign12670_e13280 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12670_e13283: f64 = (locals.var_fn133_calc_iq__vtof - assign12670_e13282);
        let assign12670_e13284: f64 = (locals.var_fn133_calc_iq__vcin - assign12670_e13283);
        let assign12670_e13286: f64 = (assign12670_e13284 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12670_e13286, (locals.var_fn133_calc_iq__vcin_dn2 / locals.var_fn133_calc_iq__two_n_phit0), ((((-(locals.var_fn133_calc_iq__vtof_dn4 - (assign12670_e13280 * locals.var_fn133_calc_iq__alpha_phit_dn4))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12670_e13284 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), (locals.var_fn133_calc_iq__vcin_dn5 / locals.var_fn133_calc_iq__two_n_phit0), (locals.var_fn133_calc_iq__vcin_dn7 / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__etac, locals.var_fn133_calc_iq__etac_dn2, locals.var_fn133_calc_iq__etac_dn4, locals.var_fn133_calc_iq__etac_dn5, locals.var_fn133_calc_iq__etac_dn7,)
    }
};
        locals.var_fn133_calc_iq__etac = assign12670_e13288;
        locals.var_fn133_calc_iq__etac_dn2 = assign12670_e13288_d_n2;
        locals.var_fn133_calc_iq__etac_dn4 = assign12670_e13288_d_n4;
        locals.var_fn133_calc_iq__etac_dn5 = assign12670_e13288_d_n5;
        locals.var_fn133_calc_iq__etac_dn7 = assign12670_e13288_d_n7;

        let assign12680_e13291: f64 = if locals.var_fn133_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign12680_e13291;

        let (assign12690_e13299, assign12690_e13299_d_n2, assign12690_e13299_d_n3, assign12690_e13299_d_n4, assign12690_e13299_d_n5, assign12690_e13299_d_n7, assign12690_e13299_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard160 != 0.0)) {
        (locals.var_fn133_calc_iq__etac, locals.var_fn133_calc_iq__etac_dn2, 0.0, locals.var_fn133_calc_iq__etac_dn4, locals.var_fn133_calc_iq__etac_dn5, locals.var_fn133_calc_iq__etac_dn7, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12690_e13299;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12690_e13299_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12690_e13299_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12690_e13299_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12690_e13299_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12690_e13299_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12690_e13299_d_n14;

        let assign12700_e13302: f64 = (-50.0);
        let assign12700_e13303: f64 = if locals.var_fn133_calc_iq__etac < assign12700_e13302 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign12700_e13303;

        let (assign12710_e13315, assign12710_e13315_d_n2, assign12710_e13315_d_n3, assign12710_e13315_d_n4, assign12710_e13315_d_n5, assign12710_e13315_d_n7, assign12710_e13315_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard160 == 0.0)) && (locals.var_guard161 != 0.0)) {
        let assign12710_e13313: f64 = (locals.var_fn133_calc_iq__etac).exp();
        (assign12710_e13313, (assign12710_e13313 * locals.var_fn133_calc_iq__etac_dn2), 0.0, (assign12710_e13313 * locals.var_fn133_calc_iq__etac_dn4), (assign12710_e13313 * locals.var_fn133_calc_iq__etac_dn5), (assign12710_e13313 * locals.var_fn133_calc_iq__etac_dn7), 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12710_e13315;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12710_e13315_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12710_e13315_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12710_e13315_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12710_e13315_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12710_e13315_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12710_e13315_d_n14;

        let (assign12720_e13331, assign12720_e13331_d_n2, assign12720_e13331_d_n3, assign12720_e13331_d_n4, assign12720_e13331_d_n5, assign12720_e13331_d_n7, assign12720_e13331_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard160 == 0.0)) && (locals.var_guard161 == 0.0)) {
        let assign12720_e13327: f64 = (locals.var_fn133_calc_iq__etac).exp();
        let assign12720_e13328: f64 = (1.0 + assign12720_e13327);
        let assign12720_e13329: f64 = (assign12720_e13328).ln();
        (assign12720_e13329, ((assign12720_e13327 * locals.var_fn133_calc_iq__etac_dn2) / assign12720_e13328), 0.0, ((assign12720_e13327 * locals.var_fn133_calc_iq__etac_dn4) / assign12720_e13328), ((assign12720_e13327 * locals.var_fn133_calc_iq__etac_dn5) / assign12720_e13328), ((assign12720_e13327 * locals.var_fn133_calc_iq__etac_dn7) / assign12720_e13328), 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12720_e13331;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12720_e13331_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12720_e13331_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12720_e13331_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12720_e13331_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12720_e13331_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12720_e13331_d_n14;

        let (assign12730_e13349, assign12730_e13349_d_n2, assign12730_e13349_d_n3, assign12730_e13349_d_n4, assign12730_e13349_d_n5, assign12730_e13349_d_n7, assign12730_e13349_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) {
        let assign12730_e13337: f64 = (locals.var_fn133_calc_iq__w * locals.var_fn133_calc_iq__ngf);
        let assign12730_e13339: f64 = (assign12730_e13337 * locals.var_fn133_calc_iq__type);
        let assign12730_e13341: f64 = (assign12730_e13339 * locals.var_fn133_calc_iq__cc);
        let assign12730_e13343: f64 = (assign12730_e13341 * locals.var_fn133_calc_iq__two_n_phit0);
        let assign12730_e13345: f64 = (assign12730_e13343 * locals.var_fn133_calc_iq__exparg);
        let assign12730_e13347: f64 = (assign12730_e13345 * locals.var_fn133_calc_iq__trapfracdl);
        (assign12730_e13347, ((assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn2) * locals.var_fn133_calc_iq__trapfracdl), ((assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn3) * locals.var_fn133_calc_iq__trapfracdl), ((((((assign12730_e13339 * locals.var_fn133_calc_iq__cc_dn4) * locals.var_fn133_calc_iq__two_n_phit0) + (assign12730_e13341 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) * locals.var_fn133_calc_iq__exparg) + (assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn4)) * locals.var_fn133_calc_iq__trapfracdl), ((assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn5) * locals.var_fn133_calc_iq__trapfracdl), ((assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn7) * locals.var_fn133_calc_iq__trapfracdl), ((assign12730_e13343 * locals.var_fn133_calc_iq__exparg_dn14) * locals.var_fn133_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn133_calc_iq__qcout, locals.var_fn133_calc_iq__qcout_dn2, locals.var_fn133_calc_iq__qcout_dn3, locals.var_fn133_calc_iq__qcout_dn4, locals.var_fn133_calc_iq__qcout_dn5, locals.var_fn133_calc_iq__qcout_dn7, locals.var_fn133_calc_iq__qcout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qcout = assign12730_e13349;
        locals.var_fn133_calc_iq__qcout_dn2 = assign12730_e13349_d_n2;
        locals.var_fn133_calc_iq__qcout_dn3 = assign12730_e13349_d_n3;
        locals.var_fn133_calc_iq__qcout_dn4 = assign12730_e13349_d_n4;
        locals.var_fn133_calc_iq__qcout_dn5 = assign12730_e13349_d_n5;
        locals.var_fn133_calc_iq__qcout_dn7 = assign12730_e13349_d_n7;
        locals.var_fn133_calc_iq__qcout_dn14 = assign12730_e13349_d_n14;

        let (assign12740_e13365, assign12740_e13365_d_n3, assign12740_e13365_d_n4, assign12740_e13365_d_n5,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) {
        let assign12740_e13357: f64 = (p.p51 * 0.5);
        let assign12740_e13359: f64 = (assign12740_e13357 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12740_e13360: f64 = (locals.var_fn133_calc_iq__vtof - assign12740_e13359);
        let assign12740_e13361: f64 = (locals.var_fn133_calc_iq__vbin - assign12740_e13360);
        let assign12740_e13363: f64 = (assign12740_e13361 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12740_e13363, (locals.var_fn133_calc_iq__vbin_dn3 / locals.var_fn133_calc_iq__two_n_phit0), ((((-(locals.var_fn133_calc_iq__vtof_dn4 - (assign12740_e13357 * locals.var_fn133_calc_iq__alpha_phit_dn4))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12740_e13361 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), (locals.var_fn133_calc_iq__vbin_dn5 / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__etab, locals.var_fn133_calc_iq__etab_dn3, locals.var_fn133_calc_iq__etab_dn4, locals.var_fn133_calc_iq__etab_dn5,)
    }
};
        locals.var_fn133_calc_iq__etab = assign12740_e13365;
        locals.var_fn133_calc_iq__etab_dn3 = assign12740_e13365_d_n3;
        locals.var_fn133_calc_iq__etab_dn4 = assign12740_e13365_d_n4;
        locals.var_fn133_calc_iq__etab_dn5 = assign12740_e13365_d_n5;

        let assign12750_e13368: f64 = if locals.var_fn133_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign12750_e13368;

        let (assign12760_e13376, assign12760_e13376_d_n2, assign12760_e13376_d_n3, assign12760_e13376_d_n4, assign12760_e13376_d_n5, assign12760_e13376_d_n7, assign12760_e13376_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard162 != 0.0)) {
        (locals.var_fn133_calc_iq__etab, 0.0, locals.var_fn133_calc_iq__etab_dn3, locals.var_fn133_calc_iq__etab_dn4, locals.var_fn133_calc_iq__etab_dn5, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12760_e13376;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12760_e13376_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12760_e13376_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12760_e13376_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12760_e13376_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12760_e13376_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12760_e13376_d_n14;

        let assign12770_e13379: f64 = (-50.0);
        let assign12770_e13380: f64 = if locals.var_fn133_calc_iq__etab < assign12770_e13379 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign12770_e13380;

        let (assign12780_e13392, assign12780_e13392_d_n2, assign12780_e13392_d_n3, assign12780_e13392_d_n4, assign12780_e13392_d_n5, assign12780_e13392_d_n7, assign12780_e13392_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign12780_e13390: f64 = (locals.var_fn133_calc_iq__etab).exp();
        (assign12780_e13390, 0.0, (assign12780_e13390 * locals.var_fn133_calc_iq__etab_dn3), (assign12780_e13390 * locals.var_fn133_calc_iq__etab_dn4), (assign12780_e13390 * locals.var_fn133_calc_iq__etab_dn5), 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12780_e13392;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12780_e13392_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12780_e13392_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12780_e13392_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12780_e13392_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12780_e13392_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12780_e13392_d_n14;

        let (assign12790_e13408, assign12790_e13408_d_n2, assign12790_e13408_d_n3, assign12790_e13408_d_n4, assign12790_e13408_d_n5, assign12790_e13408_d_n7, assign12790_e13408_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) {
        let assign12790_e13404: f64 = (locals.var_fn133_calc_iq__etab).exp();
        let assign12790_e13405: f64 = (1.0 + assign12790_e13404);
        let assign12790_e13406: f64 = (assign12790_e13405).ln();
        (assign12790_e13406, 0.0, ((assign12790_e13404 * locals.var_fn133_calc_iq__etab_dn3) / assign12790_e13405), ((assign12790_e13404 * locals.var_fn133_calc_iq__etab_dn4) / assign12790_e13405), ((assign12790_e13404 * locals.var_fn133_calc_iq__etab_dn5) / assign12790_e13405), 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12790_e13408;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12790_e13408_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12790_e13408_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12790_e13408_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12790_e13408_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12790_e13408_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12790_e13408_d_n14;

        let (assign12800_e13426, assign12800_e13426_d_n2, assign12800_e13426_d_n3, assign12800_e13426_d_n4, assign12800_e13426_d_n5, assign12800_e13426_d_n7, assign12800_e13426_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 != 0.0)) {
        let assign12800_e13414: f64 = (locals.var_fn133_calc_iq__w * locals.var_fn133_calc_iq__ngf);
        let assign12800_e13416: f64 = (assign12800_e13414 * locals.var_fn133_calc_iq__type);
        let assign12800_e13418: f64 = (assign12800_e13416 * locals.var_fn133_calc_iq__cb);
        let assign12800_e13420: f64 = (assign12800_e13418 * locals.var_fn133_calc_iq__two_n_phit0);
        let assign12800_e13422: f64 = (assign12800_e13420 * locals.var_fn133_calc_iq__exparg);
        let assign12800_e13424: f64 = (assign12800_e13422 * locals.var_fn133_calc_iq__trapfracdl);
        (assign12800_e13424, ((assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn2) * locals.var_fn133_calc_iq__trapfracdl), ((assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn3) * locals.var_fn133_calc_iq__trapfracdl), ((((((assign12800_e13416 * locals.var_fn133_calc_iq__cb_dn4) * locals.var_fn133_calc_iq__two_n_phit0) + (assign12800_e13418 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) * locals.var_fn133_calc_iq__exparg) + (assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn4)) * locals.var_fn133_calc_iq__trapfracdl), ((assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn5) * locals.var_fn133_calc_iq__trapfracdl), ((assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn7) * locals.var_fn133_calc_iq__trapfracdl), ((assign12800_e13420 * locals.var_fn133_calc_iq__exparg_dn14) * locals.var_fn133_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn133_calc_iq__qbout, locals.var_fn133_calc_iq__qbout_dn2, locals.var_fn133_calc_iq__qbout_dn3, locals.var_fn133_calc_iq__qbout_dn4, locals.var_fn133_calc_iq__qbout_dn5, locals.var_fn133_calc_iq__qbout_dn7, locals.var_fn133_calc_iq__qbout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qbout = assign12800_e13426;
        locals.var_fn133_calc_iq__qbout_dn2 = assign12800_e13426_d_n2;
        locals.var_fn133_calc_iq__qbout_dn3 = assign12800_e13426_d_n3;
        locals.var_fn133_calc_iq__qbout_dn4 = assign12800_e13426_d_n4;
        locals.var_fn133_calc_iq__qbout_dn5 = assign12800_e13426_d_n5;
        locals.var_fn133_calc_iq__qbout_dn7 = assign12800_e13426_d_n7;
        locals.var_fn133_calc_iq__qbout_dn14 = assign12800_e13426_d_n14;

        let (assign12810_e13433, assign12810_e13433_d_n2, assign12810_e13433_d_n3, assign12810_e13433_d_n4, assign12810_e13433_d_n5, assign12810_e13433_d_n7, assign12810_e13433_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qcout, locals.var_fn133_calc_iq__qcout_dn2, locals.var_fn133_calc_iq__qcout_dn3, locals.var_fn133_calc_iq__qcout_dn4, locals.var_fn133_calc_iq__qcout_dn5, locals.var_fn133_calc_iq__qcout_dn7, locals.var_fn133_calc_iq__qcout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qcout = assign12810_e13433;
        locals.var_fn133_calc_iq__qcout_dn2 = assign12810_e13433_d_n2;
        locals.var_fn133_calc_iq__qcout_dn3 = assign12810_e13433_d_n3;
        locals.var_fn133_calc_iq__qcout_dn4 = assign12810_e13433_d_n4;
        locals.var_fn133_calc_iq__qcout_dn5 = assign12810_e13433_d_n5;
        locals.var_fn133_calc_iq__qcout_dn7 = assign12810_e13433_d_n7;
        locals.var_fn133_calc_iq__qcout_dn14 = assign12810_e13433_d_n14;

        let (assign12820_e13440, assign12820_e13440_d_n2, assign12820_e13440_d_n3, assign12820_e13440_d_n4, assign12820_e13440_d_n5, assign12820_e13440_d_n7, assign12820_e13440_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard159 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qbout, locals.var_fn133_calc_iq__qbout_dn2, locals.var_fn133_calc_iq__qbout_dn3, locals.var_fn133_calc_iq__qbout_dn4, locals.var_fn133_calc_iq__qbout_dn5, locals.var_fn133_calc_iq__qbout_dn7, locals.var_fn133_calc_iq__qbout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qbout = assign12820_e13440;
        locals.var_fn133_calc_iq__qbout_dn2 = assign12820_e13440_d_n2;
        locals.var_fn133_calc_iq__qbout_dn3 = assign12820_e13440_d_n3;
        locals.var_fn133_calc_iq__qbout_dn4 = assign12820_e13440_d_n4;
        locals.var_fn133_calc_iq__qbout_dn5 = assign12820_e13440_d_n5;
        locals.var_fn133_calc_iq__qbout_dn7 = assign12820_e13440_d_n7;
        locals.var_fn133_calc_iq__qbout_dn14 = assign12820_e13440_d_n14;

        let assign12830_e13443: f64 = if locals.var_fn133_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard164 = assign12830_e13443;

        let (assign12840_e13459, assign12840_e13459_d_n2, assign12840_e13459_d_n4, assign12840_e13459_d_n5, assign12840_e13459_d_n7,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard164 != 0.0)) {
        let assign12840_e13451: f64 = (p.p51 * 0.5);
        let assign12840_e13453: f64 = (assign12840_e13451 * locals.var_fn133_calc_iq__alpha_phit);
        let assign12840_e13454: f64 = (locals.var_fn133_calc_iq__vtof - assign12840_e13453);
        let assign12840_e13455: f64 = (locals.var_fn133_calc_iq__vgsin - assign12840_e13454);
        let assign12840_e13457: f64 = (assign12840_e13455 / locals.var_fn133_calc_iq__two_n_phit0);
        (assign12840_e13457, (locals.var_fn133_calc_iq__vgsin_dn2 / locals.var_fn133_calc_iq__two_n_phit0), ((((-(locals.var_fn133_calc_iq__vtof_dn4 - (assign12840_e13451 * locals.var_fn133_calc_iq__alpha_phit_dn4))) * locals.var_fn133_calc_iq__two_n_phit0) - (assign12840_e13455 * locals.var_fn133_calc_iq__two_n_phit0_dn4)) / (locals.var_fn133_calc_iq__two_n_phit0 * locals.var_fn133_calc_iq__two_n_phit0)), (locals.var_fn133_calc_iq__vgsin_dn5 / locals.var_fn133_calc_iq__two_n_phit0), (locals.var_fn133_calc_iq__vgsin_dn7 / locals.var_fn133_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn133_calc_iq__etags, locals.var_fn133_calc_iq__etags_dn2, locals.var_fn133_calc_iq__etags_dn4, locals.var_fn133_calc_iq__etags_dn5, locals.var_fn133_calc_iq__etags_dn7,)
    }
};
        locals.var_fn133_calc_iq__etags = assign12840_e13459;
        locals.var_fn133_calc_iq__etags_dn2 = assign12840_e13459_d_n2;
        locals.var_fn133_calc_iq__etags_dn4 = assign12840_e13459_d_n4;
        locals.var_fn133_calc_iq__etags_dn5 = assign12840_e13459_d_n5;
        locals.var_fn133_calc_iq__etags_dn7 = assign12840_e13459_d_n7;

        let assign12850_e13462: f64 = if locals.var_fn133_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard165 = assign12850_e13462;

        let (assign12860_e13470, assign12860_e13470_d_n2, assign12860_e13470_d_n3, assign12860_e13470_d_n4, assign12860_e13470_d_n5, assign12860_e13470_d_n7, assign12860_e13470_d_n14,) = {
    if (((locals.var_guard132 != 0.0) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 != 0.0)) {
        (locals.var_fn133_calc_iq__etags, locals.var_fn133_calc_iq__etags_dn2, 0.0, locals.var_fn133_calc_iq__etags_dn4, locals.var_fn133_calc_iq__etags_dn5, locals.var_fn133_calc_iq__etags_dn7, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12860_e13470;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12860_e13470_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12860_e13470_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12860_e13470_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12860_e13470_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12860_e13470_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12860_e13470_d_n14;

        let assign12870_e13473: f64 = (-50.0);
        let assign12870_e13474: f64 = if locals.var_fn133_calc_iq__etags < assign12870_e13473 { 1.0 } else { 0.0 };
        locals.var_guard166 = assign12870_e13474;

        let (assign12880_e13486, assign12880_e13486_d_n2, assign12880_e13486_d_n3, assign12880_e13486_d_n4, assign12880_e13486_d_n5, assign12880_e13486_d_n7, assign12880_e13486_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign12880_e13484: f64 = (locals.var_fn133_calc_iq__etags).exp();
        (assign12880_e13484, (assign12880_e13484 * locals.var_fn133_calc_iq__etags_dn2), 0.0, (assign12880_e13484 * locals.var_fn133_calc_iq__etags_dn4), (assign12880_e13484 * locals.var_fn133_calc_iq__etags_dn5), (assign12880_e13484 * locals.var_fn133_calc_iq__etags_dn7), 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12880_e13486;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12880_e13486_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12880_e13486_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12880_e13486_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12880_e13486_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12880_e13486_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12880_e13486_d_n14;

        let (assign12890_e13502, assign12890_e13502_d_n2, assign12890_e13502_d_n3, assign12890_e13502_d_n4, assign12890_e13502_d_n5, assign12890_e13502_d_n7, assign12890_e13502_d_n14,) = {
    if ((((locals.var_guard132 != 0.0) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 == 0.0)) && (locals.var_guard166 == 0.0)) {
        let assign12890_e13498: f64 = (locals.var_fn133_calc_iq__etags).exp();
        let assign12890_e13499: f64 = (1.0 + assign12890_e13498);
        let assign12890_e13500: f64 = (assign12890_e13499).ln();
        (assign12890_e13500, ((assign12890_e13498 * locals.var_fn133_calc_iq__etags_dn2) / assign12890_e13499), 0.0, ((assign12890_e13498 * locals.var_fn133_calc_iq__etags_dn4) / assign12890_e13499), ((assign12890_e13498 * locals.var_fn133_calc_iq__etags_dn5) / assign12890_e13499), ((assign12890_e13498 * locals.var_fn133_calc_iq__etags_dn7) / assign12890_e13499), 0.0,)
    } else {
        (locals.var_fn133_calc_iq__exparg, locals.var_fn133_calc_iq__exparg_dn2, locals.var_fn133_calc_iq__exparg_dn3, locals.var_fn133_calc_iq__exparg_dn4, locals.var_fn133_calc_iq__exparg_dn5, locals.var_fn133_calc_iq__exparg_dn7, locals.var_fn133_calc_iq__exparg_dn14,)
    }
};
        locals.var_fn133_calc_iq__exparg = assign12890_e13502;
        locals.var_fn133_calc_iq__exparg_dn2 = assign12890_e13502_d_n2;
        locals.var_fn133_calc_iq__exparg_dn3 = assign12890_e13502_d_n3;
        locals.var_fn133_calc_iq__exparg_dn4 = assign12890_e13502_d_n4;
        locals.var_fn133_calc_iq__exparg_dn5 = assign12890_e13502_d_n5;
        locals.var_fn133_calc_iq__exparg_dn7 = assign12890_e13502_d_n7;
        locals.var_fn133_calc_iq__exparg_dn14 = assign12890_e13502_d_n14;

        let (assign12900_e13520, assign12900_e13520_d_n2, assign12900_e13520_d_n3, assign12900_e13520_d_n4, assign12900_e13520_d_n5, assign12900_e13520_d_n7, assign12900_e13520_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard164 != 0.0)) {
        let assign12900_e13508: f64 = (locals.var_fn133_calc_iq__w * locals.var_fn133_calc_iq__ngf);
        let assign12900_e13510: f64 = (assign12900_e13508 * locals.var_fn133_calc_iq__type);
        let assign12900_e13512: f64 = (assign12900_e13510 * locals.var_fn133_calc_iq__cs);
        let assign12900_e13514: f64 = (assign12900_e13512 * locals.var_fn133_calc_iq__two_n_phit0);
        let assign12900_e13516: f64 = (assign12900_e13514 * locals.var_fn133_calc_iq__exparg);
        let assign12900_e13518: f64 = (assign12900_e13516 * locals.var_fn133_calc_iq__trapfracdl);
        (assign12900_e13518, ((assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn2) * locals.var_fn133_calc_iq__trapfracdl), ((assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn3) * locals.var_fn133_calc_iq__trapfracdl), ((((assign12900_e13512 * locals.var_fn133_calc_iq__two_n_phit0_dn4) * locals.var_fn133_calc_iq__exparg) + (assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn4)) * locals.var_fn133_calc_iq__trapfracdl), ((assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn5) * locals.var_fn133_calc_iq__trapfracdl), ((assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn7) * locals.var_fn133_calc_iq__trapfracdl), ((assign12900_e13514 * locals.var_fn133_calc_iq__exparg_dn14) * locals.var_fn133_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn133_calc_iq__qsout, locals.var_fn133_calc_iq__qsout_dn2, locals.var_fn133_calc_iq__qsout_dn3, locals.var_fn133_calc_iq__qsout_dn4, locals.var_fn133_calc_iq__qsout_dn5, locals.var_fn133_calc_iq__qsout_dn7, locals.var_fn133_calc_iq__qsout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qsout = assign12900_e13520;
        locals.var_fn133_calc_iq__qsout_dn2 = assign12900_e13520_d_n2;
        locals.var_fn133_calc_iq__qsout_dn3 = assign12900_e13520_d_n3;
        locals.var_fn133_calc_iq__qsout_dn4 = assign12900_e13520_d_n4;
        locals.var_fn133_calc_iq__qsout_dn5 = assign12900_e13520_d_n5;
        locals.var_fn133_calc_iq__qsout_dn7 = assign12900_e13520_d_n7;
        locals.var_fn133_calc_iq__qsout_dn14 = assign12900_e13520_d_n14;

    }
}
