#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    pub(super) fn stamp_reactive_block_155(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14, ) = (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14, );
            locals.var_dnm_rv = 0.0;
        }
        let assign105140_e157521: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2384 = assign105140_e157521;
        locals.var_guard2384_rv = 0.0;
        let assign105150_e157524: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2385 = assign105150_e157524;
        locals.var_guard2385_rv = 0.0;
        if (((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) && (locals.var_guard2384 != 0.0)) && (locals.var_guard2385 != 0.0)) {
            locals.var_mm = 1.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105170_e157540: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2386 = assign105170_e157540;
        locals.var_guard2386_rv = 0.0;
        if ((((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) && (locals.var_guard2384 != 0.0)) && (locals.var_guard2385 == 0.0)) && (locals.var_guard2386 != 0.0)) {
            locals.var_mm = 2.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105190_e157559: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2387 = assign105190_e157559;
        locals.var_guard2387_rv = 0.0;
        if (((((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) && (locals.var_guard2384 != 0.0)) && (locals.var_guard2385 == 0.0)) && (locals.var_guard2386 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            locals.var_mm = 3.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105210_e157581: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2388 = assign105210_e157581;
        locals.var_guard2388_rv = 0.0;
        if ((((((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) && (locals.var_guard2384 != 0.0)) && (locals.var_guard2385 == 0.0)) && (locals.var_guard2386 == 0.0)) && (locals.var_guard2387 == 0.0)) && (locals.var_guard2388 != 0.0)) {
            locals.var_mm = 4.0;
            locals.var_mm_rv = 0.0;
        }
        if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) && (locals.var_guard2384 != 0.0)) {
            locals.var_m0 = 0.0;
            locals.var_m0_rv = 0.0;
        }
        let mut assign105240_loop_guard: usize = 0;
        while {
            let assign105240_cond_e157626: f64 = if (((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) && (locals.var_guard2384 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105240_cond_e157626 != 0.0
        } {
            assign105240_loop_guard += 1;
            assert!(assign105240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) && (locals.var_guard2384 != 0.0)) {
                let assign105240_body0_e157636: f64 = (locals.var_dnm).sqrt();
                (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14, ) = (assign105240_body0_e157636, (locals.var_dnm_dn0 / (2.0 * assign105240_body0_e157636)), (locals.var_dnm_dn2 / (2.0 * assign105240_body0_e157636)), (locals.var_dnm_dn4 / (2.0 * assign105240_body0_e157636)), (locals.var_dnm_dn5 / (2.0 * assign105240_body0_e157636)), (locals.var_dnm_dn6 / (2.0 * assign105240_body0_e157636)), (locals.var_dnm_dn7 / (2.0 * assign105240_body0_e157636)), (locals.var_dnm_dn8 / (2.0 * assign105240_body0_e157636)), (locals.var_dnm_dn9 / (2.0 * assign105240_body0_e157636)), (locals.var_dnm_dn10 / (2.0 * assign105240_body0_e157636)), (locals.var_dnm_dn11 / (2.0 * assign105240_body0_e157636)), (locals.var_dnm_dn14 / (2.0 * assign105240_body0_e157636)), );
                locals.var_dnm_rv = 0.0;
            }
            if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) && (locals.var_guard2384 != 0.0)) {
                let assign105240_body1_e157649: f64 = (locals.var_m0 + 1.0);
                locals.var_m0 = assign105240_body1_e157649;
                locals.var_m0_rv = 0.0;
            }
        }
        if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) && (locals.var_guard2384 == 0.0)) {
            let (assign105250_e157672, assign105250_e157672_d_n0, assign105250_e157672_d_n2, assign105250_e157672_d_n4, assign105250_e157672_d_n5, assign105250_e157672_d_n6, assign105250_e157672_d_n7, assign105250_e157672_d_n8, assign105250_e157672_d_n9, assign105250_e157672_d_n10, assign105250_e157672_d_n11, assign105250_e157672_d_n14,) = {
    if (locals.var_dnm == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign105250_e157669: f64 = (2.0 * 2.0);
        let assign105250_e157670: f64 = (1.0 / assign105250_e157669);
        let assign105250_e157671: f64 = (locals.var_dnm).powf(assign105250_e157670);
        (assign105250_e157671, if 0.0 == 0.0 && ((assign105250_e157670) as f64).is_finite() && ((assign105250_e157670) as f64).fract() == 0.0 { if assign105250_e157670 == 0.0 { 0.0 } else { (assign105250_e157670 * ((locals.var_dnm).powf(assign105250_e157670 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105250_e157671 * (assign105250_e157670 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105250_e157670) as f64).is_finite() && ((assign105250_e157670) as f64).fract() == 0.0 { if assign105250_e157670 == 0.0 { 0.0 } else { (assign105250_e157670 * ((locals.var_dnm).powf(assign105250_e157670 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105250_e157671 * (assign105250_e157670 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105250_e157670) as f64).is_finite() && ((assign105250_e157670) as f64).fract() == 0.0 { if assign105250_e157670 == 0.0 { 0.0 } else { (assign105250_e157670 * ((locals.var_dnm).powf(assign105250_e157670 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105250_e157671 * (assign105250_e157670 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105250_e157670) as f64).is_finite() && ((assign105250_e157670) as f64).fract() == 0.0 { if assign105250_e157670 == 0.0 { 0.0 } else { (assign105250_e157670 * ((locals.var_dnm).powf(assign105250_e157670 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105250_e157671 * (assign105250_e157670 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105250_e157670) as f64).is_finite() && ((assign105250_e157670) as f64).fract() == 0.0 { if assign105250_e157670 == 0.0 { 0.0 } else { (assign105250_e157670 * ((locals.var_dnm).powf(assign105250_e157670 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105250_e157671 * (assign105250_e157670 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105250_e157670) as f64).is_finite() && ((assign105250_e157670) as f64).fract() == 0.0 { if assign105250_e157670 == 0.0 { 0.0 } else { (assign105250_e157670 * ((locals.var_dnm).powf(assign105250_e157670 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105250_e157671 * (assign105250_e157670 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105250_e157670) as f64).is_finite() && ((assign105250_e157670) as f64).fract() == 0.0 { if assign105250_e157670 == 0.0 { 0.0 } else { (assign105250_e157670 * ((locals.var_dnm).powf(assign105250_e157670 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105250_e157671 * (assign105250_e157670 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105250_e157670) as f64).is_finite() && ((assign105250_e157670) as f64).fract() == 0.0 { if assign105250_e157670 == 0.0 { 0.0 } else { (assign105250_e157670 * ((locals.var_dnm).powf(assign105250_e157670 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105250_e157671 * (assign105250_e157670 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105250_e157670) as f64).is_finite() && ((assign105250_e157670) as f64).fract() == 0.0 { if assign105250_e157670 == 0.0 { 0.0 } else { (assign105250_e157670 * ((locals.var_dnm).powf(assign105250_e157670 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105250_e157671 * (assign105250_e157670 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105250_e157670) as f64).is_finite() && ((assign105250_e157670) as f64).fract() == 0.0 { if assign105250_e157670 == 0.0 { 0.0 } else { (assign105250_e157670 * ((locals.var_dnm).powf(assign105250_e157670 - 1.0) * locals.var_dnm_dn11)) } } else { (assign105250_e157671 * (assign105250_e157670 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105250_e157670) as f64).is_finite() && ((assign105250_e157670) as f64).fract() == 0.0 { if assign105250_e157670 == 0.0 { 0.0 } else { (assign105250_e157670 * ((locals.var_dnm).powf(assign105250_e157670 - 1.0) * locals.var_dnm_dn14)) } } else { (assign105250_e157671 * (assign105250_e157670 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
    }
};
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14, ) = (assign105250_e157672, assign105250_e157672_d_n0, assign105250_e157672_d_n2, assign105250_e157672_d_n4, assign105250_e157672_d_n5, assign105250_e157672_d_n6, assign105250_e157672_d_n7, assign105250_e157672_d_n8, assign105250_e157672_d_n9, assign105250_e157672_d_n10, assign105250_e157672_d_n11, assign105250_e157672_d_n14, );
            locals.var_dnm_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
            let assign105260_e157683: f64 = (1.0 / locals.var_dnm);
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14, ) = (assign105260_e157683, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))), );
            locals.var_dnm_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
            let assign105270_e157694: f64 = (locals.var_tmf1 * 1e-25);
            let assign105270_e157696: f64 = (assign105270_e157694 * locals.var_dnm);
            (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14, ) = (assign105270_e157696, (((locals.var_tmf1_dn0 * 1e-25) * locals.var_dnm) + (assign105270_e157694 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-25) * locals.var_dnm) + (assign105270_e157694 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-25) * locals.var_dnm) + (assign105270_e157694 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-25) * locals.var_dnm) + (assign105270_e157694 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-25) * locals.var_dnm) + (assign105270_e157694 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-25) * locals.var_dnm) + (assign105270_e157694 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-25) * locals.var_dnm) + (assign105270_e157694 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-25) * locals.var_dnm) + (assign105270_e157694 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-25) * locals.var_dnm) + (assign105270_e157694 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-25) * locals.var_dnm) + (assign105270_e157694 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-25) * locals.var_dnm) + (assign105270_e157694 * locals.var_dnm_dn14)), );
            locals.var_tmf0_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
            let assign105280_e157707: f64 = (1e-25 * locals.var_xmp);
            let assign105280_e157709: f64 = (assign105280_e157707 * locals.var_dnm);
            let assign105280_e157711: f64 = (assign105280_e157709 / locals.var_arg);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14, ) = (assign105280_e157711, ((((((1e-25 * locals.var_xmp_dn0) * locals.var_dnm) + (assign105280_e157707 * locals.var_dnm_dn0)) * locals.var_arg) - (assign105280_e157709 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn2) * locals.var_dnm) + (assign105280_e157707 * locals.var_dnm_dn2)) * locals.var_arg) - (assign105280_e157709 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn4) * locals.var_dnm) + (assign105280_e157707 * locals.var_dnm_dn4)) * locals.var_arg) - (assign105280_e157709 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn5) * locals.var_dnm) + (assign105280_e157707 * locals.var_dnm_dn5)) * locals.var_arg) - (assign105280_e157709 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn6) * locals.var_dnm) + (assign105280_e157707 * locals.var_dnm_dn6)) * locals.var_arg) - (assign105280_e157709 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn7) * locals.var_dnm) + (assign105280_e157707 * locals.var_dnm_dn7)) * locals.var_arg) - (assign105280_e157709 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn8) * locals.var_dnm) + (assign105280_e157707 * locals.var_dnm_dn8)) * locals.var_arg) - (assign105280_e157709 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn9) * locals.var_dnm) + (assign105280_e157707 * locals.var_dnm_dn9)) * locals.var_arg) - (assign105280_e157709 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn10) * locals.var_dnm) + (assign105280_e157707 * locals.var_dnm_dn10)) * locals.var_arg) - (assign105280_e157709 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn11) * locals.var_dnm) + (assign105280_e157707 * locals.var_dnm_dn11)) * locals.var_arg) - (assign105280_e157709 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn14) * locals.var_dnm) + (assign105280_e157707 * locals.var_dnm_dn14)) * locals.var_arg) - (assign105280_e157709 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)), );
            locals.var_t0_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
            let assign105290_e157722: f64 = 1e-25;
            let assign105290_e157724: f64 = (assign105290_e157722 - locals.var_tmf0);
            (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn14, ) = (assign105290_e157724, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14), );
            locals.var_gd_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14, ) = (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14, );
            locals.var_t0_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 == 0.0)) {
            (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn14, ) = (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn14, );
            locals.var_gd_rv = 0.0;
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
            let assign105330_e157762: f64 = (1.0 / locals.var_gd);
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, ) = (assign105330_e157762, (-(locals.var_gd_dn0 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn2 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn4 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn5 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn6 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn7 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn8 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn9 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn10 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn11 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn14 / (locals.var_gd * locals.var_gd))), );
            locals.var_rdd_rv = 0.0;
        }
        if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
            let assign105340_e157771: f64 = (locals.var_rdd / locals.var_weffld_nf);
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, ) = (assign105340_e157771, (locals.var_rdd_dn0 / locals.var_weffld_nf), (locals.var_rdd_dn2 / locals.var_weffld_nf), (locals.var_rdd_dn4 / locals.var_weffld_nf), (locals.var_rdd_dn5 / locals.var_weffld_nf), (locals.var_rdd_dn6 / locals.var_weffld_nf), (locals.var_rdd_dn7 / locals.var_weffld_nf), (locals.var_rdd_dn8 / locals.var_weffld_nf), (locals.var_rdd_dn9 / locals.var_weffld_nf), (locals.var_rdd_dn10 / locals.var_weffld_nf), (locals.var_rdd_dn11 / locals.var_weffld_nf), (locals.var_rdd_dn14 / locals.var_weffld_nf), );
            locals.var_rdd_rv = 0.0;
        }
        let assign105350_e157777: f64 = (1000000.0 - 1000.0);
        let assign105350_e157782: f64 = if ((locals.var_rdd > assign105350_e157777) && (1000.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2389 = assign105350_e157782;
        locals.var_guard2389_rv = 0.0;
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105360_e157791: f64 = (locals.var_rdd - 1000000.0);
            let assign105360_e157793: f64 = (assign105360_e157791 + 1000.0);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14, ) = (assign105360_e157793, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, );
            locals.var_tmf1_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105370_e157804: f64 = (locals.var_tmf1 * locals.var_tmf1);
            (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14, ) = (assign105370_e157804, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)), );
            locals.var_x2_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105380_e157815: f64 = (1000.0 * 1000.0);
            (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14, ) = (assign105380_e157815, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_xmax2_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_xp_rv = 0.0;
            (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_xmp_rv = 0.0;
            locals.var_m0 = 0.0;
            locals.var_m0_rv = 0.0;
            locals.var_mm = 0.0;
            locals.var_mm_rv = 0.0;
            (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_arg_rv = 0.0;
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_dnm_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105450_e157880: f64 = (locals.var_xp * locals.var_x2);
            (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14, ) = (assign105450_e157880, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)), );
            locals.var_xp_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105460_e157891: f64 = (locals.var_xmp * locals.var_xmax2);
            (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14, ) = (assign105460_e157891, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)), );
            locals.var_xmp_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105470_e157902: f64 = (locals.var_xp * locals.var_x2);
            (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14, ) = (assign105470_e157902, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)), );
            locals.var_xp_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105480_e157913: f64 = (locals.var_xmp * locals.var_xmax2);
            (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14, ) = (assign105480_e157913, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)), );
            locals.var_xmp_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105490_e157924: f64 = (locals.var_xp + locals.var_xmp);
            (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14, ) = (assign105490_e157924, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14), );
            locals.var_arg_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14, ) = (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14, );
            locals.var_dnm_rv = 0.0;
        }
        let assign105510_e157950: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2390 = assign105510_e157950;
        locals.var_guard2390_rv = 0.0;
        let assign105520_e157953: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2391 = assign105520_e157953;
        locals.var_guard2391_rv = 0.0;
        if (((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) && (locals.var_guard2390 != 0.0)) && (locals.var_guard2391 != 0.0)) {
            locals.var_mm = 1.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105540_e157969: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2392 = assign105540_e157969;
        locals.var_guard2392_rv = 0.0;
        if ((((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) && (locals.var_guard2390 != 0.0)) && (locals.var_guard2391 == 0.0)) && (locals.var_guard2392 != 0.0)) {
            locals.var_mm = 2.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105560_e157988: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2393 = assign105560_e157988;
        locals.var_guard2393_rv = 0.0;
        if (((((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) && (locals.var_guard2390 != 0.0)) && (locals.var_guard2391 == 0.0)) && (locals.var_guard2392 == 0.0)) && (locals.var_guard2393 != 0.0)) {
            locals.var_mm = 3.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105580_e158010: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2394 = assign105580_e158010;
        locals.var_guard2394_rv = 0.0;
        if ((((((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) && (locals.var_guard2390 != 0.0)) && (locals.var_guard2391 == 0.0)) && (locals.var_guard2392 == 0.0)) && (locals.var_guard2393 == 0.0)) && (locals.var_guard2394 != 0.0)) {
            locals.var_mm = 4.0;
            locals.var_mm_rv = 0.0;
        }
        if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) && (locals.var_guard2390 != 0.0)) {
            locals.var_m0 = 0.0;
            locals.var_m0_rv = 0.0;
        }
        let mut assign105610_loop_guard: usize = 0;
        while {
            let assign105610_cond_e158055: f64 = if (((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) && (locals.var_guard2390 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105610_cond_e158055 != 0.0
        } {
            assign105610_loop_guard += 1;
            assert!(assign105610_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) && (locals.var_guard2390 != 0.0)) {
                let assign105610_body0_e158065: f64 = (locals.var_dnm).sqrt();
                (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14, ) = (assign105610_body0_e158065, (locals.var_dnm_dn0 / (2.0 * assign105610_body0_e158065)), (locals.var_dnm_dn2 / (2.0 * assign105610_body0_e158065)), (locals.var_dnm_dn4 / (2.0 * assign105610_body0_e158065)), (locals.var_dnm_dn5 / (2.0 * assign105610_body0_e158065)), (locals.var_dnm_dn6 / (2.0 * assign105610_body0_e158065)), (locals.var_dnm_dn7 / (2.0 * assign105610_body0_e158065)), (locals.var_dnm_dn8 / (2.0 * assign105610_body0_e158065)), (locals.var_dnm_dn9 / (2.0 * assign105610_body0_e158065)), (locals.var_dnm_dn10 / (2.0 * assign105610_body0_e158065)), (locals.var_dnm_dn11 / (2.0 * assign105610_body0_e158065)), (locals.var_dnm_dn14 / (2.0 * assign105610_body0_e158065)), );
                locals.var_dnm_rv = 0.0;
            }
            if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) && (locals.var_guard2390 != 0.0)) {
                let assign105610_body1_e158078: f64 = (locals.var_m0 + 1.0);
                locals.var_m0 = assign105610_body1_e158078;
                locals.var_m0_rv = 0.0;
            }
        }
        if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) && (locals.var_guard2390 == 0.0)) {
            let (assign105620_e158101, assign105620_e158101_d_n0, assign105620_e158101_d_n2, assign105620_e158101_d_n4, assign105620_e158101_d_n5, assign105620_e158101_d_n6, assign105620_e158101_d_n7, assign105620_e158101_d_n8, assign105620_e158101_d_n9, assign105620_e158101_d_n10, assign105620_e158101_d_n11, assign105620_e158101_d_n14,) = {
    if (locals.var_dnm == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign105620_e158098: f64 = (2.0 * 2.0);
        let assign105620_e158099: f64 = (1.0 / assign105620_e158098);
        let assign105620_e158100: f64 = (locals.var_dnm).powf(assign105620_e158099);
        (assign105620_e158100, if 0.0 == 0.0 && ((assign105620_e158099) as f64).is_finite() && ((assign105620_e158099) as f64).fract() == 0.0 { if assign105620_e158099 == 0.0 { 0.0 } else { (assign105620_e158099 * ((locals.var_dnm).powf(assign105620_e158099 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105620_e158100 * (assign105620_e158099 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105620_e158099) as f64).is_finite() && ((assign105620_e158099) as f64).fract() == 0.0 { if assign105620_e158099 == 0.0 { 0.0 } else { (assign105620_e158099 * ((locals.var_dnm).powf(assign105620_e158099 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105620_e158100 * (assign105620_e158099 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105620_e158099) as f64).is_finite() && ((assign105620_e158099) as f64).fract() == 0.0 { if assign105620_e158099 == 0.0 { 0.0 } else { (assign105620_e158099 * ((locals.var_dnm).powf(assign105620_e158099 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105620_e158100 * (assign105620_e158099 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105620_e158099) as f64).is_finite() && ((assign105620_e158099) as f64).fract() == 0.0 { if assign105620_e158099 == 0.0 { 0.0 } else { (assign105620_e158099 * ((locals.var_dnm).powf(assign105620_e158099 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105620_e158100 * (assign105620_e158099 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105620_e158099) as f64).is_finite() && ((assign105620_e158099) as f64).fract() == 0.0 { if assign105620_e158099 == 0.0 { 0.0 } else { (assign105620_e158099 * ((locals.var_dnm).powf(assign105620_e158099 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105620_e158100 * (assign105620_e158099 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105620_e158099) as f64).is_finite() && ((assign105620_e158099) as f64).fract() == 0.0 { if assign105620_e158099 == 0.0 { 0.0 } else { (assign105620_e158099 * ((locals.var_dnm).powf(assign105620_e158099 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105620_e158100 * (assign105620_e158099 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105620_e158099) as f64).is_finite() && ((assign105620_e158099) as f64).fract() == 0.0 { if assign105620_e158099 == 0.0 { 0.0 } else { (assign105620_e158099 * ((locals.var_dnm).powf(assign105620_e158099 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105620_e158100 * (assign105620_e158099 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105620_e158099) as f64).is_finite() && ((assign105620_e158099) as f64).fract() == 0.0 { if assign105620_e158099 == 0.0 { 0.0 } else { (assign105620_e158099 * ((locals.var_dnm).powf(assign105620_e158099 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105620_e158100 * (assign105620_e158099 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105620_e158099) as f64).is_finite() && ((assign105620_e158099) as f64).fract() == 0.0 { if assign105620_e158099 == 0.0 { 0.0 } else { (assign105620_e158099 * ((locals.var_dnm).powf(assign105620_e158099 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105620_e158100 * (assign105620_e158099 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105620_e158099) as f64).is_finite() && ((assign105620_e158099) as f64).fract() == 0.0 { if assign105620_e158099 == 0.0 { 0.0 } else { (assign105620_e158099 * ((locals.var_dnm).powf(assign105620_e158099 - 1.0) * locals.var_dnm_dn11)) } } else { (assign105620_e158100 * (assign105620_e158099 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105620_e158099) as f64).is_finite() && ((assign105620_e158099) as f64).fract() == 0.0 { if assign105620_e158099 == 0.0 { 0.0 } else { (assign105620_e158099 * ((locals.var_dnm).powf(assign105620_e158099 - 1.0) * locals.var_dnm_dn14)) } } else { (assign105620_e158100 * (assign105620_e158099 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
    }
};
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14, ) = (assign105620_e158101, assign105620_e158101_d_n0, assign105620_e158101_d_n2, assign105620_e158101_d_n4, assign105620_e158101_d_n5, assign105620_e158101_d_n6, assign105620_e158101_d_n7, assign105620_e158101_d_n8, assign105620_e158101_d_n9, assign105620_e158101_d_n10, assign105620_e158101_d_n11, assign105620_e158101_d_n14, );
            locals.var_dnm_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105630_e158112: f64 = (1.0 / locals.var_dnm);
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14, ) = (assign105630_e158112, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))), );
            locals.var_dnm_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105640_e158123: f64 = (locals.var_tmf1 * 1000.0);
            let assign105640_e158125: f64 = (assign105640_e158123 * locals.var_dnm);
            (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14, ) = (assign105640_e158125, (((locals.var_tmf1_dn0 * 1000.0) * locals.var_dnm) + (assign105640_e158123 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1000.0) * locals.var_dnm) + (assign105640_e158123 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1000.0) * locals.var_dnm) + (assign105640_e158123 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1000.0) * locals.var_dnm) + (assign105640_e158123 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1000.0) * locals.var_dnm) + (assign105640_e158123 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1000.0) * locals.var_dnm) + (assign105640_e158123 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1000.0) * locals.var_dnm) + (assign105640_e158123 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1000.0) * locals.var_dnm) + (assign105640_e158123 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1000.0) * locals.var_dnm) + (assign105640_e158123 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1000.0) * locals.var_dnm) + (assign105640_e158123 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1000.0) * locals.var_dnm) + (assign105640_e158123 * locals.var_dnm_dn14)), );
            locals.var_tmf0_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105650_e158136: f64 = (1000.0 * locals.var_xmp);
            let assign105650_e158138: f64 = (assign105650_e158136 * locals.var_dnm);
            let assign105650_e158140: f64 = (assign105650_e158138 / locals.var_arg);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14, ) = (assign105650_e158140, ((((((1000.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign105650_e158136 * locals.var_dnm_dn0)) * locals.var_arg) - (assign105650_e158138 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign105650_e158136 * locals.var_dnm_dn2)) * locals.var_arg) - (assign105650_e158138 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign105650_e158136 * locals.var_dnm_dn4)) * locals.var_arg) - (assign105650_e158138 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign105650_e158136 * locals.var_dnm_dn5)) * locals.var_arg) - (assign105650_e158138 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign105650_e158136 * locals.var_dnm_dn6)) * locals.var_arg) - (assign105650_e158138 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign105650_e158136 * locals.var_dnm_dn7)) * locals.var_arg) - (assign105650_e158138 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign105650_e158136 * locals.var_dnm_dn8)) * locals.var_arg) - (assign105650_e158138 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign105650_e158136 * locals.var_dnm_dn9)) * locals.var_arg) - (assign105650_e158138 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign105650_e158136 * locals.var_dnm_dn10)) * locals.var_arg) - (assign105650_e158138 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn11) * locals.var_dnm) + (assign105650_e158136 * locals.var_dnm_dn11)) * locals.var_arg) - (assign105650_e158138 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn14) * locals.var_dnm) + (assign105650_e158136 * locals.var_dnm_dn14)) * locals.var_arg) - (assign105650_e158138 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)), );
            locals.var_t0_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            let assign105660_e158151: f64 = (1000000.0 - 1000.0);
            let assign105660_e158153: f64 = (assign105660_e158151 + locals.var_tmf0);
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, ) = (assign105660_e158153, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14, );
            locals.var_rdd_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 != 0.0)) {
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14, ) = (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14, );
            locals.var_t0_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2389 == 0.0)) {
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, ) = (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, );
            locals.var_rdd_rv = 0.0;
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        let assign105700_e158191: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign105700_e158192: f64 = (locals.var_uc_nover * assign105700_e158191);
        let assign105700_e158195: f64 = if ((p.p54 == 1.0) && (assign105700_e158192 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2395 = assign105700_e158195;
        locals.var_guard2395_rv = 0.0;
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2395 != 0.0)) {
            let assign105710_e158204: f64 = (p.p334 - locals.var_wdep);
            (locals.var_ddriftld, locals.var_ddriftld_dn0, locals.var_ddriftld_dn2, locals.var_ddriftld_dn4, locals.var_ddriftld_dn5, locals.var_ddriftld_dn6, locals.var_ddriftld_dn7, locals.var_ddriftld_dn8, locals.var_ddriftld_dn9, locals.var_ddriftld_dn10, locals.var_ddriftld_dn11, locals.var_ddriftld_dn14, ) = (assign105710_e158204, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn11), (-locals.var_wdep_dn14), );
            locals.var_ddriftld_rv = 0.0;
        }
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2395 != 0.0)) {
            let assign105720_e158215: f64 = (locals.var_rdd * locals.var_ldrift0);
            let assign105720_e158217: f64 = (assign105720_e158215 / locals.var_ddriftld);
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, ) = (assign105720_e158217, ((((locals.var_rdd_dn0 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105720_e158215 * locals.var_ddriftld_dn0)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn2 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105720_e158215 * locals.var_ddriftld_dn2)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn4 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105720_e158215 * locals.var_ddriftld_dn4)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn5 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105720_e158215 * locals.var_ddriftld_dn5)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn6 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105720_e158215 * locals.var_ddriftld_dn6)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn7 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105720_e158215 * locals.var_ddriftld_dn7)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn8 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105720_e158215 * locals.var_ddriftld_dn8)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn9 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105720_e158215 * locals.var_ddriftld_dn9)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn10 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105720_e158215 * locals.var_ddriftld_dn10)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn11 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105720_e158215 * locals.var_ddriftld_dn11)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn14 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105720_e158215 * locals.var_ddriftld_dn14)) / (locals.var_ddriftld * locals.var_ddriftld)), );
            locals.var_rdd_rv = 0.0;
        }
        if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
            let assign105730_e158226: f64 = (locals.var_rdd + locals.var_rd0);
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, ) = (assign105730_e158226, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, );
            locals.var_rdd_rv = 0.0;
        }
        let assign105770_e158259: f64 = if locals.var_rdd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2397 = assign105770_e158259;
        locals.var_guard2397_rv = 0.0;
        if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2397 != 0.0)) {
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, ) = (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_rdd_rv = 0.0;
        }
        if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
            let assign105790_e158275: f64 = (locals.var_rdd / locals.var_mfactor);
            (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn11, locals.var_rdde_dn14, ) = (assign105790_e158275, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn11 / locals.var_mfactor), (locals.var_rdd_dn14 / locals.var_mfactor), );
            locals.var_rdde_rv = 0.0;
        }
        let assign105800_e158280: f64 = if locals.var_rdd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2398 = assign105800_e158280;
        locals.var_guard2398_rv = 0.0;
        if ((locals.var_guard2338 == 0.0) && (locals.var_guard2398 != 0.0)) {
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, ) = (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_rdd_rv = 0.0;
        }
        let assign105820_e158290: f64 = if locals.var_rsd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2399 = assign105820_e158290;
        locals.var_guard2399_rv = 0.0;
        if ((locals.var_guard2338 == 0.0) && (locals.var_guard2399 != 0.0)) {
            (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14, ) = (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_rsd_rv = 0.0;
        }
        let assign105840_e158300: f64 = if locals.var_vdsemodenml > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2400 = assign105840_e158300;
        locals.var_guard2400_rv = 0.0;
        if ((locals.var_guard2338 == 0.0) && (locals.var_guard2400 != 0.0)) {
            let assign105850_e158307: f64 = (locals.var_rdd / locals.var_mfactor);
            (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn11, locals.var_rdde_dn14, ) = (assign105850_e158307, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn11 / locals.var_mfactor), (locals.var_rdd_dn14 / locals.var_mfactor), );
            locals.var_rdde_rv = 0.0;
        }
        if ((locals.var_guard2338 == 0.0) && (locals.var_guard2400 != 0.0)) {
            let assign105860_e158316: f64 = (locals.var_rsd / locals.var_mfactor);
            (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn11, locals.var_rsde_dn14, ) = (assign105860_e158316, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn11 / locals.var_mfactor), (locals.var_rsd_dn14 / locals.var_mfactor), );
            locals.var_rsde_rv = 0.0;
        }
        if ((locals.var_guard2338 == 0.0) && (locals.var_guard2400 == 0.0)) {
            let assign105870_e158326: f64 = (locals.var_rsd / locals.var_mfactor);
            (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn11, locals.var_rdde_dn14, ) = (assign105870_e158326, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn11 / locals.var_mfactor), (locals.var_rsd_dn14 / locals.var_mfactor), );
            locals.var_rdde_rv = 0.0;
        }
        if ((locals.var_guard2338 == 0.0) && (locals.var_guard2400 == 0.0)) {
            let assign105880_e158336: f64 = (locals.var_rdd / locals.var_mfactor);
            (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn11, locals.var_rsde_dn14, ) = (assign105880_e158336, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn11 / locals.var_mfactor), (locals.var_rdd_dn14 / locals.var_mfactor), );
            locals.var_rsde_rv = 0.0;
        }
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14, ) = (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn11, locals.var_rdde_dn14, );
        locals.var_rdd_rv = 0.0;
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14, ) = (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn11, locals.var_rsde_dn14, );
        locals.var_rsd_rv = 0.0;
        let assign105940_e158346: f64 = if locals.var_mode > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2401 = assign105940_e158346;
        locals.var_guard2401_rv = 0.0;
        if (locals.var_guard2401 != 0.0) {
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14, ) = (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn8, locals.var_idse_dn9, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn14, );
            locals.var_ids_rv = 0.0;
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn14, ) = (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14, );
            locals.var_qd_rv = 0.0;
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn14, ) = (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14, );
            locals.var_qg_rv = 0.0;
            (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn11, locals.var_qs_dn14, ) = (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14, );
            locals.var_qs_rv = 0.0;
        }
        if (locals.var_guard2401 != 0.0) {
            let assign105990_e158366: f64 = (locals.var_qge + locals.var_qde);
            let assign105990_e158368: f64 = (assign105990_e158366 + locals.var_qse);
            let assign105990_e158369: f64 = (-assign105990_e158368);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn14, ) = (assign105990_e158369, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn9 + locals.var_qde_dn9) + locals.var_qse_dn9)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn14 + locals.var_qde_dn14) + locals.var_qse_dn14)), );
            locals.var_qb_rv = 0.0;
        }
        if (locals.var_guard2401 != 0.0) {
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn14, ) = (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn4, locals.var_isube_dn5, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn8, locals.var_isube_dn9, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn14, );
            locals.var_isub_rv = 0.0;
            (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn11, locals.var_isubld_dn14, ) = (locals.var_isublde, locals.var_isublde_dn0, locals.var_isublde_dn2, locals.var_isublde_dn4, locals.var_isublde_dn5, locals.var_isublde_dn6, locals.var_isublde_dn7, locals.var_isublde_dn8, locals.var_isublde_dn9, locals.var_isublde_dn10, locals.var_isublde_dn11, locals.var_isublde_dn14, );
            locals.var_isubld_rv = 0.0;
            (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn14, ) = (locals.var_idsibpce, locals.var_idsibpce_dn0, locals.var_idsibpce_dn2, locals.var_idsibpce_dn4, locals.var_idsibpce_dn5, locals.var_idsibpce_dn6, locals.var_idsibpce_dn7, locals.var_idsibpce_dn8, locals.var_idsibpce_dn9, locals.var_idsibpce_dn10, locals.var_idsibpce_dn11, locals.var_idsibpce_dn14, );
            locals.var_idsibpc_rv = 0.0;
        }
        if ((locals.var_guard2401 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14, ) = (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn14, );
            locals.var_qdrat_rv = 0.0;
        }
        if (locals.var_guard2401 == 0.0) {
            let assign106130_e158429: f64 = (-locals.var_idse);
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14, ) = (assign106130_e158429, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn4), (-locals.var_idse_dn5), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn8), (-locals.var_idse_dn9), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn14), );
            locals.var_ids_rv = 0.0;
        }
        if (locals.var_guard2401 == 0.0) {
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn14, ) = (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14, );
            locals.var_qd_rv = 0.0;
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn14, ) = (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14, );
            locals.var_qg_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_156(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard2401 == 0.0) {
            (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn11, locals.var_qs_dn14, ) = (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14, );
            locals.var_qs_rv = 0.0;
        }
        if (locals.var_guard2401 == 0.0) {
            let assign106170_e158451: f64 = (locals.var_qge + locals.var_qde);
            let assign106170_e158453: f64 = (assign106170_e158451 + locals.var_qse);
            let assign106170_e158454: f64 = (-assign106170_e158453);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn14, ) = (assign106170_e158454, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn9 + locals.var_qde_dn9) + locals.var_qse_dn9)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn14 + locals.var_qde_dn14) + locals.var_qse_dn14)), );
            locals.var_qb_rv = 0.0;
        }
        if (locals.var_guard2401 == 0.0) {
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isub_rv = 0.0;
            (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn11, locals.var_isubld_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isubld_rv = 0.0;
            (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_idsibpc_rv = 0.0;
        }
        if ((locals.var_guard2401 == 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign106300_e158523: f64 = (1.0 - locals.var_xd);
            (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14, ) = (assign106300_e158523, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn4), (-locals.var_xd_dn5), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn8), (-locals.var_xd_dn9), (-locals.var_xd_dn10), (-locals.var_xd_dn11), (-locals.var_xd_dn14), );
            locals.var_qdrat_rv = 0.0;
        }
        let assign106310_e158528: f64 = (locals.var_qg + locals.var_qgov);
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn14, ) = (assign106310_e158528, (locals.var_qg_dn0 + locals.var_qgov_dn0), (locals.var_qg_dn2 + locals.var_qgov_dn2), (locals.var_qg_dn4 + locals.var_qgov_dn4), (locals.var_qg_dn5 + locals.var_qgov_dn5), (locals.var_qg_dn6 + locals.var_qgov_dn6), (locals.var_qg_dn7 + locals.var_qgov_dn7), (locals.var_qg_dn8 + locals.var_qgov_dn8), (locals.var_qg_dn9 + locals.var_qgov_dn9), (locals.var_qg_dn10 + locals.var_qgov_dn10), (locals.var_qg_dn11 + locals.var_qgov_dn11), (locals.var_qg_dn14 + locals.var_qgov_dn14), );
        locals.var_qg_rv = 0.0;
        let assign106320_e158531: f64 = (locals.var_qd + locals.var_qdov);
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn14, ) = (assign106320_e158531, (locals.var_qd_dn0 + locals.var_qdov_dn0), (locals.var_qd_dn2 + locals.var_qdov_dn2), (locals.var_qd_dn4 + locals.var_qdov_dn4), (locals.var_qd_dn5 + locals.var_qdov_dn5), (locals.var_qd_dn6 + locals.var_qdov_dn6), (locals.var_qd_dn7 + locals.var_qdov_dn7), (locals.var_qd_dn8 + locals.var_qdov_dn8), (locals.var_qd_dn9 + locals.var_qdov_dn9), (locals.var_qd_dn10 + locals.var_qdov_dn10), (locals.var_qd_dn11 + locals.var_qdov_dn11), (locals.var_qd_dn14 + locals.var_qdov_dn14), );
        locals.var_qd_rv = 0.0;
        let assign106330_e158534: f64 = (locals.var_qs + locals.var_qsov);
        (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn11, locals.var_qs_dn14, ) = (assign106330_e158534, (locals.var_qs_dn0 + locals.var_qsov_dn0), (locals.var_qs_dn2 + locals.var_qsov_dn2), (locals.var_qs_dn4 + locals.var_qsov_dn4), (locals.var_qs_dn5 + locals.var_qsov_dn5), (locals.var_qs_dn6 + locals.var_qsov_dn6), (locals.var_qs_dn7 + locals.var_qsov_dn7), (locals.var_qs_dn8 + locals.var_qsov_dn8), (locals.var_qs_dn9 + locals.var_qsov_dn9), (locals.var_qs_dn10 + locals.var_qsov_dn10), (locals.var_qs_dn11 + locals.var_qsov_dn11), (locals.var_qs_dn14 + locals.var_qsov_dn14), );
        locals.var_qs_rv = 0.0;
        let assign106340_e158537: f64 = (locals.var_qg + locals.var_qd);
        let assign106340_e158539: f64 = (assign106340_e158537 + locals.var_qs);
        let assign106340_e158540: f64 = (-assign106340_e158539);
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn14, ) = (assign106340_e158540, (-((locals.var_qg_dn0 + locals.var_qd_dn0) + locals.var_qs_dn0)), (-((locals.var_qg_dn2 + locals.var_qd_dn2) + locals.var_qs_dn2)), (-((locals.var_qg_dn4 + locals.var_qd_dn4) + locals.var_qs_dn4)), (-((locals.var_qg_dn5 + locals.var_qd_dn5) + locals.var_qs_dn5)), (-((locals.var_qg_dn6 + locals.var_qd_dn6) + locals.var_qs_dn6)), (-((locals.var_qg_dn7 + locals.var_qd_dn7) + locals.var_qs_dn7)), (-((locals.var_qg_dn8 + locals.var_qd_dn8) + locals.var_qs_dn8)), (-((locals.var_qg_dn9 + locals.var_qd_dn9) + locals.var_qs_dn9)), (-((locals.var_qg_dn10 + locals.var_qd_dn10) + locals.var_qs_dn10)), (-((locals.var_qg_dn11 + locals.var_qd_dn11) + locals.var_qs_dn11)), (-((locals.var_qg_dn14 + locals.var_qd_dn14) + locals.var_qs_dn14)), );
        locals.var_qb_rv = 0.0;
        (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn7, ) = (locals.var_qdp, locals.var_qdp_dn0, locals.var_qdp_dn2, locals.var_qdp_dn7, );
        locals.var_qfd_rv = 0.0;
        (locals.var_qfs, locals.var_qfs_dn2, locals.var_qfs_dn7, ) = (locals.var_qsp, locals.var_qsp_dn2, locals.var_qsp_dn7, );
        locals.var_qfs_rv = 0.0;
        (locals.var_qdext, locals.var_qdext_dn0, locals.var_qdext_dn2, locals.var_qdext_dn4, locals.var_qdext_dn5, locals.var_qdext_dn6, locals.var_qdext_dn7, locals.var_qdext_dn8, locals.var_qdext_dn9, locals.var_qdext_dn10, locals.var_qdext_dn11, locals.var_qdext_dn14, ) = (locals.var_qdexte, locals.var_qdexte_dn0, locals.var_qdexte_dn2, locals.var_qdexte_dn4, locals.var_qdexte_dn5, locals.var_qdexte_dn6, locals.var_qdexte_dn7, locals.var_qdexte_dn8, locals.var_qdexte_dn9, locals.var_qdexte_dn10, locals.var_qdexte_dn11, locals.var_qdexte_dn14, );
        locals.var_qdext_rv = 0.0;
        (locals.var_qgext, locals.var_qgext_dn0, locals.var_qgext_dn2, locals.var_qgext_dn4, locals.var_qgext_dn5, locals.var_qgext_dn6, locals.var_qgext_dn7, locals.var_qgext_dn8, locals.var_qgext_dn9, locals.var_qgext_dn10, locals.var_qgext_dn11, locals.var_qgext_dn14, ) = (locals.var_qgexte, locals.var_qgexte_dn0, locals.var_qgexte_dn2, locals.var_qgexte_dn4, locals.var_qgexte_dn5, locals.var_qgexte_dn6, locals.var_qgexte_dn7, locals.var_qgexte_dn8, locals.var_qgexte_dn9, locals.var_qgexte_dn10, locals.var_qgexte_dn11, locals.var_qgexte_dn14, );
        locals.var_qgext_rv = 0.0;
        let assign106390_e158547: f64 = (locals.var_qgexte + locals.var_qdexte);
        let assign106390_e158549: f64 = (assign106390_e158547 + locals.var_qsexte);
        let assign106390_e158550: f64 = (-assign106390_e158549);
        (locals.var_qbext, locals.var_qbext_dn0, locals.var_qbext_dn2, locals.var_qbext_dn4, locals.var_qbext_dn5, locals.var_qbext_dn6, locals.var_qbext_dn7, locals.var_qbext_dn8, locals.var_qbext_dn9, locals.var_qbext_dn10, locals.var_qbext_dn11, locals.var_qbext_dn14, ) = (assign106390_e158550, (-((locals.var_qgexte_dn0 + locals.var_qdexte_dn0) + locals.var_qsexte_dn0)), (-((locals.var_qgexte_dn2 + locals.var_qdexte_dn2) + locals.var_qsexte_dn2)), (-((locals.var_qgexte_dn4 + locals.var_qdexte_dn4) + locals.var_qsexte_dn4)), (-((locals.var_qgexte_dn5 + locals.var_qdexte_dn5) + locals.var_qsexte_dn5)), (-((locals.var_qgexte_dn6 + locals.var_qdexte_dn6) + locals.var_qsexte_dn6)), (-((locals.var_qgexte_dn7 + locals.var_qdexte_dn7) + locals.var_qsexte_dn7)), (-((locals.var_qgexte_dn8 + locals.var_qdexte_dn8) + locals.var_qsexte_dn8)), (-((locals.var_qgexte_dn9 + locals.var_qdexte_dn9) + locals.var_qsexte_dn9)), (-((locals.var_qgexte_dn10 + locals.var_qdexte_dn10) + locals.var_qsexte_dn10)), (-((locals.var_qgexte_dn11 + locals.var_qdexte_dn11) + locals.var_qsexte_dn11)), (-((locals.var_qgexte_dn14 + locals.var_qdexte_dn14) + locals.var_qsexte_dn14)), );
        locals.var_qbext_rv = 0.0;
        let assign106400_e158553: f64 = if p.p53 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2402 = assign106400_e158553;
        locals.var_guard2402_rv = 0.0;
        let assign106410_e158556: f64 = if locals.var_rth > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard2403 = assign106410_e158556;
        locals.var_guard2403_rv = 0.0;
        if ((locals.var_guard2402 != 0.0) && (locals.var_guard2403 != 0.0)) {
            let assign106420_e158562: f64 = (1.0 / locals.var_rth);
            (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14, ) = (assign106420_e158562, (-(locals.var_rth_dn0 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn2 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn4 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn5 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn6 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn7 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn8 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn9 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn10 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn11 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn14 / (locals.var_rth * locals.var_rth))), );
            locals.var_gth_rv = 0.0;
        }
        if ((locals.var_guard2402 != 0.0) && (locals.var_guard2403 == 0.0)) {
            let assign106430_e158571: f64 = (1.0 / 0.0001);
            (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14, ) = (assign106430_e158571, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_gth_rv = 0.0;
        }
        let assign106440_e158577: f64 = (locals.var_vdsei - locals.var_vdsi);
        let assign106440_e158578: f64 = (locals.var_vdsi * assign106440_e158577);
        let assign106440_e158580: f64 = if assign106440_e158578 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2404 = assign106440_e158580;
        locals.var_guard2404_rv = 0.0;
        let assign106450_e158583: f64 = if locals.var_uc_powrat == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2405 = assign106450_e158583;
        locals.var_guard2405_rv = 0.0;
        if (((locals.var_guard2402 != 0.0) && (locals.var_guard2404 != 0.0)) && (locals.var_guard2405 != 0.0)) {
            (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14, ) = (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_veffpower_rv = 0.0;
        }
        if (((locals.var_guard2402 != 0.0) && (locals.var_guard2404 != 0.0)) && (locals.var_guard2405 == 0.0)) {
            let assign106470_e158602: f64 = (locals.var_vdsei - locals.var_vdsi);
            let assign106470_e158603: f64 = (locals.var_powratio * assign106470_e158602);
            let assign106470_e158604: f64 = (locals.var_vdsi + assign106470_e158603);
            (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14, ) = (assign106470_e158604, ((locals.var_powratio_dn0 * assign106470_e158602) + (locals.var_powratio * locals.var_vdsei_dn0)), ((locals.var_powratio_dn2 * assign106470_e158602) + (locals.var_powratio * locals.var_vdsei_dn2)), (locals.var_powratio_dn4 * assign106470_e158602), (locals.var_powratio_dn5 * assign106470_e158602), (locals.var_vdsi_dn6 + ((locals.var_powratio_dn6 * assign106470_e158602) + (locals.var_powratio * (-locals.var_vdsi_dn6)))), (locals.var_powratio_dn7 * assign106470_e158602), (locals.var_vdsi_dn8 + ((locals.var_powratio_dn8 * assign106470_e158602) + (locals.var_powratio * (-locals.var_vdsi_dn8)))), (locals.var_powratio_dn9 * assign106470_e158602), (locals.var_powratio_dn10 * assign106470_e158602), (locals.var_powratio_dn11 * assign106470_e158602), (locals.var_powratio_dn14 * assign106470_e158602), );
            locals.var_veffpower_rv = 0.0;
        }
        if ((locals.var_guard2402 != 0.0) && (locals.var_guard2404 == 0.0)) {
            (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14, ) = (locals.var_vdsi, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, locals.var_vdsi_dn8, 0.0, 0.0, 0.0, 0.0, );
            locals.var_veffpower_rv = 0.0;
        }
        if (locals.var_guard2402 != 0.0) {
            let assign106490_e158617: f64 = (locals.var_ids * locals.var_veffpower);
            (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14, ) = (assign106490_e158617, ((locals.var_ids_dn0 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn0)), ((locals.var_ids_dn2 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn2)), ((locals.var_ids_dn4 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn4)), ((locals.var_ids_dn5 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn5)), ((locals.var_ids_dn6 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn6)), ((locals.var_ids_dn7 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn7)), ((locals.var_ids_dn8 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn8)), ((locals.var_ids_dn9 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn9)), ((locals.var_ids_dn10 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn10)), ((locals.var_ids_dn11 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn11)), ((locals.var_ids_dn14 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn14)), );
            locals.var_p_rv = 0.0;
        }
        let assign106500_e158622: f64 = if p.p53 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2406 = assign106500_e158622;
        locals.var_guard2406_rv = 0.0;
        if ((locals.var_guard2402 != 0.0) && (locals.var_guard2406 != 0.0)) {
            let assign106510_e158628: f64 = (p.p433 * locals.var_gth);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14, ) = (assign106510_e158628, (p.p433 * locals.var_gth_dn0), (p.p433 * locals.var_gth_dn2), (p.p433 * locals.var_gth_dn4), (p.p433 * locals.var_gth_dn5), (p.p433 * locals.var_gth_dn6), (p.p433 * locals.var_gth_dn7), (p.p433 * locals.var_gth_dn8), (p.p433 * locals.var_gth_dn9), (p.p433 * locals.var_gth_dn10), (p.p433 * locals.var_gth_dn11), (p.p433 * locals.var_gth_dn14), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard2402 != 0.0) && (locals.var_guard2406 != 0.0)) {
            let assign106520_e158636: f64 = (locals.var_t1 - locals.var_p);
            let assign106520_e158639: f64 = (p.p337 * locals.var_gth);
            let assign106520_e158640: f64 = (assign106520_e158636 - assign106520_e158639);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14, ) = (assign106520_e158640, ((locals.var_t1_dn0 - locals.var_p_dn0) - (p.p337 * locals.var_gth_dn0)), ((locals.var_t1_dn2 - locals.var_p_dn2) - (p.p337 * locals.var_gth_dn2)), ((locals.var_t1_dn4 - locals.var_p_dn4) - (p.p337 * locals.var_gth_dn4)), ((locals.var_t1_dn5 - locals.var_p_dn5) - (p.p337 * locals.var_gth_dn5)), ((locals.var_t1_dn6 - locals.var_p_dn6) - (p.p337 * locals.var_gth_dn6)), ((locals.var_t1_dn7 - locals.var_p_dn7) - (p.p337 * locals.var_gth_dn7)), ((locals.var_t1_dn8 - locals.var_p_dn8) - (p.p337 * locals.var_gth_dn8)), ((locals.var_t1_dn9 - locals.var_p_dn9) - (p.p337 * locals.var_gth_dn9)), ((locals.var_t1_dn10 - locals.var_p_dn10) - (p.p337 * locals.var_gth_dn10)), ((locals.var_t1_dn11 - locals.var_p_dn11) - (p.p337 * locals.var_gth_dn11)), ((locals.var_t1_dn14 - locals.var_p_dn14) - (p.p337 * locals.var_gth_dn14)), );
            locals.var_tmf1_rv = 0.0;
        }
        if ((locals.var_guard2402 != 0.0) && (locals.var_guard2406 != 0.0)) {
            let assign106530_e158648: f64 = (4.0 * locals.var_t1);
            let assign106530_e158651: f64 = (p.p337 * locals.var_gth);
            let assign106530_e158652: f64 = (assign106530_e158648 * assign106530_e158651);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14, ) = (assign106530_e158652, (((4.0 * locals.var_t1_dn0) * assign106530_e158651) + (assign106530_e158648 * (p.p337 * locals.var_gth_dn0))), (((4.0 * locals.var_t1_dn2) * assign106530_e158651) + (assign106530_e158648 * (p.p337 * locals.var_gth_dn2))), (((4.0 * locals.var_t1_dn4) * assign106530_e158651) + (assign106530_e158648 * (p.p337 * locals.var_gth_dn4))), (((4.0 * locals.var_t1_dn5) * assign106530_e158651) + (assign106530_e158648 * (p.p337 * locals.var_gth_dn5))), (((4.0 * locals.var_t1_dn6) * assign106530_e158651) + (assign106530_e158648 * (p.p337 * locals.var_gth_dn6))), (((4.0 * locals.var_t1_dn7) * assign106530_e158651) + (assign106530_e158648 * (p.p337 * locals.var_gth_dn7))), (((4.0 * locals.var_t1_dn8) * assign106530_e158651) + (assign106530_e158648 * (p.p337 * locals.var_gth_dn8))), (((4.0 * locals.var_t1_dn9) * assign106530_e158651) + (assign106530_e158648 * (p.p337 * locals.var_gth_dn9))), (((4.0 * locals.var_t1_dn10) * assign106530_e158651) + (assign106530_e158648 * (p.p337 * locals.var_gth_dn10))), (((4.0 * locals.var_t1_dn11) * assign106530_e158651) + (assign106530_e158648 * (p.p337 * locals.var_gth_dn11))), (((4.0 * locals.var_t1_dn14) * assign106530_e158651) + (assign106530_e158648 * (p.p337 * locals.var_gth_dn14))), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard2402 != 0.0) && (locals.var_guard2406 != 0.0)) {
            let (assign106540_e158664, assign106540_e158664_d_n0, assign106540_e158664_d_n2, assign106540_e158664_d_n4, assign106540_e158664_d_n5, assign106540_e158664_d_n6, assign106540_e158664_d_n7, assign106540_e158664_d_n8, assign106540_e158664_d_n9, assign106540_e158664_d_n10, assign106540_e158664_d_n11, assign106540_e158664_d_n14,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    } else {
        let assign106540_e158663: f64 = (-locals.var_tmf2);
        (assign106540_e158663, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14, ) = (assign106540_e158664, assign106540_e158664_d_n0, assign106540_e158664_d_n2, assign106540_e158664_d_n4, assign106540_e158664_d_n5, assign106540_e158664_d_n6, assign106540_e158664_d_n7, assign106540_e158664_d_n8, assign106540_e158664_d_n9, assign106540_e158664_d_n10, assign106540_e158664_d_n11, assign106540_e158664_d_n14, );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard2402 != 0.0) && (locals.var_guard2406 != 0.0)) {
            let assign106550_e158672: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign106550_e158674: f64 = (assign106550_e158672 + locals.var_tmf2);
            let assign106550_e158675: f64 = (assign106550_e158674).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14, ) = (assign106550_e158675, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign106550_e158675)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign106550_e158675)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign106550_e158675)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign106550_e158675)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign106550_e158675)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign106550_e158675)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign106550_e158675)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign106550_e158675)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign106550_e158675)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign106550_e158675)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign106550_e158675)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard2402 != 0.0) && (locals.var_guard2406 != 0.0)) {
            let assign106560_e158685: f64 = (locals.var_tmf1 / locals.var_tmf2);
            let assign106560_e158686: f64 = (1.0 + assign106560_e158685);
            let assign106560_e158687: f64 = (0.5 * assign106560_e158686);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14, ) = (assign106560_e158687, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))), );
            locals.var_t0_rv = 0.0;
        }
        if ((locals.var_guard2402 != 0.0) && (locals.var_guard2406 != 0.0)) {
            let assign106570_e158697: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign106570_e158698: f64 = (0.5 * assign106570_e158697);
            let assign106570_e158699: f64 = (locals.var_t1 - assign106570_e158698);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14, ) = (assign106570_e158699, (locals.var_t1_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t1_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t1_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t1_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t1_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t1_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t1_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t1_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t1_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t1_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t1_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))), );
            locals.var_t2_rv = 0.0;
        }
        if ((locals.var_guard2402 != 0.0) && (locals.var_guard2406 != 0.0)) {
            (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14, ) = (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14, );
            locals.var_p_rv = 0.0;
        }
        if (locals.var_guard2402 == 0.0) {
            (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_gth_rv = 0.0;
            (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_p_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign106670_e158755: f64 = (locals.var_qi_nqs * locals.var_qdrat);
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn14, ) = (assign106670_e158755, (locals.var_qi_nqs * locals.var_qdrat_dn0), (locals.var_qi_nqs * locals.var_qdrat_dn2), (locals.var_qi_nqs * locals.var_qdrat_dn4), (locals.var_qi_nqs * locals.var_qdrat_dn5), (locals.var_qi_nqs * locals.var_qdrat_dn6), (locals.var_qi_nqs * locals.var_qdrat_dn7), (locals.var_qi_nqs * locals.var_qdrat_dn8), (locals.var_qi_nqs * locals.var_qdrat_dn9), (locals.var_qi_nqs * locals.var_qdrat_dn10), (locals.var_qi_nqs * locals.var_qdrat_dn11), (locals.var_qi_nqs_dn12 * locals.var_qdrat), (locals.var_qi_nqs * locals.var_qdrat_dn14), );
            locals.var_qd_nqs_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign106680_e158760: f64 = (-locals.var_qi_nqs);
            let assign106680_e158762: f64 = (assign106680_e158760 - locals.var_qb_nqs);
            (locals.var_qg_nqs, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, ) = (assign106680_e158762, (-locals.var_qi_nqs_dn12), (-locals.var_qb_nqs_dn13), );
            locals.var_qg_nqs_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign106690_e158769: f64 = (1.0 - locals.var_qdrat);
            let assign106690_e158770: f64 = (locals.var_qi_nqs * assign106690_e158769);
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn14, ) = (assign106690_e158770, (locals.var_qi_nqs * (-locals.var_qdrat_dn0)), (locals.var_qi_nqs * (-locals.var_qdrat_dn2)), (locals.var_qi_nqs * (-locals.var_qdrat_dn4)), (locals.var_qi_nqs * (-locals.var_qdrat_dn5)), (locals.var_qi_nqs * (-locals.var_qdrat_dn6)), (locals.var_qi_nqs * (-locals.var_qdrat_dn7)), (locals.var_qi_nqs * (-locals.var_qdrat_dn8)), (locals.var_qi_nqs * (-locals.var_qdrat_dn9)), (locals.var_qi_nqs * (-locals.var_qdrat_dn10)), (locals.var_qi_nqs * (-locals.var_qdrat_dn11)), (locals.var_qi_nqs_dn12 * assign106690_e158769), (locals.var_qi_nqs * (-locals.var_qdrat_dn14)), );
            locals.var_qs_nqs_rv = 0.0;
        }
        if (locals.var_flg_nqs == 0.0) {
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qg_nqs, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, ) = (0.0, 0.0, 0.0, );
            locals.var_qg_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
        }
        let assign106750_e158800: f64 = (p.p87 * locals.var_mode);
        let assign106750_e158802: f64 = (assign106750_e158800 * locals.var_ids);
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn8, locals.var_idse_dn9, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn14, ) = (assign106750_e158802, (assign106750_e158800 * locals.var_ids_dn0), (assign106750_e158800 * locals.var_ids_dn2), (assign106750_e158800 * locals.var_ids_dn4), (assign106750_e158800 * locals.var_ids_dn5), (assign106750_e158800 * locals.var_ids_dn6), (assign106750_e158800 * locals.var_ids_dn7), (assign106750_e158800 * locals.var_ids_dn8), (assign106750_e158800 * locals.var_ids_dn9), (assign106750_e158800 * locals.var_ids_dn10), (assign106750_e158800 * locals.var_ids_dn11), (assign106750_e158800 * locals.var_ids_dn14), );
        locals.var_idse_rv = 0.0;
        let assign106910_e158850: f64 = locals.var_qg_dn6;
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14, ) = (assign106910_e158850, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgdbd_rv = 0.0;
        let assign106920_e158853: f64 = (p.p87 * locals.var_cgdbd);
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14, ) = (assign106920_e158853, (p.p87 * locals.var_cgdbd_dn0), (p.p87 * locals.var_cgdbd_dn2), (p.p87 * locals.var_cgdbd_dn4), (p.p87 * locals.var_cgdbd_dn5), (p.p87 * locals.var_cgdbd_dn6), (p.p87 * locals.var_cgdbd_dn7), (p.p87 * locals.var_cgdbd_dn8), (p.p87 * locals.var_cgdbd_dn9), (p.p87 * locals.var_cgdbd_dn10), (p.p87 * locals.var_cgdbd_dn11), (p.p87 * locals.var_cgdbd_dn14), );
        locals.var_cgdbd_rv = 0.0;
        let assign106930_e158856: f64 = locals.var_qg_dn8;
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14, ) = (assign106930_e158856, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgsbd_rv = 0.0;
        let assign106940_e158859: f64 = (p.p87 * locals.var_cgsbd);
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14, ) = (assign106940_e158859, (p.p87 * locals.var_cgsbd_dn0), (p.p87 * locals.var_cgsbd_dn2), (p.p87 * locals.var_cgsbd_dn4), (p.p87 * locals.var_cgsbd_dn5), (p.p87 * locals.var_cgsbd_dn6), (p.p87 * locals.var_cgsbd_dn7), (p.p87 * locals.var_cgsbd_dn8), (p.p87 * locals.var_cgsbd_dn9), (p.p87 * locals.var_cgsbd_dn10), (p.p87 * locals.var_cgsbd_dn11), (p.p87 * locals.var_cgsbd_dn14), );
        locals.var_cgsbd_rv = 0.0;
        let assign107310_e158974: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2409 = assign107310_e158974;
        locals.var_guard2409_rv = 0.0;
        if (locals.var_guard2409 != 0.0) {
            (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn11, locals.var_cgsb_dn14, ) = (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14, );
            locals.var_cgsb_rv = 0.0;
        }
        if (locals.var_guard2409 == 0.0) {
            (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn11, locals.var_cgsb_dn14, ) = (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14, );
            locals.var_cgsb_rv = 0.0;
        }
        let assign107670_e159093: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2411 = assign107670_e159093;
        locals.var_guard2411_rv = 0.0;
        if (p.p28 != 0.0) {
            locals.var_cqi = 1.0;
            locals.var_cqi_rv = 0.0;
            locals.var_cqb = 1.0;
            locals.var_cqb_rv = 0.0;
        }
    }
    pub(super) fn stamp_transient_equations_block_0(
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
        locals: &mut StampLocals,
    ) {
        let (eq1_e1022, eq1_e1022_d_n0, eq1_e1022_d_n2, eq1_e1022_d_n4, eq1_e1022_d_n5, eq1_e1022_d_n6, eq1_e1022_d_n7, eq1_e1022_d_n8, eq1_e1022_d_n9, eq1_e1022_d_n10, eq1_e1022_d_n11, eq1_e1022_d_n14, eq1_e1022_d_n16,) = {
    if (locals.var_guard2311 != 0.0) {
        let eq1_e1019: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_q_nqs_a);
        let eq1_e1020: f64 = (locals.var_inqs0_a + eq1_e1019);
        let eq1_e1020_d_n16: f64 = (locals.var_inqs0_a_dn16 + (locals.var_q_nqs_a_dn16 * ddt_scale));
        (eq1_e1020, locals.var_inqs0_a_dn0, locals.var_inqs0_a_dn2, locals.var_inqs0_a_dn4, locals.var_inqs0_a_dn5, locals.var_inqs0_a_dn6, locals.var_inqs0_a_dn7, locals.var_inqs0_a_dn8, locals.var_inqs0_a_dn9, locals.var_inqs0_a_dn10, locals.var_inqs0_a_dn11, locals.var_inqs0_a_dn14, eq1_e1020_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1022;
        let eq1_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 16];
        let eq1_node_derivatives: [f64; 12] = [eq1_e1022_d_n0, eq1_e1022_d_n2, eq1_e1022_d_n4, eq1_e1022_d_n5, eq1_e1022_d_n6, eq1_e1022_d_n7, eq1_e1022_d_n8, eq1_e1022_d_n9, eq1_e1022_d_n10, eq1_e1022_d_n11, eq1_e1022_d_n14, eq1_e1022_d_n16];
        let eq1_branch_derivative_indices: [usize; 0] = [];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(16),
            None,
            multiplicity * (eq1_value),
            &eq1_node_derivative_indices,
            &eq1_node_derivatives,
            &eq1_branch_derivative_indices,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1029, eq2_e1029_d_n0, eq2_e1029_d_n2, eq2_e1029_d_n4, eq2_e1029_d_n5, eq2_e1029_d_n6, eq2_e1029_d_n7, eq2_e1029_d_n8, eq2_e1029_d_n9, eq2_e1029_d_n10, eq2_e1029_d_n11, eq2_e1029_d_n14, eq2_e1029_d_n17,) = {
    if (locals.var_guard2311 != 0.0) {
        let eq2_e1026: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_q_nqs_k);
        let eq2_e1027: f64 = (locals.var_inqs0_k + eq2_e1026);
        let eq2_e1027_d_n17: f64 = (locals.var_inqs0_k_dn17 + (locals.var_q_nqs_k_dn17 * ddt_scale));
        (eq2_e1027, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn11, locals.var_inqs0_k_dn14, eq2_e1027_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e1029;
        let eq2_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 17];
        let eq2_node_derivatives: [f64; 12] = [eq2_e1029_d_n0, eq2_e1029_d_n2, eq2_e1029_d_n4, eq2_e1029_d_n5, eq2_e1029_d_n6, eq2_e1029_d_n7, eq2_e1029_d_n8, eq2_e1029_d_n9, eq2_e1029_d_n10, eq2_e1029_d_n11, eq2_e1029_d_n14, eq2_e1029_d_n17];
        let eq2_branch_derivative_indices: [usize; 0] = [];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(17),
            None,
            multiplicity * (eq2_value),
            &eq2_node_derivative_indices,
            &eq2_node_derivatives,
            &eq2_branch_derivative_indices,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1046, eq5_e1046_d_n0, eq5_e1046_d_n2, eq5_e1046_d_n4, eq5_e1046_d_n5, eq5_e1046_d_n6, eq5_e1046_d_n7, eq5_e1046_d_n8, eq5_e1046_d_n9, eq5_e1046_d_n10, eq5_e1046_d_n11, eq5_e1046_d_n14, eq5_e1046_d_n18,) = {
    if (locals.var_guard2312 != 0.0) {
        let eq5_e1043: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_w_nqs_a);
        let eq5_e1044: f64 = (locals.var_iwnqs0_a + eq5_e1043);
        let eq5_e1044_d_n18: f64 = (locals.var_iwnqs0_a_dn18 + (locals.var_w_nqs_a_dn18 * ddt_scale));
        (eq5_e1044, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn11, locals.var_iwnqs0_a_dn14, eq5_e1044_d_n18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1046;
        let eq5_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 18];
        let eq5_node_derivatives: [f64; 12] = [eq5_e1046_d_n0, eq5_e1046_d_n2, eq5_e1046_d_n4, eq5_e1046_d_n5, eq5_e1046_d_n6, eq5_e1046_d_n7, eq5_e1046_d_n8, eq5_e1046_d_n9, eq5_e1046_d_n10, eq5_e1046_d_n11, eq5_e1046_d_n14, eq5_e1046_d_n18];
        let eq5_branch_derivative_indices: [usize; 0] = [];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(18),
            None,
            multiplicity * (eq5_value),
            &eq5_node_derivative_indices,
            &eq5_node_derivatives,
            &eq5_branch_derivative_indices,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq7_e1055: f64 = (locals.var_ids + locals.var_idsibpc);
        let eq7_e1055_d_n0: f64 = (locals.var_ids_dn0 + locals.var_idsibpc_dn0);
        let eq7_e1055_d_n2: f64 = (locals.var_ids_dn2 + locals.var_idsibpc_dn2);
        let eq7_e1055_d_n4: f64 = (locals.var_ids_dn4 + locals.var_idsibpc_dn4);
        let eq7_e1055_d_n5: f64 = (locals.var_ids_dn5 + locals.var_idsibpc_dn5);
        let eq7_e1055_d_n6: f64 = (locals.var_ids_dn6 + locals.var_idsibpc_dn6);
        let eq7_e1055_d_n7: f64 = (locals.var_ids_dn7 + locals.var_idsibpc_dn7);
        let eq7_e1055_d_n8: f64 = (locals.var_ids_dn8 + locals.var_idsibpc_dn8);
        let eq7_e1055_d_n9: f64 = (locals.var_ids_dn9 + locals.var_idsibpc_dn9);
        let eq7_e1055_d_n10: f64 = (locals.var_ids_dn10 + locals.var_idsibpc_dn10);
        let eq7_e1055_d_n11: f64 = (locals.var_ids_dn11 + locals.var_idsibpc_dn11);
        let eq7_e1055_d_n14: f64 = (locals.var_ids_dn14 + locals.var_idsibpc_dn14);
        let eq7_e1057: f64 = (eq7_e1055 - locals.var_idsibpcs);
        let eq7_e1057_d_n0: f64 = (eq7_e1055_d_n0 - locals.var_idsibpcs_dn0);
        let eq7_e1057_d_n2: f64 = (eq7_e1055_d_n2 - locals.var_idsibpcs_dn2);
        let eq7_e1057_d_n4: f64 = (eq7_e1055_d_n4 - locals.var_idsibpcs_dn4);
        let eq7_e1057_d_n5: f64 = (eq7_e1055_d_n5 - locals.var_idsibpcs_dn5);
        let eq7_e1057_d_n6: f64 = (eq7_e1055_d_n6 - locals.var_idsibpcs_dn6);
        let eq7_e1057_d_n7: f64 = (eq7_e1055_d_n7 - locals.var_idsibpcs_dn7);
        let eq7_e1057_d_n8: f64 = (eq7_e1055_d_n8 - locals.var_idsibpcs_dn8);
        let eq7_e1057_d_n9: f64 = (eq7_e1055_d_n9 - locals.var_idsibpcs_dn9);
        let eq7_e1057_d_n10: f64 = (eq7_e1055_d_n10 - locals.var_idsibpcs_dn10);
        let eq7_e1057_d_n11: f64 = (eq7_e1055_d_n11 - locals.var_idsibpcs_dn11);
        let eq7_e1057_d_n14: f64 = (eq7_e1055_d_n14 - locals.var_idsibpcs_dn14);
        let eq7_e1058: f64 = (p.p87 * eq7_e1057);
        let eq7_e1058_d_n0: f64 = (p.p87 * eq7_e1057_d_n0);
        let eq7_e1058_d_n2: f64 = (p.p87 * eq7_e1057_d_n2);
        let eq7_e1058_d_n4: f64 = (p.p87 * eq7_e1057_d_n4);
        let eq7_e1058_d_n5: f64 = (p.p87 * eq7_e1057_d_n5);
        let eq7_e1058_d_n6: f64 = (p.p87 * eq7_e1057_d_n6);
        let eq7_e1058_d_n7: f64 = (p.p87 * eq7_e1057_d_n7);
        let eq7_e1058_d_n8: f64 = (p.p87 * eq7_e1057_d_n8);
        let eq7_e1058_d_n9: f64 = (p.p87 * eq7_e1057_d_n9);
        let eq7_e1058_d_n10: f64 = (p.p87 * eq7_e1057_d_n10);
        let eq7_e1058_d_n11: f64 = (p.p87 * eq7_e1057_d_n11);
        let eq7_e1058_d_n14: f64 = (p.p87 * eq7_e1057_d_n14);
        let eq7_value: f64 = eq7_e1058;
        let eq7_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq7_node_derivatives: [f64; 11] = [eq7_e1058_d_n0, eq7_e1058_d_n2, eq7_e1058_d_n4, eq7_e1058_d_n5, eq7_e1058_d_n6, eq7_e1058_d_n7, eq7_e1058_d_n8, eq7_e1058_d_n9, eq7_e1058_d_n10, eq7_e1058_d_n11, eq7_e1058_d_n14];
        let eq7_branch_derivative_indices: [usize; 0] = [];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq7_value),
            &eq7_node_derivative_indices,
            &eq7_node_derivatives,
            &eq7_branch_derivative_indices,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_e1062: f64 = (locals.var_ibreak - locals.var_ibreaks);
        let eq8_e1062_d_n0: f64 = (locals.var_ibreak_dn0 - locals.var_ibreaks_dn0);
        let eq8_e1062_d_n2: f64 = (locals.var_ibreak_dn2 - locals.var_ibreaks_dn2);
        let eq8_e1062_d_n4: f64 = (locals.var_ibreak_dn4 - locals.var_ibreaks_dn4);
        let eq8_e1062_d_n5: f64 = (locals.var_ibreak_dn5 - locals.var_ibreaks_dn5);
        let eq8_e1062_d_n6: f64 = (locals.var_ibreak_dn6 - locals.var_ibreaks_dn6);
        let eq8_e1062_d_n7: f64 = (locals.var_ibreak_dn7 - locals.var_ibreaks_dn7);
        let eq8_e1062_d_n8: f64 = (locals.var_ibreak_dn8 - locals.var_ibreaks_dn8);
        let eq8_e1062_d_n9: f64 = (locals.var_ibreak_dn9 - locals.var_ibreaks_dn9);
        let eq8_e1062_d_n10: f64 = (locals.var_ibreak_dn10 - locals.var_ibreaks_dn10);
        let eq8_e1062_d_n11: f64 = (locals.var_ibreak_dn11 - locals.var_ibreaks_dn11);
        let eq8_e1062_d_n14: f64 = (locals.var_ibreak_dn14 - locals.var_ibreaks_dn14);
        let eq8_e1063: f64 = (p.p87 * eq8_e1062);
        let eq8_e1063_d_n0: f64 = (p.p87 * eq8_e1062_d_n0);
        let eq8_e1063_d_n2: f64 = (p.p87 * eq8_e1062_d_n2);
        let eq8_e1063_d_n4: f64 = (p.p87 * eq8_e1062_d_n4);
        let eq8_e1063_d_n5: f64 = (p.p87 * eq8_e1062_d_n5);
        let eq8_e1063_d_n6: f64 = (p.p87 * eq8_e1062_d_n6);
        let eq8_e1063_d_n7: f64 = (p.p87 * eq8_e1062_d_n7);
        let eq8_e1063_d_n8: f64 = (p.p87 * eq8_e1062_d_n8);
        let eq8_e1063_d_n9: f64 = (p.p87 * eq8_e1062_d_n9);
        let eq8_e1063_d_n10: f64 = (p.p87 * eq8_e1062_d_n10);
        let eq8_e1063_d_n11: f64 = (p.p87 * eq8_e1062_d_n11);
        let eq8_e1063_d_n14: f64 = (p.p87 * eq8_e1062_d_n14);
        let eq8_value: f64 = eq8_e1063;
        let eq8_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq8_node_derivatives: [f64; 11] = [eq8_e1063_d_n0, eq8_e1063_d_n2, eq8_e1063_d_n4, eq8_e1063_d_n5, eq8_e1063_d_n6, eq8_e1063_d_n7, eq8_e1063_d_n8, eq8_e1063_d_n9, eq8_e1063_d_n10, eq8_e1063_d_n11, eq8_e1063_d_n14];
        let eq8_branch_derivative_indices: [usize; 0] = [];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e1067: f64 = (locals.var_igidl + locals.var_isub);
        let eq9_e1067_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq9_e1067_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq9_e1067_d_n4: f64 = (locals.var_igidl_dn4 + locals.var_isub_dn4);
        let eq9_e1067_d_n5: f64 = (locals.var_igidl_dn5 + locals.var_isub_dn5);
        let eq9_e1067_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq9_e1067_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq9_e1067_d_n8: f64 = (locals.var_igidl_dn8 + locals.var_isub_dn8);
        let eq9_e1067_d_n9: f64 = (locals.var_igidl_dn9 + locals.var_isub_dn9);
        let eq9_e1067_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq9_e1067_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq9_e1067_d_n14: f64 = (locals.var_igidl_dn14 + locals.var_isub_dn14);
        let eq9_e1069: f64 = (eq9_e1067 + locals.var_ibjt);
        let eq9_e1069_d_n0: f64 = (eq9_e1067_d_n0 + locals.var_ibjt_dn0);
        let eq9_e1069_d_n2: f64 = (eq9_e1067_d_n2 + locals.var_ibjt_dn2);
        let eq9_e1069_d_n4: f64 = (eq9_e1067_d_n4 + locals.var_ibjt_dn4);
        let eq9_e1069_d_n5: f64 = (eq9_e1067_d_n5 + locals.var_ibjt_dn5);
        let eq9_e1069_d_n6: f64 = (eq9_e1067_d_n6 + locals.var_ibjt_dn6);
        let eq9_e1069_d_n7: f64 = (eq9_e1067_d_n7 + locals.var_ibjt_dn7);
        let eq9_e1069_d_n8: f64 = (eq9_e1067_d_n8 + locals.var_ibjt_dn8);
        let eq9_e1069_d_n9: f64 = (eq9_e1067_d_n9 + locals.var_ibjt_dn9);
        let eq9_e1069_d_n10: f64 = (eq9_e1067_d_n10 + locals.var_ibjt_dn10);
        let eq9_e1069_d_n11: f64 = (eq9_e1067_d_n11 + locals.var_ibjt_dn11);
        let eq9_e1069_d_n14: f64 = (eq9_e1067_d_n14 + locals.var_ibjt_dn14);
        let eq9_e1070: f64 = (p.p87 * eq9_e1069);
        let eq9_e1070_d_n0: f64 = (p.p87 * eq9_e1069_d_n0);
        let eq9_e1070_d_n2: f64 = (p.p87 * eq9_e1069_d_n2);
        let eq9_e1070_d_n4: f64 = (p.p87 * eq9_e1069_d_n4);
        let eq9_e1070_d_n5: f64 = (p.p87 * eq9_e1069_d_n5);
        let eq9_e1070_d_n6: f64 = (p.p87 * eq9_e1069_d_n6);
        let eq9_e1070_d_n7: f64 = (p.p87 * eq9_e1069_d_n7);
        let eq9_e1070_d_n8: f64 = (p.p87 * eq9_e1069_d_n8);
        let eq9_e1070_d_n9: f64 = (p.p87 * eq9_e1069_d_n9);
        let eq9_e1070_d_n10: f64 = (p.p87 * eq9_e1069_d_n10);
        let eq9_e1070_d_n11: f64 = (p.p87 * eq9_e1069_d_n11);
        let eq9_e1070_d_n14: f64 = (p.p87 * eq9_e1069_d_n14);
        let eq9_value: f64 = eq9_e1070;
        let eq9_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq9_node_derivatives: [f64; 11] = [eq9_e1070_d_n0, eq9_e1070_d_n2, eq9_e1070_d_n4, eq9_e1070_d_n5, eq9_e1070_d_n6, eq9_e1070_d_n7, eq9_e1070_d_n8, eq9_e1070_d_n9, eq9_e1070_d_n10, eq9_e1070_d_n11, eq9_e1070_d_n14];
        let eq9_branch_derivative_indices: [usize; 0] = [];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(9),
            multiplicity * (eq9_value),
            &eq9_node_derivative_indices,
            &eq9_node_derivatives,
            &eq9_branch_derivative_indices,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e1074: f64 = (locals.var_igisl + locals.var_isubs);
        let eq10_e1074_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq10_e1074_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq10_e1074_d_n4: f64 = (locals.var_igisl_dn4 + locals.var_isubs_dn4);
        let eq10_e1074_d_n5: f64 = (locals.var_igisl_dn5 + locals.var_isubs_dn5);
        let eq10_e1074_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq10_e1074_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq10_e1074_d_n8: f64 = (locals.var_igisl_dn8 + locals.var_isubs_dn8);
        let eq10_e1074_d_n9: f64 = (locals.var_igisl_dn9 + locals.var_isubs_dn9);
        let eq10_e1074_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq10_e1074_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq10_e1074_d_n14: f64 = (locals.var_igisl_dn14 + locals.var_isubs_dn14);
        let eq10_e1076: f64 = (eq10_e1074 + locals.var_ibjts);
        let eq10_e1076_d_n0: f64 = (eq10_e1074_d_n0 + locals.var_ibjts_dn0);
        let eq10_e1076_d_n2: f64 = (eq10_e1074_d_n2 + locals.var_ibjts_dn2);
        let eq10_e1076_d_n4: f64 = (eq10_e1074_d_n4 + locals.var_ibjts_dn4);
        let eq10_e1076_d_n5: f64 = (eq10_e1074_d_n5 + locals.var_ibjts_dn5);
        let eq10_e1076_d_n6: f64 = (eq10_e1074_d_n6 + locals.var_ibjts_dn6);
        let eq10_e1076_d_n7: f64 = (eq10_e1074_d_n7 + locals.var_ibjts_dn7);
        let eq10_e1076_d_n8: f64 = (eq10_e1074_d_n8 + locals.var_ibjts_dn8);
        let eq10_e1076_d_n9: f64 = (eq10_e1074_d_n9 + locals.var_ibjts_dn9);
        let eq10_e1076_d_n10: f64 = (eq10_e1074_d_n10 + locals.var_ibjts_dn10);
        let eq10_e1076_d_n11: f64 = (eq10_e1074_d_n11 + locals.var_ibjts_dn11);
        let eq10_e1076_d_n14: f64 = (eq10_e1074_d_n14 + locals.var_ibjts_dn14);
        let eq10_e1077: f64 = (p.p87 * eq10_e1076);
        let eq10_e1077_d_n0: f64 = (p.p87 * eq10_e1076_d_n0);
        let eq10_e1077_d_n2: f64 = (p.p87 * eq10_e1076_d_n2);
        let eq10_e1077_d_n4: f64 = (p.p87 * eq10_e1076_d_n4);
        let eq10_e1077_d_n5: f64 = (p.p87 * eq10_e1076_d_n5);
        let eq10_e1077_d_n6: f64 = (p.p87 * eq10_e1076_d_n6);
        let eq10_e1077_d_n7: f64 = (p.p87 * eq10_e1076_d_n7);
        let eq10_e1077_d_n8: f64 = (p.p87 * eq10_e1076_d_n8);
        let eq10_e1077_d_n9: f64 = (p.p87 * eq10_e1076_d_n9);
        let eq10_e1077_d_n10: f64 = (p.p87 * eq10_e1076_d_n10);
        let eq10_e1077_d_n11: f64 = (p.p87 * eq10_e1076_d_n11);
        let eq10_e1077_d_n14: f64 = (p.p87 * eq10_e1076_d_n14);
        let eq10_value: f64 = eq10_e1077;
        let eq10_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq10_node_derivatives: [f64; 11] = [eq10_e1077_d_n0, eq10_e1077_d_n2, eq10_e1077_d_n4, eq10_e1077_d_n5, eq10_e1077_d_n6, eq10_e1077_d_n7, eq10_e1077_d_n8, eq10_e1077_d_n9, eq10_e1077_d_n10, eq10_e1077_d_n11, eq10_e1077_d_n14];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e1080: f64 = (p.p87 * locals.var_isubld);
        let eq11_e1080_d_n0: f64 = (p.p87 * locals.var_isubld_dn0);
        let eq11_e1080_d_n2: f64 = (p.p87 * locals.var_isubld_dn2);
        let eq11_e1080_d_n4: f64 = (p.p87 * locals.var_isubld_dn4);
        let eq11_e1080_d_n5: f64 = (p.p87 * locals.var_isubld_dn5);
        let eq11_e1080_d_n6: f64 = (p.p87 * locals.var_isubld_dn6);
        let eq11_e1080_d_n7: f64 = (p.p87 * locals.var_isubld_dn7);
        let eq11_e1080_d_n8: f64 = (p.p87 * locals.var_isubld_dn8);
        let eq11_e1080_d_n9: f64 = (p.p87 * locals.var_isubld_dn9);
        let eq11_e1080_d_n10: f64 = (p.p87 * locals.var_isubld_dn10);
        let eq11_e1080_d_n11: f64 = (p.p87 * locals.var_isubld_dn11);
        let eq11_e1080_d_n14: f64 = (p.p87 * locals.var_isubld_dn14);
        let eq11_value: f64 = eq11_e1080;
        let eq11_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq11_node_derivatives: [f64; 11] = [eq11_e1080_d_n0, eq11_e1080_d_n2, eq11_e1080_d_n4, eq11_e1080_d_n5, eq11_e1080_d_n6, eq11_e1080_d_n7, eq11_e1080_d_n8, eq11_e1080_d_n9, eq11_e1080_d_n10, eq11_e1080_d_n11, eq11_e1080_d_n14];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e1083: f64 = (p.p87 * locals.var_isublds);
        let eq12_e1083_d_n0: f64 = (p.p87 * locals.var_isublds_dn0);
        let eq12_e1083_d_n2: f64 = (p.p87 * locals.var_isublds_dn2);
        let eq12_e1083_d_n4: f64 = (p.p87 * locals.var_isublds_dn4);
        let eq12_e1083_d_n5: f64 = (p.p87 * locals.var_isublds_dn5);
        let eq12_e1083_d_n6: f64 = (p.p87 * locals.var_isublds_dn6);
        let eq12_e1083_d_n7: f64 = (p.p87 * locals.var_isublds_dn7);
        let eq12_e1083_d_n8: f64 = (p.p87 * locals.var_isublds_dn8);
        let eq12_e1083_d_n9: f64 = (p.p87 * locals.var_isublds_dn9);
        let eq12_e1083_d_n10: f64 = (p.p87 * locals.var_isublds_dn10);
        let eq12_e1083_d_n11: f64 = (p.p87 * locals.var_isublds_dn11);
        let eq12_e1083_d_n14: f64 = (p.p87 * locals.var_isublds_dn14);
        let eq12_value: f64 = eq12_e1083;
        let eq12_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq12_node_derivatives: [f64; 11] = [eq12_e1083_d_n0, eq12_e1083_d_n2, eq12_e1083_d_n4, eq12_e1083_d_n5, eq12_e1083_d_n6, eq12_e1083_d_n7, eq12_e1083_d_n8, eq12_e1083_d_n9, eq12_e1083_d_n10, eq12_e1083_d_n11, eq12_e1083_d_n14];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(9),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e1086: f64 = (p.p87 * locals.var_ibs);
        let eq13_e1086_d_n0: f64 = (p.p87 * locals.var_ibs_dn0);
        let eq13_e1086_d_n2: f64 = (p.p87 * locals.var_ibs_dn2);
        let eq13_e1086_d_n4: f64 = (p.p87 * locals.var_ibs_dn4);
        let eq13_e1086_d_n5: f64 = (p.p87 * locals.var_ibs_dn5);
        let eq13_e1086_d_n6: f64 = (p.p87 * locals.var_ibs_dn6);
        let eq13_e1086_d_n7: f64 = (p.p87 * locals.var_ibs_dn7);
        let eq13_e1086_d_n8: f64 = (p.p87 * locals.var_ibs_dn8);
        let eq13_e1086_d_n9: f64 = (p.p87 * locals.var_ibs_dn9);
        let eq13_e1086_d_n10: f64 = (p.p87 * locals.var_ibs_dn10);
        let eq13_e1086_d_n11: f64 = (p.p87 * locals.var_ibs_dn11);
        let eq13_e1086_d_n14: f64 = (p.p87 * locals.var_ibs_dn14);
        let eq13_value: f64 = eq13_e1086;
        let eq13_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq13_node_derivatives: [f64; 11] = [eq13_e1086_d_n0, eq13_e1086_d_n2, eq13_e1086_d_n4, eq13_e1086_d_n5, eq13_e1086_d_n6, eq13_e1086_d_n7, eq13_e1086_d_n8, eq13_e1086_d_n9, eq13_e1086_d_n10, eq13_e1086_d_n11, eq13_e1086_d_n14];
        let eq13_branch_derivative_indices: [usize; 0] = [];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(2),
            multiplicity * (eq13_value),
            &eq13_node_derivative_indices,
            &eq13_node_derivatives,
            &eq13_branch_derivative_indices,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e1089: f64 = (p.p87 * locals.var_ibd);
        let eq14_e1089_d_n0: f64 = (p.p87 * locals.var_ibd_dn0);
        let eq14_e1089_d_n2: f64 = (p.p87 * locals.var_ibd_dn2);
        let eq14_e1089_d_n4: f64 = (p.p87 * locals.var_ibd_dn4);
        let eq14_e1089_d_n5: f64 = (p.p87 * locals.var_ibd_dn5);
        let eq14_e1089_d_n6: f64 = (p.p87 * locals.var_ibd_dn6);
        let eq14_e1089_d_n7: f64 = (p.p87 * locals.var_ibd_dn7);
        let eq14_e1089_d_n8: f64 = (p.p87 * locals.var_ibd_dn8);
        let eq14_e1089_d_n9: f64 = (p.p87 * locals.var_ibd_dn9);
        let eq14_e1089_d_n10: f64 = (p.p87 * locals.var_ibd_dn10);
        let eq14_e1089_d_n11: f64 = (p.p87 * locals.var_ibd_dn11);
        let eq14_e1089_d_n14: f64 = (p.p87 * locals.var_ibd_dn14);
        let eq14_value: f64 = eq14_e1089;
        let eq14_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq14_node_derivatives: [f64; 11] = [eq14_e1089_d_n0, eq14_e1089_d_n2, eq14_e1089_d_n4, eq14_e1089_d_n5, eq14_e1089_d_n6, eq14_e1089_d_n7, eq14_e1089_d_n8, eq14_e1089_d_n9, eq14_e1089_d_n10, eq14_e1089_d_n11, eq14_e1089_d_n14];
        let eq14_branch_derivative_indices: [usize; 0] = [];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq14_value),
            &eq14_node_derivative_indices,
            &eq14_node_derivatives,
            &eq14_branch_derivative_indices,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e1092: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qbs);
        let eq15_e1093: f64 = (p.p87 * eq15_e1092);
        let eq15_e1093_d_n0: f64 = (p.p87 * (locals.var_qbs_dn0 * ddt_scale));
        let eq15_e1093_d_n2: f64 = (p.p87 * (locals.var_qbs_dn2 * ddt_scale));
        let eq15_e1093_d_n4: f64 = (p.p87 * (locals.var_qbs_dn4 * ddt_scale));
        let eq15_e1093_d_n5: f64 = (p.p87 * (locals.var_qbs_dn5 * ddt_scale));
        let eq15_e1093_d_n6: f64 = (p.p87 * (locals.var_qbs_dn6 * ddt_scale));
        let eq15_e1093_d_n7: f64 = (p.p87 * (locals.var_qbs_dn7 * ddt_scale));
        let eq15_e1093_d_n8: f64 = (p.p87 * (locals.var_qbs_dn8 * ddt_scale));
        let eq15_e1093_d_n9: f64 = (p.p87 * (locals.var_qbs_dn9 * ddt_scale));
        let eq15_e1093_d_n10: f64 = (p.p87 * (locals.var_qbs_dn10 * ddt_scale));
        let eq15_e1093_d_n11: f64 = (p.p87 * (locals.var_qbs_dn11 * ddt_scale));
        let eq15_e1093_d_n14: f64 = (p.p87 * (locals.var_qbs_dn14 * ddt_scale));
        let eq15_value: f64 = eq15_e1093;
        let eq15_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq15_node_derivatives: [f64; 11] = [eq15_e1093_d_n0, eq15_e1093_d_n2, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n11, eq15_e1093_d_n14];
        let eq15_branch_derivative_indices: [usize; 0] = [];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(2),
            multiplicity * (eq15_value),
            &eq15_node_derivative_indices,
            &eq15_node_derivatives,
            &eq15_branch_derivative_indices,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e1096: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, locals.var_qbd);
        let eq16_e1097: f64 = (p.p87 * eq16_e1096);
        let eq16_e1097_d_n0: f64 = (p.p87 * (locals.var_qbd_dn0 * ddt_scale));
        let eq16_e1097_d_n2: f64 = (p.p87 * (locals.var_qbd_dn2 * ddt_scale));
        let eq16_e1097_d_n4: f64 = (p.p87 * (locals.var_qbd_dn4 * ddt_scale));
        let eq16_e1097_d_n5: f64 = (p.p87 * (locals.var_qbd_dn5 * ddt_scale));
        let eq16_e1097_d_n6: f64 = (p.p87 * (locals.var_qbd_dn6 * ddt_scale));
        let eq16_e1097_d_n7: f64 = (p.p87 * (locals.var_qbd_dn7 * ddt_scale));
        let eq16_e1097_d_n8: f64 = (p.p87 * (locals.var_qbd_dn8 * ddt_scale));
        let eq16_e1097_d_n9: f64 = (p.p87 * (locals.var_qbd_dn9 * ddt_scale));
        let eq16_e1097_d_n10: f64 = (p.p87 * (locals.var_qbd_dn10 * ddt_scale));
        let eq16_e1097_d_n11: f64 = (p.p87 * (locals.var_qbd_dn11 * ddt_scale));
        let eq16_e1097_d_n14: f64 = (p.p87 * (locals.var_qbd_dn14 * ddt_scale));
        let eq16_e1097_d_n16: f64 = (p.p87 * (locals.var_qbd_dn16 * ddt_scale));
        let eq16_e1097_d_n17: f64 = (p.p87 * (locals.var_qbd_dn17 * ddt_scale));
        let eq16_e1097_d_n18: f64 = (p.p87 * (locals.var_qbd_dn18 * ddt_scale));
        let eq16_value: f64 = eq16_e1097;
        let eq16_node_derivative_indices: [usize; 14] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 16, 17, 18];
        let eq16_node_derivatives: [f64; 14] = [eq16_e1097_d_n0, eq16_e1097_d_n2, eq16_e1097_d_n4, eq16_e1097_d_n5, eq16_e1097_d_n6, eq16_e1097_d_n7, eq16_e1097_d_n8, eq16_e1097_d_n9, eq16_e1097_d_n10, eq16_e1097_d_n11, eq16_e1097_d_n14, eq16_e1097_d_n16, eq16_e1097_d_n17, eq16_e1097_d_n18];
        let eq16_branch_derivative_indices: [usize; 0] = [];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq16_value),
            &eq16_node_derivative_indices,
            &eq16_node_derivatives,
            &eq16_branch_derivative_indices,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq17_e1103, eq17_e1103_d_n0, eq17_e1103_d_n2, eq17_e1103_d_n4, eq17_e1103_d_n5, eq17_e1103_d_n6, eq17_e1103_d_n7, eq17_e1103_d_n8, eq17_e1103_d_n9, eq17_e1103_d_n10, eq17_e1103_d_n11, eq17_e1103_d_n14,) = {
    if (locals.var_guard2411 != 0.0) {
        let eq17_e1101: f64 = (p.p87 * locals.var_ibsi);
        let eq17_e1101_d_n0: f64 = (p.p87 * locals.var_ibsi_dn0);
        let eq17_e1101_d_n2: f64 = (p.p87 * locals.var_ibsi_dn2);
        let eq17_e1101_d_n4: f64 = (p.p87 * locals.var_ibsi_dn4);
        let eq17_e1101_d_n5: f64 = (p.p87 * locals.var_ibsi_dn5);
        let eq17_e1101_d_n6: f64 = (p.p87 * locals.var_ibsi_dn6);
        let eq17_e1101_d_n7: f64 = (p.p87 * locals.var_ibsi_dn7);
        let eq17_e1101_d_n8: f64 = (p.p87 * locals.var_ibsi_dn8);
        let eq17_e1101_d_n9: f64 = (p.p87 * locals.var_ibsi_dn9);
        let eq17_e1101_d_n10: f64 = (p.p87 * locals.var_ibsi_dn10);
        let eq17_e1101_d_n11: f64 = (p.p87 * locals.var_ibsi_dn11);
        let eq17_e1101_d_n14: f64 = (p.p87 * locals.var_ibsi_dn14);
        (eq17_e1101, eq17_e1101_d_n0, eq17_e1101_d_n2, eq17_e1101_d_n4, eq17_e1101_d_n5, eq17_e1101_d_n6, eq17_e1101_d_n7, eq17_e1101_d_n8, eq17_e1101_d_n9, eq17_e1101_d_n10, eq17_e1101_d_n11, eq17_e1101_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1103;
        let eq17_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq17_node_derivatives: [f64; 11] = [eq17_e1103_d_n0, eq17_e1103_d_n2, eq17_e1103_d_n4, eq17_e1103_d_n5, eq17_e1103_d_n6, eq17_e1103_d_n7, eq17_e1103_d_n8, eq17_e1103_d_n9, eq17_e1103_d_n10, eq17_e1103_d_n11, eq17_e1103_d_n14];
        let eq17_branch_derivative_indices: [usize; 0] = [];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq17_value),
            &eq17_node_derivative_indices,
            &eq17_node_derivatives,
            &eq17_branch_derivative_indices,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1109, eq18_e1109_d_n0, eq18_e1109_d_n2, eq18_e1109_d_n4, eq18_e1109_d_n5, eq18_e1109_d_n6, eq18_e1109_d_n7, eq18_e1109_d_n8, eq18_e1109_d_n9, eq18_e1109_d_n10, eq18_e1109_d_n11, eq18_e1109_d_n14,) = {
    if (locals.var_guard2411 != 0.0) {
        let eq18_e1107: f64 = (p.p87 * locals.var_ibdi);
        let eq18_e1107_d_n0: f64 = (p.p87 * locals.var_ibdi_dn0);
        let eq18_e1107_d_n2: f64 = (p.p87 * locals.var_ibdi_dn2);
        let eq18_e1107_d_n4: f64 = (p.p87 * locals.var_ibdi_dn4);
        let eq18_e1107_d_n5: f64 = (p.p87 * locals.var_ibdi_dn5);
        let eq18_e1107_d_n6: f64 = (p.p87 * locals.var_ibdi_dn6);
        let eq18_e1107_d_n7: f64 = (p.p87 * locals.var_ibdi_dn7);
        let eq18_e1107_d_n8: f64 = (p.p87 * locals.var_ibdi_dn8);
        let eq18_e1107_d_n9: f64 = (p.p87 * locals.var_ibdi_dn9);
        let eq18_e1107_d_n10: f64 = (p.p87 * locals.var_ibdi_dn10);
        let eq18_e1107_d_n11: f64 = (p.p87 * locals.var_ibdi_dn11);
        let eq18_e1107_d_n14: f64 = (p.p87 * locals.var_ibdi_dn14);
        (eq18_e1107, eq18_e1107_d_n0, eq18_e1107_d_n2, eq18_e1107_d_n4, eq18_e1107_d_n5, eq18_e1107_d_n6, eq18_e1107_d_n7, eq18_e1107_d_n8, eq18_e1107_d_n9, eq18_e1107_d_n10, eq18_e1107_d_n11, eq18_e1107_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1109;
        let eq18_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq18_node_derivatives: [f64; 11] = [eq18_e1109_d_n0, eq18_e1109_d_n2, eq18_e1109_d_n4, eq18_e1109_d_n5, eq18_e1109_d_n6, eq18_e1109_d_n7, eq18_e1109_d_n8, eq18_e1109_d_n9, eq18_e1109_d_n10, eq18_e1109_d_n11, eq18_e1109_d_n14];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
    }
    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq19_e1116, eq19_e1116_d_n0, eq19_e1116_d_n2, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, eq19_e1116_d_n14,) = {
    if (locals.var_guard2411 != 0.0) {
        let eq19_e1113: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, locals.var_qbsi);
        let eq19_e1114: f64 = (p.p87 * eq19_e1113);
        let eq19_e1114_d_n0: f64 = (p.p87 * (locals.var_qbsi_dn0 * ddt_scale));
        let eq19_e1114_d_n2: f64 = (p.p87 * (locals.var_qbsi_dn2 * ddt_scale));
        let eq19_e1114_d_n4: f64 = (p.p87 * (locals.var_qbsi_dn4 * ddt_scale));
        let eq19_e1114_d_n5: f64 = (p.p87 * (locals.var_qbsi_dn5 * ddt_scale));
        let eq19_e1114_d_n6: f64 = (p.p87 * (locals.var_qbsi_dn6 * ddt_scale));
        let eq19_e1114_d_n7: f64 = (p.p87 * (locals.var_qbsi_dn7 * ddt_scale));
        let eq19_e1114_d_n8: f64 = (p.p87 * (locals.var_qbsi_dn8 * ddt_scale));
        let eq19_e1114_d_n9: f64 = (p.p87 * (locals.var_qbsi_dn9 * ddt_scale));
        let eq19_e1114_d_n10: f64 = (p.p87 * (locals.var_qbsi_dn10 * ddt_scale));
        let eq19_e1114_d_n11: f64 = (p.p87 * (locals.var_qbsi_dn11 * ddt_scale));
        let eq19_e1114_d_n14: f64 = (p.p87 * (locals.var_qbsi_dn14 * ddt_scale));
        (eq19_e1114, eq19_e1114_d_n0, eq19_e1114_d_n2, eq19_e1114_d_n4, eq19_e1114_d_n5, eq19_e1114_d_n6, eq19_e1114_d_n7, eq19_e1114_d_n8, eq19_e1114_d_n9, eq19_e1114_d_n10, eq19_e1114_d_n11, eq19_e1114_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e1116;
        let eq19_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq19_node_derivatives: [f64; 11] = [eq19_e1116_d_n0, eq19_e1116_d_n2, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, eq19_e1116_d_n14];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n2, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n14,) = {
    if (locals.var_guard2411 != 0.0) {
        let eq20_e1120: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qbdi);
        let eq20_e1121: f64 = (p.p87 * eq20_e1120);
        let eq20_e1121_d_n0: f64 = (p.p87 * (locals.var_qbdi_dn0 * ddt_scale));
        let eq20_e1121_d_n2: f64 = (p.p87 * (locals.var_qbdi_dn2 * ddt_scale));
        let eq20_e1121_d_n4: f64 = (p.p87 * (locals.var_qbdi_dn4 * ddt_scale));
        let eq20_e1121_d_n5: f64 = (p.p87 * (locals.var_qbdi_dn5 * ddt_scale));
        let eq20_e1121_d_n6: f64 = (p.p87 * (locals.var_qbdi_dn6 * ddt_scale));
        let eq20_e1121_d_n7: f64 = (p.p87 * (locals.var_qbdi_dn7 * ddt_scale));
        let eq20_e1121_d_n8: f64 = (p.p87 * (locals.var_qbdi_dn8 * ddt_scale));
        let eq20_e1121_d_n9: f64 = (p.p87 * (locals.var_qbdi_dn9 * ddt_scale));
        let eq20_e1121_d_n10: f64 = (p.p87 * (locals.var_qbdi_dn10 * ddt_scale));
        let eq20_e1121_d_n11: f64 = (p.p87 * (locals.var_qbdi_dn11 * ddt_scale));
        let eq20_e1121_d_n14: f64 = (p.p87 * (locals.var_qbdi_dn14 * ddt_scale));
        (eq20_e1121, eq20_e1121_d_n0, eq20_e1121_d_n2, eq20_e1121_d_n4, eq20_e1121_d_n5, eq20_e1121_d_n6, eq20_e1121_d_n7, eq20_e1121_d_n8, eq20_e1121_d_n9, eq20_e1121_d_n10, eq20_e1121_d_n11, eq20_e1121_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e1123;
        let eq20_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq20_node_derivatives: [f64; 11] = [eq20_e1123_d_n0, eq20_e1123_d_n2, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n14];
        let eq20_branch_derivative_indices: [usize; 0] = [];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq21_e1129, eq21_e1129_d_n0, eq21_e1129_d_n2, eq21_e1129_d_n4, eq21_e1129_d_n5, eq21_e1129_d_n6, eq21_e1129_d_n7, eq21_e1129_d_n8, eq21_e1129_d_n9, eq21_e1129_d_n10, eq21_e1129_d_n11, eq21_e1129_d_n14,) = {
    if (locals.var_guard2412 != 0.0) {
        let eq21_e1127: f64 = (p.p87 * locals.var_igs);
        let eq21_e1127_d_n0: f64 = (p.p87 * locals.var_igs_dn0);
        let eq21_e1127_d_n2: f64 = (p.p87 * locals.var_igs_dn2);
        let eq21_e1127_d_n4: f64 = (p.p87 * locals.var_igs_dn4);
        let eq21_e1127_d_n5: f64 = (p.p87 * locals.var_igs_dn5);
        let eq21_e1127_d_n6: f64 = (p.p87 * locals.var_igs_dn6);
        let eq21_e1127_d_n7: f64 = (p.p87 * locals.var_igs_dn7);
        let eq21_e1127_d_n8: f64 = (p.p87 * locals.var_igs_dn8);
        let eq21_e1127_d_n9: f64 = (p.p87 * locals.var_igs_dn9);
        let eq21_e1127_d_n10: f64 = (p.p87 * locals.var_igs_dn10);
        let eq21_e1127_d_n11: f64 = (p.p87 * locals.var_igs_dn11);
        let eq21_e1127_d_n14: f64 = (p.p87 * locals.var_igs_dn14);
        (eq21_e1127, eq21_e1127_d_n0, eq21_e1127_d_n2, eq21_e1127_d_n4, eq21_e1127_d_n5, eq21_e1127_d_n6, eq21_e1127_d_n7, eq21_e1127_d_n8, eq21_e1127_d_n9, eq21_e1127_d_n10, eq21_e1127_d_n11, eq21_e1127_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1129;
        let eq21_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq21_node_derivatives: [f64; 11] = [eq21_e1129_d_n0, eq21_e1129_d_n2, eq21_e1129_d_n4, eq21_e1129_d_n5, eq21_e1129_d_n6, eq21_e1129_d_n7, eq21_e1129_d_n8, eq21_e1129_d_n9, eq21_e1129_d_n10, eq21_e1129_d_n11, eq21_e1129_d_n14];
        let eq21_branch_derivative_indices: [usize; 0] = [];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq21_value),
            &eq21_node_derivative_indices,
            &eq21_node_derivatives,
            &eq21_branch_derivative_indices,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e1135, eq22_e1135_d_n0, eq22_e1135_d_n2, eq22_e1135_d_n4, eq22_e1135_d_n5, eq22_e1135_d_n6, eq22_e1135_d_n7, eq22_e1135_d_n8, eq22_e1135_d_n9, eq22_e1135_d_n10, eq22_e1135_d_n11, eq22_e1135_d_n14,) = {
    if (locals.var_guard2412 != 0.0) {
        let eq22_e1133: f64 = (p.p87 * locals.var_igd);
        let eq22_e1133_d_n0: f64 = (p.p87 * locals.var_igd_dn0);
        let eq22_e1133_d_n2: f64 = (p.p87 * locals.var_igd_dn2);
        let eq22_e1133_d_n4: f64 = (p.p87 * locals.var_igd_dn4);
        let eq22_e1133_d_n5: f64 = (p.p87 * locals.var_igd_dn5);
        let eq22_e1133_d_n6: f64 = (p.p87 * locals.var_igd_dn6);
        let eq22_e1133_d_n7: f64 = (p.p87 * locals.var_igd_dn7);
        let eq22_e1133_d_n8: f64 = (p.p87 * locals.var_igd_dn8);
        let eq22_e1133_d_n9: f64 = (p.p87 * locals.var_igd_dn9);
        let eq22_e1133_d_n10: f64 = (p.p87 * locals.var_igd_dn10);
        let eq22_e1133_d_n11: f64 = (p.p87 * locals.var_igd_dn11);
        let eq22_e1133_d_n14: f64 = (p.p87 * locals.var_igd_dn14);
        (eq22_e1133, eq22_e1133_d_n0, eq22_e1133_d_n2, eq22_e1133_d_n4, eq22_e1133_d_n5, eq22_e1133_d_n6, eq22_e1133_d_n7, eq22_e1133_d_n8, eq22_e1133_d_n9, eq22_e1133_d_n10, eq22_e1133_d_n11, eq22_e1133_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e1135;
        let eq22_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq22_node_derivatives: [f64; 11] = [eq22_e1135_d_n0, eq22_e1135_d_n2, eq22_e1135_d_n4, eq22_e1135_d_n5, eq22_e1135_d_n6, eq22_e1135_d_n7, eq22_e1135_d_n8, eq22_e1135_d_n9, eq22_e1135_d_n10, eq22_e1135_d_n11, eq22_e1135_d_n14];
        let eq22_branch_derivative_indices: [usize; 0] = [];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq22_value),
            &eq22_node_derivative_indices,
            &eq22_node_derivatives,
            &eq22_branch_derivative_indices,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1141, eq23_e1141_d_n0, eq23_e1141_d_n2, eq23_e1141_d_n4, eq23_e1141_d_n5, eq23_e1141_d_n6, eq23_e1141_d_n7, eq23_e1141_d_n8, eq23_e1141_d_n9, eq23_e1141_d_n10, eq23_e1141_d_n11, eq23_e1141_d_n14,) = {
    if (locals.var_guard2412 != 0.0) {
        let eq23_e1139: f64 = (p.p87 * locals.var_igb);
        let eq23_e1139_d_n0: f64 = (p.p87 * locals.var_igb_dn0);
        let eq23_e1139_d_n2: f64 = (p.p87 * locals.var_igb_dn2);
        let eq23_e1139_d_n4: f64 = (p.p87 * locals.var_igb_dn4);
        let eq23_e1139_d_n5: f64 = (p.p87 * locals.var_igb_dn5);
        let eq23_e1139_d_n6: f64 = (p.p87 * locals.var_igb_dn6);
        let eq23_e1139_d_n7: f64 = (p.p87 * locals.var_igb_dn7);
        let eq23_e1139_d_n8: f64 = (p.p87 * locals.var_igb_dn8);
        let eq23_e1139_d_n9: f64 = (p.p87 * locals.var_igb_dn9);
        let eq23_e1139_d_n10: f64 = (p.p87 * locals.var_igb_dn10);
        let eq23_e1139_d_n11: f64 = (p.p87 * locals.var_igb_dn11);
        let eq23_e1139_d_n14: f64 = (p.p87 * locals.var_igb_dn14);
        (eq23_e1139, eq23_e1139_d_n0, eq23_e1139_d_n2, eq23_e1139_d_n4, eq23_e1139_d_n5, eq23_e1139_d_n6, eq23_e1139_d_n7, eq23_e1139_d_n8, eq23_e1139_d_n9, eq23_e1139_d_n10, eq23_e1139_d_n11, eq23_e1139_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1141;
        let eq23_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq23_node_derivatives: [f64; 11] = [eq23_e1141_d_n0, eq23_e1141_d_n2, eq23_e1141_d_n4, eq23_e1141_d_n5, eq23_e1141_d_n6, eq23_e1141_d_n7, eq23_e1141_d_n8, eq23_e1141_d_n9, eq23_e1141_d_n10, eq23_e1141_d_n11, eq23_e1141_d_n14];
        let eq23_branch_derivative_indices: [usize; 0] = [];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq23_value),
            &eq23_node_derivative_indices,
            &eq23_node_derivatives,
            &eq23_branch_derivative_indices,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1147, eq24_e1147_d_n0, eq24_e1147_d_n2, eq24_e1147_d_n4, eq24_e1147_d_n5, eq24_e1147_d_n6, eq24_e1147_d_n7, eq24_e1147_d_n8, eq24_e1147_d_n9, eq24_e1147_d_n10, eq24_e1147_d_n11, eq24_e1147_d_n14,) = {
    if (locals.var_flg_rd != 0.0) {
        let eq24_e1145: f64 = ((nv0 - nv6) / locals.var_rdd);
        let eq24_e1145_d_n0: f64 = ((locals.var_rdd - ((nv0 - nv6) * locals.var_rdd_dn0)) / (locals.var_rdd * locals.var_rdd));
        let eq24_e1145_d_n2: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn2) / (locals.var_rdd * locals.var_rdd)));
        let eq24_e1145_d_n4: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn4) / (locals.var_rdd * locals.var_rdd)));
        let eq24_e1145_d_n5: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn5) / (locals.var_rdd * locals.var_rdd)));
        let eq24_e1145_d_n6: f64 = (((-locals.var_rdd) - ((nv0 - nv6) * locals.var_rdd_dn6)) / (locals.var_rdd * locals.var_rdd));
        let eq24_e1145_d_n7: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn7) / (locals.var_rdd * locals.var_rdd)));
        let eq24_e1145_d_n8: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn8) / (locals.var_rdd * locals.var_rdd)));
        let eq24_e1145_d_n9: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn9) / (locals.var_rdd * locals.var_rdd)));
        let eq24_e1145_d_n10: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn10) / (locals.var_rdd * locals.var_rdd)));
        let eq24_e1145_d_n11: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn11) / (locals.var_rdd * locals.var_rdd)));
        let eq24_e1145_d_n14: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn14) / (locals.var_rdd * locals.var_rdd)));
        (eq24_e1145, eq24_e1145_d_n0, eq24_e1145_d_n2, eq24_e1145_d_n4, eq24_e1145_d_n5, eq24_e1145_d_n6, eq24_e1145_d_n7, eq24_e1145_d_n8, eq24_e1145_d_n9, eq24_e1145_d_n10, eq24_e1145_d_n11, eq24_e1145_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1147;
        let eq24_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq24_node_derivatives: [f64; 11] = [eq24_e1147_d_n0, eq24_e1147_d_n2, eq24_e1147_d_n4, eq24_e1147_d_n5, eq24_e1147_d_n6, eq24_e1147_d_n7, eq24_e1147_d_n8, eq24_e1147_d_n9, eq24_e1147_d_n10, eq24_e1147_d_n11, eq24_e1147_d_n14];
        let eq24_branch_derivative_indices: [usize; 0] = [];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq24_value),
            &eq24_node_derivative_indices,
            &eq24_node_derivatives,
            &eq24_branch_derivative_indices,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1158, eq26_e1158_d_n0, eq26_e1158_d_n2, eq26_e1158_d_n4, eq26_e1158_d_n5, eq26_e1158_d_n6, eq26_e1158_d_n7, eq26_e1158_d_n8, eq26_e1158_d_n9, eq26_e1158_d_n10, eq26_e1158_d_n11, eq26_e1158_d_n14,) = {
    if (locals.var_flg_rs != 0.0) {
        let eq26_e1156: f64 = ((nv8 - nv2) / locals.var_rsd);
        let eq26_e1156_d_n0: f64 = (-(((nv8 - nv2) * locals.var_rsd_dn0) / (locals.var_rsd * locals.var_rsd)));
        let eq26_e1156_d_n2: f64 = (((-locals.var_rsd) - ((nv8 - nv2) * locals.var_rsd_dn2)) / (locals.var_rsd * locals.var_rsd));
        let eq26_e1156_d_n4: f64 = (-(((nv8 - nv2) * locals.var_rsd_dn4) / (locals.var_rsd * locals.var_rsd)));
        let eq26_e1156_d_n5: f64 = (-(((nv8 - nv2) * locals.var_rsd_dn5) / (locals.var_rsd * locals.var_rsd)));
        let eq26_e1156_d_n6: f64 = (-(((nv8 - nv2) * locals.var_rsd_dn6) / (locals.var_rsd * locals.var_rsd)));
        let eq26_e1156_d_n7: f64 = (-(((nv8 - nv2) * locals.var_rsd_dn7) / (locals.var_rsd * locals.var_rsd)));
        let eq26_e1156_d_n8: f64 = ((locals.var_rsd - ((nv8 - nv2) * locals.var_rsd_dn8)) / (locals.var_rsd * locals.var_rsd));
        let eq26_e1156_d_n9: f64 = (-(((nv8 - nv2) * locals.var_rsd_dn9) / (locals.var_rsd * locals.var_rsd)));
        let eq26_e1156_d_n10: f64 = (-(((nv8 - nv2) * locals.var_rsd_dn10) / (locals.var_rsd * locals.var_rsd)));
        let eq26_e1156_d_n11: f64 = (-(((nv8 - nv2) * locals.var_rsd_dn11) / (locals.var_rsd * locals.var_rsd)));
        let eq26_e1156_d_n14: f64 = (-(((nv8 - nv2) * locals.var_rsd_dn14) / (locals.var_rsd * locals.var_rsd)));
        (eq26_e1156, eq26_e1156_d_n0, eq26_e1156_d_n2, eq26_e1156_d_n4, eq26_e1156_d_n5, eq26_e1156_d_n6, eq26_e1156_d_n7, eq26_e1156_d_n8, eq26_e1156_d_n9, eq26_e1156_d_n10, eq26_e1156_d_n11, eq26_e1156_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1158;
        let eq26_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq26_node_derivatives: [f64; 11] = [eq26_e1158_d_n0, eq26_e1158_d_n2, eq26_e1158_d_n4, eq26_e1158_d_n5, eq26_e1158_d_n6, eq26_e1158_d_n7, eq26_e1158_d_n8, eq26_e1158_d_n9, eq26_e1158_d_n10, eq26_e1158_d_n11, eq26_e1158_d_n14];
        let eq26_branch_derivative_indices: [usize; 0] = [];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq26_value),
            &eq26_node_derivative_indices,
            &eq26_node_derivatives,
            &eq26_branch_derivative_indices,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let eq28_e1167: f64 = (locals.var_qg + locals.var_qg_nqs);
        let eq28_e1168: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq28_e1167);
        let eq28_e1169: f64 = (p.p87 * eq28_e1168);
        let eq28_e1169_d_n0: f64 = (p.p87 * (locals.var_qg_dn0 * ddt_scale));
        let eq28_e1169_d_n2: f64 = (p.p87 * (locals.var_qg_dn2 * ddt_scale));
        let eq28_e1169_d_n4: f64 = (p.p87 * (locals.var_qg_dn4 * ddt_scale));
        let eq28_e1169_d_n5: f64 = (p.p87 * (locals.var_qg_dn5 * ddt_scale));
        let eq28_e1169_d_n6: f64 = (p.p87 * (locals.var_qg_dn6 * ddt_scale));
        let eq28_e1169_d_n7: f64 = (p.p87 * (locals.var_qg_dn7 * ddt_scale));
        let eq28_e1169_d_n8: f64 = (p.p87 * (locals.var_qg_dn8 * ddt_scale));
        let eq28_e1169_d_n9: f64 = (p.p87 * (locals.var_qg_dn9 * ddt_scale));
        let eq28_e1169_d_n10: f64 = (p.p87 * (locals.var_qg_dn10 * ddt_scale));
        let eq28_e1169_d_n11: f64 = (p.p87 * (locals.var_qg_dn11 * ddt_scale));
        let eq28_e1169_d_n12: f64 = (p.p87 * (locals.var_qg_nqs_dn12 * ddt_scale));
        let eq28_e1169_d_n13: f64 = (p.p87 * (locals.var_qg_nqs_dn13 * ddt_scale));
        let eq28_e1169_d_n14: f64 = (p.p87 * (locals.var_qg_dn14 * ddt_scale));
        let eq28_value: f64 = eq28_e1169;
        let eq28_node_derivative_indices: [usize; 13] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq28_node_derivatives: [f64; 13] = [eq28_e1169_d_n0, eq28_e1169_d_n2, eq28_e1169_d_n4, eq28_e1169_d_n5, eq28_e1169_d_n6, eq28_e1169_d_n7, eq28_e1169_d_n8, eq28_e1169_d_n9, eq28_e1169_d_n10, eq28_e1169_d_n11, eq28_e1169_d_n12, eq28_e1169_d_n13, eq28_e1169_d_n14];
        let eq28_branch_derivative_indices: [usize; 0] = [];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq28_value),
            &eq28_node_derivative_indices,
            &eq28_node_derivatives,
            &eq28_branch_derivative_indices,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let eq29_e1173: f64 = (locals.var_qd + locals.var_qd_nqs);
        let eq29_e1173_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);
        let eq29_e1173_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);
        let eq29_e1173_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);
        let eq29_e1173_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);
        let eq29_e1173_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);
        let eq29_e1173_d_n7: f64 = (locals.var_qd_dn7 + locals.var_qd_nqs_dn7);
        let eq29_e1173_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);
        let eq29_e1173_d_n9: f64 = (locals.var_qd_dn9 + locals.var_qd_nqs_dn9);
        let eq29_e1173_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);
        let eq29_e1173_d_n11: f64 = (locals.var_qd_dn11 + locals.var_qd_nqs_dn11);
        let eq29_e1173_d_n14: f64 = (locals.var_qd_dn14 + locals.var_qd_nqs_dn14);
        let eq29_e1174: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq29_e1173);
        let eq29_e1175: f64 = (p.p87 * eq29_e1174);
        let eq29_e1175_d_n0: f64 = (p.p87 * (eq29_e1173_d_n0 * ddt_scale));
        let eq29_e1175_d_n2: f64 = (p.p87 * (eq29_e1173_d_n2 * ddt_scale));
        let eq29_e1175_d_n4: f64 = (p.p87 * (eq29_e1173_d_n4 * ddt_scale));
        let eq29_e1175_d_n5: f64 = (p.p87 * (eq29_e1173_d_n5 * ddt_scale));
        let eq29_e1175_d_n6: f64 = (p.p87 * (eq29_e1173_d_n6 * ddt_scale));
        let eq29_e1175_d_n7: f64 = (p.p87 * (eq29_e1173_d_n7 * ddt_scale));
        let eq29_e1175_d_n8: f64 = (p.p87 * (eq29_e1173_d_n8 * ddt_scale));
        let eq29_e1175_d_n9: f64 = (p.p87 * (eq29_e1173_d_n9 * ddt_scale));
        let eq29_e1175_d_n10: f64 = (p.p87 * (eq29_e1173_d_n10 * ddt_scale));
        let eq29_e1175_d_n11: f64 = (p.p87 * (eq29_e1173_d_n11 * ddt_scale));
        let eq29_e1175_d_n12: f64 = (p.p87 * (locals.var_qd_nqs_dn12 * ddt_scale));
        let eq29_e1175_d_n14: f64 = (p.p87 * (eq29_e1173_d_n14 * ddt_scale));
        let eq29_value: f64 = eq29_e1175;
        let eq29_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14];
        let eq29_node_derivatives: [f64; 12] = [eq29_e1175_d_n0, eq29_e1175_d_n2, eq29_e1175_d_n4, eq29_e1175_d_n5, eq29_e1175_d_n6, eq29_e1175_d_n7, eq29_e1175_d_n8, eq29_e1175_d_n9, eq29_e1175_d_n10, eq29_e1175_d_n11, eq29_e1175_d_n12, eq29_e1175_d_n14];
        let eq29_branch_derivative_indices: [usize; 0] = [];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq29_value),
            &eq29_node_derivative_indices,
            &eq29_node_derivatives,
            &eq29_branch_derivative_indices,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let eq30_e1180: f64 = (locals.var_qg_nqs + locals.var_qd_nqs);
        let eq30_e1180_d_n12: f64 = (locals.var_qg_nqs_dn12 + locals.var_qd_nqs_dn12);
        let eq30_e1182: f64 = (eq30_e1180 + locals.var_qs_nqs);
        let eq30_e1182_d_n0: f64 = (locals.var_qd_nqs_dn0 + locals.var_qs_nqs_dn0);
        let eq30_e1182_d_n2: f64 = (locals.var_qd_nqs_dn2 + locals.var_qs_nqs_dn2);
        let eq30_e1182_d_n4: f64 = (locals.var_qd_nqs_dn4 + locals.var_qs_nqs_dn4);
        let eq30_e1182_d_n5: f64 = (locals.var_qd_nqs_dn5 + locals.var_qs_nqs_dn5);
        let eq30_e1182_d_n6: f64 = (locals.var_qd_nqs_dn6 + locals.var_qs_nqs_dn6);
        let eq30_e1182_d_n7: f64 = (locals.var_qd_nqs_dn7 + locals.var_qs_nqs_dn7);
        let eq30_e1182_d_n8: f64 = (locals.var_qd_nqs_dn8 + locals.var_qs_nqs_dn8);
        let eq30_e1182_d_n9: f64 = (locals.var_qd_nqs_dn9 + locals.var_qs_nqs_dn9);
        let eq30_e1182_d_n10: f64 = (locals.var_qd_nqs_dn10 + locals.var_qs_nqs_dn10);
        let eq30_e1182_d_n11: f64 = (locals.var_qd_nqs_dn11 + locals.var_qs_nqs_dn11);
        let eq30_e1182_d_n12: f64 = (eq30_e1180_d_n12 + locals.var_qs_nqs_dn12);
        let eq30_e1182_d_n14: f64 = (locals.var_qd_nqs_dn14 + locals.var_qs_nqs_dn14);
        let eq30_e1183: f64 = (locals.var_qb - eq30_e1182);
        let eq30_e1183_d_n0: f64 = (locals.var_qb_dn0 - eq30_e1182_d_n0);
        let eq30_e1183_d_n2: f64 = (locals.var_qb_dn2 - eq30_e1182_d_n2);
        let eq30_e1183_d_n4: f64 = (locals.var_qb_dn4 - eq30_e1182_d_n4);
        let eq30_e1183_d_n5: f64 = (locals.var_qb_dn5 - eq30_e1182_d_n5);
        let eq30_e1183_d_n6: f64 = (locals.var_qb_dn6 - eq30_e1182_d_n6);
        let eq30_e1183_d_n7: f64 = (locals.var_qb_dn7 - eq30_e1182_d_n7);
        let eq30_e1183_d_n8: f64 = (locals.var_qb_dn8 - eq30_e1182_d_n8);
        let eq30_e1183_d_n9: f64 = (locals.var_qb_dn9 - eq30_e1182_d_n9);
        let eq30_e1183_d_n10: f64 = (locals.var_qb_dn10 - eq30_e1182_d_n10);
        let eq30_e1183_d_n11: f64 = (locals.var_qb_dn11 - eq30_e1182_d_n11);
        let eq30_e1183_d_n14: f64 = (locals.var_qb_dn14 - eq30_e1182_d_n14);
        let eq30_e1184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq30_e1183);
        let eq30_e1185: f64 = (p.p87 * eq30_e1184);
        let eq30_e1185_d_n0: f64 = (p.p87 * (eq30_e1183_d_n0 * ddt_scale));
        let eq30_e1185_d_n2: f64 = (p.p87 * (eq30_e1183_d_n2 * ddt_scale));
        let eq30_e1185_d_n4: f64 = (p.p87 * (eq30_e1183_d_n4 * ddt_scale));
        let eq30_e1185_d_n5: f64 = (p.p87 * (eq30_e1183_d_n5 * ddt_scale));
        let eq30_e1185_d_n6: f64 = (p.p87 * (eq30_e1183_d_n6 * ddt_scale));
        let eq30_e1185_d_n7: f64 = (p.p87 * (eq30_e1183_d_n7 * ddt_scale));
        let eq30_e1185_d_n8: f64 = (p.p87 * (eq30_e1183_d_n8 * ddt_scale));
        let eq30_e1185_d_n9: f64 = (p.p87 * (eq30_e1183_d_n9 * ddt_scale));
        let eq30_e1185_d_n10: f64 = (p.p87 * (eq30_e1183_d_n10 * ddt_scale));
        let eq30_e1185_d_n11: f64 = (p.p87 * (eq30_e1183_d_n11 * ddt_scale));
        let eq30_e1185_d_n12: f64 = (p.p87 * ((-eq30_e1182_d_n12) * ddt_scale));
        let eq30_e1185_d_n13: f64 = (p.p87 * ((-locals.var_qg_nqs_dn13) * ddt_scale));
        let eq30_e1185_d_n14: f64 = (p.p87 * (eq30_e1183_d_n14 * ddt_scale));
        let eq30_value: f64 = eq30_e1185;
        let eq30_node_derivative_indices: [usize; 13] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq30_node_derivatives: [f64; 13] = [eq30_e1185_d_n0, eq30_e1185_d_n2, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14];
        let eq30_branch_derivative_indices: [usize; 0] = [];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq30_value),
            &eq30_node_derivative_indices,
            &eq30_node_derivatives,
            &eq30_branch_derivative_indices,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, locals.var_qgext);
        let eq31_e1189: f64 = (p.p87 * eq31_e1188);
        let eq31_e1189_d_n0: f64 = (p.p87 * (locals.var_qgext_dn0 * ddt_scale));
        let eq31_e1189_d_n2: f64 = (p.p87 * (locals.var_qgext_dn2 * ddt_scale));
        let eq31_e1189_d_n4: f64 = (p.p87 * (locals.var_qgext_dn4 * ddt_scale));
        let eq31_e1189_d_n5: f64 = (p.p87 * (locals.var_qgext_dn5 * ddt_scale));
        let eq31_e1189_d_n6: f64 = (p.p87 * (locals.var_qgext_dn6 * ddt_scale));
        let eq31_e1189_d_n7: f64 = (p.p87 * (locals.var_qgext_dn7 * ddt_scale));
        let eq31_e1189_d_n8: f64 = (p.p87 * (locals.var_qgext_dn8 * ddt_scale));
        let eq31_e1189_d_n9: f64 = (p.p87 * (locals.var_qgext_dn9 * ddt_scale));
        let eq31_e1189_d_n10: f64 = (p.p87 * (locals.var_qgext_dn10 * ddt_scale));
        let eq31_e1189_d_n11: f64 = (p.p87 * (locals.var_qgext_dn11 * ddt_scale));
        let eq31_e1189_d_n14: f64 = (p.p87 * (locals.var_qgext_dn14 * ddt_scale));
        let eq31_value: f64 = eq31_e1189;
        let eq31_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq31_node_derivatives: [f64; 11] = [eq31_e1189_d_n0, eq31_e1189_d_n2, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, eq31_e1189_d_n14];
        let eq31_branch_derivative_indices: [usize; 0] = [];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq31_value),
            &eq31_node_derivative_indices,
            &eq31_node_derivatives,
            &eq31_branch_derivative_indices,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, locals.var_qdext);
        let eq32_e1193: f64 = (p.p87 * eq32_e1192);
        let eq32_e1193_d_n0: f64 = (p.p87 * (locals.var_qdext_dn0 * ddt_scale));
        let eq32_e1193_d_n2: f64 = (p.p87 * (locals.var_qdext_dn2 * ddt_scale));
        let eq32_e1193_d_n4: f64 = (p.p87 * (locals.var_qdext_dn4 * ddt_scale));
        let eq32_e1193_d_n5: f64 = (p.p87 * (locals.var_qdext_dn5 * ddt_scale));
        let eq32_e1193_d_n6: f64 = (p.p87 * (locals.var_qdext_dn6 * ddt_scale));
        let eq32_e1193_d_n7: f64 = (p.p87 * (locals.var_qdext_dn7 * ddt_scale));
        let eq32_e1193_d_n8: f64 = (p.p87 * (locals.var_qdext_dn8 * ddt_scale));
        let eq32_e1193_d_n9: f64 = (p.p87 * (locals.var_qdext_dn9 * ddt_scale));
        let eq32_e1193_d_n10: f64 = (p.p87 * (locals.var_qdext_dn10 * ddt_scale));
        let eq32_e1193_d_n11: f64 = (p.p87 * (locals.var_qdext_dn11 * ddt_scale));
        let eq32_e1193_d_n14: f64 = (p.p87 * (locals.var_qdext_dn14 * ddt_scale));
        let eq32_value: f64 = eq32_e1193;
        let eq32_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq32_node_derivatives: [f64; 11] = [eq32_e1193_d_n0, eq32_e1193_d_n2, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, eq32_e1193_d_n14];
        let eq32_branch_derivative_indices: [usize; 0] = [];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq32_value),
            &eq32_node_derivative_indices,
            &eq32_node_derivatives,
            &eq32_branch_derivative_indices,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e1196: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, locals.var_qbext);
        let eq33_e1197: f64 = (p.p87 * eq33_e1196);
        let eq33_e1197_d_n0: f64 = (p.p87 * (locals.var_qbext_dn0 * ddt_scale));
        let eq33_e1197_d_n2: f64 = (p.p87 * (locals.var_qbext_dn2 * ddt_scale));
        let eq33_e1197_d_n4: f64 = (p.p87 * (locals.var_qbext_dn4 * ddt_scale));
        let eq33_e1197_d_n5: f64 = (p.p87 * (locals.var_qbext_dn5 * ddt_scale));
        let eq33_e1197_d_n6: f64 = (p.p87 * (locals.var_qbext_dn6 * ddt_scale));
        let eq33_e1197_d_n7: f64 = (p.p87 * (locals.var_qbext_dn7 * ddt_scale));
        let eq33_e1197_d_n8: f64 = (p.p87 * (locals.var_qbext_dn8 * ddt_scale));
        let eq33_e1197_d_n9: f64 = (p.p87 * (locals.var_qbext_dn9 * ddt_scale));
        let eq33_e1197_d_n10: f64 = (p.p87 * (locals.var_qbext_dn10 * ddt_scale));
        let eq33_e1197_d_n11: f64 = (p.p87 * (locals.var_qbext_dn11 * ddt_scale));
        let eq33_e1197_d_n14: f64 = (p.p87 * (locals.var_qbext_dn14 * ddt_scale));
        let eq33_value: f64 = eq33_e1197;
        let eq33_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq33_node_derivatives: [f64; 11] = [eq33_e1197_d_n0, eq33_e1197_d_n2, eq33_e1197_d_n4, eq33_e1197_d_n5, eq33_e1197_d_n6, eq33_e1197_d_n7, eq33_e1197_d_n8, eq33_e1197_d_n9, eq33_e1197_d_n10, eq33_e1197_d_n11, eq33_e1197_d_n14];
        let eq33_branch_derivative_indices: [usize; 0] = [];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq33_value),
            &eq33_node_derivative_indices,
            &eq33_node_derivatives,
            &eq33_branch_derivative_indices,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e1199: f64 = (-p.p87);
        let eq34_e1201: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, locals.var_qfd);
        let eq34_e1202: f64 = (eq34_e1199 * eq34_e1201);
        let eq34_e1202_d_n0: f64 = (eq34_e1199 * (locals.var_qfd_dn0 * ddt_scale));
        let eq34_e1202_d_n2: f64 = (eq34_e1199 * (locals.var_qfd_dn2 * ddt_scale));
        let eq34_e1202_d_n7: f64 = (eq34_e1199 * (locals.var_qfd_dn7 * ddt_scale));
        let eq34_value: f64 = eq34_e1202;
        stamper.stamp_current_node3_local(
            Some(7),
            Some(0),
            multiplicity * (eq34_value),
            0,
            multiplicity * (eq34_e1202_d_n0),
            2,
            multiplicity * (eq34_e1202_d_n2),
            7,
            multiplicity * (eq34_e1202_d_n7),
        );
        let eq35_e1204: f64 = (-p.p87);
        let eq35_e1206: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, locals.var_qfs);
        let eq35_e1207: f64 = (eq35_e1204 * eq35_e1206);
        let eq35_e1207_d_n2: f64 = (eq35_e1204 * (locals.var_qfs_dn2 * ddt_scale));
        let eq35_e1207_d_n7: f64 = (eq35_e1204 * (locals.var_qfs_dn7 * ddt_scale));
        let eq35_value: f64 = eq35_e1207;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(2),
            multiplicity * (eq35_value),
            2,
            multiplicity * (eq35_e1207_d_n2),
            7,
            multiplicity * (eq35_e1207_d_n7),
        );
    }
    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        locals: &mut StampLocals,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq40_e1233: f64 = (locals.var_ci * (nv15 - 0.0));
        let eq40_e1233_d_n0: f64 = (locals.var_ci_dn0 * (nv15 - 0.0));
        let eq40_e1233_d_n2: f64 = (locals.var_ci_dn2 * (nv15 - 0.0));
        let eq40_e1233_d_n4: f64 = (locals.var_ci_dn4 * (nv15 - 0.0));
        let eq40_e1233_d_n5: f64 = (locals.var_ci_dn5 * (nv15 - 0.0));
        let eq40_e1233_d_n6: f64 = (locals.var_ci_dn6 * (nv15 - 0.0));
        let eq40_e1233_d_n7: f64 = (locals.var_ci_dn7 * (nv15 - 0.0));
        let eq40_e1233_d_n8: f64 = (locals.var_ci_dn8 * (nv15 - 0.0));
        let eq40_e1233_d_n9: f64 = (locals.var_ci_dn9 * (nv15 - 0.0));
        let eq40_e1233_d_n10: f64 = (locals.var_ci_dn10 * (nv15 - 0.0));
        let eq40_e1233_d_n11: f64 = (locals.var_ci_dn11 * (nv15 - 0.0));
        let eq40_e1233_d_n14: f64 = (locals.var_ci_dn14 * (nv15 - 0.0));
        let eq40_value: f64 = eq40_e1233;
        let eq40_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];
        let eq40_node_derivatives: [f64; 12] = [eq40_e1233_d_n0, eq40_e1233_d_n2, eq40_e1233_d_n4, eq40_e1233_d_n5, eq40_e1233_d_n6, eq40_e1233_d_n7, eq40_e1233_d_n8, eq40_e1233_d_n9, eq40_e1233_d_n10, eq40_e1233_d_n11, eq40_e1233_d_n14, locals.var_ci];
        let eq40_branch_derivative_indices: [usize; 0] = [];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq40_value),
            &eq40_node_derivative_indices,
            &eq40_node_derivatives,
            &eq40_branch_derivative_indices,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e1236: f64 = ((nv15 - 0.0) * locals.var_sigrat_s);
        let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn0);
        let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn2);
        let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn4);
        let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn5);
        let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn6);
        let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn7);
        let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn8);
        let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn9);
        let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn10);
        let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn11);
        let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn14);
        let eq41_e1237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq41_e1236);
        let eq41_value: f64 = eq41_e1237;
        let eq41_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];
        let eq41_node_derivatives: [f64; 12] = [(eq41_e1236_d_n0 * ddt_scale), (eq41_e1236_d_n2 * ddt_scale), (eq41_e1236_d_n4 * ddt_scale), (eq41_e1236_d_n5 * ddt_scale), (eq41_e1236_d_n6 * ddt_scale), (eq41_e1236_d_n7 * ddt_scale), (eq41_e1236_d_n8 * ddt_scale), (eq41_e1236_d_n9 * ddt_scale), (eq41_e1236_d_n10 * ddt_scale), (eq41_e1236_d_n11 * ddt_scale), (eq41_e1236_d_n14 * ddt_scale), (locals.var_sigrat_s * ddt_scale)];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq41_value),
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e1240: f64 = ((nv15 - 0.0) * locals.var_sigrat_d);
        let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn0);
        let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn2);
        let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn4);
        let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn5);
        let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn6);
        let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn7);
        let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn8);
        let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn9);
        let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn10);
        let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn11);
        let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn14);
        let eq42_e1241: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq42_e1240);
        let eq42_value: f64 = eq42_e1241;
        let eq42_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];
        let eq42_node_derivatives: [f64; 12] = [(eq42_e1240_d_n0 * ddt_scale), (eq42_e1240_d_n2 * ddt_scale), (eq42_e1240_d_n4 * ddt_scale), (eq42_e1240_d_n5 * ddt_scale), (eq42_e1240_d_n6 * ddt_scale), (eq42_e1240_d_n7 * ddt_scale), (eq42_e1240_d_n8 * ddt_scale), (eq42_e1240_d_n9 * ddt_scale), (eq42_e1240_d_n10 * ddt_scale), (eq42_e1240_d_n11 * ddt_scale), (eq42_e1240_d_n14 * ddt_scale), (locals.var_sigrat_d * ddt_scale)];
        let eq42_branch_derivative_indices: [usize; 0] = [];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivative_indices,
            &eq42_node_derivatives,
            &eq42_branch_derivative_indices,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq57_e1336, eq57_e1336_d_n0, eq57_e1336_d_n2, eq57_e1336_d_n4, eq57_e1336_d_n5, eq57_e1336_d_n6, eq57_e1336_d_n7, eq57_e1336_d_n8, eq57_e1336_d_n9, eq57_e1336_d_n10, eq57_e1336_d_n11, eq57_e1336_d_n14,) = {
    if (locals.var_guard2415 != 0.0) {
        let eq57_e1334: f64 = (-locals.var_itemp);
        (eq57_e1334, (-locals.var_itemp_dn0), (-locals.var_itemp_dn2), (-locals.var_itemp_dn4), (-locals.var_itemp_dn5), (-locals.var_itemp_dn6), (-locals.var_itemp_dn7), (-locals.var_itemp_dn8), (-locals.var_itemp_dn9), (-locals.var_itemp_dn10), (-locals.var_itemp_dn11), (-locals.var_itemp_dn14),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1336;
        let eq57_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq57_node_derivatives: [f64; 11] = [eq57_e1336_d_n0, eq57_e1336_d_n2, eq57_e1336_d_n4, eq57_e1336_d_n5, eq57_e1336_d_n6, eq57_e1336_d_n7, eq57_e1336_d_n8, eq57_e1336_d_n9, eq57_e1336_d_n10, eq57_e1336_d_n11, eq57_e1336_d_n14];
        let eq57_branch_derivative_indices: [usize; 0] = [];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            None,
            multiplicity * (eq57_value),
            &eq57_node_derivative_indices,
            &eq57_node_derivatives,
            &eq57_branch_derivative_indices,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq60_e1351, eq60_e1351_d_n0, eq60_e1351_d_n2, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n14,) = {
    if (p.p28 != 0.0) {
        (locals.var_iqi_nqs, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn4, locals.var_iqi_nqs_dn5, locals.var_iqi_nqs_dn6, locals.var_iqi_nqs_dn7, locals.var_iqi_nqs_dn8, locals.var_iqi_nqs_dn9, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn12, locals.var_iqi_nqs_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1351;
        let eq60_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14];
        let eq60_node_derivatives: [f64; 12] = [eq60_e1351_d_n0, eq60_e1351_d_n2, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n14];
        let eq60_branch_derivative_indices: [usize; 0] = [];
        let eq60_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            None,
            multiplicity * (eq60_value),
            &eq60_node_derivative_indices,
            &eq60_node_derivatives,
            &eq60_branch_derivative_indices,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1355, eq61_e1355_d_n0, eq61_e1355_d_n2, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n13, eq61_e1355_d_n14,) = {
    if (p.p28 != 0.0) {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn4, locals.var_iqb_nqs_dn5, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn8, locals.var_iqb_nqs_dn9, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn13, locals.var_iqb_nqs_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1355;
        let eq61_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq61_node_derivatives: [f64; 12] = [eq61_e1355_d_n0, eq61_e1355_d_n2, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n13, eq61_e1355_d_n14];
        let eq61_branch_derivative_indices: [usize; 0] = [];
        let eq61_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivative_indices,
            &eq61_node_derivatives,
            &eq61_branch_derivative_indices,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1362, eq62_e1362_d_n12,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (locals.var_cqi * (nv12 - 0.0));
        let eq62_e1360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq62_e1359);
        (eq62_e1360, (locals.var_cqi * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1362;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (eq62_value),
            12,
            multiplicity * (eq62_e1362_d_n12),
        );
        let (eq63_e1369, eq63_e1369_d_n13,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (locals.var_cqb * (nv13 - 0.0));
        let eq63_e1367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq63_e1366);
        (eq63_e1367, (locals.var_cqb * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1369;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            13,
            multiplicity * (eq63_e1369_d_n13),
        );
        let (eq66_e1383, eq66_e1383_d_n0, eq66_e1383_d_n2, eq66_e1383_d_n4, eq66_e1383_d_n5, eq66_e1383_d_n6, eq66_e1383_d_n7, eq66_e1383_d_n8, eq66_e1383_d_n9, eq66_e1383_d_n10, eq66_e1383_d_n11, eq66_e1383_d_n14,) = {
    if (p.p29 != 0.0) {
        (locals.var_ibd_nqs, locals.var_ibd_nqs_dn0, locals.var_ibd_nqs_dn2, locals.var_ibd_nqs_dn4, locals.var_ibd_nqs_dn5, locals.var_ibd_nqs_dn6, locals.var_ibd_nqs_dn7, locals.var_ibd_nqs_dn8, locals.var_ibd_nqs_dn9, locals.var_ibd_nqs_dn10, locals.var_ibd_nqs_dn11, locals.var_ibd_nqs_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1383;
        let eq66_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq66_node_derivatives: [f64; 11] = [eq66_e1383_d_n0, eq66_e1383_d_n2, eq66_e1383_d_n4, eq66_e1383_d_n5, eq66_e1383_d_n6, eq66_e1383_d_n7, eq66_e1383_d_n8, eq66_e1383_d_n9, eq66_e1383_d_n10, eq66_e1383_d_n11, eq66_e1383_d_n14];
        let eq66_branch_derivative_indices: [usize; 0] = [];
        let eq66_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            None,
            multiplicity * (eq66_value),
            &eq66_node_derivative_indices,
            &eq66_node_derivatives,
            &eq66_branch_derivative_indices,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let (eq67_e1388, eq67_e1388_d_n14,) = {
    if (p.p29 != 0.0) {
        let eq67_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, (nv14 - 0.0));
        (eq67_e1386, ddt_scale,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e1388;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq67_value),
            14,
            multiplicity * (eq67_e1388_d_n14),
        );
    }
    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq1_e1022, eq1_e1022_d_n0, eq1_e1022_d_n2, eq1_e1022_d_n4, eq1_e1022_d_n5, eq1_e1022_d_n6, eq1_e1022_d_n7, eq1_e1022_d_n8, eq1_e1022_d_n9, eq1_e1022_d_n10, eq1_e1022_d_n11, eq1_e1022_d_n14, eq1_e1022_d_n16, eq1_e1022_q, eq1_e1022_q_d_n16,) = {
    if (locals.var_guard2311 != 0.0) {
        let eq1_e1019_q: f64 = locals.var_q_nqs_a;
        let eq1_e1020: f64 = (locals.var_inqs0_a + locals.var_q_nqs_a);
        let eq1_e1020_d_n16: f64 = (locals.var_inqs0_a_dn16 + locals.var_q_nqs_a_dn16);
        let eq1_e1020_q: f64 = eq1_e1019_q;
        (eq1_e1020, locals.var_inqs0_a_dn0, locals.var_inqs0_a_dn2, locals.var_inqs0_a_dn4, locals.var_inqs0_a_dn5, locals.var_inqs0_a_dn6, locals.var_inqs0_a_dn7, locals.var_inqs0_a_dn8, locals.var_inqs0_a_dn9, locals.var_inqs0_a_dn10, locals.var_inqs0_a_dn11, locals.var_inqs0_a_dn14, eq1_e1020_d_n16, eq1_e1020_q, locals.var_q_nqs_a_dn16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq1_e1022_q_d_n16),
        );
        let (eq2_e1029, eq2_e1029_d_n0, eq2_e1029_d_n2, eq2_e1029_d_n4, eq2_e1029_d_n5, eq2_e1029_d_n6, eq2_e1029_d_n7, eq2_e1029_d_n8, eq2_e1029_d_n9, eq2_e1029_d_n10, eq2_e1029_d_n11, eq2_e1029_d_n14, eq2_e1029_d_n17, eq2_e1029_q, eq2_e1029_q_d_n17,) = {
    if (locals.var_guard2311 != 0.0) {
        let eq2_e1026_q: f64 = locals.var_q_nqs_k;
        let eq2_e1027: f64 = (locals.var_inqs0_k + locals.var_q_nqs_k);
        let eq2_e1027_d_n17: f64 = (locals.var_inqs0_k_dn17 + locals.var_q_nqs_k_dn17);
        let eq2_e1027_q: f64 = eq2_e1026_q;
        (eq2_e1027, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn11, locals.var_inqs0_k_dn14, eq2_e1027_d_n17, eq2_e1027_q, locals.var_q_nqs_k_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq2_e1029_q_d_n17),
        );
        let (eq5_e1046, eq5_e1046_d_n0, eq5_e1046_d_n2, eq5_e1046_d_n4, eq5_e1046_d_n5, eq5_e1046_d_n6, eq5_e1046_d_n7, eq5_e1046_d_n8, eq5_e1046_d_n9, eq5_e1046_d_n10, eq5_e1046_d_n11, eq5_e1046_d_n14, eq5_e1046_d_n18, eq5_e1046_q, eq5_e1046_q_d_n18,) = {
    if (locals.var_guard2312 != 0.0) {
        let eq5_e1043_q: f64 = locals.var_w_nqs_a;
        let eq5_e1044: f64 = (locals.var_iwnqs0_a + locals.var_w_nqs_a);
        let eq5_e1044_d_n18: f64 = (locals.var_iwnqs0_a_dn18 + locals.var_w_nqs_a_dn18);
        let eq5_e1044_q: f64 = eq5_e1043_q;
        (eq5_e1044, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn11, locals.var_iwnqs0_a_dn14, eq5_e1044_d_n18, eq5_e1044_q, locals.var_w_nqs_a_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq5_e1046_q_d_n18),
        );
        let eq15_e1092_q: f64 = locals.var_qbs;
        let eq15_e1093: f64 = (p.p87 * locals.var_qbs);
        let eq15_e1093_d_n0: f64 = (p.p87 * locals.var_qbs_dn0);
        let eq15_e1093_d_n2: f64 = (p.p87 * locals.var_qbs_dn2);
        let eq15_e1093_d_n4: f64 = (p.p87 * locals.var_qbs_dn4);
        let eq15_e1093_d_n5: f64 = (p.p87 * locals.var_qbs_dn5);
        let eq15_e1093_d_n6: f64 = (p.p87 * locals.var_qbs_dn6);
        let eq15_e1093_d_n7: f64 = (p.p87 * locals.var_qbs_dn7);
        let eq15_e1093_d_n8: f64 = (p.p87 * locals.var_qbs_dn8);
        let eq15_e1093_d_n9: f64 = (p.p87 * locals.var_qbs_dn9);
        let eq15_e1093_d_n10: f64 = (p.p87 * locals.var_qbs_dn10);
        let eq15_e1093_d_n11: f64 = (p.p87 * locals.var_qbs_dn11);
        let eq15_e1093_d_n14: f64 = (p.p87 * locals.var_qbs_dn14);
        let eq15_e1093_q: f64 = (p.p87 * eq15_e1092_q);
        let eq15_reactive_node_derivatives: [f64; 19] = [eq15_e1093_d_n0, 0.0, eq15_e1093_d_n2, 0.0, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n11, 0.0, 0.0, eq15_e1093_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq15_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[2]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e1096_q: f64 = locals.var_qbd;
        let eq16_e1097: f64 = (p.p87 * locals.var_qbd);
        let eq16_e1097_d_n0: f64 = (p.p87 * locals.var_qbd_dn0);
        let eq16_e1097_d_n2: f64 = (p.p87 * locals.var_qbd_dn2);
        let eq16_e1097_d_n4: f64 = (p.p87 * locals.var_qbd_dn4);
        let eq16_e1097_d_n5: f64 = (p.p87 * locals.var_qbd_dn5);
        let eq16_e1097_d_n6: f64 = (p.p87 * locals.var_qbd_dn6);
        let eq16_e1097_d_n7: f64 = (p.p87 * locals.var_qbd_dn7);
        let eq16_e1097_d_n8: f64 = (p.p87 * locals.var_qbd_dn8);
        let eq16_e1097_d_n9: f64 = (p.p87 * locals.var_qbd_dn9);
        let eq16_e1097_d_n10: f64 = (p.p87 * locals.var_qbd_dn10);
        let eq16_e1097_d_n11: f64 = (p.p87 * locals.var_qbd_dn11);
        let eq16_e1097_d_n14: f64 = (p.p87 * locals.var_qbd_dn14);
        let eq16_e1097_d_n16: f64 = (p.p87 * locals.var_qbd_dn16);
        let eq16_e1097_d_n17: f64 = (p.p87 * locals.var_qbd_dn17);
        let eq16_e1097_d_n18: f64 = (p.p87 * locals.var_qbd_dn18);
        let eq16_e1097_q: f64 = (p.p87 * eq16_e1096_q);
        let eq16_reactive_node_derivatives: [f64; 19] = [eq16_e1097_d_n0, 0.0, eq16_e1097_d_n2, 0.0, eq16_e1097_d_n4, eq16_e1097_d_n5, eq16_e1097_d_n6, eq16_e1097_d_n7, eq16_e1097_d_n8, eq16_e1097_d_n9, eq16_e1097_d_n10, eq16_e1097_d_n11, 0.0, 0.0, eq16_e1097_d_n14, 0.0, eq16_e1097_d_n16, eq16_e1097_d_n17, eq16_e1097_d_n18];
        let eq16_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1116, eq19_e1116_d_n0, eq19_e1116_d_n2, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, eq19_e1116_d_n14, eq19_e1116_q,) = {
    if (locals.var_guard2411 != 0.0) {
        let eq19_e1113_q: f64 = locals.var_qbsi;
        let eq19_e1114: f64 = (p.p87 * locals.var_qbsi);
        let eq19_e1114_d_n0: f64 = (p.p87 * locals.var_qbsi_dn0);
        let eq19_e1114_d_n2: f64 = (p.p87 * locals.var_qbsi_dn2);
        let eq19_e1114_d_n4: f64 = (p.p87 * locals.var_qbsi_dn4);
        let eq19_e1114_d_n5: f64 = (p.p87 * locals.var_qbsi_dn5);
        let eq19_e1114_d_n6: f64 = (p.p87 * locals.var_qbsi_dn6);
        let eq19_e1114_d_n7: f64 = (p.p87 * locals.var_qbsi_dn7);
        let eq19_e1114_d_n8: f64 = (p.p87 * locals.var_qbsi_dn8);
        let eq19_e1114_d_n9: f64 = (p.p87 * locals.var_qbsi_dn9);
        let eq19_e1114_d_n10: f64 = (p.p87 * locals.var_qbsi_dn10);
        let eq19_e1114_d_n11: f64 = (p.p87 * locals.var_qbsi_dn11);
        let eq19_e1114_d_n14: f64 = (p.p87 * locals.var_qbsi_dn14);
        let eq19_e1114_q: f64 = (p.p87 * eq19_e1113_q);
        (eq19_e1114, eq19_e1114_d_n0, eq19_e1114_d_n2, eq19_e1114_d_n4, eq19_e1114_d_n5, eq19_e1114_d_n6, eq19_e1114_d_n7, eq19_e1114_d_n8, eq19_e1114_d_n9, eq19_e1114_d_n10, eq19_e1114_d_n11, eq19_e1114_d_n14, eq19_e1114_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e1116_d_n0, 0.0, eq19_e1116_d_n2, 0.0, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, 0.0, 0.0, eq19_e1116_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq19_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n2, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n14, eq20_e1123_q,) = {
    if (locals.var_guard2411 != 0.0) {
        let eq20_e1120_q: f64 = locals.var_qbdi;
        let eq20_e1121: f64 = (p.p87 * locals.var_qbdi);
        let eq20_e1121_d_n0: f64 = (p.p87 * locals.var_qbdi_dn0);
        let eq20_e1121_d_n2: f64 = (p.p87 * locals.var_qbdi_dn2);
        let eq20_e1121_d_n4: f64 = (p.p87 * locals.var_qbdi_dn4);
        let eq20_e1121_d_n5: f64 = (p.p87 * locals.var_qbdi_dn5);
        let eq20_e1121_d_n6: f64 = (p.p87 * locals.var_qbdi_dn6);
        let eq20_e1121_d_n7: f64 = (p.p87 * locals.var_qbdi_dn7);
        let eq20_e1121_d_n8: f64 = (p.p87 * locals.var_qbdi_dn8);
        let eq20_e1121_d_n9: f64 = (p.p87 * locals.var_qbdi_dn9);
        let eq20_e1121_d_n10: f64 = (p.p87 * locals.var_qbdi_dn10);
        let eq20_e1121_d_n11: f64 = (p.p87 * locals.var_qbdi_dn11);
        let eq20_e1121_d_n14: f64 = (p.p87 * locals.var_qbdi_dn14);
        let eq20_e1121_q: f64 = (p.p87 * eq20_e1120_q);
        (eq20_e1121, eq20_e1121_d_n0, eq20_e1121_d_n2, eq20_e1121_d_n4, eq20_e1121_d_n5, eq20_e1121_d_n6, eq20_e1121_d_n7, eq20_e1121_d_n8, eq20_e1121_d_n9, eq20_e1121_d_n10, eq20_e1121_d_n11, eq20_e1121_d_n14, eq20_e1121_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e1123_d_n0, 0.0, eq20_e1123_d_n2, 0.0, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, 0.0, 0.0, eq20_e1123_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq20_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e1167: f64 = (locals.var_qg + locals.var_qg_nqs);
        let eq28_e1168_q: f64 = eq28_e1167;
        let eq28_e1169: f64 = (p.p87 * eq28_e1167);
        let eq28_e1169_d_n0: f64 = (p.p87 * locals.var_qg_dn0);
        let eq28_e1169_d_n2: f64 = (p.p87 * locals.var_qg_dn2);
        let eq28_e1169_d_n4: f64 = (p.p87 * locals.var_qg_dn4);
        let eq28_e1169_d_n5: f64 = (p.p87 * locals.var_qg_dn5);
        let eq28_e1169_d_n6: f64 = (p.p87 * locals.var_qg_dn6);
        let eq28_e1169_d_n7: f64 = (p.p87 * locals.var_qg_dn7);
        let eq28_e1169_d_n8: f64 = (p.p87 * locals.var_qg_dn8);
        let eq28_e1169_d_n9: f64 = (p.p87 * locals.var_qg_dn9);
        let eq28_e1169_d_n10: f64 = (p.p87 * locals.var_qg_dn10);
        let eq28_e1169_d_n11: f64 = (p.p87 * locals.var_qg_dn11);
        let eq28_e1169_d_n12: f64 = (p.p87 * locals.var_qg_nqs_dn12);
        let eq28_e1169_d_n13: f64 = (p.p87 * locals.var_qg_nqs_dn13);
        let eq28_e1169_d_n14: f64 = (p.p87 * locals.var_qg_dn14);
        let eq28_e1169_q: f64 = (p.p87 * eq28_e1168_q);
        let eq28_reactive_node_derivatives: [f64; 19] = [eq28_e1169_d_n0, 0.0, eq28_e1169_d_n2, 0.0, eq28_e1169_d_n4, eq28_e1169_d_n5, eq28_e1169_d_n6, eq28_e1169_d_n7, eq28_e1169_d_n8, eq28_e1169_d_n9, eq28_e1169_d_n10, eq28_e1169_d_n11, eq28_e1169_d_n12, eq28_e1169_d_n13, eq28_e1169_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq28_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e1173: f64 = (locals.var_qd + locals.var_qd_nqs);
        let eq29_e1173_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);
        let eq29_e1173_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);
        let eq29_e1173_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);
        let eq29_e1173_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);
        let eq29_e1173_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);
        let eq29_e1173_d_n7: f64 = (locals.var_qd_dn7 + locals.var_qd_nqs_dn7);
        let eq29_e1173_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);
        let eq29_e1173_d_n9: f64 = (locals.var_qd_dn9 + locals.var_qd_nqs_dn9);
        let eq29_e1173_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);
        let eq29_e1173_d_n11: f64 = (locals.var_qd_dn11 + locals.var_qd_nqs_dn11);
        let eq29_e1173_d_n14: f64 = (locals.var_qd_dn14 + locals.var_qd_nqs_dn14);
        let eq29_e1174_q: f64 = eq29_e1173;
        let eq29_e1175: f64 = (p.p87 * eq29_e1173);
        let eq29_e1175_d_n0: f64 = (p.p87 * eq29_e1173_d_n0);
        let eq29_e1175_d_n2: f64 = (p.p87 * eq29_e1173_d_n2);
        let eq29_e1175_d_n4: f64 = (p.p87 * eq29_e1173_d_n4);
        let eq29_e1175_d_n5: f64 = (p.p87 * eq29_e1173_d_n5);
        let eq29_e1175_d_n6: f64 = (p.p87 * eq29_e1173_d_n6);
        let eq29_e1175_d_n7: f64 = (p.p87 * eq29_e1173_d_n7);
        let eq29_e1175_d_n8: f64 = (p.p87 * eq29_e1173_d_n8);
        let eq29_e1175_d_n9: f64 = (p.p87 * eq29_e1173_d_n9);
        let eq29_e1175_d_n10: f64 = (p.p87 * eq29_e1173_d_n10);
        let eq29_e1175_d_n11: f64 = (p.p87 * eq29_e1173_d_n11);
        let eq29_e1175_d_n12: f64 = (p.p87 * locals.var_qd_nqs_dn12);
        let eq29_e1175_d_n14: f64 = (p.p87 * eq29_e1173_d_n14);
        let eq29_e1175_q: f64 = (p.p87 * eq29_e1174_q);
        let eq29_reactive_node_derivatives: [f64; 19] = [eq29_e1175_d_n0, 0.0, eq29_e1175_d_n2, 0.0, eq29_e1175_d_n4, eq29_e1175_d_n5, eq29_e1175_d_n6, eq29_e1175_d_n7, eq29_e1175_d_n8, eq29_e1175_d_n9, eq29_e1175_d_n10, eq29_e1175_d_n11, eq29_e1175_d_n12, 0.0, eq29_e1175_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq29_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e1180: f64 = (locals.var_qg_nqs + locals.var_qd_nqs);
        let eq30_e1180_d_n12: f64 = (locals.var_qg_nqs_dn12 + locals.var_qd_nqs_dn12);
        let eq30_e1182: f64 = (eq30_e1180 + locals.var_qs_nqs);
        let eq30_e1182_d_n0: f64 = (locals.var_qd_nqs_dn0 + locals.var_qs_nqs_dn0);
        let eq30_e1182_d_n2: f64 = (locals.var_qd_nqs_dn2 + locals.var_qs_nqs_dn2);
        let eq30_e1182_d_n4: f64 = (locals.var_qd_nqs_dn4 + locals.var_qs_nqs_dn4);
        let eq30_e1182_d_n5: f64 = (locals.var_qd_nqs_dn5 + locals.var_qs_nqs_dn5);
        let eq30_e1182_d_n6: f64 = (locals.var_qd_nqs_dn6 + locals.var_qs_nqs_dn6);
        let eq30_e1182_d_n7: f64 = (locals.var_qd_nqs_dn7 + locals.var_qs_nqs_dn7);
        let eq30_e1182_d_n8: f64 = (locals.var_qd_nqs_dn8 + locals.var_qs_nqs_dn8);
        let eq30_e1182_d_n9: f64 = (locals.var_qd_nqs_dn9 + locals.var_qs_nqs_dn9);
        let eq30_e1182_d_n10: f64 = (locals.var_qd_nqs_dn10 + locals.var_qs_nqs_dn10);
        let eq30_e1182_d_n11: f64 = (locals.var_qd_nqs_dn11 + locals.var_qs_nqs_dn11);
        let eq30_e1182_d_n12: f64 = (eq30_e1180_d_n12 + locals.var_qs_nqs_dn12);
        let eq30_e1182_d_n14: f64 = (locals.var_qd_nqs_dn14 + locals.var_qs_nqs_dn14);
        let eq30_e1183: f64 = (locals.var_qb - eq30_e1182);
        let eq30_e1183_d_n0: f64 = (locals.var_qb_dn0 - eq30_e1182_d_n0);
        let eq30_e1183_d_n2: f64 = (locals.var_qb_dn2 - eq30_e1182_d_n2);
        let eq30_e1183_d_n4: f64 = (locals.var_qb_dn4 - eq30_e1182_d_n4);
        let eq30_e1183_d_n5: f64 = (locals.var_qb_dn5 - eq30_e1182_d_n5);
        let eq30_e1183_d_n6: f64 = (locals.var_qb_dn6 - eq30_e1182_d_n6);
        let eq30_e1183_d_n7: f64 = (locals.var_qb_dn7 - eq30_e1182_d_n7);
        let eq30_e1183_d_n8: f64 = (locals.var_qb_dn8 - eq30_e1182_d_n8);
        let eq30_e1183_d_n9: f64 = (locals.var_qb_dn9 - eq30_e1182_d_n9);
        let eq30_e1183_d_n10: f64 = (locals.var_qb_dn10 - eq30_e1182_d_n10);
        let eq30_e1183_d_n11: f64 = (locals.var_qb_dn11 - eq30_e1182_d_n11);
        let eq30_e1183_d_n14: f64 = (locals.var_qb_dn14 - eq30_e1182_d_n14);
        let eq30_e1184_q: f64 = eq30_e1183;
        let eq30_e1185: f64 = (p.p87 * eq30_e1183);
        let eq30_e1185_d_n0: f64 = (p.p87 * eq30_e1183_d_n0);
        let eq30_e1185_d_n2: f64 = (p.p87 * eq30_e1183_d_n2);
        let eq30_e1185_d_n4: f64 = (p.p87 * eq30_e1183_d_n4);
        let eq30_e1185_d_n5: f64 = (p.p87 * eq30_e1183_d_n5);
        let eq30_e1185_d_n6: f64 = (p.p87 * eq30_e1183_d_n6);
        let eq30_e1185_d_n7: f64 = (p.p87 * eq30_e1183_d_n7);
        let eq30_e1185_d_n8: f64 = (p.p87 * eq30_e1183_d_n8);
        let eq30_e1185_d_n9: f64 = (p.p87 * eq30_e1183_d_n9);
        let eq30_e1185_d_n10: f64 = (p.p87 * eq30_e1183_d_n10);
        let eq30_e1185_d_n11: f64 = (p.p87 * eq30_e1183_d_n11);
        let eq30_e1185_d_n12: f64 = (p.p87 * (-eq30_e1182_d_n12));
        let eq30_e1185_d_n13: f64 = (p.p87 * (-locals.var_qg_nqs_dn13));
        let eq30_e1185_d_n14: f64 = (p.p87 * eq30_e1183_d_n14);
        let eq30_e1185_q: f64 = (p.p87 * eq30_e1184_q);
        let eq30_reactive_node_derivatives: [f64; 19] = [eq30_e1185_d_n0, 0.0, eq30_e1185_d_n2, 0.0, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq30_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188_q: f64 = locals.var_qgext;
        let eq31_e1189: f64 = (p.p87 * locals.var_qgext);
        let eq31_e1189_d_n0: f64 = (p.p87 * locals.var_qgext_dn0);
        let eq31_e1189_d_n2: f64 = (p.p87 * locals.var_qgext_dn2);
        let eq31_e1189_d_n4: f64 = (p.p87 * locals.var_qgext_dn4);
        let eq31_e1189_d_n5: f64 = (p.p87 * locals.var_qgext_dn5);
        let eq31_e1189_d_n6: f64 = (p.p87 * locals.var_qgext_dn6);
        let eq31_e1189_d_n7: f64 = (p.p87 * locals.var_qgext_dn7);
        let eq31_e1189_d_n8: f64 = (p.p87 * locals.var_qgext_dn8);
        let eq31_e1189_d_n9: f64 = (p.p87 * locals.var_qgext_dn9);
        let eq31_e1189_d_n10: f64 = (p.p87 * locals.var_qgext_dn10);
        let eq31_e1189_d_n11: f64 = (p.p87 * locals.var_qgext_dn11);
        let eq31_e1189_d_n14: f64 = (p.p87 * locals.var_qgext_dn14);
        let eq31_e1189_q: f64 = (p.p87 * eq31_e1188_q);
        let eq31_reactive_node_derivatives: [f64; 19] = [eq31_e1189_d_n0, 0.0, eq31_e1189_d_n2, 0.0, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, 0.0, 0.0, eq31_e1189_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq31_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192_q: f64 = locals.var_qdext;
        let eq32_e1193: f64 = (p.p87 * locals.var_qdext);
        let eq32_e1193_d_n0: f64 = (p.p87 * locals.var_qdext_dn0);
        let eq32_e1193_d_n2: f64 = (p.p87 * locals.var_qdext_dn2);
        let eq32_e1193_d_n4: f64 = (p.p87 * locals.var_qdext_dn4);
        let eq32_e1193_d_n5: f64 = (p.p87 * locals.var_qdext_dn5);
        let eq32_e1193_d_n6: f64 = (p.p87 * locals.var_qdext_dn6);
        let eq32_e1193_d_n7: f64 = (p.p87 * locals.var_qdext_dn7);
        let eq32_e1193_d_n8: f64 = (p.p87 * locals.var_qdext_dn8);
        let eq32_e1193_d_n9: f64 = (p.p87 * locals.var_qdext_dn9);
        let eq32_e1193_d_n10: f64 = (p.p87 * locals.var_qdext_dn10);
        let eq32_e1193_d_n11: f64 = (p.p87 * locals.var_qdext_dn11);
        let eq32_e1193_d_n14: f64 = (p.p87 * locals.var_qdext_dn14);
        let eq32_e1193_q: f64 = (p.p87 * eq32_e1192_q);
        let eq32_reactive_node_derivatives: [f64; 19] = [eq32_e1193_d_n0, 0.0, eq32_e1193_d_n2, 0.0, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, 0.0, 0.0, eq32_e1193_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq32_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e1196_q: f64 = locals.var_qbext;
        let eq33_e1197: f64 = (p.p87 * locals.var_qbext);
        let eq33_e1197_d_n0: f64 = (p.p87 * locals.var_qbext_dn0);
        let eq33_e1197_d_n2: f64 = (p.p87 * locals.var_qbext_dn2);
        let eq33_e1197_d_n4: f64 = (p.p87 * locals.var_qbext_dn4);
        let eq33_e1197_d_n5: f64 = (p.p87 * locals.var_qbext_dn5);
        let eq33_e1197_d_n6: f64 = (p.p87 * locals.var_qbext_dn6);
        let eq33_e1197_d_n7: f64 = (p.p87 * locals.var_qbext_dn7);
        let eq33_e1197_d_n8: f64 = (p.p87 * locals.var_qbext_dn8);
        let eq33_e1197_d_n9: f64 = (p.p87 * locals.var_qbext_dn9);
        let eq33_e1197_d_n10: f64 = (p.p87 * locals.var_qbext_dn10);
        let eq33_e1197_d_n11: f64 = (p.p87 * locals.var_qbext_dn11);
        let eq33_e1197_d_n14: f64 = (p.p87 * locals.var_qbext_dn14);
        let eq33_e1197_q: f64 = (p.p87 * eq33_e1196_q);
        let eq33_reactive_node_derivatives: [f64; 19] = [eq33_e1197_d_n0, 0.0, eq33_e1197_d_n2, 0.0, eq33_e1197_d_n4, eq33_e1197_d_n5, eq33_e1197_d_n6, eq33_e1197_d_n7, eq33_e1197_d_n8, eq33_e1197_d_n9, eq33_e1197_d_n10, eq33_e1197_d_n11, 0.0, 0.0, eq33_e1197_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq33_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e1199: f64 = (-p.p87);
        let eq34_e1201_q: f64 = locals.var_qfd;
        let eq34_e1202: f64 = (eq34_e1199 * locals.var_qfd);
        let eq34_e1202_d_n0: f64 = (eq34_e1199 * locals.var_qfd_dn0);
        let eq34_e1202_d_n2: f64 = (eq34_e1199 * locals.var_qfd_dn2);
        let eq34_e1202_d_n7: f64 = (eq34_e1199 * locals.var_qfd_dn7);
        let eq34_e1202_q: f64 = (eq34_e1199 * eq34_e1201_q);
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq34_e1202_d_n0),
            nodes[2],
            multiplicity * (eq34_e1202_d_n2),
            nodes[7],
            multiplicity * (eq34_e1202_d_n7),
        );
        let eq35_e1204: f64 = (-p.p87);
        let eq35_e1206_q: f64 = locals.var_qfs;
        let eq35_e1207: f64 = (eq35_e1204 * locals.var_qfs);
        let eq35_e1207_d_n2: f64 = (eq35_e1204 * locals.var_qfs_dn2);
        let eq35_e1207_d_n7: f64 = (eq35_e1204 * locals.var_qfs_dn7);
        let eq35_e1207_q: f64 = (eq35_e1204 * eq35_e1206_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes[2],
            multiplicity * (eq35_e1207_d_n2),
            nodes[7],
            multiplicity * (eq35_e1207_d_n7),
        );
        let eq41_e1236: f64 = ((nv15 - 0.0) * locals.var_sigrat_s);
        let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn0);
        let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn2);
        let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn4);
        let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn5);
        let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn6);
        let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn7);
        let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn8);
        let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn9);
        let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn10);
        let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn11);
        let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn14);
        let eq41_e1237_q: f64 = eq41_e1236;
        let eq41_reactive_node_derivatives: [f64; 19] = [eq41_e1236_d_n0, 0.0, eq41_e1236_d_n2, 0.0, eq41_e1236_d_n4, eq41_e1236_d_n5, eq41_e1236_d_n6, eq41_e1236_d_n7, eq41_e1236_d_n8, eq41_e1236_d_n9, eq41_e1236_d_n10, eq41_e1236_d_n11, 0.0, 0.0, eq41_e1236_d_n14, locals.var_sigrat_s, 0.0, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1240: f64 = ((nv15 - 0.0) * locals.var_sigrat_d);
        let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn0);
        let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn2);
        let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn4);
        let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn5);
        let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn6);
        let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn7);
        let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn8);
        let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn9);
        let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn10);
        let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn11);
        let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn14);
        let eq42_e1241_q: f64 = eq42_e1240;
        let eq42_reactive_node_derivatives: [f64; 19] = [eq42_e1240_d_n0, 0.0, eq42_e1240_d_n2, 0.0, eq42_e1240_d_n4, eq42_e1240_d_n5, eq42_e1240_d_n6, eq42_e1240_d_n7, eq42_e1240_d_n8, eq42_e1240_d_n9, eq42_e1240_d_n10, eq42_e1240_d_n11, 0.0, 0.0, eq42_e1240_d_n14, locals.var_sigrat_d, 0.0, 0.0, 0.0];
        let eq42_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1362, eq62_e1362_d_n12, eq62_e1362_q,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (locals.var_cqi * (nv12 - 0.0));
        let eq62_e1360_q: f64 = eq62_e1359;
        (eq62_e1359, locals.var_cqi, eq62_e1360_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (eq62_e1362_d_n12),
        );
        let (eq63_e1369, eq63_e1369_d_n13, eq63_e1369_q,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (locals.var_cqb * (nv13 - 0.0));
        let eq63_e1367_q: f64 = eq63_e1366;
        (eq63_e1366, locals.var_cqb, eq63_e1367_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq63_e1369_d_n13),
        );
        let (eq67_e1388, eq67_e1388_d_n14, eq67_e1388_q,) = {
    if (p.p29 != 0.0) {
        let eq67_e1386_q: f64 = (nv14 - 0.0);
        ((nv14 - 0.0), 1.0, eq67_e1386_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[14]),
            None,
            nodes[14],
            multiplicity * (eq67_e1388_d_n14),
        );
    }
}
