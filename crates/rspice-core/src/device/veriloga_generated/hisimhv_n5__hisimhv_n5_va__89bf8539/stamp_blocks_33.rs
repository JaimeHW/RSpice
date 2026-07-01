#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_139(
        locals: &mut StampLocals,
    ) {
        let (assign40600_e53857, assign40600_e53857_d_n0, assign40600_e53857_d_n2, assign40600_e53857_d_n4, assign40600_e53857_d_n5, assign40600_e53857_d_n6, assign40600_e53857_d_n7, assign40600_e53857_d_n8, assign40600_e53857_d_n9, assign40600_e53857_d_n10, assign40600_e53857_d_n11, assign40600_e53857_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40600_e53857;
        locals.var_dnm_dn0 = assign40600_e53857_d_n0;
        locals.var_dnm_dn2 = assign40600_e53857_d_n2;
        locals.var_dnm_dn4 = assign40600_e53857_d_n4;
        locals.var_dnm_dn5 = assign40600_e53857_d_n5;
        locals.var_dnm_dn6 = assign40600_e53857_d_n6;
        locals.var_dnm_dn7 = assign40600_e53857_d_n7;
        locals.var_dnm_dn8 = assign40600_e53857_d_n8;
        locals.var_dnm_dn9 = assign40600_e53857_d_n9;
        locals.var_dnm_dn10 = assign40600_e53857_d_n10;
        locals.var_dnm_dn11 = assign40600_e53857_d_n11;
        locals.var_dnm_dn14 = assign40600_e53857_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign40610_e53872: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1016 = assign40610_e53872;
        locals.var_guard1016_rv = 0.0;

        let assign40620_e53875: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1017 = assign40620_e53875;
        locals.var_guard1017_rv = 0.0;

        let (assign40630_e53890,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) && (locals.var_guard1016 != 0.0)) && (locals.var_guard1017 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40630_e53890;
        locals.var_mm_rv = 0.0;

        let assign40640_e53893: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1018 = assign40640_e53893;
        locals.var_guard1018_rv = 0.0;

        let (assign40650_e53911,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) && (locals.var_guard1016 != 0.0)) && (locals.var_guard1017 == 0.0)) && (locals.var_guard1018 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40650_e53911;
        locals.var_mm_rv = 0.0;

        let assign40660_e53914: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1019 = assign40660_e53914;
        locals.var_guard1019_rv = 0.0;

        let (assign40670_e53935,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) && (locals.var_guard1016 != 0.0)) && (locals.var_guard1017 == 0.0)) && (locals.var_guard1018 == 0.0)) && (locals.var_guard1019 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40670_e53935;
        locals.var_mm_rv = 0.0;

        let assign40680_e53938: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1020 = assign40680_e53938;
        locals.var_guard1020_rv = 0.0;

        let (assign40690_e53962,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) && (locals.var_guard1016 != 0.0)) && (locals.var_guard1017 == 0.0)) && (locals.var_guard1018 == 0.0)) && (locals.var_guard1019 == 0.0)) && (locals.var_guard1020 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40690_e53962;
        locals.var_mm_rv = 0.0;

        let (assign40700_e53975,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) && (locals.var_guard1016 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign40700_e53975;
        locals.var_m0_rv = 0.0;

        let mut assign40710_loop_guard: usize = 0;
        while {
            let assign40710_cond_e53989: f64 = if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) && (locals.var_guard1016 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign40710_cond_e53989 != 0.0
        } {
            assign40710_loop_guard += 1;
            assert!(assign40710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign40710_body0_e54003, assign40710_body0_e54003_d_n0, assign40710_body0_e54003_d_n2, assign40710_body0_e54003_d_n4, assign40710_body0_e54003_d_n5, assign40710_body0_e54003_d_n6, assign40710_body0_e54003_d_n7, assign40710_body0_e54003_d_n8, assign40710_body0_e54003_d_n9, assign40710_body0_e54003_d_n10, assign40710_body0_e54003_d_n11, assign40710_body0_e54003_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) && (locals.var_guard1016 != 0.0)) {
        let assign40710_body0_e54001: f64 = (locals.var_dnm).sqrt();
        (assign40710_body0_e54001, (locals.var_dnm_dn0 / (2.0 * assign40710_body0_e54001)), (locals.var_dnm_dn2 / (2.0 * assign40710_body0_e54001)), (locals.var_dnm_dn4 / (2.0 * assign40710_body0_e54001)), (locals.var_dnm_dn5 / (2.0 * assign40710_body0_e54001)), (locals.var_dnm_dn6 / (2.0 * assign40710_body0_e54001)), (locals.var_dnm_dn7 / (2.0 * assign40710_body0_e54001)), (locals.var_dnm_dn8 / (2.0 * assign40710_body0_e54001)), (locals.var_dnm_dn9 / (2.0 * assign40710_body0_e54001)), (locals.var_dnm_dn10 / (2.0 * assign40710_body0_e54001)), (locals.var_dnm_dn11 / (2.0 * assign40710_body0_e54001)), (locals.var_dnm_dn14 / (2.0 * assign40710_body0_e54001)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign40710_body0_e54003;
            locals.var_dnm_dn0 = assign40710_body0_e54003_d_n0;
            locals.var_dnm_dn2 = assign40710_body0_e54003_d_n2;
            locals.var_dnm_dn4 = assign40710_body0_e54003_d_n4;
            locals.var_dnm_dn5 = assign40710_body0_e54003_d_n5;
            locals.var_dnm_dn6 = assign40710_body0_e54003_d_n6;
            locals.var_dnm_dn7 = assign40710_body0_e54003_d_n7;
            locals.var_dnm_dn8 = assign40710_body0_e54003_d_n8;
            locals.var_dnm_dn9 = assign40710_body0_e54003_d_n9;
            locals.var_dnm_dn10 = assign40710_body0_e54003_d_n10;
            locals.var_dnm_dn11 = assign40710_body0_e54003_d_n11;
            locals.var_dnm_dn14 = assign40710_body0_e54003_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign40710_body1_e54018,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) && (locals.var_guard1016 != 0.0)) {
        let assign40710_body1_e54016: f64 = (locals.var_m0 + 1.0);
        (assign40710_body1_e54016,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign40710_body1_e54018;
            locals.var_m0_rv = 0.0;
        }

        let (assign40720_e54043, assign40720_e54043_d_n0, assign40720_e54043_d_n2, assign40720_e54043_d_n4, assign40720_e54043_d_n5, assign40720_e54043_d_n6, assign40720_e54043_d_n7, assign40720_e54043_d_n8, assign40720_e54043_d_n9, assign40720_e54043_d_n10, assign40720_e54043_d_n11, assign40720_e54043_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) && (locals.var_guard1016 == 0.0)) {
        let (assign40720_e54041, assign40720_e54041_d_n0, assign40720_e54041_d_n2, assign40720_e54041_d_n4, assign40720_e54041_d_n5, assign40720_e54041_d_n6, assign40720_e54041_d_n7, assign40720_e54041_d_n8, assign40720_e54041_d_n9, assign40720_e54041_d_n10, assign40720_e54041_d_n11, assign40720_e54041_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign40720_e54038: f64 = (2.0 * 2.0);
                let assign40720_e54039: f64 = (1.0 / assign40720_e54038);
                let assign40720_e54040: f64 = (locals.var_dnm).powf(assign40720_e54039);
                (assign40720_e54040, if 0.0 == 0.0 && ((assign40720_e54039) as f64).is_finite() && ((assign40720_e54039) as f64).fract() == 0.0 { if assign40720_e54039 == 0.0 { 0.0 } else { (assign40720_e54039 * ((locals.var_dnm).powf(assign40720_e54039 - 1.0) * locals.var_dnm_dn0)) } } else { (assign40720_e54040 * (assign40720_e54039 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40720_e54039) as f64).is_finite() && ((assign40720_e54039) as f64).fract() == 0.0 { if assign40720_e54039 == 0.0 { 0.0 } else { (assign40720_e54039 * ((locals.var_dnm).powf(assign40720_e54039 - 1.0) * locals.var_dnm_dn2)) } } else { (assign40720_e54040 * (assign40720_e54039 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40720_e54039) as f64).is_finite() && ((assign40720_e54039) as f64).fract() == 0.0 { if assign40720_e54039 == 0.0 { 0.0 } else { (assign40720_e54039 * ((locals.var_dnm).powf(assign40720_e54039 - 1.0) * locals.var_dnm_dn4)) } } else { (assign40720_e54040 * (assign40720_e54039 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40720_e54039) as f64).is_finite() && ((assign40720_e54039) as f64).fract() == 0.0 { if assign40720_e54039 == 0.0 { 0.0 } else { (assign40720_e54039 * ((locals.var_dnm).powf(assign40720_e54039 - 1.0) * locals.var_dnm_dn5)) } } else { (assign40720_e54040 * (assign40720_e54039 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40720_e54039) as f64).is_finite() && ((assign40720_e54039) as f64).fract() == 0.0 { if assign40720_e54039 == 0.0 { 0.0 } else { (assign40720_e54039 * ((locals.var_dnm).powf(assign40720_e54039 - 1.0) * locals.var_dnm_dn6)) } } else { (assign40720_e54040 * (assign40720_e54039 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40720_e54039) as f64).is_finite() && ((assign40720_e54039) as f64).fract() == 0.0 { if assign40720_e54039 == 0.0 { 0.0 } else { (assign40720_e54039 * ((locals.var_dnm).powf(assign40720_e54039 - 1.0) * locals.var_dnm_dn7)) } } else { (assign40720_e54040 * (assign40720_e54039 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40720_e54039) as f64).is_finite() && ((assign40720_e54039) as f64).fract() == 0.0 { if assign40720_e54039 == 0.0 { 0.0 } else { (assign40720_e54039 * ((locals.var_dnm).powf(assign40720_e54039 - 1.0) * locals.var_dnm_dn8)) } } else { (assign40720_e54040 * (assign40720_e54039 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40720_e54039) as f64).is_finite() && ((assign40720_e54039) as f64).fract() == 0.0 { if assign40720_e54039 == 0.0 { 0.0 } else { (assign40720_e54039 * ((locals.var_dnm).powf(assign40720_e54039 - 1.0) * locals.var_dnm_dn9)) } } else { (assign40720_e54040 * (assign40720_e54039 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40720_e54039) as f64).is_finite() && ((assign40720_e54039) as f64).fract() == 0.0 { if assign40720_e54039 == 0.0 { 0.0 } else { (assign40720_e54039 * ((locals.var_dnm).powf(assign40720_e54039 - 1.0) * locals.var_dnm_dn10)) } } else { (assign40720_e54040 * (assign40720_e54039 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40720_e54039) as f64).is_finite() && ((assign40720_e54039) as f64).fract() == 0.0 { if assign40720_e54039 == 0.0 { 0.0 } else { (assign40720_e54039 * ((locals.var_dnm).powf(assign40720_e54039 - 1.0) * locals.var_dnm_dn11)) } } else { (assign40720_e54040 * (assign40720_e54039 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40720_e54039) as f64).is_finite() && ((assign40720_e54039) as f64).fract() == 0.0 { if assign40720_e54039 == 0.0 { 0.0 } else { (assign40720_e54039 * ((locals.var_dnm).powf(assign40720_e54039 - 1.0) * locals.var_dnm_dn14)) } } else { (assign40720_e54040 * (assign40720_e54039 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign40720_e54041, assign40720_e54041_d_n0, assign40720_e54041_d_n2, assign40720_e54041_d_n4, assign40720_e54041_d_n5, assign40720_e54041_d_n6, assign40720_e54041_d_n7, assign40720_e54041_d_n8, assign40720_e54041_d_n9, assign40720_e54041_d_n10, assign40720_e54041_d_n11, assign40720_e54041_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40720_e54043;
        locals.var_dnm_dn0 = assign40720_e54043_d_n0;
        locals.var_dnm_dn2 = assign40720_e54043_d_n2;
        locals.var_dnm_dn4 = assign40720_e54043_d_n4;
        locals.var_dnm_dn5 = assign40720_e54043_d_n5;
        locals.var_dnm_dn6 = assign40720_e54043_d_n6;
        locals.var_dnm_dn7 = assign40720_e54043_d_n7;
        locals.var_dnm_dn8 = assign40720_e54043_d_n8;
        locals.var_dnm_dn9 = assign40720_e54043_d_n9;
        locals.var_dnm_dn10 = assign40720_e54043_d_n10;
        locals.var_dnm_dn11 = assign40720_e54043_d_n11;
        locals.var_dnm_dn14 = assign40720_e54043_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign40730_e54056, assign40730_e54056_d_n0, assign40730_e54056_d_n2, assign40730_e54056_d_n4, assign40730_e54056_d_n5, assign40730_e54056_d_n6, assign40730_e54056_d_n7, assign40730_e54056_d_n8, assign40730_e54056_d_n9, assign40730_e54056_d_n10, assign40730_e54056_d_n11, assign40730_e54056_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) {
        let assign40730_e54054: f64 = (1.0 / locals.var_dnm);
        (assign40730_e54054, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40730_e54056;
        locals.var_dnm_dn0 = assign40730_e54056_d_n0;
        locals.var_dnm_dn2 = assign40730_e54056_d_n2;
        locals.var_dnm_dn4 = assign40730_e54056_d_n4;
        locals.var_dnm_dn5 = assign40730_e54056_d_n5;
        locals.var_dnm_dn6 = assign40730_e54056_d_n6;
        locals.var_dnm_dn7 = assign40730_e54056_d_n7;
        locals.var_dnm_dn8 = assign40730_e54056_d_n8;
        locals.var_dnm_dn9 = assign40730_e54056_d_n9;
        locals.var_dnm_dn10 = assign40730_e54056_d_n10;
        locals.var_dnm_dn11 = assign40730_e54056_d_n11;
        locals.var_dnm_dn14 = assign40730_e54056_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign40740_e54073, assign40740_e54073_d_n0, assign40740_e54073_d_n2, assign40740_e54073_d_n4, assign40740_e54073_d_n5, assign40740_e54073_d_n6, assign40740_e54073_d_n7, assign40740_e54073_d_n8, assign40740_e54073_d_n9, assign40740_e54073_d_n10, assign40740_e54073_d_n11, assign40740_e54073_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) {
        let assign40740_e54068: f64 = (10.0 * 2.220446049250313e-16);
        let assign40740_e54069: f64 = (locals.var_tmf1 * assign40740_e54068);
        let assign40740_e54071: f64 = (assign40740_e54069 * locals.var_dnm);
        (assign40740_e54071, (((locals.var_tmf1_dn0 * assign40740_e54068) * locals.var_dnm) + (assign40740_e54069 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign40740_e54068) * locals.var_dnm) + (assign40740_e54069 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign40740_e54068) * locals.var_dnm) + (assign40740_e54069 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign40740_e54068) * locals.var_dnm) + (assign40740_e54069 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign40740_e54068) * locals.var_dnm) + (assign40740_e54069 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign40740_e54068) * locals.var_dnm) + (assign40740_e54069 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign40740_e54068) * locals.var_dnm) + (assign40740_e54069 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign40740_e54068) * locals.var_dnm) + (assign40740_e54069 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign40740_e54068) * locals.var_dnm) + (assign40740_e54069 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign40740_e54068) * locals.var_dnm) + (assign40740_e54069 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign40740_e54068) * locals.var_dnm) + (assign40740_e54069 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign40740_e54073;
        locals.var_tmf0_dn0 = assign40740_e54073_d_n0;
        locals.var_tmf0_dn2 = assign40740_e54073_d_n2;
        locals.var_tmf0_dn4 = assign40740_e54073_d_n4;
        locals.var_tmf0_dn5 = assign40740_e54073_d_n5;
        locals.var_tmf0_dn6 = assign40740_e54073_d_n6;
        locals.var_tmf0_dn7 = assign40740_e54073_d_n7;
        locals.var_tmf0_dn8 = assign40740_e54073_d_n8;
        locals.var_tmf0_dn9 = assign40740_e54073_d_n9;
        locals.var_tmf0_dn10 = assign40740_e54073_d_n10;
        locals.var_tmf0_dn11 = assign40740_e54073_d_n11;
        locals.var_tmf0_dn14 = assign40740_e54073_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign40750_e54092, assign40750_e54092_d_n0, assign40750_e54092_d_n2, assign40750_e54092_d_n4, assign40750_e54092_d_n5, assign40750_e54092_d_n6, assign40750_e54092_d_n7, assign40750_e54092_d_n8, assign40750_e54092_d_n9, assign40750_e54092_d_n10, assign40750_e54092_d_n11, assign40750_e54092_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) {
        let assign40750_e54084: f64 = (10.0 * 2.220446049250313e-16);
        let assign40750_e54086: f64 = (assign40750_e54084 * locals.var_xmp);
        let assign40750_e54088: f64 = (assign40750_e54086 * locals.var_dnm);
        let assign40750_e54090: f64 = (assign40750_e54088 / locals.var_arg);
        (assign40750_e54090, ((((((assign40750_e54084 * locals.var_xmp_dn0) * locals.var_dnm) + (assign40750_e54086 * locals.var_dnm_dn0)) * locals.var_arg) - (assign40750_e54088 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign40750_e54084 * locals.var_xmp_dn2) * locals.var_dnm) + (assign40750_e54086 * locals.var_dnm_dn2)) * locals.var_arg) - (assign40750_e54088 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign40750_e54084 * locals.var_xmp_dn4) * locals.var_dnm) + (assign40750_e54086 * locals.var_dnm_dn4)) * locals.var_arg) - (assign40750_e54088 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign40750_e54084 * locals.var_xmp_dn5) * locals.var_dnm) + (assign40750_e54086 * locals.var_dnm_dn5)) * locals.var_arg) - (assign40750_e54088 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign40750_e54084 * locals.var_xmp_dn6) * locals.var_dnm) + (assign40750_e54086 * locals.var_dnm_dn6)) * locals.var_arg) - (assign40750_e54088 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign40750_e54084 * locals.var_xmp_dn7) * locals.var_dnm) + (assign40750_e54086 * locals.var_dnm_dn7)) * locals.var_arg) - (assign40750_e54088 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign40750_e54084 * locals.var_xmp_dn8) * locals.var_dnm) + (assign40750_e54086 * locals.var_dnm_dn8)) * locals.var_arg) - (assign40750_e54088 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign40750_e54084 * locals.var_xmp_dn9) * locals.var_dnm) + (assign40750_e54086 * locals.var_dnm_dn9)) * locals.var_arg) - (assign40750_e54088 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign40750_e54084 * locals.var_xmp_dn10) * locals.var_dnm) + (assign40750_e54086 * locals.var_dnm_dn10)) * locals.var_arg) - (assign40750_e54088 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign40750_e54084 * locals.var_xmp_dn11) * locals.var_dnm) + (assign40750_e54086 * locals.var_dnm_dn11)) * locals.var_arg) - (assign40750_e54088 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign40750_e54084 * locals.var_xmp_dn14) * locals.var_dnm) + (assign40750_e54086 * locals.var_dnm_dn14)) * locals.var_arg) - (assign40750_e54088 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign40750_e54092;
        locals.var_t0_dn0 = assign40750_e54092_d_n0;
        locals.var_t0_dn2 = assign40750_e54092_d_n2;
        locals.var_t0_dn4 = assign40750_e54092_d_n4;
        locals.var_t0_dn5 = assign40750_e54092_d_n5;
        locals.var_t0_dn6 = assign40750_e54092_d_n6;
        locals.var_t0_dn7 = assign40750_e54092_d_n7;
        locals.var_t0_dn8 = assign40750_e54092_d_n8;
        locals.var_t0_dn9 = assign40750_e54092_d_n9;
        locals.var_t0_dn10 = assign40750_e54092_d_n10;
        locals.var_t0_dn11 = assign40750_e54092_d_n11;
        locals.var_t0_dn14 = assign40750_e54092_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign40760_e54111, assign40760_e54111_d_n0, assign40760_e54111_d_n2, assign40760_e54111_d_n4, assign40760_e54111_d_n5, assign40760_e54111_d_n6, assign40760_e54111_d_n7, assign40760_e54111_d_n8, assign40760_e54111_d_n9, assign40760_e54111_d_n10, assign40760_e54111_d_n11, assign40760_e54111_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) {
        let assign40760_e54103: f64 = (10.0 * 2.220446049250313e-16);
        let assign40760_e54106: f64 = (10.0 * 2.220446049250313e-16);
        let assign40760_e54107: f64 = (assign40760_e54103 + assign40760_e54106);
        let assign40760_e54109: f64 = (assign40760_e54107 - locals.var_tmf0);
        (assign40760_e54109, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn14,)
    }
};
        locals.var_pzadd = assign40760_e54111;
        locals.var_pzadd_dn0 = assign40760_e54111_d_n0;
        locals.var_pzadd_dn2 = assign40760_e54111_d_n2;
        locals.var_pzadd_dn4 = assign40760_e54111_d_n4;
        locals.var_pzadd_dn5 = assign40760_e54111_d_n5;
        locals.var_pzadd_dn6 = assign40760_e54111_d_n6;
        locals.var_pzadd_dn7 = assign40760_e54111_d_n7;
        locals.var_pzadd_dn8 = assign40760_e54111_d_n8;
        locals.var_pzadd_dn9 = assign40760_e54111_d_n9;
        locals.var_pzadd_dn10 = assign40760_e54111_d_n10;
        locals.var_pzadd_dn11 = assign40760_e54111_d_n11;
        locals.var_pzadd_dn14 = assign40760_e54111_d_n14;
        locals.var_pzadd_rv = 0.0;

        let (assign40770_e54122, assign40770_e54122_d_n0, assign40770_e54122_d_n2, assign40770_e54122_d_n4, assign40770_e54122_d_n5, assign40770_e54122_d_n6, assign40770_e54122_d_n7, assign40770_e54122_d_n8, assign40770_e54122_d_n9, assign40770_e54122_d_n10, assign40770_e54122_d_n11, assign40770_e54122_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign40770_e54122;
        locals.var_t0_dn0 = assign40770_e54122_d_n0;
        locals.var_t0_dn2 = assign40770_e54122_d_n2;
        locals.var_t0_dn4 = assign40770_e54122_d_n4;
        locals.var_t0_dn5 = assign40770_e54122_d_n5;
        locals.var_t0_dn6 = assign40770_e54122_d_n6;
        locals.var_t0_dn7 = assign40770_e54122_d_n7;
        locals.var_t0_dn8 = assign40770_e54122_d_n8;
        locals.var_t0_dn9 = assign40770_e54122_d_n9;
        locals.var_t0_dn10 = assign40770_e54122_d_n10;
        locals.var_t0_dn11 = assign40770_e54122_d_n11;
        locals.var_t0_dn14 = assign40770_e54122_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign40780_e54134, assign40780_e54134_d_n0, assign40780_e54134_d_n2, assign40780_e54134_d_n4, assign40780_e54134_d_n5, assign40780_e54134_d_n6, assign40780_e54134_d_n7, assign40780_e54134_d_n8, assign40780_e54134_d_n9, assign40780_e54134_d_n10, assign40780_e54134_d_n11, assign40780_e54134_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 == 0.0)) {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn14,)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn14,)
    }
};
        locals.var_pzadd = assign40780_e54134;
        locals.var_pzadd_dn0 = assign40780_e54134_d_n0;
        locals.var_pzadd_dn2 = assign40780_e54134_d_n2;
        locals.var_pzadd_dn4 = assign40780_e54134_d_n4;
        locals.var_pzadd_dn5 = assign40780_e54134_d_n5;
        locals.var_pzadd_dn6 = assign40780_e54134_d_n6;
        locals.var_pzadd_dn7 = assign40780_e54134_d_n7;
        locals.var_pzadd_dn8 = assign40780_e54134_d_n8;
        locals.var_pzadd_dn9 = assign40780_e54134_d_n9;
        locals.var_pzadd_dn10 = assign40780_e54134_d_n10;
        locals.var_pzadd_dn11 = assign40780_e54134_d_n11;
        locals.var_pzadd_dn14 = assign40780_e54134_d_n14;
        locals.var_pzadd_rv = 0.0;

        let (assign40790_e54146, assign40790_e54146_d_n0, assign40790_e54146_d_n2, assign40790_e54146_d_n4, assign40790_e54146_d_n5, assign40790_e54146_d_n6, assign40790_e54146_d_n7, assign40790_e54146_d_n8, assign40790_e54146_d_n9, assign40790_e54146_d_n10, assign40790_e54146_d_n11, assign40790_e54146_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1015 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign40790_e54146;
        locals.var_t0_dn0 = assign40790_e54146_d_n0;
        locals.var_t0_dn2 = assign40790_e54146_d_n2;
        locals.var_t0_dn4 = assign40790_e54146_d_n4;
        locals.var_t0_dn5 = assign40790_e54146_d_n5;
        locals.var_t0_dn6 = assign40790_e54146_d_n6;
        locals.var_t0_dn7 = assign40790_e54146_d_n7;
        locals.var_t0_dn8 = assign40790_e54146_d_n8;
        locals.var_t0_dn9 = assign40790_e54146_d_n9;
        locals.var_t0_dn10 = assign40790_e54146_d_n10;
        locals.var_t0_dn11 = assign40790_e54146_d_n11;
        locals.var_t0_dn14 = assign40790_e54146_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign40800_e54157, assign40800_e54157_d_n0, assign40800_e54157_d_n2, assign40800_e54157_d_n4, assign40800_e54157_d_n5, assign40800_e54157_d_n6, assign40800_e54157_d_n7, assign40800_e54157_d_n8, assign40800_e54157_d_n9, assign40800_e54157_d_n10, assign40800_e54157_d_n11, assign40800_e54157_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign40800_e54155: f64 = (locals.var_ps0 + locals.var_pzadd);
        (assign40800_e54155, (locals.var_ps0_dn0 + locals.var_pzadd_dn0), (locals.var_ps0_dn2 + locals.var_pzadd_dn2), (locals.var_ps0_dn4 + locals.var_pzadd_dn4), (locals.var_ps0_dn5 + locals.var_pzadd_dn5), (locals.var_ps0_dn6 + locals.var_pzadd_dn6), (locals.var_ps0_dn7 + locals.var_pzadd_dn7), (locals.var_ps0_dn8 + locals.var_pzadd_dn8), (locals.var_ps0_dn9 + locals.var_pzadd_dn9), (locals.var_ps0_dn10 + locals.var_pzadd_dn10), (locals.var_ps0_dn11 + locals.var_pzadd_dn11), (locals.var_ps0_dn14 + locals.var_pzadd_dn14),)
    } else {
        (locals.var_ps0z, locals.var_ps0z_dn0, locals.var_ps0z_dn2, locals.var_ps0z_dn4, locals.var_ps0z_dn5, locals.var_ps0z_dn6, locals.var_ps0z_dn7, locals.var_ps0z_dn8, locals.var_ps0z_dn9, locals.var_ps0z_dn10, locals.var_ps0z_dn11, locals.var_ps0z_dn14,)
    }
};
        locals.var_ps0z = assign40800_e54157;
        locals.var_ps0z_dn0 = assign40800_e54157_d_n0;
        locals.var_ps0z_dn2 = assign40800_e54157_d_n2;
        locals.var_ps0z_dn4 = assign40800_e54157_d_n4;
        locals.var_ps0z_dn5 = assign40800_e54157_d_n5;
        locals.var_ps0z_dn6 = assign40800_e54157_d_n6;
        locals.var_ps0z_dn7 = assign40800_e54157_d_n7;
        locals.var_ps0z_dn8 = assign40800_e54157_d_n8;
        locals.var_ps0z_dn9 = assign40800_e54157_d_n9;
        locals.var_ps0z_dn10 = assign40800_e54157_d_n10;
        locals.var_ps0z_dn11 = assign40800_e54157_d_n11;
        locals.var_ps0z_dn14 = assign40800_e54157_d_n14;
        locals.var_ps0z_rv = 0.0;

        let assign40810_e54160: f64 = (locals.var_ps0z - locals.var_vds_maxb0__blk853);
        let assign40810_e54163: f64 = locals.var_ps_delta;
        let assign40810_e54168: f64 = if ((assign40810_e54160 < assign40810_e54163) && (locals.var_ps_delta >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1021 = assign40810_e54168;
        locals.var_guard1021_rv = 0.0;

        let (assign40820_e54185, assign40820_e54185_d_n0, assign40820_e54185_d_n2, assign40820_e54185_d_n4, assign40820_e54185_d_n5, assign40820_e54185_d_n6, assign40820_e54185_d_n7, assign40820_e54185_d_n8, assign40820_e54185_d_n9, assign40820_e54185_d_n10, assign40820_e54185_d_n11, assign40820_e54185_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40820_e54179: f64 = locals.var_ps_delta;
        let assign40820_e54182: f64 = (locals.var_ps0z - locals.var_vds_maxb0__blk853);
        let assign40820_e54183: f64 = (assign40820_e54179 - assign40820_e54182);
        (assign40820_e54183, (-locals.var_ps0z_dn0), (-locals.var_ps0z_dn2), (-locals.var_ps0z_dn4), (-locals.var_ps0z_dn5), (-locals.var_ps0z_dn6), (-locals.var_ps0z_dn7), (-locals.var_ps0z_dn8), (-locals.var_ps0z_dn9), (-locals.var_ps0z_dn10), (-locals.var_ps0z_dn11), (-locals.var_ps0z_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign40820_e54185;
        locals.var_tmf1_dn0 = assign40820_e54185_d_n0;
        locals.var_tmf1_dn2 = assign40820_e54185_d_n2;
        locals.var_tmf1_dn4 = assign40820_e54185_d_n4;
        locals.var_tmf1_dn5 = assign40820_e54185_d_n5;
        locals.var_tmf1_dn6 = assign40820_e54185_d_n6;
        locals.var_tmf1_dn7 = assign40820_e54185_d_n7;
        locals.var_tmf1_dn8 = assign40820_e54185_d_n8;
        locals.var_tmf1_dn9 = assign40820_e54185_d_n9;
        locals.var_tmf1_dn10 = assign40820_e54185_d_n10;
        locals.var_tmf1_dn11 = assign40820_e54185_d_n11;
        locals.var_tmf1_dn14 = assign40820_e54185_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign40830_e54198, assign40830_e54198_d_n0, assign40830_e54198_d_n2, assign40830_e54198_d_n4, assign40830_e54198_d_n5, assign40830_e54198_d_n6, assign40830_e54198_d_n7, assign40830_e54198_d_n8, assign40830_e54198_d_n9, assign40830_e54198_d_n10, assign40830_e54198_d_n11, assign40830_e54198_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40830_e54196: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign40830_e54196, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign40830_e54198;
        locals.var_x2_dn0 = assign40830_e54198_d_n0;
        locals.var_x2_dn2 = assign40830_e54198_d_n2;
        locals.var_x2_dn4 = assign40830_e54198_d_n4;
        locals.var_x2_dn5 = assign40830_e54198_d_n5;
        locals.var_x2_dn6 = assign40830_e54198_d_n6;
        locals.var_x2_dn7 = assign40830_e54198_d_n7;
        locals.var_x2_dn8 = assign40830_e54198_d_n8;
        locals.var_x2_dn9 = assign40830_e54198_d_n9;
        locals.var_x2_dn10 = assign40830_e54198_d_n10;
        locals.var_x2_dn11 = assign40830_e54198_d_n11;
        locals.var_x2_dn14 = assign40830_e54198_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign40840_e54211, assign40840_e54211_d_n0, assign40840_e54211_d_n2, assign40840_e54211_d_n4, assign40840_e54211_d_n5, assign40840_e54211_d_n6, assign40840_e54211_d_n7, assign40840_e54211_d_n8, assign40840_e54211_d_n9, assign40840_e54211_d_n10, assign40840_e54211_d_n11, assign40840_e54211_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40840_e54209: f64 = (locals.var_ps_delta * locals.var_ps_delta);
        (assign40840_e54209, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign40840_e54211;
        locals.var_xmax2_dn0 = assign40840_e54211_d_n0;
        locals.var_xmax2_dn2 = assign40840_e54211_d_n2;
        locals.var_xmax2_dn4 = assign40840_e54211_d_n4;
        locals.var_xmax2_dn5 = assign40840_e54211_d_n5;
        locals.var_xmax2_dn6 = assign40840_e54211_d_n6;
        locals.var_xmax2_dn7 = assign40840_e54211_d_n7;
        locals.var_xmax2_dn8 = assign40840_e54211_d_n8;
        locals.var_xmax2_dn9 = assign40840_e54211_d_n9;
        locals.var_xmax2_dn10 = assign40840_e54211_d_n10;
        locals.var_xmax2_dn11 = assign40840_e54211_d_n11;
        locals.var_xmax2_dn14 = assign40840_e54211_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign40850_e54222, assign40850_e54222_d_n0, assign40850_e54222_d_n2, assign40850_e54222_d_n4, assign40850_e54222_d_n5, assign40850_e54222_d_n6, assign40850_e54222_d_n7, assign40850_e54222_d_n8, assign40850_e54222_d_n9, assign40850_e54222_d_n10, assign40850_e54222_d_n11, assign40850_e54222_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40850_e54222;
        locals.var_xp_dn0 = assign40850_e54222_d_n0;
        locals.var_xp_dn2 = assign40850_e54222_d_n2;
        locals.var_xp_dn4 = assign40850_e54222_d_n4;
        locals.var_xp_dn5 = assign40850_e54222_d_n5;
        locals.var_xp_dn6 = assign40850_e54222_d_n6;
        locals.var_xp_dn7 = assign40850_e54222_d_n7;
        locals.var_xp_dn8 = assign40850_e54222_d_n8;
        locals.var_xp_dn9 = assign40850_e54222_d_n9;
        locals.var_xp_dn10 = assign40850_e54222_d_n10;
        locals.var_xp_dn11 = assign40850_e54222_d_n11;
        locals.var_xp_dn14 = assign40850_e54222_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40860_e54233, assign40860_e54233_d_n0, assign40860_e54233_d_n2, assign40860_e54233_d_n4, assign40860_e54233_d_n5, assign40860_e54233_d_n6, assign40860_e54233_d_n7, assign40860_e54233_d_n8, assign40860_e54233_d_n9, assign40860_e54233_d_n10, assign40860_e54233_d_n11, assign40860_e54233_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40860_e54233;
        locals.var_xmp_dn0 = assign40860_e54233_d_n0;
        locals.var_xmp_dn2 = assign40860_e54233_d_n2;
        locals.var_xmp_dn4 = assign40860_e54233_d_n4;
        locals.var_xmp_dn5 = assign40860_e54233_d_n5;
        locals.var_xmp_dn6 = assign40860_e54233_d_n6;
        locals.var_xmp_dn7 = assign40860_e54233_d_n7;
        locals.var_xmp_dn8 = assign40860_e54233_d_n8;
        locals.var_xmp_dn9 = assign40860_e54233_d_n9;
        locals.var_xmp_dn10 = assign40860_e54233_d_n10;
        locals.var_xmp_dn11 = assign40860_e54233_d_n11;
        locals.var_xmp_dn14 = assign40860_e54233_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40870_e54244,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign40870_e54244;
        locals.var_m0_rv = 0.0;

        let (assign40880_e54255,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40880_e54255;
        locals.var_mm_rv = 0.0;

        let (assign40890_e54266, assign40890_e54266_d_n0, assign40890_e54266_d_n2, assign40890_e54266_d_n4, assign40890_e54266_d_n5, assign40890_e54266_d_n6, assign40890_e54266_d_n7, assign40890_e54266_d_n8, assign40890_e54266_d_n9, assign40890_e54266_d_n10, assign40890_e54266_d_n11, assign40890_e54266_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign40890_e54266;
        locals.var_arg_dn0 = assign40890_e54266_d_n0;
        locals.var_arg_dn2 = assign40890_e54266_d_n2;
        locals.var_arg_dn4 = assign40890_e54266_d_n4;
        locals.var_arg_dn5 = assign40890_e54266_d_n5;
        locals.var_arg_dn6 = assign40890_e54266_d_n6;
        locals.var_arg_dn7 = assign40890_e54266_d_n7;
        locals.var_arg_dn8 = assign40890_e54266_d_n8;
        locals.var_arg_dn9 = assign40890_e54266_d_n9;
        locals.var_arg_dn10 = assign40890_e54266_d_n10;
        locals.var_arg_dn11 = assign40890_e54266_d_n11;
        locals.var_arg_dn14 = assign40890_e54266_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_140(
        locals: &mut StampLocals,
    ) {
        let (assign40900_e54277, assign40900_e54277_d_n0, assign40900_e54277_d_n2, assign40900_e54277_d_n4, assign40900_e54277_d_n5, assign40900_e54277_d_n6, assign40900_e54277_d_n7, assign40900_e54277_d_n8, assign40900_e54277_d_n9, assign40900_e54277_d_n10, assign40900_e54277_d_n11, assign40900_e54277_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40900_e54277;
        locals.var_dnm_dn0 = assign40900_e54277_d_n0;
        locals.var_dnm_dn2 = assign40900_e54277_d_n2;
        locals.var_dnm_dn4 = assign40900_e54277_d_n4;
        locals.var_dnm_dn5 = assign40900_e54277_d_n5;
        locals.var_dnm_dn6 = assign40900_e54277_d_n6;
        locals.var_dnm_dn7 = assign40900_e54277_d_n7;
        locals.var_dnm_dn8 = assign40900_e54277_d_n8;
        locals.var_dnm_dn9 = assign40900_e54277_d_n9;
        locals.var_dnm_dn10 = assign40900_e54277_d_n10;
        locals.var_dnm_dn11 = assign40900_e54277_d_n11;
        locals.var_dnm_dn14 = assign40900_e54277_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign40910_e54290, assign40910_e54290_d_n0, assign40910_e54290_d_n2, assign40910_e54290_d_n4, assign40910_e54290_d_n5, assign40910_e54290_d_n6, assign40910_e54290_d_n7, assign40910_e54290_d_n8, assign40910_e54290_d_n9, assign40910_e54290_d_n10, assign40910_e54290_d_n11, assign40910_e54290_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40910_e54288: f64 = (locals.var_xp * locals.var_x2);
        (assign40910_e54288, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40910_e54290;
        locals.var_xp_dn0 = assign40910_e54290_d_n0;
        locals.var_xp_dn2 = assign40910_e54290_d_n2;
        locals.var_xp_dn4 = assign40910_e54290_d_n4;
        locals.var_xp_dn5 = assign40910_e54290_d_n5;
        locals.var_xp_dn6 = assign40910_e54290_d_n6;
        locals.var_xp_dn7 = assign40910_e54290_d_n7;
        locals.var_xp_dn8 = assign40910_e54290_d_n8;
        locals.var_xp_dn9 = assign40910_e54290_d_n9;
        locals.var_xp_dn10 = assign40910_e54290_d_n10;
        locals.var_xp_dn11 = assign40910_e54290_d_n11;
        locals.var_xp_dn14 = assign40910_e54290_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40920_e54303, assign40920_e54303_d_n0, assign40920_e54303_d_n2, assign40920_e54303_d_n4, assign40920_e54303_d_n5, assign40920_e54303_d_n6, assign40920_e54303_d_n7, assign40920_e54303_d_n8, assign40920_e54303_d_n9, assign40920_e54303_d_n10, assign40920_e54303_d_n11, assign40920_e54303_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40920_e54301: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40920_e54301, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40920_e54303;
        locals.var_xmp_dn0 = assign40920_e54303_d_n0;
        locals.var_xmp_dn2 = assign40920_e54303_d_n2;
        locals.var_xmp_dn4 = assign40920_e54303_d_n4;
        locals.var_xmp_dn5 = assign40920_e54303_d_n5;
        locals.var_xmp_dn6 = assign40920_e54303_d_n6;
        locals.var_xmp_dn7 = assign40920_e54303_d_n7;
        locals.var_xmp_dn8 = assign40920_e54303_d_n8;
        locals.var_xmp_dn9 = assign40920_e54303_d_n9;
        locals.var_xmp_dn10 = assign40920_e54303_d_n10;
        locals.var_xmp_dn11 = assign40920_e54303_d_n11;
        locals.var_xmp_dn14 = assign40920_e54303_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40930_e54316, assign40930_e54316_d_n0, assign40930_e54316_d_n2, assign40930_e54316_d_n4, assign40930_e54316_d_n5, assign40930_e54316_d_n6, assign40930_e54316_d_n7, assign40930_e54316_d_n8, assign40930_e54316_d_n9, assign40930_e54316_d_n10, assign40930_e54316_d_n11, assign40930_e54316_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40930_e54314: f64 = (locals.var_xp * locals.var_x2);
        (assign40930_e54314, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40930_e54316;
        locals.var_xp_dn0 = assign40930_e54316_d_n0;
        locals.var_xp_dn2 = assign40930_e54316_d_n2;
        locals.var_xp_dn4 = assign40930_e54316_d_n4;
        locals.var_xp_dn5 = assign40930_e54316_d_n5;
        locals.var_xp_dn6 = assign40930_e54316_d_n6;
        locals.var_xp_dn7 = assign40930_e54316_d_n7;
        locals.var_xp_dn8 = assign40930_e54316_d_n8;
        locals.var_xp_dn9 = assign40930_e54316_d_n9;
        locals.var_xp_dn10 = assign40930_e54316_d_n10;
        locals.var_xp_dn11 = assign40930_e54316_d_n11;
        locals.var_xp_dn14 = assign40930_e54316_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40940_e54329, assign40940_e54329_d_n0, assign40940_e54329_d_n2, assign40940_e54329_d_n4, assign40940_e54329_d_n5, assign40940_e54329_d_n6, assign40940_e54329_d_n7, assign40940_e54329_d_n8, assign40940_e54329_d_n9, assign40940_e54329_d_n10, assign40940_e54329_d_n11, assign40940_e54329_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40940_e54327: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40940_e54327, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40940_e54329;
        locals.var_xmp_dn0 = assign40940_e54329_d_n0;
        locals.var_xmp_dn2 = assign40940_e54329_d_n2;
        locals.var_xmp_dn4 = assign40940_e54329_d_n4;
        locals.var_xmp_dn5 = assign40940_e54329_d_n5;
        locals.var_xmp_dn6 = assign40940_e54329_d_n6;
        locals.var_xmp_dn7 = assign40940_e54329_d_n7;
        locals.var_xmp_dn8 = assign40940_e54329_d_n8;
        locals.var_xmp_dn9 = assign40940_e54329_d_n9;
        locals.var_xmp_dn10 = assign40940_e54329_d_n10;
        locals.var_xmp_dn11 = assign40940_e54329_d_n11;
        locals.var_xmp_dn14 = assign40940_e54329_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40950_e54342, assign40950_e54342_d_n0, assign40950_e54342_d_n2, assign40950_e54342_d_n4, assign40950_e54342_d_n5, assign40950_e54342_d_n6, assign40950_e54342_d_n7, assign40950_e54342_d_n8, assign40950_e54342_d_n9, assign40950_e54342_d_n10, assign40950_e54342_d_n11, assign40950_e54342_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40950_e54340: f64 = (locals.var_xp * locals.var_x2);
        (assign40950_e54340, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40950_e54342;
        locals.var_xp_dn0 = assign40950_e54342_d_n0;
        locals.var_xp_dn2 = assign40950_e54342_d_n2;
        locals.var_xp_dn4 = assign40950_e54342_d_n4;
        locals.var_xp_dn5 = assign40950_e54342_d_n5;
        locals.var_xp_dn6 = assign40950_e54342_d_n6;
        locals.var_xp_dn7 = assign40950_e54342_d_n7;
        locals.var_xp_dn8 = assign40950_e54342_d_n8;
        locals.var_xp_dn9 = assign40950_e54342_d_n9;
        locals.var_xp_dn10 = assign40950_e54342_d_n10;
        locals.var_xp_dn11 = assign40950_e54342_d_n11;
        locals.var_xp_dn14 = assign40950_e54342_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40960_e54355, assign40960_e54355_d_n0, assign40960_e54355_d_n2, assign40960_e54355_d_n4, assign40960_e54355_d_n5, assign40960_e54355_d_n6, assign40960_e54355_d_n7, assign40960_e54355_d_n8, assign40960_e54355_d_n9, assign40960_e54355_d_n10, assign40960_e54355_d_n11, assign40960_e54355_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40960_e54353: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40960_e54353, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40960_e54355;
        locals.var_xmp_dn0 = assign40960_e54355_d_n0;
        locals.var_xmp_dn2 = assign40960_e54355_d_n2;
        locals.var_xmp_dn4 = assign40960_e54355_d_n4;
        locals.var_xmp_dn5 = assign40960_e54355_d_n5;
        locals.var_xmp_dn6 = assign40960_e54355_d_n6;
        locals.var_xmp_dn7 = assign40960_e54355_d_n7;
        locals.var_xmp_dn8 = assign40960_e54355_d_n8;
        locals.var_xmp_dn9 = assign40960_e54355_d_n9;
        locals.var_xmp_dn10 = assign40960_e54355_d_n10;
        locals.var_xmp_dn11 = assign40960_e54355_d_n11;
        locals.var_xmp_dn14 = assign40960_e54355_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40970_e54368, assign40970_e54368_d_n0, assign40970_e54368_d_n2, assign40970_e54368_d_n4, assign40970_e54368_d_n5, assign40970_e54368_d_n6, assign40970_e54368_d_n7, assign40970_e54368_d_n8, assign40970_e54368_d_n9, assign40970_e54368_d_n10, assign40970_e54368_d_n11, assign40970_e54368_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40970_e54366: f64 = (locals.var_xp * locals.var_x2);
        (assign40970_e54366, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40970_e54368;
        locals.var_xp_dn0 = assign40970_e54368_d_n0;
        locals.var_xp_dn2 = assign40970_e54368_d_n2;
        locals.var_xp_dn4 = assign40970_e54368_d_n4;
        locals.var_xp_dn5 = assign40970_e54368_d_n5;
        locals.var_xp_dn6 = assign40970_e54368_d_n6;
        locals.var_xp_dn7 = assign40970_e54368_d_n7;
        locals.var_xp_dn8 = assign40970_e54368_d_n8;
        locals.var_xp_dn9 = assign40970_e54368_d_n9;
        locals.var_xp_dn10 = assign40970_e54368_d_n10;
        locals.var_xp_dn11 = assign40970_e54368_d_n11;
        locals.var_xp_dn14 = assign40970_e54368_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40980_e54381, assign40980_e54381_d_n0, assign40980_e54381_d_n2, assign40980_e54381_d_n4, assign40980_e54381_d_n5, assign40980_e54381_d_n6, assign40980_e54381_d_n7, assign40980_e54381_d_n8, assign40980_e54381_d_n9, assign40980_e54381_d_n10, assign40980_e54381_d_n11, assign40980_e54381_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40980_e54379: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40980_e54379, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40980_e54381;
        locals.var_xmp_dn0 = assign40980_e54381_d_n0;
        locals.var_xmp_dn2 = assign40980_e54381_d_n2;
        locals.var_xmp_dn4 = assign40980_e54381_d_n4;
        locals.var_xmp_dn5 = assign40980_e54381_d_n5;
        locals.var_xmp_dn6 = assign40980_e54381_d_n6;
        locals.var_xmp_dn7 = assign40980_e54381_d_n7;
        locals.var_xmp_dn8 = assign40980_e54381_d_n8;
        locals.var_xmp_dn9 = assign40980_e54381_d_n9;
        locals.var_xmp_dn10 = assign40980_e54381_d_n10;
        locals.var_xmp_dn11 = assign40980_e54381_d_n11;
        locals.var_xmp_dn14 = assign40980_e54381_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40990_e54394, assign40990_e54394_d_n0, assign40990_e54394_d_n2, assign40990_e54394_d_n4, assign40990_e54394_d_n5, assign40990_e54394_d_n6, assign40990_e54394_d_n7, assign40990_e54394_d_n8, assign40990_e54394_d_n9, assign40990_e54394_d_n10, assign40990_e54394_d_n11, assign40990_e54394_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign40990_e54392: f64 = (locals.var_xp + locals.var_xmp);
        (assign40990_e54392, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign40990_e54394;
        locals.var_arg_dn0 = assign40990_e54394_d_n0;
        locals.var_arg_dn2 = assign40990_e54394_d_n2;
        locals.var_arg_dn4 = assign40990_e54394_d_n4;
        locals.var_arg_dn5 = assign40990_e54394_d_n5;
        locals.var_arg_dn6 = assign40990_e54394_d_n6;
        locals.var_arg_dn7 = assign40990_e54394_d_n7;
        locals.var_arg_dn8 = assign40990_e54394_d_n8;
        locals.var_arg_dn9 = assign40990_e54394_d_n9;
        locals.var_arg_dn10 = assign40990_e54394_d_n10;
        locals.var_arg_dn11 = assign40990_e54394_d_n11;
        locals.var_arg_dn14 = assign40990_e54394_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign41000_e54405, assign41000_e54405_d_n0, assign41000_e54405_d_n2, assign41000_e54405_d_n4, assign41000_e54405_d_n5, assign41000_e54405_d_n6, assign41000_e54405_d_n7, assign41000_e54405_d_n8, assign41000_e54405_d_n9, assign41000_e54405_d_n10, assign41000_e54405_d_n11, assign41000_e54405_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41000_e54405;
        locals.var_dnm_dn0 = assign41000_e54405_d_n0;
        locals.var_dnm_dn2 = assign41000_e54405_d_n2;
        locals.var_dnm_dn4 = assign41000_e54405_d_n4;
        locals.var_dnm_dn5 = assign41000_e54405_d_n5;
        locals.var_dnm_dn6 = assign41000_e54405_d_n6;
        locals.var_dnm_dn7 = assign41000_e54405_d_n7;
        locals.var_dnm_dn8 = assign41000_e54405_d_n8;
        locals.var_dnm_dn9 = assign41000_e54405_d_n9;
        locals.var_dnm_dn10 = assign41000_e54405_d_n10;
        locals.var_dnm_dn11 = assign41000_e54405_d_n11;
        locals.var_dnm_dn14 = assign41000_e54405_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign41010_e54420: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1022 = assign41010_e54420;
        locals.var_guard1022_rv = 0.0;

        let assign41020_e54423: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1023 = assign41020_e54423;
        locals.var_guard1023_rv = 0.0;

        let (assign41030_e54438,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) && (locals.var_guard1022 != 0.0)) && (locals.var_guard1023 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41030_e54438;
        locals.var_mm_rv = 0.0;

        let assign41040_e54441: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1024 = assign41040_e54441;
        locals.var_guard1024_rv = 0.0;

        let (assign41050_e54459,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) && (locals.var_guard1022 != 0.0)) && (locals.var_guard1023 == 0.0)) && (locals.var_guard1024 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41050_e54459;
        locals.var_mm_rv = 0.0;

        let assign41060_e54462: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1025 = assign41060_e54462;
        locals.var_guard1025_rv = 0.0;

        let (assign41070_e54483,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) && (locals.var_guard1022 != 0.0)) && (locals.var_guard1023 == 0.0)) && (locals.var_guard1024 == 0.0)) && (locals.var_guard1025 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41070_e54483;
        locals.var_mm_rv = 0.0;

        let assign41080_e54486: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1026 = assign41080_e54486;
        locals.var_guard1026_rv = 0.0;

        let (assign41090_e54510,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) && (locals.var_guard1022 != 0.0)) && (locals.var_guard1023 == 0.0)) && (locals.var_guard1024 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1026 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41090_e54510;
        locals.var_mm_rv = 0.0;

        let (assign41100_e54523,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) && (locals.var_guard1022 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41100_e54523;
        locals.var_m0_rv = 0.0;

        let mut assign41110_loop_guard: usize = 0;
        while {
            let assign41110_cond_e54537: f64 = if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) && (locals.var_guard1022 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign41110_cond_e54537 != 0.0
        } {
            assign41110_loop_guard += 1;
            assert!(assign41110_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41110_body0_e54551, assign41110_body0_e54551_d_n0, assign41110_body0_e54551_d_n2, assign41110_body0_e54551_d_n4, assign41110_body0_e54551_d_n5, assign41110_body0_e54551_d_n6, assign41110_body0_e54551_d_n7, assign41110_body0_e54551_d_n8, assign41110_body0_e54551_d_n9, assign41110_body0_e54551_d_n10, assign41110_body0_e54551_d_n11, assign41110_body0_e54551_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) && (locals.var_guard1022 != 0.0)) {
        let assign41110_body0_e54549: f64 = (locals.var_dnm).sqrt();
        (assign41110_body0_e54549, (locals.var_dnm_dn0 / (2.0 * assign41110_body0_e54549)), (locals.var_dnm_dn2 / (2.0 * assign41110_body0_e54549)), (locals.var_dnm_dn4 / (2.0 * assign41110_body0_e54549)), (locals.var_dnm_dn5 / (2.0 * assign41110_body0_e54549)), (locals.var_dnm_dn6 / (2.0 * assign41110_body0_e54549)), (locals.var_dnm_dn7 / (2.0 * assign41110_body0_e54549)), (locals.var_dnm_dn8 / (2.0 * assign41110_body0_e54549)), (locals.var_dnm_dn9 / (2.0 * assign41110_body0_e54549)), (locals.var_dnm_dn10 / (2.0 * assign41110_body0_e54549)), (locals.var_dnm_dn11 / (2.0 * assign41110_body0_e54549)), (locals.var_dnm_dn14 / (2.0 * assign41110_body0_e54549)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign41110_body0_e54551;
            locals.var_dnm_dn0 = assign41110_body0_e54551_d_n0;
            locals.var_dnm_dn2 = assign41110_body0_e54551_d_n2;
            locals.var_dnm_dn4 = assign41110_body0_e54551_d_n4;
            locals.var_dnm_dn5 = assign41110_body0_e54551_d_n5;
            locals.var_dnm_dn6 = assign41110_body0_e54551_d_n6;
            locals.var_dnm_dn7 = assign41110_body0_e54551_d_n7;
            locals.var_dnm_dn8 = assign41110_body0_e54551_d_n8;
            locals.var_dnm_dn9 = assign41110_body0_e54551_d_n9;
            locals.var_dnm_dn10 = assign41110_body0_e54551_d_n10;
            locals.var_dnm_dn11 = assign41110_body0_e54551_d_n11;
            locals.var_dnm_dn14 = assign41110_body0_e54551_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign41110_body1_e54566,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) && (locals.var_guard1022 != 0.0)) {
        let assign41110_body1_e54564: f64 = (locals.var_m0 + 1.0);
        (assign41110_body1_e54564,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41110_body1_e54566;
            locals.var_m0_rv = 0.0;
        }

        let (assign41120_e54591, assign41120_e54591_d_n0, assign41120_e54591_d_n2, assign41120_e54591_d_n4, assign41120_e54591_d_n5, assign41120_e54591_d_n6, assign41120_e54591_d_n7, assign41120_e54591_d_n8, assign41120_e54591_d_n9, assign41120_e54591_d_n10, assign41120_e54591_d_n11, assign41120_e54591_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) && (locals.var_guard1022 == 0.0)) {
        let (assign41120_e54589, assign41120_e54589_d_n0, assign41120_e54589_d_n2, assign41120_e54589_d_n4, assign41120_e54589_d_n5, assign41120_e54589_d_n6, assign41120_e54589_d_n7, assign41120_e54589_d_n8, assign41120_e54589_d_n9, assign41120_e54589_d_n10, assign41120_e54589_d_n11, assign41120_e54589_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign41120_e54586: f64 = (2.0 * 4.0);
                let assign41120_e54587: f64 = (1.0 / assign41120_e54586);
                let assign41120_e54588: f64 = (locals.var_dnm).powf(assign41120_e54587);
                (assign41120_e54588, if 0.0 == 0.0 && ((assign41120_e54587) as f64).is_finite() && ((assign41120_e54587) as f64).fract() == 0.0 { if assign41120_e54587 == 0.0 { 0.0 } else { (assign41120_e54587 * ((locals.var_dnm).powf(assign41120_e54587 - 1.0) * locals.var_dnm_dn0)) } } else { (assign41120_e54588 * (assign41120_e54587 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41120_e54587) as f64).is_finite() && ((assign41120_e54587) as f64).fract() == 0.0 { if assign41120_e54587 == 0.0 { 0.0 } else { (assign41120_e54587 * ((locals.var_dnm).powf(assign41120_e54587 - 1.0) * locals.var_dnm_dn2)) } } else { (assign41120_e54588 * (assign41120_e54587 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41120_e54587) as f64).is_finite() && ((assign41120_e54587) as f64).fract() == 0.0 { if assign41120_e54587 == 0.0 { 0.0 } else { (assign41120_e54587 * ((locals.var_dnm).powf(assign41120_e54587 - 1.0) * locals.var_dnm_dn4)) } } else { (assign41120_e54588 * (assign41120_e54587 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41120_e54587) as f64).is_finite() && ((assign41120_e54587) as f64).fract() == 0.0 { if assign41120_e54587 == 0.0 { 0.0 } else { (assign41120_e54587 * ((locals.var_dnm).powf(assign41120_e54587 - 1.0) * locals.var_dnm_dn5)) } } else { (assign41120_e54588 * (assign41120_e54587 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41120_e54587) as f64).is_finite() && ((assign41120_e54587) as f64).fract() == 0.0 { if assign41120_e54587 == 0.0 { 0.0 } else { (assign41120_e54587 * ((locals.var_dnm).powf(assign41120_e54587 - 1.0) * locals.var_dnm_dn6)) } } else { (assign41120_e54588 * (assign41120_e54587 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41120_e54587) as f64).is_finite() && ((assign41120_e54587) as f64).fract() == 0.0 { if assign41120_e54587 == 0.0 { 0.0 } else { (assign41120_e54587 * ((locals.var_dnm).powf(assign41120_e54587 - 1.0) * locals.var_dnm_dn7)) } } else { (assign41120_e54588 * (assign41120_e54587 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41120_e54587) as f64).is_finite() && ((assign41120_e54587) as f64).fract() == 0.0 { if assign41120_e54587 == 0.0 { 0.0 } else { (assign41120_e54587 * ((locals.var_dnm).powf(assign41120_e54587 - 1.0) * locals.var_dnm_dn8)) } } else { (assign41120_e54588 * (assign41120_e54587 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41120_e54587) as f64).is_finite() && ((assign41120_e54587) as f64).fract() == 0.0 { if assign41120_e54587 == 0.0 { 0.0 } else { (assign41120_e54587 * ((locals.var_dnm).powf(assign41120_e54587 - 1.0) * locals.var_dnm_dn9)) } } else { (assign41120_e54588 * (assign41120_e54587 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41120_e54587) as f64).is_finite() && ((assign41120_e54587) as f64).fract() == 0.0 { if assign41120_e54587 == 0.0 { 0.0 } else { (assign41120_e54587 * ((locals.var_dnm).powf(assign41120_e54587 - 1.0) * locals.var_dnm_dn10)) } } else { (assign41120_e54588 * (assign41120_e54587 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41120_e54587) as f64).is_finite() && ((assign41120_e54587) as f64).fract() == 0.0 { if assign41120_e54587 == 0.0 { 0.0 } else { (assign41120_e54587 * ((locals.var_dnm).powf(assign41120_e54587 - 1.0) * locals.var_dnm_dn11)) } } else { (assign41120_e54588 * (assign41120_e54587 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41120_e54587) as f64).is_finite() && ((assign41120_e54587) as f64).fract() == 0.0 { if assign41120_e54587 == 0.0 { 0.0 } else { (assign41120_e54587 * ((locals.var_dnm).powf(assign41120_e54587 - 1.0) * locals.var_dnm_dn14)) } } else { (assign41120_e54588 * (assign41120_e54587 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign41120_e54589, assign41120_e54589_d_n0, assign41120_e54589_d_n2, assign41120_e54589_d_n4, assign41120_e54589_d_n5, assign41120_e54589_d_n6, assign41120_e54589_d_n7, assign41120_e54589_d_n8, assign41120_e54589_d_n9, assign41120_e54589_d_n10, assign41120_e54589_d_n11, assign41120_e54589_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41120_e54591;
        locals.var_dnm_dn0 = assign41120_e54591_d_n0;
        locals.var_dnm_dn2 = assign41120_e54591_d_n2;
        locals.var_dnm_dn4 = assign41120_e54591_d_n4;
        locals.var_dnm_dn5 = assign41120_e54591_d_n5;
        locals.var_dnm_dn6 = assign41120_e54591_d_n6;
        locals.var_dnm_dn7 = assign41120_e54591_d_n7;
        locals.var_dnm_dn8 = assign41120_e54591_d_n8;
        locals.var_dnm_dn9 = assign41120_e54591_d_n9;
        locals.var_dnm_dn10 = assign41120_e54591_d_n10;
        locals.var_dnm_dn11 = assign41120_e54591_d_n11;
        locals.var_dnm_dn14 = assign41120_e54591_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41130_e54604, assign41130_e54604_d_n0, assign41130_e54604_d_n2, assign41130_e54604_d_n4, assign41130_e54604_d_n5, assign41130_e54604_d_n6, assign41130_e54604_d_n7, assign41130_e54604_d_n8, assign41130_e54604_d_n9, assign41130_e54604_d_n10, assign41130_e54604_d_n11, assign41130_e54604_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign41130_e54602: f64 = (1.0 / locals.var_dnm);
        (assign41130_e54602, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41130_e54604;
        locals.var_dnm_dn0 = assign41130_e54604_d_n0;
        locals.var_dnm_dn2 = assign41130_e54604_d_n2;
        locals.var_dnm_dn4 = assign41130_e54604_d_n4;
        locals.var_dnm_dn5 = assign41130_e54604_d_n5;
        locals.var_dnm_dn6 = assign41130_e54604_d_n6;
        locals.var_dnm_dn7 = assign41130_e54604_d_n7;
        locals.var_dnm_dn8 = assign41130_e54604_d_n8;
        locals.var_dnm_dn9 = assign41130_e54604_d_n9;
        locals.var_dnm_dn10 = assign41130_e54604_d_n10;
        locals.var_dnm_dn11 = assign41130_e54604_d_n11;
        locals.var_dnm_dn14 = assign41130_e54604_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41140_e54619, assign41140_e54619_d_n0, assign41140_e54619_d_n2, assign41140_e54619_d_n4, assign41140_e54619_d_n5, assign41140_e54619_d_n6, assign41140_e54619_d_n7, assign41140_e54619_d_n8, assign41140_e54619_d_n9, assign41140_e54619_d_n10, assign41140_e54619_d_n11, assign41140_e54619_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign41140_e54615: f64 = (locals.var_tmf1 * locals.var_ps_delta);
        let assign41140_e54617: f64 = (assign41140_e54615 * locals.var_dnm);
        (assign41140_e54617, (((locals.var_tmf1_dn0 * locals.var_ps_delta) * locals.var_dnm) + (assign41140_e54615 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_ps_delta) * locals.var_dnm) + (assign41140_e54615 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_ps_delta) * locals.var_dnm) + (assign41140_e54615 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_ps_delta) * locals.var_dnm) + (assign41140_e54615 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_ps_delta) * locals.var_dnm) + (assign41140_e54615 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_ps_delta) * locals.var_dnm) + (assign41140_e54615 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_ps_delta) * locals.var_dnm) + (assign41140_e54615 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_ps_delta) * locals.var_dnm) + (assign41140_e54615 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_ps_delta) * locals.var_dnm) + (assign41140_e54615 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * locals.var_ps_delta) * locals.var_dnm) + (assign41140_e54615 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * locals.var_ps_delta) * locals.var_dnm) + (assign41140_e54615 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign41140_e54619;
        locals.var_tmf0_dn0 = assign41140_e54619_d_n0;
        locals.var_tmf0_dn2 = assign41140_e54619_d_n2;
        locals.var_tmf0_dn4 = assign41140_e54619_d_n4;
        locals.var_tmf0_dn5 = assign41140_e54619_d_n5;
        locals.var_tmf0_dn6 = assign41140_e54619_d_n6;
        locals.var_tmf0_dn7 = assign41140_e54619_d_n7;
        locals.var_tmf0_dn8 = assign41140_e54619_d_n8;
        locals.var_tmf0_dn9 = assign41140_e54619_d_n9;
        locals.var_tmf0_dn10 = assign41140_e54619_d_n10;
        locals.var_tmf0_dn11 = assign41140_e54619_d_n11;
        locals.var_tmf0_dn14 = assign41140_e54619_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign41150_e54636, assign41150_e54636_d_n0, assign41150_e54636_d_n2, assign41150_e54636_d_n4, assign41150_e54636_d_n5, assign41150_e54636_d_n6, assign41150_e54636_d_n7, assign41150_e54636_d_n8, assign41150_e54636_d_n9, assign41150_e54636_d_n10, assign41150_e54636_d_n11, assign41150_e54636_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign41150_e54630: f64 = (locals.var_ps_delta * locals.var_xmp);
        let assign41150_e54632: f64 = (assign41150_e54630 * locals.var_dnm);
        let assign41150_e54634: f64 = (assign41150_e54632 / locals.var_arg);
        (assign41150_e54634, ((((((locals.var_ps_delta * locals.var_xmp_dn0) * locals.var_dnm) + (assign41150_e54630 * locals.var_dnm_dn0)) * locals.var_arg) - (assign41150_e54632 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn2) * locals.var_dnm) + (assign41150_e54630 * locals.var_dnm_dn2)) * locals.var_arg) - (assign41150_e54632 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn4) * locals.var_dnm) + (assign41150_e54630 * locals.var_dnm_dn4)) * locals.var_arg) - (assign41150_e54632 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn5) * locals.var_dnm) + (assign41150_e54630 * locals.var_dnm_dn5)) * locals.var_arg) - (assign41150_e54632 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn6) * locals.var_dnm) + (assign41150_e54630 * locals.var_dnm_dn6)) * locals.var_arg) - (assign41150_e54632 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn7) * locals.var_dnm) + (assign41150_e54630 * locals.var_dnm_dn7)) * locals.var_arg) - (assign41150_e54632 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn8) * locals.var_dnm) + (assign41150_e54630 * locals.var_dnm_dn8)) * locals.var_arg) - (assign41150_e54632 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn9) * locals.var_dnm) + (assign41150_e54630 * locals.var_dnm_dn9)) * locals.var_arg) - (assign41150_e54632 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn10) * locals.var_dnm) + (assign41150_e54630 * locals.var_dnm_dn10)) * locals.var_arg) - (assign41150_e54632 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn11) * locals.var_dnm) + (assign41150_e54630 * locals.var_dnm_dn11)) * locals.var_arg) - (assign41150_e54632 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn14) * locals.var_dnm) + (assign41150_e54630 * locals.var_dnm_dn14)) * locals.var_arg) - (assign41150_e54632 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41150_e54636;
        locals.var_t0_dn0 = assign41150_e54636_d_n0;
        locals.var_t0_dn2 = assign41150_e54636_d_n2;
        locals.var_t0_dn4 = assign41150_e54636_d_n4;
        locals.var_t0_dn5 = assign41150_e54636_d_n5;
        locals.var_t0_dn6 = assign41150_e54636_d_n6;
        locals.var_t0_dn7 = assign41150_e54636_d_n7;
        locals.var_t0_dn8 = assign41150_e54636_d_n8;
        locals.var_t0_dn9 = assign41150_e54636_d_n9;
        locals.var_t0_dn10 = assign41150_e54636_d_n10;
        locals.var_t0_dn11 = assign41150_e54636_d_n11;
        locals.var_t0_dn14 = assign41150_e54636_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41160_e54651, assign41160_e54651_d_n0, assign41160_e54651_d_n2, assign41160_e54651_d_n4, assign41160_e54651_d_n5, assign41160_e54651_d_n6, assign41160_e54651_d_n7, assign41160_e54651_d_n8, assign41160_e54651_d_n9, assign41160_e54651_d_n10, assign41160_e54651_d_n11, assign41160_e54651_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        let assign41160_e54647: f64 = locals.var_ps_delta;
        let assign41160_e54649: f64 = (assign41160_e54647 - locals.var_tmf0);
        (assign41160_e54649, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign41160_e54651;
        locals.var_t2_dn0 = assign41160_e54651_d_n0;
        locals.var_t2_dn2 = assign41160_e54651_d_n2;
        locals.var_t2_dn4 = assign41160_e54651_d_n4;
        locals.var_t2_dn5 = assign41160_e54651_d_n5;
        locals.var_t2_dn6 = assign41160_e54651_d_n6;
        locals.var_t2_dn7 = assign41160_e54651_d_n7;
        locals.var_t2_dn8 = assign41160_e54651_d_n8;
        locals.var_t2_dn9 = assign41160_e54651_d_n9;
        locals.var_t2_dn10 = assign41160_e54651_d_n10;
        locals.var_t2_dn11 = assign41160_e54651_d_n11;
        locals.var_t2_dn14 = assign41160_e54651_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign41170_e54662, assign41170_e54662_d_n0, assign41170_e54662_d_n2, assign41170_e54662_d_n4, assign41170_e54662_d_n5, assign41170_e54662_d_n6, assign41170_e54662_d_n7, assign41170_e54662_d_n8, assign41170_e54662_d_n9, assign41170_e54662_d_n10, assign41170_e54662_d_n11, assign41170_e54662_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41170_e54662;
        locals.var_t0_dn0 = assign41170_e54662_d_n0;
        locals.var_t0_dn2 = assign41170_e54662_d_n2;
        locals.var_t0_dn4 = assign41170_e54662_d_n4;
        locals.var_t0_dn5 = assign41170_e54662_d_n5;
        locals.var_t0_dn6 = assign41170_e54662_d_n6;
        locals.var_t0_dn7 = assign41170_e54662_d_n7;
        locals.var_t0_dn8 = assign41170_e54662_d_n8;
        locals.var_t0_dn9 = assign41170_e54662_d_n9;
        locals.var_t0_dn10 = assign41170_e54662_d_n10;
        locals.var_t0_dn11 = assign41170_e54662_d_n11;
        locals.var_t0_dn14 = assign41170_e54662_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_141(
        locals: &mut StampLocals,
    ) {
        let (assign41180_e54676, assign41180_e54676_d_n0, assign41180_e54676_d_n2, assign41180_e54676_d_n4, assign41180_e54676_d_n5, assign41180_e54676_d_n6, assign41180_e54676_d_n7, assign41180_e54676_d_n8, assign41180_e54676_d_n9, assign41180_e54676_d_n10, assign41180_e54676_d_n11, assign41180_e54676_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 == 0.0)) {
        let assign41180_e54674: f64 = (locals.var_ps0z - locals.var_vds_maxb0__blk853);
        (assign41180_e54674, locals.var_ps0z_dn0, locals.var_ps0z_dn2, locals.var_ps0z_dn4, locals.var_ps0z_dn5, locals.var_ps0z_dn6, locals.var_ps0z_dn7, locals.var_ps0z_dn8, locals.var_ps0z_dn9, locals.var_ps0z_dn10, locals.var_ps0z_dn11, locals.var_ps0z_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign41180_e54676;
        locals.var_t2_dn0 = assign41180_e54676_d_n0;
        locals.var_t2_dn2 = assign41180_e54676_d_n2;
        locals.var_t2_dn4 = assign41180_e54676_d_n4;
        locals.var_t2_dn5 = assign41180_e54676_d_n5;
        locals.var_t2_dn6 = assign41180_e54676_d_n6;
        locals.var_t2_dn7 = assign41180_e54676_d_n7;
        locals.var_t2_dn8 = assign41180_e54676_d_n8;
        locals.var_t2_dn9 = assign41180_e54676_d_n9;
        locals.var_t2_dn10 = assign41180_e54676_d_n10;
        locals.var_t2_dn11 = assign41180_e54676_d_n11;
        locals.var_t2_dn14 = assign41180_e54676_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign41190_e54688, assign41190_e54688_d_n0, assign41190_e54688_d_n2, assign41190_e54688_d_n4, assign41190_e54688_d_n5, assign41190_e54688_d_n6, assign41190_e54688_d_n7, assign41190_e54688_d_n8, assign41190_e54688_d_n9, assign41190_e54688_d_n10, assign41190_e54688_d_n11, assign41190_e54688_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1021 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41190_e54688;
        locals.var_t0_dn0 = assign41190_e54688_d_n0;
        locals.var_t0_dn2 = assign41190_e54688_d_n2;
        locals.var_t0_dn4 = assign41190_e54688_d_n4;
        locals.var_t0_dn5 = assign41190_e54688_d_n5;
        locals.var_t0_dn6 = assign41190_e54688_d_n6;
        locals.var_t0_dn7 = assign41190_e54688_d_n7;
        locals.var_t0_dn8 = assign41190_e54688_d_n8;
        locals.var_t0_dn9 = assign41190_e54688_d_n9;
        locals.var_t0_dn10 = assign41190_e54688_d_n10;
        locals.var_t0_dn11 = assign41190_e54688_d_n11;
        locals.var_t0_dn14 = assign41190_e54688_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41200_e54708, assign41200_e54708_d_n0, assign41200_e54708_d_n2, assign41200_e54708_d_n4, assign41200_e54708_d_n5, assign41200_e54708_d_n6, assign41200_e54708_d_n7, assign41200_e54708_d_n8, assign41200_e54708_d_n9, assign41200_e54708_d_n10, assign41200_e54708_d_n11, assign41200_e54708_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign41200_e54697: f64 = (locals.var_beta * locals.var_t2);
        let assign41200_e54698: f64 = (assign41200_e54697).exp();
        let assign41200_e54700: f64 = (assign41200_e54698 - 1.0);
        let assign41200_e54703: f64 = (locals.var_beta * locals.var_t2);
        let assign41200_e54704: f64 = (assign41200_e54700 - assign41200_e54703);
        let assign41200_e54706: f64 = (assign41200_e54704 + 1e-15);
        (assign41200_e54706, ((assign41200_e54698 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign41200_e54698 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign41200_e54698 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign41200_e54698 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign41200_e54698 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign41200_e54698 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign41200_e54698 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign41200_e54698 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign41200_e54698 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign41200_e54698 * ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))) - ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))), ((assign41200_e54698 * ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))) - ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign41200_e54708;
        locals.var_t4_dn0 = assign41200_e54708_d_n0;
        locals.var_t4_dn2 = assign41200_e54708_d_n2;
        locals.var_t4_dn4 = assign41200_e54708_d_n4;
        locals.var_t4_dn5 = assign41200_e54708_d_n5;
        locals.var_t4_dn6 = assign41200_e54708_d_n6;
        locals.var_t4_dn7 = assign41200_e54708_d_n7;
        locals.var_t4_dn8 = assign41200_e54708_d_n8;
        locals.var_t4_dn9 = assign41200_e54708_d_n9;
        locals.var_t4_dn10 = assign41200_e54708_d_n10;
        locals.var_t4_dn11 = assign41200_e54708_d_n11;
        locals.var_t4_dn14 = assign41200_e54708_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign41210_e54721, assign41210_e54721_d_n0, assign41210_e54721_d_n2, assign41210_e54721_d_n4, assign41210_e54721_d_n5, assign41210_e54721_d_n6, assign41210_e54721_d_n7, assign41210_e54721_d_n8, assign41210_e54721_d_n9, assign41210_e54721_d_n10, assign41210_e54721_d_n11, assign41210_e54721_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign41210_e54716: f64 = (-locals.var_cnst0);
        let assign41210_e54718: f64 = (locals.var_t4).sqrt();
        let assign41210_e54719: f64 = (assign41210_e54716 * assign41210_e54718);
        (assign41210_e54719, (((-locals.var_cnst0_dn0) * assign41210_e54718) + (assign41210_e54716 * (locals.var_t4_dn0 / (2.0 * assign41210_e54718)))), (((-locals.var_cnst0_dn2) * assign41210_e54718) + (assign41210_e54716 * (locals.var_t4_dn2 / (2.0 * assign41210_e54718)))), (((-locals.var_cnst0_dn4) * assign41210_e54718) + (assign41210_e54716 * (locals.var_t4_dn4 / (2.0 * assign41210_e54718)))), (((-locals.var_cnst0_dn5) * assign41210_e54718) + (assign41210_e54716 * (locals.var_t4_dn5 / (2.0 * assign41210_e54718)))), (((-locals.var_cnst0_dn6) * assign41210_e54718) + (assign41210_e54716 * (locals.var_t4_dn6 / (2.0 * assign41210_e54718)))), (((-locals.var_cnst0_dn7) * assign41210_e54718) + (assign41210_e54716 * (locals.var_t4_dn7 / (2.0 * assign41210_e54718)))), (((-locals.var_cnst0_dn8) * assign41210_e54718) + (assign41210_e54716 * (locals.var_t4_dn8 / (2.0 * assign41210_e54718)))), (((-locals.var_cnst0_dn9) * assign41210_e54718) + (assign41210_e54716 * (locals.var_t4_dn9 / (2.0 * assign41210_e54718)))), (((-locals.var_cnst0_dn10) * assign41210_e54718) + (assign41210_e54716 * (locals.var_t4_dn10 / (2.0 * assign41210_e54718)))), (((-locals.var_cnst0_dn11) * assign41210_e54718) + (assign41210_e54716 * (locals.var_t4_dn11 / (2.0 * assign41210_e54718)))), (((-locals.var_cnst0_dn14) * assign41210_e54718) + (assign41210_e54716 * (locals.var_t4_dn14 / (2.0 * assign41210_e54718)))),)
    } else {
        (locals.var_q_n0_sym, locals.var_q_n0_sym_dn0, locals.var_q_n0_sym_dn2, locals.var_q_n0_sym_dn4, locals.var_q_n0_sym_dn5, locals.var_q_n0_sym_dn6, locals.var_q_n0_sym_dn7, locals.var_q_n0_sym_dn8, locals.var_q_n0_sym_dn9, locals.var_q_n0_sym_dn10, locals.var_q_n0_sym_dn11, locals.var_q_n0_sym_dn14,)
    }
};
        locals.var_q_n0_sym = assign41210_e54721;
        locals.var_q_n0_sym_dn0 = assign41210_e54721_d_n0;
        locals.var_q_n0_sym_dn2 = assign41210_e54721_d_n2;
        locals.var_q_n0_sym_dn4 = assign41210_e54721_d_n4;
        locals.var_q_n0_sym_dn5 = assign41210_e54721_d_n5;
        locals.var_q_n0_sym_dn6 = assign41210_e54721_d_n6;
        locals.var_q_n0_sym_dn7 = assign41210_e54721_d_n7;
        locals.var_q_n0_sym_dn8 = assign41210_e54721_d_n8;
        locals.var_q_n0_sym_dn9 = assign41210_e54721_d_n9;
        locals.var_q_n0_sym_dn10 = assign41210_e54721_d_n10;
        locals.var_q_n0_sym_dn11 = assign41210_e54721_d_n11;
        locals.var_q_n0_sym_dn14 = assign41210_e54721_d_n14;
        locals.var_q_n0_sym_rv = 0.0;

        let assign41220_e54724: f64 = if locals.var_w_bsub0__blk838 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard1032 = assign41220_e54724;
        locals.var_guard1032_rv = 0.0;

        let (assign41230_e54735, assign41230_e54735_d_n0, assign41230_e54735_d_n2, assign41230_e54735_d_n4, assign41230_e54735_d_n5, assign41230_e54735_d_n6, assign41230_e54735_d_n7, assign41230_e54735_d_n8, assign41230_e54735_d_n9, assign41230_e54735_d_n10, assign41230_e54735_d_n11, assign41230_e54735_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 != 0.0)) {
        (locals.var_tnp, locals.var_tnp_dn0, locals.var_tnp_dn2, locals.var_tnp_dn4, locals.var_tnp_dn5, locals.var_tnp_dn6, locals.var_tnp_dn7, locals.var_tnp_dn8, locals.var_tnp_dn9, locals.var_tnp_dn10, locals.var_tnp_dn11, locals.var_tnp_dn14,)
    } else {
        (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn11, locals.var_ws_dn14,)
    }
};
        locals.var_ws = assign41230_e54735;
        locals.var_ws_dn0 = assign41230_e54735_d_n0;
        locals.var_ws_dn2 = assign41230_e54735_d_n2;
        locals.var_ws_dn4 = assign41230_e54735_d_n4;
        locals.var_ws_dn5 = assign41230_e54735_d_n5;
        locals.var_ws_dn6 = assign41230_e54735_d_n6;
        locals.var_ws_dn7 = assign41230_e54735_d_n7;
        locals.var_ws_dn8 = assign41230_e54735_d_n8;
        locals.var_ws_dn9 = assign41230_e54735_d_n9;
        locals.var_ws_dn10 = assign41230_e54735_d_n10;
        locals.var_ws_dn11 = assign41230_e54735_d_n11;
        locals.var_ws_dn14 = assign41230_e54735_d_n14;
        locals.var_ws_rv = 0.0;

        let assign41240_e54739: f64 = (-0.1);
        let assign41240_e54744: f64 = if ((locals.var_ps0 > assign41240_e54739) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1033 = assign41240_e54744;
        locals.var_guard1033_rv = 0.0;

        let (assign41250_e54762, assign41250_e54762_d_n0, assign41250_e54762_d_n2, assign41250_e54762_d_n4, assign41250_e54762_d_n5, assign41250_e54762_d_n6, assign41250_e54762_d_n7, assign41250_e54762_d_n8, assign41250_e54762_d_n9, assign41250_e54762_d_n10, assign41250_e54762_d_n11, assign41250_e54762_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        let assign41250_e54758: f64 = locals.var_ps0;
        let assign41250_e54760: f64 = (assign41250_e54758 + 0.1);
        (assign41250_e54760, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign41250_e54762;
        locals.var_tmf1_dn0 = assign41250_e54762_d_n0;
        locals.var_tmf1_dn2 = assign41250_e54762_d_n2;
        locals.var_tmf1_dn4 = assign41250_e54762_d_n4;
        locals.var_tmf1_dn5 = assign41250_e54762_d_n5;
        locals.var_tmf1_dn6 = assign41250_e54762_d_n6;
        locals.var_tmf1_dn7 = assign41250_e54762_d_n7;
        locals.var_tmf1_dn8 = assign41250_e54762_d_n8;
        locals.var_tmf1_dn9 = assign41250_e54762_d_n9;
        locals.var_tmf1_dn10 = assign41250_e54762_d_n10;
        locals.var_tmf1_dn11 = assign41250_e54762_d_n11;
        locals.var_tmf1_dn14 = assign41250_e54762_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign41260_e54778, assign41260_e54778_d_n0, assign41260_e54778_d_n2, assign41260_e54778_d_n4, assign41260_e54778_d_n5, assign41260_e54778_d_n6, assign41260_e54778_d_n7, assign41260_e54778_d_n8, assign41260_e54778_d_n9, assign41260_e54778_d_n10, assign41260_e54778_d_n11, assign41260_e54778_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        let assign41260_e54776: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign41260_e54776, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign41260_e54778;
        locals.var_x2_dn0 = assign41260_e54778_d_n0;
        locals.var_x2_dn2 = assign41260_e54778_d_n2;
        locals.var_x2_dn4 = assign41260_e54778_d_n4;
        locals.var_x2_dn5 = assign41260_e54778_d_n5;
        locals.var_x2_dn6 = assign41260_e54778_d_n6;
        locals.var_x2_dn7 = assign41260_e54778_d_n7;
        locals.var_x2_dn8 = assign41260_e54778_d_n8;
        locals.var_x2_dn9 = assign41260_e54778_d_n9;
        locals.var_x2_dn10 = assign41260_e54778_d_n10;
        locals.var_x2_dn11 = assign41260_e54778_d_n11;
        locals.var_x2_dn14 = assign41260_e54778_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign41270_e54794, assign41270_e54794_d_n0, assign41270_e54794_d_n2, assign41270_e54794_d_n4, assign41270_e54794_d_n5, assign41270_e54794_d_n6, assign41270_e54794_d_n7, assign41270_e54794_d_n8, assign41270_e54794_d_n9, assign41270_e54794_d_n10, assign41270_e54794_d_n11, assign41270_e54794_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        let assign41270_e54792: f64 = (0.1 * 0.1);
        (assign41270_e54792, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign41270_e54794;
        locals.var_xmax2_dn0 = assign41270_e54794_d_n0;
        locals.var_xmax2_dn2 = assign41270_e54794_d_n2;
        locals.var_xmax2_dn4 = assign41270_e54794_d_n4;
        locals.var_xmax2_dn5 = assign41270_e54794_d_n5;
        locals.var_xmax2_dn6 = assign41270_e54794_d_n6;
        locals.var_xmax2_dn7 = assign41270_e54794_d_n7;
        locals.var_xmax2_dn8 = assign41270_e54794_d_n8;
        locals.var_xmax2_dn9 = assign41270_e54794_d_n9;
        locals.var_xmax2_dn10 = assign41270_e54794_d_n10;
        locals.var_xmax2_dn11 = assign41270_e54794_d_n11;
        locals.var_xmax2_dn14 = assign41270_e54794_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign41280_e54808, assign41280_e54808_d_n0, assign41280_e54808_d_n2, assign41280_e54808_d_n4, assign41280_e54808_d_n5, assign41280_e54808_d_n6, assign41280_e54808_d_n7, assign41280_e54808_d_n8, assign41280_e54808_d_n9, assign41280_e54808_d_n10, assign41280_e54808_d_n11, assign41280_e54808_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign41280_e54808;
        locals.var_xp_dn0 = assign41280_e54808_d_n0;
        locals.var_xp_dn2 = assign41280_e54808_d_n2;
        locals.var_xp_dn4 = assign41280_e54808_d_n4;
        locals.var_xp_dn5 = assign41280_e54808_d_n5;
        locals.var_xp_dn6 = assign41280_e54808_d_n6;
        locals.var_xp_dn7 = assign41280_e54808_d_n7;
        locals.var_xp_dn8 = assign41280_e54808_d_n8;
        locals.var_xp_dn9 = assign41280_e54808_d_n9;
        locals.var_xp_dn10 = assign41280_e54808_d_n10;
        locals.var_xp_dn11 = assign41280_e54808_d_n11;
        locals.var_xp_dn14 = assign41280_e54808_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign41290_e54822, assign41290_e54822_d_n0, assign41290_e54822_d_n2, assign41290_e54822_d_n4, assign41290_e54822_d_n5, assign41290_e54822_d_n6, assign41290_e54822_d_n7, assign41290_e54822_d_n8, assign41290_e54822_d_n9, assign41290_e54822_d_n10, assign41290_e54822_d_n11, assign41290_e54822_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign41290_e54822;
        locals.var_xmp_dn0 = assign41290_e54822_d_n0;
        locals.var_xmp_dn2 = assign41290_e54822_d_n2;
        locals.var_xmp_dn4 = assign41290_e54822_d_n4;
        locals.var_xmp_dn5 = assign41290_e54822_d_n5;
        locals.var_xmp_dn6 = assign41290_e54822_d_n6;
        locals.var_xmp_dn7 = assign41290_e54822_d_n7;
        locals.var_xmp_dn8 = assign41290_e54822_d_n8;
        locals.var_xmp_dn9 = assign41290_e54822_d_n9;
        locals.var_xmp_dn10 = assign41290_e54822_d_n10;
        locals.var_xmp_dn11 = assign41290_e54822_d_n11;
        locals.var_xmp_dn14 = assign41290_e54822_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign41300_e54836,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41300_e54836;
        locals.var_m0_rv = 0.0;

        let (assign41310_e54850,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41310_e54850;
        locals.var_mm_rv = 0.0;

        let (assign41320_e54864, assign41320_e54864_d_n0, assign41320_e54864_d_n2, assign41320_e54864_d_n4, assign41320_e54864_d_n5, assign41320_e54864_d_n6, assign41320_e54864_d_n7, assign41320_e54864_d_n8, assign41320_e54864_d_n9, assign41320_e54864_d_n10, assign41320_e54864_d_n11, assign41320_e54864_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign41320_e54864;
        locals.var_arg_dn0 = assign41320_e54864_d_n0;
        locals.var_arg_dn2 = assign41320_e54864_d_n2;
        locals.var_arg_dn4 = assign41320_e54864_d_n4;
        locals.var_arg_dn5 = assign41320_e54864_d_n5;
        locals.var_arg_dn6 = assign41320_e54864_d_n6;
        locals.var_arg_dn7 = assign41320_e54864_d_n7;
        locals.var_arg_dn8 = assign41320_e54864_d_n8;
        locals.var_arg_dn9 = assign41320_e54864_d_n9;
        locals.var_arg_dn10 = assign41320_e54864_d_n10;
        locals.var_arg_dn11 = assign41320_e54864_d_n11;
        locals.var_arg_dn14 = assign41320_e54864_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign41330_e54878, assign41330_e54878_d_n0, assign41330_e54878_d_n2, assign41330_e54878_d_n4, assign41330_e54878_d_n5, assign41330_e54878_d_n6, assign41330_e54878_d_n7, assign41330_e54878_d_n8, assign41330_e54878_d_n9, assign41330_e54878_d_n10, assign41330_e54878_d_n11, assign41330_e54878_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41330_e54878;
        locals.var_dnm_dn0 = assign41330_e54878_d_n0;
        locals.var_dnm_dn2 = assign41330_e54878_d_n2;
        locals.var_dnm_dn4 = assign41330_e54878_d_n4;
        locals.var_dnm_dn5 = assign41330_e54878_d_n5;
        locals.var_dnm_dn6 = assign41330_e54878_d_n6;
        locals.var_dnm_dn7 = assign41330_e54878_d_n7;
        locals.var_dnm_dn8 = assign41330_e54878_d_n8;
        locals.var_dnm_dn9 = assign41330_e54878_d_n9;
        locals.var_dnm_dn10 = assign41330_e54878_d_n10;
        locals.var_dnm_dn11 = assign41330_e54878_d_n11;
        locals.var_dnm_dn14 = assign41330_e54878_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41340_e54892,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41340_e54892;
        locals.var_m0_rv = 0.0;

        let mut assign41350_loop_guard: usize = 0;
        while {
            let assign41350_cond_e54907: f64 = if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw)) { 1.0 } else { 0.0 };
            assign41350_cond_e54907 != 0.0
        } {
            assign41350_loop_guard += 1;
            assert!(assign41350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41350_body0_e54923, assign41350_body0_e54923_d_n0, assign41350_body0_e54923_d_n2, assign41350_body0_e54923_d_n4, assign41350_body0_e54923_d_n5, assign41350_body0_e54923_d_n6, assign41350_body0_e54923_d_n7, assign41350_body0_e54923_d_n8, assign41350_body0_e54923_d_n9, assign41350_body0_e54923_d_n10, assign41350_body0_e54923_d_n11, assign41350_body0_e54923_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        let assign41350_body0_e54921: f64 = (locals.var_xp * locals.var_x2);
        (assign41350_body0_e54921, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign41350_body0_e54923;
            locals.var_xp_dn0 = assign41350_body0_e54923_d_n0;
            locals.var_xp_dn2 = assign41350_body0_e54923_d_n2;
            locals.var_xp_dn4 = assign41350_body0_e54923_d_n4;
            locals.var_xp_dn5 = assign41350_body0_e54923_d_n5;
            locals.var_xp_dn6 = assign41350_body0_e54923_d_n6;
            locals.var_xp_dn7 = assign41350_body0_e54923_d_n7;
            locals.var_xp_dn8 = assign41350_body0_e54923_d_n8;
            locals.var_xp_dn9 = assign41350_body0_e54923_d_n9;
            locals.var_xp_dn10 = assign41350_body0_e54923_d_n10;
            locals.var_xp_dn11 = assign41350_body0_e54923_d_n11;
            locals.var_xp_dn14 = assign41350_body0_e54923_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign41350_body1_e54939, assign41350_body1_e54939_d_n0, assign41350_body1_e54939_d_n2, assign41350_body1_e54939_d_n4, assign41350_body1_e54939_d_n5, assign41350_body1_e54939_d_n6, assign41350_body1_e54939_d_n7, assign41350_body1_e54939_d_n8, assign41350_body1_e54939_d_n9, assign41350_body1_e54939_d_n10, assign41350_body1_e54939_d_n11, assign41350_body1_e54939_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        let assign41350_body1_e54937: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign41350_body1_e54937, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign41350_body1_e54939;
            locals.var_xmp_dn0 = assign41350_body1_e54939_d_n0;
            locals.var_xmp_dn2 = assign41350_body1_e54939_d_n2;
            locals.var_xmp_dn4 = assign41350_body1_e54939_d_n4;
            locals.var_xmp_dn5 = assign41350_body1_e54939_d_n5;
            locals.var_xmp_dn6 = assign41350_body1_e54939_d_n6;
            locals.var_xmp_dn7 = assign41350_body1_e54939_d_n7;
            locals.var_xmp_dn8 = assign41350_body1_e54939_d_n8;
            locals.var_xmp_dn9 = assign41350_body1_e54939_d_n9;
            locals.var_xmp_dn10 = assign41350_body1_e54939_d_n10;
            locals.var_xmp_dn11 = assign41350_body1_e54939_d_n11;
            locals.var_xmp_dn14 = assign41350_body1_e54939_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign41350_body2_e54955,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        let assign41350_body2_e54953: f64 = (locals.var_m0 + 1.0);
        (assign41350_body2_e54953,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41350_body2_e54955;
            locals.var_m0_rv = 0.0;
        }

        let (assign41360_e54971, assign41360_e54971_d_n0, assign41360_e54971_d_n2, assign41360_e54971_d_n4, assign41360_e54971_d_n5, assign41360_e54971_d_n6, assign41360_e54971_d_n7, assign41360_e54971_d_n8, assign41360_e54971_d_n9, assign41360_e54971_d_n10, assign41360_e54971_d_n11, assign41360_e54971_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        let assign41360_e54969: f64 = (locals.var_xp + locals.var_xmp);
        (assign41360_e54969, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign41360_e54971;
        locals.var_arg_dn0 = assign41360_e54971_d_n0;
        locals.var_arg_dn2 = assign41360_e54971_d_n2;
        locals.var_arg_dn4 = assign41360_e54971_d_n4;
        locals.var_arg_dn5 = assign41360_e54971_d_n5;
        locals.var_arg_dn6 = assign41360_e54971_d_n6;
        locals.var_arg_dn7 = assign41360_e54971_d_n7;
        locals.var_arg_dn8 = assign41360_e54971_d_n8;
        locals.var_arg_dn9 = assign41360_e54971_d_n9;
        locals.var_arg_dn10 = assign41360_e54971_d_n10;
        locals.var_arg_dn11 = assign41360_e54971_d_n11;
        locals.var_arg_dn14 = assign41360_e54971_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign41370_e54985, assign41370_e54985_d_n0, assign41370_e54985_d_n2, assign41370_e54985_d_n4, assign41370_e54985_d_n5, assign41370_e54985_d_n6, assign41370_e54985_d_n7, assign41370_e54985_d_n8, assign41370_e54985_d_n9, assign41370_e54985_d_n10, assign41370_e54985_d_n11, assign41370_e54985_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41370_e54985;
        locals.var_dnm_dn0 = assign41370_e54985_d_n0;
        locals.var_dnm_dn2 = assign41370_e54985_d_n2;
        locals.var_dnm_dn4 = assign41370_e54985_d_n4;
        locals.var_dnm_dn5 = assign41370_e54985_d_n5;
        locals.var_dnm_dn6 = assign41370_e54985_d_n6;
        locals.var_dnm_dn7 = assign41370_e54985_d_n7;
        locals.var_dnm_dn8 = assign41370_e54985_d_n8;
        locals.var_dnm_dn9 = assign41370_e54985_d_n9;
        locals.var_dnm_dn10 = assign41370_e54985_d_n10;
        locals.var_dnm_dn11 = assign41370_e54985_d_n11;
        locals.var_dnm_dn14 = assign41370_e54985_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign41380_e55000: f64 = if ((((locals.var_vgpdep_pw == 1.0) || (locals.var_vgpdep_pw == 2.0)) || (locals.var_vgpdep_pw == 4.0)) || (locals.var_vgpdep_pw == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1034 = assign41380_e55000;
        locals.var_guard1034_rv = 0.0;

        let assign41390_e55003: f64 = if locals.var_vgpdep_pw == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1035 = assign41390_e55003;
        locals.var_guard1035_rv = 0.0;

        let (assign41400_e55021,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) && (locals.var_guard1034 != 0.0)) && (locals.var_guard1035 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41400_e55021;
        locals.var_mm_rv = 0.0;

        let assign41410_e55024: f64 = if locals.var_vgpdep_pw == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1036 = assign41410_e55024;
        locals.var_guard1036_rv = 0.0;

        let (assign41420_e55045,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) && (locals.var_guard1034 != 0.0)) && (locals.var_guard1035 == 0.0)) && (locals.var_guard1036 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41420_e55045;
        locals.var_mm_rv = 0.0;

        let assign41430_e55048: f64 = if locals.var_vgpdep_pw == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1037 = assign41430_e55048;
        locals.var_guard1037_rv = 0.0;

        let (assign41440_e55072,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) && (locals.var_guard1034 != 0.0)) && (locals.var_guard1035 == 0.0)) && (locals.var_guard1036 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41440_e55072;
        locals.var_mm_rv = 0.0;

        let assign41450_e55075: f64 = if locals.var_vgpdep_pw == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1038 = assign41450_e55075;
        locals.var_guard1038_rv = 0.0;

        let (assign41460_e55102,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) && (locals.var_guard1034 != 0.0)) && (locals.var_guard1035 == 0.0)) && (locals.var_guard1036 == 0.0)) && (locals.var_guard1037 == 0.0)) && (locals.var_guard1038 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41460_e55102;
        locals.var_mm_rv = 0.0;

        let (assign41470_e55118,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) && (locals.var_guard1034 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41470_e55118;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_142(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign41480_loop_guard: usize = 0;
        while {
            let assign41480_cond_e55135: f64 = if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) && (locals.var_guard1034 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign41480_cond_e55135 != 0.0
        } {
            assign41480_loop_guard += 1;
            assert!(assign41480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41480_body0_e55152, assign41480_body0_e55152_d_n0, assign41480_body0_e55152_d_n2, assign41480_body0_e55152_d_n4, assign41480_body0_e55152_d_n5, assign41480_body0_e55152_d_n6, assign41480_body0_e55152_d_n7, assign41480_body0_e55152_d_n8, assign41480_body0_e55152_d_n9, assign41480_body0_e55152_d_n10, assign41480_body0_e55152_d_n11, assign41480_body0_e55152_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) && (locals.var_guard1034 != 0.0)) {
        let assign41480_body0_e55150: f64 = (locals.var_dnm).sqrt();
        (assign41480_body0_e55150, (locals.var_dnm_dn0 / (2.0 * assign41480_body0_e55150)), (locals.var_dnm_dn2 / (2.0 * assign41480_body0_e55150)), (locals.var_dnm_dn4 / (2.0 * assign41480_body0_e55150)), (locals.var_dnm_dn5 / (2.0 * assign41480_body0_e55150)), (locals.var_dnm_dn6 / (2.0 * assign41480_body0_e55150)), (locals.var_dnm_dn7 / (2.0 * assign41480_body0_e55150)), (locals.var_dnm_dn8 / (2.0 * assign41480_body0_e55150)), (locals.var_dnm_dn9 / (2.0 * assign41480_body0_e55150)), (locals.var_dnm_dn10 / (2.0 * assign41480_body0_e55150)), (locals.var_dnm_dn11 / (2.0 * assign41480_body0_e55150)), (locals.var_dnm_dn14 / (2.0 * assign41480_body0_e55150)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign41480_body0_e55152;
            locals.var_dnm_dn0 = assign41480_body0_e55152_d_n0;
            locals.var_dnm_dn2 = assign41480_body0_e55152_d_n2;
            locals.var_dnm_dn4 = assign41480_body0_e55152_d_n4;
            locals.var_dnm_dn5 = assign41480_body0_e55152_d_n5;
            locals.var_dnm_dn6 = assign41480_body0_e55152_d_n6;
            locals.var_dnm_dn7 = assign41480_body0_e55152_d_n7;
            locals.var_dnm_dn8 = assign41480_body0_e55152_d_n8;
            locals.var_dnm_dn9 = assign41480_body0_e55152_d_n9;
            locals.var_dnm_dn10 = assign41480_body0_e55152_d_n10;
            locals.var_dnm_dn11 = assign41480_body0_e55152_d_n11;
            locals.var_dnm_dn14 = assign41480_body0_e55152_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign41480_body1_e55170,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) && (locals.var_guard1034 != 0.0)) {
        let assign41480_body1_e55168: f64 = (locals.var_m0 + 1.0);
        (assign41480_body1_e55168,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41480_body1_e55170;
            locals.var_m0_rv = 0.0;
        }

        let (assign41490_e55198, assign41490_e55198_d_n0, assign41490_e55198_d_n2, assign41490_e55198_d_n4, assign41490_e55198_d_n5, assign41490_e55198_d_n6, assign41490_e55198_d_n7, assign41490_e55198_d_n8, assign41490_e55198_d_n9, assign41490_e55198_d_n10, assign41490_e55198_d_n11, assign41490_e55198_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) && (locals.var_guard1034 == 0.0)) {
        let (assign41490_e55196, assign41490_e55196_d_n0, assign41490_e55196_d_n2, assign41490_e55196_d_n4, assign41490_e55196_d_n5, assign41490_e55196_d_n6, assign41490_e55196_d_n7, assign41490_e55196_d_n8, assign41490_e55196_d_n9, assign41490_e55196_d_n10, assign41490_e55196_d_n11, assign41490_e55196_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign41490_e55193: f64 = (2.0 * locals.var_vgpdep_pw);
                let assign41490_e55194: f64 = (1.0 / assign41490_e55193);
                let assign41490_e55195: f64 = (locals.var_dnm).powf(assign41490_e55194);
                (assign41490_e55195, if 0.0 == 0.0 && ((assign41490_e55194) as f64).is_finite() && ((assign41490_e55194) as f64).fract() == 0.0 { if assign41490_e55194 == 0.0 { 0.0 } else { (assign41490_e55194 * ((locals.var_dnm).powf(assign41490_e55194 - 1.0) * locals.var_dnm_dn0)) } } else { (assign41490_e55195 * (assign41490_e55194 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41490_e55194) as f64).is_finite() && ((assign41490_e55194) as f64).fract() == 0.0 { if assign41490_e55194 == 0.0 { 0.0 } else { (assign41490_e55194 * ((locals.var_dnm).powf(assign41490_e55194 - 1.0) * locals.var_dnm_dn2)) } } else { (assign41490_e55195 * (assign41490_e55194 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41490_e55194) as f64).is_finite() && ((assign41490_e55194) as f64).fract() == 0.0 { if assign41490_e55194 == 0.0 { 0.0 } else { (assign41490_e55194 * ((locals.var_dnm).powf(assign41490_e55194 - 1.0) * locals.var_dnm_dn4)) } } else { (assign41490_e55195 * (assign41490_e55194 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41490_e55194) as f64).is_finite() && ((assign41490_e55194) as f64).fract() == 0.0 { if assign41490_e55194 == 0.0 { 0.0 } else { (assign41490_e55194 * ((locals.var_dnm).powf(assign41490_e55194 - 1.0) * locals.var_dnm_dn5)) } } else { (assign41490_e55195 * (assign41490_e55194 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41490_e55194) as f64).is_finite() && ((assign41490_e55194) as f64).fract() == 0.0 { if assign41490_e55194 == 0.0 { 0.0 } else { (assign41490_e55194 * ((locals.var_dnm).powf(assign41490_e55194 - 1.0) * locals.var_dnm_dn6)) } } else { (assign41490_e55195 * (assign41490_e55194 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41490_e55194) as f64).is_finite() && ((assign41490_e55194) as f64).fract() == 0.0 { if assign41490_e55194 == 0.0 { 0.0 } else { (assign41490_e55194 * ((locals.var_dnm).powf(assign41490_e55194 - 1.0) * locals.var_dnm_dn7)) } } else { (assign41490_e55195 * (assign41490_e55194 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41490_e55194) as f64).is_finite() && ((assign41490_e55194) as f64).fract() == 0.0 { if assign41490_e55194 == 0.0 { 0.0 } else { (assign41490_e55194 * ((locals.var_dnm).powf(assign41490_e55194 - 1.0) * locals.var_dnm_dn8)) } } else { (assign41490_e55195 * (assign41490_e55194 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41490_e55194) as f64).is_finite() && ((assign41490_e55194) as f64).fract() == 0.0 { if assign41490_e55194 == 0.0 { 0.0 } else { (assign41490_e55194 * ((locals.var_dnm).powf(assign41490_e55194 - 1.0) * locals.var_dnm_dn9)) } } else { (assign41490_e55195 * (assign41490_e55194 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41490_e55194) as f64).is_finite() && ((assign41490_e55194) as f64).fract() == 0.0 { if assign41490_e55194 == 0.0 { 0.0 } else { (assign41490_e55194 * ((locals.var_dnm).powf(assign41490_e55194 - 1.0) * locals.var_dnm_dn10)) } } else { (assign41490_e55195 * (assign41490_e55194 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41490_e55194) as f64).is_finite() && ((assign41490_e55194) as f64).fract() == 0.0 { if assign41490_e55194 == 0.0 { 0.0 } else { (assign41490_e55194 * ((locals.var_dnm).powf(assign41490_e55194 - 1.0) * locals.var_dnm_dn11)) } } else { (assign41490_e55195 * (assign41490_e55194 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41490_e55194) as f64).is_finite() && ((assign41490_e55194) as f64).fract() == 0.0 { if assign41490_e55194 == 0.0 { 0.0 } else { (assign41490_e55194 * ((locals.var_dnm).powf(assign41490_e55194 - 1.0) * locals.var_dnm_dn14)) } } else { (assign41490_e55195 * (assign41490_e55194 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign41490_e55196, assign41490_e55196_d_n0, assign41490_e55196_d_n2, assign41490_e55196_d_n4, assign41490_e55196_d_n5, assign41490_e55196_d_n6, assign41490_e55196_d_n7, assign41490_e55196_d_n8, assign41490_e55196_d_n9, assign41490_e55196_d_n10, assign41490_e55196_d_n11, assign41490_e55196_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41490_e55198;
        locals.var_dnm_dn0 = assign41490_e55198_d_n0;
        locals.var_dnm_dn2 = assign41490_e55198_d_n2;
        locals.var_dnm_dn4 = assign41490_e55198_d_n4;
        locals.var_dnm_dn5 = assign41490_e55198_d_n5;
        locals.var_dnm_dn6 = assign41490_e55198_d_n6;
        locals.var_dnm_dn7 = assign41490_e55198_d_n7;
        locals.var_dnm_dn8 = assign41490_e55198_d_n8;
        locals.var_dnm_dn9 = assign41490_e55198_d_n9;
        locals.var_dnm_dn10 = assign41490_e55198_d_n10;
        locals.var_dnm_dn11 = assign41490_e55198_d_n11;
        locals.var_dnm_dn14 = assign41490_e55198_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41500_e55214, assign41500_e55214_d_n0, assign41500_e55214_d_n2, assign41500_e55214_d_n4, assign41500_e55214_d_n5, assign41500_e55214_d_n6, assign41500_e55214_d_n7, assign41500_e55214_d_n8, assign41500_e55214_d_n9, assign41500_e55214_d_n10, assign41500_e55214_d_n11, assign41500_e55214_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        let assign41500_e55212: f64 = (1.0 / locals.var_dnm);
        (assign41500_e55212, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41500_e55214;
        locals.var_dnm_dn0 = assign41500_e55214_d_n0;
        locals.var_dnm_dn2 = assign41500_e55214_d_n2;
        locals.var_dnm_dn4 = assign41500_e55214_d_n4;
        locals.var_dnm_dn5 = assign41500_e55214_d_n5;
        locals.var_dnm_dn6 = assign41500_e55214_d_n6;
        locals.var_dnm_dn7 = assign41500_e55214_d_n7;
        locals.var_dnm_dn8 = assign41500_e55214_d_n8;
        locals.var_dnm_dn9 = assign41500_e55214_d_n9;
        locals.var_dnm_dn10 = assign41500_e55214_d_n10;
        locals.var_dnm_dn11 = assign41500_e55214_d_n11;
        locals.var_dnm_dn14 = assign41500_e55214_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41510_e55232, assign41510_e55232_d_n0, assign41510_e55232_d_n2, assign41510_e55232_d_n4, assign41510_e55232_d_n5, assign41510_e55232_d_n6, assign41510_e55232_d_n7, assign41510_e55232_d_n8, assign41510_e55232_d_n9, assign41510_e55232_d_n10, assign41510_e55232_d_n11, assign41510_e55232_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        let assign41510_e55228: f64 = (locals.var_tmf1 * 0.1);
        let assign41510_e55230: f64 = (assign41510_e55228 * locals.var_dnm);
        (assign41510_e55230, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign41510_e55228 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign41510_e55228 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign41510_e55228 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign41510_e55228 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign41510_e55228 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign41510_e55228 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign41510_e55228 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign41510_e55228 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign41510_e55228 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign41510_e55228 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign41510_e55228 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign41510_e55232;
        locals.var_tmf0_dn0 = assign41510_e55232_d_n0;
        locals.var_tmf0_dn2 = assign41510_e55232_d_n2;
        locals.var_tmf0_dn4 = assign41510_e55232_d_n4;
        locals.var_tmf0_dn5 = assign41510_e55232_d_n5;
        locals.var_tmf0_dn6 = assign41510_e55232_d_n6;
        locals.var_tmf0_dn7 = assign41510_e55232_d_n7;
        locals.var_tmf0_dn8 = assign41510_e55232_d_n8;
        locals.var_tmf0_dn9 = assign41510_e55232_d_n9;
        locals.var_tmf0_dn10 = assign41510_e55232_d_n10;
        locals.var_tmf0_dn11 = assign41510_e55232_d_n11;
        locals.var_tmf0_dn14 = assign41510_e55232_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign41520_e55252, assign41520_e55252_d_n0, assign41520_e55252_d_n2, assign41520_e55252_d_n4, assign41520_e55252_d_n5, assign41520_e55252_d_n6, assign41520_e55252_d_n7, assign41520_e55252_d_n8, assign41520_e55252_d_n9, assign41520_e55252_d_n10, assign41520_e55252_d_n11, assign41520_e55252_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        let assign41520_e55246: f64 = (0.1 * locals.var_xmp);
        let assign41520_e55248: f64 = (assign41520_e55246 * locals.var_dnm);
        let assign41520_e55250: f64 = (assign41520_e55248 / locals.var_arg);
        (assign41520_e55250, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign41520_e55246 * locals.var_dnm_dn0)) * locals.var_arg) - (assign41520_e55248 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign41520_e55246 * locals.var_dnm_dn2)) * locals.var_arg) - (assign41520_e55248 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign41520_e55246 * locals.var_dnm_dn4)) * locals.var_arg) - (assign41520_e55248 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign41520_e55246 * locals.var_dnm_dn5)) * locals.var_arg) - (assign41520_e55248 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign41520_e55246 * locals.var_dnm_dn6)) * locals.var_arg) - (assign41520_e55248 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign41520_e55246 * locals.var_dnm_dn7)) * locals.var_arg) - (assign41520_e55248 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign41520_e55246 * locals.var_dnm_dn8)) * locals.var_arg) - (assign41520_e55248 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign41520_e55246 * locals.var_dnm_dn9)) * locals.var_arg) - (assign41520_e55248 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign41520_e55246 * locals.var_dnm_dn10)) * locals.var_arg) - (assign41520_e55248 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign41520_e55246 * locals.var_dnm_dn11)) * locals.var_arg) - (assign41520_e55248 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign41520_e55246 * locals.var_dnm_dn14)) * locals.var_arg) - (assign41520_e55248 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41520_e55252;
        locals.var_t0_dn0 = assign41520_e55252_d_n0;
        locals.var_t0_dn2 = assign41520_e55252_d_n2;
        locals.var_t0_dn4 = assign41520_e55252_d_n4;
        locals.var_t0_dn5 = assign41520_e55252_d_n5;
        locals.var_t0_dn6 = assign41520_e55252_d_n6;
        locals.var_t0_dn7 = assign41520_e55252_d_n7;
        locals.var_t0_dn8 = assign41520_e55252_d_n8;
        locals.var_t0_dn9 = assign41520_e55252_d_n9;
        locals.var_t0_dn10 = assign41520_e55252_d_n10;
        locals.var_t0_dn11 = assign41520_e55252_d_n11;
        locals.var_t0_dn14 = assign41520_e55252_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41530_e55270, assign41530_e55270_d_n0, assign41530_e55270_d_n2, assign41530_e55270_d_n4, assign41530_e55270_d_n5, assign41530_e55270_d_n6, assign41530_e55270_d_n7, assign41530_e55270_d_n8, assign41530_e55270_d_n9, assign41530_e55270_d_n10, assign41530_e55270_d_n11, assign41530_e55270_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        let assign41530_e55266: f64 = (-0.1);
        let assign41530_e55268: f64 = (assign41530_e55266 + locals.var_tmf0);
        (assign41530_e55268, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign41530_e55270;
        locals.var_ps0dep_dn0 = assign41530_e55270_d_n0;
        locals.var_ps0dep_dn2 = assign41530_e55270_d_n2;
        locals.var_ps0dep_dn4 = assign41530_e55270_d_n4;
        locals.var_ps0dep_dn5 = assign41530_e55270_d_n5;
        locals.var_ps0dep_dn6 = assign41530_e55270_d_n6;
        locals.var_ps0dep_dn7 = assign41530_e55270_d_n7;
        locals.var_ps0dep_dn8 = assign41530_e55270_d_n8;
        locals.var_ps0dep_dn9 = assign41530_e55270_d_n9;
        locals.var_ps0dep_dn10 = assign41530_e55270_d_n10;
        locals.var_ps0dep_dn11 = assign41530_e55270_d_n11;
        locals.var_ps0dep_dn14 = assign41530_e55270_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign41540_e55284, assign41540_e55284_d_n0, assign41540_e55284_d_n2, assign41540_e55284_d_n4, assign41540_e55284_d_n5, assign41540_e55284_d_n6, assign41540_e55284_d_n7, assign41540_e55284_d_n8, assign41540_e55284_d_n9, assign41540_e55284_d_n10, assign41540_e55284_d_n11, assign41540_e55284_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41540_e55284;
        locals.var_t0_dn0 = assign41540_e55284_d_n0;
        locals.var_t0_dn2 = assign41540_e55284_d_n2;
        locals.var_t0_dn4 = assign41540_e55284_d_n4;
        locals.var_t0_dn5 = assign41540_e55284_d_n5;
        locals.var_t0_dn6 = assign41540_e55284_d_n6;
        locals.var_t0_dn7 = assign41540_e55284_d_n7;
        locals.var_t0_dn8 = assign41540_e55284_d_n8;
        locals.var_t0_dn9 = assign41540_e55284_d_n9;
        locals.var_t0_dn10 = assign41540_e55284_d_n10;
        locals.var_t0_dn11 = assign41540_e55284_d_n11;
        locals.var_t0_dn14 = assign41540_e55284_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41550_e55299, assign41550_e55299_d_n0, assign41550_e55299_d_n2, assign41550_e55299_d_n4, assign41550_e55299_d_n5, assign41550_e55299_d_n6, assign41550_e55299_d_n7, assign41550_e55299_d_n8, assign41550_e55299_d_n9, assign41550_e55299_d_n10, assign41550_e55299_d_n11, assign41550_e55299_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign41550_e55299;
        locals.var_ps0dep_dn0 = assign41550_e55299_d_n0;
        locals.var_ps0dep_dn2 = assign41550_e55299_d_n2;
        locals.var_ps0dep_dn4 = assign41550_e55299_d_n4;
        locals.var_ps0dep_dn5 = assign41550_e55299_d_n5;
        locals.var_ps0dep_dn6 = assign41550_e55299_d_n6;
        locals.var_ps0dep_dn7 = assign41550_e55299_d_n7;
        locals.var_ps0dep_dn8 = assign41550_e55299_d_n8;
        locals.var_ps0dep_dn9 = assign41550_e55299_d_n9;
        locals.var_ps0dep_dn10 = assign41550_e55299_d_n10;
        locals.var_ps0dep_dn11 = assign41550_e55299_d_n11;
        locals.var_ps0dep_dn14 = assign41550_e55299_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign41560_e55314, assign41560_e55314_d_n0, assign41560_e55314_d_n2, assign41560_e55314_d_n4, assign41560_e55314_d_n5, assign41560_e55314_d_n6, assign41560_e55314_d_n7, assign41560_e55314_d_n8, assign41560_e55314_d_n9, assign41560_e55314_d_n10, assign41560_e55314_d_n11, assign41560_e55314_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1033 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41560_e55314;
        locals.var_t0_dn0 = assign41560_e55314_d_n0;
        locals.var_t0_dn2 = assign41560_e55314_d_n2;
        locals.var_t0_dn4 = assign41560_e55314_d_n4;
        locals.var_t0_dn5 = assign41560_e55314_d_n5;
        locals.var_t0_dn6 = assign41560_e55314_d_n6;
        locals.var_t0_dn7 = assign41560_e55314_d_n7;
        locals.var_t0_dn8 = assign41560_e55314_d_n8;
        locals.var_t0_dn9 = assign41560_e55314_d_n9;
        locals.var_t0_dn10 = assign41560_e55314_d_n10;
        locals.var_t0_dn11 = assign41560_e55314_d_n11;
        locals.var_t0_dn14 = assign41560_e55314_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41570_e55334, assign41570_e55334_d_n0, assign41570_e55334_d_n2, assign41570_e55334_d_n4, assign41570_e55334_d_n5, assign41570_e55334_d_n6, assign41570_e55334_d_n7, assign41570_e55334_d_n8, assign41570_e55334_d_n9, assign41570_e55334_d_n10, assign41570_e55334_d_n11, assign41570_e55334_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign41570_e55326: f64 = (locals.var_vgs - locals.var_vgp);
        let assign41570_e55329: f64 = (locals.var_uc_vfbc - p.p392);
        let assign41570_e55331: f64 = (assign41570_e55329 - locals.var_vfboffset);
        let assign41570_e55332: f64 = (assign41570_e55326 - assign41570_e55331);
        (assign41570_e55332, (-locals.var_vgp_dn0), (-locals.var_vgp_dn2), (-locals.var_vgp_dn4), (-locals.var_vgp_dn5), (locals.var_vgs_dn6 - locals.var_vgp_dn6), (locals.var_vgs_dn7 - locals.var_vgp_dn7), (locals.var_vgs_dn8 - locals.var_vgp_dn8), (-locals.var_vgp_dn9), (-locals.var_vgp_dn10), (-locals.var_vgp_dn11), (-locals.var_vgp_dn14),)
    } else {
        (locals.var_vfb_res, locals.var_vfb_res_dn0, locals.var_vfb_res_dn2, locals.var_vfb_res_dn4, locals.var_vfb_res_dn5, locals.var_vfb_res_dn6, locals.var_vfb_res_dn7, locals.var_vfb_res_dn8, locals.var_vfb_res_dn9, locals.var_vfb_res_dn10, locals.var_vfb_res_dn11, locals.var_vfb_res_dn14,)
    }
};
        locals.var_vfb_res = assign41570_e55334;
        locals.var_vfb_res_dn0 = assign41570_e55334_d_n0;
        locals.var_vfb_res_dn2 = assign41570_e55334_d_n2;
        locals.var_vfb_res_dn4 = assign41570_e55334_d_n4;
        locals.var_vfb_res_dn5 = assign41570_e55334_d_n5;
        locals.var_vfb_res_dn6 = assign41570_e55334_d_n6;
        locals.var_vfb_res_dn7 = assign41570_e55334_d_n7;
        locals.var_vfb_res_dn8 = assign41570_e55334_d_n8;
        locals.var_vfb_res_dn9 = assign41570_e55334_d_n9;
        locals.var_vfb_res_dn10 = assign41570_e55334_d_n10;
        locals.var_vfb_res_dn11 = assign41570_e55334_d_n11;
        locals.var_vfb_res_dn14 = assign41570_e55334_d_n14;
        locals.var_vfb_res_rv = 0.0;

        let (assign41580_e55348, assign41580_e55348_d_n0, assign41580_e55348_d_n2, assign41580_e55348_d_n4, assign41580_e55348_d_n5, assign41580_e55348_d_n6, assign41580_e55348_d_n7, assign41580_e55348_d_n8, assign41580_e55348_d_n9, assign41580_e55348_d_n10, assign41580_e55348_d_n11, assign41580_e55348_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign41580_e55346: f64 = (locals.var_vgs - locals.var_vfb_res);
        (assign41580_e55346, (-locals.var_vfb_res_dn0), (-locals.var_vfb_res_dn2), (-locals.var_vfb_res_dn4), (-locals.var_vfb_res_dn5), (locals.var_vgs_dn6 - locals.var_vfb_res_dn6), (locals.var_vgs_dn7 - locals.var_vfb_res_dn7), (locals.var_vgs_dn8 - locals.var_vfb_res_dn8), (-locals.var_vfb_res_dn9), (-locals.var_vfb_res_dn10), (-locals.var_vfb_res_dn11), (-locals.var_vfb_res_dn14),)
    } else {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn11, locals.var_vgp_res_dn14,)
    }
};
        locals.var_vgp_res = assign41580_e55348;
        locals.var_vgp_res_dn0 = assign41580_e55348_d_n0;
        locals.var_vgp_res_dn2 = assign41580_e55348_d_n2;
        locals.var_vgp_res_dn4 = assign41580_e55348_d_n4;
        locals.var_vgp_res_dn5 = assign41580_e55348_d_n5;
        locals.var_vgp_res_dn6 = assign41580_e55348_d_n6;
        locals.var_vgp_res_dn7 = assign41580_e55348_d_n7;
        locals.var_vgp_res_dn8 = assign41580_e55348_d_n8;
        locals.var_vgp_res_dn9 = assign41580_e55348_d_n9;
        locals.var_vgp_res_dn10 = assign41580_e55348_d_n10;
        locals.var_vgp_res_dn11 = assign41580_e55348_d_n11;
        locals.var_vgp_res_dn14 = assign41580_e55348_d_n14;
        locals.var_vgp_res_rv = 0.0;

        let assign41590_e55352: f64 = (-locals.var_vgpdep_dlt);
        let assign41590_e55357: f64 = if ((locals.var_vgp_res > assign41590_e55352) && (locals.var_vgpdep_dlt >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1039 = assign41590_e55357;
        locals.var_guard1039_rv = 0.0;

        let (assign41600_e55375, assign41600_e55375_d_n0, assign41600_e55375_d_n2, assign41600_e55375_d_n4, assign41600_e55375_d_n5, assign41600_e55375_d_n6, assign41600_e55375_d_n7, assign41600_e55375_d_n8, assign41600_e55375_d_n9, assign41600_e55375_d_n10, assign41600_e55375_d_n11, assign41600_e55375_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        let assign41600_e55371: f64 = locals.var_vgp_res;
        let assign41600_e55373: f64 = (assign41600_e55371 + locals.var_vgpdep_dlt);
        (assign41600_e55373, (locals.var_vgp_res_dn0 + locals.var_vgpdep_dlt_dn0), (locals.var_vgp_res_dn2 + locals.var_vgpdep_dlt_dn2), (locals.var_vgp_res_dn4 + locals.var_vgpdep_dlt_dn4), (locals.var_vgp_res_dn5 + locals.var_vgpdep_dlt_dn5), (locals.var_vgp_res_dn6 + locals.var_vgpdep_dlt_dn6), (locals.var_vgp_res_dn7 + locals.var_vgpdep_dlt_dn7), (locals.var_vgp_res_dn8 + locals.var_vgpdep_dlt_dn8), (locals.var_vgp_res_dn9 + locals.var_vgpdep_dlt_dn9), (locals.var_vgp_res_dn10 + locals.var_vgpdep_dlt_dn10), (locals.var_vgp_res_dn11 + locals.var_vgpdep_dlt_dn11), (locals.var_vgp_res_dn14 + locals.var_vgpdep_dlt_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign41600_e55375;
        locals.var_tmf1_dn0 = assign41600_e55375_d_n0;
        locals.var_tmf1_dn2 = assign41600_e55375_d_n2;
        locals.var_tmf1_dn4 = assign41600_e55375_d_n4;
        locals.var_tmf1_dn5 = assign41600_e55375_d_n5;
        locals.var_tmf1_dn6 = assign41600_e55375_d_n6;
        locals.var_tmf1_dn7 = assign41600_e55375_d_n7;
        locals.var_tmf1_dn8 = assign41600_e55375_d_n8;
        locals.var_tmf1_dn9 = assign41600_e55375_d_n9;
        locals.var_tmf1_dn10 = assign41600_e55375_d_n10;
        locals.var_tmf1_dn11 = assign41600_e55375_d_n11;
        locals.var_tmf1_dn14 = assign41600_e55375_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign41610_e55391, assign41610_e55391_d_n0, assign41610_e55391_d_n2, assign41610_e55391_d_n4, assign41610_e55391_d_n5, assign41610_e55391_d_n6, assign41610_e55391_d_n7, assign41610_e55391_d_n8, assign41610_e55391_d_n9, assign41610_e55391_d_n10, assign41610_e55391_d_n11, assign41610_e55391_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        let assign41610_e55389: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign41610_e55389, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign41610_e55391;
        locals.var_x2_dn0 = assign41610_e55391_d_n0;
        locals.var_x2_dn2 = assign41610_e55391_d_n2;
        locals.var_x2_dn4 = assign41610_e55391_d_n4;
        locals.var_x2_dn5 = assign41610_e55391_d_n5;
        locals.var_x2_dn6 = assign41610_e55391_d_n6;
        locals.var_x2_dn7 = assign41610_e55391_d_n7;
        locals.var_x2_dn8 = assign41610_e55391_d_n8;
        locals.var_x2_dn9 = assign41610_e55391_d_n9;
        locals.var_x2_dn10 = assign41610_e55391_d_n10;
        locals.var_x2_dn11 = assign41610_e55391_d_n11;
        locals.var_x2_dn14 = assign41610_e55391_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign41620_e55407, assign41620_e55407_d_n0, assign41620_e55407_d_n2, assign41620_e55407_d_n4, assign41620_e55407_d_n5, assign41620_e55407_d_n6, assign41620_e55407_d_n7, assign41620_e55407_d_n8, assign41620_e55407_d_n9, assign41620_e55407_d_n10, assign41620_e55407_d_n11, assign41620_e55407_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        let assign41620_e55405: f64 = (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt);
        (assign41620_e55405, ((locals.var_vgpdep_dlt_dn0 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn0)), ((locals.var_vgpdep_dlt_dn2 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn2)), ((locals.var_vgpdep_dlt_dn4 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn4)), ((locals.var_vgpdep_dlt_dn5 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn5)), ((locals.var_vgpdep_dlt_dn6 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn6)), ((locals.var_vgpdep_dlt_dn7 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn7)), ((locals.var_vgpdep_dlt_dn8 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn8)), ((locals.var_vgpdep_dlt_dn9 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn9)), ((locals.var_vgpdep_dlt_dn10 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn10)), ((locals.var_vgpdep_dlt_dn11 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn11)), ((locals.var_vgpdep_dlt_dn14 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign41620_e55407;
        locals.var_xmax2_dn0 = assign41620_e55407_d_n0;
        locals.var_xmax2_dn2 = assign41620_e55407_d_n2;
        locals.var_xmax2_dn4 = assign41620_e55407_d_n4;
        locals.var_xmax2_dn5 = assign41620_e55407_d_n5;
        locals.var_xmax2_dn6 = assign41620_e55407_d_n6;
        locals.var_xmax2_dn7 = assign41620_e55407_d_n7;
        locals.var_xmax2_dn8 = assign41620_e55407_d_n8;
        locals.var_xmax2_dn9 = assign41620_e55407_d_n9;
        locals.var_xmax2_dn10 = assign41620_e55407_d_n10;
        locals.var_xmax2_dn11 = assign41620_e55407_d_n11;
        locals.var_xmax2_dn14 = assign41620_e55407_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign41630_e55421, assign41630_e55421_d_n0, assign41630_e55421_d_n2, assign41630_e55421_d_n4, assign41630_e55421_d_n5, assign41630_e55421_d_n6, assign41630_e55421_d_n7, assign41630_e55421_d_n8, assign41630_e55421_d_n9, assign41630_e55421_d_n10, assign41630_e55421_d_n11, assign41630_e55421_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign41630_e55421;
        locals.var_xp_dn0 = assign41630_e55421_d_n0;
        locals.var_xp_dn2 = assign41630_e55421_d_n2;
        locals.var_xp_dn4 = assign41630_e55421_d_n4;
        locals.var_xp_dn5 = assign41630_e55421_d_n5;
        locals.var_xp_dn6 = assign41630_e55421_d_n6;
        locals.var_xp_dn7 = assign41630_e55421_d_n7;
        locals.var_xp_dn8 = assign41630_e55421_d_n8;
        locals.var_xp_dn9 = assign41630_e55421_d_n9;
        locals.var_xp_dn10 = assign41630_e55421_d_n10;
        locals.var_xp_dn11 = assign41630_e55421_d_n11;
        locals.var_xp_dn14 = assign41630_e55421_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign41640_e55435, assign41640_e55435_d_n0, assign41640_e55435_d_n2, assign41640_e55435_d_n4, assign41640_e55435_d_n5, assign41640_e55435_d_n6, assign41640_e55435_d_n7, assign41640_e55435_d_n8, assign41640_e55435_d_n9, assign41640_e55435_d_n10, assign41640_e55435_d_n11, assign41640_e55435_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign41640_e55435;
        locals.var_xmp_dn0 = assign41640_e55435_d_n0;
        locals.var_xmp_dn2 = assign41640_e55435_d_n2;
        locals.var_xmp_dn4 = assign41640_e55435_d_n4;
        locals.var_xmp_dn5 = assign41640_e55435_d_n5;
        locals.var_xmp_dn6 = assign41640_e55435_d_n6;
        locals.var_xmp_dn7 = assign41640_e55435_d_n7;
        locals.var_xmp_dn8 = assign41640_e55435_d_n8;
        locals.var_xmp_dn9 = assign41640_e55435_d_n9;
        locals.var_xmp_dn10 = assign41640_e55435_d_n10;
        locals.var_xmp_dn11 = assign41640_e55435_d_n11;
        locals.var_xmp_dn14 = assign41640_e55435_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign41650_e55449,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41650_e55449;
        locals.var_m0_rv = 0.0;

        let (assign41660_e55463,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41660_e55463;
        locals.var_mm_rv = 0.0;

        let (assign41670_e55477, assign41670_e55477_d_n0, assign41670_e55477_d_n2, assign41670_e55477_d_n4, assign41670_e55477_d_n5, assign41670_e55477_d_n6, assign41670_e55477_d_n7, assign41670_e55477_d_n8, assign41670_e55477_d_n9, assign41670_e55477_d_n10, assign41670_e55477_d_n11, assign41670_e55477_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign41670_e55477;
        locals.var_arg_dn0 = assign41670_e55477_d_n0;
        locals.var_arg_dn2 = assign41670_e55477_d_n2;
        locals.var_arg_dn4 = assign41670_e55477_d_n4;
        locals.var_arg_dn5 = assign41670_e55477_d_n5;
        locals.var_arg_dn6 = assign41670_e55477_d_n6;
        locals.var_arg_dn7 = assign41670_e55477_d_n7;
        locals.var_arg_dn8 = assign41670_e55477_d_n8;
        locals.var_arg_dn9 = assign41670_e55477_d_n9;
        locals.var_arg_dn10 = assign41670_e55477_d_n10;
        locals.var_arg_dn11 = assign41670_e55477_d_n11;
        locals.var_arg_dn14 = assign41670_e55477_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign41680_e55491, assign41680_e55491_d_n0, assign41680_e55491_d_n2, assign41680_e55491_d_n4, assign41680_e55491_d_n5, assign41680_e55491_d_n6, assign41680_e55491_d_n7, assign41680_e55491_d_n8, assign41680_e55491_d_n9, assign41680_e55491_d_n10, assign41680_e55491_d_n11, assign41680_e55491_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41680_e55491;
        locals.var_dnm_dn0 = assign41680_e55491_d_n0;
        locals.var_dnm_dn2 = assign41680_e55491_d_n2;
        locals.var_dnm_dn4 = assign41680_e55491_d_n4;
        locals.var_dnm_dn5 = assign41680_e55491_d_n5;
        locals.var_dnm_dn6 = assign41680_e55491_d_n6;
        locals.var_dnm_dn7 = assign41680_e55491_d_n7;
        locals.var_dnm_dn8 = assign41680_e55491_d_n8;
        locals.var_dnm_dn9 = assign41680_e55491_d_n9;
        locals.var_dnm_dn10 = assign41680_e55491_d_n10;
        locals.var_dnm_dn11 = assign41680_e55491_d_n11;
        locals.var_dnm_dn14 = assign41680_e55491_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41690_e55505,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41690_e55505;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_143(
        locals: &mut StampLocals,
    ) {
        let mut assign41700_loop_guard: usize = 0;
        while {
            let assign41700_cond_e55520: f64 = if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw)) { 1.0 } else { 0.0 };
            assign41700_cond_e55520 != 0.0
        } {
            assign41700_loop_guard += 1;
            assert!(assign41700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41700_body0_e55536, assign41700_body0_e55536_d_n0, assign41700_body0_e55536_d_n2, assign41700_body0_e55536_d_n4, assign41700_body0_e55536_d_n5, assign41700_body0_e55536_d_n6, assign41700_body0_e55536_d_n7, assign41700_body0_e55536_d_n8, assign41700_body0_e55536_d_n9, assign41700_body0_e55536_d_n10, assign41700_body0_e55536_d_n11, assign41700_body0_e55536_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        let assign41700_body0_e55534: f64 = (locals.var_xp * locals.var_x2);
        (assign41700_body0_e55534, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign41700_body0_e55536;
            locals.var_xp_dn0 = assign41700_body0_e55536_d_n0;
            locals.var_xp_dn2 = assign41700_body0_e55536_d_n2;
            locals.var_xp_dn4 = assign41700_body0_e55536_d_n4;
            locals.var_xp_dn5 = assign41700_body0_e55536_d_n5;
            locals.var_xp_dn6 = assign41700_body0_e55536_d_n6;
            locals.var_xp_dn7 = assign41700_body0_e55536_d_n7;
            locals.var_xp_dn8 = assign41700_body0_e55536_d_n8;
            locals.var_xp_dn9 = assign41700_body0_e55536_d_n9;
            locals.var_xp_dn10 = assign41700_body0_e55536_d_n10;
            locals.var_xp_dn11 = assign41700_body0_e55536_d_n11;
            locals.var_xp_dn14 = assign41700_body0_e55536_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign41700_body1_e55552, assign41700_body1_e55552_d_n0, assign41700_body1_e55552_d_n2, assign41700_body1_e55552_d_n4, assign41700_body1_e55552_d_n5, assign41700_body1_e55552_d_n6, assign41700_body1_e55552_d_n7, assign41700_body1_e55552_d_n8, assign41700_body1_e55552_d_n9, assign41700_body1_e55552_d_n10, assign41700_body1_e55552_d_n11, assign41700_body1_e55552_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        let assign41700_body1_e55550: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign41700_body1_e55550, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign41700_body1_e55552;
            locals.var_xmp_dn0 = assign41700_body1_e55552_d_n0;
            locals.var_xmp_dn2 = assign41700_body1_e55552_d_n2;
            locals.var_xmp_dn4 = assign41700_body1_e55552_d_n4;
            locals.var_xmp_dn5 = assign41700_body1_e55552_d_n5;
            locals.var_xmp_dn6 = assign41700_body1_e55552_d_n6;
            locals.var_xmp_dn7 = assign41700_body1_e55552_d_n7;
            locals.var_xmp_dn8 = assign41700_body1_e55552_d_n8;
            locals.var_xmp_dn9 = assign41700_body1_e55552_d_n9;
            locals.var_xmp_dn10 = assign41700_body1_e55552_d_n10;
            locals.var_xmp_dn11 = assign41700_body1_e55552_d_n11;
            locals.var_xmp_dn14 = assign41700_body1_e55552_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign41700_body2_e55568,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        let assign41700_body2_e55566: f64 = (locals.var_m0 + 1.0);
        (assign41700_body2_e55566,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41700_body2_e55568;
            locals.var_m0_rv = 0.0;
        }

        let (assign41710_e55584, assign41710_e55584_d_n0, assign41710_e55584_d_n2, assign41710_e55584_d_n4, assign41710_e55584_d_n5, assign41710_e55584_d_n6, assign41710_e55584_d_n7, assign41710_e55584_d_n8, assign41710_e55584_d_n9, assign41710_e55584_d_n10, assign41710_e55584_d_n11, assign41710_e55584_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        let assign41710_e55582: f64 = (locals.var_xp + locals.var_xmp);
        (assign41710_e55582, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign41710_e55584;
        locals.var_arg_dn0 = assign41710_e55584_d_n0;
        locals.var_arg_dn2 = assign41710_e55584_d_n2;
        locals.var_arg_dn4 = assign41710_e55584_d_n4;
        locals.var_arg_dn5 = assign41710_e55584_d_n5;
        locals.var_arg_dn6 = assign41710_e55584_d_n6;
        locals.var_arg_dn7 = assign41710_e55584_d_n7;
        locals.var_arg_dn8 = assign41710_e55584_d_n8;
        locals.var_arg_dn9 = assign41710_e55584_d_n9;
        locals.var_arg_dn10 = assign41710_e55584_d_n10;
        locals.var_arg_dn11 = assign41710_e55584_d_n11;
        locals.var_arg_dn14 = assign41710_e55584_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign41720_e55598, assign41720_e55598_d_n0, assign41720_e55598_d_n2, assign41720_e55598_d_n4, assign41720_e55598_d_n5, assign41720_e55598_d_n6, assign41720_e55598_d_n7, assign41720_e55598_d_n8, assign41720_e55598_d_n9, assign41720_e55598_d_n10, assign41720_e55598_d_n11, assign41720_e55598_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41720_e55598;
        locals.var_dnm_dn0 = assign41720_e55598_d_n0;
        locals.var_dnm_dn2 = assign41720_e55598_d_n2;
        locals.var_dnm_dn4 = assign41720_e55598_d_n4;
        locals.var_dnm_dn5 = assign41720_e55598_d_n5;
        locals.var_dnm_dn6 = assign41720_e55598_d_n6;
        locals.var_dnm_dn7 = assign41720_e55598_d_n7;
        locals.var_dnm_dn8 = assign41720_e55598_d_n8;
        locals.var_dnm_dn9 = assign41720_e55598_d_n9;
        locals.var_dnm_dn10 = assign41720_e55598_d_n10;
        locals.var_dnm_dn11 = assign41720_e55598_d_n11;
        locals.var_dnm_dn14 = assign41720_e55598_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign41730_e55613: f64 = if ((((locals.var_vgpdep_pw == 1.0) || (locals.var_vgpdep_pw == 2.0)) || (locals.var_vgpdep_pw == 4.0)) || (locals.var_vgpdep_pw == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1040 = assign41730_e55613;
        locals.var_guard1040_rv = 0.0;

        let assign41740_e55616: f64 = if locals.var_vgpdep_pw == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1041 = assign41740_e55616;
        locals.var_guard1041_rv = 0.0;

        let (assign41750_e55634,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) && (locals.var_guard1040 != 0.0)) && (locals.var_guard1041 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41750_e55634;
        locals.var_mm_rv = 0.0;

        let assign41760_e55637: f64 = if locals.var_vgpdep_pw == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1042 = assign41760_e55637;
        locals.var_guard1042_rv = 0.0;

        let (assign41770_e55658,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) && (locals.var_guard1040 != 0.0)) && (locals.var_guard1041 == 0.0)) && (locals.var_guard1042 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41770_e55658;
        locals.var_mm_rv = 0.0;

        let assign41780_e55661: f64 = if locals.var_vgpdep_pw == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1043 = assign41780_e55661;
        locals.var_guard1043_rv = 0.0;

        let (assign41790_e55685,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) && (locals.var_guard1040 != 0.0)) && (locals.var_guard1041 == 0.0)) && (locals.var_guard1042 == 0.0)) && (locals.var_guard1043 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41790_e55685;
        locals.var_mm_rv = 0.0;

        let assign41800_e55688: f64 = if locals.var_vgpdep_pw == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1044 = assign41800_e55688;
        locals.var_guard1044_rv = 0.0;

        let (assign41810_e55715,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) && (locals.var_guard1040 != 0.0)) && (locals.var_guard1041 == 0.0)) && (locals.var_guard1042 == 0.0)) && (locals.var_guard1043 == 0.0)) && (locals.var_guard1044 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41810_e55715;
        locals.var_mm_rv = 0.0;

        let (assign41820_e55731,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) && (locals.var_guard1040 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41820_e55731;
        locals.var_m0_rv = 0.0;

        let mut assign41830_loop_guard: usize = 0;
        while {
            let assign41830_cond_e55748: f64 = if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) && (locals.var_guard1040 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign41830_cond_e55748 != 0.0
        } {
            assign41830_loop_guard += 1;
            assert!(assign41830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41830_body0_e55765, assign41830_body0_e55765_d_n0, assign41830_body0_e55765_d_n2, assign41830_body0_e55765_d_n4, assign41830_body0_e55765_d_n5, assign41830_body0_e55765_d_n6, assign41830_body0_e55765_d_n7, assign41830_body0_e55765_d_n8, assign41830_body0_e55765_d_n9, assign41830_body0_e55765_d_n10, assign41830_body0_e55765_d_n11, assign41830_body0_e55765_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) && (locals.var_guard1040 != 0.0)) {
        let assign41830_body0_e55763: f64 = (locals.var_dnm).sqrt();
        (assign41830_body0_e55763, (locals.var_dnm_dn0 / (2.0 * assign41830_body0_e55763)), (locals.var_dnm_dn2 / (2.0 * assign41830_body0_e55763)), (locals.var_dnm_dn4 / (2.0 * assign41830_body0_e55763)), (locals.var_dnm_dn5 / (2.0 * assign41830_body0_e55763)), (locals.var_dnm_dn6 / (2.0 * assign41830_body0_e55763)), (locals.var_dnm_dn7 / (2.0 * assign41830_body0_e55763)), (locals.var_dnm_dn8 / (2.0 * assign41830_body0_e55763)), (locals.var_dnm_dn9 / (2.0 * assign41830_body0_e55763)), (locals.var_dnm_dn10 / (2.0 * assign41830_body0_e55763)), (locals.var_dnm_dn11 / (2.0 * assign41830_body0_e55763)), (locals.var_dnm_dn14 / (2.0 * assign41830_body0_e55763)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign41830_body0_e55765;
            locals.var_dnm_dn0 = assign41830_body0_e55765_d_n0;
            locals.var_dnm_dn2 = assign41830_body0_e55765_d_n2;
            locals.var_dnm_dn4 = assign41830_body0_e55765_d_n4;
            locals.var_dnm_dn5 = assign41830_body0_e55765_d_n5;
            locals.var_dnm_dn6 = assign41830_body0_e55765_d_n6;
            locals.var_dnm_dn7 = assign41830_body0_e55765_d_n7;
            locals.var_dnm_dn8 = assign41830_body0_e55765_d_n8;
            locals.var_dnm_dn9 = assign41830_body0_e55765_d_n9;
            locals.var_dnm_dn10 = assign41830_body0_e55765_d_n10;
            locals.var_dnm_dn11 = assign41830_body0_e55765_d_n11;
            locals.var_dnm_dn14 = assign41830_body0_e55765_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign41830_body1_e55783,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) && (locals.var_guard1040 != 0.0)) {
        let assign41830_body1_e55781: f64 = (locals.var_m0 + 1.0);
        (assign41830_body1_e55781,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41830_body1_e55783;
            locals.var_m0_rv = 0.0;
        }

        let (assign41840_e55811, assign41840_e55811_d_n0, assign41840_e55811_d_n2, assign41840_e55811_d_n4, assign41840_e55811_d_n5, assign41840_e55811_d_n6, assign41840_e55811_d_n7, assign41840_e55811_d_n8, assign41840_e55811_d_n9, assign41840_e55811_d_n10, assign41840_e55811_d_n11, assign41840_e55811_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) && (locals.var_guard1040 == 0.0)) {
        let (assign41840_e55809, assign41840_e55809_d_n0, assign41840_e55809_d_n2, assign41840_e55809_d_n4, assign41840_e55809_d_n5, assign41840_e55809_d_n6, assign41840_e55809_d_n7, assign41840_e55809_d_n8, assign41840_e55809_d_n9, assign41840_e55809_d_n10, assign41840_e55809_d_n11, assign41840_e55809_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign41840_e55806: f64 = (2.0 * locals.var_vgpdep_pw);
                let assign41840_e55807: f64 = (1.0 / assign41840_e55806);
                let assign41840_e55808: f64 = (locals.var_dnm).powf(assign41840_e55807);
                (assign41840_e55808, if 0.0 == 0.0 && ((assign41840_e55807) as f64).is_finite() && ((assign41840_e55807) as f64).fract() == 0.0 { if assign41840_e55807 == 0.0 { 0.0 } else { (assign41840_e55807 * ((locals.var_dnm).powf(assign41840_e55807 - 1.0) * locals.var_dnm_dn0)) } } else { (assign41840_e55808 * (assign41840_e55807 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41840_e55807) as f64).is_finite() && ((assign41840_e55807) as f64).fract() == 0.0 { if assign41840_e55807 == 0.0 { 0.0 } else { (assign41840_e55807 * ((locals.var_dnm).powf(assign41840_e55807 - 1.0) * locals.var_dnm_dn2)) } } else { (assign41840_e55808 * (assign41840_e55807 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41840_e55807) as f64).is_finite() && ((assign41840_e55807) as f64).fract() == 0.0 { if assign41840_e55807 == 0.0 { 0.0 } else { (assign41840_e55807 * ((locals.var_dnm).powf(assign41840_e55807 - 1.0) * locals.var_dnm_dn4)) } } else { (assign41840_e55808 * (assign41840_e55807 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41840_e55807) as f64).is_finite() && ((assign41840_e55807) as f64).fract() == 0.0 { if assign41840_e55807 == 0.0 { 0.0 } else { (assign41840_e55807 * ((locals.var_dnm).powf(assign41840_e55807 - 1.0) * locals.var_dnm_dn5)) } } else { (assign41840_e55808 * (assign41840_e55807 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41840_e55807) as f64).is_finite() && ((assign41840_e55807) as f64).fract() == 0.0 { if assign41840_e55807 == 0.0 { 0.0 } else { (assign41840_e55807 * ((locals.var_dnm).powf(assign41840_e55807 - 1.0) * locals.var_dnm_dn6)) } } else { (assign41840_e55808 * (assign41840_e55807 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41840_e55807) as f64).is_finite() && ((assign41840_e55807) as f64).fract() == 0.0 { if assign41840_e55807 == 0.0 { 0.0 } else { (assign41840_e55807 * ((locals.var_dnm).powf(assign41840_e55807 - 1.0) * locals.var_dnm_dn7)) } } else { (assign41840_e55808 * (assign41840_e55807 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41840_e55807) as f64).is_finite() && ((assign41840_e55807) as f64).fract() == 0.0 { if assign41840_e55807 == 0.0 { 0.0 } else { (assign41840_e55807 * ((locals.var_dnm).powf(assign41840_e55807 - 1.0) * locals.var_dnm_dn8)) } } else { (assign41840_e55808 * (assign41840_e55807 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41840_e55807) as f64).is_finite() && ((assign41840_e55807) as f64).fract() == 0.0 { if assign41840_e55807 == 0.0 { 0.0 } else { (assign41840_e55807 * ((locals.var_dnm).powf(assign41840_e55807 - 1.0) * locals.var_dnm_dn9)) } } else { (assign41840_e55808 * (assign41840_e55807 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41840_e55807) as f64).is_finite() && ((assign41840_e55807) as f64).fract() == 0.0 { if assign41840_e55807 == 0.0 { 0.0 } else { (assign41840_e55807 * ((locals.var_dnm).powf(assign41840_e55807 - 1.0) * locals.var_dnm_dn10)) } } else { (assign41840_e55808 * (assign41840_e55807 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41840_e55807) as f64).is_finite() && ((assign41840_e55807) as f64).fract() == 0.0 { if assign41840_e55807 == 0.0 { 0.0 } else { (assign41840_e55807 * ((locals.var_dnm).powf(assign41840_e55807 - 1.0) * locals.var_dnm_dn11)) } } else { (assign41840_e55808 * (assign41840_e55807 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41840_e55807) as f64).is_finite() && ((assign41840_e55807) as f64).fract() == 0.0 { if assign41840_e55807 == 0.0 { 0.0 } else { (assign41840_e55807 * ((locals.var_dnm).powf(assign41840_e55807 - 1.0) * locals.var_dnm_dn14)) } } else { (assign41840_e55808 * (assign41840_e55807 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign41840_e55809, assign41840_e55809_d_n0, assign41840_e55809_d_n2, assign41840_e55809_d_n4, assign41840_e55809_d_n5, assign41840_e55809_d_n6, assign41840_e55809_d_n7, assign41840_e55809_d_n8, assign41840_e55809_d_n9, assign41840_e55809_d_n10, assign41840_e55809_d_n11, assign41840_e55809_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41840_e55811;
        locals.var_dnm_dn0 = assign41840_e55811_d_n0;
        locals.var_dnm_dn2 = assign41840_e55811_d_n2;
        locals.var_dnm_dn4 = assign41840_e55811_d_n4;
        locals.var_dnm_dn5 = assign41840_e55811_d_n5;
        locals.var_dnm_dn6 = assign41840_e55811_d_n6;
        locals.var_dnm_dn7 = assign41840_e55811_d_n7;
        locals.var_dnm_dn8 = assign41840_e55811_d_n8;
        locals.var_dnm_dn9 = assign41840_e55811_d_n9;
        locals.var_dnm_dn10 = assign41840_e55811_d_n10;
        locals.var_dnm_dn11 = assign41840_e55811_d_n11;
        locals.var_dnm_dn14 = assign41840_e55811_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41850_e55827, assign41850_e55827_d_n0, assign41850_e55827_d_n2, assign41850_e55827_d_n4, assign41850_e55827_d_n5, assign41850_e55827_d_n6, assign41850_e55827_d_n7, assign41850_e55827_d_n8, assign41850_e55827_d_n9, assign41850_e55827_d_n10, assign41850_e55827_d_n11, assign41850_e55827_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        let assign41850_e55825: f64 = (1.0 / locals.var_dnm);
        (assign41850_e55825, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41850_e55827;
        locals.var_dnm_dn0 = assign41850_e55827_d_n0;
        locals.var_dnm_dn2 = assign41850_e55827_d_n2;
        locals.var_dnm_dn4 = assign41850_e55827_d_n4;
        locals.var_dnm_dn5 = assign41850_e55827_d_n5;
        locals.var_dnm_dn6 = assign41850_e55827_d_n6;
        locals.var_dnm_dn7 = assign41850_e55827_d_n7;
        locals.var_dnm_dn8 = assign41850_e55827_d_n8;
        locals.var_dnm_dn9 = assign41850_e55827_d_n9;
        locals.var_dnm_dn10 = assign41850_e55827_d_n10;
        locals.var_dnm_dn11 = assign41850_e55827_d_n11;
        locals.var_dnm_dn14 = assign41850_e55827_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41860_e55845, assign41860_e55845_d_n0, assign41860_e55845_d_n2, assign41860_e55845_d_n4, assign41860_e55845_d_n5, assign41860_e55845_d_n6, assign41860_e55845_d_n7, assign41860_e55845_d_n8, assign41860_e55845_d_n9, assign41860_e55845_d_n10, assign41860_e55845_d_n11, assign41860_e55845_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        let assign41860_e55841: f64 = (locals.var_tmf1 * locals.var_vgpdep_dlt);
        let assign41860_e55843: f64 = (assign41860_e55841 * locals.var_dnm);
        (assign41860_e55843, ((((locals.var_tmf1_dn0 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn0)) * locals.var_dnm) + (assign41860_e55841 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn2)) * locals.var_dnm) + (assign41860_e55841 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn4)) * locals.var_dnm) + (assign41860_e55841 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn5)) * locals.var_dnm) + (assign41860_e55841 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn6)) * locals.var_dnm) + (assign41860_e55841 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn7)) * locals.var_dnm) + (assign41860_e55841 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn8)) * locals.var_dnm) + (assign41860_e55841 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn9)) * locals.var_dnm) + (assign41860_e55841 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn10)) * locals.var_dnm) + (assign41860_e55841 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn11)) * locals.var_dnm) + (assign41860_e55841 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn14)) * locals.var_dnm) + (assign41860_e55841 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign41860_e55845;
        locals.var_tmf0_dn0 = assign41860_e55845_d_n0;
        locals.var_tmf0_dn2 = assign41860_e55845_d_n2;
        locals.var_tmf0_dn4 = assign41860_e55845_d_n4;
        locals.var_tmf0_dn5 = assign41860_e55845_d_n5;
        locals.var_tmf0_dn6 = assign41860_e55845_d_n6;
        locals.var_tmf0_dn7 = assign41860_e55845_d_n7;
        locals.var_tmf0_dn8 = assign41860_e55845_d_n8;
        locals.var_tmf0_dn9 = assign41860_e55845_d_n9;
        locals.var_tmf0_dn10 = assign41860_e55845_d_n10;
        locals.var_tmf0_dn11 = assign41860_e55845_d_n11;
        locals.var_tmf0_dn14 = assign41860_e55845_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign41870_e55865, assign41870_e55865_d_n0, assign41870_e55865_d_n2, assign41870_e55865_d_n4, assign41870_e55865_d_n5, assign41870_e55865_d_n6, assign41870_e55865_d_n7, assign41870_e55865_d_n8, assign41870_e55865_d_n9, assign41870_e55865_d_n10, assign41870_e55865_d_n11, assign41870_e55865_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        let assign41870_e55859: f64 = (locals.var_vgpdep_dlt * locals.var_xmp);
        let assign41870_e55861: f64 = (assign41870_e55859 * locals.var_dnm);
        let assign41870_e55863: f64 = (assign41870_e55861 / locals.var_arg);
        (assign41870_e55863, (((((((locals.var_vgpdep_dlt_dn0 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn0)) * locals.var_dnm) + (assign41870_e55859 * locals.var_dnm_dn0)) * locals.var_arg) - (assign41870_e55861 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn2 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn2)) * locals.var_dnm) + (assign41870_e55859 * locals.var_dnm_dn2)) * locals.var_arg) - (assign41870_e55861 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn4 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn4)) * locals.var_dnm) + (assign41870_e55859 * locals.var_dnm_dn4)) * locals.var_arg) - (assign41870_e55861 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn5 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn5)) * locals.var_dnm) + (assign41870_e55859 * locals.var_dnm_dn5)) * locals.var_arg) - (assign41870_e55861 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn6 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn6)) * locals.var_dnm) + (assign41870_e55859 * locals.var_dnm_dn6)) * locals.var_arg) - (assign41870_e55861 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn7 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn7)) * locals.var_dnm) + (assign41870_e55859 * locals.var_dnm_dn7)) * locals.var_arg) - (assign41870_e55861 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn8 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn8)) * locals.var_dnm) + (assign41870_e55859 * locals.var_dnm_dn8)) * locals.var_arg) - (assign41870_e55861 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn9 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn9)) * locals.var_dnm) + (assign41870_e55859 * locals.var_dnm_dn9)) * locals.var_arg) - (assign41870_e55861 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn10 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn10)) * locals.var_dnm) + (assign41870_e55859 * locals.var_dnm_dn10)) * locals.var_arg) - (assign41870_e55861 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn11 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn11)) * locals.var_dnm) + (assign41870_e55859 * locals.var_dnm_dn11)) * locals.var_arg) - (assign41870_e55861 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn14 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn14)) * locals.var_dnm) + (assign41870_e55859 * locals.var_dnm_dn14)) * locals.var_arg) - (assign41870_e55861 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41870_e55865;
        locals.var_t0_dn0 = assign41870_e55865_d_n0;
        locals.var_t0_dn2 = assign41870_e55865_d_n2;
        locals.var_t0_dn4 = assign41870_e55865_d_n4;
        locals.var_t0_dn5 = assign41870_e55865_d_n5;
        locals.var_t0_dn6 = assign41870_e55865_d_n6;
        locals.var_t0_dn7 = assign41870_e55865_d_n7;
        locals.var_t0_dn8 = assign41870_e55865_d_n8;
        locals.var_t0_dn9 = assign41870_e55865_d_n9;
        locals.var_t0_dn10 = assign41870_e55865_d_n10;
        locals.var_t0_dn11 = assign41870_e55865_d_n11;
        locals.var_t0_dn14 = assign41870_e55865_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41880_e55883, assign41880_e55883_d_n0, assign41880_e55883_d_n2, assign41880_e55883_d_n4, assign41880_e55883_d_n5, assign41880_e55883_d_n6, assign41880_e55883_d_n7, assign41880_e55883_d_n8, assign41880_e55883_d_n9, assign41880_e55883_d_n10, assign41880_e55883_d_n11, assign41880_e55883_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        let assign41880_e55879: f64 = (-locals.var_vgpdep_dlt);
        let assign41880_e55881: f64 = (assign41880_e55879 + locals.var_tmf0);
        (assign41880_e55881, ((-locals.var_vgpdep_dlt_dn0) + locals.var_tmf0_dn0), ((-locals.var_vgpdep_dlt_dn2) + locals.var_tmf0_dn2), ((-locals.var_vgpdep_dlt_dn4) + locals.var_tmf0_dn4), ((-locals.var_vgpdep_dlt_dn5) + locals.var_tmf0_dn5), ((-locals.var_vgpdep_dlt_dn6) + locals.var_tmf0_dn6), ((-locals.var_vgpdep_dlt_dn7) + locals.var_tmf0_dn7), ((-locals.var_vgpdep_dlt_dn8) + locals.var_tmf0_dn8), ((-locals.var_vgpdep_dlt_dn9) + locals.var_tmf0_dn9), ((-locals.var_vgpdep_dlt_dn10) + locals.var_tmf0_dn10), ((-locals.var_vgpdep_dlt_dn11) + locals.var_tmf0_dn11), ((-locals.var_vgpdep_dlt_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn11, locals.var_vgp_res_dn14,)
    }
};
        locals.var_vgp_res = assign41880_e55883;
        locals.var_vgp_res_dn0 = assign41880_e55883_d_n0;
        locals.var_vgp_res_dn2 = assign41880_e55883_d_n2;
        locals.var_vgp_res_dn4 = assign41880_e55883_d_n4;
        locals.var_vgp_res_dn5 = assign41880_e55883_d_n5;
        locals.var_vgp_res_dn6 = assign41880_e55883_d_n6;
        locals.var_vgp_res_dn7 = assign41880_e55883_d_n7;
        locals.var_vgp_res_dn8 = assign41880_e55883_d_n8;
        locals.var_vgp_res_dn9 = assign41880_e55883_d_n9;
        locals.var_vgp_res_dn10 = assign41880_e55883_d_n10;
        locals.var_vgp_res_dn11 = assign41880_e55883_d_n11;
        locals.var_vgp_res_dn14 = assign41880_e55883_d_n14;
        locals.var_vgp_res_rv = 0.0;

        let (assign41890_e55897, assign41890_e55897_d_n0, assign41890_e55897_d_n2, assign41890_e55897_d_n4, assign41890_e55897_d_n5, assign41890_e55897_d_n6, assign41890_e55897_d_n7, assign41890_e55897_d_n8, assign41890_e55897_d_n9, assign41890_e55897_d_n10, assign41890_e55897_d_n11, assign41890_e55897_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41890_e55897;
        locals.var_t0_dn0 = assign41890_e55897_d_n0;
        locals.var_t0_dn2 = assign41890_e55897_d_n2;
        locals.var_t0_dn4 = assign41890_e55897_d_n4;
        locals.var_t0_dn5 = assign41890_e55897_d_n5;
        locals.var_t0_dn6 = assign41890_e55897_d_n6;
        locals.var_t0_dn7 = assign41890_e55897_d_n7;
        locals.var_t0_dn8 = assign41890_e55897_d_n8;
        locals.var_t0_dn9 = assign41890_e55897_d_n9;
        locals.var_t0_dn10 = assign41890_e55897_d_n10;
        locals.var_t0_dn11 = assign41890_e55897_d_n11;
        locals.var_t0_dn14 = assign41890_e55897_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41900_e55912, assign41900_e55912_d_n0, assign41900_e55912_d_n2, assign41900_e55912_d_n4, assign41900_e55912_d_n5, assign41900_e55912_d_n6, assign41900_e55912_d_n7, assign41900_e55912_d_n8, assign41900_e55912_d_n9, assign41900_e55912_d_n10, assign41900_e55912_d_n11, assign41900_e55912_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 == 0.0)) {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn11, locals.var_vgp_res_dn14,)
    } else {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn11, locals.var_vgp_res_dn14,)
    }
};
        locals.var_vgp_res = assign41900_e55912;
        locals.var_vgp_res_dn0 = assign41900_e55912_d_n0;
        locals.var_vgp_res_dn2 = assign41900_e55912_d_n2;
        locals.var_vgp_res_dn4 = assign41900_e55912_d_n4;
        locals.var_vgp_res_dn5 = assign41900_e55912_d_n5;
        locals.var_vgp_res_dn6 = assign41900_e55912_d_n6;
        locals.var_vgp_res_dn7 = assign41900_e55912_d_n7;
        locals.var_vgp_res_dn8 = assign41900_e55912_d_n8;
        locals.var_vgp_res_dn9 = assign41900_e55912_d_n9;
        locals.var_vgp_res_dn10 = assign41900_e55912_d_n10;
        locals.var_vgp_res_dn11 = assign41900_e55912_d_n11;
        locals.var_vgp_res_dn14 = assign41900_e55912_d_n14;
        locals.var_vgp_res_rv = 0.0;

        let (assign41910_e55927, assign41910_e55927_d_n0, assign41910_e55927_d_n2, assign41910_e55927_d_n4, assign41910_e55927_d_n5, assign41910_e55927_d_n6, assign41910_e55927_d_n7, assign41910_e55927_d_n8, assign41910_e55927_d_n9, assign41910_e55927_d_n10, assign41910_e55927_d_n11, assign41910_e55927_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1039 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41910_e55927;
        locals.var_t0_dn0 = assign41910_e55927_d_n0;
        locals.var_t0_dn2 = assign41910_e55927_d_n2;
        locals.var_t0_dn4 = assign41910_e55927_d_n4;
        locals.var_t0_dn5 = assign41910_e55927_d_n5;
        locals.var_t0_dn6 = assign41910_e55927_d_n6;
        locals.var_t0_dn7 = assign41910_e55927_d_n7;
        locals.var_t0_dn8 = assign41910_e55927_d_n8;
        locals.var_t0_dn9 = assign41910_e55927_d_n9;
        locals.var_t0_dn10 = assign41910_e55927_d_n10;
        locals.var_t0_dn11 = assign41910_e55927_d_n11;
        locals.var_t0_dn14 = assign41910_e55927_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41920_e55939,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign41920_e55939;
        locals.var_flg_conv_rv = 0.0;

        let (assign41930_e55951,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign41930_e55951;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_144(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign41940_loop_guard: usize = 0;
        while {
            let assign41940_cond_e55964: f64 = if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign41940_cond_e55964 != 0.0
        } {
            assign41940_loop_guard += 1;
            assert!(assign41940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41940_body0_e55978, assign41940_body0_e55978_d_n0, assign41940_body0_e55978_d_n2, assign41940_body0_e55978_d_n4, assign41940_body0_e55978_d_n5, assign41940_body0_e55978_d_n6, assign41940_body0_e55978_d_n7, assign41940_body0_e55978_d_n8, assign41940_body0_e55978_d_n9, assign41940_body0_e55978_d_n10, assign41940_body0_e55978_d_n11, assign41940_body0_e55978_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign41940_body0_e55976: f64 = (locals.var_beta * locals.var_ps0dep);
        (assign41940_body0_e55976, ((locals.var_beta_dn0 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn0)), ((locals.var_beta_dn2 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn2)), ((locals.var_beta_dn4 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn4)), ((locals.var_beta_dn5 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn5)), ((locals.var_beta_dn6 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn6)), ((locals.var_beta_dn7 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn7)), ((locals.var_beta_dn8 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn8)), ((locals.var_beta_dn9 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn9)), ((locals.var_beta_dn10 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn10)), ((locals.var_beta_dn11 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn11)), ((locals.var_beta_dn14 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign41940_body0_e55978;
            locals.var_t1_dn0 = assign41940_body0_e55978_d_n0;
            locals.var_t1_dn2 = assign41940_body0_e55978_d_n2;
            locals.var_t1_dn4 = assign41940_body0_e55978_d_n4;
            locals.var_t1_dn5 = assign41940_body0_e55978_d_n5;
            locals.var_t1_dn6 = assign41940_body0_e55978_d_n6;
            locals.var_t1_dn7 = assign41940_body0_e55978_d_n7;
            locals.var_t1_dn8 = assign41940_body0_e55978_d_n8;
            locals.var_t1_dn9 = assign41940_body0_e55978_d_n9;
            locals.var_t1_dn10 = assign41940_body0_e55978_d_n10;
            locals.var_t1_dn11 = assign41940_body0_e55978_d_n11;
            locals.var_t1_dn14 = assign41940_body0_e55978_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign41940_body1_e55991, assign41940_body1_e55991_d_n0, assign41940_body1_e55991_d_n2, assign41940_body1_e55991_d_n4, assign41940_body1_e55991_d_n5, assign41940_body1_e55991_d_n6, assign41940_body1_e55991_d_n7, assign41940_body1_e55991_d_n8, assign41940_body1_e55991_d_n9, assign41940_body1_e55991_d_n10, assign41940_body1_e55991_d_n11, assign41940_body1_e55991_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign41940_body1_e55989: f64 = (locals.var_t1).exp();
        (assign41940_body1_e55989, (assign41940_body1_e55989 * locals.var_t1_dn0), (assign41940_body1_e55989 * locals.var_t1_dn2), (assign41940_body1_e55989 * locals.var_t1_dn4), (assign41940_body1_e55989 * locals.var_t1_dn5), (assign41940_body1_e55989 * locals.var_t1_dn6), (assign41940_body1_e55989 * locals.var_t1_dn7), (assign41940_body1_e55989 * locals.var_t1_dn8), (assign41940_body1_e55989 * locals.var_t1_dn9), (assign41940_body1_e55989 * locals.var_t1_dn10), (assign41940_body1_e55989 * locals.var_t1_dn11), (assign41940_body1_e55989 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign41940_body1_e55991;
            locals.var_t2_dn0 = assign41940_body1_e55991_d_n0;
            locals.var_t2_dn2 = assign41940_body1_e55991_d_n2;
            locals.var_t2_dn4 = assign41940_body1_e55991_d_n4;
            locals.var_t2_dn5 = assign41940_body1_e55991_d_n5;
            locals.var_t2_dn6 = assign41940_body1_e55991_d_n6;
            locals.var_t2_dn7 = assign41940_body1_e55991_d_n7;
            locals.var_t2_dn8 = assign41940_body1_e55991_d_n8;
            locals.var_t2_dn9 = assign41940_body1_e55991_d_n9;
            locals.var_t2_dn10 = assign41940_body1_e55991_d_n10;
            locals.var_t2_dn11 = assign41940_body1_e55991_d_n11;
            locals.var_t2_dn14 = assign41940_body1_e55991_d_n14;
            locals.var_t2_rv = 0.0;
            let assign41940_body2_e55994: f64 = if locals.var_ps0dep >= 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1045 = assign41940_body2_e55994;
            locals.var_guard1045_rv = 0.0;
            let (assign41940_body3_e56018, assign41940_body3_e56018_d_n0, assign41940_body3_e56018_d_n2, assign41940_body3_e56018_d_n4, assign41940_body3_e56018_d_n5, assign41940_body3_e56018_d_n6, assign41940_body3_e56018_d_n7, assign41940_body3_e56018_d_n8, assign41940_body3_e56018_d_n9, assign41940_body3_e56018_d_n10, assign41940_body3_e56018_d_n11, assign41940_body3_e56018_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1045 != 0.0)) {
        let assign41940_body3_e56007: f64 = (-locals.var_cnst0);
        let assign41940_body3_e56010: f64 = (locals.var_t2 - 1.0);
        let assign41940_body3_e56012: f64 = (assign41940_body3_e56010 - locals.var_t1);
        let assign41940_body3_e56014: f64 = (assign41940_body3_e56012 + 1e-15);
        let assign41940_body3_e56015: f64 = (assign41940_body3_e56014).sqrt();
        let assign41940_body3_e56016: f64 = (assign41940_body3_e56007 * assign41940_body3_e56015);
        (assign41940_body3_e56016, (((-locals.var_cnst0_dn0) * assign41940_body3_e56015) + (assign41940_body3_e56007 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign41940_body3_e56015)))), (((-locals.var_cnst0_dn2) * assign41940_body3_e56015) + (assign41940_body3_e56007 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign41940_body3_e56015)))), (((-locals.var_cnst0_dn4) * assign41940_body3_e56015) + (assign41940_body3_e56007 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign41940_body3_e56015)))), (((-locals.var_cnst0_dn5) * assign41940_body3_e56015) + (assign41940_body3_e56007 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign41940_body3_e56015)))), (((-locals.var_cnst0_dn6) * assign41940_body3_e56015) + (assign41940_body3_e56007 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign41940_body3_e56015)))), (((-locals.var_cnst0_dn7) * assign41940_body3_e56015) + (assign41940_body3_e56007 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign41940_body3_e56015)))), (((-locals.var_cnst0_dn8) * assign41940_body3_e56015) + (assign41940_body3_e56007 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign41940_body3_e56015)))), (((-locals.var_cnst0_dn9) * assign41940_body3_e56015) + (assign41940_body3_e56007 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign41940_body3_e56015)))), (((-locals.var_cnst0_dn10) * assign41940_body3_e56015) + (assign41940_body3_e56007 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign41940_body3_e56015)))), (((-locals.var_cnst0_dn11) * assign41940_body3_e56015) + (assign41940_body3_e56007 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign41940_body3_e56015)))), (((-locals.var_cnst0_dn14) * assign41940_body3_e56015) + (assign41940_body3_e56007 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign41940_body3_e56015)))),)
    } else {
        (locals.var_q_s0__blk1030, locals.var_q_s0__blk1030_dn0, locals.var_q_s0__blk1030_dn2, locals.var_q_s0__blk1030_dn4, locals.var_q_s0__blk1030_dn5, locals.var_q_s0__blk1030_dn6, locals.var_q_s0__blk1030_dn7, locals.var_q_s0__blk1030_dn8, locals.var_q_s0__blk1030_dn9, locals.var_q_s0__blk1030_dn10, locals.var_q_s0__blk1030_dn11, locals.var_q_s0__blk1030_dn14,)
    }
};
            locals.var_q_s0__blk1030 = assign41940_body3_e56018;
            locals.var_q_s0__blk1030_dn0 = assign41940_body3_e56018_d_n0;
            locals.var_q_s0__blk1030_dn2 = assign41940_body3_e56018_d_n2;
            locals.var_q_s0__blk1030_dn4 = assign41940_body3_e56018_d_n4;
            locals.var_q_s0__blk1030_dn5 = assign41940_body3_e56018_d_n5;
            locals.var_q_s0__blk1030_dn6 = assign41940_body3_e56018_d_n6;
            locals.var_q_s0__blk1030_dn7 = assign41940_body3_e56018_d_n7;
            locals.var_q_s0__blk1030_dn8 = assign41940_body3_e56018_d_n8;
            locals.var_q_s0__blk1030_dn9 = assign41940_body3_e56018_d_n9;
            locals.var_q_s0__blk1030_dn10 = assign41940_body3_e56018_d_n10;
            locals.var_q_s0__blk1030_dn11 = assign41940_body3_e56018_d_n11;
            locals.var_q_s0__blk1030_dn14 = assign41940_body3_e56018_d_n14;
            locals.var_q_s0__blk1030_rv = 0.0;
            let (assign41940_body4_e56044, assign41940_body4_e56044_d_n0, assign41940_body4_e56044_d_n2, assign41940_body4_e56044_d_n4, assign41940_body4_e56044_d_n5, assign41940_body4_e56044_d_n6, assign41940_body4_e56044_d_n7, assign41940_body4_e56044_d_n8, assign41940_body4_e56044_d_n9, assign41940_body4_e56044_d_n10, assign41940_body4_e56044_d_n11, assign41940_body4_e56044_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1045 != 0.0)) {
        let assign41940_body4_e56032: f64 = (0.5 * locals.var_cnst0);
        let assign41940_body4_e56034: f64 = (assign41940_body4_e56032 * locals.var_cnst0);
        let assign41940_body4_e56036: f64 = (assign41940_body4_e56034 / locals.var_q_s0__blk1030);
        let assign41940_body4_e56039: f64 = (locals.var_beta * locals.var_t2);
        let assign41940_body4_e56041: f64 = (assign41940_body4_e56039 - locals.var_beta);
        let assign41940_body4_e56042: f64 = (assign41940_body4_e56036 * assign41940_body4_e56041);
        (assign41940_body4_e56042, ((((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign41940_body4_e56032 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1030) - (assign41940_body4_e56034 * locals.var_q_s0__blk1030_dn0)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)) * assign41940_body4_e56041) + (assign41940_body4_e56036 * (((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0))), ((((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign41940_body4_e56032 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1030) - (assign41940_body4_e56034 * locals.var_q_s0__blk1030_dn2)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)) * assign41940_body4_e56041) + (assign41940_body4_e56036 * (((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2))), ((((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign41940_body4_e56032 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1030) - (assign41940_body4_e56034 * locals.var_q_s0__blk1030_dn4)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)) * assign41940_body4_e56041) + (assign41940_body4_e56036 * (((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4))), ((((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign41940_body4_e56032 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1030) - (assign41940_body4_e56034 * locals.var_q_s0__blk1030_dn5)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)) * assign41940_body4_e56041) + (assign41940_body4_e56036 * (((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5))), ((((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign41940_body4_e56032 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1030) - (assign41940_body4_e56034 * locals.var_q_s0__blk1030_dn6)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)) * assign41940_body4_e56041) + (assign41940_body4_e56036 * (((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6))), ((((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign41940_body4_e56032 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1030) - (assign41940_body4_e56034 * locals.var_q_s0__blk1030_dn7)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)) * assign41940_body4_e56041) + (assign41940_body4_e56036 * (((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7))), ((((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign41940_body4_e56032 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1030) - (assign41940_body4_e56034 * locals.var_q_s0__blk1030_dn8)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)) * assign41940_body4_e56041) + (assign41940_body4_e56036 * (((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8))), ((((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign41940_body4_e56032 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1030) - (assign41940_body4_e56034 * locals.var_q_s0__blk1030_dn9)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)) * assign41940_body4_e56041) + (assign41940_body4_e56036 * (((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9))), ((((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign41940_body4_e56032 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1030) - (assign41940_body4_e56034 * locals.var_q_s0__blk1030_dn10)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)) * assign41940_body4_e56041) + (assign41940_body4_e56036 * (((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10))), ((((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign41940_body4_e56032 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1030) - (assign41940_body4_e56034 * locals.var_q_s0__blk1030_dn11)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)) * assign41940_body4_e56041) + (assign41940_body4_e56036 * (((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11))), ((((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign41940_body4_e56032 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1030) - (assign41940_body4_e56034 * locals.var_q_s0__blk1030_dn14)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)) * assign41940_body4_e56041) + (assign41940_body4_e56036 * (((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14))),)
    } else {
        (locals.var_q_s0_dps__blk1031, locals.var_q_s0_dps__blk1031_dn0, locals.var_q_s0_dps__blk1031_dn2, locals.var_q_s0_dps__blk1031_dn4, locals.var_q_s0_dps__blk1031_dn5, locals.var_q_s0_dps__blk1031_dn6, locals.var_q_s0_dps__blk1031_dn7, locals.var_q_s0_dps__blk1031_dn8, locals.var_q_s0_dps__blk1031_dn9, locals.var_q_s0_dps__blk1031_dn10, locals.var_q_s0_dps__blk1031_dn11, locals.var_q_s0_dps__blk1031_dn14,)
    }
};
            locals.var_q_s0_dps__blk1031 = assign41940_body4_e56044;
            locals.var_q_s0_dps__blk1031_dn0 = assign41940_body4_e56044_d_n0;
            locals.var_q_s0_dps__blk1031_dn2 = assign41940_body4_e56044_d_n2;
            locals.var_q_s0_dps__blk1031_dn4 = assign41940_body4_e56044_d_n4;
            locals.var_q_s0_dps__blk1031_dn5 = assign41940_body4_e56044_d_n5;
            locals.var_q_s0_dps__blk1031_dn6 = assign41940_body4_e56044_d_n6;
            locals.var_q_s0_dps__blk1031_dn7 = assign41940_body4_e56044_d_n7;
            locals.var_q_s0_dps__blk1031_dn8 = assign41940_body4_e56044_d_n8;
            locals.var_q_s0_dps__blk1031_dn9 = assign41940_body4_e56044_d_n9;
            locals.var_q_s0_dps__blk1031_dn10 = assign41940_body4_e56044_d_n10;
            locals.var_q_s0_dps__blk1031_dn11 = assign41940_body4_e56044_d_n11;
            locals.var_q_s0_dps__blk1031_dn14 = assign41940_body4_e56044_d_n14;
            locals.var_q_s0_dps__blk1031_rv = 0.0;
            let (assign41940_body5_e56065, assign41940_body5_e56065_d_n0, assign41940_body5_e56065_d_n2, assign41940_body5_e56065_d_n4, assign41940_body5_e56065_d_n5, assign41940_body5_e56065_d_n6, assign41940_body5_e56065_d_n7, assign41940_body5_e56065_d_n8, assign41940_body5_e56065_d_n9, assign41940_body5_e56065_d_n10, assign41940_body5_e56065_d_n11, assign41940_body5_e56065_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1045 == 0.0)) {
        let assign41940_body5_e56058: f64 = (-locals.var_beta);
        let assign41940_body5_e56061: f64 = (locals.var_ps0dep - locals.var_vbsc);
        let assign41940_body5_e56062: f64 = (assign41940_body5_e56058 * assign41940_body5_e56061);
        let assign41940_body5_e56063: f64 = (assign41940_body5_e56062).exp();
        (assign41940_body5_e56063, (assign41940_body5_e56063 * (((-locals.var_beta_dn0) * assign41940_body5_e56061) + (assign41940_body5_e56058 * (locals.var_ps0dep_dn0 - locals.var_vbsc_dn0)))), (assign41940_body5_e56063 * (((-locals.var_beta_dn2) * assign41940_body5_e56061) + (assign41940_body5_e56058 * (locals.var_ps0dep_dn2 - locals.var_vbsc_dn2)))), (assign41940_body5_e56063 * (((-locals.var_beta_dn4) * assign41940_body5_e56061) + (assign41940_body5_e56058 * (locals.var_ps0dep_dn4 - locals.var_vbsc_dn4)))), (assign41940_body5_e56063 * (((-locals.var_beta_dn5) * assign41940_body5_e56061) + (assign41940_body5_e56058 * (locals.var_ps0dep_dn5 - locals.var_vbsc_dn5)))), (assign41940_body5_e56063 * (((-locals.var_beta_dn6) * assign41940_body5_e56061) + (assign41940_body5_e56058 * (locals.var_ps0dep_dn6 - locals.var_vbsc_dn6)))), (assign41940_body5_e56063 * (((-locals.var_beta_dn7) * assign41940_body5_e56061) + (assign41940_body5_e56058 * (locals.var_ps0dep_dn7 - locals.var_vbsc_dn7)))), (assign41940_body5_e56063 * (((-locals.var_beta_dn8) * assign41940_body5_e56061) + (assign41940_body5_e56058 * (locals.var_ps0dep_dn8 - locals.var_vbsc_dn8)))), (assign41940_body5_e56063 * (((-locals.var_beta_dn9) * assign41940_body5_e56061) + (assign41940_body5_e56058 * (locals.var_ps0dep_dn9 - locals.var_vbsc_dn9)))), (assign41940_body5_e56063 * (((-locals.var_beta_dn10) * assign41940_body5_e56061) + (assign41940_body5_e56058 * (locals.var_ps0dep_dn10 - locals.var_vbsc_dn10)))), (assign41940_body5_e56063 * (((-locals.var_beta_dn11) * assign41940_body5_e56061) + (assign41940_body5_e56058 * (locals.var_ps0dep_dn11 - locals.var_vbsc_dn11)))), (assign41940_body5_e56063 * (((-locals.var_beta_dn14) * assign41940_body5_e56061) + (assign41940_body5_e56058 * (locals.var_ps0dep_dn14 - locals.var_vbsc_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign41940_body5_e56065;
            locals.var_t3_dn0 = assign41940_body5_e56065_d_n0;
            locals.var_t3_dn2 = assign41940_body5_e56065_d_n2;
            locals.var_t3_dn4 = assign41940_body5_e56065_d_n4;
            locals.var_t3_dn5 = assign41940_body5_e56065_d_n5;
            locals.var_t3_dn6 = assign41940_body5_e56065_d_n6;
            locals.var_t3_dn7 = assign41940_body5_e56065_d_n7;
            locals.var_t3_dn8 = assign41940_body5_e56065_d_n8;
            locals.var_t3_dn9 = assign41940_body5_e56065_d_n9;
            locals.var_t3_dn10 = assign41940_body5_e56065_d_n10;
            locals.var_t3_dn11 = assign41940_body5_e56065_d_n11;
            locals.var_t3_dn14 = assign41940_body5_e56065_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign41940_body6_e56085, assign41940_body6_e56085_d_n0, assign41940_body6_e56085_d_n2, assign41940_body6_e56085_d_n4, assign41940_body6_e56085_d_n5, assign41940_body6_e56085_d_n6, assign41940_body6_e56085_d_n7, assign41940_body6_e56085_d_n8, assign41940_body6_e56085_d_n9, assign41940_body6_e56085_d_n10, assign41940_body6_e56085_d_n11, assign41940_body6_e56085_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1045 == 0.0)) {
        let assign41940_body6_e56079: f64 = (-locals.var_beta);
        let assign41940_body6_e56081: f64 = (-locals.var_vbsc);
        let assign41940_body6_e56082: f64 = (assign41940_body6_e56079 * assign41940_body6_e56081);
        let assign41940_body6_e56083: f64 = (assign41940_body6_e56082).exp();
        (assign41940_body6_e56083, (assign41940_body6_e56083 * (((-locals.var_beta_dn0) * assign41940_body6_e56081) + (assign41940_body6_e56079 * (-locals.var_vbsc_dn0)))), (assign41940_body6_e56083 * (((-locals.var_beta_dn2) * assign41940_body6_e56081) + (assign41940_body6_e56079 * (-locals.var_vbsc_dn2)))), (assign41940_body6_e56083 * (((-locals.var_beta_dn4) * assign41940_body6_e56081) + (assign41940_body6_e56079 * (-locals.var_vbsc_dn4)))), (assign41940_body6_e56083 * (((-locals.var_beta_dn5) * assign41940_body6_e56081) + (assign41940_body6_e56079 * (-locals.var_vbsc_dn5)))), (assign41940_body6_e56083 * (((-locals.var_beta_dn6) * assign41940_body6_e56081) + (assign41940_body6_e56079 * (-locals.var_vbsc_dn6)))), (assign41940_body6_e56083 * (((-locals.var_beta_dn7) * assign41940_body6_e56081) + (assign41940_body6_e56079 * (-locals.var_vbsc_dn7)))), (assign41940_body6_e56083 * (((-locals.var_beta_dn8) * assign41940_body6_e56081) + (assign41940_body6_e56079 * (-locals.var_vbsc_dn8)))), (assign41940_body6_e56083 * (((-locals.var_beta_dn9) * assign41940_body6_e56081) + (assign41940_body6_e56079 * (-locals.var_vbsc_dn9)))), (assign41940_body6_e56083 * (((-locals.var_beta_dn10) * assign41940_body6_e56081) + (assign41940_body6_e56079 * (-locals.var_vbsc_dn10)))), (assign41940_body6_e56083 * (((-locals.var_beta_dn11) * assign41940_body6_e56081) + (assign41940_body6_e56079 * (-locals.var_vbsc_dn11)))), (assign41940_body6_e56083 * (((-locals.var_beta_dn14) * assign41940_body6_e56081) + (assign41940_body6_e56079 * (-locals.var_vbsc_dn14)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign41940_body6_e56085;
            locals.var_t4_dn0 = assign41940_body6_e56085_d_n0;
            locals.var_t4_dn2 = assign41940_body6_e56085_d_n2;
            locals.var_t4_dn4 = assign41940_body6_e56085_d_n4;
            locals.var_t4_dn5 = assign41940_body6_e56085_d_n5;
            locals.var_t4_dn6 = assign41940_body6_e56085_d_n6;
            locals.var_t4_dn7 = assign41940_body6_e56085_d_n7;
            locals.var_t4_dn8 = assign41940_body6_e56085_d_n8;
            locals.var_t4_dn9 = assign41940_body6_e56085_d_n9;
            locals.var_t4_dn10 = assign41940_body6_e56085_d_n10;
            locals.var_t4_dn11 = assign41940_body6_e56085_d_n11;
            locals.var_t4_dn14 = assign41940_body6_e56085_d_n14;
            locals.var_t4_rv = 0.0;
            let (assign41940_body7_e56115, assign41940_body7_e56115_d_n0, assign41940_body7_e56115_d_n2, assign41940_body7_e56115_d_n4, assign41940_body7_e56115_d_n5, assign41940_body7_e56115_d_n6, assign41940_body7_e56115_d_n7, assign41940_body7_e56115_d_n8, assign41940_body7_e56115_d_n9, assign41940_body7_e56115_d_n10, assign41940_body7_e56115_d_n11, assign41940_body7_e56115_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1045 == 0.0)) {
        let assign41940_body7_e56101: f64 = (locals.var_t2 - 1.0);
        let assign41940_body7_e56103: f64 = (assign41940_body7_e56101 - locals.var_t1);
        let assign41940_body7_e56107: f64 = (locals.var_t3 - locals.var_t4);
        let assign41940_body7_e56108: f64 = (locals.var_cnst1 * assign41940_body7_e56107);
        let assign41940_body7_e56109: f64 = (assign41940_body7_e56103 + assign41940_body7_e56108);
        let assign41940_body7_e56111: f64 = (assign41940_body7_e56109 + 1e-15);
        let assign41940_body7_e56112: f64 = (assign41940_body7_e56111).sqrt();
        let assign41940_body7_e56113: f64 = (locals.var_cnst0 * assign41940_body7_e56112);
        (assign41940_body7_e56113, ((locals.var_cnst0_dn0 * assign41940_body7_e56112) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign41940_body7_e56107) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign41940_body7_e56112)))), ((locals.var_cnst0_dn2 * assign41940_body7_e56112) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign41940_body7_e56107) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign41940_body7_e56112)))), ((locals.var_cnst0_dn4 * assign41940_body7_e56112) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign41940_body7_e56107) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign41940_body7_e56112)))), ((locals.var_cnst0_dn5 * assign41940_body7_e56112) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign41940_body7_e56107) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign41940_body7_e56112)))), ((locals.var_cnst0_dn6 * assign41940_body7_e56112) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign41940_body7_e56107) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign41940_body7_e56112)))), ((locals.var_cnst0_dn7 * assign41940_body7_e56112) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign41940_body7_e56107) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign41940_body7_e56112)))), ((locals.var_cnst0_dn8 * assign41940_body7_e56112) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign41940_body7_e56107) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign41940_body7_e56112)))), ((locals.var_cnst0_dn9 * assign41940_body7_e56112) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign41940_body7_e56107) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign41940_body7_e56112)))), ((locals.var_cnst0_dn10 * assign41940_body7_e56112) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign41940_body7_e56107) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign41940_body7_e56112)))), ((locals.var_cnst0_dn11 * assign41940_body7_e56112) + (locals.var_cnst0 * (((locals.var_t2_dn11 - locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign41940_body7_e56107) + (locals.var_cnst1 * (locals.var_t3_dn11 - locals.var_t4_dn11)))) / (2.0 * assign41940_body7_e56112)))), ((locals.var_cnst0_dn14 * assign41940_body7_e56112) + (locals.var_cnst0 * (((locals.var_t2_dn14 - locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign41940_body7_e56107) + (locals.var_cnst1 * (locals.var_t3_dn14 - locals.var_t4_dn14)))) / (2.0 * assign41940_body7_e56112)))),)
    } else {
        (locals.var_q_s0__blk1030, locals.var_q_s0__blk1030_dn0, locals.var_q_s0__blk1030_dn2, locals.var_q_s0__blk1030_dn4, locals.var_q_s0__blk1030_dn5, locals.var_q_s0__blk1030_dn6, locals.var_q_s0__blk1030_dn7, locals.var_q_s0__blk1030_dn8, locals.var_q_s0__blk1030_dn9, locals.var_q_s0__blk1030_dn10, locals.var_q_s0__blk1030_dn11, locals.var_q_s0__blk1030_dn14,)
    }
};
            locals.var_q_s0__blk1030 = assign41940_body7_e56115;
            locals.var_q_s0__blk1030_dn0 = assign41940_body7_e56115_d_n0;
            locals.var_q_s0__blk1030_dn2 = assign41940_body7_e56115_d_n2;
            locals.var_q_s0__blk1030_dn4 = assign41940_body7_e56115_d_n4;
            locals.var_q_s0__blk1030_dn5 = assign41940_body7_e56115_d_n5;
            locals.var_q_s0__blk1030_dn6 = assign41940_body7_e56115_d_n6;
            locals.var_q_s0__blk1030_dn7 = assign41940_body7_e56115_d_n7;
            locals.var_q_s0__blk1030_dn8 = assign41940_body7_e56115_d_n8;
            locals.var_q_s0__blk1030_dn9 = assign41940_body7_e56115_d_n9;
            locals.var_q_s0__blk1030_dn10 = assign41940_body7_e56115_d_n10;
            locals.var_q_s0__blk1030_dn11 = assign41940_body7_e56115_d_n11;
            locals.var_q_s0__blk1030_dn14 = assign41940_body7_e56115_d_n14;
            locals.var_q_s0__blk1030_rv = 0.0;
            let (assign41940_body8_e56136, assign41940_body8_e56136_d_n0, assign41940_body8_e56136_d_n2, assign41940_body8_e56136_d_n4, assign41940_body8_e56136_d_n5, assign41940_body8_e56136_d_n6, assign41940_body8_e56136_d_n7, assign41940_body8_e56136_d_n8, assign41940_body8_e56136_d_n9, assign41940_body8_e56136_d_n10, assign41940_body8_e56136_d_n11, assign41940_body8_e56136_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1045 == 0.0)) {
        let assign41940_body8_e56130: f64 = (0.5 * locals.var_cnst0);
        let assign41940_body8_e56132: f64 = (assign41940_body8_e56130 * locals.var_cnst0);
        let assign41940_body8_e56134: f64 = (assign41940_body8_e56132 / locals.var_q_s0__blk1030);
        (assign41940_body8_e56134, ((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign41940_body8_e56130 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1030) - (assign41940_body8_e56132 * locals.var_q_s0__blk1030_dn0)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)), ((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign41940_body8_e56130 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1030) - (assign41940_body8_e56132 * locals.var_q_s0__blk1030_dn2)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)), ((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign41940_body8_e56130 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1030) - (assign41940_body8_e56132 * locals.var_q_s0__blk1030_dn4)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)), ((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign41940_body8_e56130 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1030) - (assign41940_body8_e56132 * locals.var_q_s0__blk1030_dn5)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)), ((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign41940_body8_e56130 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1030) - (assign41940_body8_e56132 * locals.var_q_s0__blk1030_dn6)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)), ((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign41940_body8_e56130 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1030) - (assign41940_body8_e56132 * locals.var_q_s0__blk1030_dn7)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)), ((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign41940_body8_e56130 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1030) - (assign41940_body8_e56132 * locals.var_q_s0__blk1030_dn8)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)), ((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign41940_body8_e56130 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1030) - (assign41940_body8_e56132 * locals.var_q_s0__blk1030_dn9)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)), ((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign41940_body8_e56130 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1030) - (assign41940_body8_e56132 * locals.var_q_s0__blk1030_dn10)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)), ((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign41940_body8_e56130 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1030) - (assign41940_body8_e56132 * locals.var_q_s0__blk1030_dn11)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)), ((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign41940_body8_e56130 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1030) - (assign41940_body8_e56132 * locals.var_q_s0__blk1030_dn14)) / (locals.var_q_s0__blk1030 * locals.var_q_s0__blk1030)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
            locals.var_t5 = assign41940_body8_e56136;
            locals.var_t5_dn0 = assign41940_body8_e56136_d_n0;
            locals.var_t5_dn2 = assign41940_body8_e56136_d_n2;
            locals.var_t5_dn4 = assign41940_body8_e56136_d_n4;
            locals.var_t5_dn5 = assign41940_body8_e56136_d_n5;
            locals.var_t5_dn6 = assign41940_body8_e56136_d_n6;
            locals.var_t5_dn7 = assign41940_body8_e56136_d_n7;
            locals.var_t5_dn8 = assign41940_body8_e56136_d_n8;
            locals.var_t5_dn9 = assign41940_body8_e56136_d_n9;
            locals.var_t5_dn10 = assign41940_body8_e56136_d_n10;
            locals.var_t5_dn11 = assign41940_body8_e56136_d_n11;
            locals.var_t5_dn14 = assign41940_body8_e56136_d_n14;
            locals.var_t5_rv = 0.0;
            let (assign41940_body9_e56164, assign41940_body9_e56164_d_n0, assign41940_body9_e56164_d_n2, assign41940_body9_e56164_d_n4, assign41940_body9_e56164_d_n5, assign41940_body9_e56164_d_n6, assign41940_body9_e56164_d_n7, assign41940_body9_e56164_d_n8, assign41940_body9_e56164_d_n9, assign41940_body9_e56164_d_n10, assign41940_body9_e56164_d_n11, assign41940_body9_e56164_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1045 == 0.0)) {
        let assign41940_body9_e56152: f64 = (locals.var_beta * locals.var_t2);
        let assign41940_body9_e56154: f64 = (assign41940_body9_e56152 - locals.var_beta);
        let assign41940_body9_e56157: f64 = (-locals.var_beta);
        let assign41940_body9_e56159: f64 = (assign41940_body9_e56157 * locals.var_t3);
        let assign41940_body9_e56160: f64 = (locals.var_cnst1 * assign41940_body9_e56159);
        let assign41940_body9_e56161: f64 = (assign41940_body9_e56154 + assign41940_body9_e56160);
        let assign41940_body9_e56162: f64 = (locals.var_t5 * assign41940_body9_e56161);
        (assign41940_body9_e56162, ((locals.var_t5_dn0 * assign41940_body9_e56161) + (locals.var_t5 * ((((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0) + ((locals.var_cnst1_dn0 * assign41940_body9_e56159) + (locals.var_cnst1 * (((-locals.var_beta_dn0) * locals.var_t3) + (assign41940_body9_e56157 * locals.var_t3_dn0))))))), ((locals.var_t5_dn2 * assign41940_body9_e56161) + (locals.var_t5 * ((((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2) + ((locals.var_cnst1_dn2 * assign41940_body9_e56159) + (locals.var_cnst1 * (((-locals.var_beta_dn2) * locals.var_t3) + (assign41940_body9_e56157 * locals.var_t3_dn2))))))), ((locals.var_t5_dn4 * assign41940_body9_e56161) + (locals.var_t5 * ((((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4) + ((locals.var_cnst1_dn4 * assign41940_body9_e56159) + (locals.var_cnst1 * (((-locals.var_beta_dn4) * locals.var_t3) + (assign41940_body9_e56157 * locals.var_t3_dn4))))))), ((locals.var_t5_dn5 * assign41940_body9_e56161) + (locals.var_t5 * ((((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5) + ((locals.var_cnst1_dn5 * assign41940_body9_e56159) + (locals.var_cnst1 * (((-locals.var_beta_dn5) * locals.var_t3) + (assign41940_body9_e56157 * locals.var_t3_dn5))))))), ((locals.var_t5_dn6 * assign41940_body9_e56161) + (locals.var_t5 * ((((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6) + ((locals.var_cnst1_dn6 * assign41940_body9_e56159) + (locals.var_cnst1 * (((-locals.var_beta_dn6) * locals.var_t3) + (assign41940_body9_e56157 * locals.var_t3_dn6))))))), ((locals.var_t5_dn7 * assign41940_body9_e56161) + (locals.var_t5 * ((((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7) + ((locals.var_cnst1_dn7 * assign41940_body9_e56159) + (locals.var_cnst1 * (((-locals.var_beta_dn7) * locals.var_t3) + (assign41940_body9_e56157 * locals.var_t3_dn7))))))), ((locals.var_t5_dn8 * assign41940_body9_e56161) + (locals.var_t5 * ((((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8) + ((locals.var_cnst1_dn8 * assign41940_body9_e56159) + (locals.var_cnst1 * (((-locals.var_beta_dn8) * locals.var_t3) + (assign41940_body9_e56157 * locals.var_t3_dn8))))))), ((locals.var_t5_dn9 * assign41940_body9_e56161) + (locals.var_t5 * ((((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9) + ((locals.var_cnst1_dn9 * assign41940_body9_e56159) + (locals.var_cnst1 * (((-locals.var_beta_dn9) * locals.var_t3) + (assign41940_body9_e56157 * locals.var_t3_dn9))))))), ((locals.var_t5_dn10 * assign41940_body9_e56161) + (locals.var_t5 * ((((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10) + ((locals.var_cnst1_dn10 * assign41940_body9_e56159) + (locals.var_cnst1 * (((-locals.var_beta_dn10) * locals.var_t3) + (assign41940_body9_e56157 * locals.var_t3_dn10))))))), ((locals.var_t5_dn11 * assign41940_body9_e56161) + (locals.var_t5 * ((((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11) + ((locals.var_cnst1_dn11 * assign41940_body9_e56159) + (locals.var_cnst1 * (((-locals.var_beta_dn11) * locals.var_t3) + (assign41940_body9_e56157 * locals.var_t3_dn11))))))), ((locals.var_t5_dn14 * assign41940_body9_e56161) + (locals.var_t5 * ((((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14) + ((locals.var_cnst1_dn14 * assign41940_body9_e56159) + (locals.var_cnst1 * (((-locals.var_beta_dn14) * locals.var_t3) + (assign41940_body9_e56157 * locals.var_t3_dn14))))))),)
    } else {
        (locals.var_q_s0_dps__blk1031, locals.var_q_s0_dps__blk1031_dn0, locals.var_q_s0_dps__blk1031_dn2, locals.var_q_s0_dps__blk1031_dn4, locals.var_q_s0_dps__blk1031_dn5, locals.var_q_s0_dps__blk1031_dn6, locals.var_q_s0_dps__blk1031_dn7, locals.var_q_s0_dps__blk1031_dn8, locals.var_q_s0_dps__blk1031_dn9, locals.var_q_s0_dps__blk1031_dn10, locals.var_q_s0_dps__blk1031_dn11, locals.var_q_s0_dps__blk1031_dn14,)
    }
};
            locals.var_q_s0_dps__blk1031 = assign41940_body9_e56164;
            locals.var_q_s0_dps__blk1031_dn0 = assign41940_body9_e56164_d_n0;
            locals.var_q_s0_dps__blk1031_dn2 = assign41940_body9_e56164_d_n2;
            locals.var_q_s0_dps__blk1031_dn4 = assign41940_body9_e56164_d_n4;
            locals.var_q_s0_dps__blk1031_dn5 = assign41940_body9_e56164_d_n5;
            locals.var_q_s0_dps__blk1031_dn6 = assign41940_body9_e56164_d_n6;
            locals.var_q_s0_dps__blk1031_dn7 = assign41940_body9_e56164_d_n7;
            locals.var_q_s0_dps__blk1031_dn8 = assign41940_body9_e56164_d_n8;
            locals.var_q_s0_dps__blk1031_dn9 = assign41940_body9_e56164_d_n9;
            locals.var_q_s0_dps__blk1031_dn10 = assign41940_body9_e56164_d_n10;
            locals.var_q_s0_dps__blk1031_dn11 = assign41940_body9_e56164_d_n11;
            locals.var_q_s0_dps__blk1031_dn14 = assign41940_body9_e56164_d_n14;
            locals.var_q_s0_dps__blk1031_rv = 0.0;
            let (assign41940_body10_e56180,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign41940_body10_e56178: f64 = (150.0 + 1.0);
        (assign41940_body10_e56178,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign41940_body10_e56180;
            locals.var_lp_s0_rv = 0.0;
            let (assign41940_body11_e56201, assign41940_body11_e56201_d_n0, assign41940_body11_e56201_d_n2, assign41940_body11_e56201_d_n4, assign41940_body11_e56201_d_n5, assign41940_body11_e56201_d_n6, assign41940_body11_e56201_d_n7, assign41940_body11_e56201_d_n8, assign41940_body11_e56201_d_n9, assign41940_body11_e56201_d_n10, assign41940_body11_e56201_d_n11, assign41940_body11_e56201_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41940_body11_e56196: f64 = (locals.var_vgp_res - locals.var_ps0dep);
        let assign41940_body11_e56197: f64 = (locals.var_cox * assign41940_body11_e56196);
        let assign41940_body11_e56199: f64 = (assign41940_body11_e56197 + locals.var_q_s0__blk1030);
        (assign41940_body11_e56199, (((locals.var_cox_dn0 * assign41940_body11_e56196) + (locals.var_cox * (locals.var_vgp_res_dn0 - locals.var_ps0dep_dn0))) + locals.var_q_s0__blk1030_dn0), (((locals.var_cox_dn2 * assign41940_body11_e56196) + (locals.var_cox * (locals.var_vgp_res_dn2 - locals.var_ps0dep_dn2))) + locals.var_q_s0__blk1030_dn2), (((locals.var_cox_dn4 * assign41940_body11_e56196) + (locals.var_cox * (locals.var_vgp_res_dn4 - locals.var_ps0dep_dn4))) + locals.var_q_s0__blk1030_dn4), (((locals.var_cox_dn5 * assign41940_body11_e56196) + (locals.var_cox * (locals.var_vgp_res_dn5 - locals.var_ps0dep_dn5))) + locals.var_q_s0__blk1030_dn5), (((locals.var_cox_dn6 * assign41940_body11_e56196) + (locals.var_cox * (locals.var_vgp_res_dn6 - locals.var_ps0dep_dn6))) + locals.var_q_s0__blk1030_dn6), (((locals.var_cox_dn7 * assign41940_body11_e56196) + (locals.var_cox * (locals.var_vgp_res_dn7 - locals.var_ps0dep_dn7))) + locals.var_q_s0__blk1030_dn7), (((locals.var_cox_dn8 * assign41940_body11_e56196) + (locals.var_cox * (locals.var_vgp_res_dn8 - locals.var_ps0dep_dn8))) + locals.var_q_s0__blk1030_dn8), (((locals.var_cox_dn9 * assign41940_body11_e56196) + (locals.var_cox * (locals.var_vgp_res_dn9 - locals.var_ps0dep_dn9))) + locals.var_q_s0__blk1030_dn9), (((locals.var_cox_dn10 * assign41940_body11_e56196) + (locals.var_cox * (locals.var_vgp_res_dn10 - locals.var_ps0dep_dn10))) + locals.var_q_s0__blk1030_dn10), (((locals.var_cox_dn11 * assign41940_body11_e56196) + (locals.var_cox * (locals.var_vgp_res_dn11 - locals.var_ps0dep_dn11))) + locals.var_q_s0__blk1030_dn11), (((locals.var_cox_dn14 * assign41940_body11_e56196) + (locals.var_cox * (locals.var_vgp_res_dn14 - locals.var_ps0dep_dn14))) + locals.var_q_s0__blk1030_dn14),)
    } else {
        (locals.var_pf1, locals.var_pf1_dn0, locals.var_pf1_dn2, locals.var_pf1_dn4, locals.var_pf1_dn5, locals.var_pf1_dn6, locals.var_pf1_dn7, locals.var_pf1_dn8, locals.var_pf1_dn9, locals.var_pf1_dn10, locals.var_pf1_dn11, locals.var_pf1_dn14,)
    }
};
            locals.var_pf1 = assign41940_body11_e56201;
            locals.var_pf1_dn0 = assign41940_body11_e56201_d_n0;
            locals.var_pf1_dn2 = assign41940_body11_e56201_d_n2;
            locals.var_pf1_dn4 = assign41940_body11_e56201_d_n4;
            locals.var_pf1_dn5 = assign41940_body11_e56201_d_n5;
            locals.var_pf1_dn6 = assign41940_body11_e56201_d_n6;
            locals.var_pf1_dn7 = assign41940_body11_e56201_d_n7;
            locals.var_pf1_dn8 = assign41940_body11_e56201_d_n8;
            locals.var_pf1_dn9 = assign41940_body11_e56201_d_n9;
            locals.var_pf1_dn10 = assign41940_body11_e56201_d_n10;
            locals.var_pf1_dn11 = assign41940_body11_e56201_d_n11;
            locals.var_pf1_dn14 = assign41940_body11_e56201_d_n14;
            locals.var_pf1_rv = 0.0;
            let (assign41940_body12_e56219, assign41940_body12_e56219_d_n0, assign41940_body12_e56219_d_n2, assign41940_body12_e56219_d_n4, assign41940_body12_e56219_d_n5, assign41940_body12_e56219_d_n6, assign41940_body12_e56219_d_n7, assign41940_body12_e56219_d_n8, assign41940_body12_e56219_d_n9, assign41940_body12_e56219_d_n10, assign41940_body12_e56219_d_n11, assign41940_body12_e56219_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41940_body12_e56215: f64 = (-locals.var_cox);
        let assign41940_body12_e56217: f64 = (assign41940_body12_e56215 + locals.var_q_s0_dps__blk1031);
        (assign41940_body12_e56217, ((-locals.var_cox_dn0) + locals.var_q_s0_dps__blk1031_dn0), ((-locals.var_cox_dn2) + locals.var_q_s0_dps__blk1031_dn2), ((-locals.var_cox_dn4) + locals.var_q_s0_dps__blk1031_dn4), ((-locals.var_cox_dn5) + locals.var_q_s0_dps__blk1031_dn5), ((-locals.var_cox_dn6) + locals.var_q_s0_dps__blk1031_dn6), ((-locals.var_cox_dn7) + locals.var_q_s0_dps__blk1031_dn7), ((-locals.var_cox_dn8) + locals.var_q_s0_dps__blk1031_dn8), ((-locals.var_cox_dn9) + locals.var_q_s0_dps__blk1031_dn9), ((-locals.var_cox_dn10) + locals.var_q_s0_dps__blk1031_dn10), ((-locals.var_cox_dn11) + locals.var_q_s0_dps__blk1031_dn11), ((-locals.var_cox_dn14) + locals.var_q_s0_dps__blk1031_dn14),)
    } else {
        (locals.var_pf11, locals.var_pf11_dn0, locals.var_pf11_dn2, locals.var_pf11_dn4, locals.var_pf11_dn5, locals.var_pf11_dn6, locals.var_pf11_dn7, locals.var_pf11_dn8, locals.var_pf11_dn9, locals.var_pf11_dn10, locals.var_pf11_dn11, locals.var_pf11_dn14,)
    }
};
            locals.var_pf11 = assign41940_body12_e56219;
            locals.var_pf11_dn0 = assign41940_body12_e56219_d_n0;
            locals.var_pf11_dn2 = assign41940_body12_e56219_d_n2;
            locals.var_pf11_dn4 = assign41940_body12_e56219_d_n4;
            locals.var_pf11_dn5 = assign41940_body12_e56219_d_n5;
            locals.var_pf11_dn6 = assign41940_body12_e56219_d_n6;
            locals.var_pf11_dn7 = assign41940_body12_e56219_d_n7;
            locals.var_pf11_dn8 = assign41940_body12_e56219_d_n8;
            locals.var_pf11_dn9 = assign41940_body12_e56219_d_n9;
            locals.var_pf11_dn10 = assign41940_body12_e56219_d_n10;
            locals.var_pf11_dn11 = assign41940_body12_e56219_d_n11;
            locals.var_pf11_dn14 = assign41940_body12_e56219_d_n14;
            locals.var_pf11_rv = 0.0;
            let (assign41940_body13_e56237, assign41940_body13_e56237_d_n0, assign41940_body13_e56237_d_n2, assign41940_body13_e56237_d_n4, assign41940_body13_e56237_d_n5, assign41940_body13_e56237_d_n6, assign41940_body13_e56237_d_n7, assign41940_body13_e56237_d_n8, assign41940_body13_e56237_d_n9, assign41940_body13_e56237_d_n10, assign41940_body13_e56237_d_n11, assign41940_body13_e56237_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41940_body13_e56233: f64 = (-locals.var_pf1);
        let assign41940_body13_e56235: f64 = (assign41940_body13_e56233 / locals.var_pf11);
        (assign41940_body13_e56235, ((((-locals.var_pf1_dn0) * locals.var_pf11) - (assign41940_body13_e56233 * locals.var_pf11_dn0)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn2) * locals.var_pf11) - (assign41940_body13_e56233 * locals.var_pf11_dn2)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn4) * locals.var_pf11) - (assign41940_body13_e56233 * locals.var_pf11_dn4)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn5) * locals.var_pf11) - (assign41940_body13_e56233 * locals.var_pf11_dn5)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn6) * locals.var_pf11) - (assign41940_body13_e56233 * locals.var_pf11_dn6)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn7) * locals.var_pf11) - (assign41940_body13_e56233 * locals.var_pf11_dn7)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn8) * locals.var_pf11) - (assign41940_body13_e56233 * locals.var_pf11_dn8)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn9) * locals.var_pf11) - (assign41940_body13_e56233 * locals.var_pf11_dn9)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn10) * locals.var_pf11) - (assign41940_body13_e56233 * locals.var_pf11_dn10)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn11) * locals.var_pf11) - (assign41940_body13_e56233 * locals.var_pf11_dn11)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn14) * locals.var_pf11) - (assign41940_body13_e56233 * locals.var_pf11_dn14)) / (locals.var_pf11 * locals.var_pf11)),)
    } else {
        (locals.var_dps, locals.var_dps_dn0, locals.var_dps_dn2, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn11, locals.var_dps_dn14,)
    }
};
            locals.var_dps = assign41940_body13_e56237;
            locals.var_dps_dn0 = assign41940_body13_e56237_d_n0;
            locals.var_dps_dn2 = assign41940_body13_e56237_d_n2;
            locals.var_dps_dn4 = assign41940_body13_e56237_d_n4;
            locals.var_dps_dn5 = assign41940_body13_e56237_d_n5;
            locals.var_dps_dn6 = assign41940_body13_e56237_d_n6;
            locals.var_dps_dn7 = assign41940_body13_e56237_d_n7;
            locals.var_dps_dn8 = assign41940_body13_e56237_d_n8;
            locals.var_dps_dn9 = assign41940_body13_e56237_d_n9;
            locals.var_dps_dn10 = assign41940_body13_e56237_d_n10;
            locals.var_dps_dn11 = assign41940_body13_e56237_d_n11;
            locals.var_dps_dn14 = assign41940_body13_e56237_d_n14;
            locals.var_dps_rv = 0.0;
            let assign41940_body14_e56239: f64 = (locals.var_dps).abs();
            let assign41940_body14_e56242: f64 = (1e-10 * 100.0);
            let assign41940_body14_e56243: f64 = if assign41940_body14_e56239 < assign41940_body14_e56242 { 1.0 } else { 0.0 };
            locals.var_guard1046 = assign41940_body14_e56243;
            locals.var_guard1046_rv = 0.0;
            let (assign41940_body15_e56260,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1046 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign41940_body15_e56260;
            locals.var_flg_conv_rv = 0.0;
            let assign41940_body16_e56263: f64 = if locals.var_dps > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1047 = assign41940_body16_e56263;
            locals.var_guard1047_rv = 0.0;
            let (assign41940_body17_e56283, assign41940_body17_e56283_d_n0, assign41940_body17_e56283_d_n2, assign41940_body17_e56283_d_n4, assign41940_body17_e56283_d_n5, assign41940_body17_e56283_d_n6, assign41940_body17_e56283_d_n7, assign41940_body17_e56283_d_n8, assign41940_body17_e56283_d_n9, assign41940_body17_e56283_d_n10, assign41940_body17_e56283_d_n11, assign41940_body17_e56283_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1046 == 0.0)) && (locals.var_guard1047 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps, locals.var_dps_dn0, locals.var_dps_dn2, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn11, locals.var_dps_dn14,)
    }
};
            locals.var_dps = assign41940_body17_e56283;
            locals.var_dps_dn0 = assign41940_body17_e56283_d_n0;
            locals.var_dps_dn2 = assign41940_body17_e56283_d_n2;
            locals.var_dps_dn4 = assign41940_body17_e56283_d_n4;
            locals.var_dps_dn5 = assign41940_body17_e56283_d_n5;
            locals.var_dps_dn6 = assign41940_body17_e56283_d_n6;
            locals.var_dps_dn7 = assign41940_body17_e56283_d_n7;
            locals.var_dps_dn8 = assign41940_body17_e56283_d_n8;
            locals.var_dps_dn9 = assign41940_body17_e56283_d_n9;
            locals.var_dps_dn10 = assign41940_body17_e56283_d_n10;
            locals.var_dps_dn11 = assign41940_body17_e56283_d_n11;
            locals.var_dps_dn14 = assign41940_body17_e56283_d_n14;
            locals.var_dps_rv = 0.0;
            let assign41940_body18_e56286: f64 = (-0.1);
            let assign41940_body18_e56287: f64 = if locals.var_dps < assign41940_body18_e56286 { 1.0 } else { 0.0 };
            locals.var_guard1048 = assign41940_body18_e56287;
            locals.var_guard1048_rv = 0.0;
            let (assign41940_body19_e56311, assign41940_body19_e56311_d_n0, assign41940_body19_e56311_d_n2, assign41940_body19_e56311_d_n4, assign41940_body19_e56311_d_n5, assign41940_body19_e56311_d_n6, assign41940_body19_e56311_d_n7, assign41940_body19_e56311_d_n8, assign41940_body19_e56311_d_n9, assign41940_body19_e56311_d_n10, assign41940_body19_e56311_d_n11, assign41940_body19_e56311_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1046 == 0.0)) && (locals.var_guard1047 == 0.0)) && (locals.var_guard1048 != 0.0)) {
        let assign41940_body19_e56309: f64 = (-0.1);
        (assign41940_body19_e56309, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps, locals.var_dps_dn0, locals.var_dps_dn2, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn11, locals.var_dps_dn14,)
    }
};
            locals.var_dps = assign41940_body19_e56311;
            locals.var_dps_dn0 = assign41940_body19_e56311_d_n0;
            locals.var_dps_dn2 = assign41940_body19_e56311_d_n2;
            locals.var_dps_dn4 = assign41940_body19_e56311_d_n4;
            locals.var_dps_dn5 = assign41940_body19_e56311_d_n5;
            locals.var_dps_dn6 = assign41940_body19_e56311_d_n6;
            locals.var_dps_dn7 = assign41940_body19_e56311_d_n7;
            locals.var_dps_dn8 = assign41940_body19_e56311_d_n8;
            locals.var_dps_dn9 = assign41940_body19_e56311_d_n9;
            locals.var_dps_dn10 = assign41940_body19_e56311_d_n10;
            locals.var_dps_dn11 = assign41940_body19_e56311_d_n11;
            locals.var_dps_dn14 = assign41940_body19_e56311_d_n14;
            locals.var_dps_rv = 0.0;
            let (assign41940_body20_e56328, assign41940_body20_e56328_d_n0, assign41940_body20_e56328_d_n2, assign41940_body20_e56328_d_n4, assign41940_body20_e56328_d_n5, assign41940_body20_e56328_d_n6, assign41940_body20_e56328_d_n7, assign41940_body20_e56328_d_n8, assign41940_body20_e56328_d_n9, assign41940_body20_e56328_d_n10, assign41940_body20_e56328_d_n11, assign41940_body20_e56328_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41940_body20_e56326: f64 = (locals.var_ps0dep + locals.var_dps);
        (assign41940_body20_e56326, (locals.var_ps0dep_dn0 + locals.var_dps_dn0), (locals.var_ps0dep_dn2 + locals.var_dps_dn2), (locals.var_ps0dep_dn4 + locals.var_dps_dn4), (locals.var_ps0dep_dn5 + locals.var_dps_dn5), (locals.var_ps0dep_dn6 + locals.var_dps_dn6), (locals.var_ps0dep_dn7 + locals.var_dps_dn7), (locals.var_ps0dep_dn8 + locals.var_dps_dn8), (locals.var_ps0dep_dn9 + locals.var_dps_dn9), (locals.var_ps0dep_dn10 + locals.var_dps_dn10), (locals.var_ps0dep_dn11 + locals.var_dps_dn11), (locals.var_ps0dep_dn14 + locals.var_dps_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
            locals.var_ps0dep = assign41940_body20_e56328;
            locals.var_ps0dep_dn0 = assign41940_body20_e56328_d_n0;
            locals.var_ps0dep_dn2 = assign41940_body20_e56328_d_n2;
            locals.var_ps0dep_dn4 = assign41940_body20_e56328_d_n4;
            locals.var_ps0dep_dn5 = assign41940_body20_e56328_d_n5;
            locals.var_ps0dep_dn6 = assign41940_body20_e56328_d_n6;
            locals.var_ps0dep_dn7 = assign41940_body20_e56328_d_n7;
            locals.var_ps0dep_dn8 = assign41940_body20_e56328_d_n8;
            locals.var_ps0dep_dn9 = assign41940_body20_e56328_d_n9;
            locals.var_ps0dep_dn10 = assign41940_body20_e56328_d_n10;
            locals.var_ps0dep_dn11 = assign41940_body20_e56328_d_n11;
            locals.var_ps0dep_dn14 = assign41940_body20_e56328_d_n14;
            locals.var_ps0dep_rv = 0.0;
            let (assign41940_body21_e56342,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign41940_body21_e56340: f64 = (locals.var_lp_s0 + 1.0);
        (assign41940_body21_e56340,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign41940_body21_e56342;
            locals.var_lp_s0_rv = 0.0;
        }

        let (assign41960_e56358, assign41960_e56358_d_n0, assign41960_e56358_d_n2, assign41960_e56358_d_n4, assign41960_e56358_d_n5, assign41960_e56358_d_n6, assign41960_e56358_d_n7, assign41960_e56358_d_n8, assign41960_e56358_d_n9, assign41960_e56358_d_n10, assign41960_e56358_d_n11, assign41960_e56358_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign41960_e56356: f64 = (-locals.var_ps0dep);
        (assign41960_e56356, (-locals.var_ps0dep_dn0), (-locals.var_ps0dep_dn2), (-locals.var_ps0dep_dn4), (-locals.var_ps0dep_dn5), (-locals.var_ps0dep_dn6), (-locals.var_ps0dep_dn7), (-locals.var_ps0dep_dn8), (-locals.var_ps0dep_dn9), (-locals.var_ps0dep_dn10), (-locals.var_ps0dep_dn11), (-locals.var_ps0dep_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign41960_e56358;
        locals.var_ps0dep_dn0 = assign41960_e56358_d_n0;
        locals.var_ps0dep_dn2 = assign41960_e56358_d_n2;
        locals.var_ps0dep_dn4 = assign41960_e56358_d_n4;
        locals.var_ps0dep_dn5 = assign41960_e56358_d_n5;
        locals.var_ps0dep_dn6 = assign41960_e56358_d_n6;
        locals.var_ps0dep_dn7 = assign41960_e56358_d_n7;
        locals.var_ps0dep_dn8 = assign41960_e56358_d_n8;
        locals.var_ps0dep_dn9 = assign41960_e56358_d_n9;
        locals.var_ps0dep_dn10 = assign41960_e56358_d_n10;
        locals.var_ps0dep_dn11 = assign41960_e56358_d_n11;
        locals.var_ps0dep_dn14 = assign41960_e56358_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign41970_e56378, assign41970_e56378_d_n0, assign41970_e56378_d_n2, assign41970_e56378_d_n4, assign41970_e56378_d_n5, assign41970_e56378_d_n6, assign41970_e56378_d_n7, assign41970_e56378_d_n8, assign41970_e56378_d_n9, assign41970_e56378_d_n10, assign41970_e56378_d_n11, assign41970_e56378_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign41970_e56370: f64 = (locals.var_q_ndepm__blk907 * locals.var_tnp);
        let assign41970_e56372: f64 = (assign41970_e56370 * locals.var_tnp);
        let assign41970_e56374: f64 = (assign41970_e56372 / 2.0);
        let assign41970_e56376: f64 = (assign41970_e56374 / 1.034943e-10);
        (assign41970_e56376, ((((((locals.var_q_ndepm__blk907_dn0 * locals.var_tnp) + (locals.var_q_ndepm__blk907 * locals.var_tnp_dn0)) * locals.var_tnp) + (assign41970_e56370 * locals.var_tnp_dn0)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk907_dn2 * locals.var_tnp) + (locals.var_q_ndepm__blk907 * locals.var_tnp_dn2)) * locals.var_tnp) + (assign41970_e56370 * locals.var_tnp_dn2)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk907_dn4 * locals.var_tnp) + (locals.var_q_ndepm__blk907 * locals.var_tnp_dn4)) * locals.var_tnp) + (assign41970_e56370 * locals.var_tnp_dn4)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk907_dn5 * locals.var_tnp) + (locals.var_q_ndepm__blk907 * locals.var_tnp_dn5)) * locals.var_tnp) + (assign41970_e56370 * locals.var_tnp_dn5)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk907_dn6 * locals.var_tnp) + (locals.var_q_ndepm__blk907 * locals.var_tnp_dn6)) * locals.var_tnp) + (assign41970_e56370 * locals.var_tnp_dn6)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk907_dn7 * locals.var_tnp) + (locals.var_q_ndepm__blk907 * locals.var_tnp_dn7)) * locals.var_tnp) + (assign41970_e56370 * locals.var_tnp_dn7)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk907_dn8 * locals.var_tnp) + (locals.var_q_ndepm__blk907 * locals.var_tnp_dn8)) * locals.var_tnp) + (assign41970_e56370 * locals.var_tnp_dn8)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk907_dn9 * locals.var_tnp) + (locals.var_q_ndepm__blk907 * locals.var_tnp_dn9)) * locals.var_tnp) + (assign41970_e56370 * locals.var_tnp_dn9)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk907_dn10 * locals.var_tnp) + (locals.var_q_ndepm__blk907 * locals.var_tnp_dn10)) * locals.var_tnp) + (assign41970_e56370 * locals.var_tnp_dn10)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk907_dn11 * locals.var_tnp) + (locals.var_q_ndepm__blk907 * locals.var_tnp_dn11)) * locals.var_tnp) + (assign41970_e56370 * locals.var_tnp_dn11)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk907_dn14 * locals.var_tnp) + (locals.var_q_ndepm__blk907 * locals.var_tnp_dn14)) * locals.var_tnp) + (assign41970_e56370 * locals.var_tnp_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1028, locals.var_dphi_sb__blk1028_dn0, locals.var_dphi_sb__blk1028_dn2, locals.var_dphi_sb__blk1028_dn4, locals.var_dphi_sb__blk1028_dn5, locals.var_dphi_sb__blk1028_dn6, locals.var_dphi_sb__blk1028_dn7, locals.var_dphi_sb__blk1028_dn8, locals.var_dphi_sb__blk1028_dn9, locals.var_dphi_sb__blk1028_dn10, locals.var_dphi_sb__blk1028_dn11, locals.var_dphi_sb__blk1028_dn14,)
    }
};
        locals.var_dphi_sb__blk1028 = assign41970_e56378;
        locals.var_dphi_sb__blk1028_dn0 = assign41970_e56378_d_n0;
        locals.var_dphi_sb__blk1028_dn2 = assign41970_e56378_d_n2;
        locals.var_dphi_sb__blk1028_dn4 = assign41970_e56378_d_n4;
        locals.var_dphi_sb__blk1028_dn5 = assign41970_e56378_d_n5;
        locals.var_dphi_sb__blk1028_dn6 = assign41970_e56378_d_n6;
        locals.var_dphi_sb__blk1028_dn7 = assign41970_e56378_d_n7;
        locals.var_dphi_sb__blk1028_dn8 = assign41970_e56378_d_n8;
        locals.var_dphi_sb__blk1028_dn9 = assign41970_e56378_d_n9;
        locals.var_dphi_sb__blk1028_dn10 = assign41970_e56378_d_n10;
        locals.var_dphi_sb__blk1028_dn11 = assign41970_e56378_d_n11;
        locals.var_dphi_sb__blk1028_dn14 = assign41970_e56378_d_n14;
        locals.var_dphi_sb__blk1028_rv = 0.0;

        let (assign41980_e56397, assign41980_e56397_d_n0, assign41980_e56397_d_n2, assign41980_e56397_d_n4, assign41980_e56397_d_n5, assign41980_e56397_d_n6, assign41980_e56397_d_n7, assign41980_e56397_d_n8, assign41980_e56397_d_n9, assign41980_e56397_d_n10, assign41980_e56397_d_n11, assign41980_e56397_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign41980_e56391: f64 = (2.0 * locals.var_beta);
        let assign41980_e56393: f64 = (assign41980_e56391 * locals.var_dphi_sb__blk1028);
        let assign41980_e56394: f64 = (assign41980_e56393).sqrt();
        let assign41980_e56395: f64 = (p.p394 * assign41980_e56394);
        (assign41980_e56395, (p.p394 * ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1028) + (assign41980_e56391 * locals.var_dphi_sb__blk1028_dn0)) / (2.0 * assign41980_e56394))), (p.p394 * ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1028) + (assign41980_e56391 * locals.var_dphi_sb__blk1028_dn2)) / (2.0 * assign41980_e56394))), (p.p394 * ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1028) + (assign41980_e56391 * locals.var_dphi_sb__blk1028_dn4)) / (2.0 * assign41980_e56394))), (p.p394 * ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1028) + (assign41980_e56391 * locals.var_dphi_sb__blk1028_dn5)) / (2.0 * assign41980_e56394))), (p.p394 * ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1028) + (assign41980_e56391 * locals.var_dphi_sb__blk1028_dn6)) / (2.0 * assign41980_e56394))), (p.p394 * ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1028) + (assign41980_e56391 * locals.var_dphi_sb__blk1028_dn7)) / (2.0 * assign41980_e56394))), (p.p394 * ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1028) + (assign41980_e56391 * locals.var_dphi_sb__blk1028_dn8)) / (2.0 * assign41980_e56394))), (p.p394 * ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1028) + (assign41980_e56391 * locals.var_dphi_sb__blk1028_dn9)) / (2.0 * assign41980_e56394))), (p.p394 * ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1028) + (assign41980_e56391 * locals.var_dphi_sb__blk1028_dn10)) / (2.0 * assign41980_e56394))), (p.p394 * ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb__blk1028) + (assign41980_e56391 * locals.var_dphi_sb__blk1028_dn11)) / (2.0 * assign41980_e56394))), (p.p394 * ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb__blk1028) + (assign41980_e56391 * locals.var_dphi_sb__blk1028_dn14)) / (2.0 * assign41980_e56394))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41980_e56397;
        locals.var_t0_dn0 = assign41980_e56397_d_n0;
        locals.var_t0_dn2 = assign41980_e56397_d_n2;
        locals.var_t0_dn4 = assign41980_e56397_d_n4;
        locals.var_t0_dn5 = assign41980_e56397_d_n5;
        locals.var_t0_dn6 = assign41980_e56397_d_n6;
        locals.var_t0_dn7 = assign41980_e56397_d_n7;
        locals.var_t0_dn8 = assign41980_e56397_d_n8;
        locals.var_t0_dn9 = assign41980_e56397_d_n9;
        locals.var_t0_dn10 = assign41980_e56397_d_n10;
        locals.var_t0_dn11 = assign41980_e56397_d_n11;
        locals.var_t0_dn14 = assign41980_e56397_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41990_e56416, assign41990_e56416_d_n0, assign41990_e56416_d_n2, assign41990_e56416_d_n4, assign41990_e56416_d_n5, assign41990_e56416_d_n6, assign41990_e56416_d_n7, assign41990_e56416_d_n8, assign41990_e56416_d_n9, assign41990_e56416_d_n10, assign41990_e56416_d_n11, assign41990_e56416_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign41990_e56408: f64 = (locals.var_t0).exp();
        let assign41990_e56410: f64 = (-locals.var_t0);
        let assign41990_e56411: f64 = (assign41990_e56410).exp();
        let assign41990_e56412: f64 = (assign41990_e56408 + assign41990_e56411);
        let assign41990_e56414: f64 = (assign41990_e56412 / 2.0);
        (assign41990_e56414, (((assign41990_e56408 * locals.var_t0_dn0) + (assign41990_e56411 * (-locals.var_t0_dn0))) / 2.0), (((assign41990_e56408 * locals.var_t0_dn2) + (assign41990_e56411 * (-locals.var_t0_dn2))) / 2.0), (((assign41990_e56408 * locals.var_t0_dn4) + (assign41990_e56411 * (-locals.var_t0_dn4))) / 2.0), (((assign41990_e56408 * locals.var_t0_dn5) + (assign41990_e56411 * (-locals.var_t0_dn5))) / 2.0), (((assign41990_e56408 * locals.var_t0_dn6) + (assign41990_e56411 * (-locals.var_t0_dn6))) / 2.0), (((assign41990_e56408 * locals.var_t0_dn7) + (assign41990_e56411 * (-locals.var_t0_dn7))) / 2.0), (((assign41990_e56408 * locals.var_t0_dn8) + (assign41990_e56411 * (-locals.var_t0_dn8))) / 2.0), (((assign41990_e56408 * locals.var_t0_dn9) + (assign41990_e56411 * (-locals.var_t0_dn9))) / 2.0), (((assign41990_e56408 * locals.var_t0_dn10) + (assign41990_e56411 * (-locals.var_t0_dn10))) / 2.0), (((assign41990_e56408 * locals.var_t0_dn11) + (assign41990_e56411 * (-locals.var_t0_dn11))) / 2.0), (((assign41990_e56408 * locals.var_t0_dn14) + (assign41990_e56411 * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign41990_e56416;
        locals.var_t1_dn0 = assign41990_e56416_d_n0;
        locals.var_t1_dn2 = assign41990_e56416_d_n2;
        locals.var_t1_dn4 = assign41990_e56416_d_n4;
        locals.var_t1_dn5 = assign41990_e56416_d_n5;
        locals.var_t1_dn6 = assign41990_e56416_d_n6;
        locals.var_t1_dn7 = assign41990_e56416_d_n7;
        locals.var_t1_dn8 = assign41990_e56416_d_n8;
        locals.var_t1_dn9 = assign41990_e56416_d_n9;
        locals.var_t1_dn10 = assign41990_e56416_d_n10;
        locals.var_t1_dn11 = assign41990_e56416_d_n11;
        locals.var_t1_dn14 = assign41990_e56416_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_145(
        locals: &mut StampLocals,
    ) {
        let (assign42000_e56431, assign42000_e56431_d_n0, assign42000_e56431_d_n2, assign42000_e56431_d_n4, assign42000_e56431_d_n5, assign42000_e56431_d_n6, assign42000_e56431_d_n7, assign42000_e56431_d_n8, assign42000_e56431_d_n9, assign42000_e56431_d_n10, assign42000_e56431_d_n11, assign42000_e56431_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign42000_e56427: f64 = (locals.var_t1).ln();
        let assign42000_e56429: f64 = (assign42000_e56427 / locals.var_dphi_sb__blk1028);
        (assign42000_e56429, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1028) - (assign42000_e56427 * locals.var_dphi_sb__blk1028_dn0)) / (locals.var_dphi_sb__blk1028 * locals.var_dphi_sb__blk1028)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1028) - (assign42000_e56427 * locals.var_dphi_sb__blk1028_dn2)) / (locals.var_dphi_sb__blk1028 * locals.var_dphi_sb__blk1028)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1028) - (assign42000_e56427 * locals.var_dphi_sb__blk1028_dn4)) / (locals.var_dphi_sb__blk1028 * locals.var_dphi_sb__blk1028)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1028) - (assign42000_e56427 * locals.var_dphi_sb__blk1028_dn5)) / (locals.var_dphi_sb__blk1028 * locals.var_dphi_sb__blk1028)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1028) - (assign42000_e56427 * locals.var_dphi_sb__blk1028_dn6)) / (locals.var_dphi_sb__blk1028 * locals.var_dphi_sb__blk1028)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1028) - (assign42000_e56427 * locals.var_dphi_sb__blk1028_dn7)) / (locals.var_dphi_sb__blk1028 * locals.var_dphi_sb__blk1028)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1028) - (assign42000_e56427 * locals.var_dphi_sb__blk1028_dn8)) / (locals.var_dphi_sb__blk1028 * locals.var_dphi_sb__blk1028)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1028) - (assign42000_e56427 * locals.var_dphi_sb__blk1028_dn9)) / (locals.var_dphi_sb__blk1028 * locals.var_dphi_sb__blk1028)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1028) - (assign42000_e56427 * locals.var_dphi_sb__blk1028_dn10)) / (locals.var_dphi_sb__blk1028 * locals.var_dphi_sb__blk1028)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb__blk1028) - (assign42000_e56427 * locals.var_dphi_sb__blk1028_dn11)) / (locals.var_dphi_sb__blk1028 * locals.var_dphi_sb__blk1028)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb__blk1028) - (assign42000_e56427 * locals.var_dphi_sb__blk1028_dn14)) / (locals.var_dphi_sb__blk1028 * locals.var_dphi_sb__blk1028)),)
    } else {
        (locals.var_c_sb__blk1029, locals.var_c_sb__blk1029_dn0, locals.var_c_sb__blk1029_dn2, locals.var_c_sb__blk1029_dn4, locals.var_c_sb__blk1029_dn5, locals.var_c_sb__blk1029_dn6, locals.var_c_sb__blk1029_dn7, locals.var_c_sb__blk1029_dn8, locals.var_c_sb__blk1029_dn9, locals.var_c_sb__blk1029_dn10, locals.var_c_sb__blk1029_dn11, locals.var_c_sb__blk1029_dn14,)
    }
};
        locals.var_c_sb__blk1029 = assign42000_e56431;
        locals.var_c_sb__blk1029_dn0 = assign42000_e56431_d_n0;
        locals.var_c_sb__blk1029_dn2 = assign42000_e56431_d_n2;
        locals.var_c_sb__blk1029_dn4 = assign42000_e56431_d_n4;
        locals.var_c_sb__blk1029_dn5 = assign42000_e56431_d_n5;
        locals.var_c_sb__blk1029_dn6 = assign42000_e56431_d_n6;
        locals.var_c_sb__blk1029_dn7 = assign42000_e56431_d_n7;
        locals.var_c_sb__blk1029_dn8 = assign42000_e56431_d_n8;
        locals.var_c_sb__blk1029_dn9 = assign42000_e56431_d_n9;
        locals.var_c_sb__blk1029_dn10 = assign42000_e56431_d_n10;
        locals.var_c_sb__blk1029_dn11 = assign42000_e56431_d_n11;
        locals.var_c_sb__blk1029_dn14 = assign42000_e56431_d_n14;
        locals.var_c_sb__blk1029_rv = 0.0;

        let (assign42010_e56445, assign42010_e56445_d_n0, assign42010_e56445_d_n2, assign42010_e56445_d_n4, assign42010_e56445_d_n5, assign42010_e56445_d_n6, assign42010_e56445_d_n7, assign42010_e56445_d_n8, assign42010_e56445_d_n9, assign42010_e56445_d_n10, assign42010_e56445_d_n11, assign42010_e56445_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign42010_e56443: f64 = (locals.var_c_sb__blk1029 * locals.var_ps0dep);
        (assign42010_e56443, ((locals.var_c_sb__blk1029_dn0 * locals.var_ps0dep) + (locals.var_c_sb__blk1029 * locals.var_ps0dep_dn0)), ((locals.var_c_sb__blk1029_dn2 * locals.var_ps0dep) + (locals.var_c_sb__blk1029 * locals.var_ps0dep_dn2)), ((locals.var_c_sb__blk1029_dn4 * locals.var_ps0dep) + (locals.var_c_sb__blk1029 * locals.var_ps0dep_dn4)), ((locals.var_c_sb__blk1029_dn5 * locals.var_ps0dep) + (locals.var_c_sb__blk1029 * locals.var_ps0dep_dn5)), ((locals.var_c_sb__blk1029_dn6 * locals.var_ps0dep) + (locals.var_c_sb__blk1029 * locals.var_ps0dep_dn6)), ((locals.var_c_sb__blk1029_dn7 * locals.var_ps0dep) + (locals.var_c_sb__blk1029 * locals.var_ps0dep_dn7)), ((locals.var_c_sb__blk1029_dn8 * locals.var_ps0dep) + (locals.var_c_sb__blk1029 * locals.var_ps0dep_dn8)), ((locals.var_c_sb__blk1029_dn9 * locals.var_ps0dep) + (locals.var_c_sb__blk1029 * locals.var_ps0dep_dn9)), ((locals.var_c_sb__blk1029_dn10 * locals.var_ps0dep) + (locals.var_c_sb__blk1029 * locals.var_ps0dep_dn10)), ((locals.var_c_sb__blk1029_dn11 * locals.var_ps0dep) + (locals.var_c_sb__blk1029 * locals.var_ps0dep_dn11)), ((locals.var_c_sb__blk1029_dn14 * locals.var_ps0dep) + (locals.var_c_sb__blk1029 * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign42010_e56445;
        locals.var_tx_dn0 = assign42010_e56445_d_n0;
        locals.var_tx_dn2 = assign42010_e56445_d_n2;
        locals.var_tx_dn4 = assign42010_e56445_d_n4;
        locals.var_tx_dn5 = assign42010_e56445_d_n5;
        locals.var_tx_dn6 = assign42010_e56445_d_n6;
        locals.var_tx_dn7 = assign42010_e56445_d_n7;
        locals.var_tx_dn8 = assign42010_e56445_d_n8;
        locals.var_tx_dn9 = assign42010_e56445_d_n9;
        locals.var_tx_dn10 = assign42010_e56445_d_n10;
        locals.var_tx_dn11 = assign42010_e56445_d_n11;
        locals.var_tx_dn14 = assign42010_e56445_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign42020_e56461, assign42020_e56461_d_n0, assign42020_e56461_d_n2, assign42020_e56461_d_n4, assign42020_e56461_d_n5, assign42020_e56461_d_n6, assign42020_e56461_d_n7, assign42020_e56461_d_n8, assign42020_e56461_d_n9, assign42020_e56461_d_n10, assign42020_e56461_d_n11, assign42020_e56461_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let assign42020_e56456: f64 = (-locals.var_c_sb__blk1029);
        let assign42020_e56458: f64 = (assign42020_e56456 * locals.var_dphi_sb__blk1028);
        let assign42020_e56459: f64 = (assign42020_e56458).exp();
        (assign42020_e56459, (assign42020_e56459 * (((-locals.var_c_sb__blk1029_dn0) * locals.var_dphi_sb__blk1028) + (assign42020_e56456 * locals.var_dphi_sb__blk1028_dn0))), (assign42020_e56459 * (((-locals.var_c_sb__blk1029_dn2) * locals.var_dphi_sb__blk1028) + (assign42020_e56456 * locals.var_dphi_sb__blk1028_dn2))), (assign42020_e56459 * (((-locals.var_c_sb__blk1029_dn4) * locals.var_dphi_sb__blk1028) + (assign42020_e56456 * locals.var_dphi_sb__blk1028_dn4))), (assign42020_e56459 * (((-locals.var_c_sb__blk1029_dn5) * locals.var_dphi_sb__blk1028) + (assign42020_e56456 * locals.var_dphi_sb__blk1028_dn5))), (assign42020_e56459 * (((-locals.var_c_sb__blk1029_dn6) * locals.var_dphi_sb__blk1028) + (assign42020_e56456 * locals.var_dphi_sb__blk1028_dn6))), (assign42020_e56459 * (((-locals.var_c_sb__blk1029_dn7) * locals.var_dphi_sb__blk1028) + (assign42020_e56456 * locals.var_dphi_sb__blk1028_dn7))), (assign42020_e56459 * (((-locals.var_c_sb__blk1029_dn8) * locals.var_dphi_sb__blk1028) + (assign42020_e56456 * locals.var_dphi_sb__blk1028_dn8))), (assign42020_e56459 * (((-locals.var_c_sb__blk1029_dn9) * locals.var_dphi_sb__blk1028) + (assign42020_e56456 * locals.var_dphi_sb__blk1028_dn9))), (assign42020_e56459 * (((-locals.var_c_sb__blk1029_dn10) * locals.var_dphi_sb__blk1028) + (assign42020_e56456 * locals.var_dphi_sb__blk1028_dn10))), (assign42020_e56459 * (((-locals.var_c_sb__blk1029_dn11) * locals.var_dphi_sb__blk1028) + (assign42020_e56456 * locals.var_dphi_sb__blk1028_dn11))), (assign42020_e56459 * (((-locals.var_c_sb__blk1029_dn14) * locals.var_dphi_sb__blk1028) + (assign42020_e56456 * locals.var_dphi_sb__blk1028_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign42020_e56461;
        locals.var_t0_dn0 = assign42020_e56461_d_n0;
        locals.var_t0_dn2 = assign42020_e56461_d_n2;
        locals.var_t0_dn4 = assign42020_e56461_d_n4;
        locals.var_t0_dn5 = assign42020_e56461_d_n5;
        locals.var_t0_dn6 = assign42020_e56461_d_n6;
        locals.var_t0_dn7 = assign42020_e56461_d_n7;
        locals.var_t0_dn8 = assign42020_e56461_d_n8;
        locals.var_t0_dn9 = assign42020_e56461_d_n9;
        locals.var_t0_dn10 = assign42020_e56461_d_n10;
        locals.var_t0_dn11 = assign42020_e56461_d_n11;
        locals.var_t0_dn14 = assign42020_e56461_d_n14;
        locals.var_t0_rv = 0.0;

        let assign42030_e56463: f64 = (locals.var_tx).abs();
        let assign42030_e56465: f64 = if assign42030_e56463 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1050 = assign42030_e56465;
        locals.var_guard1050_rv = 0.0;

        let (assign42040_e56482, assign42040_e56482_d_n0, assign42040_e56482_d_n2, assign42040_e56482_d_n4, assign42040_e56482_d_n5, assign42040_e56482_d_n6, assign42040_e56482_d_n7, assign42040_e56482_d_n8, assign42040_e56482_d_n9, assign42040_e56482_d_n10, assign42040_e56482_d_n11, assign42040_e56482_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1050 != 0.0)) {
        let assign42040_e56478: f64 = (locals.var_tx).exp();
        let assign42040_e56480: f64 = (assign42040_e56478 * locals.var_t0);
        (assign42040_e56480, (((assign42040_e56478 * locals.var_tx_dn0) * locals.var_t0) + (assign42040_e56478 * locals.var_t0_dn0)), (((assign42040_e56478 * locals.var_tx_dn2) * locals.var_t0) + (assign42040_e56478 * locals.var_t0_dn2)), (((assign42040_e56478 * locals.var_tx_dn4) * locals.var_t0) + (assign42040_e56478 * locals.var_t0_dn4)), (((assign42040_e56478 * locals.var_tx_dn5) * locals.var_t0) + (assign42040_e56478 * locals.var_t0_dn5)), (((assign42040_e56478 * locals.var_tx_dn6) * locals.var_t0) + (assign42040_e56478 * locals.var_t0_dn6)), (((assign42040_e56478 * locals.var_tx_dn7) * locals.var_t0) + (assign42040_e56478 * locals.var_t0_dn7)), (((assign42040_e56478 * locals.var_tx_dn8) * locals.var_t0) + (assign42040_e56478 * locals.var_t0_dn8)), (((assign42040_e56478 * locals.var_tx_dn9) * locals.var_t0) + (assign42040_e56478 * locals.var_t0_dn9)), (((assign42040_e56478 * locals.var_tx_dn10) * locals.var_t0) + (assign42040_e56478 * locals.var_t0_dn10)), (((assign42040_e56478 * locals.var_tx_dn11) * locals.var_t0) + (assign42040_e56478 * locals.var_t0_dn11)), (((assign42040_e56478 * locals.var_tx_dn14) * locals.var_t0) + (assign42040_e56478 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign42040_e56482;
        locals.var_t1_dn0 = assign42040_e56482_d_n0;
        locals.var_t1_dn2 = assign42040_e56482_d_n2;
        locals.var_t1_dn4 = assign42040_e56482_d_n4;
        locals.var_t1_dn5 = assign42040_e56482_d_n5;
        locals.var_t1_dn6 = assign42040_e56482_d_n6;
        locals.var_t1_dn7 = assign42040_e56482_d_n7;
        locals.var_t1_dn8 = assign42040_e56482_d_n8;
        locals.var_t1_dn9 = assign42040_e56482_d_n9;
        locals.var_t1_dn10 = assign42040_e56482_d_n10;
        locals.var_t1_dn11 = assign42040_e56482_d_n11;
        locals.var_t1_dn14 = assign42040_e56482_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign42050_e56498, assign42050_e56498_d_n0, assign42050_e56498_d_n2, assign42050_e56498_d_n4, assign42050_e56498_d_n5, assign42050_e56498_d_n6, assign42050_e56498_d_n7, assign42050_e56498_d_n8, assign42050_e56498_d_n9, assign42050_e56498_d_n10, assign42050_e56498_d_n11, assign42050_e56498_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1050 != 0.0)) {
        let assign42050_e56496: f64 = (locals.var_t1 - locals.var_t0);
        (assign42050_e56496, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42050_e56498;
        locals.var_t2_dn0 = assign42050_e56498_d_n0;
        locals.var_t2_dn2 = assign42050_e56498_d_n2;
        locals.var_t2_dn4 = assign42050_e56498_d_n4;
        locals.var_t2_dn5 = assign42050_e56498_d_n5;
        locals.var_t2_dn6 = assign42050_e56498_d_n6;
        locals.var_t2_dn7 = assign42050_e56498_d_n7;
        locals.var_t2_dn8 = assign42050_e56498_d_n8;
        locals.var_t2_dn9 = assign42050_e56498_d_n9;
        locals.var_t2_dn10 = assign42050_e56498_d_n10;
        locals.var_t2_dn11 = assign42050_e56498_d_n11;
        locals.var_t2_dn14 = assign42050_e56498_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign42060_e56517, assign42060_e56517_d_n0, assign42060_e56517_d_n2, assign42060_e56517_d_n4, assign42060_e56517_d_n5, assign42060_e56517_d_n6, assign42060_e56517_d_n7, assign42060_e56517_d_n8, assign42060_e56517_d_n9, assign42060_e56517_d_n10, assign42060_e56517_d_n11, assign42060_e56517_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1050 == 0.0)) {
        let assign42060_e56513: f64 = (1.0 + locals.var_tx);
        let assign42060_e56515: f64 = (assign42060_e56513 * locals.var_t0);
        (assign42060_e56515, ((locals.var_tx_dn0 * locals.var_t0) + (assign42060_e56513 * locals.var_t0_dn0)), ((locals.var_tx_dn2 * locals.var_t0) + (assign42060_e56513 * locals.var_t0_dn2)), ((locals.var_tx_dn4 * locals.var_t0) + (assign42060_e56513 * locals.var_t0_dn4)), ((locals.var_tx_dn5 * locals.var_t0) + (assign42060_e56513 * locals.var_t0_dn5)), ((locals.var_tx_dn6 * locals.var_t0) + (assign42060_e56513 * locals.var_t0_dn6)), ((locals.var_tx_dn7 * locals.var_t0) + (assign42060_e56513 * locals.var_t0_dn7)), ((locals.var_tx_dn8 * locals.var_t0) + (assign42060_e56513 * locals.var_t0_dn8)), ((locals.var_tx_dn9 * locals.var_t0) + (assign42060_e56513 * locals.var_t0_dn9)), ((locals.var_tx_dn10 * locals.var_t0) + (assign42060_e56513 * locals.var_t0_dn10)), ((locals.var_tx_dn11 * locals.var_t0) + (assign42060_e56513 * locals.var_t0_dn11)), ((locals.var_tx_dn14 * locals.var_t0) + (assign42060_e56513 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign42060_e56517;
        locals.var_t1_dn0 = assign42060_e56517_d_n0;
        locals.var_t1_dn2 = assign42060_e56517_d_n2;
        locals.var_t1_dn4 = assign42060_e56517_d_n4;
        locals.var_t1_dn5 = assign42060_e56517_d_n5;
        locals.var_t1_dn6 = assign42060_e56517_d_n6;
        locals.var_t1_dn7 = assign42060_e56517_d_n7;
        locals.var_t1_dn8 = assign42060_e56517_d_n8;
        locals.var_t1_dn9 = assign42060_e56517_d_n9;
        locals.var_t1_dn10 = assign42060_e56517_d_n10;
        locals.var_t1_dn11 = assign42060_e56517_d_n11;
        locals.var_t1_dn14 = assign42060_e56517_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign42070_e56540, assign42070_e56540_d_n0, assign42070_e56540_d_n2, assign42070_e56540_d_n4, assign42070_e56540_d_n5, assign42070_e56540_d_n6, assign42070_e56540_d_n7, assign42070_e56540_d_n8, assign42070_e56540_d_n9, assign42070_e56540_d_n10, assign42070_e56540_d_n11, assign42070_e56540_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1050 == 0.0)) {
        let assign42070_e56534: f64 = (locals.var_tx / 2.0);
        let assign42070_e56535: f64 = (1.0 + assign42070_e56534);
        let assign42070_e56536: f64 = (locals.var_tx * assign42070_e56535);
        let assign42070_e56538: f64 = (assign42070_e56536 * locals.var_t0);
        (assign42070_e56538, ((((locals.var_tx_dn0 * assign42070_e56535) + (locals.var_tx * (locals.var_tx_dn0 / 2.0))) * locals.var_t0) + (assign42070_e56536 * locals.var_t0_dn0)), ((((locals.var_tx_dn2 * assign42070_e56535) + (locals.var_tx * (locals.var_tx_dn2 / 2.0))) * locals.var_t0) + (assign42070_e56536 * locals.var_t0_dn2)), ((((locals.var_tx_dn4 * assign42070_e56535) + (locals.var_tx * (locals.var_tx_dn4 / 2.0))) * locals.var_t0) + (assign42070_e56536 * locals.var_t0_dn4)), ((((locals.var_tx_dn5 * assign42070_e56535) + (locals.var_tx * (locals.var_tx_dn5 / 2.0))) * locals.var_t0) + (assign42070_e56536 * locals.var_t0_dn5)), ((((locals.var_tx_dn6 * assign42070_e56535) + (locals.var_tx * (locals.var_tx_dn6 / 2.0))) * locals.var_t0) + (assign42070_e56536 * locals.var_t0_dn6)), ((((locals.var_tx_dn7 * assign42070_e56535) + (locals.var_tx * (locals.var_tx_dn7 / 2.0))) * locals.var_t0) + (assign42070_e56536 * locals.var_t0_dn7)), ((((locals.var_tx_dn8 * assign42070_e56535) + (locals.var_tx * (locals.var_tx_dn8 / 2.0))) * locals.var_t0) + (assign42070_e56536 * locals.var_t0_dn8)), ((((locals.var_tx_dn9 * assign42070_e56535) + (locals.var_tx * (locals.var_tx_dn9 / 2.0))) * locals.var_t0) + (assign42070_e56536 * locals.var_t0_dn9)), ((((locals.var_tx_dn10 * assign42070_e56535) + (locals.var_tx * (locals.var_tx_dn10 / 2.0))) * locals.var_t0) + (assign42070_e56536 * locals.var_t0_dn10)), ((((locals.var_tx_dn11 * assign42070_e56535) + (locals.var_tx * (locals.var_tx_dn11 / 2.0))) * locals.var_t0) + (assign42070_e56536 * locals.var_t0_dn11)), ((((locals.var_tx_dn14 * assign42070_e56535) + (locals.var_tx * (locals.var_tx_dn14 / 2.0))) * locals.var_t0) + (assign42070_e56536 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42070_e56540;
        locals.var_t2_dn0 = assign42070_e56540_d_n0;
        locals.var_t2_dn2 = assign42070_e56540_d_n2;
        locals.var_t2_dn4 = assign42070_e56540_d_n4;
        locals.var_t2_dn5 = assign42070_e56540_d_n5;
        locals.var_t2_dn6 = assign42070_e56540_d_n6;
        locals.var_t2_dn7 = assign42070_e56540_d_n7;
        locals.var_t2_dn8 = assign42070_e56540_d_n8;
        locals.var_t2_dn9 = assign42070_e56540_d_n9;
        locals.var_t2_dn10 = assign42070_e56540_d_n10;
        locals.var_t2_dn11 = assign42070_e56540_d_n11;
        locals.var_t2_dn14 = assign42070_e56540_d_n14;
        locals.var_t2_rv = 0.0;

        let assign42080_e56542: f64 = (locals.var_t2).abs();
        let assign42080_e56544: f64 = if assign42080_e56542 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1051 = assign42080_e56544;
        locals.var_guard1051_rv = 0.0;

        let (assign42090_e56563, assign42090_e56563_d_n0, assign42090_e56563_d_n2, assign42090_e56563_d_n4, assign42090_e56563_d_n5, assign42090_e56563_d_n6, assign42090_e56563_d_n7, assign42090_e56563_d_n8, assign42090_e56563_d_n9, assign42090_e56563_d_n10, assign42090_e56563_d_n11, assign42090_e56563_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1051 != 0.0)) {
        let assign42090_e56558: f64 = (1.0 + locals.var_t2);
        let assign42090_e56559: f64 = (assign42090_e56558).ln();
        let assign42090_e56561: f64 = (assign42090_e56559 / locals.var_c_sb__blk1029);
        (assign42090_e56561, ((((locals.var_t2_dn0 / assign42090_e56558) * locals.var_c_sb__blk1029) - (assign42090_e56559 * locals.var_c_sb__blk1029_dn0)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), ((((locals.var_t2_dn2 / assign42090_e56558) * locals.var_c_sb__blk1029) - (assign42090_e56559 * locals.var_c_sb__blk1029_dn2)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), ((((locals.var_t2_dn4 / assign42090_e56558) * locals.var_c_sb__blk1029) - (assign42090_e56559 * locals.var_c_sb__blk1029_dn4)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), ((((locals.var_t2_dn5 / assign42090_e56558) * locals.var_c_sb__blk1029) - (assign42090_e56559 * locals.var_c_sb__blk1029_dn5)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), ((((locals.var_t2_dn6 / assign42090_e56558) * locals.var_c_sb__blk1029) - (assign42090_e56559 * locals.var_c_sb__blk1029_dn6)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), ((((locals.var_t2_dn7 / assign42090_e56558) * locals.var_c_sb__blk1029) - (assign42090_e56559 * locals.var_c_sb__blk1029_dn7)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), ((((locals.var_t2_dn8 / assign42090_e56558) * locals.var_c_sb__blk1029) - (assign42090_e56559 * locals.var_c_sb__blk1029_dn8)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), ((((locals.var_t2_dn9 / assign42090_e56558) * locals.var_c_sb__blk1029) - (assign42090_e56559 * locals.var_c_sb__blk1029_dn9)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), ((((locals.var_t2_dn10 / assign42090_e56558) * locals.var_c_sb__blk1029) - (assign42090_e56559 * locals.var_c_sb__blk1029_dn10)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), ((((locals.var_t2_dn11 / assign42090_e56558) * locals.var_c_sb__blk1029) - (assign42090_e56559 * locals.var_c_sb__blk1029_dn11)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), ((((locals.var_t2_dn14 / assign42090_e56558) * locals.var_c_sb__blk1029) - (assign42090_e56559 * locals.var_c_sb__blk1029_dn14)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)),)
    } else {
        (locals.var_pb0dep, locals.var_pb0dep_dn0, locals.var_pb0dep_dn2, locals.var_pb0dep_dn4, locals.var_pb0dep_dn5, locals.var_pb0dep_dn6, locals.var_pb0dep_dn7, locals.var_pb0dep_dn8, locals.var_pb0dep_dn9, locals.var_pb0dep_dn10, locals.var_pb0dep_dn11, locals.var_pb0dep_dn14,)
    }
};
        locals.var_pb0dep = assign42090_e56563;
        locals.var_pb0dep_dn0 = assign42090_e56563_d_n0;
        locals.var_pb0dep_dn2 = assign42090_e56563_d_n2;
        locals.var_pb0dep_dn4 = assign42090_e56563_d_n4;
        locals.var_pb0dep_dn5 = assign42090_e56563_d_n5;
        locals.var_pb0dep_dn6 = assign42090_e56563_d_n6;
        locals.var_pb0dep_dn7 = assign42090_e56563_d_n7;
        locals.var_pb0dep_dn8 = assign42090_e56563_d_n8;
        locals.var_pb0dep_dn9 = assign42090_e56563_d_n9;
        locals.var_pb0dep_dn10 = assign42090_e56563_d_n10;
        locals.var_pb0dep_dn11 = assign42090_e56563_d_n11;
        locals.var_pb0dep_dn14 = assign42090_e56563_d_n14;
        locals.var_pb0dep_rv = 0.0;

        let (assign42100_e56580, assign42100_e56580_d_n0, assign42100_e56580_d_n2, assign42100_e56580_d_n4, assign42100_e56580_d_n5, assign42100_e56580_d_n6, assign42100_e56580_d_n7, assign42100_e56580_d_n8, assign42100_e56580_d_n9, assign42100_e56580_d_n10, assign42100_e56580_d_n11, assign42100_e56580_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1051 == 0.0)) {
        let assign42100_e56578: f64 = (locals.var_t2 / locals.var_c_sb__blk1029);
        (assign42100_e56578, (((locals.var_t2_dn0 * locals.var_c_sb__blk1029) - (locals.var_t2 * locals.var_c_sb__blk1029_dn0)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), (((locals.var_t2_dn2 * locals.var_c_sb__blk1029) - (locals.var_t2 * locals.var_c_sb__blk1029_dn2)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), (((locals.var_t2_dn4 * locals.var_c_sb__blk1029) - (locals.var_t2 * locals.var_c_sb__blk1029_dn4)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), (((locals.var_t2_dn5 * locals.var_c_sb__blk1029) - (locals.var_t2 * locals.var_c_sb__blk1029_dn5)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), (((locals.var_t2_dn6 * locals.var_c_sb__blk1029) - (locals.var_t2 * locals.var_c_sb__blk1029_dn6)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), (((locals.var_t2_dn7 * locals.var_c_sb__blk1029) - (locals.var_t2 * locals.var_c_sb__blk1029_dn7)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), (((locals.var_t2_dn8 * locals.var_c_sb__blk1029) - (locals.var_t2 * locals.var_c_sb__blk1029_dn8)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), (((locals.var_t2_dn9 * locals.var_c_sb__blk1029) - (locals.var_t2 * locals.var_c_sb__blk1029_dn9)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), (((locals.var_t2_dn10 * locals.var_c_sb__blk1029) - (locals.var_t2 * locals.var_c_sb__blk1029_dn10)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), (((locals.var_t2_dn11 * locals.var_c_sb__blk1029) - (locals.var_t2 * locals.var_c_sb__blk1029_dn11)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)), (((locals.var_t2_dn14 * locals.var_c_sb__blk1029) - (locals.var_t2 * locals.var_c_sb__blk1029_dn14)) / (locals.var_c_sb__blk1029 * locals.var_c_sb__blk1029)),)
    } else {
        (locals.var_pb0dep, locals.var_pb0dep_dn0, locals.var_pb0dep_dn2, locals.var_pb0dep_dn4, locals.var_pb0dep_dn5, locals.var_pb0dep_dn6, locals.var_pb0dep_dn7, locals.var_pb0dep_dn8, locals.var_pb0dep_dn9, locals.var_pb0dep_dn10, locals.var_pb0dep_dn11, locals.var_pb0dep_dn14,)
    }
};
        locals.var_pb0dep = assign42100_e56580;
        locals.var_pb0dep_dn0 = assign42100_e56580_d_n0;
        locals.var_pb0dep_dn2 = assign42100_e56580_d_n2;
        locals.var_pb0dep_dn4 = assign42100_e56580_d_n4;
        locals.var_pb0dep_dn5 = assign42100_e56580_d_n5;
        locals.var_pb0dep_dn6 = assign42100_e56580_d_n6;
        locals.var_pb0dep_dn7 = assign42100_e56580_d_n7;
        locals.var_pb0dep_dn8 = assign42100_e56580_d_n8;
        locals.var_pb0dep_dn9 = assign42100_e56580_d_n9;
        locals.var_pb0dep_dn10 = assign42100_e56580_d_n10;
        locals.var_pb0dep_dn11 = assign42100_e56580_d_n11;
        locals.var_pb0dep_dn14 = assign42100_e56580_d_n14;
        locals.var_pb0dep_rv = 0.0;

        let assign42110_e56583: f64 = (2.0 * 1.034943e-10);
        let assign42110_e56586: f64 = (locals.var_ps0dep - locals.var_pb0dep);
        let assign42110_e56587: f64 = (assign42110_e56583 * assign42110_e56586);
        let assign42110_e56589: f64 = (assign42110_e56587 / locals.var_q_ndepm__blk907);
        let assign42110_e56591: f64 = if assign42110_e56589 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1052 = assign42110_e56591;
        locals.var_guard1052_rv = 0.0;

        let (assign42120_e56605, assign42120_e56605_d_n0, assign42120_e56605_d_n2, assign42120_e56605_d_n4, assign42120_e56605_d_n5, assign42120_e56605_d_n6, assign42120_e56605_d_n7, assign42120_e56605_d_n8, assign42120_e56605_d_n9, assign42120_e56605_d_n10, assign42120_e56605_d_n11, assign42120_e56605_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1052 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn11, locals.var_ws_dn14,)
    }
};
        locals.var_ws = assign42120_e56605;
        locals.var_ws_dn0 = assign42120_e56605_d_n0;
        locals.var_ws_dn2 = assign42120_e56605_d_n2;
        locals.var_ws_dn4 = assign42120_e56605_d_n4;
        locals.var_ws_dn5 = assign42120_e56605_d_n5;
        locals.var_ws_dn6 = assign42120_e56605_d_n6;
        locals.var_ws_dn7 = assign42120_e56605_d_n7;
        locals.var_ws_dn8 = assign42120_e56605_d_n8;
        locals.var_ws_dn9 = assign42120_e56605_d_n9;
        locals.var_ws_dn10 = assign42120_e56605_d_n10;
        locals.var_ws_dn11 = assign42120_e56605_d_n11;
        locals.var_ws_dn14 = assign42120_e56605_d_n14;
        locals.var_ws_rv = 0.0;

        let (assign42130_e56629, assign42130_e56629_d_n0, assign42130_e56629_d_n2, assign42130_e56629_d_n4, assign42130_e56629_d_n5, assign42130_e56629_d_n6, assign42130_e56629_d_n7, assign42130_e56629_d_n8, assign42130_e56629_d_n9, assign42130_e56629_d_n10, assign42130_e56629_d_n11, assign42130_e56629_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) && (locals.var_guard1052 == 0.0)) {
        let assign42130_e56620: f64 = (2.0 * 1.034943e-10);
        let assign42130_e56623: f64 = (locals.var_ps0dep - locals.var_pb0dep);
        let assign42130_e56624: f64 = (assign42130_e56620 * assign42130_e56623);
        let assign42130_e56626: f64 = (assign42130_e56624 / locals.var_q_ndepm__blk907);
        let assign42130_e56627: f64 = (assign42130_e56626).sqrt();
        (assign42130_e56627, (((((assign42130_e56620 * (locals.var_ps0dep_dn0 - locals.var_pb0dep_dn0)) * locals.var_q_ndepm__blk907) - (assign42130_e56624 * locals.var_q_ndepm__blk907_dn0)) / (locals.var_q_ndepm__blk907 * locals.var_q_ndepm__blk907)) / (2.0 * assign42130_e56627)), (((((assign42130_e56620 * (locals.var_ps0dep_dn2 - locals.var_pb0dep_dn2)) * locals.var_q_ndepm__blk907) - (assign42130_e56624 * locals.var_q_ndepm__blk907_dn2)) / (locals.var_q_ndepm__blk907 * locals.var_q_ndepm__blk907)) / (2.0 * assign42130_e56627)), (((((assign42130_e56620 * (locals.var_ps0dep_dn4 - locals.var_pb0dep_dn4)) * locals.var_q_ndepm__blk907) - (assign42130_e56624 * locals.var_q_ndepm__blk907_dn4)) / (locals.var_q_ndepm__blk907 * locals.var_q_ndepm__blk907)) / (2.0 * assign42130_e56627)), (((((assign42130_e56620 * (locals.var_ps0dep_dn5 - locals.var_pb0dep_dn5)) * locals.var_q_ndepm__blk907) - (assign42130_e56624 * locals.var_q_ndepm__blk907_dn5)) / (locals.var_q_ndepm__blk907 * locals.var_q_ndepm__blk907)) / (2.0 * assign42130_e56627)), (((((assign42130_e56620 * (locals.var_ps0dep_dn6 - locals.var_pb0dep_dn6)) * locals.var_q_ndepm__blk907) - (assign42130_e56624 * locals.var_q_ndepm__blk907_dn6)) / (locals.var_q_ndepm__blk907 * locals.var_q_ndepm__blk907)) / (2.0 * assign42130_e56627)), (((((assign42130_e56620 * (locals.var_ps0dep_dn7 - locals.var_pb0dep_dn7)) * locals.var_q_ndepm__blk907) - (assign42130_e56624 * locals.var_q_ndepm__blk907_dn7)) / (locals.var_q_ndepm__blk907 * locals.var_q_ndepm__blk907)) / (2.0 * assign42130_e56627)), (((((assign42130_e56620 * (locals.var_ps0dep_dn8 - locals.var_pb0dep_dn8)) * locals.var_q_ndepm__blk907) - (assign42130_e56624 * locals.var_q_ndepm__blk907_dn8)) / (locals.var_q_ndepm__blk907 * locals.var_q_ndepm__blk907)) / (2.0 * assign42130_e56627)), (((((assign42130_e56620 * (locals.var_ps0dep_dn9 - locals.var_pb0dep_dn9)) * locals.var_q_ndepm__blk907) - (assign42130_e56624 * locals.var_q_ndepm__blk907_dn9)) / (locals.var_q_ndepm__blk907 * locals.var_q_ndepm__blk907)) / (2.0 * assign42130_e56627)), (((((assign42130_e56620 * (locals.var_ps0dep_dn10 - locals.var_pb0dep_dn10)) * locals.var_q_ndepm__blk907) - (assign42130_e56624 * locals.var_q_ndepm__blk907_dn10)) / (locals.var_q_ndepm__blk907 * locals.var_q_ndepm__blk907)) / (2.0 * assign42130_e56627)), (((((assign42130_e56620 * (locals.var_ps0dep_dn11 - locals.var_pb0dep_dn11)) * locals.var_q_ndepm__blk907) - (assign42130_e56624 * locals.var_q_ndepm__blk907_dn11)) / (locals.var_q_ndepm__blk907 * locals.var_q_ndepm__blk907)) / (2.0 * assign42130_e56627)), (((((assign42130_e56620 * (locals.var_ps0dep_dn14 - locals.var_pb0dep_dn14)) * locals.var_q_ndepm__blk907) - (assign42130_e56624 * locals.var_q_ndepm__blk907_dn14)) / (locals.var_q_ndepm__blk907 * locals.var_q_ndepm__blk907)) / (2.0 * assign42130_e56627)),)
    } else {
        (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn11, locals.var_ws_dn14,)
    }
};
        locals.var_ws = assign42130_e56629;
        locals.var_ws_dn0 = assign42130_e56629_d_n0;
        locals.var_ws_dn2 = assign42130_e56629_d_n2;
        locals.var_ws_dn4 = assign42130_e56629_d_n4;
        locals.var_ws_dn5 = assign42130_e56629_d_n5;
        locals.var_ws_dn6 = assign42130_e56629_d_n6;
        locals.var_ws_dn7 = assign42130_e56629_d_n7;
        locals.var_ws_dn8 = assign42130_e56629_d_n8;
        locals.var_ws_dn9 = assign42130_e56629_d_n9;
        locals.var_ws_dn10 = assign42130_e56629_d_n10;
        locals.var_ws_dn11 = assign42130_e56629_d_n11;
        locals.var_ws_dn14 = assign42130_e56629_d_n14;
        locals.var_ws_rv = 0.0;

        let (assign42140_e56646, assign42140_e56646_d_n0, assign42140_e56646_d_n2, assign42140_e56646_d_n4, assign42140_e56646_d_n5, assign42140_e56646_d_n6, assign42140_e56646_d_n7, assign42140_e56646_d_n8, assign42140_e56646_d_n9, assign42140_e56646_d_n10, assign42140_e56646_d_n11, assign42140_e56646_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1032 == 0.0)) {
        let (assign42140_e56644, assign42140_e56644_d_n0, assign42140_e56644_d_n2, assign42140_e56644_d_n4, assign42140_e56644_d_n5, assign42140_e56644_d_n6, assign42140_e56644_d_n7, assign42140_e56644_d_n8, assign42140_e56644_d_n9, assign42140_e56644_d_n10, assign42140_e56644_d_n11, assign42140_e56644_d_n14,) = {
            if (locals.var_ws > locals.var_tnp) {
                (locals.var_tnp, locals.var_tnp_dn0, locals.var_tnp_dn2, locals.var_tnp_dn4, locals.var_tnp_dn5, locals.var_tnp_dn6, locals.var_tnp_dn7, locals.var_tnp_dn8, locals.var_tnp_dn9, locals.var_tnp_dn10, locals.var_tnp_dn11, locals.var_tnp_dn14,)
            } else {
                (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn11, locals.var_ws_dn14,)
            }
        };
        (assign42140_e56644, assign42140_e56644_d_n0, assign42140_e56644_d_n2, assign42140_e56644_d_n4, assign42140_e56644_d_n5, assign42140_e56644_d_n6, assign42140_e56644_d_n7, assign42140_e56644_d_n8, assign42140_e56644_d_n9, assign42140_e56644_d_n10, assign42140_e56644_d_n11, assign42140_e56644_d_n14,)
    } else {
        (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn11, locals.var_ws_dn14,)
    }
};
        locals.var_ws = assign42140_e56646;
        locals.var_ws_dn0 = assign42140_e56646_d_n0;
        locals.var_ws_dn2 = assign42140_e56646_d_n2;
        locals.var_ws_dn4 = assign42140_e56646_d_n4;
        locals.var_ws_dn5 = assign42140_e56646_d_n5;
        locals.var_ws_dn6 = assign42140_e56646_d_n6;
        locals.var_ws_dn7 = assign42140_e56646_d_n7;
        locals.var_ws_dn8 = assign42140_e56646_d_n8;
        locals.var_ws_dn9 = assign42140_e56646_d_n9;
        locals.var_ws_dn10 = assign42140_e56646_d_n10;
        locals.var_ws_dn11 = assign42140_e56646_d_n11;
        locals.var_ws_dn14 = assign42140_e56646_d_n14;
        locals.var_ws_rv = 0.0;

        let assign42150_e56649: f64 = if locals.var_ws < locals.var_tnp { 1.0 } else { 0.0 };
        locals.var_guard1053 = assign42150_e56649;
        locals.var_guard1053_rv = 0.0;

        let (assign42160_e56662, assign42160_e56662_d_n0, assign42160_e56662_d_n2, assign42160_e56662_d_n4, assign42160_e56662_d_n5, assign42160_e56662_d_n6, assign42160_e56662_d_n7, assign42160_e56662_d_n8, assign42160_e56662_d_n9, assign42160_e56662_d_n10, assign42160_e56662_d_n11, assign42160_e56662_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1053 != 0.0)) {
        let assign42160_e56660: f64 = (locals.var_tnp - locals.var_ws);
        (assign42160_e56660, (locals.var_tnp_dn0 - locals.var_ws_dn0), (locals.var_tnp_dn2 - locals.var_ws_dn2), (locals.var_tnp_dn4 - locals.var_ws_dn4), (locals.var_tnp_dn5 - locals.var_ws_dn5), (locals.var_tnp_dn6 - locals.var_ws_dn6), (locals.var_tnp_dn7 - locals.var_ws_dn7), (locals.var_tnp_dn8 - locals.var_ws_dn8), (locals.var_tnp_dn9 - locals.var_ws_dn9), (locals.var_tnp_dn10 - locals.var_ws_dn10), (locals.var_tnp_dn11 - locals.var_ws_dn11), (locals.var_tnp_dn14 - locals.var_ws_dn14),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign42160_e56662;
        locals.var_w_res_dn0 = assign42160_e56662_d_n0;
        locals.var_w_res_dn2 = assign42160_e56662_d_n2;
        locals.var_w_res_dn4 = assign42160_e56662_d_n4;
        locals.var_w_res_dn5 = assign42160_e56662_d_n5;
        locals.var_w_res_dn6 = assign42160_e56662_d_n6;
        locals.var_w_res_dn7 = assign42160_e56662_d_n7;
        locals.var_w_res_dn8 = assign42160_e56662_d_n8;
        locals.var_w_res_dn9 = assign42160_e56662_d_n9;
        locals.var_w_res_dn10 = assign42160_e56662_d_n10;
        locals.var_w_res_dn11 = assign42160_e56662_d_n11;
        locals.var_w_res_dn14 = assign42160_e56662_d_n14;
        locals.var_w_res_rv = 0.0;

        let (assign42170_e56674, assign42170_e56674_d_n0, assign42170_e56674_d_n2, assign42170_e56674_d_n4, assign42170_e56674_d_n5, assign42170_e56674_d_n6, assign42170_e56674_d_n7, assign42170_e56674_d_n8, assign42170_e56674_d_n9, assign42170_e56674_d_n10, assign42170_e56674_d_n11, assign42170_e56674_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1053 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign42170_e56674;
        locals.var_w_res_dn0 = assign42170_e56674_d_n0;
        locals.var_w_res_dn2 = assign42170_e56674_d_n2;
        locals.var_w_res_dn4 = assign42170_e56674_d_n4;
        locals.var_w_res_dn5 = assign42170_e56674_d_n5;
        locals.var_w_res_dn6 = assign42170_e56674_d_n6;
        locals.var_w_res_dn7 = assign42170_e56674_d_n7;
        locals.var_w_res_dn8 = assign42170_e56674_d_n8;
        locals.var_w_res_dn9 = assign42170_e56674_d_n9;
        locals.var_w_res_dn10 = assign42170_e56674_d_n10;
        locals.var_w_res_dn11 = assign42170_e56674_d_n11;
        locals.var_w_res_dn14 = assign42170_e56674_d_n14;
        locals.var_w_res_rv = 0.0;

        let (assign42180_e56686, assign42180_e56686_d_n0, assign42180_e56686_d_n2, assign42180_e56686_d_n4, assign42180_e56686_d_n5, assign42180_e56686_d_n6, assign42180_e56686_d_n7, assign42180_e56686_d_n8, assign42180_e56686_d_n9, assign42180_e56686_d_n10, assign42180_e56686_d_n11, assign42180_e56686_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42180_e56683: f64 = (locals.var_q_n0_cur__blk891 + locals.var_q_nl_cur__blk892);
        let assign42180_e56684: f64 = (-assign42180_e56683);
        (assign42180_e56684, (-(locals.var_q_n0_cur__blk891_dn0 + locals.var_q_nl_cur__blk892_dn0)), (-(locals.var_q_n0_cur__blk891_dn2 + locals.var_q_nl_cur__blk892_dn2)), (-(locals.var_q_n0_cur__blk891_dn4 + locals.var_q_nl_cur__blk892_dn4)), (-(locals.var_q_n0_cur__blk891_dn5 + locals.var_q_nl_cur__blk892_dn5)), (-(locals.var_q_n0_cur__blk891_dn6 + locals.var_q_nl_cur__blk892_dn6)), (-(locals.var_q_n0_cur__blk891_dn7 + locals.var_q_nl_cur__blk892_dn7)), (-(locals.var_q_n0_cur__blk891_dn8 + locals.var_q_nl_cur__blk892_dn8)), (-(locals.var_q_n0_cur__blk891_dn9 + locals.var_q_nl_cur__blk892_dn9)), (-(locals.var_q_n0_cur__blk891_dn10 + locals.var_q_nl_cur__blk892_dn10)), (-(locals.var_q_n0_cur__blk891_dn11 + locals.var_q_nl_cur__blk892_dn11)), (-(locals.var_q_n0_cur__blk891_dn14 + locals.var_q_nl_cur__blk892_dn14)),)
    } else {
        (locals.var_qn_drift__blk896, locals.var_qn_drift__blk896_dn0, locals.var_qn_drift__blk896_dn2, locals.var_qn_drift__blk896_dn4, locals.var_qn_drift__blk896_dn5, locals.var_qn_drift__blk896_dn6, locals.var_qn_drift__blk896_dn7, locals.var_qn_drift__blk896_dn8, locals.var_qn_drift__blk896_dn9, locals.var_qn_drift__blk896_dn10, locals.var_qn_drift__blk896_dn11, locals.var_qn_drift__blk896_dn14,)
    }
};
        locals.var_qn_drift__blk896 = assign42180_e56686;
        locals.var_qn_drift__blk896_dn0 = assign42180_e56686_d_n0;
        locals.var_qn_drift__blk896_dn2 = assign42180_e56686_d_n2;
        locals.var_qn_drift__blk896_dn4 = assign42180_e56686_d_n4;
        locals.var_qn_drift__blk896_dn5 = assign42180_e56686_d_n5;
        locals.var_qn_drift__blk896_dn6 = assign42180_e56686_d_n6;
        locals.var_qn_drift__blk896_dn7 = assign42180_e56686_d_n7;
        locals.var_qn_drift__blk896_dn8 = assign42180_e56686_d_n8;
        locals.var_qn_drift__blk896_dn9 = assign42180_e56686_d_n9;
        locals.var_qn_drift__blk896_dn10 = assign42180_e56686_d_n10;
        locals.var_qn_drift__blk896_dn11 = assign42180_e56686_d_n11;
        locals.var_qn_drift__blk896_dn14 = assign42180_e56686_d_n14;
        locals.var_qn_drift__blk896_rv = 0.0;

        let assign42190_e56689: f64 = if locals.var_pds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1054 = assign42190_e56689;
        locals.var_guard1054_rv = 0.0;

        let (assign42200_e56700, assign42200_e56700_d_n0, assign42200_e56700_d_n2, assign42200_e56700_d_n4, assign42200_e56700_d_n5, assign42200_e56700_d_n6, assign42200_e56700_d_n7, assign42200_e56700_d_n8, assign42200_e56700_d_n9, assign42200_e56700_d_n10, assign42200_e56700_d_n11, assign42200_e56700_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1054 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign42200_e56700;
        locals.var_pds_dn0 = assign42200_e56700_d_n0;
        locals.var_pds_dn2 = assign42200_e56700_d_n2;
        locals.var_pds_dn4 = assign42200_e56700_d_n4;
        locals.var_pds_dn5 = assign42200_e56700_d_n5;
        locals.var_pds_dn6 = assign42200_e56700_d_n6;
        locals.var_pds_dn7 = assign42200_e56700_d_n7;
        locals.var_pds_dn8 = assign42200_e56700_d_n8;
        locals.var_pds_dn9 = assign42200_e56700_d_n9;
        locals.var_pds_dn10 = assign42200_e56700_d_n10;
        locals.var_pds_dn11 = assign42200_e56700_d_n11;
        locals.var_pds_dn14 = assign42200_e56700_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign42210_e56711, assign42210_e56711_d_n0, assign42210_e56711_d_n2, assign42210_e56711_d_n4, assign42210_e56711_d_n5, assign42210_e56711_d_n6, assign42210_e56711_d_n7, assign42210_e56711_d_n8, assign42210_e56711_d_n9, assign42210_e56711_d_n10, assign42210_e56711_d_n11, assign42210_e56711_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1054 != 0.0)) {
        (locals.var_phi_s0_dep__blk855, locals.var_phi_s0_dep__blk855_dn0, locals.var_phi_s0_dep__blk855_dn2, locals.var_phi_s0_dep__blk855_dn4, locals.var_phi_s0_dep__blk855_dn5, locals.var_phi_s0_dep__blk855_dn6, locals.var_phi_s0_dep__blk855_dn7, locals.var_phi_s0_dep__blk855_dn8, locals.var_phi_s0_dep__blk855_dn9, locals.var_phi_s0_dep__blk855_dn10, locals.var_phi_s0_dep__blk855_dn11, locals.var_phi_s0_dep__blk855_dn14,)
    } else {
        (locals.var_phi_sl_dep__blk856, locals.var_phi_sl_dep__blk856_dn0, locals.var_phi_sl_dep__blk856_dn2, locals.var_phi_sl_dep__blk856_dn4, locals.var_phi_sl_dep__blk856_dn5, locals.var_phi_sl_dep__blk856_dn6, locals.var_phi_sl_dep__blk856_dn7, locals.var_phi_sl_dep__blk856_dn8, locals.var_phi_sl_dep__blk856_dn9, locals.var_phi_sl_dep__blk856_dn10, locals.var_phi_sl_dep__blk856_dn11, locals.var_phi_sl_dep__blk856_dn14,)
    }
};
        locals.var_phi_sl_dep__blk856 = assign42210_e56711;
        locals.var_phi_sl_dep__blk856_dn0 = assign42210_e56711_d_n0;
        locals.var_phi_sl_dep__blk856_dn2 = assign42210_e56711_d_n2;
        locals.var_phi_sl_dep__blk856_dn4 = assign42210_e56711_d_n4;
        locals.var_phi_sl_dep__blk856_dn5 = assign42210_e56711_d_n5;
        locals.var_phi_sl_dep__blk856_dn6 = assign42210_e56711_d_n6;
        locals.var_phi_sl_dep__blk856_dn7 = assign42210_e56711_d_n7;
        locals.var_phi_sl_dep__blk856_dn8 = assign42210_e56711_d_n8;
        locals.var_phi_sl_dep__blk856_dn9 = assign42210_e56711_d_n9;
        locals.var_phi_sl_dep__blk856_dn10 = assign42210_e56711_d_n10;
        locals.var_phi_sl_dep__blk856_dn11 = assign42210_e56711_d_n11;
        locals.var_phi_sl_dep__blk856_dn14 = assign42210_e56711_d_n14;
        locals.var_phi_sl_dep__blk856_rv = 0.0;

        let (assign42220_e56722, assign42220_e56722_d_n0, assign42220_e56722_d_n2, assign42220_e56722_d_n4, assign42220_e56722_d_n5, assign42220_e56722_d_n6, assign42220_e56722_d_n7, assign42220_e56722_d_n8, assign42220_e56722_d_n9, assign42220_e56722_d_n10, assign42220_e56722_d_n11, assign42220_e56722_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1054 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
    }
};
        locals.var_idd = assign42220_e56722;
        locals.var_idd_dn0 = assign42220_e56722_d_n0;
        locals.var_idd_dn2 = assign42220_e56722_d_n2;
        locals.var_idd_dn4 = assign42220_e56722_d_n4;
        locals.var_idd_dn5 = assign42220_e56722_d_n5;
        locals.var_idd_dn6 = assign42220_e56722_d_n6;
        locals.var_idd_dn7 = assign42220_e56722_d_n7;
        locals.var_idd_dn8 = assign42220_e56722_d_n8;
        locals.var_idd_dn9 = assign42220_e56722_d_n9;
        locals.var_idd_dn10 = assign42220_e56722_d_n10;
        locals.var_idd_dn11 = assign42220_e56722_d_n11;
        locals.var_idd_dn14 = assign42220_e56722_d_n14;
        locals.var_idd_rv = 0.0;

        let (assign42230_e56740, assign42230_e56740_d_n0, assign42230_e56740_d_n2, assign42230_e56740_d_n4, assign42230_e56740_d_n5, assign42230_e56740_d_n6, assign42230_e56740_d_n7, assign42230_e56740_d_n8, assign42230_e56740_d_n9, assign42230_e56740_d_n10, assign42230_e56740_d_n11, assign42230_e56740_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1054 == 0.0)) {
        let assign42230_e56734: f64 = (locals.var_beta * locals.var_qn_drift__blk896);
        let assign42230_e56736: f64 = (assign42230_e56734 / 2.0);
        let assign42230_e56738: f64 = (assign42230_e56736 * locals.var_pds);
        (assign42230_e56738, (((((locals.var_beta_dn0 * locals.var_qn_drift__blk896) + (locals.var_beta * locals.var_qn_drift__blk896_dn0)) / 2.0) * locals.var_pds) + (assign42230_e56736 * locals.var_pds_dn0)), (((((locals.var_beta_dn2 * locals.var_qn_drift__blk896) + (locals.var_beta * locals.var_qn_drift__blk896_dn2)) / 2.0) * locals.var_pds) + (assign42230_e56736 * locals.var_pds_dn2)), (((((locals.var_beta_dn4 * locals.var_qn_drift__blk896) + (locals.var_beta * locals.var_qn_drift__blk896_dn4)) / 2.0) * locals.var_pds) + (assign42230_e56736 * locals.var_pds_dn4)), (((((locals.var_beta_dn5 * locals.var_qn_drift__blk896) + (locals.var_beta * locals.var_qn_drift__blk896_dn5)) / 2.0) * locals.var_pds) + (assign42230_e56736 * locals.var_pds_dn5)), (((((locals.var_beta_dn6 * locals.var_qn_drift__blk896) + (locals.var_beta * locals.var_qn_drift__blk896_dn6)) / 2.0) * locals.var_pds) + (assign42230_e56736 * locals.var_pds_dn6)), (((((locals.var_beta_dn7 * locals.var_qn_drift__blk896) + (locals.var_beta * locals.var_qn_drift__blk896_dn7)) / 2.0) * locals.var_pds) + (assign42230_e56736 * locals.var_pds_dn7)), (((((locals.var_beta_dn8 * locals.var_qn_drift__blk896) + (locals.var_beta * locals.var_qn_drift__blk896_dn8)) / 2.0) * locals.var_pds) + (assign42230_e56736 * locals.var_pds_dn8)), (((((locals.var_beta_dn9 * locals.var_qn_drift__blk896) + (locals.var_beta * locals.var_qn_drift__blk896_dn9)) / 2.0) * locals.var_pds) + (assign42230_e56736 * locals.var_pds_dn9)), (((((locals.var_beta_dn10 * locals.var_qn_drift__blk896) + (locals.var_beta * locals.var_qn_drift__blk896_dn10)) / 2.0) * locals.var_pds) + (assign42230_e56736 * locals.var_pds_dn10)), (((((locals.var_beta_dn11 * locals.var_qn_drift__blk896) + (locals.var_beta * locals.var_qn_drift__blk896_dn11)) / 2.0) * locals.var_pds) + (assign42230_e56736 * locals.var_pds_dn11)), (((((locals.var_beta_dn14 * locals.var_qn_drift__blk896) + (locals.var_beta * locals.var_qn_drift__blk896_dn14)) / 2.0) * locals.var_pds) + (assign42230_e56736 * locals.var_pds_dn14)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
    }
};
        locals.var_idd = assign42230_e56740;
        locals.var_idd_dn0 = assign42230_e56740_d_n0;
        locals.var_idd_dn2 = assign42230_e56740_d_n2;
        locals.var_idd_dn4 = assign42230_e56740_d_n4;
        locals.var_idd_dn5 = assign42230_e56740_d_n5;
        locals.var_idd_dn6 = assign42230_e56740_d_n6;
        locals.var_idd_dn7 = assign42230_e56740_d_n7;
        locals.var_idd_dn8 = assign42230_e56740_d_n8;
        locals.var_idd_dn9 = assign42230_e56740_d_n9;
        locals.var_idd_dn10 = assign42230_e56740_d_n10;
        locals.var_idd_dn11 = assign42230_e56740_d_n11;
        locals.var_idd_dn14 = assign42230_e56740_d_n14;
        locals.var_idd_rv = 0.0;

        let (assign42240_e56757, assign42240_e56757_d_n0, assign42240_e56757_d_n2, assign42240_e56757_d_n4, assign42240_e56757_d_n5, assign42240_e56757_d_n6, assign42240_e56757_d_n7, assign42240_e56757_d_n8, assign42240_e56757_d_n9, assign42240_e56757_d_n10, assign42240_e56757_d_n11, assign42240_e56757_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1054 == 0.0)) {
        let (assign42240_e56755, assign42240_e56755_d_n0, assign42240_e56755_d_n2, assign42240_e56755_d_n4, assign42240_e56755_d_n5, assign42240_e56755_d_n6, assign42240_e56755_d_n7, assign42240_e56755_d_n8, assign42240_e56755_d_n9, assign42240_e56755_d_n10, assign42240_e56755_d_n11, assign42240_e56755_d_n14,) = {
            if (locals.var_idd < 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
            }
        };
        (assign42240_e56755, assign42240_e56755_d_n0, assign42240_e56755_d_n2, assign42240_e56755_d_n4, assign42240_e56755_d_n5, assign42240_e56755_d_n6, assign42240_e56755_d_n7, assign42240_e56755_d_n8, assign42240_e56755_d_n9, assign42240_e56755_d_n10, assign42240_e56755_d_n11, assign42240_e56755_d_n14,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
    }
};
        locals.var_idd = assign42240_e56757;
        locals.var_idd_dn0 = assign42240_e56757_d_n0;
        locals.var_idd_dn2 = assign42240_e56757_d_n2;
        locals.var_idd_dn4 = assign42240_e56757_d_n4;
        locals.var_idd_dn5 = assign42240_e56757_d_n5;
        locals.var_idd_dn6 = assign42240_e56757_d_n6;
        locals.var_idd_dn7 = assign42240_e56757_d_n7;
        locals.var_idd_dn8 = assign42240_e56757_d_n8;
        locals.var_idd_dn9 = assign42240_e56757_d_n9;
        locals.var_idd_dn10 = assign42240_e56757_d_n10;
        locals.var_idd_dn11 = assign42240_e56757_d_n11;
        locals.var_idd_dn14 = assign42240_e56757_d_n14;
        locals.var_idd_rv = 0.0;

        let (assign42250_e56767, assign42250_e56767_d_n0, assign42250_e56767_d_n2, assign42250_e56767_d_n4, assign42250_e56767_d_n5, assign42250_e56767_d_n6, assign42250_e56767_d_n7, assign42250_e56767_d_n8, assign42250_e56767_d_n9, assign42250_e56767_d_n10, assign42250_e56767_d_n11, assign42250_e56767_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42250_e56765: f64 = (-locals.var_q_n0_sym);
        (assign42250_e56765, (-locals.var_q_n0_sym_dn0), (-locals.var_q_n0_sym_dn2), (-locals.var_q_n0_sym_dn4), (-locals.var_q_n0_sym_dn5), (-locals.var_q_n0_sym_dn6), (-locals.var_q_n0_sym_dn7), (-locals.var_q_n0_sym_dn8), (-locals.var_q_n0_sym_dn9), (-locals.var_q_n0_sym_dn10), (-locals.var_q_n0_sym_dn11), (-locals.var_q_n0_sym_dn14),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn14,)
    }
};
        locals.var_qn0 = assign42250_e56767;
        locals.var_qn0_dn0 = assign42250_e56767_d_n0;
        locals.var_qn0_dn2 = assign42250_e56767_d_n2;
        locals.var_qn0_dn4 = assign42250_e56767_d_n4;
        locals.var_qn0_dn5 = assign42250_e56767_d_n5;
        locals.var_qn0_dn6 = assign42250_e56767_d_n6;
        locals.var_qn0_dn7 = assign42250_e56767_d_n7;
        locals.var_qn0_dn8 = assign42250_e56767_d_n8;
        locals.var_qn0_dn9 = assign42250_e56767_d_n9;
        locals.var_qn0_dn10 = assign42250_e56767_d_n10;
        locals.var_qn0_dn11 = assign42250_e56767_d_n11;
        locals.var_qn0_dn14 = assign42250_e56767_d_n14;
        locals.var_qn0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_146(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign42260_e56776, assign42260_e56776_d_n0, assign42260_e56776_d_n2, assign42260_e56776_d_n4, assign42260_e56776_d_n5, assign42260_e56776_d_n6, assign42260_e56776_d_n7, assign42260_e56776_d_n8, assign42260_e56776_d_n9, assign42260_e56776_d_n10, assign42260_e56776_d_n11, assign42260_e56776_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        (locals.var_leff, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    }
};
        locals.var_lch = assign42260_e56776;
        locals.var_lch_dn0 = assign42260_e56776_d_n0;
        locals.var_lch_dn2 = assign42260_e56776_d_n2;
        locals.var_lch_dn4 = assign42260_e56776_d_n4;
        locals.var_lch_dn5 = assign42260_e56776_d_n5;
        locals.var_lch_dn6 = assign42260_e56776_d_n6;
        locals.var_lch_dn7 = assign42260_e56776_d_n7;
        locals.var_lch_dn8 = assign42260_e56776_d_n8;
        locals.var_lch_dn9 = assign42260_e56776_d_n9;
        locals.var_lch_dn10 = assign42260_e56776_d_n10;
        locals.var_lch_dn11 = assign42260_e56776_d_n11;
        locals.var_lch_dn14 = assign42260_e56776_d_n14;
        locals.var_lch_rv = 0.0;

        let (assign42270_e56787, assign42270_e56787_d_n0, assign42270_e56787_d_n2, assign42270_e56787_d_n4, assign42270_e56787_d_n5, assign42270_e56787_d_n6, assign42270_e56787_d_n7, assign42270_e56787_d_n8, assign42270_e56787_d_n9, assign42270_e56787_d_n10, assign42270_e56787_d_n11, assign42270_e56787_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42270_e56785: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign42270_e56785, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42270_e56787;
        locals.var_t2_dn0 = assign42270_e56787_d_n0;
        locals.var_t2_dn2 = assign42270_e56787_d_n2;
        locals.var_t2_dn4 = assign42270_e56787_d_n4;
        locals.var_t2_dn5 = assign42270_e56787_d_n5;
        locals.var_t2_dn6 = assign42270_e56787_d_n6;
        locals.var_t2_dn7 = assign42270_e56787_d_n7;
        locals.var_t2_dn8 = assign42270_e56787_d_n8;
        locals.var_t2_dn9 = assign42270_e56787_d_n9;
        locals.var_t2_dn10 = assign42270_e56787_d_n10;
        locals.var_t2_dn11 = assign42270_e56787_d_n11;
        locals.var_t2_dn14 = assign42270_e56787_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign42280_e56796, assign42280_e56796_d_n0, assign42280_e56796_d_n2, assign42280_e56796_d_n4, assign42280_e56796_d_n5, assign42280_e56796_d_n6, assign42280_e56796_d_n7, assign42280_e56796_d_n8, assign42280_e56796_d_n9, assign42280_e56796_d_n10, assign42280_e56796_d_n11, assign42280_e56796_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign42280_e56796;
        locals.var_t0_dn0 = assign42280_e56796_d_n0;
        locals.var_t0_dn2 = assign42280_e56796_d_n2;
        locals.var_t0_dn4 = assign42280_e56796_d_n4;
        locals.var_t0_dn5 = assign42280_e56796_d_n5;
        locals.var_t0_dn6 = assign42280_e56796_d_n6;
        locals.var_t0_dn7 = assign42280_e56796_d_n7;
        locals.var_t0_dn8 = assign42280_e56796_d_n8;
        locals.var_t0_dn9 = assign42280_e56796_d_n9;
        locals.var_t0_dn10 = assign42280_e56796_d_n10;
        locals.var_t0_dn11 = assign42280_e56796_d_n11;
        locals.var_t0_dn14 = assign42280_e56796_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign42290_e56813, assign42290_e56813_d_n0, assign42290_e56813_d_n2, assign42290_e56813_d_n4, assign42290_e56813_d_n5, assign42290_e56813_d_n6, assign42290_e56813_d_n7, assign42290_e56813_d_n8, assign42290_e56813_d_n9, assign42290_e56813_d_n10, assign42290_e56813_d_n11, assign42290_e56813_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42290_e56805: f64 = (locals.var_pds * locals.var_pds);
        let assign42290_e56807: f64 = (assign42290_e56805 + p.p262);
        let assign42290_e56808: f64 = (assign42290_e56807).sqrt();
        let assign42290_e56810: f64 = (p.p262).sqrt();
        let assign42290_e56811: f64 = (assign42290_e56808 - assign42290_e56810);
        (assign42290_e56811, (((locals.var_pds_dn0 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn0)) / (2.0 * assign42290_e56808)), (((locals.var_pds_dn2 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn2)) / (2.0 * assign42290_e56808)), (((locals.var_pds_dn4 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn4)) / (2.0 * assign42290_e56808)), (((locals.var_pds_dn5 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn5)) / (2.0 * assign42290_e56808)), (((locals.var_pds_dn6 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn6)) / (2.0 * assign42290_e56808)), (((locals.var_pds_dn7 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn7)) / (2.0 * assign42290_e56808)), (((locals.var_pds_dn8 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn8)) / (2.0 * assign42290_e56808)), (((locals.var_pds_dn9 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn9)) / (2.0 * assign42290_e56808)), (((locals.var_pds_dn10 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn10)) / (2.0 * assign42290_e56808)), (((locals.var_pds_dn11 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn11)) / (2.0 * assign42290_e56808)), (((locals.var_pds_dn14 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn14)) / (2.0 * assign42290_e56808)),)
    } else {
        (locals.var_pdsz, locals.var_pdsz_dn0, locals.var_pdsz_dn2, locals.var_pdsz_dn4, locals.var_pdsz_dn5, locals.var_pdsz_dn6, locals.var_pdsz_dn7, locals.var_pdsz_dn8, locals.var_pdsz_dn9, locals.var_pdsz_dn10, locals.var_pdsz_dn11, locals.var_pdsz_dn14,)
    }
};
        locals.var_pdsz = assign42290_e56813;
        locals.var_pdsz_dn0 = assign42290_e56813_d_n0;
        locals.var_pdsz_dn2 = assign42290_e56813_d_n2;
        locals.var_pdsz_dn4 = assign42290_e56813_d_n4;
        locals.var_pdsz_dn5 = assign42290_e56813_d_n5;
        locals.var_pdsz_dn6 = assign42290_e56813_d_n6;
        locals.var_pdsz_dn7 = assign42290_e56813_d_n7;
        locals.var_pdsz_dn8 = assign42290_e56813_d_n8;
        locals.var_pdsz_dn9 = assign42290_e56813_d_n9;
        locals.var_pdsz_dn10 = assign42290_e56813_d_n10;
        locals.var_pdsz_dn11 = assign42290_e56813_d_n11;
        locals.var_pdsz_dn14 = assign42290_e56813_d_n14;
        locals.var_pdsz_rv = 0.0;

        let (assign42300_e56826, assign42300_e56826_d_n0, assign42300_e56826_d_n2, assign42300_e56826_d_n4, assign42300_e56826_d_n5, assign42300_e56826_d_n6, assign42300_e56826_d_n7, assign42300_e56826_d_n8, assign42300_e56826_d_n9, assign42300_e56826_d_n10, assign42300_e56826_d_n11, assign42300_e56826_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42300_e56823: f64 = (locals.var_pdsz * locals.var_t0);
        let assign42300_e56824: f64 = (1.0 + assign42300_e56823);
        (assign42300_e56824, ((locals.var_pdsz_dn0 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn0)), ((locals.var_pdsz_dn2 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn2)), ((locals.var_pdsz_dn4 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn4)), ((locals.var_pdsz_dn5 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn5)), ((locals.var_pdsz_dn6 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn6)), ((locals.var_pdsz_dn7 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn7)), ((locals.var_pdsz_dn8 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn8)), ((locals.var_pdsz_dn9 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn9)), ((locals.var_pdsz_dn10 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn10)), ((locals.var_pdsz_dn11 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn11)), ((locals.var_pdsz_dn14 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign42300_e56826;
        locals.var_t4_dn0 = assign42300_e56826_d_n0;
        locals.var_t4_dn2 = assign42300_e56826_d_n2;
        locals.var_t4_dn4 = assign42300_e56826_d_n4;
        locals.var_t4_dn5 = assign42300_e56826_d_n5;
        locals.var_t4_dn6 = assign42300_e56826_d_n6;
        locals.var_t4_dn7 = assign42300_e56826_d_n7;
        locals.var_t4_dn8 = assign42300_e56826_d_n8;
        locals.var_t4_dn9 = assign42300_e56826_d_n9;
        locals.var_t4_dn10 = assign42300_e56826_d_n10;
        locals.var_t4_dn11 = assign42300_e56826_d_n11;
        locals.var_t4_dn14 = assign42300_e56826_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign42310_e56837, assign42310_e56837_d_n0, assign42310_e56837_d_n2, assign42310_e56837_d_n4, assign42310_e56837_d_n5, assign42310_e56837_d_n6, assign42310_e56837_d_n7, assign42310_e56837_d_n8, assign42310_e56837_d_n9, assign42310_e56837_d_n10, assign42310_e56837_d_n11, assign42310_e56837_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42310_e56835: f64 = (locals.var_t2 * locals.var_qn0);
        (assign42310_e56835, ((locals.var_t2_dn0 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn0)), ((locals.var_t2_dn2 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn2)), ((locals.var_t2_dn4 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn4)), ((locals.var_t2_dn5 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn5)), ((locals.var_t2_dn6 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn6)), ((locals.var_t2_dn7 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn7)), ((locals.var_t2_dn8 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn8)), ((locals.var_t2_dn9 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn9)), ((locals.var_t2_dn10 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn10)), ((locals.var_t2_dn11 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn11)), ((locals.var_t2_dn14 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42310_e56837;
        locals.var_t5_dn0 = assign42310_e56837_d_n0;
        locals.var_t5_dn2 = assign42310_e56837_d_n2;
        locals.var_t5_dn4 = assign42310_e56837_d_n4;
        locals.var_t5_dn5 = assign42310_e56837_d_n5;
        locals.var_t5_dn6 = assign42310_e56837_d_n6;
        locals.var_t5_dn7 = assign42310_e56837_d_n7;
        locals.var_t5_dn8 = assign42310_e56837_d_n8;
        locals.var_t5_dn9 = assign42310_e56837_d_n9;
        locals.var_t5_dn10 = assign42310_e56837_d_n10;
        locals.var_t5_dn11 = assign42310_e56837_d_n11;
        locals.var_t5_dn14 = assign42310_e56837_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign42320_e56848, assign42320_e56848_d_n0, assign42320_e56848_d_n2, assign42320_e56848_d_n4, assign42320_e56848_d_n5, assign42320_e56848_d_n6, assign42320_e56848_d_n7, assign42320_e56848_d_n8, assign42320_e56848_d_n9, assign42320_e56848_d_n10, assign42320_e56848_d_n11, assign42320_e56848_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42320_e56846: f64 = (locals.var_t5 / locals.var_t4);
        (assign42320_e56846, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign42320_e56848;
        locals.var_t3_dn0 = assign42320_e56848_d_n0;
        locals.var_t3_dn2 = assign42320_e56848_d_n2;
        locals.var_t3_dn4 = assign42320_e56848_d_n4;
        locals.var_t3_dn5 = assign42320_e56848_d_n5;
        locals.var_t3_dn6 = assign42320_e56848_d_n6;
        locals.var_t3_dn7 = assign42320_e56848_d_n7;
        locals.var_t3_dn8 = assign42320_e56848_d_n8;
        locals.var_t3_dn9 = assign42320_e56848_d_n9;
        locals.var_t3_dn10 = assign42320_e56848_d_n10;
        locals.var_t3_dn11 = assign42320_e56848_d_n11;
        locals.var_t3_dn14 = assign42320_e56848_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign42330_e56857, assign42330_e56857_d_n0, assign42330_e56857_d_n2, assign42330_e56857_d_n4, assign42330_e56857_d_n5, assign42330_e56857_d_n6, assign42330_e56857_d_n7, assign42330_e56857_d_n8, assign42330_e56857_d_n9, assign42330_e56857_d_n10, assign42330_e56857_d_n11, assign42330_e56857_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign42330_e56857;
        locals.var_eeff_dn0 = assign42330_e56857_d_n0;
        locals.var_eeff_dn2 = assign42330_e56857_d_n2;
        locals.var_eeff_dn4 = assign42330_e56857_d_n4;
        locals.var_eeff_dn5 = assign42330_e56857_d_n5;
        locals.var_eeff_dn6 = assign42330_e56857_d_n6;
        locals.var_eeff_dn7 = assign42330_e56857_d_n7;
        locals.var_eeff_dn8 = assign42330_e56857_d_n8;
        locals.var_eeff_dn9 = assign42330_e56857_d_n9;
        locals.var_eeff_dn10 = assign42330_e56857_d_n10;
        locals.var_eeff_dn11 = assign42330_e56857_d_n11;
        locals.var_eeff_dn14 = assign42330_e56857_d_n14;
        locals.var_eeff_rv = 0.0;

        let (assign42340_e56875, assign42340_e56875_d_n0, assign42340_e56875_d_n2, assign42340_e56875_d_n4, assign42340_e56875_d_n5, assign42340_e56875_d_n6, assign42340_e56875_d_n7, assign42340_e56875_d_n8, assign42340_e56875_d_n9, assign42340_e56875_d_n10, assign42340_e56875_d_n11, assign42340_e56875_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let (assign42340_e56873, assign42340_e56873_d_n0, assign42340_e56873_d_n2, assign42340_e56873_d_n4, assign42340_e56873_d_n5, assign42340_e56873_d_n6, assign42340_e56873_d_n7, assign42340_e56873_d_n8, assign42340_e56873_d_n9, assign42340_e56873_d_n10, assign42340_e56873_d_n11, assign42340_e56873_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42340_e56871: f64 = (p.p160 - 1.0);
                let assign42340_e56872: f64 = (locals.var_eeff).powf(assign42340_e56871);
                (assign42340_e56872, if 0.0 == 0.0 && ((assign42340_e56871) as f64).is_finite() && ((assign42340_e56871) as f64).fract() == 0.0 { if assign42340_e56871 == 0.0 { 0.0 } else { (assign42340_e56871 * ((locals.var_eeff).powf(assign42340_e56871 - 1.0) * locals.var_eeff_dn0)) } } else { (assign42340_e56872 * (assign42340_e56871 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56871) as f64).is_finite() && ((assign42340_e56871) as f64).fract() == 0.0 { if assign42340_e56871 == 0.0 { 0.0 } else { (assign42340_e56871 * ((locals.var_eeff).powf(assign42340_e56871 - 1.0) * locals.var_eeff_dn2)) } } else { (assign42340_e56872 * (assign42340_e56871 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56871) as f64).is_finite() && ((assign42340_e56871) as f64).fract() == 0.0 { if assign42340_e56871 == 0.0 { 0.0 } else { (assign42340_e56871 * ((locals.var_eeff).powf(assign42340_e56871 - 1.0) * locals.var_eeff_dn4)) } } else { (assign42340_e56872 * (assign42340_e56871 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56871) as f64).is_finite() && ((assign42340_e56871) as f64).fract() == 0.0 { if assign42340_e56871 == 0.0 { 0.0 } else { (assign42340_e56871 * ((locals.var_eeff).powf(assign42340_e56871 - 1.0) * locals.var_eeff_dn5)) } } else { (assign42340_e56872 * (assign42340_e56871 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56871) as f64).is_finite() && ((assign42340_e56871) as f64).fract() == 0.0 { if assign42340_e56871 == 0.0 { 0.0 } else { (assign42340_e56871 * ((locals.var_eeff).powf(assign42340_e56871 - 1.0) * locals.var_eeff_dn6)) } } else { (assign42340_e56872 * (assign42340_e56871 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56871) as f64).is_finite() && ((assign42340_e56871) as f64).fract() == 0.0 { if assign42340_e56871 == 0.0 { 0.0 } else { (assign42340_e56871 * ((locals.var_eeff).powf(assign42340_e56871 - 1.0) * locals.var_eeff_dn7)) } } else { (assign42340_e56872 * (assign42340_e56871 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56871) as f64).is_finite() && ((assign42340_e56871) as f64).fract() == 0.0 { if assign42340_e56871 == 0.0 { 0.0 } else { (assign42340_e56871 * ((locals.var_eeff).powf(assign42340_e56871 - 1.0) * locals.var_eeff_dn8)) } } else { (assign42340_e56872 * (assign42340_e56871 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56871) as f64).is_finite() && ((assign42340_e56871) as f64).fract() == 0.0 { if assign42340_e56871 == 0.0 { 0.0 } else { (assign42340_e56871 * ((locals.var_eeff).powf(assign42340_e56871 - 1.0) * locals.var_eeff_dn9)) } } else { (assign42340_e56872 * (assign42340_e56871 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56871) as f64).is_finite() && ((assign42340_e56871) as f64).fract() == 0.0 { if assign42340_e56871 == 0.0 { 0.0 } else { (assign42340_e56871 * ((locals.var_eeff).powf(assign42340_e56871 - 1.0) * locals.var_eeff_dn10)) } } else { (assign42340_e56872 * (assign42340_e56871 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56871) as f64).is_finite() && ((assign42340_e56871) as f64).fract() == 0.0 { if assign42340_e56871 == 0.0 { 0.0 } else { (assign42340_e56871 * ((locals.var_eeff).powf(assign42340_e56871 - 1.0) * locals.var_eeff_dn11)) } } else { (assign42340_e56872 * (assign42340_e56871 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56871) as f64).is_finite() && ((assign42340_e56871) as f64).fract() == 0.0 { if assign42340_e56871 == 0.0 { 0.0 } else { (assign42340_e56871 * ((locals.var_eeff).powf(assign42340_e56871 - 1.0) * locals.var_eeff_dn14)) } } else { (assign42340_e56872 * (assign42340_e56871 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign42340_e56873, assign42340_e56873_d_n0, assign42340_e56873_d_n2, assign42340_e56873_d_n4, assign42340_e56873_d_n5, assign42340_e56873_d_n6, assign42340_e56873_d_n7, assign42340_e56873_d_n8, assign42340_e56873_d_n9, assign42340_e56873_d_n10, assign42340_e56873_d_n11, assign42340_e56873_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42340_e56875;
        locals.var_t5_dn0 = assign42340_e56875_d_n0;
        locals.var_t5_dn2 = assign42340_e56875_d_n2;
        locals.var_t5_dn4 = assign42340_e56875_d_n4;
        locals.var_t5_dn5 = assign42340_e56875_d_n5;
        locals.var_t5_dn6 = assign42340_e56875_d_n6;
        locals.var_t5_dn7 = assign42340_e56875_d_n7;
        locals.var_t5_dn8 = assign42340_e56875_d_n8;
        locals.var_t5_dn9 = assign42340_e56875_d_n9;
        locals.var_t5_dn10 = assign42340_e56875_d_n10;
        locals.var_t5_dn11 = assign42340_e56875_d_n11;
        locals.var_t5_dn14 = assign42340_e56875_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign42350_e56886, assign42350_e56886_d_n0, assign42350_e56886_d_n2, assign42350_e56886_d_n4, assign42350_e56886_d_n5, assign42350_e56886_d_n6, assign42350_e56886_d_n7, assign42350_e56886_d_n8, assign42350_e56886_d_n9, assign42350_e56886_d_n10, assign42350_e56886_d_n11, assign42350_e56886_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42350_e56884: f64 = (locals.var_t5 * locals.var_eeff);
        (assign42350_e56884, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn11 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn11)), ((locals.var_t5_dn14 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign42350_e56886;
        locals.var_t8_dn0 = assign42350_e56886_d_n0;
        locals.var_t8_dn2 = assign42350_e56886_d_n2;
        locals.var_t8_dn4 = assign42350_e56886_d_n4;
        locals.var_t8_dn5 = assign42350_e56886_d_n5;
        locals.var_t8_dn6 = assign42350_e56886_d_n6;
        locals.var_t8_dn7 = assign42350_e56886_d_n7;
        locals.var_t8_dn8 = assign42350_e56886_d_n8;
        locals.var_t8_dn9 = assign42350_e56886_d_n9;
        locals.var_t8_dn10 = assign42350_e56886_d_n10;
        locals.var_t8_dn11 = assign42350_e56886_d_n11;
        locals.var_t8_dn14 = assign42350_e56886_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign42360_e56904, assign42360_e56904_d_n0, assign42360_e56904_d_n2, assign42360_e56904_d_n4, assign42360_e56904_d_n5, assign42360_e56904_d_n6, assign42360_e56904_d_n7, assign42360_e56904_d_n8, assign42360_e56904_d_n9, assign42360_e56904_d_n10, assign42360_e56904_d_n11, assign42360_e56904_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let (assign42360_e56902, assign42360_e56902_d_n0, assign42360_e56902_d_n2, assign42360_e56902_d_n4, assign42360_e56902_d_n5, assign42360_e56902_d_n6, assign42360_e56902_d_n7, assign42360_e56902_d_n8, assign42360_e56902_d_n9, assign42360_e56902_d_n10, assign42360_e56902_d_n11, assign42360_e56902_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42360_e56900: f64 = (locals.var_muesr - 1.0);
                let assign42360_e56901: f64 = (locals.var_eeff).powf(assign42360_e56900);
                (assign42360_e56901, if 0.0 == 0.0 && ((assign42360_e56900) as f64).is_finite() && ((assign42360_e56900) as f64).fract() == 0.0 { if assign42360_e56900 == 0.0 { 0.0 } else { (assign42360_e56900 * ((locals.var_eeff).powf(assign42360_e56900 - 1.0) * locals.var_eeff_dn0)) } } else { (assign42360_e56901 * (assign42360_e56900 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56900) as f64).is_finite() && ((assign42360_e56900) as f64).fract() == 0.0 { if assign42360_e56900 == 0.0 { 0.0 } else { (assign42360_e56900 * ((locals.var_eeff).powf(assign42360_e56900 - 1.0) * locals.var_eeff_dn2)) } } else { (assign42360_e56901 * (assign42360_e56900 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56900) as f64).is_finite() && ((assign42360_e56900) as f64).fract() == 0.0 { if assign42360_e56900 == 0.0 { 0.0 } else { (assign42360_e56900 * ((locals.var_eeff).powf(assign42360_e56900 - 1.0) * locals.var_eeff_dn4)) } } else { (assign42360_e56901 * (assign42360_e56900 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56900) as f64).is_finite() && ((assign42360_e56900) as f64).fract() == 0.0 { if assign42360_e56900 == 0.0 { 0.0 } else { (assign42360_e56900 * ((locals.var_eeff).powf(assign42360_e56900 - 1.0) * locals.var_eeff_dn5)) } } else { (assign42360_e56901 * (assign42360_e56900 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56900) as f64).is_finite() && ((assign42360_e56900) as f64).fract() == 0.0 { if assign42360_e56900 == 0.0 { 0.0 } else { (assign42360_e56900 * ((locals.var_eeff).powf(assign42360_e56900 - 1.0) * locals.var_eeff_dn6)) } } else { (assign42360_e56901 * (assign42360_e56900 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56900) as f64).is_finite() && ((assign42360_e56900) as f64).fract() == 0.0 { if assign42360_e56900 == 0.0 { 0.0 } else { (assign42360_e56900 * ((locals.var_eeff).powf(assign42360_e56900 - 1.0) * locals.var_eeff_dn7)) } } else { (assign42360_e56901 * (assign42360_e56900 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56900) as f64).is_finite() && ((assign42360_e56900) as f64).fract() == 0.0 { if assign42360_e56900 == 0.0 { 0.0 } else { (assign42360_e56900 * ((locals.var_eeff).powf(assign42360_e56900 - 1.0) * locals.var_eeff_dn8)) } } else { (assign42360_e56901 * (assign42360_e56900 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56900) as f64).is_finite() && ((assign42360_e56900) as f64).fract() == 0.0 { if assign42360_e56900 == 0.0 { 0.0 } else { (assign42360_e56900 * ((locals.var_eeff).powf(assign42360_e56900 - 1.0) * locals.var_eeff_dn9)) } } else { (assign42360_e56901 * (assign42360_e56900 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56900) as f64).is_finite() && ((assign42360_e56900) as f64).fract() == 0.0 { if assign42360_e56900 == 0.0 { 0.0 } else { (assign42360_e56900 * ((locals.var_eeff).powf(assign42360_e56900 - 1.0) * locals.var_eeff_dn10)) } } else { (assign42360_e56901 * (assign42360_e56900 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56900) as f64).is_finite() && ((assign42360_e56900) as f64).fract() == 0.0 { if assign42360_e56900 == 0.0 { 0.0 } else { (assign42360_e56900 * ((locals.var_eeff).powf(assign42360_e56900 - 1.0) * locals.var_eeff_dn11)) } } else { (assign42360_e56901 * (assign42360_e56900 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56900) as f64).is_finite() && ((assign42360_e56900) as f64).fract() == 0.0 { if assign42360_e56900 == 0.0 { 0.0 } else { (assign42360_e56900 * ((locals.var_eeff).powf(assign42360_e56900 - 1.0) * locals.var_eeff_dn14)) } } else { (assign42360_e56901 * (assign42360_e56900 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign42360_e56902, assign42360_e56902_d_n0, assign42360_e56902_d_n2, assign42360_e56902_d_n4, assign42360_e56902_d_n5, assign42360_e56902_d_n6, assign42360_e56902_d_n7, assign42360_e56902_d_n8, assign42360_e56902_d_n9, assign42360_e56902_d_n10, assign42360_e56902_d_n11, assign42360_e56902_d_n14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign42360_e56904;
        locals.var_t7_dn0 = assign42360_e56904_d_n0;
        locals.var_t7_dn2 = assign42360_e56904_d_n2;
        locals.var_t7_dn4 = assign42360_e56904_d_n4;
        locals.var_t7_dn5 = assign42360_e56904_d_n5;
        locals.var_t7_dn6 = assign42360_e56904_d_n6;
        locals.var_t7_dn7 = assign42360_e56904_d_n7;
        locals.var_t7_dn8 = assign42360_e56904_d_n8;
        locals.var_t7_dn9 = assign42360_e56904_d_n9;
        locals.var_t7_dn10 = assign42360_e56904_d_n10;
        locals.var_t7_dn11 = assign42360_e56904_d_n11;
        locals.var_t7_dn14 = assign42360_e56904_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign42370_e56915, assign42370_e56915_d_n0, assign42370_e56915_d_n2, assign42370_e56915_d_n4, assign42370_e56915_d_n5, assign42370_e56915_d_n6, assign42370_e56915_d_n7, assign42370_e56915_d_n8, assign42370_e56915_d_n9, assign42370_e56915_d_n10, assign42370_e56915_d_n11, assign42370_e56915_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42370_e56913: f64 = (locals.var_t7 * locals.var_eeff);
        (assign42370_e56913, ((locals.var_t7_dn0 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn0)), ((locals.var_t7_dn2 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn2)), ((locals.var_t7_dn4 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn4)), ((locals.var_t7_dn5 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn5)), ((locals.var_t7_dn6 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn6)), ((locals.var_t7_dn7 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn7)), ((locals.var_t7_dn8 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn8)), ((locals.var_t7_dn9 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn9)), ((locals.var_t7_dn10 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn10)), ((locals.var_t7_dn11 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn11)), ((locals.var_t7_dn14 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign42370_e56915;
        locals.var_t6_dn0 = assign42370_e56915_d_n0;
        locals.var_t6_dn2 = assign42370_e56915_d_n2;
        locals.var_t6_dn4 = assign42370_e56915_d_n4;
        locals.var_t6_dn5 = assign42370_e56915_d_n5;
        locals.var_t6_dn6 = assign42370_e56915_d_n6;
        locals.var_t6_dn7 = assign42370_e56915_d_n7;
        locals.var_t6_dn8 = assign42370_e56915_d_n8;
        locals.var_t6_dn9 = assign42370_e56915_d_n9;
        locals.var_t6_dn10 = assign42370_e56915_d_n10;
        locals.var_t6_dn11 = assign42370_e56915_d_n11;
        locals.var_t6_dn14 = assign42370_e56915_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign42380_e56926, assign42380_e56926_d_n0, assign42380_e56926_d_n2, assign42380_e56926_d_n4, assign42380_e56926_d_n5, assign42380_e56926_d_n6, assign42380_e56926_d_n7, assign42380_e56926_d_n8, assign42380_e56926_d_n9, assign42380_e56926_d_n10, assign42380_e56926_d_n11, assign42380_e56926_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42380_e56924: f64 = (1.6021918e-19 * 10000.0);
        (assign42380_e56924, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign42380_e56926;
        locals.var_t9_dn0 = assign42380_e56926_d_n0;
        locals.var_t9_dn2 = assign42380_e56926_d_n2;
        locals.var_t9_dn4 = assign42380_e56926_d_n4;
        locals.var_t9_dn5 = assign42380_e56926_d_n5;
        locals.var_t9_dn6 = assign42380_e56926_d_n6;
        locals.var_t9_dn7 = assign42380_e56926_d_n7;
        locals.var_t9_dn8 = assign42380_e56926_d_n8;
        locals.var_t9_dn9 = assign42380_e56926_d_n9;
        locals.var_t9_dn10 = assign42380_e56926_d_n10;
        locals.var_t9_dn11 = assign42380_e56926_d_n11;
        locals.var_t9_dn14 = assign42380_e56926_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign42390_e56937, assign42390_e56937_d_n0, assign42390_e56937_d_n2, assign42390_e56937_d_n4, assign42390_e56937_d_n5, assign42390_e56937_d_n6, assign42390_e56937_d_n7, assign42390_e56937_d_n8, assign42390_e56937_d_n9, assign42390_e56937_d_n10, assign42390_e56937_d_n11, assign42390_e56937_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42390_e56935: f64 = (locals.var_qn0 / locals.var_t9);
        (assign42390_e56935, (((locals.var_qn0_dn0 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn2 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn4 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn5 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn6 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn7 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn8 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn9 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn10 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn11 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn14 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign42390_e56937;
        locals.var_rns_dn0 = assign42390_e56937_d_n0;
        locals.var_rns_dn2 = assign42390_e56937_d_n2;
        locals.var_rns_dn4 = assign42390_e56937_d_n4;
        locals.var_rns_dn5 = assign42390_e56937_d_n5;
        locals.var_rns_dn6 = assign42390_e56937_d_n6;
        locals.var_rns_dn7 = assign42390_e56937_d_n7;
        locals.var_rns_dn8 = assign42390_e56937_d_n8;
        locals.var_rns_dn9 = assign42390_e56937_d_n9;
        locals.var_rns_dn10 = assign42390_e56937_d_n10;
        locals.var_rns_dn11 = assign42390_e56937_d_n11;
        locals.var_rns_dn14 = assign42390_e56937_d_n14;
        locals.var_rns_rv = 0.0;

        let (assign42400_e56946, assign42400_e56946_d_n0, assign42400_e56946_d_n2, assign42400_e56946_d_n4, assign42400_e56946_d_n5, assign42400_e56946_d_n6, assign42400_e56946_d_n7, assign42400_e56946_d_n8, assign42400_e56946_d_n9, assign42400_e56946_d_n10, assign42400_e56946_d_n11, assign42400_e56946_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        (locals.var_uc_muecb0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42400_e56946;
        locals.var_t2_dn0 = assign42400_e56946_d_n0;
        locals.var_t2_dn2 = assign42400_e56946_d_n2;
        locals.var_t2_dn4 = assign42400_e56946_d_n4;
        locals.var_t2_dn5 = assign42400_e56946_d_n5;
        locals.var_t2_dn6 = assign42400_e56946_d_n6;
        locals.var_t2_dn7 = assign42400_e56946_d_n7;
        locals.var_t2_dn8 = assign42400_e56946_d_n8;
        locals.var_t2_dn9 = assign42400_e56946_d_n9;
        locals.var_t2_dn10 = assign42400_e56946_d_n10;
        locals.var_t2_dn11 = assign42400_e56946_d_n11;
        locals.var_t2_dn14 = assign42400_e56946_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign42410_e56971, assign42410_e56971_d_n0, assign42410_e56971_d_n2, assign42410_e56971_d_n4, assign42410_e56971_d_n5, assign42410_e56971_d_n6, assign42410_e56971_d_n7, assign42410_e56971_d_n8, assign42410_e56971_d_n9, assign42410_e56971_d_n10, assign42410_e56971_d_n11, assign42410_e56971_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42410_e56957: f64 = (locals.var_uc_muecb1 * locals.var_rns);
        let assign42410_e56959: f64 = (assign42410_e56957 / 100000000000.0);
        let assign42410_e56960: f64 = (locals.var_t2 + assign42410_e56959);
        let assign42410_e56961: f64 = (1.0 / assign42410_e56960);
        let assign42410_e56964: f64 = (locals.var_mphn0 * locals.var_t8);
        let assign42410_e56965: f64 = (assign42410_e56961 + assign42410_e56964);
        let assign42410_e56968: f64 = (locals.var_t6 / locals.var_uc_muesr1);
        let assign42410_e56969: f64 = (assign42410_e56965 + assign42410_e56968);
        (assign42410_e56969, (((-((locals.var_t2_dn0 + ((locals.var_uc_muecb1 * locals.var_rns_dn0) / 100000000000.0)) / (assign42410_e56960 * assign42410_e56960))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn2 + ((locals.var_uc_muecb1 * locals.var_rns_dn2) / 100000000000.0)) / (assign42410_e56960 * assign42410_e56960))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn4 + ((locals.var_uc_muecb1 * locals.var_rns_dn4) / 100000000000.0)) / (assign42410_e56960 * assign42410_e56960))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn5 + ((locals.var_uc_muecb1 * locals.var_rns_dn5) / 100000000000.0)) / (assign42410_e56960 * assign42410_e56960))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn6 + ((locals.var_uc_muecb1 * locals.var_rns_dn6) / 100000000000.0)) / (assign42410_e56960 * assign42410_e56960))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn7 + ((locals.var_uc_muecb1 * locals.var_rns_dn7) / 100000000000.0)) / (assign42410_e56960 * assign42410_e56960))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn8 + ((locals.var_uc_muecb1 * locals.var_rns_dn8) / 100000000000.0)) / (assign42410_e56960 * assign42410_e56960))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn9 + ((locals.var_uc_muecb1 * locals.var_rns_dn9) / 100000000000.0)) / (assign42410_e56960 * assign42410_e56960))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn10 + ((locals.var_uc_muecb1 * locals.var_rns_dn10) / 100000000000.0)) / (assign42410_e56960 * assign42410_e56960))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn11 + ((locals.var_uc_muecb1 * locals.var_rns_dn11) / 100000000000.0)) / (assign42410_e56960 * assign42410_e56960))) + ((locals.var_mphn0_dn11 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn11))) + (locals.var_t6_dn11 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn14 + ((locals.var_uc_muecb1 * locals.var_rns_dn14) / 100000000000.0)) / (assign42410_e56960 * assign42410_e56960))) + ((locals.var_mphn0_dn14 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn14))) + (locals.var_t6_dn14 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign42410_e56971;
        locals.var_t1_dn0 = assign42410_e56971_d_n0;
        locals.var_t1_dn2 = assign42410_e56971_d_n2;
        locals.var_t1_dn4 = assign42410_e56971_d_n4;
        locals.var_t1_dn5 = assign42410_e56971_d_n5;
        locals.var_t1_dn6 = assign42410_e56971_d_n6;
        locals.var_t1_dn7 = assign42410_e56971_d_n7;
        locals.var_t1_dn8 = assign42410_e56971_d_n8;
        locals.var_t1_dn9 = assign42410_e56971_d_n9;
        locals.var_t1_dn10 = assign42410_e56971_d_n10;
        locals.var_t1_dn11 = assign42410_e56971_d_n11;
        locals.var_t1_dn14 = assign42410_e56971_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign42420_e56982, assign42420_e56982_d_n0, assign42420_e56982_d_n2, assign42420_e56982_d_n4, assign42420_e56982_d_n5, assign42420_e56982_d_n6, assign42420_e56982_d_n7, assign42420_e56982_d_n8, assign42420_e56982_d_n9, assign42420_e56982_d_n10, assign42420_e56982_d_n11, assign42420_e56982_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42420_e56980: f64 = (1.0 / locals.var_t1);
        (assign42420_e56980, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign42420_e56982;
        locals.var_muun_dn0 = assign42420_e56982_d_n0;
        locals.var_muun_dn2 = assign42420_e56982_d_n2;
        locals.var_muun_dn4 = assign42420_e56982_d_n4;
        locals.var_muun_dn5 = assign42420_e56982_d_n5;
        locals.var_muun_dn6 = assign42420_e56982_d_n6;
        locals.var_muun_dn7 = assign42420_e56982_d_n7;
        locals.var_muun_dn8 = assign42420_e56982_d_n8;
        locals.var_muun_dn9 = assign42420_e56982_d_n9;
        locals.var_muun_dn10 = assign42420_e56982_d_n10;
        locals.var_muun_dn11 = assign42420_e56982_d_n11;
        locals.var_muun_dn14 = assign42420_e56982_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign42430_e56993, assign42430_e56993_d_n0, assign42430_e56993_d_n2, assign42430_e56993_d_n4, assign42430_e56993_d_n5, assign42430_e56993_d_n6, assign42430_e56993_d_n7, assign42430_e56993_d_n8, assign42430_e56993_d_n9, assign42430_e56993_d_n10, assign42430_e56993_d_n11, assign42430_e56993_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42430_e56991: f64 = (locals.var_muun / 10000.0);
        (assign42430_e56991, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign42430_e56993;
        locals.var_muun_dn0 = assign42430_e56993_d_n0;
        locals.var_muun_dn2 = assign42430_e56993_d_n2;
        locals.var_muun_dn4 = assign42430_e56993_d_n4;
        locals.var_muun_dn5 = assign42430_e56993_d_n5;
        locals.var_muun_dn6 = assign42430_e56993_d_n6;
        locals.var_muun_dn7 = assign42430_e56993_d_n7;
        locals.var_muun_dn8 = assign42430_e56993_d_n8;
        locals.var_muun_dn9 = assign42430_e56993_d_n9;
        locals.var_muun_dn10 = assign42430_e56993_d_n10;
        locals.var_muun_dn11 = assign42430_e56993_d_n11;
        locals.var_muun_dn14 = assign42430_e56993_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign42440_e57008, assign42440_e57008_d_n0, assign42440_e57008_d_n2, assign42440_e57008_d_n4, assign42440_e57008_d_n5, assign42440_e57008_d_n6, assign42440_e57008_d_n7, assign42440_e57008_d_n8, assign42440_e57008_d_n9, assign42440_e57008_d_n10, assign42440_e57008_d_n11, assign42440_e57008_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42440_e57003: f64 = (locals.var_qn0 + 1e-25);
        let assign42440_e57004: f64 = (locals.var_beta * assign42440_e57003);
        let assign42440_e57006: f64 = (assign42440_e57004 * locals.var_lch);
        (assign42440_e57006, ((((locals.var_beta_dn0 * assign42440_e57003) + (locals.var_beta * locals.var_qn0_dn0)) * locals.var_lch) + (assign42440_e57004 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign42440_e57003) + (locals.var_beta * locals.var_qn0_dn2)) * locals.var_lch) + (assign42440_e57004 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign42440_e57003) + (locals.var_beta * locals.var_qn0_dn4)) * locals.var_lch) + (assign42440_e57004 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign42440_e57003) + (locals.var_beta * locals.var_qn0_dn5)) * locals.var_lch) + (assign42440_e57004 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign42440_e57003) + (locals.var_beta * locals.var_qn0_dn6)) * locals.var_lch) + (assign42440_e57004 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign42440_e57003) + (locals.var_beta * locals.var_qn0_dn7)) * locals.var_lch) + (assign42440_e57004 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign42440_e57003) + (locals.var_beta * locals.var_qn0_dn8)) * locals.var_lch) + (assign42440_e57004 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign42440_e57003) + (locals.var_beta * locals.var_qn0_dn9)) * locals.var_lch) + (assign42440_e57004 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign42440_e57003) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign42440_e57004 * locals.var_lch_dn10)), ((((locals.var_beta_dn11 * assign42440_e57003) + (locals.var_beta * locals.var_qn0_dn11)) * locals.var_lch) + (assign42440_e57004 * locals.var_lch_dn11)), ((((locals.var_beta_dn14 * assign42440_e57003) + (locals.var_beta * locals.var_qn0_dn14)) * locals.var_lch) + (assign42440_e57004 * locals.var_lch_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42440_e57008;
        locals.var_t2_dn0 = assign42440_e57008_d_n0;
        locals.var_t2_dn2 = assign42440_e57008_d_n2;
        locals.var_t2_dn4 = assign42440_e57008_d_n4;
        locals.var_t2_dn5 = assign42440_e57008_d_n5;
        locals.var_t2_dn6 = assign42440_e57008_d_n6;
        locals.var_t2_dn7 = assign42440_e57008_d_n7;
        locals.var_t2_dn8 = assign42440_e57008_d_n8;
        locals.var_t2_dn9 = assign42440_e57008_d_n9;
        locals.var_t2_dn10 = assign42440_e57008_d_n10;
        locals.var_t2_dn11 = assign42440_e57008_d_n11;
        locals.var_t2_dn14 = assign42440_e57008_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign42450_e57019, assign42450_e57019_d_n0, assign42450_e57019_d_n2, assign42450_e57019_d_n4, assign42450_e57019_d_n5, assign42450_e57019_d_n6, assign42450_e57019_d_n7, assign42450_e57019_d_n8, assign42450_e57019_d_n9, assign42450_e57019_d_n10, assign42450_e57019_d_n11, assign42450_e57019_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42450_e57017: f64 = (1.0 / locals.var_t2);
        (assign42450_e57017, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign42450_e57019;
        locals.var_t1_dn0 = assign42450_e57019_d_n0;
        locals.var_t1_dn2 = assign42450_e57019_d_n2;
        locals.var_t1_dn4 = assign42450_e57019_d_n4;
        locals.var_t1_dn5 = assign42450_e57019_d_n5;
        locals.var_t1_dn6 = assign42450_e57019_d_n6;
        locals.var_t1_dn7 = assign42450_e57019_d_n7;
        locals.var_t1_dn8 = assign42450_e57019_d_n8;
        locals.var_t1_dn9 = assign42450_e57019_d_n9;
        locals.var_t1_dn10 = assign42450_e57019_d_n10;
        locals.var_t1_dn11 = assign42450_e57019_d_n11;
        locals.var_t1_dn14 = assign42450_e57019_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign42460_e57030, assign42460_e57030_d_n0, assign42460_e57030_d_n2, assign42460_e57030_d_n4, assign42460_e57030_d_n5, assign42460_e57030_d_n6, assign42460_e57030_d_n7, assign42460_e57030_d_n8, assign42460_e57030_d_n9, assign42460_e57030_d_n10, assign42460_e57030_d_n11, assign42460_e57030_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42460_e57028: f64 = (locals.var_idd * locals.var_t1);
        (assign42460_e57028, ((locals.var_idd_dn0 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn0)), ((locals.var_idd_dn2 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn2)), ((locals.var_idd_dn4 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn4)), ((locals.var_idd_dn5 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn5)), ((locals.var_idd_dn6 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn6)), ((locals.var_idd_dn7 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn7)), ((locals.var_idd_dn8 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn8)), ((locals.var_idd_dn9 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn9)), ((locals.var_idd_dn10 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn10)), ((locals.var_idd_dn11 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn11)), ((locals.var_idd_dn14 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign42460_e57030;
        locals.var_ty_dn0 = assign42460_e57030_d_n0;
        locals.var_ty_dn2 = assign42460_e57030_d_n2;
        locals.var_ty_dn4 = assign42460_e57030_d_n4;
        locals.var_ty_dn5 = assign42460_e57030_d_n5;
        locals.var_ty_dn6 = assign42460_e57030_d_n6;
        locals.var_ty_dn7 = assign42460_e57030_d_n7;
        locals.var_ty_dn8 = assign42460_e57030_d_n8;
        locals.var_ty_dn9 = assign42460_e57030_d_n9;
        locals.var_ty_dn10 = assign42460_e57030_d_n10;
        locals.var_ty_dn11 = assign42460_e57030_d_n11;
        locals.var_ty_dn14 = assign42460_e57030_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign42470_e57043, assign42470_e57043_d_n0, assign42470_e57043_d_n2, assign42470_e57043_d_n4, assign42470_e57043_d_n5, assign42470_e57043_d_n6, assign42470_e57043_d_n7, assign42470_e57043_d_n8, assign42470_e57043_d_n9, assign42470_e57043_d_n10, assign42470_e57043_d_n11, assign42470_e57043_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42470_e57039: f64 = (0.2 * locals.var_vmaxe);
        let assign42470_e57041: f64 = (assign42470_e57039 / locals.var_muun);
        (assign42470_e57041, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign42470_e57039 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign42470_e57039 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign42470_e57039 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign42470_e57039 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign42470_e57039 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign42470_e57039 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign42470_e57039 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign42470_e57039 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign42470_e57039 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign42470_e57039 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn14) * locals.var_muun) - (assign42470_e57039 * locals.var_muun_dn14)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42470_e57043;
        locals.var_t2_dn0 = assign42470_e57043_d_n0;
        locals.var_t2_dn2 = assign42470_e57043_d_n2;
        locals.var_t2_dn4 = assign42470_e57043_d_n4;
        locals.var_t2_dn5 = assign42470_e57043_d_n5;
        locals.var_t2_dn6 = assign42470_e57043_d_n6;
        locals.var_t2_dn7 = assign42470_e57043_d_n7;
        locals.var_t2_dn8 = assign42470_e57043_d_n8;
        locals.var_t2_dn9 = assign42470_e57043_d_n9;
        locals.var_t2_dn10 = assign42470_e57043_d_n10;
        locals.var_t2_dn11 = assign42470_e57043_d_n11;
        locals.var_t2_dn14 = assign42470_e57043_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_147(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign42480_e57059, assign42480_e57059_d_n0, assign42480_e57059_d_n2, assign42480_e57059_d_n4, assign42480_e57059_d_n5, assign42480_e57059_d_n6, assign42480_e57059_d_n7, assign42480_e57059_d_n8, assign42480_e57059_d_n9, assign42480_e57059_d_n10, assign42480_e57059_d_n11, assign42480_e57059_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42480_e57052: f64 = (locals.var_ty * locals.var_ty);
        let assign42480_e57055: f64 = (locals.var_t2 * locals.var_t2);
        let assign42480_e57056: f64 = (assign42480_e57052 + assign42480_e57055);
        let assign42480_e57057: f64 = (assign42480_e57056).sqrt();
        (assign42480_e57057, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign42480_e57057)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign42480_e57057)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign42480_e57057)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign42480_e57057)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign42480_e57057)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign42480_e57057)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign42480_e57057)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign42480_e57057)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign42480_e57057)), ((((locals.var_ty_dn11 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn11)) + ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))) / (2.0 * assign42480_e57057)), ((((locals.var_ty_dn14 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn14)) + ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))) / (2.0 * assign42480_e57057)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    }
};
        locals.var_ey = assign42480_e57059;
        locals.var_ey_dn0 = assign42480_e57059_d_n0;
        locals.var_ey_dn2 = assign42480_e57059_d_n2;
        locals.var_ey_dn4 = assign42480_e57059_d_n4;
        locals.var_ey_dn5 = assign42480_e57059_d_n5;
        locals.var_ey_dn6 = assign42480_e57059_d_n6;
        locals.var_ey_dn7 = assign42480_e57059_d_n7;
        locals.var_ey_dn8 = assign42480_e57059_d_n8;
        locals.var_ey_dn9 = assign42480_e57059_d_n9;
        locals.var_ey_dn10 = assign42480_e57059_d_n10;
        locals.var_ey_dn11 = assign42480_e57059_d_n11;
        locals.var_ey_dn14 = assign42480_e57059_d_n14;
        locals.var_ey_rv = 0.0;

        let (assign42490_e57070, assign42490_e57070_d_n0, assign42490_e57070_d_n2, assign42490_e57070_d_n4, assign42490_e57070_d_n5, assign42490_e57070_d_n6, assign42490_e57070_d_n7, assign42490_e57070_d_n8, assign42490_e57070_d_n9, assign42490_e57070_d_n10, assign42490_e57070_d_n11, assign42490_e57070_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42490_e57068: f64 = (1.0 / locals.var_ey);
        (assign42490_e57068, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn11 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn14 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign42490_e57070;
        locals.var_t4_dn0 = assign42490_e57070_d_n0;
        locals.var_t4_dn2 = assign42490_e57070_d_n2;
        locals.var_t4_dn4 = assign42490_e57070_d_n4;
        locals.var_t4_dn5 = assign42490_e57070_d_n5;
        locals.var_t4_dn6 = assign42490_e57070_d_n6;
        locals.var_t4_dn7 = assign42490_e57070_d_n7;
        locals.var_t4_dn8 = assign42490_e57070_d_n8;
        locals.var_t4_dn9 = assign42490_e57070_d_n9;
        locals.var_t4_dn10 = assign42490_e57070_d_n10;
        locals.var_t4_dn11 = assign42490_e57070_d_n11;
        locals.var_t4_dn14 = assign42490_e57070_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign42500_e57081, assign42500_e57081_d_n0, assign42500_e57081_d_n2, assign42500_e57081_d_n4, assign42500_e57081_d_n5, assign42500_e57081_d_n6, assign42500_e57081_d_n7, assign42500_e57081_d_n8, assign42500_e57081_d_n9, assign42500_e57081_d_n10, assign42500_e57081_d_n11, assign42500_e57081_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42500_e57079: f64 = (locals.var_muun * locals.var_ey);
        (assign42500_e57079, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11)), ((locals.var_muun_dn14 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn14)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11, locals.var_em_dn14,)
    }
};
        locals.var_em = assign42500_e57081;
        locals.var_em_dn0 = assign42500_e57081_d_n0;
        locals.var_em_dn2 = assign42500_e57081_d_n2;
        locals.var_em_dn4 = assign42500_e57081_d_n4;
        locals.var_em_dn5 = assign42500_e57081_d_n5;
        locals.var_em_dn6 = assign42500_e57081_d_n6;
        locals.var_em_dn7 = assign42500_e57081_d_n7;
        locals.var_em_dn8 = assign42500_e57081_d_n8;
        locals.var_em_dn9 = assign42500_e57081_d_n9;
        locals.var_em_dn10 = assign42500_e57081_d_n10;
        locals.var_em_dn11 = assign42500_e57081_d_n11;
        locals.var_em_dn14 = assign42500_e57081_d_n14;
        locals.var_em_rv = 0.0;

        let (assign42510_e57092, assign42510_e57092_d_n0, assign42510_e57092_d_n2, assign42510_e57092_d_n4, assign42510_e57092_d_n5, assign42510_e57092_d_n6, assign42510_e57092_d_n7, assign42510_e57092_d_n8, assign42510_e57092_d_n9, assign42510_e57092_d_n10, assign42510_e57092_d_n11, assign42510_e57092_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42510_e57090: f64 = (locals.var_em / locals.var_vmaxe);
        (assign42510_e57090, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn14 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn14)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign42510_e57092;
        locals.var_t1_dn0 = assign42510_e57092_d_n0;
        locals.var_t1_dn2 = assign42510_e57092_d_n2;
        locals.var_t1_dn4 = assign42510_e57092_d_n4;
        locals.var_t1_dn5 = assign42510_e57092_d_n5;
        locals.var_t1_dn6 = assign42510_e57092_d_n6;
        locals.var_t1_dn7 = assign42510_e57092_d_n7;
        locals.var_t1_dn8 = assign42510_e57092_d_n8;
        locals.var_t1_dn9 = assign42510_e57092_d_n9;
        locals.var_t1_dn10 = assign42510_e57092_d_n10;
        locals.var_t1_dn11 = assign42510_e57092_d_n11;
        locals.var_t1_dn14 = assign42510_e57092_d_n14;
        locals.var_t1_rv = 0.0;

        let assign42520_e57096: f64 = (10.0 * 2.220446049250313e-16);
        let assign42520_e57097: f64 = (1.0 - assign42520_e57096);
        let assign42520_e57104: f64 = (10.0 * 2.220446049250313e-16);
        let assign42520_e57105: f64 = (1.0 + assign42520_e57104);
        let assign42520_e57107: f64 = if ((assign42520_e57097 <= p.p178) && (p.p178 <= assign42520_e57105)) { 1.0 } else { 0.0 };
        locals.var_guard1055 = assign42520_e57107;
        locals.var_guard1055_rv = 0.0;

        let (assign42530_e57118, assign42530_e57118_d_n0, assign42530_e57118_d_n2, assign42530_e57118_d_n4, assign42530_e57118_d_n5, assign42530_e57118_d_n6, assign42530_e57118_d_n7, assign42530_e57118_d_n8, assign42530_e57118_d_n9, assign42530_e57118_d_n10, assign42530_e57118_d_n11, assign42530_e57118_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1055 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign42530_e57118;
        locals.var_t3_dn0 = assign42530_e57118_d_n0;
        locals.var_t3_dn2 = assign42530_e57118_d_n2;
        locals.var_t3_dn4 = assign42530_e57118_d_n4;
        locals.var_t3_dn5 = assign42530_e57118_d_n5;
        locals.var_t3_dn6 = assign42530_e57118_d_n6;
        locals.var_t3_dn7 = assign42530_e57118_d_n7;
        locals.var_t3_dn8 = assign42530_e57118_d_n8;
        locals.var_t3_dn9 = assign42530_e57118_d_n9;
        locals.var_t3_dn10 = assign42530_e57118_d_n10;
        locals.var_t3_dn11 = assign42530_e57118_d_n11;
        locals.var_t3_dn14 = assign42530_e57118_d_n14;
        locals.var_t3_rv = 0.0;

        let assign42540_e57122: f64 = (10.0 * 2.220446049250313e-16);
        let assign42540_e57123: f64 = (2.0 - assign42540_e57122);
        let assign42540_e57130: f64 = (10.0 * 2.220446049250313e-16);
        let assign42540_e57131: f64 = (2.0 + assign42540_e57130);
        let assign42540_e57133: f64 = if ((assign42540_e57123 <= p.p178) && (p.p178 <= assign42540_e57131)) { 1.0 } else { 0.0 };
        locals.var_guard1056 = assign42540_e57133;
        locals.var_guard1056_rv = 0.0;

        let (assign42550_e57147, assign42550_e57147_d_n0, assign42550_e57147_d_n2, assign42550_e57147_d_n4, assign42550_e57147_d_n5, assign42550_e57147_d_n6, assign42550_e57147_d_n7, assign42550_e57147_d_n8, assign42550_e57147_d_n9, assign42550_e57147_d_n10, assign42550_e57147_d_n11, assign42550_e57147_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1055 == 0.0)) && (locals.var_guard1056 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign42550_e57147;
        locals.var_t3_dn0 = assign42550_e57147_d_n0;
        locals.var_t3_dn2 = assign42550_e57147_d_n2;
        locals.var_t3_dn4 = assign42550_e57147_d_n4;
        locals.var_t3_dn5 = assign42550_e57147_d_n5;
        locals.var_t3_dn6 = assign42550_e57147_d_n6;
        locals.var_t3_dn7 = assign42550_e57147_d_n7;
        locals.var_t3_dn8 = assign42550_e57147_d_n8;
        locals.var_t3_dn9 = assign42550_e57147_d_n9;
        locals.var_t3_dn10 = assign42550_e57147_d_n10;
        locals.var_t3_dn11 = assign42550_e57147_d_n11;
        locals.var_t3_dn14 = assign42550_e57147_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign42560_e57171, assign42560_e57171_d_n0, assign42560_e57171_d_n2, assign42560_e57171_d_n4, assign42560_e57171_d_n5, assign42560_e57171_d_n6, assign42560_e57171_d_n7, assign42560_e57171_d_n8, assign42560_e57171_d_n9, assign42560_e57171_d_n10, assign42560_e57171_d_n11, assign42560_e57171_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1055 == 0.0)) && (locals.var_guard1056 == 0.0)) {
        let (assign42560_e57169, assign42560_e57169_d_n0, assign42560_e57169_d_n2, assign42560_e57169_d_n4, assign42560_e57169_d_n5, assign42560_e57169_d_n6, assign42560_e57169_d_n7, assign42560_e57169_d_n8, assign42560_e57169_d_n9, assign42560_e57169_d_n10, assign42560_e57169_d_n11, assign42560_e57169_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42560_e57167: f64 = (p.p178 - 1.0);
                let assign42560_e57168: f64 = (locals.var_t1).powf(assign42560_e57167);
                (assign42560_e57168, if 0.0 == 0.0 && ((assign42560_e57167) as f64).is_finite() && ((assign42560_e57167) as f64).fract() == 0.0 { if assign42560_e57167 == 0.0 { 0.0 } else { (assign42560_e57167 * ((locals.var_t1).powf(assign42560_e57167 - 1.0) * locals.var_t1_dn0)) } } else { (assign42560_e57168 * (assign42560_e57167 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42560_e57167) as f64).is_finite() && ((assign42560_e57167) as f64).fract() == 0.0 { if assign42560_e57167 == 0.0 { 0.0 } else { (assign42560_e57167 * ((locals.var_t1).powf(assign42560_e57167 - 1.0) * locals.var_t1_dn2)) } } else { (assign42560_e57168 * (assign42560_e57167 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42560_e57167) as f64).is_finite() && ((assign42560_e57167) as f64).fract() == 0.0 { if assign42560_e57167 == 0.0 { 0.0 } else { (assign42560_e57167 * ((locals.var_t1).powf(assign42560_e57167 - 1.0) * locals.var_t1_dn4)) } } else { (assign42560_e57168 * (assign42560_e57167 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42560_e57167) as f64).is_finite() && ((assign42560_e57167) as f64).fract() == 0.0 { if assign42560_e57167 == 0.0 { 0.0 } else { (assign42560_e57167 * ((locals.var_t1).powf(assign42560_e57167 - 1.0) * locals.var_t1_dn5)) } } else { (assign42560_e57168 * (assign42560_e57167 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42560_e57167) as f64).is_finite() && ((assign42560_e57167) as f64).fract() == 0.0 { if assign42560_e57167 == 0.0 { 0.0 } else { (assign42560_e57167 * ((locals.var_t1).powf(assign42560_e57167 - 1.0) * locals.var_t1_dn6)) } } else { (assign42560_e57168 * (assign42560_e57167 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42560_e57167) as f64).is_finite() && ((assign42560_e57167) as f64).fract() == 0.0 { if assign42560_e57167 == 0.0 { 0.0 } else { (assign42560_e57167 * ((locals.var_t1).powf(assign42560_e57167 - 1.0) * locals.var_t1_dn7)) } } else { (assign42560_e57168 * (assign42560_e57167 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42560_e57167) as f64).is_finite() && ((assign42560_e57167) as f64).fract() == 0.0 { if assign42560_e57167 == 0.0 { 0.0 } else { (assign42560_e57167 * ((locals.var_t1).powf(assign42560_e57167 - 1.0) * locals.var_t1_dn8)) } } else { (assign42560_e57168 * (assign42560_e57167 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42560_e57167) as f64).is_finite() && ((assign42560_e57167) as f64).fract() == 0.0 { if assign42560_e57167 == 0.0 { 0.0 } else { (assign42560_e57167 * ((locals.var_t1).powf(assign42560_e57167 - 1.0) * locals.var_t1_dn9)) } } else { (assign42560_e57168 * (assign42560_e57167 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42560_e57167) as f64).is_finite() && ((assign42560_e57167) as f64).fract() == 0.0 { if assign42560_e57167 == 0.0 { 0.0 } else { (assign42560_e57167 * ((locals.var_t1).powf(assign42560_e57167 - 1.0) * locals.var_t1_dn10)) } } else { (assign42560_e57168 * (assign42560_e57167 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42560_e57167) as f64).is_finite() && ((assign42560_e57167) as f64).fract() == 0.0 { if assign42560_e57167 == 0.0 { 0.0 } else { (assign42560_e57167 * ((locals.var_t1).powf(assign42560_e57167 - 1.0) * locals.var_t1_dn11)) } } else { (assign42560_e57168 * (assign42560_e57167 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42560_e57167) as f64).is_finite() && ((assign42560_e57167) as f64).fract() == 0.0 { if assign42560_e57167 == 0.0 { 0.0 } else { (assign42560_e57167 * ((locals.var_t1).powf(assign42560_e57167 - 1.0) * locals.var_t1_dn14)) } } else { (assign42560_e57168 * (assign42560_e57167 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign42560_e57169, assign42560_e57169_d_n0, assign42560_e57169_d_n2, assign42560_e57169_d_n4, assign42560_e57169_d_n5, assign42560_e57169_d_n6, assign42560_e57169_d_n7, assign42560_e57169_d_n8, assign42560_e57169_d_n9, assign42560_e57169_d_n10, assign42560_e57169_d_n11, assign42560_e57169_d_n14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign42560_e57171;
        locals.var_t3_dn0 = assign42560_e57171_d_n0;
        locals.var_t3_dn2 = assign42560_e57171_d_n2;
        locals.var_t3_dn4 = assign42560_e57171_d_n4;
        locals.var_t3_dn5 = assign42560_e57171_d_n5;
        locals.var_t3_dn6 = assign42560_e57171_d_n6;
        locals.var_t3_dn7 = assign42560_e57171_d_n7;
        locals.var_t3_dn8 = assign42560_e57171_d_n8;
        locals.var_t3_dn9 = assign42560_e57171_d_n9;
        locals.var_t3_dn10 = assign42560_e57171_d_n10;
        locals.var_t3_dn11 = assign42560_e57171_d_n11;
        locals.var_t3_dn14 = assign42560_e57171_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign42570_e57182, assign42570_e57182_d_n0, assign42570_e57182_d_n2, assign42570_e57182_d_n4, assign42570_e57182_d_n5, assign42570_e57182_d_n6, assign42570_e57182_d_n7, assign42570_e57182_d_n8, assign42570_e57182_d_n9, assign42570_e57182_d_n10, assign42570_e57182_d_n11, assign42570_e57182_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42570_e57180: f64 = (locals.var_t1 * locals.var_t3);
        (assign42570_e57180, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42570_e57182;
        locals.var_t2_dn0 = assign42570_e57182_d_n0;
        locals.var_t2_dn2 = assign42570_e57182_d_n2;
        locals.var_t2_dn4 = assign42570_e57182_d_n4;
        locals.var_t2_dn5 = assign42570_e57182_d_n5;
        locals.var_t2_dn6 = assign42570_e57182_d_n6;
        locals.var_t2_dn7 = assign42570_e57182_d_n7;
        locals.var_t2_dn8 = assign42570_e57182_d_n8;
        locals.var_t2_dn9 = assign42570_e57182_d_n9;
        locals.var_t2_dn10 = assign42570_e57182_d_n10;
        locals.var_t2_dn11 = assign42570_e57182_d_n11;
        locals.var_t2_dn14 = assign42570_e57182_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign42580_e57193, assign42580_e57193_d_n0, assign42580_e57193_d_n2, assign42580_e57193_d_n4, assign42580_e57193_d_n5, assign42580_e57193_d_n6, assign42580_e57193_d_n7, assign42580_e57193_d_n8, assign42580_e57193_d_n9, assign42580_e57193_d_n10, assign42580_e57193_d_n11, assign42580_e57193_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42580_e57191: f64 = (1.0 + locals.var_t2);
        (assign42580_e57191, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign42580_e57193;
        locals.var_t4_dn0 = assign42580_e57193_d_n0;
        locals.var_t4_dn2 = assign42580_e57193_d_n2;
        locals.var_t4_dn4 = assign42580_e57193_d_n4;
        locals.var_t4_dn5 = assign42580_e57193_d_n5;
        locals.var_t4_dn6 = assign42580_e57193_d_n6;
        locals.var_t4_dn7 = assign42580_e57193_d_n7;
        locals.var_t4_dn8 = assign42580_e57193_d_n8;
        locals.var_t4_dn9 = assign42580_e57193_d_n9;
        locals.var_t4_dn10 = assign42580_e57193_d_n10;
        locals.var_t4_dn11 = assign42580_e57193_d_n11;
        locals.var_t4_dn14 = assign42580_e57193_d_n14;
        locals.var_t4_rv = 0.0;

        let assign42590_e57197: f64 = (10.0 * 2.220446049250313e-16);
        let assign42590_e57198: f64 = (1.0 - assign42590_e57197);
        let assign42590_e57205: f64 = (10.0 * 2.220446049250313e-16);
        let assign42590_e57206: f64 = (1.0 + assign42590_e57205);
        let assign42590_e57208: f64 = if ((assign42590_e57198 <= p.p178) && (p.p178 <= assign42590_e57206)) { 1.0 } else { 0.0 };
        locals.var_guard1057 = assign42590_e57208;
        locals.var_guard1057_rv = 0.0;

        let (assign42600_e57221, assign42600_e57221_d_n0, assign42600_e57221_d_n2, assign42600_e57221_d_n4, assign42600_e57221_d_n5, assign42600_e57221_d_n6, assign42600_e57221_d_n7, assign42600_e57221_d_n8, assign42600_e57221_d_n9, assign42600_e57221_d_n10, assign42600_e57221_d_n11, assign42600_e57221_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let assign42600_e57219: f64 = (1.0 / locals.var_t4);
        (assign42600_e57219, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42600_e57221;
        locals.var_t5_dn0 = assign42600_e57221_d_n0;
        locals.var_t5_dn2 = assign42600_e57221_d_n2;
        locals.var_t5_dn4 = assign42600_e57221_d_n4;
        locals.var_t5_dn5 = assign42600_e57221_d_n5;
        locals.var_t5_dn6 = assign42600_e57221_d_n6;
        locals.var_t5_dn7 = assign42600_e57221_d_n7;
        locals.var_t5_dn8 = assign42600_e57221_d_n8;
        locals.var_t5_dn9 = assign42600_e57221_d_n9;
        locals.var_t5_dn10 = assign42600_e57221_d_n10;
        locals.var_t5_dn11 = assign42600_e57221_d_n11;
        locals.var_t5_dn14 = assign42600_e57221_d_n14;
        locals.var_t5_rv = 0.0;

        let assign42610_e57225: f64 = (10.0 * 2.220446049250313e-16);
        let assign42610_e57226: f64 = (2.0 - assign42610_e57225);
        let assign42610_e57233: f64 = (10.0 * 2.220446049250313e-16);
        let assign42610_e57234: f64 = (2.0 + assign42610_e57233);
        let assign42610_e57236: f64 = if ((assign42610_e57226 <= p.p178) && (p.p178 <= assign42610_e57234)) { 1.0 } else { 0.0 };
        locals.var_guard1058 = assign42610_e57236;
        locals.var_guard1058_rv = 0.0;

        let (assign42620_e57253, assign42620_e57253_d_n0, assign42620_e57253_d_n2, assign42620_e57253_d_n4, assign42620_e57253_d_n5, assign42620_e57253_d_n6, assign42620_e57253_d_n7, assign42620_e57253_d_n8, assign42620_e57253_d_n9, assign42620_e57253_d_n10, assign42620_e57253_d_n11, assign42620_e57253_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1057 == 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign42620_e57250: f64 = (locals.var_t4).sqrt();
        let assign42620_e57251: f64 = (1.0 / assign42620_e57250);
        (assign42620_e57251, (-((locals.var_t4_dn0 / (2.0 * assign42620_e57250)) / (assign42620_e57250 * assign42620_e57250))), (-((locals.var_t4_dn2 / (2.0 * assign42620_e57250)) / (assign42620_e57250 * assign42620_e57250))), (-((locals.var_t4_dn4 / (2.0 * assign42620_e57250)) / (assign42620_e57250 * assign42620_e57250))), (-((locals.var_t4_dn5 / (2.0 * assign42620_e57250)) / (assign42620_e57250 * assign42620_e57250))), (-((locals.var_t4_dn6 / (2.0 * assign42620_e57250)) / (assign42620_e57250 * assign42620_e57250))), (-((locals.var_t4_dn7 / (2.0 * assign42620_e57250)) / (assign42620_e57250 * assign42620_e57250))), (-((locals.var_t4_dn8 / (2.0 * assign42620_e57250)) / (assign42620_e57250 * assign42620_e57250))), (-((locals.var_t4_dn9 / (2.0 * assign42620_e57250)) / (assign42620_e57250 * assign42620_e57250))), (-((locals.var_t4_dn10 / (2.0 * assign42620_e57250)) / (assign42620_e57250 * assign42620_e57250))), (-((locals.var_t4_dn11 / (2.0 * assign42620_e57250)) / (assign42620_e57250 * assign42620_e57250))), (-((locals.var_t4_dn14 / (2.0 * assign42620_e57250)) / (assign42620_e57250 * assign42620_e57250))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42620_e57253;
        locals.var_t5_dn0 = assign42620_e57253_d_n0;
        locals.var_t5_dn2 = assign42620_e57253_d_n2;
        locals.var_t5_dn4 = assign42620_e57253_d_n4;
        locals.var_t5_dn5 = assign42620_e57253_d_n5;
        locals.var_t5_dn6 = assign42620_e57253_d_n6;
        locals.var_t5_dn7 = assign42620_e57253_d_n7;
        locals.var_t5_dn8 = assign42620_e57253_d_n8;
        locals.var_t5_dn9 = assign42620_e57253_d_n9;
        locals.var_t5_dn10 = assign42620_e57253_d_n10;
        locals.var_t5_dn11 = assign42620_e57253_d_n11;
        locals.var_t5_dn14 = assign42620_e57253_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign42630_e57280, assign42630_e57280_d_n0, assign42630_e57280_d_n2, assign42630_e57280_d_n4, assign42630_e57280_d_n5, assign42630_e57280_d_n6, assign42630_e57280_d_n7, assign42630_e57280_d_n8, assign42630_e57280_d_n9, assign42630_e57280_d_n10, assign42630_e57280_d_n11, assign42630_e57280_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1057 == 0.0)) && (locals.var_guard1058 == 0.0)) {
        let (assign42630_e57278, assign42630_e57278_d_n0, assign42630_e57278_d_n2, assign42630_e57278_d_n4, assign42630_e57278_d_n5, assign42630_e57278_d_n6, assign42630_e57278_d_n7, assign42630_e57278_d_n8, assign42630_e57278_d_n9, assign42630_e57278_d_n10, assign42630_e57278_d_n11, assign42630_e57278_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42630_e57272: f64 = (-1.0);
                let assign42630_e57274: f64 = (assign42630_e57272 / p.p178);
                let assign42630_e57276: f64 = (assign42630_e57274 - 1.0);
                let assign42630_e57277: f64 = (locals.var_t4).powf(assign42630_e57276);
                (assign42630_e57277, if 0.0 == 0.0 && ((assign42630_e57276) as f64).is_finite() && ((assign42630_e57276) as f64).fract() == 0.0 { if assign42630_e57276 == 0.0 { 0.0 } else { (assign42630_e57276 * ((locals.var_t4).powf(assign42630_e57276 - 1.0) * locals.var_t4_dn0)) } } else { (assign42630_e57277 * (assign42630_e57276 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42630_e57276) as f64).is_finite() && ((assign42630_e57276) as f64).fract() == 0.0 { if assign42630_e57276 == 0.0 { 0.0 } else { (assign42630_e57276 * ((locals.var_t4).powf(assign42630_e57276 - 1.0) * locals.var_t4_dn2)) } } else { (assign42630_e57277 * (assign42630_e57276 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42630_e57276) as f64).is_finite() && ((assign42630_e57276) as f64).fract() == 0.0 { if assign42630_e57276 == 0.0 { 0.0 } else { (assign42630_e57276 * ((locals.var_t4).powf(assign42630_e57276 - 1.0) * locals.var_t4_dn4)) } } else { (assign42630_e57277 * (assign42630_e57276 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42630_e57276) as f64).is_finite() && ((assign42630_e57276) as f64).fract() == 0.0 { if assign42630_e57276 == 0.0 { 0.0 } else { (assign42630_e57276 * ((locals.var_t4).powf(assign42630_e57276 - 1.0) * locals.var_t4_dn5)) } } else { (assign42630_e57277 * (assign42630_e57276 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42630_e57276) as f64).is_finite() && ((assign42630_e57276) as f64).fract() == 0.0 { if assign42630_e57276 == 0.0 { 0.0 } else { (assign42630_e57276 * ((locals.var_t4).powf(assign42630_e57276 - 1.0) * locals.var_t4_dn6)) } } else { (assign42630_e57277 * (assign42630_e57276 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42630_e57276) as f64).is_finite() && ((assign42630_e57276) as f64).fract() == 0.0 { if assign42630_e57276 == 0.0 { 0.0 } else { (assign42630_e57276 * ((locals.var_t4).powf(assign42630_e57276 - 1.0) * locals.var_t4_dn7)) } } else { (assign42630_e57277 * (assign42630_e57276 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42630_e57276) as f64).is_finite() && ((assign42630_e57276) as f64).fract() == 0.0 { if assign42630_e57276 == 0.0 { 0.0 } else { (assign42630_e57276 * ((locals.var_t4).powf(assign42630_e57276 - 1.0) * locals.var_t4_dn8)) } } else { (assign42630_e57277 * (assign42630_e57276 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42630_e57276) as f64).is_finite() && ((assign42630_e57276) as f64).fract() == 0.0 { if assign42630_e57276 == 0.0 { 0.0 } else { (assign42630_e57276 * ((locals.var_t4).powf(assign42630_e57276 - 1.0) * locals.var_t4_dn9)) } } else { (assign42630_e57277 * (assign42630_e57276 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42630_e57276) as f64).is_finite() && ((assign42630_e57276) as f64).fract() == 0.0 { if assign42630_e57276 == 0.0 { 0.0 } else { (assign42630_e57276 * ((locals.var_t4).powf(assign42630_e57276 - 1.0) * locals.var_t4_dn10)) } } else { (assign42630_e57277 * (assign42630_e57276 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42630_e57276) as f64).is_finite() && ((assign42630_e57276) as f64).fract() == 0.0 { if assign42630_e57276 == 0.0 { 0.0 } else { (assign42630_e57276 * ((locals.var_t4).powf(assign42630_e57276 - 1.0) * locals.var_t4_dn11)) } } else { (assign42630_e57277 * (assign42630_e57276 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42630_e57276) as f64).is_finite() && ((assign42630_e57276) as f64).fract() == 0.0 { if assign42630_e57276 == 0.0 { 0.0 } else { (assign42630_e57276 * ((locals.var_t4).powf(assign42630_e57276 - 1.0) * locals.var_t4_dn14)) } } else { (assign42630_e57277 * (assign42630_e57276 * (locals.var_t4_dn14 / locals.var_t4))) },)
            }
        };
        (assign42630_e57278, assign42630_e57278_d_n0, assign42630_e57278_d_n2, assign42630_e57278_d_n4, assign42630_e57278_d_n5, assign42630_e57278_d_n6, assign42630_e57278_d_n7, assign42630_e57278_d_n8, assign42630_e57278_d_n9, assign42630_e57278_d_n10, assign42630_e57278_d_n11, assign42630_e57278_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign42630_e57280;
        locals.var_t6_dn0 = assign42630_e57280_d_n0;
        locals.var_t6_dn2 = assign42630_e57280_d_n2;
        locals.var_t6_dn4 = assign42630_e57280_d_n4;
        locals.var_t6_dn5 = assign42630_e57280_d_n5;
        locals.var_t6_dn6 = assign42630_e57280_d_n6;
        locals.var_t6_dn7 = assign42630_e57280_d_n7;
        locals.var_t6_dn8 = assign42630_e57280_d_n8;
        locals.var_t6_dn9 = assign42630_e57280_d_n9;
        locals.var_t6_dn10 = assign42630_e57280_d_n10;
        locals.var_t6_dn11 = assign42630_e57280_d_n11;
        locals.var_t6_dn14 = assign42630_e57280_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign42640_e57297, assign42640_e57297_d_n0, assign42640_e57297_d_n2, assign42640_e57297_d_n4, assign42640_e57297_d_n5, assign42640_e57297_d_n6, assign42640_e57297_d_n7, assign42640_e57297_d_n8, assign42640_e57297_d_n9, assign42640_e57297_d_n10, assign42640_e57297_d_n11, assign42640_e57297_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1057 == 0.0)) && (locals.var_guard1058 == 0.0)) {
        let assign42640_e57295: f64 = (locals.var_t4 * locals.var_t6);
        (assign42640_e57295, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42640_e57297;
        locals.var_t5_dn0 = assign42640_e57297_d_n0;
        locals.var_t5_dn2 = assign42640_e57297_d_n2;
        locals.var_t5_dn4 = assign42640_e57297_d_n4;
        locals.var_t5_dn5 = assign42640_e57297_d_n5;
        locals.var_t5_dn6 = assign42640_e57297_d_n6;
        locals.var_t5_dn7 = assign42640_e57297_d_n7;
        locals.var_t5_dn8 = assign42640_e57297_d_n8;
        locals.var_t5_dn9 = assign42640_e57297_d_n9;
        locals.var_t5_dn10 = assign42640_e57297_d_n10;
        locals.var_t5_dn11 = assign42640_e57297_d_n11;
        locals.var_t5_dn14 = assign42640_e57297_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign42650_e57308, assign42650_e57308_d_n0, assign42650_e57308_d_n2, assign42650_e57308_d_n4, assign42650_e57308_d_n5, assign42650_e57308_d_n6, assign42650_e57308_d_n7, assign42650_e57308_d_n8, assign42650_e57308_d_n9, assign42650_e57308_d_n10, assign42650_e57308_d_n11, assign42650_e57308_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign42650_e57306: f64 = (locals.var_muun * locals.var_t5);
        (assign42650_e57306, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn11 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn11)), ((locals.var_muun_dn14 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    }
};
        locals.var_mu = assign42650_e57308;
        locals.var_mu_dn0 = assign42650_e57308_d_n0;
        locals.var_mu_dn2 = assign42650_e57308_d_n2;
        locals.var_mu_dn4 = assign42650_e57308_d_n4;
        locals.var_mu_dn5 = assign42650_e57308_d_n5;
        locals.var_mu_dn6 = assign42650_e57308_d_n6;
        locals.var_mu_dn7 = assign42650_e57308_d_n7;
        locals.var_mu_dn8 = assign42650_e57308_d_n8;
        locals.var_mu_dn9 = assign42650_e57308_d_n9;
        locals.var_mu_dn10 = assign42650_e57308_d_n10;
        locals.var_mu_dn11 = assign42650_e57308_d_n11;
        locals.var_mu_dn14 = assign42650_e57308_d_n14;
        locals.var_mu_rv = 0.0;

        let (assign42660_e57317, assign42660_e57317_d_n0, assign42660_e57317_d_n2, assign42660_e57317_d_n4, assign42660_e57317_d_n5, assign42660_e57317_d_n6, assign42660_e57317_d_n7, assign42660_e57317_d_n8, assign42660_e57317_d_n9, assign42660_e57317_d_n10, assign42660_e57317_d_n11, assign42660_e57317_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    } else {
        (locals.var_mu_acc, locals.var_mu_acc_dn0, locals.var_mu_acc_dn2, locals.var_mu_acc_dn4, locals.var_mu_acc_dn5, locals.var_mu_acc_dn6, locals.var_mu_acc_dn7, locals.var_mu_acc_dn8, locals.var_mu_acc_dn9, locals.var_mu_acc_dn10, locals.var_mu_acc_dn11, locals.var_mu_acc_dn14,)
    }
};
        locals.var_mu_acc = assign42660_e57317;
        locals.var_mu_acc_dn0 = assign42660_e57317_d_n0;
        locals.var_mu_acc_dn2 = assign42660_e57317_d_n2;
        locals.var_mu_acc_dn4 = assign42660_e57317_d_n4;
        locals.var_mu_acc_dn5 = assign42660_e57317_d_n5;
        locals.var_mu_acc_dn6 = assign42660_e57317_d_n6;
        locals.var_mu_acc_dn7 = assign42660_e57317_d_n7;
        locals.var_mu_acc_dn8 = assign42660_e57317_d_n8;
        locals.var_mu_acc_dn9 = assign42660_e57317_d_n9;
        locals.var_mu_acc_dn10 = assign42660_e57317_d_n10;
        locals.var_mu_acc_dn11 = assign42660_e57317_d_n11;
        locals.var_mu_acc_dn14 = assign42660_e57317_d_n14;
        locals.var_mu_acc_rv = 0.0;

        let (assign42670_e57326, assign42670_e57326_d_n0, assign42670_e57326_d_n2, assign42670_e57326_d_n4, assign42670_e57326_d_n5, assign42670_e57326_d_n6, assign42670_e57326_d_n7, assign42670_e57326_d_n8, assign42670_e57326_d_n9, assign42670_e57326_d_n10, assign42670_e57326_d_n11, assign42670_e57326_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    } else {
        (locals.var_ey_acc, locals.var_ey_acc_dn0, locals.var_ey_acc_dn2, locals.var_ey_acc_dn4, locals.var_ey_acc_dn5, locals.var_ey_acc_dn6, locals.var_ey_acc_dn7, locals.var_ey_acc_dn8, locals.var_ey_acc_dn9, locals.var_ey_acc_dn10, locals.var_ey_acc_dn11, locals.var_ey_acc_dn14,)
    }
};
        locals.var_ey_acc = assign42670_e57326;
        locals.var_ey_acc_dn0 = assign42670_e57326_d_n0;
        locals.var_ey_acc_dn2 = assign42670_e57326_d_n2;
        locals.var_ey_acc_dn4 = assign42670_e57326_d_n4;
        locals.var_ey_acc_dn5 = assign42670_e57326_d_n5;
        locals.var_ey_acc_dn6 = assign42670_e57326_d_n6;
        locals.var_ey_acc_dn7 = assign42670_e57326_d_n7;
        locals.var_ey_acc_dn8 = assign42670_e57326_d_n8;
        locals.var_ey_acc_dn9 = assign42670_e57326_d_n9;
        locals.var_ey_acc_dn10 = assign42670_e57326_d_n10;
        locals.var_ey_acc_dn11 = assign42670_e57326_d_n11;
        locals.var_ey_acc_dn14 = assign42670_e57326_d_n14;
        locals.var_ey_acc_rv = 0.0;

        let (assign42680_e57335, assign42680_e57335_d_n0, assign42680_e57335_d_n2, assign42680_e57335_d_n4, assign42680_e57335_d_n5, assign42680_e57335_d_n6, assign42680_e57335_d_n7, assign42680_e57335_d_n8, assign42680_e57335_d_n9, assign42680_e57335_d_n10, assign42680_e57335_d_n11, assign42680_e57335_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign42680_e57335;
        locals.var_vds_res_dn0 = assign42680_e57335_d_n0;
        locals.var_vds_res_dn2 = assign42680_e57335_d_n2;
        locals.var_vds_res_dn4 = assign42680_e57335_d_n4;
        locals.var_vds_res_dn5 = assign42680_e57335_d_n5;
        locals.var_vds_res_dn6 = assign42680_e57335_d_n6;
        locals.var_vds_res_dn7 = assign42680_e57335_d_n7;
        locals.var_vds_res_dn8 = assign42680_e57335_d_n8;
        locals.var_vds_res_dn9 = assign42680_e57335_d_n9;
        locals.var_vds_res_dn10 = assign42680_e57335_d_n10;
        locals.var_vds_res_dn11 = assign42680_e57335_d_n11;
        locals.var_vds_res_dn14 = assign42680_e57335_d_n14;
        locals.var_vds_res_rv = 0.0;

        let assign42690_e57338: f64 = if locals.var_vdsorg > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1059 = assign42690_e57338;
        locals.var_guard1059_rv = 0.0;

        let (assign42700_e57353, assign42700_e57353_d_n0, assign42700_e57353_d_n2, assign42700_e57353_d_n4, assign42700_e57353_d_n5, assign42700_e57353_d_n6, assign42700_e57353_d_n7, assign42700_e57353_d_n8, assign42700_e57353_d_n9, assign42700_e57353_d_n10, assign42700_e57353_d_n11, assign42700_e57353_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let assign42700_e57349: f64 = (locals.var_vbsc + locals.var_beta_inv);
        let assign42700_e57351: f64 = (assign42700_e57349 * p.p396);
        (assign42700_e57351, ((locals.var_vbsc_dn0 + locals.var_beta_inv_dn0) * p.p396), ((locals.var_vbsc_dn2 + locals.var_beta_inv_dn2) * p.p396), ((locals.var_vbsc_dn4 + locals.var_beta_inv_dn4) * p.p396), ((locals.var_vbsc_dn5 + locals.var_beta_inv_dn5) * p.p396), ((locals.var_vbsc_dn6 + locals.var_beta_inv_dn6) * p.p396), ((locals.var_vbsc_dn7 + locals.var_beta_inv_dn7) * p.p396), ((locals.var_vbsc_dn8 + locals.var_beta_inv_dn8) * p.p396), ((locals.var_vbsc_dn9 + locals.var_beta_inv_dn9) * p.p396), ((locals.var_vbsc_dn10 + locals.var_beta_inv_dn10) * p.p396), ((locals.var_vbsc_dn11 + locals.var_beta_inv_dn11) * p.p396), ((locals.var_vbsc_dn14 + locals.var_beta_inv_dn14) * p.p396),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign42700_e57353;
        locals.var_t10_dn0 = assign42700_e57353_d_n0;
        locals.var_t10_dn2 = assign42700_e57353_d_n2;
        locals.var_t10_dn4 = assign42700_e57353_d_n4;
        locals.var_t10_dn5 = assign42700_e57353_d_n5;
        locals.var_t10_dn6 = assign42700_e57353_d_n6;
        locals.var_t10_dn7 = assign42700_e57353_d_n7;
        locals.var_t10_dn8 = assign42700_e57353_d_n8;
        locals.var_t10_dn9 = assign42700_e57353_d_n9;
        locals.var_t10_dn10 = assign42700_e57353_d_n10;
        locals.var_t10_dn11 = assign42700_e57353_d_n11;
        locals.var_t10_dn14 = assign42700_e57353_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign42710_e57370, assign42710_e57370_d_n0, assign42710_e57370_d_n2, assign42710_e57370_d_n4, assign42710_e57370_d_n5, assign42710_e57370_d_n6, assign42710_e57370_d_n7, assign42710_e57370_d_n8, assign42710_e57370_d_n9, assign42710_e57370_d_n10, assign42710_e57370_d_n11, assign42710_e57370_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let assign42710_e57366: f64 = (locals.var_vgp - locals.var_t10);
        let assign42710_e57367: f64 = (locals.var_c2_q_ndepm_esi_cox_inv2 * assign42710_e57366);
        let assign42710_e57368: f64 = (1.0 + assign42710_e57367);
        (assign42710_e57368, ((locals.var_c2_q_ndepm_esi_cox_inv2_dn0 * assign42710_e57366) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn0 - locals.var_t10_dn0))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn2 * assign42710_e57366) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn2 - locals.var_t10_dn2))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn4 * assign42710_e57366) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn4 - locals.var_t10_dn4))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn5 * assign42710_e57366) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn5 - locals.var_t10_dn5))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn6 * assign42710_e57366) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn6 - locals.var_t10_dn6))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn7 * assign42710_e57366) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn7 - locals.var_t10_dn7))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn8 * assign42710_e57366) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn8 - locals.var_t10_dn8))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn9 * assign42710_e57366) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn9 - locals.var_t10_dn9))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn10 * assign42710_e57366) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn10 - locals.var_t10_dn10))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn11 * assign42710_e57366) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn11 - locals.var_t10_dn11))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn14 * assign42710_e57366) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn14 - locals.var_t10_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign42710_e57370;
        locals.var_t4_dn0 = assign42710_e57370_d_n0;
        locals.var_t4_dn2 = assign42710_e57370_d_n2;
        locals.var_t4_dn4 = assign42710_e57370_d_n4;
        locals.var_t4_dn5 = assign42710_e57370_d_n5;
        locals.var_t4_dn6 = assign42710_e57370_d_n6;
        locals.var_t4_dn7 = assign42710_e57370_d_n7;
        locals.var_t4_dn8 = assign42710_e57370_d_n8;
        locals.var_t4_dn9 = assign42710_e57370_d_n9;
        locals.var_t4_dn10 = assign42710_e57370_d_n10;
        locals.var_t4_dn11 = assign42710_e57370_d_n11;
        locals.var_t4_dn14 = assign42710_e57370_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign42720_e57383, assign42720_e57383_d_n0, assign42720_e57383_d_n2, assign42720_e57383_d_n4, assign42720_e57383_d_n5, assign42720_e57383_d_n6, assign42720_e57383_d_n7, assign42720_e57383_d_n8, assign42720_e57383_d_n9, assign42720_e57383_d_n10, assign42720_e57383_d_n11, assign42720_e57383_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let assign42720_e57381: f64 = (1.0 + locals.var_c2_q_ndepm_esi_cox_inv2);
        (assign42720_e57381, locals.var_c2_q_ndepm_esi_cox_inv2_dn0, locals.var_c2_q_ndepm_esi_cox_inv2_dn2, locals.var_c2_q_ndepm_esi_cox_inv2_dn4, locals.var_c2_q_ndepm_esi_cox_inv2_dn5, locals.var_c2_q_ndepm_esi_cox_inv2_dn6, locals.var_c2_q_ndepm_esi_cox_inv2_dn7, locals.var_c2_q_ndepm_esi_cox_inv2_dn8, locals.var_c2_q_ndepm_esi_cox_inv2_dn9, locals.var_c2_q_ndepm_esi_cox_inv2_dn10, locals.var_c2_q_ndepm_esi_cox_inv2_dn11, locals.var_c2_q_ndepm_esi_cox_inv2_dn14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42720_e57383;
        locals.var_t5_dn0 = assign42720_e57383_d_n0;
        locals.var_t5_dn2 = assign42720_e57383_d_n2;
        locals.var_t5_dn4 = assign42720_e57383_d_n4;
        locals.var_t5_dn5 = assign42720_e57383_d_n5;
        locals.var_t5_dn6 = assign42720_e57383_d_n6;
        locals.var_t5_dn7 = assign42720_e57383_d_n7;
        locals.var_t5_dn8 = assign42720_e57383_d_n8;
        locals.var_t5_dn9 = assign42720_e57383_d_n9;
        locals.var_t5_dn10 = assign42720_e57383_d_n10;
        locals.var_t5_dn11 = assign42720_e57383_d_n11;
        locals.var_t5_dn14 = assign42720_e57383_d_n14;
        locals.var_t5_rv = 0.0;

        let assign42730_e57387: f64 = locals.var_t5;
        let assign42730_e57392: f64 = if ((locals.var_t4 < assign42730_e57387) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1060 = assign42730_e57392;
        locals.var_guard1060_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_148(
        locals: &mut StampLocals,
    ) {
        let (assign42740_e57409, assign42740_e57409_d_n0, assign42740_e57409_d_n2, assign42740_e57409_d_n4, assign42740_e57409_d_n5, assign42740_e57409_d_n6, assign42740_e57409_d_n7, assign42740_e57409_d_n8, assign42740_e57409_d_n9, assign42740_e57409_d_n10, assign42740_e57409_d_n11, assign42740_e57409_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign42740_e57405: f64 = locals.var_t5;
        let assign42740_e57407: f64 = (assign42740_e57405 - locals.var_t4);
        (assign42740_e57407, (locals.var_t5_dn0 - locals.var_t4_dn0), (locals.var_t5_dn2 - locals.var_t4_dn2), (locals.var_t5_dn4 - locals.var_t4_dn4), (locals.var_t5_dn5 - locals.var_t4_dn5), (locals.var_t5_dn6 - locals.var_t4_dn6), (locals.var_t5_dn7 - locals.var_t4_dn7), (locals.var_t5_dn8 - locals.var_t4_dn8), (locals.var_t5_dn9 - locals.var_t4_dn9), (locals.var_t5_dn10 - locals.var_t4_dn10), (locals.var_t5_dn11 - locals.var_t4_dn11), (locals.var_t5_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign42740_e57409;
        locals.var_tmf1_dn0 = assign42740_e57409_d_n0;
        locals.var_tmf1_dn2 = assign42740_e57409_d_n2;
        locals.var_tmf1_dn4 = assign42740_e57409_d_n4;
        locals.var_tmf1_dn5 = assign42740_e57409_d_n5;
        locals.var_tmf1_dn6 = assign42740_e57409_d_n6;
        locals.var_tmf1_dn7 = assign42740_e57409_d_n7;
        locals.var_tmf1_dn8 = assign42740_e57409_d_n8;
        locals.var_tmf1_dn9 = assign42740_e57409_d_n9;
        locals.var_tmf1_dn10 = assign42740_e57409_d_n10;
        locals.var_tmf1_dn11 = assign42740_e57409_d_n11;
        locals.var_tmf1_dn14 = assign42740_e57409_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign42750_e57424, assign42750_e57424_d_n0, assign42750_e57424_d_n2, assign42750_e57424_d_n4, assign42750_e57424_d_n5, assign42750_e57424_d_n6, assign42750_e57424_d_n7, assign42750_e57424_d_n8, assign42750_e57424_d_n9, assign42750_e57424_d_n10, assign42750_e57424_d_n11, assign42750_e57424_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign42750_e57422: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign42750_e57422, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign42750_e57424;
        locals.var_x2_dn0 = assign42750_e57424_d_n0;
        locals.var_x2_dn2 = assign42750_e57424_d_n2;
        locals.var_x2_dn4 = assign42750_e57424_d_n4;
        locals.var_x2_dn5 = assign42750_e57424_d_n5;
        locals.var_x2_dn6 = assign42750_e57424_d_n6;
        locals.var_x2_dn7 = assign42750_e57424_d_n7;
        locals.var_x2_dn8 = assign42750_e57424_d_n8;
        locals.var_x2_dn9 = assign42750_e57424_d_n9;
        locals.var_x2_dn10 = assign42750_e57424_d_n10;
        locals.var_x2_dn11 = assign42750_e57424_d_n11;
        locals.var_x2_dn14 = assign42750_e57424_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign42760_e57439, assign42760_e57439_d_n0, assign42760_e57439_d_n2, assign42760_e57439_d_n4, assign42760_e57439_d_n5, assign42760_e57439_d_n6, assign42760_e57439_d_n7, assign42760_e57439_d_n8, assign42760_e57439_d_n9, assign42760_e57439_d_n10, assign42760_e57439_d_n11, assign42760_e57439_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign42760_e57437: f64 = (locals.var_t5 * locals.var_t5);
        (assign42760_e57437, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)), ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)), ((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign42760_e57439;
        locals.var_xmax2_dn0 = assign42760_e57439_d_n0;
        locals.var_xmax2_dn2 = assign42760_e57439_d_n2;
        locals.var_xmax2_dn4 = assign42760_e57439_d_n4;
        locals.var_xmax2_dn5 = assign42760_e57439_d_n5;
        locals.var_xmax2_dn6 = assign42760_e57439_d_n6;
        locals.var_xmax2_dn7 = assign42760_e57439_d_n7;
        locals.var_xmax2_dn8 = assign42760_e57439_d_n8;
        locals.var_xmax2_dn9 = assign42760_e57439_d_n9;
        locals.var_xmax2_dn10 = assign42760_e57439_d_n10;
        locals.var_xmax2_dn11 = assign42760_e57439_d_n11;
        locals.var_xmax2_dn14 = assign42760_e57439_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign42770_e57452, assign42770_e57452_d_n0, assign42770_e57452_d_n2, assign42770_e57452_d_n4, assign42770_e57452_d_n5, assign42770_e57452_d_n6, assign42770_e57452_d_n7, assign42770_e57452_d_n8, assign42770_e57452_d_n9, assign42770_e57452_d_n10, assign42770_e57452_d_n11, assign42770_e57452_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign42770_e57452;
        locals.var_xp_dn0 = assign42770_e57452_d_n0;
        locals.var_xp_dn2 = assign42770_e57452_d_n2;
        locals.var_xp_dn4 = assign42770_e57452_d_n4;
        locals.var_xp_dn5 = assign42770_e57452_d_n5;
        locals.var_xp_dn6 = assign42770_e57452_d_n6;
        locals.var_xp_dn7 = assign42770_e57452_d_n7;
        locals.var_xp_dn8 = assign42770_e57452_d_n8;
        locals.var_xp_dn9 = assign42770_e57452_d_n9;
        locals.var_xp_dn10 = assign42770_e57452_d_n10;
        locals.var_xp_dn11 = assign42770_e57452_d_n11;
        locals.var_xp_dn14 = assign42770_e57452_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign42780_e57465, assign42780_e57465_d_n0, assign42780_e57465_d_n2, assign42780_e57465_d_n4, assign42780_e57465_d_n5, assign42780_e57465_d_n6, assign42780_e57465_d_n7, assign42780_e57465_d_n8, assign42780_e57465_d_n9, assign42780_e57465_d_n10, assign42780_e57465_d_n11, assign42780_e57465_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign42780_e57465;
        locals.var_xmp_dn0 = assign42780_e57465_d_n0;
        locals.var_xmp_dn2 = assign42780_e57465_d_n2;
        locals.var_xmp_dn4 = assign42780_e57465_d_n4;
        locals.var_xmp_dn5 = assign42780_e57465_d_n5;
        locals.var_xmp_dn6 = assign42780_e57465_d_n6;
        locals.var_xmp_dn7 = assign42780_e57465_d_n7;
        locals.var_xmp_dn8 = assign42780_e57465_d_n8;
        locals.var_xmp_dn9 = assign42780_e57465_d_n9;
        locals.var_xmp_dn10 = assign42780_e57465_d_n10;
        locals.var_xmp_dn11 = assign42780_e57465_d_n11;
        locals.var_xmp_dn14 = assign42780_e57465_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign42790_e57478,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign42790_e57478;
        locals.var_m0_rv = 0.0;

        let (assign42800_e57491,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42800_e57491;
        locals.var_mm_rv = 0.0;

        let (assign42810_e57504, assign42810_e57504_d_n0, assign42810_e57504_d_n2, assign42810_e57504_d_n4, assign42810_e57504_d_n5, assign42810_e57504_d_n6, assign42810_e57504_d_n7, assign42810_e57504_d_n8, assign42810_e57504_d_n9, assign42810_e57504_d_n10, assign42810_e57504_d_n11, assign42810_e57504_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign42810_e57504;
        locals.var_arg_dn0 = assign42810_e57504_d_n0;
        locals.var_arg_dn2 = assign42810_e57504_d_n2;
        locals.var_arg_dn4 = assign42810_e57504_d_n4;
        locals.var_arg_dn5 = assign42810_e57504_d_n5;
        locals.var_arg_dn6 = assign42810_e57504_d_n6;
        locals.var_arg_dn7 = assign42810_e57504_d_n7;
        locals.var_arg_dn8 = assign42810_e57504_d_n8;
        locals.var_arg_dn9 = assign42810_e57504_d_n9;
        locals.var_arg_dn10 = assign42810_e57504_d_n10;
        locals.var_arg_dn11 = assign42810_e57504_d_n11;
        locals.var_arg_dn14 = assign42810_e57504_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign42820_e57517, assign42820_e57517_d_n0, assign42820_e57517_d_n2, assign42820_e57517_d_n4, assign42820_e57517_d_n5, assign42820_e57517_d_n6, assign42820_e57517_d_n7, assign42820_e57517_d_n8, assign42820_e57517_d_n9, assign42820_e57517_d_n10, assign42820_e57517_d_n11, assign42820_e57517_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign42820_e57517;
        locals.var_dnm_dn0 = assign42820_e57517_d_n0;
        locals.var_dnm_dn2 = assign42820_e57517_d_n2;
        locals.var_dnm_dn4 = assign42820_e57517_d_n4;
        locals.var_dnm_dn5 = assign42820_e57517_d_n5;
        locals.var_dnm_dn6 = assign42820_e57517_d_n6;
        locals.var_dnm_dn7 = assign42820_e57517_d_n7;
        locals.var_dnm_dn8 = assign42820_e57517_d_n8;
        locals.var_dnm_dn9 = assign42820_e57517_d_n9;
        locals.var_dnm_dn10 = assign42820_e57517_d_n10;
        locals.var_dnm_dn11 = assign42820_e57517_d_n11;
        locals.var_dnm_dn14 = assign42820_e57517_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign42830_e57532, assign42830_e57532_d_n0, assign42830_e57532_d_n2, assign42830_e57532_d_n4, assign42830_e57532_d_n5, assign42830_e57532_d_n6, assign42830_e57532_d_n7, assign42830_e57532_d_n8, assign42830_e57532_d_n9, assign42830_e57532_d_n10, assign42830_e57532_d_n11, assign42830_e57532_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign42830_e57530: f64 = (locals.var_xp * locals.var_x2);
        (assign42830_e57530, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign42830_e57532;
        locals.var_xp_dn0 = assign42830_e57532_d_n0;
        locals.var_xp_dn2 = assign42830_e57532_d_n2;
        locals.var_xp_dn4 = assign42830_e57532_d_n4;
        locals.var_xp_dn5 = assign42830_e57532_d_n5;
        locals.var_xp_dn6 = assign42830_e57532_d_n6;
        locals.var_xp_dn7 = assign42830_e57532_d_n7;
        locals.var_xp_dn8 = assign42830_e57532_d_n8;
        locals.var_xp_dn9 = assign42830_e57532_d_n9;
        locals.var_xp_dn10 = assign42830_e57532_d_n10;
        locals.var_xp_dn11 = assign42830_e57532_d_n11;
        locals.var_xp_dn14 = assign42830_e57532_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign42840_e57547, assign42840_e57547_d_n0, assign42840_e57547_d_n2, assign42840_e57547_d_n4, assign42840_e57547_d_n5, assign42840_e57547_d_n6, assign42840_e57547_d_n7, assign42840_e57547_d_n8, assign42840_e57547_d_n9, assign42840_e57547_d_n10, assign42840_e57547_d_n11, assign42840_e57547_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign42840_e57545: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign42840_e57545, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign42840_e57547;
        locals.var_xmp_dn0 = assign42840_e57547_d_n0;
        locals.var_xmp_dn2 = assign42840_e57547_d_n2;
        locals.var_xmp_dn4 = assign42840_e57547_d_n4;
        locals.var_xmp_dn5 = assign42840_e57547_d_n5;
        locals.var_xmp_dn6 = assign42840_e57547_d_n6;
        locals.var_xmp_dn7 = assign42840_e57547_d_n7;
        locals.var_xmp_dn8 = assign42840_e57547_d_n8;
        locals.var_xmp_dn9 = assign42840_e57547_d_n9;
        locals.var_xmp_dn10 = assign42840_e57547_d_n10;
        locals.var_xmp_dn11 = assign42840_e57547_d_n11;
        locals.var_xmp_dn14 = assign42840_e57547_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign42850_e57562, assign42850_e57562_d_n0, assign42850_e57562_d_n2, assign42850_e57562_d_n4, assign42850_e57562_d_n5, assign42850_e57562_d_n6, assign42850_e57562_d_n7, assign42850_e57562_d_n8, assign42850_e57562_d_n9, assign42850_e57562_d_n10, assign42850_e57562_d_n11, assign42850_e57562_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign42850_e57560: f64 = (locals.var_xp * locals.var_x2);
        (assign42850_e57560, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign42850_e57562;
        locals.var_xp_dn0 = assign42850_e57562_d_n0;
        locals.var_xp_dn2 = assign42850_e57562_d_n2;
        locals.var_xp_dn4 = assign42850_e57562_d_n4;
        locals.var_xp_dn5 = assign42850_e57562_d_n5;
        locals.var_xp_dn6 = assign42850_e57562_d_n6;
        locals.var_xp_dn7 = assign42850_e57562_d_n7;
        locals.var_xp_dn8 = assign42850_e57562_d_n8;
        locals.var_xp_dn9 = assign42850_e57562_d_n9;
        locals.var_xp_dn10 = assign42850_e57562_d_n10;
        locals.var_xp_dn11 = assign42850_e57562_d_n11;
        locals.var_xp_dn14 = assign42850_e57562_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign42860_e57577, assign42860_e57577_d_n0, assign42860_e57577_d_n2, assign42860_e57577_d_n4, assign42860_e57577_d_n5, assign42860_e57577_d_n6, assign42860_e57577_d_n7, assign42860_e57577_d_n8, assign42860_e57577_d_n9, assign42860_e57577_d_n10, assign42860_e57577_d_n11, assign42860_e57577_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign42860_e57575: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign42860_e57575, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign42860_e57577;
        locals.var_xmp_dn0 = assign42860_e57577_d_n0;
        locals.var_xmp_dn2 = assign42860_e57577_d_n2;
        locals.var_xmp_dn4 = assign42860_e57577_d_n4;
        locals.var_xmp_dn5 = assign42860_e57577_d_n5;
        locals.var_xmp_dn6 = assign42860_e57577_d_n6;
        locals.var_xmp_dn7 = assign42860_e57577_d_n7;
        locals.var_xmp_dn8 = assign42860_e57577_d_n8;
        locals.var_xmp_dn9 = assign42860_e57577_d_n9;
        locals.var_xmp_dn10 = assign42860_e57577_d_n10;
        locals.var_xmp_dn11 = assign42860_e57577_d_n11;
        locals.var_xmp_dn14 = assign42860_e57577_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign42870_e57592, assign42870_e57592_d_n0, assign42870_e57592_d_n2, assign42870_e57592_d_n4, assign42870_e57592_d_n5, assign42870_e57592_d_n6, assign42870_e57592_d_n7, assign42870_e57592_d_n8, assign42870_e57592_d_n9, assign42870_e57592_d_n10, assign42870_e57592_d_n11, assign42870_e57592_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign42870_e57590: f64 = (locals.var_xp + locals.var_xmp);
        (assign42870_e57590, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign42870_e57592;
        locals.var_arg_dn0 = assign42870_e57592_d_n0;
        locals.var_arg_dn2 = assign42870_e57592_d_n2;
        locals.var_arg_dn4 = assign42870_e57592_d_n4;
        locals.var_arg_dn5 = assign42870_e57592_d_n5;
        locals.var_arg_dn6 = assign42870_e57592_d_n6;
        locals.var_arg_dn7 = assign42870_e57592_d_n7;
        locals.var_arg_dn8 = assign42870_e57592_d_n8;
        locals.var_arg_dn9 = assign42870_e57592_d_n9;
        locals.var_arg_dn10 = assign42870_e57592_d_n10;
        locals.var_arg_dn11 = assign42870_e57592_d_n11;
        locals.var_arg_dn14 = assign42870_e57592_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign42880_e57605, assign42880_e57605_d_n0, assign42880_e57605_d_n2, assign42880_e57605_d_n4, assign42880_e57605_d_n5, assign42880_e57605_d_n6, assign42880_e57605_d_n7, assign42880_e57605_d_n8, assign42880_e57605_d_n9, assign42880_e57605_d_n10, assign42880_e57605_d_n11, assign42880_e57605_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign42880_e57605;
        locals.var_dnm_dn0 = assign42880_e57605_d_n0;
        locals.var_dnm_dn2 = assign42880_e57605_d_n2;
        locals.var_dnm_dn4 = assign42880_e57605_d_n4;
        locals.var_dnm_dn5 = assign42880_e57605_d_n5;
        locals.var_dnm_dn6 = assign42880_e57605_d_n6;
        locals.var_dnm_dn7 = assign42880_e57605_d_n7;
        locals.var_dnm_dn8 = assign42880_e57605_d_n8;
        locals.var_dnm_dn9 = assign42880_e57605_d_n9;
        locals.var_dnm_dn10 = assign42880_e57605_d_n10;
        locals.var_dnm_dn11 = assign42880_e57605_d_n11;
        locals.var_dnm_dn14 = assign42880_e57605_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign42890_e57620: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1061 = assign42890_e57620;
        locals.var_guard1061_rv = 0.0;

        let assign42900_e57623: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1062 = assign42900_e57623;
        locals.var_guard1062_rv = 0.0;

        let (assign42910_e57640,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42910_e57640;
        locals.var_mm_rv = 0.0;

        let assign42920_e57643: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1063 = assign42920_e57643;
        locals.var_guard1063_rv = 0.0;

        let (assign42930_e57663,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1063 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42930_e57663;
        locals.var_mm_rv = 0.0;

        let assign42940_e57666: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1064 = assign42940_e57666;
        locals.var_guard1064_rv = 0.0;

        let (assign42950_e57689,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1063 == 0.0)) && (locals.var_guard1064 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42950_e57689;
        locals.var_mm_rv = 0.0;

        let assign42960_e57692: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1065 = assign42960_e57692;
        locals.var_guard1065_rv = 0.0;

        let (assign42970_e57718,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1063 == 0.0)) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1065 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42970_e57718;
        locals.var_mm_rv = 0.0;

        let (assign42980_e57733,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign42980_e57733;
        locals.var_m0_rv = 0.0;

        let mut assign42990_loop_guard: usize = 0;
        while {
            let assign42990_cond_e57749: f64 = if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign42990_cond_e57749 != 0.0
        } {
            assign42990_loop_guard += 1;
            assert!(assign42990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign42990_body0_e57765, assign42990_body0_e57765_d_n0, assign42990_body0_e57765_d_n2, assign42990_body0_e57765_d_n4, assign42990_body0_e57765_d_n5, assign42990_body0_e57765_d_n6, assign42990_body0_e57765_d_n7, assign42990_body0_e57765_d_n8, assign42990_body0_e57765_d_n9, assign42990_body0_e57765_d_n10, assign42990_body0_e57765_d_n11, assign42990_body0_e57765_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) {
        let assign42990_body0_e57763: f64 = (locals.var_dnm).sqrt();
        (assign42990_body0_e57763, (locals.var_dnm_dn0 / (2.0 * assign42990_body0_e57763)), (locals.var_dnm_dn2 / (2.0 * assign42990_body0_e57763)), (locals.var_dnm_dn4 / (2.0 * assign42990_body0_e57763)), (locals.var_dnm_dn5 / (2.0 * assign42990_body0_e57763)), (locals.var_dnm_dn6 / (2.0 * assign42990_body0_e57763)), (locals.var_dnm_dn7 / (2.0 * assign42990_body0_e57763)), (locals.var_dnm_dn8 / (2.0 * assign42990_body0_e57763)), (locals.var_dnm_dn9 / (2.0 * assign42990_body0_e57763)), (locals.var_dnm_dn10 / (2.0 * assign42990_body0_e57763)), (locals.var_dnm_dn11 / (2.0 * assign42990_body0_e57763)), (locals.var_dnm_dn14 / (2.0 * assign42990_body0_e57763)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign42990_body0_e57765;
            locals.var_dnm_dn0 = assign42990_body0_e57765_d_n0;
            locals.var_dnm_dn2 = assign42990_body0_e57765_d_n2;
            locals.var_dnm_dn4 = assign42990_body0_e57765_d_n4;
            locals.var_dnm_dn5 = assign42990_body0_e57765_d_n5;
            locals.var_dnm_dn6 = assign42990_body0_e57765_d_n6;
            locals.var_dnm_dn7 = assign42990_body0_e57765_d_n7;
            locals.var_dnm_dn8 = assign42990_body0_e57765_d_n8;
            locals.var_dnm_dn9 = assign42990_body0_e57765_d_n9;
            locals.var_dnm_dn10 = assign42990_body0_e57765_d_n10;
            locals.var_dnm_dn11 = assign42990_body0_e57765_d_n11;
            locals.var_dnm_dn14 = assign42990_body0_e57765_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign42990_body1_e57782,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) {
        let assign42990_body1_e57780: f64 = (locals.var_m0 + 1.0);
        (assign42990_body1_e57780,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign42990_body1_e57782;
            locals.var_m0_rv = 0.0;
        }

        let (assign43000_e57809, assign43000_e57809_d_n0, assign43000_e57809_d_n2, assign43000_e57809_d_n4, assign43000_e57809_d_n5, assign43000_e57809_d_n6, assign43000_e57809_d_n7, assign43000_e57809_d_n8, assign43000_e57809_d_n9, assign43000_e57809_d_n10, assign43000_e57809_d_n11, assign43000_e57809_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
        let (assign43000_e57807, assign43000_e57807_d_n0, assign43000_e57807_d_n2, assign43000_e57807_d_n4, assign43000_e57807_d_n5, assign43000_e57807_d_n6, assign43000_e57807_d_n7, assign43000_e57807_d_n8, assign43000_e57807_d_n9, assign43000_e57807_d_n10, assign43000_e57807_d_n11, assign43000_e57807_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43000_e57804: f64 = (2.0 * 2.0);
                let assign43000_e57805: f64 = (1.0 / assign43000_e57804);
                let assign43000_e57806: f64 = (locals.var_dnm).powf(assign43000_e57805);
                (assign43000_e57806, if 0.0 == 0.0 && ((assign43000_e57805) as f64).is_finite() && ((assign43000_e57805) as f64).fract() == 0.0 { if assign43000_e57805 == 0.0 { 0.0 } else { (assign43000_e57805 * ((locals.var_dnm).powf(assign43000_e57805 - 1.0) * locals.var_dnm_dn0)) } } else { (assign43000_e57806 * (assign43000_e57805 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43000_e57805) as f64).is_finite() && ((assign43000_e57805) as f64).fract() == 0.0 { if assign43000_e57805 == 0.0 { 0.0 } else { (assign43000_e57805 * ((locals.var_dnm).powf(assign43000_e57805 - 1.0) * locals.var_dnm_dn2)) } } else { (assign43000_e57806 * (assign43000_e57805 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43000_e57805) as f64).is_finite() && ((assign43000_e57805) as f64).fract() == 0.0 { if assign43000_e57805 == 0.0 { 0.0 } else { (assign43000_e57805 * ((locals.var_dnm).powf(assign43000_e57805 - 1.0) * locals.var_dnm_dn4)) } } else { (assign43000_e57806 * (assign43000_e57805 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43000_e57805) as f64).is_finite() && ((assign43000_e57805) as f64).fract() == 0.0 { if assign43000_e57805 == 0.0 { 0.0 } else { (assign43000_e57805 * ((locals.var_dnm).powf(assign43000_e57805 - 1.0) * locals.var_dnm_dn5)) } } else { (assign43000_e57806 * (assign43000_e57805 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43000_e57805) as f64).is_finite() && ((assign43000_e57805) as f64).fract() == 0.0 { if assign43000_e57805 == 0.0 { 0.0 } else { (assign43000_e57805 * ((locals.var_dnm).powf(assign43000_e57805 - 1.0) * locals.var_dnm_dn6)) } } else { (assign43000_e57806 * (assign43000_e57805 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43000_e57805) as f64).is_finite() && ((assign43000_e57805) as f64).fract() == 0.0 { if assign43000_e57805 == 0.0 { 0.0 } else { (assign43000_e57805 * ((locals.var_dnm).powf(assign43000_e57805 - 1.0) * locals.var_dnm_dn7)) } } else { (assign43000_e57806 * (assign43000_e57805 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43000_e57805) as f64).is_finite() && ((assign43000_e57805) as f64).fract() == 0.0 { if assign43000_e57805 == 0.0 { 0.0 } else { (assign43000_e57805 * ((locals.var_dnm).powf(assign43000_e57805 - 1.0) * locals.var_dnm_dn8)) } } else { (assign43000_e57806 * (assign43000_e57805 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43000_e57805) as f64).is_finite() && ((assign43000_e57805) as f64).fract() == 0.0 { if assign43000_e57805 == 0.0 { 0.0 } else { (assign43000_e57805 * ((locals.var_dnm).powf(assign43000_e57805 - 1.0) * locals.var_dnm_dn9)) } } else { (assign43000_e57806 * (assign43000_e57805 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43000_e57805) as f64).is_finite() && ((assign43000_e57805) as f64).fract() == 0.0 { if assign43000_e57805 == 0.0 { 0.0 } else { (assign43000_e57805 * ((locals.var_dnm).powf(assign43000_e57805 - 1.0) * locals.var_dnm_dn10)) } } else { (assign43000_e57806 * (assign43000_e57805 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43000_e57805) as f64).is_finite() && ((assign43000_e57805) as f64).fract() == 0.0 { if assign43000_e57805 == 0.0 { 0.0 } else { (assign43000_e57805 * ((locals.var_dnm).powf(assign43000_e57805 - 1.0) * locals.var_dnm_dn11)) } } else { (assign43000_e57806 * (assign43000_e57805 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43000_e57805) as f64).is_finite() && ((assign43000_e57805) as f64).fract() == 0.0 { if assign43000_e57805 == 0.0 { 0.0 } else { (assign43000_e57805 * ((locals.var_dnm).powf(assign43000_e57805 - 1.0) * locals.var_dnm_dn14)) } } else { (assign43000_e57806 * (assign43000_e57805 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign43000_e57807, assign43000_e57807_d_n0, assign43000_e57807_d_n2, assign43000_e57807_d_n4, assign43000_e57807_d_n5, assign43000_e57807_d_n6, assign43000_e57807_d_n7, assign43000_e57807_d_n8, assign43000_e57807_d_n9, assign43000_e57807_d_n10, assign43000_e57807_d_n11, assign43000_e57807_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43000_e57809;
        locals.var_dnm_dn0 = assign43000_e57809_d_n0;
        locals.var_dnm_dn2 = assign43000_e57809_d_n2;
        locals.var_dnm_dn4 = assign43000_e57809_d_n4;
        locals.var_dnm_dn5 = assign43000_e57809_d_n5;
        locals.var_dnm_dn6 = assign43000_e57809_d_n6;
        locals.var_dnm_dn7 = assign43000_e57809_d_n7;
        locals.var_dnm_dn8 = assign43000_e57809_d_n8;
        locals.var_dnm_dn9 = assign43000_e57809_d_n9;
        locals.var_dnm_dn10 = assign43000_e57809_d_n10;
        locals.var_dnm_dn11 = assign43000_e57809_d_n11;
        locals.var_dnm_dn14 = assign43000_e57809_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43010_e57824, assign43010_e57824_d_n0, assign43010_e57824_d_n2, assign43010_e57824_d_n4, assign43010_e57824_d_n5, assign43010_e57824_d_n6, assign43010_e57824_d_n7, assign43010_e57824_d_n8, assign43010_e57824_d_n9, assign43010_e57824_d_n10, assign43010_e57824_d_n11, assign43010_e57824_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign43010_e57822: f64 = (1.0 / locals.var_dnm);
        (assign43010_e57822, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43010_e57824;
        locals.var_dnm_dn0 = assign43010_e57824_d_n0;
        locals.var_dnm_dn2 = assign43010_e57824_d_n2;
        locals.var_dnm_dn4 = assign43010_e57824_d_n4;
        locals.var_dnm_dn5 = assign43010_e57824_d_n5;
        locals.var_dnm_dn6 = assign43010_e57824_d_n6;
        locals.var_dnm_dn7 = assign43010_e57824_d_n7;
        locals.var_dnm_dn8 = assign43010_e57824_d_n8;
        locals.var_dnm_dn9 = assign43010_e57824_d_n9;
        locals.var_dnm_dn10 = assign43010_e57824_d_n10;
        locals.var_dnm_dn11 = assign43010_e57824_d_n11;
        locals.var_dnm_dn14 = assign43010_e57824_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43020_e57841, assign43020_e57841_d_n0, assign43020_e57841_d_n2, assign43020_e57841_d_n4, assign43020_e57841_d_n5, assign43020_e57841_d_n6, assign43020_e57841_d_n7, assign43020_e57841_d_n8, assign43020_e57841_d_n9, assign43020_e57841_d_n10, assign43020_e57841_d_n11, assign43020_e57841_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign43020_e57837: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign43020_e57839: f64 = (assign43020_e57837 * locals.var_dnm);
        (assign43020_e57839, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign43020_e57837 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign43020_e57837 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn4)) * locals.var_dnm) + (assign43020_e57837 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn5)) * locals.var_dnm) + (assign43020_e57837 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign43020_e57837 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign43020_e57837 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn8)) * locals.var_dnm) + (assign43020_e57837 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn9)) * locals.var_dnm) + (assign43020_e57837 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign43020_e57837 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign43020_e57837 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn14)) * locals.var_dnm) + (assign43020_e57837 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign43020_e57841;
        locals.var_tmf0_dn0 = assign43020_e57841_d_n0;
        locals.var_tmf0_dn2 = assign43020_e57841_d_n2;
        locals.var_tmf0_dn4 = assign43020_e57841_d_n4;
        locals.var_tmf0_dn5 = assign43020_e57841_d_n5;
        locals.var_tmf0_dn6 = assign43020_e57841_d_n6;
        locals.var_tmf0_dn7 = assign43020_e57841_d_n7;
        locals.var_tmf0_dn8 = assign43020_e57841_d_n8;
        locals.var_tmf0_dn9 = assign43020_e57841_d_n9;
        locals.var_tmf0_dn10 = assign43020_e57841_d_n10;
        locals.var_tmf0_dn11 = assign43020_e57841_d_n11;
        locals.var_tmf0_dn14 = assign43020_e57841_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign43030_e57860, assign43030_e57860_d_n0, assign43030_e57860_d_n2, assign43030_e57860_d_n4, assign43030_e57860_d_n5, assign43030_e57860_d_n6, assign43030_e57860_d_n7, assign43030_e57860_d_n8, assign43030_e57860_d_n9, assign43030_e57860_d_n10, assign43030_e57860_d_n11, assign43030_e57860_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign43030_e57854: f64 = (locals.var_t5 * locals.var_xmp);
        let assign43030_e57856: f64 = (assign43030_e57854 * locals.var_dnm);
        let assign43030_e57858: f64 = (assign43030_e57856 / locals.var_arg);
        (assign43030_e57858, (((((((locals.var_t5_dn0 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign43030_e57854 * locals.var_dnm_dn0)) * locals.var_arg) - (assign43030_e57856 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn2 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign43030_e57854 * locals.var_dnm_dn2)) * locals.var_arg) - (assign43030_e57856 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn4 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign43030_e57854 * locals.var_dnm_dn4)) * locals.var_arg) - (assign43030_e57856 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn5 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign43030_e57854 * locals.var_dnm_dn5)) * locals.var_arg) - (assign43030_e57856 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn6 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign43030_e57854 * locals.var_dnm_dn6)) * locals.var_arg) - (assign43030_e57856 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn7 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign43030_e57854 * locals.var_dnm_dn7)) * locals.var_arg) - (assign43030_e57856 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn8 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign43030_e57854 * locals.var_dnm_dn8)) * locals.var_arg) - (assign43030_e57856 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn9 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign43030_e57854 * locals.var_dnm_dn9)) * locals.var_arg) - (assign43030_e57856 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn10 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign43030_e57854 * locals.var_dnm_dn10)) * locals.var_arg) - (assign43030_e57856 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn11 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign43030_e57854 * locals.var_dnm_dn11)) * locals.var_arg) - (assign43030_e57856 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn14 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign43030_e57854 * locals.var_dnm_dn14)) * locals.var_arg) - (assign43030_e57856 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43030_e57860;
        locals.var_t0_dn0 = assign43030_e57860_d_n0;
        locals.var_t0_dn2 = assign43030_e57860_d_n2;
        locals.var_t0_dn4 = assign43030_e57860_d_n4;
        locals.var_t0_dn5 = assign43030_e57860_d_n5;
        locals.var_t0_dn6 = assign43030_e57860_d_n6;
        locals.var_t0_dn7 = assign43030_e57860_d_n7;
        locals.var_t0_dn8 = assign43030_e57860_d_n8;
        locals.var_t0_dn9 = assign43030_e57860_d_n9;
        locals.var_t0_dn10 = assign43030_e57860_d_n10;
        locals.var_t0_dn11 = assign43030_e57860_d_n11;
        locals.var_t0_dn14 = assign43030_e57860_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_149(
        locals: &mut StampLocals,
    ) {
        let (assign43040_e57877, assign43040_e57877_d_n0, assign43040_e57877_d_n2, assign43040_e57877_d_n4, assign43040_e57877_d_n5, assign43040_e57877_d_n6, assign43040_e57877_d_n7, assign43040_e57877_d_n8, assign43040_e57877_d_n9, assign43040_e57877_d_n10, assign43040_e57877_d_n11, assign43040_e57877_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign43040_e57873: f64 = locals.var_t5;
        let assign43040_e57875: f64 = (assign43040_e57873 - locals.var_tmf0);
        (assign43040_e57875, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn4 - locals.var_tmf0_dn4), (locals.var_t5_dn5 - locals.var_tmf0_dn5), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn8 - locals.var_tmf0_dn8), (locals.var_t5_dn9 - locals.var_tmf0_dn9), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign43040_e57877;
        locals.var_t4_dn0 = assign43040_e57877_d_n0;
        locals.var_t4_dn2 = assign43040_e57877_d_n2;
        locals.var_t4_dn4 = assign43040_e57877_d_n4;
        locals.var_t4_dn5 = assign43040_e57877_d_n5;
        locals.var_t4_dn6 = assign43040_e57877_d_n6;
        locals.var_t4_dn7 = assign43040_e57877_d_n7;
        locals.var_t4_dn8 = assign43040_e57877_d_n8;
        locals.var_t4_dn9 = assign43040_e57877_d_n9;
        locals.var_t4_dn10 = assign43040_e57877_d_n10;
        locals.var_t4_dn11 = assign43040_e57877_d_n11;
        locals.var_t4_dn14 = assign43040_e57877_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign43050_e57890, assign43050_e57890_d_n0, assign43050_e57890_d_n2, assign43050_e57890_d_n4, assign43050_e57890_d_n5, assign43050_e57890_d_n6, assign43050_e57890_d_n7, assign43050_e57890_d_n8, assign43050_e57890_d_n9, assign43050_e57890_d_n10, assign43050_e57890_d_n11, assign43050_e57890_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43050_e57890;
        locals.var_t0_dn0 = assign43050_e57890_d_n0;
        locals.var_t0_dn2 = assign43050_e57890_d_n2;
        locals.var_t0_dn4 = assign43050_e57890_d_n4;
        locals.var_t0_dn5 = assign43050_e57890_d_n5;
        locals.var_t0_dn6 = assign43050_e57890_d_n6;
        locals.var_t0_dn7 = assign43050_e57890_d_n7;
        locals.var_t0_dn8 = assign43050_e57890_d_n8;
        locals.var_t0_dn9 = assign43050_e57890_d_n9;
        locals.var_t0_dn10 = assign43050_e57890_d_n10;
        locals.var_t0_dn11 = assign43050_e57890_d_n11;
        locals.var_t0_dn14 = assign43050_e57890_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43060_e57904, assign43060_e57904_d_n0, assign43060_e57904_d_n2, assign43060_e57904_d_n4, assign43060_e57904_d_n5, assign43060_e57904_d_n6, assign43060_e57904_d_n7, assign43060_e57904_d_n8, assign43060_e57904_d_n9, assign43060_e57904_d_n10, assign43060_e57904_d_n11, assign43060_e57904_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign43060_e57904;
        locals.var_t4_dn0 = assign43060_e57904_d_n0;
        locals.var_t4_dn2 = assign43060_e57904_d_n2;
        locals.var_t4_dn4 = assign43060_e57904_d_n4;
        locals.var_t4_dn5 = assign43060_e57904_d_n5;
        locals.var_t4_dn6 = assign43060_e57904_d_n6;
        locals.var_t4_dn7 = assign43060_e57904_d_n7;
        locals.var_t4_dn8 = assign43060_e57904_d_n8;
        locals.var_t4_dn9 = assign43060_e57904_d_n9;
        locals.var_t4_dn10 = assign43060_e57904_d_n10;
        locals.var_t4_dn11 = assign43060_e57904_d_n11;
        locals.var_t4_dn14 = assign43060_e57904_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign43070_e57918, assign43070_e57918_d_n0, assign43070_e57918_d_n2, assign43070_e57918_d_n4, assign43070_e57918_d_n5, assign43070_e57918_d_n6, assign43070_e57918_d_n7, assign43070_e57918_d_n8, assign43070_e57918_d_n9, assign43070_e57918_d_n10, assign43070_e57918_d_n11, assign43070_e57918_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43070_e57918;
        locals.var_t0_dn0 = assign43070_e57918_d_n0;
        locals.var_t0_dn2 = assign43070_e57918_d_n2;
        locals.var_t0_dn4 = assign43070_e57918_d_n4;
        locals.var_t0_dn5 = assign43070_e57918_d_n5;
        locals.var_t0_dn6 = assign43070_e57918_d_n6;
        locals.var_t0_dn7 = assign43070_e57918_d_n7;
        locals.var_t0_dn8 = assign43070_e57918_d_n8;
        locals.var_t0_dn9 = assign43070_e57918_d_n9;
        locals.var_t0_dn10 = assign43070_e57918_d_n10;
        locals.var_t0_dn11 = assign43070_e57918_d_n11;
        locals.var_t0_dn14 = assign43070_e57918_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43080_e57930, assign43080_e57930_d_n0, assign43080_e57930_d_n2, assign43080_e57930_d_n4, assign43080_e57930_d_n5, assign43080_e57930_d_n6, assign43080_e57930_d_n7, assign43080_e57930_d_n8, assign43080_e57930_d_n9, assign43080_e57930_d_n10, assign43080_e57930_d_n11, assign43080_e57930_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let assign43080_e57928: f64 = (locals.var_t4).sqrt();
        (assign43080_e57928, (locals.var_t4_dn0 / (2.0 * assign43080_e57928)), (locals.var_t4_dn2 / (2.0 * assign43080_e57928)), (locals.var_t4_dn4 / (2.0 * assign43080_e57928)), (locals.var_t4_dn5 / (2.0 * assign43080_e57928)), (locals.var_t4_dn6 / (2.0 * assign43080_e57928)), (locals.var_t4_dn7 / (2.0 * assign43080_e57928)), (locals.var_t4_dn8 / (2.0 * assign43080_e57928)), (locals.var_t4_dn9 / (2.0 * assign43080_e57928)), (locals.var_t4_dn10 / (2.0 * assign43080_e57928)), (locals.var_t4_dn11 / (2.0 * assign43080_e57928)), (locals.var_t4_dn14 / (2.0 * assign43080_e57928)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign43080_e57930;
        locals.var_t3_dn0 = assign43080_e57930_d_n0;
        locals.var_t3_dn2 = assign43080_e57930_d_n2;
        locals.var_t3_dn4 = assign43080_e57930_d_n4;
        locals.var_t3_dn5 = assign43080_e57930_d_n5;
        locals.var_t3_dn6 = assign43080_e57930_d_n6;
        locals.var_t3_dn7 = assign43080_e57930_d_n7;
        locals.var_t3_dn8 = assign43080_e57930_d_n8;
        locals.var_t3_dn9 = assign43080_e57930_d_n9;
        locals.var_t3_dn10 = assign43080_e57930_d_n10;
        locals.var_t3_dn11 = assign43080_e57930_d_n11;
        locals.var_t3_dn14 = assign43080_e57930_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign43090_e57947, assign43090_e57947_d_n0, assign43090_e57947_d_n2, assign43090_e57947_d_n4, assign43090_e57947_d_n5, assign43090_e57947_d_n6, assign43090_e57947_d_n7, assign43090_e57947_d_n8, assign43090_e57947_d_n9, assign43090_e57947_d_n10, assign43090_e57947_d_n11, assign43090_e57947_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let assign43090_e57943: f64 = (1.0 - locals.var_t3);
        let assign43090_e57944: f64 = (locals.var_q_ndepm_esi_cox_inv2 * assign43090_e57943);
        let assign43090_e57945: f64 = (locals.var_vgp + assign43090_e57944);
        (assign43090_e57945, (locals.var_vgp_dn0 + ((locals.var_q_ndepm_esi_cox_inv2_dn0 * assign43090_e57943) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn0)))), (locals.var_vgp_dn2 + ((locals.var_q_ndepm_esi_cox_inv2_dn2 * assign43090_e57943) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn2)))), (locals.var_vgp_dn4 + ((locals.var_q_ndepm_esi_cox_inv2_dn4 * assign43090_e57943) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn4)))), (locals.var_vgp_dn5 + ((locals.var_q_ndepm_esi_cox_inv2_dn5 * assign43090_e57943) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn5)))), (locals.var_vgp_dn6 + ((locals.var_q_ndepm_esi_cox_inv2_dn6 * assign43090_e57943) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn6)))), (locals.var_vgp_dn7 + ((locals.var_q_ndepm_esi_cox_inv2_dn7 * assign43090_e57943) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn7)))), (locals.var_vgp_dn8 + ((locals.var_q_ndepm_esi_cox_inv2_dn8 * assign43090_e57943) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn8)))), (locals.var_vgp_dn9 + ((locals.var_q_ndepm_esi_cox_inv2_dn9 * assign43090_e57943) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn9)))), (locals.var_vgp_dn10 + ((locals.var_q_ndepm_esi_cox_inv2_dn10 * assign43090_e57943) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn10)))), (locals.var_vgp_dn11 + ((locals.var_q_ndepm_esi_cox_inv2_dn11 * assign43090_e57943) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn11)))), (locals.var_vgp_dn14 + ((locals.var_q_ndepm_esi_cox_inv2_dn14 * assign43090_e57943) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn14)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign43090_e57947;
        locals.var_t10_dn0 = assign43090_e57947_d_n0;
        locals.var_t10_dn2 = assign43090_e57947_d_n2;
        locals.var_t10_dn4 = assign43090_e57947_d_n4;
        locals.var_t10_dn5 = assign43090_e57947_d_n5;
        locals.var_t10_dn6 = assign43090_e57947_d_n6;
        locals.var_t10_dn7 = assign43090_e57947_d_n7;
        locals.var_t10_dn8 = assign43090_e57947_d_n8;
        locals.var_t10_dn9 = assign43090_e57947_d_n9;
        locals.var_t10_dn10 = assign43090_e57947_d_n10;
        locals.var_t10_dn11 = assign43090_e57947_d_n11;
        locals.var_t10_dn14 = assign43090_e57947_d_n14;
        locals.var_t10_rv = 0.0;

        let assign43100_e57951: f64 = (locals.var_uc_depleak + locals.var_depqfn_dlt);
        let assign43100_e57956: f64 = if ((locals.var_t10 < assign43100_e57951) && (locals.var_depqfn_dlt >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1066 = assign43100_e57956;
        locals.var_guard1066_rv = 0.0;

        let (assign43110_e57973, assign43110_e57973_d_n0, assign43110_e57973_d_n2, assign43110_e57973_d_n4, assign43110_e57973_d_n5, assign43110_e57973_d_n6, assign43110_e57973_d_n7, assign43110_e57973_d_n8, assign43110_e57973_d_n9, assign43110_e57973_d_n10, assign43110_e57973_d_n11, assign43110_e57973_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43110_e57969: f64 = (locals.var_uc_depleak + locals.var_depqfn_dlt);
        let assign43110_e57971: f64 = (assign43110_e57969 - locals.var_t10);
        (assign43110_e57971, (locals.var_uc_depleak_dn0 - locals.var_t10_dn0), (locals.var_uc_depleak_dn2 - locals.var_t10_dn2), (locals.var_uc_depleak_dn4 - locals.var_t10_dn4), (locals.var_uc_depleak_dn5 - locals.var_t10_dn5), (locals.var_uc_depleak_dn6 - locals.var_t10_dn6), (locals.var_uc_depleak_dn7 - locals.var_t10_dn7), (locals.var_uc_depleak_dn8 - locals.var_t10_dn8), (locals.var_uc_depleak_dn9 - locals.var_t10_dn9), (locals.var_uc_depleak_dn10 - locals.var_t10_dn10), (locals.var_uc_depleak_dn11 - locals.var_t10_dn11), (locals.var_uc_depleak_dn14 - locals.var_t10_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign43110_e57973;
        locals.var_tmf1_dn0 = assign43110_e57973_d_n0;
        locals.var_tmf1_dn2 = assign43110_e57973_d_n2;
        locals.var_tmf1_dn4 = assign43110_e57973_d_n4;
        locals.var_tmf1_dn5 = assign43110_e57973_d_n5;
        locals.var_tmf1_dn6 = assign43110_e57973_d_n6;
        locals.var_tmf1_dn7 = assign43110_e57973_d_n7;
        locals.var_tmf1_dn8 = assign43110_e57973_d_n8;
        locals.var_tmf1_dn9 = assign43110_e57973_d_n9;
        locals.var_tmf1_dn10 = assign43110_e57973_d_n10;
        locals.var_tmf1_dn11 = assign43110_e57973_d_n11;
        locals.var_tmf1_dn14 = assign43110_e57973_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign43120_e57988, assign43120_e57988_d_n0, assign43120_e57988_d_n2, assign43120_e57988_d_n4, assign43120_e57988_d_n5, assign43120_e57988_d_n6, assign43120_e57988_d_n7, assign43120_e57988_d_n8, assign43120_e57988_d_n9, assign43120_e57988_d_n10, assign43120_e57988_d_n11, assign43120_e57988_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43120_e57986: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign43120_e57986, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign43120_e57988;
        locals.var_x2_dn0 = assign43120_e57988_d_n0;
        locals.var_x2_dn2 = assign43120_e57988_d_n2;
        locals.var_x2_dn4 = assign43120_e57988_d_n4;
        locals.var_x2_dn5 = assign43120_e57988_d_n5;
        locals.var_x2_dn6 = assign43120_e57988_d_n6;
        locals.var_x2_dn7 = assign43120_e57988_d_n7;
        locals.var_x2_dn8 = assign43120_e57988_d_n8;
        locals.var_x2_dn9 = assign43120_e57988_d_n9;
        locals.var_x2_dn10 = assign43120_e57988_d_n10;
        locals.var_x2_dn11 = assign43120_e57988_d_n11;
        locals.var_x2_dn14 = assign43120_e57988_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign43130_e58003, assign43130_e58003_d_n0, assign43130_e58003_d_n2, assign43130_e58003_d_n4, assign43130_e58003_d_n5, assign43130_e58003_d_n6, assign43130_e58003_d_n7, assign43130_e58003_d_n8, assign43130_e58003_d_n9, assign43130_e58003_d_n10, assign43130_e58003_d_n11, assign43130_e58003_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43130_e58001: f64 = (locals.var_depqfn_dlt * locals.var_depqfn_dlt);
        (assign43130_e58001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign43130_e58003;
        locals.var_xmax2_dn0 = assign43130_e58003_d_n0;
        locals.var_xmax2_dn2 = assign43130_e58003_d_n2;
        locals.var_xmax2_dn4 = assign43130_e58003_d_n4;
        locals.var_xmax2_dn5 = assign43130_e58003_d_n5;
        locals.var_xmax2_dn6 = assign43130_e58003_d_n6;
        locals.var_xmax2_dn7 = assign43130_e58003_d_n7;
        locals.var_xmax2_dn8 = assign43130_e58003_d_n8;
        locals.var_xmax2_dn9 = assign43130_e58003_d_n9;
        locals.var_xmax2_dn10 = assign43130_e58003_d_n10;
        locals.var_xmax2_dn11 = assign43130_e58003_d_n11;
        locals.var_xmax2_dn14 = assign43130_e58003_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign43140_e58016, assign43140_e58016_d_n0, assign43140_e58016_d_n2, assign43140_e58016_d_n4, assign43140_e58016_d_n5, assign43140_e58016_d_n6, assign43140_e58016_d_n7, assign43140_e58016_d_n8, assign43140_e58016_d_n9, assign43140_e58016_d_n10, assign43140_e58016_d_n11, assign43140_e58016_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43140_e58016;
        locals.var_xp_dn0 = assign43140_e58016_d_n0;
        locals.var_xp_dn2 = assign43140_e58016_d_n2;
        locals.var_xp_dn4 = assign43140_e58016_d_n4;
        locals.var_xp_dn5 = assign43140_e58016_d_n5;
        locals.var_xp_dn6 = assign43140_e58016_d_n6;
        locals.var_xp_dn7 = assign43140_e58016_d_n7;
        locals.var_xp_dn8 = assign43140_e58016_d_n8;
        locals.var_xp_dn9 = assign43140_e58016_d_n9;
        locals.var_xp_dn10 = assign43140_e58016_d_n10;
        locals.var_xp_dn11 = assign43140_e58016_d_n11;
        locals.var_xp_dn14 = assign43140_e58016_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43150_e58029, assign43150_e58029_d_n0, assign43150_e58029_d_n2, assign43150_e58029_d_n4, assign43150_e58029_d_n5, assign43150_e58029_d_n6, assign43150_e58029_d_n7, assign43150_e58029_d_n8, assign43150_e58029_d_n9, assign43150_e58029_d_n10, assign43150_e58029_d_n11, assign43150_e58029_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43150_e58029;
        locals.var_xmp_dn0 = assign43150_e58029_d_n0;
        locals.var_xmp_dn2 = assign43150_e58029_d_n2;
        locals.var_xmp_dn4 = assign43150_e58029_d_n4;
        locals.var_xmp_dn5 = assign43150_e58029_d_n5;
        locals.var_xmp_dn6 = assign43150_e58029_d_n6;
        locals.var_xmp_dn7 = assign43150_e58029_d_n7;
        locals.var_xmp_dn8 = assign43150_e58029_d_n8;
        locals.var_xmp_dn9 = assign43150_e58029_d_n9;
        locals.var_xmp_dn10 = assign43150_e58029_d_n10;
        locals.var_xmp_dn11 = assign43150_e58029_d_n11;
        locals.var_xmp_dn14 = assign43150_e58029_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign43160_e58042,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43160_e58042;
        locals.var_m0_rv = 0.0;

        let (assign43170_e58055,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43170_e58055;
        locals.var_mm_rv = 0.0;

        let (assign43180_e58068, assign43180_e58068_d_n0, assign43180_e58068_d_n2, assign43180_e58068_d_n4, assign43180_e58068_d_n5, assign43180_e58068_d_n6, assign43180_e58068_d_n7, assign43180_e58068_d_n8, assign43180_e58068_d_n9, assign43180_e58068_d_n10, assign43180_e58068_d_n11, assign43180_e58068_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign43180_e58068;
        locals.var_arg_dn0 = assign43180_e58068_d_n0;
        locals.var_arg_dn2 = assign43180_e58068_d_n2;
        locals.var_arg_dn4 = assign43180_e58068_d_n4;
        locals.var_arg_dn5 = assign43180_e58068_d_n5;
        locals.var_arg_dn6 = assign43180_e58068_d_n6;
        locals.var_arg_dn7 = assign43180_e58068_d_n7;
        locals.var_arg_dn8 = assign43180_e58068_d_n8;
        locals.var_arg_dn9 = assign43180_e58068_d_n9;
        locals.var_arg_dn10 = assign43180_e58068_d_n10;
        locals.var_arg_dn11 = assign43180_e58068_d_n11;
        locals.var_arg_dn14 = assign43180_e58068_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign43190_e58081, assign43190_e58081_d_n0, assign43190_e58081_d_n2, assign43190_e58081_d_n4, assign43190_e58081_d_n5, assign43190_e58081_d_n6, assign43190_e58081_d_n7, assign43190_e58081_d_n8, assign43190_e58081_d_n9, assign43190_e58081_d_n10, assign43190_e58081_d_n11, assign43190_e58081_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43190_e58081;
        locals.var_dnm_dn0 = assign43190_e58081_d_n0;
        locals.var_dnm_dn2 = assign43190_e58081_d_n2;
        locals.var_dnm_dn4 = assign43190_e58081_d_n4;
        locals.var_dnm_dn5 = assign43190_e58081_d_n5;
        locals.var_dnm_dn6 = assign43190_e58081_d_n6;
        locals.var_dnm_dn7 = assign43190_e58081_d_n7;
        locals.var_dnm_dn8 = assign43190_e58081_d_n8;
        locals.var_dnm_dn9 = assign43190_e58081_d_n9;
        locals.var_dnm_dn10 = assign43190_e58081_d_n10;
        locals.var_dnm_dn11 = assign43190_e58081_d_n11;
        locals.var_dnm_dn14 = assign43190_e58081_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43200_e58096, assign43200_e58096_d_n0, assign43200_e58096_d_n2, assign43200_e58096_d_n4, assign43200_e58096_d_n5, assign43200_e58096_d_n6, assign43200_e58096_d_n7, assign43200_e58096_d_n8, assign43200_e58096_d_n9, assign43200_e58096_d_n10, assign43200_e58096_d_n11, assign43200_e58096_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43200_e58094: f64 = (locals.var_xp * locals.var_x2);
        (assign43200_e58094, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43200_e58096;
        locals.var_xp_dn0 = assign43200_e58096_d_n0;
        locals.var_xp_dn2 = assign43200_e58096_d_n2;
        locals.var_xp_dn4 = assign43200_e58096_d_n4;
        locals.var_xp_dn5 = assign43200_e58096_d_n5;
        locals.var_xp_dn6 = assign43200_e58096_d_n6;
        locals.var_xp_dn7 = assign43200_e58096_d_n7;
        locals.var_xp_dn8 = assign43200_e58096_d_n8;
        locals.var_xp_dn9 = assign43200_e58096_d_n9;
        locals.var_xp_dn10 = assign43200_e58096_d_n10;
        locals.var_xp_dn11 = assign43200_e58096_d_n11;
        locals.var_xp_dn14 = assign43200_e58096_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43210_e58111, assign43210_e58111_d_n0, assign43210_e58111_d_n2, assign43210_e58111_d_n4, assign43210_e58111_d_n5, assign43210_e58111_d_n6, assign43210_e58111_d_n7, assign43210_e58111_d_n8, assign43210_e58111_d_n9, assign43210_e58111_d_n10, assign43210_e58111_d_n11, assign43210_e58111_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43210_e58109: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43210_e58109, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43210_e58111;
        locals.var_xmp_dn0 = assign43210_e58111_d_n0;
        locals.var_xmp_dn2 = assign43210_e58111_d_n2;
        locals.var_xmp_dn4 = assign43210_e58111_d_n4;
        locals.var_xmp_dn5 = assign43210_e58111_d_n5;
        locals.var_xmp_dn6 = assign43210_e58111_d_n6;
        locals.var_xmp_dn7 = assign43210_e58111_d_n7;
        locals.var_xmp_dn8 = assign43210_e58111_d_n8;
        locals.var_xmp_dn9 = assign43210_e58111_d_n9;
        locals.var_xmp_dn10 = assign43210_e58111_d_n10;
        locals.var_xmp_dn11 = assign43210_e58111_d_n11;
        locals.var_xmp_dn14 = assign43210_e58111_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign43220_e58126, assign43220_e58126_d_n0, assign43220_e58126_d_n2, assign43220_e58126_d_n4, assign43220_e58126_d_n5, assign43220_e58126_d_n6, assign43220_e58126_d_n7, assign43220_e58126_d_n8, assign43220_e58126_d_n9, assign43220_e58126_d_n10, assign43220_e58126_d_n11, assign43220_e58126_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43220_e58124: f64 = (locals.var_xp * locals.var_x2);
        (assign43220_e58124, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43220_e58126;
        locals.var_xp_dn0 = assign43220_e58126_d_n0;
        locals.var_xp_dn2 = assign43220_e58126_d_n2;
        locals.var_xp_dn4 = assign43220_e58126_d_n4;
        locals.var_xp_dn5 = assign43220_e58126_d_n5;
        locals.var_xp_dn6 = assign43220_e58126_d_n6;
        locals.var_xp_dn7 = assign43220_e58126_d_n7;
        locals.var_xp_dn8 = assign43220_e58126_d_n8;
        locals.var_xp_dn9 = assign43220_e58126_d_n9;
        locals.var_xp_dn10 = assign43220_e58126_d_n10;
        locals.var_xp_dn11 = assign43220_e58126_d_n11;
        locals.var_xp_dn14 = assign43220_e58126_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43230_e58141, assign43230_e58141_d_n0, assign43230_e58141_d_n2, assign43230_e58141_d_n4, assign43230_e58141_d_n5, assign43230_e58141_d_n6, assign43230_e58141_d_n7, assign43230_e58141_d_n8, assign43230_e58141_d_n9, assign43230_e58141_d_n10, assign43230_e58141_d_n11, assign43230_e58141_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43230_e58139: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43230_e58139, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43230_e58141;
        locals.var_xmp_dn0 = assign43230_e58141_d_n0;
        locals.var_xmp_dn2 = assign43230_e58141_d_n2;
        locals.var_xmp_dn4 = assign43230_e58141_d_n4;
        locals.var_xmp_dn5 = assign43230_e58141_d_n5;
        locals.var_xmp_dn6 = assign43230_e58141_d_n6;
        locals.var_xmp_dn7 = assign43230_e58141_d_n7;
        locals.var_xmp_dn8 = assign43230_e58141_d_n8;
        locals.var_xmp_dn9 = assign43230_e58141_d_n9;
        locals.var_xmp_dn10 = assign43230_e58141_d_n10;
        locals.var_xmp_dn11 = assign43230_e58141_d_n11;
        locals.var_xmp_dn14 = assign43230_e58141_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign43240_e58156, assign43240_e58156_d_n0, assign43240_e58156_d_n2, assign43240_e58156_d_n4, assign43240_e58156_d_n5, assign43240_e58156_d_n6, assign43240_e58156_d_n7, assign43240_e58156_d_n8, assign43240_e58156_d_n9, assign43240_e58156_d_n10, assign43240_e58156_d_n11, assign43240_e58156_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43240_e58154: f64 = (locals.var_xp + locals.var_xmp);
        (assign43240_e58154, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign43240_e58156;
        locals.var_arg_dn0 = assign43240_e58156_d_n0;
        locals.var_arg_dn2 = assign43240_e58156_d_n2;
        locals.var_arg_dn4 = assign43240_e58156_d_n4;
        locals.var_arg_dn5 = assign43240_e58156_d_n5;
        locals.var_arg_dn6 = assign43240_e58156_d_n6;
        locals.var_arg_dn7 = assign43240_e58156_d_n7;
        locals.var_arg_dn8 = assign43240_e58156_d_n8;
        locals.var_arg_dn9 = assign43240_e58156_d_n9;
        locals.var_arg_dn10 = assign43240_e58156_d_n10;
        locals.var_arg_dn11 = assign43240_e58156_d_n11;
        locals.var_arg_dn14 = assign43240_e58156_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign43250_e58169, assign43250_e58169_d_n0, assign43250_e58169_d_n2, assign43250_e58169_d_n4, assign43250_e58169_d_n5, assign43250_e58169_d_n6, assign43250_e58169_d_n7, assign43250_e58169_d_n8, assign43250_e58169_d_n9, assign43250_e58169_d_n10, assign43250_e58169_d_n11, assign43250_e58169_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43250_e58169;
        locals.var_dnm_dn0 = assign43250_e58169_d_n0;
        locals.var_dnm_dn2 = assign43250_e58169_d_n2;
        locals.var_dnm_dn4 = assign43250_e58169_d_n4;
        locals.var_dnm_dn5 = assign43250_e58169_d_n5;
        locals.var_dnm_dn6 = assign43250_e58169_d_n6;
        locals.var_dnm_dn7 = assign43250_e58169_d_n7;
        locals.var_dnm_dn8 = assign43250_e58169_d_n8;
        locals.var_dnm_dn9 = assign43250_e58169_d_n9;
        locals.var_dnm_dn10 = assign43250_e58169_d_n10;
        locals.var_dnm_dn11 = assign43250_e58169_d_n11;
        locals.var_dnm_dn14 = assign43250_e58169_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign43260_e58184: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1067 = assign43260_e58184;
        locals.var_guard1067_rv = 0.0;

        let assign43270_e58187: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1068 = assign43270_e58187;
        locals.var_guard1068_rv = 0.0;

        let (assign43280_e58204,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43280_e58204;
        locals.var_mm_rv = 0.0;

        let assign43290_e58207: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1069 = assign43290_e58207;
        locals.var_guard1069_rv = 0.0;

        let (assign43300_e58227,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) && (locals.var_guard1068 == 0.0)) && (locals.var_guard1069 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43300_e58227;
        locals.var_mm_rv = 0.0;

        let assign43310_e58230: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1070 = assign43310_e58230;
        locals.var_guard1070_rv = 0.0;

        let (assign43320_e58253,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) && (locals.var_guard1068 == 0.0)) && (locals.var_guard1069 == 0.0)) && (locals.var_guard1070 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43320_e58253;
        locals.var_mm_rv = 0.0;

        let assign43330_e58256: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1071 = assign43330_e58256;
        locals.var_guard1071_rv = 0.0;

        let (assign43340_e58282,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) && (locals.var_guard1068 == 0.0)) && (locals.var_guard1069 == 0.0)) && (locals.var_guard1070 == 0.0)) && (locals.var_guard1071 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43340_e58282;
        locals.var_mm_rv = 0.0;

        let (assign43350_e58297,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43350_e58297;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_150(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign43360_loop_guard: usize = 0;
        while {
            let assign43360_cond_e58313: f64 = if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign43360_cond_e58313 != 0.0
        } {
            assign43360_loop_guard += 1;
            assert!(assign43360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign43360_body0_e58329, assign43360_body0_e58329_d_n0, assign43360_body0_e58329_d_n2, assign43360_body0_e58329_d_n4, assign43360_body0_e58329_d_n5, assign43360_body0_e58329_d_n6, assign43360_body0_e58329_d_n7, assign43360_body0_e58329_d_n8, assign43360_body0_e58329_d_n9, assign43360_body0_e58329_d_n10, assign43360_body0_e58329_d_n11, assign43360_body0_e58329_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) {
        let assign43360_body0_e58327: f64 = (locals.var_dnm).sqrt();
        (assign43360_body0_e58327, (locals.var_dnm_dn0 / (2.0 * assign43360_body0_e58327)), (locals.var_dnm_dn2 / (2.0 * assign43360_body0_e58327)), (locals.var_dnm_dn4 / (2.0 * assign43360_body0_e58327)), (locals.var_dnm_dn5 / (2.0 * assign43360_body0_e58327)), (locals.var_dnm_dn6 / (2.0 * assign43360_body0_e58327)), (locals.var_dnm_dn7 / (2.0 * assign43360_body0_e58327)), (locals.var_dnm_dn8 / (2.0 * assign43360_body0_e58327)), (locals.var_dnm_dn9 / (2.0 * assign43360_body0_e58327)), (locals.var_dnm_dn10 / (2.0 * assign43360_body0_e58327)), (locals.var_dnm_dn11 / (2.0 * assign43360_body0_e58327)), (locals.var_dnm_dn14 / (2.0 * assign43360_body0_e58327)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign43360_body0_e58329;
            locals.var_dnm_dn0 = assign43360_body0_e58329_d_n0;
            locals.var_dnm_dn2 = assign43360_body0_e58329_d_n2;
            locals.var_dnm_dn4 = assign43360_body0_e58329_d_n4;
            locals.var_dnm_dn5 = assign43360_body0_e58329_d_n5;
            locals.var_dnm_dn6 = assign43360_body0_e58329_d_n6;
            locals.var_dnm_dn7 = assign43360_body0_e58329_d_n7;
            locals.var_dnm_dn8 = assign43360_body0_e58329_d_n8;
            locals.var_dnm_dn9 = assign43360_body0_e58329_d_n9;
            locals.var_dnm_dn10 = assign43360_body0_e58329_d_n10;
            locals.var_dnm_dn11 = assign43360_body0_e58329_d_n11;
            locals.var_dnm_dn14 = assign43360_body0_e58329_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign43360_body1_e58346,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) {
        let assign43360_body1_e58344: f64 = (locals.var_m0 + 1.0);
        (assign43360_body1_e58344,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign43360_body1_e58346;
            locals.var_m0_rv = 0.0;
        }

        let (assign43370_e58373, assign43370_e58373_d_n0, assign43370_e58373_d_n2, assign43370_e58373_d_n4, assign43370_e58373_d_n5, assign43370_e58373_d_n6, assign43370_e58373_d_n7, assign43370_e58373_d_n8, assign43370_e58373_d_n9, assign43370_e58373_d_n10, assign43370_e58373_d_n11, assign43370_e58373_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 == 0.0)) {
        let (assign43370_e58371, assign43370_e58371_d_n0, assign43370_e58371_d_n2, assign43370_e58371_d_n4, assign43370_e58371_d_n5, assign43370_e58371_d_n6, assign43370_e58371_d_n7, assign43370_e58371_d_n8, assign43370_e58371_d_n9, assign43370_e58371_d_n10, assign43370_e58371_d_n11, assign43370_e58371_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43370_e58368: f64 = (2.0 * 2.0);
                let assign43370_e58369: f64 = (1.0 / assign43370_e58368);
                let assign43370_e58370: f64 = (locals.var_dnm).powf(assign43370_e58369);
                (assign43370_e58370, if 0.0 == 0.0 && ((assign43370_e58369) as f64).is_finite() && ((assign43370_e58369) as f64).fract() == 0.0 { if assign43370_e58369 == 0.0 { 0.0 } else { (assign43370_e58369 * ((locals.var_dnm).powf(assign43370_e58369 - 1.0) * locals.var_dnm_dn0)) } } else { (assign43370_e58370 * (assign43370_e58369 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43370_e58369) as f64).is_finite() && ((assign43370_e58369) as f64).fract() == 0.0 { if assign43370_e58369 == 0.0 { 0.0 } else { (assign43370_e58369 * ((locals.var_dnm).powf(assign43370_e58369 - 1.0) * locals.var_dnm_dn2)) } } else { (assign43370_e58370 * (assign43370_e58369 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43370_e58369) as f64).is_finite() && ((assign43370_e58369) as f64).fract() == 0.0 { if assign43370_e58369 == 0.0 { 0.0 } else { (assign43370_e58369 * ((locals.var_dnm).powf(assign43370_e58369 - 1.0) * locals.var_dnm_dn4)) } } else { (assign43370_e58370 * (assign43370_e58369 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43370_e58369) as f64).is_finite() && ((assign43370_e58369) as f64).fract() == 0.0 { if assign43370_e58369 == 0.0 { 0.0 } else { (assign43370_e58369 * ((locals.var_dnm).powf(assign43370_e58369 - 1.0) * locals.var_dnm_dn5)) } } else { (assign43370_e58370 * (assign43370_e58369 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43370_e58369) as f64).is_finite() && ((assign43370_e58369) as f64).fract() == 0.0 { if assign43370_e58369 == 0.0 { 0.0 } else { (assign43370_e58369 * ((locals.var_dnm).powf(assign43370_e58369 - 1.0) * locals.var_dnm_dn6)) } } else { (assign43370_e58370 * (assign43370_e58369 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43370_e58369) as f64).is_finite() && ((assign43370_e58369) as f64).fract() == 0.0 { if assign43370_e58369 == 0.0 { 0.0 } else { (assign43370_e58369 * ((locals.var_dnm).powf(assign43370_e58369 - 1.0) * locals.var_dnm_dn7)) } } else { (assign43370_e58370 * (assign43370_e58369 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43370_e58369) as f64).is_finite() && ((assign43370_e58369) as f64).fract() == 0.0 { if assign43370_e58369 == 0.0 { 0.0 } else { (assign43370_e58369 * ((locals.var_dnm).powf(assign43370_e58369 - 1.0) * locals.var_dnm_dn8)) } } else { (assign43370_e58370 * (assign43370_e58369 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43370_e58369) as f64).is_finite() && ((assign43370_e58369) as f64).fract() == 0.0 { if assign43370_e58369 == 0.0 { 0.0 } else { (assign43370_e58369 * ((locals.var_dnm).powf(assign43370_e58369 - 1.0) * locals.var_dnm_dn9)) } } else { (assign43370_e58370 * (assign43370_e58369 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43370_e58369) as f64).is_finite() && ((assign43370_e58369) as f64).fract() == 0.0 { if assign43370_e58369 == 0.0 { 0.0 } else { (assign43370_e58369 * ((locals.var_dnm).powf(assign43370_e58369 - 1.0) * locals.var_dnm_dn10)) } } else { (assign43370_e58370 * (assign43370_e58369 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43370_e58369) as f64).is_finite() && ((assign43370_e58369) as f64).fract() == 0.0 { if assign43370_e58369 == 0.0 { 0.0 } else { (assign43370_e58369 * ((locals.var_dnm).powf(assign43370_e58369 - 1.0) * locals.var_dnm_dn11)) } } else { (assign43370_e58370 * (assign43370_e58369 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43370_e58369) as f64).is_finite() && ((assign43370_e58369) as f64).fract() == 0.0 { if assign43370_e58369 == 0.0 { 0.0 } else { (assign43370_e58369 * ((locals.var_dnm).powf(assign43370_e58369 - 1.0) * locals.var_dnm_dn14)) } } else { (assign43370_e58370 * (assign43370_e58369 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign43370_e58371, assign43370_e58371_d_n0, assign43370_e58371_d_n2, assign43370_e58371_d_n4, assign43370_e58371_d_n5, assign43370_e58371_d_n6, assign43370_e58371_d_n7, assign43370_e58371_d_n8, assign43370_e58371_d_n9, assign43370_e58371_d_n10, assign43370_e58371_d_n11, assign43370_e58371_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43370_e58373;
        locals.var_dnm_dn0 = assign43370_e58373_d_n0;
        locals.var_dnm_dn2 = assign43370_e58373_d_n2;
        locals.var_dnm_dn4 = assign43370_e58373_d_n4;
        locals.var_dnm_dn5 = assign43370_e58373_d_n5;
        locals.var_dnm_dn6 = assign43370_e58373_d_n6;
        locals.var_dnm_dn7 = assign43370_e58373_d_n7;
        locals.var_dnm_dn8 = assign43370_e58373_d_n8;
        locals.var_dnm_dn9 = assign43370_e58373_d_n9;
        locals.var_dnm_dn10 = assign43370_e58373_d_n10;
        locals.var_dnm_dn11 = assign43370_e58373_d_n11;
        locals.var_dnm_dn14 = assign43370_e58373_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43380_e58388, assign43380_e58388_d_n0, assign43380_e58388_d_n2, assign43380_e58388_d_n4, assign43380_e58388_d_n5, assign43380_e58388_d_n6, assign43380_e58388_d_n7, assign43380_e58388_d_n8, assign43380_e58388_d_n9, assign43380_e58388_d_n10, assign43380_e58388_d_n11, assign43380_e58388_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43380_e58386: f64 = (1.0 / locals.var_dnm);
        (assign43380_e58386, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43380_e58388;
        locals.var_dnm_dn0 = assign43380_e58388_d_n0;
        locals.var_dnm_dn2 = assign43380_e58388_d_n2;
        locals.var_dnm_dn4 = assign43380_e58388_d_n4;
        locals.var_dnm_dn5 = assign43380_e58388_d_n5;
        locals.var_dnm_dn6 = assign43380_e58388_d_n6;
        locals.var_dnm_dn7 = assign43380_e58388_d_n7;
        locals.var_dnm_dn8 = assign43380_e58388_d_n8;
        locals.var_dnm_dn9 = assign43380_e58388_d_n9;
        locals.var_dnm_dn10 = assign43380_e58388_d_n10;
        locals.var_dnm_dn11 = assign43380_e58388_d_n11;
        locals.var_dnm_dn14 = assign43380_e58388_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43390_e58405, assign43390_e58405_d_n0, assign43390_e58405_d_n2, assign43390_e58405_d_n4, assign43390_e58405_d_n5, assign43390_e58405_d_n6, assign43390_e58405_d_n7, assign43390_e58405_d_n8, assign43390_e58405_d_n9, assign43390_e58405_d_n10, assign43390_e58405_d_n11, assign43390_e58405_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43390_e58401: f64 = (locals.var_tmf1 * locals.var_depqfn_dlt);
        let assign43390_e58403: f64 = (assign43390_e58401 * locals.var_dnm);
        (assign43390_e58403, (((locals.var_tmf1_dn0 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43390_e58401 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43390_e58401 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43390_e58401 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43390_e58401 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43390_e58401 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43390_e58401 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43390_e58401 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43390_e58401 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43390_e58401 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43390_e58401 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43390_e58401 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign43390_e58405;
        locals.var_tmf0_dn0 = assign43390_e58405_d_n0;
        locals.var_tmf0_dn2 = assign43390_e58405_d_n2;
        locals.var_tmf0_dn4 = assign43390_e58405_d_n4;
        locals.var_tmf0_dn5 = assign43390_e58405_d_n5;
        locals.var_tmf0_dn6 = assign43390_e58405_d_n6;
        locals.var_tmf0_dn7 = assign43390_e58405_d_n7;
        locals.var_tmf0_dn8 = assign43390_e58405_d_n8;
        locals.var_tmf0_dn9 = assign43390_e58405_d_n9;
        locals.var_tmf0_dn10 = assign43390_e58405_d_n10;
        locals.var_tmf0_dn11 = assign43390_e58405_d_n11;
        locals.var_tmf0_dn14 = assign43390_e58405_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign43400_e58424, assign43400_e58424_d_n0, assign43400_e58424_d_n2, assign43400_e58424_d_n4, assign43400_e58424_d_n5, assign43400_e58424_d_n6, assign43400_e58424_d_n7, assign43400_e58424_d_n8, assign43400_e58424_d_n9, assign43400_e58424_d_n10, assign43400_e58424_d_n11, assign43400_e58424_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43400_e58418: f64 = (locals.var_depqfn_dlt * locals.var_xmp);
        let assign43400_e58420: f64 = (assign43400_e58418 * locals.var_dnm);
        let assign43400_e58422: f64 = (assign43400_e58420 / locals.var_arg);
        (assign43400_e58422, ((((((locals.var_depqfn_dlt * locals.var_xmp_dn0) * locals.var_dnm) + (assign43400_e58418 * locals.var_dnm_dn0)) * locals.var_arg) - (assign43400_e58420 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn2) * locals.var_dnm) + (assign43400_e58418 * locals.var_dnm_dn2)) * locals.var_arg) - (assign43400_e58420 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn4) * locals.var_dnm) + (assign43400_e58418 * locals.var_dnm_dn4)) * locals.var_arg) - (assign43400_e58420 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn5) * locals.var_dnm) + (assign43400_e58418 * locals.var_dnm_dn5)) * locals.var_arg) - (assign43400_e58420 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn6) * locals.var_dnm) + (assign43400_e58418 * locals.var_dnm_dn6)) * locals.var_arg) - (assign43400_e58420 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn7) * locals.var_dnm) + (assign43400_e58418 * locals.var_dnm_dn7)) * locals.var_arg) - (assign43400_e58420 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn8) * locals.var_dnm) + (assign43400_e58418 * locals.var_dnm_dn8)) * locals.var_arg) - (assign43400_e58420 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn9) * locals.var_dnm) + (assign43400_e58418 * locals.var_dnm_dn9)) * locals.var_arg) - (assign43400_e58420 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn10) * locals.var_dnm) + (assign43400_e58418 * locals.var_dnm_dn10)) * locals.var_arg) - (assign43400_e58420 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn11) * locals.var_dnm) + (assign43400_e58418 * locals.var_dnm_dn11)) * locals.var_arg) - (assign43400_e58420 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn14) * locals.var_dnm) + (assign43400_e58418 * locals.var_dnm_dn14)) * locals.var_arg) - (assign43400_e58420 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43400_e58424;
        locals.var_t0_dn0 = assign43400_e58424_d_n0;
        locals.var_t0_dn2 = assign43400_e58424_d_n2;
        locals.var_t0_dn4 = assign43400_e58424_d_n4;
        locals.var_t0_dn5 = assign43400_e58424_d_n5;
        locals.var_t0_dn6 = assign43400_e58424_d_n6;
        locals.var_t0_dn7 = assign43400_e58424_d_n7;
        locals.var_t0_dn8 = assign43400_e58424_d_n8;
        locals.var_t0_dn9 = assign43400_e58424_d_n9;
        locals.var_t0_dn10 = assign43400_e58424_d_n10;
        locals.var_t0_dn11 = assign43400_e58424_d_n11;
        locals.var_t0_dn14 = assign43400_e58424_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43410_e58441, assign43410_e58441_d_n0, assign43410_e58441_d_n2, assign43410_e58441_d_n4, assign43410_e58441_d_n5, assign43410_e58441_d_n6, assign43410_e58441_d_n7, assign43410_e58441_d_n8, assign43410_e58441_d_n9, assign43410_e58441_d_n10, assign43410_e58441_d_n11, assign43410_e58441_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign43410_e58437: f64 = (locals.var_uc_depleak + locals.var_depqfn_dlt);
        let assign43410_e58439: f64 = (assign43410_e58437 - locals.var_tmf0);
        (assign43410_e58439, (locals.var_uc_depleak_dn0 - locals.var_tmf0_dn0), (locals.var_uc_depleak_dn2 - locals.var_tmf0_dn2), (locals.var_uc_depleak_dn4 - locals.var_tmf0_dn4), (locals.var_uc_depleak_dn5 - locals.var_tmf0_dn5), (locals.var_uc_depleak_dn6 - locals.var_tmf0_dn6), (locals.var_uc_depleak_dn7 - locals.var_tmf0_dn7), (locals.var_uc_depleak_dn8 - locals.var_tmf0_dn8), (locals.var_uc_depleak_dn9 - locals.var_tmf0_dn9), (locals.var_uc_depleak_dn10 - locals.var_tmf0_dn10), (locals.var_uc_depleak_dn11 - locals.var_tmf0_dn11), (locals.var_uc_depleak_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign43410_e58441;
        locals.var_t10_dn0 = assign43410_e58441_d_n0;
        locals.var_t10_dn2 = assign43410_e58441_d_n2;
        locals.var_t10_dn4 = assign43410_e58441_d_n4;
        locals.var_t10_dn5 = assign43410_e58441_d_n5;
        locals.var_t10_dn6 = assign43410_e58441_d_n6;
        locals.var_t10_dn7 = assign43410_e58441_d_n7;
        locals.var_t10_dn8 = assign43410_e58441_d_n8;
        locals.var_t10_dn9 = assign43410_e58441_d_n9;
        locals.var_t10_dn10 = assign43410_e58441_d_n10;
        locals.var_t10_dn11 = assign43410_e58441_d_n11;
        locals.var_t10_dn14 = assign43410_e58441_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign43420_e58454, assign43420_e58454_d_n0, assign43420_e58454_d_n2, assign43420_e58454_d_n4, assign43420_e58454_d_n5, assign43420_e58454_d_n6, assign43420_e58454_d_n7, assign43420_e58454_d_n8, assign43420_e58454_d_n9, assign43420_e58454_d_n10, assign43420_e58454_d_n11, assign43420_e58454_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43420_e58454;
        locals.var_t0_dn0 = assign43420_e58454_d_n0;
        locals.var_t0_dn2 = assign43420_e58454_d_n2;
        locals.var_t0_dn4 = assign43420_e58454_d_n4;
        locals.var_t0_dn5 = assign43420_e58454_d_n5;
        locals.var_t0_dn6 = assign43420_e58454_d_n6;
        locals.var_t0_dn7 = assign43420_e58454_d_n7;
        locals.var_t0_dn8 = assign43420_e58454_d_n8;
        locals.var_t0_dn9 = assign43420_e58454_d_n9;
        locals.var_t0_dn10 = assign43420_e58454_d_n10;
        locals.var_t0_dn11 = assign43420_e58454_d_n11;
        locals.var_t0_dn14 = assign43420_e58454_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43430_e58468, assign43430_e58468_d_n0, assign43430_e58468_d_n2, assign43430_e58468_d_n4, assign43430_e58468_d_n5, assign43430_e58468_d_n6, assign43430_e58468_d_n7, assign43430_e58468_d_n8, assign43430_e58468_d_n9, assign43430_e58468_d_n10, assign43430_e58468_d_n11, assign43430_e58468_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 == 0.0)) {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign43430_e58468;
        locals.var_t10_dn0 = assign43430_e58468_d_n0;
        locals.var_t10_dn2 = assign43430_e58468_d_n2;
        locals.var_t10_dn4 = assign43430_e58468_d_n4;
        locals.var_t10_dn5 = assign43430_e58468_d_n5;
        locals.var_t10_dn6 = assign43430_e58468_d_n6;
        locals.var_t10_dn7 = assign43430_e58468_d_n7;
        locals.var_t10_dn8 = assign43430_e58468_d_n8;
        locals.var_t10_dn9 = assign43430_e58468_d_n9;
        locals.var_t10_dn10 = assign43430_e58468_d_n10;
        locals.var_t10_dn11 = assign43430_e58468_d_n11;
        locals.var_t10_dn14 = assign43430_e58468_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign43440_e58482, assign43440_e58482_d_n0, assign43440_e58482_d_n2, assign43440_e58482_d_n4, assign43440_e58482_d_n5, assign43440_e58482_d_n6, assign43440_e58482_d_n7, assign43440_e58482_d_n8, assign43440_e58482_d_n9, assign43440_e58482_d_n10, assign43440_e58482_d_n11, assign43440_e58482_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1066 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43440_e58482;
        locals.var_t0_dn0 = assign43440_e58482_d_n0;
        locals.var_t0_dn2 = assign43440_e58482_d_n2;
        locals.var_t0_dn4 = assign43440_e58482_d_n4;
        locals.var_t0_dn5 = assign43440_e58482_d_n5;
        locals.var_t0_dn6 = assign43440_e58482_d_n6;
        locals.var_t0_dn7 = assign43440_e58482_d_n7;
        locals.var_t0_dn8 = assign43440_e58482_d_n8;
        locals.var_t0_dn9 = assign43440_e58482_d_n9;
        locals.var_t0_dn10 = assign43440_e58482_d_n10;
        locals.var_t0_dn11 = assign43440_e58482_d_n11;
        locals.var_t0_dn14 = assign43440_e58482_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43450_e58495, assign43450_e58495_d_n0, assign43450_e58495_d_n2, assign43450_e58495_d_n4, assign43450_e58495_d_n5, assign43450_e58495_d_n6, assign43450_e58495_d_n7, assign43450_e58495_d_n8, assign43450_e58495_d_n9, assign43450_e58495_d_n10, assign43450_e58495_d_n11, assign43450_e58495_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let assign43450_e58493: f64 = (locals.var_vds_res / locals.var_t10);
        (assign43450_e58493, (((locals.var_vds_res_dn0 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn2 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn4 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn5 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn6 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn7 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn8 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn9 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn10 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn11 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn14 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43450_e58495;
        locals.var_t1_dn0 = assign43450_e58495_d_n0;
        locals.var_t1_dn2 = assign43450_e58495_d_n2;
        locals.var_t1_dn4 = assign43450_e58495_d_n4;
        locals.var_t1_dn5 = assign43450_e58495_d_n5;
        locals.var_t1_dn6 = assign43450_e58495_d_n6;
        locals.var_t1_dn7 = assign43450_e58495_d_n7;
        locals.var_t1_dn8 = assign43450_e58495_d_n8;
        locals.var_t1_dn9 = assign43450_e58495_d_n9;
        locals.var_t1_dn10 = assign43450_e58495_d_n10;
        locals.var_t1_dn11 = assign43450_e58495_d_n11;
        locals.var_t1_dn14 = assign43450_e58495_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign43460_e58515, assign43460_e58515_d_n0, assign43460_e58515_d_n2, assign43460_e58515_d_n4, assign43460_e58515_d_n5, assign43460_e58515_d_n6, assign43460_e58515_d_n7, assign43460_e58515_d_n8, assign43460_e58515_d_n9, assign43460_e58515_d_n10, assign43460_e58515_d_n11, assign43460_e58515_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let (assign43460_e58513, assign43460_e58513_d_n0, assign43460_e58513_d_n2, assign43460_e58513_d_n4, assign43460_e58513_d_n5, assign43460_e58513_d_n6, assign43460_e58513_d_n7, assign43460_e58513_d_n8, assign43460_e58513_d_n9, assign43460_e58513_d_n10, assign43460_e58513_d_n11, assign43460_e58513_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43460_e58511: f64 = (p.p383 - 1.0);
                let assign43460_e58512: f64 = (locals.var_t1).powf(assign43460_e58511);
                (assign43460_e58512, if 0.0 == 0.0 && ((assign43460_e58511) as f64).is_finite() && ((assign43460_e58511) as f64).fract() == 0.0 { if assign43460_e58511 == 0.0 { 0.0 } else { (assign43460_e58511 * ((locals.var_t1).powf(assign43460_e58511 - 1.0) * locals.var_t1_dn0)) } } else { (assign43460_e58512 * (assign43460_e58511 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43460_e58511) as f64).is_finite() && ((assign43460_e58511) as f64).fract() == 0.0 { if assign43460_e58511 == 0.0 { 0.0 } else { (assign43460_e58511 * ((locals.var_t1).powf(assign43460_e58511 - 1.0) * locals.var_t1_dn2)) } } else { (assign43460_e58512 * (assign43460_e58511 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43460_e58511) as f64).is_finite() && ((assign43460_e58511) as f64).fract() == 0.0 { if assign43460_e58511 == 0.0 { 0.0 } else { (assign43460_e58511 * ((locals.var_t1).powf(assign43460_e58511 - 1.0) * locals.var_t1_dn4)) } } else { (assign43460_e58512 * (assign43460_e58511 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43460_e58511) as f64).is_finite() && ((assign43460_e58511) as f64).fract() == 0.0 { if assign43460_e58511 == 0.0 { 0.0 } else { (assign43460_e58511 * ((locals.var_t1).powf(assign43460_e58511 - 1.0) * locals.var_t1_dn5)) } } else { (assign43460_e58512 * (assign43460_e58511 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43460_e58511) as f64).is_finite() && ((assign43460_e58511) as f64).fract() == 0.0 { if assign43460_e58511 == 0.0 { 0.0 } else { (assign43460_e58511 * ((locals.var_t1).powf(assign43460_e58511 - 1.0) * locals.var_t1_dn6)) } } else { (assign43460_e58512 * (assign43460_e58511 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43460_e58511) as f64).is_finite() && ((assign43460_e58511) as f64).fract() == 0.0 { if assign43460_e58511 == 0.0 { 0.0 } else { (assign43460_e58511 * ((locals.var_t1).powf(assign43460_e58511 - 1.0) * locals.var_t1_dn7)) } } else { (assign43460_e58512 * (assign43460_e58511 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43460_e58511) as f64).is_finite() && ((assign43460_e58511) as f64).fract() == 0.0 { if assign43460_e58511 == 0.0 { 0.0 } else { (assign43460_e58511 * ((locals.var_t1).powf(assign43460_e58511 - 1.0) * locals.var_t1_dn8)) } } else { (assign43460_e58512 * (assign43460_e58511 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43460_e58511) as f64).is_finite() && ((assign43460_e58511) as f64).fract() == 0.0 { if assign43460_e58511 == 0.0 { 0.0 } else { (assign43460_e58511 * ((locals.var_t1).powf(assign43460_e58511 - 1.0) * locals.var_t1_dn9)) } } else { (assign43460_e58512 * (assign43460_e58511 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43460_e58511) as f64).is_finite() && ((assign43460_e58511) as f64).fract() == 0.0 { if assign43460_e58511 == 0.0 { 0.0 } else { (assign43460_e58511 * ((locals.var_t1).powf(assign43460_e58511 - 1.0) * locals.var_t1_dn10)) } } else { (assign43460_e58512 * (assign43460_e58511 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43460_e58511) as f64).is_finite() && ((assign43460_e58511) as f64).fract() == 0.0 { if assign43460_e58511 == 0.0 { 0.0 } else { (assign43460_e58511 * ((locals.var_t1).powf(assign43460_e58511 - 1.0) * locals.var_t1_dn11)) } } else { (assign43460_e58512 * (assign43460_e58511 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43460_e58511) as f64).is_finite() && ((assign43460_e58511) as f64).fract() == 0.0 { if assign43460_e58511 == 0.0 { 0.0 } else { (assign43460_e58511 * ((locals.var_t1).powf(assign43460_e58511 - 1.0) * locals.var_t1_dn14)) } } else { (assign43460_e58512 * (assign43460_e58511 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign43460_e58513, assign43460_e58513_d_n0, assign43460_e58513_d_n2, assign43460_e58513_d_n4, assign43460_e58513_d_n5, assign43460_e58513_d_n6, assign43460_e58513_d_n7, assign43460_e58513_d_n8, assign43460_e58513_d_n9, assign43460_e58513_d_n10, assign43460_e58513_d_n11, assign43460_e58513_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign43460_e58515;
        locals.var_t2_dn0 = assign43460_e58515_d_n0;
        locals.var_t2_dn2 = assign43460_e58515_d_n2;
        locals.var_t2_dn4 = assign43460_e58515_d_n4;
        locals.var_t2_dn5 = assign43460_e58515_d_n5;
        locals.var_t2_dn6 = assign43460_e58515_d_n6;
        locals.var_t2_dn7 = assign43460_e58515_d_n7;
        locals.var_t2_dn8 = assign43460_e58515_d_n8;
        locals.var_t2_dn9 = assign43460_e58515_d_n9;
        locals.var_t2_dn10 = assign43460_e58515_d_n10;
        locals.var_t2_dn11 = assign43460_e58515_d_n11;
        locals.var_t2_dn14 = assign43460_e58515_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign43470_e58530, assign43470_e58530_d_n0, assign43470_e58530_d_n2, assign43470_e58530_d_n4, assign43470_e58530_d_n5, assign43470_e58530_d_n6, assign43470_e58530_d_n7, assign43470_e58530_d_n8, assign43470_e58530_d_n9, assign43470_e58530_d_n10, assign43470_e58530_d_n11, assign43470_e58530_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let assign43470_e58527: f64 = (locals.var_t2 * locals.var_t1);
        let assign43470_e58528: f64 = (1.0 + assign43470_e58527);
        (assign43470_e58528, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign43470_e58530;
        locals.var_t3_dn0 = assign43470_e58530_d_n0;
        locals.var_t3_dn2 = assign43470_e58530_d_n2;
        locals.var_t3_dn4 = assign43470_e58530_d_n4;
        locals.var_t3_dn5 = assign43470_e58530_d_n5;
        locals.var_t3_dn6 = assign43470_e58530_d_n6;
        locals.var_t3_dn7 = assign43470_e58530_d_n7;
        locals.var_t3_dn8 = assign43470_e58530_d_n8;
        locals.var_t3_dn9 = assign43470_e58530_d_n9;
        locals.var_t3_dn10 = assign43470_e58530_d_n10;
        locals.var_t3_dn11 = assign43470_e58530_d_n11;
        locals.var_t3_dn14 = assign43470_e58530_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign43480_e58552, assign43480_e58552_d_n0, assign43480_e58552_d_n2, assign43480_e58552_d_n4, assign43480_e58552_d_n5, assign43480_e58552_d_n6, assign43480_e58552_d_n7, assign43480_e58552_d_n8, assign43480_e58552_d_n9, assign43480_e58552_d_n10, assign43480_e58552_d_n11, assign43480_e58552_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let (assign43480_e58550, assign43480_e58550_d_n0, assign43480_e58550_d_n2, assign43480_e58550_d_n4, assign43480_e58550_d_n5, assign43480_e58550_d_n6, assign43480_e58550_d_n7, assign43480_e58550_d_n8, assign43480_e58550_d_n9, assign43480_e58550_d_n10, assign43480_e58550_d_n11, assign43480_e58550_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43480_e58546: f64 = (1.0 / p.p383);
                let assign43480_e58548: f64 = (assign43480_e58546 - 1.0);
                let assign43480_e58549: f64 = (locals.var_t3).powf(assign43480_e58548);
                (assign43480_e58549, if 0.0 == 0.0 && ((assign43480_e58548) as f64).is_finite() && ((assign43480_e58548) as f64).fract() == 0.0 { if assign43480_e58548 == 0.0 { 0.0 } else { (assign43480_e58548 * ((locals.var_t3).powf(assign43480_e58548 - 1.0) * locals.var_t3_dn0)) } } else { (assign43480_e58549 * (assign43480_e58548 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43480_e58548) as f64).is_finite() && ((assign43480_e58548) as f64).fract() == 0.0 { if assign43480_e58548 == 0.0 { 0.0 } else { (assign43480_e58548 * ((locals.var_t3).powf(assign43480_e58548 - 1.0) * locals.var_t3_dn2)) } } else { (assign43480_e58549 * (assign43480_e58548 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43480_e58548) as f64).is_finite() && ((assign43480_e58548) as f64).fract() == 0.0 { if assign43480_e58548 == 0.0 { 0.0 } else { (assign43480_e58548 * ((locals.var_t3).powf(assign43480_e58548 - 1.0) * locals.var_t3_dn4)) } } else { (assign43480_e58549 * (assign43480_e58548 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43480_e58548) as f64).is_finite() && ((assign43480_e58548) as f64).fract() == 0.0 { if assign43480_e58548 == 0.0 { 0.0 } else { (assign43480_e58548 * ((locals.var_t3).powf(assign43480_e58548 - 1.0) * locals.var_t3_dn5)) } } else { (assign43480_e58549 * (assign43480_e58548 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43480_e58548) as f64).is_finite() && ((assign43480_e58548) as f64).fract() == 0.0 { if assign43480_e58548 == 0.0 { 0.0 } else { (assign43480_e58548 * ((locals.var_t3).powf(assign43480_e58548 - 1.0) * locals.var_t3_dn6)) } } else { (assign43480_e58549 * (assign43480_e58548 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43480_e58548) as f64).is_finite() && ((assign43480_e58548) as f64).fract() == 0.0 { if assign43480_e58548 == 0.0 { 0.0 } else { (assign43480_e58548 * ((locals.var_t3).powf(assign43480_e58548 - 1.0) * locals.var_t3_dn7)) } } else { (assign43480_e58549 * (assign43480_e58548 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43480_e58548) as f64).is_finite() && ((assign43480_e58548) as f64).fract() == 0.0 { if assign43480_e58548 == 0.0 { 0.0 } else { (assign43480_e58548 * ((locals.var_t3).powf(assign43480_e58548 - 1.0) * locals.var_t3_dn8)) } } else { (assign43480_e58549 * (assign43480_e58548 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43480_e58548) as f64).is_finite() && ((assign43480_e58548) as f64).fract() == 0.0 { if assign43480_e58548 == 0.0 { 0.0 } else { (assign43480_e58548 * ((locals.var_t3).powf(assign43480_e58548 - 1.0) * locals.var_t3_dn9)) } } else { (assign43480_e58549 * (assign43480_e58548 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43480_e58548) as f64).is_finite() && ((assign43480_e58548) as f64).fract() == 0.0 { if assign43480_e58548 == 0.0 { 0.0 } else { (assign43480_e58548 * ((locals.var_t3).powf(assign43480_e58548 - 1.0) * locals.var_t3_dn10)) } } else { (assign43480_e58549 * (assign43480_e58548 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43480_e58548) as f64).is_finite() && ((assign43480_e58548) as f64).fract() == 0.0 { if assign43480_e58548 == 0.0 { 0.0 } else { (assign43480_e58548 * ((locals.var_t3).powf(assign43480_e58548 - 1.0) * locals.var_t3_dn11)) } } else { (assign43480_e58549 * (assign43480_e58548 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43480_e58548) as f64).is_finite() && ((assign43480_e58548) as f64).fract() == 0.0 { if assign43480_e58548 == 0.0 { 0.0 } else { (assign43480_e58548 * ((locals.var_t3).powf(assign43480_e58548 - 1.0) * locals.var_t3_dn14)) } } else { (assign43480_e58549 * (assign43480_e58548 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign43480_e58550, assign43480_e58550_d_n0, assign43480_e58550_d_n2, assign43480_e58550_d_n4, assign43480_e58550_d_n5, assign43480_e58550_d_n6, assign43480_e58550_d_n7, assign43480_e58550_d_n8, assign43480_e58550_d_n9, assign43480_e58550_d_n10, assign43480_e58550_d_n11, assign43480_e58550_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign43480_e58552;
        locals.var_t4_dn0 = assign43480_e58552_d_n0;
        locals.var_t4_dn2 = assign43480_e58552_d_n2;
        locals.var_t4_dn4 = assign43480_e58552_d_n4;
        locals.var_t4_dn5 = assign43480_e58552_d_n5;
        locals.var_t4_dn6 = assign43480_e58552_d_n6;
        locals.var_t4_dn7 = assign43480_e58552_d_n7;
        locals.var_t4_dn8 = assign43480_e58552_d_n8;
        locals.var_t4_dn9 = assign43480_e58552_d_n9;
        locals.var_t4_dn10 = assign43480_e58552_d_n10;
        locals.var_t4_dn11 = assign43480_e58552_d_n11;
        locals.var_t4_dn14 = assign43480_e58552_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign43490_e58565, assign43490_e58565_d_n0, assign43490_e58565_d_n2, assign43490_e58565_d_n4, assign43490_e58565_d_n5, assign43490_e58565_d_n6, assign43490_e58565_d_n7, assign43490_e58565_d_n8, assign43490_e58565_d_n9, assign43490_e58565_d_n10, assign43490_e58565_d_n11, assign43490_e58565_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let assign43490_e58563: f64 = (locals.var_t4 * locals.var_t3);
        (assign43490_e58563, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign43490_e58565;
        locals.var_t6_dn0 = assign43490_e58565_d_n0;
        locals.var_t6_dn2 = assign43490_e58565_d_n2;
        locals.var_t6_dn4 = assign43490_e58565_d_n4;
        locals.var_t6_dn5 = assign43490_e58565_d_n5;
        locals.var_t6_dn6 = assign43490_e58565_d_n6;
        locals.var_t6_dn7 = assign43490_e58565_d_n7;
        locals.var_t6_dn8 = assign43490_e58565_d_n8;
        locals.var_t6_dn9 = assign43490_e58565_d_n9;
        locals.var_t6_dn10 = assign43490_e58565_d_n10;
        locals.var_t6_dn11 = assign43490_e58565_d_n11;
        locals.var_t6_dn14 = assign43490_e58565_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign43500_e58578, assign43500_e58578_d_n0, assign43500_e58578_d_n2, assign43500_e58578_d_n4, assign43500_e58578_d_n5, assign43500_e58578_d_n6, assign43500_e58578_d_n7, assign43500_e58578_d_n8, assign43500_e58578_d_n9, assign43500_e58578_d_n10, assign43500_e58578_d_n11, assign43500_e58578_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let assign43500_e58576: f64 = (locals.var_vds_res / locals.var_t6);
        (assign43500_e58576, (((locals.var_vds_res_dn0 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn2 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn4 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn5 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn6 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn7 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn8 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn9 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn10 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn11 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn14 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign43500_e58578;
        locals.var_vds_res_dn0 = assign43500_e58578_d_n0;
        locals.var_vds_res_dn2 = assign43500_e58578_d_n2;
        locals.var_vds_res_dn4 = assign43500_e58578_d_n4;
        locals.var_vds_res_dn5 = assign43500_e58578_d_n5;
        locals.var_vds_res_dn6 = assign43500_e58578_d_n6;
        locals.var_vds_res_dn7 = assign43500_e58578_d_n7;
        locals.var_vds_res_dn8 = assign43500_e58578_d_n8;
        locals.var_vds_res_dn9 = assign43500_e58578_d_n9;
        locals.var_vds_res_dn10 = assign43500_e58578_d_n10;
        locals.var_vds_res_dn11 = assign43500_e58578_d_n11;
        locals.var_vds_res_dn14 = assign43500_e58578_d_n14;
        locals.var_vds_res_rv = 0.0;

        let (assign43510_e58589, assign43510_e58589_d_n0, assign43510_e58589_d_n2, assign43510_e58589_d_n4, assign43510_e58589_d_n5, assign43510_e58589_d_n6, assign43510_e58589_d_n7, assign43510_e58589_d_n8, assign43510_e58589_d_n9, assign43510_e58589_d_n10, assign43510_e58589_d_n11, assign43510_e58589_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43510_e58587: f64 = (locals.var_vgs - locals.var_vbsc);
        (assign43510_e58587, (-locals.var_vbsc_dn0), (-locals.var_vbsc_dn2), (-locals.var_vbsc_dn4), (-locals.var_vbsc_dn5), (locals.var_vgs_dn6 - locals.var_vbsc_dn6), (locals.var_vgs_dn7 - locals.var_vbsc_dn7), (locals.var_vgs_dn8 - locals.var_vbsc_dn8), (-locals.var_vbsc_dn9), (-locals.var_vbsc_dn10), (-locals.var_vbsc_dn11), (-locals.var_vbsc_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43510_e58589;
        locals.var_t1_dn0 = assign43510_e58589_d_n0;
        locals.var_t1_dn2 = assign43510_e58589_d_n2;
        locals.var_t1_dn4 = assign43510_e58589_d_n4;
        locals.var_t1_dn5 = assign43510_e58589_d_n5;
        locals.var_t1_dn6 = assign43510_e58589_d_n6;
        locals.var_t1_dn7 = assign43510_e58589_d_n7;
        locals.var_t1_dn8 = assign43510_e58589_d_n8;
        locals.var_t1_dn9 = assign43510_e58589_d_n9;
        locals.var_t1_dn10 = assign43510_e58589_d_n10;
        locals.var_t1_dn11 = assign43510_e58589_d_n11;
        locals.var_t1_dn14 = assign43510_e58589_d_n14;
        locals.var_t1_rv = 0.0;

        let assign43520_e58593: f64 = 1.0;
        let assign43520_e58598: f64 = if ((locals.var_t1 < assign43520_e58593) && (1.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1072 = assign43520_e58598;
        locals.var_guard1072_rv = 0.0;

        let (assign43530_e58613, assign43530_e58613_d_n0, assign43530_e58613_d_n2, assign43530_e58613_d_n4, assign43530_e58613_d_n5, assign43530_e58613_d_n6, assign43530_e58613_d_n7, assign43530_e58613_d_n8, assign43530_e58613_d_n9, assign43530_e58613_d_n10, assign43530_e58613_d_n11, assign43530_e58613_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43530_e58609: f64 = 1.0;
        let assign43530_e58611: f64 = (assign43530_e58609 - locals.var_t1);
        (assign43530_e58611, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign43530_e58613;
        locals.var_tmf1_dn0 = assign43530_e58613_d_n0;
        locals.var_tmf1_dn2 = assign43530_e58613_d_n2;
        locals.var_tmf1_dn4 = assign43530_e58613_d_n4;
        locals.var_tmf1_dn5 = assign43530_e58613_d_n5;
        locals.var_tmf1_dn6 = assign43530_e58613_d_n6;
        locals.var_tmf1_dn7 = assign43530_e58613_d_n7;
        locals.var_tmf1_dn8 = assign43530_e58613_d_n8;
        locals.var_tmf1_dn9 = assign43530_e58613_d_n9;
        locals.var_tmf1_dn10 = assign43530_e58613_d_n10;
        locals.var_tmf1_dn11 = assign43530_e58613_d_n11;
        locals.var_tmf1_dn14 = assign43530_e58613_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign43540_e58626, assign43540_e58626_d_n0, assign43540_e58626_d_n2, assign43540_e58626_d_n4, assign43540_e58626_d_n5, assign43540_e58626_d_n6, assign43540_e58626_d_n7, assign43540_e58626_d_n8, assign43540_e58626_d_n9, assign43540_e58626_d_n10, assign43540_e58626_d_n11, assign43540_e58626_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43540_e58624: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign43540_e58624, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign43540_e58626;
        locals.var_x2_dn0 = assign43540_e58626_d_n0;
        locals.var_x2_dn2 = assign43540_e58626_d_n2;
        locals.var_x2_dn4 = assign43540_e58626_d_n4;
        locals.var_x2_dn5 = assign43540_e58626_d_n5;
        locals.var_x2_dn6 = assign43540_e58626_d_n6;
        locals.var_x2_dn7 = assign43540_e58626_d_n7;
        locals.var_x2_dn8 = assign43540_e58626_d_n8;
        locals.var_x2_dn9 = assign43540_e58626_d_n9;
        locals.var_x2_dn10 = assign43540_e58626_d_n10;
        locals.var_x2_dn11 = assign43540_e58626_d_n11;
        locals.var_x2_dn14 = assign43540_e58626_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign43550_e58639, assign43550_e58639_d_n0, assign43550_e58639_d_n2, assign43550_e58639_d_n4, assign43550_e58639_d_n5, assign43550_e58639_d_n6, assign43550_e58639_d_n7, assign43550_e58639_d_n8, assign43550_e58639_d_n9, assign43550_e58639_d_n10, assign43550_e58639_d_n11, assign43550_e58639_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43550_e58637: f64 = 1.0;
        (assign43550_e58637, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign43550_e58639;
        locals.var_xmax2_dn0 = assign43550_e58639_d_n0;
        locals.var_xmax2_dn2 = assign43550_e58639_d_n2;
        locals.var_xmax2_dn4 = assign43550_e58639_d_n4;
        locals.var_xmax2_dn5 = assign43550_e58639_d_n5;
        locals.var_xmax2_dn6 = assign43550_e58639_d_n6;
        locals.var_xmax2_dn7 = assign43550_e58639_d_n7;
        locals.var_xmax2_dn8 = assign43550_e58639_d_n8;
        locals.var_xmax2_dn9 = assign43550_e58639_d_n9;
        locals.var_xmax2_dn10 = assign43550_e58639_d_n10;
        locals.var_xmax2_dn11 = assign43550_e58639_d_n11;
        locals.var_xmax2_dn14 = assign43550_e58639_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign43560_e58650, assign43560_e58650_d_n0, assign43560_e58650_d_n2, assign43560_e58650_d_n4, assign43560_e58650_d_n5, assign43560_e58650_d_n6, assign43560_e58650_d_n7, assign43560_e58650_d_n8, assign43560_e58650_d_n9, assign43560_e58650_d_n10, assign43560_e58650_d_n11, assign43560_e58650_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43560_e58650;
        locals.var_xp_dn0 = assign43560_e58650_d_n0;
        locals.var_xp_dn2 = assign43560_e58650_d_n2;
        locals.var_xp_dn4 = assign43560_e58650_d_n4;
        locals.var_xp_dn5 = assign43560_e58650_d_n5;
        locals.var_xp_dn6 = assign43560_e58650_d_n6;
        locals.var_xp_dn7 = assign43560_e58650_d_n7;
        locals.var_xp_dn8 = assign43560_e58650_d_n8;
        locals.var_xp_dn9 = assign43560_e58650_d_n9;
        locals.var_xp_dn10 = assign43560_e58650_d_n10;
        locals.var_xp_dn11 = assign43560_e58650_d_n11;
        locals.var_xp_dn14 = assign43560_e58650_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43570_e58661, assign43570_e58661_d_n0, assign43570_e58661_d_n2, assign43570_e58661_d_n4, assign43570_e58661_d_n5, assign43570_e58661_d_n6, assign43570_e58661_d_n7, assign43570_e58661_d_n8, assign43570_e58661_d_n9, assign43570_e58661_d_n10, assign43570_e58661_d_n11, assign43570_e58661_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43570_e58661;
        locals.var_xmp_dn0 = assign43570_e58661_d_n0;
        locals.var_xmp_dn2 = assign43570_e58661_d_n2;
        locals.var_xmp_dn4 = assign43570_e58661_d_n4;
        locals.var_xmp_dn5 = assign43570_e58661_d_n5;
        locals.var_xmp_dn6 = assign43570_e58661_d_n6;
        locals.var_xmp_dn7 = assign43570_e58661_d_n7;
        locals.var_xmp_dn8 = assign43570_e58661_d_n8;
        locals.var_xmp_dn9 = assign43570_e58661_d_n9;
        locals.var_xmp_dn10 = assign43570_e58661_d_n10;
        locals.var_xmp_dn11 = assign43570_e58661_d_n11;
        locals.var_xmp_dn14 = assign43570_e58661_d_n14;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_151(
        locals: &mut StampLocals,
    ) {
        let (assign43580_e58672,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43580_e58672;
        locals.var_m0_rv = 0.0;

        let (assign43590_e58683,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43590_e58683;
        locals.var_mm_rv = 0.0;

        let (assign43600_e58694, assign43600_e58694_d_n0, assign43600_e58694_d_n2, assign43600_e58694_d_n4, assign43600_e58694_d_n5, assign43600_e58694_d_n6, assign43600_e58694_d_n7, assign43600_e58694_d_n8, assign43600_e58694_d_n9, assign43600_e58694_d_n10, assign43600_e58694_d_n11, assign43600_e58694_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign43600_e58694;
        locals.var_arg_dn0 = assign43600_e58694_d_n0;
        locals.var_arg_dn2 = assign43600_e58694_d_n2;
        locals.var_arg_dn4 = assign43600_e58694_d_n4;
        locals.var_arg_dn5 = assign43600_e58694_d_n5;
        locals.var_arg_dn6 = assign43600_e58694_d_n6;
        locals.var_arg_dn7 = assign43600_e58694_d_n7;
        locals.var_arg_dn8 = assign43600_e58694_d_n8;
        locals.var_arg_dn9 = assign43600_e58694_d_n9;
        locals.var_arg_dn10 = assign43600_e58694_d_n10;
        locals.var_arg_dn11 = assign43600_e58694_d_n11;
        locals.var_arg_dn14 = assign43600_e58694_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign43610_e58705, assign43610_e58705_d_n0, assign43610_e58705_d_n2, assign43610_e58705_d_n4, assign43610_e58705_d_n5, assign43610_e58705_d_n6, assign43610_e58705_d_n7, assign43610_e58705_d_n8, assign43610_e58705_d_n9, assign43610_e58705_d_n10, assign43610_e58705_d_n11, assign43610_e58705_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43610_e58705;
        locals.var_dnm_dn0 = assign43610_e58705_d_n0;
        locals.var_dnm_dn2 = assign43610_e58705_d_n2;
        locals.var_dnm_dn4 = assign43610_e58705_d_n4;
        locals.var_dnm_dn5 = assign43610_e58705_d_n5;
        locals.var_dnm_dn6 = assign43610_e58705_d_n6;
        locals.var_dnm_dn7 = assign43610_e58705_d_n7;
        locals.var_dnm_dn8 = assign43610_e58705_d_n8;
        locals.var_dnm_dn9 = assign43610_e58705_d_n9;
        locals.var_dnm_dn10 = assign43610_e58705_d_n10;
        locals.var_dnm_dn11 = assign43610_e58705_d_n11;
        locals.var_dnm_dn14 = assign43610_e58705_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43620_e58718, assign43620_e58718_d_n0, assign43620_e58718_d_n2, assign43620_e58718_d_n4, assign43620_e58718_d_n5, assign43620_e58718_d_n6, assign43620_e58718_d_n7, assign43620_e58718_d_n8, assign43620_e58718_d_n9, assign43620_e58718_d_n10, assign43620_e58718_d_n11, assign43620_e58718_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43620_e58716: f64 = (locals.var_xp * locals.var_x2);
        (assign43620_e58716, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43620_e58718;
        locals.var_xp_dn0 = assign43620_e58718_d_n0;
        locals.var_xp_dn2 = assign43620_e58718_d_n2;
        locals.var_xp_dn4 = assign43620_e58718_d_n4;
        locals.var_xp_dn5 = assign43620_e58718_d_n5;
        locals.var_xp_dn6 = assign43620_e58718_d_n6;
        locals.var_xp_dn7 = assign43620_e58718_d_n7;
        locals.var_xp_dn8 = assign43620_e58718_d_n8;
        locals.var_xp_dn9 = assign43620_e58718_d_n9;
        locals.var_xp_dn10 = assign43620_e58718_d_n10;
        locals.var_xp_dn11 = assign43620_e58718_d_n11;
        locals.var_xp_dn14 = assign43620_e58718_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43630_e58731, assign43630_e58731_d_n0, assign43630_e58731_d_n2, assign43630_e58731_d_n4, assign43630_e58731_d_n5, assign43630_e58731_d_n6, assign43630_e58731_d_n7, assign43630_e58731_d_n8, assign43630_e58731_d_n9, assign43630_e58731_d_n10, assign43630_e58731_d_n11, assign43630_e58731_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43630_e58729: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43630_e58729, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43630_e58731;
        locals.var_xmp_dn0 = assign43630_e58731_d_n0;
        locals.var_xmp_dn2 = assign43630_e58731_d_n2;
        locals.var_xmp_dn4 = assign43630_e58731_d_n4;
        locals.var_xmp_dn5 = assign43630_e58731_d_n5;
        locals.var_xmp_dn6 = assign43630_e58731_d_n6;
        locals.var_xmp_dn7 = assign43630_e58731_d_n7;
        locals.var_xmp_dn8 = assign43630_e58731_d_n8;
        locals.var_xmp_dn9 = assign43630_e58731_d_n9;
        locals.var_xmp_dn10 = assign43630_e58731_d_n10;
        locals.var_xmp_dn11 = assign43630_e58731_d_n11;
        locals.var_xmp_dn14 = assign43630_e58731_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign43640_e58744, assign43640_e58744_d_n0, assign43640_e58744_d_n2, assign43640_e58744_d_n4, assign43640_e58744_d_n5, assign43640_e58744_d_n6, assign43640_e58744_d_n7, assign43640_e58744_d_n8, assign43640_e58744_d_n9, assign43640_e58744_d_n10, assign43640_e58744_d_n11, assign43640_e58744_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43640_e58742: f64 = (locals.var_xp * locals.var_x2);
        (assign43640_e58742, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43640_e58744;
        locals.var_xp_dn0 = assign43640_e58744_d_n0;
        locals.var_xp_dn2 = assign43640_e58744_d_n2;
        locals.var_xp_dn4 = assign43640_e58744_d_n4;
        locals.var_xp_dn5 = assign43640_e58744_d_n5;
        locals.var_xp_dn6 = assign43640_e58744_d_n6;
        locals.var_xp_dn7 = assign43640_e58744_d_n7;
        locals.var_xp_dn8 = assign43640_e58744_d_n8;
        locals.var_xp_dn9 = assign43640_e58744_d_n9;
        locals.var_xp_dn10 = assign43640_e58744_d_n10;
        locals.var_xp_dn11 = assign43640_e58744_d_n11;
        locals.var_xp_dn14 = assign43640_e58744_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43650_e58757, assign43650_e58757_d_n0, assign43650_e58757_d_n2, assign43650_e58757_d_n4, assign43650_e58757_d_n5, assign43650_e58757_d_n6, assign43650_e58757_d_n7, assign43650_e58757_d_n8, assign43650_e58757_d_n9, assign43650_e58757_d_n10, assign43650_e58757_d_n11, assign43650_e58757_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43650_e58755: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43650_e58755, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43650_e58757;
        locals.var_xmp_dn0 = assign43650_e58757_d_n0;
        locals.var_xmp_dn2 = assign43650_e58757_d_n2;
        locals.var_xmp_dn4 = assign43650_e58757_d_n4;
        locals.var_xmp_dn5 = assign43650_e58757_d_n5;
        locals.var_xmp_dn6 = assign43650_e58757_d_n6;
        locals.var_xmp_dn7 = assign43650_e58757_d_n7;
        locals.var_xmp_dn8 = assign43650_e58757_d_n8;
        locals.var_xmp_dn9 = assign43650_e58757_d_n9;
        locals.var_xmp_dn10 = assign43650_e58757_d_n10;
        locals.var_xmp_dn11 = assign43650_e58757_d_n11;
        locals.var_xmp_dn14 = assign43650_e58757_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign43660_e58770, assign43660_e58770_d_n0, assign43660_e58770_d_n2, assign43660_e58770_d_n4, assign43660_e58770_d_n5, assign43660_e58770_d_n6, assign43660_e58770_d_n7, assign43660_e58770_d_n8, assign43660_e58770_d_n9, assign43660_e58770_d_n10, assign43660_e58770_d_n11, assign43660_e58770_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43660_e58768: f64 = (locals.var_xp + locals.var_xmp);
        (assign43660_e58768, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign43660_e58770;
        locals.var_arg_dn0 = assign43660_e58770_d_n0;
        locals.var_arg_dn2 = assign43660_e58770_d_n2;
        locals.var_arg_dn4 = assign43660_e58770_d_n4;
        locals.var_arg_dn5 = assign43660_e58770_d_n5;
        locals.var_arg_dn6 = assign43660_e58770_d_n6;
        locals.var_arg_dn7 = assign43660_e58770_d_n7;
        locals.var_arg_dn8 = assign43660_e58770_d_n8;
        locals.var_arg_dn9 = assign43660_e58770_d_n9;
        locals.var_arg_dn10 = assign43660_e58770_d_n10;
        locals.var_arg_dn11 = assign43660_e58770_d_n11;
        locals.var_arg_dn14 = assign43660_e58770_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign43670_e58781, assign43670_e58781_d_n0, assign43670_e58781_d_n2, assign43670_e58781_d_n4, assign43670_e58781_d_n5, assign43670_e58781_d_n6, assign43670_e58781_d_n7, assign43670_e58781_d_n8, assign43670_e58781_d_n9, assign43670_e58781_d_n10, assign43670_e58781_d_n11, assign43670_e58781_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43670_e58781;
        locals.var_dnm_dn0 = assign43670_e58781_d_n0;
        locals.var_dnm_dn2 = assign43670_e58781_d_n2;
        locals.var_dnm_dn4 = assign43670_e58781_d_n4;
        locals.var_dnm_dn5 = assign43670_e58781_d_n5;
        locals.var_dnm_dn6 = assign43670_e58781_d_n6;
        locals.var_dnm_dn7 = assign43670_e58781_d_n7;
        locals.var_dnm_dn8 = assign43670_e58781_d_n8;
        locals.var_dnm_dn9 = assign43670_e58781_d_n9;
        locals.var_dnm_dn10 = assign43670_e58781_d_n10;
        locals.var_dnm_dn11 = assign43670_e58781_d_n11;
        locals.var_dnm_dn14 = assign43670_e58781_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign43680_e58796: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1073 = assign43680_e58796;
        locals.var_guard1073_rv = 0.0;

        let assign43690_e58799: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1074 = assign43690_e58799;
        locals.var_guard1074_rv = 0.0;

        let (assign43700_e58814,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43700_e58814;
        locals.var_mm_rv = 0.0;

        let assign43710_e58817: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1075 = assign43710_e58817;
        locals.var_guard1075_rv = 0.0;

        let (assign43720_e58835,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) && (locals.var_guard1075 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43720_e58835;
        locals.var_mm_rv = 0.0;

        let assign43730_e58838: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1076 = assign43730_e58838;
        locals.var_guard1076_rv = 0.0;

        let (assign43740_e58859,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1076 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43740_e58859;
        locals.var_mm_rv = 0.0;

        let assign43750_e58862: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1077 = assign43750_e58862;
        locals.var_guard1077_rv = 0.0;

        let (assign43760_e58886,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1076 == 0.0)) && (locals.var_guard1077 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43760_e58886;
        locals.var_mm_rv = 0.0;

        let (assign43770_e58899,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43770_e58899;
        locals.var_m0_rv = 0.0;

        let mut assign43780_loop_guard: usize = 0;
        while {
            let assign43780_cond_e58913: f64 = if (((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign43780_cond_e58913 != 0.0
        } {
            assign43780_loop_guard += 1;
            assert!(assign43780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign43780_body0_e58927, assign43780_body0_e58927_d_n0, assign43780_body0_e58927_d_n2, assign43780_body0_e58927_d_n4, assign43780_body0_e58927_d_n5, assign43780_body0_e58927_d_n6, assign43780_body0_e58927_d_n7, assign43780_body0_e58927_d_n8, assign43780_body0_e58927_d_n9, assign43780_body0_e58927_d_n10, assign43780_body0_e58927_d_n11, assign43780_body0_e58927_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) {
        let assign43780_body0_e58925: f64 = (locals.var_dnm).sqrt();
        (assign43780_body0_e58925, (locals.var_dnm_dn0 / (2.0 * assign43780_body0_e58925)), (locals.var_dnm_dn2 / (2.0 * assign43780_body0_e58925)), (locals.var_dnm_dn4 / (2.0 * assign43780_body0_e58925)), (locals.var_dnm_dn5 / (2.0 * assign43780_body0_e58925)), (locals.var_dnm_dn6 / (2.0 * assign43780_body0_e58925)), (locals.var_dnm_dn7 / (2.0 * assign43780_body0_e58925)), (locals.var_dnm_dn8 / (2.0 * assign43780_body0_e58925)), (locals.var_dnm_dn9 / (2.0 * assign43780_body0_e58925)), (locals.var_dnm_dn10 / (2.0 * assign43780_body0_e58925)), (locals.var_dnm_dn11 / (2.0 * assign43780_body0_e58925)), (locals.var_dnm_dn14 / (2.0 * assign43780_body0_e58925)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign43780_body0_e58927;
            locals.var_dnm_dn0 = assign43780_body0_e58927_d_n0;
            locals.var_dnm_dn2 = assign43780_body0_e58927_d_n2;
            locals.var_dnm_dn4 = assign43780_body0_e58927_d_n4;
            locals.var_dnm_dn5 = assign43780_body0_e58927_d_n5;
            locals.var_dnm_dn6 = assign43780_body0_e58927_d_n6;
            locals.var_dnm_dn7 = assign43780_body0_e58927_d_n7;
            locals.var_dnm_dn8 = assign43780_body0_e58927_d_n8;
            locals.var_dnm_dn9 = assign43780_body0_e58927_d_n9;
            locals.var_dnm_dn10 = assign43780_body0_e58927_d_n10;
            locals.var_dnm_dn11 = assign43780_body0_e58927_d_n11;
            locals.var_dnm_dn14 = assign43780_body0_e58927_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign43780_body1_e58942,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) {
        let assign43780_body1_e58940: f64 = (locals.var_m0 + 1.0);
        (assign43780_body1_e58940,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign43780_body1_e58942;
            locals.var_m0_rv = 0.0;
        }

        let (assign43790_e58967, assign43790_e58967_d_n0, assign43790_e58967_d_n2, assign43790_e58967_d_n4, assign43790_e58967_d_n5, assign43790_e58967_d_n6, assign43790_e58967_d_n7, assign43790_e58967_d_n8, assign43790_e58967_d_n9, assign43790_e58967_d_n10, assign43790_e58967_d_n11, assign43790_e58967_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 == 0.0)) {
        let (assign43790_e58965, assign43790_e58965_d_n0, assign43790_e58965_d_n2, assign43790_e58965_d_n4, assign43790_e58965_d_n5, assign43790_e58965_d_n6, assign43790_e58965_d_n7, assign43790_e58965_d_n8, assign43790_e58965_d_n9, assign43790_e58965_d_n10, assign43790_e58965_d_n11, assign43790_e58965_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43790_e58962: f64 = (2.0 * 2.0);
                let assign43790_e58963: f64 = (1.0 / assign43790_e58962);
                let assign43790_e58964: f64 = (locals.var_dnm).powf(assign43790_e58963);
                (assign43790_e58964, if 0.0 == 0.0 && ((assign43790_e58963) as f64).is_finite() && ((assign43790_e58963) as f64).fract() == 0.0 { if assign43790_e58963 == 0.0 { 0.0 } else { (assign43790_e58963 * ((locals.var_dnm).powf(assign43790_e58963 - 1.0) * locals.var_dnm_dn0)) } } else { (assign43790_e58964 * (assign43790_e58963 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43790_e58963) as f64).is_finite() && ((assign43790_e58963) as f64).fract() == 0.0 { if assign43790_e58963 == 0.0 { 0.0 } else { (assign43790_e58963 * ((locals.var_dnm).powf(assign43790_e58963 - 1.0) * locals.var_dnm_dn2)) } } else { (assign43790_e58964 * (assign43790_e58963 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43790_e58963) as f64).is_finite() && ((assign43790_e58963) as f64).fract() == 0.0 { if assign43790_e58963 == 0.0 { 0.0 } else { (assign43790_e58963 * ((locals.var_dnm).powf(assign43790_e58963 - 1.0) * locals.var_dnm_dn4)) } } else { (assign43790_e58964 * (assign43790_e58963 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43790_e58963) as f64).is_finite() && ((assign43790_e58963) as f64).fract() == 0.0 { if assign43790_e58963 == 0.0 { 0.0 } else { (assign43790_e58963 * ((locals.var_dnm).powf(assign43790_e58963 - 1.0) * locals.var_dnm_dn5)) } } else { (assign43790_e58964 * (assign43790_e58963 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43790_e58963) as f64).is_finite() && ((assign43790_e58963) as f64).fract() == 0.0 { if assign43790_e58963 == 0.0 { 0.0 } else { (assign43790_e58963 * ((locals.var_dnm).powf(assign43790_e58963 - 1.0) * locals.var_dnm_dn6)) } } else { (assign43790_e58964 * (assign43790_e58963 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43790_e58963) as f64).is_finite() && ((assign43790_e58963) as f64).fract() == 0.0 { if assign43790_e58963 == 0.0 { 0.0 } else { (assign43790_e58963 * ((locals.var_dnm).powf(assign43790_e58963 - 1.0) * locals.var_dnm_dn7)) } } else { (assign43790_e58964 * (assign43790_e58963 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43790_e58963) as f64).is_finite() && ((assign43790_e58963) as f64).fract() == 0.0 { if assign43790_e58963 == 0.0 { 0.0 } else { (assign43790_e58963 * ((locals.var_dnm).powf(assign43790_e58963 - 1.0) * locals.var_dnm_dn8)) } } else { (assign43790_e58964 * (assign43790_e58963 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43790_e58963) as f64).is_finite() && ((assign43790_e58963) as f64).fract() == 0.0 { if assign43790_e58963 == 0.0 { 0.0 } else { (assign43790_e58963 * ((locals.var_dnm).powf(assign43790_e58963 - 1.0) * locals.var_dnm_dn9)) } } else { (assign43790_e58964 * (assign43790_e58963 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43790_e58963) as f64).is_finite() && ((assign43790_e58963) as f64).fract() == 0.0 { if assign43790_e58963 == 0.0 { 0.0 } else { (assign43790_e58963 * ((locals.var_dnm).powf(assign43790_e58963 - 1.0) * locals.var_dnm_dn10)) } } else { (assign43790_e58964 * (assign43790_e58963 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43790_e58963) as f64).is_finite() && ((assign43790_e58963) as f64).fract() == 0.0 { if assign43790_e58963 == 0.0 { 0.0 } else { (assign43790_e58963 * ((locals.var_dnm).powf(assign43790_e58963 - 1.0) * locals.var_dnm_dn11)) } } else { (assign43790_e58964 * (assign43790_e58963 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43790_e58963) as f64).is_finite() && ((assign43790_e58963) as f64).fract() == 0.0 { if assign43790_e58963 == 0.0 { 0.0 } else { (assign43790_e58963 * ((locals.var_dnm).powf(assign43790_e58963 - 1.0) * locals.var_dnm_dn14)) } } else { (assign43790_e58964 * (assign43790_e58963 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign43790_e58965, assign43790_e58965_d_n0, assign43790_e58965_d_n2, assign43790_e58965_d_n4, assign43790_e58965_d_n5, assign43790_e58965_d_n6, assign43790_e58965_d_n7, assign43790_e58965_d_n8, assign43790_e58965_d_n9, assign43790_e58965_d_n10, assign43790_e58965_d_n11, assign43790_e58965_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43790_e58967;
        locals.var_dnm_dn0 = assign43790_e58967_d_n0;
        locals.var_dnm_dn2 = assign43790_e58967_d_n2;
        locals.var_dnm_dn4 = assign43790_e58967_d_n4;
        locals.var_dnm_dn5 = assign43790_e58967_d_n5;
        locals.var_dnm_dn6 = assign43790_e58967_d_n6;
        locals.var_dnm_dn7 = assign43790_e58967_d_n7;
        locals.var_dnm_dn8 = assign43790_e58967_d_n8;
        locals.var_dnm_dn9 = assign43790_e58967_d_n9;
        locals.var_dnm_dn10 = assign43790_e58967_d_n10;
        locals.var_dnm_dn11 = assign43790_e58967_d_n11;
        locals.var_dnm_dn14 = assign43790_e58967_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43800_e58980, assign43800_e58980_d_n0, assign43800_e58980_d_n2, assign43800_e58980_d_n4, assign43800_e58980_d_n5, assign43800_e58980_d_n6, assign43800_e58980_d_n7, assign43800_e58980_d_n8, assign43800_e58980_d_n9, assign43800_e58980_d_n10, assign43800_e58980_d_n11, assign43800_e58980_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43800_e58978: f64 = (1.0 / locals.var_dnm);
        (assign43800_e58978, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43800_e58980;
        locals.var_dnm_dn0 = assign43800_e58980_d_n0;
        locals.var_dnm_dn2 = assign43800_e58980_d_n2;
        locals.var_dnm_dn4 = assign43800_e58980_d_n4;
        locals.var_dnm_dn5 = assign43800_e58980_d_n5;
        locals.var_dnm_dn6 = assign43800_e58980_d_n6;
        locals.var_dnm_dn7 = assign43800_e58980_d_n7;
        locals.var_dnm_dn8 = assign43800_e58980_d_n8;
        locals.var_dnm_dn9 = assign43800_e58980_d_n9;
        locals.var_dnm_dn10 = assign43800_e58980_d_n10;
        locals.var_dnm_dn11 = assign43800_e58980_d_n11;
        locals.var_dnm_dn14 = assign43800_e58980_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43810_e58995, assign43810_e58995_d_n0, assign43810_e58995_d_n2, assign43810_e58995_d_n4, assign43810_e58995_d_n5, assign43810_e58995_d_n6, assign43810_e58995_d_n7, assign43810_e58995_d_n8, assign43810_e58995_d_n9, assign43810_e58995_d_n10, assign43810_e58995_d_n11, assign43810_e58995_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43810_e58991: f64 = locals.var_tmf1;
        let assign43810_e58993: f64 = (assign43810_e58991 * locals.var_dnm);
        (assign43810_e58993, ((locals.var_tmf1_dn0 * locals.var_dnm) + (assign43810_e58991 * locals.var_dnm_dn0)), ((locals.var_tmf1_dn2 * locals.var_dnm) + (assign43810_e58991 * locals.var_dnm_dn2)), ((locals.var_tmf1_dn4 * locals.var_dnm) + (assign43810_e58991 * locals.var_dnm_dn4)), ((locals.var_tmf1_dn5 * locals.var_dnm) + (assign43810_e58991 * locals.var_dnm_dn5)), ((locals.var_tmf1_dn6 * locals.var_dnm) + (assign43810_e58991 * locals.var_dnm_dn6)), ((locals.var_tmf1_dn7 * locals.var_dnm) + (assign43810_e58991 * locals.var_dnm_dn7)), ((locals.var_tmf1_dn8 * locals.var_dnm) + (assign43810_e58991 * locals.var_dnm_dn8)), ((locals.var_tmf1_dn9 * locals.var_dnm) + (assign43810_e58991 * locals.var_dnm_dn9)), ((locals.var_tmf1_dn10 * locals.var_dnm) + (assign43810_e58991 * locals.var_dnm_dn10)), ((locals.var_tmf1_dn11 * locals.var_dnm) + (assign43810_e58991 * locals.var_dnm_dn11)), ((locals.var_tmf1_dn14 * locals.var_dnm) + (assign43810_e58991 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign43810_e58995;
        locals.var_tmf0_dn0 = assign43810_e58995_d_n0;
        locals.var_tmf0_dn2 = assign43810_e58995_d_n2;
        locals.var_tmf0_dn4 = assign43810_e58995_d_n4;
        locals.var_tmf0_dn5 = assign43810_e58995_d_n5;
        locals.var_tmf0_dn6 = assign43810_e58995_d_n6;
        locals.var_tmf0_dn7 = assign43810_e58995_d_n7;
        locals.var_tmf0_dn8 = assign43810_e58995_d_n8;
        locals.var_tmf0_dn9 = assign43810_e58995_d_n9;
        locals.var_tmf0_dn10 = assign43810_e58995_d_n10;
        locals.var_tmf0_dn11 = assign43810_e58995_d_n11;
        locals.var_tmf0_dn14 = assign43810_e58995_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign43820_e59012, assign43820_e59012_d_n0, assign43820_e59012_d_n2, assign43820_e59012_d_n4, assign43820_e59012_d_n5, assign43820_e59012_d_n6, assign43820_e59012_d_n7, assign43820_e59012_d_n8, assign43820_e59012_d_n9, assign43820_e59012_d_n10, assign43820_e59012_d_n11, assign43820_e59012_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43820_e59006: f64 = locals.var_xmp;
        let assign43820_e59008: f64 = (assign43820_e59006 * locals.var_dnm);
        let assign43820_e59010: f64 = (assign43820_e59008 / locals.var_arg);
        (assign43820_e59010, (((((locals.var_xmp_dn0 * locals.var_dnm) + (assign43820_e59006 * locals.var_dnm_dn0)) * locals.var_arg) - (assign43820_e59008 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn2 * locals.var_dnm) + (assign43820_e59006 * locals.var_dnm_dn2)) * locals.var_arg) - (assign43820_e59008 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn4 * locals.var_dnm) + (assign43820_e59006 * locals.var_dnm_dn4)) * locals.var_arg) - (assign43820_e59008 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn5 * locals.var_dnm) + (assign43820_e59006 * locals.var_dnm_dn5)) * locals.var_arg) - (assign43820_e59008 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn6 * locals.var_dnm) + (assign43820_e59006 * locals.var_dnm_dn6)) * locals.var_arg) - (assign43820_e59008 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn7 * locals.var_dnm) + (assign43820_e59006 * locals.var_dnm_dn7)) * locals.var_arg) - (assign43820_e59008 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn8 * locals.var_dnm) + (assign43820_e59006 * locals.var_dnm_dn8)) * locals.var_arg) - (assign43820_e59008 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn9 * locals.var_dnm) + (assign43820_e59006 * locals.var_dnm_dn9)) * locals.var_arg) - (assign43820_e59008 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn10 * locals.var_dnm) + (assign43820_e59006 * locals.var_dnm_dn10)) * locals.var_arg) - (assign43820_e59008 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn11 * locals.var_dnm) + (assign43820_e59006 * locals.var_dnm_dn11)) * locals.var_arg) - (assign43820_e59008 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn14 * locals.var_dnm) + (assign43820_e59006 * locals.var_dnm_dn14)) * locals.var_arg) - (assign43820_e59008 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43820_e59012;
        locals.var_t0_dn0 = assign43820_e59012_d_n0;
        locals.var_t0_dn2 = assign43820_e59012_d_n2;
        locals.var_t0_dn4 = assign43820_e59012_d_n4;
        locals.var_t0_dn5 = assign43820_e59012_d_n5;
        locals.var_t0_dn6 = assign43820_e59012_d_n6;
        locals.var_t0_dn7 = assign43820_e59012_d_n7;
        locals.var_t0_dn8 = assign43820_e59012_d_n8;
        locals.var_t0_dn9 = assign43820_e59012_d_n9;
        locals.var_t0_dn10 = assign43820_e59012_d_n10;
        locals.var_t0_dn11 = assign43820_e59012_d_n11;
        locals.var_t0_dn14 = assign43820_e59012_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43830_e59027, assign43830_e59027_d_n0, assign43830_e59027_d_n2, assign43830_e59027_d_n4, assign43830_e59027_d_n5, assign43830_e59027_d_n6, assign43830_e59027_d_n7, assign43830_e59027_d_n8, assign43830_e59027_d_n9, assign43830_e59027_d_n10, assign43830_e59027_d_n11, assign43830_e59027_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        let assign43830_e59023: f64 = 1.0;
        let assign43830_e59025: f64 = (assign43830_e59023 - locals.var_tmf0);
        (assign43830_e59025, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43830_e59027;
        locals.var_t1_dn0 = assign43830_e59027_d_n0;
        locals.var_t1_dn2 = assign43830_e59027_d_n2;
        locals.var_t1_dn4 = assign43830_e59027_d_n4;
        locals.var_t1_dn5 = assign43830_e59027_d_n5;
        locals.var_t1_dn6 = assign43830_e59027_d_n6;
        locals.var_t1_dn7 = assign43830_e59027_d_n7;
        locals.var_t1_dn8 = assign43830_e59027_d_n8;
        locals.var_t1_dn9 = assign43830_e59027_d_n9;
        locals.var_t1_dn10 = assign43830_e59027_d_n10;
        locals.var_t1_dn11 = assign43830_e59027_d_n11;
        locals.var_t1_dn14 = assign43830_e59027_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign43840_e59038, assign43840_e59038_d_n0, assign43840_e59038_d_n2, assign43840_e59038_d_n4, assign43840_e59038_d_n5, assign43840_e59038_d_n6, assign43840_e59038_d_n7, assign43840_e59038_d_n8, assign43840_e59038_d_n9, assign43840_e59038_d_n10, assign43840_e59038_d_n11, assign43840_e59038_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43840_e59038;
        locals.var_t0_dn0 = assign43840_e59038_d_n0;
        locals.var_t0_dn2 = assign43840_e59038_d_n2;
        locals.var_t0_dn4 = assign43840_e59038_d_n4;
        locals.var_t0_dn5 = assign43840_e59038_d_n5;
        locals.var_t0_dn6 = assign43840_e59038_d_n6;
        locals.var_t0_dn7 = assign43840_e59038_d_n7;
        locals.var_t0_dn8 = assign43840_e59038_d_n8;
        locals.var_t0_dn9 = assign43840_e59038_d_n9;
        locals.var_t0_dn10 = assign43840_e59038_d_n10;
        locals.var_t0_dn11 = assign43840_e59038_d_n11;
        locals.var_t0_dn14 = assign43840_e59038_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43850_e59050, assign43850_e59050_d_n0, assign43850_e59050_d_n2, assign43850_e59050_d_n4, assign43850_e59050_d_n5, assign43850_e59050_d_n6, assign43850_e59050_d_n7, assign43850_e59050_d_n8, assign43850_e59050_d_n9, assign43850_e59050_d_n10, assign43850_e59050_d_n11, assign43850_e59050_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43850_e59050;
        locals.var_t1_dn0 = assign43850_e59050_d_n0;
        locals.var_t1_dn2 = assign43850_e59050_d_n2;
        locals.var_t1_dn4 = assign43850_e59050_d_n4;
        locals.var_t1_dn5 = assign43850_e59050_d_n5;
        locals.var_t1_dn6 = assign43850_e59050_d_n6;
        locals.var_t1_dn7 = assign43850_e59050_d_n7;
        locals.var_t1_dn8 = assign43850_e59050_d_n8;
        locals.var_t1_dn9 = assign43850_e59050_d_n9;
        locals.var_t1_dn10 = assign43850_e59050_d_n10;
        locals.var_t1_dn11 = assign43850_e59050_d_n11;
        locals.var_t1_dn14 = assign43850_e59050_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign43860_e59062, assign43860_e59062_d_n0, assign43860_e59062_d_n2, assign43860_e59062_d_n4, assign43860_e59062_d_n5, assign43860_e59062_d_n6, assign43860_e59062_d_n7, assign43860_e59062_d_n8, assign43860_e59062_d_n9, assign43860_e59062_d_n10, assign43860_e59062_d_n11, assign43860_e59062_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1072 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43860_e59062;
        locals.var_t0_dn0 = assign43860_e59062_d_n0;
        locals.var_t0_dn2 = assign43860_e59062_d_n2;
        locals.var_t0_dn4 = assign43860_e59062_d_n4;
        locals.var_t0_dn5 = assign43860_e59062_d_n5;
        locals.var_t0_dn6 = assign43860_e59062_d_n6;
        locals.var_t0_dn7 = assign43860_e59062_d_n7;
        locals.var_t0_dn8 = assign43860_e59062_d_n8;
        locals.var_t0_dn9 = assign43860_e59062_d_n9;
        locals.var_t0_dn10 = assign43860_e59062_d_n10;
        locals.var_t0_dn11 = assign43860_e59062_d_n11;
        locals.var_t0_dn14 = assign43860_e59062_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43870_e59073, assign43870_e59073_d_n0, assign43870_e59073_d_n2, assign43870_e59073_d_n4, assign43870_e59073_d_n5, assign43870_e59073_d_n6, assign43870_e59073_d_n7, assign43870_e59073_d_n8, assign43870_e59073_d_n9, assign43870_e59073_d_n10, assign43870_e59073_d_n11, assign43870_e59073_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43870_e59071: f64 = (locals.var_t1 / locals.var_uc_depthn);
        (assign43870_e59071, (((locals.var_t1_dn0 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn0)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn2 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn2)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn4 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn4)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn5 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn5)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn6 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn6)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn7 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn7)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn8 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn8)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn9 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn9)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn10 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn10)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn11 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn11)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn14 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn14)) / (locals.var_uc_depthn * locals.var_uc_depthn)),)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign43870_e59073;
        locals.var_eeff_dn0 = assign43870_e59073_d_n0;
        locals.var_eeff_dn2 = assign43870_e59073_d_n2;
        locals.var_eeff_dn4 = assign43870_e59073_d_n4;
        locals.var_eeff_dn5 = assign43870_e59073_d_n5;
        locals.var_eeff_dn6 = assign43870_e59073_d_n6;
        locals.var_eeff_dn7 = assign43870_e59073_d_n7;
        locals.var_eeff_dn8 = assign43870_e59073_d_n8;
        locals.var_eeff_dn9 = assign43870_e59073_d_n9;
        locals.var_eeff_dn10 = assign43870_e59073_d_n10;
        locals.var_eeff_dn11 = assign43870_e59073_d_n11;
        locals.var_eeff_dn14 = assign43870_e59073_d_n14;
        locals.var_eeff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_152(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign43880_e59091, assign43880_e59091_d_n0, assign43880_e59091_d_n2, assign43880_e59091_d_n4, assign43880_e59091_d_n5, assign43880_e59091_d_n6, assign43880_e59091_d_n7, assign43880_e59091_d_n8, assign43880_e59091_d_n9, assign43880_e59091_d_n10, assign43880_e59091_d_n11, assign43880_e59091_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let (assign43880_e59089, assign43880_e59089_d_n0, assign43880_e59089_d_n2, assign43880_e59089_d_n4, assign43880_e59089_d_n5, assign43880_e59089_d_n6, assign43880_e59089_d_n7, assign43880_e59089_d_n8, assign43880_e59089_d_n9, assign43880_e59089_d_n10, assign43880_e59089_d_n11, assign43880_e59089_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43880_e59087: f64 = (p.p353 - 1.0);
                let assign43880_e59088: f64 = (locals.var_eeff).powf(assign43880_e59087);
                (assign43880_e59088, if 0.0 == 0.0 && ((assign43880_e59087) as f64).is_finite() && ((assign43880_e59087) as f64).fract() == 0.0 { if assign43880_e59087 == 0.0 { 0.0 } else { (assign43880_e59087 * ((locals.var_eeff).powf(assign43880_e59087 - 1.0) * locals.var_eeff_dn0)) } } else { (assign43880_e59088 * (assign43880_e59087 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43880_e59087) as f64).is_finite() && ((assign43880_e59087) as f64).fract() == 0.0 { if assign43880_e59087 == 0.0 { 0.0 } else { (assign43880_e59087 * ((locals.var_eeff).powf(assign43880_e59087 - 1.0) * locals.var_eeff_dn2)) } } else { (assign43880_e59088 * (assign43880_e59087 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43880_e59087) as f64).is_finite() && ((assign43880_e59087) as f64).fract() == 0.0 { if assign43880_e59087 == 0.0 { 0.0 } else { (assign43880_e59087 * ((locals.var_eeff).powf(assign43880_e59087 - 1.0) * locals.var_eeff_dn4)) } } else { (assign43880_e59088 * (assign43880_e59087 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43880_e59087) as f64).is_finite() && ((assign43880_e59087) as f64).fract() == 0.0 { if assign43880_e59087 == 0.0 { 0.0 } else { (assign43880_e59087 * ((locals.var_eeff).powf(assign43880_e59087 - 1.0) * locals.var_eeff_dn5)) } } else { (assign43880_e59088 * (assign43880_e59087 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43880_e59087) as f64).is_finite() && ((assign43880_e59087) as f64).fract() == 0.0 { if assign43880_e59087 == 0.0 { 0.0 } else { (assign43880_e59087 * ((locals.var_eeff).powf(assign43880_e59087 - 1.0) * locals.var_eeff_dn6)) } } else { (assign43880_e59088 * (assign43880_e59087 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43880_e59087) as f64).is_finite() && ((assign43880_e59087) as f64).fract() == 0.0 { if assign43880_e59087 == 0.0 { 0.0 } else { (assign43880_e59087 * ((locals.var_eeff).powf(assign43880_e59087 - 1.0) * locals.var_eeff_dn7)) } } else { (assign43880_e59088 * (assign43880_e59087 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43880_e59087) as f64).is_finite() && ((assign43880_e59087) as f64).fract() == 0.0 { if assign43880_e59087 == 0.0 { 0.0 } else { (assign43880_e59087 * ((locals.var_eeff).powf(assign43880_e59087 - 1.0) * locals.var_eeff_dn8)) } } else { (assign43880_e59088 * (assign43880_e59087 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43880_e59087) as f64).is_finite() && ((assign43880_e59087) as f64).fract() == 0.0 { if assign43880_e59087 == 0.0 { 0.0 } else { (assign43880_e59087 * ((locals.var_eeff).powf(assign43880_e59087 - 1.0) * locals.var_eeff_dn9)) } } else { (assign43880_e59088 * (assign43880_e59087 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43880_e59087) as f64).is_finite() && ((assign43880_e59087) as f64).fract() == 0.0 { if assign43880_e59087 == 0.0 { 0.0 } else { (assign43880_e59087 * ((locals.var_eeff).powf(assign43880_e59087 - 1.0) * locals.var_eeff_dn10)) } } else { (assign43880_e59088 * (assign43880_e59087 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43880_e59087) as f64).is_finite() && ((assign43880_e59087) as f64).fract() == 0.0 { if assign43880_e59087 == 0.0 { 0.0 } else { (assign43880_e59087 * ((locals.var_eeff).powf(assign43880_e59087 - 1.0) * locals.var_eeff_dn11)) } } else { (assign43880_e59088 * (assign43880_e59087 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43880_e59087) as f64).is_finite() && ((assign43880_e59087) as f64).fract() == 0.0 { if assign43880_e59087 == 0.0 { 0.0 } else { (assign43880_e59087 * ((locals.var_eeff).powf(assign43880_e59087 - 1.0) * locals.var_eeff_dn14)) } } else { (assign43880_e59088 * (assign43880_e59087 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign43880_e59089, assign43880_e59089_d_n0, assign43880_e59089_d_n2, assign43880_e59089_d_n4, assign43880_e59089_d_n5, assign43880_e59089_d_n6, assign43880_e59089_d_n7, assign43880_e59089_d_n8, assign43880_e59089_d_n9, assign43880_e59089_d_n10, assign43880_e59089_d_n11, assign43880_e59089_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign43880_e59091;
        locals.var_t5_dn0 = assign43880_e59091_d_n0;
        locals.var_t5_dn2 = assign43880_e59091_d_n2;
        locals.var_t5_dn4 = assign43880_e59091_d_n4;
        locals.var_t5_dn5 = assign43880_e59091_d_n5;
        locals.var_t5_dn6 = assign43880_e59091_d_n6;
        locals.var_t5_dn7 = assign43880_e59091_d_n7;
        locals.var_t5_dn8 = assign43880_e59091_d_n8;
        locals.var_t5_dn9 = assign43880_e59091_d_n9;
        locals.var_t5_dn10 = assign43880_e59091_d_n10;
        locals.var_t5_dn11 = assign43880_e59091_d_n11;
        locals.var_t5_dn14 = assign43880_e59091_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign43890_e59102, assign43890_e59102_d_n0, assign43890_e59102_d_n2, assign43890_e59102_d_n4, assign43890_e59102_d_n5, assign43890_e59102_d_n6, assign43890_e59102_d_n7, assign43890_e59102_d_n8, assign43890_e59102_d_n9, assign43890_e59102_d_n10, assign43890_e59102_d_n11, assign43890_e59102_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43890_e59100: f64 = (locals.var_t5 * locals.var_eeff);
        (assign43890_e59100, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn11 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn11)), ((locals.var_t5_dn14 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign43890_e59102;
        locals.var_t8_dn0 = assign43890_e59102_d_n0;
        locals.var_t8_dn2 = assign43890_e59102_d_n2;
        locals.var_t8_dn4 = assign43890_e59102_d_n4;
        locals.var_t8_dn5 = assign43890_e59102_d_n5;
        locals.var_t8_dn6 = assign43890_e59102_d_n6;
        locals.var_t8_dn7 = assign43890_e59102_d_n7;
        locals.var_t8_dn8 = assign43890_e59102_d_n8;
        locals.var_t8_dn9 = assign43890_e59102_d_n9;
        locals.var_t8_dn10 = assign43890_e59102_d_n10;
        locals.var_t8_dn11 = assign43890_e59102_d_n11;
        locals.var_t8_dn14 = assign43890_e59102_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign43900_e59113, assign43900_e59113_d_n0, assign43900_e59113_d_n2, assign43900_e59113_d_n4, assign43900_e59113_d_n5, assign43900_e59113_d_n6, assign43900_e59113_d_n7, assign43900_e59113_d_n8, assign43900_e59113_d_n9, assign43900_e59113_d_n10, assign43900_e59113_d_n11, assign43900_e59113_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43900_e59111: f64 = (locals.var_uc_depmue0 + 1e-25);
        (assign43900_e59111, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign43900_e59113;
        locals.var_t2_dn0 = assign43900_e59113_d_n0;
        locals.var_t2_dn2 = assign43900_e59113_d_n2;
        locals.var_t2_dn4 = assign43900_e59113_d_n4;
        locals.var_t2_dn5 = assign43900_e59113_d_n5;
        locals.var_t2_dn6 = assign43900_e59113_d_n6;
        locals.var_t2_dn7 = assign43900_e59113_d_n7;
        locals.var_t2_dn8 = assign43900_e59113_d_n8;
        locals.var_t2_dn9 = assign43900_e59113_d_n9;
        locals.var_t2_dn10 = assign43900_e59113_d_n10;
        locals.var_t2_dn11 = assign43900_e59113_d_n11;
        locals.var_t2_dn14 = assign43900_e59113_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign43910_e59128, assign43910_e59128_d_n0, assign43910_e59128_d_n2, assign43910_e59128_d_n4, assign43910_e59128_d_n5, assign43910_e59128_d_n6, assign43910_e59128_d_n7, assign43910_e59128_d_n8, assign43910_e59128_d_n9, assign43910_e59128_d_n10, assign43910_e59128_d_n11, assign43910_e59128_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43910_e59122: f64 = (1.0 / locals.var_t2);
        let assign43910_e59125: f64 = (locals.var_t8 / locals.var_uc_depmue2);
        let assign43910_e59126: f64 = (assign43910_e59122 + assign43910_e59125);
        (assign43910_e59126, ((-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn0 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn0)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn2 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn2)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn4 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn4)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn5 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn5)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn6 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn6)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn7 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn7)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn8 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn8)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn9 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn9)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn10 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn10)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn11 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn11)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn14 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn14)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43910_e59128;
        locals.var_t1_dn0 = assign43910_e59128_d_n0;
        locals.var_t1_dn2 = assign43910_e59128_d_n2;
        locals.var_t1_dn4 = assign43910_e59128_d_n4;
        locals.var_t1_dn5 = assign43910_e59128_d_n5;
        locals.var_t1_dn6 = assign43910_e59128_d_n6;
        locals.var_t1_dn7 = assign43910_e59128_d_n7;
        locals.var_t1_dn8 = assign43910_e59128_d_n8;
        locals.var_t1_dn9 = assign43910_e59128_d_n9;
        locals.var_t1_dn10 = assign43910_e59128_d_n10;
        locals.var_t1_dn11 = assign43910_e59128_d_n11;
        locals.var_t1_dn14 = assign43910_e59128_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign43920_e59139, assign43920_e59139_d_n0, assign43920_e59139_d_n2, assign43920_e59139_d_n4, assign43920_e59139_d_n5, assign43920_e59139_d_n6, assign43920_e59139_d_n7, assign43920_e59139_d_n8, assign43920_e59139_d_n9, assign43920_e59139_d_n10, assign43920_e59139_d_n11, assign43920_e59139_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43920_e59137: f64 = (1.0 / locals.var_t1);
        (assign43920_e59137, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign43920_e59139;
        locals.var_muun_dn0 = assign43920_e59139_d_n0;
        locals.var_muun_dn2 = assign43920_e59139_d_n2;
        locals.var_muun_dn4 = assign43920_e59139_d_n4;
        locals.var_muun_dn5 = assign43920_e59139_d_n5;
        locals.var_muun_dn6 = assign43920_e59139_d_n6;
        locals.var_muun_dn7 = assign43920_e59139_d_n7;
        locals.var_muun_dn8 = assign43920_e59139_d_n8;
        locals.var_muun_dn9 = assign43920_e59139_d_n9;
        locals.var_muun_dn10 = assign43920_e59139_d_n10;
        locals.var_muun_dn11 = assign43920_e59139_d_n11;
        locals.var_muun_dn14 = assign43920_e59139_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign43930_e59150, assign43930_e59150_d_n0, assign43930_e59150_d_n2, assign43930_e59150_d_n4, assign43930_e59150_d_n5, assign43930_e59150_d_n6, assign43930_e59150_d_n7, assign43930_e59150_d_n8, assign43930_e59150_d_n9, assign43930_e59150_d_n10, assign43930_e59150_d_n11, assign43930_e59150_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43930_e59148: f64 = (locals.var_muun / 10000.0);
        (assign43930_e59148, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign43930_e59150;
        locals.var_muun_dn0 = assign43930_e59150_d_n0;
        locals.var_muun_dn2 = assign43930_e59150_d_n2;
        locals.var_muun_dn4 = assign43930_e59150_d_n4;
        locals.var_muun_dn5 = assign43930_e59150_d_n5;
        locals.var_muun_dn6 = assign43930_e59150_d_n6;
        locals.var_muun_dn7 = assign43930_e59150_d_n7;
        locals.var_muun_dn8 = assign43930_e59150_d_n8;
        locals.var_muun_dn9 = assign43930_e59150_d_n9;
        locals.var_muun_dn10 = assign43930_e59150_d_n10;
        locals.var_muun_dn11 = assign43930_e59150_d_n11;
        locals.var_muun_dn14 = assign43930_e59150_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign43940_e59161, assign43940_e59161_d_n0, assign43940_e59161_d_n2, assign43940_e59161_d_n4, assign43940_e59161_d_n5, assign43940_e59161_d_n6, assign43940_e59161_d_n7, assign43940_e59161_d_n8, assign43940_e59161_d_n9, assign43940_e59161_d_n10, assign43940_e59161_d_n11, assign43940_e59161_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43940_e59159: f64 = (locals.var_vds_res / locals.var_lch);
        (assign43940_e59159, (((locals.var_vds_res_dn0 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn2 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn4 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn5 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn6 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn7 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn8 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn9 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn10 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn11 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn14 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_edri__blk887, locals.var_edri__blk887_dn0, locals.var_edri__blk887_dn2, locals.var_edri__blk887_dn4, locals.var_edri__blk887_dn5, locals.var_edri__blk887_dn6, locals.var_edri__blk887_dn7, locals.var_edri__blk887_dn8, locals.var_edri__blk887_dn9, locals.var_edri__blk887_dn10, locals.var_edri__blk887_dn11, locals.var_edri__blk887_dn14,)
    }
};
        locals.var_edri__blk887 = assign43940_e59161;
        locals.var_edri__blk887_dn0 = assign43940_e59161_d_n0;
        locals.var_edri__blk887_dn2 = assign43940_e59161_d_n2;
        locals.var_edri__blk887_dn4 = assign43940_e59161_d_n4;
        locals.var_edri__blk887_dn5 = assign43940_e59161_d_n5;
        locals.var_edri__blk887_dn6 = assign43940_e59161_d_n6;
        locals.var_edri__blk887_dn7 = assign43940_e59161_d_n7;
        locals.var_edri__blk887_dn8 = assign43940_e59161_d_n8;
        locals.var_edri__blk887_dn9 = assign43940_e59161_d_n9;
        locals.var_edri__blk887_dn10 = assign43940_e59161_d_n10;
        locals.var_edri__blk887_dn11 = assign43940_e59161_d_n11;
        locals.var_edri__blk887_dn14 = assign43940_e59161_d_n14;
        locals.var_edri__blk887_rv = 0.0;

        let (assign43950_e59172, assign43950_e59172_d_n0, assign43950_e59172_d_n2, assign43950_e59172_d_n4, assign43950_e59172_d_n5, assign43950_e59172_d_n6, assign43950_e59172_d_n7, assign43950_e59172_d_n8, assign43950_e59172_d_n9, assign43950_e59172_d_n10, assign43950_e59172_d_n11, assign43950_e59172_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43950_e59170: f64 = (locals.var_vds_res).powf(2.0);
        (assign43950_e59170, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn0)) } } else { (assign43950_e59170 * (2.0 * (locals.var_vds_res_dn0 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn2)) } } else { (assign43950_e59170 * (2.0 * (locals.var_vds_res_dn2 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn4)) } } else { (assign43950_e59170 * (2.0 * (locals.var_vds_res_dn4 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn5)) } } else { (assign43950_e59170 * (2.0 * (locals.var_vds_res_dn5 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn6)) } } else { (assign43950_e59170 * (2.0 * (locals.var_vds_res_dn6 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn7)) } } else { (assign43950_e59170 * (2.0 * (locals.var_vds_res_dn7 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn8)) } } else { (assign43950_e59170 * (2.0 * (locals.var_vds_res_dn8 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn9)) } } else { (assign43950_e59170 * (2.0 * (locals.var_vds_res_dn9 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn10)) } } else { (assign43950_e59170 * (2.0 * (locals.var_vds_res_dn10 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn11)) } } else { (assign43950_e59170 * (2.0 * (locals.var_vds_res_dn11 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn14)) } } else { (assign43950_e59170 * (2.0 * (locals.var_vds_res_dn14 / locals.var_vds_res))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign43950_e59172;
        locals.var_tmf1_dn0 = assign43950_e59172_d_n0;
        locals.var_tmf1_dn2 = assign43950_e59172_d_n2;
        locals.var_tmf1_dn4 = assign43950_e59172_d_n4;
        locals.var_tmf1_dn5 = assign43950_e59172_d_n5;
        locals.var_tmf1_dn6 = assign43950_e59172_d_n6;
        locals.var_tmf1_dn7 = assign43950_e59172_d_n7;
        locals.var_tmf1_dn8 = assign43950_e59172_d_n8;
        locals.var_tmf1_dn9 = assign43950_e59172_d_n9;
        locals.var_tmf1_dn10 = assign43950_e59172_d_n10;
        locals.var_tmf1_dn11 = assign43950_e59172_d_n11;
        locals.var_tmf1_dn14 = assign43950_e59172_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign43960_e59183, assign43960_e59183_d_n0, assign43960_e59183_d_n2, assign43960_e59183_d_n4, assign43960_e59183_d_n5, assign43960_e59183_d_n6, assign43960_e59183_d_n7, assign43960_e59183_d_n8, assign43960_e59183_d_n9, assign43960_e59183_d_n10, assign43960_e59183_d_n11, assign43960_e59183_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43960_e59181: f64 = (0.1_f64).powf(2.0);
        (assign43960_e59181, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign43960_e59183;
        locals.var_tmf2_dn0 = assign43960_e59183_d_n0;
        locals.var_tmf2_dn2 = assign43960_e59183_d_n2;
        locals.var_tmf2_dn4 = assign43960_e59183_d_n4;
        locals.var_tmf2_dn5 = assign43960_e59183_d_n5;
        locals.var_tmf2_dn6 = assign43960_e59183_d_n6;
        locals.var_tmf2_dn7 = assign43960_e59183_d_n7;
        locals.var_tmf2_dn8 = assign43960_e59183_d_n8;
        locals.var_tmf2_dn9 = assign43960_e59183_d_n9;
        locals.var_tmf2_dn10 = assign43960_e59183_d_n10;
        locals.var_tmf2_dn11 = assign43960_e59183_d_n11;
        locals.var_tmf2_dn14 = assign43960_e59183_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign43970_e59204, assign43970_e59204_d_n0, assign43970_e59204_d_n2, assign43970_e59204_d_n4, assign43970_e59204_d_n5, assign43970_e59204_d_n6, assign43970_e59204_d_n7, assign43970_e59204_d_n8, assign43970_e59204_d_n9, assign43970_e59204_d_n10, assign43970_e59204_d_n11, assign43970_e59204_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43970_e59192: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign43970_e59195: f64 = (1.0 / 2.0);
        let assign43970_e59196: f64 = (assign43970_e59192).powf(assign43970_e59195);
        let assign43970_e59200: f64 = (1.0 / 2.0);
        let assign43970_e59201: f64 = (locals.var_tmf2).powf(assign43970_e59200);
        let assign43970_e59202: f64 = (assign43970_e59196 - assign43970_e59201);
        (assign43970_e59202, (if 0.0 == 0.0 && ((assign43970_e59195) as f64).is_finite() && ((assign43970_e59195) as f64).fract() == 0.0 { if assign43970_e59195 == 0.0 { 0.0 } else { (assign43970_e59195 * ((assign43970_e59192).powf(assign43970_e59195 - 1.0) * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))) } } else { (assign43970_e59196 * (assign43970_e59195 * ((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) / assign43970_e59192))) } - if 0.0 == 0.0 && ((assign43970_e59200) as f64).is_finite() && ((assign43970_e59200) as f64).fract() == 0.0 { if assign43970_e59200 == 0.0 { 0.0 } else { (assign43970_e59200 * ((locals.var_tmf2).powf(assign43970_e59200 - 1.0) * locals.var_tmf2_dn0)) } } else { (assign43970_e59201 * (assign43970_e59200 * (locals.var_tmf2_dn0 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43970_e59195) as f64).is_finite() && ((assign43970_e59195) as f64).fract() == 0.0 { if assign43970_e59195 == 0.0 { 0.0 } else { (assign43970_e59195 * ((assign43970_e59192).powf(assign43970_e59195 - 1.0) * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))) } } else { (assign43970_e59196 * (assign43970_e59195 * ((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) / assign43970_e59192))) } - if 0.0 == 0.0 && ((assign43970_e59200) as f64).is_finite() && ((assign43970_e59200) as f64).fract() == 0.0 { if assign43970_e59200 == 0.0 { 0.0 } else { (assign43970_e59200 * ((locals.var_tmf2).powf(assign43970_e59200 - 1.0) * locals.var_tmf2_dn2)) } } else { (assign43970_e59201 * (assign43970_e59200 * (locals.var_tmf2_dn2 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43970_e59195) as f64).is_finite() && ((assign43970_e59195) as f64).fract() == 0.0 { if assign43970_e59195 == 0.0 { 0.0 } else { (assign43970_e59195 * ((assign43970_e59192).powf(assign43970_e59195 - 1.0) * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))) } } else { (assign43970_e59196 * (assign43970_e59195 * ((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) / assign43970_e59192))) } - if 0.0 == 0.0 && ((assign43970_e59200) as f64).is_finite() && ((assign43970_e59200) as f64).fract() == 0.0 { if assign43970_e59200 == 0.0 { 0.0 } else { (assign43970_e59200 * ((locals.var_tmf2).powf(assign43970_e59200 - 1.0) * locals.var_tmf2_dn4)) } } else { (assign43970_e59201 * (assign43970_e59200 * (locals.var_tmf2_dn4 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43970_e59195) as f64).is_finite() && ((assign43970_e59195) as f64).fract() == 0.0 { if assign43970_e59195 == 0.0 { 0.0 } else { (assign43970_e59195 * ((assign43970_e59192).powf(assign43970_e59195 - 1.0) * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))) } } else { (assign43970_e59196 * (assign43970_e59195 * ((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) / assign43970_e59192))) } - if 0.0 == 0.0 && ((assign43970_e59200) as f64).is_finite() && ((assign43970_e59200) as f64).fract() == 0.0 { if assign43970_e59200 == 0.0 { 0.0 } else { (assign43970_e59200 * ((locals.var_tmf2).powf(assign43970_e59200 - 1.0) * locals.var_tmf2_dn5)) } } else { (assign43970_e59201 * (assign43970_e59200 * (locals.var_tmf2_dn5 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43970_e59195) as f64).is_finite() && ((assign43970_e59195) as f64).fract() == 0.0 { if assign43970_e59195 == 0.0 { 0.0 } else { (assign43970_e59195 * ((assign43970_e59192).powf(assign43970_e59195 - 1.0) * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))) } } else { (assign43970_e59196 * (assign43970_e59195 * ((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) / assign43970_e59192))) } - if 0.0 == 0.0 && ((assign43970_e59200) as f64).is_finite() && ((assign43970_e59200) as f64).fract() == 0.0 { if assign43970_e59200 == 0.0 { 0.0 } else { (assign43970_e59200 * ((locals.var_tmf2).powf(assign43970_e59200 - 1.0) * locals.var_tmf2_dn6)) } } else { (assign43970_e59201 * (assign43970_e59200 * (locals.var_tmf2_dn6 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43970_e59195) as f64).is_finite() && ((assign43970_e59195) as f64).fract() == 0.0 { if assign43970_e59195 == 0.0 { 0.0 } else { (assign43970_e59195 * ((assign43970_e59192).powf(assign43970_e59195 - 1.0) * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))) } } else { (assign43970_e59196 * (assign43970_e59195 * ((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) / assign43970_e59192))) } - if 0.0 == 0.0 && ((assign43970_e59200) as f64).is_finite() && ((assign43970_e59200) as f64).fract() == 0.0 { if assign43970_e59200 == 0.0 { 0.0 } else { (assign43970_e59200 * ((locals.var_tmf2).powf(assign43970_e59200 - 1.0) * locals.var_tmf2_dn7)) } } else { (assign43970_e59201 * (assign43970_e59200 * (locals.var_tmf2_dn7 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43970_e59195) as f64).is_finite() && ((assign43970_e59195) as f64).fract() == 0.0 { if assign43970_e59195 == 0.0 { 0.0 } else { (assign43970_e59195 * ((assign43970_e59192).powf(assign43970_e59195 - 1.0) * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))) } } else { (assign43970_e59196 * (assign43970_e59195 * ((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) / assign43970_e59192))) } - if 0.0 == 0.0 && ((assign43970_e59200) as f64).is_finite() && ((assign43970_e59200) as f64).fract() == 0.0 { if assign43970_e59200 == 0.0 { 0.0 } else { (assign43970_e59200 * ((locals.var_tmf2).powf(assign43970_e59200 - 1.0) * locals.var_tmf2_dn8)) } } else { (assign43970_e59201 * (assign43970_e59200 * (locals.var_tmf2_dn8 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43970_e59195) as f64).is_finite() && ((assign43970_e59195) as f64).fract() == 0.0 { if assign43970_e59195 == 0.0 { 0.0 } else { (assign43970_e59195 * ((assign43970_e59192).powf(assign43970_e59195 - 1.0) * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))) } } else { (assign43970_e59196 * (assign43970_e59195 * ((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) / assign43970_e59192))) } - if 0.0 == 0.0 && ((assign43970_e59200) as f64).is_finite() && ((assign43970_e59200) as f64).fract() == 0.0 { if assign43970_e59200 == 0.0 { 0.0 } else { (assign43970_e59200 * ((locals.var_tmf2).powf(assign43970_e59200 - 1.0) * locals.var_tmf2_dn9)) } } else { (assign43970_e59201 * (assign43970_e59200 * (locals.var_tmf2_dn9 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43970_e59195) as f64).is_finite() && ((assign43970_e59195) as f64).fract() == 0.0 { if assign43970_e59195 == 0.0 { 0.0 } else { (assign43970_e59195 * ((assign43970_e59192).powf(assign43970_e59195 - 1.0) * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))) } } else { (assign43970_e59196 * (assign43970_e59195 * ((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) / assign43970_e59192))) } - if 0.0 == 0.0 && ((assign43970_e59200) as f64).is_finite() && ((assign43970_e59200) as f64).fract() == 0.0 { if assign43970_e59200 == 0.0 { 0.0 } else { (assign43970_e59200 * ((locals.var_tmf2).powf(assign43970_e59200 - 1.0) * locals.var_tmf2_dn10)) } } else { (assign43970_e59201 * (assign43970_e59200 * (locals.var_tmf2_dn10 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43970_e59195) as f64).is_finite() && ((assign43970_e59195) as f64).fract() == 0.0 { if assign43970_e59195 == 0.0 { 0.0 } else { (assign43970_e59195 * ((assign43970_e59192).powf(assign43970_e59195 - 1.0) * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))) } } else { (assign43970_e59196 * (assign43970_e59195 * ((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) / assign43970_e59192))) } - if 0.0 == 0.0 && ((assign43970_e59200) as f64).is_finite() && ((assign43970_e59200) as f64).fract() == 0.0 { if assign43970_e59200 == 0.0 { 0.0 } else { (assign43970_e59200 * ((locals.var_tmf2).powf(assign43970_e59200 - 1.0) * locals.var_tmf2_dn11)) } } else { (assign43970_e59201 * (assign43970_e59200 * (locals.var_tmf2_dn11 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43970_e59195) as f64).is_finite() && ((assign43970_e59195) as f64).fract() == 0.0 { if assign43970_e59195 == 0.0 { 0.0 } else { (assign43970_e59195 * ((assign43970_e59192).powf(assign43970_e59195 - 1.0) * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))) } } else { (assign43970_e59196 * (assign43970_e59195 * ((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) / assign43970_e59192))) } - if 0.0 == 0.0 && ((assign43970_e59200) as f64).is_finite() && ((assign43970_e59200) as f64).fract() == 0.0 { if assign43970_e59200 == 0.0 { 0.0 } else { (assign43970_e59200 * ((locals.var_tmf2).powf(assign43970_e59200 - 1.0) * locals.var_tmf2_dn14)) } } else { (assign43970_e59201 * (assign43970_e59200 * (locals.var_tmf2_dn14 / locals.var_tmf2))) }),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43970_e59204;
        locals.var_t1_dn0 = assign43970_e59204_d_n0;
        locals.var_t1_dn2 = assign43970_e59204_d_n2;
        locals.var_t1_dn4 = assign43970_e59204_d_n4;
        locals.var_t1_dn5 = assign43970_e59204_d_n5;
        locals.var_t1_dn6 = assign43970_e59204_d_n6;
        locals.var_t1_dn7 = assign43970_e59204_d_n7;
        locals.var_t1_dn8 = assign43970_e59204_d_n8;
        locals.var_t1_dn9 = assign43970_e59204_d_n9;
        locals.var_t1_dn10 = assign43970_e59204_d_n10;
        locals.var_t1_dn11 = assign43970_e59204_d_n11;
        locals.var_t1_dn14 = assign43970_e59204_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign43980_e59215, assign43980_e59215_d_n0, assign43980_e59215_d_n2, assign43980_e59215_d_n4, assign43980_e59215_d_n5, assign43980_e59215_d_n6, assign43980_e59215_d_n7, assign43980_e59215_d_n8, assign43980_e59215_d_n9, assign43980_e59215_d_n10, assign43980_e59215_d_n11, assign43980_e59215_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43980_e59213: f64 = (locals.var_t1 / locals.var_lch);
        (assign43980_e59213, (((locals.var_t1_dn0 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn2 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn4 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn5 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn6 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn7 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn8 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn9 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn10 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn11 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn14 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43980_e59215;
        locals.var_t1_dn0 = assign43980_e59215_d_n0;
        locals.var_t1_dn2 = assign43980_e59215_d_n2;
        locals.var_t1_dn4 = assign43980_e59215_d_n4;
        locals.var_t1_dn5 = assign43980_e59215_d_n5;
        locals.var_t1_dn6 = assign43980_e59215_d_n6;
        locals.var_t1_dn7 = assign43980_e59215_d_n7;
        locals.var_t1_dn8 = assign43980_e59215_d_n8;
        locals.var_t1_dn9 = assign43980_e59215_d_n9;
        locals.var_t1_dn10 = assign43980_e59215_d_n10;
        locals.var_t1_dn11 = assign43980_e59215_d_n11;
        locals.var_t1_dn14 = assign43980_e59215_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign43990_e59228, assign43990_e59228_d_n0, assign43990_e59228_d_n2, assign43990_e59228_d_n4, assign43990_e59228_d_n5, assign43990_e59228_d_n6, assign43990_e59228_d_n7, assign43990_e59228_d_n8, assign43990_e59228_d_n9, assign43990_e59228_d_n10, assign43990_e59228_d_n11, assign43990_e59228_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign43990_e59224: f64 = (locals.var_muun * locals.var_t1);
        let assign43990_e59226: f64 = (assign43990_e59224 / locals.var_uc_depvmax);
        (assign43990_e59226, (((((locals.var_muun_dn0 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn0)) * locals.var_uc_depvmax) - (assign43990_e59224 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn2 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn2)) * locals.var_uc_depvmax) - (assign43990_e59224 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn4 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn4)) * locals.var_uc_depvmax) - (assign43990_e59224 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn5 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn5)) * locals.var_uc_depvmax) - (assign43990_e59224 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn6 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn6)) * locals.var_uc_depvmax) - (assign43990_e59224 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn7 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn7)) * locals.var_uc_depvmax) - (assign43990_e59224 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn8 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn8)) * locals.var_uc_depvmax) - (assign43990_e59224 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn9 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn9)) * locals.var_uc_depvmax) - (assign43990_e59224 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn10 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn10)) * locals.var_uc_depvmax) - (assign43990_e59224 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn11 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn11)) * locals.var_uc_depvmax) - (assign43990_e59224 * locals.var_uc_depvmax_dn11)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn14 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn14)) * locals.var_uc_depvmax) - (assign43990_e59224 * locals.var_uc_depvmax_dn14)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43990_e59228;
        locals.var_t1_dn0 = assign43990_e59228_d_n0;
        locals.var_t1_dn2 = assign43990_e59228_d_n2;
        locals.var_t1_dn4 = assign43990_e59228_d_n4;
        locals.var_t1_dn5 = assign43990_e59228_d_n5;
        locals.var_t1_dn6 = assign43990_e59228_d_n6;
        locals.var_t1_dn7 = assign43990_e59228_d_n7;
        locals.var_t1_dn8 = assign43990_e59228_d_n8;
        locals.var_t1_dn9 = assign43990_e59228_d_n9;
        locals.var_t1_dn10 = assign43990_e59228_d_n10;
        locals.var_t1_dn11 = assign43990_e59228_d_n11;
        locals.var_t1_dn14 = assign43990_e59228_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign44000_e59244, assign44000_e59244_d_n0, assign44000_e59244_d_n2, assign44000_e59244_d_n4, assign44000_e59244_d_n5, assign44000_e59244_d_n6, assign44000_e59244_d_n7, assign44000_e59244_d_n8, assign44000_e59244_d_n9, assign44000_e59244_d_n10, assign44000_e59244_d_n11, assign44000_e59244_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let (assign44000_e59242, assign44000_e59242_d_n0, assign44000_e59242_d_n2, assign44000_e59242_d_n4, assign44000_e59242_d_n5, assign44000_e59242_d_n6, assign44000_e59242_d_n7, assign44000_e59242_d_n8, assign44000_e59242_d_n9, assign44000_e59242_d_n10, assign44000_e59242_d_n11, assign44000_e59242_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign44000_e59241: f64 = (locals.var_t1).powf(p.p378);
                (assign44000_e59241, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn0)) } } else { (assign44000_e59241 * (p.p378 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn2)) } } else { (assign44000_e59241 * (p.p378 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn4)) } } else { (assign44000_e59241 * (p.p378 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn5)) } } else { (assign44000_e59241 * (p.p378 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn6)) } } else { (assign44000_e59241 * (p.p378 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn7)) } } else { (assign44000_e59241 * (p.p378 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn8)) } } else { (assign44000_e59241 * (p.p378 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn9)) } } else { (assign44000_e59241 * (p.p378 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn10)) } } else { (assign44000_e59241 * (p.p378 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn11)) } } else { (assign44000_e59241 * (p.p378 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn14)) } } else { (assign44000_e59241 * (p.p378 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign44000_e59242, assign44000_e59242_d_n0, assign44000_e59242_d_n2, assign44000_e59242_d_n4, assign44000_e59242_d_n5, assign44000_e59242_d_n6, assign44000_e59242_d_n7, assign44000_e59242_d_n8, assign44000_e59242_d_n9, assign44000_e59242_d_n10, assign44000_e59242_d_n11, assign44000_e59242_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign44000_e59244;
        locals.var_t2_dn0 = assign44000_e59244_d_n0;
        locals.var_t2_dn2 = assign44000_e59244_d_n2;
        locals.var_t2_dn4 = assign44000_e59244_d_n4;
        locals.var_t2_dn5 = assign44000_e59244_d_n5;
        locals.var_t2_dn6 = assign44000_e59244_d_n6;
        locals.var_t2_dn7 = assign44000_e59244_d_n7;
        locals.var_t2_dn8 = assign44000_e59244_d_n8;
        locals.var_t2_dn9 = assign44000_e59244_d_n9;
        locals.var_t2_dn10 = assign44000_e59244_d_n10;
        locals.var_t2_dn11 = assign44000_e59244_d_n11;
        locals.var_t2_dn14 = assign44000_e59244_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign44010_e59255, assign44010_e59255_d_n0, assign44010_e59255_d_n2, assign44010_e59255_d_n4, assign44010_e59255_d_n5, assign44010_e59255_d_n6, assign44010_e59255_d_n7, assign44010_e59255_d_n8, assign44010_e59255_d_n9, assign44010_e59255_d_n10, assign44010_e59255_d_n11, assign44010_e59255_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign44010_e59253: f64 = (1.0 + locals.var_t2);
        (assign44010_e59253, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign44010_e59255;
        locals.var_t3_dn0 = assign44010_e59255_d_n0;
        locals.var_t3_dn2 = assign44010_e59255_d_n2;
        locals.var_t3_dn4 = assign44010_e59255_d_n4;
        locals.var_t3_dn5 = assign44010_e59255_d_n5;
        locals.var_t3_dn6 = assign44010_e59255_d_n6;
        locals.var_t3_dn7 = assign44010_e59255_d_n7;
        locals.var_t3_dn8 = assign44010_e59255_d_n8;
        locals.var_t3_dn9 = assign44010_e59255_d_n9;
        locals.var_t3_dn10 = assign44010_e59255_d_n10;
        locals.var_t3_dn11 = assign44010_e59255_d_n11;
        locals.var_t3_dn14 = assign44010_e59255_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign44020_e59273, assign44020_e59273_d_n0, assign44020_e59273_d_n2, assign44020_e59273_d_n4, assign44020_e59273_d_n5, assign44020_e59273_d_n6, assign44020_e59273_d_n7, assign44020_e59273_d_n8, assign44020_e59273_d_n9, assign44020_e59273_d_n10, assign44020_e59273_d_n11, assign44020_e59273_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let (assign44020_e59271, assign44020_e59271_d_n0, assign44020_e59271_d_n2, assign44020_e59271_d_n4, assign44020_e59271_d_n5, assign44020_e59271_d_n6, assign44020_e59271_d_n7, assign44020_e59271_d_n8, assign44020_e59271_d_n9, assign44020_e59271_d_n10, assign44020_e59271_d_n11, assign44020_e59271_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign44020_e59269: f64 = (1.0 / p.p378);
                let assign44020_e59270: f64 = (locals.var_t3).powf(assign44020_e59269);
                (assign44020_e59270, if 0.0 == 0.0 && ((assign44020_e59269) as f64).is_finite() && ((assign44020_e59269) as f64).fract() == 0.0 { if assign44020_e59269 == 0.0 { 0.0 } else { (assign44020_e59269 * ((locals.var_t3).powf(assign44020_e59269 - 1.0) * locals.var_t3_dn0)) } } else { (assign44020_e59270 * (assign44020_e59269 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44020_e59269) as f64).is_finite() && ((assign44020_e59269) as f64).fract() == 0.0 { if assign44020_e59269 == 0.0 { 0.0 } else { (assign44020_e59269 * ((locals.var_t3).powf(assign44020_e59269 - 1.0) * locals.var_t3_dn2)) } } else { (assign44020_e59270 * (assign44020_e59269 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44020_e59269) as f64).is_finite() && ((assign44020_e59269) as f64).fract() == 0.0 { if assign44020_e59269 == 0.0 { 0.0 } else { (assign44020_e59269 * ((locals.var_t3).powf(assign44020_e59269 - 1.0) * locals.var_t3_dn4)) } } else { (assign44020_e59270 * (assign44020_e59269 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44020_e59269) as f64).is_finite() && ((assign44020_e59269) as f64).fract() == 0.0 { if assign44020_e59269 == 0.0 { 0.0 } else { (assign44020_e59269 * ((locals.var_t3).powf(assign44020_e59269 - 1.0) * locals.var_t3_dn5)) } } else { (assign44020_e59270 * (assign44020_e59269 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44020_e59269) as f64).is_finite() && ((assign44020_e59269) as f64).fract() == 0.0 { if assign44020_e59269 == 0.0 { 0.0 } else { (assign44020_e59269 * ((locals.var_t3).powf(assign44020_e59269 - 1.0) * locals.var_t3_dn6)) } } else { (assign44020_e59270 * (assign44020_e59269 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44020_e59269) as f64).is_finite() && ((assign44020_e59269) as f64).fract() == 0.0 { if assign44020_e59269 == 0.0 { 0.0 } else { (assign44020_e59269 * ((locals.var_t3).powf(assign44020_e59269 - 1.0) * locals.var_t3_dn7)) } } else { (assign44020_e59270 * (assign44020_e59269 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44020_e59269) as f64).is_finite() && ((assign44020_e59269) as f64).fract() == 0.0 { if assign44020_e59269 == 0.0 { 0.0 } else { (assign44020_e59269 * ((locals.var_t3).powf(assign44020_e59269 - 1.0) * locals.var_t3_dn8)) } } else { (assign44020_e59270 * (assign44020_e59269 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44020_e59269) as f64).is_finite() && ((assign44020_e59269) as f64).fract() == 0.0 { if assign44020_e59269 == 0.0 { 0.0 } else { (assign44020_e59269 * ((locals.var_t3).powf(assign44020_e59269 - 1.0) * locals.var_t3_dn9)) } } else { (assign44020_e59270 * (assign44020_e59269 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44020_e59269) as f64).is_finite() && ((assign44020_e59269) as f64).fract() == 0.0 { if assign44020_e59269 == 0.0 { 0.0 } else { (assign44020_e59269 * ((locals.var_t3).powf(assign44020_e59269 - 1.0) * locals.var_t3_dn10)) } } else { (assign44020_e59270 * (assign44020_e59269 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44020_e59269) as f64).is_finite() && ((assign44020_e59269) as f64).fract() == 0.0 { if assign44020_e59269 == 0.0 { 0.0 } else { (assign44020_e59269 * ((locals.var_t3).powf(assign44020_e59269 - 1.0) * locals.var_t3_dn11)) } } else { (assign44020_e59270 * (assign44020_e59269 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44020_e59269) as f64).is_finite() && ((assign44020_e59269) as f64).fract() == 0.0 { if assign44020_e59269 == 0.0 { 0.0 } else { (assign44020_e59269 * ((locals.var_t3).powf(assign44020_e59269 - 1.0) * locals.var_t3_dn14)) } } else { (assign44020_e59270 * (assign44020_e59269 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign44020_e59271, assign44020_e59271_d_n0, assign44020_e59271_d_n2, assign44020_e59271_d_n4, assign44020_e59271_d_n5, assign44020_e59271_d_n6, assign44020_e59271_d_n7, assign44020_e59271_d_n8, assign44020_e59271_d_n9, assign44020_e59271_d_n10, assign44020_e59271_d_n11, assign44020_e59271_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign44020_e59273;
        locals.var_t4_dn0 = assign44020_e59273_d_n0;
        locals.var_t4_dn2 = assign44020_e59273_d_n2;
        locals.var_t4_dn4 = assign44020_e59273_d_n4;
        locals.var_t4_dn5 = assign44020_e59273_d_n5;
        locals.var_t4_dn6 = assign44020_e59273_d_n6;
        locals.var_t4_dn7 = assign44020_e59273_d_n7;
        locals.var_t4_dn8 = assign44020_e59273_d_n8;
        locals.var_t4_dn9 = assign44020_e59273_d_n9;
        locals.var_t4_dn10 = assign44020_e59273_d_n10;
        locals.var_t4_dn11 = assign44020_e59273_d_n11;
        locals.var_t4_dn14 = assign44020_e59273_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign44030_e59284, assign44030_e59284_d_n0, assign44030_e59284_d_n2, assign44030_e59284_d_n4, assign44030_e59284_d_n5, assign44030_e59284_d_n6, assign44030_e59284_d_n7, assign44030_e59284_d_n8, assign44030_e59284_d_n9, assign44030_e59284_d_n10, assign44030_e59284_d_n11, assign44030_e59284_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign44030_e59282: f64 = (locals.var_muun / locals.var_t4);
        (assign44030_e59282, (((locals.var_muun_dn0 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn2 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn4 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn5 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn6 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn7 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn8 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn9 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn10 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn11 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn14 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_mu_res, locals.var_mu_res_dn0, locals.var_mu_res_dn2, locals.var_mu_res_dn4, locals.var_mu_res_dn5, locals.var_mu_res_dn6, locals.var_mu_res_dn7, locals.var_mu_res_dn8, locals.var_mu_res_dn9, locals.var_mu_res_dn10, locals.var_mu_res_dn11, locals.var_mu_res_dn14,)
    }
};
        locals.var_mu_res = assign44030_e59284;
        locals.var_mu_res_dn0 = assign44030_e59284_d_n0;
        locals.var_mu_res_dn2 = assign44030_e59284_d_n2;
        locals.var_mu_res_dn4 = assign44030_e59284_d_n4;
        locals.var_mu_res_dn5 = assign44030_e59284_d_n5;
        locals.var_mu_res_dn6 = assign44030_e59284_d_n6;
        locals.var_mu_res_dn7 = assign44030_e59284_d_n7;
        locals.var_mu_res_dn8 = assign44030_e59284_d_n8;
        locals.var_mu_res_dn9 = assign44030_e59284_d_n9;
        locals.var_mu_res_dn10 = assign44030_e59284_d_n10;
        locals.var_mu_res_dn11 = assign44030_e59284_d_n11;
        locals.var_mu_res_dn14 = assign44030_e59284_d_n14;
        locals.var_mu_res_rv = 0.0;

        let (assign44040_e59298, assign44040_e59298_d_n0, assign44040_e59298_d_n2, assign44040_e59298_d_n4, assign44040_e59298_d_n5, assign44040_e59298_d_n6, assign44040_e59298_d_n7, assign44040_e59298_d_n8, assign44040_e59298_d_n9, assign44040_e59298_d_n10, assign44040_e59298_d_n11, assign44040_e59298_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign44040_e59292: f64 = (-locals.var_w_res);
        let assign44040_e59294: f64 = (assign44040_e59292 * 1.6021918e-19);
        let assign44040_e59296: f64 = (assign44040_e59294 * locals.var_uc_ndepm);
        (assign44040_e59296, ((((-locals.var_w_res_dn0) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44040_e59294 * locals.var_uc_ndepm_dn0)), ((((-locals.var_w_res_dn2) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44040_e59294 * locals.var_uc_ndepm_dn2)), ((((-locals.var_w_res_dn4) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44040_e59294 * locals.var_uc_ndepm_dn4)), ((((-locals.var_w_res_dn5) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44040_e59294 * locals.var_uc_ndepm_dn5)), ((((-locals.var_w_res_dn6) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44040_e59294 * locals.var_uc_ndepm_dn6)), ((((-locals.var_w_res_dn7) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44040_e59294 * locals.var_uc_ndepm_dn7)), ((((-locals.var_w_res_dn8) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44040_e59294 * locals.var_uc_ndepm_dn8)), ((((-locals.var_w_res_dn9) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44040_e59294 * locals.var_uc_ndepm_dn9)), ((((-locals.var_w_res_dn10) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44040_e59294 * locals.var_uc_ndepm_dn10)), ((((-locals.var_w_res_dn11) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44040_e59294 * locals.var_uc_ndepm_dn11)), ((((-locals.var_w_res_dn14) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44040_e59294 * locals.var_uc_ndepm_dn14)),)
    } else {
        (locals.var_qn_res, locals.var_qn_res_dn0, locals.var_qn_res_dn2, locals.var_qn_res_dn4, locals.var_qn_res_dn5, locals.var_qn_res_dn6, locals.var_qn_res_dn7, locals.var_qn_res_dn8, locals.var_qn_res_dn9, locals.var_qn_res_dn10, locals.var_qn_res_dn11, locals.var_qn_res_dn14,)
    }
};
        locals.var_qn_res = assign44040_e59298;
        locals.var_qn_res_dn0 = assign44040_e59298_d_n0;
        locals.var_qn_res_dn2 = assign44040_e59298_d_n2;
        locals.var_qn_res_dn4 = assign44040_e59298_d_n4;
        locals.var_qn_res_dn5 = assign44040_e59298_d_n5;
        locals.var_qn_res_dn6 = assign44040_e59298_d_n6;
        locals.var_qn_res_dn7 = assign44040_e59298_d_n7;
        locals.var_qn_res_dn8 = assign44040_e59298_d_n8;
        locals.var_qn_res_dn9 = assign44040_e59298_d_n9;
        locals.var_qn_res_dn10 = assign44040_e59298_d_n10;
        locals.var_qn_res_dn11 = assign44040_e59298_d_n11;
        locals.var_qn_res_dn14 = assign44040_e59298_d_n14;
        locals.var_qn_res_rv = 0.0;

        let (assign44050_e59314, assign44050_e59314_d_n0, assign44050_e59314_d_n2, assign44050_e59314_d_n4, assign44050_e59314_d_n5, assign44050_e59314_d_n6, assign44050_e59314_d_n7, assign44050_e59314_d_n8, assign44050_e59314_d_n9, assign44050_e59314_d_n10, assign44050_e59314_d_n11, assign44050_e59314_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign44050_e59307: f64 = (-locals.var_qn_res);
        let assign44050_e59308: f64 = (locals.var_weff_nf * assign44050_e59307);
        let assign44050_e59310: f64 = (assign44050_e59308 * locals.var_mu_res);
        let assign44050_e59312: f64 = (assign44050_e59310 * locals.var_edri__blk887);
        (assign44050_e59312, (((((locals.var_weff_nf * (-locals.var_qn_res_dn0)) * locals.var_mu_res) + (assign44050_e59308 * locals.var_mu_res_dn0)) * locals.var_edri__blk887) + (assign44050_e59310 * locals.var_edri__blk887_dn0)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn2)) * locals.var_mu_res) + (assign44050_e59308 * locals.var_mu_res_dn2)) * locals.var_edri__blk887) + (assign44050_e59310 * locals.var_edri__blk887_dn2)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn4)) * locals.var_mu_res) + (assign44050_e59308 * locals.var_mu_res_dn4)) * locals.var_edri__blk887) + (assign44050_e59310 * locals.var_edri__blk887_dn4)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn5)) * locals.var_mu_res) + (assign44050_e59308 * locals.var_mu_res_dn5)) * locals.var_edri__blk887) + (assign44050_e59310 * locals.var_edri__blk887_dn5)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn6)) * locals.var_mu_res) + (assign44050_e59308 * locals.var_mu_res_dn6)) * locals.var_edri__blk887) + (assign44050_e59310 * locals.var_edri__blk887_dn6)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn7)) * locals.var_mu_res) + (assign44050_e59308 * locals.var_mu_res_dn7)) * locals.var_edri__blk887) + (assign44050_e59310 * locals.var_edri__blk887_dn7)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn8)) * locals.var_mu_res) + (assign44050_e59308 * locals.var_mu_res_dn8)) * locals.var_edri__blk887) + (assign44050_e59310 * locals.var_edri__blk887_dn8)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn9)) * locals.var_mu_res) + (assign44050_e59308 * locals.var_mu_res_dn9)) * locals.var_edri__blk887) + (assign44050_e59310 * locals.var_edri__blk887_dn9)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn10)) * locals.var_mu_res) + (assign44050_e59308 * locals.var_mu_res_dn10)) * locals.var_edri__blk887) + (assign44050_e59310 * locals.var_edri__blk887_dn10)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn11)) * locals.var_mu_res) + (assign44050_e59308 * locals.var_mu_res_dn11)) * locals.var_edri__blk887) + (assign44050_e59310 * locals.var_edri__blk887_dn11)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn14)) * locals.var_mu_res) + (assign44050_e59308 * locals.var_mu_res_dn14)) * locals.var_edri__blk887) + (assign44050_e59310 * locals.var_edri__blk887_dn14)),)
    } else {
        (locals.var_ids_res, locals.var_ids_res_dn0, locals.var_ids_res_dn2, locals.var_ids_res_dn4, locals.var_ids_res_dn5, locals.var_ids_res_dn6, locals.var_ids_res_dn7, locals.var_ids_res_dn8, locals.var_ids_res_dn9, locals.var_ids_res_dn10, locals.var_ids_res_dn11, locals.var_ids_res_dn14,)
    }
};
        locals.var_ids_res = assign44050_e59314;
        locals.var_ids_res_dn0 = assign44050_e59314_d_n0;
        locals.var_ids_res_dn2 = assign44050_e59314_d_n2;
        locals.var_ids_res_dn4 = assign44050_e59314_d_n4;
        locals.var_ids_res_dn5 = assign44050_e59314_d_n5;
        locals.var_ids_res_dn6 = assign44050_e59314_d_n6;
        locals.var_ids_res_dn7 = assign44050_e59314_d_n7;
        locals.var_ids_res_dn8 = assign44050_e59314_d_n8;
        locals.var_ids_res_dn9 = assign44050_e59314_d_n9;
        locals.var_ids_res_dn10 = assign44050_e59314_d_n10;
        locals.var_ids_res_dn11 = assign44050_e59314_d_n11;
        locals.var_ids_res_dn14 = assign44050_e59314_d_n14;
        locals.var_ids_res_rv = 0.0;

        let (assign44060_e59327, assign44060_e59327_d_n0, assign44060_e59327_d_n2, assign44060_e59327_d_n4, assign44060_e59327_d_n5, assign44060_e59327_d_n6, assign44060_e59327_d_n7, assign44060_e59327_d_n8, assign44060_e59327_d_n9, assign44060_e59327_d_n10, assign44060_e59327_d_n11, assign44060_e59327_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign44060_e59323: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign44060_e59325: f64 = (assign44060_e59323 / locals.var_lch);
        (assign44060_e59325, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign44060_e59323 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign44060_e59323 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign44060_e59323 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign44060_e59323 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign44060_e59323 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign44060_e59323 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign44060_e59323 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign44060_e59323 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign44060_e59323 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn11) * locals.var_lch) - (assign44060_e59323 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn14) * locals.var_lch) - (assign44060_e59323 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn11, locals.var_betawl_dn14,)
    }
};
        locals.var_betawl = assign44060_e59327;
        locals.var_betawl_dn0 = assign44060_e59327_d_n0;
        locals.var_betawl_dn2 = assign44060_e59327_d_n2;
        locals.var_betawl_dn4 = assign44060_e59327_d_n4;
        locals.var_betawl_dn5 = assign44060_e59327_d_n5;
        locals.var_betawl_dn6 = assign44060_e59327_d_n6;
        locals.var_betawl_dn7 = assign44060_e59327_d_n7;
        locals.var_betawl_dn8 = assign44060_e59327_d_n8;
        locals.var_betawl_dn9 = assign44060_e59327_d_n9;
        locals.var_betawl_dn10 = assign44060_e59327_d_n10;
        locals.var_betawl_dn11 = assign44060_e59327_d_n11;
        locals.var_betawl_dn14 = assign44060_e59327_d_n14;
        locals.var_betawl_rv = 0.0;

        let (assign44070_e59340, assign44070_e59340_d_n0, assign44070_e59340_d_n2, assign44070_e59340_d_n4, assign44070_e59340_d_n5, assign44070_e59340_d_n6, assign44070_e59340_d_n7, assign44070_e59340_d_n8, assign44070_e59340_d_n9, assign44070_e59340_d_n10, assign44070_e59340_d_n11, assign44070_e59340_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign44070_e59336: f64 = (locals.var_betawl * locals.var_idd);
        let assign44070_e59338: f64 = (assign44070_e59336 * locals.var_mu_acc);
        (assign44070_e59338, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu_acc) + (assign44070_e59336 * locals.var_mu_acc_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu_acc) + (assign44070_e59336 * locals.var_mu_acc_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu_acc) + (assign44070_e59336 * locals.var_mu_acc_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu_acc) + (assign44070_e59336 * locals.var_mu_acc_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu_acc) + (assign44070_e59336 * locals.var_mu_acc_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu_acc) + (assign44070_e59336 * locals.var_mu_acc_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu_acc) + (assign44070_e59336 * locals.var_mu_acc_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu_acc) + (assign44070_e59336 * locals.var_mu_acc_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu_acc) + (assign44070_e59336 * locals.var_mu_acc_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu_acc) + (assign44070_e59336 * locals.var_mu_acc_dn11)), ((((locals.var_betawl_dn14 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn14)) * locals.var_mu_acc) + (assign44070_e59336 * locals.var_mu_acc_dn14)),)
    } else {
        (locals.var_ids_acc, locals.var_ids_acc_dn0, locals.var_ids_acc_dn2, locals.var_ids_acc_dn4, locals.var_ids_acc_dn5, locals.var_ids_acc_dn6, locals.var_ids_acc_dn7, locals.var_ids_acc_dn8, locals.var_ids_acc_dn9, locals.var_ids_acc_dn10, locals.var_ids_acc_dn11, locals.var_ids_acc_dn14,)
    }
};
        locals.var_ids_acc = assign44070_e59340;
        locals.var_ids_acc_dn0 = assign44070_e59340_d_n0;
        locals.var_ids_acc_dn2 = assign44070_e59340_d_n2;
        locals.var_ids_acc_dn4 = assign44070_e59340_d_n4;
        locals.var_ids_acc_dn5 = assign44070_e59340_d_n5;
        locals.var_ids_acc_dn6 = assign44070_e59340_d_n6;
        locals.var_ids_acc_dn7 = assign44070_e59340_d_n7;
        locals.var_ids_acc_dn8 = assign44070_e59340_d_n8;
        locals.var_ids_acc_dn9 = assign44070_e59340_d_n9;
        locals.var_ids_acc_dn10 = assign44070_e59340_d_n10;
        locals.var_ids_acc_dn11 = assign44070_e59340_d_n11;
        locals.var_ids_acc_dn14 = assign44070_e59340_d_n14;
        locals.var_ids_acc_rv = 0.0;

        let (assign44080_e59351, assign44080_e59351_d_n0, assign44080_e59351_d_n2, assign44080_e59351_d_n4, assign44080_e59351_d_n5, assign44080_e59351_d_n6, assign44080_e59351_d_n7, assign44080_e59351_d_n8, assign44080_e59351_d_n9, assign44080_e59351_d_n10, assign44080_e59351_d_n11, assign44080_e59351_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        let assign44080_e59349: f64 = (locals.var_ids_acc + locals.var_ids_res);
        (assign44080_e59349, (locals.var_ids_acc_dn0 + locals.var_ids_res_dn0), (locals.var_ids_acc_dn2 + locals.var_ids_res_dn2), (locals.var_ids_acc_dn4 + locals.var_ids_res_dn4), (locals.var_ids_acc_dn5 + locals.var_ids_res_dn5), (locals.var_ids_acc_dn6 + locals.var_ids_res_dn6), (locals.var_ids_acc_dn7 + locals.var_ids_res_dn7), (locals.var_ids_acc_dn8 + locals.var_ids_res_dn8), (locals.var_ids_acc_dn9 + locals.var_ids_res_dn9), (locals.var_ids_acc_dn10 + locals.var_ids_res_dn10), (locals.var_ids_acc_dn11 + locals.var_ids_res_dn11), (locals.var_ids_acc_dn14 + locals.var_ids_res_dn14),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign44080_e59351;
        locals.var_ids0_dn0 = assign44080_e59351_d_n0;
        locals.var_ids0_dn2 = assign44080_e59351_d_n2;
        locals.var_ids0_dn4 = assign44080_e59351_d_n4;
        locals.var_ids0_dn5 = assign44080_e59351_d_n5;
        locals.var_ids0_dn6 = assign44080_e59351_d_n6;
        locals.var_ids0_dn7 = assign44080_e59351_d_n7;
        locals.var_ids0_dn8 = assign44080_e59351_d_n8;
        locals.var_ids0_dn9 = assign44080_e59351_d_n9;
        locals.var_ids0_dn10 = assign44080_e59351_d_n10;
        locals.var_ids0_dn11 = assign44080_e59351_d_n11;
        locals.var_ids0_dn14 = assign44080_e59351_d_n14;
        locals.var_ids0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_153(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign44090_e59360, assign44090_e59360_d_n0, assign44090_e59360_d_n2, assign44090_e59360_d_n4, assign44090_e59360_d_n5, assign44090_e59360_d_n6, assign44090_e59360_d_n7, assign44090_e59360_d_n8, assign44090_e59360_d_n9, assign44090_e59360_d_n10, assign44090_e59360_d_n11, assign44090_e59360_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign44090_e59360;
        locals.var_vds_dn0 = assign44090_e59360_d_n0;
        locals.var_vds_dn2 = assign44090_e59360_d_n2;
        locals.var_vds_dn4 = assign44090_e59360_d_n4;
        locals.var_vds_dn5 = assign44090_e59360_d_n5;
        locals.var_vds_dn6 = assign44090_e59360_d_n6;
        locals.var_vds_dn7 = assign44090_e59360_d_n7;
        locals.var_vds_dn8 = assign44090_e59360_d_n8;
        locals.var_vds_dn9 = assign44090_e59360_d_n9;
        locals.var_vds_dn10 = assign44090_e59360_d_n10;
        locals.var_vds_dn11 = assign44090_e59360_d_n11;
        locals.var_vds_dn14 = assign44090_e59360_d_n14;
        locals.var_vds_rv = 0.0;

        let assign44100_e59363: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1078 = assign44100_e59363;
        locals.var_guard1078_rv = 0.0;

        let (assign44110_e59378, assign44110_e59378_d_n0, assign44110_e59378_d_n2, assign44110_e59378_d_n4, assign44110_e59378_d_n5, assign44110_e59378_d_n6, assign44110_e59378_d_n7, assign44110_e59378_d_n8, assign44110_e59378_d_n9, assign44110_e59378_d_n10, assign44110_e59378_d_n11, assign44110_e59378_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44110_e59375: f64 = (locals.var_vds - locals.var_pds);
        let assign44110_e59376: f64 = (0.5 * assign44110_e59375);
        (assign44110_e59376, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign44110_e59378;
        locals.var_t1_dn0 = assign44110_e59378_d_n0;
        locals.var_t1_dn2 = assign44110_e59378_d_n2;
        locals.var_t1_dn4 = assign44110_e59378_d_n4;
        locals.var_t1_dn5 = assign44110_e59378_d_n5;
        locals.var_t1_dn6 = assign44110_e59378_d_n6;
        locals.var_t1_dn7 = assign44110_e59378_d_n7;
        locals.var_t1_dn8 = assign44110_e59378_d_n8;
        locals.var_t1_dn9 = assign44110_e59378_d_n9;
        locals.var_t1_dn10 = assign44110_e59378_d_n10;
        locals.var_t1_dn11 = assign44110_e59378_d_n11;
        locals.var_t1_dn14 = assign44110_e59378_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign44120_e59393, assign44120_e59393_d_n0, assign44120_e59393_d_n2, assign44120_e59393_d_n4, assign44120_e59393_d_n5, assign44120_e59393_d_n6, assign44120_e59393_d_n7, assign44120_e59393_d_n8, assign44120_e59393_d_n9, assign44120_e59393_d_n10, assign44120_e59393_d_n11, assign44120_e59393_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44120_e59389: f64 = (2.0 * locals.var_t1);
        let assign44120_e59391: f64 = (assign44120_e59389 / 0.01);
        (assign44120_e59391, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn11) / 0.01), ((2.0 * locals.var_t1_dn14) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign44120_e59393;
        locals.var_tmf1_dn0 = assign44120_e59393_d_n0;
        locals.var_tmf1_dn2 = assign44120_e59393_d_n2;
        locals.var_tmf1_dn4 = assign44120_e59393_d_n4;
        locals.var_tmf1_dn5 = assign44120_e59393_d_n5;
        locals.var_tmf1_dn6 = assign44120_e59393_d_n6;
        locals.var_tmf1_dn7 = assign44120_e59393_d_n7;
        locals.var_tmf1_dn8 = assign44120_e59393_d_n8;
        locals.var_tmf1_dn9 = assign44120_e59393_d_n9;
        locals.var_tmf1_dn10 = assign44120_e59393_d_n10;
        locals.var_tmf1_dn11 = assign44120_e59393_d_n11;
        locals.var_tmf1_dn14 = assign44120_e59393_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign44130_e59440, assign44130_e59440_d_n0, assign44130_e59440_d_n2, assign44130_e59440_d_n4, assign44130_e59440_d_n5, assign44130_e59440_d_n6, assign44130_e59440_d_n7, assign44130_e59440_d_n8, assign44130_e59440_d_n9, assign44130_e59440_d_n10, assign44130_e59440_d_n11, assign44130_e59440_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44130_e59406: f64 = (1.0 / 2.0);
        let assign44130_e59410: f64 = (1.0 / 6.0);
        let assign44130_e59414: f64 = (1.0 / 24.0);
        let assign44130_e59418: f64 = (1.0 / 120.0);
        let assign44130_e59422: f64 = (1.0 / 720.0);
        let assign44130_e59426: f64 = (1.0 / 5040.0);
        let assign44130_e59427: f64 = (locals.var_tmf1 * assign44130_e59426);
        let assign44130_e59428: f64 = (assign44130_e59422 + assign44130_e59427);
        let assign44130_e59429: f64 = (locals.var_tmf1 * assign44130_e59428);
        let assign44130_e59430: f64 = (assign44130_e59418 + assign44130_e59429);
        let assign44130_e59431: f64 = (locals.var_tmf1 * assign44130_e59430);
        let assign44130_e59432: f64 = (assign44130_e59414 + assign44130_e59431);
        let assign44130_e59433: f64 = (locals.var_tmf1 * assign44130_e59432);
        let assign44130_e59434: f64 = (assign44130_e59410 + assign44130_e59433);
        let assign44130_e59435: f64 = (locals.var_tmf1 * assign44130_e59434);
        let assign44130_e59436: f64 = (assign44130_e59406 + assign44130_e59435);
        let assign44130_e59437: f64 = (locals.var_tmf1 * assign44130_e59436);
        let assign44130_e59438: f64 = (1.0 + assign44130_e59437);
        (assign44130_e59438, ((locals.var_tmf1_dn0 * assign44130_e59436) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44130_e59434) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44130_e59432) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44130_e59430) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44130_e59428) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign44130_e59426))))))))))), ((locals.var_tmf1_dn2 * assign44130_e59436) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44130_e59434) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44130_e59432) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44130_e59430) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44130_e59428) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign44130_e59426))))))))))), ((locals.var_tmf1_dn4 * assign44130_e59436) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44130_e59434) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44130_e59432) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44130_e59430) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44130_e59428) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign44130_e59426))))))))))), ((locals.var_tmf1_dn5 * assign44130_e59436) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44130_e59434) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44130_e59432) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44130_e59430) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44130_e59428) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign44130_e59426))))))))))), ((locals.var_tmf1_dn6 * assign44130_e59436) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44130_e59434) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44130_e59432) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44130_e59430) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44130_e59428) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign44130_e59426))))))))))), ((locals.var_tmf1_dn7 * assign44130_e59436) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44130_e59434) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44130_e59432) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44130_e59430) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44130_e59428) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign44130_e59426))))))))))), ((locals.var_tmf1_dn8 * assign44130_e59436) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44130_e59434) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44130_e59432) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44130_e59430) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44130_e59428) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign44130_e59426))))))))))), ((locals.var_tmf1_dn9 * assign44130_e59436) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44130_e59434) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44130_e59432) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44130_e59430) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44130_e59428) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign44130_e59426))))))))))), ((locals.var_tmf1_dn10 * assign44130_e59436) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44130_e59434) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44130_e59432) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44130_e59430) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44130_e59428) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign44130_e59426))))))))))), ((locals.var_tmf1_dn11 * assign44130_e59436) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign44130_e59434) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign44130_e59432) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign44130_e59430) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign44130_e59428) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign44130_e59426))))))))))), ((locals.var_tmf1_dn14 * assign44130_e59436) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign44130_e59434) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign44130_e59432) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign44130_e59430) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign44130_e59428) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign44130_e59426))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign44130_e59440;
        locals.var_tmf2_dn0 = assign44130_e59440_d_n0;
        locals.var_tmf2_dn2 = assign44130_e59440_d_n2;
        locals.var_tmf2_dn4 = assign44130_e59440_d_n4;
        locals.var_tmf2_dn5 = assign44130_e59440_d_n5;
        locals.var_tmf2_dn6 = assign44130_e59440_d_n6;
        locals.var_tmf2_dn7 = assign44130_e59440_d_n7;
        locals.var_tmf2_dn8 = assign44130_e59440_d_n8;
        locals.var_tmf2_dn9 = assign44130_e59440_d_n9;
        locals.var_tmf2_dn10 = assign44130_e59440_d_n10;
        locals.var_tmf2_dn11 = assign44130_e59440_d_n11;
        locals.var_tmf2_dn14 = assign44130_e59440_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign44140_e59483, assign44140_e59483_d_n0, assign44140_e59483_d_n2, assign44140_e59483_d_n4, assign44140_e59483_d_n5, assign44140_e59483_d_n6, assign44140_e59483_d_n7, assign44140_e59483_d_n8, assign44140_e59483_d_n9, assign44140_e59483_d_n10, assign44140_e59483_d_n11, assign44140_e59483_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44140_e59451: f64 = (1.0 / 2.0);
        let assign44140_e59455: f64 = (1.0 / 3.0);
        let assign44140_e59459: f64 = (1.0 / 8.0);
        let assign44140_e59463: f64 = (1.0 / 30.0);
        let assign44140_e59467: f64 = (1.0 / 144.0);
        let assign44140_e59471: f64 = (1.0 / 840.0);
        let assign44140_e59472: f64 = (locals.var_tmf1 * assign44140_e59471);
        let assign44140_e59473: f64 = (assign44140_e59467 + assign44140_e59472);
        let assign44140_e59474: f64 = (locals.var_tmf1 * assign44140_e59473);
        let assign44140_e59475: f64 = (assign44140_e59463 + assign44140_e59474);
        let assign44140_e59476: f64 = (locals.var_tmf1 * assign44140_e59475);
        let assign44140_e59477: f64 = (assign44140_e59459 + assign44140_e59476);
        let assign44140_e59478: f64 = (locals.var_tmf1 * assign44140_e59477);
        let assign44140_e59479: f64 = (assign44140_e59455 + assign44140_e59478);
        let assign44140_e59480: f64 = (locals.var_tmf1 * assign44140_e59479);
        let assign44140_e59481: f64 = (assign44140_e59451 + assign44140_e59480);
        (assign44140_e59481, ((locals.var_tmf1_dn0 * assign44140_e59479) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44140_e59477) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44140_e59475) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44140_e59473) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign44140_e59471))))))))), ((locals.var_tmf1_dn2 * assign44140_e59479) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44140_e59477) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44140_e59475) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44140_e59473) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign44140_e59471))))))))), ((locals.var_tmf1_dn4 * assign44140_e59479) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44140_e59477) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44140_e59475) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44140_e59473) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign44140_e59471))))))))), ((locals.var_tmf1_dn5 * assign44140_e59479) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44140_e59477) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44140_e59475) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44140_e59473) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign44140_e59471))))))))), ((locals.var_tmf1_dn6 * assign44140_e59479) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44140_e59477) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44140_e59475) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44140_e59473) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign44140_e59471))))))))), ((locals.var_tmf1_dn7 * assign44140_e59479) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44140_e59477) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44140_e59475) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44140_e59473) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign44140_e59471))))))))), ((locals.var_tmf1_dn8 * assign44140_e59479) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44140_e59477) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44140_e59475) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44140_e59473) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign44140_e59471))))))))), ((locals.var_tmf1_dn9 * assign44140_e59479) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44140_e59477) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44140_e59475) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44140_e59473) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign44140_e59471))))))))), ((locals.var_tmf1_dn10 * assign44140_e59479) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44140_e59477) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44140_e59475) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44140_e59473) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign44140_e59471))))))))), ((locals.var_tmf1_dn11 * assign44140_e59479) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign44140_e59477) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign44140_e59475) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign44140_e59473) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign44140_e59471))))))))), ((locals.var_tmf1_dn14 * assign44140_e59479) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign44140_e59477) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign44140_e59475) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign44140_e59473) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign44140_e59471))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign44140_e59483;
        locals.var_tmf3_dn0 = assign44140_e59483_d_n0;
        locals.var_tmf3_dn2 = assign44140_e59483_d_n2;
        locals.var_tmf3_dn4 = assign44140_e59483_d_n4;
        locals.var_tmf3_dn5 = assign44140_e59483_d_n5;
        locals.var_tmf3_dn6 = assign44140_e59483_d_n6;
        locals.var_tmf3_dn7 = assign44140_e59483_d_n7;
        locals.var_tmf3_dn8 = assign44140_e59483_d_n8;
        locals.var_tmf3_dn9 = assign44140_e59483_d_n9;
        locals.var_tmf3_dn10 = assign44140_e59483_d_n10;
        locals.var_tmf3_dn11 = assign44140_e59483_d_n11;
        locals.var_tmf3_dn14 = assign44140_e59483_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign44150_e59496, assign44150_e59496_d_n0, assign44150_e59496_d_n2, assign44150_e59496_d_n4, assign44150_e59496_d_n5, assign44150_e59496_d_n6, assign44150_e59496_d_n7, assign44150_e59496_d_n8, assign44150_e59496_d_n9, assign44150_e59496_d_n10, assign44150_e59496_d_n11, assign44150_e59496_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44150_e59494: f64 = (0.01 / locals.var_tmf2);
        (assign44150_e59494, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign44150_e59496;
        locals.var_t6_dn0 = assign44150_e59496_d_n0;
        locals.var_t6_dn2 = assign44150_e59496_d_n2;
        locals.var_t6_dn4 = assign44150_e59496_d_n4;
        locals.var_t6_dn5 = assign44150_e59496_d_n5;
        locals.var_t6_dn6 = assign44150_e59496_d_n6;
        locals.var_t6_dn7 = assign44150_e59496_d_n7;
        locals.var_t6_dn8 = assign44150_e59496_d_n8;
        locals.var_t6_dn9 = assign44150_e59496_d_n9;
        locals.var_t6_dn10 = assign44150_e59496_d_n10;
        locals.var_t6_dn11 = assign44150_e59496_d_n11;
        locals.var_t6_dn14 = assign44150_e59496_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign44160_e59514, assign44160_e59514_d_n0, assign44160_e59514_d_n2, assign44160_e59514_d_n4, assign44160_e59514_d_n5, assign44160_e59514_d_n6, assign44160_e59514_d_n7, assign44160_e59514_d_n8, assign44160_e59514_d_n9, assign44160_e59514_d_n10, assign44160_e59514_d_n11, assign44160_e59514_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44160_e59506: f64 = (-2.0);
        let assign44160_e59508: f64 = (assign44160_e59506 * locals.var_tmf3);
        let assign44160_e59511: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign44160_e59512: f64 = (assign44160_e59508 / assign44160_e59511);
        (assign44160_e59512, ((((assign44160_e59506 * locals.var_tmf3_dn0) * assign44160_e59511) - (assign44160_e59508 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign44160_e59511 * assign44160_e59511)), ((((assign44160_e59506 * locals.var_tmf3_dn2) * assign44160_e59511) - (assign44160_e59508 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign44160_e59511 * assign44160_e59511)), ((((assign44160_e59506 * locals.var_tmf3_dn4) * assign44160_e59511) - (assign44160_e59508 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign44160_e59511 * assign44160_e59511)), ((((assign44160_e59506 * locals.var_tmf3_dn5) * assign44160_e59511) - (assign44160_e59508 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign44160_e59511 * assign44160_e59511)), ((((assign44160_e59506 * locals.var_tmf3_dn6) * assign44160_e59511) - (assign44160_e59508 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign44160_e59511 * assign44160_e59511)), ((((assign44160_e59506 * locals.var_tmf3_dn7) * assign44160_e59511) - (assign44160_e59508 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign44160_e59511 * assign44160_e59511)), ((((assign44160_e59506 * locals.var_tmf3_dn8) * assign44160_e59511) - (assign44160_e59508 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign44160_e59511 * assign44160_e59511)), ((((assign44160_e59506 * locals.var_tmf3_dn9) * assign44160_e59511) - (assign44160_e59508 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign44160_e59511 * assign44160_e59511)), ((((assign44160_e59506 * locals.var_tmf3_dn10) * assign44160_e59511) - (assign44160_e59508 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign44160_e59511 * assign44160_e59511)), ((((assign44160_e59506 * locals.var_tmf3_dn11) * assign44160_e59511) - (assign44160_e59508 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign44160_e59511 * assign44160_e59511)), ((((assign44160_e59506 * locals.var_tmf3_dn14) * assign44160_e59511) - (assign44160_e59508 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign44160_e59511 * assign44160_e59511)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign44160_e59514;
        locals.var_t2_dn0 = assign44160_e59514_d_n0;
        locals.var_t2_dn2 = assign44160_e59514_d_n2;
        locals.var_t2_dn4 = assign44160_e59514_d_n4;
        locals.var_t2_dn5 = assign44160_e59514_d_n5;
        locals.var_t2_dn6 = assign44160_e59514_d_n6;
        locals.var_t2_dn7 = assign44160_e59514_d_n7;
        locals.var_t2_dn8 = assign44160_e59514_d_n8;
        locals.var_t2_dn9 = assign44160_e59514_d_n9;
        locals.var_t2_dn10 = assign44160_e59514_d_n10;
        locals.var_t2_dn11 = assign44160_e59514_d_n11;
        locals.var_t2_dn14 = assign44160_e59514_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign44170_e59529, assign44170_e59529_d_n0, assign44170_e59529_d_n2, assign44170_e59529_d_n4, assign44170_e59529_d_n5, assign44170_e59529_d_n6, assign44170_e59529_d_n7, assign44170_e59529_d_n8, assign44170_e59529_d_n9, assign44170_e59529_d_n10, assign44170_e59529_d_n11, assign44170_e59529_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44170_e59526: f64 = (locals.var_phi_s0_dep__blk855 + locals.var_t6);
        let assign44170_e59527: f64 = (1.1 - assign44170_e59526);
        (assign44170_e59527, (-(locals.var_phi_s0_dep__blk855_dn0 + locals.var_t6_dn0)), (-(locals.var_phi_s0_dep__blk855_dn2 + locals.var_t6_dn2)), (-(locals.var_phi_s0_dep__blk855_dn4 + locals.var_t6_dn4)), (-(locals.var_phi_s0_dep__blk855_dn5 + locals.var_t6_dn5)), (-(locals.var_phi_s0_dep__blk855_dn6 + locals.var_t6_dn6)), (-(locals.var_phi_s0_dep__blk855_dn7 + locals.var_t6_dn7)), (-(locals.var_phi_s0_dep__blk855_dn8 + locals.var_t6_dn8)), (-(locals.var_phi_s0_dep__blk855_dn9 + locals.var_t6_dn9)), (-(locals.var_phi_s0_dep__blk855_dn10 + locals.var_t6_dn10)), (-(locals.var_phi_s0_dep__blk855_dn11 + locals.var_t6_dn11)), (-(locals.var_phi_s0_dep__blk855_dn14 + locals.var_t6_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign44170_e59529;
        locals.var_t1_dn0 = assign44170_e59529_d_n0;
        locals.var_t1_dn2 = assign44170_e59529_d_n2;
        locals.var_t1_dn4 = assign44170_e59529_d_n4;
        locals.var_t1_dn5 = assign44170_e59529_d_n5;
        locals.var_t1_dn6 = assign44170_e59529_d_n6;
        locals.var_t1_dn7 = assign44170_e59529_d_n7;
        locals.var_t1_dn8 = assign44170_e59529_d_n8;
        locals.var_t1_dn9 = assign44170_e59529_d_n9;
        locals.var_t1_dn10 = assign44170_e59529_d_n10;
        locals.var_t1_dn11 = assign44170_e59529_d_n11;
        locals.var_t1_dn14 = assign44170_e59529_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign44180_e59549, assign44180_e59549_d_n0, assign44180_e59549_d_n2, assign44180_e59549_d_n4, assign44180_e59549_d_n5, assign44180_e59549_d_n6, assign44180_e59549_d_n7, assign44180_e59549_d_n8, assign44180_e59549_d_n9, assign44180_e59549_d_n10, assign44180_e59549_d_n11, assign44180_e59549_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44180_e59540: f64 = (locals.var_t1 * locals.var_t1);
        let assign44180_e59543: f64 = (4.0 * 0.05);
        let assign44180_e59545: f64 = (assign44180_e59543 * 0.05);
        let assign44180_e59546: f64 = (assign44180_e59540 + assign44180_e59545);
        let assign44180_e59547: f64 = (assign44180_e59546).sqrt();
        (assign44180_e59547, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign44180_e59547)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign44180_e59547)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign44180_e59547)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign44180_e59547)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign44180_e59547)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign44180_e59547)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign44180_e59547)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign44180_e59547)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign44180_e59547)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign44180_e59547)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign44180_e59547)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign44180_e59549;
        locals.var_tmf2_dn0 = assign44180_e59549_d_n0;
        locals.var_tmf2_dn2 = assign44180_e59549_d_n2;
        locals.var_tmf2_dn4 = assign44180_e59549_d_n4;
        locals.var_tmf2_dn5 = assign44180_e59549_d_n5;
        locals.var_tmf2_dn6 = assign44180_e59549_d_n6;
        locals.var_tmf2_dn7 = assign44180_e59549_d_n7;
        locals.var_tmf2_dn8 = assign44180_e59549_d_n8;
        locals.var_tmf2_dn9 = assign44180_e59549_d_n9;
        locals.var_tmf2_dn10 = assign44180_e59549_d_n10;
        locals.var_tmf2_dn11 = assign44180_e59549_d_n11;
        locals.var_tmf2_dn14 = assign44180_e59549_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign44190_e59566, assign44190_e59566_d_n0, assign44190_e59566_d_n2, assign44190_e59566_d_n4, assign44190_e59566_d_n5, assign44190_e59566_d_n6, assign44190_e59566_d_n7, assign44190_e59566_d_n8, assign44190_e59566_d_n9, assign44190_e59566_d_n10, assign44190_e59566_d_n11, assign44190_e59566_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44190_e59562: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign44190_e59563: f64 = (1.0 + assign44190_e59562);
        let assign44190_e59564: f64 = (0.5 * assign44190_e59563);
        (assign44190_e59564, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign44190_e59566;
        locals.var_t0_dn0 = assign44190_e59566_d_n0;
        locals.var_t0_dn2 = assign44190_e59566_d_n2;
        locals.var_t0_dn4 = assign44190_e59566_d_n4;
        locals.var_t0_dn5 = assign44190_e59566_d_n5;
        locals.var_t0_dn6 = assign44190_e59566_d_n6;
        locals.var_t0_dn7 = assign44190_e59566_d_n7;
        locals.var_t0_dn8 = assign44190_e59566_d_n8;
        locals.var_t0_dn9 = assign44190_e59566_d_n9;
        locals.var_t0_dn10 = assign44190_e59566_d_n10;
        locals.var_t0_dn11 = assign44190_e59566_d_n11;
        locals.var_t0_dn14 = assign44190_e59566_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign44200_e59581, assign44200_e59581_d_n0, assign44200_e59581_d_n2, assign44200_e59581_d_n4, assign44200_e59581_d_n5, assign44200_e59581_d_n6, assign44200_e59581_d_n7, assign44200_e59581_d_n8, assign44200_e59581_d_n9, assign44200_e59581_d_n10, assign44200_e59581_d_n11, assign44200_e59581_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44200_e59578: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign44200_e59579: f64 = (0.5 * assign44200_e59578);
        (assign44200_e59579, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign44200_e59581;
        locals.var_t2_dn0 = assign44200_e59581_d_n0;
        locals.var_t2_dn2 = assign44200_e59581_d_n2;
        locals.var_t2_dn4 = assign44200_e59581_d_n4;
        locals.var_t2_dn5 = assign44200_e59581_d_n5;
        locals.var_t2_dn6 = assign44200_e59581_d_n6;
        locals.var_t2_dn7 = assign44200_e59581_d_n7;
        locals.var_t2_dn8 = assign44200_e59581_d_n8;
        locals.var_t2_dn9 = assign44200_e59581_d_n9;
        locals.var_t2_dn10 = assign44200_e59581_d_n10;
        locals.var_t2_dn11 = assign44200_e59581_d_n11;
        locals.var_t2_dn14 = assign44200_e59581_d_n14;
        locals.var_t2_rv = 0.0;

        let assign44210_e59584: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1079 = assign44210_e59584;
        locals.var_guard1079_rv = 0.0;

        let (assign44220_e59597, assign44220_e59597_d_n0, assign44220_e59597_d_n2, assign44220_e59597_d_n4, assign44220_e59597_d_n5, assign44220_e59597_d_n6, assign44220_e59597_d_n7, assign44220_e59597_d_n8, assign44220_e59597_d_n9, assign44220_e59597_d_n10, assign44220_e59597_d_n11, assign44220_e59597_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign44220_e59597;
        locals.var_t2_dn0 = assign44220_e59597_d_n0;
        locals.var_t2_dn2 = assign44220_e59597_d_n2;
        locals.var_t2_dn4 = assign44220_e59597_d_n4;
        locals.var_t2_dn5 = assign44220_e59597_d_n5;
        locals.var_t2_dn6 = assign44220_e59597_d_n6;
        locals.var_t2_dn7 = assign44220_e59597_d_n7;
        locals.var_t2_dn8 = assign44220_e59597_d_n8;
        locals.var_t2_dn9 = assign44220_e59597_d_n9;
        locals.var_t2_dn10 = assign44220_e59597_d_n10;
        locals.var_t2_dn11 = assign44220_e59597_d_n11;
        locals.var_t2_dn14 = assign44220_e59597_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign44230_e59610, assign44230_e59610_d_n0, assign44230_e59610_d_n2, assign44230_e59610_d_n4, assign44230_e59610_d_n5, assign44230_e59610_d_n6, assign44230_e59610_d_n7, assign44230_e59610_d_n8, assign44230_e59610_d_n9, assign44230_e59610_d_n10, assign44230_e59610_d_n11, assign44230_e59610_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign44230_e59610;
        locals.var_t0_dn0 = assign44230_e59610_d_n0;
        locals.var_t0_dn2 = assign44230_e59610_d_n2;
        locals.var_t0_dn4 = assign44230_e59610_d_n4;
        locals.var_t0_dn5 = assign44230_e59610_d_n5;
        locals.var_t0_dn6 = assign44230_e59610_d_n6;
        locals.var_t0_dn7 = assign44230_e59610_d_n7;
        locals.var_t0_dn8 = assign44230_e59610_d_n8;
        locals.var_t0_dn9 = assign44230_e59610_d_n9;
        locals.var_t0_dn10 = assign44230_e59610_d_n10;
        locals.var_t0_dn11 = assign44230_e59610_d_n11;
        locals.var_t0_dn14 = assign44230_e59610_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign44240_e59623, assign44240_e59623_d_n0, assign44240_e59623_d_n2, assign44240_e59623_d_n4, assign44240_e59623_d_n5, assign44240_e59623_d_n6, assign44240_e59623_d_n7, assign44240_e59623_d_n8, assign44240_e59623_d_n9, assign44240_e59623_d_n10, assign44240_e59623_d_n11, assign44240_e59623_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44240_e59621: f64 = (locals.var_t2 + 1e-25);
        (assign44240_e59621, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign44240_e59623;
        locals.var_t2_dn0 = assign44240_e59623_d_n0;
        locals.var_t2_dn2 = assign44240_e59623_d_n2;
        locals.var_t2_dn4 = assign44240_e59623_d_n4;
        locals.var_t2_dn5 = assign44240_e59623_d_n5;
        locals.var_t2_dn6 = assign44240_e59623_d_n6;
        locals.var_t2_dn7 = assign44240_e59623_d_n7;
        locals.var_t2_dn8 = assign44240_e59623_d_n8;
        locals.var_t2_dn9 = assign44240_e59623_d_n9;
        locals.var_t2_dn10 = assign44240_e59623_d_n10;
        locals.var_t2_dn11 = assign44240_e59623_d_n11;
        locals.var_t2_dn14 = assign44240_e59623_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign44250_e59636, assign44250_e59636_d_n0, assign44250_e59636_d_n2, assign44250_e59636_d_n4, assign44250_e59636_d_n5, assign44250_e59636_d_n6, assign44250_e59636_d_n7, assign44250_e59636_d_n8, assign44250_e59636_d_n9, assign44250_e59636_d_n10, assign44250_e59636_d_n11, assign44250_e59636_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44250_e59634: f64 = (locals.var_beta * locals.var_ptl0);
        (assign44250_e59634, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn11 * locals.var_ptl0), (locals.var_beta_dn14 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign44250_e59636;
        locals.var_t0_dn0 = assign44250_e59636_d_n0;
        locals.var_t0_dn2 = assign44250_e59636_d_n2;
        locals.var_t0_dn4 = assign44250_e59636_d_n4;
        locals.var_t0_dn5 = assign44250_e59636_d_n5;
        locals.var_t0_dn6 = assign44250_e59636_d_n6;
        locals.var_t0_dn7 = assign44250_e59636_d_n7;
        locals.var_t0_dn8 = assign44250_e59636_d_n8;
        locals.var_t0_dn9 = assign44250_e59636_d_n9;
        locals.var_t0_dn10 = assign44250_e59636_d_n10;
        locals.var_t0_dn11 = assign44250_e59636_d_n11;
        locals.var_t0_dn14 = assign44250_e59636_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign44260_e59649, assign44260_e59649_d_n0, assign44260_e59649_d_n2, assign44260_e59649_d_n4, assign44260_e59649_d_n5, assign44260_e59649_d_n6, assign44260_e59649_d_n7, assign44260_e59649_d_n8, assign44260_e59649_d_n9, assign44260_e59649_d_n10, assign44260_e59649_d_n11, assign44260_e59649_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44260_e59647: f64 = (locals.var_cox * locals.var_t0);
        (assign44260_e59647, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn11 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn11)), ((locals.var_cox_dn14 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign44260_e59649;
        locals.var_t3_dn0 = assign44260_e59649_d_n0;
        locals.var_t3_dn2 = assign44260_e59649_d_n2;
        locals.var_t3_dn4 = assign44260_e59649_d_n4;
        locals.var_t3_dn5 = assign44260_e59649_d_n5;
        locals.var_t3_dn6 = assign44260_e59649_d_n6;
        locals.var_t3_dn7 = assign44260_e59649_d_n7;
        locals.var_t3_dn8 = assign44260_e59649_d_n8;
        locals.var_t3_dn9 = assign44260_e59649_d_n9;
        locals.var_t3_dn10 = assign44260_e59649_d_n10;
        locals.var_t3_dn11 = assign44260_e59649_d_n11;
        locals.var_t3_dn14 = assign44260_e59649_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign44270_e59662, assign44270_e59662_d_n0, assign44270_e59662_d_n2, assign44270_e59662_d_n4, assign44270_e59662_d_n5, assign44270_e59662_d_n6, assign44270_e59662_d_n7, assign44270_e59662_d_n8, assign44270_e59662_d_n9, assign44270_e59662_d_n10, assign44270_e59662_d_n11, assign44270_e59662_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44270_e59660: f64 = (locals.var_t2).powf(p.p284);
        (assign44270_e59660, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign44270_e59660 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign44270_e59660 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign44270_e59660 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign44270_e59660 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign44270_e59660 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign44270_e59660 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign44270_e59660 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign44270_e59660 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign44270_e59660 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn11)) } } else { (assign44270_e59660 * (p.p284 * (locals.var_t2_dn11 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn14)) } } else { (assign44270_e59660 * (p.p284 * (locals.var_t2_dn14 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign44270_e59662;
        locals.var_t0_dn0 = assign44270_e59662_d_n0;
        locals.var_t0_dn2 = assign44270_e59662_d_n2;
        locals.var_t0_dn4 = assign44270_e59662_d_n4;
        locals.var_t0_dn5 = assign44270_e59662_d_n5;
        locals.var_t0_dn6 = assign44270_e59662_d_n6;
        locals.var_t0_dn7 = assign44270_e59662_d_n7;
        locals.var_t0_dn8 = assign44270_e59662_d_n8;
        locals.var_t0_dn9 = assign44270_e59662_d_n9;
        locals.var_t0_dn10 = assign44270_e59662_d_n10;
        locals.var_t0_dn11 = assign44270_e59662_d_n11;
        locals.var_t0_dn14 = assign44270_e59662_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign44280_e59675, assign44280_e59675_d_n0, assign44280_e59675_d_n2, assign44280_e59675_d_n4, assign44280_e59675_d_n5, assign44280_e59675_d_n6, assign44280_e59675_d_n7, assign44280_e59675_d_n8, assign44280_e59675_d_n9, assign44280_e59675_d_n10, assign44280_e59675_d_n11, assign44280_e59675_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44280_e59673: f64 = (locals.var_t3 * locals.var_t0);
        (assign44280_e59673, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn11 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn11)), ((locals.var_t3_dn14 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign44280_e59675;
        locals.var_t9_dn0 = assign44280_e59675_d_n0;
        locals.var_t9_dn2 = assign44280_e59675_d_n2;
        locals.var_t9_dn4 = assign44280_e59675_d_n4;
        locals.var_t9_dn5 = assign44280_e59675_d_n5;
        locals.var_t9_dn6 = assign44280_e59675_d_n6;
        locals.var_t9_dn7 = assign44280_e59675_d_n7;
        locals.var_t9_dn8 = assign44280_e59675_d_n8;
        locals.var_t9_dn9 = assign44280_e59675_d_n9;
        locals.var_t9_dn10 = assign44280_e59675_d_n10;
        locals.var_t9_dn11 = assign44280_e59675_d_n11;
        locals.var_t9_dn14 = assign44280_e59675_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign44290_e59690, assign44290_e59690_d_n0, assign44290_e59690_d_n2, assign44290_e59690_d_n4, assign44290_e59690_d_n5, assign44290_e59690_d_n6, assign44290_e59690_d_n7, assign44290_e59690_d_n8, assign44290_e59690_d_n9, assign44290_e59690_d_n10, assign44290_e59690_d_n11, assign44290_e59690_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44290_e59687: f64 = (locals.var_vdsz__blk441 * p.p285);
        let assign44290_e59688: f64 = (1.0 + assign44290_e59687);
        (assign44290_e59688, (locals.var_vdsz__blk441_dn0 * p.p285), (locals.var_vdsz__blk441_dn2 * p.p285), (locals.var_vdsz__blk441_dn4 * p.p285), (locals.var_vdsz__blk441_dn5 * p.p285), (locals.var_vdsz__blk441_dn6 * p.p285), (locals.var_vdsz__blk441_dn7 * p.p285), (locals.var_vdsz__blk441_dn8 * p.p285), (locals.var_vdsz__blk441_dn9 * p.p285), (locals.var_vdsz__blk441_dn10 * p.p285), (locals.var_vdsz__blk441_dn11 * p.p285), (locals.var_vdsz__blk441_dn14 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign44290_e59690;
        locals.var_t4_dn0 = assign44290_e59690_d_n0;
        locals.var_t4_dn2 = assign44290_e59690_d_n2;
        locals.var_t4_dn4 = assign44290_e59690_d_n4;
        locals.var_t4_dn5 = assign44290_e59690_d_n5;
        locals.var_t4_dn6 = assign44290_e59690_d_n6;
        locals.var_t4_dn7 = assign44290_e59690_d_n7;
        locals.var_t4_dn8 = assign44290_e59690_d_n8;
        locals.var_t4_dn9 = assign44290_e59690_d_n9;
        locals.var_t4_dn10 = assign44290_e59690_d_n10;
        locals.var_t4_dn11 = assign44290_e59690_d_n11;
        locals.var_t4_dn14 = assign44290_e59690_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign44300_e59701, assign44300_e59701_d_n0, assign44300_e59701_d_n2, assign44300_e59701_d_n4, assign44300_e59701_d_n5, assign44300_e59701_d_n6, assign44300_e59701_d_n7, assign44300_e59701_d_n8, assign44300_e59701_d_n9, assign44300_e59701_d_n10, assign44300_e59701_d_n11, assign44300_e59701_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign44300_e59701;
        locals.var_t0_dn0 = assign44300_e59701_d_n0;
        locals.var_t0_dn2 = assign44300_e59701_d_n2;
        locals.var_t0_dn4 = assign44300_e59701_d_n4;
        locals.var_t0_dn5 = assign44300_e59701_d_n5;
        locals.var_t0_dn6 = assign44300_e59701_d_n6;
        locals.var_t0_dn7 = assign44300_e59701_d_n7;
        locals.var_t0_dn8 = assign44300_e59701_d_n8;
        locals.var_t0_dn9 = assign44300_e59701_d_n9;
        locals.var_t0_dn10 = assign44300_e59701_d_n10;
        locals.var_t0_dn11 = assign44300_e59701_d_n11;
        locals.var_t0_dn14 = assign44300_e59701_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_154(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign44310_e59716, assign44310_e59716_d_n0, assign44310_e59716_d_n2, assign44310_e59716_d_n4, assign44310_e59716_d_n5, assign44310_e59716_d_n6, assign44310_e59716_d_n7, assign44310_e59716_d_n8, assign44310_e59716_d_n9, assign44310_e59716_d_n10, assign44310_e59716_d_n11, assign44310_e59716_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44310_e59712: f64 = (locals.var_phi_s0_dep__blk855 + locals.var_t6);
        let assign44310_e59714: f64 = (assign44310_e59712 - locals.var_vbsz__blk440);
        (assign44310_e59714, ((locals.var_phi_s0_dep__blk855_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk440_dn0), ((locals.var_phi_s0_dep__blk855_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk440_dn2), ((locals.var_phi_s0_dep__blk855_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk440_dn4), ((locals.var_phi_s0_dep__blk855_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk440_dn5), ((locals.var_phi_s0_dep__blk855_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk440_dn6), ((locals.var_phi_s0_dep__blk855_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk440_dn7), ((locals.var_phi_s0_dep__blk855_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk440_dn8), ((locals.var_phi_s0_dep__blk855_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk440_dn9), ((locals.var_phi_s0_dep__blk855_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk440_dn10), ((locals.var_phi_s0_dep__blk855_dn11 + locals.var_t6_dn11) - locals.var_vbsz__blk440_dn11), ((locals.var_phi_s0_dep__blk855_dn14 + locals.var_t6_dn14) - locals.var_vbsz__blk440_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign44310_e59716;
        locals.var_t5_dn0 = assign44310_e59716_d_n0;
        locals.var_t5_dn2 = assign44310_e59716_d_n2;
        locals.var_t5_dn4 = assign44310_e59716_d_n4;
        locals.var_t5_dn5 = assign44310_e59716_d_n5;
        locals.var_t5_dn6 = assign44310_e59716_d_n6;
        locals.var_t5_dn7 = assign44310_e59716_d_n7;
        locals.var_t5_dn8 = assign44310_e59716_d_n8;
        locals.var_t5_dn9 = assign44310_e59716_d_n9;
        locals.var_t5_dn10 = assign44310_e59716_d_n10;
        locals.var_t5_dn11 = assign44310_e59716_d_n11;
        locals.var_t5_dn14 = assign44310_e59716_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign44320_e59733, assign44320_e59733_d_n0, assign44320_e59733_d_n2, assign44320_e59733_d_n4, assign44320_e59733_d_n5, assign44320_e59733_d_n6, assign44320_e59733_d_n7, assign44320_e59733_d_n8, assign44320_e59733_d_n9, assign44320_e59733_d_n10, assign44320_e59733_d_n11, assign44320_e59733_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44320_e59728: f64 = (locals.var_vdsz__blk441 * locals.var_t0);
        let assign44320_e59730: f64 = (assign44320_e59728 * locals.var_t5);
        let assign44320_e59731: f64 = (locals.var_t4 + assign44320_e59730);
        (assign44320_e59731, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk441_dn0 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn0)) * locals.var_t5) + (assign44320_e59728 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk441_dn2 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn2)) * locals.var_t5) + (assign44320_e59728 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk441_dn4 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn4)) * locals.var_t5) + (assign44320_e59728 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk441_dn5 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn5)) * locals.var_t5) + (assign44320_e59728 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk441_dn6 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn6)) * locals.var_t5) + (assign44320_e59728 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk441_dn7 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn7)) * locals.var_t5) + (assign44320_e59728 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk441_dn8 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn8)) * locals.var_t5) + (assign44320_e59728 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk441_dn9 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn9)) * locals.var_t5) + (assign44320_e59728 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk441_dn10 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn10)) * locals.var_t5) + (assign44320_e59728 * locals.var_t5_dn10))), (locals.var_t4_dn11 + ((((locals.var_vdsz__blk441_dn11 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn11)) * locals.var_t5) + (assign44320_e59728 * locals.var_t5_dn11))), (locals.var_t4_dn14 + ((((locals.var_vdsz__blk441_dn14 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn14)) * locals.var_t5) + (assign44320_e59728 * locals.var_t5_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign44320_e59733;
        locals.var_t4_dn0 = assign44320_e59733_d_n0;
        locals.var_t4_dn2 = assign44320_e59733_d_n2;
        locals.var_t4_dn4 = assign44320_e59733_d_n4;
        locals.var_t4_dn5 = assign44320_e59733_d_n5;
        locals.var_t4_dn6 = assign44320_e59733_d_n6;
        locals.var_t4_dn7 = assign44320_e59733_d_n7;
        locals.var_t4_dn8 = assign44320_e59733_d_n8;
        locals.var_t4_dn9 = assign44320_e59733_d_n9;
        locals.var_t4_dn10 = assign44320_e59733_d_n10;
        locals.var_t4_dn11 = assign44320_e59733_d_n11;
        locals.var_t4_dn14 = assign44320_e59733_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign44330_e59746, assign44330_e59746_d_n0, assign44330_e59746_d_n2, assign44330_e59746_d_n4, assign44330_e59746_d_n5, assign44330_e59746_d_n6, assign44330_e59746_d_n7, assign44330_e59746_d_n8, assign44330_e59746_d_n9, assign44330_e59746_d_n10, assign44330_e59746_d_n11, assign44330_e59746_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44330_e59744: f64 = (locals.var_t9 * locals.var_t4);
        (assign44330_e59744, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn7 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn7)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn9 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn9)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn11 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn11)), ((locals.var_t9_dn14 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign44330_e59746;
        locals.var_t6_dn0 = assign44330_e59746_d_n0;
        locals.var_t6_dn2 = assign44330_e59746_d_n2;
        locals.var_t6_dn4 = assign44330_e59746_d_n4;
        locals.var_t6_dn5 = assign44330_e59746_d_n5;
        locals.var_t6_dn6 = assign44330_e59746_d_n6;
        locals.var_t6_dn7 = assign44330_e59746_d_n7;
        locals.var_t6_dn8 = assign44330_e59746_d_n8;
        locals.var_t6_dn9 = assign44330_e59746_d_n9;
        locals.var_t6_dn10 = assign44330_e59746_d_n10;
        locals.var_t6_dn11 = assign44330_e59746_d_n11;
        locals.var_t6_dn14 = assign44330_e59746_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign44340_e59757, assign44340_e59757_d_n0, assign44340_e59757_d_n2, assign44340_e59757_d_n4, assign44340_e59757_d_n5, assign44340_e59757_d_n6, assign44340_e59757_d_n7, assign44340_e59757_d_n8, assign44340_e59757_d_n9, assign44340_e59757_d_n10, assign44340_e59757_d_n11, assign44340_e59757_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign44340_e59757;
        locals.var_t9_dn0 = assign44340_e59757_d_n0;
        locals.var_t9_dn2 = assign44340_e59757_d_n2;
        locals.var_t9_dn4 = assign44340_e59757_d_n4;
        locals.var_t9_dn5 = assign44340_e59757_d_n5;
        locals.var_t9_dn6 = assign44340_e59757_d_n6;
        locals.var_t9_dn7 = assign44340_e59757_d_n7;
        locals.var_t9_dn8 = assign44340_e59757_d_n8;
        locals.var_t9_dn9 = assign44340_e59757_d_n9;
        locals.var_t9_dn10 = assign44340_e59757_d_n10;
        locals.var_t9_dn11 = assign44340_e59757_d_n11;
        locals.var_t9_dn14 = assign44340_e59757_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign44350_e59769, assign44350_e59769_d_n0, assign44350_e59769_d_n2, assign44350_e59769_d_n4, assign44350_e59769_d_n5, assign44350_e59769_d_n6, assign44350_e59769_d_n7, assign44350_e59769_d_n8, assign44350_e59769_d_n9, assign44350_e59769_d_n10, assign44350_e59769_d_n11, assign44350_e59769_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1078 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign44350_e59769;
        locals.var_t9_dn0 = assign44350_e59769_d_n0;
        locals.var_t9_dn2 = assign44350_e59769_d_n2;
        locals.var_t9_dn4 = assign44350_e59769_d_n4;
        locals.var_t9_dn5 = assign44350_e59769_d_n5;
        locals.var_t9_dn6 = assign44350_e59769_d_n6;
        locals.var_t9_dn7 = assign44350_e59769_d_n7;
        locals.var_t9_dn8 = assign44350_e59769_d_n8;
        locals.var_t9_dn9 = assign44350_e59769_d_n9;
        locals.var_t9_dn10 = assign44350_e59769_d_n10;
        locals.var_t9_dn11 = assign44350_e59769_d_n11;
        locals.var_t9_dn14 = assign44350_e59769_d_n14;
        locals.var_t9_rv = 0.0;

        let assign44360_e59772: f64 = if p.p287 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign44360_e59772;
        locals.var_guard1080_rv = 0.0;

        let (assign44370_e59785, assign44370_e59785_d_n0, assign44370_e59785_d_n2, assign44370_e59785_d_n4, assign44370_e59785_d_n5, assign44370_e59785_d_n6, assign44370_e59785_d_n7, assign44370_e59785_d_n8, assign44370_e59785_d_n9, assign44370_e59785_d_n10, assign44370_e59785_d_n11, assign44370_e59785_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1080 != 0.0)) {
        let assign44370_e59783: f64 = (locals.var_beta * locals.var_gdl0);
        (assign44370_e59783, (locals.var_beta_dn0 * locals.var_gdl0), (locals.var_beta_dn2 * locals.var_gdl0), (locals.var_beta_dn4 * locals.var_gdl0), (locals.var_beta_dn5 * locals.var_gdl0), (locals.var_beta_dn6 * locals.var_gdl0), (locals.var_beta_dn7 * locals.var_gdl0), (locals.var_beta_dn8 * locals.var_gdl0), (locals.var_beta_dn9 * locals.var_gdl0), (locals.var_beta_dn10 * locals.var_gdl0), (locals.var_beta_dn11 * locals.var_gdl0), (locals.var_beta_dn14 * locals.var_gdl0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign44370_e59785;
        locals.var_t1_dn0 = assign44370_e59785_d_n0;
        locals.var_t1_dn2 = assign44370_e59785_d_n2;
        locals.var_t1_dn4 = assign44370_e59785_d_n4;
        locals.var_t1_dn5 = assign44370_e59785_d_n5;
        locals.var_t1_dn6 = assign44370_e59785_d_n6;
        locals.var_t1_dn7 = assign44370_e59785_d_n7;
        locals.var_t1_dn8 = assign44370_e59785_d_n8;
        locals.var_t1_dn9 = assign44370_e59785_d_n9;
        locals.var_t1_dn10 = assign44370_e59785_d_n10;
        locals.var_t1_dn11 = assign44370_e59785_d_n11;
        locals.var_t1_dn14 = assign44370_e59785_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign44380_e59798, assign44380_e59798_d_n0, assign44380_e59798_d_n2, assign44380_e59798_d_n4, assign44380_e59798_d_n5, assign44380_e59798_d_n6, assign44380_e59798_d_n7, assign44380_e59798_d_n8, assign44380_e59798_d_n9, assign44380_e59798_d_n10, assign44380_e59798_d_n11, assign44380_e59798_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1080 != 0.0)) {
        let assign44380_e59796: f64 = (locals.var_cox * locals.var_t1);
        (assign44380_e59796, ((locals.var_cox_dn0 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn0)), ((locals.var_cox_dn2 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn2)), ((locals.var_cox_dn4 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn4)), ((locals.var_cox_dn5 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn5)), ((locals.var_cox_dn6 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn6)), ((locals.var_cox_dn7 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn7)), ((locals.var_cox_dn8 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn8)), ((locals.var_cox_dn9 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn9)), ((locals.var_cox_dn10 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn10)), ((locals.var_cox_dn11 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn11)), ((locals.var_cox_dn14 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign44380_e59798;
        locals.var_t2_dn0 = assign44380_e59798_d_n0;
        locals.var_t2_dn2 = assign44380_e59798_d_n2;
        locals.var_t2_dn4 = assign44380_e59798_d_n4;
        locals.var_t2_dn5 = assign44380_e59798_d_n5;
        locals.var_t2_dn6 = assign44380_e59798_d_n6;
        locals.var_t2_dn7 = assign44380_e59798_d_n7;
        locals.var_t2_dn8 = assign44380_e59798_d_n8;
        locals.var_t2_dn9 = assign44380_e59798_d_n9;
        locals.var_t2_dn10 = assign44380_e59798_d_n10;
        locals.var_t2_dn11 = assign44380_e59798_d_n11;
        locals.var_t2_dn14 = assign44380_e59798_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign44390_e59811, assign44390_e59811_d_n0, assign44390_e59811_d_n2, assign44390_e59811_d_n4, assign44390_e59811_d_n5, assign44390_e59811_d_n6, assign44390_e59811_d_n7, assign44390_e59811_d_n8, assign44390_e59811_d_n9, assign44390_e59811_d_n10, assign44390_e59811_d_n11, assign44390_e59811_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1080 != 0.0)) {
        let assign44390_e59809: f64 = (locals.var_t2 * locals.var_vdsz__blk441);
        (assign44390_e59809, ((locals.var_t2_dn0 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn0)), ((locals.var_t2_dn2 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn2)), ((locals.var_t2_dn4 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn4)), ((locals.var_t2_dn5 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn5)), ((locals.var_t2_dn6 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn6)), ((locals.var_t2_dn7 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn7)), ((locals.var_t2_dn8 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn8)), ((locals.var_t2_dn9 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn9)), ((locals.var_t2_dn10 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn10)), ((locals.var_t2_dn11 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn11)), ((locals.var_t2_dn14 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign44390_e59811;
        locals.var_t8_dn0 = assign44390_e59811_d_n0;
        locals.var_t8_dn2 = assign44390_e59811_d_n2;
        locals.var_t8_dn4 = assign44390_e59811_d_n4;
        locals.var_t8_dn5 = assign44390_e59811_d_n5;
        locals.var_t8_dn6 = assign44390_e59811_d_n6;
        locals.var_t8_dn7 = assign44390_e59811_d_n7;
        locals.var_t8_dn8 = assign44390_e59811_d_n8;
        locals.var_t8_dn9 = assign44390_e59811_d_n9;
        locals.var_t8_dn10 = assign44390_e59811_d_n10;
        locals.var_t8_dn11 = assign44390_e59811_d_n11;
        locals.var_t8_dn14 = assign44390_e59811_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign44400_e59823, assign44400_e59823_d_n0, assign44400_e59823_d_n2, assign44400_e59823_d_n4, assign44400_e59823_d_n5, assign44400_e59823_d_n6, assign44400_e59823_d_n7, assign44400_e59823_d_n8, assign44400_e59823_d_n9, assign44400_e59823_d_n10, assign44400_e59823_d_n11, assign44400_e59823_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1080 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign44400_e59823;
        locals.var_t8_dn0 = assign44400_e59823_d_n0;
        locals.var_t8_dn2 = assign44400_e59823_d_n2;
        locals.var_t8_dn4 = assign44400_e59823_d_n4;
        locals.var_t8_dn5 = assign44400_e59823_d_n5;
        locals.var_t8_dn6 = assign44400_e59823_d_n6;
        locals.var_t8_dn7 = assign44400_e59823_d_n7;
        locals.var_t8_dn8 = assign44400_e59823_d_n8;
        locals.var_t8_dn9 = assign44400_e59823_d_n9;
        locals.var_t8_dn10 = assign44400_e59823_d_n10;
        locals.var_t8_dn11 = assign44400_e59823_d_n11;
        locals.var_t8_dn14 = assign44400_e59823_d_n14;
        locals.var_t8_rv = 0.0;

        let assign44410_e59826: f64 = (locals.var_t9 + locals.var_t8);
        let assign44410_e59828: f64 = if assign44410_e59826 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign44410_e59828;
        locals.var_guard1081_rv = 0.0;

        let (assign44420_e59843, assign44420_e59843_d_n0, assign44420_e59843_d_n2, assign44420_e59843_d_n4, assign44420_e59843_d_n5, assign44420_e59843_d_n6, assign44420_e59843_d_n7, assign44420_e59843_d_n8, assign44420_e59843_d_n9, assign44420_e59843_d_n10, assign44420_e59843_d_n11, assign44420_e59843_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1081 != 0.0)) {
        let assign44420_e59840: f64 = (locals.var_t9 + locals.var_t8);
        let assign44420_e59841: f64 = (locals.var_pds * assign44420_e59840);
        (assign44420_e59841, ((locals.var_pds_dn0 * assign44420_e59840) + (locals.var_pds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pds_dn2 * assign44420_e59840) + (locals.var_pds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pds_dn4 * assign44420_e59840) + (locals.var_pds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pds_dn5 * assign44420_e59840) + (locals.var_pds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pds_dn6 * assign44420_e59840) + (locals.var_pds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pds_dn7 * assign44420_e59840) + (locals.var_pds * (locals.var_t9_dn7 + locals.var_t8_dn7))), ((locals.var_pds_dn8 * assign44420_e59840) + (locals.var_pds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pds_dn9 * assign44420_e59840) + (locals.var_pds * (locals.var_t9_dn9 + locals.var_t8_dn9))), ((locals.var_pds_dn10 * assign44420_e59840) + (locals.var_pds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pds_dn11 * assign44420_e59840) + (locals.var_pds * (locals.var_t9_dn11 + locals.var_t8_dn11))), ((locals.var_pds_dn14 * assign44420_e59840) + (locals.var_pds * (locals.var_t9_dn14 + locals.var_t8_dn14))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn4, locals.var_idd1_dn5, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn8, locals.var_idd1_dn9, locals.var_idd1_dn10, locals.var_idd1_dn11, locals.var_idd1_dn14,)
    }
};
        locals.var_idd1 = assign44420_e59843;
        locals.var_idd1_dn0 = assign44420_e59843_d_n0;
        locals.var_idd1_dn2 = assign44420_e59843_d_n2;
        locals.var_idd1_dn4 = assign44420_e59843_d_n4;
        locals.var_idd1_dn5 = assign44420_e59843_d_n5;
        locals.var_idd1_dn6 = assign44420_e59843_d_n6;
        locals.var_idd1_dn7 = assign44420_e59843_d_n7;
        locals.var_idd1_dn8 = assign44420_e59843_d_n8;
        locals.var_idd1_dn9 = assign44420_e59843_d_n9;
        locals.var_idd1_dn10 = assign44420_e59843_d_n10;
        locals.var_idd1_dn11 = assign44420_e59843_d_n11;
        locals.var_idd1_dn14 = assign44420_e59843_d_n14;
        locals.var_idd1_rv = 0.0;

        let (assign44430_e59860, assign44430_e59860_d_n0, assign44430_e59860_d_n2, assign44430_e59860_d_n4, assign44430_e59860_d_n5, assign44430_e59860_d_n6, assign44430_e59860_d_n7, assign44430_e59860_d_n8, assign44430_e59860_d_n9, assign44430_e59860_d_n10, assign44430_e59860_d_n11, assign44430_e59860_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1081 != 0.0)) {
        let assign44430_e59855: f64 = (locals.var_betawl * locals.var_idd1);
        let assign44430_e59857: f64 = (assign44430_e59855 * locals.var_mu);
        let assign44430_e59858: f64 = (locals.var_ids0 + assign44430_e59857);
        (assign44430_e59858, (locals.var_ids0_dn0 + ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign44430_e59855 * locals.var_mu_dn0))), (locals.var_ids0_dn2 + ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign44430_e59855 * locals.var_mu_dn2))), (locals.var_ids0_dn4 + ((((locals.var_betawl_dn4 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn4)) * locals.var_mu) + (assign44430_e59855 * locals.var_mu_dn4))), (locals.var_ids0_dn5 + ((((locals.var_betawl_dn5 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn5)) * locals.var_mu) + (assign44430_e59855 * locals.var_mu_dn5))), (locals.var_ids0_dn6 + ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign44430_e59855 * locals.var_mu_dn6))), (locals.var_ids0_dn7 + ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign44430_e59855 * locals.var_mu_dn7))), (locals.var_ids0_dn8 + ((((locals.var_betawl_dn8 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn8)) * locals.var_mu) + (assign44430_e59855 * locals.var_mu_dn8))), (locals.var_ids0_dn9 + ((((locals.var_betawl_dn9 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn9)) * locals.var_mu) + (assign44430_e59855 * locals.var_mu_dn9))), (locals.var_ids0_dn10 + ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign44430_e59855 * locals.var_mu_dn10))), (locals.var_ids0_dn11 + ((((locals.var_betawl_dn11 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn11)) * locals.var_mu) + (assign44430_e59855 * locals.var_mu_dn11))), (locals.var_ids0_dn14 + ((((locals.var_betawl_dn14 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn14)) * locals.var_mu) + (assign44430_e59855 * locals.var_mu_dn14))),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign44430_e59860;
        locals.var_ids0_dn0 = assign44430_e59860_d_n0;
        locals.var_ids0_dn2 = assign44430_e59860_d_n2;
        locals.var_ids0_dn4 = assign44430_e59860_d_n4;
        locals.var_ids0_dn5 = assign44430_e59860_d_n5;
        locals.var_ids0_dn6 = assign44430_e59860_d_n6;
        locals.var_ids0_dn7 = assign44430_e59860_d_n7;
        locals.var_ids0_dn8 = assign44430_e59860_d_n8;
        locals.var_ids0_dn9 = assign44430_e59860_d_n9;
        locals.var_ids0_dn10 = assign44430_e59860_d_n10;
        locals.var_ids0_dn11 = assign44430_e59860_d_n11;
        locals.var_ids0_dn14 = assign44430_e59860_d_n14;
        locals.var_ids0_rv = 0.0;

        let assign44440_e59867: f64 = if ((locals.var_flg_rsrd == 2.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign44440_e59867;
        locals.var_guard1082_rv = 0.0;

        let assign44450_e59870: f64 = if p.p296 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign44450_e59870;
        locals.var_guard1083_rv = 0.0;

        let (assign44460_e59883, assign44460_e59883_d_n0, assign44460_e59883_d_n2, assign44460_e59883_d_n4, assign44460_e59883_d_n5, assign44460_e59883_d_n6, assign44460_e59883_d_n7, assign44460_e59883_d_n8, assign44460_e59883_d_n9, assign44460_e59883_d_n10, assign44460_e59883_d_n11, assign44460_e59883_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign44460_e59883;
        locals.var_t4_dn0 = assign44460_e59883_d_n0;
        locals.var_t4_dn2 = assign44460_e59883_d_n2;
        locals.var_t4_dn4 = assign44460_e59883_d_n4;
        locals.var_t4_dn5 = assign44460_e59883_d_n5;
        locals.var_t4_dn6 = assign44460_e59883_d_n6;
        locals.var_t4_dn7 = assign44460_e59883_d_n7;
        locals.var_t4_dn8 = assign44460_e59883_d_n8;
        locals.var_t4_dn9 = assign44460_e59883_d_n9;
        locals.var_t4_dn10 = assign44460_e59883_d_n10;
        locals.var_t4_dn11 = assign44460_e59883_d_n11;
        locals.var_t4_dn14 = assign44460_e59883_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign44470_e59900, assign44470_e59900_d_n0, assign44470_e59900_d_n2, assign44470_e59900_d_n4, assign44470_e59900_d_n5, assign44470_e59900_d_n6, assign44470_e59900_d_n7, assign44470_e59900_d_n8, assign44470_e59900_d_n9, assign44470_e59900_d_n10, assign44470_e59900_d_n11, assign44470_e59900_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign44470_e59897: f64 = (locals.var_vgse - p.p300);
        let assign44470_e59898: f64 = (locals.var_uc_rd24 * assign44470_e59897);
        (assign44470_e59898, (locals.var_uc_rd24 * locals.var_vgse_dn0), (locals.var_uc_rd24 * locals.var_vgse_dn2), 0.0, 0.0, 0.0, (locals.var_uc_rd24 * locals.var_vgse_dn7), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign44470_e59900;
        locals.var_t1_dn0 = assign44470_e59900_d_n0;
        locals.var_t1_dn2 = assign44470_e59900_d_n2;
        locals.var_t1_dn4 = assign44470_e59900_d_n4;
        locals.var_t1_dn5 = assign44470_e59900_d_n5;
        locals.var_t1_dn6 = assign44470_e59900_d_n6;
        locals.var_t1_dn7 = assign44470_e59900_d_n7;
        locals.var_t1_dn8 = assign44470_e59900_d_n8;
        locals.var_t1_dn9 = assign44470_e59900_d_n9;
        locals.var_t1_dn10 = assign44470_e59900_d_n10;
        locals.var_t1_dn11 = assign44470_e59900_d_n11;
        locals.var_t1_dn14 = assign44470_e59900_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign44480_e59919, assign44480_e59919_d_n0, assign44480_e59919_d_n2, assign44480_e59919_d_n4, assign44480_e59919_d_n5, assign44480_e59919_d_n6, assign44480_e59919_d_n7, assign44480_e59919_d_n8, assign44480_e59919_d_n9, assign44480_e59919_d_n10, assign44480_e59919_d_n11, assign44480_e59919_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign44480_e59913: f64 = (locals.var_t1 - locals.var_t4);
        let assign44480_e59916: f64 = (0.01 * 0.01);
        let assign44480_e59917: f64 = (assign44480_e59913 - assign44480_e59916);
        (assign44480_e59917, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn11 - locals.var_t4_dn11), (locals.var_t1_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign44480_e59919;
        locals.var_tmf1_dn0 = assign44480_e59919_d_n0;
        locals.var_tmf1_dn2 = assign44480_e59919_d_n2;
        locals.var_tmf1_dn4 = assign44480_e59919_d_n4;
        locals.var_tmf1_dn5 = assign44480_e59919_d_n5;
        locals.var_tmf1_dn6 = assign44480_e59919_d_n6;
        locals.var_tmf1_dn7 = assign44480_e59919_d_n7;
        locals.var_tmf1_dn8 = assign44480_e59919_d_n8;
        locals.var_tmf1_dn9 = assign44480_e59919_d_n9;
        locals.var_tmf1_dn10 = assign44480_e59919_d_n10;
        locals.var_tmf1_dn11 = assign44480_e59919_d_n11;
        locals.var_tmf1_dn14 = assign44480_e59919_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign44490_e59938, assign44490_e59938_d_n0, assign44490_e59938_d_n2, assign44490_e59938_d_n4, assign44490_e59938_d_n5, assign44490_e59938_d_n6, assign44490_e59938_d_n7, assign44490_e59938_d_n8, assign44490_e59938_d_n9, assign44490_e59938_d_n10, assign44490_e59938_d_n11, assign44490_e59938_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign44490_e59932: f64 = (4.0 * locals.var_t4);
        let assign44490_e59935: f64 = (0.01 * 0.01);
        let assign44490_e59936: f64 = (assign44490_e59932 * assign44490_e59935);
        (assign44490_e59936, ((4.0 * locals.var_t4_dn0) * assign44490_e59935), ((4.0 * locals.var_t4_dn2) * assign44490_e59935), ((4.0 * locals.var_t4_dn4) * assign44490_e59935), ((4.0 * locals.var_t4_dn5) * assign44490_e59935), ((4.0 * locals.var_t4_dn6) * assign44490_e59935), ((4.0 * locals.var_t4_dn7) * assign44490_e59935), ((4.0 * locals.var_t4_dn8) * assign44490_e59935), ((4.0 * locals.var_t4_dn9) * assign44490_e59935), ((4.0 * locals.var_t4_dn10) * assign44490_e59935), ((4.0 * locals.var_t4_dn11) * assign44490_e59935), ((4.0 * locals.var_t4_dn14) * assign44490_e59935),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign44490_e59938;
        locals.var_tmf2_dn0 = assign44490_e59938_d_n0;
        locals.var_tmf2_dn2 = assign44490_e59938_d_n2;
        locals.var_tmf2_dn4 = assign44490_e59938_d_n4;
        locals.var_tmf2_dn5 = assign44490_e59938_d_n5;
        locals.var_tmf2_dn6 = assign44490_e59938_d_n6;
        locals.var_tmf2_dn7 = assign44490_e59938_d_n7;
        locals.var_tmf2_dn8 = assign44490_e59938_d_n8;
        locals.var_tmf2_dn9 = assign44490_e59938_d_n9;
        locals.var_tmf2_dn10 = assign44490_e59938_d_n10;
        locals.var_tmf2_dn11 = assign44490_e59938_d_n11;
        locals.var_tmf2_dn14 = assign44490_e59938_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign44500_e59957, assign44500_e59957_d_n0, assign44500_e59957_d_n2, assign44500_e59957_d_n4, assign44500_e59957_d_n5, assign44500_e59957_d_n6, assign44500_e59957_d_n7, assign44500_e59957_d_n8, assign44500_e59957_d_n9, assign44500_e59957_d_n10, assign44500_e59957_d_n11, assign44500_e59957_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let (assign44500_e59955, assign44500_e59955_d_n0, assign44500_e59955_d_n2, assign44500_e59955_d_n4, assign44500_e59955_d_n5, assign44500_e59955_d_n6, assign44500_e59955_d_n7, assign44500_e59955_d_n8, assign44500_e59955_d_n9, assign44500_e59955_d_n10, assign44500_e59955_d_n11, assign44500_e59955_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign44500_e59954: f64 = (-locals.var_tmf2);
                (assign44500_e59954, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign44500_e59955, assign44500_e59955_d_n0, assign44500_e59955_d_n2, assign44500_e59955_d_n4, assign44500_e59955_d_n5, assign44500_e59955_d_n6, assign44500_e59955_d_n7, assign44500_e59955_d_n8, assign44500_e59955_d_n9, assign44500_e59955_d_n10, assign44500_e59955_d_n11, assign44500_e59955_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign44500_e59957;
        locals.var_tmf2_dn0 = assign44500_e59957_d_n0;
        locals.var_tmf2_dn2 = assign44500_e59957_d_n2;
        locals.var_tmf2_dn4 = assign44500_e59957_d_n4;
        locals.var_tmf2_dn5 = assign44500_e59957_d_n5;
        locals.var_tmf2_dn6 = assign44500_e59957_d_n6;
        locals.var_tmf2_dn7 = assign44500_e59957_d_n7;
        locals.var_tmf2_dn8 = assign44500_e59957_d_n8;
        locals.var_tmf2_dn9 = assign44500_e59957_d_n9;
        locals.var_tmf2_dn10 = assign44500_e59957_d_n10;
        locals.var_tmf2_dn11 = assign44500_e59957_d_n11;
        locals.var_tmf2_dn14 = assign44500_e59957_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign44510_e59975, assign44510_e59975_d_n0, assign44510_e59975_d_n2, assign44510_e59975_d_n4, assign44510_e59975_d_n5, assign44510_e59975_d_n6, assign44510_e59975_d_n7, assign44510_e59975_d_n8, assign44510_e59975_d_n9, assign44510_e59975_d_n10, assign44510_e59975_d_n11, assign44510_e59975_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign44510_e59970: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign44510_e59972: f64 = (assign44510_e59970 + locals.var_tmf2);
        let assign44510_e59973: f64 = (assign44510_e59972).sqrt();
        (assign44510_e59973, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign44510_e59973)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign44510_e59973)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign44510_e59973)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign44510_e59973)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign44510_e59973)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign44510_e59973)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign44510_e59973)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign44510_e59973)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign44510_e59973)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign44510_e59973)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign44510_e59973)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign44510_e59975;
        locals.var_tmf2_dn0 = assign44510_e59975_d_n0;
        locals.var_tmf2_dn2 = assign44510_e59975_d_n2;
        locals.var_tmf2_dn4 = assign44510_e59975_d_n4;
        locals.var_tmf2_dn5 = assign44510_e59975_d_n5;
        locals.var_tmf2_dn6 = assign44510_e59975_d_n6;
        locals.var_tmf2_dn7 = assign44510_e59975_d_n7;
        locals.var_tmf2_dn8 = assign44510_e59975_d_n8;
        locals.var_tmf2_dn9 = assign44510_e59975_d_n9;
        locals.var_tmf2_dn10 = assign44510_e59975_d_n10;
        locals.var_tmf2_dn11 = assign44510_e59975_d_n11;
        locals.var_tmf2_dn14 = assign44510_e59975_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign44520_e59994, assign44520_e59994_d_n0, assign44520_e59994_d_n2, assign44520_e59994_d_n4, assign44520_e59994_d_n5, assign44520_e59994_d_n6, assign44520_e59994_d_n7, assign44520_e59994_d_n8, assign44520_e59994_d_n9, assign44520_e59994_d_n10, assign44520_e59994_d_n11, assign44520_e59994_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign44520_e59990: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign44520_e59991: f64 = (1.0 + assign44520_e59990);
        let assign44520_e59992: f64 = (0.5 * assign44520_e59991);
        (assign44520_e59992, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign44520_e59994;
        locals.var_t0_dn0 = assign44520_e59994_d_n0;
        locals.var_t0_dn2 = assign44520_e59994_d_n2;
        locals.var_t0_dn4 = assign44520_e59994_d_n4;
        locals.var_t0_dn5 = assign44520_e59994_d_n5;
        locals.var_t0_dn6 = assign44520_e59994_d_n6;
        locals.var_t0_dn7 = assign44520_e59994_d_n7;
        locals.var_t0_dn8 = assign44520_e59994_d_n8;
        locals.var_t0_dn9 = assign44520_e59994_d_n9;
        locals.var_t0_dn10 = assign44520_e59994_d_n10;
        locals.var_t0_dn11 = assign44520_e59994_d_n11;
        locals.var_t0_dn14 = assign44520_e59994_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign44530_e60013, assign44530_e60013_d_n0, assign44530_e60013_d_n2, assign44530_e60013_d_n4, assign44530_e60013_d_n5, assign44530_e60013_d_n6, assign44530_e60013_d_n7, assign44530_e60013_d_n8, assign44530_e60013_d_n9, assign44530_e60013_d_n10, assign44530_e60013_d_n11, assign44530_e60013_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign44530_e60009: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign44530_e60010: f64 = (0.5 * assign44530_e60009);
        let assign44530_e60011: f64 = (locals.var_t4 + assign44530_e60010);
        (assign44530_e60011, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign44530_e60013;
        locals.var_t2_dn0 = assign44530_e60013_d_n0;
        locals.var_t2_dn2 = assign44530_e60013_d_n2;
        locals.var_t2_dn4 = assign44530_e60013_d_n4;
        locals.var_t2_dn5 = assign44530_e60013_d_n5;
        locals.var_t2_dn6 = assign44530_e60013_d_n6;
        locals.var_t2_dn7 = assign44530_e60013_d_n7;
        locals.var_t2_dn8 = assign44530_e60013_d_n8;
        locals.var_t2_dn9 = assign44530_e60013_d_n9;
        locals.var_t2_dn10 = assign44530_e60013_d_n10;
        locals.var_t2_dn11 = assign44530_e60013_d_n11;
        locals.var_t2_dn14 = assign44530_e60013_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign44540_e60030, assign44540_e60030_d_n0, assign44540_e60030_d_n2, assign44540_e60030_d_n4, assign44540_e60030_d_n5, assign44540_e60030_d_n6, assign44540_e60030_d_n7, assign44540_e60030_d_n8, assign44540_e60030_d_n9, assign44540_e60030_d_n10, assign44540_e60030_d_n11, assign44540_e60030_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign44540_e60027: f64 = (p.p296 + 1.0);
        let assign44540_e60028: f64 = (locals.var_t4 * assign44540_e60027);
        (assign44540_e60028, (locals.var_t4_dn0 * assign44540_e60027), (locals.var_t4_dn2 * assign44540_e60027), (locals.var_t4_dn4 * assign44540_e60027), (locals.var_t4_dn5 * assign44540_e60027), (locals.var_t4_dn6 * assign44540_e60027), (locals.var_t4_dn7 * assign44540_e60027), (locals.var_t4_dn8 * assign44540_e60027), (locals.var_t4_dn9 * assign44540_e60027), (locals.var_t4_dn10 * assign44540_e60027), (locals.var_t4_dn11 * assign44540_e60027), (locals.var_t4_dn14 * assign44540_e60027),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign44540_e60030;
        locals.var_t3_dn0 = assign44540_e60030_d_n0;
        locals.var_t3_dn2 = assign44540_e60030_d_n2;
        locals.var_t3_dn4 = assign44540_e60030_d_n4;
        locals.var_t3_dn5 = assign44540_e60030_d_n5;
        locals.var_t3_dn6 = assign44540_e60030_d_n6;
        locals.var_t3_dn7 = assign44540_e60030_d_n7;
        locals.var_t3_dn8 = assign44540_e60030_d_n8;
        locals.var_t3_dn9 = assign44540_e60030_d_n9;
        locals.var_t3_dn10 = assign44540_e60030_d_n10;
        locals.var_t3_dn11 = assign44540_e60030_d_n11;
        locals.var_t3_dn14 = assign44540_e60030_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign44550_e60049, assign44550_e60049_d_n0, assign44550_e60049_d_n2, assign44550_e60049_d_n4, assign44550_e60049_d_n5, assign44550_e60049_d_n6, assign44550_e60049_d_n7, assign44550_e60049_d_n8, assign44550_e60049_d_n9, assign44550_e60049_d_n10, assign44550_e60049_d_n11, assign44550_e60049_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard447 != 0.0) && (locals.var_guard446 == 0.0))) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign44550_e60043: f64 = (locals.var_t3 - locals.var_t2);
        let assign44550_e60046: f64 = (0.01 * 0.01);
        let assign44550_e60047: f64 = (assign44550_e60043 - assign44550_e60046);
        (assign44550_e60047, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn11 - locals.var_t2_dn11), (locals.var_t3_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign44550_e60049;
        locals.var_tmf1_dn0 = assign44550_e60049_d_n0;
        locals.var_tmf1_dn2 = assign44550_e60049_d_n2;
        locals.var_tmf1_dn4 = assign44550_e60049_d_n4;
        locals.var_tmf1_dn5 = assign44550_e60049_d_n5;
        locals.var_tmf1_dn6 = assign44550_e60049_d_n6;
        locals.var_tmf1_dn7 = assign44550_e60049_d_n7;
        locals.var_tmf1_dn8 = assign44550_e60049_d_n8;
        locals.var_tmf1_dn9 = assign44550_e60049_d_n9;
        locals.var_tmf1_dn10 = assign44550_e60049_d_n10;
        locals.var_tmf1_dn11 = assign44550_e60049_d_n11;
        locals.var_tmf1_dn14 = assign44550_e60049_d_n14;
        locals.var_tmf1_rv = 0.0;

    }
}
