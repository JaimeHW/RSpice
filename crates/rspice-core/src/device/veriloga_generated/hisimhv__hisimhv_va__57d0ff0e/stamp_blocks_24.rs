#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_384(
        locals: &mut StampLocals,
    ) {
        let (assign105150_e157513, assign105150_e157513_d_n0, assign105150_e157513_d_n2, assign105150_e157513_d_n4, assign105150_e157513_d_n5, assign105150_e157513_d_n6, assign105150_e157513_d_n7, assign105150_e157513_d_n8, assign105150_e157513_d_n9, assign105150_e157513_d_n10, assign105150_e157513_d_n11, assign105150_e157513_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105150_e157513;
        locals.var_dnm_dn0 = assign105150_e157513_d_n0;
        locals.var_dnm_dn2 = assign105150_e157513_d_n2;
        locals.var_dnm_dn4 = assign105150_e157513_d_n4;
        locals.var_dnm_dn5 = assign105150_e157513_d_n5;
        locals.var_dnm_dn6 = assign105150_e157513_d_n6;
        locals.var_dnm_dn7 = assign105150_e157513_d_n7;
        locals.var_dnm_dn8 = assign105150_e157513_d_n8;
        locals.var_dnm_dn9 = assign105150_e157513_d_n9;
        locals.var_dnm_dn10 = assign105150_e157513_d_n10;
        locals.var_dnm_dn11 = assign105150_e157513_d_n11;
        locals.var_dnm_dn14 = assign105150_e157513_d_n14;

        let assign105160_e157528: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2386 = assign105160_e157528;

        let assign105170_e157531: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2387 = assign105170_e157531;

        let (assign105180_e157544,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) && (locals.var_guard2387 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105180_e157544;

        let assign105190_e157547: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2388 = assign105190_e157547;

        let (assign105200_e157563,) = {
    if ((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) && (locals.var_guard2387 == 0.0)) && (locals.var_guard2388 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105200_e157563;

        let assign105210_e157566: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2389 = assign105210_e157566;

        let (assign105220_e157585,) = {
    if (((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) && (locals.var_guard2387 == 0.0)) && (locals.var_guard2388 == 0.0)) && (locals.var_guard2389 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105220_e157585;

        let assign105230_e157588: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2390 = assign105230_e157588;

        let (assign105240_e157610,) = {
    if ((((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) && (locals.var_guard2387 == 0.0)) && (locals.var_guard2388 == 0.0)) && (locals.var_guard2389 == 0.0)) && (locals.var_guard2390 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105240_e157610;

        let (assign105250_e157621,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105250_e157621;

        let mut assign105260_loop_guard: usize = 0;
        while {
            let assign105260_cond_e157633: f64 = if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105260_cond_e157633 != 0.0
        } {
            assign105260_loop_guard += 1;
            assert!(assign105260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105260_body0_e157645, assign105260_body0_e157645_d_n0, assign105260_body0_e157645_d_n2, assign105260_body0_e157645_d_n4, assign105260_body0_e157645_d_n5, assign105260_body0_e157645_d_n6, assign105260_body0_e157645_d_n7, assign105260_body0_e157645_d_n8, assign105260_body0_e157645_d_n9, assign105260_body0_e157645_d_n10, assign105260_body0_e157645_d_n11, assign105260_body0_e157645_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) {
        let assign105260_body0_e157643: f64 = (locals.var_dnm).sqrt();
        (assign105260_body0_e157643, (locals.var_dnm_dn0 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn2 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn4 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn5 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn6 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn7 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn8 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn9 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn10 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn11 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn14 / (2.0 * assign105260_body0_e157643)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign105260_body0_e157645;
            locals.var_dnm_dn0 = assign105260_body0_e157645_d_n0;
            locals.var_dnm_dn2 = assign105260_body0_e157645_d_n2;
            locals.var_dnm_dn4 = assign105260_body0_e157645_d_n4;
            locals.var_dnm_dn5 = assign105260_body0_e157645_d_n5;
            locals.var_dnm_dn6 = assign105260_body0_e157645_d_n6;
            locals.var_dnm_dn7 = assign105260_body0_e157645_d_n7;
            locals.var_dnm_dn8 = assign105260_body0_e157645_d_n8;
            locals.var_dnm_dn9 = assign105260_body0_e157645_d_n9;
            locals.var_dnm_dn10 = assign105260_body0_e157645_d_n10;
            locals.var_dnm_dn11 = assign105260_body0_e157645_d_n11;
            locals.var_dnm_dn14 = assign105260_body0_e157645_d_n14;
            let (assign105260_body1_e157658,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) {
        let assign105260_body1_e157656: f64 = (locals.var_m0 + 1.0);
        (assign105260_body1_e157656,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign105260_body1_e157658;
        }

        let (assign105270_e157681, assign105270_e157681_d_n0, assign105270_e157681_d_n2, assign105270_e157681_d_n4, assign105270_e157681_d_n5, assign105270_e157681_d_n6, assign105270_e157681_d_n7, assign105270_e157681_d_n8, assign105270_e157681_d_n9, assign105270_e157681_d_n10, assign105270_e157681_d_n11, assign105270_e157681_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 == 0.0)) {
        let (assign105270_e157679, assign105270_e157679_d_n0, assign105270_e157679_d_n2, assign105270_e157679_d_n4, assign105270_e157679_d_n5, assign105270_e157679_d_n6, assign105270_e157679_d_n7, assign105270_e157679_d_n8, assign105270_e157679_d_n9, assign105270_e157679_d_n10, assign105270_e157679_d_n11, assign105270_e157679_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105270_e157676: f64 = (2.0 * 2.0);
                let assign105270_e157677: f64 = (1.0 / assign105270_e157676);
                let assign105270_e157678: f64 = (locals.var_dnm).powf(assign105270_e157677);
                (assign105270_e157678, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn11)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn14)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign105270_e157679, assign105270_e157679_d_n0, assign105270_e157679_d_n2, assign105270_e157679_d_n4, assign105270_e157679_d_n5, assign105270_e157679_d_n6, assign105270_e157679_d_n7, assign105270_e157679_d_n8, assign105270_e157679_d_n9, assign105270_e157679_d_n10, assign105270_e157679_d_n11, assign105270_e157679_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105270_e157681;
        locals.var_dnm_dn0 = assign105270_e157681_d_n0;
        locals.var_dnm_dn2 = assign105270_e157681_d_n2;
        locals.var_dnm_dn4 = assign105270_e157681_d_n4;
        locals.var_dnm_dn5 = assign105270_e157681_d_n5;
        locals.var_dnm_dn6 = assign105270_e157681_d_n6;
        locals.var_dnm_dn7 = assign105270_e157681_d_n7;
        locals.var_dnm_dn8 = assign105270_e157681_d_n8;
        locals.var_dnm_dn9 = assign105270_e157681_d_n9;
        locals.var_dnm_dn10 = assign105270_e157681_d_n10;
        locals.var_dnm_dn11 = assign105270_e157681_d_n11;
        locals.var_dnm_dn14 = assign105270_e157681_d_n14;

        let (assign105280_e157692, assign105280_e157692_d_n0, assign105280_e157692_d_n2, assign105280_e157692_d_n4, assign105280_e157692_d_n5, assign105280_e157692_d_n6, assign105280_e157692_d_n7, assign105280_e157692_d_n8, assign105280_e157692_d_n9, assign105280_e157692_d_n10, assign105280_e157692_d_n11, assign105280_e157692_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105280_e157690: f64 = (1.0 / locals.var_dnm);
        (assign105280_e157690, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105280_e157692;
        locals.var_dnm_dn0 = assign105280_e157692_d_n0;
        locals.var_dnm_dn2 = assign105280_e157692_d_n2;
        locals.var_dnm_dn4 = assign105280_e157692_d_n4;
        locals.var_dnm_dn5 = assign105280_e157692_d_n5;
        locals.var_dnm_dn6 = assign105280_e157692_d_n6;
        locals.var_dnm_dn7 = assign105280_e157692_d_n7;
        locals.var_dnm_dn8 = assign105280_e157692_d_n8;
        locals.var_dnm_dn9 = assign105280_e157692_d_n9;
        locals.var_dnm_dn10 = assign105280_e157692_d_n10;
        locals.var_dnm_dn11 = assign105280_e157692_d_n11;
        locals.var_dnm_dn14 = assign105280_e157692_d_n14;

        let (assign105290_e157705, assign105290_e157705_d_n0, assign105290_e157705_d_n2, assign105290_e157705_d_n4, assign105290_e157705_d_n5, assign105290_e157705_d_n6, assign105290_e157705_d_n7, assign105290_e157705_d_n8, assign105290_e157705_d_n9, assign105290_e157705_d_n10, assign105290_e157705_d_n11, assign105290_e157705_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105290_e157701: f64 = (locals.var_tmf1 * 1e-25);
        let assign105290_e157703: f64 = (assign105290_e157701 * locals.var_dnm);
        (assign105290_e157703, (((locals.var_tmf1_dn0 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign105290_e157705;
        locals.var_tmf0_dn0 = assign105290_e157705_d_n0;
        locals.var_tmf0_dn2 = assign105290_e157705_d_n2;
        locals.var_tmf0_dn4 = assign105290_e157705_d_n4;
        locals.var_tmf0_dn5 = assign105290_e157705_d_n5;
        locals.var_tmf0_dn6 = assign105290_e157705_d_n6;
        locals.var_tmf0_dn7 = assign105290_e157705_d_n7;
        locals.var_tmf0_dn8 = assign105290_e157705_d_n8;
        locals.var_tmf0_dn9 = assign105290_e157705_d_n9;
        locals.var_tmf0_dn10 = assign105290_e157705_d_n10;
        locals.var_tmf0_dn11 = assign105290_e157705_d_n11;
        locals.var_tmf0_dn14 = assign105290_e157705_d_n14;

        let (assign105300_e157720, assign105300_e157720_d_n0, assign105300_e157720_d_n2, assign105300_e157720_d_n4, assign105300_e157720_d_n5, assign105300_e157720_d_n6, assign105300_e157720_d_n7, assign105300_e157720_d_n8, assign105300_e157720_d_n9, assign105300_e157720_d_n10, assign105300_e157720_d_n11, assign105300_e157720_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105300_e157714: f64 = (1e-25 * locals.var_xmp);
        let assign105300_e157716: f64 = (assign105300_e157714 * locals.var_dnm);
        let assign105300_e157718: f64 = (assign105300_e157716 / locals.var_arg);
        (assign105300_e157718, ((((((1e-25 * locals.var_xmp_dn0) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn0)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn2) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn2)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn4) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn4)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn5) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn5)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn6) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn6)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn7) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn7)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn8) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn8)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn9) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn9)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn10) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn10)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn11) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn11)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn14) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn14)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105300_e157720;
        locals.var_t0_dn0 = assign105300_e157720_d_n0;
        locals.var_t0_dn2 = assign105300_e157720_d_n2;
        locals.var_t0_dn4 = assign105300_e157720_d_n4;
        locals.var_t0_dn5 = assign105300_e157720_d_n5;
        locals.var_t0_dn6 = assign105300_e157720_d_n6;
        locals.var_t0_dn7 = assign105300_e157720_d_n7;
        locals.var_t0_dn8 = assign105300_e157720_d_n8;
        locals.var_t0_dn9 = assign105300_e157720_d_n9;
        locals.var_t0_dn10 = assign105300_e157720_d_n10;
        locals.var_t0_dn11 = assign105300_e157720_d_n11;
        locals.var_t0_dn14 = assign105300_e157720_d_n14;

        let (assign105310_e157733, assign105310_e157733_d_n0, assign105310_e157733_d_n2, assign105310_e157733_d_n4, assign105310_e157733_d_n5, assign105310_e157733_d_n6, assign105310_e157733_d_n7, assign105310_e157733_d_n8, assign105310_e157733_d_n9, assign105310_e157733_d_n10, assign105310_e157733_d_n11, assign105310_e157733_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105310_e157729: f64 = 1e-25;
        let assign105310_e157731: f64 = (assign105310_e157729 - locals.var_tmf0);
        (assign105310_e157731, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn14,)
    }
};
        locals.var_gd = assign105310_e157733;
        locals.var_gd_dn0 = assign105310_e157733_d_n0;
        locals.var_gd_dn2 = assign105310_e157733_d_n2;
        locals.var_gd_dn4 = assign105310_e157733_d_n4;
        locals.var_gd_dn5 = assign105310_e157733_d_n5;
        locals.var_gd_dn6 = assign105310_e157733_d_n6;
        locals.var_gd_dn7 = assign105310_e157733_d_n7;
        locals.var_gd_dn8 = assign105310_e157733_d_n8;
        locals.var_gd_dn9 = assign105310_e157733_d_n9;
        locals.var_gd_dn10 = assign105310_e157733_d_n10;
        locals.var_gd_dn11 = assign105310_e157733_d_n11;
        locals.var_gd_dn14 = assign105310_e157733_d_n14;

        let (assign105320_e157742, assign105320_e157742_d_n0, assign105320_e157742_d_n2, assign105320_e157742_d_n4, assign105320_e157742_d_n5, assign105320_e157742_d_n6, assign105320_e157742_d_n7, assign105320_e157742_d_n8, assign105320_e157742_d_n9, assign105320_e157742_d_n10, assign105320_e157742_d_n11, assign105320_e157742_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105320_e157742;
        locals.var_t0_dn0 = assign105320_e157742_d_n0;
        locals.var_t0_dn2 = assign105320_e157742_d_n2;
        locals.var_t0_dn4 = assign105320_e157742_d_n4;
        locals.var_t0_dn5 = assign105320_e157742_d_n5;
        locals.var_t0_dn6 = assign105320_e157742_d_n6;
        locals.var_t0_dn7 = assign105320_e157742_d_n7;
        locals.var_t0_dn8 = assign105320_e157742_d_n8;
        locals.var_t0_dn9 = assign105320_e157742_d_n9;
        locals.var_t0_dn10 = assign105320_e157742_d_n10;
        locals.var_t0_dn11 = assign105320_e157742_d_n11;
        locals.var_t0_dn14 = assign105320_e157742_d_n14;

        let (assign105330_e157752, assign105330_e157752_d_n0, assign105330_e157752_d_n2, assign105330_e157752_d_n4, assign105330_e157752_d_n5, assign105330_e157752_d_n6, assign105330_e157752_d_n7, assign105330_e157752_d_n8, assign105330_e157752_d_n9, assign105330_e157752_d_n10, assign105330_e157752_d_n11, assign105330_e157752_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 == 0.0)) {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn14,)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn14,)
    }
};
        locals.var_gd = assign105330_e157752;
        locals.var_gd_dn0 = assign105330_e157752_d_n0;
        locals.var_gd_dn2 = assign105330_e157752_d_n2;
        locals.var_gd_dn4 = assign105330_e157752_d_n4;
        locals.var_gd_dn5 = assign105330_e157752_d_n5;
        locals.var_gd_dn6 = assign105330_e157752_d_n6;
        locals.var_gd_dn7 = assign105330_e157752_d_n7;
        locals.var_gd_dn8 = assign105330_e157752_d_n8;
        locals.var_gd_dn9 = assign105330_e157752_d_n9;
        locals.var_gd_dn10 = assign105330_e157752_d_n10;
        locals.var_gd_dn11 = assign105330_e157752_d_n11;
        locals.var_gd_dn14 = assign105330_e157752_d_n14;

        let (assign105340_e157762, assign105340_e157762_d_n0, assign105340_e157762_d_n2, assign105340_e157762_d_n4, assign105340_e157762_d_n5, assign105340_e157762_d_n6, assign105340_e157762_d_n7, assign105340_e157762_d_n8, assign105340_e157762_d_n9, assign105340_e157762_d_n10, assign105340_e157762_d_n11, assign105340_e157762_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105340_e157762;
        locals.var_t0_dn0 = assign105340_e157762_d_n0;
        locals.var_t0_dn2 = assign105340_e157762_d_n2;
        locals.var_t0_dn4 = assign105340_e157762_d_n4;
        locals.var_t0_dn5 = assign105340_e157762_d_n5;
        locals.var_t0_dn6 = assign105340_e157762_d_n6;
        locals.var_t0_dn7 = assign105340_e157762_d_n7;
        locals.var_t0_dn8 = assign105340_e157762_d_n8;
        locals.var_t0_dn9 = assign105340_e157762_d_n9;
        locals.var_t0_dn10 = assign105340_e157762_d_n10;
        locals.var_t0_dn11 = assign105340_e157762_d_n11;
        locals.var_t0_dn14 = assign105340_e157762_d_n14;

        let (assign105350_e157771, assign105350_e157771_d_n0, assign105350_e157771_d_n2, assign105350_e157771_d_n4, assign105350_e157771_d_n5, assign105350_e157771_d_n6, assign105350_e157771_d_n7, assign105350_e157771_d_n8, assign105350_e157771_d_n9, assign105350_e157771_d_n10, assign105350_e157771_d_n11, assign105350_e157771_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign105350_e157769: f64 = (1.0 / locals.var_gd);
        (assign105350_e157769, (-(locals.var_gd_dn0 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn2 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn4 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn5 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn6 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn7 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn8 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn9 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn10 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn11 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn14 / (locals.var_gd * locals.var_gd))),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105350_e157771;
        locals.var_rdd_dn0 = assign105350_e157771_d_n0;
        locals.var_rdd_dn2 = assign105350_e157771_d_n2;
        locals.var_rdd_dn4 = assign105350_e157771_d_n4;
        locals.var_rdd_dn5 = assign105350_e157771_d_n5;
        locals.var_rdd_dn6 = assign105350_e157771_d_n6;
        locals.var_rdd_dn7 = assign105350_e157771_d_n7;
        locals.var_rdd_dn8 = assign105350_e157771_d_n8;
        locals.var_rdd_dn9 = assign105350_e157771_d_n9;
        locals.var_rdd_dn10 = assign105350_e157771_d_n10;
        locals.var_rdd_dn11 = assign105350_e157771_d_n11;
        locals.var_rdd_dn14 = assign105350_e157771_d_n14;

        let (assign105360_e157780, assign105360_e157780_d_n0, assign105360_e157780_d_n2, assign105360_e157780_d_n4, assign105360_e157780_d_n5, assign105360_e157780_d_n6, assign105360_e157780_d_n7, assign105360_e157780_d_n8, assign105360_e157780_d_n9, assign105360_e157780_d_n10, assign105360_e157780_d_n11, assign105360_e157780_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign105360_e157778: f64 = (locals.var_rdd / locals.var_weffld_nf);
        (assign105360_e157778, (locals.var_rdd_dn0 / locals.var_weffld_nf), (locals.var_rdd_dn2 / locals.var_weffld_nf), (locals.var_rdd_dn4 / locals.var_weffld_nf), (locals.var_rdd_dn5 / locals.var_weffld_nf), (locals.var_rdd_dn6 / locals.var_weffld_nf), (locals.var_rdd_dn7 / locals.var_weffld_nf), (locals.var_rdd_dn8 / locals.var_weffld_nf), (locals.var_rdd_dn9 / locals.var_weffld_nf), (locals.var_rdd_dn10 / locals.var_weffld_nf), (locals.var_rdd_dn11 / locals.var_weffld_nf), (locals.var_rdd_dn14 / locals.var_weffld_nf),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105360_e157780;
        locals.var_rdd_dn0 = assign105360_e157780_d_n0;
        locals.var_rdd_dn2 = assign105360_e157780_d_n2;
        locals.var_rdd_dn4 = assign105360_e157780_d_n4;
        locals.var_rdd_dn5 = assign105360_e157780_d_n5;
        locals.var_rdd_dn6 = assign105360_e157780_d_n6;
        locals.var_rdd_dn7 = assign105360_e157780_d_n7;
        locals.var_rdd_dn8 = assign105360_e157780_d_n8;
        locals.var_rdd_dn9 = assign105360_e157780_d_n9;
        locals.var_rdd_dn10 = assign105360_e157780_d_n10;
        locals.var_rdd_dn11 = assign105360_e157780_d_n11;
        locals.var_rdd_dn14 = assign105360_e157780_d_n14;

        let assign105370_e157784: f64 = (1000000.0 - 1000.0);
        let assign105370_e157789: f64 = if ((locals.var_rdd > assign105370_e157784) && (1000.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2391 = assign105370_e157789;

        let (assign105380_e157802, assign105380_e157802_d_n0, assign105380_e157802_d_n2, assign105380_e157802_d_n4, assign105380_e157802_d_n5, assign105380_e157802_d_n6, assign105380_e157802_d_n7, assign105380_e157802_d_n8, assign105380_e157802_d_n9, assign105380_e157802_d_n10, assign105380_e157802_d_n11, assign105380_e157802_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105380_e157798: f64 = (locals.var_rdd - 1000000.0);
        let assign105380_e157800: f64 = (assign105380_e157798 + 1000.0);
        (assign105380_e157800, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign105380_e157802;
        locals.var_tmf1_dn0 = assign105380_e157802_d_n0;
        locals.var_tmf1_dn2 = assign105380_e157802_d_n2;
        locals.var_tmf1_dn4 = assign105380_e157802_d_n4;
        locals.var_tmf1_dn5 = assign105380_e157802_d_n5;
        locals.var_tmf1_dn6 = assign105380_e157802_d_n6;
        locals.var_tmf1_dn7 = assign105380_e157802_d_n7;
        locals.var_tmf1_dn8 = assign105380_e157802_d_n8;
        locals.var_tmf1_dn9 = assign105380_e157802_d_n9;
        locals.var_tmf1_dn10 = assign105380_e157802_d_n10;
        locals.var_tmf1_dn11 = assign105380_e157802_d_n11;
        locals.var_tmf1_dn14 = assign105380_e157802_d_n14;

        let (assign105390_e157813, assign105390_e157813_d_n0, assign105390_e157813_d_n2, assign105390_e157813_d_n4, assign105390_e157813_d_n5, assign105390_e157813_d_n6, assign105390_e157813_d_n7, assign105390_e157813_d_n8, assign105390_e157813_d_n9, assign105390_e157813_d_n10, assign105390_e157813_d_n11, assign105390_e157813_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105390_e157811: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign105390_e157811, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign105390_e157813;
        locals.var_x2_dn0 = assign105390_e157813_d_n0;
        locals.var_x2_dn2 = assign105390_e157813_d_n2;
        locals.var_x2_dn4 = assign105390_e157813_d_n4;
        locals.var_x2_dn5 = assign105390_e157813_d_n5;
        locals.var_x2_dn6 = assign105390_e157813_d_n6;
        locals.var_x2_dn7 = assign105390_e157813_d_n7;
        locals.var_x2_dn8 = assign105390_e157813_d_n8;
        locals.var_x2_dn9 = assign105390_e157813_d_n9;
        locals.var_x2_dn10 = assign105390_e157813_d_n10;
        locals.var_x2_dn11 = assign105390_e157813_d_n11;
        locals.var_x2_dn14 = assign105390_e157813_d_n14;

        let (assign105400_e157824, assign105400_e157824_d_n0, assign105400_e157824_d_n2, assign105400_e157824_d_n4, assign105400_e157824_d_n5, assign105400_e157824_d_n6, assign105400_e157824_d_n7, assign105400_e157824_d_n8, assign105400_e157824_d_n9, assign105400_e157824_d_n10, assign105400_e157824_d_n11, assign105400_e157824_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105400_e157822: f64 = (1000.0 * 1000.0);
        (assign105400_e157822, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign105400_e157824;
        locals.var_xmax2_dn0 = assign105400_e157824_d_n0;
        locals.var_xmax2_dn2 = assign105400_e157824_d_n2;
        locals.var_xmax2_dn4 = assign105400_e157824_d_n4;
        locals.var_xmax2_dn5 = assign105400_e157824_d_n5;
        locals.var_xmax2_dn6 = assign105400_e157824_d_n6;
        locals.var_xmax2_dn7 = assign105400_e157824_d_n7;
        locals.var_xmax2_dn8 = assign105400_e157824_d_n8;
        locals.var_xmax2_dn9 = assign105400_e157824_d_n9;
        locals.var_xmax2_dn10 = assign105400_e157824_d_n10;
        locals.var_xmax2_dn11 = assign105400_e157824_d_n11;
        locals.var_xmax2_dn14 = assign105400_e157824_d_n14;

        let (assign105410_e157833, assign105410_e157833_d_n0, assign105410_e157833_d_n2, assign105410_e157833_d_n4, assign105410_e157833_d_n5, assign105410_e157833_d_n6, assign105410_e157833_d_n7, assign105410_e157833_d_n8, assign105410_e157833_d_n9, assign105410_e157833_d_n10, assign105410_e157833_d_n11, assign105410_e157833_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105410_e157833;
        locals.var_xp_dn0 = assign105410_e157833_d_n0;
        locals.var_xp_dn2 = assign105410_e157833_d_n2;
        locals.var_xp_dn4 = assign105410_e157833_d_n4;
        locals.var_xp_dn5 = assign105410_e157833_d_n5;
        locals.var_xp_dn6 = assign105410_e157833_d_n6;
        locals.var_xp_dn7 = assign105410_e157833_d_n7;
        locals.var_xp_dn8 = assign105410_e157833_d_n8;
        locals.var_xp_dn9 = assign105410_e157833_d_n9;
        locals.var_xp_dn10 = assign105410_e157833_d_n10;
        locals.var_xp_dn11 = assign105410_e157833_d_n11;
        locals.var_xp_dn14 = assign105410_e157833_d_n14;

        let (assign105420_e157842, assign105420_e157842_d_n0, assign105420_e157842_d_n2, assign105420_e157842_d_n4, assign105420_e157842_d_n5, assign105420_e157842_d_n6, assign105420_e157842_d_n7, assign105420_e157842_d_n8, assign105420_e157842_d_n9, assign105420_e157842_d_n10, assign105420_e157842_d_n11, assign105420_e157842_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105420_e157842;
        locals.var_xmp_dn0 = assign105420_e157842_d_n0;
        locals.var_xmp_dn2 = assign105420_e157842_d_n2;
        locals.var_xmp_dn4 = assign105420_e157842_d_n4;
        locals.var_xmp_dn5 = assign105420_e157842_d_n5;
        locals.var_xmp_dn6 = assign105420_e157842_d_n6;
        locals.var_xmp_dn7 = assign105420_e157842_d_n7;
        locals.var_xmp_dn8 = assign105420_e157842_d_n8;
        locals.var_xmp_dn9 = assign105420_e157842_d_n9;
        locals.var_xmp_dn10 = assign105420_e157842_d_n10;
        locals.var_xmp_dn11 = assign105420_e157842_d_n11;
        locals.var_xmp_dn14 = assign105420_e157842_d_n14;

        let (assign105430_e157851,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105430_e157851;

        let (assign105440_e157860,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105440_e157860;

        let (assign105450_e157869, assign105450_e157869_d_n0, assign105450_e157869_d_n2, assign105450_e157869_d_n4, assign105450_e157869_d_n5, assign105450_e157869_d_n6, assign105450_e157869_d_n7, assign105450_e157869_d_n8, assign105450_e157869_d_n9, assign105450_e157869_d_n10, assign105450_e157869_d_n11, assign105450_e157869_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign105450_e157869;
        locals.var_arg_dn0 = assign105450_e157869_d_n0;
        locals.var_arg_dn2 = assign105450_e157869_d_n2;
        locals.var_arg_dn4 = assign105450_e157869_d_n4;
        locals.var_arg_dn5 = assign105450_e157869_d_n5;
        locals.var_arg_dn6 = assign105450_e157869_d_n6;
        locals.var_arg_dn7 = assign105450_e157869_d_n7;
        locals.var_arg_dn8 = assign105450_e157869_d_n8;
        locals.var_arg_dn9 = assign105450_e157869_d_n9;
        locals.var_arg_dn10 = assign105450_e157869_d_n10;
        locals.var_arg_dn11 = assign105450_e157869_d_n11;
        locals.var_arg_dn14 = assign105450_e157869_d_n14;

        let (assign105460_e157878, assign105460_e157878_d_n0, assign105460_e157878_d_n2, assign105460_e157878_d_n4, assign105460_e157878_d_n5, assign105460_e157878_d_n6, assign105460_e157878_d_n7, assign105460_e157878_d_n8, assign105460_e157878_d_n9, assign105460_e157878_d_n10, assign105460_e157878_d_n11, assign105460_e157878_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105460_e157878;
        locals.var_dnm_dn0 = assign105460_e157878_d_n0;
        locals.var_dnm_dn2 = assign105460_e157878_d_n2;
        locals.var_dnm_dn4 = assign105460_e157878_d_n4;
        locals.var_dnm_dn5 = assign105460_e157878_d_n5;
        locals.var_dnm_dn6 = assign105460_e157878_d_n6;
        locals.var_dnm_dn7 = assign105460_e157878_d_n7;
        locals.var_dnm_dn8 = assign105460_e157878_d_n8;
        locals.var_dnm_dn9 = assign105460_e157878_d_n9;
        locals.var_dnm_dn10 = assign105460_e157878_d_n10;
        locals.var_dnm_dn11 = assign105460_e157878_d_n11;
        locals.var_dnm_dn14 = assign105460_e157878_d_n14;

    }

    pub(super) fn stamp_transient_block_385(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign105470_e157889, assign105470_e157889_d_n0, assign105470_e157889_d_n2, assign105470_e157889_d_n4, assign105470_e157889_d_n5, assign105470_e157889_d_n6, assign105470_e157889_d_n7, assign105470_e157889_d_n8, assign105470_e157889_d_n9, assign105470_e157889_d_n10, assign105470_e157889_d_n11, assign105470_e157889_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105470_e157887: f64 = (locals.var_xp * locals.var_x2);
        (assign105470_e157887, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105470_e157889;
        locals.var_xp_dn0 = assign105470_e157889_d_n0;
        locals.var_xp_dn2 = assign105470_e157889_d_n2;
        locals.var_xp_dn4 = assign105470_e157889_d_n4;
        locals.var_xp_dn5 = assign105470_e157889_d_n5;
        locals.var_xp_dn6 = assign105470_e157889_d_n6;
        locals.var_xp_dn7 = assign105470_e157889_d_n7;
        locals.var_xp_dn8 = assign105470_e157889_d_n8;
        locals.var_xp_dn9 = assign105470_e157889_d_n9;
        locals.var_xp_dn10 = assign105470_e157889_d_n10;
        locals.var_xp_dn11 = assign105470_e157889_d_n11;
        locals.var_xp_dn14 = assign105470_e157889_d_n14;

        let (assign105480_e157900, assign105480_e157900_d_n0, assign105480_e157900_d_n2, assign105480_e157900_d_n4, assign105480_e157900_d_n5, assign105480_e157900_d_n6, assign105480_e157900_d_n7, assign105480_e157900_d_n8, assign105480_e157900_d_n9, assign105480_e157900_d_n10, assign105480_e157900_d_n11, assign105480_e157900_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105480_e157898: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105480_e157898, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105480_e157900;
        locals.var_xmp_dn0 = assign105480_e157900_d_n0;
        locals.var_xmp_dn2 = assign105480_e157900_d_n2;
        locals.var_xmp_dn4 = assign105480_e157900_d_n4;
        locals.var_xmp_dn5 = assign105480_e157900_d_n5;
        locals.var_xmp_dn6 = assign105480_e157900_d_n6;
        locals.var_xmp_dn7 = assign105480_e157900_d_n7;
        locals.var_xmp_dn8 = assign105480_e157900_d_n8;
        locals.var_xmp_dn9 = assign105480_e157900_d_n9;
        locals.var_xmp_dn10 = assign105480_e157900_d_n10;
        locals.var_xmp_dn11 = assign105480_e157900_d_n11;
        locals.var_xmp_dn14 = assign105480_e157900_d_n14;

        let (assign105490_e157911, assign105490_e157911_d_n0, assign105490_e157911_d_n2, assign105490_e157911_d_n4, assign105490_e157911_d_n5, assign105490_e157911_d_n6, assign105490_e157911_d_n7, assign105490_e157911_d_n8, assign105490_e157911_d_n9, assign105490_e157911_d_n10, assign105490_e157911_d_n11, assign105490_e157911_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105490_e157909: f64 = (locals.var_xp * locals.var_x2);
        (assign105490_e157909, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105490_e157911;
        locals.var_xp_dn0 = assign105490_e157911_d_n0;
        locals.var_xp_dn2 = assign105490_e157911_d_n2;
        locals.var_xp_dn4 = assign105490_e157911_d_n4;
        locals.var_xp_dn5 = assign105490_e157911_d_n5;
        locals.var_xp_dn6 = assign105490_e157911_d_n6;
        locals.var_xp_dn7 = assign105490_e157911_d_n7;
        locals.var_xp_dn8 = assign105490_e157911_d_n8;
        locals.var_xp_dn9 = assign105490_e157911_d_n9;
        locals.var_xp_dn10 = assign105490_e157911_d_n10;
        locals.var_xp_dn11 = assign105490_e157911_d_n11;
        locals.var_xp_dn14 = assign105490_e157911_d_n14;

        let (assign105500_e157922, assign105500_e157922_d_n0, assign105500_e157922_d_n2, assign105500_e157922_d_n4, assign105500_e157922_d_n5, assign105500_e157922_d_n6, assign105500_e157922_d_n7, assign105500_e157922_d_n8, assign105500_e157922_d_n9, assign105500_e157922_d_n10, assign105500_e157922_d_n11, assign105500_e157922_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105500_e157920: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105500_e157920, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105500_e157922;
        locals.var_xmp_dn0 = assign105500_e157922_d_n0;
        locals.var_xmp_dn2 = assign105500_e157922_d_n2;
        locals.var_xmp_dn4 = assign105500_e157922_d_n4;
        locals.var_xmp_dn5 = assign105500_e157922_d_n5;
        locals.var_xmp_dn6 = assign105500_e157922_d_n6;
        locals.var_xmp_dn7 = assign105500_e157922_d_n7;
        locals.var_xmp_dn8 = assign105500_e157922_d_n8;
        locals.var_xmp_dn9 = assign105500_e157922_d_n9;
        locals.var_xmp_dn10 = assign105500_e157922_d_n10;
        locals.var_xmp_dn11 = assign105500_e157922_d_n11;
        locals.var_xmp_dn14 = assign105500_e157922_d_n14;

        let (assign105510_e157933, assign105510_e157933_d_n0, assign105510_e157933_d_n2, assign105510_e157933_d_n4, assign105510_e157933_d_n5, assign105510_e157933_d_n6, assign105510_e157933_d_n7, assign105510_e157933_d_n8, assign105510_e157933_d_n9, assign105510_e157933_d_n10, assign105510_e157933_d_n11, assign105510_e157933_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105510_e157931: f64 = (locals.var_xp + locals.var_xmp);
        (assign105510_e157931, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign105510_e157933;
        locals.var_arg_dn0 = assign105510_e157933_d_n0;
        locals.var_arg_dn2 = assign105510_e157933_d_n2;
        locals.var_arg_dn4 = assign105510_e157933_d_n4;
        locals.var_arg_dn5 = assign105510_e157933_d_n5;
        locals.var_arg_dn6 = assign105510_e157933_d_n6;
        locals.var_arg_dn7 = assign105510_e157933_d_n7;
        locals.var_arg_dn8 = assign105510_e157933_d_n8;
        locals.var_arg_dn9 = assign105510_e157933_d_n9;
        locals.var_arg_dn10 = assign105510_e157933_d_n10;
        locals.var_arg_dn11 = assign105510_e157933_d_n11;
        locals.var_arg_dn14 = assign105510_e157933_d_n14;

        let (assign105520_e157942, assign105520_e157942_d_n0, assign105520_e157942_d_n2, assign105520_e157942_d_n4, assign105520_e157942_d_n5, assign105520_e157942_d_n6, assign105520_e157942_d_n7, assign105520_e157942_d_n8, assign105520_e157942_d_n9, assign105520_e157942_d_n10, assign105520_e157942_d_n11, assign105520_e157942_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105520_e157942;
        locals.var_dnm_dn0 = assign105520_e157942_d_n0;
        locals.var_dnm_dn2 = assign105520_e157942_d_n2;
        locals.var_dnm_dn4 = assign105520_e157942_d_n4;
        locals.var_dnm_dn5 = assign105520_e157942_d_n5;
        locals.var_dnm_dn6 = assign105520_e157942_d_n6;
        locals.var_dnm_dn7 = assign105520_e157942_d_n7;
        locals.var_dnm_dn8 = assign105520_e157942_d_n8;
        locals.var_dnm_dn9 = assign105520_e157942_d_n9;
        locals.var_dnm_dn10 = assign105520_e157942_d_n10;
        locals.var_dnm_dn11 = assign105520_e157942_d_n11;
        locals.var_dnm_dn14 = assign105520_e157942_d_n14;

        let assign105530_e157957: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2392 = assign105530_e157957;

        let assign105540_e157960: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2393 = assign105540_e157960;

        let (assign105550_e157973,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) && (locals.var_guard2393 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105550_e157973;

        let assign105560_e157976: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2394 = assign105560_e157976;

        let (assign105570_e157992,) = {
    if ((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) && (locals.var_guard2393 == 0.0)) && (locals.var_guard2394 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105570_e157992;

        let assign105580_e157995: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2395 = assign105580_e157995;

        let (assign105590_e158014,) = {
    if (((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) && (locals.var_guard2393 == 0.0)) && (locals.var_guard2394 == 0.0)) && (locals.var_guard2395 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105590_e158014;

        let assign105600_e158017: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2396 = assign105600_e158017;

        let (assign105610_e158039,) = {
    if ((((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) && (locals.var_guard2393 == 0.0)) && (locals.var_guard2394 == 0.0)) && (locals.var_guard2395 == 0.0)) && (locals.var_guard2396 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105610_e158039;

        let (assign105620_e158050,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105620_e158050;

        let mut assign105630_loop_guard: usize = 0;
        while {
            let assign105630_cond_e158062: f64 = if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105630_cond_e158062 != 0.0
        } {
            assign105630_loop_guard += 1;
            assert!(assign105630_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105630_body0_e158074, assign105630_body0_e158074_d_n0, assign105630_body0_e158074_d_n2, assign105630_body0_e158074_d_n4, assign105630_body0_e158074_d_n5, assign105630_body0_e158074_d_n6, assign105630_body0_e158074_d_n7, assign105630_body0_e158074_d_n8, assign105630_body0_e158074_d_n9, assign105630_body0_e158074_d_n10, assign105630_body0_e158074_d_n11, assign105630_body0_e158074_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) {
        let assign105630_body0_e158072: f64 = (locals.var_dnm).sqrt();
        (assign105630_body0_e158072, (locals.var_dnm_dn0 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn2 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn4 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn5 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn6 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn7 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn8 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn9 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn10 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn11 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn14 / (2.0 * assign105630_body0_e158072)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign105630_body0_e158074;
            locals.var_dnm_dn0 = assign105630_body0_e158074_d_n0;
            locals.var_dnm_dn2 = assign105630_body0_e158074_d_n2;
            locals.var_dnm_dn4 = assign105630_body0_e158074_d_n4;
            locals.var_dnm_dn5 = assign105630_body0_e158074_d_n5;
            locals.var_dnm_dn6 = assign105630_body0_e158074_d_n6;
            locals.var_dnm_dn7 = assign105630_body0_e158074_d_n7;
            locals.var_dnm_dn8 = assign105630_body0_e158074_d_n8;
            locals.var_dnm_dn9 = assign105630_body0_e158074_d_n9;
            locals.var_dnm_dn10 = assign105630_body0_e158074_d_n10;
            locals.var_dnm_dn11 = assign105630_body0_e158074_d_n11;
            locals.var_dnm_dn14 = assign105630_body0_e158074_d_n14;
            let (assign105630_body1_e158087,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) {
        let assign105630_body1_e158085: f64 = (locals.var_m0 + 1.0);
        (assign105630_body1_e158085,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign105630_body1_e158087;
        }

        let (assign105640_e158110, assign105640_e158110_d_n0, assign105640_e158110_d_n2, assign105640_e158110_d_n4, assign105640_e158110_d_n5, assign105640_e158110_d_n6, assign105640_e158110_d_n7, assign105640_e158110_d_n8, assign105640_e158110_d_n9, assign105640_e158110_d_n10, assign105640_e158110_d_n11, assign105640_e158110_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 == 0.0)) {
        let (assign105640_e158108, assign105640_e158108_d_n0, assign105640_e158108_d_n2, assign105640_e158108_d_n4, assign105640_e158108_d_n5, assign105640_e158108_d_n6, assign105640_e158108_d_n7, assign105640_e158108_d_n8, assign105640_e158108_d_n9, assign105640_e158108_d_n10, assign105640_e158108_d_n11, assign105640_e158108_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105640_e158105: f64 = (2.0 * 2.0);
                let assign105640_e158106: f64 = (1.0 / assign105640_e158105);
                let assign105640_e158107: f64 = (locals.var_dnm).powf(assign105640_e158106);
                (assign105640_e158107, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn11)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn14)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign105640_e158108, assign105640_e158108_d_n0, assign105640_e158108_d_n2, assign105640_e158108_d_n4, assign105640_e158108_d_n5, assign105640_e158108_d_n6, assign105640_e158108_d_n7, assign105640_e158108_d_n8, assign105640_e158108_d_n9, assign105640_e158108_d_n10, assign105640_e158108_d_n11, assign105640_e158108_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105640_e158110;
        locals.var_dnm_dn0 = assign105640_e158110_d_n0;
        locals.var_dnm_dn2 = assign105640_e158110_d_n2;
        locals.var_dnm_dn4 = assign105640_e158110_d_n4;
        locals.var_dnm_dn5 = assign105640_e158110_d_n5;
        locals.var_dnm_dn6 = assign105640_e158110_d_n6;
        locals.var_dnm_dn7 = assign105640_e158110_d_n7;
        locals.var_dnm_dn8 = assign105640_e158110_d_n8;
        locals.var_dnm_dn9 = assign105640_e158110_d_n9;
        locals.var_dnm_dn10 = assign105640_e158110_d_n10;
        locals.var_dnm_dn11 = assign105640_e158110_d_n11;
        locals.var_dnm_dn14 = assign105640_e158110_d_n14;

        let (assign105650_e158121, assign105650_e158121_d_n0, assign105650_e158121_d_n2, assign105650_e158121_d_n4, assign105650_e158121_d_n5, assign105650_e158121_d_n6, assign105650_e158121_d_n7, assign105650_e158121_d_n8, assign105650_e158121_d_n9, assign105650_e158121_d_n10, assign105650_e158121_d_n11, assign105650_e158121_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105650_e158119: f64 = (1.0 / locals.var_dnm);
        (assign105650_e158119, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105650_e158121;
        locals.var_dnm_dn0 = assign105650_e158121_d_n0;
        locals.var_dnm_dn2 = assign105650_e158121_d_n2;
        locals.var_dnm_dn4 = assign105650_e158121_d_n4;
        locals.var_dnm_dn5 = assign105650_e158121_d_n5;
        locals.var_dnm_dn6 = assign105650_e158121_d_n6;
        locals.var_dnm_dn7 = assign105650_e158121_d_n7;
        locals.var_dnm_dn8 = assign105650_e158121_d_n8;
        locals.var_dnm_dn9 = assign105650_e158121_d_n9;
        locals.var_dnm_dn10 = assign105650_e158121_d_n10;
        locals.var_dnm_dn11 = assign105650_e158121_d_n11;
        locals.var_dnm_dn14 = assign105650_e158121_d_n14;

        let (assign105660_e158134, assign105660_e158134_d_n0, assign105660_e158134_d_n2, assign105660_e158134_d_n4, assign105660_e158134_d_n5, assign105660_e158134_d_n6, assign105660_e158134_d_n7, assign105660_e158134_d_n8, assign105660_e158134_d_n9, assign105660_e158134_d_n10, assign105660_e158134_d_n11, assign105660_e158134_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105660_e158130: f64 = (locals.var_tmf1 * 1000.0);
        let assign105660_e158132: f64 = (assign105660_e158130 * locals.var_dnm);
        (assign105660_e158132, (((locals.var_tmf1_dn0 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign105660_e158134;
        locals.var_tmf0_dn0 = assign105660_e158134_d_n0;
        locals.var_tmf0_dn2 = assign105660_e158134_d_n2;
        locals.var_tmf0_dn4 = assign105660_e158134_d_n4;
        locals.var_tmf0_dn5 = assign105660_e158134_d_n5;
        locals.var_tmf0_dn6 = assign105660_e158134_d_n6;
        locals.var_tmf0_dn7 = assign105660_e158134_d_n7;
        locals.var_tmf0_dn8 = assign105660_e158134_d_n8;
        locals.var_tmf0_dn9 = assign105660_e158134_d_n9;
        locals.var_tmf0_dn10 = assign105660_e158134_d_n10;
        locals.var_tmf0_dn11 = assign105660_e158134_d_n11;
        locals.var_tmf0_dn14 = assign105660_e158134_d_n14;

        let (assign105670_e158149, assign105670_e158149_d_n0, assign105670_e158149_d_n2, assign105670_e158149_d_n4, assign105670_e158149_d_n5, assign105670_e158149_d_n6, assign105670_e158149_d_n7, assign105670_e158149_d_n8, assign105670_e158149_d_n9, assign105670_e158149_d_n10, assign105670_e158149_d_n11, assign105670_e158149_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105670_e158143: f64 = (1000.0 * locals.var_xmp);
        let assign105670_e158145: f64 = (assign105670_e158143 * locals.var_dnm);
        let assign105670_e158147: f64 = (assign105670_e158145 / locals.var_arg);
        (assign105670_e158147, ((((((1000.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn0)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn2)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn4)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn5)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn6)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn7)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn8)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn9)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn10)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn11) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn11)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn14) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn14)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105670_e158149;
        locals.var_t0_dn0 = assign105670_e158149_d_n0;
        locals.var_t0_dn2 = assign105670_e158149_d_n2;
        locals.var_t0_dn4 = assign105670_e158149_d_n4;
        locals.var_t0_dn5 = assign105670_e158149_d_n5;
        locals.var_t0_dn6 = assign105670_e158149_d_n6;
        locals.var_t0_dn7 = assign105670_e158149_d_n7;
        locals.var_t0_dn8 = assign105670_e158149_d_n8;
        locals.var_t0_dn9 = assign105670_e158149_d_n9;
        locals.var_t0_dn10 = assign105670_e158149_d_n10;
        locals.var_t0_dn11 = assign105670_e158149_d_n11;
        locals.var_t0_dn14 = assign105670_e158149_d_n14;

        let (assign105680_e158162, assign105680_e158162_d_n0, assign105680_e158162_d_n2, assign105680_e158162_d_n4, assign105680_e158162_d_n5, assign105680_e158162_d_n6, assign105680_e158162_d_n7, assign105680_e158162_d_n8, assign105680_e158162_d_n9, assign105680_e158162_d_n10, assign105680_e158162_d_n11, assign105680_e158162_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105680_e158158: f64 = (1000000.0 - 1000.0);
        let assign105680_e158160: f64 = (assign105680_e158158 + locals.var_tmf0);
        (assign105680_e158160, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105680_e158162;
        locals.var_rdd_dn0 = assign105680_e158162_d_n0;
        locals.var_rdd_dn2 = assign105680_e158162_d_n2;
        locals.var_rdd_dn4 = assign105680_e158162_d_n4;
        locals.var_rdd_dn5 = assign105680_e158162_d_n5;
        locals.var_rdd_dn6 = assign105680_e158162_d_n6;
        locals.var_rdd_dn7 = assign105680_e158162_d_n7;
        locals.var_rdd_dn8 = assign105680_e158162_d_n8;
        locals.var_rdd_dn9 = assign105680_e158162_d_n9;
        locals.var_rdd_dn10 = assign105680_e158162_d_n10;
        locals.var_rdd_dn11 = assign105680_e158162_d_n11;
        locals.var_rdd_dn14 = assign105680_e158162_d_n14;

        let (assign105690_e158171, assign105690_e158171_d_n0, assign105690_e158171_d_n2, assign105690_e158171_d_n4, assign105690_e158171_d_n5, assign105690_e158171_d_n6, assign105690_e158171_d_n7, assign105690_e158171_d_n8, assign105690_e158171_d_n9, assign105690_e158171_d_n10, assign105690_e158171_d_n11, assign105690_e158171_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105690_e158171;
        locals.var_t0_dn0 = assign105690_e158171_d_n0;
        locals.var_t0_dn2 = assign105690_e158171_d_n2;
        locals.var_t0_dn4 = assign105690_e158171_d_n4;
        locals.var_t0_dn5 = assign105690_e158171_d_n5;
        locals.var_t0_dn6 = assign105690_e158171_d_n6;
        locals.var_t0_dn7 = assign105690_e158171_d_n7;
        locals.var_t0_dn8 = assign105690_e158171_d_n8;
        locals.var_t0_dn9 = assign105690_e158171_d_n9;
        locals.var_t0_dn10 = assign105690_e158171_d_n10;
        locals.var_t0_dn11 = assign105690_e158171_d_n11;
        locals.var_t0_dn14 = assign105690_e158171_d_n14;

        let (assign105700_e158181, assign105700_e158181_d_n0, assign105700_e158181_d_n2, assign105700_e158181_d_n4, assign105700_e158181_d_n5, assign105700_e158181_d_n6, assign105700_e158181_d_n7, assign105700_e158181_d_n8, assign105700_e158181_d_n9, assign105700_e158181_d_n10, assign105700_e158181_d_n11, assign105700_e158181_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 == 0.0)) {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105700_e158181;
        locals.var_rdd_dn0 = assign105700_e158181_d_n0;
        locals.var_rdd_dn2 = assign105700_e158181_d_n2;
        locals.var_rdd_dn4 = assign105700_e158181_d_n4;
        locals.var_rdd_dn5 = assign105700_e158181_d_n5;
        locals.var_rdd_dn6 = assign105700_e158181_d_n6;
        locals.var_rdd_dn7 = assign105700_e158181_d_n7;
        locals.var_rdd_dn8 = assign105700_e158181_d_n8;
        locals.var_rdd_dn9 = assign105700_e158181_d_n9;
        locals.var_rdd_dn10 = assign105700_e158181_d_n10;
        locals.var_rdd_dn11 = assign105700_e158181_d_n11;
        locals.var_rdd_dn14 = assign105700_e158181_d_n14;

        let (assign105710_e158191, assign105710_e158191_d_n0, assign105710_e158191_d_n2, assign105710_e158191_d_n4, assign105710_e158191_d_n5, assign105710_e158191_d_n6, assign105710_e158191_d_n7, assign105710_e158191_d_n8, assign105710_e158191_d_n9, assign105710_e158191_d_n10, assign105710_e158191_d_n11, assign105710_e158191_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105710_e158191;
        locals.var_t0_dn0 = assign105710_e158191_d_n0;
        locals.var_t0_dn2 = assign105710_e158191_d_n2;
        locals.var_t0_dn4 = assign105710_e158191_d_n4;
        locals.var_t0_dn5 = assign105710_e158191_d_n5;
        locals.var_t0_dn6 = assign105710_e158191_d_n6;
        locals.var_t0_dn7 = assign105710_e158191_d_n7;
        locals.var_t0_dn8 = assign105710_e158191_d_n8;
        locals.var_t0_dn9 = assign105710_e158191_d_n9;
        locals.var_t0_dn10 = assign105710_e158191_d_n10;
        locals.var_t0_dn11 = assign105710_e158191_d_n11;
        locals.var_t0_dn14 = assign105710_e158191_d_n14;

        let assign105720_e158198: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign105720_e158199: f64 = (locals.var_uc_nover * assign105720_e158198);
        let assign105720_e158202: f64 = if ((p.p54 == 1.0) && (assign105720_e158199 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2397 = assign105720_e158202;

        let (assign105730_e158213, assign105730_e158213_d_n0, assign105730_e158213_d_n2, assign105730_e158213_d_n4, assign105730_e158213_d_n5, assign105730_e158213_d_n6, assign105730_e158213_d_n7, assign105730_e158213_d_n8, assign105730_e158213_d_n9, assign105730_e158213_d_n10, assign105730_e158213_d_n11, assign105730_e158213_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2397 != 0.0)) {
        let assign105730_e158211: f64 = (p.p334 - locals.var_wdep);
        (assign105730_e158211, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn11), (-locals.var_wdep_dn14),)
    } else {
        (locals.var_ddriftld, locals.var_ddriftld_dn0, locals.var_ddriftld_dn2, locals.var_ddriftld_dn4, locals.var_ddriftld_dn5, locals.var_ddriftld_dn6, locals.var_ddriftld_dn7, locals.var_ddriftld_dn8, locals.var_ddriftld_dn9, locals.var_ddriftld_dn10, locals.var_ddriftld_dn11, locals.var_ddriftld_dn14,)
    }
};
        locals.var_ddriftld = assign105730_e158213;
        locals.var_ddriftld_dn0 = assign105730_e158213_d_n0;
        locals.var_ddriftld_dn2 = assign105730_e158213_d_n2;
        locals.var_ddriftld_dn4 = assign105730_e158213_d_n4;
        locals.var_ddriftld_dn5 = assign105730_e158213_d_n5;
        locals.var_ddriftld_dn6 = assign105730_e158213_d_n6;
        locals.var_ddriftld_dn7 = assign105730_e158213_d_n7;
        locals.var_ddriftld_dn8 = assign105730_e158213_d_n8;
        locals.var_ddriftld_dn9 = assign105730_e158213_d_n9;
        locals.var_ddriftld_dn10 = assign105730_e158213_d_n10;
        locals.var_ddriftld_dn11 = assign105730_e158213_d_n11;
        locals.var_ddriftld_dn14 = assign105730_e158213_d_n14;

        let (assign105740_e158226, assign105740_e158226_d_n0, assign105740_e158226_d_n2, assign105740_e158226_d_n4, assign105740_e158226_d_n5, assign105740_e158226_d_n6, assign105740_e158226_d_n7, assign105740_e158226_d_n8, assign105740_e158226_d_n9, assign105740_e158226_d_n10, assign105740_e158226_d_n11, assign105740_e158226_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2397 != 0.0)) {
        let assign105740_e158222: f64 = (locals.var_rdd * locals.var_ldrift0);
        let assign105740_e158224: f64 = (assign105740_e158222 / locals.var_ddriftld);
        (assign105740_e158224, ((((locals.var_rdd_dn0 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn0)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn2 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn2)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn4 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn4)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn5 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn5)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn6 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn6)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn7 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn7)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn8 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn8)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn9 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn9)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn10 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn10)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn11 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn11)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn14 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn14)) / (locals.var_ddriftld * locals.var_ddriftld)),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105740_e158226;
        locals.var_rdd_dn0 = assign105740_e158226_d_n0;
        locals.var_rdd_dn2 = assign105740_e158226_d_n2;
        locals.var_rdd_dn4 = assign105740_e158226_d_n4;
        locals.var_rdd_dn5 = assign105740_e158226_d_n5;
        locals.var_rdd_dn6 = assign105740_e158226_d_n6;
        locals.var_rdd_dn7 = assign105740_e158226_d_n7;
        locals.var_rdd_dn8 = assign105740_e158226_d_n8;
        locals.var_rdd_dn9 = assign105740_e158226_d_n9;
        locals.var_rdd_dn10 = assign105740_e158226_d_n10;
        locals.var_rdd_dn11 = assign105740_e158226_d_n11;
        locals.var_rdd_dn14 = assign105740_e158226_d_n14;

        let (assign105750_e158235, assign105750_e158235_d_n0, assign105750_e158235_d_n2, assign105750_e158235_d_n4, assign105750_e158235_d_n5, assign105750_e158235_d_n6, assign105750_e158235_d_n7, assign105750_e158235_d_n8, assign105750_e158235_d_n9, assign105750_e158235_d_n10, assign105750_e158235_d_n11, assign105750_e158235_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign105750_e158233: f64 = (locals.var_rdd + locals.var_rd0);
        (assign105750_e158233, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105750_e158235;
        locals.var_rdd_dn0 = assign105750_e158235_d_n0;
        locals.var_rdd_dn2 = assign105750_e158235_d_n2;
        locals.var_rdd_dn4 = assign105750_e158235_d_n4;
        locals.var_rdd_dn5 = assign105750_e158235_d_n5;
        locals.var_rdd_dn6 = assign105750_e158235_d_n6;
        locals.var_rdd_dn7 = assign105750_e158235_d_n7;
        locals.var_rdd_dn8 = assign105750_e158235_d_n8;
        locals.var_rdd_dn9 = assign105750_e158235_d_n9;
        locals.var_rdd_dn10 = assign105750_e158235_d_n10;
        locals.var_rdd_dn11 = assign105750_e158235_d_n11;
        locals.var_rdd_dn14 = assign105750_e158235_d_n14;

        let assign105790_e158266: f64 = if locals.var_rdd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2399 = assign105790_e158266;

        let (assign105800_e158275, assign105800_e158275_d_n0, assign105800_e158275_d_n2, assign105800_e158275_d_n4, assign105800_e158275_d_n5, assign105800_e158275_d_n6, assign105800_e158275_d_n7, assign105800_e158275_d_n8, assign105800_e158275_d_n9, assign105800_e158275_d_n10, assign105800_e158275_d_n11, assign105800_e158275_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2399 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105800_e158275;
        locals.var_rdd_dn0 = assign105800_e158275_d_n0;
        locals.var_rdd_dn2 = assign105800_e158275_d_n2;
        locals.var_rdd_dn4 = assign105800_e158275_d_n4;
        locals.var_rdd_dn5 = assign105800_e158275_d_n5;
        locals.var_rdd_dn6 = assign105800_e158275_d_n6;
        locals.var_rdd_dn7 = assign105800_e158275_d_n7;
        locals.var_rdd_dn8 = assign105800_e158275_d_n8;
        locals.var_rdd_dn9 = assign105800_e158275_d_n9;
        locals.var_rdd_dn10 = assign105800_e158275_d_n10;
        locals.var_rdd_dn11 = assign105800_e158275_d_n11;
        locals.var_rdd_dn14 = assign105800_e158275_d_n14;

    }

    pub(super) fn stamp_transient_block_386(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign105810_e158284, assign105810_e158284_d_n0, assign105810_e158284_d_n2, assign105810_e158284_d_n4, assign105810_e158284_d_n5, assign105810_e158284_d_n6, assign105810_e158284_d_n7, assign105810_e158284_d_n8, assign105810_e158284_d_n9, assign105810_e158284_d_n10, assign105810_e158284_d_n11, assign105810_e158284_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign105810_e158282: f64 = (locals.var_rdd / locals.var_mfactor);
        (assign105810_e158282, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn11 / locals.var_mfactor), (locals.var_rdd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn11, locals.var_rdde_dn14,)
    }
};
        locals.var_rdde = assign105810_e158284;
        locals.var_rdde_dn0 = assign105810_e158284_d_n0;
        locals.var_rdde_dn2 = assign105810_e158284_d_n2;
        locals.var_rdde_dn4 = assign105810_e158284_d_n4;
        locals.var_rdde_dn5 = assign105810_e158284_d_n5;
        locals.var_rdde_dn6 = assign105810_e158284_d_n6;
        locals.var_rdde_dn7 = assign105810_e158284_d_n7;
        locals.var_rdde_dn8 = assign105810_e158284_d_n8;
        locals.var_rdde_dn9 = assign105810_e158284_d_n9;
        locals.var_rdde_dn10 = assign105810_e158284_d_n10;
        locals.var_rdde_dn11 = assign105810_e158284_d_n11;
        locals.var_rdde_dn14 = assign105810_e158284_d_n14;

        let assign105820_e158287: f64 = if locals.var_rdd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2400 = assign105820_e158287;

        let (assign105830_e158294, assign105830_e158294_d_n0, assign105830_e158294_d_n2, assign105830_e158294_d_n4, assign105830_e158294_d_n5, assign105830_e158294_d_n6, assign105830_e158294_d_n7, assign105830_e158294_d_n8, assign105830_e158294_d_n9, assign105830_e158294_d_n10, assign105830_e158294_d_n11, assign105830_e158294_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2400 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105830_e158294;
        locals.var_rdd_dn0 = assign105830_e158294_d_n0;
        locals.var_rdd_dn2 = assign105830_e158294_d_n2;
        locals.var_rdd_dn4 = assign105830_e158294_d_n4;
        locals.var_rdd_dn5 = assign105830_e158294_d_n5;
        locals.var_rdd_dn6 = assign105830_e158294_d_n6;
        locals.var_rdd_dn7 = assign105830_e158294_d_n7;
        locals.var_rdd_dn8 = assign105830_e158294_d_n8;
        locals.var_rdd_dn9 = assign105830_e158294_d_n9;
        locals.var_rdd_dn10 = assign105830_e158294_d_n10;
        locals.var_rdd_dn11 = assign105830_e158294_d_n11;
        locals.var_rdd_dn14 = assign105830_e158294_d_n14;

        let assign105840_e158297: f64 = if locals.var_rsd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2401 = assign105840_e158297;

        let (assign105850_e158304, assign105850_e158304_d_n0, assign105850_e158304_d_n2, assign105850_e158304_d_n4, assign105850_e158304_d_n5, assign105850_e158304_d_n6, assign105850_e158304_d_n7, assign105850_e158304_d_n8, assign105850_e158304_d_n9, assign105850_e158304_d_n10, assign105850_e158304_d_n11, assign105850_e158304_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2401 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign105850_e158304;
        locals.var_rsd_dn0 = assign105850_e158304_d_n0;
        locals.var_rsd_dn2 = assign105850_e158304_d_n2;
        locals.var_rsd_dn4 = assign105850_e158304_d_n4;
        locals.var_rsd_dn5 = assign105850_e158304_d_n5;
        locals.var_rsd_dn6 = assign105850_e158304_d_n6;
        locals.var_rsd_dn7 = assign105850_e158304_d_n7;
        locals.var_rsd_dn8 = assign105850_e158304_d_n8;
        locals.var_rsd_dn9 = assign105850_e158304_d_n9;
        locals.var_rsd_dn10 = assign105850_e158304_d_n10;
        locals.var_rsd_dn11 = assign105850_e158304_d_n11;
        locals.var_rsd_dn14 = assign105850_e158304_d_n14;

        let assign105860_e158307: f64 = if locals.var_vdsemodenml > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2402 = assign105860_e158307;

        let (assign105870_e158316, assign105870_e158316_d_n0, assign105870_e158316_d_n2, assign105870_e158316_d_n4, assign105870_e158316_d_n5, assign105870_e158316_d_n6, assign105870_e158316_d_n7, assign105870_e158316_d_n8, assign105870_e158316_d_n9, assign105870_e158316_d_n10, assign105870_e158316_d_n11, assign105870_e158316_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2402 != 0.0)) {
        let assign105870_e158314: f64 = (locals.var_rdd / locals.var_mfactor);
        (assign105870_e158314, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn11 / locals.var_mfactor), (locals.var_rdd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn11, locals.var_rdde_dn14,)
    }
};
        locals.var_rdde = assign105870_e158316;
        locals.var_rdde_dn0 = assign105870_e158316_d_n0;
        locals.var_rdde_dn2 = assign105870_e158316_d_n2;
        locals.var_rdde_dn4 = assign105870_e158316_d_n4;
        locals.var_rdde_dn5 = assign105870_e158316_d_n5;
        locals.var_rdde_dn6 = assign105870_e158316_d_n6;
        locals.var_rdde_dn7 = assign105870_e158316_d_n7;
        locals.var_rdde_dn8 = assign105870_e158316_d_n8;
        locals.var_rdde_dn9 = assign105870_e158316_d_n9;
        locals.var_rdde_dn10 = assign105870_e158316_d_n10;
        locals.var_rdde_dn11 = assign105870_e158316_d_n11;
        locals.var_rdde_dn14 = assign105870_e158316_d_n14;

        let (assign105880_e158325, assign105880_e158325_d_n0, assign105880_e158325_d_n2, assign105880_e158325_d_n4, assign105880_e158325_d_n5, assign105880_e158325_d_n6, assign105880_e158325_d_n7, assign105880_e158325_d_n8, assign105880_e158325_d_n9, assign105880_e158325_d_n10, assign105880_e158325_d_n11, assign105880_e158325_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2402 != 0.0)) {
        let assign105880_e158323: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign105880_e158323, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn11 / locals.var_mfactor), (locals.var_rsd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn11, locals.var_rsde_dn14,)
    }
};
        locals.var_rsde = assign105880_e158325;
        locals.var_rsde_dn0 = assign105880_e158325_d_n0;
        locals.var_rsde_dn2 = assign105880_e158325_d_n2;
        locals.var_rsde_dn4 = assign105880_e158325_d_n4;
        locals.var_rsde_dn5 = assign105880_e158325_d_n5;
        locals.var_rsde_dn6 = assign105880_e158325_d_n6;
        locals.var_rsde_dn7 = assign105880_e158325_d_n7;
        locals.var_rsde_dn8 = assign105880_e158325_d_n8;
        locals.var_rsde_dn9 = assign105880_e158325_d_n9;
        locals.var_rsde_dn10 = assign105880_e158325_d_n10;
        locals.var_rsde_dn11 = assign105880_e158325_d_n11;
        locals.var_rsde_dn14 = assign105880_e158325_d_n14;

        let (assign105890_e158335, assign105890_e158335_d_n0, assign105890_e158335_d_n2, assign105890_e158335_d_n4, assign105890_e158335_d_n5, assign105890_e158335_d_n6, assign105890_e158335_d_n7, assign105890_e158335_d_n8, assign105890_e158335_d_n9, assign105890_e158335_d_n10, assign105890_e158335_d_n11, assign105890_e158335_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2402 == 0.0)) {
        let assign105890_e158333: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign105890_e158333, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn11 / locals.var_mfactor), (locals.var_rsd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn11, locals.var_rdde_dn14,)
    }
};
        locals.var_rdde = assign105890_e158335;
        locals.var_rdde_dn0 = assign105890_e158335_d_n0;
        locals.var_rdde_dn2 = assign105890_e158335_d_n2;
        locals.var_rdde_dn4 = assign105890_e158335_d_n4;
        locals.var_rdde_dn5 = assign105890_e158335_d_n5;
        locals.var_rdde_dn6 = assign105890_e158335_d_n6;
        locals.var_rdde_dn7 = assign105890_e158335_d_n7;
        locals.var_rdde_dn8 = assign105890_e158335_d_n8;
        locals.var_rdde_dn9 = assign105890_e158335_d_n9;
        locals.var_rdde_dn10 = assign105890_e158335_d_n10;
        locals.var_rdde_dn11 = assign105890_e158335_d_n11;
        locals.var_rdde_dn14 = assign105890_e158335_d_n14;

        let (assign105900_e158345, assign105900_e158345_d_n0, assign105900_e158345_d_n2, assign105900_e158345_d_n4, assign105900_e158345_d_n5, assign105900_e158345_d_n6, assign105900_e158345_d_n7, assign105900_e158345_d_n8, assign105900_e158345_d_n9, assign105900_e158345_d_n10, assign105900_e158345_d_n11, assign105900_e158345_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2402 == 0.0)) {
        let assign105900_e158343: f64 = (locals.var_rdd / locals.var_mfactor);
        (assign105900_e158343, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn11 / locals.var_mfactor), (locals.var_rdd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn11, locals.var_rsde_dn14,)
    }
};
        locals.var_rsde = assign105900_e158345;
        locals.var_rsde_dn0 = assign105900_e158345_d_n0;
        locals.var_rsde_dn2 = assign105900_e158345_d_n2;
        locals.var_rsde_dn4 = assign105900_e158345_d_n4;
        locals.var_rsde_dn5 = assign105900_e158345_d_n5;
        locals.var_rsde_dn6 = assign105900_e158345_d_n6;
        locals.var_rsde_dn7 = assign105900_e158345_d_n7;
        locals.var_rsde_dn8 = assign105900_e158345_d_n8;
        locals.var_rsde_dn9 = assign105900_e158345_d_n9;
        locals.var_rsde_dn10 = assign105900_e158345_d_n10;
        locals.var_rsde_dn11 = assign105900_e158345_d_n11;
        locals.var_rsde_dn14 = assign105900_e158345_d_n14;

        locals.var_rdd = locals.var_rdde;
        locals.var_rdd_dn0 = locals.var_rdde_dn0;
        locals.var_rdd_dn2 = locals.var_rdde_dn2;
        locals.var_rdd_dn4 = locals.var_rdde_dn4;
        locals.var_rdd_dn5 = locals.var_rdde_dn5;
        locals.var_rdd_dn6 = locals.var_rdde_dn6;
        locals.var_rdd_dn7 = locals.var_rdde_dn7;
        locals.var_rdd_dn8 = locals.var_rdde_dn8;
        locals.var_rdd_dn9 = locals.var_rdde_dn9;
        locals.var_rdd_dn10 = locals.var_rdde_dn10;
        locals.var_rdd_dn11 = locals.var_rdde_dn11;
        locals.var_rdd_dn14 = locals.var_rdde_dn14;

        locals.var_rsd = locals.var_rsde;
        locals.var_rsd_dn0 = locals.var_rsde_dn0;
        locals.var_rsd_dn2 = locals.var_rsde_dn2;
        locals.var_rsd_dn4 = locals.var_rsde_dn4;
        locals.var_rsd_dn5 = locals.var_rsde_dn5;
        locals.var_rsd_dn6 = locals.var_rsde_dn6;
        locals.var_rsd_dn7 = locals.var_rsde_dn7;
        locals.var_rsd_dn8 = locals.var_rsde_dn8;
        locals.var_rsd_dn9 = locals.var_rsde_dn9;
        locals.var_rsd_dn10 = locals.var_rsde_dn10;
        locals.var_rsd_dn11 = locals.var_rsde_dn11;
        locals.var_rsd_dn14 = locals.var_rsde_dn14;

        let assign105960_e158353: f64 = if locals.var_mode > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2403 = assign105960_e158353;

        let (assign105970_e158357, assign105970_e158357_d_n0, assign105970_e158357_d_n2, assign105970_e158357_d_n4, assign105970_e158357_d_n5, assign105970_e158357_d_n6, assign105970_e158357_d_n7, assign105970_e158357_d_n8, assign105970_e158357_d_n9, assign105970_e158357_d_n10, assign105970_e158357_d_n11, assign105970_e158357_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn8, locals.var_idse_dn9, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn14,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign105970_e158357;
        locals.var_ids_dn0 = assign105970_e158357_d_n0;
        locals.var_ids_dn2 = assign105970_e158357_d_n2;
        locals.var_ids_dn4 = assign105970_e158357_d_n4;
        locals.var_ids_dn5 = assign105970_e158357_d_n5;
        locals.var_ids_dn6 = assign105970_e158357_d_n6;
        locals.var_ids_dn7 = assign105970_e158357_d_n7;
        locals.var_ids_dn8 = assign105970_e158357_d_n8;
        locals.var_ids_dn9 = assign105970_e158357_d_n9;
        locals.var_ids_dn10 = assign105970_e158357_d_n10;
        locals.var_ids_dn11 = assign105970_e158357_d_n11;
        locals.var_ids_dn14 = assign105970_e158357_d_n14;

        let (assign105980_e158361, assign105980_e158361_d_n0, assign105980_e158361_d_n2, assign105980_e158361_d_n4, assign105980_e158361_d_n5, assign105980_e158361_d_n6, assign105980_e158361_d_n7, assign105980_e158361_d_n8, assign105980_e158361_d_n9, assign105980_e158361_d_n10, assign105980_e158361_d_n11, assign105980_e158361_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn14,)
    }
};
        locals.var_qd = assign105980_e158361;
        locals.var_qd_dn0 = assign105980_e158361_d_n0;
        locals.var_qd_dn2 = assign105980_e158361_d_n2;
        locals.var_qd_dn4 = assign105980_e158361_d_n4;
        locals.var_qd_dn5 = assign105980_e158361_d_n5;
        locals.var_qd_dn6 = assign105980_e158361_d_n6;
        locals.var_qd_dn7 = assign105980_e158361_d_n7;
        locals.var_qd_dn8 = assign105980_e158361_d_n8;
        locals.var_qd_dn9 = assign105980_e158361_d_n9;
        locals.var_qd_dn10 = assign105980_e158361_d_n10;
        locals.var_qd_dn11 = assign105980_e158361_d_n11;
        locals.var_qd_dn14 = assign105980_e158361_d_n14;

        let (assign105990_e158365, assign105990_e158365_d_n0, assign105990_e158365_d_n2, assign105990_e158365_d_n4, assign105990_e158365_d_n5, assign105990_e158365_d_n6, assign105990_e158365_d_n7, assign105990_e158365_d_n8, assign105990_e158365_d_n9, assign105990_e158365_d_n10, assign105990_e158365_d_n11, assign105990_e158365_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn14,)
    }
};
        locals.var_qg = assign105990_e158365;
        locals.var_qg_dn0 = assign105990_e158365_d_n0;
        locals.var_qg_dn2 = assign105990_e158365_d_n2;
        locals.var_qg_dn4 = assign105990_e158365_d_n4;
        locals.var_qg_dn5 = assign105990_e158365_d_n5;
        locals.var_qg_dn6 = assign105990_e158365_d_n6;
        locals.var_qg_dn7 = assign105990_e158365_d_n7;
        locals.var_qg_dn8 = assign105990_e158365_d_n8;
        locals.var_qg_dn9 = assign105990_e158365_d_n9;
        locals.var_qg_dn10 = assign105990_e158365_d_n10;
        locals.var_qg_dn11 = assign105990_e158365_d_n11;
        locals.var_qg_dn14 = assign105990_e158365_d_n14;

        let (assign106000_e158369, assign106000_e158369_d_n0, assign106000_e158369_d_n2, assign106000_e158369_d_n4, assign106000_e158369_d_n5, assign106000_e158369_d_n6, assign106000_e158369_d_n7, assign106000_e158369_d_n8, assign106000_e158369_d_n9, assign106000_e158369_d_n10, assign106000_e158369_d_n11, assign106000_e158369_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    } else {
        (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn11, locals.var_qs_dn14,)
    }
};
        locals.var_qs = assign106000_e158369;
        locals.var_qs_dn0 = assign106000_e158369_d_n0;
        locals.var_qs_dn2 = assign106000_e158369_d_n2;
        locals.var_qs_dn4 = assign106000_e158369_d_n4;
        locals.var_qs_dn5 = assign106000_e158369_d_n5;
        locals.var_qs_dn6 = assign106000_e158369_d_n6;
        locals.var_qs_dn7 = assign106000_e158369_d_n7;
        locals.var_qs_dn8 = assign106000_e158369_d_n8;
        locals.var_qs_dn9 = assign106000_e158369_d_n9;
        locals.var_qs_dn10 = assign106000_e158369_d_n10;
        locals.var_qs_dn11 = assign106000_e158369_d_n11;
        locals.var_qs_dn14 = assign106000_e158369_d_n14;

        let (assign106010_e158378, assign106010_e158378_d_n0, assign106010_e158378_d_n2, assign106010_e158378_d_n4, assign106010_e158378_d_n5, assign106010_e158378_d_n6, assign106010_e158378_d_n7, assign106010_e158378_d_n8, assign106010_e158378_d_n9, assign106010_e158378_d_n10, assign106010_e158378_d_n11, assign106010_e158378_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        let assign106010_e158373: f64 = (locals.var_qge + locals.var_qde);
        let assign106010_e158375: f64 = (assign106010_e158373 + locals.var_qse);
        let assign106010_e158376: f64 = (-assign106010_e158375);
        (assign106010_e158376, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn9 + locals.var_qde_dn9) + locals.var_qse_dn9)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn14 + locals.var_qde_dn14) + locals.var_qse_dn14)),)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn14,)
    }
};
        locals.var_qb = assign106010_e158378;
        locals.var_qb_dn0 = assign106010_e158378_d_n0;
        locals.var_qb_dn2 = assign106010_e158378_d_n2;
        locals.var_qb_dn4 = assign106010_e158378_d_n4;
        locals.var_qb_dn5 = assign106010_e158378_d_n5;
        locals.var_qb_dn6 = assign106010_e158378_d_n6;
        locals.var_qb_dn7 = assign106010_e158378_d_n7;
        locals.var_qb_dn8 = assign106010_e158378_d_n8;
        locals.var_qb_dn9 = assign106010_e158378_d_n9;
        locals.var_qb_dn10 = assign106010_e158378_d_n10;
        locals.var_qb_dn11 = assign106010_e158378_d_n11;
        locals.var_qb_dn14 = assign106010_e158378_d_n14;

        let (assign106020_e158382, assign106020_e158382_d_n0, assign106020_e158382_d_n2, assign106020_e158382_d_n4, assign106020_e158382_d_n5, assign106020_e158382_d_n6, assign106020_e158382_d_n7, assign106020_e158382_d_n8, assign106020_e158382_d_n9, assign106020_e158382_d_n10, assign106020_e158382_d_n11, assign106020_e158382_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn4, locals.var_isube_dn5, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn8, locals.var_isube_dn9, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn14,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn14,)
    }
};
        locals.var_isub = assign106020_e158382;
        locals.var_isub_dn0 = assign106020_e158382_d_n0;
        locals.var_isub_dn2 = assign106020_e158382_d_n2;
        locals.var_isub_dn4 = assign106020_e158382_d_n4;
        locals.var_isub_dn5 = assign106020_e158382_d_n5;
        locals.var_isub_dn6 = assign106020_e158382_d_n6;
        locals.var_isub_dn7 = assign106020_e158382_d_n7;
        locals.var_isub_dn8 = assign106020_e158382_d_n8;
        locals.var_isub_dn9 = assign106020_e158382_d_n9;
        locals.var_isub_dn10 = assign106020_e158382_d_n10;
        locals.var_isub_dn11 = assign106020_e158382_d_n11;
        locals.var_isub_dn14 = assign106020_e158382_d_n14;

        let (assign106040_e158390, assign106040_e158390_d_n0, assign106040_e158390_d_n2, assign106040_e158390_d_n4, assign106040_e158390_d_n5, assign106040_e158390_d_n6, assign106040_e158390_d_n7, assign106040_e158390_d_n8, assign106040_e158390_d_n9, assign106040_e158390_d_n10, assign106040_e158390_d_n11, assign106040_e158390_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_isublde, locals.var_isublde_dn0, locals.var_isublde_dn2, locals.var_isublde_dn4, locals.var_isublde_dn5, locals.var_isublde_dn6, locals.var_isublde_dn7, locals.var_isublde_dn8, locals.var_isublde_dn9, locals.var_isublde_dn10, locals.var_isublde_dn11, locals.var_isublde_dn14,)
    } else {
        (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn11, locals.var_isubld_dn14,)
    }
};
        locals.var_isubld = assign106040_e158390;
        locals.var_isubld_dn0 = assign106040_e158390_d_n0;
        locals.var_isubld_dn2 = assign106040_e158390_d_n2;
        locals.var_isubld_dn4 = assign106040_e158390_d_n4;
        locals.var_isubld_dn5 = assign106040_e158390_d_n5;
        locals.var_isubld_dn6 = assign106040_e158390_d_n6;
        locals.var_isubld_dn7 = assign106040_e158390_d_n7;
        locals.var_isubld_dn8 = assign106040_e158390_d_n8;
        locals.var_isubld_dn9 = assign106040_e158390_d_n9;
        locals.var_isubld_dn10 = assign106040_e158390_d_n10;
        locals.var_isubld_dn11 = assign106040_e158390_d_n11;
        locals.var_isubld_dn14 = assign106040_e158390_d_n14;

        let (assign106060_e158398, assign106060_e158398_d_n0, assign106060_e158398_d_n2, assign106060_e158398_d_n4, assign106060_e158398_d_n5, assign106060_e158398_d_n6, assign106060_e158398_d_n7, assign106060_e158398_d_n8, assign106060_e158398_d_n9, assign106060_e158398_d_n10, assign106060_e158398_d_n11, assign106060_e158398_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_idsibpce, locals.var_idsibpce_dn0, locals.var_idsibpce_dn2, locals.var_idsibpce_dn4, locals.var_idsibpce_dn5, locals.var_idsibpce_dn6, locals.var_idsibpce_dn7, locals.var_idsibpce_dn8, locals.var_idsibpce_dn9, locals.var_idsibpce_dn10, locals.var_idsibpce_dn11, locals.var_idsibpce_dn14,)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn14,)
    }
};
        locals.var_idsibpc = assign106060_e158398;
        locals.var_idsibpc_dn0 = assign106060_e158398_d_n0;
        locals.var_idsibpc_dn2 = assign106060_e158398_d_n2;
        locals.var_idsibpc_dn4 = assign106060_e158398_d_n4;
        locals.var_idsibpc_dn5 = assign106060_e158398_d_n5;
        locals.var_idsibpc_dn6 = assign106060_e158398_d_n6;
        locals.var_idsibpc_dn7 = assign106060_e158398_d_n7;
        locals.var_idsibpc_dn8 = assign106060_e158398_d_n8;
        locals.var_idsibpc_dn9 = assign106060_e158398_d_n9;
        locals.var_idsibpc_dn10 = assign106060_e158398_d_n10;
        locals.var_idsibpc_dn11 = assign106060_e158398_d_n11;
        locals.var_idsibpc_dn14 = assign106060_e158398_d_n14;

        let (assign106140_e158432, assign106140_e158432_d_n0, assign106140_e158432_d_n2, assign106140_e158432_d_n4, assign106140_e158432_d_n5, assign106140_e158432_d_n6, assign106140_e158432_d_n7, assign106140_e158432_d_n8, assign106140_e158432_d_n9, assign106140_e158432_d_n10, assign106140_e158432_d_n11, assign106140_e158432_d_n14,) = {
    if ((locals.var_guard2403 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn14,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign106140_e158432;
        locals.var_qdrat_dn0 = assign106140_e158432_d_n0;
        locals.var_qdrat_dn2 = assign106140_e158432_d_n2;
        locals.var_qdrat_dn4 = assign106140_e158432_d_n4;
        locals.var_qdrat_dn5 = assign106140_e158432_d_n5;
        locals.var_qdrat_dn6 = assign106140_e158432_d_n6;
        locals.var_qdrat_dn7 = assign106140_e158432_d_n7;
        locals.var_qdrat_dn8 = assign106140_e158432_d_n8;
        locals.var_qdrat_dn9 = assign106140_e158432_d_n9;
        locals.var_qdrat_dn10 = assign106140_e158432_d_n10;
        locals.var_qdrat_dn11 = assign106140_e158432_d_n11;
        locals.var_qdrat_dn14 = assign106140_e158432_d_n14;

        let (assign106150_e158438, assign106150_e158438_d_n0, assign106150_e158438_d_n2, assign106150_e158438_d_n4, assign106150_e158438_d_n5, assign106150_e158438_d_n6, assign106150_e158438_d_n7, assign106150_e158438_d_n8, assign106150_e158438_d_n9, assign106150_e158438_d_n10, assign106150_e158438_d_n11, assign106150_e158438_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        let assign106150_e158436: f64 = (-locals.var_idse);
        (assign106150_e158436, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn4), (-locals.var_idse_dn5), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn8), (-locals.var_idse_dn9), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn14),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign106150_e158438;
        locals.var_ids_dn0 = assign106150_e158438_d_n0;
        locals.var_ids_dn2 = assign106150_e158438_d_n2;
        locals.var_ids_dn4 = assign106150_e158438_d_n4;
        locals.var_ids_dn5 = assign106150_e158438_d_n5;
        locals.var_ids_dn6 = assign106150_e158438_d_n6;
        locals.var_ids_dn7 = assign106150_e158438_d_n7;
        locals.var_ids_dn8 = assign106150_e158438_d_n8;
        locals.var_ids_dn9 = assign106150_e158438_d_n9;
        locals.var_ids_dn10 = assign106150_e158438_d_n10;
        locals.var_ids_dn11 = assign106150_e158438_d_n11;
        locals.var_ids_dn14 = assign106150_e158438_d_n14;

        let (assign106160_e158443, assign106160_e158443_d_n0, assign106160_e158443_d_n2, assign106160_e158443_d_n4, assign106160_e158443_d_n5, assign106160_e158443_d_n6, assign106160_e158443_d_n7, assign106160_e158443_d_n8, assign106160_e158443_d_n9, assign106160_e158443_d_n10, assign106160_e158443_d_n11, assign106160_e158443_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn14,)
    }
};
        locals.var_qd = assign106160_e158443;
        locals.var_qd_dn0 = assign106160_e158443_d_n0;
        locals.var_qd_dn2 = assign106160_e158443_d_n2;
        locals.var_qd_dn4 = assign106160_e158443_d_n4;
        locals.var_qd_dn5 = assign106160_e158443_d_n5;
        locals.var_qd_dn6 = assign106160_e158443_d_n6;
        locals.var_qd_dn7 = assign106160_e158443_d_n7;
        locals.var_qd_dn8 = assign106160_e158443_d_n8;
        locals.var_qd_dn9 = assign106160_e158443_d_n9;
        locals.var_qd_dn10 = assign106160_e158443_d_n10;
        locals.var_qd_dn11 = assign106160_e158443_d_n11;
        locals.var_qd_dn14 = assign106160_e158443_d_n14;

        let (assign106170_e158448, assign106170_e158448_d_n0, assign106170_e158448_d_n2, assign106170_e158448_d_n4, assign106170_e158448_d_n5, assign106170_e158448_d_n6, assign106170_e158448_d_n7, assign106170_e158448_d_n8, assign106170_e158448_d_n9, assign106170_e158448_d_n10, assign106170_e158448_d_n11, assign106170_e158448_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn14,)
    }
};
        locals.var_qg = assign106170_e158448;
        locals.var_qg_dn0 = assign106170_e158448_d_n0;
        locals.var_qg_dn2 = assign106170_e158448_d_n2;
        locals.var_qg_dn4 = assign106170_e158448_d_n4;
        locals.var_qg_dn5 = assign106170_e158448_d_n5;
        locals.var_qg_dn6 = assign106170_e158448_d_n6;
        locals.var_qg_dn7 = assign106170_e158448_d_n7;
        locals.var_qg_dn8 = assign106170_e158448_d_n8;
        locals.var_qg_dn9 = assign106170_e158448_d_n9;
        locals.var_qg_dn10 = assign106170_e158448_d_n10;
        locals.var_qg_dn11 = assign106170_e158448_d_n11;
        locals.var_qg_dn14 = assign106170_e158448_d_n14;

        let (assign106180_e158453, assign106180_e158453_d_n0, assign106180_e158453_d_n2, assign106180_e158453_d_n4, assign106180_e158453_d_n5, assign106180_e158453_d_n6, assign106180_e158453_d_n7, assign106180_e158453_d_n8, assign106180_e158453_d_n9, assign106180_e158453_d_n10, assign106180_e158453_d_n11, assign106180_e158453_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    } else {
        (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn11, locals.var_qs_dn14,)
    }
};
        locals.var_qs = assign106180_e158453;
        locals.var_qs_dn0 = assign106180_e158453_d_n0;
        locals.var_qs_dn2 = assign106180_e158453_d_n2;
        locals.var_qs_dn4 = assign106180_e158453_d_n4;
        locals.var_qs_dn5 = assign106180_e158453_d_n5;
        locals.var_qs_dn6 = assign106180_e158453_d_n6;
        locals.var_qs_dn7 = assign106180_e158453_d_n7;
        locals.var_qs_dn8 = assign106180_e158453_d_n8;
        locals.var_qs_dn9 = assign106180_e158453_d_n9;
        locals.var_qs_dn10 = assign106180_e158453_d_n10;
        locals.var_qs_dn11 = assign106180_e158453_d_n11;
        locals.var_qs_dn14 = assign106180_e158453_d_n14;

        let (assign106190_e158463, assign106190_e158463_d_n0, assign106190_e158463_d_n2, assign106190_e158463_d_n4, assign106190_e158463_d_n5, assign106190_e158463_d_n6, assign106190_e158463_d_n7, assign106190_e158463_d_n8, assign106190_e158463_d_n9, assign106190_e158463_d_n10, assign106190_e158463_d_n11, assign106190_e158463_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        let assign106190_e158458: f64 = (locals.var_qge + locals.var_qde);
        let assign106190_e158460: f64 = (assign106190_e158458 + locals.var_qse);
        let assign106190_e158461: f64 = (-assign106190_e158460);
        (assign106190_e158461, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn9 + locals.var_qde_dn9) + locals.var_qse_dn9)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn14 + locals.var_qde_dn14) + locals.var_qse_dn14)),)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn14,)
    }
};
        locals.var_qb = assign106190_e158463;
        locals.var_qb_dn0 = assign106190_e158463_d_n0;
        locals.var_qb_dn2 = assign106190_e158463_d_n2;
        locals.var_qb_dn4 = assign106190_e158463_d_n4;
        locals.var_qb_dn5 = assign106190_e158463_d_n5;
        locals.var_qb_dn6 = assign106190_e158463_d_n6;
        locals.var_qb_dn7 = assign106190_e158463_d_n7;
        locals.var_qb_dn8 = assign106190_e158463_d_n8;
        locals.var_qb_dn9 = assign106190_e158463_d_n9;
        locals.var_qb_dn10 = assign106190_e158463_d_n10;
        locals.var_qb_dn11 = assign106190_e158463_d_n11;
        locals.var_qb_dn14 = assign106190_e158463_d_n14;

        let (assign106200_e158468, assign106200_e158468_d_n0, assign106200_e158468_d_n2, assign106200_e158468_d_n4, assign106200_e158468_d_n5, assign106200_e158468_d_n6, assign106200_e158468_d_n7, assign106200_e158468_d_n8, assign106200_e158468_d_n9, assign106200_e158468_d_n10, assign106200_e158468_d_n11, assign106200_e158468_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn14,)
    }
};
        locals.var_isub = assign106200_e158468;
        locals.var_isub_dn0 = assign106200_e158468_d_n0;
        locals.var_isub_dn2 = assign106200_e158468_d_n2;
        locals.var_isub_dn4 = assign106200_e158468_d_n4;
        locals.var_isub_dn5 = assign106200_e158468_d_n5;
        locals.var_isub_dn6 = assign106200_e158468_d_n6;
        locals.var_isub_dn7 = assign106200_e158468_d_n7;
        locals.var_isub_dn8 = assign106200_e158468_d_n8;
        locals.var_isub_dn9 = assign106200_e158468_d_n9;
        locals.var_isub_dn10 = assign106200_e158468_d_n10;
        locals.var_isub_dn11 = assign106200_e158468_d_n11;
        locals.var_isub_dn14 = assign106200_e158468_d_n14;

        let (assign106220_e158478, assign106220_e158478_d_n0, assign106220_e158478_d_n2, assign106220_e158478_d_n4, assign106220_e158478_d_n5, assign106220_e158478_d_n6, assign106220_e158478_d_n7, assign106220_e158478_d_n8, assign106220_e158478_d_n9, assign106220_e158478_d_n10, assign106220_e158478_d_n11, assign106220_e158478_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn11, locals.var_isubld_dn14,)
    }
};
        locals.var_isubld = assign106220_e158478;
        locals.var_isubld_dn0 = assign106220_e158478_d_n0;
        locals.var_isubld_dn2 = assign106220_e158478_d_n2;
        locals.var_isubld_dn4 = assign106220_e158478_d_n4;
        locals.var_isubld_dn5 = assign106220_e158478_d_n5;
        locals.var_isubld_dn6 = assign106220_e158478_d_n6;
        locals.var_isubld_dn7 = assign106220_e158478_d_n7;
        locals.var_isubld_dn8 = assign106220_e158478_d_n8;
        locals.var_isubld_dn9 = assign106220_e158478_d_n9;
        locals.var_isubld_dn10 = assign106220_e158478_d_n10;
        locals.var_isubld_dn11 = assign106220_e158478_d_n11;
        locals.var_isubld_dn14 = assign106220_e158478_d_n14;

    }

    pub(super) fn stamp_transient_block_387(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign106240_e158488, assign106240_e158488_d_n0, assign106240_e158488_d_n2, assign106240_e158488_d_n4, assign106240_e158488_d_n5, assign106240_e158488_d_n6, assign106240_e158488_d_n7, assign106240_e158488_d_n8, assign106240_e158488_d_n9, assign106240_e158488_d_n10, assign106240_e158488_d_n11, assign106240_e158488_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn14,)
    }
};
        locals.var_idsibpc = assign106240_e158488;
        locals.var_idsibpc_dn0 = assign106240_e158488_d_n0;
        locals.var_idsibpc_dn2 = assign106240_e158488_d_n2;
        locals.var_idsibpc_dn4 = assign106240_e158488_d_n4;
        locals.var_idsibpc_dn5 = assign106240_e158488_d_n5;
        locals.var_idsibpc_dn6 = assign106240_e158488_d_n6;
        locals.var_idsibpc_dn7 = assign106240_e158488_d_n7;
        locals.var_idsibpc_dn8 = assign106240_e158488_d_n8;
        locals.var_idsibpc_dn9 = assign106240_e158488_d_n9;
        locals.var_idsibpc_dn10 = assign106240_e158488_d_n10;
        locals.var_idsibpc_dn11 = assign106240_e158488_d_n11;
        locals.var_idsibpc_dn14 = assign106240_e158488_d_n14;

        let (assign106320_e158532, assign106320_e158532_d_n0, assign106320_e158532_d_n2, assign106320_e158532_d_n4, assign106320_e158532_d_n5, assign106320_e158532_d_n6, assign106320_e158532_d_n7, assign106320_e158532_d_n8, assign106320_e158532_d_n9, assign106320_e158532_d_n10, assign106320_e158532_d_n11, assign106320_e158532_d_n14,) = {
    if ((locals.var_guard2403 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign106320_e158530: f64 = (1.0 - locals.var_xd);
        (assign106320_e158530, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn4), (-locals.var_xd_dn5), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn8), (-locals.var_xd_dn9), (-locals.var_xd_dn10), (-locals.var_xd_dn11), (-locals.var_xd_dn14),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign106320_e158532;
        locals.var_qdrat_dn0 = assign106320_e158532_d_n0;
        locals.var_qdrat_dn2 = assign106320_e158532_d_n2;
        locals.var_qdrat_dn4 = assign106320_e158532_d_n4;
        locals.var_qdrat_dn5 = assign106320_e158532_d_n5;
        locals.var_qdrat_dn6 = assign106320_e158532_d_n6;
        locals.var_qdrat_dn7 = assign106320_e158532_d_n7;
        locals.var_qdrat_dn8 = assign106320_e158532_d_n8;
        locals.var_qdrat_dn9 = assign106320_e158532_d_n9;
        locals.var_qdrat_dn10 = assign106320_e158532_d_n10;
        locals.var_qdrat_dn11 = assign106320_e158532_d_n11;
        locals.var_qdrat_dn14 = assign106320_e158532_d_n14;

        let assign106330_e158535: f64 = (locals.var_qg + locals.var_qgov);
        locals.var_qg = assign106330_e158535;
        locals.var_qg_dn0 = (locals.var_qg_dn0 + locals.var_qgov_dn0);
        locals.var_qg_dn2 = (locals.var_qg_dn2 + locals.var_qgov_dn2);
        locals.var_qg_dn4 = (locals.var_qg_dn4 + locals.var_qgov_dn4);
        locals.var_qg_dn5 = (locals.var_qg_dn5 + locals.var_qgov_dn5);
        locals.var_qg_dn6 = (locals.var_qg_dn6 + locals.var_qgov_dn6);
        locals.var_qg_dn7 = (locals.var_qg_dn7 + locals.var_qgov_dn7);
        locals.var_qg_dn8 = (locals.var_qg_dn8 + locals.var_qgov_dn8);
        locals.var_qg_dn9 = (locals.var_qg_dn9 + locals.var_qgov_dn9);
        locals.var_qg_dn10 = (locals.var_qg_dn10 + locals.var_qgov_dn10);
        locals.var_qg_dn11 = (locals.var_qg_dn11 + locals.var_qgov_dn11);
        locals.var_qg_dn14 = (locals.var_qg_dn14 + locals.var_qgov_dn14);

        let assign106340_e158538: f64 = (locals.var_qd + locals.var_qdov);
        locals.var_qd = assign106340_e158538;
        locals.var_qd_dn0 = (locals.var_qd_dn0 + locals.var_qdov_dn0);
        locals.var_qd_dn2 = (locals.var_qd_dn2 + locals.var_qdov_dn2);
        locals.var_qd_dn4 = (locals.var_qd_dn4 + locals.var_qdov_dn4);
        locals.var_qd_dn5 = (locals.var_qd_dn5 + locals.var_qdov_dn5);
        locals.var_qd_dn6 = (locals.var_qd_dn6 + locals.var_qdov_dn6);
        locals.var_qd_dn7 = (locals.var_qd_dn7 + locals.var_qdov_dn7);
        locals.var_qd_dn8 = (locals.var_qd_dn8 + locals.var_qdov_dn8);
        locals.var_qd_dn9 = (locals.var_qd_dn9 + locals.var_qdov_dn9);
        locals.var_qd_dn10 = (locals.var_qd_dn10 + locals.var_qdov_dn10);
        locals.var_qd_dn11 = (locals.var_qd_dn11 + locals.var_qdov_dn11);
        locals.var_qd_dn14 = (locals.var_qd_dn14 + locals.var_qdov_dn14);

        let assign106350_e158541: f64 = (locals.var_qs + locals.var_qsov);
        locals.var_qs = assign106350_e158541;
        locals.var_qs_dn0 = (locals.var_qs_dn0 + locals.var_qsov_dn0);
        locals.var_qs_dn2 = (locals.var_qs_dn2 + locals.var_qsov_dn2);
        locals.var_qs_dn4 = (locals.var_qs_dn4 + locals.var_qsov_dn4);
        locals.var_qs_dn5 = (locals.var_qs_dn5 + locals.var_qsov_dn5);
        locals.var_qs_dn6 = (locals.var_qs_dn6 + locals.var_qsov_dn6);
        locals.var_qs_dn7 = (locals.var_qs_dn7 + locals.var_qsov_dn7);
        locals.var_qs_dn8 = (locals.var_qs_dn8 + locals.var_qsov_dn8);
        locals.var_qs_dn9 = (locals.var_qs_dn9 + locals.var_qsov_dn9);
        locals.var_qs_dn10 = (locals.var_qs_dn10 + locals.var_qsov_dn10);
        locals.var_qs_dn11 = (locals.var_qs_dn11 + locals.var_qsov_dn11);
        locals.var_qs_dn14 = (locals.var_qs_dn14 + locals.var_qsov_dn14);

        let assign106360_e158544: f64 = (locals.var_qg + locals.var_qd);
        let assign106360_e158546: f64 = (assign106360_e158544 + locals.var_qs);
        let assign106360_e158547: f64 = (-assign106360_e158546);
        locals.var_qb = assign106360_e158547;
        locals.var_qb_dn0 = (-((locals.var_qg_dn0 + locals.var_qd_dn0) + locals.var_qs_dn0));
        locals.var_qb_dn2 = (-((locals.var_qg_dn2 + locals.var_qd_dn2) + locals.var_qs_dn2));
        locals.var_qb_dn4 = (-((locals.var_qg_dn4 + locals.var_qd_dn4) + locals.var_qs_dn4));
        locals.var_qb_dn5 = (-((locals.var_qg_dn5 + locals.var_qd_dn5) + locals.var_qs_dn5));
        locals.var_qb_dn6 = (-((locals.var_qg_dn6 + locals.var_qd_dn6) + locals.var_qs_dn6));
        locals.var_qb_dn7 = (-((locals.var_qg_dn7 + locals.var_qd_dn7) + locals.var_qs_dn7));
        locals.var_qb_dn8 = (-((locals.var_qg_dn8 + locals.var_qd_dn8) + locals.var_qs_dn8));
        locals.var_qb_dn9 = (-((locals.var_qg_dn9 + locals.var_qd_dn9) + locals.var_qs_dn9));
        locals.var_qb_dn10 = (-((locals.var_qg_dn10 + locals.var_qd_dn10) + locals.var_qs_dn10));
        locals.var_qb_dn11 = (-((locals.var_qg_dn11 + locals.var_qd_dn11) + locals.var_qs_dn11));
        locals.var_qb_dn14 = (-((locals.var_qg_dn14 + locals.var_qd_dn14) + locals.var_qs_dn14));

        locals.var_qfd = locals.var_qdp;
        locals.var_qfd_dn0 = locals.var_qdp_dn0;
        locals.var_qfd_dn2 = locals.var_qdp_dn2;
        locals.var_qfd_dn7 = locals.var_qdp_dn7;

        locals.var_qfs = locals.var_qsp;
        locals.var_qfs_dn2 = locals.var_qsp_dn2;
        locals.var_qfs_dn7 = locals.var_qsp_dn7;

        locals.var_qdext = locals.var_qdexte;
        locals.var_qdext_dn0 = locals.var_qdexte_dn0;
        locals.var_qdext_dn2 = locals.var_qdexte_dn2;
        locals.var_qdext_dn4 = locals.var_qdexte_dn4;
        locals.var_qdext_dn5 = locals.var_qdexte_dn5;
        locals.var_qdext_dn6 = locals.var_qdexte_dn6;
        locals.var_qdext_dn7 = locals.var_qdexte_dn7;
        locals.var_qdext_dn8 = locals.var_qdexte_dn8;
        locals.var_qdext_dn9 = locals.var_qdexte_dn9;
        locals.var_qdext_dn10 = locals.var_qdexte_dn10;
        locals.var_qdext_dn11 = locals.var_qdexte_dn11;
        locals.var_qdext_dn14 = locals.var_qdexte_dn14;

        locals.var_qgext = locals.var_qgexte;
        locals.var_qgext_dn0 = locals.var_qgexte_dn0;
        locals.var_qgext_dn2 = locals.var_qgexte_dn2;
        locals.var_qgext_dn4 = locals.var_qgexte_dn4;
        locals.var_qgext_dn5 = locals.var_qgexte_dn5;
        locals.var_qgext_dn6 = locals.var_qgexte_dn6;
        locals.var_qgext_dn7 = locals.var_qgexte_dn7;
        locals.var_qgext_dn8 = locals.var_qgexte_dn8;
        locals.var_qgext_dn9 = locals.var_qgexte_dn9;
        locals.var_qgext_dn10 = locals.var_qgexte_dn10;
        locals.var_qgext_dn11 = locals.var_qgexte_dn11;
        locals.var_qgext_dn14 = locals.var_qgexte_dn14;

        let assign106410_e158554: f64 = (locals.var_qgexte + locals.var_qdexte);
        let assign106410_e158556: f64 = (assign106410_e158554 + locals.var_qsexte);
        let assign106410_e158557: f64 = (-assign106410_e158556);
        locals.var_qbext = assign106410_e158557;
        locals.var_qbext_dn0 = (-((locals.var_qgexte_dn0 + locals.var_qdexte_dn0) + locals.var_qsexte_dn0));
        locals.var_qbext_dn2 = (-((locals.var_qgexte_dn2 + locals.var_qdexte_dn2) + locals.var_qsexte_dn2));
        locals.var_qbext_dn4 = (-((locals.var_qgexte_dn4 + locals.var_qdexte_dn4) + locals.var_qsexte_dn4));
        locals.var_qbext_dn5 = (-((locals.var_qgexte_dn5 + locals.var_qdexte_dn5) + locals.var_qsexte_dn5));
        locals.var_qbext_dn6 = (-((locals.var_qgexte_dn6 + locals.var_qdexte_dn6) + locals.var_qsexte_dn6));
        locals.var_qbext_dn7 = (-((locals.var_qgexte_dn7 + locals.var_qdexte_dn7) + locals.var_qsexte_dn7));
        locals.var_qbext_dn8 = (-((locals.var_qgexte_dn8 + locals.var_qdexte_dn8) + locals.var_qsexte_dn8));
        locals.var_qbext_dn9 = (-((locals.var_qgexte_dn9 + locals.var_qdexte_dn9) + locals.var_qsexte_dn9));
        locals.var_qbext_dn10 = (-((locals.var_qgexte_dn10 + locals.var_qdexte_dn10) + locals.var_qsexte_dn10));
        locals.var_qbext_dn11 = (-((locals.var_qgexte_dn11 + locals.var_qdexte_dn11) + locals.var_qsexte_dn11));
        locals.var_qbext_dn14 = (-((locals.var_qgexte_dn14 + locals.var_qdexte_dn14) + locals.var_qsexte_dn14));

        let assign106420_e158560: f64 = if p.p53 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2404 = assign106420_e158560;

        let assign106430_e158563: f64 = if locals.var_rth > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard2405 = assign106430_e158563;

        let (assign106440_e158571, assign106440_e158571_d_n0, assign106440_e158571_d_n2, assign106440_e158571_d_n4, assign106440_e158571_d_n5, assign106440_e158571_d_n6, assign106440_e158571_d_n7, assign106440_e158571_d_n8, assign106440_e158571_d_n9, assign106440_e158571_d_n10, assign106440_e158571_d_n11, assign106440_e158571_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2405 != 0.0)) {
        let assign106440_e158569: f64 = (1.0 / locals.var_rth);
        (assign106440_e158569, (-(locals.var_rth_dn0 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn2 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn4 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn5 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn6 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn7 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn8 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn9 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn10 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn11 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn14 / (locals.var_rth * locals.var_rth))),)
    } else {
        (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14,)
    }
};
        locals.var_gth = assign106440_e158571;
        locals.var_gth_dn0 = assign106440_e158571_d_n0;
        locals.var_gth_dn2 = assign106440_e158571_d_n2;
        locals.var_gth_dn4 = assign106440_e158571_d_n4;
        locals.var_gth_dn5 = assign106440_e158571_d_n5;
        locals.var_gth_dn6 = assign106440_e158571_d_n6;
        locals.var_gth_dn7 = assign106440_e158571_d_n7;
        locals.var_gth_dn8 = assign106440_e158571_d_n8;
        locals.var_gth_dn9 = assign106440_e158571_d_n9;
        locals.var_gth_dn10 = assign106440_e158571_d_n10;
        locals.var_gth_dn11 = assign106440_e158571_d_n11;
        locals.var_gth_dn14 = assign106440_e158571_d_n14;

        let (assign106450_e158580, assign106450_e158580_d_n0, assign106450_e158580_d_n2, assign106450_e158580_d_n4, assign106450_e158580_d_n5, assign106450_e158580_d_n6, assign106450_e158580_d_n7, assign106450_e158580_d_n8, assign106450_e158580_d_n9, assign106450_e158580_d_n10, assign106450_e158580_d_n11, assign106450_e158580_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2405 == 0.0)) {
        let assign106450_e158578: f64 = (1.0 / 0.0001);
        (assign106450_e158578, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14,)
    }
};
        locals.var_gth = assign106450_e158580;
        locals.var_gth_dn0 = assign106450_e158580_d_n0;
        locals.var_gth_dn2 = assign106450_e158580_d_n2;
        locals.var_gth_dn4 = assign106450_e158580_d_n4;
        locals.var_gth_dn5 = assign106450_e158580_d_n5;
        locals.var_gth_dn6 = assign106450_e158580_d_n6;
        locals.var_gth_dn7 = assign106450_e158580_d_n7;
        locals.var_gth_dn8 = assign106450_e158580_d_n8;
        locals.var_gth_dn9 = assign106450_e158580_d_n9;
        locals.var_gth_dn10 = assign106450_e158580_d_n10;
        locals.var_gth_dn11 = assign106450_e158580_d_n11;
        locals.var_gth_dn14 = assign106450_e158580_d_n14;

        let assign106460_e158584: f64 = (locals.var_vdsei - locals.var_vdsi);
        let assign106460_e158585: f64 = (locals.var_vdsi * assign106460_e158584);
        let assign106460_e158587: f64 = if assign106460_e158585 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2406 = assign106460_e158587;

        let assign106470_e158590: f64 = if locals.var_uc_powrat == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2407 = assign106470_e158590;

        let (assign106480_e158598, assign106480_e158598_d_n0, assign106480_e158598_d_n2, assign106480_e158598_d_n4, assign106480_e158598_d_n5, assign106480_e158598_d_n6, assign106480_e158598_d_n7, assign106480_e158598_d_n8, assign106480_e158598_d_n9, assign106480_e158598_d_n10, assign106480_e158598_d_n11, assign106480_e158598_d_n14,) = {
    if (((locals.var_guard2404 != 0.0) && (locals.var_guard2406 != 0.0)) && (locals.var_guard2407 != 0.0)) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14,)
    }
};
        locals.var_veffpower = assign106480_e158598;
        locals.var_veffpower_dn0 = assign106480_e158598_d_n0;
        locals.var_veffpower_dn2 = assign106480_e158598_d_n2;
        locals.var_veffpower_dn4 = assign106480_e158598_d_n4;
        locals.var_veffpower_dn5 = assign106480_e158598_d_n5;
        locals.var_veffpower_dn6 = assign106480_e158598_d_n6;
        locals.var_veffpower_dn7 = assign106480_e158598_d_n7;
        locals.var_veffpower_dn8 = assign106480_e158598_d_n8;
        locals.var_veffpower_dn9 = assign106480_e158598_d_n9;
        locals.var_veffpower_dn10 = assign106480_e158598_d_n10;
        locals.var_veffpower_dn11 = assign106480_e158598_d_n11;
        locals.var_veffpower_dn14 = assign106480_e158598_d_n14;

        let (assign106490_e158613, assign106490_e158613_d_n0, assign106490_e158613_d_n2, assign106490_e158613_d_n4, assign106490_e158613_d_n5, assign106490_e158613_d_n6, assign106490_e158613_d_n7, assign106490_e158613_d_n8, assign106490_e158613_d_n9, assign106490_e158613_d_n10, assign106490_e158613_d_n11, assign106490_e158613_d_n14,) = {
    if (((locals.var_guard2404 != 0.0) && (locals.var_guard2406 != 0.0)) && (locals.var_guard2407 == 0.0)) {
        let assign106490_e158609: f64 = (locals.var_vdsei - locals.var_vdsi);
        let assign106490_e158610: f64 = (locals.var_powratio * assign106490_e158609);
        let assign106490_e158611: f64 = (locals.var_vdsi + assign106490_e158610);
        (assign106490_e158611, ((locals.var_powratio_dn0 * assign106490_e158609) + (locals.var_powratio * locals.var_vdsei_dn0)), ((locals.var_powratio_dn2 * assign106490_e158609) + (locals.var_powratio * locals.var_vdsei_dn2)), (locals.var_powratio_dn4 * assign106490_e158609), (locals.var_powratio_dn5 * assign106490_e158609), (locals.var_vdsi_dn6 + ((locals.var_powratio_dn6 * assign106490_e158609) + (locals.var_powratio * (-locals.var_vdsi_dn6)))), (locals.var_powratio_dn7 * assign106490_e158609), (locals.var_vdsi_dn8 + ((locals.var_powratio_dn8 * assign106490_e158609) + (locals.var_powratio * (-locals.var_vdsi_dn8)))), (locals.var_powratio_dn9 * assign106490_e158609), (locals.var_powratio_dn10 * assign106490_e158609), (locals.var_powratio_dn11 * assign106490_e158609), (locals.var_powratio_dn14 * assign106490_e158609),)
    } else {
        (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14,)
    }
};
        locals.var_veffpower = assign106490_e158613;
        locals.var_veffpower_dn0 = assign106490_e158613_d_n0;
        locals.var_veffpower_dn2 = assign106490_e158613_d_n2;
        locals.var_veffpower_dn4 = assign106490_e158613_d_n4;
        locals.var_veffpower_dn5 = assign106490_e158613_d_n5;
        locals.var_veffpower_dn6 = assign106490_e158613_d_n6;
        locals.var_veffpower_dn7 = assign106490_e158613_d_n7;
        locals.var_veffpower_dn8 = assign106490_e158613_d_n8;
        locals.var_veffpower_dn9 = assign106490_e158613_d_n9;
        locals.var_veffpower_dn10 = assign106490_e158613_d_n10;
        locals.var_veffpower_dn11 = assign106490_e158613_d_n11;
        locals.var_veffpower_dn14 = assign106490_e158613_d_n14;

        let (assign106500_e158620, assign106500_e158620_d_n0, assign106500_e158620_d_n2, assign106500_e158620_d_n4, assign106500_e158620_d_n5, assign106500_e158620_d_n6, assign106500_e158620_d_n7, assign106500_e158620_d_n8, assign106500_e158620_d_n9, assign106500_e158620_d_n10, assign106500_e158620_d_n11, assign106500_e158620_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2406 == 0.0)) {
        (locals.var_vdsi, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, locals.var_vdsi_dn8, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14,)
    }
};
        locals.var_veffpower = assign106500_e158620;
        locals.var_veffpower_dn0 = assign106500_e158620_d_n0;
        locals.var_veffpower_dn2 = assign106500_e158620_d_n2;
        locals.var_veffpower_dn4 = assign106500_e158620_d_n4;
        locals.var_veffpower_dn5 = assign106500_e158620_d_n5;
        locals.var_veffpower_dn6 = assign106500_e158620_d_n6;
        locals.var_veffpower_dn7 = assign106500_e158620_d_n7;
        locals.var_veffpower_dn8 = assign106500_e158620_d_n8;
        locals.var_veffpower_dn9 = assign106500_e158620_d_n9;
        locals.var_veffpower_dn10 = assign106500_e158620_d_n10;
        locals.var_veffpower_dn11 = assign106500_e158620_d_n11;
        locals.var_veffpower_dn14 = assign106500_e158620_d_n14;

        let (assign106510_e158626, assign106510_e158626_d_n0, assign106510_e158626_d_n2, assign106510_e158626_d_n4, assign106510_e158626_d_n5, assign106510_e158626_d_n6, assign106510_e158626_d_n7, assign106510_e158626_d_n8, assign106510_e158626_d_n9, assign106510_e158626_d_n10, assign106510_e158626_d_n11, assign106510_e158626_d_n14,) = {
    if (locals.var_guard2404 != 0.0) {
        let assign106510_e158624: f64 = (locals.var_ids * locals.var_veffpower);
        (assign106510_e158624, ((locals.var_ids_dn0 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn0)), ((locals.var_ids_dn2 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn2)), ((locals.var_ids_dn4 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn4)), ((locals.var_ids_dn5 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn5)), ((locals.var_ids_dn6 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn6)), ((locals.var_ids_dn7 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn7)), ((locals.var_ids_dn8 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn8)), ((locals.var_ids_dn9 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn9)), ((locals.var_ids_dn10 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn10)), ((locals.var_ids_dn11 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn11)), ((locals.var_ids_dn14 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn14)),)
    } else {
        (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14,)
    }
};
        locals.var_p = assign106510_e158626;
        locals.var_p_dn0 = assign106510_e158626_d_n0;
        locals.var_p_dn2 = assign106510_e158626_d_n2;
        locals.var_p_dn4 = assign106510_e158626_d_n4;
        locals.var_p_dn5 = assign106510_e158626_d_n5;
        locals.var_p_dn6 = assign106510_e158626_d_n6;
        locals.var_p_dn7 = assign106510_e158626_d_n7;
        locals.var_p_dn8 = assign106510_e158626_d_n8;
        locals.var_p_dn9 = assign106510_e158626_d_n9;
        locals.var_p_dn10 = assign106510_e158626_d_n10;
        locals.var_p_dn11 = assign106510_e158626_d_n11;
        locals.var_p_dn14 = assign106510_e158626_d_n14;

        let assign106520_e158629: f64 = if p.p53 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2408 = assign106520_e158629;

        let (assign106530_e158637, assign106530_e158637_d_n0, assign106530_e158637_d_n2, assign106530_e158637_d_n4, assign106530_e158637_d_n5, assign106530_e158637_d_n6, assign106530_e158637_d_n7, assign106530_e158637_d_n8, assign106530_e158637_d_n9, assign106530_e158637_d_n10, assign106530_e158637_d_n11, assign106530_e158637_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106530_e158635: f64 = (p.p433 * locals.var_gth);
        (assign106530_e158635, (p.p433 * locals.var_gth_dn0), (p.p433 * locals.var_gth_dn2), (p.p433 * locals.var_gth_dn4), (p.p433 * locals.var_gth_dn5), (p.p433 * locals.var_gth_dn6), (p.p433 * locals.var_gth_dn7), (p.p433 * locals.var_gth_dn8), (p.p433 * locals.var_gth_dn9), (p.p433 * locals.var_gth_dn10), (p.p433 * locals.var_gth_dn11), (p.p433 * locals.var_gth_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign106530_e158637;
        locals.var_t1_dn0 = assign106530_e158637_d_n0;
        locals.var_t1_dn2 = assign106530_e158637_d_n2;
        locals.var_t1_dn4 = assign106530_e158637_d_n4;
        locals.var_t1_dn5 = assign106530_e158637_d_n5;
        locals.var_t1_dn6 = assign106530_e158637_d_n6;
        locals.var_t1_dn7 = assign106530_e158637_d_n7;
        locals.var_t1_dn8 = assign106530_e158637_d_n8;
        locals.var_t1_dn9 = assign106530_e158637_d_n9;
        locals.var_t1_dn10 = assign106530_e158637_d_n10;
        locals.var_t1_dn11 = assign106530_e158637_d_n11;
        locals.var_t1_dn14 = assign106530_e158637_d_n14;

        let (assign106540_e158649, assign106540_e158649_d_n0, assign106540_e158649_d_n2, assign106540_e158649_d_n4, assign106540_e158649_d_n5, assign106540_e158649_d_n6, assign106540_e158649_d_n7, assign106540_e158649_d_n8, assign106540_e158649_d_n9, assign106540_e158649_d_n10, assign106540_e158649_d_n11, assign106540_e158649_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106540_e158643: f64 = (locals.var_t1 - locals.var_p);
        let assign106540_e158646: f64 = (p.p337 * locals.var_gth);
        let assign106540_e158647: f64 = (assign106540_e158643 - assign106540_e158646);
        (assign106540_e158647, ((locals.var_t1_dn0 - locals.var_p_dn0) - (p.p337 * locals.var_gth_dn0)), ((locals.var_t1_dn2 - locals.var_p_dn2) - (p.p337 * locals.var_gth_dn2)), ((locals.var_t1_dn4 - locals.var_p_dn4) - (p.p337 * locals.var_gth_dn4)), ((locals.var_t1_dn5 - locals.var_p_dn5) - (p.p337 * locals.var_gth_dn5)), ((locals.var_t1_dn6 - locals.var_p_dn6) - (p.p337 * locals.var_gth_dn6)), ((locals.var_t1_dn7 - locals.var_p_dn7) - (p.p337 * locals.var_gth_dn7)), ((locals.var_t1_dn8 - locals.var_p_dn8) - (p.p337 * locals.var_gth_dn8)), ((locals.var_t1_dn9 - locals.var_p_dn9) - (p.p337 * locals.var_gth_dn9)), ((locals.var_t1_dn10 - locals.var_p_dn10) - (p.p337 * locals.var_gth_dn10)), ((locals.var_t1_dn11 - locals.var_p_dn11) - (p.p337 * locals.var_gth_dn11)), ((locals.var_t1_dn14 - locals.var_p_dn14) - (p.p337 * locals.var_gth_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign106540_e158649;
        locals.var_tmf1_dn0 = assign106540_e158649_d_n0;
        locals.var_tmf1_dn2 = assign106540_e158649_d_n2;
        locals.var_tmf1_dn4 = assign106540_e158649_d_n4;
        locals.var_tmf1_dn5 = assign106540_e158649_d_n5;
        locals.var_tmf1_dn6 = assign106540_e158649_d_n6;
        locals.var_tmf1_dn7 = assign106540_e158649_d_n7;
        locals.var_tmf1_dn8 = assign106540_e158649_d_n8;
        locals.var_tmf1_dn9 = assign106540_e158649_d_n9;
        locals.var_tmf1_dn10 = assign106540_e158649_d_n10;
        locals.var_tmf1_dn11 = assign106540_e158649_d_n11;
        locals.var_tmf1_dn14 = assign106540_e158649_d_n14;

        let (assign106550_e158661, assign106550_e158661_d_n0, assign106550_e158661_d_n2, assign106550_e158661_d_n4, assign106550_e158661_d_n5, assign106550_e158661_d_n6, assign106550_e158661_d_n7, assign106550_e158661_d_n8, assign106550_e158661_d_n9, assign106550_e158661_d_n10, assign106550_e158661_d_n11, assign106550_e158661_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106550_e158655: f64 = (4.0 * locals.var_t1);
        let assign106550_e158658: f64 = (p.p337 * locals.var_gth);
        let assign106550_e158659: f64 = (assign106550_e158655 * assign106550_e158658);
        (assign106550_e158659, (((4.0 * locals.var_t1_dn0) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn0))), (((4.0 * locals.var_t1_dn2) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn2))), (((4.0 * locals.var_t1_dn4) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn4))), (((4.0 * locals.var_t1_dn5) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn5))), (((4.0 * locals.var_t1_dn6) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn6))), (((4.0 * locals.var_t1_dn7) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn7))), (((4.0 * locals.var_t1_dn8) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn8))), (((4.0 * locals.var_t1_dn9) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn9))), (((4.0 * locals.var_t1_dn10) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn10))), (((4.0 * locals.var_t1_dn11) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn11))), (((4.0 * locals.var_t1_dn14) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn14))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign106550_e158661;
        locals.var_tmf2_dn0 = assign106550_e158661_d_n0;
        locals.var_tmf2_dn2 = assign106550_e158661_d_n2;
        locals.var_tmf2_dn4 = assign106550_e158661_d_n4;
        locals.var_tmf2_dn5 = assign106550_e158661_d_n5;
        locals.var_tmf2_dn6 = assign106550_e158661_d_n6;
        locals.var_tmf2_dn7 = assign106550_e158661_d_n7;
        locals.var_tmf2_dn8 = assign106550_e158661_d_n8;
        locals.var_tmf2_dn9 = assign106550_e158661_d_n9;
        locals.var_tmf2_dn10 = assign106550_e158661_d_n10;
        locals.var_tmf2_dn11 = assign106550_e158661_d_n11;
        locals.var_tmf2_dn14 = assign106550_e158661_d_n14;

        let (assign106560_e158673, assign106560_e158673_d_n0, assign106560_e158673_d_n2, assign106560_e158673_d_n4, assign106560_e158673_d_n5, assign106560_e158673_d_n6, assign106560_e158673_d_n7, assign106560_e158673_d_n8, assign106560_e158673_d_n9, assign106560_e158673_d_n10, assign106560_e158673_d_n11, assign106560_e158673_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let (assign106560_e158671, assign106560_e158671_d_n0, assign106560_e158671_d_n2, assign106560_e158671_d_n4, assign106560_e158671_d_n5, assign106560_e158671_d_n6, assign106560_e158671_d_n7, assign106560_e158671_d_n8, assign106560_e158671_d_n9, assign106560_e158671_d_n10, assign106560_e158671_d_n11, assign106560_e158671_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign106560_e158670: f64 = (-locals.var_tmf2);
                (assign106560_e158670, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign106560_e158671, assign106560_e158671_d_n0, assign106560_e158671_d_n2, assign106560_e158671_d_n4, assign106560_e158671_d_n5, assign106560_e158671_d_n6, assign106560_e158671_d_n7, assign106560_e158671_d_n8, assign106560_e158671_d_n9, assign106560_e158671_d_n10, assign106560_e158671_d_n11, assign106560_e158671_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign106560_e158673;
        locals.var_tmf2_dn0 = assign106560_e158673_d_n0;
        locals.var_tmf2_dn2 = assign106560_e158673_d_n2;
        locals.var_tmf2_dn4 = assign106560_e158673_d_n4;
        locals.var_tmf2_dn5 = assign106560_e158673_d_n5;
        locals.var_tmf2_dn6 = assign106560_e158673_d_n6;
        locals.var_tmf2_dn7 = assign106560_e158673_d_n7;
        locals.var_tmf2_dn8 = assign106560_e158673_d_n8;
        locals.var_tmf2_dn9 = assign106560_e158673_d_n9;
        locals.var_tmf2_dn10 = assign106560_e158673_d_n10;
        locals.var_tmf2_dn11 = assign106560_e158673_d_n11;
        locals.var_tmf2_dn14 = assign106560_e158673_d_n14;

        let (assign106570_e158684, assign106570_e158684_d_n0, assign106570_e158684_d_n2, assign106570_e158684_d_n4, assign106570_e158684_d_n5, assign106570_e158684_d_n6, assign106570_e158684_d_n7, assign106570_e158684_d_n8, assign106570_e158684_d_n9, assign106570_e158684_d_n10, assign106570_e158684_d_n11, assign106570_e158684_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106570_e158679: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign106570_e158681: f64 = (assign106570_e158679 + locals.var_tmf2);
        let assign106570_e158682: f64 = (assign106570_e158681).sqrt();
        (assign106570_e158682, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign106570_e158682)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign106570_e158684;
        locals.var_tmf2_dn0 = assign106570_e158684_d_n0;
        locals.var_tmf2_dn2 = assign106570_e158684_d_n2;
        locals.var_tmf2_dn4 = assign106570_e158684_d_n4;
        locals.var_tmf2_dn5 = assign106570_e158684_d_n5;
        locals.var_tmf2_dn6 = assign106570_e158684_d_n6;
        locals.var_tmf2_dn7 = assign106570_e158684_d_n7;
        locals.var_tmf2_dn8 = assign106570_e158684_d_n8;
        locals.var_tmf2_dn9 = assign106570_e158684_d_n9;
        locals.var_tmf2_dn10 = assign106570_e158684_d_n10;
        locals.var_tmf2_dn11 = assign106570_e158684_d_n11;
        locals.var_tmf2_dn14 = assign106570_e158684_d_n14;

        let (assign106580_e158696, assign106580_e158696_d_n0, assign106580_e158696_d_n2, assign106580_e158696_d_n4, assign106580_e158696_d_n5, assign106580_e158696_d_n6, assign106580_e158696_d_n7, assign106580_e158696_d_n8, assign106580_e158696_d_n9, assign106580_e158696_d_n10, assign106580_e158696_d_n11, assign106580_e158696_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106580_e158692: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign106580_e158693: f64 = (1.0 + assign106580_e158692);
        let assign106580_e158694: f64 = (0.5 * assign106580_e158693);
        (assign106580_e158694, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign106580_e158696;
        locals.var_t0_dn0 = assign106580_e158696_d_n0;
        locals.var_t0_dn2 = assign106580_e158696_d_n2;
        locals.var_t0_dn4 = assign106580_e158696_d_n4;
        locals.var_t0_dn5 = assign106580_e158696_d_n5;
        locals.var_t0_dn6 = assign106580_e158696_d_n6;
        locals.var_t0_dn7 = assign106580_e158696_d_n7;
        locals.var_t0_dn8 = assign106580_e158696_d_n8;
        locals.var_t0_dn9 = assign106580_e158696_d_n9;
        locals.var_t0_dn10 = assign106580_e158696_d_n10;
        locals.var_t0_dn11 = assign106580_e158696_d_n11;
        locals.var_t0_dn14 = assign106580_e158696_d_n14;

        let (assign106590_e158708, assign106590_e158708_d_n0, assign106590_e158708_d_n2, assign106590_e158708_d_n4, assign106590_e158708_d_n5, assign106590_e158708_d_n6, assign106590_e158708_d_n7, assign106590_e158708_d_n8, assign106590_e158708_d_n9, assign106590_e158708_d_n10, assign106590_e158708_d_n11, assign106590_e158708_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106590_e158704: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign106590_e158705: f64 = (0.5 * assign106590_e158704);
        let assign106590_e158706: f64 = (locals.var_t1 - assign106590_e158705);
        (assign106590_e158706, (locals.var_t1_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t1_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t1_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t1_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t1_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t1_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t1_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t1_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t1_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t1_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t1_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign106590_e158708;
        locals.var_t2_dn0 = assign106590_e158708_d_n0;
        locals.var_t2_dn2 = assign106590_e158708_d_n2;
        locals.var_t2_dn4 = assign106590_e158708_d_n4;
        locals.var_t2_dn5 = assign106590_e158708_d_n5;
        locals.var_t2_dn6 = assign106590_e158708_d_n6;
        locals.var_t2_dn7 = assign106590_e158708_d_n7;
        locals.var_t2_dn8 = assign106590_e158708_d_n8;
        locals.var_t2_dn9 = assign106590_e158708_d_n9;
        locals.var_t2_dn10 = assign106590_e158708_d_n10;
        locals.var_t2_dn11 = assign106590_e158708_d_n11;
        locals.var_t2_dn14 = assign106590_e158708_d_n14;

        let (assign106600_e158714, assign106600_e158714_d_n0, assign106600_e158714_d_n2, assign106600_e158714_d_n4, assign106600_e158714_d_n5, assign106600_e158714_d_n6, assign106600_e158714_d_n7, assign106600_e158714_d_n8, assign106600_e158714_d_n9, assign106600_e158714_d_n10, assign106600_e158714_d_n11, assign106600_e158714_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14,)
    }
};
        locals.var_p = assign106600_e158714;
        locals.var_p_dn0 = assign106600_e158714_d_n0;
        locals.var_p_dn2 = assign106600_e158714_d_n2;
        locals.var_p_dn4 = assign106600_e158714_d_n4;
        locals.var_p_dn5 = assign106600_e158714_d_n5;
        locals.var_p_dn6 = assign106600_e158714_d_n6;
        locals.var_p_dn7 = assign106600_e158714_d_n7;
        locals.var_p_dn8 = assign106600_e158714_d_n8;
        locals.var_p_dn9 = assign106600_e158714_d_n9;
        locals.var_p_dn10 = assign106600_e158714_d_n10;
        locals.var_p_dn11 = assign106600_e158714_d_n11;
        locals.var_p_dn14 = assign106600_e158714_d_n14;

        let (assign106610_e158719, assign106610_e158719_d_n0, assign106610_e158719_d_n2, assign106610_e158719_d_n4, assign106610_e158719_d_n5, assign106610_e158719_d_n6, assign106610_e158719_d_n7, assign106610_e158719_d_n8, assign106610_e158719_d_n9, assign106610_e158719_d_n10, assign106610_e158719_d_n11, assign106610_e158719_d_n14,) = {
    if (locals.var_guard2404 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14,)
    }
};
        locals.var_gth = assign106610_e158719;
        locals.var_gth_dn0 = assign106610_e158719_d_n0;
        locals.var_gth_dn2 = assign106610_e158719_d_n2;
        locals.var_gth_dn4 = assign106610_e158719_d_n4;
        locals.var_gth_dn5 = assign106610_e158719_d_n5;
        locals.var_gth_dn6 = assign106610_e158719_d_n6;
        locals.var_gth_dn7 = assign106610_e158719_d_n7;
        locals.var_gth_dn8 = assign106610_e158719_d_n8;
        locals.var_gth_dn9 = assign106610_e158719_d_n9;
        locals.var_gth_dn10 = assign106610_e158719_d_n10;
        locals.var_gth_dn11 = assign106610_e158719_d_n11;
        locals.var_gth_dn14 = assign106610_e158719_d_n14;

    }

    pub(super) fn stamp_transient_block_388(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign106620_e158724, assign106620_e158724_d_n0, assign106620_e158724_d_n2, assign106620_e158724_d_n4, assign106620_e158724_d_n5, assign106620_e158724_d_n6, assign106620_e158724_d_n7, assign106620_e158724_d_n8, assign106620_e158724_d_n9, assign106620_e158724_d_n10, assign106620_e158724_d_n11, assign106620_e158724_d_n14,) = {
    if (locals.var_guard2404 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14,)
    }
};
        locals.var_p = assign106620_e158724;
        locals.var_p_dn0 = assign106620_e158724_d_n0;
        locals.var_p_dn2 = assign106620_e158724_d_n2;
        locals.var_p_dn4 = assign106620_e158724_d_n4;
        locals.var_p_dn5 = assign106620_e158724_d_n5;
        locals.var_p_dn6 = assign106620_e158724_d_n6;
        locals.var_p_dn7 = assign106620_e158724_d_n7;
        locals.var_p_dn8 = assign106620_e158724_d_n8;
        locals.var_p_dn9 = assign106620_e158724_d_n9;
        locals.var_p_dn10 = assign106620_e158724_d_n10;
        locals.var_p_dn11 = assign106620_e158724_d_n11;
        locals.var_p_dn14 = assign106620_e158724_d_n14;

        let (assign106690_e158764, assign106690_e158764_d_n0, assign106690_e158764_d_n2, assign106690_e158764_d_n4, assign106690_e158764_d_n5, assign106690_e158764_d_n6, assign106690_e158764_d_n7, assign106690_e158764_d_n8, assign106690_e158764_d_n9, assign106690_e158764_d_n10, assign106690_e158764_d_n11, assign106690_e158764_d_n12, assign106690_e158764_d_n14,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign106690_e158762: f64 = (locals.var_qi_nqs * locals.var_qdrat);
        (assign106690_e158762, (locals.var_qi_nqs * locals.var_qdrat_dn0), (locals.var_qi_nqs * locals.var_qdrat_dn2), (locals.var_qi_nqs * locals.var_qdrat_dn4), (locals.var_qi_nqs * locals.var_qdrat_dn5), (locals.var_qi_nqs * locals.var_qdrat_dn6), (locals.var_qi_nqs * locals.var_qdrat_dn7), (locals.var_qi_nqs * locals.var_qdrat_dn8), (locals.var_qi_nqs * locals.var_qdrat_dn9), (locals.var_qi_nqs * locals.var_qdrat_dn10), (locals.var_qi_nqs * locals.var_qdrat_dn11), (locals.var_qi_nqs_dn12 * locals.var_qdrat), (locals.var_qi_nqs * locals.var_qdrat_dn14),)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn14,)
    }
};
        locals.var_qd_nqs = assign106690_e158764;
        locals.var_qd_nqs_dn0 = assign106690_e158764_d_n0;
        locals.var_qd_nqs_dn2 = assign106690_e158764_d_n2;
        locals.var_qd_nqs_dn4 = assign106690_e158764_d_n4;
        locals.var_qd_nqs_dn5 = assign106690_e158764_d_n5;
        locals.var_qd_nqs_dn6 = assign106690_e158764_d_n6;
        locals.var_qd_nqs_dn7 = assign106690_e158764_d_n7;
        locals.var_qd_nqs_dn8 = assign106690_e158764_d_n8;
        locals.var_qd_nqs_dn9 = assign106690_e158764_d_n9;
        locals.var_qd_nqs_dn10 = assign106690_e158764_d_n10;
        locals.var_qd_nqs_dn11 = assign106690_e158764_d_n11;
        locals.var_qd_nqs_dn12 = assign106690_e158764_d_n12;
        locals.var_qd_nqs_dn14 = assign106690_e158764_d_n14;

        let (assign106700_e158771, assign106700_e158771_d_n12, assign106700_e158771_d_n13,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign106700_e158767: f64 = (-locals.var_qi_nqs);
        let assign106700_e158769: f64 = (assign106700_e158767 - locals.var_qb_nqs);
        (assign106700_e158769, (-locals.var_qi_nqs_dn12), (-locals.var_qb_nqs_dn13),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13,)
    }
};
        locals.var_qg_nqs = assign106700_e158771;
        locals.var_qg_nqs_dn12 = assign106700_e158771_d_n12;
        locals.var_qg_nqs_dn13 = assign106700_e158771_d_n13;

        let (assign106710_e158779, assign106710_e158779_d_n0, assign106710_e158779_d_n2, assign106710_e158779_d_n4, assign106710_e158779_d_n5, assign106710_e158779_d_n6, assign106710_e158779_d_n7, assign106710_e158779_d_n8, assign106710_e158779_d_n9, assign106710_e158779_d_n10, assign106710_e158779_d_n11, assign106710_e158779_d_n12, assign106710_e158779_d_n14,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign106710_e158776: f64 = (1.0 - locals.var_qdrat);
        let assign106710_e158777: f64 = (locals.var_qi_nqs * assign106710_e158776);
        (assign106710_e158777, (locals.var_qi_nqs * (-locals.var_qdrat_dn0)), (locals.var_qi_nqs * (-locals.var_qdrat_dn2)), (locals.var_qi_nqs * (-locals.var_qdrat_dn4)), (locals.var_qi_nqs * (-locals.var_qdrat_dn5)), (locals.var_qi_nqs * (-locals.var_qdrat_dn6)), (locals.var_qi_nqs * (-locals.var_qdrat_dn7)), (locals.var_qi_nqs * (-locals.var_qdrat_dn8)), (locals.var_qi_nqs * (-locals.var_qdrat_dn9)), (locals.var_qi_nqs * (-locals.var_qdrat_dn10)), (locals.var_qi_nqs * (-locals.var_qdrat_dn11)), (locals.var_qi_nqs_dn12 * assign106710_e158776), (locals.var_qi_nqs * (-locals.var_qdrat_dn14)),)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn14,)
    }
};
        locals.var_qs_nqs = assign106710_e158779;
        locals.var_qs_nqs_dn0 = assign106710_e158779_d_n0;
        locals.var_qs_nqs_dn2 = assign106710_e158779_d_n2;
        locals.var_qs_nqs_dn4 = assign106710_e158779_d_n4;
        locals.var_qs_nqs_dn5 = assign106710_e158779_d_n5;
        locals.var_qs_nqs_dn6 = assign106710_e158779_d_n6;
        locals.var_qs_nqs_dn7 = assign106710_e158779_d_n7;
        locals.var_qs_nqs_dn8 = assign106710_e158779_d_n8;
        locals.var_qs_nqs_dn9 = assign106710_e158779_d_n9;
        locals.var_qs_nqs_dn10 = assign106710_e158779_d_n10;
        locals.var_qs_nqs_dn11 = assign106710_e158779_d_n11;
        locals.var_qs_nqs_dn12 = assign106710_e158779_d_n12;
        locals.var_qs_nqs_dn14 = assign106710_e158779_d_n14;

        let (assign106740_e158794, assign106740_e158794_d_n0, assign106740_e158794_d_n2, assign106740_e158794_d_n4, assign106740_e158794_d_n5, assign106740_e158794_d_n6, assign106740_e158794_d_n7, assign106740_e158794_d_n8, assign106740_e158794_d_n9, assign106740_e158794_d_n10, assign106740_e158794_d_n11, assign106740_e158794_d_n12, assign106740_e158794_d_n14,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn14,)
    }
};
        locals.var_qd_nqs = assign106740_e158794;
        locals.var_qd_nqs_dn0 = assign106740_e158794_d_n0;
        locals.var_qd_nqs_dn2 = assign106740_e158794_d_n2;
        locals.var_qd_nqs_dn4 = assign106740_e158794_d_n4;
        locals.var_qd_nqs_dn5 = assign106740_e158794_d_n5;
        locals.var_qd_nqs_dn6 = assign106740_e158794_d_n6;
        locals.var_qd_nqs_dn7 = assign106740_e158794_d_n7;
        locals.var_qd_nqs_dn8 = assign106740_e158794_d_n8;
        locals.var_qd_nqs_dn9 = assign106740_e158794_d_n9;
        locals.var_qd_nqs_dn10 = assign106740_e158794_d_n10;
        locals.var_qd_nqs_dn11 = assign106740_e158794_d_n11;
        locals.var_qd_nqs_dn12 = assign106740_e158794_d_n12;
        locals.var_qd_nqs_dn14 = assign106740_e158794_d_n14;

        let (assign106750_e158799, assign106750_e158799_d_n12, assign106750_e158799_d_n13,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13,)
    }
};
        locals.var_qg_nqs = assign106750_e158799;
        locals.var_qg_nqs_dn12 = assign106750_e158799_d_n12;
        locals.var_qg_nqs_dn13 = assign106750_e158799_d_n13;

        let (assign106760_e158804, assign106760_e158804_d_n0, assign106760_e158804_d_n2, assign106760_e158804_d_n4, assign106760_e158804_d_n5, assign106760_e158804_d_n6, assign106760_e158804_d_n7, assign106760_e158804_d_n8, assign106760_e158804_d_n9, assign106760_e158804_d_n10, assign106760_e158804_d_n11, assign106760_e158804_d_n12, assign106760_e158804_d_n14,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn14,)
    }
};
        locals.var_qs_nqs = assign106760_e158804;
        locals.var_qs_nqs_dn0 = assign106760_e158804_d_n0;
        locals.var_qs_nqs_dn2 = assign106760_e158804_d_n2;
        locals.var_qs_nqs_dn4 = assign106760_e158804_d_n4;
        locals.var_qs_nqs_dn5 = assign106760_e158804_d_n5;
        locals.var_qs_nqs_dn6 = assign106760_e158804_d_n6;
        locals.var_qs_nqs_dn7 = assign106760_e158804_d_n7;
        locals.var_qs_nqs_dn8 = assign106760_e158804_d_n8;
        locals.var_qs_nqs_dn9 = assign106760_e158804_d_n9;
        locals.var_qs_nqs_dn10 = assign106760_e158804_d_n10;
        locals.var_qs_nqs_dn11 = assign106760_e158804_d_n11;
        locals.var_qs_nqs_dn12 = assign106760_e158804_d_n12;
        locals.var_qs_nqs_dn14 = assign106760_e158804_d_n14;

        let assign106770_e158807: f64 = (p.p87 * locals.var_mode);
        let assign106770_e158809: f64 = (assign106770_e158807 * locals.var_ids);
        locals.var_idse = assign106770_e158809;
        locals.var_idse_dn0 = (assign106770_e158807 * locals.var_ids_dn0);
        locals.var_idse_dn2 = (assign106770_e158807 * locals.var_ids_dn2);
        locals.var_idse_dn4 = (assign106770_e158807 * locals.var_ids_dn4);
        locals.var_idse_dn5 = (assign106770_e158807 * locals.var_ids_dn5);
        locals.var_idse_dn6 = (assign106770_e158807 * locals.var_ids_dn6);
        locals.var_idse_dn7 = (assign106770_e158807 * locals.var_ids_dn7);
        locals.var_idse_dn8 = (assign106770_e158807 * locals.var_ids_dn8);
        locals.var_idse_dn9 = (assign106770_e158807 * locals.var_ids_dn9);
        locals.var_idse_dn10 = (assign106770_e158807 * locals.var_ids_dn10);
        locals.var_idse_dn11 = (assign106770_e158807 * locals.var_ids_dn11);
        locals.var_idse_dn14 = (assign106770_e158807 * locals.var_ids_dn14);

        let assign106930_e158857: f64 = locals.var_qg_dn6;
        locals.var_cgdbd = assign106930_e158857;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn4 = 0.0;
        locals.var_cgdbd_dn5 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn8 = 0.0;
        locals.var_cgdbd_dn9 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn14 = 0.0;

        let assign106940_e158860: f64 = (p.p87 * locals.var_cgdbd);
        locals.var_cgdbd = assign106940_e158860;
        locals.var_cgdbd_dn0 = (p.p87 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p87 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn4 = (p.p87 * locals.var_cgdbd_dn4);
        locals.var_cgdbd_dn5 = (p.p87 * locals.var_cgdbd_dn5);
        locals.var_cgdbd_dn6 = (p.p87 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p87 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn8 = (p.p87 * locals.var_cgdbd_dn8);
        locals.var_cgdbd_dn9 = (p.p87 * locals.var_cgdbd_dn9);
        locals.var_cgdbd_dn10 = (p.p87 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p87 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn14 = (p.p87 * locals.var_cgdbd_dn14);

        let assign106950_e158863: f64 = locals.var_qg_dn8;
        locals.var_cgsbd = assign106950_e158863;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn4 = 0.0;
        locals.var_cgsbd_dn5 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn8 = 0.0;
        locals.var_cgsbd_dn9 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn14 = 0.0;

        let assign106960_e158866: f64 = (p.p87 * locals.var_cgsbd);
        locals.var_cgsbd = assign106960_e158866;
        locals.var_cgsbd_dn0 = (p.p87 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p87 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn4 = (p.p87 * locals.var_cgsbd_dn4);
        locals.var_cgsbd_dn5 = (p.p87 * locals.var_cgsbd_dn5);
        locals.var_cgsbd_dn6 = (p.p87 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p87 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn8 = (p.p87 * locals.var_cgsbd_dn8);
        locals.var_cgsbd_dn9 = (p.p87 * locals.var_cgsbd_dn9);
        locals.var_cgsbd_dn10 = (p.p87 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p87 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn14 = (p.p87 * locals.var_cgsbd_dn14);

        let assign107330_e158981: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2411 = assign107330_e158981;

        let (assign107360_e158993, assign107360_e158993_d_n0, assign107360_e158993_d_n2, assign107360_e158993_d_n4, assign107360_e158993_d_n5, assign107360_e158993_d_n6, assign107360_e158993_d_n7, assign107360_e158993_d_n8, assign107360_e158993_d_n9, assign107360_e158993_d_n10, assign107360_e158993_d_n11, assign107360_e158993_d_n14,) = {
    if (locals.var_guard2411 != 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14,)
    } else {
        (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn11, locals.var_cgsb_dn14,)
    }
};
        locals.var_cgsb = assign107360_e158993;
        locals.var_cgsb_dn0 = assign107360_e158993_d_n0;
        locals.var_cgsb_dn2 = assign107360_e158993_d_n2;
        locals.var_cgsb_dn4 = assign107360_e158993_d_n4;
        locals.var_cgsb_dn5 = assign107360_e158993_d_n5;
        locals.var_cgsb_dn6 = assign107360_e158993_d_n6;
        locals.var_cgsb_dn7 = assign107360_e158993_d_n7;
        locals.var_cgsb_dn8 = assign107360_e158993_d_n8;
        locals.var_cgsb_dn9 = assign107360_e158993_d_n9;
        locals.var_cgsb_dn10 = assign107360_e158993_d_n10;
        locals.var_cgsb_dn11 = assign107360_e158993_d_n11;
        locals.var_cgsb_dn14 = assign107360_e158993_d_n14;

        let (assign107460_e159037, assign107460_e159037_d_n0, assign107460_e159037_d_n2, assign107460_e159037_d_n4, assign107460_e159037_d_n5, assign107460_e159037_d_n6, assign107460_e159037_d_n7, assign107460_e159037_d_n8, assign107460_e159037_d_n9, assign107460_e159037_d_n10, assign107460_e159037_d_n11, assign107460_e159037_d_n14,) = {
    if (locals.var_guard2411 == 0.0) {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14,)
    } else {
        (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn11, locals.var_cgsb_dn14,)
    }
};
        locals.var_cgsb = assign107460_e159037;
        locals.var_cgsb_dn0 = assign107460_e159037_d_n0;
        locals.var_cgsb_dn2 = assign107460_e159037_d_n2;
        locals.var_cgsb_dn4 = assign107460_e159037_d_n4;
        locals.var_cgsb_dn5 = assign107460_e159037_d_n5;
        locals.var_cgsb_dn6 = assign107460_e159037_d_n6;
        locals.var_cgsb_dn7 = assign107460_e159037_d_n7;
        locals.var_cgsb_dn8 = assign107460_e159037_d_n8;
        locals.var_cgsb_dn9 = assign107460_e159037_d_n9;
        locals.var_cgsb_dn10 = assign107460_e159037_d_n10;
        locals.var_cgsb_dn11 = assign107460_e159037_d_n11;
        locals.var_cgsb_dn14 = assign107460_e159037_d_n14;

        let assign107690_e159100: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2413 = assign107690_e159100;

        let (assign107780_e159145,) = {
    if (p.p28 != 0.0) {
        (1.0,)
    } else {
        (locals.var_cqi,)
    }
};
        locals.var_cqi = assign107780_e159145;

        let (assign107790_e159149,) = {
    if (p.p28 != 0.0) {
        (1.0,)
    } else {
        (locals.var_cqb,)
    }
};
        locals.var_cqb = assign107790_e159149;

    }

    pub(super) fn stamp_reactive_block_0(
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign50_e1412: f64 = if param_given[12] { 1.0 } else { 0.0 };
        locals.var_nsubcdfm_given = assign50_e1412;
        locals.var_nsubcdfm_given_rv = 0.0;

        let assign60_e1414: f64 = if param_given[268] { 1.0 } else { 0.0 };
        locals.var_cgdo_given = assign60_e1414;
        locals.var_cgdo_given_rv = 0.0;

        let assign70_e1416: f64 = if param_given[269] { 1.0 } else { 0.0 };
        locals.var_cgso_given = assign70_e1416;
        locals.var_cgso_given_rv = 0.0;

        locals.var_cgdoe = 0.0;
        locals.var_cgdoe_rv = 0.0;

        locals.var_cgsoe = 0.0;
        locals.var_cgsoe_rv = 0.0;

        locals.var_xd = 0.0;
        locals.var_xd_dn0 = 0.0;
        locals.var_xd_dn2 = 0.0;
        locals.var_xd_dn4 = 0.0;
        locals.var_xd_dn5 = 0.0;
        locals.var_xd_dn6 = 0.0;
        locals.var_xd_dn7 = 0.0;
        locals.var_xd_dn8 = 0.0;
        locals.var_xd_dn9 = 0.0;
        locals.var_xd_dn10 = 0.0;
        locals.var_xd_dn11 = 0.0;
        locals.var_xd_dn14 = 0.0;
        locals.var_xd_rv = 0.0;

        locals.var_rdd = 0.0;
        locals.var_rdd_dn0 = 0.0;
        locals.var_rdd_dn2 = 0.0;
        locals.var_rdd_dn4 = 0.0;
        locals.var_rdd_dn5 = 0.0;
        locals.var_rdd_dn6 = 0.0;
        locals.var_rdd_dn7 = 0.0;
        locals.var_rdd_dn8 = 0.0;
        locals.var_rdd_dn9 = 0.0;
        locals.var_rdd_dn10 = 0.0;
        locals.var_rdd_dn11 = 0.0;
        locals.var_rdd_dn14 = 0.0;
        locals.var_rdd_rv = 0.0;

        locals.var_rsd = 0.0;
        locals.var_rsd_dn0 = 0.0;
        locals.var_rsd_dn2 = 0.0;
        locals.var_rsd_dn4 = 0.0;
        locals.var_rsd_dn5 = 0.0;
        locals.var_rsd_dn6 = 0.0;
        locals.var_rsd_dn7 = 0.0;
        locals.var_rsd_dn8 = 0.0;
        locals.var_rsd_dn9 = 0.0;
        locals.var_rsd_dn10 = 0.0;
        locals.var_rsd_dn11 = 0.0;
        locals.var_rsd_dn14 = 0.0;
        locals.var_rsd_rv = 0.0;

        locals.var_flg_ign = 0.0;
        locals.var_flg_ign_rv = 0.0;

        locals.var_flg_noqi = 0.0;
        locals.var_flg_noqi_rv = 0.0;

        locals.var_flg_rsrd = 0.0;
        locals.var_flg_rsrd_rv = 0.0;

        locals.var_flg_zone = 0.0;
        locals.var_flg_zone_rv = 0.0;

        locals.var_rd_ps0ld = 0.0;
        locals.var_rd_ps0ld_dn0 = 0.0;
        locals.var_rd_ps0ld_dn2 = 0.0;
        locals.var_rd_ps0ld_dn4 = 0.0;
        locals.var_rd_ps0ld_dn5 = 0.0;
        locals.var_rd_ps0ld_dn6 = 0.0;
        locals.var_rd_ps0ld_dn7 = 0.0;
        locals.var_rd_ps0ld_dn8 = 0.0;
        locals.var_rd_ps0ld_dn9 = 0.0;
        locals.var_rd_ps0ld_dn10 = 0.0;
        locals.var_rd_ps0ld_dn11 = 0.0;
        locals.var_rd_ps0ld_dn14 = 0.0;
        locals.var_rd_ps0ld_rv = 0.0;

        locals.var_rd_qbuld = 0.0;
        locals.var_rd_qbuld_dn0 = 0.0;
        locals.var_rd_qbuld_dn2 = 0.0;
        locals.var_rd_qbuld_dn4 = 0.0;
        locals.var_rd_qbuld_dn5 = 0.0;
        locals.var_rd_qbuld_dn6 = 0.0;
        locals.var_rd_qbuld_dn7 = 0.0;
        locals.var_rd_qbuld_dn8 = 0.0;
        locals.var_rd_qbuld_dn9 = 0.0;
        locals.var_rd_qbuld_dn10 = 0.0;
        locals.var_rd_qbuld_dn11 = 0.0;
        locals.var_rd_qbuld_dn14 = 0.0;
        locals.var_rd_qbuld_rv = 0.0;

        locals.var_vbs_max = 0.8;
        locals.var_vbs_max_dn0 = 0.0;
        locals.var_vbs_max_dn2 = 0.0;
        locals.var_vbs_max_dn4 = 0.0;
        locals.var_vbs_max_dn5 = 0.0;
        locals.var_vbs_max_dn6 = 0.0;
        locals.var_vbs_max_dn7 = 0.0;
        locals.var_vbs_max_dn8 = 0.0;
        locals.var_vbs_max_dn9 = 0.0;
        locals.var_vbs_max_dn10 = 0.0;
        locals.var_vbs_max_dn11 = 0.0;
        locals.var_vbs_max_dn14 = 0.0;
        locals.var_vbs_max_rv = 0.0;

        locals.var_vbs_bnd = 0.4;
        locals.var_vbs_bnd_dn0 = 0.0;
        locals.var_vbs_bnd_dn2 = 0.0;
        locals.var_vbs_bnd_dn4 = 0.0;
        locals.var_vbs_bnd_dn5 = 0.0;
        locals.var_vbs_bnd_dn6 = 0.0;
        locals.var_vbs_bnd_dn7 = 0.0;
        locals.var_vbs_bnd_dn8 = 0.0;
        locals.var_vbs_bnd_dn9 = 0.0;
        locals.var_vbs_bnd_dn10 = 0.0;
        locals.var_vbs_bnd_dn11 = 0.0;
        locals.var_vbs_bnd_dn14 = 0.0;
        locals.var_vbs_bnd_rv = 0.0;

        locals.var_flg_pprv = 0.0;
        locals.var_flg_pprv_rv = 0.0;

        locals.var_flg_conv = 0.0;
        locals.var_flg_conv_rv = 0.0;

        locals.var_flg_qme = 0.0;
        locals.var_flg_qme_rv = 0.0;

        locals.var_flg_nqs = 0.0;
        locals.var_flg_nqs_rv = 0.0;

        locals.var_vbscl = 0.0;
        locals.var_vbscl_dn0 = 0.0;
        locals.var_vbscl_dn2 = 0.0;
        locals.var_vbscl_dn4 = 0.0;
        locals.var_vbscl_dn5 = 0.0;
        locals.var_vbscl_dn6 = 0.0;
        locals.var_vbscl_dn7 = 0.0;
        locals.var_vbscl_dn8 = 0.0;
        locals.var_vbscl_dn9 = 0.0;
        locals.var_vbscl_dn10 = 0.0;
        locals.var_vbscl_dn11 = 0.0;
        locals.var_vbscl_dn14 = 0.0;
        locals.var_vbscl_rv = 0.0;

        locals.var_vbscldvbs = 0.0;
        locals.var_vbscldvbs_dn0 = 0.0;
        locals.var_vbscldvbs_dn2 = 0.0;
        locals.var_vbscldvbs_dn4 = 0.0;
        locals.var_vbscldvbs_dn5 = 0.0;
        locals.var_vbscldvbs_dn6 = 0.0;
        locals.var_vbscldvbs_dn7 = 0.0;
        locals.var_vbscldvbs_dn8 = 0.0;
        locals.var_vbscldvbs_dn9 = 0.0;
        locals.var_vbscldvbs_dn10 = 0.0;
        locals.var_vbscldvbs_dn11 = 0.0;
        locals.var_vbscldvbs_dn14 = 0.0;
        locals.var_vbscldvbs_rv = 0.0;

        locals.var_vgp = 0.0;
        locals.var_vgp_dn0 = 0.0;
        locals.var_vgp_dn2 = 0.0;
        locals.var_vgp_dn4 = 0.0;
        locals.var_vgp_dn5 = 0.0;
        locals.var_vgp_dn6 = 0.0;
        locals.var_vgp_dn7 = 0.0;
        locals.var_vgp_dn8 = 0.0;
        locals.var_vgp_dn9 = 0.0;
        locals.var_vgp_dn10 = 0.0;
        locals.var_vgp_dn11 = 0.0;
        locals.var_vgp_dn14 = 0.0;
        locals.var_vgp_rv = 0.0;

        locals.var_vgs_fb = 0.0;
        locals.var_vgs_fb_dn0 = 0.0;
        locals.var_vgs_fb_dn2 = 0.0;
        locals.var_vgs_fb_dn4 = 0.0;
        locals.var_vgs_fb_dn5 = 0.0;
        locals.var_vgs_fb_dn6 = 0.0;
        locals.var_vgs_fb_dn7 = 0.0;
        locals.var_vgs_fb_dn8 = 0.0;
        locals.var_vgs_fb_dn9 = 0.0;
        locals.var_vgs_fb_dn10 = 0.0;
        locals.var_vgs_fb_dn11 = 0.0;
        locals.var_vgs_fb_dn14 = 0.0;
        locals.var_vgs_fb_rv = 0.0;

        locals.var_ps0 = 0.0;
        locals.var_ps0_dn0 = 0.0;
        locals.var_ps0_dn2 = 0.0;
        locals.var_ps0_dn4 = 0.0;
        locals.var_ps0_dn5 = 0.0;
        locals.var_ps0_dn6 = 0.0;
        locals.var_ps0_dn7 = 0.0;
        locals.var_ps0_dn8 = 0.0;
        locals.var_ps0_dn9 = 0.0;
        locals.var_ps0_dn10 = 0.0;
        locals.var_ps0_dn11 = 0.0;
        locals.var_ps0_dn14 = 0.0;
        locals.var_ps0_rv = 0.0;

        locals.var_ps0_ini = 0.0;
        locals.var_ps0_ini_dn0 = 0.0;
        locals.var_ps0_ini_dn2 = 0.0;
        locals.var_ps0_ini_dn4 = 0.0;
        locals.var_ps0_ini_dn5 = 0.0;
        locals.var_ps0_ini_dn6 = 0.0;
        locals.var_ps0_ini_dn7 = 0.0;
        locals.var_ps0_ini_dn8 = 0.0;
        locals.var_ps0_ini_dn9 = 0.0;
        locals.var_ps0_ini_dn10 = 0.0;
        locals.var_ps0_ini_dn11 = 0.0;
        locals.var_ps0_ini_dn14 = 0.0;
        locals.var_ps0_ini_rv = 0.0;

        locals.var_ps0_inia = 0.0;
        locals.var_ps0_inia_dn0 = 0.0;
        locals.var_ps0_inia_dn2 = 0.0;
        locals.var_ps0_inia_dn4 = 0.0;
        locals.var_ps0_inia_dn5 = 0.0;
        locals.var_ps0_inia_dn6 = 0.0;
        locals.var_ps0_inia_dn7 = 0.0;
        locals.var_ps0_inia_dn8 = 0.0;
        locals.var_ps0_inia_dn9 = 0.0;
        locals.var_ps0_inia_dn10 = 0.0;
        locals.var_ps0_inia_dn11 = 0.0;
        locals.var_ps0_inia_dn14 = 0.0;
        locals.var_ps0_inia_rv = 0.0;

        locals.var_ps0_inib = 0.0;
        locals.var_ps0_inib_dn0 = 0.0;
        locals.var_ps0_inib_dn2 = 0.0;
        locals.var_ps0_inib_dn4 = 0.0;
        locals.var_ps0_inib_dn5 = 0.0;
        locals.var_ps0_inib_dn6 = 0.0;
        locals.var_ps0_inib_dn7 = 0.0;
        locals.var_ps0_inib_dn8 = 0.0;
        locals.var_ps0_inib_dn9 = 0.0;
        locals.var_ps0_inib_dn10 = 0.0;
        locals.var_ps0_inib_dn11 = 0.0;
        locals.var_ps0_inib_dn14 = 0.0;
        locals.var_ps0_inib_rv = 0.0;

        locals.var_psl = 0.0;
        locals.var_psl_dn0 = 0.0;
        locals.var_psl_dn2 = 0.0;
        locals.var_psl_dn4 = 0.0;
        locals.var_psl_dn5 = 0.0;
        locals.var_psl_dn6 = 0.0;
        locals.var_psl_dn7 = 0.0;
        locals.var_psl_dn8 = 0.0;
        locals.var_psl_dn9 = 0.0;
        locals.var_psl_dn10 = 0.0;
        locals.var_psl_dn11 = 0.0;
        locals.var_psl_dn14 = 0.0;
        locals.var_psl_rv = 0.0;

        locals.var_psl_lim = 0.0;
        locals.var_psl_lim_dn0 = 0.0;
        locals.var_psl_lim_dn2 = 0.0;
        locals.var_psl_lim_dn4 = 0.0;
        locals.var_psl_lim_dn5 = 0.0;
        locals.var_psl_lim_dn6 = 0.0;
        locals.var_psl_lim_dn7 = 0.0;
        locals.var_psl_lim_dn8 = 0.0;
        locals.var_psl_lim_dn9 = 0.0;
        locals.var_psl_lim_dn10 = 0.0;
        locals.var_psl_lim_dn11 = 0.0;
        locals.var_psl_lim_dn14 = 0.0;
        locals.var_psl_lim_rv = 0.0;

        locals.var_dplim = 0.0;
        locals.var_dplim_dn0 = 0.0;
        locals.var_dplim_dn2 = 0.0;
        locals.var_dplim_dn4 = 0.0;
        locals.var_dplim_dn5 = 0.0;
        locals.var_dplim_dn6 = 0.0;
        locals.var_dplim_dn7 = 0.0;
        locals.var_dplim_dn8 = 0.0;
        locals.var_dplim_dn9 = 0.0;
        locals.var_dplim_dn10 = 0.0;
        locals.var_dplim_dn11 = 0.0;
        locals.var_dplim_dn14 = 0.0;
        locals.var_dplim_rv = 0.0;

        locals.var_pds = 0.0;
        locals.var_pds_dn0 = 0.0;
        locals.var_pds_dn2 = 0.0;
        locals.var_pds_dn4 = 0.0;
        locals.var_pds_dn5 = 0.0;
        locals.var_pds_dn6 = 0.0;
        locals.var_pds_dn7 = 0.0;
        locals.var_pds_dn8 = 0.0;
        locals.var_pds_dn9 = 0.0;
        locals.var_pds_dn10 = 0.0;
        locals.var_pds_dn11 = 0.0;
        locals.var_pds_dn14 = 0.0;
        locals.var_pds_rv = 0.0;

        locals.var_pds_ini = 0.0;
        locals.var_pds_ini_dn0 = 0.0;
        locals.var_pds_ini_dn2 = 0.0;
        locals.var_pds_ini_dn4 = 0.0;
        locals.var_pds_ini_dn5 = 0.0;
        locals.var_pds_ini_dn6 = 0.0;
        locals.var_pds_ini_dn7 = 0.0;
        locals.var_pds_ini_dn8 = 0.0;
        locals.var_pds_ini_dn9 = 0.0;
        locals.var_pds_ini_dn10 = 0.0;
        locals.var_pds_ini_dn11 = 0.0;
        locals.var_pds_ini_dn14 = 0.0;
        locals.var_pds_ini_rv = 0.0;

        locals.var_pds_max = 0.0;
        locals.var_pds_max_dn0 = 0.0;
        locals.var_pds_max_dn2 = 0.0;
        locals.var_pds_max_dn4 = 0.0;
        locals.var_pds_max_dn5 = 0.0;
        locals.var_pds_max_dn6 = 0.0;
        locals.var_pds_max_dn7 = 0.0;
        locals.var_pds_max_dn8 = 0.0;
        locals.var_pds_max_dn9 = 0.0;
        locals.var_pds_max_dn10 = 0.0;
        locals.var_pds_max_dn11 = 0.0;
        locals.var_pds_max_dn14 = 0.0;
        locals.var_pds_max_rv = 0.0;

        locals.var_lp_s0 = 0.0;
        locals.var_lp_s0_rv = 0.0;

        locals.var_lp_sl = 0.0;
        locals.var_lp_sl_rv = 0.0;

        locals.var_xi0 = 0.0;
        locals.var_xi0_dn0 = 0.0;
        locals.var_xi0_dn2 = 0.0;
        locals.var_xi0_dn4 = 0.0;
        locals.var_xi0_dn5 = 0.0;
        locals.var_xi0_dn6 = 0.0;
        locals.var_xi0_dn7 = 0.0;
        locals.var_xi0_dn8 = 0.0;
        locals.var_xi0_dn9 = 0.0;
        locals.var_xi0_dn10 = 0.0;
        locals.var_xi0_dn11 = 0.0;
        locals.var_xi0_dn14 = 0.0;
        locals.var_xi0_rv = 0.0;

        locals.var_xi0p12 = 0.0;
        locals.var_xi0p12_dn0 = 0.0;
        locals.var_xi0p12_dn2 = 0.0;
        locals.var_xi0p12_dn4 = 0.0;
        locals.var_xi0p12_dn5 = 0.0;
        locals.var_xi0p12_dn6 = 0.0;
        locals.var_xi0p12_dn7 = 0.0;
        locals.var_xi0p12_dn8 = 0.0;
        locals.var_xi0p12_dn9 = 0.0;
        locals.var_xi0p12_dn10 = 0.0;
        locals.var_xi0p12_dn11 = 0.0;
        locals.var_xi0p12_dn14 = 0.0;
        locals.var_xi0p12_rv = 0.0;

        locals.var_xi0p32 = 0.0;
        locals.var_xi0p32_dn0 = 0.0;
        locals.var_xi0p32_dn2 = 0.0;
        locals.var_xi0p32_dn4 = 0.0;
        locals.var_xi0p32_dn5 = 0.0;
        locals.var_xi0p32_dn6 = 0.0;
        locals.var_xi0p32_dn7 = 0.0;
        locals.var_xi0p32_dn8 = 0.0;
        locals.var_xi0p32_dn9 = 0.0;
        locals.var_xi0p32_dn10 = 0.0;
        locals.var_xi0p32_dn11 = 0.0;
        locals.var_xi0p32_dn14 = 0.0;
        locals.var_xi0p32_rv = 0.0;

        locals.var_xil = 0.0;
        locals.var_xil_dn0 = 0.0;
        locals.var_xil_dn2 = 0.0;
        locals.var_xil_dn4 = 0.0;
        locals.var_xil_dn5 = 0.0;
        locals.var_xil_dn6 = 0.0;
        locals.var_xil_dn7 = 0.0;
        locals.var_xil_dn8 = 0.0;
        locals.var_xil_dn9 = 0.0;
        locals.var_xil_dn10 = 0.0;
        locals.var_xil_dn11 = 0.0;
        locals.var_xil_dn14 = 0.0;
        locals.var_xil_rv = 0.0;

        locals.var_xilp12 = 0.0;
        locals.var_xilp12_dn0 = 0.0;
        locals.var_xilp12_dn2 = 0.0;
        locals.var_xilp12_dn4 = 0.0;
        locals.var_xilp12_dn5 = 0.0;
        locals.var_xilp12_dn6 = 0.0;
        locals.var_xilp12_dn7 = 0.0;
        locals.var_xilp12_dn8 = 0.0;
        locals.var_xilp12_dn9 = 0.0;
        locals.var_xilp12_dn10 = 0.0;
        locals.var_xilp12_dn11 = 0.0;
        locals.var_xilp12_dn14 = 0.0;
        locals.var_xilp12_rv = 0.0;

        locals.var_xilp32 = 0.0;
        locals.var_xilp32_dn0 = 0.0;
        locals.var_xilp32_dn2 = 0.0;
        locals.var_xilp32_dn4 = 0.0;
        locals.var_xilp32_dn5 = 0.0;
        locals.var_xilp32_dn6 = 0.0;
        locals.var_xilp32_dn7 = 0.0;
        locals.var_xilp32_dn8 = 0.0;
        locals.var_xilp32_dn9 = 0.0;
        locals.var_xilp32_dn10 = 0.0;
        locals.var_xilp32_dn11 = 0.0;
        locals.var_xilp32_dn14 = 0.0;
        locals.var_xilp32_rv = 0.0;

        locals.var_vbsz = 0.0;
        locals.var_vbsz_dn0 = 0.0;
        locals.var_vbsz_dn2 = 0.0;
        locals.var_vbsz_dn4 = 0.0;
        locals.var_vbsz_dn5 = 0.0;
        locals.var_vbsz_dn6 = 0.0;
        locals.var_vbsz_dn7 = 0.0;
        locals.var_vbsz_dn8 = 0.0;
        locals.var_vbsz_dn9 = 0.0;
        locals.var_vbsz_dn10 = 0.0;
        locals.var_vbsz_dn11 = 0.0;
        locals.var_vbsz_dn14 = 0.0;
        locals.var_vbsz_rv = 0.0;

        locals.var_vdsz = 0.0;
        locals.var_vdsz_dn0 = 0.0;
        locals.var_vdsz_dn2 = 0.0;
        locals.var_vdsz_dn4 = 0.0;
        locals.var_vdsz_dn5 = 0.0;
        locals.var_vdsz_dn6 = 0.0;
        locals.var_vdsz_dn7 = 0.0;
        locals.var_vdsz_dn8 = 0.0;
        locals.var_vdsz_dn9 = 0.0;
        locals.var_vdsz_dn10 = 0.0;
        locals.var_vdsz_dn11 = 0.0;
        locals.var_vdsz_dn14 = 0.0;
        locals.var_vdsz_rv = 0.0;

        locals.var_vgsz = 0.0;
        locals.var_vgsz_dn0 = 0.0;
        locals.var_vgsz_dn2 = 0.0;
        locals.var_vgsz_dn4 = 0.0;
        locals.var_vgsz_dn5 = 0.0;
        locals.var_vgsz_dn6 = 0.0;
        locals.var_vgsz_dn7 = 0.0;
        locals.var_vgsz_dn8 = 0.0;
        locals.var_vgsz_dn9 = 0.0;
        locals.var_vgsz_dn10 = 0.0;
        locals.var_vgsz_dn11 = 0.0;
        locals.var_vgsz_dn14 = 0.0;
        locals.var_vgsz_rv = 0.0;

        locals.var_vzadd = 0.0;
        locals.var_vzadd_dn0 = 0.0;
        locals.var_vzadd_dn2 = 0.0;
        locals.var_vzadd_dn4 = 0.0;
        locals.var_vzadd_dn5 = 0.0;
        locals.var_vzadd_dn6 = 0.0;
        locals.var_vzadd_dn7 = 0.0;
        locals.var_vzadd_dn8 = 0.0;
        locals.var_vzadd_dn9 = 0.0;
        locals.var_vzadd_dn10 = 0.0;
        locals.var_vzadd_dn11 = 0.0;
        locals.var_vzadd_dn14 = 0.0;
        locals.var_vzadd_rv = 0.0;

        locals.var_ps0z = 0.0;
        locals.var_ps0z_dn0 = 0.0;
        locals.var_ps0z_dn2 = 0.0;
        locals.var_ps0z_dn4 = 0.0;
        locals.var_ps0z_dn5 = 0.0;
        locals.var_ps0z_dn6 = 0.0;
        locals.var_ps0z_dn7 = 0.0;
        locals.var_ps0z_dn8 = 0.0;
        locals.var_ps0z_dn9 = 0.0;
        locals.var_ps0z_dn10 = 0.0;
        locals.var_ps0z_dn11 = 0.0;
        locals.var_ps0z_dn14 = 0.0;
        locals.var_ps0z_rv = 0.0;

        locals.var_pzadd = 0.0;
        locals.var_pzadd_dn0 = 0.0;
        locals.var_pzadd_dn2 = 0.0;
        locals.var_pzadd_dn4 = 0.0;
        locals.var_pzadd_dn5 = 0.0;
        locals.var_pzadd_dn6 = 0.0;
        locals.var_pzadd_dn7 = 0.0;
        locals.var_pzadd_dn8 = 0.0;
        locals.var_pzadd_dn9 = 0.0;
        locals.var_pzadd_dn10 = 0.0;
        locals.var_pzadd_dn11 = 0.0;
        locals.var_pzadd_dn14 = 0.0;
        locals.var_pzadd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        locals: &mut StampLocals,
    ) {
        locals.var_dvbsibpc = 0.0;
        locals.var_dvbsibpc_dn0 = 0.0;
        locals.var_dvbsibpc_dn2 = 0.0;
        locals.var_dvbsibpc_dn4 = 0.0;
        locals.var_dvbsibpc_dn5 = 0.0;
        locals.var_dvbsibpc_dn6 = 0.0;
        locals.var_dvbsibpc_dn7 = 0.0;
        locals.var_dvbsibpc_dn8 = 0.0;
        locals.var_dvbsibpc_dn9 = 0.0;
        locals.var_dvbsibpc_dn10 = 0.0;
        locals.var_dvbsibpc_dn11 = 0.0;
        locals.var_dvbsibpc_dn14 = 0.0;
        locals.var_dvbsibpc_rv = 0.0;

        locals.var_dg3 = 0.0;
        locals.var_dg3_dn0 = 0.0;
        locals.var_dg3_dn2 = 0.0;
        locals.var_dg3_dn4 = 0.0;
        locals.var_dg3_dn5 = 0.0;
        locals.var_dg3_dn6 = 0.0;
        locals.var_dg3_dn7 = 0.0;
        locals.var_dg3_dn8 = 0.0;
        locals.var_dg3_dn9 = 0.0;
        locals.var_dg3_dn10 = 0.0;
        locals.var_dg3_dn11 = 0.0;
        locals.var_dg3_dn14 = 0.0;
        locals.var_dg3_rv = 0.0;

        locals.var_dg4 = 0.0;
        locals.var_dg4_dn0 = 0.0;
        locals.var_dg4_dn2 = 0.0;
        locals.var_dg4_dn4 = 0.0;
        locals.var_dg4_dn5 = 0.0;
        locals.var_dg4_dn6 = 0.0;
        locals.var_dg4_dn7 = 0.0;
        locals.var_dg4_dn8 = 0.0;
        locals.var_dg4_dn9 = 0.0;
        locals.var_dg4_dn10 = 0.0;
        locals.var_dg4_dn11 = 0.0;
        locals.var_dg4_dn14 = 0.0;
        locals.var_dg4_rv = 0.0;

        locals.var_didd = 0.0;
        locals.var_didd_dn0 = 0.0;
        locals.var_didd_dn2 = 0.0;
        locals.var_didd_dn4 = 0.0;
        locals.var_didd_dn5 = 0.0;
        locals.var_didd_dn6 = 0.0;
        locals.var_didd_dn7 = 0.0;
        locals.var_didd_dn8 = 0.0;
        locals.var_didd_dn9 = 0.0;
        locals.var_didd_dn10 = 0.0;
        locals.var_didd_dn11 = 0.0;
        locals.var_didd_dn14 = 0.0;
        locals.var_didd_rv = 0.0;

        locals.var_betawl = 0.0;
        locals.var_betawl_dn0 = 0.0;
        locals.var_betawl_dn2 = 0.0;
        locals.var_betawl_dn4 = 0.0;
        locals.var_betawl_dn5 = 0.0;
        locals.var_betawl_dn6 = 0.0;
        locals.var_betawl_dn7 = 0.0;
        locals.var_betawl_dn8 = 0.0;
        locals.var_betawl_dn9 = 0.0;
        locals.var_betawl_dn10 = 0.0;
        locals.var_betawl_dn11 = 0.0;
        locals.var_betawl_dn14 = 0.0;
        locals.var_betawl_rv = 0.0;

        locals.var_chi = 0.0;
        locals.var_chi_dn0 = 0.0;
        locals.var_chi_dn2 = 0.0;
        locals.var_chi_dn4 = 0.0;
        locals.var_chi_dn5 = 0.0;
        locals.var_chi_dn6 = 0.0;
        locals.var_chi_dn7 = 0.0;
        locals.var_chi_dn8 = 0.0;
        locals.var_chi_dn9 = 0.0;
        locals.var_chi_dn10 = 0.0;
        locals.var_chi_dn11 = 0.0;
        locals.var_chi_dn14 = 0.0;
        locals.var_chi_rv = 0.0;

        locals.var_chib = 0.0;
        locals.var_chib_dn0 = 0.0;
        locals.var_chib_dn2 = 0.0;
        locals.var_chib_dn4 = 0.0;
        locals.var_chib_dn5 = 0.0;
        locals.var_chib_dn6 = 0.0;
        locals.var_chib_dn7 = 0.0;
        locals.var_chib_dn8 = 0.0;
        locals.var_chib_dn9 = 0.0;
        locals.var_chib_dn10 = 0.0;
        locals.var_chib_dn11 = 0.0;
        locals.var_chib_dn14 = 0.0;
        locals.var_chib_rv = 0.0;

        locals.var_rho = 0.0;
        locals.var_rho_dn0 = 0.0;
        locals.var_rho_dn2 = 0.0;
        locals.var_rho_dn4 = 0.0;
        locals.var_rho_dn5 = 0.0;
        locals.var_rho_dn6 = 0.0;
        locals.var_rho_dn7 = 0.0;
        locals.var_rho_dn8 = 0.0;
        locals.var_rho_dn9 = 0.0;
        locals.var_rho_dn10 = 0.0;
        locals.var_rho_dn11 = 0.0;
        locals.var_rho_dn14 = 0.0;
        locals.var_rho_rv = 0.0;

        locals.var_vth = 0.0;
        locals.var_vth_dn0 = 0.0;
        locals.var_vth_dn2 = 0.0;
        locals.var_vth_dn4 = 0.0;
        locals.var_vth_dn5 = 0.0;
        locals.var_vth_dn6 = 0.0;
        locals.var_vth_dn7 = 0.0;
        locals.var_vth_dn8 = 0.0;
        locals.var_vth_dn9 = 0.0;
        locals.var_vth_dn10 = 0.0;
        locals.var_vth_dn11 = 0.0;
        locals.var_vth_dn14 = 0.0;
        locals.var_vth_rv = 0.0;

        locals.var_vth0 = 0.0;
        locals.var_vth0_dn0 = 0.0;
        locals.var_vth0_dn2 = 0.0;
        locals.var_vth0_dn4 = 0.0;
        locals.var_vth0_dn5 = 0.0;
        locals.var_vth0_dn6 = 0.0;
        locals.var_vth0_dn7 = 0.0;
        locals.var_vth0_dn8 = 0.0;
        locals.var_vth0_dn9 = 0.0;
        locals.var_vth0_dn10 = 0.0;
        locals.var_vth0_dn11 = 0.0;
        locals.var_vth0_dn14 = 0.0;
        locals.var_vth0_rv = 0.0;

        locals.var_dvth = 0.0;
        locals.var_dvth_dn0 = 0.0;
        locals.var_dvth_dn2 = 0.0;
        locals.var_dvth_dn4 = 0.0;
        locals.var_dvth_dn5 = 0.0;
        locals.var_dvth_dn6 = 0.0;
        locals.var_dvth_dn7 = 0.0;
        locals.var_dvth_dn8 = 0.0;
        locals.var_dvth_dn9 = 0.0;
        locals.var_dvth_dn10 = 0.0;
        locals.var_dvth_dn11 = 0.0;
        locals.var_dvth_dn14 = 0.0;
        locals.var_dvth_rv = 0.0;

        locals.var_dvth0 = 0.0;
        locals.var_dvth0_dn0 = 0.0;
        locals.var_dvth0_dn2 = 0.0;
        locals.var_dvth0_dn4 = 0.0;
        locals.var_dvth0_dn5 = 0.0;
        locals.var_dvth0_dn6 = 0.0;
        locals.var_dvth0_dn7 = 0.0;
        locals.var_dvth0_dn8 = 0.0;
        locals.var_dvth0_dn9 = 0.0;
        locals.var_dvth0_dn10 = 0.0;
        locals.var_dvth0_dn11 = 0.0;
        locals.var_dvth0_dn14 = 0.0;
        locals.var_dvth0_rv = 0.0;

        locals.var_dvthsc = 0.0;
        locals.var_dvthsc_dn0 = 0.0;
        locals.var_dvthsc_dn2 = 0.0;
        locals.var_dvthsc_dn4 = 0.0;
        locals.var_dvthsc_dn5 = 0.0;
        locals.var_dvthsc_dn6 = 0.0;
        locals.var_dvthsc_dn7 = 0.0;
        locals.var_dvthsc_dn8 = 0.0;
        locals.var_dvthsc_dn9 = 0.0;
        locals.var_dvthsc_dn10 = 0.0;
        locals.var_dvthsc_dn11 = 0.0;
        locals.var_dvthsc_dn14 = 0.0;
        locals.var_dvthsc_rv = 0.0;

        locals.var_pb20b = 0.0;
        locals.var_pb20b_dn0 = 0.0;
        locals.var_pb20b_dn2 = 0.0;
        locals.var_pb20b_dn4 = 0.0;
        locals.var_pb20b_dn5 = 0.0;
        locals.var_pb20b_dn6 = 0.0;
        locals.var_pb20b_dn7 = 0.0;
        locals.var_pb20b_dn8 = 0.0;
        locals.var_pb20b_dn9 = 0.0;
        locals.var_pb20b_dn10 = 0.0;
        locals.var_pb20b_dn11 = 0.0;
        locals.var_pb20b_dn14 = 0.0;
        locals.var_pb20b_rv = 0.0;

        locals.var_dvthw = 0.0;
        locals.var_dvthw_dn0 = 0.0;
        locals.var_dvthw_dn2 = 0.0;
        locals.var_dvthw_dn4 = 0.0;
        locals.var_dvthw_dn5 = 0.0;
        locals.var_dvthw_dn6 = 0.0;
        locals.var_dvthw_dn7 = 0.0;
        locals.var_dvthw_dn8 = 0.0;
        locals.var_dvthw_dn9 = 0.0;
        locals.var_dvthw_dn10 = 0.0;
        locals.var_dvthw_dn11 = 0.0;
        locals.var_dvthw_dn14 = 0.0;
        locals.var_dvthw_rv = 0.0;

        locals.var_alpha = 0.0;
        locals.var_alpha_dn0 = 0.0;
        locals.var_alpha_dn2 = 0.0;
        locals.var_alpha_dn4 = 0.0;
        locals.var_alpha_dn5 = 0.0;
        locals.var_alpha_dn6 = 0.0;
        locals.var_alpha_dn7 = 0.0;
        locals.var_alpha_dn8 = 0.0;
        locals.var_alpha_dn9 = 0.0;
        locals.var_alpha_dn10 = 0.0;
        locals.var_alpha_dn11 = 0.0;
        locals.var_alpha_dn14 = 0.0;
        locals.var_alpha_rv = 0.0;

        locals.var_achi = 0.0;
        locals.var_achi_dn0 = 0.0;
        locals.var_achi_dn2 = 0.0;
        locals.var_achi_dn4 = 0.0;
        locals.var_achi_dn5 = 0.0;
        locals.var_achi_dn6 = 0.0;
        locals.var_achi_dn7 = 0.0;
        locals.var_achi_dn8 = 0.0;
        locals.var_achi_dn9 = 0.0;
        locals.var_achi_dn10 = 0.0;
        locals.var_achi_dn11 = 0.0;
        locals.var_achi_dn14 = 0.0;
        locals.var_achi_rv = 0.0;

        locals.var_vgvt = 0.0;
        locals.var_vgvt_dn0 = 0.0;
        locals.var_vgvt_dn2 = 0.0;
        locals.var_vgvt_dn4 = 0.0;
        locals.var_vgvt_dn5 = 0.0;
        locals.var_vgvt_dn6 = 0.0;
        locals.var_vgvt_dn7 = 0.0;
        locals.var_vgvt_dn8 = 0.0;
        locals.var_vgvt_dn9 = 0.0;
        locals.var_vgvt_dn10 = 0.0;
        locals.var_vgvt_dn11 = 0.0;
        locals.var_vgvt_dn14 = 0.0;
        locals.var_vgvt_rv = 0.0;

        locals.var_pslsat = 0.0;
        locals.var_pslsat_dn0 = 0.0;
        locals.var_pslsat_dn2 = 0.0;
        locals.var_pslsat_dn4 = 0.0;
        locals.var_pslsat_dn5 = 0.0;
        locals.var_pslsat_dn6 = 0.0;
        locals.var_pslsat_dn7 = 0.0;
        locals.var_pslsat_dn8 = 0.0;
        locals.var_pslsat_dn9 = 0.0;
        locals.var_pslsat_dn10 = 0.0;
        locals.var_pslsat_dn11 = 0.0;
        locals.var_pslsat_dn14 = 0.0;
        locals.var_pslsat_rv = 0.0;

        locals.var_vdsats = 0.0;
        locals.var_vdsats_dn0 = 0.0;
        locals.var_vdsats_dn2 = 0.0;
        locals.var_vdsats_dn4 = 0.0;
        locals.var_vdsats_dn5 = 0.0;
        locals.var_vdsats_dn6 = 0.0;
        locals.var_vdsats_dn7 = 0.0;
        locals.var_vdsats_dn8 = 0.0;
        locals.var_vdsats_dn9 = 0.0;
        locals.var_vdsats_dn10 = 0.0;
        locals.var_vdsats_dn11 = 0.0;
        locals.var_vdsats_dn14 = 0.0;
        locals.var_vdsats_rv = 0.0;

        locals.var_delta = 0.0;
        locals.var_delta_dn0 = 0.0;
        locals.var_delta_dn2 = 0.0;
        locals.var_delta_dn4 = 0.0;
        locals.var_delta_dn5 = 0.0;
        locals.var_delta_dn6 = 0.0;
        locals.var_delta_dn7 = 0.0;
        locals.var_delta_dn8 = 0.0;
        locals.var_delta_dn9 = 0.0;
        locals.var_delta_dn10 = 0.0;
        locals.var_delta_dn11 = 0.0;
        locals.var_delta_dn14 = 0.0;
        locals.var_delta_rv = 0.0;

        locals.var_qb = 0.0;
        locals.var_qb_dn0 = 0.0;
        locals.var_qb_dn2 = 0.0;
        locals.var_qb_dn4 = 0.0;
        locals.var_qb_dn5 = 0.0;
        locals.var_qb_dn6 = 0.0;
        locals.var_qb_dn7 = 0.0;
        locals.var_qb_dn8 = 0.0;
        locals.var_qb_dn9 = 0.0;
        locals.var_qb_dn10 = 0.0;
        locals.var_qb_dn11 = 0.0;
        locals.var_qb_dn14 = 0.0;
        locals.var_qb_rv = 0.0;

        locals.var_qbu = 0.0;
        locals.var_qbu_dn0 = 0.0;
        locals.var_qbu_dn2 = 0.0;
        locals.var_qbu_dn4 = 0.0;
        locals.var_qbu_dn5 = 0.0;
        locals.var_qbu_dn6 = 0.0;
        locals.var_qbu_dn7 = 0.0;
        locals.var_qbu_dn8 = 0.0;
        locals.var_qbu_dn9 = 0.0;
        locals.var_qbu_dn10 = 0.0;
        locals.var_qbu_dn11 = 0.0;
        locals.var_qbu_dn14 = 0.0;
        locals.var_qbu_rv = 0.0;

        locals.var_qi = 0.0;
        locals.var_qi_dn0 = 0.0;
        locals.var_qi_dn2 = 0.0;
        locals.var_qi_dn4 = 0.0;
        locals.var_qi_dn5 = 0.0;
        locals.var_qi_dn6 = 0.0;
        locals.var_qi_dn7 = 0.0;
        locals.var_qi_dn8 = 0.0;
        locals.var_qi_dn9 = 0.0;
        locals.var_qi_dn10 = 0.0;
        locals.var_qi_dn11 = 0.0;
        locals.var_qi_dn14 = 0.0;
        locals.var_qi_rv = 0.0;

        locals.var_qiu = 0.0;
        locals.var_qiu_dn0 = 0.0;
        locals.var_qiu_dn2 = 0.0;
        locals.var_qiu_dn4 = 0.0;
        locals.var_qiu_dn5 = 0.0;
        locals.var_qiu_dn6 = 0.0;
        locals.var_qiu_dn7 = 0.0;
        locals.var_qiu_dn8 = 0.0;
        locals.var_qiu_dn9 = 0.0;
        locals.var_qiu_dn10 = 0.0;
        locals.var_qiu_dn11 = 0.0;
        locals.var_qiu_dn14 = 0.0;
        locals.var_qiu_rv = 0.0;

        locals.var_qd = 0.0;
        locals.var_qd_dn0 = 0.0;
        locals.var_qd_dn2 = 0.0;
        locals.var_qd_dn4 = 0.0;
        locals.var_qd_dn5 = 0.0;
        locals.var_qd_dn6 = 0.0;
        locals.var_qd_dn7 = 0.0;
        locals.var_qd_dn8 = 0.0;
        locals.var_qd_dn9 = 0.0;
        locals.var_qd_dn10 = 0.0;
        locals.var_qd_dn11 = 0.0;
        locals.var_qd_dn14 = 0.0;
        locals.var_qd_rv = 0.0;

        locals.var_ids = 0.0;
        locals.var_ids_dn0 = 0.0;
        locals.var_ids_dn2 = 0.0;
        locals.var_ids_dn4 = 0.0;
        locals.var_ids_dn5 = 0.0;
        locals.var_ids_dn6 = 0.0;
        locals.var_ids_dn7 = 0.0;
        locals.var_ids_dn8 = 0.0;
        locals.var_ids_dn9 = 0.0;
        locals.var_ids_dn10 = 0.0;
        locals.var_ids_dn11 = 0.0;
        locals.var_ids_dn14 = 0.0;
        locals.var_ids_rv = 0.0;

        locals.var_ids0 = 0.0;
        locals.var_ids0_dn0 = 0.0;
        locals.var_ids0_dn2 = 0.0;
        locals.var_ids0_dn4 = 0.0;
        locals.var_ids0_dn5 = 0.0;
        locals.var_ids0_dn6 = 0.0;
        locals.var_ids0_dn7 = 0.0;
        locals.var_ids0_dn8 = 0.0;
        locals.var_ids0_dn9 = 0.0;
        locals.var_ids0_dn10 = 0.0;
        locals.var_ids0_dn11 = 0.0;
        locals.var_ids0_dn14 = 0.0;
        locals.var_ids0_rv = 0.0;

        locals.var_dvthscsti = 0.0;
        locals.var_dvthscsti_dn0 = 0.0;
        locals.var_dvthscsti_dn2 = 0.0;
        locals.var_dvthscsti_dn4 = 0.0;
        locals.var_dvthscsti_dn5 = 0.0;
        locals.var_dvthscsti_dn6 = 0.0;
        locals.var_dvthscsti_dn7 = 0.0;
        locals.var_dvthscsti_dn8 = 0.0;
        locals.var_dvthscsti_dn9 = 0.0;
        locals.var_dvthscsti_dn10 = 0.0;
        locals.var_dvthscsti_dn11 = 0.0;
        locals.var_dvthscsti_dn14 = 0.0;
        locals.var_dvthscsti_rv = 0.0;

        locals.var_vgssti = 0.0;
        locals.var_vgssti_dn0 = 0.0;
        locals.var_vgssti_dn2 = 0.0;
        locals.var_vgssti_dn4 = 0.0;
        locals.var_vgssti_dn5 = 0.0;
        locals.var_vgssti_dn6 = 0.0;
        locals.var_vgssti_dn7 = 0.0;
        locals.var_vgssti_dn8 = 0.0;
        locals.var_vgssti_dn9 = 0.0;
        locals.var_vgssti_dn10 = 0.0;
        locals.var_vgssti_dn11 = 0.0;
        locals.var_vgssti_dn14 = 0.0;
        locals.var_vgssti_rv = 0.0;

        locals.var_costi0 = 0.0;
        locals.var_costi0_dn0 = 0.0;
        locals.var_costi0_dn2 = 0.0;
        locals.var_costi0_dn4 = 0.0;
        locals.var_costi0_dn5 = 0.0;
        locals.var_costi0_dn6 = 0.0;
        locals.var_costi0_dn7 = 0.0;
        locals.var_costi0_dn8 = 0.0;
        locals.var_costi0_dn9 = 0.0;
        locals.var_costi0_dn10 = 0.0;
        locals.var_costi0_dn11 = 0.0;
        locals.var_costi0_dn14 = 0.0;
        locals.var_costi0_rv = 0.0;

        locals.var_costi1 = 0.0;
        locals.var_costi1_dn0 = 0.0;
        locals.var_costi1_dn2 = 0.0;
        locals.var_costi1_dn4 = 0.0;
        locals.var_costi1_dn5 = 0.0;
        locals.var_costi1_dn6 = 0.0;
        locals.var_costi1_dn7 = 0.0;
        locals.var_costi1_dn8 = 0.0;
        locals.var_costi1_dn9 = 0.0;
        locals.var_costi1_dn10 = 0.0;
        locals.var_costi1_dn11 = 0.0;
        locals.var_costi1_dn14 = 0.0;
        locals.var_costi1_rv = 0.0;

        locals.var_costi3 = 0.0;
        locals.var_costi3_dn0 = 0.0;
        locals.var_costi3_dn2 = 0.0;
        locals.var_costi3_dn4 = 0.0;
        locals.var_costi3_dn5 = 0.0;
        locals.var_costi3_dn6 = 0.0;
        locals.var_costi3_dn7 = 0.0;
        locals.var_costi3_dn8 = 0.0;
        locals.var_costi3_dn9 = 0.0;
        locals.var_costi3_dn10 = 0.0;
        locals.var_costi3_dn11 = 0.0;
        locals.var_costi3_dn14 = 0.0;
        locals.var_costi3_rv = 0.0;

        locals.var_costi4 = 0.0;
        locals.var_costi4_dn0 = 0.0;
        locals.var_costi4_dn2 = 0.0;
        locals.var_costi4_dn4 = 0.0;
        locals.var_costi4_dn5 = 0.0;
        locals.var_costi4_dn6 = 0.0;
        locals.var_costi4_dn7 = 0.0;
        locals.var_costi4_dn8 = 0.0;
        locals.var_costi4_dn9 = 0.0;
        locals.var_costi4_dn10 = 0.0;
        locals.var_costi4_dn11 = 0.0;
        locals.var_costi4_dn14 = 0.0;
        locals.var_costi4_rv = 0.0;

        locals.var_costi5 = 0.0;
        locals.var_costi5_dn0 = 0.0;
        locals.var_costi5_dn2 = 0.0;
        locals.var_costi5_dn4 = 0.0;
        locals.var_costi5_dn5 = 0.0;
        locals.var_costi5_dn6 = 0.0;
        locals.var_costi5_dn7 = 0.0;
        locals.var_costi5_dn8 = 0.0;
        locals.var_costi5_dn9 = 0.0;
        locals.var_costi5_dn10 = 0.0;
        locals.var_costi5_dn11 = 0.0;
        locals.var_costi5_dn14 = 0.0;
        locals.var_costi5_rv = 0.0;

        locals.var_costi6 = 0.0;
        locals.var_costi6_dn0 = 0.0;
        locals.var_costi6_dn2 = 0.0;
        locals.var_costi6_dn4 = 0.0;
        locals.var_costi6_dn5 = 0.0;
        locals.var_costi6_dn6 = 0.0;
        locals.var_costi6_dn7 = 0.0;
        locals.var_costi6_dn8 = 0.0;
        locals.var_costi6_dn9 = 0.0;
        locals.var_costi6_dn10 = 0.0;
        locals.var_costi6_dn11 = 0.0;
        locals.var_costi6_dn14 = 0.0;
        locals.var_costi6_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        locals: &mut StampLocals,
    ) {
        locals.var_costi7 = 0.0;
        locals.var_costi7_dn0 = 0.0;
        locals.var_costi7_dn2 = 0.0;
        locals.var_costi7_dn4 = 0.0;
        locals.var_costi7_dn5 = 0.0;
        locals.var_costi7_dn6 = 0.0;
        locals.var_costi7_dn7 = 0.0;
        locals.var_costi7_dn8 = 0.0;
        locals.var_costi7_dn9 = 0.0;
        locals.var_costi7_dn10 = 0.0;
        locals.var_costi7_dn11 = 0.0;
        locals.var_costi7_dn14 = 0.0;
        locals.var_costi7_rv = 0.0;

        locals.var_psasti = 0.0;
        locals.var_psasti_dn0 = 0.0;
        locals.var_psasti_dn2 = 0.0;
        locals.var_psasti_dn4 = 0.0;
        locals.var_psasti_dn5 = 0.0;
        locals.var_psasti_dn6 = 0.0;
        locals.var_psasti_dn7 = 0.0;
        locals.var_psasti_dn8 = 0.0;
        locals.var_psasti_dn9 = 0.0;
        locals.var_psasti_dn10 = 0.0;
        locals.var_psasti_dn11 = 0.0;
        locals.var_psasti_dn14 = 0.0;
        locals.var_psasti_rv = 0.0;

        locals.var_psbsti = 0.0;
        locals.var_psbsti_dn0 = 0.0;
        locals.var_psbsti_dn2 = 0.0;
        locals.var_psbsti_dn4 = 0.0;
        locals.var_psbsti_dn5 = 0.0;
        locals.var_psbsti_dn6 = 0.0;
        locals.var_psbsti_dn7 = 0.0;
        locals.var_psbsti_dn8 = 0.0;
        locals.var_psbsti_dn9 = 0.0;
        locals.var_psbsti_dn10 = 0.0;
        locals.var_psbsti_dn11 = 0.0;
        locals.var_psbsti_dn14 = 0.0;
        locals.var_psbsti_rv = 0.0;

        locals.var_psab = 0.0;
        locals.var_psab_dn0 = 0.0;
        locals.var_psab_dn2 = 0.0;
        locals.var_psab_dn4 = 0.0;
        locals.var_psab_dn5 = 0.0;
        locals.var_psab_dn6 = 0.0;
        locals.var_psab_dn7 = 0.0;
        locals.var_psab_dn8 = 0.0;
        locals.var_psab_dn9 = 0.0;
        locals.var_psab_dn10 = 0.0;
        locals.var_psab_dn11 = 0.0;
        locals.var_psab_dn14 = 0.0;
        locals.var_psab_rv = 0.0;

        locals.var_psti = 0.0;
        locals.var_psti_dn0 = 0.0;
        locals.var_psti_dn2 = 0.0;
        locals.var_psti_dn4 = 0.0;
        locals.var_psti_dn5 = 0.0;
        locals.var_psti_dn6 = 0.0;
        locals.var_psti_dn7 = 0.0;
        locals.var_psti_dn8 = 0.0;
        locals.var_psti_dn9 = 0.0;
        locals.var_psti_dn10 = 0.0;
        locals.var_psti_dn11 = 0.0;
        locals.var_psti_dn14 = 0.0;
        locals.var_psti_rv = 0.0;

        locals.var_sq1sti = 0.0;
        locals.var_sq1sti_dn0 = 0.0;
        locals.var_sq1sti_dn2 = 0.0;
        locals.var_sq1sti_dn4 = 0.0;
        locals.var_sq1sti_dn5 = 0.0;
        locals.var_sq1sti_dn6 = 0.0;
        locals.var_sq1sti_dn7 = 0.0;
        locals.var_sq1sti_dn8 = 0.0;
        locals.var_sq1sti_dn9 = 0.0;
        locals.var_sq1sti_dn10 = 0.0;
        locals.var_sq1sti_dn11 = 0.0;
        locals.var_sq1sti_dn14 = 0.0;
        locals.var_sq1sti_rv = 0.0;

        locals.var_sq2sti = 0.0;
        locals.var_sq2sti_dn0 = 0.0;
        locals.var_sq2sti_dn2 = 0.0;
        locals.var_sq2sti_dn4 = 0.0;
        locals.var_sq2sti_dn5 = 0.0;
        locals.var_sq2sti_dn6 = 0.0;
        locals.var_sq2sti_dn7 = 0.0;
        locals.var_sq2sti_dn8 = 0.0;
        locals.var_sq2sti_dn9 = 0.0;
        locals.var_sq2sti_dn10 = 0.0;
        locals.var_sq2sti_dn11 = 0.0;
        locals.var_sq2sti_dn14 = 0.0;
        locals.var_sq2sti_rv = 0.0;

        locals.var_qn0sti = 0.0;
        locals.var_qn0sti_dn0 = 0.0;
        locals.var_qn0sti_dn2 = 0.0;
        locals.var_qn0sti_dn4 = 0.0;
        locals.var_qn0sti_dn5 = 0.0;
        locals.var_qn0sti_dn6 = 0.0;
        locals.var_qn0sti_dn7 = 0.0;
        locals.var_qn0sti_dn8 = 0.0;
        locals.var_qn0sti_dn9 = 0.0;
        locals.var_qn0sti_dn10 = 0.0;
        locals.var_qn0sti_dn11 = 0.0;
        locals.var_qn0sti_dn14 = 0.0;
        locals.var_qn0sti_rv = 0.0;

        locals.var_idssti = 0.0;
        locals.var_idssti_dn0 = 0.0;
        locals.var_idssti_dn2 = 0.0;
        locals.var_idssti_dn4 = 0.0;
        locals.var_idssti_dn5 = 0.0;
        locals.var_idssti_dn6 = 0.0;
        locals.var_idssti_dn7 = 0.0;
        locals.var_idssti_dn8 = 0.0;
        locals.var_idssti_dn9 = 0.0;
        locals.var_idssti_dn10 = 0.0;
        locals.var_idssti_dn11 = 0.0;
        locals.var_idssti_dn14 = 0.0;
        locals.var_idssti_rv = 0.0;

        locals.var_beta = 0.0;
        locals.var_beta_dn0 = 0.0;
        locals.var_beta_dn2 = 0.0;
        locals.var_beta_dn4 = 0.0;
        locals.var_beta_dn5 = 0.0;
        locals.var_beta_dn6 = 0.0;
        locals.var_beta_dn7 = 0.0;
        locals.var_beta_dn8 = 0.0;
        locals.var_beta_dn9 = 0.0;
        locals.var_beta_dn10 = 0.0;
        locals.var_beta_dn11 = 0.0;
        locals.var_beta_dn14 = 0.0;
        locals.var_beta_rv = 0.0;

        locals.var_beta_inv = 0.0;
        locals.var_beta_inv_dn0 = 0.0;
        locals.var_beta_inv_dn2 = 0.0;
        locals.var_beta_inv_dn4 = 0.0;
        locals.var_beta_inv_dn5 = 0.0;
        locals.var_beta_inv_dn6 = 0.0;
        locals.var_beta_inv_dn7 = 0.0;
        locals.var_beta_inv_dn8 = 0.0;
        locals.var_beta_inv_dn9 = 0.0;
        locals.var_beta_inv_dn10 = 0.0;
        locals.var_beta_inv_dn11 = 0.0;
        locals.var_beta_inv_dn14 = 0.0;
        locals.var_beta_inv_rv = 0.0;

        locals.var_beta2 = 0.0;
        locals.var_beta2_dn0 = 0.0;
        locals.var_beta2_dn2 = 0.0;
        locals.var_beta2_dn4 = 0.0;
        locals.var_beta2_dn5 = 0.0;
        locals.var_beta2_dn6 = 0.0;
        locals.var_beta2_dn7 = 0.0;
        locals.var_beta2_dn8 = 0.0;
        locals.var_beta2_dn9 = 0.0;
        locals.var_beta2_dn10 = 0.0;
        locals.var_beta2_dn11 = 0.0;
        locals.var_beta2_dn14 = 0.0;
        locals.var_beta2_rv = 0.0;

        locals.var_pb2 = 0.0;
        locals.var_pb2_dn0 = 0.0;
        locals.var_pb2_dn2 = 0.0;
        locals.var_pb2_dn4 = 0.0;
        locals.var_pb2_dn5 = 0.0;
        locals.var_pb2_dn6 = 0.0;
        locals.var_pb2_dn7 = 0.0;
        locals.var_pb2_dn8 = 0.0;
        locals.var_pb2_dn9 = 0.0;
        locals.var_pb2_dn10 = 0.0;
        locals.var_pb2_dn11 = 0.0;
        locals.var_pb2_dn14 = 0.0;
        locals.var_pb2_rv = 0.0;

        locals.var_pb20 = 0.0;
        locals.var_pb20_dn0 = 0.0;
        locals.var_pb20_dn2 = 0.0;
        locals.var_pb20_dn4 = 0.0;
        locals.var_pb20_dn5 = 0.0;
        locals.var_pb20_dn6 = 0.0;
        locals.var_pb20_dn7 = 0.0;
        locals.var_pb20_dn8 = 0.0;
        locals.var_pb20_dn9 = 0.0;
        locals.var_pb20_dn10 = 0.0;
        locals.var_pb20_dn11 = 0.0;
        locals.var_pb20_dn14 = 0.0;
        locals.var_pb20_rv = 0.0;

        locals.var_pb2c = 0.0;
        locals.var_pb2c_dn0 = 0.0;
        locals.var_pb2c_dn2 = 0.0;
        locals.var_pb2c_dn4 = 0.0;
        locals.var_pb2c_dn5 = 0.0;
        locals.var_pb2c_dn6 = 0.0;
        locals.var_pb2c_dn7 = 0.0;
        locals.var_pb2c_dn8 = 0.0;
        locals.var_pb2c_dn9 = 0.0;
        locals.var_pb2c_dn10 = 0.0;
        locals.var_pb2c_dn11 = 0.0;
        locals.var_pb2c_dn14 = 0.0;
        locals.var_pb2c_rv = 0.0;

        locals.var_vfb = 0.0;
        locals.var_vfb_rv = 0.0;

        locals.var_c_eox = 0.0;
        locals.var_c_eox_rv = 0.0;

        locals.var_leff = 0.0;
        locals.var_leff_rv = 0.0;

        locals.var_weff = 0.0;
        locals.var_weff_rv = 0.0;

        locals.var_weffld_nf = 0.0;
        locals.var_weffld_nf_rv = 0.0;

        locals.var_ldrift0 = 0.0;
        locals.var_ldrift0_rv = 0.0;

        locals.var_q_nsub = 0.0;
        locals.var_q_nsub_dn0 = 0.0;
        locals.var_q_nsub_dn2 = 0.0;
        locals.var_q_nsub_dn4 = 0.0;
        locals.var_q_nsub_dn5 = 0.0;
        locals.var_q_nsub_dn6 = 0.0;
        locals.var_q_nsub_dn7 = 0.0;
        locals.var_q_nsub_dn8 = 0.0;
        locals.var_q_nsub_dn9 = 0.0;
        locals.var_q_nsub_dn10 = 0.0;
        locals.var_q_nsub_dn11 = 0.0;
        locals.var_q_nsub_dn14 = 0.0;
        locals.var_q_nsub_rv = 0.0;

        locals.var_psa = 0.0;
        locals.var_psa_dn0 = 0.0;
        locals.var_psa_dn2 = 0.0;
        locals.var_psa_dn4 = 0.0;
        locals.var_psa_dn5 = 0.0;
        locals.var_psa_dn6 = 0.0;
        locals.var_psa_dn7 = 0.0;
        locals.var_psa_dn8 = 0.0;
        locals.var_psa_dn9 = 0.0;
        locals.var_psa_dn10 = 0.0;
        locals.var_psa_dn11 = 0.0;
        locals.var_psa_dn14 = 0.0;
        locals.var_psa_rv = 0.0;

        locals.var_psdl = 0.0;
        locals.var_psdl_dn0 = 0.0;
        locals.var_psdl_dn2 = 0.0;
        locals.var_psdl_dn4 = 0.0;
        locals.var_psdl_dn5 = 0.0;
        locals.var_psdl_dn6 = 0.0;
        locals.var_psdl_dn7 = 0.0;
        locals.var_psdl_dn8 = 0.0;
        locals.var_psdl_dn9 = 0.0;
        locals.var_psdl_dn10 = 0.0;
        locals.var_psdl_dn11 = 0.0;
        locals.var_psdl_dn14 = 0.0;
        locals.var_psdl_rv = 0.0;

        locals.var_lred = 0.0;
        locals.var_lred_dn0 = 0.0;
        locals.var_lred_dn2 = 0.0;
        locals.var_lred_dn4 = 0.0;
        locals.var_lred_dn5 = 0.0;
        locals.var_lred_dn6 = 0.0;
        locals.var_lred_dn7 = 0.0;
        locals.var_lred_dn8 = 0.0;
        locals.var_lred_dn9 = 0.0;
        locals.var_lred_dn10 = 0.0;
        locals.var_lred_dn11 = 0.0;
        locals.var_lred_dn14 = 0.0;
        locals.var_lred_rv = 0.0;

        locals.var_lch = 0.0;
        locals.var_lch_dn0 = 0.0;
        locals.var_lch_dn2 = 0.0;
        locals.var_lch_dn4 = 0.0;
        locals.var_lch_dn5 = 0.0;
        locals.var_lch_dn6 = 0.0;
        locals.var_lch_dn7 = 0.0;
        locals.var_lch_dn8 = 0.0;
        locals.var_lch_dn9 = 0.0;
        locals.var_lch_dn10 = 0.0;
        locals.var_lch_dn11 = 0.0;
        locals.var_lch_dn14 = 0.0;
        locals.var_lch_rv = 0.0;

        locals.var_wd = 0.0;
        locals.var_wd_dn0 = 0.0;
        locals.var_wd_dn2 = 0.0;
        locals.var_wd_dn4 = 0.0;
        locals.var_wd_dn5 = 0.0;
        locals.var_wd_dn6 = 0.0;
        locals.var_wd_dn7 = 0.0;
        locals.var_wd_dn8 = 0.0;
        locals.var_wd_dn9 = 0.0;
        locals.var_wd_dn10 = 0.0;
        locals.var_wd_dn11 = 0.0;
        locals.var_wd_dn14 = 0.0;
        locals.var_wd_rv = 0.0;

        locals.var_aclm = 0.0;
        locals.var_aclm_rv = 0.0;

        locals.var_vthp = 0.0;
        locals.var_vthp_dn0 = 0.0;
        locals.var_vthp_dn2 = 0.0;
        locals.var_vthp_dn4 = 0.0;
        locals.var_vthp_dn5 = 0.0;
        locals.var_vthp_dn6 = 0.0;
        locals.var_vthp_dn7 = 0.0;
        locals.var_vthp_dn8 = 0.0;
        locals.var_vthp_dn9 = 0.0;
        locals.var_vthp_dn10 = 0.0;
        locals.var_vthp_dn11 = 0.0;
        locals.var_vthp_dn14 = 0.0;
        locals.var_vthp_rv = 0.0;

        locals.var_dvthlp = 0.0;
        locals.var_dvthlp_dn0 = 0.0;
        locals.var_dvthlp_dn2 = 0.0;
        locals.var_dvthlp_dn4 = 0.0;
        locals.var_dvthlp_dn5 = 0.0;
        locals.var_dvthlp_dn6 = 0.0;
        locals.var_dvthlp_dn7 = 0.0;
        locals.var_dvthlp_dn8 = 0.0;
        locals.var_dvthlp_dn9 = 0.0;
        locals.var_dvthlp_dn10 = 0.0;
        locals.var_dvthlp_dn11 = 0.0;
        locals.var_dvthlp_dn14 = 0.0;
        locals.var_dvthlp_rv = 0.0;

        locals.var_bs12 = 0.0;
        locals.var_bs12_dn0 = 0.0;
        locals.var_bs12_dn2 = 0.0;
        locals.var_bs12_dn4 = 0.0;
        locals.var_bs12_dn5 = 0.0;
        locals.var_bs12_dn6 = 0.0;
        locals.var_bs12_dn7 = 0.0;
        locals.var_bs12_dn8 = 0.0;
        locals.var_bs12_dn9 = 0.0;
        locals.var_bs12_dn10 = 0.0;
        locals.var_bs12_dn11 = 0.0;
        locals.var_bs12_dn14 = 0.0;
        locals.var_bs12_rv = 0.0;

        locals.var_qbmm = 0.0;
        locals.var_qbmm_dn0 = 0.0;
        locals.var_qbmm_dn2 = 0.0;
        locals.var_qbmm_dn4 = 0.0;
        locals.var_qbmm_dn5 = 0.0;
        locals.var_qbmm_dn6 = 0.0;
        locals.var_qbmm_dn7 = 0.0;
        locals.var_qbmm_dn8 = 0.0;
        locals.var_qbmm_dn9 = 0.0;
        locals.var_qbmm_dn10 = 0.0;
        locals.var_qbmm_dn11 = 0.0;
        locals.var_qbmm_dn14 = 0.0;
        locals.var_qbmm_rv = 0.0;

        locals.var_dqb = 0.0;
        locals.var_dqb_dn0 = 0.0;
        locals.var_dqb_dn2 = 0.0;
        locals.var_dqb_dn4 = 0.0;
        locals.var_dqb_dn5 = 0.0;
        locals.var_dqb_dn6 = 0.0;
        locals.var_dqb_dn7 = 0.0;
        locals.var_dqb_dn8 = 0.0;
        locals.var_dqb_dn9 = 0.0;
        locals.var_dqb_dn10 = 0.0;
        locals.var_dqb_dn11 = 0.0;
        locals.var_dqb_dn14 = 0.0;
        locals.var_dqb_rv = 0.0;

        locals.var_vdx = 0.0;
        locals.var_vdx_dn0 = 0.0;
        locals.var_vdx_dn2 = 0.0;
        locals.var_vdx_dn4 = 0.0;
        locals.var_vdx_dn5 = 0.0;
        locals.var_vdx_dn6 = 0.0;
        locals.var_vdx_dn7 = 0.0;
        locals.var_vdx_dn8 = 0.0;
        locals.var_vdx_dn9 = 0.0;
        locals.var_vdx_dn10 = 0.0;
        locals.var_vdx_dn11 = 0.0;
        locals.var_vdx_dn14 = 0.0;
        locals.var_vdx_rv = 0.0;

        locals.var_vdx2 = 0.0;
        locals.var_vdx2_dn0 = 0.0;
        locals.var_vdx2_dn2 = 0.0;
        locals.var_vdx2_dn4 = 0.0;
        locals.var_vdx2_dn5 = 0.0;
        locals.var_vdx2_dn6 = 0.0;
        locals.var_vdx2_dn7 = 0.0;
        locals.var_vdx2_dn8 = 0.0;
        locals.var_vdx2_dn9 = 0.0;
        locals.var_vdx2_dn10 = 0.0;
        locals.var_vdx2_dn11 = 0.0;
        locals.var_vdx2_dn14 = 0.0;
        locals.var_vdx2_rv = 0.0;

        locals.var_pbsum = 0.0;
        locals.var_pbsum_dn0 = 0.0;
        locals.var_pbsum_dn2 = 0.0;
        locals.var_pbsum_dn4 = 0.0;
        locals.var_pbsum_dn5 = 0.0;
        locals.var_pbsum_dn6 = 0.0;
        locals.var_pbsum_dn7 = 0.0;
        locals.var_pbsum_dn8 = 0.0;
        locals.var_pbsum_dn9 = 0.0;
        locals.var_pbsum_dn10 = 0.0;
        locals.var_pbsum_dn11 = 0.0;
        locals.var_pbsum_dn14 = 0.0;
        locals.var_pbsum_rv = 0.0;

        locals.var_sqrt_pbsum = 0.0;
        locals.var_sqrt_pbsum_dn0 = 0.0;
        locals.var_sqrt_pbsum_dn2 = 0.0;
        locals.var_sqrt_pbsum_dn4 = 0.0;
        locals.var_sqrt_pbsum_dn5 = 0.0;
        locals.var_sqrt_pbsum_dn6 = 0.0;
        locals.var_sqrt_pbsum_dn7 = 0.0;
        locals.var_sqrt_pbsum_dn8 = 0.0;
        locals.var_sqrt_pbsum_dn9 = 0.0;
        locals.var_sqrt_pbsum_dn10 = 0.0;
        locals.var_sqrt_pbsum_dn11 = 0.0;
        locals.var_sqrt_pbsum_dn14 = 0.0;
        locals.var_sqrt_pbsum_rv = 0.0;

        locals.var_dppg = 0.0;
        locals.var_dppg_dn0 = 0.0;
        locals.var_dppg_dn2 = 0.0;
        locals.var_dppg_dn4 = 0.0;
        locals.var_dppg_dn5 = 0.0;
        locals.var_dppg_dn6 = 0.0;
        locals.var_dppg_dn7 = 0.0;
        locals.var_dppg_dn8 = 0.0;
        locals.var_dppg_dn9 = 0.0;
        locals.var_dppg_dn10 = 0.0;
        locals.var_dppg_dn11 = 0.0;
        locals.var_dppg_dn14 = 0.0;
        locals.var_dppg_rv = 0.0;

        locals.var_dtox = 0.0;
        locals.var_dtox_dn0 = 0.0;
        locals.var_dtox_dn2 = 0.0;
        locals.var_dtox_dn4 = 0.0;
        locals.var_dtox_dn5 = 0.0;
        locals.var_dtox_dn6 = 0.0;
        locals.var_dtox_dn7 = 0.0;
        locals.var_dtox_dn8 = 0.0;
        locals.var_dtox_dn9 = 0.0;
        locals.var_dtox_dn10 = 0.0;
        locals.var_dtox_dn11 = 0.0;
        locals.var_dtox_dn14 = 0.0;
        locals.var_dtox_rv = 0.0;

        locals.var_cox = 0.0;
        locals.var_cox_dn0 = 0.0;
        locals.var_cox_dn2 = 0.0;
        locals.var_cox_dn4 = 0.0;
        locals.var_cox_dn5 = 0.0;
        locals.var_cox_dn6 = 0.0;
        locals.var_cox_dn7 = 0.0;
        locals.var_cox_dn8 = 0.0;
        locals.var_cox_dn9 = 0.0;
        locals.var_cox_dn10 = 0.0;
        locals.var_cox_dn11 = 0.0;
        locals.var_cox_dn14 = 0.0;
        locals.var_cox_rv = 0.0;

        locals.var_cox_inv = 0.0;
        locals.var_cox_inv_dn0 = 0.0;
        locals.var_cox_inv_dn2 = 0.0;
        locals.var_cox_inv_dn4 = 0.0;
        locals.var_cox_inv_dn5 = 0.0;
        locals.var_cox_inv_dn6 = 0.0;
        locals.var_cox_inv_dn7 = 0.0;
        locals.var_cox_inv_dn8 = 0.0;
        locals.var_cox_inv_dn9 = 0.0;
        locals.var_cox_inv_dn10 = 0.0;
        locals.var_cox_inv_dn11 = 0.0;
        locals.var_cox_inv_dn14 = 0.0;
        locals.var_cox_inv_rv = 0.0;

        locals.var_tox0 = 0.0;
        locals.var_tox0_rv = 0.0;

        locals.var_cox0 = 0.0;
        locals.var_cox0_rv = 0.0;

        locals.var_coxb0 = 0.0;
        locals.var_coxb0_rv = 0.0;

        locals.var_cox0_inv = 0.0;
        locals.var_cox0_inv_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        locals: &mut StampLocals,
    ) {
        locals.var_vthq = 0.0;
        locals.var_vthq_dn0 = 0.0;
        locals.var_vthq_dn2 = 0.0;
        locals.var_vthq_dn4 = 0.0;
        locals.var_vthq_dn5 = 0.0;
        locals.var_vthq_dn6 = 0.0;
        locals.var_vthq_dn7 = 0.0;
        locals.var_vthq_dn8 = 0.0;
        locals.var_vthq_dn9 = 0.0;
        locals.var_vthq_dn10 = 0.0;
        locals.var_vthq_dn11 = 0.0;
        locals.var_vthq_dn14 = 0.0;
        locals.var_vthq_rv = 0.0;

        locals.var_psdlz = 0.0;
        locals.var_psdlz_dn0 = 0.0;
        locals.var_psdlz_dn2 = 0.0;
        locals.var_psdlz_dn4 = 0.0;
        locals.var_psdlz_dn5 = 0.0;
        locals.var_psdlz_dn6 = 0.0;
        locals.var_psdlz_dn7 = 0.0;
        locals.var_psdlz_dn8 = 0.0;
        locals.var_psdlz_dn9 = 0.0;
        locals.var_psdlz_dn10 = 0.0;
        locals.var_psdlz_dn11 = 0.0;
        locals.var_psdlz_dn14 = 0.0;
        locals.var_psdlz_rv = 0.0;

        locals.var_egp12 = 0.0;
        locals.var_egp12_dn0 = 0.0;
        locals.var_egp12_dn2 = 0.0;
        locals.var_egp12_dn4 = 0.0;
        locals.var_egp12_dn5 = 0.0;
        locals.var_egp12_dn6 = 0.0;
        locals.var_egp12_dn7 = 0.0;
        locals.var_egp12_dn8 = 0.0;
        locals.var_egp12_dn9 = 0.0;
        locals.var_egp12_dn10 = 0.0;
        locals.var_egp12_dn11 = 0.0;
        locals.var_egp12_dn14 = 0.0;
        locals.var_egp12_rv = 0.0;

        locals.var_egp32 = 0.0;
        locals.var_egp32_dn0 = 0.0;
        locals.var_egp32_dn2 = 0.0;
        locals.var_egp32_dn4 = 0.0;
        locals.var_egp32_dn5 = 0.0;
        locals.var_egp32_dn6 = 0.0;
        locals.var_egp32_dn7 = 0.0;
        locals.var_egp32_dn8 = 0.0;
        locals.var_egp32_dn9 = 0.0;
        locals.var_egp32_dn10 = 0.0;
        locals.var_egp32_dn11 = 0.0;
        locals.var_egp32_dn14 = 0.0;
        locals.var_egp32_rv = 0.0;

        locals.var_e1 = 0.0;
        locals.var_e1_dn0 = 0.0;
        locals.var_e1_dn2 = 0.0;
        locals.var_e1_dn4 = 0.0;
        locals.var_e1_dn5 = 0.0;
        locals.var_e1_dn6 = 0.0;
        locals.var_e1_dn7 = 0.0;
        locals.var_e1_dn8 = 0.0;
        locals.var_e1_dn9 = 0.0;
        locals.var_e1_dn10 = 0.0;
        locals.var_e1_dn11 = 0.0;
        locals.var_e1_dn14 = 0.0;
        locals.var_e1_rv = 0.0;

        locals.var_etun = 0.0;
        locals.var_etun_dn0 = 0.0;
        locals.var_etun_dn2 = 0.0;
        locals.var_etun_dn4 = 0.0;
        locals.var_etun_dn5 = 0.0;
        locals.var_etun_dn6 = 0.0;
        locals.var_etun_dn7 = 0.0;
        locals.var_etun_dn8 = 0.0;
        locals.var_etun_dn9 = 0.0;
        locals.var_etun_dn10 = 0.0;
        locals.var_etun_dn11 = 0.0;
        locals.var_etun_dn14 = 0.0;
        locals.var_etun_rv = 0.0;

        locals.var_vdsp = 0.0;
        locals.var_vdsp_dn0 = 0.0;
        locals.var_vdsp_dn2 = 0.0;
        locals.var_vdsp_dn4 = 0.0;
        locals.var_vdsp_dn5 = 0.0;
        locals.var_vdsp_dn6 = 0.0;
        locals.var_vdsp_dn7 = 0.0;
        locals.var_vdsp_dn8 = 0.0;
        locals.var_vdsp_dn9 = 0.0;
        locals.var_vdsp_dn10 = 0.0;
        locals.var_vdsp_dn11 = 0.0;
        locals.var_vdsp_dn14 = 0.0;
        locals.var_vdsp_rv = 0.0;

        locals.var_egidl = 0.0;
        locals.var_egidl_dn0 = 0.0;
        locals.var_egidl_dn2 = 0.0;
        locals.var_egidl_dn4 = 0.0;
        locals.var_egidl_dn5 = 0.0;
        locals.var_egidl_dn6 = 0.0;
        locals.var_egidl_dn7 = 0.0;
        locals.var_egidl_dn8 = 0.0;
        locals.var_egidl_dn9 = 0.0;
        locals.var_egidl_dn10 = 0.0;
        locals.var_egidl_dn11 = 0.0;
        locals.var_egidl_dn14 = 0.0;
        locals.var_egidl_rv = 0.0;

        locals.var_egisl = 0.0;
        locals.var_egisl_dn0 = 0.0;
        locals.var_egisl_dn2 = 0.0;
        locals.var_egisl_dn4 = 0.0;
        locals.var_egisl_dn5 = 0.0;
        locals.var_egisl_dn6 = 0.0;
        locals.var_egisl_dn7 = 0.0;
        locals.var_egisl_dn8 = 0.0;
        locals.var_egisl_dn9 = 0.0;
        locals.var_egisl_dn10 = 0.0;
        locals.var_egisl_dn11 = 0.0;
        locals.var_egisl_dn14 = 0.0;
        locals.var_egisl_rv = 0.0;

        locals.var_vdb = 0.0;
        locals.var_vdb_dn0 = 0.0;
        locals.var_vdb_dn2 = 0.0;
        locals.var_vdb_dn4 = 0.0;
        locals.var_vdb_dn5 = 0.0;
        locals.var_vdb_dn6 = 0.0;
        locals.var_vdb_dn7 = 0.0;
        locals.var_vdb_dn8 = 0.0;
        locals.var_vdb_dn9 = 0.0;
        locals.var_vdb_dn10 = 0.0;
        locals.var_vdb_dn11 = 0.0;
        locals.var_vdb_dn14 = 0.0;
        locals.var_vdb_rv = 0.0;

        locals.var_vsb = 0.0;
        locals.var_vsb_dn6 = 0.0;
        locals.var_vsb_dn8 = 0.0;
        locals.var_vsb_dn9 = 0.0;
        locals.var_vsb_rv = 0.0;

        locals.var_fd2 = 0.0;
        locals.var_fd2_dn0 = 0.0;
        locals.var_fd2_dn2 = 0.0;
        locals.var_fd2_dn4 = 0.0;
        locals.var_fd2_dn5 = 0.0;
        locals.var_fd2_dn6 = 0.0;
        locals.var_fd2_dn7 = 0.0;
        locals.var_fd2_dn8 = 0.0;
        locals.var_fd2_dn9 = 0.0;
        locals.var_fd2_dn10 = 0.0;
        locals.var_fd2_dn11 = 0.0;
        locals.var_fd2_dn14 = 0.0;
        locals.var_fd2_rv = 0.0;

        locals.var_fmdvds = 0.0;
        locals.var_fmdvds_dn0 = 0.0;
        locals.var_fmdvds_dn2 = 0.0;
        locals.var_fmdvds_dn4 = 0.0;
        locals.var_fmdvds_dn5 = 0.0;
        locals.var_fmdvds_dn6 = 0.0;
        locals.var_fmdvds_dn7 = 0.0;
        locals.var_fmdvds_dn8 = 0.0;
        locals.var_fmdvds_dn9 = 0.0;
        locals.var_fmdvds_dn10 = 0.0;
        locals.var_fmdvds_dn11 = 0.0;
        locals.var_fmdvds_dn14 = 0.0;
        locals.var_fmdvds_rv = 0.0;

        locals.var_cnst0 = 0.0;
        locals.var_cnst0_dn0 = 0.0;
        locals.var_cnst0_dn2 = 0.0;
        locals.var_cnst0_dn4 = 0.0;
        locals.var_cnst0_dn5 = 0.0;
        locals.var_cnst0_dn6 = 0.0;
        locals.var_cnst0_dn7 = 0.0;
        locals.var_cnst0_dn8 = 0.0;
        locals.var_cnst0_dn9 = 0.0;
        locals.var_cnst0_dn10 = 0.0;
        locals.var_cnst0_dn11 = 0.0;
        locals.var_cnst0_dn14 = 0.0;
        locals.var_cnst0_rv = 0.0;

        locals.var_cnst1 = 0.0;
        locals.var_cnst1_dn0 = 0.0;
        locals.var_cnst1_dn2 = 0.0;
        locals.var_cnst1_dn4 = 0.0;
        locals.var_cnst1_dn5 = 0.0;
        locals.var_cnst1_dn6 = 0.0;
        locals.var_cnst1_dn7 = 0.0;
        locals.var_cnst1_dn8 = 0.0;
        locals.var_cnst1_dn9 = 0.0;
        locals.var_cnst1_dn10 = 0.0;
        locals.var_cnst1_dn11 = 0.0;
        locals.var_cnst1_dn14 = 0.0;
        locals.var_cnst1_rv = 0.0;

        locals.var_cnstcoxi = 0.0;
        locals.var_cnstcoxi_dn0 = 0.0;
        locals.var_cnstcoxi_dn2 = 0.0;
        locals.var_cnstcoxi_dn4 = 0.0;
        locals.var_cnstcoxi_dn5 = 0.0;
        locals.var_cnstcoxi_dn6 = 0.0;
        locals.var_cnstcoxi_dn7 = 0.0;
        locals.var_cnstcoxi_dn8 = 0.0;
        locals.var_cnstcoxi_dn9 = 0.0;
        locals.var_cnstcoxi_dn10 = 0.0;
        locals.var_cnstcoxi_dn11 = 0.0;
        locals.var_cnstcoxi_dn14 = 0.0;
        locals.var_cnstcoxi_rv = 0.0;

        locals.var_fac1 = 0.0;
        locals.var_fac1_dn0 = 0.0;
        locals.var_fac1_dn2 = 0.0;
        locals.var_fac1_dn4 = 0.0;
        locals.var_fac1_dn5 = 0.0;
        locals.var_fac1_dn6 = 0.0;
        locals.var_fac1_dn7 = 0.0;
        locals.var_fac1_dn8 = 0.0;
        locals.var_fac1_dn9 = 0.0;
        locals.var_fac1_dn10 = 0.0;
        locals.var_fac1_dn11 = 0.0;
        locals.var_fac1_dn14 = 0.0;
        locals.var_fac1_rv = 0.0;

        locals.var_fac1p2 = 0.0;
        locals.var_fac1p2_dn0 = 0.0;
        locals.var_fac1p2_dn2 = 0.0;
        locals.var_fac1p2_dn4 = 0.0;
        locals.var_fac1p2_dn5 = 0.0;
        locals.var_fac1p2_dn6 = 0.0;
        locals.var_fac1p2_dn7 = 0.0;
        locals.var_fac1p2_dn8 = 0.0;
        locals.var_fac1p2_dn9 = 0.0;
        locals.var_fac1p2_dn10 = 0.0;
        locals.var_fac1p2_dn11 = 0.0;
        locals.var_fac1p2_dn14 = 0.0;
        locals.var_fac1p2_rv = 0.0;

        locals.var_fs01 = 0.0;
        locals.var_fs01_dn0 = 0.0;
        locals.var_fs01_dn2 = 0.0;
        locals.var_fs01_dn4 = 0.0;
        locals.var_fs01_dn5 = 0.0;
        locals.var_fs01_dn6 = 0.0;
        locals.var_fs01_dn7 = 0.0;
        locals.var_fs01_dn8 = 0.0;
        locals.var_fs01_dn9 = 0.0;
        locals.var_fs01_dn10 = 0.0;
        locals.var_fs01_dn11 = 0.0;
        locals.var_fs01_dn14 = 0.0;
        locals.var_fs01_rv = 0.0;

        locals.var_fs01_dps0 = 0.0;
        locals.var_fs01_dps0_dn0 = 0.0;
        locals.var_fs01_dps0_dn2 = 0.0;
        locals.var_fs01_dps0_dn4 = 0.0;
        locals.var_fs01_dps0_dn5 = 0.0;
        locals.var_fs01_dps0_dn6 = 0.0;
        locals.var_fs01_dps0_dn7 = 0.0;
        locals.var_fs01_dps0_dn8 = 0.0;
        locals.var_fs01_dps0_dn9 = 0.0;
        locals.var_fs01_dps0_dn10 = 0.0;
        locals.var_fs01_dps0_dn11 = 0.0;
        locals.var_fs01_dps0_dn14 = 0.0;
        locals.var_fs01_dps0_rv = 0.0;

        locals.var_fs02 = 0.0;
        locals.var_fs02_dn0 = 0.0;
        locals.var_fs02_dn2 = 0.0;
        locals.var_fs02_dn4 = 0.0;
        locals.var_fs02_dn5 = 0.0;
        locals.var_fs02_dn6 = 0.0;
        locals.var_fs02_dn7 = 0.0;
        locals.var_fs02_dn8 = 0.0;
        locals.var_fs02_dn9 = 0.0;
        locals.var_fs02_dn10 = 0.0;
        locals.var_fs02_dn11 = 0.0;
        locals.var_fs02_dn14 = 0.0;
        locals.var_fs02_rv = 0.0;

        locals.var_fs02_dps0 = 0.0;
        locals.var_fs02_dps0_dn0 = 0.0;
        locals.var_fs02_dps0_dn2 = 0.0;
        locals.var_fs02_dps0_dn4 = 0.0;
        locals.var_fs02_dps0_dn5 = 0.0;
        locals.var_fs02_dps0_dn6 = 0.0;
        locals.var_fs02_dps0_dn7 = 0.0;
        locals.var_fs02_dps0_dn8 = 0.0;
        locals.var_fs02_dps0_dn9 = 0.0;
        locals.var_fs02_dps0_dn10 = 0.0;
        locals.var_fs02_dps0_dn11 = 0.0;
        locals.var_fs02_dps0_dn14 = 0.0;
        locals.var_fs02_dps0_rv = 0.0;

        locals.var_fsl1 = 0.0;
        locals.var_fsl1_dn0 = 0.0;
        locals.var_fsl1_dn2 = 0.0;
        locals.var_fsl1_dn4 = 0.0;
        locals.var_fsl1_dn5 = 0.0;
        locals.var_fsl1_dn6 = 0.0;
        locals.var_fsl1_dn7 = 0.0;
        locals.var_fsl1_dn8 = 0.0;
        locals.var_fsl1_dn9 = 0.0;
        locals.var_fsl1_dn10 = 0.0;
        locals.var_fsl1_dn11 = 0.0;
        locals.var_fsl1_dn14 = 0.0;
        locals.var_fsl1_rv = 0.0;

        locals.var_fsl1_dpsl = 0.0;
        locals.var_fsl1_dpsl_dn0 = 0.0;
        locals.var_fsl1_dpsl_dn2 = 0.0;
        locals.var_fsl1_dpsl_dn4 = 0.0;
        locals.var_fsl1_dpsl_dn5 = 0.0;
        locals.var_fsl1_dpsl_dn6 = 0.0;
        locals.var_fsl1_dpsl_dn7 = 0.0;
        locals.var_fsl1_dpsl_dn8 = 0.0;
        locals.var_fsl1_dpsl_dn9 = 0.0;
        locals.var_fsl1_dpsl_dn10 = 0.0;
        locals.var_fsl1_dpsl_dn11 = 0.0;
        locals.var_fsl1_dpsl_dn14 = 0.0;
        locals.var_fsl1_dpsl_rv = 0.0;

        locals.var_fsl2 = 0.0;
        locals.var_fsl2_dn0 = 0.0;
        locals.var_fsl2_dn2 = 0.0;
        locals.var_fsl2_dn4 = 0.0;
        locals.var_fsl2_dn5 = 0.0;
        locals.var_fsl2_dn6 = 0.0;
        locals.var_fsl2_dn7 = 0.0;
        locals.var_fsl2_dn8 = 0.0;
        locals.var_fsl2_dn9 = 0.0;
        locals.var_fsl2_dn10 = 0.0;
        locals.var_fsl2_dn11 = 0.0;
        locals.var_fsl2_dn14 = 0.0;
        locals.var_fsl2_rv = 0.0;

        locals.var_fsl2_dpsl = 0.0;
        locals.var_fsl2_dpsl_dn0 = 0.0;
        locals.var_fsl2_dpsl_dn2 = 0.0;
        locals.var_fsl2_dpsl_dn4 = 0.0;
        locals.var_fsl2_dpsl_dn5 = 0.0;
        locals.var_fsl2_dpsl_dn6 = 0.0;
        locals.var_fsl2_dpsl_dn7 = 0.0;
        locals.var_fsl2_dpsl_dn8 = 0.0;
        locals.var_fsl2_dpsl_dn9 = 0.0;
        locals.var_fsl2_dpsl_dn10 = 0.0;
        locals.var_fsl2_dpsl_dn11 = 0.0;
        locals.var_fsl2_dpsl_dn14 = 0.0;
        locals.var_fsl2_dpsl_rv = 0.0;

        locals.var_cfs1 = 0.0;
        locals.var_cfs1_dn0 = 0.0;
        locals.var_cfs1_dn2 = 0.0;
        locals.var_cfs1_dn4 = 0.0;
        locals.var_cfs1_dn5 = 0.0;
        locals.var_cfs1_dn6 = 0.0;
        locals.var_cfs1_dn7 = 0.0;
        locals.var_cfs1_dn8 = 0.0;
        locals.var_cfs1_dn9 = 0.0;
        locals.var_cfs1_dn10 = 0.0;
        locals.var_cfs1_dn11 = 0.0;
        locals.var_cfs1_dn14 = 0.0;
        locals.var_cfs1_rv = 0.0;

        locals.var_fb = 0.0;
        locals.var_fb_dn0 = 0.0;
        locals.var_fb_dn2 = 0.0;
        locals.var_fb_dn4 = 0.0;
        locals.var_fb_dn5 = 0.0;
        locals.var_fb_dn6 = 0.0;
        locals.var_fb_dn7 = 0.0;
        locals.var_fb_dn8 = 0.0;
        locals.var_fb_dn9 = 0.0;
        locals.var_fb_dn10 = 0.0;
        locals.var_fb_dn11 = 0.0;
        locals.var_fb_dn14 = 0.0;
        locals.var_fb_rv = 0.0;

        locals.var_fb_dchi = 0.0;
        locals.var_fb_dchi_dn0 = 0.0;
        locals.var_fb_dchi_dn2 = 0.0;
        locals.var_fb_dchi_dn4 = 0.0;
        locals.var_fb_dchi_dn5 = 0.0;
        locals.var_fb_dchi_dn6 = 0.0;
        locals.var_fb_dchi_dn7 = 0.0;
        locals.var_fb_dchi_dn8 = 0.0;
        locals.var_fb_dchi_dn9 = 0.0;
        locals.var_fb_dchi_dn10 = 0.0;
        locals.var_fb_dchi_dn11 = 0.0;
        locals.var_fb_dchi_dn14 = 0.0;
        locals.var_fb_dchi_rv = 0.0;

        locals.var_fi = 0.0;
        locals.var_fi_dn0 = 0.0;
        locals.var_fi_dn2 = 0.0;
        locals.var_fi_dn4 = 0.0;
        locals.var_fi_dn5 = 0.0;
        locals.var_fi_dn6 = 0.0;
        locals.var_fi_dn7 = 0.0;
        locals.var_fi_dn8 = 0.0;
        locals.var_fi_dn9 = 0.0;
        locals.var_fi_dn10 = 0.0;
        locals.var_fi_dn11 = 0.0;
        locals.var_fi_dn14 = 0.0;
        locals.var_fi_rv = 0.0;

        locals.var_fi_dchi = 0.0;
        locals.var_fi_dchi_dn0 = 0.0;
        locals.var_fi_dchi_dn2 = 0.0;
        locals.var_fi_dchi_dn4 = 0.0;
        locals.var_fi_dchi_dn5 = 0.0;
        locals.var_fi_dchi_dn6 = 0.0;
        locals.var_fi_dchi_dn7 = 0.0;
        locals.var_fi_dchi_dn8 = 0.0;
        locals.var_fi_dchi_dn9 = 0.0;
        locals.var_fi_dchi_dn10 = 0.0;
        locals.var_fi_dchi_dn11 = 0.0;
        locals.var_fi_dchi_dn14 = 0.0;
        locals.var_fi_dchi_rv = 0.0;

        locals.var_exp_chi = 0.0;
        locals.var_exp_chi_dn0 = 0.0;
        locals.var_exp_chi_dn2 = 0.0;
        locals.var_exp_chi_dn4 = 0.0;
        locals.var_exp_chi_dn5 = 0.0;
        locals.var_exp_chi_dn6 = 0.0;
        locals.var_exp_chi_dn7 = 0.0;
        locals.var_exp_chi_dn8 = 0.0;
        locals.var_exp_chi_dn9 = 0.0;
        locals.var_exp_chi_dn10 = 0.0;
        locals.var_exp_chi_dn11 = 0.0;
        locals.var_exp_chi_dn14 = 0.0;
        locals.var_exp_chi_rv = 0.0;

        locals.var_exp_rho = 0.0;
        locals.var_exp_rho_dn0 = 0.0;
        locals.var_exp_rho_dn2 = 0.0;
        locals.var_exp_rho_dn4 = 0.0;
        locals.var_exp_rho_dn5 = 0.0;
        locals.var_exp_rho_dn6 = 0.0;
        locals.var_exp_rho_dn7 = 0.0;
        locals.var_exp_rho_dn8 = 0.0;
        locals.var_exp_rho_dn9 = 0.0;
        locals.var_exp_rho_dn10 = 0.0;
        locals.var_exp_rho_dn11 = 0.0;
        locals.var_exp_rho_dn14 = 0.0;
        locals.var_exp_rho_rv = 0.0;

        locals.var_exp_bvbs = 0.0;
        locals.var_exp_bvbs_dn0 = 0.0;
        locals.var_exp_bvbs_dn2 = 0.0;
        locals.var_exp_bvbs_dn4 = 0.0;
        locals.var_exp_bvbs_dn5 = 0.0;
        locals.var_exp_bvbs_dn6 = 0.0;
        locals.var_exp_bvbs_dn7 = 0.0;
        locals.var_exp_bvbs_dn8 = 0.0;
        locals.var_exp_bvbs_dn9 = 0.0;
        locals.var_exp_bvbs_dn10 = 0.0;
        locals.var_exp_bvbs_dn11 = 0.0;
        locals.var_exp_bvbs_dn14 = 0.0;
        locals.var_exp_bvbs_rv = 0.0;

        locals.var_exp_bvbsvds = 0.0;
        locals.var_exp_bvbsvds_dn0 = 0.0;
        locals.var_exp_bvbsvds_dn2 = 0.0;
        locals.var_exp_bvbsvds_dn4 = 0.0;
        locals.var_exp_bvbsvds_dn5 = 0.0;
        locals.var_exp_bvbsvds_dn6 = 0.0;
        locals.var_exp_bvbsvds_dn7 = 0.0;
        locals.var_exp_bvbsvds_dn8 = 0.0;
        locals.var_exp_bvbsvds_dn9 = 0.0;
        locals.var_exp_bvbsvds_dn10 = 0.0;
        locals.var_exp_bvbsvds_dn11 = 0.0;
        locals.var_exp_bvbsvds_dn14 = 0.0;
        locals.var_exp_bvbsvds_rv = 0.0;

        locals.var_exp_bps0 = 0.0;
        locals.var_exp_bps0_dn0 = 0.0;
        locals.var_exp_bps0_dn2 = 0.0;
        locals.var_exp_bps0_dn4 = 0.0;
        locals.var_exp_bps0_dn5 = 0.0;
        locals.var_exp_bps0_dn6 = 0.0;
        locals.var_exp_bps0_dn7 = 0.0;
        locals.var_exp_bps0_dn8 = 0.0;
        locals.var_exp_bps0_dn9 = 0.0;
        locals.var_exp_bps0_dn10 = 0.0;
        locals.var_exp_bps0_dn11 = 0.0;
        locals.var_exp_bps0_dn14 = 0.0;
        locals.var_exp_bps0_rv = 0.0;

        locals.var_fs0 = 0.0;
        locals.var_fs0_dn0 = 0.0;
        locals.var_fs0_dn2 = 0.0;
        locals.var_fs0_dn4 = 0.0;
        locals.var_fs0_dn5 = 0.0;
        locals.var_fs0_dn6 = 0.0;
        locals.var_fs0_dn7 = 0.0;
        locals.var_fs0_dn8 = 0.0;
        locals.var_fs0_dn9 = 0.0;
        locals.var_fs0_dn10 = 0.0;
        locals.var_fs0_dn11 = 0.0;
        locals.var_fs0_dn14 = 0.0;
        locals.var_fs0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        locals: &mut StampLocals,
    ) {
        locals.var_fs0_dps0 = 0.0;
        locals.var_fs0_dps0_dn0 = 0.0;
        locals.var_fs0_dps0_dn2 = 0.0;
        locals.var_fs0_dps0_dn4 = 0.0;
        locals.var_fs0_dps0_dn5 = 0.0;
        locals.var_fs0_dps0_dn6 = 0.0;
        locals.var_fs0_dps0_dn7 = 0.0;
        locals.var_fs0_dps0_dn8 = 0.0;
        locals.var_fs0_dps0_dn9 = 0.0;
        locals.var_fs0_dps0_dn10 = 0.0;
        locals.var_fs0_dps0_dn11 = 0.0;
        locals.var_fs0_dps0_dn14 = 0.0;
        locals.var_fs0_dps0_rv = 0.0;

        locals.var_fsl = 0.0;
        locals.var_fsl_dn0 = 0.0;
        locals.var_fsl_dn2 = 0.0;
        locals.var_fsl_dn4 = 0.0;
        locals.var_fsl_dn5 = 0.0;
        locals.var_fsl_dn6 = 0.0;
        locals.var_fsl_dn7 = 0.0;
        locals.var_fsl_dn8 = 0.0;
        locals.var_fsl_dn9 = 0.0;
        locals.var_fsl_dn10 = 0.0;
        locals.var_fsl_dn11 = 0.0;
        locals.var_fsl_dn14 = 0.0;
        locals.var_fsl_rv = 0.0;

        locals.var_fsl_dpsl = 0.0;
        locals.var_fsl_dpsl_dn0 = 0.0;
        locals.var_fsl_dpsl_dn2 = 0.0;
        locals.var_fsl_dpsl_dn4 = 0.0;
        locals.var_fsl_dpsl_dn5 = 0.0;
        locals.var_fsl_dpsl_dn6 = 0.0;
        locals.var_fsl_dpsl_dn7 = 0.0;
        locals.var_fsl_dpsl_dn8 = 0.0;
        locals.var_fsl_dpsl_dn9 = 0.0;
        locals.var_fsl_dpsl_dn10 = 0.0;
        locals.var_fsl_dpsl_dn11 = 0.0;
        locals.var_fsl_dpsl_dn14 = 0.0;
        locals.var_fsl_dpsl_rv = 0.0;

        locals.var_dps0 = 0.0;
        locals.var_dps0_dn0 = 0.0;
        locals.var_dps0_dn2 = 0.0;
        locals.var_dps0_dn4 = 0.0;
        locals.var_dps0_dn5 = 0.0;
        locals.var_dps0_dn6 = 0.0;
        locals.var_dps0_dn7 = 0.0;
        locals.var_dps0_dn8 = 0.0;
        locals.var_dps0_dn9 = 0.0;
        locals.var_dps0_dn10 = 0.0;
        locals.var_dps0_dn11 = 0.0;
        locals.var_dps0_dn14 = 0.0;
        locals.var_dps0_rv = 0.0;

        locals.var_dpsl = 0.0;
        locals.var_dpsl_dn0 = 0.0;
        locals.var_dpsl_dn2 = 0.0;
        locals.var_dpsl_dn4 = 0.0;
        locals.var_dpsl_dn5 = 0.0;
        locals.var_dpsl_dn6 = 0.0;
        locals.var_dpsl_dn7 = 0.0;
        locals.var_dpsl_dn8 = 0.0;
        locals.var_dpsl_dn9 = 0.0;
        locals.var_dpsl_dn10 = 0.0;
        locals.var_dpsl_dn11 = 0.0;
        locals.var_dpsl_dn14 = 0.0;
        locals.var_dpsl_rv = 0.0;

        locals.var_qn0 = 0.0;
        locals.var_qn0_dn0 = 0.0;
        locals.var_qn0_dn2 = 0.0;
        locals.var_qn0_dn4 = 0.0;
        locals.var_qn0_dn5 = 0.0;
        locals.var_qn0_dn6 = 0.0;
        locals.var_qn0_dn7 = 0.0;
        locals.var_qn0_dn8 = 0.0;
        locals.var_qn0_dn9 = 0.0;
        locals.var_qn0_dn10 = 0.0;
        locals.var_qn0_dn11 = 0.0;
        locals.var_qn0_dn14 = 0.0;
        locals.var_qn0_rv = 0.0;

        locals.var_qb0 = 0.0;
        locals.var_qb0_dn0 = 0.0;
        locals.var_qb0_dn2 = 0.0;
        locals.var_qb0_dn4 = 0.0;
        locals.var_qb0_dn5 = 0.0;
        locals.var_qb0_dn6 = 0.0;
        locals.var_qb0_dn7 = 0.0;
        locals.var_qb0_dn8 = 0.0;
        locals.var_qb0_dn9 = 0.0;
        locals.var_qb0_dn10 = 0.0;
        locals.var_qb0_dn11 = 0.0;
        locals.var_qb0_dn14 = 0.0;
        locals.var_qb0_rv = 0.0;

        locals.var_qbnm = 0.0;
        locals.var_qbnm_dn0 = 0.0;
        locals.var_qbnm_dn2 = 0.0;
        locals.var_qbnm_dn4 = 0.0;
        locals.var_qbnm_dn5 = 0.0;
        locals.var_qbnm_dn6 = 0.0;
        locals.var_qbnm_dn7 = 0.0;
        locals.var_qbnm_dn8 = 0.0;
        locals.var_qbnm_dn9 = 0.0;
        locals.var_qbnm_dn10 = 0.0;
        locals.var_qbnm_dn11 = 0.0;
        locals.var_qbnm_dn14 = 0.0;
        locals.var_qbnm_rv = 0.0;

        locals.var_dtpds = 0.0;
        locals.var_dtpds_dn0 = 0.0;
        locals.var_dtpds_dn2 = 0.0;
        locals.var_dtpds_dn4 = 0.0;
        locals.var_dtpds_dn5 = 0.0;
        locals.var_dtpds_dn6 = 0.0;
        locals.var_dtpds_dn7 = 0.0;
        locals.var_dtpds_dn8 = 0.0;
        locals.var_dtpds_dn9 = 0.0;
        locals.var_dtpds_dn10 = 0.0;
        locals.var_dtpds_dn11 = 0.0;
        locals.var_dtpds_dn14 = 0.0;
        locals.var_dtpds_rv = 0.0;

        locals.var_qinm = 0.0;
        locals.var_qinm_dn0 = 0.0;
        locals.var_qinm_dn2 = 0.0;
        locals.var_qinm_dn4 = 0.0;
        locals.var_qinm_dn5 = 0.0;
        locals.var_qinm_dn6 = 0.0;
        locals.var_qinm_dn7 = 0.0;
        locals.var_qinm_dn8 = 0.0;
        locals.var_qinm_dn9 = 0.0;
        locals.var_qinm_dn10 = 0.0;
        locals.var_qinm_dn11 = 0.0;
        locals.var_qinm_dn14 = 0.0;
        locals.var_qinm_rv = 0.0;

        locals.var_qidn = 0.0;
        locals.var_qidn_dn0 = 0.0;
        locals.var_qidn_dn2 = 0.0;
        locals.var_qidn_dn4 = 0.0;
        locals.var_qidn_dn5 = 0.0;
        locals.var_qidn_dn6 = 0.0;
        locals.var_qidn_dn7 = 0.0;
        locals.var_qidn_dn8 = 0.0;
        locals.var_qidn_dn9 = 0.0;
        locals.var_qidn_dn10 = 0.0;
        locals.var_qidn_dn11 = 0.0;
        locals.var_qidn_dn14 = 0.0;
        locals.var_qidn_rv = 0.0;

        locals.var_qdnm = 0.0;
        locals.var_qdnm_dn0 = 0.0;
        locals.var_qdnm_dn2 = 0.0;
        locals.var_qdnm_dn4 = 0.0;
        locals.var_qdnm_dn5 = 0.0;
        locals.var_qdnm_dn6 = 0.0;
        locals.var_qdnm_dn7 = 0.0;
        locals.var_qdnm_dn8 = 0.0;
        locals.var_qdnm_dn9 = 0.0;
        locals.var_qdnm_dn10 = 0.0;
        locals.var_qdnm_dn11 = 0.0;
        locals.var_qdnm_dn14 = 0.0;
        locals.var_qdnm_rv = 0.0;

        locals.var_qddn = 0.0;
        locals.var_qddn_dn0 = 0.0;
        locals.var_qddn_dn2 = 0.0;
        locals.var_qddn_dn4 = 0.0;
        locals.var_qddn_dn5 = 0.0;
        locals.var_qddn_dn6 = 0.0;
        locals.var_qddn_dn7 = 0.0;
        locals.var_qddn_dn8 = 0.0;
        locals.var_qddn_dn9 = 0.0;
        locals.var_qddn_dn10 = 0.0;
        locals.var_qddn_dn11 = 0.0;
        locals.var_qddn_dn14 = 0.0;
        locals.var_qddn_rv = 0.0;

        locals.var_quot = 0.0;
        locals.var_quot_dn0 = 0.0;
        locals.var_quot_dn2 = 0.0;
        locals.var_quot_dn4 = 0.0;
        locals.var_quot_dn5 = 0.0;
        locals.var_quot_dn6 = 0.0;
        locals.var_quot_dn7 = 0.0;
        locals.var_quot_dn8 = 0.0;
        locals.var_quot_dn9 = 0.0;
        locals.var_quot_dn10 = 0.0;
        locals.var_quot_dn11 = 0.0;
        locals.var_quot_dn14 = 0.0;
        locals.var_quot_rv = 0.0;

        locals.var_qdrat = 0.5;
        locals.var_qdrat_dn0 = 0.0;
        locals.var_qdrat_dn2 = 0.0;
        locals.var_qdrat_dn4 = 0.0;
        locals.var_qdrat_dn5 = 0.0;
        locals.var_qdrat_dn6 = 0.0;
        locals.var_qdrat_dn7 = 0.0;
        locals.var_qdrat_dn8 = 0.0;
        locals.var_qdrat_dn9 = 0.0;
        locals.var_qdrat_dn10 = 0.0;
        locals.var_qdrat_dn11 = 0.0;
        locals.var_qdrat_dn14 = 0.0;
        locals.var_qdrat_rv = 0.0;

        locals.var_idd = 0.0;
        locals.var_idd_dn0 = 0.0;
        locals.var_idd_dn2 = 0.0;
        locals.var_idd_dn4 = 0.0;
        locals.var_idd_dn5 = 0.0;
        locals.var_idd_dn6 = 0.0;
        locals.var_idd_dn7 = 0.0;
        locals.var_idd_dn8 = 0.0;
        locals.var_idd_dn9 = 0.0;
        locals.var_idd_dn10 = 0.0;
        locals.var_idd_dn11 = 0.0;
        locals.var_idd_dn14 = 0.0;
        locals.var_idd_rv = 0.0;

        locals.var_idd1 = 0.0;
        locals.var_idd1_dn0 = 0.0;
        locals.var_idd1_dn2 = 0.0;
        locals.var_idd1_dn4 = 0.0;
        locals.var_idd1_dn5 = 0.0;
        locals.var_idd1_dn6 = 0.0;
        locals.var_idd1_dn7 = 0.0;
        locals.var_idd1_dn8 = 0.0;
        locals.var_idd1_dn9 = 0.0;
        locals.var_idd1_dn10 = 0.0;
        locals.var_idd1_dn11 = 0.0;
        locals.var_idd1_dn14 = 0.0;
        locals.var_idd1_rv = 0.0;

        locals.var_fdd = 0.0;
        locals.var_fdd_dn0 = 0.0;
        locals.var_fdd_dn2 = 0.0;
        locals.var_fdd_dn4 = 0.0;
        locals.var_fdd_dn5 = 0.0;
        locals.var_fdd_dn6 = 0.0;
        locals.var_fdd_dn7 = 0.0;
        locals.var_fdd_dn8 = 0.0;
        locals.var_fdd_dn9 = 0.0;
        locals.var_fdd_dn10 = 0.0;
        locals.var_fdd_dn11 = 0.0;
        locals.var_fdd_dn14 = 0.0;
        locals.var_fdd_rv = 0.0;

        locals.var_eeff = 0.0;
        locals.var_eeff_dn0 = 0.0;
        locals.var_eeff_dn2 = 0.0;
        locals.var_eeff_dn4 = 0.0;
        locals.var_eeff_dn5 = 0.0;
        locals.var_eeff_dn6 = 0.0;
        locals.var_eeff_dn7 = 0.0;
        locals.var_eeff_dn8 = 0.0;
        locals.var_eeff_dn9 = 0.0;
        locals.var_eeff_dn10 = 0.0;
        locals.var_eeff_dn11 = 0.0;
        locals.var_eeff_dn14 = 0.0;
        locals.var_eeff_rv = 0.0;

        locals.var_rns = 0.0;
        locals.var_rns_dn0 = 0.0;
        locals.var_rns_dn2 = 0.0;
        locals.var_rns_dn4 = 0.0;
        locals.var_rns_dn5 = 0.0;
        locals.var_rns_dn6 = 0.0;
        locals.var_rns_dn7 = 0.0;
        locals.var_rns_dn8 = 0.0;
        locals.var_rns_dn9 = 0.0;
        locals.var_rns_dn10 = 0.0;
        locals.var_rns_dn11 = 0.0;
        locals.var_rns_dn14 = 0.0;
        locals.var_rns_rv = 0.0;

        locals.var_mu = 0.0;
        locals.var_mu_dn0 = 0.0;
        locals.var_mu_dn2 = 0.0;
        locals.var_mu_dn4 = 0.0;
        locals.var_mu_dn5 = 0.0;
        locals.var_mu_dn6 = 0.0;
        locals.var_mu_dn7 = 0.0;
        locals.var_mu_dn8 = 0.0;
        locals.var_mu_dn9 = 0.0;
        locals.var_mu_dn10 = 0.0;
        locals.var_mu_dn11 = 0.0;
        locals.var_mu_dn14 = 0.0;
        locals.var_mu_rv = 0.0;

        locals.var_muun = 0.0;
        locals.var_muun_dn0 = 0.0;
        locals.var_muun_dn2 = 0.0;
        locals.var_muun_dn4 = 0.0;
        locals.var_muun_dn5 = 0.0;
        locals.var_muun_dn6 = 0.0;
        locals.var_muun_dn7 = 0.0;
        locals.var_muun_dn8 = 0.0;
        locals.var_muun_dn9 = 0.0;
        locals.var_muun_dn10 = 0.0;
        locals.var_muun_dn11 = 0.0;
        locals.var_muun_dn14 = 0.0;
        locals.var_muun_rv = 0.0;

        locals.var_ey = 0.0;
        locals.var_ey_dn0 = 0.0;
        locals.var_ey_dn2 = 0.0;
        locals.var_ey_dn4 = 0.0;
        locals.var_ey_dn5 = 0.0;
        locals.var_ey_dn6 = 0.0;
        locals.var_ey_dn7 = 0.0;
        locals.var_ey_dn8 = 0.0;
        locals.var_ey_dn9 = 0.0;
        locals.var_ey_dn10 = 0.0;
        locals.var_ey_dn11 = 0.0;
        locals.var_ey_dn14 = 0.0;
        locals.var_ey_rv = 0.0;

        locals.var_em = 0.0;
        locals.var_em_dn0 = 0.0;
        locals.var_em_dn2 = 0.0;
        locals.var_em_dn4 = 0.0;
        locals.var_em_dn5 = 0.0;
        locals.var_em_dn6 = 0.0;
        locals.var_em_dn7 = 0.0;
        locals.var_em_dn8 = 0.0;
        locals.var_em_dn9 = 0.0;
        locals.var_em_dn10 = 0.0;
        locals.var_em_dn11 = 0.0;
        locals.var_em_dn14 = 0.0;
        locals.var_em_rv = 0.0;

        locals.var_eta = 0.0;
        locals.var_eta_dn0 = 0.0;
        locals.var_eta_dn2 = 0.0;
        locals.var_eta_dn4 = 0.0;
        locals.var_eta_dn5 = 0.0;
        locals.var_eta_dn6 = 0.0;
        locals.var_eta_dn7 = 0.0;
        locals.var_eta_dn8 = 0.0;
        locals.var_eta_dn9 = 0.0;
        locals.var_eta_dn10 = 0.0;
        locals.var_eta_dn11 = 0.0;
        locals.var_eta_dn14 = 0.0;
        locals.var_eta_rv = 0.0;

        locals.var_eta1 = 0.0;
        locals.var_eta1_dn0 = 0.0;
        locals.var_eta1_dn2 = 0.0;
        locals.var_eta1_dn4 = 0.0;
        locals.var_eta1_dn5 = 0.0;
        locals.var_eta1_dn6 = 0.0;
        locals.var_eta1_dn7 = 0.0;
        locals.var_eta1_dn8 = 0.0;
        locals.var_eta1_dn9 = 0.0;
        locals.var_eta1_dn10 = 0.0;
        locals.var_eta1_dn11 = 0.0;
        locals.var_eta1_dn14 = 0.0;
        locals.var_eta1_rv = 0.0;

        locals.var_eta1p12 = 0.0;
        locals.var_eta1p12_dn0 = 0.0;
        locals.var_eta1p12_dn2 = 0.0;
        locals.var_eta1p12_dn4 = 0.0;
        locals.var_eta1p12_dn5 = 0.0;
        locals.var_eta1p12_dn6 = 0.0;
        locals.var_eta1p12_dn7 = 0.0;
        locals.var_eta1p12_dn8 = 0.0;
        locals.var_eta1p12_dn9 = 0.0;
        locals.var_eta1p12_dn10 = 0.0;
        locals.var_eta1p12_dn11 = 0.0;
        locals.var_eta1p12_dn14 = 0.0;
        locals.var_eta1p12_rv = 0.0;

        locals.var_eta1p32 = 0.0;
        locals.var_eta1p32_dn0 = 0.0;
        locals.var_eta1p32_dn2 = 0.0;
        locals.var_eta1p32_dn4 = 0.0;
        locals.var_eta1p32_dn5 = 0.0;
        locals.var_eta1p32_dn6 = 0.0;
        locals.var_eta1p32_dn7 = 0.0;
        locals.var_eta1p32_dn8 = 0.0;
        locals.var_eta1p32_dn9 = 0.0;
        locals.var_eta1p32_dn10 = 0.0;
        locals.var_eta1p32_dn11 = 0.0;
        locals.var_eta1p32_dn14 = 0.0;
        locals.var_eta1p32_rv = 0.0;

        locals.var_eta1p52 = 0.0;
        locals.var_eta1p52_dn0 = 0.0;
        locals.var_eta1p52_dn2 = 0.0;
        locals.var_eta1p52_dn4 = 0.0;
        locals.var_eta1p52_dn5 = 0.0;
        locals.var_eta1p52_dn6 = 0.0;
        locals.var_eta1p52_dn7 = 0.0;
        locals.var_eta1p52_dn8 = 0.0;
        locals.var_eta1p52_dn9 = 0.0;
        locals.var_eta1p52_dn10 = 0.0;
        locals.var_eta1p52_dn11 = 0.0;
        locals.var_eta1p52_dn14 = 0.0;
        locals.var_eta1p52_rv = 0.0;

        locals.var_zeta12 = 0.0;
        locals.var_zeta12_dn0 = 0.0;
        locals.var_zeta12_dn2 = 0.0;
        locals.var_zeta12_dn4 = 0.0;
        locals.var_zeta12_dn5 = 0.0;
        locals.var_zeta12_dn6 = 0.0;
        locals.var_zeta12_dn7 = 0.0;
        locals.var_zeta12_dn8 = 0.0;
        locals.var_zeta12_dn9 = 0.0;
        locals.var_zeta12_dn10 = 0.0;
        locals.var_zeta12_dn11 = 0.0;
        locals.var_zeta12_dn14 = 0.0;
        locals.var_zeta12_rv = 0.0;

        locals.var_zeta32 = 0.0;
        locals.var_zeta32_dn0 = 0.0;
        locals.var_zeta32_dn2 = 0.0;
        locals.var_zeta32_dn4 = 0.0;
        locals.var_zeta32_dn5 = 0.0;
        locals.var_zeta32_dn6 = 0.0;
        locals.var_zeta32_dn7 = 0.0;
        locals.var_zeta32_dn8 = 0.0;
        locals.var_zeta32_dn9 = 0.0;
        locals.var_zeta32_dn10 = 0.0;
        locals.var_zeta32_dn11 = 0.0;
        locals.var_zeta32_dn14 = 0.0;
        locals.var_zeta32_rv = 0.0;

        locals.var_zeta52 = 0.0;
        locals.var_zeta52_dn0 = 0.0;
        locals.var_zeta52_dn2 = 0.0;
        locals.var_zeta52_dn4 = 0.0;
        locals.var_zeta52_dn5 = 0.0;
        locals.var_zeta52_dn6 = 0.0;
        locals.var_zeta52_dn7 = 0.0;
        locals.var_zeta52_dn8 = 0.0;
        locals.var_zeta52_dn9 = 0.0;
        locals.var_zeta52_dn10 = 0.0;
        locals.var_zeta52_dn11 = 0.0;
        locals.var_zeta52_dn14 = 0.0;
        locals.var_zeta52_rv = 0.0;

        locals.var_f00 = 0.0;
        locals.var_f00_dn0 = 0.0;
        locals.var_f00_dn2 = 0.0;
        locals.var_f00_dn4 = 0.0;
        locals.var_f00_dn5 = 0.0;
        locals.var_f00_dn6 = 0.0;
        locals.var_f00_dn7 = 0.0;
        locals.var_f00_dn8 = 0.0;
        locals.var_f00_dn9 = 0.0;
        locals.var_f00_dn10 = 0.0;
        locals.var_f00_dn11 = 0.0;
        locals.var_f00_dn14 = 0.0;
        locals.var_f00_rv = 0.0;

        locals.var_f10 = 0.0;
        locals.var_f10_dn0 = 0.0;
        locals.var_f10_dn2 = 0.0;
        locals.var_f10_dn4 = 0.0;
        locals.var_f10_dn5 = 0.0;
        locals.var_f10_dn6 = 0.0;
        locals.var_f10_dn7 = 0.0;
        locals.var_f10_dn8 = 0.0;
        locals.var_f10_dn9 = 0.0;
        locals.var_f10_dn10 = 0.0;
        locals.var_f10_dn11 = 0.0;
        locals.var_f10_dn14 = 0.0;
        locals.var_f10_rv = 0.0;

        locals.var_f30 = 0.0;
        locals.var_f30_dn0 = 0.0;
        locals.var_f30_dn2 = 0.0;
        locals.var_f30_dn4 = 0.0;
        locals.var_f30_dn5 = 0.0;
        locals.var_f30_dn6 = 0.0;
        locals.var_f30_dn7 = 0.0;
        locals.var_f30_dn8 = 0.0;
        locals.var_f30_dn9 = 0.0;
        locals.var_f30_dn10 = 0.0;
        locals.var_f30_dn11 = 0.0;
        locals.var_f30_dn14 = 0.0;
        locals.var_f30_rv = 0.0;

        locals.var_f11 = 0.0;
        locals.var_f11_dn0 = 0.0;
        locals.var_f11_dn2 = 0.0;
        locals.var_f11_dn4 = 0.0;
        locals.var_f11_dn5 = 0.0;
        locals.var_f11_dn6 = 0.0;
        locals.var_f11_dn7 = 0.0;
        locals.var_f11_dn8 = 0.0;
        locals.var_f11_dn9 = 0.0;
        locals.var_f11_dn10 = 0.0;
        locals.var_f11_dn11 = 0.0;
        locals.var_f11_dn14 = 0.0;
        locals.var_f11_rv = 0.0;

        locals.var_vgs_min = 0.0;
        locals.var_vgs_min_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        locals: &mut StampLocals,
    ) {
        locals.var_ps0_min = 0.0;
        locals.var_ps0_min_dn0 = 0.0;
        locals.var_ps0_min_dn2 = 0.0;
        locals.var_ps0_min_dn4 = 0.0;
        locals.var_ps0_min_dn5 = 0.0;
        locals.var_ps0_min_dn6 = 0.0;
        locals.var_ps0_min_dn7 = 0.0;
        locals.var_ps0_min_dn8 = 0.0;
        locals.var_ps0_min_dn9 = 0.0;
        locals.var_ps0_min_dn10 = 0.0;
        locals.var_ps0_min_dn11 = 0.0;
        locals.var_ps0_min_dn14 = 0.0;
        locals.var_ps0_min_rv = 0.0;

        locals.var_acn = 0.0;
        locals.var_acn_dn0 = 0.0;
        locals.var_acn_dn2 = 0.0;
        locals.var_acn_dn4 = 0.0;
        locals.var_acn_dn5 = 0.0;
        locals.var_acn_dn6 = 0.0;
        locals.var_acn_dn7 = 0.0;
        locals.var_acn_dn8 = 0.0;
        locals.var_acn_dn9 = 0.0;
        locals.var_acn_dn10 = 0.0;
        locals.var_acn_dn11 = 0.0;
        locals.var_acn_dn14 = 0.0;
        locals.var_acn_rv = 0.0;

        locals.var_acd = 0.0;
        locals.var_acd_dn0 = 0.0;
        locals.var_acd_dn2 = 0.0;
        locals.var_acd_dn4 = 0.0;
        locals.var_acd_dn5 = 0.0;
        locals.var_acd_dn6 = 0.0;
        locals.var_acd_dn7 = 0.0;
        locals.var_acd_dn8 = 0.0;
        locals.var_acd_dn9 = 0.0;
        locals.var_acd_dn10 = 0.0;
        locals.var_acd_dn11 = 0.0;
        locals.var_acd_dn14 = 0.0;
        locals.var_acd_rv = 0.0;

        locals.var_ac1 = 0.0;
        locals.var_ac1_dn0 = 0.0;
        locals.var_ac1_dn2 = 0.0;
        locals.var_ac1_dn4 = 0.0;
        locals.var_ac1_dn5 = 0.0;
        locals.var_ac1_dn6 = 0.0;
        locals.var_ac1_dn7 = 0.0;
        locals.var_ac1_dn8 = 0.0;
        locals.var_ac1_dn9 = 0.0;
        locals.var_ac1_dn10 = 0.0;
        locals.var_ac1_dn11 = 0.0;
        locals.var_ac1_dn14 = 0.0;
        locals.var_ac1_rv = 0.0;

        locals.var_ac2 = 0.0;
        locals.var_ac2_dn0 = 0.0;
        locals.var_ac2_dn2 = 0.0;
        locals.var_ac2_dn4 = 0.0;
        locals.var_ac2_dn5 = 0.0;
        locals.var_ac2_dn6 = 0.0;
        locals.var_ac2_dn7 = 0.0;
        locals.var_ac2_dn8 = 0.0;
        locals.var_ac2_dn9 = 0.0;
        locals.var_ac2_dn10 = 0.0;
        locals.var_ac2_dn11 = 0.0;
        locals.var_ac2_dn14 = 0.0;
        locals.var_ac2_rv = 0.0;

        locals.var_ac3 = 0.0;
        locals.var_ac3_dn0 = 0.0;
        locals.var_ac3_dn2 = 0.0;
        locals.var_ac3_dn4 = 0.0;
        locals.var_ac3_dn5 = 0.0;
        locals.var_ac3_dn6 = 0.0;
        locals.var_ac3_dn7 = 0.0;
        locals.var_ac3_dn8 = 0.0;
        locals.var_ac3_dn9 = 0.0;
        locals.var_ac3_dn10 = 0.0;
        locals.var_ac3_dn11 = 0.0;
        locals.var_ac3_dn14 = 0.0;
        locals.var_ac3_rv = 0.0;

        locals.var_ac4 = 0.0;
        locals.var_ac4_dn0 = 0.0;
        locals.var_ac4_dn2 = 0.0;
        locals.var_ac4_dn4 = 0.0;
        locals.var_ac4_dn5 = 0.0;
        locals.var_ac4_dn6 = 0.0;
        locals.var_ac4_dn7 = 0.0;
        locals.var_ac4_dn8 = 0.0;
        locals.var_ac4_dn9 = 0.0;
        locals.var_ac4_dn10 = 0.0;
        locals.var_ac4_dn11 = 0.0;
        locals.var_ac4_dn14 = 0.0;
        locals.var_ac4_rv = 0.0;

        locals.var_ac31 = 0.0;
        locals.var_ac31_dn0 = 0.0;
        locals.var_ac31_dn2 = 0.0;
        locals.var_ac31_dn4 = 0.0;
        locals.var_ac31_dn5 = 0.0;
        locals.var_ac31_dn6 = 0.0;
        locals.var_ac31_dn7 = 0.0;
        locals.var_ac31_dn8 = 0.0;
        locals.var_ac31_dn9 = 0.0;
        locals.var_ac31_dn10 = 0.0;
        locals.var_ac31_dn11 = 0.0;
        locals.var_ac31_dn14 = 0.0;
        locals.var_ac31_rv = 0.0;

        locals.var_ac41 = 0.0;
        locals.var_ac41_dn0 = 0.0;
        locals.var_ac41_dn2 = 0.0;
        locals.var_ac41_dn4 = 0.0;
        locals.var_ac41_dn5 = 0.0;
        locals.var_ac41_dn6 = 0.0;
        locals.var_ac41_dn7 = 0.0;
        locals.var_ac41_dn8 = 0.0;
        locals.var_ac41_dn9 = 0.0;
        locals.var_ac41_dn10 = 0.0;
        locals.var_ac41_dn11 = 0.0;
        locals.var_ac41_dn14 = 0.0;
        locals.var_ac41_rv = 0.0;

        locals.var_isub = 0.0;
        locals.var_isub_dn0 = 0.0;
        locals.var_isub_dn2 = 0.0;
        locals.var_isub_dn4 = 0.0;
        locals.var_isub_dn5 = 0.0;
        locals.var_isub_dn6 = 0.0;
        locals.var_isub_dn7 = 0.0;
        locals.var_isub_dn8 = 0.0;
        locals.var_isub_dn9 = 0.0;
        locals.var_isub_dn10 = 0.0;
        locals.var_isub_dn11 = 0.0;
        locals.var_isub_dn14 = 0.0;
        locals.var_isub_rv = 0.0;

        locals.var_isubld = 0.0;
        locals.var_isubld_dn0 = 0.0;
        locals.var_isubld_dn2 = 0.0;
        locals.var_isubld_dn4 = 0.0;
        locals.var_isubld_dn5 = 0.0;
        locals.var_isubld_dn6 = 0.0;
        locals.var_isubld_dn7 = 0.0;
        locals.var_isubld_dn8 = 0.0;
        locals.var_isubld_dn9 = 0.0;
        locals.var_isubld_dn10 = 0.0;
        locals.var_isubld_dn11 = 0.0;
        locals.var_isubld_dn14 = 0.0;
        locals.var_isubld_rv = 0.0;

        locals.var_psislsat = 0.0;
        locals.var_psislsat_dn0 = 0.0;
        locals.var_psislsat_dn2 = 0.0;
        locals.var_psislsat_dn4 = 0.0;
        locals.var_psislsat_dn5 = 0.0;
        locals.var_psislsat_dn6 = 0.0;
        locals.var_psislsat_dn7 = 0.0;
        locals.var_psislsat_dn8 = 0.0;
        locals.var_psislsat_dn9 = 0.0;
        locals.var_psislsat_dn10 = 0.0;
        locals.var_psislsat_dn11 = 0.0;
        locals.var_psislsat_dn14 = 0.0;
        locals.var_psislsat_rv = 0.0;

        locals.var_psisubsat = 0.0;
        locals.var_psisubsat_dn0 = 0.0;
        locals.var_psisubsat_dn2 = 0.0;
        locals.var_psisubsat_dn4 = 0.0;
        locals.var_psisubsat_dn5 = 0.0;
        locals.var_psisubsat_dn6 = 0.0;
        locals.var_psisubsat_dn7 = 0.0;
        locals.var_psisubsat_dn8 = 0.0;
        locals.var_psisubsat_dn9 = 0.0;
        locals.var_psisubsat_dn10 = 0.0;
        locals.var_psisubsat_dn11 = 0.0;
        locals.var_psisubsat_dn14 = 0.0;
        locals.var_psisubsat_rv = 0.0;

        locals.var_eg12 = 0.0;
        locals.var_eg12_dn0 = 0.0;
        locals.var_eg12_dn2 = 0.0;
        locals.var_eg12_dn4 = 0.0;
        locals.var_eg12_dn5 = 0.0;
        locals.var_eg12_dn6 = 0.0;
        locals.var_eg12_dn7 = 0.0;
        locals.var_eg12_dn8 = 0.0;
        locals.var_eg12_dn9 = 0.0;
        locals.var_eg12_dn10 = 0.0;
        locals.var_eg12_dn11 = 0.0;
        locals.var_eg12_dn14 = 0.0;
        locals.var_eg12_rv = 0.0;

        locals.var_eg32 = 0.0;
        locals.var_eg32_dn0 = 0.0;
        locals.var_eg32_dn2 = 0.0;
        locals.var_eg32_dn4 = 0.0;
        locals.var_eg32_dn5 = 0.0;
        locals.var_eg32_dn6 = 0.0;
        locals.var_eg32_dn7 = 0.0;
        locals.var_eg32_dn8 = 0.0;
        locals.var_eg32_dn9 = 0.0;
        locals.var_eg32_dn10 = 0.0;
        locals.var_eg32_dn11 = 0.0;
        locals.var_eg32_dn14 = 0.0;
        locals.var_eg32_rv = 0.0;

        locals.var_cov_slp = 0.0;
        locals.var_cov_slp_rv = 0.0;

        locals.var_cov_mag = 0.0;
        locals.var_cov_mag_rv = 0.0;

        locals.var_qgos = 0.0;
        locals.var_qgos_dn0 = 0.0;
        locals.var_qgos_dn2 = 0.0;
        locals.var_qgos_dn4 = 0.0;
        locals.var_qgos_dn5 = 0.0;
        locals.var_qgos_dn6 = 0.0;
        locals.var_qgos_dn7 = 0.0;
        locals.var_qgos_dn8 = 0.0;
        locals.var_qgos_dn9 = 0.0;
        locals.var_qgos_dn10 = 0.0;
        locals.var_qgos_dn11 = 0.0;
        locals.var_qgos_dn14 = 0.0;
        locals.var_qgos_rv = 0.0;

        locals.var_qgod = 0.0;
        locals.var_qgod_dn0 = 0.0;
        locals.var_qgod_dn2 = 0.0;
        locals.var_qgod_dn4 = 0.0;
        locals.var_qgod_dn5 = 0.0;
        locals.var_qgod_dn6 = 0.0;
        locals.var_qgod_dn7 = 0.0;
        locals.var_qgod_dn8 = 0.0;
        locals.var_qgod_dn9 = 0.0;
        locals.var_qgod_dn10 = 0.0;
        locals.var_qgod_dn11 = 0.0;
        locals.var_qgod_dn14 = 0.0;
        locals.var_qgod_rv = 0.0;

        locals.var_qgbo = 0.0;
        locals.var_qgbo_dn7 = 0.0;
        locals.var_qgbo_dn8 = 0.0;
        locals.var_qgbo_dn9 = 0.0;
        locals.var_qgbo_rv = 0.0;

        locals.var_cgbo_loc = 0.0;
        locals.var_cgbo_loc_rv = 0.0;

        locals.var_qgso = 0.0;
        locals.var_qgso_dn2 = 0.0;
        locals.var_qgso_dn7 = 0.0;
        locals.var_qgso_rv = 0.0;

        locals.var_qgdo = 0.0;
        locals.var_qgdo_dn0 = 0.0;
        locals.var_qgdo_dn2 = 0.0;
        locals.var_qgdo_dn7 = 0.0;
        locals.var_qgdo_rv = 0.0;

        locals.var_qfd = 0.0;
        locals.var_qfd_dn0 = 0.0;
        locals.var_qfd_dn2 = 0.0;
        locals.var_qfd_dn7 = 0.0;
        locals.var_qfd_rv = 0.0;

        locals.var_cfd = 0.0;
        locals.var_cfd_rv = 0.0;

        locals.var_qfs = 0.0;
        locals.var_qfs_dn2 = 0.0;
        locals.var_qfs_dn7 = 0.0;
        locals.var_qfs_rv = 0.0;

        locals.var_cfs = 0.0;
        locals.var_cfs_rv = 0.0;

        locals.var_ec = 0.0;
        locals.var_ec_dn0 = 0.0;
        locals.var_ec_dn2 = 0.0;
        locals.var_ec_dn4 = 0.0;
        locals.var_ec_dn5 = 0.0;
        locals.var_ec_dn6 = 0.0;
        locals.var_ec_dn7 = 0.0;
        locals.var_ec_dn8 = 0.0;
        locals.var_ec_dn9 = 0.0;
        locals.var_ec_dn10 = 0.0;
        locals.var_ec_dn11 = 0.0;
        locals.var_ec_dn14 = 0.0;
        locals.var_ec_rv = 0.0;

        locals.var_pslk = 0.0;
        locals.var_pslk_dn0 = 0.0;
        locals.var_pslk_dn2 = 0.0;
        locals.var_pslk_dn4 = 0.0;
        locals.var_pslk_dn5 = 0.0;
        locals.var_pslk_dn6 = 0.0;
        locals.var_pslk_dn7 = 0.0;
        locals.var_pslk_dn8 = 0.0;
        locals.var_pslk_dn9 = 0.0;
        locals.var_pslk_dn10 = 0.0;
        locals.var_pslk_dn11 = 0.0;
        locals.var_pslk_dn14 = 0.0;
        locals.var_pslk_rv = 0.0;

        locals.var_qy = 0.0;
        locals.var_qy_dn0 = 0.0;
        locals.var_qy_dn2 = 0.0;
        locals.var_qy_dn4 = 0.0;
        locals.var_qy_dn5 = 0.0;
        locals.var_qy_dn6 = 0.0;
        locals.var_qy_dn7 = 0.0;
        locals.var_qy_dn8 = 0.0;
        locals.var_qy_dn9 = 0.0;
        locals.var_qy_dn10 = 0.0;
        locals.var_qy_dn11 = 0.0;
        locals.var_qy_dn14 = 0.0;
        locals.var_qy_rv = 0.0;

        locals.var_eyd = 0.0;
        locals.var_eyd_dn0 = 0.0;
        locals.var_eyd_dn2 = 0.0;
        locals.var_eyd_dn4 = 0.0;
        locals.var_eyd_dn5 = 0.0;
        locals.var_eyd_dn6 = 0.0;
        locals.var_eyd_dn7 = 0.0;
        locals.var_eyd_dn8 = 0.0;
        locals.var_eyd_dn9 = 0.0;
        locals.var_eyd_dn10 = 0.0;
        locals.var_eyd_dn11 = 0.0;
        locals.var_eyd_dn14 = 0.0;
        locals.var_eyd_rv = 0.0;

        locals.var_mu_ave = 0.0;
        locals.var_mu_ave_dn0 = 0.0;
        locals.var_mu_ave_dn2 = 0.0;
        locals.var_mu_ave_dn4 = 0.0;
        locals.var_mu_ave_dn5 = 0.0;
        locals.var_mu_ave_dn6 = 0.0;
        locals.var_mu_ave_dn7 = 0.0;
        locals.var_mu_ave_dn8 = 0.0;
        locals.var_mu_ave_dn9 = 0.0;
        locals.var_mu_ave_dn10 = 0.0;
        locals.var_mu_ave_dn11 = 0.0;
        locals.var_mu_ave_dn14 = 0.0;
        locals.var_mu_ave_rv = 0.0;

        locals.var_nthrml = 0.0;
        locals.var_nthrml_dn0 = 0.0;
        locals.var_nthrml_dn2 = 0.0;
        locals.var_nthrml_dn4 = 0.0;
        locals.var_nthrml_dn5 = 0.0;
        locals.var_nthrml_dn6 = 0.0;
        locals.var_nthrml_dn7 = 0.0;
        locals.var_nthrml_dn8 = 0.0;
        locals.var_nthrml_dn9 = 0.0;
        locals.var_nthrml_dn10 = 0.0;
        locals.var_nthrml_dn11 = 0.0;
        locals.var_nthrml_dn14 = 0.0;
        locals.var_nthrml_rv = 0.0;

        locals.var_mud_hoso = 0.0;
        locals.var_mud_hoso_dn0 = 0.0;
        locals.var_mud_hoso_dn2 = 0.0;
        locals.var_mud_hoso_dn4 = 0.0;
        locals.var_mud_hoso_dn5 = 0.0;
        locals.var_mud_hoso_dn6 = 0.0;
        locals.var_mud_hoso_dn7 = 0.0;
        locals.var_mud_hoso_dn8 = 0.0;
        locals.var_mud_hoso_dn9 = 0.0;
        locals.var_mud_hoso_dn10 = 0.0;
        locals.var_mud_hoso_dn11 = 0.0;
        locals.var_mud_hoso_dn14 = 0.0;
        locals.var_mud_hoso_rv = 0.0;

        locals.var_kusai00 = 0.0;
        locals.var_kusai00_dn0 = 0.0;
        locals.var_kusai00_dn2 = 0.0;
        locals.var_kusai00_dn4 = 0.0;
        locals.var_kusai00_dn5 = 0.0;
        locals.var_kusai00_dn6 = 0.0;
        locals.var_kusai00_dn7 = 0.0;
        locals.var_kusai00_dn8 = 0.0;
        locals.var_kusai00_dn9 = 0.0;
        locals.var_kusai00_dn10 = 0.0;
        locals.var_kusai00_dn11 = 0.0;
        locals.var_kusai00_dn14 = 0.0;
        locals.var_kusai00_rv = 0.0;

        locals.var_kusaidd = 0.0;
        locals.var_kusaidd_dn0 = 0.0;
        locals.var_kusaidd_dn2 = 0.0;
        locals.var_kusaidd_dn4 = 0.0;
        locals.var_kusaidd_dn5 = 0.0;
        locals.var_kusaidd_dn6 = 0.0;
        locals.var_kusaidd_dn7 = 0.0;
        locals.var_kusaidd_dn8 = 0.0;
        locals.var_kusaidd_dn9 = 0.0;
        locals.var_kusaidd_dn10 = 0.0;
        locals.var_kusaidd_dn11 = 0.0;
        locals.var_kusaidd_dn14 = 0.0;
        locals.var_kusaidd_rv = 0.0;

        locals.var_kusail = 0.0;
        locals.var_kusail_dn0 = 0.0;
        locals.var_kusail_dn2 = 0.0;
        locals.var_kusail_dn4 = 0.0;
        locals.var_kusail_dn5 = 0.0;
        locals.var_kusail_dn6 = 0.0;
        locals.var_kusail_dn7 = 0.0;
        locals.var_kusail_dn8 = 0.0;
        locals.var_kusail_dn9 = 0.0;
        locals.var_kusail_dn10 = 0.0;
        locals.var_kusail_dn11 = 0.0;
        locals.var_kusail_dn14 = 0.0;
        locals.var_kusail_rv = 0.0;

        locals.var_kusai00l = 0.0;
        locals.var_kusai00l_dn0 = 0.0;
        locals.var_kusai00l_dn2 = 0.0;
        locals.var_kusai00l_dn4 = 0.0;
        locals.var_kusai00l_dn5 = 0.0;
        locals.var_kusai00l_dn6 = 0.0;
        locals.var_kusai00l_dn7 = 0.0;
        locals.var_kusai00l_dn8 = 0.0;
        locals.var_kusai00l_dn9 = 0.0;
        locals.var_kusai00l_dn10 = 0.0;
        locals.var_kusai00l_dn11 = 0.0;
        locals.var_kusai00l_dn14 = 0.0;
        locals.var_kusai00l_rv = 0.0;

        locals.var_sqrtkusail = 0.0;
        locals.var_sqrtkusail_dn0 = 0.0;
        locals.var_sqrtkusail_dn2 = 0.0;
        locals.var_sqrtkusail_dn4 = 0.0;
        locals.var_sqrtkusail_dn5 = 0.0;
        locals.var_sqrtkusail_dn6 = 0.0;
        locals.var_sqrtkusail_dn7 = 0.0;
        locals.var_sqrtkusail_dn8 = 0.0;
        locals.var_sqrtkusail_dn9 = 0.0;
        locals.var_sqrtkusail_dn10 = 0.0;
        locals.var_sqrtkusail_dn11 = 0.0;
        locals.var_sqrtkusail_dn14 = 0.0;
        locals.var_sqrtkusail_rv = 0.0;

        locals.var_kusai_ig = 0.0;
        locals.var_kusai_ig_dn0 = 0.0;
        locals.var_kusai_ig_dn2 = 0.0;
        locals.var_kusai_ig_dn4 = 0.0;
        locals.var_kusai_ig_dn5 = 0.0;
        locals.var_kusai_ig_dn6 = 0.0;
        locals.var_kusai_ig_dn7 = 0.0;
        locals.var_kusai_ig_dn8 = 0.0;
        locals.var_kusai_ig_dn9 = 0.0;
        locals.var_kusai_ig_dn10 = 0.0;
        locals.var_kusai_ig_dn11 = 0.0;
        locals.var_kusai_ig_dn14 = 0.0;
        locals.var_kusai_ig_rv = 0.0;

        locals.var_gds0_ign = 0.0;
        locals.var_gds0_ign_dn0 = 0.0;
        locals.var_gds0_ign_dn2 = 0.0;
        locals.var_gds0_ign_dn4 = 0.0;
        locals.var_gds0_ign_dn5 = 0.0;
        locals.var_gds0_ign_dn6 = 0.0;
        locals.var_gds0_ign_dn7 = 0.0;
        locals.var_gds0_ign_dn8 = 0.0;
        locals.var_gds0_ign_dn9 = 0.0;
        locals.var_gds0_ign_dn10 = 0.0;
        locals.var_gds0_ign_dn11 = 0.0;
        locals.var_gds0_ign_dn14 = 0.0;
        locals.var_gds0_ign_rv = 0.0;

        locals.var_nign0 = 0.0;
        locals.var_nign0_dn0 = 0.0;
        locals.var_nign0_dn2 = 0.0;
        locals.var_nign0_dn4 = 0.0;
        locals.var_nign0_dn5 = 0.0;
        locals.var_nign0_dn6 = 0.0;
        locals.var_nign0_dn7 = 0.0;
        locals.var_nign0_dn8 = 0.0;
        locals.var_nign0_dn9 = 0.0;
        locals.var_nign0_dn10 = 0.0;
        locals.var_nign0_dn11 = 0.0;
        locals.var_nign0_dn14 = 0.0;
        locals.var_nign0_rv = 0.0;

        locals.var_mumoda = 0.0;
        locals.var_mumoda_dn0 = 0.0;
        locals.var_mumoda_dn2 = 0.0;
        locals.var_mumoda_dn4 = 0.0;
        locals.var_mumoda_dn5 = 0.0;
        locals.var_mumoda_dn6 = 0.0;
        locals.var_mumoda_dn7 = 0.0;
        locals.var_mumoda_dn8 = 0.0;
        locals.var_mumoda_dn9 = 0.0;
        locals.var_mumoda_dn10 = 0.0;
        locals.var_mumoda_dn11 = 0.0;
        locals.var_mumoda_dn14 = 0.0;
        locals.var_mumoda_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        locals: &mut StampLocals,
    ) {
        locals.var_mumodb = 0.0;
        locals.var_mumodb_dn0 = 0.0;
        locals.var_mumodb_dn2 = 0.0;
        locals.var_mumodb_dn4 = 0.0;
        locals.var_mumodb_dn5 = 0.0;
        locals.var_mumodb_dn6 = 0.0;
        locals.var_mumodb_dn7 = 0.0;
        locals.var_mumodb_dn8 = 0.0;
        locals.var_mumodb_dn9 = 0.0;
        locals.var_mumodb_dn10 = 0.0;
        locals.var_mumodb_dn11 = 0.0;
        locals.var_mumodb_dn14 = 0.0;
        locals.var_mumodb_rv = 0.0;

        locals.var_correct_w1 = 0.0;
        locals.var_correct_w1_dn0 = 0.0;
        locals.var_correct_w1_dn2 = 0.0;
        locals.var_correct_w1_dn4 = 0.0;
        locals.var_correct_w1_dn5 = 0.0;
        locals.var_correct_w1_dn6 = 0.0;
        locals.var_correct_w1_dn7 = 0.0;
        locals.var_correct_w1_dn8 = 0.0;
        locals.var_correct_w1_dn9 = 0.0;
        locals.var_correct_w1_dn10 = 0.0;
        locals.var_correct_w1_dn11 = 0.0;
        locals.var_correct_w1_dn14 = 0.0;
        locals.var_correct_w1_rv = 0.0;

        locals.var_tx = 0.0;
        locals.var_tx_dn0 = 0.0;
        locals.var_tx_dn2 = 0.0;
        locals.var_tx_dn4 = 0.0;
        locals.var_tx_dn5 = 0.0;
        locals.var_tx_dn6 = 0.0;
        locals.var_tx_dn7 = 0.0;
        locals.var_tx_dn8 = 0.0;
        locals.var_tx_dn9 = 0.0;
        locals.var_tx_dn10 = 0.0;
        locals.var_tx_dn11 = 0.0;
        locals.var_tx_dn14 = 0.0;
        locals.var_tx_rv = 0.0;

        locals.var_ty = 0.0;
        locals.var_ty_dn0 = 0.0;
        locals.var_ty_dn2 = 0.0;
        locals.var_ty_dn4 = 0.0;
        locals.var_ty_dn5 = 0.0;
        locals.var_ty_dn6 = 0.0;
        locals.var_ty_dn7 = 0.0;
        locals.var_ty_dn8 = 0.0;
        locals.var_ty_dn9 = 0.0;
        locals.var_ty_dn10 = 0.0;
        locals.var_ty_dn11 = 0.0;
        locals.var_ty_dn14 = 0.0;
        locals.var_ty_rv = 0.0;

        locals.var_t0 = 0.0;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        locals.var_t1 = 0.0;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        locals.var_t2 = 0.0;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn14 = 0.0;
        locals.var_t2_rv = 0.0;

        locals.var_t3 = 0.0;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn14 = 0.0;
        locals.var_t3_rv = 0.0;

        locals.var_t4 = 0.0;
        locals.var_t4_dn0 = 0.0;
        locals.var_t4_dn2 = 0.0;
        locals.var_t4_dn4 = 0.0;
        locals.var_t4_dn5 = 0.0;
        locals.var_t4_dn6 = 0.0;
        locals.var_t4_dn7 = 0.0;
        locals.var_t4_dn8 = 0.0;
        locals.var_t4_dn9 = 0.0;
        locals.var_t4_dn10 = 0.0;
        locals.var_t4_dn11 = 0.0;
        locals.var_t4_dn14 = 0.0;
        locals.var_t4_rv = 0.0;

        locals.var_t5 = 0.0;
        locals.var_t5_dn0 = 0.0;
        locals.var_t5_dn2 = 0.0;
        locals.var_t5_dn4 = 0.0;
        locals.var_t5_dn5 = 0.0;
        locals.var_t5_dn6 = 0.0;
        locals.var_t5_dn7 = 0.0;
        locals.var_t5_dn8 = 0.0;
        locals.var_t5_dn9 = 0.0;
        locals.var_t5_dn10 = 0.0;
        locals.var_t5_dn11 = 0.0;
        locals.var_t5_dn14 = 0.0;
        locals.var_t5_rv = 0.0;

        locals.var_t6 = 0.0;
        locals.var_t6_dn0 = 0.0;
        locals.var_t6_dn2 = 0.0;
        locals.var_t6_dn4 = 0.0;
        locals.var_t6_dn5 = 0.0;
        locals.var_t6_dn6 = 0.0;
        locals.var_t6_dn7 = 0.0;
        locals.var_t6_dn8 = 0.0;
        locals.var_t6_dn9 = 0.0;
        locals.var_t6_dn10 = 0.0;
        locals.var_t6_dn11 = 0.0;
        locals.var_t6_dn14 = 0.0;
        locals.var_t6_rv = 0.0;

        locals.var_t7 = 0.0;
        locals.var_t7_dn0 = 0.0;
        locals.var_t7_dn2 = 0.0;
        locals.var_t7_dn4 = 0.0;
        locals.var_t7_dn5 = 0.0;
        locals.var_t7_dn6 = 0.0;
        locals.var_t7_dn7 = 0.0;
        locals.var_t7_dn8 = 0.0;
        locals.var_t7_dn9 = 0.0;
        locals.var_t7_dn10 = 0.0;
        locals.var_t7_dn11 = 0.0;
        locals.var_t7_dn14 = 0.0;
        locals.var_t7_rv = 0.0;

        locals.var_t8 = 0.0;
        locals.var_t8_dn0 = 0.0;
        locals.var_t8_dn2 = 0.0;
        locals.var_t8_dn4 = 0.0;
        locals.var_t8_dn5 = 0.0;
        locals.var_t8_dn6 = 0.0;
        locals.var_t8_dn7 = 0.0;
        locals.var_t8_dn8 = 0.0;
        locals.var_t8_dn9 = 0.0;
        locals.var_t8_dn10 = 0.0;
        locals.var_t8_dn11 = 0.0;
        locals.var_t8_dn14 = 0.0;
        locals.var_t8_rv = 0.0;

        locals.var_t9 = 0.0;
        locals.var_t9_dn0 = 0.0;
        locals.var_t9_dn2 = 0.0;
        locals.var_t9_dn4 = 0.0;
        locals.var_t9_dn5 = 0.0;
        locals.var_t9_dn6 = 0.0;
        locals.var_t9_dn7 = 0.0;
        locals.var_t9_dn8 = 0.0;
        locals.var_t9_dn9 = 0.0;
        locals.var_t9_dn10 = 0.0;
        locals.var_t9_dn11 = 0.0;
        locals.var_t9_dn14 = 0.0;
        locals.var_t9_rv = 0.0;

        locals.var_t10 = 0.0;
        locals.var_t10_dn0 = 0.0;
        locals.var_t10_dn2 = 0.0;
        locals.var_t10_dn4 = 0.0;
        locals.var_t10_dn5 = 0.0;
        locals.var_t10_dn6 = 0.0;
        locals.var_t10_dn7 = 0.0;
        locals.var_t10_dn8 = 0.0;
        locals.var_t10_dn9 = 0.0;
        locals.var_t10_dn10 = 0.0;
        locals.var_t10_dn11 = 0.0;
        locals.var_t10_dn14 = 0.0;
        locals.var_t10_rv = 0.0;

        locals.var_t11 = 0.0;
        locals.var_t11_dn0 = 0.0;
        locals.var_t11_dn2 = 0.0;
        locals.var_t11_dn4 = 0.0;
        locals.var_t11_dn5 = 0.0;
        locals.var_t11_dn6 = 0.0;
        locals.var_t11_dn7 = 0.0;
        locals.var_t11_dn8 = 0.0;
        locals.var_t11_dn9 = 0.0;
        locals.var_t11_dn10 = 0.0;
        locals.var_t11_dn11 = 0.0;
        locals.var_t11_dn14 = 0.0;
        locals.var_t11_rv = 0.0;

        locals.var_t12 = 0.0;
        locals.var_t12_dn0 = 0.0;
        locals.var_t12_dn2 = 0.0;
        locals.var_t12_dn4 = 0.0;
        locals.var_t12_dn5 = 0.0;
        locals.var_t12_dn6 = 0.0;
        locals.var_t12_dn7 = 0.0;
        locals.var_t12_dn8 = 0.0;
        locals.var_t12_dn9 = 0.0;
        locals.var_t12_dn10 = 0.0;
        locals.var_t12_dn11 = 0.0;
        locals.var_t12_dn14 = 0.0;
        locals.var_t12_rv = 0.0;

        locals.var_vdseff = 0.0;
        locals.var_vdseff_dn0 = 0.0;
        locals.var_vdseff_dn2 = 0.0;
        locals.var_vdseff_dn4 = 0.0;
        locals.var_vdseff_dn5 = 0.0;
        locals.var_vdseff_dn6 = 0.0;
        locals.var_vdseff_dn7 = 0.0;
        locals.var_vdseff_dn8 = 0.0;
        locals.var_vdseff_dn9 = 0.0;
        locals.var_vdseff_dn10 = 0.0;
        locals.var_vdseff_dn11 = 0.0;
        locals.var_vdseff_dn14 = 0.0;
        locals.var_vdseff_rv = 0.0;

        locals.var_vdsorg = 0.0;
        locals.var_vdsorg_dn0 = 0.0;
        locals.var_vdsorg_dn2 = 0.0;
        locals.var_vdsorg_dn4 = 0.0;
        locals.var_vdsorg_dn5 = 0.0;
        locals.var_vdsorg_dn6 = 0.0;
        locals.var_vdsorg_dn7 = 0.0;
        locals.var_vdsorg_dn8 = 0.0;
        locals.var_vdsorg_dn9 = 0.0;
        locals.var_vdsorg_dn10 = 0.0;
        locals.var_vdsorg_dn11 = 0.0;
        locals.var_vdsorg_dn14 = 0.0;
        locals.var_vdsorg_rv = 0.0;

        locals.var_qovdext = 0.0;
        locals.var_qovdext_dn0 = 0.0;
        locals.var_qovdext_dn2 = 0.0;
        locals.var_qovdext_dn4 = 0.0;
        locals.var_qovdext_dn5 = 0.0;
        locals.var_qovdext_dn6 = 0.0;
        locals.var_qovdext_dn7 = 0.0;
        locals.var_qovdext_dn8 = 0.0;
        locals.var_qovdext_dn9 = 0.0;
        locals.var_qovdext_dn10 = 0.0;
        locals.var_qovdext_dn11 = 0.0;
        locals.var_qovdext_dn14 = 0.0;
        locals.var_qovdext_rv = 0.0;

        locals.var_qovsext = 0.0;
        locals.var_qovsext_dn0 = 0.0;
        locals.var_qovsext_dn2 = 0.0;
        locals.var_qovsext_dn4 = 0.0;
        locals.var_qovsext_dn5 = 0.0;
        locals.var_qovsext_dn6 = 0.0;
        locals.var_qovsext_dn7 = 0.0;
        locals.var_qovsext_dn8 = 0.0;
        locals.var_qovsext_dn9 = 0.0;
        locals.var_qovsext_dn10 = 0.0;
        locals.var_qovsext_dn11 = 0.0;
        locals.var_qovsext_dn14 = 0.0;
        locals.var_qovsext_rv = 0.0;

        locals.var_qovd = 0.0;
        locals.var_qovd_dn0 = 0.0;
        locals.var_qovd_dn2 = 0.0;
        locals.var_qovd_dn4 = 0.0;
        locals.var_qovd_dn5 = 0.0;
        locals.var_qovd_dn6 = 0.0;
        locals.var_qovd_dn7 = 0.0;
        locals.var_qovd_dn8 = 0.0;
        locals.var_qovd_dn9 = 0.0;
        locals.var_qovd_dn10 = 0.0;
        locals.var_qovd_dn11 = 0.0;
        locals.var_qovd_dn14 = 0.0;
        locals.var_qovd_rv = 0.0;

        locals.var_qovs = 0.0;
        locals.var_qovs_dn0 = 0.0;
        locals.var_qovs_dn2 = 0.0;
        locals.var_qovs_dn4 = 0.0;
        locals.var_qovs_dn5 = 0.0;
        locals.var_qovs_dn6 = 0.0;
        locals.var_qovs_dn7 = 0.0;
        locals.var_qovs_dn8 = 0.0;
        locals.var_qovs_dn9 = 0.0;
        locals.var_qovs_dn10 = 0.0;
        locals.var_qovs_dn11 = 0.0;
        locals.var_qovs_dn14 = 0.0;
        locals.var_qovs_rv = 0.0;

        locals.var_qbuld = 0.0;
        locals.var_qbuld_dn0 = 0.0;
        locals.var_qbuld_dn2 = 0.0;
        locals.var_qbuld_dn4 = 0.0;
        locals.var_qbuld_dn5 = 0.0;
        locals.var_qbuld_dn6 = 0.0;
        locals.var_qbuld_dn7 = 0.0;
        locals.var_qbuld_dn8 = 0.0;
        locals.var_qbuld_dn9 = 0.0;
        locals.var_qbuld_dn10 = 0.0;
        locals.var_qbuld_dn11 = 0.0;
        locals.var_qbuld_dn14 = 0.0;
        locals.var_qbuld_rv = 0.0;

        locals.var_qbdld = 0.0;
        locals.var_qbdld_dn0 = 0.0;
        locals.var_qbdld_dn2 = 0.0;
        locals.var_qbdld_dn4 = 0.0;
        locals.var_qbdld_dn5 = 0.0;
        locals.var_qbdld_dn6 = 0.0;
        locals.var_qbdld_dn7 = 0.0;
        locals.var_qbdld_dn8 = 0.0;
        locals.var_qbdld_dn9 = 0.0;
        locals.var_qbdld_dn10 = 0.0;
        locals.var_qbdld_dn11 = 0.0;
        locals.var_qbdld_dn14 = 0.0;
        locals.var_qbdld_rv = 0.0;

        locals.var_qbsld = 0.0;
        locals.var_qbsld_dn0 = 0.0;
        locals.var_qbsld_dn2 = 0.0;
        locals.var_qbsld_dn4 = 0.0;
        locals.var_qbsld_dn5 = 0.0;
        locals.var_qbsld_dn6 = 0.0;
        locals.var_qbsld_dn7 = 0.0;
        locals.var_qbsld_dn8 = 0.0;
        locals.var_qbsld_dn9 = 0.0;
        locals.var_qbsld_dn10 = 0.0;
        locals.var_qbsld_dn11 = 0.0;
        locals.var_qbsld_dn14 = 0.0;
        locals.var_qbsld_rv = 0.0;

        locals.var_qodad = 0.0;
        locals.var_qodad_dn0 = 0.0;
        locals.var_qodad_dn2 = 0.0;
        locals.var_qodad_dn4 = 0.0;
        locals.var_qodad_dn5 = 0.0;
        locals.var_qodad_dn6 = 0.0;
        locals.var_qodad_dn7 = 0.0;
        locals.var_qodad_dn8 = 0.0;
        locals.var_qodad_dn9 = 0.0;
        locals.var_qodad_dn10 = 0.0;
        locals.var_qodad_dn11 = 0.0;
        locals.var_qodad_dn14 = 0.0;
        locals.var_qodad_rv = 0.0;

        locals.var_qbdldext = 0.0;
        locals.var_qbdldext_dn0 = 0.0;
        locals.var_qbdldext_dn2 = 0.0;
        locals.var_qbdldext_dn4 = 0.0;
        locals.var_qbdldext_dn5 = 0.0;
        locals.var_qbdldext_dn6 = 0.0;
        locals.var_qbdldext_dn7 = 0.0;
        locals.var_qbdldext_dn8 = 0.0;
        locals.var_qbdldext_dn9 = 0.0;
        locals.var_qbdldext_dn10 = 0.0;
        locals.var_qbdldext_dn11 = 0.0;
        locals.var_qbdldext_dn14 = 0.0;
        locals.var_qbdldext_rv = 0.0;

        locals.var_qbsldext = 0.0;
        locals.var_qbsldext_dn0 = 0.0;
        locals.var_qbsldext_dn2 = 0.0;
        locals.var_qbsldext_dn4 = 0.0;
        locals.var_qbsldext_dn5 = 0.0;
        locals.var_qbsldext_dn6 = 0.0;
        locals.var_qbsldext_dn7 = 0.0;
        locals.var_qbsldext_dn8 = 0.0;
        locals.var_qbsldext_dn9 = 0.0;
        locals.var_qbsldext_dn10 = 0.0;
        locals.var_qbsldext_dn11 = 0.0;
        locals.var_qbsldext_dn14 = 0.0;
        locals.var_qbsldext_rv = 0.0;

        locals.var_vbsz2 = 0.0;
        locals.var_vbsz2_dn0 = 0.0;
        locals.var_vbsz2_dn2 = 0.0;
        locals.var_vbsz2_dn4 = 0.0;
        locals.var_vbsz2_dn5 = 0.0;
        locals.var_vbsz2_dn6 = 0.0;
        locals.var_vbsz2_dn7 = 0.0;
        locals.var_vbsz2_dn8 = 0.0;
        locals.var_vbsz2_dn9 = 0.0;
        locals.var_vbsz2_dn10 = 0.0;
        locals.var_vbsz2_dn11 = 0.0;
        locals.var_vbsz2_dn14 = 0.0;
        locals.var_vbsz2_rv = 0.0;

        locals.var_rdrift = 0.0;
        locals.var_rdrift_dn0 = 0.0;
        locals.var_rdrift_dn2 = 0.0;
        locals.var_rdrift_dn4 = 0.0;
        locals.var_rdrift_dn5 = 0.0;
        locals.var_rdrift_dn6 = 0.0;
        locals.var_rdrift_dn7 = 0.0;
        locals.var_rdrift_dn8 = 0.0;
        locals.var_rdrift_dn9 = 0.0;
        locals.var_rdrift_dn10 = 0.0;
        locals.var_rdrift_dn11 = 0.0;
        locals.var_rdrift_dn14 = 0.0;
        locals.var_rdrift_rv = 0.0;

        locals.var_rsdrift = 0.0;
        locals.var_rsdrift_dn0 = 0.0;
        locals.var_rsdrift_dn2 = 0.0;
        locals.var_rsdrift_dn4 = 0.0;
        locals.var_rsdrift_dn5 = 0.0;
        locals.var_rsdrift_dn6 = 0.0;
        locals.var_rsdrift_dn7 = 0.0;
        locals.var_rsdrift_dn8 = 0.0;
        locals.var_rsdrift_dn9 = 0.0;
        locals.var_rsdrift_dn10 = 0.0;
        locals.var_rsdrift_dn11 = 0.0;
        locals.var_rsdrift_dn14 = 0.0;
        locals.var_rsdrift_rv = 0.0;

        locals.var_ra = 0.0;
        locals.var_ra_dn0 = 0.0;
        locals.var_ra_dn2 = 0.0;
        locals.var_ra_dn4 = 0.0;
        locals.var_ra_dn5 = 0.0;
        locals.var_ra_dn6 = 0.0;
        locals.var_ra_dn7 = 0.0;
        locals.var_ra_dn8 = 0.0;
        locals.var_ra_dn9 = 0.0;
        locals.var_ra_dn10 = 0.0;
        locals.var_ra_dn11 = 0.0;
        locals.var_ra_dn14 = 0.0;
        locals.var_ra_rv = 0.0;

        locals.var_vdse_eff = 0.0;
        locals.var_vdse_eff_dn0 = 0.0;
        locals.var_vdse_eff_dn2 = 0.0;
        locals.var_vdse_eff_rv = 0.0;

        locals.var_vdsemodenml = 0.0;
        locals.var_vdsemodenml_rv = 0.0;

        locals.var_vdsemodervs = 0.0;
        locals.var_vdsemodervs_rv = 0.0;

        locals.var_vbsegmt = 0.0;
        locals.var_vbsegmt_dn2 = 0.0;
        locals.var_vbsegmt_dn9 = 0.0;
        locals.var_vbsegmt_rv = 0.0;

        locals.var_vdsegmt = 0.0;
        locals.var_vdsegmt_dn0 = 0.0;
        locals.var_vdsegmt_dn2 = 0.0;
        locals.var_vdsegmt_rv = 0.0;

        locals.var_vgsegmt = 0.0;
        locals.var_vgsegmt_dn2 = 0.0;
        locals.var_vgsegmt_dn7 = 0.0;
        locals.var_vgsegmt_rv = 0.0;

        locals.var_vbserev = 0.0;
        locals.var_vbserev_dn0 = 0.0;
        locals.var_vbserev_dn2 = 0.0;
        locals.var_vbserev_dn9 = 0.0;
        locals.var_vbserev_rv = 0.0;

        locals.var_vdserev = 0.0;
        locals.var_vdserev_dn0 = 0.0;
        locals.var_vdserev_dn2 = 0.0;
        locals.var_vdserev_rv = 0.0;

        locals.var_vgserev = 0.0;
        locals.var_vgserev_dn0 = 0.0;
        locals.var_vgserev_dn2 = 0.0;
        locals.var_vgserev_dn7 = 0.0;
        locals.var_vgserev_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_vdserevz = 0.0;
        locals.var_vdserevz_dn0 = 0.0;
        locals.var_vdserevz_dn2 = 0.0;
        locals.var_vdserevz_dn4 = 0.0;
        locals.var_vdserevz_dn5 = 0.0;
        locals.var_vdserevz_dn6 = 0.0;
        locals.var_vdserevz_dn7 = 0.0;
        locals.var_vdserevz_dn8 = 0.0;
        locals.var_vdserevz_dn9 = 0.0;
        locals.var_vdserevz_dn10 = 0.0;
        locals.var_vdserevz_dn11 = 0.0;
        locals.var_vdserevz_dn14 = 0.0;
        locals.var_vdserevz_rv = 0.0;

        locals.var_vgserevz = 0.0;
        locals.var_vgserevz_dn0 = 0.0;
        locals.var_vgserevz_dn2 = 0.0;
        locals.var_vgserevz_dn4 = 0.0;
        locals.var_vgserevz_dn5 = 0.0;
        locals.var_vgserevz_dn6 = 0.0;
        locals.var_vgserevz_dn7 = 0.0;
        locals.var_vgserevz_dn8 = 0.0;
        locals.var_vgserevz_dn9 = 0.0;
        locals.var_vgserevz_dn10 = 0.0;
        locals.var_vgserevz_dn11 = 0.0;
        locals.var_vgserevz_dn14 = 0.0;
        locals.var_vgserevz_rv = 0.0;

        locals.var_vbserevz = 0.0;
        locals.var_vbserevz_dn0 = 0.0;
        locals.var_vbserevz_dn2 = 0.0;
        locals.var_vbserevz_dn4 = 0.0;
        locals.var_vbserevz_dn5 = 0.0;
        locals.var_vbserevz_dn6 = 0.0;
        locals.var_vbserevz_dn7 = 0.0;
        locals.var_vbserevz_dn8 = 0.0;
        locals.var_vbserevz_dn9 = 0.0;
        locals.var_vbserevz_dn10 = 0.0;
        locals.var_vbserevz_dn11 = 0.0;
        locals.var_vbserevz_dn14 = 0.0;
        locals.var_vbserevz_rv = 0.0;

        locals.var_vsubsrev = 0.0;
        locals.var_vsubsrev_dn0 = 0.0;
        locals.var_vsubsrev_dn2 = 0.0;
        locals.var_vsubsrev_dn4 = 0.0;
        locals.var_vsubsrev_rv = 0.0;

        locals.var_ttemp = 0.0;
        locals.var_ttemp_dn0 = 0.0;
        locals.var_ttemp_dn2 = 0.0;
        locals.var_ttemp_dn4 = 0.0;
        locals.var_ttemp_dn5 = 0.0;
        locals.var_ttemp_dn6 = 0.0;
        locals.var_ttemp_dn7 = 0.0;
        locals.var_ttemp_dn8 = 0.0;
        locals.var_ttemp_dn9 = 0.0;
        locals.var_ttemp_dn10 = 0.0;
        locals.var_ttemp_dn11 = 0.0;
        locals.var_ttemp_dn14 = 0.0;
        locals.var_ttemp_rv = 0.0;

        locals.var_ttemp0 = 0.0;
        locals.var_ttemp0_dn0 = 0.0;
        locals.var_ttemp0_dn2 = 0.0;
        locals.var_ttemp0_dn4 = 0.0;
        locals.var_ttemp0_dn5 = 0.0;
        locals.var_ttemp0_dn6 = 0.0;
        locals.var_ttemp0_dn7 = 0.0;
        locals.var_ttemp0_dn8 = 0.0;
        locals.var_ttemp0_dn9 = 0.0;
        locals.var_ttemp0_dn10 = 0.0;
        locals.var_ttemp0_dn11 = 0.0;
        locals.var_ttemp0_dn14 = 0.0;
        locals.var_ttemp0_rv = 0.0;

        locals.var_tdiff0 = 0.0;
        locals.var_tdiff0_dn0 = 0.0;
        locals.var_tdiff0_dn2 = 0.0;
        locals.var_tdiff0_dn4 = 0.0;
        locals.var_tdiff0_dn5 = 0.0;
        locals.var_tdiff0_dn6 = 0.0;
        locals.var_tdiff0_dn7 = 0.0;
        locals.var_tdiff0_dn8 = 0.0;
        locals.var_tdiff0_dn9 = 0.0;
        locals.var_tdiff0_dn10 = 0.0;
        locals.var_tdiff0_dn11 = 0.0;
        locals.var_tdiff0_dn14 = 0.0;
        locals.var_tdiff0_rv = 0.0;

        locals.var_tdiff0_2 = 0.0;
        locals.var_tdiff0_2_dn0 = 0.0;
        locals.var_tdiff0_2_dn2 = 0.0;
        locals.var_tdiff0_2_dn4 = 0.0;
        locals.var_tdiff0_2_dn5 = 0.0;
        locals.var_tdiff0_2_dn6 = 0.0;
        locals.var_tdiff0_2_dn7 = 0.0;
        locals.var_tdiff0_2_dn8 = 0.0;
        locals.var_tdiff0_2_dn9 = 0.0;
        locals.var_tdiff0_2_dn10 = 0.0;
        locals.var_tdiff0_2_dn11 = 0.0;
        locals.var_tdiff0_2_dn14 = 0.0;
        locals.var_tdiff0_2_rv = 0.0;

        locals.var_tdiff = 0.0;
        locals.var_tdiff_dn0 = 0.0;
        locals.var_tdiff_dn2 = 0.0;
        locals.var_tdiff_dn4 = 0.0;
        locals.var_tdiff_dn5 = 0.0;
        locals.var_tdiff_dn6 = 0.0;
        locals.var_tdiff_dn7 = 0.0;
        locals.var_tdiff_dn8 = 0.0;
        locals.var_tdiff_dn9 = 0.0;
        locals.var_tdiff_dn10 = 0.0;
        locals.var_tdiff_dn11 = 0.0;
        locals.var_tdiff_dn14 = 0.0;
        locals.var_tdiff_rv = 0.0;

        locals.var_tdiff_2 = 0.0;
        locals.var_tdiff_2_dn0 = 0.0;
        locals.var_tdiff_2_dn2 = 0.0;
        locals.var_tdiff_2_dn4 = 0.0;
        locals.var_tdiff_2_dn5 = 0.0;
        locals.var_tdiff_2_dn6 = 0.0;
        locals.var_tdiff_2_dn7 = 0.0;
        locals.var_tdiff_2_dn8 = 0.0;
        locals.var_tdiff_2_dn9 = 0.0;
        locals.var_tdiff_2_dn10 = 0.0;
        locals.var_tdiff_2_dn11 = 0.0;
        locals.var_tdiff_2_dn14 = 0.0;
        locals.var_tdiff_2_rv = 0.0;

        locals.var_eg = 0.0;
        locals.var_eg_dn0 = 0.0;
        locals.var_eg_dn2 = 0.0;
        locals.var_eg_dn4 = 0.0;
        locals.var_eg_dn5 = 0.0;
        locals.var_eg_dn6 = 0.0;
        locals.var_eg_dn7 = 0.0;
        locals.var_eg_dn8 = 0.0;
        locals.var_eg_dn9 = 0.0;
        locals.var_eg_dn10 = 0.0;
        locals.var_eg_dn11 = 0.0;
        locals.var_eg_dn14 = 0.0;
        locals.var_eg_rv = 0.0;

        locals.var_nin = 0.0;
        locals.var_nin_dn0 = 0.0;
        locals.var_nin_dn2 = 0.0;
        locals.var_nin_dn4 = 0.0;
        locals.var_nin_dn5 = 0.0;
        locals.var_nin_dn6 = 0.0;
        locals.var_nin_dn7 = 0.0;
        locals.var_nin_dn8 = 0.0;
        locals.var_nin_dn9 = 0.0;
        locals.var_nin_dn10 = 0.0;
        locals.var_nin_dn11 = 0.0;
        locals.var_nin_dn14 = 0.0;
        locals.var_nin_rv = 0.0;

        locals.var_vgbgmt = 0.0;
        locals.var_vgbgmt_dn2 = 0.0;
        locals.var_vgbgmt_dn7 = 0.0;
        locals.var_vgbgmt_dn8 = 0.0;
        locals.var_vgbgmt_dn9 = 0.0;
        locals.var_vgbgmt_rv = 0.0;

        locals.var_vxbgmt = 0.0;
        locals.var_vxbgmt_dn0 = 0.0;
        locals.var_vxbgmt_dn2 = 0.0;
        locals.var_vxbgmt_dn4 = 0.0;
        locals.var_vxbgmt_dn5 = 0.0;
        locals.var_vxbgmt_dn6 = 0.0;
        locals.var_vxbgmt_dn7 = 0.0;
        locals.var_vxbgmt_dn8 = 0.0;
        locals.var_vxbgmt_dn9 = 0.0;
        locals.var_vxbgmt_dn10 = 0.0;
        locals.var_vxbgmt_dn11 = 0.0;
        locals.var_vxbgmt_dn14 = 0.0;
        locals.var_vxbgmt_rv = 0.0;

        locals.var_vxbgmtcl = 0.0;
        locals.var_vxbgmtcl_dn0 = 0.0;
        locals.var_vxbgmtcl_dn2 = 0.0;
        locals.var_vxbgmtcl_dn4 = 0.0;
        locals.var_vxbgmtcl_dn5 = 0.0;
        locals.var_vxbgmtcl_dn6 = 0.0;
        locals.var_vxbgmtcl_dn7 = 0.0;
        locals.var_vxbgmtcl_dn8 = 0.0;
        locals.var_vxbgmtcl_dn9 = 0.0;
        locals.var_vxbgmtcl_dn10 = 0.0;
        locals.var_vxbgmtcl_dn11 = 0.0;
        locals.var_vxbgmtcl_dn14 = 0.0;
        locals.var_vxbgmtcl_rv = 0.0;

        locals.var_qsuld = 0.0;
        locals.var_qsuld_dn0 = 0.0;
        locals.var_qsuld_dn2 = 0.0;
        locals.var_qsuld_dn4 = 0.0;
        locals.var_qsuld_dn5 = 0.0;
        locals.var_qsuld_dn6 = 0.0;
        locals.var_qsuld_dn7 = 0.0;
        locals.var_qsuld_dn8 = 0.0;
        locals.var_qsuld_dn9 = 0.0;
        locals.var_qsuld_dn10 = 0.0;
        locals.var_qsuld_dn11 = 0.0;
        locals.var_qsuld_dn14 = 0.0;
        locals.var_qsuld_rv = 0.0;

        locals.var_qiuld = 0.0;
        locals.var_qiuld_dn0 = 0.0;
        locals.var_qiuld_dn2 = 0.0;
        locals.var_qiuld_dn4 = 0.0;
        locals.var_qiuld_dn5 = 0.0;
        locals.var_qiuld_dn6 = 0.0;
        locals.var_qiuld_dn7 = 0.0;
        locals.var_qiuld_dn8 = 0.0;
        locals.var_qiuld_dn9 = 0.0;
        locals.var_qiuld_dn10 = 0.0;
        locals.var_qiuld_dn11 = 0.0;
        locals.var_qiuld_dn14 = 0.0;
        locals.var_qiuld_rv = 0.0;

        locals.var_idsibpc = 0.0;
        locals.var_idsibpc_dn0 = 0.0;
        locals.var_idsibpc_dn2 = 0.0;
        locals.var_idsibpc_dn4 = 0.0;
        locals.var_idsibpc_dn5 = 0.0;
        locals.var_idsibpc_dn6 = 0.0;
        locals.var_idsibpc_dn7 = 0.0;
        locals.var_idsibpc_dn8 = 0.0;
        locals.var_idsibpc_dn9 = 0.0;
        locals.var_idsibpc_dn10 = 0.0;
        locals.var_idsibpc_dn11 = 0.0;
        locals.var_idsibpc_dn14 = 0.0;
        locals.var_idsibpc_rv = 0.0;

        locals.var_vgpld = 0.0;
        locals.var_vgpld_dn2 = 0.0;
        locals.var_vgpld_dn7 = 0.0;
        locals.var_vgpld_dn8 = 0.0;
        locals.var_vgpld_dn9 = 0.0;
        locals.var_vgpld_rv = 0.0;

        locals.var_vgb_fb_ld = 0.0;
        locals.var_vgb_fb_ld_dn0 = 0.0;
        locals.var_vgb_fb_ld_dn2 = 0.0;
        locals.var_vgb_fb_ld_dn4 = 0.0;
        locals.var_vgb_fb_ld_dn5 = 0.0;
        locals.var_vgb_fb_ld_dn6 = 0.0;
        locals.var_vgb_fb_ld_dn7 = 0.0;
        locals.var_vgb_fb_ld_dn8 = 0.0;
        locals.var_vgb_fb_ld_dn9 = 0.0;
        locals.var_vgb_fb_ld_dn10 = 0.0;
        locals.var_vgb_fb_ld_dn11 = 0.0;
        locals.var_vgb_fb_ld_dn14 = 0.0;
        locals.var_vgb_fb_ld_rv = 0.0;

        locals.var_ps0ld = 0.0;
        locals.var_ps0ld_dn0 = 0.0;
        locals.var_ps0ld_dn2 = 0.0;
        locals.var_ps0ld_dn4 = 0.0;
        locals.var_ps0ld_dn5 = 0.0;
        locals.var_ps0ld_dn6 = 0.0;
        locals.var_ps0ld_dn7 = 0.0;
        locals.var_ps0ld_dn8 = 0.0;
        locals.var_ps0ld_dn9 = 0.0;
        locals.var_ps0ld_dn10 = 0.0;
        locals.var_ps0ld_dn11 = 0.0;
        locals.var_ps0ld_dn14 = 0.0;
        locals.var_ps0ld_rv = 0.0;

        locals.var_cnst1over = 0.0;
        locals.var_cnst1over_dn0 = 0.0;
        locals.var_cnst1over_dn2 = 0.0;
        locals.var_cnst1over_dn4 = 0.0;
        locals.var_cnst1over_dn5 = 0.0;
        locals.var_cnst1over_dn6 = 0.0;
        locals.var_cnst1over_dn7 = 0.0;
        locals.var_cnst1over_dn8 = 0.0;
        locals.var_cnst1over_dn9 = 0.0;
        locals.var_cnst1over_dn10 = 0.0;
        locals.var_cnst1over_dn11 = 0.0;
        locals.var_cnst1over_dn14 = 0.0;
        locals.var_cnst1over_rv = 0.0;

        locals.var_ddriftld = p.p334;
        locals.var_ddriftld_dn0 = 0.0;
        locals.var_ddriftld_dn2 = 0.0;
        locals.var_ddriftld_dn4 = 0.0;
        locals.var_ddriftld_dn5 = 0.0;
        locals.var_ddriftld_dn6 = 0.0;
        locals.var_ddriftld_dn7 = 0.0;
        locals.var_ddriftld_dn8 = 0.0;
        locals.var_ddriftld_dn9 = 0.0;
        locals.var_ddriftld_dn10 = 0.0;
        locals.var_ddriftld_dn11 = 0.0;
        locals.var_ddriftld_dn14 = 0.0;
        locals.var_ddriftld_rv = 0.0;

        locals.var_ddriftldc = p.p334;
        locals.var_ddriftldc_dn0 = 0.0;
        locals.var_ddriftldc_dn2 = 0.0;
        locals.var_ddriftldc_dn4 = 0.0;
        locals.var_ddriftldc_dn5 = 0.0;
        locals.var_ddriftldc_dn6 = 0.0;
        locals.var_ddriftldc_dn7 = 0.0;
        locals.var_ddriftldc_dn8 = 0.0;
        locals.var_ddriftldc_dn9 = 0.0;
        locals.var_ddriftldc_dn10 = 0.0;
        locals.var_ddriftldc_dn11 = 0.0;
        locals.var_ddriftldc_dn14 = 0.0;
        locals.var_ddriftldc_rv = 0.0;

        locals.var_nover_func = 0.0;
        locals.var_nover_func_rv = 0.0;

        locals.var_cnst0over_func = 0.0;
        locals.var_cnst0over_func_dn0 = 0.0;
        locals.var_cnst0over_func_dn2 = 0.0;
        locals.var_cnst0over_func_dn4 = 0.0;
        locals.var_cnst0over_func_dn5 = 0.0;
        locals.var_cnst0over_func_dn6 = 0.0;
        locals.var_cnst0over_func_dn7 = 0.0;
        locals.var_cnst0over_func_dn8 = 0.0;
        locals.var_cnst0over_func_dn9 = 0.0;
        locals.var_cnst0over_func_dn10 = 0.0;
        locals.var_cnst0over_func_dn11 = 0.0;
        locals.var_cnst0over_func_dn14 = 0.0;
        locals.var_cnst0over_func_rv = 0.0;

        locals.var_ta = 0.0093868;
        locals.var_ta_rv = 0.0;

        let assign3360_e1746: f64 = (-0.1047839);
        locals.var_tb = assign3360_e1746;
        locals.var_tb_rv = 0.0;

        locals.var_chi_1 = 0.0;
        locals.var_chi_1_dn0 = 0.0;
        locals.var_chi_1_dn2 = 0.0;
        locals.var_chi_1_dn4 = 0.0;
        locals.var_chi_1_dn5 = 0.0;
        locals.var_chi_1_dn6 = 0.0;
        locals.var_chi_1_dn7 = 0.0;
        locals.var_chi_1_dn8 = 0.0;
        locals.var_chi_1_dn9 = 0.0;
        locals.var_chi_1_dn10 = 0.0;
        locals.var_chi_1_dn11 = 0.0;
        locals.var_chi_1_dn14 = 0.0;
        locals.var_chi_1_rv = 0.0;

        locals.var_mueph = 0.0;
        locals.var_mueph_dn0 = 0.0;
        locals.var_mueph_dn2 = 0.0;
        locals.var_mueph_dn4 = 0.0;
        locals.var_mueph_dn5 = 0.0;
        locals.var_mueph_dn6 = 0.0;
        locals.var_mueph_dn7 = 0.0;
        locals.var_mueph_dn8 = 0.0;
        locals.var_mueph_dn9 = 0.0;
        locals.var_mueph_dn10 = 0.0;
        locals.var_mueph_dn11 = 0.0;
        locals.var_mueph_dn14 = 0.0;
        locals.var_mueph_rv = 0.0;

        locals.var_dl = 0.0;
        locals.var_dl_rv = 0.0;

        locals.var_dlld = 0.0;
        locals.var_dlld_rv = 0.0;

        locals.var_lg = 0.0;
        locals.var_lg_rv = 0.0;

        locals.var_dw = 0.0;
        locals.var_dw_rv = 0.0;

        locals.var_dwld = 0.0;
        locals.var_dwld_rv = 0.0;

        locals.var_dwcv = 0.0;
        locals.var_dwcv_rv = 0.0;

        locals.var_wg = 0.0;
        locals.var_wg_rv = 0.0;

        locals.var_wlg = 0.0;
        locals.var_wlg_rv = 0.0;

        locals.var_lgate = 0.0;
        locals.var_lgate_rv = 0.0;

        locals.var_wgate = 0.0;
        locals.var_wgate_rv = 0.0;

        locals.var_nsubpp = 0.0;
        locals.var_nsubpp_dn0 = 0.0;
        locals.var_nsubpp_dn2 = 0.0;
        locals.var_nsubpp_dn4 = 0.0;
        locals.var_nsubpp_dn5 = 0.0;
        locals.var_nsubpp_dn6 = 0.0;
        locals.var_nsubpp_dn7 = 0.0;
        locals.var_nsubpp_dn8 = 0.0;
        locals.var_nsubpp_dn9 = 0.0;
        locals.var_nsubpp_dn10 = 0.0;
        locals.var_nsubpp_dn11 = 0.0;
        locals.var_nsubpp_dn14 = 0.0;
        locals.var_nsubpp_rv = 0.0;

        locals.var_nsubps = 0.0;
        locals.var_nsubps_dn0 = 0.0;
        locals.var_nsubps_dn2 = 0.0;
        locals.var_nsubps_dn4 = 0.0;
        locals.var_nsubps_dn5 = 0.0;
        locals.var_nsubps_dn6 = 0.0;
        locals.var_nsubps_dn7 = 0.0;
        locals.var_nsubps_dn8 = 0.0;
        locals.var_nsubps_dn9 = 0.0;
        locals.var_nsubps_dn10 = 0.0;
        locals.var_nsubps_dn11 = 0.0;
        locals.var_nsubps_dn14 = 0.0;
        locals.var_nsubps_rv = 0.0;

        locals.var_nsub = 0.0;
        locals.var_nsub_dn0 = 0.0;
        locals.var_nsub_dn2 = 0.0;
        locals.var_nsub_dn4 = 0.0;
        locals.var_nsub_dn5 = 0.0;
        locals.var_nsub_dn6 = 0.0;
        locals.var_nsub_dn7 = 0.0;
        locals.var_nsub_dn8 = 0.0;
        locals.var_nsub_dn9 = 0.0;
        locals.var_nsub_dn10 = 0.0;
        locals.var_nsub_dn11 = 0.0;
        locals.var_nsub_dn14 = 0.0;
        locals.var_nsub_rv = 0.0;

        locals.var_nsubb = 0.0;
        locals.var_nsubb_dn0 = 0.0;
        locals.var_nsubb_dn2 = 0.0;
        locals.var_nsubb_dn4 = 0.0;
        locals.var_nsubb_dn5 = 0.0;
        locals.var_nsubb_dn6 = 0.0;
        locals.var_nsubb_dn7 = 0.0;
        locals.var_nsubb_dn8 = 0.0;
        locals.var_nsubb_dn9 = 0.0;
        locals.var_nsubb_dn10 = 0.0;
        locals.var_nsubb_dn11 = 0.0;
        locals.var_nsubb_dn14 = 0.0;
        locals.var_nsubb_rv = 0.0;

        locals.var_lod_half = 0.0;
        locals.var_lod_half_dn0 = 0.0;
        locals.var_lod_half_dn2 = 0.0;
        locals.var_lod_half_dn4 = 0.0;
        locals.var_lod_half_dn5 = 0.0;
        locals.var_lod_half_dn6 = 0.0;
        locals.var_lod_half_dn7 = 0.0;
        locals.var_lod_half_dn8 = 0.0;
        locals.var_lod_half_dn9 = 0.0;
        locals.var_lod_half_dn10 = 0.0;
        locals.var_lod_half_dn11 = 0.0;
        locals.var_lod_half_dn14 = 0.0;
        locals.var_lod_half_rv = 0.0;

        locals.var_lod_half_ref = 0.0;
        locals.var_lod_half_ref_dn0 = 0.0;
        locals.var_lod_half_ref_dn2 = 0.0;
        locals.var_lod_half_ref_dn4 = 0.0;
        locals.var_lod_half_ref_dn5 = 0.0;
        locals.var_lod_half_ref_dn6 = 0.0;
        locals.var_lod_half_ref_dn7 = 0.0;
        locals.var_lod_half_ref_dn8 = 0.0;
        locals.var_lod_half_ref_dn9 = 0.0;
        locals.var_lod_half_ref_dn10 = 0.0;
        locals.var_lod_half_ref_dn11 = 0.0;
        locals.var_lod_half_ref_dn14 = 0.0;
        locals.var_lod_half_ref_rv = 0.0;

        locals.var_log_tratio = 0.0;
        locals.var_log_tratio_dn0 = 0.0;
        locals.var_log_tratio_dn2 = 0.0;
        locals.var_log_tratio_dn4 = 0.0;
        locals.var_log_tratio_dn5 = 0.0;
        locals.var_log_tratio_dn6 = 0.0;
        locals.var_log_tratio_dn7 = 0.0;
        locals.var_log_tratio_dn8 = 0.0;
        locals.var_log_tratio_dn9 = 0.0;
        locals.var_log_tratio_dn10 = 0.0;
        locals.var_log_tratio_dn11 = 0.0;
        locals.var_log_tratio_dn14 = 0.0;
        locals.var_log_tratio_rv = 0.0;

        locals.var_edri = 0.0;
        locals.var_edri_dn0 = 0.0;
        locals.var_edri_dn2 = 0.0;
        locals.var_edri_dn4 = 0.0;
        locals.var_edri_dn5 = 0.0;
        locals.var_edri_dn6 = 0.0;
        locals.var_edri_dn7 = 0.0;
        locals.var_edri_dn8 = 0.0;
        locals.var_edri_dn9 = 0.0;
        locals.var_edri_dn10 = 0.0;
        locals.var_edri_dn11 = 0.0;
        locals.var_edri_dn14 = 0.0;
        locals.var_edri_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        locals: &mut StampLocals,
    ) {
        locals.var_vdri = 0.0;
        locals.var_vdri_dn0 = 0.0;
        locals.var_vdri_dn2 = 0.0;
        locals.var_vdri_dn4 = 0.0;
        locals.var_vdri_dn5 = 0.0;
        locals.var_vdri_dn6 = 0.0;
        locals.var_vdri_dn7 = 0.0;
        locals.var_vdri_dn8 = 0.0;
        locals.var_vdri_dn9 = 0.0;
        locals.var_vdri_dn10 = 0.0;
        locals.var_vdri_dn11 = 0.0;
        locals.var_vdri_dn14 = 0.0;
        locals.var_vdri_rv = 0.0;

        locals.var_mu0 = 0.0;
        locals.var_mu0_dn0 = 0.0;
        locals.var_mu0_dn2 = 0.0;
        locals.var_mu0_dn4 = 0.0;
        locals.var_mu0_dn5 = 0.0;
        locals.var_mu0_dn6 = 0.0;
        locals.var_mu0_dn7 = 0.0;
        locals.var_mu0_dn8 = 0.0;
        locals.var_mu0_dn9 = 0.0;
        locals.var_mu0_dn10 = 0.0;
        locals.var_mu0_dn11 = 0.0;
        locals.var_mu0_dn14 = 0.0;
        locals.var_mu0_rv = 0.0;

        locals.var_cx = 0.0;
        locals.var_cx_rv = 0.0;

        locals.var_car = 0.0;
        locals.var_car_rv = 0.0;

        locals.var_xov = 0.0;
        locals.var_xov_dn0 = 0.0;
        locals.var_xov_dn2 = 0.0;
        locals.var_xov_dn4 = 0.0;
        locals.var_xov_dn5 = 0.0;
        locals.var_xov_dn6 = 0.0;
        locals.var_xov_dn7 = 0.0;
        locals.var_xov_dn8 = 0.0;
        locals.var_xov_dn9 = 0.0;
        locals.var_xov_dn10 = 0.0;
        locals.var_xov_dn11 = 0.0;
        locals.var_xov_dn14 = 0.0;
        locals.var_xov_rv = 0.0;

        locals.var_carr = 0.0;
        locals.var_carr_dn0 = 0.0;
        locals.var_carr_dn2 = 0.0;
        locals.var_carr_dn4 = 0.0;
        locals.var_carr_dn5 = 0.0;
        locals.var_carr_dn6 = 0.0;
        locals.var_carr_dn7 = 0.0;
        locals.var_carr_dn8 = 0.0;
        locals.var_carr_dn9 = 0.0;
        locals.var_carr_dn10 = 0.0;
        locals.var_carr_dn11 = 0.0;
        locals.var_carr_dn14 = 0.0;
        locals.var_carr_rv = 0.0;

        locals.var_gd = 0.0;
        locals.var_gd_dn0 = 0.0;
        locals.var_gd_dn2 = 0.0;
        locals.var_gd_dn4 = 0.0;
        locals.var_gd_dn5 = 0.0;
        locals.var_gd_dn6 = 0.0;
        locals.var_gd_dn7 = 0.0;
        locals.var_gd_dn8 = 0.0;
        locals.var_gd_dn9 = 0.0;
        locals.var_gd_dn10 = 0.0;
        locals.var_gd_dn11 = 0.0;
        locals.var_gd_dn14 = 0.0;
        locals.var_gd_rv = 0.0;

        locals.var_vddpz = 0.0;
        locals.var_vddpz_dn0 = 0.0;
        locals.var_vddpz_dn2 = 0.0;
        locals.var_vddpz_dn4 = 0.0;
        locals.var_vddpz_dn5 = 0.0;
        locals.var_vddpz_dn6 = 0.0;
        locals.var_vddpz_dn7 = 0.0;
        locals.var_vddpz_dn8 = 0.0;
        locals.var_vddpz_dn9 = 0.0;
        locals.var_vddpz_dn10 = 0.0;
        locals.var_vddpz_dn11 = 0.0;
        locals.var_vddpz_dn14 = 0.0;
        locals.var_vddpz_rv = 0.0;

        locals.var_arg = 0.0;
        locals.var_arg_dn0 = 0.0;
        locals.var_arg_dn2 = 0.0;
        locals.var_arg_dn4 = 0.0;
        locals.var_arg_dn5 = 0.0;
        locals.var_arg_dn6 = 0.0;
        locals.var_arg_dn7 = 0.0;
        locals.var_arg_dn8 = 0.0;
        locals.var_arg_dn9 = 0.0;
        locals.var_arg_dn10 = 0.0;
        locals.var_arg_dn11 = 0.0;
        locals.var_arg_dn14 = 0.0;
        locals.var_arg_rv = 0.0;

        locals.var_vbd = 0.0;
        locals.var_vbd_dn6 = 0.0;
        locals.var_vbd_dn8 = 0.0;
        locals.var_vbd_dn9 = 0.0;
        locals.var_vbd_rv = 0.0;

        locals.var_vbsi = 0.0;
        locals.var_vbsi_dn8 = 0.0;
        locals.var_vbsi_dn9 = 0.0;
        locals.var_vbsi_rv = 0.0;

        locals.var_vdsi = 0.0;
        locals.var_vdsi_dn6 = 0.0;
        locals.var_vdsi_dn8 = 0.0;
        locals.var_vdsi_rv = 0.0;

        locals.var_vgd = 0.0;
        locals.var_vgd_dn6 = 0.0;
        locals.var_vgd_dn7 = 0.0;
        locals.var_vgd_dn8 = 0.0;
        locals.var_vgd_rv = 0.0;

        locals.var_vgsi = 0.0;
        locals.var_vgsi_dn7 = 0.0;
        locals.var_vgsi_dn8 = 0.0;
        locals.var_vgsi_rv = 0.0;

        locals.var_deltemp = 0.0;
        locals.var_deltemp_dn0 = 0.0;
        locals.var_deltemp_dn2 = 0.0;
        locals.var_deltemp_dn4 = 0.0;
        locals.var_deltemp_dn5 = 0.0;
        locals.var_deltemp_dn6 = 0.0;
        locals.var_deltemp_dn7 = 0.0;
        locals.var_deltemp_dn8 = 0.0;
        locals.var_deltemp_dn9 = 0.0;
        locals.var_deltemp_dn10 = 0.0;
        locals.var_deltemp_dn11 = 0.0;
        locals.var_deltemp_dn14 = 0.0;
        locals.var_deltemp_rv = 0.0;

        locals.var_vdsei = 0.0;
        locals.var_vdsei_dn0 = 0.0;
        locals.var_vdsei_dn2 = 0.0;
        locals.var_vdsei_rv = 0.0;

        locals.var_vgsei = 0.0;
        locals.var_vgsei_dn2 = 0.0;
        locals.var_vgsei_dn7 = 0.0;
        locals.var_vgsei_rv = 0.0;

        locals.var_vbsei = 0.0;
        locals.var_vbsei_dn2 = 0.0;
        locals.var_vbsei_dn9 = 0.0;
        locals.var_vbsei_rv = 0.0;

        locals.var_gth = 0.0;
        locals.var_gth_dn0 = 0.0;
        locals.var_gth_dn2 = 0.0;
        locals.var_gth_dn4 = 0.0;
        locals.var_gth_dn5 = 0.0;
        locals.var_gth_dn6 = 0.0;
        locals.var_gth_dn7 = 0.0;
        locals.var_gth_dn8 = 0.0;
        locals.var_gth_dn9 = 0.0;
        locals.var_gth_dn10 = 0.0;
        locals.var_gth_dn11 = 0.0;
        locals.var_gth_dn14 = 0.0;
        locals.var_gth_rv = 0.0;

        locals.var_qg = 0.0;
        locals.var_qg_dn0 = 0.0;
        locals.var_qg_dn2 = 0.0;
        locals.var_qg_dn4 = 0.0;
        locals.var_qg_dn5 = 0.0;
        locals.var_qg_dn6 = 0.0;
        locals.var_qg_dn7 = 0.0;
        locals.var_qg_dn8 = 0.0;
        locals.var_qg_dn9 = 0.0;
        locals.var_qg_dn10 = 0.0;
        locals.var_qg_dn11 = 0.0;
        locals.var_qg_dn14 = 0.0;
        locals.var_qg_rv = 0.0;

        locals.var_qs = 0.0;
        locals.var_qs_dn0 = 0.0;
        locals.var_qs_dn2 = 0.0;
        locals.var_qs_dn4 = 0.0;
        locals.var_qs_dn5 = 0.0;
        locals.var_qs_dn6 = 0.0;
        locals.var_qs_dn7 = 0.0;
        locals.var_qs_dn8 = 0.0;
        locals.var_qs_dn9 = 0.0;
        locals.var_qs_dn10 = 0.0;
        locals.var_qs_dn11 = 0.0;
        locals.var_qs_dn14 = 0.0;
        locals.var_qs_rv = 0.0;

        locals.var_veffpower = 0.0;
        locals.var_veffpower_dn0 = 0.0;
        locals.var_veffpower_dn2 = 0.0;
        locals.var_veffpower_dn4 = 0.0;
        locals.var_veffpower_dn5 = 0.0;
        locals.var_veffpower_dn6 = 0.0;
        locals.var_veffpower_dn7 = 0.0;
        locals.var_veffpower_dn8 = 0.0;
        locals.var_veffpower_dn9 = 0.0;
        locals.var_veffpower_dn10 = 0.0;
        locals.var_veffpower_dn11 = 0.0;
        locals.var_veffpower_dn14 = 0.0;
        locals.var_veffpower_rv = 0.0;

        locals.var_p = 0.0;
        locals.var_p_dn0 = 0.0;
        locals.var_p_dn2 = 0.0;
        locals.var_p_dn4 = 0.0;
        locals.var_p_dn5 = 0.0;
        locals.var_p_dn6 = 0.0;
        locals.var_p_dn7 = 0.0;
        locals.var_p_dn8 = 0.0;
        locals.var_p_dn9 = 0.0;
        locals.var_p_dn10 = 0.0;
        locals.var_p_dn11 = 0.0;
        locals.var_p_dn14 = 0.0;
        locals.var_p_rv = 0.0;

        locals.var_qi_nqs = 0.0;
        locals.var_qi_nqs_dn12 = 0.0;
        locals.var_qi_nqs_rv = 0.0;

        locals.var_qb_nqs = 0.0;
        locals.var_qb_nqs_dn13 = 0.0;
        locals.var_qb_nqs_rv = 0.0;

        locals.var_qd_nqs = 0.0;
        locals.var_qd_nqs_dn0 = 0.0;
        locals.var_qd_nqs_dn2 = 0.0;
        locals.var_qd_nqs_dn4 = 0.0;
        locals.var_qd_nqs_dn5 = 0.0;
        locals.var_qd_nqs_dn6 = 0.0;
        locals.var_qd_nqs_dn7 = 0.0;
        locals.var_qd_nqs_dn8 = 0.0;
        locals.var_qd_nqs_dn9 = 0.0;
        locals.var_qd_nqs_dn10 = 0.0;
        locals.var_qd_nqs_dn11 = 0.0;
        locals.var_qd_nqs_dn12 = 0.0;
        locals.var_qd_nqs_dn14 = 0.0;
        locals.var_qd_nqs_rv = 0.0;

        locals.var_qs_nqs = 0.0;
        locals.var_qs_nqs_dn0 = 0.0;
        locals.var_qs_nqs_dn2 = 0.0;
        locals.var_qs_nqs_dn4 = 0.0;
        locals.var_qs_nqs_dn5 = 0.0;
        locals.var_qs_nqs_dn6 = 0.0;
        locals.var_qs_nqs_dn7 = 0.0;
        locals.var_qs_nqs_dn8 = 0.0;
        locals.var_qs_nqs_dn9 = 0.0;
        locals.var_qs_nqs_dn10 = 0.0;
        locals.var_qs_nqs_dn11 = 0.0;
        locals.var_qs_nqs_dn12 = 0.0;
        locals.var_qs_nqs_dn14 = 0.0;
        locals.var_qs_nqs_rv = 0.0;

        locals.var_qg_nqs = 0.0;
        locals.var_qg_nqs_dn12 = 0.0;
        locals.var_qg_nqs_dn13 = 0.0;
        locals.var_qg_nqs_rv = 0.0;

        locals.var_cgsb = 0.0;
        locals.var_cgsb_dn0 = 0.0;
        locals.var_cgsb_dn2 = 0.0;
        locals.var_cgsb_dn4 = 0.0;
        locals.var_cgsb_dn5 = 0.0;
        locals.var_cgsb_dn6 = 0.0;
        locals.var_cgsb_dn7 = 0.0;
        locals.var_cgsb_dn8 = 0.0;
        locals.var_cgsb_dn9 = 0.0;
        locals.var_cgsb_dn10 = 0.0;
        locals.var_cgsb_dn11 = 0.0;
        locals.var_cgsb_dn14 = 0.0;
        locals.var_cgsb_rv = 0.0;

        locals.var_ninvde = 0.0;
        locals.var_ninvde_dn0 = 0.0;
        locals.var_ninvde_dn2 = 0.0;
        locals.var_ninvde_dn4 = 0.0;
        locals.var_ninvde_dn5 = 0.0;
        locals.var_ninvde_dn6 = 0.0;
        locals.var_ninvde_dn7 = 0.0;
        locals.var_ninvde_dn8 = 0.0;
        locals.var_ninvde_dn9 = 0.0;
        locals.var_ninvde_dn10 = 0.0;
        locals.var_ninvde_dn11 = 0.0;
        locals.var_ninvde_dn14 = 0.0;
        locals.var_ninvde_rv = 0.0;

        locals.var_ninvdecres = 0.0;
        locals.var_ninvdecres_dn0 = 0.0;
        locals.var_ninvdecres_dn2 = 0.0;
        locals.var_ninvdecres_dn4 = 0.0;
        locals.var_ninvdecres_dn5 = 0.0;
        locals.var_ninvdecres_dn6 = 0.0;
        locals.var_ninvdecres_dn7 = 0.0;
        locals.var_ninvdecres_dn8 = 0.0;
        locals.var_ninvdecres_dn9 = 0.0;
        locals.var_ninvdecres_dn10 = 0.0;
        locals.var_ninvdecres_dn11 = 0.0;
        locals.var_ninvdecres_dn14 = 0.0;
        locals.var_ninvdecres_rv = 0.0;

        locals.var_ninvdehres = 0.0;
        locals.var_ninvdehres_dn0 = 0.0;
        locals.var_ninvdehres_dn2 = 0.0;
        locals.var_ninvdehres_dn4 = 0.0;
        locals.var_ninvdehres_dn5 = 0.0;
        locals.var_ninvdehres_dn6 = 0.0;
        locals.var_ninvdehres_dn7 = 0.0;
        locals.var_ninvdehres_dn8 = 0.0;
        locals.var_ninvdehres_dn9 = 0.0;
        locals.var_ninvdehres_dn10 = 0.0;
        locals.var_ninvdehres_dn11 = 0.0;
        locals.var_ninvdehres_dn14 = 0.0;
        locals.var_ninvdehres_rv = 0.0;

        locals.var_rrdrmue = 0.0;
        locals.var_rrdrmue_dn0 = 0.0;
        locals.var_rrdrmue_dn2 = 0.0;
        locals.var_rrdrmue_dn4 = 0.0;
        locals.var_rrdrmue_dn5 = 0.0;
        locals.var_rrdrmue_dn6 = 0.0;
        locals.var_rrdrmue_dn7 = 0.0;
        locals.var_rrdrmue_dn8 = 0.0;
        locals.var_rrdrmue_dn9 = 0.0;
        locals.var_rrdrmue_dn10 = 0.0;
        locals.var_rrdrmue_dn11 = 0.0;
        locals.var_rrdrmue_dn14 = 0.0;
        locals.var_rrdrmue_rv = 0.0;

        locals.var_rrdrmues = 0.0;
        locals.var_rrdrmues_dn0 = 0.0;
        locals.var_rrdrmues_dn2 = 0.0;
        locals.var_rrdrmues_dn4 = 0.0;
        locals.var_rrdrmues_dn5 = 0.0;
        locals.var_rrdrmues_dn6 = 0.0;
        locals.var_rrdrmues_dn7 = 0.0;
        locals.var_rrdrmues_dn8 = 0.0;
        locals.var_rrdrmues_dn9 = 0.0;
        locals.var_rrdrmues_dn10 = 0.0;
        locals.var_rrdrmues_dn11 = 0.0;
        locals.var_rrdrmues_dn14 = 0.0;
        locals.var_rrdrmues_rv = 0.0;

        locals.var_rrdrvmax = 0.0;
        locals.var_rrdrvmax_dn0 = 0.0;
        locals.var_rrdrvmax_dn2 = 0.0;
        locals.var_rrdrvmax_dn4 = 0.0;
        locals.var_rrdrvmax_dn5 = 0.0;
        locals.var_rrdrvmax_dn6 = 0.0;
        locals.var_rrdrvmax_dn7 = 0.0;
        locals.var_rrdrvmax_dn8 = 0.0;
        locals.var_rrdrvmax_dn9 = 0.0;
        locals.var_rrdrvmax_dn10 = 0.0;
        locals.var_rrdrvmax_dn11 = 0.0;
        locals.var_rrdrvmax_dn14 = 0.0;
        locals.var_rrdrvmax_rv = 0.0;

        locals.var_rde = 0.0;
        locals.var_rde_dn0 = 0.0;
        locals.var_rde_dn2 = 0.0;
        locals.var_rde_dn4 = 0.0;
        locals.var_rde_dn5 = 0.0;
        locals.var_rde_dn6 = 0.0;
        locals.var_rde_dn7 = 0.0;
        locals.var_rde_dn8 = 0.0;
        locals.var_rde_dn9 = 0.0;
        locals.var_rde_dn10 = 0.0;
        locals.var_rde_dn11 = 0.0;
        locals.var_rde_dn14 = 0.0;
        locals.var_rde_rv = 0.0;

        locals.var_rdvde = 0.0;
        locals.var_rdvde_dn0 = 0.0;
        locals.var_rdvde_dn2 = 0.0;
        locals.var_rdvde_dn4 = 0.0;
        locals.var_rdvde_dn5 = 0.0;
        locals.var_rdvde_dn6 = 0.0;
        locals.var_rdvde_dn7 = 0.0;
        locals.var_rdvde_dn8 = 0.0;
        locals.var_rdvde_dn9 = 0.0;
        locals.var_rdvde_dn10 = 0.0;
        locals.var_rdvde_dn11 = 0.0;
        locals.var_rdvde_dn14 = 0.0;
        locals.var_rdvde_rv = 0.0;

        locals.var_rse = 0.0;
        locals.var_rse_dn0 = 0.0;
        locals.var_rse_dn2 = 0.0;
        locals.var_rse_dn4 = 0.0;
        locals.var_rse_dn5 = 0.0;
        locals.var_rse_dn6 = 0.0;
        locals.var_rse_dn7 = 0.0;
        locals.var_rse_dn8 = 0.0;
        locals.var_rse_dn9 = 0.0;
        locals.var_rse_dn10 = 0.0;
        locals.var_rse_dn11 = 0.0;
        locals.var_rse_dn14 = 0.0;
        locals.var_rse_rv = 0.0;

        locals.var_rsvde = 0.0;
        locals.var_rsvde_dn0 = 0.0;
        locals.var_rsvde_dn2 = 0.0;
        locals.var_rsvde_dn4 = 0.0;
        locals.var_rsvde_dn5 = 0.0;
        locals.var_rsvde_dn6 = 0.0;
        locals.var_rsvde_dn7 = 0.0;
        locals.var_rsvde_dn8 = 0.0;
        locals.var_rsvde_dn9 = 0.0;
        locals.var_rsvde_dn10 = 0.0;
        locals.var_rsvde_dn11 = 0.0;
        locals.var_rsvde_dn14 = 0.0;
        locals.var_rsvde_rv = 0.0;

        locals.var_rrdrvmaxs = 0.0;
        locals.var_rrdrvmaxs_dn0 = 0.0;
        locals.var_rrdrvmaxs_dn2 = 0.0;
        locals.var_rrdrvmaxs_dn4 = 0.0;
        locals.var_rrdrvmaxs_dn5 = 0.0;
        locals.var_rrdrvmaxs_dn6 = 0.0;
        locals.var_rrdrvmaxs_dn7 = 0.0;
        locals.var_rrdrvmaxs_dn8 = 0.0;
        locals.var_rrdrvmaxs_dn9 = 0.0;
        locals.var_rrdrvmaxs_dn10 = 0.0;
        locals.var_rrdrvmaxs_dn11 = 0.0;
        locals.var_rrdrvmaxs_dn14 = 0.0;
        locals.var_rrdrvmaxs_rv = 0.0;

        locals.var_tratio = 0.0;
        locals.var_tratio_dn0 = 0.0;
        locals.var_tratio_dn2 = 0.0;
        locals.var_tratio_dn4 = 0.0;
        locals.var_tratio_dn5 = 0.0;
        locals.var_tratio_dn6 = 0.0;
        locals.var_tratio_dn7 = 0.0;
        locals.var_tratio_dn8 = 0.0;
        locals.var_tratio_dn9 = 0.0;
        locals.var_tratio_dn10 = 0.0;
        locals.var_tratio_dn11 = 0.0;
        locals.var_tratio_dn14 = 0.0;
        locals.var_tratio_rv = 0.0;

        locals.var_vmaxeff = 0.0;
        locals.var_vmaxeff_dn0 = 0.0;
        locals.var_vmaxeff_dn2 = 0.0;
        locals.var_vmaxeff_dn4 = 0.0;
        locals.var_vmaxeff_dn5 = 0.0;
        locals.var_vmaxeff_dn6 = 0.0;
        locals.var_vmaxeff_dn7 = 0.0;
        locals.var_vmaxeff_dn8 = 0.0;
        locals.var_vmaxeff_dn9 = 0.0;
        locals.var_vmaxeff_dn10 = 0.0;
        locals.var_vmaxeff_dn11 = 0.0;
        locals.var_vmaxeff_dn14 = 0.0;
        locals.var_vmaxeff_rv = 0.0;

        locals.var_betatnom = 0.0;
        locals.var_betatnom_rv = 0.0;

        locals.var_cnst0over = 0.0;
        locals.var_cnst0over_dn0 = 0.0;
        locals.var_cnst0over_dn2 = 0.0;
        locals.var_cnst0over_dn4 = 0.0;
        locals.var_cnst0over_dn5 = 0.0;
        locals.var_cnst0over_dn6 = 0.0;
        locals.var_cnst0over_dn7 = 0.0;
        locals.var_cnst0over_dn8 = 0.0;
        locals.var_cnst0over_dn9 = 0.0;
        locals.var_cnst0over_dn10 = 0.0;
        locals.var_cnst0over_dn11 = 0.0;
        locals.var_cnst0over_dn14 = 0.0;
        locals.var_cnst0over_rv = 0.0;

        locals.var_cnst0overs = 0.0;
        locals.var_cnst0overs_dn0 = 0.0;
        locals.var_cnst0overs_dn2 = 0.0;
        locals.var_cnst0overs_dn4 = 0.0;
        locals.var_cnst0overs_dn5 = 0.0;
        locals.var_cnst0overs_dn6 = 0.0;
        locals.var_cnst0overs_dn7 = 0.0;
        locals.var_cnst0overs_dn8 = 0.0;
        locals.var_cnst0overs_dn9 = 0.0;
        locals.var_cnst0overs_dn10 = 0.0;
        locals.var_cnst0overs_dn11 = 0.0;
        locals.var_cnst0overs_dn14 = 0.0;
        locals.var_cnst0overs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_costi0_p2 = 0.0;
        locals.var_costi0_p2_dn0 = 0.0;
        locals.var_costi0_p2_dn2 = 0.0;
        locals.var_costi0_p2_dn4 = 0.0;
        locals.var_costi0_p2_dn5 = 0.0;
        locals.var_costi0_p2_dn6 = 0.0;
        locals.var_costi0_p2_dn7 = 0.0;
        locals.var_costi0_p2_dn8 = 0.0;
        locals.var_costi0_p2_dn9 = 0.0;
        locals.var_costi0_p2_dn10 = 0.0;
        locals.var_costi0_p2_dn11 = 0.0;
        locals.var_costi0_p2_dn14 = 0.0;
        locals.var_costi0_p2_rv = 0.0;

        locals.var_mphn0 = 0.0;
        locals.var_mphn0_dn0 = 0.0;
        locals.var_mphn0_dn2 = 0.0;
        locals.var_mphn0_dn4 = 0.0;
        locals.var_mphn0_dn5 = 0.0;
        locals.var_mphn0_dn6 = 0.0;
        locals.var_mphn0_dn7 = 0.0;
        locals.var_mphn0_dn8 = 0.0;
        locals.var_mphn0_dn9 = 0.0;
        locals.var_mphn0_dn10 = 0.0;
        locals.var_mphn0_dn11 = 0.0;
        locals.var_mphn0_dn14 = 0.0;
        locals.var_mphn0_rv = 0.0;

        locals.var_powratio = 0.0;
        locals.var_powratio_dn0 = 0.0;
        locals.var_powratio_dn2 = 0.0;
        locals.var_powratio_dn4 = 0.0;
        locals.var_powratio_dn5 = 0.0;
        locals.var_powratio_dn6 = 0.0;
        locals.var_powratio_dn7 = 0.0;
        locals.var_powratio_dn8 = 0.0;
        locals.var_powratio_dn9 = 0.0;
        locals.var_powratio_dn10 = 0.0;
        locals.var_powratio_dn11 = 0.0;
        locals.var_powratio_dn14 = 0.0;
        locals.var_powratio_rv = 0.0;

        locals.var_ptovr = 0.0;
        locals.var_ptovr_dn0 = 0.0;
        locals.var_ptovr_dn2 = 0.0;
        locals.var_ptovr_dn4 = 0.0;
        locals.var_ptovr_dn5 = 0.0;
        locals.var_ptovr_dn6 = 0.0;
        locals.var_ptovr_dn7 = 0.0;
        locals.var_ptovr_dn8 = 0.0;
        locals.var_ptovr_dn9 = 0.0;
        locals.var_ptovr_dn10 = 0.0;
        locals.var_ptovr_dn11 = 0.0;
        locals.var_ptovr_dn14 = 0.0;
        locals.var_ptovr_rv = 0.0;

        locals.var_sqrt_eg = 0.0;
        locals.var_sqrt_eg_dn0 = 0.0;
        locals.var_sqrt_eg_dn2 = 0.0;
        locals.var_sqrt_eg_dn4 = 0.0;
        locals.var_sqrt_eg_dn5 = 0.0;
        locals.var_sqrt_eg_dn6 = 0.0;
        locals.var_sqrt_eg_dn7 = 0.0;
        locals.var_sqrt_eg_dn8 = 0.0;
        locals.var_sqrt_eg_dn9 = 0.0;
        locals.var_sqrt_eg_dn10 = 0.0;
        locals.var_sqrt_eg_dn11 = 0.0;
        locals.var_sqrt_eg_dn14 = 0.0;
        locals.var_sqrt_eg_rv = 0.0;

        locals.var_wdpl = 0.0;
        locals.var_wdpl_dn0 = 0.0;
        locals.var_wdpl_dn2 = 0.0;
        locals.var_wdpl_dn4 = 0.0;
        locals.var_wdpl_dn5 = 0.0;
        locals.var_wdpl_dn6 = 0.0;
        locals.var_wdpl_dn7 = 0.0;
        locals.var_wdpl_dn8 = 0.0;
        locals.var_wdpl_dn9 = 0.0;
        locals.var_wdpl_dn10 = 0.0;
        locals.var_wdpl_dn11 = 0.0;
        locals.var_wdpl_dn14 = 0.0;
        locals.var_wdpl_rv = 0.0;

        locals.var_wdplp = 0.0;
        locals.var_wdplp_dn0 = 0.0;
        locals.var_wdplp_dn2 = 0.0;
        locals.var_wdplp_dn4 = 0.0;
        locals.var_wdplp_dn5 = 0.0;
        locals.var_wdplp_dn6 = 0.0;
        locals.var_wdplp_dn7 = 0.0;
        locals.var_wdplp_dn8 = 0.0;
        locals.var_wdplp_dn9 = 0.0;
        locals.var_wdplp_dn10 = 0.0;
        locals.var_wdplp_dn11 = 0.0;
        locals.var_wdplp_dn14 = 0.0;
        locals.var_wdplp_rv = 0.0;

        locals.var_uc_rdrbb = p.p436;
        locals.var_uc_rdrbb_dn0 = 0.0;
        locals.var_uc_rdrbb_dn2 = 0.0;
        locals.var_uc_rdrbb_dn4 = 0.0;
        locals.var_uc_rdrbb_dn5 = 0.0;
        locals.var_uc_rdrbb_dn6 = 0.0;
        locals.var_uc_rdrbb_dn7 = 0.0;
        locals.var_uc_rdrbb_dn8 = 0.0;
        locals.var_uc_rdrbb_dn9 = 0.0;
        locals.var_uc_rdrbb_dn10 = 0.0;
        locals.var_uc_rdrbb_dn11 = 0.0;
        locals.var_uc_rdrbb_dn14 = 0.0;
        locals.var_uc_rdrbb_rv = 0.0;

        locals.var_uc_rdrbb_s = p.p437;
        locals.var_uc_rdrbb_s_dn0 = 0.0;
        locals.var_uc_rdrbb_s_dn2 = 0.0;
        locals.var_uc_rdrbb_s_dn4 = 0.0;
        locals.var_uc_rdrbb_s_dn5 = 0.0;
        locals.var_uc_rdrbb_s_dn6 = 0.0;
        locals.var_uc_rdrbb_s_dn7 = 0.0;
        locals.var_uc_rdrbb_s_dn8 = 0.0;
        locals.var_uc_rdrbb_s_dn9 = 0.0;
        locals.var_uc_rdrbb_s_dn10 = 0.0;
        locals.var_uc_rdrbb_s_dn11 = 0.0;
        locals.var_uc_rdrbb_s_dn14 = 0.0;
        locals.var_uc_rdrbb_s_rv = 0.0;

        locals.var_ids_acc = 0.0;
        locals.var_ids_acc_dn0 = 0.0;
        locals.var_ids_acc_dn2 = 0.0;
        locals.var_ids_acc_dn4 = 0.0;
        locals.var_ids_acc_dn5 = 0.0;
        locals.var_ids_acc_dn6 = 0.0;
        locals.var_ids_acc_dn7 = 0.0;
        locals.var_ids_acc_dn8 = 0.0;
        locals.var_ids_acc_dn9 = 0.0;
        locals.var_ids_acc_dn10 = 0.0;
        locals.var_ids_acc_dn11 = 0.0;
        locals.var_ids_acc_dn14 = 0.0;
        locals.var_ids_acc_rv = 0.0;

        locals.var_ids_res = 0.0;
        locals.var_ids_res_dn0 = 0.0;
        locals.var_ids_res_dn2 = 0.0;
        locals.var_ids_res_dn4 = 0.0;
        locals.var_ids_res_dn5 = 0.0;
        locals.var_ids_res_dn6 = 0.0;
        locals.var_ids_res_dn7 = 0.0;
        locals.var_ids_res_dn8 = 0.0;
        locals.var_ids_res_dn9 = 0.0;
        locals.var_ids_res_dn10 = 0.0;
        locals.var_ids_res_dn11 = 0.0;
        locals.var_ids_res_dn14 = 0.0;
        locals.var_ids_res_rv = 0.0;

        locals.var_ires_leak = 0.0;
        locals.var_ires_leak_dn0 = 0.0;
        locals.var_ires_leak_dn2 = 0.0;
        locals.var_ires_leak_dn4 = 0.0;
        locals.var_ires_leak_dn5 = 0.0;
        locals.var_ires_leak_dn6 = 0.0;
        locals.var_ires_leak_dn7 = 0.0;
        locals.var_ires_leak_dn8 = 0.0;
        locals.var_ires_leak_dn9 = 0.0;
        locals.var_ires_leak_dn10 = 0.0;
        locals.var_ires_leak_dn11 = 0.0;
        locals.var_ires_leak_dn14 = 0.0;
        locals.var_ires_leak_rv = 0.0;

        locals.var_pb2n = 0.0;
        locals.var_pb2n_dn0 = 0.0;
        locals.var_pb2n_dn2 = 0.0;
        locals.var_pb2n_dn4 = 0.0;
        locals.var_pb2n_dn5 = 0.0;
        locals.var_pb2n_dn6 = 0.0;
        locals.var_pb2n_dn7 = 0.0;
        locals.var_pb2n_dn8 = 0.0;
        locals.var_pb2n_dn9 = 0.0;
        locals.var_pb2n_dn10 = 0.0;
        locals.var_pb2n_dn11 = 0.0;
        locals.var_pb2n_dn14 = 0.0;
        locals.var_pb2n_rv = 0.0;

        locals.var_vbipn = 0.0;
        locals.var_vbipn_dn0 = 0.0;
        locals.var_vbipn_dn2 = 0.0;
        locals.var_vbipn_dn4 = 0.0;
        locals.var_vbipn_dn5 = 0.0;
        locals.var_vbipn_dn6 = 0.0;
        locals.var_vbipn_dn7 = 0.0;
        locals.var_vbipn_dn8 = 0.0;
        locals.var_vbipn_dn9 = 0.0;
        locals.var_vbipn_dn10 = 0.0;
        locals.var_vbipn_dn11 = 0.0;
        locals.var_vbipn_dn14 = 0.0;
        locals.var_vbipn_rv = 0.0;

        locals.var_hbdceff = p.p447;
        locals.var_hbdceff_dn0 = 0.0;
        locals.var_hbdceff_dn2 = 0.0;
        locals.var_hbdceff_dn4 = 0.0;
        locals.var_hbdceff_dn5 = 0.0;
        locals.var_hbdceff_dn6 = 0.0;
        locals.var_hbdceff_dn7 = 0.0;
        locals.var_hbdceff_dn8 = 0.0;
        locals.var_hbdceff_dn9 = 0.0;
        locals.var_hbdceff_dn10 = 0.0;
        locals.var_hbdceff_dn11 = 0.0;
        locals.var_hbdceff_dn14 = 0.0;
        locals.var_hbdceff_rv = 0.0;

        locals.var_uc_subtmp = p.p193;
        locals.var_uc_subtmp_rv = 0.0;

        locals.var_depmphn0 = 0.0;
        locals.var_depmphn0_dn0 = 0.0;
        locals.var_depmphn0_dn2 = 0.0;
        locals.var_depmphn0_dn4 = 0.0;
        locals.var_depmphn0_dn5 = 0.0;
        locals.var_depmphn0_dn6 = 0.0;
        locals.var_depmphn0_dn7 = 0.0;
        locals.var_depmphn0_dn8 = 0.0;
        locals.var_depmphn0_dn9 = 0.0;
        locals.var_depmphn0_dn10 = 0.0;
        locals.var_depmphn0_dn11 = 0.0;
        locals.var_depmphn0_dn14 = 0.0;
        locals.var_depmphn0_rv = 0.0;

        locals.var_qiu_noi = 0.0;
        locals.var_qiu_noi_dn0 = 0.0;
        locals.var_qiu_noi_dn2 = 0.0;
        locals.var_qiu_noi_dn4 = 0.0;
        locals.var_qiu_noi_dn5 = 0.0;
        locals.var_qiu_noi_dn6 = 0.0;
        locals.var_qiu_noi_dn7 = 0.0;
        locals.var_qiu_noi_dn8 = 0.0;
        locals.var_qiu_noi_dn9 = 0.0;
        locals.var_qiu_noi_dn10 = 0.0;
        locals.var_qiu_noi_dn11 = 0.0;
        locals.var_qiu_noi_dn14 = 0.0;
        locals.var_qiu_noi_rv = 0.0;

        locals.var_lp_s0_max = 40.0;
        locals.var_lp_s0_max_rv = 0.0;

        locals.var_js = 0.0;
        locals.var_js_dn0 = 0.0;
        locals.var_js_dn2 = 0.0;
        locals.var_js_dn4 = 0.0;
        locals.var_js_dn5 = 0.0;
        locals.var_js_dn6 = 0.0;
        locals.var_js_dn7 = 0.0;
        locals.var_js_dn8 = 0.0;
        locals.var_js_dn9 = 0.0;
        locals.var_js_dn10 = 0.0;
        locals.var_js_dn11 = 0.0;
        locals.var_js_dn14 = 0.0;
        locals.var_js_rv = 0.0;

        locals.var_jssw = 0.0;
        locals.var_jssw_dn0 = 0.0;
        locals.var_jssw_dn2 = 0.0;
        locals.var_jssw_dn4 = 0.0;
        locals.var_jssw_dn5 = 0.0;
        locals.var_jssw_dn6 = 0.0;
        locals.var_jssw_dn7 = 0.0;
        locals.var_jssw_dn8 = 0.0;
        locals.var_jssw_dn9 = 0.0;
        locals.var_jssw_dn10 = 0.0;
        locals.var_jssw_dn11 = 0.0;
        locals.var_jssw_dn14 = 0.0;
        locals.var_jssw_rv = 0.0;

        locals.var_js2 = 0.0;
        locals.var_js2_dn0 = 0.0;
        locals.var_js2_dn2 = 0.0;
        locals.var_js2_dn4 = 0.0;
        locals.var_js2_dn5 = 0.0;
        locals.var_js2_dn6 = 0.0;
        locals.var_js2_dn7 = 0.0;
        locals.var_js2_dn8 = 0.0;
        locals.var_js2_dn9 = 0.0;
        locals.var_js2_dn10 = 0.0;
        locals.var_js2_dn11 = 0.0;
        locals.var_js2_dn14 = 0.0;
        locals.var_js2_rv = 0.0;

        locals.var_jssw2 = 0.0;
        locals.var_jssw2_dn0 = 0.0;
        locals.var_jssw2_dn2 = 0.0;
        locals.var_jssw2_dn4 = 0.0;
        locals.var_jssw2_dn5 = 0.0;
        locals.var_jssw2_dn6 = 0.0;
        locals.var_jssw2_dn7 = 0.0;
        locals.var_jssw2_dn8 = 0.0;
        locals.var_jssw2_dn9 = 0.0;
        locals.var_jssw2_dn10 = 0.0;
        locals.var_jssw2_dn11 = 0.0;
        locals.var_jssw2_dn14 = 0.0;
        locals.var_jssw2_rv = 0.0;

        locals.var_qbs = 0.0;
        locals.var_qbs_dn0 = 0.0;
        locals.var_qbs_dn2 = 0.0;
        locals.var_qbs_dn4 = 0.0;
        locals.var_qbs_dn5 = 0.0;
        locals.var_qbs_dn6 = 0.0;
        locals.var_qbs_dn7 = 0.0;
        locals.var_qbs_dn8 = 0.0;
        locals.var_qbs_dn9 = 0.0;
        locals.var_qbs_dn10 = 0.0;
        locals.var_qbs_dn11 = 0.0;
        locals.var_qbs_dn14 = 0.0;
        locals.var_qbs_rv = 0.0;

        locals.var_qbd = 0.0;
        locals.var_qbd_dn0 = 0.0;
        locals.var_qbd_dn2 = 0.0;
        locals.var_qbd_dn4 = 0.0;
        locals.var_qbd_dn5 = 0.0;
        locals.var_qbd_dn6 = 0.0;
        locals.var_qbd_dn7 = 0.0;
        locals.var_qbd_dn8 = 0.0;
        locals.var_qbd_dn9 = 0.0;
        locals.var_qbd_dn10 = 0.0;
        locals.var_qbd_dn11 = 0.0;
        locals.var_qbd_dn14 = 0.0;
        locals.var_qbd_dn16 = 0.0;
        locals.var_qbd_dn17 = 0.0;
        locals.var_qbd_dn18 = 0.0;
        locals.var_qbd_rv = 0.0;

        locals.var_qbsi = 0.0;
        locals.var_qbsi_dn0 = 0.0;
        locals.var_qbsi_dn2 = 0.0;
        locals.var_qbsi_dn4 = 0.0;
        locals.var_qbsi_dn5 = 0.0;
        locals.var_qbsi_dn6 = 0.0;
        locals.var_qbsi_dn7 = 0.0;
        locals.var_qbsi_dn8 = 0.0;
        locals.var_qbsi_dn9 = 0.0;
        locals.var_qbsi_dn10 = 0.0;
        locals.var_qbsi_dn11 = 0.0;
        locals.var_qbsi_dn14 = 0.0;
        locals.var_qbsi_rv = 0.0;

        locals.var_qbdi = 0.0;
        locals.var_qbdi_dn0 = 0.0;
        locals.var_qbdi_dn2 = 0.0;
        locals.var_qbdi_dn4 = 0.0;
        locals.var_qbdi_dn5 = 0.0;
        locals.var_qbdi_dn6 = 0.0;
        locals.var_qbdi_dn7 = 0.0;
        locals.var_qbdi_dn8 = 0.0;
        locals.var_qbdi_dn9 = 0.0;
        locals.var_qbdi_dn10 = 0.0;
        locals.var_qbdi_dn11 = 0.0;
        locals.var_qbdi_dn14 = 0.0;
        locals.var_qbdi_rv = 0.0;

        locals.var_czbd = 0.0;
        locals.var_czbd_dn0 = 0.0;
        locals.var_czbd_dn2 = 0.0;
        locals.var_czbd_dn4 = 0.0;
        locals.var_czbd_dn5 = 0.0;
        locals.var_czbd_dn6 = 0.0;
        locals.var_czbd_dn7 = 0.0;
        locals.var_czbd_dn8 = 0.0;
        locals.var_czbd_dn9 = 0.0;
        locals.var_czbd_dn10 = 0.0;
        locals.var_czbd_dn11 = 0.0;
        locals.var_czbd_dn14 = 0.0;
        locals.var_czbd_rv = 0.0;

        locals.var_czbdsw = 0.0;
        locals.var_czbdsw_dn0 = 0.0;
        locals.var_czbdsw_dn2 = 0.0;
        locals.var_czbdsw_dn4 = 0.0;
        locals.var_czbdsw_dn5 = 0.0;
        locals.var_czbdsw_dn6 = 0.0;
        locals.var_czbdsw_dn7 = 0.0;
        locals.var_czbdsw_dn8 = 0.0;
        locals.var_czbdsw_dn9 = 0.0;
        locals.var_czbdsw_dn10 = 0.0;
        locals.var_czbdsw_dn11 = 0.0;
        locals.var_czbdsw_dn14 = 0.0;
        locals.var_czbdsw_rv = 0.0;

        locals.var_czbdswg = 0.0;
        locals.var_czbdswg_dn0 = 0.0;
        locals.var_czbdswg_dn2 = 0.0;
        locals.var_czbdswg_dn4 = 0.0;
        locals.var_czbdswg_dn5 = 0.0;
        locals.var_czbdswg_dn6 = 0.0;
        locals.var_czbdswg_dn7 = 0.0;
        locals.var_czbdswg_dn8 = 0.0;
        locals.var_czbdswg_dn9 = 0.0;
        locals.var_czbdswg_dn10 = 0.0;
        locals.var_czbdswg_dn11 = 0.0;
        locals.var_czbdswg_dn14 = 0.0;
        locals.var_czbdswg_rv = 0.0;

        locals.var_czbs = 0.0;
        locals.var_czbs_dn0 = 0.0;
        locals.var_czbs_dn2 = 0.0;
        locals.var_czbs_dn4 = 0.0;
        locals.var_czbs_dn5 = 0.0;
        locals.var_czbs_dn6 = 0.0;
        locals.var_czbs_dn7 = 0.0;
        locals.var_czbs_dn8 = 0.0;
        locals.var_czbs_dn9 = 0.0;
        locals.var_czbs_dn10 = 0.0;
        locals.var_czbs_dn11 = 0.0;
        locals.var_czbs_dn14 = 0.0;
        locals.var_czbs_rv = 0.0;

        locals.var_czbssw = 0.0;
        locals.var_czbssw_dn0 = 0.0;
        locals.var_czbssw_dn2 = 0.0;
        locals.var_czbssw_dn4 = 0.0;
        locals.var_czbssw_dn5 = 0.0;
        locals.var_czbssw_dn6 = 0.0;
        locals.var_czbssw_dn7 = 0.0;
        locals.var_czbssw_dn8 = 0.0;
        locals.var_czbssw_dn9 = 0.0;
        locals.var_czbssw_dn10 = 0.0;
        locals.var_czbssw_dn11 = 0.0;
        locals.var_czbssw_dn14 = 0.0;
        locals.var_czbssw_rv = 0.0;

        locals.var_czbsswg = 0.0;
        locals.var_czbsswg_dn0 = 0.0;
        locals.var_czbsswg_dn2 = 0.0;
        locals.var_czbsswg_dn4 = 0.0;
        locals.var_czbsswg_dn5 = 0.0;
        locals.var_czbsswg_dn6 = 0.0;
        locals.var_czbsswg_dn7 = 0.0;
        locals.var_czbsswg_dn8 = 0.0;
        locals.var_czbsswg_dn9 = 0.0;
        locals.var_czbsswg_dn10 = 0.0;
        locals.var_czbsswg_dn11 = 0.0;
        locals.var_czbsswg_dn14 = 0.0;
        locals.var_czbsswg_rv = 0.0;

        locals.var_pzbd = 0.0;
        locals.var_pzbd_dn0 = 0.0;
        locals.var_pzbd_dn2 = 0.0;
        locals.var_pzbd_dn4 = 0.0;
        locals.var_pzbd_dn5 = 0.0;
        locals.var_pzbd_dn6 = 0.0;
        locals.var_pzbd_dn7 = 0.0;
        locals.var_pzbd_dn8 = 0.0;
        locals.var_pzbd_dn9 = 0.0;
        locals.var_pzbd_dn10 = 0.0;
        locals.var_pzbd_dn11 = 0.0;
        locals.var_pzbd_dn14 = 0.0;
        locals.var_pzbd_rv = 0.0;

        locals.var_pzbdsw = 0.0;
        locals.var_pzbdsw_dn0 = 0.0;
        locals.var_pzbdsw_dn2 = 0.0;
        locals.var_pzbdsw_dn4 = 0.0;
        locals.var_pzbdsw_dn5 = 0.0;
        locals.var_pzbdsw_dn6 = 0.0;
        locals.var_pzbdsw_dn7 = 0.0;
        locals.var_pzbdsw_dn8 = 0.0;
        locals.var_pzbdsw_dn9 = 0.0;
        locals.var_pzbdsw_dn10 = 0.0;
        locals.var_pzbdsw_dn11 = 0.0;
        locals.var_pzbdsw_dn14 = 0.0;
        locals.var_pzbdsw_rv = 0.0;

        locals.var_pzbdswg = 0.0;
        locals.var_pzbdswg_dn0 = 0.0;
        locals.var_pzbdswg_dn2 = 0.0;
        locals.var_pzbdswg_dn4 = 0.0;
        locals.var_pzbdswg_dn5 = 0.0;
        locals.var_pzbdswg_dn6 = 0.0;
        locals.var_pzbdswg_dn7 = 0.0;
        locals.var_pzbdswg_dn8 = 0.0;
        locals.var_pzbdswg_dn9 = 0.0;
        locals.var_pzbdswg_dn10 = 0.0;
        locals.var_pzbdswg_dn11 = 0.0;
        locals.var_pzbdswg_dn14 = 0.0;
        locals.var_pzbdswg_rv = 0.0;

        locals.var_pzbs = 0.0;
        locals.var_pzbs_dn0 = 0.0;
        locals.var_pzbs_dn2 = 0.0;
        locals.var_pzbs_dn4 = 0.0;
        locals.var_pzbs_dn5 = 0.0;
        locals.var_pzbs_dn6 = 0.0;
        locals.var_pzbs_dn7 = 0.0;
        locals.var_pzbs_dn8 = 0.0;
        locals.var_pzbs_dn9 = 0.0;
        locals.var_pzbs_dn10 = 0.0;
        locals.var_pzbs_dn11 = 0.0;
        locals.var_pzbs_dn14 = 0.0;
        locals.var_pzbs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        locals: &mut StampLocals,
    ) {
        locals.var_pzbssw = 0.0;
        locals.var_pzbssw_dn0 = 0.0;
        locals.var_pzbssw_dn2 = 0.0;
        locals.var_pzbssw_dn4 = 0.0;
        locals.var_pzbssw_dn5 = 0.0;
        locals.var_pzbssw_dn6 = 0.0;
        locals.var_pzbssw_dn7 = 0.0;
        locals.var_pzbssw_dn8 = 0.0;
        locals.var_pzbssw_dn9 = 0.0;
        locals.var_pzbssw_dn10 = 0.0;
        locals.var_pzbssw_dn11 = 0.0;
        locals.var_pzbssw_dn14 = 0.0;
        locals.var_pzbssw_rv = 0.0;

        locals.var_pzbsswg = 0.0;
        locals.var_pzbsswg_dn0 = 0.0;
        locals.var_pzbsswg_dn2 = 0.0;
        locals.var_pzbsswg_dn4 = 0.0;
        locals.var_pzbsswg_dn5 = 0.0;
        locals.var_pzbsswg_dn6 = 0.0;
        locals.var_pzbsswg_dn7 = 0.0;
        locals.var_pzbsswg_dn8 = 0.0;
        locals.var_pzbsswg_dn9 = 0.0;
        locals.var_pzbsswg_dn10 = 0.0;
        locals.var_pzbsswg_dn11 = 0.0;
        locals.var_pzbsswg_dn14 = 0.0;
        locals.var_pzbsswg_rv = 0.0;

        locals.var_sarg = 0.0;
        locals.var_sarg_dn0 = 0.0;
        locals.var_sarg_dn2 = 0.0;
        locals.var_sarg_dn4 = 0.0;
        locals.var_sarg_dn5 = 0.0;
        locals.var_sarg_dn6 = 0.0;
        locals.var_sarg_dn7 = 0.0;
        locals.var_sarg_dn8 = 0.0;
        locals.var_sarg_dn9 = 0.0;
        locals.var_sarg_dn10 = 0.0;
        locals.var_sarg_dn11 = 0.0;
        locals.var_sarg_dn14 = 0.0;
        locals.var_sarg_rv = 0.0;

        locals.var_vsbs = 0.0;
        locals.var_vsbs_dn2 = 0.0;
        locals.var_vsbs_dn11 = 0.0;
        locals.var_vsbs_rv = 0.0;

        locals.var_vdbd = 0.0;
        locals.var_vdbd_dn0 = 0.0;
        locals.var_vdbd_dn10 = 0.0;
        locals.var_vdbd_rv = 0.0;

        locals.var_vbs_jct = 0.0;
        locals.var_vbs_jct_dn2 = 0.0;
        locals.var_vbs_jct_dn11 = 0.0;
        locals.var_vbs_jct_rv = 0.0;

        locals.var_vbd_jct = 0.0;
        locals.var_vbd_jct_dn0 = 0.0;
        locals.var_vbd_jct_dn10 = 0.0;
        locals.var_vbd_jct_rv = 0.0;

        locals.var_vbpsp = 0.0;
        locals.var_vbpsp_dn8 = 0.0;
        locals.var_vbpsp_dn9 = 0.0;
        locals.var_vbpsp_rv = 0.0;

        locals.var_vbpdp = 0.0;
        locals.var_vbpdp_dn6 = 0.0;
        locals.var_vbpdp_dn9 = 0.0;
        locals.var_vbpdp_rv = 0.0;

        locals.var_vbsi_jct = 0.0;
        locals.var_vbsi_jct_dn8 = 0.0;
        locals.var_vbsi_jct_dn9 = 0.0;
        locals.var_vbsi_jct_rv = 0.0;

        locals.var_vbdi_jct = 0.0;
        locals.var_vbdi_jct_dn6 = 0.0;
        locals.var_vbdi_jct_dn9 = 0.0;
        locals.var_vbdi_jct_rv = 0.0;

        locals.var_exptempd = 0.0;
        locals.var_exptempd_dn0 = 0.0;
        locals.var_exptempd_dn2 = 0.0;
        locals.var_exptempd_dn4 = 0.0;
        locals.var_exptempd_dn5 = 0.0;
        locals.var_exptempd_dn6 = 0.0;
        locals.var_exptempd_dn7 = 0.0;
        locals.var_exptempd_dn8 = 0.0;
        locals.var_exptempd_dn9 = 0.0;
        locals.var_exptempd_dn10 = 0.0;
        locals.var_exptempd_dn11 = 0.0;
        locals.var_exptempd_dn14 = 0.0;
        locals.var_exptempd_rv = 0.0;

        locals.var_exptemps = 0.0;
        locals.var_exptemps_dn0 = 0.0;
        locals.var_exptemps_dn2 = 0.0;
        locals.var_exptemps_dn4 = 0.0;
        locals.var_exptemps_dn5 = 0.0;
        locals.var_exptemps_dn6 = 0.0;
        locals.var_exptemps_dn7 = 0.0;
        locals.var_exptemps_dn8 = 0.0;
        locals.var_exptemps_dn9 = 0.0;
        locals.var_exptemps_dn10 = 0.0;
        locals.var_exptemps_dn11 = 0.0;
        locals.var_exptemps_dn14 = 0.0;
        locals.var_exptemps_rv = 0.0;

        locals.var_isbd = 0.0;
        locals.var_isbd_dn0 = 0.0;
        locals.var_isbd_dn2 = 0.0;
        locals.var_isbd_dn4 = 0.0;
        locals.var_isbd_dn5 = 0.0;
        locals.var_isbd_dn6 = 0.0;
        locals.var_isbd_dn7 = 0.0;
        locals.var_isbd_dn8 = 0.0;
        locals.var_isbd_dn9 = 0.0;
        locals.var_isbd_dn10 = 0.0;
        locals.var_isbd_dn11 = 0.0;
        locals.var_isbd_dn14 = 0.0;
        locals.var_isbd_rv = 0.0;

        locals.var_isbs = 0.0;
        locals.var_isbs_dn0 = 0.0;
        locals.var_isbs_dn2 = 0.0;
        locals.var_isbs_dn4 = 0.0;
        locals.var_isbs_dn5 = 0.0;
        locals.var_isbs_dn6 = 0.0;
        locals.var_isbs_dn7 = 0.0;
        locals.var_isbs_dn8 = 0.0;
        locals.var_isbs_dn9 = 0.0;
        locals.var_isbs_dn10 = 0.0;
        locals.var_isbs_dn11 = 0.0;
        locals.var_isbs_dn14 = 0.0;
        locals.var_isbs_rv = 0.0;

        locals.var_jd_expcd = 0.0;
        locals.var_jd_expcd_dn0 = 0.0;
        locals.var_jd_expcd_dn2 = 0.0;
        locals.var_jd_expcd_dn4 = 0.0;
        locals.var_jd_expcd_dn5 = 0.0;
        locals.var_jd_expcd_dn6 = 0.0;
        locals.var_jd_expcd_dn7 = 0.0;
        locals.var_jd_expcd_dn8 = 0.0;
        locals.var_jd_expcd_dn9 = 0.0;
        locals.var_jd_expcd_dn10 = 0.0;
        locals.var_jd_expcd_dn11 = 0.0;
        locals.var_jd_expcd_dn14 = 0.0;
        locals.var_jd_expcd_rv = 0.0;

        locals.var_jd_expcs = 0.0;
        locals.var_jd_expcs_dn0 = 0.0;
        locals.var_jd_expcs_dn2 = 0.0;
        locals.var_jd_expcs_dn4 = 0.0;
        locals.var_jd_expcs_dn5 = 0.0;
        locals.var_jd_expcs_dn6 = 0.0;
        locals.var_jd_expcs_dn7 = 0.0;
        locals.var_jd_expcs_dn8 = 0.0;
        locals.var_jd_expcs_dn9 = 0.0;
        locals.var_jd_expcs_dn10 = 0.0;
        locals.var_jd_expcs_dn11 = 0.0;
        locals.var_jd_expcs_dn14 = 0.0;
        locals.var_jd_expcs_rv = 0.0;

        locals.var_vbdt = 0.0;
        locals.var_vbdt_dn0 = 0.0;
        locals.var_vbdt_dn2 = 0.0;
        locals.var_vbdt_dn4 = 0.0;
        locals.var_vbdt_dn5 = 0.0;
        locals.var_vbdt_dn6 = 0.0;
        locals.var_vbdt_dn7 = 0.0;
        locals.var_vbdt_dn8 = 0.0;
        locals.var_vbdt_dn9 = 0.0;
        locals.var_vbdt_dn10 = 0.0;
        locals.var_vbdt_dn11 = 0.0;
        locals.var_vbdt_dn14 = 0.0;
        locals.var_vbdt_rv = 0.0;

        locals.var_vbst = 0.0;
        locals.var_vbst_dn0 = 0.0;
        locals.var_vbst_dn2 = 0.0;
        locals.var_vbst_dn4 = 0.0;
        locals.var_vbst_dn5 = 0.0;
        locals.var_vbst_dn6 = 0.0;
        locals.var_vbst_dn7 = 0.0;
        locals.var_vbst_dn8 = 0.0;
        locals.var_vbst_dn9 = 0.0;
        locals.var_vbst_dn10 = 0.0;
        locals.var_vbst_dn11 = 0.0;
        locals.var_vbst_dn14 = 0.0;
        locals.var_vbst_rv = 0.0;

        locals.var_jd_nvtm_invd = 0.0;
        locals.var_jd_nvtm_invd_dn0 = 0.0;
        locals.var_jd_nvtm_invd_dn2 = 0.0;
        locals.var_jd_nvtm_invd_dn4 = 0.0;
        locals.var_jd_nvtm_invd_dn5 = 0.0;
        locals.var_jd_nvtm_invd_dn6 = 0.0;
        locals.var_jd_nvtm_invd_dn7 = 0.0;
        locals.var_jd_nvtm_invd_dn8 = 0.0;
        locals.var_jd_nvtm_invd_dn9 = 0.0;
        locals.var_jd_nvtm_invd_dn10 = 0.0;
        locals.var_jd_nvtm_invd_dn11 = 0.0;
        locals.var_jd_nvtm_invd_dn14 = 0.0;
        locals.var_jd_nvtm_invd_rv = 0.0;

        locals.var_jd_nvtm_invs = 0.0;
        locals.var_jd_nvtm_invs_dn0 = 0.0;
        locals.var_jd_nvtm_invs_dn2 = 0.0;
        locals.var_jd_nvtm_invs_dn4 = 0.0;
        locals.var_jd_nvtm_invs_dn5 = 0.0;
        locals.var_jd_nvtm_invs_dn6 = 0.0;
        locals.var_jd_nvtm_invs_dn7 = 0.0;
        locals.var_jd_nvtm_invs_dn8 = 0.0;
        locals.var_jd_nvtm_invs_dn9 = 0.0;
        locals.var_jd_nvtm_invs_dn10 = 0.0;
        locals.var_jd_nvtm_invs_dn11 = 0.0;
        locals.var_jd_nvtm_invs_dn14 = 0.0;
        locals.var_jd_nvtm_invs_rv = 0.0;

        locals.var_end_of_part_1 = 0.0;
        locals.var_end_of_part_1_rv = 0.0;

        locals.var_flg_brk1 = 0.0;
        locals.var_flg_brk1_rv = 0.0;

        locals.var_start_of_loopl = 0.0;
        locals.var_start_of_loopl_rv = 0.0;

        locals.var_flg_brk2 = 0.0;
        locals.var_flg_brk2_rv = 0.0;

        locals.var_start_of_mobility = 0.0;
        locals.var_start_of_mobility_rv = 0.0;

        locals.var_qbd_qs = 0.0;
        locals.var_qbd_qs_dn0 = 0.0;
        locals.var_qbd_qs_dn2 = 0.0;
        locals.var_qbd_qs_dn4 = 0.0;
        locals.var_qbd_qs_dn5 = 0.0;
        locals.var_qbd_qs_dn6 = 0.0;
        locals.var_qbd_qs_dn7 = 0.0;
        locals.var_qbd_qs_dn8 = 0.0;
        locals.var_qbd_qs_dn9 = 0.0;
        locals.var_qbd_qs_dn10 = 0.0;
        locals.var_qbd_qs_dn11 = 0.0;
        locals.var_qbd_qs_dn14 = 0.0;
        locals.var_qbd_qs_rv = 0.0;

        locals.var_isbd_btm = 0.0;
        locals.var_isbd_btm_dn0 = 0.0;
        locals.var_isbd_btm_dn2 = 0.0;
        locals.var_isbd_btm_dn4 = 0.0;
        locals.var_isbd_btm_dn5 = 0.0;
        locals.var_isbd_btm_dn6 = 0.0;
        locals.var_isbd_btm_dn7 = 0.0;
        locals.var_isbd_btm_dn8 = 0.0;
        locals.var_isbd_btm_dn9 = 0.0;
        locals.var_isbd_btm_dn10 = 0.0;
        locals.var_isbd_btm_dn11 = 0.0;
        locals.var_isbd_btm_dn14 = 0.0;
        locals.var_isbd_btm_rv = 0.0;

        locals.var_isbd2_btm = 0.0;
        locals.var_isbd2_btm_dn0 = 0.0;
        locals.var_isbd2_btm_dn2 = 0.0;
        locals.var_isbd2_btm_dn4 = 0.0;
        locals.var_isbd2_btm_dn5 = 0.0;
        locals.var_isbd2_btm_dn6 = 0.0;
        locals.var_isbd2_btm_dn7 = 0.0;
        locals.var_isbd2_btm_dn8 = 0.0;
        locals.var_isbd2_btm_dn9 = 0.0;
        locals.var_isbd2_btm_dn10 = 0.0;
        locals.var_isbd2_btm_dn11 = 0.0;
        locals.var_isbd2_btm_dn14 = 0.0;
        locals.var_isbd2_btm_rv = 0.0;

        locals.var_isbd_sws = 0.0;
        locals.var_isbd_sws_dn0 = 0.0;
        locals.var_isbd_sws_dn2 = 0.0;
        locals.var_isbd_sws_dn4 = 0.0;
        locals.var_isbd_sws_dn5 = 0.0;
        locals.var_isbd_sws_dn6 = 0.0;
        locals.var_isbd_sws_dn7 = 0.0;
        locals.var_isbd_sws_dn8 = 0.0;
        locals.var_isbd_sws_dn9 = 0.0;
        locals.var_isbd_sws_dn10 = 0.0;
        locals.var_isbd_sws_dn11 = 0.0;
        locals.var_isbd_sws_dn14 = 0.0;
        locals.var_isbd_sws_rv = 0.0;

        locals.var_isbd2_sws = 0.0;
        locals.var_isbd2_sws_dn0 = 0.0;
        locals.var_isbd2_sws_dn2 = 0.0;
        locals.var_isbd2_sws_dn4 = 0.0;
        locals.var_isbd2_sws_dn5 = 0.0;
        locals.var_isbd2_sws_dn6 = 0.0;
        locals.var_isbd2_sws_dn7 = 0.0;
        locals.var_isbd2_sws_dn8 = 0.0;
        locals.var_isbd2_sws_dn9 = 0.0;
        locals.var_isbd2_sws_dn10 = 0.0;
        locals.var_isbd2_sws_dn11 = 0.0;
        locals.var_isbd2_sws_dn14 = 0.0;
        locals.var_isbd2_sws_rv = 0.0;

        locals.var_isbd_swg = 0.0;
        locals.var_isbd_swg_dn0 = 0.0;
        locals.var_isbd_swg_dn2 = 0.0;
        locals.var_isbd_swg_dn4 = 0.0;
        locals.var_isbd_swg_dn5 = 0.0;
        locals.var_isbd_swg_dn6 = 0.0;
        locals.var_isbd_swg_dn7 = 0.0;
        locals.var_isbd_swg_dn8 = 0.0;
        locals.var_isbd_swg_dn9 = 0.0;
        locals.var_isbd_swg_dn10 = 0.0;
        locals.var_isbd_swg_dn11 = 0.0;
        locals.var_isbd_swg_dn14 = 0.0;
        locals.var_isbd_swg_rv = 0.0;

        locals.var_isbd2_swg = 0.0;
        locals.var_isbd2_swg_dn0 = 0.0;
        locals.var_isbd2_swg_dn2 = 0.0;
        locals.var_isbd2_swg_dn4 = 0.0;
        locals.var_isbd2_swg_dn5 = 0.0;
        locals.var_isbd2_swg_dn6 = 0.0;
        locals.var_isbd2_swg_dn7 = 0.0;
        locals.var_isbd2_swg_dn8 = 0.0;
        locals.var_isbd2_swg_dn9 = 0.0;
        locals.var_isbd2_swg_dn10 = 0.0;
        locals.var_isbd2_swg_dn11 = 0.0;
        locals.var_isbd2_swg_dn14 = 0.0;
        locals.var_isbd2_swg_rv = 0.0;

        locals.var_isbs_btm = 0.0;
        locals.var_isbs_btm_dn0 = 0.0;
        locals.var_isbs_btm_dn2 = 0.0;
        locals.var_isbs_btm_dn4 = 0.0;
        locals.var_isbs_btm_dn5 = 0.0;
        locals.var_isbs_btm_dn6 = 0.0;
        locals.var_isbs_btm_dn7 = 0.0;
        locals.var_isbs_btm_dn8 = 0.0;
        locals.var_isbs_btm_dn9 = 0.0;
        locals.var_isbs_btm_dn10 = 0.0;
        locals.var_isbs_btm_dn11 = 0.0;
        locals.var_isbs_btm_dn14 = 0.0;
        locals.var_isbs_btm_rv = 0.0;

        locals.var_isbs2_btm = 0.0;
        locals.var_isbs2_btm_dn0 = 0.0;
        locals.var_isbs2_btm_dn2 = 0.0;
        locals.var_isbs2_btm_dn4 = 0.0;
        locals.var_isbs2_btm_dn5 = 0.0;
        locals.var_isbs2_btm_dn6 = 0.0;
        locals.var_isbs2_btm_dn7 = 0.0;
        locals.var_isbs2_btm_dn8 = 0.0;
        locals.var_isbs2_btm_dn9 = 0.0;
        locals.var_isbs2_btm_dn10 = 0.0;
        locals.var_isbs2_btm_dn11 = 0.0;
        locals.var_isbs2_btm_dn14 = 0.0;
        locals.var_isbs2_btm_rv = 0.0;

        locals.var_isbs_sws = 0.0;
        locals.var_isbs_sws_dn0 = 0.0;
        locals.var_isbs_sws_dn2 = 0.0;
        locals.var_isbs_sws_dn4 = 0.0;
        locals.var_isbs_sws_dn5 = 0.0;
        locals.var_isbs_sws_dn6 = 0.0;
        locals.var_isbs_sws_dn7 = 0.0;
        locals.var_isbs_sws_dn8 = 0.0;
        locals.var_isbs_sws_dn9 = 0.0;
        locals.var_isbs_sws_dn10 = 0.0;
        locals.var_isbs_sws_dn11 = 0.0;
        locals.var_isbs_sws_dn14 = 0.0;
        locals.var_isbs_sws_rv = 0.0;

        locals.var_isbs2_sws = 0.0;
        locals.var_isbs2_sws_dn0 = 0.0;
        locals.var_isbs2_sws_dn2 = 0.0;
        locals.var_isbs2_sws_dn4 = 0.0;
        locals.var_isbs2_sws_dn5 = 0.0;
        locals.var_isbs2_sws_dn6 = 0.0;
        locals.var_isbs2_sws_dn7 = 0.0;
        locals.var_isbs2_sws_dn8 = 0.0;
        locals.var_isbs2_sws_dn9 = 0.0;
        locals.var_isbs2_sws_dn10 = 0.0;
        locals.var_isbs2_sws_dn11 = 0.0;
        locals.var_isbs2_sws_dn14 = 0.0;
        locals.var_isbs2_sws_rv = 0.0;

        locals.var_isbs_swg = 0.0;
        locals.var_isbs_swg_dn0 = 0.0;
        locals.var_isbs_swg_dn2 = 0.0;
        locals.var_isbs_swg_dn4 = 0.0;
        locals.var_isbs_swg_dn5 = 0.0;
        locals.var_isbs_swg_dn6 = 0.0;
        locals.var_isbs_swg_dn7 = 0.0;
        locals.var_isbs_swg_dn8 = 0.0;
        locals.var_isbs_swg_dn9 = 0.0;
        locals.var_isbs_swg_dn10 = 0.0;
        locals.var_isbs_swg_dn11 = 0.0;
        locals.var_isbs_swg_dn14 = 0.0;
        locals.var_isbs_swg_rv = 0.0;

        locals.var_isbs2_swg = 0.0;
        locals.var_isbs2_swg_dn0 = 0.0;
        locals.var_isbs2_swg_dn2 = 0.0;
        locals.var_isbs2_swg_dn4 = 0.0;
        locals.var_isbs2_swg_dn5 = 0.0;
        locals.var_isbs2_swg_dn6 = 0.0;
        locals.var_isbs2_swg_dn7 = 0.0;
        locals.var_isbs2_swg_dn8 = 0.0;
        locals.var_isbs2_swg_dn9 = 0.0;
        locals.var_isbs2_swg_dn10 = 0.0;
        locals.var_isbs2_swg_dn11 = 0.0;
        locals.var_isbs2_swg_dn14 = 0.0;
        locals.var_isbs2_swg_rv = 0.0;

        locals.var_qovd_add = 0.0;
        locals.var_qovd_add_dn0 = 0.0;
        locals.var_qovd_add_dn2 = 0.0;
        locals.var_qovd_add_dn4 = 0.0;
        locals.var_qovd_add_dn5 = 0.0;
        locals.var_qovd_add_dn6 = 0.0;
        locals.var_qovd_add_dn7 = 0.0;
        locals.var_qovd_add_dn8 = 0.0;
        locals.var_qovd_add_dn9 = 0.0;
        locals.var_qovd_add_dn10 = 0.0;
        locals.var_qovd_add_dn11 = 0.0;
        locals.var_qovd_add_dn14 = 0.0;
        locals.var_qovd_add_rv = 0.0;

        locals.var_qovs_add = 0.0;
        locals.var_qovs_add_dn0 = 0.0;
        locals.var_qovs_add_dn2 = 0.0;
        locals.var_qovs_add_dn4 = 0.0;
        locals.var_qovs_add_dn5 = 0.0;
        locals.var_qovs_add_dn6 = 0.0;
        locals.var_qovs_add_dn7 = 0.0;
        locals.var_qovs_add_dn8 = 0.0;
        locals.var_qovs_add_dn9 = 0.0;
        locals.var_qovs_add_dn10 = 0.0;
        locals.var_qovs_add_dn11 = 0.0;
        locals.var_qovs_add_dn14 = 0.0;
        locals.var_qovs_add_rv = 0.0;

        locals.var_qbdld_add = 0.0;
        locals.var_qbdld_add_dn0 = 0.0;
        locals.var_qbdld_add_dn2 = 0.0;
        locals.var_qbdld_add_dn4 = 0.0;
        locals.var_qbdld_add_dn5 = 0.0;
        locals.var_qbdld_add_dn6 = 0.0;
        locals.var_qbdld_add_dn7 = 0.0;
        locals.var_qbdld_add_dn8 = 0.0;
        locals.var_qbdld_add_dn9 = 0.0;
        locals.var_qbdld_add_dn10 = 0.0;
        locals.var_qbdld_add_dn11 = 0.0;
        locals.var_qbdld_add_dn14 = 0.0;
        locals.var_qbdld_add_rv = 0.0;

        locals.var_qbsld_add = 0.0;
        locals.var_qbsld_add_dn0 = 0.0;
        locals.var_qbsld_add_dn2 = 0.0;
        locals.var_qbsld_add_dn4 = 0.0;
        locals.var_qbsld_add_dn5 = 0.0;
        locals.var_qbsld_add_dn6 = 0.0;
        locals.var_qbsld_add_dn7 = 0.0;
        locals.var_qbsld_add_dn8 = 0.0;
        locals.var_qbsld_add_dn9 = 0.0;
        locals.var_qbsld_add_dn10 = 0.0;
        locals.var_qbsld_add_dn11 = 0.0;
        locals.var_qbsld_add_dn14 = 0.0;
        locals.var_qbsld_add_rv = 0.0;

        locals.var_wjuncld = 0.0;
        locals.var_wjuncld_dn0 = 0.0;
        locals.var_wjuncld_dn2 = 0.0;
        locals.var_wjuncld_dn4 = 0.0;
        locals.var_wjuncld_dn5 = 0.0;
        locals.var_wjuncld_dn6 = 0.0;
        locals.var_wjuncld_dn7 = 0.0;
        locals.var_wjuncld_dn8 = 0.0;
        locals.var_wjuncld_dn9 = 0.0;
        locals.var_wjuncld_dn10 = 0.0;
        locals.var_wjuncld_dn11 = 0.0;
        locals.var_wjuncld_dn14 = 0.0;
        locals.var_wjuncld_rv = 0.0;

        locals.var_idspt0 = 0.0;
        locals.var_idspt0_dn0 = 0.0;
        locals.var_idspt0_dn2 = 0.0;
        locals.var_idspt0_dn4 = 0.0;
        locals.var_idspt0_dn5 = 0.0;
        locals.var_idspt0_dn6 = 0.0;
        locals.var_idspt0_dn7 = 0.0;
        locals.var_idspt0_dn8 = 0.0;
        locals.var_idspt0_dn9 = 0.0;
        locals.var_idspt0_dn10 = 0.0;
        locals.var_idspt0_dn11 = 0.0;
        locals.var_idspt0_dn14 = 0.0;
        locals.var_idspt0_rv = 0.0;

    }
}
