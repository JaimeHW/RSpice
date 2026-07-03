#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    pub(super) fn stamp_reactive_block_155(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13, ) = (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13, );
            locals.var_dnm_rv = 0.0;
        }
        let assign105120_e157508: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2382 = assign105120_e157508;
        locals.var_guard2382_rv = 0.0;
        let assign105130_e157511: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2383 = assign105130_e157511;
        locals.var_guard2383_rv = 0.0;
        if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 != 0.0)) {
            locals.var_mm = 1.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105150_e157527: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2384 = assign105150_e157527;
        locals.var_guard2384_rv = 0.0;
        if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 == 0.0)) && (locals.var_guard2384 != 0.0)) {
            locals.var_mm = 2.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105170_e157546: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2385 = assign105170_e157546;
        locals.var_guard2385_rv = 0.0;
        if (((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 == 0.0)) && (locals.var_guard2384 == 0.0)) && (locals.var_guard2385 != 0.0)) {
            locals.var_mm = 3.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105190_e157568: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2386 = assign105190_e157568;
        locals.var_guard2386_rv = 0.0;
        if ((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 == 0.0)) && (locals.var_guard2384 == 0.0)) && (locals.var_guard2385 == 0.0)) && (locals.var_guard2386 != 0.0)) {
            locals.var_mm = 4.0;
            locals.var_mm_rv = 0.0;
        }
        if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) {
            locals.var_m0 = 0.0;
            locals.var_m0_rv = 0.0;
        }
        let mut assign105220_loop_guard: usize = 0;
        while {
            let assign105220_cond_e157613: f64 = if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105220_cond_e157613 != 0.0
        } {
            assign105220_loop_guard += 1;
            assert!(assign105220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) {
                let assign105220_body0_e157623: f64 = (locals.var_dnm).sqrt();
                (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13, ) = (assign105220_body0_e157623, (locals.var_dnm_dn0 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn2 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn4 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn5 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn6 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn7 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn8 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn9 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn10 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn13 / (2.0 * assign105220_body0_e157623)), );
                locals.var_dnm_rv = 0.0;
            }
            if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) {
                let assign105220_body1_e157636: f64 = (locals.var_m0 + 1.0);
                locals.var_m0 = assign105220_body1_e157636;
                locals.var_m0_rv = 0.0;
            }
        }
        if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 == 0.0)) {
            let (assign105230_e157659, assign105230_e157659_d_n0, assign105230_e157659_d_n2, assign105230_e157659_d_n4, assign105230_e157659_d_n5, assign105230_e157659_d_n6, assign105230_e157659_d_n7, assign105230_e157659_d_n8, assign105230_e157659_d_n9, assign105230_e157659_d_n10, assign105230_e157659_d_n13,) = {
    if (locals.var_dnm == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign105230_e157656: f64 = (2.0 * 2.0);
        let assign105230_e157657: f64 = (1.0 / assign105230_e157656);
        let assign105230_e157658: f64 = (locals.var_dnm).powf(assign105230_e157657);
        (assign105230_e157658, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn13)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
    }
};
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13, ) = (assign105230_e157659, assign105230_e157659_d_n0, assign105230_e157659_d_n2, assign105230_e157659_d_n4, assign105230_e157659_d_n5, assign105230_e157659_d_n6, assign105230_e157659_d_n7, assign105230_e157659_d_n8, assign105230_e157659_d_n9, assign105230_e157659_d_n10, assign105230_e157659_d_n13, );
            locals.var_dnm_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
            let assign105240_e157670: f64 = (1.0 / locals.var_dnm);
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13, ) = (assign105240_e157670, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))), );
            locals.var_dnm_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
            let assign105250_e157681: f64 = (locals.var_tmf1 * 1e-25);
            let assign105250_e157683: f64 = (assign105250_e157681 * locals.var_dnm);
            (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13, ) = (assign105250_e157683, (((locals.var_tmf1_dn0 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn13)), );
            locals.var_tmf0_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
            let assign105260_e157694: f64 = (1e-25 * locals.var_xmp);
            let assign105260_e157696: f64 = (assign105260_e157694 * locals.var_dnm);
            let assign105260_e157698: f64 = (assign105260_e157696 / locals.var_arg);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13, ) = (assign105260_e157698, ((((((1e-25 * locals.var_xmp_dn0) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn0)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn2) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn2)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn4) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn4)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn5) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn5)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn6) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn6)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn7) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn7)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn8) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn8)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn9) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn9)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn10) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn10)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn13) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn13)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)), );
            locals.var_t0_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
            let assign105270_e157709: f64 = 1e-25;
            let assign105270_e157711: f64 = (assign105270_e157709 - locals.var_tmf0);
            (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn13, ) = (assign105270_e157711, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13), );
            locals.var_gd_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13, ) = (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13, );
            locals.var_t0_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 == 0.0)) {
            (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn13, ) = (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn13, );
            locals.var_gd_rv = 0.0;
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
            let assign105310_e157749: f64 = (1.0 / locals.var_gd);
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, ) = (assign105310_e157749, (-(locals.var_gd_dn0 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn2 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn4 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn5 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn6 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn7 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn8 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn9 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn10 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn13 / (locals.var_gd * locals.var_gd))), );
            locals.var_rdd_rv = 0.0;
        }
        if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
            let assign105320_e157758: f64 = (locals.var_rdd / locals.var_weffld_nf);
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, ) = (assign105320_e157758, (locals.var_rdd_dn0 / locals.var_weffld_nf), (locals.var_rdd_dn2 / locals.var_weffld_nf), (locals.var_rdd_dn4 / locals.var_weffld_nf), (locals.var_rdd_dn5 / locals.var_weffld_nf), (locals.var_rdd_dn6 / locals.var_weffld_nf), (locals.var_rdd_dn7 / locals.var_weffld_nf), (locals.var_rdd_dn8 / locals.var_weffld_nf), (locals.var_rdd_dn9 / locals.var_weffld_nf), (locals.var_rdd_dn10 / locals.var_weffld_nf), (locals.var_rdd_dn13 / locals.var_weffld_nf), );
            locals.var_rdd_rv = 0.0;
        }
        let assign105330_e157764: f64 = (1000000.0 - 1000.0);
        let assign105330_e157769: f64 = if ((locals.var_rdd > assign105330_e157764) && (1000.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2387 = assign105330_e157769;
        locals.var_guard2387_rv = 0.0;
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105340_e157778: f64 = (locals.var_rdd - 1000000.0);
            let assign105340_e157780: f64 = (assign105340_e157778 + 1000.0);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13, ) = (assign105340_e157780, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, );
            locals.var_tmf1_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105350_e157791: f64 = (locals.var_tmf1 * locals.var_tmf1);
            (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13, ) = (assign105350_e157791, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)), );
            locals.var_x2_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105360_e157802: f64 = (1000.0 * 1000.0);
            (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13, ) = (assign105360_e157802, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_xmax2_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_xp_rv = 0.0;
            (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_xmp_rv = 0.0;
            locals.var_m0 = 0.0;
            locals.var_m0_rv = 0.0;
            locals.var_mm = 0.0;
            locals.var_mm_rv = 0.0;
            (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_arg_rv = 0.0;
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_dnm_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105430_e157867: f64 = (locals.var_xp * locals.var_x2);
            (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13, ) = (assign105430_e157867, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)), );
            locals.var_xp_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105440_e157878: f64 = (locals.var_xmp * locals.var_xmax2);
            (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13, ) = (assign105440_e157878, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)), );
            locals.var_xmp_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105450_e157889: f64 = (locals.var_xp * locals.var_x2);
            (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13, ) = (assign105450_e157889, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)), );
            locals.var_xp_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105460_e157900: f64 = (locals.var_xmp * locals.var_xmax2);
            (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13, ) = (assign105460_e157900, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)), );
            locals.var_xmp_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105470_e157911: f64 = (locals.var_xp + locals.var_xmp);
            (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13, ) = (assign105470_e157911, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13), );
            locals.var_arg_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13, ) = (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13, );
            locals.var_dnm_rv = 0.0;
        }
        let assign105490_e157937: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2388 = assign105490_e157937;
        locals.var_guard2388_rv = 0.0;
        let assign105500_e157940: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2389 = assign105500_e157940;
        locals.var_guard2389_rv = 0.0;
        if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 != 0.0)) {
            locals.var_mm = 1.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105520_e157956: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2390 = assign105520_e157956;
        locals.var_guard2390_rv = 0.0;
        if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 == 0.0)) && (locals.var_guard2390 != 0.0)) {
            locals.var_mm = 2.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105540_e157975: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2391 = assign105540_e157975;
        locals.var_guard2391_rv = 0.0;
        if (((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 == 0.0)) && (locals.var_guard2390 == 0.0)) && (locals.var_guard2391 != 0.0)) {
            locals.var_mm = 3.0;
            locals.var_mm_rv = 0.0;
        }
        let assign105560_e157997: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2392 = assign105560_e157997;
        locals.var_guard2392_rv = 0.0;
        if ((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 == 0.0)) && (locals.var_guard2390 == 0.0)) && (locals.var_guard2391 == 0.0)) && (locals.var_guard2392 != 0.0)) {
            locals.var_mm = 4.0;
            locals.var_mm_rv = 0.0;
        }
        if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) {
            locals.var_m0 = 0.0;
            locals.var_m0_rv = 0.0;
        }
        let mut assign105590_loop_guard: usize = 0;
        while {
            let assign105590_cond_e158042: f64 = if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105590_cond_e158042 != 0.0
        } {
            assign105590_loop_guard += 1;
            assert!(assign105590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) {
                let assign105590_body0_e158052: f64 = (locals.var_dnm).sqrt();
                (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13, ) = (assign105590_body0_e158052, (locals.var_dnm_dn0 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn2 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn4 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn5 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn6 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn7 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn8 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn9 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn10 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn13 / (2.0 * assign105590_body0_e158052)), );
                locals.var_dnm_rv = 0.0;
            }
            if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) {
                let assign105590_body1_e158065: f64 = (locals.var_m0 + 1.0);
                locals.var_m0 = assign105590_body1_e158065;
                locals.var_m0_rv = 0.0;
            }
        }
        if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 == 0.0)) {
            let (assign105600_e158088, assign105600_e158088_d_n0, assign105600_e158088_d_n2, assign105600_e158088_d_n4, assign105600_e158088_d_n5, assign105600_e158088_d_n6, assign105600_e158088_d_n7, assign105600_e158088_d_n8, assign105600_e158088_d_n9, assign105600_e158088_d_n10, assign105600_e158088_d_n13,) = {
    if (locals.var_dnm == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign105600_e158085: f64 = (2.0 * 2.0);
        let assign105600_e158086: f64 = (1.0 / assign105600_e158085);
        let assign105600_e158087: f64 = (locals.var_dnm).powf(assign105600_e158086);
        (assign105600_e158087, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn13)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
    }
};
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13, ) = (assign105600_e158088, assign105600_e158088_d_n0, assign105600_e158088_d_n2, assign105600_e158088_d_n4, assign105600_e158088_d_n5, assign105600_e158088_d_n6, assign105600_e158088_d_n7, assign105600_e158088_d_n8, assign105600_e158088_d_n9, assign105600_e158088_d_n10, assign105600_e158088_d_n13, );
            locals.var_dnm_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105610_e158099: f64 = (1.0 / locals.var_dnm);
            (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13, ) = (assign105610_e158099, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))), );
            locals.var_dnm_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105620_e158110: f64 = (locals.var_tmf1 * 1000.0);
            let assign105620_e158112: f64 = (assign105620_e158110 * locals.var_dnm);
            (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13, ) = (assign105620_e158112, (((locals.var_tmf1_dn0 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn13)), );
            locals.var_tmf0_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105630_e158123: f64 = (1000.0 * locals.var_xmp);
            let assign105630_e158125: f64 = (assign105630_e158123 * locals.var_dnm);
            let assign105630_e158127: f64 = (assign105630_e158125 / locals.var_arg);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13, ) = (assign105630_e158127, ((((((1000.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn0)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn2)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn4)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn5)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn6)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn7)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn8)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn9)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn10)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn13) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn13)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)), );
            locals.var_t0_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            let assign105640_e158138: f64 = (1000000.0 - 1000.0);
            let assign105640_e158140: f64 = (assign105640_e158138 + locals.var_tmf0);
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, ) = (assign105640_e158140, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13, );
            locals.var_rdd_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13, ) = (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13, );
            locals.var_t0_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 == 0.0)) {
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, ) = (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, );
            locals.var_rdd_rv = 0.0;
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        let assign105680_e158178: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign105680_e158179: f64 = (locals.var_uc_nover * assign105680_e158178);
        let assign105680_e158182: f64 = if ((p.p54 == 1.0) && (assign105680_e158179 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2393 = assign105680_e158182;
        locals.var_guard2393_rv = 0.0;
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2393 != 0.0)) {
            let assign105690_e158191: f64 = (p.p334 - locals.var_wdep);
            (locals.var_ddriftld, locals.var_ddriftld_dn0, locals.var_ddriftld_dn2, locals.var_ddriftld_dn4, locals.var_ddriftld_dn5, locals.var_ddriftld_dn6, locals.var_ddriftld_dn7, locals.var_ddriftld_dn8, locals.var_ddriftld_dn9, locals.var_ddriftld_dn10, locals.var_ddriftld_dn13, ) = (assign105690_e158191, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn13), );
            locals.var_ddriftld_rv = 0.0;
        }
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2393 != 0.0)) {
            let assign105700_e158202: f64 = (locals.var_rdd * locals.var_ldrift0);
            let assign105700_e158204: f64 = (assign105700_e158202 / locals.var_ddriftld);
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, ) = (assign105700_e158204, ((((locals.var_rdd_dn0 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn0)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn2 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn2)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn4 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn4)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn5 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn5)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn6 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn6)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn7 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn7)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn8 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn8)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn9 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn9)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn10 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn10)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn13 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn13)) / (locals.var_ddriftld * locals.var_ddriftld)), );
            locals.var_rdd_rv = 0.0;
        }
        if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
            let assign105710_e158213: f64 = (locals.var_rdd + locals.var_rd0);
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, ) = (assign105710_e158213, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, );
            locals.var_rdd_rv = 0.0;
        }
        let assign105750_e158246: f64 = if locals.var_rdd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2395 = assign105750_e158246;
        locals.var_guard2395_rv = 0.0;
        if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2395 != 0.0)) {
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, ) = (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_rdd_rv = 0.0;
        }
        if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
            let assign105770_e158262: f64 = (locals.var_rdd / locals.var_mfactor);
            (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn13, ) = (assign105770_e158262, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn13 / locals.var_mfactor), );
            locals.var_rdde_rv = 0.0;
        }
        let assign105780_e158267: f64 = if locals.var_rdd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2396 = assign105780_e158267;
        locals.var_guard2396_rv = 0.0;
        if ((locals.var_guard2336 == 0.0) && (locals.var_guard2396 != 0.0)) {
            (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, ) = (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_rdd_rv = 0.0;
        }
        let assign105800_e158277: f64 = if locals.var_rsd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2397 = assign105800_e158277;
        locals.var_guard2397_rv = 0.0;
        if ((locals.var_guard2336 == 0.0) && (locals.var_guard2397 != 0.0)) {
            (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13, ) = (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_rsd_rv = 0.0;
        }
        let assign105820_e158287: f64 = if locals.var_vdsemodenml > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2398 = assign105820_e158287;
        locals.var_guard2398_rv = 0.0;
        if ((locals.var_guard2336 == 0.0) && (locals.var_guard2398 != 0.0)) {
            let assign105830_e158294: f64 = (locals.var_rdd / locals.var_mfactor);
            (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn13, ) = (assign105830_e158294, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn13 / locals.var_mfactor), );
            locals.var_rdde_rv = 0.0;
        }
        if ((locals.var_guard2336 == 0.0) && (locals.var_guard2398 != 0.0)) {
            let assign105840_e158303: f64 = (locals.var_rsd / locals.var_mfactor);
            (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn13, ) = (assign105840_e158303, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn13 / locals.var_mfactor), );
            locals.var_rsde_rv = 0.0;
        }
        if ((locals.var_guard2336 == 0.0) && (locals.var_guard2398 == 0.0)) {
            let assign105850_e158313: f64 = (locals.var_rsd / locals.var_mfactor);
            (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn13, ) = (assign105850_e158313, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn13 / locals.var_mfactor), );
            locals.var_rdde_rv = 0.0;
        }
        if ((locals.var_guard2336 == 0.0) && (locals.var_guard2398 == 0.0)) {
            let assign105860_e158323: f64 = (locals.var_rdd / locals.var_mfactor);
            (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn13, ) = (assign105860_e158323, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn13 / locals.var_mfactor), );
            locals.var_rsde_rv = 0.0;
        }
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13, ) = (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn13, );
        locals.var_rdd_rv = 0.0;
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13, ) = (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn13, );
        locals.var_rsd_rv = 0.0;
        let assign105920_e158333: f64 = if locals.var_mode > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2399 = assign105920_e158333;
        locals.var_guard2399_rv = 0.0;
        if (locals.var_guard2399 != 0.0) {
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13, ) = (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn8, locals.var_idse_dn9, locals.var_idse_dn10, locals.var_idse_dn13, );
            locals.var_ids_rv = 0.0;
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn13, ) = (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn13, );
            locals.var_qd_rv = 0.0;
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn13, ) = (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn13, );
            locals.var_qg_rv = 0.0;
            (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn13, ) = (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn13, );
            locals.var_qs_rv = 0.0;
        }
        if (locals.var_guard2399 != 0.0) {
            let assign105970_e158353: f64 = (locals.var_qge + locals.var_qde);
            let assign105970_e158355: f64 = (assign105970_e158353 + locals.var_qse);
            let assign105970_e158356: f64 = (-assign105970_e158355);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn13, ) = (assign105970_e158356, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn9 + locals.var_qde_dn9) + locals.var_qse_dn9)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), );
            locals.var_qb_rv = 0.0;
        }
        if (locals.var_guard2399 != 0.0) {
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn13, ) = (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn4, locals.var_isube_dn5, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn8, locals.var_isube_dn9, locals.var_isube_dn10, locals.var_isube_dn13, );
            locals.var_isub_rv = 0.0;
            (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn13, ) = (locals.var_isublde, locals.var_isublde_dn0, locals.var_isublde_dn2, locals.var_isublde_dn4, locals.var_isublde_dn5, locals.var_isublde_dn6, locals.var_isublde_dn7, locals.var_isublde_dn8, locals.var_isublde_dn9, locals.var_isublde_dn10, locals.var_isublde_dn13, );
            locals.var_isubld_rv = 0.0;
            (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn13, ) = (locals.var_idsibpce, locals.var_idsibpce_dn0, locals.var_idsibpce_dn2, locals.var_idsibpce_dn4, locals.var_idsibpce_dn5, locals.var_idsibpce_dn6, locals.var_idsibpce_dn7, locals.var_idsibpce_dn8, locals.var_idsibpce_dn9, locals.var_idsibpce_dn10, locals.var_idsibpce_dn13, );
            locals.var_idsibpc_rv = 0.0;
        }
        if ((locals.var_guard2399 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13, ) = (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn13, );
            locals.var_qdrat_rv = 0.0;
        }
        if (locals.var_guard2399 == 0.0) {
            let assign106110_e158416: f64 = (-locals.var_idse);
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13, ) = (assign106110_e158416, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn4), (-locals.var_idse_dn5), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn8), (-locals.var_idse_dn9), (-locals.var_idse_dn10), (-locals.var_idse_dn13), );
            locals.var_ids_rv = 0.0;
        }
        if (locals.var_guard2399 == 0.0) {
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn13, ) = (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn13, );
            locals.var_qd_rv = 0.0;
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn13, ) = (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn13, );
            locals.var_qg_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_156(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard2399 == 0.0) {
            (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn13, ) = (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn13, );
            locals.var_qs_rv = 0.0;
        }
        if (locals.var_guard2399 == 0.0) {
            let assign106150_e158438: f64 = (locals.var_qge + locals.var_qde);
            let assign106150_e158440: f64 = (assign106150_e158438 + locals.var_qse);
            let assign106150_e158441: f64 = (-assign106150_e158440);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn13, ) = (assign106150_e158441, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn9 + locals.var_qde_dn9) + locals.var_qse_dn9)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), );
            locals.var_qb_rv = 0.0;
        }
        if (locals.var_guard2399 == 0.0) {
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn13, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isub_rv = 0.0;
            (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn13, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isubld_rv = 0.0;
            (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn13, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_idsibpc_rv = 0.0;
        }
        if ((locals.var_guard2399 == 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign106280_e158510: f64 = (1.0 - locals.var_xd);
            (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13, ) = (assign106280_e158510, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn4), (-locals.var_xd_dn5), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn8), (-locals.var_xd_dn9), (-locals.var_xd_dn10), (-locals.var_xd_dn13), );
            locals.var_qdrat_rv = 0.0;
        }
        let assign106290_e158515: f64 = (locals.var_qg + locals.var_qgov);
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn13, ) = (assign106290_e158515, (locals.var_qg_dn0 + locals.var_qgov_dn0), (locals.var_qg_dn2 + locals.var_qgov_dn2), (locals.var_qg_dn4 + locals.var_qgov_dn4), (locals.var_qg_dn5 + locals.var_qgov_dn5), (locals.var_qg_dn6 + locals.var_qgov_dn6), (locals.var_qg_dn7 + locals.var_qgov_dn7), (locals.var_qg_dn8 + locals.var_qgov_dn8), (locals.var_qg_dn9 + locals.var_qgov_dn9), (locals.var_qg_dn10 + locals.var_qgov_dn10), (locals.var_qg_dn13 + locals.var_qgov_dn13), );
        locals.var_qg_rv = 0.0;
        let assign106300_e158518: f64 = (locals.var_qd + locals.var_qdov);
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn13, ) = (assign106300_e158518, (locals.var_qd_dn0 + locals.var_qdov_dn0), (locals.var_qd_dn2 + locals.var_qdov_dn2), (locals.var_qd_dn4 + locals.var_qdov_dn4), (locals.var_qd_dn5 + locals.var_qdov_dn5), (locals.var_qd_dn6 + locals.var_qdov_dn6), (locals.var_qd_dn7 + locals.var_qdov_dn7), (locals.var_qd_dn8 + locals.var_qdov_dn8), (locals.var_qd_dn9 + locals.var_qdov_dn9), (locals.var_qd_dn10 + locals.var_qdov_dn10), (locals.var_qd_dn13 + locals.var_qdov_dn13), );
        locals.var_qd_rv = 0.0;
        let assign106310_e158521: f64 = (locals.var_qs + locals.var_qsov);
        (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn13, ) = (assign106310_e158521, (locals.var_qs_dn0 + locals.var_qsov_dn0), (locals.var_qs_dn2 + locals.var_qsov_dn2), (locals.var_qs_dn4 + locals.var_qsov_dn4), (locals.var_qs_dn5 + locals.var_qsov_dn5), (locals.var_qs_dn6 + locals.var_qsov_dn6), (locals.var_qs_dn7 + locals.var_qsov_dn7), (locals.var_qs_dn8 + locals.var_qsov_dn8), (locals.var_qs_dn9 + locals.var_qsov_dn9), (locals.var_qs_dn10 + locals.var_qsov_dn10), (locals.var_qs_dn13 + locals.var_qsov_dn13), );
        locals.var_qs_rv = 0.0;
        let assign106320_e158524: f64 = (locals.var_qg + locals.var_qd);
        let assign106320_e158526: f64 = (assign106320_e158524 + locals.var_qs);
        let assign106320_e158527: f64 = (-assign106320_e158526);
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn13, ) = (assign106320_e158527, (-((locals.var_qg_dn0 + locals.var_qd_dn0) + locals.var_qs_dn0)), (-((locals.var_qg_dn2 + locals.var_qd_dn2) + locals.var_qs_dn2)), (-((locals.var_qg_dn4 + locals.var_qd_dn4) + locals.var_qs_dn4)), (-((locals.var_qg_dn5 + locals.var_qd_dn5) + locals.var_qs_dn5)), (-((locals.var_qg_dn6 + locals.var_qd_dn6) + locals.var_qs_dn6)), (-((locals.var_qg_dn7 + locals.var_qd_dn7) + locals.var_qs_dn7)), (-((locals.var_qg_dn8 + locals.var_qd_dn8) + locals.var_qs_dn8)), (-((locals.var_qg_dn9 + locals.var_qd_dn9) + locals.var_qs_dn9)), (-((locals.var_qg_dn10 + locals.var_qd_dn10) + locals.var_qs_dn10)), (-((locals.var_qg_dn13 + locals.var_qd_dn13) + locals.var_qs_dn13)), );
        locals.var_qb_rv = 0.0;
        (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, ) = (locals.var_qdp, locals.var_qdp_dn0, locals.var_qdp_dn2, locals.var_qdp_dn6, );
        locals.var_qfd_rv = 0.0;
        (locals.var_qfs, locals.var_qfs_dn2, locals.var_qfs_dn6, ) = (locals.var_qsp, locals.var_qsp_dn2, locals.var_qsp_dn6, );
        locals.var_qfs_rv = 0.0;
        (locals.var_qdext, locals.var_qdext_dn0, locals.var_qdext_dn2, locals.var_qdext_dn4, locals.var_qdext_dn5, locals.var_qdext_dn6, locals.var_qdext_dn7, locals.var_qdext_dn8, locals.var_qdext_dn9, locals.var_qdext_dn10, locals.var_qdext_dn13, ) = (locals.var_qdexte, locals.var_qdexte_dn0, locals.var_qdexte_dn2, locals.var_qdexte_dn4, locals.var_qdexte_dn5, locals.var_qdexte_dn6, locals.var_qdexte_dn7, locals.var_qdexte_dn8, locals.var_qdexte_dn9, locals.var_qdexte_dn10, locals.var_qdexte_dn13, );
        locals.var_qdext_rv = 0.0;
        (locals.var_qgext, locals.var_qgext_dn0, locals.var_qgext_dn2, locals.var_qgext_dn4, locals.var_qgext_dn5, locals.var_qgext_dn6, locals.var_qgext_dn7, locals.var_qgext_dn8, locals.var_qgext_dn9, locals.var_qgext_dn10, locals.var_qgext_dn13, ) = (locals.var_qgexte, locals.var_qgexte_dn0, locals.var_qgexte_dn2, locals.var_qgexte_dn4, locals.var_qgexte_dn5, locals.var_qgexte_dn6, locals.var_qgexte_dn7, locals.var_qgexte_dn8, locals.var_qgexte_dn9, locals.var_qgexte_dn10, locals.var_qgexte_dn13, );
        locals.var_qgext_rv = 0.0;
        let assign106370_e158534: f64 = (locals.var_qgexte + locals.var_qdexte);
        let assign106370_e158536: f64 = (assign106370_e158534 + locals.var_qsexte);
        let assign106370_e158537: f64 = (-assign106370_e158536);
        (locals.var_qbext, locals.var_qbext_dn0, locals.var_qbext_dn2, locals.var_qbext_dn4, locals.var_qbext_dn5, locals.var_qbext_dn6, locals.var_qbext_dn7, locals.var_qbext_dn8, locals.var_qbext_dn9, locals.var_qbext_dn10, locals.var_qbext_dn13, ) = (assign106370_e158537, (-((locals.var_qgexte_dn0 + locals.var_qdexte_dn0) + locals.var_qsexte_dn0)), (-((locals.var_qgexte_dn2 + locals.var_qdexte_dn2) + locals.var_qsexte_dn2)), (-((locals.var_qgexte_dn4 + locals.var_qdexte_dn4) + locals.var_qsexte_dn4)), (-((locals.var_qgexte_dn5 + locals.var_qdexte_dn5) + locals.var_qsexte_dn5)), (-((locals.var_qgexte_dn6 + locals.var_qdexte_dn6) + locals.var_qsexte_dn6)), (-((locals.var_qgexte_dn7 + locals.var_qdexte_dn7) + locals.var_qsexte_dn7)), (-((locals.var_qgexte_dn8 + locals.var_qdexte_dn8) + locals.var_qsexte_dn8)), (-((locals.var_qgexte_dn9 + locals.var_qdexte_dn9) + locals.var_qsexte_dn9)), (-((locals.var_qgexte_dn10 + locals.var_qdexte_dn10) + locals.var_qsexte_dn10)), (-((locals.var_qgexte_dn13 + locals.var_qdexte_dn13) + locals.var_qsexte_dn13)), );
        locals.var_qbext_rv = 0.0;
        let assign106380_e158540: f64 = if p.p53 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2400 = assign106380_e158540;
        locals.var_guard2400_rv = 0.0;
        let assign106390_e158543: f64 = if locals.var_rth > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard2401 = assign106390_e158543;
        locals.var_guard2401_rv = 0.0;
        if ((locals.var_guard2400 != 0.0) && (locals.var_guard2401 != 0.0)) {
            let assign106400_e158549: f64 = (1.0 / locals.var_rth);
            (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn13, ) = (assign106400_e158549, (-(locals.var_rth_dn0 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn2 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn4 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn5 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn6 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn7 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn8 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn9 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn10 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn13 / (locals.var_rth * locals.var_rth))), );
            locals.var_gth_rv = 0.0;
        }
        if ((locals.var_guard2400 != 0.0) && (locals.var_guard2401 == 0.0)) {
            let assign106410_e158558: f64 = (1.0 / 0.0001);
            (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn13, ) = (assign106410_e158558, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_gth_rv = 0.0;
        }
        let assign106420_e158564: f64 = (locals.var_vdsei - locals.var_vdsi);
        let assign106420_e158565: f64 = (locals.var_vdsi * assign106420_e158564);
        let assign106420_e158567: f64 = if assign106420_e158565 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2402 = assign106420_e158567;
        locals.var_guard2402_rv = 0.0;
        let assign106430_e158570: f64 = if locals.var_uc_powrat == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2403 = assign106430_e158570;
        locals.var_guard2403_rv = 0.0;
        if (((locals.var_guard2400 != 0.0) && (locals.var_guard2402 != 0.0)) && (locals.var_guard2403 != 0.0)) {
            (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn13, ) = (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_veffpower_rv = 0.0;
        }
        if (((locals.var_guard2400 != 0.0) && (locals.var_guard2402 != 0.0)) && (locals.var_guard2403 == 0.0)) {
            let assign106450_e158589: f64 = (locals.var_vdsei - locals.var_vdsi);
            let assign106450_e158590: f64 = (locals.var_powratio * assign106450_e158589);
            let assign106450_e158591: f64 = (locals.var_vdsi + assign106450_e158590);
            (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn13, ) = (assign106450_e158591, ((locals.var_powratio_dn0 * assign106450_e158589) + (locals.var_powratio * locals.var_vdsei_dn0)), ((locals.var_powratio_dn2 * assign106450_e158589) + (locals.var_powratio * locals.var_vdsei_dn2)), (locals.var_powratio_dn4 * assign106450_e158589), (locals.var_vdsi_dn5 + ((locals.var_powratio_dn5 * assign106450_e158589) + (locals.var_powratio * (-locals.var_vdsi_dn5)))), (locals.var_powratio_dn6 * assign106450_e158589), (locals.var_vdsi_dn7 + ((locals.var_powratio_dn7 * assign106450_e158589) + (locals.var_powratio * (-locals.var_vdsi_dn7)))), (locals.var_powratio_dn8 * assign106450_e158589), (locals.var_powratio_dn9 * assign106450_e158589), (locals.var_powratio_dn10 * assign106450_e158589), (locals.var_powratio_dn13 * assign106450_e158589), );
            locals.var_veffpower_rv = 0.0;
        }
        if ((locals.var_guard2400 != 0.0) && (locals.var_guard2402 == 0.0)) {
            (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn13, ) = (locals.var_vdsi, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, locals.var_vdsi_dn7, 0.0, 0.0, 0.0, 0.0, );
            locals.var_veffpower_rv = 0.0;
        }
        if (locals.var_guard2400 != 0.0) {
            let assign106470_e158604: f64 = (locals.var_ids * locals.var_veffpower);
            (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn13, ) = (assign106470_e158604, ((locals.var_ids_dn0 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn0)), ((locals.var_ids_dn2 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn2)), ((locals.var_ids_dn4 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn4)), ((locals.var_ids_dn5 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn5)), ((locals.var_ids_dn6 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn6)), ((locals.var_ids_dn7 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn7)), ((locals.var_ids_dn8 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn8)), ((locals.var_ids_dn9 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn9)), ((locals.var_ids_dn10 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn10)), ((locals.var_ids_dn13 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn13)), );
            locals.var_p_rv = 0.0;
        }
        let assign106480_e158609: f64 = if p.p53 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2404 = assign106480_e158609;
        locals.var_guard2404_rv = 0.0;
        if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
            let assign106490_e158615: f64 = (p.p433 * locals.var_gth);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13, ) = (assign106490_e158615, (p.p433 * locals.var_gth_dn0), (p.p433 * locals.var_gth_dn2), (p.p433 * locals.var_gth_dn4), (p.p433 * locals.var_gth_dn5), (p.p433 * locals.var_gth_dn6), (p.p433 * locals.var_gth_dn7), (p.p433 * locals.var_gth_dn8), (p.p433 * locals.var_gth_dn9), (p.p433 * locals.var_gth_dn10), (p.p433 * locals.var_gth_dn13), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
            let assign106500_e158623: f64 = (locals.var_t1 - locals.var_p);
            let assign106500_e158626: f64 = (p.p337 * locals.var_gth);
            let assign106500_e158627: f64 = (assign106500_e158623 - assign106500_e158626);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13, ) = (assign106500_e158627, ((locals.var_t1_dn0 - locals.var_p_dn0) - (p.p337 * locals.var_gth_dn0)), ((locals.var_t1_dn2 - locals.var_p_dn2) - (p.p337 * locals.var_gth_dn2)), ((locals.var_t1_dn4 - locals.var_p_dn4) - (p.p337 * locals.var_gth_dn4)), ((locals.var_t1_dn5 - locals.var_p_dn5) - (p.p337 * locals.var_gth_dn5)), ((locals.var_t1_dn6 - locals.var_p_dn6) - (p.p337 * locals.var_gth_dn6)), ((locals.var_t1_dn7 - locals.var_p_dn7) - (p.p337 * locals.var_gth_dn7)), ((locals.var_t1_dn8 - locals.var_p_dn8) - (p.p337 * locals.var_gth_dn8)), ((locals.var_t1_dn9 - locals.var_p_dn9) - (p.p337 * locals.var_gth_dn9)), ((locals.var_t1_dn10 - locals.var_p_dn10) - (p.p337 * locals.var_gth_dn10)), ((locals.var_t1_dn13 - locals.var_p_dn13) - (p.p337 * locals.var_gth_dn13)), );
            locals.var_tmf1_rv = 0.0;
        }
        if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
            let assign106510_e158635: f64 = (4.0 * locals.var_t1);
            let assign106510_e158638: f64 = (p.p337 * locals.var_gth);
            let assign106510_e158639: f64 = (assign106510_e158635 * assign106510_e158638);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13, ) = (assign106510_e158639, (((4.0 * locals.var_t1_dn0) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn0))), (((4.0 * locals.var_t1_dn2) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn2))), (((4.0 * locals.var_t1_dn4) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn4))), (((4.0 * locals.var_t1_dn5) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn5))), (((4.0 * locals.var_t1_dn6) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn6))), (((4.0 * locals.var_t1_dn7) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn7))), (((4.0 * locals.var_t1_dn8) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn8))), (((4.0 * locals.var_t1_dn9) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn9))), (((4.0 * locals.var_t1_dn10) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn10))), (((4.0 * locals.var_t1_dn13) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn13))), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
            let (assign106520_e158651, assign106520_e158651_d_n0, assign106520_e158651_d_n2, assign106520_e158651_d_n4, assign106520_e158651_d_n5, assign106520_e158651_d_n6, assign106520_e158651_d_n7, assign106520_e158651_d_n8, assign106520_e158651_d_n9, assign106520_e158651_d_n10, assign106520_e158651_d_n13,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    } else {
        let assign106520_e158650: f64 = (-locals.var_tmf2);
        (assign106520_e158650, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13, ) = (assign106520_e158651, assign106520_e158651_d_n0, assign106520_e158651_d_n2, assign106520_e158651_d_n4, assign106520_e158651_d_n5, assign106520_e158651_d_n6, assign106520_e158651_d_n7, assign106520_e158651_d_n8, assign106520_e158651_d_n9, assign106520_e158651_d_n10, assign106520_e158651_d_n13, );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
            let assign106530_e158659: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign106530_e158661: f64 = (assign106530_e158659 + locals.var_tmf2);
            let assign106530_e158662: f64 = (assign106530_e158661).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13, ) = (assign106530_e158662, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign106530_e158662)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
            let assign106540_e158672: f64 = (locals.var_tmf1 / locals.var_tmf2);
            let assign106540_e158673: f64 = (1.0 + assign106540_e158672);
            let assign106540_e158674: f64 = (0.5 * assign106540_e158673);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13, ) = (assign106540_e158674, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))), );
            locals.var_t0_rv = 0.0;
        }
        if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
            let assign106550_e158684: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign106550_e158685: f64 = (0.5 * assign106550_e158684);
            let assign106550_e158686: f64 = (locals.var_t1 - assign106550_e158685);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13, ) = (assign106550_e158686, (locals.var_t1_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t1_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t1_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t1_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t1_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t1_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t1_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t1_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t1_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t1_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))), );
            locals.var_t2_rv = 0.0;
        }
        if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
            (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn13, ) = (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13, );
            locals.var_p_rv = 0.0;
        }
        if (locals.var_guard2400 == 0.0) {
            (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn13, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_gth_rv = 0.0;
            (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn13, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_p_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign106650_e158742: f64 = (locals.var_qi_nqs * locals.var_qdrat);
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn13, ) = (assign106650_e158742, (locals.var_qi_nqs * locals.var_qdrat_dn0), (locals.var_qi_nqs * locals.var_qdrat_dn2), (locals.var_qi_nqs * locals.var_qdrat_dn4), (locals.var_qi_nqs * locals.var_qdrat_dn5), (locals.var_qi_nqs * locals.var_qdrat_dn6), (locals.var_qi_nqs * locals.var_qdrat_dn7), (locals.var_qi_nqs * locals.var_qdrat_dn8), (locals.var_qi_nqs * locals.var_qdrat_dn9), (locals.var_qi_nqs * locals.var_qdrat_dn10), (locals.var_qi_nqs_dn11 * locals.var_qdrat), (locals.var_qi_nqs * locals.var_qdrat_dn13), );
            locals.var_qd_nqs_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign106660_e158747: f64 = (-locals.var_qi_nqs);
            let assign106660_e158749: f64 = (assign106660_e158747 - locals.var_qb_nqs);
            (locals.var_qg_nqs, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, ) = (assign106660_e158749, (-locals.var_qi_nqs_dn11), (-locals.var_qb_nqs_dn12), );
            locals.var_qg_nqs_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign106670_e158756: f64 = (1.0 - locals.var_qdrat);
            let assign106670_e158757: f64 = (locals.var_qi_nqs * assign106670_e158756);
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn13, ) = (assign106670_e158757, (locals.var_qi_nqs * (-locals.var_qdrat_dn0)), (locals.var_qi_nqs * (-locals.var_qdrat_dn2)), (locals.var_qi_nqs * (-locals.var_qdrat_dn4)), (locals.var_qi_nqs * (-locals.var_qdrat_dn5)), (locals.var_qi_nqs * (-locals.var_qdrat_dn6)), (locals.var_qi_nqs * (-locals.var_qdrat_dn7)), (locals.var_qi_nqs * (-locals.var_qdrat_dn8)), (locals.var_qi_nqs * (-locals.var_qdrat_dn9)), (locals.var_qi_nqs * (-locals.var_qdrat_dn10)), (locals.var_qi_nqs_dn11 * assign106670_e158756), (locals.var_qi_nqs * (-locals.var_qdrat_dn13)), );
            locals.var_qs_nqs_rv = 0.0;
        }
        if (locals.var_flg_nqs == 0.0) {
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn13, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qg_nqs, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, ) = (0.0, 0.0, 0.0, );
            locals.var_qg_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn13, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
        }
        let assign106730_e158787: f64 = (p.p87 * locals.var_mode);
        let assign106730_e158789: f64 = (assign106730_e158787 * locals.var_ids);
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn8, locals.var_idse_dn9, locals.var_idse_dn10, locals.var_idse_dn13, ) = (assign106730_e158789, (assign106730_e158787 * locals.var_ids_dn0), (assign106730_e158787 * locals.var_ids_dn2), (assign106730_e158787 * locals.var_ids_dn4), (assign106730_e158787 * locals.var_ids_dn5), (assign106730_e158787 * locals.var_ids_dn6), (assign106730_e158787 * locals.var_ids_dn7), (assign106730_e158787 * locals.var_ids_dn8), (assign106730_e158787 * locals.var_ids_dn9), (assign106730_e158787 * locals.var_ids_dn10), (assign106730_e158787 * locals.var_ids_dn13), );
        locals.var_idse_rv = 0.0;
        let assign106890_e158837: f64 = locals.var_qg_dn5;
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn13, ) = (assign106890_e158837, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgdbd_rv = 0.0;
        let assign106900_e158840: f64 = (p.p87 * locals.var_cgdbd);
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn13, ) = (assign106900_e158840, (p.p87 * locals.var_cgdbd_dn0), (p.p87 * locals.var_cgdbd_dn2), (p.p87 * locals.var_cgdbd_dn4), (p.p87 * locals.var_cgdbd_dn5), (p.p87 * locals.var_cgdbd_dn6), (p.p87 * locals.var_cgdbd_dn7), (p.p87 * locals.var_cgdbd_dn8), (p.p87 * locals.var_cgdbd_dn9), (p.p87 * locals.var_cgdbd_dn10), (p.p87 * locals.var_cgdbd_dn13), );
        locals.var_cgdbd_rv = 0.0;
        let assign106910_e158843: f64 = locals.var_qg_dn7;
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn13, ) = (assign106910_e158843, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgsbd_rv = 0.0;
        let assign106920_e158846: f64 = (p.p87 * locals.var_cgsbd);
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn13, ) = (assign106920_e158846, (p.p87 * locals.var_cgsbd_dn0), (p.p87 * locals.var_cgsbd_dn2), (p.p87 * locals.var_cgsbd_dn4), (p.p87 * locals.var_cgsbd_dn5), (p.p87 * locals.var_cgsbd_dn6), (p.p87 * locals.var_cgsbd_dn7), (p.p87 * locals.var_cgsbd_dn8), (p.p87 * locals.var_cgsbd_dn9), (p.p87 * locals.var_cgsbd_dn10), (p.p87 * locals.var_cgsbd_dn13), );
        locals.var_cgsbd_rv = 0.0;
        let assign107290_e158961: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2407 = assign107290_e158961;
        locals.var_guard2407_rv = 0.0;
        if (locals.var_guard2407 != 0.0) {
            (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn13, ) = (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn13, );
            locals.var_cgsb_rv = 0.0;
        }
        if (locals.var_guard2407 == 0.0) {
            (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn13, ) = (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn13, );
            locals.var_cgsb_rv = 0.0;
        }
        let assign107650_e159080: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2409 = assign107650_e159080;
        locals.var_guard2409_rv = 0.0;
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
        let (eq0_e1018, eq0_e1018_d_n0, eq0_e1018_d_n2, eq0_e1018_d_n4, eq0_e1018_d_n5, eq0_e1018_d_n6, eq0_e1018_d_n7, eq0_e1018_d_n8, eq0_e1018_d_n9, eq0_e1018_d_n10, eq0_e1018_d_n13, eq0_e1018_d_n15,) = {
    if (locals.var_guard2309 != 0.0) {
        let eq0_e1015: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_q_nqs_a);
        let eq0_e1016: f64 = (locals.var_inqs0_a + eq0_e1015);
        let eq0_e1016_d_n15: f64 = (locals.var_inqs0_a_dn15 + (locals.var_q_nqs_a_dn15 * ddt_scale));
        (eq0_e1016, locals.var_inqs0_a_dn0, locals.var_inqs0_a_dn2, locals.var_inqs0_a_dn4, locals.var_inqs0_a_dn5, locals.var_inqs0_a_dn6, locals.var_inqs0_a_dn7, locals.var_inqs0_a_dn8, locals.var_inqs0_a_dn9, locals.var_inqs0_a_dn10, locals.var_inqs0_a_dn13, eq0_e1016_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e1018;
        let eq0_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 13, 15];
        let eq0_node_derivatives: [f64; 11] = [eq0_e1018_d_n0, eq0_e1018_d_n2, eq0_e1018_d_n4, eq0_e1018_d_n5, eq0_e1018_d_n6, eq0_e1018_d_n7, eq0_e1018_d_n8, eq0_e1018_d_n9, eq0_e1018_d_n10, eq0_e1018_d_n13, eq0_e1018_d_n15];
        let eq0_branch_derivative_indices: [usize; 0] = [];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq0_value),
            &eq0_node_derivative_indices,
            &eq0_node_derivatives,
            &eq0_branch_derivative_indices,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e1025, eq1_e1025_d_n0, eq1_e1025_d_n2, eq1_e1025_d_n4, eq1_e1025_d_n5, eq1_e1025_d_n6, eq1_e1025_d_n7, eq1_e1025_d_n8, eq1_e1025_d_n9, eq1_e1025_d_n10, eq1_e1025_d_n13, eq1_e1025_d_n16,) = {
    if (locals.var_guard2309 != 0.0) {
        let eq1_e1022: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_q_nqs_k);
        let eq1_e1023: f64 = (locals.var_inqs0_k + eq1_e1022);
        let eq1_e1023_d_n16: f64 = (locals.var_inqs0_k_dn16 + (locals.var_q_nqs_k_dn16 * ddt_scale));
        (eq1_e1023, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn13, eq1_e1023_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1025;
        let eq1_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 13, 16];
        let eq1_node_derivatives: [f64; 11] = [eq1_e1025_d_n0, eq1_e1025_d_n2, eq1_e1025_d_n4, eq1_e1025_d_n5, eq1_e1025_d_n6, eq1_e1025_d_n7, eq1_e1025_d_n8, eq1_e1025_d_n9, eq1_e1025_d_n10, eq1_e1025_d_n13, eq1_e1025_d_n16];
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
        let (eq4_e1042, eq4_e1042_d_n0, eq4_e1042_d_n2, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n13, eq4_e1042_d_n17,) = {
    if (locals.var_guard2310 != 0.0) {
        let eq4_e1039: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_w_nqs_a);
        let eq4_e1040: f64 = (locals.var_iwnqs0_a + eq4_e1039);
        let eq4_e1040_d_n17: f64 = (locals.var_iwnqs0_a_dn17 + (locals.var_w_nqs_a_dn17 * ddt_scale));
        (eq4_e1040, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn13, eq4_e1040_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1042;
        let eq4_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 13, 17];
        let eq4_node_derivatives: [f64; 11] = [eq4_e1042_d_n0, eq4_e1042_d_n2, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n13, eq4_e1042_d_n17];
        let eq4_branch_derivative_indices: [usize; 0] = [];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(17),
            None,
            multiplicity * (eq4_value),
            &eq4_node_derivative_indices,
            &eq4_node_derivatives,
            &eq4_branch_derivative_indices,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let eq6_e1051: f64 = (locals.var_ids + locals.var_idsibpc);
        let eq6_e1051_d_n0: f64 = (locals.var_ids_dn0 + locals.var_idsibpc_dn0);
        let eq6_e1051_d_n2: f64 = (locals.var_ids_dn2 + locals.var_idsibpc_dn2);
        let eq6_e1051_d_n4: f64 = (locals.var_ids_dn4 + locals.var_idsibpc_dn4);
        let eq6_e1051_d_n5: f64 = (locals.var_ids_dn5 + locals.var_idsibpc_dn5);
        let eq6_e1051_d_n6: f64 = (locals.var_ids_dn6 + locals.var_idsibpc_dn6);
        let eq6_e1051_d_n7: f64 = (locals.var_ids_dn7 + locals.var_idsibpc_dn7);
        let eq6_e1051_d_n8: f64 = (locals.var_ids_dn8 + locals.var_idsibpc_dn8);
        let eq6_e1051_d_n9: f64 = (locals.var_ids_dn9 + locals.var_idsibpc_dn9);
        let eq6_e1051_d_n10: f64 = (locals.var_ids_dn10 + locals.var_idsibpc_dn10);
        let eq6_e1051_d_n13: f64 = (locals.var_ids_dn13 + locals.var_idsibpc_dn13);
        let eq6_e1053: f64 = (eq6_e1051 - locals.var_idsibpcs);
        let eq6_e1053_d_n0: f64 = (eq6_e1051_d_n0 - locals.var_idsibpcs_dn0);
        let eq6_e1053_d_n2: f64 = (eq6_e1051_d_n2 - locals.var_idsibpcs_dn2);
        let eq6_e1053_d_n4: f64 = (eq6_e1051_d_n4 - locals.var_idsibpcs_dn4);
        let eq6_e1053_d_n5: f64 = (eq6_e1051_d_n5 - locals.var_idsibpcs_dn5);
        let eq6_e1053_d_n6: f64 = (eq6_e1051_d_n6 - locals.var_idsibpcs_dn6);
        let eq6_e1053_d_n7: f64 = (eq6_e1051_d_n7 - locals.var_idsibpcs_dn7);
        let eq6_e1053_d_n8: f64 = (eq6_e1051_d_n8 - locals.var_idsibpcs_dn8);
        let eq6_e1053_d_n9: f64 = (eq6_e1051_d_n9 - locals.var_idsibpcs_dn9);
        let eq6_e1053_d_n10: f64 = (eq6_e1051_d_n10 - locals.var_idsibpcs_dn10);
        let eq6_e1053_d_n13: f64 = (eq6_e1051_d_n13 - locals.var_idsibpcs_dn13);
        let eq6_e1054: f64 = (p.p87 * eq6_e1053);
        let eq6_e1054_d_n0: f64 = (p.p87 * eq6_e1053_d_n0);
        let eq6_e1054_d_n2: f64 = (p.p87 * eq6_e1053_d_n2);
        let eq6_e1054_d_n4: f64 = (p.p87 * eq6_e1053_d_n4);
        let eq6_e1054_d_n5: f64 = (p.p87 * eq6_e1053_d_n5);
        let eq6_e1054_d_n6: f64 = (p.p87 * eq6_e1053_d_n6);
        let eq6_e1054_d_n7: f64 = (p.p87 * eq6_e1053_d_n7);
        let eq6_e1054_d_n8: f64 = (p.p87 * eq6_e1053_d_n8);
        let eq6_e1054_d_n9: f64 = (p.p87 * eq6_e1053_d_n9);
        let eq6_e1054_d_n10: f64 = (p.p87 * eq6_e1053_d_n10);
        let eq6_e1054_d_n13: f64 = (p.p87 * eq6_e1053_d_n13);
        let eq6_value: f64 = eq6_e1054;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq6_e1054_d_n0), multiplicity * (eq6_e1054_d_n2), multiplicity * (eq6_e1054_d_n4), multiplicity * (eq6_e1054_d_n5), multiplicity * (eq6_e1054_d_n6), multiplicity * (eq6_e1054_d_n7), multiplicity * (eq6_e1054_d_n8), multiplicity * (eq6_e1054_d_n9), multiplicity * (eq6_e1054_d_n10), multiplicity * (eq6_e1054_d_n13)],
            [],
            [],
            1.0,
        );
        let eq7_e1058: f64 = (locals.var_ibreak - locals.var_ibreaks);
        let eq7_e1058_d_n0: f64 = (locals.var_ibreak_dn0 - locals.var_ibreaks_dn0);
        let eq7_e1058_d_n2: f64 = (locals.var_ibreak_dn2 - locals.var_ibreaks_dn2);
        let eq7_e1058_d_n4: f64 = (locals.var_ibreak_dn4 - locals.var_ibreaks_dn4);
        let eq7_e1058_d_n5: f64 = (locals.var_ibreak_dn5 - locals.var_ibreaks_dn5);
        let eq7_e1058_d_n6: f64 = (locals.var_ibreak_dn6 - locals.var_ibreaks_dn6);
        let eq7_e1058_d_n7: f64 = (locals.var_ibreak_dn7 - locals.var_ibreaks_dn7);
        let eq7_e1058_d_n8: f64 = (locals.var_ibreak_dn8 - locals.var_ibreaks_dn8);
        let eq7_e1058_d_n9: f64 = (locals.var_ibreak_dn9 - locals.var_ibreaks_dn9);
        let eq7_e1058_d_n10: f64 = (locals.var_ibreak_dn10 - locals.var_ibreaks_dn10);
        let eq7_e1058_d_n13: f64 = (locals.var_ibreak_dn13 - locals.var_ibreaks_dn13);
        let eq7_e1059: f64 = (p.p87 * eq7_e1058);
        let eq7_e1059_d_n0: f64 = (p.p87 * eq7_e1058_d_n0);
        let eq7_e1059_d_n2: f64 = (p.p87 * eq7_e1058_d_n2);
        let eq7_e1059_d_n4: f64 = (p.p87 * eq7_e1058_d_n4);
        let eq7_e1059_d_n5: f64 = (p.p87 * eq7_e1058_d_n5);
        let eq7_e1059_d_n6: f64 = (p.p87 * eq7_e1058_d_n6);
        let eq7_e1059_d_n7: f64 = (p.p87 * eq7_e1058_d_n7);
        let eq7_e1059_d_n8: f64 = (p.p87 * eq7_e1058_d_n8);
        let eq7_e1059_d_n9: f64 = (p.p87 * eq7_e1058_d_n9);
        let eq7_e1059_d_n10: f64 = (p.p87 * eq7_e1058_d_n10);
        let eq7_e1059_d_n13: f64 = (p.p87 * eq7_e1058_d_n13);
        let eq7_value: f64 = eq7_e1059;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(0),
            Some(2),
            multiplicity * (eq7_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq7_e1059_d_n0), multiplicity * (eq7_e1059_d_n2), multiplicity * (eq7_e1059_d_n4), multiplicity * (eq7_e1059_d_n5), multiplicity * (eq7_e1059_d_n6), multiplicity * (eq7_e1059_d_n7), multiplicity * (eq7_e1059_d_n8), multiplicity * (eq7_e1059_d_n9), multiplicity * (eq7_e1059_d_n10), multiplicity * (eq7_e1059_d_n13)],
            [],
            [],
            1.0,
        );
        let eq8_e1063: f64 = (locals.var_igidl + locals.var_isub);
        let eq8_e1063_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq8_e1063_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq8_e1063_d_n4: f64 = (locals.var_igidl_dn4 + locals.var_isub_dn4);
        let eq8_e1063_d_n5: f64 = (locals.var_igidl_dn5 + locals.var_isub_dn5);
        let eq8_e1063_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq8_e1063_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq8_e1063_d_n8: f64 = (locals.var_igidl_dn8 + locals.var_isub_dn8);
        let eq8_e1063_d_n9: f64 = (locals.var_igidl_dn9 + locals.var_isub_dn9);
        let eq8_e1063_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq8_e1063_d_n13: f64 = (locals.var_igidl_dn13 + locals.var_isub_dn13);
        let eq8_e1065: f64 = (eq8_e1063 + locals.var_ibjt);
        let eq8_e1065_d_n0: f64 = (eq8_e1063_d_n0 + locals.var_ibjt_dn0);
        let eq8_e1065_d_n2: f64 = (eq8_e1063_d_n2 + locals.var_ibjt_dn2);
        let eq8_e1065_d_n4: f64 = (eq8_e1063_d_n4 + locals.var_ibjt_dn4);
        let eq8_e1065_d_n5: f64 = (eq8_e1063_d_n5 + locals.var_ibjt_dn5);
        let eq8_e1065_d_n6: f64 = (eq8_e1063_d_n6 + locals.var_ibjt_dn6);
        let eq8_e1065_d_n7: f64 = (eq8_e1063_d_n7 + locals.var_ibjt_dn7);
        let eq8_e1065_d_n8: f64 = (eq8_e1063_d_n8 + locals.var_ibjt_dn8);
        let eq8_e1065_d_n9: f64 = (eq8_e1063_d_n9 + locals.var_ibjt_dn9);
        let eq8_e1065_d_n10: f64 = (eq8_e1063_d_n10 + locals.var_ibjt_dn10);
        let eq8_e1065_d_n13: f64 = (eq8_e1063_d_n13 + locals.var_ibjt_dn13);
        let eq8_e1066: f64 = (p.p87 * eq8_e1065);
        let eq8_e1066_d_n0: f64 = (p.p87 * eq8_e1065_d_n0);
        let eq8_e1066_d_n2: f64 = (p.p87 * eq8_e1065_d_n2);
        let eq8_e1066_d_n4: f64 = (p.p87 * eq8_e1065_d_n4);
        let eq8_e1066_d_n5: f64 = (p.p87 * eq8_e1065_d_n5);
        let eq8_e1066_d_n6: f64 = (p.p87 * eq8_e1065_d_n6);
        let eq8_e1066_d_n7: f64 = (p.p87 * eq8_e1065_d_n7);
        let eq8_e1066_d_n8: f64 = (p.p87 * eq8_e1065_d_n8);
        let eq8_e1066_d_n9: f64 = (p.p87 * eq8_e1065_d_n9);
        let eq8_e1066_d_n10: f64 = (p.p87 * eq8_e1065_d_n10);
        let eq8_e1066_d_n13: f64 = (p.p87 * eq8_e1065_d_n13);
        let eq8_value: f64 = eq8_e1066;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq8_e1066_d_n0), multiplicity * (eq8_e1066_d_n2), multiplicity * (eq8_e1066_d_n4), multiplicity * (eq8_e1066_d_n5), multiplicity * (eq8_e1066_d_n6), multiplicity * (eq8_e1066_d_n7), multiplicity * (eq8_e1066_d_n8), multiplicity * (eq8_e1066_d_n9), multiplicity * (eq8_e1066_d_n10), multiplicity * (eq8_e1066_d_n13)],
            [],
            [],
            1.0,
        );
        let eq9_e1070: f64 = (locals.var_igisl + locals.var_isubs);
        let eq9_e1070_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq9_e1070_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq9_e1070_d_n4: f64 = (locals.var_igisl_dn4 + locals.var_isubs_dn4);
        let eq9_e1070_d_n5: f64 = (locals.var_igisl_dn5 + locals.var_isubs_dn5);
        let eq9_e1070_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq9_e1070_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq9_e1070_d_n8: f64 = (locals.var_igisl_dn8 + locals.var_isubs_dn8);
        let eq9_e1070_d_n9: f64 = (locals.var_igisl_dn9 + locals.var_isubs_dn9);
        let eq9_e1070_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq9_e1070_d_n13: f64 = (locals.var_igisl_dn13 + locals.var_isubs_dn13);
        let eq9_e1072: f64 = (eq9_e1070 + locals.var_ibjts);
        let eq9_e1072_d_n0: f64 = (eq9_e1070_d_n0 + locals.var_ibjts_dn0);
        let eq9_e1072_d_n2: f64 = (eq9_e1070_d_n2 + locals.var_ibjts_dn2);
        let eq9_e1072_d_n4: f64 = (eq9_e1070_d_n4 + locals.var_ibjts_dn4);
        let eq9_e1072_d_n5: f64 = (eq9_e1070_d_n5 + locals.var_ibjts_dn5);
        let eq9_e1072_d_n6: f64 = (eq9_e1070_d_n6 + locals.var_ibjts_dn6);
        let eq9_e1072_d_n7: f64 = (eq9_e1070_d_n7 + locals.var_ibjts_dn7);
        let eq9_e1072_d_n8: f64 = (eq9_e1070_d_n8 + locals.var_ibjts_dn8);
        let eq9_e1072_d_n9: f64 = (eq9_e1070_d_n9 + locals.var_ibjts_dn9);
        let eq9_e1072_d_n10: f64 = (eq9_e1070_d_n10 + locals.var_ibjts_dn10);
        let eq9_e1072_d_n13: f64 = (eq9_e1070_d_n13 + locals.var_ibjts_dn13);
        let eq9_e1073: f64 = (p.p87 * eq9_e1072);
        let eq9_e1073_d_n0: f64 = (p.p87 * eq9_e1072_d_n0);
        let eq9_e1073_d_n2: f64 = (p.p87 * eq9_e1072_d_n2);
        let eq9_e1073_d_n4: f64 = (p.p87 * eq9_e1072_d_n4);
        let eq9_e1073_d_n5: f64 = (p.p87 * eq9_e1072_d_n5);
        let eq9_e1073_d_n6: f64 = (p.p87 * eq9_e1072_d_n6);
        let eq9_e1073_d_n7: f64 = (p.p87 * eq9_e1072_d_n7);
        let eq9_e1073_d_n8: f64 = (p.p87 * eq9_e1072_d_n8);
        let eq9_e1073_d_n9: f64 = (p.p87 * eq9_e1072_d_n9);
        let eq9_e1073_d_n10: f64 = (p.p87 * eq9_e1072_d_n10);
        let eq9_e1073_d_n13: f64 = (p.p87 * eq9_e1072_d_n13);
        let eq9_value: f64 = eq9_e1073;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq9_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq9_e1073_d_n0), multiplicity * (eq9_e1073_d_n2), multiplicity * (eq9_e1073_d_n4), multiplicity * (eq9_e1073_d_n5), multiplicity * (eq9_e1073_d_n6), multiplicity * (eq9_e1073_d_n7), multiplicity * (eq9_e1073_d_n8), multiplicity * (eq9_e1073_d_n9), multiplicity * (eq9_e1073_d_n10), multiplicity * (eq9_e1073_d_n13)],
            [],
            [],
            1.0,
        );
        let eq10_e1076: f64 = (p.p87 * locals.var_isubld);
        let eq10_e1076_d_n0: f64 = (p.p87 * locals.var_isubld_dn0);
        let eq10_e1076_d_n2: f64 = (p.p87 * locals.var_isubld_dn2);
        let eq10_e1076_d_n4: f64 = (p.p87 * locals.var_isubld_dn4);
        let eq10_e1076_d_n5: f64 = (p.p87 * locals.var_isubld_dn5);
        let eq10_e1076_d_n6: f64 = (p.p87 * locals.var_isubld_dn6);
        let eq10_e1076_d_n7: f64 = (p.p87 * locals.var_isubld_dn7);
        let eq10_e1076_d_n8: f64 = (p.p87 * locals.var_isubld_dn8);
        let eq10_e1076_d_n9: f64 = (p.p87 * locals.var_isubld_dn9);
        let eq10_e1076_d_n10: f64 = (p.p87 * locals.var_isubld_dn10);
        let eq10_e1076_d_n13: f64 = (p.p87 * locals.var_isubld_dn13);
        let eq10_value: f64 = eq10_e1076;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(0),
            Some(8),
            multiplicity * (eq10_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq10_e1076_d_n0), multiplicity * (eq10_e1076_d_n2), multiplicity * (eq10_e1076_d_n4), multiplicity * (eq10_e1076_d_n5), multiplicity * (eq10_e1076_d_n6), multiplicity * (eq10_e1076_d_n7), multiplicity * (eq10_e1076_d_n8), multiplicity * (eq10_e1076_d_n9), multiplicity * (eq10_e1076_d_n10), multiplicity * (eq10_e1076_d_n13)],
            [],
            [],
            1.0,
        );
        let eq11_e1079: f64 = (p.p87 * locals.var_isublds);
        let eq11_e1079_d_n0: f64 = (p.p87 * locals.var_isublds_dn0);
        let eq11_e1079_d_n2: f64 = (p.p87 * locals.var_isublds_dn2);
        let eq11_e1079_d_n4: f64 = (p.p87 * locals.var_isublds_dn4);
        let eq11_e1079_d_n5: f64 = (p.p87 * locals.var_isublds_dn5);
        let eq11_e1079_d_n6: f64 = (p.p87 * locals.var_isublds_dn6);
        let eq11_e1079_d_n7: f64 = (p.p87 * locals.var_isublds_dn7);
        let eq11_e1079_d_n8: f64 = (p.p87 * locals.var_isublds_dn8);
        let eq11_e1079_d_n9: f64 = (p.p87 * locals.var_isublds_dn9);
        let eq11_e1079_d_n10: f64 = (p.p87 * locals.var_isublds_dn10);
        let eq11_e1079_d_n13: f64 = (p.p87 * locals.var_isublds_dn13);
        let eq11_value: f64 = eq11_e1079;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(2),
            Some(8),
            multiplicity * (eq11_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq11_e1079_d_n0), multiplicity * (eq11_e1079_d_n2), multiplicity * (eq11_e1079_d_n4), multiplicity * (eq11_e1079_d_n5), multiplicity * (eq11_e1079_d_n6), multiplicity * (eq11_e1079_d_n7), multiplicity * (eq11_e1079_d_n8), multiplicity * (eq11_e1079_d_n9), multiplicity * (eq11_e1079_d_n10), multiplicity * (eq11_e1079_d_n13)],
            [],
            [],
            1.0,
        );
        let eq12_e1082: f64 = (p.p87 * locals.var_ibs);
        let eq12_e1082_d_n0: f64 = (p.p87 * locals.var_ibs_dn0);
        let eq12_e1082_d_n2: f64 = (p.p87 * locals.var_ibs_dn2);
        let eq12_e1082_d_n4: f64 = (p.p87 * locals.var_ibs_dn4);
        let eq12_e1082_d_n5: f64 = (p.p87 * locals.var_ibs_dn5);
        let eq12_e1082_d_n6: f64 = (p.p87 * locals.var_ibs_dn6);
        let eq12_e1082_d_n7: f64 = (p.p87 * locals.var_ibs_dn7);
        let eq12_e1082_d_n8: f64 = (p.p87 * locals.var_ibs_dn8);
        let eq12_e1082_d_n9: f64 = (p.p87 * locals.var_ibs_dn9);
        let eq12_e1082_d_n10: f64 = (p.p87 * locals.var_ibs_dn10);
        let eq12_e1082_d_n13: f64 = (p.p87 * locals.var_ibs_dn13);
        let eq12_value: f64 = eq12_e1082;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(2),
            multiplicity * (eq12_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq12_e1082_d_n0), multiplicity * (eq12_e1082_d_n2), multiplicity * (eq12_e1082_d_n4), multiplicity * (eq12_e1082_d_n5), multiplicity * (eq12_e1082_d_n6), multiplicity * (eq12_e1082_d_n7), multiplicity * (eq12_e1082_d_n8), multiplicity * (eq12_e1082_d_n9), multiplicity * (eq12_e1082_d_n10), multiplicity * (eq12_e1082_d_n13)],
            [],
            [],
            1.0,
        );
        let eq13_e1085: f64 = (p.p87 * locals.var_ibd);
        let eq13_e1085_d_n0: f64 = (p.p87 * locals.var_ibd_dn0);
        let eq13_e1085_d_n2: f64 = (p.p87 * locals.var_ibd_dn2);
        let eq13_e1085_d_n4: f64 = (p.p87 * locals.var_ibd_dn4);
        let eq13_e1085_d_n5: f64 = (p.p87 * locals.var_ibd_dn5);
        let eq13_e1085_d_n6: f64 = (p.p87 * locals.var_ibd_dn6);
        let eq13_e1085_d_n7: f64 = (p.p87 * locals.var_ibd_dn7);
        let eq13_e1085_d_n8: f64 = (p.p87 * locals.var_ibd_dn8);
        let eq13_e1085_d_n9: f64 = (p.p87 * locals.var_ibd_dn9);
        let eq13_e1085_d_n10: f64 = (p.p87 * locals.var_ibd_dn10);
        let eq13_e1085_d_n13: f64 = (p.p87 * locals.var_ibd_dn13);
        let eq13_value: f64 = eq13_e1085;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(0),
            multiplicity * (eq13_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq13_e1085_d_n0), multiplicity * (eq13_e1085_d_n2), multiplicity * (eq13_e1085_d_n4), multiplicity * (eq13_e1085_d_n5), multiplicity * (eq13_e1085_d_n6), multiplicity * (eq13_e1085_d_n7), multiplicity * (eq13_e1085_d_n8), multiplicity * (eq13_e1085_d_n9), multiplicity * (eq13_e1085_d_n10), multiplicity * (eq13_e1085_d_n13)],
            [],
            [],
            1.0,
        );
        let eq14_e1088: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qbs);
        let eq14_e1089: f64 = (p.p87 * eq14_e1088);
        let eq14_e1089_d_n0: f64 = (p.p87 * (locals.var_qbs_dn0 * ddt_scale));
        let eq14_e1089_d_n2: f64 = (p.p87 * (locals.var_qbs_dn2 * ddt_scale));
        let eq14_e1089_d_n4: f64 = (p.p87 * (locals.var_qbs_dn4 * ddt_scale));
        let eq14_e1089_d_n5: f64 = (p.p87 * (locals.var_qbs_dn5 * ddt_scale));
        let eq14_e1089_d_n6: f64 = (p.p87 * (locals.var_qbs_dn6 * ddt_scale));
        let eq14_e1089_d_n7: f64 = (p.p87 * (locals.var_qbs_dn7 * ddt_scale));
        let eq14_e1089_d_n8: f64 = (p.p87 * (locals.var_qbs_dn8 * ddt_scale));
        let eq14_e1089_d_n9: f64 = (p.p87 * (locals.var_qbs_dn9 * ddt_scale));
        let eq14_e1089_d_n10: f64 = (p.p87 * (locals.var_qbs_dn10 * ddt_scale));
        let eq14_e1089_d_n13: f64 = (p.p87 * (locals.var_qbs_dn13 * ddt_scale));
        let eq14_value: f64 = eq14_e1089;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(2),
            multiplicity * (eq14_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq14_e1089_d_n0), multiplicity * (eq14_e1089_d_n2), multiplicity * (eq14_e1089_d_n4), multiplicity * (eq14_e1089_d_n5), multiplicity * (eq14_e1089_d_n6), multiplicity * (eq14_e1089_d_n7), multiplicity * (eq14_e1089_d_n8), multiplicity * (eq14_e1089_d_n9), multiplicity * (eq14_e1089_d_n10), multiplicity * (eq14_e1089_d_n13)],
            [],
            [],
            1.0,
        );
        let eq15_e1092: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, locals.var_qbd);
        let eq15_e1093: f64 = (p.p87 * eq15_e1092);
        let eq15_e1093_d_n0: f64 = (p.p87 * (locals.var_qbd_dn0 * ddt_scale));
        let eq15_e1093_d_n2: f64 = (p.p87 * (locals.var_qbd_dn2 * ddt_scale));
        let eq15_e1093_d_n4: f64 = (p.p87 * (locals.var_qbd_dn4 * ddt_scale));
        let eq15_e1093_d_n5: f64 = (p.p87 * (locals.var_qbd_dn5 * ddt_scale));
        let eq15_e1093_d_n6: f64 = (p.p87 * (locals.var_qbd_dn6 * ddt_scale));
        let eq15_e1093_d_n7: f64 = (p.p87 * (locals.var_qbd_dn7 * ddt_scale));
        let eq15_e1093_d_n8: f64 = (p.p87 * (locals.var_qbd_dn8 * ddt_scale));
        let eq15_e1093_d_n9: f64 = (p.p87 * (locals.var_qbd_dn9 * ddt_scale));
        let eq15_e1093_d_n10: f64 = (p.p87 * (locals.var_qbd_dn10 * ddt_scale));
        let eq15_e1093_d_n13: f64 = (p.p87 * (locals.var_qbd_dn13 * ddt_scale));
        let eq15_e1093_d_n15: f64 = (p.p87 * (locals.var_qbd_dn15 * ddt_scale));
        let eq15_e1093_d_n16: f64 = (p.p87 * (locals.var_qbd_dn16 * ddt_scale));
        let eq15_e1093_d_n17: f64 = (p.p87 * (locals.var_qbd_dn17 * ddt_scale));
        let eq15_value: f64 = eq15_e1093;
        let eq15_node_derivative_indices: [usize; 13] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 13, 15, 16, 17];
        let eq15_node_derivatives: [f64; 13] = [eq15_e1093_d_n0, eq15_e1093_d_n2, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n13, eq15_e1093_d_n15, eq15_e1093_d_n16, eq15_e1093_d_n17];
        let eq15_branch_derivative_indices: [usize; 0] = [];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(0),
            multiplicity * (eq15_value),
            &eq15_node_derivative_indices,
            &eq15_node_derivatives,
            &eq15_branch_derivative_indices,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq16_e1099, eq16_e1099_d_n0, eq16_e1099_d_n2, eq16_e1099_d_n4, eq16_e1099_d_n5, eq16_e1099_d_n6, eq16_e1099_d_n7, eq16_e1099_d_n8, eq16_e1099_d_n9, eq16_e1099_d_n10, eq16_e1099_d_n13,) = {
    if (locals.var_guard2409 != 0.0) {
        let eq16_e1097: f64 = (p.p87 * locals.var_ibsi);
        let eq16_e1097_d_n0: f64 = (p.p87 * locals.var_ibsi_dn0);
        let eq16_e1097_d_n2: f64 = (p.p87 * locals.var_ibsi_dn2);
        let eq16_e1097_d_n4: f64 = (p.p87 * locals.var_ibsi_dn4);
        let eq16_e1097_d_n5: f64 = (p.p87 * locals.var_ibsi_dn5);
        let eq16_e1097_d_n6: f64 = (p.p87 * locals.var_ibsi_dn6);
        let eq16_e1097_d_n7: f64 = (p.p87 * locals.var_ibsi_dn7);
        let eq16_e1097_d_n8: f64 = (p.p87 * locals.var_ibsi_dn8);
        let eq16_e1097_d_n9: f64 = (p.p87 * locals.var_ibsi_dn9);
        let eq16_e1097_d_n10: f64 = (p.p87 * locals.var_ibsi_dn10);
        let eq16_e1097_d_n13: f64 = (p.p87 * locals.var_ibsi_dn13);
        (eq16_e1097, eq16_e1097_d_n0, eq16_e1097_d_n2, eq16_e1097_d_n4, eq16_e1097_d_n5, eq16_e1097_d_n6, eq16_e1097_d_n7, eq16_e1097_d_n8, eq16_e1097_d_n9, eq16_e1097_d_n10, eq16_e1097_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e1099;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq16_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq16_e1099_d_n0), multiplicity * (eq16_e1099_d_n2), multiplicity * (eq16_e1099_d_n4), multiplicity * (eq16_e1099_d_n5), multiplicity * (eq16_e1099_d_n6), multiplicity * (eq16_e1099_d_n7), multiplicity * (eq16_e1099_d_n8), multiplicity * (eq16_e1099_d_n9), multiplicity * (eq16_e1099_d_n10), multiplicity * (eq16_e1099_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq17_e1105, eq17_e1105_d_n0, eq17_e1105_d_n2, eq17_e1105_d_n4, eq17_e1105_d_n5, eq17_e1105_d_n6, eq17_e1105_d_n7, eq17_e1105_d_n8, eq17_e1105_d_n9, eq17_e1105_d_n10, eq17_e1105_d_n13,) = {
    if (locals.var_guard2409 != 0.0) {
        let eq17_e1103: f64 = (p.p87 * locals.var_ibdi);
        let eq17_e1103_d_n0: f64 = (p.p87 * locals.var_ibdi_dn0);
        let eq17_e1103_d_n2: f64 = (p.p87 * locals.var_ibdi_dn2);
        let eq17_e1103_d_n4: f64 = (p.p87 * locals.var_ibdi_dn4);
        let eq17_e1103_d_n5: f64 = (p.p87 * locals.var_ibdi_dn5);
        let eq17_e1103_d_n6: f64 = (p.p87 * locals.var_ibdi_dn6);
        let eq17_e1103_d_n7: f64 = (p.p87 * locals.var_ibdi_dn7);
        let eq17_e1103_d_n8: f64 = (p.p87 * locals.var_ibdi_dn8);
        let eq17_e1103_d_n9: f64 = (p.p87 * locals.var_ibdi_dn9);
        let eq17_e1103_d_n10: f64 = (p.p87 * locals.var_ibdi_dn10);
        let eq17_e1103_d_n13: f64 = (p.p87 * locals.var_ibdi_dn13);
        (eq17_e1103, eq17_e1103_d_n0, eq17_e1103_d_n2, eq17_e1103_d_n4, eq17_e1103_d_n5, eq17_e1103_d_n6, eq17_e1103_d_n7, eq17_e1103_d_n8, eq17_e1103_d_n9, eq17_e1103_d_n10, eq17_e1103_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1105;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq17_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq17_e1105_d_n0), multiplicity * (eq17_e1105_d_n2), multiplicity * (eq17_e1105_d_n4), multiplicity * (eq17_e1105_d_n5), multiplicity * (eq17_e1105_d_n6), multiplicity * (eq17_e1105_d_n7), multiplicity * (eq17_e1105_d_n8), multiplicity * (eq17_e1105_d_n9), multiplicity * (eq17_e1105_d_n10), multiplicity * (eq17_e1105_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq18_e1112, eq18_e1112_d_n0, eq18_e1112_d_n2, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n13,) = {
    if (locals.var_guard2409 != 0.0) {
        let eq18_e1109: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, locals.var_qbsi);
        let eq18_e1110: f64 = (p.p87 * eq18_e1109);
        let eq18_e1110_d_n0: f64 = (p.p87 * (locals.var_qbsi_dn0 * ddt_scale));
        let eq18_e1110_d_n2: f64 = (p.p87 * (locals.var_qbsi_dn2 * ddt_scale));
        let eq18_e1110_d_n4: f64 = (p.p87 * (locals.var_qbsi_dn4 * ddt_scale));
        let eq18_e1110_d_n5: f64 = (p.p87 * (locals.var_qbsi_dn5 * ddt_scale));
        let eq18_e1110_d_n6: f64 = (p.p87 * (locals.var_qbsi_dn6 * ddt_scale));
        let eq18_e1110_d_n7: f64 = (p.p87 * (locals.var_qbsi_dn7 * ddt_scale));
        let eq18_e1110_d_n8: f64 = (p.p87 * (locals.var_qbsi_dn8 * ddt_scale));
        let eq18_e1110_d_n9: f64 = (p.p87 * (locals.var_qbsi_dn9 * ddt_scale));
        let eq18_e1110_d_n10: f64 = (p.p87 * (locals.var_qbsi_dn10 * ddt_scale));
        let eq18_e1110_d_n13: f64 = (p.p87 * (locals.var_qbsi_dn13 * ddt_scale));
        (eq18_e1110, eq18_e1110_d_n0, eq18_e1110_d_n2, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1112;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq18_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq18_e1112_d_n0), multiplicity * (eq18_e1112_d_n2), multiplicity * (eq18_e1112_d_n4), multiplicity * (eq18_e1112_d_n5), multiplicity * (eq18_e1112_d_n6), multiplicity * (eq18_e1112_d_n7), multiplicity * (eq18_e1112_d_n8), multiplicity * (eq18_e1112_d_n9), multiplicity * (eq18_e1112_d_n10), multiplicity * (eq18_e1112_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq19_e1119, eq19_e1119_d_n0, eq19_e1119_d_n2, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n13,) = {
    if (locals.var_guard2409 != 0.0) {
        let eq19_e1116: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qbdi);
        let eq19_e1117: f64 = (p.p87 * eq19_e1116);
        let eq19_e1117_d_n0: f64 = (p.p87 * (locals.var_qbdi_dn0 * ddt_scale));
        let eq19_e1117_d_n2: f64 = (p.p87 * (locals.var_qbdi_dn2 * ddt_scale));
        let eq19_e1117_d_n4: f64 = (p.p87 * (locals.var_qbdi_dn4 * ddt_scale));
        let eq19_e1117_d_n5: f64 = (p.p87 * (locals.var_qbdi_dn5 * ddt_scale));
        let eq19_e1117_d_n6: f64 = (p.p87 * (locals.var_qbdi_dn6 * ddt_scale));
        let eq19_e1117_d_n7: f64 = (p.p87 * (locals.var_qbdi_dn7 * ddt_scale));
        let eq19_e1117_d_n8: f64 = (p.p87 * (locals.var_qbdi_dn8 * ddt_scale));
        let eq19_e1117_d_n9: f64 = (p.p87 * (locals.var_qbdi_dn9 * ddt_scale));
        let eq19_e1117_d_n10: f64 = (p.p87 * (locals.var_qbdi_dn10 * ddt_scale));
        let eq19_e1117_d_n13: f64 = (p.p87 * (locals.var_qbdi_dn13 * ddt_scale));
        (eq19_e1117, eq19_e1117_d_n0, eq19_e1117_d_n2, eq19_e1117_d_n4, eq19_e1117_d_n5, eq19_e1117_d_n6, eq19_e1117_d_n7, eq19_e1117_d_n8, eq19_e1117_d_n9, eq19_e1117_d_n10, eq19_e1117_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e1119;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq19_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq19_e1119_d_n0), multiplicity * (eq19_e1119_d_n2), multiplicity * (eq19_e1119_d_n4), multiplicity * (eq19_e1119_d_n5), multiplicity * (eq19_e1119_d_n6), multiplicity * (eq19_e1119_d_n7), multiplicity * (eq19_e1119_d_n8), multiplicity * (eq19_e1119_d_n9), multiplicity * (eq19_e1119_d_n10), multiplicity * (eq19_e1119_d_n13)],
            [],
            [],
            1.0,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq20_e1125, eq20_e1125_d_n0, eq20_e1125_d_n2, eq20_e1125_d_n4, eq20_e1125_d_n5, eq20_e1125_d_n6, eq20_e1125_d_n7, eq20_e1125_d_n8, eq20_e1125_d_n9, eq20_e1125_d_n10, eq20_e1125_d_n13,) = {
    if (locals.var_guard2410 != 0.0) {
        let eq20_e1123: f64 = (p.p87 * locals.var_igs);
        let eq20_e1123_d_n0: f64 = (p.p87 * locals.var_igs_dn0);
        let eq20_e1123_d_n2: f64 = (p.p87 * locals.var_igs_dn2);
        let eq20_e1123_d_n4: f64 = (p.p87 * locals.var_igs_dn4);
        let eq20_e1123_d_n5: f64 = (p.p87 * locals.var_igs_dn5);
        let eq20_e1123_d_n6: f64 = (p.p87 * locals.var_igs_dn6);
        let eq20_e1123_d_n7: f64 = (p.p87 * locals.var_igs_dn7);
        let eq20_e1123_d_n8: f64 = (p.p87 * locals.var_igs_dn8);
        let eq20_e1123_d_n9: f64 = (p.p87 * locals.var_igs_dn9);
        let eq20_e1123_d_n10: f64 = (p.p87 * locals.var_igs_dn10);
        let eq20_e1123_d_n13: f64 = (p.p87 * locals.var_igs_dn13);
        (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n2, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e1125;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq20_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq20_e1125_d_n0), multiplicity * (eq20_e1125_d_n2), multiplicity * (eq20_e1125_d_n4), multiplicity * (eq20_e1125_d_n5), multiplicity * (eq20_e1125_d_n6), multiplicity * (eq20_e1125_d_n7), multiplicity * (eq20_e1125_d_n8), multiplicity * (eq20_e1125_d_n9), multiplicity * (eq20_e1125_d_n10), multiplicity * (eq20_e1125_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq21_e1131, eq21_e1131_d_n0, eq21_e1131_d_n2, eq21_e1131_d_n4, eq21_e1131_d_n5, eq21_e1131_d_n6, eq21_e1131_d_n7, eq21_e1131_d_n8, eq21_e1131_d_n9, eq21_e1131_d_n10, eq21_e1131_d_n13,) = {
    if (locals.var_guard2410 != 0.0) {
        let eq21_e1129: f64 = (p.p87 * locals.var_igd);
        let eq21_e1129_d_n0: f64 = (p.p87 * locals.var_igd_dn0);
        let eq21_e1129_d_n2: f64 = (p.p87 * locals.var_igd_dn2);
        let eq21_e1129_d_n4: f64 = (p.p87 * locals.var_igd_dn4);
        let eq21_e1129_d_n5: f64 = (p.p87 * locals.var_igd_dn5);
        let eq21_e1129_d_n6: f64 = (p.p87 * locals.var_igd_dn6);
        let eq21_e1129_d_n7: f64 = (p.p87 * locals.var_igd_dn7);
        let eq21_e1129_d_n8: f64 = (p.p87 * locals.var_igd_dn8);
        let eq21_e1129_d_n9: f64 = (p.p87 * locals.var_igd_dn9);
        let eq21_e1129_d_n10: f64 = (p.p87 * locals.var_igd_dn10);
        let eq21_e1129_d_n13: f64 = (p.p87 * locals.var_igd_dn13);
        (eq21_e1129, eq21_e1129_d_n0, eq21_e1129_d_n2, eq21_e1129_d_n4, eq21_e1129_d_n5, eq21_e1129_d_n6, eq21_e1129_d_n7, eq21_e1129_d_n8, eq21_e1129_d_n9, eq21_e1129_d_n10, eq21_e1129_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1131;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(5),
            multiplicity * (eq21_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq21_e1131_d_n0), multiplicity * (eq21_e1131_d_n2), multiplicity * (eq21_e1131_d_n4), multiplicity * (eq21_e1131_d_n5), multiplicity * (eq21_e1131_d_n6), multiplicity * (eq21_e1131_d_n7), multiplicity * (eq21_e1131_d_n8), multiplicity * (eq21_e1131_d_n9), multiplicity * (eq21_e1131_d_n10), multiplicity * (eq21_e1131_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq22_e1137, eq22_e1137_d_n0, eq22_e1137_d_n2, eq22_e1137_d_n4, eq22_e1137_d_n5, eq22_e1137_d_n6, eq22_e1137_d_n7, eq22_e1137_d_n8, eq22_e1137_d_n9, eq22_e1137_d_n10, eq22_e1137_d_n13,) = {
    if (locals.var_guard2410 != 0.0) {
        let eq22_e1135: f64 = (p.p87 * locals.var_igb);
        let eq22_e1135_d_n0: f64 = (p.p87 * locals.var_igb_dn0);
        let eq22_e1135_d_n2: f64 = (p.p87 * locals.var_igb_dn2);
        let eq22_e1135_d_n4: f64 = (p.p87 * locals.var_igb_dn4);
        let eq22_e1135_d_n5: f64 = (p.p87 * locals.var_igb_dn5);
        let eq22_e1135_d_n6: f64 = (p.p87 * locals.var_igb_dn6);
        let eq22_e1135_d_n7: f64 = (p.p87 * locals.var_igb_dn7);
        let eq22_e1135_d_n8: f64 = (p.p87 * locals.var_igb_dn8);
        let eq22_e1135_d_n9: f64 = (p.p87 * locals.var_igb_dn9);
        let eq22_e1135_d_n10: f64 = (p.p87 * locals.var_igb_dn10);
        let eq22_e1135_d_n13: f64 = (p.p87 * locals.var_igb_dn13);
        (eq22_e1135, eq22_e1135_d_n0, eq22_e1135_d_n2, eq22_e1135_d_n4, eq22_e1135_d_n5, eq22_e1135_d_n6, eq22_e1135_d_n7, eq22_e1135_d_n8, eq22_e1135_d_n9, eq22_e1135_d_n10, eq22_e1135_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e1137;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq22_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq22_e1137_d_n0), multiplicity * (eq22_e1137_d_n2), multiplicity * (eq22_e1137_d_n4), multiplicity * (eq22_e1137_d_n5), multiplicity * (eq22_e1137_d_n6), multiplicity * (eq22_e1137_d_n7), multiplicity * (eq22_e1137_d_n8), multiplicity * (eq22_e1137_d_n9), multiplicity * (eq22_e1137_d_n10), multiplicity * (eq22_e1137_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq23_e1143, eq23_e1143_d_n0, eq23_e1143_d_n2, eq23_e1143_d_n4, eq23_e1143_d_n5, eq23_e1143_d_n6, eq23_e1143_d_n7, eq23_e1143_d_n8, eq23_e1143_d_n9, eq23_e1143_d_n10, eq23_e1143_d_n13,) = {
    if (locals.var_flg_rd != 0.0) {
        let eq23_e1141: f64 = ((nv0 - nv5) / locals.var_rdd);
        let eq23_e1141_d_n0: f64 = ((locals.var_rdd - ((nv0 - nv5) * locals.var_rdd_dn0)) / (locals.var_rdd * locals.var_rdd));
        let eq23_e1141_d_n2: f64 = (-(((nv0 - nv5) * locals.var_rdd_dn2) / (locals.var_rdd * locals.var_rdd)));
        let eq23_e1141_d_n4: f64 = (-(((nv0 - nv5) * locals.var_rdd_dn4) / (locals.var_rdd * locals.var_rdd)));
        let eq23_e1141_d_n5: f64 = (((-locals.var_rdd) - ((nv0 - nv5) * locals.var_rdd_dn5)) / (locals.var_rdd * locals.var_rdd));
        let eq23_e1141_d_n6: f64 = (-(((nv0 - nv5) * locals.var_rdd_dn6) / (locals.var_rdd * locals.var_rdd)));
        let eq23_e1141_d_n7: f64 = (-(((nv0 - nv5) * locals.var_rdd_dn7) / (locals.var_rdd * locals.var_rdd)));
        let eq23_e1141_d_n8: f64 = (-(((nv0 - nv5) * locals.var_rdd_dn8) / (locals.var_rdd * locals.var_rdd)));
        let eq23_e1141_d_n9: f64 = (-(((nv0 - nv5) * locals.var_rdd_dn9) / (locals.var_rdd * locals.var_rdd)));
        let eq23_e1141_d_n10: f64 = (-(((nv0 - nv5) * locals.var_rdd_dn10) / (locals.var_rdd * locals.var_rdd)));
        let eq23_e1141_d_n13: f64 = (-(((nv0 - nv5) * locals.var_rdd_dn13) / (locals.var_rdd * locals.var_rdd)));
        (eq23_e1141, eq23_e1141_d_n0, eq23_e1141_d_n2, eq23_e1141_d_n4, eq23_e1141_d_n5, eq23_e1141_d_n6, eq23_e1141_d_n7, eq23_e1141_d_n8, eq23_e1141_d_n9, eq23_e1141_d_n10, eq23_e1141_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1143;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(0),
            Some(5),
            multiplicity * (eq23_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq23_e1143_d_n0), multiplicity * (eq23_e1143_d_n2), multiplicity * (eq23_e1143_d_n4), multiplicity * (eq23_e1143_d_n5), multiplicity * (eq23_e1143_d_n6), multiplicity * (eq23_e1143_d_n7), multiplicity * (eq23_e1143_d_n8), multiplicity * (eq23_e1143_d_n9), multiplicity * (eq23_e1143_d_n10), multiplicity * (eq23_e1143_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq25_e1154, eq25_e1154_d_n0, eq25_e1154_d_n2, eq25_e1154_d_n4, eq25_e1154_d_n5, eq25_e1154_d_n6, eq25_e1154_d_n7, eq25_e1154_d_n8, eq25_e1154_d_n9, eq25_e1154_d_n10, eq25_e1154_d_n13,) = {
    if (locals.var_flg_rs != 0.0) {
        let eq25_e1152: f64 = ((nv7 - nv2) / locals.var_rsd);
        let eq25_e1152_d_n0: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn0) / (locals.var_rsd * locals.var_rsd)));
        let eq25_e1152_d_n2: f64 = (((-locals.var_rsd) - ((nv7 - nv2) * locals.var_rsd_dn2)) / (locals.var_rsd * locals.var_rsd));
        let eq25_e1152_d_n4: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn4) / (locals.var_rsd * locals.var_rsd)));
        let eq25_e1152_d_n5: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn5) / (locals.var_rsd * locals.var_rsd)));
        let eq25_e1152_d_n6: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn6) / (locals.var_rsd * locals.var_rsd)));
        let eq25_e1152_d_n7: f64 = ((locals.var_rsd - ((nv7 - nv2) * locals.var_rsd_dn7)) / (locals.var_rsd * locals.var_rsd));
        let eq25_e1152_d_n8: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn8) / (locals.var_rsd * locals.var_rsd)));
        let eq25_e1152_d_n9: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn9) / (locals.var_rsd * locals.var_rsd)));
        let eq25_e1152_d_n10: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn10) / (locals.var_rsd * locals.var_rsd)));
        let eq25_e1152_d_n13: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn13) / (locals.var_rsd * locals.var_rsd)));
        (eq25_e1152, eq25_e1152_d_n0, eq25_e1152_d_n2, eq25_e1152_d_n4, eq25_e1152_d_n5, eq25_e1152_d_n6, eq25_e1152_d_n7, eq25_e1152_d_n8, eq25_e1152_d_n9, eq25_e1152_d_n10, eq25_e1152_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e1154;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(2),
            multiplicity * (eq25_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq25_e1154_d_n0), multiplicity * (eq25_e1154_d_n2), multiplicity * (eq25_e1154_d_n4), multiplicity * (eq25_e1154_d_n5), multiplicity * (eq25_e1154_d_n6), multiplicity * (eq25_e1154_d_n7), multiplicity * (eq25_e1154_d_n8), multiplicity * (eq25_e1154_d_n9), multiplicity * (eq25_e1154_d_n10), multiplicity * (eq25_e1154_d_n13)],
            [],
            [],
            1.0,
        );
        let eq27_e1163: f64 = (locals.var_qg + locals.var_qg_nqs);
        let eq27_e1164: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq27_e1163);
        let eq27_e1165: f64 = (p.p87 * eq27_e1164);
        let eq27_e1165_d_n0: f64 = (p.p87 * (locals.var_qg_dn0 * ddt_scale));
        let eq27_e1165_d_n2: f64 = (p.p87 * (locals.var_qg_dn2 * ddt_scale));
        let eq27_e1165_d_n4: f64 = (p.p87 * (locals.var_qg_dn4 * ddt_scale));
        let eq27_e1165_d_n5: f64 = (p.p87 * (locals.var_qg_dn5 * ddt_scale));
        let eq27_e1165_d_n6: f64 = (p.p87 * (locals.var_qg_dn6 * ddt_scale));
        let eq27_e1165_d_n7: f64 = (p.p87 * (locals.var_qg_dn7 * ddt_scale));
        let eq27_e1165_d_n8: f64 = (p.p87 * (locals.var_qg_dn8 * ddt_scale));
        let eq27_e1165_d_n9: f64 = (p.p87 * (locals.var_qg_dn9 * ddt_scale));
        let eq27_e1165_d_n10: f64 = (p.p87 * (locals.var_qg_dn10 * ddt_scale));
        let eq27_e1165_d_n11: f64 = (p.p87 * (locals.var_qg_nqs_dn11 * ddt_scale));
        let eq27_e1165_d_n12: f64 = (p.p87 * (locals.var_qg_nqs_dn12 * ddt_scale));
        let eq27_e1165_d_n13: f64 = (p.p87 * (locals.var_qg_dn13 * ddt_scale));
        let eq27_value: f64 = eq27_e1165;
        let eq27_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let eq27_node_derivatives: [f64; 12] = [eq27_e1165_d_n0, eq27_e1165_d_n2, eq27_e1165_d_n4, eq27_e1165_d_n5, eq27_e1165_d_n6, eq27_e1165_d_n7, eq27_e1165_d_n8, eq27_e1165_d_n9, eq27_e1165_d_n10, eq27_e1165_d_n11, eq27_e1165_d_n12, eq27_e1165_d_n13];
        let eq27_branch_derivative_indices: [usize; 0] = [];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq27_value),
            &eq27_node_derivative_indices,
            &eq27_node_derivatives,
            &eq27_branch_derivative_indices,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let eq28_e1169: f64 = (locals.var_qd + locals.var_qd_nqs);
        let eq28_e1169_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);
        let eq28_e1169_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);
        let eq28_e1169_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);
        let eq28_e1169_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);
        let eq28_e1169_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);
        let eq28_e1169_d_n7: f64 = (locals.var_qd_dn7 + locals.var_qd_nqs_dn7);
        let eq28_e1169_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);
        let eq28_e1169_d_n9: f64 = (locals.var_qd_dn9 + locals.var_qd_nqs_dn9);
        let eq28_e1169_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);
        let eq28_e1169_d_n13: f64 = (locals.var_qd_dn13 + locals.var_qd_nqs_dn13);
        let eq28_e1170: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq28_e1169);
        let eq28_e1171: f64 = (p.p87 * eq28_e1170);
        let eq28_e1171_d_n0: f64 = (p.p87 * (eq28_e1169_d_n0 * ddt_scale));
        let eq28_e1171_d_n2: f64 = (p.p87 * (eq28_e1169_d_n2 * ddt_scale));
        let eq28_e1171_d_n4: f64 = (p.p87 * (eq28_e1169_d_n4 * ddt_scale));
        let eq28_e1171_d_n5: f64 = (p.p87 * (eq28_e1169_d_n5 * ddt_scale));
        let eq28_e1171_d_n6: f64 = (p.p87 * (eq28_e1169_d_n6 * ddt_scale));
        let eq28_e1171_d_n7: f64 = (p.p87 * (eq28_e1169_d_n7 * ddt_scale));
        let eq28_e1171_d_n8: f64 = (p.p87 * (eq28_e1169_d_n8 * ddt_scale));
        let eq28_e1171_d_n9: f64 = (p.p87 * (eq28_e1169_d_n9 * ddt_scale));
        let eq28_e1171_d_n10: f64 = (p.p87 * (eq28_e1169_d_n10 * ddt_scale));
        let eq28_e1171_d_n11: f64 = (p.p87 * (locals.var_qd_nqs_dn11 * ddt_scale));
        let eq28_e1171_d_n13: f64 = (p.p87 * (eq28_e1169_d_n13 * ddt_scale));
        let eq28_value: f64 = eq28_e1171;
        let eq28_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 13];
        let eq28_node_derivatives: [f64; 11] = [eq28_e1171_d_n0, eq28_e1171_d_n2, eq28_e1171_d_n4, eq28_e1171_d_n5, eq28_e1171_d_n6, eq28_e1171_d_n7, eq28_e1171_d_n8, eq28_e1171_d_n9, eq28_e1171_d_n10, eq28_e1171_d_n11, eq28_e1171_d_n13];
        let eq28_branch_derivative_indices: [usize; 0] = [];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq28_value),
            &eq28_node_derivative_indices,
            &eq28_node_derivatives,
            &eq28_branch_derivative_indices,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let eq29_e1176: f64 = (locals.var_qg_nqs + locals.var_qd_nqs);
        let eq29_e1176_d_n11: f64 = (locals.var_qg_nqs_dn11 + locals.var_qd_nqs_dn11);
        let eq29_e1178: f64 = (eq29_e1176 + locals.var_qs_nqs);
        let eq29_e1178_d_n0: f64 = (locals.var_qd_nqs_dn0 + locals.var_qs_nqs_dn0);
        let eq29_e1178_d_n2: f64 = (locals.var_qd_nqs_dn2 + locals.var_qs_nqs_dn2);
        let eq29_e1178_d_n4: f64 = (locals.var_qd_nqs_dn4 + locals.var_qs_nqs_dn4);
        let eq29_e1178_d_n5: f64 = (locals.var_qd_nqs_dn5 + locals.var_qs_nqs_dn5);
        let eq29_e1178_d_n6: f64 = (locals.var_qd_nqs_dn6 + locals.var_qs_nqs_dn6);
        let eq29_e1178_d_n7: f64 = (locals.var_qd_nqs_dn7 + locals.var_qs_nqs_dn7);
        let eq29_e1178_d_n8: f64 = (locals.var_qd_nqs_dn8 + locals.var_qs_nqs_dn8);
        let eq29_e1178_d_n9: f64 = (locals.var_qd_nqs_dn9 + locals.var_qs_nqs_dn9);
        let eq29_e1178_d_n10: f64 = (locals.var_qd_nqs_dn10 + locals.var_qs_nqs_dn10);
        let eq29_e1178_d_n11: f64 = (eq29_e1176_d_n11 + locals.var_qs_nqs_dn11);
        let eq29_e1178_d_n13: f64 = (locals.var_qd_nqs_dn13 + locals.var_qs_nqs_dn13);
        let eq29_e1179: f64 = (locals.var_qb - eq29_e1178);
        let eq29_e1179_d_n0: f64 = (locals.var_qb_dn0 - eq29_e1178_d_n0);
        let eq29_e1179_d_n2: f64 = (locals.var_qb_dn2 - eq29_e1178_d_n2);
        let eq29_e1179_d_n4: f64 = (locals.var_qb_dn4 - eq29_e1178_d_n4);
        let eq29_e1179_d_n5: f64 = (locals.var_qb_dn5 - eq29_e1178_d_n5);
        let eq29_e1179_d_n6: f64 = (locals.var_qb_dn6 - eq29_e1178_d_n6);
        let eq29_e1179_d_n7: f64 = (locals.var_qb_dn7 - eq29_e1178_d_n7);
        let eq29_e1179_d_n8: f64 = (locals.var_qb_dn8 - eq29_e1178_d_n8);
        let eq29_e1179_d_n9: f64 = (locals.var_qb_dn9 - eq29_e1178_d_n9);
        let eq29_e1179_d_n10: f64 = (locals.var_qb_dn10 - eq29_e1178_d_n10);
        let eq29_e1179_d_n13: f64 = (locals.var_qb_dn13 - eq29_e1178_d_n13);
        let eq29_e1180: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq29_e1179);
        let eq29_e1181: f64 = (p.p87 * eq29_e1180);
        let eq29_e1181_d_n0: f64 = (p.p87 * (eq29_e1179_d_n0 * ddt_scale));
        let eq29_e1181_d_n2: f64 = (p.p87 * (eq29_e1179_d_n2 * ddt_scale));
        let eq29_e1181_d_n4: f64 = (p.p87 * (eq29_e1179_d_n4 * ddt_scale));
        let eq29_e1181_d_n5: f64 = (p.p87 * (eq29_e1179_d_n5 * ddt_scale));
        let eq29_e1181_d_n6: f64 = (p.p87 * (eq29_e1179_d_n6 * ddt_scale));
        let eq29_e1181_d_n7: f64 = (p.p87 * (eq29_e1179_d_n7 * ddt_scale));
        let eq29_e1181_d_n8: f64 = (p.p87 * (eq29_e1179_d_n8 * ddt_scale));
        let eq29_e1181_d_n9: f64 = (p.p87 * (eq29_e1179_d_n9 * ddt_scale));
        let eq29_e1181_d_n10: f64 = (p.p87 * (eq29_e1179_d_n10 * ddt_scale));
        let eq29_e1181_d_n11: f64 = (p.p87 * ((-eq29_e1178_d_n11) * ddt_scale));
        let eq29_e1181_d_n12: f64 = (p.p87 * ((-locals.var_qg_nqs_dn12) * ddt_scale));
        let eq29_e1181_d_n13: f64 = (p.p87 * (eq29_e1179_d_n13 * ddt_scale));
        let eq29_value: f64 = eq29_e1181;
        let eq29_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let eq29_node_derivatives: [f64; 12] = [eq29_e1181_d_n0, eq29_e1181_d_n2, eq29_e1181_d_n4, eq29_e1181_d_n5, eq29_e1181_d_n6, eq29_e1181_d_n7, eq29_e1181_d_n8, eq29_e1181_d_n9, eq29_e1181_d_n10, eq29_e1181_d_n11, eq29_e1181_d_n12, eq29_e1181_d_n13];
        let eq29_branch_derivative_indices: [usize; 0] = [];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq29_value),
            &eq29_node_derivative_indices,
            &eq29_node_derivatives,
            &eq29_branch_derivative_indices,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let eq30_e1184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, locals.var_qgext);
        let eq30_e1185: f64 = (p.p87 * eq30_e1184);
        let eq30_e1185_d_n0: f64 = (p.p87 * (locals.var_qgext_dn0 * ddt_scale));
        let eq30_e1185_d_n2: f64 = (p.p87 * (locals.var_qgext_dn2 * ddt_scale));
        let eq30_e1185_d_n4: f64 = (p.p87 * (locals.var_qgext_dn4 * ddt_scale));
        let eq30_e1185_d_n5: f64 = (p.p87 * (locals.var_qgext_dn5 * ddt_scale));
        let eq30_e1185_d_n6: f64 = (p.p87 * (locals.var_qgext_dn6 * ddt_scale));
        let eq30_e1185_d_n7: f64 = (p.p87 * (locals.var_qgext_dn7 * ddt_scale));
        let eq30_e1185_d_n8: f64 = (p.p87 * (locals.var_qgext_dn8 * ddt_scale));
        let eq30_e1185_d_n9: f64 = (p.p87 * (locals.var_qgext_dn9 * ddt_scale));
        let eq30_e1185_d_n10: f64 = (p.p87 * (locals.var_qgext_dn10 * ddt_scale));
        let eq30_e1185_d_n13: f64 = (p.p87 * (locals.var_qgext_dn13 * ddt_scale));
        let eq30_value: f64 = eq30_e1185;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(2),
            multiplicity * (eq30_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq30_e1185_d_n0), multiplicity * (eq30_e1185_d_n2), multiplicity * (eq30_e1185_d_n4), multiplicity * (eq30_e1185_d_n5), multiplicity * (eq30_e1185_d_n6), multiplicity * (eq30_e1185_d_n7), multiplicity * (eq30_e1185_d_n8), multiplicity * (eq30_e1185_d_n9), multiplicity * (eq30_e1185_d_n10), multiplicity * (eq30_e1185_d_n13)],
            [],
            [],
            1.0,
        );
        let eq31_e1188: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, locals.var_qdext);
        let eq31_e1189: f64 = (p.p87 * eq31_e1188);
        let eq31_e1189_d_n0: f64 = (p.p87 * (locals.var_qdext_dn0 * ddt_scale));
        let eq31_e1189_d_n2: f64 = (p.p87 * (locals.var_qdext_dn2 * ddt_scale));
        let eq31_e1189_d_n4: f64 = (p.p87 * (locals.var_qdext_dn4 * ddt_scale));
        let eq31_e1189_d_n5: f64 = (p.p87 * (locals.var_qdext_dn5 * ddt_scale));
        let eq31_e1189_d_n6: f64 = (p.p87 * (locals.var_qdext_dn6 * ddt_scale));
        let eq31_e1189_d_n7: f64 = (p.p87 * (locals.var_qdext_dn7 * ddt_scale));
        let eq31_e1189_d_n8: f64 = (p.p87 * (locals.var_qdext_dn8 * ddt_scale));
        let eq31_e1189_d_n9: f64 = (p.p87 * (locals.var_qdext_dn9 * ddt_scale));
        let eq31_e1189_d_n10: f64 = (p.p87 * (locals.var_qdext_dn10 * ddt_scale));
        let eq31_e1189_d_n13: f64 = (p.p87 * (locals.var_qdext_dn13 * ddt_scale));
        let eq31_value: f64 = eq31_e1189;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(0),
            Some(2),
            multiplicity * (eq31_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq31_e1189_d_n0), multiplicity * (eq31_e1189_d_n2), multiplicity * (eq31_e1189_d_n4), multiplicity * (eq31_e1189_d_n5), multiplicity * (eq31_e1189_d_n6), multiplicity * (eq31_e1189_d_n7), multiplicity * (eq31_e1189_d_n8), multiplicity * (eq31_e1189_d_n9), multiplicity * (eq31_e1189_d_n10), multiplicity * (eq31_e1189_d_n13)],
            [],
            [],
            1.0,
        );
        let eq32_e1192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, locals.var_qbext);
        let eq32_e1193: f64 = (p.p87 * eq32_e1192);
        let eq32_e1193_d_n0: f64 = (p.p87 * (locals.var_qbext_dn0 * ddt_scale));
        let eq32_e1193_d_n2: f64 = (p.p87 * (locals.var_qbext_dn2 * ddt_scale));
        let eq32_e1193_d_n4: f64 = (p.p87 * (locals.var_qbext_dn4 * ddt_scale));
        let eq32_e1193_d_n5: f64 = (p.p87 * (locals.var_qbext_dn5 * ddt_scale));
        let eq32_e1193_d_n6: f64 = (p.p87 * (locals.var_qbext_dn6 * ddt_scale));
        let eq32_e1193_d_n7: f64 = (p.p87 * (locals.var_qbext_dn7 * ddt_scale));
        let eq32_e1193_d_n8: f64 = (p.p87 * (locals.var_qbext_dn8 * ddt_scale));
        let eq32_e1193_d_n9: f64 = (p.p87 * (locals.var_qbext_dn9 * ddt_scale));
        let eq32_e1193_d_n10: f64 = (p.p87 * (locals.var_qbext_dn10 * ddt_scale));
        let eq32_e1193_d_n13: f64 = (p.p87 * (locals.var_qbext_dn13 * ddt_scale));
        let eq32_value: f64 = eq32_e1193;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(2),
            multiplicity * (eq32_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq32_e1193_d_n0), multiplicity * (eq32_e1193_d_n2), multiplicity * (eq32_e1193_d_n4), multiplicity * (eq32_e1193_d_n5), multiplicity * (eq32_e1193_d_n6), multiplicity * (eq32_e1193_d_n7), multiplicity * (eq32_e1193_d_n8), multiplicity * (eq32_e1193_d_n9), multiplicity * (eq32_e1193_d_n10), multiplicity * (eq32_e1193_d_n13)],
            [],
            [],
            1.0,
        );
        let eq33_e1195: f64 = (-p.p87);
        let eq33_e1197: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, locals.var_qfd);
        let eq33_e1198: f64 = (eq33_e1195 * eq33_e1197);
        let eq33_e1198_d_n0: f64 = (eq33_e1195 * (locals.var_qfd_dn0 * ddt_scale));
        let eq33_e1198_d_n2: f64 = (eq33_e1195 * (locals.var_qfd_dn2 * ddt_scale));
        let eq33_e1198_d_n6: f64 = (eq33_e1195 * (locals.var_qfd_dn6 * ddt_scale));
        let eq33_value: f64 = eq33_e1198;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(0),
            multiplicity * (eq33_value),
            0,
            multiplicity * (eq33_e1198_d_n0),
            2,
            multiplicity * (eq33_e1198_d_n2),
            6,
            multiplicity * (eq33_e1198_d_n6),
        );
        let eq34_e1200: f64 = (-p.p87);
        let eq34_e1202: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, locals.var_qfs);
        let eq34_e1203: f64 = (eq34_e1200 * eq34_e1202);
        let eq34_e1203_d_n2: f64 = (eq34_e1200 * (locals.var_qfs_dn2 * ddt_scale));
        let eq34_e1203_d_n6: f64 = (eq34_e1200 * (locals.var_qfs_dn6 * ddt_scale));
        let eq34_value: f64 = eq34_e1203;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(2),
            multiplicity * (eq34_value),
            2,
            multiplicity * (eq34_e1203_d_n2),
            6,
            multiplicity * (eq34_e1203_d_n6),
        );
        let eq39_e1229: f64 = (locals.var_ci * (nv14 - 0.0));
        let eq39_e1229_d_n0: f64 = (locals.var_ci_dn0 * (nv14 - 0.0));
        let eq39_e1229_d_n2: f64 = (locals.var_ci_dn2 * (nv14 - 0.0));
        let eq39_e1229_d_n4: f64 = (locals.var_ci_dn4 * (nv14 - 0.0));
        let eq39_e1229_d_n5: f64 = (locals.var_ci_dn5 * (nv14 - 0.0));
        let eq39_e1229_d_n6: f64 = (locals.var_ci_dn6 * (nv14 - 0.0));
        let eq39_e1229_d_n7: f64 = (locals.var_ci_dn7 * (nv14 - 0.0));
        let eq39_e1229_d_n8: f64 = (locals.var_ci_dn8 * (nv14 - 0.0));
        let eq39_e1229_d_n9: f64 = (locals.var_ci_dn9 * (nv14 - 0.0));
        let eq39_e1229_d_n10: f64 = (locals.var_ci_dn10 * (nv14 - 0.0));
        let eq39_e1229_d_n13: f64 = (locals.var_ci_dn13 * (nv14 - 0.0));
        let eq39_value: f64 = eq39_e1229;
        let eq39_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 13, 14];
        let eq39_node_derivatives: [f64; 11] = [eq39_e1229_d_n0, eq39_e1229_d_n2, eq39_e1229_d_n4, eq39_e1229_d_n5, eq39_e1229_d_n6, eq39_e1229_d_n7, eq39_e1229_d_n8, eq39_e1229_d_n9, eq39_e1229_d_n10, eq39_e1229_d_n13, locals.var_ci];
        let eq39_branch_derivative_indices: [usize; 0] = [];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq39_value),
            &eq39_node_derivative_indices,
            &eq39_node_derivatives,
            &eq39_branch_derivative_indices,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let eq40_e1232: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq40_e1232_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq40_e1232_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq40_e1232_d_n4: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn4);
        let eq40_e1232_d_n5: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn5);
        let eq40_e1232_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq40_e1232_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq40_e1232_d_n8: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn8);
        let eq40_e1232_d_n9: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn9);
        let eq40_e1232_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq40_e1232_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq40_e1233: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq40_e1232);
        let eq40_value: f64 = eq40_e1233;
        let eq40_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 13, 14];
        let eq40_node_derivatives: [f64; 11] = [(eq40_e1232_d_n0 * ddt_scale), (eq40_e1232_d_n2 * ddt_scale), (eq40_e1232_d_n4 * ddt_scale), (eq40_e1232_d_n5 * ddt_scale), (eq40_e1232_d_n6 * ddt_scale), (eq40_e1232_d_n7 * ddt_scale), (eq40_e1232_d_n8 * ddt_scale), (eq40_e1232_d_n9 * ddt_scale), (eq40_e1232_d_n10 * ddt_scale), (eq40_e1232_d_n13 * ddt_scale), (locals.var_sigrat_s * ddt_scale)];
        let eq40_branch_derivative_indices: [usize; 0] = [];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq40_value),
            &eq40_node_derivative_indices,
            &eq40_node_derivatives,
            &eq40_branch_derivative_indices,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e1236: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq41_e1236_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq41_e1236_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq41_e1236_d_n4: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn4);
        let eq41_e1236_d_n5: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn5);
        let eq41_e1236_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq41_e1236_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq41_e1236_d_n8: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn8);
        let eq41_e1236_d_n9: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn9);
        let eq41_e1236_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq41_e1236_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq41_e1237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq41_e1236);
        let eq41_value: f64 = eq41_e1237;
        let eq41_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 13, 14];
        let eq41_node_derivatives: [f64; 11] = [(eq41_e1236_d_n0 * ddt_scale), (eq41_e1236_d_n2 * ddt_scale), (eq41_e1236_d_n4 * ddt_scale), (eq41_e1236_d_n5 * ddt_scale), (eq41_e1236_d_n6 * ddt_scale), (eq41_e1236_d_n7 * ddt_scale), (eq41_e1236_d_n8 * ddt_scale), (eq41_e1236_d_n9 * ddt_scale), (eq41_e1236_d_n10 * ddt_scale), (eq41_e1236_d_n13 * ddt_scale), (locals.var_sigrat_d * ddt_scale)];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq41_value),
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let (eq56_e1332, eq56_e1332_d_n0, eq56_e1332_d_n2, eq56_e1332_d_n4, eq56_e1332_d_n5, eq56_e1332_d_n6, eq56_e1332_d_n7, eq56_e1332_d_n8, eq56_e1332_d_n9, eq56_e1332_d_n10, eq56_e1332_d_n13,) = {
    if (locals.var_guard2413 != 0.0) {
        let eq56_e1330: f64 = (-locals.var_itemp);
        (eq56_e1330, (-locals.var_itemp_dn0), (-locals.var_itemp_dn2), (-locals.var_itemp_dn4), (-locals.var_itemp_dn5), (-locals.var_itemp_dn6), (-locals.var_itemp_dn7), (-locals.var_itemp_dn8), (-locals.var_itemp_dn9), (-locals.var_itemp_dn10), (-locals.var_itemp_dn13),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e1332;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(4),
            None,
            multiplicity * (eq56_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq56_e1332_d_n0), multiplicity * (eq56_e1332_d_n2), multiplicity * (eq56_e1332_d_n4), multiplicity * (eq56_e1332_d_n5), multiplicity * (eq56_e1332_d_n6), multiplicity * (eq56_e1332_d_n7), multiplicity * (eq56_e1332_d_n8), multiplicity * (eq56_e1332_d_n9), multiplicity * (eq56_e1332_d_n10), multiplicity * (eq56_e1332_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq59_e1347, eq59_e1347_d_n0, eq59_e1347_d_n2, eq59_e1347_d_n4, eq59_e1347_d_n5, eq59_e1347_d_n6, eq59_e1347_d_n7, eq59_e1347_d_n8, eq59_e1347_d_n9, eq59_e1347_d_n10, eq59_e1347_d_n11, eq59_e1347_d_n13,) = {
    if (p.p28 != 0.0) {
        (locals.var_iqi_nqs, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn4, locals.var_iqi_nqs_dn5, locals.var_iqi_nqs_dn6, locals.var_iqi_nqs_dn7, locals.var_iqi_nqs_dn8, locals.var_iqi_nqs_dn9, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1347;
        let eq59_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 13];
        let eq59_node_derivatives: [f64; 11] = [eq59_e1347_d_n0, eq59_e1347_d_n2, eq59_e1347_d_n4, eq59_e1347_d_n5, eq59_e1347_d_n6, eq59_e1347_d_n7, eq59_e1347_d_n8, eq59_e1347_d_n9, eq59_e1347_d_n10, eq59_e1347_d_n11, eq59_e1347_d_n13];
        let eq59_branch_derivative_indices: [usize; 0] = [];
        let eq59_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            None,
            multiplicity * (eq59_value),
            &eq59_node_derivative_indices,
            &eq59_node_derivatives,
            &eq59_branch_derivative_indices,
            &eq59_branch_derivatives,
            multiplicity,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq60_e1351, eq60_e1351_d_n0, eq60_e1351_d_n2, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n12, eq60_e1351_d_n13,) = {
    if (p.p28 != 0.0) {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn4, locals.var_iqb_nqs_dn5, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn8, locals.var_iqb_nqs_dn9, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn12, locals.var_iqb_nqs_dn13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1351;
        let eq60_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 12, 13];
        let eq60_node_derivatives: [f64; 11] = [eq60_e1351_d_n0, eq60_e1351_d_n2, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n12, eq60_e1351_d_n13];
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
        let (eq61_e1358, eq61_e1358_d_n11,) = {
    if (p.p28 != 0.0) {
        let eq61_e1355: f64 = (locals.var_cqi * (nv11 - 0.0));
        let eq61_e1356: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq61_e1355);
        (eq61_e1356, (locals.var_cqi * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1358;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (eq61_value),
            11,
            multiplicity * (eq61_e1358_d_n11),
        );
        let (eq62_e1365, eq62_e1365_d_n12,) = {
    if (p.p28 != 0.0) {
        let eq62_e1362: f64 = (locals.var_cqb * (nv12 - 0.0));
        let eq62_e1363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq62_e1362);
        (eq62_e1363, (locals.var_cqb * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1365;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (eq62_value),
            12,
            multiplicity * (eq62_e1365_d_n12),
        );
        let (eq65_e1379, eq65_e1379_d_n0, eq65_e1379_d_n2, eq65_e1379_d_n4, eq65_e1379_d_n5, eq65_e1379_d_n6, eq65_e1379_d_n7, eq65_e1379_d_n8, eq65_e1379_d_n9, eq65_e1379_d_n10, eq65_e1379_d_n13,) = {
    if (p.p29 != 0.0) {
        (locals.var_ibd_nqs, locals.var_ibd_nqs_dn0, locals.var_ibd_nqs_dn2, locals.var_ibd_nqs_dn4, locals.var_ibd_nqs_dn5, locals.var_ibd_nqs_dn6, locals.var_ibd_nqs_dn7, locals.var_ibd_nqs_dn8, locals.var_ibd_nqs_dn9, locals.var_ibd_nqs_dn10, locals.var_ibd_nqs_dn13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e1379;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(13),
            None,
            multiplicity * (eq65_value),
            [0, 2, 4, 5, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq65_e1379_d_n0), multiplicity * (eq65_e1379_d_n2), multiplicity * (eq65_e1379_d_n4), multiplicity * (eq65_e1379_d_n5), multiplicity * (eq65_e1379_d_n6), multiplicity * (eq65_e1379_d_n7), multiplicity * (eq65_e1379_d_n8), multiplicity * (eq65_e1379_d_n9), multiplicity * (eq65_e1379_d_n10), multiplicity * (eq65_e1379_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq66_e1384, eq66_e1384_d_n13,) = {
    if (p.p29 != 0.0) {
        let eq66_e1382: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, (nv13 - 0.0));
        (eq66_e1382, ddt_scale,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1384;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq66_value),
            13,
            multiplicity * (eq66_e1384_d_n13),
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq0_e1018, eq0_e1018_d_n0, eq0_e1018_d_n2, eq0_e1018_d_n4, eq0_e1018_d_n5, eq0_e1018_d_n6, eq0_e1018_d_n7, eq0_e1018_d_n8, eq0_e1018_d_n9, eq0_e1018_d_n10, eq0_e1018_d_n13, eq0_e1018_d_n15, eq0_e1018_q, eq0_e1018_q_d_n15,) = {
    if (locals.var_guard2309 != 0.0) {
        let eq0_e1015_q: f64 = locals.var_q_nqs_a;
        let eq0_e1016: f64 = (locals.var_inqs0_a + locals.var_q_nqs_a);
        let eq0_e1016_d_n15: f64 = (locals.var_inqs0_a_dn15 + locals.var_q_nqs_a_dn15);
        let eq0_e1016_q: f64 = eq0_e1015_q;
        (eq0_e1016, locals.var_inqs0_a_dn0, locals.var_inqs0_a_dn2, locals.var_inqs0_a_dn4, locals.var_inqs0_a_dn5, locals.var_inqs0_a_dn6, locals.var_inqs0_a_dn7, locals.var_inqs0_a_dn8, locals.var_inqs0_a_dn9, locals.var_inqs0_a_dn10, locals.var_inqs0_a_dn13, eq0_e1016_d_n15, eq0_e1016_q, locals.var_q_nqs_a_dn15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq0_e1018_q_d_n15),
        );
        let (eq1_e1025, eq1_e1025_d_n0, eq1_e1025_d_n2, eq1_e1025_d_n4, eq1_e1025_d_n5, eq1_e1025_d_n6, eq1_e1025_d_n7, eq1_e1025_d_n8, eq1_e1025_d_n9, eq1_e1025_d_n10, eq1_e1025_d_n13, eq1_e1025_d_n16, eq1_e1025_q, eq1_e1025_q_d_n16,) = {
    if (locals.var_guard2309 != 0.0) {
        let eq1_e1022_q: f64 = locals.var_q_nqs_k;
        let eq1_e1023: f64 = (locals.var_inqs0_k + locals.var_q_nqs_k);
        let eq1_e1023_d_n16: f64 = (locals.var_inqs0_k_dn16 + locals.var_q_nqs_k_dn16);
        let eq1_e1023_q: f64 = eq1_e1022_q;
        (eq1_e1023, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn13, eq1_e1023_d_n16, eq1_e1023_q, locals.var_q_nqs_k_dn16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq1_e1025_q_d_n16),
        );
        let (eq4_e1042, eq4_e1042_d_n0, eq4_e1042_d_n2, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n13, eq4_e1042_d_n17, eq4_e1042_q, eq4_e1042_q_d_n17,) = {
    if (locals.var_guard2310 != 0.0) {
        let eq4_e1039_q: f64 = locals.var_w_nqs_a;
        let eq4_e1040: f64 = (locals.var_iwnqs0_a + locals.var_w_nqs_a);
        let eq4_e1040_d_n17: f64 = (locals.var_iwnqs0_a_dn17 + locals.var_w_nqs_a_dn17);
        let eq4_e1040_q: f64 = eq4_e1039_q;
        (eq4_e1040, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn13, eq4_e1040_d_n17, eq4_e1040_q, locals.var_w_nqs_a_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq4_e1042_q_d_n17),
        );
        let eq14_e1088_q: f64 = locals.var_qbs;
        let eq14_e1089: f64 = (p.p87 * locals.var_qbs);
        let eq14_e1089_d_n0: f64 = (p.p87 * locals.var_qbs_dn0);
        let eq14_e1089_d_n2: f64 = (p.p87 * locals.var_qbs_dn2);
        let eq14_e1089_d_n4: f64 = (p.p87 * locals.var_qbs_dn4);
        let eq14_e1089_d_n5: f64 = (p.p87 * locals.var_qbs_dn5);
        let eq14_e1089_d_n6: f64 = (p.p87 * locals.var_qbs_dn6);
        let eq14_e1089_d_n7: f64 = (p.p87 * locals.var_qbs_dn7);
        let eq14_e1089_d_n8: f64 = (p.p87 * locals.var_qbs_dn8);
        let eq14_e1089_d_n9: f64 = (p.p87 * locals.var_qbs_dn9);
        let eq14_e1089_d_n10: f64 = (p.p87 * locals.var_qbs_dn10);
        let eq14_e1089_d_n13: f64 = (p.p87 * locals.var_qbs_dn13);
        let eq14_e1089_q: f64 = (p.p87 * eq14_e1088_q);
        let eq14_reactive_node_derivatives: [f64; 18] = [eq14_e1089_d_n0, 0.0, eq14_e1089_d_n2, 0.0, eq14_e1089_d_n4, eq14_e1089_d_n5, eq14_e1089_d_n6, eq14_e1089_d_n7, eq14_e1089_d_n8, eq14_e1089_d_n9, eq14_e1089_d_n10, 0.0, 0.0, eq14_e1089_d_n13, 0.0, 0.0, 0.0, 0.0];
        let eq14_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e1092_q: f64 = locals.var_qbd;
        let eq15_e1093: f64 = (p.p87 * locals.var_qbd);
        let eq15_e1093_d_n0: f64 = (p.p87 * locals.var_qbd_dn0);
        let eq15_e1093_d_n2: f64 = (p.p87 * locals.var_qbd_dn2);
        let eq15_e1093_d_n4: f64 = (p.p87 * locals.var_qbd_dn4);
        let eq15_e1093_d_n5: f64 = (p.p87 * locals.var_qbd_dn5);
        let eq15_e1093_d_n6: f64 = (p.p87 * locals.var_qbd_dn6);
        let eq15_e1093_d_n7: f64 = (p.p87 * locals.var_qbd_dn7);
        let eq15_e1093_d_n8: f64 = (p.p87 * locals.var_qbd_dn8);
        let eq15_e1093_d_n9: f64 = (p.p87 * locals.var_qbd_dn9);
        let eq15_e1093_d_n10: f64 = (p.p87 * locals.var_qbd_dn10);
        let eq15_e1093_d_n13: f64 = (p.p87 * locals.var_qbd_dn13);
        let eq15_e1093_d_n15: f64 = (p.p87 * locals.var_qbd_dn15);
        let eq15_e1093_d_n16: f64 = (p.p87 * locals.var_qbd_dn16);
        let eq15_e1093_d_n17: f64 = (p.p87 * locals.var_qbd_dn17);
        let eq15_e1093_q: f64 = (p.p87 * eq15_e1092_q);
        let eq15_reactive_node_derivatives: [f64; 18] = [eq15_e1093_d_n0, 0.0, eq15_e1093_d_n2, 0.0, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, 0.0, 0.0, eq15_e1093_d_n13, 0.0, eq15_e1093_d_n15, eq15_e1093_d_n16, eq15_e1093_d_n17];
        let eq15_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[0]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1112, eq18_e1112_d_n0, eq18_e1112_d_n2, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n13, eq18_e1112_q,) = {
    if (locals.var_guard2409 != 0.0) {
        let eq18_e1109_q: f64 = locals.var_qbsi;
        let eq18_e1110: f64 = (p.p87 * locals.var_qbsi);
        let eq18_e1110_d_n0: f64 = (p.p87 * locals.var_qbsi_dn0);
        let eq18_e1110_d_n2: f64 = (p.p87 * locals.var_qbsi_dn2);
        let eq18_e1110_d_n4: f64 = (p.p87 * locals.var_qbsi_dn4);
        let eq18_e1110_d_n5: f64 = (p.p87 * locals.var_qbsi_dn5);
        let eq18_e1110_d_n6: f64 = (p.p87 * locals.var_qbsi_dn6);
        let eq18_e1110_d_n7: f64 = (p.p87 * locals.var_qbsi_dn7);
        let eq18_e1110_d_n8: f64 = (p.p87 * locals.var_qbsi_dn8);
        let eq18_e1110_d_n9: f64 = (p.p87 * locals.var_qbsi_dn9);
        let eq18_e1110_d_n10: f64 = (p.p87 * locals.var_qbsi_dn10);
        let eq18_e1110_d_n13: f64 = (p.p87 * locals.var_qbsi_dn13);
        let eq18_e1110_q: f64 = (p.p87 * eq18_e1109_q);
        (eq18_e1110, eq18_e1110_d_n0, eq18_e1110_d_n2, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n13, eq18_e1110_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_reactive_node_derivatives: [f64; 18] = [eq18_e1112_d_n0, 0.0, eq18_e1112_d_n2, 0.0, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, 0.0, 0.0, eq18_e1112_d_n13, 0.0, 0.0, 0.0, 0.0];
        let eq18_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1119, eq19_e1119_d_n0, eq19_e1119_d_n2, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n13, eq19_e1119_q,) = {
    if (locals.var_guard2409 != 0.0) {
        let eq19_e1116_q: f64 = locals.var_qbdi;
        let eq19_e1117: f64 = (p.p87 * locals.var_qbdi);
        let eq19_e1117_d_n0: f64 = (p.p87 * locals.var_qbdi_dn0);
        let eq19_e1117_d_n2: f64 = (p.p87 * locals.var_qbdi_dn2);
        let eq19_e1117_d_n4: f64 = (p.p87 * locals.var_qbdi_dn4);
        let eq19_e1117_d_n5: f64 = (p.p87 * locals.var_qbdi_dn5);
        let eq19_e1117_d_n6: f64 = (p.p87 * locals.var_qbdi_dn6);
        let eq19_e1117_d_n7: f64 = (p.p87 * locals.var_qbdi_dn7);
        let eq19_e1117_d_n8: f64 = (p.p87 * locals.var_qbdi_dn8);
        let eq19_e1117_d_n9: f64 = (p.p87 * locals.var_qbdi_dn9);
        let eq19_e1117_d_n10: f64 = (p.p87 * locals.var_qbdi_dn10);
        let eq19_e1117_d_n13: f64 = (p.p87 * locals.var_qbdi_dn13);
        let eq19_e1117_q: f64 = (p.p87 * eq19_e1116_q);
        (eq19_e1117, eq19_e1117_d_n0, eq19_e1117_d_n2, eq19_e1117_d_n4, eq19_e1117_d_n5, eq19_e1117_d_n6, eq19_e1117_d_n7, eq19_e1117_d_n8, eq19_e1117_d_n9, eq19_e1117_d_n10, eq19_e1117_d_n13, eq19_e1117_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 18] = [eq19_e1119_d_n0, 0.0, eq19_e1119_d_n2, 0.0, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, 0.0, 0.0, eq19_e1119_d_n13, 0.0, 0.0, 0.0, 0.0];
        let eq19_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq27_e1163: f64 = (locals.var_qg + locals.var_qg_nqs);
        let eq27_e1164_q: f64 = eq27_e1163;
        let eq27_e1165: f64 = (p.p87 * eq27_e1163);
        let eq27_e1165_d_n0: f64 = (p.p87 * locals.var_qg_dn0);
        let eq27_e1165_d_n2: f64 = (p.p87 * locals.var_qg_dn2);
        let eq27_e1165_d_n4: f64 = (p.p87 * locals.var_qg_dn4);
        let eq27_e1165_d_n5: f64 = (p.p87 * locals.var_qg_dn5);
        let eq27_e1165_d_n6: f64 = (p.p87 * locals.var_qg_dn6);
        let eq27_e1165_d_n7: f64 = (p.p87 * locals.var_qg_dn7);
        let eq27_e1165_d_n8: f64 = (p.p87 * locals.var_qg_dn8);
        let eq27_e1165_d_n9: f64 = (p.p87 * locals.var_qg_dn9);
        let eq27_e1165_d_n10: f64 = (p.p87 * locals.var_qg_dn10);
        let eq27_e1165_d_n11: f64 = (p.p87 * locals.var_qg_nqs_dn11);
        let eq27_e1165_d_n12: f64 = (p.p87 * locals.var_qg_nqs_dn12);
        let eq27_e1165_d_n13: f64 = (p.p87 * locals.var_qg_dn13);
        let eq27_e1165_q: f64 = (p.p87 * eq27_e1164_q);
        let eq27_reactive_node_derivatives: [f64; 18] = [eq27_e1165_d_n0, 0.0, eq27_e1165_d_n2, 0.0, eq27_e1165_d_n4, eq27_e1165_d_n5, eq27_e1165_d_n6, eq27_e1165_d_n7, eq27_e1165_d_n8, eq27_e1165_d_n9, eq27_e1165_d_n10, eq27_e1165_d_n11, eq27_e1165_d_n12, eq27_e1165_d_n13, 0.0, 0.0, 0.0, 0.0];
        let eq27_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e1169: f64 = (locals.var_qd + locals.var_qd_nqs);
        let eq28_e1169_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);
        let eq28_e1169_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);
        let eq28_e1169_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);
        let eq28_e1169_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);
        let eq28_e1169_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);
        let eq28_e1169_d_n7: f64 = (locals.var_qd_dn7 + locals.var_qd_nqs_dn7);
        let eq28_e1169_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);
        let eq28_e1169_d_n9: f64 = (locals.var_qd_dn9 + locals.var_qd_nqs_dn9);
        let eq28_e1169_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);
        let eq28_e1169_d_n13: f64 = (locals.var_qd_dn13 + locals.var_qd_nqs_dn13);
        let eq28_e1170_q: f64 = eq28_e1169;
        let eq28_e1171: f64 = (p.p87 * eq28_e1169);
        let eq28_e1171_d_n0: f64 = (p.p87 * eq28_e1169_d_n0);
        let eq28_e1171_d_n2: f64 = (p.p87 * eq28_e1169_d_n2);
        let eq28_e1171_d_n4: f64 = (p.p87 * eq28_e1169_d_n4);
        let eq28_e1171_d_n5: f64 = (p.p87 * eq28_e1169_d_n5);
        let eq28_e1171_d_n6: f64 = (p.p87 * eq28_e1169_d_n6);
        let eq28_e1171_d_n7: f64 = (p.p87 * eq28_e1169_d_n7);
        let eq28_e1171_d_n8: f64 = (p.p87 * eq28_e1169_d_n8);
        let eq28_e1171_d_n9: f64 = (p.p87 * eq28_e1169_d_n9);
        let eq28_e1171_d_n10: f64 = (p.p87 * eq28_e1169_d_n10);
        let eq28_e1171_d_n11: f64 = (p.p87 * locals.var_qd_nqs_dn11);
        let eq28_e1171_d_n13: f64 = (p.p87 * eq28_e1169_d_n13);
        let eq28_e1171_q: f64 = (p.p87 * eq28_e1170_q);
        let eq28_reactive_node_derivatives: [f64; 18] = [eq28_e1171_d_n0, 0.0, eq28_e1171_d_n2, 0.0, eq28_e1171_d_n4, eq28_e1171_d_n5, eq28_e1171_d_n6, eq28_e1171_d_n7, eq28_e1171_d_n8, eq28_e1171_d_n9, eq28_e1171_d_n10, eq28_e1171_d_n11, 0.0, eq28_e1171_d_n13, 0.0, 0.0, 0.0, 0.0];
        let eq28_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e1176: f64 = (locals.var_qg_nqs + locals.var_qd_nqs);
        let eq29_e1176_d_n11: f64 = (locals.var_qg_nqs_dn11 + locals.var_qd_nqs_dn11);
        let eq29_e1178: f64 = (eq29_e1176 + locals.var_qs_nqs);
        let eq29_e1178_d_n0: f64 = (locals.var_qd_nqs_dn0 + locals.var_qs_nqs_dn0);
        let eq29_e1178_d_n2: f64 = (locals.var_qd_nqs_dn2 + locals.var_qs_nqs_dn2);
        let eq29_e1178_d_n4: f64 = (locals.var_qd_nqs_dn4 + locals.var_qs_nqs_dn4);
        let eq29_e1178_d_n5: f64 = (locals.var_qd_nqs_dn5 + locals.var_qs_nqs_dn5);
        let eq29_e1178_d_n6: f64 = (locals.var_qd_nqs_dn6 + locals.var_qs_nqs_dn6);
        let eq29_e1178_d_n7: f64 = (locals.var_qd_nqs_dn7 + locals.var_qs_nqs_dn7);
        let eq29_e1178_d_n8: f64 = (locals.var_qd_nqs_dn8 + locals.var_qs_nqs_dn8);
        let eq29_e1178_d_n9: f64 = (locals.var_qd_nqs_dn9 + locals.var_qs_nqs_dn9);
        let eq29_e1178_d_n10: f64 = (locals.var_qd_nqs_dn10 + locals.var_qs_nqs_dn10);
        let eq29_e1178_d_n11: f64 = (eq29_e1176_d_n11 + locals.var_qs_nqs_dn11);
        let eq29_e1178_d_n13: f64 = (locals.var_qd_nqs_dn13 + locals.var_qs_nqs_dn13);
        let eq29_e1179: f64 = (locals.var_qb - eq29_e1178);
        let eq29_e1179_d_n0: f64 = (locals.var_qb_dn0 - eq29_e1178_d_n0);
        let eq29_e1179_d_n2: f64 = (locals.var_qb_dn2 - eq29_e1178_d_n2);
        let eq29_e1179_d_n4: f64 = (locals.var_qb_dn4 - eq29_e1178_d_n4);
        let eq29_e1179_d_n5: f64 = (locals.var_qb_dn5 - eq29_e1178_d_n5);
        let eq29_e1179_d_n6: f64 = (locals.var_qb_dn6 - eq29_e1178_d_n6);
        let eq29_e1179_d_n7: f64 = (locals.var_qb_dn7 - eq29_e1178_d_n7);
        let eq29_e1179_d_n8: f64 = (locals.var_qb_dn8 - eq29_e1178_d_n8);
        let eq29_e1179_d_n9: f64 = (locals.var_qb_dn9 - eq29_e1178_d_n9);
        let eq29_e1179_d_n10: f64 = (locals.var_qb_dn10 - eq29_e1178_d_n10);
        let eq29_e1179_d_n13: f64 = (locals.var_qb_dn13 - eq29_e1178_d_n13);
        let eq29_e1180_q: f64 = eq29_e1179;
        let eq29_e1181: f64 = (p.p87 * eq29_e1179);
        let eq29_e1181_d_n0: f64 = (p.p87 * eq29_e1179_d_n0);
        let eq29_e1181_d_n2: f64 = (p.p87 * eq29_e1179_d_n2);
        let eq29_e1181_d_n4: f64 = (p.p87 * eq29_e1179_d_n4);
        let eq29_e1181_d_n5: f64 = (p.p87 * eq29_e1179_d_n5);
        let eq29_e1181_d_n6: f64 = (p.p87 * eq29_e1179_d_n6);
        let eq29_e1181_d_n7: f64 = (p.p87 * eq29_e1179_d_n7);
        let eq29_e1181_d_n8: f64 = (p.p87 * eq29_e1179_d_n8);
        let eq29_e1181_d_n9: f64 = (p.p87 * eq29_e1179_d_n9);
        let eq29_e1181_d_n10: f64 = (p.p87 * eq29_e1179_d_n10);
        let eq29_e1181_d_n11: f64 = (p.p87 * (-eq29_e1178_d_n11));
        let eq29_e1181_d_n12: f64 = (p.p87 * (-locals.var_qg_nqs_dn12));
        let eq29_e1181_d_n13: f64 = (p.p87 * eq29_e1179_d_n13);
        let eq29_e1181_q: f64 = (p.p87 * eq29_e1180_q);
        let eq29_reactive_node_derivatives: [f64; 18] = [eq29_e1181_d_n0, 0.0, eq29_e1181_d_n2, 0.0, eq29_e1181_d_n4, eq29_e1181_d_n5, eq29_e1181_d_n6, eq29_e1181_d_n7, eq29_e1181_d_n8, eq29_e1181_d_n9, eq29_e1181_d_n10, eq29_e1181_d_n11, eq29_e1181_d_n12, eq29_e1181_d_n13, 0.0, 0.0, 0.0, 0.0];
        let eq29_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e1184_q: f64 = locals.var_qgext;
        let eq30_e1185: f64 = (p.p87 * locals.var_qgext);
        let eq30_e1185_d_n0: f64 = (p.p87 * locals.var_qgext_dn0);
        let eq30_e1185_d_n2: f64 = (p.p87 * locals.var_qgext_dn2);
        let eq30_e1185_d_n4: f64 = (p.p87 * locals.var_qgext_dn4);
        let eq30_e1185_d_n5: f64 = (p.p87 * locals.var_qgext_dn5);
        let eq30_e1185_d_n6: f64 = (p.p87 * locals.var_qgext_dn6);
        let eq30_e1185_d_n7: f64 = (p.p87 * locals.var_qgext_dn7);
        let eq30_e1185_d_n8: f64 = (p.p87 * locals.var_qgext_dn8);
        let eq30_e1185_d_n9: f64 = (p.p87 * locals.var_qgext_dn9);
        let eq30_e1185_d_n10: f64 = (p.p87 * locals.var_qgext_dn10);
        let eq30_e1185_d_n13: f64 = (p.p87 * locals.var_qgext_dn13);
        let eq30_e1185_q: f64 = (p.p87 * eq30_e1184_q);
        let eq30_reactive_node_derivatives: [f64; 18] = [eq30_e1185_d_n0, 0.0, eq30_e1185_d_n2, 0.0, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, 0.0, 0.0, eq30_e1185_d_n13, 0.0, 0.0, 0.0, 0.0];
        let eq30_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188_q: f64 = locals.var_qdext;
        let eq31_e1189: f64 = (p.p87 * locals.var_qdext);
        let eq31_e1189_d_n0: f64 = (p.p87 * locals.var_qdext_dn0);
        let eq31_e1189_d_n2: f64 = (p.p87 * locals.var_qdext_dn2);
        let eq31_e1189_d_n4: f64 = (p.p87 * locals.var_qdext_dn4);
        let eq31_e1189_d_n5: f64 = (p.p87 * locals.var_qdext_dn5);
        let eq31_e1189_d_n6: f64 = (p.p87 * locals.var_qdext_dn6);
        let eq31_e1189_d_n7: f64 = (p.p87 * locals.var_qdext_dn7);
        let eq31_e1189_d_n8: f64 = (p.p87 * locals.var_qdext_dn8);
        let eq31_e1189_d_n9: f64 = (p.p87 * locals.var_qdext_dn9);
        let eq31_e1189_d_n10: f64 = (p.p87 * locals.var_qdext_dn10);
        let eq31_e1189_d_n13: f64 = (p.p87 * locals.var_qdext_dn13);
        let eq31_e1189_q: f64 = (p.p87 * eq31_e1188_q);
        let eq31_reactive_node_derivatives: [f64; 18] = [eq31_e1189_d_n0, 0.0, eq31_e1189_d_n2, 0.0, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, 0.0, 0.0, eq31_e1189_d_n13, 0.0, 0.0, 0.0, 0.0];
        let eq31_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192_q: f64 = locals.var_qbext;
        let eq32_e1193: f64 = (p.p87 * locals.var_qbext);
        let eq32_e1193_d_n0: f64 = (p.p87 * locals.var_qbext_dn0);
        let eq32_e1193_d_n2: f64 = (p.p87 * locals.var_qbext_dn2);
        let eq32_e1193_d_n4: f64 = (p.p87 * locals.var_qbext_dn4);
        let eq32_e1193_d_n5: f64 = (p.p87 * locals.var_qbext_dn5);
        let eq32_e1193_d_n6: f64 = (p.p87 * locals.var_qbext_dn6);
        let eq32_e1193_d_n7: f64 = (p.p87 * locals.var_qbext_dn7);
        let eq32_e1193_d_n8: f64 = (p.p87 * locals.var_qbext_dn8);
        let eq32_e1193_d_n9: f64 = (p.p87 * locals.var_qbext_dn9);
        let eq32_e1193_d_n10: f64 = (p.p87 * locals.var_qbext_dn10);
        let eq32_e1193_d_n13: f64 = (p.p87 * locals.var_qbext_dn13);
        let eq32_e1193_q: f64 = (p.p87 * eq32_e1192_q);
        let eq32_reactive_node_derivatives: [f64; 18] = [eq32_e1193_d_n0, 0.0, eq32_e1193_d_n2, 0.0, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, 0.0, 0.0, eq32_e1193_d_n13, 0.0, 0.0, 0.0, 0.0];
        let eq32_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e1195: f64 = (-p.p87);
        let eq33_e1197_q: f64 = locals.var_qfd;
        let eq33_e1198: f64 = (eq33_e1195 * locals.var_qfd);
        let eq33_e1198_d_n0: f64 = (eq33_e1195 * locals.var_qfd_dn0);
        let eq33_e1198_d_n2: f64 = (eq33_e1195 * locals.var_qfd_dn2);
        let eq33_e1198_d_n6: f64 = (eq33_e1195 * locals.var_qfd_dn6);
        let eq33_e1198_q: f64 = (eq33_e1195 * eq33_e1197_q);
        stamper.stamp_current_reactive_node3(
            Some(nodes[6]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq33_e1198_d_n0),
            nodes[2],
            multiplicity * (eq33_e1198_d_n2),
            nodes[6],
            multiplicity * (eq33_e1198_d_n6),
        );
        let eq34_e1200: f64 = (-p.p87);
        let eq34_e1202_q: f64 = locals.var_qfs;
        let eq34_e1203: f64 = (eq34_e1200 * locals.var_qfs);
        let eq34_e1203_d_n2: f64 = (eq34_e1200 * locals.var_qfs_dn2);
        let eq34_e1203_d_n6: f64 = (eq34_e1200 * locals.var_qfs_dn6);
        let eq34_e1203_q: f64 = (eq34_e1200 * eq34_e1202_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[6]),
            Some(nodes[2]),
            nodes[2],
            multiplicity * (eq34_e1203_d_n2),
            nodes[6],
            multiplicity * (eq34_e1203_d_n6),
        );
        let eq40_e1232: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq40_e1232_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq40_e1232_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq40_e1232_d_n4: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn4);
        let eq40_e1232_d_n5: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn5);
        let eq40_e1232_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq40_e1232_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq40_e1232_d_n8: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn8);
        let eq40_e1232_d_n9: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn9);
        let eq40_e1232_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq40_e1232_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq40_e1233_q: f64 = eq40_e1232;
        let eq40_reactive_node_derivatives: [f64; 18] = [eq40_e1232_d_n0, 0.0, eq40_e1232_d_n2, 0.0, eq40_e1232_d_n4, eq40_e1232_d_n5, eq40_e1232_d_n6, eq40_e1232_d_n7, eq40_e1232_d_n8, eq40_e1232_d_n9, eq40_e1232_d_n10, 0.0, 0.0, eq40_e1232_d_n13, locals.var_sigrat_s, 0.0, 0.0, 0.0];
        let eq40_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e1236: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq41_e1236_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq41_e1236_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq41_e1236_d_n4: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn4);
        let eq41_e1236_d_n5: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn5);
        let eq41_e1236_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq41_e1236_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq41_e1236_d_n8: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn8);
        let eq41_e1236_d_n9: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn9);
        let eq41_e1236_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq41_e1236_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq41_e1237_q: f64 = eq41_e1236;
        let eq41_reactive_node_derivatives: [f64; 18] = [eq41_e1236_d_n0, 0.0, eq41_e1236_d_n2, 0.0, eq41_e1236_d_n4, eq41_e1236_d_n5, eq41_e1236_d_n6, eq41_e1236_d_n7, eq41_e1236_d_n8, eq41_e1236_d_n9, eq41_e1236_d_n10, 0.0, 0.0, eq41_e1236_d_n13, locals.var_sigrat_d, 0.0, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1358, eq61_e1358_d_n11, eq61_e1358_q,) = {
    if (p.p28 != 0.0) {
        let eq61_e1355: f64 = (locals.var_cqi * (nv11 - 0.0));
        let eq61_e1356_q: f64 = eq61_e1355;
        (eq61_e1355, locals.var_cqi, eq61_e1356_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[11]),
            None,
            nodes[11],
            multiplicity * (eq61_e1358_d_n11),
        );
        let (eq62_e1365, eq62_e1365_d_n12, eq62_e1365_q,) = {
    if (p.p28 != 0.0) {
        let eq62_e1362: f64 = (locals.var_cqb * (nv12 - 0.0));
        let eq62_e1363_q: f64 = eq62_e1362;
        (eq62_e1362, locals.var_cqb, eq62_e1363_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (eq62_e1365_d_n12),
        );
        let (eq66_e1384, eq66_e1384_d_n13, eq66_e1384_q,) = {
    if (p.p29 != 0.0) {
        let eq66_e1382_q: f64 = (nv13 - 0.0);
        ((nv13 - 0.0), 1.0, eq66_e1382_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq66_e1384_d_n13),
        );
    }
}
