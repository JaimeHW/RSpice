#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_47(
        locals: &mut StampLocals,
    ) {
        let (assign13560_e19242,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13560_e19242;
        locals.var_m0_rv = 0.0;

        let (assign13570_e19251,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13570_e19251;
        locals.var_mm_rv = 0.0;

        let (assign13580_e19260, assign13580_e19260_d_n0, assign13580_e19260_d_n2, assign13580_e19260_d_n6, assign13580_e19260_d_n7, assign13580_e19260_d_n10, assign13580_e19260_d_n11, assign13580_e19260_d_n12, assign13580_e19260_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13580_e19260;
        locals.var_arg_dn0 = assign13580_e19260_d_n0;
        locals.var_arg_dn2 = assign13580_e19260_d_n2;
        locals.var_arg_dn6 = assign13580_e19260_d_n6;
        locals.var_arg_dn7 = assign13580_e19260_d_n7;
        locals.var_arg_dn10 = assign13580_e19260_d_n10;
        locals.var_arg_dn11 = assign13580_e19260_d_n11;
        locals.var_arg_dn12 = assign13580_e19260_d_n12;
        locals.var_arg_dn17 = assign13580_e19260_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign13590_e19269, assign13590_e19269_d_n0, assign13590_e19269_d_n2, assign13590_e19269_d_n6, assign13590_e19269_d_n7, assign13590_e19269_d_n10, assign13590_e19269_d_n11, assign13590_e19269_d_n12, assign13590_e19269_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13590_e19269;
        locals.var_dnm_dn0 = assign13590_e19269_d_n0;
        locals.var_dnm_dn2 = assign13590_e19269_d_n2;
        locals.var_dnm_dn6 = assign13590_e19269_d_n6;
        locals.var_dnm_dn7 = assign13590_e19269_d_n7;
        locals.var_dnm_dn10 = assign13590_e19269_d_n10;
        locals.var_dnm_dn11 = assign13590_e19269_d_n11;
        locals.var_dnm_dn12 = assign13590_e19269_d_n12;
        locals.var_dnm_dn17 = assign13590_e19269_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign13600_e19280, assign13600_e19280_d_n0, assign13600_e19280_d_n2, assign13600_e19280_d_n6, assign13600_e19280_d_n7, assign13600_e19280_d_n10, assign13600_e19280_d_n11, assign13600_e19280_d_n12, assign13600_e19280_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13600_e19278: f64 = (locals.var_xp * locals.var_x2);
        (assign13600_e19278, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13600_e19280;
        locals.var_xp_dn0 = assign13600_e19280_d_n0;
        locals.var_xp_dn2 = assign13600_e19280_d_n2;
        locals.var_xp_dn6 = assign13600_e19280_d_n6;
        locals.var_xp_dn7 = assign13600_e19280_d_n7;
        locals.var_xp_dn10 = assign13600_e19280_d_n10;
        locals.var_xp_dn11 = assign13600_e19280_d_n11;
        locals.var_xp_dn12 = assign13600_e19280_d_n12;
        locals.var_xp_dn17 = assign13600_e19280_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign13610_e19291, assign13610_e19291_d_n0, assign13610_e19291_d_n2, assign13610_e19291_d_n6, assign13610_e19291_d_n7, assign13610_e19291_d_n10, assign13610_e19291_d_n11, assign13610_e19291_d_n12, assign13610_e19291_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13610_e19289: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13610_e19289, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13610_e19291;
        locals.var_xmp_dn0 = assign13610_e19291_d_n0;
        locals.var_xmp_dn2 = assign13610_e19291_d_n2;
        locals.var_xmp_dn6 = assign13610_e19291_d_n6;
        locals.var_xmp_dn7 = assign13610_e19291_d_n7;
        locals.var_xmp_dn10 = assign13610_e19291_d_n10;
        locals.var_xmp_dn11 = assign13610_e19291_d_n11;
        locals.var_xmp_dn12 = assign13610_e19291_d_n12;
        locals.var_xmp_dn17 = assign13610_e19291_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign13620_e19302, assign13620_e19302_d_n0, assign13620_e19302_d_n2, assign13620_e19302_d_n6, assign13620_e19302_d_n7, assign13620_e19302_d_n10, assign13620_e19302_d_n11, assign13620_e19302_d_n12, assign13620_e19302_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13620_e19300: f64 = (locals.var_xp * locals.var_x2);
        (assign13620_e19300, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13620_e19302;
        locals.var_xp_dn0 = assign13620_e19302_d_n0;
        locals.var_xp_dn2 = assign13620_e19302_d_n2;
        locals.var_xp_dn6 = assign13620_e19302_d_n6;
        locals.var_xp_dn7 = assign13620_e19302_d_n7;
        locals.var_xp_dn10 = assign13620_e19302_d_n10;
        locals.var_xp_dn11 = assign13620_e19302_d_n11;
        locals.var_xp_dn12 = assign13620_e19302_d_n12;
        locals.var_xp_dn17 = assign13620_e19302_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign13630_e19313, assign13630_e19313_d_n0, assign13630_e19313_d_n2, assign13630_e19313_d_n6, assign13630_e19313_d_n7, assign13630_e19313_d_n10, assign13630_e19313_d_n11, assign13630_e19313_d_n12, assign13630_e19313_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13630_e19311: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13630_e19311, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13630_e19313;
        locals.var_xmp_dn0 = assign13630_e19313_d_n0;
        locals.var_xmp_dn2 = assign13630_e19313_d_n2;
        locals.var_xmp_dn6 = assign13630_e19313_d_n6;
        locals.var_xmp_dn7 = assign13630_e19313_d_n7;
        locals.var_xmp_dn10 = assign13630_e19313_d_n10;
        locals.var_xmp_dn11 = assign13630_e19313_d_n11;
        locals.var_xmp_dn12 = assign13630_e19313_d_n12;
        locals.var_xmp_dn17 = assign13630_e19313_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign13640_e19324, assign13640_e19324_d_n0, assign13640_e19324_d_n2, assign13640_e19324_d_n6, assign13640_e19324_d_n7, assign13640_e19324_d_n10, assign13640_e19324_d_n11, assign13640_e19324_d_n12, assign13640_e19324_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13640_e19322: f64 = (locals.var_xp + locals.var_xmp);
        (assign13640_e19322, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13640_e19324;
        locals.var_arg_dn0 = assign13640_e19324_d_n0;
        locals.var_arg_dn2 = assign13640_e19324_d_n2;
        locals.var_arg_dn6 = assign13640_e19324_d_n6;
        locals.var_arg_dn7 = assign13640_e19324_d_n7;
        locals.var_arg_dn10 = assign13640_e19324_d_n10;
        locals.var_arg_dn11 = assign13640_e19324_d_n11;
        locals.var_arg_dn12 = assign13640_e19324_d_n12;
        locals.var_arg_dn17 = assign13640_e19324_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign13650_e19333, assign13650_e19333_d_n0, assign13650_e19333_d_n2, assign13650_e19333_d_n6, assign13650_e19333_d_n7, assign13650_e19333_d_n10, assign13650_e19333_d_n11, assign13650_e19333_d_n12, assign13650_e19333_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13650_e19333;
        locals.var_dnm_dn0 = assign13650_e19333_d_n0;
        locals.var_dnm_dn2 = assign13650_e19333_d_n2;
        locals.var_dnm_dn6 = assign13650_e19333_d_n6;
        locals.var_dnm_dn7 = assign13650_e19333_d_n7;
        locals.var_dnm_dn10 = assign13650_e19333_d_n10;
        locals.var_dnm_dn11 = assign13650_e19333_d_n11;
        locals.var_dnm_dn12 = assign13650_e19333_d_n12;
        locals.var_dnm_dn17 = assign13650_e19333_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign13660_e19348: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard424 = assign13660_e19348;
        locals.var_guard424_rv = 0.0;

        let assign13670_e19351: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard425 = assign13670_e19351;
        locals.var_guard425_rv = 0.0;

        let (assign13680_e19364,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) && (locals.var_guard425 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13680_e19364;
        locals.var_mm_rv = 0.0;

        let assign13690_e19367: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard426 = assign13690_e19367;
        locals.var_guard426_rv = 0.0;

        let (assign13700_e19383,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard426 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13700_e19383;
        locals.var_mm_rv = 0.0;

        let assign13710_e19386: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard427 = assign13710_e19386;
        locals.var_guard427_rv = 0.0;

        let (assign13720_e19405,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard426 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13720_e19405;
        locals.var_mm_rv = 0.0;

        let assign13730_e19408: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard428 = assign13730_e19408;
        locals.var_guard428_rv = 0.0;

        let (assign13740_e19430,) = {
    if ((((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard426 == 0.0)) && (locals.var_guard427 == 0.0)) && (locals.var_guard428 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13740_e19430;
        locals.var_mm_rv = 0.0;

        let (assign13750_e19441,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13750_e19441;
        locals.var_m0_rv = 0.0;

        let mut assign13760_loop_guard: usize = 0;
        while {
            let assign13760_cond_e19453: f64 = if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign13760_cond_e19453 != 0.0
        } {
            assign13760_loop_guard += 1;
            assert!(assign13760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign13760_body0_e19465, assign13760_body0_e19465_d_n0, assign13760_body0_e19465_d_n2, assign13760_body0_e19465_d_n6, assign13760_body0_e19465_d_n7, assign13760_body0_e19465_d_n10, assign13760_body0_e19465_d_n11, assign13760_body0_e19465_d_n12, assign13760_body0_e19465_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) {
        let assign13760_body0_e19463: f64 = (locals.var_dnm).sqrt();
        (assign13760_body0_e19463, (locals.var_dnm_dn0 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn2 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn6 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn7 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn10 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn11 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn12 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn17 / (2.0 * assign13760_body0_e19463)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign13760_body0_e19465;
            locals.var_dnm_dn0 = assign13760_body0_e19465_d_n0;
            locals.var_dnm_dn2 = assign13760_body0_e19465_d_n2;
            locals.var_dnm_dn6 = assign13760_body0_e19465_d_n6;
            locals.var_dnm_dn7 = assign13760_body0_e19465_d_n7;
            locals.var_dnm_dn10 = assign13760_body0_e19465_d_n10;
            locals.var_dnm_dn11 = assign13760_body0_e19465_d_n11;
            locals.var_dnm_dn12 = assign13760_body0_e19465_d_n12;
            locals.var_dnm_dn17 = assign13760_body0_e19465_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign13760_body1_e19478,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) {
        let assign13760_body1_e19476: f64 = (locals.var_m0 + 1.0);
        (assign13760_body1_e19476,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign13760_body1_e19478;
            locals.var_m0_rv = 0.0;
        }

        let (assign13770_e19496, assign13770_e19496_d_n0, assign13770_e19496_d_n2, assign13770_e19496_d_n6, assign13770_e19496_d_n7, assign13770_e19496_d_n10, assign13770_e19496_d_n11, assign13770_e19496_d_n12, assign13770_e19496_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 == 0.0)) {
        let assign13770_e19492: f64 = (2.0 * 2.0);
        let assign13770_e19493: f64 = (1.0 / assign13770_e19492);
        let assign13770_e19494: f64 = (locals.var_dnm).powf(assign13770_e19493);
        (assign13770_e19494, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn0)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn2)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn6)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn7)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn10)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn11)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn12)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn17)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13770_e19496;
        locals.var_dnm_dn0 = assign13770_e19496_d_n0;
        locals.var_dnm_dn2 = assign13770_e19496_d_n2;
        locals.var_dnm_dn6 = assign13770_e19496_d_n6;
        locals.var_dnm_dn7 = assign13770_e19496_d_n7;
        locals.var_dnm_dn10 = assign13770_e19496_d_n10;
        locals.var_dnm_dn11 = assign13770_e19496_d_n11;
        locals.var_dnm_dn12 = assign13770_e19496_d_n12;
        locals.var_dnm_dn17 = assign13770_e19496_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign13780_e19507, assign13780_e19507_d_n0, assign13780_e19507_d_n2, assign13780_e19507_d_n6, assign13780_e19507_d_n7, assign13780_e19507_d_n10, assign13780_e19507_d_n11, assign13780_e19507_d_n12, assign13780_e19507_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13780_e19505: f64 = (1.0 / locals.var_dnm);
        (assign13780_e19505, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13780_e19507;
        locals.var_dnm_dn0 = assign13780_e19507_d_n0;
        locals.var_dnm_dn2 = assign13780_e19507_d_n2;
        locals.var_dnm_dn6 = assign13780_e19507_d_n6;
        locals.var_dnm_dn7 = assign13780_e19507_d_n7;
        locals.var_dnm_dn10 = assign13780_e19507_d_n10;
        locals.var_dnm_dn11 = assign13780_e19507_d_n11;
        locals.var_dnm_dn12 = assign13780_e19507_d_n12;
        locals.var_dnm_dn17 = assign13780_e19507_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign13790_e19522, assign13790_e19522_d_n0, assign13790_e19522_d_n2, assign13790_e19522_d_n6, assign13790_e19522_d_n7, assign13790_e19522_d_n10, assign13790_e19522_d_n11, assign13790_e19522_d_n12, assign13790_e19522_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13790_e19517: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13790_e19518: f64 = (locals.var_tmf1 * assign13790_e19517);
        let assign13790_e19520: f64 = (assign13790_e19518 * locals.var_dnm);
        (assign13790_e19520, ((((locals.var_tmf1_dn0 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn0 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn2 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn6 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn7 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn10 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn11 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn12 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn17 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign13790_e19522;
        locals.var_tmf0_dn0 = assign13790_e19522_d_n0;
        locals.var_tmf0_dn2 = assign13790_e19522_d_n2;
        locals.var_tmf0_dn6 = assign13790_e19522_d_n6;
        locals.var_tmf0_dn7 = assign13790_e19522_d_n7;
        locals.var_tmf0_dn10 = assign13790_e19522_d_n10;
        locals.var_tmf0_dn11 = assign13790_e19522_d_n11;
        locals.var_tmf0_dn12 = assign13790_e19522_d_n12;
        locals.var_tmf0_dn17 = assign13790_e19522_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign13800_e19537, assign13800_e19537_d_n0, assign13800_e19537_d_n2, assign13800_e19537_d_n6, assign13800_e19537_d_n7, assign13800_e19537_d_n10, assign13800_e19537_d_n11, assign13800_e19537_d_n12, assign13800_e19537_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13800_e19532: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13800_e19533: f64 = assign13800_e19532;
        let assign13800_e19535: f64 = (assign13800_e19533 - locals.var_tmf0);
        (assign13800_e19535, ((locals.var_q_fd_soi_dn0 * 1e-5) - locals.var_tmf0_dn0), ((locals.var_q_fd_soi_dn2 * 1e-5) - locals.var_tmf0_dn2), ((locals.var_q_fd_soi_dn6 * 1e-5) - locals.var_tmf0_dn6), ((locals.var_q_fd_soi_dn7 * 1e-5) - locals.var_tmf0_dn7), ((locals.var_q_fd_soi_dn10 * 1e-5) - locals.var_tmf0_dn10), ((locals.var_q_fd_soi_dn11 * 1e-5) - locals.var_tmf0_dn11), ((locals.var_q_fd_soi_dn12 * 1e-5) - locals.var_tmf0_dn12), ((locals.var_q_fd_soi_dn17 * 1e-5) - locals.var_tmf0_dn17),)
    } else {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    }
};
        locals.var_rrr_cc = assign13800_e19537;
        locals.var_rrr_cc_dn0 = assign13800_e19537_d_n0;
        locals.var_rrr_cc_dn2 = assign13800_e19537_d_n2;
        locals.var_rrr_cc_dn6 = assign13800_e19537_d_n6;
        locals.var_rrr_cc_dn7 = assign13800_e19537_d_n7;
        locals.var_rrr_cc_dn10 = assign13800_e19537_d_n10;
        locals.var_rrr_cc_dn11 = assign13800_e19537_d_n11;
        locals.var_rrr_cc_dn12 = assign13800_e19537_d_n12;
        locals.var_rrr_cc_dn17 = assign13800_e19537_d_n17;
        locals.var_rrr_cc_rv = 0.0;

        let (assign13810_e19547, assign13810_e19547_d_n0, assign13810_e19547_d_n2, assign13810_e19547_d_n6, assign13810_e19547_d_n7, assign13810_e19547_d_n10, assign13810_e19547_d_n11, assign13810_e19547_d_n12, assign13810_e19547_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 == 0.0)) {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    } else {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    }
};
        locals.var_rrr_cc = assign13810_e19547;
        locals.var_rrr_cc_dn0 = assign13810_e19547_d_n0;
        locals.var_rrr_cc_dn2 = assign13810_e19547_d_n2;
        locals.var_rrr_cc_dn6 = assign13810_e19547_d_n6;
        locals.var_rrr_cc_dn7 = assign13810_e19547_d_n7;
        locals.var_rrr_cc_dn10 = assign13810_e19547_d_n10;
        locals.var_rrr_cc_dn11 = assign13810_e19547_d_n11;
        locals.var_rrr_cc_dn12 = assign13810_e19547_d_n12;
        locals.var_rrr_cc_dn17 = assign13810_e19547_d_n17;
        locals.var_rrr_cc_rv = 0.0;

        let (assign13820_e19567, assign13820_e19567_d_n0, assign13820_e19567_d_n2, assign13820_e19567_d_n6, assign13820_e19567_d_n7, assign13820_e19567_d_n10, assign13820_e19567_d_n11, assign13820_e19567_d_n12, assign13820_e19567_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13820_e19555: f64 = (-locals.var_rrr_cc);
        let assign13820_e19556: f64 = (2.0 * assign13820_e19555);
        let assign13820_e19559: f64 = (locals.var_beta * locals.var_c_fox);
        let assign13820_e19561: f64 = (assign13820_e19559 * locals.var_rrr_p0);
        let assign13820_e19563: f64 = (assign13820_e19561 * locals.var_rrr_p0);
        let assign13820_e19564: f64 = (assign13820_e19556 / assign13820_e19563);
        let assign13820_e19565: f64 = (1.0 + assign13820_e19564);
        (assign13820_e19565, ((((2.0 * (-locals.var_rrr_cc_dn0)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn0) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn0)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn2)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn2) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn2)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn6)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn6) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn6)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn7)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn7) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn7)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn10)) * assign13820_e19563) - (assign13820_e19556 * ((((((locals.var_beta_dn10 * locals.var_c_fox) + (locals.var_beta * locals.var_c_fox_dn10)) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn10)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn11)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn11) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn11)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn12)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn12) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn12)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn17)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn17) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn17)))) / (assign13820_e19563 * assign13820_e19563)),)
    } else {
        (locals.var_rrr_alpha_soi, locals.var_rrr_alpha_soi_dn0, locals.var_rrr_alpha_soi_dn2, locals.var_rrr_alpha_soi_dn6, locals.var_rrr_alpha_soi_dn7, locals.var_rrr_alpha_soi_dn10, locals.var_rrr_alpha_soi_dn11, locals.var_rrr_alpha_soi_dn12, locals.var_rrr_alpha_soi_dn17,)
    }
};
        locals.var_rrr_alpha_soi = assign13820_e19567;
        locals.var_rrr_alpha_soi_dn0 = assign13820_e19567_d_n0;
        locals.var_rrr_alpha_soi_dn2 = assign13820_e19567_d_n2;
        locals.var_rrr_alpha_soi_dn6 = assign13820_e19567_d_n6;
        locals.var_rrr_alpha_soi_dn7 = assign13820_e19567_d_n7;
        locals.var_rrr_alpha_soi_dn10 = assign13820_e19567_d_n10;
        locals.var_rrr_alpha_soi_dn11 = assign13820_e19567_d_n11;
        locals.var_rrr_alpha_soi_dn12 = assign13820_e19567_d_n12;
        locals.var_rrr_alpha_soi_dn17 = assign13820_e19567_d_n17;
        locals.var_rrr_alpha_soi_rv = 0.0;

        let (assign13830_e19580, assign13830_e19580_d_n0, assign13830_e19580_d_n2, assign13830_e19580_d_n6, assign13830_e19580_d_n7, assign13830_e19580_d_n10, assign13830_e19580_d_n11, assign13830_e19580_d_n12, assign13830_e19580_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13830_e19574: f64 = (locals.var_rrr_p0 * locals.var_rrr_p0);
        let assign13830_e19576: f64 = (assign13830_e19574 * locals.var_rrr_p0);
        let assign13830_e19578: f64 = (assign13830_e19576 * locals.var_rrr_p0);
        (assign13830_e19578, ((((((locals.var_rrr_p0_dn0 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn0)), ((((((locals.var_rrr_p0_dn2 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn2)), ((((((locals.var_rrr_p0_dn6 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn6)), ((((((locals.var_rrr_p0_dn7 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn7)), ((((((locals.var_rrr_p0_dn10 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn10)), ((((((locals.var_rrr_p0_dn11 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn11)), ((((((locals.var_rrr_p0_dn12 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn12)), ((((((locals.var_rrr_p0_dn17 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13830_e19580;
        locals.var_t1_dn0 = assign13830_e19580_d_n0;
        locals.var_t1_dn2 = assign13830_e19580_d_n2;
        locals.var_t1_dn6 = assign13830_e19580_d_n6;
        locals.var_t1_dn7 = assign13830_e19580_d_n7;
        locals.var_t1_dn10 = assign13830_e19580_d_n10;
        locals.var_t1_dn11 = assign13830_e19580_d_n11;
        locals.var_t1_dn12 = assign13830_e19580_d_n12;
        locals.var_t1_dn17 = assign13830_e19580_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign13840_e19589, assign13840_e19589_d_n0, assign13840_e19589_d_n2, assign13840_e19589_d_n6, assign13840_e19589_d_n7, assign13840_e19589_d_n10, assign13840_e19589_d_n11, assign13840_e19589_d_n12, assign13840_e19589_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13840_e19587: f64 = (locals.var_rrr_alpha_soi * locals.var_rrr_p0);
        (assign13840_e19587, ((locals.var_rrr_alpha_soi_dn0 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn0)), ((locals.var_rrr_alpha_soi_dn2 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn2)), ((locals.var_rrr_alpha_soi_dn6 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn6)), ((locals.var_rrr_alpha_soi_dn7 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn7)), ((locals.var_rrr_alpha_soi_dn10 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn10)), ((locals.var_rrr_alpha_soi_dn11 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn11)), ((locals.var_rrr_alpha_soi_dn12 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn12)), ((locals.var_rrr_alpha_soi_dn17 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn17)),)
    } else {
        (locals.var_rrr_dd, locals.var_rrr_dd_dn0, locals.var_rrr_dd_dn2, locals.var_rrr_dd_dn6, locals.var_rrr_dd_dn7, locals.var_rrr_dd_dn10, locals.var_rrr_dd_dn11, locals.var_rrr_dd_dn12, locals.var_rrr_dd_dn17,)
    }
};
        locals.var_rrr_dd = assign13840_e19589;
        locals.var_rrr_dd_dn0 = assign13840_e19589_d_n0;
        locals.var_rrr_dd_dn2 = assign13840_e19589_d_n2;
        locals.var_rrr_dd_dn6 = assign13840_e19589_d_n6;
        locals.var_rrr_dd_dn7 = assign13840_e19589_d_n7;
        locals.var_rrr_dd_dn10 = assign13840_e19589_d_n10;
        locals.var_rrr_dd_dn11 = assign13840_e19589_d_n11;
        locals.var_rrr_dd_dn12 = assign13840_e19589_d_n12;
        locals.var_rrr_dd_dn17 = assign13840_e19589_d_n17;
        locals.var_rrr_dd_rv = 0.0;

        let (assign13850_e19600, assign13850_e19600_d_n0, assign13850_e19600_d_n2, assign13850_e19600_d_n6, assign13850_e19600_d_n7, assign13850_e19600_d_n10, assign13850_e19600_d_n11, assign13850_e19600_d_n12, assign13850_e19600_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13850_e19597: f64 = (locals.var_rrr_dd / locals.var_vgvt);
        let assign13850_e19598: f64 = (1.0 - assign13850_e19597);
        (assign13850_e19598, (-(((locals.var_rrr_dd_dn0 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn0)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn2 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn2)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn6 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn6)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn7 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn7)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn10 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn10)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn11 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn11)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn12 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn12)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn17 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn17)) / (locals.var_vgvt * locals.var_vgvt))),)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign13850_e19600;
        locals.var_rrr_eta_dn0 = assign13850_e19600_d_n0;
        locals.var_rrr_eta_dn2 = assign13850_e19600_d_n2;
        locals.var_rrr_eta_dn6 = assign13850_e19600_d_n6;
        locals.var_rrr_eta_dn7 = assign13850_e19600_d_n7;
        locals.var_rrr_eta_dn10 = assign13850_e19600_d_n10;
        locals.var_rrr_eta_dn11 = assign13850_e19600_d_n11;
        locals.var_rrr_eta_dn12 = assign13850_e19600_d_n12;
        locals.var_rrr_eta_dn17 = assign13850_e19600_d_n17;
        locals.var_rrr_eta_rv = 0.0;

        let assign13860_e19604: f64 = 1e-5;
        let assign13860_e19609: f64 = if ((locals.var_rrr_eta < assign13860_e19604) && (1e-5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard429 = assign13860_e19609;
        locals.var_guard429_rv = 0.0;

        let (assign13870_e19622, assign13870_e19622_d_n0, assign13870_e19622_d_n2, assign13870_e19622_d_n6, assign13870_e19622_d_n7, assign13870_e19622_d_n10, assign13870_e19622_d_n11, assign13870_e19622_d_n12, assign13870_e19622_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13870_e19618: f64 = 1e-5;
        let assign13870_e19620: f64 = (assign13870_e19618 - locals.var_rrr_eta);
        (assign13870_e19620, (-locals.var_rrr_eta_dn0), (-locals.var_rrr_eta_dn2), (-locals.var_rrr_eta_dn6), (-locals.var_rrr_eta_dn7), (-locals.var_rrr_eta_dn10), (-locals.var_rrr_eta_dn11), (-locals.var_rrr_eta_dn12), (-locals.var_rrr_eta_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign13870_e19622;
        locals.var_tmf1_dn0 = assign13870_e19622_d_n0;
        locals.var_tmf1_dn2 = assign13870_e19622_d_n2;
        locals.var_tmf1_dn6 = assign13870_e19622_d_n6;
        locals.var_tmf1_dn7 = assign13870_e19622_d_n7;
        locals.var_tmf1_dn10 = assign13870_e19622_d_n10;
        locals.var_tmf1_dn11 = assign13870_e19622_d_n11;
        locals.var_tmf1_dn12 = assign13870_e19622_d_n12;
        locals.var_tmf1_dn17 = assign13870_e19622_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign13880_e19633, assign13880_e19633_d_n0, assign13880_e19633_d_n2, assign13880_e19633_d_n6, assign13880_e19633_d_n7, assign13880_e19633_d_n10, assign13880_e19633_d_n11, assign13880_e19633_d_n12, assign13880_e19633_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13880_e19631: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign13880_e19631, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign13880_e19633;
        locals.var_x2_dn0 = assign13880_e19633_d_n0;
        locals.var_x2_dn2 = assign13880_e19633_d_n2;
        locals.var_x2_dn6 = assign13880_e19633_d_n6;
        locals.var_x2_dn7 = assign13880_e19633_d_n7;
        locals.var_x2_dn10 = assign13880_e19633_d_n10;
        locals.var_x2_dn11 = assign13880_e19633_d_n11;
        locals.var_x2_dn12 = assign13880_e19633_d_n12;
        locals.var_x2_dn17 = assign13880_e19633_d_n17;
        locals.var_x2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_48(
        locals: &mut StampLocals,
    ) {
        let (assign13890_e19644, assign13890_e19644_d_n0, assign13890_e19644_d_n2, assign13890_e19644_d_n6, assign13890_e19644_d_n7, assign13890_e19644_d_n10, assign13890_e19644_d_n11, assign13890_e19644_d_n12, assign13890_e19644_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13890_e19642: f64 = (1e-5 * 1e-5);
        (assign13890_e19642, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13890_e19644;
        locals.var_xmax2_dn0 = assign13890_e19644_d_n0;
        locals.var_xmax2_dn2 = assign13890_e19644_d_n2;
        locals.var_xmax2_dn6 = assign13890_e19644_d_n6;
        locals.var_xmax2_dn7 = assign13890_e19644_d_n7;
        locals.var_xmax2_dn10 = assign13890_e19644_d_n10;
        locals.var_xmax2_dn11 = assign13890_e19644_d_n11;
        locals.var_xmax2_dn12 = assign13890_e19644_d_n12;
        locals.var_xmax2_dn17 = assign13890_e19644_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign13900_e19653, assign13900_e19653_d_n0, assign13900_e19653_d_n2, assign13900_e19653_d_n6, assign13900_e19653_d_n7, assign13900_e19653_d_n10, assign13900_e19653_d_n11, assign13900_e19653_d_n12, assign13900_e19653_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13900_e19653;
        locals.var_xp_dn0 = assign13900_e19653_d_n0;
        locals.var_xp_dn2 = assign13900_e19653_d_n2;
        locals.var_xp_dn6 = assign13900_e19653_d_n6;
        locals.var_xp_dn7 = assign13900_e19653_d_n7;
        locals.var_xp_dn10 = assign13900_e19653_d_n10;
        locals.var_xp_dn11 = assign13900_e19653_d_n11;
        locals.var_xp_dn12 = assign13900_e19653_d_n12;
        locals.var_xp_dn17 = assign13900_e19653_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign13910_e19662, assign13910_e19662_d_n0, assign13910_e19662_d_n2, assign13910_e19662_d_n6, assign13910_e19662_d_n7, assign13910_e19662_d_n10, assign13910_e19662_d_n11, assign13910_e19662_d_n12, assign13910_e19662_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13910_e19662;
        locals.var_xmp_dn0 = assign13910_e19662_d_n0;
        locals.var_xmp_dn2 = assign13910_e19662_d_n2;
        locals.var_xmp_dn6 = assign13910_e19662_d_n6;
        locals.var_xmp_dn7 = assign13910_e19662_d_n7;
        locals.var_xmp_dn10 = assign13910_e19662_d_n10;
        locals.var_xmp_dn11 = assign13910_e19662_d_n11;
        locals.var_xmp_dn12 = assign13910_e19662_d_n12;
        locals.var_xmp_dn17 = assign13910_e19662_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign13920_e19671,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13920_e19671;
        locals.var_m0_rv = 0.0;

        let (assign13930_e19680,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13930_e19680;
        locals.var_mm_rv = 0.0;

        let (assign13940_e19689, assign13940_e19689_d_n0, assign13940_e19689_d_n2, assign13940_e19689_d_n6, assign13940_e19689_d_n7, assign13940_e19689_d_n10, assign13940_e19689_d_n11, assign13940_e19689_d_n12, assign13940_e19689_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13940_e19689;
        locals.var_arg_dn0 = assign13940_e19689_d_n0;
        locals.var_arg_dn2 = assign13940_e19689_d_n2;
        locals.var_arg_dn6 = assign13940_e19689_d_n6;
        locals.var_arg_dn7 = assign13940_e19689_d_n7;
        locals.var_arg_dn10 = assign13940_e19689_d_n10;
        locals.var_arg_dn11 = assign13940_e19689_d_n11;
        locals.var_arg_dn12 = assign13940_e19689_d_n12;
        locals.var_arg_dn17 = assign13940_e19689_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign13950_e19698, assign13950_e19698_d_n0, assign13950_e19698_d_n2, assign13950_e19698_d_n6, assign13950_e19698_d_n7, assign13950_e19698_d_n10, assign13950_e19698_d_n11, assign13950_e19698_d_n12, assign13950_e19698_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13950_e19698;
        locals.var_dnm_dn0 = assign13950_e19698_d_n0;
        locals.var_dnm_dn2 = assign13950_e19698_d_n2;
        locals.var_dnm_dn6 = assign13950_e19698_d_n6;
        locals.var_dnm_dn7 = assign13950_e19698_d_n7;
        locals.var_dnm_dn10 = assign13950_e19698_d_n10;
        locals.var_dnm_dn11 = assign13950_e19698_d_n11;
        locals.var_dnm_dn12 = assign13950_e19698_d_n12;
        locals.var_dnm_dn17 = assign13950_e19698_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign13960_e19709, assign13960_e19709_d_n0, assign13960_e19709_d_n2, assign13960_e19709_d_n6, assign13960_e19709_d_n7, assign13960_e19709_d_n10, assign13960_e19709_d_n11, assign13960_e19709_d_n12, assign13960_e19709_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13960_e19707: f64 = (locals.var_xp * locals.var_x2);
        (assign13960_e19707, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13960_e19709;
        locals.var_xp_dn0 = assign13960_e19709_d_n0;
        locals.var_xp_dn2 = assign13960_e19709_d_n2;
        locals.var_xp_dn6 = assign13960_e19709_d_n6;
        locals.var_xp_dn7 = assign13960_e19709_d_n7;
        locals.var_xp_dn10 = assign13960_e19709_d_n10;
        locals.var_xp_dn11 = assign13960_e19709_d_n11;
        locals.var_xp_dn12 = assign13960_e19709_d_n12;
        locals.var_xp_dn17 = assign13960_e19709_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign13970_e19720, assign13970_e19720_d_n0, assign13970_e19720_d_n2, assign13970_e19720_d_n6, assign13970_e19720_d_n7, assign13970_e19720_d_n10, assign13970_e19720_d_n11, assign13970_e19720_d_n12, assign13970_e19720_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13970_e19718: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13970_e19718, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13970_e19720;
        locals.var_xmp_dn0 = assign13970_e19720_d_n0;
        locals.var_xmp_dn2 = assign13970_e19720_d_n2;
        locals.var_xmp_dn6 = assign13970_e19720_d_n6;
        locals.var_xmp_dn7 = assign13970_e19720_d_n7;
        locals.var_xmp_dn10 = assign13970_e19720_d_n10;
        locals.var_xmp_dn11 = assign13970_e19720_d_n11;
        locals.var_xmp_dn12 = assign13970_e19720_d_n12;
        locals.var_xmp_dn17 = assign13970_e19720_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign13980_e19731, assign13980_e19731_d_n0, assign13980_e19731_d_n2, assign13980_e19731_d_n6, assign13980_e19731_d_n7, assign13980_e19731_d_n10, assign13980_e19731_d_n11, assign13980_e19731_d_n12, assign13980_e19731_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13980_e19729: f64 = (locals.var_xp * locals.var_x2);
        (assign13980_e19729, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13980_e19731;
        locals.var_xp_dn0 = assign13980_e19731_d_n0;
        locals.var_xp_dn2 = assign13980_e19731_d_n2;
        locals.var_xp_dn6 = assign13980_e19731_d_n6;
        locals.var_xp_dn7 = assign13980_e19731_d_n7;
        locals.var_xp_dn10 = assign13980_e19731_d_n10;
        locals.var_xp_dn11 = assign13980_e19731_d_n11;
        locals.var_xp_dn12 = assign13980_e19731_d_n12;
        locals.var_xp_dn17 = assign13980_e19731_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign13990_e19742, assign13990_e19742_d_n0, assign13990_e19742_d_n2, assign13990_e19742_d_n6, assign13990_e19742_d_n7, assign13990_e19742_d_n10, assign13990_e19742_d_n11, assign13990_e19742_d_n12, assign13990_e19742_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13990_e19740: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13990_e19740, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13990_e19742;
        locals.var_xmp_dn0 = assign13990_e19742_d_n0;
        locals.var_xmp_dn2 = assign13990_e19742_d_n2;
        locals.var_xmp_dn6 = assign13990_e19742_d_n6;
        locals.var_xmp_dn7 = assign13990_e19742_d_n7;
        locals.var_xmp_dn10 = assign13990_e19742_d_n10;
        locals.var_xmp_dn11 = assign13990_e19742_d_n11;
        locals.var_xmp_dn12 = assign13990_e19742_d_n12;
        locals.var_xmp_dn17 = assign13990_e19742_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign14000_e19753, assign14000_e19753_d_n0, assign14000_e19753_d_n2, assign14000_e19753_d_n6, assign14000_e19753_d_n7, assign14000_e19753_d_n10, assign14000_e19753_d_n11, assign14000_e19753_d_n12, assign14000_e19753_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign14000_e19751: f64 = (locals.var_xp + locals.var_xmp);
        (assign14000_e19751, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign14000_e19753;
        locals.var_arg_dn0 = assign14000_e19753_d_n0;
        locals.var_arg_dn2 = assign14000_e19753_d_n2;
        locals.var_arg_dn6 = assign14000_e19753_d_n6;
        locals.var_arg_dn7 = assign14000_e19753_d_n7;
        locals.var_arg_dn10 = assign14000_e19753_d_n10;
        locals.var_arg_dn11 = assign14000_e19753_d_n11;
        locals.var_arg_dn12 = assign14000_e19753_d_n12;
        locals.var_arg_dn17 = assign14000_e19753_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign14010_e19762, assign14010_e19762_d_n0, assign14010_e19762_d_n2, assign14010_e19762_d_n6, assign14010_e19762_d_n7, assign14010_e19762_d_n10, assign14010_e19762_d_n11, assign14010_e19762_d_n12, assign14010_e19762_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14010_e19762;
        locals.var_dnm_dn0 = assign14010_e19762_d_n0;
        locals.var_dnm_dn2 = assign14010_e19762_d_n2;
        locals.var_dnm_dn6 = assign14010_e19762_d_n6;
        locals.var_dnm_dn7 = assign14010_e19762_d_n7;
        locals.var_dnm_dn10 = assign14010_e19762_d_n10;
        locals.var_dnm_dn11 = assign14010_e19762_d_n11;
        locals.var_dnm_dn12 = assign14010_e19762_d_n12;
        locals.var_dnm_dn17 = assign14010_e19762_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign14020_e19777: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard430 = assign14020_e19777;
        locals.var_guard430_rv = 0.0;

        let assign14030_e19780: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard431 = assign14030_e19780;
        locals.var_guard431_rv = 0.0;

        let (assign14040_e19793,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) && (locals.var_guard431 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14040_e19793;
        locals.var_mm_rv = 0.0;

        let assign14050_e19796: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign14050_e19796;
        locals.var_guard432_rv = 0.0;

        let (assign14060_e19812,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard432 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14060_e19812;
        locals.var_mm_rv = 0.0;

        let assign14070_e19815: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard433 = assign14070_e19815;
        locals.var_guard433_rv = 0.0;

        let (assign14080_e19834,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard432 == 0.0)) && (locals.var_guard433 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14080_e19834;
        locals.var_mm_rv = 0.0;

        let assign14090_e19837: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign14090_e19837;
        locals.var_guard434_rv = 0.0;

        let (assign14100_e19859,) = {
    if ((((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard432 == 0.0)) && (locals.var_guard433 == 0.0)) && (locals.var_guard434 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14100_e19859;
        locals.var_mm_rv = 0.0;

        let (assign14110_e19870,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign14110_e19870;
        locals.var_m0_rv = 0.0;

        let mut assign14120_loop_guard: usize = 0;
        while {
            let assign14120_cond_e19882: f64 = if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign14120_cond_e19882 != 0.0
        } {
            assign14120_loop_guard += 1;
            assert!(assign14120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign14120_body0_e19894, assign14120_body0_e19894_d_n0, assign14120_body0_e19894_d_n2, assign14120_body0_e19894_d_n6, assign14120_body0_e19894_d_n7, assign14120_body0_e19894_d_n10, assign14120_body0_e19894_d_n11, assign14120_body0_e19894_d_n12, assign14120_body0_e19894_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) {
        let assign14120_body0_e19892: f64 = (locals.var_dnm).sqrt();
        (assign14120_body0_e19892, (locals.var_dnm_dn0 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn2 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn6 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn7 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn10 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn11 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn12 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn17 / (2.0 * assign14120_body0_e19892)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign14120_body0_e19894;
            locals.var_dnm_dn0 = assign14120_body0_e19894_d_n0;
            locals.var_dnm_dn2 = assign14120_body0_e19894_d_n2;
            locals.var_dnm_dn6 = assign14120_body0_e19894_d_n6;
            locals.var_dnm_dn7 = assign14120_body0_e19894_d_n7;
            locals.var_dnm_dn10 = assign14120_body0_e19894_d_n10;
            locals.var_dnm_dn11 = assign14120_body0_e19894_d_n11;
            locals.var_dnm_dn12 = assign14120_body0_e19894_d_n12;
            locals.var_dnm_dn17 = assign14120_body0_e19894_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign14120_body1_e19907,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) {
        let assign14120_body1_e19905: f64 = (locals.var_m0 + 1.0);
        (assign14120_body1_e19905,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign14120_body1_e19907;
            locals.var_m0_rv = 0.0;
        }

        let (assign14130_e19925, assign14130_e19925_d_n0, assign14130_e19925_d_n2, assign14130_e19925_d_n6, assign14130_e19925_d_n7, assign14130_e19925_d_n10, assign14130_e19925_d_n11, assign14130_e19925_d_n12, assign14130_e19925_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 == 0.0)) {
        let assign14130_e19921: f64 = (2.0 * 2.0);
        let assign14130_e19922: f64 = (1.0 / assign14130_e19921);
        let assign14130_e19923: f64 = (locals.var_dnm).powf(assign14130_e19922);
        (assign14130_e19923, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn0)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn2)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn6)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn7)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn10)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn11)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn12)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn17)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14130_e19925;
        locals.var_dnm_dn0 = assign14130_e19925_d_n0;
        locals.var_dnm_dn2 = assign14130_e19925_d_n2;
        locals.var_dnm_dn6 = assign14130_e19925_d_n6;
        locals.var_dnm_dn7 = assign14130_e19925_d_n7;
        locals.var_dnm_dn10 = assign14130_e19925_d_n10;
        locals.var_dnm_dn11 = assign14130_e19925_d_n11;
        locals.var_dnm_dn12 = assign14130_e19925_d_n12;
        locals.var_dnm_dn17 = assign14130_e19925_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign14140_e19936, assign14140_e19936_d_n0, assign14140_e19936_d_n2, assign14140_e19936_d_n6, assign14140_e19936_d_n7, assign14140_e19936_d_n10, assign14140_e19936_d_n11, assign14140_e19936_d_n12, assign14140_e19936_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign14140_e19934: f64 = (1.0 / locals.var_dnm);
        (assign14140_e19934, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14140_e19936;
        locals.var_dnm_dn0 = assign14140_e19936_d_n0;
        locals.var_dnm_dn2 = assign14140_e19936_d_n2;
        locals.var_dnm_dn6 = assign14140_e19936_d_n6;
        locals.var_dnm_dn7 = assign14140_e19936_d_n7;
        locals.var_dnm_dn10 = assign14140_e19936_d_n10;
        locals.var_dnm_dn11 = assign14140_e19936_d_n11;
        locals.var_dnm_dn12 = assign14140_e19936_d_n12;
        locals.var_dnm_dn17 = assign14140_e19936_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign14150_e19949, assign14150_e19949_d_n0, assign14150_e19949_d_n2, assign14150_e19949_d_n6, assign14150_e19949_d_n7, assign14150_e19949_d_n10, assign14150_e19949_d_n11, assign14150_e19949_d_n12, assign14150_e19949_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign14150_e19945: f64 = (locals.var_tmf1 * 1e-5);
        let assign14150_e19947: f64 = (assign14150_e19945 * locals.var_dnm);
        (assign14150_e19947, (((locals.var_tmf1_dn0 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign14150_e19949;
        locals.var_tmf0_dn0 = assign14150_e19949_d_n0;
        locals.var_tmf0_dn2 = assign14150_e19949_d_n2;
        locals.var_tmf0_dn6 = assign14150_e19949_d_n6;
        locals.var_tmf0_dn7 = assign14150_e19949_d_n7;
        locals.var_tmf0_dn10 = assign14150_e19949_d_n10;
        locals.var_tmf0_dn11 = assign14150_e19949_d_n11;
        locals.var_tmf0_dn12 = assign14150_e19949_d_n12;
        locals.var_tmf0_dn17 = assign14150_e19949_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign14160_e19962, assign14160_e19962_d_n0, assign14160_e19962_d_n2, assign14160_e19962_d_n6, assign14160_e19962_d_n7, assign14160_e19962_d_n10, assign14160_e19962_d_n11, assign14160_e19962_d_n12, assign14160_e19962_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign14160_e19958: f64 = 1e-5;
        let assign14160_e19960: f64 = (assign14160_e19958 - locals.var_tmf0);
        (assign14160_e19960, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14160_e19962;
        locals.var_rrr_eta_dn0 = assign14160_e19962_d_n0;
        locals.var_rrr_eta_dn2 = assign14160_e19962_d_n2;
        locals.var_rrr_eta_dn6 = assign14160_e19962_d_n6;
        locals.var_rrr_eta_dn7 = assign14160_e19962_d_n7;
        locals.var_rrr_eta_dn10 = assign14160_e19962_d_n10;
        locals.var_rrr_eta_dn11 = assign14160_e19962_d_n11;
        locals.var_rrr_eta_dn12 = assign14160_e19962_d_n12;
        locals.var_rrr_eta_dn17 = assign14160_e19962_d_n17;
        locals.var_rrr_eta_rv = 0.0;

        let (assign14170_e19972, assign14170_e19972_d_n0, assign14170_e19972_d_n2, assign14170_e19972_d_n6, assign14170_e19972_d_n7, assign14170_e19972_d_n10, assign14170_e19972_d_n11, assign14170_e19972_d_n12, assign14170_e19972_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14170_e19972;
        locals.var_rrr_eta_dn0 = assign14170_e19972_d_n0;
        locals.var_rrr_eta_dn2 = assign14170_e19972_d_n2;
        locals.var_rrr_eta_dn6 = assign14170_e19972_d_n6;
        locals.var_rrr_eta_dn7 = assign14170_e19972_d_n7;
        locals.var_rrr_eta_dn10 = assign14170_e19972_d_n10;
        locals.var_rrr_eta_dn11 = assign14170_e19972_d_n11;
        locals.var_rrr_eta_dn12 = assign14170_e19972_d_n12;
        locals.var_rrr_eta_dn17 = assign14170_e19972_d_n17;
        locals.var_rrr_eta_rv = 0.0;

        let (assign14180_e19979, assign14180_e19979_d_n0, assign14180_e19979_d_n2, assign14180_e19979_d_n6, assign14180_e19979_d_n7, assign14180_e19979_d_n10, assign14180_e19979_d_n11, assign14180_e19979_d_n12, assign14180_e19979_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    }
};
        locals.var_alpha = assign14180_e19979;
        locals.var_alpha_dn0 = assign14180_e19979_d_n0;
        locals.var_alpha_dn2 = assign14180_e19979_d_n2;
        locals.var_alpha_dn6 = assign14180_e19979_d_n6;
        locals.var_alpha_dn7 = assign14180_e19979_d_n7;
        locals.var_alpha_dn10 = assign14180_e19979_d_n10;
        locals.var_alpha_dn11 = assign14180_e19979_d_n11;
        locals.var_alpha_dn12 = assign14180_e19979_d_n12;
        locals.var_alpha_dn17 = assign14180_e19979_d_n17;
        locals.var_alpha_rv = 0.0;

        let (assign14190_e19992, assign14190_e19992_d_n0, assign14190_e19992_d_n2, assign14190_e19992_d_n6, assign14190_e19992_d_n7, assign14190_e19992_d_n10, assign14190_e19992_d_n11, assign14190_e19992_d_n12, assign14190_e19992_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign14190_e19988: f64 = (1.0 + locals.var_alpha);
        let assign14190_e19989: f64 = (locals.var_alpha * assign14190_e19988);
        let assign14190_e19990: f64 = (1.0 + assign14190_e19989);
        (assign14190_e19990, ((locals.var_alpha_dn0 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn12, locals.var_qinm_dn17,)
    }
};
        locals.var_qinm = assign14190_e19992;
        locals.var_qinm_dn0 = assign14190_e19992_d_n0;
        locals.var_qinm_dn2 = assign14190_e19992_d_n2;
        locals.var_qinm_dn6 = assign14190_e19992_d_n6;
        locals.var_qinm_dn7 = assign14190_e19992_d_n7;
        locals.var_qinm_dn10 = assign14190_e19992_d_n10;
        locals.var_qinm_dn11 = assign14190_e19992_d_n11;
        locals.var_qinm_dn12 = assign14190_e19992_d_n12;
        locals.var_qinm_dn17 = assign14190_e19992_d_n17;
        locals.var_qinm_rv = 0.0;

        let (assign14200_e20012, assign14200_e20012_d_n0, assign14200_e20012_d_n2, assign14200_e20012_d_n6, assign14200_e20012_d_n7, assign14200_e20012_d_n10, assign14200_e20012_d_n11, assign14200_e20012_d_n12, assign14200_e20012_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign14200_e19999: f64 = (1.0 + locals.var_alpha);
        let assign14200_e20002: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14200_e20010, assign14200_e20010_d_n0, assign14200_e20010_d_n2, assign14200_e20010_d_n6, assign14200_e20010_d_n7, assign14200_e20010_d_n10, assign14200_e20010_d_n11, assign14200_e20010_d_n12, assign14200_e20010_d_n17,) = {
            if (assign14200_e19999 >= assign14200_e20002) {
                let assign14200_e20006: f64 = (1.0 + locals.var_alpha);
                (assign14200_e20006, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
            } else {
                let assign14200_e20009: f64 = (10.0 * 2.220446049250313e-16);
                (assign14200_e20009, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14200_e20010, assign14200_e20010_d_n0, assign14200_e20010_d_n2, assign14200_e20010_d_n6, assign14200_e20010_d_n7, assign14200_e20010_d_n10, assign14200_e20010_d_n11, assign14200_e20010_d_n12, assign14200_e20010_d_n17,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn12, locals.var_qidn_dn17,)
    }
};
        locals.var_qidn = assign14200_e20012;
        locals.var_qidn_dn0 = assign14200_e20012_d_n0;
        locals.var_qidn_dn2 = assign14200_e20012_d_n2;
        locals.var_qidn_dn6 = assign14200_e20012_d_n6;
        locals.var_qidn_dn7 = assign14200_e20012_d_n7;
        locals.var_qidn_dn10 = assign14200_e20012_d_n10;
        locals.var_qidn_dn11 = assign14200_e20012_d_n11;
        locals.var_qidn_dn12 = assign14200_e20012_d_n12;
        locals.var_qidn_dn17 = assign14200_e20012_d_n17;
        locals.var_qidn_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14210_e20024, assign14210_e20024_d_n0, assign14210_e20024_d_n2, assign14210_e20024_d_n6, assign14210_e20024_d_n7, assign14210_e20024_d_n10, assign14210_e20024_d_n11, assign14210_e20024_d_n12, assign14210_e20024_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign14210_e20018: f64 = (-0.5);
        let assign14210_e20021: f64 = (locals.var_q_n0 + locals.var_q_nl);
        let assign14210_e20022: f64 = (assign14210_e20018 * assign14210_e20021);
        (assign14210_e20022, (assign14210_e20018 * (locals.var_q_n0_dn0 + locals.var_q_nl_dn0)), (assign14210_e20018 * (locals.var_q_n0_dn2 + locals.var_q_nl_dn2)), (assign14210_e20018 * (locals.var_q_n0_dn6 + locals.var_q_nl_dn6)), (assign14210_e20018 * (locals.var_q_n0_dn7 + locals.var_q_nl_dn7)), (assign14210_e20018 * (locals.var_q_n0_dn10 + locals.var_q_nl_dn10)), (assign14210_e20018 * (locals.var_q_n0_dn11 + locals.var_q_nl_dn11)), (assign14210_e20018 * (locals.var_q_n0_dn12 + locals.var_q_nl_dn12)), (assign14210_e20018 * (locals.var_q_n0_dn17 + locals.var_q_nl_dn17)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign14210_e20024;
        locals.var_qiu_dn0 = assign14210_e20024_d_n0;
        locals.var_qiu_dn2 = assign14210_e20024_d_n2;
        locals.var_qiu_dn6 = assign14210_e20024_d_n6;
        locals.var_qiu_dn7 = assign14210_e20024_d_n7;
        locals.var_qiu_dn10 = assign14210_e20024_d_n10;
        locals.var_qiu_dn11 = assign14210_e20024_d_n11;
        locals.var_qiu_dn12 = assign14210_e20024_d_n12;
        locals.var_qiu_dn17 = assign14210_e20024_d_n17;
        locals.var_qiu_rv = 0.0;

        let (assign14280_e20057, assign14280_e20057_d_n0, assign14280_e20057_d_n2, assign14280_e20057_d_n6, assign14280_e20057_d_n7, assign14280_e20057_d_n10, assign14280_e20057_d_n11, assign14280_e20057_d_n12, assign14280_e20057_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    } else {
        (locals.var_vbcs_cl, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    }
};
        locals.var_vbcs_cl = assign14280_e20057;
        locals.var_vbcs_cl_dn0 = assign14280_e20057_d_n0;
        locals.var_vbcs_cl_dn2 = assign14280_e20057_d_n2;
        locals.var_vbcs_cl_dn6 = assign14280_e20057_d_n6;
        locals.var_vbcs_cl_dn7 = assign14280_e20057_d_n7;
        locals.var_vbcs_cl_dn10 = assign14280_e20057_d_n10;
        locals.var_vbcs_cl_dn11 = assign14280_e20057_d_n11;
        locals.var_vbcs_cl_dn12 = assign14280_e20057_d_n12;
        locals.var_vbcs_cl_dn17 = assign14280_e20057_d_n17;
        locals.var_vbcs_cl_rv = 0.0;

        let assign14290_e20060: f64 = if locals.var_wdsoi_ini < p.p237 { 1.0 } else { 0.0 };
        locals.var_guard441 = assign14290_e20060;
        locals.var_guard441_rv = 0.0;

        let (assign14300_e20067,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard441 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14300_e20067;
        locals.var_flg_depmode_rv = 0.0;

        let (assign14310_e20075,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard441 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14310_e20075;
        locals.var_flg_depmode_rv = 0.0;

        let (assign14320_e20086, assign14320_e20086_d_n0, assign14320_e20086_d_n2, assign14320_e20086_d_n6, assign14320_e20086_d_n7, assign14320_e20086_d_n10, assign14320_e20086_d_n11, assign14320_e20086_d_n12, assign14320_e20086_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign14320_e20080: f64 = (locals.var_vfb - locals.var_dvth);
        let assign14320_e20082: f64 = (assign14320_e20080 + locals.var_dppg);
        let assign14320_e20084: f64 = (assign14320_e20082 + locals.var_vbcs_cl);
        (assign14320_e20084, (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbcs_cl_dn0), (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbcs_cl_dn2), (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbcs_cl_dn6), (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbcs_cl_dn7), (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbcs_cl_dn10), (((-locals.var_dvth_dn11) + locals.var_dppg_dn11) + locals.var_vbcs_cl_dn11), (((-locals.var_dvth_dn12) + locals.var_dppg_dn12) + locals.var_vbcs_cl_dn12), (((-locals.var_dvth_dn17) + locals.var_dppg_dn17) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn7, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn11, locals.var_vgs_fb_dn12, locals.var_vgs_fb_dn17,)
    }
};
        locals.var_vgs_fb = assign14320_e20086;
        locals.var_vgs_fb_dn0 = assign14320_e20086_d_n0;
        locals.var_vgs_fb_dn2 = assign14320_e20086_d_n2;
        locals.var_vgs_fb_dn6 = assign14320_e20086_d_n6;
        locals.var_vgs_fb_dn7 = assign14320_e20086_d_n7;
        locals.var_vgs_fb_dn10 = assign14320_e20086_d_n10;
        locals.var_vgs_fb_dn11 = assign14320_e20086_d_n11;
        locals.var_vgs_fb_dn12 = assign14320_e20086_d_n12;
        locals.var_vgs_fb_dn17 = assign14320_e20086_d_n17;
        locals.var_vgs_fb_rv = 0.0;

        let assign14330_e20089: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard442 = assign14330_e20089;
        locals.var_guard442_rv = 0.0;

        let (assign14340_e20097,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14340_e20095: f64 = (-1.0);
        (assign14340_e20095,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign14340_e20097;
        locals.var_flg_zone_rv = 0.0;

        let (assign14350_e20112, assign14350_e20112_d_n0, assign14350_e20112_d_n2, assign14350_e20112_d_n6, assign14350_e20112_d_n7, assign14350_e20112_d_n10, assign14350_e20112_d_n11, assign14350_e20112_d_n12, assign14350_e20112_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14350_e20104: f64 = (2.0 * locals.var_beta_inv);
        let assign14350_e20106: f64 = (-locals.var_vgs_min);
        let assign14350_e20108: f64 = (assign14350_e20106 / locals.var_fac1);
        let assign14350_e20109: f64 = (assign14350_e20108).ln();
        let assign14350_e20110: f64 = (assign14350_e20104 * assign14350_e20109);
        (assign14350_e20110, (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (((2.0 * locals.var_beta_inv_dn10) * assign14350_e20109) + (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108))), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn12) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn17) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12, locals.var_ps0_min_dn17,)
    }
};
        locals.var_ps0_min = assign14350_e20112;
        locals.var_ps0_min_dn0 = assign14350_e20112_d_n0;
        locals.var_ps0_min_dn2 = assign14350_e20112_d_n2;
        locals.var_ps0_min_dn6 = assign14350_e20112_d_n6;
        locals.var_ps0_min_dn7 = assign14350_e20112_d_n7;
        locals.var_ps0_min_dn10 = assign14350_e20112_d_n10;
        locals.var_ps0_min_dn11 = assign14350_e20112_d_n11;
        locals.var_ps0_min_dn12 = assign14350_e20112_d_n12;
        locals.var_ps0_min_dn17 = assign14350_e20112_d_n17;
        locals.var_ps0_min_rv = 0.0;

        let (assign14360_e20123, assign14360_e20123_d_n0, assign14360_e20123_d_n2, assign14360_e20123_d_n6, assign14360_e20123_d_n7, assign14360_e20123_d_n10, assign14360_e20123_d_n11, assign14360_e20123_d_n12, assign14360_e20123_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14360_e20120: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14360_e20121: f64 = (locals.var_beta * assign14360_e20120);
        (assign14360_e20121, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14360_e20120) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14360_e20123;
        locals.var_tx_dn0 = assign14360_e20123_d_n0;
        locals.var_tx_dn2 = assign14360_e20123_d_n2;
        locals.var_tx_dn6 = assign14360_e20123_d_n6;
        locals.var_tx_dn7 = assign14360_e20123_d_n7;
        locals.var_tx_dn10 = assign14360_e20123_d_n10;
        locals.var_tx_dn11 = assign14360_e20123_d_n11;
        locals.var_tx_dn12 = assign14360_e20123_d_n12;
        locals.var_tx_dn17 = assign14360_e20123_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14370_e20134, assign14370_e20134_d_n0, assign14370_e20134_d_n2, assign14370_e20134_d_n6, assign14370_e20134_d_n7, assign14370_e20134_d_n10, assign14370_e20134_d_n11, assign14370_e20134_d_n12, assign14370_e20134_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14370_e20131: f64 = (locals.var_beta * locals.var_cnst0soi);
        let assign14370_e20132: f64 = (1.0 / assign14370_e20131);
        (assign14370_e20132, (-((locals.var_beta * locals.var_cnst0soi_dn0) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn2) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn6) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn7) / (assign14370_e20131 * assign14370_e20131))), (-(((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn11) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn12) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn17) / (assign14370_e20131 * assign14370_e20131))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14370_e20134;
        locals.var_t1_dn0 = assign14370_e20134_d_n0;
        locals.var_t1_dn2 = assign14370_e20134_d_n2;
        locals.var_t1_dn6 = assign14370_e20134_d_n6;
        locals.var_t1_dn7 = assign14370_e20134_d_n7;
        locals.var_t1_dn10 = assign14370_e20134_d_n10;
        locals.var_t1_dn11 = assign14370_e20134_d_n11;
        locals.var_t1_dn12 = assign14370_e20134_d_n12;
        locals.var_t1_dn17 = assign14370_e20134_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14380_e20143, assign14380_e20143_d_n0, assign14380_e20143_d_n2, assign14380_e20143_d_n6, assign14380_e20143_d_n7, assign14380_e20143_d_n10, assign14380_e20143_d_n11, assign14380_e20143_d_n12, assign14380_e20143_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14380_e20141: f64 = (locals.var_t1 * locals.var_c_fox);
        (assign14380_e20141, ((locals.var_t1_dn0 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn0)), ((locals.var_t1_dn2 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn2)), ((locals.var_t1_dn6 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn6)), ((locals.var_t1_dn7 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn7)), ((locals.var_t1_dn10 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn10)), ((locals.var_t1_dn11 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn11)), ((locals.var_t1_dn12 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn12)), ((locals.var_t1_dn17 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14380_e20143;
        locals.var_ty_dn0 = assign14380_e20143_d_n0;
        locals.var_ty_dn2 = assign14380_e20143_d_n2;
        locals.var_ty_dn6 = assign14380_e20143_d_n6;
        locals.var_ty_dn7 = assign14380_e20143_d_n7;
        locals.var_ty_dn10 = assign14380_e20143_d_n10;
        locals.var_ty_dn11 = assign14380_e20143_d_n11;
        locals.var_ty_dn12 = assign14380_e20143_d_n12;
        locals.var_ty_dn17 = assign14380_e20143_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign14390_e20156, assign14390_e20156_d_n0, assign14390_e20156_d_n2, assign14390_e20156_d_n6, assign14390_e20156_d_n7, assign14390_e20156_d_n10, assign14390_e20156_d_n11, assign14390_e20156_d_n12, assign14390_e20156_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14390_e20151: f64 = (3.0 * 1.414213562373095);
        let assign14390_e20153: f64 = (assign14390_e20151 * locals.var_ty);
        let assign14390_e20154: f64 = (2.0 + assign14390_e20153);
        (assign14390_e20154, (assign14390_e20151 * locals.var_ty_dn0), (assign14390_e20151 * locals.var_ty_dn2), (assign14390_e20151 * locals.var_ty_dn6), (assign14390_e20151 * locals.var_ty_dn7), (assign14390_e20151 * locals.var_ty_dn10), (assign14390_e20151 * locals.var_ty_dn11), (assign14390_e20151 * locals.var_ty_dn12), (assign14390_e20151 * locals.var_ty_dn17),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12, locals.var_ac41_dn17,)
    }
};
        locals.var_ac41 = assign14390_e20156;
        locals.var_ac41_dn0 = assign14390_e20156_d_n0;
        locals.var_ac41_dn2 = assign14390_e20156_d_n2;
        locals.var_ac41_dn6 = assign14390_e20156_d_n6;
        locals.var_ac41_dn7 = assign14390_e20156_d_n7;
        locals.var_ac41_dn10 = assign14390_e20156_d_n10;
        locals.var_ac41_dn11 = assign14390_e20156_d_n11;
        locals.var_ac41_dn12 = assign14390_e20156_d_n12;
        locals.var_ac41_dn17 = assign14390_e20156_d_n17;
        locals.var_ac41_rv = 0.0;

        let (assign14400_e20169, assign14400_e20169_d_n0, assign14400_e20169_d_n2, assign14400_e20169_d_n6, assign14400_e20169_d_n7, assign14400_e20169_d_n10, assign14400_e20169_d_n11, assign14400_e20169_d_n12, assign14400_e20169_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14400_e20163: f64 = (8.0 * locals.var_ac41);
        let assign14400_e20165: f64 = (assign14400_e20163 * locals.var_ac41);
        let assign14400_e20167: f64 = (assign14400_e20165 * locals.var_ac41);
        (assign14400_e20167, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn12)), (((((8.0 * locals.var_ac41_dn17) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn17)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn17)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12, locals.var_ac4_dn17,)
    }
};
        locals.var_ac4 = assign14400_e20169;
        locals.var_ac4_dn0 = assign14400_e20169_d_n0;
        locals.var_ac4_dn2 = assign14400_e20169_d_n2;
        locals.var_ac4_dn6 = assign14400_e20169_d_n6;
        locals.var_ac4_dn7 = assign14400_e20169_d_n7;
        locals.var_ac4_dn10 = assign14400_e20169_d_n10;
        locals.var_ac4_dn11 = assign14400_e20169_d_n11;
        locals.var_ac4_dn12 = assign14400_e20169_d_n12;
        locals.var_ac4_dn17 = assign14400_e20169_d_n17;
        locals.var_ac4_rv = 0.0;

        let (assign14410_e20178, assign14410_e20178_d_n0, assign14410_e20178_d_n2, assign14410_e20178_d_n6, assign14410_e20178_d_n7, assign14410_e20178_d_n10, assign14410_e20178_d_n11, assign14410_e20178_d_n12, assign14410_e20178_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14410_e20176: f64 = (locals.var_tx - 2.0);
        (assign14410_e20176, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14410_e20178;
        locals.var_t4_dn0 = assign14410_e20178_d_n0;
        locals.var_t4_dn2 = assign14410_e20178_d_n2;
        locals.var_t4_dn6 = assign14410_e20178_d_n6;
        locals.var_t4_dn7 = assign14410_e20178_d_n7;
        locals.var_t4_dn10 = assign14410_e20178_d_n10;
        locals.var_t4_dn11 = assign14410_e20178_d_n11;
        locals.var_t4_dn12 = assign14410_e20178_d_n12;
        locals.var_t4_dn17 = assign14410_e20178_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign14420_e20189, assign14420_e20189_d_n0, assign14420_e20189_d_n2, assign14420_e20189_d_n6, assign14420_e20189_d_n7, assign14420_e20189_d_n10, assign14420_e20189_d_n11, assign14420_e20189_d_n12, assign14420_e20189_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14420_e20185: f64 = (9.0 * locals.var_ty);
        let assign14420_e20187: f64 = (assign14420_e20185 * locals.var_t4);
        (assign14420_e20187, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn12) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn12)), (((9.0 * locals.var_ty_dn17) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14420_e20189;
        locals.var_t5_dn0 = assign14420_e20189_d_n0;
        locals.var_t5_dn2 = assign14420_e20189_d_n2;
        locals.var_t5_dn6 = assign14420_e20189_d_n6;
        locals.var_t5_dn7 = assign14420_e20189_d_n7;
        locals.var_t5_dn10 = assign14420_e20189_d_n10;
        locals.var_t5_dn11 = assign14420_e20189_d_n11;
        locals.var_t5_dn12 = assign14420_e20189_d_n12;
        locals.var_t5_dn17 = assign14420_e20189_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign14430_e20200, assign14430_e20200_d_n0, assign14430_e20200_d_n2, assign14430_e20200_d_n6, assign14430_e20200_d_n7, assign14430_e20200_d_n10, assign14430_e20200_d_n11, assign14430_e20200_d_n12, assign14430_e20200_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14430_e20196: f64 = (7.0 * 1.414213562373095);
        let assign14430_e20198: f64 = (assign14430_e20196 - locals.var_t5);
        (assign14430_e20198, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn12), (-locals.var_t5_dn17),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12, locals.var_ac31_dn17,)
    }
};
        locals.var_ac31 = assign14430_e20200;
        locals.var_ac31_dn0 = assign14430_e20200_d_n0;
        locals.var_ac31_dn2 = assign14430_e20200_d_n2;
        locals.var_ac31_dn6 = assign14430_e20200_d_n6;
        locals.var_ac31_dn7 = assign14430_e20200_d_n7;
        locals.var_ac31_dn10 = assign14430_e20200_d_n10;
        locals.var_ac31_dn11 = assign14430_e20200_d_n11;
        locals.var_ac31_dn12 = assign14430_e20200_d_n12;
        locals.var_ac31_dn17 = assign14430_e20200_d_n17;
        locals.var_ac31_rv = 0.0;

        let (assign14440_e20209, assign14440_e20209_d_n0, assign14440_e20209_d_n2, assign14440_e20209_d_n6, assign14440_e20209_d_n7, assign14440_e20209_d_n10, assign14440_e20209_d_n11, assign14440_e20209_d_n12, assign14440_e20209_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14440_e20207: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign14440_e20207, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)), ((locals.var_ac31_dn17 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn17)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12, locals.var_ac3_dn17,)
    }
};
        locals.var_ac3 = assign14440_e20209;
        locals.var_ac3_dn0 = assign14440_e20209_d_n0;
        locals.var_ac3_dn2 = assign14440_e20209_d_n2;
        locals.var_ac3_dn6 = assign14440_e20209_d_n6;
        locals.var_ac3_dn7 = assign14440_e20209_d_n7;
        locals.var_ac3_dn10 = assign14440_e20209_d_n10;
        locals.var_ac3_dn11 = assign14440_e20209_d_n11;
        locals.var_ac3_dn12 = assign14440_e20209_d_n12;
        locals.var_ac3_dn17 = assign14440_e20209_d_n17;
        locals.var_ac3_rv = 0.0;

        let assign14450_e20213: f64 = (locals.var_ac3 * 1e-8);
        let assign14450_e20214: f64 = if locals.var_ac4 < assign14450_e20213 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign14450_e20214;
        locals.var_guard443_rv = 0.0;

        let (assign14460_e20236, assign14460_e20236_d_n0, assign14460_e20236_d_n2, assign14460_e20236_d_n6, assign14460_e20236_d_n7, assign14460_e20236_d_n10, assign14460_e20236_d_n11, assign14460_e20236_d_n12, assign14460_e20236_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14460_e20222: f64 = (-7.0);
        let assign14460_e20224: f64 = (assign14460_e20222 * 1.414213562373095);
        let assign14460_e20226: f64 = (assign14460_e20224 + locals.var_ac31);
        let assign14460_e20229: f64 = (0.5 * locals.var_ac4);
        let assign14460_e20231: f64 = (assign14460_e20229 / locals.var_ac31);
        let assign14460_e20232: f64 = (assign14460_e20226 + assign14460_e20231);
        let assign14460_e20234: f64 = (assign14460_e20232 + locals.var_t5);
        (assign14460_e20234, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn11), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn12), ((locals.var_ac31_dn17 + ((((0.5 * locals.var_ac4_dn17) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn17)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14460_e20236;
        locals.var_ac1_dn0 = assign14460_e20236_d_n0;
        locals.var_ac1_dn2 = assign14460_e20236_d_n2;
        locals.var_ac1_dn6 = assign14460_e20236_d_n6;
        locals.var_ac1_dn7 = assign14460_e20236_d_n7;
        locals.var_ac1_dn10 = assign14460_e20236_d_n10;
        locals.var_ac1_dn11 = assign14460_e20236_d_n11;
        locals.var_ac1_dn12 = assign14460_e20236_d_n12;
        locals.var_ac1_dn17 = assign14460_e20236_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign14470_e20249, assign14470_e20249_d_n0, assign14470_e20249_d_n2, assign14470_e20249_d_n6, assign14470_e20249_d_n7, assign14470_e20249_d_n10, assign14470_e20249_d_n11, assign14470_e20249_d_n12, assign14470_e20249_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign14470_e20246: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign14470_e20247: f64 = (assign14470_e20246).sqrt();
        (assign14470_e20247, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn17 + locals.var_ac3_dn17) / (2.0 * assign14470_e20247)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12, locals.var_ac2_dn17,)
    }
};
        locals.var_ac2 = assign14470_e20249;
        locals.var_ac2_dn0 = assign14470_e20249_d_n0;
        locals.var_ac2_dn2 = assign14470_e20249_d_n2;
        locals.var_ac2_dn6 = assign14470_e20249_d_n6;
        locals.var_ac2_dn7 = assign14470_e20249_d_n7;
        locals.var_ac2_dn10 = assign14470_e20249_d_n10;
        locals.var_ac2_dn11 = assign14470_e20249_d_n11;
        locals.var_ac2_dn12 = assign14470_e20249_d_n12;
        locals.var_ac2_dn17 = assign14470_e20249_d_n17;
        locals.var_ac2_rv = 0.0;

        let (assign14480_e20266, assign14480_e20266_d_n0, assign14480_e20266_d_n2, assign14480_e20266_d_n6, assign14480_e20266_d_n7, assign14480_e20266_d_n10, assign14480_e20266_d_n11, assign14480_e20266_d_n12, assign14480_e20266_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign14480_e20258: f64 = (-7.0);
        let assign14480_e20260: f64 = (assign14480_e20258 * 1.414213562373095);
        let assign14480_e20262: f64 = (assign14480_e20260 + locals.var_ac2);
        let assign14480_e20264: f64 = (assign14480_e20262 + locals.var_t5);
        (assign14480_e20264, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn11 + locals.var_t5_dn11), (locals.var_ac2_dn12 + locals.var_t5_dn12), (locals.var_ac2_dn17 + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14480_e20266;
        locals.var_ac1_dn0 = assign14480_e20266_d_n0;
        locals.var_ac1_dn2 = assign14480_e20266_d_n2;
        locals.var_ac1_dn6 = assign14480_e20266_d_n6;
        locals.var_ac1_dn7 = assign14480_e20266_d_n7;
        locals.var_ac1_dn10 = assign14480_e20266_d_n10;
        locals.var_ac1_dn11 = assign14480_e20266_d_n11;
        locals.var_ac1_dn12 = assign14480_e20266_d_n12;
        locals.var_ac1_dn17 = assign14480_e20266_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign14490_e20275, assign14490_e20275_d_n0, assign14490_e20275_d_n2, assign14490_e20275_d_n6, assign14490_e20275_d_n7, assign14490_e20275_d_n10, assign14490_e20275_d_n11, assign14490_e20275_d_n12, assign14490_e20275_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14490_e20273: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign14490_e20273, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn17)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn17 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12, locals.var_acd_dn17,)
    }
};
        locals.var_acd = assign14490_e20275;
        locals.var_acd_dn0 = assign14490_e20275_d_n0;
        locals.var_acd_dn2 = assign14490_e20275_d_n2;
        locals.var_acd_dn6 = assign14490_e20275_d_n6;
        locals.var_acd_dn7 = assign14490_e20275_d_n7;
        locals.var_acd_dn10 = assign14490_e20275_d_n10;
        locals.var_acd_dn11 = assign14490_e20275_d_n11;
        locals.var_acd_dn12 = assign14490_e20275_d_n12;
        locals.var_acd_dn17 = assign14490_e20275_d_n17;
        locals.var_acd_rv = 0.0;

        let (assign14500_e20299, assign14500_e20299_d_n0, assign14500_e20299_d_n2, assign14500_e20299_d_n6, assign14500_e20299_d_n7, assign14500_e20299_d_n10, assign14500_e20299_d_n11, assign14500_e20299_d_n12, assign14500_e20299_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14500_e20281: f64 = (-4.0);
        let assign14500_e20283: f64 = (assign14500_e20281 * 1.414213562373095);
        let assign14500_e20286: f64 = (12.0 * locals.var_ty);
        let assign14500_e20287: f64 = (assign14500_e20283 - assign14500_e20286);
        let assign14500_e20290: f64 = (2.0 * locals.var_acd);
        let assign14500_e20291: f64 = (assign14500_e20287 + assign14500_e20290);
        let assign14500_e20294: f64 = (1.414213562373095 * locals.var_acd);
        let assign14500_e20296: f64 = (assign14500_e20294 * locals.var_acd);
        let assign14500_e20297: f64 = (assign14500_e20291 + assign14500_e20296);
        (assign14500_e20297, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn12))), (((-(12.0 * locals.var_ty_dn17)) + (2.0 * locals.var_acd_dn17)) + (((1.414213562373095 * locals.var_acd_dn17) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn17))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12, locals.var_acn_dn17,)
    }
};
        locals.var_acn = assign14500_e20299;
        locals.var_acn_dn0 = assign14500_e20299_d_n0;
        locals.var_acn_dn2 = assign14500_e20299_d_n2;
        locals.var_acn_dn6 = assign14500_e20299_d_n6;
        locals.var_acn_dn7 = assign14500_e20299_d_n7;
        locals.var_acn_dn10 = assign14500_e20299_d_n10;
        locals.var_acn_dn11 = assign14500_e20299_d_n11;
        locals.var_acn_dn12 = assign14500_e20299_d_n12;
        locals.var_acn_dn17 = assign14500_e20299_d_n17;
        locals.var_acn_rv = 0.0;

        let (assign14510_e20308, assign14510_e20308_d_n0, assign14510_e20308_d_n2, assign14510_e20308_d_n6, assign14510_e20308_d_n7, assign14510_e20308_d_n10, assign14510_e20308_d_n11, assign14510_e20308_d_n12, assign14510_e20308_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14510_e20306: f64 = (1.0 / locals.var_acd);
        (assign14510_e20306, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn12 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn17 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14510_e20308;
        locals.var_t1_dn0 = assign14510_e20308_d_n0;
        locals.var_t1_dn2 = assign14510_e20308_d_n2;
        locals.var_t1_dn6 = assign14510_e20308_d_n6;
        locals.var_t1_dn7 = assign14510_e20308_d_n7;
        locals.var_t1_dn10 = assign14510_e20308_d_n10;
        locals.var_t1_dn11 = assign14510_e20308_d_n11;
        locals.var_t1_dn12 = assign14510_e20308_d_n12;
        locals.var_t1_dn17 = assign14510_e20308_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14520_e20317, assign14520_e20317_d_n0, assign14520_e20317_d_n2, assign14520_e20317_d_n6, assign14520_e20317_d_n7, assign14520_e20317_d_n10, assign14520_e20317_d_n11, assign14520_e20317_d_n12, assign14520_e20317_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14520_e20315: f64 = (locals.var_acn * locals.var_t1);
        (assign14520_e20315, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn12 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn12)), ((locals.var_acn_dn17 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14520_e20317;
        locals.var_chi_dn0 = assign14520_e20317_d_n0;
        locals.var_chi_dn2 = assign14520_e20317_d_n2;
        locals.var_chi_dn6 = assign14520_e20317_d_n6;
        locals.var_chi_dn7 = assign14520_e20317_d_n7;
        locals.var_chi_dn10 = assign14520_e20317_d_n10;
        locals.var_chi_dn11 = assign14520_e20317_d_n11;
        locals.var_chi_dn12 = assign14520_e20317_d_n12;
        locals.var_chi_dn17 = assign14520_e20317_d_n17;
        locals.var_chi_rv = 0.0;

        let (assign14530_e20328, assign14530_e20328_d_n0, assign14530_e20328_d_n2, assign14530_e20328_d_n6, assign14530_e20328_d_n7, assign14530_e20328_d_n10, assign14530_e20328_d_n11, assign14530_e20328_d_n12, assign14530_e20328_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14530_e20324: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign14530_e20326: f64 = (assign14530_e20324 + locals.var_vbcs_cl);
        (assign14530_e20326, ((locals.var_chi_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_chi_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_chi_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_chi_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12, locals.var_psa_dn17,)
    }
};
        locals.var_psa = assign14530_e20328;
        locals.var_psa_dn0 = assign14530_e20328_d_n0;
        locals.var_psa_dn2 = assign14530_e20328_d_n2;
        locals.var_psa_dn6 = assign14530_e20328_d_n6;
        locals.var_psa_dn7 = assign14530_e20328_d_n7;
        locals.var_psa_dn10 = assign14530_e20328_d_n10;
        locals.var_psa_dn11 = assign14530_e20328_d_n11;
        locals.var_psa_dn12 = assign14530_e20328_d_n12;
        locals.var_psa_dn17 = assign14530_e20328_d_n17;
        locals.var_psa_rv = 0.0;

        let (assign14540_e20337, assign14540_e20337_d_n0, assign14540_e20337_d_n2, assign14540_e20337_d_n6, assign14540_e20337_d_n7, assign14540_e20337_d_n10, assign14540_e20337_d_n11, assign14540_e20337_d_n12, assign14540_e20337_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14540_e20335: f64 = (locals.var_psa - locals.var_vbcs_cl);
        (assign14540_e20335, (locals.var_psa_dn0 - locals.var_vbcs_cl_dn0), (locals.var_psa_dn2 - locals.var_vbcs_cl_dn2), (locals.var_psa_dn6 - locals.var_vbcs_cl_dn6), (locals.var_psa_dn7 - locals.var_vbcs_cl_dn7), (locals.var_psa_dn10 - locals.var_vbcs_cl_dn10), (locals.var_psa_dn11 - locals.var_vbcs_cl_dn11), (locals.var_psa_dn12 - locals.var_vbcs_cl_dn12), (locals.var_psa_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14540_e20337;
        locals.var_t1_dn0 = assign14540_e20337_d_n0;
        locals.var_t1_dn2 = assign14540_e20337_d_n2;
        locals.var_t1_dn6 = assign14540_e20337_d_n6;
        locals.var_t1_dn7 = assign14540_e20337_d_n7;
        locals.var_t1_dn10 = assign14540_e20337_d_n10;
        locals.var_t1_dn11 = assign14540_e20337_d_n11;
        locals.var_t1_dn12 = assign14540_e20337_d_n12;
        locals.var_t1_dn17 = assign14540_e20337_d_n17;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_50(
        locals: &mut StampLocals,
    ) {
        let (assign14550_e20346, assign14550_e20346_d_n0, assign14550_e20346_d_n2, assign14550_e20346_d_n6, assign14550_e20346_d_n7, assign14550_e20346_d_n10, assign14550_e20346_d_n11, assign14550_e20346_d_n12, assign14550_e20346_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14550_e20344: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign14550_e20344, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn17 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn17)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14550_e20346;
        locals.var_t2_dn0 = assign14550_e20346_d_n0;
        locals.var_t2_dn2 = assign14550_e20346_d_n2;
        locals.var_t2_dn6 = assign14550_e20346_d_n6;
        locals.var_t2_dn7 = assign14550_e20346_d_n7;
        locals.var_t2_dn10 = assign14550_e20346_d_n10;
        locals.var_t2_dn11 = assign14550_e20346_d_n11;
        locals.var_t2_dn12 = assign14550_e20346_d_n12;
        locals.var_t2_dn17 = assign14550_e20346_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign14560_e20358, assign14560_e20358_d_n0, assign14560_e20358_d_n2, assign14560_e20358_d_n6, assign14560_e20358_d_n7, assign14560_e20358_d_n10, assign14560_e20358_d_n11, assign14560_e20358_d_n12, assign14560_e20358_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14560_e20354: f64 = (locals.var_t2 * locals.var_t2);
        let assign14560_e20355: f64 = (1.0 + assign14560_e20354);
        let assign14560_e20356: f64 = (assign14560_e20355).sqrt();
        (assign14560_e20356, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)) / (2.0 * assign14560_e20356)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14560_e20358;
        locals.var_t3_dn0 = assign14560_e20358_d_n0;
        locals.var_t3_dn2 = assign14560_e20358_d_n2;
        locals.var_t3_dn6 = assign14560_e20358_d_n6;
        locals.var_t3_dn7 = assign14560_e20358_d_n7;
        locals.var_t3_dn10 = assign14560_e20358_d_n10;
        locals.var_t3_dn11 = assign14560_e20358_d_n11;
        locals.var_t3_dn12 = assign14560_e20358_d_n12;
        locals.var_t3_dn17 = assign14560_e20358_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign14570_e20369, assign14570_e20369_d_n0, assign14570_e20369_d_n2, assign14570_e20369_d_n6, assign14570_e20369_d_n7, assign14570_e20369_d_n10, assign14570_e20369_d_n11, assign14570_e20369_d_n12, assign14570_e20369_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14570_e20365: f64 = (locals.var_t1 / locals.var_t3);
        let assign14570_e20367: f64 = (assign14570_e20365 + locals.var_vbcs_cl);
        (assign14570_e20367, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn2), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn7), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn11), ((((locals.var_t1_dn12 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn12), ((((locals.var_t1_dn17 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14570_e20369;
        locals.var_ps0_dn0 = assign14570_e20369_d_n0;
        locals.var_ps0_dn2 = assign14570_e20369_d_n2;
        locals.var_ps0_dn6 = assign14570_e20369_d_n6;
        locals.var_ps0_dn7 = assign14570_e20369_d_n7;
        locals.var_ps0_dn10 = assign14570_e20369_d_n10;
        locals.var_ps0_dn11 = assign14570_e20369_d_n11;
        locals.var_ps0_dn12 = assign14570_e20369_d_n12;
        locals.var_ps0_dn17 = assign14570_e20369_d_n17;
        locals.var_ps0_rv = 0.0;

        let assign14580_e20372: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard444 = assign14580_e20372;
        locals.var_guard444_rv = 0.0;

        let (assign14590_e20382, assign14590_e20382_d_n0, assign14590_e20382_d_n2, assign14590_e20382_d_n6, assign14590_e20382_d_n7, assign14590_e20382_d_n10, assign14590_e20382_d_n11, assign14590_e20382_d_n12, assign14590_e20382_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign14590_e20382;
        locals.var_phi_s0_soi_dn0 = assign14590_e20382_d_n0;
        locals.var_phi_s0_soi_dn2 = assign14590_e20382_d_n2;
        locals.var_phi_s0_soi_dn6 = assign14590_e20382_d_n6;
        locals.var_phi_s0_soi_dn7 = assign14590_e20382_d_n7;
        locals.var_phi_s0_soi_dn10 = assign14590_e20382_d_n10;
        locals.var_phi_s0_soi_dn11 = assign14590_e20382_d_n11;
        locals.var_phi_s0_soi_dn12 = assign14590_e20382_d_n12;
        locals.var_phi_s0_soi_dn17 = assign14590_e20382_d_n17;
        locals.var_phi_s0_soi_rv = 0.0;

        let (assign14600_e20392, assign14600_e20392_d_n0, assign14600_e20392_d_n2, assign14600_e20392_d_n6, assign14600_e20392_d_n7, assign14600_e20392_d_n10, assign14600_e20392_d_n11, assign14600_e20392_d_n12, assign14600_e20392_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14600_e20392;
        locals.var_ps0_ini_dn0 = assign14600_e20392_d_n0;
        locals.var_ps0_ini_dn2 = assign14600_e20392_d_n2;
        locals.var_ps0_ini_dn6 = assign14600_e20392_d_n6;
        locals.var_ps0_ini_dn7 = assign14600_e20392_d_n7;
        locals.var_ps0_ini_dn10 = assign14600_e20392_d_n10;
        locals.var_ps0_ini_dn11 = assign14600_e20392_d_n11;
        locals.var_ps0_ini_dn12 = assign14600_e20392_d_n12;
        locals.var_ps0_ini_dn17 = assign14600_e20392_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14610_e20417, assign14610_e20417_d_n0, assign14610_e20417_d_n2, assign14610_e20417_d_n6, assign14610_e20417_d_n7, assign14610_e20417_d_n10, assign14610_e20417_d_n11, assign14610_e20417_d_n12, assign14610_e20417_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14610_e20406: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14610_e20407: f64 = (locals.var_beta * assign14610_e20406);
        let assign14610_e20409: f64 = (assign14610_e20407 - 1.0);
        let assign14610_e20410: f64 = (4.0 * assign14610_e20409);
        let assign14610_e20413: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign14610_e20414: f64 = (assign14610_e20410 / assign14610_e20413);
        let assign14610_e20415: f64 = (1.0 + assign14610_e20414);
        (assign14610_e20415, ((((4.0 * (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * ((locals.var_beta_dn10 * assign14610_e20406) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10)))) * assign14610_e20413) - (assign14610_e20410 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14610_e20417;
        locals.var_tx_dn0 = assign14610_e20417_d_n0;
        locals.var_tx_dn2 = assign14610_e20417_d_n2;
        locals.var_tx_dn6 = assign14610_e20417_d_n6;
        locals.var_tx_dn7 = assign14610_e20417_d_n7;
        locals.var_tx_dn10 = assign14610_e20417_d_n10;
        locals.var_tx_dn11 = assign14610_e20417_d_n11;
        locals.var_tx_dn12 = assign14610_e20417_d_n12;
        locals.var_tx_dn17 = assign14610_e20417_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14620_e20437, assign14620_e20437_d_n0, assign14620_e20437_d_n2, assign14620_e20437_d_n6, assign14620_e20437_d_n7, assign14620_e20437_d_n10, assign14620_e20437_d_n11, assign14620_e20437_d_n12, assign14620_e20437_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14620_e20429: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14620_e20435, assign14620_e20435_d_n0, assign14620_e20435_d_n2, assign14620_e20435_d_n6, assign14620_e20435_d_n7, assign14620_e20435_d_n10, assign14620_e20435_d_n11, assign14620_e20435_d_n12, assign14620_e20435_d_n17,) = {
            if (locals.var_tx >= assign14620_e20429) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
            } else {
                let assign14620_e20434: f64 = (10.0 * 2.220446049250313e-16);
                (assign14620_e20434, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14620_e20435, assign14620_e20435_d_n0, assign14620_e20435_d_n2, assign14620_e20435_d_n6, assign14620_e20435_d_n7, assign14620_e20435_d_n10, assign14620_e20435_d_n11, assign14620_e20435_d_n12, assign14620_e20435_d_n17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14620_e20437;
        locals.var_tx_dn0 = assign14620_e20437_d_n0;
        locals.var_tx_dn2 = assign14620_e20437_d_n2;
        locals.var_tx_dn6 = assign14620_e20437_d_n6;
        locals.var_tx_dn7 = assign14620_e20437_d_n7;
        locals.var_tx_dn10 = assign14620_e20437_d_n10;
        locals.var_tx_dn11 = assign14620_e20437_d_n11;
        locals.var_tx_dn12 = assign14620_e20437_d_n12;
        locals.var_tx_dn17 = assign14620_e20437_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14630_e20459, assign14630_e20459_d_n0, assign14630_e20459_d_n2, assign14630_e20459_d_n6, assign14630_e20459_d_n7, assign14630_e20459_d_n10, assign14630_e20459_d_n11, assign14630_e20459_d_n12, assign14630_e20459_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14630_e20449: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign14630_e20451: f64 = (assign14630_e20449 * 0.5);
        let assign14630_e20454: f64 = (locals.var_tx).sqrt();
        let assign14630_e20455: f64 = (1.0 - assign14630_e20454);
        let assign14630_e20456: f64 = (assign14630_e20451 * assign14630_e20455);
        let assign14630_e20457: f64 = (locals.var_vgp + assign14630_e20456);
        (assign14630_e20457, (locals.var_vgp_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn0 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn2 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn6 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn7 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn10 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn11 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn12 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn17 / (2.0 * assign14630_e20454)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14630_e20459;
        locals.var_ps0_inia_dn0 = assign14630_e20459_d_n0;
        locals.var_ps0_inia_dn2 = assign14630_e20459_d_n2;
        locals.var_ps0_inia_dn6 = assign14630_e20459_d_n6;
        locals.var_ps0_inia_dn7 = assign14630_e20459_d_n7;
        locals.var_ps0_inia_dn10 = assign14630_e20459_d_n10;
        locals.var_ps0_inia_dn11 = assign14630_e20459_d_n11;
        locals.var_ps0_inia_dn12 = assign14630_e20459_d_n12;
        locals.var_ps0_inia_dn17 = assign14630_e20459_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign14640_e20474, assign14640_e20474_d_n0, assign14640_e20474_d_n2, assign14640_e20474_d_n6, assign14640_e20474_d_n7, assign14640_e20474_d_n10, assign14640_e20474_d_n11, assign14640_e20474_d_n12, assign14640_e20474_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14640_e20471: f64 = (locals.var_ps0_inia - locals.var_vbcs_cl);
        let assign14640_e20472: f64 = (locals.var_beta * assign14640_e20471);
        (assign14640_e20472, (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14640_e20471) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_ps0_inia_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14640_e20474;
        locals.var_chi_dn0 = assign14640_e20474_d_n0;
        locals.var_chi_dn2 = assign14640_e20474_d_n2;
        locals.var_chi_dn6 = assign14640_e20474_d_n6;
        locals.var_chi_dn7 = assign14640_e20474_d_n7;
        locals.var_chi_dn10 = assign14640_e20474_d_n10;
        locals.var_chi_dn11 = assign14640_e20474_d_n11;
        locals.var_chi_dn12 = assign14640_e20474_d_n12;
        locals.var_chi_dn17 = assign14640_e20474_d_n17;
        locals.var_chi_rv = 0.0;

        let assign14650_e20477: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign14650_e20477;
        locals.var_guard445_rv = 0.0;

        let (assign14660_e20494, assign14660_e20494_d_n0, assign14660_e20494_d_n2, assign14660_e20494_d_n6, assign14660_e20494_d_n7, assign14660_e20494_d_n10, assign14660_e20494_d_n11, assign14660_e20494_d_n12, assign14660_e20494_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14660_e20491: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14660_e20492: f64 = (locals.var_beta * assign14660_e20491);
        (assign14660_e20492, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14660_e20491) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14660_e20494;
        locals.var_ty_dn0 = assign14660_e20494_d_n0;
        locals.var_ty_dn2 = assign14660_e20494_d_n2;
        locals.var_ty_dn6 = assign14660_e20494_d_n6;
        locals.var_ty_dn7 = assign14660_e20494_d_n7;
        locals.var_ty_dn10 = assign14660_e20494_d_n10;
        locals.var_ty_dn11 = assign14660_e20494_d_n11;
        locals.var_ty_dn12 = assign14660_e20494_d_n12;
        locals.var_ty_dn17 = assign14660_e20494_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign14670_e20515, assign14670_e20515_d_n0, assign14670_e20515_d_n2, assign14670_e20515_d_n6, assign14670_e20515_d_n7, assign14670_e20515_d_n10, assign14670_e20515_d_n11, assign14670_e20515_d_n12, assign14670_e20515_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14670_e20508: f64 = (1.414213562373095 / 108.0);
        let assign14670_e20510: f64 = (assign14670_e20508 * locals.var_beta);
        let assign14670_e20512: f64 = (assign14670_e20510 * locals.var_fac1);
        let assign14670_e20513: f64 = (1.0 / assign14670_e20512);
        (assign14670_e20513, (-((assign14670_e20510 * locals.var_fac1_dn0) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn2) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn6) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn7) / (assign14670_e20512 * assign14670_e20512))), (-((((assign14670_e20508 * locals.var_beta_dn10) * locals.var_fac1) + (assign14670_e20510 * locals.var_fac1_dn10)) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn11) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn12) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn17) / (assign14670_e20512 * assign14670_e20512))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14670_e20515;
        locals.var_t1_dn0 = assign14670_e20515_d_n0;
        locals.var_t1_dn2 = assign14670_e20515_d_n2;
        locals.var_t1_dn6 = assign14670_e20515_d_n6;
        locals.var_t1_dn7 = assign14670_e20515_d_n7;
        locals.var_t1_dn10 = assign14670_e20515_d_n10;
        locals.var_t1_dn11 = assign14670_e20515_d_n11;
        locals.var_t1_dn12 = assign14670_e20515_d_n12;
        locals.var_t1_dn17 = assign14670_e20515_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14680_e20532, assign14680_e20532_d_n0, assign14680_e20532_d_n2, assign14680_e20532_d_n6, assign14680_e20532_d_n7, assign14680_e20532_d_n10, assign14680_e20532_d_n11, assign14680_e20532_d_n12, assign14680_e20532_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14680_e20529: f64 = (3.0 * locals.var_t1);
        let assign14680_e20530: f64 = (81.0 + assign14680_e20529);
        (assign14680_e20530, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn12), (3.0 * locals.var_t1_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14680_e20532;
        locals.var_t2_dn0 = assign14680_e20532_d_n0;
        locals.var_t2_dn2 = assign14680_e20532_d_n2;
        locals.var_t2_dn6 = assign14680_e20532_d_n6;
        locals.var_t2_dn7 = assign14680_e20532_d_n7;
        locals.var_t2_dn10 = assign14680_e20532_d_n10;
        locals.var_t2_dn11 = assign14680_e20532_d_n11;
        locals.var_t2_dn12 = assign14680_e20532_d_n12;
        locals.var_t2_dn17 = assign14680_e20532_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign14690_e20556, assign14690_e20556_d_n0, assign14690_e20556_d_n2, assign14690_e20556_d_n6, assign14690_e20556_d_n7, assign14690_e20556_d_n10, assign14690_e20556_d_n11, assign14690_e20556_d_n12, assign14690_e20556_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14690_e20544: f64 = (-2916.0);
        let assign14690_e20547: f64 = (81.0 * locals.var_t1);
        let assign14690_e20548: f64 = (assign14690_e20544 - assign14690_e20547);
        let assign14690_e20551: f64 = (27.0 * locals.var_t1);
        let assign14690_e20553: f64 = (assign14690_e20551 * locals.var_ty);
        let assign14690_e20554: f64 = (assign14690_e20548 + assign14690_e20553);
        (assign14690_e20554, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14690_e20556;
        locals.var_t3_dn0 = assign14690_e20556_d_n0;
        locals.var_t3_dn2 = assign14690_e20556_d_n2;
        locals.var_t3_dn6 = assign14690_e20556_d_n6;
        locals.var_t3_dn7 = assign14690_e20556_d_n7;
        locals.var_t3_dn10 = assign14690_e20556_d_n10;
        locals.var_t3_dn11 = assign14690_e20556_d_n11;
        locals.var_t3_dn12 = assign14690_e20556_d_n12;
        locals.var_t3_dn17 = assign14690_e20556_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign14700_e20581, assign14700_e20581_d_n0, assign14700_e20581_d_n2, assign14700_e20581_d_n6, assign14700_e20581_d_n7, assign14700_e20581_d_n10, assign14700_e20581_d_n11, assign14700_e20581_d_n12, assign14700_e20581_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14700_e20571: f64 = (54.0 + locals.var_t1);
        let assign14700_e20572: f64 = (81.0 * assign14700_e20571);
        let assign14700_e20573: f64 = (1458.0 - assign14700_e20572);
        let assign14700_e20576: f64 = (27.0 * locals.var_t1);
        let assign14700_e20578: f64 = (assign14700_e20576 * locals.var_ty);
        let assign14700_e20579: f64 = (assign14700_e20573 + assign14700_e20578);
        (assign14700_e20579, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14700_e20581;
        locals.var_t4_dn0 = assign14700_e20581_d_n0;
        locals.var_t4_dn2 = assign14700_e20581_d_n2;
        locals.var_t4_dn6 = assign14700_e20581_d_n6;
        locals.var_t4_dn7 = assign14700_e20581_d_n7;
        locals.var_t4_dn10 = assign14700_e20581_d_n10;
        locals.var_t4_dn11 = assign14700_e20581_d_n11;
        locals.var_t4_dn12 = assign14700_e20581_d_n12;
        locals.var_t4_dn17 = assign14700_e20581_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign14710_e20596, assign14710_e20596_d_n0, assign14710_e20596_d_n2, assign14710_e20596_d_n6, assign14710_e20596_d_n7, assign14710_e20596_d_n10, assign14710_e20596_d_n11, assign14710_e20596_d_n12, assign14710_e20596_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14710_e20594: f64 = (locals.var_t4 * locals.var_t4);
        (assign14710_e20594, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)), ((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14710_e20596;
        locals.var_t4_dn0 = assign14710_e20596_d_n0;
        locals.var_t4_dn2 = assign14710_e20596_d_n2;
        locals.var_t4_dn6 = assign14710_e20596_d_n6;
        locals.var_t4_dn7 = assign14710_e20596_d_n7;
        locals.var_t4_dn10 = assign14710_e20596_d_n10;
        locals.var_t4_dn11 = assign14710_e20596_d_n11;
        locals.var_t4_dn12 = assign14710_e20596_d_n12;
        locals.var_t4_dn17 = assign14710_e20596_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign14720_e20622, assign14720_e20622_d_n0, assign14720_e20622_d_n2, assign14720_e20622_d_n6, assign14720_e20622_d_n7, assign14720_e20622_d_n10, assign14720_e20622_d_n11, assign14720_e20622_d_n12, assign14720_e20622_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14720_e20610: f64 = (4.0 * locals.var_t2);
        let assign14720_e20612: f64 = (assign14720_e20610 * locals.var_t2);
        let assign14720_e20614: f64 = (assign14720_e20612 * locals.var_t2);
        let assign14720_e20616: f64 = (assign14720_e20614 + locals.var_t4);
        let assign14720_e20617: f64 = (assign14720_e20616).sqrt();
        let assign14720_e20618: f64 = (locals.var_t3 + assign14720_e20617);
        let assign14720_e20620: f64 = (assign14720_e20618).powf(0.3333333333333333);
        (assign14720_e20620, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn0)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn0)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn2)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn2)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn6)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn6)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn7)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn7)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn10)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn10)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn11)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn11)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn12)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn12)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn17)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn17)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14720_e20617))) / assign14720_e20618))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14720_e20622;
        locals.var_t5_dn0 = assign14720_e20622_d_n0;
        locals.var_t5_dn2 = assign14720_e20622_d_n2;
        locals.var_t5_dn6 = assign14720_e20622_d_n6;
        locals.var_t5_dn7 = assign14720_e20622_d_n7;
        locals.var_t5_dn10 = assign14720_e20622_d_n10;
        locals.var_t5_dn11 = assign14720_e20622_d_n11;
        locals.var_t5_dn12 = assign14720_e20622_d_n12;
        locals.var_t5_dn17 = assign14720_e20622_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign14730_e20651, assign14730_e20651_d_n0, assign14730_e20651_d_n2, assign14730_e20651_d_n6, assign14730_e20651_d_n7, assign14730_e20651_d_n10, assign14730_e20651_d_n11, assign14730_e20651_d_n12, assign14730_e20651_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14730_e20636: f64 = (1.259921049894873 * locals.var_t2);
        let assign14730_e20639: f64 = (3.0 * locals.var_t5);
        let assign14730_e20640: f64 = (assign14730_e20636 / assign14730_e20639);
        let assign14730_e20641: f64 = (3.0 - assign14730_e20640);
        let assign14730_e20645: f64 = (3.0 * 1.259921049894873);
        let assign14730_e20646: f64 = (1.0 / assign14730_e20645);
        let assign14730_e20648: f64 = (assign14730_e20646 * locals.var_t5);
        let assign14730_e20649: f64 = (assign14730_e20641 + assign14730_e20648);
        (assign14730_e20649, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn0))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn2))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn6))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn7))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn10))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn11))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn12) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn12))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn12)), ((-((((1.259921049894873 * locals.var_t2_dn17) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn17))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14730_e20651;
        locals.var_tx_dn0 = assign14730_e20651_d_n0;
        locals.var_tx_dn2 = assign14730_e20651_d_n2;
        locals.var_tx_dn6 = assign14730_e20651_d_n6;
        locals.var_tx_dn7 = assign14730_e20651_d_n7;
        locals.var_tx_dn10 = assign14730_e20651_d_n10;
        locals.var_tx_dn11 = assign14730_e20651_d_n11;
        locals.var_tx_dn12 = assign14730_e20651_d_n12;
        locals.var_tx_dn17 = assign14730_e20651_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14740_e20668, assign14740_e20668_d_n0, assign14740_e20668_d_n2, assign14740_e20668_d_n6, assign14740_e20668_d_n7, assign14740_e20668_d_n10, assign14740_e20668_d_n11, assign14740_e20668_d_n12, assign14740_e20668_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14740_e20664: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign14740_e20666: f64 = (assign14740_e20664 + locals.var_vbcs_cl);
        (assign14740_e20666, ((locals.var_tx_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_tx_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_tx_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_tx_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14740_e20668;
        locals.var_ps0_inia_dn0 = assign14740_e20668_d_n0;
        locals.var_ps0_inia_dn2 = assign14740_e20668_d_n2;
        locals.var_ps0_inia_dn6 = assign14740_e20668_d_n6;
        locals.var_ps0_inia_dn7 = assign14740_e20668_d_n7;
        locals.var_ps0_inia_dn10 = assign14740_e20668_d_n10;
        locals.var_ps0_inia_dn11 = assign14740_e20668_d_n11;
        locals.var_ps0_inia_dn12 = assign14740_e20668_d_n12;
        locals.var_ps0_inia_dn17 = assign14740_e20668_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign14750_e20681, assign14750_e20681_d_n0, assign14750_e20681_d_n2, assign14750_e20681_d_n6, assign14750_e20681_d_n7, assign14750_e20681_d_n10, assign14750_e20681_d_n11, assign14750_e20681_d_n12, assign14750_e20681_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14750_e20681;
        locals.var_ps0_ini_dn0 = assign14750_e20681_d_n0;
        locals.var_ps0_ini_dn2 = assign14750_e20681_d_n2;
        locals.var_ps0_ini_dn6 = assign14750_e20681_d_n6;
        locals.var_ps0_ini_dn7 = assign14750_e20681_d_n7;
        locals.var_ps0_ini_dn10 = assign14750_e20681_d_n10;
        locals.var_ps0_ini_dn11 = assign14750_e20681_d_n11;
        locals.var_ps0_ini_dn12 = assign14750_e20681_d_n12;
        locals.var_ps0_ini_dn17 = assign14750_e20681_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign14760_e20684: f64 = if locals.var_vgs <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard446 = assign14760_e20684;
        locals.var_guard446_rv = 0.0;

        let (assign14770_e20700, assign14770_e20700_d_n0, assign14770_e20700_d_n2, assign14770_e20700_d_n6, assign14770_e20700_d_n7, assign14770_e20700_d_n10, assign14770_e20700_d_n11, assign14770_e20700_d_n12, assign14770_e20700_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14770_e20700;
        locals.var_ps0_ini_dn0 = assign14770_e20700_d_n0;
        locals.var_ps0_ini_dn2 = assign14770_e20700_d_n2;
        locals.var_ps0_ini_dn6 = assign14770_e20700_d_n6;
        locals.var_ps0_ini_dn7 = assign14770_e20700_d_n7;
        locals.var_ps0_ini_dn10 = assign14770_e20700_d_n10;
        locals.var_ps0_ini_dn11 = assign14770_e20700_d_n11;
        locals.var_ps0_ini_dn12 = assign14770_e20700_d_n12;
        locals.var_ps0_ini_dn17 = assign14770_e20700_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14780_e20721, assign14780_e20721_d_n0, assign14780_e20721_d_n2, assign14780_e20721_d_n6, assign14780_e20721_d_n7, assign14780_e20721_d_n10, assign14780_e20721_d_n11, assign14780_e20721_d_n12, assign14780_e20721_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14780_e20717: f64 = (1.0 / locals.var_cnst1soi);
        let assign14780_e20719: f64 = (assign14780_e20717 / locals.var_cnstc_foxi);
        (assign14780_e20719, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14780_e20721;
        locals.var_t1_dn0 = assign14780_e20721_d_n0;
        locals.var_t1_dn2 = assign14780_e20721_d_n2;
        locals.var_t1_dn6 = assign14780_e20721_d_n6;
        locals.var_t1_dn7 = assign14780_e20721_d_n7;
        locals.var_t1_dn10 = assign14780_e20721_d_n10;
        locals.var_t1_dn11 = assign14780_e20721_d_n11;
        locals.var_t1_dn12 = assign14780_e20721_d_n12;
        locals.var_t1_dn17 = assign14780_e20721_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14790_e20742, assign14790_e20742_d_n0, assign14790_e20742_d_n2, assign14790_e20742_d_n6, assign14790_e20742_d_n7, assign14790_e20742_d_n10, assign14790_e20742_d_n11, assign14790_e20742_d_n12, assign14790_e20742_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14790_e20738: f64 = (locals.var_t1 * locals.var_vgp);
        let assign14790_e20740: f64 = (assign14790_e20738 * locals.var_vgp);
        (assign14790_e20740, ((((locals.var_t1_dn0 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn0)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn2)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn2)), ((((locals.var_t1_dn6 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn6)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn7)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn7)), ((((locals.var_t1_dn10 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn10)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn10)), ((((locals.var_t1_dn11 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn11)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn11)), ((((locals.var_t1_dn12 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn12)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn12)), ((((locals.var_t1_dn17 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn17)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14790_e20742;
        locals.var_t2_dn0 = assign14790_e20742_d_n0;
        locals.var_t2_dn2 = assign14790_e20742_d_n2;
        locals.var_t2_dn6 = assign14790_e20742_d_n6;
        locals.var_t2_dn7 = assign14790_e20742_d_n7;
        locals.var_t2_dn10 = assign14790_e20742_d_n10;
        locals.var_t2_dn11 = assign14790_e20742_d_n11;
        locals.var_t2_dn12 = assign14790_e20742_d_n12;
        locals.var_t2_dn17 = assign14790_e20742_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign14800_e20763, assign14800_e20763_d_n0, assign14800_e20763_d_n2, assign14800_e20763_d_n6, assign14800_e20763_d_n7, assign14800_e20763_d_n10, assign14800_e20763_d_n11, assign14800_e20763_d_n12, assign14800_e20763_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14800_e20760: f64 = (2.0 / locals.var_vgp);
        let assign14800_e20761: f64 = (locals.var_beta + assign14800_e20760);
        (assign14800_e20761, (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))), (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn12) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn17) / (locals.var_vgp * locals.var_vgp))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14800_e20763;
        locals.var_t3_dn0 = assign14800_e20763_d_n0;
        locals.var_t3_dn2 = assign14800_e20763_d_n2;
        locals.var_t3_dn6 = assign14800_e20763_d_n6;
        locals.var_t3_dn7 = assign14800_e20763_d_n7;
        locals.var_t3_dn10 = assign14800_e20763_d_n10;
        locals.var_t3_dn11 = assign14800_e20763_d_n11;
        locals.var_t3_dn12 = assign14800_e20763_d_n12;
        locals.var_t3_dn17 = assign14800_e20763_d_n17;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_51(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let (assign14810_e20783, assign14810_e20783_d_n0, assign14810_e20783_d_n2, assign14810_e20783_d_n6, assign14810_e20783_d_n7, assign14810_e20783_d_n10, assign14810_e20783_d_n11, assign14810_e20783_d_n12, assign14810_e20783_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14810_e20779: f64 = (locals.var_t2).ln();
        let assign14810_e20781: f64 = (assign14810_e20779 / locals.var_t3);
        (assign14810_e20781, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign14810_e20783;
        locals.var_ps0_inib_dn0 = assign14810_e20783_d_n0;
        locals.var_ps0_inib_dn2 = assign14810_e20783_d_n2;
        locals.var_ps0_inib_dn6 = assign14810_e20783_d_n6;
        locals.var_ps0_inib_dn7 = assign14810_e20783_d_n7;
        locals.var_ps0_inib_dn10 = assign14810_e20783_d_n10;
        locals.var_ps0_inib_dn11 = assign14810_e20783_d_n11;
        locals.var_ps0_inib_dn12 = assign14810_e20783_d_n12;
        locals.var_ps0_inib_dn17 = assign14810_e20783_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let (assign14820_e20804, assign14820_e20804_d_n0, assign14820_e20804_d_n2, assign14820_e20804_d_n6, assign14820_e20804_d_n7, assign14820_e20804_d_n10, assign14820_e20804_d_n11, assign14820_e20804_d_n12, assign14820_e20804_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14820_e20800: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign14820_e20802: f64 = (assign14820_e20800 - 0.0008);
        (assign14820_e20802, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn12 - locals.var_ps0_inia_dn12), (locals.var_ps0_inib_dn17 - locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign14820_e20804;
        locals.var_tmf1_dn0 = assign14820_e20804_d_n0;
        locals.var_tmf1_dn2 = assign14820_e20804_d_n2;
        locals.var_tmf1_dn6 = assign14820_e20804_d_n6;
        locals.var_tmf1_dn7 = assign14820_e20804_d_n7;
        locals.var_tmf1_dn10 = assign14820_e20804_d_n10;
        locals.var_tmf1_dn11 = assign14820_e20804_d_n11;
        locals.var_tmf1_dn12 = assign14820_e20804_d_n12;
        locals.var_tmf1_dn17 = assign14820_e20804_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign14830_e20825, assign14830_e20825_d_n0, assign14830_e20825_d_n2, assign14830_e20825_d_n6, assign14830_e20825_d_n7, assign14830_e20825_d_n10, assign14830_e20825_d_n11, assign14830_e20825_d_n12, assign14830_e20825_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14830_e20821: f64 = (4.0 * locals.var_ps0_inib);
        let assign14830_e20823: f64 = (assign14830_e20821 * 0.0008);
        (assign14830_e20823, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn12) * 0.0008), ((4.0 * locals.var_ps0_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14830_e20825;
        locals.var_tmf2_dn0 = assign14830_e20825_d_n0;
        locals.var_tmf2_dn2 = assign14830_e20825_d_n2;
        locals.var_tmf2_dn6 = assign14830_e20825_d_n6;
        locals.var_tmf2_dn7 = assign14830_e20825_d_n7;
        locals.var_tmf2_dn10 = assign14830_e20825_d_n10;
        locals.var_tmf2_dn11 = assign14830_e20825_d_n11;
        locals.var_tmf2_dn12 = assign14830_e20825_d_n12;
        locals.var_tmf2_dn17 = assign14830_e20825_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign14840_e20848, assign14840_e20848_d_n0, assign14840_e20848_d_n2, assign14840_e20848_d_n6, assign14840_e20848_d_n7, assign14840_e20848_d_n10, assign14840_e20848_d_n11, assign14840_e20848_d_n12, assign14840_e20848_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let (assign14840_e20846, assign14840_e20846_d_n0, assign14840_e20846_d_n2, assign14840_e20846_d_n6, assign14840_e20846_d_n7, assign14840_e20846_d_n10, assign14840_e20846_d_n11, assign14840_e20846_d_n12, assign14840_e20846_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign14840_e20845: f64 = (-locals.var_tmf2);
                (assign14840_e20845, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign14840_e20846, assign14840_e20846_d_n0, assign14840_e20846_d_n2, assign14840_e20846_d_n6, assign14840_e20846_d_n7, assign14840_e20846_d_n10, assign14840_e20846_d_n11, assign14840_e20846_d_n12, assign14840_e20846_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14840_e20848;
        locals.var_tmf2_dn0 = assign14840_e20848_d_n0;
        locals.var_tmf2_dn2 = assign14840_e20848_d_n2;
        locals.var_tmf2_dn6 = assign14840_e20848_d_n6;
        locals.var_tmf2_dn7 = assign14840_e20848_d_n7;
        locals.var_tmf2_dn10 = assign14840_e20848_d_n10;
        locals.var_tmf2_dn11 = assign14840_e20848_d_n11;
        locals.var_tmf2_dn12 = assign14840_e20848_d_n12;
        locals.var_tmf2_dn17 = assign14840_e20848_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign14850_e20870, assign14850_e20870_d_n0, assign14850_e20870_d_n2, assign14850_e20870_d_n6, assign14850_e20870_d_n7, assign14850_e20870_d_n10, assign14850_e20870_d_n11, assign14850_e20870_d_n12, assign14850_e20870_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14850_e20865: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14850_e20867: f64 = (assign14850_e20865 + locals.var_tmf2);
        let assign14850_e20868: f64 = (assign14850_e20867).sqrt();
        (assign14850_e20868, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign14850_e20868)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14850_e20870;
        locals.var_tmf2_dn0 = assign14850_e20870_d_n0;
        locals.var_tmf2_dn2 = assign14850_e20870_d_n2;
        locals.var_tmf2_dn6 = assign14850_e20870_d_n6;
        locals.var_tmf2_dn7 = assign14850_e20870_d_n7;
        locals.var_tmf2_dn10 = assign14850_e20870_d_n10;
        locals.var_tmf2_dn11 = assign14850_e20870_d_n11;
        locals.var_tmf2_dn12 = assign14850_e20870_d_n12;
        locals.var_tmf2_dn17 = assign14850_e20870_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign14860_e20893, assign14860_e20893_d_n0, assign14860_e20893_d_n2, assign14860_e20893_d_n6, assign14860_e20893_d_n7, assign14860_e20893_d_n10, assign14860_e20893_d_n11, assign14860_e20893_d_n12, assign14860_e20893_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14860_e20889: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14860_e20890: f64 = (0.5 * assign14860_e20889);
        let assign14860_e20891: f64 = (locals.var_ps0_inib - assign14860_e20890);
        (assign14860_e20891, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_ps0_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14860_e20893;
        locals.var_ps0_ini_dn0 = assign14860_e20893_d_n0;
        locals.var_ps0_ini_dn2 = assign14860_e20893_d_n2;
        locals.var_ps0_ini_dn6 = assign14860_e20893_d_n6;
        locals.var_ps0_ini_dn7 = assign14860_e20893_d_n7;
        locals.var_ps0_ini_dn10 = assign14860_e20893_d_n10;
        locals.var_ps0_ini_dn11 = assign14860_e20893_d_n11;
        locals.var_ps0_ini_dn12 = assign14860_e20893_d_n12;
        locals.var_ps0_ini_dn17 = assign14860_e20893_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14870_e20908, assign14870_e20908_d_n0, assign14870_e20908_d_n2, assign14870_e20908_d_n6, assign14870_e20908_d_n7, assign14870_e20908_d_n10, assign14870_e20908_d_n11, assign14870_e20908_d_n12, assign14870_e20908_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14870_e20905: f64 = (5e-12 / 2.0);
        let assign14870_e20906: f64 = (locals.var_vbcs_cl + assign14870_e20905);
        (assign14870_e20906, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14870_e20908;
        locals.var_tx_dn0 = assign14870_e20908_d_n0;
        locals.var_tx_dn2 = assign14870_e20908_d_n2;
        locals.var_tx_dn6 = assign14870_e20908_d_n6;
        locals.var_tx_dn7 = assign14870_e20908_d_n7;
        locals.var_tx_dn10 = assign14870_e20908_d_n10;
        locals.var_tx_dn11 = assign14870_e20908_d_n11;
        locals.var_tx_dn12 = assign14870_e20908_d_n12;
        locals.var_tx_dn17 = assign14870_e20908_d_n17;
        locals.var_tx_rv = 0.0;

        let assign14880_e20911: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard447 = assign14880_e20911;
        locals.var_guard447_rv = 0.0;

        let (assign14890_e20924, assign14890_e20924_d_n0, assign14890_e20924_d_n2, assign14890_e20924_d_n6, assign14890_e20924_d_n7, assign14890_e20924_d_n10, assign14890_e20924_d_n11, assign14890_e20924_d_n12, assign14890_e20924_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard447 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14890_e20924;
        locals.var_ps0_ini_dn0 = assign14890_e20924_d_n0;
        locals.var_ps0_ini_dn2 = assign14890_e20924_d_n2;
        locals.var_ps0_ini_dn6 = assign14890_e20924_d_n6;
        locals.var_ps0_ini_dn7 = assign14890_e20924_d_n7;
        locals.var_ps0_ini_dn10 = assign14890_e20924_d_n10;
        locals.var_ps0_ini_dn11 = assign14890_e20924_d_n11;
        locals.var_ps0_ini_dn12 = assign14890_e20924_d_n12;
        locals.var_ps0_ini_dn17 = assign14890_e20924_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14900_e20932, assign14900_e20932_d_n0, assign14900_e20932_d_n2, assign14900_e20932_d_n6, assign14900_e20932_d_n7, assign14900_e20932_d_n10, assign14900_e20932_d_n11, assign14900_e20932_d_n12, assign14900_e20932_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14900_e20932;
        locals.var_ps0_dn0 = assign14900_e20932_d_n0;
        locals.var_ps0_dn2 = assign14900_e20932_d_n2;
        locals.var_ps0_dn6 = assign14900_e20932_d_n6;
        locals.var_ps0_dn7 = assign14900_e20932_d_n7;
        locals.var_ps0_dn10 = assign14900_e20932_d_n10;
        locals.var_ps0_dn11 = assign14900_e20932_d_n11;
        locals.var_ps0_dn12 = assign14900_e20932_d_n12;
        locals.var_ps0_dn17 = assign14900_e20932_d_n17;
        locals.var_ps0_rv = 0.0;

        let (assign14910_e20940, assign14910_e20940_d_n0, assign14910_e20940_d_n2, assign14910_e20940_d_n6, assign14910_e20940_d_n7, assign14910_e20940_d_n10, assign14910_e20940_d_n11, assign14910_e20940_d_n12, assign14910_e20940_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign14910_e20940;
        locals.var_psl_lim_dn0 = assign14910_e20940_d_n0;
        locals.var_psl_lim_dn2 = assign14910_e20940_d_n2;
        locals.var_psl_lim_dn6 = assign14910_e20940_d_n6;
        locals.var_psl_lim_dn7 = assign14910_e20940_d_n7;
        locals.var_psl_lim_dn10 = assign14910_e20940_d_n10;
        locals.var_psl_lim_dn11 = assign14910_e20940_d_n11;
        locals.var_psl_lim_dn12 = assign14910_e20940_d_n12;
        locals.var_psl_lim_dn17 = assign14910_e20940_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let assign14920_e20947: f64 = if ((p.p25 == 1.0) && (p.p26 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard448 = assign14920_e20947;
        locals.var_guard448_rv = 0.0;

        let (assign14930_e20958, assign14930_e20958_d_n0, assign14930_e20958_d_n2, assign14930_e20958_d_n6, assign14930_e20958_d_n7, assign14930_e20958_d_n10, assign14930_e20958_d_n11, assign14930_e20958_d_n12, assign14930_e20958_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard448 != 0.0)) {
        let assign14930_e20954: f64 = (1e-9 / 0.0001);
        let assign14930_e20956: f64 = (assign14930_e20954 * (nv17 - 0.0));
        (assign14930_e20956, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign14930_e20954,)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign14930_e20958;
        locals.var_qhs_dn0 = assign14930_e20958_d_n0;
        locals.var_qhs_dn2 = assign14930_e20958_d_n2;
        locals.var_qhs_dn6 = assign14930_e20958_d_n6;
        locals.var_qhs_dn7 = assign14930_e20958_d_n7;
        locals.var_qhs_dn10 = assign14930_e20958_d_n10;
        locals.var_qhs_dn11 = assign14930_e20958_d_n11;
        locals.var_qhs_dn12 = assign14930_e20958_d_n12;
        locals.var_qhs_dn17 = assign14930_e20958_d_n17;
        locals.var_qhs_rv = 0.0;

        let (assign14940_e20966, assign14940_e20966_d_n0, assign14940_e20966_d_n2, assign14940_e20966_d_n6, assign14940_e20966_d_n7, assign14940_e20966_d_n10, assign14940_e20966_d_n11, assign14940_e20966_d_n12, assign14940_e20966_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard448 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign14940_e20966;
        locals.var_qhs_dn0 = assign14940_e20966_d_n0;
        locals.var_qhs_dn2 = assign14940_e20966_d_n2;
        locals.var_qhs_dn6 = assign14940_e20966_d_n6;
        locals.var_qhs_dn7 = assign14940_e20966_d_n7;
        locals.var_qhs_dn10 = assign14940_e20966_d_n10;
        locals.var_qhs_dn11 = assign14940_e20966_d_n11;
        locals.var_qhs_dn12 = assign14940_e20966_d_n12;
        locals.var_qhs_dn17 = assign14940_e20966_d_n17;
        locals.var_qhs_rv = 0.0;

        let (assign14960_e20979, assign14960_e20979_d_n0, assign14960_e20979_d_n2, assign14960_e20979_d_n6, assign14960_e20979_d_n7, assign14960_e20979_d_n10, assign14960_e20979_d_n11, assign14960_e20979_d_n12, assign14960_e20979_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign14960_e20976: f64 = (locals.var_beta * locals.var_vbcs_cl);
        let assign14960_e20977: f64 = (assign14960_e20976).exp();
        (assign14960_e20977, (assign14960_e20977 * (locals.var_beta * locals.var_vbcs_cl_dn0)), (assign14960_e20977 * (locals.var_beta * locals.var_vbcs_cl_dn2)), (assign14960_e20977 * (locals.var_beta * locals.var_vbcs_cl_dn6)), (assign14960_e20977 * (locals.var_beta * locals.var_vbcs_cl_dn7)), (assign14960_e20977 * ((locals.var_beta_dn10 * locals.var_vbcs_cl) + (locals.var_beta * locals.var_vbcs_cl_dn10))), (assign14960_e20977 * (locals.var_beta * locals.var_vbcs_cl_dn11)), (assign14960_e20977 * (locals.var_beta * locals.var_vbcs_cl_dn12)), (assign14960_e20977 * (locals.var_beta * locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn12, locals.var_exp_bvbs_dn17,)
    }
};
        locals.var_exp_bvbs = assign14960_e20979;
        locals.var_exp_bvbs_dn0 = assign14960_e20979_d_n0;
        locals.var_exp_bvbs_dn2 = assign14960_e20979_d_n2;
        locals.var_exp_bvbs_dn6 = assign14960_e20979_d_n6;
        locals.var_exp_bvbs_dn7 = assign14960_e20979_d_n7;
        locals.var_exp_bvbs_dn10 = assign14960_e20979_d_n10;
        locals.var_exp_bvbs_dn11 = assign14960_e20979_d_n11;
        locals.var_exp_bvbs_dn12 = assign14960_e20979_d_n12;
        locals.var_exp_bvbs_dn17 = assign14960_e20979_d_n17;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign14970_e20986, assign14970_e20986_d_n0, assign14970_e20986_d_n2, assign14970_e20986_d_n6, assign14970_e20986_d_n7, assign14970_e20986_d_n10, assign14970_e20986_d_n11, assign14970_e20986_d_n12, assign14970_e20986_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign14970_e20984: f64 = (locals.var_cnst1soi * locals.var_exp_bvbs);
        (assign14970_e20984, ((locals.var_cnst1soi_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1soi_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1soi_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1soi_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1soi_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1soi_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1soi_dn12 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn12)), ((locals.var_cnst1soi_dn17 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn17)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn12, locals.var_cfs1_dn17,)
    }
};
        locals.var_cfs1 = assign14970_e20986;
        locals.var_cfs1_dn0 = assign14970_e20986_d_n0;
        locals.var_cfs1_dn2 = assign14970_e20986_d_n2;
        locals.var_cfs1_dn6 = assign14970_e20986_d_n6;
        locals.var_cfs1_dn7 = assign14970_e20986_d_n7;
        locals.var_cfs1_dn10 = assign14970_e20986_d_n10;
        locals.var_cfs1_dn11 = assign14970_e20986_d_n11;
        locals.var_cfs1_dn12 = assign14970_e20986_d_n12;
        locals.var_cfs1_dn17 = assign14970_e20986_d_n17;
        locals.var_cfs1_rv = 0.0;

        let (assign14980_e20991,) = {
    if (locals.var_guard111 == 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign14980_e20991;
        locals.var_flg_conv_rv = 0.0;

        let (assign14990_e20996, assign14990_e20996_d_n0, assign14990_e20996_d_n2, assign14990_e20996_d_n6, assign14990_e20996_d_n7, assign14990_e20996_d_n10, assign14990_e20996_d_n11, assign14990_e20996_d_n12, assign14990_e20996_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign14990_e20996;
        locals.var_phi_s0_soi_dn0 = assign14990_e20996_d_n0;
        locals.var_phi_s0_soi_dn2 = assign14990_e20996_d_n2;
        locals.var_phi_s0_soi_dn6 = assign14990_e20996_d_n6;
        locals.var_phi_s0_soi_dn7 = assign14990_e20996_d_n7;
        locals.var_phi_s0_soi_dn10 = assign14990_e20996_d_n10;
        locals.var_phi_s0_soi_dn11 = assign14990_e20996_d_n11;
        locals.var_phi_s0_soi_dn12 = assign14990_e20996_d_n12;
        locals.var_phi_s0_soi_dn17 = assign14990_e20996_d_n17;
        locals.var_phi_s0_soi_rv = 0.0;

        let (assign15000_e21009, assign15000_e21009_d_n0, assign15000_e21009_d_n2, assign15000_e21009_d_n6, assign15000_e21009_d_n7, assign15000_e21009_d_n10, assign15000_e21009_d_n11, assign15000_e21009_d_n12, assign15000_e21009_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15000_e21001: f64 = (locals.var_q_nsub * p.p237);
        let assign15000_e21003: f64 = (assign15000_e21001 * p.p237);
        let assign15000_e21005: f64 = (assign15000_e21003 / 2.0);
        let assign15000_e21007: f64 = (assign15000_e21005 / 1.034943e-10);
        (assign15000_e21007, ((((locals.var_q_nsub_dn0 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn2 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn6 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn7 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn10 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn11 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn12 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn17 * p.p237) * p.p237) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn12, locals.var_dphi_sb_dn17,)
    }
};
        locals.var_dphi_sb = assign15000_e21009;
        locals.var_dphi_sb_dn0 = assign15000_e21009_d_n0;
        locals.var_dphi_sb_dn2 = assign15000_e21009_d_n2;
        locals.var_dphi_sb_dn6 = assign15000_e21009_d_n6;
        locals.var_dphi_sb_dn7 = assign15000_e21009_d_n7;
        locals.var_dphi_sb_dn10 = assign15000_e21009_d_n10;
        locals.var_dphi_sb_dn11 = assign15000_e21009_d_n11;
        locals.var_dphi_sb_dn12 = assign15000_e21009_d_n12;
        locals.var_dphi_sb_dn17 = assign15000_e21009_d_n17;
        locals.var_dphi_sb_rv = 0.0;

        let (assign15010_e21019, assign15010_e21019_d_n0, assign15010_e21019_d_n2, assign15010_e21019_d_n6, assign15010_e21019_d_n7, assign15010_e21019_d_n10, assign15010_e21019_d_n11, assign15010_e21019_d_n12, assign15010_e21019_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15010_e21014: f64 = (2.0 * locals.var_beta);
        let assign15010_e21016: f64 = (assign15010_e21014 * locals.var_dphi_sb);
        let assign15010_e21017: f64 = (assign15010_e21016).sqrt();
        (assign15010_e21017, ((assign15010_e21014 * locals.var_dphi_sb_dn0) / (2.0 * assign15010_e21017)), ((assign15010_e21014 * locals.var_dphi_sb_dn2) / (2.0 * assign15010_e21017)), ((assign15010_e21014 * locals.var_dphi_sb_dn6) / (2.0 * assign15010_e21017)), ((assign15010_e21014 * locals.var_dphi_sb_dn7) / (2.0 * assign15010_e21017)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign15010_e21014 * locals.var_dphi_sb_dn10)) / (2.0 * assign15010_e21017)), ((assign15010_e21014 * locals.var_dphi_sb_dn11) / (2.0 * assign15010_e21017)), ((assign15010_e21014 * locals.var_dphi_sb_dn12) / (2.0 * assign15010_e21017)), ((assign15010_e21014 * locals.var_dphi_sb_dn17) / (2.0 * assign15010_e21017)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign15010_e21019;
        locals.var_t0_dn0 = assign15010_e21019_d_n0;
        locals.var_t0_dn2 = assign15010_e21019_d_n2;
        locals.var_t0_dn6 = assign15010_e21019_d_n6;
        locals.var_t0_dn7 = assign15010_e21019_d_n7;
        locals.var_t0_dn10 = assign15010_e21019_d_n10;
        locals.var_t0_dn11 = assign15010_e21019_d_n11;
        locals.var_t0_dn12 = assign15010_e21019_d_n12;
        locals.var_t0_dn17 = assign15010_e21019_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign15020_e21031, assign15020_e21031_d_n0, assign15020_e21031_d_n2, assign15020_e21031_d_n6, assign15020_e21031_d_n7, assign15020_e21031_d_n10, assign15020_e21031_d_n11, assign15020_e21031_d_n12, assign15020_e21031_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15020_e21023: f64 = (locals.var_t0).exp();
        let assign15020_e21025: f64 = (-locals.var_t0);
        let assign15020_e21026: f64 = (assign15020_e21025).exp();
        let assign15020_e21027: f64 = (assign15020_e21023 + assign15020_e21026);
        let assign15020_e21029: f64 = (assign15020_e21027 / 2.0);
        (assign15020_e21029, (((assign15020_e21023 * locals.var_t0_dn0) + (assign15020_e21026 * (-locals.var_t0_dn0))) / 2.0), (((assign15020_e21023 * locals.var_t0_dn2) + (assign15020_e21026 * (-locals.var_t0_dn2))) / 2.0), (((assign15020_e21023 * locals.var_t0_dn6) + (assign15020_e21026 * (-locals.var_t0_dn6))) / 2.0), (((assign15020_e21023 * locals.var_t0_dn7) + (assign15020_e21026 * (-locals.var_t0_dn7))) / 2.0), (((assign15020_e21023 * locals.var_t0_dn10) + (assign15020_e21026 * (-locals.var_t0_dn10))) / 2.0), (((assign15020_e21023 * locals.var_t0_dn11) + (assign15020_e21026 * (-locals.var_t0_dn11))) / 2.0), (((assign15020_e21023 * locals.var_t0_dn12) + (assign15020_e21026 * (-locals.var_t0_dn12))) / 2.0), (((assign15020_e21023 * locals.var_t0_dn17) + (assign15020_e21026 * (-locals.var_t0_dn17))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15020_e21031;
        locals.var_t1_dn0 = assign15020_e21031_d_n0;
        locals.var_t1_dn2 = assign15020_e21031_d_n2;
        locals.var_t1_dn6 = assign15020_e21031_d_n6;
        locals.var_t1_dn7 = assign15020_e21031_d_n7;
        locals.var_t1_dn10 = assign15020_e21031_d_n10;
        locals.var_t1_dn11 = assign15020_e21031_d_n11;
        locals.var_t1_dn12 = assign15020_e21031_d_n12;
        locals.var_t1_dn17 = assign15020_e21031_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign15030_e21039, assign15030_e21039_d_n0, assign15030_e21039_d_n2, assign15030_e21039_d_n6, assign15030_e21039_d_n7, assign15030_e21039_d_n10, assign15030_e21039_d_n11, assign15030_e21039_d_n12, assign15030_e21039_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15030_e21035: f64 = (locals.var_t1).ln();
        let assign15030_e21037: f64 = (assign15030_e21035 / locals.var_dphi_sb);
        (assign15030_e21037, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign15030_e21035 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign15030_e21035 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign15030_e21035 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign15030_e21035 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign15030_e21035 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign15030_e21035 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn12 / locals.var_t1) * locals.var_dphi_sb) - (assign15030_e21035 * locals.var_dphi_sb_dn12)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn17 / locals.var_t1) * locals.var_dphi_sb) - (assign15030_e21035 * locals.var_dphi_sb_dn17)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn12, locals.var_c_sb_dn17,)
    }
};
        locals.var_c_sb = assign15030_e21039;
        locals.var_c_sb_dn0 = assign15030_e21039_d_n0;
        locals.var_c_sb_dn2 = assign15030_e21039_d_n2;
        locals.var_c_sb_dn6 = assign15030_e21039_d_n6;
        locals.var_c_sb_dn7 = assign15030_e21039_d_n7;
        locals.var_c_sb_dn10 = assign15030_e21039_d_n10;
        locals.var_c_sb_dn11 = assign15030_e21039_d_n11;
        locals.var_c_sb_dn12 = assign15030_e21039_d_n12;
        locals.var_c_sb_dn17 = assign15030_e21039_d_n17;
        locals.var_c_sb_rv = 0.0;

        let (assign15040_e21044,) = {
    if (locals.var_guard111 == 0.0) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign15040_e21044;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign15050_loop_guard: usize = 0;
        while {
            let assign15050_cond_e21050: f64 = (locals.var_lp_s0_max + 1.0);
            let assign15050_cond_e21052: f64 = if ((locals.var_guard111 == 0.0) && (locals.var_lp_s0 <= assign15050_cond_e21050)) { 1.0 } else { 0.0 };
            assign15050_cond_e21052 != 0.0
        } {
            assign15050_loop_guard += 1;
            assert!(assign15050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15050_body0_e21059, assign15050_body0_e21059_d_n0, assign15050_body0_e21059_d_n2, assign15050_body0_e21059_d_n6, assign15050_body0_e21059_d_n7, assign15050_body0_e21059_d_n10, assign15050_body0_e21059_d_n11, assign15050_body0_e21059_d_n12, assign15050_body0_e21059_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15050_body0_e21057: f64 = (locals.var_phi_s0_soi - locals.var_vbcs_cl);
        (assign15050_body0_e21057, (locals.var_phi_s0_soi_dn0 - locals.var_vbcs_cl_dn0), (locals.var_phi_s0_soi_dn2 - locals.var_vbcs_cl_dn2), (locals.var_phi_s0_soi_dn6 - locals.var_vbcs_cl_dn6), (locals.var_phi_s0_soi_dn7 - locals.var_vbcs_cl_dn7), (locals.var_phi_s0_soi_dn10 - locals.var_vbcs_cl_dn10), (locals.var_phi_s0_soi_dn11 - locals.var_vbcs_cl_dn11), (locals.var_phi_s0_soi_dn12 - locals.var_vbcs_cl_dn12), (locals.var_phi_s0_soi_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_phi_soi0, locals.var_phi_soi0_dn0, locals.var_phi_soi0_dn2, locals.var_phi_soi0_dn6, locals.var_phi_soi0_dn7, locals.var_phi_soi0_dn10, locals.var_phi_soi0_dn11, locals.var_phi_soi0_dn12, locals.var_phi_soi0_dn17,)
    }
};
            locals.var_phi_soi0 = assign15050_body0_e21059;
            locals.var_phi_soi0_dn0 = assign15050_body0_e21059_d_n0;
            locals.var_phi_soi0_dn2 = assign15050_body0_e21059_d_n2;
            locals.var_phi_soi0_dn6 = assign15050_body0_e21059_d_n6;
            locals.var_phi_soi0_dn7 = assign15050_body0_e21059_d_n7;
            locals.var_phi_soi0_dn10 = assign15050_body0_e21059_d_n10;
            locals.var_phi_soi0_dn11 = assign15050_body0_e21059_d_n11;
            locals.var_phi_soi0_dn12 = assign15050_body0_e21059_d_n12;
            locals.var_phi_soi0_dn17 = assign15050_body0_e21059_d_n17;
            locals.var_phi_soi0_rv = 0.0;
            let (assign15050_body1_e21066, assign15050_body1_e21066_d_n0, assign15050_body1_e21066_d_n2, assign15050_body1_e21066_d_n6, assign15050_body1_e21066_d_n7, assign15050_body1_e21066_d_n10, assign15050_body1_e21066_d_n11, assign15050_body1_e21066_d_n12, assign15050_body1_e21066_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15050_body1_e21064: f64 = (locals.var_beta * locals.var_phi_soi0);
        (assign15050_body1_e21064, (locals.var_beta * locals.var_phi_soi0_dn0), (locals.var_beta * locals.var_phi_soi0_dn2), (locals.var_beta * locals.var_phi_soi0_dn6), (locals.var_beta * locals.var_phi_soi0_dn7), ((locals.var_beta_dn10 * locals.var_phi_soi0) + (locals.var_beta * locals.var_phi_soi0_dn10)), (locals.var_beta * locals.var_phi_soi0_dn11), (locals.var_beta * locals.var_phi_soi0_dn12), (locals.var_beta * locals.var_phi_soi0_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
            locals.var_chi = assign15050_body1_e21066;
            locals.var_chi_dn0 = assign15050_body1_e21066_d_n0;
            locals.var_chi_dn2 = assign15050_body1_e21066_d_n2;
            locals.var_chi_dn6 = assign15050_body1_e21066_d_n6;
            locals.var_chi_dn7 = assign15050_body1_e21066_d_n7;
            locals.var_chi_dn10 = assign15050_body1_e21066_d_n10;
            locals.var_chi_dn11 = assign15050_body1_e21066_d_n11;
            locals.var_chi_dn12 = assign15050_body1_e21066_d_n12;
            locals.var_chi_dn17 = assign15050_body1_e21066_d_n17;
            locals.var_chi_rv = 0.0;
            let (assign15050_body2_e21075, assign15050_body2_e21075_d_n0, assign15050_body2_e21075_d_n2, assign15050_body2_e21075_d_n6, assign15050_body2_e21075_d_n7, assign15050_body2_e21075_d_n10, assign15050_body2_e21075_d_n11, assign15050_body2_e21075_d_n12, assign15050_body2_e21075_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15050_body2_e21072: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        let assign15050_body2_e21073: f64 = (locals.var_c_sb * assign15050_body2_e21072);
        (assign15050_body2_e21073, ((locals.var_c_sb_dn0 * assign15050_body2_e21072) + (locals.var_c_sb * (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign15050_body2_e21072) + (locals.var_c_sb * (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn6 * assign15050_body2_e21072) + (locals.var_c_sb * (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign15050_body2_e21072) + (locals.var_c_sb * (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn10 * assign15050_body2_e21072) + (locals.var_c_sb * (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign15050_body2_e21072) + (locals.var_c_sb * (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn12 * assign15050_body2_e21072) + (locals.var_c_sb * (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12))), ((locals.var_c_sb_dn17 * assign15050_body2_e21072) + (locals.var_c_sb * (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
            locals.var_ty = assign15050_body2_e21075;
            locals.var_ty_dn0 = assign15050_body2_e21075_d_n0;
            locals.var_ty_dn2 = assign15050_body2_e21075_d_n2;
            locals.var_ty_dn6 = assign15050_body2_e21075_d_n6;
            locals.var_ty_dn7 = assign15050_body2_e21075_d_n7;
            locals.var_ty_dn10 = assign15050_body2_e21075_d_n10;
            locals.var_ty_dn11 = assign15050_body2_e21075_d_n11;
            locals.var_ty_dn12 = assign15050_body2_e21075_d_n12;
            locals.var_ty_dn17 = assign15050_body2_e21075_d_n17;
            locals.var_ty_rv = 0.0;
            let assign15050_body3_e21078: f64 = if locals.var_ty < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard449 = assign15050_body3_e21078;
            locals.var_guard449_rv = 0.0;
            let (assign15050_body4_e21086, assign15050_body4_e21086_d_n0, assign15050_body4_e21086_d_n2, assign15050_body4_e21086_d_n6, assign15050_body4_e21086_d_n7, assign15050_body4_e21086_d_n10, assign15050_body4_e21086_d_n11, assign15050_body4_e21086_d_n12, assign15050_body4_e21086_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard449 != 0.0)) {
        let assign15050_body4_e21084: f64 = (locals.var_ty).exp();
        (assign15050_body4_e21084, (assign15050_body4_e21084 * locals.var_ty_dn0), (assign15050_body4_e21084 * locals.var_ty_dn2), (assign15050_body4_e21084 * locals.var_ty_dn6), (assign15050_body4_e21084 * locals.var_ty_dn7), (assign15050_body4_e21084 * locals.var_ty_dn10), (assign15050_body4_e21084 * locals.var_ty_dn11), (assign15050_body4_e21084 * locals.var_ty_dn12), (assign15050_body4_e21084 * locals.var_ty_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15050_body4_e21086;
            locals.var_t1_dn0 = assign15050_body4_e21086_d_n0;
            locals.var_t1_dn2 = assign15050_body4_e21086_d_n2;
            locals.var_t1_dn6 = assign15050_body4_e21086_d_n6;
            locals.var_t1_dn7 = assign15050_body4_e21086_d_n7;
            locals.var_t1_dn10 = assign15050_body4_e21086_d_n10;
            locals.var_t1_dn11 = assign15050_body4_e21086_d_n11;
            locals.var_t1_dn12 = assign15050_body4_e21086_d_n12;
            locals.var_t1_dn17 = assign15050_body4_e21086_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign15050_body5_e21097, assign15050_body5_e21097_d_n0, assign15050_body5_e21097_d_n2, assign15050_body5_e21097_d_n6, assign15050_body5_e21097_d_n7, assign15050_body5_e21097_d_n10, assign15050_body5_e21097_d_n11, assign15050_body5_e21097_d_n12, assign15050_body5_e21097_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard449 != 0.0)) {
        let assign15050_body5_e21092: f64 = (-locals.var_c_sb);
        let assign15050_body5_e21094: f64 = (assign15050_body5_e21092 * locals.var_dphi_sb);
        let assign15050_body5_e21095: f64 = (assign15050_body5_e21094).exp();
        (assign15050_body5_e21095, (assign15050_body5_e21095 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign15050_body5_e21092 * locals.var_dphi_sb_dn0))), (assign15050_body5_e21095 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign15050_body5_e21092 * locals.var_dphi_sb_dn2))), (assign15050_body5_e21095 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign15050_body5_e21092 * locals.var_dphi_sb_dn6))), (assign15050_body5_e21095 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign15050_body5_e21092 * locals.var_dphi_sb_dn7))), (assign15050_body5_e21095 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign15050_body5_e21092 * locals.var_dphi_sb_dn10))), (assign15050_body5_e21095 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign15050_body5_e21092 * locals.var_dphi_sb_dn11))), (assign15050_body5_e21095 * (((-locals.var_c_sb_dn12) * locals.var_dphi_sb) + (assign15050_body5_e21092 * locals.var_dphi_sb_dn12))), (assign15050_body5_e21095 * (((-locals.var_c_sb_dn17) * locals.var_dphi_sb) + (assign15050_body5_e21092 * locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15050_body5_e21097;
            locals.var_t0_dn0 = assign15050_body5_e21097_d_n0;
            locals.var_t0_dn2 = assign15050_body5_e21097_d_n2;
            locals.var_t0_dn6 = assign15050_body5_e21097_d_n6;
            locals.var_t0_dn7 = assign15050_body5_e21097_d_n7;
            locals.var_t0_dn10 = assign15050_body5_e21097_d_n10;
            locals.var_t0_dn11 = assign15050_body5_e21097_d_n11;
            locals.var_t0_dn12 = assign15050_body5_e21097_d_n12;
            locals.var_t0_dn17 = assign15050_body5_e21097_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15050_body6_e21106, assign15050_body6_e21106_d_n0, assign15050_body6_e21106_d_n2, assign15050_body6_e21106_d_n6, assign15050_body6_e21106_d_n7, assign15050_body6_e21106_d_n10, assign15050_body6_e21106_d_n11, assign15050_body6_e21106_d_n12, assign15050_body6_e21106_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard449 != 0.0)) {
        let assign15050_body6_e21104: f64 = (locals.var_t1 - locals.var_t0);
        (assign15050_body6_e21104, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn12 - locals.var_t0_dn12), (locals.var_t1_dn17 - locals.var_t0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign15050_body6_e21106;
            locals.var_t2_dn0 = assign15050_body6_e21106_d_n0;
            locals.var_t2_dn2 = assign15050_body6_e21106_d_n2;
            locals.var_t2_dn6 = assign15050_body6_e21106_d_n6;
            locals.var_t2_dn7 = assign15050_body6_e21106_d_n7;
            locals.var_t2_dn10 = assign15050_body6_e21106_d_n10;
            locals.var_t2_dn11 = assign15050_body6_e21106_d_n11;
            locals.var_t2_dn12 = assign15050_body6_e21106_d_n12;
            locals.var_t2_dn17 = assign15050_body6_e21106_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign15050_body7_e21118, assign15050_body7_e21118_d_n0, assign15050_body7_e21118_d_n2, assign15050_body7_e21118_d_n6, assign15050_body7_e21118_d_n7, assign15050_body7_e21118_d_n10, assign15050_body7_e21118_d_n11, assign15050_body7_e21118_d_n12, assign15050_body7_e21118_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard449 != 0.0)) {
        let assign15050_body7_e21113: f64 = (1.0 + locals.var_t2);
        let assign15050_body7_e21114: f64 = (assign15050_body7_e21113).ln();
        let assign15050_body7_e21116: f64 = (assign15050_body7_e21114 / locals.var_c_sb);
        (assign15050_body7_e21116, ((((locals.var_t2_dn0 / assign15050_body7_e21113) * locals.var_c_sb) - (assign15050_body7_e21114 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign15050_body7_e21113) * locals.var_c_sb) - (assign15050_body7_e21114 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign15050_body7_e21113) * locals.var_c_sb) - (assign15050_body7_e21114 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign15050_body7_e21113) * locals.var_c_sb) - (assign15050_body7_e21114 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign15050_body7_e21113) * locals.var_c_sb) - (assign15050_body7_e21114 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign15050_body7_e21113) * locals.var_c_sb) - (assign15050_body7_e21114 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn12 / assign15050_body7_e21113) * locals.var_c_sb) - (assign15050_body7_e21114 * locals.var_c_sb_dn12)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn17 / assign15050_body7_e21113) * locals.var_c_sb) - (assign15050_body7_e21114 * locals.var_c_sb_dn17)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign15050_body7_e21118;
            locals.var_phi_soib_dn0 = assign15050_body7_e21118_d_n0;
            locals.var_phi_soib_dn2 = assign15050_body7_e21118_d_n2;
            locals.var_phi_soib_dn6 = assign15050_body7_e21118_d_n6;
            locals.var_phi_soib_dn7 = assign15050_body7_e21118_d_n7;
            locals.var_phi_soib_dn10 = assign15050_body7_e21118_d_n10;
            locals.var_phi_soib_dn11 = assign15050_body7_e21118_d_n11;
            locals.var_phi_soib_dn12 = assign15050_body7_e21118_d_n12;
            locals.var_phi_soib_dn17 = assign15050_body7_e21118_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign15050_body8_e21129, assign15050_body8_e21129_d_n0, assign15050_body8_e21129_d_n2, assign15050_body8_e21129_d_n6, assign15050_body8_e21129_d_n7, assign15050_body8_e21129_d_n10, assign15050_body8_e21129_d_n11, assign15050_body8_e21129_d_n12, assign15050_body8_e21129_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard449 != 0.0)) {
        let assign15050_body8_e21126: f64 = (1.0 + locals.var_t2);
        let assign15050_body8_e21127: f64 = (locals.var_t1 / assign15050_body8_e21126);
        (assign15050_body8_e21127, (((locals.var_t1_dn0 * assign15050_body8_e21126) - (locals.var_t1 * locals.var_t2_dn0)) / (assign15050_body8_e21126 * assign15050_body8_e21126)), (((locals.var_t1_dn2 * assign15050_body8_e21126) - (locals.var_t1 * locals.var_t2_dn2)) / (assign15050_body8_e21126 * assign15050_body8_e21126)), (((locals.var_t1_dn6 * assign15050_body8_e21126) - (locals.var_t1 * locals.var_t2_dn6)) / (assign15050_body8_e21126 * assign15050_body8_e21126)), (((locals.var_t1_dn7 * assign15050_body8_e21126) - (locals.var_t1 * locals.var_t2_dn7)) / (assign15050_body8_e21126 * assign15050_body8_e21126)), (((locals.var_t1_dn10 * assign15050_body8_e21126) - (locals.var_t1 * locals.var_t2_dn10)) / (assign15050_body8_e21126 * assign15050_body8_e21126)), (((locals.var_t1_dn11 * assign15050_body8_e21126) - (locals.var_t1 * locals.var_t2_dn11)) / (assign15050_body8_e21126 * assign15050_body8_e21126)), (((locals.var_t1_dn12 * assign15050_body8_e21126) - (locals.var_t1 * locals.var_t2_dn12)) / (assign15050_body8_e21126 * assign15050_body8_e21126)), (((locals.var_t1_dn17 * assign15050_body8_e21126) - (locals.var_t1 * locals.var_t2_dn17)) / (assign15050_body8_e21126 * assign15050_body8_e21126)),)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign15050_body8_e21129;
            locals.var_phi_soib_dpss_dn0 = assign15050_body8_e21129_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign15050_body8_e21129_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign15050_body8_e21129_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign15050_body8_e21129_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign15050_body8_e21129_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign15050_body8_e21129_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign15050_body8_e21129_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign15050_body8_e21129_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign15050_body9_e21139, assign15050_body9_e21139_d_n0, assign15050_body9_e21139_d_n2, assign15050_body9_e21139_d_n6, assign15050_body9_e21139_d_n7, assign15050_body9_e21139_d_n10, assign15050_body9_e21139_d_n11, assign15050_body9_e21139_d_n12, assign15050_body9_e21139_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard449 == 0.0)) {
        let assign15050_body9_e21137: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        (assign15050_body9_e21137, (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0), (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2), (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6), (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7), (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10), (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11), (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12), (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign15050_body9_e21139;
            locals.var_phi_soib_dn0 = assign15050_body9_e21139_d_n0;
            locals.var_phi_soib_dn2 = assign15050_body9_e21139_d_n2;
            locals.var_phi_soib_dn6 = assign15050_body9_e21139_d_n6;
            locals.var_phi_soib_dn7 = assign15050_body9_e21139_d_n7;
            locals.var_phi_soib_dn10 = assign15050_body9_e21139_d_n10;
            locals.var_phi_soib_dn11 = assign15050_body9_e21139_d_n11;
            locals.var_phi_soib_dn12 = assign15050_body9_e21139_d_n12;
            locals.var_phi_soib_dn17 = assign15050_body9_e21139_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign15050_body10_e21147, assign15050_body10_e21147_d_n0, assign15050_body10_e21147_d_n2, assign15050_body10_e21147_d_n6, assign15050_body10_e21147_d_n7, assign15050_body10_e21147_d_n10, assign15050_body10_e21147_d_n11, assign15050_body10_e21147_d_n12, assign15050_body10_e21147_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard449 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign15050_body10_e21147;
            locals.var_phi_soib_dpss_dn0 = assign15050_body10_e21147_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign15050_body10_e21147_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign15050_body10_e21147_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign15050_body10_e21147_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign15050_body10_e21147_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign15050_body10_e21147_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign15050_body10_e21147_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign15050_body10_e21147_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign15050_body11_e21154, assign15050_body11_e21154_d_n0, assign15050_body11_e21154_d_n2, assign15050_body11_e21154_d_n6, assign15050_body11_e21154_d_n7, assign15050_body11_e21154_d_n10, assign15050_body11_e21154_d_n11, assign15050_body11_e21154_d_n12, assign15050_body11_e21154_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15050_body11_e21152: f64 = (locals.var_beta * locals.var_phi_soib);
        (assign15050_body11_e21152, (locals.var_beta * locals.var_phi_soib_dn0), (locals.var_beta * locals.var_phi_soib_dn2), (locals.var_beta * locals.var_phi_soib_dn6), (locals.var_beta * locals.var_phi_soib_dn7), ((locals.var_beta_dn10 * locals.var_phi_soib) + (locals.var_beta * locals.var_phi_soib_dn10)), (locals.var_beta * locals.var_phi_soib_dn11), (locals.var_beta * locals.var_phi_soib_dn12), (locals.var_beta * locals.var_phi_soib_dn17),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn12, locals.var_chib_dn17,)
    }
};
            locals.var_chib = assign15050_body11_e21154;
            locals.var_chib_dn0 = assign15050_body11_e21154_d_n0;
            locals.var_chib_dn2 = assign15050_body11_e21154_d_n2;
            locals.var_chib_dn6 = assign15050_body11_e21154_d_n6;
            locals.var_chib_dn7 = assign15050_body11_e21154_d_n7;
            locals.var_chib_dn10 = assign15050_body11_e21154_d_n10;
            locals.var_chib_dn11 = assign15050_body11_e21154_d_n11;
            locals.var_chib_dn12 = assign15050_body11_e21154_d_n12;
            locals.var_chib_dn17 = assign15050_body11_e21154_d_n17;
            locals.var_chib_rv = 0.0;
            let assign15050_body12_e21156: f64 = (locals.var_chi).abs();
            let assign15050_body12_e21158: f64 = if assign15050_body12_e21156 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard450 = assign15050_body12_e21158;
            locals.var_guard450_rv = 0.0;
            let (assign15050_body13_e21172, assign15050_body13_e21172_d_n0, assign15050_body13_e21172_d_n2, assign15050_body13_e21172_d_n6, assign15050_body13_e21172_d_n7, assign15050_body13_e21172_d_n10, assign15050_body13_e21172_d_n11, assign15050_body13_e21172_d_n12, assign15050_body13_e21172_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard450 != 0.0)) {
        let assign15050_body13_e21166: f64 = (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss);
        let assign15050_body13_e21167: f64 = (1.0 - assign15050_body13_e21166);
        let assign15050_body13_e21169: f64 = (assign15050_body13_e21167 / 2.0);
        let assign15050_body13_e21170: f64 = (assign15050_body13_e21169).sqrt();
        (assign15050_body13_e21170, (((-((locals.var_phi_soib_dpss_dn0 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn0))) / 2.0) / (2.0 * assign15050_body13_e21170)), (((-((locals.var_phi_soib_dpss_dn2 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn2))) / 2.0) / (2.0 * assign15050_body13_e21170)), (((-((locals.var_phi_soib_dpss_dn6 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn6))) / 2.0) / (2.0 * assign15050_body13_e21170)), (((-((locals.var_phi_soib_dpss_dn7 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn7))) / 2.0) / (2.0 * assign15050_body13_e21170)), (((-((locals.var_phi_soib_dpss_dn10 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn10))) / 2.0) / (2.0 * assign15050_body13_e21170)), (((-((locals.var_phi_soib_dpss_dn11 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn11))) / 2.0) / (2.0 * assign15050_body13_e21170)), (((-((locals.var_phi_soib_dpss_dn12 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn12))) / 2.0) / (2.0 * assign15050_body13_e21170)), (((-((locals.var_phi_soib_dpss_dn17 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn17))) / 2.0) / (2.0 * assign15050_body13_e21170)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15050_body13_e21172;
            locals.var_t0_dn0 = assign15050_body13_e21172_d_n0;
            locals.var_t0_dn2 = assign15050_body13_e21172_d_n2;
            locals.var_t0_dn6 = assign15050_body13_e21172_d_n6;
            locals.var_t0_dn7 = assign15050_body13_e21172_d_n7;
            locals.var_t0_dn10 = assign15050_body13_e21172_d_n10;
            locals.var_t0_dn11 = assign15050_body13_e21172_d_n11;
            locals.var_t0_dn12 = assign15050_body13_e21172_d_n12;
            locals.var_t0_dn17 = assign15050_body13_e21172_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15050_body14_e21181, assign15050_body14_e21181_d_n0, assign15050_body14_e21181_d_n2, assign15050_body14_e21181_d_n6, assign15050_body14_e21181_d_n7, assign15050_body14_e21181_d_n10, assign15050_body14_e21181_d_n11, assign15050_body14_e21181_d_n12, assign15050_body14_e21181_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard450 != 0.0)) {
        let assign15050_body14_e21179: f64 = (locals.var_chi * locals.var_t0);
        (assign15050_body14_e21179, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn12 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn12)), ((locals.var_chi_dn17 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn17)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15050_body14_e21181;
            locals.var_fb_dn0 = assign15050_body14_e21181_d_n0;
            locals.var_fb_dn2 = assign15050_body14_e21181_d_n2;
            locals.var_fb_dn6 = assign15050_body14_e21181_d_n6;
            locals.var_fb_dn7 = assign15050_body14_e21181_d_n7;
            locals.var_fb_dn10 = assign15050_body14_e21181_d_n10;
            locals.var_fb_dn11 = assign15050_body14_e21181_d_n11;
            locals.var_fb_dn12 = assign15050_body14_e21181_d_n12;
            locals.var_fb_dn17 = assign15050_body14_e21181_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15050_body15_e21190, assign15050_body15_e21190_d_n0, assign15050_body15_e21190_d_n2, assign15050_body15_e21190_d_n6, assign15050_body15_e21190_d_n7, assign15050_body15_e21190_d_n10, assign15050_body15_e21190_d_n11, assign15050_body15_e21190_d_n12, assign15050_body15_e21190_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard450 != 0.0)) {
        let assign15050_body15_e21188: f64 = (locals.var_beta * locals.var_t0);
        (assign15050_body15_e21188, (locals.var_beta * locals.var_t0_dn0), (locals.var_beta * locals.var_t0_dn2), (locals.var_beta * locals.var_t0_dn6), (locals.var_beta * locals.var_t0_dn7), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), (locals.var_beta * locals.var_t0_dn11), (locals.var_beta * locals.var_t0_dn12), (locals.var_beta * locals.var_t0_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15050_body15_e21190;
            locals.var_fb_dpss_dn0 = assign15050_body15_e21190_d_n0;
            locals.var_fb_dpss_dn2 = assign15050_body15_e21190_d_n2;
            locals.var_fb_dpss_dn6 = assign15050_body15_e21190_d_n6;
            locals.var_fb_dpss_dn7 = assign15050_body15_e21190_d_n7;
            locals.var_fb_dpss_dn10 = assign15050_body15_e21190_d_n10;
            locals.var_fb_dpss_dn11 = assign15050_body15_e21190_d_n11;
            locals.var_fb_dpss_dn12 = assign15050_body15_e21190_d_n12;
            locals.var_fb_dpss_dn17 = assign15050_body15_e21190_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign15050_body16_e21193: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard451 = assign15050_body16_e21193;
            locals.var_guard451_rv = 0.0;
            let (assign15050_body17_e21203, assign15050_body17_e21203_d_n0, assign15050_body17_e21203_d_n2, assign15050_body17_e21203_d_n6, assign15050_body17_e21203_d_n7, assign15050_body17_e21203_d_n10, assign15050_body17_e21203_d_n11, assign15050_body17_e21203_d_n12, assign15050_body17_e21203_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 != 0.0)) && (locals.var_guard451 != 0.0)) {
        let assign15050_body17_e21201: f64 = (-locals.var_fb);
        (assign15050_body17_e21201, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15050_body17_e21203;
            locals.var_fb_dn0 = assign15050_body17_e21203_d_n0;
            locals.var_fb_dn2 = assign15050_body17_e21203_d_n2;
            locals.var_fb_dn6 = assign15050_body17_e21203_d_n6;
            locals.var_fb_dn7 = assign15050_body17_e21203_d_n7;
            locals.var_fb_dn10 = assign15050_body17_e21203_d_n10;
            locals.var_fb_dn11 = assign15050_body17_e21203_d_n11;
            locals.var_fb_dn12 = assign15050_body17_e21203_d_n12;
            locals.var_fb_dn17 = assign15050_body17_e21203_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15050_body18_e21213, assign15050_body18_e21213_d_n0, assign15050_body18_e21213_d_n2, assign15050_body18_e21213_d_n6, assign15050_body18_e21213_d_n7, assign15050_body18_e21213_d_n10, assign15050_body18_e21213_d_n11, assign15050_body18_e21213_d_n12, assign15050_body18_e21213_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 != 0.0)) && (locals.var_guard451 != 0.0)) {
        let assign15050_body18_e21211: f64 = (-locals.var_fb_dpss);
        (assign15050_body18_e21211, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15050_body18_e21213;
            locals.var_fb_dpss_dn0 = assign15050_body18_e21213_d_n0;
            locals.var_fb_dpss_dn2 = assign15050_body18_e21213_d_n2;
            locals.var_fb_dpss_dn6 = assign15050_body18_e21213_d_n6;
            locals.var_fb_dpss_dn7 = assign15050_body18_e21213_d_n7;
            locals.var_fb_dpss_dn10 = assign15050_body18_e21213_d_n10;
            locals.var_fb_dpss_dn11 = assign15050_body18_e21213_d_n11;
            locals.var_fb_dpss_dn12 = assign15050_body18_e21213_d_n12;
            locals.var_fb_dpss_dn17 = assign15050_body18_e21213_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign15050_body19_e21215: f64 = (locals.var_chi).abs();
            let assign15050_body19_e21217: f64 = if assign15050_body19_e21215 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard452 = assign15050_body19_e21217;
            locals.var_guard452_rv = 0.0;
            let (assign15050_body20_e21249, assign15050_body20_e21249_d_n0, assign15050_body20_e21249_d_n2, assign15050_body20_e21249_d_n6, assign15050_body20_e21249_d_n7, assign15050_body20_e21249_d_n10, assign15050_body20_e21249_d_n11, assign15050_body20_e21249_d_n12, assign15050_body20_e21249_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 == 0.0)) && (locals.var_guard452 != 0.0)) {
        let assign15050_body20_e21227: f64 = (locals.var_chi * locals.var_chi);
        let assign15050_body20_e21229: f64 = (assign15050_body20_e21227 / 2.0);
        let assign15050_body20_e21233: f64 = (locals.var_chi / 3.0);
        let assign15050_body20_e21237: f64 = (locals.var_chi / 4.0);
        let assign15050_body20_e21241: f64 = (locals.var_chi / 5.0);
        let assign15050_body20_e21242: f64 = (1.0 - assign15050_body20_e21241);
        let assign15050_body20_e21243: f64 = (assign15050_body20_e21237 * assign15050_body20_e21242);
        let assign15050_body20_e21244: f64 = (1.0 - assign15050_body20_e21243);
        let assign15050_body20_e21245: f64 = (assign15050_body20_e21233 * assign15050_body20_e21244);
        let assign15050_body20_e21246: f64 = (1.0 - assign15050_body20_e21245);
        let assign15050_body20_e21247: f64 = (assign15050_body20_e21229 * assign15050_body20_e21246);
        (assign15050_body20_e21247, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign15050_body20_e21246) + (assign15050_body20_e21229 * (-(((locals.var_chi_dn0 / 3.0) * assign15050_body20_e21244) + (assign15050_body20_e21233 * (-(((locals.var_chi_dn0 / 4.0) * assign15050_body20_e21242) + (assign15050_body20_e21237 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign15050_body20_e21246) + (assign15050_body20_e21229 * (-(((locals.var_chi_dn2 / 3.0) * assign15050_body20_e21244) + (assign15050_body20_e21233 * (-(((locals.var_chi_dn2 / 4.0) * assign15050_body20_e21242) + (assign15050_body20_e21237 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign15050_body20_e21246) + (assign15050_body20_e21229 * (-(((locals.var_chi_dn6 / 3.0) * assign15050_body20_e21244) + (assign15050_body20_e21233 * (-(((locals.var_chi_dn6 / 4.0) * assign15050_body20_e21242) + (assign15050_body20_e21237 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign15050_body20_e21246) + (assign15050_body20_e21229 * (-(((locals.var_chi_dn7 / 3.0) * assign15050_body20_e21244) + (assign15050_body20_e21233 * (-(((locals.var_chi_dn7 / 4.0) * assign15050_body20_e21242) + (assign15050_body20_e21237 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign15050_body20_e21246) + (assign15050_body20_e21229 * (-(((locals.var_chi_dn10 / 3.0) * assign15050_body20_e21244) + (assign15050_body20_e21233 * (-(((locals.var_chi_dn10 / 4.0) * assign15050_body20_e21242) + (assign15050_body20_e21237 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign15050_body20_e21246) + (assign15050_body20_e21229 * (-(((locals.var_chi_dn11 / 3.0) * assign15050_body20_e21244) + (assign15050_body20_e21233 * (-(((locals.var_chi_dn11 / 4.0) * assign15050_body20_e21242) + (assign15050_body20_e21237 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) / 2.0) * assign15050_body20_e21246) + (assign15050_body20_e21229 * (-(((locals.var_chi_dn12 / 3.0) * assign15050_body20_e21244) + (assign15050_body20_e21233 * (-(((locals.var_chi_dn12 / 4.0) * assign15050_body20_e21242) + (assign15050_body20_e21237 * (-(locals.var_chi_dn12 / 5.0)))))))))), (((((locals.var_chi_dn17 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn17)) / 2.0) * assign15050_body20_e21246) + (assign15050_body20_e21229 * (-(((locals.var_chi_dn17 / 3.0) * assign15050_body20_e21244) + (assign15050_body20_e21233 * (-(((locals.var_chi_dn17 / 4.0) * assign15050_body20_e21242) + (assign15050_body20_e21237 * (-(locals.var_chi_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15050_body20_e21249;
            locals.var_t0_dn0 = assign15050_body20_e21249_d_n0;
            locals.var_t0_dn2 = assign15050_body20_e21249_d_n2;
            locals.var_t0_dn6 = assign15050_body20_e21249_d_n6;
            locals.var_t0_dn7 = assign15050_body20_e21249_d_n7;
            locals.var_t0_dn10 = assign15050_body20_e21249_d_n10;
            locals.var_t0_dn11 = assign15050_body20_e21249_d_n11;
            locals.var_t0_dn12 = assign15050_body20_e21249_d_n12;
            locals.var_t0_dn17 = assign15050_body20_e21249_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15050_body21_e21277, assign15050_body21_e21277_d_n0, assign15050_body21_e21277_d_n2, assign15050_body21_e21277_d_n6, assign15050_body21_e21277_d_n7, assign15050_body21_e21277_d_n10, assign15050_body21_e21277_d_n11, assign15050_body21_e21277_d_n12, assign15050_body21_e21277_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 == 0.0)) && (locals.var_guard452 != 0.0)) {
        let assign15050_body21_e21261: f64 = (locals.var_chi / 2.0);
        let assign15050_body21_e21265: f64 = (locals.var_chi / 3.0);
        let assign15050_body21_e21269: f64 = (locals.var_chi / 4.0);
        let assign15050_body21_e21270: f64 = (1.0 - assign15050_body21_e21269);
        let assign15050_body21_e21271: f64 = (assign15050_body21_e21265 * assign15050_body21_e21270);
        let assign15050_body21_e21272: f64 = (1.0 - assign15050_body21_e21271);
        let assign15050_body21_e21273: f64 = (assign15050_body21_e21261 * assign15050_body21_e21272);
        let assign15050_body21_e21274: f64 = (1.0 - assign15050_body21_e21273);
        let assign15050_body21_e21275: f64 = (locals.var_chi * assign15050_body21_e21274);
        (assign15050_body21_e21275, ((locals.var_chi_dn0 * assign15050_body21_e21274) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign15050_body21_e21272) + (assign15050_body21_e21261 * (-(((locals.var_chi_dn0 / 3.0) * assign15050_body21_e21270) + (assign15050_body21_e21265 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign15050_body21_e21274) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign15050_body21_e21272) + (assign15050_body21_e21261 * (-(((locals.var_chi_dn2 / 3.0) * assign15050_body21_e21270) + (assign15050_body21_e21265 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn6 * assign15050_body21_e21274) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign15050_body21_e21272) + (assign15050_body21_e21261 * (-(((locals.var_chi_dn6 / 3.0) * assign15050_body21_e21270) + (assign15050_body21_e21265 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign15050_body21_e21274) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign15050_body21_e21272) + (assign15050_body21_e21261 * (-(((locals.var_chi_dn7 / 3.0) * assign15050_body21_e21270) + (assign15050_body21_e21265 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn10 * assign15050_body21_e21274) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign15050_body21_e21272) + (assign15050_body21_e21261 * (-(((locals.var_chi_dn10 / 3.0) * assign15050_body21_e21270) + (assign15050_body21_e21265 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign15050_body21_e21274) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign15050_body21_e21272) + (assign15050_body21_e21261 * (-(((locals.var_chi_dn11 / 3.0) * assign15050_body21_e21270) + (assign15050_body21_e21265 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn12 * assign15050_body21_e21274) + (locals.var_chi * (-(((locals.var_chi_dn12 / 2.0) * assign15050_body21_e21272) + (assign15050_body21_e21261 * (-(((locals.var_chi_dn12 / 3.0) * assign15050_body21_e21270) + (assign15050_body21_e21265 * (-(locals.var_chi_dn12 / 4.0)))))))))), ((locals.var_chi_dn17 * assign15050_body21_e21274) + (locals.var_chi * (-(((locals.var_chi_dn17 / 2.0) * assign15050_body21_e21272) + (assign15050_body21_e21261 * (-(((locals.var_chi_dn17 / 3.0) * assign15050_body21_e21270) + (assign15050_body21_e21265 * (-(locals.var_chi_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15050_body21_e21277;
            locals.var_t1_dn0 = assign15050_body21_e21277_d_n0;
            locals.var_t1_dn2 = assign15050_body21_e21277_d_n2;
            locals.var_t1_dn6 = assign15050_body21_e21277_d_n6;
            locals.var_t1_dn7 = assign15050_body21_e21277_d_n7;
            locals.var_t1_dn10 = assign15050_body21_e21277_d_n10;
            locals.var_t1_dn11 = assign15050_body21_e21277_d_n11;
            locals.var_t1_dn12 = assign15050_body21_e21277_d_n12;
            locals.var_t1_dn17 = assign15050_body21_e21277_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign15050_body22_e21309, assign15050_body22_e21309_d_n0, assign15050_body22_e21309_d_n2, assign15050_body22_e21309_d_n6, assign15050_body22_e21309_d_n7, assign15050_body22_e21309_d_n10, assign15050_body22_e21309_d_n11, assign15050_body22_e21309_d_n12, assign15050_body22_e21309_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 == 0.0)) && (locals.var_guard452 != 0.0)) {
        let assign15050_body22_e21287: f64 = (locals.var_chib * locals.var_chib);
        let assign15050_body22_e21289: f64 = (assign15050_body22_e21287 / 2.0);
        let assign15050_body22_e21293: f64 = (locals.var_chib / 3.0);
        let assign15050_body22_e21297: f64 = (locals.var_chib / 4.0);
        let assign15050_body22_e21301: f64 = (locals.var_chib / 5.0);
        let assign15050_body22_e21302: f64 = (1.0 - assign15050_body22_e21301);
        let assign15050_body22_e21303: f64 = (assign15050_body22_e21297 * assign15050_body22_e21302);
        let assign15050_body22_e21304: f64 = (1.0 - assign15050_body22_e21303);
        let assign15050_body22_e21305: f64 = (assign15050_body22_e21293 * assign15050_body22_e21304);
        let assign15050_body22_e21306: f64 = (1.0 - assign15050_body22_e21305);
        let assign15050_body22_e21307: f64 = (assign15050_body22_e21289 * assign15050_body22_e21306);
        (assign15050_body22_e21307, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign15050_body22_e21306) + (assign15050_body22_e21289 * (-(((locals.var_chib_dn0 / 3.0) * assign15050_body22_e21304) + (assign15050_body22_e21293 * (-(((locals.var_chib_dn0 / 4.0) * assign15050_body22_e21302) + (assign15050_body22_e21297 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign15050_body22_e21306) + (assign15050_body22_e21289 * (-(((locals.var_chib_dn2 / 3.0) * assign15050_body22_e21304) + (assign15050_body22_e21293 * (-(((locals.var_chib_dn2 / 4.0) * assign15050_body22_e21302) + (assign15050_body22_e21297 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign15050_body22_e21306) + (assign15050_body22_e21289 * (-(((locals.var_chib_dn6 / 3.0) * assign15050_body22_e21304) + (assign15050_body22_e21293 * (-(((locals.var_chib_dn6 / 4.0) * assign15050_body22_e21302) + (assign15050_body22_e21297 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign15050_body22_e21306) + (assign15050_body22_e21289 * (-(((locals.var_chib_dn7 / 3.0) * assign15050_body22_e21304) + (assign15050_body22_e21293 * (-(((locals.var_chib_dn7 / 4.0) * assign15050_body22_e21302) + (assign15050_body22_e21297 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign15050_body22_e21306) + (assign15050_body22_e21289 * (-(((locals.var_chib_dn10 / 3.0) * assign15050_body22_e21304) + (assign15050_body22_e21293 * (-(((locals.var_chib_dn10 / 4.0) * assign15050_body22_e21302) + (assign15050_body22_e21297 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign15050_body22_e21306) + (assign15050_body22_e21289 * (-(((locals.var_chib_dn11 / 3.0) * assign15050_body22_e21304) + (assign15050_body22_e21293 * (-(((locals.var_chib_dn11 / 4.0) * assign15050_body22_e21302) + (assign15050_body22_e21297 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn12 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn12)) / 2.0) * assign15050_body22_e21306) + (assign15050_body22_e21289 * (-(((locals.var_chib_dn12 / 3.0) * assign15050_body22_e21304) + (assign15050_body22_e21293 * (-(((locals.var_chib_dn12 / 4.0) * assign15050_body22_e21302) + (assign15050_body22_e21297 * (-(locals.var_chib_dn12 / 5.0)))))))))), (((((locals.var_chib_dn17 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn17)) / 2.0) * assign15050_body22_e21306) + (assign15050_body22_e21289 * (-(((locals.var_chib_dn17 / 3.0) * assign15050_body22_e21304) + (assign15050_body22_e21293 * (-(((locals.var_chib_dn17 / 4.0) * assign15050_body22_e21302) + (assign15050_body22_e21297 * (-(locals.var_chib_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign15050_body22_e21309;
            locals.var_t2_dn0 = assign15050_body22_e21309_d_n0;
            locals.var_t2_dn2 = assign15050_body22_e21309_d_n2;
            locals.var_t2_dn6 = assign15050_body22_e21309_d_n6;
            locals.var_t2_dn7 = assign15050_body22_e21309_d_n7;
            locals.var_t2_dn10 = assign15050_body22_e21309_d_n10;
            locals.var_t2_dn11 = assign15050_body22_e21309_d_n11;
            locals.var_t2_dn12 = assign15050_body22_e21309_d_n12;
            locals.var_t2_dn17 = assign15050_body22_e21309_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign15050_body23_e21337, assign15050_body23_e21337_d_n0, assign15050_body23_e21337_d_n2, assign15050_body23_e21337_d_n6, assign15050_body23_e21337_d_n7, assign15050_body23_e21337_d_n10, assign15050_body23_e21337_d_n11, assign15050_body23_e21337_d_n12, assign15050_body23_e21337_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 == 0.0)) && (locals.var_guard452 != 0.0)) {
        let assign15050_body23_e21321: f64 = (locals.var_chib / 2.0);
        let assign15050_body23_e21325: f64 = (locals.var_chib / 3.0);
        let assign15050_body23_e21329: f64 = (locals.var_chib / 4.0);
        let assign15050_body23_e21330: f64 = (1.0 - assign15050_body23_e21329);
        let assign15050_body23_e21331: f64 = (assign15050_body23_e21325 * assign15050_body23_e21330);
        let assign15050_body23_e21332: f64 = (1.0 - assign15050_body23_e21331);
        let assign15050_body23_e21333: f64 = (assign15050_body23_e21321 * assign15050_body23_e21332);
        let assign15050_body23_e21334: f64 = (1.0 - assign15050_body23_e21333);
        let assign15050_body23_e21335: f64 = (locals.var_chib * assign15050_body23_e21334);
        (assign15050_body23_e21335, ((locals.var_chib_dn0 * assign15050_body23_e21334) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign15050_body23_e21332) + (assign15050_body23_e21321 * (-(((locals.var_chib_dn0 / 3.0) * assign15050_body23_e21330) + (assign15050_body23_e21325 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign15050_body23_e21334) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign15050_body23_e21332) + (assign15050_body23_e21321 * (-(((locals.var_chib_dn2 / 3.0) * assign15050_body23_e21330) + (assign15050_body23_e21325 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn6 * assign15050_body23_e21334) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign15050_body23_e21332) + (assign15050_body23_e21321 * (-(((locals.var_chib_dn6 / 3.0) * assign15050_body23_e21330) + (assign15050_body23_e21325 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign15050_body23_e21334) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign15050_body23_e21332) + (assign15050_body23_e21321 * (-(((locals.var_chib_dn7 / 3.0) * assign15050_body23_e21330) + (assign15050_body23_e21325 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn10 * assign15050_body23_e21334) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign15050_body23_e21332) + (assign15050_body23_e21321 * (-(((locals.var_chib_dn10 / 3.0) * assign15050_body23_e21330) + (assign15050_body23_e21325 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign15050_body23_e21334) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign15050_body23_e21332) + (assign15050_body23_e21321 * (-(((locals.var_chib_dn11 / 3.0) * assign15050_body23_e21330) + (assign15050_body23_e21325 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn12 * assign15050_body23_e21334) + (locals.var_chib * (-(((locals.var_chib_dn12 / 2.0) * assign15050_body23_e21332) + (assign15050_body23_e21321 * (-(((locals.var_chib_dn12 / 3.0) * assign15050_body23_e21330) + (assign15050_body23_e21325 * (-(locals.var_chib_dn12 / 4.0)))))))))), ((locals.var_chib_dn17 * assign15050_body23_e21334) + (locals.var_chib * (-(((locals.var_chib_dn17 / 2.0) * assign15050_body23_e21332) + (assign15050_body23_e21321 * (-(((locals.var_chib_dn17 / 3.0) * assign15050_body23_e21330) + (assign15050_body23_e21325 * (-(locals.var_chib_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign15050_body23_e21337;
            locals.var_t3_dn0 = assign15050_body23_e21337_d_n0;
            locals.var_t3_dn2 = assign15050_body23_e21337_d_n2;
            locals.var_t3_dn6 = assign15050_body23_e21337_d_n6;
            locals.var_t3_dn7 = assign15050_body23_e21337_d_n7;
            locals.var_t3_dn10 = assign15050_body23_e21337_d_n10;
            locals.var_t3_dn11 = assign15050_body23_e21337_d_n11;
            locals.var_t3_dn12 = assign15050_body23_e21337_d_n12;
            locals.var_t3_dn17 = assign15050_body23_e21337_d_n17;
            locals.var_t3_rv = 0.0;
            let (assign15050_body24_e21350, assign15050_body24_e21350_d_n0, assign15050_body24_e21350_d_n2, assign15050_body24_e21350_d_n6, assign15050_body24_e21350_d_n7, assign15050_body24_e21350_d_n10, assign15050_body24_e21350_d_n11, assign15050_body24_e21350_d_n12, assign15050_body24_e21350_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 == 0.0)) && (locals.var_guard452 != 0.0)) {
        let assign15050_body24_e21347: f64 = (locals.var_t0 - locals.var_t2);
        let assign15050_body24_e21348: f64 = (assign15050_body24_e21347).sqrt();
        (assign15050_body24_e21348, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign15050_body24_e21348)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign15050_body24_e21348)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign15050_body24_e21348)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign15050_body24_e21348)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign15050_body24_e21348)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign15050_body24_e21348)), ((locals.var_t0_dn12 - locals.var_t2_dn12) / (2.0 * assign15050_body24_e21348)), ((locals.var_t0_dn17 - locals.var_t2_dn17) / (2.0 * assign15050_body24_e21348)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15050_body24_e21350;
            locals.var_fb_dn0 = assign15050_body24_e21350_d_n0;
            locals.var_fb_dn2 = assign15050_body24_e21350_d_n2;
            locals.var_fb_dn6 = assign15050_body24_e21350_d_n6;
            locals.var_fb_dn7 = assign15050_body24_e21350_d_n7;
            locals.var_fb_dn10 = assign15050_body24_e21350_d_n10;
            locals.var_fb_dn11 = assign15050_body24_e21350_d_n11;
            locals.var_fb_dn12 = assign15050_body24_e21350_d_n12;
            locals.var_fb_dn17 = assign15050_body24_e21350_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15050_body25_e21370, assign15050_body25_e21370_d_n0, assign15050_body25_e21370_d_n2, assign15050_body25_e21370_d_n6, assign15050_body25_e21370_d_n7, assign15050_body25_e21370_d_n10, assign15050_body25_e21370_d_n11, assign15050_body25_e21370_d_n12, assign15050_body25_e21370_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 == 0.0)) && (locals.var_guard452 != 0.0)) {
        let assign15050_body25_e21360: f64 = (locals.var_beta * 0.5);
        let assign15050_body25_e21364: f64 = (locals.var_phi_soib_dpss * locals.var_t3);
        let assign15050_body25_e21365: f64 = (locals.var_t1 - assign15050_body25_e21364);
        let assign15050_body25_e21366: f64 = (assign15050_body25_e21360 * assign15050_body25_e21365);
        let assign15050_body25_e21368: f64 = (assign15050_body25_e21366 / locals.var_fb);
        (assign15050_body25_e21368, ((((assign15050_body25_e21360 * (locals.var_t1_dn0 - ((locals.var_phi_soib_dpss_dn0 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn0)))) * locals.var_fb) - (assign15050_body25_e21366 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body25_e21360 * (locals.var_t1_dn2 - ((locals.var_phi_soib_dpss_dn2 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn2)))) * locals.var_fb) - (assign15050_body25_e21366 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body25_e21360 * (locals.var_t1_dn6 - ((locals.var_phi_soib_dpss_dn6 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn6)))) * locals.var_fb) - (assign15050_body25_e21366 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body25_e21360 * (locals.var_t1_dn7 - ((locals.var_phi_soib_dpss_dn7 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn7)))) * locals.var_fb) - (assign15050_body25_e21366 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign15050_body25_e21365) + (assign15050_body25_e21360 * (locals.var_t1_dn10 - ((locals.var_phi_soib_dpss_dn10 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign15050_body25_e21366 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body25_e21360 * (locals.var_t1_dn11 - ((locals.var_phi_soib_dpss_dn11 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn11)))) * locals.var_fb) - (assign15050_body25_e21366 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body25_e21360 * (locals.var_t1_dn12 - ((locals.var_phi_soib_dpss_dn12 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn12)))) * locals.var_fb) - (assign15050_body25_e21366 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body25_e21360 * (locals.var_t1_dn17 - ((locals.var_phi_soib_dpss_dn17 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn17)))) * locals.var_fb) - (assign15050_body25_e21366 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15050_body25_e21370;
            locals.var_fb_dpss_dn0 = assign15050_body25_e21370_d_n0;
            locals.var_fb_dpss_dn2 = assign15050_body25_e21370_d_n2;
            locals.var_fb_dpss_dn6 = assign15050_body25_e21370_d_n6;
            locals.var_fb_dpss_dn7 = assign15050_body25_e21370_d_n7;
            locals.var_fb_dpss_dn10 = assign15050_body25_e21370_d_n10;
            locals.var_fb_dpss_dn11 = assign15050_body25_e21370_d_n11;
            locals.var_fb_dpss_dn12 = assign15050_body25_e21370_d_n12;
            locals.var_fb_dpss_dn17 = assign15050_body25_e21370_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let (assign15050_body26_e21383, assign15050_body26_e21383_d_n0, assign15050_body26_e21383_d_n2, assign15050_body26_e21383_d_n6, assign15050_body26_e21383_d_n7, assign15050_body26_e21383_d_n10, assign15050_body26_e21383_d_n11, assign15050_body26_e21383_d_n12, assign15050_body26_e21383_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 == 0.0)) && (locals.var_guard452 == 0.0)) {
        let assign15050_body26_e21380: f64 = (-locals.var_chi);
        let assign15050_body26_e21381: f64 = (assign15050_body26_e21380).exp();
        (assign15050_body26_e21381, (assign15050_body26_e21381 * (-locals.var_chi_dn0)), (assign15050_body26_e21381 * (-locals.var_chi_dn2)), (assign15050_body26_e21381 * (-locals.var_chi_dn6)), (assign15050_body26_e21381 * (-locals.var_chi_dn7)), (assign15050_body26_e21381 * (-locals.var_chi_dn10)), (assign15050_body26_e21381 * (-locals.var_chi_dn11)), (assign15050_body26_e21381 * (-locals.var_chi_dn12)), (assign15050_body26_e21381 * (-locals.var_chi_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15050_body26_e21383;
            locals.var_t0_dn0 = assign15050_body26_e21383_d_n0;
            locals.var_t0_dn2 = assign15050_body26_e21383_d_n2;
            locals.var_t0_dn6 = assign15050_body26_e21383_d_n6;
            locals.var_t0_dn7 = assign15050_body26_e21383_d_n7;
            locals.var_t0_dn10 = assign15050_body26_e21383_d_n10;
            locals.var_t0_dn11 = assign15050_body26_e21383_d_n11;
            locals.var_t0_dn12 = assign15050_body26_e21383_d_n12;
            locals.var_t0_dn17 = assign15050_body26_e21383_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15050_body27_e21396, assign15050_body27_e21396_d_n0, assign15050_body27_e21396_d_n2, assign15050_body27_e21396_d_n6, assign15050_body27_e21396_d_n7, assign15050_body27_e21396_d_n10, assign15050_body27_e21396_d_n11, assign15050_body27_e21396_d_n12, assign15050_body27_e21396_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 == 0.0)) && (locals.var_guard452 == 0.0)) {
        let assign15050_body27_e21393: f64 = (-locals.var_chib);
        let assign15050_body27_e21394: f64 = (assign15050_body27_e21393).exp();
        (assign15050_body27_e21394, (assign15050_body27_e21394 * (-locals.var_chib_dn0)), (assign15050_body27_e21394 * (-locals.var_chib_dn2)), (assign15050_body27_e21394 * (-locals.var_chib_dn6)), (assign15050_body27_e21394 * (-locals.var_chib_dn7)), (assign15050_body27_e21394 * (-locals.var_chib_dn10)), (assign15050_body27_e21394 * (-locals.var_chib_dn11)), (assign15050_body27_e21394 * (-locals.var_chib_dn12)), (assign15050_body27_e21394 * (-locals.var_chib_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15050_body27_e21396;
            locals.var_t1_dn0 = assign15050_body27_e21396_d_n0;
            locals.var_t1_dn2 = assign15050_body27_e21396_d_n2;
            locals.var_t1_dn6 = assign15050_body27_e21396_d_n6;
            locals.var_t1_dn7 = assign15050_body27_e21396_d_n7;
            locals.var_t1_dn10 = assign15050_body27_e21396_d_n10;
            locals.var_t1_dn11 = assign15050_body27_e21396_d_n11;
            locals.var_t1_dn12 = assign15050_body27_e21396_d_n12;
            locals.var_t1_dn17 = assign15050_body27_e21396_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign15050_body28_e21414, assign15050_body28_e21414_d_n0, assign15050_body28_e21414_d_n2, assign15050_body28_e21414_d_n6, assign15050_body28_e21414_d_n7, assign15050_body28_e21414_d_n10, assign15050_body28_e21414_d_n11, assign15050_body28_e21414_d_n12, assign15050_body28_e21414_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 == 0.0)) && (locals.var_guard452 == 0.0)) {
        let assign15050_body28_e21407: f64 = (locals.var_chi - locals.var_chib);
        let assign15050_body28_e21410: f64 = (locals.var_t0 - locals.var_t1);
        let assign15050_body28_e21411: f64 = (assign15050_body28_e21407 + assign15050_body28_e21410);
        let assign15050_body28_e21412: f64 = (assign15050_body28_e21411).sqrt();
        (assign15050_body28_e21412, (((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign15050_body28_e21412)), (((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign15050_body28_e21412)), (((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign15050_body28_e21412)), (((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign15050_body28_e21412)), (((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign15050_body28_e21412)), (((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign15050_body28_e21412)), (((locals.var_chi_dn12 - locals.var_chib_dn12) + (locals.var_t0_dn12 - locals.var_t1_dn12)) / (2.0 * assign15050_body28_e21412)), (((locals.var_chi_dn17 - locals.var_chib_dn17) + (locals.var_t0_dn17 - locals.var_t1_dn17)) / (2.0 * assign15050_body28_e21412)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15050_body28_e21414;
            locals.var_fb_dn0 = assign15050_body28_e21414_d_n0;
            locals.var_fb_dn2 = assign15050_body28_e21414_d_n2;
            locals.var_fb_dn6 = assign15050_body28_e21414_d_n6;
            locals.var_fb_dn7 = assign15050_body28_e21414_d_n7;
            locals.var_fb_dn10 = assign15050_body28_e21414_d_n10;
            locals.var_fb_dn11 = assign15050_body28_e21414_d_n11;
            locals.var_fb_dn12 = assign15050_body28_e21414_d_n12;
            locals.var_fb_dn17 = assign15050_body28_e21414_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15050_body29_e21439, assign15050_body29_e21439_d_n0, assign15050_body29_e21439_d_n2, assign15050_body29_e21439_d_n6, assign15050_body29_e21439_d_n7, assign15050_body29_e21439_d_n10, assign15050_body29_e21439_d_n11, assign15050_body29_e21439_d_n12, assign15050_body29_e21439_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard450 == 0.0)) && (locals.var_guard452 == 0.0)) {
        let assign15050_body29_e21425: f64 = (locals.var_beta * 0.5);
        let assign15050_body29_e21428: f64 = (1.0 - locals.var_t0);
        let assign15050_body29_e21432: f64 = (1.0 - locals.var_t1);
        let assign15050_body29_e21433: f64 = (locals.var_phi_soib_dpss * assign15050_body29_e21432);
        let assign15050_body29_e21434: f64 = (assign15050_body29_e21428 - assign15050_body29_e21433);
        let assign15050_body29_e21435: f64 = (assign15050_body29_e21425 * assign15050_body29_e21434);
        let assign15050_body29_e21437: f64 = (assign15050_body29_e21435 / locals.var_fb);
        (assign15050_body29_e21437, ((((assign15050_body29_e21425 * ((-locals.var_t0_dn0) - ((locals.var_phi_soib_dpss_dn0 * assign15050_body29_e21432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn0))))) * locals.var_fb) - (assign15050_body29_e21435 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body29_e21425 * ((-locals.var_t0_dn2) - ((locals.var_phi_soib_dpss_dn2 * assign15050_body29_e21432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn2))))) * locals.var_fb) - (assign15050_body29_e21435 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body29_e21425 * ((-locals.var_t0_dn6) - ((locals.var_phi_soib_dpss_dn6 * assign15050_body29_e21432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn6))))) * locals.var_fb) - (assign15050_body29_e21435 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body29_e21425 * ((-locals.var_t0_dn7) - ((locals.var_phi_soib_dpss_dn7 * assign15050_body29_e21432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn7))))) * locals.var_fb) - (assign15050_body29_e21435 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign15050_body29_e21434) + (assign15050_body29_e21425 * ((-locals.var_t0_dn10) - ((locals.var_phi_soib_dpss_dn10 * assign15050_body29_e21432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign15050_body29_e21435 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body29_e21425 * ((-locals.var_t0_dn11) - ((locals.var_phi_soib_dpss_dn11 * assign15050_body29_e21432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn11))))) * locals.var_fb) - (assign15050_body29_e21435 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body29_e21425 * ((-locals.var_t0_dn12) - ((locals.var_phi_soib_dpss_dn12 * assign15050_body29_e21432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn12))))) * locals.var_fb) - (assign15050_body29_e21435 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign15050_body29_e21425 * ((-locals.var_t0_dn17) - ((locals.var_phi_soib_dpss_dn17 * assign15050_body29_e21432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn17))))) * locals.var_fb) - (assign15050_body29_e21435 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15050_body29_e21439;
            locals.var_fb_dpss_dn0 = assign15050_body29_e21439_d_n0;
            locals.var_fb_dpss_dn2 = assign15050_body29_e21439_d_n2;
            locals.var_fb_dpss_dn6 = assign15050_body29_e21439_d_n6;
            locals.var_fb_dpss_dn7 = assign15050_body29_e21439_d_n7;
            locals.var_fb_dpss_dn10 = assign15050_body29_e21439_d_n10;
            locals.var_fb_dpss_dn11 = assign15050_body29_e21439_d_n11;
            locals.var_fb_dpss_dn12 = assign15050_body29_e21439_d_n12;
            locals.var_fb_dpss_dn17 = assign15050_body29_e21439_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign15050_body30_e21446: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_chi < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard453 = assign15050_body30_e21446;
            locals.var_guard453_rv = 0.0;
            let (assign15050_body31_e21454,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard453 != 0.0)) {
        let assign15050_body31_e21452: f64 = (-1.0);
        (assign15050_body31_e21452,)
    } else {
        (locals.var_flg_zone,)
    }
};
            locals.var_flg_zone = assign15050_body31_e21454;
            locals.var_flg_zone_rv = 0.0;
            let assign15050_body32_e21457: f64 = (-1.0);
            let assign15050_body32_e21458: f64 = if locals.var_flg_zone == assign15050_body32_e21457 { 1.0 } else { 0.0 };
            locals.var_guard454 = assign15050_body32_e21458;
            locals.var_guard454_rv = 0.0;
            let (assign15050_body33_e21465, assign15050_body33_e21465_d_n0, assign15050_body33_e21465_d_n2, assign15050_body33_e21465_d_n6, assign15050_body33_e21465_d_n7, assign15050_body33_e21465_d_n10, assign15050_body33_e21465_d_n11, assign15050_body33_e21465_d_n12, assign15050_body33_e21465_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard454 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign15050_body33_e21465;
            locals.var_wdsoi_dn0 = assign15050_body33_e21465_d_n0;
            locals.var_wdsoi_dn2 = assign15050_body33_e21465_d_n2;
            locals.var_wdsoi_dn6 = assign15050_body33_e21465_d_n6;
            locals.var_wdsoi_dn7 = assign15050_body33_e21465_d_n7;
            locals.var_wdsoi_dn10 = assign15050_body33_e21465_d_n10;
            locals.var_wdsoi_dn11 = assign15050_body33_e21465_d_n11;
            locals.var_wdsoi_dn12 = assign15050_body33_e21465_d_n12;
            locals.var_wdsoi_dn17 = assign15050_body33_e21465_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let (assign15050_body34_e21475, assign15050_body34_e21475_d_n0, assign15050_body34_e21475_d_n2, assign15050_body34_e21475_d_n6, assign15050_body34_e21475_d_n7, assign15050_body34_e21475_d_n10, assign15050_body34_e21475_d_n11, assign15050_body34_e21475_d_n12, assign15050_body34_e21475_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard454 == 0.0)) {
        let assign15050_body34_e21473: f64 = (locals.var_c_w_soi * locals.var_fb);
        (assign15050_body34_e21473, ((locals.var_c_w_soi_dn0 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn0)), ((locals.var_c_w_soi_dn2 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn2)), ((locals.var_c_w_soi_dn6 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn6)), ((locals.var_c_w_soi_dn7 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn7)), ((locals.var_c_w_soi_dn10 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn10)), ((locals.var_c_w_soi_dn11 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn11)), ((locals.var_c_w_soi_dn12 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn12)), ((locals.var_c_w_soi_dn17 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn17)),)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign15050_body34_e21475;
            locals.var_wdsoi_dn0 = assign15050_body34_e21475_d_n0;
            locals.var_wdsoi_dn2 = assign15050_body34_e21475_d_n2;
            locals.var_wdsoi_dn6 = assign15050_body34_e21475_d_n6;
            locals.var_wdsoi_dn7 = assign15050_body34_e21475_d_n7;
            locals.var_wdsoi_dn10 = assign15050_body34_e21475_d_n10;
            locals.var_wdsoi_dn11 = assign15050_body34_e21475_d_n11;
            locals.var_wdsoi_dn12 = assign15050_body34_e21475_d_n12;
            locals.var_wdsoi_dn17 = assign15050_body34_e21475_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let assign15050_body35_e21479: f64 = (p.p237 * 1.01);
            let assign15050_body35_e21480: f64 = if locals.var_wdsoi < assign15050_body35_e21479 { 1.0 } else { 0.0 };
            locals.var_guard455 = assign15050_body35_e21480;
            locals.var_guard455_rv = 0.0;
            let (assign15050_body36_e21487,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard455 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
            locals.var_flg_depmode = assign15050_body36_e21487;
            locals.var_flg_depmode_rv = 0.0;
            let (assign15050_body37_e21495,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard455 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
            locals.var_flg_depmode = assign15050_body37_e21495;
            locals.var_flg_depmode_rv = 0.0;
            let (assign15050_body38_e21502, assign15050_body38_e21502_d_n0, assign15050_body38_e21502_d_n2, assign15050_body38_e21502_d_n6, assign15050_body38_e21502_d_n7, assign15050_body38_e21502_d_n10, assign15050_body38_e21502_d_n11, assign15050_body38_e21502_d_n12, assign15050_body38_e21502_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15050_body38_e21500: f64 = (locals.var_q_nsub * locals.var_wdsoi);
        (assign15050_body38_e21500, ((locals.var_q_nsub_dn0 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn0)), ((locals.var_q_nsub_dn2 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn2)), ((locals.var_q_nsub_dn6 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn6)), ((locals.var_q_nsub_dn7 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn7)), ((locals.var_q_nsub_dn10 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn10)), ((locals.var_q_nsub_dn11 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn11)), ((locals.var_q_nsub_dn12 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn12)), ((locals.var_q_nsub_dn17 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn17)),)
    } else {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    }
};
            locals.var_q_dep_soi = assign15050_body38_e21502;
            locals.var_q_dep_soi_dn0 = assign15050_body38_e21502_d_n0;
            locals.var_q_dep_soi_dn2 = assign15050_body38_e21502_d_n2;
            locals.var_q_dep_soi_dn6 = assign15050_body38_e21502_d_n6;
            locals.var_q_dep_soi_dn7 = assign15050_body38_e21502_d_n7;
            locals.var_q_dep_soi_dn10 = assign15050_body38_e21502_d_n10;
            locals.var_q_dep_soi_dn11 = assign15050_body38_e21502_d_n11;
            locals.var_q_dep_soi_dn12 = assign15050_body38_e21502_d_n12;
            locals.var_q_dep_soi_dn17 = assign15050_body38_e21502_d_n17;
            locals.var_q_dep_soi_rv = 0.0;
            let assign15050_body39_e21505: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard456 = assign15050_body39_e21505;
            locals.var_guard456_rv = 0.0;
            let (assign15050_body40_e21513, assign15050_body40_e21513_d_n0, assign15050_body40_e21513_d_n2, assign15050_body40_e21513_d_n6, assign15050_body40_e21513_d_n7, assign15050_body40_e21513_d_n10, assign15050_body40_e21513_d_n11, assign15050_body40_e21513_d_n12, assign15050_body40_e21513_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard456 != 0.0)) {
        let assign15050_body40_e21511: f64 = (-locals.var_fb);
        (assign15050_body40_e21511, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15050_body40_e21513;
            locals.var_fs02_dn0 = assign15050_body40_e21513_d_n0;
            locals.var_fs02_dn2 = assign15050_body40_e21513_d_n2;
            locals.var_fs02_dn6 = assign15050_body40_e21513_d_n6;
            locals.var_fs02_dn7 = assign15050_body40_e21513_d_n7;
            locals.var_fs02_dn10 = assign15050_body40_e21513_d_n10;
            locals.var_fs02_dn11 = assign15050_body40_e21513_d_n11;
            locals.var_fs02_dn12 = assign15050_body40_e21513_d_n12;
            locals.var_fs02_dn17 = assign15050_body40_e21513_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign15050_body41_e21521, assign15050_body41_e21521_d_n0, assign15050_body41_e21521_d_n2, assign15050_body41_e21521_d_n6, assign15050_body41_e21521_d_n7, assign15050_body41_e21521_d_n10, assign15050_body41_e21521_d_n11, assign15050_body41_e21521_d_n12, assign15050_body41_e21521_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard456 != 0.0)) {
        let assign15050_body41_e21519: f64 = (-locals.var_fb_dpss);
        (assign15050_body41_e21519, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15050_body41_e21521;
            locals.var_fs02_dps0_dn0 = assign15050_body41_e21521_d_n0;
            locals.var_fs02_dps0_dn2 = assign15050_body41_e21521_d_n2;
            locals.var_fs02_dps0_dn6 = assign15050_body41_e21521_d_n6;
            locals.var_fs02_dps0_dn7 = assign15050_body41_e21521_d_n7;
            locals.var_fs02_dps0_dn10 = assign15050_body41_e21521_d_n10;
            locals.var_fs02_dps0_dn11 = assign15050_body41_e21521_d_n11;
            locals.var_fs02_dps0_dn12 = assign15050_body41_e21521_d_n12;
            locals.var_fs02_dps0_dn17 = assign15050_body41_e21521_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let assign15050_body42_e21524: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard457 = assign15050_body42_e21524;
            locals.var_guard457_rv = 0.0;
            let (assign15050_body43_e21534, assign15050_body43_e21534_d_n0, assign15050_body43_e21534_d_n2, assign15050_body43_e21534_d_n6, assign15050_body43_e21534_d_n7, assign15050_body43_e21534_d_n10, assign15050_body43_e21534_d_n11, assign15050_body43_e21534_d_n12, assign15050_body43_e21534_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15050_body43_e21534;
            locals.var_fs02_dn0 = assign15050_body43_e21534_d_n0;
            locals.var_fs02_dn2 = assign15050_body43_e21534_d_n2;
            locals.var_fs02_dn6 = assign15050_body43_e21534_d_n6;
            locals.var_fs02_dn7 = assign15050_body43_e21534_d_n7;
            locals.var_fs02_dn10 = assign15050_body43_e21534_d_n10;
            locals.var_fs02_dn11 = assign15050_body43_e21534_d_n11;
            locals.var_fs02_dn12 = assign15050_body43_e21534_d_n12;
            locals.var_fs02_dn17 = assign15050_body43_e21534_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign15050_body44_e21544, assign15050_body44_e21544_d_n0, assign15050_body44_e21544_d_n2, assign15050_body44_e21544_d_n6, assign15050_body44_e21544_d_n7, assign15050_body44_e21544_d_n10, assign15050_body44_e21544_d_n11, assign15050_body44_e21544_d_n12, assign15050_body44_e21544_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 != 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15050_body44_e21544;
            locals.var_fs02_dps0_dn0 = assign15050_body44_e21544_d_n0;
            locals.var_fs02_dps0_dn2 = assign15050_body44_e21544_d_n2;
            locals.var_fs02_dps0_dn6 = assign15050_body44_e21544_d_n6;
            locals.var_fs02_dps0_dn7 = assign15050_body44_e21544_d_n7;
            locals.var_fs02_dps0_dn10 = assign15050_body44_e21544_d_n10;
            locals.var_fs02_dps0_dn11 = assign15050_body44_e21544_d_n11;
            locals.var_fs02_dps0_dn12 = assign15050_body44_e21544_d_n12;
            locals.var_fs02_dps0_dn17 = assign15050_body44_e21544_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let assign15050_body45_e21547: f64 = if locals.var_chi < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard458 = assign15050_body45_e21547;
            locals.var_guard458_rv = 0.0;
            let (assign15050_body46_e21561, assign15050_body46_e21561_d_n0, assign15050_body46_e21561_d_n2, assign15050_body46_e21561_d_n6, assign15050_body46_e21561_d_n7, assign15050_body46_e21561_d_n10, assign15050_body46_e21561_d_n11, assign15050_body46_e21561_d_n12, assign15050_body46_e21561_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign15050_body46_e21559: f64 = (locals.var_chi).exp();
        (assign15050_body46_e21559, (assign15050_body46_e21559 * locals.var_chi_dn0), (assign15050_body46_e21559 * locals.var_chi_dn2), (assign15050_body46_e21559 * locals.var_chi_dn6), (assign15050_body46_e21559 * locals.var_chi_dn7), (assign15050_body46_e21559 * locals.var_chi_dn10), (assign15050_body46_e21559 * locals.var_chi_dn11), (assign15050_body46_e21559 * locals.var_chi_dn12), (assign15050_body46_e21559 * locals.var_chi_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign15050_body46_e21561;
            locals.var_exp_chi_dn0 = assign15050_body46_e21561_d_n0;
            locals.var_exp_chi_dn2 = assign15050_body46_e21561_d_n2;
            locals.var_exp_chi_dn6 = assign15050_body46_e21561_d_n6;
            locals.var_exp_chi_dn7 = assign15050_body46_e21561_d_n7;
            locals.var_exp_chi_dn10 = assign15050_body46_e21561_d_n10;
            locals.var_exp_chi_dn11 = assign15050_body46_e21561_d_n11;
            locals.var_exp_chi_dn12 = assign15050_body46_e21561_d_n12;
            locals.var_exp_chi_dn17 = assign15050_body46_e21561_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign15050_body47_e21580, assign15050_body47_e21580_d_n0, assign15050_body47_e21580_d_n2, assign15050_body47_e21580_d_n6, assign15050_body47_e21580_d_n7, assign15050_body47_e21580_d_n10, assign15050_body47_e21580_d_n11, assign15050_body47_e21580_d_n12, assign15050_body47_e21580_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign15050_body47_e21576: f64 = (locals.var_chi + 1.0);
        let assign15050_body47_e21577: f64 = (locals.var_exp_chi - assign15050_body47_e21576);
        let assign15050_body47_e21578: f64 = (locals.var_cfs1 * assign15050_body47_e21577);
        (assign15050_body47_e21578, ((locals.var_cfs1_dn0 * assign15050_body47_e21577) + (locals.var_cfs1 * (locals.var_exp_chi_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign15050_body47_e21577) + (locals.var_cfs1 * (locals.var_exp_chi_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn6 * assign15050_body47_e21577) + (locals.var_cfs1 * (locals.var_exp_chi_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign15050_body47_e21577) + (locals.var_cfs1 * (locals.var_exp_chi_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn10 * assign15050_body47_e21577) + (locals.var_cfs1 * (locals.var_exp_chi_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign15050_body47_e21577) + (locals.var_cfs1 * (locals.var_exp_chi_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn12 * assign15050_body47_e21577) + (locals.var_cfs1 * (locals.var_exp_chi_dn12 - locals.var_chi_dn12))), ((locals.var_cfs1_dn17 * assign15050_body47_e21577) + (locals.var_cfs1 * (locals.var_exp_chi_dn17 - locals.var_chi_dn17))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, locals.var_fs01_dn17,)
    }
};
            locals.var_fs01 = assign15050_body47_e21580;
            locals.var_fs01_dn0 = assign15050_body47_e21580_d_n0;
            locals.var_fs01_dn2 = assign15050_body47_e21580_d_n2;
            locals.var_fs01_dn6 = assign15050_body47_e21580_d_n6;
            locals.var_fs01_dn7 = assign15050_body47_e21580_d_n7;
            locals.var_fs01_dn10 = assign15050_body47_e21580_d_n10;
            locals.var_fs01_dn11 = assign15050_body47_e21580_d_n11;
            locals.var_fs01_dn12 = assign15050_body47_e21580_d_n12;
            locals.var_fs01_dn17 = assign15050_body47_e21580_d_n17;
            locals.var_fs01_rv = 0.0;
            let (assign15050_body48_e21599, assign15050_body48_e21599_d_n0, assign15050_body48_e21599_d_n2, assign15050_body48_e21599_d_n6, assign15050_body48_e21599_d_n7, assign15050_body48_e21599_d_n10, assign15050_body48_e21599_d_n11, assign15050_body48_e21599_d_n12, assign15050_body48_e21599_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign15050_body48_e21593: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign15050_body48_e21596: f64 = (locals.var_exp_chi - 1.0);
        let assign15050_body48_e21597: f64 = (assign15050_body48_e21593 * assign15050_body48_e21596);
        (assign15050_body48_e21597, (((locals.var_cfs1_dn0 * locals.var_beta) * assign15050_body48_e21596) + (assign15050_body48_e21593 * locals.var_exp_chi_dn0)), (((locals.var_cfs1_dn2 * locals.var_beta) * assign15050_body48_e21596) + (assign15050_body48_e21593 * locals.var_exp_chi_dn2)), (((locals.var_cfs1_dn6 * locals.var_beta) * assign15050_body48_e21596) + (assign15050_body48_e21593 * locals.var_exp_chi_dn6)), (((locals.var_cfs1_dn7 * locals.var_beta) * assign15050_body48_e21596) + (assign15050_body48_e21593 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * assign15050_body48_e21596) + (assign15050_body48_e21593 * locals.var_exp_chi_dn10)), (((locals.var_cfs1_dn11 * locals.var_beta) * assign15050_body48_e21596) + (assign15050_body48_e21593 * locals.var_exp_chi_dn11)), (((locals.var_cfs1_dn12 * locals.var_beta) * assign15050_body48_e21596) + (assign15050_body48_e21593 * locals.var_exp_chi_dn12)), (((locals.var_cfs1_dn17 * locals.var_beta) * assign15050_body48_e21596) + (assign15050_body48_e21593 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, locals.var_fs01_dps0_dn17,)
    }
};
            locals.var_fs01_dps0 = assign15050_body48_e21599;
            locals.var_fs01_dps0_dn0 = assign15050_body48_e21599_d_n0;
            locals.var_fs01_dps0_dn2 = assign15050_body48_e21599_d_n2;
            locals.var_fs01_dps0_dn6 = assign15050_body48_e21599_d_n6;
            locals.var_fs01_dps0_dn7 = assign15050_body48_e21599_d_n7;
            locals.var_fs01_dps0_dn10 = assign15050_body48_e21599_d_n10;
            locals.var_fs01_dps0_dn11 = assign15050_body48_e21599_d_n11;
            locals.var_fs01_dps0_dn12 = assign15050_body48_e21599_d_n12;
            locals.var_fs01_dps0_dn17 = assign15050_body48_e21599_d_n17;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign15050_body49_e21616, assign15050_body49_e21616_d_n0, assign15050_body49_e21616_d_n2, assign15050_body49_e21616_d_n6, assign15050_body49_e21616_d_n7, assign15050_body49_e21616_d_n10, assign15050_body49_e21616_d_n11, assign15050_body49_e21616_d_n12, assign15050_body49_e21616_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 == 0.0)) {
        let assign15050_body49_e21613: f64 = (locals.var_beta * locals.var_phi_s0_soi);
        let assign15050_body49_e21614: f64 = (assign15050_body49_e21613).exp();
        (assign15050_body49_e21614, (assign15050_body49_e21614 * (locals.var_beta * locals.var_phi_s0_soi_dn0)), (assign15050_body49_e21614 * (locals.var_beta * locals.var_phi_s0_soi_dn2)), (assign15050_body49_e21614 * (locals.var_beta * locals.var_phi_s0_soi_dn6)), (assign15050_body49_e21614 * (locals.var_beta * locals.var_phi_s0_soi_dn7)), (assign15050_body49_e21614 * ((locals.var_beta_dn10 * locals.var_phi_s0_soi) + (locals.var_beta * locals.var_phi_s0_soi_dn10))), (assign15050_body49_e21614 * (locals.var_beta * locals.var_phi_s0_soi_dn11)), (assign15050_body49_e21614 * (locals.var_beta * locals.var_phi_s0_soi_dn12)), (assign15050_body49_e21614 * (locals.var_beta * locals.var_phi_s0_soi_dn17)),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn12, locals.var_exp_bps0_dn17,)
    }
};
            locals.var_exp_bps0 = assign15050_body49_e21616;
            locals.var_exp_bps0_dn0 = assign15050_body49_e21616_d_n0;
            locals.var_exp_bps0_dn2 = assign15050_body49_e21616_d_n2;
            locals.var_exp_bps0_dn6 = assign15050_body49_e21616_d_n6;
            locals.var_exp_bps0_dn7 = assign15050_body49_e21616_d_n7;
            locals.var_exp_bps0_dn10 = assign15050_body49_e21616_d_n10;
            locals.var_exp_bps0_dn11 = assign15050_body49_e21616_d_n11;
            locals.var_exp_bps0_dn12 = assign15050_body49_e21616_d_n12;
            locals.var_exp_bps0_dn17 = assign15050_body49_e21616_d_n17;
            locals.var_exp_bps0_rv = 0.0;
            let (assign15050_body50_e21638, assign15050_body50_e21638_d_n0, assign15050_body50_e21638_d_n2, assign15050_body50_e21638_d_n6, assign15050_body50_e21638_d_n7, assign15050_body50_e21638_d_n10, assign15050_body50_e21638_d_n11, assign15050_body50_e21638_d_n12, assign15050_body50_e21638_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 == 0.0)) {
        let assign15050_body50_e21633: f64 = (locals.var_chi + 1.0);
        let assign15050_body50_e21634: f64 = (locals.var_exp_bvbs * assign15050_body50_e21633);
        let assign15050_body50_e21635: f64 = (locals.var_exp_bps0 - assign15050_body50_e21634);
        let assign15050_body50_e21636: f64 = (locals.var_cnst1soi * assign15050_body50_e21635);
        (assign15050_body50_e21636, ((locals.var_cnst1soi_dn0 * assign15050_body50_e21635) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign15050_body50_e21633) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1soi_dn2 * assign15050_body50_e21635) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign15050_body50_e21633) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1soi_dn6 * assign15050_body50_e21635) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign15050_body50_e21633) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1soi_dn7 * assign15050_body50_e21635) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign15050_body50_e21633) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1soi_dn10 * assign15050_body50_e21635) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign15050_body50_e21633) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1soi_dn11 * assign15050_body50_e21635) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign15050_body50_e21633) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1soi_dn12 * assign15050_body50_e21635) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn12 - ((locals.var_exp_bvbs_dn12 * assign15050_body50_e21633) + (locals.var_exp_bvbs * locals.var_chi_dn12))))), ((locals.var_cnst1soi_dn17 * assign15050_body50_e21635) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn17 - ((locals.var_exp_bvbs_dn17 * assign15050_body50_e21633) + (locals.var_exp_bvbs * locals.var_chi_dn17))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, locals.var_fs01_dn17,)
    }
};
            locals.var_fs01 = assign15050_body50_e21638;
            locals.var_fs01_dn0 = assign15050_body50_e21638_d_n0;
            locals.var_fs01_dn2 = assign15050_body50_e21638_d_n2;
            locals.var_fs01_dn6 = assign15050_body50_e21638_d_n6;
            locals.var_fs01_dn7 = assign15050_body50_e21638_d_n7;
            locals.var_fs01_dn10 = assign15050_body50_e21638_d_n10;
            locals.var_fs01_dn11 = assign15050_body50_e21638_d_n11;
            locals.var_fs01_dn12 = assign15050_body50_e21638_d_n12;
            locals.var_fs01_dn17 = assign15050_body50_e21638_d_n17;
            locals.var_fs01_rv = 0.0;
            let (assign15050_body51_e21658, assign15050_body51_e21658_d_n0, assign15050_body51_e21658_d_n2, assign15050_body51_e21658_d_n6, assign15050_body51_e21658_d_n7, assign15050_body51_e21658_d_n10, assign15050_body51_e21658_d_n11, assign15050_body51_e21658_d_n12, assign15050_body51_e21658_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 == 0.0)) {
        let assign15050_body51_e21652: f64 = (locals.var_cnst1soi * locals.var_beta);
        let assign15050_body51_e21655: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign15050_body51_e21656: f64 = (assign15050_body51_e21652 * assign15050_body51_e21655);
        (assign15050_body51_e21656, (((locals.var_cnst1soi_dn0 * locals.var_beta) * assign15050_body51_e21655) + (assign15050_body51_e21652 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), (((locals.var_cnst1soi_dn2 * locals.var_beta) * assign15050_body51_e21655) + (assign15050_body51_e21652 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), (((locals.var_cnst1soi_dn6 * locals.var_beta) * assign15050_body51_e21655) + (assign15050_body51_e21652 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), (((locals.var_cnst1soi_dn7 * locals.var_beta) * assign15050_body51_e21655) + (assign15050_body51_e21652 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1soi_dn10 * locals.var_beta) + (locals.var_cnst1soi * locals.var_beta_dn10)) * assign15050_body51_e21655) + (assign15050_body51_e21652 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), (((locals.var_cnst1soi_dn11 * locals.var_beta) * assign15050_body51_e21655) + (assign15050_body51_e21652 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), (((locals.var_cnst1soi_dn12 * locals.var_beta) * assign15050_body51_e21655) + (assign15050_body51_e21652 * (locals.var_exp_bps0_dn12 - locals.var_exp_bvbs_dn12))), (((locals.var_cnst1soi_dn17 * locals.var_beta) * assign15050_body51_e21655) + (assign15050_body51_e21652 * (locals.var_exp_bps0_dn17 - locals.var_exp_bvbs_dn17))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, locals.var_fs01_dps0_dn17,)
    }
};
            locals.var_fs01_dps0 = assign15050_body51_e21658;
            locals.var_fs01_dps0_dn0 = assign15050_body51_e21658_d_n0;
            locals.var_fs01_dps0_dn2 = assign15050_body51_e21658_d_n2;
            locals.var_fs01_dps0_dn6 = assign15050_body51_e21658_d_n6;
            locals.var_fs01_dps0_dn7 = assign15050_body51_e21658_d_n7;
            locals.var_fs01_dps0_dn10 = assign15050_body51_e21658_d_n10;
            locals.var_fs01_dps0_dn11 = assign15050_body51_e21658_d_n11;
            locals.var_fs01_dps0_dn12 = assign15050_body51_e21658_d_n12;
            locals.var_fs01_dps0_dn17 = assign15050_body51_e21658_d_n17;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign15050_body52_e21674, assign15050_body52_e21674_d_n0, assign15050_body52_e21674_d_n2, assign15050_body52_e21674_d_n6, assign15050_body52_e21674_d_n7, assign15050_body52_e21674_d_n10, assign15050_body52_e21674_d_n11, assign15050_body52_e21674_d_n12, assign15050_body52_e21674_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 == 0.0)) {
        let assign15050_body52_e21669: f64 = (locals.var_fb * locals.var_fb);
        let assign15050_body52_e21671: f64 = (assign15050_body52_e21669 + locals.var_fs01);
        let assign15050_body52_e21672: f64 = (assign15050_body52_e21671).sqrt();
        (assign15050_body52_e21672, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign15050_body52_e21672)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign15050_body52_e21672)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign15050_body52_e21672)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign15050_body52_e21672)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign15050_body52_e21672)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign15050_body52_e21672)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fs01_dn12) / (2.0 * assign15050_body52_e21672)), ((((locals.var_fb_dn17 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn17)) + locals.var_fs01_dn17) / (2.0 * assign15050_body52_e21672)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15050_body52_e21674;
            locals.var_fs02_dn0 = assign15050_body52_e21674_d_n0;
            locals.var_fs02_dn2 = assign15050_body52_e21674_d_n2;
            locals.var_fs02_dn6 = assign15050_body52_e21674_d_n6;
            locals.var_fs02_dn7 = assign15050_body52_e21674_d_n7;
            locals.var_fs02_dn10 = assign15050_body52_e21674_d_n10;
            locals.var_fs02_dn11 = assign15050_body52_e21674_d_n11;
            locals.var_fs02_dn12 = assign15050_body52_e21674_d_n12;
            locals.var_fs02_dn17 = assign15050_body52_e21674_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign15050_body53_e21695, assign15050_body53_e21695_d_n0, assign15050_body53_e21695_d_n2, assign15050_body53_e21695_d_n6, assign15050_body53_e21695_d_n7, assign15050_body53_e21695_d_n10, assign15050_body53_e21695_d_n11, assign15050_body53_e21695_d_n12, assign15050_body53_e21695_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 == 0.0)) {
        let assign15050_body53_e21686: f64 = (2.0 * locals.var_fb_dpss);
        let assign15050_body53_e21688: f64 = (assign15050_body53_e21686 * locals.var_fb);
        let assign15050_body53_e21690: f64 = (assign15050_body53_e21688 + locals.var_fs01_dps0);
        let assign15050_body53_e21691: f64 = (0.5 * assign15050_body53_e21690);
        let assign15050_body53_e21693: f64 = (assign15050_body53_e21691 / locals.var_fs02);
        (assign15050_body53_e21693, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign15050_body53_e21686 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign15050_body53_e21691 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign15050_body53_e21686 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign15050_body53_e21691 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign15050_body53_e21686 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign15050_body53_e21691 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign15050_body53_e21686 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign15050_body53_e21691 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign15050_body53_e21686 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign15050_body53_e21691 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign15050_body53_e21686 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign15050_body53_e21691 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn12) * locals.var_fb) + (assign15050_body53_e21686 * locals.var_fb_dn12)) + locals.var_fs01_dps0_dn12)) * locals.var_fs02) - (assign15050_body53_e21691 * locals.var_fs02_dn12)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn17) * locals.var_fb) + (assign15050_body53_e21686 * locals.var_fb_dn17)) + locals.var_fs01_dps0_dn17)) * locals.var_fs02) - (assign15050_body53_e21691 * locals.var_fs02_dn17)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15050_body53_e21695;
            locals.var_fs02_dps0_dn0 = assign15050_body53_e21695_d_n0;
            locals.var_fs02_dps0_dn2 = assign15050_body53_e21695_d_n2;
            locals.var_fs02_dps0_dn6 = assign15050_body53_e21695_d_n6;
            locals.var_fs02_dps0_dn7 = assign15050_body53_e21695_d_n7;
            locals.var_fs02_dps0_dn10 = assign15050_body53_e21695_d_n10;
            locals.var_fs02_dps0_dn11 = assign15050_body53_e21695_d_n11;
            locals.var_fs02_dps0_dn12 = assign15050_body53_e21695_d_n12;
            locals.var_fs02_dps0_dn17 = assign15050_body53_e21695_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign15050_body54_e21711, assign15050_body54_e21711_d_n0, assign15050_body54_e21711_d_n2, assign15050_body54_e21711_d_n6, assign15050_body54_e21711_d_n7, assign15050_body54_e21711_d_n10, assign15050_body54_e21711_d_n11, assign15050_body54_e21711_d_n12, assign15050_body54_e21711_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15050_body54_e21699: f64 = (-locals.var_vgp);
        let assign15050_body54_e21701: f64 = (assign15050_body54_e21699 + locals.var_phi_s0_soi);
        let assign15050_body54_e21704: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign15050_body54_e21705: f64 = (assign15050_body54_e21701 + assign15050_body54_e21704);
        let assign15050_body54_e21708: f64 = (locals.var_c_fox_inv * locals.var_qhs);
        let assign15050_body54_e21709: f64 = (assign15050_body54_e21705 - assign15050_body54_e21708);
        (assign15050_body54_e21709, ((((-locals.var_vgp_dn0) + locals.var_phi_s0_soi_dn0) + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))) - ((locals.var_c_fox_inv_dn0 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn0))), ((((-locals.var_vgp_dn2) + locals.var_phi_s0_soi_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))) - ((locals.var_c_fox_inv_dn2 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn2))), ((((-locals.var_vgp_dn6) + locals.var_phi_s0_soi_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))) - ((locals.var_c_fox_inv_dn6 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn6))), ((((-locals.var_vgp_dn7) + locals.var_phi_s0_soi_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))) - ((locals.var_c_fox_inv_dn7 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn7))), ((((-locals.var_vgp_dn10) + locals.var_phi_s0_soi_dn10) + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))) - ((locals.var_c_fox_inv_dn10 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn10))), ((((-locals.var_vgp_dn11) + locals.var_phi_s0_soi_dn11) + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))) - ((locals.var_c_fox_inv_dn11 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn11))), ((((-locals.var_vgp_dn12) + locals.var_phi_s0_soi_dn12) + ((locals.var_fac1_dn12 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn12))) - ((locals.var_c_fox_inv_dn12 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn12))), ((((-locals.var_vgp_dn17) + locals.var_phi_s0_soi_dn17) + ((locals.var_fac1_dn17 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn17))) - ((locals.var_c_fox_inv_dn17 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn17))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn12, locals.var_fs0_dn17,)
    }
};
            locals.var_fs0 = assign15050_body54_e21711;
            locals.var_fs0_dn0 = assign15050_body54_e21711_d_n0;
            locals.var_fs0_dn2 = assign15050_body54_e21711_d_n2;
            locals.var_fs0_dn6 = assign15050_body54_e21711_d_n6;
            locals.var_fs0_dn7 = assign15050_body54_e21711_d_n7;
            locals.var_fs0_dn10 = assign15050_body54_e21711_d_n10;
            locals.var_fs0_dn11 = assign15050_body54_e21711_d_n11;
            locals.var_fs0_dn12 = assign15050_body54_e21711_d_n12;
            locals.var_fs0_dn17 = assign15050_body54_e21711_d_n17;
            locals.var_fs0_rv = 0.0;
            let (assign15050_body55_e21720, assign15050_body55_e21720_d_n0, assign15050_body55_e21720_d_n2, assign15050_body55_e21720_d_n6, assign15050_body55_e21720_d_n7, assign15050_body55_e21720_d_n10, assign15050_body55_e21720_d_n11, assign15050_body55_e21720_d_n12, assign15050_body55_e21720_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15050_body55_e21717: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign15050_body55_e21718: f64 = (1.0 + assign15050_body55_e21717);
        (assign15050_body55_e21718, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn12 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn12)), ((locals.var_fac1_dn17 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn17)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn12, locals.var_fs0_dps0_dn17,)
    }
};
            locals.var_fs0_dps0 = assign15050_body55_e21720;
            locals.var_fs0_dps0_dn0 = assign15050_body55_e21720_d_n0;
            locals.var_fs0_dps0_dn2 = assign15050_body55_e21720_d_n2;
            locals.var_fs0_dps0_dn6 = assign15050_body55_e21720_d_n6;
            locals.var_fs0_dps0_dn7 = assign15050_body55_e21720_d_n7;
            locals.var_fs0_dps0_dn10 = assign15050_body55_e21720_d_n10;
            locals.var_fs0_dps0_dn11 = assign15050_body55_e21720_d_n11;
            locals.var_fs0_dps0_dn12 = assign15050_body55_e21720_d_n12;
            locals.var_fs0_dps0_dn17 = assign15050_body55_e21720_d_n17;
            locals.var_fs0_dps0_rv = 0.0;
            let assign15050_body56_e21723: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard459 = assign15050_body56_e21723;
            locals.var_guard459_rv = 0.0;
            let (assign15050_body57_e21732,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard459 != 0.0)) {
        let assign15050_body57_e21730: f64 = (locals.var_lp_s0_max + 1.0);
        (assign15050_body57_e21730,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign15050_body57_e21732;
            locals.var_lp_s0_rv = 0.0;
            let (assign15050_body58_e21743, assign15050_body58_e21743_d_n0, assign15050_body58_e21743_d_n2, assign15050_body58_e21743_d_n6, assign15050_body58_e21743_d_n7, assign15050_body58_e21743_d_n10, assign15050_body58_e21743_d_n11, assign15050_body58_e21743_d_n12, assign15050_body58_e21743_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard459 == 0.0)) {
        let assign15050_body58_e21739: f64 = (-locals.var_fs0);
        let assign15050_body58_e21741: f64 = (assign15050_body58_e21739 / locals.var_fs0_dps0);
        (assign15050_body58_e21741, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign15050_body58_e21739 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign15050_body58_e21739 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign15050_body58_e21739 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign15050_body58_e21739 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign15050_body58_e21739 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign15050_body58_e21739 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn12) * locals.var_fs0_dps0) - (assign15050_body58_e21739 * locals.var_fs0_dps0_dn12)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn17) * locals.var_fs0_dps0) - (assign15050_body58_e21739 * locals.var_fs0_dps0_dn17)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign15050_body58_e21743;
            locals.var_dps0_dn0 = assign15050_body58_e21743_d_n0;
            locals.var_dps0_dn2 = assign15050_body58_e21743_d_n2;
            locals.var_dps0_dn6 = assign15050_body58_e21743_d_n6;
            locals.var_dps0_dn7 = assign15050_body58_e21743_d_n7;
            locals.var_dps0_dn10 = assign15050_body58_e21743_d_n10;
            locals.var_dps0_dn11 = assign15050_body58_e21743_d_n11;
            locals.var_dps0_dn12 = assign15050_body58_e21743_d_n12;
            locals.var_dps0_dn17 = assign15050_body58_e21743_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign15050_body59_e21764, assign15050_body59_e21764_d_n0, assign15050_body59_e21764_d_n2, assign15050_body59_e21764_d_n6, assign15050_body59_e21764_d_n7, assign15050_body59_e21764_d_n10, assign15050_body59_e21764_d_n11, assign15050_body59_e21764_d_n12, assign15050_body59_e21764_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard459 == 0.0)) {
        let assign15050_body59_e21751: f64 = (0.5 * 0.1);
        let assign15050_body59_e21755: f64 = (locals.var_phi_s0_soi).abs();
        let (assign15050_body59_e21760, assign15050_body59_e21760_d_n0, assign15050_body59_e21760_d_n2, assign15050_body59_e21760_d_n6, assign15050_body59_e21760_d_n7, assign15050_body59_e21760_d_n10, assign15050_body59_e21760_d_n11, assign15050_body59_e21760_d_n12, assign15050_body59_e21760_d_n17,) = {
            if (1.0 >= assign15050_body59_e21755) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15050_body59_e21759: f64 = (locals.var_phi_s0_soi).abs();
                (assign15050_body59_e21759, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn0 } else { (-locals.var_phi_s0_soi_dn0) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn2 } else { (-locals.var_phi_s0_soi_dn2) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn6 } else { (-locals.var_phi_s0_soi_dn6) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn7 } else { (-locals.var_phi_s0_soi_dn7) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn10 } else { (-locals.var_phi_s0_soi_dn10) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn11 } else { (-locals.var_phi_s0_soi_dn11) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn12 } else { (-locals.var_phi_s0_soi_dn12) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn17 } else { (-locals.var_phi_s0_soi_dn17) },)
            }
        };
        let assign15050_body59_e21761: f64 = (1.0 + assign15050_body59_e21760);
        let assign15050_body59_e21762: f64 = (assign15050_body59_e21751 * assign15050_body59_e21761);
        (assign15050_body59_e21762, (assign15050_body59_e21751 * assign15050_body59_e21760_d_n0), (assign15050_body59_e21751 * assign15050_body59_e21760_d_n2), (assign15050_body59_e21751 * assign15050_body59_e21760_d_n6), (assign15050_body59_e21751 * assign15050_body59_e21760_d_n7), (assign15050_body59_e21751 * assign15050_body59_e21760_d_n10), (assign15050_body59_e21751 * assign15050_body59_e21760_d_n11), (assign15050_body59_e21751 * assign15050_body59_e21760_d_n12), (assign15050_body59_e21751 * assign15050_body59_e21760_d_n17),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, locals.var_dplim_dn17,)
    }
};
            locals.var_dplim = assign15050_body59_e21764;
            locals.var_dplim_dn0 = assign15050_body59_e21764_d_n0;
            locals.var_dplim_dn2 = assign15050_body59_e21764_d_n2;
            locals.var_dplim_dn6 = assign15050_body59_e21764_d_n6;
            locals.var_dplim_dn7 = assign15050_body59_e21764_d_n7;
            locals.var_dplim_dn10 = assign15050_body59_e21764_d_n10;
            locals.var_dplim_dn11 = assign15050_body59_e21764_d_n11;
            locals.var_dplim_dn12 = assign15050_body59_e21764_d_n12;
            locals.var_dplim_dn17 = assign15050_body59_e21764_d_n17;
            locals.var_dplim_rv = 0.0;
            let assign15050_body60_e21766: f64 = (locals.var_dps0).abs();
            let assign15050_body60_e21768: f64 = if assign15050_body60_e21766 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard460 = assign15050_body60_e21768;
            locals.var_guard460_rv = 0.0;
            let (assign15050_body61_e21786, assign15050_body61_e21786_d_n0, assign15050_body61_e21786_d_n2, assign15050_body61_e21786_d_n6, assign15050_body61_e21786_d_n7, assign15050_body61_e21786_d_n10, assign15050_body61_e21786_d_n11, assign15050_body61_e21786_d_n12, assign15050_body61_e21786_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard459 == 0.0)) && (locals.var_guard460 != 0.0)) {
        let (assign15050_body61_e21783,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign15050_body61_e21782: f64 = (-1.0);
                (assign15050_body61_e21782,)
            }
        };
        let assign15050_body61_e21784: f64 = (locals.var_dplim * assign15050_body61_e21783);
        (assign15050_body61_e21784, (locals.var_dplim_dn0 * assign15050_body61_e21783), (locals.var_dplim_dn2 * assign15050_body61_e21783), (locals.var_dplim_dn6 * assign15050_body61_e21783), (locals.var_dplim_dn7 * assign15050_body61_e21783), (locals.var_dplim_dn10 * assign15050_body61_e21783), (locals.var_dplim_dn11 * assign15050_body61_e21783), (locals.var_dplim_dn12 * assign15050_body61_e21783), (locals.var_dplim_dn17 * assign15050_body61_e21783),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign15050_body61_e21786;
            locals.var_dps0_dn0 = assign15050_body61_e21786_d_n0;
            locals.var_dps0_dn2 = assign15050_body61_e21786_d_n2;
            locals.var_dps0_dn6 = assign15050_body61_e21786_d_n6;
            locals.var_dps0_dn7 = assign15050_body61_e21786_d_n7;
            locals.var_dps0_dn10 = assign15050_body61_e21786_d_n10;
            locals.var_dps0_dn11 = assign15050_body61_e21786_d_n11;
            locals.var_dps0_dn12 = assign15050_body61_e21786_d_n12;
            locals.var_dps0_dn17 = assign15050_body61_e21786_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign15050_body62_e21796, assign15050_body62_e21796_d_n0, assign15050_body62_e21796_d_n2, assign15050_body62_e21796_d_n6, assign15050_body62_e21796_d_n7, assign15050_body62_e21796_d_n10, assign15050_body62_e21796_d_n11, assign15050_body62_e21796_d_n12, assign15050_body62_e21796_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard459 == 0.0)) {
        let assign15050_body62_e21794: f64 = (locals.var_phi_s0_soi + locals.var_dps0);
        (assign15050_body62_e21794, (locals.var_phi_s0_soi_dn0 + locals.var_dps0_dn0), (locals.var_phi_s0_soi_dn2 + locals.var_dps0_dn2), (locals.var_phi_s0_soi_dn6 + locals.var_dps0_dn6), (locals.var_phi_s0_soi_dn7 + locals.var_dps0_dn7), (locals.var_phi_s0_soi_dn10 + locals.var_dps0_dn10), (locals.var_phi_s0_soi_dn11 + locals.var_dps0_dn11), (locals.var_phi_s0_soi_dn12 + locals.var_dps0_dn12), (locals.var_phi_s0_soi_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
            locals.var_phi_s0_soi = assign15050_body62_e21796;
            locals.var_phi_s0_soi_dn0 = assign15050_body62_e21796_d_n0;
            locals.var_phi_s0_soi_dn2 = assign15050_body62_e21796_d_n2;
            locals.var_phi_s0_soi_dn6 = assign15050_body62_e21796_d_n6;
            locals.var_phi_s0_soi_dn7 = assign15050_body62_e21796_d_n7;
            locals.var_phi_s0_soi_dn10 = assign15050_body62_e21796_d_n10;
            locals.var_phi_s0_soi_dn11 = assign15050_body62_e21796_d_n11;
            locals.var_phi_s0_soi_dn12 = assign15050_body62_e21796_d_n12;
            locals.var_phi_s0_soi_dn17 = assign15050_body62_e21796_d_n17;
            locals.var_phi_s0_soi_rv = 0.0;
            let assign15050_body63_e21798: f64 = (locals.var_dps0).abs();
            let assign15050_body63_e21802: f64 = (locals.var_fs0).abs();
            let assign15050_body63_e21805: f64 = if ((assign15050_body63_e21798 <= 5e-12) && (assign15050_body63_e21802 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard461 = assign15050_body63_e21805;
            locals.var_guard461_rv = 0.0;
            let (assign15050_body64_e21815,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard459 == 0.0)) && (locals.var_guard461 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign15050_body64_e21815;
            locals.var_flg_conv_rv = 0.0;
            let (assign15050_body65_e21822,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15050_body65_e21820: f64 = (locals.var_lp_s0 + 1.0);
        (assign15050_body65_e21820,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign15050_body65_e21822;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_53(
        locals: &mut StampLocals,
    ) {
        let (assign15060_e21829,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15060_e21827: f64 = (locals.var_lp_s0 - 1.0);
        (assign15060_e21827,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign15060_e21829;
        locals.var_lp_s0_rv = 0.0;

        let (assign15070_e21834, assign15070_e21834_d_n0, assign15070_e21834_d_n2, assign15070_e21834_d_n6, assign15070_e21834_d_n7, assign15070_e21834_d_n10, assign15070_e21834_d_n11, assign15070_e21834_d_n12, assign15070_e21834_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    } else {
        (locals.var_q_deps0, locals.var_q_deps0_dn0, locals.var_q_deps0_dn2, locals.var_q_deps0_dn6, locals.var_q_deps0_dn7, locals.var_q_deps0_dn10, locals.var_q_deps0_dn11, locals.var_q_deps0_dn12, locals.var_q_deps0_dn17,)
    }
};
        locals.var_q_deps0 = assign15070_e21834;
        locals.var_q_deps0_dn0 = assign15070_e21834_d_n0;
        locals.var_q_deps0_dn2 = assign15070_e21834_d_n2;
        locals.var_q_deps0_dn6 = assign15070_e21834_d_n6;
        locals.var_q_deps0_dn7 = assign15070_e21834_d_n7;
        locals.var_q_deps0_dn10 = assign15070_e21834_d_n10;
        locals.var_q_deps0_dn11 = assign15070_e21834_d_n11;
        locals.var_q_deps0_dn12 = assign15070_e21834_d_n12;
        locals.var_q_deps0_dn17 = assign15070_e21834_d_n17;
        locals.var_q_deps0_rv = 0.0;

        let (assign15080_e21839, assign15080_e21839_d_n0, assign15080_e21839_d_n2, assign15080_e21839_d_n6, assign15080_e21839_d_n7, assign15080_e21839_d_n10, assign15080_e21839_d_n11, assign15080_e21839_d_n12, assign15080_e21839_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        (locals.var_q_deps0, locals.var_q_deps0_dn0, locals.var_q_deps0_dn2, locals.var_q_deps0_dn6, locals.var_q_deps0_dn7, locals.var_q_deps0_dn10, locals.var_q_deps0_dn11, locals.var_q_deps0_dn12, locals.var_q_deps0_dn17,)
    } else {
        (locals.var_q_dep0, locals.var_q_dep0_dn0, locals.var_q_dep0_dn2, locals.var_q_dep0_dn6, locals.var_q_dep0_dn7, locals.var_q_dep0_dn10, locals.var_q_dep0_dn11, locals.var_q_dep0_dn12, locals.var_q_dep0_dn17,)
    }
};
        locals.var_q_dep0 = assign15080_e21839;
        locals.var_q_dep0_dn0 = assign15080_e21839_d_n0;
        locals.var_q_dep0_dn2 = assign15080_e21839_d_n2;
        locals.var_q_dep0_dn6 = assign15080_e21839_d_n6;
        locals.var_q_dep0_dn7 = assign15080_e21839_d_n7;
        locals.var_q_dep0_dn10 = assign15080_e21839_d_n10;
        locals.var_q_dep0_dn11 = assign15080_e21839_d_n11;
        locals.var_q_dep0_dn12 = assign15080_e21839_d_n12;
        locals.var_q_dep0_dn17 = assign15080_e21839_d_n17;
        locals.var_q_dep0_rv = 0.0;

        let (assign15090_e21844, assign15090_e21844_d_n0, assign15090_e21844_d_n2, assign15090_e21844_d_n6, assign15090_e21844_d_n7, assign15090_e21844_d_n10, assign15090_e21844_d_n11, assign15090_e21844_d_n12, assign15090_e21844_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign15090_e21844;
        locals.var_ps0_dn0 = assign15090_e21844_d_n0;
        locals.var_ps0_dn2 = assign15090_e21844_d_n2;
        locals.var_ps0_dn6 = assign15090_e21844_d_n6;
        locals.var_ps0_dn7 = assign15090_e21844_d_n7;
        locals.var_ps0_dn10 = assign15090_e21844_d_n10;
        locals.var_ps0_dn11 = assign15090_e21844_d_n11;
        locals.var_ps0_dn12 = assign15090_e21844_d_n12;
        locals.var_ps0_dn17 = assign15090_e21844_d_n17;
        locals.var_ps0_rv = 0.0;

        let (assign15110_e21856, assign15110_e21856_d_n0, assign15110_e21856_d_n2, assign15110_e21856_d_n6, assign15110_e21856_d_n7, assign15110_e21856_d_n10, assign15110_e21856_d_n11, assign15110_e21856_d_n12, assign15110_e21856_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15110_e21854: f64 = (locals.var_q_deps0 / locals.var_cnst0soi);
        (assign15110_e21854, (((locals.var_q_deps0_dn0 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn0)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn2 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn2)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn6 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn6)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn7 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn7)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn10 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn10)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn11 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn11)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn12 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn12)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn17 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn17)) / (locals.var_cnst0soi * locals.var_cnst0soi)),)
    } else {
        (locals.var_q_deps0_soi_o_cnst0soi, locals.var_q_deps0_soi_o_cnst0soi_dn0, locals.var_q_deps0_soi_o_cnst0soi_dn2, locals.var_q_deps0_soi_o_cnst0soi_dn6, locals.var_q_deps0_soi_o_cnst0soi_dn7, locals.var_q_deps0_soi_o_cnst0soi_dn10, locals.var_q_deps0_soi_o_cnst0soi_dn11, locals.var_q_deps0_soi_o_cnst0soi_dn12, locals.var_q_deps0_soi_o_cnst0soi_dn17,)
    }
};
        locals.var_q_deps0_soi_o_cnst0soi = assign15110_e21856;
        locals.var_q_deps0_soi_o_cnst0soi_dn0 = assign15110_e21856_d_n0;
        locals.var_q_deps0_soi_o_cnst0soi_dn2 = assign15110_e21856_d_n2;
        locals.var_q_deps0_soi_o_cnst0soi_dn6 = assign15110_e21856_d_n6;
        locals.var_q_deps0_soi_o_cnst0soi_dn7 = assign15110_e21856_d_n7;
        locals.var_q_deps0_soi_o_cnst0soi_dn10 = assign15110_e21856_d_n10;
        locals.var_q_deps0_soi_o_cnst0soi_dn11 = assign15110_e21856_d_n11;
        locals.var_q_deps0_soi_o_cnst0soi_dn12 = assign15110_e21856_d_n12;
        locals.var_q_deps0_soi_o_cnst0soi_dn17 = assign15110_e21856_d_n17;
        locals.var_q_deps0_soi_o_cnst0soi_rv = 0.0;

        let (assign15120_e21867, assign15120_e21867_d_n0, assign15120_e21867_d_n2, assign15120_e21867_d_n6, assign15120_e21867_d_n7, assign15120_e21867_d_n10, assign15120_e21867_d_n11, assign15120_e21867_d_n12, assign15120_e21867_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15120_e21861: f64 = (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi);
        let assign15120_e21864: f64 = (10.0 * 2.220446049250313e-16);
        let assign15120_e21865: f64 = (assign15120_e21861 + assign15120_e21864);
        (assign15120_e21865, ((locals.var_q_deps0_soi_o_cnst0soi_dn0 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn0)), ((locals.var_q_deps0_soi_o_cnst0soi_dn2 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn2)), ((locals.var_q_deps0_soi_o_cnst0soi_dn6 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn6)), ((locals.var_q_deps0_soi_o_cnst0soi_dn7 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn7)), ((locals.var_q_deps0_soi_o_cnst0soi_dn10 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn10)), ((locals.var_q_deps0_soi_o_cnst0soi_dn11 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn11)), ((locals.var_q_deps0_soi_o_cnst0soi_dn12 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn12)), ((locals.var_q_deps0_soi_o_cnst0soi_dn17 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn17)),)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn6, locals.var_xi0_dn7, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12, locals.var_xi0_dn17,)
    }
};
        locals.var_xi0 = assign15120_e21867;
        locals.var_xi0_dn0 = assign15120_e21867_d_n0;
        locals.var_xi0_dn2 = assign15120_e21867_d_n2;
        locals.var_xi0_dn6 = assign15120_e21867_d_n6;
        locals.var_xi0_dn7 = assign15120_e21867_d_n7;
        locals.var_xi0_dn10 = assign15120_e21867_d_n10;
        locals.var_xi0_dn11 = assign15120_e21867_d_n11;
        locals.var_xi0_dn12 = assign15120_e21867_d_n12;
        locals.var_xi0_dn17 = assign15120_e21867_d_n17;
        locals.var_xi0_rv = 0.0;

        let (assign15130_e21874, assign15130_e21874_d_n0, assign15130_e21874_d_n2, assign15130_e21874_d_n6, assign15130_e21874_d_n7, assign15130_e21874_d_n10, assign15130_e21874_d_n11, assign15130_e21874_d_n12, assign15130_e21874_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15130_e21872: f64 = (2.0 * locals.var_q_deps0_soi_o_cnst0soi);
        (assign15130_e21872, (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn0), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn2), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn6), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn7), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn10), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn11), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn12), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15130_e21874;
        locals.var_t1_dn0 = assign15130_e21874_d_n0;
        locals.var_t1_dn2 = assign15130_e21874_d_n2;
        locals.var_t1_dn6 = assign15130_e21874_d_n6;
        locals.var_t1_dn7 = assign15130_e21874_d_n7;
        locals.var_t1_dn10 = assign15130_e21874_d_n10;
        locals.var_t1_dn11 = assign15130_e21874_d_n11;
        locals.var_t1_dn12 = assign15130_e21874_d_n12;
        locals.var_t1_dn17 = assign15130_e21874_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign15140_e21883, assign15140_e21883_d_n0, assign15140_e21883_d_n2, assign15140_e21883_d_n6, assign15140_e21883_d_n7, assign15140_e21883_d_n10, assign15140_e21883_d_n11, assign15140_e21883_d_n12, assign15140_e21883_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15140_e21880: f64 = (10.0 * 2.220446049250313e-16);
        let assign15140_e21881: f64 = (locals.var_q_deps0_soi_o_cnst0soi + assign15140_e21880);
        (assign15140_e21881, locals.var_q_deps0_soi_o_cnst0soi_dn0, locals.var_q_deps0_soi_o_cnst0soi_dn2, locals.var_q_deps0_soi_o_cnst0soi_dn6, locals.var_q_deps0_soi_o_cnst0soi_dn7, locals.var_q_deps0_soi_o_cnst0soi_dn10, locals.var_q_deps0_soi_o_cnst0soi_dn11, locals.var_q_deps0_soi_o_cnst0soi_dn12, locals.var_q_deps0_soi_o_cnst0soi_dn17,)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn12, locals.var_xi0p12_dn17,)
    }
};
        locals.var_xi0p12 = assign15140_e21883;
        locals.var_xi0p12_dn0 = assign15140_e21883_d_n0;
        locals.var_xi0p12_dn2 = assign15140_e21883_d_n2;
        locals.var_xi0p12_dn6 = assign15140_e21883_d_n6;
        locals.var_xi0p12_dn7 = assign15140_e21883_d_n7;
        locals.var_xi0p12_dn10 = assign15140_e21883_d_n10;
        locals.var_xi0p12_dn11 = assign15140_e21883_d_n11;
        locals.var_xi0p12_dn12 = assign15140_e21883_d_n12;
        locals.var_xi0p12_dn17 = assign15140_e21883_d_n17;
        locals.var_xi0p12_rv = 0.0;

        let (assign15150_e21890, assign15150_e21890_d_n0, assign15150_e21890_d_n2, assign15150_e21890_d_n6, assign15150_e21890_d_n7, assign15150_e21890_d_n10, assign15150_e21890_d_n11, assign15150_e21890_d_n12, assign15150_e21890_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15150_e21888: f64 = (locals.var_cnst0soi * locals.var_xi0p12);
        (assign15150_e21888, ((locals.var_cnst0soi_dn0 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn0)), ((locals.var_cnst0soi_dn2 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn2)), ((locals.var_cnst0soi_dn6 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn6)), ((locals.var_cnst0soi_dn7 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn7)), ((locals.var_cnst0soi_dn10 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn10)), ((locals.var_cnst0soi_dn11 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn11)), ((locals.var_cnst0soi_dn12 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn12)), ((locals.var_cnst0soi_dn17 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn17)),)
    } else {
        (locals.var_qb0, locals.var_qb0_dn0, locals.var_qb0_dn2, locals.var_qb0_dn6, locals.var_qb0_dn7, locals.var_qb0_dn10, locals.var_qb0_dn11, locals.var_qb0_dn12, locals.var_qb0_dn17,)
    }
};
        locals.var_qb0 = assign15150_e21890;
        locals.var_qb0_dn0 = assign15150_e21890_d_n0;
        locals.var_qb0_dn2 = assign15150_e21890_d_n2;
        locals.var_qb0_dn6 = assign15150_e21890_d_n6;
        locals.var_qb0_dn7 = assign15150_e21890_d_n7;
        locals.var_qb0_dn10 = assign15150_e21890_d_n10;
        locals.var_qb0_dn11 = assign15150_e21890_d_n11;
        locals.var_qb0_dn12 = assign15150_e21890_d_n12;
        locals.var_qb0_dn17 = assign15150_e21890_d_n17;
        locals.var_qb0_rv = 0.0;

        let (assign15160_e21899, assign15160_e21899_d_n0, assign15160_e21899_d_n2, assign15160_e21899_d_n6, assign15160_e21899_d_n7, assign15160_e21899_d_n10, assign15160_e21899_d_n11, assign15160_e21899_d_n12, assign15160_e21899_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15160_e21896: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign15160_e21897: f64 = (1.0 / assign15160_e21896);
        (assign15160_e21897, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign15160_e21896 * assign15160_e21896))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign15160_e21896 * assign15160_e21896))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign15160_e21896 * assign15160_e21896))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign15160_e21896 * assign15160_e21896))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign15160_e21896 * assign15160_e21896))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign15160_e21896 * assign15160_e21896))), (-((locals.var_fs02_dn12 + locals.var_xi0p12_dn12) / (assign15160_e21896 * assign15160_e21896))), (-((locals.var_fs02_dn17 + locals.var_xi0p12_dn17) / (assign15160_e21896 * assign15160_e21896))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15160_e21899;
        locals.var_t1_dn0 = assign15160_e21899_d_n0;
        locals.var_t1_dn2 = assign15160_e21899_d_n2;
        locals.var_t1_dn6 = assign15160_e21899_d_n6;
        locals.var_t1_dn7 = assign15160_e21899_d_n7;
        locals.var_t1_dn10 = assign15160_e21899_d_n10;
        locals.var_t1_dn11 = assign15160_e21899_d_n11;
        locals.var_t1_dn12 = assign15160_e21899_d_n12;
        locals.var_t1_dn17 = assign15160_e21899_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign15170_e21908, assign15170_e21908_d_n0, assign15170_e21908_d_n2, assign15170_e21908_d_n6, assign15170_e21908_d_n7, assign15170_e21908_d_n10, assign15170_e21908_d_n11, assign15170_e21908_d_n12, assign15170_e21908_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15170_e21904: f64 = (locals.var_cnst0soi * locals.var_fs01);
        let assign15170_e21906: f64 = (assign15170_e21904 * locals.var_t1);
        (assign15170_e21906, ((((locals.var_cnst0soi_dn0 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn0)) * locals.var_t1) + (assign15170_e21904 * locals.var_t1_dn0)), ((((locals.var_cnst0soi_dn2 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn2)) * locals.var_t1) + (assign15170_e21904 * locals.var_t1_dn2)), ((((locals.var_cnst0soi_dn6 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn6)) * locals.var_t1) + (assign15170_e21904 * locals.var_t1_dn6)), ((((locals.var_cnst0soi_dn7 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn7)) * locals.var_t1) + (assign15170_e21904 * locals.var_t1_dn7)), ((((locals.var_cnst0soi_dn10 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn10)) * locals.var_t1) + (assign15170_e21904 * locals.var_t1_dn10)), ((((locals.var_cnst0soi_dn11 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn11)) * locals.var_t1) + (assign15170_e21904 * locals.var_t1_dn11)), ((((locals.var_cnst0soi_dn12 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn12)) * locals.var_t1) + (assign15170_e21904 * locals.var_t1_dn12)), ((((locals.var_cnst0soi_dn17 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn17)) * locals.var_t1) + (assign15170_e21904 * locals.var_t1_dn17)),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn12, locals.var_qn0_dn17,)
    }
};
        locals.var_qn0 = assign15170_e21908;
        locals.var_qn0_dn0 = assign15170_e21908_d_n0;
        locals.var_qn0_dn2 = assign15170_e21908_d_n2;
        locals.var_qn0_dn6 = assign15170_e21908_d_n6;
        locals.var_qn0_dn7 = assign15170_e21908_d_n7;
        locals.var_qn0_dn10 = assign15170_e21908_d_n10;
        locals.var_qn0_dn11 = assign15170_e21908_d_n11;
        locals.var_qn0_dn12 = assign15170_e21908_d_n12;
        locals.var_qn0_dn17 = assign15170_e21908_d_n17;
        locals.var_qn0_rv = 0.0;

        let (assign15180_e21914, assign15180_e21914_d_n0, assign15180_e21914_d_n2, assign15180_e21914_d_n6, assign15180_e21914_d_n7, assign15180_e21914_d_n10, assign15180_e21914_d_n11, assign15180_e21914_d_n12, assign15180_e21914_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15180_e21912: f64 = (-locals.var_qn0);
        (assign15180_e21912, (-locals.var_qn0_dn0), (-locals.var_qn0_dn2), (-locals.var_qn0_dn6), (-locals.var_qn0_dn7), (-locals.var_qn0_dn10), (-locals.var_qn0_dn11), (-locals.var_qn0_dn12), (-locals.var_qn0_dn17),)
    } else {
        (locals.var_q_n0, locals.var_q_n0_dn0, locals.var_q_n0_dn2, locals.var_q_n0_dn6, locals.var_q_n0_dn7, locals.var_q_n0_dn10, locals.var_q_n0_dn11, locals.var_q_n0_dn12, locals.var_q_n0_dn17,)
    }
};
        locals.var_q_n0 = assign15180_e21914;
        locals.var_q_n0_dn0 = assign15180_e21914_d_n0;
        locals.var_q_n0_dn2 = assign15180_e21914_d_n2;
        locals.var_q_n0_dn6 = assign15180_e21914_d_n6;
        locals.var_q_n0_dn7 = assign15180_e21914_d_n7;
        locals.var_q_n0_dn10 = assign15180_e21914_d_n10;
        locals.var_q_n0_dn11 = assign15180_e21914_d_n11;
        locals.var_q_n0_dn12 = assign15180_e21914_d_n12;
        locals.var_q_n0_dn17 = assign15180_e21914_d_n17;
        locals.var_q_n0_rv = 0.0;

        let (assign15190_e21921, assign15190_e21921_d_n0, assign15190_e21921_d_n2, assign15190_e21921_d_n6, assign15190_e21921_d_n7, assign15190_e21921_d_n10, assign15190_e21921_d_n11, assign15190_e21921_d_n12, assign15190_e21921_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        let assign15190_e21919: f64 = (locals.var_qn0 * locals.var_c_fox_inv);
        (assign15190_e21919, ((locals.var_qn0_dn0 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn0)), ((locals.var_qn0_dn2 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn2)), ((locals.var_qn0_dn6 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn6)), ((locals.var_qn0_dn7 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn7)), ((locals.var_qn0_dn10 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn10)), ((locals.var_qn0_dn11 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn11)), ((locals.var_qn0_dn12 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn12)), ((locals.var_qn0_dn17 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign15190_e21921;
        locals.var_vgvt_dn0 = assign15190_e21921_d_n0;
        locals.var_vgvt_dn2 = assign15190_e21921_d_n2;
        locals.var_vgvt_dn6 = assign15190_e21921_d_n6;
        locals.var_vgvt_dn7 = assign15190_e21921_d_n7;
        locals.var_vgvt_dn10 = assign15190_e21921_d_n10;
        locals.var_vgvt_dn11 = assign15190_e21921_d_n11;
        locals.var_vgvt_dn12 = assign15190_e21921_d_n12;
        locals.var_vgvt_dn17 = assign15190_e21921_d_n17;
        locals.var_vgvt_rv = 0.0;

        let assign15200_e21924: f64 = (-1.0);
        let assign15200_e21929: f64 = if ((locals.var_flg_zone == assign15200_e21924) || (locals.var_vgvt <= 1e-12)) { 1.0 } else { 0.0 };
        locals.var_guard462 = assign15200_e21929;
        locals.var_guard462_rv = 0.0;

        let (assign15210_e21936,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign15210_e21936;
        locals.var_flg_zone_rv = 0.0;

        let (assign15220_e21943,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign15220_e21943;
        locals.var_flg_noqi_rv = 0.0;

        let (assign15230_e21952, assign15230_e21952_d_n0, assign15230_e21952_d_n2, assign15230_e21952_d_n6, assign15230_e21952_d_n7, assign15230_e21952_d_n10, assign15230_e21952_d_n11, assign15230_e21952_d_n12, assign15230_e21952_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        let assign15230_e21950: f64 = (locals.var_vgp - locals.var_ps0);
        (assign15230_e21950, (locals.var_vgp_dn0 - locals.var_ps0_dn0), (locals.var_vgp_dn2 - locals.var_ps0_dn2), (locals.var_vgp_dn6 - locals.var_ps0_dn6), (locals.var_vgp_dn7 - locals.var_ps0_dn7), (locals.var_vgp_dn10 - locals.var_ps0_dn10), (locals.var_vgp_dn11 - locals.var_ps0_dn11), (locals.var_vgp_dn12 - locals.var_ps0_dn12), (locals.var_vgp_dn17 - locals.var_ps0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign15230_e21952;
        locals.var_t2_dn0 = assign15230_e21952_d_n0;
        locals.var_t2_dn2 = assign15230_e21952_d_n2;
        locals.var_t2_dn6 = assign15230_e21952_d_n6;
        locals.var_t2_dn7 = assign15230_e21952_d_n7;
        locals.var_t2_dn10 = assign15230_e21952_d_n10;
        locals.var_t2_dn11 = assign15230_e21952_d_n11;
        locals.var_t2_dn12 = assign15230_e21952_d_n12;
        locals.var_t2_dn17 = assign15230_e21952_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign15240_e21961, assign15240_e21961_d_n0, assign15240_e21961_d_n2, assign15240_e21961_d_n6, assign15240_e21961_d_n7, assign15240_e21961_d_n10, assign15240_e21961_d_n11, assign15240_e21961_d_n12, assign15240_e21961_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        let assign15240_e21959: f64 = (locals.var_c_fox * locals.var_t2);
        (assign15240_e21959, ((locals.var_c_fox_dn0 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn0)), ((locals.var_c_fox_dn2 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn2)), ((locals.var_c_fox_dn6 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn6)), ((locals.var_c_fox_dn7 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn7)), ((locals.var_c_fox_dn10 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn10)), ((locals.var_c_fox_dn11 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn11)), ((locals.var_c_fox_dn12 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn12)), ((locals.var_c_fox_dn17 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign15240_e21961;
        locals.var_qbu_dn0 = assign15240_e21961_d_n0;
        locals.var_qbu_dn2 = assign15240_e21961_d_n2;
        locals.var_qbu_dn6 = assign15240_e21961_d_n6;
        locals.var_qbu_dn7 = assign15240_e21961_d_n7;
        locals.var_qbu_dn10 = assign15240_e21961_d_n10;
        locals.var_qbu_dn11 = assign15240_e21961_d_n11;
        locals.var_qbu_dn12 = assign15240_e21961_d_n12;
        locals.var_qbu_dn17 = assign15240_e21961_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign15250_e21971, assign15250_e21971_d_n0, assign15250_e21971_d_n2, assign15250_e21971_d_n6, assign15250_e21971_d_n7, assign15250_e21971_d_n10, assign15250_e21971_d_n11, assign15250_e21971_d_n12, assign15250_e21971_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        let assign15250_e21967: f64 = (-locals.var_weffcv_nf);
        let assign15250_e21969: f64 = (assign15250_e21967 * locals.var_leff_cv);
        (assign15250_e21969, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign15250_e21971;
        locals.var_t0_dn0 = assign15250_e21971_d_n0;
        locals.var_t0_dn2 = assign15250_e21971_d_n2;
        locals.var_t0_dn6 = assign15250_e21971_d_n6;
        locals.var_t0_dn7 = assign15250_e21971_d_n7;
        locals.var_t0_dn10 = assign15250_e21971_d_n10;
        locals.var_t0_dn11 = assign15250_e21971_d_n11;
        locals.var_t0_dn12 = assign15250_e21971_d_n12;
        locals.var_t0_dn17 = assign15250_e21971_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign15260_e21980, assign15260_e21980_d_n0, assign15260_e21980_d_n2, assign15260_e21980_d_n6, assign15260_e21980_d_n7, assign15260_e21980_d_n10, assign15260_e21980_d_n11, assign15260_e21980_d_n12, assign15260_e21980_d_n13, assign15260_e21980_d_n15, assign15260_e21980_d_n16, assign15260_e21980_d_n17, assign15260_e21980_d_n18,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        let assign15260_e21978: f64 = (locals.var_t0 * locals.var_qbu);
        (assign15260_e21978, ((locals.var_t0_dn0 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn0)), ((locals.var_t0_dn2 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn2)), ((locals.var_t0_dn6 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn6)), ((locals.var_t0_dn7 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn7)), ((locals.var_t0_dn10 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn10)), ((locals.var_t0_dn11 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn11)), ((locals.var_t0_dn12 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn12)), 0.0, 0.0, 0.0, ((locals.var_t0_dn17 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn17)), 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign15260_e21980;
        locals.var_qb_dn0 = assign15260_e21980_d_n0;
        locals.var_qb_dn2 = assign15260_e21980_d_n2;
        locals.var_qb_dn6 = assign15260_e21980_d_n6;
        locals.var_qb_dn7 = assign15260_e21980_d_n7;
        locals.var_qb_dn10 = assign15260_e21980_d_n10;
        locals.var_qb_dn11 = assign15260_e21980_d_n11;
        locals.var_qb_dn12 = assign15260_e21980_d_n12;
        locals.var_qb_dn13 = assign15260_e21980_d_n13;
        locals.var_qb_dn15 = assign15260_e21980_d_n15;
        locals.var_qb_dn16 = assign15260_e21980_d_n16;
        locals.var_qb_dn17 = assign15260_e21980_d_n17;
        locals.var_qb_dn18 = assign15260_e21980_d_n18;
        locals.var_qb_rv = 0.0;

        let (assign15270_e21987, assign15270_e21987_d_n0, assign15270_e21987_d_n2, assign15270_e21987_d_n6, assign15270_e21987_d_n7, assign15270_e21987_d_n10, assign15270_e21987_d_n11, assign15270_e21987_d_n12, assign15270_e21987_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, locals.var_qi_dn17,)
    }
};
        locals.var_qi = assign15270_e21987;
        locals.var_qi_dn0 = assign15270_e21987_d_n0;
        locals.var_qi_dn2 = assign15270_e21987_d_n2;
        locals.var_qi_dn6 = assign15270_e21987_d_n6;
        locals.var_qi_dn7 = assign15270_e21987_d_n7;
        locals.var_qi_dn10 = assign15270_e21987_d_n10;
        locals.var_qi_dn11 = assign15270_e21987_d_n11;
        locals.var_qi_dn12 = assign15270_e21987_d_n12;
        locals.var_qi_dn17 = assign15270_e21987_d_n17;
        locals.var_qi_rv = 0.0;

        let (assign15280_e21994, assign15280_e21994_d_n0, assign15280_e21994_d_n2, assign15280_e21994_d_n6, assign15280_e21994_d_n7, assign15280_e21994_d_n10, assign15280_e21994_d_n11, assign15280_e21994_d_n12, assign15280_e21994_d_n13, assign15280_e21994_d_n15, assign15280_e21994_d_n16, assign15280_e21994_d_n17, assign15280_e21994_d_n18,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign15280_e21994;
        locals.var_qd_dn0 = assign15280_e21994_d_n0;
        locals.var_qd_dn2 = assign15280_e21994_d_n2;
        locals.var_qd_dn6 = assign15280_e21994_d_n6;
        locals.var_qd_dn7 = assign15280_e21994_d_n7;
        locals.var_qd_dn10 = assign15280_e21994_d_n10;
        locals.var_qd_dn11 = assign15280_e21994_d_n11;
        locals.var_qd_dn12 = assign15280_e21994_d_n12;
        locals.var_qd_dn13 = assign15280_e21994_d_n13;
        locals.var_qd_dn15 = assign15280_e21994_d_n15;
        locals.var_qd_dn16 = assign15280_e21994_d_n16;
        locals.var_qd_dn17 = assign15280_e21994_d_n17;
        locals.var_qd_dn18 = assign15280_e21994_d_n18;
        locals.var_qd_rv = 0.0;

        let (assign15290_e22004, assign15290_e22004_d_n0, assign15290_e22004_d_n2, assign15290_e22004_d_n6, assign15290_e22004_d_n7, assign15290_e22004_d_n10, assign15290_e22004_d_n11, assign15290_e22004_d_n12, assign15290_e22004_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        let assign15290_e22000: f64 = (-locals.var_area_bt_n);
        let assign15290_e22002: f64 = (assign15290_e22000 * locals.var_qbu);
        (assign15290_e22002, (assign15290_e22000 * locals.var_qbu_dn0), (assign15290_e22000 * locals.var_qbu_dn2), (assign15290_e22000 * locals.var_qbu_dn6), (assign15290_e22000 * locals.var_qbu_dn7), (assign15290_e22000 * locals.var_qbu_dn10), (assign15290_e22000 * locals.var_qbu_dn11), (assign15290_e22000 * locals.var_qbu_dn12), (assign15290_e22000 * locals.var_qbu_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign15290_e22004;
        locals.var_t2_dn0 = assign15290_e22004_d_n0;
        locals.var_t2_dn2 = assign15290_e22004_d_n2;
        locals.var_t2_dn6 = assign15290_e22004_d_n6;
        locals.var_t2_dn7 = assign15290_e22004_d_n7;
        locals.var_t2_dn10 = assign15290_e22004_d_n10;
        locals.var_t2_dn11 = assign15290_e22004_d_n11;
        locals.var_t2_dn12 = assign15290_e22004_d_n12;
        locals.var_t2_dn17 = assign15290_e22004_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign15300_e22013, assign15300_e22013_d_n0, assign15300_e22013_d_n2, assign15300_e22013_d_n6, assign15300_e22013_d_n7, assign15300_e22013_d_n10, assign15300_e22013_d_n11, assign15300_e22013_d_n12, assign15300_e22013_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        let assign15300_e22011: f64 = (locals.var_t2 * locals.var_qdrat);
        (assign15300_e22011, ((locals.var_t2_dn0 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn0)), ((locals.var_t2_dn2 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn2)), ((locals.var_t2_dn6 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn6)), ((locals.var_t2_dn7 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn7)), ((locals.var_t2_dn10 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn10)), ((locals.var_t2_dn11 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn11)), ((locals.var_t2_dn12 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn12)), ((locals.var_t2_dn17 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn17)),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign15300_e22013;
        locals.var_qbody_bt_n_sud_dn0 = assign15300_e22013_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign15300_e22013_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign15300_e22013_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign15300_e22013_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign15300_e22013_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign15300_e22013_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign15300_e22013_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign15300_e22013_d_n17;
        locals.var_qbody_bt_n_sud_rv = 0.0;

        let (assign15310_e22022, assign15310_e22022_d_n0, assign15310_e22022_d_n2, assign15310_e22022_d_n6, assign15310_e22022_d_n7, assign15310_e22022_d_n10, assign15310_e22022_d_n11, assign15310_e22022_d_n12, assign15310_e22022_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        let assign15310_e22020: f64 = (locals.var_t2 - locals.var_qbody_bt_n_sud);
        (assign15310_e22020, (locals.var_t2_dn0 - locals.var_qbody_bt_n_sud_dn0), (locals.var_t2_dn2 - locals.var_qbody_bt_n_sud_dn2), (locals.var_t2_dn6 - locals.var_qbody_bt_n_sud_dn6), (locals.var_t2_dn7 - locals.var_qbody_bt_n_sud_dn7), (locals.var_t2_dn10 - locals.var_qbody_bt_n_sud_dn10), (locals.var_t2_dn11 - locals.var_qbody_bt_n_sud_dn11), (locals.var_t2_dn12 - locals.var_qbody_bt_n_sud_dn12), (locals.var_t2_dn17 - locals.var_qbody_bt_n_sud_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign15310_e22022;
        locals.var_qbody_bt_n_sus_dn0 = assign15310_e22022_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign15310_e22022_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign15310_e22022_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign15310_e22022_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign15310_e22022_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign15310_e22022_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign15310_e22022_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign15310_e22022_d_n17;
        locals.var_qbody_bt_n_sus_rv = 0.0;

        let (assign15320_e22029, assign15320_e22029_d_n0, assign15320_e22029_d_n2, assign15320_e22029_d_n6, assign15320_e22029_d_n7, assign15320_e22029_d_n10, assign15320_e22029_d_n11, assign15320_e22029_d_n12, assign15320_e22029_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign15320_e22029;
        locals.var_qbody_bt_n_iud_dn0 = assign15320_e22029_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign15320_e22029_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign15320_e22029_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign15320_e22029_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign15320_e22029_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign15320_e22029_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign15320_e22029_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign15320_e22029_d_n17;
        locals.var_qbody_bt_n_iud_rv = 0.0;

        let (assign15330_e22036, assign15330_e22036_d_n0, assign15330_e22036_d_n2, assign15330_e22036_d_n6, assign15330_e22036_d_n7, assign15330_e22036_d_n10, assign15330_e22036_d_n11, assign15330_e22036_d_n12, assign15330_e22036_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign15330_e22036;
        locals.var_qbody_bt_n_ius_dn0 = assign15330_e22036_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign15330_e22036_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign15330_e22036_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign15330_e22036_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign15330_e22036_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign15330_e22036_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign15330_e22036_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign15330_e22036_d_n17;
        locals.var_qbody_bt_n_ius_rv = 0.0;

        let (assign15340_e22043, assign15340_e22043_d_n0, assign15340_e22043_d_n2, assign15340_e22043_d_n6, assign15340_e22043_d_n7, assign15340_e22043_d_n10, assign15340_e22043_d_n11, assign15340_e22043_d_n12, assign15340_e22043_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign15340_e22043;
        locals.var_ids_dn0 = assign15340_e22043_d_n0;
        locals.var_ids_dn2 = assign15340_e22043_d_n2;
        locals.var_ids_dn6 = assign15340_e22043_d_n6;
        locals.var_ids_dn7 = assign15340_e22043_d_n7;
        locals.var_ids_dn10 = assign15340_e22043_d_n10;
        locals.var_ids_dn11 = assign15340_e22043_d_n11;
        locals.var_ids_dn12 = assign15340_e22043_d_n12;
        locals.var_ids_dn17 = assign15340_e22043_d_n17;
        locals.var_ids_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_54(
        locals: &mut StampLocals,
    ) {
        let (assign15350_e22050, assign15350_e22050_d_n0, assign15350_e22050_d_n2, assign15350_e22050_d_n6, assign15350_e22050_d_n7, assign15350_e22050_d_n10, assign15350_e22050_d_n11, assign15350_e22050_d_n12, assign15350_e22050_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign15350_e22050;
        locals.var_vgvt_dn0 = assign15350_e22050_d_n0;
        locals.var_vgvt_dn2 = assign15350_e22050_d_n2;
        locals.var_vgvt_dn6 = assign15350_e22050_d_n6;
        locals.var_vgvt_dn7 = assign15350_e22050_d_n7;
        locals.var_vgvt_dn10 = assign15350_e22050_d_n10;
        locals.var_vgvt_dn11 = assign15350_e22050_d_n11;
        locals.var_vgvt_dn12 = assign15350_e22050_d_n12;
        locals.var_vgvt_dn17 = assign15350_e22050_d_n17;
        locals.var_vgvt_rv = 0.0;

        let (assign15360_e22057,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign15360_e22057;
        locals.var_flg_noqi_rv = 0.0;

        let (assign15370_e22064, assign15370_e22064_d_n0, assign15370_e22064_d_n2, assign15370_e22064_d_n6, assign15370_e22064_d_n7, assign15370_e22064_d_n10, assign15370_e22064_d_n11, assign15370_e22064_d_n12, assign15370_e22064_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign15370_e22064;
        locals.var_phi_sl_soi_dn0 = assign15370_e22064_d_n0;
        locals.var_phi_sl_soi_dn2 = assign15370_e22064_d_n2;
        locals.var_phi_sl_soi_dn6 = assign15370_e22064_d_n6;
        locals.var_phi_sl_soi_dn7 = assign15370_e22064_d_n7;
        locals.var_phi_sl_soi_dn10 = assign15370_e22064_d_n10;
        locals.var_phi_sl_soi_dn11 = assign15370_e22064_d_n11;
        locals.var_phi_sl_soi_dn12 = assign15370_e22064_d_n12;
        locals.var_phi_sl_soi_dn17 = assign15370_e22064_d_n17;
        locals.var_phi_sl_soi_rv = 0.0;

        let (assign15380_e22071, assign15380_e22071_d_n0, assign15380_e22071_d_n2, assign15380_e22071_d_n6, assign15380_e22071_d_n7, assign15380_e22071_d_n10, assign15380_e22071_d_n11, assign15380_e22071_d_n12, assign15380_e22071_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign15380_e22071;
        locals.var_psl_dn0 = assign15380_e22071_d_n0;
        locals.var_psl_dn2 = assign15380_e22071_d_n2;
        locals.var_psl_dn6 = assign15380_e22071_d_n6;
        locals.var_psl_dn7 = assign15380_e22071_d_n7;
        locals.var_psl_dn10 = assign15380_e22071_d_n10;
        locals.var_psl_dn11 = assign15380_e22071_d_n11;
        locals.var_psl_dn12 = assign15380_e22071_d_n12;
        locals.var_psl_dn17 = assign15380_e22071_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign15390_e22078, assign15390_e22078_d_n0, assign15390_e22078_d_n2, assign15390_e22078_d_n6, assign15390_e22078_d_n7, assign15390_e22078_d_n10, assign15390_e22078_d_n11, assign15390_e22078_d_n12, assign15390_e22078_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign15390_e22078;
        locals.var_psdl_dn0 = assign15390_e22078_d_n0;
        locals.var_psdl_dn2 = assign15390_e22078_d_n2;
        locals.var_psdl_dn6 = assign15390_e22078_d_n6;
        locals.var_psdl_dn7 = assign15390_e22078_d_n7;
        locals.var_psdl_dn10 = assign15390_e22078_d_n10;
        locals.var_psdl_dn11 = assign15390_e22078_d_n11;
        locals.var_psdl_dn12 = assign15390_e22078_d_n12;
        locals.var_psdl_dn17 = assign15390_e22078_d_n17;
        locals.var_psdl_rv = 0.0;

        let (assign15400_e22085,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard462 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_end_of_part_1,)
    }
};
        locals.var_end_of_part_1 = assign15400_e22085;
        locals.var_end_of_part_1_rv = 0.0;

        let assign15410_e22088: f64 = if locals.var_end_of_part_1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard463 = assign15410_e22088;
        locals.var_guard463_rv = 0.0;

        let (assign15420_e22095, assign15420_e22095_d_n0, assign15420_e22095_d_n2, assign15420_e22095_d_n6, assign15420_e22095_d_n7, assign15420_e22095_d_n10, assign15420_e22095_d_n11, assign15420_e22095_d_n12, assign15420_e22095_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    }
};
        locals.var_vdsorg = assign15420_e22095;
        locals.var_vdsorg_dn0 = assign15420_e22095_d_n0;
        locals.var_vdsorg_dn2 = assign15420_e22095_d_n2;
        locals.var_vdsorg_dn6 = assign15420_e22095_d_n6;
        locals.var_vdsorg_dn7 = assign15420_e22095_d_n7;
        locals.var_vdsorg_dn10 = assign15420_e22095_d_n10;
        locals.var_vdsorg_dn11 = assign15420_e22095_d_n11;
        locals.var_vdsorg_dn12 = assign15420_e22095_d_n12;
        locals.var_vdsorg_dn17 = assign15420_e22095_d_n17;
        locals.var_vdsorg_rv = 0.0;

        let (assign15430_e22102, assign15430_e22102_d_n0, assign15430_e22102_d_n2, assign15430_e22102_d_n6, assign15430_e22102_d_n7, assign15430_e22102_d_n10, assign15430_e22102_d_n11, assign15430_e22102_d_n12, assign15430_e22102_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10__blk470, locals.var_t10__blk470_dn0, locals.var_t10__blk470_dn2, locals.var_t10__blk470_dn6, locals.var_t10__blk470_dn7, locals.var_t10__blk470_dn10, locals.var_t10__blk470_dn11, locals.var_t10__blk470_dn12, locals.var_t10__blk470_dn17,)
    }
};
        locals.var_t10__blk470 = assign15430_e22102;
        locals.var_t10__blk470_dn0 = assign15430_e22102_d_n0;
        locals.var_t10__blk470_dn2 = assign15430_e22102_d_n2;
        locals.var_t10__blk470_dn6 = assign15430_e22102_d_n6;
        locals.var_t10__blk470_dn7 = assign15430_e22102_d_n7;
        locals.var_t10__blk470_dn10 = assign15430_e22102_d_n10;
        locals.var_t10__blk470_dn11 = assign15430_e22102_d_n11;
        locals.var_t10__blk470_dn12 = assign15430_e22102_d_n12;
        locals.var_t10__blk470_dn17 = assign15430_e22102_d_n17;
        locals.var_t10__blk470_rv = 0.0;

        let (assign15440_e22113, assign15440_e22113_d_n0, assign15440_e22113_d_n2, assign15440_e22113_d_n6, assign15440_e22113_d_n7, assign15440_e22113_d_n10, assign15440_e22113_d_n11, assign15440_e22113_d_n12, assign15440_e22113_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15440_e22110: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign15440_e22111: f64 = (locals.var_qnsub_esi / assign15440_e22110);
        (assign15440_e22111, (((locals.var_qnsub_esi_dn0 * assign15440_e22110) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign15440_e22110 * assign15440_e22110)), (((locals.var_qnsub_esi_dn2 * assign15440_e22110) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign15440_e22110 * assign15440_e22110)), (((locals.var_qnsub_esi_dn6 * assign15440_e22110) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign15440_e22110 * assign15440_e22110)), (((locals.var_qnsub_esi_dn7 * assign15440_e22110) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign15440_e22110 * assign15440_e22110)), (((locals.var_qnsub_esi_dn10 * assign15440_e22110) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign15440_e22110 * assign15440_e22110)), (((locals.var_qnsub_esi_dn11 * assign15440_e22110) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign15440_e22110 * assign15440_e22110)), (((locals.var_qnsub_esi_dn12 * assign15440_e22110) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign15440_e22110 * assign15440_e22110)), (((locals.var_qnsub_esi_dn17 * assign15440_e22110) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign15440_e22110 * assign15440_e22110)),)
    } else {
        (locals.var_t2__blk465, locals.var_t2__blk465_dn0, locals.var_t2__blk465_dn2, locals.var_t2__blk465_dn6, locals.var_t2__blk465_dn7, locals.var_t2__blk465_dn10, locals.var_t2__blk465_dn11, locals.var_t2__blk465_dn12, locals.var_t2__blk465_dn17,)
    }
};
        locals.var_t2__blk465 = assign15440_e22113;
        locals.var_t2__blk465_dn0 = assign15440_e22113_d_n0;
        locals.var_t2__blk465_dn2 = assign15440_e22113_d_n2;
        locals.var_t2__blk465_dn6 = assign15440_e22113_d_n6;
        locals.var_t2__blk465_dn7 = assign15440_e22113_d_n7;
        locals.var_t2__blk465_dn10 = assign15440_e22113_d_n10;
        locals.var_t2__blk465_dn11 = assign15440_e22113_d_n11;
        locals.var_t2__blk465_dn12 = assign15440_e22113_d_n12;
        locals.var_t2__blk465_dn17 = assign15440_e22113_d_n17;
        locals.var_t2__blk465_rv = 0.0;

        let (assign15450_e22128, assign15450_e22128_d_n0, assign15450_e22128_d_n2, assign15450_e22128_d_n6, assign15450_e22128_d_n7, assign15450_e22128_d_n10, assign15450_e22128_d_n11, assign15450_e22128_d_n12, assign15450_e22128_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15450_e22121: f64 = (2.0 / locals.var_t2__blk465);
        let assign15450_e22124: f64 = (locals.var_vgp - locals.var_t10__blk470);
        let assign15450_e22125: f64 = (assign15450_e22121 * assign15450_e22124);
        let assign15450_e22126: f64 = (1.0 + assign15450_e22125);
        (assign15450_e22126, (((-((2.0 * locals.var_t2__blk465_dn0) / (locals.var_t2__blk465 * locals.var_t2__blk465))) * assign15450_e22124) + (assign15450_e22121 * (locals.var_vgp_dn0 - locals.var_t10__blk470_dn0))), (((-((2.0 * locals.var_t2__blk465_dn2) / (locals.var_t2__blk465 * locals.var_t2__blk465))) * assign15450_e22124) + (assign15450_e22121 * (locals.var_vgp_dn2 - locals.var_t10__blk470_dn2))), (((-((2.0 * locals.var_t2__blk465_dn6) / (locals.var_t2__blk465 * locals.var_t2__blk465))) * assign15450_e22124) + (assign15450_e22121 * (locals.var_vgp_dn6 - locals.var_t10__blk470_dn6))), (((-((2.0 * locals.var_t2__blk465_dn7) / (locals.var_t2__blk465 * locals.var_t2__blk465))) * assign15450_e22124) + (assign15450_e22121 * (locals.var_vgp_dn7 - locals.var_t10__blk470_dn7))), (((-((2.0 * locals.var_t2__blk465_dn10) / (locals.var_t2__blk465 * locals.var_t2__blk465))) * assign15450_e22124) + (assign15450_e22121 * (locals.var_vgp_dn10 - locals.var_t10__blk470_dn10))), (((-((2.0 * locals.var_t2__blk465_dn11) / (locals.var_t2__blk465 * locals.var_t2__blk465))) * assign15450_e22124) + (assign15450_e22121 * (locals.var_vgp_dn11 - locals.var_t10__blk470_dn11))), (((-((2.0 * locals.var_t2__blk465_dn12) / (locals.var_t2__blk465 * locals.var_t2__blk465))) * assign15450_e22124) + (assign15450_e22121 * (locals.var_vgp_dn12 - locals.var_t10__blk470_dn12))), (((-((2.0 * locals.var_t2__blk465_dn17) / (locals.var_t2__blk465 * locals.var_t2__blk465))) * assign15450_e22124) + (assign15450_e22121 * (locals.var_vgp_dn17 - locals.var_t10__blk470_dn17))),)
    } else {
        (locals.var_t4__blk467, locals.var_t4__blk467_dn0, locals.var_t4__blk467_dn2, locals.var_t4__blk467_dn6, locals.var_t4__blk467_dn7, locals.var_t4__blk467_dn10, locals.var_t4__blk467_dn11, locals.var_t4__blk467_dn12, locals.var_t4__blk467_dn17,)
    }
};
        locals.var_t4__blk467 = assign15450_e22128;
        locals.var_t4__blk467_dn0 = assign15450_e22128_d_n0;
        locals.var_t4__blk467_dn2 = assign15450_e22128_d_n2;
        locals.var_t4__blk467_dn6 = assign15450_e22128_d_n6;
        locals.var_t4__blk467_dn7 = assign15450_e22128_d_n7;
        locals.var_t4__blk467_dn10 = assign15450_e22128_d_n10;
        locals.var_t4__blk467_dn11 = assign15450_e22128_d_n11;
        locals.var_t4__blk467_dn12 = assign15450_e22128_d_n12;
        locals.var_t4__blk467_dn17 = assign15450_e22128_d_n17;
        locals.var_t4__blk467_rv = 0.0;

        let (assign15460_e22139, assign15460_e22139_d_n0, assign15460_e22139_d_n2, assign15460_e22139_d_n6, assign15460_e22139_d_n7, assign15460_e22139_d_n10, assign15460_e22139_d_n11, assign15460_e22139_d_n12, assign15460_e22139_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15460_e22136: f64 = (2.0 / locals.var_t2__blk465);
        let assign15460_e22137: f64 = (1.0 + assign15460_e22136);
        (assign15460_e22137, (-((2.0 * locals.var_t2__blk465_dn0) / (locals.var_t2__blk465 * locals.var_t2__blk465))), (-((2.0 * locals.var_t2__blk465_dn2) / (locals.var_t2__blk465 * locals.var_t2__blk465))), (-((2.0 * locals.var_t2__blk465_dn6) / (locals.var_t2__blk465 * locals.var_t2__blk465))), (-((2.0 * locals.var_t2__blk465_dn7) / (locals.var_t2__blk465 * locals.var_t2__blk465))), (-((2.0 * locals.var_t2__blk465_dn10) / (locals.var_t2__blk465 * locals.var_t2__blk465))), (-((2.0 * locals.var_t2__blk465_dn11) / (locals.var_t2__blk465 * locals.var_t2__blk465))), (-((2.0 * locals.var_t2__blk465_dn12) / (locals.var_t2__blk465 * locals.var_t2__blk465))), (-((2.0 * locals.var_t2__blk465_dn17) / (locals.var_t2__blk465 * locals.var_t2__blk465))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign15460_e22139;
        locals.var_t5_dn0 = assign15460_e22139_d_n0;
        locals.var_t5_dn2 = assign15460_e22139_d_n2;
        locals.var_t5_dn6 = assign15460_e22139_d_n6;
        locals.var_t5_dn7 = assign15460_e22139_d_n7;
        locals.var_t5_dn10 = assign15460_e22139_d_n10;
        locals.var_t5_dn11 = assign15460_e22139_d_n11;
        locals.var_t5_dn12 = assign15460_e22139_d_n12;
        locals.var_t5_dn17 = assign15460_e22139_d_n17;
        locals.var_t5_rv = 0.0;

        let assign15470_e22143: f64 = locals.var_t5;
        let assign15470_e22148: f64 = if ((locals.var_t4__blk467 < assign15470_e22143) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard471 = assign15470_e22148;
        locals.var_guard471_rv = 0.0;

        let (assign15480_e22161, assign15480_e22161_d_n0, assign15480_e22161_d_n2, assign15480_e22161_d_n6, assign15480_e22161_d_n7, assign15480_e22161_d_n10, assign15480_e22161_d_n11, assign15480_e22161_d_n12, assign15480_e22161_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15480_e22157: f64 = locals.var_t5;
        let assign15480_e22159: f64 = (assign15480_e22157 - locals.var_t4__blk467);
        (assign15480_e22159, (locals.var_t5_dn0 - locals.var_t4__blk467_dn0), (locals.var_t5_dn2 - locals.var_t4__blk467_dn2), (locals.var_t5_dn6 - locals.var_t4__blk467_dn6), (locals.var_t5_dn7 - locals.var_t4__blk467_dn7), (locals.var_t5_dn10 - locals.var_t4__blk467_dn10), (locals.var_t5_dn11 - locals.var_t4__blk467_dn11), (locals.var_t5_dn12 - locals.var_t4__blk467_dn12), (locals.var_t5_dn17 - locals.var_t4__blk467_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign15480_e22161;
        locals.var_tmf1_dn0 = assign15480_e22161_d_n0;
        locals.var_tmf1_dn2 = assign15480_e22161_d_n2;
        locals.var_tmf1_dn6 = assign15480_e22161_d_n6;
        locals.var_tmf1_dn7 = assign15480_e22161_d_n7;
        locals.var_tmf1_dn10 = assign15480_e22161_d_n10;
        locals.var_tmf1_dn11 = assign15480_e22161_d_n11;
        locals.var_tmf1_dn12 = assign15480_e22161_d_n12;
        locals.var_tmf1_dn17 = assign15480_e22161_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign15490_e22172, assign15490_e22172_d_n0, assign15490_e22172_d_n2, assign15490_e22172_d_n6, assign15490_e22172_d_n7, assign15490_e22172_d_n10, assign15490_e22172_d_n11, assign15490_e22172_d_n12, assign15490_e22172_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15490_e22170: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign15490_e22170, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign15490_e22172;
        locals.var_x2_dn0 = assign15490_e22172_d_n0;
        locals.var_x2_dn2 = assign15490_e22172_d_n2;
        locals.var_x2_dn6 = assign15490_e22172_d_n6;
        locals.var_x2_dn7 = assign15490_e22172_d_n7;
        locals.var_x2_dn10 = assign15490_e22172_d_n10;
        locals.var_x2_dn11 = assign15490_e22172_d_n11;
        locals.var_x2_dn12 = assign15490_e22172_d_n12;
        locals.var_x2_dn17 = assign15490_e22172_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign15500_e22183, assign15500_e22183_d_n0, assign15500_e22183_d_n2, assign15500_e22183_d_n6, assign15500_e22183_d_n7, assign15500_e22183_d_n10, assign15500_e22183_d_n11, assign15500_e22183_d_n12, assign15500_e22183_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15500_e22181: f64 = (locals.var_t5 * locals.var_t5);
        (assign15500_e22181, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn12 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn12)), ((locals.var_t5_dn17 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign15500_e22183;
        locals.var_xmax2_dn0 = assign15500_e22183_d_n0;
        locals.var_xmax2_dn2 = assign15500_e22183_d_n2;
        locals.var_xmax2_dn6 = assign15500_e22183_d_n6;
        locals.var_xmax2_dn7 = assign15500_e22183_d_n7;
        locals.var_xmax2_dn10 = assign15500_e22183_d_n10;
        locals.var_xmax2_dn11 = assign15500_e22183_d_n11;
        locals.var_xmax2_dn12 = assign15500_e22183_d_n12;
        locals.var_xmax2_dn17 = assign15500_e22183_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign15510_e22192, assign15510_e22192_d_n0, assign15510_e22192_d_n2, assign15510_e22192_d_n6, assign15510_e22192_d_n7, assign15510_e22192_d_n10, assign15510_e22192_d_n11, assign15510_e22192_d_n12, assign15510_e22192_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15510_e22192;
        locals.var_xp_dn0 = assign15510_e22192_d_n0;
        locals.var_xp_dn2 = assign15510_e22192_d_n2;
        locals.var_xp_dn6 = assign15510_e22192_d_n6;
        locals.var_xp_dn7 = assign15510_e22192_d_n7;
        locals.var_xp_dn10 = assign15510_e22192_d_n10;
        locals.var_xp_dn11 = assign15510_e22192_d_n11;
        locals.var_xp_dn12 = assign15510_e22192_d_n12;
        locals.var_xp_dn17 = assign15510_e22192_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15520_e22201, assign15520_e22201_d_n0, assign15520_e22201_d_n2, assign15520_e22201_d_n6, assign15520_e22201_d_n7, assign15520_e22201_d_n10, assign15520_e22201_d_n11, assign15520_e22201_d_n12, assign15520_e22201_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15520_e22201;
        locals.var_xmp_dn0 = assign15520_e22201_d_n0;
        locals.var_xmp_dn2 = assign15520_e22201_d_n2;
        locals.var_xmp_dn6 = assign15520_e22201_d_n6;
        locals.var_xmp_dn7 = assign15520_e22201_d_n7;
        locals.var_xmp_dn10 = assign15520_e22201_d_n10;
        locals.var_xmp_dn11 = assign15520_e22201_d_n11;
        locals.var_xmp_dn12 = assign15520_e22201_d_n12;
        locals.var_xmp_dn17 = assign15520_e22201_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15530_e22210,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign15530_e22210;
        locals.var_m0_rv = 0.0;

        let (assign15540_e22219,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15540_e22219;
        locals.var_mm_rv = 0.0;

        let (assign15550_e22228, assign15550_e22228_d_n0, assign15550_e22228_d_n2, assign15550_e22228_d_n6, assign15550_e22228_d_n7, assign15550_e22228_d_n10, assign15550_e22228_d_n11, assign15550_e22228_d_n12, assign15550_e22228_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign15550_e22228;
        locals.var_arg_dn0 = assign15550_e22228_d_n0;
        locals.var_arg_dn2 = assign15550_e22228_d_n2;
        locals.var_arg_dn6 = assign15550_e22228_d_n6;
        locals.var_arg_dn7 = assign15550_e22228_d_n7;
        locals.var_arg_dn10 = assign15550_e22228_d_n10;
        locals.var_arg_dn11 = assign15550_e22228_d_n11;
        locals.var_arg_dn12 = assign15550_e22228_d_n12;
        locals.var_arg_dn17 = assign15550_e22228_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign15560_e22237, assign15560_e22237_d_n0, assign15560_e22237_d_n2, assign15560_e22237_d_n6, assign15560_e22237_d_n7, assign15560_e22237_d_n10, assign15560_e22237_d_n11, assign15560_e22237_d_n12, assign15560_e22237_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15560_e22237;
        locals.var_dnm_dn0 = assign15560_e22237_d_n0;
        locals.var_dnm_dn2 = assign15560_e22237_d_n2;
        locals.var_dnm_dn6 = assign15560_e22237_d_n6;
        locals.var_dnm_dn7 = assign15560_e22237_d_n7;
        locals.var_dnm_dn10 = assign15560_e22237_d_n10;
        locals.var_dnm_dn11 = assign15560_e22237_d_n11;
        locals.var_dnm_dn12 = assign15560_e22237_d_n12;
        locals.var_dnm_dn17 = assign15560_e22237_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign15570_e22248, assign15570_e22248_d_n0, assign15570_e22248_d_n2, assign15570_e22248_d_n6, assign15570_e22248_d_n7, assign15570_e22248_d_n10, assign15570_e22248_d_n11, assign15570_e22248_d_n12, assign15570_e22248_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15570_e22246: f64 = (locals.var_xp * locals.var_x2);
        (assign15570_e22246, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15570_e22248;
        locals.var_xp_dn0 = assign15570_e22248_d_n0;
        locals.var_xp_dn2 = assign15570_e22248_d_n2;
        locals.var_xp_dn6 = assign15570_e22248_d_n6;
        locals.var_xp_dn7 = assign15570_e22248_d_n7;
        locals.var_xp_dn10 = assign15570_e22248_d_n10;
        locals.var_xp_dn11 = assign15570_e22248_d_n11;
        locals.var_xp_dn12 = assign15570_e22248_d_n12;
        locals.var_xp_dn17 = assign15570_e22248_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15580_e22259, assign15580_e22259_d_n0, assign15580_e22259_d_n2, assign15580_e22259_d_n6, assign15580_e22259_d_n7, assign15580_e22259_d_n10, assign15580_e22259_d_n11, assign15580_e22259_d_n12, assign15580_e22259_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15580_e22257: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15580_e22257, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15580_e22259;
        locals.var_xmp_dn0 = assign15580_e22259_d_n0;
        locals.var_xmp_dn2 = assign15580_e22259_d_n2;
        locals.var_xmp_dn6 = assign15580_e22259_d_n6;
        locals.var_xmp_dn7 = assign15580_e22259_d_n7;
        locals.var_xmp_dn10 = assign15580_e22259_d_n10;
        locals.var_xmp_dn11 = assign15580_e22259_d_n11;
        locals.var_xmp_dn12 = assign15580_e22259_d_n12;
        locals.var_xmp_dn17 = assign15580_e22259_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15590_e22270, assign15590_e22270_d_n0, assign15590_e22270_d_n2, assign15590_e22270_d_n6, assign15590_e22270_d_n7, assign15590_e22270_d_n10, assign15590_e22270_d_n11, assign15590_e22270_d_n12, assign15590_e22270_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15590_e22268: f64 = (locals.var_xp * locals.var_x2);
        (assign15590_e22268, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15590_e22270;
        locals.var_xp_dn0 = assign15590_e22270_d_n0;
        locals.var_xp_dn2 = assign15590_e22270_d_n2;
        locals.var_xp_dn6 = assign15590_e22270_d_n6;
        locals.var_xp_dn7 = assign15590_e22270_d_n7;
        locals.var_xp_dn10 = assign15590_e22270_d_n10;
        locals.var_xp_dn11 = assign15590_e22270_d_n11;
        locals.var_xp_dn12 = assign15590_e22270_d_n12;
        locals.var_xp_dn17 = assign15590_e22270_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15600_e22281, assign15600_e22281_d_n0, assign15600_e22281_d_n2, assign15600_e22281_d_n6, assign15600_e22281_d_n7, assign15600_e22281_d_n10, assign15600_e22281_d_n11, assign15600_e22281_d_n12, assign15600_e22281_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15600_e22279: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15600_e22279, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15600_e22281;
        locals.var_xmp_dn0 = assign15600_e22281_d_n0;
        locals.var_xmp_dn2 = assign15600_e22281_d_n2;
        locals.var_xmp_dn6 = assign15600_e22281_d_n6;
        locals.var_xmp_dn7 = assign15600_e22281_d_n7;
        locals.var_xmp_dn10 = assign15600_e22281_d_n10;
        locals.var_xmp_dn11 = assign15600_e22281_d_n11;
        locals.var_xmp_dn12 = assign15600_e22281_d_n12;
        locals.var_xmp_dn17 = assign15600_e22281_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15610_e22292, assign15610_e22292_d_n0, assign15610_e22292_d_n2, assign15610_e22292_d_n6, assign15610_e22292_d_n7, assign15610_e22292_d_n10, assign15610_e22292_d_n11, assign15610_e22292_d_n12, assign15610_e22292_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15610_e22290: f64 = (locals.var_xp * locals.var_x2);
        (assign15610_e22290, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15610_e22292;
        locals.var_xp_dn0 = assign15610_e22292_d_n0;
        locals.var_xp_dn2 = assign15610_e22292_d_n2;
        locals.var_xp_dn6 = assign15610_e22292_d_n6;
        locals.var_xp_dn7 = assign15610_e22292_d_n7;
        locals.var_xp_dn10 = assign15610_e22292_d_n10;
        locals.var_xp_dn11 = assign15610_e22292_d_n11;
        locals.var_xp_dn12 = assign15610_e22292_d_n12;
        locals.var_xp_dn17 = assign15610_e22292_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15620_e22303, assign15620_e22303_d_n0, assign15620_e22303_d_n2, assign15620_e22303_d_n6, assign15620_e22303_d_n7, assign15620_e22303_d_n10, assign15620_e22303_d_n11, assign15620_e22303_d_n12, assign15620_e22303_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15620_e22301: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15620_e22301, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15620_e22303;
        locals.var_xmp_dn0 = assign15620_e22303_d_n0;
        locals.var_xmp_dn2 = assign15620_e22303_d_n2;
        locals.var_xmp_dn6 = assign15620_e22303_d_n6;
        locals.var_xmp_dn7 = assign15620_e22303_d_n7;
        locals.var_xmp_dn10 = assign15620_e22303_d_n10;
        locals.var_xmp_dn11 = assign15620_e22303_d_n11;
        locals.var_xmp_dn12 = assign15620_e22303_d_n12;
        locals.var_xmp_dn17 = assign15620_e22303_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15630_e22314, assign15630_e22314_d_n0, assign15630_e22314_d_n2, assign15630_e22314_d_n6, assign15630_e22314_d_n7, assign15630_e22314_d_n10, assign15630_e22314_d_n11, assign15630_e22314_d_n12, assign15630_e22314_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15630_e22312: f64 = (locals.var_xp * locals.var_x2);
        (assign15630_e22312, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15630_e22314;
        locals.var_xp_dn0 = assign15630_e22314_d_n0;
        locals.var_xp_dn2 = assign15630_e22314_d_n2;
        locals.var_xp_dn6 = assign15630_e22314_d_n6;
        locals.var_xp_dn7 = assign15630_e22314_d_n7;
        locals.var_xp_dn10 = assign15630_e22314_d_n10;
        locals.var_xp_dn11 = assign15630_e22314_d_n11;
        locals.var_xp_dn12 = assign15630_e22314_d_n12;
        locals.var_xp_dn17 = assign15630_e22314_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15640_e22325, assign15640_e22325_d_n0, assign15640_e22325_d_n2, assign15640_e22325_d_n6, assign15640_e22325_d_n7, assign15640_e22325_d_n10, assign15640_e22325_d_n11, assign15640_e22325_d_n12, assign15640_e22325_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15640_e22323: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15640_e22323, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15640_e22325;
        locals.var_xmp_dn0 = assign15640_e22325_d_n0;
        locals.var_xmp_dn2 = assign15640_e22325_d_n2;
        locals.var_xmp_dn6 = assign15640_e22325_d_n6;
        locals.var_xmp_dn7 = assign15640_e22325_d_n7;
        locals.var_xmp_dn10 = assign15640_e22325_d_n10;
        locals.var_xmp_dn11 = assign15640_e22325_d_n11;
        locals.var_xmp_dn12 = assign15640_e22325_d_n12;
        locals.var_xmp_dn17 = assign15640_e22325_d_n17;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_55(
        locals: &mut StampLocals,
    ) {
        let (assign15650_e22336, assign15650_e22336_d_n0, assign15650_e22336_d_n2, assign15650_e22336_d_n6, assign15650_e22336_d_n7, assign15650_e22336_d_n10, assign15650_e22336_d_n11, assign15650_e22336_d_n12, assign15650_e22336_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15650_e22334: f64 = (locals.var_xp + locals.var_xmp);
        (assign15650_e22334, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign15650_e22336;
        locals.var_arg_dn0 = assign15650_e22336_d_n0;
        locals.var_arg_dn2 = assign15650_e22336_d_n2;
        locals.var_arg_dn6 = assign15650_e22336_d_n6;
        locals.var_arg_dn7 = assign15650_e22336_d_n7;
        locals.var_arg_dn10 = assign15650_e22336_d_n10;
        locals.var_arg_dn11 = assign15650_e22336_d_n11;
        locals.var_arg_dn12 = assign15650_e22336_d_n12;
        locals.var_arg_dn17 = assign15650_e22336_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign15660_e22345, assign15660_e22345_d_n0, assign15660_e22345_d_n2, assign15660_e22345_d_n6, assign15660_e22345_d_n7, assign15660_e22345_d_n10, assign15660_e22345_d_n11, assign15660_e22345_d_n12, assign15660_e22345_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15660_e22345;
        locals.var_dnm_dn0 = assign15660_e22345_d_n0;
        locals.var_dnm_dn2 = assign15660_e22345_d_n2;
        locals.var_dnm_dn6 = assign15660_e22345_d_n6;
        locals.var_dnm_dn7 = assign15660_e22345_d_n7;
        locals.var_dnm_dn10 = assign15660_e22345_d_n10;
        locals.var_dnm_dn11 = assign15660_e22345_d_n11;
        locals.var_dnm_dn12 = assign15660_e22345_d_n12;
        locals.var_dnm_dn17 = assign15660_e22345_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign15670_e22360: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard472 = assign15670_e22360;
        locals.var_guard472_rv = 0.0;

        let assign15680_e22363: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign15680_e22363;
        locals.var_guard473_rv = 0.0;

        let (assign15690_e22376,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15690_e22376;
        locals.var_mm_rv = 0.0;

        let assign15700_e22379: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign15700_e22379;
        locals.var_guard474_rv = 0.0;

        let (assign15710_e22395,) = {
    if ((((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15710_e22395;
        locals.var_mm_rv = 0.0;

        let assign15720_e22398: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign15720_e22398;
        locals.var_guard475_rv = 0.0;

        let (assign15730_e22417,) = {
    if (((((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 == 0.0)) && (locals.var_guard475 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15730_e22417;
        locals.var_mm_rv = 0.0;

        let assign15740_e22420: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard476 = assign15740_e22420;
        locals.var_guard476_rv = 0.0;

        let (assign15750_e22442,) = {
    if ((((((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 == 0.0)) && (locals.var_guard475 == 0.0)) && (locals.var_guard476 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15750_e22442;
        locals.var_mm_rv = 0.0;

        let (assign15760_e22453,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) && (locals.var_guard472 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign15760_e22453;
        locals.var_m0_rv = 0.0;

        let mut assign15770_loop_guard: usize = 0;
        while {
            let assign15770_cond_e22465: f64 = if (((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign15770_cond_e22465 != 0.0
        } {
            assign15770_loop_guard += 1;
            assert!(assign15770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15770_body0_e22477, assign15770_body0_e22477_d_n0, assign15770_body0_e22477_d_n2, assign15770_body0_e22477_d_n6, assign15770_body0_e22477_d_n7, assign15770_body0_e22477_d_n10, assign15770_body0_e22477_d_n11, assign15770_body0_e22477_d_n12, assign15770_body0_e22477_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) && (locals.var_guard472 != 0.0)) {
        let assign15770_body0_e22475: f64 = (locals.var_dnm).sqrt();
        (assign15770_body0_e22475, (locals.var_dnm_dn0 / (2.0 * assign15770_body0_e22475)), (locals.var_dnm_dn2 / (2.0 * assign15770_body0_e22475)), (locals.var_dnm_dn6 / (2.0 * assign15770_body0_e22475)), (locals.var_dnm_dn7 / (2.0 * assign15770_body0_e22475)), (locals.var_dnm_dn10 / (2.0 * assign15770_body0_e22475)), (locals.var_dnm_dn11 / (2.0 * assign15770_body0_e22475)), (locals.var_dnm_dn12 / (2.0 * assign15770_body0_e22475)), (locals.var_dnm_dn17 / (2.0 * assign15770_body0_e22475)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign15770_body0_e22477;
            locals.var_dnm_dn0 = assign15770_body0_e22477_d_n0;
            locals.var_dnm_dn2 = assign15770_body0_e22477_d_n2;
            locals.var_dnm_dn6 = assign15770_body0_e22477_d_n6;
            locals.var_dnm_dn7 = assign15770_body0_e22477_d_n7;
            locals.var_dnm_dn10 = assign15770_body0_e22477_d_n10;
            locals.var_dnm_dn11 = assign15770_body0_e22477_d_n11;
            locals.var_dnm_dn12 = assign15770_body0_e22477_d_n12;
            locals.var_dnm_dn17 = assign15770_body0_e22477_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign15770_body1_e22490,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) && (locals.var_guard472 != 0.0)) {
        let assign15770_body1_e22488: f64 = (locals.var_m0 + 1.0);
        (assign15770_body1_e22488,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign15770_body1_e22490;
            locals.var_m0_rv = 0.0;
        }

        let (assign15780_e22508, assign15780_e22508_d_n0, assign15780_e22508_d_n2, assign15780_e22508_d_n6, assign15780_e22508_d_n7, assign15780_e22508_d_n10, assign15780_e22508_d_n11, assign15780_e22508_d_n12, assign15780_e22508_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) && (locals.var_guard472 == 0.0)) {
        let assign15780_e22504: f64 = (2.0 * 4.0);
        let assign15780_e22505: f64 = (1.0 / assign15780_e22504);
        let assign15780_e22506: f64 = (locals.var_dnm).powf(assign15780_e22505);
        (assign15780_e22506, if 0.0 == 0.0 && ((assign15780_e22505) as f64).is_finite() && ((assign15780_e22505) as f64).fract() == 0.0 { if assign15780_e22505 == 0.0 { 0.0 } else { (assign15780_e22505 * ((locals.var_dnm).powf(assign15780_e22505 - 1.0) * locals.var_dnm_dn0)) } } else { (assign15780_e22506 * (assign15780_e22505 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15780_e22505) as f64).is_finite() && ((assign15780_e22505) as f64).fract() == 0.0 { if assign15780_e22505 == 0.0 { 0.0 } else { (assign15780_e22505 * ((locals.var_dnm).powf(assign15780_e22505 - 1.0) * locals.var_dnm_dn2)) } } else { (assign15780_e22506 * (assign15780_e22505 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15780_e22505) as f64).is_finite() && ((assign15780_e22505) as f64).fract() == 0.0 { if assign15780_e22505 == 0.0 { 0.0 } else { (assign15780_e22505 * ((locals.var_dnm).powf(assign15780_e22505 - 1.0) * locals.var_dnm_dn6)) } } else { (assign15780_e22506 * (assign15780_e22505 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15780_e22505) as f64).is_finite() && ((assign15780_e22505) as f64).fract() == 0.0 { if assign15780_e22505 == 0.0 { 0.0 } else { (assign15780_e22505 * ((locals.var_dnm).powf(assign15780_e22505 - 1.0) * locals.var_dnm_dn7)) } } else { (assign15780_e22506 * (assign15780_e22505 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15780_e22505) as f64).is_finite() && ((assign15780_e22505) as f64).fract() == 0.0 { if assign15780_e22505 == 0.0 { 0.0 } else { (assign15780_e22505 * ((locals.var_dnm).powf(assign15780_e22505 - 1.0) * locals.var_dnm_dn10)) } } else { (assign15780_e22506 * (assign15780_e22505 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15780_e22505) as f64).is_finite() && ((assign15780_e22505) as f64).fract() == 0.0 { if assign15780_e22505 == 0.0 { 0.0 } else { (assign15780_e22505 * ((locals.var_dnm).powf(assign15780_e22505 - 1.0) * locals.var_dnm_dn11)) } } else { (assign15780_e22506 * (assign15780_e22505 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15780_e22505) as f64).is_finite() && ((assign15780_e22505) as f64).fract() == 0.0 { if assign15780_e22505 == 0.0 { 0.0 } else { (assign15780_e22505 * ((locals.var_dnm).powf(assign15780_e22505 - 1.0) * locals.var_dnm_dn12)) } } else { (assign15780_e22506 * (assign15780_e22505 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15780_e22505) as f64).is_finite() && ((assign15780_e22505) as f64).fract() == 0.0 { if assign15780_e22505 == 0.0 { 0.0 } else { (assign15780_e22505 * ((locals.var_dnm).powf(assign15780_e22505 - 1.0) * locals.var_dnm_dn17)) } } else { (assign15780_e22506 * (assign15780_e22505 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15780_e22508;
        locals.var_dnm_dn0 = assign15780_e22508_d_n0;
        locals.var_dnm_dn2 = assign15780_e22508_d_n2;
        locals.var_dnm_dn6 = assign15780_e22508_d_n6;
        locals.var_dnm_dn7 = assign15780_e22508_d_n7;
        locals.var_dnm_dn10 = assign15780_e22508_d_n10;
        locals.var_dnm_dn11 = assign15780_e22508_d_n11;
        locals.var_dnm_dn12 = assign15780_e22508_d_n12;
        locals.var_dnm_dn17 = assign15780_e22508_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign15790_e22519, assign15790_e22519_d_n0, assign15790_e22519_d_n2, assign15790_e22519_d_n6, assign15790_e22519_d_n7, assign15790_e22519_d_n10, assign15790_e22519_d_n11, assign15790_e22519_d_n12, assign15790_e22519_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15790_e22517: f64 = (1.0 / locals.var_dnm);
        (assign15790_e22517, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15790_e22519;
        locals.var_dnm_dn0 = assign15790_e22519_d_n0;
        locals.var_dnm_dn2 = assign15790_e22519_d_n2;
        locals.var_dnm_dn6 = assign15790_e22519_d_n6;
        locals.var_dnm_dn7 = assign15790_e22519_d_n7;
        locals.var_dnm_dn10 = assign15790_e22519_d_n10;
        locals.var_dnm_dn11 = assign15790_e22519_d_n11;
        locals.var_dnm_dn12 = assign15790_e22519_d_n12;
        locals.var_dnm_dn17 = assign15790_e22519_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign15800_e22532, assign15800_e22532_d_n0, assign15800_e22532_d_n2, assign15800_e22532_d_n6, assign15800_e22532_d_n7, assign15800_e22532_d_n10, assign15800_e22532_d_n11, assign15800_e22532_d_n12, assign15800_e22532_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15800_e22528: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign15800_e22530: f64 = (assign15800_e22528 * locals.var_dnm);
        (assign15800_e22530, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign15800_e22528 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign15800_e22528 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign15800_e22528 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign15800_e22528 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign15800_e22528 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign15800_e22528 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn12)) * locals.var_dnm) + (assign15800_e22528 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn17)) * locals.var_dnm) + (assign15800_e22528 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign15800_e22532;
        locals.var_tmf0_dn0 = assign15800_e22532_d_n0;
        locals.var_tmf0_dn2 = assign15800_e22532_d_n2;
        locals.var_tmf0_dn6 = assign15800_e22532_d_n6;
        locals.var_tmf0_dn7 = assign15800_e22532_d_n7;
        locals.var_tmf0_dn10 = assign15800_e22532_d_n10;
        locals.var_tmf0_dn11 = assign15800_e22532_d_n11;
        locals.var_tmf0_dn12 = assign15800_e22532_d_n12;
        locals.var_tmf0_dn17 = assign15800_e22532_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign15810_e22545, assign15810_e22545_d_n0, assign15810_e22545_d_n2, assign15810_e22545_d_n6, assign15810_e22545_d_n7, assign15810_e22545_d_n10, assign15810_e22545_d_n11, assign15810_e22545_d_n12, assign15810_e22545_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign15810_e22541: f64 = locals.var_t5;
        let assign15810_e22543: f64 = (assign15810_e22541 - locals.var_tmf0);
        (assign15810_e22543, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn12 - locals.var_tmf0_dn12), (locals.var_t5_dn17 - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t4__blk467, locals.var_t4__blk467_dn0, locals.var_t4__blk467_dn2, locals.var_t4__blk467_dn6, locals.var_t4__blk467_dn7, locals.var_t4__blk467_dn10, locals.var_t4__blk467_dn11, locals.var_t4__blk467_dn12, locals.var_t4__blk467_dn17,)
    }
};
        locals.var_t4__blk467 = assign15810_e22545;
        locals.var_t4__blk467_dn0 = assign15810_e22545_d_n0;
        locals.var_t4__blk467_dn2 = assign15810_e22545_d_n2;
        locals.var_t4__blk467_dn6 = assign15810_e22545_d_n6;
        locals.var_t4__blk467_dn7 = assign15810_e22545_d_n7;
        locals.var_t4__blk467_dn10 = assign15810_e22545_d_n10;
        locals.var_t4__blk467_dn11 = assign15810_e22545_d_n11;
        locals.var_t4__blk467_dn12 = assign15810_e22545_d_n12;
        locals.var_t4__blk467_dn17 = assign15810_e22545_d_n17;
        locals.var_t4__blk467_rv = 0.0;

        let (assign15820_e22555, assign15820_e22555_d_n0, assign15820_e22555_d_n2, assign15820_e22555_d_n6, assign15820_e22555_d_n7, assign15820_e22555_d_n10, assign15820_e22555_d_n11, assign15820_e22555_d_n12, assign15820_e22555_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard471 == 0.0)) {
        (locals.var_t4__blk467, locals.var_t4__blk467_dn0, locals.var_t4__blk467_dn2, locals.var_t4__blk467_dn6, locals.var_t4__blk467_dn7, locals.var_t4__blk467_dn10, locals.var_t4__blk467_dn11, locals.var_t4__blk467_dn12, locals.var_t4__blk467_dn17,)
    } else {
        (locals.var_t4__blk467, locals.var_t4__blk467_dn0, locals.var_t4__blk467_dn2, locals.var_t4__blk467_dn6, locals.var_t4__blk467_dn7, locals.var_t4__blk467_dn10, locals.var_t4__blk467_dn11, locals.var_t4__blk467_dn12, locals.var_t4__blk467_dn17,)
    }
};
        locals.var_t4__blk467 = assign15820_e22555;
        locals.var_t4__blk467_dn0 = assign15820_e22555_d_n0;
        locals.var_t4__blk467_dn2 = assign15820_e22555_d_n2;
        locals.var_t4__blk467_dn6 = assign15820_e22555_d_n6;
        locals.var_t4__blk467_dn7 = assign15820_e22555_d_n7;
        locals.var_t4__blk467_dn10 = assign15820_e22555_d_n10;
        locals.var_t4__blk467_dn11 = assign15820_e22555_d_n11;
        locals.var_t4__blk467_dn12 = assign15820_e22555_d_n12;
        locals.var_t4__blk467_dn17 = assign15820_e22555_d_n17;
        locals.var_t4__blk467_rv = 0.0;

        let (assign15830_e22563, assign15830_e22563_d_n0, assign15830_e22563_d_n2, assign15830_e22563_d_n6, assign15830_e22563_d_n7, assign15830_e22563_d_n10, assign15830_e22563_d_n11, assign15830_e22563_d_n12, assign15830_e22563_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15830_e22561: f64 = (locals.var_t4__blk467).sqrt();
        (assign15830_e22561, (locals.var_t4__blk467_dn0 / (2.0 * assign15830_e22561)), (locals.var_t4__blk467_dn2 / (2.0 * assign15830_e22561)), (locals.var_t4__blk467_dn6 / (2.0 * assign15830_e22561)), (locals.var_t4__blk467_dn7 / (2.0 * assign15830_e22561)), (locals.var_t4__blk467_dn10 / (2.0 * assign15830_e22561)), (locals.var_t4__blk467_dn11 / (2.0 * assign15830_e22561)), (locals.var_t4__blk467_dn12 / (2.0 * assign15830_e22561)), (locals.var_t4__blk467_dn17 / (2.0 * assign15830_e22561)),)
    } else {
        (locals.var_t3__blk466, locals.var_t3__blk466_dn0, locals.var_t3__blk466_dn2, locals.var_t3__blk466_dn6, locals.var_t3__blk466_dn7, locals.var_t3__blk466_dn10, locals.var_t3__blk466_dn11, locals.var_t3__blk466_dn12, locals.var_t3__blk466_dn17,)
    }
};
        locals.var_t3__blk466 = assign15830_e22563;
        locals.var_t3__blk466_dn0 = assign15830_e22563_d_n0;
        locals.var_t3__blk466_dn2 = assign15830_e22563_d_n2;
        locals.var_t3__blk466_dn6 = assign15830_e22563_d_n6;
        locals.var_t3__blk466_dn7 = assign15830_e22563_d_n7;
        locals.var_t3__blk466_dn10 = assign15830_e22563_d_n10;
        locals.var_t3__blk466_dn11 = assign15830_e22563_d_n11;
        locals.var_t3__blk466_dn12 = assign15830_e22563_d_n12;
        locals.var_t3__blk466_dn17 = assign15830_e22563_d_n17;
        locals.var_t3__blk466_rv = 0.0;

        let (assign15840_e22576, assign15840_e22576_d_n0, assign15840_e22576_d_n2, assign15840_e22576_d_n6, assign15840_e22576_d_n7, assign15840_e22576_d_n10, assign15840_e22576_d_n11, assign15840_e22576_d_n12, assign15840_e22576_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15840_e22572: f64 = (1.0 - locals.var_t3__blk466);
        let assign15840_e22573: f64 = (locals.var_t2__blk465 * assign15840_e22572);
        let assign15840_e22574: f64 = (locals.var_vgp + assign15840_e22573);
        (assign15840_e22574, (locals.var_vgp_dn0 + ((locals.var_t2__blk465_dn0 * assign15840_e22572) + (locals.var_t2__blk465 * (-locals.var_t3__blk466_dn0)))), (locals.var_vgp_dn2 + ((locals.var_t2__blk465_dn2 * assign15840_e22572) + (locals.var_t2__blk465 * (-locals.var_t3__blk466_dn2)))), (locals.var_vgp_dn6 + ((locals.var_t2__blk465_dn6 * assign15840_e22572) + (locals.var_t2__blk465 * (-locals.var_t3__blk466_dn6)))), (locals.var_vgp_dn7 + ((locals.var_t2__blk465_dn7 * assign15840_e22572) + (locals.var_t2__blk465 * (-locals.var_t3__blk466_dn7)))), (locals.var_vgp_dn10 + ((locals.var_t2__blk465_dn10 * assign15840_e22572) + (locals.var_t2__blk465 * (-locals.var_t3__blk466_dn10)))), (locals.var_vgp_dn11 + ((locals.var_t2__blk465_dn11 * assign15840_e22572) + (locals.var_t2__blk465 * (-locals.var_t3__blk466_dn11)))), (locals.var_vgp_dn12 + ((locals.var_t2__blk465_dn12 * assign15840_e22572) + (locals.var_t2__blk465 * (-locals.var_t3__blk466_dn12)))), (locals.var_vgp_dn17 + ((locals.var_t2__blk465_dn17 * assign15840_e22572) + (locals.var_t2__blk465 * (-locals.var_t3__blk466_dn17)))),)
    } else {
        (locals.var_t10__blk470, locals.var_t10__blk470_dn0, locals.var_t10__blk470_dn2, locals.var_t10__blk470_dn6, locals.var_t10__blk470_dn7, locals.var_t10__blk470_dn10, locals.var_t10__blk470_dn11, locals.var_t10__blk470_dn12, locals.var_t10__blk470_dn17,)
    }
};
        locals.var_t10__blk470 = assign15840_e22576;
        locals.var_t10__blk470_dn0 = assign15840_e22576_d_n0;
        locals.var_t10__blk470_dn2 = assign15840_e22576_d_n2;
        locals.var_t10__blk470_dn6 = assign15840_e22576_d_n6;
        locals.var_t10__blk470_dn7 = assign15840_e22576_d_n7;
        locals.var_t10__blk470_dn10 = assign15840_e22576_d_n10;
        locals.var_t10__blk470_dn11 = assign15840_e22576_d_n11;
        locals.var_t10__blk470_dn12 = assign15840_e22576_d_n12;
        locals.var_t10__blk470_dn17 = assign15840_e22576_d_n17;
        locals.var_t10__blk470_rv = 0.0;

        let (assign15850_e22592, assign15850_e22592_d_n0, assign15850_e22592_d_n2, assign15850_e22592_d_n6, assign15850_e22592_d_n7, assign15850_e22592_d_n10, assign15850_e22592_d_n11, assign15850_e22592_d_n12, assign15850_e22592_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15850_e22583: f64 = (locals.var_t10__blk470 * locals.var_t10__blk470);
        let assign15850_e22586: f64 = (4.0 * 0.01);
        let assign15850_e22588: f64 = (assign15850_e22586 * 0.01);
        let assign15850_e22589: f64 = (assign15850_e22583 + assign15850_e22588);
        let assign15850_e22590: f64 = (assign15850_e22589).sqrt();
        (assign15850_e22590, (((locals.var_t10__blk470_dn0 * locals.var_t10__blk470) + (locals.var_t10__blk470 * locals.var_t10__blk470_dn0)) / (2.0 * assign15850_e22590)), (((locals.var_t10__blk470_dn2 * locals.var_t10__blk470) + (locals.var_t10__blk470 * locals.var_t10__blk470_dn2)) / (2.0 * assign15850_e22590)), (((locals.var_t10__blk470_dn6 * locals.var_t10__blk470) + (locals.var_t10__blk470 * locals.var_t10__blk470_dn6)) / (2.0 * assign15850_e22590)), (((locals.var_t10__blk470_dn7 * locals.var_t10__blk470) + (locals.var_t10__blk470 * locals.var_t10__blk470_dn7)) / (2.0 * assign15850_e22590)), (((locals.var_t10__blk470_dn10 * locals.var_t10__blk470) + (locals.var_t10__blk470 * locals.var_t10__blk470_dn10)) / (2.0 * assign15850_e22590)), (((locals.var_t10__blk470_dn11 * locals.var_t10__blk470) + (locals.var_t10__blk470 * locals.var_t10__blk470_dn11)) / (2.0 * assign15850_e22590)), (((locals.var_t10__blk470_dn12 * locals.var_t10__blk470) + (locals.var_t10__blk470 * locals.var_t10__blk470_dn12)) / (2.0 * assign15850_e22590)), (((locals.var_t10__blk470_dn17 * locals.var_t10__blk470) + (locals.var_t10__blk470 * locals.var_t10__blk470_dn17)) / (2.0 * assign15850_e22590)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign15850_e22592;
        locals.var_tmf1_dn0 = assign15850_e22592_d_n0;
        locals.var_tmf1_dn2 = assign15850_e22592_d_n2;
        locals.var_tmf1_dn6 = assign15850_e22592_d_n6;
        locals.var_tmf1_dn7 = assign15850_e22592_d_n7;
        locals.var_tmf1_dn10 = assign15850_e22592_d_n10;
        locals.var_tmf1_dn11 = assign15850_e22592_d_n11;
        locals.var_tmf1_dn12 = assign15850_e22592_d_n12;
        locals.var_tmf1_dn17 = assign15850_e22592_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign15860_e22607, assign15860_e22607_d_n0, assign15860_e22607_d_n2, assign15860_e22607_d_n6, assign15860_e22607_d_n7, assign15860_e22607_d_n10, assign15860_e22607_d_n11, assign15860_e22607_d_n12, assign15860_e22607_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15860_e22600: f64 = (locals.var_t10__blk470 + locals.var_tmf1);
        let assign15860_e22601: f64 = (0.5 * assign15860_e22600);
        let assign15860_e22604: f64 = (1e-10 * 0.01);
        let assign15860_e22605: f64 = (assign15860_e22601 + assign15860_e22604);
        (assign15860_e22605, (0.5 * (locals.var_t10__blk470_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t10__blk470_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t10__blk470_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t10__blk470_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t10__blk470_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t10__blk470_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t10__blk470_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t10__blk470_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t10__blk470, locals.var_t10__blk470_dn0, locals.var_t10__blk470_dn2, locals.var_t10__blk470_dn6, locals.var_t10__blk470_dn7, locals.var_t10__blk470_dn10, locals.var_t10__blk470_dn11, locals.var_t10__blk470_dn12, locals.var_t10__blk470_dn17,)
    }
};
        locals.var_t10__blk470 = assign15860_e22607;
        locals.var_t10__blk470_dn0 = assign15860_e22607_d_n0;
        locals.var_t10__blk470_dn2 = assign15860_e22607_d_n2;
        locals.var_t10__blk470_dn6 = assign15860_e22607_d_n6;
        locals.var_t10__blk470_dn7 = assign15860_e22607_d_n7;
        locals.var_t10__blk470_dn10 = assign15860_e22607_d_n10;
        locals.var_t10__blk470_dn11 = assign15860_e22607_d_n11;
        locals.var_t10__blk470_dn12 = assign15860_e22607_d_n12;
        locals.var_t10__blk470_dn17 = assign15860_e22607_d_n17;
        locals.var_t10__blk470_rv = 0.0;

        let assign15870_e22610: f64 = if locals.var_t10__blk470 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign15870_e22610;
        locals.var_guard477_rv = 0.0;

        let (assign15880_e22619, assign15880_e22619_d_n0, assign15880_e22619_d_n2, assign15880_e22619_d_n6, assign15880_e22619_d_n7, assign15880_e22619_d_n10, assign15880_e22619_d_n11, assign15880_e22619_d_n12, assign15880_e22619_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard477 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10__blk470, locals.var_t10__blk470_dn0, locals.var_t10__blk470_dn2, locals.var_t10__blk470_dn6, locals.var_t10__blk470_dn7, locals.var_t10__blk470_dn10, locals.var_t10__blk470_dn11, locals.var_t10__blk470_dn12, locals.var_t10__blk470_dn17,)
    }
};
        locals.var_t10__blk470 = assign15880_e22619;
        locals.var_t10__blk470_dn0 = assign15880_e22619_d_n0;
        locals.var_t10__blk470_dn2 = assign15880_e22619_d_n2;
        locals.var_t10__blk470_dn6 = assign15880_e22619_d_n6;
        locals.var_t10__blk470_dn7 = assign15880_e22619_d_n7;
        locals.var_t10__blk470_dn10 = assign15880_e22619_d_n10;
        locals.var_t10__blk470_dn11 = assign15880_e22619_d_n11;
        locals.var_t10__blk470_dn12 = assign15880_e22619_d_n12;
        locals.var_t10__blk470_dn17 = assign15880_e22619_d_n17;
        locals.var_t10__blk470_rv = 0.0;

        let (assign15900_e22635, assign15900_e22635_d_n0, assign15900_e22635_d_n2, assign15900_e22635_d_n6, assign15900_e22635_d_n7, assign15900_e22635_d_n10, assign15900_e22635_d_n11, assign15900_e22635_d_n12, assign15900_e22635_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15900_e22633: f64 = (locals.var_vds / locals.var_t10__blk470);
        (assign15900_e22633, (((locals.var_vds_dn0 * locals.var_t10__blk470) - (locals.var_vds * locals.var_t10__blk470_dn0)) / (locals.var_t10__blk470 * locals.var_t10__blk470)), (((locals.var_vds_dn2 * locals.var_t10__blk470) - (locals.var_vds * locals.var_t10__blk470_dn2)) / (locals.var_t10__blk470 * locals.var_t10__blk470)), (((locals.var_vds_dn6 * locals.var_t10__blk470) - (locals.var_vds * locals.var_t10__blk470_dn6)) / (locals.var_t10__blk470 * locals.var_t10__blk470)), (((locals.var_vds_dn7 * locals.var_t10__blk470) - (locals.var_vds * locals.var_t10__blk470_dn7)) / (locals.var_t10__blk470 * locals.var_t10__blk470)), (((locals.var_vds_dn10 * locals.var_t10__blk470) - (locals.var_vds * locals.var_t10__blk470_dn10)) / (locals.var_t10__blk470 * locals.var_t10__blk470)), (((locals.var_vds_dn11 * locals.var_t10__blk470) - (locals.var_vds * locals.var_t10__blk470_dn11)) / (locals.var_t10__blk470 * locals.var_t10__blk470)), (((locals.var_vds_dn12 * locals.var_t10__blk470) - (locals.var_vds * locals.var_t10__blk470_dn12)) / (locals.var_t10__blk470 * locals.var_t10__blk470)), (((locals.var_vds_dn17 * locals.var_t10__blk470) - (locals.var_vds * locals.var_t10__blk470_dn17)) / (locals.var_t10__blk470 * locals.var_t10__blk470)),)
    } else {
        (locals.var_t1__blk464, locals.var_t1__blk464_dn0, locals.var_t1__blk464_dn2, locals.var_t1__blk464_dn6, locals.var_t1__blk464_dn7, locals.var_t1__blk464_dn10, locals.var_t1__blk464_dn11, locals.var_t1__blk464_dn12, locals.var_t1__blk464_dn17,)
    }
};
        locals.var_t1__blk464 = assign15900_e22635;
        locals.var_t1__blk464_dn0 = assign15900_e22635_d_n0;
        locals.var_t1__blk464_dn2 = assign15900_e22635_d_n2;
        locals.var_t1__blk464_dn6 = assign15900_e22635_d_n6;
        locals.var_t1__blk464_dn7 = assign15900_e22635_d_n7;
        locals.var_t1__blk464_dn10 = assign15900_e22635_d_n10;
        locals.var_t1__blk464_dn11 = assign15900_e22635_d_n11;
        locals.var_t1__blk464_dn12 = assign15900_e22635_d_n12;
        locals.var_t1__blk464_dn17 = assign15900_e22635_d_n17;
        locals.var_t1__blk464_rv = 0.0;

        let (assign15910_e22646, assign15910_e22646_d_n0, assign15910_e22646_d_n2, assign15910_e22646_d_n6, assign15910_e22646_d_n7, assign15910_e22646_d_n10, assign15910_e22646_d_n11, assign15910_e22646_d_n12, assign15910_e22646_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15910_e22643: f64 = (locals.var_ddlte - 1.0);
        let assign15910_e22644: f64 = (locals.var_t1__blk464).powf(assign15910_e22643);
        (assign15910_e22644, if 0.0 == 0.0 && ((assign15910_e22643) as f64).is_finite() && ((assign15910_e22643) as f64).fract() == 0.0 { if assign15910_e22643 == 0.0 { 0.0 } else { (assign15910_e22643 * ((locals.var_t1__blk464).powf(assign15910_e22643 - 1.0) * locals.var_t1__blk464_dn0)) } } else { (assign15910_e22644 * (assign15910_e22643 * (locals.var_t1__blk464_dn0 / locals.var_t1__blk464))) }, if 0.0 == 0.0 && ((assign15910_e22643) as f64).is_finite() && ((assign15910_e22643) as f64).fract() == 0.0 { if assign15910_e22643 == 0.0 { 0.0 } else { (assign15910_e22643 * ((locals.var_t1__blk464).powf(assign15910_e22643 - 1.0) * locals.var_t1__blk464_dn2)) } } else { (assign15910_e22644 * (assign15910_e22643 * (locals.var_t1__blk464_dn2 / locals.var_t1__blk464))) }, if 0.0 == 0.0 && ((assign15910_e22643) as f64).is_finite() && ((assign15910_e22643) as f64).fract() == 0.0 { if assign15910_e22643 == 0.0 { 0.0 } else { (assign15910_e22643 * ((locals.var_t1__blk464).powf(assign15910_e22643 - 1.0) * locals.var_t1__blk464_dn6)) } } else { (assign15910_e22644 * (assign15910_e22643 * (locals.var_t1__blk464_dn6 / locals.var_t1__blk464))) }, if 0.0 == 0.0 && ((assign15910_e22643) as f64).is_finite() && ((assign15910_e22643) as f64).fract() == 0.0 { if assign15910_e22643 == 0.0 { 0.0 } else { (assign15910_e22643 * ((locals.var_t1__blk464).powf(assign15910_e22643 - 1.0) * locals.var_t1__blk464_dn7)) } } else { (assign15910_e22644 * (assign15910_e22643 * (locals.var_t1__blk464_dn7 / locals.var_t1__blk464))) }, if 0.0 == 0.0 && ((assign15910_e22643) as f64).is_finite() && ((assign15910_e22643) as f64).fract() == 0.0 { if assign15910_e22643 == 0.0 { 0.0 } else { (assign15910_e22643 * ((locals.var_t1__blk464).powf(assign15910_e22643 - 1.0) * locals.var_t1__blk464_dn10)) } } else { (assign15910_e22644 * (assign15910_e22643 * (locals.var_t1__blk464_dn10 / locals.var_t1__blk464))) }, if 0.0 == 0.0 && ((assign15910_e22643) as f64).is_finite() && ((assign15910_e22643) as f64).fract() == 0.0 { if assign15910_e22643 == 0.0 { 0.0 } else { (assign15910_e22643 * ((locals.var_t1__blk464).powf(assign15910_e22643 - 1.0) * locals.var_t1__blk464_dn11)) } } else { (assign15910_e22644 * (assign15910_e22643 * (locals.var_t1__blk464_dn11 / locals.var_t1__blk464))) }, if 0.0 == 0.0 && ((assign15910_e22643) as f64).is_finite() && ((assign15910_e22643) as f64).fract() == 0.0 { if assign15910_e22643 == 0.0 { 0.0 } else { (assign15910_e22643 * ((locals.var_t1__blk464).powf(assign15910_e22643 - 1.0) * locals.var_t1__blk464_dn12)) } } else { (assign15910_e22644 * (assign15910_e22643 * (locals.var_t1__blk464_dn12 / locals.var_t1__blk464))) }, if 0.0 == 0.0 && ((assign15910_e22643) as f64).is_finite() && ((assign15910_e22643) as f64).fract() == 0.0 { if assign15910_e22643 == 0.0 { 0.0 } else { (assign15910_e22643 * ((locals.var_t1__blk464).powf(assign15910_e22643 - 1.0) * locals.var_t1__blk464_dn17)) } } else { (assign15910_e22644 * (assign15910_e22643 * (locals.var_t1__blk464_dn17 / locals.var_t1__blk464))) },)
    } else {
        (locals.var_t2__blk465, locals.var_t2__blk465_dn0, locals.var_t2__blk465_dn2, locals.var_t2__blk465_dn6, locals.var_t2__blk465_dn7, locals.var_t2__blk465_dn10, locals.var_t2__blk465_dn11, locals.var_t2__blk465_dn12, locals.var_t2__blk465_dn17,)
    }
};
        locals.var_t2__blk465 = assign15910_e22646;
        locals.var_t2__blk465_dn0 = assign15910_e22646_d_n0;
        locals.var_t2__blk465_dn2 = assign15910_e22646_d_n2;
        locals.var_t2__blk465_dn6 = assign15910_e22646_d_n6;
        locals.var_t2__blk465_dn7 = assign15910_e22646_d_n7;
        locals.var_t2__blk465_dn10 = assign15910_e22646_d_n10;
        locals.var_t2__blk465_dn11 = assign15910_e22646_d_n11;
        locals.var_t2__blk465_dn12 = assign15910_e22646_d_n12;
        locals.var_t2__blk465_dn17 = assign15910_e22646_d_n17;
        locals.var_t2__blk465_rv = 0.0;

        let (assign15920_e22655, assign15920_e22655_d_n0, assign15920_e22655_d_n2, assign15920_e22655_d_n6, assign15920_e22655_d_n7, assign15920_e22655_d_n10, assign15920_e22655_d_n11, assign15920_e22655_d_n12, assign15920_e22655_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15920_e22653: f64 = (locals.var_t2__blk465 * locals.var_t1__blk464);
        (assign15920_e22653, ((locals.var_t2__blk465_dn0 * locals.var_t1__blk464) + (locals.var_t2__blk465 * locals.var_t1__blk464_dn0)), ((locals.var_t2__blk465_dn2 * locals.var_t1__blk464) + (locals.var_t2__blk465 * locals.var_t1__blk464_dn2)), ((locals.var_t2__blk465_dn6 * locals.var_t1__blk464) + (locals.var_t2__blk465 * locals.var_t1__blk464_dn6)), ((locals.var_t2__blk465_dn7 * locals.var_t1__blk464) + (locals.var_t2__blk465 * locals.var_t1__blk464_dn7)), ((locals.var_t2__blk465_dn10 * locals.var_t1__blk464) + (locals.var_t2__blk465 * locals.var_t1__blk464_dn10)), ((locals.var_t2__blk465_dn11 * locals.var_t1__blk464) + (locals.var_t2__blk465 * locals.var_t1__blk464_dn11)), ((locals.var_t2__blk465_dn12 * locals.var_t1__blk464) + (locals.var_t2__blk465 * locals.var_t1__blk464_dn12)), ((locals.var_t2__blk465_dn17 * locals.var_t1__blk464) + (locals.var_t2__blk465 * locals.var_t1__blk464_dn17)),)
    } else {
        (locals.var_t7__blk469, locals.var_t7__blk469_dn0, locals.var_t7__blk469_dn2, locals.var_t7__blk469_dn6, locals.var_t7__blk469_dn7, locals.var_t7__blk469_dn10, locals.var_t7__blk469_dn11, locals.var_t7__blk469_dn12, locals.var_t7__blk469_dn17,)
    }
};
        locals.var_t7__blk469 = assign15920_e22655;
        locals.var_t7__blk469_dn0 = assign15920_e22655_d_n0;
        locals.var_t7__blk469_dn2 = assign15920_e22655_d_n2;
        locals.var_t7__blk469_dn6 = assign15920_e22655_d_n6;
        locals.var_t7__blk469_dn7 = assign15920_e22655_d_n7;
        locals.var_t7__blk469_dn10 = assign15920_e22655_d_n10;
        locals.var_t7__blk469_dn11 = assign15920_e22655_d_n11;
        locals.var_t7__blk469_dn12 = assign15920_e22655_d_n12;
        locals.var_t7__blk469_dn17 = assign15920_e22655_d_n17;
        locals.var_t7__blk469_rv = 0.0;

        let (assign15930_e22664, assign15930_e22664_d_n0, assign15930_e22664_d_n2, assign15930_e22664_d_n6, assign15930_e22664_d_n7, assign15930_e22664_d_n10, assign15930_e22664_d_n11, assign15930_e22664_d_n12, assign15930_e22664_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15930_e22662: f64 = (1.0 + locals.var_t7__blk469);
        (assign15930_e22662, locals.var_t7__blk469_dn0, locals.var_t7__blk469_dn2, locals.var_t7__blk469_dn6, locals.var_t7__blk469_dn7, locals.var_t7__blk469_dn10, locals.var_t7__blk469_dn11, locals.var_t7__blk469_dn12, locals.var_t7__blk469_dn17,)
    } else {
        (locals.var_t3__blk466, locals.var_t3__blk466_dn0, locals.var_t3__blk466_dn2, locals.var_t3__blk466_dn6, locals.var_t3__blk466_dn7, locals.var_t3__blk466_dn10, locals.var_t3__blk466_dn11, locals.var_t3__blk466_dn12, locals.var_t3__blk466_dn17,)
    }
};
        locals.var_t3__blk466 = assign15930_e22664;
        locals.var_t3__blk466_dn0 = assign15930_e22664_d_n0;
        locals.var_t3__blk466_dn2 = assign15930_e22664_d_n2;
        locals.var_t3__blk466_dn6 = assign15930_e22664_d_n6;
        locals.var_t3__blk466_dn7 = assign15930_e22664_d_n7;
        locals.var_t3__blk466_dn10 = assign15930_e22664_d_n10;
        locals.var_t3__blk466_dn11 = assign15930_e22664_d_n11;
        locals.var_t3__blk466_dn12 = assign15930_e22664_d_n12;
        locals.var_t3__blk466_dn17 = assign15930_e22664_d_n17;
        locals.var_t3__blk466_rv = 0.0;

        let (assign15940_e22677, assign15940_e22677_d_n0, assign15940_e22677_d_n2, assign15940_e22677_d_n6, assign15940_e22677_d_n7, assign15940_e22677_d_n10, assign15940_e22677_d_n11, assign15940_e22677_d_n12, assign15940_e22677_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15940_e22672: f64 = (1.0 / locals.var_ddlte);
        let assign15940_e22674: f64 = (assign15940_e22672 - 1.0);
        let assign15940_e22675: f64 = (locals.var_t3__blk466).powf(assign15940_e22674);
        (assign15940_e22675, if 0.0 == 0.0 && ((assign15940_e22674) as f64).is_finite() && ((assign15940_e22674) as f64).fract() == 0.0 { if assign15940_e22674 == 0.0 { 0.0 } else { (assign15940_e22674 * ((locals.var_t3__blk466).powf(assign15940_e22674 - 1.0) * locals.var_t3__blk466_dn0)) } } else { (assign15940_e22675 * (assign15940_e22674 * (locals.var_t3__blk466_dn0 / locals.var_t3__blk466))) }, if 0.0 == 0.0 && ((assign15940_e22674) as f64).is_finite() && ((assign15940_e22674) as f64).fract() == 0.0 { if assign15940_e22674 == 0.0 { 0.0 } else { (assign15940_e22674 * ((locals.var_t3__blk466).powf(assign15940_e22674 - 1.0) * locals.var_t3__blk466_dn2)) } } else { (assign15940_e22675 * (assign15940_e22674 * (locals.var_t3__blk466_dn2 / locals.var_t3__blk466))) }, if 0.0 == 0.0 && ((assign15940_e22674) as f64).is_finite() && ((assign15940_e22674) as f64).fract() == 0.0 { if assign15940_e22674 == 0.0 { 0.0 } else { (assign15940_e22674 * ((locals.var_t3__blk466).powf(assign15940_e22674 - 1.0) * locals.var_t3__blk466_dn6)) } } else { (assign15940_e22675 * (assign15940_e22674 * (locals.var_t3__blk466_dn6 / locals.var_t3__blk466))) }, if 0.0 == 0.0 && ((assign15940_e22674) as f64).is_finite() && ((assign15940_e22674) as f64).fract() == 0.0 { if assign15940_e22674 == 0.0 { 0.0 } else { (assign15940_e22674 * ((locals.var_t3__blk466).powf(assign15940_e22674 - 1.0) * locals.var_t3__blk466_dn7)) } } else { (assign15940_e22675 * (assign15940_e22674 * (locals.var_t3__blk466_dn7 / locals.var_t3__blk466))) }, if 0.0 == 0.0 && ((assign15940_e22674) as f64).is_finite() && ((assign15940_e22674) as f64).fract() == 0.0 { if assign15940_e22674 == 0.0 { 0.0 } else { (assign15940_e22674 * ((locals.var_t3__blk466).powf(assign15940_e22674 - 1.0) * locals.var_t3__blk466_dn10)) } } else { (assign15940_e22675 * (assign15940_e22674 * (locals.var_t3__blk466_dn10 / locals.var_t3__blk466))) }, if 0.0 == 0.0 && ((assign15940_e22674) as f64).is_finite() && ((assign15940_e22674) as f64).fract() == 0.0 { if assign15940_e22674 == 0.0 { 0.0 } else { (assign15940_e22674 * ((locals.var_t3__blk466).powf(assign15940_e22674 - 1.0) * locals.var_t3__blk466_dn11)) } } else { (assign15940_e22675 * (assign15940_e22674 * (locals.var_t3__blk466_dn11 / locals.var_t3__blk466))) }, if 0.0 == 0.0 && ((assign15940_e22674) as f64).is_finite() && ((assign15940_e22674) as f64).fract() == 0.0 { if assign15940_e22674 == 0.0 { 0.0 } else { (assign15940_e22674 * ((locals.var_t3__blk466).powf(assign15940_e22674 - 1.0) * locals.var_t3__blk466_dn12)) } } else { (assign15940_e22675 * (assign15940_e22674 * (locals.var_t3__blk466_dn12 / locals.var_t3__blk466))) }, if 0.0 == 0.0 && ((assign15940_e22674) as f64).is_finite() && ((assign15940_e22674) as f64).fract() == 0.0 { if assign15940_e22674 == 0.0 { 0.0 } else { (assign15940_e22674 * ((locals.var_t3__blk466).powf(assign15940_e22674 - 1.0) * locals.var_t3__blk466_dn17)) } } else { (assign15940_e22675 * (assign15940_e22674 * (locals.var_t3__blk466_dn17 / locals.var_t3__blk466))) },)
    } else {
        (locals.var_t4__blk467, locals.var_t4__blk467_dn0, locals.var_t4__blk467_dn2, locals.var_t4__blk467_dn6, locals.var_t4__blk467_dn7, locals.var_t4__blk467_dn10, locals.var_t4__blk467_dn11, locals.var_t4__blk467_dn12, locals.var_t4__blk467_dn17,)
    }
};
        locals.var_t4__blk467 = assign15940_e22677;
        locals.var_t4__blk467_dn0 = assign15940_e22677_d_n0;
        locals.var_t4__blk467_dn2 = assign15940_e22677_d_n2;
        locals.var_t4__blk467_dn6 = assign15940_e22677_d_n6;
        locals.var_t4__blk467_dn7 = assign15940_e22677_d_n7;
        locals.var_t4__blk467_dn10 = assign15940_e22677_d_n10;
        locals.var_t4__blk467_dn11 = assign15940_e22677_d_n11;
        locals.var_t4__blk467_dn12 = assign15940_e22677_d_n12;
        locals.var_t4__blk467_dn17 = assign15940_e22677_d_n17;
        locals.var_t4__blk467_rv = 0.0;

        let (assign15950_e22686, assign15950_e22686_d_n0, assign15950_e22686_d_n2, assign15950_e22686_d_n6, assign15950_e22686_d_n7, assign15950_e22686_d_n10, assign15950_e22686_d_n11, assign15950_e22686_d_n12, assign15950_e22686_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15950_e22684: f64 = (locals.var_t4__blk467 * locals.var_t3__blk466);
        (assign15950_e22684, ((locals.var_t4__blk467_dn0 * locals.var_t3__blk466) + (locals.var_t4__blk467 * locals.var_t3__blk466_dn0)), ((locals.var_t4__blk467_dn2 * locals.var_t3__blk466) + (locals.var_t4__blk467 * locals.var_t3__blk466_dn2)), ((locals.var_t4__blk467_dn6 * locals.var_t3__blk466) + (locals.var_t4__blk467 * locals.var_t3__blk466_dn6)), ((locals.var_t4__blk467_dn7 * locals.var_t3__blk466) + (locals.var_t4__blk467 * locals.var_t3__blk466_dn7)), ((locals.var_t4__blk467_dn10 * locals.var_t3__blk466) + (locals.var_t4__blk467 * locals.var_t3__blk466_dn10)), ((locals.var_t4__blk467_dn11 * locals.var_t3__blk466) + (locals.var_t4__blk467 * locals.var_t3__blk466_dn11)), ((locals.var_t4__blk467_dn12 * locals.var_t3__blk466) + (locals.var_t4__blk467 * locals.var_t3__blk466_dn12)), ((locals.var_t4__blk467_dn17 * locals.var_t3__blk466) + (locals.var_t4__blk467 * locals.var_t3__blk466_dn17)),)
    } else {
        (locals.var_t6__blk468, locals.var_t6__blk468_dn0, locals.var_t6__blk468_dn2, locals.var_t6__blk468_dn6, locals.var_t6__blk468_dn7, locals.var_t6__blk468_dn10, locals.var_t6__blk468_dn11, locals.var_t6__blk468_dn12, locals.var_t6__blk468_dn17,)
    }
};
        locals.var_t6__blk468 = assign15950_e22686;
        locals.var_t6__blk468_dn0 = assign15950_e22686_d_n0;
        locals.var_t6__blk468_dn2 = assign15950_e22686_d_n2;
        locals.var_t6__blk468_dn6 = assign15950_e22686_d_n6;
        locals.var_t6__blk468_dn7 = assign15950_e22686_d_n7;
        locals.var_t6__blk468_dn10 = assign15950_e22686_d_n10;
        locals.var_t6__blk468_dn11 = assign15950_e22686_d_n11;
        locals.var_t6__blk468_dn12 = assign15950_e22686_d_n12;
        locals.var_t6__blk468_dn17 = assign15950_e22686_d_n17;
        locals.var_t6__blk468_rv = 0.0;

        let (assign15960_e22695, assign15960_e22695_d_n0, assign15960_e22695_d_n2, assign15960_e22695_d_n6, assign15960_e22695_d_n7, assign15960_e22695_d_n10, assign15960_e22695_d_n11, assign15960_e22695_d_n12, assign15960_e22695_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15960_e22693: f64 = (locals.var_vds / locals.var_t6__blk468);
        (assign15960_e22693, (((locals.var_vds_dn0 * locals.var_t6__blk468) - (locals.var_vds * locals.var_t6__blk468_dn0)) / (locals.var_t6__blk468 * locals.var_t6__blk468)), (((locals.var_vds_dn2 * locals.var_t6__blk468) - (locals.var_vds * locals.var_t6__blk468_dn2)) / (locals.var_t6__blk468 * locals.var_t6__blk468)), (((locals.var_vds_dn6 * locals.var_t6__blk468) - (locals.var_vds * locals.var_t6__blk468_dn6)) / (locals.var_t6__blk468 * locals.var_t6__blk468)), (((locals.var_vds_dn7 * locals.var_t6__blk468) - (locals.var_vds * locals.var_t6__blk468_dn7)) / (locals.var_t6__blk468 * locals.var_t6__blk468)), (((locals.var_vds_dn10 * locals.var_t6__blk468) - (locals.var_vds * locals.var_t6__blk468_dn10)) / (locals.var_t6__blk468 * locals.var_t6__blk468)), (((locals.var_vds_dn11 * locals.var_t6__blk468) - (locals.var_vds * locals.var_t6__blk468_dn11)) / (locals.var_t6__blk468 * locals.var_t6__blk468)), (((locals.var_vds_dn12 * locals.var_t6__blk468) - (locals.var_vds * locals.var_t6__blk468_dn12)) / (locals.var_t6__blk468 * locals.var_t6__blk468)), (((locals.var_vds_dn17 * locals.var_t6__blk468) - (locals.var_vds * locals.var_t6__blk468_dn17)) / (locals.var_t6__blk468 * locals.var_t6__blk468)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    }
};
        locals.var_vdseff = assign15960_e22695;
        locals.var_vdseff_dn0 = assign15960_e22695_d_n0;
        locals.var_vdseff_dn2 = assign15960_e22695_d_n2;
        locals.var_vdseff_dn6 = assign15960_e22695_d_n6;
        locals.var_vdseff_dn7 = assign15960_e22695_d_n7;
        locals.var_vdseff_dn10 = assign15960_e22695_d_n10;
        locals.var_vdseff_dn11 = assign15960_e22695_d_n11;
        locals.var_vdseff_dn12 = assign15960_e22695_d_n12;
        locals.var_vdseff_dn17 = assign15960_e22695_d_n17;
        locals.var_vdseff_rv = 0.0;

        let (assign15970_e22702, assign15970_e22702_d_n0, assign15970_e22702_d_n2, assign15970_e22702_d_n6, assign15970_e22702_d_n7, assign15970_e22702_d_n10, assign15970_e22702_d_n11, assign15970_e22702_d_n12, assign15970_e22702_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign15970_e22702;
        locals.var_vds_dn0 = assign15970_e22702_d_n0;
        locals.var_vds_dn2 = assign15970_e22702_d_n2;
        locals.var_vds_dn6 = assign15970_e22702_d_n6;
        locals.var_vds_dn7 = assign15970_e22702_d_n7;
        locals.var_vds_dn10 = assign15970_e22702_d_n10;
        locals.var_vds_dn11 = assign15970_e22702_d_n11;
        locals.var_vds_dn12 = assign15970_e22702_d_n12;
        locals.var_vds_dn17 = assign15970_e22702_d_n17;
        locals.var_vds_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_56(
        locals: &mut StampLocals,
    ) {
        let (assign15980_e22714, assign15980_e22714_d_n0, assign15980_e22714_d_n2, assign15980_e22714_d_n6, assign15980_e22714_d_n7, assign15980_e22714_d_n10, assign15980_e22714_d_n11, assign15980_e22714_d_n12, assign15980_e22714_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign15980_e22710: f64 = (locals.var_vbcs_cl - locals.var_vds);
        let assign15980_e22711: f64 = (locals.var_beta * assign15980_e22710);
        let assign15980_e22712: f64 = (assign15980_e22711).exp();
        (assign15980_e22712, (assign15980_e22712 * (locals.var_beta * (locals.var_vbcs_cl_dn0 - locals.var_vds_dn0))), (assign15980_e22712 * (locals.var_beta * (locals.var_vbcs_cl_dn2 - locals.var_vds_dn2))), (assign15980_e22712 * (locals.var_beta * (locals.var_vbcs_cl_dn6 - locals.var_vds_dn6))), (assign15980_e22712 * (locals.var_beta * (locals.var_vbcs_cl_dn7 - locals.var_vds_dn7))), (assign15980_e22712 * ((locals.var_beta_dn10 * assign15980_e22710) + (locals.var_beta * (locals.var_vbcs_cl_dn10 - locals.var_vds_dn10)))), (assign15980_e22712 * (locals.var_beta * (locals.var_vbcs_cl_dn11 - locals.var_vds_dn11))), (assign15980_e22712 * (locals.var_beta * (locals.var_vbcs_cl_dn12 - locals.var_vds_dn12))), (assign15980_e22712 * (locals.var_beta * (locals.var_vbcs_cl_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn11, locals.var_exp_bvbsvds_dn12, locals.var_exp_bvbsvds_dn17,)
    }
};
        locals.var_exp_bvbsvds = assign15980_e22714;
        locals.var_exp_bvbsvds_dn0 = assign15980_e22714_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign15980_e22714_d_n2;
        locals.var_exp_bvbsvds_dn6 = assign15980_e22714_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign15980_e22714_d_n7;
        locals.var_exp_bvbsvds_dn10 = assign15980_e22714_d_n10;
        locals.var_exp_bvbsvds_dn11 = assign15980_e22714_d_n11;
        locals.var_exp_bvbsvds_dn12 = assign15980_e22714_d_n12;
        locals.var_exp_bvbsvds_dn17 = assign15980_e22714_d_n17;
        locals.var_exp_bvbsvds_rv = 0.0;

        let assign15990_e22717: f64 = if locals.var_vds <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard478 = assign15990_e22717;
        locals.var_guard478_rv = 0.0;

        let (assign16000_e22726, assign16000_e22726_d_n0, assign16000_e22726_d_n2, assign16000_e22726_d_n6, assign16000_e22726_d_n7, assign16000_e22726_d_n10, assign16000_e22726_d_n11, assign16000_e22726_d_n12, assign16000_e22726_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign16000_e22726;
        locals.var_pds_dn0 = assign16000_e22726_d_n0;
        locals.var_pds_dn2 = assign16000_e22726_d_n2;
        locals.var_pds_dn6 = assign16000_e22726_d_n6;
        locals.var_pds_dn7 = assign16000_e22726_d_n7;
        locals.var_pds_dn10 = assign16000_e22726_d_n10;
        locals.var_pds_dn11 = assign16000_e22726_d_n11;
        locals.var_pds_dn12 = assign16000_e22726_d_n12;
        locals.var_pds_dn17 = assign16000_e22726_d_n17;
        locals.var_pds_rv = 0.0;

        let (assign16010_e22735, assign16010_e22735_d_n0, assign16010_e22735_d_n2, assign16010_e22735_d_n6, assign16010_e22735_d_n7, assign16010_e22735_d_n10, assign16010_e22735_d_n11, assign16010_e22735_d_n12, assign16010_e22735_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign16010_e22735;
        locals.var_psl_dn0 = assign16010_e22735_d_n0;
        locals.var_psl_dn2 = assign16010_e22735_d_n2;
        locals.var_psl_dn6 = assign16010_e22735_d_n6;
        locals.var_psl_dn7 = assign16010_e22735_d_n7;
        locals.var_psl_dn10 = assign16010_e22735_d_n10;
        locals.var_psl_dn11 = assign16010_e22735_d_n11;
        locals.var_psl_dn12 = assign16010_e22735_d_n12;
        locals.var_psl_dn17 = assign16010_e22735_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign16020_e22744,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign16020_e22744;
        locals.var_flg_conv_rv = 0.0;

        let assign16030_e22747: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign16030_e22747;
        locals.var_guard479_rv = 0.0;

        let (assign16040_e22759, assign16040_e22759_d_n0, assign16040_e22759_d_n2, assign16040_e22759_d_n6, assign16040_e22759_d_n7, assign16040_e22759_d_n10, assign16040_e22759_d_n11, assign16040_e22759_d_n12, assign16040_e22759_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard479 != 0.0)) {
        (locals.var_pssl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign16040_e22759;
        locals.var_phi_sl_soi_dn0 = assign16040_e22759_d_n0;
        locals.var_phi_sl_soi_dn2 = assign16040_e22759_d_n2;
        locals.var_phi_sl_soi_dn6 = assign16040_e22759_d_n6;
        locals.var_phi_sl_soi_dn7 = assign16040_e22759_d_n7;
        locals.var_phi_sl_soi_dn10 = assign16040_e22759_d_n10;
        locals.var_phi_sl_soi_dn11 = assign16040_e22759_d_n11;
        locals.var_phi_sl_soi_dn12 = assign16040_e22759_d_n12;
        locals.var_phi_sl_soi_dn17 = assign16040_e22759_d_n17;
        locals.var_phi_sl_soi_rv = 0.0;

        let (assign16050_e22773, assign16050_e22773_d_n0, assign16050_e22773_d_n2, assign16050_e22773_d_n6, assign16050_e22773_d_n7, assign16050_e22773_d_n10, assign16050_e22773_d_n11, assign16050_e22773_d_n12, assign16050_e22773_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard479 != 0.0)) {
        let assign16050_e22771: f64 = (locals.var_pssl_ini - locals.var_ps0);
        (assign16050_e22771, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn10), (-locals.var_ps0_dn11), (-locals.var_ps0_dn12), (-locals.var_ps0_dn17),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16050_e22773;
        locals.var_pds_ini_dn0 = assign16050_e22773_d_n0;
        locals.var_pds_ini_dn2 = assign16050_e22773_d_n2;
        locals.var_pds_ini_dn6 = assign16050_e22773_d_n6;
        locals.var_pds_ini_dn7 = assign16050_e22773_d_n7;
        locals.var_pds_ini_dn10 = assign16050_e22773_d_n10;
        locals.var_pds_ini_dn11 = assign16050_e22773_d_n11;
        locals.var_pds_ini_dn12 = assign16050_e22773_d_n12;
        locals.var_pds_ini_dn17 = assign16050_e22773_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign16060_e22776: f64 = if locals.var_flg_pprv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard480 = assign16060_e22776;
        locals.var_guard480_rv = 0.0;

        let (assign16070_e22797, assign16070_e22797_d_n0, assign16070_e22797_d_n2, assign16070_e22797_d_n6, assign16070_e22797_d_n7, assign16070_e22797_d_n10, assign16070_e22797_d_n11, assign16070_e22797_d_n12, assign16070_e22797_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign16070_e22788: f64 = (locals.var_psl_lim - locals.var_ps0);
        let (assign16070_e22795, assign16070_e22795_d_n0, assign16070_e22795_d_n2, assign16070_e22795_d_n6, assign16070_e22795_d_n7, assign16070_e22795_d_n10, assign16070_e22795_d_n11, assign16070_e22795_d_n12, assign16070_e22795_d_n17,) = {
            if (assign16070_e22788 >= 0.0) {
                let assign16070_e22793: f64 = (locals.var_psl_lim - locals.var_ps0);
                (assign16070_e22793, (locals.var_psl_lim_dn0 - locals.var_ps0_dn0), (locals.var_psl_lim_dn2 - locals.var_ps0_dn2), (locals.var_psl_lim_dn6 - locals.var_ps0_dn6), (locals.var_psl_lim_dn7 - locals.var_ps0_dn7), (locals.var_psl_lim_dn10 - locals.var_ps0_dn10), (locals.var_psl_lim_dn11 - locals.var_ps0_dn11), (locals.var_psl_lim_dn12 - locals.var_ps0_dn12), (locals.var_psl_lim_dn17 - locals.var_ps0_dn17),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign16070_e22795, assign16070_e22795_d_n0, assign16070_e22795_d_n2, assign16070_e22795_d_n6, assign16070_e22795_d_n7, assign16070_e22795_d_n10, assign16070_e22795_d_n11, assign16070_e22795_d_n12, assign16070_e22795_d_n17,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign16070_e22797;
        locals.var_pds_max_dn0 = assign16070_e22797_d_n0;
        locals.var_pds_max_dn2 = assign16070_e22797_d_n2;
        locals.var_pds_max_dn6 = assign16070_e22797_d_n6;
        locals.var_pds_max_dn7 = assign16070_e22797_d_n7;
        locals.var_pds_max_dn10 = assign16070_e22797_d_n10;
        locals.var_pds_max_dn11 = assign16070_e22797_d_n11;
        locals.var_pds_max_dn12 = assign16070_e22797_d_n12;
        locals.var_pds_max_dn17 = assign16070_e22797_d_n17;
        locals.var_pds_max_rv = 0.0;

        let (assign16080_e22817, assign16080_e22817_d_n0, assign16080_e22817_d_n2, assign16080_e22817_d_n6, assign16080_e22817_d_n7, assign16080_e22817_d_n10, assign16080_e22817_d_n11, assign16080_e22817_d_n12, assign16080_e22817_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign16080_e22809: f64 = (1.0 + 0.3);
        let assign16080_e22811: f64 = (assign16080_e22809 * locals.var_pds_max);
        let assign16080_e22813: f64 = (assign16080_e22811 - locals.var_vds);
        let assign16080_e22815: f64 = (assign16080_e22813 - 0.03);
        (assign16080_e22815, ((assign16080_e22809 * locals.var_pds_max_dn0) - locals.var_vds_dn0), ((assign16080_e22809 * locals.var_pds_max_dn2) - locals.var_vds_dn2), ((assign16080_e22809 * locals.var_pds_max_dn6) - locals.var_vds_dn6), ((assign16080_e22809 * locals.var_pds_max_dn7) - locals.var_vds_dn7), ((assign16080_e22809 * locals.var_pds_max_dn10) - locals.var_vds_dn10), ((assign16080_e22809 * locals.var_pds_max_dn11) - locals.var_vds_dn11), ((assign16080_e22809 * locals.var_pds_max_dn12) - locals.var_vds_dn12), ((assign16080_e22809 * locals.var_pds_max_dn17) - locals.var_vds_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign16080_e22817;
        locals.var_tmf1_dn0 = assign16080_e22817_d_n0;
        locals.var_tmf1_dn2 = assign16080_e22817_d_n2;
        locals.var_tmf1_dn6 = assign16080_e22817_d_n6;
        locals.var_tmf1_dn7 = assign16080_e22817_d_n7;
        locals.var_tmf1_dn10 = assign16080_e22817_d_n10;
        locals.var_tmf1_dn11 = assign16080_e22817_d_n11;
        locals.var_tmf1_dn12 = assign16080_e22817_d_n12;
        locals.var_tmf1_dn17 = assign16080_e22817_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign16090_e22837, assign16090_e22837_d_n0, assign16090_e22837_d_n2, assign16090_e22837_d_n6, assign16090_e22837_d_n7, assign16090_e22837_d_n10, assign16090_e22837_d_n11, assign16090_e22837_d_n12, assign16090_e22837_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign16090_e22830: f64 = (1.0 + 0.3);
        let assign16090_e22832: f64 = (assign16090_e22830 * locals.var_pds_max);
        let assign16090_e22833: f64 = (4.0 * assign16090_e22832);
        let assign16090_e22835: f64 = (assign16090_e22833 * 0.03);
        (assign16090_e22835, ((4.0 * (assign16090_e22830 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign16090_e22830 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign16090_e22830 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign16090_e22830 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign16090_e22830 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign16090_e22830 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign16090_e22830 * locals.var_pds_max_dn12)) * 0.03), ((4.0 * (assign16090_e22830 * locals.var_pds_max_dn17)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16090_e22837;
        locals.var_tmf2_dn0 = assign16090_e22837_d_n0;
        locals.var_tmf2_dn2 = assign16090_e22837_d_n2;
        locals.var_tmf2_dn6 = assign16090_e22837_d_n6;
        locals.var_tmf2_dn7 = assign16090_e22837_d_n7;
        locals.var_tmf2_dn10 = assign16090_e22837_d_n10;
        locals.var_tmf2_dn11 = assign16090_e22837_d_n11;
        locals.var_tmf2_dn12 = assign16090_e22837_d_n12;
        locals.var_tmf2_dn17 = assign16090_e22837_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign16100_e22855, assign16100_e22855_d_n0, assign16100_e22855_d_n2, assign16100_e22855_d_n6, assign16100_e22855_d_n7, assign16100_e22855_d_n10, assign16100_e22855_d_n11, assign16100_e22855_d_n12, assign16100_e22855_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard480 != 0.0)) {
        let (assign16100_e22853, assign16100_e22853_d_n0, assign16100_e22853_d_n2, assign16100_e22853_d_n6, assign16100_e22853_d_n7, assign16100_e22853_d_n10, assign16100_e22853_d_n11, assign16100_e22853_d_n12, assign16100_e22853_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign16100_e22852: f64 = (-locals.var_tmf2);
                (assign16100_e22852, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign16100_e22853, assign16100_e22853_d_n0, assign16100_e22853_d_n2, assign16100_e22853_d_n6, assign16100_e22853_d_n7, assign16100_e22853_d_n10, assign16100_e22853_d_n11, assign16100_e22853_d_n12, assign16100_e22853_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16100_e22855;
        locals.var_tmf2_dn0 = assign16100_e22855_d_n0;
        locals.var_tmf2_dn2 = assign16100_e22855_d_n2;
        locals.var_tmf2_dn6 = assign16100_e22855_d_n6;
        locals.var_tmf2_dn7 = assign16100_e22855_d_n7;
        locals.var_tmf2_dn10 = assign16100_e22855_d_n10;
        locals.var_tmf2_dn11 = assign16100_e22855_d_n11;
        locals.var_tmf2_dn12 = assign16100_e22855_d_n12;
        locals.var_tmf2_dn17 = assign16100_e22855_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign16110_e22872, assign16110_e22872_d_n0, assign16110_e22872_d_n2, assign16110_e22872_d_n6, assign16110_e22872_d_n7, assign16110_e22872_d_n10, assign16110_e22872_d_n11, assign16110_e22872_d_n12, assign16110_e22872_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign16110_e22867: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign16110_e22869: f64 = (assign16110_e22867 + locals.var_tmf2);
        let assign16110_e22870: f64 = (assign16110_e22869).sqrt();
        (assign16110_e22870, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign16110_e22870)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign16110_e22870)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign16110_e22870)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign16110_e22870)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign16110_e22870)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign16110_e22870)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign16110_e22870)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign16110_e22870)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16110_e22872;
        locals.var_tmf2_dn0 = assign16110_e22872_d_n0;
        locals.var_tmf2_dn2 = assign16110_e22872_d_n2;
        locals.var_tmf2_dn6 = assign16110_e22872_d_n6;
        locals.var_tmf2_dn7 = assign16110_e22872_d_n7;
        locals.var_tmf2_dn10 = assign16110_e22872_d_n10;
        locals.var_tmf2_dn11 = assign16110_e22872_d_n11;
        locals.var_tmf2_dn12 = assign16110_e22872_d_n12;
        locals.var_tmf2_dn17 = assign16110_e22872_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign16120_e22894, assign16120_e22894_d_n0, assign16120_e22894_d_n2, assign16120_e22894_d_n6, assign16120_e22894_d_n7, assign16120_e22894_d_n10, assign16120_e22894_d_n11, assign16120_e22894_d_n12, assign16120_e22894_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign16120_e22884: f64 = (1.0 + 0.3);
        let assign16120_e22886: f64 = (assign16120_e22884 * locals.var_pds_max);
        let assign16120_e22890: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign16120_e22891: f64 = (0.5 * assign16120_e22890);
        let assign16120_e22892: f64 = (assign16120_e22886 - assign16120_e22891);
        (assign16120_e22892, ((assign16120_e22884 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign16120_e22884 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign16120_e22884 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign16120_e22884 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign16120_e22884 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign16120_e22884 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign16120_e22884 * locals.var_pds_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((assign16120_e22884 * locals.var_pds_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16120_e22894;
        locals.var_pds_ini_dn0 = assign16120_e22894_d_n0;
        locals.var_pds_ini_dn2 = assign16120_e22894_d_n2;
        locals.var_pds_ini_dn6 = assign16120_e22894_d_n6;
        locals.var_pds_ini_dn7 = assign16120_e22894_d_n7;
        locals.var_pds_ini_dn10 = assign16120_e22894_d_n10;
        locals.var_pds_ini_dn11 = assign16120_e22894_d_n11;
        locals.var_pds_ini_dn12 = assign16120_e22894_d_n12;
        locals.var_pds_ini_dn17 = assign16120_e22894_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let (assign16130_e22911, assign16130_e22911_d_n0, assign16130_e22911_d_n2, assign16130_e22911_d_n6, assign16130_e22911_d_n7, assign16130_e22911_d_n10, assign16130_e22911_d_n11, assign16130_e22911_d_n12, assign16130_e22911_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard480 != 0.0)) {
        let (assign16130_e22909, assign16130_e22909_d_n0, assign16130_e22909_d_n2, assign16130_e22909_d_n6, assign16130_e22909_d_n7, assign16130_e22909_d_n10, assign16130_e22909_d_n11, assign16130_e22909_d_n12, assign16130_e22909_d_n17,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
            }
        };
        (assign16130_e22909, assign16130_e22909_d_n0, assign16130_e22909_d_n2, assign16130_e22909_d_n6, assign16130_e22909_d_n7, assign16130_e22909_d_n10, assign16130_e22909_d_n11, assign16130_e22909_d_n12, assign16130_e22909_d_n17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16130_e22911;
        locals.var_pds_ini_dn0 = assign16130_e22911_d_n0;
        locals.var_pds_ini_dn2 = assign16130_e22911_d_n2;
        locals.var_pds_ini_dn6 = assign16130_e22911_d_n6;
        locals.var_pds_ini_dn7 = assign16130_e22911_d_n7;
        locals.var_pds_ini_dn10 = assign16130_e22911_d_n10;
        locals.var_pds_ini_dn11 = assign16130_e22911_d_n11;
        locals.var_pds_ini_dn12 = assign16130_e22911_d_n12;
        locals.var_pds_ini_dn17 = assign16130_e22911_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign16140_e22914: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard481 = assign16140_e22914;
        locals.var_guard481_rv = 0.0;

        let (assign16150_e22926, assign16150_e22926_d_n0, assign16150_e22926_d_n2, assign16150_e22926_d_n6, assign16150_e22926_d_n7, assign16150_e22926_d_n10, assign16150_e22926_d_n11, assign16150_e22926_d_n12, assign16150_e22926_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard481 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16150_e22926;
        locals.var_pds_ini_dn0 = assign16150_e22926_d_n0;
        locals.var_pds_ini_dn2 = assign16150_e22926_d_n2;
        locals.var_pds_ini_dn6 = assign16150_e22926_d_n6;
        locals.var_pds_ini_dn7 = assign16150_e22926_d_n7;
        locals.var_pds_ini_dn10 = assign16150_e22926_d_n10;
        locals.var_pds_ini_dn11 = assign16150_e22926_d_n11;
        locals.var_pds_ini_dn12 = assign16150_e22926_d_n12;
        locals.var_pds_ini_dn17 = assign16150_e22926_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign16160_e22929: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard482 = assign16160_e22929;
        locals.var_guard482_rv = 0.0;

        let (assign16170_e22944, assign16170_e22944_d_n0, assign16170_e22944_d_n2, assign16170_e22944_d_n6, assign16170_e22944_d_n7, assign16170_e22944_d_n10, assign16170_e22944_d_n11, assign16170_e22944_d_n12, assign16170_e22944_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard481 == 0.0)) && (locals.var_guard482 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16170_e22944;
        locals.var_pds_ini_dn0 = assign16170_e22944_d_n0;
        locals.var_pds_ini_dn2 = assign16170_e22944_d_n2;
        locals.var_pds_ini_dn6 = assign16170_e22944_d_n6;
        locals.var_pds_ini_dn7 = assign16170_e22944_d_n7;
        locals.var_pds_ini_dn10 = assign16170_e22944_d_n10;
        locals.var_pds_ini_dn11 = assign16170_e22944_d_n11;
        locals.var_pds_ini_dn12 = assign16170_e22944_d_n12;
        locals.var_pds_ini_dn17 = assign16170_e22944_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let (assign16180_e22954, assign16180_e22954_d_n0, assign16180_e22954_d_n2, assign16180_e22954_d_n6, assign16180_e22954_d_n7, assign16180_e22954_d_n10, assign16180_e22954_d_n11, assign16180_e22954_d_n12, assign16180_e22954_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign16180_e22954;
        locals.var_pds_dn0 = assign16180_e22954_d_n0;
        locals.var_pds_dn2 = assign16180_e22954_d_n2;
        locals.var_pds_dn6 = assign16180_e22954_d_n6;
        locals.var_pds_dn7 = assign16180_e22954_d_n7;
        locals.var_pds_dn10 = assign16180_e22954_d_n10;
        locals.var_pds_dn11 = assign16180_e22954_d_n11;
        locals.var_pds_dn12 = assign16180_e22954_d_n12;
        locals.var_pds_dn17 = assign16180_e22954_d_n17;
        locals.var_pds_rv = 0.0;

        let (assign16190_e22966, assign16190_e22966_d_n0, assign16190_e22966_d_n2, assign16190_e22966_d_n6, assign16190_e22966_d_n7, assign16190_e22966_d_n10, assign16190_e22966_d_n11, assign16190_e22966_d_n12, assign16190_e22966_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) {
        let assign16190_e22964: f64 = (locals.var_ps0 + locals.var_pds);
        (assign16190_e22964, (locals.var_ps0_dn0 + locals.var_pds_dn0), (locals.var_ps0_dn2 + locals.var_pds_dn2), (locals.var_ps0_dn6 + locals.var_pds_dn6), (locals.var_ps0_dn7 + locals.var_pds_dn7), (locals.var_ps0_dn10 + locals.var_pds_dn10), (locals.var_ps0_dn11 + locals.var_pds_dn11), (locals.var_ps0_dn12 + locals.var_pds_dn12), (locals.var_ps0_dn17 + locals.var_pds_dn17),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign16190_e22966;
        locals.var_psl_dn0 = assign16190_e22966_d_n0;
        locals.var_psl_dn2 = assign16190_e22966_d_n2;
        locals.var_psl_dn6 = assign16190_e22966_d_n6;
        locals.var_psl_dn7 = assign16190_e22966_d_n7;
        locals.var_psl_dn10 = assign16190_e22966_d_n10;
        locals.var_psl_dn11 = assign16190_e22966_d_n11;
        locals.var_psl_dn12 = assign16190_e22966_d_n12;
        locals.var_psl_dn17 = assign16190_e22966_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign16200_e22976,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard478 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign16200_e22976;
        locals.var_flg_conv_rv = 0.0;

        let (assign16210_e22983, assign16210_e22983_d_n0, assign16210_e22983_d_n2, assign16210_e22983_d_n6, assign16210_e22983_d_n7, assign16210_e22983_d_n10, assign16210_e22983_d_n11, assign16210_e22983_d_n12, assign16210_e22983_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign16210_e22983;
        locals.var_phi_sl_soi_dn0 = assign16210_e22983_d_n0;
        locals.var_phi_sl_soi_dn2 = assign16210_e22983_d_n2;
        locals.var_phi_sl_soi_dn6 = assign16210_e22983_d_n6;
        locals.var_phi_sl_soi_dn7 = assign16210_e22983_d_n7;
        locals.var_phi_sl_soi_dn10 = assign16210_e22983_d_n10;
        locals.var_phi_sl_soi_dn11 = assign16210_e22983_d_n11;
        locals.var_phi_sl_soi_dn12 = assign16210_e22983_d_n12;
        locals.var_phi_sl_soi_dn17 = assign16210_e22983_d_n17;
        locals.var_phi_sl_soi_rv = 0.0;

        let (assign16220_e22990,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign16220_e22990;
        locals.var_lp_sl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_57(
        locals: &mut StampLocals,
    ) {
        let mut assign16230_loop_guard: usize = 0;
        while {
            let assign16230_cond_e22998: f64 = (locals.var_lp_sl_max + 1.0);
            let assign16230_cond_e23000: f64 = if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_lp_sl <= assign16230_cond_e22998)) { 1.0 } else { 0.0 };
            assign16230_cond_e23000 != 0.0
        } {
            assign16230_loop_guard += 1;
            assert!(assign16230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign16230_body0_e23009, assign16230_body0_e23009_d_n0, assign16230_body0_e23009_d_n2, assign16230_body0_e23009_d_n6, assign16230_body0_e23009_d_n7, assign16230_body0_e23009_d_n10, assign16230_body0_e23009_d_n11, assign16230_body0_e23009_d_n12, assign16230_body0_e23009_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16230_body0_e23007: f64 = (locals.var_phi_sl_soi - locals.var_vbcs_cl);
        (assign16230_body0_e23007, (locals.var_phi_sl_soi_dn0 - locals.var_vbcs_cl_dn0), (locals.var_phi_sl_soi_dn2 - locals.var_vbcs_cl_dn2), (locals.var_phi_sl_soi_dn6 - locals.var_vbcs_cl_dn6), (locals.var_phi_sl_soi_dn7 - locals.var_vbcs_cl_dn7), (locals.var_phi_sl_soi_dn10 - locals.var_vbcs_cl_dn10), (locals.var_phi_sl_soi_dn11 - locals.var_vbcs_cl_dn11), (locals.var_phi_sl_soi_dn12 - locals.var_vbcs_cl_dn12), (locals.var_phi_sl_soi_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_phi_soil, locals.var_phi_soil_dn0, locals.var_phi_soil_dn2, locals.var_phi_soil_dn6, locals.var_phi_soil_dn7, locals.var_phi_soil_dn10, locals.var_phi_soil_dn11, locals.var_phi_soil_dn12, locals.var_phi_soil_dn17,)
    }
};
            locals.var_phi_soil = assign16230_body0_e23009;
            locals.var_phi_soil_dn0 = assign16230_body0_e23009_d_n0;
            locals.var_phi_soil_dn2 = assign16230_body0_e23009_d_n2;
            locals.var_phi_soil_dn6 = assign16230_body0_e23009_d_n6;
            locals.var_phi_soil_dn7 = assign16230_body0_e23009_d_n7;
            locals.var_phi_soil_dn10 = assign16230_body0_e23009_d_n10;
            locals.var_phi_soil_dn11 = assign16230_body0_e23009_d_n11;
            locals.var_phi_soil_dn12 = assign16230_body0_e23009_d_n12;
            locals.var_phi_soil_dn17 = assign16230_body0_e23009_d_n17;
            locals.var_phi_soil_rv = 0.0;
            let (assign16230_body1_e23018, assign16230_body1_e23018_d_n0, assign16230_body1_e23018_d_n2, assign16230_body1_e23018_d_n6, assign16230_body1_e23018_d_n7, assign16230_body1_e23018_d_n10, assign16230_body1_e23018_d_n11, assign16230_body1_e23018_d_n12, assign16230_body1_e23018_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16230_body1_e23016: f64 = (locals.var_beta * locals.var_phi_soil);
        (assign16230_body1_e23016, (locals.var_beta * locals.var_phi_soil_dn0), (locals.var_beta * locals.var_phi_soil_dn2), (locals.var_beta * locals.var_phi_soil_dn6), (locals.var_beta * locals.var_phi_soil_dn7), ((locals.var_beta_dn10 * locals.var_phi_soil) + (locals.var_beta * locals.var_phi_soil_dn10)), (locals.var_beta * locals.var_phi_soil_dn11), (locals.var_beta * locals.var_phi_soil_dn12), (locals.var_beta * locals.var_phi_soil_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
            locals.var_chi = assign16230_body1_e23018;
            locals.var_chi_dn0 = assign16230_body1_e23018_d_n0;
            locals.var_chi_dn2 = assign16230_body1_e23018_d_n2;
            locals.var_chi_dn6 = assign16230_body1_e23018_d_n6;
            locals.var_chi_dn7 = assign16230_body1_e23018_d_n7;
            locals.var_chi_dn10 = assign16230_body1_e23018_d_n10;
            locals.var_chi_dn11 = assign16230_body1_e23018_d_n11;
            locals.var_chi_dn12 = assign16230_body1_e23018_d_n12;
            locals.var_chi_dn17 = assign16230_body1_e23018_d_n17;
            locals.var_chi_rv = 0.0;
            let (assign16230_body2_e23029, assign16230_body2_e23029_d_n0, assign16230_body2_e23029_d_n2, assign16230_body2_e23029_d_n6, assign16230_body2_e23029_d_n7, assign16230_body2_e23029_d_n10, assign16230_body2_e23029_d_n11, assign16230_body2_e23029_d_n12, assign16230_body2_e23029_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16230_body2_e23026: f64 = (locals.var_phi_soil - locals.var_dphi_sb);
        let assign16230_body2_e23027: f64 = (locals.var_c_sb * assign16230_body2_e23026);
        (assign16230_body2_e23027, ((locals.var_c_sb_dn0 * assign16230_body2_e23026) + (locals.var_c_sb * (locals.var_phi_soil_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign16230_body2_e23026) + (locals.var_c_sb * (locals.var_phi_soil_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn6 * assign16230_body2_e23026) + (locals.var_c_sb * (locals.var_phi_soil_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign16230_body2_e23026) + (locals.var_c_sb * (locals.var_phi_soil_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn10 * assign16230_body2_e23026) + (locals.var_c_sb * (locals.var_phi_soil_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign16230_body2_e23026) + (locals.var_c_sb * (locals.var_phi_soil_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn12 * assign16230_body2_e23026) + (locals.var_c_sb * (locals.var_phi_soil_dn12 - locals.var_dphi_sb_dn12))), ((locals.var_c_sb_dn17 * assign16230_body2_e23026) + (locals.var_c_sb * (locals.var_phi_soil_dn17 - locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
            locals.var_ty = assign16230_body2_e23029;
            locals.var_ty_dn0 = assign16230_body2_e23029_d_n0;
            locals.var_ty_dn2 = assign16230_body2_e23029_d_n2;
            locals.var_ty_dn6 = assign16230_body2_e23029_d_n6;
            locals.var_ty_dn7 = assign16230_body2_e23029_d_n7;
            locals.var_ty_dn10 = assign16230_body2_e23029_d_n10;
            locals.var_ty_dn11 = assign16230_body2_e23029_d_n11;
            locals.var_ty_dn12 = assign16230_body2_e23029_d_n12;
            locals.var_ty_dn17 = assign16230_body2_e23029_d_n17;
            locals.var_ty_rv = 0.0;
            let assign16230_body3_e23032: f64 = if locals.var_ty < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard483 = assign16230_body3_e23032;
            locals.var_guard483_rv = 0.0;
            let (assign16230_body4_e23042, assign16230_body4_e23042_d_n0, assign16230_body4_e23042_d_n2, assign16230_body4_e23042_d_n6, assign16230_body4_e23042_d_n7, assign16230_body4_e23042_d_n10, assign16230_body4_e23042_d_n11, assign16230_body4_e23042_d_n12, assign16230_body4_e23042_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign16230_body4_e23040: f64 = (locals.var_ty).exp();
        (assign16230_body4_e23040, (assign16230_body4_e23040 * locals.var_ty_dn0), (assign16230_body4_e23040 * locals.var_ty_dn2), (assign16230_body4_e23040 * locals.var_ty_dn6), (assign16230_body4_e23040 * locals.var_ty_dn7), (assign16230_body4_e23040 * locals.var_ty_dn10), (assign16230_body4_e23040 * locals.var_ty_dn11), (assign16230_body4_e23040 * locals.var_ty_dn12), (assign16230_body4_e23040 * locals.var_ty_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16230_body4_e23042;
            locals.var_t1_dn0 = assign16230_body4_e23042_d_n0;
            locals.var_t1_dn2 = assign16230_body4_e23042_d_n2;
            locals.var_t1_dn6 = assign16230_body4_e23042_d_n6;
            locals.var_t1_dn7 = assign16230_body4_e23042_d_n7;
            locals.var_t1_dn10 = assign16230_body4_e23042_d_n10;
            locals.var_t1_dn11 = assign16230_body4_e23042_d_n11;
            locals.var_t1_dn12 = assign16230_body4_e23042_d_n12;
            locals.var_t1_dn17 = assign16230_body4_e23042_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign16230_body5_e23055, assign16230_body5_e23055_d_n0, assign16230_body5_e23055_d_n2, assign16230_body5_e23055_d_n6, assign16230_body5_e23055_d_n7, assign16230_body5_e23055_d_n10, assign16230_body5_e23055_d_n11, assign16230_body5_e23055_d_n12, assign16230_body5_e23055_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign16230_body5_e23050: f64 = (-locals.var_c_sb);
        let assign16230_body5_e23052: f64 = (assign16230_body5_e23050 * locals.var_dphi_sb);
        let assign16230_body5_e23053: f64 = (assign16230_body5_e23052).exp();
        (assign16230_body5_e23053, (assign16230_body5_e23053 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign16230_body5_e23050 * locals.var_dphi_sb_dn0))), (assign16230_body5_e23053 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign16230_body5_e23050 * locals.var_dphi_sb_dn2))), (assign16230_body5_e23053 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign16230_body5_e23050 * locals.var_dphi_sb_dn6))), (assign16230_body5_e23053 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign16230_body5_e23050 * locals.var_dphi_sb_dn7))), (assign16230_body5_e23053 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign16230_body5_e23050 * locals.var_dphi_sb_dn10))), (assign16230_body5_e23053 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign16230_body5_e23050 * locals.var_dphi_sb_dn11))), (assign16230_body5_e23053 * (((-locals.var_c_sb_dn12) * locals.var_dphi_sb) + (assign16230_body5_e23050 * locals.var_dphi_sb_dn12))), (assign16230_body5_e23053 * (((-locals.var_c_sb_dn17) * locals.var_dphi_sb) + (assign16230_body5_e23050 * locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16230_body5_e23055;
            locals.var_t0_dn0 = assign16230_body5_e23055_d_n0;
            locals.var_t0_dn2 = assign16230_body5_e23055_d_n2;
            locals.var_t0_dn6 = assign16230_body5_e23055_d_n6;
            locals.var_t0_dn7 = assign16230_body5_e23055_d_n7;
            locals.var_t0_dn10 = assign16230_body5_e23055_d_n10;
            locals.var_t0_dn11 = assign16230_body5_e23055_d_n11;
            locals.var_t0_dn12 = assign16230_body5_e23055_d_n12;
            locals.var_t0_dn17 = assign16230_body5_e23055_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16230_body6_e23066, assign16230_body6_e23066_d_n0, assign16230_body6_e23066_d_n2, assign16230_body6_e23066_d_n6, assign16230_body6_e23066_d_n7, assign16230_body6_e23066_d_n10, assign16230_body6_e23066_d_n11, assign16230_body6_e23066_d_n12, assign16230_body6_e23066_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign16230_body6_e23064: f64 = (locals.var_t1 - locals.var_t0);
        (assign16230_body6_e23064, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn12 - locals.var_t0_dn12), (locals.var_t1_dn17 - locals.var_t0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign16230_body6_e23066;
            locals.var_t2_dn0 = assign16230_body6_e23066_d_n0;
            locals.var_t2_dn2 = assign16230_body6_e23066_d_n2;
            locals.var_t2_dn6 = assign16230_body6_e23066_d_n6;
            locals.var_t2_dn7 = assign16230_body6_e23066_d_n7;
            locals.var_t2_dn10 = assign16230_body6_e23066_d_n10;
            locals.var_t2_dn11 = assign16230_body6_e23066_d_n11;
            locals.var_t2_dn12 = assign16230_body6_e23066_d_n12;
            locals.var_t2_dn17 = assign16230_body6_e23066_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign16230_body7_e23080, assign16230_body7_e23080_d_n0, assign16230_body7_e23080_d_n2, assign16230_body7_e23080_d_n6, assign16230_body7_e23080_d_n7, assign16230_body7_e23080_d_n10, assign16230_body7_e23080_d_n11, assign16230_body7_e23080_d_n12, assign16230_body7_e23080_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign16230_body7_e23075: f64 = (1.0 + locals.var_t2);
        let assign16230_body7_e23076: f64 = (assign16230_body7_e23075).ln();
        let assign16230_body7_e23078: f64 = (assign16230_body7_e23076 / locals.var_c_sb);
        (assign16230_body7_e23078, ((((locals.var_t2_dn0 / assign16230_body7_e23075) * locals.var_c_sb) - (assign16230_body7_e23076 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign16230_body7_e23075) * locals.var_c_sb) - (assign16230_body7_e23076 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign16230_body7_e23075) * locals.var_c_sb) - (assign16230_body7_e23076 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign16230_body7_e23075) * locals.var_c_sb) - (assign16230_body7_e23076 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign16230_body7_e23075) * locals.var_c_sb) - (assign16230_body7_e23076 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign16230_body7_e23075) * locals.var_c_sb) - (assign16230_body7_e23076 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn12 / assign16230_body7_e23075) * locals.var_c_sb) - (assign16230_body7_e23076 * locals.var_c_sb_dn12)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn17 / assign16230_body7_e23075) * locals.var_c_sb) - (assign16230_body7_e23076 * locals.var_c_sb_dn17)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign16230_body7_e23080;
            locals.var_phi_soib_dn0 = assign16230_body7_e23080_d_n0;
            locals.var_phi_soib_dn2 = assign16230_body7_e23080_d_n2;
            locals.var_phi_soib_dn6 = assign16230_body7_e23080_d_n6;
            locals.var_phi_soib_dn7 = assign16230_body7_e23080_d_n7;
            locals.var_phi_soib_dn10 = assign16230_body7_e23080_d_n10;
            locals.var_phi_soib_dn11 = assign16230_body7_e23080_d_n11;
            locals.var_phi_soib_dn12 = assign16230_body7_e23080_d_n12;
            locals.var_phi_soib_dn17 = assign16230_body7_e23080_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign16230_body8_e23093, assign16230_body8_e23093_d_n0, assign16230_body8_e23093_d_n2, assign16230_body8_e23093_d_n6, assign16230_body8_e23093_d_n7, assign16230_body8_e23093_d_n10, assign16230_body8_e23093_d_n11, assign16230_body8_e23093_d_n12, assign16230_body8_e23093_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign16230_body8_e23090: f64 = (1.0 + locals.var_t2);
        let assign16230_body8_e23091: f64 = (locals.var_t1 / assign16230_body8_e23090);
        (assign16230_body8_e23091, (((locals.var_t1_dn0 * assign16230_body8_e23090) - (locals.var_t1 * locals.var_t2_dn0)) / (assign16230_body8_e23090 * assign16230_body8_e23090)), (((locals.var_t1_dn2 * assign16230_body8_e23090) - (locals.var_t1 * locals.var_t2_dn2)) / (assign16230_body8_e23090 * assign16230_body8_e23090)), (((locals.var_t1_dn6 * assign16230_body8_e23090) - (locals.var_t1 * locals.var_t2_dn6)) / (assign16230_body8_e23090 * assign16230_body8_e23090)), (((locals.var_t1_dn7 * assign16230_body8_e23090) - (locals.var_t1 * locals.var_t2_dn7)) / (assign16230_body8_e23090 * assign16230_body8_e23090)), (((locals.var_t1_dn10 * assign16230_body8_e23090) - (locals.var_t1 * locals.var_t2_dn10)) / (assign16230_body8_e23090 * assign16230_body8_e23090)), (((locals.var_t1_dn11 * assign16230_body8_e23090) - (locals.var_t1 * locals.var_t2_dn11)) / (assign16230_body8_e23090 * assign16230_body8_e23090)), (((locals.var_t1_dn12 * assign16230_body8_e23090) - (locals.var_t1 * locals.var_t2_dn12)) / (assign16230_body8_e23090 * assign16230_body8_e23090)), (((locals.var_t1_dn17 * assign16230_body8_e23090) - (locals.var_t1 * locals.var_t2_dn17)) / (assign16230_body8_e23090 * assign16230_body8_e23090)),)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign16230_body8_e23093;
            locals.var_phi_soib_dpss_dn0 = assign16230_body8_e23093_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign16230_body8_e23093_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign16230_body8_e23093_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign16230_body8_e23093_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign16230_body8_e23093_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign16230_body8_e23093_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign16230_body8_e23093_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign16230_body8_e23093_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign16230_body9_e23105, assign16230_body9_e23105_d_n0, assign16230_body9_e23105_d_n2, assign16230_body9_e23105_d_n6, assign16230_body9_e23105_d_n7, assign16230_body9_e23105_d_n10, assign16230_body9_e23105_d_n11, assign16230_body9_e23105_d_n12, assign16230_body9_e23105_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard483 == 0.0)) {
        let assign16230_body9_e23103: f64 = (locals.var_phi_soil - locals.var_dphi_sb);
        (assign16230_body9_e23103, (locals.var_phi_soil_dn0 - locals.var_dphi_sb_dn0), (locals.var_phi_soil_dn2 - locals.var_dphi_sb_dn2), (locals.var_phi_soil_dn6 - locals.var_dphi_sb_dn6), (locals.var_phi_soil_dn7 - locals.var_dphi_sb_dn7), (locals.var_phi_soil_dn10 - locals.var_dphi_sb_dn10), (locals.var_phi_soil_dn11 - locals.var_dphi_sb_dn11), (locals.var_phi_soil_dn12 - locals.var_dphi_sb_dn12), (locals.var_phi_soil_dn17 - locals.var_dphi_sb_dn17),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign16230_body9_e23105;
            locals.var_phi_soib_dn0 = assign16230_body9_e23105_d_n0;
            locals.var_phi_soib_dn2 = assign16230_body9_e23105_d_n2;
            locals.var_phi_soib_dn6 = assign16230_body9_e23105_d_n6;
            locals.var_phi_soib_dn7 = assign16230_body9_e23105_d_n7;
            locals.var_phi_soib_dn10 = assign16230_body9_e23105_d_n10;
            locals.var_phi_soib_dn11 = assign16230_body9_e23105_d_n11;
            locals.var_phi_soib_dn12 = assign16230_body9_e23105_d_n12;
            locals.var_phi_soib_dn17 = assign16230_body9_e23105_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign16230_body10_e23115, assign16230_body10_e23115_d_n0, assign16230_body10_e23115_d_n2, assign16230_body10_e23115_d_n6, assign16230_body10_e23115_d_n7, assign16230_body10_e23115_d_n10, assign16230_body10_e23115_d_n11, assign16230_body10_e23115_d_n12, assign16230_body10_e23115_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard483 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign16230_body10_e23115;
            locals.var_phi_soib_dpss_dn0 = assign16230_body10_e23115_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign16230_body10_e23115_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign16230_body10_e23115_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign16230_body10_e23115_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign16230_body10_e23115_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign16230_body10_e23115_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign16230_body10_e23115_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign16230_body10_e23115_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign16230_body11_e23124, assign16230_body11_e23124_d_n0, assign16230_body11_e23124_d_n2, assign16230_body11_e23124_d_n6, assign16230_body11_e23124_d_n7, assign16230_body11_e23124_d_n10, assign16230_body11_e23124_d_n11, assign16230_body11_e23124_d_n12, assign16230_body11_e23124_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16230_body11_e23122: f64 = (locals.var_beta * locals.var_phi_soib);
        (assign16230_body11_e23122, (locals.var_beta * locals.var_phi_soib_dn0), (locals.var_beta * locals.var_phi_soib_dn2), (locals.var_beta * locals.var_phi_soib_dn6), (locals.var_beta * locals.var_phi_soib_dn7), ((locals.var_beta_dn10 * locals.var_phi_soib) + (locals.var_beta * locals.var_phi_soib_dn10)), (locals.var_beta * locals.var_phi_soib_dn11), (locals.var_beta * locals.var_phi_soib_dn12), (locals.var_beta * locals.var_phi_soib_dn17),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn12, locals.var_chib_dn17,)
    }
};
            locals.var_chib = assign16230_body11_e23124;
            locals.var_chib_dn0 = assign16230_body11_e23124_d_n0;
            locals.var_chib_dn2 = assign16230_body11_e23124_d_n2;
            locals.var_chib_dn6 = assign16230_body11_e23124_d_n6;
            locals.var_chib_dn7 = assign16230_body11_e23124_d_n7;
            locals.var_chib_dn10 = assign16230_body11_e23124_d_n10;
            locals.var_chib_dn11 = assign16230_body11_e23124_d_n11;
            locals.var_chib_dn12 = assign16230_body11_e23124_d_n12;
            locals.var_chib_dn17 = assign16230_body11_e23124_d_n17;
            locals.var_chib_rv = 0.0;
            let assign16230_body12_e23126: f64 = (locals.var_chi).abs();
            let assign16230_body12_e23128: f64 = if assign16230_body12_e23126 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard484 = assign16230_body12_e23128;
            locals.var_guard484_rv = 0.0;
            let (assign16230_body13_e23144, assign16230_body13_e23144_d_n0, assign16230_body13_e23144_d_n2, assign16230_body13_e23144_d_n6, assign16230_body13_e23144_d_n7, assign16230_body13_e23144_d_n10, assign16230_body13_e23144_d_n11, assign16230_body13_e23144_d_n12, assign16230_body13_e23144_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16230_body13_e23138: f64 = (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss);
        let assign16230_body13_e23139: f64 = (1.0 - assign16230_body13_e23138);
        let assign16230_body13_e23141: f64 = (assign16230_body13_e23139 / 2.0);
        let assign16230_body13_e23142: f64 = (assign16230_body13_e23141).sqrt();
        (assign16230_body13_e23142, (((-((locals.var_phi_soib_dpss_dn0 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn0))) / 2.0) / (2.0 * assign16230_body13_e23142)), (((-((locals.var_phi_soib_dpss_dn2 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn2))) / 2.0) / (2.0 * assign16230_body13_e23142)), (((-((locals.var_phi_soib_dpss_dn6 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn6))) / 2.0) / (2.0 * assign16230_body13_e23142)), (((-((locals.var_phi_soib_dpss_dn7 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn7))) / 2.0) / (2.0 * assign16230_body13_e23142)), (((-((locals.var_phi_soib_dpss_dn10 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn10))) / 2.0) / (2.0 * assign16230_body13_e23142)), (((-((locals.var_phi_soib_dpss_dn11 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn11))) / 2.0) / (2.0 * assign16230_body13_e23142)), (((-((locals.var_phi_soib_dpss_dn12 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn12))) / 2.0) / (2.0 * assign16230_body13_e23142)), (((-((locals.var_phi_soib_dpss_dn17 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn17))) / 2.0) / (2.0 * assign16230_body13_e23142)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16230_body13_e23144;
            locals.var_t0_dn0 = assign16230_body13_e23144_d_n0;
            locals.var_t0_dn2 = assign16230_body13_e23144_d_n2;
            locals.var_t0_dn6 = assign16230_body13_e23144_d_n6;
            locals.var_t0_dn7 = assign16230_body13_e23144_d_n7;
            locals.var_t0_dn10 = assign16230_body13_e23144_d_n10;
            locals.var_t0_dn11 = assign16230_body13_e23144_d_n11;
            locals.var_t0_dn12 = assign16230_body13_e23144_d_n12;
            locals.var_t0_dn17 = assign16230_body13_e23144_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16230_body14_e23155, assign16230_body14_e23155_d_n0, assign16230_body14_e23155_d_n2, assign16230_body14_e23155_d_n6, assign16230_body14_e23155_d_n7, assign16230_body14_e23155_d_n10, assign16230_body14_e23155_d_n11, assign16230_body14_e23155_d_n12, assign16230_body14_e23155_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16230_body14_e23153: f64 = (locals.var_chi * locals.var_t0);
        (assign16230_body14_e23153, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn12 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn12)), ((locals.var_chi_dn17 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn17)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16230_body14_e23155;
            locals.var_fb_dn0 = assign16230_body14_e23155_d_n0;
            locals.var_fb_dn2 = assign16230_body14_e23155_d_n2;
            locals.var_fb_dn6 = assign16230_body14_e23155_d_n6;
            locals.var_fb_dn7 = assign16230_body14_e23155_d_n7;
            locals.var_fb_dn10 = assign16230_body14_e23155_d_n10;
            locals.var_fb_dn11 = assign16230_body14_e23155_d_n11;
            locals.var_fb_dn12 = assign16230_body14_e23155_d_n12;
            locals.var_fb_dn17 = assign16230_body14_e23155_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16230_body15_e23166, assign16230_body15_e23166_d_n0, assign16230_body15_e23166_d_n2, assign16230_body15_e23166_d_n6, assign16230_body15_e23166_d_n7, assign16230_body15_e23166_d_n10, assign16230_body15_e23166_d_n11, assign16230_body15_e23166_d_n12, assign16230_body15_e23166_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16230_body15_e23164: f64 = (locals.var_beta * locals.var_t0);
        (assign16230_body15_e23164, (locals.var_beta * locals.var_t0_dn0), (locals.var_beta * locals.var_t0_dn2), (locals.var_beta * locals.var_t0_dn6), (locals.var_beta * locals.var_t0_dn7), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), (locals.var_beta * locals.var_t0_dn11), (locals.var_beta * locals.var_t0_dn12), (locals.var_beta * locals.var_t0_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16230_body15_e23166;
            locals.var_fb_dpss_dn0 = assign16230_body15_e23166_d_n0;
            locals.var_fb_dpss_dn2 = assign16230_body15_e23166_d_n2;
            locals.var_fb_dpss_dn6 = assign16230_body15_e23166_d_n6;
            locals.var_fb_dpss_dn7 = assign16230_body15_e23166_d_n7;
            locals.var_fb_dpss_dn10 = assign16230_body15_e23166_d_n10;
            locals.var_fb_dpss_dn11 = assign16230_body15_e23166_d_n11;
            locals.var_fb_dpss_dn12 = assign16230_body15_e23166_d_n12;
            locals.var_fb_dpss_dn17 = assign16230_body15_e23166_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign16230_body16_e23169: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard485 = assign16230_body16_e23169;
            locals.var_guard485_rv = 0.0;
            let (assign16230_body17_e23181, assign16230_body17_e23181_d_n0, assign16230_body17_e23181_d_n2, assign16230_body17_e23181_d_n6, assign16230_body17_e23181_d_n7, assign16230_body17_e23181_d_n10, assign16230_body17_e23181_d_n11, assign16230_body17_e23181_d_n12, assign16230_body17_e23181_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 != 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign16230_body17_e23179: f64 = (-locals.var_fb);
        (assign16230_body17_e23179, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16230_body17_e23181;
            locals.var_fb_dn0 = assign16230_body17_e23181_d_n0;
            locals.var_fb_dn2 = assign16230_body17_e23181_d_n2;
            locals.var_fb_dn6 = assign16230_body17_e23181_d_n6;
            locals.var_fb_dn7 = assign16230_body17_e23181_d_n7;
            locals.var_fb_dn10 = assign16230_body17_e23181_d_n10;
            locals.var_fb_dn11 = assign16230_body17_e23181_d_n11;
            locals.var_fb_dn12 = assign16230_body17_e23181_d_n12;
            locals.var_fb_dn17 = assign16230_body17_e23181_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16230_body18_e23193, assign16230_body18_e23193_d_n0, assign16230_body18_e23193_d_n2, assign16230_body18_e23193_d_n6, assign16230_body18_e23193_d_n7, assign16230_body18_e23193_d_n10, assign16230_body18_e23193_d_n11, assign16230_body18_e23193_d_n12, assign16230_body18_e23193_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 != 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign16230_body18_e23191: f64 = (-locals.var_fb_dpss);
        (assign16230_body18_e23191, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16230_body18_e23193;
            locals.var_fb_dpss_dn0 = assign16230_body18_e23193_d_n0;
            locals.var_fb_dpss_dn2 = assign16230_body18_e23193_d_n2;
            locals.var_fb_dpss_dn6 = assign16230_body18_e23193_d_n6;
            locals.var_fb_dpss_dn7 = assign16230_body18_e23193_d_n7;
            locals.var_fb_dpss_dn10 = assign16230_body18_e23193_d_n10;
            locals.var_fb_dpss_dn11 = assign16230_body18_e23193_d_n11;
            locals.var_fb_dpss_dn12 = assign16230_body18_e23193_d_n12;
            locals.var_fb_dpss_dn17 = assign16230_body18_e23193_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign16230_body19_e23195: f64 = (locals.var_chi).abs();
            let assign16230_body19_e23197: f64 = if assign16230_body19_e23195 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard486 = assign16230_body19_e23197;
            locals.var_guard486_rv = 0.0;
            let (assign16230_body20_e23231, assign16230_body20_e23231_d_n0, assign16230_body20_e23231_d_n2, assign16230_body20_e23231_d_n6, assign16230_body20_e23231_d_n7, assign16230_body20_e23231_d_n10, assign16230_body20_e23231_d_n11, assign16230_body20_e23231_d_n12, assign16230_body20_e23231_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16230_body20_e23209: f64 = (locals.var_chi * locals.var_chi);
        let assign16230_body20_e23211: f64 = (assign16230_body20_e23209 / 2.0);
        let assign16230_body20_e23215: f64 = (locals.var_chi / 3.0);
        let assign16230_body20_e23219: f64 = (locals.var_chi / 4.0);
        let assign16230_body20_e23223: f64 = (locals.var_chi / 5.0);
        let assign16230_body20_e23224: f64 = (1.0 - assign16230_body20_e23223);
        let assign16230_body20_e23225: f64 = (assign16230_body20_e23219 * assign16230_body20_e23224);
        let assign16230_body20_e23226: f64 = (1.0 - assign16230_body20_e23225);
        let assign16230_body20_e23227: f64 = (assign16230_body20_e23215 * assign16230_body20_e23226);
        let assign16230_body20_e23228: f64 = (1.0 - assign16230_body20_e23227);
        let assign16230_body20_e23229: f64 = (assign16230_body20_e23211 * assign16230_body20_e23228);
        (assign16230_body20_e23229, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign16230_body20_e23228) + (assign16230_body20_e23211 * (-(((locals.var_chi_dn0 / 3.0) * assign16230_body20_e23226) + (assign16230_body20_e23215 * (-(((locals.var_chi_dn0 / 4.0) * assign16230_body20_e23224) + (assign16230_body20_e23219 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign16230_body20_e23228) + (assign16230_body20_e23211 * (-(((locals.var_chi_dn2 / 3.0) * assign16230_body20_e23226) + (assign16230_body20_e23215 * (-(((locals.var_chi_dn2 / 4.0) * assign16230_body20_e23224) + (assign16230_body20_e23219 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign16230_body20_e23228) + (assign16230_body20_e23211 * (-(((locals.var_chi_dn6 / 3.0) * assign16230_body20_e23226) + (assign16230_body20_e23215 * (-(((locals.var_chi_dn6 / 4.0) * assign16230_body20_e23224) + (assign16230_body20_e23219 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign16230_body20_e23228) + (assign16230_body20_e23211 * (-(((locals.var_chi_dn7 / 3.0) * assign16230_body20_e23226) + (assign16230_body20_e23215 * (-(((locals.var_chi_dn7 / 4.0) * assign16230_body20_e23224) + (assign16230_body20_e23219 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign16230_body20_e23228) + (assign16230_body20_e23211 * (-(((locals.var_chi_dn10 / 3.0) * assign16230_body20_e23226) + (assign16230_body20_e23215 * (-(((locals.var_chi_dn10 / 4.0) * assign16230_body20_e23224) + (assign16230_body20_e23219 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign16230_body20_e23228) + (assign16230_body20_e23211 * (-(((locals.var_chi_dn11 / 3.0) * assign16230_body20_e23226) + (assign16230_body20_e23215 * (-(((locals.var_chi_dn11 / 4.0) * assign16230_body20_e23224) + (assign16230_body20_e23219 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) / 2.0) * assign16230_body20_e23228) + (assign16230_body20_e23211 * (-(((locals.var_chi_dn12 / 3.0) * assign16230_body20_e23226) + (assign16230_body20_e23215 * (-(((locals.var_chi_dn12 / 4.0) * assign16230_body20_e23224) + (assign16230_body20_e23219 * (-(locals.var_chi_dn12 / 5.0)))))))))), (((((locals.var_chi_dn17 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn17)) / 2.0) * assign16230_body20_e23228) + (assign16230_body20_e23211 * (-(((locals.var_chi_dn17 / 3.0) * assign16230_body20_e23226) + (assign16230_body20_e23215 * (-(((locals.var_chi_dn17 / 4.0) * assign16230_body20_e23224) + (assign16230_body20_e23219 * (-(locals.var_chi_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16230_body20_e23231;
            locals.var_t0_dn0 = assign16230_body20_e23231_d_n0;
            locals.var_t0_dn2 = assign16230_body20_e23231_d_n2;
            locals.var_t0_dn6 = assign16230_body20_e23231_d_n6;
            locals.var_t0_dn7 = assign16230_body20_e23231_d_n7;
            locals.var_t0_dn10 = assign16230_body20_e23231_d_n10;
            locals.var_t0_dn11 = assign16230_body20_e23231_d_n11;
            locals.var_t0_dn12 = assign16230_body20_e23231_d_n12;
            locals.var_t0_dn17 = assign16230_body20_e23231_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16230_body21_e23261, assign16230_body21_e23261_d_n0, assign16230_body21_e23261_d_n2, assign16230_body21_e23261_d_n6, assign16230_body21_e23261_d_n7, assign16230_body21_e23261_d_n10, assign16230_body21_e23261_d_n11, assign16230_body21_e23261_d_n12, assign16230_body21_e23261_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16230_body21_e23245: f64 = (locals.var_chi / 2.0);
        let assign16230_body21_e23249: f64 = (locals.var_chi / 3.0);
        let assign16230_body21_e23253: f64 = (locals.var_chi / 4.0);
        let assign16230_body21_e23254: f64 = (1.0 - assign16230_body21_e23253);
        let assign16230_body21_e23255: f64 = (assign16230_body21_e23249 * assign16230_body21_e23254);
        let assign16230_body21_e23256: f64 = (1.0 - assign16230_body21_e23255);
        let assign16230_body21_e23257: f64 = (assign16230_body21_e23245 * assign16230_body21_e23256);
        let assign16230_body21_e23258: f64 = (1.0 - assign16230_body21_e23257);
        let assign16230_body21_e23259: f64 = (locals.var_chi * assign16230_body21_e23258);
        (assign16230_body21_e23259, ((locals.var_chi_dn0 * assign16230_body21_e23258) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign16230_body21_e23256) + (assign16230_body21_e23245 * (-(((locals.var_chi_dn0 / 3.0) * assign16230_body21_e23254) + (assign16230_body21_e23249 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign16230_body21_e23258) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign16230_body21_e23256) + (assign16230_body21_e23245 * (-(((locals.var_chi_dn2 / 3.0) * assign16230_body21_e23254) + (assign16230_body21_e23249 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn6 * assign16230_body21_e23258) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign16230_body21_e23256) + (assign16230_body21_e23245 * (-(((locals.var_chi_dn6 / 3.0) * assign16230_body21_e23254) + (assign16230_body21_e23249 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign16230_body21_e23258) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign16230_body21_e23256) + (assign16230_body21_e23245 * (-(((locals.var_chi_dn7 / 3.0) * assign16230_body21_e23254) + (assign16230_body21_e23249 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn10 * assign16230_body21_e23258) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign16230_body21_e23256) + (assign16230_body21_e23245 * (-(((locals.var_chi_dn10 / 3.0) * assign16230_body21_e23254) + (assign16230_body21_e23249 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign16230_body21_e23258) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign16230_body21_e23256) + (assign16230_body21_e23245 * (-(((locals.var_chi_dn11 / 3.0) * assign16230_body21_e23254) + (assign16230_body21_e23249 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn12 * assign16230_body21_e23258) + (locals.var_chi * (-(((locals.var_chi_dn12 / 2.0) * assign16230_body21_e23256) + (assign16230_body21_e23245 * (-(((locals.var_chi_dn12 / 3.0) * assign16230_body21_e23254) + (assign16230_body21_e23249 * (-(locals.var_chi_dn12 / 4.0)))))))))), ((locals.var_chi_dn17 * assign16230_body21_e23258) + (locals.var_chi * (-(((locals.var_chi_dn17 / 2.0) * assign16230_body21_e23256) + (assign16230_body21_e23245 * (-(((locals.var_chi_dn17 / 3.0) * assign16230_body21_e23254) + (assign16230_body21_e23249 * (-(locals.var_chi_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16230_body21_e23261;
            locals.var_t1_dn0 = assign16230_body21_e23261_d_n0;
            locals.var_t1_dn2 = assign16230_body21_e23261_d_n2;
            locals.var_t1_dn6 = assign16230_body21_e23261_d_n6;
            locals.var_t1_dn7 = assign16230_body21_e23261_d_n7;
            locals.var_t1_dn10 = assign16230_body21_e23261_d_n10;
            locals.var_t1_dn11 = assign16230_body21_e23261_d_n11;
            locals.var_t1_dn12 = assign16230_body21_e23261_d_n12;
            locals.var_t1_dn17 = assign16230_body21_e23261_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign16230_body22_e23295, assign16230_body22_e23295_d_n0, assign16230_body22_e23295_d_n2, assign16230_body22_e23295_d_n6, assign16230_body22_e23295_d_n7, assign16230_body22_e23295_d_n10, assign16230_body22_e23295_d_n11, assign16230_body22_e23295_d_n12, assign16230_body22_e23295_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16230_body22_e23273: f64 = (locals.var_chib * locals.var_chib);
        let assign16230_body22_e23275: f64 = (assign16230_body22_e23273 / 2.0);
        let assign16230_body22_e23279: f64 = (locals.var_chib / 3.0);
        let assign16230_body22_e23283: f64 = (locals.var_chib / 4.0);
        let assign16230_body22_e23287: f64 = (locals.var_chib / 5.0);
        let assign16230_body22_e23288: f64 = (1.0 - assign16230_body22_e23287);
        let assign16230_body22_e23289: f64 = (assign16230_body22_e23283 * assign16230_body22_e23288);
        let assign16230_body22_e23290: f64 = (1.0 - assign16230_body22_e23289);
        let assign16230_body22_e23291: f64 = (assign16230_body22_e23279 * assign16230_body22_e23290);
        let assign16230_body22_e23292: f64 = (1.0 - assign16230_body22_e23291);
        let assign16230_body22_e23293: f64 = (assign16230_body22_e23275 * assign16230_body22_e23292);
        (assign16230_body22_e23293, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign16230_body22_e23292) + (assign16230_body22_e23275 * (-(((locals.var_chib_dn0 / 3.0) * assign16230_body22_e23290) + (assign16230_body22_e23279 * (-(((locals.var_chib_dn0 / 4.0) * assign16230_body22_e23288) + (assign16230_body22_e23283 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign16230_body22_e23292) + (assign16230_body22_e23275 * (-(((locals.var_chib_dn2 / 3.0) * assign16230_body22_e23290) + (assign16230_body22_e23279 * (-(((locals.var_chib_dn2 / 4.0) * assign16230_body22_e23288) + (assign16230_body22_e23283 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign16230_body22_e23292) + (assign16230_body22_e23275 * (-(((locals.var_chib_dn6 / 3.0) * assign16230_body22_e23290) + (assign16230_body22_e23279 * (-(((locals.var_chib_dn6 / 4.0) * assign16230_body22_e23288) + (assign16230_body22_e23283 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign16230_body22_e23292) + (assign16230_body22_e23275 * (-(((locals.var_chib_dn7 / 3.0) * assign16230_body22_e23290) + (assign16230_body22_e23279 * (-(((locals.var_chib_dn7 / 4.0) * assign16230_body22_e23288) + (assign16230_body22_e23283 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign16230_body22_e23292) + (assign16230_body22_e23275 * (-(((locals.var_chib_dn10 / 3.0) * assign16230_body22_e23290) + (assign16230_body22_e23279 * (-(((locals.var_chib_dn10 / 4.0) * assign16230_body22_e23288) + (assign16230_body22_e23283 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign16230_body22_e23292) + (assign16230_body22_e23275 * (-(((locals.var_chib_dn11 / 3.0) * assign16230_body22_e23290) + (assign16230_body22_e23279 * (-(((locals.var_chib_dn11 / 4.0) * assign16230_body22_e23288) + (assign16230_body22_e23283 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn12 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn12)) / 2.0) * assign16230_body22_e23292) + (assign16230_body22_e23275 * (-(((locals.var_chib_dn12 / 3.0) * assign16230_body22_e23290) + (assign16230_body22_e23279 * (-(((locals.var_chib_dn12 / 4.0) * assign16230_body22_e23288) + (assign16230_body22_e23283 * (-(locals.var_chib_dn12 / 5.0)))))))))), (((((locals.var_chib_dn17 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn17)) / 2.0) * assign16230_body22_e23292) + (assign16230_body22_e23275 * (-(((locals.var_chib_dn17 / 3.0) * assign16230_body22_e23290) + (assign16230_body22_e23279 * (-(((locals.var_chib_dn17 / 4.0) * assign16230_body22_e23288) + (assign16230_body22_e23283 * (-(locals.var_chib_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign16230_body22_e23295;
            locals.var_t2_dn0 = assign16230_body22_e23295_d_n0;
            locals.var_t2_dn2 = assign16230_body22_e23295_d_n2;
            locals.var_t2_dn6 = assign16230_body22_e23295_d_n6;
            locals.var_t2_dn7 = assign16230_body22_e23295_d_n7;
            locals.var_t2_dn10 = assign16230_body22_e23295_d_n10;
            locals.var_t2_dn11 = assign16230_body22_e23295_d_n11;
            locals.var_t2_dn12 = assign16230_body22_e23295_d_n12;
            locals.var_t2_dn17 = assign16230_body22_e23295_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign16230_body23_e23325, assign16230_body23_e23325_d_n0, assign16230_body23_e23325_d_n2, assign16230_body23_e23325_d_n6, assign16230_body23_e23325_d_n7, assign16230_body23_e23325_d_n10, assign16230_body23_e23325_d_n11, assign16230_body23_e23325_d_n12, assign16230_body23_e23325_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16230_body23_e23309: f64 = (locals.var_chib / 2.0);
        let assign16230_body23_e23313: f64 = (locals.var_chib / 3.0);
        let assign16230_body23_e23317: f64 = (locals.var_chib / 4.0);
        let assign16230_body23_e23318: f64 = (1.0 - assign16230_body23_e23317);
        let assign16230_body23_e23319: f64 = (assign16230_body23_e23313 * assign16230_body23_e23318);
        let assign16230_body23_e23320: f64 = (1.0 - assign16230_body23_e23319);
        let assign16230_body23_e23321: f64 = (assign16230_body23_e23309 * assign16230_body23_e23320);
        let assign16230_body23_e23322: f64 = (1.0 - assign16230_body23_e23321);
        let assign16230_body23_e23323: f64 = (locals.var_chib * assign16230_body23_e23322);
        (assign16230_body23_e23323, ((locals.var_chib_dn0 * assign16230_body23_e23322) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign16230_body23_e23320) + (assign16230_body23_e23309 * (-(((locals.var_chib_dn0 / 3.0) * assign16230_body23_e23318) + (assign16230_body23_e23313 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign16230_body23_e23322) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign16230_body23_e23320) + (assign16230_body23_e23309 * (-(((locals.var_chib_dn2 / 3.0) * assign16230_body23_e23318) + (assign16230_body23_e23313 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn6 * assign16230_body23_e23322) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign16230_body23_e23320) + (assign16230_body23_e23309 * (-(((locals.var_chib_dn6 / 3.0) * assign16230_body23_e23318) + (assign16230_body23_e23313 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign16230_body23_e23322) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign16230_body23_e23320) + (assign16230_body23_e23309 * (-(((locals.var_chib_dn7 / 3.0) * assign16230_body23_e23318) + (assign16230_body23_e23313 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn10 * assign16230_body23_e23322) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign16230_body23_e23320) + (assign16230_body23_e23309 * (-(((locals.var_chib_dn10 / 3.0) * assign16230_body23_e23318) + (assign16230_body23_e23313 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign16230_body23_e23322) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign16230_body23_e23320) + (assign16230_body23_e23309 * (-(((locals.var_chib_dn11 / 3.0) * assign16230_body23_e23318) + (assign16230_body23_e23313 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn12 * assign16230_body23_e23322) + (locals.var_chib * (-(((locals.var_chib_dn12 / 2.0) * assign16230_body23_e23320) + (assign16230_body23_e23309 * (-(((locals.var_chib_dn12 / 3.0) * assign16230_body23_e23318) + (assign16230_body23_e23313 * (-(locals.var_chib_dn12 / 4.0)))))))))), ((locals.var_chib_dn17 * assign16230_body23_e23322) + (locals.var_chib * (-(((locals.var_chib_dn17 / 2.0) * assign16230_body23_e23320) + (assign16230_body23_e23309 * (-(((locals.var_chib_dn17 / 3.0) * assign16230_body23_e23318) + (assign16230_body23_e23313 * (-(locals.var_chib_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign16230_body23_e23325;
            locals.var_t3_dn0 = assign16230_body23_e23325_d_n0;
            locals.var_t3_dn2 = assign16230_body23_e23325_d_n2;
            locals.var_t3_dn6 = assign16230_body23_e23325_d_n6;
            locals.var_t3_dn7 = assign16230_body23_e23325_d_n7;
            locals.var_t3_dn10 = assign16230_body23_e23325_d_n10;
            locals.var_t3_dn11 = assign16230_body23_e23325_d_n11;
            locals.var_t3_dn12 = assign16230_body23_e23325_d_n12;
            locals.var_t3_dn17 = assign16230_body23_e23325_d_n17;
            locals.var_t3_rv = 0.0;
            let (assign16230_body24_e23340, assign16230_body24_e23340_d_n0, assign16230_body24_e23340_d_n2, assign16230_body24_e23340_d_n6, assign16230_body24_e23340_d_n7, assign16230_body24_e23340_d_n10, assign16230_body24_e23340_d_n11, assign16230_body24_e23340_d_n12, assign16230_body24_e23340_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16230_body24_e23337: f64 = (locals.var_t0 - locals.var_t2);
        let assign16230_body24_e23338: f64 = (assign16230_body24_e23337).sqrt();
        (assign16230_body24_e23338, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign16230_body24_e23338)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign16230_body24_e23338)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign16230_body24_e23338)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign16230_body24_e23338)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign16230_body24_e23338)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign16230_body24_e23338)), ((locals.var_t0_dn12 - locals.var_t2_dn12) / (2.0 * assign16230_body24_e23338)), ((locals.var_t0_dn17 - locals.var_t2_dn17) / (2.0 * assign16230_body24_e23338)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16230_body24_e23340;
            locals.var_fb_dn0 = assign16230_body24_e23340_d_n0;
            locals.var_fb_dn2 = assign16230_body24_e23340_d_n2;
            locals.var_fb_dn6 = assign16230_body24_e23340_d_n6;
            locals.var_fb_dn7 = assign16230_body24_e23340_d_n7;
            locals.var_fb_dn10 = assign16230_body24_e23340_d_n10;
            locals.var_fb_dn11 = assign16230_body24_e23340_d_n11;
            locals.var_fb_dn12 = assign16230_body24_e23340_d_n12;
            locals.var_fb_dn17 = assign16230_body24_e23340_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16230_body25_e23362, assign16230_body25_e23362_d_n0, assign16230_body25_e23362_d_n2, assign16230_body25_e23362_d_n6, assign16230_body25_e23362_d_n7, assign16230_body25_e23362_d_n10, assign16230_body25_e23362_d_n11, assign16230_body25_e23362_d_n12, assign16230_body25_e23362_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16230_body25_e23352: f64 = (locals.var_beta * 0.5);
        let assign16230_body25_e23356: f64 = (locals.var_phi_soib_dpss * locals.var_t3);
        let assign16230_body25_e23357: f64 = (locals.var_t1 - assign16230_body25_e23356);
        let assign16230_body25_e23358: f64 = (assign16230_body25_e23352 * assign16230_body25_e23357);
        let assign16230_body25_e23360: f64 = (assign16230_body25_e23358 / locals.var_fb);
        (assign16230_body25_e23360, ((((assign16230_body25_e23352 * (locals.var_t1_dn0 - ((locals.var_phi_soib_dpss_dn0 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn0)))) * locals.var_fb) - (assign16230_body25_e23358 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body25_e23352 * (locals.var_t1_dn2 - ((locals.var_phi_soib_dpss_dn2 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn2)))) * locals.var_fb) - (assign16230_body25_e23358 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body25_e23352 * (locals.var_t1_dn6 - ((locals.var_phi_soib_dpss_dn6 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn6)))) * locals.var_fb) - (assign16230_body25_e23358 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body25_e23352 * (locals.var_t1_dn7 - ((locals.var_phi_soib_dpss_dn7 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn7)))) * locals.var_fb) - (assign16230_body25_e23358 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign16230_body25_e23357) + (assign16230_body25_e23352 * (locals.var_t1_dn10 - ((locals.var_phi_soib_dpss_dn10 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign16230_body25_e23358 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body25_e23352 * (locals.var_t1_dn11 - ((locals.var_phi_soib_dpss_dn11 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn11)))) * locals.var_fb) - (assign16230_body25_e23358 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body25_e23352 * (locals.var_t1_dn12 - ((locals.var_phi_soib_dpss_dn12 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn12)))) * locals.var_fb) - (assign16230_body25_e23358 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body25_e23352 * (locals.var_t1_dn17 - ((locals.var_phi_soib_dpss_dn17 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn17)))) * locals.var_fb) - (assign16230_body25_e23358 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16230_body25_e23362;
            locals.var_fb_dpss_dn0 = assign16230_body25_e23362_d_n0;
            locals.var_fb_dpss_dn2 = assign16230_body25_e23362_d_n2;
            locals.var_fb_dpss_dn6 = assign16230_body25_e23362_d_n6;
            locals.var_fb_dpss_dn7 = assign16230_body25_e23362_d_n7;
            locals.var_fb_dpss_dn10 = assign16230_body25_e23362_d_n10;
            locals.var_fb_dpss_dn11 = assign16230_body25_e23362_d_n11;
            locals.var_fb_dpss_dn12 = assign16230_body25_e23362_d_n12;
            locals.var_fb_dpss_dn17 = assign16230_body25_e23362_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let (assign16230_body26_e23377, assign16230_body26_e23377_d_n0, assign16230_body26_e23377_d_n2, assign16230_body26_e23377_d_n6, assign16230_body26_e23377_d_n7, assign16230_body26_e23377_d_n10, assign16230_body26_e23377_d_n11, assign16230_body26_e23377_d_n12, assign16230_body26_e23377_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard486 == 0.0)) {
        let assign16230_body26_e23374: f64 = (-locals.var_chi);
        let assign16230_body26_e23375: f64 = (assign16230_body26_e23374).exp();
        (assign16230_body26_e23375, (assign16230_body26_e23375 * (-locals.var_chi_dn0)), (assign16230_body26_e23375 * (-locals.var_chi_dn2)), (assign16230_body26_e23375 * (-locals.var_chi_dn6)), (assign16230_body26_e23375 * (-locals.var_chi_dn7)), (assign16230_body26_e23375 * (-locals.var_chi_dn10)), (assign16230_body26_e23375 * (-locals.var_chi_dn11)), (assign16230_body26_e23375 * (-locals.var_chi_dn12)), (assign16230_body26_e23375 * (-locals.var_chi_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16230_body26_e23377;
            locals.var_t0_dn0 = assign16230_body26_e23377_d_n0;
            locals.var_t0_dn2 = assign16230_body26_e23377_d_n2;
            locals.var_t0_dn6 = assign16230_body26_e23377_d_n6;
            locals.var_t0_dn7 = assign16230_body26_e23377_d_n7;
            locals.var_t0_dn10 = assign16230_body26_e23377_d_n10;
            locals.var_t0_dn11 = assign16230_body26_e23377_d_n11;
            locals.var_t0_dn12 = assign16230_body26_e23377_d_n12;
            locals.var_t0_dn17 = assign16230_body26_e23377_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16230_body27_e23392, assign16230_body27_e23392_d_n0, assign16230_body27_e23392_d_n2, assign16230_body27_e23392_d_n6, assign16230_body27_e23392_d_n7, assign16230_body27_e23392_d_n10, assign16230_body27_e23392_d_n11, assign16230_body27_e23392_d_n12, assign16230_body27_e23392_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard486 == 0.0)) {
        let assign16230_body27_e23389: f64 = (-locals.var_chib);
        let assign16230_body27_e23390: f64 = (assign16230_body27_e23389).exp();
        (assign16230_body27_e23390, (assign16230_body27_e23390 * (-locals.var_chib_dn0)), (assign16230_body27_e23390 * (-locals.var_chib_dn2)), (assign16230_body27_e23390 * (-locals.var_chib_dn6)), (assign16230_body27_e23390 * (-locals.var_chib_dn7)), (assign16230_body27_e23390 * (-locals.var_chib_dn10)), (assign16230_body27_e23390 * (-locals.var_chib_dn11)), (assign16230_body27_e23390 * (-locals.var_chib_dn12)), (assign16230_body27_e23390 * (-locals.var_chib_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16230_body27_e23392;
            locals.var_t1_dn0 = assign16230_body27_e23392_d_n0;
            locals.var_t1_dn2 = assign16230_body27_e23392_d_n2;
            locals.var_t1_dn6 = assign16230_body27_e23392_d_n6;
            locals.var_t1_dn7 = assign16230_body27_e23392_d_n7;
            locals.var_t1_dn10 = assign16230_body27_e23392_d_n10;
            locals.var_t1_dn11 = assign16230_body27_e23392_d_n11;
            locals.var_t1_dn12 = assign16230_body27_e23392_d_n12;
            locals.var_t1_dn17 = assign16230_body27_e23392_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign16230_body28_e23412, assign16230_body28_e23412_d_n0, assign16230_body28_e23412_d_n2, assign16230_body28_e23412_d_n6, assign16230_body28_e23412_d_n7, assign16230_body28_e23412_d_n10, assign16230_body28_e23412_d_n11, assign16230_body28_e23412_d_n12, assign16230_body28_e23412_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard486 == 0.0)) {
        let assign16230_body28_e23405: f64 = (locals.var_chi - locals.var_chib);
        let assign16230_body28_e23408: f64 = (locals.var_t0 - locals.var_t1);
        let assign16230_body28_e23409: f64 = (assign16230_body28_e23405 + assign16230_body28_e23408);
        let assign16230_body28_e23410: f64 = (assign16230_body28_e23409).sqrt();
        (assign16230_body28_e23410, (((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign16230_body28_e23410)), (((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign16230_body28_e23410)), (((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign16230_body28_e23410)), (((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign16230_body28_e23410)), (((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign16230_body28_e23410)), (((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign16230_body28_e23410)), (((locals.var_chi_dn12 - locals.var_chib_dn12) + (locals.var_t0_dn12 - locals.var_t1_dn12)) / (2.0 * assign16230_body28_e23410)), (((locals.var_chi_dn17 - locals.var_chib_dn17) + (locals.var_t0_dn17 - locals.var_t1_dn17)) / (2.0 * assign16230_body28_e23410)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16230_body28_e23412;
            locals.var_fb_dn0 = assign16230_body28_e23412_d_n0;
            locals.var_fb_dn2 = assign16230_body28_e23412_d_n2;
            locals.var_fb_dn6 = assign16230_body28_e23412_d_n6;
            locals.var_fb_dn7 = assign16230_body28_e23412_d_n7;
            locals.var_fb_dn10 = assign16230_body28_e23412_d_n10;
            locals.var_fb_dn11 = assign16230_body28_e23412_d_n11;
            locals.var_fb_dn12 = assign16230_body28_e23412_d_n12;
            locals.var_fb_dn17 = assign16230_body28_e23412_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16230_body29_e23439, assign16230_body29_e23439_d_n0, assign16230_body29_e23439_d_n2, assign16230_body29_e23439_d_n6, assign16230_body29_e23439_d_n7, assign16230_body29_e23439_d_n10, assign16230_body29_e23439_d_n11, assign16230_body29_e23439_d_n12, assign16230_body29_e23439_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard486 == 0.0)) {
        let assign16230_body29_e23425: f64 = (locals.var_beta * 0.5);
        let assign16230_body29_e23428: f64 = (1.0 - locals.var_t0);
        let assign16230_body29_e23432: f64 = (1.0 - locals.var_t1);
        let assign16230_body29_e23433: f64 = (locals.var_phi_soib_dpss * assign16230_body29_e23432);
        let assign16230_body29_e23434: f64 = (assign16230_body29_e23428 - assign16230_body29_e23433);
        let assign16230_body29_e23435: f64 = (assign16230_body29_e23425 * assign16230_body29_e23434);
        let assign16230_body29_e23437: f64 = (assign16230_body29_e23435 / locals.var_fb);
        (assign16230_body29_e23437, ((((assign16230_body29_e23425 * ((-locals.var_t0_dn0) - ((locals.var_phi_soib_dpss_dn0 * assign16230_body29_e23432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn0))))) * locals.var_fb) - (assign16230_body29_e23435 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body29_e23425 * ((-locals.var_t0_dn2) - ((locals.var_phi_soib_dpss_dn2 * assign16230_body29_e23432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn2))))) * locals.var_fb) - (assign16230_body29_e23435 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body29_e23425 * ((-locals.var_t0_dn6) - ((locals.var_phi_soib_dpss_dn6 * assign16230_body29_e23432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn6))))) * locals.var_fb) - (assign16230_body29_e23435 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body29_e23425 * ((-locals.var_t0_dn7) - ((locals.var_phi_soib_dpss_dn7 * assign16230_body29_e23432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn7))))) * locals.var_fb) - (assign16230_body29_e23435 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign16230_body29_e23434) + (assign16230_body29_e23425 * ((-locals.var_t0_dn10) - ((locals.var_phi_soib_dpss_dn10 * assign16230_body29_e23432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign16230_body29_e23435 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body29_e23425 * ((-locals.var_t0_dn11) - ((locals.var_phi_soib_dpss_dn11 * assign16230_body29_e23432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn11))))) * locals.var_fb) - (assign16230_body29_e23435 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body29_e23425 * ((-locals.var_t0_dn12) - ((locals.var_phi_soib_dpss_dn12 * assign16230_body29_e23432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn12))))) * locals.var_fb) - (assign16230_body29_e23435 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign16230_body29_e23425 * ((-locals.var_t0_dn17) - ((locals.var_phi_soib_dpss_dn17 * assign16230_body29_e23432) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn17))))) * locals.var_fb) - (assign16230_body29_e23435 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16230_body29_e23439;
            locals.var_fb_dpss_dn0 = assign16230_body29_e23439_d_n0;
            locals.var_fb_dpss_dn2 = assign16230_body29_e23439_d_n2;
            locals.var_fb_dpss_dn6 = assign16230_body29_e23439_d_n6;
            locals.var_fb_dpss_dn7 = assign16230_body29_e23439_d_n7;
            locals.var_fb_dpss_dn10 = assign16230_body29_e23439_d_n10;
            locals.var_fb_dpss_dn11 = assign16230_body29_e23439_d_n11;
            locals.var_fb_dpss_dn12 = assign16230_body29_e23439_d_n12;
            locals.var_fb_dpss_dn17 = assign16230_body29_e23439_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign16230_body30_e23442: f64 = (-1.0);
            let assign16230_body30_e23443: f64 = if locals.var_flg_zone == assign16230_body30_e23442 { 1.0 } else { 0.0 };
            locals.var_guard487 = assign16230_body30_e23443;
            locals.var_guard487_rv = 0.0;
            let (assign16230_body31_e23452, assign16230_body31_e23452_d_n0, assign16230_body31_e23452_d_n2, assign16230_body31_e23452_d_n6, assign16230_body31_e23452_d_n7, assign16230_body31_e23452_d_n10, assign16230_body31_e23452_d_n11, assign16230_body31_e23452_d_n12, assign16230_body31_e23452_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard487 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign16230_body31_e23452;
            locals.var_wdsoi_dn0 = assign16230_body31_e23452_d_n0;
            locals.var_wdsoi_dn2 = assign16230_body31_e23452_d_n2;
            locals.var_wdsoi_dn6 = assign16230_body31_e23452_d_n6;
            locals.var_wdsoi_dn7 = assign16230_body31_e23452_d_n7;
            locals.var_wdsoi_dn10 = assign16230_body31_e23452_d_n10;
            locals.var_wdsoi_dn11 = assign16230_body31_e23452_d_n11;
            locals.var_wdsoi_dn12 = assign16230_body31_e23452_d_n12;
            locals.var_wdsoi_dn17 = assign16230_body31_e23452_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let (assign16230_body32_e23464, assign16230_body32_e23464_d_n0, assign16230_body32_e23464_d_n2, assign16230_body32_e23464_d_n6, assign16230_body32_e23464_d_n7, assign16230_body32_e23464_d_n10, assign16230_body32_e23464_d_n11, assign16230_body32_e23464_d_n12, assign16230_body32_e23464_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16230_body32_e23462: f64 = (locals.var_c_w_soi * locals.var_fb);
        (assign16230_body32_e23462, ((locals.var_c_w_soi_dn0 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn0)), ((locals.var_c_w_soi_dn2 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn2)), ((locals.var_c_w_soi_dn6 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn6)), ((locals.var_c_w_soi_dn7 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn7)), ((locals.var_c_w_soi_dn10 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn10)), ((locals.var_c_w_soi_dn11 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn11)), ((locals.var_c_w_soi_dn12 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn12)), ((locals.var_c_w_soi_dn17 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn17)),)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign16230_body32_e23464;
            locals.var_wdsoi_dn0 = assign16230_body32_e23464_d_n0;
            locals.var_wdsoi_dn2 = assign16230_body32_e23464_d_n2;
            locals.var_wdsoi_dn6 = assign16230_body32_e23464_d_n6;
            locals.var_wdsoi_dn7 = assign16230_body32_e23464_d_n7;
            locals.var_wdsoi_dn10 = assign16230_body32_e23464_d_n10;
            locals.var_wdsoi_dn11 = assign16230_body32_e23464_d_n11;
            locals.var_wdsoi_dn12 = assign16230_body32_e23464_d_n12;
            locals.var_wdsoi_dn17 = assign16230_body32_e23464_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let (assign16230_body33_e23473, assign16230_body33_e23473_d_n0, assign16230_body33_e23473_d_n2, assign16230_body33_e23473_d_n6, assign16230_body33_e23473_d_n7, assign16230_body33_e23473_d_n10, assign16230_body33_e23473_d_n11, assign16230_body33_e23473_d_n12, assign16230_body33_e23473_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16230_body33_e23471: f64 = (locals.var_q_nsub * locals.var_wdsoi);
        (assign16230_body33_e23471, ((locals.var_q_nsub_dn0 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn0)), ((locals.var_q_nsub_dn2 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn2)), ((locals.var_q_nsub_dn6 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn6)), ((locals.var_q_nsub_dn7 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn7)), ((locals.var_q_nsub_dn10 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn10)), ((locals.var_q_nsub_dn11 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn11)), ((locals.var_q_nsub_dn12 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn12)), ((locals.var_q_nsub_dn17 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn17)),)
    } else {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    }
};
            locals.var_q_dep_soi = assign16230_body33_e23473;
            locals.var_q_dep_soi_dn0 = assign16230_body33_e23473_d_n0;
            locals.var_q_dep_soi_dn2 = assign16230_body33_e23473_d_n2;
            locals.var_q_dep_soi_dn6 = assign16230_body33_e23473_d_n6;
            locals.var_q_dep_soi_dn7 = assign16230_body33_e23473_d_n7;
            locals.var_q_dep_soi_dn10 = assign16230_body33_e23473_d_n10;
            locals.var_q_dep_soi_dn11 = assign16230_body33_e23473_d_n11;
            locals.var_q_dep_soi_dn12 = assign16230_body33_e23473_d_n12;
            locals.var_q_dep_soi_dn17 = assign16230_body33_e23473_d_n17;
            locals.var_q_dep_soi_rv = 0.0;
            let assign16230_body34_e23476: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard488 = assign16230_body34_e23476;
            locals.var_guard488_rv = 0.0;
            let (assign16230_body35_e23486, assign16230_body35_e23486_d_n0, assign16230_body35_e23486_d_n2, assign16230_body35_e23486_d_n6, assign16230_body35_e23486_d_n7, assign16230_body35_e23486_d_n10, assign16230_body35_e23486_d_n11, assign16230_body35_e23486_d_n12, assign16230_body35_e23486_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard488 != 0.0)) {
        let assign16230_body35_e23484: f64 = (-locals.var_fb);
        (assign16230_body35_e23484, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16230_body35_e23486;
            locals.var_fsl2_dn0 = assign16230_body35_e23486_d_n0;
            locals.var_fsl2_dn2 = assign16230_body35_e23486_d_n2;
            locals.var_fsl2_dn6 = assign16230_body35_e23486_d_n6;
            locals.var_fsl2_dn7 = assign16230_body35_e23486_d_n7;
            locals.var_fsl2_dn10 = assign16230_body35_e23486_d_n10;
            locals.var_fsl2_dn11 = assign16230_body35_e23486_d_n11;
            locals.var_fsl2_dn12 = assign16230_body35_e23486_d_n12;
            locals.var_fsl2_dn17 = assign16230_body35_e23486_d_n17;
            locals.var_fsl2_rv = 0.0;
            let (assign16230_body36_e23496, assign16230_body36_e23496_d_n0, assign16230_body36_e23496_d_n2, assign16230_body36_e23496_d_n6, assign16230_body36_e23496_d_n7, assign16230_body36_e23496_d_n10, assign16230_body36_e23496_d_n11, assign16230_body36_e23496_d_n12, assign16230_body36_e23496_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard488 != 0.0)) {
        let assign16230_body36_e23494: f64 = (-locals.var_fb_dpss);
        (assign16230_body36_e23494, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16230_body36_e23496;
            locals.var_fsl2_dpsl_dn0 = assign16230_body36_e23496_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16230_body36_e23496_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16230_body36_e23496_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16230_body36_e23496_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16230_body36_e23496_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16230_body36_e23496_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16230_body36_e23496_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16230_body36_e23496_d_n17;
            locals.var_fsl2_dpsl_rv = 0.0;
            let assign16230_body37_e23499: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard489 = assign16230_body37_e23499;
            locals.var_guard489_rv = 0.0;
            let (assign16230_body38_e23511, assign16230_body38_e23511_d_n0, assign16230_body38_e23511_d_n2, assign16230_body38_e23511_d_n6, assign16230_body38_e23511_d_n7, assign16230_body38_e23511_d_n10, assign16230_body38_e23511_d_n11, assign16230_body38_e23511_d_n12, assign16230_body38_e23511_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard489 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16230_body38_e23511;
            locals.var_fsl2_dn0 = assign16230_body38_e23511_d_n0;
            locals.var_fsl2_dn2 = assign16230_body38_e23511_d_n2;
            locals.var_fsl2_dn6 = assign16230_body38_e23511_d_n6;
            locals.var_fsl2_dn7 = assign16230_body38_e23511_d_n7;
            locals.var_fsl2_dn10 = assign16230_body38_e23511_d_n10;
            locals.var_fsl2_dn11 = assign16230_body38_e23511_d_n11;
            locals.var_fsl2_dn12 = assign16230_body38_e23511_d_n12;
            locals.var_fsl2_dn17 = assign16230_body38_e23511_d_n17;
            locals.var_fsl2_rv = 0.0;
            let (assign16230_body39_e23523, assign16230_body39_e23523_d_n0, assign16230_body39_e23523_d_n2, assign16230_body39_e23523_d_n6, assign16230_body39_e23523_d_n7, assign16230_body39_e23523_d_n10, assign16230_body39_e23523_d_n11, assign16230_body39_e23523_d_n12, assign16230_body39_e23523_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard489 != 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16230_body39_e23523;
            locals.var_fsl2_dpsl_dn0 = assign16230_body39_e23523_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16230_body39_e23523_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16230_body39_e23523_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16230_body39_e23523_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16230_body39_e23523_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16230_body39_e23523_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16230_body39_e23523_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16230_body39_e23523_d_n17;
            locals.var_fsl2_dpsl_rv = 0.0;
            let (assign16230_body40_e23540, assign16230_body40_e23540_d_n0, assign16230_body40_e23540_d_n2, assign16230_body40_e23540_d_n6, assign16230_body40_e23540_d_n7, assign16230_body40_e23540_d_n10, assign16230_body40_e23540_d_n11, assign16230_body40_e23540_d_n12, assign16230_body40_e23540_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard489 == 0.0)) {
        let assign16230_body40_e23537: f64 = (locals.var_phi_sl_soi - locals.var_vds);
        let assign16230_body40_e23538: f64 = (locals.var_beta * assign16230_body40_e23537);
        (assign16230_body40_e23538, (locals.var_beta * (locals.var_phi_sl_soi_dn0 - locals.var_vds_dn0)), (locals.var_beta * (locals.var_phi_sl_soi_dn2 - locals.var_vds_dn2)), (locals.var_beta * (locals.var_phi_sl_soi_dn6 - locals.var_vds_dn6)), (locals.var_beta * (locals.var_phi_sl_soi_dn7 - locals.var_vds_dn7)), ((locals.var_beta_dn10 * assign16230_body40_e23537) + (locals.var_beta * (locals.var_phi_sl_soi_dn10 - locals.var_vds_dn10))), (locals.var_beta * (locals.var_phi_sl_soi_dn11 - locals.var_vds_dn11)), (locals.var_beta * (locals.var_phi_sl_soi_dn12 - locals.var_vds_dn12)), (locals.var_beta * (locals.var_phi_sl_soi_dn17 - locals.var_vds_dn17)),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn12, locals.var_rho_dn17,)
    }
};
            locals.var_rho = assign16230_body40_e23540;
            locals.var_rho_dn0 = assign16230_body40_e23540_d_n0;
            locals.var_rho_dn2 = assign16230_body40_e23540_d_n2;
            locals.var_rho_dn6 = assign16230_body40_e23540_d_n6;
            locals.var_rho_dn7 = assign16230_body40_e23540_d_n7;
            locals.var_rho_dn10 = assign16230_body40_e23540_d_n10;
            locals.var_rho_dn11 = assign16230_body40_e23540_d_n11;
            locals.var_rho_dn12 = assign16230_body40_e23540_d_n12;
            locals.var_rho_dn17 = assign16230_body40_e23540_d_n17;
            locals.var_rho_rv = 0.0;
            let (assign16230_body41_e23554, assign16230_body41_e23554_d_n0, assign16230_body41_e23554_d_n2, assign16230_body41_e23554_d_n6, assign16230_body41_e23554_d_n7, assign16230_body41_e23554_d_n10, assign16230_body41_e23554_d_n11, assign16230_body41_e23554_d_n12, assign16230_body41_e23554_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard489 == 0.0)) {
        let assign16230_body41_e23552: f64 = (locals.var_rho).exp();
        (assign16230_body41_e23552, (assign16230_body41_e23552 * locals.var_rho_dn0), (assign16230_body41_e23552 * locals.var_rho_dn2), (assign16230_body41_e23552 * locals.var_rho_dn6), (assign16230_body41_e23552 * locals.var_rho_dn7), (assign16230_body41_e23552 * locals.var_rho_dn10), (assign16230_body41_e23552 * locals.var_rho_dn11), (assign16230_body41_e23552 * locals.var_rho_dn12), (assign16230_body41_e23552 * locals.var_rho_dn17),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn10, locals.var_exp_rho_dn11, locals.var_exp_rho_dn12, locals.var_exp_rho_dn17,)
    }
};
            locals.var_exp_rho = assign16230_body41_e23554;
            locals.var_exp_rho_dn0 = assign16230_body41_e23554_d_n0;
            locals.var_exp_rho_dn2 = assign16230_body41_e23554_d_n2;
            locals.var_exp_rho_dn6 = assign16230_body41_e23554_d_n6;
            locals.var_exp_rho_dn7 = assign16230_body41_e23554_d_n7;
            locals.var_exp_rho_dn10 = assign16230_body41_e23554_d_n10;
            locals.var_exp_rho_dn11 = assign16230_body41_e23554_d_n11;
            locals.var_exp_rho_dn12 = assign16230_body41_e23554_d_n12;
            locals.var_exp_rho_dn17 = assign16230_body41_e23554_d_n17;
            locals.var_exp_rho_rv = 0.0;
            let (assign16230_body42_e23575, assign16230_body42_e23575_d_n0, assign16230_body42_e23575_d_n2, assign16230_body42_e23575_d_n6, assign16230_body42_e23575_d_n7, assign16230_body42_e23575_d_n10, assign16230_body42_e23575_d_n11, assign16230_body42_e23575_d_n12, assign16230_body42_e23575_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard489 == 0.0)) {
        let assign16230_body42_e23570: f64 = (locals.var_chi + 1.0);
        let assign16230_body42_e23571: f64 = (locals.var_exp_bvbsvds * assign16230_body42_e23570);
        let assign16230_body42_e23572: f64 = (locals.var_exp_rho - assign16230_body42_e23571);
        let assign16230_body42_e23573: f64 = (locals.var_cnst1soi * assign16230_body42_e23572);
        (assign16230_body42_e23573, ((locals.var_cnst1soi_dn0 * assign16230_body42_e23572) + (locals.var_cnst1soi * (locals.var_exp_rho_dn0 - ((locals.var_exp_bvbsvds_dn0 * assign16230_body42_e23570) + (locals.var_exp_bvbsvds * locals.var_chi_dn0))))), ((locals.var_cnst1soi_dn2 * assign16230_body42_e23572) + (locals.var_cnst1soi * (locals.var_exp_rho_dn2 - ((locals.var_exp_bvbsvds_dn2 * assign16230_body42_e23570) + (locals.var_exp_bvbsvds * locals.var_chi_dn2))))), ((locals.var_cnst1soi_dn6 * assign16230_body42_e23572) + (locals.var_cnst1soi * (locals.var_exp_rho_dn6 - ((locals.var_exp_bvbsvds_dn6 * assign16230_body42_e23570) + (locals.var_exp_bvbsvds * locals.var_chi_dn6))))), ((locals.var_cnst1soi_dn7 * assign16230_body42_e23572) + (locals.var_cnst1soi * (locals.var_exp_rho_dn7 - ((locals.var_exp_bvbsvds_dn7 * assign16230_body42_e23570) + (locals.var_exp_bvbsvds * locals.var_chi_dn7))))), ((locals.var_cnst1soi_dn10 * assign16230_body42_e23572) + (locals.var_cnst1soi * (locals.var_exp_rho_dn10 - ((locals.var_exp_bvbsvds_dn10 * assign16230_body42_e23570) + (locals.var_exp_bvbsvds * locals.var_chi_dn10))))), ((locals.var_cnst1soi_dn11 * assign16230_body42_e23572) + (locals.var_cnst1soi * (locals.var_exp_rho_dn11 - ((locals.var_exp_bvbsvds_dn11 * assign16230_body42_e23570) + (locals.var_exp_bvbsvds * locals.var_chi_dn11))))), ((locals.var_cnst1soi_dn12 * assign16230_body42_e23572) + (locals.var_cnst1soi * (locals.var_exp_rho_dn12 - ((locals.var_exp_bvbsvds_dn12 * assign16230_body42_e23570) + (locals.var_exp_bvbsvds * locals.var_chi_dn12))))), ((locals.var_cnst1soi_dn17 * assign16230_body42_e23572) + (locals.var_cnst1soi * (locals.var_exp_rho_dn17 - ((locals.var_exp_bvbsvds_dn17 * assign16230_body42_e23570) + (locals.var_exp_bvbsvds * locals.var_chi_dn17))))),)
    } else {
        (locals.var_fsl1, locals.var_fsl1_dn0, locals.var_fsl1_dn2, locals.var_fsl1_dn6, locals.var_fsl1_dn7, locals.var_fsl1_dn10, locals.var_fsl1_dn11, locals.var_fsl1_dn12, locals.var_fsl1_dn17,)
    }
};
            locals.var_fsl1 = assign16230_body42_e23575;
            locals.var_fsl1_dn0 = assign16230_body42_e23575_d_n0;
            locals.var_fsl1_dn2 = assign16230_body42_e23575_d_n2;
            locals.var_fsl1_dn6 = assign16230_body42_e23575_d_n6;
            locals.var_fsl1_dn7 = assign16230_body42_e23575_d_n7;
            locals.var_fsl1_dn10 = assign16230_body42_e23575_d_n10;
            locals.var_fsl1_dn11 = assign16230_body42_e23575_d_n11;
            locals.var_fsl1_dn12 = assign16230_body42_e23575_d_n12;
            locals.var_fsl1_dn17 = assign16230_body42_e23575_d_n17;
            locals.var_fsl1_rv = 0.0;
            let (assign16230_body43_e23594, assign16230_body43_e23594_d_n0, assign16230_body43_e23594_d_n2, assign16230_body43_e23594_d_n6, assign16230_body43_e23594_d_n7, assign16230_body43_e23594_d_n10, assign16230_body43_e23594_d_n11, assign16230_body43_e23594_d_n12, assign16230_body43_e23594_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard489 == 0.0)) {
        let assign16230_body43_e23588: f64 = (locals.var_cnst1soi * locals.var_beta);
        let assign16230_body43_e23591: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign16230_body43_e23592: f64 = (assign16230_body43_e23588 * assign16230_body43_e23591);
        (assign16230_body43_e23592, (((locals.var_cnst1soi_dn0 * locals.var_beta) * assign16230_body43_e23591) + (assign16230_body43_e23588 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), (((locals.var_cnst1soi_dn2 * locals.var_beta) * assign16230_body43_e23591) + (assign16230_body43_e23588 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), (((locals.var_cnst1soi_dn6 * locals.var_beta) * assign16230_body43_e23591) + (assign16230_body43_e23588 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), (((locals.var_cnst1soi_dn7 * locals.var_beta) * assign16230_body43_e23591) + (assign16230_body43_e23588 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((((locals.var_cnst1soi_dn10 * locals.var_beta) + (locals.var_cnst1soi * locals.var_beta_dn10)) * assign16230_body43_e23591) + (assign16230_body43_e23588 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), (((locals.var_cnst1soi_dn11 * locals.var_beta) * assign16230_body43_e23591) + (assign16230_body43_e23588 * (locals.var_exp_rho_dn11 - locals.var_exp_bvbsvds_dn11))), (((locals.var_cnst1soi_dn12 * locals.var_beta) * assign16230_body43_e23591) + (assign16230_body43_e23588 * (locals.var_exp_rho_dn12 - locals.var_exp_bvbsvds_dn12))), (((locals.var_cnst1soi_dn17 * locals.var_beta) * assign16230_body43_e23591) + (assign16230_body43_e23588 * (locals.var_exp_rho_dn17 - locals.var_exp_bvbsvds_dn17))),)
    } else {
        (locals.var_fsl1_dpsl, locals.var_fsl1_dpsl_dn0, locals.var_fsl1_dpsl_dn2, locals.var_fsl1_dpsl_dn6, locals.var_fsl1_dpsl_dn7, locals.var_fsl1_dpsl_dn10, locals.var_fsl1_dpsl_dn11, locals.var_fsl1_dpsl_dn12, locals.var_fsl1_dpsl_dn17,)
    }
};
            locals.var_fsl1_dpsl = assign16230_body43_e23594;
            locals.var_fsl1_dpsl_dn0 = assign16230_body43_e23594_d_n0;
            locals.var_fsl1_dpsl_dn2 = assign16230_body43_e23594_d_n2;
            locals.var_fsl1_dpsl_dn6 = assign16230_body43_e23594_d_n6;
            locals.var_fsl1_dpsl_dn7 = assign16230_body43_e23594_d_n7;
            locals.var_fsl1_dpsl_dn10 = assign16230_body43_e23594_d_n10;
            locals.var_fsl1_dpsl_dn11 = assign16230_body43_e23594_d_n11;
            locals.var_fsl1_dpsl_dn12 = assign16230_body43_e23594_d_n12;
            locals.var_fsl1_dpsl_dn17 = assign16230_body43_e23594_d_n17;
            locals.var_fsl1_dpsl_rv = 0.0;
            let (assign16230_body44_e23612, assign16230_body44_e23612_d_n0, assign16230_body44_e23612_d_n2, assign16230_body44_e23612_d_n6, assign16230_body44_e23612_d_n7, assign16230_body44_e23612_d_n10, assign16230_body44_e23612_d_n11, assign16230_body44_e23612_d_n12, assign16230_body44_e23612_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard489 == 0.0)) {
        let assign16230_body44_e23607: f64 = (locals.var_fb * locals.var_fb);
        let assign16230_body44_e23609: f64 = (assign16230_body44_e23607 + locals.var_fsl1);
        let assign16230_body44_e23610: f64 = (assign16230_body44_e23609).sqrt();
        (assign16230_body44_e23610, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fsl1_dn0) / (2.0 * assign16230_body44_e23610)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fsl1_dn2) / (2.0 * assign16230_body44_e23610)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fsl1_dn6) / (2.0 * assign16230_body44_e23610)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fsl1_dn7) / (2.0 * assign16230_body44_e23610)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fsl1_dn10) / (2.0 * assign16230_body44_e23610)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fsl1_dn11) / (2.0 * assign16230_body44_e23610)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fsl1_dn12) / (2.0 * assign16230_body44_e23610)), ((((locals.var_fb_dn17 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn17)) + locals.var_fsl1_dn17) / (2.0 * assign16230_body44_e23610)),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16230_body44_e23612;
            locals.var_fsl2_dn0 = assign16230_body44_e23612_d_n0;
            locals.var_fsl2_dn2 = assign16230_body44_e23612_d_n2;
            locals.var_fsl2_dn6 = assign16230_body44_e23612_d_n6;
            locals.var_fsl2_dn7 = assign16230_body44_e23612_d_n7;
            locals.var_fsl2_dn10 = assign16230_body44_e23612_d_n10;
            locals.var_fsl2_dn11 = assign16230_body44_e23612_d_n11;
            locals.var_fsl2_dn12 = assign16230_body44_e23612_d_n12;
            locals.var_fsl2_dn17 = assign16230_body44_e23612_d_n17;
            locals.var_fsl2_rv = 0.0;
            let (assign16230_body45_e23635, assign16230_body45_e23635_d_n0, assign16230_body45_e23635_d_n2, assign16230_body45_e23635_d_n6, assign16230_body45_e23635_d_n7, assign16230_body45_e23635_d_n10, assign16230_body45_e23635_d_n11, assign16230_body45_e23635_d_n12, assign16230_body45_e23635_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard489 == 0.0)) {
        let assign16230_body45_e23626: f64 = (2.0 * locals.var_fb_dpss);
        let assign16230_body45_e23628: f64 = (assign16230_body45_e23626 * locals.var_fb);
        let assign16230_body45_e23630: f64 = (assign16230_body45_e23628 + locals.var_fsl1_dpsl);
        let assign16230_body45_e23631: f64 = (0.5 * assign16230_body45_e23630);
        let assign16230_body45_e23633: f64 = (assign16230_body45_e23631 / locals.var_fsl2);
        (assign16230_body45_e23633, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign16230_body45_e23626 * locals.var_fb_dn0)) + locals.var_fsl1_dpsl_dn0)) * locals.var_fsl2) - (assign16230_body45_e23631 * locals.var_fsl2_dn0)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign16230_body45_e23626 * locals.var_fb_dn2)) + locals.var_fsl1_dpsl_dn2)) * locals.var_fsl2) - (assign16230_body45_e23631 * locals.var_fsl2_dn2)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign16230_body45_e23626 * locals.var_fb_dn6)) + locals.var_fsl1_dpsl_dn6)) * locals.var_fsl2) - (assign16230_body45_e23631 * locals.var_fsl2_dn6)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign16230_body45_e23626 * locals.var_fb_dn7)) + locals.var_fsl1_dpsl_dn7)) * locals.var_fsl2) - (assign16230_body45_e23631 * locals.var_fsl2_dn7)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign16230_body45_e23626 * locals.var_fb_dn10)) + locals.var_fsl1_dpsl_dn10)) * locals.var_fsl2) - (assign16230_body45_e23631 * locals.var_fsl2_dn10)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign16230_body45_e23626 * locals.var_fb_dn11)) + locals.var_fsl1_dpsl_dn11)) * locals.var_fsl2) - (assign16230_body45_e23631 * locals.var_fsl2_dn11)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn12) * locals.var_fb) + (assign16230_body45_e23626 * locals.var_fb_dn12)) + locals.var_fsl1_dpsl_dn12)) * locals.var_fsl2) - (assign16230_body45_e23631 * locals.var_fsl2_dn12)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn17) * locals.var_fb) + (assign16230_body45_e23626 * locals.var_fb_dn17)) + locals.var_fsl1_dpsl_dn17)) * locals.var_fsl2) - (assign16230_body45_e23631 * locals.var_fsl2_dn17)) / (locals.var_fsl2 * locals.var_fsl2)),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16230_body45_e23635;
            locals.var_fsl2_dpsl_dn0 = assign16230_body45_e23635_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16230_body45_e23635_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16230_body45_e23635_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16230_body45_e23635_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16230_body45_e23635_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16230_body45_e23635_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16230_body45_e23635_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16230_body45_e23635_d_n17;
            locals.var_fsl2_dpsl_rv = 0.0;
            let (assign16230_body46_e23653, assign16230_body46_e23653_d_n0, assign16230_body46_e23653_d_n2, assign16230_body46_e23653_d_n6, assign16230_body46_e23653_d_n7, assign16230_body46_e23653_d_n10, assign16230_body46_e23653_d_n11, assign16230_body46_e23653_d_n12, assign16230_body46_e23653_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16230_body46_e23641: f64 = (-locals.var_vgp);
        let assign16230_body46_e23643: f64 = (assign16230_body46_e23641 + locals.var_phi_sl_soi);
        let assign16230_body46_e23646: f64 = (locals.var_fac1 * locals.var_fsl2);
        let assign16230_body46_e23647: f64 = (assign16230_body46_e23643 + assign16230_body46_e23646);
        let assign16230_body46_e23650: f64 = (locals.var_c_fox_inv * locals.var_qhs);
        let assign16230_body46_e23651: f64 = (assign16230_body46_e23647 - assign16230_body46_e23650);
        (assign16230_body46_e23651, ((((-locals.var_vgp_dn0) + locals.var_phi_sl_soi_dn0) + ((locals.var_fac1_dn0 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn0))) - ((locals.var_c_fox_inv_dn0 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn0))), ((((-locals.var_vgp_dn2) + locals.var_phi_sl_soi_dn2) + ((locals.var_fac1_dn2 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn2))) - ((locals.var_c_fox_inv_dn2 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn2))), ((((-locals.var_vgp_dn6) + locals.var_phi_sl_soi_dn6) + ((locals.var_fac1_dn6 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn6))) - ((locals.var_c_fox_inv_dn6 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn6))), ((((-locals.var_vgp_dn7) + locals.var_phi_sl_soi_dn7) + ((locals.var_fac1_dn7 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn7))) - ((locals.var_c_fox_inv_dn7 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn7))), ((((-locals.var_vgp_dn10) + locals.var_phi_sl_soi_dn10) + ((locals.var_fac1_dn10 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn10))) - ((locals.var_c_fox_inv_dn10 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn10))), ((((-locals.var_vgp_dn11) + locals.var_phi_sl_soi_dn11) + ((locals.var_fac1_dn11 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn11))) - ((locals.var_c_fox_inv_dn11 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn11))), ((((-locals.var_vgp_dn12) + locals.var_phi_sl_soi_dn12) + ((locals.var_fac1_dn12 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn12))) - ((locals.var_c_fox_inv_dn12 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn12))), ((((-locals.var_vgp_dn17) + locals.var_phi_sl_soi_dn17) + ((locals.var_fac1_dn17 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn17))) - ((locals.var_c_fox_inv_dn17 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn17))),)
    } else {
        (locals.var_fsl, locals.var_fsl_dn0, locals.var_fsl_dn2, locals.var_fsl_dn6, locals.var_fsl_dn7, locals.var_fsl_dn10, locals.var_fsl_dn11, locals.var_fsl_dn12, locals.var_fsl_dn17,)
    }
};
            locals.var_fsl = assign16230_body46_e23653;
            locals.var_fsl_dn0 = assign16230_body46_e23653_d_n0;
            locals.var_fsl_dn2 = assign16230_body46_e23653_d_n2;
            locals.var_fsl_dn6 = assign16230_body46_e23653_d_n6;
            locals.var_fsl_dn7 = assign16230_body46_e23653_d_n7;
            locals.var_fsl_dn10 = assign16230_body46_e23653_d_n10;
            locals.var_fsl_dn11 = assign16230_body46_e23653_d_n11;
            locals.var_fsl_dn12 = assign16230_body46_e23653_d_n12;
            locals.var_fsl_dn17 = assign16230_body46_e23653_d_n17;
            locals.var_fsl_rv = 0.0;
            let (assign16230_body47_e23664, assign16230_body47_e23664_d_n0, assign16230_body47_e23664_d_n2, assign16230_body47_e23664_d_n6, assign16230_body47_e23664_d_n7, assign16230_body47_e23664_d_n10, assign16230_body47_e23664_d_n11, assign16230_body47_e23664_d_n12, assign16230_body47_e23664_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16230_body47_e23661: f64 = (locals.var_fac1 * locals.var_fsl2_dpsl);
        let assign16230_body47_e23662: f64 = (1.0 + assign16230_body47_e23661);
        (assign16230_body47_e23662, ((locals.var_fac1_dn0 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn0)), ((locals.var_fac1_dn2 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn2)), ((locals.var_fac1_dn6 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn6)), ((locals.var_fac1_dn7 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn7)), ((locals.var_fac1_dn10 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn10)), ((locals.var_fac1_dn11 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn11)), ((locals.var_fac1_dn12 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn12)), ((locals.var_fac1_dn17 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn17)),)
    } else {
        (locals.var_fsl_dpsl, locals.var_fsl_dpsl_dn0, locals.var_fsl_dpsl_dn2, locals.var_fsl_dpsl_dn6, locals.var_fsl_dpsl_dn7, locals.var_fsl_dpsl_dn10, locals.var_fsl_dpsl_dn11, locals.var_fsl_dpsl_dn12, locals.var_fsl_dpsl_dn17,)
    }
};
            locals.var_fsl_dpsl = assign16230_body47_e23664;
            locals.var_fsl_dpsl_dn0 = assign16230_body47_e23664_d_n0;
            locals.var_fsl_dpsl_dn2 = assign16230_body47_e23664_d_n2;
            locals.var_fsl_dpsl_dn6 = assign16230_body47_e23664_d_n6;
            locals.var_fsl_dpsl_dn7 = assign16230_body47_e23664_d_n7;
            locals.var_fsl_dpsl_dn10 = assign16230_body47_e23664_d_n10;
            locals.var_fsl_dpsl_dn11 = assign16230_body47_e23664_d_n11;
            locals.var_fsl_dpsl_dn12 = assign16230_body47_e23664_d_n12;
            locals.var_fsl_dpsl_dn17 = assign16230_body47_e23664_d_n17;
            locals.var_fsl_dpsl_rv = 0.0;
            let assign16230_body48_e23671: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_lp_sl > 3.0)) { 1.0 } else { 0.0 };
            locals.var_guard490 = assign16230_body48_e23671;
            locals.var_guard490_rv = 0.0;
            let (assign16230_body49_e23682,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard490 != 0.0)) {
        let assign16230_body49_e23680: f64 = (locals.var_lp_sl_max + 1.0);
        (assign16230_body49_e23680,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign16230_body49_e23682;
            locals.var_lp_sl_rv = 0.0;
            let (assign16230_body50_e23695, assign16230_body50_e23695_d_n0, assign16230_body50_e23695_d_n2, assign16230_body50_e23695_d_n6, assign16230_body50_e23695_d_n7, assign16230_body50_e23695_d_n10, assign16230_body50_e23695_d_n11, assign16230_body50_e23695_d_n12, assign16230_body50_e23695_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard490 == 0.0)) {
        let assign16230_body50_e23691: f64 = (-locals.var_fsl);
        let assign16230_body50_e23693: f64 = (assign16230_body50_e23691 / locals.var_fsl_dpsl);
        (assign16230_body50_e23693, ((((-locals.var_fsl_dn0) * locals.var_fsl_dpsl) - (assign16230_body50_e23691 * locals.var_fsl_dpsl_dn0)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn2) * locals.var_fsl_dpsl) - (assign16230_body50_e23691 * locals.var_fsl_dpsl_dn2)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn6) * locals.var_fsl_dpsl) - (assign16230_body50_e23691 * locals.var_fsl_dpsl_dn6)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn7) * locals.var_fsl_dpsl) - (assign16230_body50_e23691 * locals.var_fsl_dpsl_dn7)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn10) * locals.var_fsl_dpsl) - (assign16230_body50_e23691 * locals.var_fsl_dpsl_dn10)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn11) * locals.var_fsl_dpsl) - (assign16230_body50_e23691 * locals.var_fsl_dpsl_dn11)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn12) * locals.var_fsl_dpsl) - (assign16230_body50_e23691 * locals.var_fsl_dpsl_dn12)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn17) * locals.var_fsl_dpsl) - (assign16230_body50_e23691 * locals.var_fsl_dpsl_dn17)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn12, locals.var_dpsl_dn17,)
    }
};
            locals.var_dpsl = assign16230_body50_e23695;
            locals.var_dpsl_dn0 = assign16230_body50_e23695_d_n0;
            locals.var_dpsl_dn2 = assign16230_body50_e23695_d_n2;
            locals.var_dpsl_dn6 = assign16230_body50_e23695_d_n6;
            locals.var_dpsl_dn7 = assign16230_body50_e23695_d_n7;
            locals.var_dpsl_dn10 = assign16230_body50_e23695_d_n10;
            locals.var_dpsl_dn11 = assign16230_body50_e23695_d_n11;
            locals.var_dpsl_dn12 = assign16230_body50_e23695_d_n12;
            locals.var_dpsl_dn17 = assign16230_body50_e23695_d_n17;
            locals.var_dpsl_rv = 0.0;
            let (assign16230_body51_e23718, assign16230_body51_e23718_d_n0, assign16230_body51_e23718_d_n2, assign16230_body51_e23718_d_n6, assign16230_body51_e23718_d_n7, assign16230_body51_e23718_d_n10, assign16230_body51_e23718_d_n11, assign16230_body51_e23718_d_n12, assign16230_body51_e23718_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard490 == 0.0)) {
        let assign16230_body51_e23705: f64 = (0.5 * 0.1);
        let assign16230_body51_e23709: f64 = (locals.var_phi_sl_soi).abs();
        let (assign16230_body51_e23714, assign16230_body51_e23714_d_n0, assign16230_body51_e23714_d_n2, assign16230_body51_e23714_d_n6, assign16230_body51_e23714_d_n7, assign16230_body51_e23714_d_n10, assign16230_body51_e23714_d_n11, assign16230_body51_e23714_d_n12, assign16230_body51_e23714_d_n17,) = {
            if (1.0 >= assign16230_body51_e23709) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign16230_body51_e23713: f64 = (locals.var_phi_sl_soi).abs();
                (assign16230_body51_e23713, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn0 } else { (-locals.var_phi_sl_soi_dn0) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn2 } else { (-locals.var_phi_sl_soi_dn2) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn6 } else { (-locals.var_phi_sl_soi_dn6) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn7 } else { (-locals.var_phi_sl_soi_dn7) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn10 } else { (-locals.var_phi_sl_soi_dn10) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn11 } else { (-locals.var_phi_sl_soi_dn11) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn12 } else { (-locals.var_phi_sl_soi_dn12) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn17 } else { (-locals.var_phi_sl_soi_dn17) },)
            }
        };
        let assign16230_body51_e23715: f64 = (1.0 + assign16230_body51_e23714);
        let assign16230_body51_e23716: f64 = (assign16230_body51_e23705 * assign16230_body51_e23715);
        (assign16230_body51_e23716, (assign16230_body51_e23705 * assign16230_body51_e23714_d_n0), (assign16230_body51_e23705 * assign16230_body51_e23714_d_n2), (assign16230_body51_e23705 * assign16230_body51_e23714_d_n6), (assign16230_body51_e23705 * assign16230_body51_e23714_d_n7), (assign16230_body51_e23705 * assign16230_body51_e23714_d_n10), (assign16230_body51_e23705 * assign16230_body51_e23714_d_n11), (assign16230_body51_e23705 * assign16230_body51_e23714_d_n12), (assign16230_body51_e23705 * assign16230_body51_e23714_d_n17),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, locals.var_dplim_dn17,)
    }
};
            locals.var_dplim = assign16230_body51_e23718;
            locals.var_dplim_dn0 = assign16230_body51_e23718_d_n0;
            locals.var_dplim_dn2 = assign16230_body51_e23718_d_n2;
            locals.var_dplim_dn6 = assign16230_body51_e23718_d_n6;
            locals.var_dplim_dn7 = assign16230_body51_e23718_d_n7;
            locals.var_dplim_dn10 = assign16230_body51_e23718_d_n10;
            locals.var_dplim_dn11 = assign16230_body51_e23718_d_n11;
            locals.var_dplim_dn12 = assign16230_body51_e23718_d_n12;
            locals.var_dplim_dn17 = assign16230_body51_e23718_d_n17;
            locals.var_dplim_rv = 0.0;
            let assign16230_body52_e23720: f64 = (locals.var_dpsl).abs();
            let assign16230_body52_e23722: f64 = if assign16230_body52_e23720 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard491 = assign16230_body52_e23722;
            locals.var_guard491_rv = 0.0;
            let (assign16230_body53_e23742, assign16230_body53_e23742_d_n0, assign16230_body53_e23742_d_n2, assign16230_body53_e23742_d_n6, assign16230_body53_e23742_d_n7, assign16230_body53_e23742_d_n10, assign16230_body53_e23742_d_n11, assign16230_body53_e23742_d_n12, assign16230_body53_e23742_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard490 == 0.0)) && (locals.var_guard491 != 0.0)) {
        let (assign16230_body53_e23739,) = {
            if (locals.var_dpsl >= 0.0) {
                (1.0,)
            } else {
                let assign16230_body53_e23738: f64 = (-1.0);
                (assign16230_body53_e23738,)
            }
        };
        let assign16230_body53_e23740: f64 = (locals.var_dplim * assign16230_body53_e23739);
        (assign16230_body53_e23740, (locals.var_dplim_dn0 * assign16230_body53_e23739), (locals.var_dplim_dn2 * assign16230_body53_e23739), (locals.var_dplim_dn6 * assign16230_body53_e23739), (locals.var_dplim_dn7 * assign16230_body53_e23739), (locals.var_dplim_dn10 * assign16230_body53_e23739), (locals.var_dplim_dn11 * assign16230_body53_e23739), (locals.var_dplim_dn12 * assign16230_body53_e23739), (locals.var_dplim_dn17 * assign16230_body53_e23739),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn12, locals.var_dpsl_dn17,)
    }
};
            locals.var_dpsl = assign16230_body53_e23742;
            locals.var_dpsl_dn0 = assign16230_body53_e23742_d_n0;
            locals.var_dpsl_dn2 = assign16230_body53_e23742_d_n2;
            locals.var_dpsl_dn6 = assign16230_body53_e23742_d_n6;
            locals.var_dpsl_dn7 = assign16230_body53_e23742_d_n7;
            locals.var_dpsl_dn10 = assign16230_body53_e23742_d_n10;
            locals.var_dpsl_dn11 = assign16230_body53_e23742_d_n11;
            locals.var_dpsl_dn12 = assign16230_body53_e23742_d_n12;
            locals.var_dpsl_dn17 = assign16230_body53_e23742_d_n17;
            locals.var_dpsl_rv = 0.0;
            let (assign16230_body54_e23754, assign16230_body54_e23754_d_n0, assign16230_body54_e23754_d_n2, assign16230_body54_e23754_d_n6, assign16230_body54_e23754_d_n7, assign16230_body54_e23754_d_n10, assign16230_body54_e23754_d_n11, assign16230_body54_e23754_d_n12, assign16230_body54_e23754_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard490 == 0.0)) {
        let assign16230_body54_e23752: f64 = (locals.var_phi_sl_soi + locals.var_dpsl);
        (assign16230_body54_e23752, (locals.var_phi_sl_soi_dn0 + locals.var_dpsl_dn0), (locals.var_phi_sl_soi_dn2 + locals.var_dpsl_dn2), (locals.var_phi_sl_soi_dn6 + locals.var_dpsl_dn6), (locals.var_phi_sl_soi_dn7 + locals.var_dpsl_dn7), (locals.var_phi_sl_soi_dn10 + locals.var_dpsl_dn10), (locals.var_phi_sl_soi_dn11 + locals.var_dpsl_dn11), (locals.var_phi_sl_soi_dn12 + locals.var_dpsl_dn12), (locals.var_phi_sl_soi_dn17 + locals.var_dpsl_dn17),)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
            locals.var_phi_sl_soi = assign16230_body54_e23754;
            locals.var_phi_sl_soi_dn0 = assign16230_body54_e23754_d_n0;
            locals.var_phi_sl_soi_dn2 = assign16230_body54_e23754_d_n2;
            locals.var_phi_sl_soi_dn6 = assign16230_body54_e23754_d_n6;
            locals.var_phi_sl_soi_dn7 = assign16230_body54_e23754_d_n7;
            locals.var_phi_sl_soi_dn10 = assign16230_body54_e23754_d_n10;
            locals.var_phi_sl_soi_dn11 = assign16230_body54_e23754_d_n11;
            locals.var_phi_sl_soi_dn12 = assign16230_body54_e23754_d_n12;
            locals.var_phi_sl_soi_dn17 = assign16230_body54_e23754_d_n17;
            locals.var_phi_sl_soi_rv = 0.0;
            let assign16230_body55_e23756: f64 = (locals.var_dpsl).abs();
            let assign16230_body55_e23760: f64 = (locals.var_fsl).abs();
            let assign16230_body55_e23763: f64 = if ((assign16230_body55_e23756 <= 5e-12) && (assign16230_body55_e23760 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard492 = assign16230_body55_e23763;
            locals.var_guard492_rv = 0.0;
            let (assign16230_body56_e23775,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard490 == 0.0)) && (locals.var_guard492 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign16230_body56_e23775;
            locals.var_flg_conv_rv = 0.0;
            let (assign16230_body57_e23784,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16230_body57_e23782: f64 = (locals.var_lp_sl + 1.0);
        (assign16230_body57_e23782,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign16230_body57_e23784;
            locals.var_lp_sl_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_58(
        locals: &mut StampLocals,
    ) {
        let (assign16240_e23793,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16240_e23791: f64 = (locals.var_lp_sl - 1.0);
        (assign16240_e23791,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign16240_e23793;
        locals.var_lp_sl_rv = 0.0;

        let (assign16250_e23800, assign16250_e23800_d_n0, assign16250_e23800_d_n2, assign16250_e23800_d_n6, assign16250_e23800_d_n7, assign16250_e23800_d_n10, assign16250_e23800_d_n11, assign16250_e23800_d_n12, assign16250_e23800_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    } else {
        (locals.var_q_depsl, locals.var_q_depsl_dn0, locals.var_q_depsl_dn2, locals.var_q_depsl_dn6, locals.var_q_depsl_dn7, locals.var_q_depsl_dn10, locals.var_q_depsl_dn11, locals.var_q_depsl_dn12, locals.var_q_depsl_dn17,)
    }
};
        locals.var_q_depsl = assign16250_e23800;
        locals.var_q_depsl_dn0 = assign16250_e23800_d_n0;
        locals.var_q_depsl_dn2 = assign16250_e23800_d_n2;
        locals.var_q_depsl_dn6 = assign16250_e23800_d_n6;
        locals.var_q_depsl_dn7 = assign16250_e23800_d_n7;
        locals.var_q_depsl_dn10 = assign16250_e23800_d_n10;
        locals.var_q_depsl_dn11 = assign16250_e23800_d_n11;
        locals.var_q_depsl_dn12 = assign16250_e23800_d_n12;
        locals.var_q_depsl_dn17 = assign16250_e23800_d_n17;
        locals.var_q_depsl_rv = 0.0;

        let (assign16260_e23807, assign16260_e23807_d_n0, assign16260_e23807_d_n2, assign16260_e23807_d_n6, assign16260_e23807_d_n7, assign16260_e23807_d_n10, assign16260_e23807_d_n11, assign16260_e23807_d_n12, assign16260_e23807_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (locals.var_q_depsl, locals.var_q_depsl_dn0, locals.var_q_depsl_dn2, locals.var_q_depsl_dn6, locals.var_q_depsl_dn7, locals.var_q_depsl_dn10, locals.var_q_depsl_dn11, locals.var_q_depsl_dn12, locals.var_q_depsl_dn17,)
    } else {
        (locals.var_q_depl, locals.var_q_depl_dn0, locals.var_q_depl_dn2, locals.var_q_depl_dn6, locals.var_q_depl_dn7, locals.var_q_depl_dn10, locals.var_q_depl_dn11, locals.var_q_depl_dn12, locals.var_q_depl_dn17,)
    }
};
        locals.var_q_depl = assign16260_e23807;
        locals.var_q_depl_dn0 = assign16260_e23807_d_n0;
        locals.var_q_depl_dn2 = assign16260_e23807_d_n2;
        locals.var_q_depl_dn6 = assign16260_e23807_d_n6;
        locals.var_q_depl_dn7 = assign16260_e23807_d_n7;
        locals.var_q_depl_dn10 = assign16260_e23807_d_n10;
        locals.var_q_depl_dn11 = assign16260_e23807_d_n11;
        locals.var_q_depl_dn12 = assign16260_e23807_d_n12;
        locals.var_q_depl_dn17 = assign16260_e23807_d_n17;
        locals.var_q_depl_rv = 0.0;

        let (assign16270_e23814, assign16270_e23814_d_n0, assign16270_e23814_d_n2, assign16270_e23814_d_n6, assign16270_e23814_d_n7, assign16270_e23814_d_n10, assign16270_e23814_d_n11, assign16270_e23814_d_n12, assign16270_e23814_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign16270_e23814;
        locals.var_psl_dn0 = assign16270_e23814_d_n0;
        locals.var_psl_dn2 = assign16270_e23814_d_n2;
        locals.var_psl_dn6 = assign16270_e23814_d_n6;
        locals.var_psl_dn7 = assign16270_e23814_d_n7;
        locals.var_psl_dn10 = assign16270_e23814_d_n10;
        locals.var_psl_dn11 = assign16270_e23814_d_n11;
        locals.var_psl_dn12 = assign16270_e23814_d_n12;
        locals.var_psl_dn17 = assign16270_e23814_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign16290_e23830, assign16290_e23830_d_n0, assign16290_e23830_d_n2, assign16290_e23830_d_n6, assign16290_e23830_d_n7, assign16290_e23830_d_n10, assign16290_e23830_d_n11, assign16290_e23830_d_n12, assign16290_e23830_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16290_e23828: f64 = (locals.var_q_depsl / locals.var_cnst0soi);
        (assign16290_e23828, (((locals.var_q_depsl_dn0 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn0)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn2 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn2)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn6 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn6)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn7 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn7)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn10 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn10)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn11 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn11)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn12 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn12)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn17 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn17)) / (locals.var_cnst0soi * locals.var_cnst0soi)),)
    } else {
        (locals.var_q_depsl_soi_o_cnst0soi, locals.var_q_depsl_soi_o_cnst0soi_dn0, locals.var_q_depsl_soi_o_cnst0soi_dn2, locals.var_q_depsl_soi_o_cnst0soi_dn6, locals.var_q_depsl_soi_o_cnst0soi_dn7, locals.var_q_depsl_soi_o_cnst0soi_dn10, locals.var_q_depsl_soi_o_cnst0soi_dn11, locals.var_q_depsl_soi_o_cnst0soi_dn12, locals.var_q_depsl_soi_o_cnst0soi_dn17,)
    }
};
        locals.var_q_depsl_soi_o_cnst0soi = assign16290_e23830;
        locals.var_q_depsl_soi_o_cnst0soi_dn0 = assign16290_e23830_d_n0;
        locals.var_q_depsl_soi_o_cnst0soi_dn2 = assign16290_e23830_d_n2;
        locals.var_q_depsl_soi_o_cnst0soi_dn6 = assign16290_e23830_d_n6;
        locals.var_q_depsl_soi_o_cnst0soi_dn7 = assign16290_e23830_d_n7;
        locals.var_q_depsl_soi_o_cnst0soi_dn10 = assign16290_e23830_d_n10;
        locals.var_q_depsl_soi_o_cnst0soi_dn11 = assign16290_e23830_d_n11;
        locals.var_q_depsl_soi_o_cnst0soi_dn12 = assign16290_e23830_d_n12;
        locals.var_q_depsl_soi_o_cnst0soi_dn17 = assign16290_e23830_d_n17;
        locals.var_q_depsl_soi_o_cnst0soi_rv = 0.0;

        let (assign16300_e23841, assign16300_e23841_d_n0, assign16300_e23841_d_n2, assign16300_e23841_d_n6, assign16300_e23841_d_n7, assign16300_e23841_d_n10, assign16300_e23841_d_n11, assign16300_e23841_d_n12, assign16300_e23841_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16300_e23838: f64 = (10.0 * 2.220446049250313e-16);
        let assign16300_e23839: f64 = (locals.var_q_depsl_soi_o_cnst0soi + assign16300_e23838);
        (assign16300_e23839, locals.var_q_depsl_soi_o_cnst0soi_dn0, locals.var_q_depsl_soi_o_cnst0soi_dn2, locals.var_q_depsl_soi_o_cnst0soi_dn6, locals.var_q_depsl_soi_o_cnst0soi_dn7, locals.var_q_depsl_soi_o_cnst0soi_dn10, locals.var_q_depsl_soi_o_cnst0soi_dn11, locals.var_q_depsl_soi_o_cnst0soi_dn12, locals.var_q_depsl_soi_o_cnst0soi_dn17,)
    } else {
        (locals.var_xilp12, locals.var_xilp12_dn0, locals.var_xilp12_dn2, locals.var_xilp12_dn6, locals.var_xilp12_dn7, locals.var_xilp12_dn10, locals.var_xilp12_dn11, locals.var_xilp12_dn12, locals.var_xilp12_dn17,)
    }
};
        locals.var_xilp12 = assign16300_e23841;
        locals.var_xilp12_dn0 = assign16300_e23841_d_n0;
        locals.var_xilp12_dn2 = assign16300_e23841_d_n2;
        locals.var_xilp12_dn6 = assign16300_e23841_d_n6;
        locals.var_xilp12_dn7 = assign16300_e23841_d_n7;
        locals.var_xilp12_dn10 = assign16300_e23841_d_n10;
        locals.var_xilp12_dn11 = assign16300_e23841_d_n11;
        locals.var_xilp12_dn12 = assign16300_e23841_d_n12;
        locals.var_xilp12_dn17 = assign16300_e23841_d_n17;
        locals.var_xilp12_rv = 0.0;

        let (assign16310_e23852, assign16310_e23852_d_n0, assign16310_e23852_d_n2, assign16310_e23852_d_n6, assign16310_e23852_d_n7, assign16310_e23852_d_n10, assign16310_e23852_d_n11, assign16310_e23852_d_n12, assign16310_e23852_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16310_e23849: f64 = (locals.var_fsl2 + locals.var_xilp12);
        let assign16310_e23850: f64 = (1.0 / assign16310_e23849);
        (assign16310_e23850, (-((locals.var_fsl2_dn0 + locals.var_xilp12_dn0) / (assign16310_e23849 * assign16310_e23849))), (-((locals.var_fsl2_dn2 + locals.var_xilp12_dn2) / (assign16310_e23849 * assign16310_e23849))), (-((locals.var_fsl2_dn6 + locals.var_xilp12_dn6) / (assign16310_e23849 * assign16310_e23849))), (-((locals.var_fsl2_dn7 + locals.var_xilp12_dn7) / (assign16310_e23849 * assign16310_e23849))), (-((locals.var_fsl2_dn10 + locals.var_xilp12_dn10) / (assign16310_e23849 * assign16310_e23849))), (-((locals.var_fsl2_dn11 + locals.var_xilp12_dn11) / (assign16310_e23849 * assign16310_e23849))), (-((locals.var_fsl2_dn12 + locals.var_xilp12_dn12) / (assign16310_e23849 * assign16310_e23849))), (-((locals.var_fsl2_dn17 + locals.var_xilp12_dn17) / (assign16310_e23849 * assign16310_e23849))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16310_e23852;
        locals.var_t1_dn0 = assign16310_e23852_d_n0;
        locals.var_t1_dn2 = assign16310_e23852_d_n2;
        locals.var_t1_dn6 = assign16310_e23852_d_n6;
        locals.var_t1_dn7 = assign16310_e23852_d_n7;
        locals.var_t1_dn10 = assign16310_e23852_d_n10;
        locals.var_t1_dn11 = assign16310_e23852_d_n11;
        locals.var_t1_dn12 = assign16310_e23852_d_n12;
        locals.var_t1_dn17 = assign16310_e23852_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16320_e23863, assign16320_e23863_d_n0, assign16320_e23863_d_n2, assign16320_e23863_d_n6, assign16320_e23863_d_n7, assign16320_e23863_d_n10, assign16320_e23863_d_n11, assign16320_e23863_d_n12, assign16320_e23863_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16320_e23859: f64 = (locals.var_cnst0soi * locals.var_fsl1);
        let assign16320_e23861: f64 = (assign16320_e23859 * locals.var_t1);
        (assign16320_e23861, ((((locals.var_cnst0soi_dn0 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn0)) * locals.var_t1) + (assign16320_e23859 * locals.var_t1_dn0)), ((((locals.var_cnst0soi_dn2 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn2)) * locals.var_t1) + (assign16320_e23859 * locals.var_t1_dn2)), ((((locals.var_cnst0soi_dn6 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn6)) * locals.var_t1) + (assign16320_e23859 * locals.var_t1_dn6)), ((((locals.var_cnst0soi_dn7 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn7)) * locals.var_t1) + (assign16320_e23859 * locals.var_t1_dn7)), ((((locals.var_cnst0soi_dn10 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn10)) * locals.var_t1) + (assign16320_e23859 * locals.var_t1_dn10)), ((((locals.var_cnst0soi_dn11 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn11)) * locals.var_t1) + (assign16320_e23859 * locals.var_t1_dn11)), ((((locals.var_cnst0soi_dn12 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn12)) * locals.var_t1) + (assign16320_e23859 * locals.var_t1_dn12)), ((((locals.var_cnst0soi_dn17 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn17)) * locals.var_t1) + (assign16320_e23859 * locals.var_t1_dn17)),)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn12, locals.var_q_nl_dn17,)
    }
};
        locals.var_q_nl = assign16320_e23863;
        locals.var_q_nl_dn0 = assign16320_e23863_d_n0;
        locals.var_q_nl_dn2 = assign16320_e23863_d_n2;
        locals.var_q_nl_dn6 = assign16320_e23863_d_n6;
        locals.var_q_nl_dn7 = assign16320_e23863_d_n7;
        locals.var_q_nl_dn10 = assign16320_e23863_d_n10;
        locals.var_q_nl_dn11 = assign16320_e23863_d_n11;
        locals.var_q_nl_dn12 = assign16320_e23863_d_n12;
        locals.var_q_nl_dn17 = assign16320_e23863_d_n17;
        locals.var_q_nl_rv = 0.0;

        let (assign16330_e23871, assign16330_e23871_d_n0, assign16330_e23871_d_n2, assign16330_e23871_d_n6, assign16330_e23871_d_n7, assign16330_e23871_d_n10, assign16330_e23871_d_n11, assign16330_e23871_d_n12, assign16330_e23871_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16330_e23869: f64 = (-locals.var_q_nl);
        (assign16330_e23869, (-locals.var_q_nl_dn0), (-locals.var_q_nl_dn2), (-locals.var_q_nl_dn6), (-locals.var_q_nl_dn7), (-locals.var_q_nl_dn10), (-locals.var_q_nl_dn11), (-locals.var_q_nl_dn12), (-locals.var_q_nl_dn17),)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn12, locals.var_q_nl_dn17,)
    }
};
        locals.var_q_nl = assign16330_e23871;
        locals.var_q_nl_dn0 = assign16330_e23871_d_n0;
        locals.var_q_nl_dn2 = assign16330_e23871_d_n2;
        locals.var_q_nl_dn6 = assign16330_e23871_d_n6;
        locals.var_q_nl_dn7 = assign16330_e23871_d_n7;
        locals.var_q_nl_dn10 = assign16330_e23871_d_n10;
        locals.var_q_nl_dn11 = assign16330_e23871_d_n11;
        locals.var_q_nl_dn12 = assign16330_e23871_d_n12;
        locals.var_q_nl_dn17 = assign16330_e23871_d_n17;
        locals.var_q_nl_rv = 0.0;

        let (assign16340_e23880, assign16340_e23880_d_n0, assign16340_e23880_d_n2, assign16340_e23880_d_n6, assign16340_e23880_d_n7, assign16340_e23880_d_n10, assign16340_e23880_d_n11, assign16340_e23880_d_n12, assign16340_e23880_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16340_e23878: f64 = (locals.var_psl - locals.var_ps0);
        (assign16340_e23878, (locals.var_psl_dn0 - locals.var_ps0_dn0), (locals.var_psl_dn2 - locals.var_ps0_dn2), (locals.var_psl_dn6 - locals.var_ps0_dn6), (locals.var_psl_dn7 - locals.var_ps0_dn7), (locals.var_psl_dn10 - locals.var_ps0_dn10), (locals.var_psl_dn11 - locals.var_ps0_dn11), (locals.var_psl_dn12 - locals.var_ps0_dn12), (locals.var_psl_dn17 - locals.var_ps0_dn17),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign16340_e23880;
        locals.var_pds_dn0 = assign16340_e23880_d_n0;
        locals.var_pds_dn2 = assign16340_e23880_d_n2;
        locals.var_pds_dn6 = assign16340_e23880_d_n6;
        locals.var_pds_dn7 = assign16340_e23880_d_n7;
        locals.var_pds_dn10 = assign16340_e23880_d_n10;
        locals.var_pds_dn11 = assign16340_e23880_d_n11;
        locals.var_pds_dn12 = assign16340_e23880_d_n12;
        locals.var_pds_dn17 = assign16340_e23880_d_n17;
        locals.var_pds_rv = 0.0;

        let (assign16350_e23887, assign16350_e23887_d_n0, assign16350_e23887_d_n2, assign16350_e23887_d_n6, assign16350_e23887_d_n7, assign16350_e23887_d_n10, assign16350_e23887_d_n11, assign16350_e23887_d_n12, assign16350_e23887_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign16350_e23887;
        locals.var_vds_dn0 = assign16350_e23887_d_n0;
        locals.var_vds_dn2 = assign16350_e23887_d_n2;
        locals.var_vds_dn6 = assign16350_e23887_d_n6;
        locals.var_vds_dn7 = assign16350_e23887_d_n7;
        locals.var_vds_dn10 = assign16350_e23887_d_n10;
        locals.var_vds_dn11 = assign16350_e23887_d_n11;
        locals.var_vds_dn12 = assign16350_e23887_d_n12;
        locals.var_vds_dn17 = assign16350_e23887_d_n17;
        locals.var_vds_rv = 0.0;

        let (assign16360_e23896, assign16360_e23896_d_n0, assign16360_e23896_d_n2, assign16360_e23896_d_n6, assign16360_e23896_d_n7, assign16360_e23896_d_n10, assign16360_e23896_d_n11, assign16360_e23896_d_n12, assign16360_e23896_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16360_e23894: f64 = (locals.var_beta / locals.var_xi0);
        (assign16360_e23894, (-((locals.var_beta * locals.var_xi0_dn0) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn2) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn6) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn7) / (locals.var_xi0 * locals.var_xi0))), (((locals.var_beta_dn10 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn10)) / (locals.var_xi0 * locals.var_xi0)), (-((locals.var_beta * locals.var_xi0_dn11) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn12) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn17) / (locals.var_xi0 * locals.var_xi0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16360_e23896;
        locals.var_t1_dn0 = assign16360_e23896_d_n0;
        locals.var_t1_dn2 = assign16360_e23896_d_n2;
        locals.var_t1_dn6 = assign16360_e23896_d_n6;
        locals.var_t1_dn7 = assign16360_e23896_d_n7;
        locals.var_t1_dn10 = assign16360_e23896_d_n10;
        locals.var_t1_dn11 = assign16360_e23896_d_n11;
        locals.var_t1_dn12 = assign16360_e23896_d_n12;
        locals.var_t1_dn17 = assign16360_e23896_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16370_e23905, assign16370_e23905_d_n0, assign16370_e23905_d_n2, assign16370_e23905_d_n6, assign16370_e23905_d_n7, assign16370_e23905_d_n10, assign16370_e23905_d_n11, assign16370_e23905_d_n12, assign16370_e23905_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16370_e23903: f64 = (locals.var_t1 * locals.var_pds);
        (assign16370_e23903, ((locals.var_t1_dn0 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn0)), ((locals.var_t1_dn2 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn2)), ((locals.var_t1_dn6 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn6)), ((locals.var_t1_dn7 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn7)), ((locals.var_t1_dn10 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn10)), ((locals.var_t1_dn11 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn11)), ((locals.var_t1_dn12 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn12)), ((locals.var_t1_dn17 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn17)),)
    } else {
        (locals.var_eta, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn12, locals.var_eta_dn17,)
    }
};
        locals.var_eta = assign16370_e23905;
        locals.var_eta_dn0 = assign16370_e23905_d_n0;
        locals.var_eta_dn2 = assign16370_e23905_d_n2;
        locals.var_eta_dn6 = assign16370_e23905_d_n6;
        locals.var_eta_dn7 = assign16370_e23905_d_n7;
        locals.var_eta_dn10 = assign16370_e23905_d_n10;
        locals.var_eta_dn11 = assign16370_e23905_d_n11;
        locals.var_eta_dn12 = assign16370_e23905_d_n12;
        locals.var_eta_dn17 = assign16370_e23905_d_n17;
        locals.var_eta_rv = 0.0;

        let (assign16380_e23914, assign16380_e23914_d_n0, assign16380_e23914_d_n2, assign16380_e23914_d_n6, assign16380_e23914_d_n7, assign16380_e23914_d_n10, assign16380_e23914_d_n11, assign16380_e23914_d_n12, assign16380_e23914_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16380_e23912: f64 = (locals.var_eta + 1.0);
        (assign16380_e23912, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn12, locals.var_eta_dn17,)
    } else {
        (locals.var_eta1, locals.var_eta1_dn0, locals.var_eta1_dn2, locals.var_eta1_dn6, locals.var_eta1_dn7, locals.var_eta1_dn10, locals.var_eta1_dn11, locals.var_eta1_dn12, locals.var_eta1_dn17,)
    }
};
        locals.var_eta1 = assign16380_e23914;
        locals.var_eta1_dn0 = assign16380_e23914_d_n0;
        locals.var_eta1_dn2 = assign16380_e23914_d_n2;
        locals.var_eta1_dn6 = assign16380_e23914_d_n6;
        locals.var_eta1_dn7 = assign16380_e23914_d_n7;
        locals.var_eta1_dn10 = assign16380_e23914_d_n10;
        locals.var_eta1_dn11 = assign16380_e23914_d_n11;
        locals.var_eta1_dn12 = assign16380_e23914_d_n12;
        locals.var_eta1_dn17 = assign16380_e23914_d_n17;
        locals.var_eta1_rv = 0.0;

        let (assign16390_e23922, assign16390_e23922_d_n0, assign16390_e23922_d_n2, assign16390_e23922_d_n6, assign16390_e23922_d_n7, assign16390_e23922_d_n10, assign16390_e23922_d_n11, assign16390_e23922_d_n12, assign16390_e23922_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16390_e23920: f64 = (locals.var_eta1).sqrt();
        (assign16390_e23920, (locals.var_eta1_dn0 / (2.0 * assign16390_e23920)), (locals.var_eta1_dn2 / (2.0 * assign16390_e23920)), (locals.var_eta1_dn6 / (2.0 * assign16390_e23920)), (locals.var_eta1_dn7 / (2.0 * assign16390_e23920)), (locals.var_eta1_dn10 / (2.0 * assign16390_e23920)), (locals.var_eta1_dn11 / (2.0 * assign16390_e23920)), (locals.var_eta1_dn12 / (2.0 * assign16390_e23920)), (locals.var_eta1_dn17 / (2.0 * assign16390_e23920)),)
    } else {
        (locals.var_eta1p12, locals.var_eta1p12_dn0, locals.var_eta1p12_dn2, locals.var_eta1p12_dn6, locals.var_eta1p12_dn7, locals.var_eta1p12_dn10, locals.var_eta1p12_dn11, locals.var_eta1p12_dn12, locals.var_eta1p12_dn17,)
    }
};
        locals.var_eta1p12 = assign16390_e23922;
        locals.var_eta1p12_dn0 = assign16390_e23922_d_n0;
        locals.var_eta1p12_dn2 = assign16390_e23922_d_n2;
        locals.var_eta1p12_dn6 = assign16390_e23922_d_n6;
        locals.var_eta1p12_dn7 = assign16390_e23922_d_n7;
        locals.var_eta1p12_dn10 = assign16390_e23922_d_n10;
        locals.var_eta1p12_dn11 = assign16390_e23922_d_n11;
        locals.var_eta1p12_dn12 = assign16390_e23922_d_n12;
        locals.var_eta1p12_dn17 = assign16390_e23922_d_n17;
        locals.var_eta1p12_rv = 0.0;

        let (assign16400_e23933, assign16400_e23933_d_n0, assign16400_e23933_d_n2, assign16400_e23933_d_n6, assign16400_e23933_d_n7, assign16400_e23933_d_n10, assign16400_e23933_d_n11, assign16400_e23933_d_n12, assign16400_e23933_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16400_e23930: f64 = (locals.var_eta1p12 + 1.0);
        let assign16400_e23931: f64 = (1.0 / assign16400_e23930);
        (assign16400_e23931, (-(locals.var_eta1p12_dn0 / (assign16400_e23930 * assign16400_e23930))), (-(locals.var_eta1p12_dn2 / (assign16400_e23930 * assign16400_e23930))), (-(locals.var_eta1p12_dn6 / (assign16400_e23930 * assign16400_e23930))), (-(locals.var_eta1p12_dn7 / (assign16400_e23930 * assign16400_e23930))), (-(locals.var_eta1p12_dn10 / (assign16400_e23930 * assign16400_e23930))), (-(locals.var_eta1p12_dn11 / (assign16400_e23930 * assign16400_e23930))), (-(locals.var_eta1p12_dn12 / (assign16400_e23930 * assign16400_e23930))), (-(locals.var_eta1p12_dn17 / (assign16400_e23930 * assign16400_e23930))),)
    } else {
        (locals.var_zeta12, locals.var_zeta12_dn0, locals.var_zeta12_dn2, locals.var_zeta12_dn6, locals.var_zeta12_dn7, locals.var_zeta12_dn10, locals.var_zeta12_dn11, locals.var_zeta12_dn12, locals.var_zeta12_dn17,)
    }
};
        locals.var_zeta12 = assign16400_e23933;
        locals.var_zeta12_dn0 = assign16400_e23933_d_n0;
        locals.var_zeta12_dn2 = assign16400_e23933_d_n2;
        locals.var_zeta12_dn6 = assign16400_e23933_d_n6;
        locals.var_zeta12_dn7 = assign16400_e23933_d_n7;
        locals.var_zeta12_dn10 = assign16400_e23933_d_n10;
        locals.var_zeta12_dn11 = assign16400_e23933_d_n11;
        locals.var_zeta12_dn12 = assign16400_e23933_d_n12;
        locals.var_zeta12_dn17 = assign16400_e23933_d_n17;
        locals.var_zeta12_rv = 0.0;

        let (assign16410_e23942, assign16410_e23942_d_n0, assign16410_e23942_d_n2, assign16410_e23942_d_n6, assign16410_e23942_d_n7, assign16410_e23942_d_n10, assign16410_e23942_d_n11, assign16410_e23942_d_n12, assign16410_e23942_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16410_e23940: f64 = (locals.var_zeta12 / locals.var_xi0p12);
        (assign16410_e23940, (((locals.var_zeta12_dn0 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn0)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn2 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn2)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn6 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn6)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn7 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn7)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn10 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn10)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn11 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn11)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn12 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn12)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn17 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn17)) / (locals.var_xi0p12 * locals.var_xi0p12)),)
    } else {
        (locals.var_f00, locals.var_f00_dn0, locals.var_f00_dn2, locals.var_f00_dn6, locals.var_f00_dn7, locals.var_f00_dn10, locals.var_f00_dn11, locals.var_f00_dn12, locals.var_f00_dn17,)
    }
};
        locals.var_f00 = assign16410_e23942;
        locals.var_f00_dn0 = assign16410_e23942_d_n0;
        locals.var_f00_dn2 = assign16410_e23942_d_n2;
        locals.var_f00_dn6 = assign16410_e23942_d_n6;
        locals.var_f00_dn7 = assign16410_e23942_d_n7;
        locals.var_f00_dn10 = assign16410_e23942_d_n10;
        locals.var_f00_dn11 = assign16410_e23942_d_n11;
        locals.var_f00_dn12 = assign16410_e23942_d_n12;
        locals.var_f00_dn17 = assign16410_e23942_d_n17;
        locals.var_f00_rv = 0.0;

        let (assign16420_e23953, assign16420_e23953_d_n0, assign16420_e23953_d_n2, assign16420_e23953_d_n6, assign16420_e23953_d_n7, assign16420_e23953_d_n10, assign16420_e23953_d_n11, assign16420_e23953_d_n12, assign16420_e23953_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16420_e23950: f64 = (locals.var_q_deps0_soi_o_cnst0soi + locals.var_q_depsl_soi_o_cnst0soi);
        let assign16420_e23951: f64 = (0.5 * assign16420_e23950);
        (assign16420_e23951, (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn0 + locals.var_q_depsl_soi_o_cnst0soi_dn0)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn2 + locals.var_q_depsl_soi_o_cnst0soi_dn2)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn6 + locals.var_q_depsl_soi_o_cnst0soi_dn6)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn7 + locals.var_q_depsl_soi_o_cnst0soi_dn7)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn10 + locals.var_q_depsl_soi_o_cnst0soi_dn10)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn11 + locals.var_q_depsl_soi_o_cnst0soi_dn11)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn12 + locals.var_q_depsl_soi_o_cnst0soi_dn12)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn17 + locals.var_q_depsl_soi_o_cnst0soi_dn17)),)
    } else {
        (locals.var_f10, locals.var_f10_dn0, locals.var_f10_dn2, locals.var_f10_dn6, locals.var_f10_dn7, locals.var_f10_dn10, locals.var_f10_dn11, locals.var_f10_dn12, locals.var_f10_dn17,)
    }
};
        locals.var_f10 = assign16420_e23953;
        locals.var_f10_dn0 = assign16420_e23953_d_n0;
        locals.var_f10_dn2 = assign16420_e23953_d_n2;
        locals.var_f10_dn6 = assign16420_e23953_d_n6;
        locals.var_f10_dn7 = assign16420_e23953_d_n7;
        locals.var_f10_dn10 = assign16420_e23953_d_n10;
        locals.var_f10_dn11 = assign16420_e23953_d_n11;
        locals.var_f10_dn12 = assign16420_e23953_d_n12;
        locals.var_f10_dn17 = assign16420_e23953_d_n17;
        locals.var_f10_rv = 0.0;

        let (assign16430_e23970, assign16430_e23970_d_n0, assign16430_e23970_d_n2, assign16430_e23970_d_n6, assign16430_e23970_d_n7, assign16430_e23970_d_n10, assign16430_e23970_d_n11, assign16430_e23970_d_n12, assign16430_e23970_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16430_e23960: f64 = (locals.var_vgp + locals.var_beta_inv);
        let assign16430_e23964: f64 = (2.0 * locals.var_ps0);
        let assign16430_e23966: f64 = (assign16430_e23964 + locals.var_pds);
        let assign16430_e23967: f64 = (0.5 * assign16430_e23966);
        let assign16430_e23968: f64 = (assign16430_e23960 - assign16430_e23967);
        (assign16430_e23968, (locals.var_vgp_dn0 - (0.5 * ((2.0 * locals.var_ps0_dn0) + locals.var_pds_dn0))), (locals.var_vgp_dn2 - (0.5 * ((2.0 * locals.var_ps0_dn2) + locals.var_pds_dn2))), (locals.var_vgp_dn6 - (0.5 * ((2.0 * locals.var_ps0_dn6) + locals.var_pds_dn6))), (locals.var_vgp_dn7 - (0.5 * ((2.0 * locals.var_ps0_dn7) + locals.var_pds_dn7))), ((locals.var_vgp_dn10 + locals.var_beta_inv_dn10) - (0.5 * ((2.0 * locals.var_ps0_dn10) + locals.var_pds_dn10))), (locals.var_vgp_dn11 - (0.5 * ((2.0 * locals.var_ps0_dn11) + locals.var_pds_dn11))), (locals.var_vgp_dn12 - (0.5 * ((2.0 * locals.var_ps0_dn12) + locals.var_pds_dn12))), (locals.var_vgp_dn17 - (0.5 * ((2.0 * locals.var_ps0_dn17) + locals.var_pds_dn17))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16430_e23970;
        locals.var_t1_dn0 = assign16430_e23970_d_n0;
        locals.var_t1_dn2 = assign16430_e23970_d_n2;
        locals.var_t1_dn6 = assign16430_e23970_d_n6;
        locals.var_t1_dn7 = assign16430_e23970_d_n7;
        locals.var_t1_dn10 = assign16430_e23970_d_n10;
        locals.var_t1_dn11 = assign16430_e23970_d_n11;
        locals.var_t1_dn12 = assign16430_e23970_d_n12;
        locals.var_t1_dn17 = assign16430_e23970_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16440_e23980, assign16440_e23980_d_n0, assign16440_e23980_d_n2, assign16440_e23980_d_n6, assign16440_e23980_d_n7, assign16440_e23980_d_n10, assign16440_e23980_d_n11, assign16440_e23980_d_n12, assign16440_e23980_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16440_e23976: f64 = (-locals.var_f10);
        let assign16440_e23978: f64 = (assign16440_e23976 + locals.var_f00);
        (assign16440_e23978, ((-locals.var_f10_dn0) + locals.var_f00_dn0), ((-locals.var_f10_dn2) + locals.var_f00_dn2), ((-locals.var_f10_dn6) + locals.var_f00_dn6), ((-locals.var_f10_dn7) + locals.var_f00_dn7), ((-locals.var_f10_dn10) + locals.var_f00_dn10), ((-locals.var_f10_dn11) + locals.var_f00_dn11), ((-locals.var_f10_dn12) + locals.var_f00_dn12), ((-locals.var_f10_dn17) + locals.var_f00_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign16440_e23980;
        locals.var_t2_dn0 = assign16440_e23980_d_n0;
        locals.var_t2_dn2 = assign16440_e23980_d_n2;
        locals.var_t2_dn6 = assign16440_e23980_d_n6;
        locals.var_t2_dn7 = assign16440_e23980_d_n7;
        locals.var_t2_dn10 = assign16440_e23980_d_n10;
        locals.var_t2_dn11 = assign16440_e23980_d_n11;
        locals.var_t2_dn12 = assign16440_e23980_d_n12;
        locals.var_t2_dn17 = assign16440_e23980_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign16450_e23989, assign16450_e23989_d_n0, assign16450_e23989_d_n2, assign16450_e23989_d_n6, assign16450_e23989_d_n7, assign16450_e23989_d_n10, assign16450_e23989_d_n11, assign16450_e23989_d_n12, assign16450_e23989_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16450_e23987: f64 = (locals.var_beta * locals.var_c_fox);
        (assign16450_e23987, (locals.var_beta * locals.var_c_fox_dn0), (locals.var_beta * locals.var_c_fox_dn2), (locals.var_beta * locals.var_c_fox_dn6), (locals.var_beta * locals.var_c_fox_dn7), ((locals.var_beta_dn10 * locals.var_c_fox) + (locals.var_beta * locals.var_c_fox_dn10)), (locals.var_beta * locals.var_c_fox_dn11), (locals.var_beta * locals.var_c_fox_dn12), (locals.var_beta * locals.var_c_fox_dn17),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign16450_e23989;
        locals.var_t3_dn0 = assign16450_e23989_d_n0;
        locals.var_t3_dn2 = assign16450_e23989_d_n2;
        locals.var_t3_dn6 = assign16450_e23989_d_n6;
        locals.var_t3_dn7 = assign16450_e23989_d_n7;
        locals.var_t3_dn10 = assign16450_e23989_d_n10;
        locals.var_t3_dn11 = assign16450_e23989_d_n11;
        locals.var_t3_dn12 = assign16450_e23989_d_n12;
        locals.var_t3_dn17 = assign16450_e23989_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign16460_e23998, assign16460_e23998_d_n0, assign16460_e23998_d_n2, assign16460_e23998_d_n6, assign16460_e23998_d_n7, assign16460_e23998_d_n10, assign16460_e23998_d_n11, assign16460_e23998_d_n12, assign16460_e23998_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16460_e23996: f64 = (locals.var_beta * locals.var_cnst0soi);
        (assign16460_e23996, (locals.var_beta * locals.var_cnst0soi_dn0), (locals.var_beta * locals.var_cnst0soi_dn2), (locals.var_beta * locals.var_cnst0soi_dn6), (locals.var_beta * locals.var_cnst0soi_dn7), ((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)), (locals.var_beta * locals.var_cnst0soi_dn11), (locals.var_beta * locals.var_cnst0soi_dn12), (locals.var_beta * locals.var_cnst0soi_dn17),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign16460_e23998;
        locals.var_t4_dn0 = assign16460_e23998_d_n0;
        locals.var_t4_dn2 = assign16460_e23998_d_n2;
        locals.var_t4_dn6 = assign16460_e23998_d_n6;
        locals.var_t4_dn7 = assign16460_e23998_d_n7;
        locals.var_t4_dn10 = assign16460_e23998_d_n10;
        locals.var_t4_dn11 = assign16460_e23998_d_n11;
        locals.var_t4_dn12 = assign16460_e23998_d_n12;
        locals.var_t4_dn17 = assign16460_e23998_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign16470_e24011, assign16470_e24011_d_n0, assign16470_e24011_d_n2, assign16470_e24011_d_n6, assign16470_e24011_d_n7, assign16470_e24011_d_n10, assign16470_e24011_d_n11, assign16470_e24011_d_n12, assign16470_e24011_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16470_e24005: f64 = (locals.var_t3 * locals.var_t1);
        let assign16470_e24008: f64 = (locals.var_t4 * locals.var_t2);
        let assign16470_e24009: f64 = (assign16470_e24005 + assign16470_e24008);
        (assign16470_e24009, (((locals.var_t3_dn0 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn0))), (((locals.var_t3_dn2 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn2))), (((locals.var_t3_dn6 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn6))), (((locals.var_t3_dn7 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn7))), (((locals.var_t3_dn10 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn10))), (((locals.var_t3_dn11 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn11))), (((locals.var_t3_dn12 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn12)) + ((locals.var_t4_dn12 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn12))), (((locals.var_t3_dn17 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn17)) + ((locals.var_t4_dn17 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn17))),)
    } else {
        (locals.var_fdd, locals.var_fdd_dn0, locals.var_fdd_dn2, locals.var_fdd_dn6, locals.var_fdd_dn7, locals.var_fdd_dn10, locals.var_fdd_dn11, locals.var_fdd_dn12, locals.var_fdd_dn17,)
    }
};
        locals.var_fdd = assign16470_e24011;
        locals.var_fdd_dn0 = assign16470_e24011_d_n0;
        locals.var_fdd_dn2 = assign16470_e24011_d_n2;
        locals.var_fdd_dn6 = assign16470_e24011_d_n6;
        locals.var_fdd_dn7 = assign16470_e24011_d_n7;
        locals.var_fdd_dn10 = assign16470_e24011_d_n10;
        locals.var_fdd_dn11 = assign16470_e24011_d_n11;
        locals.var_fdd_dn12 = assign16470_e24011_d_n12;
        locals.var_fdd_dn17 = assign16470_e24011_d_n17;
        locals.var_fdd_rv = 0.0;

        let (assign16480_e24022, assign16480_e24022_d_n0, assign16480_e24022_d_n2, assign16480_e24022_d_n6, assign16480_e24022_d_n7, assign16480_e24022_d_n10, assign16480_e24022_d_n11, assign16480_e24022_d_n12, assign16480_e24022_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16480_e24018: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign16480_e24020: f64 = (assign16480_e24018 / 2.0);
        (assign16480_e24020, ((locals.var_q_depl_dn0 + locals.var_q_dep0_dn0) / 2.0), ((locals.var_q_depl_dn2 + locals.var_q_dep0_dn2) / 2.0), ((locals.var_q_depl_dn6 + locals.var_q_dep0_dn6) / 2.0), ((locals.var_q_depl_dn7 + locals.var_q_dep0_dn7) / 2.0), ((locals.var_q_depl_dn10 + locals.var_q_dep0_dn10) / 2.0), ((locals.var_q_depl_dn11 + locals.var_q_dep0_dn11) / 2.0), ((locals.var_q_depl_dn12 + locals.var_q_dep0_dn12) / 2.0), ((locals.var_q_depl_dn17 + locals.var_q_dep0_dn17) / 2.0),)
    } else {
        (locals.var_ab, locals.var_ab_dn0, locals.var_ab_dn2, locals.var_ab_dn6, locals.var_ab_dn7, locals.var_ab_dn10, locals.var_ab_dn11, locals.var_ab_dn12, locals.var_ab_dn17,)
    }
};
        locals.var_ab = assign16480_e24022;
        locals.var_ab_dn0 = assign16480_e24022_d_n0;
        locals.var_ab_dn2 = assign16480_e24022_d_n2;
        locals.var_ab_dn6 = assign16480_e24022_d_n6;
        locals.var_ab_dn7 = assign16480_e24022_d_n7;
        locals.var_ab_dn10 = assign16480_e24022_d_n10;
        locals.var_ab_dn11 = assign16480_e24022_d_n11;
        locals.var_ab_dn12 = assign16480_e24022_d_n12;
        locals.var_ab_dn17 = assign16480_e24022_d_n17;
        locals.var_ab_rv = 0.0;

        let (assign16490_e24034, assign16490_e24034_d_n0, assign16490_e24034_d_n2, assign16490_e24034_d_n6, assign16490_e24034_d_n7, assign16490_e24034_d_n10, assign16490_e24034_d_n11, assign16490_e24034_d_n12, assign16490_e24034_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16490_e24029: f64 = (locals.var_q_nl + locals.var_q_n0);
        let assign16490_e24030: f64 = (-assign16490_e24029);
        let assign16490_e24032: f64 = (assign16490_e24030 / 2.0);
        (assign16490_e24032, ((-(locals.var_q_nl_dn0 + locals.var_q_n0_dn0)) / 2.0), ((-(locals.var_q_nl_dn2 + locals.var_q_n0_dn2)) / 2.0), ((-(locals.var_q_nl_dn6 + locals.var_q_n0_dn6)) / 2.0), ((-(locals.var_q_nl_dn7 + locals.var_q_n0_dn7)) / 2.0), ((-(locals.var_q_nl_dn10 + locals.var_q_n0_dn10)) / 2.0), ((-(locals.var_q_nl_dn11 + locals.var_q_n0_dn11)) / 2.0), ((-(locals.var_q_nl_dn12 + locals.var_q_n0_dn12)) / 2.0), ((-(locals.var_q_nl_dn17 + locals.var_q_n0_dn17)) / 2.0),)
    } else {
        (locals.var_ai, locals.var_ai_dn0, locals.var_ai_dn2, locals.var_ai_dn6, locals.var_ai_dn7, locals.var_ai_dn10, locals.var_ai_dn11, locals.var_ai_dn12, locals.var_ai_dn17,)
    }
};
        locals.var_ai = assign16490_e24034;
        locals.var_ai_dn0 = assign16490_e24034_d_n0;
        locals.var_ai_dn2 = assign16490_e24034_d_n2;
        locals.var_ai_dn6 = assign16490_e24034_d_n6;
        locals.var_ai_dn7 = assign16490_e24034_d_n7;
        locals.var_ai_dn10 = assign16490_e24034_d_n10;
        locals.var_ai_dn11 = assign16490_e24034_d_n11;
        locals.var_ai_dn12 = assign16490_e24034_d_n12;
        locals.var_ai_dn17 = assign16490_e24034_d_n17;
        locals.var_ai_rv = 0.0;

        let (assign16500_e24043, assign16500_e24043_d_n0, assign16500_e24043_d_n2, assign16500_e24043_d_n6, assign16500_e24043_d_n7, assign16500_e24043_d_n10, assign16500_e24043_d_n11, assign16500_e24043_d_n12, assign16500_e24043_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16500_e24041: f64 = (locals.var_q_depl - locals.var_q_dep0);
        (assign16500_e24041, (locals.var_q_depl_dn0 - locals.var_q_dep0_dn0), (locals.var_q_depl_dn2 - locals.var_q_dep0_dn2), (locals.var_q_depl_dn6 - locals.var_q_dep0_dn6), (locals.var_q_depl_dn7 - locals.var_q_dep0_dn7), (locals.var_q_depl_dn10 - locals.var_q_dep0_dn10), (locals.var_q_depl_dn11 - locals.var_q_dep0_dn11), (locals.var_q_depl_dn12 - locals.var_q_dep0_dn12), (locals.var_q_depl_dn17 - locals.var_q_dep0_dn17),)
    } else {
        (locals.var_db, locals.var_db_dn0, locals.var_db_dn2, locals.var_db_dn6, locals.var_db_dn7, locals.var_db_dn10, locals.var_db_dn11, locals.var_db_dn12, locals.var_db_dn17,)
    }
};
        locals.var_db = assign16500_e24043;
        locals.var_db_dn0 = assign16500_e24043_d_n0;
        locals.var_db_dn2 = assign16500_e24043_d_n2;
        locals.var_db_dn6 = assign16500_e24043_d_n6;
        locals.var_db_dn7 = assign16500_e24043_d_n7;
        locals.var_db_dn10 = assign16500_e24043_d_n10;
        locals.var_db_dn11 = assign16500_e24043_d_n11;
        locals.var_db_dn12 = assign16500_e24043_d_n12;
        locals.var_db_dn17 = assign16500_e24043_d_n17;
        locals.var_db_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_59(
        locals: &mut StampLocals,
    ) {
        let (assign16510_e24053, assign16510_e24053_d_n0, assign16510_e24053_d_n2, assign16510_e24053_d_n6, assign16510_e24053_d_n7, assign16510_e24053_d_n10, assign16510_e24053_d_n11, assign16510_e24053_d_n12, assign16510_e24053_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16510_e24050: f64 = (locals.var_q_nl - locals.var_q_n0);
        let assign16510_e24051: f64 = (-assign16510_e24050);
        (assign16510_e24051, (-(locals.var_q_nl_dn0 - locals.var_q_n0_dn0)), (-(locals.var_q_nl_dn2 - locals.var_q_n0_dn2)), (-(locals.var_q_nl_dn6 - locals.var_q_n0_dn6)), (-(locals.var_q_nl_dn7 - locals.var_q_n0_dn7)), (-(locals.var_q_nl_dn10 - locals.var_q_n0_dn10)), (-(locals.var_q_nl_dn11 - locals.var_q_n0_dn11)), (-(locals.var_q_nl_dn12 - locals.var_q_n0_dn12)), (-(locals.var_q_nl_dn17 - locals.var_q_n0_dn17)),)
    } else {
        (locals.var_di, locals.var_di_dn0, locals.var_di_dn2, locals.var_di_dn6, locals.var_di_dn7, locals.var_di_dn10, locals.var_di_dn11, locals.var_di_dn12, locals.var_di_dn17,)
    }
};
        locals.var_di = assign16510_e24053;
        locals.var_di_dn0 = assign16510_e24053_d_n0;
        locals.var_di_dn2 = assign16510_e24053_d_n2;
        locals.var_di_dn6 = assign16510_e24053_d_n6;
        locals.var_di_dn7 = assign16510_e24053_d_n7;
        locals.var_di_dn10 = assign16510_e24053_d_n10;
        locals.var_di_dn11 = assign16510_e24053_d_n11;
        locals.var_di_dn12 = assign16510_e24053_d_n12;
        locals.var_di_dn17 = assign16510_e24053_d_n17;
        locals.var_di_rv = 0.0;

        let (assign16520_e24062, assign16520_e24062_d_n0, assign16520_e24062_d_n2, assign16520_e24062_d_n6, assign16520_e24062_d_n7, assign16520_e24062_d_n10, assign16520_e24062_d_n11, assign16520_e24062_d_n12, assign16520_e24062_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16520_e24060: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        (assign16520_e24060, ((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)), ((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)), ((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)), ((locals.var_cnst0soi_dn7 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn7)), ((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)), ((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)), ((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)), ((locals.var_cnst0soi_dn17 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn17)),)
    } else {
        (locals.var_c2, locals.var_c2_dn0, locals.var_c2_dn2, locals.var_c2_dn6, locals.var_c2_dn7, locals.var_c2_dn10, locals.var_c2_dn11, locals.var_c2_dn12, locals.var_c2_dn17,)
    }
};
        locals.var_c2 = assign16520_e24062;
        locals.var_c2_dn0 = assign16520_e24062_d_n0;
        locals.var_c2_dn2 = assign16520_e24062_d_n2;
        locals.var_c2_dn6 = assign16520_e24062_d_n6;
        locals.var_c2_dn7 = assign16520_e24062_d_n7;
        locals.var_c2_dn10 = assign16520_e24062_d_n10;
        locals.var_c2_dn11 = assign16520_e24062_d_n11;
        locals.var_c2_dn12 = assign16520_e24062_d_n12;
        locals.var_c2_dn17 = assign16520_e24062_d_n17;
        locals.var_c2_rv = 0.0;

        let assign16530_e24065: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard493 = assign16530_e24065;
        locals.var_guard493_rv = 0.0;

        let (assign16540_e24090, assign16540_e24090_d_n0, assign16540_e24090_d_n2, assign16540_e24090_d_n6, assign16540_e24090_d_n7, assign16540_e24090_d_n10, assign16540_e24090_d_n11, assign16540_e24090_d_n12, assign16540_e24090_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard493 != 0.0)) {
        let assign16540_e24074: f64 = (locals.var_ai * locals.var_beta);
        let assign16540_e24076: f64 = (assign16540_e24074 * locals.var_pds);
        let assign16540_e24078: f64 = (assign16540_e24076 - locals.var_di);
        let assign16540_e24081: f64 = (locals.var_db * locals.var_db);
        let assign16540_e24083: f64 = (assign16540_e24081 * locals.var_db);
        let assign16540_e24085: f64 = (assign16540_e24083 / locals.var_c2);
        let assign16540_e24087: f64 = (assign16540_e24085 / 6.0);
        let assign16540_e24088: f64 = (assign16540_e24078 - assign16540_e24087);
        (assign16540_e24088, (((((locals.var_ai_dn0 * locals.var_beta) * locals.var_pds) + (assign16540_e24074 * locals.var_pds_dn0)) - locals.var_di_dn0) - ((((((((locals.var_db_dn0 * locals.var_db) + (locals.var_db * locals.var_db_dn0)) * locals.var_db) + (assign16540_e24081 * locals.var_db_dn0)) * locals.var_c2) - (assign16540_e24083 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn2 * locals.var_beta) * locals.var_pds) + (assign16540_e24074 * locals.var_pds_dn2)) - locals.var_di_dn2) - ((((((((locals.var_db_dn2 * locals.var_db) + (locals.var_db * locals.var_db_dn2)) * locals.var_db) + (assign16540_e24081 * locals.var_db_dn2)) * locals.var_c2) - (assign16540_e24083 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn6 * locals.var_beta) * locals.var_pds) + (assign16540_e24074 * locals.var_pds_dn6)) - locals.var_di_dn6) - ((((((((locals.var_db_dn6 * locals.var_db) + (locals.var_db * locals.var_db_dn6)) * locals.var_db) + (assign16540_e24081 * locals.var_db_dn6)) * locals.var_c2) - (assign16540_e24083 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn7 * locals.var_beta) * locals.var_pds) + (assign16540_e24074 * locals.var_pds_dn7)) - locals.var_di_dn7) - ((((((((locals.var_db_dn7 * locals.var_db) + (locals.var_db * locals.var_db_dn7)) * locals.var_db) + (assign16540_e24081 * locals.var_db_dn7)) * locals.var_c2) - (assign16540_e24083 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((locals.var_ai_dn10 * locals.var_beta) + (locals.var_ai * locals.var_beta_dn10)) * locals.var_pds) + (assign16540_e24074 * locals.var_pds_dn10)) - locals.var_di_dn10) - ((((((((locals.var_db_dn10 * locals.var_db) + (locals.var_db * locals.var_db_dn10)) * locals.var_db) + (assign16540_e24081 * locals.var_db_dn10)) * locals.var_c2) - (assign16540_e24083 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn11 * locals.var_beta) * locals.var_pds) + (assign16540_e24074 * locals.var_pds_dn11)) - locals.var_di_dn11) - ((((((((locals.var_db_dn11 * locals.var_db) + (locals.var_db * locals.var_db_dn11)) * locals.var_db) + (assign16540_e24081 * locals.var_db_dn11)) * locals.var_c2) - (assign16540_e24083 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn12 * locals.var_beta) * locals.var_pds) + (assign16540_e24074 * locals.var_pds_dn12)) - locals.var_di_dn12) - ((((((((locals.var_db_dn12 * locals.var_db) + (locals.var_db * locals.var_db_dn12)) * locals.var_db) + (assign16540_e24081 * locals.var_db_dn12)) * locals.var_c2) - (assign16540_e24083 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn17 * locals.var_beta) * locals.var_pds) + (assign16540_e24074 * locals.var_pds_dn17)) - locals.var_di_dn17) - ((((((((locals.var_db_dn17 * locals.var_db) + (locals.var_db * locals.var_db_dn17)) * locals.var_db) + (assign16540_e24081 * locals.var_db_dn17)) * locals.var_c2) - (assign16540_e24083 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16540_e24090;
        locals.var_idd_dn0 = assign16540_e24090_d_n0;
        locals.var_idd_dn2 = assign16540_e24090_d_n2;
        locals.var_idd_dn6 = assign16540_e24090_d_n6;
        locals.var_idd_dn7 = assign16540_e24090_d_n7;
        locals.var_idd_dn10 = assign16540_e24090_d_n10;
        locals.var_idd_dn11 = assign16540_e24090_d_n11;
        locals.var_idd_dn12 = assign16540_e24090_d_n12;
        locals.var_idd_dn17 = assign16540_e24090_d_n17;
        locals.var_idd_rv = 0.0;

        let (assign16550_e24102, assign16550_e24102_d_n0, assign16550_e24102_d_n2, assign16550_e24102_d_n6, assign16550_e24102_d_n7, assign16550_e24102_d_n10, assign16550_e24102_d_n11, assign16550_e24102_d_n12, assign16550_e24102_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard493 == 0.0)) {
        let assign16550_e24100: f64 = (locals.var_pds * locals.var_fdd);
        (assign16550_e24100, ((locals.var_pds_dn0 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn0)), ((locals.var_pds_dn2 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn2)), ((locals.var_pds_dn6 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn6)), ((locals.var_pds_dn7 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn7)), ((locals.var_pds_dn10 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn10)), ((locals.var_pds_dn11 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn11)), ((locals.var_pds_dn12 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn12)), ((locals.var_pds_dn17 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn17)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16550_e24102;
        locals.var_idd_dn0 = assign16550_e24102_d_n0;
        locals.var_idd_dn2 = assign16550_e24102_d_n2;
        locals.var_idd_dn6 = assign16550_e24102_d_n6;
        locals.var_idd_dn7 = assign16550_e24102_d_n7;
        locals.var_idd_dn10 = assign16550_e24102_d_n10;
        locals.var_idd_dn11 = assign16550_e24102_d_n11;
        locals.var_idd_dn12 = assign16550_e24102_d_n12;
        locals.var_idd_dn17 = assign16550_e24102_d_n17;
        locals.var_idd_rv = 0.0;

        let assign16560_e24109: f64 = if ((locals.var_flg_info >= 1.0) && (locals.var_idd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard494 = assign16560_e24109;
        locals.var_guard494_rv = 0.0;

        let (assign16570_e24118, assign16570_e24118_d_n0, assign16570_e24118_d_n2, assign16570_e24118_d_n6, assign16570_e24118_d_n7, assign16570_e24118_d_n10, assign16570_e24118_d_n11, assign16570_e24118_d_n12, assign16570_e24118_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard494 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16570_e24118;
        locals.var_idd_dn0 = assign16570_e24118_d_n0;
        locals.var_idd_dn2 = assign16570_e24118_d_n2;
        locals.var_idd_dn6 = assign16570_e24118_d_n6;
        locals.var_idd_dn7 = assign16570_e24118_d_n7;
        locals.var_idd_dn10 = assign16570_e24118_d_n10;
        locals.var_idd_dn11 = assign16570_e24118_d_n11;
        locals.var_idd_dn12 = assign16570_e24118_d_n12;
        locals.var_idd_dn17 = assign16570_e24118_d_n17;
        locals.var_idd_rv = 0.0;

        let assign16580_e24121: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard495 = assign16580_e24121;
        locals.var_guard495_rv = 0.0;

        let assign16590_e24123: f64 = (locals.var_pds).abs();
        let assign16590_e24125: f64 = if assign16590_e24123 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard496 = assign16590_e24125;
        locals.var_guard496_rv = 0.0;

        let (assign16600_e24182, assign16600_e24182_d_n0, assign16600_e24182_d_n2, assign16600_e24182_d_n6, assign16600_e24182_d_n7, assign16600_e24182_d_n10, assign16600_e24182_d_n11, assign16600_e24182_d_n12, assign16600_e24182_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_guard496 != 0.0)) {
        let assign16600_e24137: f64 = (locals.var_ai * locals.var_beta);
        let assign16600_e24139: f64 = (assign16600_e24137 * locals.var_pds);
        let assign16600_e24141: f64 = (assign16600_e24139 - locals.var_di);
        let assign16600_e24142: f64 = (locals.var_ab * assign16600_e24141);
        let assign16600_e24146: f64 = (2.0 * locals.var_ab);
        let assign16600_e24147: f64 = (locals.var_ai - assign16600_e24146);
        let assign16600_e24150: f64 = (locals.var_c_fox / locals.var_beta);
        let assign16600_e24154: f64 = (2.0 * locals.var_ab);
        let assign16600_e24156: f64 = (assign16600_e24154 * locals.var_ab);
        let assign16600_e24158: f64 = (assign16600_e24156 / locals.var_c2);
        let assign16600_e24159: f64 = (1.0 - assign16600_e24158);
        let assign16600_e24162: f64 = (locals.var_db * locals.var_db);
        let assign16600_e24164: f64 = (assign16600_e24162 / locals.var_c2);
        let assign16600_e24166: f64 = (assign16600_e24164 / 10.0);
        let assign16600_e24167: f64 = (assign16600_e24159 + assign16600_e24166);
        let assign16600_e24168: f64 = (assign16600_e24150 * assign16600_e24167);
        let assign16600_e24169: f64 = (assign16600_e24147 + assign16600_e24168);
        let assign16600_e24171: f64 = (assign16600_e24169 * locals.var_db);
        let assign16600_e24173: f64 = (assign16600_e24171 * locals.var_db);
        let assign16600_e24175: f64 = (assign16600_e24173 * locals.var_db);
        let assign16600_e24177: f64 = (assign16600_e24175 / locals.var_c2);
        let assign16600_e24179: f64 = (assign16600_e24177 / 6.0);
        let assign16600_e24180: f64 = (assign16600_e24142 + assign16600_e24179);
        (assign16600_e24180, (((locals.var_ab_dn0 * assign16600_e24141) + (locals.var_ab * ((((locals.var_ai_dn0 * locals.var_beta) * locals.var_pds) + (assign16600_e24137 * locals.var_pds_dn0)) - locals.var_di_dn0))) + ((((((((((((locals.var_ai_dn0 - (2.0 * locals.var_ab_dn0)) + (((locals.var_c_fox_dn0 / locals.var_beta) * assign16600_e24167) + (assign16600_e24150 * ((-((((((2.0 * locals.var_ab_dn0) * locals.var_ab) + (assign16600_e24154 * locals.var_ab_dn0)) * locals.var_c2) - (assign16600_e24156 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn0 * locals.var_db) + (locals.var_db * locals.var_db_dn0)) * locals.var_c2) - (assign16600_e24162 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16600_e24169 * locals.var_db_dn0)) * locals.var_db) + (assign16600_e24171 * locals.var_db_dn0)) * locals.var_db) + (assign16600_e24173 * locals.var_db_dn0)) * locals.var_c2) - (assign16600_e24175 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn2 * assign16600_e24141) + (locals.var_ab * ((((locals.var_ai_dn2 * locals.var_beta) * locals.var_pds) + (assign16600_e24137 * locals.var_pds_dn2)) - locals.var_di_dn2))) + ((((((((((((locals.var_ai_dn2 - (2.0 * locals.var_ab_dn2)) + (((locals.var_c_fox_dn2 / locals.var_beta) * assign16600_e24167) + (assign16600_e24150 * ((-((((((2.0 * locals.var_ab_dn2) * locals.var_ab) + (assign16600_e24154 * locals.var_ab_dn2)) * locals.var_c2) - (assign16600_e24156 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn2 * locals.var_db) + (locals.var_db * locals.var_db_dn2)) * locals.var_c2) - (assign16600_e24162 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16600_e24169 * locals.var_db_dn2)) * locals.var_db) + (assign16600_e24171 * locals.var_db_dn2)) * locals.var_db) + (assign16600_e24173 * locals.var_db_dn2)) * locals.var_c2) - (assign16600_e24175 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn6 * assign16600_e24141) + (locals.var_ab * ((((locals.var_ai_dn6 * locals.var_beta) * locals.var_pds) + (assign16600_e24137 * locals.var_pds_dn6)) - locals.var_di_dn6))) + ((((((((((((locals.var_ai_dn6 - (2.0 * locals.var_ab_dn6)) + (((locals.var_c_fox_dn6 / locals.var_beta) * assign16600_e24167) + (assign16600_e24150 * ((-((((((2.0 * locals.var_ab_dn6) * locals.var_ab) + (assign16600_e24154 * locals.var_ab_dn6)) * locals.var_c2) - (assign16600_e24156 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn6 * locals.var_db) + (locals.var_db * locals.var_db_dn6)) * locals.var_c2) - (assign16600_e24162 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16600_e24169 * locals.var_db_dn6)) * locals.var_db) + (assign16600_e24171 * locals.var_db_dn6)) * locals.var_db) + (assign16600_e24173 * locals.var_db_dn6)) * locals.var_c2) - (assign16600_e24175 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn7 * assign16600_e24141) + (locals.var_ab * ((((locals.var_ai_dn7 * locals.var_beta) * locals.var_pds) + (assign16600_e24137 * locals.var_pds_dn7)) - locals.var_di_dn7))) + ((((((((((((locals.var_ai_dn7 - (2.0 * locals.var_ab_dn7)) + (((locals.var_c_fox_dn7 / locals.var_beta) * assign16600_e24167) + (assign16600_e24150 * ((-((((((2.0 * locals.var_ab_dn7) * locals.var_ab) + (assign16600_e24154 * locals.var_ab_dn7)) * locals.var_c2) - (assign16600_e24156 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn7 * locals.var_db) + (locals.var_db * locals.var_db_dn7)) * locals.var_c2) - (assign16600_e24162 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16600_e24169 * locals.var_db_dn7)) * locals.var_db) + (assign16600_e24171 * locals.var_db_dn7)) * locals.var_db) + (assign16600_e24173 * locals.var_db_dn7)) * locals.var_c2) - (assign16600_e24175 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn10 * assign16600_e24141) + (locals.var_ab * (((((locals.var_ai_dn10 * locals.var_beta) + (locals.var_ai * locals.var_beta_dn10)) * locals.var_pds) + (assign16600_e24137 * locals.var_pds_dn10)) - locals.var_di_dn10))) + ((((((((((((locals.var_ai_dn10 - (2.0 * locals.var_ab_dn10)) + (((((locals.var_c_fox_dn10 * locals.var_beta) - (locals.var_c_fox * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) * assign16600_e24167) + (assign16600_e24150 * ((-((((((2.0 * locals.var_ab_dn10) * locals.var_ab) + (assign16600_e24154 * locals.var_ab_dn10)) * locals.var_c2) - (assign16600_e24156 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn10 * locals.var_db) + (locals.var_db * locals.var_db_dn10)) * locals.var_c2) - (assign16600_e24162 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16600_e24169 * locals.var_db_dn10)) * locals.var_db) + (assign16600_e24171 * locals.var_db_dn10)) * locals.var_db) + (assign16600_e24173 * locals.var_db_dn10)) * locals.var_c2) - (assign16600_e24175 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn11 * assign16600_e24141) + (locals.var_ab * ((((locals.var_ai_dn11 * locals.var_beta) * locals.var_pds) + (assign16600_e24137 * locals.var_pds_dn11)) - locals.var_di_dn11))) + ((((((((((((locals.var_ai_dn11 - (2.0 * locals.var_ab_dn11)) + (((locals.var_c_fox_dn11 / locals.var_beta) * assign16600_e24167) + (assign16600_e24150 * ((-((((((2.0 * locals.var_ab_dn11) * locals.var_ab) + (assign16600_e24154 * locals.var_ab_dn11)) * locals.var_c2) - (assign16600_e24156 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn11 * locals.var_db) + (locals.var_db * locals.var_db_dn11)) * locals.var_c2) - (assign16600_e24162 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16600_e24169 * locals.var_db_dn11)) * locals.var_db) + (assign16600_e24171 * locals.var_db_dn11)) * locals.var_db) + (assign16600_e24173 * locals.var_db_dn11)) * locals.var_c2) - (assign16600_e24175 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn12 * assign16600_e24141) + (locals.var_ab * ((((locals.var_ai_dn12 * locals.var_beta) * locals.var_pds) + (assign16600_e24137 * locals.var_pds_dn12)) - locals.var_di_dn12))) + ((((((((((((locals.var_ai_dn12 - (2.0 * locals.var_ab_dn12)) + (((locals.var_c_fox_dn12 / locals.var_beta) * assign16600_e24167) + (assign16600_e24150 * ((-((((((2.0 * locals.var_ab_dn12) * locals.var_ab) + (assign16600_e24154 * locals.var_ab_dn12)) * locals.var_c2) - (assign16600_e24156 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn12 * locals.var_db) + (locals.var_db * locals.var_db_dn12)) * locals.var_c2) - (assign16600_e24162 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16600_e24169 * locals.var_db_dn12)) * locals.var_db) + (assign16600_e24171 * locals.var_db_dn12)) * locals.var_db) + (assign16600_e24173 * locals.var_db_dn12)) * locals.var_c2) - (assign16600_e24175 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn17 * assign16600_e24141) + (locals.var_ab * ((((locals.var_ai_dn17 * locals.var_beta) * locals.var_pds) + (assign16600_e24137 * locals.var_pds_dn17)) - locals.var_di_dn17))) + ((((((((((((locals.var_ai_dn17 - (2.0 * locals.var_ab_dn17)) + (((locals.var_c_fox_dn17 / locals.var_beta) * assign16600_e24167) + (assign16600_e24150 * ((-((((((2.0 * locals.var_ab_dn17) * locals.var_ab) + (assign16600_e24154 * locals.var_ab_dn17)) * locals.var_c2) - (assign16600_e24156 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn17 * locals.var_db) + (locals.var_db * locals.var_db_dn17)) * locals.var_c2) - (assign16600_e24162 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16600_e24169 * locals.var_db_dn17)) * locals.var_db) + (assign16600_e24171 * locals.var_db_dn17)) * locals.var_db) + (assign16600_e24173 * locals.var_db_dn17)) * locals.var_c2) - (assign16600_e24175 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16600_e24182;
        locals.var_qbu_dn0 = assign16600_e24182_d_n0;
        locals.var_qbu_dn2 = assign16600_e24182_d_n2;
        locals.var_qbu_dn6 = assign16600_e24182_d_n6;
        locals.var_qbu_dn7 = assign16600_e24182_d_n7;
        locals.var_qbu_dn10 = assign16600_e24182_d_n10;
        locals.var_qbu_dn11 = assign16600_e24182_d_n11;
        locals.var_qbu_dn12 = assign16600_e24182_d_n12;
        locals.var_qbu_dn17 = assign16600_e24182_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16610_e24195, assign16610_e24195_d_n0, assign16610_e24195_d_n2, assign16610_e24195_d_n6, assign16610_e24195_d_n7, assign16610_e24195_d_n10, assign16610_e24195_d_n11, assign16610_e24195_d_n12, assign16610_e24195_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_guard496 != 0.0)) {
        let assign16610_e24193: f64 = (locals.var_qbu / locals.var_idd);
        (assign16610_e24193, (((locals.var_qbu_dn0 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn0)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn2 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn2)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn6 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn6)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn7 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn7)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn10 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn10)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn11 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn11)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn12 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn12)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn17 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn17)) / (locals.var_idd * locals.var_idd)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16610_e24195;
        locals.var_qbu_dn0 = assign16610_e24195_d_n0;
        locals.var_qbu_dn2 = assign16610_e24195_d_n2;
        locals.var_qbu_dn6 = assign16610_e24195_d_n6;
        locals.var_qbu_dn7 = assign16610_e24195_d_n7;
        locals.var_qbu_dn10 = assign16610_e24195_d_n10;
        locals.var_qbu_dn11 = assign16610_e24195_d_n11;
        locals.var_qbu_dn12 = assign16610_e24195_d_n12;
        locals.var_qbu_dn17 = assign16610_e24195_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16620_e24207, assign16620_e24207_d_n0, assign16620_e24207_d_n2, assign16620_e24207_d_n6, assign16620_e24207_d_n7, assign16620_e24207_d_n10, assign16620_e24207_d_n11, assign16620_e24207_d_n12, assign16620_e24207_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_guard496 == 0.0)) {
        (locals.var_ab, locals.var_ab_dn0, locals.var_ab_dn2, locals.var_ab_dn6, locals.var_ab_dn7, locals.var_ab_dn10, locals.var_ab_dn11, locals.var_ab_dn12, locals.var_ab_dn17,)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16620_e24207;
        locals.var_qbu_dn0 = assign16620_e24207_d_n0;
        locals.var_qbu_dn2 = assign16620_e24207_d_n2;
        locals.var_qbu_dn6 = assign16620_e24207_d_n6;
        locals.var_qbu_dn7 = assign16620_e24207_d_n7;
        locals.var_qbu_dn10 = assign16620_e24207_d_n10;
        locals.var_qbu_dn11 = assign16620_e24207_d_n11;
        locals.var_qbu_dn12 = assign16620_e24207_d_n12;
        locals.var_qbu_dn17 = assign16620_e24207_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16630_e24221, assign16630_e24221_d_n0, assign16630_e24221_d_n2, assign16630_e24221_d_n6, assign16630_e24221_d_n7, assign16630_e24221_d_n10, assign16630_e24221_d_n11, assign16630_e24221_d_n12, assign16630_e24221_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard495 == 0.0)) {
        let assign16630_e24218: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign16630_e24219: f64 = (0.5 * assign16630_e24218);
        (assign16630_e24219, (0.5 * (locals.var_q_depl_dn0 + locals.var_q_dep0_dn0)), (0.5 * (locals.var_q_depl_dn2 + locals.var_q_dep0_dn2)), (0.5 * (locals.var_q_depl_dn6 + locals.var_q_dep0_dn6)), (0.5 * (locals.var_q_depl_dn7 + locals.var_q_dep0_dn7)), (0.5 * (locals.var_q_depl_dn10 + locals.var_q_dep0_dn10)), (0.5 * (locals.var_q_depl_dn11 + locals.var_q_dep0_dn11)), (0.5 * (locals.var_q_depl_dn12 + locals.var_q_dep0_dn12)), (0.5 * (locals.var_q_depl_dn17 + locals.var_q_dep0_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16630_e24221;
        locals.var_qbu_dn0 = assign16630_e24221_d_n0;
        locals.var_qbu_dn2 = assign16630_e24221_d_n2;
        locals.var_qbu_dn6 = assign16630_e24221_d_n6;
        locals.var_qbu_dn7 = assign16630_e24221_d_n7;
        locals.var_qbu_dn10 = assign16630_e24221_d_n10;
        locals.var_qbu_dn11 = assign16630_e24221_d_n11;
        locals.var_qbu_dn12 = assign16630_e24221_d_n12;
        locals.var_qbu_dn17 = assign16630_e24221_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16640_e24230, assign16640_e24230_d_n0, assign16640_e24230_d_n2, assign16640_e24230_d_n6, assign16640_e24230_d_n7, assign16640_e24230_d_n10, assign16640_e24230_d_n11, assign16640_e24230_d_n12, assign16640_e24230_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16640_e24228: f64 = (2.0 * locals.var_fac1);
        (assign16640_e24228, (2.0 * locals.var_fac1_dn0), (2.0 * locals.var_fac1_dn2), (2.0 * locals.var_fac1_dn6), (2.0 * locals.var_fac1_dn7), (2.0 * locals.var_fac1_dn10), (2.0 * locals.var_fac1_dn11), (2.0 * locals.var_fac1_dn12), (2.0 * locals.var_fac1_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16640_e24230;
        locals.var_t1_dn0 = assign16640_e24230_d_n0;
        locals.var_t1_dn2 = assign16640_e24230_d_n2;
        locals.var_t1_dn6 = assign16640_e24230_d_n6;
        locals.var_t1_dn7 = assign16640_e24230_d_n7;
        locals.var_t1_dn10 = assign16640_e24230_d_n10;
        locals.var_t1_dn11 = assign16640_e24230_d_n11;
        locals.var_t1_dn12 = assign16640_e24230_d_n12;
        locals.var_t1_dn17 = assign16640_e24230_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16650_e24241, assign16650_e24241_d_n0, assign16650_e24241_d_n2, assign16650_e24241_d_n6, assign16650_e24241_d_n7, assign16650_e24241_d_n10, assign16650_e24241_d_n11, assign16650_e24241_d_n12, assign16650_e24241_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16650_e24238: f64 = (locals.var_f10 - locals.var_xi0p12);
        let assign16650_e24239: f64 = (locals.var_t1 * assign16650_e24238);
        (assign16650_e24239, ((locals.var_t1_dn0 * assign16650_e24238) + (locals.var_t1 * (locals.var_f10_dn0 - locals.var_xi0p12_dn0))), ((locals.var_t1_dn2 * assign16650_e24238) + (locals.var_t1 * (locals.var_f10_dn2 - locals.var_xi0p12_dn2))), ((locals.var_t1_dn6 * assign16650_e24238) + (locals.var_t1 * (locals.var_f10_dn6 - locals.var_xi0p12_dn6))), ((locals.var_t1_dn7 * assign16650_e24238) + (locals.var_t1 * (locals.var_f10_dn7 - locals.var_xi0p12_dn7))), ((locals.var_t1_dn10 * assign16650_e24238) + (locals.var_t1 * (locals.var_f10_dn10 - locals.var_xi0p12_dn10))), ((locals.var_t1_dn11 * assign16650_e24238) + (locals.var_t1 * (locals.var_f10_dn11 - locals.var_xi0p12_dn11))), ((locals.var_t1_dn12 * assign16650_e24238) + (locals.var_t1 * (locals.var_f10_dn12 - locals.var_xi0p12_dn12))), ((locals.var_t1_dn17 * assign16650_e24238) + (locals.var_t1 * (locals.var_f10_dn17 - locals.var_xi0p12_dn17))),)
    } else {
        (locals.var_dtpds, locals.var_dtpds_dn0, locals.var_dtpds_dn2, locals.var_dtpds_dn6, locals.var_dtpds_dn7, locals.var_dtpds_dn10, locals.var_dtpds_dn11, locals.var_dtpds_dn12, locals.var_dtpds_dn17,)
    }
};
        locals.var_dtpds = assign16650_e24241;
        locals.var_dtpds_dn0 = assign16650_e24241_d_n0;
        locals.var_dtpds_dn2 = assign16650_e24241_d_n2;
        locals.var_dtpds_dn6 = assign16650_e24241_d_n6;
        locals.var_dtpds_dn7 = assign16650_e24241_d_n7;
        locals.var_dtpds_dn10 = assign16650_e24241_d_n10;
        locals.var_dtpds_dn11 = assign16650_e24241_d_n11;
        locals.var_dtpds_dn12 = assign16650_e24241_d_n12;
        locals.var_dtpds_dn17 = assign16650_e24241_d_n17;
        locals.var_dtpds_rv = 0.0;

        let (assign16660_e24250, assign16660_e24250_d_n0, assign16660_e24250_d_n2, assign16660_e24250_d_n6, assign16660_e24250_d_n7, assign16660_e24250_d_n10, assign16660_e24250_d_n11, assign16660_e24250_d_n12, assign16660_e24250_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16660_e24248: f64 = (locals.var_pds + locals.var_dtpds);
        (assign16660_e24248, (locals.var_pds_dn0 + locals.var_dtpds_dn0), (locals.var_pds_dn2 + locals.var_dtpds_dn2), (locals.var_pds_dn6 + locals.var_dtpds_dn6), (locals.var_pds_dn7 + locals.var_dtpds_dn7), (locals.var_pds_dn10 + locals.var_dtpds_dn10), (locals.var_pds_dn11 + locals.var_dtpds_dn11), (locals.var_pds_dn12 + locals.var_dtpds_dn12), (locals.var_pds_dn17 + locals.var_dtpds_dn17),)
    } else {
        (locals.var_achi, locals.var_achi_dn0, locals.var_achi_dn2, locals.var_achi_dn6, locals.var_achi_dn7, locals.var_achi_dn10, locals.var_achi_dn11, locals.var_achi_dn12, locals.var_achi_dn17,)
    }
};
        locals.var_achi = assign16660_e24250;
        locals.var_achi_dn0 = assign16660_e24250_d_n0;
        locals.var_achi_dn2 = assign16660_e24250_d_n2;
        locals.var_achi_dn6 = assign16660_e24250_d_n6;
        locals.var_achi_dn7 = assign16660_e24250_d_n7;
        locals.var_achi_dn10 = assign16660_e24250_d_n10;
        locals.var_achi_dn11 = assign16660_e24250_d_n11;
        locals.var_achi_dn12 = assign16660_e24250_d_n12;
        locals.var_achi_dn17 = assign16660_e24250_d_n17;
        locals.var_achi_rv = 0.0;

        let (assign16670_e24259, assign16670_e24259_d_n0, assign16670_e24259_d_n2, assign16670_e24259_d_n6, assign16670_e24259_d_n7, assign16670_e24259_d_n10, assign16670_e24259_d_n11, assign16670_e24259_d_n12, assign16670_e24259_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16670_e24257: f64 = (1.0 / locals.var_vgvt);
        (assign16670_e24257, (-(locals.var_vgvt_dn0 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn2 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn6 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn7 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn10 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn11 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn12 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn17 / (locals.var_vgvt * locals.var_vgvt))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16670_e24259;
        locals.var_t1_dn0 = assign16670_e24259_d_n0;
        locals.var_t1_dn2 = assign16670_e24259_d_n2;
        locals.var_t1_dn6 = assign16670_e24259_d_n6;
        locals.var_t1_dn7 = assign16670_e24259_d_n7;
        locals.var_t1_dn10 = assign16670_e24259_d_n10;
        locals.var_t1_dn11 = assign16670_e24259_d_n11;
        locals.var_t1_dn12 = assign16670_e24259_d_n12;
        locals.var_t1_dn17 = assign16670_e24259_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16680_e24268, assign16680_e24268_d_n0, assign16680_e24268_d_n2, assign16680_e24268_d_n6, assign16680_e24268_d_n7, assign16680_e24268_d_n10, assign16680_e24268_d_n11, assign16680_e24268_d_n12, assign16680_e24268_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16680_e24266: f64 = (locals.var_achi * locals.var_t1);
        (assign16680_e24266, ((locals.var_achi_dn0 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn0)), ((locals.var_achi_dn2 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn2)), ((locals.var_achi_dn6 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn6)), ((locals.var_achi_dn7 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn7)), ((locals.var_achi_dn10 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn10)), ((locals.var_achi_dn11 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn11)), ((locals.var_achi_dn12 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn12)), ((locals.var_achi_dn17 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign16680_e24268;
        locals.var_t2_dn0 = assign16680_e24268_d_n0;
        locals.var_t2_dn2 = assign16680_e24268_d_n2;
        locals.var_t2_dn6 = assign16680_e24268_d_n6;
        locals.var_t2_dn7 = assign16680_e24268_d_n7;
        locals.var_t2_dn10 = assign16680_e24268_d_n10;
        locals.var_t2_dn11 = assign16680_e24268_d_n11;
        locals.var_t2_dn12 = assign16680_e24268_d_n12;
        locals.var_t2_dn17 = assign16680_e24268_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign16690_e24277, assign16690_e24277_d_n0, assign16690_e24277_d_n2, assign16690_e24277_d_n6, assign16690_e24277_d_n7, assign16690_e24277_d_n10, assign16690_e24277_d_n11, assign16690_e24277_d_n12, assign16690_e24277_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16690_e24275: f64 = (1.0 - locals.var_t2);
        (assign16690_e24275, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn12), (-locals.var_t2_dn17),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign16690_e24277;
        locals.var_t3_dn0 = assign16690_e24277_d_n0;
        locals.var_t3_dn2 = assign16690_e24277_d_n2;
        locals.var_t3_dn6 = assign16690_e24277_d_n6;
        locals.var_t3_dn7 = assign16690_e24277_d_n7;
        locals.var_t3_dn10 = assign16690_e24277_d_n10;
        locals.var_t3_dn11 = assign16690_e24277_d_n11;
        locals.var_t3_dn12 = assign16690_e24277_d_n12;
        locals.var_t3_dn17 = assign16690_e24277_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign16700_e24286, assign16700_e24286_d_n0, assign16700_e24286_d_n2, assign16700_e24286_d_n6, assign16700_e24286_d_n7, assign16700_e24286_d_n10, assign16700_e24286_d_n11, assign16700_e24286_d_n12, assign16700_e24286_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16700_e24284: f64 = (1.0 - locals.var_t3);
        (assign16700_e24284, (-locals.var_t3_dn0), (-locals.var_t3_dn2), (-locals.var_t3_dn6), (-locals.var_t3_dn7), (-locals.var_t3_dn10), (-locals.var_t3_dn11), (-locals.var_t3_dn12), (-locals.var_t3_dn17),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign16700_e24286;
        locals.var_tx_dn0 = assign16700_e24286_d_n0;
        locals.var_tx_dn2 = assign16700_e24286_d_n2;
        locals.var_tx_dn6 = assign16700_e24286_d_n6;
        locals.var_tx_dn7 = assign16700_e24286_d_n7;
        locals.var_tx_dn10 = assign16700_e24286_d_n10;
        locals.var_tx_dn11 = assign16700_e24286_d_n11;
        locals.var_tx_dn12 = assign16700_e24286_d_n12;
        locals.var_tx_dn17 = assign16700_e24286_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign16710_e24295, assign16710_e24295_d_n0, assign16710_e24295_d_n2, assign16710_e24295_d_n6, assign16710_e24295_d_n7, assign16710_e24295_d_n10, assign16710_e24295_d_n11, assign16710_e24295_d_n12, assign16710_e24295_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16710_e24293: f64 = (locals.var_tx * locals.var_tx);
        (assign16710_e24293, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)), ((locals.var_tx_dn12 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn12)), ((locals.var_tx_dn17 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign16710_e24295;
        locals.var_x2_dn0 = assign16710_e24295_d_n0;
        locals.var_x2_dn2 = assign16710_e24295_d_n2;
        locals.var_x2_dn6 = assign16710_e24295_d_n6;
        locals.var_x2_dn7 = assign16710_e24295_d_n7;
        locals.var_x2_dn10 = assign16710_e24295_d_n10;
        locals.var_x2_dn11 = assign16710_e24295_d_n11;
        locals.var_x2_dn12 = assign16710_e24295_d_n12;
        locals.var_x2_dn17 = assign16710_e24295_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign16720_e24304, assign16720_e24304_d_n0, assign16720_e24304_d_n2, assign16720_e24304_d_n6, assign16720_e24304_d_n7, assign16720_e24304_d_n10, assign16720_e24304_d_n11, assign16720_e24304_d_n12, assign16720_e24304_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16720_e24302: f64 = 1.0;
        (assign16720_e24302, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign16720_e24304;
        locals.var_xmax2_dn0 = assign16720_e24304_d_n0;
        locals.var_xmax2_dn2 = assign16720_e24304_d_n2;
        locals.var_xmax2_dn6 = assign16720_e24304_d_n6;
        locals.var_xmax2_dn7 = assign16720_e24304_d_n7;
        locals.var_xmax2_dn10 = assign16720_e24304_d_n10;
        locals.var_xmax2_dn11 = assign16720_e24304_d_n11;
        locals.var_xmax2_dn12 = assign16720_e24304_d_n12;
        locals.var_xmax2_dn17 = assign16720_e24304_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign16730_e24311, assign16730_e24311_d_n0, assign16730_e24311_d_n2, assign16730_e24311_d_n6, assign16730_e24311_d_n7, assign16730_e24311_d_n10, assign16730_e24311_d_n11, assign16730_e24311_d_n12, assign16730_e24311_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16730_e24311;
        locals.var_xp_dn0 = assign16730_e24311_d_n0;
        locals.var_xp_dn2 = assign16730_e24311_d_n2;
        locals.var_xp_dn6 = assign16730_e24311_d_n6;
        locals.var_xp_dn7 = assign16730_e24311_d_n7;
        locals.var_xp_dn10 = assign16730_e24311_d_n10;
        locals.var_xp_dn11 = assign16730_e24311_d_n11;
        locals.var_xp_dn12 = assign16730_e24311_d_n12;
        locals.var_xp_dn17 = assign16730_e24311_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16740_e24318, assign16740_e24318_d_n0, assign16740_e24318_d_n2, assign16740_e24318_d_n6, assign16740_e24318_d_n7, assign16740_e24318_d_n10, assign16740_e24318_d_n11, assign16740_e24318_d_n12, assign16740_e24318_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16740_e24318;
        locals.var_xmp_dn0 = assign16740_e24318_d_n0;
        locals.var_xmp_dn2 = assign16740_e24318_d_n2;
        locals.var_xmp_dn6 = assign16740_e24318_d_n6;
        locals.var_xmp_dn7 = assign16740_e24318_d_n7;
        locals.var_xmp_dn10 = assign16740_e24318_d_n10;
        locals.var_xmp_dn11 = assign16740_e24318_d_n11;
        locals.var_xmp_dn12 = assign16740_e24318_d_n12;
        locals.var_xmp_dn17 = assign16740_e24318_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16750_e24325,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign16750_e24325;
        locals.var_m0_rv = 0.0;

        let (assign16760_e24332,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16760_e24332;
        locals.var_mm_rv = 0.0;

        let (assign16770_e24339, assign16770_e24339_d_n0, assign16770_e24339_d_n2, assign16770_e24339_d_n6, assign16770_e24339_d_n7, assign16770_e24339_d_n10, assign16770_e24339_d_n11, assign16770_e24339_d_n12, assign16770_e24339_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign16770_e24339;
        locals.var_arg_dn0 = assign16770_e24339_d_n0;
        locals.var_arg_dn2 = assign16770_e24339_d_n2;
        locals.var_arg_dn6 = assign16770_e24339_d_n6;
        locals.var_arg_dn7 = assign16770_e24339_d_n7;
        locals.var_arg_dn10 = assign16770_e24339_d_n10;
        locals.var_arg_dn11 = assign16770_e24339_d_n11;
        locals.var_arg_dn12 = assign16770_e24339_d_n12;
        locals.var_arg_dn17 = assign16770_e24339_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign16780_e24346, assign16780_e24346_d_n0, assign16780_e24346_d_n2, assign16780_e24346_d_n6, assign16780_e24346_d_n7, assign16780_e24346_d_n10, assign16780_e24346_d_n11, assign16780_e24346_d_n12, assign16780_e24346_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16780_e24346;
        locals.var_dnm_dn0 = assign16780_e24346_d_n0;
        locals.var_dnm_dn2 = assign16780_e24346_d_n2;
        locals.var_dnm_dn6 = assign16780_e24346_d_n6;
        locals.var_dnm_dn7 = assign16780_e24346_d_n7;
        locals.var_dnm_dn10 = assign16780_e24346_d_n10;
        locals.var_dnm_dn11 = assign16780_e24346_d_n11;
        locals.var_dnm_dn12 = assign16780_e24346_d_n12;
        locals.var_dnm_dn17 = assign16780_e24346_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign16790_e24355, assign16790_e24355_d_n0, assign16790_e24355_d_n2, assign16790_e24355_d_n6, assign16790_e24355_d_n7, assign16790_e24355_d_n10, assign16790_e24355_d_n11, assign16790_e24355_d_n12, assign16790_e24355_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16790_e24353: f64 = (locals.var_xp * locals.var_x2);
        (assign16790_e24353, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16790_e24355;
        locals.var_xp_dn0 = assign16790_e24355_d_n0;
        locals.var_xp_dn2 = assign16790_e24355_d_n2;
        locals.var_xp_dn6 = assign16790_e24355_d_n6;
        locals.var_xp_dn7 = assign16790_e24355_d_n7;
        locals.var_xp_dn10 = assign16790_e24355_d_n10;
        locals.var_xp_dn11 = assign16790_e24355_d_n11;
        locals.var_xp_dn12 = assign16790_e24355_d_n12;
        locals.var_xp_dn17 = assign16790_e24355_d_n17;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_60(
        locals: &mut StampLocals,
    ) {
        let (assign16800_e24364, assign16800_e24364_d_n0, assign16800_e24364_d_n2, assign16800_e24364_d_n6, assign16800_e24364_d_n7, assign16800_e24364_d_n10, assign16800_e24364_d_n11, assign16800_e24364_d_n12, assign16800_e24364_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16800_e24362: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16800_e24362, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16800_e24364;
        locals.var_xmp_dn0 = assign16800_e24364_d_n0;
        locals.var_xmp_dn2 = assign16800_e24364_d_n2;
        locals.var_xmp_dn6 = assign16800_e24364_d_n6;
        locals.var_xmp_dn7 = assign16800_e24364_d_n7;
        locals.var_xmp_dn10 = assign16800_e24364_d_n10;
        locals.var_xmp_dn11 = assign16800_e24364_d_n11;
        locals.var_xmp_dn12 = assign16800_e24364_d_n12;
        locals.var_xmp_dn17 = assign16800_e24364_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16810_e24373, assign16810_e24373_d_n0, assign16810_e24373_d_n2, assign16810_e24373_d_n6, assign16810_e24373_d_n7, assign16810_e24373_d_n10, assign16810_e24373_d_n11, assign16810_e24373_d_n12, assign16810_e24373_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16810_e24371: f64 = (locals.var_xp * locals.var_x2);
        (assign16810_e24371, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16810_e24373;
        locals.var_xp_dn0 = assign16810_e24373_d_n0;
        locals.var_xp_dn2 = assign16810_e24373_d_n2;
        locals.var_xp_dn6 = assign16810_e24373_d_n6;
        locals.var_xp_dn7 = assign16810_e24373_d_n7;
        locals.var_xp_dn10 = assign16810_e24373_d_n10;
        locals.var_xp_dn11 = assign16810_e24373_d_n11;
        locals.var_xp_dn12 = assign16810_e24373_d_n12;
        locals.var_xp_dn17 = assign16810_e24373_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16820_e24382, assign16820_e24382_d_n0, assign16820_e24382_d_n2, assign16820_e24382_d_n6, assign16820_e24382_d_n7, assign16820_e24382_d_n10, assign16820_e24382_d_n11, assign16820_e24382_d_n12, assign16820_e24382_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16820_e24380: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16820_e24380, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16820_e24382;
        locals.var_xmp_dn0 = assign16820_e24382_d_n0;
        locals.var_xmp_dn2 = assign16820_e24382_d_n2;
        locals.var_xmp_dn6 = assign16820_e24382_d_n6;
        locals.var_xmp_dn7 = assign16820_e24382_d_n7;
        locals.var_xmp_dn10 = assign16820_e24382_d_n10;
        locals.var_xmp_dn11 = assign16820_e24382_d_n11;
        locals.var_xmp_dn12 = assign16820_e24382_d_n12;
        locals.var_xmp_dn17 = assign16820_e24382_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16830_e24391, assign16830_e24391_d_n0, assign16830_e24391_d_n2, assign16830_e24391_d_n6, assign16830_e24391_d_n7, assign16830_e24391_d_n10, assign16830_e24391_d_n11, assign16830_e24391_d_n12, assign16830_e24391_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16830_e24389: f64 = (locals.var_xp * locals.var_x2);
        (assign16830_e24389, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16830_e24391;
        locals.var_xp_dn0 = assign16830_e24391_d_n0;
        locals.var_xp_dn2 = assign16830_e24391_d_n2;
        locals.var_xp_dn6 = assign16830_e24391_d_n6;
        locals.var_xp_dn7 = assign16830_e24391_d_n7;
        locals.var_xp_dn10 = assign16830_e24391_d_n10;
        locals.var_xp_dn11 = assign16830_e24391_d_n11;
        locals.var_xp_dn12 = assign16830_e24391_d_n12;
        locals.var_xp_dn17 = assign16830_e24391_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16840_e24400, assign16840_e24400_d_n0, assign16840_e24400_d_n2, assign16840_e24400_d_n6, assign16840_e24400_d_n7, assign16840_e24400_d_n10, assign16840_e24400_d_n11, assign16840_e24400_d_n12, assign16840_e24400_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16840_e24398: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16840_e24398, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16840_e24400;
        locals.var_xmp_dn0 = assign16840_e24400_d_n0;
        locals.var_xmp_dn2 = assign16840_e24400_d_n2;
        locals.var_xmp_dn6 = assign16840_e24400_d_n6;
        locals.var_xmp_dn7 = assign16840_e24400_d_n7;
        locals.var_xmp_dn10 = assign16840_e24400_d_n10;
        locals.var_xmp_dn11 = assign16840_e24400_d_n11;
        locals.var_xmp_dn12 = assign16840_e24400_d_n12;
        locals.var_xmp_dn17 = assign16840_e24400_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16850_e24409, assign16850_e24409_d_n0, assign16850_e24409_d_n2, assign16850_e24409_d_n6, assign16850_e24409_d_n7, assign16850_e24409_d_n10, assign16850_e24409_d_n11, assign16850_e24409_d_n12, assign16850_e24409_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16850_e24407: f64 = (locals.var_xp * locals.var_x2);
        (assign16850_e24407, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16850_e24409;
        locals.var_xp_dn0 = assign16850_e24409_d_n0;
        locals.var_xp_dn2 = assign16850_e24409_d_n2;
        locals.var_xp_dn6 = assign16850_e24409_d_n6;
        locals.var_xp_dn7 = assign16850_e24409_d_n7;
        locals.var_xp_dn10 = assign16850_e24409_d_n10;
        locals.var_xp_dn11 = assign16850_e24409_d_n11;
        locals.var_xp_dn12 = assign16850_e24409_d_n12;
        locals.var_xp_dn17 = assign16850_e24409_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16860_e24418, assign16860_e24418_d_n0, assign16860_e24418_d_n2, assign16860_e24418_d_n6, assign16860_e24418_d_n7, assign16860_e24418_d_n10, assign16860_e24418_d_n11, assign16860_e24418_d_n12, assign16860_e24418_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16860_e24416: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16860_e24416, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16860_e24418;
        locals.var_xmp_dn0 = assign16860_e24418_d_n0;
        locals.var_xmp_dn2 = assign16860_e24418_d_n2;
        locals.var_xmp_dn6 = assign16860_e24418_d_n6;
        locals.var_xmp_dn7 = assign16860_e24418_d_n7;
        locals.var_xmp_dn10 = assign16860_e24418_d_n10;
        locals.var_xmp_dn11 = assign16860_e24418_d_n11;
        locals.var_xmp_dn12 = assign16860_e24418_d_n12;
        locals.var_xmp_dn17 = assign16860_e24418_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16870_e24427, assign16870_e24427_d_n0, assign16870_e24427_d_n2, assign16870_e24427_d_n6, assign16870_e24427_d_n7, assign16870_e24427_d_n10, assign16870_e24427_d_n11, assign16870_e24427_d_n12, assign16870_e24427_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign16870_e24425: f64 = (locals.var_xp + locals.var_xmp);
        (assign16870_e24425, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign16870_e24427;
        locals.var_arg_dn0 = assign16870_e24427_d_n0;
        locals.var_arg_dn2 = assign16870_e24427_d_n2;
        locals.var_arg_dn6 = assign16870_e24427_d_n6;
        locals.var_arg_dn7 = assign16870_e24427_d_n7;
        locals.var_arg_dn10 = assign16870_e24427_d_n10;
        locals.var_arg_dn11 = assign16870_e24427_d_n11;
        locals.var_arg_dn12 = assign16870_e24427_d_n12;
        locals.var_arg_dn17 = assign16870_e24427_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign16880_e24434, assign16880_e24434_d_n0, assign16880_e24434_d_n2, assign16880_e24434_d_n6, assign16880_e24434_d_n7, assign16880_e24434_d_n10, assign16880_e24434_d_n11, assign16880_e24434_d_n12, assign16880_e24434_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16880_e24434;
        locals.var_dnm_dn0 = assign16880_e24434_d_n0;
        locals.var_dnm_dn2 = assign16880_e24434_d_n2;
        locals.var_dnm_dn6 = assign16880_e24434_d_n6;
        locals.var_dnm_dn7 = assign16880_e24434_d_n7;
        locals.var_dnm_dn10 = assign16880_e24434_d_n10;
        locals.var_dnm_dn11 = assign16880_e24434_d_n11;
        locals.var_dnm_dn12 = assign16880_e24434_d_n12;
        locals.var_dnm_dn17 = assign16880_e24434_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign16890_e24449: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard497 = assign16890_e24449;
        locals.var_guard497_rv = 0.0;

        let assign16900_e24452: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard498 = assign16900_e24452;
        locals.var_guard498_rv = 0.0;

        let (assign16910_e24463,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16910_e24463;
        locals.var_mm_rv = 0.0;

        let assign16920_e24466: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard499 = assign16920_e24466;
        locals.var_guard499_rv = 0.0;

        let (assign16930_e24480,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard499 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16930_e24480;
        locals.var_mm_rv = 0.0;

        let assign16940_e24483: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard500 = assign16940_e24483;
        locals.var_guard500_rv = 0.0;

        let (assign16950_e24500,) = {
    if ((((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard500 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16950_e24500;
        locals.var_mm_rv = 0.0;

        let assign16960_e24503: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard501 = assign16960_e24503;
        locals.var_guard501_rv = 0.0;

        let (assign16970_e24523,) = {
    if (((((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard500 == 0.0)) && (locals.var_guard501 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16970_e24523;
        locals.var_mm_rv = 0.0;

        let (assign16980_e24532,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard497 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign16980_e24532;
        locals.var_m0_rv = 0.0;

        let mut assign16990_loop_guard: usize = 0;
        while {
            let assign16990_cond_e24542: f64 = if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign16990_cond_e24542 != 0.0
        } {
            assign16990_loop_guard += 1;
            assert!(assign16990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign16990_body0_e24552, assign16990_body0_e24552_d_n0, assign16990_body0_e24552_d_n2, assign16990_body0_e24552_d_n6, assign16990_body0_e24552_d_n7, assign16990_body0_e24552_d_n10, assign16990_body0_e24552_d_n11, assign16990_body0_e24552_d_n12, assign16990_body0_e24552_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard497 != 0.0)) {
        let assign16990_body0_e24550: f64 = (locals.var_dnm).sqrt();
        (assign16990_body0_e24550, (locals.var_dnm_dn0 / (2.0 * assign16990_body0_e24550)), (locals.var_dnm_dn2 / (2.0 * assign16990_body0_e24550)), (locals.var_dnm_dn6 / (2.0 * assign16990_body0_e24550)), (locals.var_dnm_dn7 / (2.0 * assign16990_body0_e24550)), (locals.var_dnm_dn10 / (2.0 * assign16990_body0_e24550)), (locals.var_dnm_dn11 / (2.0 * assign16990_body0_e24550)), (locals.var_dnm_dn12 / (2.0 * assign16990_body0_e24550)), (locals.var_dnm_dn17 / (2.0 * assign16990_body0_e24550)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign16990_body0_e24552;
            locals.var_dnm_dn0 = assign16990_body0_e24552_d_n0;
            locals.var_dnm_dn2 = assign16990_body0_e24552_d_n2;
            locals.var_dnm_dn6 = assign16990_body0_e24552_d_n6;
            locals.var_dnm_dn7 = assign16990_body0_e24552_d_n7;
            locals.var_dnm_dn10 = assign16990_body0_e24552_d_n10;
            locals.var_dnm_dn11 = assign16990_body0_e24552_d_n11;
            locals.var_dnm_dn12 = assign16990_body0_e24552_d_n12;
            locals.var_dnm_dn17 = assign16990_body0_e24552_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign16990_body1_e24563,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard497 != 0.0)) {
        let assign16990_body1_e24561: f64 = (locals.var_m0 + 1.0);
        (assign16990_body1_e24561,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign16990_body1_e24563;
            locals.var_m0_rv = 0.0;
        }

        let (assign17000_e24579, assign17000_e24579_d_n0, assign17000_e24579_d_n2, assign17000_e24579_d_n6, assign17000_e24579_d_n7, assign17000_e24579_d_n10, assign17000_e24579_d_n11, assign17000_e24579_d_n12, assign17000_e24579_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard497 == 0.0)) {
        let assign17000_e24575: f64 = (2.0 * 4.0);
        let assign17000_e24576: f64 = (1.0 / assign17000_e24575);
        let assign17000_e24577: f64 = (locals.var_dnm).powf(assign17000_e24576);
        (assign17000_e24577, if 0.0 == 0.0 && ((assign17000_e24576) as f64).is_finite() && ((assign17000_e24576) as f64).fract() == 0.0 { if assign17000_e24576 == 0.0 { 0.0 } else { (assign17000_e24576 * ((locals.var_dnm).powf(assign17000_e24576 - 1.0) * locals.var_dnm_dn0)) } } else { (assign17000_e24577 * (assign17000_e24576 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17000_e24576) as f64).is_finite() && ((assign17000_e24576) as f64).fract() == 0.0 { if assign17000_e24576 == 0.0 { 0.0 } else { (assign17000_e24576 * ((locals.var_dnm).powf(assign17000_e24576 - 1.0) * locals.var_dnm_dn2)) } } else { (assign17000_e24577 * (assign17000_e24576 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17000_e24576) as f64).is_finite() && ((assign17000_e24576) as f64).fract() == 0.0 { if assign17000_e24576 == 0.0 { 0.0 } else { (assign17000_e24576 * ((locals.var_dnm).powf(assign17000_e24576 - 1.0) * locals.var_dnm_dn6)) } } else { (assign17000_e24577 * (assign17000_e24576 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17000_e24576) as f64).is_finite() && ((assign17000_e24576) as f64).fract() == 0.0 { if assign17000_e24576 == 0.0 { 0.0 } else { (assign17000_e24576 * ((locals.var_dnm).powf(assign17000_e24576 - 1.0) * locals.var_dnm_dn7)) } } else { (assign17000_e24577 * (assign17000_e24576 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17000_e24576) as f64).is_finite() && ((assign17000_e24576) as f64).fract() == 0.0 { if assign17000_e24576 == 0.0 { 0.0 } else { (assign17000_e24576 * ((locals.var_dnm).powf(assign17000_e24576 - 1.0) * locals.var_dnm_dn10)) } } else { (assign17000_e24577 * (assign17000_e24576 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17000_e24576) as f64).is_finite() && ((assign17000_e24576) as f64).fract() == 0.0 { if assign17000_e24576 == 0.0 { 0.0 } else { (assign17000_e24576 * ((locals.var_dnm).powf(assign17000_e24576 - 1.0) * locals.var_dnm_dn11)) } } else { (assign17000_e24577 * (assign17000_e24576 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17000_e24576) as f64).is_finite() && ((assign17000_e24576) as f64).fract() == 0.0 { if assign17000_e24576 == 0.0 { 0.0 } else { (assign17000_e24576 * ((locals.var_dnm).powf(assign17000_e24576 - 1.0) * locals.var_dnm_dn12)) } } else { (assign17000_e24577 * (assign17000_e24576 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17000_e24576) as f64).is_finite() && ((assign17000_e24576) as f64).fract() == 0.0 { if assign17000_e24576 == 0.0 { 0.0 } else { (assign17000_e24576 * ((locals.var_dnm).powf(assign17000_e24576 - 1.0) * locals.var_dnm_dn17)) } } else { (assign17000_e24577 * (assign17000_e24576 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign17000_e24579;
        locals.var_dnm_dn0 = assign17000_e24579_d_n0;
        locals.var_dnm_dn2 = assign17000_e24579_d_n2;
        locals.var_dnm_dn6 = assign17000_e24579_d_n6;
        locals.var_dnm_dn7 = assign17000_e24579_d_n7;
        locals.var_dnm_dn10 = assign17000_e24579_d_n10;
        locals.var_dnm_dn11 = assign17000_e24579_d_n11;
        locals.var_dnm_dn12 = assign17000_e24579_d_n12;
        locals.var_dnm_dn17 = assign17000_e24579_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign17010_e24588, assign17010_e24588_d_n0, assign17010_e24588_d_n2, assign17010_e24588_d_n6, assign17010_e24588_d_n7, assign17010_e24588_d_n10, assign17010_e24588_d_n11, assign17010_e24588_d_n12, assign17010_e24588_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign17010_e24586: f64 = (1.0 / locals.var_dnm);
        (assign17010_e24586, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign17010_e24588;
        locals.var_dnm_dn0 = assign17010_e24588_d_n0;
        locals.var_dnm_dn2 = assign17010_e24588_d_n2;
        locals.var_dnm_dn6 = assign17010_e24588_d_n6;
        locals.var_dnm_dn7 = assign17010_e24588_d_n7;
        locals.var_dnm_dn10 = assign17010_e24588_d_n10;
        locals.var_dnm_dn11 = assign17010_e24588_d_n11;
        locals.var_dnm_dn12 = assign17010_e24588_d_n12;
        locals.var_dnm_dn17 = assign17010_e24588_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign17020_e24599, assign17020_e24599_d_n0, assign17020_e24599_d_n2, assign17020_e24599_d_n6, assign17020_e24599_d_n7, assign17020_e24599_d_n10, assign17020_e24599_d_n11, assign17020_e24599_d_n12, assign17020_e24599_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign17020_e24595: f64 = locals.var_tx;
        let assign17020_e24597: f64 = (assign17020_e24595 * locals.var_dnm);
        (assign17020_e24597, ((locals.var_tx_dn0 * locals.var_dnm) + (assign17020_e24595 * locals.var_dnm_dn0)), ((locals.var_tx_dn2 * locals.var_dnm) + (assign17020_e24595 * locals.var_dnm_dn2)), ((locals.var_tx_dn6 * locals.var_dnm) + (assign17020_e24595 * locals.var_dnm_dn6)), ((locals.var_tx_dn7 * locals.var_dnm) + (assign17020_e24595 * locals.var_dnm_dn7)), ((locals.var_tx_dn10 * locals.var_dnm) + (assign17020_e24595 * locals.var_dnm_dn10)), ((locals.var_tx_dn11 * locals.var_dnm) + (assign17020_e24595 * locals.var_dnm_dn11)), ((locals.var_tx_dn12 * locals.var_dnm) + (assign17020_e24595 * locals.var_dnm_dn12)), ((locals.var_tx_dn17 * locals.var_dnm) + (assign17020_e24595 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign17020_e24599;
        locals.var_ty_dn0 = assign17020_e24599_d_n0;
        locals.var_ty_dn2 = assign17020_e24599_d_n2;
        locals.var_ty_dn6 = assign17020_e24599_d_n6;
        locals.var_ty_dn7 = assign17020_e24599_d_n7;
        locals.var_ty_dn10 = assign17020_e24599_d_n10;
        locals.var_ty_dn11 = assign17020_e24599_d_n11;
        locals.var_ty_dn12 = assign17020_e24599_d_n12;
        locals.var_ty_dn17 = assign17020_e24599_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign17030_e24608, assign17030_e24608_d_n0, assign17030_e24608_d_n2, assign17030_e24608_d_n6, assign17030_e24608_d_n7, assign17030_e24608_d_n10, assign17030_e24608_d_n11, assign17030_e24608_d_n12, assign17030_e24608_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign17030_e24606: f64 = (1.0 - locals.var_ty);
        (assign17030_e24606, (-locals.var_ty_dn0), (-locals.var_ty_dn2), (-locals.var_ty_dn6), (-locals.var_ty_dn7), (-locals.var_ty_dn10), (-locals.var_ty_dn11), (-locals.var_ty_dn12), (-locals.var_ty_dn17),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    }
};
        locals.var_alpha = assign17030_e24608;
        locals.var_alpha_dn0 = assign17030_e24608_d_n0;
        locals.var_alpha_dn2 = assign17030_e24608_d_n2;
        locals.var_alpha_dn6 = assign17030_e24608_d_n6;
        locals.var_alpha_dn7 = assign17030_e24608_d_n7;
        locals.var_alpha_dn10 = assign17030_e24608_d_n10;
        locals.var_alpha_dn11 = assign17030_e24608_d_n11;
        locals.var_alpha_dn12 = assign17030_e24608_d_n12;
        locals.var_alpha_dn17 = assign17030_e24608_d_n17;
        locals.var_alpha_rv = 0.0;

        let (assign17040_e24621, assign17040_e24621_d_n0, assign17040_e24621_d_n2, assign17040_e24621_d_n6, assign17040_e24621_d_n7, assign17040_e24621_d_n10, assign17040_e24621_d_n11, assign17040_e24621_d_n12, assign17040_e24621_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign17040_e24617: f64 = (1.0 + locals.var_alpha);
        let assign17040_e24618: f64 = (locals.var_alpha * assign17040_e24617);
        let assign17040_e24619: f64 = (1.0 + assign17040_e24618);
        (assign17040_e24619, ((locals.var_alpha_dn0 * assign17040_e24617) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign17040_e24617) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * assign17040_e24617) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign17040_e24617) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * assign17040_e24617) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign17040_e24617) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * assign17040_e24617) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * assign17040_e24617) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn12, locals.var_qinm_dn17,)
    }
};
        locals.var_qinm = assign17040_e24621;
        locals.var_qinm_dn0 = assign17040_e24621_d_n0;
        locals.var_qinm_dn2 = assign17040_e24621_d_n2;
        locals.var_qinm_dn6 = assign17040_e24621_d_n6;
        locals.var_qinm_dn7 = assign17040_e24621_d_n7;
        locals.var_qinm_dn10 = assign17040_e24621_d_n10;
        locals.var_qinm_dn11 = assign17040_e24621_d_n11;
        locals.var_qinm_dn12 = assign17040_e24621_d_n12;
        locals.var_qinm_dn17 = assign17040_e24621_d_n17;
        locals.var_qinm_rv = 0.0;

        let (assign17050_e24641, assign17050_e24641_d_n0, assign17050_e24641_d_n2, assign17050_e24641_d_n6, assign17050_e24641_d_n7, assign17050_e24641_d_n10, assign17050_e24641_d_n11, assign17050_e24641_d_n12, assign17050_e24641_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign17050_e24628: f64 = (1.0 + locals.var_alpha);
        let assign17050_e24631: f64 = (10.0 * 2.220446049250313e-16);
        let (assign17050_e24639, assign17050_e24639_d_n0, assign17050_e24639_d_n2, assign17050_e24639_d_n6, assign17050_e24639_d_n7, assign17050_e24639_d_n10, assign17050_e24639_d_n11, assign17050_e24639_d_n12, assign17050_e24639_d_n17,) = {
            if (assign17050_e24628 >= assign17050_e24631) {
                let assign17050_e24635: f64 = (1.0 + locals.var_alpha);
                (assign17050_e24635, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
            } else {
                let assign17050_e24638: f64 = (10.0 * 2.220446049250313e-16);
                (assign17050_e24638, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign17050_e24639, assign17050_e24639_d_n0, assign17050_e24639_d_n2, assign17050_e24639_d_n6, assign17050_e24639_d_n7, assign17050_e24639_d_n10, assign17050_e24639_d_n11, assign17050_e24639_d_n12, assign17050_e24639_d_n17,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn12, locals.var_qidn_dn17,)
    }
};
        locals.var_qidn = assign17050_e24641;
        locals.var_qidn_dn0 = assign17050_e24641_d_n0;
        locals.var_qidn_dn2 = assign17050_e24641_d_n2;
        locals.var_qidn_dn6 = assign17050_e24641_d_n6;
        locals.var_qidn_dn7 = assign17050_e24641_d_n7;
        locals.var_qidn_dn10 = assign17050_e24641_d_n10;
        locals.var_qidn_dn11 = assign17050_e24641_d_n11;
        locals.var_qidn_dn12 = assign17050_e24641_d_n12;
        locals.var_qidn_dn17 = assign17050_e24641_d_n17;
        locals.var_qidn_rv = 0.0;

        let (assign17060_e24654, assign17060_e24654_d_n0, assign17060_e24654_d_n2, assign17060_e24654_d_n6, assign17060_e24654_d_n7, assign17060_e24654_d_n10, assign17060_e24654_d_n11, assign17060_e24654_d_n12, assign17060_e24654_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) {
        let assign17060_e24648: f64 = (0.6666666666666667 * locals.var_vgvt);
        let assign17060_e24650: f64 = (assign17060_e24648 * locals.var_qinm);
        let assign17060_e24652: f64 = (assign17060_e24650 / locals.var_qidn);
        (assign17060_e24652, ((((((0.6666666666666667 * locals.var_vgvt_dn0) * locals.var_qinm) + (assign17060_e24648 * locals.var_qinm_dn0)) * locals.var_qidn) - (assign17060_e24650 * locals.var_qidn_dn0)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn2) * locals.var_qinm) + (assign17060_e24648 * locals.var_qinm_dn2)) * locals.var_qidn) - (assign17060_e24650 * locals.var_qidn_dn2)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn6) * locals.var_qinm) + (assign17060_e24648 * locals.var_qinm_dn6)) * locals.var_qidn) - (assign17060_e24650 * locals.var_qidn_dn6)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn7) * locals.var_qinm) + (assign17060_e24648 * locals.var_qinm_dn7)) * locals.var_qidn) - (assign17060_e24650 * locals.var_qidn_dn7)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn10) * locals.var_qinm) + (assign17060_e24648 * locals.var_qinm_dn10)) * locals.var_qidn) - (assign17060_e24650 * locals.var_qidn_dn10)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn11) * locals.var_qinm) + (assign17060_e24648 * locals.var_qinm_dn11)) * locals.var_qidn) - (assign17060_e24650 * locals.var_qidn_dn11)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn12) * locals.var_qinm) + (assign17060_e24648 * locals.var_qinm_dn12)) * locals.var_qidn) - (assign17060_e24650 * locals.var_qidn_dn12)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn17) * locals.var_qinm) + (assign17060_e24648 * locals.var_qinm_dn17)) * locals.var_qidn) - (assign17060_e24650 * locals.var_qidn_dn17)) / (locals.var_qidn * locals.var_qidn)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign17060_e24654;
        locals.var_t1_dn0 = assign17060_e24654_d_n0;
        locals.var_t1_dn2 = assign17060_e24654_d_n2;
        locals.var_t1_dn6 = assign17060_e24654_d_n6;
        locals.var_t1_dn7 = assign17060_e24654_d_n7;
        locals.var_t1_dn10 = assign17060_e24654_d_n10;
        locals.var_t1_dn11 = assign17060_e24654_d_n11;
        locals.var_t1_dn12 = assign17060_e24654_d_n12;
        locals.var_t1_dn17 = assign17060_e24654_d_n17;
        locals.var_t1_rv = 0.0;

        let assign17070_e24657: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard502 = assign17070_e24657;
        locals.var_guard502_rv = 0.0;

        let assign17080_e24659: f64 = (locals.var_pds).abs();
        let assign17080_e24661: f64 = if assign17080_e24659 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard503 = assign17080_e24661;
        locals.var_guard503_rv = 0.0;

        let (assign17090_e24714, assign17090_e24714_d_n0, assign17090_e24714_d_n2, assign17090_e24714_d_n6, assign17090_e24714_d_n7, assign17090_e24714_d_n10, assign17090_e24714_d_n11, assign17090_e24714_d_n12, assign17090_e24714_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard502 != 0.0)) && (locals.var_guard503 != 0.0)) {
        let assign17090_e24672: f64 = (locals.var_ai * locals.var_ai);
        let assign17090_e24675: f64 = (locals.var_di * locals.var_di);
        let assign17090_e24677: f64 = (assign17090_e24675 / 12.0);
        let assign17090_e24678: f64 = (assign17090_e24672 + assign17090_e24677);
        let assign17090_e24680: f64 = (assign17090_e24678 * locals.var_beta);
        let assign17090_e24682: f64 = (assign17090_e24680 * locals.var_pds);
        let assign17090_e24685: f64 = (locals.var_ai * locals.var_di);
        let assign17090_e24686: f64 = (assign17090_e24682 - assign17090_e24685);
        let assign17090_e24689: f64 = (2.0 * locals.var_ai);
        let assign17090_e24692: f64 = (locals.var_c_fox / locals.var_beta);
        let assign17090_e24694: f64 = (assign17090_e24692 * locals.var_db);
        let assign17090_e24696: f64 = (assign17090_e24694 * locals.var_db);
        let assign17090_e24698: f64 = (assign17090_e24696 / locals.var_c2);
        let assign17090_e24700: f64 = (assign17090_e24698 / 5.0);
        let assign17090_e24701: f64 = (assign17090_e24689 + assign17090_e24700);
        let assign17090_e24703: f64 = (assign17090_e24701 * locals.var_db);
        let assign17090_e24705: f64 = (assign17090_e24703 * locals.var_db);
        let assign17090_e24707: f64 = (assign17090_e24705 * locals.var_db);
        let assign17090_e24709: f64 = (assign17090_e24707 / locals.var_c2);
        let assign17090_e24711: f64 = (assign17090_e24709 / 6.0);
        let assign17090_e24712: f64 = (assign17090_e24686 - assign17090_e24711);
        (assign17090_e24712, ((((((((locals.var_ai_dn0 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn0)) + (((locals.var_di_dn0 * locals.var_di) + (locals.var_di * locals.var_di_dn0)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17090_e24680 * locals.var_pds_dn0)) - ((locals.var_ai_dn0 * locals.var_di) + (locals.var_ai * locals.var_di_dn0))) - ((((((((((((2.0 * locals.var_ai_dn0) + (((((((((locals.var_c_fox_dn0 / locals.var_beta) * locals.var_db) + (assign17090_e24692 * locals.var_db_dn0)) * locals.var_db) + (assign17090_e24694 * locals.var_db_dn0)) * locals.var_c2) - (assign17090_e24696 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17090_e24701 * locals.var_db_dn0)) * locals.var_db) + (assign17090_e24703 * locals.var_db_dn0)) * locals.var_db) + (assign17090_e24705 * locals.var_db_dn0)) * locals.var_c2) - (assign17090_e24707 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn2 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn2)) + (((locals.var_di_dn2 * locals.var_di) + (locals.var_di * locals.var_di_dn2)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17090_e24680 * locals.var_pds_dn2)) - ((locals.var_ai_dn2 * locals.var_di) + (locals.var_ai * locals.var_di_dn2))) - ((((((((((((2.0 * locals.var_ai_dn2) + (((((((((locals.var_c_fox_dn2 / locals.var_beta) * locals.var_db) + (assign17090_e24692 * locals.var_db_dn2)) * locals.var_db) + (assign17090_e24694 * locals.var_db_dn2)) * locals.var_c2) - (assign17090_e24696 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17090_e24701 * locals.var_db_dn2)) * locals.var_db) + (assign17090_e24703 * locals.var_db_dn2)) * locals.var_db) + (assign17090_e24705 * locals.var_db_dn2)) * locals.var_c2) - (assign17090_e24707 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn6 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn6)) + (((locals.var_di_dn6 * locals.var_di) + (locals.var_di * locals.var_di_dn6)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17090_e24680 * locals.var_pds_dn6)) - ((locals.var_ai_dn6 * locals.var_di) + (locals.var_ai * locals.var_di_dn6))) - ((((((((((((2.0 * locals.var_ai_dn6) + (((((((((locals.var_c_fox_dn6 / locals.var_beta) * locals.var_db) + (assign17090_e24692 * locals.var_db_dn6)) * locals.var_db) + (assign17090_e24694 * locals.var_db_dn6)) * locals.var_c2) - (assign17090_e24696 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17090_e24701 * locals.var_db_dn6)) * locals.var_db) + (assign17090_e24703 * locals.var_db_dn6)) * locals.var_db) + (assign17090_e24705 * locals.var_db_dn6)) * locals.var_c2) - (assign17090_e24707 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn7 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn7)) + (((locals.var_di_dn7 * locals.var_di) + (locals.var_di * locals.var_di_dn7)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17090_e24680 * locals.var_pds_dn7)) - ((locals.var_ai_dn7 * locals.var_di) + (locals.var_ai * locals.var_di_dn7))) - ((((((((((((2.0 * locals.var_ai_dn7) + (((((((((locals.var_c_fox_dn7 / locals.var_beta) * locals.var_db) + (assign17090_e24692 * locals.var_db_dn7)) * locals.var_db) + (assign17090_e24694 * locals.var_db_dn7)) * locals.var_c2) - (assign17090_e24696 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17090_e24701 * locals.var_db_dn7)) * locals.var_db) + (assign17090_e24703 * locals.var_db_dn7)) * locals.var_db) + (assign17090_e24705 * locals.var_db_dn7)) * locals.var_c2) - (assign17090_e24707 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((((((locals.var_ai_dn10 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn10)) + (((locals.var_di_dn10 * locals.var_di) + (locals.var_di * locals.var_di_dn10)) / 12.0)) * locals.var_beta) + (assign17090_e24678 * locals.var_beta_dn10)) * locals.var_pds) + (assign17090_e24680 * locals.var_pds_dn10)) - ((locals.var_ai_dn10 * locals.var_di) + (locals.var_ai * locals.var_di_dn10))) - ((((((((((((2.0 * locals.var_ai_dn10) + (((((((((((locals.var_c_fox_dn10 * locals.var_beta) - (locals.var_c_fox * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) * locals.var_db) + (assign17090_e24692 * locals.var_db_dn10)) * locals.var_db) + (assign17090_e24694 * locals.var_db_dn10)) * locals.var_c2) - (assign17090_e24696 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17090_e24701 * locals.var_db_dn10)) * locals.var_db) + (assign17090_e24703 * locals.var_db_dn10)) * locals.var_db) + (assign17090_e24705 * locals.var_db_dn10)) * locals.var_c2) - (assign17090_e24707 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn11 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn11)) + (((locals.var_di_dn11 * locals.var_di) + (locals.var_di * locals.var_di_dn11)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17090_e24680 * locals.var_pds_dn11)) - ((locals.var_ai_dn11 * locals.var_di) + (locals.var_ai * locals.var_di_dn11))) - ((((((((((((2.0 * locals.var_ai_dn11) + (((((((((locals.var_c_fox_dn11 / locals.var_beta) * locals.var_db) + (assign17090_e24692 * locals.var_db_dn11)) * locals.var_db) + (assign17090_e24694 * locals.var_db_dn11)) * locals.var_c2) - (assign17090_e24696 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17090_e24701 * locals.var_db_dn11)) * locals.var_db) + (assign17090_e24703 * locals.var_db_dn11)) * locals.var_db) + (assign17090_e24705 * locals.var_db_dn11)) * locals.var_c2) - (assign17090_e24707 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn12 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn12)) + (((locals.var_di_dn12 * locals.var_di) + (locals.var_di * locals.var_di_dn12)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17090_e24680 * locals.var_pds_dn12)) - ((locals.var_ai_dn12 * locals.var_di) + (locals.var_ai * locals.var_di_dn12))) - ((((((((((((2.0 * locals.var_ai_dn12) + (((((((((locals.var_c_fox_dn12 / locals.var_beta) * locals.var_db) + (assign17090_e24692 * locals.var_db_dn12)) * locals.var_db) + (assign17090_e24694 * locals.var_db_dn12)) * locals.var_c2) - (assign17090_e24696 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17090_e24701 * locals.var_db_dn12)) * locals.var_db) + (assign17090_e24703 * locals.var_db_dn12)) * locals.var_db) + (assign17090_e24705 * locals.var_db_dn12)) * locals.var_c2) - (assign17090_e24707 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn17 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn17)) + (((locals.var_di_dn17 * locals.var_di) + (locals.var_di * locals.var_di_dn17)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17090_e24680 * locals.var_pds_dn17)) - ((locals.var_ai_dn17 * locals.var_di) + (locals.var_ai * locals.var_di_dn17))) - ((((((((((((2.0 * locals.var_ai_dn17) + (((((((((locals.var_c_fox_dn17 / locals.var_beta) * locals.var_db) + (assign17090_e24692 * locals.var_db_dn17)) * locals.var_db) + (assign17090_e24694 * locals.var_db_dn17)) * locals.var_c2) - (assign17090_e24696 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17090_e24701 * locals.var_db_dn17)) * locals.var_db) + (assign17090_e24703 * locals.var_db_dn17)) * locals.var_db) + (assign17090_e24705 * locals.var_db_dn17)) * locals.var_c2) - (assign17090_e24707 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17090_e24714;
        locals.var_qiu_dn0 = assign17090_e24714_d_n0;
        locals.var_qiu_dn2 = assign17090_e24714_d_n2;
        locals.var_qiu_dn6 = assign17090_e24714_d_n6;
        locals.var_qiu_dn7 = assign17090_e24714_d_n7;
        locals.var_qiu_dn10 = assign17090_e24714_d_n10;
        locals.var_qiu_dn11 = assign17090_e24714_d_n11;
        locals.var_qiu_dn12 = assign17090_e24714_d_n12;
        locals.var_qiu_dn17 = assign17090_e24714_d_n17;
        locals.var_qiu_rv = 0.0;

        let (assign17100_e24727, assign17100_e24727_d_n0, assign17100_e24727_d_n2, assign17100_e24727_d_n6, assign17100_e24727_d_n7, assign17100_e24727_d_n10, assign17100_e24727_d_n11, assign17100_e24727_d_n12, assign17100_e24727_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard502 != 0.0)) && (locals.var_guard503 != 0.0)) {
        let assign17100_e24725: f64 = (locals.var_qiu / locals.var_idd);
        (assign17100_e24725, (((locals.var_qiu_dn0 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn0)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn2 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn2)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn6 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn6)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn7 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn7)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn10 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn10)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn11 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn11)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn12 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn12)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn17 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn17)) / (locals.var_idd * locals.var_idd)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17100_e24727;
        locals.var_qiu_dn0 = assign17100_e24727_d_n0;
        locals.var_qiu_dn2 = assign17100_e24727_d_n2;
        locals.var_qiu_dn6 = assign17100_e24727_d_n6;
        locals.var_qiu_dn7 = assign17100_e24727_d_n7;
        locals.var_qiu_dn10 = assign17100_e24727_d_n10;
        locals.var_qiu_dn11 = assign17100_e24727_d_n11;
        locals.var_qiu_dn12 = assign17100_e24727_d_n12;
        locals.var_qiu_dn17 = assign17100_e24727_d_n17;
        locals.var_qiu_rv = 0.0;

        let (assign17110_e24739, assign17110_e24739_d_n0, assign17110_e24739_d_n2, assign17110_e24739_d_n6, assign17110_e24739_d_n7, assign17110_e24739_d_n10, assign17110_e24739_d_n11, assign17110_e24739_d_n12, assign17110_e24739_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard502 != 0.0)) && (locals.var_guard503 == 0.0)) {
        (locals.var_ai, locals.var_ai_dn0, locals.var_ai_dn2, locals.var_ai_dn6, locals.var_ai_dn7, locals.var_ai_dn10, locals.var_ai_dn11, locals.var_ai_dn12, locals.var_ai_dn17,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17110_e24739;
        locals.var_qiu_dn0 = assign17110_e24739_d_n0;
        locals.var_qiu_dn2 = assign17110_e24739_d_n2;
        locals.var_qiu_dn6 = assign17110_e24739_d_n6;
        locals.var_qiu_dn7 = assign17110_e24739_d_n7;
        locals.var_qiu_dn10 = assign17110_e24739_d_n10;
        locals.var_qiu_dn11 = assign17110_e24739_d_n11;
        locals.var_qiu_dn12 = assign17110_e24739_d_n12;
        locals.var_qiu_dn17 = assign17110_e24739_d_n17;
        locals.var_qiu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17120_e24754, assign17120_e24754_d_n0, assign17120_e24754_d_n2, assign17120_e24754_d_n6, assign17120_e24754_d_n7, assign17120_e24754_d_n10, assign17120_e24754_d_n11, assign17120_e24754_d_n12, assign17120_e24754_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard463 != 0.0)) && (locals.var_guard502 == 0.0)) {
        let assign17120_e24748: f64 = (-0.5);
        let assign17120_e24751: f64 = (locals.var_q_n0 + locals.var_q_nl);
        let assign17120_e24752: f64 = (assign17120_e24748 * assign17120_e24751);
        (assign17120_e24752, (assign17120_e24748 * (locals.var_q_n0_dn0 + locals.var_q_nl_dn0)), (assign17120_e24748 * (locals.var_q_n0_dn2 + locals.var_q_nl_dn2)), (assign17120_e24748 * (locals.var_q_n0_dn6 + locals.var_q_nl_dn6)), (assign17120_e24748 * (locals.var_q_n0_dn7 + locals.var_q_nl_dn7)), (assign17120_e24748 * (locals.var_q_n0_dn10 + locals.var_q_nl_dn10)), (assign17120_e24748 * (locals.var_q_n0_dn11 + locals.var_q_nl_dn11)), (assign17120_e24748 * (locals.var_q_n0_dn12 + locals.var_q_nl_dn12)), (assign17120_e24748 * (locals.var_q_n0_dn17 + locals.var_q_nl_dn17)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17120_e24754;
        locals.var_qiu_dn0 = assign17120_e24754_d_n0;
        locals.var_qiu_dn2 = assign17120_e24754_d_n2;
        locals.var_qiu_dn6 = assign17120_e24754_d_n6;
        locals.var_qiu_dn7 = assign17120_e24754_d_n7;
        locals.var_qiu_dn10 = assign17120_e24754_d_n10;
        locals.var_qiu_dn11 = assign17120_e24754_d_n11;
        locals.var_qiu_dn12 = assign17120_e24754_d_n12;
        locals.var_qiu_dn17 = assign17120_e24754_d_n17;
        locals.var_qiu_rv = 0.0;

        let assign17160_e24768: f64 = if locals.var_end_of_part_1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard507 = assign17160_e24768;
        locals.var_guard507_rv = 0.0;

        let (assign17170_e24774, assign17170_e24774_d_n0, assign17170_e24774_d_n2, assign17170_e24774_d_n6, assign17170_e24774_d_n7, assign17170_e24774_d_n10, assign17170_e24774_d_n11, assign17170_e24774_d_n12, assign17170_e24774_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        let assign17170_e24772: f64 = (0.5 + locals.var_alpha);
        (assign17170_e24772, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    } else {
        (locals.var_qdnm, locals.var_qdnm_dn0, locals.var_qdnm_dn2, locals.var_qdnm_dn6, locals.var_qdnm_dn7, locals.var_qdnm_dn10, locals.var_qdnm_dn11, locals.var_qdnm_dn12, locals.var_qdnm_dn17,)
    }
};
        locals.var_qdnm = assign17170_e24774;
        locals.var_qdnm_dn0 = assign17170_e24774_d_n0;
        locals.var_qdnm_dn2 = assign17170_e24774_d_n2;
        locals.var_qdnm_dn6 = assign17170_e24774_d_n6;
        locals.var_qdnm_dn7 = assign17170_e24774_d_n7;
        locals.var_qdnm_dn10 = assign17170_e24774_d_n10;
        locals.var_qdnm_dn11 = assign17170_e24774_d_n11;
        locals.var_qdnm_dn12 = assign17170_e24774_d_n12;
        locals.var_qdnm_dn17 = assign17170_e24774_d_n17;
        locals.var_qdnm_rv = 0.0;

        let (assign17180_e24780, assign17180_e24780_d_n0, assign17180_e24780_d_n2, assign17180_e24780_d_n6, assign17180_e24780_d_n7, assign17180_e24780_d_n10, assign17180_e24780_d_n11, assign17180_e24780_d_n12, assign17180_e24780_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        let assign17180_e24778: f64 = (locals.var_qidn * locals.var_qinm);
        (assign17180_e24778, ((locals.var_qidn_dn0 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn0)), ((locals.var_qidn_dn2 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn2)), ((locals.var_qidn_dn6 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn6)), ((locals.var_qidn_dn7 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn7)), ((locals.var_qidn_dn10 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn10)), ((locals.var_qidn_dn11 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn11)), ((locals.var_qidn_dn12 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn12)), ((locals.var_qidn_dn17 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn17)),)
    } else {
        (locals.var_qddn, locals.var_qddn_dn0, locals.var_qddn_dn2, locals.var_qddn_dn6, locals.var_qddn_dn7, locals.var_qddn_dn10, locals.var_qddn_dn11, locals.var_qddn_dn12, locals.var_qddn_dn17,)
    }
};
        locals.var_qddn = assign17180_e24780;
        locals.var_qddn_dn0 = assign17180_e24780_d_n0;
        locals.var_qddn_dn2 = assign17180_e24780_d_n2;
        locals.var_qddn_dn6 = assign17180_e24780_d_n6;
        locals.var_qddn_dn7 = assign17180_e24780_d_n7;
        locals.var_qddn_dn10 = assign17180_e24780_d_n10;
        locals.var_qddn_dn11 = assign17180_e24780_d_n11;
        locals.var_qddn_dn12 = assign17180_e24780_d_n12;
        locals.var_qddn_dn17 = assign17180_e24780_d_n17;
        locals.var_qddn_rv = 0.0;

        let (assign17190_e24788, assign17190_e24788_d_n0, assign17190_e24788_d_n2, assign17190_e24788_d_n6, assign17190_e24788_d_n7, assign17190_e24788_d_n10, assign17190_e24788_d_n11, assign17190_e24788_d_n12, assign17190_e24788_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        let assign17190_e24784: f64 = (0.4 * locals.var_qdnm);
        let assign17190_e24786: f64 = (assign17190_e24784 / locals.var_qddn);
        (assign17190_e24786, ((((0.4 * locals.var_qdnm_dn0) * locals.var_qddn) - (assign17190_e24784 * locals.var_qddn_dn0)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn2) * locals.var_qddn) - (assign17190_e24784 * locals.var_qddn_dn2)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn6) * locals.var_qddn) - (assign17190_e24784 * locals.var_qddn_dn6)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn7) * locals.var_qddn) - (assign17190_e24784 * locals.var_qddn_dn7)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn10) * locals.var_qddn) - (assign17190_e24784 * locals.var_qddn_dn10)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn11) * locals.var_qddn) - (assign17190_e24784 * locals.var_qddn_dn11)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn12) * locals.var_qddn) - (assign17190_e24784 * locals.var_qddn_dn12)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn17) * locals.var_qddn) - (assign17190_e24784 * locals.var_qddn_dn17)) / (locals.var_qddn * locals.var_qddn)),)
    } else {
        (locals.var_quot, locals.var_quot_dn0, locals.var_quot_dn2, locals.var_quot_dn6, locals.var_quot_dn7, locals.var_quot_dn10, locals.var_quot_dn11, locals.var_quot_dn12, locals.var_quot_dn17,)
    }
};
        locals.var_quot = assign17190_e24788;
        locals.var_quot_dn0 = assign17190_e24788_d_n0;
        locals.var_quot_dn2 = assign17190_e24788_d_n2;
        locals.var_quot_dn6 = assign17190_e24788_d_n6;
        locals.var_quot_dn7 = assign17190_e24788_d_n7;
        locals.var_quot_dn10 = assign17190_e24788_d_n10;
        locals.var_quot_dn11 = assign17190_e24788_d_n11;
        locals.var_quot_dn12 = assign17190_e24788_d_n12;
        locals.var_quot_dn17 = assign17190_e24788_d_n17;
        locals.var_quot_rv = 0.0;

        let (assign17200_e24794, assign17200_e24794_d_n0, assign17200_e24794_d_n2, assign17200_e24794_d_n6, assign17200_e24794_d_n7, assign17200_e24794_d_n10, assign17200_e24794_d_n11, assign17200_e24794_d_n12, assign17200_e24794_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        let assign17200_e24792: f64 = (0.6 - locals.var_quot);
        (assign17200_e24792, (-locals.var_quot_dn0), (-locals.var_quot_dn2), (-locals.var_quot_dn6), (-locals.var_quot_dn7), (-locals.var_quot_dn10), (-locals.var_quot_dn11), (-locals.var_quot_dn12), (-locals.var_quot_dn17),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17200_e24794;
        locals.var_qdrat_dn0 = assign17200_e24794_d_n0;
        locals.var_qdrat_dn2 = assign17200_e24794_d_n2;
        locals.var_qdrat_dn6 = assign17200_e24794_d_n6;
        locals.var_qdrat_dn7 = assign17200_e24794_d_n7;
        locals.var_qdrat_dn10 = assign17200_e24794_d_n10;
        locals.var_qdrat_dn11 = assign17200_e24794_d_n11;
        locals.var_qdrat_dn12 = assign17200_e24794_d_n12;
        locals.var_qdrat_dn17 = assign17200_e24794_d_n17;
        locals.var_qdrat_rv = 0.0;

        let assign17210_e24798: f64 = (0.5 + 1e-8);
        let assign17210_e24799: f64 = if locals.var_qdrat > assign17210_e24798 { 1.0 } else { 0.0 };
        locals.var_guard508 = assign17210_e24799;
        locals.var_guard508_rv = 0.0;

        let (assign17230_e24808, assign17230_e24808_d_n0, assign17230_e24808_d_n2, assign17230_e24808_d_n6, assign17230_e24808_d_n7, assign17230_e24808_d_n10, assign17230_e24808_d_n11, assign17230_e24808_d_n12, assign17230_e24808_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard508 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17230_e24808;
        locals.var_qdrat_dn0 = assign17230_e24808_d_n0;
        locals.var_qdrat_dn2 = assign17230_e24808_d_n2;
        locals.var_qdrat_dn6 = assign17230_e24808_d_n6;
        locals.var_qdrat_dn7 = assign17230_e24808_d_n7;
        locals.var_qdrat_dn10 = assign17230_e24808_d_n10;
        locals.var_qdrat_dn11 = assign17230_e24808_d_n11;
        locals.var_qdrat_dn12 = assign17230_e24808_d_n12;
        locals.var_qdrat_dn17 = assign17230_e24808_d_n17;
        locals.var_qdrat_rv = 0.0;

        let (assign17240_e24812, assign17240_e24812_d_n0, assign17240_e24812_d_n2, assign17240_e24812_d_n6, assign17240_e24812_d_n7, assign17240_e24812_d_n10, assign17240_e24812_d_n11, assign17240_e24812_d_n12, assign17240_e24812_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    } else {
        (locals.var_qdrat_noi, locals.var_qdrat_noi_dn0, locals.var_qdrat_noi_dn2, locals.var_qdrat_noi_dn6, locals.var_qdrat_noi_dn7, locals.var_qdrat_noi_dn10, locals.var_qdrat_noi_dn11, locals.var_qdrat_noi_dn12, locals.var_qdrat_noi_dn17,)
    }
};
        locals.var_qdrat_noi = assign17240_e24812;
        locals.var_qdrat_noi_dn0 = assign17240_e24812_d_n0;
        locals.var_qdrat_noi_dn2 = assign17240_e24812_d_n2;
        locals.var_qdrat_noi_dn6 = assign17240_e24812_d_n6;
        locals.var_qdrat_noi_dn7 = assign17240_e24812_d_n7;
        locals.var_qdrat_noi_dn10 = assign17240_e24812_d_n10;
        locals.var_qdrat_noi_dn11 = assign17240_e24812_d_n11;
        locals.var_qdrat_noi_dn12 = assign17240_e24812_d_n12;
        locals.var_qdrat_noi_dn17 = assign17240_e24812_d_n17;
        locals.var_qdrat_noi_rv = 0.0;

        let (assign17250_e24816, assign17250_e24816_d_n0, assign17250_e24816_d_n2, assign17250_e24816_d_n6, assign17250_e24816_d_n7, assign17250_e24816_d_n10, assign17250_e24816_d_n11, assign17250_e24816_d_n12, assign17250_e24816_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17250_e24816;
        locals.var_qdrat_dn0 = assign17250_e24816_d_n0;
        locals.var_qdrat_dn2 = assign17250_e24816_d_n2;
        locals.var_qdrat_dn6 = assign17250_e24816_d_n6;
        locals.var_qdrat_dn7 = assign17250_e24816_d_n7;
        locals.var_qdrat_dn10 = assign17250_e24816_d_n10;
        locals.var_qdrat_dn11 = assign17250_e24816_d_n11;
        locals.var_qdrat_dn12 = assign17250_e24816_d_n12;
        locals.var_qdrat_dn17 = assign17250_e24816_d_n17;
        locals.var_qdrat_rv = 0.0;

        let assign17260_e24819: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard510 = assign17260_e24819;
        locals.var_guard510_rv = 0.0;

        let assign17270_e24823: f64 = (10.0 * 2.220446049250313e-16);
        let assign17270_e24828: f64 = (10.0 * 2.220446049250313e-16);
        let assign17270_e24830: f64 = if ((p.p190 < assign17270_e24823) && (p.p191 < assign17270_e24828)) { 1.0 } else { 0.0 };
        locals.var_guard526 = assign17270_e24830;
        locals.var_guard526_rv = 0.0;

        let (assign17280_e24838, assign17280_e24838_d_n0, assign17280_e24838_d_n2, assign17280_e24838_d_n6, assign17280_e24838_d_n7, assign17280_e24838_d_n10, assign17280_e24838_d_n11, assign17280_e24838_d_n12, assign17280_e24838_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17280_e24838;
        locals.var_lred_dn0 = assign17280_e24838_d_n0;
        locals.var_lred_dn2 = assign17280_e24838_d_n2;
        locals.var_lred_dn6 = assign17280_e24838_d_n6;
        locals.var_lred_dn7 = assign17280_e24838_d_n7;
        locals.var_lred_dn10 = assign17280_e24838_d_n10;
        locals.var_lred_dn11 = assign17280_e24838_d_n11;
        locals.var_lred_dn12 = assign17280_e24838_d_n12;
        locals.var_lred_dn17 = assign17280_e24838_d_n17;
        locals.var_lred_rv = 0.0;

        let (assign17290_e24846, assign17290_e24846_d_n0, assign17290_e24846_d_n2, assign17290_e24846_d_n6, assign17290_e24846_d_n7, assign17290_e24846_d_n10, assign17290_e24846_d_n11, assign17290_e24846_d_n12, assign17290_e24846_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17290_e24846;
        locals.var_psdl_dn0 = assign17290_e24846_d_n0;
        locals.var_psdl_dn2 = assign17290_e24846_d_n2;
        locals.var_psdl_dn6 = assign17290_e24846_d_n6;
        locals.var_psdl_dn7 = assign17290_e24846_d_n7;
        locals.var_psdl_dn10 = assign17290_e24846_d_n10;
        locals.var_psdl_dn11 = assign17290_e24846_d_n11;
        locals.var_psdl_dn12 = assign17290_e24846_d_n12;
        locals.var_psdl_dn17 = assign17290_e24846_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign17300_e24850: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17300_e24853: f64 = (10.0 * 2.220446049250313e-16);
        let assign17300_e24854: f64 = (assign17300_e24850 - assign17300_e24853);
        let assign17300_e24855: f64 = if locals.var_psdl > assign17300_e24854 { 1.0 } else { 0.0 };
        locals.var_guard527 = assign17300_e24855;
        locals.var_guard527_rv = 0.0;

        let (assign17310_e24871, assign17310_e24871_d_n0, assign17310_e24871_d_n2, assign17310_e24871_d_n6, assign17310_e24871_d_n7, assign17310_e24871_d_n10, assign17310_e24871_d_n11, assign17310_e24871_d_n12, assign17310_e24871_d_n17,) = {
    if ((((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 != 0.0)) && (locals.var_guard527 != 0.0)) {
        let assign17310_e24865: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17310_e24868: f64 = (10.0 * 2.220446049250313e-16);
        let assign17310_e24869: f64 = (assign17310_e24865 - assign17310_e24868);
        (assign17310_e24869, (locals.var_ps0_dn0 + locals.var_vdsz_dn0), (locals.var_ps0_dn2 + locals.var_vdsz_dn2), (locals.var_ps0_dn6 + locals.var_vdsz_dn6), (locals.var_ps0_dn7 + locals.var_vdsz_dn7), (locals.var_ps0_dn10 + locals.var_vdsz_dn10), (locals.var_ps0_dn11 + locals.var_vdsz_dn11), (locals.var_ps0_dn12 + locals.var_vdsz_dn12), (locals.var_ps0_dn17 + locals.var_vdsz_dn17),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17310_e24871;
        locals.var_psdl_dn0 = assign17310_e24871_d_n0;
        locals.var_psdl_dn2 = assign17310_e24871_d_n2;
        locals.var_psdl_dn6 = assign17310_e24871_d_n6;
        locals.var_psdl_dn7 = assign17310_e24871_d_n7;
        locals.var_psdl_dn10 = assign17310_e24871_d_n10;
        locals.var_psdl_dn11 = assign17310_e24871_d_n11;
        locals.var_psdl_dn12 = assign17310_e24871_d_n12;
        locals.var_psdl_dn17 = assign17310_e24871_d_n17;
        locals.var_psdl_rv = 0.0;

        let (assign17320_e24885,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let (assign17320_e24883,) = {
            if (p.p43 == 1.0) {
                (p.p237,)
            } else {
                (locals.var_wdsoi_0,)
            }
        };
        (assign17320_e24883,)
    } else {
        (locals.var_wd,)
    }
};
        locals.var_wd = assign17320_e24885;
        locals.var_wd_rv = 0.0;

        let (assign17330_e24896,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17330_e24894: f64 = (1.0 / locals.var_wd);
        (assign17330_e24894,)
    } else {
        (locals.var_t0__blk511,)
    }
};
        locals.var_t0__blk511 = assign17330_e24896;
        locals.var_t0__blk511_rv = 0.0;

        let (assign17340_e24907, assign17340_e24907_d_n0, assign17340_e24907_d_n2, assign17340_e24907_d_n6, assign17340_e24907_d_n7, assign17340_e24907_d_n10, assign17340_e24907_d_n11, assign17340_e24907_d_n12, assign17340_e24907_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17340_e24905: f64 = (locals.var_qn0 * locals.var_t0__blk511);
        (assign17340_e24905, (locals.var_qn0_dn0 * locals.var_t0__blk511), (locals.var_qn0_dn2 * locals.var_t0__blk511), (locals.var_qn0_dn6 * locals.var_t0__blk511), (locals.var_qn0_dn7 * locals.var_t0__blk511), (locals.var_qn0_dn10 * locals.var_t0__blk511), (locals.var_qn0_dn11 * locals.var_t0__blk511), (locals.var_qn0_dn12 * locals.var_t0__blk511), (locals.var_qn0_dn17 * locals.var_t0__blk511),)
    } else {
        (locals.var_t1__blk512, locals.var_t1__blk512_dn0, locals.var_t1__blk512_dn2, locals.var_t1__blk512_dn6, locals.var_t1__blk512_dn7, locals.var_t1__blk512_dn10, locals.var_t1__blk512_dn11, locals.var_t1__blk512_dn12, locals.var_t1__blk512_dn17,)
    }
};
        locals.var_t1__blk512 = assign17340_e24907;
        locals.var_t1__blk512_dn0 = assign17340_e24907_d_n0;
        locals.var_t1__blk512_dn2 = assign17340_e24907_d_n2;
        locals.var_t1__blk512_dn6 = assign17340_e24907_d_n6;
        locals.var_t1__blk512_dn7 = assign17340_e24907_d_n7;
        locals.var_t1__blk512_dn10 = assign17340_e24907_d_n10;
        locals.var_t1__blk512_dn11 = assign17340_e24907_d_n11;
        locals.var_t1__blk512_dn12 = assign17340_e24907_d_n12;
        locals.var_t1__blk512_dn17 = assign17340_e24907_d_n17;
        locals.var_t1__blk512_rv = 0.0;

        let (assign17350_e24918, assign17350_e24918_d_n0, assign17350_e24918_d_n2, assign17350_e24918_d_n6, assign17350_e24918_d_n7, assign17350_e24918_d_n10, assign17350_e24918_d_n11, assign17350_e24918_d_n12, assign17350_e24918_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17350_e24916: f64 = (p.p191 * locals.var_t1__blk512);
        (assign17350_e24916, (p.p191 * locals.var_t1__blk512_dn0), (p.p191 * locals.var_t1__blk512_dn2), (p.p191 * locals.var_t1__blk512_dn6), (p.p191 * locals.var_t1__blk512_dn7), (p.p191 * locals.var_t1__blk512_dn10), (p.p191 * locals.var_t1__blk512_dn11), (p.p191 * locals.var_t1__blk512_dn12), (p.p191 * locals.var_t1__blk512_dn17),)
    } else {
        (locals.var_t2__blk513, locals.var_t2__blk513_dn0, locals.var_t2__blk513_dn2, locals.var_t2__blk513_dn6, locals.var_t2__blk513_dn7, locals.var_t2__blk513_dn10, locals.var_t2__blk513_dn11, locals.var_t2__blk513_dn12, locals.var_t2__blk513_dn17,)
    }
};
        locals.var_t2__blk513 = assign17350_e24918;
        locals.var_t2__blk513_dn0 = assign17350_e24918_d_n0;
        locals.var_t2__blk513_dn2 = assign17350_e24918_d_n2;
        locals.var_t2__blk513_dn6 = assign17350_e24918_d_n6;
        locals.var_t2__blk513_dn7 = assign17350_e24918_d_n7;
        locals.var_t2__blk513_dn10 = assign17350_e24918_d_n10;
        locals.var_t2__blk513_dn11 = assign17350_e24918_d_n11;
        locals.var_t2__blk513_dn12 = assign17350_e24918_d_n12;
        locals.var_t2__blk513_dn17 = assign17350_e24918_d_n17;
        locals.var_t2__blk513_rv = 0.0;

        let (assign17360_e24931, assign17360_e24931_d_n0, assign17360_e24931_d_n2, assign17360_e24931_d_n6, assign17360_e24931_d_n7, assign17360_e24931_d_n10, assign17360_e24931_d_n11, assign17360_e24931_d_n12, assign17360_e24931_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17360_e24927: f64 = (locals.var_uc_clm2 * locals.var_q_nsub);
        let assign17360_e24929: f64 = (assign17360_e24927 + locals.var_t2__blk513);
        (assign17360_e24929, (((locals.var_uc_clm2_dn0 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn0)) + locals.var_t2__blk513_dn0), (((locals.var_uc_clm2_dn2 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn2)) + locals.var_t2__blk513_dn2), (((locals.var_uc_clm2_dn6 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn6)) + locals.var_t2__blk513_dn6), (((locals.var_uc_clm2_dn7 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn7)) + locals.var_t2__blk513_dn7), (((locals.var_uc_clm2_dn10 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn10)) + locals.var_t2__blk513_dn10), (((locals.var_uc_clm2_dn11 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn11)) + locals.var_t2__blk513_dn11), (((locals.var_uc_clm2_dn12 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn12)) + locals.var_t2__blk513_dn12), (((locals.var_uc_clm2_dn17 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn17)) + locals.var_t2__blk513_dn17),)
    } else {
        (locals.var_t5__blk516, locals.var_t5__blk516_dn0, locals.var_t5__blk516_dn2, locals.var_t5__blk516_dn6, locals.var_t5__blk516_dn7, locals.var_t5__blk516_dn10, locals.var_t5__blk516_dn11, locals.var_t5__blk516_dn12, locals.var_t5__blk516_dn17,)
    }
};
        locals.var_t5__blk516 = assign17360_e24931;
        locals.var_t5__blk516_dn0 = assign17360_e24931_d_n0;
        locals.var_t5__blk516_dn2 = assign17360_e24931_d_n2;
        locals.var_t5__blk516_dn6 = assign17360_e24931_d_n6;
        locals.var_t5__blk516_dn7 = assign17360_e24931_d_n7;
        locals.var_t5__blk516_dn10 = assign17360_e24931_d_n10;
        locals.var_t5__blk516_dn11 = assign17360_e24931_d_n11;
        locals.var_t5__blk516_dn12 = assign17360_e24931_d_n12;
        locals.var_t5__blk516_dn17 = assign17360_e24931_d_n17;
        locals.var_t5__blk516_rv = 0.0;

        let (assign17370_e24942, assign17370_e24942_d_n0, assign17370_e24942_d_n2, assign17370_e24942_d_n6, assign17370_e24942_d_n7, assign17370_e24942_d_n10, assign17370_e24942_d_n11, assign17370_e24942_d_n12, assign17370_e24942_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17370_e24940: f64 = (1.0 / locals.var_t5__blk516);
        (assign17370_e24940, (-(locals.var_t5__blk516_dn0 / (locals.var_t5__blk516 * locals.var_t5__blk516))), (-(locals.var_t5__blk516_dn2 / (locals.var_t5__blk516 * locals.var_t5__blk516))), (-(locals.var_t5__blk516_dn6 / (locals.var_t5__blk516 * locals.var_t5__blk516))), (-(locals.var_t5__blk516_dn7 / (locals.var_t5__blk516 * locals.var_t5__blk516))), (-(locals.var_t5__blk516_dn10 / (locals.var_t5__blk516 * locals.var_t5__blk516))), (-(locals.var_t5__blk516_dn11 / (locals.var_t5__blk516 * locals.var_t5__blk516))), (-(locals.var_t5__blk516_dn12 / (locals.var_t5__blk516 * locals.var_t5__blk516))), (-(locals.var_t5__blk516_dn17 / (locals.var_t5__blk516 * locals.var_t5__blk516))),)
    } else {
        (locals.var_t1__blk512, locals.var_t1__blk512_dn0, locals.var_t1__blk512_dn2, locals.var_t1__blk512_dn6, locals.var_t1__blk512_dn7, locals.var_t1__blk512_dn10, locals.var_t1__blk512_dn11, locals.var_t1__blk512_dn12, locals.var_t1__blk512_dn17,)
    }
};
        locals.var_t1__blk512 = assign17370_e24942;
        locals.var_t1__blk512_dn0 = assign17370_e24942_d_n0;
        locals.var_t1__blk512_dn2 = assign17370_e24942_d_n2;
        locals.var_t1__blk512_dn6 = assign17370_e24942_d_n6;
        locals.var_t1__blk512_dn7 = assign17370_e24942_d_n7;
        locals.var_t1__blk512_dn10 = assign17370_e24942_d_n10;
        locals.var_t1__blk512_dn11 = assign17370_e24942_d_n11;
        locals.var_t1__blk512_dn12 = assign17370_e24942_d_n12;
        locals.var_t1__blk512_dn17 = assign17370_e24942_d_n17;
        locals.var_t1__blk512_rv = 0.0;

        let (assign17380_e24953, assign17380_e24953_d_n0, assign17380_e24953_d_n2, assign17380_e24953_d_n6, assign17380_e24953_d_n7, assign17380_e24953_d_n10, assign17380_e24953_d_n11, assign17380_e24953_d_n12, assign17380_e24953_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17380_e24951: f64 = (1.034943e-10 * locals.var_t1__blk512);
        (assign17380_e24951, (1.034943e-10 * locals.var_t1__blk512_dn0), (1.034943e-10 * locals.var_t1__blk512_dn2), (1.034943e-10 * locals.var_t1__blk512_dn6), (1.034943e-10 * locals.var_t1__blk512_dn7), (1.034943e-10 * locals.var_t1__blk512_dn10), (1.034943e-10 * locals.var_t1__blk512_dn11), (1.034943e-10 * locals.var_t1__blk512_dn12), (1.034943e-10 * locals.var_t1__blk512_dn17),)
    } else {
        (locals.var_t4__blk515, locals.var_t4__blk515_dn0, locals.var_t4__blk515_dn2, locals.var_t4__blk515_dn6, locals.var_t4__blk515_dn7, locals.var_t4__blk515_dn10, locals.var_t4__blk515_dn11, locals.var_t4__blk515_dn12, locals.var_t4__blk515_dn17,)
    }
};
        locals.var_t4__blk515 = assign17380_e24953;
        locals.var_t4__blk515_dn0 = assign17380_e24953_d_n0;
        locals.var_t4__blk515_dn2 = assign17380_e24953_d_n2;
        locals.var_t4__blk515_dn6 = assign17380_e24953_d_n6;
        locals.var_t4__blk515_dn7 = assign17380_e24953_d_n7;
        locals.var_t4__blk515_dn10 = assign17380_e24953_d_n10;
        locals.var_t4__blk515_dn11 = assign17380_e24953_d_n11;
        locals.var_t4__blk515_dn12 = assign17380_e24953_d_n12;
        locals.var_t4__blk515_dn17 = assign17380_e24953_d_n17;
        locals.var_t4__blk515_rv = 0.0;

        let (assign17390_e24964, assign17390_e24964_d_n0, assign17390_e24964_d_n2, assign17390_e24964_d_n6, assign17390_e24964_d_n7, assign17390_e24964_d_n10, assign17390_e24964_d_n11, assign17390_e24964_d_n12, assign17390_e24964_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17390_e24962: f64 = (1.0 - p.p189);
        (assign17390_e24962, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk512, locals.var_t1__blk512_dn0, locals.var_t1__blk512_dn2, locals.var_t1__blk512_dn6, locals.var_t1__blk512_dn7, locals.var_t1__blk512_dn10, locals.var_t1__blk512_dn11, locals.var_t1__blk512_dn12, locals.var_t1__blk512_dn17,)
    }
};
        locals.var_t1__blk512 = assign17390_e24964;
        locals.var_t1__blk512_dn0 = assign17390_e24964_d_n0;
        locals.var_t1__blk512_dn2 = assign17390_e24964_d_n2;
        locals.var_t1__blk512_dn6 = assign17390_e24964_d_n6;
        locals.var_t1__blk512_dn7 = assign17390_e24964_d_n7;
        locals.var_t1__blk512_dn10 = assign17390_e24964_d_n10;
        locals.var_t1__blk512_dn11 = assign17390_e24964_d_n11;
        locals.var_t1__blk512_dn12 = assign17390_e24964_d_n12;
        locals.var_t1__blk512_dn17 = assign17390_e24964_d_n17;
        locals.var_t1__blk512_rv = 0.0;

        let (assign17400_e24981, assign17400_e24981_d_n0, assign17400_e24981_d_n2, assign17400_e24981_d_n6, assign17400_e24981_d_n7, assign17400_e24981_d_n10, assign17400_e24981_d_n11, assign17400_e24981_d_n12, assign17400_e24981_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17400_e24974: f64 = (locals.var_vds + locals.var_ps0);
        let assign17400_e24975: f64 = (p.p189 * assign17400_e24974);
        let assign17400_e24978: f64 = (locals.var_t1__blk512 * locals.var_psl);
        let assign17400_e24979: f64 = (assign17400_e24975 + assign17400_e24978);
        (assign17400_e24979, ((p.p189 * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + ((locals.var_t1__blk512_dn0 * locals.var_psl) + (locals.var_t1__blk512 * locals.var_psl_dn0))), ((p.p189 * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + ((locals.var_t1__blk512_dn2 * locals.var_psl) + (locals.var_t1__blk512 * locals.var_psl_dn2))), ((p.p189 * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + ((locals.var_t1__blk512_dn6 * locals.var_psl) + (locals.var_t1__blk512 * locals.var_psl_dn6))), ((p.p189 * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + ((locals.var_t1__blk512_dn7 * locals.var_psl) + (locals.var_t1__blk512 * locals.var_psl_dn7))), ((p.p189 * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + ((locals.var_t1__blk512_dn10 * locals.var_psl) + (locals.var_t1__blk512 * locals.var_psl_dn10))), ((p.p189 * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + ((locals.var_t1__blk512_dn11 * locals.var_psl) + (locals.var_t1__blk512 * locals.var_psl_dn11))), ((p.p189 * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + ((locals.var_t1__blk512_dn12 * locals.var_psl) + (locals.var_t1__blk512 * locals.var_psl_dn12))), ((p.p189 * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + ((locals.var_t1__blk512_dn17 * locals.var_psl) + (locals.var_t1__blk512 * locals.var_psl_dn17))),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17400_e24981;
        locals.var_psdl_dn0 = assign17400_e24981_d_n0;
        locals.var_psdl_dn2 = assign17400_e24981_d_n2;
        locals.var_psdl_dn6 = assign17400_e24981_d_n6;
        locals.var_psdl_dn7 = assign17400_e24981_d_n7;
        locals.var_psdl_dn10 = assign17400_e24981_d_n10;
        locals.var_psdl_dn11 = assign17400_e24981_d_n11;
        locals.var_psdl_dn12 = assign17400_e24981_d_n12;
        locals.var_psdl_dn17 = assign17400_e24981_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign17410_e24985: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17410_e24988: f64 = (10.0 * 2.220446049250313e-16);
        let assign17410_e24989: f64 = (assign17410_e24985 - assign17410_e24988);
        let assign17410_e24990: f64 = if locals.var_psdl > assign17410_e24989 { 1.0 } else { 0.0 };
        locals.var_guard528 = assign17410_e24990;
        locals.var_guard528_rv = 0.0;

        let (assign17420_e25007, assign17420_e25007_d_n0, assign17420_e25007_d_n2, assign17420_e25007_d_n6, assign17420_e25007_d_n7, assign17420_e25007_d_n10, assign17420_e25007_d_n11, assign17420_e25007_d_n12, assign17420_e25007_d_n17,) = {
    if ((((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) && (locals.var_guard528 != 0.0)) {
        let assign17420_e25001: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17420_e25004: f64 = (10.0 * 2.220446049250313e-16);
        let assign17420_e25005: f64 = (assign17420_e25001 - assign17420_e25004);
        (assign17420_e25005, (locals.var_ps0_dn0 + locals.var_vdsz_dn0), (locals.var_ps0_dn2 + locals.var_vdsz_dn2), (locals.var_ps0_dn6 + locals.var_vdsz_dn6), (locals.var_ps0_dn7 + locals.var_vdsz_dn7), (locals.var_ps0_dn10 + locals.var_vdsz_dn10), (locals.var_ps0_dn11 + locals.var_vdsz_dn11), (locals.var_ps0_dn12 + locals.var_vdsz_dn12), (locals.var_ps0_dn17 + locals.var_vdsz_dn17),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17420_e25007;
        locals.var_psdl_dn0 = assign17420_e25007_d_n0;
        locals.var_psdl_dn2 = assign17420_e25007_d_n2;
        locals.var_psdl_dn6 = assign17420_e25007_d_n6;
        locals.var_psdl_dn7 = assign17420_e25007_d_n7;
        locals.var_psdl_dn10 = assign17420_e25007_d_n10;
        locals.var_psdl_dn11 = assign17420_e25007_d_n11;
        locals.var_psdl_dn12 = assign17420_e25007_d_n12;
        locals.var_psdl_dn17 = assign17420_e25007_d_n17;
        locals.var_psdl_rv = 0.0;

        let (assign17430_e25018, assign17430_e25018_d_n0, assign17430_e25018_d_n2, assign17430_e25018_d_n6, assign17430_e25018_d_n7, assign17430_e25018_d_n10, assign17430_e25018_d_n11, assign17430_e25018_d_n12, assign17430_e25018_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17430_e25016: f64 = (locals.var_psdl - locals.var_psl);
        (assign17430_e25016, (locals.var_psdl_dn0 - locals.var_psl_dn0), (locals.var_psdl_dn2 - locals.var_psl_dn2), (locals.var_psdl_dn6 - locals.var_psl_dn6), (locals.var_psdl_dn7 - locals.var_psl_dn7), (locals.var_psdl_dn10 - locals.var_psl_dn10), (locals.var_psdl_dn11 - locals.var_psl_dn11), (locals.var_psdl_dn12 - locals.var_psl_dn12), (locals.var_psdl_dn17 - locals.var_psl_dn17),)
    } else {
        (locals.var_t6w__blk518, locals.var_t6w__blk518_dn0, locals.var_t6w__blk518_dn2, locals.var_t6w__blk518_dn6, locals.var_t6w__blk518_dn7, locals.var_t6w__blk518_dn10, locals.var_t6w__blk518_dn11, locals.var_t6w__blk518_dn12, locals.var_t6w__blk518_dn17,)
    }
};
        locals.var_t6w__blk518 = assign17430_e25018;
        locals.var_t6w__blk518_dn0 = assign17430_e25018_d_n0;
        locals.var_t6w__blk518_dn2 = assign17430_e25018_d_n2;
        locals.var_t6w__blk518_dn6 = assign17430_e25018_d_n6;
        locals.var_t6w__blk518_dn7 = assign17430_e25018_d_n7;
        locals.var_t6w__blk518_dn10 = assign17430_e25018_d_n10;
        locals.var_t6w__blk518_dn11 = assign17430_e25018_d_n11;
        locals.var_t6w__blk518_dn12 = assign17430_e25018_d_n12;
        locals.var_t6w__blk518_dn17 = assign17430_e25018_d_n17;
        locals.var_t6w__blk518_rv = 0.0;

        let (assign17440_e25036, assign17440_e25036_d_n0, assign17440_e25036_d_n2, assign17440_e25036_d_n6, assign17440_e25036_d_n7, assign17440_e25036_d_n10, assign17440_e25036_d_n11, assign17440_e25036_d_n12, assign17440_e25036_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17440_e25027: f64 = (locals.var_t6w__blk518 * locals.var_t6w__blk518);
        let assign17440_e25030: f64 = (4.0 * 0.001);
        let assign17440_e25032: f64 = (assign17440_e25030 * 0.001);
        let assign17440_e25033: f64 = (assign17440_e25027 + assign17440_e25032);
        let assign17440_e25034: f64 = (assign17440_e25033).sqrt();
        (assign17440_e25034, (((locals.var_t6w__blk518_dn0 * locals.var_t6w__blk518) + (locals.var_t6w__blk518 * locals.var_t6w__blk518_dn0)) / (2.0 * assign17440_e25034)), (((locals.var_t6w__blk518_dn2 * locals.var_t6w__blk518) + (locals.var_t6w__blk518 * locals.var_t6w__blk518_dn2)) / (2.0 * assign17440_e25034)), (((locals.var_t6w__blk518_dn6 * locals.var_t6w__blk518) + (locals.var_t6w__blk518 * locals.var_t6w__blk518_dn6)) / (2.0 * assign17440_e25034)), (((locals.var_t6w__blk518_dn7 * locals.var_t6w__blk518) + (locals.var_t6w__blk518 * locals.var_t6w__blk518_dn7)) / (2.0 * assign17440_e25034)), (((locals.var_t6w__blk518_dn10 * locals.var_t6w__blk518) + (locals.var_t6w__blk518 * locals.var_t6w__blk518_dn10)) / (2.0 * assign17440_e25034)), (((locals.var_t6w__blk518_dn11 * locals.var_t6w__blk518) + (locals.var_t6w__blk518 * locals.var_t6w__blk518_dn11)) / (2.0 * assign17440_e25034)), (((locals.var_t6w__blk518_dn12 * locals.var_t6w__blk518) + (locals.var_t6w__blk518 * locals.var_t6w__blk518_dn12)) / (2.0 * assign17440_e25034)), (((locals.var_t6w__blk518_dn17 * locals.var_t6w__blk518) + (locals.var_t6w__blk518 * locals.var_t6w__blk518_dn17)) / (2.0 * assign17440_e25034)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign17440_e25036;
        locals.var_tmf1_dn0 = assign17440_e25036_d_n0;
        locals.var_tmf1_dn2 = assign17440_e25036_d_n2;
        locals.var_tmf1_dn6 = assign17440_e25036_d_n6;
        locals.var_tmf1_dn7 = assign17440_e25036_d_n7;
        locals.var_tmf1_dn10 = assign17440_e25036_d_n10;
        locals.var_tmf1_dn11 = assign17440_e25036_d_n11;
        locals.var_tmf1_dn12 = assign17440_e25036_d_n12;
        locals.var_tmf1_dn17 = assign17440_e25036_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign17450_e25053, assign17450_e25053_d_n0, assign17450_e25053_d_n2, assign17450_e25053_d_n6, assign17450_e25053_d_n7, assign17450_e25053_d_n10, assign17450_e25053_d_n11, assign17450_e25053_d_n12, assign17450_e25053_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17450_e25046: f64 = (locals.var_t6w__blk518 + locals.var_tmf1);
        let assign17450_e25047: f64 = (0.5 * assign17450_e25046);
        let assign17450_e25050: f64 = (1e-10 * 0.001);
        let assign17450_e25051: f64 = (assign17450_e25047 + assign17450_e25050);
        (assign17450_e25051, (0.5 * (locals.var_t6w__blk518_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t6w__blk518_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t6w__blk518_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t6w__blk518_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t6w__blk518_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t6w__blk518_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t6w__blk518_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t6w__blk518_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t6__blk517, locals.var_t6__blk517_dn0, locals.var_t6__blk517_dn2, locals.var_t6__blk517_dn6, locals.var_t6__blk517_dn7, locals.var_t6__blk517_dn10, locals.var_t6__blk517_dn11, locals.var_t6__blk517_dn12, locals.var_t6__blk517_dn17,)
    }
};
        locals.var_t6__blk517 = assign17450_e25053;
        locals.var_t6__blk517_dn0 = assign17450_e25053_d_n0;
        locals.var_t6__blk517_dn2 = assign17450_e25053_d_n2;
        locals.var_t6__blk517_dn6 = assign17450_e25053_d_n6;
        locals.var_t6__blk517_dn7 = assign17450_e25053_d_n7;
        locals.var_t6__blk517_dn10 = assign17450_e25053_d_n10;
        locals.var_t6__blk517_dn11 = assign17450_e25053_d_n11;
        locals.var_t6__blk517_dn12 = assign17450_e25053_d_n12;
        locals.var_t6__blk517_dn17 = assign17450_e25053_d_n17;
        locals.var_t6__blk517_rv = 0.0;

        let assign17460_e25056: f64 = if locals.var_t6__blk517 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard529 = assign17460_e25056;
        locals.var_guard529_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17470_e25067, assign17470_e25067_d_n0, assign17470_e25067_d_n2, assign17470_e25067_d_n6, assign17470_e25067_d_n7, assign17470_e25067_d_n10, assign17470_e25067_d_n11, assign17470_e25067_d_n12, assign17470_e25067_d_n17,) = {
    if ((((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) && (locals.var_guard529 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk517, locals.var_t6__blk517_dn0, locals.var_t6__blk517_dn2, locals.var_t6__blk517_dn6, locals.var_t6__blk517_dn7, locals.var_t6__blk517_dn10, locals.var_t6__blk517_dn11, locals.var_t6__blk517_dn12, locals.var_t6__blk517_dn17,)
    }
};
        locals.var_t6__blk517 = assign17470_e25067;
        locals.var_t6__blk517_dn0 = assign17470_e25067_d_n0;
        locals.var_t6__blk517_dn2 = assign17470_e25067_d_n2;
        locals.var_t6__blk517_dn6 = assign17470_e25067_d_n6;
        locals.var_t6__blk517_dn7 = assign17470_e25067_d_n7;
        locals.var_t6__blk517_dn10 = assign17470_e25067_d_n10;
        locals.var_t6__blk517_dn11 = assign17470_e25067_d_n11;
        locals.var_t6__blk517_dn12 = assign17470_e25067_d_n12;
        locals.var_t6__blk517_dn17 = assign17470_e25067_d_n17;
        locals.var_t6__blk517_rv = 0.0;

        let (assign17480_e25078, assign17480_e25078_d_n0, assign17480_e25078_d_n2, assign17480_e25078_d_n6, assign17480_e25078_d_n7, assign17480_e25078_d_n10, assign17480_e25078_d_n11, assign17480_e25078_d_n12, assign17480_e25078_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17480_e25076: f64 = (locals.var_beta * locals.var_qn0);
        (assign17480_e25076, (locals.var_beta * locals.var_qn0_dn0), (locals.var_beta * locals.var_qn0_dn2), (locals.var_beta * locals.var_qn0_dn6), (locals.var_beta * locals.var_qn0_dn7), ((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)), (locals.var_beta * locals.var_qn0_dn11), (locals.var_beta * locals.var_qn0_dn12), (locals.var_beta * locals.var_qn0_dn17),)
    } else {
        (locals.var_t3__blk514, locals.var_t3__blk514_dn0, locals.var_t3__blk514_dn2, locals.var_t3__blk514_dn6, locals.var_t3__blk514_dn7, locals.var_t3__blk514_dn10, locals.var_t3__blk514_dn11, locals.var_t3__blk514_dn12, locals.var_t3__blk514_dn17,)
    }
};
        locals.var_t3__blk514 = assign17480_e25078;
        locals.var_t3__blk514_dn0 = assign17480_e25078_d_n0;
        locals.var_t3__blk514_dn2 = assign17480_e25078_d_n2;
        locals.var_t3__blk514_dn6 = assign17480_e25078_d_n6;
        locals.var_t3__blk514_dn7 = assign17480_e25078_d_n7;
        locals.var_t3__blk514_dn10 = assign17480_e25078_d_n10;
        locals.var_t3__blk514_dn11 = assign17480_e25078_d_n11;
        locals.var_t3__blk514_dn12 = assign17480_e25078_d_n12;
        locals.var_t3__blk514_dn17 = assign17480_e25078_d_n17;
        locals.var_t3__blk514_rv = 0.0;

        let (assign17490_e25089, assign17490_e25089_d_n0, assign17490_e25089_d_n2, assign17490_e25089_d_n6, assign17490_e25089_d_n7, assign17490_e25089_d_n10, assign17490_e25089_d_n11, assign17490_e25089_d_n12, assign17490_e25089_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17490_e25087: f64 = (1.0 / locals.var_t3__blk514);
        (assign17490_e25087, (-(locals.var_t3__blk514_dn0 / (locals.var_t3__blk514 * locals.var_t3__blk514))), (-(locals.var_t3__blk514_dn2 / (locals.var_t3__blk514 * locals.var_t3__blk514))), (-(locals.var_t3__blk514_dn6 / (locals.var_t3__blk514 * locals.var_t3__blk514))), (-(locals.var_t3__blk514_dn7 / (locals.var_t3__blk514 * locals.var_t3__blk514))), (-(locals.var_t3__blk514_dn10 / (locals.var_t3__blk514 * locals.var_t3__blk514))), (-(locals.var_t3__blk514_dn11 / (locals.var_t3__blk514 * locals.var_t3__blk514))), (-(locals.var_t3__blk514_dn12 / (locals.var_t3__blk514 * locals.var_t3__blk514))), (-(locals.var_t3__blk514_dn17 / (locals.var_t3__blk514 * locals.var_t3__blk514))),)
    } else {
        (locals.var_t1__blk512, locals.var_t1__blk512_dn0, locals.var_t1__blk512_dn2, locals.var_t1__blk512_dn6, locals.var_t1__blk512_dn7, locals.var_t1__blk512_dn10, locals.var_t1__blk512_dn11, locals.var_t1__blk512_dn12, locals.var_t1__blk512_dn17,)
    }
};
        locals.var_t1__blk512 = assign17490_e25089;
        locals.var_t1__blk512_dn0 = assign17490_e25089_d_n0;
        locals.var_t1__blk512_dn2 = assign17490_e25089_d_n2;
        locals.var_t1__blk512_dn6 = assign17490_e25089_d_n6;
        locals.var_t1__blk512_dn7 = assign17490_e25089_d_n7;
        locals.var_t1__blk512_dn10 = assign17490_e25089_d_n10;
        locals.var_t1__blk512_dn11 = assign17490_e25089_d_n11;
        locals.var_t1__blk512_dn12 = assign17490_e25089_d_n12;
        locals.var_t1__blk512_dn17 = assign17490_e25089_d_n17;
        locals.var_t1__blk512_rv = 0.0;

        let (assign17500_e25100, assign17500_e25100_d_n0, assign17500_e25100_d_n2, assign17500_e25100_d_n6, assign17500_e25100_d_n7, assign17500_e25100_d_n10, assign17500_e25100_d_n11, assign17500_e25100_d_n12, assign17500_e25100_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17500_e25098: f64 = (locals.var_idd * locals.var_t1__blk512);
        (assign17500_e25098, ((locals.var_idd_dn0 * locals.var_t1__blk512) + (locals.var_idd * locals.var_t1__blk512_dn0)), ((locals.var_idd_dn2 * locals.var_t1__blk512) + (locals.var_idd * locals.var_t1__blk512_dn2)), ((locals.var_idd_dn6 * locals.var_t1__blk512) + (locals.var_idd * locals.var_t1__blk512_dn6)), ((locals.var_idd_dn7 * locals.var_t1__blk512) + (locals.var_idd * locals.var_t1__blk512_dn7)), ((locals.var_idd_dn10 * locals.var_t1__blk512) + (locals.var_idd * locals.var_t1__blk512_dn10)), ((locals.var_idd_dn11 * locals.var_t1__blk512) + (locals.var_idd * locals.var_t1__blk512_dn11)), ((locals.var_idd_dn12 * locals.var_t1__blk512) + (locals.var_idd * locals.var_t1__blk512_dn12)), ((locals.var_idd_dn17 * locals.var_t1__blk512) + (locals.var_idd * locals.var_t1__blk512_dn17)),)
    } else {
        (locals.var_t5__blk516, locals.var_t5__blk516_dn0, locals.var_t5__blk516_dn2, locals.var_t5__blk516_dn6, locals.var_t5__blk516_dn7, locals.var_t5__blk516_dn10, locals.var_t5__blk516_dn11, locals.var_t5__blk516_dn12, locals.var_t5__blk516_dn17,)
    }
};
        locals.var_t5__blk516 = assign17500_e25100;
        locals.var_t5__blk516_dn0 = assign17500_e25100_d_n0;
        locals.var_t5__blk516_dn2 = assign17500_e25100_d_n2;
        locals.var_t5__blk516_dn6 = assign17500_e25100_d_n6;
        locals.var_t5__blk516_dn7 = assign17500_e25100_d_n7;
        locals.var_t5__blk516_dn10 = assign17500_e25100_d_n10;
        locals.var_t5__blk516_dn11 = assign17500_e25100_d_n11;
        locals.var_t5__blk516_dn12 = assign17500_e25100_d_n12;
        locals.var_t5__blk516_dn17 = assign17500_e25100_d_n17;
        locals.var_t5__blk516_rv = 0.0;

        let assign17510_e25103: f64 = if locals.var_t5__blk516 < locals.var_beta_inv { 1.0 } else { 0.0 };
        locals.var_guard530 = assign17510_e25103;
        locals.var_guard530_rv = 0.0;

        let (assign17520_e25114, assign17520_e25114_d_n0, assign17520_e25114_d_n2, assign17520_e25114_d_n6, assign17520_e25114_d_n7, assign17520_e25114_d_n10, assign17520_e25114_d_n11, assign17520_e25114_d_n12, assign17520_e25114_d_n17,) = {
    if ((((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) && (locals.var_guard530 != 0.0)) {
        (locals.var_beta_inv, 0.0, 0.0, 0.0, 0.0, locals.var_beta_inv_dn10, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk516, locals.var_t5__blk516_dn0, locals.var_t5__blk516_dn2, locals.var_t5__blk516_dn6, locals.var_t5__blk516_dn7, locals.var_t5__blk516_dn10, locals.var_t5__blk516_dn11, locals.var_t5__blk516_dn12, locals.var_t5__blk516_dn17,)
    }
};
        locals.var_t5__blk516 = assign17520_e25114;
        locals.var_t5__blk516_dn0 = assign17520_e25114_d_n0;
        locals.var_t5__blk516_dn2 = assign17520_e25114_d_n2;
        locals.var_t5__blk516_dn6 = assign17520_e25114_d_n6;
        locals.var_t5__blk516_dn7 = assign17520_e25114_d_n7;
        locals.var_t5__blk516_dn10 = assign17520_e25114_d_n10;
        locals.var_t5__blk516_dn11 = assign17520_e25114_d_n11;
        locals.var_t5__blk516_dn12 = assign17520_e25114_d_n12;
        locals.var_t5__blk516_dn17 = assign17520_e25114_d_n17;
        locals.var_t5__blk516_rv = 0.0;

        let (assign17530_e25125, assign17530_e25125_d_n0, assign17530_e25125_d_n2, assign17530_e25125_d_n6, assign17530_e25125_d_n7, assign17530_e25125_d_n10, assign17530_e25125_d_n11, assign17530_e25125_d_n12, assign17530_e25125_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17530_e25123: f64 = (locals.var_q_nsub / 1.034943e-10);
        (assign17530_e25123, (locals.var_q_nsub_dn0 / 1.034943e-10), (locals.var_q_nsub_dn2 / 1.034943e-10), (locals.var_q_nsub_dn6 / 1.034943e-10), (locals.var_q_nsub_dn7 / 1.034943e-10), (locals.var_q_nsub_dn10 / 1.034943e-10), (locals.var_q_nsub_dn11 / 1.034943e-10), (locals.var_q_nsub_dn12 / 1.034943e-10), (locals.var_q_nsub_dn17 / 1.034943e-10),)
    } else {
        (locals.var_t10__blk522, locals.var_t10__blk522_dn0, locals.var_t10__blk522_dn2, locals.var_t10__blk522_dn6, locals.var_t10__blk522_dn7, locals.var_t10__blk522_dn10, locals.var_t10__blk522_dn11, locals.var_t10__blk522_dn12, locals.var_t10__blk522_dn17,)
    }
};
        locals.var_t10__blk522 = assign17530_e25125;
        locals.var_t10__blk522_dn0 = assign17530_e25125_d_n0;
        locals.var_t10__blk522_dn2 = assign17530_e25125_d_n2;
        locals.var_t10__blk522_dn6 = assign17530_e25125_d_n6;
        locals.var_t10__blk522_dn7 = assign17530_e25125_d_n7;
        locals.var_t10__blk522_dn10 = assign17530_e25125_d_n10;
        locals.var_t10__blk522_dn11 = assign17530_e25125_d_n11;
        locals.var_t10__blk522_dn12 = assign17530_e25125_d_n12;
        locals.var_t10__blk522_dn17 = assign17530_e25125_d_n17;
        locals.var_t10__blk522_rv = 0.0;

        let (assign17540_e25136, assign17540_e25136_d_n0, assign17540_e25136_d_n2, assign17540_e25136_d_n6, assign17540_e25136_d_n7, assign17540_e25136_d_n10, assign17540_e25136_d_n11, assign17540_e25136_d_n12, assign17540_e25136_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17540_e25134: f64 = (100000.0 * 10000.0);
        (assign17540_e25134, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk512, locals.var_t1__blk512_dn0, locals.var_t1__blk512_dn2, locals.var_t1__blk512_dn6, locals.var_t1__blk512_dn7, locals.var_t1__blk512_dn10, locals.var_t1__blk512_dn11, locals.var_t1__blk512_dn12, locals.var_t1__blk512_dn17,)
    }
};
        locals.var_t1__blk512 = assign17540_e25136;
        locals.var_t1__blk512_dn0 = assign17540_e25136_d_n0;
        locals.var_t1__blk512_dn2 = assign17540_e25136_d_n2;
        locals.var_t1__blk512_dn6 = assign17540_e25136_d_n6;
        locals.var_t1__blk512_dn7 = assign17540_e25136_d_n7;
        locals.var_t1__blk512_dn10 = assign17540_e25136_d_n10;
        locals.var_t1__blk512_dn11 = assign17540_e25136_d_n11;
        locals.var_t1__blk512_dn12 = assign17540_e25136_d_n12;
        locals.var_t1__blk512_dn17 = assign17540_e25136_d_n17;
        locals.var_t1__blk512_rv = 0.0;

        let (assign17550_e25147, assign17550_e25147_d_n0, assign17550_e25147_d_n2, assign17550_e25147_d_n6, assign17550_e25147_d_n7, assign17550_e25147_d_n10, assign17550_e25147_d_n11, assign17550_e25147_d_n12, assign17550_e25147_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17550_e25145: f64 = (1.0 / locals.var_leff);
        (assign17550_e25145, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk513, locals.var_t2__blk513_dn0, locals.var_t2__blk513_dn2, locals.var_t2__blk513_dn6, locals.var_t2__blk513_dn7, locals.var_t2__blk513_dn10, locals.var_t2__blk513_dn11, locals.var_t2__blk513_dn12, locals.var_t2__blk513_dn17,)
    }
};
        locals.var_t2__blk513 = assign17550_e25147;
        locals.var_t2__blk513_dn0 = assign17550_e25147_d_n0;
        locals.var_t2__blk513_dn2 = assign17550_e25147_d_n2;
        locals.var_t2__blk513_dn6 = assign17550_e25147_d_n6;
        locals.var_t2__blk513_dn7 = assign17550_e25147_d_n7;
        locals.var_t2__blk513_dn10 = assign17550_e25147_d_n10;
        locals.var_t2__blk513_dn11 = assign17550_e25147_d_n11;
        locals.var_t2__blk513_dn12 = assign17550_e25147_d_n12;
        locals.var_t2__blk513_dn17 = assign17550_e25147_d_n17;
        locals.var_t2__blk513_rv = 0.0;

        let (assign17560_e25172, assign17560_e25172_d_n0, assign17560_e25172_d_n2, assign17560_e25172_d_n6, assign17560_e25172_d_n7, assign17560_e25172_d_n10, assign17560_e25172_d_n11, assign17560_e25172_d_n12, assign17560_e25172_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17560_e25156: f64 = (2.0 * locals.var_t5__blk516);
        let assign17560_e25159: f64 = (2.0 * locals.var_t10__blk522);
        let assign17560_e25161: f64 = (assign17560_e25159 * locals.var_t6__blk517);
        let assign17560_e25163: f64 = (assign17560_e25161 * locals.var_t4__blk515);
        let assign17560_e25164: f64 = (assign17560_e25156 + assign17560_e25163);
        let assign17560_e25167: f64 = (locals.var_t1__blk512 * locals.var_t4__blk515);
        let assign17560_e25168: f64 = (assign17560_e25164 + assign17560_e25167);
        let assign17560_e25170: f64 = (assign17560_e25168 * locals.var_t2__blk513);
        (assign17560_e25170, (((((2.0 * locals.var_t5__blk516_dn0) + (((((2.0 * locals.var_t10__blk522_dn0) * locals.var_t6__blk517) + (assign17560_e25159 * locals.var_t6__blk517_dn0)) * locals.var_t4__blk515) + (assign17560_e25161 * locals.var_t4__blk515_dn0))) + ((locals.var_t1__blk512_dn0 * locals.var_t4__blk515) + (locals.var_t1__blk512 * locals.var_t4__blk515_dn0))) * locals.var_t2__blk513) + (assign17560_e25168 * locals.var_t2__blk513_dn0)), (((((2.0 * locals.var_t5__blk516_dn2) + (((((2.0 * locals.var_t10__blk522_dn2) * locals.var_t6__blk517) + (assign17560_e25159 * locals.var_t6__blk517_dn2)) * locals.var_t4__blk515) + (assign17560_e25161 * locals.var_t4__blk515_dn2))) + ((locals.var_t1__blk512_dn2 * locals.var_t4__blk515) + (locals.var_t1__blk512 * locals.var_t4__blk515_dn2))) * locals.var_t2__blk513) + (assign17560_e25168 * locals.var_t2__blk513_dn2)), (((((2.0 * locals.var_t5__blk516_dn6) + (((((2.0 * locals.var_t10__blk522_dn6) * locals.var_t6__blk517) + (assign17560_e25159 * locals.var_t6__blk517_dn6)) * locals.var_t4__blk515) + (assign17560_e25161 * locals.var_t4__blk515_dn6))) + ((locals.var_t1__blk512_dn6 * locals.var_t4__blk515) + (locals.var_t1__blk512 * locals.var_t4__blk515_dn6))) * locals.var_t2__blk513) + (assign17560_e25168 * locals.var_t2__blk513_dn6)), (((((2.0 * locals.var_t5__blk516_dn7) + (((((2.0 * locals.var_t10__blk522_dn7) * locals.var_t6__blk517) + (assign17560_e25159 * locals.var_t6__blk517_dn7)) * locals.var_t4__blk515) + (assign17560_e25161 * locals.var_t4__blk515_dn7))) + ((locals.var_t1__blk512_dn7 * locals.var_t4__blk515) + (locals.var_t1__blk512 * locals.var_t4__blk515_dn7))) * locals.var_t2__blk513) + (assign17560_e25168 * locals.var_t2__blk513_dn7)), (((((2.0 * locals.var_t5__blk516_dn10) + (((((2.0 * locals.var_t10__blk522_dn10) * locals.var_t6__blk517) + (assign17560_e25159 * locals.var_t6__blk517_dn10)) * locals.var_t4__blk515) + (assign17560_e25161 * locals.var_t4__blk515_dn10))) + ((locals.var_t1__blk512_dn10 * locals.var_t4__blk515) + (locals.var_t1__blk512 * locals.var_t4__blk515_dn10))) * locals.var_t2__blk513) + (assign17560_e25168 * locals.var_t2__blk513_dn10)), (((((2.0 * locals.var_t5__blk516_dn11) + (((((2.0 * locals.var_t10__blk522_dn11) * locals.var_t6__blk517) + (assign17560_e25159 * locals.var_t6__blk517_dn11)) * locals.var_t4__blk515) + (assign17560_e25161 * locals.var_t4__blk515_dn11))) + ((locals.var_t1__blk512_dn11 * locals.var_t4__blk515) + (locals.var_t1__blk512 * locals.var_t4__blk515_dn11))) * locals.var_t2__blk513) + (assign17560_e25168 * locals.var_t2__blk513_dn11)), (((((2.0 * locals.var_t5__blk516_dn12) + (((((2.0 * locals.var_t10__blk522_dn12) * locals.var_t6__blk517) + (assign17560_e25159 * locals.var_t6__blk517_dn12)) * locals.var_t4__blk515) + (assign17560_e25161 * locals.var_t4__blk515_dn12))) + ((locals.var_t1__blk512_dn12 * locals.var_t4__blk515) + (locals.var_t1__blk512 * locals.var_t4__blk515_dn12))) * locals.var_t2__blk513) + (assign17560_e25168 * locals.var_t2__blk513_dn12)), (((((2.0 * locals.var_t5__blk516_dn17) + (((((2.0 * locals.var_t10__blk522_dn17) * locals.var_t6__blk517) + (assign17560_e25159 * locals.var_t6__blk517_dn17)) * locals.var_t4__blk515) + (assign17560_e25161 * locals.var_t4__blk515_dn17))) + ((locals.var_t1__blk512_dn17 * locals.var_t4__blk515) + (locals.var_t1__blk512 * locals.var_t4__blk515_dn17))) * locals.var_t2__blk513) + (assign17560_e25168 * locals.var_t2__blk513_dn17)),)
    } else {
        (locals.var_t11w, locals.var_t11w_dn0, locals.var_t11w_dn2, locals.var_t11w_dn6, locals.var_t11w_dn7, locals.var_t11w_dn10, locals.var_t11w_dn11, locals.var_t11w_dn12, locals.var_t11w_dn17,)
    }
};
        locals.var_t11w = assign17560_e25172;
        locals.var_t11w_dn0 = assign17560_e25172_d_n0;
        locals.var_t11w_dn2 = assign17560_e25172_d_n2;
        locals.var_t11w_dn6 = assign17560_e25172_d_n6;
        locals.var_t11w_dn7 = assign17560_e25172_d_n7;
        locals.var_t11w_dn10 = assign17560_e25172_d_n10;
        locals.var_t11w_dn11 = assign17560_e25172_d_n11;
        locals.var_t11w_dn12 = assign17560_e25172_d_n12;
        locals.var_t11w_dn17 = assign17560_e25172_d_n17;
        locals.var_t11w_rv = 0.0;

        let (assign17570_e25183, assign17570_e25183_d_n0, assign17570_e25183_d_n2, assign17570_e25183_d_n6, assign17570_e25183_d_n7, assign17570_e25183_d_n10, assign17570_e25183_d_n11, assign17570_e25183_d_n12, assign17570_e25183_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17570_e25181: f64 = (locals.var_t11w * locals.var_t4__blk515);
        (assign17570_e25181, ((locals.var_t11w_dn0 * locals.var_t4__blk515) + (locals.var_t11w * locals.var_t4__blk515_dn0)), ((locals.var_t11w_dn2 * locals.var_t4__blk515) + (locals.var_t11w * locals.var_t4__blk515_dn2)), ((locals.var_t11w_dn6 * locals.var_t4__blk515) + (locals.var_t11w * locals.var_t4__blk515_dn6)), ((locals.var_t11w_dn7 * locals.var_t4__blk515) + (locals.var_t11w * locals.var_t4__blk515_dn7)), ((locals.var_t11w_dn10 * locals.var_t4__blk515) + (locals.var_t11w * locals.var_t4__blk515_dn10)), ((locals.var_t11w_dn11 * locals.var_t4__blk515) + (locals.var_t11w * locals.var_t4__blk515_dn11)), ((locals.var_t11w_dn12 * locals.var_t4__blk515) + (locals.var_t11w * locals.var_t4__blk515_dn12)), ((locals.var_t11w_dn17 * locals.var_t4__blk515) + (locals.var_t11w * locals.var_t4__blk515_dn17)),)
    } else {
        (locals.var_t7__blk519, locals.var_t7__blk519_dn0, locals.var_t7__blk519_dn2, locals.var_t7__blk519_dn6, locals.var_t7__blk519_dn7, locals.var_t7__blk519_dn10, locals.var_t7__blk519_dn11, locals.var_t7__blk519_dn12, locals.var_t7__blk519_dn17,)
    }
};
        locals.var_t7__blk519 = assign17570_e25183;
        locals.var_t7__blk519_dn0 = assign17570_e25183_d_n0;
        locals.var_t7__blk519_dn2 = assign17570_e25183_d_n2;
        locals.var_t7__blk519_dn6 = assign17570_e25183_d_n6;
        locals.var_t7__blk519_dn7 = assign17570_e25183_d_n7;
        locals.var_t7__blk519_dn10 = assign17570_e25183_d_n10;
        locals.var_t7__blk519_dn11 = assign17570_e25183_d_n11;
        locals.var_t7__blk519_dn12 = assign17570_e25183_d_n12;
        locals.var_t7__blk519_dn17 = assign17570_e25183_d_n17;
        locals.var_t7__blk519_rv = 0.0;

        let (assign17580_e25200, assign17580_e25200_d_n0, assign17580_e25200_d_n2, assign17580_e25200_d_n6, assign17580_e25200_d_n7, assign17580_e25200_d_n10, assign17580_e25200_d_n11, assign17580_e25200_d_n12, assign17580_e25200_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17580_e25193: f64 = (2.0 * locals.var_t10__blk522);
        let assign17580_e25195: f64 = (assign17580_e25193 * locals.var_t6__blk517);
        let assign17580_e25197: f64 = (assign17580_e25195 + locals.var_t1__blk512);
        let assign17580_e25198: f64 = (4.0 * assign17580_e25197);
        (assign17580_e25198, (4.0 * ((((2.0 * locals.var_t10__blk522_dn0) * locals.var_t6__blk517) + (assign17580_e25193 * locals.var_t6__blk517_dn0)) + locals.var_t1__blk512_dn0)), (4.0 * ((((2.0 * locals.var_t10__blk522_dn2) * locals.var_t6__blk517) + (assign17580_e25193 * locals.var_t6__blk517_dn2)) + locals.var_t1__blk512_dn2)), (4.0 * ((((2.0 * locals.var_t10__blk522_dn6) * locals.var_t6__blk517) + (assign17580_e25193 * locals.var_t6__blk517_dn6)) + locals.var_t1__blk512_dn6)), (4.0 * ((((2.0 * locals.var_t10__blk522_dn7) * locals.var_t6__blk517) + (assign17580_e25193 * locals.var_t6__blk517_dn7)) + locals.var_t1__blk512_dn7)), (4.0 * ((((2.0 * locals.var_t10__blk522_dn10) * locals.var_t6__blk517) + (assign17580_e25193 * locals.var_t6__blk517_dn10)) + locals.var_t1__blk512_dn10)), (4.0 * ((((2.0 * locals.var_t10__blk522_dn11) * locals.var_t6__blk517) + (assign17580_e25193 * locals.var_t6__blk517_dn11)) + locals.var_t1__blk512_dn11)), (4.0 * ((((2.0 * locals.var_t10__blk522_dn12) * locals.var_t6__blk517) + (assign17580_e25193 * locals.var_t6__blk517_dn12)) + locals.var_t1__blk512_dn12)), (4.0 * ((((2.0 * locals.var_t10__blk522_dn17) * locals.var_t6__blk517) + (assign17580_e25193 * locals.var_t6__blk517_dn17)) + locals.var_t1__blk512_dn17)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12, locals.var_t11_dn17,)
    }
};
        locals.var_t11 = assign17580_e25200;
        locals.var_t11_dn0 = assign17580_e25200_d_n0;
        locals.var_t11_dn2 = assign17580_e25200_d_n2;
        locals.var_t11_dn6 = assign17580_e25200_d_n6;
        locals.var_t11_dn7 = assign17580_e25200_d_n7;
        locals.var_t11_dn10 = assign17580_e25200_d_n10;
        locals.var_t11_dn11 = assign17580_e25200_d_n11;
        locals.var_t11_dn12 = assign17580_e25200_d_n12;
        locals.var_t11_dn17 = assign17580_e25200_d_n17;
        locals.var_t11_rv = 0.0;

        let (assign17590_e25213, assign17590_e25213_d_n0, assign17590_e25213_d_n2, assign17590_e25213_d_n6, assign17590_e25213_d_n7, assign17590_e25213_d_n10, assign17590_e25213_d_n11, assign17590_e25213_d_n12, assign17590_e25213_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17590_e25209: f64 = (locals.var_t11 * locals.var_t4__blk515);
        let assign17590_e25211: f64 = (assign17590_e25209 * locals.var_t4__blk515);
        (assign17590_e25211, ((((locals.var_t11_dn0 * locals.var_t4__blk515) + (locals.var_t11 * locals.var_t4__blk515_dn0)) * locals.var_t4__blk515) + (assign17590_e25209 * locals.var_t4__blk515_dn0)), ((((locals.var_t11_dn2 * locals.var_t4__blk515) + (locals.var_t11 * locals.var_t4__blk515_dn2)) * locals.var_t4__blk515) + (assign17590_e25209 * locals.var_t4__blk515_dn2)), ((((locals.var_t11_dn6 * locals.var_t4__blk515) + (locals.var_t11 * locals.var_t4__blk515_dn6)) * locals.var_t4__blk515) + (assign17590_e25209 * locals.var_t4__blk515_dn6)), ((((locals.var_t11_dn7 * locals.var_t4__blk515) + (locals.var_t11 * locals.var_t4__blk515_dn7)) * locals.var_t4__blk515) + (assign17590_e25209 * locals.var_t4__blk515_dn7)), ((((locals.var_t11_dn10 * locals.var_t4__blk515) + (locals.var_t11 * locals.var_t4__blk515_dn10)) * locals.var_t4__blk515) + (assign17590_e25209 * locals.var_t4__blk515_dn10)), ((((locals.var_t11_dn11 * locals.var_t4__blk515) + (locals.var_t11 * locals.var_t4__blk515_dn11)) * locals.var_t4__blk515) + (assign17590_e25209 * locals.var_t4__blk515_dn11)), ((((locals.var_t11_dn12 * locals.var_t4__blk515) + (locals.var_t11 * locals.var_t4__blk515_dn12)) * locals.var_t4__blk515) + (assign17590_e25209 * locals.var_t4__blk515_dn12)), ((((locals.var_t11_dn17 * locals.var_t4__blk515) + (locals.var_t11 * locals.var_t4__blk515_dn17)) * locals.var_t4__blk515) + (assign17590_e25209 * locals.var_t4__blk515_dn17)),)
    } else {
        (locals.var_t8__blk520, locals.var_t8__blk520_dn0, locals.var_t8__blk520_dn2, locals.var_t8__blk520_dn6, locals.var_t8__blk520_dn7, locals.var_t8__blk520_dn10, locals.var_t8__blk520_dn11, locals.var_t8__blk520_dn12, locals.var_t8__blk520_dn17,)
    }
};
        locals.var_t8__blk520 = assign17590_e25213;
        locals.var_t8__blk520_dn0 = assign17590_e25213_d_n0;
        locals.var_t8__blk520_dn2 = assign17590_e25213_d_n2;
        locals.var_t8__blk520_dn6 = assign17590_e25213_d_n6;
        locals.var_t8__blk520_dn7 = assign17590_e25213_d_n7;
        locals.var_t8__blk520_dn10 = assign17590_e25213_d_n10;
        locals.var_t8__blk520_dn11 = assign17590_e25213_d_n11;
        locals.var_t8__blk520_dn12 = assign17590_e25213_d_n12;
        locals.var_t8__blk520_dn17 = assign17590_e25213_d_n17;
        locals.var_t8__blk520_rv = 0.0;

        let (assign17600_e25227, assign17600_e25227_d_n0, assign17600_e25227_d_n2, assign17600_e25227_d_n6, assign17600_e25227_d_n7, assign17600_e25227_d_n10, assign17600_e25227_d_n11, assign17600_e25227_d_n12, assign17600_e25227_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17600_e25222: f64 = (locals.var_t7__blk519 * locals.var_t7__blk519);
        let assign17600_e25224: f64 = (assign17600_e25222 + locals.var_t8__blk520);
        let assign17600_e25225: f64 = (assign17600_e25224).sqrt();
        (assign17600_e25225, ((((locals.var_t7__blk519_dn0 * locals.var_t7__blk519) + (locals.var_t7__blk519 * locals.var_t7__blk519_dn0)) + locals.var_t8__blk520_dn0) / (2.0 * assign17600_e25225)), ((((locals.var_t7__blk519_dn2 * locals.var_t7__blk519) + (locals.var_t7__blk519 * locals.var_t7__blk519_dn2)) + locals.var_t8__blk520_dn2) / (2.0 * assign17600_e25225)), ((((locals.var_t7__blk519_dn6 * locals.var_t7__blk519) + (locals.var_t7__blk519 * locals.var_t7__blk519_dn6)) + locals.var_t8__blk520_dn6) / (2.0 * assign17600_e25225)), ((((locals.var_t7__blk519_dn7 * locals.var_t7__blk519) + (locals.var_t7__blk519 * locals.var_t7__blk519_dn7)) + locals.var_t8__blk520_dn7) / (2.0 * assign17600_e25225)), ((((locals.var_t7__blk519_dn10 * locals.var_t7__blk519) + (locals.var_t7__blk519 * locals.var_t7__blk519_dn10)) + locals.var_t8__blk520_dn10) / (2.0 * assign17600_e25225)), ((((locals.var_t7__blk519_dn11 * locals.var_t7__blk519) + (locals.var_t7__blk519 * locals.var_t7__blk519_dn11)) + locals.var_t8__blk520_dn11) / (2.0 * assign17600_e25225)), ((((locals.var_t7__blk519_dn12 * locals.var_t7__blk519) + (locals.var_t7__blk519 * locals.var_t7__blk519_dn12)) + locals.var_t8__blk520_dn12) / (2.0 * assign17600_e25225)), ((((locals.var_t7__blk519_dn17 * locals.var_t7__blk519) + (locals.var_t7__blk519 * locals.var_t7__blk519_dn17)) + locals.var_t8__blk520_dn17) / (2.0 * assign17600_e25225)),)
    } else {
        (locals.var_t9__blk521, locals.var_t9__blk521_dn0, locals.var_t9__blk521_dn2, locals.var_t9__blk521_dn6, locals.var_t9__blk521_dn7, locals.var_t9__blk521_dn10, locals.var_t9__blk521_dn11, locals.var_t9__blk521_dn12, locals.var_t9__blk521_dn17,)
    }
};
        locals.var_t9__blk521 = assign17600_e25227;
        locals.var_t9__blk521_dn0 = assign17600_e25227_d_n0;
        locals.var_t9__blk521_dn2 = assign17600_e25227_d_n2;
        locals.var_t9__blk521_dn6 = assign17600_e25227_d_n6;
        locals.var_t9__blk521_dn7 = assign17600_e25227_d_n7;
        locals.var_t9__blk521_dn10 = assign17600_e25227_d_n10;
        locals.var_t9__blk521_dn11 = assign17600_e25227_d_n11;
        locals.var_t9__blk521_dn12 = assign17600_e25227_d_n12;
        locals.var_t9__blk521_dn17 = assign17600_e25227_d_n17;
        locals.var_t9__blk521_rv = 0.0;

        let (assign17610_e25243, assign17610_e25243_d_n0, assign17610_e25243_d_n2, assign17610_e25243_d_n6, assign17610_e25243_d_n7, assign17610_e25243_d_n10, assign17610_e25243_d_n11, assign17610_e25243_d_n12, assign17610_e25243_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign17610_e25237: f64 = (-locals.var_t7__blk519);
        let assign17610_e25239: f64 = (assign17610_e25237 + locals.var_t9__blk521);
        let assign17610_e25240: f64 = (0.5 * assign17610_e25239);
        let assign17610_e25241: f64 = (locals.var_fmdvds * assign17610_e25240);
        (assign17610_e25241, ((locals.var_fmdvds_dn0 * assign17610_e25240) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk519_dn0) + locals.var_t9__blk521_dn0)))), ((locals.var_fmdvds_dn2 * assign17610_e25240) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk519_dn2) + locals.var_t9__blk521_dn2)))), ((locals.var_fmdvds_dn6 * assign17610_e25240) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk519_dn6) + locals.var_t9__blk521_dn6)))), ((locals.var_fmdvds_dn7 * assign17610_e25240) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk519_dn7) + locals.var_t9__blk521_dn7)))), ((locals.var_fmdvds_dn10 * assign17610_e25240) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk519_dn10) + locals.var_t9__blk521_dn10)))), ((locals.var_fmdvds_dn11 * assign17610_e25240) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk519_dn11) + locals.var_t9__blk521_dn11)))), ((locals.var_fmdvds_dn12 * assign17610_e25240) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk519_dn12) + locals.var_t9__blk521_dn12)))), ((locals.var_fmdvds_dn17 * assign17610_e25240) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk519_dn17) + locals.var_t9__blk521_dn17)))),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17610_e25243;
        locals.var_lred_dn0 = assign17610_e25243_d_n0;
        locals.var_lred_dn2 = assign17610_e25243_d_n2;
        locals.var_lred_dn6 = assign17610_e25243_d_n6;
        locals.var_lred_dn7 = assign17610_e25243_d_n7;
        locals.var_lred_dn10 = assign17610_e25243_d_n10;
        locals.var_lred_dn11 = assign17610_e25243_d_n11;
        locals.var_lred_dn12 = assign17610_e25243_d_n12;
        locals.var_lred_dn17 = assign17610_e25243_d_n17;
        locals.var_lred_rv = 0.0;

        let (assign17620_e25251, assign17620_e25251_d_n0, assign17620_e25251_d_n2, assign17620_e25251_d_n6, assign17620_e25251_d_n7, assign17620_e25251_d_n10, assign17620_e25251_d_n11, assign17620_e25251_d_n12, assign17620_e25251_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard510 != 0.0)) {
        let assign17620_e25249: f64 = (locals.var_lred * locals.var_clmmod);
        (assign17620_e25249, (locals.var_lred_dn0 * locals.var_clmmod), (locals.var_lred_dn2 * locals.var_clmmod), (locals.var_lred_dn6 * locals.var_clmmod), (locals.var_lred_dn7 * locals.var_clmmod), (locals.var_lred_dn10 * locals.var_clmmod), (locals.var_lred_dn11 * locals.var_clmmod), (locals.var_lred_dn12 * locals.var_clmmod), (locals.var_lred_dn17 * locals.var_clmmod),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17620_e25251;
        locals.var_lred_dn0 = assign17620_e25251_d_n0;
        locals.var_lred_dn2 = assign17620_e25251_d_n2;
        locals.var_lred_dn6 = assign17620_e25251_d_n6;
        locals.var_lred_dn7 = assign17620_e25251_d_n7;
        locals.var_lred_dn10 = assign17620_e25251_d_n10;
        locals.var_lred_dn11 = assign17620_e25251_d_n11;
        locals.var_lred_dn12 = assign17620_e25251_d_n12;
        locals.var_lred_dn17 = assign17620_e25251_d_n17;
        locals.var_lred_rv = 0.0;

        let (assign17630_e25257, assign17630_e25257_d_n0, assign17630_e25257_d_n2, assign17630_e25257_d_n6, assign17630_e25257_d_n7, assign17630_e25257_d_n10, assign17630_e25257_d_n11, assign17630_e25257_d_n12, assign17630_e25257_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        let assign17630_e25255: f64 = (locals.var_leff - locals.var_lred);
        (assign17630_e25255, (-locals.var_lred_dn0), (-locals.var_lred_dn2), (-locals.var_lred_dn6), (-locals.var_lred_dn7), (-locals.var_lred_dn10), (-locals.var_lred_dn11), (-locals.var_lred_dn12), (-locals.var_lred_dn17),)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn12, locals.var_lch_dn17,)
    }
};
        locals.var_lch = assign17630_e25257;
        locals.var_lch_dn0 = assign17630_e25257_d_n0;
        locals.var_lch_dn2 = assign17630_e25257_d_n2;
        locals.var_lch_dn6 = assign17630_e25257_d_n6;
        locals.var_lch_dn7 = assign17630_e25257_d_n7;
        locals.var_lch_dn10 = assign17630_e25257_d_n10;
        locals.var_lch_dn11 = assign17630_e25257_d_n11;
        locals.var_lch_dn12 = assign17630_e25257_d_n12;
        locals.var_lch_dn17 = assign17630_e25257_d_n17;
        locals.var_lch_rv = 0.0;

        let assign17650_e25266: f64 = if locals.var_lch < 1e-9 { 1.0 } else { 0.0 };
        locals.var_guard531 = assign17650_e25266;
        locals.var_guard531_rv = 0.0;

        let (assign17660_e25272, assign17660_e25272_d_n0, assign17660_e25272_d_n2, assign17660_e25272_d_n6, assign17660_e25272_d_n7, assign17660_e25272_d_n10, assign17660_e25272_d_n11, assign17660_e25272_d_n12, assign17660_e25272_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard531 != 0.0)) {
        (1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn12, locals.var_lch_dn17,)
    }
};
        locals.var_lch = assign17660_e25272;
        locals.var_lch_dn0 = assign17660_e25272_d_n0;
        locals.var_lch_dn2 = assign17660_e25272_d_n2;
        locals.var_lch_dn6 = assign17660_e25272_d_n6;
        locals.var_lch_dn7 = assign17660_e25272_d_n7;
        locals.var_lch_dn10 = assign17660_e25272_d_n10;
        locals.var_lch_dn11 = assign17660_e25272_d_n11;
        locals.var_lch_dn12 = assign17660_e25272_d_n12;
        locals.var_lch_dn17 = assign17660_e25272_d_n17;
        locals.var_lch_rv = 0.0;

        let (assign17670_e25279, assign17670_e25279_d_n0, assign17670_e25279_d_n2, assign17670_e25279_d_n6, assign17670_e25279_d_n7, assign17670_e25279_d_n10, assign17670_e25279_d_n11, assign17670_e25279_d_n12, assign17670_e25279_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        let assign17670_e25275: f64 = (-locals.var_weffcv_nf);
        let assign17670_e25277: f64 = (assign17670_e25275 * locals.var_leff_cv);
        (assign17670_e25277, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign17670_e25279;
        locals.var_t1_dn0 = assign17670_e25279_d_n0;
        locals.var_t1_dn2 = assign17670_e25279_d_n2;
        locals.var_t1_dn6 = assign17670_e25279_d_n6;
        locals.var_t1_dn7 = assign17670_e25279_d_n7;
        locals.var_t1_dn10 = assign17670_e25279_d_n10;
        locals.var_t1_dn11 = assign17670_e25279_d_n11;
        locals.var_t1_dn12 = assign17670_e25279_d_n12;
        locals.var_t1_dn17 = assign17670_e25279_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign17680_e25285, assign17680_e25285_d_n0, assign17680_e25285_d_n2, assign17680_e25285_d_n6, assign17680_e25285_d_n7, assign17680_e25285_d_n10, assign17680_e25285_d_n11, assign17680_e25285_d_n12, assign17680_e25285_d_n13, assign17680_e25285_d_n15, assign17680_e25285_d_n16, assign17680_e25285_d_n17, assign17680_e25285_d_n18,) = {
    if (locals.var_guard507 != 0.0) {
        let assign17680_e25283: f64 = (locals.var_t1 * locals.var_qbu);
        (assign17680_e25283, ((locals.var_t1_dn0 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn0)), ((locals.var_t1_dn2 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn2)), ((locals.var_t1_dn6 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn6)), ((locals.var_t1_dn7 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn7)), ((locals.var_t1_dn10 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn10)), ((locals.var_t1_dn11 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn11)), ((locals.var_t1_dn12 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn12)), 0.0, 0.0, 0.0, ((locals.var_t1_dn17 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn17)), 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign17680_e25285;
        locals.var_qb_dn0 = assign17680_e25285_d_n0;
        locals.var_qb_dn2 = assign17680_e25285_d_n2;
        locals.var_qb_dn6 = assign17680_e25285_d_n6;
        locals.var_qb_dn7 = assign17680_e25285_d_n7;
        locals.var_qb_dn10 = assign17680_e25285_d_n10;
        locals.var_qb_dn11 = assign17680_e25285_d_n11;
        locals.var_qb_dn12 = assign17680_e25285_d_n12;
        locals.var_qb_dn13 = assign17680_e25285_d_n13;
        locals.var_qb_dn15 = assign17680_e25285_d_n15;
        locals.var_qb_dn16 = assign17680_e25285_d_n16;
        locals.var_qb_dn17 = assign17680_e25285_d_n17;
        locals.var_qb_dn18 = assign17680_e25285_d_n18;
        locals.var_qb_rv = 0.0;

        let (assign17690_e25291, assign17690_e25291_d_n0, assign17690_e25291_d_n2, assign17690_e25291_d_n6, assign17690_e25291_d_n7, assign17690_e25291_d_n10, assign17690_e25291_d_n11, assign17690_e25291_d_n12, assign17690_e25291_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        let assign17690_e25289: f64 = (locals.var_t1 * locals.var_qiu);
        (assign17690_e25289, ((locals.var_t1_dn0 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn0)), ((locals.var_t1_dn2 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn2)), ((locals.var_t1_dn6 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn6)), ((locals.var_t1_dn7 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn7)), ((locals.var_t1_dn10 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn10)), ((locals.var_t1_dn11 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn11)), ((locals.var_t1_dn12 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn12)), ((locals.var_t1_dn17 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn17)),)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, locals.var_qi_dn17,)
    }
};
        locals.var_qi = assign17690_e25291;
        locals.var_qi_dn0 = assign17690_e25291_d_n0;
        locals.var_qi_dn2 = assign17690_e25291_d_n2;
        locals.var_qi_dn6 = assign17690_e25291_d_n6;
        locals.var_qi_dn7 = assign17690_e25291_d_n7;
        locals.var_qi_dn10 = assign17690_e25291_d_n10;
        locals.var_qi_dn11 = assign17690_e25291_d_n11;
        locals.var_qi_dn12 = assign17690_e25291_d_n12;
        locals.var_qi_dn17 = assign17690_e25291_d_n17;
        locals.var_qi_rv = 0.0;

        let (assign17700_e25297, assign17700_e25297_d_n0, assign17700_e25297_d_n2, assign17700_e25297_d_n6, assign17700_e25297_d_n7, assign17700_e25297_d_n10, assign17700_e25297_d_n11, assign17700_e25297_d_n12, assign17700_e25297_d_n13, assign17700_e25297_d_n15, assign17700_e25297_d_n16, assign17700_e25297_d_n17, assign17700_e25297_d_n18,) = {
    if (locals.var_guard507 != 0.0) {
        let assign17700_e25295: f64 = (locals.var_qi * locals.var_qdrat);
        (assign17700_e25295, ((locals.var_qi_dn0 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn0)), ((locals.var_qi_dn2 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn2)), ((locals.var_qi_dn6 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn6)), ((locals.var_qi_dn7 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn7)), ((locals.var_qi_dn10 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn10)), ((locals.var_qi_dn11 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn11)), ((locals.var_qi_dn12 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn12)), 0.0, 0.0, 0.0, ((locals.var_qi_dn17 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn17)), 0.0,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign17700_e25297;
        locals.var_qd_dn0 = assign17700_e25297_d_n0;
        locals.var_qd_dn2 = assign17700_e25297_d_n2;
        locals.var_qd_dn6 = assign17700_e25297_d_n6;
        locals.var_qd_dn7 = assign17700_e25297_d_n7;
        locals.var_qd_dn10 = assign17700_e25297_d_n10;
        locals.var_qd_dn11 = assign17700_e25297_d_n11;
        locals.var_qd_dn12 = assign17700_e25297_d_n12;
        locals.var_qd_dn13 = assign17700_e25297_d_n13;
        locals.var_qd_dn15 = assign17700_e25297_d_n15;
        locals.var_qd_dn16 = assign17700_e25297_d_n16;
        locals.var_qd_dn17 = assign17700_e25297_d_n17;
        locals.var_qd_dn18 = assign17700_e25297_d_n18;
        locals.var_qd_rv = 0.0;

        let assign17710_e25300: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard532 = assign17710_e25300;
        locals.var_guard532_rv = 0.0;

        let (assign17720_e25308, assign17720_e25308_d_n0, assign17720_e25308_d_n2, assign17720_e25308_d_n6, assign17720_e25308_d_n7, assign17720_e25308_d_n10, assign17720_e25308_d_n11, assign17720_e25308_d_n12, assign17720_e25308_d_n13, assign17720_e25308_d_n15, assign17720_e25308_d_n16, assign17720_e25308_d_n17, assign17720_e25308_d_n18,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard532 != 0.0)) {
        let assign17720_e25306: f64 = (locals.var_qb * 0.5);
        (assign17720_e25306, (locals.var_qb_dn0 * 0.5), (locals.var_qb_dn2 * 0.5), (locals.var_qb_dn6 * 0.5), (locals.var_qb_dn7 * 0.5), (locals.var_qb_dn10 * 0.5), (locals.var_qb_dn11 * 0.5), (locals.var_qb_dn12 * 0.5), (locals.var_qb_dn13 * 0.5), (locals.var_qb_dn15 * 0.5), (locals.var_qb_dn16 * 0.5), (locals.var_qb_dn17 * 0.5), (locals.var_qb_dn18 * 0.5),)
    } else {
        (locals.var_qd_fb, locals.var_qd_fb_dn0, locals.var_qd_fb_dn2, locals.var_qd_fb_dn6, locals.var_qd_fb_dn7, locals.var_qd_fb_dn10, locals.var_qd_fb_dn11, locals.var_qd_fb_dn12, locals.var_qd_fb_dn13, locals.var_qd_fb_dn15, locals.var_qd_fb_dn16, locals.var_qd_fb_dn17, locals.var_qd_fb_dn18,)
    }
};
        locals.var_qd_fb = assign17720_e25308;
        locals.var_qd_fb_dn0 = assign17720_e25308_d_n0;
        locals.var_qd_fb_dn2 = assign17720_e25308_d_n2;
        locals.var_qd_fb_dn6 = assign17720_e25308_d_n6;
        locals.var_qd_fb_dn7 = assign17720_e25308_d_n7;
        locals.var_qd_fb_dn10 = assign17720_e25308_d_n10;
        locals.var_qd_fb_dn11 = assign17720_e25308_d_n11;
        locals.var_qd_fb_dn12 = assign17720_e25308_d_n12;
        locals.var_qd_fb_dn13 = assign17720_e25308_d_n13;
        locals.var_qd_fb_dn15 = assign17720_e25308_d_n15;
        locals.var_qd_fb_dn16 = assign17720_e25308_d_n16;
        locals.var_qd_fb_dn17 = assign17720_e25308_d_n17;
        locals.var_qd_fb_dn18 = assign17720_e25308_d_n18;
        locals.var_qd_fb_rv = 0.0;

        let (assign17730_e25318, assign17730_e25318_d_n0, assign17730_e25318_d_n2, assign17730_e25318_d_n6, assign17730_e25318_d_n7, assign17730_e25318_d_n10, assign17730_e25318_d_n11, assign17730_e25318_d_n12, assign17730_e25318_d_n13, assign17730_e25318_d_n15, assign17730_e25318_d_n16, assign17730_e25318_d_n17, assign17730_e25318_d_n18,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard532 != 0.0)) {
        let assign17730_e25315: f64 = (1.0 - 0.5);
        let assign17730_e25316: f64 = (locals.var_qb * assign17730_e25315);
        (assign17730_e25316, (locals.var_qb_dn0 * assign17730_e25315), (locals.var_qb_dn2 * assign17730_e25315), (locals.var_qb_dn6 * assign17730_e25315), (locals.var_qb_dn7 * assign17730_e25315), (locals.var_qb_dn10 * assign17730_e25315), (locals.var_qb_dn11 * assign17730_e25315), (locals.var_qb_dn12 * assign17730_e25315), (locals.var_qb_dn13 * assign17730_e25315), (locals.var_qb_dn15 * assign17730_e25315), (locals.var_qb_dn16 * assign17730_e25315), (locals.var_qb_dn17 * assign17730_e25315), (locals.var_qb_dn18 * assign17730_e25315),)
    } else {
        (locals.var_qs_fb, locals.var_qs_fb_dn0, locals.var_qs_fb_dn2, locals.var_qs_fb_dn6, locals.var_qs_fb_dn7, locals.var_qs_fb_dn10, locals.var_qs_fb_dn11, locals.var_qs_fb_dn12, locals.var_qs_fb_dn13, locals.var_qs_fb_dn15, locals.var_qs_fb_dn16, locals.var_qs_fb_dn17, locals.var_qs_fb_dn18,)
    }
};
        locals.var_qs_fb = assign17730_e25318;
        locals.var_qs_fb_dn0 = assign17730_e25318_d_n0;
        locals.var_qs_fb_dn2 = assign17730_e25318_d_n2;
        locals.var_qs_fb_dn6 = assign17730_e25318_d_n6;
        locals.var_qs_fb_dn7 = assign17730_e25318_d_n7;
        locals.var_qs_fb_dn10 = assign17730_e25318_d_n10;
        locals.var_qs_fb_dn11 = assign17730_e25318_d_n11;
        locals.var_qs_fb_dn12 = assign17730_e25318_d_n12;
        locals.var_qs_fb_dn13 = assign17730_e25318_d_n13;
        locals.var_qs_fb_dn15 = assign17730_e25318_d_n15;
        locals.var_qs_fb_dn16 = assign17730_e25318_d_n16;
        locals.var_qs_fb_dn17 = assign17730_e25318_d_n17;
        locals.var_qs_fb_dn18 = assign17730_e25318_d_n18;
        locals.var_qs_fb_rv = 0.0;

        let (assign17740_e25332, assign17740_e25332_d_n0, assign17740_e25332_d_n2, assign17740_e25332_d_n6, assign17740_e25332_d_n7, assign17740_e25332_d_n10, assign17740_e25332_d_n11, assign17740_e25332_d_n12, assign17740_e25332_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard532 != 0.0)) {
        let assign17740_e25325: f64 = (locals.var_q_s0_bulk + locals.var_q_sl_bulk);
        let assign17740_e25326: f64 = (0.5 * assign17740_e25325);
        let assign17740_e25328: f64 = (assign17740_e25326 * locals.var_leff_cv);
        let assign17740_e25330: f64 = (assign17740_e25328 * locals.var_weffcv_nf);
        (assign17740_e25330, (((0.5 * (locals.var_q_s0_bulk_dn0 + locals.var_q_sl_bulk_dn0)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn2 + locals.var_q_sl_bulk_dn2)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn6 + locals.var_q_sl_bulk_dn6)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn7 + locals.var_q_sl_bulk_dn7)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn10 + locals.var_q_sl_bulk_dn10)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn11 + locals.var_q_sl_bulk_dn11)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn12 + locals.var_q_sl_bulk_dn12)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn17 + locals.var_q_sl_bulk_dn17)) * locals.var_leff_cv) * locals.var_weffcv_nf),)
    } else {
        (locals.var_qsub, locals.var_qsub_dn0, locals.var_qsub_dn2, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12, locals.var_qsub_dn17,)
    }
};
        locals.var_qsub = assign17740_e25332;
        locals.var_qsub_dn0 = assign17740_e25332_d_n0;
        locals.var_qsub_dn2 = assign17740_e25332_d_n2;
        locals.var_qsub_dn6 = assign17740_e25332_d_n6;
        locals.var_qsub_dn7 = assign17740_e25332_d_n7;
        locals.var_qsub_dn10 = assign17740_e25332_d_n10;
        locals.var_qsub_dn11 = assign17740_e25332_d_n11;
        locals.var_qsub_dn12 = assign17740_e25332_d_n12;
        locals.var_qsub_dn17 = assign17740_e25332_d_n17;
        locals.var_qsub_rv = 0.0;

    }
}
