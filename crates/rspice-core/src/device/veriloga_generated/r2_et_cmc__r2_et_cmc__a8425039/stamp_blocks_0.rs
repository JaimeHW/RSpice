#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_g0_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_guard54_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_guard57_slot: &mut f64,
        var_guard59_slot: &mut f64,
        var_l_um_slot: &mut f64,
        var_leff_um_slot: &mut f64,
        var_lfactor_slot: &mut f64,
        var_r0_slot: &mut f64,
        var_scalefac_slot: &mut f64,
        var_shrinkl_slot: &mut f64,
        var_tdevc_slot: &mut f64,
        var_tdevc_dn2_slot: &mut f64,
        var_tinik_slot: &mut f64,
        var_w_um_slot: &mut f64,
        var_weff_um_slot: &mut f64,
        var_xleff_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_g0: f64 = *var_g0_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_guard54: f64 = *var_guard54_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard57: f64 = *var_guard57_slot;
        let mut var_guard59: f64 = *var_guard59_slot;
        let mut var_l_um: f64 = *var_l_um_slot;
        let mut var_leff_um: f64 = *var_leff_um_slot;
        let mut var_lfactor: f64 = *var_lfactor_slot;
        let mut var_r0: f64 = *var_r0_slot;
        let mut var_scalefac: f64 = *var_scalefac_slot;
        let mut var_shrinkl: f64 = *var_shrinkl_slot;
        let mut var_tdevc: f64 = *var_tdevc_slot;
        let mut var_tdevc_dn2: f64 = *var_tdevc_dn2_slot;
        let mut var_tinik: f64 = *var_tinik_slot;
        let mut var_w_um: f64 = *var_w_um_slot;
        let mut var_weff_um: f64 = *var_weff_um_slot;
        let mut var_xleff: f64 = *var_xleff_slot;

        let assign10_e84: f64 = if param_given[10] { 1.0 } else { 0.0 };
        var_guard41 = assign10_e84;

        let (assign20_e88,) = {
    if (var_guard41 != 0.0) {
        (p.p10,)
    } else {
        (var_scalefac,)
    }
};
        var_scalefac = assign20_e88;

        let (assign30_e95,) = {
    if (var_guard41 == 0.0) {
        let assign30_e93: f64 = 1.0;
        (assign30_e93,)
    } else {
        (var_scalefac,)
    }
};
        var_scalefac = assign30_e95;

        let assign40_e97: f64 = if param_given[11] { 1.0 } else { 0.0 };
        var_guard42 = assign40_e97;

        let (assign50_e105,) = {
    if (var_guard42 != 0.0) {
        let assign50_e102: f64 = (0.01 * p.p11);
        let assign50_e103: f64 = (1.0 - assign50_e102);
        (assign50_e103,)
    } else {
        (var_shrinkl,)
    }
};
        var_shrinkl = assign50_e105;

        let (assign60_e116,) = {
    if (var_guard42 == 0.0) {
        let assign60_e112: f64 = 0.0;
        let assign60_e113: f64 = (0.01 * assign60_e112);
        let assign60_e114: f64 = (1.0 - assign60_e113);
        (assign60_e114,)
    } else {
        (var_shrinkl,)
    }
};
        var_shrinkl = assign60_e116;

        let assign100_e132: f64 = (var_shrinkl * var_scalefac);
        let assign100_e134: f64 = (assign100_e132 * 1000000.0);
        var_lfactor = assign100_e134;

        let assign110_e137: f64 = (273.15 + p.p16);
        var_tinik = assign110_e137;

        let assign120_e138: f64 = ctx_temp;
        let assign120_e140: f64 = (assign120_e138 + p.p5);
        let assign120_e142: f64 = (assign120_e140 - 273.15);
        var_tdevc = assign120_e142;
        var_tdevc_dn2 = 0.0;

        let assign150_e151: f64 = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        var_guard46 = assign150_e151;

        let (assign160_e155,) = {
    if (var_guard46 != 0.0) {
        (p.p23,)
    } else {
        (var_xleff,)
    }
};
        var_xleff = assign160_e155;

        let assign170_e158: f64 = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        var_guard47 = assign170_e158;

        let (assign180_e167,) = {
    if ((var_guard46 == 0.0) && (var_guard47 != 0.0)) {
        let assign180_e165: f64 = (p.p23 * 0.5);
        (assign180_e165,)
    } else {
        (var_xleff,)
    }
};
        var_xleff = assign180_e167;

        let (assign190_e175,) = {
    if ((var_guard46 == 0.0) && (var_guard47 == 0.0)) {
        (0.0,)
    } else {
        (var_xleff,)
    }
};
        var_xleff = assign190_e175;

        let assign200_e184: f64 = if ((param_given[1] && param_given[2]) && (!param_given[0])) { 1.0 } else { 0.0 };
        var_guard48 = assign200_e184;

        let assign210_e191: f64 = if ((p.p2 == 0.0) || (p.p1 == 0.0)) { 1.0 } else { 0.0 };
        var_guard49 = assign210_e191;

        let (assign220_e197,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign220_e197;

        let (assign230_e203,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign230_e203;

        let (assign240_e211,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        let assign240_e209: f64 = (p.p0 * var_lfactor);
        (assign240_e209,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign240_e211;

        let (assign250_e219,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        let assign250_e217: f64 = (var_w_um + p.p22);
        (assign250_e217,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign250_e219;

        let (assign260_e225,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign260_e225;

        let (assign270_e231,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        (1e99,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign270_e231;

        let (assign280_e240,) = {
    if ((var_guard48 != 0.0) && (var_guard49 == 0.0)) {
        let assign280_e238: f64 = (p.p1 * var_lfactor);
        (assign280_e238,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign280_e240;

        let (assign290_e249,) = {
    if ((var_guard48 != 0.0) && (var_guard49 == 0.0)) {
        let assign290_e247: f64 = (var_l_um + var_xleff);
        (assign290_e247,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign290_e249;

        let assign310_e255: f64 = if var_leff_um > 0.0 { 1.0 } else { 0.0 };
        var_guard51 = assign310_e255;

        let (assign320_e268,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 != 0.0)) {
        let assign320_e264: f64 = (p.p17 / p.p2);
        let assign320_e266: f64 = (assign320_e264 * var_leff_um);
        (assign320_e266,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign320_e268;

        let (assign330_e279,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 != 0.0)) {
        let assign330_e277: f64 = (var_weff_um - p.p22);
        (assign330_e277,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign330_e279;

        let (assign350_e291,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 != 0.0)) {
        (p.p2,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign350_e291;

        let (assign360_e302,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 != 0.0)) {
        let assign360_e300: f64 = (1.0 / var_r0);
        (assign360_e300,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign360_e302;

        let (assign370_e314,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 == 0.0)) {
        let assign370_e312: f64 = (p.p0 * var_lfactor);
        (assign370_e312,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign370_e314;

        let (assign380_e326,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 == 0.0)) {
        let assign380_e324: f64 = (var_w_um + p.p22);
        (assign380_e324,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign380_e326;

        let (assign390_e336,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 == 0.0)) {
        (0.0,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign390_e336;

        let (assign400_e346,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 == 0.0)) {
        (1e99,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign400_e346;

        let assign410_e352: f64 = if (param_given[2] && (!param_given[1])) { 1.0 } else { 0.0 };
        var_guard53 = assign410_e352;

        let assign420_e355: f64 = if p.p2 == 0.0 { 1.0 } else { 0.0 };
        var_guard54 = assign420_e355;

        let (assign430_e364,) = {
    if (((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 != 0.0)) {
        (0.0,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign430_e364;

        let (assign440_e373,) = {
    if (((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 != 0.0)) {
        (0.0,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign440_e373;

        let (assign450_e384,) = {
    if (((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 != 0.0)) {
        let assign450_e382: f64 = (p.p0 * var_lfactor);
        (assign450_e382,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign450_e384;

        let (assign460_e395,) = {
    if (((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 != 0.0)) {
        let assign460_e393: f64 = (var_w_um + p.p22);
        (assign460_e393,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign460_e395;

        let (assign470_e404,) = {
    if (((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 != 0.0)) {
        (0.0,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign470_e404;

        let (assign480_e413,) = {
    if (((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 != 0.0)) {
        (1e99,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign480_e413;

        let assign490_e416: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        var_guard55 = assign490_e416;

        let (assign500_e428,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        (0.0,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign500_e428;

        let (assign510_e440,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        (0.0,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign510_e440;

        let (assign520_e454,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        let assign520_e452: f64 = (p.p1 * var_lfactor);
        (assign520_e452,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign520_e454;

        let (assign530_e468,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        let assign530_e466: f64 = (var_l_um + var_xleff);
        (assign530_e466,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign530_e468;

        let (assign540_e480,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        (1e99,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign540_e480;

        let (assign550_e492,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        (0.0,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign550_e492;

        let (assign560_e507,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) {
        let assign560_e505: f64 = (p.p0 * var_lfactor);
        (assign560_e505,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign560_e507;

        let (assign570_e522,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) {
        let assign570_e520: f64 = (var_w_um + p.p22);
        (assign570_e520,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign570_e522;

        let assign590_e528: f64 = if var_weff_um > 0.0 { 1.0 } else { 0.0 };
        var_guard57 = assign590_e528;

        let (assign600_e547,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 != 0.0)) {
        let assign600_e543: f64 = (p.p2 / p.p17);
        let assign600_e545: f64 = (assign600_e543 * var_weff_um);
        (assign600_e545,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign600_e547;

        let (assign610_e564,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 != 0.0)) {
        let assign610_e562: f64 = (var_leff_um - var_xleff);
        (assign610_e562,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign610_e564;

        let (assign630_e582,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 != 0.0)) {
        (p.p2,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign630_e582;

        let (assign640_e599,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 != 0.0)) {
        let assign640_e597: f64 = (1.0 / var_r0);
        (assign640_e597,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign640_e599;

        let (assign650_e617,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 == 0.0)) {
        let assign650_e615: f64 = (p.p1 * var_lfactor);
        (assign650_e615,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign650_e617;

        let (assign660_e635,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 == 0.0)) {
        let assign660_e633: f64 = (var_l_um + var_xleff);
        (assign660_e633,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign660_e635;

        let (assign670_e651,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 == 0.0)) {
        (1e99,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign670_e651;

        let (assign680_e667,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 == 0.0)) {
        (0.0,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign680_e667;

        let assign690_e670: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        var_guard59 = assign690_e670;

        let (assign700_e680,) = {
    if (((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 != 0.0)) {
        (0.0,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign700_e680;

        let (assign710_e690,) = {
    if (((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 != 0.0)) {
        (0.0,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign710_e690;

        let (assign720_e702,) = {
    if (((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 != 0.0)) {
        let assign720_e700: f64 = (p.p1 * var_lfactor);
        (assign720_e700,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign720_e702;

        *var_g0_slot = var_g0;
        *var_guard41_slot = var_guard41;
        *var_guard42_slot = var_guard42;
        *var_guard46_slot = var_guard46;
        *var_guard47_slot = var_guard47;
        *var_guard48_slot = var_guard48;
        *var_guard49_slot = var_guard49;
        *var_guard51_slot = var_guard51;
        *var_guard53_slot = var_guard53;
        *var_guard54_slot = var_guard54;
        *var_guard55_slot = var_guard55;
        *var_guard57_slot = var_guard57;
        *var_guard59_slot = var_guard59;
        *var_l_um_slot = var_l_um;
        *var_leff_um_slot = var_leff_um;
        *var_lfactor_slot = var_lfactor;
        *var_r0_slot = var_r0;
        *var_scalefac_slot = var_scalefac;
        *var_shrinkl_slot = var_shrinkl;
        *var_tdevc_slot = var_tdevc;
        *var_tdevc_dn2_slot = var_tdevc_dn2;
        *var_tinik_slot = var_tinik;
        *var_w_um_slot = var_w_um;
        *var_weff_um_slot = var_weff_um;
        *var_xleff_slot = var_xleff;
    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard48: f64,
        var_guard53: f64,
        var_guard59: f64,
        var_lfactor: f64,
        var_tinik: f64,
        var_xleff: f64,
        var_a_um2_slot: &mut f64,
        var_cth_slot: &mut f64,
        var_delt_slot: &mut f64,
        var_delt_dn2_slot: &mut f64,
        var_e_slot: &mut f64,
        var_e_dn0_slot: &mut f64,
        var_e_dn1_slot: &mut f64,
        var_g0_slot: &mut f64,
        var_gth_slot: &mut f64,
        var_guard60_slot: &mut f64,
        var_guard62_slot: &mut f64,
        var_guard64_slot: &mut f64,
        var_guard70_slot: &mut f64,
        var_guard71_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_guard75_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard78_slot: &mut f64,
        var_guard79_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_l_um_slot: &mut f64,
        var_l_umfore_slot: &mut f64,
        var_leff_um_slot: &mut f64,
        var_p_um_slot: &mut f64,
        var_q2e_slot: &mut f64,
        var_q2e_dn0_slot: &mut f64,
        var_q2e_dn1_slot: &mut f64,
        var_q3e_slot: &mut f64,
        var_q3e_dn0_slot: &mut f64,
        var_q3e_dn1_slot: &mut f64,
        var_r0_slot: &mut f64,
        var_r0_t_slot: &mut f64,
        var_r0_t_dn2_slot: &mut f64,
        var_sqrf_slot: &mut f64,
        var_sqrf_dn0_slot: &mut f64,
        var_sqrf_dn1_slot: &mut f64,
        var_tc1e_slot: &mut f64,
        var_tc2e_slot: &mut f64,
        var_tcr_slot: &mut f64,
        var_tcr_dn2_slot: &mut f64,
        var_tdevc_slot: &mut f64,
        var_tdevc_dn2_slot: &mut f64,
        var_tdevk_slot: &mut f64,
        var_tdevk_dn2_slot: &mut f64,
        var_vin_slot: &mut f64,
        var_vin_dn0_slot: &mut f64,
        var_vin_dn1_slot: &mut f64,
        var_vrth_slot: &mut f64,
        var_vrth_dn2_slot: &mut f64,
        var_w_um_slot: &mut f64,
        var_weff_um_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let mut var_a_um2: f64 = *var_a_um2_slot;
        let mut var_cth: f64 = *var_cth_slot;
        let mut var_delt: f64 = *var_delt_slot;
        let mut var_delt_dn2: f64 = *var_delt_dn2_slot;
        let mut var_e: f64 = *var_e_slot;
        let mut var_e_dn0: f64 = *var_e_dn0_slot;
        let mut var_e_dn1: f64 = *var_e_dn1_slot;
        let mut var_g0: f64 = *var_g0_slot;
        let mut var_gth: f64 = *var_gth_slot;
        let mut var_guard60: f64 = *var_guard60_slot;
        let mut var_guard62: f64 = *var_guard62_slot;
        let mut var_guard64: f64 = *var_guard64_slot;
        let mut var_guard70: f64 = *var_guard70_slot;
        let mut var_guard71: f64 = *var_guard71_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_guard75: f64 = *var_guard75_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_guard79: f64 = *var_guard79_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_l_um: f64 = *var_l_um_slot;
        let mut var_l_umfore: f64 = *var_l_umfore_slot;
        let mut var_leff_um: f64 = *var_leff_um_slot;
        let mut var_p_um: f64 = *var_p_um_slot;
        let mut var_q2e: f64 = *var_q2e_slot;
        let mut var_q2e_dn0: f64 = *var_q2e_dn0_slot;
        let mut var_q2e_dn1: f64 = *var_q2e_dn1_slot;
        let mut var_q3e: f64 = *var_q3e_slot;
        let mut var_q3e_dn0: f64 = *var_q3e_dn0_slot;
        let mut var_q3e_dn1: f64 = *var_q3e_dn1_slot;
        let mut var_r0: f64 = *var_r0_slot;
        let mut var_r0_t: f64 = *var_r0_t_slot;
        let mut var_r0_t_dn2: f64 = *var_r0_t_dn2_slot;
        let mut var_sqrf: f64 = *var_sqrf_slot;
        let mut var_sqrf_dn0: f64 = *var_sqrf_dn0_slot;
        let mut var_sqrf_dn1: f64 = *var_sqrf_dn1_slot;
        let mut var_tc1e: f64 = *var_tc1e_slot;
        let mut var_tc2e: f64 = *var_tc2e_slot;
        let mut var_tcr: f64 = *var_tcr_slot;
        let mut var_tcr_dn2: f64 = *var_tcr_dn2_slot;
        let mut var_tdevc: f64 = *var_tdevc_slot;
        let mut var_tdevc_dn2: f64 = *var_tdevc_dn2_slot;
        let mut var_tdevk: f64 = *var_tdevk_slot;
        let mut var_tdevk_dn2: f64 = *var_tdevk_dn2_slot;
        let mut var_vin: f64 = *var_vin_slot;
        let mut var_vin_dn0: f64 = *var_vin_dn0_slot;
        let mut var_vin_dn1: f64 = *var_vin_dn1_slot;
        let mut var_vrth: f64 = *var_vrth_slot;
        let mut var_vrth_dn2: f64 = *var_vrth_dn2_slot;
        let mut var_w_um: f64 = *var_w_um_slot;
        let mut var_weff_um: f64 = *var_weff_um_slot;

        let (assign730_e714,) = {
    if (((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 != 0.0)) {
        let assign730_e712: f64 = (var_l_um + var_xleff);
        (assign730_e712,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign730_e714;

        let (assign740_e724,) = {
    if (((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 != 0.0)) {
        (1e99,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign740_e724;

        let (assign750_e734,) = {
    if (((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 != 0.0)) {
        (0.0,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign750_e734;

        let assign760_e737: f64 = if p.p1 == 0.0 { 1.0 } else { 0.0 };
        var_guard60 = assign760_e737;

        let (assign770_e750,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 != 0.0)) {
        (0.0,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign770_e750;

        let (assign780_e763,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 != 0.0)) {
        (0.0,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign780_e763;

        let (assign790_e778,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 != 0.0)) {
        let assign790_e776: f64 = (p.p0 * var_lfactor);
        (assign790_e776,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign790_e778;

        let (assign800_e793,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 != 0.0)) {
        let assign800_e791: f64 = (var_w_um + p.p22);
        (assign800_e791,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign800_e793;

        let (assign810_e806,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 != 0.0)) {
        (0.0,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign810_e806;

        let (assign820_e819,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 != 0.0)) {
        (1e99,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign820_e819;

        let (assign830_e835,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) {
        let assign830_e833: f64 = (p.p0 * var_lfactor);
        (assign830_e833,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign830_e835;

        let (assign840_e851,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) {
        let assign840_e849: f64 = (var_w_um + p.p22);
        (assign840_e849,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign840_e851;

        let (assign860_e870,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) {
        let assign860_e868: f64 = (p.p1 * var_lfactor);
        (assign860_e868,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign860_e870;

        let (assign870_e886,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) {
        let assign870_e884: f64 = (var_l_um + var_xleff);
        (assign870_e884,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign870_e886;

        let assign880_e889: f64 = if var_weff_um > 0.0 { 1.0 } else { 0.0 };
        var_guard62 = assign880_e889;

        let assign900_e895: f64 = if var_leff_um > 0.0 { 1.0 } else { 0.0 };
        var_guard64 = assign900_e895;

        let (assign910_e917,) = {
    if ((((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) && (var_guard62 != 0.0)) && (var_guard64 != 0.0)) {
        let assign910_e914: f64 = (var_leff_um / var_weff_um);
        let assign910_e915: f64 = (p.p17 * assign910_e914);
        (assign910_e915,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign910_e917;

        let (assign920_e937,) = {
    if ((((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) && (var_guard62 != 0.0)) && (var_guard64 != 0.0)) {
        let assign920_e935: f64 = (1.0 / var_r0);
        (assign920_e935,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign920_e937;

        let (assign930_e956,) = {
    if ((((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) && (var_guard62 != 0.0)) && (var_guard64 == 0.0)) {
        (0.0,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign930_e956;

        let (assign940_e975,) = {
    if ((((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) && (var_guard62 != 0.0)) && (var_guard64 == 0.0)) {
        (1e99,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign940_e975;

        let (assign950_e992,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) && (var_guard62 == 0.0)) {
        (1e99,)
    } else {
        (var_r0,)
    }
};
        var_r0 = assign950_e992;

        let (assign960_e1009,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) && (var_guard62 == 0.0)) {
        (0.0,)
    } else {
        (var_g0,)
    }
};
        var_g0 = assign960_e1009;

        let (assign1010_e1027,) = {
    if (p.p25 != 0.0) {
        let assign1010_e1025: f64 = (var_leff_um + p.p24);
        (assign1010_e1025,)
    } else {
        (var_l_umfore,)
    }
};
        var_l_umfore = assign1010_e1027;

        let (assign1020_e1034,) = {
    if (p.p25 == 0.0) {
        let assign1020_e1032: f64 = (var_l_um + p.p24);
        (assign1020_e1032,)
    } else {
        (var_l_umfore,)
    }
};
        var_l_umfore = assign1020_e1034;

        var_tc1e = p.p37;

        var_tc2e = p.p38;

        let assign1060_e1054: f64 = if var_leff_um > 0.0 { 1.0 } else { 0.0 };
        var_guard70 = assign1060_e1054;

        let assign1070_e1057: f64 = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        var_guard71 = assign1070_e1057;

        let (assign1080_e1067,) = {
    if ((var_guard70 != 0.0) && (var_guard71 != 0.0)) {
        let assign1080_e1064: f64 = (p.p39 / var_leff_um);
        let assign1080_e1065: f64 = (var_tc1e + assign1080_e1064);
        (assign1080_e1065,)
    } else {
        (var_tc1e,)
    }
};
        var_tc1e = assign1080_e1067;

        let (assign1090_e1077,) = {
    if ((var_guard70 != 0.0) && (var_guard71 != 0.0)) {
        let assign1090_e1074: f64 = (p.p40 / var_leff_um);
        let assign1090_e1075: f64 = (var_tc2e + assign1090_e1074);
        (assign1090_e1075,)
    } else {
        (var_tc2e,)
    }
};
        var_tc2e = assign1090_e1077;

        let assign1100_e1080: f64 = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        var_guard72 = assign1100_e1080;

        let (assign1110_e1095,) = {
    if (((var_guard70 != 0.0) && (var_guard71 == 0.0)) && (var_guard72 != 0.0)) {
        let assign1110_e1090: f64 = (0.5 * p.p39);
        let assign1110_e1092: f64 = (assign1110_e1090 / var_leff_um);
        let assign1110_e1093: f64 = (var_tc1e + assign1110_e1092);
        (assign1110_e1093,)
    } else {
        (var_tc1e,)
    }
};
        var_tc1e = assign1110_e1095;

        let (assign1120_e1110,) = {
    if (((var_guard70 != 0.0) && (var_guard71 == 0.0)) && (var_guard72 != 0.0)) {
        let assign1120_e1105: f64 = (0.5 * p.p40);
        let assign1120_e1107: f64 = (assign1120_e1105 / var_leff_um);
        let assign1120_e1108: f64 = (var_tc2e + assign1120_e1107);
        (assign1120_e1108,)
    } else {
        (var_tc2e,)
    }
};
        var_tc2e = assign1120_e1110;

        let assign1130_e1113: f64 = if var_weff_um > 0.0 { 1.0 } else { 0.0 };
        var_guard73 = assign1130_e1113;

        let (assign1140_e1121,) = {
    if (var_guard73 != 0.0) {
        let assign1140_e1118: f64 = (p.p41 / var_weff_um);
        let assign1140_e1119: f64 = (var_tc1e + assign1140_e1118);
        (assign1140_e1119,)
    } else {
        (var_tc1e,)
    }
};
        var_tc1e = assign1140_e1121;

        let (assign1150_e1129,) = {
    if (var_guard73 != 0.0) {
        let assign1150_e1126: f64 = (p.p42 / var_weff_um);
        let assign1150_e1127: f64 = (var_tc2e + assign1150_e1126);
        (assign1150_e1127,)
    } else {
        (var_tc2e,)
    }
};
        var_tc2e = assign1150_e1129;

        let assign1190_e1146: f64 = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        var_guard75 = assign1190_e1146;

        let (assign1200_e1154,) = {
    if (var_guard75 != 0.0) {
        let assign1200_e1151: f64 = (var_l_um + var_w_um);
        let assign1200_e1152: f64 = (2.0 * assign1200_e1151);
        (assign1200_e1152,)
    } else {
        (var_p_um,)
    }
};
        var_p_um = assign1200_e1154;

        let assign1210_e1157: f64 = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        var_guard76 = assign1210_e1157;

        let (assign1220_e1168,) = {
    if ((var_guard75 == 0.0) && (var_guard76 != 0.0)) {
        let assign1220_e1164: f64 = (2.0 * var_l_um);
        let assign1220_e1166: f64 = (assign1220_e1164 + var_w_um);
        (assign1220_e1166,)
    } else {
        (var_p_um,)
    }
};
        var_p_um = assign1220_e1168;

        let (assign1230_e1178,) = {
    if ((var_guard75 == 0.0) && (var_guard76 == 0.0)) {
        let assign1230_e1176: f64 = (2.0 * var_l_um);
        (assign1230_e1176,)
    } else {
        (var_p_um,)
    }
};
        var_p_um = assign1230_e1178;

        let assign1240_e1181: f64 = (var_l_um * var_w_um);
        var_a_um2 = assign1240_e1181;

        let assign1250_e1185: f64 = (p.p45 * var_p_um);
        let assign1250_e1186: f64 = (p.p44 + assign1250_e1185);
        let assign1250_e1189: f64 = (p.p46 * var_a_um2);
        let assign1250_e1190: f64 = (assign1250_e1186 + assign1250_e1189);
        var_gth = assign1250_e1190;

        let assign1260_e1194: f64 = (p.p48 * var_p_um);
        let assign1260_e1195: f64 = (p.p47 + assign1260_e1194);
        let assign1260_e1198: f64 = (p.p49 * var_a_um2);
        let assign1260_e1199: f64 = (assign1260_e1195 + assign1260_e1198);
        var_cth = assign1260_e1199;

        var_vrth = (nv2 - 0.0);
        var_vrth_dn2 = 1.0;

        let assign1290_e1206: f64 = (p.p7 * var_vrth);
        let assign1290_e1207: f64 = (var_tdevc + assign1290_e1206);
        var_tdevc = assign1290_e1207;
        var_tdevc_dn2 = (var_tdevc_dn2 + (p.p7 * var_vrth_dn2));

        let assign1300_e1211: f64 = (p.p35 + 1.0);
        let assign1300_e1212: f64 = if var_tdevc < assign1300_e1211 { 1.0 } else { 0.0 };
        var_guard78 = assign1300_e1212;

        let (assign1310_e1223, assign1310_e1223_d_n2,) = {
    if (var_guard78 != 0.0) {
        let assign1310_e1217: f64 = (var_tdevc - p.p35);
        let assign1310_e1219: f64 = (assign1310_e1217 - 1.0);
        let assign1310_e1220: f64 = (assign1310_e1219).exp();
        let assign1310_e1221: f64 = (p.p35 + assign1310_e1220);
        (assign1310_e1221, (assign1310_e1220 * var_tdevc_dn2),)
    } else {
        (var_tdevc, var_tdevc_dn2,)
    }
};
        var_tdevc = assign1310_e1223;
        var_tdevc_dn2 = assign1310_e1223_d_n2;

        let assign1320_e1227: f64 = (p.p36 - 1.0);
        let assign1320_e1228: f64 = if var_tdevc > assign1320_e1227 { 1.0 } else { 0.0 };
        var_guard79 = assign1320_e1228;

        let (assign1330_e1242, assign1330_e1242_d_n2,) = {
    if ((var_guard78 == 0.0) && (var_guard79 != 0.0)) {
        let assign1330_e1236: f64 = (p.p36 - var_tdevc);
        let assign1330_e1238: f64 = (assign1330_e1236 - 1.0);
        let assign1330_e1239: f64 = (assign1330_e1238).exp();
        let assign1330_e1240: f64 = (p.p36 - assign1330_e1239);
        (assign1330_e1240, (-(assign1330_e1239 * (-var_tdevc_dn2))),)
    } else {
        (var_tdevc, var_tdevc_dn2,)
    }
};
        var_tdevc = assign1330_e1242;
        var_tdevc_dn2 = assign1330_e1242_d_n2;

        let (assign1340_e1250, assign1340_e1250_d_n2,) = {
    if ((var_guard78 == 0.0) && (var_guard79 == 0.0)) {
        (var_tdevc, var_tdevc_dn2,)
    } else {
        (var_tdevc, var_tdevc_dn2,)
    }
};
        var_tdevc = assign1340_e1250;
        var_tdevc_dn2 = assign1340_e1250_d_n2;

        let assign1350_e1253: f64 = (var_tdevc + 273.15);
        var_tdevk = assign1350_e1253;
        var_tdevk_dn2 = var_tdevc_dn2;

        let assign1360_e1256: f64 = (var_tdevk - var_tinik);
        var_delt = assign1360_e1256;
        var_delt_dn2 = var_tdevk_dn2;

        let assign1370_e1262: f64 = (var_delt * var_tc2e);
        let assign1370_e1263: f64 = (var_tc1e + assign1370_e1262);
        let assign1370_e1264: f64 = (var_delt * assign1370_e1263);
        let assign1370_e1265: f64 = (1.0 + assign1370_e1264);
        var_tcr = assign1370_e1265;
        var_tcr_dn2 = ((var_delt_dn2 * assign1370_e1263) + (var_delt * (var_delt_dn2 * var_tc2e)));

        let assign1380_e1269: f64 = (0.01 + 0.1);
        let assign1380_e1270: f64 = if var_tcr < assign1380_e1269 { 1.0 } else { 0.0 };
        var_guard80 = assign1380_e1270;

        let (assign1390_e1285, assign1390_e1285_d_n2,) = {
    if (var_guard80 != 0.0) {
        let assign1390_e1277: f64 = (var_tcr - 0.01);
        let assign1390_e1278: f64 = (10.0 * assign1390_e1277);
        let assign1390_e1280: f64 = (assign1390_e1278 - 1.0);
        let assign1390_e1281: f64 = (assign1390_e1280).exp();
        let assign1390_e1282: f64 = (0.1 * assign1390_e1281);
        let assign1390_e1283: f64 = (0.01 + assign1390_e1282);
        (assign1390_e1283, (0.1 * (assign1390_e1281 * (10.0 * var_tcr_dn2))),)
    } else {
        (var_tcr, var_tcr_dn2,)
    }
};
        var_tcr = assign1390_e1285;
        var_tcr_dn2 = assign1390_e1285_d_n2;

        let (assign1400_e1290, assign1400_e1290_d_n2,) = {
    if (var_guard80 == 0.0) {
        (var_tcr, var_tcr_dn2,)
    } else {
        (var_tcr, var_tcr_dn2,)
    }
};
        var_tcr = assign1400_e1290;
        var_tcr_dn2 = assign1400_e1290_d_n2;

        let assign1410_e1293: f64 = (var_r0 * var_tcr);
        var_r0_t = assign1410_e1293;
        var_r0_t_dn2 = (var_r0 * var_tcr_dn2);

        var_vin = (nv0 - nv1);
        var_vin_dn0 = 1.0;
        var_vin_dn1 = -1.0;

        let assign1470_e1322: f64 = if ((var_r0 > 0.0) && ((p.p29 > 0.0) || (p.p27 > 0.0))) { 1.0 } else { 0.0 };
        var_guard82 = assign1470_e1322;

        let (assign1480_e1328, assign1480_e1328_d_n0, assign1480_e1328_d_n1,) = {
    if (var_guard82 != 0.0) {
        let assign1480_e1326: f64 = (var_vin / var_l_umfore);
        (assign1480_e1326, (var_vin_dn0 / var_l_umfore), (var_vin_dn1 / var_l_umfore),)
    } else {
        (var_e, var_e_dn0, var_e_dn1,)
    }
};
        var_e = assign1480_e1328;
        var_e_dn0 = assign1480_e1328_d_n0;
        var_e_dn1 = assign1480_e1328_d_n1;

        let (assign1490_e1334, assign1490_e1334_d_n0, assign1490_e1334_d_n1,) = {
    if (var_guard82 != 0.0) {
        let assign1490_e1332: f64 = (p.p28 * var_e);
        (assign1490_e1332, (p.p28 * var_e_dn0), (p.p28 * var_e_dn1),)
    } else {
        (var_q2e, var_q2e_dn0, var_q2e_dn1,)
    }
};
        var_q2e = assign1490_e1334;
        var_q2e_dn0 = assign1490_e1334_d_n0;
        var_q2e_dn1 = assign1490_e1334_d_n1;

        let (assign1500_e1343, assign1500_e1343_d_n0, assign1500_e1343_d_n1,) = {
    if (var_guard82 != 0.0) {
        let assign1500_e1339: f64 = (var_q2e * var_q2e);
        let assign1500_e1340: f64 = (1.0 + assign1500_e1339);
        let assign1500_e1341: f64 = (assign1500_e1340).sqrt();
        (assign1500_e1341, (((var_q2e_dn0 * var_q2e) + (var_q2e * var_q2e_dn0)) / (2.0 * assign1500_e1341)), (((var_q2e_dn1 * var_q2e) + (var_q2e * var_q2e_dn1)) / (2.0 * assign1500_e1341)),)
    } else {
        (var_sqrf, var_sqrf_dn0, var_sqrf_dn1,)
    }
};
        var_sqrf = assign1500_e1343;
        var_sqrf_dn0 = assign1500_e1343_d_n0;
        var_sqrf_dn1 = assign1500_e1343_d_n1;

        let (assign1510_e1350, assign1510_e1350_d_n0, assign1510_e1350_d_n1,) = {
    if (var_guard82 != 0.0) {
        let assign1510_e1347: f64 = (var_e).abs();
        let assign1510_e1348: f64 = (p.p26 * assign1510_e1347);
        (assign1510_e1348, (p.p26 * if var_e >= 0.0 { var_e_dn0 } else { (-var_e_dn0) }), (p.p26 * if var_e >= 0.0 { var_e_dn1 } else { (-var_e_dn1) }),)
    } else {
        (var_q3e, var_q3e_dn0, var_q3e_dn1,)
    }
};
        var_q3e = assign1510_e1350;
        var_q3e_dn0 = assign1510_e1350_d_n0;
        var_q3e_dn1 = assign1510_e1350_d_n1;

        *var_a_um2_slot = var_a_um2;
        *var_cth_slot = var_cth;
        *var_delt_slot = var_delt;
        *var_delt_dn2_slot = var_delt_dn2;
        *var_e_slot = var_e;
        *var_e_dn0_slot = var_e_dn0;
        *var_e_dn1_slot = var_e_dn1;
        *var_g0_slot = var_g0;
        *var_gth_slot = var_gth;
        *var_guard60_slot = var_guard60;
        *var_guard62_slot = var_guard62;
        *var_guard64_slot = var_guard64;
        *var_guard70_slot = var_guard70;
        *var_guard71_slot = var_guard71;
        *var_guard72_slot = var_guard72;
        *var_guard73_slot = var_guard73;
        *var_guard75_slot = var_guard75;
        *var_guard76_slot = var_guard76;
        *var_guard78_slot = var_guard78;
        *var_guard79_slot = var_guard79;
        *var_guard80_slot = var_guard80;
        *var_guard82_slot = var_guard82;
        *var_l_um_slot = var_l_um;
        *var_l_umfore_slot = var_l_umfore;
        *var_leff_um_slot = var_leff_um;
        *var_p_um_slot = var_p_um;
        *var_q2e_slot = var_q2e;
        *var_q2e_dn0_slot = var_q2e_dn0;
        *var_q2e_dn1_slot = var_q2e_dn1;
        *var_q3e_slot = var_q3e;
        *var_q3e_dn0_slot = var_q3e_dn0;
        *var_q3e_dn1_slot = var_q3e_dn1;
        *var_r0_slot = var_r0;
        *var_r0_t_slot = var_r0_t;
        *var_r0_t_dn2_slot = var_r0_t_dn2;
        *var_sqrf_slot = var_sqrf;
        *var_sqrf_dn0_slot = var_sqrf_dn0;
        *var_sqrf_dn1_slot = var_sqrf_dn1;
        *var_tc1e_slot = var_tc1e;
        *var_tc2e_slot = var_tc2e;
        *var_tcr_slot = var_tcr;
        *var_tcr_dn2_slot = var_tcr_dn2;
        *var_tdevc_slot = var_tdevc;
        *var_tdevc_dn2_slot = var_tdevc_dn2;
        *var_tdevk_slot = var_tdevk;
        *var_tdevk_dn2_slot = var_tdevk_dn2;
        *var_vin_slot = var_vin;
        *var_vin_dn0_slot = var_vin_dn0;
        *var_vin_dn1_slot = var_vin_dn1;
        *var_vrth_slot = var_vrth;
        *var_vrth_dn2_slot = var_vrth_dn2;
        *var_w_um_slot = var_w_um;
        *var_weff_um_slot = var_weff_um;
    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        var_cth: f64,
        var_g0: f64,
        var_gth: f64,
        var_guard82: f64,
        var_q3e: f64,
        var_q3e_dn0: f64,
        var_q3e_dn1: f64,
        var_r0: f64,
        var_r0_t: f64,
        var_r0_t_dn2: f64,
        var_sqrf: f64,
        var_sqrf_dn0: f64,
        var_sqrf_dn1: f64,
        var_vin: f64,
        var_vin_dn0: f64,
        var_vin_dn1: f64,
        var_vrth: f64,
        var_vrth_dn2: f64,
        var_cbrf_slot: &mut f64,
        var_cbrf_dn0_slot: &mut f64,
        var_cbrf_dn1_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_i_slot: &mut f64,
        var_i_dn0_slot: &mut f64,
        var_i_dn1_slot: &mut f64,
        var_i_dn2_slot: &mut f64,
        var_irth_slot: &mut f64,
        var_irth_dn2_slot: &mut f64,
        var_ith_slot: &mut f64,
        var_ith_dn0_slot: &mut f64,
        var_ith_dn1_slot: &mut f64,
        var_ith_dn2_slot: &mut f64,
        var_qcth_slot: &mut f64,
        var_qcth_dn2_slot: &mut f64,
        var_r_dc_slot: &mut f64,
        var_r_dc_dn0_slot: &mut f64,
        var_r_dc_dn1_slot: &mut f64,
        var_r_dc_dn2_slot: &mut f64,
        var_rfactor_slot: &mut f64,
        var_rfactor_dn0_slot: &mut f64,
        var_rfactor_dn1_slot: &mut f64,
        var_v_slot: &mut f64,
        var_v_dn0_slot: &mut f64,
        var_v_dn1_slot: &mut f64,
    ) {
        let mut var_cbrf: f64 = *var_cbrf_slot;
        let mut var_cbrf_dn0: f64 = *var_cbrf_dn0_slot;
        let mut var_cbrf_dn1: f64 = *var_cbrf_dn1_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_i: f64 = *var_i_slot;
        let mut var_i_dn0: f64 = *var_i_dn0_slot;
        let mut var_i_dn1: f64 = *var_i_dn1_slot;
        let mut var_i_dn2: f64 = *var_i_dn2_slot;
        let mut var_irth: f64 = *var_irth_slot;
        let mut var_irth_dn2: f64 = *var_irth_dn2_slot;
        let mut var_ith: f64 = *var_ith_slot;
        let mut var_ith_dn0: f64 = *var_ith_dn0_slot;
        let mut var_ith_dn1: f64 = *var_ith_dn1_slot;
        let mut var_ith_dn2: f64 = *var_ith_dn2_slot;
        let mut var_qcth: f64 = *var_qcth_slot;
        let mut var_qcth_dn2: f64 = *var_qcth_dn2_slot;
        let mut var_r_dc: f64 = *var_r_dc_slot;
        let mut var_r_dc_dn0: f64 = *var_r_dc_dn0_slot;
        let mut var_r_dc_dn1: f64 = *var_r_dc_dn1_slot;
        let mut var_r_dc_dn2: f64 = *var_r_dc_dn2_slot;
        let mut var_rfactor: f64 = *var_rfactor_slot;
        let mut var_rfactor_dn0: f64 = *var_rfactor_dn0_slot;
        let mut var_rfactor_dn1: f64 = *var_rfactor_dn1_slot;
        let mut var_v: f64 = *var_v_slot;
        let mut var_v_dn0: f64 = *var_v_dn0_slot;
        let mut var_v_dn1: f64 = *var_v_dn1_slot;

        let (assign1520_e1362, assign1520_e1362_d_n0, assign1520_e1362_d_n1,) = {
    if (var_guard82 != 0.0) {
        let assign1520_e1355: f64 = (var_q3e * var_q3e);
        let assign1520_e1357: f64 = (assign1520_e1355 * var_q3e);
        let assign1520_e1358: f64 = (1.0 + assign1520_e1357);
        let assign1520_e1360: f64 = (assign1520_e1358).powf(0.3333333333333333);
        (assign1520_e1360, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign1520_e1358).powf(0.3333333333333333 - 1.0) * ((((var_q3e_dn0 * var_q3e) + (var_q3e * var_q3e_dn0)) * var_q3e) + (assign1520_e1355 * var_q3e_dn0)))) } } else { (assign1520_e1360 * (0.3333333333333333 * (((((var_q3e_dn0 * var_q3e) + (var_q3e * var_q3e_dn0)) * var_q3e) + (assign1520_e1355 * var_q3e_dn0)) / assign1520_e1358))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign1520_e1358).powf(0.3333333333333333 - 1.0) * ((((var_q3e_dn1 * var_q3e) + (var_q3e * var_q3e_dn1)) * var_q3e) + (assign1520_e1355 * var_q3e_dn1)))) } } else { (assign1520_e1360 * (0.3333333333333333 * (((((var_q3e_dn1 * var_q3e) + (var_q3e * var_q3e_dn1)) * var_q3e) + (assign1520_e1355 * var_q3e_dn1)) / assign1520_e1358))) },)
    } else {
        (var_cbrf, var_cbrf_dn0, var_cbrf_dn1,)
    }
};
        var_cbrf = assign1520_e1362;
        var_cbrf_dn0 = assign1520_e1362_d_n0;
        var_cbrf_dn1 = assign1520_e1362_d_n1;

        let (assign1530_e1378, assign1530_e1378_d_n0, assign1530_e1378_d_n1,) = {
    if (var_guard82 != 0.0) {
        let assign1530_e1366: f64 = (1.0 - p.p29);
        let assign1530_e1368: f64 = (assign1530_e1366 - p.p27);
        let assign1530_e1371: f64 = (p.p29 * var_sqrf);
        let assign1530_e1372: f64 = (assign1530_e1368 + assign1530_e1371);
        let assign1530_e1375: f64 = (p.p27 * var_cbrf);
        let assign1530_e1376: f64 = (assign1530_e1372 + assign1530_e1375);
        (assign1530_e1376, ((p.p29 * var_sqrf_dn0) + (p.p27 * var_cbrf_dn0)), ((p.p29 * var_sqrf_dn1) + (p.p27 * var_cbrf_dn1)),)
    } else {
        (var_rfactor, var_rfactor_dn0, var_rfactor_dn1,)
    }
};
        var_rfactor = assign1530_e1378;
        var_rfactor_dn0 = assign1530_e1378_d_n0;
        var_rfactor_dn1 = assign1530_e1378_d_n1;

        let (assign1540_e1383, assign1540_e1383_d_n0, assign1540_e1383_d_n1,) = {
    if (var_guard82 == 0.0) {
        (1.0, 0.0, 0.0,)
    } else {
        (var_rfactor, var_rfactor_dn0, var_rfactor_dn1,)
    }
};
        var_rfactor = assign1540_e1383;
        var_rfactor_dn0 = assign1540_e1383_d_n0;
        var_rfactor_dn1 = assign1540_e1383_d_n1;

        let assign1550_e1386: f64 = (var_r0_t * var_rfactor);
        var_r_dc = assign1550_e1386;
        var_r_dc_dn0 = (var_r0_t * var_rfactor_dn0);
        var_r_dc_dn1 = (var_r0_t * var_rfactor_dn1);
        var_r_dc_dn2 = (var_r0_t_dn2 * var_rfactor);

        var_v = var_vin;
        var_v_dn0 = var_vin_dn0;
        var_v_dn1 = var_vin_dn1;

        let assign1570_e1390: f64 = (var_v / var_r_dc);
        var_i = assign1570_e1390;
        var_i_dn0 = (((var_v_dn0 * var_r_dc) - (var_v * var_r_dc_dn0)) / (var_r_dc * var_r_dc));
        var_i_dn1 = (((var_v_dn1 * var_r_dc) - (var_v * var_r_dc_dn1)) / (var_r_dc * var_r_dc));
        var_i_dn2 = (-((var_v * var_r_dc_dn2) / (var_r_dc * var_r_dc)));

        let assign1580_e1392: f64 = (-var_v);
        let assign1580_e1394: f64 = (assign1580_e1392 * var_i);
        var_ith = assign1580_e1394;
        var_ith_dn0 = (((-var_v_dn0) * var_i) + (assign1580_e1392 * var_i_dn0));
        var_ith_dn1 = (((-var_v_dn1) * var_i) + (assign1580_e1392 * var_i_dn1));
        var_ith_dn2 = (assign1580_e1392 * var_i_dn2);

        let assign1590_e1397: f64 = (var_vrth * var_gth);
        var_irth = assign1590_e1397;
        var_irth_dn2 = (var_vrth_dn2 * var_gth);

        let assign1620_e1409: f64 = (var_vrth * var_cth);
        var_qcth = assign1620_e1409;
        var_qcth_dn2 = (var_vrth_dn2 * var_cth);

        let assign1750_e1523: f64 = if ((var_r0 > 0.0) && (var_g0 > 0.0)) { 1.0 } else { 0.0 };
        var_guard89 = assign1750_e1523;

        let (assign1760_e1529, assign1760_e1529_d_n0, assign1760_e1529_d_n1, assign1760_e1529_d_n2,) = {
    if (var_guard89 != 0.0) {
        let assign1760_e1527: f64 = (var_r0_t * var_rfactor);
        (assign1760_e1527, (var_r0_t * var_rfactor_dn0), (var_r0_t * var_rfactor_dn1), (var_r0_t_dn2 * var_rfactor),)
    } else {
        (var_r_dc, var_r_dc_dn0, var_r_dc_dn1, var_r_dc_dn2,)
    }
};
        var_r_dc = assign1760_e1529;
        var_r_dc_dn0 = assign1760_e1529_d_n0;
        var_r_dc_dn1 = assign1760_e1529_d_n1;
        var_r_dc_dn2 = assign1760_e1529_d_n2;

        let (assign1860_e1604, assign1860_e1604_d_n0, assign1860_e1604_d_n1, assign1860_e1604_d_n2,) = {
    if (var_guard89 == 0.0) {
        (var_r0, 0.0, 0.0, 0.0,)
    } else {
        (var_r_dc, var_r_dc_dn0, var_r_dc_dn1, var_r_dc_dn2,)
    }
};
        var_r_dc = assign1860_e1604;
        var_r_dc_dn0 = assign1860_e1604_d_n0;
        var_r_dc_dn1 = assign1860_e1604_d_n1;
        var_r_dc_dn2 = assign1860_e1604_d_n2;

        *var_cbrf_slot = var_cbrf;
        *var_cbrf_dn0_slot = var_cbrf_dn0;
        *var_cbrf_dn1_slot = var_cbrf_dn1;
        *var_guard89_slot = var_guard89;
        *var_i_slot = var_i;
        *var_i_dn0_slot = var_i_dn0;
        *var_i_dn1_slot = var_i_dn1;
        *var_i_dn2_slot = var_i_dn2;
        *var_irth_slot = var_irth;
        *var_irth_dn2_slot = var_irth_dn2;
        *var_ith_slot = var_ith;
        *var_ith_dn0_slot = var_ith_dn0;
        *var_ith_dn1_slot = var_ith_dn1;
        *var_ith_dn2_slot = var_ith_dn2;
        *var_qcth_slot = var_qcth;
        *var_qcth_dn2_slot = var_qcth_dn2;
        *var_r_dc_slot = var_r_dc;
        *var_r_dc_dn0_slot = var_r_dc_dn0;
        *var_r_dc_dn1_slot = var_r_dc_dn1;
        *var_r_dc_dn2_slot = var_r_dc_dn2;
        *var_rfactor_slot = var_rfactor;
        *var_rfactor_dn0_slot = var_rfactor_dn0;
        *var_rfactor_dn1_slot = var_rfactor_dn1;
        *var_v_slot = var_v;
        *var_v_dn0_slot = var_v_dn0;
        *var_v_dn1_slot = var_v_dn1;
    }

    pub(super) fn stamp_reactive_block_0(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard41_slot: &mut f64,
        var_guard41_rv_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard42_rv_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard46_rv_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard47_rv_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard48_rv_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_guard49_rv_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard51_rv_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_guard53_rv_slot: &mut f64,
        var_guard54_slot: &mut f64,
        var_guard54_rv_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_guard55_rv_slot: &mut f64,
        var_guard57_slot: &mut f64,
        var_guard57_rv_slot: &mut f64,
        var_guard59_slot: &mut f64,
        var_guard59_rv_slot: &mut f64,
        var_guard60_slot: &mut f64,
        var_guard60_rv_slot: &mut f64,
        var_l_um_slot: &mut f64,
        var_l_um_rv_slot: &mut f64,
        var_leff_um_slot: &mut f64,
        var_leff_um_rv_slot: &mut f64,
        var_lfactor_slot: &mut f64,
        var_lfactor_rv_slot: &mut f64,
        var_scalefac_slot: &mut f64,
        var_scalefac_rv_slot: &mut f64,
        var_shrinkl_slot: &mut f64,
        var_shrinkl_rv_slot: &mut f64,
        var_w_um_slot: &mut f64,
        var_w_um_rv_slot: &mut f64,
        var_weff_um_slot: &mut f64,
        var_weff_um_rv_slot: &mut f64,
        var_xleff_slot: &mut f64,
        var_xleff_rv_slot: &mut f64,
    ) {
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_guard41_rv: f64 = *var_guard41_rv_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard42_rv: f64 = *var_guard42_rv_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard46_rv: f64 = *var_guard46_rv_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard47_rv: f64 = *var_guard47_rv_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard48_rv: f64 = *var_guard48_rv_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_guard49_rv: f64 = *var_guard49_rv_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard51_rv: f64 = *var_guard51_rv_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_guard53_rv: f64 = *var_guard53_rv_slot;
        let mut var_guard54: f64 = *var_guard54_slot;
        let mut var_guard54_rv: f64 = *var_guard54_rv_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard55_rv: f64 = *var_guard55_rv_slot;
        let mut var_guard57: f64 = *var_guard57_slot;
        let mut var_guard57_rv: f64 = *var_guard57_rv_slot;
        let mut var_guard59: f64 = *var_guard59_slot;
        let mut var_guard59_rv: f64 = *var_guard59_rv_slot;
        let mut var_guard60: f64 = *var_guard60_slot;
        let mut var_guard60_rv: f64 = *var_guard60_rv_slot;
        let mut var_l_um: f64 = *var_l_um_slot;
        let mut var_l_um_rv: f64 = *var_l_um_rv_slot;
        let mut var_leff_um: f64 = *var_leff_um_slot;
        let mut var_leff_um_rv: f64 = *var_leff_um_rv_slot;
        let mut var_lfactor: f64 = *var_lfactor_slot;
        let mut var_lfactor_rv: f64 = *var_lfactor_rv_slot;
        let mut var_scalefac: f64 = *var_scalefac_slot;
        let mut var_scalefac_rv: f64 = *var_scalefac_rv_slot;
        let mut var_shrinkl: f64 = *var_shrinkl_slot;
        let mut var_shrinkl_rv: f64 = *var_shrinkl_rv_slot;
        let mut var_w_um: f64 = *var_w_um_slot;
        let mut var_w_um_rv: f64 = *var_w_um_rv_slot;
        let mut var_weff_um: f64 = *var_weff_um_slot;
        let mut var_weff_um_rv: f64 = *var_weff_um_rv_slot;
        let mut var_xleff: f64 = *var_xleff_slot;
        let mut var_xleff_rv: f64 = *var_xleff_rv_slot;

        let assign10_e84: f64 = if param_given[10] { 1.0 } else { 0.0 };
        var_guard41 = assign10_e84;
        var_guard41_rv = 0.0;

        let (assign20_e88,) = {
    if (var_guard41 != 0.0) {
        (p.p10,)
    } else {
        (var_scalefac,)
    }
};
        var_scalefac = assign20_e88;
        var_scalefac_rv = 0.0;

        let (assign30_e95,) = {
    if (var_guard41 == 0.0) {
        let assign30_e93: f64 = 1.0;
        (assign30_e93,)
    } else {
        (var_scalefac,)
    }
};
        var_scalefac = assign30_e95;
        var_scalefac_rv = 0.0;

        let assign40_e97: f64 = if param_given[11] { 1.0 } else { 0.0 };
        var_guard42 = assign40_e97;
        var_guard42_rv = 0.0;

        let (assign50_e105,) = {
    if (var_guard42 != 0.0) {
        let assign50_e102: f64 = (0.01 * p.p11);
        let assign50_e103: f64 = (1.0 - assign50_e102);
        (assign50_e103,)
    } else {
        (var_shrinkl,)
    }
};
        var_shrinkl = assign50_e105;
        var_shrinkl_rv = 0.0;

        let (assign60_e116,) = {
    if (var_guard42 == 0.0) {
        let assign60_e112: f64 = 0.0;
        let assign60_e113: f64 = (0.01 * assign60_e112);
        let assign60_e114: f64 = (1.0 - assign60_e113);
        (assign60_e114,)
    } else {
        (var_shrinkl,)
    }
};
        var_shrinkl = assign60_e116;
        var_shrinkl_rv = 0.0;

        let assign100_e132: f64 = (var_shrinkl * var_scalefac);
        let assign100_e134: f64 = (assign100_e132 * 1000000.0);
        var_lfactor = assign100_e134;
        var_lfactor_rv = 0.0;

        let assign150_e151: f64 = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        var_guard46 = assign150_e151;
        var_guard46_rv = 0.0;

        let (assign160_e155,) = {
    if (var_guard46 != 0.0) {
        (p.p23,)
    } else {
        (var_xleff,)
    }
};
        var_xleff = assign160_e155;
        var_xleff_rv = 0.0;

        let assign170_e158: f64 = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        var_guard47 = assign170_e158;
        var_guard47_rv = 0.0;

        let (assign180_e167,) = {
    if ((var_guard46 == 0.0) && (var_guard47 != 0.0)) {
        let assign180_e165: f64 = (p.p23 * 0.5);
        (assign180_e165,)
    } else {
        (var_xleff,)
    }
};
        var_xleff = assign180_e167;
        var_xleff_rv = 0.0;

        let (assign190_e175,) = {
    if ((var_guard46 == 0.0) && (var_guard47 == 0.0)) {
        (0.0,)
    } else {
        (var_xleff,)
    }
};
        var_xleff = assign190_e175;
        var_xleff_rv = 0.0;

        let assign200_e184: f64 = if ((param_given[1] && param_given[2]) && (!param_given[0])) { 1.0 } else { 0.0 };
        var_guard48 = assign200_e184;
        var_guard48_rv = 0.0;

        let assign210_e191: f64 = if ((p.p2 == 0.0) || (p.p1 == 0.0)) { 1.0 } else { 0.0 };
        var_guard49 = assign210_e191;
        var_guard49_rv = 0.0;

        let (assign220_e197,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign220_e197;
        var_l_um_rv = 0.0;

        let (assign230_e203,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign230_e203;
        var_leff_um_rv = 0.0;

        let (assign240_e211,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        let assign240_e209: f64 = (p.p0 * var_lfactor);
        (assign240_e209,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign240_e211;
        var_w_um_rv = 0.0;

        let (assign250_e219,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        let assign250_e217: f64 = (var_w_um + p.p22);
        (assign250_e217,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign250_e219;
        var_weff_um_rv = 0.0;

        let (assign280_e240,) = {
    if ((var_guard48 != 0.0) && (var_guard49 == 0.0)) {
        let assign280_e238: f64 = (p.p1 * var_lfactor);
        (assign280_e238,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign280_e240;
        var_l_um_rv = 0.0;

        let (assign290_e249,) = {
    if ((var_guard48 != 0.0) && (var_guard49 == 0.0)) {
        let assign290_e247: f64 = (var_l_um + var_xleff);
        (assign290_e247,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign290_e249;
        var_leff_um_rv = 0.0;

        let assign310_e255: f64 = if var_leff_um > 0.0 { 1.0 } else { 0.0 };
        var_guard51 = assign310_e255;
        var_guard51_rv = 0.0;

        let (assign320_e268,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 != 0.0)) {
        let assign320_e264: f64 = (p.p17 / p.p2);
        let assign320_e266: f64 = (assign320_e264 * var_leff_um);
        (assign320_e266,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign320_e268;
        var_weff_um_rv = 0.0;

        let (assign330_e279,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 != 0.0)) {
        let assign330_e277: f64 = (var_weff_um - p.p22);
        (assign330_e277,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign330_e279;
        var_w_um_rv = 0.0;

        let (assign370_e314,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 == 0.0)) {
        let assign370_e312: f64 = (p.p0 * var_lfactor);
        (assign370_e312,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign370_e314;
        var_w_um_rv = 0.0;

        let (assign380_e326,) = {
    if (((var_guard48 != 0.0) && (var_guard49 == 0.0)) && (var_guard51 == 0.0)) {
        let assign380_e324: f64 = (var_w_um + p.p22);
        (assign380_e324,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign380_e326;
        var_weff_um_rv = 0.0;

        let assign410_e352: f64 = if (param_given[2] && (!param_given[1])) { 1.0 } else { 0.0 };
        var_guard53 = assign410_e352;
        var_guard53_rv = 0.0;

        let assign420_e355: f64 = if p.p2 == 0.0 { 1.0 } else { 0.0 };
        var_guard54 = assign420_e355;
        var_guard54_rv = 0.0;

        let (assign430_e364,) = {
    if (((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 != 0.0)) {
        (0.0,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign430_e364;
        var_l_um_rv = 0.0;

        let (assign440_e373,) = {
    if (((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 != 0.0)) {
        (0.0,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign440_e373;
        var_leff_um_rv = 0.0;

        let (assign450_e384,) = {
    if (((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 != 0.0)) {
        let assign450_e382: f64 = (p.p0 * var_lfactor);
        (assign450_e382,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign450_e384;
        var_w_um_rv = 0.0;

        let (assign460_e395,) = {
    if (((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 != 0.0)) {
        let assign460_e393: f64 = (var_w_um + p.p22);
        (assign460_e393,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign460_e395;
        var_weff_um_rv = 0.0;

        let assign490_e416: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        var_guard55 = assign490_e416;
        var_guard55_rv = 0.0;

        let (assign500_e428,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        (0.0,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign500_e428;
        var_w_um_rv = 0.0;

        let (assign510_e440,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        (0.0,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign510_e440;
        var_weff_um_rv = 0.0;

        let (assign520_e454,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        let assign520_e452: f64 = (p.p1 * var_lfactor);
        (assign520_e452,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign520_e454;
        var_l_um_rv = 0.0;

        let (assign530_e468,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        let assign530_e466: f64 = (var_l_um + var_xleff);
        (assign530_e466,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign530_e468;
        var_leff_um_rv = 0.0;

        let (assign560_e507,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) {
        let assign560_e505: f64 = (p.p0 * var_lfactor);
        (assign560_e505,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign560_e507;
        var_w_um_rv = 0.0;

        let (assign570_e522,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) {
        let assign570_e520: f64 = (var_w_um + p.p22);
        (assign570_e520,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign570_e522;
        var_weff_um_rv = 0.0;

        let assign590_e528: f64 = if var_weff_um > 0.0 { 1.0 } else { 0.0 };
        var_guard57 = assign590_e528;
        var_guard57_rv = 0.0;

        let (assign600_e547,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 != 0.0)) {
        let assign600_e543: f64 = (p.p2 / p.p17);
        let assign600_e545: f64 = (assign600_e543 * var_weff_um);
        (assign600_e545,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign600_e547;
        var_leff_um_rv = 0.0;

        let (assign610_e564,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 != 0.0)) {
        let assign610_e562: f64 = (var_leff_um - var_xleff);
        (assign610_e562,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign610_e564;
        var_l_um_rv = 0.0;

        let (assign650_e617,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 == 0.0)) {
        let assign650_e615: f64 = (p.p1 * var_lfactor);
        (assign650_e615,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign650_e617;
        var_l_um_rv = 0.0;

        let (assign660_e635,) = {
    if (((((var_guard48 == 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard57 == 0.0)) {
        let assign660_e633: f64 = (var_l_um + var_xleff);
        (assign660_e633,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign660_e635;
        var_leff_um_rv = 0.0;

        let assign690_e670: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        var_guard59 = assign690_e670;
        var_guard59_rv = 0.0;

        let (assign700_e680,) = {
    if (((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 != 0.0)) {
        (0.0,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign700_e680;
        var_w_um_rv = 0.0;

        let (assign710_e690,) = {
    if (((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 != 0.0)) {
        (0.0,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign710_e690;
        var_weff_um_rv = 0.0;

        let (assign720_e702,) = {
    if (((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 != 0.0)) {
        let assign720_e700: f64 = (p.p1 * var_lfactor);
        (assign720_e700,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign720_e702;
        var_l_um_rv = 0.0;

        let (assign730_e714,) = {
    if (((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 != 0.0)) {
        let assign730_e712: f64 = (var_l_um + var_xleff);
        (assign730_e712,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign730_e714;
        var_leff_um_rv = 0.0;

        let assign760_e737: f64 = if p.p1 == 0.0 { 1.0 } else { 0.0 };
        var_guard60 = assign760_e737;
        var_guard60_rv = 0.0;

        let (assign770_e750,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 != 0.0)) {
        (0.0,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign770_e750;
        var_l_um_rv = 0.0;

        let (assign780_e763,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 != 0.0)) {
        (0.0,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign780_e763;
        var_leff_um_rv = 0.0;

        let (assign790_e778,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 != 0.0)) {
        let assign790_e776: f64 = (p.p0 * var_lfactor);
        (assign790_e776,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign790_e778;
        var_w_um_rv = 0.0;

        let (assign800_e793,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 != 0.0)) {
        let assign800_e791: f64 = (var_w_um + p.p22);
        (assign800_e791,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign800_e793;
        var_weff_um_rv = 0.0;

        let (assign830_e835,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) {
        let assign830_e833: f64 = (p.p0 * var_lfactor);
        (assign830_e833,)
    } else {
        (var_w_um,)
    }
};
        var_w_um = assign830_e835;
        var_w_um_rv = 0.0;

        let (assign840_e851,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) {
        let assign840_e849: f64 = (var_w_um + p.p22);
        (assign840_e849,)
    } else {
        (var_weff_um,)
    }
};
        var_weff_um = assign840_e851;
        var_weff_um_rv = 0.0;

        let (assign860_e870,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) {
        let assign860_e868: f64 = (p.p1 * var_lfactor);
        (assign860_e868,)
    } else {
        (var_l_um,)
    }
};
        var_l_um = assign860_e870;
        var_l_um_rv = 0.0;

        *var_guard41_slot = var_guard41;
        *var_guard41_rv_slot = var_guard41_rv;
        *var_guard42_slot = var_guard42;
        *var_guard42_rv_slot = var_guard42_rv;
        *var_guard46_slot = var_guard46;
        *var_guard46_rv_slot = var_guard46_rv;
        *var_guard47_slot = var_guard47;
        *var_guard47_rv_slot = var_guard47_rv;
        *var_guard48_slot = var_guard48;
        *var_guard48_rv_slot = var_guard48_rv;
        *var_guard49_slot = var_guard49;
        *var_guard49_rv_slot = var_guard49_rv;
        *var_guard51_slot = var_guard51;
        *var_guard51_rv_slot = var_guard51_rv;
        *var_guard53_slot = var_guard53;
        *var_guard53_rv_slot = var_guard53_rv;
        *var_guard54_slot = var_guard54;
        *var_guard54_rv_slot = var_guard54_rv;
        *var_guard55_slot = var_guard55;
        *var_guard55_rv_slot = var_guard55_rv;
        *var_guard57_slot = var_guard57;
        *var_guard57_rv_slot = var_guard57_rv;
        *var_guard59_slot = var_guard59;
        *var_guard59_rv_slot = var_guard59_rv;
        *var_guard60_slot = var_guard60;
        *var_guard60_rv_slot = var_guard60_rv;
        *var_l_um_slot = var_l_um;
        *var_l_um_rv_slot = var_l_um_rv;
        *var_leff_um_slot = var_leff_um;
        *var_leff_um_rv_slot = var_leff_um_rv;
        *var_lfactor_slot = var_lfactor;
        *var_lfactor_rv_slot = var_lfactor_rv;
        *var_scalefac_slot = var_scalefac;
        *var_scalefac_rv_slot = var_scalefac_rv;
        *var_shrinkl_slot = var_shrinkl;
        *var_shrinkl_rv_slot = var_shrinkl_rv;
        *var_w_um_slot = var_w_um;
        *var_w_um_rv_slot = var_w_um_rv;
        *var_weff_um_slot = var_weff_um;
        *var_weff_um_rv_slot = var_weff_um_rv;
        *var_xleff_slot = var_xleff;
        *var_xleff_rv_slot = var_xleff_rv;
    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard48: f64,
        var_guard53: f64,
        var_guard59: f64,
        var_guard60: f64,
        var_l_um: f64,
        var_w_um: f64,
        var_xleff: f64,
        var_a_um2_slot: &mut f64,
        var_a_um2_rv_slot: &mut f64,
        var_cth_slot: &mut f64,
        var_cth_rv_slot: &mut f64,
        var_guard75_slot: &mut f64,
        var_guard75_rv_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard76_rv_slot: &mut f64,
        var_leff_um_slot: &mut f64,
        var_leff_um_rv_slot: &mut f64,
        var_p_um_slot: &mut f64,
        var_p_um_rv_slot: &mut f64,
        var_qcth_slot: &mut f64,
        var_qcth_dn2_slot: &mut f64,
        var_qcth_rv_slot: &mut f64,
        var_vrth_slot: &mut f64,
        var_vrth_dn2_slot: &mut f64,
        var_vrth_rv_slot: &mut f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let mut var_a_um2: f64 = *var_a_um2_slot;
        let mut var_a_um2_rv: f64 = *var_a_um2_rv_slot;
        let mut var_cth: f64 = *var_cth_slot;
        let mut var_cth_rv: f64 = *var_cth_rv_slot;
        let mut var_guard75: f64 = *var_guard75_slot;
        let mut var_guard75_rv: f64 = *var_guard75_rv_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard76_rv: f64 = *var_guard76_rv_slot;
        let mut var_leff_um: f64 = *var_leff_um_slot;
        let mut var_leff_um_rv: f64 = *var_leff_um_rv_slot;
        let mut var_p_um: f64 = *var_p_um_slot;
        let mut var_p_um_rv: f64 = *var_p_um_rv_slot;
        let mut var_qcth: f64 = *var_qcth_slot;
        let mut var_qcth_dn2: f64 = *var_qcth_dn2_slot;
        let mut var_qcth_rv: f64 = *var_qcth_rv_slot;
        let mut var_vrth: f64 = *var_vrth_slot;
        let mut var_vrth_dn2: f64 = *var_vrth_dn2_slot;
        let mut var_vrth_rv: f64 = *var_vrth_rv_slot;

        let (assign870_e886,) = {
    if ((((var_guard48 == 0.0) && (var_guard53 == 0.0)) && (var_guard59 == 0.0)) && (var_guard60 == 0.0)) {
        let assign870_e884: f64 = (var_l_um + var_xleff);
        (assign870_e884,)
    } else {
        (var_leff_um,)
    }
};
        var_leff_um = assign870_e886;
        var_leff_um_rv = 0.0;

        let assign1190_e1146: f64 = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        var_guard75 = assign1190_e1146;
        var_guard75_rv = 0.0;

        let (assign1200_e1154,) = {
    if (var_guard75 != 0.0) {
        let assign1200_e1151: f64 = (var_l_um + var_w_um);
        let assign1200_e1152: f64 = (2.0 * assign1200_e1151);
        (assign1200_e1152,)
    } else {
        (var_p_um,)
    }
};
        var_p_um = assign1200_e1154;
        var_p_um_rv = 0.0;

        let assign1210_e1157: f64 = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        var_guard76 = assign1210_e1157;
        var_guard76_rv = 0.0;

        let (assign1220_e1168,) = {
    if ((var_guard75 == 0.0) && (var_guard76 != 0.0)) {
        let assign1220_e1164: f64 = (2.0 * var_l_um);
        let assign1220_e1166: f64 = (assign1220_e1164 + var_w_um);
        (assign1220_e1166,)
    } else {
        (var_p_um,)
    }
};
        var_p_um = assign1220_e1168;
        var_p_um_rv = 0.0;

        let (assign1230_e1178,) = {
    if ((var_guard75 == 0.0) && (var_guard76 == 0.0)) {
        let assign1230_e1176: f64 = (2.0 * var_l_um);
        (assign1230_e1176,)
    } else {
        (var_p_um,)
    }
};
        var_p_um = assign1230_e1178;
        var_p_um_rv = 0.0;

        let assign1240_e1181: f64 = (var_l_um * var_w_um);
        var_a_um2 = assign1240_e1181;
        var_a_um2_rv = 0.0;

        let assign1260_e1194: f64 = (p.p48 * var_p_um);
        let assign1260_e1195: f64 = (p.p47 + assign1260_e1194);
        let assign1260_e1198: f64 = (p.p49 * var_a_um2);
        let assign1260_e1199: f64 = (assign1260_e1195 + assign1260_e1198);
        var_cth = assign1260_e1199;
        var_cth_rv = 0.0;

        var_vrth = (nv2 - 0.0);
        var_vrth_dn2 = 1.0;
        var_vrth_rv = 0.0;

        let assign1620_e1409: f64 = (var_vrth * var_cth);
        var_qcth = assign1620_e1409;
        var_qcth_dn2 = (var_vrth_dn2 * var_cth);
        var_qcth_rv = 0.0;

        *var_a_um2_slot = var_a_um2;
        *var_a_um2_rv_slot = var_a_um2_rv;
        *var_cth_slot = var_cth;
        *var_cth_rv_slot = var_cth_rv;
        *var_guard75_slot = var_guard75;
        *var_guard75_rv_slot = var_guard75_rv;
        *var_guard76_slot = var_guard76;
        *var_guard76_rv_slot = var_guard76_rv;
        *var_leff_um_slot = var_leff_um;
        *var_leff_um_rv_slot = var_leff_um_rv;
        *var_p_um_slot = var_p_um;
        *var_p_um_rv_slot = var_p_um_rv;
        *var_qcth_slot = var_qcth;
        *var_qcth_dn2_slot = var_qcth_dn2;
        *var_qcth_rv_slot = var_qcth_rv;
        *var_vrth_slot = var_vrth;
        *var_vrth_dn2_slot = var_vrth_dn2;
        *var_vrth_rv_slot = var_vrth_rv;
    }
}
