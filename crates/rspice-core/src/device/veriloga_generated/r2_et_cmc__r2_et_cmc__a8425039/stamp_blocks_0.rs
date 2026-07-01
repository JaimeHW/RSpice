#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign10_e84: f64 = if param_given[10] { 1.0 } else { 0.0 };
        locals.var_guard41 = assign10_e84;

        let (assign20_e88,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p10,)
    } else {
        (locals.var_scalefac,)
    }
};
        locals.var_scalefac = assign20_e88;

        let (assign30_e95,) = {
    if (locals.var_guard41 == 0.0) {
        let assign30_e93: f64 = 1.0;
        (assign30_e93,)
    } else {
        (locals.var_scalefac,)
    }
};
        locals.var_scalefac = assign30_e95;

        let assign40_e97: f64 = if param_given[11] { 1.0 } else { 0.0 };
        locals.var_guard42 = assign40_e97;

        let (assign50_e105,) = {
    if (locals.var_guard42 != 0.0) {
        let assign50_e102: f64 = (0.01 * p.p11);
        let assign50_e103: f64 = (1.0 - assign50_e102);
        (assign50_e103,)
    } else {
        (locals.var_shrinkl,)
    }
};
        locals.var_shrinkl = assign50_e105;

        let (assign60_e116,) = {
    if (locals.var_guard42 == 0.0) {
        let assign60_e112: f64 = 0.0;
        let assign60_e113: f64 = (0.01 * assign60_e112);
        let assign60_e114: f64 = (1.0 - assign60_e113);
        (assign60_e114,)
    } else {
        (locals.var_shrinkl,)
    }
};
        locals.var_shrinkl = assign60_e116;

        let assign100_e132: f64 = (locals.var_shrinkl * locals.var_scalefac);
        let assign100_e134: f64 = (assign100_e132 * 1000000.0);
        locals.var_lfactor = assign100_e134;

        let assign110_e137: f64 = (273.15 + p.p16);
        locals.var_tinik = assign110_e137;

        let assign120_e138: f64 = ctx_temp;
        let assign120_e140: f64 = (assign120_e138 + p.p5);
        let assign120_e142: f64 = (assign120_e140 - 273.15);
        locals.var_tdevc = assign120_e142;
        locals.var_tdevc_dn2 = 0.0;

        let assign150_e151: f64 = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard46 = assign150_e151;

        let (assign160_e155,) = {
    if (locals.var_guard46 != 0.0) {
        (p.p23,)
    } else {
        (locals.var_xleff,)
    }
};
        locals.var_xleff = assign160_e155;

        let assign170_e158: f64 = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard47 = assign170_e158;

        let (assign180_e167,) = {
    if ((locals.var_guard46 == 0.0) && (locals.var_guard47 != 0.0)) {
        let assign180_e165: f64 = (p.p23 * 0.5);
        (assign180_e165,)
    } else {
        (locals.var_xleff,)
    }
};
        locals.var_xleff = assign180_e167;

        let (assign190_e175,) = {
    if ((locals.var_guard46 == 0.0) && (locals.var_guard47 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_xleff,)
    }
};
        locals.var_xleff = assign190_e175;

        let assign200_e184: f64 = if ((param_given[1] && param_given[2]) && (!param_given[0])) { 1.0 } else { 0.0 };
        locals.var_guard48 = assign200_e184;

        let assign210_e191: f64 = if ((p.p2 == 0.0) || (p.p1 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard49 = assign210_e191;

        let (assign220_e197,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign220_e197;

        let (assign230_e203,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign230_e203;

        let (assign240_e211,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        let assign240_e209: f64 = (p.p0 * locals.var_lfactor);
        (assign240_e209,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign240_e211;

        let (assign250_e219,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        let assign250_e217: f64 = (locals.var_w_um + p.p22);
        (assign250_e217,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign250_e219;

        let (assign260_e225,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign260_e225;

        let (assign270_e231,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        (1e99,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign270_e231;

        let (assign280_e240,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) {
        let assign280_e238: f64 = (p.p1 * locals.var_lfactor);
        (assign280_e238,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign280_e240;

        let (assign290_e249,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) {
        let assign290_e247: f64 = (locals.var_l_um + locals.var_xleff);
        (assign290_e247,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign290_e249;

        let assign310_e255: f64 = if locals.var_leff_um > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign310_e255;

        let (assign320_e268,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 != 0.0)) {
        let assign320_e264: f64 = (p.p17 / p.p2);
        let assign320_e266: f64 = (assign320_e264 * locals.var_leff_um);
        (assign320_e266,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign320_e268;

        let (assign330_e279,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 != 0.0)) {
        let assign330_e277: f64 = (locals.var_weff_um - p.p22);
        (assign330_e277,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign330_e279;

        let (assign350_e291,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign350_e291;

        let (assign360_e302,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 != 0.0)) {
        let assign360_e300: f64 = (1.0 / locals.var_r0);
        (assign360_e300,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign360_e302;

        let (assign370_e314,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 == 0.0)) {
        let assign370_e312: f64 = (p.p0 * locals.var_lfactor);
        (assign370_e312,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign370_e314;

        let (assign380_e326,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 == 0.0)) {
        let assign380_e324: f64 = (locals.var_w_um + p.p22);
        (assign380_e324,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign380_e326;

        let (assign390_e336,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign390_e336;

        let (assign400_e346,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 == 0.0)) {
        (1e99,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign400_e346;

        let assign410_e352: f64 = if (param_given[2] && (!param_given[1])) { 1.0 } else { 0.0 };
        locals.var_guard53 = assign410_e352;

        let assign420_e355: f64 = if p.p2 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign420_e355;

        let (assign430_e364,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign430_e364;

        let (assign440_e373,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign440_e373;

        let (assign450_e384,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        let assign450_e382: f64 = (p.p0 * locals.var_lfactor);
        (assign450_e382,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign450_e384;

        let (assign460_e395,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        let assign460_e393: f64 = (locals.var_w_um + p.p22);
        (assign460_e393,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign460_e395;

        let (assign470_e404,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign470_e404;

        let (assign480_e413,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        (1e99,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign480_e413;

        let assign490_e416: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign490_e416;

        let (assign500_e428,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign500_e428;

        let (assign510_e440,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign510_e440;

        let (assign520_e454,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        let assign520_e452: f64 = (p.p1 * locals.var_lfactor);
        (assign520_e452,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign520_e454;

        let (assign530_e468,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        let assign530_e466: f64 = (locals.var_l_um + locals.var_xleff);
        (assign530_e466,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign530_e468;

        let (assign540_e480,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        (1e99,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign540_e480;

        let (assign550_e492,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign550_e492;

        let (assign560_e507,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) {
        let assign560_e505: f64 = (p.p0 * locals.var_lfactor);
        (assign560_e505,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign560_e507;

        let (assign570_e522,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) {
        let assign570_e520: f64 = (locals.var_w_um + p.p22);
        (assign570_e520,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign570_e522;

        let assign590_e528: f64 = if locals.var_weff_um > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard57 = assign590_e528;

        let (assign600_e547,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 != 0.0)) {
        let assign600_e543: f64 = (p.p2 / p.p17);
        let assign600_e545: f64 = (assign600_e543 * locals.var_weff_um);
        (assign600_e545,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign600_e547;

        let (assign610_e564,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 != 0.0)) {
        let assign610_e562: f64 = (locals.var_leff_um - locals.var_xleff);
        (assign610_e562,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign610_e564;

        let (assign630_e582,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign630_e582;

        let (assign640_e599,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 != 0.0)) {
        let assign640_e597: f64 = (1.0 / locals.var_r0);
        (assign640_e597,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign640_e599;

        let (assign650_e617,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 == 0.0)) {
        let assign650_e615: f64 = (p.p1 * locals.var_lfactor);
        (assign650_e615,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign650_e617;

        let (assign660_e635,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 == 0.0)) {
        let assign660_e633: f64 = (locals.var_l_um + locals.var_xleff);
        (assign660_e633,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign660_e635;

        let (assign670_e651,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 == 0.0)) {
        (1e99,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign670_e651;

        let (assign680_e667,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign680_e667;

        let assign690_e670: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard59 = assign690_e670;

        let (assign700_e680,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign700_e680;

        let (assign710_e690,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign710_e690;

        let (assign720_e702,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 != 0.0)) {
        let assign720_e700: f64 = (p.p1 * locals.var_lfactor);
        (assign720_e700,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign720_e702;

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (assign730_e714,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 != 0.0)) {
        let assign730_e712: f64 = (locals.var_l_um + locals.var_xleff);
        (assign730_e712,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign730_e714;

        let (assign740_e724,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 != 0.0)) {
        (1e99,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign740_e724;

        let (assign750_e734,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign750_e734;

        let assign760_e737: f64 = if p.p1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard60 = assign760_e737;

        let (assign770_e750,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign770_e750;

        let (assign780_e763,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign780_e763;

        let (assign790_e778,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 != 0.0)) {
        let assign790_e776: f64 = (p.p0 * locals.var_lfactor);
        (assign790_e776,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign790_e778;

        let (assign800_e793,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 != 0.0)) {
        let assign800_e791: f64 = (locals.var_w_um + p.p22);
        (assign800_e791,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign800_e793;

        let (assign810_e806,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign810_e806;

        let (assign820_e819,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 != 0.0)) {
        (1e99,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign820_e819;

        let (assign830_e835,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) {
        let assign830_e833: f64 = (p.p0 * locals.var_lfactor);
        (assign830_e833,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign830_e835;

        let (assign840_e851,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) {
        let assign840_e849: f64 = (locals.var_w_um + p.p22);
        (assign840_e849,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign840_e851;

        let (assign860_e870,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) {
        let assign860_e868: f64 = (p.p1 * locals.var_lfactor);
        (assign860_e868,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign860_e870;

        let (assign870_e886,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) {
        let assign870_e884: f64 = (locals.var_l_um + locals.var_xleff);
        (assign870_e884,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign870_e886;

        let assign880_e889: f64 = if locals.var_weff_um > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard62 = assign880_e889;

        let assign900_e895: f64 = if locals.var_leff_um > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard64 = assign900_e895;

        let (assign910_e917,) = {
    if ((((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) && (locals.var_guard62 != 0.0)) && (locals.var_guard64 != 0.0)) {
        let assign910_e914: f64 = (locals.var_leff_um / locals.var_weff_um);
        let assign910_e915: f64 = (p.p17 * assign910_e914);
        (assign910_e915,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign910_e917;

        let (assign920_e937,) = {
    if ((((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) && (locals.var_guard62 != 0.0)) && (locals.var_guard64 != 0.0)) {
        let assign920_e935: f64 = (1.0 / locals.var_r0);
        (assign920_e935,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign920_e937;

        let (assign930_e956,) = {
    if ((((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) && (locals.var_guard62 != 0.0)) && (locals.var_guard64 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign930_e956;

        let (assign940_e975,) = {
    if ((((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) && (locals.var_guard62 != 0.0)) && (locals.var_guard64 == 0.0)) {
        (1e99,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign940_e975;

        let (assign950_e992,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) && (locals.var_guard62 == 0.0)) {
        (1e99,)
    } else {
        (locals.var_r0,)
    }
};
        locals.var_r0 = assign950_e992;

        let (assign960_e1009,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) && (locals.var_guard62 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_g0,)
    }
};
        locals.var_g0 = assign960_e1009;

        let (assign1010_e1027,) = {
    if (p.p25 != 0.0) {
        let assign1010_e1025: f64 = (locals.var_leff_um + p.p24);
        (assign1010_e1025,)
    } else {
        (locals.var_l_umfore,)
    }
};
        locals.var_l_umfore = assign1010_e1027;

        let (assign1020_e1034,) = {
    if (p.p25 == 0.0) {
        let assign1020_e1032: f64 = (locals.var_l_um + p.p24);
        (assign1020_e1032,)
    } else {
        (locals.var_l_umfore,)
    }
};
        locals.var_l_umfore = assign1020_e1034;

        locals.var_tc1e = p.p37;

        locals.var_tc2e = p.p38;

        let assign1060_e1054: f64 = if locals.var_leff_um > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign1060_e1054;

        let assign1070_e1057: f64 = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard71 = assign1070_e1057;

        let (assign1080_e1067,) = {
    if ((locals.var_guard70 != 0.0) && (locals.var_guard71 != 0.0)) {
        let assign1080_e1064: f64 = (p.p39 / locals.var_leff_um);
        let assign1080_e1065: f64 = (locals.var_tc1e + assign1080_e1064);
        (assign1080_e1065,)
    } else {
        (locals.var_tc1e,)
    }
};
        locals.var_tc1e = assign1080_e1067;

        let (assign1090_e1077,) = {
    if ((locals.var_guard70 != 0.0) && (locals.var_guard71 != 0.0)) {
        let assign1090_e1074: f64 = (p.p40 / locals.var_leff_um);
        let assign1090_e1075: f64 = (locals.var_tc2e + assign1090_e1074);
        (assign1090_e1075,)
    } else {
        (locals.var_tc2e,)
    }
};
        locals.var_tc2e = assign1090_e1077;

        let assign1100_e1080: f64 = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard72 = assign1100_e1080;

        let (assign1110_e1095,) = {
    if (((locals.var_guard70 != 0.0) && (locals.var_guard71 == 0.0)) && (locals.var_guard72 != 0.0)) {
        let assign1110_e1090: f64 = (0.5 * p.p39);
        let assign1110_e1092: f64 = (assign1110_e1090 / locals.var_leff_um);
        let assign1110_e1093: f64 = (locals.var_tc1e + assign1110_e1092);
        (assign1110_e1093,)
    } else {
        (locals.var_tc1e,)
    }
};
        locals.var_tc1e = assign1110_e1095;

        let (assign1120_e1110,) = {
    if (((locals.var_guard70 != 0.0) && (locals.var_guard71 == 0.0)) && (locals.var_guard72 != 0.0)) {
        let assign1120_e1105: f64 = (0.5 * p.p40);
        let assign1120_e1107: f64 = (assign1120_e1105 / locals.var_leff_um);
        let assign1120_e1108: f64 = (locals.var_tc2e + assign1120_e1107);
        (assign1120_e1108,)
    } else {
        (locals.var_tc2e,)
    }
};
        locals.var_tc2e = assign1120_e1110;

        let assign1130_e1113: f64 = if locals.var_weff_um > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign1130_e1113;

        let (assign1140_e1121,) = {
    if (locals.var_guard73 != 0.0) {
        let assign1140_e1118: f64 = (p.p41 / locals.var_weff_um);
        let assign1140_e1119: f64 = (locals.var_tc1e + assign1140_e1118);
        (assign1140_e1119,)
    } else {
        (locals.var_tc1e,)
    }
};
        locals.var_tc1e = assign1140_e1121;

        let (assign1150_e1129,) = {
    if (locals.var_guard73 != 0.0) {
        let assign1150_e1126: f64 = (p.p42 / locals.var_weff_um);
        let assign1150_e1127: f64 = (locals.var_tc2e + assign1150_e1126);
        (assign1150_e1127,)
    } else {
        (locals.var_tc2e,)
    }
};
        locals.var_tc2e = assign1150_e1129;

        let assign1190_e1146: f64 = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard75 = assign1190_e1146;

        let (assign1200_e1154,) = {
    if (locals.var_guard75 != 0.0) {
        let assign1200_e1151: f64 = (locals.var_l_um + locals.var_w_um);
        let assign1200_e1152: f64 = (2.0 * assign1200_e1151);
        (assign1200_e1152,)
    } else {
        (locals.var_p_um,)
    }
};
        locals.var_p_um = assign1200_e1154;

        let assign1210_e1157: f64 = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard76 = assign1210_e1157;

        let (assign1220_e1168,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard76 != 0.0)) {
        let assign1220_e1164: f64 = (2.0 * locals.var_l_um);
        let assign1220_e1166: f64 = (assign1220_e1164 + locals.var_w_um);
        (assign1220_e1166,)
    } else {
        (locals.var_p_um,)
    }
};
        locals.var_p_um = assign1220_e1168;

        let (assign1230_e1178,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard76 == 0.0)) {
        let assign1230_e1176: f64 = (2.0 * locals.var_l_um);
        (assign1230_e1176,)
    } else {
        (locals.var_p_um,)
    }
};
        locals.var_p_um = assign1230_e1178;

        let assign1240_e1181: f64 = (locals.var_l_um * locals.var_w_um);
        locals.var_a_um2 = assign1240_e1181;

        let assign1250_e1185: f64 = (p.p45 * locals.var_p_um);
        let assign1250_e1186: f64 = (p.p44 + assign1250_e1185);
        let assign1250_e1189: f64 = (p.p46 * locals.var_a_um2);
        let assign1250_e1190: f64 = (assign1250_e1186 + assign1250_e1189);
        locals.var_gth = assign1250_e1190;

        let assign1260_e1194: f64 = (p.p48 * locals.var_p_um);
        let assign1260_e1195: f64 = (p.p47 + assign1260_e1194);
        let assign1260_e1198: f64 = (p.p49 * locals.var_a_um2);
        let assign1260_e1199: f64 = (assign1260_e1195 + assign1260_e1198);
        locals.var_cth = assign1260_e1199;

        locals.var_vrth = (nv2 - 0.0);
        locals.var_vrth_dn2 = 1.0;

        let assign1290_e1206: f64 = (p.p7 * locals.var_vrth);
        let assign1290_e1207: f64 = (locals.var_tdevc + assign1290_e1206);
        locals.var_tdevc = assign1290_e1207;
        locals.var_tdevc_dn2 = (locals.var_tdevc_dn2 + (p.p7 * locals.var_vrth_dn2));

        let assign1300_e1211: f64 = (p.p35 + 1.0);
        let assign1300_e1212: f64 = if locals.var_tdevc < assign1300_e1211 { 1.0 } else { 0.0 };
        locals.var_guard78 = assign1300_e1212;

        let (assign1310_e1223, assign1310_e1223_d_n2,) = {
    if (locals.var_guard78 != 0.0) {
        let assign1310_e1217: f64 = (locals.var_tdevc - p.p35);
        let assign1310_e1219: f64 = (assign1310_e1217 - 1.0);
        let assign1310_e1220: f64 = (assign1310_e1219).exp();
        let assign1310_e1221: f64 = (p.p35 + assign1310_e1220);
        (assign1310_e1221, (assign1310_e1220 * locals.var_tdevc_dn2),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn2,)
    }
};
        locals.var_tdevc = assign1310_e1223;
        locals.var_tdevc_dn2 = assign1310_e1223_d_n2;

        let assign1320_e1227: f64 = (p.p36 - 1.0);
        let assign1320_e1228: f64 = if locals.var_tdevc > assign1320_e1227 { 1.0 } else { 0.0 };
        locals.var_guard79 = assign1320_e1228;

        let (assign1330_e1242, assign1330_e1242_d_n2,) = {
    if ((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) {
        let assign1330_e1236: f64 = (p.p36 - locals.var_tdevc);
        let assign1330_e1238: f64 = (assign1330_e1236 - 1.0);
        let assign1330_e1239: f64 = (assign1330_e1238).exp();
        let assign1330_e1240: f64 = (p.p36 - assign1330_e1239);
        (assign1330_e1240, (-(assign1330_e1239 * (-locals.var_tdevc_dn2))),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn2,)
    }
};
        locals.var_tdevc = assign1330_e1242;
        locals.var_tdevc_dn2 = assign1330_e1242_d_n2;

        let (assign1340_e1250, assign1340_e1250_d_n2,) = {
    if ((locals.var_guard78 == 0.0) && (locals.var_guard79 == 0.0)) {
        (locals.var_tdevc, locals.var_tdevc_dn2,)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn2,)
    }
};
        locals.var_tdevc = assign1340_e1250;
        locals.var_tdevc_dn2 = assign1340_e1250_d_n2;

        let assign1350_e1253: f64 = (locals.var_tdevc + 273.15);
        locals.var_tdevk = assign1350_e1253;
        locals.var_tdevk_dn2 = locals.var_tdevc_dn2;

        let assign1360_e1256: f64 = (locals.var_tdevk - locals.var_tinik);
        locals.var_delt = assign1360_e1256;
        locals.var_delt_dn2 = locals.var_tdevk_dn2;

        let assign1370_e1262: f64 = (locals.var_delt * locals.var_tc2e);
        let assign1370_e1263: f64 = (locals.var_tc1e + assign1370_e1262);
        let assign1370_e1264: f64 = (locals.var_delt * assign1370_e1263);
        let assign1370_e1265: f64 = (1.0 + assign1370_e1264);
        locals.var_tcr = assign1370_e1265;
        locals.var_tcr_dn2 = ((locals.var_delt_dn2 * assign1370_e1263) + (locals.var_delt * (locals.var_delt_dn2 * locals.var_tc2e)));

        let assign1380_e1269: f64 = (0.01 + 0.1);
        let assign1380_e1270: f64 = if locals.var_tcr < assign1380_e1269 { 1.0 } else { 0.0 };
        locals.var_guard80 = assign1380_e1270;

        let (assign1390_e1285, assign1390_e1285_d_n2,) = {
    if (locals.var_guard80 != 0.0) {
        let assign1390_e1277: f64 = (locals.var_tcr - 0.01);
        let assign1390_e1278: f64 = (10.0 * assign1390_e1277);
        let assign1390_e1280: f64 = (assign1390_e1278 - 1.0);
        let assign1390_e1281: f64 = (assign1390_e1280).exp();
        let assign1390_e1282: f64 = (0.1 * assign1390_e1281);
        let assign1390_e1283: f64 = (0.01 + assign1390_e1282);
        (assign1390_e1283, (0.1 * (assign1390_e1281 * (10.0 * locals.var_tcr_dn2))),)
    } else {
        (locals.var_tcr, locals.var_tcr_dn2,)
    }
};
        locals.var_tcr = assign1390_e1285;
        locals.var_tcr_dn2 = assign1390_e1285_d_n2;

        let (assign1400_e1290, assign1400_e1290_d_n2,) = {
    if (locals.var_guard80 == 0.0) {
        (locals.var_tcr, locals.var_tcr_dn2,)
    } else {
        (locals.var_tcr, locals.var_tcr_dn2,)
    }
};
        locals.var_tcr = assign1400_e1290;
        locals.var_tcr_dn2 = assign1400_e1290_d_n2;

        let assign1410_e1293: f64 = (locals.var_r0 * locals.var_tcr);
        locals.var_r0_t = assign1410_e1293;
        locals.var_r0_t_dn2 = (locals.var_r0 * locals.var_tcr_dn2);

        locals.var_vin = (nv0 - nv1);
        locals.var_vin_dn0 = 1.0;
        locals.var_vin_dn1 = -1.0;

        let assign1470_e1322: f64 = if ((locals.var_r0 > 0.0) && ((p.p29 > 0.0) || (p.p27 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard82 = assign1470_e1322;

        let (assign1480_e1328, assign1480_e1328_d_n0, assign1480_e1328_d_n1,) = {
    if (locals.var_guard82 != 0.0) {
        let assign1480_e1326: f64 = (locals.var_vin / locals.var_l_umfore);
        (assign1480_e1326, (locals.var_vin_dn0 / locals.var_l_umfore), (locals.var_vin_dn1 / locals.var_l_umfore),)
    } else {
        (locals.var_e, locals.var_e_dn0, locals.var_e_dn1,)
    }
};
        locals.var_e = assign1480_e1328;
        locals.var_e_dn0 = assign1480_e1328_d_n0;
        locals.var_e_dn1 = assign1480_e1328_d_n1;

        let (assign1490_e1334, assign1490_e1334_d_n0, assign1490_e1334_d_n1,) = {
    if (locals.var_guard82 != 0.0) {
        let assign1490_e1332: f64 = (p.p28 * locals.var_e);
        (assign1490_e1332, (p.p28 * locals.var_e_dn0), (p.p28 * locals.var_e_dn1),)
    } else {
        (locals.var_q2e, locals.var_q2e_dn0, locals.var_q2e_dn1,)
    }
};
        locals.var_q2e = assign1490_e1334;
        locals.var_q2e_dn0 = assign1490_e1334_d_n0;
        locals.var_q2e_dn1 = assign1490_e1334_d_n1;

        let (assign1500_e1343, assign1500_e1343_d_n0, assign1500_e1343_d_n1,) = {
    if (locals.var_guard82 != 0.0) {
        let assign1500_e1339: f64 = (locals.var_q2e * locals.var_q2e);
        let assign1500_e1340: f64 = (1.0 + assign1500_e1339);
        let assign1500_e1341: f64 = (assign1500_e1340).sqrt();
        (assign1500_e1341, (((locals.var_q2e_dn0 * locals.var_q2e) + (locals.var_q2e * locals.var_q2e_dn0)) / (2.0 * assign1500_e1341)), (((locals.var_q2e_dn1 * locals.var_q2e) + (locals.var_q2e * locals.var_q2e_dn1)) / (2.0 * assign1500_e1341)),)
    } else {
        (locals.var_sqrf, locals.var_sqrf_dn0, locals.var_sqrf_dn1,)
    }
};
        locals.var_sqrf = assign1500_e1343;
        locals.var_sqrf_dn0 = assign1500_e1343_d_n0;
        locals.var_sqrf_dn1 = assign1500_e1343_d_n1;

        let (assign1510_e1350, assign1510_e1350_d_n0, assign1510_e1350_d_n1,) = {
    if (locals.var_guard82 != 0.0) {
        let assign1510_e1347: f64 = (locals.var_e).abs();
        let assign1510_e1348: f64 = (p.p26 * assign1510_e1347);
        (assign1510_e1348, (p.p26 * if locals.var_e >= 0.0 { locals.var_e_dn0 } else { (-locals.var_e_dn0) }), (p.p26 * if locals.var_e >= 0.0 { locals.var_e_dn1 } else { (-locals.var_e_dn1) }),)
    } else {
        (locals.var_q3e, locals.var_q3e_dn0, locals.var_q3e_dn1,)
    }
};
        locals.var_q3e = assign1510_e1350;
        locals.var_q3e_dn0 = assign1510_e1350_d_n0;
        locals.var_q3e_dn1 = assign1510_e1350_d_n1;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1520_e1362, assign1520_e1362_d_n0, assign1520_e1362_d_n1,) = {
    if (locals.var_guard82 != 0.0) {
        let assign1520_e1355: f64 = (locals.var_q3e * locals.var_q3e);
        let assign1520_e1357: f64 = (assign1520_e1355 * locals.var_q3e);
        let assign1520_e1358: f64 = (1.0 + assign1520_e1357);
        let assign1520_e1360: f64 = (assign1520_e1358).powf(0.3333333333333333);
        (assign1520_e1360, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign1520_e1358).powf(0.3333333333333333 - 1.0) * ((((locals.var_q3e_dn0 * locals.var_q3e) + (locals.var_q3e * locals.var_q3e_dn0)) * locals.var_q3e) + (assign1520_e1355 * locals.var_q3e_dn0)))) } } else { (assign1520_e1360 * (0.3333333333333333 * (((((locals.var_q3e_dn0 * locals.var_q3e) + (locals.var_q3e * locals.var_q3e_dn0)) * locals.var_q3e) + (assign1520_e1355 * locals.var_q3e_dn0)) / assign1520_e1358))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign1520_e1358).powf(0.3333333333333333 - 1.0) * ((((locals.var_q3e_dn1 * locals.var_q3e) + (locals.var_q3e * locals.var_q3e_dn1)) * locals.var_q3e) + (assign1520_e1355 * locals.var_q3e_dn1)))) } } else { (assign1520_e1360 * (0.3333333333333333 * (((((locals.var_q3e_dn1 * locals.var_q3e) + (locals.var_q3e * locals.var_q3e_dn1)) * locals.var_q3e) + (assign1520_e1355 * locals.var_q3e_dn1)) / assign1520_e1358))) },)
    } else {
        (locals.var_cbrf, locals.var_cbrf_dn0, locals.var_cbrf_dn1,)
    }
};
        locals.var_cbrf = assign1520_e1362;
        locals.var_cbrf_dn0 = assign1520_e1362_d_n0;
        locals.var_cbrf_dn1 = assign1520_e1362_d_n1;

        let (assign1530_e1378, assign1530_e1378_d_n0, assign1530_e1378_d_n1,) = {
    if (locals.var_guard82 != 0.0) {
        let assign1530_e1366: f64 = (1.0 - p.p29);
        let assign1530_e1368: f64 = (assign1530_e1366 - p.p27);
        let assign1530_e1371: f64 = (p.p29 * locals.var_sqrf);
        let assign1530_e1372: f64 = (assign1530_e1368 + assign1530_e1371);
        let assign1530_e1375: f64 = (p.p27 * locals.var_cbrf);
        let assign1530_e1376: f64 = (assign1530_e1372 + assign1530_e1375);
        (assign1530_e1376, ((p.p29 * locals.var_sqrf_dn0) + (p.p27 * locals.var_cbrf_dn0)), ((p.p29 * locals.var_sqrf_dn1) + (p.p27 * locals.var_cbrf_dn1)),)
    } else {
        (locals.var_rfactor, locals.var_rfactor_dn0, locals.var_rfactor_dn1,)
    }
};
        locals.var_rfactor = assign1530_e1378;
        locals.var_rfactor_dn0 = assign1530_e1378_d_n0;
        locals.var_rfactor_dn1 = assign1530_e1378_d_n1;

        let (assign1540_e1383, assign1540_e1383_d_n0, assign1540_e1383_d_n1,) = {
    if (locals.var_guard82 == 0.0) {
        (1.0, 0.0, 0.0,)
    } else {
        (locals.var_rfactor, locals.var_rfactor_dn0, locals.var_rfactor_dn1,)
    }
};
        locals.var_rfactor = assign1540_e1383;
        locals.var_rfactor_dn0 = assign1540_e1383_d_n0;
        locals.var_rfactor_dn1 = assign1540_e1383_d_n1;

        let assign1550_e1386: f64 = (locals.var_r0_t * locals.var_rfactor);
        locals.var_r_dc = assign1550_e1386;
        locals.var_r_dc_dn0 = (locals.var_r0_t * locals.var_rfactor_dn0);
        locals.var_r_dc_dn1 = (locals.var_r0_t * locals.var_rfactor_dn1);
        locals.var_r_dc_dn2 = (locals.var_r0_t_dn2 * locals.var_rfactor);

        locals.var_v = locals.var_vin;
        locals.var_v_dn0 = locals.var_vin_dn0;
        locals.var_v_dn1 = locals.var_vin_dn1;

        let assign1570_e1390: f64 = (locals.var_v / locals.var_r_dc);
        locals.var_i = assign1570_e1390;
        locals.var_i_dn0 = (((locals.var_v_dn0 * locals.var_r_dc) - (locals.var_v * locals.var_r_dc_dn0)) / (locals.var_r_dc * locals.var_r_dc));
        locals.var_i_dn1 = (((locals.var_v_dn1 * locals.var_r_dc) - (locals.var_v * locals.var_r_dc_dn1)) / (locals.var_r_dc * locals.var_r_dc));
        locals.var_i_dn2 = (-((locals.var_v * locals.var_r_dc_dn2) / (locals.var_r_dc * locals.var_r_dc)));

        let assign1580_e1392: f64 = (-locals.var_v);
        let assign1580_e1394: f64 = (assign1580_e1392 * locals.var_i);
        locals.var_ith = assign1580_e1394;
        locals.var_ith_dn0 = (((-locals.var_v_dn0) * locals.var_i) + (assign1580_e1392 * locals.var_i_dn0));
        locals.var_ith_dn1 = (((-locals.var_v_dn1) * locals.var_i) + (assign1580_e1392 * locals.var_i_dn1));
        locals.var_ith_dn2 = (assign1580_e1392 * locals.var_i_dn2);

        let assign1590_e1397: f64 = (locals.var_vrth * locals.var_gth);
        locals.var_irth = assign1590_e1397;
        locals.var_irth_dn2 = (locals.var_vrth_dn2 * locals.var_gth);

        let assign1620_e1409: f64 = (locals.var_vrth * locals.var_cth);
        locals.var_qcth = assign1620_e1409;
        locals.var_qcth_dn2 = (locals.var_vrth_dn2 * locals.var_cth);

        let assign1750_e1523: f64 = if ((locals.var_r0 > 0.0) && (locals.var_g0 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard89 = assign1750_e1523;

        let (assign1760_e1529, assign1760_e1529_d_n0, assign1760_e1529_d_n1, assign1760_e1529_d_n2,) = {
    if (locals.var_guard89 != 0.0) {
        let assign1760_e1527: f64 = (locals.var_r0_t * locals.var_rfactor);
        (assign1760_e1527, (locals.var_r0_t * locals.var_rfactor_dn0), (locals.var_r0_t * locals.var_rfactor_dn1), (locals.var_r0_t_dn2 * locals.var_rfactor),)
    } else {
        (locals.var_r_dc, locals.var_r_dc_dn0, locals.var_r_dc_dn1, locals.var_r_dc_dn2,)
    }
};
        locals.var_r_dc = assign1760_e1529;
        locals.var_r_dc_dn0 = assign1760_e1529_d_n0;
        locals.var_r_dc_dn1 = assign1760_e1529_d_n1;
        locals.var_r_dc_dn2 = assign1760_e1529_d_n2;

        let (assign1860_e1604, assign1860_e1604_d_n0, assign1860_e1604_d_n1, assign1860_e1604_d_n2,) = {
    if (locals.var_guard89 == 0.0) {
        (locals.var_r0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_r_dc, locals.var_r_dc_dn0, locals.var_r_dc_dn1, locals.var_r_dc_dn2,)
    }
};
        locals.var_r_dc = assign1860_e1604;
        locals.var_r_dc_dn0 = assign1860_e1604_d_n0;
        locals.var_r_dc_dn1 = assign1860_e1604_d_n1;
        locals.var_r_dc_dn2 = assign1860_e1604_d_n2;

    }

    pub(super) fn stamp_reactive_block_0(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign10_e84: f64 = if param_given[10] { 1.0 } else { 0.0 };
        locals.var_guard41 = assign10_e84;
        locals.var_guard41_rv = 0.0;

        let (assign20_e88,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p10,)
    } else {
        (locals.var_scalefac,)
    }
};
        locals.var_scalefac = assign20_e88;
        locals.var_scalefac_rv = 0.0;

        let (assign30_e95,) = {
    if (locals.var_guard41 == 0.0) {
        let assign30_e93: f64 = 1.0;
        (assign30_e93,)
    } else {
        (locals.var_scalefac,)
    }
};
        locals.var_scalefac = assign30_e95;
        locals.var_scalefac_rv = 0.0;

        let assign40_e97: f64 = if param_given[11] { 1.0 } else { 0.0 };
        locals.var_guard42 = assign40_e97;
        locals.var_guard42_rv = 0.0;

        let (assign50_e105,) = {
    if (locals.var_guard42 != 0.0) {
        let assign50_e102: f64 = (0.01 * p.p11);
        let assign50_e103: f64 = (1.0 - assign50_e102);
        (assign50_e103,)
    } else {
        (locals.var_shrinkl,)
    }
};
        locals.var_shrinkl = assign50_e105;
        locals.var_shrinkl_rv = 0.0;

        let (assign60_e116,) = {
    if (locals.var_guard42 == 0.0) {
        let assign60_e112: f64 = 0.0;
        let assign60_e113: f64 = (0.01 * assign60_e112);
        let assign60_e114: f64 = (1.0 - assign60_e113);
        (assign60_e114,)
    } else {
        (locals.var_shrinkl,)
    }
};
        locals.var_shrinkl = assign60_e116;
        locals.var_shrinkl_rv = 0.0;

        let assign100_e132: f64 = (locals.var_shrinkl * locals.var_scalefac);
        let assign100_e134: f64 = (assign100_e132 * 1000000.0);
        locals.var_lfactor = assign100_e134;
        locals.var_lfactor_rv = 0.0;

        let assign150_e151: f64 = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard46 = assign150_e151;
        locals.var_guard46_rv = 0.0;

        let (assign160_e155,) = {
    if (locals.var_guard46 != 0.0) {
        (p.p23,)
    } else {
        (locals.var_xleff,)
    }
};
        locals.var_xleff = assign160_e155;
        locals.var_xleff_rv = 0.0;

        let assign170_e158: f64 = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard47 = assign170_e158;
        locals.var_guard47_rv = 0.0;

        let (assign180_e167,) = {
    if ((locals.var_guard46 == 0.0) && (locals.var_guard47 != 0.0)) {
        let assign180_e165: f64 = (p.p23 * 0.5);
        (assign180_e165,)
    } else {
        (locals.var_xleff,)
    }
};
        locals.var_xleff = assign180_e167;
        locals.var_xleff_rv = 0.0;

        let (assign190_e175,) = {
    if ((locals.var_guard46 == 0.0) && (locals.var_guard47 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_xleff,)
    }
};
        locals.var_xleff = assign190_e175;
        locals.var_xleff_rv = 0.0;

        let assign200_e184: f64 = if ((param_given[1] && param_given[2]) && (!param_given[0])) { 1.0 } else { 0.0 };
        locals.var_guard48 = assign200_e184;
        locals.var_guard48_rv = 0.0;

        let assign210_e191: f64 = if ((p.p2 == 0.0) || (p.p1 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard49 = assign210_e191;
        locals.var_guard49_rv = 0.0;

        let (assign220_e197,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign220_e197;
        locals.var_l_um_rv = 0.0;

        let (assign230_e203,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign230_e203;
        locals.var_leff_um_rv = 0.0;

        let (assign240_e211,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        let assign240_e209: f64 = (p.p0 * locals.var_lfactor);
        (assign240_e209,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign240_e211;
        locals.var_w_um_rv = 0.0;

        let (assign250_e219,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        let assign250_e217: f64 = (locals.var_w_um + p.p22);
        (assign250_e217,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign250_e219;
        locals.var_weff_um_rv = 0.0;

        let (assign280_e240,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) {
        let assign280_e238: f64 = (p.p1 * locals.var_lfactor);
        (assign280_e238,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign280_e240;
        locals.var_l_um_rv = 0.0;

        let (assign290_e249,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) {
        let assign290_e247: f64 = (locals.var_l_um + locals.var_xleff);
        (assign290_e247,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign290_e249;
        locals.var_leff_um_rv = 0.0;

        let assign310_e255: f64 = if locals.var_leff_um > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign310_e255;
        locals.var_guard51_rv = 0.0;

        let (assign320_e268,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 != 0.0)) {
        let assign320_e264: f64 = (p.p17 / p.p2);
        let assign320_e266: f64 = (assign320_e264 * locals.var_leff_um);
        (assign320_e266,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign320_e268;
        locals.var_weff_um_rv = 0.0;

        let (assign330_e279,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 != 0.0)) {
        let assign330_e277: f64 = (locals.var_weff_um - p.p22);
        (assign330_e277,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign330_e279;
        locals.var_w_um_rv = 0.0;

        let (assign370_e314,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 == 0.0)) {
        let assign370_e312: f64 = (p.p0 * locals.var_lfactor);
        (assign370_e312,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign370_e314;
        locals.var_w_um_rv = 0.0;

        let (assign380_e326,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard51 == 0.0)) {
        let assign380_e324: f64 = (locals.var_w_um + p.p22);
        (assign380_e324,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign380_e326;
        locals.var_weff_um_rv = 0.0;

        let assign410_e352: f64 = if (param_given[2] && (!param_given[1])) { 1.0 } else { 0.0 };
        locals.var_guard53 = assign410_e352;
        locals.var_guard53_rv = 0.0;

        let assign420_e355: f64 = if p.p2 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign420_e355;
        locals.var_guard54_rv = 0.0;

        let (assign430_e364,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign430_e364;
        locals.var_l_um_rv = 0.0;

        let (assign440_e373,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign440_e373;
        locals.var_leff_um_rv = 0.0;

        let (assign450_e384,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        let assign450_e382: f64 = (p.p0 * locals.var_lfactor);
        (assign450_e382,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign450_e384;
        locals.var_w_um_rv = 0.0;

        let (assign460_e395,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 != 0.0)) {
        let assign460_e393: f64 = (locals.var_w_um + p.p22);
        (assign460_e393,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign460_e395;
        locals.var_weff_um_rv = 0.0;

        let assign490_e416: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign490_e416;
        locals.var_guard55_rv = 0.0;

        let (assign500_e428,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign500_e428;
        locals.var_w_um_rv = 0.0;

        let (assign510_e440,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign510_e440;
        locals.var_weff_um_rv = 0.0;

        let (assign520_e454,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        let assign520_e452: f64 = (p.p1 * locals.var_lfactor);
        (assign520_e452,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign520_e454;
        locals.var_l_um_rv = 0.0;

        let (assign530_e468,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        let assign530_e466: f64 = (locals.var_l_um + locals.var_xleff);
        (assign530_e466,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign530_e468;
        locals.var_leff_um_rv = 0.0;

        let (assign560_e507,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) {
        let assign560_e505: f64 = (p.p0 * locals.var_lfactor);
        (assign560_e505,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign560_e507;
        locals.var_w_um_rv = 0.0;

        let (assign570_e522,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) {
        let assign570_e520: f64 = (locals.var_w_um + p.p22);
        (assign570_e520,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign570_e522;
        locals.var_weff_um_rv = 0.0;

        let assign590_e528: f64 = if locals.var_weff_um > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard57 = assign590_e528;
        locals.var_guard57_rv = 0.0;

        let (assign600_e547,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 != 0.0)) {
        let assign600_e543: f64 = (p.p2 / p.p17);
        let assign600_e545: f64 = (assign600_e543 * locals.var_weff_um);
        (assign600_e545,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign600_e547;
        locals.var_leff_um_rv = 0.0;

        let (assign610_e564,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 != 0.0)) {
        let assign610_e562: f64 = (locals.var_leff_um - locals.var_xleff);
        (assign610_e562,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign610_e564;
        locals.var_l_um_rv = 0.0;

        let (assign650_e617,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 == 0.0)) {
        let assign650_e615: f64 = (p.p1 * locals.var_lfactor);
        (assign650_e615,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign650_e617;
        locals.var_l_um_rv = 0.0;

        let (assign660_e635,) = {
    if (((((locals.var_guard48 == 0.0) && (locals.var_guard53 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) && (locals.var_guard57 == 0.0)) {
        let assign660_e633: f64 = (locals.var_l_um + locals.var_xleff);
        (assign660_e633,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign660_e635;
        locals.var_leff_um_rv = 0.0;

        let assign690_e670: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard59 = assign690_e670;
        locals.var_guard59_rv = 0.0;

        let (assign700_e680,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign700_e680;
        locals.var_w_um_rv = 0.0;

        let (assign710_e690,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign710_e690;
        locals.var_weff_um_rv = 0.0;

        let (assign720_e702,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 != 0.0)) {
        let assign720_e700: f64 = (p.p1 * locals.var_lfactor);
        (assign720_e700,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign720_e702;
        locals.var_l_um_rv = 0.0;

        let (assign730_e714,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 != 0.0)) {
        let assign730_e712: f64 = (locals.var_l_um + locals.var_xleff);
        (assign730_e712,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign730_e714;
        locals.var_leff_um_rv = 0.0;

        let assign760_e737: f64 = if p.p1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard60 = assign760_e737;
        locals.var_guard60_rv = 0.0;

        let (assign770_e750,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign770_e750;
        locals.var_l_um_rv = 0.0;

        let (assign780_e763,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign780_e763;
        locals.var_leff_um_rv = 0.0;

        let (assign790_e778,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 != 0.0)) {
        let assign790_e776: f64 = (p.p0 * locals.var_lfactor);
        (assign790_e776,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign790_e778;
        locals.var_w_um_rv = 0.0;

        let (assign800_e793,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 != 0.0)) {
        let assign800_e791: f64 = (locals.var_w_um + p.p22);
        (assign800_e791,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign800_e793;
        locals.var_weff_um_rv = 0.0;

        let (assign830_e835,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) {
        let assign830_e833: f64 = (p.p0 * locals.var_lfactor);
        (assign830_e833,)
    } else {
        (locals.var_w_um,)
    }
};
        locals.var_w_um = assign830_e835;
        locals.var_w_um_rv = 0.0;

        let (assign840_e851,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) {
        let assign840_e849: f64 = (locals.var_w_um + p.p22);
        (assign840_e849,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign840_e851;
        locals.var_weff_um_rv = 0.0;

        let (assign860_e870,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) {
        let assign860_e868: f64 = (p.p1 * locals.var_lfactor);
        (assign860_e868,)
    } else {
        (locals.var_l_um,)
    }
};
        locals.var_l_um = assign860_e870;
        locals.var_l_um_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let (assign870_e886,) = {
    if ((((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 == 0.0)) && (locals.var_guard60 == 0.0)) {
        let assign870_e884: f64 = (locals.var_l_um + locals.var_xleff);
        (assign870_e884,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign870_e886;
        locals.var_leff_um_rv = 0.0;

        let assign1190_e1146: f64 = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard75 = assign1190_e1146;
        locals.var_guard75_rv = 0.0;

        let (assign1200_e1154,) = {
    if (locals.var_guard75 != 0.0) {
        let assign1200_e1151: f64 = (locals.var_l_um + locals.var_w_um);
        let assign1200_e1152: f64 = (2.0 * assign1200_e1151);
        (assign1200_e1152,)
    } else {
        (locals.var_p_um,)
    }
};
        locals.var_p_um = assign1200_e1154;
        locals.var_p_um_rv = 0.0;

        let assign1210_e1157: f64 = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard76 = assign1210_e1157;
        locals.var_guard76_rv = 0.0;

        let (assign1220_e1168,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard76 != 0.0)) {
        let assign1220_e1164: f64 = (2.0 * locals.var_l_um);
        let assign1220_e1166: f64 = (assign1220_e1164 + locals.var_w_um);
        (assign1220_e1166,)
    } else {
        (locals.var_p_um,)
    }
};
        locals.var_p_um = assign1220_e1168;
        locals.var_p_um_rv = 0.0;

        let (assign1230_e1178,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard76 == 0.0)) {
        let assign1230_e1176: f64 = (2.0 * locals.var_l_um);
        (assign1230_e1176,)
    } else {
        (locals.var_p_um,)
    }
};
        locals.var_p_um = assign1230_e1178;
        locals.var_p_um_rv = 0.0;

        let assign1240_e1181: f64 = (locals.var_l_um * locals.var_w_um);
        locals.var_a_um2 = assign1240_e1181;
        locals.var_a_um2_rv = 0.0;

        let assign1260_e1194: f64 = (p.p48 * locals.var_p_um);
        let assign1260_e1195: f64 = (p.p47 + assign1260_e1194);
        let assign1260_e1198: f64 = (p.p49 * locals.var_a_um2);
        let assign1260_e1199: f64 = (assign1260_e1195 + assign1260_e1198);
        locals.var_cth = assign1260_e1199;
        locals.var_cth_rv = 0.0;

        locals.var_vrth = (nv2 - 0.0);
        locals.var_vrth_dn2 = 1.0;
        locals.var_vrth_rv = 0.0;

        let assign1620_e1409: f64 = (locals.var_vrth * locals.var_cth);
        locals.var_qcth = assign1620_e1409;
        locals.var_qcth_dn2 = (locals.var_vrth_dn2 * locals.var_cth);
        locals.var_qcth_rv = 0.0;

    }
}
