#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
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

        let (assign730_e714,) = {
    if (((locals.var_guard48 == 0.0) && (locals.var_guard53 == 0.0)) && (locals.var_guard59 != 0.0)) {
        let assign730_e712: f64 = (locals.var_l_um + locals.var_xleff);
        (assign730_e712,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign730_e714;

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

        let assign1260_e1194: f64 = (p.p48 * locals.var_p_um);
        let assign1260_e1195: f64 = (p.p47 + assign1260_e1194);
        let assign1260_e1198: f64 = (p.p49 * locals.var_a_um2);
        let assign1260_e1199: f64 = (assign1260_e1195 + assign1260_e1198);
        locals.var_cth = assign1260_e1199;

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        locals.var_vrth = (nv2 - 0.0);
        locals.var_vrth_dn2 = 1.0;

        let assign1620_e1409: f64 = (locals.var_vrth * locals.var_cth);
        locals.var_qcth = assign1620_e1409;
        locals.var_qcth_dn2 = (locals.var_vrth_dn2 * locals.var_cth);

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
