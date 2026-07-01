#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25130_e25901, assign25130_e25901_d_n4, assign25130_e25901_d_n6, assign25130_e25901_d_n7, assign25130_e25901_d_n8, assign25130_e25901_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard717 == 0.0)) && (locals.var_guard718 == 0.0)) {
        let assign25130_e25877: f64 = (-locals.var_sp_ov_xbar);
        let assign25130_e25879: f64 = (assign25130_e25877 - 80.0);
        let assign25130_e25883: f64 = (-locals.var_sp_ov_xbar);
        let assign25130_e25885: f64 = (assign25130_e25883 - 80.0);
        let assign25130_e25886: f64 = (0.5 * assign25130_e25885);
        let assign25130_e25889: f64 = (-locals.var_sp_ov_xbar);
        let assign25130_e25891: f64 = (assign25130_e25889 - 80.0);
        let assign25130_e25893: f64 = (assign25130_e25891 * 0.3333333333333);
        let assign25130_e25894: f64 = (1.0 + assign25130_e25893);
        let assign25130_e25895: f64 = (assign25130_e25886 * assign25130_e25894);
        let assign25130_e25896: f64 = (1.0 + assign25130_e25895);
        let assign25130_e25897: f64 = (assign25130_e25879 * assign25130_e25896);
        let assign25130_e25898: f64 = (1.0 + assign25130_e25897);
        let assign25130_e25899: f64 = (5.54062e34 * assign25130_e25898);
        (assign25130_e25899, (5.54062e34 * (((-locals.var_sp_ov_xbar_dn4) * assign25130_e25896) + (assign25130_e25879 * (((0.5 * (-locals.var_sp_ov_xbar_dn4)) * assign25130_e25894) + (assign25130_e25886 * ((-locals.var_sp_ov_xbar_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn6) * assign25130_e25896) + (assign25130_e25879 * (((0.5 * (-locals.var_sp_ov_xbar_dn6)) * assign25130_e25894) + (assign25130_e25886 * ((-locals.var_sp_ov_xbar_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn7) * assign25130_e25896) + (assign25130_e25879 * (((0.5 * (-locals.var_sp_ov_xbar_dn7)) * assign25130_e25894) + (assign25130_e25886 * ((-locals.var_sp_ov_xbar_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn8) * assign25130_e25896) + (assign25130_e25879 * (((0.5 * (-locals.var_sp_ov_xbar_dn8)) * assign25130_e25894) + (assign25130_e25886 * ((-locals.var_sp_ov_xbar_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn9) * assign25130_e25896) + (assign25130_e25879 * (((0.5 * (-locals.var_sp_ov_xbar_dn9)) * assign25130_e25894) + (assign25130_e25886 * ((-locals.var_sp_ov_xbar_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25130_e25901;
        locals.var_sp_ov_temp_dn4 = assign25130_e25901_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25130_e25901_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25130_e25901_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25130_e25901_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25130_e25901_d_n9;

        let (assign25140_e25913, assign25140_e25913_d_n4, assign25140_e25913_d_n6, assign25140_e25913_d_n7, assign25140_e25913_d_n8, assign25140_e25913_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25140_e25911: f64 = (1.0 - locals.var_sp_ov_temp);
        (assign25140_e25911, (-locals.var_sp_ov_temp_dn4), (-locals.var_sp_ov_temp_dn6), (-locals.var_sp_ov_temp_dn7), (-locals.var_sp_ov_temp_dn8), (-locals.var_sp_ov_temp_dn9),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign25140_e25913;
        locals.var_sp_ov_w_dn4 = assign25140_e25913_d_n4;
        locals.var_sp_ov_w_dn6 = assign25140_e25913_d_n6;
        locals.var_sp_ov_w_dn7 = assign25140_e25913_d_n7;
        locals.var_sp_ov_w_dn8 = assign25140_e25913_d_n8;
        locals.var_sp_ov_w_dn9 = assign25140_e25913_d_n9;

        let (assign25150_e25938, assign25150_e25938_d_n4, assign25150_e25938_d_n6, assign25150_e25938_d_n7, assign25150_e25938_d_n8, assign25150_e25938_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25150_e25924: f64 = (locals.var_gov2 * 0.5);
        let assign25150_e25925: f64 = (locals.var_xgd_ovcv + assign25150_e25924);
        let assign25150_e25930: f64 = (locals.var_gov2 * 0.25);
        let assign25150_e25931: f64 = (locals.var_xgd_ovcv + assign25150_e25930);
        let assign25150_e25933: f64 = (assign25150_e25931 - locals.var_sp_ov_w);
        let assign25150_e25934: f64 = (assign25150_e25933).sqrt();
        let assign25150_e25935: f64 = (locals.var_gov * assign25150_e25934);
        let assign25150_e25936: f64 = (assign25150_e25925 - assign25150_e25935);
        (assign25150_e25936, ((locals.var_xgd_ovcv_dn4 + (locals.var_gov2_dn4 * 0.5)) - ((locals.var_gov_dn4 * assign25150_e25934) + (locals.var_gov * (((locals.var_xgd_ovcv_dn4 + (locals.var_gov2_dn4 * 0.25)) - locals.var_sp_ov_w_dn4) / (2.0 * assign25150_e25934))))), ((locals.var_xgd_ovcv_dn6 + (locals.var_gov2_dn6 * 0.5)) - ((locals.var_gov_dn6 * assign25150_e25934) + (locals.var_gov * (((locals.var_xgd_ovcv_dn6 + (locals.var_gov2_dn6 * 0.25)) - locals.var_sp_ov_w_dn6) / (2.0 * assign25150_e25934))))), ((locals.var_xgd_ovcv_dn7 + (locals.var_gov2_dn7 * 0.5)) - ((locals.var_gov_dn7 * assign25150_e25934) + (locals.var_gov * (((locals.var_xgd_ovcv_dn7 + (locals.var_gov2_dn7 * 0.25)) - locals.var_sp_ov_w_dn7) / (2.0 * assign25150_e25934))))), ((locals.var_xgd_ovcv_dn8 + (locals.var_gov2_dn8 * 0.5)) - ((locals.var_gov_dn8 * assign25150_e25934) + (locals.var_gov * (((locals.var_xgd_ovcv_dn8 + (locals.var_gov2_dn8 * 0.25)) - locals.var_sp_ov_w_dn8) / (2.0 * assign25150_e25934))))), ((locals.var_xgd_ovcv_dn9 + (locals.var_gov2_dn9 * 0.5)) - ((locals.var_gov_dn9 * assign25150_e25934) + (locals.var_gov * (((locals.var_xgd_ovcv_dn9 + (locals.var_gov2_dn9 * 0.25)) - locals.var_sp_ov_w_dn9) / (2.0 * assign25150_e25934))))),)
    } else {
        (locals.var_sp_ov_x0, locals.var_sp_ov_x0_dn4, locals.var_sp_ov_x0_dn6, locals.var_sp_ov_x0_dn7, locals.var_sp_ov_x0_dn8, locals.var_sp_ov_x0_dn9,)
    }
};
        locals.var_sp_ov_x0 = assign25150_e25938;
        locals.var_sp_ov_x0_dn4 = assign25150_e25938_d_n4;
        locals.var_sp_ov_x0_dn6 = assign25150_e25938_d_n6;
        locals.var_sp_ov_x0_dn7 = assign25150_e25938_d_n7;
        locals.var_sp_ov_x0_dn8 = assign25150_e25938_d_n8;
        locals.var_sp_ov_x0_dn9 = assign25150_e25938_d_n9;

        let assign25160_e25940: f64 = (-locals.var_sp_ov_x0);
        let assign25160_e25941: f64 = (assign25160_e25940).abs();
        let assign25160_e25943: f64 = if assign25160_e25941 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard719 = assign25160_e25943;

        let (assign25170_e25957, assign25170_e25957_d_n4, assign25170_e25957_d_n6, assign25170_e25957_d_n7, assign25170_e25957_d_n8, assign25170_e25957_d_n9,) = {
    if ((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard719 != 0.0)) {
        let assign25170_e25954: f64 = (-locals.var_sp_ov_x0);
        let assign25170_e25955: f64 = (assign25170_e25954).exp();
        (assign25170_e25955, (assign25170_e25955 * (-locals.var_sp_ov_x0_dn4)), (assign25170_e25955 * (-locals.var_sp_ov_x0_dn6)), (assign25170_e25955 * (-locals.var_sp_ov_x0_dn7)), (assign25170_e25955 * (-locals.var_sp_ov_x0_dn8)), (assign25170_e25955 * (-locals.var_sp_ov_x0_dn9)),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25170_e25957;
        locals.var_sp_ov_d0_dn4 = assign25170_e25957_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25170_e25957_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25170_e25957_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25170_e25957_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25170_e25957_d_n9;

        let assign25180_e25959: f64 = (-locals.var_sp_ov_x0);
        let assign25180_e25961: f64 = (-80.0);
        let assign25180_e25962: f64 = if assign25180_e25959 < assign25180_e25961 { 1.0 } else { 0.0 };
        locals.var_guard720 = assign25180_e25962;

        let (assign25190_e26005, assign25190_e26005_d_n4, assign25190_e26005_d_n6, assign25190_e26005_d_n7, assign25190_e26005_d_n8, assign25190_e26005_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 != 0.0)) {
        let assign25190_e25978: f64 = (-locals.var_sp_ov_x0);
        let assign25190_e25979: f64 = (-assign25190_e25978);
        let assign25190_e25981: f64 = (assign25190_e25979 - 80.0);
        let assign25190_e25985: f64 = (-locals.var_sp_ov_x0);
        let assign25190_e25986: f64 = (-assign25190_e25985);
        let assign25190_e25988: f64 = (assign25190_e25986 - 80.0);
        let assign25190_e25989: f64 = (0.5 * assign25190_e25988);
        let assign25190_e25992: f64 = (-locals.var_sp_ov_x0);
        let assign25190_e25993: f64 = (-assign25190_e25992);
        let assign25190_e25995: f64 = (assign25190_e25993 - 80.0);
        let assign25190_e25997: f64 = (assign25190_e25995 * 0.3333333333333);
        let assign25190_e25998: f64 = (1.0 + assign25190_e25997);
        let assign25190_e25999: f64 = (assign25190_e25989 * assign25190_e25998);
        let assign25190_e26000: f64 = (1.0 + assign25190_e25999);
        let assign25190_e26001: f64 = (assign25190_e25981 * assign25190_e26000);
        let assign25190_e26002: f64 = (1.0 + assign25190_e26001);
        let assign25190_e26003: f64 = (1.80485e-35 / assign25190_e26002);
        (assign25190_e26003, (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn4)) * assign25190_e26000) + (assign25190_e25981 * (((0.5 * (-(-locals.var_sp_ov_x0_dn4))) * assign25190_e25998) + (assign25190_e25989 * ((-(-locals.var_sp_ov_x0_dn4)) * 0.3333333333333)))))) / (assign25190_e26002 * assign25190_e26002))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn6)) * assign25190_e26000) + (assign25190_e25981 * (((0.5 * (-(-locals.var_sp_ov_x0_dn6))) * assign25190_e25998) + (assign25190_e25989 * ((-(-locals.var_sp_ov_x0_dn6)) * 0.3333333333333)))))) / (assign25190_e26002 * assign25190_e26002))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn7)) * assign25190_e26000) + (assign25190_e25981 * (((0.5 * (-(-locals.var_sp_ov_x0_dn7))) * assign25190_e25998) + (assign25190_e25989 * ((-(-locals.var_sp_ov_x0_dn7)) * 0.3333333333333)))))) / (assign25190_e26002 * assign25190_e26002))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn8)) * assign25190_e26000) + (assign25190_e25981 * (((0.5 * (-(-locals.var_sp_ov_x0_dn8))) * assign25190_e25998) + (assign25190_e25989 * ((-(-locals.var_sp_ov_x0_dn8)) * 0.3333333333333)))))) / (assign25190_e26002 * assign25190_e26002))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn9)) * assign25190_e26000) + (assign25190_e25981 * (((0.5 * (-(-locals.var_sp_ov_x0_dn9))) * assign25190_e25998) + (assign25190_e25989 * ((-(-locals.var_sp_ov_x0_dn9)) * 0.3333333333333)))))) / (assign25190_e26002 * assign25190_e26002))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25190_e26005;
        locals.var_sp_ov_d0_dn4 = assign25190_e26005_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25190_e26005_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25190_e26005_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25190_e26005_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25190_e26005_d_n9;

        let (assign25200_e26046, assign25200_e26046_d_n4, assign25200_e26046_d_n6, assign25200_e26046_d_n7, assign25200_e26046_d_n8, assign25200_e26046_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 == 0.0)) {
        let assign25200_e26022: f64 = (-locals.var_sp_ov_x0);
        let assign25200_e26024: f64 = (assign25200_e26022 - 80.0);
        let assign25200_e26028: f64 = (-locals.var_sp_ov_x0);
        let assign25200_e26030: f64 = (assign25200_e26028 - 80.0);
        let assign25200_e26031: f64 = (0.5 * assign25200_e26030);
        let assign25200_e26034: f64 = (-locals.var_sp_ov_x0);
        let assign25200_e26036: f64 = (assign25200_e26034 - 80.0);
        let assign25200_e26038: f64 = (assign25200_e26036 * 0.3333333333333);
        let assign25200_e26039: f64 = (1.0 + assign25200_e26038);
        let assign25200_e26040: f64 = (assign25200_e26031 * assign25200_e26039);
        let assign25200_e26041: f64 = (1.0 + assign25200_e26040);
        let assign25200_e26042: f64 = (assign25200_e26024 * assign25200_e26041);
        let assign25200_e26043: f64 = (1.0 + assign25200_e26042);
        let assign25200_e26044: f64 = (5.54062e34 * assign25200_e26043);
        (assign25200_e26044, (5.54062e34 * (((-locals.var_sp_ov_x0_dn4) * assign25200_e26041) + (assign25200_e26024 * (((0.5 * (-locals.var_sp_ov_x0_dn4)) * assign25200_e26039) + (assign25200_e26031 * ((-locals.var_sp_ov_x0_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn6) * assign25200_e26041) + (assign25200_e26024 * (((0.5 * (-locals.var_sp_ov_x0_dn6)) * assign25200_e26039) + (assign25200_e26031 * ((-locals.var_sp_ov_x0_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn7) * assign25200_e26041) + (assign25200_e26024 * (((0.5 * (-locals.var_sp_ov_x0_dn7)) * assign25200_e26039) + (assign25200_e26031 * ((-locals.var_sp_ov_x0_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn8) * assign25200_e26041) + (assign25200_e26024 * (((0.5 * (-locals.var_sp_ov_x0_dn8)) * assign25200_e26039) + (assign25200_e26031 * ((-locals.var_sp_ov_x0_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn9) * assign25200_e26041) + (assign25200_e26024 * (((0.5 * (-locals.var_sp_ov_x0_dn9)) * assign25200_e26039) + (assign25200_e26031 * ((-locals.var_sp_ov_x0_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25200_e26046;
        locals.var_sp_ov_d0_dn4 = assign25200_e26046_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25200_e26046_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25200_e26046_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25200_e26046_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25200_e26046_d_n9;

        let (assign25210_e26066, assign25210_e26066_d_n4, assign25210_e26066_d_n6, assign25210_e26066_d_n7, assign25210_e26066_d_n8, assign25210_e26066_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25210_e26057: f64 = (locals.var_xgd_ovcv - locals.var_sp_ov_x0);
        let assign25210_e26058: f64 = (2.0 * assign25210_e26057);
        let assign25210_e26062: f64 = (1.0 - locals.var_sp_ov_d0);
        let assign25210_e26063: f64 = (locals.var_gov2 * assign25210_e26062);
        let assign25210_e26064: f64 = (assign25210_e26058 + assign25210_e26063);
        (assign25210_e26064, ((2.0 * (locals.var_xgd_ovcv_dn4 - locals.var_sp_ov_x0_dn4)) + ((locals.var_gov2_dn4 * assign25210_e26062) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn4)))), ((2.0 * (locals.var_xgd_ovcv_dn6 - locals.var_sp_ov_x0_dn6)) + ((locals.var_gov2_dn6 * assign25210_e26062) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn6)))), ((2.0 * (locals.var_xgd_ovcv_dn7 - locals.var_sp_ov_x0_dn7)) + ((locals.var_gov2_dn7 * assign25210_e26062) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn7)))), ((2.0 * (locals.var_xgd_ovcv_dn8 - locals.var_sp_ov_x0_dn8)) + ((locals.var_gov2_dn8 * assign25210_e26062) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn8)))), ((2.0 * (locals.var_xgd_ovcv_dn9 - locals.var_sp_ov_x0_dn9)) + ((locals.var_gov2_dn9 * assign25210_e26062) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign25210_e26066;
        locals.var_sp_ov_p_dn4 = assign25210_e26066_d_n4;
        locals.var_sp_ov_p_dn6 = assign25210_e26066_d_n6;
        locals.var_sp_ov_p_dn7 = assign25210_e26066_d_n7;
        locals.var_sp_ov_p_dn8 = assign25210_e26066_d_n8;
        locals.var_sp_ov_p_dn9 = assign25210_e26066_d_n9;

        let (assign25220_e26090, assign25220_e26090_d_n4, assign25220_e26090_d_n6, assign25220_e26090_d_n7, assign25220_e26090_d_n8, assign25220_e26090_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25220_e26076: f64 = (locals.var_xgd_ovcv - locals.var_sp_ov_x0);
        let assign25220_e26079: f64 = (locals.var_xgd_ovcv - locals.var_sp_ov_x0);
        let assign25220_e26080: f64 = (assign25220_e26076 * assign25220_e26079);
        let assign25220_e26084: f64 = (locals.var_sp_ov_x0 - 1.0);
        let assign25220_e26086: f64 = (assign25220_e26084 + locals.var_sp_ov_d0);
        let assign25220_e26087: f64 = (locals.var_gov2 * assign25220_e26086);
        let assign25220_e26088: f64 = (assign25220_e26080 - assign25220_e26087);
        (assign25220_e26088, ((((locals.var_xgd_ovcv_dn4 - locals.var_sp_ov_x0_dn4) * assign25220_e26079) + (assign25220_e26076 * (locals.var_xgd_ovcv_dn4 - locals.var_sp_ov_x0_dn4))) - ((locals.var_gov2_dn4 * assign25220_e26086) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_d0_dn4)))), ((((locals.var_xgd_ovcv_dn6 - locals.var_sp_ov_x0_dn6) * assign25220_e26079) + (assign25220_e26076 * (locals.var_xgd_ovcv_dn6 - locals.var_sp_ov_x0_dn6))) - ((locals.var_gov2_dn6 * assign25220_e26086) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_d0_dn6)))), ((((locals.var_xgd_ovcv_dn7 - locals.var_sp_ov_x0_dn7) * assign25220_e26079) + (assign25220_e26076 * (locals.var_xgd_ovcv_dn7 - locals.var_sp_ov_x0_dn7))) - ((locals.var_gov2_dn7 * assign25220_e26086) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_d0_dn7)))), ((((locals.var_xgd_ovcv_dn8 - locals.var_sp_ov_x0_dn8) * assign25220_e26079) + (assign25220_e26076 * (locals.var_xgd_ovcv_dn8 - locals.var_sp_ov_x0_dn8))) - ((locals.var_gov2_dn8 * assign25220_e26086) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_d0_dn8)))), ((((locals.var_xgd_ovcv_dn9 - locals.var_sp_ov_x0_dn9) * assign25220_e26079) + (assign25220_e26076 * (locals.var_xgd_ovcv_dn9 - locals.var_sp_ov_x0_dn9))) - ((locals.var_gov2_dn9 * assign25220_e26086) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign25220_e26090;
        locals.var_sp_ov_q_dn4 = assign25220_e26090_d_n4;
        locals.var_sp_ov_q_dn6 = assign25220_e26090_d_n6;
        locals.var_sp_ov_q_dn7 = assign25220_e26090_d_n7;
        locals.var_sp_ov_q_dn8 = assign25220_e26090_d_n8;
        locals.var_sp_ov_q_dn9 = assign25220_e26090_d_n9;

        let (assign25230_e26106, assign25230_e26106_d_n4, assign25230_e26106_d_n6, assign25230_e26106_d_n7, assign25230_e26106_d_n8, assign25230_e26106_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25230_e26101: f64 = (locals.var_gov2 * 0.5);
        let assign25230_e26103: f64 = (assign25230_e26101 * locals.var_sp_ov_d0);
        let assign25230_e26104: f64 = (1.0 - assign25230_e26103);
        (assign25230_e26104, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign25230_e26101 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign25230_e26101 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign25230_e26101 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign25230_e26101 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign25230_e26101 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign25230_e26106;
        locals.var_sp_ov_xi_dn4 = assign25230_e26106_d_n4;
        locals.var_sp_ov_xi_dn6 = assign25230_e26106_d_n6;
        locals.var_sp_ov_xi_dn7 = assign25230_e26106_d_n7;
        locals.var_sp_ov_xi_dn8 = assign25230_e26106_d_n8;
        locals.var_sp_ov_xi_dn9 = assign25230_e26106_d_n9;

        let (assign25240_e26124, assign25240_e26124_d_n4, assign25240_e26124_d_n6, assign25240_e26124_d_n7, assign25240_e26124_d_n8, assign25240_e26124_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25240_e26116: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign25240_e26120: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign25240_e26121: f64 = (4.0 * assign25240_e26120);
        let assign25240_e26122: f64 = (assign25240_e26116 - assign25240_e26121);
        (assign25240_e26122, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25240_e26124;
        locals.var_sp_ov_temp_dn4 = assign25240_e26124_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25240_e26124_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25240_e26124_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25240_e26124_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25240_e26124_d_n9;

        let (assign25250_e26141, assign25250_e26141_d_n4, assign25250_e26141_d_n6, assign25250_e26141_d_n7, assign25250_e26141_d_n8, assign25250_e26141_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25250_e26134: f64 = (2.0 * locals.var_sp_ov_q);
        let assign25250_e26137: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign25250_e26138: f64 = (locals.var_sp_ov_p + assign25250_e26137);
        let assign25250_e26139: f64 = (assign25250_e26134 / assign25250_e26138);
        (assign25250_e26139, ((((2.0 * locals.var_sp_ov_q_dn4) * assign25250_e26138) - (assign25250_e26134 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign25250_e26137))))) / (assign25250_e26138 * assign25250_e26138)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign25250_e26138) - (assign25250_e26134 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign25250_e26137))))) / (assign25250_e26138 * assign25250_e26138)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign25250_e26138) - (assign25250_e26134 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign25250_e26137))))) / (assign25250_e26138 * assign25250_e26138)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign25250_e26138) - (assign25250_e26134 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign25250_e26137))))) / (assign25250_e26138 * assign25250_e26138)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign25250_e26138) - (assign25250_e26134 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign25250_e26137))))) / (assign25250_e26138 * assign25250_e26138)),)
    } else {
        (locals.var_sp_ov_u, locals.var_sp_ov_u_dn4, locals.var_sp_ov_u_dn6, locals.var_sp_ov_u_dn7, locals.var_sp_ov_u_dn8, locals.var_sp_ov_u_dn9,)
    }
};
        locals.var_sp_ov_u = assign25250_e26141;
        locals.var_sp_ov_u_dn4 = assign25250_e26141_d_n4;
        locals.var_sp_ov_u_dn6 = assign25250_e26141_d_n6;
        locals.var_sp_ov_u_dn7 = assign25250_e26141_d_n7;
        locals.var_sp_ov_u_dn8 = assign25250_e26141_d_n8;
        locals.var_sp_ov_u_dn9 = assign25250_e26141_d_n9;

        let (assign25260_e26153, assign25260_e26153_d_n4, assign25260_e26153_d_n6, assign25260_e26153_d_n7, assign25260_e26153_d_n8, assign25260_e26153_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25260_e26151: f64 = (locals.var_sp_ov_x0 + locals.var_sp_ov_u);
        (assign25260_e26151, (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_u_dn4), (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_u_dn6), (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_u_dn7), (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_u_dn8), (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_u_dn9),)
    } else {
        (locals.var_xd_ovcv, locals.var_xd_ovcv_dn4, locals.var_xd_ovcv_dn6, locals.var_xd_ovcv_dn7, locals.var_xd_ovcv_dn8, locals.var_xd_ovcv_dn9,)
    }
};
        locals.var_xd_ovcv = assign25260_e26153;
        locals.var_xd_ovcv_dn4 = assign25260_e26153_d_n4;
        locals.var_xd_ovcv_dn6 = assign25260_e26153_d_n6;
        locals.var_xd_ovcv_dn7 = assign25260_e26153_d_n7;
        locals.var_xd_ovcv_dn8 = assign25260_e26153_d_n8;
        locals.var_xd_ovcv_dn9 = assign25260_e26153_d_n9;

        let (assign25270_e26161, assign25270_e26161_d_n4, assign25270_e26161_d_n6, assign25270_e26161_d_n7, assign25270_e26161_d_n8, assign25270_e26161_d_n9,) = {
    if ((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) {
        let assign25270_e26159: f64 = (-locals.var_xd_ovcv);
        (assign25270_e26159, (-locals.var_xd_ovcv_dn4), (-locals.var_xd_ovcv_dn6), (-locals.var_xd_ovcv_dn7), (-locals.var_xd_ovcv_dn8), (-locals.var_xd_ovcv_dn9),)
    } else {
        (locals.var_xd_ovcv, locals.var_xd_ovcv_dn4, locals.var_xd_ovcv_dn6, locals.var_xd_ovcv_dn7, locals.var_xd_ovcv_dn8, locals.var_xd_ovcv_dn9,)
    }
};
        locals.var_xd_ovcv = assign25270_e26161;
        locals.var_xd_ovcv_dn4 = assign25270_e26161_d_n4;
        locals.var_xd_ovcv_dn6 = assign25270_e26161_d_n6;
        locals.var_xd_ovcv_dn7 = assign25270_e26161_d_n7;
        locals.var_xd_ovcv_dn8 = assign25270_e26161_d_n8;
        locals.var_xd_ovcv_dn9 = assign25270_e26161_d_n9;

        let assign25280_e26163: f64 = (-locals.var_phit0);
        let assign25280_e26166: f64 = (locals.var_xgs_ov + locals.var_xs_ov);
        let assign25280_e26167: f64 = (assign25280_e26163 * assign25280_e26166);
        locals.var_vovs = assign25280_e26167;
        locals.var_vovs_dn4 = (((-locals.var_phit0_dn4) * assign25280_e26166) + (assign25280_e26163 * (locals.var_xgs_ov_dn4 + locals.var_xs_ov_dn4)));
        locals.var_vovs_dn6 = (((-locals.var_phit0_dn6) * assign25280_e26166) + (assign25280_e26163 * (locals.var_xgs_ov_dn6 + locals.var_xs_ov_dn6)));
        locals.var_vovs_dn7 = (((-locals.var_phit0_dn7) * assign25280_e26166) + (assign25280_e26163 * (locals.var_xgs_ov_dn7 + locals.var_xs_ov_dn7)));
        locals.var_vovs_dn8 = (((-locals.var_phit0_dn8) * assign25280_e26166) + (assign25280_e26163 * (locals.var_xgs_ov_dn8 + locals.var_xs_ov_dn8)));
        locals.var_vovs_dn9 = (((-locals.var_phit0_dn9) * assign25280_e26166) + (assign25280_e26163 * (locals.var_xgs_ov_dn9 + locals.var_xs_ov_dn9)));

        let assign25290_e26169: f64 = (-locals.var_phit0);
        let assign25290_e26172: f64 = (locals.var_xgd_ov + locals.var_xd_ov);
        let assign25290_e26173: f64 = (assign25290_e26169 * assign25290_e26172);
        locals.var_vovd = assign25290_e26173;
        locals.var_vovd_dn4 = (((-locals.var_phit0_dn4) * assign25290_e26172) + (assign25290_e26169 * (locals.var_xgd_ov_dn4 + locals.var_xd_ov_dn4)));
        locals.var_vovd_dn6 = (((-locals.var_phit0_dn6) * assign25290_e26172) + (assign25290_e26169 * (locals.var_xgd_ov_dn6 + locals.var_xd_ov_dn6)));
        locals.var_vovd_dn7 = (((-locals.var_phit0_dn7) * assign25290_e26172) + (assign25290_e26169 * (locals.var_xgd_ov_dn7 + locals.var_xd_ov_dn7)));
        locals.var_vovd_dn8 = (((-locals.var_phit0_dn8) * assign25290_e26172) + (assign25290_e26169 * (locals.var_xgd_ov_dn8 + locals.var_xd_ov_dn8)));
        locals.var_vovd_dn9 = (((-locals.var_phit0_dn9) * assign25290_e26172) + (assign25290_e26169 * (locals.var_xgd_ov_dn9 + locals.var_xd_ov_dn9)));

        let assign25300_e26175: f64 = (-locals.var_phit0);
        let assign25300_e26178: f64 = (locals.var_xgs_ovcv + locals.var_xs_ovcv);
        let assign25300_e26179: f64 = (assign25300_e26175 * assign25300_e26178);
        locals.var_vovscv = assign25300_e26179;
        locals.var_vovscv_dn4 = (((-locals.var_phit0_dn4) * assign25300_e26178) + (assign25300_e26175 * (locals.var_xgs_ovcv_dn4 + locals.var_xs_ovcv_dn4)));
        locals.var_vovscv_dn6 = (((-locals.var_phit0_dn6) * assign25300_e26178) + (assign25300_e26175 * (locals.var_xgs_ovcv_dn6 + locals.var_xs_ovcv_dn6)));
        locals.var_vovscv_dn7 = (((-locals.var_phit0_dn7) * assign25300_e26178) + (assign25300_e26175 * (locals.var_xgs_ovcv_dn7 + locals.var_xs_ovcv_dn7)));
        locals.var_vovscv_dn8 = (((-locals.var_phit0_dn8) * assign25300_e26178) + (assign25300_e26175 * (locals.var_xgs_ovcv_dn8 + locals.var_xs_ovcv_dn8)));
        locals.var_vovscv_dn9 = (((-locals.var_phit0_dn9) * assign25300_e26178) + (assign25300_e26175 * (locals.var_xgs_ovcv_dn9 + locals.var_xs_ovcv_dn9)));

        let assign25310_e26181: f64 = (-locals.var_phit0);
        let assign25310_e26184: f64 = (locals.var_xgd_ovcv + locals.var_xd_ovcv);
        let assign25310_e26185: f64 = (assign25310_e26181 * assign25310_e26184);
        locals.var_vovdcv = assign25310_e26185;
        locals.var_vovdcv_dn4 = (((-locals.var_phit0_dn4) * assign25310_e26184) + (assign25310_e26181 * (locals.var_xgd_ovcv_dn4 + locals.var_xd_ovcv_dn4)));
        locals.var_vovdcv_dn6 = (((-locals.var_phit0_dn6) * assign25310_e26184) + (assign25310_e26181 * (locals.var_xgd_ovcv_dn6 + locals.var_xd_ovcv_dn6)));
        locals.var_vovdcv_dn7 = (((-locals.var_phit0_dn7) * assign25310_e26184) + (assign25310_e26181 * (locals.var_xgd_ovcv_dn7 + locals.var_xd_ovcv_dn7)));
        locals.var_vovdcv_dn8 = (((-locals.var_phit0_dn8) * assign25310_e26184) + (assign25310_e26181 * (locals.var_xgd_ovcv_dn8 + locals.var_xd_ovcv_dn8)));
        locals.var_vovdcv_dn9 = (((-locals.var_phit0_dn9) * assign25310_e26184) + (assign25310_e26181 * (locals.var_xgd_ovcv_dn9 + locals.var_xd_ovcv_dn9)));

        let assign25390_e26195: f64 = if p.p3 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard721 = assign25390_e26195;

        let assign25400_e26202: f64 = if ((locals.var_igovinv_i > 0.0) || (locals.var_igovacc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard722 = assign25400_e26202;

        let (assign25410_e26210, assign25410_e26210_d_n4, assign25410_e26210_d_n6, assign25410_e26210_d_n7, assign25410_e26210_d_n8, assign25410_e26210_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25410_e26208: f64 = (locals.var_vovs + locals.var_dov);
        (assign25410_e26208, (locals.var_vovs_dn4 + locals.var_dov_dn4), (locals.var_vovs_dn6 + locals.var_dov_dn6), (locals.var_vovs_dn7 + locals.var_dov_dn7), (locals.var_vovs_dn8 + locals.var_dov_dn8), (locals.var_vovs_dn9 + locals.var_dov_dn9),)
    } else {
        (locals.var_arg2mina, locals.var_arg2mina_dn4, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8, locals.var_arg2mina_dn9,)
    }
};
        locals.var_arg2mina = assign25410_e26210;
        locals.var_arg2mina_dn4 = assign25410_e26210_d_n4;
        locals.var_arg2mina_dn6 = assign25410_e26210_d_n6;
        locals.var_arg2mina_dn7 = assign25410_e26210_d_n7;
        locals.var_arg2mina_dn8 = assign25410_e26210_d_n8;
        locals.var_arg2mina_dn9 = assign25410_e26210_d_n9;

        let (assign25420_e26231, assign25420_e26231_d_n4, assign25420_e26231_d_n6, assign25420_e26231_d_n7, assign25420_e26231_d_n8, assign25420_e26231_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25420_e26217: f64 = locals.var_arg2mina;
        let assign25420_e26220: f64 = (-locals.var_arg2mina);
        let assign25420_e26223: f64 = (-locals.var_arg2mina);
        let assign25420_e26224: f64 = (assign25420_e26220 * assign25420_e26223);
        let assign25420_e26226: f64 = (assign25420_e26224 + 0.01);
        let assign25420_e26227: f64 = (assign25420_e26226).sqrt();
        let assign25420_e26228: f64 = (assign25420_e26217 - assign25420_e26227);
        let assign25420_e26229: f64 = (0.5 * assign25420_e26228);
        (assign25420_e26229, (0.5 * (locals.var_arg2mina_dn4 - ((((-locals.var_arg2mina_dn4) * assign25420_e26223) + (assign25420_e26220 * (-locals.var_arg2mina_dn4))) / (2.0 * assign25420_e26227)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign25420_e26223) + (assign25420_e26220 * (-locals.var_arg2mina_dn6))) / (2.0 * assign25420_e26227)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign25420_e26223) + (assign25420_e26220 * (-locals.var_arg2mina_dn7))) / (2.0 * assign25420_e26227)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign25420_e26223) + (assign25420_e26220 * (-locals.var_arg2mina_dn8))) / (2.0 * assign25420_e26227)))), (0.5 * (locals.var_arg2mina_dn9 - ((((-locals.var_arg2mina_dn9) * assign25420_e26223) + (assign25420_e26220 * (-locals.var_arg2mina_dn9))) / (2.0 * assign25420_e26227)))),)
    } else {
        (locals.var_psi_t, locals.var_psi_t_dn4, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8, locals.var_psi_t_dn9,)
    }
};
        locals.var_psi_t = assign25420_e26231;
        locals.var_psi_t_dn4 = assign25420_e26231_d_n4;
        locals.var_psi_t_dn6 = assign25420_e26231_d_n6;
        locals.var_psi_t_dn7 = assign25420_e26231_d_n7;
        locals.var_psi_t_dn8 = assign25420_e26231_d_n8;
        locals.var_psi_t_dn9 = assign25420_e26231_d_n9;

        let (assign25430_e26244, assign25430_e26244_d_n4, assign25430_e26244_d_n6, assign25430_e26244_d_n7, assign25430_e26244_d_n8, assign25430_e26244_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25430_e26237: f64 = (locals.var_vovs * locals.var_vovs);
        let assign25430_e26239: f64 = (assign25430_e26237 + 0.0001);
        let assign25430_e26240: f64 = (assign25430_e26239).sqrt();
        let assign25430_e26242: f64 = (assign25430_e26240 * locals.var_inv_chib);
        (assign25430_e26242, ((((locals.var_vovs_dn4 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn4)) / (2.0 * assign25430_e26240)) * locals.var_inv_chib), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign25430_e26240)) * locals.var_inv_chib), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) / (2.0 * assign25430_e26240)) * locals.var_inv_chib), ((((locals.var_vovs_dn8 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn8)) / (2.0 * assign25430_e26240)) * locals.var_inv_chib), ((((locals.var_vovs_dn9 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn9)) / (2.0 * assign25430_e26240)) * locals.var_inv_chib),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign25430_e26244;
        locals.var_zg_dn4 = assign25430_e26244_d_n4;
        locals.var_zg_dn6 = assign25430_e26244_d_n6;
        locals.var_zg_dn7 = assign25430_e26244_d_n7;
        locals.var_zg_dn8 = assign25430_e26244_d_n8;
        locals.var_zg_dn9 = assign25430_e26244_d_n9;

        let assign25440_e26247: f64 = (0.5 * locals.var_xgs_ov);
        let assign25440_e26248: f64 = (assign25440_e26247).abs();
        let assign25440_e26250: f64 = if assign25440_e26248 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard723 = assign25440_e26250;

        let (assign25450_e26261, assign25450_e26261_d_n4, assign25450_e26261_d_n6, assign25450_e26261_d_n7, assign25450_e26261_d_n8, assign25450_e26261_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign25450_e26258: f64 = (0.5 * locals.var_xgs_ov);
        let assign25450_e26259: f64 = (assign25450_e26258).exp();
        (assign25450_e26259, (assign25450_e26259 * (0.5 * locals.var_xgs_ov_dn4)), (assign25450_e26259 * (0.5 * locals.var_xgs_ov_dn6)), (assign25450_e26259 * (0.5 * locals.var_xgs_ov_dn7)), (assign25450_e26259 * (0.5 * locals.var_xgs_ov_dn8)), (assign25450_e26259 * (0.5 * locals.var_xgs_ov_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25450_e26261;
        locals.var_temp_dn4 = assign25450_e26261_d_n4;
        locals.var_temp_dn6 = assign25450_e26261_d_n6;
        locals.var_temp_dn7 = assign25450_e26261_d_n7;
        locals.var_temp_dn8 = assign25450_e26261_d_n8;
        locals.var_temp_dn9 = assign25450_e26261_d_n9;

        let assign25460_e26264: f64 = (0.5 * locals.var_xgs_ov);
        let assign25460_e26266: f64 = (-80.0);
        let assign25460_e26267: f64 = if assign25460_e26264 < assign25460_e26266 { 1.0 } else { 0.0 };
        locals.var_guard724 = assign25460_e26267;

        let (assign25470_e26309, assign25470_e26309_d_n4, assign25470_e26309_d_n6, assign25470_e26309_d_n7, assign25470_e26309_d_n8, assign25470_e26309_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard723 == 0.0)) && (locals.var_guard724 != 0.0)) {
        let assign25470_e26280: f64 = (0.5 * locals.var_xgs_ov);
        let assign25470_e26281: f64 = (-assign25470_e26280);
        let assign25470_e26283: f64 = (assign25470_e26281 - 80.0);
        let assign25470_e26288: f64 = (0.5 * locals.var_xgs_ov);
        let assign25470_e26289: f64 = (-assign25470_e26288);
        let assign25470_e26291: f64 = (assign25470_e26289 - 80.0);
        let assign25470_e26292: f64 = (0.5 * assign25470_e26291);
        let assign25470_e26296: f64 = (0.5 * locals.var_xgs_ov);
        let assign25470_e26297: f64 = (-assign25470_e26296);
        let assign25470_e26299: f64 = (assign25470_e26297 - 80.0);
        let assign25470_e26301: f64 = (assign25470_e26299 * 0.3333333333333);
        let assign25470_e26302: f64 = (1.0 + assign25470_e26301);
        let assign25470_e26303: f64 = (assign25470_e26292 * assign25470_e26302);
        let assign25470_e26304: f64 = (1.0 + assign25470_e26303);
        let assign25470_e26305: f64 = (assign25470_e26283 * assign25470_e26304);
        let assign25470_e26306: f64 = (1.0 + assign25470_e26305);
        let assign25470_e26307: f64 = (1.80485e-35 / assign25470_e26306);
        (assign25470_e26307, (-((1.80485e-35 * (((-(0.5 * locals.var_xgs_ov_dn4)) * assign25470_e26304) + (assign25470_e26283 * (((0.5 * (-(0.5 * locals.var_xgs_ov_dn4))) * assign25470_e26302) + (assign25470_e26292 * ((-(0.5 * locals.var_xgs_ov_dn4)) * 0.3333333333333)))))) / (assign25470_e26306 * assign25470_e26306))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgs_ov_dn6)) * assign25470_e26304) + (assign25470_e26283 * (((0.5 * (-(0.5 * locals.var_xgs_ov_dn6))) * assign25470_e26302) + (assign25470_e26292 * ((-(0.5 * locals.var_xgs_ov_dn6)) * 0.3333333333333)))))) / (assign25470_e26306 * assign25470_e26306))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgs_ov_dn7)) * assign25470_e26304) + (assign25470_e26283 * (((0.5 * (-(0.5 * locals.var_xgs_ov_dn7))) * assign25470_e26302) + (assign25470_e26292 * ((-(0.5 * locals.var_xgs_ov_dn7)) * 0.3333333333333)))))) / (assign25470_e26306 * assign25470_e26306))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgs_ov_dn8)) * assign25470_e26304) + (assign25470_e26283 * (((0.5 * (-(0.5 * locals.var_xgs_ov_dn8))) * assign25470_e26302) + (assign25470_e26292 * ((-(0.5 * locals.var_xgs_ov_dn8)) * 0.3333333333333)))))) / (assign25470_e26306 * assign25470_e26306))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgs_ov_dn9)) * assign25470_e26304) + (assign25470_e26283 * (((0.5 * (-(0.5 * locals.var_xgs_ov_dn9))) * assign25470_e26302) + (assign25470_e26292 * ((-(0.5 * locals.var_xgs_ov_dn9)) * 0.3333333333333)))))) / (assign25470_e26306 * assign25470_e26306))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25470_e26309;
        locals.var_temp_dn4 = assign25470_e26309_d_n4;
        locals.var_temp_dn6 = assign25470_e26309_d_n6;
        locals.var_temp_dn7 = assign25470_e26309_d_n7;
        locals.var_temp_dn8 = assign25470_e26309_d_n8;
        locals.var_temp_dn9 = assign25470_e26309_d_n9;

        let (assign25480_e26349, assign25480_e26349_d_n4, assign25480_e26349_d_n6, assign25480_e26349_d_n7, assign25480_e26349_d_n8, assign25480_e26349_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard723 == 0.0)) && (locals.var_guard724 == 0.0)) {
        let assign25480_e26323: f64 = (0.5 * locals.var_xgs_ov);
        let assign25480_e26325: f64 = (assign25480_e26323 - 80.0);
        let assign25480_e26330: f64 = (0.5 * locals.var_xgs_ov);
        let assign25480_e26332: f64 = (assign25480_e26330 - 80.0);
        let assign25480_e26333: f64 = (0.5 * assign25480_e26332);
        let assign25480_e26337: f64 = (0.5 * locals.var_xgs_ov);
        let assign25480_e26339: f64 = (assign25480_e26337 - 80.0);
        let assign25480_e26341: f64 = (assign25480_e26339 * 0.3333333333333);
        let assign25480_e26342: f64 = (1.0 + assign25480_e26341);
        let assign25480_e26343: f64 = (assign25480_e26333 * assign25480_e26342);
        let assign25480_e26344: f64 = (1.0 + assign25480_e26343);
        let assign25480_e26345: f64 = (assign25480_e26325 * assign25480_e26344);
        let assign25480_e26346: f64 = (1.0 + assign25480_e26345);
        let assign25480_e26347: f64 = (5.54062e34 * assign25480_e26346);
        (assign25480_e26347, (5.54062e34 * (((0.5 * locals.var_xgs_ov_dn4) * assign25480_e26344) + (assign25480_e26325 * (((0.5 * (0.5 * locals.var_xgs_ov_dn4)) * assign25480_e26342) + (assign25480_e26333 * ((0.5 * locals.var_xgs_ov_dn4) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgs_ov_dn6) * assign25480_e26344) + (assign25480_e26325 * (((0.5 * (0.5 * locals.var_xgs_ov_dn6)) * assign25480_e26342) + (assign25480_e26333 * ((0.5 * locals.var_xgs_ov_dn6) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgs_ov_dn7) * assign25480_e26344) + (assign25480_e26325 * (((0.5 * (0.5 * locals.var_xgs_ov_dn7)) * assign25480_e26342) + (assign25480_e26333 * ((0.5 * locals.var_xgs_ov_dn7) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgs_ov_dn8) * assign25480_e26344) + (assign25480_e26325 * (((0.5 * (0.5 * locals.var_xgs_ov_dn8)) * assign25480_e26342) + (assign25480_e26333 * ((0.5 * locals.var_xgs_ov_dn8) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgs_ov_dn9) * assign25480_e26344) + (assign25480_e26325 * (((0.5 * (0.5 * locals.var_xgs_ov_dn9)) * assign25480_e26342) + (assign25480_e26333 * ((0.5 * locals.var_xgs_ov_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25480_e26349;
        locals.var_temp_dn4 = assign25480_e26349_d_n4;
        locals.var_temp_dn6 = assign25480_e26349_d_n6;
        locals.var_temp_dn7 = assign25480_e26349_d_n7;
        locals.var_temp_dn8 = assign25480_e26349_d_n8;
        locals.var_temp_dn9 = assign25480_e26349_d_n9;

        let (assign25490_e26359, assign25490_e26359_d_n4, assign25490_e26359_d_n6, assign25490_e26359_d_n7, assign25490_e26359_d_n8, assign25490_e26359_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25490_e26356: f64 = (1.0 + locals.var_temp);
        let assign25490_e26357: f64 = (1.0 / assign25490_e26356);
        (assign25490_e26357, (-(locals.var_temp_dn4 / (assign25490_e26356 * assign25490_e26356))), (-(locals.var_temp_dn6 / (assign25490_e26356 * assign25490_e26356))), (-(locals.var_temp_dn7 / (assign25490_e26356 * assign25490_e26356))), (-(locals.var_temp_dn8 / (assign25490_e26356 * assign25490_e26356))), (-(locals.var_temp_dn9 / (assign25490_e26356 * assign25490_e26356))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign25490_e26359;
        locals.var_temp1_dn4 = assign25490_e26359_d_n4;
        locals.var_temp1_dn6 = assign25490_e26359_d_n6;
        locals.var_temp1_dn7 = assign25490_e26359_d_n7;
        locals.var_temp1_dn8 = assign25490_e26359_d_n8;
        locals.var_temp1_dn9 = assign25490_e26359_d_n9;

        let (assign25500_e26367, assign25500_e26367_d_n4, assign25500_e26367_d_n6, assign25500_e26367_d_n7, assign25500_e26367_d_n8, assign25500_e26367_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25500_e26365: f64 = (1.0 - locals.var_temp1);
        (assign25500_e26365, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign25500_e26367;
        locals.var_temp2_dn4 = assign25500_e26367_d_n4;
        locals.var_temp2_dn6 = assign25500_e26367_d_n6;
        locals.var_temp2_dn7 = assign25500_e26367_d_n7;
        locals.var_temp2_dn8 = assign25500_e26367_d_n8;
        locals.var_temp2_dn9 = assign25500_e26367_d_n9;

        let (assign25510_e26379, assign25510_e26379_d_n4, assign25510_e26379_d_n6, assign25510_e26379_d_n7, assign25510_e26379_d_n8, assign25510_e26379_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25510_e26373: f64 = (locals.var_gc2ovacc_i * locals.var_temp1);
        let assign25510_e26376: f64 = (locals.var_gc2ovinv_i * locals.var_temp2);
        let assign25510_e26377: f64 = (assign25510_e26373 + assign25510_e26376);
        (assign25510_e26377, ((locals.var_gc2ovacc_i * locals.var_temp1_dn4) + (locals.var_gc2ovinv_i * locals.var_temp2_dn4)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn6) + (locals.var_gc2ovinv_i * locals.var_temp2_dn6)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn7) + (locals.var_gc2ovinv_i * locals.var_temp2_dn7)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn8) + (locals.var_gc2ovinv_i * locals.var_temp2_dn8)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn9) + (locals.var_gc2ovinv_i * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gc2oveff, locals.var_gc2oveff_dn4, locals.var_gc2oveff_dn6, locals.var_gc2oveff_dn7, locals.var_gc2oveff_dn8, locals.var_gc2oveff_dn9,)
    }
};
        locals.var_gc2oveff = assign25510_e26379;
        locals.var_gc2oveff_dn4 = assign25510_e26379_d_n4;
        locals.var_gc2oveff_dn6 = assign25510_e26379_d_n6;
        locals.var_gc2oveff_dn7 = assign25510_e26379_d_n7;
        locals.var_gc2oveff_dn8 = assign25510_e26379_d_n8;
        locals.var_gc2oveff_dn9 = assign25510_e26379_d_n9;

    }

    pub(super) fn stamp_transient_block_65(
        locals: &mut StampLocals,
    ) {
        let (assign25520_e26391, assign25520_e26391_d_n4, assign25520_e26391_d_n6, assign25520_e26391_d_n7, assign25520_e26391_d_n8, assign25520_e26391_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25520_e26385: f64 = (locals.var_gc3ovacc_i * locals.var_temp1);
        let assign25520_e26388: f64 = (locals.var_gc3ovinv_i * locals.var_temp2);
        let assign25520_e26389: f64 = (assign25520_e26385 + assign25520_e26388);
        (assign25520_e26389, ((locals.var_gc3ovacc_i * locals.var_temp1_dn4) + (locals.var_gc3ovinv_i * locals.var_temp2_dn4)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn6) + (locals.var_gc3ovinv_i * locals.var_temp2_dn6)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn7) + (locals.var_gc3ovinv_i * locals.var_temp2_dn7)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn8) + (locals.var_gc3ovinv_i * locals.var_temp2_dn8)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn9) + (locals.var_gc3ovinv_i * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gc3oveff, locals.var_gc3oveff_dn4, locals.var_gc3oveff_dn6, locals.var_gc3oveff_dn7, locals.var_gc3oveff_dn8, locals.var_gc3oveff_dn9,)
    }
};
        locals.var_gc3oveff = assign25520_e26391;
        locals.var_gc3oveff_dn4 = assign25520_e26391_d_n4;
        locals.var_gc3oveff_dn6 = assign25520_e26391_d_n6;
        locals.var_gc3oveff_dn7 = assign25520_e26391_d_n7;
        locals.var_gc3oveff_dn8 = assign25520_e26391_d_n8;
        locals.var_gc3oveff_dn9 = assign25520_e26391_d_n9;

        let (assign25530_e26403, assign25530_e26403_d_n4, assign25530_e26403_d_n6, assign25530_e26403_d_n7, assign25530_e26403_d_n8, assign25530_e26403_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25530_e26397: f64 = (locals.var_gcqovacc * locals.var_temp1);
        let assign25530_e26400: f64 = (locals.var_gcqovinv * locals.var_temp2);
        let assign25530_e26401: f64 = (assign25530_e26397 + assign25530_e26400);
        (assign25530_e26401, ((locals.var_gcqovacc * locals.var_temp1_dn4) + (locals.var_gcqovinv * locals.var_temp2_dn4)), ((locals.var_gcqovacc * locals.var_temp1_dn6) + (locals.var_gcqovinv * locals.var_temp2_dn6)), ((locals.var_gcqovacc * locals.var_temp1_dn7) + (locals.var_gcqovinv * locals.var_temp2_dn7)), ((locals.var_gcqovacc * locals.var_temp1_dn8) + (locals.var_gcqovinv * locals.var_temp2_dn8)), ((locals.var_gcqovacc * locals.var_temp1_dn9) + (locals.var_gcqovinv * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gcqoveff, locals.var_gcqoveff_dn4, locals.var_gcqoveff_dn6, locals.var_gcqoveff_dn7, locals.var_gcqoveff_dn8, locals.var_gcqoveff_dn9,)
    }
};
        locals.var_gcqoveff = assign25530_e26403;
        locals.var_gcqoveff_dn4 = assign25530_e26403_d_n4;
        locals.var_gcqoveff_dn6 = assign25530_e26403_d_n6;
        locals.var_gcqoveff_dn7 = assign25530_e26403_d_n7;
        locals.var_gcqoveff_dn8 = assign25530_e26403_d_n8;
        locals.var_gcqoveff_dn9 = assign25530_e26403_d_n9;

        let (assign25560_e26438, assign25560_e26438_d_n4, assign25560_e26438_d_n6, assign25560_e26438_d_n7, assign25560_e26438_d_n8, assign25560_e26438_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25560_e26431: f64 = (-1.0);
        let assign25560_e26433: f64 = (assign25560_e26431 * locals.var_gcovinvfn_i);
        let assign25560_e26435: f64 = (assign25560_e26433 / locals.var_zg);
        let assign25560_e26436: f64 = (locals.var_bov * assign25560_e26435);
        (assign25560_e26436, ((locals.var_bov_dn4 * assign25560_e26435) + (locals.var_bov * (-((assign25560_e26433 * locals.var_zg_dn4) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn6 * assign25560_e26435) + (locals.var_bov * (-((assign25560_e26433 * locals.var_zg_dn6) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn7 * assign25560_e26435) + (locals.var_bov * (-((assign25560_e26433 * locals.var_zg_dn7) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn8 * assign25560_e26435) + (locals.var_bov * (-((assign25560_e26433 * locals.var_zg_dn8) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn9 * assign25560_e26435) + (locals.var_bov * (-((assign25560_e26433 * locals.var_zg_dn9) / (locals.var_zg * locals.var_zg))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign25560_e26438;
        locals.var_temp1_dn4 = assign25560_e26438_d_n4;
        locals.var_temp1_dn6 = assign25560_e26438_d_n6;
        locals.var_temp1_dn7 = assign25560_e26438_d_n7;
        locals.var_temp1_dn8 = assign25560_e26438_d_n8;
        locals.var_temp1_dn9 = assign25560_e26438_d_n9;

        let assign25570_e26441: f64 = if locals.var_gc3oveff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard725 = assign25570_e26441;

        let (assign25580_e26464, assign25580_e26464_d_n4, assign25580_e26464_d_n6, assign25580_e26464_d_n7, assign25580_e26464_d_n8, assign25580_e26464_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard725 != 0.0)) {
        let assign25580_e26450: f64 = (locals.var_zg + locals.var_gcqoveff);
        let assign25580_e26453: f64 = (locals.var_zg - locals.var_gcqoveff);
        let assign25580_e26456: f64 = (locals.var_zg - locals.var_gcqoveff);
        let assign25580_e26457: f64 = (assign25580_e26453 * assign25580_e26456);
        let assign25580_e26459: f64 = (assign25580_e26457 + 1e-6);
        let assign25580_e26460: f64 = (assign25580_e26459).sqrt();
        let assign25580_e26461: f64 = (assign25580_e26450 - assign25580_e26460);
        let assign25580_e26462: f64 = (0.5 * assign25580_e26461);
        (assign25580_e26462, (0.5 * ((locals.var_zg_dn4 + locals.var_gcqoveff_dn4) - ((((locals.var_zg_dn4 - locals.var_gcqoveff_dn4) * assign25580_e26456) + (assign25580_e26453 * (locals.var_zg_dn4 - locals.var_gcqoveff_dn4))) / (2.0 * assign25580_e26460)))), (0.5 * ((locals.var_zg_dn6 + locals.var_gcqoveff_dn6) - ((((locals.var_zg_dn6 - locals.var_gcqoveff_dn6) * assign25580_e26456) + (assign25580_e26453 * (locals.var_zg_dn6 - locals.var_gcqoveff_dn6))) / (2.0 * assign25580_e26460)))), (0.5 * ((locals.var_zg_dn7 + locals.var_gcqoveff_dn7) - ((((locals.var_zg_dn7 - locals.var_gcqoveff_dn7) * assign25580_e26456) + (assign25580_e26453 * (locals.var_zg_dn7 - locals.var_gcqoveff_dn7))) / (2.0 * assign25580_e26460)))), (0.5 * ((locals.var_zg_dn8 + locals.var_gcqoveff_dn8) - ((((locals.var_zg_dn8 - locals.var_gcqoveff_dn8) * assign25580_e26456) + (assign25580_e26453 * (locals.var_zg_dn8 - locals.var_gcqoveff_dn8))) / (2.0 * assign25580_e26460)))), (0.5 * ((locals.var_zg_dn9 + locals.var_gcqoveff_dn9) - ((((locals.var_zg_dn9 - locals.var_gcqoveff_dn9) * assign25580_e26456) + (assign25580_e26453 * (locals.var_zg_dn9 - locals.var_gcqoveff_dn9))) / (2.0 * assign25580_e26460)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign25580_e26464;
        locals.var_zg_dn4 = assign25580_e26464_d_n4;
        locals.var_zg_dn6 = assign25580_e26464_d_n6;
        locals.var_zg_dn7 = assign25580_e26464_d_n7;
        locals.var_zg_dn8 = assign25580_e26464_d_n8;
        locals.var_zg_dn9 = assign25580_e26464_d_n9;

        let (assign25590_e26476, assign25590_e26476_d_n4, assign25590_e26476_d_n6, assign25590_e26476_d_n7, assign25590_e26476_d_n8, assign25590_e26476_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25590_e26470: f64 = (3.0 + locals.var_xs_ov);
        let assign25590_e26473: f64 = (locals.var_psi_t * locals.var_inv_phit0);
        let assign25590_e26474: f64 = (assign25590_e26470 + assign25590_e26473);
        (assign25590_e26474, (locals.var_xs_ov_dn4 + ((locals.var_psi_t_dn4 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn4))), (locals.var_xs_ov_dn6 + ((locals.var_psi_t_dn6 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn6))), (locals.var_xs_ov_dn7 + ((locals.var_psi_t_dn7 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn7))), (locals.var_xs_ov_dn8 + ((locals.var_psi_t_dn8 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn8))), (locals.var_xs_ov_dn9 + ((locals.var_psi_t_dn9 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn9))),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign25590_e26476;
        locals.var_arg1_dn4 = assign25590_e26476_d_n4;
        locals.var_arg1_dn6 = assign25590_e26476_d_n6;
        locals.var_arg1_dn7 = assign25590_e26476_d_n7;
        locals.var_arg1_dn8 = assign25590_e26476_d_n8;
        locals.var_arg1_dn9 = assign25590_e26476_d_n9;

        let assign25600_e26478: f64 = (locals.var_arg1).abs();
        let assign25600_e26480: f64 = if assign25600_e26478 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard726 = assign25600_e26480;

        let (assign25610_e26489, assign25610_e26489_d_n4, assign25610_e26489_d_n6, assign25610_e26489_d_n7, assign25610_e26489_d_n8, assign25610_e26489_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard726 != 0.0)) {
        let assign25610_e26487: f64 = (locals.var_arg1).exp();
        (assign25610_e26487, (assign25610_e26487 * locals.var_arg1_dn4), (assign25610_e26487 * locals.var_arg1_dn6), (assign25610_e26487 * locals.var_arg1_dn7), (assign25610_e26487 * locals.var_arg1_dn8), (assign25610_e26487 * locals.var_arg1_dn9),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign25610_e26489;
        locals.var_dsi_dn4 = assign25610_e26489_d_n4;
        locals.var_dsi_dn6 = assign25610_e26489_d_n6;
        locals.var_dsi_dn7 = assign25610_e26489_d_n7;
        locals.var_dsi_dn8 = assign25610_e26489_d_n8;
        locals.var_dsi_dn9 = assign25610_e26489_d_n9;

        let assign25620_e26492: f64 = (-80.0);
        let assign25620_e26493: f64 = if locals.var_arg1 < assign25620_e26492 { 1.0 } else { 0.0 };
        locals.var_guard727 = assign25620_e26493;

        let (assign25630_e26529, assign25630_e26529_d_n4, assign25630_e26529_d_n6, assign25630_e26529_d_n7, assign25630_e26529_d_n8, assign25630_e26529_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard726 == 0.0)) && (locals.var_guard727 != 0.0)) {
        let assign25630_e26505: f64 = (-locals.var_arg1);
        let assign25630_e26507: f64 = (assign25630_e26505 - 80.0);
        let assign25630_e26511: f64 = (-locals.var_arg1);
        let assign25630_e26513: f64 = (assign25630_e26511 - 80.0);
        let assign25630_e26514: f64 = (0.5 * assign25630_e26513);
        let assign25630_e26517: f64 = (-locals.var_arg1);
        let assign25630_e26519: f64 = (assign25630_e26517 - 80.0);
        let assign25630_e26521: f64 = (assign25630_e26519 * 0.3333333333333);
        let assign25630_e26522: f64 = (1.0 + assign25630_e26521);
        let assign25630_e26523: f64 = (assign25630_e26514 * assign25630_e26522);
        let assign25630_e26524: f64 = (1.0 + assign25630_e26523);
        let assign25630_e26525: f64 = (assign25630_e26507 * assign25630_e26524);
        let assign25630_e26526: f64 = (1.0 + assign25630_e26525);
        let assign25630_e26527: f64 = (1.80485e-35 / assign25630_e26526);
        (assign25630_e26527, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign25630_e26524) + (assign25630_e26507 * (((0.5 * (-locals.var_arg1_dn4)) * assign25630_e26522) + (assign25630_e26514 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign25630_e26526 * assign25630_e26526))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign25630_e26524) + (assign25630_e26507 * (((0.5 * (-locals.var_arg1_dn6)) * assign25630_e26522) + (assign25630_e26514 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign25630_e26526 * assign25630_e26526))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign25630_e26524) + (assign25630_e26507 * (((0.5 * (-locals.var_arg1_dn7)) * assign25630_e26522) + (assign25630_e26514 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign25630_e26526 * assign25630_e26526))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign25630_e26524) + (assign25630_e26507 * (((0.5 * (-locals.var_arg1_dn8)) * assign25630_e26522) + (assign25630_e26514 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign25630_e26526 * assign25630_e26526))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign25630_e26524) + (assign25630_e26507 * (((0.5 * (-locals.var_arg1_dn9)) * assign25630_e26522) + (assign25630_e26514 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign25630_e26526 * assign25630_e26526))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign25630_e26529;
        locals.var_dsi_dn4 = assign25630_e26529_d_n4;
        locals.var_dsi_dn6 = assign25630_e26529_d_n6;
        locals.var_dsi_dn7 = assign25630_e26529_d_n7;
        locals.var_dsi_dn8 = assign25630_e26529_d_n8;
        locals.var_dsi_dn9 = assign25630_e26529_d_n9;

        let (assign25640_e26563, assign25640_e26563_d_n4, assign25640_e26563_d_n6, assign25640_e26563_d_n7, assign25640_e26563_d_n8, assign25640_e26563_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard726 == 0.0)) && (locals.var_guard727 == 0.0)) {
        let assign25640_e26543: f64 = (locals.var_arg1 - 80.0);
        let assign25640_e26548: f64 = (locals.var_arg1 - 80.0);
        let assign25640_e26549: f64 = (0.5 * assign25640_e26548);
        let assign25640_e26553: f64 = (locals.var_arg1 - 80.0);
        let assign25640_e26555: f64 = (assign25640_e26553 * 0.3333333333333);
        let assign25640_e26556: f64 = (1.0 + assign25640_e26555);
        let assign25640_e26557: f64 = (assign25640_e26549 * assign25640_e26556);
        let assign25640_e26558: f64 = (1.0 + assign25640_e26557);
        let assign25640_e26559: f64 = (assign25640_e26543 * assign25640_e26558);
        let assign25640_e26560: f64 = (1.0 + assign25640_e26559);
        let assign25640_e26561: f64 = (5.54062e34 * assign25640_e26560);
        (assign25640_e26561, (5.54062e34 * ((locals.var_arg1_dn4 * assign25640_e26558) + (assign25640_e26543 * (((0.5 * locals.var_arg1_dn4) * assign25640_e26556) + (assign25640_e26549 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign25640_e26558) + (assign25640_e26543 * (((0.5 * locals.var_arg1_dn6) * assign25640_e26556) + (assign25640_e26549 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign25640_e26558) + (assign25640_e26543 * (((0.5 * locals.var_arg1_dn7) * assign25640_e26556) + (assign25640_e26549 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign25640_e26558) + (assign25640_e26543 * (((0.5 * locals.var_arg1_dn8) * assign25640_e26556) + (assign25640_e26549 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign25640_e26558) + (assign25640_e26543 * (((0.5 * locals.var_arg1_dn9) * assign25640_e26556) + (assign25640_e26549 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign25640_e26563;
        locals.var_dsi_dn4 = assign25640_e26563_d_n4;
        locals.var_dsi_dn6 = assign25640_e26563_d_n6;
        locals.var_dsi_dn7 = assign25640_e26563_d_n7;
        locals.var_dsi_dn8 = assign25640_e26563_d_n8;
        locals.var_dsi_dn9 = assign25640_e26563_d_n9;

        let (assign25650_e26577, assign25650_e26577_d_n4, assign25650_e26577_d_n6, assign25650_e26577_d_n7, assign25650_e26577_d_n8, assign25650_e26577_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25650_e26569: f64 = (3.0 + locals.var_xs_ov);
        let assign25650_e26572: f64 = (locals.var_psi_t * locals.var_inv_phit0);
        let assign25650_e26573: f64 = (assign25650_e26569 + assign25650_e26572);
        let assign25650_e26575: f64 = (assign25650_e26573 + locals.var_xgs_ov);
        (assign25650_e26575, ((locals.var_xs_ov_dn4 + ((locals.var_psi_t_dn4 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn4))) + locals.var_xgs_ov_dn4), ((locals.var_xs_ov_dn6 + ((locals.var_psi_t_dn6 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn6))) + locals.var_xgs_ov_dn6), ((locals.var_xs_ov_dn7 + ((locals.var_psi_t_dn7 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn7))) + locals.var_xgs_ov_dn7), ((locals.var_xs_ov_dn8 + ((locals.var_psi_t_dn8 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn8))) + locals.var_xgs_ov_dn8), ((locals.var_xs_ov_dn9 + ((locals.var_psi_t_dn9 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn9))) + locals.var_xgs_ov_dn9),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign25650_e26577;
        locals.var_arg1_dn4 = assign25650_e26577_d_n4;
        locals.var_arg1_dn6 = assign25650_e26577_d_n6;
        locals.var_arg1_dn7 = assign25650_e26577_d_n7;
        locals.var_arg1_dn8 = assign25650_e26577_d_n8;
        locals.var_arg1_dn9 = assign25650_e26577_d_n9;

        let assign25660_e26579: f64 = (locals.var_arg1).abs();
        let assign25660_e26581: f64 = if assign25660_e26579 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard728 = assign25660_e26581;

        let (assign25670_e26590, assign25670_e26590_d_n4, assign25670_e26590_d_n6, assign25670_e26590_d_n7, assign25670_e26590_d_n8, assign25670_e26590_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard728 != 0.0)) {
        let assign25670_e26588: f64 = (locals.var_arg1).exp();
        (assign25670_e26588, (assign25670_e26588 * locals.var_arg1_dn4), (assign25670_e26588 * locals.var_arg1_dn6), (assign25670_e26588 * locals.var_arg1_dn7), (assign25670_e26588 * locals.var_arg1_dn8), (assign25670_e26588 * locals.var_arg1_dn9),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign25670_e26590;
        locals.var_dgate_dn4 = assign25670_e26590_d_n4;
        locals.var_dgate_dn6 = assign25670_e26590_d_n6;
        locals.var_dgate_dn7 = assign25670_e26590_d_n7;
        locals.var_dgate_dn8 = assign25670_e26590_d_n8;
        locals.var_dgate_dn9 = assign25670_e26590_d_n9;

        let assign25680_e26593: f64 = (-80.0);
        let assign25680_e26594: f64 = if locals.var_arg1 < assign25680_e26593 { 1.0 } else { 0.0 };
        locals.var_guard729 = assign25680_e26594;

        let (assign25690_e26630, assign25690_e26630_d_n4, assign25690_e26630_d_n6, assign25690_e26630_d_n7, assign25690_e26630_d_n8, assign25690_e26630_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 != 0.0)) {
        let assign25690_e26606: f64 = (-locals.var_arg1);
        let assign25690_e26608: f64 = (assign25690_e26606 - 80.0);
        let assign25690_e26612: f64 = (-locals.var_arg1);
        let assign25690_e26614: f64 = (assign25690_e26612 - 80.0);
        let assign25690_e26615: f64 = (0.5 * assign25690_e26614);
        let assign25690_e26618: f64 = (-locals.var_arg1);
        let assign25690_e26620: f64 = (assign25690_e26618 - 80.0);
        let assign25690_e26622: f64 = (assign25690_e26620 * 0.3333333333333);
        let assign25690_e26623: f64 = (1.0 + assign25690_e26622);
        let assign25690_e26624: f64 = (assign25690_e26615 * assign25690_e26623);
        let assign25690_e26625: f64 = (1.0 + assign25690_e26624);
        let assign25690_e26626: f64 = (assign25690_e26608 * assign25690_e26625);
        let assign25690_e26627: f64 = (1.0 + assign25690_e26626);
        let assign25690_e26628: f64 = (1.80485e-35 / assign25690_e26627);
        (assign25690_e26628, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign25690_e26625) + (assign25690_e26608 * (((0.5 * (-locals.var_arg1_dn4)) * assign25690_e26623) + (assign25690_e26615 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign25690_e26627 * assign25690_e26627))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign25690_e26625) + (assign25690_e26608 * (((0.5 * (-locals.var_arg1_dn6)) * assign25690_e26623) + (assign25690_e26615 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign25690_e26627 * assign25690_e26627))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign25690_e26625) + (assign25690_e26608 * (((0.5 * (-locals.var_arg1_dn7)) * assign25690_e26623) + (assign25690_e26615 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign25690_e26627 * assign25690_e26627))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign25690_e26625) + (assign25690_e26608 * (((0.5 * (-locals.var_arg1_dn8)) * assign25690_e26623) + (assign25690_e26615 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign25690_e26627 * assign25690_e26627))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign25690_e26625) + (assign25690_e26608 * (((0.5 * (-locals.var_arg1_dn9)) * assign25690_e26623) + (assign25690_e26615 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign25690_e26627 * assign25690_e26627))),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign25690_e26630;
        locals.var_dgate_dn4 = assign25690_e26630_d_n4;
        locals.var_dgate_dn6 = assign25690_e26630_d_n6;
        locals.var_dgate_dn7 = assign25690_e26630_d_n7;
        locals.var_dgate_dn8 = assign25690_e26630_d_n8;
        locals.var_dgate_dn9 = assign25690_e26630_d_n9;

        let (assign25700_e26664, assign25700_e26664_d_n4, assign25700_e26664_d_n6, assign25700_e26664_d_n7, assign25700_e26664_d_n8, assign25700_e26664_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) {
        let assign25700_e26644: f64 = (locals.var_arg1 - 80.0);
        let assign25700_e26649: f64 = (locals.var_arg1 - 80.0);
        let assign25700_e26650: f64 = (0.5 * assign25700_e26649);
        let assign25700_e26654: f64 = (locals.var_arg1 - 80.0);
        let assign25700_e26656: f64 = (assign25700_e26654 * 0.3333333333333);
        let assign25700_e26657: f64 = (1.0 + assign25700_e26656);
        let assign25700_e26658: f64 = (assign25700_e26650 * assign25700_e26657);
        let assign25700_e26659: f64 = (1.0 + assign25700_e26658);
        let assign25700_e26660: f64 = (assign25700_e26644 * assign25700_e26659);
        let assign25700_e26661: f64 = (1.0 + assign25700_e26660);
        let assign25700_e26662: f64 = (5.54062e34 * assign25700_e26661);
        (assign25700_e26662, (5.54062e34 * ((locals.var_arg1_dn4 * assign25700_e26659) + (assign25700_e26644 * (((0.5 * locals.var_arg1_dn4) * assign25700_e26657) + (assign25700_e26650 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign25700_e26659) + (assign25700_e26644 * (((0.5 * locals.var_arg1_dn6) * assign25700_e26657) + (assign25700_e26650 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign25700_e26659) + (assign25700_e26644 * (((0.5 * locals.var_arg1_dn7) * assign25700_e26657) + (assign25700_e26650 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign25700_e26659) + (assign25700_e26644 * (((0.5 * locals.var_arg1_dn8) * assign25700_e26657) + (assign25700_e26650 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign25700_e26659) + (assign25700_e26644 * (((0.5 * locals.var_arg1_dn9) * assign25700_e26657) + (assign25700_e26650 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign25700_e26664;
        locals.var_dgate_dn4 = assign25700_e26664_d_n4;
        locals.var_dgate_dn6 = assign25700_e26664_d_n6;
        locals.var_dgate_dn7 = assign25700_e26664_d_n7;
        locals.var_dgate_dn8 = assign25700_e26664_d_n8;
        locals.var_dgate_dn9 = assign25700_e26664_d_n9;

        let (assign25710_e26681, assign25710_e26681_d_n4, assign25710_e26681_d_n6, assign25710_e26681_d_n7, assign25710_e26681_d_n8, assign25710_e26681_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25710_e26670: f64 = (-1.5);
        let assign25710_e26675: f64 = (locals.var_gc3oveff * locals.var_zg);
        let assign25710_e26676: f64 = (locals.var_gc2oveff + assign25710_e26675);
        let assign25710_e26677: f64 = (locals.var_zg * assign25710_e26676);
        let assign25710_e26678: f64 = (assign25710_e26670 + assign25710_e26677);
        let assign25710_e26679: f64 = (locals.var_bov * assign25710_e26678);
        (assign25710_e26679, ((locals.var_bov_dn4 * assign25710_e26678) + (locals.var_bov * ((locals.var_zg_dn4 * assign25710_e26676) + (locals.var_zg * (locals.var_gc2oveff_dn4 + ((locals.var_gc3oveff_dn4 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn4))))))), ((locals.var_bov_dn6 * assign25710_e26678) + (locals.var_bov * ((locals.var_zg_dn6 * assign25710_e26676) + (locals.var_zg * (locals.var_gc2oveff_dn6 + ((locals.var_gc3oveff_dn6 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn6))))))), ((locals.var_bov_dn7 * assign25710_e26678) + (locals.var_bov * ((locals.var_zg_dn7 * assign25710_e26676) + (locals.var_zg * (locals.var_gc2oveff_dn7 + ((locals.var_gc3oveff_dn7 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn7))))))), ((locals.var_bov_dn8 * assign25710_e26678) + (locals.var_bov * ((locals.var_zg_dn8 * assign25710_e26676) + (locals.var_zg * (locals.var_gc2oveff_dn8 + ((locals.var_gc3oveff_dn8 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn8))))))), ((locals.var_bov_dn9 * assign25710_e26678) + (locals.var_bov * ((locals.var_zg_dn9 * assign25710_e26676) + (locals.var_zg * (locals.var_gc2oveff_dn9 + ((locals.var_gc3oveff_dn9 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn9))))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25710_e26681;
        locals.var_temp_dn4 = assign25710_e26681_d_n4;
        locals.var_temp_dn6 = assign25710_e26681_d_n6;
        locals.var_temp_dn7 = assign25710_e26681_d_n7;
        locals.var_temp_dn8 = assign25710_e26681_d_n8;
        locals.var_temp_dn9 = assign25710_e26681_d_n9;

        let (assign25820_e26849, assign25820_e26849_d_n4, assign25820_e26849_d_n6, assign25820_e26849_d_n7, assign25820_e26849_d_n8, assign25820_e26849_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25820_e26843: f64 = (1.0 + locals.var_dsi);
        let assign25820_e26846: f64 = (1.0 + locals.var_dgate);
        let assign25820_e26847: f64 = (assign25820_e26843 / assign25820_e26846);
        (assign25820_e26847, (((locals.var_dsi_dn4 * assign25820_e26846) - (assign25820_e26843 * locals.var_dgate_dn4)) / (assign25820_e26846 * assign25820_e26846)), (((locals.var_dsi_dn6 * assign25820_e26846) - (assign25820_e26843 * locals.var_dgate_dn6)) / (assign25820_e26846 * assign25820_e26846)), (((locals.var_dsi_dn7 * assign25820_e26846) - (assign25820_e26843 * locals.var_dgate_dn7)) / (assign25820_e26846 * assign25820_e26846)), (((locals.var_dsi_dn8 * assign25820_e26846) - (assign25820_e26843 * locals.var_dgate_dn8)) / (assign25820_e26846 * assign25820_e26846)), (((locals.var_dsi_dn9 * assign25820_e26846) - (assign25820_e26843 * locals.var_dgate_dn9)) / (assign25820_e26846 * assign25820_e26846)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25820_e26849;
        locals.var_temp_dn4 = assign25820_e26849_d_n4;
        locals.var_temp_dn6 = assign25820_e26849_d_n6;
        locals.var_temp_dn7 = assign25820_e26849_d_n7;
        locals.var_temp_dn8 = assign25820_e26849_d_n8;
        locals.var_temp_dn9 = assign25820_e26849_d_n9;

        let assign25830_e26852: f64 = if locals.var_temp < 1e-80 { 1.0 } else { 0.0 };
        locals.var_guard734 = assign25830_e26852;

        let (assign25840_e26860, assign25840_e26860_d_n4, assign25840_e26860_d_n6, assign25840_e26860_d_n7, assign25840_e26860_d_n8, assign25840_e26860_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard734 != 0.0)) {
        (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25840_e26860;
        locals.var_temp_dn4 = assign25840_e26860_d_n4;
        locals.var_temp_dn6 = assign25840_e26860_d_n6;
        locals.var_temp_dn7 = assign25840_e26860_d_n7;
        locals.var_temp_dn8 = assign25840_e26860_d_n8;
        locals.var_temp_dn9 = assign25840_e26860_d_n9;

        let (assign25850_e26870, assign25850_e26870_d_n4, assign25850_e26870_d_n6, assign25850_e26870_d_n7, assign25850_e26870_d_n8, assign25850_e26870_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25850_e26867: f64 = (locals.var_vgdu - locals.var_gcvdov_i);
        let assign25850_e26868: f64 = (locals.var_gcdov_i * assign25850_e26867);
        (assign25850_e26868, 0.0, (locals.var_gcdov_i * locals.var_vgdu_dn6), (locals.var_gcdov_i * locals.var_vgdu_dn7), 0.0, (locals.var_gcdov_i * locals.var_vgdu_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign25850_e26870;
        locals.var_temp1_dn4 = assign25850_e26870_d_n4;
        locals.var_temp1_dn6 = assign25850_e26870_d_n6;
        locals.var_temp1_dn7 = assign25850_e26870_d_n7;
        locals.var_temp1_dn8 = assign25850_e26870_d_n8;
        locals.var_temp1_dn9 = assign25850_e26870_d_n9;

        let assign25860_e26872: f64 = (locals.var_temp1).abs();
        let assign25860_e26874: f64 = if assign25860_e26872 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard735 = assign25860_e26874;

        let (assign25870_e26883, assign25870_e26883_d_n4, assign25870_e26883_d_n6, assign25870_e26883_d_n7, assign25870_e26883_d_n8, assign25870_e26883_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard735 != 0.0)) {
        let assign25870_e26881: f64 = (locals.var_temp1).exp();
        (assign25870_e26881, (assign25870_e26881 * locals.var_temp1_dn4), (assign25870_e26881 * locals.var_temp1_dn6), (assign25870_e26881 * locals.var_temp1_dn7), (assign25870_e26881 * locals.var_temp1_dn8), (assign25870_e26881 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign25870_e26883;
        locals.var_temp2_dn4 = assign25870_e26883_d_n4;
        locals.var_temp2_dn6 = assign25870_e26883_d_n6;
        locals.var_temp2_dn7 = assign25870_e26883_d_n7;
        locals.var_temp2_dn8 = assign25870_e26883_d_n8;
        locals.var_temp2_dn9 = assign25870_e26883_d_n9;

        let assign25880_e26886: f64 = (-80.0);
        let assign25880_e26887: f64 = if locals.var_temp1 < assign25880_e26886 { 1.0 } else { 0.0 };
        locals.var_guard736 = assign25880_e26887;

        let (assign25890_e26923, assign25890_e26923_d_n4, assign25890_e26923_d_n6, assign25890_e26923_d_n7, assign25890_e26923_d_n8, assign25890_e26923_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign25890_e26899: f64 = (-locals.var_temp1);
        let assign25890_e26901: f64 = (assign25890_e26899 - 80.0);
        let assign25890_e26905: f64 = (-locals.var_temp1);
        let assign25890_e26907: f64 = (assign25890_e26905 - 80.0);
        let assign25890_e26908: f64 = (0.5 * assign25890_e26907);
        let assign25890_e26911: f64 = (-locals.var_temp1);
        let assign25890_e26913: f64 = (assign25890_e26911 - 80.0);
        let assign25890_e26915: f64 = (assign25890_e26913 * 0.3333333333333);
        let assign25890_e26916: f64 = (1.0 + assign25890_e26915);
        let assign25890_e26917: f64 = (assign25890_e26908 * assign25890_e26916);
        let assign25890_e26918: f64 = (1.0 + assign25890_e26917);
        let assign25890_e26919: f64 = (assign25890_e26901 * assign25890_e26918);
        let assign25890_e26920: f64 = (1.0 + assign25890_e26919);
        let assign25890_e26921: f64 = (1.80485e-35 / assign25890_e26920);
        (assign25890_e26921, (-((1.80485e-35 * (((-locals.var_temp1_dn4) * assign25890_e26918) + (assign25890_e26901 * (((0.5 * (-locals.var_temp1_dn4)) * assign25890_e26916) + (assign25890_e26908 * ((-locals.var_temp1_dn4) * 0.3333333333333)))))) / (assign25890_e26920 * assign25890_e26920))), (-((1.80485e-35 * (((-locals.var_temp1_dn6) * assign25890_e26918) + (assign25890_e26901 * (((0.5 * (-locals.var_temp1_dn6)) * assign25890_e26916) + (assign25890_e26908 * ((-locals.var_temp1_dn6) * 0.3333333333333)))))) / (assign25890_e26920 * assign25890_e26920))), (-((1.80485e-35 * (((-locals.var_temp1_dn7) * assign25890_e26918) + (assign25890_e26901 * (((0.5 * (-locals.var_temp1_dn7)) * assign25890_e26916) + (assign25890_e26908 * ((-locals.var_temp1_dn7) * 0.3333333333333)))))) / (assign25890_e26920 * assign25890_e26920))), (-((1.80485e-35 * (((-locals.var_temp1_dn8) * assign25890_e26918) + (assign25890_e26901 * (((0.5 * (-locals.var_temp1_dn8)) * assign25890_e26916) + (assign25890_e26908 * ((-locals.var_temp1_dn8) * 0.3333333333333)))))) / (assign25890_e26920 * assign25890_e26920))), (-((1.80485e-35 * (((-locals.var_temp1_dn9) * assign25890_e26918) + (assign25890_e26901 * (((0.5 * (-locals.var_temp1_dn9)) * assign25890_e26916) + (assign25890_e26908 * ((-locals.var_temp1_dn9) * 0.3333333333333)))))) / (assign25890_e26920 * assign25890_e26920))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign25890_e26923;
        locals.var_temp2_dn4 = assign25890_e26923_d_n4;
        locals.var_temp2_dn6 = assign25890_e26923_d_n6;
        locals.var_temp2_dn7 = assign25890_e26923_d_n7;
        locals.var_temp2_dn8 = assign25890_e26923_d_n8;
        locals.var_temp2_dn9 = assign25890_e26923_d_n9;

        let (assign25900_e26957, assign25900_e26957_d_n4, assign25900_e26957_d_n6, assign25900_e26957_d_n7, assign25900_e26957_d_n8, assign25900_e26957_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 == 0.0)) {
        let assign25900_e26937: f64 = (locals.var_temp1 - 80.0);
        let assign25900_e26942: f64 = (locals.var_temp1 - 80.0);
        let assign25900_e26943: f64 = (0.5 * assign25900_e26942);
        let assign25900_e26947: f64 = (locals.var_temp1 - 80.0);
        let assign25900_e26949: f64 = (assign25900_e26947 * 0.3333333333333);
        let assign25900_e26950: f64 = (1.0 + assign25900_e26949);
        let assign25900_e26951: f64 = (assign25900_e26943 * assign25900_e26950);
        let assign25900_e26952: f64 = (1.0 + assign25900_e26951);
        let assign25900_e26953: f64 = (assign25900_e26937 * assign25900_e26952);
        let assign25900_e26954: f64 = (1.0 + assign25900_e26953);
        let assign25900_e26955: f64 = (5.54062e34 * assign25900_e26954);
        (assign25900_e26955, (5.54062e34 * ((locals.var_temp1_dn4 * assign25900_e26952) + (assign25900_e26937 * (((0.5 * locals.var_temp1_dn4) * assign25900_e26950) + (assign25900_e26943 * (locals.var_temp1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn6 * assign25900_e26952) + (assign25900_e26937 * (((0.5 * locals.var_temp1_dn6) * assign25900_e26950) + (assign25900_e26943 * (locals.var_temp1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn7 * assign25900_e26952) + (assign25900_e26937 * (((0.5 * locals.var_temp1_dn7) * assign25900_e26950) + (assign25900_e26943 * (locals.var_temp1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn8 * assign25900_e26952) + (assign25900_e26937 * (((0.5 * locals.var_temp1_dn8) * assign25900_e26950) + (assign25900_e26943 * (locals.var_temp1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn9 * assign25900_e26952) + (assign25900_e26937 * (((0.5 * locals.var_temp1_dn9) * assign25900_e26950) + (assign25900_e26943 * (locals.var_temp1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign25900_e26957;
        locals.var_temp2_dn4 = assign25900_e26957_d_n4;
        locals.var_temp2_dn6 = assign25900_e26957_d_n6;
        locals.var_temp2_dn7 = assign25900_e26957_d_n7;
        locals.var_temp2_dn8 = assign25900_e26957_d_n8;
        locals.var_temp2_dn9 = assign25900_e26957_d_n9;

        let (assign25910_e26967, assign25910_e26967_d_n4, assign25910_e26967_d_n6, assign25910_e26967_d_n7, assign25910_e26967_d_n8, assign25910_e26967_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25910_e26963: f64 = (locals.var_gcdov_i * locals.var_vsdu);
        let assign25910_e26965: f64 = (assign25910_e26963 + locals.var_temp1);
        (assign25910_e26965, locals.var_temp1_dn4, ((locals.var_gcdov_i * locals.var_vsdu_dn6) + locals.var_temp1_dn6), ((locals.var_gcdov_i * locals.var_vsdu_dn7) + locals.var_temp1_dn7), locals.var_temp1_dn8, locals.var_temp1_dn9,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign25910_e26967;
        locals.var_temp3_dn4 = assign25910_e26967_d_n4;
        locals.var_temp3_dn6 = assign25910_e26967_d_n6;
        locals.var_temp3_dn7 = assign25910_e26967_d_n7;
        locals.var_temp3_dn8 = assign25910_e26967_d_n8;
        locals.var_temp3_dn9 = assign25910_e26967_d_n9;

        let assign25920_e26969: f64 = (locals.var_temp3).abs();
        let assign25920_e26971: f64 = if assign25920_e26969 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard737 = assign25920_e26971;

        let (assign25930_e26980, assign25930_e26980_d_n4, assign25930_e26980_d_n6, assign25930_e26980_d_n7, assign25930_e26980_d_n8, assign25930_e26980_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard737 != 0.0)) {
        let assign25930_e26978: f64 = (locals.var_temp3).exp();
        (assign25930_e26978, (assign25930_e26978 * locals.var_temp3_dn4), (assign25930_e26978 * locals.var_temp3_dn6), (assign25930_e26978 * locals.var_temp3_dn7), (assign25930_e26978 * locals.var_temp3_dn8), (assign25930_e26978 * locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign25930_e26980;
        locals.var_temp4_dn4 = assign25930_e26980_d_n4;
        locals.var_temp4_dn6 = assign25930_e26980_d_n6;
        locals.var_temp4_dn7 = assign25930_e26980_d_n7;
        locals.var_temp4_dn8 = assign25930_e26980_d_n8;
        locals.var_temp4_dn9 = assign25930_e26980_d_n9;

        let assign25940_e26983: f64 = (-80.0);
        let assign25940_e26984: f64 = if locals.var_temp3 < assign25940_e26983 { 1.0 } else { 0.0 };
        locals.var_guard738 = assign25940_e26984;

        let (assign25950_e27020, assign25950_e27020_d_n4, assign25950_e27020_d_n6, assign25950_e27020_d_n7, assign25950_e27020_d_n8, assign25950_e27020_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign25950_e26996: f64 = (-locals.var_temp3);
        let assign25950_e26998: f64 = (assign25950_e26996 - 80.0);
        let assign25950_e27002: f64 = (-locals.var_temp3);
        let assign25950_e27004: f64 = (assign25950_e27002 - 80.0);
        let assign25950_e27005: f64 = (0.5 * assign25950_e27004);
        let assign25950_e27008: f64 = (-locals.var_temp3);
        let assign25950_e27010: f64 = (assign25950_e27008 - 80.0);
        let assign25950_e27012: f64 = (assign25950_e27010 * 0.3333333333333);
        let assign25950_e27013: f64 = (1.0 + assign25950_e27012);
        let assign25950_e27014: f64 = (assign25950_e27005 * assign25950_e27013);
        let assign25950_e27015: f64 = (1.0 + assign25950_e27014);
        let assign25950_e27016: f64 = (assign25950_e26998 * assign25950_e27015);
        let assign25950_e27017: f64 = (1.0 + assign25950_e27016);
        let assign25950_e27018: f64 = (1.80485e-35 / assign25950_e27017);
        (assign25950_e27018, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign25950_e27015) + (assign25950_e26998 * (((0.5 * (-locals.var_temp3_dn4)) * assign25950_e27013) + (assign25950_e27005 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign25950_e27017 * assign25950_e27017))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign25950_e27015) + (assign25950_e26998 * (((0.5 * (-locals.var_temp3_dn6)) * assign25950_e27013) + (assign25950_e27005 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign25950_e27017 * assign25950_e27017))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign25950_e27015) + (assign25950_e26998 * (((0.5 * (-locals.var_temp3_dn7)) * assign25950_e27013) + (assign25950_e27005 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign25950_e27017 * assign25950_e27017))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign25950_e27015) + (assign25950_e26998 * (((0.5 * (-locals.var_temp3_dn8)) * assign25950_e27013) + (assign25950_e27005 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign25950_e27017 * assign25950_e27017))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign25950_e27015) + (assign25950_e26998 * (((0.5 * (-locals.var_temp3_dn9)) * assign25950_e27013) + (assign25950_e27005 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign25950_e27017 * assign25950_e27017))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign25950_e27020;
        locals.var_temp4_dn4 = assign25950_e27020_d_n4;
        locals.var_temp4_dn6 = assign25950_e27020_d_n6;
        locals.var_temp4_dn7 = assign25950_e27020_d_n7;
        locals.var_temp4_dn8 = assign25950_e27020_d_n8;
        locals.var_temp4_dn9 = assign25950_e27020_d_n9;

        let (assign25960_e27054, assign25960_e27054_d_n4, assign25960_e27054_d_n6, assign25960_e27054_d_n7, assign25960_e27054_d_n8, assign25960_e27054_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        let assign25960_e27034: f64 = (locals.var_temp3 - 80.0);
        let assign25960_e27039: f64 = (locals.var_temp3 - 80.0);
        let assign25960_e27040: f64 = (0.5 * assign25960_e27039);
        let assign25960_e27044: f64 = (locals.var_temp3 - 80.0);
        let assign25960_e27046: f64 = (assign25960_e27044 * 0.3333333333333);
        let assign25960_e27047: f64 = (1.0 + assign25960_e27046);
        let assign25960_e27048: f64 = (assign25960_e27040 * assign25960_e27047);
        let assign25960_e27049: f64 = (1.0 + assign25960_e27048);
        let assign25960_e27050: f64 = (assign25960_e27034 * assign25960_e27049);
        let assign25960_e27051: f64 = (1.0 + assign25960_e27050);
        let assign25960_e27052: f64 = (5.54062e34 * assign25960_e27051);
        (assign25960_e27052, (5.54062e34 * ((locals.var_temp3_dn4 * assign25960_e27049) + (assign25960_e27034 * (((0.5 * locals.var_temp3_dn4) * assign25960_e27047) + (assign25960_e27040 * (locals.var_temp3_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn6 * assign25960_e27049) + (assign25960_e27034 * (((0.5 * locals.var_temp3_dn6) * assign25960_e27047) + (assign25960_e27040 * (locals.var_temp3_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn7 * assign25960_e27049) + (assign25960_e27034 * (((0.5 * locals.var_temp3_dn7) * assign25960_e27047) + (assign25960_e27040 * (locals.var_temp3_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn8 * assign25960_e27049) + (assign25960_e27034 * (((0.5 * locals.var_temp3_dn8) * assign25960_e27047) + (assign25960_e27040 * (locals.var_temp3_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn9 * assign25960_e27049) + (assign25960_e27034 * (((0.5 * locals.var_temp3_dn9) * assign25960_e27047) + (assign25960_e27040 * (locals.var_temp3_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign25960_e27054;
        locals.var_temp4_dn4 = assign25960_e27054_d_n4;
        locals.var_temp4_dn6 = assign25960_e27054_d_n6;
        locals.var_temp4_dn7 = assign25960_e27054_d_n7;
        locals.var_temp4_dn8 = assign25960_e27054_d_n8;
        locals.var_temp4_dn9 = assign25960_e27054_d_n9;

        let assign25980_e27092: f64 = if ((locals.var_igovinvd_i > 0.0) || (locals.var_igovaccd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard739 = assign25980_e27092;

    }

    pub(super) fn stamp_transient_block_66(
        locals: &mut StampLocals,
    ) {
        let (assign25990_e27100, assign25990_e27100_d_n4, assign25990_e27100_d_n6, assign25990_e27100_d_n7, assign25990_e27100_d_n8, assign25990_e27100_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign25990_e27098: f64 = (locals.var_vovd + locals.var_dov);
        (assign25990_e27098, (locals.var_vovd_dn4 + locals.var_dov_dn4), (locals.var_vovd_dn6 + locals.var_dov_dn6), (locals.var_vovd_dn7 + locals.var_dov_dn7), (locals.var_vovd_dn8 + locals.var_dov_dn8), (locals.var_vovd_dn9 + locals.var_dov_dn9),)
    } else {
        (locals.var_arg2mina, locals.var_arg2mina_dn4, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8, locals.var_arg2mina_dn9,)
    }
};
        locals.var_arg2mina = assign25990_e27100;
        locals.var_arg2mina_dn4 = assign25990_e27100_d_n4;
        locals.var_arg2mina_dn6 = assign25990_e27100_d_n6;
        locals.var_arg2mina_dn7 = assign25990_e27100_d_n7;
        locals.var_arg2mina_dn8 = assign25990_e27100_d_n8;
        locals.var_arg2mina_dn9 = assign25990_e27100_d_n9;

        let (assign26000_e27121, assign26000_e27121_d_n4, assign26000_e27121_d_n6, assign26000_e27121_d_n7, assign26000_e27121_d_n8, assign26000_e27121_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26000_e27107: f64 = locals.var_arg2mina;
        let assign26000_e27110: f64 = (-locals.var_arg2mina);
        let assign26000_e27113: f64 = (-locals.var_arg2mina);
        let assign26000_e27114: f64 = (assign26000_e27110 * assign26000_e27113);
        let assign26000_e27116: f64 = (assign26000_e27114 + 0.01);
        let assign26000_e27117: f64 = (assign26000_e27116).sqrt();
        let assign26000_e27118: f64 = (assign26000_e27107 - assign26000_e27117);
        let assign26000_e27119: f64 = (0.5 * assign26000_e27118);
        (assign26000_e27119, (0.5 * (locals.var_arg2mina_dn4 - ((((-locals.var_arg2mina_dn4) * assign26000_e27113) + (assign26000_e27110 * (-locals.var_arg2mina_dn4))) / (2.0 * assign26000_e27117)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign26000_e27113) + (assign26000_e27110 * (-locals.var_arg2mina_dn6))) / (2.0 * assign26000_e27117)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign26000_e27113) + (assign26000_e27110 * (-locals.var_arg2mina_dn7))) / (2.0 * assign26000_e27117)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign26000_e27113) + (assign26000_e27110 * (-locals.var_arg2mina_dn8))) / (2.0 * assign26000_e27117)))), (0.5 * (locals.var_arg2mina_dn9 - ((((-locals.var_arg2mina_dn9) * assign26000_e27113) + (assign26000_e27110 * (-locals.var_arg2mina_dn9))) / (2.0 * assign26000_e27117)))),)
    } else {
        (locals.var_psi_t, locals.var_psi_t_dn4, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8, locals.var_psi_t_dn9,)
    }
};
        locals.var_psi_t = assign26000_e27121;
        locals.var_psi_t_dn4 = assign26000_e27121_d_n4;
        locals.var_psi_t_dn6 = assign26000_e27121_d_n6;
        locals.var_psi_t_dn7 = assign26000_e27121_d_n7;
        locals.var_psi_t_dn8 = assign26000_e27121_d_n8;
        locals.var_psi_t_dn9 = assign26000_e27121_d_n9;

        let (assign26010_e27134, assign26010_e27134_d_n4, assign26010_e27134_d_n6, assign26010_e27134_d_n7, assign26010_e27134_d_n8, assign26010_e27134_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26010_e27127: f64 = (locals.var_vovd * locals.var_vovd);
        let assign26010_e27129: f64 = (assign26010_e27127 + 0.0001);
        let assign26010_e27130: f64 = (assign26010_e27129).sqrt();
        let assign26010_e27132: f64 = (assign26010_e27130 * locals.var_inv_chib);
        (assign26010_e27132, ((((locals.var_vovd_dn4 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn4)) / (2.0 * assign26010_e27130)) * locals.var_inv_chib), ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) / (2.0 * assign26010_e27130)) * locals.var_inv_chib), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) / (2.0 * assign26010_e27130)) * locals.var_inv_chib), ((((locals.var_vovd_dn8 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn8)) / (2.0 * assign26010_e27130)) * locals.var_inv_chib), ((((locals.var_vovd_dn9 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn9)) / (2.0 * assign26010_e27130)) * locals.var_inv_chib),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign26010_e27134;
        locals.var_zg_dn4 = assign26010_e27134_d_n4;
        locals.var_zg_dn6 = assign26010_e27134_d_n6;
        locals.var_zg_dn7 = assign26010_e27134_d_n7;
        locals.var_zg_dn8 = assign26010_e27134_d_n8;
        locals.var_zg_dn9 = assign26010_e27134_d_n9;

        let assign26020_e27137: f64 = (0.5 * locals.var_xgd_ov);
        let assign26020_e27138: f64 = (assign26020_e27137).abs();
        let assign26020_e27140: f64 = if assign26020_e27138 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign26020_e27140;

        let (assign26030_e27151, assign26030_e27151_d_n4, assign26030_e27151_d_n6, assign26030_e27151_d_n7, assign26030_e27151_d_n8, assign26030_e27151_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign26030_e27148: f64 = (0.5 * locals.var_xgd_ov);
        let assign26030_e27149: f64 = (assign26030_e27148).exp();
        (assign26030_e27149, (assign26030_e27149 * (0.5 * locals.var_xgd_ov_dn4)), (assign26030_e27149 * (0.5 * locals.var_xgd_ov_dn6)), (assign26030_e27149 * (0.5 * locals.var_xgd_ov_dn7)), (assign26030_e27149 * (0.5 * locals.var_xgd_ov_dn8)), (assign26030_e27149 * (0.5 * locals.var_xgd_ov_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26030_e27151;
        locals.var_temp_dn4 = assign26030_e27151_d_n4;
        locals.var_temp_dn6 = assign26030_e27151_d_n6;
        locals.var_temp_dn7 = assign26030_e27151_d_n7;
        locals.var_temp_dn8 = assign26030_e27151_d_n8;
        locals.var_temp_dn9 = assign26030_e27151_d_n9;

        let assign26040_e27154: f64 = (0.5 * locals.var_xgd_ov);
        let assign26040_e27156: f64 = (-80.0);
        let assign26040_e27157: f64 = if assign26040_e27154 < assign26040_e27156 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign26040_e27157;

        let (assign26050_e27199, assign26050_e27199_d_n4, assign26050_e27199_d_n6, assign26050_e27199_d_n7, assign26050_e27199_d_n8, assign26050_e27199_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard740 == 0.0)) && (locals.var_guard741 != 0.0)) {
        let assign26050_e27170: f64 = (0.5 * locals.var_xgd_ov);
        let assign26050_e27171: f64 = (-assign26050_e27170);
        let assign26050_e27173: f64 = (assign26050_e27171 - 80.0);
        let assign26050_e27178: f64 = (0.5 * locals.var_xgd_ov);
        let assign26050_e27179: f64 = (-assign26050_e27178);
        let assign26050_e27181: f64 = (assign26050_e27179 - 80.0);
        let assign26050_e27182: f64 = (0.5 * assign26050_e27181);
        let assign26050_e27186: f64 = (0.5 * locals.var_xgd_ov);
        let assign26050_e27187: f64 = (-assign26050_e27186);
        let assign26050_e27189: f64 = (assign26050_e27187 - 80.0);
        let assign26050_e27191: f64 = (assign26050_e27189 * 0.3333333333333);
        let assign26050_e27192: f64 = (1.0 + assign26050_e27191);
        let assign26050_e27193: f64 = (assign26050_e27182 * assign26050_e27192);
        let assign26050_e27194: f64 = (1.0 + assign26050_e27193);
        let assign26050_e27195: f64 = (assign26050_e27173 * assign26050_e27194);
        let assign26050_e27196: f64 = (1.0 + assign26050_e27195);
        let assign26050_e27197: f64 = (1.80485e-35 / assign26050_e27196);
        (assign26050_e27197, (-((1.80485e-35 * (((-(0.5 * locals.var_xgd_ov_dn4)) * assign26050_e27194) + (assign26050_e27173 * (((0.5 * (-(0.5 * locals.var_xgd_ov_dn4))) * assign26050_e27192) + (assign26050_e27182 * ((-(0.5 * locals.var_xgd_ov_dn4)) * 0.3333333333333)))))) / (assign26050_e27196 * assign26050_e27196))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgd_ov_dn6)) * assign26050_e27194) + (assign26050_e27173 * (((0.5 * (-(0.5 * locals.var_xgd_ov_dn6))) * assign26050_e27192) + (assign26050_e27182 * ((-(0.5 * locals.var_xgd_ov_dn6)) * 0.3333333333333)))))) / (assign26050_e27196 * assign26050_e27196))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgd_ov_dn7)) * assign26050_e27194) + (assign26050_e27173 * (((0.5 * (-(0.5 * locals.var_xgd_ov_dn7))) * assign26050_e27192) + (assign26050_e27182 * ((-(0.5 * locals.var_xgd_ov_dn7)) * 0.3333333333333)))))) / (assign26050_e27196 * assign26050_e27196))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgd_ov_dn8)) * assign26050_e27194) + (assign26050_e27173 * (((0.5 * (-(0.5 * locals.var_xgd_ov_dn8))) * assign26050_e27192) + (assign26050_e27182 * ((-(0.5 * locals.var_xgd_ov_dn8)) * 0.3333333333333)))))) / (assign26050_e27196 * assign26050_e27196))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgd_ov_dn9)) * assign26050_e27194) + (assign26050_e27173 * (((0.5 * (-(0.5 * locals.var_xgd_ov_dn9))) * assign26050_e27192) + (assign26050_e27182 * ((-(0.5 * locals.var_xgd_ov_dn9)) * 0.3333333333333)))))) / (assign26050_e27196 * assign26050_e27196))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26050_e27199;
        locals.var_temp_dn4 = assign26050_e27199_d_n4;
        locals.var_temp_dn6 = assign26050_e27199_d_n6;
        locals.var_temp_dn7 = assign26050_e27199_d_n7;
        locals.var_temp_dn8 = assign26050_e27199_d_n8;
        locals.var_temp_dn9 = assign26050_e27199_d_n9;

        let (assign26060_e27239, assign26060_e27239_d_n4, assign26060_e27239_d_n6, assign26060_e27239_d_n7, assign26060_e27239_d_n8, assign26060_e27239_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard740 == 0.0)) && (locals.var_guard741 == 0.0)) {
        let assign26060_e27213: f64 = (0.5 * locals.var_xgd_ov);
        let assign26060_e27215: f64 = (assign26060_e27213 - 80.0);
        let assign26060_e27220: f64 = (0.5 * locals.var_xgd_ov);
        let assign26060_e27222: f64 = (assign26060_e27220 - 80.0);
        let assign26060_e27223: f64 = (0.5 * assign26060_e27222);
        let assign26060_e27227: f64 = (0.5 * locals.var_xgd_ov);
        let assign26060_e27229: f64 = (assign26060_e27227 - 80.0);
        let assign26060_e27231: f64 = (assign26060_e27229 * 0.3333333333333);
        let assign26060_e27232: f64 = (1.0 + assign26060_e27231);
        let assign26060_e27233: f64 = (assign26060_e27223 * assign26060_e27232);
        let assign26060_e27234: f64 = (1.0 + assign26060_e27233);
        let assign26060_e27235: f64 = (assign26060_e27215 * assign26060_e27234);
        let assign26060_e27236: f64 = (1.0 + assign26060_e27235);
        let assign26060_e27237: f64 = (5.54062e34 * assign26060_e27236);
        (assign26060_e27237, (5.54062e34 * (((0.5 * locals.var_xgd_ov_dn4) * assign26060_e27234) + (assign26060_e27215 * (((0.5 * (0.5 * locals.var_xgd_ov_dn4)) * assign26060_e27232) + (assign26060_e27223 * ((0.5 * locals.var_xgd_ov_dn4) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgd_ov_dn6) * assign26060_e27234) + (assign26060_e27215 * (((0.5 * (0.5 * locals.var_xgd_ov_dn6)) * assign26060_e27232) + (assign26060_e27223 * ((0.5 * locals.var_xgd_ov_dn6) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgd_ov_dn7) * assign26060_e27234) + (assign26060_e27215 * (((0.5 * (0.5 * locals.var_xgd_ov_dn7)) * assign26060_e27232) + (assign26060_e27223 * ((0.5 * locals.var_xgd_ov_dn7) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgd_ov_dn8) * assign26060_e27234) + (assign26060_e27215 * (((0.5 * (0.5 * locals.var_xgd_ov_dn8)) * assign26060_e27232) + (assign26060_e27223 * ((0.5 * locals.var_xgd_ov_dn8) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgd_ov_dn9) * assign26060_e27234) + (assign26060_e27215 * (((0.5 * (0.5 * locals.var_xgd_ov_dn9)) * assign26060_e27232) + (assign26060_e27223 * ((0.5 * locals.var_xgd_ov_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26060_e27239;
        locals.var_temp_dn4 = assign26060_e27239_d_n4;
        locals.var_temp_dn6 = assign26060_e27239_d_n6;
        locals.var_temp_dn7 = assign26060_e27239_d_n7;
        locals.var_temp_dn8 = assign26060_e27239_d_n8;
        locals.var_temp_dn9 = assign26060_e27239_d_n9;

        let (assign26070_e27249, assign26070_e27249_d_n4, assign26070_e27249_d_n6, assign26070_e27249_d_n7, assign26070_e27249_d_n8, assign26070_e27249_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26070_e27246: f64 = (1.0 + locals.var_temp);
        let assign26070_e27247: f64 = (1.0 / assign26070_e27246);
        (assign26070_e27247, (-(locals.var_temp_dn4 / (assign26070_e27246 * assign26070_e27246))), (-(locals.var_temp_dn6 / (assign26070_e27246 * assign26070_e27246))), (-(locals.var_temp_dn7 / (assign26070_e27246 * assign26070_e27246))), (-(locals.var_temp_dn8 / (assign26070_e27246 * assign26070_e27246))), (-(locals.var_temp_dn9 / (assign26070_e27246 * assign26070_e27246))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign26070_e27249;
        locals.var_temp1_dn4 = assign26070_e27249_d_n4;
        locals.var_temp1_dn6 = assign26070_e27249_d_n6;
        locals.var_temp1_dn7 = assign26070_e27249_d_n7;
        locals.var_temp1_dn8 = assign26070_e27249_d_n8;
        locals.var_temp1_dn9 = assign26070_e27249_d_n9;

        let (assign26080_e27257, assign26080_e27257_d_n4, assign26080_e27257_d_n6, assign26080_e27257_d_n7, assign26080_e27257_d_n8, assign26080_e27257_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26080_e27255: f64 = (1.0 - locals.var_temp1);
        (assign26080_e27255, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign26080_e27257;
        locals.var_temp2_dn4 = assign26080_e27257_d_n4;
        locals.var_temp2_dn6 = assign26080_e27257_d_n6;
        locals.var_temp2_dn7 = assign26080_e27257_d_n7;
        locals.var_temp2_dn8 = assign26080_e27257_d_n8;
        locals.var_temp2_dn9 = assign26080_e27257_d_n9;

        let (assign26090_e27269, assign26090_e27269_d_n4, assign26090_e27269_d_n6, assign26090_e27269_d_n7, assign26090_e27269_d_n8, assign26090_e27269_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26090_e27263: f64 = (locals.var_gc2ovacc_i * locals.var_temp1);
        let assign26090_e27266: f64 = (locals.var_gc2ovinv_i * locals.var_temp2);
        let assign26090_e27267: f64 = (assign26090_e27263 + assign26090_e27266);
        (assign26090_e27267, ((locals.var_gc2ovacc_i * locals.var_temp1_dn4) + (locals.var_gc2ovinv_i * locals.var_temp2_dn4)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn6) + (locals.var_gc2ovinv_i * locals.var_temp2_dn6)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn7) + (locals.var_gc2ovinv_i * locals.var_temp2_dn7)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn8) + (locals.var_gc2ovinv_i * locals.var_temp2_dn8)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn9) + (locals.var_gc2ovinv_i * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gc2oveff, locals.var_gc2oveff_dn4, locals.var_gc2oveff_dn6, locals.var_gc2oveff_dn7, locals.var_gc2oveff_dn8, locals.var_gc2oveff_dn9,)
    }
};
        locals.var_gc2oveff = assign26090_e27269;
        locals.var_gc2oveff_dn4 = assign26090_e27269_d_n4;
        locals.var_gc2oveff_dn6 = assign26090_e27269_d_n6;
        locals.var_gc2oveff_dn7 = assign26090_e27269_d_n7;
        locals.var_gc2oveff_dn8 = assign26090_e27269_d_n8;
        locals.var_gc2oveff_dn9 = assign26090_e27269_d_n9;

        let (assign26100_e27281, assign26100_e27281_d_n4, assign26100_e27281_d_n6, assign26100_e27281_d_n7, assign26100_e27281_d_n8, assign26100_e27281_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26100_e27275: f64 = (locals.var_gc3ovacc_i * locals.var_temp1);
        let assign26100_e27278: f64 = (locals.var_gc3ovinv_i * locals.var_temp2);
        let assign26100_e27279: f64 = (assign26100_e27275 + assign26100_e27278);
        (assign26100_e27279, ((locals.var_gc3ovacc_i * locals.var_temp1_dn4) + (locals.var_gc3ovinv_i * locals.var_temp2_dn4)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn6) + (locals.var_gc3ovinv_i * locals.var_temp2_dn6)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn7) + (locals.var_gc3ovinv_i * locals.var_temp2_dn7)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn8) + (locals.var_gc3ovinv_i * locals.var_temp2_dn8)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn9) + (locals.var_gc3ovinv_i * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gc3oveff, locals.var_gc3oveff_dn4, locals.var_gc3oveff_dn6, locals.var_gc3oveff_dn7, locals.var_gc3oveff_dn8, locals.var_gc3oveff_dn9,)
    }
};
        locals.var_gc3oveff = assign26100_e27281;
        locals.var_gc3oveff_dn4 = assign26100_e27281_d_n4;
        locals.var_gc3oveff_dn6 = assign26100_e27281_d_n6;
        locals.var_gc3oveff_dn7 = assign26100_e27281_d_n7;
        locals.var_gc3oveff_dn8 = assign26100_e27281_d_n8;
        locals.var_gc3oveff_dn9 = assign26100_e27281_d_n9;

        let (assign26110_e27293, assign26110_e27293_d_n4, assign26110_e27293_d_n6, assign26110_e27293_d_n7, assign26110_e27293_d_n8, assign26110_e27293_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26110_e27287: f64 = (locals.var_gcqovacc * locals.var_temp1);
        let assign26110_e27290: f64 = (locals.var_gcqovinv * locals.var_temp2);
        let assign26110_e27291: f64 = (assign26110_e27287 + assign26110_e27290);
        (assign26110_e27291, ((locals.var_gcqovacc * locals.var_temp1_dn4) + (locals.var_gcqovinv * locals.var_temp2_dn4)), ((locals.var_gcqovacc * locals.var_temp1_dn6) + (locals.var_gcqovinv * locals.var_temp2_dn6)), ((locals.var_gcqovacc * locals.var_temp1_dn7) + (locals.var_gcqovinv * locals.var_temp2_dn7)), ((locals.var_gcqovacc * locals.var_temp1_dn8) + (locals.var_gcqovinv * locals.var_temp2_dn8)), ((locals.var_gcqovacc * locals.var_temp1_dn9) + (locals.var_gcqovinv * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gcqoveff, locals.var_gcqoveff_dn4, locals.var_gcqoveff_dn6, locals.var_gcqoveff_dn7, locals.var_gcqoveff_dn8, locals.var_gcqoveff_dn9,)
    }
};
        locals.var_gcqoveff = assign26110_e27293;
        locals.var_gcqoveff_dn4 = assign26110_e27293_d_n4;
        locals.var_gcqoveff_dn6 = assign26110_e27293_d_n6;
        locals.var_gcqoveff_dn7 = assign26110_e27293_d_n7;
        locals.var_gcqoveff_dn8 = assign26110_e27293_d_n8;
        locals.var_gcqoveff_dn9 = assign26110_e27293_d_n9;

        let (assign26140_e27328, assign26140_e27328_d_n4, assign26140_e27328_d_n6, assign26140_e27328_d_n7, assign26140_e27328_d_n8, assign26140_e27328_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26140_e27321: f64 = (-1.0);
        let assign26140_e27323: f64 = (assign26140_e27321 * locals.var_gcovinvfn_i);
        let assign26140_e27325: f64 = (assign26140_e27323 / locals.var_zg);
        let assign26140_e27326: f64 = (locals.var_bov * assign26140_e27325);
        (assign26140_e27326, ((locals.var_bov_dn4 * assign26140_e27325) + (locals.var_bov * (-((assign26140_e27323 * locals.var_zg_dn4) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn6 * assign26140_e27325) + (locals.var_bov * (-((assign26140_e27323 * locals.var_zg_dn6) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn7 * assign26140_e27325) + (locals.var_bov * (-((assign26140_e27323 * locals.var_zg_dn7) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn8 * assign26140_e27325) + (locals.var_bov * (-((assign26140_e27323 * locals.var_zg_dn8) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn9 * assign26140_e27325) + (locals.var_bov * (-((assign26140_e27323 * locals.var_zg_dn9) / (locals.var_zg * locals.var_zg))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign26140_e27328;
        locals.var_temp1_dn4 = assign26140_e27328_d_n4;
        locals.var_temp1_dn6 = assign26140_e27328_d_n6;
        locals.var_temp1_dn7 = assign26140_e27328_d_n7;
        locals.var_temp1_dn8 = assign26140_e27328_d_n8;
        locals.var_temp1_dn9 = assign26140_e27328_d_n9;

        let assign26150_e27331: f64 = if locals.var_gc3oveff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign26150_e27331;

        let (assign26160_e27354, assign26160_e27354_d_n4, assign26160_e27354_d_n6, assign26160_e27354_d_n7, assign26160_e27354_d_n8, assign26160_e27354_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign26160_e27340: f64 = (locals.var_zg + locals.var_gcqoveff);
        let assign26160_e27343: f64 = (locals.var_zg - locals.var_gcqoveff);
        let assign26160_e27346: f64 = (locals.var_zg - locals.var_gcqoveff);
        let assign26160_e27347: f64 = (assign26160_e27343 * assign26160_e27346);
        let assign26160_e27349: f64 = (assign26160_e27347 + 1e-6);
        let assign26160_e27350: f64 = (assign26160_e27349).sqrt();
        let assign26160_e27351: f64 = (assign26160_e27340 - assign26160_e27350);
        let assign26160_e27352: f64 = (0.5 * assign26160_e27351);
        (assign26160_e27352, (0.5 * ((locals.var_zg_dn4 + locals.var_gcqoveff_dn4) - ((((locals.var_zg_dn4 - locals.var_gcqoveff_dn4) * assign26160_e27346) + (assign26160_e27343 * (locals.var_zg_dn4 - locals.var_gcqoveff_dn4))) / (2.0 * assign26160_e27350)))), (0.5 * ((locals.var_zg_dn6 + locals.var_gcqoveff_dn6) - ((((locals.var_zg_dn6 - locals.var_gcqoveff_dn6) * assign26160_e27346) + (assign26160_e27343 * (locals.var_zg_dn6 - locals.var_gcqoveff_dn6))) / (2.0 * assign26160_e27350)))), (0.5 * ((locals.var_zg_dn7 + locals.var_gcqoveff_dn7) - ((((locals.var_zg_dn7 - locals.var_gcqoveff_dn7) * assign26160_e27346) + (assign26160_e27343 * (locals.var_zg_dn7 - locals.var_gcqoveff_dn7))) / (2.0 * assign26160_e27350)))), (0.5 * ((locals.var_zg_dn8 + locals.var_gcqoveff_dn8) - ((((locals.var_zg_dn8 - locals.var_gcqoveff_dn8) * assign26160_e27346) + (assign26160_e27343 * (locals.var_zg_dn8 - locals.var_gcqoveff_dn8))) / (2.0 * assign26160_e27350)))), (0.5 * ((locals.var_zg_dn9 + locals.var_gcqoveff_dn9) - ((((locals.var_zg_dn9 - locals.var_gcqoveff_dn9) * assign26160_e27346) + (assign26160_e27343 * (locals.var_zg_dn9 - locals.var_gcqoveff_dn9))) / (2.0 * assign26160_e27350)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign26160_e27354;
        locals.var_zg_dn4 = assign26160_e27354_d_n4;
        locals.var_zg_dn6 = assign26160_e27354_d_n6;
        locals.var_zg_dn7 = assign26160_e27354_d_n7;
        locals.var_zg_dn8 = assign26160_e27354_d_n8;
        locals.var_zg_dn9 = assign26160_e27354_d_n9;

        let (assign26170_e27366, assign26170_e27366_d_n4, assign26170_e27366_d_n6, assign26170_e27366_d_n7, assign26170_e27366_d_n8, assign26170_e27366_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26170_e27360: f64 = (3.0 + locals.var_xd_ov);
        let assign26170_e27363: f64 = (locals.var_psi_t * locals.var_inv_phit0);
        let assign26170_e27364: f64 = (assign26170_e27360 + assign26170_e27363);
        (assign26170_e27364, (locals.var_xd_ov_dn4 + ((locals.var_psi_t_dn4 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn4))), (locals.var_xd_ov_dn6 + ((locals.var_psi_t_dn6 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn6))), (locals.var_xd_ov_dn7 + ((locals.var_psi_t_dn7 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn7))), (locals.var_xd_ov_dn8 + ((locals.var_psi_t_dn8 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn8))), (locals.var_xd_ov_dn9 + ((locals.var_psi_t_dn9 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn9))),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign26170_e27366;
        locals.var_arg1_dn4 = assign26170_e27366_d_n4;
        locals.var_arg1_dn6 = assign26170_e27366_d_n6;
        locals.var_arg1_dn7 = assign26170_e27366_d_n7;
        locals.var_arg1_dn8 = assign26170_e27366_d_n8;
        locals.var_arg1_dn9 = assign26170_e27366_d_n9;

        let assign26180_e27368: f64 = (locals.var_arg1).abs();
        let assign26180_e27370: f64 = if assign26180_e27368 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard743 = assign26180_e27370;

        let (assign26190_e27379, assign26190_e27379_d_n4, assign26190_e27379_d_n6, assign26190_e27379_d_n7, assign26190_e27379_d_n8, assign26190_e27379_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard743 != 0.0)) {
        let assign26190_e27377: f64 = (locals.var_arg1).exp();
        (assign26190_e27377, (assign26190_e27377 * locals.var_arg1_dn4), (assign26190_e27377 * locals.var_arg1_dn6), (assign26190_e27377 * locals.var_arg1_dn7), (assign26190_e27377 * locals.var_arg1_dn8), (assign26190_e27377 * locals.var_arg1_dn9),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26190_e27379;
        locals.var_dsi_dn4 = assign26190_e27379_d_n4;
        locals.var_dsi_dn6 = assign26190_e27379_d_n6;
        locals.var_dsi_dn7 = assign26190_e27379_d_n7;
        locals.var_dsi_dn8 = assign26190_e27379_d_n8;
        locals.var_dsi_dn9 = assign26190_e27379_d_n9;

        let assign26200_e27382: f64 = (-80.0);
        let assign26200_e27383: f64 = if locals.var_arg1 < assign26200_e27382 { 1.0 } else { 0.0 };
        locals.var_guard744 = assign26200_e27383;

        let (assign26210_e27419, assign26210_e27419_d_n4, assign26210_e27419_d_n6, assign26210_e27419_d_n7, assign26210_e27419_d_n8, assign26210_e27419_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard743 == 0.0)) && (locals.var_guard744 != 0.0)) {
        let assign26210_e27395: f64 = (-locals.var_arg1);
        let assign26210_e27397: f64 = (assign26210_e27395 - 80.0);
        let assign26210_e27401: f64 = (-locals.var_arg1);
        let assign26210_e27403: f64 = (assign26210_e27401 - 80.0);
        let assign26210_e27404: f64 = (0.5 * assign26210_e27403);
        let assign26210_e27407: f64 = (-locals.var_arg1);
        let assign26210_e27409: f64 = (assign26210_e27407 - 80.0);
        let assign26210_e27411: f64 = (assign26210_e27409 * 0.3333333333333);
        let assign26210_e27412: f64 = (1.0 + assign26210_e27411);
        let assign26210_e27413: f64 = (assign26210_e27404 * assign26210_e27412);
        let assign26210_e27414: f64 = (1.0 + assign26210_e27413);
        let assign26210_e27415: f64 = (assign26210_e27397 * assign26210_e27414);
        let assign26210_e27416: f64 = (1.0 + assign26210_e27415);
        let assign26210_e27417: f64 = (1.80485e-35 / assign26210_e27416);
        (assign26210_e27417, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign26210_e27414) + (assign26210_e27397 * (((0.5 * (-locals.var_arg1_dn4)) * assign26210_e27412) + (assign26210_e27404 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign26210_e27416 * assign26210_e27416))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign26210_e27414) + (assign26210_e27397 * (((0.5 * (-locals.var_arg1_dn6)) * assign26210_e27412) + (assign26210_e27404 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign26210_e27416 * assign26210_e27416))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign26210_e27414) + (assign26210_e27397 * (((0.5 * (-locals.var_arg1_dn7)) * assign26210_e27412) + (assign26210_e27404 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign26210_e27416 * assign26210_e27416))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign26210_e27414) + (assign26210_e27397 * (((0.5 * (-locals.var_arg1_dn8)) * assign26210_e27412) + (assign26210_e27404 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign26210_e27416 * assign26210_e27416))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign26210_e27414) + (assign26210_e27397 * (((0.5 * (-locals.var_arg1_dn9)) * assign26210_e27412) + (assign26210_e27404 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign26210_e27416 * assign26210_e27416))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26210_e27419;
        locals.var_dsi_dn4 = assign26210_e27419_d_n4;
        locals.var_dsi_dn6 = assign26210_e27419_d_n6;
        locals.var_dsi_dn7 = assign26210_e27419_d_n7;
        locals.var_dsi_dn8 = assign26210_e27419_d_n8;
        locals.var_dsi_dn9 = assign26210_e27419_d_n9;

        let (assign26220_e27453, assign26220_e27453_d_n4, assign26220_e27453_d_n6, assign26220_e27453_d_n7, assign26220_e27453_d_n8, assign26220_e27453_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard743 == 0.0)) && (locals.var_guard744 == 0.0)) {
        let assign26220_e27433: f64 = (locals.var_arg1 - 80.0);
        let assign26220_e27438: f64 = (locals.var_arg1 - 80.0);
        let assign26220_e27439: f64 = (0.5 * assign26220_e27438);
        let assign26220_e27443: f64 = (locals.var_arg1 - 80.0);
        let assign26220_e27445: f64 = (assign26220_e27443 * 0.3333333333333);
        let assign26220_e27446: f64 = (1.0 + assign26220_e27445);
        let assign26220_e27447: f64 = (assign26220_e27439 * assign26220_e27446);
        let assign26220_e27448: f64 = (1.0 + assign26220_e27447);
        let assign26220_e27449: f64 = (assign26220_e27433 * assign26220_e27448);
        let assign26220_e27450: f64 = (1.0 + assign26220_e27449);
        let assign26220_e27451: f64 = (5.54062e34 * assign26220_e27450);
        (assign26220_e27451, (5.54062e34 * ((locals.var_arg1_dn4 * assign26220_e27448) + (assign26220_e27433 * (((0.5 * locals.var_arg1_dn4) * assign26220_e27446) + (assign26220_e27439 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign26220_e27448) + (assign26220_e27433 * (((0.5 * locals.var_arg1_dn6) * assign26220_e27446) + (assign26220_e27439 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign26220_e27448) + (assign26220_e27433 * (((0.5 * locals.var_arg1_dn7) * assign26220_e27446) + (assign26220_e27439 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign26220_e27448) + (assign26220_e27433 * (((0.5 * locals.var_arg1_dn8) * assign26220_e27446) + (assign26220_e27439 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign26220_e27448) + (assign26220_e27433 * (((0.5 * locals.var_arg1_dn9) * assign26220_e27446) + (assign26220_e27439 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26220_e27453;
        locals.var_dsi_dn4 = assign26220_e27453_d_n4;
        locals.var_dsi_dn6 = assign26220_e27453_d_n6;
        locals.var_dsi_dn7 = assign26220_e27453_d_n7;
        locals.var_dsi_dn8 = assign26220_e27453_d_n8;
        locals.var_dsi_dn9 = assign26220_e27453_d_n9;

        let (assign26230_e27467, assign26230_e27467_d_n4, assign26230_e27467_d_n6, assign26230_e27467_d_n7, assign26230_e27467_d_n8, assign26230_e27467_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26230_e27459: f64 = (3.0 + locals.var_xd_ov);
        let assign26230_e27462: f64 = (locals.var_psi_t * locals.var_inv_phit0);
        let assign26230_e27463: f64 = (assign26230_e27459 + assign26230_e27462);
        let assign26230_e27465: f64 = (assign26230_e27463 + locals.var_xgd_ov);
        (assign26230_e27465, ((locals.var_xd_ov_dn4 + ((locals.var_psi_t_dn4 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn4))) + locals.var_xgd_ov_dn4), ((locals.var_xd_ov_dn6 + ((locals.var_psi_t_dn6 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn6))) + locals.var_xgd_ov_dn6), ((locals.var_xd_ov_dn7 + ((locals.var_psi_t_dn7 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn7))) + locals.var_xgd_ov_dn7), ((locals.var_xd_ov_dn8 + ((locals.var_psi_t_dn8 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn8))) + locals.var_xgd_ov_dn8), ((locals.var_xd_ov_dn9 + ((locals.var_psi_t_dn9 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn9))) + locals.var_xgd_ov_dn9),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign26230_e27467;
        locals.var_arg1_dn4 = assign26230_e27467_d_n4;
        locals.var_arg1_dn6 = assign26230_e27467_d_n6;
        locals.var_arg1_dn7 = assign26230_e27467_d_n7;
        locals.var_arg1_dn8 = assign26230_e27467_d_n8;
        locals.var_arg1_dn9 = assign26230_e27467_d_n9;

        let assign26240_e27469: f64 = (locals.var_arg1).abs();
        let assign26240_e27471: f64 = if assign26240_e27469 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign26240_e27471;

        let (assign26250_e27480, assign26250_e27480_d_n4, assign26250_e27480_d_n6, assign26250_e27480_d_n7, assign26250_e27480_d_n8, assign26250_e27480_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard745 != 0.0)) {
        let assign26250_e27478: f64 = (locals.var_arg1).exp();
        (assign26250_e27478, (assign26250_e27478 * locals.var_arg1_dn4), (assign26250_e27478 * locals.var_arg1_dn6), (assign26250_e27478 * locals.var_arg1_dn7), (assign26250_e27478 * locals.var_arg1_dn8), (assign26250_e27478 * locals.var_arg1_dn9),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign26250_e27480;
        locals.var_dgate_dn4 = assign26250_e27480_d_n4;
        locals.var_dgate_dn6 = assign26250_e27480_d_n6;
        locals.var_dgate_dn7 = assign26250_e27480_d_n7;
        locals.var_dgate_dn8 = assign26250_e27480_d_n8;
        locals.var_dgate_dn9 = assign26250_e27480_d_n9;

        let assign26260_e27483: f64 = (-80.0);
        let assign26260_e27484: f64 = if locals.var_arg1 < assign26260_e27483 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign26260_e27484;

        let (assign26270_e27520, assign26270_e27520_d_n4, assign26270_e27520_d_n6, assign26270_e27520_d_n7, assign26270_e27520_d_n8, assign26270_e27520_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 != 0.0)) {
        let assign26270_e27496: f64 = (-locals.var_arg1);
        let assign26270_e27498: f64 = (assign26270_e27496 - 80.0);
        let assign26270_e27502: f64 = (-locals.var_arg1);
        let assign26270_e27504: f64 = (assign26270_e27502 - 80.0);
        let assign26270_e27505: f64 = (0.5 * assign26270_e27504);
        let assign26270_e27508: f64 = (-locals.var_arg1);
        let assign26270_e27510: f64 = (assign26270_e27508 - 80.0);
        let assign26270_e27512: f64 = (assign26270_e27510 * 0.3333333333333);
        let assign26270_e27513: f64 = (1.0 + assign26270_e27512);
        let assign26270_e27514: f64 = (assign26270_e27505 * assign26270_e27513);
        let assign26270_e27515: f64 = (1.0 + assign26270_e27514);
        let assign26270_e27516: f64 = (assign26270_e27498 * assign26270_e27515);
        let assign26270_e27517: f64 = (1.0 + assign26270_e27516);
        let assign26270_e27518: f64 = (1.80485e-35 / assign26270_e27517);
        (assign26270_e27518, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign26270_e27515) + (assign26270_e27498 * (((0.5 * (-locals.var_arg1_dn4)) * assign26270_e27513) + (assign26270_e27505 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign26270_e27517 * assign26270_e27517))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign26270_e27515) + (assign26270_e27498 * (((0.5 * (-locals.var_arg1_dn6)) * assign26270_e27513) + (assign26270_e27505 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign26270_e27517 * assign26270_e27517))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign26270_e27515) + (assign26270_e27498 * (((0.5 * (-locals.var_arg1_dn7)) * assign26270_e27513) + (assign26270_e27505 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign26270_e27517 * assign26270_e27517))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign26270_e27515) + (assign26270_e27498 * (((0.5 * (-locals.var_arg1_dn8)) * assign26270_e27513) + (assign26270_e27505 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign26270_e27517 * assign26270_e27517))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign26270_e27515) + (assign26270_e27498 * (((0.5 * (-locals.var_arg1_dn9)) * assign26270_e27513) + (assign26270_e27505 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign26270_e27517 * assign26270_e27517))),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign26270_e27520;
        locals.var_dgate_dn4 = assign26270_e27520_d_n4;
        locals.var_dgate_dn6 = assign26270_e27520_d_n6;
        locals.var_dgate_dn7 = assign26270_e27520_d_n7;
        locals.var_dgate_dn8 = assign26270_e27520_d_n8;
        locals.var_dgate_dn9 = assign26270_e27520_d_n9;

        let (assign26280_e27554, assign26280_e27554_d_n4, assign26280_e27554_d_n6, assign26280_e27554_d_n7, assign26280_e27554_d_n8, assign26280_e27554_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) {
        let assign26280_e27534: f64 = (locals.var_arg1 - 80.0);
        let assign26280_e27539: f64 = (locals.var_arg1 - 80.0);
        let assign26280_e27540: f64 = (0.5 * assign26280_e27539);
        let assign26280_e27544: f64 = (locals.var_arg1 - 80.0);
        let assign26280_e27546: f64 = (assign26280_e27544 * 0.3333333333333);
        let assign26280_e27547: f64 = (1.0 + assign26280_e27546);
        let assign26280_e27548: f64 = (assign26280_e27540 * assign26280_e27547);
        let assign26280_e27549: f64 = (1.0 + assign26280_e27548);
        let assign26280_e27550: f64 = (assign26280_e27534 * assign26280_e27549);
        let assign26280_e27551: f64 = (1.0 + assign26280_e27550);
        let assign26280_e27552: f64 = (5.54062e34 * assign26280_e27551);
        (assign26280_e27552, (5.54062e34 * ((locals.var_arg1_dn4 * assign26280_e27549) + (assign26280_e27534 * (((0.5 * locals.var_arg1_dn4) * assign26280_e27547) + (assign26280_e27540 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign26280_e27549) + (assign26280_e27534 * (((0.5 * locals.var_arg1_dn6) * assign26280_e27547) + (assign26280_e27540 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign26280_e27549) + (assign26280_e27534 * (((0.5 * locals.var_arg1_dn7) * assign26280_e27547) + (assign26280_e27540 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign26280_e27549) + (assign26280_e27534 * (((0.5 * locals.var_arg1_dn8) * assign26280_e27547) + (assign26280_e27540 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign26280_e27549) + (assign26280_e27534 * (((0.5 * locals.var_arg1_dn9) * assign26280_e27547) + (assign26280_e27540 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign26280_e27554;
        locals.var_dgate_dn4 = assign26280_e27554_d_n4;
        locals.var_dgate_dn6 = assign26280_e27554_d_n6;
        locals.var_dgate_dn7 = assign26280_e27554_d_n7;
        locals.var_dgate_dn8 = assign26280_e27554_d_n8;
        locals.var_dgate_dn9 = assign26280_e27554_d_n9;

        let (assign26290_e27571, assign26290_e27571_d_n4, assign26290_e27571_d_n6, assign26290_e27571_d_n7, assign26290_e27571_d_n8, assign26290_e27571_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26290_e27560: f64 = (-1.5);
        let assign26290_e27565: f64 = (locals.var_gc3oveff * locals.var_zg);
        let assign26290_e27566: f64 = (locals.var_gc2oveff + assign26290_e27565);
        let assign26290_e27567: f64 = (locals.var_zg * assign26290_e27566);
        let assign26290_e27568: f64 = (assign26290_e27560 + assign26290_e27567);
        let assign26290_e27569: f64 = (locals.var_bov * assign26290_e27568);
        (assign26290_e27569, ((locals.var_bov_dn4 * assign26290_e27568) + (locals.var_bov * ((locals.var_zg_dn4 * assign26290_e27566) + (locals.var_zg * (locals.var_gc2oveff_dn4 + ((locals.var_gc3oveff_dn4 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn4))))))), ((locals.var_bov_dn6 * assign26290_e27568) + (locals.var_bov * ((locals.var_zg_dn6 * assign26290_e27566) + (locals.var_zg * (locals.var_gc2oveff_dn6 + ((locals.var_gc3oveff_dn6 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn6))))))), ((locals.var_bov_dn7 * assign26290_e27568) + (locals.var_bov * ((locals.var_zg_dn7 * assign26290_e27566) + (locals.var_zg * (locals.var_gc2oveff_dn7 + ((locals.var_gc3oveff_dn7 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn7))))))), ((locals.var_bov_dn8 * assign26290_e27568) + (locals.var_bov * ((locals.var_zg_dn8 * assign26290_e27566) + (locals.var_zg * (locals.var_gc2oveff_dn8 + ((locals.var_gc3oveff_dn8 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn8))))))), ((locals.var_bov_dn9 * assign26290_e27568) + (locals.var_bov * ((locals.var_zg_dn9 * assign26290_e27566) + (locals.var_zg * (locals.var_gc2oveff_dn9 + ((locals.var_gc3oveff_dn9 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn9))))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26290_e27571;
        locals.var_temp_dn4 = assign26290_e27571_d_n4;
        locals.var_temp_dn6 = assign26290_e27571_d_n6;
        locals.var_temp_dn7 = assign26290_e27571_d_n7;
        locals.var_temp_dn8 = assign26290_e27571_d_n8;
        locals.var_temp_dn9 = assign26290_e27571_d_n9;

        let (assign26400_e27739, assign26400_e27739_d_n4, assign26400_e27739_d_n6, assign26400_e27739_d_n7, assign26400_e27739_d_n8, assign26400_e27739_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26400_e27733: f64 = (1.0 + locals.var_dsi);
        let assign26400_e27736: f64 = (1.0 + locals.var_dgate);
        let assign26400_e27737: f64 = (assign26400_e27733 / assign26400_e27736);
        (assign26400_e27737, (((locals.var_dsi_dn4 * assign26400_e27736) - (assign26400_e27733 * locals.var_dgate_dn4)) / (assign26400_e27736 * assign26400_e27736)), (((locals.var_dsi_dn6 * assign26400_e27736) - (assign26400_e27733 * locals.var_dgate_dn6)) / (assign26400_e27736 * assign26400_e27736)), (((locals.var_dsi_dn7 * assign26400_e27736) - (assign26400_e27733 * locals.var_dgate_dn7)) / (assign26400_e27736 * assign26400_e27736)), (((locals.var_dsi_dn8 * assign26400_e27736) - (assign26400_e27733 * locals.var_dgate_dn8)) / (assign26400_e27736 * assign26400_e27736)), (((locals.var_dsi_dn9 * assign26400_e27736) - (assign26400_e27733 * locals.var_dgate_dn9)) / (assign26400_e27736 * assign26400_e27736)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26400_e27739;
        locals.var_temp_dn4 = assign26400_e27739_d_n4;
        locals.var_temp_dn6 = assign26400_e27739_d_n6;
        locals.var_temp_dn7 = assign26400_e27739_d_n7;
        locals.var_temp_dn8 = assign26400_e27739_d_n8;
        locals.var_temp_dn9 = assign26400_e27739_d_n9;

        let assign26410_e27742: f64 = if locals.var_temp < 1e-80 { 1.0 } else { 0.0 };
        locals.var_guard751 = assign26410_e27742;

        let (assign26420_e27750, assign26420_e27750_d_n4, assign26420_e27750_d_n6, assign26420_e27750_d_n7, assign26420_e27750_d_n8, assign26420_e27750_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard751 != 0.0)) {
        (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26420_e27750;
        locals.var_temp_dn4 = assign26420_e27750_d_n4;
        locals.var_temp_dn6 = assign26420_e27750_d_n6;
        locals.var_temp_dn7 = assign26420_e27750_d_n7;
        locals.var_temp_dn8 = assign26420_e27750_d_n8;
        locals.var_temp_dn9 = assign26420_e27750_d_n9;

    }

    pub(super) fn stamp_transient_block_67(
        locals: &mut StampLocals,
    ) {
        let (assign26430_e27760, assign26430_e27760_d_n4, assign26430_e27760_d_n6, assign26430_e27760_d_n7, assign26430_e27760_d_n8, assign26430_e27760_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26430_e27757: f64 = (locals.var_vgsu - locals.var_gcvdov_i);
        let assign26430_e27758: f64 = (locals.var_gcdov_i * assign26430_e27757);
        (assign26430_e27758, 0.0, (locals.var_gcdov_i * locals.var_vgsu_dn6), 0.0, 0.0, (locals.var_gcdov_i * locals.var_vgsu_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign26430_e27760;
        locals.var_temp1_dn4 = assign26430_e27760_d_n4;
        locals.var_temp1_dn6 = assign26430_e27760_d_n6;
        locals.var_temp1_dn7 = assign26430_e27760_d_n7;
        locals.var_temp1_dn8 = assign26430_e27760_d_n8;
        locals.var_temp1_dn9 = assign26430_e27760_d_n9;

        let assign26440_e27762: f64 = (locals.var_temp1).abs();
        let assign26440_e27764: f64 = if assign26440_e27762 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard752 = assign26440_e27764;

        let (assign26450_e27773, assign26450_e27773_d_n4, assign26450_e27773_d_n6, assign26450_e27773_d_n7, assign26450_e27773_d_n8, assign26450_e27773_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard752 != 0.0)) {
        let assign26450_e27771: f64 = (locals.var_temp1).exp();
        (assign26450_e27771, (assign26450_e27771 * locals.var_temp1_dn4), (assign26450_e27771 * locals.var_temp1_dn6), (assign26450_e27771 * locals.var_temp1_dn7), (assign26450_e27771 * locals.var_temp1_dn8), (assign26450_e27771 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign26450_e27773;
        locals.var_temp2_dn4 = assign26450_e27773_d_n4;
        locals.var_temp2_dn6 = assign26450_e27773_d_n6;
        locals.var_temp2_dn7 = assign26450_e27773_d_n7;
        locals.var_temp2_dn8 = assign26450_e27773_d_n8;
        locals.var_temp2_dn9 = assign26450_e27773_d_n9;

        let assign26460_e27776: f64 = (-80.0);
        let assign26460_e27777: f64 = if locals.var_temp1 < assign26460_e27776 { 1.0 } else { 0.0 };
        locals.var_guard753 = assign26460_e27777;

        let (assign26470_e27813, assign26470_e27813_d_n4, assign26470_e27813_d_n6, assign26470_e27813_d_n7, assign26470_e27813_d_n8, assign26470_e27813_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard752 == 0.0)) && (locals.var_guard753 != 0.0)) {
        let assign26470_e27789: f64 = (-locals.var_temp1);
        let assign26470_e27791: f64 = (assign26470_e27789 - 80.0);
        let assign26470_e27795: f64 = (-locals.var_temp1);
        let assign26470_e27797: f64 = (assign26470_e27795 - 80.0);
        let assign26470_e27798: f64 = (0.5 * assign26470_e27797);
        let assign26470_e27801: f64 = (-locals.var_temp1);
        let assign26470_e27803: f64 = (assign26470_e27801 - 80.0);
        let assign26470_e27805: f64 = (assign26470_e27803 * 0.3333333333333);
        let assign26470_e27806: f64 = (1.0 + assign26470_e27805);
        let assign26470_e27807: f64 = (assign26470_e27798 * assign26470_e27806);
        let assign26470_e27808: f64 = (1.0 + assign26470_e27807);
        let assign26470_e27809: f64 = (assign26470_e27791 * assign26470_e27808);
        let assign26470_e27810: f64 = (1.0 + assign26470_e27809);
        let assign26470_e27811: f64 = (1.80485e-35 / assign26470_e27810);
        (assign26470_e27811, (-((1.80485e-35 * (((-locals.var_temp1_dn4) * assign26470_e27808) + (assign26470_e27791 * (((0.5 * (-locals.var_temp1_dn4)) * assign26470_e27806) + (assign26470_e27798 * ((-locals.var_temp1_dn4) * 0.3333333333333)))))) / (assign26470_e27810 * assign26470_e27810))), (-((1.80485e-35 * (((-locals.var_temp1_dn6) * assign26470_e27808) + (assign26470_e27791 * (((0.5 * (-locals.var_temp1_dn6)) * assign26470_e27806) + (assign26470_e27798 * ((-locals.var_temp1_dn6) * 0.3333333333333)))))) / (assign26470_e27810 * assign26470_e27810))), (-((1.80485e-35 * (((-locals.var_temp1_dn7) * assign26470_e27808) + (assign26470_e27791 * (((0.5 * (-locals.var_temp1_dn7)) * assign26470_e27806) + (assign26470_e27798 * ((-locals.var_temp1_dn7) * 0.3333333333333)))))) / (assign26470_e27810 * assign26470_e27810))), (-((1.80485e-35 * (((-locals.var_temp1_dn8) * assign26470_e27808) + (assign26470_e27791 * (((0.5 * (-locals.var_temp1_dn8)) * assign26470_e27806) + (assign26470_e27798 * ((-locals.var_temp1_dn8) * 0.3333333333333)))))) / (assign26470_e27810 * assign26470_e27810))), (-((1.80485e-35 * (((-locals.var_temp1_dn9) * assign26470_e27808) + (assign26470_e27791 * (((0.5 * (-locals.var_temp1_dn9)) * assign26470_e27806) + (assign26470_e27798 * ((-locals.var_temp1_dn9) * 0.3333333333333)))))) / (assign26470_e27810 * assign26470_e27810))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign26470_e27813;
        locals.var_temp2_dn4 = assign26470_e27813_d_n4;
        locals.var_temp2_dn6 = assign26470_e27813_d_n6;
        locals.var_temp2_dn7 = assign26470_e27813_d_n7;
        locals.var_temp2_dn8 = assign26470_e27813_d_n8;
        locals.var_temp2_dn9 = assign26470_e27813_d_n9;

        let (assign26480_e27847, assign26480_e27847_d_n4, assign26480_e27847_d_n6, assign26480_e27847_d_n7, assign26480_e27847_d_n8, assign26480_e27847_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard752 == 0.0)) && (locals.var_guard753 == 0.0)) {
        let assign26480_e27827: f64 = (locals.var_temp1 - 80.0);
        let assign26480_e27832: f64 = (locals.var_temp1 - 80.0);
        let assign26480_e27833: f64 = (0.5 * assign26480_e27832);
        let assign26480_e27837: f64 = (locals.var_temp1 - 80.0);
        let assign26480_e27839: f64 = (assign26480_e27837 * 0.3333333333333);
        let assign26480_e27840: f64 = (1.0 + assign26480_e27839);
        let assign26480_e27841: f64 = (assign26480_e27833 * assign26480_e27840);
        let assign26480_e27842: f64 = (1.0 + assign26480_e27841);
        let assign26480_e27843: f64 = (assign26480_e27827 * assign26480_e27842);
        let assign26480_e27844: f64 = (1.0 + assign26480_e27843);
        let assign26480_e27845: f64 = (5.54062e34 * assign26480_e27844);
        (assign26480_e27845, (5.54062e34 * ((locals.var_temp1_dn4 * assign26480_e27842) + (assign26480_e27827 * (((0.5 * locals.var_temp1_dn4) * assign26480_e27840) + (assign26480_e27833 * (locals.var_temp1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn6 * assign26480_e27842) + (assign26480_e27827 * (((0.5 * locals.var_temp1_dn6) * assign26480_e27840) + (assign26480_e27833 * (locals.var_temp1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn7 * assign26480_e27842) + (assign26480_e27827 * (((0.5 * locals.var_temp1_dn7) * assign26480_e27840) + (assign26480_e27833 * (locals.var_temp1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn8 * assign26480_e27842) + (assign26480_e27827 * (((0.5 * locals.var_temp1_dn8) * assign26480_e27840) + (assign26480_e27833 * (locals.var_temp1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn9 * assign26480_e27842) + (assign26480_e27827 * (((0.5 * locals.var_temp1_dn9) * assign26480_e27840) + (assign26480_e27833 * (locals.var_temp1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign26480_e27847;
        locals.var_temp2_dn4 = assign26480_e27847_d_n4;
        locals.var_temp2_dn6 = assign26480_e27847_d_n6;
        locals.var_temp2_dn7 = assign26480_e27847_d_n7;
        locals.var_temp2_dn8 = assign26480_e27847_d_n8;
        locals.var_temp2_dn9 = assign26480_e27847_d_n9;

        let (assign26490_e27857, assign26490_e27857_d_n4, assign26490_e27857_d_n6, assign26490_e27857_d_n7, assign26490_e27857_d_n8, assign26490_e27857_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26490_e27853: f64 = (locals.var_gcdov_i * locals.var_vdsu);
        let assign26490_e27855: f64 = (assign26490_e27853 + locals.var_temp1);
        (assign26490_e27855, locals.var_temp1_dn4, ((locals.var_gcdov_i * locals.var_vdsu_dn6) + locals.var_temp1_dn6), ((locals.var_gcdov_i * locals.var_vdsu_dn7) + locals.var_temp1_dn7), locals.var_temp1_dn8, locals.var_temp1_dn9,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign26490_e27857;
        locals.var_temp3_dn4 = assign26490_e27857_d_n4;
        locals.var_temp3_dn6 = assign26490_e27857_d_n6;
        locals.var_temp3_dn7 = assign26490_e27857_d_n7;
        locals.var_temp3_dn8 = assign26490_e27857_d_n8;
        locals.var_temp3_dn9 = assign26490_e27857_d_n9;

        let assign26500_e27859: f64 = (locals.var_temp3).abs();
        let assign26500_e27861: f64 = if assign26500_e27859 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard754 = assign26500_e27861;

        let (assign26510_e27870, assign26510_e27870_d_n4, assign26510_e27870_d_n6, assign26510_e27870_d_n7, assign26510_e27870_d_n8, assign26510_e27870_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard754 != 0.0)) {
        let assign26510_e27868: f64 = (locals.var_temp3).exp();
        (assign26510_e27868, (assign26510_e27868 * locals.var_temp3_dn4), (assign26510_e27868 * locals.var_temp3_dn6), (assign26510_e27868 * locals.var_temp3_dn7), (assign26510_e27868 * locals.var_temp3_dn8), (assign26510_e27868 * locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign26510_e27870;
        locals.var_temp4_dn4 = assign26510_e27870_d_n4;
        locals.var_temp4_dn6 = assign26510_e27870_d_n6;
        locals.var_temp4_dn7 = assign26510_e27870_d_n7;
        locals.var_temp4_dn8 = assign26510_e27870_d_n8;
        locals.var_temp4_dn9 = assign26510_e27870_d_n9;

        let assign26520_e27873: f64 = (-80.0);
        let assign26520_e27874: f64 = if locals.var_temp3 < assign26520_e27873 { 1.0 } else { 0.0 };
        locals.var_guard755 = assign26520_e27874;

        let (assign26530_e27910, assign26530_e27910_d_n4, assign26530_e27910_d_n6, assign26530_e27910_d_n7, assign26530_e27910_d_n8, assign26530_e27910_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard754 == 0.0)) && (locals.var_guard755 != 0.0)) {
        let assign26530_e27886: f64 = (-locals.var_temp3);
        let assign26530_e27888: f64 = (assign26530_e27886 - 80.0);
        let assign26530_e27892: f64 = (-locals.var_temp3);
        let assign26530_e27894: f64 = (assign26530_e27892 - 80.0);
        let assign26530_e27895: f64 = (0.5 * assign26530_e27894);
        let assign26530_e27898: f64 = (-locals.var_temp3);
        let assign26530_e27900: f64 = (assign26530_e27898 - 80.0);
        let assign26530_e27902: f64 = (assign26530_e27900 * 0.3333333333333);
        let assign26530_e27903: f64 = (1.0 + assign26530_e27902);
        let assign26530_e27904: f64 = (assign26530_e27895 * assign26530_e27903);
        let assign26530_e27905: f64 = (1.0 + assign26530_e27904);
        let assign26530_e27906: f64 = (assign26530_e27888 * assign26530_e27905);
        let assign26530_e27907: f64 = (1.0 + assign26530_e27906);
        let assign26530_e27908: f64 = (1.80485e-35 / assign26530_e27907);
        (assign26530_e27908, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign26530_e27905) + (assign26530_e27888 * (((0.5 * (-locals.var_temp3_dn4)) * assign26530_e27903) + (assign26530_e27895 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign26530_e27907 * assign26530_e27907))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign26530_e27905) + (assign26530_e27888 * (((0.5 * (-locals.var_temp3_dn6)) * assign26530_e27903) + (assign26530_e27895 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign26530_e27907 * assign26530_e27907))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign26530_e27905) + (assign26530_e27888 * (((0.5 * (-locals.var_temp3_dn7)) * assign26530_e27903) + (assign26530_e27895 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign26530_e27907 * assign26530_e27907))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign26530_e27905) + (assign26530_e27888 * (((0.5 * (-locals.var_temp3_dn8)) * assign26530_e27903) + (assign26530_e27895 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign26530_e27907 * assign26530_e27907))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign26530_e27905) + (assign26530_e27888 * (((0.5 * (-locals.var_temp3_dn9)) * assign26530_e27903) + (assign26530_e27895 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign26530_e27907 * assign26530_e27907))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign26530_e27910;
        locals.var_temp4_dn4 = assign26530_e27910_d_n4;
        locals.var_temp4_dn6 = assign26530_e27910_d_n6;
        locals.var_temp4_dn7 = assign26530_e27910_d_n7;
        locals.var_temp4_dn8 = assign26530_e27910_d_n8;
        locals.var_temp4_dn9 = assign26530_e27910_d_n9;

        let (assign26540_e27944, assign26540_e27944_d_n4, assign26540_e27944_d_n6, assign26540_e27944_d_n7, assign26540_e27944_d_n8, assign26540_e27944_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard754 == 0.0)) && (locals.var_guard755 == 0.0)) {
        let assign26540_e27924: f64 = (locals.var_temp3 - 80.0);
        let assign26540_e27929: f64 = (locals.var_temp3 - 80.0);
        let assign26540_e27930: f64 = (0.5 * assign26540_e27929);
        let assign26540_e27934: f64 = (locals.var_temp3 - 80.0);
        let assign26540_e27936: f64 = (assign26540_e27934 * 0.3333333333333);
        let assign26540_e27937: f64 = (1.0 + assign26540_e27936);
        let assign26540_e27938: f64 = (assign26540_e27930 * assign26540_e27937);
        let assign26540_e27939: f64 = (1.0 + assign26540_e27938);
        let assign26540_e27940: f64 = (assign26540_e27924 * assign26540_e27939);
        let assign26540_e27941: f64 = (1.0 + assign26540_e27940);
        let assign26540_e27942: f64 = (5.54062e34 * assign26540_e27941);
        (assign26540_e27942, (5.54062e34 * ((locals.var_temp3_dn4 * assign26540_e27939) + (assign26540_e27924 * (((0.5 * locals.var_temp3_dn4) * assign26540_e27937) + (assign26540_e27930 * (locals.var_temp3_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn6 * assign26540_e27939) + (assign26540_e27924 * (((0.5 * locals.var_temp3_dn6) * assign26540_e27937) + (assign26540_e27930 * (locals.var_temp3_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn7 * assign26540_e27939) + (assign26540_e27924 * (((0.5 * locals.var_temp3_dn7) * assign26540_e27937) + (assign26540_e27930 * (locals.var_temp3_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn8 * assign26540_e27939) + (assign26540_e27924 * (((0.5 * locals.var_temp3_dn8) * assign26540_e27937) + (assign26540_e27930 * (locals.var_temp3_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn9 * assign26540_e27939) + (assign26540_e27924 * (((0.5 * locals.var_temp3_dn9) * assign26540_e27937) + (assign26540_e27930 * (locals.var_temp3_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign26540_e27944;
        locals.var_temp4_dn4 = assign26540_e27944_d_n4;
        locals.var_temp4_dn6 = assign26540_e27944_d_n6;
        locals.var_temp4_dn7 = assign26540_e27944_d_n7;
        locals.var_temp4_dn8 = assign26540_e27944_d_n8;
        locals.var_temp4_dn9 = assign26540_e27944_d_n9;

        let assign26560_e27978: f64 = if locals.var_iginv_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard756 = assign26560_e27978;

        let (assign26570_e27987, assign26570_e27987_d_n4, assign26570_e27987_d_n6, assign26570_e27987_d_n7, assign26570_e27987_d_n8, assign26570_e27987_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26570_e27983: f64 = (-locals.var_delta_k1q1_dc);
        let assign26570_e27985: f64 = (assign26570_e27983 * locals.var_inv_k1_dc);
        (assign26570_e27985, (((-locals.var_delta_k1q1_dc_dn4) * locals.var_inv_k1_dc) + (assign26570_e27983 * locals.var_inv_k1_dc_dn4)), (((-locals.var_delta_k1q1_dc_dn6) * locals.var_inv_k1_dc) + (assign26570_e27983 * locals.var_inv_k1_dc_dn6)), (((-locals.var_delta_k1q1_dc_dn7) * locals.var_inv_k1_dc) + (assign26570_e27983 * locals.var_inv_k1_dc_dn7)), (((-locals.var_delta_k1q1_dc_dn8) * locals.var_inv_k1_dc) + (assign26570_e27983 * locals.var_inv_k1_dc_dn8)), (((-locals.var_delta_k1q1_dc_dn9) * locals.var_inv_k1_dc) + (assign26570_e27983 * locals.var_inv_k1_dc_dn9)),)
    } else {
        (locals.var_half_x_ds, locals.var_half_x_ds_dn4, locals.var_half_x_ds_dn6, locals.var_half_x_ds_dn7, locals.var_half_x_ds_dn8, locals.var_half_x_ds_dn9,)
    }
};
        locals.var_half_x_ds = assign26570_e27987;
        locals.var_half_x_ds_dn4 = assign26570_e27987_d_n4;
        locals.var_half_x_ds_dn6 = assign26570_e27987_d_n6;
        locals.var_half_x_ds_dn7 = assign26570_e27987_d_n7;
        locals.var_half_x_ds_dn8 = assign26570_e27987_d_n8;
        locals.var_half_x_ds_dn9 = assign26570_e27987_d_n9;

        let assign26580_e27990: f64 = (2.0 * locals.var_half_x_ds);
        let assign26580_e27992: f64 = (assign26580_e27990 - locals.var_xdeff_dc);
        let assign26580_e27993: f64 = (assign26580_e27992).abs();
        let assign26580_e27995: f64 = if assign26580_e27993 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard757 = assign26580_e27995;

        let (assign26590_e28008, assign26590_e28008_d_n4, assign26590_e28008_d_n6, assign26590_e28008_d_n7, assign26590_e28008_d_n8, assign26590_e28008_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign26590_e28003: f64 = (2.0 * locals.var_half_x_ds);
        let assign26590_e28005: f64 = (assign26590_e28003 - locals.var_xdeff_dc);
        let assign26590_e28006: f64 = (assign26590_e28005).exp();
        (assign26590_e28006, (assign26590_e28006 * ((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4)), (assign26590_e28006 * ((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6)), (assign26590_e28006 * ((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7)), (assign26590_e28006 * ((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8)), (assign26590_e28006 * ((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26590_e28008;
        locals.var_temp_dn4 = assign26590_e28008_d_n4;
        locals.var_temp_dn6 = assign26590_e28008_d_n6;
        locals.var_temp_dn7 = assign26590_e28008_d_n7;
        locals.var_temp_dn8 = assign26590_e28008_d_n8;
        locals.var_temp_dn9 = assign26590_e28008_d_n9;

        let assign26600_e28011: f64 = (2.0 * locals.var_half_x_ds);
        let assign26600_e28013: f64 = (assign26600_e28011 - locals.var_xdeff_dc);
        let assign26600_e28015: f64 = (-80.0);
        let assign26600_e28016: f64 = if assign26600_e28013 < assign26600_e28015 { 1.0 } else { 0.0 };
        locals.var_guard758 = assign26600_e28016;

        let (assign26610_e28064, assign26610_e28064_d_n4, assign26610_e28064_d_n6, assign26610_e28064_d_n7, assign26610_e28064_d_n8, assign26610_e28064_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard757 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign26610_e28029: f64 = (2.0 * locals.var_half_x_ds);
        let assign26610_e28031: f64 = (assign26610_e28029 - locals.var_xdeff_dc);
        let assign26610_e28032: f64 = (-assign26610_e28031);
        let assign26610_e28034: f64 = (assign26610_e28032 - 80.0);
        let assign26610_e28039: f64 = (2.0 * locals.var_half_x_ds);
        let assign26610_e28041: f64 = (assign26610_e28039 - locals.var_xdeff_dc);
        let assign26610_e28042: f64 = (-assign26610_e28041);
        let assign26610_e28044: f64 = (assign26610_e28042 - 80.0);
        let assign26610_e28045: f64 = (0.5 * assign26610_e28044);
        let assign26610_e28049: f64 = (2.0 * locals.var_half_x_ds);
        let assign26610_e28051: f64 = (assign26610_e28049 - locals.var_xdeff_dc);
        let assign26610_e28052: f64 = (-assign26610_e28051);
        let assign26610_e28054: f64 = (assign26610_e28052 - 80.0);
        let assign26610_e28056: f64 = (assign26610_e28054 * 0.3333333333333);
        let assign26610_e28057: f64 = (1.0 + assign26610_e28056);
        let assign26610_e28058: f64 = (assign26610_e28045 * assign26610_e28057);
        let assign26610_e28059: f64 = (1.0 + assign26610_e28058);
        let assign26610_e28060: f64 = (assign26610_e28034 * assign26610_e28059);
        let assign26610_e28061: f64 = (1.0 + assign26610_e28060);
        let assign26610_e28062: f64 = (1.80485e-35 / assign26610_e28061);
        (assign26610_e28062, (-((1.80485e-35 * (((-((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4)) * assign26610_e28059) + (assign26610_e28034 * (((0.5 * (-((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4))) * assign26610_e28057) + (assign26610_e28045 * ((-((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4)) * 0.3333333333333)))))) / (assign26610_e28061 * assign26610_e28061))), (-((1.80485e-35 * (((-((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6)) * assign26610_e28059) + (assign26610_e28034 * (((0.5 * (-((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6))) * assign26610_e28057) + (assign26610_e28045 * ((-((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6)) * 0.3333333333333)))))) / (assign26610_e28061 * assign26610_e28061))), (-((1.80485e-35 * (((-((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7)) * assign26610_e28059) + (assign26610_e28034 * (((0.5 * (-((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7))) * assign26610_e28057) + (assign26610_e28045 * ((-((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7)) * 0.3333333333333)))))) / (assign26610_e28061 * assign26610_e28061))), (-((1.80485e-35 * (((-((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8)) * assign26610_e28059) + (assign26610_e28034 * (((0.5 * (-((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8))) * assign26610_e28057) + (assign26610_e28045 * ((-((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8)) * 0.3333333333333)))))) / (assign26610_e28061 * assign26610_e28061))), (-((1.80485e-35 * (((-((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9)) * assign26610_e28059) + (assign26610_e28034 * (((0.5 * (-((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9))) * assign26610_e28057) + (assign26610_e28045 * ((-((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9)) * 0.3333333333333)))))) / (assign26610_e28061 * assign26610_e28061))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26610_e28064;
        locals.var_temp_dn4 = assign26610_e28064_d_n4;
        locals.var_temp_dn6 = assign26610_e28064_d_n6;
        locals.var_temp_dn7 = assign26610_e28064_d_n7;
        locals.var_temp_dn8 = assign26610_e28064_d_n8;
        locals.var_temp_dn9 = assign26610_e28064_d_n9;

        let (assign26620_e28110, assign26620_e28110_d_n4, assign26620_e28110_d_n6, assign26620_e28110_d_n7, assign26620_e28110_d_n8, assign26620_e28110_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard757 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign26620_e28078: f64 = (2.0 * locals.var_half_x_ds);
        let assign26620_e28080: f64 = (assign26620_e28078 - locals.var_xdeff_dc);
        let assign26620_e28082: f64 = (assign26620_e28080 - 80.0);
        let assign26620_e28087: f64 = (2.0 * locals.var_half_x_ds);
        let assign26620_e28089: f64 = (assign26620_e28087 - locals.var_xdeff_dc);
        let assign26620_e28091: f64 = (assign26620_e28089 - 80.0);
        let assign26620_e28092: f64 = (0.5 * assign26620_e28091);
        let assign26620_e28096: f64 = (2.0 * locals.var_half_x_ds);
        let assign26620_e28098: f64 = (assign26620_e28096 - locals.var_xdeff_dc);
        let assign26620_e28100: f64 = (assign26620_e28098 - 80.0);
        let assign26620_e28102: f64 = (assign26620_e28100 * 0.3333333333333);
        let assign26620_e28103: f64 = (1.0 + assign26620_e28102);
        let assign26620_e28104: f64 = (assign26620_e28092 * assign26620_e28103);
        let assign26620_e28105: f64 = (1.0 + assign26620_e28104);
        let assign26620_e28106: f64 = (assign26620_e28082 * assign26620_e28105);
        let assign26620_e28107: f64 = (1.0 + assign26620_e28106);
        let assign26620_e28108: f64 = (5.54062e34 * assign26620_e28107);
        (assign26620_e28108, (5.54062e34 * ((((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4) * assign26620_e28105) + (assign26620_e28082 * (((0.5 * ((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4)) * assign26620_e28103) + (assign26620_e28092 * (((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6) * assign26620_e28105) + (assign26620_e28082 * (((0.5 * ((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6)) * assign26620_e28103) + (assign26620_e28092 * (((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7) * assign26620_e28105) + (assign26620_e28082 * (((0.5 * ((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7)) * assign26620_e28103) + (assign26620_e28092 * (((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8) * assign26620_e28105) + (assign26620_e28082 * (((0.5 * ((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8)) * assign26620_e28103) + (assign26620_e28092 * (((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9) * assign26620_e28105) + (assign26620_e28082 * (((0.5 * ((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9)) * assign26620_e28103) + (assign26620_e28092 * (((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26620_e28110;
        locals.var_temp_dn4 = assign26620_e28110_d_n4;
        locals.var_temp_dn6 = assign26620_e28110_d_n6;
        locals.var_temp_dn7 = assign26620_e28110_d_n7;
        locals.var_temp_dn8 = assign26620_e28110_d_n8;
        locals.var_temp_dn9 = assign26620_e28110_d_n9;

        let (assign26630_e28125, assign26630_e28125_d_n4, assign26630_e28125_d_n6, assign26630_e28125_d_n7, assign26630_e28125_d_n8, assign26630_e28125_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26630_e28117: f64 = (locals.var_half_x_ds + 0.6931471805599);
        let assign26630_e28120: f64 = (1.0 + locals.var_temp);
        let assign26630_e28121: f64 = (assign26630_e28120).ln();
        let assign26630_e28122: f64 = (assign26630_e28117 - assign26630_e28121);
        let assign26630_e28123: f64 = (locals.var_phit * assign26630_e28122);
        (assign26630_e28123, ((locals.var_phit_dn4 * assign26630_e28122) + (locals.var_phit * (locals.var_half_x_ds_dn4 - (locals.var_temp_dn4 / assign26630_e28120)))), ((locals.var_phit_dn6 * assign26630_e28122) + (locals.var_phit * (locals.var_half_x_ds_dn6 - (locals.var_temp_dn6 / assign26630_e28120)))), ((locals.var_phit_dn7 * assign26630_e28122) + (locals.var_phit * (locals.var_half_x_ds_dn7 - (locals.var_temp_dn7 / assign26630_e28120)))), ((locals.var_phit_dn8 * assign26630_e28122) + (locals.var_phit * (locals.var_half_x_ds_dn8 - (locals.var_temp_dn8 / assign26630_e28120)))), ((locals.var_phit_dn9 * assign26630_e28122) + (locals.var_phit * (locals.var_half_x_ds_dn9 - (locals.var_temp_dn9 / assign26630_e28120)))),)
    } else {
        (locals.var_vm, locals.var_vm_dn4, locals.var_vm_dn6, locals.var_vm_dn7, locals.var_vm_dn8, locals.var_vm_dn9,)
    }
};
        locals.var_vm = assign26630_e28125;
        locals.var_vm_dn4 = assign26630_e28125_d_n4;
        locals.var_vm_dn6 = assign26630_e28125_d_n6;
        locals.var_vm_dn7 = assign26630_e28125_d_n7;
        locals.var_vm_dn8 = assign26630_e28125_d_n8;
        locals.var_vm_dn9 = assign26630_e28125_d_n9;

        let (assign26640_e28135, assign26640_e28135_d_n4, assign26640_e28135_d_n6, assign26640_e28135_d_n7, assign26640_e28135_d_n8, assign26640_e28135_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26640_e28132: f64 = (locals.var_q1s_dc + locals.var_q1d_dc);
        let assign26640_e28133: f64 = (0.5 * assign26640_e28132);
        (assign26640_e28133, (0.5 * (locals.var_q1s_dc_dn4 + locals.var_q1d_dc_dn4)), (0.5 * (locals.var_q1s_dc_dn6 + locals.var_q1d_dc_dn6)), (0.5 * (locals.var_q1s_dc_dn7 + locals.var_q1d_dc_dn7)), (0.5 * (locals.var_q1s_dc_dn8 + locals.var_q1d_dc_dn8)), (0.5 * (locals.var_q1s_dc_dn9 + locals.var_q1d_dc_dn9)),)
    } else {
        (locals.var_q1m, locals.var_q1m_dn4, locals.var_q1m_dn6, locals.var_q1m_dn7, locals.var_q1m_dn8, locals.var_q1m_dn9,)
    }
};
        locals.var_q1m = assign26640_e28135;
        locals.var_q1m_dn4 = assign26640_e28135_d_n4;
        locals.var_q1m_dn6 = assign26640_e28135_d_n6;
        locals.var_q1m_dn7 = assign26640_e28135_d_n7;
        locals.var_q1m_dn8 = assign26640_e28135_d_n8;
        locals.var_q1m_dn9 = assign26640_e28135_d_n9;

        let (assign26650_e28143, assign26650_e28143_d_n4, assign26650_e28143_d_n6, assign26650_e28143_d_n7, assign26650_e28143_d_n8, assign26650_e28143_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26650_e28141: f64 = (locals.var_phit * locals.var_q1m);
        (assign26650_e28141, ((locals.var_phit_dn4 * locals.var_q1m) + (locals.var_phit * locals.var_q1m_dn4)), ((locals.var_phit_dn6 * locals.var_q1m) + (locals.var_phit * locals.var_q1m_dn6)), ((locals.var_phit_dn7 * locals.var_q1m) + (locals.var_phit * locals.var_q1m_dn7)), ((locals.var_phit_dn8 * locals.var_q1m) + (locals.var_phit * locals.var_q1m_dn8)), ((locals.var_phit_dn9 * locals.var_q1m) + (locals.var_phit * locals.var_q1m_dn9)),)
    } else {
        (locals.var_voxm, locals.var_voxm_dn4, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8, locals.var_voxm_dn9,)
    }
};
        locals.var_voxm = assign26650_e28143;
        locals.var_voxm_dn4 = assign26650_e28143_d_n4;
        locals.var_voxm_dn6 = assign26650_e28143_d_n6;
        locals.var_voxm_dn7 = assign26650_e28143_d_n7;
        locals.var_voxm_dn8 = assign26650_e28143_d_n8;
        locals.var_voxm_dn9 = assign26650_e28143_d_n9;

        let (assign26660_e28151, assign26660_e28151_d_n4, assign26660_e28151_d_n6, assign26660_e28151_d_n7, assign26660_e28151_d_n8, assign26660_e28151_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26660_e28149: f64 = (locals.var_voxm + locals.var_dch);
        (assign26660_e28149, (locals.var_voxm_dn4 + locals.var_dch_dn4), (locals.var_voxm_dn6 + locals.var_dch_dn6), (locals.var_voxm_dn7 + locals.var_dch_dn7), (locals.var_voxm_dn8 + locals.var_dch_dn8), (locals.var_voxm_dn9 + locals.var_dch_dn9),)
    } else {
        (locals.var_arg2mina, locals.var_arg2mina_dn4, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8, locals.var_arg2mina_dn9,)
    }
};
        locals.var_arg2mina = assign26660_e28151;
        locals.var_arg2mina_dn4 = assign26660_e28151_d_n4;
        locals.var_arg2mina_dn6 = assign26660_e28151_d_n6;
        locals.var_arg2mina_dn7 = assign26660_e28151_d_n7;
        locals.var_arg2mina_dn8 = assign26660_e28151_d_n8;
        locals.var_arg2mina_dn9 = assign26660_e28151_d_n9;

        let (assign26670_e28172, assign26670_e28172_d_n4, assign26670_e28172_d_n6, assign26670_e28172_d_n7, assign26670_e28172_d_n8, assign26670_e28172_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26670_e28158: f64 = locals.var_arg2mina;
        let assign26670_e28161: f64 = (-locals.var_arg2mina);
        let assign26670_e28164: f64 = (-locals.var_arg2mina);
        let assign26670_e28165: f64 = (assign26670_e28161 * assign26670_e28164);
        let assign26670_e28167: f64 = (assign26670_e28165 + 0.01);
        let assign26670_e28168: f64 = (assign26670_e28167).sqrt();
        let assign26670_e28169: f64 = (assign26670_e28158 - assign26670_e28168);
        let assign26670_e28170: f64 = (0.5 * assign26670_e28169);
        (assign26670_e28170, (0.5 * (locals.var_arg2mina_dn4 - ((((-locals.var_arg2mina_dn4) * assign26670_e28164) + (assign26670_e28161 * (-locals.var_arg2mina_dn4))) / (2.0 * assign26670_e28168)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign26670_e28164) + (assign26670_e28161 * (-locals.var_arg2mina_dn6))) / (2.0 * assign26670_e28168)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign26670_e28164) + (assign26670_e28161 * (-locals.var_arg2mina_dn7))) / (2.0 * assign26670_e28168)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign26670_e28164) + (assign26670_e28161 * (-locals.var_arg2mina_dn8))) / (2.0 * assign26670_e28168)))), (0.5 * (locals.var_arg2mina_dn9 - ((((-locals.var_arg2mina_dn9) * assign26670_e28164) + (assign26670_e28161 * (-locals.var_arg2mina_dn9))) / (2.0 * assign26670_e28168)))),)
    } else {
        (locals.var_psi_t, locals.var_psi_t_dn4, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8, locals.var_psi_t_dn9,)
    }
};
        locals.var_psi_t = assign26670_e28172;
        locals.var_psi_t_dn4 = assign26670_e28172_d_n4;
        locals.var_psi_t_dn6 = assign26670_e28172_d_n6;
        locals.var_psi_t_dn7 = assign26670_e28172_d_n7;
        locals.var_psi_t_dn8 = assign26670_e28172_d_n8;
        locals.var_psi_t_dn9 = assign26670_e28172_d_n9;

        let (assign26680_e28185, assign26680_e28185_d_n4, assign26680_e28185_d_n6, assign26680_e28185_d_n7, assign26680_e28185_d_n8, assign26680_e28185_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26680_e28178: f64 = (locals.var_voxm * locals.var_voxm);
        let assign26680_e28180: f64 = (assign26680_e28178 + 0.0001);
        let assign26680_e28181: f64 = (assign26680_e28180).sqrt();
        let assign26680_e28183: f64 = (assign26680_e28181 * locals.var_inv_chib);
        (assign26680_e28183, ((((locals.var_voxm_dn4 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn4)) / (2.0 * assign26680_e28181)) * locals.var_inv_chib), ((((locals.var_voxm_dn6 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn6)) / (2.0 * assign26680_e28181)) * locals.var_inv_chib), ((((locals.var_voxm_dn7 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn7)) / (2.0 * assign26680_e28181)) * locals.var_inv_chib), ((((locals.var_voxm_dn8 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn8)) / (2.0 * assign26680_e28181)) * locals.var_inv_chib), ((((locals.var_voxm_dn9 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn9)) / (2.0 * assign26680_e28181)) * locals.var_inv_chib),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign26680_e28185;
        locals.var_zg_dn4 = assign26680_e28185_d_n4;
        locals.var_zg_dn6 = assign26680_e28185_d_n6;
        locals.var_zg_dn7 = assign26680_e28185_d_n7;
        locals.var_zg_dn8 = assign26680_e28185_d_n8;
        locals.var_zg_dn9 = assign26680_e28185_d_n9;

        let assign26690_e28188: f64 = if locals.var_gc3ch_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard759 = assign26690_e28188;

        let (assign26700_e28211, assign26700_e28211_d_n4, assign26700_e28211_d_n6, assign26700_e28211_d_n7, assign26700_e28211_d_n8, assign26700_e28211_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard759 != 0.0)) {
        let assign26700_e28197: f64 = (locals.var_zg + locals.var_gcqch);
        let assign26700_e28200: f64 = (locals.var_zg - locals.var_gcqch);
        let assign26700_e28203: f64 = (locals.var_zg - locals.var_gcqch);
        let assign26700_e28204: f64 = (assign26700_e28200 * assign26700_e28203);
        let assign26700_e28206: f64 = (assign26700_e28204 + 1e-6);
        let assign26700_e28207: f64 = (assign26700_e28206).sqrt();
        let assign26700_e28208: f64 = (assign26700_e28197 - assign26700_e28207);
        let assign26700_e28209: f64 = (0.5 * assign26700_e28208);
        (assign26700_e28209, (0.5 * (locals.var_zg_dn4 - (((locals.var_zg_dn4 * assign26700_e28203) + (assign26700_e28200 * locals.var_zg_dn4)) / (2.0 * assign26700_e28207)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign26700_e28203) + (assign26700_e28200 * locals.var_zg_dn6)) / (2.0 * assign26700_e28207)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign26700_e28203) + (assign26700_e28200 * locals.var_zg_dn7)) / (2.0 * assign26700_e28207)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign26700_e28203) + (assign26700_e28200 * locals.var_zg_dn8)) / (2.0 * assign26700_e28207)))), (0.5 * (locals.var_zg_dn9 - (((locals.var_zg_dn9 * assign26700_e28203) + (assign26700_e28200 * locals.var_zg_dn9)) / (2.0 * assign26700_e28207)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign26700_e28211;
        locals.var_zg_dn4 = assign26700_e28211_d_n4;
        locals.var_zg_dn6 = assign26700_e28211_d_n6;
        locals.var_zg_dn7 = assign26700_e28211_d_n7;
        locals.var_zg_dn8 = assign26700_e28211_d_n8;
        locals.var_zg_dn9 = assign26700_e28211_d_n9;

        let (assign26710_e28219, assign26710_e28219_d_n4, assign26710_e28219_d_n6, assign26710_e28219_d_n7, assign26710_e28219_d_n8, assign26710_e28219_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26710_e28217: f64 = (locals.var_xg1x_dc + locals.var_eg_2phit0);
        (assign26710_e28217, (locals.var_xg1x_dc_dn4 + locals.var_eg_2phit0_dn4), (locals.var_xg1x_dc_dn6 + locals.var_eg_2phit0_dn6), (locals.var_xg1x_dc_dn7 + locals.var_eg_2phit0_dn7), (locals.var_xg1x_dc_dn8 + locals.var_eg_2phit0_dn8), (locals.var_xg1x_dc_dn9 + locals.var_eg_2phit0_dn9),)
    } else {
        (locals.var_xg1xshift, locals.var_xg1xshift_dn4, locals.var_xg1xshift_dn6, locals.var_xg1xshift_dn7, locals.var_xg1xshift_dn8, locals.var_xg1xshift_dn9,)
    }
};
        locals.var_xg1xshift = assign26710_e28219;
        locals.var_xg1xshift_dn4 = assign26710_e28219_d_n4;
        locals.var_xg1xshift_dn6 = assign26710_e28219_d_n6;
        locals.var_xg1xshift_dn7 = assign26710_e28219_d_n7;
        locals.var_xg1xshift_dn8 = assign26710_e28219_d_n8;
        locals.var_xg1xshift_dn9 = assign26710_e28219_d_n9;

        let (assign26720_e28227, assign26720_e28227_d_n4, assign26720_e28227_d_n6, assign26720_e28227_d_n7, assign26720_e28227_d_n8, assign26720_e28227_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26720_e28225: f64 = (locals.var_xg1xshift - locals.var_q1m);
        (assign26720_e28225, (locals.var_xg1xshift_dn4 - locals.var_q1m_dn4), (locals.var_xg1xshift_dn6 - locals.var_q1m_dn6), (locals.var_xg1xshift_dn7 - locals.var_q1m_dn7), (locals.var_xg1xshift_dn8 - locals.var_q1m_dn8), (locals.var_xg1xshift_dn9 - locals.var_q1m_dn9),)
    } else {
        (locals.var_x_m, locals.var_x_m_dn4, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, locals.var_x_m_dn9,)
    }
};
        locals.var_x_m = assign26720_e28227;
        locals.var_x_m_dn4 = assign26720_e28227_d_n4;
        locals.var_x_m_dn6 = assign26720_e28227_d_n6;
        locals.var_x_m_dn7 = assign26720_e28227_d_n7;
        locals.var_x_m_dn8 = assign26720_e28227_d_n8;
        locals.var_x_m_dn9 = assign26720_e28227_d_n9;

        let (assign26730_e28243, assign26730_e28243_d_n4, assign26730_e28243_d_n6, assign26730_e28243_d_n7, assign26730_e28243_d_n8, assign26730_e28243_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26730_e28234: f64 = (locals.var_psi_t - locals.var_alpha_b);
        let assign26730_e28236: f64 = (assign26730_e28234 - locals.var_vm);
        let assign26730_e28238: f64 = (assign26730_e28236 * locals.var_inv_phit);
        let assign26730_e28239: f64 = (locals.var_x_m + assign26730_e28238);
        let assign26730_e28241: f64 = (assign26730_e28239 * locals.var_n_iginv);
        (assign26730_e28241, (((locals.var_x_m_dn4 + ((((locals.var_psi_t_dn4 - locals.var_alpha_b_dn4) - locals.var_vm_dn4) * locals.var_inv_phit) + (assign26730_e28236 * locals.var_inv_phit_dn4))) * locals.var_n_iginv) + (assign26730_e28239 * locals.var_n_iginv_dn4)), (((locals.var_x_m_dn6 + ((((locals.var_psi_t_dn6 - locals.var_alpha_b_dn6) - locals.var_vm_dn6) * locals.var_inv_phit) + (assign26730_e28236 * locals.var_inv_phit_dn6))) * locals.var_n_iginv) + (assign26730_e28239 * locals.var_n_iginv_dn6)), (((locals.var_x_m_dn7 + ((((locals.var_psi_t_dn7 - locals.var_alpha_b_dn7) - locals.var_vm_dn7) * locals.var_inv_phit) + (assign26730_e28236 * locals.var_inv_phit_dn7))) * locals.var_n_iginv) + (assign26730_e28239 * locals.var_n_iginv_dn7)), (((locals.var_x_m_dn8 + ((((locals.var_psi_t_dn8 - locals.var_alpha_b_dn8) - locals.var_vm_dn8) * locals.var_inv_phit) + (assign26730_e28236 * locals.var_inv_phit_dn8))) * locals.var_n_iginv) + (assign26730_e28239 * locals.var_n_iginv_dn8)), (((locals.var_x_m_dn9 + ((((locals.var_psi_t_dn9 - locals.var_alpha_b_dn9) - locals.var_vm_dn9) * locals.var_inv_phit) + (assign26730_e28236 * locals.var_inv_phit_dn9))) * locals.var_n_iginv) + (assign26730_e28239 * locals.var_n_iginv_dn9)),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign26730_e28243;
        locals.var_arg1_dn4 = assign26730_e28243_d_n4;
        locals.var_arg1_dn6 = assign26730_e28243_d_n6;
        locals.var_arg1_dn7 = assign26730_e28243_d_n7;
        locals.var_arg1_dn8 = assign26730_e28243_d_n8;
        locals.var_arg1_dn9 = assign26730_e28243_d_n9;

        let assign26740_e28245: f64 = (locals.var_arg1).abs();
        let assign26740_e28247: f64 = if assign26740_e28245 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard760 = assign26740_e28247;

        let (assign26750_e28256, assign26750_e28256_d_n4, assign26750_e28256_d_n6, assign26750_e28256_d_n7, assign26750_e28256_d_n8, assign26750_e28256_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard760 != 0.0)) {
        let assign26750_e28254: f64 = (locals.var_arg1).exp();
        (assign26750_e28254, (assign26750_e28254 * locals.var_arg1_dn4), (assign26750_e28254 * locals.var_arg1_dn6), (assign26750_e28254 * locals.var_arg1_dn7), (assign26750_e28254 * locals.var_arg1_dn8), (assign26750_e28254 * locals.var_arg1_dn9),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26750_e28256;
        locals.var_dsi_dn4 = assign26750_e28256_d_n4;
        locals.var_dsi_dn6 = assign26750_e28256_d_n6;
        locals.var_dsi_dn7 = assign26750_e28256_d_n7;
        locals.var_dsi_dn8 = assign26750_e28256_d_n8;
        locals.var_dsi_dn9 = assign26750_e28256_d_n9;

        let assign26760_e28259: f64 = (-80.0);
        let assign26760_e28260: f64 = if locals.var_arg1 < assign26760_e28259 { 1.0 } else { 0.0 };
        locals.var_guard761 = assign26760_e28260;

    }

    pub(super) fn stamp_transient_block_68(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26770_e28296, assign26770_e28296_d_n4, assign26770_e28296_d_n6, assign26770_e28296_d_n7, assign26770_e28296_d_n8, assign26770_e28296_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 != 0.0)) {
        let assign26770_e28272: f64 = (-locals.var_arg1);
        let assign26770_e28274: f64 = (assign26770_e28272 - 80.0);
        let assign26770_e28278: f64 = (-locals.var_arg1);
        let assign26770_e28280: f64 = (assign26770_e28278 - 80.0);
        let assign26770_e28281: f64 = (0.5 * assign26770_e28280);
        let assign26770_e28284: f64 = (-locals.var_arg1);
        let assign26770_e28286: f64 = (assign26770_e28284 - 80.0);
        let assign26770_e28288: f64 = (assign26770_e28286 * 0.3333333333333);
        let assign26770_e28289: f64 = (1.0 + assign26770_e28288);
        let assign26770_e28290: f64 = (assign26770_e28281 * assign26770_e28289);
        let assign26770_e28291: f64 = (1.0 + assign26770_e28290);
        let assign26770_e28292: f64 = (assign26770_e28274 * assign26770_e28291);
        let assign26770_e28293: f64 = (1.0 + assign26770_e28292);
        let assign26770_e28294: f64 = (1.80485e-35 / assign26770_e28293);
        (assign26770_e28294, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign26770_e28291) + (assign26770_e28274 * (((0.5 * (-locals.var_arg1_dn4)) * assign26770_e28289) + (assign26770_e28281 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign26770_e28293 * assign26770_e28293))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign26770_e28291) + (assign26770_e28274 * (((0.5 * (-locals.var_arg1_dn6)) * assign26770_e28289) + (assign26770_e28281 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign26770_e28293 * assign26770_e28293))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign26770_e28291) + (assign26770_e28274 * (((0.5 * (-locals.var_arg1_dn7)) * assign26770_e28289) + (assign26770_e28281 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign26770_e28293 * assign26770_e28293))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign26770_e28291) + (assign26770_e28274 * (((0.5 * (-locals.var_arg1_dn8)) * assign26770_e28289) + (assign26770_e28281 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign26770_e28293 * assign26770_e28293))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign26770_e28291) + (assign26770_e28274 * (((0.5 * (-locals.var_arg1_dn9)) * assign26770_e28289) + (assign26770_e28281 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign26770_e28293 * assign26770_e28293))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26770_e28296;
        locals.var_dsi_dn4 = assign26770_e28296_d_n4;
        locals.var_dsi_dn6 = assign26770_e28296_d_n6;
        locals.var_dsi_dn7 = assign26770_e28296_d_n7;
        locals.var_dsi_dn8 = assign26770_e28296_d_n8;
        locals.var_dsi_dn9 = assign26770_e28296_d_n9;

        let (assign26780_e28330, assign26780_e28330_d_n4, assign26780_e28330_d_n6, assign26780_e28330_d_n7, assign26780_e28330_d_n8, assign26780_e28330_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 == 0.0)) {
        let assign26780_e28310: f64 = (locals.var_arg1 - 80.0);
        let assign26780_e28315: f64 = (locals.var_arg1 - 80.0);
        let assign26780_e28316: f64 = (0.5 * assign26780_e28315);
        let assign26780_e28320: f64 = (locals.var_arg1 - 80.0);
        let assign26780_e28322: f64 = (assign26780_e28320 * 0.3333333333333);
        let assign26780_e28323: f64 = (1.0 + assign26780_e28322);
        let assign26780_e28324: f64 = (assign26780_e28316 * assign26780_e28323);
        let assign26780_e28325: f64 = (1.0 + assign26780_e28324);
        let assign26780_e28326: f64 = (assign26780_e28310 * assign26780_e28325);
        let assign26780_e28327: f64 = (1.0 + assign26780_e28326);
        let assign26780_e28328: f64 = (5.54062e34 * assign26780_e28327);
        (assign26780_e28328, (5.54062e34 * ((locals.var_arg1_dn4 * assign26780_e28325) + (assign26780_e28310 * (((0.5 * locals.var_arg1_dn4) * assign26780_e28323) + (assign26780_e28316 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign26780_e28325) + (assign26780_e28310 * (((0.5 * locals.var_arg1_dn6) * assign26780_e28323) + (assign26780_e28316 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign26780_e28325) + (assign26780_e28310 * (((0.5 * locals.var_arg1_dn7) * assign26780_e28323) + (assign26780_e28316 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign26780_e28325) + (assign26780_e28310 * (((0.5 * locals.var_arg1_dn8) * assign26780_e28323) + (assign26780_e28316 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign26780_e28325) + (assign26780_e28310 * (((0.5 * locals.var_arg1_dn9) * assign26780_e28323) + (assign26780_e28316 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26780_e28330;
        locals.var_dsi_dn4 = assign26780_e28330_d_n4;
        locals.var_dsi_dn6 = assign26780_e28330_d_n6;
        locals.var_dsi_dn7 = assign26780_e28330_d_n7;
        locals.var_dsi_dn8 = assign26780_e28330_d_n8;
        locals.var_dsi_dn9 = assign26780_e28330_d_n9;

        let (assign26790_e28343, assign26790_e28343_d_n4, assign26790_e28343_d_n6, assign26790_e28343_d_n7, assign26790_e28343_d_n8, assign26790_e28343_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26790_e28336: f64 = (locals.var_vgs - locals.var_vm);
        let assign26790_e28337: f64 = (-assign26790_e28336);
        let assign26790_e28339: f64 = (assign26790_e28337 * locals.var_inv_phit);
        let assign26790_e28341: f64 = (assign26790_e28339 * locals.var_n_iginv);
        (assign26790_e28341, (((((-(-locals.var_vm_dn4)) * locals.var_inv_phit) + (assign26790_e28337 * locals.var_inv_phit_dn4)) * locals.var_n_iginv) + (assign26790_e28339 * locals.var_n_iginv_dn4)), (((((-(locals.var_vgs_dn6 - locals.var_vm_dn6)) * locals.var_inv_phit) + (assign26790_e28337 * locals.var_inv_phit_dn6)) * locals.var_n_iginv) + (assign26790_e28339 * locals.var_n_iginv_dn6)), (((((-(locals.var_vgs_dn7 - locals.var_vm_dn7)) * locals.var_inv_phit) + (assign26790_e28337 * locals.var_inv_phit_dn7)) * locals.var_n_iginv) + (assign26790_e28339 * locals.var_n_iginv_dn7)), (((((-(-locals.var_vm_dn8)) * locals.var_inv_phit) + (assign26790_e28337 * locals.var_inv_phit_dn8)) * locals.var_n_iginv) + (assign26790_e28339 * locals.var_n_iginv_dn8)), (((((-(locals.var_vgs_dn9 - locals.var_vm_dn9)) * locals.var_inv_phit) + (assign26790_e28337 * locals.var_inv_phit_dn9)) * locals.var_n_iginv) + (assign26790_e28339 * locals.var_n_iginv_dn9)),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign26790_e28343;
        locals.var_arg1_dn4 = assign26790_e28343_d_n4;
        locals.var_arg1_dn6 = assign26790_e28343_d_n6;
        locals.var_arg1_dn7 = assign26790_e28343_d_n7;
        locals.var_arg1_dn8 = assign26790_e28343_d_n8;
        locals.var_arg1_dn9 = assign26790_e28343_d_n9;

        let assign26800_e28345: f64 = (locals.var_arg1).abs();
        let assign26800_e28347: f64 = if assign26800_e28345 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard762 = assign26800_e28347;

        let (assign26810_e28356, assign26810_e28356_d_n4, assign26810_e28356_d_n6, assign26810_e28356_d_n7, assign26810_e28356_d_n8, assign26810_e28356_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard762 != 0.0)) {
        let assign26810_e28354: f64 = (locals.var_arg1).exp();
        (assign26810_e28354, (assign26810_e28354 * locals.var_arg1_dn4), (assign26810_e28354 * locals.var_arg1_dn6), (assign26810_e28354 * locals.var_arg1_dn7), (assign26810_e28354 * locals.var_arg1_dn8), (assign26810_e28354 * locals.var_arg1_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26810_e28356;
        locals.var_temp_dn4 = assign26810_e28356_d_n4;
        locals.var_temp_dn6 = assign26810_e28356_d_n6;
        locals.var_temp_dn7 = assign26810_e28356_d_n7;
        locals.var_temp_dn8 = assign26810_e28356_d_n8;
        locals.var_temp_dn9 = assign26810_e28356_d_n9;

        let assign26820_e28359: f64 = (-80.0);
        let assign26820_e28360: f64 = if locals.var_arg1 < assign26820_e28359 { 1.0 } else { 0.0 };
        locals.var_guard763 = assign26820_e28360;

        let (assign26830_e28396, assign26830_e28396_d_n4, assign26830_e28396_d_n6, assign26830_e28396_d_n7, assign26830_e28396_d_n8, assign26830_e28396_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 != 0.0)) {
        let assign26830_e28372: f64 = (-locals.var_arg1);
        let assign26830_e28374: f64 = (assign26830_e28372 - 80.0);
        let assign26830_e28378: f64 = (-locals.var_arg1);
        let assign26830_e28380: f64 = (assign26830_e28378 - 80.0);
        let assign26830_e28381: f64 = (0.5 * assign26830_e28380);
        let assign26830_e28384: f64 = (-locals.var_arg1);
        let assign26830_e28386: f64 = (assign26830_e28384 - 80.0);
        let assign26830_e28388: f64 = (assign26830_e28386 * 0.3333333333333);
        let assign26830_e28389: f64 = (1.0 + assign26830_e28388);
        let assign26830_e28390: f64 = (assign26830_e28381 * assign26830_e28389);
        let assign26830_e28391: f64 = (1.0 + assign26830_e28390);
        let assign26830_e28392: f64 = (assign26830_e28374 * assign26830_e28391);
        let assign26830_e28393: f64 = (1.0 + assign26830_e28392);
        let assign26830_e28394: f64 = (1.80485e-35 / assign26830_e28393);
        (assign26830_e28394, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign26830_e28391) + (assign26830_e28374 * (((0.5 * (-locals.var_arg1_dn4)) * assign26830_e28389) + (assign26830_e28381 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign26830_e28393 * assign26830_e28393))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign26830_e28391) + (assign26830_e28374 * (((0.5 * (-locals.var_arg1_dn6)) * assign26830_e28389) + (assign26830_e28381 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign26830_e28393 * assign26830_e28393))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign26830_e28391) + (assign26830_e28374 * (((0.5 * (-locals.var_arg1_dn7)) * assign26830_e28389) + (assign26830_e28381 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign26830_e28393 * assign26830_e28393))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign26830_e28391) + (assign26830_e28374 * (((0.5 * (-locals.var_arg1_dn8)) * assign26830_e28389) + (assign26830_e28381 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign26830_e28393 * assign26830_e28393))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign26830_e28391) + (assign26830_e28374 * (((0.5 * (-locals.var_arg1_dn9)) * assign26830_e28389) + (assign26830_e28381 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign26830_e28393 * assign26830_e28393))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26830_e28396;
        locals.var_temp_dn4 = assign26830_e28396_d_n4;
        locals.var_temp_dn6 = assign26830_e28396_d_n6;
        locals.var_temp_dn7 = assign26830_e28396_d_n7;
        locals.var_temp_dn8 = assign26830_e28396_d_n8;
        locals.var_temp_dn9 = assign26830_e28396_d_n9;

        let (assign26840_e28430, assign26840_e28430_d_n4, assign26840_e28430_d_n6, assign26840_e28430_d_n7, assign26840_e28430_d_n8, assign26840_e28430_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 == 0.0)) {
        let assign26840_e28410: f64 = (locals.var_arg1 - 80.0);
        let assign26840_e28415: f64 = (locals.var_arg1 - 80.0);
        let assign26840_e28416: f64 = (0.5 * assign26840_e28415);
        let assign26840_e28420: f64 = (locals.var_arg1 - 80.0);
        let assign26840_e28422: f64 = (assign26840_e28420 * 0.3333333333333);
        let assign26840_e28423: f64 = (1.0 + assign26840_e28422);
        let assign26840_e28424: f64 = (assign26840_e28416 * assign26840_e28423);
        let assign26840_e28425: f64 = (1.0 + assign26840_e28424);
        let assign26840_e28426: f64 = (assign26840_e28410 * assign26840_e28425);
        let assign26840_e28427: f64 = (1.0 + assign26840_e28426);
        let assign26840_e28428: f64 = (5.54062e34 * assign26840_e28427);
        (assign26840_e28428, (5.54062e34 * ((locals.var_arg1_dn4 * assign26840_e28425) + (assign26840_e28410 * (((0.5 * locals.var_arg1_dn4) * assign26840_e28423) + (assign26840_e28416 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign26840_e28425) + (assign26840_e28410 * (((0.5 * locals.var_arg1_dn6) * assign26840_e28423) + (assign26840_e28416 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign26840_e28425) + (assign26840_e28410 * (((0.5 * locals.var_arg1_dn7) * assign26840_e28423) + (assign26840_e28416 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign26840_e28425) + (assign26840_e28410 * (((0.5 * locals.var_arg1_dn8) * assign26840_e28423) + (assign26840_e28416 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign26840_e28425) + (assign26840_e28410 * (((0.5 * locals.var_arg1_dn9) * assign26840_e28423) + (assign26840_e28416 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26840_e28430;
        locals.var_temp_dn4 = assign26840_e28430_d_n4;
        locals.var_temp_dn6 = assign26840_e28430_d_n6;
        locals.var_temp_dn7 = assign26840_e28430_d_n7;
        locals.var_temp_dn8 = assign26840_e28430_d_n8;
        locals.var_temp_dn9 = assign26840_e28430_d_n9;

        let (assign26850_e28438, assign26850_e28438_d_n4, assign26850_e28438_d_n6, assign26850_e28438_d_n7, assign26850_e28438_d_n8, assign26850_e28438_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26850_e28436: f64 = (locals.var_dsi * locals.var_temp);
        (assign26850_e28436, ((locals.var_dsi_dn4 * locals.var_temp) + (locals.var_dsi * locals.var_temp_dn4)), ((locals.var_dsi_dn6 * locals.var_temp) + (locals.var_dsi * locals.var_temp_dn6)), ((locals.var_dsi_dn7 * locals.var_temp) + (locals.var_dsi * locals.var_temp_dn7)), ((locals.var_dsi_dn8 * locals.var_temp) + (locals.var_dsi * locals.var_temp_dn8)), ((locals.var_dsi_dn9 * locals.var_temp) + (locals.var_dsi * locals.var_temp_dn9)),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign26850_e28438;
        locals.var_dgate_dn4 = assign26850_e28438_d_n4;
        locals.var_dgate_dn6 = assign26850_e28438_d_n6;
        locals.var_dgate_dn7 = assign26850_e28438_d_n7;
        locals.var_dgate_dn8 = assign26850_e28438_d_n8;
        locals.var_dgate_dn9 = assign26850_e28438_d_n9;

        let (assign26860_e28455, assign26860_e28455_d_n4, assign26860_e28455_d_n6, assign26860_e28455_d_n7, assign26860_e28455_d_n8, assign26860_e28455_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26860_e28444: f64 = (-1.5);
        let assign26860_e28449: f64 = (locals.var_gc3ch_i * locals.var_zg);
        let assign26860_e28450: f64 = (locals.var_gc2ch_i + assign26860_e28449);
        let assign26860_e28451: f64 = (locals.var_zg * assign26860_e28450);
        let assign26860_e28452: f64 = (assign26860_e28444 + assign26860_e28451);
        let assign26860_e28453: f64 = (locals.var_bch * assign26860_e28452);
        (assign26860_e28453, ((locals.var_bch_dn4 * assign26860_e28452) + (locals.var_bch * ((locals.var_zg_dn4 * assign26860_e28450) + (locals.var_zg * (locals.var_gc3ch_i * locals.var_zg_dn4))))), ((locals.var_bch_dn6 * assign26860_e28452) + (locals.var_bch * ((locals.var_zg_dn6 * assign26860_e28450) + (locals.var_zg * (locals.var_gc3ch_i * locals.var_zg_dn6))))), ((locals.var_bch_dn7 * assign26860_e28452) + (locals.var_bch * ((locals.var_zg_dn7 * assign26860_e28450) + (locals.var_zg * (locals.var_gc3ch_i * locals.var_zg_dn7))))), ((locals.var_bch_dn8 * assign26860_e28452) + (locals.var_bch * ((locals.var_zg_dn8 * assign26860_e28450) + (locals.var_zg * (locals.var_gc3ch_i * locals.var_zg_dn8))))), ((locals.var_bch_dn9 * assign26860_e28452) + (locals.var_bch * ((locals.var_zg_dn9 * assign26860_e28450) + (locals.var_zg * (locals.var_gc3ch_i * locals.var_zg_dn9))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26860_e28455;
        locals.var_temp_dn4 = assign26860_e28455_d_n4;
        locals.var_temp_dn6 = assign26860_e28455_d_n6;
        locals.var_temp_dn7 = assign26860_e28455_d_n7;
        locals.var_temp_dn8 = assign26860_e28455_d_n8;
        locals.var_temp_dn9 = assign26860_e28455_d_n9;

        let assign26950_e28604: f64 = if ((locals.var_xg1xshift <= 0.0) || ((locals.var_gc2ch_i == 0.0) && (locals.var_gc3ch_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard767 = assign26950_e28604;

        let (assign26980_e28635, assign26980_e28635_d_n4, assign26980_e28635_d_n6, assign26980_e28635_d_n7, assign26980_e28635_d_n8, assign26980_e28635_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign26980_e28630: f64 = (2.0 * locals.var_gc3ch_i);
        let assign26980_e28632: f64 = (assign26980_e28630 * locals.var_zg);
        let assign26980_e28633: f64 = (locals.var_gc2ch_i + assign26980_e28632);
        (assign26980_e28633, (assign26980_e28630 * locals.var_zg_dn4), (assign26980_e28630 * locals.var_zg_dn6), (assign26980_e28630 * locals.var_zg_dn7), (assign26980_e28630 * locals.var_zg_dn8), (assign26980_e28630 * locals.var_zg_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26980_e28635;
        locals.var_temp_dn4 = assign26980_e28635_d_n4;
        locals.var_temp_dn6 = assign26980_e28635_d_n6;
        locals.var_temp_dn7 = assign26980_e28635_d_n7;
        locals.var_temp_dn8 = assign26980_e28635_d_n8;
        locals.var_temp_dn9 = assign26980_e28635_d_n9;

        let (assign26990_e28650, assign26990_e28650_d_n4, assign26990_e28650_d_n6, assign26990_e28650_d_n7, assign26990_e28650_d_n8, assign26990_e28650_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign26990_e28645: f64 = (locals.var_temp * locals.var_bch);
        let assign26990_e28646: f64 = (locals.var_chib_i / assign26990_e28645);
        let assign26990_e28648: f64 = (assign26990_e28646 * locals.var_inv_phit);
        (assign26990_e28648, (((-((locals.var_chib_i * ((locals.var_temp_dn4 * locals.var_bch) + (locals.var_temp * locals.var_bch_dn4))) / (assign26990_e28645 * assign26990_e28645))) * locals.var_inv_phit) + (assign26990_e28646 * locals.var_inv_phit_dn4)), (((-((locals.var_chib_i * ((locals.var_temp_dn6 * locals.var_bch) + (locals.var_temp * locals.var_bch_dn6))) / (assign26990_e28645 * assign26990_e28645))) * locals.var_inv_phit) + (assign26990_e28646 * locals.var_inv_phit_dn6)), (((-((locals.var_chib_i * ((locals.var_temp_dn7 * locals.var_bch) + (locals.var_temp * locals.var_bch_dn7))) / (assign26990_e28645 * assign26990_e28645))) * locals.var_inv_phit) + (assign26990_e28646 * locals.var_inv_phit_dn7)), (((-((locals.var_chib_i * ((locals.var_temp_dn8 * locals.var_bch) + (locals.var_temp * locals.var_bch_dn8))) / (assign26990_e28645 * assign26990_e28645))) * locals.var_inv_phit) + (assign26990_e28646 * locals.var_inv_phit_dn8)), (((-((locals.var_chib_i * ((locals.var_temp_dn9 * locals.var_bch) + (locals.var_temp * locals.var_bch_dn9))) / (assign26990_e28645 * assign26990_e28645))) * locals.var_inv_phit) + (assign26990_e28646 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_u0, locals.var_u0_dn4, locals.var_u0_dn6, locals.var_u0_dn7, locals.var_u0_dn8, locals.var_u0_dn9,)
    }
};
        locals.var_u0 = assign26990_e28650;
        locals.var_u0_dn4 = assign26990_e28650_d_n4;
        locals.var_u0_dn6 = assign26990_e28650_d_n6;
        locals.var_u0_dn7 = assign26990_e28650_d_n7;
        locals.var_u0_dn8 = assign26990_e28650_d_n8;
        locals.var_u0_dn9 = assign26990_e28650_d_n9;

        let (assign27000_e28661, assign27000_e28661_d_n4, assign27000_e28661_d_n6, assign27000_e28661_d_n7, assign27000_e28661_d_n8, assign27000_e28661_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign27000_e28659: f64 = (locals.var_half_x_ds / locals.var_u0);
        (assign27000_e28659, (((locals.var_half_x_ds_dn4 * locals.var_u0) - (locals.var_half_x_ds * locals.var_u0_dn4)) / (locals.var_u0 * locals.var_u0)), (((locals.var_half_x_ds_dn6 * locals.var_u0) - (locals.var_half_x_ds * locals.var_u0_dn6)) / (locals.var_u0 * locals.var_u0)), (((locals.var_half_x_ds_dn7 * locals.var_u0) - (locals.var_half_x_ds * locals.var_u0_dn7)) / (locals.var_u0 * locals.var_u0)), (((locals.var_half_x_ds_dn8 * locals.var_u0) - (locals.var_half_x_ds * locals.var_u0_dn8)) / (locals.var_u0 * locals.var_u0)), (((locals.var_half_x_ds_dn9 * locals.var_u0) - (locals.var_half_x_ds * locals.var_u0_dn9)) / (locals.var_u0 * locals.var_u0)),)
    } else {
        (locals.var_x, locals.var_x_dn4, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9,)
    }
};
        locals.var_x = assign27000_e28661;
        locals.var_x_dn4 = assign27000_e28661_d_n4;
        locals.var_x_dn6 = assign27000_e28661_d_n6;
        locals.var_x_dn7 = assign27000_e28661_d_n7;
        locals.var_x_dn8 = assign27000_e28661_d_n8;
        locals.var_x_dn9 = assign27000_e28661_d_n9;

        let assign27040_e28705: f64 = if locals.var_x < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard768 = assign27040_e28705;

        let assign27090_e28798: f64 = (locals.var_x).abs();
        let assign27090_e28800: f64 = if assign27090_e28798 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard769 = assign27090_e28800;

        let (assign27100_e28815, assign27100_e28815_d_n4, assign27100_e28815_d_n6, assign27100_e28815_d_n7, assign27100_e28815_d_n8, assign27100_e28815_d_n9,) = {
    if (((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) && (locals.var_guard769 != 0.0)) {
        let assign27100_e28813: f64 = (locals.var_x).exp();
        (assign27100_e28813, (assign27100_e28813 * locals.var_x_dn4), (assign27100_e28813 * locals.var_x_dn6), (assign27100_e28813 * locals.var_x_dn7), (assign27100_e28813 * locals.var_x_dn8), (assign27100_e28813 * locals.var_x_dn9),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign27100_e28815;
        locals.var_ex_dn4 = assign27100_e28815_d_n4;
        locals.var_ex_dn6 = assign27100_e28815_d_n6;
        locals.var_ex_dn7 = assign27100_e28815_d_n7;
        locals.var_ex_dn8 = assign27100_e28815_d_n8;
        locals.var_ex_dn9 = assign27100_e28815_d_n9;

        let assign27110_e28818: f64 = (-80.0);
        let assign27110_e28819: f64 = if locals.var_x < assign27110_e28818 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign27110_e28819;

        let (assign27120_e28861, assign27120_e28861_d_n4, assign27120_e28861_d_n6, assign27120_e28861_d_n7, assign27120_e28861_d_n8, assign27120_e28861_d_n9,) = {
    if ((((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) && (locals.var_guard769 == 0.0)) && (locals.var_guard770 != 0.0)) {
        let assign27120_e28837: f64 = (-locals.var_x);
        let assign27120_e28839: f64 = (assign27120_e28837 - 80.0);
        let assign27120_e28843: f64 = (-locals.var_x);
        let assign27120_e28845: f64 = (assign27120_e28843 - 80.0);
        let assign27120_e28846: f64 = (0.5 * assign27120_e28845);
        let assign27120_e28849: f64 = (-locals.var_x);
        let assign27120_e28851: f64 = (assign27120_e28849 - 80.0);
        let assign27120_e28853: f64 = (assign27120_e28851 * 0.3333333333333);
        let assign27120_e28854: f64 = (1.0 + assign27120_e28853);
        let assign27120_e28855: f64 = (assign27120_e28846 * assign27120_e28854);
        let assign27120_e28856: f64 = (1.0 + assign27120_e28855);
        let assign27120_e28857: f64 = (assign27120_e28839 * assign27120_e28856);
        let assign27120_e28858: f64 = (1.0 + assign27120_e28857);
        let assign27120_e28859: f64 = (1.80485e-35 / assign27120_e28858);
        (assign27120_e28859, (-((1.80485e-35 * (((-locals.var_x_dn4) * assign27120_e28856) + (assign27120_e28839 * (((0.5 * (-locals.var_x_dn4)) * assign27120_e28854) + (assign27120_e28846 * ((-locals.var_x_dn4) * 0.3333333333333)))))) / (assign27120_e28858 * assign27120_e28858))), (-((1.80485e-35 * (((-locals.var_x_dn6) * assign27120_e28856) + (assign27120_e28839 * (((0.5 * (-locals.var_x_dn6)) * assign27120_e28854) + (assign27120_e28846 * ((-locals.var_x_dn6) * 0.3333333333333)))))) / (assign27120_e28858 * assign27120_e28858))), (-((1.80485e-35 * (((-locals.var_x_dn7) * assign27120_e28856) + (assign27120_e28839 * (((0.5 * (-locals.var_x_dn7)) * assign27120_e28854) + (assign27120_e28846 * ((-locals.var_x_dn7) * 0.3333333333333)))))) / (assign27120_e28858 * assign27120_e28858))), (-((1.80485e-35 * (((-locals.var_x_dn8) * assign27120_e28856) + (assign27120_e28839 * (((0.5 * (-locals.var_x_dn8)) * assign27120_e28854) + (assign27120_e28846 * ((-locals.var_x_dn8) * 0.3333333333333)))))) / (assign27120_e28858 * assign27120_e28858))), (-((1.80485e-35 * (((-locals.var_x_dn9) * assign27120_e28856) + (assign27120_e28839 * (((0.5 * (-locals.var_x_dn9)) * assign27120_e28854) + (assign27120_e28846 * ((-locals.var_x_dn9) * 0.3333333333333)))))) / (assign27120_e28858 * assign27120_e28858))),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign27120_e28861;
        locals.var_ex_dn4 = assign27120_e28861_d_n4;
        locals.var_ex_dn6 = assign27120_e28861_d_n6;
        locals.var_ex_dn7 = assign27120_e28861_d_n7;
        locals.var_ex_dn8 = assign27120_e28861_d_n8;
        locals.var_ex_dn9 = assign27120_e28861_d_n9;

        let (assign27130_e28901, assign27130_e28901_d_n4, assign27130_e28901_d_n6, assign27130_e28901_d_n7, assign27130_e28901_d_n8, assign27130_e28901_d_n9,) = {
    if ((((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) && (locals.var_guard769 == 0.0)) && (locals.var_guard770 == 0.0)) {
        let assign27130_e28881: f64 = (locals.var_x - 80.0);
        let assign27130_e28886: f64 = (locals.var_x - 80.0);
        let assign27130_e28887: f64 = (0.5 * assign27130_e28886);
        let assign27130_e28891: f64 = (locals.var_x - 80.0);
        let assign27130_e28893: f64 = (assign27130_e28891 * 0.3333333333333);
        let assign27130_e28894: f64 = (1.0 + assign27130_e28893);
        let assign27130_e28895: f64 = (assign27130_e28887 * assign27130_e28894);
        let assign27130_e28896: f64 = (1.0 + assign27130_e28895);
        let assign27130_e28897: f64 = (assign27130_e28881 * assign27130_e28896);
        let assign27130_e28898: f64 = (1.0 + assign27130_e28897);
        let assign27130_e28899: f64 = (5.54062e34 * assign27130_e28898);
        (assign27130_e28899, (5.54062e34 * ((locals.var_x_dn4 * assign27130_e28896) + (assign27130_e28881 * (((0.5 * locals.var_x_dn4) * assign27130_e28894) + (assign27130_e28887 * (locals.var_x_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_x_dn6 * assign27130_e28896) + (assign27130_e28881 * (((0.5 * locals.var_x_dn6) * assign27130_e28894) + (assign27130_e28887 * (locals.var_x_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_x_dn7 * assign27130_e28896) + (assign27130_e28881 * (((0.5 * locals.var_x_dn7) * assign27130_e28894) + (assign27130_e28887 * (locals.var_x_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_x_dn8 * assign27130_e28896) + (assign27130_e28881 * (((0.5 * locals.var_x_dn8) * assign27130_e28894) + (assign27130_e28887 * (locals.var_x_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_x_dn9 * assign27130_e28896) + (assign27130_e28881 * (((0.5 * locals.var_x_dn9) * assign27130_e28894) + (assign27130_e28887 * (locals.var_x_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign27130_e28901;
        locals.var_ex_dn4 = assign27130_e28901_d_n4;
        locals.var_ex_dn6 = assign27130_e28901_d_n6;
        locals.var_ex_dn7 = assign27130_e28901_d_n7;
        locals.var_ex_dn8 = assign27130_e28901_d_n8;
        locals.var_ex_dn9 = assign27130_e28901_d_n9;

        let (assign27140_e28915, assign27140_e28915_d_n4, assign27140_e28915_d_n6, assign27140_e28915_d_n7, assign27140_e28915_d_n8, assign27140_e28915_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
        let assign27140_e28913: f64 = (1.0 / locals.var_ex);
        (assign27140_e28913, (-(locals.var_ex_dn4 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn6 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn7 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn8 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn9 / (locals.var_ex * locals.var_ex))),)
    } else {
        (locals.var_inv_ex, locals.var_inv_ex_dn4, locals.var_inv_ex_dn6, locals.var_inv_ex_dn7, locals.var_inv_ex_dn8, locals.var_inv_ex_dn9,)
    }
};
        locals.var_inv_ex = assign27140_e28915;
        locals.var_inv_ex_dn4 = assign27140_e28915_d_n4;
        locals.var_inv_ex_dn6 = assign27140_e28915_d_n6;
        locals.var_inv_ex_dn7 = assign27140_e28915_d_n7;
        locals.var_inv_ex_dn8 = assign27140_e28915_d_n8;
        locals.var_inv_ex_dn9 = assign27140_e28915_d_n9;

        let (assign27150_e28929, assign27150_e28929_d_n4, assign27150_e28929_d_n6, assign27150_e28929_d_n7, assign27150_e28929_d_n8, assign27150_e28929_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
        let assign27150_e28927: f64 = (locals.var_ex - locals.var_inv_ex);
        (assign27150_e28927, (locals.var_ex_dn4 - locals.var_inv_ex_dn4), (locals.var_ex_dn6 - locals.var_inv_ex_dn6), (locals.var_ex_dn7 - locals.var_inv_ex_dn7), (locals.var_ex_dn8 - locals.var_inv_ex_dn8), (locals.var_ex_dn9 - locals.var_inv_ex_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign27150_e28929;
        locals.var_temp_dn4 = assign27150_e28929_d_n4;
        locals.var_temp_dn6 = assign27150_e28929_d_n6;
        locals.var_temp_dn7 = assign27150_e28929_d_n7;
        locals.var_temp_dn8 = assign27150_e28929_d_n8;
        locals.var_temp_dn9 = assign27150_e28929_d_n9;

        let (assign27160_e28943, assign27160_e28943_d_n4, assign27160_e28943_d_n6, assign27160_e28943_d_n7, assign27160_e28943_d_n8, assign27160_e28943_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
        let assign27160_e28941: f64 = (locals.var_ex + locals.var_inv_ex);
        (assign27160_e28941, (locals.var_ex_dn4 + locals.var_inv_ex_dn4), (locals.var_ex_dn6 + locals.var_inv_ex_dn6), (locals.var_ex_dn7 + locals.var_inv_ex_dn7), (locals.var_ex_dn8 + locals.var_inv_ex_dn8), (locals.var_ex_dn9 + locals.var_inv_ex_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27160_e28943;
        locals.var_temp2_dn4 = assign27160_e28943_d_n4;
        locals.var_temp2_dn6 = assign27160_e28943_d_n6;
        locals.var_temp2_dn7 = assign27160_e28943_d_n7;
        locals.var_temp2_dn8 = assign27160_e28943_d_n8;
        locals.var_temp2_dn9 = assign27160_e28943_d_n9;

        let assign27280_e29070: f64 = if (((p.p4 > 0.0) && (locals.var_agidl_i > 0.0)) && (locals.var_vovs < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard772 = assign27280_e29070;

        let (assign27290_e29087, assign27290_e29087_d_n4, assign27290_e29087_d_n6, assign27290_e29087_d_n7, assign27290_e29087_d_n8, assign27290_e29087_d_n9,) = {
    if (locals.var_guard772 != 0.0) {
        let assign27290_e29074: f64 = (locals.var_vovs * locals.var_vovs);
        let assign27290_e29077: f64 = (locals.var_cgidl_i * locals.var_cgidl_i);
        let assign27290_e29079: f64 = (assign27290_e29077 * locals.var_vsbu);
        let assign27290_e29081: f64 = (assign27290_e29079 * locals.var_vsbu);
        let assign27290_e29082: f64 = (assign27290_e29074 + assign27290_e29081);
        let assign27290_e29084: f64 = (assign27290_e29082 + 1e-6);
        let assign27290_e29085: f64 = (assign27290_e29084).sqrt();
        (assign27290_e29085, (((locals.var_vovs_dn4 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn4)) / (2.0 * assign27290_e29085)), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) + (((assign27290_e29077 * locals.var_vsbu_dn6) * locals.var_vsbu) + (assign27290_e29079 * locals.var_vsbu_dn6))) / (2.0 * assign27290_e29085)), (((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) / (2.0 * assign27290_e29085)), ((((locals.var_vovs_dn8 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn8)) + (((assign27290_e29077 * locals.var_vsbu_dn8) * locals.var_vsbu) + (assign27290_e29079 * locals.var_vsbu_dn8))) / (2.0 * assign27290_e29085)), (((locals.var_vovs_dn9 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn9)) / (2.0 * assign27290_e29085)),)
    } else {
        (locals.var_vtovs, locals.var_vtovs_dn4, locals.var_vtovs_dn6, locals.var_vtovs_dn7, locals.var_vtovs_dn8, locals.var_vtovs_dn9,)
    }
};
        locals.var_vtovs = assign27290_e29087;
        locals.var_vtovs_dn4 = assign27290_e29087_d_n4;
        locals.var_vtovs_dn6 = assign27290_e29087_d_n6;
        locals.var_vtovs_dn7 = assign27290_e29087_d_n7;
        locals.var_vtovs_dn8 = assign27290_e29087_d_n8;
        locals.var_vtovs_dn9 = assign27290_e29087_d_n9;

        let (assign27300_e29094, assign27300_e29094_d_n4, assign27300_e29094_d_n6, assign27300_e29094_d_n7, assign27300_e29094_d_n8, assign27300_e29094_d_n9,) = {
    if (locals.var_guard772 != 0.0) {
        let assign27300_e29090: f64 = (-locals.var_bgidl_i);
        let assign27300_e29092: f64 = (assign27300_e29090 / locals.var_vtovs);
        (assign27300_e29092, ((((-locals.var_bgidl_i_dn4) * locals.var_vtovs) - (assign27300_e29090 * locals.var_vtovs_dn4)) / (locals.var_vtovs * locals.var_vtovs)), ((((-locals.var_bgidl_i_dn6) * locals.var_vtovs) - (assign27300_e29090 * locals.var_vtovs_dn6)) / (locals.var_vtovs * locals.var_vtovs)), ((((-locals.var_bgidl_i_dn7) * locals.var_vtovs) - (assign27300_e29090 * locals.var_vtovs_dn7)) / (locals.var_vtovs * locals.var_vtovs)), ((((-locals.var_bgidl_i_dn8) * locals.var_vtovs) - (assign27300_e29090 * locals.var_vtovs_dn8)) / (locals.var_vtovs * locals.var_vtovs)), ((((-locals.var_bgidl_i_dn9) * locals.var_vtovs) - (assign27300_e29090 * locals.var_vtovs_dn9)) / (locals.var_vtovs * locals.var_vtovs)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign27300_e29094;
        locals.var_temp_dn4 = assign27300_e29094_d_n4;
        locals.var_temp_dn6 = assign27300_e29094_d_n6;
        locals.var_temp_dn7 = assign27300_e29094_d_n7;
        locals.var_temp_dn8 = assign27300_e29094_d_n8;
        locals.var_temp_dn9 = assign27300_e29094_d_n9;

        let assign27310_e29096: f64 = (locals.var_temp).abs();
        let assign27310_e29098: f64 = if assign27310_e29096 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard773 = assign27310_e29098;

        let (assign27320_e29105, assign27320_e29105_d_n4, assign27320_e29105_d_n6, assign27320_e29105_d_n7, assign27320_e29105_d_n8, assign27320_e29105_d_n9,) = {
    if ((locals.var_guard772 != 0.0) && (locals.var_guard773 != 0.0)) {
        let assign27320_e29103: f64 = (locals.var_temp).exp();
        (assign27320_e29103, (assign27320_e29103 * locals.var_temp_dn4), (assign27320_e29103 * locals.var_temp_dn6), (assign27320_e29103 * locals.var_temp_dn7), (assign27320_e29103 * locals.var_temp_dn8), (assign27320_e29103 * locals.var_temp_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27320_e29105;
        locals.var_temp2_dn4 = assign27320_e29105_d_n4;
        locals.var_temp2_dn6 = assign27320_e29105_d_n6;
        locals.var_temp2_dn7 = assign27320_e29105_d_n7;
        locals.var_temp2_dn8 = assign27320_e29105_d_n8;
        locals.var_temp2_dn9 = assign27320_e29105_d_n9;

        let assign27330_e29108: f64 = (-80.0);
        let assign27330_e29109: f64 = if locals.var_temp < assign27330_e29108 { 1.0 } else { 0.0 };
        locals.var_guard774 = assign27330_e29109;

        let (assign27340_e29143, assign27340_e29143_d_n4, assign27340_e29143_d_n6, assign27340_e29143_d_n7, assign27340_e29143_d_n8, assign27340_e29143_d_n9,) = {
    if (((locals.var_guard772 != 0.0) && (locals.var_guard773 == 0.0)) && (locals.var_guard774 != 0.0)) {
        let assign27340_e29119: f64 = (-locals.var_temp);
        let assign27340_e29121: f64 = (assign27340_e29119 - 80.0);
        let assign27340_e29125: f64 = (-locals.var_temp);
        let assign27340_e29127: f64 = (assign27340_e29125 - 80.0);
        let assign27340_e29128: f64 = (0.5 * assign27340_e29127);
        let assign27340_e29131: f64 = (-locals.var_temp);
        let assign27340_e29133: f64 = (assign27340_e29131 - 80.0);
        let assign27340_e29135: f64 = (assign27340_e29133 * 0.3333333333333);
        let assign27340_e29136: f64 = (1.0 + assign27340_e29135);
        let assign27340_e29137: f64 = (assign27340_e29128 * assign27340_e29136);
        let assign27340_e29138: f64 = (1.0 + assign27340_e29137);
        let assign27340_e29139: f64 = (assign27340_e29121 * assign27340_e29138);
        let assign27340_e29140: f64 = (1.0 + assign27340_e29139);
        let assign27340_e29141: f64 = (1.80485e-35 / assign27340_e29140);
        (assign27340_e29141, (-((1.80485e-35 * (((-locals.var_temp_dn4) * assign27340_e29138) + (assign27340_e29121 * (((0.5 * (-locals.var_temp_dn4)) * assign27340_e29136) + (assign27340_e29128 * ((-locals.var_temp_dn4) * 0.3333333333333)))))) / (assign27340_e29140 * assign27340_e29140))), (-((1.80485e-35 * (((-locals.var_temp_dn6) * assign27340_e29138) + (assign27340_e29121 * (((0.5 * (-locals.var_temp_dn6)) * assign27340_e29136) + (assign27340_e29128 * ((-locals.var_temp_dn6) * 0.3333333333333)))))) / (assign27340_e29140 * assign27340_e29140))), (-((1.80485e-35 * (((-locals.var_temp_dn7) * assign27340_e29138) + (assign27340_e29121 * (((0.5 * (-locals.var_temp_dn7)) * assign27340_e29136) + (assign27340_e29128 * ((-locals.var_temp_dn7) * 0.3333333333333)))))) / (assign27340_e29140 * assign27340_e29140))), (-((1.80485e-35 * (((-locals.var_temp_dn8) * assign27340_e29138) + (assign27340_e29121 * (((0.5 * (-locals.var_temp_dn8)) * assign27340_e29136) + (assign27340_e29128 * ((-locals.var_temp_dn8) * 0.3333333333333)))))) / (assign27340_e29140 * assign27340_e29140))), (-((1.80485e-35 * (((-locals.var_temp_dn9) * assign27340_e29138) + (assign27340_e29121 * (((0.5 * (-locals.var_temp_dn9)) * assign27340_e29136) + (assign27340_e29128 * ((-locals.var_temp_dn9) * 0.3333333333333)))))) / (assign27340_e29140 * assign27340_e29140))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27340_e29143;
        locals.var_temp2_dn4 = assign27340_e29143_d_n4;
        locals.var_temp2_dn6 = assign27340_e29143_d_n6;
        locals.var_temp2_dn7 = assign27340_e29143_d_n7;
        locals.var_temp2_dn8 = assign27340_e29143_d_n8;
        locals.var_temp2_dn9 = assign27340_e29143_d_n9;

        let (assign27350_e29175, assign27350_e29175_d_n4, assign27350_e29175_d_n6, assign27350_e29175_d_n7, assign27350_e29175_d_n8, assign27350_e29175_d_n9,) = {
    if (((locals.var_guard772 != 0.0) && (locals.var_guard773 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign27350_e29155: f64 = (locals.var_temp - 80.0);
        let assign27350_e29160: f64 = (locals.var_temp - 80.0);
        let assign27350_e29161: f64 = (0.5 * assign27350_e29160);
        let assign27350_e29165: f64 = (locals.var_temp - 80.0);
        let assign27350_e29167: f64 = (assign27350_e29165 * 0.3333333333333);
        let assign27350_e29168: f64 = (1.0 + assign27350_e29167);
        let assign27350_e29169: f64 = (assign27350_e29161 * assign27350_e29168);
        let assign27350_e29170: f64 = (1.0 + assign27350_e29169);
        let assign27350_e29171: f64 = (assign27350_e29155 * assign27350_e29170);
        let assign27350_e29172: f64 = (1.0 + assign27350_e29171);
        let assign27350_e29173: f64 = (5.54062e34 * assign27350_e29172);
        (assign27350_e29173, (5.54062e34 * ((locals.var_temp_dn4 * assign27350_e29170) + (assign27350_e29155 * (((0.5 * locals.var_temp_dn4) * assign27350_e29168) + (assign27350_e29161 * (locals.var_temp_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn6 * assign27350_e29170) + (assign27350_e29155 * (((0.5 * locals.var_temp_dn6) * assign27350_e29168) + (assign27350_e29161 * (locals.var_temp_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn7 * assign27350_e29170) + (assign27350_e29155 * (((0.5 * locals.var_temp_dn7) * assign27350_e29168) + (assign27350_e29161 * (locals.var_temp_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn8 * assign27350_e29170) + (assign27350_e29155 * (((0.5 * locals.var_temp_dn8) * assign27350_e29168) + (assign27350_e29161 * (locals.var_temp_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn9 * assign27350_e29170) + (assign27350_e29155 * (((0.5 * locals.var_temp_dn9) * assign27350_e29168) + (assign27350_e29161 * (locals.var_temp_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27350_e29175;
        locals.var_temp2_dn4 = assign27350_e29175_d_n4;
        locals.var_temp2_dn6 = assign27350_e29175_d_n6;
        locals.var_temp2_dn7 = assign27350_e29175_d_n7;
        locals.var_temp2_dn8 = assign27350_e29175_d_n8;
        locals.var_temp2_dn9 = assign27350_e29175_d_n9;

        let (assign27360_e29181, assign27360_e29181_d_n4, assign27360_e29181_d_n6, assign27360_e29181_d_n7, assign27360_e29181_d_n8, assign27360_e29181_d_n9,) = {
    if (locals.var_guard772 != 0.0) {
        let assign27360_e29179: f64 = (locals.var_dgidl_i * locals.var_vsdu);
        (assign27360_e29179, 0.0, (locals.var_dgidl_i * locals.var_vsdu_dn6), (locals.var_dgidl_i * locals.var_vsdu_dn7), 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign27360_e29181;
        locals.var_temp3_dn4 = assign27360_e29181_d_n4;
        locals.var_temp3_dn6 = assign27360_e29181_d_n6;
        locals.var_temp3_dn7 = assign27360_e29181_d_n7;
        locals.var_temp3_dn8 = assign27360_e29181_d_n8;
        locals.var_temp3_dn9 = assign27360_e29181_d_n9;

        let assign27370_e29183: f64 = (locals.var_temp3).abs();
        let assign27370_e29185: f64 = if assign27370_e29183 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard775 = assign27370_e29185;

        let (assign27380_e29192, assign27380_e29192_d_n4, assign27380_e29192_d_n6, assign27380_e29192_d_n7, assign27380_e29192_d_n8, assign27380_e29192_d_n9,) = {
    if ((locals.var_guard772 != 0.0) && (locals.var_guard775 != 0.0)) {
        let assign27380_e29190: f64 = (locals.var_temp3).exp();
        (assign27380_e29190, (assign27380_e29190 * locals.var_temp3_dn4), (assign27380_e29190 * locals.var_temp3_dn6), (assign27380_e29190 * locals.var_temp3_dn7), (assign27380_e29190 * locals.var_temp3_dn8), (assign27380_e29190 * locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27380_e29192;
        locals.var_temp4_dn4 = assign27380_e29192_d_n4;
        locals.var_temp4_dn6 = assign27380_e29192_d_n6;
        locals.var_temp4_dn7 = assign27380_e29192_d_n7;
        locals.var_temp4_dn8 = assign27380_e29192_d_n8;
        locals.var_temp4_dn9 = assign27380_e29192_d_n9;

        let assign27390_e29195: f64 = (-80.0);
        let assign27390_e29196: f64 = if locals.var_temp3 < assign27390_e29195 { 1.0 } else { 0.0 };
        locals.var_guard776 = assign27390_e29196;

    }

    pub(super) fn stamp_transient_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27400_e29230, assign27400_e29230_d_n4, assign27400_e29230_d_n6, assign27400_e29230_d_n7, assign27400_e29230_d_n8, assign27400_e29230_d_n9,) = {
    if (((locals.var_guard772 != 0.0) && (locals.var_guard775 == 0.0)) && (locals.var_guard776 != 0.0)) {
        let assign27400_e29206: f64 = (-locals.var_temp3);
        let assign27400_e29208: f64 = (assign27400_e29206 - 80.0);
        let assign27400_e29212: f64 = (-locals.var_temp3);
        let assign27400_e29214: f64 = (assign27400_e29212 - 80.0);
        let assign27400_e29215: f64 = (0.5 * assign27400_e29214);
        let assign27400_e29218: f64 = (-locals.var_temp3);
        let assign27400_e29220: f64 = (assign27400_e29218 - 80.0);
        let assign27400_e29222: f64 = (assign27400_e29220 * 0.3333333333333);
        let assign27400_e29223: f64 = (1.0 + assign27400_e29222);
        let assign27400_e29224: f64 = (assign27400_e29215 * assign27400_e29223);
        let assign27400_e29225: f64 = (1.0 + assign27400_e29224);
        let assign27400_e29226: f64 = (assign27400_e29208 * assign27400_e29225);
        let assign27400_e29227: f64 = (1.0 + assign27400_e29226);
        let assign27400_e29228: f64 = (1.80485e-35 / assign27400_e29227);
        (assign27400_e29228, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign27400_e29225) + (assign27400_e29208 * (((0.5 * (-locals.var_temp3_dn4)) * assign27400_e29223) + (assign27400_e29215 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign27400_e29227 * assign27400_e29227))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign27400_e29225) + (assign27400_e29208 * (((0.5 * (-locals.var_temp3_dn6)) * assign27400_e29223) + (assign27400_e29215 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign27400_e29227 * assign27400_e29227))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign27400_e29225) + (assign27400_e29208 * (((0.5 * (-locals.var_temp3_dn7)) * assign27400_e29223) + (assign27400_e29215 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign27400_e29227 * assign27400_e29227))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign27400_e29225) + (assign27400_e29208 * (((0.5 * (-locals.var_temp3_dn8)) * assign27400_e29223) + (assign27400_e29215 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign27400_e29227 * assign27400_e29227))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign27400_e29225) + (assign27400_e29208 * (((0.5 * (-locals.var_temp3_dn9)) * assign27400_e29223) + (assign27400_e29215 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign27400_e29227 * assign27400_e29227))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27400_e29230;
        locals.var_temp4_dn4 = assign27400_e29230_d_n4;
        locals.var_temp4_dn6 = assign27400_e29230_d_n6;
        locals.var_temp4_dn7 = assign27400_e29230_d_n7;
        locals.var_temp4_dn8 = assign27400_e29230_d_n8;
        locals.var_temp4_dn9 = assign27400_e29230_d_n9;

        let (assign27410_e29262, assign27410_e29262_d_n4, assign27410_e29262_d_n6, assign27410_e29262_d_n7, assign27410_e29262_d_n8, assign27410_e29262_d_n9,) = {
    if (((locals.var_guard772 != 0.0) && (locals.var_guard775 == 0.0)) && (locals.var_guard776 == 0.0)) {
        let assign27410_e29242: f64 = (locals.var_temp3 - 80.0);
        let assign27410_e29247: f64 = (locals.var_temp3 - 80.0);
        let assign27410_e29248: f64 = (0.5 * assign27410_e29247);
        let assign27410_e29252: f64 = (locals.var_temp3 - 80.0);
        let assign27410_e29254: f64 = (assign27410_e29252 * 0.3333333333333);
        let assign27410_e29255: f64 = (1.0 + assign27410_e29254);
        let assign27410_e29256: f64 = (assign27410_e29248 * assign27410_e29255);
        let assign27410_e29257: f64 = (1.0 + assign27410_e29256);
        let assign27410_e29258: f64 = (assign27410_e29242 * assign27410_e29257);
        let assign27410_e29259: f64 = (1.0 + assign27410_e29258);
        let assign27410_e29260: f64 = (5.54062e34 * assign27410_e29259);
        (assign27410_e29260, (5.54062e34 * ((locals.var_temp3_dn4 * assign27410_e29257) + (assign27410_e29242 * (((0.5 * locals.var_temp3_dn4) * assign27410_e29255) + (assign27410_e29248 * (locals.var_temp3_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn6 * assign27410_e29257) + (assign27410_e29242 * (((0.5 * locals.var_temp3_dn6) * assign27410_e29255) + (assign27410_e29248 * (locals.var_temp3_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn7 * assign27410_e29257) + (assign27410_e29242 * (((0.5 * locals.var_temp3_dn7) * assign27410_e29255) + (assign27410_e29248 * (locals.var_temp3_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn8 * assign27410_e29257) + (assign27410_e29242 * (((0.5 * locals.var_temp3_dn8) * assign27410_e29255) + (assign27410_e29248 * (locals.var_temp3_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn9 * assign27410_e29257) + (assign27410_e29242 * (((0.5 * locals.var_temp3_dn9) * assign27410_e29255) + (assign27410_e29248 * (locals.var_temp3_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27410_e29262;
        locals.var_temp4_dn4 = assign27410_e29262_d_n4;
        locals.var_temp4_dn6 = assign27410_e29262_d_n6;
        locals.var_temp4_dn7 = assign27410_e29262_d_n7;
        locals.var_temp4_dn8 = assign27410_e29262_d_n8;
        locals.var_temp4_dn9 = assign27410_e29262_d_n9;

        let assign27440_e29293: f64 = if (((p.p4 > 0.0) && (locals.var_agidld_i > 0.0)) && (locals.var_vovd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard777 = assign27440_e29293;

        let (assign27450_e29310, assign27450_e29310_d_n4, assign27450_e29310_d_n6, assign27450_e29310_d_n7, assign27450_e29310_d_n8, assign27450_e29310_d_n9,) = {
    if (locals.var_guard777 != 0.0) {
        let assign27450_e29297: f64 = (locals.var_vovd * locals.var_vovd);
        let assign27450_e29300: f64 = (locals.var_cgidld_i * locals.var_cgidld_i);
        let assign27450_e29302: f64 = (assign27450_e29300 * locals.var_vdbu);
        let assign27450_e29304: f64 = (assign27450_e29302 * locals.var_vdbu);
        let assign27450_e29305: f64 = (assign27450_e29297 + assign27450_e29304);
        let assign27450_e29307: f64 = (assign27450_e29305 + 1e-6);
        let assign27450_e29308: f64 = (assign27450_e29307).sqrt();
        (assign27450_e29308, (((locals.var_vovd_dn4 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn4)) / (2.0 * assign27450_e29308)), ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) + (((assign27450_e29300 * locals.var_vdbu_dn6) * locals.var_vdbu) + (assign27450_e29302 * locals.var_vdbu_dn6))) / (2.0 * assign27450_e29308)), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) + (((assign27450_e29300 * locals.var_vdbu_dn7) * locals.var_vdbu) + (assign27450_e29302 * locals.var_vdbu_dn7))) / (2.0 * assign27450_e29308)), ((((locals.var_vovd_dn8 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn8)) + (((assign27450_e29300 * locals.var_vdbu_dn8) * locals.var_vdbu) + (assign27450_e29302 * locals.var_vdbu_dn8))) / (2.0 * assign27450_e29308)), (((locals.var_vovd_dn9 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn9)) / (2.0 * assign27450_e29308)),)
    } else {
        (locals.var_vtovd, locals.var_vtovd_dn4, locals.var_vtovd_dn6, locals.var_vtovd_dn7, locals.var_vtovd_dn8, locals.var_vtovd_dn9,)
    }
};
        locals.var_vtovd = assign27450_e29310;
        locals.var_vtovd_dn4 = assign27450_e29310_d_n4;
        locals.var_vtovd_dn6 = assign27450_e29310_d_n6;
        locals.var_vtovd_dn7 = assign27450_e29310_d_n7;
        locals.var_vtovd_dn8 = assign27450_e29310_d_n8;
        locals.var_vtovd_dn9 = assign27450_e29310_d_n9;

        let (assign27460_e29317, assign27460_e29317_d_n4, assign27460_e29317_d_n6, assign27460_e29317_d_n7, assign27460_e29317_d_n8, assign27460_e29317_d_n9,) = {
    if (locals.var_guard777 != 0.0) {
        let assign27460_e29313: f64 = (-locals.var_bgidld_i);
        let assign27460_e29315: f64 = (assign27460_e29313 / locals.var_vtovd);
        (assign27460_e29315, ((((-locals.var_bgidld_i_dn4) * locals.var_vtovd) - (assign27460_e29313 * locals.var_vtovd_dn4)) / (locals.var_vtovd * locals.var_vtovd)), ((((-locals.var_bgidld_i_dn6) * locals.var_vtovd) - (assign27460_e29313 * locals.var_vtovd_dn6)) / (locals.var_vtovd * locals.var_vtovd)), ((((-locals.var_bgidld_i_dn7) * locals.var_vtovd) - (assign27460_e29313 * locals.var_vtovd_dn7)) / (locals.var_vtovd * locals.var_vtovd)), ((((-locals.var_bgidld_i_dn8) * locals.var_vtovd) - (assign27460_e29313 * locals.var_vtovd_dn8)) / (locals.var_vtovd * locals.var_vtovd)), ((((-locals.var_bgidld_i_dn9) * locals.var_vtovd) - (assign27460_e29313 * locals.var_vtovd_dn9)) / (locals.var_vtovd * locals.var_vtovd)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign27460_e29317;
        locals.var_temp_dn4 = assign27460_e29317_d_n4;
        locals.var_temp_dn6 = assign27460_e29317_d_n6;
        locals.var_temp_dn7 = assign27460_e29317_d_n7;
        locals.var_temp_dn8 = assign27460_e29317_d_n8;
        locals.var_temp_dn9 = assign27460_e29317_d_n9;

        let assign27470_e29319: f64 = (locals.var_temp).abs();
        let assign27470_e29321: f64 = if assign27470_e29319 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard778 = assign27470_e29321;

        let (assign27480_e29328, assign27480_e29328_d_n4, assign27480_e29328_d_n6, assign27480_e29328_d_n7, assign27480_e29328_d_n8, assign27480_e29328_d_n9,) = {
    if ((locals.var_guard777 != 0.0) && (locals.var_guard778 != 0.0)) {
        let assign27480_e29326: f64 = (locals.var_temp).exp();
        (assign27480_e29326, (assign27480_e29326 * locals.var_temp_dn4), (assign27480_e29326 * locals.var_temp_dn6), (assign27480_e29326 * locals.var_temp_dn7), (assign27480_e29326 * locals.var_temp_dn8), (assign27480_e29326 * locals.var_temp_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27480_e29328;
        locals.var_temp2_dn4 = assign27480_e29328_d_n4;
        locals.var_temp2_dn6 = assign27480_e29328_d_n6;
        locals.var_temp2_dn7 = assign27480_e29328_d_n7;
        locals.var_temp2_dn8 = assign27480_e29328_d_n8;
        locals.var_temp2_dn9 = assign27480_e29328_d_n9;

        let assign27490_e29331: f64 = (-80.0);
        let assign27490_e29332: f64 = if locals.var_temp < assign27490_e29331 { 1.0 } else { 0.0 };
        locals.var_guard779 = assign27490_e29332;

        let (assign27500_e29366, assign27500_e29366_d_n4, assign27500_e29366_d_n6, assign27500_e29366_d_n7, assign27500_e29366_d_n8, assign27500_e29366_d_n9,) = {
    if (((locals.var_guard777 != 0.0) && (locals.var_guard778 == 0.0)) && (locals.var_guard779 != 0.0)) {
        let assign27500_e29342: f64 = (-locals.var_temp);
        let assign27500_e29344: f64 = (assign27500_e29342 - 80.0);
        let assign27500_e29348: f64 = (-locals.var_temp);
        let assign27500_e29350: f64 = (assign27500_e29348 - 80.0);
        let assign27500_e29351: f64 = (0.5 * assign27500_e29350);
        let assign27500_e29354: f64 = (-locals.var_temp);
        let assign27500_e29356: f64 = (assign27500_e29354 - 80.0);
        let assign27500_e29358: f64 = (assign27500_e29356 * 0.3333333333333);
        let assign27500_e29359: f64 = (1.0 + assign27500_e29358);
        let assign27500_e29360: f64 = (assign27500_e29351 * assign27500_e29359);
        let assign27500_e29361: f64 = (1.0 + assign27500_e29360);
        let assign27500_e29362: f64 = (assign27500_e29344 * assign27500_e29361);
        let assign27500_e29363: f64 = (1.0 + assign27500_e29362);
        let assign27500_e29364: f64 = (1.80485e-35 / assign27500_e29363);
        (assign27500_e29364, (-((1.80485e-35 * (((-locals.var_temp_dn4) * assign27500_e29361) + (assign27500_e29344 * (((0.5 * (-locals.var_temp_dn4)) * assign27500_e29359) + (assign27500_e29351 * ((-locals.var_temp_dn4) * 0.3333333333333)))))) / (assign27500_e29363 * assign27500_e29363))), (-((1.80485e-35 * (((-locals.var_temp_dn6) * assign27500_e29361) + (assign27500_e29344 * (((0.5 * (-locals.var_temp_dn6)) * assign27500_e29359) + (assign27500_e29351 * ((-locals.var_temp_dn6) * 0.3333333333333)))))) / (assign27500_e29363 * assign27500_e29363))), (-((1.80485e-35 * (((-locals.var_temp_dn7) * assign27500_e29361) + (assign27500_e29344 * (((0.5 * (-locals.var_temp_dn7)) * assign27500_e29359) + (assign27500_e29351 * ((-locals.var_temp_dn7) * 0.3333333333333)))))) / (assign27500_e29363 * assign27500_e29363))), (-((1.80485e-35 * (((-locals.var_temp_dn8) * assign27500_e29361) + (assign27500_e29344 * (((0.5 * (-locals.var_temp_dn8)) * assign27500_e29359) + (assign27500_e29351 * ((-locals.var_temp_dn8) * 0.3333333333333)))))) / (assign27500_e29363 * assign27500_e29363))), (-((1.80485e-35 * (((-locals.var_temp_dn9) * assign27500_e29361) + (assign27500_e29344 * (((0.5 * (-locals.var_temp_dn9)) * assign27500_e29359) + (assign27500_e29351 * ((-locals.var_temp_dn9) * 0.3333333333333)))))) / (assign27500_e29363 * assign27500_e29363))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27500_e29366;
        locals.var_temp2_dn4 = assign27500_e29366_d_n4;
        locals.var_temp2_dn6 = assign27500_e29366_d_n6;
        locals.var_temp2_dn7 = assign27500_e29366_d_n7;
        locals.var_temp2_dn8 = assign27500_e29366_d_n8;
        locals.var_temp2_dn9 = assign27500_e29366_d_n9;

        let (assign27510_e29398, assign27510_e29398_d_n4, assign27510_e29398_d_n6, assign27510_e29398_d_n7, assign27510_e29398_d_n8, assign27510_e29398_d_n9,) = {
    if (((locals.var_guard777 != 0.0) && (locals.var_guard778 == 0.0)) && (locals.var_guard779 == 0.0)) {
        let assign27510_e29378: f64 = (locals.var_temp - 80.0);
        let assign27510_e29383: f64 = (locals.var_temp - 80.0);
        let assign27510_e29384: f64 = (0.5 * assign27510_e29383);
        let assign27510_e29388: f64 = (locals.var_temp - 80.0);
        let assign27510_e29390: f64 = (assign27510_e29388 * 0.3333333333333);
        let assign27510_e29391: f64 = (1.0 + assign27510_e29390);
        let assign27510_e29392: f64 = (assign27510_e29384 * assign27510_e29391);
        let assign27510_e29393: f64 = (1.0 + assign27510_e29392);
        let assign27510_e29394: f64 = (assign27510_e29378 * assign27510_e29393);
        let assign27510_e29395: f64 = (1.0 + assign27510_e29394);
        let assign27510_e29396: f64 = (5.54062e34 * assign27510_e29395);
        (assign27510_e29396, (5.54062e34 * ((locals.var_temp_dn4 * assign27510_e29393) + (assign27510_e29378 * (((0.5 * locals.var_temp_dn4) * assign27510_e29391) + (assign27510_e29384 * (locals.var_temp_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn6 * assign27510_e29393) + (assign27510_e29378 * (((0.5 * locals.var_temp_dn6) * assign27510_e29391) + (assign27510_e29384 * (locals.var_temp_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn7 * assign27510_e29393) + (assign27510_e29378 * (((0.5 * locals.var_temp_dn7) * assign27510_e29391) + (assign27510_e29384 * (locals.var_temp_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn8 * assign27510_e29393) + (assign27510_e29378 * (((0.5 * locals.var_temp_dn8) * assign27510_e29391) + (assign27510_e29384 * (locals.var_temp_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn9 * assign27510_e29393) + (assign27510_e29378 * (((0.5 * locals.var_temp_dn9) * assign27510_e29391) + (assign27510_e29384 * (locals.var_temp_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27510_e29398;
        locals.var_temp2_dn4 = assign27510_e29398_d_n4;
        locals.var_temp2_dn6 = assign27510_e29398_d_n6;
        locals.var_temp2_dn7 = assign27510_e29398_d_n7;
        locals.var_temp2_dn8 = assign27510_e29398_d_n8;
        locals.var_temp2_dn9 = assign27510_e29398_d_n9;

        let (assign27520_e29404, assign27520_e29404_d_n4, assign27520_e29404_d_n6, assign27520_e29404_d_n7, assign27520_e29404_d_n8, assign27520_e29404_d_n9,) = {
    if (locals.var_guard777 != 0.0) {
        let assign27520_e29402: f64 = (locals.var_dgidld_i * locals.var_vdsu);
        (assign27520_e29402, 0.0, (locals.var_dgidld_i * locals.var_vdsu_dn6), (locals.var_dgidld_i * locals.var_vdsu_dn7), 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign27520_e29404;
        locals.var_temp3_dn4 = assign27520_e29404_d_n4;
        locals.var_temp3_dn6 = assign27520_e29404_d_n6;
        locals.var_temp3_dn7 = assign27520_e29404_d_n7;
        locals.var_temp3_dn8 = assign27520_e29404_d_n8;
        locals.var_temp3_dn9 = assign27520_e29404_d_n9;

        let assign27530_e29406: f64 = (locals.var_temp3).abs();
        let assign27530_e29408: f64 = if assign27530_e29406 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard780 = assign27530_e29408;

        let (assign27540_e29415, assign27540_e29415_d_n4, assign27540_e29415_d_n6, assign27540_e29415_d_n7, assign27540_e29415_d_n8, assign27540_e29415_d_n9,) = {
    if ((locals.var_guard777 != 0.0) && (locals.var_guard780 != 0.0)) {
        let assign27540_e29413: f64 = (locals.var_temp3).exp();
        (assign27540_e29413, (assign27540_e29413 * locals.var_temp3_dn4), (assign27540_e29413 * locals.var_temp3_dn6), (assign27540_e29413 * locals.var_temp3_dn7), (assign27540_e29413 * locals.var_temp3_dn8), (assign27540_e29413 * locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27540_e29415;
        locals.var_temp4_dn4 = assign27540_e29415_d_n4;
        locals.var_temp4_dn6 = assign27540_e29415_d_n6;
        locals.var_temp4_dn7 = assign27540_e29415_d_n7;
        locals.var_temp4_dn8 = assign27540_e29415_d_n8;
        locals.var_temp4_dn9 = assign27540_e29415_d_n9;

        let assign27550_e29418: f64 = (-80.0);
        let assign27550_e29419: f64 = if locals.var_temp3 < assign27550_e29418 { 1.0 } else { 0.0 };
        locals.var_guard781 = assign27550_e29419;

        let (assign27560_e29453, assign27560_e29453_d_n4, assign27560_e29453_d_n6, assign27560_e29453_d_n7, assign27560_e29453_d_n8, assign27560_e29453_d_n9,) = {
    if (((locals.var_guard777 != 0.0) && (locals.var_guard780 == 0.0)) && (locals.var_guard781 != 0.0)) {
        let assign27560_e29429: f64 = (-locals.var_temp3);
        let assign27560_e29431: f64 = (assign27560_e29429 - 80.0);
        let assign27560_e29435: f64 = (-locals.var_temp3);
        let assign27560_e29437: f64 = (assign27560_e29435 - 80.0);
        let assign27560_e29438: f64 = (0.5 * assign27560_e29437);
        let assign27560_e29441: f64 = (-locals.var_temp3);
        let assign27560_e29443: f64 = (assign27560_e29441 - 80.0);
        let assign27560_e29445: f64 = (assign27560_e29443 * 0.3333333333333);
        let assign27560_e29446: f64 = (1.0 + assign27560_e29445);
        let assign27560_e29447: f64 = (assign27560_e29438 * assign27560_e29446);
        let assign27560_e29448: f64 = (1.0 + assign27560_e29447);
        let assign27560_e29449: f64 = (assign27560_e29431 * assign27560_e29448);
        let assign27560_e29450: f64 = (1.0 + assign27560_e29449);
        let assign27560_e29451: f64 = (1.80485e-35 / assign27560_e29450);
        (assign27560_e29451, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign27560_e29448) + (assign27560_e29431 * (((0.5 * (-locals.var_temp3_dn4)) * assign27560_e29446) + (assign27560_e29438 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign27560_e29450 * assign27560_e29450))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign27560_e29448) + (assign27560_e29431 * (((0.5 * (-locals.var_temp3_dn6)) * assign27560_e29446) + (assign27560_e29438 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign27560_e29450 * assign27560_e29450))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign27560_e29448) + (assign27560_e29431 * (((0.5 * (-locals.var_temp3_dn7)) * assign27560_e29446) + (assign27560_e29438 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign27560_e29450 * assign27560_e29450))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign27560_e29448) + (assign27560_e29431 * (((0.5 * (-locals.var_temp3_dn8)) * assign27560_e29446) + (assign27560_e29438 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign27560_e29450 * assign27560_e29450))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign27560_e29448) + (assign27560_e29431 * (((0.5 * (-locals.var_temp3_dn9)) * assign27560_e29446) + (assign27560_e29438 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign27560_e29450 * assign27560_e29450))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27560_e29453;
        locals.var_temp4_dn4 = assign27560_e29453_d_n4;
        locals.var_temp4_dn6 = assign27560_e29453_d_n6;
        locals.var_temp4_dn7 = assign27560_e29453_d_n7;
        locals.var_temp4_dn8 = assign27560_e29453_d_n8;
        locals.var_temp4_dn9 = assign27560_e29453_d_n9;

        let (assign27570_e29485, assign27570_e29485_d_n4, assign27570_e29485_d_n6, assign27570_e29485_d_n7, assign27570_e29485_d_n8, assign27570_e29485_d_n9,) = {
    if (((locals.var_guard777 != 0.0) && (locals.var_guard780 == 0.0)) && (locals.var_guard781 == 0.0)) {
        let assign27570_e29465: f64 = (locals.var_temp3 - 80.0);
        let assign27570_e29470: f64 = (locals.var_temp3 - 80.0);
        let assign27570_e29471: f64 = (0.5 * assign27570_e29470);
        let assign27570_e29475: f64 = (locals.var_temp3 - 80.0);
        let assign27570_e29477: f64 = (assign27570_e29475 * 0.3333333333333);
        let assign27570_e29478: f64 = (1.0 + assign27570_e29477);
        let assign27570_e29479: f64 = (assign27570_e29471 * assign27570_e29478);
        let assign27570_e29480: f64 = (1.0 + assign27570_e29479);
        let assign27570_e29481: f64 = (assign27570_e29465 * assign27570_e29480);
        let assign27570_e29482: f64 = (1.0 + assign27570_e29481);
        let assign27570_e29483: f64 = (5.54062e34 * assign27570_e29482);
        (assign27570_e29483, (5.54062e34 * ((locals.var_temp3_dn4 * assign27570_e29480) + (assign27570_e29465 * (((0.5 * locals.var_temp3_dn4) * assign27570_e29478) + (assign27570_e29471 * (locals.var_temp3_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn6 * assign27570_e29480) + (assign27570_e29465 * (((0.5 * locals.var_temp3_dn6) * assign27570_e29478) + (assign27570_e29471 * (locals.var_temp3_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn7 * assign27570_e29480) + (assign27570_e29465 * (((0.5 * locals.var_temp3_dn7) * assign27570_e29478) + (assign27570_e29471 * (locals.var_temp3_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn8 * assign27570_e29480) + (assign27570_e29465 * (((0.5 * locals.var_temp3_dn8) * assign27570_e29478) + (assign27570_e29471 * (locals.var_temp3_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn9 * assign27570_e29480) + (assign27570_e29465 * (((0.5 * locals.var_temp3_dn9) * assign27570_e29478) + (assign27570_e29471 * (locals.var_temp3_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27570_e29485;
        locals.var_temp4_dn4 = assign27570_e29485_d_n4;
        locals.var_temp4_dn6 = assign27570_e29485_d_n6;
        locals.var_temp4_dn7 = assign27570_e29485_d_n7;
        locals.var_temp4_dn8 = assign27570_e29485_d_n8;
        locals.var_temp4_dn9 = assign27570_e29485_d_n9;

        locals.var_ids_edge = 0.0;
        locals.var_ids_edge_dn4 = 0.0;
        locals.var_ids_edge_dn6 = 0.0;
        locals.var_ids_edge_dn7 = 0.0;
        locals.var_ids_edge_dn8 = 0.0;
        locals.var_ids_edge_dn9 = 0.0;

        let assign27600_e29508: f64 = if p.p12 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard782 = assign27600_e29508;

        let (assign27610_e29514, assign27610_e29514_d_n4, assign27610_e29514_d_n6, assign27610_e29514_d_n7, assign27610_e29514_d_n8, assign27610_e29514_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27610_e29512: f64 = (locals.var_vds * locals.var_inv_phit_edge);
        (assign27610_e29512, (locals.var_vds * locals.var_inv_phit_edge_dn4), ((locals.var_vds_dn6 * locals.var_inv_phit_edge) + (locals.var_vds * locals.var_inv_phit_edge_dn6)), ((locals.var_vds_dn7 * locals.var_inv_phit_edge) + (locals.var_vds * locals.var_inv_phit_edge_dn7)), (locals.var_vds * locals.var_inv_phit_edge_dn8), (locals.var_vds * locals.var_inv_phit_edge_dn9),)
    } else {
        (locals.var_xd_edge, locals.var_xd_edge_dn4, locals.var_xd_edge_dn6, locals.var_xd_edge_dn7, locals.var_xd_edge_dn8, locals.var_xd_edge_dn9,)
    }
};
        locals.var_xd_edge = assign27610_e29514;
        locals.var_xd_edge_dn4 = assign27610_e29514_d_n4;
        locals.var_xd_edge_dn6 = assign27610_e29514_d_n6;
        locals.var_xd_edge_dn7 = assign27610_e29514_d_n7;
        locals.var_xd_edge_dn8 = assign27610_e29514_d_n8;
        locals.var_xd_edge_dn9 = assign27610_e29514_d_n9;

        let (assign27620_e29527, assign27620_e29527_d_n4, assign27620_e29527_d_n6, assign27620_e29527_d_n7, assign27620_e29527_d_n8, assign27620_e29527_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27620_e29518: f64 = (locals.var_vds * locals.var_vds);
        let assign27620_e29520: f64 = (assign27620_e29518 + 0.01);
        let assign27620_e29521: f64 = (assign27620_e29520).sqrt();
        let assign27620_e29523: f64 = (assign27620_e29521 - 0.1);
        let assign27620_e29525: f64 = (assign27620_e29523 * locals.var_inv_phit_edge);
        (assign27620_e29525, (assign27620_e29523 * locals.var_inv_phit_edge_dn4), (((((locals.var_vds_dn6 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn6)) / (2.0 * assign27620_e29521)) * locals.var_inv_phit_edge) + (assign27620_e29523 * locals.var_inv_phit_edge_dn6)), (((((locals.var_vds_dn7 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn7)) / (2.0 * assign27620_e29521)) * locals.var_inv_phit_edge) + (assign27620_e29523 * locals.var_inv_phit_edge_dn7)), (assign27620_e29523 * locals.var_inv_phit_edge_dn8), (assign27620_e29523 * locals.var_inv_phit_edge_dn9),)
    } else {
        (locals.var_xdsx_edge, locals.var_xdsx_edge_dn4, locals.var_xdsx_edge_dn6, locals.var_xdsx_edge_dn7, locals.var_xdsx_edge_dn8, locals.var_xdsx_edge_dn9,)
    }
};
        locals.var_xdsx_edge = assign27620_e29527;
        locals.var_xdsx_edge_dn4 = assign27620_e29527_d_n4;
        locals.var_xdsx_edge_dn6 = assign27620_e29527_d_n6;
        locals.var_xdsx_edge_dn7 = assign27620_e29527_d_n7;
        locals.var_xdsx_edge_dn8 = assign27620_e29527_d_n8;
        locals.var_xdsx_edge_dn9 = assign27620_e29527_d_n9;

        let (assign27630_e29535, assign27630_e29535_d_n4, assign27630_e29535_d_n6, assign27630_e29535_d_n7, assign27630_e29535_d_n8, assign27630_e29535_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27630_e29532: f64 = (locals.var_xd_edge - locals.var_xdsx_edge);
        let assign27630_e29533: f64 = (0.5 * assign27630_e29532);
        (assign27630_e29533, (0.5 * (locals.var_xd_edge_dn4 - locals.var_xdsx_edge_dn4)), (0.5 * (locals.var_xd_edge_dn6 - locals.var_xdsx_edge_dn6)), (0.5 * (locals.var_xd_edge_dn7 - locals.var_xdsx_edge_dn7)), (0.5 * (locals.var_xd_edge_dn8 - locals.var_xdsx_edge_dn8)), (0.5 * (locals.var_xd_edge_dn9 - locals.var_xdsx_edge_dn9)),)
    } else {
        (locals.var_dxdsx_edge, locals.var_dxdsx_edge_dn4, locals.var_dxdsx_edge_dn6, locals.var_dxdsx_edge_dn7, locals.var_dxdsx_edge_dn8, locals.var_dxdsx_edge_dn9,)
    }
};
        locals.var_dxdsx_edge = assign27630_e29535;
        locals.var_dxdsx_edge_dn4 = assign27630_e29535_d_n4;
        locals.var_dxdsx_edge_dn6 = assign27630_e29535_d_n6;
        locals.var_dxdsx_edge_dn7 = assign27630_e29535_d_n7;
        locals.var_dxdsx_edge_dn8 = assign27630_e29535_d_n8;
        locals.var_dxdsx_edge_dn9 = assign27630_e29535_d_n9;

        let (assign27640_e29547, assign27640_e29547_d_n4, assign27640_e29547_d_n6, assign27640_e29547_d_n7, assign27640_e29547_d_n8, assign27640_e29547_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27640_e29539: f64 = (locals.var_vgs - locals.var_vfb1edge_i);
        let assign27640_e29541: f64 = (assign27640_e29539 * locals.var_inv_phit_edge);
        let assign27640_e29543: f64 = (assign27640_e29541 - locals.var_dxdsx_edge);
        let assign27640_e29545: f64 = (assign27640_e29543 - locals.var_eg_2phit0);
        (assign27640_e29545, (((((-locals.var_vfb1edge_i_dn4) * locals.var_inv_phit_edge) + (assign27640_e29539 * locals.var_inv_phit_edge_dn4)) - locals.var_dxdsx_edge_dn4) - locals.var_eg_2phit0_dn4), (((((locals.var_vgs_dn6 - locals.var_vfb1edge_i_dn6) * locals.var_inv_phit_edge) + (assign27640_e29539 * locals.var_inv_phit_edge_dn6)) - locals.var_dxdsx_edge_dn6) - locals.var_eg_2phit0_dn6), (((((locals.var_vgs_dn7 - locals.var_vfb1edge_i_dn7) * locals.var_inv_phit_edge) + (assign27640_e29539 * locals.var_inv_phit_edge_dn7)) - locals.var_dxdsx_edge_dn7) - locals.var_eg_2phit0_dn7), (((((-locals.var_vfb1edge_i_dn8) * locals.var_inv_phit_edge) + (assign27640_e29539 * locals.var_inv_phit_edge_dn8)) - locals.var_dxdsx_edge_dn8) - locals.var_eg_2phit0_dn8), (((((locals.var_vgs_dn9 - locals.var_vfb1edge_i_dn9) * locals.var_inv_phit_edge) + (assign27640_e29539 * locals.var_inv_phit_edge_dn9)) - locals.var_dxdsx_edge_dn9) - locals.var_eg_2phit0_dn9),)
    } else {
        (locals.var_xg10_edge, locals.var_xg10_edge_dn4, locals.var_xg10_edge_dn6, locals.var_xg10_edge_dn7, locals.var_xg10_edge_dn8, locals.var_xg10_edge_dn9,)
    }
};
        locals.var_xg10_edge = assign27640_e29547;
        locals.var_xg10_edge_dn4 = assign27640_e29547_d_n4;
        locals.var_xg10_edge_dn6 = assign27640_e29547_d_n6;
        locals.var_xg10_edge_dn7 = assign27640_e29547_d_n7;
        locals.var_xg10_edge_dn8 = assign27640_e29547_d_n8;
        locals.var_xg10_edge_dn9 = assign27640_e29547_d_n9;

        let (assign27650_e29560, assign27650_e29560_d_n4, assign27650_e29560_d_n6, assign27650_e29560_d_n7, assign27650_e29560_d_n8, assign27650_e29560_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27650_e29550: f64 = (-locals.var_vsb);
        let assign27650_e29552: f64 = (assign27650_e29550 - locals.var_vfb2edge_i);
        let assign27650_e29554: f64 = (assign27650_e29552 * locals.var_inv_phit_edge);
        let assign27650_e29556: f64 = (assign27650_e29554 - locals.var_dxdsx_edge);
        let assign27650_e29558: f64 = (assign27650_e29556 - locals.var_eg_2phit0);
        (assign27650_e29558, (((((-locals.var_vfb2edge_i_dn4) * locals.var_inv_phit_edge) + (assign27650_e29552 * locals.var_inv_phit_edge_dn4)) - locals.var_dxdsx_edge_dn4) - locals.var_eg_2phit0_dn4), ((((((-locals.var_vsb_dn6) - locals.var_vfb2edge_i_dn6) * locals.var_inv_phit_edge) + (assign27650_e29552 * locals.var_inv_phit_edge_dn6)) - locals.var_dxdsx_edge_dn6) - locals.var_eg_2phit0_dn6), ((((((-locals.var_vsb_dn7) - locals.var_vfb2edge_i_dn7) * locals.var_inv_phit_edge) + (assign27650_e29552 * locals.var_inv_phit_edge_dn7)) - locals.var_dxdsx_edge_dn7) - locals.var_eg_2phit0_dn7), ((((((-locals.var_vsb_dn8) - locals.var_vfb2edge_i_dn8) * locals.var_inv_phit_edge) + (assign27650_e29552 * locals.var_inv_phit_edge_dn8)) - locals.var_dxdsx_edge_dn8) - locals.var_eg_2phit0_dn8), (((((-locals.var_vfb2edge_i_dn9) * locals.var_inv_phit_edge) + (assign27650_e29552 * locals.var_inv_phit_edge_dn9)) - locals.var_dxdsx_edge_dn9) - locals.var_eg_2phit0_dn9),)
    } else {
        (locals.var_xg20_edge, locals.var_xg20_edge_dn4, locals.var_xg20_edge_dn6, locals.var_xg20_edge_dn7, locals.var_xg20_edge_dn8, locals.var_xg20_edge_dn9,)
    }
};
        locals.var_xg20_edge = assign27650_e29560;
        locals.var_xg20_edge_dn4 = assign27650_e29560_d_n4;
        locals.var_xg20_edge_dn6 = assign27650_e29560_d_n6;
        locals.var_xg20_edge_dn7 = assign27650_e29560_d_n7;
        locals.var_xg20_edge_dn8 = assign27650_e29560_d_n8;
        locals.var_xg20_edge_dn9 = assign27650_e29560_d_n9;

        let (assign27660_e29568, assign27660_e29568_d_n4, assign27660_e29568_d_n6, assign27660_e29568_d_n7, assign27660_e29568_d_n8, assign27660_e29568_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27660_e29565: f64 = (1.0 + locals.var_psce1edge_i);
        let assign27660_e29566: f64 = (1.0 / assign27660_e29565);
        (assign27660_e29566, (-(locals.var_psce1edge_i_dn4 / (assign27660_e29565 * assign27660_e29565))), (-(locals.var_psce1edge_i_dn6 / (assign27660_e29565 * assign27660_e29565))), (-(locals.var_psce1edge_i_dn7 / (assign27660_e29565 * assign27660_e29565))), (-(locals.var_psce1edge_i_dn8 / (assign27660_e29565 * assign27660_e29565))), (-(locals.var_psce1edge_i_dn9 / (assign27660_e29565 * assign27660_e29565))),)
    } else {
        (locals.var_sce1_edge, locals.var_sce1_edge_dn4, locals.var_sce1_edge_dn6, locals.var_sce1_edge_dn7, locals.var_sce1_edge_dn8, locals.var_sce1_edge_dn9,)
    }
};
        locals.var_sce1_edge = assign27660_e29568;
        locals.var_sce1_edge_dn4 = assign27660_e29568_d_n4;
        locals.var_sce1_edge_dn6 = assign27660_e29568_d_n6;
        locals.var_sce1_edge_dn7 = assign27660_e29568_d_n7;
        locals.var_sce1_edge_dn8 = assign27660_e29568_d_n8;
        locals.var_sce1_edge_dn9 = assign27660_e29568_d_n9;

        let (assign27670_e29576, assign27670_e29576_d_n4, assign27670_e29576_d_n6, assign27670_e29576_d_n7, assign27670_e29576_d_n8, assign27670_e29576_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27670_e29573: f64 = (1.0 + locals.var_psce2edge_i);
        let assign27670_e29574: f64 = (1.0 / assign27670_e29573);
        (assign27670_e29574, (-(locals.var_psce2edge_i_dn4 / (assign27670_e29573 * assign27670_e29573))), (-(locals.var_psce2edge_i_dn6 / (assign27670_e29573 * assign27670_e29573))), (-(locals.var_psce2edge_i_dn7 / (assign27670_e29573 * assign27670_e29573))), (-(locals.var_psce2edge_i_dn8 / (assign27670_e29573 * assign27670_e29573))), (-(locals.var_psce2edge_i_dn9 / (assign27670_e29573 * assign27670_e29573))),)
    } else {
        (locals.var_sce2_edge, locals.var_sce2_edge_dn4, locals.var_sce2_edge_dn6, locals.var_sce2_edge_dn7, locals.var_sce2_edge_dn8, locals.var_sce2_edge_dn9,)
    }
};
        locals.var_sce2_edge = assign27670_e29576;
        locals.var_sce2_edge_dn4 = assign27670_e29576_d_n4;
        locals.var_sce2_edge_dn6 = assign27670_e29576_d_n6;
        locals.var_sce2_edge_dn7 = assign27670_e29576_d_n7;
        locals.var_sce2_edge_dn8 = assign27670_e29576_d_n8;
        locals.var_sce2_edge_dn9 = assign27670_e29576_d_n9;

        let (assign27680_e29582, assign27680_e29582_d_n4, assign27680_e29582_d_n6, assign27680_e29582_d_n7, assign27680_e29582_d_n8, assign27680_e29582_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27680_e29580: f64 = (locals.var_cfdedge_i * locals.var_inv_phit_edge);
        (assign27680_e29580, (locals.var_cfdedge_i * locals.var_inv_phit_edge_dn4), (locals.var_cfdedge_i * locals.var_inv_phit_edge_dn6), (locals.var_cfdedge_i * locals.var_inv_phit_edge_dn7), (locals.var_cfdedge_i * locals.var_inv_phit_edge_dn8), (locals.var_cfdedge_i * locals.var_inv_phit_edge_dn9),)
    } else {
        (locals.var_xd0_edge, locals.var_xd0_edge_dn4, locals.var_xd0_edge_dn6, locals.var_xd0_edge_dn7, locals.var_xd0_edge_dn8, locals.var_xd0_edge_dn9,)
    }
};
        locals.var_xd0_edge = assign27680_e29582;
        locals.var_xd0_edge_dn4 = assign27680_e29582_d_n4;
        locals.var_xd0_edge_dn6 = assign27680_e29582_d_n6;
        locals.var_xd0_edge_dn7 = assign27680_e29582_d_n7;
        locals.var_xd0_edge_dn8 = assign27680_e29582_d_n8;
        locals.var_xd0_edge_dn9 = assign27680_e29582_d_n9;

        let (assign27690_e29597, assign27690_e29597_d_n4, assign27690_e29597_d_n6, assign27690_e29597_d_n7, assign27690_e29597_d_n8, assign27690_e29597_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27690_e29586: f64 = (2.0 * locals.var_xd0_edge);
        let assign27690_e29590: f64 = (locals.var_xdsx_edge / locals.var_xd0_edge);
        let assign27690_e29591: f64 = (1.0 + assign27690_e29590);
        let assign27690_e29592: f64 = (assign27690_e29591).sqrt();
        let assign27690_e29594: f64 = (assign27690_e29592 - 1.0);
        let assign27690_e29595: f64 = (assign27690_e29586 * assign27690_e29594);
        (assign27690_e29595, (((2.0 * locals.var_xd0_edge_dn4) * assign27690_e29594) + (assign27690_e29586 * ((((locals.var_xdsx_edge_dn4 * locals.var_xd0_edge) - (locals.var_xdsx_edge * locals.var_xd0_edge_dn4)) / (locals.var_xd0_edge * locals.var_xd0_edge)) / (2.0 * assign27690_e29592)))), (((2.0 * locals.var_xd0_edge_dn6) * assign27690_e29594) + (assign27690_e29586 * ((((locals.var_xdsx_edge_dn6 * locals.var_xd0_edge) - (locals.var_xdsx_edge * locals.var_xd0_edge_dn6)) / (locals.var_xd0_edge * locals.var_xd0_edge)) / (2.0 * assign27690_e29592)))), (((2.0 * locals.var_xd0_edge_dn7) * assign27690_e29594) + (assign27690_e29586 * ((((locals.var_xdsx_edge_dn7 * locals.var_xd0_edge) - (locals.var_xdsx_edge * locals.var_xd0_edge_dn7)) / (locals.var_xd0_edge * locals.var_xd0_edge)) / (2.0 * assign27690_e29592)))), (((2.0 * locals.var_xd0_edge_dn8) * assign27690_e29594) + (assign27690_e29586 * ((((locals.var_xdsx_edge_dn8 * locals.var_xd0_edge) - (locals.var_xdsx_edge * locals.var_xd0_edge_dn8)) / (locals.var_xd0_edge * locals.var_xd0_edge)) / (2.0 * assign27690_e29592)))), (((2.0 * locals.var_xd0_edge_dn9) * assign27690_e29594) + (assign27690_e29586 * ((((locals.var_xdsx_edge_dn9 * locals.var_xd0_edge) - (locals.var_xdsx_edge * locals.var_xd0_edge_dn9)) / (locals.var_xd0_edge * locals.var_xd0_edge)) / (2.0 * assign27690_e29592)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign27690_e29597;
        locals.var_temp_dn4 = assign27690_e29597_d_n4;
        locals.var_temp_dn6 = assign27690_e29597_d_n6;
        locals.var_temp_dn7 = assign27690_e29597_d_n7;
        locals.var_temp_dn8 = assign27690_e29597_d_n8;
        locals.var_temp_dn9 = assign27690_e29597_d_n9;

        let (assign27700_e29603, assign27700_e29603_d_n4, assign27700_e29603_d_n6, assign27700_e29603_d_n7, assign27700_e29603_d_n8, assign27700_e29603_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27700_e29601: f64 = (locals.var_cf1edge_i * locals.var_temp);
        (assign27700_e29601, ((locals.var_cf1edge_i_dn4 * locals.var_temp) + (locals.var_cf1edge_i * locals.var_temp_dn4)), ((locals.var_cf1edge_i_dn6 * locals.var_temp) + (locals.var_cf1edge_i * locals.var_temp_dn6)), ((locals.var_cf1edge_i_dn7 * locals.var_temp) + (locals.var_cf1edge_i * locals.var_temp_dn7)), ((locals.var_cf1edge_i_dn8 * locals.var_temp) + (locals.var_cf1edge_i * locals.var_temp_dn8)), ((locals.var_cf1edge_i_dn9 * locals.var_temp) + (locals.var_cf1edge_i * locals.var_temp_dn9)),)
    } else {
        (locals.var_dxg1_dibl_edge, locals.var_dxg1_dibl_edge_dn4, locals.var_dxg1_dibl_edge_dn6, locals.var_dxg1_dibl_edge_dn7, locals.var_dxg1_dibl_edge_dn8, locals.var_dxg1_dibl_edge_dn9,)
    }
};
        locals.var_dxg1_dibl_edge = assign27700_e29603;
        locals.var_dxg1_dibl_edge_dn4 = assign27700_e29603_d_n4;
        locals.var_dxg1_dibl_edge_dn6 = assign27700_e29603_d_n6;
        locals.var_dxg1_dibl_edge_dn7 = assign27700_e29603_d_n7;
        locals.var_dxg1_dibl_edge_dn8 = assign27700_e29603_d_n8;
        locals.var_dxg1_dibl_edge_dn9 = assign27700_e29603_d_n9;

        let (assign27710_e29609, assign27710_e29609_d_n4, assign27710_e29609_d_n6, assign27710_e29609_d_n7, assign27710_e29609_d_n8, assign27710_e29609_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27710_e29607: f64 = (locals.var_cf2edge_i * locals.var_temp);
        (assign27710_e29607, ((locals.var_cf2edge_i_dn4 * locals.var_temp) + (locals.var_cf2edge_i * locals.var_temp_dn4)), ((locals.var_cf2edge_i_dn6 * locals.var_temp) + (locals.var_cf2edge_i * locals.var_temp_dn6)), ((locals.var_cf2edge_i_dn7 * locals.var_temp) + (locals.var_cf2edge_i * locals.var_temp_dn7)), ((locals.var_cf2edge_i_dn8 * locals.var_temp) + (locals.var_cf2edge_i * locals.var_temp_dn8)), ((locals.var_cf2edge_i_dn9 * locals.var_temp) + (locals.var_cf2edge_i * locals.var_temp_dn9)),)
    } else {
        (locals.var_dxg2_dibl_edge, locals.var_dxg2_dibl_edge_dn4, locals.var_dxg2_dibl_edge_dn6, locals.var_dxg2_dibl_edge_dn7, locals.var_dxg2_dibl_edge_dn8, locals.var_dxg2_dibl_edge_dn9,)
    }
};
        locals.var_dxg2_dibl_edge = assign27710_e29609;
        locals.var_dxg2_dibl_edge_dn4 = assign27710_e29609_d_n4;
        locals.var_dxg2_dibl_edge_dn6 = assign27710_e29609_d_n6;
        locals.var_dxg2_dibl_edge_dn7 = assign27710_e29609_d_n7;
        locals.var_dxg2_dibl_edge_dn8 = assign27710_e29609_d_n8;
        locals.var_dxg2_dibl_edge_dn9 = assign27710_e29609_d_n9;

        let (assign27720_e29619, assign27720_e29619_d_n4, assign27720_e29619_d_n6, assign27720_e29619_d_n7, assign27720_e29619_d_n8, assign27720_e29619_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27720_e29613: f64 = (locals.var_xg10_edge + locals.var_dxg1_dibl_edge);
        let assign27720_e29615: f64 = (assign27720_e29613 * locals.var_sce1_edge);
        let assign27720_e29617: f64 = (assign27720_e29615 + locals.var_dxdsx_edge);
        (assign27720_e29617, ((((locals.var_xg10_edge_dn4 + locals.var_dxg1_dibl_edge_dn4) * locals.var_sce1_edge) + (assign27720_e29613 * locals.var_sce1_edge_dn4)) + locals.var_dxdsx_edge_dn4), ((((locals.var_xg10_edge_dn6 + locals.var_dxg1_dibl_edge_dn6) * locals.var_sce1_edge) + (assign27720_e29613 * locals.var_sce1_edge_dn6)) + locals.var_dxdsx_edge_dn6), ((((locals.var_xg10_edge_dn7 + locals.var_dxg1_dibl_edge_dn7) * locals.var_sce1_edge) + (assign27720_e29613 * locals.var_sce1_edge_dn7)) + locals.var_dxdsx_edge_dn7), ((((locals.var_xg10_edge_dn8 + locals.var_dxg1_dibl_edge_dn8) * locals.var_sce1_edge) + (assign27720_e29613 * locals.var_sce1_edge_dn8)) + locals.var_dxdsx_edge_dn8), ((((locals.var_xg10_edge_dn9 + locals.var_dxg1_dibl_edge_dn9) * locals.var_sce1_edge) + (assign27720_e29613 * locals.var_sce1_edge_dn9)) + locals.var_dxdsx_edge_dn9),)
    } else {
        (locals.var_xg1_edge, locals.var_xg1_edge_dn4, locals.var_xg1_edge_dn6, locals.var_xg1_edge_dn7, locals.var_xg1_edge_dn8, locals.var_xg1_edge_dn9,)
    }
};
        locals.var_xg1_edge = assign27720_e29619;
        locals.var_xg1_edge_dn4 = assign27720_e29619_d_n4;
        locals.var_xg1_edge_dn6 = assign27720_e29619_d_n6;
        locals.var_xg1_edge_dn7 = assign27720_e29619_d_n7;
        locals.var_xg1_edge_dn8 = assign27720_e29619_d_n8;
        locals.var_xg1_edge_dn9 = assign27720_e29619_d_n9;

        let (assign27730_e29629, assign27730_e29629_d_n4, assign27730_e29629_d_n6, assign27730_e29629_d_n7, assign27730_e29629_d_n8, assign27730_e29629_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27730_e29623: f64 = (locals.var_xg20_edge + locals.var_dxg2_dibl_edge);
        let assign27730_e29625: f64 = (assign27730_e29623 * locals.var_sce2_edge);
        let assign27730_e29627: f64 = (assign27730_e29625 + locals.var_dxdsx_edge);
        (assign27730_e29627, ((((locals.var_xg20_edge_dn4 + locals.var_dxg2_dibl_edge_dn4) * locals.var_sce2_edge) + (assign27730_e29623 * locals.var_sce2_edge_dn4)) + locals.var_dxdsx_edge_dn4), ((((locals.var_xg20_edge_dn6 + locals.var_dxg2_dibl_edge_dn6) * locals.var_sce2_edge) + (assign27730_e29623 * locals.var_sce2_edge_dn6)) + locals.var_dxdsx_edge_dn6), ((((locals.var_xg20_edge_dn7 + locals.var_dxg2_dibl_edge_dn7) * locals.var_sce2_edge) + (assign27730_e29623 * locals.var_sce2_edge_dn7)) + locals.var_dxdsx_edge_dn7), ((((locals.var_xg20_edge_dn8 + locals.var_dxg2_dibl_edge_dn8) * locals.var_sce2_edge) + (assign27730_e29623 * locals.var_sce2_edge_dn8)) + locals.var_dxdsx_edge_dn8), ((((locals.var_xg20_edge_dn9 + locals.var_dxg2_dibl_edge_dn9) * locals.var_sce2_edge) + (assign27730_e29623 * locals.var_sce2_edge_dn9)) + locals.var_dxdsx_edge_dn9),)
    } else {
        (locals.var_xg2_edge, locals.var_xg2_edge_dn4, locals.var_xg2_edge_dn6, locals.var_xg2_edge_dn7, locals.var_xg2_edge_dn8, locals.var_xg2_edge_dn9,)
    }
};
        locals.var_xg2_edge = assign27730_e29629;
        locals.var_xg2_edge_dn4 = assign27730_e29629_d_n4;
        locals.var_xg2_edge_dn6 = assign27730_e29629_d_n6;
        locals.var_xg2_edge_dn7 = assign27730_e29629_d_n7;
        locals.var_xg2_edge_dn8 = assign27730_e29629_d_n8;
        locals.var_xg2_edge_dn9 = assign27730_e29629_d_n9;

    }

    pub(super) fn stamp_transient_block_70(
        locals: &mut StampLocals,
    ) {
        let (assign27740_e29666, assign27740_e29666_d_n4, assign27740_e29666_d_n6, assign27740_e29666_d_n7, assign27740_e29666_d_n8, assign27740_e29666_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27740_e29636: f64 = (locals.var_xg1_edge - locals.var_xg2_edge);
        let assign27740_e29637: f64 = (locals.var_cic1edge_i * assign27740_e29636);
        let assign27740_e29638: f64 = (locals.var_xg2_edge + assign27740_e29637);
        let assign27740_e29640: f64 = (assign27740_e29638 + locals.var_xsatmax);
        let assign27740_e29645: f64 = (locals.var_xg1_edge - locals.var_xg2_edge);
        let assign27740_e29646: f64 = (locals.var_cic1edge_i * assign27740_e29645);
        let assign27740_e29647: f64 = (locals.var_xg2_edge + assign27740_e29646);
        let assign27740_e29649: f64 = (assign27740_e29647 - locals.var_xsatmax);
        let assign27740_e29654: f64 = (locals.var_xg1_edge - locals.var_xg2_edge);
        let assign27740_e29655: f64 = (locals.var_cic1edge_i * assign27740_e29654);
        let assign27740_e29656: f64 = (locals.var_xg2_edge + assign27740_e29655);
        let assign27740_e29658: f64 = (assign27740_e29656 - locals.var_xsatmax);
        let assign27740_e29659: f64 = (assign27740_e29649 * assign27740_e29658);
        let assign27740_e29661: f64 = (assign27740_e29659 + 0.01);
        let assign27740_e29662: f64 = (assign27740_e29661).sqrt();
        let assign27740_e29663: f64 = (assign27740_e29640 - assign27740_e29662);
        let assign27740_e29664: f64 = (0.5 * assign27740_e29663);
        (assign27740_e29664, (0.5 * (((locals.var_xg2_edge_dn4 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn4 - locals.var_xg2_edge_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg2_edge_dn4 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn4 - locals.var_xg2_edge_dn4))) - locals.var_xsatmax_dn4) * assign27740_e29658) + (assign27740_e29649 * ((locals.var_xg2_edge_dn4 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn4 - locals.var_xg2_edge_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign27740_e29662)))), (0.5 * (((locals.var_xg2_edge_dn6 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn6 - locals.var_xg2_edge_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg2_edge_dn6 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn6 - locals.var_xg2_edge_dn6))) - locals.var_xsatmax_dn6) * assign27740_e29658) + (assign27740_e29649 * ((locals.var_xg2_edge_dn6 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn6 - locals.var_xg2_edge_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign27740_e29662)))), (0.5 * (((locals.var_xg2_edge_dn7 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn7 - locals.var_xg2_edge_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg2_edge_dn7 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn7 - locals.var_xg2_edge_dn7))) - locals.var_xsatmax_dn7) * assign27740_e29658) + (assign27740_e29649 * ((locals.var_xg2_edge_dn7 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn7 - locals.var_xg2_edge_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign27740_e29662)))), (0.5 * (((locals.var_xg2_edge_dn8 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn8 - locals.var_xg2_edge_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg2_edge_dn8 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn8 - locals.var_xg2_edge_dn8))) - locals.var_xsatmax_dn8) * assign27740_e29658) + (assign27740_e29649 * ((locals.var_xg2_edge_dn8 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn8 - locals.var_xg2_edge_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign27740_e29662)))), (0.5 * (((locals.var_xg2_edge_dn9 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn9 - locals.var_xg2_edge_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg2_edge_dn9 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn9 - locals.var_xg2_edge_dn9))) - locals.var_xsatmax_dn9) * assign27740_e29658) + (assign27740_e29649 * ((locals.var_xg2_edge_dn9 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn9 - locals.var_xg2_edge_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign27740_e29662)))),)
    } else {
        (locals.var_xg1x_edge, locals.var_xg1x_edge_dn4, locals.var_xg1x_edge_dn6, locals.var_xg1x_edge_dn7, locals.var_xg1x_edge_dn8, locals.var_xg1x_edge_dn9,)
    }
};
        locals.var_xg1x_edge = assign27740_e29666;
        locals.var_xg1x_edge_dn4 = assign27740_e29666_d_n4;
        locals.var_xg1x_edge_dn6 = assign27740_e29666_d_n6;
        locals.var_xg1x_edge_dn7 = assign27740_e29666_d_n7;
        locals.var_xg1x_edge_dn8 = assign27740_e29666_d_n8;
        locals.var_xg1x_edge_dn9 = assign27740_e29666_d_n9;

        let (assign27750_e29703, assign27750_e29703_d_n4, assign27750_e29703_d_n6, assign27750_e29703_d_n7, assign27750_e29703_d_n8, assign27750_e29703_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27750_e29673: f64 = (locals.var_xg2_edge - locals.var_xg1_edge);
        let assign27750_e29674: f64 = (locals.var_cic2edge_i * assign27750_e29673);
        let assign27750_e29675: f64 = (locals.var_xg1_edge + assign27750_e29674);
        let assign27750_e29677: f64 = (assign27750_e29675 + locals.var_xsatmax);
        let assign27750_e29682: f64 = (locals.var_xg2_edge - locals.var_xg1_edge);
        let assign27750_e29683: f64 = (locals.var_cic2edge_i * assign27750_e29682);
        let assign27750_e29684: f64 = (locals.var_xg1_edge + assign27750_e29683);
        let assign27750_e29686: f64 = (assign27750_e29684 - locals.var_xsatmax);
        let assign27750_e29691: f64 = (locals.var_xg2_edge - locals.var_xg1_edge);
        let assign27750_e29692: f64 = (locals.var_cic2edge_i * assign27750_e29691);
        let assign27750_e29693: f64 = (locals.var_xg1_edge + assign27750_e29692);
        let assign27750_e29695: f64 = (assign27750_e29693 - locals.var_xsatmax);
        let assign27750_e29696: f64 = (assign27750_e29686 * assign27750_e29695);
        let assign27750_e29698: f64 = (assign27750_e29696 + 0.01);
        let assign27750_e29699: f64 = (assign27750_e29698).sqrt();
        let assign27750_e29700: f64 = (assign27750_e29677 - assign27750_e29699);
        let assign27750_e29701: f64 = (0.5 * assign27750_e29700);
        (assign27750_e29701, (0.5 * (((locals.var_xg1_edge_dn4 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn4 - locals.var_xg1_edge_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg1_edge_dn4 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn4 - locals.var_xg1_edge_dn4))) - locals.var_xsatmax_dn4) * assign27750_e29695) + (assign27750_e29686 * ((locals.var_xg1_edge_dn4 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn4 - locals.var_xg1_edge_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign27750_e29699)))), (0.5 * (((locals.var_xg1_edge_dn6 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn6 - locals.var_xg1_edge_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg1_edge_dn6 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn6 - locals.var_xg1_edge_dn6))) - locals.var_xsatmax_dn6) * assign27750_e29695) + (assign27750_e29686 * ((locals.var_xg1_edge_dn6 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn6 - locals.var_xg1_edge_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign27750_e29699)))), (0.5 * (((locals.var_xg1_edge_dn7 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn7 - locals.var_xg1_edge_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg1_edge_dn7 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn7 - locals.var_xg1_edge_dn7))) - locals.var_xsatmax_dn7) * assign27750_e29695) + (assign27750_e29686 * ((locals.var_xg1_edge_dn7 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn7 - locals.var_xg1_edge_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign27750_e29699)))), (0.5 * (((locals.var_xg1_edge_dn8 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn8 - locals.var_xg1_edge_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg1_edge_dn8 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn8 - locals.var_xg1_edge_dn8))) - locals.var_xsatmax_dn8) * assign27750_e29695) + (assign27750_e29686 * ((locals.var_xg1_edge_dn8 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn8 - locals.var_xg1_edge_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign27750_e29699)))), (0.5 * (((locals.var_xg1_edge_dn9 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn9 - locals.var_xg1_edge_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg1_edge_dn9 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn9 - locals.var_xg1_edge_dn9))) - locals.var_xsatmax_dn9) * assign27750_e29695) + (assign27750_e29686 * ((locals.var_xg1_edge_dn9 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn9 - locals.var_xg1_edge_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign27750_e29699)))),)
    } else {
        (locals.var_xg2x_edge, locals.var_xg2x_edge_dn4, locals.var_xg2x_edge_dn6, locals.var_xg2x_edge_dn7, locals.var_xg2x_edge_dn8, locals.var_xg2x_edge_dn9,)
    }
};
        locals.var_xg2x_edge = assign27750_e29703;
        locals.var_xg2x_edge_dn4 = assign27750_e29703_d_n4;
        locals.var_xg2x_edge_dn6 = assign27750_e29703_d_n6;
        locals.var_xg2x_edge_dn7 = assign27750_e29703_d_n7;
        locals.var_xg2x_edge_dn8 = assign27750_e29703_d_n8;
        locals.var_xg2x_edge_dn9 = assign27750_e29703_d_n9;

        let (assign27760_e29709, assign27760_e29709_d_n4, assign27760_e29709_d_n6, assign27760_e29709_d_n7, assign27760_e29709_d_n8, assign27760_e29709_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27760_e29707: f64 = (locals.var_k1_1d / locals.var_sce1_edge);
        (assign27760_e29707, (-((locals.var_k1_1d * locals.var_sce1_edge_dn4) / (locals.var_sce1_edge * locals.var_sce1_edge))), (-((locals.var_k1_1d * locals.var_sce1_edge_dn6) / (locals.var_sce1_edge * locals.var_sce1_edge))), (-((locals.var_k1_1d * locals.var_sce1_edge_dn7) / (locals.var_sce1_edge * locals.var_sce1_edge))), (-((locals.var_k1_1d * locals.var_sce1_edge_dn8) / (locals.var_sce1_edge * locals.var_sce1_edge))), (-((locals.var_k1_1d * locals.var_sce1_edge_dn9) / (locals.var_sce1_edge * locals.var_sce1_edge))),)
    } else {
        (locals.var_k1_edge, locals.var_k1_edge_dn4, locals.var_k1_edge_dn6, locals.var_k1_edge_dn7, locals.var_k1_edge_dn8, locals.var_k1_edge_dn9,)
    }
};
        locals.var_k1_edge = assign27760_e29709;
        locals.var_k1_edge_dn4 = assign27760_e29709_d_n4;
        locals.var_k1_edge_dn6 = assign27760_e29709_d_n6;
        locals.var_k1_edge_dn7 = assign27760_e29709_d_n7;
        locals.var_k1_edge_dn8 = assign27760_e29709_d_n8;
        locals.var_k1_edge_dn9 = assign27760_e29709_d_n9;

        let (assign27770_e29715, assign27770_e29715_d_n4, assign27770_e29715_d_n6, assign27770_e29715_d_n7, assign27770_e29715_d_n8, assign27770_e29715_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27770_e29713: f64 = (locals.var_k2_1d / locals.var_sce2_edge);
        (assign27770_e29713, (-((locals.var_k2_1d * locals.var_sce2_edge_dn4) / (locals.var_sce2_edge * locals.var_sce2_edge))), (-((locals.var_k2_1d * locals.var_sce2_edge_dn6) / (locals.var_sce2_edge * locals.var_sce2_edge))), (-((locals.var_k2_1d * locals.var_sce2_edge_dn7) / (locals.var_sce2_edge * locals.var_sce2_edge))), (-((locals.var_k2_1d * locals.var_sce2_edge_dn8) / (locals.var_sce2_edge * locals.var_sce2_edge))), (-((locals.var_k2_1d * locals.var_sce2_edge_dn9) / (locals.var_sce2_edge * locals.var_sce2_edge))),)
    } else {
        (locals.var_k2_edge, locals.var_k2_edge_dn4, locals.var_k2_edge_dn6, locals.var_k2_edge_dn7, locals.var_k2_edge_dn8, locals.var_k2_edge_dn9,)
    }
};
        locals.var_k2_edge = assign27770_e29715;
        locals.var_k2_edge_dn4 = assign27770_e29715_d_n4;
        locals.var_k2_edge_dn6 = assign27770_e29715_d_n6;
        locals.var_k2_edge_dn7 = assign27770_e29715_d_n7;
        locals.var_k2_edge_dn8 = assign27770_e29715_d_n8;
        locals.var_k2_edge_dn9 = assign27770_e29715_d_n9;

        let (assign27780_e29721, assign27780_e29721_d_n4, assign27780_e29721_d_n6, assign27780_e29721_d_n7, assign27780_e29721_d_n8, assign27780_e29721_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27780_e29719: f64 = (1.0 / locals.var_k1_edge);
        (assign27780_e29719, (-(locals.var_k1_edge_dn4 / (locals.var_k1_edge * locals.var_k1_edge))), (-(locals.var_k1_edge_dn6 / (locals.var_k1_edge * locals.var_k1_edge))), (-(locals.var_k1_edge_dn7 / (locals.var_k1_edge * locals.var_k1_edge))), (-(locals.var_k1_edge_dn8 / (locals.var_k1_edge * locals.var_k1_edge))), (-(locals.var_k1_edge_dn9 / (locals.var_k1_edge * locals.var_k1_edge))),)
    } else {
        (locals.var_inv_k1_edge, locals.var_inv_k1_edge_dn4, locals.var_inv_k1_edge_dn6, locals.var_inv_k1_edge_dn7, locals.var_inv_k1_edge_dn8, locals.var_inv_k1_edge_dn9,)
    }
};
        locals.var_inv_k1_edge = assign27780_e29721;
        locals.var_inv_k1_edge_dn4 = assign27780_e29721_d_n4;
        locals.var_inv_k1_edge_dn6 = assign27780_e29721_d_n6;
        locals.var_inv_k1_edge_dn7 = assign27780_e29721_d_n7;
        locals.var_inv_k1_edge_dn8 = assign27780_e29721_d_n8;
        locals.var_inv_k1_edge_dn9 = assign27780_e29721_d_n9;

        let (assign27790_e29727, assign27790_e29727_d_n4, assign27790_e29727_d_n6, assign27790_e29727_d_n7, assign27790_e29727_d_n8, assign27790_e29727_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27790_e29725: f64 = (1.0 / locals.var_k2_edge);
        (assign27790_e29725, (-(locals.var_k2_edge_dn4 / (locals.var_k2_edge * locals.var_k2_edge))), (-(locals.var_k2_edge_dn6 / (locals.var_k2_edge * locals.var_k2_edge))), (-(locals.var_k2_edge_dn7 / (locals.var_k2_edge * locals.var_k2_edge))), (-(locals.var_k2_edge_dn8 / (locals.var_k2_edge * locals.var_k2_edge))), (-(locals.var_k2_edge_dn9 / (locals.var_k2_edge * locals.var_k2_edge))),)
    } else {
        (locals.var_inv_k2_edge, locals.var_inv_k2_edge_dn4, locals.var_inv_k2_edge_dn6, locals.var_inv_k2_edge_dn7, locals.var_inv_k2_edge_dn8, locals.var_inv_k2_edge_dn9,)
    }
};
        locals.var_inv_k2_edge = assign27790_e29727;
        locals.var_inv_k2_edge_dn4 = assign27790_e29727_d_n4;
        locals.var_inv_k2_edge_dn6 = assign27790_e29727_d_n6;
        locals.var_inv_k2_edge_dn7 = assign27790_e29727_d_n7;
        locals.var_inv_k2_edge_dn8 = assign27790_e29727_d_n8;
        locals.var_inv_k2_edge_dn9 = assign27790_e29727_d_n9;

        let (assign27800_e29737, assign27800_e29737_d_n4, assign27800_e29737_d_n6, assign27800_e29737_d_n7, assign27800_e29737_d_n8, assign27800_e29737_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27800_e29732: f64 = (1.0 + locals.var_inv_k1_edge);
        let assign27800_e29734: f64 = (assign27800_e29732 + locals.var_inv_k2_edge);
        let assign27800_e29735: f64 = (1.0 / assign27800_e29734);
        (assign27800_e29735, (-((locals.var_inv_k1_edge_dn4 + locals.var_inv_k2_edge_dn4) / (assign27800_e29734 * assign27800_e29734))), (-((locals.var_inv_k1_edge_dn6 + locals.var_inv_k2_edge_dn6) / (assign27800_e29734 * assign27800_e29734))), (-((locals.var_inv_k1_edge_dn7 + locals.var_inv_k2_edge_dn7) / (assign27800_e29734 * assign27800_e29734))), (-((locals.var_inv_k1_edge_dn8 + locals.var_inv_k2_edge_dn8) / (assign27800_e29734 * assign27800_e29734))), (-((locals.var_inv_k1_edge_dn9 + locals.var_inv_k2_edge_dn9) / (assign27800_e29734 * assign27800_e29734))),)
    } else {
        (locals.var_keq_edge, locals.var_keq_edge_dn4, locals.var_keq_edge_dn6, locals.var_keq_edge_dn7, locals.var_keq_edge_dn8, locals.var_keq_edge_dn9,)
    }
};
        locals.var_keq_edge = assign27800_e29737;
        locals.var_keq_edge_dn4 = assign27800_e29737_d_n4;
        locals.var_keq_edge_dn6 = assign27800_e29737_d_n6;
        locals.var_keq_edge_dn7 = assign27800_e29737_d_n7;
        locals.var_keq_edge_dn8 = assign27800_e29737_d_n8;
        locals.var_keq_edge_dn9 = assign27800_e29737_d_n9;

        let (assign27810_e29745, assign27810_e29745_d_n4, assign27810_e29745_d_n6, assign27810_e29745_d_n7, assign27810_e29745_d_n8, assign27810_e29745_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27810_e29742: f64 = (locals.var_csiprime_dc * locals.var_csiprime_dc);
        let assign27810_e29743: f64 = (locals.var_a0_csisq_edge / assign27810_e29742);
        (assign27810_e29743, (((locals.var_a0_csisq_edge_dn4 * assign27810_e29742) - (locals.var_a0_csisq_edge * ((locals.var_csiprime_dc_dn4 * locals.var_csiprime_dc) + (locals.var_csiprime_dc * locals.var_csiprime_dc_dn4)))) / (assign27810_e29742 * assign27810_e29742)), (((locals.var_a0_csisq_edge_dn6 * assign27810_e29742) - (locals.var_a0_csisq_edge * ((locals.var_csiprime_dc_dn6 * locals.var_csiprime_dc) + (locals.var_csiprime_dc * locals.var_csiprime_dc_dn6)))) / (assign27810_e29742 * assign27810_e29742)), (((locals.var_a0_csisq_edge_dn7 * assign27810_e29742) - (locals.var_a0_csisq_edge * ((locals.var_csiprime_dc_dn7 * locals.var_csiprime_dc) + (locals.var_csiprime_dc * locals.var_csiprime_dc_dn7)))) / (assign27810_e29742 * assign27810_e29742)), (((locals.var_a0_csisq_edge_dn8 * assign27810_e29742) - (locals.var_a0_csisq_edge * ((locals.var_csiprime_dc_dn8 * locals.var_csiprime_dc) + (locals.var_csiprime_dc * locals.var_csiprime_dc_dn8)))) / (assign27810_e29742 * assign27810_e29742)), (((locals.var_a0_csisq_edge_dn9 * assign27810_e29742) - (locals.var_a0_csisq_edge * ((locals.var_csiprime_dc_dn9 * locals.var_csiprime_dc) + (locals.var_csiprime_dc * locals.var_csiprime_dc_dn9)))) / (assign27810_e29742 * assign27810_e29742)),)
    } else {
        (locals.var_a0_edge, locals.var_a0_edge_dn4, locals.var_a0_edge_dn6, locals.var_a0_edge_dn7, locals.var_a0_edge_dn8, locals.var_a0_edge_dn9,)
    }
};
        locals.var_a0_edge = assign27810_e29745;
        locals.var_a0_edge_dn4 = assign27810_e29745_d_n4;
        locals.var_a0_edge_dn6 = assign27810_e29745_d_n6;
        locals.var_a0_edge_dn7 = assign27810_e29745_d_n7;
        locals.var_a0_edge_dn8 = assign27810_e29745_d_n8;
        locals.var_a0_edge_dn9 = assign27810_e29745_d_n9;

        let (assign27820_e29753, assign27820_e29753_d_n4, assign27820_e29753_d_n6, assign27820_e29753_d_n7, assign27820_e29753_d_n8, assign27820_e29753_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27820_e29750: f64 = (locals.var_xg1x_edge - locals.var_xg2x_edge);
        let assign27820_e29751: f64 = (locals.var_keq_edge * assign27820_e29750);
        (assign27820_e29751, ((locals.var_keq_edge_dn4 * assign27820_e29750) + (locals.var_keq_edge * (locals.var_xg1x_edge_dn4 - locals.var_xg2x_edge_dn4))), ((locals.var_keq_edge_dn6 * assign27820_e29750) + (locals.var_keq_edge * (locals.var_xg1x_edge_dn6 - locals.var_xg2x_edge_dn6))), ((locals.var_keq_edge_dn7 * assign27820_e29750) + (locals.var_keq_edge * (locals.var_xg1x_edge_dn7 - locals.var_xg2x_edge_dn7))), ((locals.var_keq_edge_dn8 * assign27820_e29750) + (locals.var_keq_edge * (locals.var_xg1x_edge_dn8 - locals.var_xg2x_edge_dn8))), ((locals.var_keq_edge_dn9 * assign27820_e29750) + (locals.var_keq_edge * (locals.var_xg1x_edge_dn9 - locals.var_xg2x_edge_dn9))),)
    } else {
        (locals.var_dx_wi_edge, locals.var_dx_wi_edge_dn4, locals.var_dx_wi_edge_dn6, locals.var_dx_wi_edge_dn7, locals.var_dx_wi_edge_dn8, locals.var_dx_wi_edge_dn9,)
    }
};
        locals.var_dx_wi_edge = assign27820_e29753;
        locals.var_dx_wi_edge_dn4 = assign27820_e29753_d_n4;
        locals.var_dx_wi_edge_dn6 = assign27820_e29753_d_n6;
        locals.var_dx_wi_edge_dn7 = assign27820_e29753_d_n7;
        locals.var_dx_wi_edge_dn8 = assign27820_e29753_d_n8;
        locals.var_dx_wi_edge_dn9 = assign27820_e29753_d_n9;

        let assign27830_e29756: f64 = (locals.var_xg2x_edge - locals.var_xg1x_edge);
        let assign27830_e29757: f64 = (assign27830_e29756).abs();
        let assign27830_e29759: f64 = if assign27830_e29757 <= 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard783 = assign27830_e29759;

        let (assign27840_e29773, assign27840_e29773_d_n4, assign27840_e29773_d_n6, assign27840_e29773_d_n7, assign27840_e29773_d_n8, assign27840_e29773_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 != 0.0)) {
        let assign27840_e29766: f64 = (locals.var_keq_edge * locals.var_inv_k1_edge);
        let assign27840_e29767: f64 = (1.0 - assign27840_e29766);
        let assign27840_e29770: f64 = (locals.var_keq_edge * locals.var_inv_k2_edge);
        let assign27840_e29771: f64 = (assign27840_e29767 - assign27840_e29770);
        (assign27840_e29771, ((-((locals.var_keq_edge_dn4 * locals.var_inv_k1_edge) + (locals.var_keq_edge * locals.var_inv_k1_edge_dn4))) - ((locals.var_keq_edge_dn4 * locals.var_inv_k2_edge) + (locals.var_keq_edge * locals.var_inv_k2_edge_dn4))), ((-((locals.var_keq_edge_dn6 * locals.var_inv_k1_edge) + (locals.var_keq_edge * locals.var_inv_k1_edge_dn6))) - ((locals.var_keq_edge_dn6 * locals.var_inv_k2_edge) + (locals.var_keq_edge * locals.var_inv_k2_edge_dn6))), ((-((locals.var_keq_edge_dn7 * locals.var_inv_k1_edge) + (locals.var_keq_edge * locals.var_inv_k1_edge_dn7))) - ((locals.var_keq_edge_dn7 * locals.var_inv_k2_edge) + (locals.var_keq_edge * locals.var_inv_k2_edge_dn7))), ((-((locals.var_keq_edge_dn8 * locals.var_inv_k1_edge) + (locals.var_keq_edge * locals.var_inv_k1_edge_dn8))) - ((locals.var_keq_edge_dn8 * locals.var_inv_k2_edge) + (locals.var_keq_edge * locals.var_inv_k2_edge_dn8))), ((-((locals.var_keq_edge_dn9 * locals.var_inv_k1_edge) + (locals.var_keq_edge * locals.var_inv_k1_edge_dn9))) - ((locals.var_keq_edge_dn9 * locals.var_inv_k2_edge) + (locals.var_keq_edge * locals.var_inv_k2_edge_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign27840_e29773;
        locals.var_temp1_dn4 = assign27840_e29773_d_n4;
        locals.var_temp1_dn6 = assign27840_e29773_d_n6;
        locals.var_temp1_dn7 = assign27840_e29773_d_n7;
        locals.var_temp1_dn8 = assign27840_e29773_d_n8;
        locals.var_temp1_dn9 = assign27840_e29773_d_n9;

        let (assign27850_e29801, assign27850_e29801_d_n4, assign27850_e29801_d_n6, assign27850_e29801_d_n7, assign27850_e29801_d_n8, assign27850_e29801_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 != 0.0)) {
        let assign27850_e29780: f64 = (0.5 * locals.var_inv_k1_edge);
        let assign27850_e29782: f64 = (assign27850_e29780 * locals.var_keq_edge);
        let assign27850_e29784: f64 = (assign27850_e29782 * locals.var_inv_k1_edge);
        let assign27850_e29785: f64 = (locals.var_inv_k2_edge + assign27850_e29784);
        let assign27850_e29788: f64 = (0.5 * locals.var_inv_k2_edge);
        let assign27850_e29790: f64 = (assign27850_e29788 * locals.var_keq_edge);
        let assign27850_e29792: f64 = (assign27850_e29790 * locals.var_inv_k2_edge);
        let assign27850_e29793: f64 = (assign27850_e29785 - assign27850_e29792);
        let assign27850_e29796: f64 = (0.5 / locals.var_keq_edge);
        let assign27850_e29797: f64 = (assign27850_e29793 - assign27850_e29796);
        let assign27850_e29799: f64 = (assign27850_e29797 * locals.var_dx_wi_edge);
        (assign27850_e29799, (((((locals.var_inv_k2_edge_dn4 + (((((0.5 * locals.var_inv_k1_edge_dn4) * locals.var_keq_edge) + (assign27850_e29780 * locals.var_keq_edge_dn4)) * locals.var_inv_k1_edge) + (assign27850_e29782 * locals.var_inv_k1_edge_dn4))) - (((((0.5 * locals.var_inv_k2_edge_dn4) * locals.var_keq_edge) + (assign27850_e29788 * locals.var_keq_edge_dn4)) * locals.var_inv_k2_edge) + (assign27850_e29790 * locals.var_inv_k2_edge_dn4))) - (-((0.5 * locals.var_keq_edge_dn4) / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27850_e29797 * locals.var_dx_wi_edge_dn4)), (((((locals.var_inv_k2_edge_dn6 + (((((0.5 * locals.var_inv_k1_edge_dn6) * locals.var_keq_edge) + (assign27850_e29780 * locals.var_keq_edge_dn6)) * locals.var_inv_k1_edge) + (assign27850_e29782 * locals.var_inv_k1_edge_dn6))) - (((((0.5 * locals.var_inv_k2_edge_dn6) * locals.var_keq_edge) + (assign27850_e29788 * locals.var_keq_edge_dn6)) * locals.var_inv_k2_edge) + (assign27850_e29790 * locals.var_inv_k2_edge_dn6))) - (-((0.5 * locals.var_keq_edge_dn6) / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27850_e29797 * locals.var_dx_wi_edge_dn6)), (((((locals.var_inv_k2_edge_dn7 + (((((0.5 * locals.var_inv_k1_edge_dn7) * locals.var_keq_edge) + (assign27850_e29780 * locals.var_keq_edge_dn7)) * locals.var_inv_k1_edge) + (assign27850_e29782 * locals.var_inv_k1_edge_dn7))) - (((((0.5 * locals.var_inv_k2_edge_dn7) * locals.var_keq_edge) + (assign27850_e29788 * locals.var_keq_edge_dn7)) * locals.var_inv_k2_edge) + (assign27850_e29790 * locals.var_inv_k2_edge_dn7))) - (-((0.5 * locals.var_keq_edge_dn7) / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27850_e29797 * locals.var_dx_wi_edge_dn7)), (((((locals.var_inv_k2_edge_dn8 + (((((0.5 * locals.var_inv_k1_edge_dn8) * locals.var_keq_edge) + (assign27850_e29780 * locals.var_keq_edge_dn8)) * locals.var_inv_k1_edge) + (assign27850_e29782 * locals.var_inv_k1_edge_dn8))) - (((((0.5 * locals.var_inv_k2_edge_dn8) * locals.var_keq_edge) + (assign27850_e29788 * locals.var_keq_edge_dn8)) * locals.var_inv_k2_edge) + (assign27850_e29790 * locals.var_inv_k2_edge_dn8))) - (-((0.5 * locals.var_keq_edge_dn8) / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27850_e29797 * locals.var_dx_wi_edge_dn8)), (((((locals.var_inv_k2_edge_dn9 + (((((0.5 * locals.var_inv_k1_edge_dn9) * locals.var_keq_edge) + (assign27850_e29780 * locals.var_keq_edge_dn9)) * locals.var_inv_k1_edge) + (assign27850_e29782 * locals.var_inv_k1_edge_dn9))) - (((((0.5 * locals.var_inv_k2_edge_dn9) * locals.var_keq_edge) + (assign27850_e29788 * locals.var_keq_edge_dn9)) * locals.var_inv_k2_edge) + (assign27850_e29790 * locals.var_inv_k2_edge_dn9))) - (-((0.5 * locals.var_keq_edge_dn9) / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27850_e29797 * locals.var_dx_wi_edge_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27850_e29801;
        locals.var_temp2_dn4 = assign27850_e29801_d_n4;
        locals.var_temp2_dn6 = assign27850_e29801_d_n6;
        locals.var_temp2_dn7 = assign27850_e29801_d_n7;
        locals.var_temp2_dn8 = assign27850_e29801_d_n8;
        locals.var_temp2_dn9 = assign27850_e29801_d_n9;

        let (assign27860_e29815, assign27860_e29815_d_n4, assign27860_e29815_d_n6, assign27860_e29815_d_n7, assign27860_e29815_d_n8, assign27860_e29815_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 != 0.0)) {
        let assign27860_e29808: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign27860_e29809: f64 = (0.5 * assign27860_e29808);
        let assign27860_e29811: f64 = (assign27860_e29809 * locals.var_a0_edge);
        let assign27860_e29813: f64 = (assign27860_e29811 / locals.var_keq_edge);
        (assign27860_e29813, ((((((0.5 * (locals.var_temp1_dn4 - locals.var_temp2_dn4)) * locals.var_a0_edge) + (assign27860_e29809 * locals.var_a0_edge_dn4)) * locals.var_keq_edge) - (assign27860_e29811 * locals.var_keq_edge_dn4)) / (locals.var_keq_edge * locals.var_keq_edge)), ((((((0.5 * (locals.var_temp1_dn6 - locals.var_temp2_dn6)) * locals.var_a0_edge) + (assign27860_e29809 * locals.var_a0_edge_dn6)) * locals.var_keq_edge) - (assign27860_e29811 * locals.var_keq_edge_dn6)) / (locals.var_keq_edge * locals.var_keq_edge)), ((((((0.5 * (locals.var_temp1_dn7 - locals.var_temp2_dn7)) * locals.var_a0_edge) + (assign27860_e29809 * locals.var_a0_edge_dn7)) * locals.var_keq_edge) - (assign27860_e29811 * locals.var_keq_edge_dn7)) / (locals.var_keq_edge * locals.var_keq_edge)), ((((((0.5 * (locals.var_temp1_dn8 - locals.var_temp2_dn8)) * locals.var_a0_edge) + (assign27860_e29809 * locals.var_a0_edge_dn8)) * locals.var_keq_edge) - (assign27860_e29811 * locals.var_keq_edge_dn8)) / (locals.var_keq_edge * locals.var_keq_edge)), ((((((0.5 * (locals.var_temp1_dn9 - locals.var_temp2_dn9)) * locals.var_a0_edge) + (assign27860_e29809 * locals.var_a0_edge_dn9)) * locals.var_keq_edge) - (assign27860_e29811 * locals.var_keq_edge_dn9)) / (locals.var_keq_edge * locals.var_keq_edge)),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign27860_e29815;
        locals.var_temp3_dn4 = assign27860_e29815_d_n4;
        locals.var_temp3_dn6 = assign27860_e29815_d_n6;
        locals.var_temp3_dn7 = assign27860_e29815_d_n7;
        locals.var_temp3_dn8 = assign27860_e29815_d_n8;
        locals.var_temp3_dn9 = assign27860_e29815_d_n9;

        let (assign27870_e29826, assign27870_e29826_d_n4, assign27870_e29826_d_n6, assign27870_e29826_d_n7, assign27870_e29826_d_n8, assign27870_e29826_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 == 0.0)) {
        let assign27870_e29821: f64 = (-locals.var_inv_k1_edge);
        let assign27870_e29823: f64 = (assign27870_e29821 * locals.var_dx_wi_edge);
        let assign27870_e29824: f64 = (assign27870_e29823).exp();
        (assign27870_e29824, (assign27870_e29824 * (((-locals.var_inv_k1_edge_dn4) * locals.var_dx_wi_edge) + (assign27870_e29821 * locals.var_dx_wi_edge_dn4))), (assign27870_e29824 * (((-locals.var_inv_k1_edge_dn6) * locals.var_dx_wi_edge) + (assign27870_e29821 * locals.var_dx_wi_edge_dn6))), (assign27870_e29824 * (((-locals.var_inv_k1_edge_dn7) * locals.var_dx_wi_edge) + (assign27870_e29821 * locals.var_dx_wi_edge_dn7))), (assign27870_e29824 * (((-locals.var_inv_k1_edge_dn8) * locals.var_dx_wi_edge) + (assign27870_e29821 * locals.var_dx_wi_edge_dn8))), (assign27870_e29824 * (((-locals.var_inv_k1_edge_dn9) * locals.var_dx_wi_edge) + (assign27870_e29821 * locals.var_dx_wi_edge_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign27870_e29826;
        locals.var_temp1_dn4 = assign27870_e29826_d_n4;
        locals.var_temp1_dn6 = assign27870_e29826_d_n6;
        locals.var_temp1_dn7 = assign27870_e29826_d_n7;
        locals.var_temp1_dn8 = assign27870_e29826_d_n8;
        locals.var_temp1_dn9 = assign27870_e29826_d_n9;

        let (assign27880_e29840, assign27880_e29840_d_n4, assign27880_e29840_d_n6, assign27880_e29840_d_n7, assign27880_e29840_d_n8, assign27880_e29840_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 == 0.0)) {
        let assign27880_e29834: f64 = (1.0 / locals.var_keq_edge);
        let assign27880_e29835: f64 = (locals.var_inv_k2_edge - assign27880_e29834);
        let assign27880_e29837: f64 = (assign27880_e29835 * locals.var_dx_wi_edge);
        let assign27880_e29838: f64 = (assign27880_e29837).exp();
        (assign27880_e29838, (assign27880_e29838 * (((locals.var_inv_k2_edge_dn4 - (-(locals.var_keq_edge_dn4 / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27880_e29835 * locals.var_dx_wi_edge_dn4))), (assign27880_e29838 * (((locals.var_inv_k2_edge_dn6 - (-(locals.var_keq_edge_dn6 / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27880_e29835 * locals.var_dx_wi_edge_dn6))), (assign27880_e29838 * (((locals.var_inv_k2_edge_dn7 - (-(locals.var_keq_edge_dn7 / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27880_e29835 * locals.var_dx_wi_edge_dn7))), (assign27880_e29838 * (((locals.var_inv_k2_edge_dn8 - (-(locals.var_keq_edge_dn8 / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27880_e29835 * locals.var_dx_wi_edge_dn8))), (assign27880_e29838 * (((locals.var_inv_k2_edge_dn9 - (-(locals.var_keq_edge_dn9 / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27880_e29835 * locals.var_dx_wi_edge_dn9))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27880_e29840;
        locals.var_temp2_dn4 = assign27880_e29840_d_n4;
        locals.var_temp2_dn6 = assign27880_e29840_d_n6;
        locals.var_temp2_dn7 = assign27880_e29840_d_n7;
        locals.var_temp2_dn8 = assign27880_e29840_d_n8;
        locals.var_temp2_dn9 = assign27880_e29840_d_n9;

        let (assign27890_e29855, assign27890_e29855_d_n4, assign27890_e29855_d_n6, assign27890_e29855_d_n7, assign27890_e29855_d_n8, assign27890_e29855_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 == 0.0)) {
        let assign27890_e29848: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign27890_e29849: f64 = (locals.var_a0_edge * assign27890_e29848);
        let assign27890_e29852: f64 = (2.0 * locals.var_dx_wi_edge);
        let assign27890_e29853: f64 = (assign27890_e29849 / assign27890_e29852);
        (assign27890_e29853, (((((locals.var_a0_edge_dn4 * assign27890_e29848) + (locals.var_a0_edge * (locals.var_temp1_dn4 - locals.var_temp2_dn4))) * assign27890_e29852) - (assign27890_e29849 * (2.0 * locals.var_dx_wi_edge_dn4))) / (assign27890_e29852 * assign27890_e29852)), (((((locals.var_a0_edge_dn6 * assign27890_e29848) + (locals.var_a0_edge * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) * assign27890_e29852) - (assign27890_e29849 * (2.0 * locals.var_dx_wi_edge_dn6))) / (assign27890_e29852 * assign27890_e29852)), (((((locals.var_a0_edge_dn7 * assign27890_e29848) + (locals.var_a0_edge * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) * assign27890_e29852) - (assign27890_e29849 * (2.0 * locals.var_dx_wi_edge_dn7))) / (assign27890_e29852 * assign27890_e29852)), (((((locals.var_a0_edge_dn8 * assign27890_e29848) + (locals.var_a0_edge * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) * assign27890_e29852) - (assign27890_e29849 * (2.0 * locals.var_dx_wi_edge_dn8))) / (assign27890_e29852 * assign27890_e29852)), (((((locals.var_a0_edge_dn9 * assign27890_e29848) + (locals.var_a0_edge * (locals.var_temp1_dn9 - locals.var_temp2_dn9))) * assign27890_e29852) - (assign27890_e29849 * (2.0 * locals.var_dx_wi_edge_dn9))) / (assign27890_e29852 * assign27890_e29852)),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign27890_e29855;
        locals.var_temp3_dn4 = assign27890_e29855_d_n4;
        locals.var_temp3_dn6 = assign27890_e29855_d_n6;
        locals.var_temp3_dn7 = assign27890_e29855_d_n7;
        locals.var_temp3_dn8 = assign27890_e29855_d_n8;
        locals.var_temp3_dn9 = assign27890_e29855_d_n9;

        let (assign27900_e29859, assign27900_e29859_d_n4, assign27900_e29859_d_n6, assign27900_e29859_d_n7, assign27900_e29859_d_n8, assign27900_e29859_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    } else {
        (locals.var_prefac_qilow_edge, locals.var_prefac_qilow_edge_dn4, locals.var_prefac_qilow_edge_dn6, locals.var_prefac_qilow_edge_dn7, locals.var_prefac_qilow_edge_dn8, locals.var_prefac_qilow_edge_dn9,)
    }
};
        locals.var_prefac_qilow_edge = assign27900_e29859;
        locals.var_prefac_qilow_edge_dn4 = assign27900_e29859_d_n4;
        locals.var_prefac_qilow_edge_dn6 = assign27900_e29859_d_n6;
        locals.var_prefac_qilow_edge_dn7 = assign27900_e29859_d_n7;
        locals.var_prefac_qilow_edge_dn8 = assign27900_e29859_d_n8;
        locals.var_prefac_qilow_edge_dn9 = assign27900_e29859_d_n9;

        let assign27910_e29862: f64 = if locals.var_xg1x_edge < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard784 = assign27910_e29862;

        let (assign27920_e29874, assign27920_e29874_d_n4, assign27920_e29874_d_n6, assign27920_e29874_d_n7, assign27920_e29874_d_n8, assign27920_e29874_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard784 != 0.0)) {
        let assign27920_e29869: f64 = (locals.var_xg1x_edge).exp();
        let assign27920_e29870: f64 = (locals.var_prefac_qilow_edge * assign27920_e29869);
        let assign27920_e29871: f64 = (1.0 + assign27920_e29870);
        let assign27920_e29872: f64 = (assign27920_e29871).ln();
        (assign27920_e29872, (((locals.var_prefac_qilow_edge_dn4 * assign27920_e29869) + (locals.var_prefac_qilow_edge * (assign27920_e29869 * locals.var_xg1x_edge_dn4))) / assign27920_e29871), (((locals.var_prefac_qilow_edge_dn6 * assign27920_e29869) + (locals.var_prefac_qilow_edge * (assign27920_e29869 * locals.var_xg1x_edge_dn6))) / assign27920_e29871), (((locals.var_prefac_qilow_edge_dn7 * assign27920_e29869) + (locals.var_prefac_qilow_edge * (assign27920_e29869 * locals.var_xg1x_edge_dn7))) / assign27920_e29871), (((locals.var_prefac_qilow_edge_dn8 * assign27920_e29869) + (locals.var_prefac_qilow_edge * (assign27920_e29869 * locals.var_xg1x_edge_dn8))) / assign27920_e29871), (((locals.var_prefac_qilow_edge_dn9 * assign27920_e29869) + (locals.var_prefac_qilow_edge * (assign27920_e29869 * locals.var_xg1x_edge_dn9))) / assign27920_e29871),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign27920_e29874;
        locals.var_w_temp_dn4 = assign27920_e29874_d_n4;
        locals.var_w_temp_dn6 = assign27920_e29874_d_n6;
        locals.var_w_temp_dn7 = assign27920_e29874_d_n7;
        locals.var_w_temp_dn8 = assign27920_e29874_d_n8;
        locals.var_w_temp_dn9 = assign27920_e29874_d_n9;

        let (assign27930_e29891, assign27930_e29891_d_n4, assign27930_e29891_d_n6, assign27930_e29891_d_n7, assign27930_e29891_d_n8, assign27930_e29891_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard784 != 0.0)) {
        let assign27930_e29882: f64 = (1.0 + locals.var_w_temp);
        let assign27930_e29883: f64 = (assign27930_e29882).ln();
        let assign27930_e29886: f64 = (2.0 + locals.var_w_temp);
        let assign27930_e29887: f64 = (assign27930_e29883 / assign27930_e29886);
        let assign27930_e29888: f64 = (1.0 - assign27930_e29887);
        let assign27930_e29889: f64 = (locals.var_w_temp * assign27930_e29888);
        (assign27930_e29889, ((locals.var_w_temp_dn4 * assign27930_e29888) + (locals.var_w_temp * (-((((locals.var_w_temp_dn4 / assign27930_e29882) * assign27930_e29886) - (assign27930_e29883 * locals.var_w_temp_dn4)) / (assign27930_e29886 * assign27930_e29886))))), ((locals.var_w_temp_dn6 * assign27930_e29888) + (locals.var_w_temp * (-((((locals.var_w_temp_dn6 / assign27930_e29882) * assign27930_e29886) - (assign27930_e29883 * locals.var_w_temp_dn6)) / (assign27930_e29886 * assign27930_e29886))))), ((locals.var_w_temp_dn7 * assign27930_e29888) + (locals.var_w_temp * (-((((locals.var_w_temp_dn7 / assign27930_e29882) * assign27930_e29886) - (assign27930_e29883 * locals.var_w_temp_dn7)) / (assign27930_e29886 * assign27930_e29886))))), ((locals.var_w_temp_dn8 * assign27930_e29888) + (locals.var_w_temp * (-((((locals.var_w_temp_dn8 / assign27930_e29882) * assign27930_e29886) - (assign27930_e29883 * locals.var_w_temp_dn8)) / (assign27930_e29886 * assign27930_e29886))))), ((locals.var_w_temp_dn9 * assign27930_e29888) + (locals.var_w_temp * (-((((locals.var_w_temp_dn9 / assign27930_e29882) * assign27930_e29886) - (assign27930_e29883 * locals.var_w_temp_dn9)) / (assign27930_e29886 * assign27930_e29886))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign27930_e29891;
        locals.var_temp_dn4 = assign27930_e29891_d_n4;
        locals.var_temp_dn6 = assign27930_e29891_d_n6;
        locals.var_temp_dn7 = assign27930_e29891_d_n7;
        locals.var_temp_dn8 = assign27930_e29891_d_n8;
        locals.var_temp_dn9 = assign27930_e29891_d_n9;

        let assign27940_e29894: f64 = if locals.var_xg1x_edge < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard785 = assign27940_e29894;

        let assign27950_e29897: f64 = (-80.0);
        let assign27950_e29898: f64 = if locals.var_xg1x_edge > assign27950_e29897 { 1.0 } else { 0.0 };
        locals.var_guard786 = assign27950_e29898;

        let (assign27960_e29910, assign27960_e29910_d_n4, assign27960_e29910_d_n6, assign27960_e29910_d_n7, assign27960_e29910_d_n8, assign27960_e29910_d_n9,) = {
    if ((((locals.var_guard782 != 0.0) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign27960_e29908: f64 = (locals.var_xg1x_edge).exp();
        (assign27960_e29908, (assign27960_e29908 * locals.var_xg1x_edge_dn4), (assign27960_e29908 * locals.var_xg1x_edge_dn6), (assign27960_e29908 * locals.var_xg1x_edge_dn7), (assign27960_e29908 * locals.var_xg1x_edge_dn8), (assign27960_e29908 * locals.var_xg1x_edge_dn9),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign27960_e29910;
        locals.var_w_temp_dn4 = assign27960_e29910_d_n4;
        locals.var_w_temp_dn6 = assign27960_e29910_d_n6;
        locals.var_w_temp_dn7 = assign27960_e29910_d_n7;
        locals.var_w_temp_dn8 = assign27960_e29910_d_n8;
        locals.var_w_temp_dn9 = assign27960_e29910_d_n9;

        let (assign27970_e29947, assign27970_e29947_d_n4, assign27970_e29947_d_n6, assign27970_e29947_d_n7, assign27970_e29947_d_n8, assign27970_e29947_d_n9,) = {
    if ((((locals.var_guard782 != 0.0) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_guard786 == 0.0)) {
        let assign27970_e29923: f64 = (-locals.var_xg1x_edge);
        let assign27970_e29925: f64 = (assign27970_e29923 - 80.0);
        let assign27970_e29929: f64 = (-locals.var_xg1x_edge);
        let assign27970_e29931: f64 = (assign27970_e29929 - 80.0);
        let assign27970_e29932: f64 = (0.5 * assign27970_e29931);
        let assign27970_e29935: f64 = (-locals.var_xg1x_edge);
        let assign27970_e29937: f64 = (assign27970_e29935 - 80.0);
        let assign27970_e29939: f64 = (assign27970_e29937 * 0.3333333333333);
        let assign27970_e29940: f64 = (1.0 + assign27970_e29939);
        let assign27970_e29941: f64 = (assign27970_e29932 * assign27970_e29940);
        let assign27970_e29942: f64 = (1.0 + assign27970_e29941);
        let assign27970_e29943: f64 = (assign27970_e29925 * assign27970_e29942);
        let assign27970_e29944: f64 = (1.0 + assign27970_e29943);
        let assign27970_e29945: f64 = (1.80485e-35 / assign27970_e29944);
        (assign27970_e29945, (-((1.80485e-35 * (((-locals.var_xg1x_edge_dn4) * assign27970_e29942) + (assign27970_e29925 * (((0.5 * (-locals.var_xg1x_edge_dn4)) * assign27970_e29940) + (assign27970_e29932 * ((-locals.var_xg1x_edge_dn4) * 0.3333333333333)))))) / (assign27970_e29944 * assign27970_e29944))), (-((1.80485e-35 * (((-locals.var_xg1x_edge_dn6) * assign27970_e29942) + (assign27970_e29925 * (((0.5 * (-locals.var_xg1x_edge_dn6)) * assign27970_e29940) + (assign27970_e29932 * ((-locals.var_xg1x_edge_dn6) * 0.3333333333333)))))) / (assign27970_e29944 * assign27970_e29944))), (-((1.80485e-35 * (((-locals.var_xg1x_edge_dn7) * assign27970_e29942) + (assign27970_e29925 * (((0.5 * (-locals.var_xg1x_edge_dn7)) * assign27970_e29940) + (assign27970_e29932 * ((-locals.var_xg1x_edge_dn7) * 0.3333333333333)))))) / (assign27970_e29944 * assign27970_e29944))), (-((1.80485e-35 * (((-locals.var_xg1x_edge_dn8) * assign27970_e29942) + (assign27970_e29925 * (((0.5 * (-locals.var_xg1x_edge_dn8)) * assign27970_e29940) + (assign27970_e29932 * ((-locals.var_xg1x_edge_dn8) * 0.3333333333333)))))) / (assign27970_e29944 * assign27970_e29944))), (-((1.80485e-35 * (((-locals.var_xg1x_edge_dn9) * assign27970_e29942) + (assign27970_e29925 * (((0.5 * (-locals.var_xg1x_edge_dn9)) * assign27970_e29940) + (assign27970_e29932 * ((-locals.var_xg1x_edge_dn9) * 0.3333333333333)))))) / (assign27970_e29944 * assign27970_e29944))),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign27970_e29947;
        locals.var_w_temp_dn4 = assign27970_e29947_d_n4;
        locals.var_w_temp_dn6 = assign27970_e29947_d_n6;
        locals.var_w_temp_dn7 = assign27970_e29947_d_n7;
        locals.var_w_temp_dn8 = assign27970_e29947_d_n8;
        locals.var_w_temp_dn9 = assign27970_e29947_d_n9;

        let (assign27980_e29958, assign27980_e29958_d_n4, assign27980_e29958_d_n6, assign27980_e29958_d_n7, assign27980_e29958_d_n8, assign27980_e29958_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 != 0.0)) {
        let assign27980_e29956: f64 = (locals.var_prefac_qilow_edge * locals.var_w_temp);
        (assign27980_e29956, ((locals.var_prefac_qilow_edge_dn4 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn4)), ((locals.var_prefac_qilow_edge_dn6 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn6)), ((locals.var_prefac_qilow_edge_dn7 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn7)), ((locals.var_prefac_qilow_edge_dn8 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn8)), ((locals.var_prefac_qilow_edge_dn9 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign27980_e29958;
        locals.var_temp_dn4 = assign27980_e29958_d_n4;
        locals.var_temp_dn6 = assign27980_e29958_d_n6;
        locals.var_temp_dn7 = assign27980_e29958_d_n7;
        locals.var_temp_dn8 = assign27980_e29958_d_n8;
        locals.var_temp_dn9 = assign27980_e29958_d_n9;

        let (assign27990_e29971, assign27990_e29971_d_n4, assign27990_e29971_d_n6, assign27990_e29971_d_n7, assign27990_e29971_d_n8, assign27990_e29971_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) {
        let assign27990_e29967: f64 = (locals.var_prefac_qilow_edge).ln();
        let assign27990_e29969: f64 = (assign27990_e29967 + locals.var_xg1x_edge);
        (assign27990_e29969, ((locals.var_prefac_qilow_edge_dn4 / locals.var_prefac_qilow_edge) + locals.var_xg1x_edge_dn4), ((locals.var_prefac_qilow_edge_dn6 / locals.var_prefac_qilow_edge) + locals.var_xg1x_edge_dn6), ((locals.var_prefac_qilow_edge_dn7 / locals.var_prefac_qilow_edge) + locals.var_xg1x_edge_dn7), ((locals.var_prefac_qilow_edge_dn8 / locals.var_prefac_qilow_edge) + locals.var_xg1x_edge_dn8), ((locals.var_prefac_qilow_edge_dn9 / locals.var_prefac_qilow_edge) + locals.var_xg1x_edge_dn9),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign27990_e29971;
        locals.var_w_temp_dn4 = assign27990_e29971_d_n4;
        locals.var_w_temp_dn6 = assign27990_e29971_d_n6;
        locals.var_w_temp_dn7 = assign27990_e29971_d_n7;
        locals.var_w_temp_dn8 = assign27990_e29971_d_n8;
        locals.var_w_temp_dn9 = assign27990_e29971_d_n9;

        let (assign28000_e29992, assign28000_e29992_d_n4, assign28000_e29992_d_n6, assign28000_e29992_d_n7, assign28000_e29992_d_n8, assign28000_e29992_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) {
        let assign28000_e29983: f64 = (1.0 + locals.var_w_temp);
        let assign28000_e29984: f64 = (assign28000_e29983).ln();
        let assign28000_e29987: f64 = (2.0 + locals.var_w_temp);
        let assign28000_e29988: f64 = (assign28000_e29984 / assign28000_e29987);
        let assign28000_e29989: f64 = (1.0 - assign28000_e29988);
        let assign28000_e29990: f64 = (locals.var_w_temp * assign28000_e29989);
        (assign28000_e29990, ((locals.var_w_temp_dn4 * assign28000_e29989) + (locals.var_w_temp * (-((((locals.var_w_temp_dn4 / assign28000_e29983) * assign28000_e29987) - (assign28000_e29984 * locals.var_w_temp_dn4)) / (assign28000_e29987 * assign28000_e29987))))), ((locals.var_w_temp_dn6 * assign28000_e29989) + (locals.var_w_temp * (-((((locals.var_w_temp_dn6 / assign28000_e29983) * assign28000_e29987) - (assign28000_e29984 * locals.var_w_temp_dn6)) / (assign28000_e29987 * assign28000_e29987))))), ((locals.var_w_temp_dn7 * assign28000_e29989) + (locals.var_w_temp * (-((((locals.var_w_temp_dn7 / assign28000_e29983) * assign28000_e29987) - (assign28000_e29984 * locals.var_w_temp_dn7)) / (assign28000_e29987 * assign28000_e29987))))), ((locals.var_w_temp_dn8 * assign28000_e29989) + (locals.var_w_temp * (-((((locals.var_w_temp_dn8 / assign28000_e29983) * assign28000_e29987) - (assign28000_e29984 * locals.var_w_temp_dn8)) / (assign28000_e29987 * assign28000_e29987))))), ((locals.var_w_temp_dn9 * assign28000_e29989) + (locals.var_w_temp * (-((((locals.var_w_temp_dn9 / assign28000_e29983) * assign28000_e29987) - (assign28000_e29984 * locals.var_w_temp_dn9)) / (assign28000_e29987 * assign28000_e29987))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28000_e29992;
        locals.var_temp_dn4 = assign28000_e29992_d_n4;
        locals.var_temp_dn6 = assign28000_e29992_d_n6;
        locals.var_temp_dn7 = assign28000_e29992_d_n7;
        locals.var_temp_dn8 = assign28000_e29992_d_n8;
        locals.var_temp_dn9 = assign28000_e29992_d_n9;

        let (assign28010_e29996, assign28010_e29996_d_n4, assign28010_e29996_d_n6, assign28010_e29996_d_n7, assign28010_e29996_d_n8, assign28010_e29996_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    } else {
        (locals.var_qis_edge, locals.var_qis_edge_dn4, locals.var_qis_edge_dn6, locals.var_qis_edge_dn7, locals.var_qis_edge_dn8, locals.var_qis_edge_dn9,)
    }
};
        locals.var_qis_edge = assign28010_e29996;
        locals.var_qis_edge_dn4 = assign28010_e29996_d_n4;
        locals.var_qis_edge_dn6 = assign28010_e29996_d_n6;
        locals.var_qis_edge_dn7 = assign28010_e29996_d_n7;
        locals.var_qis_edge_dn8 = assign28010_e29996_d_n8;
        locals.var_qis_edge_dn9 = assign28010_e29996_d_n9;

        let assign28020_e29999: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28020_e30001: f64 = if assign28020_e29999 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard787 = assign28020_e30001;

        let (assign28030_e30015, assign28030_e30015_d_n4, assign28030_e30015_d_n6, assign28030_e30015_d_n7, assign28030_e30015_d_n8, assign28030_e30015_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard787 != 0.0)) {
        let assign28030_e30009: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28030_e30010: f64 = (assign28030_e30009).exp();
        let assign28030_e30011: f64 = (locals.var_prefac_qilow_edge * assign28030_e30010);
        let assign28030_e30012: f64 = (1.0 + assign28030_e30011);
        let assign28030_e30013: f64 = (assign28030_e30012).ln();
        (assign28030_e30013, (((locals.var_prefac_qilow_edge_dn4 * assign28030_e30010) + (locals.var_prefac_qilow_edge * (assign28030_e30010 * (locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4)))) / assign28030_e30012), (((locals.var_prefac_qilow_edge_dn6 * assign28030_e30010) + (locals.var_prefac_qilow_edge * (assign28030_e30010 * (locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6)))) / assign28030_e30012), (((locals.var_prefac_qilow_edge_dn7 * assign28030_e30010) + (locals.var_prefac_qilow_edge * (assign28030_e30010 * (locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7)))) / assign28030_e30012), (((locals.var_prefac_qilow_edge_dn8 * assign28030_e30010) + (locals.var_prefac_qilow_edge * (assign28030_e30010 * (locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8)))) / assign28030_e30012), (((locals.var_prefac_qilow_edge_dn9 * assign28030_e30010) + (locals.var_prefac_qilow_edge * (assign28030_e30010 * (locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9)))) / assign28030_e30012),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28030_e30015;
        locals.var_w_temp_dn4 = assign28030_e30015_d_n4;
        locals.var_w_temp_dn6 = assign28030_e30015_d_n6;
        locals.var_w_temp_dn7 = assign28030_e30015_d_n7;
        locals.var_w_temp_dn8 = assign28030_e30015_d_n8;
        locals.var_w_temp_dn9 = assign28030_e30015_d_n9;

        let (assign28040_e30032, assign28040_e30032_d_n4, assign28040_e30032_d_n6, assign28040_e30032_d_n7, assign28040_e30032_d_n8, assign28040_e30032_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard787 != 0.0)) {
        let assign28040_e30023: f64 = (1.0 + locals.var_w_temp);
        let assign28040_e30024: f64 = (assign28040_e30023).ln();
        let assign28040_e30027: f64 = (2.0 + locals.var_w_temp);
        let assign28040_e30028: f64 = (assign28040_e30024 / assign28040_e30027);
        let assign28040_e30029: f64 = (1.0 - assign28040_e30028);
        let assign28040_e30030: f64 = (locals.var_w_temp * assign28040_e30029);
        (assign28040_e30030, ((locals.var_w_temp_dn4 * assign28040_e30029) + (locals.var_w_temp * (-((((locals.var_w_temp_dn4 / assign28040_e30023) * assign28040_e30027) - (assign28040_e30024 * locals.var_w_temp_dn4)) / (assign28040_e30027 * assign28040_e30027))))), ((locals.var_w_temp_dn6 * assign28040_e30029) + (locals.var_w_temp * (-((((locals.var_w_temp_dn6 / assign28040_e30023) * assign28040_e30027) - (assign28040_e30024 * locals.var_w_temp_dn6)) / (assign28040_e30027 * assign28040_e30027))))), ((locals.var_w_temp_dn7 * assign28040_e30029) + (locals.var_w_temp * (-((((locals.var_w_temp_dn7 / assign28040_e30023) * assign28040_e30027) - (assign28040_e30024 * locals.var_w_temp_dn7)) / (assign28040_e30027 * assign28040_e30027))))), ((locals.var_w_temp_dn8 * assign28040_e30029) + (locals.var_w_temp * (-((((locals.var_w_temp_dn8 / assign28040_e30023) * assign28040_e30027) - (assign28040_e30024 * locals.var_w_temp_dn8)) / (assign28040_e30027 * assign28040_e30027))))), ((locals.var_w_temp_dn9 * assign28040_e30029) + (locals.var_w_temp * (-((((locals.var_w_temp_dn9 / assign28040_e30023) * assign28040_e30027) - (assign28040_e30024 * locals.var_w_temp_dn9)) / (assign28040_e30027 * assign28040_e30027))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28040_e30032;
        locals.var_temp_dn4 = assign28040_e30032_d_n4;
        locals.var_temp_dn6 = assign28040_e30032_d_n6;
        locals.var_temp_dn7 = assign28040_e30032_d_n7;
        locals.var_temp_dn8 = assign28040_e30032_d_n8;
        locals.var_temp_dn9 = assign28040_e30032_d_n9;

        let assign28050_e30035: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28050_e30037: f64 = if assign28050_e30035 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard788 = assign28050_e30037;

        let assign28060_e30040: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28060_e30042: f64 = (-80.0);
        let assign28060_e30043: f64 = if assign28060_e30040 > assign28060_e30042 { 1.0 } else { 0.0 };
        locals.var_guard789 = assign28060_e30043;

    }

    pub(super) fn stamp_transient_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28070_e30057, assign28070_e30057_d_n4, assign28070_e30057_d_n6, assign28070_e30057_d_n7, assign28070_e30057_d_n8, assign28070_e30057_d_n9,) = {
    if ((((locals.var_guard782 != 0.0) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 != 0.0)) {
        let assign28070_e30054: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28070_e30055: f64 = (assign28070_e30054).exp();
        (assign28070_e30055, (assign28070_e30055 * (locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4)), (assign28070_e30055 * (locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6)), (assign28070_e30055 * (locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7)), (assign28070_e30055 * (locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8)), (assign28070_e30055 * (locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9)),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28070_e30057;
        locals.var_w_temp_dn4 = assign28070_e30057_d_n4;
        locals.var_w_temp_dn6 = assign28070_e30057_d_n6;
        locals.var_w_temp_dn7 = assign28070_e30057_d_n7;
        locals.var_w_temp_dn8 = assign28070_e30057_d_n8;
        locals.var_w_temp_dn9 = assign28070_e30057_d_n9;

        let (assign28080_e30100, assign28080_e30100_d_n4, assign28080_e30100_d_n6, assign28080_e30100_d_n7, assign28080_e30100_d_n8, assign28080_e30100_d_n9,) = {
    if ((((locals.var_guard782 != 0.0) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 == 0.0)) {
        let assign28080_e30071: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28080_e30072: f64 = (-assign28080_e30071);
        let assign28080_e30074: f64 = (assign28080_e30072 - 80.0);
        let assign28080_e30079: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28080_e30080: f64 = (-assign28080_e30079);
        let assign28080_e30082: f64 = (assign28080_e30080 - 80.0);
        let assign28080_e30083: f64 = (0.5 * assign28080_e30082);
        let assign28080_e30087: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28080_e30088: f64 = (-assign28080_e30087);
        let assign28080_e30090: f64 = (assign28080_e30088 - 80.0);
        let assign28080_e30092: f64 = (assign28080_e30090 * 0.3333333333333);
        let assign28080_e30093: f64 = (1.0 + assign28080_e30092);
        let assign28080_e30094: f64 = (assign28080_e30083 * assign28080_e30093);
        let assign28080_e30095: f64 = (1.0 + assign28080_e30094);
        let assign28080_e30096: f64 = (assign28080_e30074 * assign28080_e30095);
        let assign28080_e30097: f64 = (1.0 + assign28080_e30096);
        let assign28080_e30098: f64 = (1.80485e-35 / assign28080_e30097);
        (assign28080_e30098, (-((1.80485e-35 * (((-(locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4)) * assign28080_e30095) + (assign28080_e30074 * (((0.5 * (-(locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4))) * assign28080_e30093) + (assign28080_e30083 * ((-(locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4)) * 0.3333333333333)))))) / (assign28080_e30097 * assign28080_e30097))), (-((1.80485e-35 * (((-(locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6)) * assign28080_e30095) + (assign28080_e30074 * (((0.5 * (-(locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6))) * assign28080_e30093) + (assign28080_e30083 * ((-(locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6)) * 0.3333333333333)))))) / (assign28080_e30097 * assign28080_e30097))), (-((1.80485e-35 * (((-(locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7)) * assign28080_e30095) + (assign28080_e30074 * (((0.5 * (-(locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7))) * assign28080_e30093) + (assign28080_e30083 * ((-(locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7)) * 0.3333333333333)))))) / (assign28080_e30097 * assign28080_e30097))), (-((1.80485e-35 * (((-(locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8)) * assign28080_e30095) + (assign28080_e30074 * (((0.5 * (-(locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8))) * assign28080_e30093) + (assign28080_e30083 * ((-(locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8)) * 0.3333333333333)))))) / (assign28080_e30097 * assign28080_e30097))), (-((1.80485e-35 * (((-(locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9)) * assign28080_e30095) + (assign28080_e30074 * (((0.5 * (-(locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9))) * assign28080_e30093) + (assign28080_e30083 * ((-(locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9)) * 0.3333333333333)))))) / (assign28080_e30097 * assign28080_e30097))),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28080_e30100;
        locals.var_w_temp_dn4 = assign28080_e30100_d_n4;
        locals.var_w_temp_dn6 = assign28080_e30100_d_n6;
        locals.var_w_temp_dn7 = assign28080_e30100_d_n7;
        locals.var_w_temp_dn8 = assign28080_e30100_d_n8;
        locals.var_w_temp_dn9 = assign28080_e30100_d_n9;

        let (assign28090_e30111, assign28090_e30111_d_n4, assign28090_e30111_d_n6, assign28090_e30111_d_n7, assign28090_e30111_d_n8, assign28090_e30111_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign28090_e30109: f64 = (locals.var_prefac_qilow_edge * locals.var_w_temp);
        (assign28090_e30109, ((locals.var_prefac_qilow_edge_dn4 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn4)), ((locals.var_prefac_qilow_edge_dn6 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn6)), ((locals.var_prefac_qilow_edge_dn7 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn7)), ((locals.var_prefac_qilow_edge_dn8 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn8)), ((locals.var_prefac_qilow_edge_dn9 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28090_e30111;
        locals.var_temp_dn4 = assign28090_e30111_d_n4;
        locals.var_temp_dn6 = assign28090_e30111_d_n6;
        locals.var_temp_dn7 = assign28090_e30111_d_n7;
        locals.var_temp_dn8 = assign28090_e30111_d_n8;
        locals.var_temp_dn9 = assign28090_e30111_d_n9;

        let (assign28100_e30126, assign28100_e30126_d_n4, assign28100_e30126_d_n6, assign28100_e30126_d_n7, assign28100_e30126_d_n8, assign28100_e30126_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 == 0.0)) {
        let assign28100_e30120: f64 = (locals.var_prefac_qilow_edge).ln();
        let assign28100_e30123: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28100_e30124: f64 = (assign28100_e30120 + assign28100_e30123);
        (assign28100_e30124, ((locals.var_prefac_qilow_edge_dn4 / locals.var_prefac_qilow_edge) + (locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4)), ((locals.var_prefac_qilow_edge_dn6 / locals.var_prefac_qilow_edge) + (locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6)), ((locals.var_prefac_qilow_edge_dn7 / locals.var_prefac_qilow_edge) + (locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7)), ((locals.var_prefac_qilow_edge_dn8 / locals.var_prefac_qilow_edge) + (locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8)), ((locals.var_prefac_qilow_edge_dn9 / locals.var_prefac_qilow_edge) + (locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9)),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28100_e30126;
        locals.var_w_temp_dn4 = assign28100_e30126_d_n4;
        locals.var_w_temp_dn6 = assign28100_e30126_d_n6;
        locals.var_w_temp_dn7 = assign28100_e30126_d_n7;
        locals.var_w_temp_dn8 = assign28100_e30126_d_n8;
        locals.var_w_temp_dn9 = assign28100_e30126_d_n9;

        let (assign28110_e30147, assign28110_e30147_d_n4, assign28110_e30147_d_n6, assign28110_e30147_d_n7, assign28110_e30147_d_n8, assign28110_e30147_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 == 0.0)) {
        let assign28110_e30138: f64 = (1.0 + locals.var_w_temp);
        let assign28110_e30139: f64 = (assign28110_e30138).ln();
        let assign28110_e30142: f64 = (2.0 + locals.var_w_temp);
        let assign28110_e30143: f64 = (assign28110_e30139 / assign28110_e30142);
        let assign28110_e30144: f64 = (1.0 - assign28110_e30143);
        let assign28110_e30145: f64 = (locals.var_w_temp * assign28110_e30144);
        (assign28110_e30145, ((locals.var_w_temp_dn4 * assign28110_e30144) + (locals.var_w_temp * (-((((locals.var_w_temp_dn4 / assign28110_e30138) * assign28110_e30142) - (assign28110_e30139 * locals.var_w_temp_dn4)) / (assign28110_e30142 * assign28110_e30142))))), ((locals.var_w_temp_dn6 * assign28110_e30144) + (locals.var_w_temp * (-((((locals.var_w_temp_dn6 / assign28110_e30138) * assign28110_e30142) - (assign28110_e30139 * locals.var_w_temp_dn6)) / (assign28110_e30142 * assign28110_e30142))))), ((locals.var_w_temp_dn7 * assign28110_e30144) + (locals.var_w_temp * (-((((locals.var_w_temp_dn7 / assign28110_e30138) * assign28110_e30142) - (assign28110_e30139 * locals.var_w_temp_dn7)) / (assign28110_e30142 * assign28110_e30142))))), ((locals.var_w_temp_dn8 * assign28110_e30144) + (locals.var_w_temp * (-((((locals.var_w_temp_dn8 / assign28110_e30138) * assign28110_e30142) - (assign28110_e30139 * locals.var_w_temp_dn8)) / (assign28110_e30142 * assign28110_e30142))))), ((locals.var_w_temp_dn9 * assign28110_e30144) + (locals.var_w_temp * (-((((locals.var_w_temp_dn9 / assign28110_e30138) * assign28110_e30142) - (assign28110_e30139 * locals.var_w_temp_dn9)) / (assign28110_e30142 * assign28110_e30142))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28110_e30147;
        locals.var_temp_dn4 = assign28110_e30147_d_n4;
        locals.var_temp_dn6 = assign28110_e30147_d_n6;
        locals.var_temp_dn7 = assign28110_e30147_d_n7;
        locals.var_temp_dn8 = assign28110_e30147_d_n8;
        locals.var_temp_dn9 = assign28110_e30147_d_n9;

        let (assign28120_e30151, assign28120_e30151_d_n4, assign28120_e30151_d_n6, assign28120_e30151_d_n7, assign28120_e30151_d_n8, assign28120_e30151_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    } else {
        (locals.var_qid_edge, locals.var_qid_edge_dn4, locals.var_qid_edge_dn6, locals.var_qid_edge_dn7, locals.var_qid_edge_dn8, locals.var_qid_edge_dn9,)
    }
};
        locals.var_qid_edge = assign28120_e30151;
        locals.var_qid_edge_dn4 = assign28120_e30151_d_n4;
        locals.var_qid_edge_dn6 = assign28120_e30151_d_n6;
        locals.var_qid_edge_dn7 = assign28120_e30151_d_n7;
        locals.var_qid_edge_dn8 = assign28120_e30151_d_n8;
        locals.var_qid_edge_dn9 = assign28120_e30151_d_n9;

        let (assign28130_e30165, assign28130_e30165_d_n4, assign28130_e30165_d_n6, assign28130_e30165_d_n7, assign28130_e30165_d_n8, assign28130_e30165_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign28130_e30156: f64 = (locals.var_qis_edge + locals.var_qid_edge);
        let assign28130_e30157: f64 = (0.5 * assign28130_e30156);
        let assign28130_e30159: f64 = (assign28130_e30157 + 1.0);
        let assign28130_e30162: f64 = (locals.var_qis_edge - locals.var_qid_edge);
        let assign28130_e30163: f64 = (assign28130_e30159 * assign28130_e30162);
        (assign28130_e30163, (((0.5 * (locals.var_qis_edge_dn4 + locals.var_qid_edge_dn4)) * assign28130_e30162) + (assign28130_e30159 * (locals.var_qis_edge_dn4 - locals.var_qid_edge_dn4))), (((0.5 * (locals.var_qis_edge_dn6 + locals.var_qid_edge_dn6)) * assign28130_e30162) + (assign28130_e30159 * (locals.var_qis_edge_dn6 - locals.var_qid_edge_dn6))), (((0.5 * (locals.var_qis_edge_dn7 + locals.var_qid_edge_dn7)) * assign28130_e30162) + (assign28130_e30159 * (locals.var_qis_edge_dn7 - locals.var_qid_edge_dn7))), (((0.5 * (locals.var_qis_edge_dn8 + locals.var_qid_edge_dn8)) * assign28130_e30162) + (assign28130_e30159 * (locals.var_qis_edge_dn8 - locals.var_qid_edge_dn8))), (((0.5 * (locals.var_qis_edge_dn9 + locals.var_qid_edge_dn9)) * assign28130_e30162) + (assign28130_e30159 * (locals.var_qis_edge_dn9 - locals.var_qid_edge_dn9))),)
    } else {
        (locals.var_norm_ids_edge, locals.var_norm_ids_edge_dn4, locals.var_norm_ids_edge_dn6, locals.var_norm_ids_edge_dn7, locals.var_norm_ids_edge_dn8, locals.var_norm_ids_edge_dn9,)
    }
};
        locals.var_norm_ids_edge = assign28130_e30165;
        locals.var_norm_ids_edge_dn4 = assign28130_e30165_d_n4;
        locals.var_norm_ids_edge_dn6 = assign28130_e30165_d_n6;
        locals.var_norm_ids_edge_dn7 = assign28130_e30165_d_n7;
        locals.var_norm_ids_edge_dn8 = assign28130_e30165_d_n8;
        locals.var_norm_ids_edge_dn9 = assign28130_e30165_d_n9;

        let (assign28140_e30173, assign28140_e30173_d_n4, assign28140_e30173_d_n6, assign28140_e30173_d_n7, assign28140_e30173_d_n8, assign28140_e30173_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign28140_e30169: f64 = (locals.var_phit_edge * locals.var_phit_edge);
        let assign28140_e30171: f64 = (assign28140_e30169 * locals.var_betnedge_i);
        (assign28140_e30171, ((((locals.var_phit_edge_dn4 * locals.var_phit_edge) + (locals.var_phit_edge * locals.var_phit_edge_dn4)) * locals.var_betnedge_i) + (assign28140_e30169 * locals.var_betnedge_i_dn4)), ((((locals.var_phit_edge_dn6 * locals.var_phit_edge) + (locals.var_phit_edge * locals.var_phit_edge_dn6)) * locals.var_betnedge_i) + (assign28140_e30169 * locals.var_betnedge_i_dn6)), ((((locals.var_phit_edge_dn7 * locals.var_phit_edge) + (locals.var_phit_edge * locals.var_phit_edge_dn7)) * locals.var_betnedge_i) + (assign28140_e30169 * locals.var_betnedge_i_dn7)), ((((locals.var_phit_edge_dn8 * locals.var_phit_edge) + (locals.var_phit_edge * locals.var_phit_edge_dn8)) * locals.var_betnedge_i) + (assign28140_e30169 * locals.var_betnedge_i_dn8)), ((((locals.var_phit_edge_dn9 * locals.var_phit_edge) + (locals.var_phit_edge * locals.var_phit_edge_dn9)) * locals.var_betnedge_i) + (assign28140_e30169 * locals.var_betnedge_i_dn9)),)
    } else {
        (locals.var_fact_ids_edge, locals.var_fact_ids_edge_dn4, locals.var_fact_ids_edge_dn6, locals.var_fact_ids_edge_dn7, locals.var_fact_ids_edge_dn8, locals.var_fact_ids_edge_dn9,)
    }
};
        locals.var_fact_ids_edge = assign28140_e30173;
        locals.var_fact_ids_edge_dn4 = assign28140_e30173_d_n4;
        locals.var_fact_ids_edge_dn6 = assign28140_e30173_d_n6;
        locals.var_fact_ids_edge_dn7 = assign28140_e30173_d_n7;
        locals.var_fact_ids_edge_dn8 = assign28140_e30173_d_n8;
        locals.var_fact_ids_edge_dn9 = assign28140_e30173_d_n9;

        let (assign28150_e30183, assign28150_e30183_d_n4, assign28150_e30183_d_n6, assign28150_e30183_d_n7, assign28150_e30183_d_n8, assign28150_e30183_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign28150_e30177: f64 = (locals.var_fact_ids_edge * locals.var_cox1prime);
        let assign28150_e30179: f64 = (assign28150_e30177 * locals.var_norm_ids_edge);
        let assign28150_e30181: f64 = (assign28150_e30179 / locals.var_gmob_dc);
        (assign28150_e30181, ((((((locals.var_fact_ids_edge_dn4 * locals.var_cox1prime) * locals.var_norm_ids_edge) + (assign28150_e30177 * locals.var_norm_ids_edge_dn4)) * locals.var_gmob_dc) - (assign28150_e30179 * locals.var_gmob_dc_dn4)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((locals.var_fact_ids_edge_dn6 * locals.var_cox1prime) * locals.var_norm_ids_edge) + (assign28150_e30177 * locals.var_norm_ids_edge_dn6)) * locals.var_gmob_dc) - (assign28150_e30179 * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((locals.var_fact_ids_edge_dn7 * locals.var_cox1prime) * locals.var_norm_ids_edge) + (assign28150_e30177 * locals.var_norm_ids_edge_dn7)) * locals.var_gmob_dc) - (assign28150_e30179 * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((locals.var_fact_ids_edge_dn8 * locals.var_cox1prime) * locals.var_norm_ids_edge) + (assign28150_e30177 * locals.var_norm_ids_edge_dn8)) * locals.var_gmob_dc) - (assign28150_e30179 * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((locals.var_fact_ids_edge_dn9 * locals.var_cox1prime) * locals.var_norm_ids_edge) + (assign28150_e30177 * locals.var_norm_ids_edge_dn9)) * locals.var_gmob_dc) - (assign28150_e30179 * locals.var_gmob_dc_dn9)) / (locals.var_gmob_dc * locals.var_gmob_dc)),)
    } else {
        (locals.var_ids_edge, locals.var_ids_edge_dn4, locals.var_ids_edge_dn6, locals.var_ids_edge_dn7, locals.var_ids_edge_dn8, locals.var_ids_edge_dn9,)
    }
};
        locals.var_ids_edge = assign28150_e30183;
        locals.var_ids_edge_dn4 = assign28150_e30183_d_n4;
        locals.var_ids_edge_dn6 = assign28150_e30183_d_n6;
        locals.var_ids_edge_dn7 = assign28150_e30183_d_n7;
        locals.var_ids_edge_dn8 = assign28150_e30183_d_n8;
        locals.var_ids_edge_dn9 = assign28150_e30183_d_n9;

        locals.var_mavl = 0.0;
        locals.var_mavl_dn4 = 0.0;
        locals.var_mavl_dn6 = 0.0;
        locals.var_mavl_dn7 = 0.0;
        locals.var_mavl_dn8 = 0.0;
        locals.var_mavl_dn9 = 0.0;

        locals.var_iimpact = 0.0;
        locals.var_iimpact_dn4 = 0.0;
        locals.var_iimpact_dn6 = 0.0;
        locals.var_iimpact_dn7 = 0.0;
        locals.var_iimpact_dn8 = 0.0;
        locals.var_iimpact_dn9 = 0.0;

        let assign28180_e30188: f64 = if p.p8 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard790 = assign28180_e30188;

        let (assign28190_e30198, assign28190_e30198_d_n4, assign28190_e30198_d_n6, assign28190_e30198_d_n7, assign28190_e30198_d_n8, assign28190_e30198_d_n9,) = {
    if (locals.var_guard790 != 0.0) {
        let assign28190_e30193: f64 = (locals.var_a3_i * locals.var_xdeff_dc);
        let assign28190_e30194: f64 = (locals.var_xd - assign28190_e30193);
        let assign28190_e30196: f64 = (assign28190_e30194 / locals.var_inv_phit);
        (assign28190_e30196, ((((locals.var_xd_dn4 - (locals.var_a3_i * locals.var_xdeff_dc_dn4)) * locals.var_inv_phit) - (assign28190_e30194 * locals.var_inv_phit_dn4)) / (locals.var_inv_phit * locals.var_inv_phit)), ((((locals.var_xd_dn6 - (locals.var_a3_i * locals.var_xdeff_dc_dn6)) * locals.var_inv_phit) - (assign28190_e30194 * locals.var_inv_phit_dn6)) / (locals.var_inv_phit * locals.var_inv_phit)), ((((locals.var_xd_dn7 - (locals.var_a3_i * locals.var_xdeff_dc_dn7)) * locals.var_inv_phit) - (assign28190_e30194 * locals.var_inv_phit_dn7)) / (locals.var_inv_phit * locals.var_inv_phit)), ((((locals.var_xd_dn8 - (locals.var_a3_i * locals.var_xdeff_dc_dn8)) * locals.var_inv_phit) - (assign28190_e30194 * locals.var_inv_phit_dn8)) / (locals.var_inv_phit * locals.var_inv_phit)), ((((locals.var_xd_dn9 - (locals.var_a3_i * locals.var_xdeff_dc_dn9)) * locals.var_inv_phit) - (assign28190_e30194 * locals.var_inv_phit_dn9)) / (locals.var_inv_phit * locals.var_inv_phit)),)
    } else {
        (locals.var_delvsat, locals.var_delvsat_dn4, locals.var_delvsat_dn6, locals.var_delvsat_dn7, locals.var_delvsat_dn8, locals.var_delvsat_dn9,)
    }
};
        locals.var_delvsat = assign28190_e30198;
        locals.var_delvsat_dn4 = assign28190_e30198_d_n4;
        locals.var_delvsat_dn6 = assign28190_e30198_d_n6;
        locals.var_delvsat_dn7 = assign28190_e30198_d_n7;
        locals.var_delvsat_dn8 = assign28190_e30198_d_n8;
        locals.var_delvsat_dn9 = assign28190_e30198_d_n9;

        let assign28200_e30201: f64 = if locals.var_delvsat > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard791 = assign28200_e30201;

        let (assign28210_e30214, assign28210_e30214_d_n4, assign28210_e30214_d_n6, assign28210_e30214_d_n7, assign28210_e30214_d_n8, assign28210_e30214_d_n9,) = {
    if ((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) {
        let assign28210_e30206: f64 = (-1.0);
        let assign28210_e30208: f64 = (assign28210_e30206 * locals.var_a2_i);
        let assign28210_e30211: f64 = (locals.var_delvsat + 1e-30);
        let assign28210_e30212: f64 = (assign28210_e30208 / assign28210_e30211);
        (assign28210_e30212, ((((assign28210_e30206 * locals.var_a2_i_dn4) * assign28210_e30211) - (assign28210_e30208 * locals.var_delvsat_dn4)) / (assign28210_e30211 * assign28210_e30211)), ((((assign28210_e30206 * locals.var_a2_i_dn6) * assign28210_e30211) - (assign28210_e30208 * locals.var_delvsat_dn6)) / (assign28210_e30211 * assign28210_e30211)), ((((assign28210_e30206 * locals.var_a2_i_dn7) * assign28210_e30211) - (assign28210_e30208 * locals.var_delvsat_dn7)) / (assign28210_e30211 * assign28210_e30211)), ((((assign28210_e30206 * locals.var_a2_i_dn8) * assign28210_e30211) - (assign28210_e30208 * locals.var_delvsat_dn8)) / (assign28210_e30211 * assign28210_e30211)), ((((assign28210_e30206 * locals.var_a2_i_dn9) * assign28210_e30211) - (assign28210_e30208 * locals.var_delvsat_dn9)) / (assign28210_e30211 * assign28210_e30211)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign28210_e30214;
        locals.var_temp2_dn4 = assign28210_e30214_d_n4;
        locals.var_temp2_dn6 = assign28210_e30214_d_n6;
        locals.var_temp2_dn7 = assign28210_e30214_d_n7;
        locals.var_temp2_dn8 = assign28210_e30214_d_n8;
        locals.var_temp2_dn9 = assign28210_e30214_d_n9;

        let assign28220_e30216: f64 = (locals.var_temp2).abs();
        let assign28220_e30218: f64 = if assign28220_e30216 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard792 = assign28220_e30218;

        let (assign28230_e30227, assign28230_e30227_d_n4, assign28230_e30227_d_n6, assign28230_e30227_d_n7, assign28230_e30227_d_n8, assign28230_e30227_d_n9,) = {
    if (((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign28230_e30225: f64 = (locals.var_temp2).exp();
        (assign28230_e30225, (assign28230_e30225 * locals.var_temp2_dn4), (assign28230_e30225 * locals.var_temp2_dn6), (assign28230_e30225 * locals.var_temp2_dn7), (assign28230_e30225 * locals.var_temp2_dn8), (assign28230_e30225 * locals.var_temp2_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28230_e30227;
        locals.var_temp_dn4 = assign28230_e30227_d_n4;
        locals.var_temp_dn6 = assign28230_e30227_d_n6;
        locals.var_temp_dn7 = assign28230_e30227_d_n7;
        locals.var_temp_dn8 = assign28230_e30227_d_n8;
        locals.var_temp_dn9 = assign28230_e30227_d_n9;

        let assign28240_e30230: f64 = (-80.0);
        let assign28240_e30231: f64 = if locals.var_temp2 < assign28240_e30230 { 1.0 } else { 0.0 };
        locals.var_guard793 = assign28240_e30231;

        let (assign28250_e30267, assign28250_e30267_d_n4, assign28250_e30267_d_n6, assign28250_e30267_d_n7, assign28250_e30267_d_n8, assign28250_e30267_d_n9,) = {
    if ((((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) && (locals.var_guard792 == 0.0)) && (locals.var_guard793 != 0.0)) {
        let assign28250_e30243: f64 = (-locals.var_temp2);
        let assign28250_e30245: f64 = (assign28250_e30243 - 80.0);
        let assign28250_e30249: f64 = (-locals.var_temp2);
        let assign28250_e30251: f64 = (assign28250_e30249 - 80.0);
        let assign28250_e30252: f64 = (0.5 * assign28250_e30251);
        let assign28250_e30255: f64 = (-locals.var_temp2);
        let assign28250_e30257: f64 = (assign28250_e30255 - 80.0);
        let assign28250_e30259: f64 = (assign28250_e30257 * 0.3333333333333);
        let assign28250_e30260: f64 = (1.0 + assign28250_e30259);
        let assign28250_e30261: f64 = (assign28250_e30252 * assign28250_e30260);
        let assign28250_e30262: f64 = (1.0 + assign28250_e30261);
        let assign28250_e30263: f64 = (assign28250_e30245 * assign28250_e30262);
        let assign28250_e30264: f64 = (1.0 + assign28250_e30263);
        let assign28250_e30265: f64 = (1.80485e-35 / assign28250_e30264);
        (assign28250_e30265, (-((1.80485e-35 * (((-locals.var_temp2_dn4) * assign28250_e30262) + (assign28250_e30245 * (((0.5 * (-locals.var_temp2_dn4)) * assign28250_e30260) + (assign28250_e30252 * ((-locals.var_temp2_dn4) * 0.3333333333333)))))) / (assign28250_e30264 * assign28250_e30264))), (-((1.80485e-35 * (((-locals.var_temp2_dn6) * assign28250_e30262) + (assign28250_e30245 * (((0.5 * (-locals.var_temp2_dn6)) * assign28250_e30260) + (assign28250_e30252 * ((-locals.var_temp2_dn6) * 0.3333333333333)))))) / (assign28250_e30264 * assign28250_e30264))), (-((1.80485e-35 * (((-locals.var_temp2_dn7) * assign28250_e30262) + (assign28250_e30245 * (((0.5 * (-locals.var_temp2_dn7)) * assign28250_e30260) + (assign28250_e30252 * ((-locals.var_temp2_dn7) * 0.3333333333333)))))) / (assign28250_e30264 * assign28250_e30264))), (-((1.80485e-35 * (((-locals.var_temp2_dn8) * assign28250_e30262) + (assign28250_e30245 * (((0.5 * (-locals.var_temp2_dn8)) * assign28250_e30260) + (assign28250_e30252 * ((-locals.var_temp2_dn8) * 0.3333333333333)))))) / (assign28250_e30264 * assign28250_e30264))), (-((1.80485e-35 * (((-locals.var_temp2_dn9) * assign28250_e30262) + (assign28250_e30245 * (((0.5 * (-locals.var_temp2_dn9)) * assign28250_e30260) + (assign28250_e30252 * ((-locals.var_temp2_dn9) * 0.3333333333333)))))) / (assign28250_e30264 * assign28250_e30264))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28250_e30267;
        locals.var_temp_dn4 = assign28250_e30267_d_n4;
        locals.var_temp_dn6 = assign28250_e30267_d_n6;
        locals.var_temp_dn7 = assign28250_e30267_d_n7;
        locals.var_temp_dn8 = assign28250_e30267_d_n8;
        locals.var_temp_dn9 = assign28250_e30267_d_n9;

        let (assign28260_e30301, assign28260_e30301_d_n4, assign28260_e30301_d_n6, assign28260_e30301_d_n7, assign28260_e30301_d_n8, assign28260_e30301_d_n9,) = {
    if ((((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) && (locals.var_guard792 == 0.0)) && (locals.var_guard793 == 0.0)) {
        let assign28260_e30281: f64 = (locals.var_temp2 - 80.0);
        let assign28260_e30286: f64 = (locals.var_temp2 - 80.0);
        let assign28260_e30287: f64 = (0.5 * assign28260_e30286);
        let assign28260_e30291: f64 = (locals.var_temp2 - 80.0);
        let assign28260_e30293: f64 = (assign28260_e30291 * 0.3333333333333);
        let assign28260_e30294: f64 = (1.0 + assign28260_e30293);
        let assign28260_e30295: f64 = (assign28260_e30287 * assign28260_e30294);
        let assign28260_e30296: f64 = (1.0 + assign28260_e30295);
        let assign28260_e30297: f64 = (assign28260_e30281 * assign28260_e30296);
        let assign28260_e30298: f64 = (1.0 + assign28260_e30297);
        let assign28260_e30299: f64 = (5.54062e34 * assign28260_e30298);
        (assign28260_e30299, (5.54062e34 * ((locals.var_temp2_dn4 * assign28260_e30296) + (assign28260_e30281 * (((0.5 * locals.var_temp2_dn4) * assign28260_e30294) + (assign28260_e30287 * (locals.var_temp2_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn6 * assign28260_e30296) + (assign28260_e30281 * (((0.5 * locals.var_temp2_dn6) * assign28260_e30294) + (assign28260_e30287 * (locals.var_temp2_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn7 * assign28260_e30296) + (assign28260_e30281 * (((0.5 * locals.var_temp2_dn7) * assign28260_e30294) + (assign28260_e30287 * (locals.var_temp2_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn8 * assign28260_e30296) + (assign28260_e30281 * (((0.5 * locals.var_temp2_dn8) * assign28260_e30294) + (assign28260_e30287 * (locals.var_temp2_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn9 * assign28260_e30296) + (assign28260_e30281 * (((0.5 * locals.var_temp2_dn9) * assign28260_e30294) + (assign28260_e30287 * (locals.var_temp2_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28260_e30301;
        locals.var_temp_dn4 = assign28260_e30301_d_n4;
        locals.var_temp_dn6 = assign28260_e30301_d_n6;
        locals.var_temp_dn7 = assign28260_e30301_d_n7;
        locals.var_temp_dn8 = assign28260_e30301_d_n8;
        locals.var_temp_dn9 = assign28260_e30301_d_n9;

        let (assign28270_e30311, assign28270_e30311_d_n4, assign28270_e30311_d_n6, assign28270_e30311_d_n7, assign28270_e30311_d_n8, assign28270_e30311_d_n9,) = {
    if ((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) {
        let assign28270_e30307: f64 = (locals.var_a1_i * locals.var_delvsat);
        let assign28270_e30309: f64 = (assign28270_e30307 * locals.var_temp);
        (assign28270_e30309, (((locals.var_a1_i * locals.var_delvsat_dn4) * locals.var_temp) + (assign28270_e30307 * locals.var_temp_dn4)), (((locals.var_a1_i * locals.var_delvsat_dn6) * locals.var_temp) + (assign28270_e30307 * locals.var_temp_dn6)), (((locals.var_a1_i * locals.var_delvsat_dn7) * locals.var_temp) + (assign28270_e30307 * locals.var_temp_dn7)), (((locals.var_a1_i * locals.var_delvsat_dn8) * locals.var_temp) + (assign28270_e30307 * locals.var_temp_dn8)), (((locals.var_a1_i * locals.var_delvsat_dn9) * locals.var_temp) + (assign28270_e30307 * locals.var_temp_dn9)),)
    } else {
        (locals.var_mavl, locals.var_mavl_dn4, locals.var_mavl_dn6, locals.var_mavl_dn7, locals.var_mavl_dn8, locals.var_mavl_dn9,)
    }
};
        locals.var_mavl = assign28270_e30311;
        locals.var_mavl_dn4 = assign28270_e30311_d_n4;
        locals.var_mavl_dn6 = assign28270_e30311_d_n6;
        locals.var_mavl_dn7 = assign28270_e30311_d_n7;
        locals.var_mavl_dn8 = assign28270_e30311_d_n8;
        locals.var_mavl_dn9 = assign28270_e30311_d_n9;

        let (assign28280_e30321, assign28280_e30321_d_n4, assign28280_e30321_d_n6, assign28280_e30321_d_n7, assign28280_e30321_d_n8, assign28280_e30321_d_n9,) = {
    if ((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) {
        let assign28280_e30318: f64 = (locals.var_ids + locals.var_ids_edge);
        let assign28280_e30319: f64 = (locals.var_mavl * assign28280_e30318);
        (assign28280_e30319, ((locals.var_mavl_dn4 * assign28280_e30318) + (locals.var_mavl * (locals.var_ids_dn4 + locals.var_ids_edge_dn4))), ((locals.var_mavl_dn6 * assign28280_e30318) + (locals.var_mavl * (locals.var_ids_dn6 + locals.var_ids_edge_dn6))), ((locals.var_mavl_dn7 * assign28280_e30318) + (locals.var_mavl * (locals.var_ids_dn7 + locals.var_ids_edge_dn7))), ((locals.var_mavl_dn8 * assign28280_e30318) + (locals.var_mavl * (locals.var_ids_dn8 + locals.var_ids_edge_dn8))), ((locals.var_mavl_dn9 * assign28280_e30318) + (locals.var_mavl * (locals.var_ids_dn9 + locals.var_ids_edge_dn9))),)
    } else {
        (locals.var_iimpact, locals.var_iimpact_dn4, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, locals.var_iimpact_dn9,)
    }
};
        locals.var_iimpact = assign28280_e30321;
        locals.var_iimpact_dn4 = assign28280_e30321_d_n4;
        locals.var_iimpact_dn6 = assign28280_e30321_d_n6;
        locals.var_iimpact_dn7 = assign28280_e30321_d_n7;
        locals.var_iimpact_dn8 = assign28280_e30321_d_n8;
        locals.var_iimpact_dn9 = assign28280_e30321_d_n9;

        let assign28290_e30324: f64 = if locals.var_swshe_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard794 = assign28290_e30324;

        let (assign28300_e30335, assign28300_e30335_d_n4, assign28300_e30335_d_n6, assign28300_e30335_d_n7, assign28300_e30335_d_n8, assign28300_e30335_d_n9,) = {
    if (locals.var_guard794 != 0.0) {
        let assign28300_e30328: f64 = (locals.var_ids + locals.var_ids_edge);
        let assign28300_e30330: f64 = (assign28300_e30328 * locals.var_vds);
        let assign28300_e30331: f64 = (assign28300_e30330).abs();
        let assign28300_e30333: f64 = (assign28300_e30331 * locals.var_rth_i);
        (assign28300_e30333, ((if assign28300_e30330 >= 0.0 { ((locals.var_ids_dn4 + locals.var_ids_edge_dn4) * locals.var_vds) } else { (-((locals.var_ids_dn4 + locals.var_ids_edge_dn4) * locals.var_vds)) } * locals.var_rth_i) + (assign28300_e30331 * locals.var_rth_i_dn4)), ((if assign28300_e30330 >= 0.0 { (((locals.var_ids_dn6 + locals.var_ids_edge_dn6) * locals.var_vds) + (assign28300_e30328 * locals.var_vds_dn6)) } else { (-(((locals.var_ids_dn6 + locals.var_ids_edge_dn6) * locals.var_vds) + (assign28300_e30328 * locals.var_vds_dn6))) } * locals.var_rth_i) + (assign28300_e30331 * locals.var_rth_i_dn6)), ((if assign28300_e30330 >= 0.0 { (((locals.var_ids_dn7 + locals.var_ids_edge_dn7) * locals.var_vds) + (assign28300_e30328 * locals.var_vds_dn7)) } else { (-(((locals.var_ids_dn7 + locals.var_ids_edge_dn7) * locals.var_vds) + (assign28300_e30328 * locals.var_vds_dn7))) } * locals.var_rth_i) + (assign28300_e30331 * locals.var_rth_i_dn7)), ((if assign28300_e30330 >= 0.0 { ((locals.var_ids_dn8 + locals.var_ids_edge_dn8) * locals.var_vds) } else { (-((locals.var_ids_dn8 + locals.var_ids_edge_dn8) * locals.var_vds)) } * locals.var_rth_i) + (assign28300_e30331 * locals.var_rth_i_dn8)), ((if assign28300_e30330 >= 0.0 { ((locals.var_ids_dn9 + locals.var_ids_edge_dn9) * locals.var_vds) } else { (-((locals.var_ids_dn9 + locals.var_ids_edge_dn9) * locals.var_vds)) } * locals.var_rth_i) + (assign28300_e30331 * locals.var_rth_i_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28300_e30335;
        locals.var_temp_dn4 = assign28300_e30335_d_n4;
        locals.var_temp_dn6 = assign28300_e30335_d_n6;
        locals.var_temp_dn7 = assign28300_e30335_d_n7;
        locals.var_temp_dn8 = assign28300_e30335_d_n8;
        locals.var_temp_dn9 = assign28300_e30335_d_n9;

        let assign28310_e30339: f64 = (100000000.0 * p.p16);
        let assign28310_e30340: f64 = if locals.var_temp > assign28310_e30339 { 1.0 } else { 0.0 };
        locals.var_guard795 = assign28310_e30340;

        let (assign28320_e30353, assign28320_e30353_d_n4, assign28320_e30353_d_n6, assign28320_e30353_d_n7, assign28320_e30353_d_n8, assign28320_e30353_d_n9,) = {
    if ((locals.var_guard794 != 0.0) && (locals.var_guard795 != 0.0)) {
        let assign28320_e30347: f64 = (0.25 / p.p16);
        let assign28320_e30348: f64 = (p.p16 + assign28320_e30347);
        let assign28320_e30349: f64 = (-assign28320_e30348);
        let assign28320_e30351: f64 = (assign28320_e30349 / locals.var_rth_i);
        (assign28320_e30351, (-((assign28320_e30349 * locals.var_rth_i_dn4) / (locals.var_rth_i * locals.var_rth_i))), (-((assign28320_e30349 * locals.var_rth_i_dn6) / (locals.var_rth_i * locals.var_rth_i))), (-((assign28320_e30349 * locals.var_rth_i_dn7) / (locals.var_rth_i * locals.var_rth_i))), (-((assign28320_e30349 * locals.var_rth_i_dn8) / (locals.var_rth_i * locals.var_rth_i))), (-((assign28320_e30349 * locals.var_rth_i_dn9) / (locals.var_rth_i * locals.var_rth_i))),)
    } else {
        (locals.var_ithpwr, locals.var_ithpwr_dn4, locals.var_ithpwr_dn6, locals.var_ithpwr_dn7, locals.var_ithpwr_dn8, locals.var_ithpwr_dn9,)
    }
};
        locals.var_ithpwr = assign28320_e30353;
        locals.var_ithpwr_dn4 = assign28320_e30353_d_n4;
        locals.var_ithpwr_dn6 = assign28320_e30353_d_n6;
        locals.var_ithpwr_dn7 = assign28320_e30353_d_n7;
        locals.var_ithpwr_dn8 = assign28320_e30353_d_n8;
        locals.var_ithpwr_dn9 = assign28320_e30353_d_n9;

        let (assign28330_e30384, assign28330_e30384_d_n4, assign28330_e30384_d_n6, assign28330_e30384_d_n7, assign28330_e30384_d_n8, assign28330_e30384_d_n9,) = {
    if ((locals.var_guard794 != 0.0) && (locals.var_guard795 == 0.0)) {
        let assign28330_e30360: f64 = 0.5;
        let assign28330_e30363: f64 = (locals.var_temp + p.p16);
        let assign28330_e30366: f64 = (locals.var_temp - p.p16);
        let assign28330_e30369: f64 = (locals.var_temp - p.p16);
        let assign28330_e30370: f64 = (assign28330_e30366 * assign28330_e30369);
        let assign28330_e30372: f64 = (assign28330_e30370 + 1.0);
        let assign28330_e30373: f64 = (assign28330_e30372).sqrt();
        let assign28330_e30374: f64 = (assign28330_e30363 - assign28330_e30373);
        let assign28330_e30375: f64 = (assign28330_e30360 * assign28330_e30374);
        let assign28330_e30378: f64 = (0.25 / p.p16);
        let assign28330_e30379: f64 = (assign28330_e30375 + assign28330_e30378);
        let assign28330_e30380: f64 = (-assign28330_e30379);
        let assign28330_e30382: f64 = (assign28330_e30380 / locals.var_rth_i);
        (assign28330_e30382, ((((-(assign28330_e30360 * (locals.var_temp_dn4 - (((locals.var_temp_dn4 * assign28330_e30369) + (assign28330_e30366 * locals.var_temp_dn4)) / (2.0 * assign28330_e30373))))) * locals.var_rth_i) - (assign28330_e30380 * locals.var_rth_i_dn4)) / (locals.var_rth_i * locals.var_rth_i)), ((((-(assign28330_e30360 * (locals.var_temp_dn6 - (((locals.var_temp_dn6 * assign28330_e30369) + (assign28330_e30366 * locals.var_temp_dn6)) / (2.0 * assign28330_e30373))))) * locals.var_rth_i) - (assign28330_e30380 * locals.var_rth_i_dn6)) / (locals.var_rth_i * locals.var_rth_i)), ((((-(assign28330_e30360 * (locals.var_temp_dn7 - (((locals.var_temp_dn7 * assign28330_e30369) + (assign28330_e30366 * locals.var_temp_dn7)) / (2.0 * assign28330_e30373))))) * locals.var_rth_i) - (assign28330_e30380 * locals.var_rth_i_dn7)) / (locals.var_rth_i * locals.var_rth_i)), ((((-(assign28330_e30360 * (locals.var_temp_dn8 - (((locals.var_temp_dn8 * assign28330_e30369) + (assign28330_e30366 * locals.var_temp_dn8)) / (2.0 * assign28330_e30373))))) * locals.var_rth_i) - (assign28330_e30380 * locals.var_rth_i_dn8)) / (locals.var_rth_i * locals.var_rth_i)), ((((-(assign28330_e30360 * (locals.var_temp_dn9 - (((locals.var_temp_dn9 * assign28330_e30369) + (assign28330_e30366 * locals.var_temp_dn9)) / (2.0 * assign28330_e30373))))) * locals.var_rth_i) - (assign28330_e30380 * locals.var_rth_i_dn9)) / (locals.var_rth_i * locals.var_rth_i)),)
    } else {
        (locals.var_ithpwr, locals.var_ithpwr_dn4, locals.var_ithpwr_dn6, locals.var_ithpwr_dn7, locals.var_ithpwr_dn8, locals.var_ithpwr_dn9,)
    }
};
        locals.var_ithpwr = assign28330_e30384;
        locals.var_ithpwr_dn4 = assign28330_e30384_d_n4;
        locals.var_ithpwr_dn6 = assign28330_e30384_d_n6;
        locals.var_ithpwr_dn7 = assign28330_e30384_d_n7;
        locals.var_ithpwr_dn8 = assign28330_e30384_d_n8;
        locals.var_ithpwr_dn9 = assign28330_e30384_d_n9;

        let (assign28350_e30395, assign28350_e30395_d_n4, assign28350_e30395_d_n6, assign28350_e30395_d_n7, assign28350_e30395_d_n8, assign28350_e30395_d_n9,) = {
    if (locals.var_guard794 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ithpwr, locals.var_ithpwr_dn4, locals.var_ithpwr_dn6, locals.var_ithpwr_dn7, locals.var_ithpwr_dn8, locals.var_ithpwr_dn9,)
    }
};
        locals.var_ithpwr = assign28350_e30395;
        locals.var_ithpwr_dn4 = assign28350_e30395_d_n4;
        locals.var_ithpwr_dn6 = assign28350_e30395_d_n6;
        locals.var_ithpwr_dn7 = assign28350_e30395_d_n7;
        locals.var_ithpwr_dn8 = assign28350_e30395_d_n8;
        locals.var_ithpwr_dn9 = assign28350_e30395_d_n9;

        let assign28410_e30417: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign28410_e30417;

        let (assign28420_e30421, assign28420_e30421_d_n4, assign28420_e30421_d_n6, assign28420_e30421_d_n7, assign28420_e30421_d_n8, assign28420_e30421_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_vfbac1_i, locals.var_vfbac1_i_dn4, locals.var_vfbac1_i_dn6, locals.var_vfbac1_i_dn7, locals.var_vfbac1_i_dn8, locals.var_vfbac1_i_dn9,)
    } else {
        (locals.var_vfb1_loc__blk890, locals.var_vfb1_loc__blk890_dn4, locals.var_vfb1_loc__blk890_dn6, locals.var_vfb1_loc__blk890_dn7, locals.var_vfb1_loc__blk890_dn8, locals.var_vfb1_loc__blk890_dn9,)
    }
};
        locals.var_vfb1_loc__blk890 = assign28420_e30421;
        locals.var_vfb1_loc__blk890_dn4 = assign28420_e30421_d_n4;
        locals.var_vfb1_loc__blk890_dn6 = assign28420_e30421_d_n6;
        locals.var_vfb1_loc__blk890_dn7 = assign28420_e30421_d_n7;
        locals.var_vfb1_loc__blk890_dn8 = assign28420_e30421_d_n8;
        locals.var_vfb1_loc__blk890_dn9 = assign28420_e30421_d_n9;

        let (assign28430_e30425, assign28430_e30425_d_n4, assign28430_e30425_d_n6, assign28430_e30425_d_n7, assign28430_e30425_d_n8, assign28430_e30425_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_vfbac2_i, locals.var_vfbac2_i_dn4, locals.var_vfbac2_i_dn6, locals.var_vfbac2_i_dn7, locals.var_vfbac2_i_dn8, locals.var_vfbac2_i_dn9,)
    } else {
        (locals.var_vfb2_loc__blk891, locals.var_vfb2_loc__blk891_dn4, locals.var_vfb2_loc__blk891_dn6, locals.var_vfb2_loc__blk891_dn7, locals.var_vfb2_loc__blk891_dn8, locals.var_vfb2_loc__blk891_dn9,)
    }
};
        locals.var_vfb2_loc__blk891 = assign28430_e30425;
        locals.var_vfb2_loc__blk891_dn4 = assign28430_e30425_d_n4;
        locals.var_vfb2_loc__blk891_dn6 = assign28430_e30425_d_n6;
        locals.var_vfb2_loc__blk891_dn7 = assign28430_e30425_d_n7;
        locals.var_vfb2_loc__blk891_dn8 = assign28430_e30425_d_n8;
        locals.var_vfb2_loc__blk891_dn9 = assign28430_e30425_d_n9;

        let (assign28440_e30429,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_psceac1_i,)
    } else {
        (locals.var_psce1_loc__blk892,)
    }
};
        locals.var_psce1_loc__blk892 = assign28440_e30429;

        let (assign28450_e30433,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_psceac2_i,)
    } else {
        (locals.var_psce2_loc__blk893,)
    }
};
        locals.var_psce2_loc__blk893 = assign28450_e30433;

        let (assign28460_e30437, assign28460_e30437_d_n4, assign28460_e30437_d_n6, assign28460_e30437_d_n7, assign28460_e30437_d_n8, assign28460_e30437_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_cfac1_i, locals.var_cfac1_i_dn4, locals.var_cfac1_i_dn6, locals.var_cfac1_i_dn7, locals.var_cfac1_i_dn8, locals.var_cfac1_i_dn9,)
    } else {
        (locals.var_cf1_loc__blk894, locals.var_cf1_loc__blk894_dn4, locals.var_cf1_loc__blk894_dn6, locals.var_cf1_loc__blk894_dn7, locals.var_cf1_loc__blk894_dn8, locals.var_cf1_loc__blk894_dn9,)
    }
};
        locals.var_cf1_loc__blk894 = assign28460_e30437;
        locals.var_cf1_loc__blk894_dn4 = assign28460_e30437_d_n4;
        locals.var_cf1_loc__blk894_dn6 = assign28460_e30437_d_n6;
        locals.var_cf1_loc__blk894_dn7 = assign28460_e30437_d_n7;
        locals.var_cf1_loc__blk894_dn8 = assign28460_e30437_d_n8;
        locals.var_cf1_loc__blk894_dn9 = assign28460_e30437_d_n9;

        let (assign28470_e30441, assign28470_e30441_d_n4, assign28470_e30441_d_n6, assign28470_e30441_d_n7, assign28470_e30441_d_n8, assign28470_e30441_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_cfac2_i, locals.var_cfac2_i_dn4, locals.var_cfac2_i_dn6, locals.var_cfac2_i_dn7, locals.var_cfac2_i_dn8, locals.var_cfac2_i_dn9,)
    } else {
        (locals.var_cf2_loc__blk895, locals.var_cf2_loc__blk895_dn4, locals.var_cf2_loc__blk895_dn6, locals.var_cf2_loc__blk895_dn7, locals.var_cf2_loc__blk895_dn8, locals.var_cf2_loc__blk895_dn9,)
    }
};
        locals.var_cf2_loc__blk895 = assign28470_e30441;
        locals.var_cf2_loc__blk895_dn4 = assign28470_e30441_d_n4;
        locals.var_cf2_loc__blk895_dn6 = assign28470_e30441_d_n6;
        locals.var_cf2_loc__blk895_dn7 = assign28470_e30441_d_n7;
        locals.var_cf2_loc__blk895_dn8 = assign28470_e30441_d_n8;
        locals.var_cf2_loc__blk895_dn9 = assign28470_e30441_d_n9;

        let (assign28480_e30445, assign28480_e30445_d_n4, assign28480_e30445_d_n6, assign28480_e30445_d_n7, assign28480_e30445_d_n8, assign28480_e30445_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_sat_phit_ac, locals.var_sat_phit_ac_dn4, locals.var_sat_phit_ac_dn6, locals.var_sat_phit_ac_dn7, locals.var_sat_phit_ac_dn8, locals.var_sat_phit_ac_dn9,)
    } else {
        (locals.var_sat_phit_loc__blk896, locals.var_sat_phit_loc__blk896_dn4, locals.var_sat_phit_loc__blk896_dn6, locals.var_sat_phit_loc__blk896_dn7, locals.var_sat_phit_loc__blk896_dn8, locals.var_sat_phit_loc__blk896_dn9,)
    }
};
        locals.var_sat_phit_loc__blk896 = assign28480_e30445;
        locals.var_sat_phit_loc__blk896_dn4 = assign28480_e30445_d_n4;
        locals.var_sat_phit_loc__blk896_dn6 = assign28480_e30445_d_n6;
        locals.var_sat_phit_loc__blk896_dn7 = assign28480_e30445_d_n7;
        locals.var_sat_phit_loc__blk896_dn8 = assign28480_e30445_d_n8;
        locals.var_sat_phit_loc__blk896_dn9 = assign28480_e30445_d_n9;

        let (assign28490_e30449,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_gamax_ac,)
    } else {
        (locals.var_gamax_loc__blk897,)
    }
};
        locals.var_gamax_loc__blk897 = assign28490_e30449;

    }

    pub(super) fn stamp_transient_block_72(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28500_e30453,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_alpac_i,)
    } else {
        (locals.var_alp_loc__blk898,)
    }
};
        locals.var_alp_loc__blk898 = assign28500_e30453;

        let (assign28510_e30465, assign28510_e30465_d_n4, assign28510_e30465_d_n6, assign28510_e30465_d_n7, assign28510_e30465_d_n8, assign28510_e30465_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign28510_e30457: f64 = (locals.var_vgs - locals.var_vfb1_loc__blk890);
        let assign28510_e30459: f64 = (assign28510_e30457 * locals.var_inv_phit);
        let assign28510_e30461: f64 = (assign28510_e30459 - locals.var_dxdsx);
        let assign28510_e30463: f64 = (assign28510_e30461 - locals.var_eg_2phit0);
        (assign28510_e30463, (((((-locals.var_vfb1_loc__blk890_dn4) * locals.var_inv_phit) + (assign28510_e30457 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4) - locals.var_eg_2phit0_dn4), (((((locals.var_vgs_dn6 - locals.var_vfb1_loc__blk890_dn6) * locals.var_inv_phit) + (assign28510_e30457 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6) - locals.var_eg_2phit0_dn6), (((((locals.var_vgs_dn7 - locals.var_vfb1_loc__blk890_dn7) * locals.var_inv_phit) + (assign28510_e30457 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7) - locals.var_eg_2phit0_dn7), (((((-locals.var_vfb1_loc__blk890_dn8) * locals.var_inv_phit) + (assign28510_e30457 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8) - locals.var_eg_2phit0_dn8), (((((locals.var_vgs_dn9 - locals.var_vfb1_loc__blk890_dn9) * locals.var_inv_phit) + (assign28510_e30457 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9) - locals.var_eg_2phit0_dn9),)
    } else {
        (locals.var_xg10__blk899, locals.var_xg10__blk899_dn4, locals.var_xg10__blk899_dn6, locals.var_xg10__blk899_dn7, locals.var_xg10__blk899_dn8, locals.var_xg10__blk899_dn9,)
    }
};
        locals.var_xg10__blk899 = assign28510_e30465;
        locals.var_xg10__blk899_dn4 = assign28510_e30465_d_n4;
        locals.var_xg10__blk899_dn6 = assign28510_e30465_d_n6;
        locals.var_xg10__blk899_dn7 = assign28510_e30465_d_n7;
        locals.var_xg10__blk899_dn8 = assign28510_e30465_d_n8;
        locals.var_xg10__blk899_dn9 = assign28510_e30465_d_n9;

        let (assign28520_e30476, assign28520_e30476_d_n4, assign28520_e30476_d_n6, assign28520_e30476_d_n7, assign28520_e30476_d_n8, assign28520_e30476_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign28520_e30468: f64 = (-locals.var_vsb);
        let assign28520_e30470: f64 = (assign28520_e30468 - locals.var_vfb2_loc__blk891);
        let assign28520_e30472: f64 = (assign28520_e30470 * locals.var_inv_phit);
        let assign28520_e30474: f64 = (assign28520_e30472 - locals.var_dxdsx);
        (assign28520_e30474, ((((-locals.var_vfb2_loc__blk891_dn4) * locals.var_inv_phit) + (assign28520_e30470 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4), (((((-locals.var_vsb_dn6) - locals.var_vfb2_loc__blk891_dn6) * locals.var_inv_phit) + (assign28520_e30470 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6), (((((-locals.var_vsb_dn7) - locals.var_vfb2_loc__blk891_dn7) * locals.var_inv_phit) + (assign28520_e30470 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7), (((((-locals.var_vsb_dn8) - locals.var_vfb2_loc__blk891_dn8) * locals.var_inv_phit) + (assign28520_e30470 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8), ((((-locals.var_vfb2_loc__blk891_dn9) * locals.var_inv_phit) + (assign28520_e30470 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9),)
    } else {
        (locals.var_xg20shift__blk900, locals.var_xg20shift__blk900_dn4, locals.var_xg20shift__blk900_dn6, locals.var_xg20shift__blk900_dn7, locals.var_xg20shift__blk900_dn8, locals.var_xg20shift__blk900_dn9,)
    }
};
        locals.var_xg20shift__blk900 = assign28520_e30476;
        locals.var_xg20shift__blk900_dn4 = assign28520_e30476_d_n4;
        locals.var_xg20shift__blk900_dn6 = assign28520_e30476_d_n6;
        locals.var_xg20shift__blk900_dn7 = assign28520_e30476_d_n7;
        locals.var_xg20shift__blk900_dn8 = assign28520_e30476_d_n8;
        locals.var_xg20shift__blk900_dn9 = assign28520_e30476_d_n9;

        let (assign28530_e30482, assign28530_e30482_d_n4, assign28530_e30482_d_n6, assign28530_e30482_d_n7, assign28530_e30482_d_n8, assign28530_e30482_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign28530_e30480: f64 = (locals.var_xg20shift__blk900 - locals.var_eg_2phit0);
        (assign28530_e30480, (locals.var_xg20shift__blk900_dn4 - locals.var_eg_2phit0_dn4), (locals.var_xg20shift__blk900_dn6 - locals.var_eg_2phit0_dn6), (locals.var_xg20shift__blk900_dn7 - locals.var_eg_2phit0_dn7), (locals.var_xg20shift__blk900_dn8 - locals.var_eg_2phit0_dn8), (locals.var_xg20shift__blk900_dn9 - locals.var_eg_2phit0_dn9),)
    } else {
        (locals.var_xg20__blk901, locals.var_xg20__blk901_dn4, locals.var_xg20__blk901_dn6, locals.var_xg20__blk901_dn7, locals.var_xg20__blk901_dn8, locals.var_xg20__blk901_dn9,)
    }
};
        locals.var_xg20__blk901 = assign28530_e30482;
        locals.var_xg20__blk901_dn4 = assign28530_e30482_d_n4;
        locals.var_xg20__blk901_dn6 = assign28530_e30482_d_n6;
        locals.var_xg20__blk901_dn7 = assign28530_e30482_d_n7;
        locals.var_xg20__blk901_dn8 = assign28530_e30482_d_n8;
        locals.var_xg20__blk901_dn9 = assign28530_e30482_d_n9;

        let assign28540_e30485: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign28540_e30485;

        let (assign28550_e30493, assign28550_e30493_d_n4, assign28550_e30493_d_n6, assign28550_e30493_d_n7, assign28550_e30493_d_n8, assign28550_e30493_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28550_e30491: f64 = (p.p14 * locals.var_typesub_i);
        (assign28550_e30491, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28550_e30493;
        locals.var_temp_dn4 = assign28550_e30493_d_n4;
        locals.var_temp_dn6 = assign28550_e30493_d_n6;
        locals.var_temp_dn7 = assign28550_e30493_d_n7;
        locals.var_temp_dn8 = assign28550_e30493_d_n8;
        locals.var_temp_dn9 = assign28550_e30493_d_n9;

        let (assign28560_e30505, assign28560_e30505_d_n4, assign28560_e30505_d_n6, assign28560_e30505_d_n7, assign28560_e30505_d_n8, assign28560_e30505_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28560_e30499: f64 = (1.0 + locals.var_k1_1d);
        let assign28560_e30502: f64 = (1.0 + locals.var_k2_1d);
        let assign28560_e30503: f64 = (assign28560_e30499 / assign28560_e30502);
        (assign28560_e30503, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_dxth__blk902, locals.var_exp_dxth__blk902_dn4, locals.var_exp_dxth__blk902_dn6, locals.var_exp_dxth__blk902_dn7, locals.var_exp_dxth__blk902_dn8, locals.var_exp_dxth__blk902_dn9,)
    }
};
        locals.var_exp_dxth__blk902 = assign28560_e30505;
        locals.var_exp_dxth__blk902_dn4 = assign28560_e30505_d_n4;
        locals.var_exp_dxth__blk902_dn6 = assign28560_e30505_d_n6;
        locals.var_exp_dxth__blk902_dn7 = assign28560_e30505_d_n7;
        locals.var_exp_dxth__blk902_dn8 = assign28560_e30505_d_n8;
        locals.var_exp_dxth__blk902_dn9 = assign28560_e30505_d_n9;

        let (assign28570_e30512, assign28570_e30512_d_n4, assign28570_e30512_d_n6, assign28570_e30512_d_n7, assign28570_e30512_d_n8, assign28570_e30512_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28570_e30510: f64 = (locals.var_exp_dxth__blk902).ln();
        (assign28570_e30510, (locals.var_exp_dxth__blk902_dn4 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn6 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn7 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn8 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn9 / locals.var_exp_dxth__blk902),)
    } else {
        (locals.var_dxth__blk903, locals.var_dxth__blk903_dn4, locals.var_dxth__blk903_dn6, locals.var_dxth__blk903_dn7, locals.var_dxth__blk903_dn8, locals.var_dxth__blk903_dn9,)
    }
};
        locals.var_dxth__blk903 = assign28570_e30512;
        locals.var_dxth__blk903_dn4 = assign28570_e30512_d_n4;
        locals.var_dxth__blk903_dn6 = assign28570_e30512_d_n6;
        locals.var_dxth__blk903_dn7 = assign28570_e30512_d_n7;
        locals.var_dxth__blk903_dn8 = assign28570_e30512_d_n8;
        locals.var_dxth__blk903_dn9 = assign28570_e30512_d_n9;

        let assign28580_e30515: f64 = if locals.var_dxth__blk903 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign28580_e30515;

        let (assign28590_e30533, assign28590_e30533_d_n4, assign28590_e30533_d_n6, assign28590_e30533_d_n7, assign28590_e30533_d_n8, assign28590_e30533_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
        let assign28590_e30523: f64 = (2.0 * locals.var_dxth__blk903);
        let assign28590_e30526: f64 = (locals.var_exp_dxth__blk902 + 1.0);
        let assign28590_e30527: f64 = (assign28590_e30523 * assign28590_e30526);
        let assign28590_e30530: f64 = (locals.var_exp_dxth__blk902 - 1.0);
        let assign28590_e30531: f64 = (assign28590_e30527 / assign28590_e30530);
        (assign28590_e30531, ((((((2.0 * locals.var_dxth__blk903_dn4) * assign28590_e30526) + (assign28590_e30523 * locals.var_exp_dxth__blk902_dn4)) * assign28590_e30530) - (assign28590_e30527 * locals.var_exp_dxth__blk902_dn4)) / (assign28590_e30530 * assign28590_e30530)), ((((((2.0 * locals.var_dxth__blk903_dn6) * assign28590_e30526) + (assign28590_e30523 * locals.var_exp_dxth__blk902_dn6)) * assign28590_e30530) - (assign28590_e30527 * locals.var_exp_dxth__blk902_dn6)) / (assign28590_e30530 * assign28590_e30530)), ((((((2.0 * locals.var_dxth__blk903_dn7) * assign28590_e30526) + (assign28590_e30523 * locals.var_exp_dxth__blk902_dn7)) * assign28590_e30530) - (assign28590_e30527 * locals.var_exp_dxth__blk902_dn7)) / (assign28590_e30530 * assign28590_e30530)), ((((((2.0 * locals.var_dxth__blk903_dn8) * assign28590_e30526) + (assign28590_e30523 * locals.var_exp_dxth__blk902_dn8)) * assign28590_e30530) - (assign28590_e30527 * locals.var_exp_dxth__blk902_dn8)) / (assign28590_e30530 * assign28590_e30530)), ((((((2.0 * locals.var_dxth__blk903_dn9) * assign28590_e30526) + (assign28590_e30523 * locals.var_exp_dxth__blk902_dn9)) * assign28590_e30530) - (assign28590_e30527 * locals.var_exp_dxth__blk902_dn9)) / (assign28590_e30530 * assign28590_e30530)),)
    } else {
        (locals.var_diff_min__blk904, locals.var_diff_min__blk904_dn4, locals.var_diff_min__blk904_dn6, locals.var_diff_min__blk904_dn7, locals.var_diff_min__blk904_dn8, locals.var_diff_min__blk904_dn9,)
    }
};
        locals.var_diff_min__blk904 = assign28590_e30533;
        locals.var_diff_min__blk904_dn4 = assign28590_e30533_d_n4;
        locals.var_diff_min__blk904_dn6 = assign28590_e30533_d_n6;
        locals.var_diff_min__blk904_dn7 = assign28590_e30533_d_n7;
        locals.var_diff_min__blk904_dn8 = assign28590_e30533_d_n8;
        locals.var_diff_min__blk904_dn9 = assign28590_e30533_d_n9;

        let (assign28600_e30546, assign28600_e30546_d_n4, assign28600_e30546_d_n6, assign28600_e30546_d_n7, assign28600_e30546_d_n8, assign28600_e30546_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 == 0.0)) {
        let assign28600_e30543: f64 = (2.0 + locals.var_dxth__blk903);
        let assign28600_e30544: f64 = (2.0 * assign28600_e30543);
        (assign28600_e30544, (2.0 * locals.var_dxth__blk903_dn4), (2.0 * locals.var_dxth__blk903_dn6), (2.0 * locals.var_dxth__blk903_dn7), (2.0 * locals.var_dxth__blk903_dn8), (2.0 * locals.var_dxth__blk903_dn9),)
    } else {
        (locals.var_diff_min__blk904, locals.var_diff_min__blk904_dn4, locals.var_diff_min__blk904_dn6, locals.var_diff_min__blk904_dn7, locals.var_diff_min__blk904_dn8, locals.var_diff_min__blk904_dn9,)
    }
};
        locals.var_diff_min__blk904 = assign28600_e30546;
        locals.var_diff_min__blk904_dn4 = assign28600_e30546_d_n4;
        locals.var_diff_min__blk904_dn6 = assign28600_e30546_d_n6;
        locals.var_diff_min__blk904_dn7 = assign28600_e30546_d_n7;
        locals.var_diff_min__blk904_dn8 = assign28600_e30546_d_n8;
        locals.var_diff_min__blk904_dn9 = assign28600_e30546_d_n9;

        let (assign28610_e30556, assign28610_e30556_d_n4, assign28610_e30556_d_n6, assign28610_e30556_d_n7, assign28610_e30556_d_n8, assign28610_e30556_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28610_e30553: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign28610_e30554: f64 = (locals.var_a0_csisq / assign28610_e30553);
        (assign28610_e30554, (locals.var_a0_csisq_dn4 / assign28610_e30553), (locals.var_a0_csisq_dn6 / assign28610_e30553), (locals.var_a0_csisq_dn7 / assign28610_e30553), (locals.var_a0_csisq_dn8 / assign28610_e30553), (locals.var_a0_csisq_dn9 / assign28610_e30553),)
    } else {
        (locals.var_a0__blk905, locals.var_a0__blk905_dn4, locals.var_a0__blk905_dn6, locals.var_a0__blk905_dn7, locals.var_a0__blk905_dn8, locals.var_a0__blk905_dn9,)
    }
};
        locals.var_a0__blk905 = assign28610_e30556;
        locals.var_a0__blk905_dn4 = assign28610_e30556_d_n4;
        locals.var_a0__blk905_dn6 = assign28610_e30556_d_n6;
        locals.var_a0__blk905_dn7 = assign28610_e30556_d_n7;
        locals.var_a0__blk905_dn8 = assign28610_e30556_d_n8;
        locals.var_a0__blk905_dn9 = assign28610_e30556_d_n9;

        let (assign28620_e30564, assign28620_e30564_d_n4, assign28620_e30564_d_n6, assign28620_e30564_d_n7, assign28620_e30564_d_n8, assign28620_e30564_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28620_e30562: f64 = (1.0 / locals.var_k1_1d);
        (assign28620_e30562, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k1__blk906, locals.var_inv_k1__blk906_dn4, locals.var_inv_k1__blk906_dn6, locals.var_inv_k1__blk906_dn7, locals.var_inv_k1__blk906_dn8, locals.var_inv_k1__blk906_dn9,)
    }
};
        locals.var_inv_k1__blk906 = assign28620_e30564;
        locals.var_inv_k1__blk906_dn4 = assign28620_e30564_d_n4;
        locals.var_inv_k1__blk906_dn6 = assign28620_e30564_d_n6;
        locals.var_inv_k1__blk906_dn7 = assign28620_e30564_d_n7;
        locals.var_inv_k1__blk906_dn8 = assign28620_e30564_d_n8;
        locals.var_inv_k1__blk906_dn9 = assign28620_e30564_d_n9;

        let (assign28630_e30572, assign28630_e30572_d_n4, assign28630_e30572_d_n6, assign28630_e30572_d_n7, assign28630_e30572_d_n8, assign28630_e30572_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28630_e30570: f64 = (1.0 / locals.var_k2_1d);
        (assign28630_e30570, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k2__blk907, locals.var_inv_k2__blk907_dn4, locals.var_inv_k2__blk907_dn6, locals.var_inv_k2__blk907_dn7, locals.var_inv_k2__blk907_dn8, locals.var_inv_k2__blk907_dn9,)
    }
};
        locals.var_inv_k2__blk907 = assign28630_e30572;
        locals.var_inv_k2__blk907_dn4 = assign28630_e30572_d_n4;
        locals.var_inv_k2__blk907_dn6 = assign28630_e30572_d_n6;
        locals.var_inv_k2__blk907_dn7 = assign28630_e30572_d_n7;
        locals.var_inv_k2__blk907_dn8 = assign28630_e30572_d_n8;
        locals.var_inv_k2__blk907_dn9 = assign28630_e30572_d_n9;

        let (assign28640_e30584, assign28640_e30584_d_n4, assign28640_e30584_d_n6, assign28640_e30584_d_n7, assign28640_e30584_d_n8, assign28640_e30584_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28640_e30579: f64 = (1.0 + locals.var_inv_k1__blk906);
        let assign28640_e30581: f64 = (assign28640_e30579 + locals.var_inv_k2__blk907);
        let assign28640_e30582: f64 = (1.0 / assign28640_e30581);
        (assign28640_e30582, (-((locals.var_inv_k1__blk906_dn4 + locals.var_inv_k2__blk907_dn4) / (assign28640_e30581 * assign28640_e30581))), (-((locals.var_inv_k1__blk906_dn6 + locals.var_inv_k2__blk907_dn6) / (assign28640_e30581 * assign28640_e30581))), (-((locals.var_inv_k1__blk906_dn7 + locals.var_inv_k2__blk907_dn7) / (assign28640_e30581 * assign28640_e30581))), (-((locals.var_inv_k1__blk906_dn8 + locals.var_inv_k2__blk907_dn8) / (assign28640_e30581 * assign28640_e30581))), (-((locals.var_inv_k1__blk906_dn9 + locals.var_inv_k2__blk907_dn9) / (assign28640_e30581 * assign28640_e30581))),)
    } else {
        (locals.var_keq__blk934, locals.var_keq__blk934_dn4, locals.var_keq__blk934_dn6, locals.var_keq__blk934_dn7, locals.var_keq__blk934_dn8, locals.var_keq__blk934_dn9,)
    }
};
        locals.var_keq__blk934 = assign28640_e30584;
        locals.var_keq__blk934_dn4 = assign28640_e30584_d_n4;
        locals.var_keq__blk934_dn6 = assign28640_e30584_d_n6;
        locals.var_keq__blk934_dn7 = assign28640_e30584_d_n7;
        locals.var_keq__blk934_dn8 = assign28640_e30584_d_n8;
        locals.var_keq__blk934_dn9 = assign28640_e30584_d_n9;

        let (assign28650_e30594, assign28650_e30594_d_n4, assign28650_e30594_d_n6, assign28650_e30594_d_n7, assign28650_e30594_d_n8, assign28650_e30594_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28650_e30591: f64 = (locals.var_xg10__blk899 - locals.var_xg20__blk901);
        let assign28650_e30592: f64 = (locals.var_keq__blk934 * assign28650_e30591);
        (assign28650_e30592, ((locals.var_keq__blk934_dn4 * assign28650_e30591) + (locals.var_keq__blk934 * (locals.var_xg10__blk899_dn4 - locals.var_xg20__blk901_dn4))), ((locals.var_keq__blk934_dn6 * assign28650_e30591) + (locals.var_keq__blk934 * (locals.var_xg10__blk899_dn6 - locals.var_xg20__blk901_dn6))), ((locals.var_keq__blk934_dn7 * assign28650_e30591) + (locals.var_keq__blk934 * (locals.var_xg10__blk899_dn7 - locals.var_xg20__blk901_dn7))), ((locals.var_keq__blk934_dn8 * assign28650_e30591) + (locals.var_keq__blk934 * (locals.var_xg10__blk899_dn8 - locals.var_xg20__blk901_dn8))), ((locals.var_keq__blk934_dn9 * assign28650_e30591) + (locals.var_keq__blk934 * (locals.var_xg10__blk899_dn9 - locals.var_xg20__blk901_dn9))),)
    } else {
        (locals.var_dx_wi__blk935, locals.var_dx_wi__blk935_dn4, locals.var_dx_wi__blk935_dn6, locals.var_dx_wi__blk935_dn7, locals.var_dx_wi__blk935_dn8, locals.var_dx_wi__blk935_dn9,)
    }
};
        locals.var_dx_wi__blk935 = assign28650_e30594;
        locals.var_dx_wi__blk935_dn4 = assign28650_e30594_d_n4;
        locals.var_dx_wi__blk935_dn6 = assign28650_e30594_d_n6;
        locals.var_dx_wi__blk935_dn7 = assign28650_e30594_d_n7;
        locals.var_dx_wi__blk935_dn8 = assign28650_e30594_d_n8;
        locals.var_dx_wi__blk935_dn9 = assign28650_e30594_d_n9;

        let (assign28660_e30604, assign28660_e30604_d_n4, assign28660_e30604_d_n6, assign28660_e30604_d_n7, assign28660_e30604_d_n8, assign28660_e30604_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28660_e30601: f64 = (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906);
        let assign28660_e30602: f64 = (locals.var_xg10__blk899 - assign28660_e30601);
        (assign28660_e30602, (locals.var_xg10__blk899_dn4 - ((locals.var_dx_wi__blk935_dn4 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn4))), (locals.var_xg10__blk899_dn6 - ((locals.var_dx_wi__blk935_dn6 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn6))), (locals.var_xg10__blk899_dn7 - ((locals.var_dx_wi__blk935_dn7 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn7))), (locals.var_xg10__blk899_dn8 - ((locals.var_dx_wi__blk935_dn8 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn8))), (locals.var_xg10__blk899_dn9 - ((locals.var_dx_wi__blk935_dn9 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn9))),)
    } else {
        (locals.var_x1_wi0__blk908, locals.var_x1_wi0__blk908_dn4, locals.var_x1_wi0__blk908_dn6, locals.var_x1_wi0__blk908_dn7, locals.var_x1_wi0__blk908_dn8, locals.var_x1_wi0__blk908_dn9,)
    }
};
        locals.var_x1_wi0__blk908 = assign28660_e30604;
        locals.var_x1_wi0__blk908_dn4 = assign28660_e30604_d_n4;
        locals.var_x1_wi0__blk908_dn6 = assign28660_e30604_d_n6;
        locals.var_x1_wi0__blk908_dn7 = assign28660_e30604_d_n7;
        locals.var_x1_wi0__blk908_dn8 = assign28660_e30604_d_n8;
        locals.var_x1_wi0__blk908_dn9 = assign28660_e30604_d_n9;

        let (assign28670_e30614, assign28670_e30614_d_n4, assign28670_e30614_d_n6, assign28670_e30614_d_n7, assign28670_e30614_d_n8, assign28670_e30614_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28670_e30611: f64 = (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907);
        let assign28670_e30612: f64 = (locals.var_xg20__blk901 + assign28670_e30611);
        (assign28670_e30612, (locals.var_xg20__blk901_dn4 + ((locals.var_dx_wi__blk935_dn4 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn4))), (locals.var_xg20__blk901_dn6 + ((locals.var_dx_wi__blk935_dn6 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn6))), (locals.var_xg20__blk901_dn7 + ((locals.var_dx_wi__blk935_dn7 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn7))), (locals.var_xg20__blk901_dn8 + ((locals.var_dx_wi__blk935_dn8 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn8))), (locals.var_xg20__blk901_dn9 + ((locals.var_dx_wi__blk935_dn9 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn9))),)
    } else {
        (locals.var_x2_wi0__blk909, locals.var_x2_wi0__blk909_dn4, locals.var_x2_wi0__blk909_dn6, locals.var_x2_wi0__blk909_dn7, locals.var_x2_wi0__blk909_dn8, locals.var_x2_wi0__blk909_dn9,)
    }
};
        locals.var_x2_wi0__blk909 = assign28670_e30614;
        locals.var_x2_wi0__blk909_dn4 = assign28670_e30614_d_n4;
        locals.var_x2_wi0__blk909_dn6 = assign28670_e30614_d_n6;
        locals.var_x2_wi0__blk909_dn7 = assign28670_e30614_d_n7;
        locals.var_x2_wi0__blk909_dn8 = assign28670_e30614_d_n8;
        locals.var_x2_wi0__blk909_dn9 = assign28670_e30614_d_n9;

        let (assign28680_e30624, assign28680_e30624_d_n4, assign28680_e30624_d_n6, assign28680_e30624_d_n7, assign28680_e30624_d_n8, assign28680_e30624_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28680_e30621: f64 = (locals.var_k1_1d + 1.0);
        let assign28680_e30622: f64 = (1.0 / assign28680_e30621);
        (assign28680_e30622, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign28680_e30624;
        locals.var_q_temp1__blk814_dn4 = assign28680_e30624_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign28680_e30624_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign28680_e30624_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign28680_e30624_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign28680_e30624_d_n9;

        let (assign28690_e30634, assign28690_e30634_d_n4, assign28690_e30634_d_n6, assign28690_e30634_d_n7, assign28690_e30634_d_n8, assign28690_e30634_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28690_e30631: f64 = (locals.var_k2_1d + 1.0);
        let assign28690_e30632: f64 = (1.0 / assign28690_e30631);
        (assign28690_e30632, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign28690_e30634;
        locals.var_q_temp2__blk815_dn4 = assign28690_e30634_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign28690_e30634_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign28690_e30634_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign28690_e30634_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign28690_e30634_d_n9;

        let (assign28700_e30653, assign28700_e30653_d_n4, assign28700_e30653_d_n6, assign28700_e30653_d_n7, assign28700_e30653_d_n8, assign28700_e30653_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28700_e30641: f64 = (locals.var_k2_1d * locals.var_q_temp2__blk815);
        let assign28700_e30642: f64 = (locals.var_k1_1d + assign28700_e30641);
        let assign28700_e30644: f64 = (assign28700_e30642 * locals.var_diff_min__blk904);
        let assign28700_e30646: f64 = (assign28700_e30644 / locals.var_a0__blk905);
        let assign28700_e30647: f64 = (assign28700_e30646).ln();
        let assign28700_e30649: f64 = assign28700_e30647;
        let assign28700_e30651: f64 = (assign28700_e30649 + 1.5);
        (assign28700_e30651, (((((((locals.var_k2_1d * locals.var_q_temp2__blk815_dn4) * locals.var_diff_min__blk904) + (assign28700_e30642 * locals.var_diff_min__blk904_dn4)) * locals.var_a0__blk905) - (assign28700_e30644 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28700_e30646), (((((((locals.var_k2_1d * locals.var_q_temp2__blk815_dn6) * locals.var_diff_min__blk904) + (assign28700_e30642 * locals.var_diff_min__blk904_dn6)) * locals.var_a0__blk905) - (assign28700_e30644 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28700_e30646), (((((((locals.var_k2_1d * locals.var_q_temp2__blk815_dn7) * locals.var_diff_min__blk904) + (assign28700_e30642 * locals.var_diff_min__blk904_dn7)) * locals.var_a0__blk905) - (assign28700_e30644 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28700_e30646), (((((((locals.var_k2_1d * locals.var_q_temp2__blk815_dn8) * locals.var_diff_min__blk904) + (assign28700_e30642 * locals.var_diff_min__blk904_dn8)) * locals.var_a0__blk905) - (assign28700_e30644 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28700_e30646), (((((((locals.var_k2_1d * locals.var_q_temp2__blk815_dn9) * locals.var_diff_min__blk904) + (assign28700_e30642 * locals.var_diff_min__blk904_dn9)) * locals.var_a0__blk905) - (assign28700_e30644 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28700_e30646),)
    } else {
        (locals.var_q_x1sat__blk817, locals.var_q_x1sat__blk817_dn4, locals.var_q_x1sat__blk817_dn6, locals.var_q_x1sat__blk817_dn7, locals.var_q_x1sat__blk817_dn8, locals.var_q_x1sat__blk817_dn9,)
    }
};
        locals.var_q_x1sat__blk817 = assign28700_e30653;
        locals.var_q_x1sat__blk817_dn4 = assign28700_e30653_d_n4;
        locals.var_q_x1sat__blk817_dn6 = assign28700_e30653_d_n6;
        locals.var_q_x1sat__blk817_dn7 = assign28700_e30653_d_n7;
        locals.var_q_x1sat__blk817_dn8 = assign28700_e30653_d_n8;
        locals.var_q_x1sat__blk817_dn9 = assign28700_e30653_d_n9;

        let (assign28710_e30672, assign28710_e30672_d_n4, assign28710_e30672_d_n6, assign28710_e30672_d_n7, assign28710_e30672_d_n8, assign28710_e30672_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28710_e30660: f64 = (locals.var_k1_1d * locals.var_q_temp1__blk814);
        let assign28710_e30661: f64 = (locals.var_k2_1d + assign28710_e30660);
        let assign28710_e30663: f64 = (assign28710_e30661 * locals.var_diff_min__blk904);
        let assign28710_e30665: f64 = (assign28710_e30663 / locals.var_a0__blk905);
        let assign28710_e30666: f64 = (assign28710_e30665).ln();
        let assign28710_e30668: f64 = assign28710_e30666;
        let assign28710_e30670: f64 = (assign28710_e30668 + 1.5);
        (assign28710_e30670, (((((((locals.var_k1_1d * locals.var_q_temp1__blk814_dn4) * locals.var_diff_min__blk904) + (assign28710_e30661 * locals.var_diff_min__blk904_dn4)) * locals.var_a0__blk905) - (assign28710_e30663 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28710_e30665), (((((((locals.var_k1_1d * locals.var_q_temp1__blk814_dn6) * locals.var_diff_min__blk904) + (assign28710_e30661 * locals.var_diff_min__blk904_dn6)) * locals.var_a0__blk905) - (assign28710_e30663 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28710_e30665), (((((((locals.var_k1_1d * locals.var_q_temp1__blk814_dn7) * locals.var_diff_min__blk904) + (assign28710_e30661 * locals.var_diff_min__blk904_dn7)) * locals.var_a0__blk905) - (assign28710_e30663 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28710_e30665), (((((((locals.var_k1_1d * locals.var_q_temp1__blk814_dn8) * locals.var_diff_min__blk904) + (assign28710_e30661 * locals.var_diff_min__blk904_dn8)) * locals.var_a0__blk905) - (assign28710_e30663 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28710_e30665), (((((((locals.var_k1_1d * locals.var_q_temp1__blk814_dn9) * locals.var_diff_min__blk904) + (assign28710_e30661 * locals.var_diff_min__blk904_dn9)) * locals.var_a0__blk905) - (assign28710_e30663 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28710_e30665),)
    } else {
        (locals.var_q_x2sat__blk818, locals.var_q_x2sat__blk818_dn4, locals.var_q_x2sat__blk818_dn6, locals.var_q_x2sat__blk818_dn7, locals.var_q_x2sat__blk818_dn8, locals.var_q_x2sat__blk818_dn9,)
    }
};
        locals.var_q_x2sat__blk818 = assign28710_e30672;
        locals.var_q_x2sat__blk818_dn4 = assign28710_e30672_d_n4;
        locals.var_q_x2sat__blk818_dn6 = assign28710_e30672_d_n6;
        locals.var_q_x2sat__blk818_dn7 = assign28710_e30672_d_n7;
        locals.var_q_x2sat__blk818_dn8 = assign28710_e30672_d_n8;
        locals.var_q_x2sat__blk818_dn9 = assign28710_e30672_d_n9;

        let assign28720_e30675: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign28720_e30677: f64 = (assign28720_e30675 / 1.5);
        let assign28720_e30679: f64 = if assign28720_e30677 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign28720_e30679;

        let (assign28730_e30695, assign28730_e30695_d_n4, assign28730_e30695_d_n6, assign28730_e30695_d_n7, assign28730_e30695_d_n8, assign28730_e30695_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign28730_e30688: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign28730_e30690: f64 = (assign28730_e30688 / 1.5);
        let assign28730_e30691: f64 = (assign28730_e30690).exp();
        let assign28730_e30692: f64 = (1.0 + assign28730_e30691);
        let assign28730_e30693: f64 = (assign28730_e30692).ln();
        (assign28730_e30693, ((assign28730_e30691 * ((locals.var_q_x1sat__blk817_dn4 - locals.var_x1_wi0__blk908_dn4) / 1.5)) / assign28730_e30692), ((assign28730_e30691 * ((locals.var_q_x1sat__blk817_dn6 - locals.var_x1_wi0__blk908_dn6) / 1.5)) / assign28730_e30692), ((assign28730_e30691 * ((locals.var_q_x1sat__blk817_dn7 - locals.var_x1_wi0__blk908_dn7) / 1.5)) / assign28730_e30692), ((assign28730_e30691 * ((locals.var_q_x1sat__blk817_dn8 - locals.var_x1_wi0__blk908_dn8) / 1.5)) / assign28730_e30692), ((assign28730_e30691 * ((locals.var_q_x1sat__blk817_dn9 - locals.var_x1_wi0__blk908_dn9) / 1.5)) / assign28730_e30692),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign28730_e30695;
        locals.var_q_temp3__blk816_dn4 = assign28730_e30695_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign28730_e30695_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign28730_e30695_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign28730_e30695_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign28730_e30695_d_n9;

        let (assign28740_e30708, assign28740_e30708_d_n4, assign28740_e30708_d_n6, assign28740_e30708_d_n7, assign28740_e30708_d_n8, assign28740_e30708_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1083 == 0.0)) {
        let assign28740_e30704: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign28740_e30706: f64 = (assign28740_e30704 / 1.5);
        (assign28740_e30706, ((locals.var_q_x1sat__blk817_dn4 - locals.var_x1_wi0__blk908_dn4) / 1.5), ((locals.var_q_x1sat__blk817_dn6 - locals.var_x1_wi0__blk908_dn6) / 1.5), ((locals.var_q_x1sat__blk817_dn7 - locals.var_x1_wi0__blk908_dn7) / 1.5), ((locals.var_q_x1sat__blk817_dn8 - locals.var_x1_wi0__blk908_dn8) / 1.5), ((locals.var_q_x1sat__blk817_dn9 - locals.var_x1_wi0__blk908_dn9) / 1.5),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign28740_e30708;
        locals.var_q_temp3__blk816_dn4 = assign28740_e30708_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign28740_e30708_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign28740_e30708_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign28740_e30708_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign28740_e30708_d_n9;

        let (assign28750_e30718, assign28750_e30718_d_n4, assign28750_e30718_d_n6, assign28750_e30718_d_n7, assign28750_e30718_d_n8, assign28750_e30718_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28750_e30715: f64 = (1.5 * locals.var_q_temp3__blk816);
        let assign28750_e30716: f64 = (locals.var_q_x1sat__blk817 - assign28750_e30715);
        (assign28750_e30716, (locals.var_q_x1sat__blk817_dn4 - (1.5 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x1sat__blk817_dn6 - (1.5 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x1sat__blk817_dn7 - (1.5 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x1sat__blk817_dn8 - (1.5 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x1sat__blk817_dn9 - (1.5 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x1__blk821, locals.var_q_x1__blk821_dn4, locals.var_q_x1__blk821_dn6, locals.var_q_x1__blk821_dn7, locals.var_q_x1__blk821_dn8, locals.var_q_x1__blk821_dn9,)
    }
};
        locals.var_q_x1__blk821 = assign28750_e30718;
        locals.var_q_x1__blk821_dn4 = assign28750_e30718_d_n4;
        locals.var_q_x1__blk821_dn6 = assign28750_e30718_d_n6;
        locals.var_q_x1__blk821_dn7 = assign28750_e30718_d_n7;
        locals.var_q_x1__blk821_dn8 = assign28750_e30718_d_n8;
        locals.var_q_x1__blk821_dn9 = assign28750_e30718_d_n9;

        let (assign28760_e30730, assign28760_e30730_d_n4, assign28760_e30730_d_n6, assign28760_e30730_d_n7, assign28760_e30730_d_n8, assign28760_e30730_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28760_e30724: f64 = (locals.var_k2_1d * locals.var_xg20__blk901);
        let assign28760_e30726: f64 = (assign28760_e30724 + locals.var_q_x1__blk821);
        let assign28760_e30728: f64 = (assign28760_e30726 * locals.var_q_temp2__blk815);
        (assign28760_e30728, ((((locals.var_k2_1d * locals.var_xg20__blk901_dn4) + locals.var_q_x1__blk821_dn4) * locals.var_q_temp2__blk815) + (assign28760_e30726 * locals.var_q_temp2__blk815_dn4)), ((((locals.var_k2_1d * locals.var_xg20__blk901_dn6) + locals.var_q_x1__blk821_dn6) * locals.var_q_temp2__blk815) + (assign28760_e30726 * locals.var_q_temp2__blk815_dn6)), ((((locals.var_k2_1d * locals.var_xg20__blk901_dn7) + locals.var_q_x1__blk821_dn7) * locals.var_q_temp2__blk815) + (assign28760_e30726 * locals.var_q_temp2__blk815_dn7)), ((((locals.var_k2_1d * locals.var_xg20__blk901_dn8) + locals.var_q_x1__blk821_dn8) * locals.var_q_temp2__blk815) + (assign28760_e30726 * locals.var_q_temp2__blk815_dn8)), ((((locals.var_k2_1d * locals.var_xg20__blk901_dn9) + locals.var_q_x1__blk821_dn9) * locals.var_q_temp2__blk815) + (assign28760_e30726 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_x2_wi__blk820, locals.var_q_x2_wi__blk820_dn4, locals.var_q_x2_wi__blk820_dn6, locals.var_q_x2_wi__blk820_dn7, locals.var_q_x2_wi__blk820_dn8, locals.var_q_x2_wi__blk820_dn9,)
    }
};
        locals.var_q_x2_wi__blk820 = assign28760_e30730;
        locals.var_q_x2_wi__blk820_dn4 = assign28760_e30730_d_n4;
        locals.var_q_x2_wi__blk820_dn6 = assign28760_e30730_d_n6;
        locals.var_q_x2_wi__blk820_dn7 = assign28760_e30730_d_n7;
        locals.var_q_x2_wi__blk820_dn8 = assign28760_e30730_d_n8;
        locals.var_q_x2_wi__blk820_dn9 = assign28760_e30730_d_n9;

        let assign28770_e30733: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign28770_e30735: f64 = (assign28770_e30733 / 1.5);
        let assign28770_e30737: f64 = if assign28770_e30735 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1084 = assign28770_e30737;

        let (assign28780_e30753, assign28780_e30753_d_n4, assign28780_e30753_d_n6, assign28780_e30753_d_n7, assign28780_e30753_d_n8, assign28780_e30753_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) {
        let assign28780_e30746: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign28780_e30748: f64 = (assign28780_e30746 / 1.5);
        let assign28780_e30749: f64 = (assign28780_e30748).exp();
        let assign28780_e30750: f64 = (1.0 + assign28780_e30749);
        let assign28780_e30751: f64 = (assign28780_e30750).ln();
        (assign28780_e30751, ((assign28780_e30749 * ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) / 1.5)) / assign28780_e30750), ((assign28780_e30749 * ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) / 1.5)) / assign28780_e30750), ((assign28780_e30749 * ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) / 1.5)) / assign28780_e30750), ((assign28780_e30749 * ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) / 1.5)) / assign28780_e30750), ((assign28780_e30749 * ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) / 1.5)) / assign28780_e30750),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign28780_e30753;
        locals.var_q_temp3__blk816_dn4 = assign28780_e30753_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign28780_e30753_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign28780_e30753_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign28780_e30753_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign28780_e30753_d_n9;

        let (assign28790_e30766, assign28790_e30766_d_n4, assign28790_e30766_d_n6, assign28790_e30766_d_n7, assign28790_e30766_d_n8, assign28790_e30766_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 == 0.0)) {
        let assign28790_e30762: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign28790_e30764: f64 = (assign28790_e30762 / 1.5);
        (assign28790_e30764, ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) / 1.5), ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) / 1.5), ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) / 1.5), ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) / 1.5), ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) / 1.5),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign28790_e30766;
        locals.var_q_temp3__blk816_dn4 = assign28790_e30766_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign28790_e30766_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign28790_e30766_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign28790_e30766_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign28790_e30766_d_n9;

        let (assign28800_e30776, assign28800_e30776_d_n4, assign28800_e30776_d_n6, assign28800_e30776_d_n7, assign28800_e30776_d_n8, assign28800_e30776_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28800_e30773: f64 = (1.5 * locals.var_q_temp3__blk816);
        let assign28800_e30774: f64 = (locals.var_q_x2sat__blk818 - assign28800_e30773);
        (assign28800_e30774, (locals.var_q_x2sat__blk818_dn4 - (1.5 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x2sat__blk818_dn6 - (1.5 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x2sat__blk818_dn7 - (1.5 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x2sat__blk818_dn8 - (1.5 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x2sat__blk818_dn9 - (1.5 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9,)
    }
};
        locals.var_temp0 = assign28800_e30776;
        locals.var_temp0_dn4 = assign28800_e30776_d_n4;
        locals.var_temp0_dn6 = assign28800_e30776_d_n6;
        locals.var_temp0_dn7 = assign28800_e30776_d_n7;
        locals.var_temp0_dn8 = assign28800_e30776_d_n8;
        locals.var_temp0_dn9 = assign28800_e30776_d_n9;

        let (assign28810_e30784, assign28810_e30784_d_n4, assign28810_e30784_d_n6, assign28810_e30784_d_n7, assign28810_e30784_d_n8, assign28810_e30784_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28810_e30782: f64 = (locals.var_temp * locals.var_temp0);
        (assign28810_e30782, ((locals.var_temp_dn4 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn4)), ((locals.var_temp_dn6 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn6)), ((locals.var_temp_dn7 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn7)), ((locals.var_temp_dn8 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn8)), ((locals.var_temp_dn9 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign28810_e30784;
        locals.var_temp1_dn4 = assign28810_e30784_d_n4;
        locals.var_temp1_dn6 = assign28810_e30784_d_n6;
        locals.var_temp1_dn7 = assign28810_e30784_d_n7;
        locals.var_temp1_dn8 = assign28810_e30784_d_n8;
        locals.var_temp1_dn9 = assign28810_e30784_d_n9;

        let (assign28820_e30792, assign28820_e30792_d_n4, assign28820_e30792_d_n6, assign28820_e30792_d_n7, assign28820_e30792_d_n8, assign28820_e30792_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28820_e30790: f64 = (locals.var_temp * locals.var_xg20__blk901);
        (assign28820_e30790, ((locals.var_temp_dn4 * locals.var_xg20__blk901) + (locals.var_temp * locals.var_xg20__blk901_dn4)), ((locals.var_temp_dn6 * locals.var_xg20__blk901) + (locals.var_temp * locals.var_xg20__blk901_dn6)), ((locals.var_temp_dn7 * locals.var_xg20__blk901) + (locals.var_temp * locals.var_xg20__blk901_dn7)), ((locals.var_temp_dn8 * locals.var_xg20__blk901) + (locals.var_temp * locals.var_xg20__blk901_dn8)), ((locals.var_temp_dn9 * locals.var_xg20__blk901) + (locals.var_temp * locals.var_xg20__blk901_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign28820_e30792;
        locals.var_temp2_dn4 = assign28820_e30792_d_n4;
        locals.var_temp2_dn6 = assign28820_e30792_d_n6;
        locals.var_temp2_dn7 = assign28820_e30792_d_n7;
        locals.var_temp2_dn8 = assign28820_e30792_d_n8;
        locals.var_temp2_dn9 = assign28820_e30792_d_n9;

        let (assign28830_e30800, assign28830_e30800_d_n4, assign28830_e30800_d_n6, assign28830_e30800_d_n7, assign28830_e30800_d_n8, assign28830_e30800_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28830_e30798: f64 = (locals.var_temp1 - locals.var_temp2);
        (assign28830_e30798, (locals.var_temp1_dn4 - locals.var_temp2_dn4), (locals.var_temp1_dn6 - locals.var_temp2_dn6), (locals.var_temp1_dn7 - locals.var_temp2_dn7), (locals.var_temp1_dn8 - locals.var_temp2_dn8), (locals.var_temp1_dn9 - locals.var_temp2_dn9),)
    } else {
        (locals.var_spsub_xgb__blk866, locals.var_spsub_xgb__blk866_dn4, locals.var_spsub_xgb__blk866_dn6, locals.var_spsub_xgb__blk866_dn7, locals.var_spsub_xgb__blk866_dn8, locals.var_spsub_xgb__blk866_dn9,)
    }
};
        locals.var_spsub_xgb__blk866 = assign28830_e30800;
        locals.var_spsub_xgb__blk866_dn4 = assign28830_e30800_d_n4;
        locals.var_spsub_xgb__blk866_dn6 = assign28830_e30800_d_n6;
        locals.var_spsub_xgb__blk866_dn7 = assign28830_e30800_d_n7;
        locals.var_spsub_xgb__blk866_dn8 = assign28830_e30800_d_n8;
        locals.var_spsub_xgb__blk866_dn9 = assign28830_e30800_d_n9;

        let assign28840_e30802: f64 = (-locals.var_xn_sub);
        let assign28840_e30803: f64 = (assign28840_e30802).abs();
        let assign28840_e30805: f64 = if assign28840_e30803 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1085 = assign28840_e30805;

    }

    pub(super) fn stamp_transient_block_73(
        locals: &mut StampLocals,
    ) {
        let (assign28850_e30815, assign28850_e30815_d_n4, assign28850_e30815_d_n6, assign28850_e30815_d_n7, assign28850_e30815_d_n8, assign28850_e30815_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1085 != 0.0)) {
        let assign28850_e30812: f64 = (-locals.var_xn_sub);
        let assign28850_e30813: f64 = (assign28850_e30812).exp();
        (assign28850_e30813, (assign28850_e30813 * (-locals.var_xn_sub_dn4)), (assign28850_e30813 * (-locals.var_xn_sub_dn6)), (assign28850_e30813 * (-locals.var_xn_sub_dn7)), (assign28850_e30813 * (-locals.var_xn_sub_dn8)), (assign28850_e30813 * (-locals.var_xn_sub_dn9)),)
    } else {
        (locals.var_spsub_delta__blk867, locals.var_spsub_delta__blk867_dn4, locals.var_spsub_delta__blk867_dn6, locals.var_spsub_delta__blk867_dn7, locals.var_spsub_delta__blk867_dn8, locals.var_spsub_delta__blk867_dn9,)
    }
};
        locals.var_spsub_delta__blk867 = assign28850_e30815;
        locals.var_spsub_delta__blk867_dn4 = assign28850_e30815_d_n4;
        locals.var_spsub_delta__blk867_dn6 = assign28850_e30815_d_n6;
        locals.var_spsub_delta__blk867_dn7 = assign28850_e30815_d_n7;
        locals.var_spsub_delta__blk867_dn8 = assign28850_e30815_d_n8;
        locals.var_spsub_delta__blk867_dn9 = assign28850_e30815_d_n9;

        let assign28860_e30817: f64 = (-locals.var_xn_sub);
        let assign28860_e30819: f64 = (-80.0);
        let assign28860_e30820: f64 = if assign28860_e30817 < assign28860_e30819 { 1.0 } else { 0.0 };
        locals.var_guard1086 = assign28860_e30820;

        let (assign28870_e30859, assign28870_e30859_d_n4, assign28870_e30859_d_n6, assign28870_e30859_d_n7, assign28870_e30859_d_n8, assign28870_e30859_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1085 == 0.0)) && (locals.var_guard1086 != 0.0)) {
        let assign28870_e30832: f64 = (-locals.var_xn_sub);
        let assign28870_e30833: f64 = (-assign28870_e30832);
        let assign28870_e30835: f64 = (assign28870_e30833 - 80.0);
        let assign28870_e30839: f64 = (-locals.var_xn_sub);
        let assign28870_e30840: f64 = (-assign28870_e30839);
        let assign28870_e30842: f64 = (assign28870_e30840 - 80.0);
        let assign28870_e30843: f64 = (0.5 * assign28870_e30842);
        let assign28870_e30846: f64 = (-locals.var_xn_sub);
        let assign28870_e30847: f64 = (-assign28870_e30846);
        let assign28870_e30849: f64 = (assign28870_e30847 - 80.0);
        let assign28870_e30851: f64 = (assign28870_e30849 * 0.3333333333333);
        let assign28870_e30852: f64 = (1.0 + assign28870_e30851);
        let assign28870_e30853: f64 = (assign28870_e30843 * assign28870_e30852);
        let assign28870_e30854: f64 = (1.0 + assign28870_e30853);
        let assign28870_e30855: f64 = (assign28870_e30835 * assign28870_e30854);
        let assign28870_e30856: f64 = (1.0 + assign28870_e30855);
        let assign28870_e30857: f64 = (1.80485e-35 / assign28870_e30856);
        (assign28870_e30857, (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn4)) * assign28870_e30854) + (assign28870_e30835 * (((0.5 * (-(-locals.var_xn_sub_dn4))) * assign28870_e30852) + (assign28870_e30843 * ((-(-locals.var_xn_sub_dn4)) * 0.3333333333333)))))) / (assign28870_e30856 * assign28870_e30856))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn6)) * assign28870_e30854) + (assign28870_e30835 * (((0.5 * (-(-locals.var_xn_sub_dn6))) * assign28870_e30852) + (assign28870_e30843 * ((-(-locals.var_xn_sub_dn6)) * 0.3333333333333)))))) / (assign28870_e30856 * assign28870_e30856))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn7)) * assign28870_e30854) + (assign28870_e30835 * (((0.5 * (-(-locals.var_xn_sub_dn7))) * assign28870_e30852) + (assign28870_e30843 * ((-(-locals.var_xn_sub_dn7)) * 0.3333333333333)))))) / (assign28870_e30856 * assign28870_e30856))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn8)) * assign28870_e30854) + (assign28870_e30835 * (((0.5 * (-(-locals.var_xn_sub_dn8))) * assign28870_e30852) + (assign28870_e30843 * ((-(-locals.var_xn_sub_dn8)) * 0.3333333333333)))))) / (assign28870_e30856 * assign28870_e30856))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn9)) * assign28870_e30854) + (assign28870_e30835 * (((0.5 * (-(-locals.var_xn_sub_dn9))) * assign28870_e30852) + (assign28870_e30843 * ((-(-locals.var_xn_sub_dn9)) * 0.3333333333333)))))) / (assign28870_e30856 * assign28870_e30856))),)
    } else {
        (locals.var_spsub_delta__blk867, locals.var_spsub_delta__blk867_dn4, locals.var_spsub_delta__blk867_dn6, locals.var_spsub_delta__blk867_dn7, locals.var_spsub_delta__blk867_dn8, locals.var_spsub_delta__blk867_dn9,)
    }
};
        locals.var_spsub_delta__blk867 = assign28870_e30859;
        locals.var_spsub_delta__blk867_dn4 = assign28870_e30859_d_n4;
        locals.var_spsub_delta__blk867_dn6 = assign28870_e30859_d_n6;
        locals.var_spsub_delta__blk867_dn7 = assign28870_e30859_d_n7;
        locals.var_spsub_delta__blk867_dn8 = assign28870_e30859_d_n8;
        locals.var_spsub_delta__blk867_dn9 = assign28870_e30859_d_n9;

        let (assign28880_e30896, assign28880_e30896_d_n4, assign28880_e30896_d_n6, assign28880_e30896_d_n7, assign28880_e30896_d_n8, assign28880_e30896_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1085 == 0.0)) && (locals.var_guard1086 == 0.0)) {
        let assign28880_e30872: f64 = (-locals.var_xn_sub);
        let assign28880_e30874: f64 = (assign28880_e30872 - 80.0);
        let assign28880_e30878: f64 = (-locals.var_xn_sub);
        let assign28880_e30880: f64 = (assign28880_e30878 - 80.0);
        let assign28880_e30881: f64 = (0.5 * assign28880_e30880);
        let assign28880_e30884: f64 = (-locals.var_xn_sub);
        let assign28880_e30886: f64 = (assign28880_e30884 - 80.0);
        let assign28880_e30888: f64 = (assign28880_e30886 * 0.3333333333333);
        let assign28880_e30889: f64 = (1.0 + assign28880_e30888);
        let assign28880_e30890: f64 = (assign28880_e30881 * assign28880_e30889);
        let assign28880_e30891: f64 = (1.0 + assign28880_e30890);
        let assign28880_e30892: f64 = (assign28880_e30874 * assign28880_e30891);
        let assign28880_e30893: f64 = (1.0 + assign28880_e30892);
        let assign28880_e30894: f64 = (5.54062e34 * assign28880_e30893);
        (assign28880_e30894, (5.54062e34 * (((-locals.var_xn_sub_dn4) * assign28880_e30891) + (assign28880_e30874 * (((0.5 * (-locals.var_xn_sub_dn4)) * assign28880_e30889) + (assign28880_e30881 * ((-locals.var_xn_sub_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn6) * assign28880_e30891) + (assign28880_e30874 * (((0.5 * (-locals.var_xn_sub_dn6)) * assign28880_e30889) + (assign28880_e30881 * ((-locals.var_xn_sub_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn7) * assign28880_e30891) + (assign28880_e30874 * (((0.5 * (-locals.var_xn_sub_dn7)) * assign28880_e30889) + (assign28880_e30881 * ((-locals.var_xn_sub_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn8) * assign28880_e30891) + (assign28880_e30874 * (((0.5 * (-locals.var_xn_sub_dn8)) * assign28880_e30889) + (assign28880_e30881 * ((-locals.var_xn_sub_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn9) * assign28880_e30891) + (assign28880_e30874 * (((0.5 * (-locals.var_xn_sub_dn9)) * assign28880_e30889) + (assign28880_e30881 * ((-locals.var_xn_sub_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_spsub_delta__blk867, locals.var_spsub_delta__blk867_dn4, locals.var_spsub_delta__blk867_dn6, locals.var_spsub_delta__blk867_dn7, locals.var_spsub_delta__blk867_dn8, locals.var_spsub_delta__blk867_dn9,)
    }
};
        locals.var_spsub_delta__blk867 = assign28880_e30896;
        locals.var_spsub_delta__blk867_dn4 = assign28880_e30896_d_n4;
        locals.var_spsub_delta__blk867_dn6 = assign28880_e30896_d_n6;
        locals.var_spsub_delta__blk867_dn7 = assign28880_e30896_d_n7;
        locals.var_spsub_delta__blk867_dn8 = assign28880_e30896_d_n8;
        locals.var_spsub_delta__blk867_dn9 = assign28880_e30896_d_n9;

        let assign28890_e30898: f64 = (locals.var_spsub_xgb__blk866).abs();
        let assign28890_e30900: f64 = if assign28890_e30898 <= locals.var_margin_sub { 1.0 } else { 0.0 };
        locals.var_guard1087 = assign28890_e30900;

        let (assign28900_e30914, assign28900_e30914_d_n4, assign28900_e30914_d_n6, assign28900_e30914_d_n7, assign28900_e30914_d_n8, assign28900_e30914_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 != 0.0)) {
        let assign28900_e30908: f64 = (locals.var_inv_xisub * locals.var_inv_xisub);
        let assign28900_e30910: f64 = (assign28900_e30908 * 0.1666666666667);
        let assign28900_e30912: f64 = (assign28900_e30910 / 1.4142135623731);
        (assign28900_e30912, ((((locals.var_inv_xisub_dn4 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn4)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn6 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn6)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn7 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn7)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn8 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn8)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn9 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn9)) * 0.1666666666667) / 1.4142135623731),)
    } else {
        (locals.var_spsub_temp1__blk864, locals.var_spsub_temp1__blk864_dn4, locals.var_spsub_temp1__blk864_dn6, locals.var_spsub_temp1__blk864_dn7, locals.var_spsub_temp1__blk864_dn8, locals.var_spsub_temp1__blk864_dn9,)
    }
};
        locals.var_spsub_temp1__blk864 = assign28900_e30914;
        locals.var_spsub_temp1__blk864_dn4 = assign28900_e30914_d_n4;
        locals.var_spsub_temp1__blk864_dn6 = assign28900_e30914_d_n6;
        locals.var_spsub_temp1__blk864_dn7 = assign28900_e30914_d_n7;
        locals.var_spsub_temp1__blk864_dn8 = assign28900_e30914_d_n8;
        locals.var_spsub_temp1__blk864_dn9 = assign28900_e30914_d_n9;

        let (assign28910_e30936, assign28910_e30936_d_n4, assign28910_e30936_d_n6, assign28910_e30936_d_n7, assign28910_e30936_d_n8, assign28910_e30936_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 != 0.0)) {
        let assign28910_e30922: f64 = (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub);
        let assign28910_e30927: f64 = (1.0 - locals.var_spsub_delta__blk867);
        let assign28910_e30928: f64 = (locals.var_spsub_xgb__blk866 * assign28910_e30927);
        let assign28910_e30930: f64 = (assign28910_e30928 * locals.var_gfsub);
        let assign28910_e30932: f64 = (assign28910_e30930 * locals.var_spsub_temp1__blk864);
        let assign28910_e30933: f64 = (1.0 + assign28910_e30932);
        let assign28910_e30934: f64 = (assign28910_e30922 * assign28910_e30933);
        (assign28910_e30934, ((((locals.var_spsub_xgb__blk866_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn4)) * assign28910_e30933) + (assign28910_e30922 * ((((((locals.var_spsub_xgb__blk866_dn4 * assign28910_e30927) + (locals.var_spsub_xgb__blk866 * (-locals.var_spsub_delta__blk867_dn4))) * locals.var_gfsub) + (assign28910_e30928 * locals.var_gfsub_dn4)) * locals.var_spsub_temp1__blk864) + (assign28910_e30930 * locals.var_spsub_temp1__blk864_dn4)))), ((((locals.var_spsub_xgb__blk866_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn6)) * assign28910_e30933) + (assign28910_e30922 * ((((((locals.var_spsub_xgb__blk866_dn6 * assign28910_e30927) + (locals.var_spsub_xgb__blk866 * (-locals.var_spsub_delta__blk867_dn6))) * locals.var_gfsub) + (assign28910_e30928 * locals.var_gfsub_dn6)) * locals.var_spsub_temp1__blk864) + (assign28910_e30930 * locals.var_spsub_temp1__blk864_dn6)))), ((((locals.var_spsub_xgb__blk866_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn7)) * assign28910_e30933) + (assign28910_e30922 * ((((((locals.var_spsub_xgb__blk866_dn7 * assign28910_e30927) + (locals.var_spsub_xgb__blk866 * (-locals.var_spsub_delta__blk867_dn7))) * locals.var_gfsub) + (assign28910_e30928 * locals.var_gfsub_dn7)) * locals.var_spsub_temp1__blk864) + (assign28910_e30930 * locals.var_spsub_temp1__blk864_dn7)))), ((((locals.var_spsub_xgb__blk866_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn8)) * assign28910_e30933) + (assign28910_e30922 * ((((((locals.var_spsub_xgb__blk866_dn8 * assign28910_e30927) + (locals.var_spsub_xgb__blk866 * (-locals.var_spsub_delta__blk867_dn8))) * locals.var_gfsub) + (assign28910_e30928 * locals.var_gfsub_dn8)) * locals.var_spsub_temp1__blk864) + (assign28910_e30930 * locals.var_spsub_temp1__blk864_dn8)))), ((((locals.var_spsub_xgb__blk866_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn9)) * assign28910_e30933) + (assign28910_e30922 * ((((((locals.var_spsub_xgb__blk866_dn9 * assign28910_e30927) + (locals.var_spsub_xgb__blk866 * (-locals.var_spsub_delta__blk867_dn9))) * locals.var_gfsub) + (assign28910_e30928 * locals.var_gfsub_dn9)) * locals.var_spsub_temp1__blk864) + (assign28910_e30930 * locals.var_spsub_temp1__blk864_dn9)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign28910_e30936;
        locals.var_temp3_dn4 = assign28910_e30936_d_n4;
        locals.var_temp3_dn6 = assign28910_e30936_d_n6;
        locals.var_temp3_dn7 = assign28910_e30936_d_n7;
        locals.var_temp3_dn8 = assign28910_e30936_d_n8;
        locals.var_temp3_dn9 = assign28910_e30936_d_n9;

        let assign28920_e30939: f64 = (-locals.var_margin_sub);
        let assign28920_e30940: f64 = if locals.var_spsub_xgb__blk866 < assign28920_e30939 { 1.0 } else { 0.0 };
        locals.var_guard1088 = assign28920_e30940;

        let (assign28930_e30952, assign28930_e30952_d_n4, assign28930_e30952_d_n6, assign28930_e30952_d_n7, assign28930_e30952_d_n8, assign28930_e30952_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign28930_e30950: f64 = (-locals.var_spsub_xgb__blk866);
        (assign28930_e30950, (-locals.var_spsub_xgb__blk866_dn4), (-locals.var_spsub_xgb__blk866_dn6), (-locals.var_spsub_xgb__blk866_dn7), (-locals.var_spsub_xgb__blk866_dn8), (-locals.var_spsub_xgb__blk866_dn9),)
    } else {
        (locals.var_spsub_yg__blk868, locals.var_spsub_yg__blk868_dn4, locals.var_spsub_yg__blk868_dn6, locals.var_spsub_yg__blk868_dn7, locals.var_spsub_yg__blk868_dn8, locals.var_spsub_yg__blk868_dn9,)
    }
};
        locals.var_spsub_yg__blk868 = assign28930_e30952;
        locals.var_spsub_yg__blk868_dn4 = assign28930_e30952_d_n4;
        locals.var_spsub_yg__blk868_dn6 = assign28930_e30952_d_n6;
        locals.var_spsub_yg__blk868_dn7 = assign28930_e30952_d_n7;
        locals.var_spsub_yg__blk868_dn8 = assign28930_e30952_d_n8;
        locals.var_spsub_yg__blk868_dn9 = assign28930_e30952_d_n9;

        let (assign28940_e30967, assign28940_e30967_d_n4, assign28940_e30967_d_n6, assign28940_e30967_d_n7, assign28940_e30967_d_n8, assign28940_e30967_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign28940_e30964: f64 = (locals.var_spsub_yg__blk868 * locals.var_inv_xisub);
        let assign28940_e30965: f64 = (1.25 * assign28940_e30964);
        (assign28940_e30965, (1.25 * ((locals.var_spsub_yg__blk868_dn4 * locals.var_inv_xisub) + (locals.var_spsub_yg__blk868 * locals.var_inv_xisub_dn4))), (1.25 * ((locals.var_spsub_yg__blk868_dn6 * locals.var_inv_xisub) + (locals.var_spsub_yg__blk868 * locals.var_inv_xisub_dn6))), (1.25 * ((locals.var_spsub_yg__blk868_dn7 * locals.var_inv_xisub) + (locals.var_spsub_yg__blk868 * locals.var_inv_xisub_dn7))), (1.25 * ((locals.var_spsub_yg__blk868_dn8 * locals.var_inv_xisub) + (locals.var_spsub_yg__blk868 * locals.var_inv_xisub_dn8))), (1.25 * ((locals.var_spsub_yg__blk868_dn9 * locals.var_inv_xisub) + (locals.var_spsub_yg__blk868 * locals.var_inv_xisub_dn9))),)
    } else {
        (locals.var_spsub_ysub__blk869, locals.var_spsub_ysub__blk869_dn4, locals.var_spsub_ysub__blk869_dn6, locals.var_spsub_ysub__blk869_dn7, locals.var_spsub_ysub__blk869_dn8, locals.var_spsub_ysub__blk869_dn9,)
    }
};
        locals.var_spsub_ysub__blk869 = assign28940_e30967;
        locals.var_spsub_ysub__blk869_dn4 = assign28940_e30967_d_n4;
        locals.var_spsub_ysub__blk869_dn6 = assign28940_e30967_d_n6;
        locals.var_spsub_ysub__blk869_dn7 = assign28940_e30967_d_n7;
        locals.var_spsub_ysub__blk869_dn8 = assign28940_e30967_d_n8;
        locals.var_spsub_ysub__blk869_dn9 = assign28940_e30967_d_n9;

        let (assign28950_e30993, assign28950_e30993_d_n4, assign28950_e30993_d_n6, assign28950_e30993_d_n7, assign28950_e30993_d_n8, assign28950_e30993_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign28950_e30979: f64 = (locals.var_spsub_ysub__blk869 + 10.0);
        let assign28950_e30982: f64 = (locals.var_spsub_ysub__blk869 - 6.0);
        let assign28950_e30985: f64 = (locals.var_spsub_ysub__blk869 - 6.0);
        let assign28950_e30986: f64 = (assign28950_e30982 * assign28950_e30985);
        let assign28950_e30988: f64 = (assign28950_e30986 + 64.0);
        let assign28950_e30989: f64 = (assign28950_e30988).sqrt();
        let assign28950_e30990: f64 = (assign28950_e30979 - assign28950_e30989);
        let assign28950_e30991: f64 = (0.5 * assign28950_e30990);
        (assign28950_e30991, (0.5 * (locals.var_spsub_ysub__blk869_dn4 - (((locals.var_spsub_ysub__blk869_dn4 * assign28950_e30985) + (assign28950_e30982 * locals.var_spsub_ysub__blk869_dn4)) / (2.0 * assign28950_e30989)))), (0.5 * (locals.var_spsub_ysub__blk869_dn6 - (((locals.var_spsub_ysub__blk869_dn6 * assign28950_e30985) + (assign28950_e30982 * locals.var_spsub_ysub__blk869_dn6)) / (2.0 * assign28950_e30989)))), (0.5 * (locals.var_spsub_ysub__blk869_dn7 - (((locals.var_spsub_ysub__blk869_dn7 * assign28950_e30985) + (assign28950_e30982 * locals.var_spsub_ysub__blk869_dn7)) / (2.0 * assign28950_e30989)))), (0.5 * (locals.var_spsub_ysub__blk869_dn8 - (((locals.var_spsub_ysub__blk869_dn8 * assign28950_e30985) + (assign28950_e30982 * locals.var_spsub_ysub__blk869_dn8)) / (2.0 * assign28950_e30989)))), (0.5 * (locals.var_spsub_ysub__blk869_dn9 - (((locals.var_spsub_ysub__blk869_dn9 * assign28950_e30985) + (assign28950_e30982 * locals.var_spsub_ysub__blk869_dn9)) / (2.0 * assign28950_e30989)))),)
    } else {
        (locals.var_spsub_eta__blk870, locals.var_spsub_eta__blk870_dn4, locals.var_spsub_eta__blk870_dn6, locals.var_spsub_eta__blk870_dn7, locals.var_spsub_eta__blk870_dn8, locals.var_spsub_eta__blk870_dn9,)
    }
};
        locals.var_spsub_eta__blk870 = assign28950_e30993;
        locals.var_spsub_eta__blk870_dn4 = assign28950_e30993_d_n4;
        locals.var_spsub_eta__blk870_dn6 = assign28950_e30993_d_n6;
        locals.var_spsub_eta__blk870_dn7 = assign28950_e30993_d_n7;
        locals.var_spsub_eta__blk870_dn8 = assign28950_e30993_d_n8;
        locals.var_spsub_eta__blk870_dn9 = assign28950_e30993_d_n9;

        let (assign28960_e31006, assign28960_e31006_d_n4, assign28960_e31006_d_n6, assign28960_e31006_d_n7, assign28960_e31006_d_n8, assign28960_e31006_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign28960_e31004: f64 = (locals.var_spsub_yg__blk868 - locals.var_spsub_eta__blk870);
        (assign28960_e31004, (locals.var_spsub_yg__blk868_dn4 - locals.var_spsub_eta__blk870_dn4), (locals.var_spsub_yg__blk868_dn6 - locals.var_spsub_eta__blk870_dn6), (locals.var_spsub_yg__blk868_dn7 - locals.var_spsub_eta__blk870_dn7), (locals.var_spsub_yg__blk868_dn8 - locals.var_spsub_eta__blk870_dn8), (locals.var_spsub_yg__blk868_dn9 - locals.var_spsub_eta__blk870_dn9),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign28960_e31006;
        locals.var_spsub_temp__blk863_dn4 = assign28960_e31006_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign28960_e31006_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign28960_e31006_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign28960_e31006_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign28960_e31006_d_n9;

        let (assign28970_e31025, assign28970_e31025_d_n4, assign28970_e31025_d_n6, assign28970_e31025_d_n7, assign28970_e31025_d_n8, assign28970_e31025_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign28970_e31017: f64 = (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863);
        let assign28970_e31021: f64 = (locals.var_spsub_eta__blk870 + 1.0);
        let assign28970_e31022: f64 = (locals.var_gfsub2 * assign28970_e31021);
        let assign28970_e31023: f64 = (assign28970_e31017 + assign28970_e31022);
        (assign28970_e31023, (((locals.var_spsub_temp__blk863_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn4)) + ((locals.var_gfsub2_dn4 * assign28970_e31021) + (locals.var_gfsub2 * locals.var_spsub_eta__blk870_dn4))), (((locals.var_spsub_temp__blk863_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn6)) + ((locals.var_gfsub2_dn6 * assign28970_e31021) + (locals.var_gfsub2 * locals.var_spsub_eta__blk870_dn6))), (((locals.var_spsub_temp__blk863_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn7)) + ((locals.var_gfsub2_dn7 * assign28970_e31021) + (locals.var_gfsub2 * locals.var_spsub_eta__blk870_dn7))), (((locals.var_spsub_temp__blk863_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn8)) + ((locals.var_gfsub2_dn8 * assign28970_e31021) + (locals.var_gfsub2 * locals.var_spsub_eta__blk870_dn8))), (((locals.var_spsub_temp__blk863_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn9)) + ((locals.var_gfsub2_dn9 * assign28970_e31021) + (locals.var_gfsub2 * locals.var_spsub_eta__blk870_dn9))),)
    } else {
        (locals.var_spsub_a__blk871, locals.var_spsub_a__blk871_dn4, locals.var_spsub_a__blk871_dn6, locals.var_spsub_a__blk871_dn7, locals.var_spsub_a__blk871_dn8, locals.var_spsub_a__blk871_dn9,)
    }
};
        locals.var_spsub_a__blk871 = assign28970_e31025;
        locals.var_spsub_a__blk871_dn4 = assign28970_e31025_d_n4;
        locals.var_spsub_a__blk871_dn6 = assign28970_e31025_d_n6;
        locals.var_spsub_a__blk871_dn7 = assign28970_e31025_d_n7;
        locals.var_spsub_a__blk871_dn8 = assign28970_e31025_d_n8;
        locals.var_spsub_a__blk871_dn9 = assign28970_e31025_d_n9;

        let (assign28980_e31040, assign28980_e31040_d_n4, assign28980_e31040_d_n6, assign28980_e31040_d_n7, assign28980_e31040_d_n8, assign28980_e31040_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign28980_e31036: f64 = (2.0 * locals.var_spsub_temp__blk863);
        let assign28980_e31038: f64 = (assign28980_e31036 - locals.var_gfsub2);
        (assign28980_e31038, ((2.0 * locals.var_spsub_temp__blk863_dn4) - locals.var_gfsub2_dn4), ((2.0 * locals.var_spsub_temp__blk863_dn6) - locals.var_gfsub2_dn6), ((2.0 * locals.var_spsub_temp__blk863_dn7) - locals.var_gfsub2_dn7), ((2.0 * locals.var_spsub_temp__blk863_dn8) - locals.var_gfsub2_dn8), ((2.0 * locals.var_spsub_temp__blk863_dn9) - locals.var_gfsub2_dn9),)
    } else {
        (locals.var_spsub_c__blk873, locals.var_spsub_c__blk873_dn4, locals.var_spsub_c__blk873_dn6, locals.var_spsub_c__blk873_dn7, locals.var_spsub_c__blk873_dn8, locals.var_spsub_c__blk873_dn9,)
    }
};
        locals.var_spsub_c__blk873 = assign28980_e31040;
        locals.var_spsub_c__blk873_dn4 = assign28980_e31040_d_n4;
        locals.var_spsub_c__blk873_dn6 = assign28980_e31040_d_n6;
        locals.var_spsub_c__blk873_dn7 = assign28980_e31040_d_n7;
        locals.var_spsub_c__blk873_dn8 = assign28980_e31040_d_n8;
        locals.var_spsub_c__blk873_dn9 = assign28980_e31040_d_n9;

        let (assign28990_e31057, assign28990_e31057_d_n4, assign28990_e31057_d_n6, assign28990_e31057_d_n7, assign28990_e31057_d_n8, assign28990_e31057_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign28990_e31050: f64 = (-locals.var_spsub_eta__blk870);
        let assign28990_e31053: f64 = (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2);
        let assign28990_e31054: f64 = (assign28990_e31053).ln();
        let assign28990_e31055: f64 = (assign28990_e31050 + assign28990_e31054);
        (assign28990_e31055, ((-locals.var_spsub_eta__blk870_dn4) + (((locals.var_spsub_a__blk871_dn4 * locals.var_inv_gfsub2) + (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2_dn4)) / assign28990_e31053)), ((-locals.var_spsub_eta__blk870_dn6) + (((locals.var_spsub_a__blk871_dn6 * locals.var_inv_gfsub2) + (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2_dn6)) / assign28990_e31053)), ((-locals.var_spsub_eta__blk870_dn7) + (((locals.var_spsub_a__blk871_dn7 * locals.var_inv_gfsub2) + (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2_dn7)) / assign28990_e31053)), ((-locals.var_spsub_eta__blk870_dn8) + (((locals.var_spsub_a__blk871_dn8 * locals.var_inv_gfsub2) + (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2_dn8)) / assign28990_e31053)), ((-locals.var_spsub_eta__blk870_dn9) + (((locals.var_spsub_a__blk871_dn9 * locals.var_inv_gfsub2) + (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2_dn9)) / assign28990_e31053)),)
    } else {
        (locals.var_spsub_tau__blk874, locals.var_spsub_tau__blk874_dn4, locals.var_spsub_tau__blk874_dn6, locals.var_spsub_tau__blk874_dn7, locals.var_spsub_tau__blk874_dn8, locals.var_spsub_tau__blk874_dn9,)
    }
};
        locals.var_spsub_tau__blk874 = assign28990_e31057;
        locals.var_spsub_tau__blk874_dn4 = assign28990_e31057_d_n4;
        locals.var_spsub_tau__blk874_dn6 = assign28990_e31057_d_n6;
        locals.var_spsub_tau__blk874_dn7 = assign28990_e31057_d_n7;
        locals.var_spsub_tau__blk874_dn8 = assign28990_e31057_d_n8;
        locals.var_spsub_tau__blk874_dn9 = assign28990_e31057_d_n9;

        let (assign29000_e31070, assign29000_e31070_d_n4, assign29000_e31070_d_n6, assign29000_e31070_d_n7, assign29000_e31070_d_n8, assign29000_e31070_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29000_e31068: f64 = (locals.var_spsub_a__blk871 + locals.var_spsub_c__blk873);
        (assign29000_e31068, (locals.var_spsub_a__blk871_dn4 + locals.var_spsub_c__blk873_dn4), (locals.var_spsub_a__blk871_dn6 + locals.var_spsub_c__blk873_dn6), (locals.var_spsub_a__blk871_dn7 + locals.var_spsub_c__blk873_dn7), (locals.var_spsub_a__blk871_dn8 + locals.var_spsub_c__blk873_dn8), (locals.var_spsub_a__blk871_dn9 + locals.var_spsub_c__blk873_dn9),)
    } else {
        (locals.var_nu__blk861, locals.var_nu__blk861_dn4, locals.var_nu__blk861_dn6, locals.var_nu__blk861_dn7, locals.var_nu__blk861_dn8, locals.var_nu__blk861_dn9,)
    }
};
        locals.var_nu__blk861 = assign29000_e31070;
        locals.var_nu__blk861_dn4 = assign29000_e31070_d_n4;
        locals.var_nu__blk861_dn6 = assign29000_e31070_d_n6;
        locals.var_nu__blk861_dn7 = assign29000_e31070_d_n7;
        locals.var_nu__blk861_dn8 = assign29000_e31070_d_n8;
        locals.var_nu__blk861_dn9 = assign29000_e31070_d_n9;

        let (assign29010_e31093, assign29010_e31093_d_n4, assign29010_e31093_d_n6, assign29010_e31093_d_n7, assign29010_e31093_d_n8, assign29010_e31093_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29010_e31081: f64 = (locals.var_nu__blk861 * locals.var_nu__blk861);
        let assign29010_e31085: f64 = (0.5 * locals.var_spsub_c__blk873);
        let assign29010_e31087: f64 = (assign29010_e31085 * locals.var_spsub_c__blk873);
        let assign29010_e31089: f64 = (assign29010_e31087 - locals.var_spsub_a__blk871);
        let assign29010_e31090: f64 = (locals.var_spsub_tau__blk874 * assign29010_e31089);
        let assign29010_e31091: f64 = (assign29010_e31081 + assign29010_e31090);
        (assign29010_e31091, (((locals.var_nu__blk861_dn4 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn4)) + ((locals.var_spsub_tau__blk874_dn4 * assign29010_e31089) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn4) * locals.var_spsub_c__blk873) + (assign29010_e31085 * locals.var_spsub_c__blk873_dn4)) - locals.var_spsub_a__blk871_dn4)))), (((locals.var_nu__blk861_dn6 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn6)) + ((locals.var_spsub_tau__blk874_dn6 * assign29010_e31089) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn6) * locals.var_spsub_c__blk873) + (assign29010_e31085 * locals.var_spsub_c__blk873_dn6)) - locals.var_spsub_a__blk871_dn6)))), (((locals.var_nu__blk861_dn7 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn7)) + ((locals.var_spsub_tau__blk874_dn7 * assign29010_e31089) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn7) * locals.var_spsub_c__blk873) + (assign29010_e31085 * locals.var_spsub_c__blk873_dn7)) - locals.var_spsub_a__blk871_dn7)))), (((locals.var_nu__blk861_dn8 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn8)) + ((locals.var_spsub_tau__blk874_dn8 * assign29010_e31089) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn8) * locals.var_spsub_c__blk873) + (assign29010_e31085 * locals.var_spsub_c__blk873_dn8)) - locals.var_spsub_a__blk871_dn8)))), (((locals.var_nu__blk861_dn9 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn9)) + ((locals.var_spsub_tau__blk874_dn9 * assign29010_e31089) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn9) * locals.var_spsub_c__blk873) + (assign29010_e31085 * locals.var_spsub_c__blk873_dn9)) - locals.var_spsub_a__blk871_dn9)))),)
    } else {
        (locals.var_mutau__blk862, locals.var_mutau__blk862_dn4, locals.var_mutau__blk862_dn6, locals.var_mutau__blk862_dn7, locals.var_mutau__blk862_dn8, locals.var_mutau__blk862_dn9,)
    }
};
        locals.var_mutau__blk862 = assign29010_e31093;
        locals.var_mutau__blk862_dn4 = assign29010_e31093_d_n4;
        locals.var_mutau__blk862_dn6 = assign29010_e31093_d_n6;
        locals.var_mutau__blk862_dn7 = assign29010_e31093_d_n7;
        locals.var_mutau__blk862_dn8 = assign29010_e31093_d_n8;
        locals.var_mutau__blk862_dn9 = assign29010_e31093_d_n9;

        let (assign29020_e31130, assign29020_e31130_d_n4, assign29020_e31130_d_n6, assign29020_e31130_d_n7, assign29020_e31130_d_n8, assign29020_e31130_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29020_e31105: f64 = (locals.var_spsub_a__blk871 * locals.var_nu__blk861);
        let assign29020_e31107: f64 = (assign29020_e31105 * locals.var_spsub_tau__blk874);
        let assign29020_e31111: f64 = (locals.var_nu__blk861 / locals.var_mutau__blk862);
        let assign29020_e31113: f64 = (assign29020_e31111 * locals.var_spsub_tau__blk874);
        let assign29020_e31115: f64 = (assign29020_e31113 * locals.var_spsub_tau__blk874);
        let assign29020_e31117: f64 = (assign29020_e31115 * locals.var_spsub_c__blk873);
        let assign29020_e31120: f64 = (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873);
        let assign29020_e31122: f64 = (assign29020_e31120 * 0.3333333333333);
        let assign29020_e31124: f64 = (assign29020_e31122 - locals.var_spsub_a__blk871);
        let assign29020_e31125: f64 = (assign29020_e31117 * assign29020_e31124);
        let assign29020_e31126: f64 = (locals.var_mutau__blk862 + assign29020_e31125);
        let assign29020_e31127: f64 = (assign29020_e31107 / assign29020_e31126);
        let assign29020_e31128: f64 = (locals.var_spsub_eta__blk870 + assign29020_e31127);
        (assign29020_e31128, (locals.var_spsub_eta__blk870_dn4 + (((((((locals.var_spsub_a__blk871_dn4 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn4)) * locals.var_spsub_tau__blk874) + (assign29020_e31105 * locals.var_spsub_tau__blk874_dn4)) * assign29020_e31126) - (assign29020_e31107 * (locals.var_mutau__blk862_dn4 + (((((((((((locals.var_nu__blk861_dn4 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn4)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29020_e31111 * locals.var_spsub_tau__blk874_dn4)) * locals.var_spsub_tau__blk874) + (assign29020_e31113 * locals.var_spsub_tau__blk874_dn4)) * locals.var_spsub_c__blk873) + (assign29020_e31115 * locals.var_spsub_c__blk873_dn4)) * assign29020_e31124) + (assign29020_e31117 * ((((locals.var_spsub_c__blk873_dn4 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn4)) * 0.3333333333333) - locals.var_spsub_a__blk871_dn4)))))) / (assign29020_e31126 * assign29020_e31126))), (locals.var_spsub_eta__blk870_dn6 + (((((((locals.var_spsub_a__blk871_dn6 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn6)) * locals.var_spsub_tau__blk874) + (assign29020_e31105 * locals.var_spsub_tau__blk874_dn6)) * assign29020_e31126) - (assign29020_e31107 * (locals.var_mutau__blk862_dn6 + (((((((((((locals.var_nu__blk861_dn6 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn6)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29020_e31111 * locals.var_spsub_tau__blk874_dn6)) * locals.var_spsub_tau__blk874) + (assign29020_e31113 * locals.var_spsub_tau__blk874_dn6)) * locals.var_spsub_c__blk873) + (assign29020_e31115 * locals.var_spsub_c__blk873_dn6)) * assign29020_e31124) + (assign29020_e31117 * ((((locals.var_spsub_c__blk873_dn6 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn6)) * 0.3333333333333) - locals.var_spsub_a__blk871_dn6)))))) / (assign29020_e31126 * assign29020_e31126))), (locals.var_spsub_eta__blk870_dn7 + (((((((locals.var_spsub_a__blk871_dn7 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn7)) * locals.var_spsub_tau__blk874) + (assign29020_e31105 * locals.var_spsub_tau__blk874_dn7)) * assign29020_e31126) - (assign29020_e31107 * (locals.var_mutau__blk862_dn7 + (((((((((((locals.var_nu__blk861_dn7 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn7)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29020_e31111 * locals.var_spsub_tau__blk874_dn7)) * locals.var_spsub_tau__blk874) + (assign29020_e31113 * locals.var_spsub_tau__blk874_dn7)) * locals.var_spsub_c__blk873) + (assign29020_e31115 * locals.var_spsub_c__blk873_dn7)) * assign29020_e31124) + (assign29020_e31117 * ((((locals.var_spsub_c__blk873_dn7 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn7)) * 0.3333333333333) - locals.var_spsub_a__blk871_dn7)))))) / (assign29020_e31126 * assign29020_e31126))), (locals.var_spsub_eta__blk870_dn8 + (((((((locals.var_spsub_a__blk871_dn8 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn8)) * locals.var_spsub_tau__blk874) + (assign29020_e31105 * locals.var_spsub_tau__blk874_dn8)) * assign29020_e31126) - (assign29020_e31107 * (locals.var_mutau__blk862_dn8 + (((((((((((locals.var_nu__blk861_dn8 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn8)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29020_e31111 * locals.var_spsub_tau__blk874_dn8)) * locals.var_spsub_tau__blk874) + (assign29020_e31113 * locals.var_spsub_tau__blk874_dn8)) * locals.var_spsub_c__blk873) + (assign29020_e31115 * locals.var_spsub_c__blk873_dn8)) * assign29020_e31124) + (assign29020_e31117 * ((((locals.var_spsub_c__blk873_dn8 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn8)) * 0.3333333333333) - locals.var_spsub_a__blk871_dn8)))))) / (assign29020_e31126 * assign29020_e31126))), (locals.var_spsub_eta__blk870_dn9 + (((((((locals.var_spsub_a__blk871_dn9 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn9)) * locals.var_spsub_tau__blk874) + (assign29020_e31105 * locals.var_spsub_tau__blk874_dn9)) * assign29020_e31126) - (assign29020_e31107 * (locals.var_mutau__blk862_dn9 + (((((((((((locals.var_nu__blk861_dn9 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn9)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29020_e31111 * locals.var_spsub_tau__blk874_dn9)) * locals.var_spsub_tau__blk874) + (assign29020_e31113 * locals.var_spsub_tau__blk874_dn9)) * locals.var_spsub_c__blk873) + (assign29020_e31115 * locals.var_spsub_c__blk873_dn9)) * assign29020_e31124) + (assign29020_e31117 * ((((locals.var_spsub_c__blk873_dn9 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn9)) * 0.3333333333333) - locals.var_spsub_a__blk871_dn9)))))) / (assign29020_e31126 * assign29020_e31126))),)
    } else {
        (locals.var_spsub_y0__blk875, locals.var_spsub_y0__blk875_dn4, locals.var_spsub_y0__blk875_dn6, locals.var_spsub_y0__blk875_dn7, locals.var_spsub_y0__blk875_dn8, locals.var_spsub_y0__blk875_dn9,)
    }
};
        locals.var_spsub_y0__blk875 = assign29020_e31130;
        locals.var_spsub_y0__blk875_dn4 = assign29020_e31130_d_n4;
        locals.var_spsub_y0__blk875_dn6 = assign29020_e31130_d_n6;
        locals.var_spsub_y0__blk875_dn7 = assign29020_e31130_d_n7;
        locals.var_spsub_y0__blk875_dn8 = assign29020_e31130_d_n8;
        locals.var_spsub_y0__blk875_dn9 = assign29020_e31130_d_n9;

        let assign29030_e31133: f64 = if locals.var_spsub_y0__blk875 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1089 = assign29030_e31133;

        let (assign29040_e31147, assign29040_e31147_d_n4, assign29040_e31147_d_n6, assign29040_e31147_d_n7, assign29040_e31147_d_n8, assign29040_e31147_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) && (locals.var_guard1089 != 0.0)) {
        let assign29040_e31145: f64 = (locals.var_spsub_y0__blk875).exp();
        (assign29040_e31145, (assign29040_e31145 * locals.var_spsub_y0__blk875_dn4), (assign29040_e31145 * locals.var_spsub_y0__blk875_dn6), (assign29040_e31145 * locals.var_spsub_y0__blk875_dn7), (assign29040_e31145 * locals.var_spsub_y0__blk875_dn8), (assign29040_e31145 * locals.var_spsub_y0__blk875_dn9),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29040_e31147;
        locals.var_spsub_delta0__blk876_dn4 = assign29040_e31147_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29040_e31147_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29040_e31147_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29040_e31147_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29040_e31147_d_n9;

        let (assign29050_e31183, assign29050_e31183_d_n4, assign29050_e31183_d_n6, assign29050_e31183_d_n7, assign29050_e31183_d_n8, assign29050_e31183_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) && (locals.var_guard1089 == 0.0)) {
        let assign29050_e31163: f64 = (locals.var_spsub_y0__blk875 - 80.0);
        let assign29050_e31168: f64 = (locals.var_spsub_y0__blk875 - 80.0);
        let assign29050_e31169: f64 = (0.5 * assign29050_e31168);
        let assign29050_e31173: f64 = (locals.var_spsub_y0__blk875 - 80.0);
        let assign29050_e31175: f64 = (assign29050_e31173 * 0.3333333333333);
        let assign29050_e31176: f64 = (1.0 + assign29050_e31175);
        let assign29050_e31177: f64 = (assign29050_e31169 * assign29050_e31176);
        let assign29050_e31178: f64 = (1.0 + assign29050_e31177);
        let assign29050_e31179: f64 = (assign29050_e31163 * assign29050_e31178);
        let assign29050_e31180: f64 = (1.0 + assign29050_e31179);
        let assign29050_e31181: f64 = (5.54062e34 * assign29050_e31180);
        (assign29050_e31181, (5.54062e34 * ((locals.var_spsub_y0__blk875_dn4 * assign29050_e31178) + (assign29050_e31163 * (((0.5 * locals.var_spsub_y0__blk875_dn4) * assign29050_e31176) + (assign29050_e31169 * (locals.var_spsub_y0__blk875_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0__blk875_dn6 * assign29050_e31178) + (assign29050_e31163 * (((0.5 * locals.var_spsub_y0__blk875_dn6) * assign29050_e31176) + (assign29050_e31169 * (locals.var_spsub_y0__blk875_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0__blk875_dn7 * assign29050_e31178) + (assign29050_e31163 * (((0.5 * locals.var_spsub_y0__blk875_dn7) * assign29050_e31176) + (assign29050_e31169 * (locals.var_spsub_y0__blk875_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0__blk875_dn8 * assign29050_e31178) + (assign29050_e31163 * (((0.5 * locals.var_spsub_y0__blk875_dn8) * assign29050_e31176) + (assign29050_e31169 * (locals.var_spsub_y0__blk875_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0__blk875_dn9 * assign29050_e31178) + (assign29050_e31163 * (((0.5 * locals.var_spsub_y0__blk875_dn9) * assign29050_e31176) + (assign29050_e31169 * (locals.var_spsub_y0__blk875_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29050_e31183;
        locals.var_spsub_delta0__blk876_dn4 = assign29050_e31183_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29050_e31183_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29050_e31183_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29050_e31183_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29050_e31183_d_n9;

        let (assign29060_e31196, assign29060_e31196_d_n4, assign29060_e31196_d_n6, assign29060_e31196_d_n7, assign29060_e31196_d_n8, assign29060_e31196_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29060_e31194: f64 = (1.0 / locals.var_spsub_delta0__blk876);
        (assign29060_e31194, (-(locals.var_spsub_delta0__blk876_dn4 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn6 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn7 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn8 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn9 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))),)
    } else {
        (locals.var_spsub_delta1__blk877, locals.var_spsub_delta1__blk877_dn4, locals.var_spsub_delta1__blk877_dn6, locals.var_spsub_delta1__blk877_dn7, locals.var_spsub_delta1__blk877_dn8, locals.var_spsub_delta1__blk877_dn9,)
    }
};
        locals.var_spsub_delta1__blk877 = assign29060_e31196;
        locals.var_spsub_delta1__blk877_dn4 = assign29060_e31196_d_n4;
        locals.var_spsub_delta1__blk877_dn6 = assign29060_e31196_d_n6;
        locals.var_spsub_delta1__blk877_dn7 = assign29060_e31196_d_n7;
        locals.var_spsub_delta1__blk877_dn8 = assign29060_e31196_d_n8;
        locals.var_spsub_delta1__blk877_dn9 = assign29060_e31196_d_n9;

        let (assign29070_e31213, assign29070_e31213_d_n4, assign29070_e31213_d_n6, assign29070_e31213_d_n7, assign29070_e31213_d_n8, assign29070_e31213_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29070_e31209: f64 = (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875);
        let assign29070_e31210: f64 = (2.0 + assign29070_e31209);
        let assign29070_e31211: f64 = (1.0 / assign29070_e31210);
        (assign29070_e31211, (-(((locals.var_spsub_y0__blk875_dn4 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn4)) / (assign29070_e31210 * assign29070_e31210))), (-(((locals.var_spsub_y0__blk875_dn6 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn6)) / (assign29070_e31210 * assign29070_e31210))), (-(((locals.var_spsub_y0__blk875_dn7 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn7)) / (assign29070_e31210 * assign29070_e31210))), (-(((locals.var_spsub_y0__blk875_dn8 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn8)) / (assign29070_e31210 * assign29070_e31210))), (-(((locals.var_spsub_y0__blk875_dn9 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn9)) / (assign29070_e31210 * assign29070_e31210))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29070_e31213;
        locals.var_spsub_temp__blk863_dn4 = assign29070_e31213_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29070_e31213_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29070_e31213_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29070_e31213_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29070_e31213_d_n9;

        let (assign29080_e31228, assign29080_e31228_d_n4, assign29080_e31228_d_n6, assign29080_e31228_d_n7, assign29080_e31228_d_n8, assign29080_e31228_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29080_e31224: f64 = (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875);
        let assign29080_e31226: f64 = (assign29080_e31224 * locals.var_spsub_temp__blk863);
        (assign29080_e31226, ((((locals.var_spsub_y0__blk875_dn4 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn4)) * locals.var_spsub_temp__blk863) + (assign29080_e31224 * locals.var_spsub_temp__blk863_dn4)), ((((locals.var_spsub_y0__blk875_dn6 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn6)) * locals.var_spsub_temp__blk863) + (assign29080_e31224 * locals.var_spsub_temp__blk863_dn6)), ((((locals.var_spsub_y0__blk875_dn7 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn7)) * locals.var_spsub_temp__blk863) + (assign29080_e31224 * locals.var_spsub_temp__blk863_dn7)), ((((locals.var_spsub_y0__blk875_dn8 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn8)) * locals.var_spsub_temp__blk863) + (assign29080_e31224 * locals.var_spsub_temp__blk863_dn8)), ((((locals.var_spsub_y0__blk875_dn9 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn9)) * locals.var_spsub_temp__blk863) + (assign29080_e31224 * locals.var_spsub_temp__blk863_dn9)),)
    } else {
        (locals.var_spsub_xi0__blk878, locals.var_spsub_xi0__blk878_dn4, locals.var_spsub_xi0__blk878_dn6, locals.var_spsub_xi0__blk878_dn7, locals.var_spsub_xi0__blk878_dn8, locals.var_spsub_xi0__blk878_dn9,)
    }
};
        locals.var_spsub_xi0__blk878 = assign29080_e31228;
        locals.var_spsub_xi0__blk878_dn4 = assign29080_e31228_d_n4;
        locals.var_spsub_xi0__blk878_dn6 = assign29080_e31228_d_n6;
        locals.var_spsub_xi0__blk878_dn7 = assign29080_e31228_d_n7;
        locals.var_spsub_xi0__blk878_dn8 = assign29080_e31228_d_n8;
        locals.var_spsub_xi0__blk878_dn9 = assign29080_e31228_d_n9;

        let (assign29090_e31245, assign29090_e31245_d_n4, assign29090_e31245_d_n6, assign29090_e31245_d_n7, assign29090_e31245_d_n8, assign29090_e31245_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29090_e31240: f64 = (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863);
        let assign29090_e31242: f64 = (assign29090_e31240 * locals.var_spsub_temp__blk863);
        let assign29090_e31243: f64 = (4.0 * assign29090_e31242);
        (assign29090_e31243, (4.0 * ((((locals.var_spsub_y0__blk875_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863_dn4)) * locals.var_spsub_temp__blk863) + (assign29090_e31240 * locals.var_spsub_temp__blk863_dn4))), (4.0 * ((((locals.var_spsub_y0__blk875_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863_dn6)) * locals.var_spsub_temp__blk863) + (assign29090_e31240 * locals.var_spsub_temp__blk863_dn6))), (4.0 * ((((locals.var_spsub_y0__blk875_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863_dn7)) * locals.var_spsub_temp__blk863) + (assign29090_e31240 * locals.var_spsub_temp__blk863_dn7))), (4.0 * ((((locals.var_spsub_y0__blk875_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863_dn8)) * locals.var_spsub_temp__blk863) + (assign29090_e31240 * locals.var_spsub_temp__blk863_dn8))), (4.0 * ((((locals.var_spsub_y0__blk875_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863_dn9)) * locals.var_spsub_temp__blk863) + (assign29090_e31240 * locals.var_spsub_temp__blk863_dn9))),)
    } else {
        (locals.var_spsub_xi1__blk879, locals.var_spsub_xi1__blk879_dn4, locals.var_spsub_xi1__blk879_dn6, locals.var_spsub_xi1__blk879_dn7, locals.var_spsub_xi1__blk879_dn8, locals.var_spsub_xi1__blk879_dn9,)
    }
};
        locals.var_spsub_xi1__blk879 = assign29090_e31245;
        locals.var_spsub_xi1__blk879_dn4 = assign29090_e31245_d_n4;
        locals.var_spsub_xi1__blk879_dn6 = assign29090_e31245_d_n6;
        locals.var_spsub_xi1__blk879_dn7 = assign29090_e31245_d_n7;
        locals.var_spsub_xi1__blk879_dn8 = assign29090_e31245_d_n8;
        locals.var_spsub_xi1__blk879_dn9 = assign29090_e31245_d_n9;

        let (assign29100_e31266, assign29100_e31266_d_n4, assign29100_e31266_d_n6, assign29100_e31266_d_n7, assign29100_e31266_d_n8, assign29100_e31266_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29100_e31256: f64 = (8.0 * locals.var_spsub_temp__blk863);
        let assign29100_e31259: f64 = (12.0 * locals.var_spsub_xi0__blk878);
        let assign29100_e31260: f64 = (assign29100_e31256 - assign29100_e31259);
        let assign29100_e31262: f64 = (assign29100_e31260 * locals.var_spsub_temp__blk863);
        let assign29100_e31264: f64 = (assign29100_e31262 * locals.var_spsub_temp__blk863);
        (assign29100_e31264, ((((((8.0 * locals.var_spsub_temp__blk863_dn4) - (12.0 * locals.var_spsub_xi0__blk878_dn4)) * locals.var_spsub_temp__blk863) + (assign29100_e31260 * locals.var_spsub_temp__blk863_dn4)) * locals.var_spsub_temp__blk863) + (assign29100_e31262 * locals.var_spsub_temp__blk863_dn4)), ((((((8.0 * locals.var_spsub_temp__blk863_dn6) - (12.0 * locals.var_spsub_xi0__blk878_dn6)) * locals.var_spsub_temp__blk863) + (assign29100_e31260 * locals.var_spsub_temp__blk863_dn6)) * locals.var_spsub_temp__blk863) + (assign29100_e31262 * locals.var_spsub_temp__blk863_dn6)), ((((((8.0 * locals.var_spsub_temp__blk863_dn7) - (12.0 * locals.var_spsub_xi0__blk878_dn7)) * locals.var_spsub_temp__blk863) + (assign29100_e31260 * locals.var_spsub_temp__blk863_dn7)) * locals.var_spsub_temp__blk863) + (assign29100_e31262 * locals.var_spsub_temp__blk863_dn7)), ((((((8.0 * locals.var_spsub_temp__blk863_dn8) - (12.0 * locals.var_spsub_xi0__blk878_dn8)) * locals.var_spsub_temp__blk863) + (assign29100_e31260 * locals.var_spsub_temp__blk863_dn8)) * locals.var_spsub_temp__blk863) + (assign29100_e31262 * locals.var_spsub_temp__blk863_dn8)), ((((((8.0 * locals.var_spsub_temp__blk863_dn9) - (12.0 * locals.var_spsub_xi0__blk878_dn9)) * locals.var_spsub_temp__blk863) + (assign29100_e31260 * locals.var_spsub_temp__blk863_dn9)) * locals.var_spsub_temp__blk863) + (assign29100_e31262 * locals.var_spsub_temp__blk863_dn9)),)
    } else {
        (locals.var_spsub_xi2__blk880, locals.var_spsub_xi2__blk880_dn4, locals.var_spsub_xi2__blk880_dn6, locals.var_spsub_xi2__blk880_dn7, locals.var_spsub_xi2__blk880_dn8, locals.var_spsub_xi2__blk880_dn9,)
    }
};
        locals.var_spsub_xi2__blk880 = assign29100_e31266;
        locals.var_spsub_xi2__blk880_dn4 = assign29100_e31266_d_n4;
        locals.var_spsub_xi2__blk880_dn6 = assign29100_e31266_d_n6;
        locals.var_spsub_xi2__blk880_dn7 = assign29100_e31266_d_n7;
        locals.var_spsub_xi2__blk880_dn8 = assign29100_e31266_d_n8;
        locals.var_spsub_xi2__blk880_dn9 = assign29100_e31266_d_n9;

        let (assign29110_e31279, assign29110_e31279_d_n4, assign29110_e31279_d_n6, assign29110_e31279_d_n7, assign29110_e31279_d_n8, assign29110_e31279_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29110_e31277: f64 = (locals.var_spsub_yg__blk868 - locals.var_spsub_y0__blk875);
        (assign29110_e31277, (locals.var_spsub_yg__blk868_dn4 - locals.var_spsub_y0__blk875_dn4), (locals.var_spsub_yg__blk868_dn6 - locals.var_spsub_y0__blk875_dn6), (locals.var_spsub_yg__blk868_dn7 - locals.var_spsub_y0__blk875_dn7), (locals.var_spsub_yg__blk868_dn8 - locals.var_spsub_y0__blk875_dn8), (locals.var_spsub_yg__blk868_dn9 - locals.var_spsub_y0__blk875_dn9),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29110_e31279;
        locals.var_spsub_temp__blk863_dn4 = assign29110_e31279_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29110_e31279_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29110_e31279_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29110_e31279_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29110_e31279_d_n9;

        let (assign29120_e31292, assign29120_e31292_d_n4, assign29120_e31292_d_n6, assign29120_e31292_d_n7, assign29120_e31292_d_n8, assign29120_e31292_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29120_e31290: f64 = (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877);
        (assign29120_e31290, ((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_delta1__blk877) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877_dn4)), ((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_delta1__blk877) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877_dn6)), ((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_delta1__blk877) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877_dn7)), ((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_delta1__blk877) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877_dn8)), ((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_delta1__blk877) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877_dn9)),)
    } else {
        (locals.var_spsub_temp1__blk864, locals.var_spsub_temp1__blk864_dn4, locals.var_spsub_temp1__blk864_dn6, locals.var_spsub_temp1__blk864_dn7, locals.var_spsub_temp1__blk864_dn8, locals.var_spsub_temp1__blk864_dn9,)
    }
};
        locals.var_spsub_temp1__blk864 = assign29120_e31292;
        locals.var_spsub_temp1__blk864_dn4 = assign29120_e31292_d_n4;
        locals.var_spsub_temp1__blk864_dn6 = assign29120_e31292_d_n6;
        locals.var_spsub_temp1__blk864_dn7 = assign29120_e31292_d_n7;
        locals.var_spsub_temp1__blk864_dn8 = assign29120_e31292_d_n8;
        locals.var_spsub_temp1__blk864_dn9 = assign29120_e31292_d_n9;

        let (assign29130_e31319, assign29130_e31319_d_n4, assign29130_e31319_d_n6, assign29130_e31319_d_n7, assign29130_e31319_d_n8, assign29130_e31319_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29130_e31303: f64 = (2.0 * locals.var_spsub_temp__blk863);
        let assign29130_e31307: f64 = (locals.var_spsub_delta0__blk876 - 1.0);
        let assign29130_e31309: f64 = (assign29130_e31307 - locals.var_spsub_temp1__blk864);
        let assign29130_e31313: f64 = (1.0 - locals.var_spsub_xi1__blk879);
        let assign29130_e31314: f64 = (locals.var_spsub_delta__blk867 * assign29130_e31313);
        let assign29130_e31315: f64 = (assign29130_e31309 + assign29130_e31314);
        let assign29130_e31316: f64 = (locals.var_gfsub2 * assign29130_e31315);
        let assign29130_e31317: f64 = (assign29130_e31303 + assign29130_e31316);
        (assign29130_e31317, ((2.0 * locals.var_spsub_temp__blk863_dn4) + ((locals.var_gfsub2_dn4 * assign29130_e31315) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn4 - locals.var_spsub_temp1__blk864_dn4) + ((locals.var_spsub_delta__blk867_dn4 * assign29130_e31313) + (locals.var_spsub_delta__blk867 * (-locals.var_spsub_xi1__blk879_dn4))))))), ((2.0 * locals.var_spsub_temp__blk863_dn6) + ((locals.var_gfsub2_dn6 * assign29130_e31315) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn6 - locals.var_spsub_temp1__blk864_dn6) + ((locals.var_spsub_delta__blk867_dn6 * assign29130_e31313) + (locals.var_spsub_delta__blk867 * (-locals.var_spsub_xi1__blk879_dn6))))))), ((2.0 * locals.var_spsub_temp__blk863_dn7) + ((locals.var_gfsub2_dn7 * assign29130_e31315) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn7 - locals.var_spsub_temp1__blk864_dn7) + ((locals.var_spsub_delta__blk867_dn7 * assign29130_e31313) + (locals.var_spsub_delta__blk867 * (-locals.var_spsub_xi1__blk879_dn7))))))), ((2.0 * locals.var_spsub_temp__blk863_dn8) + ((locals.var_gfsub2_dn8 * assign29130_e31315) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn8 - locals.var_spsub_temp1__blk864_dn8) + ((locals.var_spsub_delta__blk867_dn8 * assign29130_e31313) + (locals.var_spsub_delta__blk867 * (-locals.var_spsub_xi1__blk879_dn8))))))), ((2.0 * locals.var_spsub_temp__blk863_dn9) + ((locals.var_gfsub2_dn9 * assign29130_e31315) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn9 - locals.var_spsub_temp1__blk864_dn9) + ((locals.var_spsub_delta__blk867_dn9 * assign29130_e31313) + (locals.var_spsub_delta__blk867 * (-locals.var_spsub_xi1__blk879_dn9))))))),)
    } else {
        (locals.var_spsub_pc__blk881, locals.var_spsub_pc__blk881_dn4, locals.var_spsub_pc__blk881_dn6, locals.var_spsub_pc__blk881_dn7, locals.var_spsub_pc__blk881_dn8, locals.var_spsub_pc__blk881_dn9,)
    }
};
        locals.var_spsub_pc__blk881 = assign29130_e31319;
        locals.var_spsub_pc__blk881_dn4 = assign29130_e31319_d_n4;
        locals.var_spsub_pc__blk881_dn6 = assign29130_e31319_d_n6;
        locals.var_spsub_pc__blk881_dn7 = assign29130_e31319_d_n7;
        locals.var_spsub_pc__blk881_dn8 = assign29130_e31319_d_n8;
        locals.var_spsub_pc__blk881_dn9 = assign29130_e31319_d_n9;

        let (assign29140_e31350, assign29140_e31350_d_n4, assign29140_e31350_d_n6, assign29140_e31350_d_n7, assign29140_e31350_d_n8, assign29140_e31350_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29140_e31330: f64 = (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863);
        let assign29140_e31334: f64 = (locals.var_spsub_delta0__blk876 - locals.var_spsub_y0__blk875);
        let assign29140_e31336: f64 = (assign29140_e31334 - 1.0);
        let assign29140_e31338: f64 = (assign29140_e31336 + locals.var_spsub_temp1__blk864);
        let assign29140_e31342: f64 = (locals.var_spsub_y0__blk875 - 1.0);
        let assign29140_e31344: f64 = (assign29140_e31342 - locals.var_spsub_xi0__blk878);
        let assign29140_e31345: f64 = (locals.var_spsub_delta__blk867 * assign29140_e31344);
        let assign29140_e31346: f64 = (assign29140_e31338 + assign29140_e31345);
        let assign29140_e31347: f64 = (locals.var_gfsub2 * assign29140_e31346);
        let assign29140_e31348: f64 = (assign29140_e31330 - assign29140_e31347);
        (assign29140_e31348, (((locals.var_spsub_temp__blk863_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn4)) - ((locals.var_gfsub2_dn4 * assign29140_e31346) + (locals.var_gfsub2 * (((locals.var_spsub_delta0__blk876_dn4 - locals.var_spsub_y0__blk875_dn4) + locals.var_spsub_temp1__blk864_dn4) + ((locals.var_spsub_delta__blk867_dn4 * assign29140_e31344) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_y0__blk875_dn4 - locals.var_spsub_xi0__blk878_dn4))))))), (((locals.var_spsub_temp__blk863_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn6)) - ((locals.var_gfsub2_dn6 * assign29140_e31346) + (locals.var_gfsub2 * (((locals.var_spsub_delta0__blk876_dn6 - locals.var_spsub_y0__blk875_dn6) + locals.var_spsub_temp1__blk864_dn6) + ((locals.var_spsub_delta__blk867_dn6 * assign29140_e31344) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_y0__blk875_dn6 - locals.var_spsub_xi0__blk878_dn6))))))), (((locals.var_spsub_temp__blk863_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn7)) - ((locals.var_gfsub2_dn7 * assign29140_e31346) + (locals.var_gfsub2 * (((locals.var_spsub_delta0__blk876_dn7 - locals.var_spsub_y0__blk875_dn7) + locals.var_spsub_temp1__blk864_dn7) + ((locals.var_spsub_delta__blk867_dn7 * assign29140_e31344) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_y0__blk875_dn7 - locals.var_spsub_xi0__blk878_dn7))))))), (((locals.var_spsub_temp__blk863_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn8)) - ((locals.var_gfsub2_dn8 * assign29140_e31346) + (locals.var_gfsub2 * (((locals.var_spsub_delta0__blk876_dn8 - locals.var_spsub_y0__blk875_dn8) + locals.var_spsub_temp1__blk864_dn8) + ((locals.var_spsub_delta__blk867_dn8 * assign29140_e31344) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_y0__blk875_dn8 - locals.var_spsub_xi0__blk878_dn8))))))), (((locals.var_spsub_temp__blk863_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn9)) - ((locals.var_gfsub2_dn9 * assign29140_e31346) + (locals.var_gfsub2 * (((locals.var_spsub_delta0__blk876_dn9 - locals.var_spsub_y0__blk875_dn9) + locals.var_spsub_temp1__blk864_dn9) + ((locals.var_spsub_delta__blk867_dn9 * assign29140_e31344) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_y0__blk875_dn9 - locals.var_spsub_xi0__blk878_dn9))))))),)
    } else {
        (locals.var_spsub_qc__blk882, locals.var_spsub_qc__blk882_dn4, locals.var_spsub_qc__blk882_dn6, locals.var_spsub_qc__blk882_dn7, locals.var_spsub_qc__blk882_dn8, locals.var_spsub_qc__blk882_dn9,)
    }
};
        locals.var_spsub_qc__blk882 = assign29140_e31350;
        locals.var_spsub_qc__blk882_dn4 = assign29140_e31350_d_n4;
        locals.var_spsub_qc__blk882_dn6 = assign29140_e31350_d_n6;
        locals.var_spsub_qc__blk882_dn7 = assign29140_e31350_d_n7;
        locals.var_spsub_qc__blk882_dn8 = assign29140_e31350_d_n8;
        locals.var_spsub_qc__blk882_dn9 = assign29140_e31350_d_n9;

    }

    pub(super) fn stamp_transient_block_74(
        locals: &mut StampLocals,
    ) {
        let (assign29150_e31371, assign29150_e31371_d_n4, assign29150_e31371_d_n6, assign29150_e31371_d_n7, assign29150_e31371_d_n8, assign29150_e31371_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29150_e31363: f64 = (locals.var_spsub_delta0__blk876 + locals.var_spsub_temp1__blk864);
        let assign29150_e31366: f64 = (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880);
        let assign29150_e31367: f64 = (assign29150_e31363 - assign29150_e31366);
        let assign29150_e31368: f64 = (locals.var_gfsub2 * assign29150_e31367);
        let assign29150_e31369: f64 = (2.0 - assign29150_e31368);
        (assign29150_e31369, (-((locals.var_gfsub2_dn4 * assign29150_e31367) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn4 + locals.var_spsub_temp1__blk864_dn4) - ((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn4)))))), (-((locals.var_gfsub2_dn6 * assign29150_e31367) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn6 + locals.var_spsub_temp1__blk864_dn6) - ((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn6)))))), (-((locals.var_gfsub2_dn7 * assign29150_e31367) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn7 + locals.var_spsub_temp1__blk864_dn7) - ((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn7)))))), (-((locals.var_gfsub2_dn8 * assign29150_e31367) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn8 + locals.var_spsub_temp1__blk864_dn8) - ((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn8)))))), (-((locals.var_gfsub2_dn9 * assign29150_e31367) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn9 + locals.var_spsub_temp1__blk864_dn9) - ((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn9)))))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29150_e31371;
        locals.var_spsub_temp__blk863_dn4 = assign29150_e31371_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29150_e31371_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29150_e31371_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29150_e31371_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29150_e31371_d_n9;

        let (assign29160_e31390, assign29160_e31390_d_n4, assign29160_e31390_d_n6, assign29160_e31390_d_n7, assign29160_e31390_d_n8, assign29160_e31390_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29160_e31382: f64 = (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881);
        let assign29160_e31386: f64 = (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863);
        let assign29160_e31387: f64 = (2.0 * assign29160_e31386);
        let assign29160_e31388: f64 = (assign29160_e31382 - assign29160_e31387);
        (assign29160_e31388, (((locals.var_spsub_pc__blk881_dn4 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn4)) - (2.0 * ((locals.var_spsub_qc__blk882_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn4)))), (((locals.var_spsub_pc__blk881_dn6 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn6)) - (2.0 * ((locals.var_spsub_qc__blk882_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn6)))), (((locals.var_spsub_pc__blk881_dn7 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn7)) - (2.0 * ((locals.var_spsub_qc__blk882_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn7)))), (((locals.var_spsub_pc__blk881_dn8 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn8)) - (2.0 * ((locals.var_spsub_qc__blk882_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn8)))), (((locals.var_spsub_pc__blk881_dn9 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn9)) - (2.0 * ((locals.var_spsub_qc__blk882_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn9)))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29160_e31390;
        locals.var_spsub_temp__blk863_dn4 = assign29160_e31390_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29160_e31390_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29160_e31390_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29160_e31390_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29160_e31390_d_n9;

        let (assign29170_e31411, assign29170_e31411_d_n4, assign29170_e31411_d_n6, assign29170_e31411_d_n7, assign29170_e31411_d_n8, assign29170_e31411_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29170_e31400: f64 = (-locals.var_spsub_y0__blk875);
        let assign29170_e31405: f64 = (locals.var_spsub_temp__blk863).sqrt();
        let assign29170_e31406: f64 = (locals.var_spsub_pc__blk881 + assign29170_e31405);
        let assign29170_e31407: f64 = (locals.var_spsub_qc__blk882 / assign29170_e31406);
        let assign29170_e31408: f64 = (2.0 * assign29170_e31407);
        let assign29170_e31409: f64 = (assign29170_e31400 - assign29170_e31408);
        (assign29170_e31409, ((-locals.var_spsub_y0__blk875_dn4) - (2.0 * (((locals.var_spsub_qc__blk882_dn4 * assign29170_e31406) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn4 + (locals.var_spsub_temp__blk863_dn4 / (2.0 * assign29170_e31405))))) / (assign29170_e31406 * assign29170_e31406)))), ((-locals.var_spsub_y0__blk875_dn6) - (2.0 * (((locals.var_spsub_qc__blk882_dn6 * assign29170_e31406) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn6 + (locals.var_spsub_temp__blk863_dn6 / (2.0 * assign29170_e31405))))) / (assign29170_e31406 * assign29170_e31406)))), ((-locals.var_spsub_y0__blk875_dn7) - (2.0 * (((locals.var_spsub_qc__blk882_dn7 * assign29170_e31406) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn7 + (locals.var_spsub_temp__blk863_dn7 / (2.0 * assign29170_e31405))))) / (assign29170_e31406 * assign29170_e31406)))), ((-locals.var_spsub_y0__blk875_dn8) - (2.0 * (((locals.var_spsub_qc__blk882_dn8 * assign29170_e31406) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn8 + (locals.var_spsub_temp__blk863_dn8 / (2.0 * assign29170_e31405))))) / (assign29170_e31406 * assign29170_e31406)))), ((-locals.var_spsub_y0__blk875_dn9) - (2.0 * (((locals.var_spsub_qc__blk882_dn9 * assign29170_e31406) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn9 + (locals.var_spsub_temp__blk863_dn9 / (2.0 * assign29170_e31405))))) / (assign29170_e31406 * assign29170_e31406)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign29170_e31411;
        locals.var_temp3_dn4 = assign29170_e31411_d_n4;
        locals.var_temp3_dn6 = assign29170_e31411_d_n6;
        locals.var_temp3_dn7 = assign29170_e31411_d_n7;
        locals.var_temp3_dn8 = assign29170_e31411_d_n8;
        locals.var_temp3_dn9 = assign29170_e31411_d_n9;

        let (assign29180_e31429, assign29180_e31429_d_n4, assign29180_e31429_d_n6, assign29180_e31429_d_n7, assign29180_e31429_d_n8, assign29180_e31429_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29180_e31425: f64 = (locals.var_gfsub * 0.732464877560822);
        let assign29180_e31426: f64 = (1.25 + assign29180_e31425);
        let assign29180_e31427: f64 = (1.0 / assign29180_e31426);
        (assign29180_e31427, (-((locals.var_gfsub_dn4 * 0.732464877560822) / (assign29180_e31426 * assign29180_e31426))), (-((locals.var_gfsub_dn6 * 0.732464877560822) / (assign29180_e31426 * assign29180_e31426))), (-((locals.var_gfsub_dn7 * 0.732464877560822) / (assign29180_e31426 * assign29180_e31426))), (-((locals.var_gfsub_dn8 * 0.732464877560822) / (assign29180_e31426 * assign29180_e31426))), (-((locals.var_gfsub_dn9 * 0.732464877560822) / (assign29180_e31426 * assign29180_e31426))),)
    } else {
        (locals.var_spsub_xg1__blk883, locals.var_spsub_xg1__blk883_dn4, locals.var_spsub_xg1__blk883_dn6, locals.var_spsub_xg1__blk883_dn7, locals.var_spsub_xg1__blk883_dn8, locals.var_spsub_xg1__blk883_dn9,)
    }
};
        locals.var_spsub_xg1__blk883 = assign29180_e31429;
        locals.var_spsub_xg1__blk883_dn4 = assign29180_e31429_d_n4;
        locals.var_spsub_xg1__blk883_dn6 = assign29180_e31429_d_n6;
        locals.var_spsub_xg1__blk883_dn7 = assign29180_e31429_d_n7;
        locals.var_spsub_xg1__blk883_dn8 = assign29180_e31429_d_n8;
        locals.var_spsub_xg1__blk883_dn9 = assign29180_e31429_d_n9;

        let (assign29190_e31449, assign29190_e31449_d_n4, assign29190_e31449_d_n6, assign29190_e31449_d_n7, assign29190_e31449_d_n8, assign29190_e31449_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29190_e31441: f64 = (1.25 * locals.var_xisub);
        let assign29190_e31443: f64 = (assign29190_e31441 * locals.var_spsub_xg1__blk883);
        let assign29190_e31445: f64 = (assign29190_e31443 - 1.0);
        let assign29190_e31447: f64 = (assign29190_e31445 * locals.var_spsub_xg1__blk883);
        (assign29190_e31447, (((((1.25 * locals.var_xisub_dn4) * locals.var_spsub_xg1__blk883) + (assign29190_e31441 * locals.var_spsub_xg1__blk883_dn4)) * locals.var_spsub_xg1__blk883) + (assign29190_e31445 * locals.var_spsub_xg1__blk883_dn4)), (((((1.25 * locals.var_xisub_dn6) * locals.var_spsub_xg1__blk883) + (assign29190_e31441 * locals.var_spsub_xg1__blk883_dn6)) * locals.var_spsub_xg1__blk883) + (assign29190_e31445 * locals.var_spsub_xg1__blk883_dn6)), (((((1.25 * locals.var_xisub_dn7) * locals.var_spsub_xg1__blk883) + (assign29190_e31441 * locals.var_spsub_xg1__blk883_dn7)) * locals.var_spsub_xg1__blk883) + (assign29190_e31445 * locals.var_spsub_xg1__blk883_dn7)), (((((1.25 * locals.var_xisub_dn8) * locals.var_spsub_xg1__blk883) + (assign29190_e31441 * locals.var_spsub_xg1__blk883_dn8)) * locals.var_spsub_xg1__blk883) + (assign29190_e31445 * locals.var_spsub_xg1__blk883_dn8)), (((((1.25 * locals.var_xisub_dn9) * locals.var_spsub_xg1__blk883) + (assign29190_e31441 * locals.var_spsub_xg1__blk883_dn9)) * locals.var_spsub_xg1__blk883) + (assign29190_e31445 * locals.var_spsub_xg1__blk883_dn9)),)
    } else {
        (locals.var_spsub_a_fac__blk884, locals.var_spsub_a_fac__blk884_dn4, locals.var_spsub_a_fac__blk884_dn6, locals.var_spsub_a_fac__blk884_dn7, locals.var_spsub_a_fac__blk884_dn8, locals.var_spsub_a_fac__blk884_dn9,)
    }
};
        locals.var_spsub_a_fac__blk884 = assign29190_e31449;
        locals.var_spsub_a_fac__blk884_dn4 = assign29190_e31449_d_n4;
        locals.var_spsub_a_fac__blk884_dn6 = assign29190_e31449_d_n6;
        locals.var_spsub_a_fac__blk884_dn7 = assign29190_e31449_d_n7;
        locals.var_spsub_a_fac__blk884_dn8 = assign29190_e31449_d_n8;
        locals.var_spsub_a_fac__blk884_dn9 = assign29190_e31449_d_n9;

        let (assign29200_e31469, assign29200_e31469_d_n4, assign29200_e31469_d_n6, assign29200_e31469_d_n7, assign29200_e31469_d_n8, assign29200_e31469_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29200_e31461: f64 = (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub);
        let assign29200_e31465: f64 = (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866);
        let assign29200_e31466: f64 = (1.0 + assign29200_e31465);
        let assign29200_e31467: f64 = (assign29200_e31461 * assign29200_e31466);
        (assign29200_e31467, ((((locals.var_spsub_xgb__blk866_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn4)) * assign29200_e31466) + (assign29200_e31461 * ((locals.var_spsub_a_fac__blk884_dn4 * locals.var_spsub_xgb__blk866) + (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866_dn4)))), ((((locals.var_spsub_xgb__blk866_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn6)) * assign29200_e31466) + (assign29200_e31461 * ((locals.var_spsub_a_fac__blk884_dn6 * locals.var_spsub_xgb__blk866) + (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866_dn6)))), ((((locals.var_spsub_xgb__blk866_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn7)) * assign29200_e31466) + (assign29200_e31461 * ((locals.var_spsub_a_fac__blk884_dn7 * locals.var_spsub_xgb__blk866) + (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866_dn7)))), ((((locals.var_spsub_xgb__blk866_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn8)) * assign29200_e31466) + (assign29200_e31461 * ((locals.var_spsub_a_fac__blk884_dn8 * locals.var_spsub_xgb__blk866) + (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866_dn8)))), ((((locals.var_spsub_xgb__blk866_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn9)) * assign29200_e31466) + (assign29200_e31461 * ((locals.var_spsub_a_fac__blk884_dn9 * locals.var_spsub_xgb__blk866) + (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866_dn9)))),)
    } else {
        (locals.var_spsub_xbar__blk885, locals.var_spsub_xbar__blk885_dn4, locals.var_spsub_xbar__blk885_dn6, locals.var_spsub_xbar__blk885_dn7, locals.var_spsub_xbar__blk885_dn8, locals.var_spsub_xbar__blk885_dn9,)
    }
};
        locals.var_spsub_xbar__blk885 = assign29200_e31469;
        locals.var_spsub_xbar__blk885_dn4 = assign29200_e31469_d_n4;
        locals.var_spsub_xbar__blk885_dn6 = assign29200_e31469_d_n6;
        locals.var_spsub_xbar__blk885_dn7 = assign29200_e31469_d_n7;
        locals.var_spsub_xbar__blk885_dn8 = assign29200_e31469_d_n8;
        locals.var_spsub_xbar__blk885_dn9 = assign29200_e31469_d_n9;

        let assign29210_e31471: f64 = (-locals.var_spsub_xbar__blk885);
        let assign29210_e31473: f64 = (-80.0);
        let assign29210_e31474: f64 = if assign29210_e31471 > assign29210_e31473 { 1.0 } else { 0.0 };
        locals.var_guard1090 = assign29210_e31474;

        let (assign29220_e31490, assign29220_e31490_d_n4, assign29220_e31490_d_n6, assign29220_e31490_d_n7, assign29220_e31490_d_n8, assign29220_e31490_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1090 != 0.0)) {
        let assign29220_e31487: f64 = (-locals.var_spsub_xbar__blk885);
        let assign29220_e31488: f64 = (assign29220_e31487).exp();
        (assign29220_e31488, (assign29220_e31488 * (-locals.var_spsub_xbar__blk885_dn4)), (assign29220_e31488 * (-locals.var_spsub_xbar__blk885_dn6)), (assign29220_e31488 * (-locals.var_spsub_xbar__blk885_dn7)), (assign29220_e31488 * (-locals.var_spsub_xbar__blk885_dn8)), (assign29220_e31488 * (-locals.var_spsub_xbar__blk885_dn9)),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29220_e31490;
        locals.var_spsub_temp__blk863_dn4 = assign29220_e31490_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29220_e31490_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29220_e31490_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29220_e31490_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29220_e31490_d_n9;

        let (assign29230_e31533, assign29230_e31533_d_n4, assign29230_e31533_d_n6, assign29230_e31533_d_n7, assign29230_e31533_d_n8, assign29230_e31533_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1090 == 0.0)) {
        let assign29230_e31506: f64 = (-locals.var_spsub_xbar__blk885);
        let assign29230_e31507: f64 = (-assign29230_e31506);
        let assign29230_e31509: f64 = (assign29230_e31507 - 80.0);
        let assign29230_e31513: f64 = (-locals.var_spsub_xbar__blk885);
        let assign29230_e31514: f64 = (-assign29230_e31513);
        let assign29230_e31516: f64 = (assign29230_e31514 - 80.0);
        let assign29230_e31517: f64 = (0.5 * assign29230_e31516);
        let assign29230_e31520: f64 = (-locals.var_spsub_xbar__blk885);
        let assign29230_e31521: f64 = (-assign29230_e31520);
        let assign29230_e31523: f64 = (assign29230_e31521 - 80.0);
        let assign29230_e31525: f64 = (assign29230_e31523 * 0.3333333333333);
        let assign29230_e31526: f64 = (1.0 + assign29230_e31525);
        let assign29230_e31527: f64 = (assign29230_e31517 * assign29230_e31526);
        let assign29230_e31528: f64 = (1.0 + assign29230_e31527);
        let assign29230_e31529: f64 = (assign29230_e31509 * assign29230_e31528);
        let assign29230_e31530: f64 = (1.0 + assign29230_e31529);
        let assign29230_e31531: f64 = (1.80485e-35 / assign29230_e31530);
        (assign29230_e31531, (-((1.80485e-35 * (((-(-locals.var_spsub_xbar__blk885_dn4)) * assign29230_e31528) + (assign29230_e31509 * (((0.5 * (-(-locals.var_spsub_xbar__blk885_dn4))) * assign29230_e31526) + (assign29230_e31517 * ((-(-locals.var_spsub_xbar__blk885_dn4)) * 0.3333333333333)))))) / (assign29230_e31530 * assign29230_e31530))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar__blk885_dn6)) * assign29230_e31528) + (assign29230_e31509 * (((0.5 * (-(-locals.var_spsub_xbar__blk885_dn6))) * assign29230_e31526) + (assign29230_e31517 * ((-(-locals.var_spsub_xbar__blk885_dn6)) * 0.3333333333333)))))) / (assign29230_e31530 * assign29230_e31530))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar__blk885_dn7)) * assign29230_e31528) + (assign29230_e31509 * (((0.5 * (-(-locals.var_spsub_xbar__blk885_dn7))) * assign29230_e31526) + (assign29230_e31517 * ((-(-locals.var_spsub_xbar__blk885_dn7)) * 0.3333333333333)))))) / (assign29230_e31530 * assign29230_e31530))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar__blk885_dn8)) * assign29230_e31528) + (assign29230_e31509 * (((0.5 * (-(-locals.var_spsub_xbar__blk885_dn8))) * assign29230_e31526) + (assign29230_e31517 * ((-(-locals.var_spsub_xbar__blk885_dn8)) * 0.3333333333333)))))) / (assign29230_e31530 * assign29230_e31530))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar__blk885_dn9)) * assign29230_e31528) + (assign29230_e31509 * (((0.5 * (-(-locals.var_spsub_xbar__blk885_dn9))) * assign29230_e31526) + (assign29230_e31517 * ((-(-locals.var_spsub_xbar__blk885_dn9)) * 0.3333333333333)))))) / (assign29230_e31530 * assign29230_e31530))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29230_e31533;
        locals.var_spsub_temp__blk863_dn4 = assign29230_e31533_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29230_e31533_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29230_e31533_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29230_e31533_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29230_e31533_d_n9;

        let (assign29240_e31547, assign29240_e31547_d_n4, assign29240_e31547_d_n6, assign29240_e31547_d_n7, assign29240_e31547_d_n8, assign29240_e31547_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29240_e31545: f64 = (1.0 - locals.var_spsub_temp__blk863);
        (assign29240_e31545, (-locals.var_spsub_temp__blk863_dn4), (-locals.var_spsub_temp__blk863_dn6), (-locals.var_spsub_temp__blk863_dn7), (-locals.var_spsub_temp__blk863_dn8), (-locals.var_spsub_temp__blk863_dn9),)
    } else {
        (locals.var_spsub_w__blk886, locals.var_spsub_w__blk886_dn4, locals.var_spsub_w__blk886_dn6, locals.var_spsub_w__blk886_dn7, locals.var_spsub_w__blk886_dn8, locals.var_spsub_w__blk886_dn9,)
    }
};
        locals.var_spsub_w__blk886 = assign29240_e31547;
        locals.var_spsub_w__blk886_dn4 = assign29240_e31547_d_n4;
        locals.var_spsub_w__blk886_dn6 = assign29240_e31547_d_n6;
        locals.var_spsub_w__blk886_dn7 = assign29240_e31547_d_n7;
        locals.var_spsub_w__blk886_dn8 = assign29240_e31547_d_n8;
        locals.var_spsub_w__blk886_dn9 = assign29240_e31547_d_n9;

        let (assign29250_e31574, assign29250_e31574_d_n4, assign29250_e31574_d_n6, assign29250_e31574_d_n7, assign29250_e31574_d_n8, assign29250_e31574_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29250_e31560: f64 = (locals.var_gfsub2 * 0.5);
        let assign29250_e31561: f64 = (locals.var_spsub_xgb__blk866 + assign29250_e31560);
        let assign29250_e31566: f64 = (locals.var_gfsub2 * 0.25);
        let assign29250_e31567: f64 = (locals.var_spsub_xgb__blk866 + assign29250_e31566);
        let assign29250_e31569: f64 = (assign29250_e31567 - locals.var_spsub_w__blk886);
        let assign29250_e31570: f64 = (assign29250_e31569).sqrt();
        let assign29250_e31571: f64 = (locals.var_gfsub * assign29250_e31570);
        let assign29250_e31572: f64 = (assign29250_e31561 - assign29250_e31571);
        (assign29250_e31572, ((locals.var_spsub_xgb__blk866_dn4 + (locals.var_gfsub2_dn4 * 0.5)) - ((locals.var_gfsub_dn4 * assign29250_e31570) + (locals.var_gfsub * (((locals.var_spsub_xgb__blk866_dn4 + (locals.var_gfsub2_dn4 * 0.25)) - locals.var_spsub_w__blk886_dn4) / (2.0 * assign29250_e31570))))), ((locals.var_spsub_xgb__blk866_dn6 + (locals.var_gfsub2_dn6 * 0.5)) - ((locals.var_gfsub_dn6 * assign29250_e31570) + (locals.var_gfsub * (((locals.var_spsub_xgb__blk866_dn6 + (locals.var_gfsub2_dn6 * 0.25)) - locals.var_spsub_w__blk886_dn6) / (2.0 * assign29250_e31570))))), ((locals.var_spsub_xgb__blk866_dn7 + (locals.var_gfsub2_dn7 * 0.5)) - ((locals.var_gfsub_dn7 * assign29250_e31570) + (locals.var_gfsub * (((locals.var_spsub_xgb__blk866_dn7 + (locals.var_gfsub2_dn7 * 0.25)) - locals.var_spsub_w__blk886_dn7) / (2.0 * assign29250_e31570))))), ((locals.var_spsub_xgb__blk866_dn8 + (locals.var_gfsub2_dn8 * 0.5)) - ((locals.var_gfsub_dn8 * assign29250_e31570) + (locals.var_gfsub * (((locals.var_spsub_xgb__blk866_dn8 + (locals.var_gfsub2_dn8 * 0.25)) - locals.var_spsub_w__blk886_dn8) / (2.0 * assign29250_e31570))))), ((locals.var_spsub_xgb__blk866_dn9 + (locals.var_gfsub2_dn9 * 0.5)) - ((locals.var_gfsub_dn9 * assign29250_e31570) + (locals.var_gfsub * (((locals.var_spsub_xgb__blk866_dn9 + (locals.var_gfsub2_dn9 * 0.25)) - locals.var_spsub_w__blk886_dn9) / (2.0 * assign29250_e31570))))),)
    } else {
        (locals.var_spsub_x1__blk887, locals.var_spsub_x1__blk887_dn4, locals.var_spsub_x1__blk887_dn6, locals.var_spsub_x1__blk887_dn7, locals.var_spsub_x1__blk887_dn8, locals.var_spsub_x1__blk887_dn9,)
    }
};
        locals.var_spsub_x1__blk887 = assign29250_e31574;
        locals.var_spsub_x1__blk887_dn4 = assign29250_e31574_d_n4;
        locals.var_spsub_x1__blk887_dn6 = assign29250_e31574_d_n6;
        locals.var_spsub_x1__blk887_dn7 = assign29250_e31574_d_n7;
        locals.var_spsub_x1__blk887_dn8 = assign29250_e31574_d_n8;
        locals.var_spsub_x1__blk887_dn9 = assign29250_e31574_d_n9;

        let (assign29260_e31588, assign29260_e31588_d_n4, assign29260_e31588_d_n6, assign29260_e31588_d_n7, assign29260_e31588_d_n8, assign29260_e31588_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29260_e31586: f64 = (locals.var_xn_sub + 3.0);
        (assign29260_e31586, locals.var_xn_sub_dn4, locals.var_xn_sub_dn6, locals.var_xn_sub_dn7, locals.var_xn_sub_dn8, locals.var_xn_sub_dn9,)
    } else {
        (locals.var_spsub_bx__blk888, locals.var_spsub_bx__blk888_dn4, locals.var_spsub_bx__blk888_dn6, locals.var_spsub_bx__blk888_dn7, locals.var_spsub_bx__blk888_dn8, locals.var_spsub_bx__blk888_dn9,)
    }
};
        locals.var_spsub_bx__blk888 = assign29260_e31588;
        locals.var_spsub_bx__blk888_dn4 = assign29260_e31588_d_n4;
        locals.var_spsub_bx__blk888_dn6 = assign29260_e31588_d_n6;
        locals.var_spsub_bx__blk888_dn7 = assign29260_e31588_d_n7;
        locals.var_spsub_bx__blk888_dn8 = assign29260_e31588_d_n8;
        locals.var_spsub_bx__blk888_dn9 = assign29260_e31588_d_n9;

        let (assign29270_e31626, assign29270_e31626_d_n4, assign29270_e31626_d_n6, assign29270_e31626_d_n7, assign29270_e31626_d_n8, assign29270_e31626_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29270_e31601: f64 = (locals.var_spsub_x1__blk887 + locals.var_spsub_bx__blk888);
        let assign29270_e31604: f64 = (locals.var_spsub_x1__blk887 - locals.var_spsub_bx__blk888);
        let assign29270_e31607: f64 = (locals.var_spsub_x1__blk887 - locals.var_spsub_bx__blk888);
        let assign29270_e31608: f64 = (assign29270_e31604 * assign29270_e31607);
        let assign29270_e31610: f64 = (assign29270_e31608 + 5.0);
        let assign29270_e31611: f64 = (assign29270_e31610).sqrt();
        let assign29270_e31612: f64 = (assign29270_e31601 - assign29270_e31611);
        let assign29270_e31613: f64 = (0.5 * assign29270_e31612);
        let assign29270_e31618: f64 = (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888);
        let assign29270_e31620: f64 = (assign29270_e31618 + 5.0);
        let assign29270_e31621: f64 = (assign29270_e31620).sqrt();
        let assign29270_e31622: f64 = (locals.var_spsub_bx__blk888 - assign29270_e31621);
        let assign29270_e31623: f64 = (0.5 * assign29270_e31622);
        let assign29270_e31624: f64 = (assign29270_e31613 - assign29270_e31623);
        (assign29270_e31624, ((0.5 * ((locals.var_spsub_x1__blk887_dn4 + locals.var_spsub_bx__blk888_dn4) - ((((locals.var_spsub_x1__blk887_dn4 - locals.var_spsub_bx__blk888_dn4) * assign29270_e31607) + (assign29270_e31604 * (locals.var_spsub_x1__blk887_dn4 - locals.var_spsub_bx__blk888_dn4))) / (2.0 * assign29270_e31611)))) - (0.5 * (locals.var_spsub_bx__blk888_dn4 - (((locals.var_spsub_bx__blk888_dn4 * locals.var_spsub_bx__blk888) + (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888_dn4)) / (2.0 * assign29270_e31621))))), ((0.5 * ((locals.var_spsub_x1__blk887_dn6 + locals.var_spsub_bx__blk888_dn6) - ((((locals.var_spsub_x1__blk887_dn6 - locals.var_spsub_bx__blk888_dn6) * assign29270_e31607) + (assign29270_e31604 * (locals.var_spsub_x1__blk887_dn6 - locals.var_spsub_bx__blk888_dn6))) / (2.0 * assign29270_e31611)))) - (0.5 * (locals.var_spsub_bx__blk888_dn6 - (((locals.var_spsub_bx__blk888_dn6 * locals.var_spsub_bx__blk888) + (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888_dn6)) / (2.0 * assign29270_e31621))))), ((0.5 * ((locals.var_spsub_x1__blk887_dn7 + locals.var_spsub_bx__blk888_dn7) - ((((locals.var_spsub_x1__blk887_dn7 - locals.var_spsub_bx__blk888_dn7) * assign29270_e31607) + (assign29270_e31604 * (locals.var_spsub_x1__blk887_dn7 - locals.var_spsub_bx__blk888_dn7))) / (2.0 * assign29270_e31611)))) - (0.5 * (locals.var_spsub_bx__blk888_dn7 - (((locals.var_spsub_bx__blk888_dn7 * locals.var_spsub_bx__blk888) + (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888_dn7)) / (2.0 * assign29270_e31621))))), ((0.5 * ((locals.var_spsub_x1__blk887_dn8 + locals.var_spsub_bx__blk888_dn8) - ((((locals.var_spsub_x1__blk887_dn8 - locals.var_spsub_bx__blk888_dn8) * assign29270_e31607) + (assign29270_e31604 * (locals.var_spsub_x1__blk887_dn8 - locals.var_spsub_bx__blk888_dn8))) / (2.0 * assign29270_e31611)))) - (0.5 * (locals.var_spsub_bx__blk888_dn8 - (((locals.var_spsub_bx__blk888_dn8 * locals.var_spsub_bx__blk888) + (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888_dn8)) / (2.0 * assign29270_e31621))))), ((0.5 * ((locals.var_spsub_x1__blk887_dn9 + locals.var_spsub_bx__blk888_dn9) - ((((locals.var_spsub_x1__blk887_dn9 - locals.var_spsub_bx__blk888_dn9) * assign29270_e31607) + (assign29270_e31604 * (locals.var_spsub_x1__blk887_dn9 - locals.var_spsub_bx__blk888_dn9))) / (2.0 * assign29270_e31611)))) - (0.5 * (locals.var_spsub_bx__blk888_dn9 - (((locals.var_spsub_bx__blk888_dn9 * locals.var_spsub_bx__blk888) + (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888_dn9)) / (2.0 * assign29270_e31621))))),)
    } else {
        (locals.var_spsub_eta__blk870, locals.var_spsub_eta__blk870_dn4, locals.var_spsub_eta__blk870_dn6, locals.var_spsub_eta__blk870_dn7, locals.var_spsub_eta__blk870_dn8, locals.var_spsub_eta__blk870_dn9,)
    }
};
        locals.var_spsub_eta__blk870 = assign29270_e31626;
        locals.var_spsub_eta__blk870_dn4 = assign29270_e31626_d_n4;
        locals.var_spsub_eta__blk870_dn6 = assign29270_e31626_d_n6;
        locals.var_spsub_eta__blk870_dn7 = assign29270_e31626_d_n7;
        locals.var_spsub_eta__blk870_dn8 = assign29270_e31626_d_n8;
        locals.var_spsub_eta__blk870_dn9 = assign29270_e31626_d_n9;

        let (assign29280_e31640, assign29280_e31640_d_n4, assign29280_e31640_d_n6, assign29280_e31640_d_n7, assign29280_e31640_d_n8, assign29280_e31640_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29280_e31638: f64 = (locals.var_spsub_xgb__blk866 - locals.var_spsub_eta__blk870);
        (assign29280_e31638, (locals.var_spsub_xgb__blk866_dn4 - locals.var_spsub_eta__blk870_dn4), (locals.var_spsub_xgb__blk866_dn6 - locals.var_spsub_eta__blk870_dn6), (locals.var_spsub_xgb__blk866_dn7 - locals.var_spsub_eta__blk870_dn7), (locals.var_spsub_xgb__blk866_dn8 - locals.var_spsub_eta__blk870_dn8), (locals.var_spsub_xgb__blk866_dn9 - locals.var_spsub_eta__blk870_dn9),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29280_e31640;
        locals.var_spsub_temp__blk863_dn4 = assign29280_e31640_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29280_e31640_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29280_e31640_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29280_e31640_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29280_e31640_d_n9;

        let (assign29290_e31654, assign29290_e31654_d_n4, assign29290_e31654_d_n6, assign29290_e31654_d_n7, assign29290_e31654_d_n8, assign29290_e31654_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29290_e31651: f64 = (-locals.var_spsub_eta__blk870);
        let assign29290_e31652: f64 = (assign29290_e31651).exp();
        (assign29290_e31652, (assign29290_e31652 * (-locals.var_spsub_eta__blk870_dn4)), (assign29290_e31652 * (-locals.var_spsub_eta__blk870_dn6)), (assign29290_e31652 * (-locals.var_spsub_eta__blk870_dn7)), (assign29290_e31652 * (-locals.var_spsub_eta__blk870_dn8)), (assign29290_e31652 * (-locals.var_spsub_eta__blk870_dn9)),)
    } else {
        (locals.var_spsub_temp1__blk864, locals.var_spsub_temp1__blk864_dn4, locals.var_spsub_temp1__blk864_dn6, locals.var_spsub_temp1__blk864_dn7, locals.var_spsub_temp1__blk864_dn8, locals.var_spsub_temp1__blk864_dn9,)
    }
};
        locals.var_spsub_temp1__blk864 = assign29290_e31654;
        locals.var_spsub_temp1__blk864_dn4 = assign29290_e31654_d_n4;
        locals.var_spsub_temp1__blk864_dn6 = assign29290_e31654_d_n6;
        locals.var_spsub_temp1__blk864_dn7 = assign29290_e31654_d_n7;
        locals.var_spsub_temp1__blk864_dn8 = assign29290_e31654_d_n8;
        locals.var_spsub_temp1__blk864_dn9 = assign29290_e31654_d_n9;

        let (assign29300_e31672, assign29300_e31672_d_n4, assign29300_e31672_d_n6, assign29300_e31672_d_n7, assign29300_e31672_d_n8, assign29300_e31672_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29300_e31668: f64 = (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870);
        let assign29300_e31669: f64 = (2.0 + assign29300_e31668);
        let assign29300_e31670: f64 = (1.0 / assign29300_e31669);
        (assign29300_e31670, (-(((locals.var_spsub_eta__blk870_dn4 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn4)) / (assign29300_e31669 * assign29300_e31669))), (-(((locals.var_spsub_eta__blk870_dn6 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn6)) / (assign29300_e31669 * assign29300_e31669))), (-(((locals.var_spsub_eta__blk870_dn7 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn7)) / (assign29300_e31669 * assign29300_e31669))), (-(((locals.var_spsub_eta__blk870_dn8 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn8)) / (assign29300_e31669 * assign29300_e31669))), (-(((locals.var_spsub_eta__blk870_dn9 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn9)) / (assign29300_e31669 * assign29300_e31669))),)
    } else {
        (locals.var_spsub_temp2__blk865, locals.var_spsub_temp2__blk865_dn4, locals.var_spsub_temp2__blk865_dn6, locals.var_spsub_temp2__blk865_dn7, locals.var_spsub_temp2__blk865_dn8, locals.var_spsub_temp2__blk865_dn9,)
    }
};
        locals.var_spsub_temp2__blk865 = assign29300_e31672;
        locals.var_spsub_temp2__blk865_dn4 = assign29300_e31672_d_n4;
        locals.var_spsub_temp2__blk865_dn6 = assign29300_e31672_d_n6;
        locals.var_spsub_temp2__blk865_dn7 = assign29300_e31672_d_n7;
        locals.var_spsub_temp2__blk865_dn8 = assign29300_e31672_d_n8;
        locals.var_spsub_temp2__blk865_dn9 = assign29300_e31672_d_n9;

        let (assign29310_e31688, assign29310_e31688_d_n4, assign29310_e31688_d_n6, assign29310_e31688_d_n7, assign29310_e31688_d_n8, assign29310_e31688_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29310_e31684: f64 = (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870);
        let assign29310_e31686: f64 = (assign29310_e31684 * locals.var_spsub_temp2__blk865);
        (assign29310_e31686, ((((locals.var_spsub_eta__blk870_dn4 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn4)) * locals.var_spsub_temp2__blk865) + (assign29310_e31684 * locals.var_spsub_temp2__blk865_dn4)), ((((locals.var_spsub_eta__blk870_dn6 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn6)) * locals.var_spsub_temp2__blk865) + (assign29310_e31684 * locals.var_spsub_temp2__blk865_dn6)), ((((locals.var_spsub_eta__blk870_dn7 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn7)) * locals.var_spsub_temp2__blk865) + (assign29310_e31684 * locals.var_spsub_temp2__blk865_dn7)), ((((locals.var_spsub_eta__blk870_dn8 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn8)) * locals.var_spsub_temp2__blk865) + (assign29310_e31684 * locals.var_spsub_temp2__blk865_dn8)), ((((locals.var_spsub_eta__blk870_dn9 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn9)) * locals.var_spsub_temp2__blk865) + (assign29310_e31684 * locals.var_spsub_temp2__blk865_dn9)),)
    } else {
        (locals.var_spsub_xi0__blk878, locals.var_spsub_xi0__blk878_dn4, locals.var_spsub_xi0__blk878_dn6, locals.var_spsub_xi0__blk878_dn7, locals.var_spsub_xi0__blk878_dn8, locals.var_spsub_xi0__blk878_dn9,)
    }
};
        locals.var_spsub_xi0__blk878 = assign29310_e31688;
        locals.var_spsub_xi0__blk878_dn4 = assign29310_e31688_d_n4;
        locals.var_spsub_xi0__blk878_dn6 = assign29310_e31688_d_n6;
        locals.var_spsub_xi0__blk878_dn7 = assign29310_e31688_d_n7;
        locals.var_spsub_xi0__blk878_dn8 = assign29310_e31688_d_n8;
        locals.var_spsub_xi0__blk878_dn9 = assign29310_e31688_d_n9;

        let (assign29320_e31706, assign29320_e31706_d_n4, assign29320_e31706_d_n6, assign29320_e31706_d_n7, assign29320_e31706_d_n8, assign29320_e31706_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29320_e31701: f64 = (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865);
        let assign29320_e31703: f64 = (assign29320_e31701 * locals.var_spsub_temp2__blk865);
        let assign29320_e31704: f64 = (4.0 * assign29320_e31703);
        (assign29320_e31704, (4.0 * ((((locals.var_spsub_eta__blk870_dn4 * locals.var_spsub_temp2__blk865) + (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865_dn4)) * locals.var_spsub_temp2__blk865) + (assign29320_e31701 * locals.var_spsub_temp2__blk865_dn4))), (4.0 * ((((locals.var_spsub_eta__blk870_dn6 * locals.var_spsub_temp2__blk865) + (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865_dn6)) * locals.var_spsub_temp2__blk865) + (assign29320_e31701 * locals.var_spsub_temp2__blk865_dn6))), (4.0 * ((((locals.var_spsub_eta__blk870_dn7 * locals.var_spsub_temp2__blk865) + (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865_dn7)) * locals.var_spsub_temp2__blk865) + (assign29320_e31701 * locals.var_spsub_temp2__blk865_dn7))), (4.0 * ((((locals.var_spsub_eta__blk870_dn8 * locals.var_spsub_temp2__blk865) + (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865_dn8)) * locals.var_spsub_temp2__blk865) + (assign29320_e31701 * locals.var_spsub_temp2__blk865_dn8))), (4.0 * ((((locals.var_spsub_eta__blk870_dn9 * locals.var_spsub_temp2__blk865) + (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865_dn9)) * locals.var_spsub_temp2__blk865) + (assign29320_e31701 * locals.var_spsub_temp2__blk865_dn9))),)
    } else {
        (locals.var_spsub_xi1__blk879, locals.var_spsub_xi1__blk879_dn4, locals.var_spsub_xi1__blk879_dn6, locals.var_spsub_xi1__blk879_dn7, locals.var_spsub_xi1__blk879_dn8, locals.var_spsub_xi1__blk879_dn9,)
    }
};
        locals.var_spsub_xi1__blk879 = assign29320_e31706;
        locals.var_spsub_xi1__blk879_dn4 = assign29320_e31706_d_n4;
        locals.var_spsub_xi1__blk879_dn6 = assign29320_e31706_d_n6;
        locals.var_spsub_xi1__blk879_dn7 = assign29320_e31706_d_n7;
        locals.var_spsub_xi1__blk879_dn8 = assign29320_e31706_d_n8;
        locals.var_spsub_xi1__blk879_dn9 = assign29320_e31706_d_n9;

        let (assign29330_e31728, assign29330_e31728_d_n4, assign29330_e31728_d_n6, assign29330_e31728_d_n7, assign29330_e31728_d_n8, assign29330_e31728_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29330_e31718: f64 = (8.0 * locals.var_spsub_temp2__blk865);
        let assign29330_e31721: f64 = (12.0 * locals.var_spsub_xi0__blk878);
        let assign29330_e31722: f64 = (assign29330_e31718 - assign29330_e31721);
        let assign29330_e31724: f64 = (assign29330_e31722 * locals.var_spsub_temp2__blk865);
        let assign29330_e31726: f64 = (assign29330_e31724 * locals.var_spsub_temp2__blk865);
        (assign29330_e31726, ((((((8.0 * locals.var_spsub_temp2__blk865_dn4) - (12.0 * locals.var_spsub_xi0__blk878_dn4)) * locals.var_spsub_temp2__blk865) + (assign29330_e31722 * locals.var_spsub_temp2__blk865_dn4)) * locals.var_spsub_temp2__blk865) + (assign29330_e31724 * locals.var_spsub_temp2__blk865_dn4)), ((((((8.0 * locals.var_spsub_temp2__blk865_dn6) - (12.0 * locals.var_spsub_xi0__blk878_dn6)) * locals.var_spsub_temp2__blk865) + (assign29330_e31722 * locals.var_spsub_temp2__blk865_dn6)) * locals.var_spsub_temp2__blk865) + (assign29330_e31724 * locals.var_spsub_temp2__blk865_dn6)), ((((((8.0 * locals.var_spsub_temp2__blk865_dn7) - (12.0 * locals.var_spsub_xi0__blk878_dn7)) * locals.var_spsub_temp2__blk865) + (assign29330_e31722 * locals.var_spsub_temp2__blk865_dn7)) * locals.var_spsub_temp2__blk865) + (assign29330_e31724 * locals.var_spsub_temp2__blk865_dn7)), ((((((8.0 * locals.var_spsub_temp2__blk865_dn8) - (12.0 * locals.var_spsub_xi0__blk878_dn8)) * locals.var_spsub_temp2__blk865) + (assign29330_e31722 * locals.var_spsub_temp2__blk865_dn8)) * locals.var_spsub_temp2__blk865) + (assign29330_e31724 * locals.var_spsub_temp2__blk865_dn8)), ((((((8.0 * locals.var_spsub_temp2__blk865_dn9) - (12.0 * locals.var_spsub_xi0__blk878_dn9)) * locals.var_spsub_temp2__blk865) + (assign29330_e31722 * locals.var_spsub_temp2__blk865_dn9)) * locals.var_spsub_temp2__blk865) + (assign29330_e31724 * locals.var_spsub_temp2__blk865_dn9)),)
    } else {
        (locals.var_spsub_xi2__blk880, locals.var_spsub_xi2__blk880_dn4, locals.var_spsub_xi2__blk880_dn6, locals.var_spsub_xi2__blk880_dn7, locals.var_spsub_xi2__blk880_dn8, locals.var_spsub_xi2__blk880_dn9,)
    }
};
        locals.var_spsub_xi2__blk880 = assign29330_e31728;
        locals.var_spsub_xi2__blk880_dn4 = assign29330_e31728_d_n4;
        locals.var_spsub_xi2__blk880_dn6 = assign29330_e31728_d_n6;
        locals.var_spsub_xi2__blk880_dn7 = assign29330_e31728_d_n7;
        locals.var_spsub_xi2__blk880_dn8 = assign29330_e31728_d_n8;
        locals.var_spsub_xi2__blk880_dn9 = assign29330_e31728_d_n9;

        let (assign29340_e31760, assign29340_e31760_d_n4, assign29340_e31760_d_n6, assign29340_e31760_d_n7, assign29340_e31760_d_n8, assign29340_e31760_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29340_e31741: f64 = (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863);
        let assign29340_e31745: f64 = (locals.var_spsub_temp1__blk864 + locals.var_spsub_eta__blk870);
        let assign29340_e31747: f64 = (assign29340_e31745 - 1.0);
        let assign29340_e31751: f64 = (locals.var_spsub_eta__blk870 + 1.0);
        let assign29340_e31753: f64 = (assign29340_e31751 + locals.var_spsub_xi0__blk878);
        let assign29340_e31754: f64 = (locals.var_spsub_delta__blk867 * assign29340_e31753);
        let assign29340_e31755: f64 = (assign29340_e31747 - assign29340_e31754);
        let assign29340_e31756: f64 = (locals.var_gfsub2 * assign29340_e31755);
        let assign29340_e31757: f64 = (assign29340_e31741 - assign29340_e31756);
        let assign29340_e31758: f64 = (1e-40_f64).max(assign29340_e31757);
        (assign29340_e31758, if 1e-40 >= assign29340_e31757 { 0.0 } else { (((locals.var_spsub_temp__blk863_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn4)) - ((locals.var_gfsub2_dn4 * assign29340_e31755) + (locals.var_gfsub2 * ((locals.var_spsub_temp1__blk864_dn4 + locals.var_spsub_eta__blk870_dn4) - ((locals.var_spsub_delta__blk867_dn4 * assign29340_e31753) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_eta__blk870_dn4 + locals.var_spsub_xi0__blk878_dn4))))))) }, if 1e-40 >= assign29340_e31757 { 0.0 } else { (((locals.var_spsub_temp__blk863_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn6)) - ((locals.var_gfsub2_dn6 * assign29340_e31755) + (locals.var_gfsub2 * ((locals.var_spsub_temp1__blk864_dn6 + locals.var_spsub_eta__blk870_dn6) - ((locals.var_spsub_delta__blk867_dn6 * assign29340_e31753) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_eta__blk870_dn6 + locals.var_spsub_xi0__blk878_dn6))))))) }, if 1e-40 >= assign29340_e31757 { 0.0 } else { (((locals.var_spsub_temp__blk863_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn7)) - ((locals.var_gfsub2_dn7 * assign29340_e31755) + (locals.var_gfsub2 * ((locals.var_spsub_temp1__blk864_dn7 + locals.var_spsub_eta__blk870_dn7) - ((locals.var_spsub_delta__blk867_dn7 * assign29340_e31753) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_eta__blk870_dn7 + locals.var_spsub_xi0__blk878_dn7))))))) }, if 1e-40 >= assign29340_e31757 { 0.0 } else { (((locals.var_spsub_temp__blk863_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn8)) - ((locals.var_gfsub2_dn8 * assign29340_e31755) + (locals.var_gfsub2 * ((locals.var_spsub_temp1__blk864_dn8 + locals.var_spsub_eta__blk870_dn8) - ((locals.var_spsub_delta__blk867_dn8 * assign29340_e31753) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_eta__blk870_dn8 + locals.var_spsub_xi0__blk878_dn8))))))) }, if 1e-40 >= assign29340_e31757 { 0.0 } else { (((locals.var_spsub_temp__blk863_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn9)) - ((locals.var_gfsub2_dn9 * assign29340_e31755) + (locals.var_gfsub2 * ((locals.var_spsub_temp1__blk864_dn9 + locals.var_spsub_eta__blk870_dn9) - ((locals.var_spsub_delta__blk867_dn9 * assign29340_e31753) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_eta__blk870_dn9 + locals.var_spsub_xi0__blk878_dn9))))))) },)
    } else {
        (locals.var_spsub_a__blk871, locals.var_spsub_a__blk871_dn4, locals.var_spsub_a__blk871_dn6, locals.var_spsub_a__blk871_dn7, locals.var_spsub_a__blk871_dn8, locals.var_spsub_a__blk871_dn9,)
    }
};
        locals.var_spsub_a__blk871 = assign29340_e31760;
        locals.var_spsub_a__blk871_dn4 = assign29340_e31760_d_n4;
        locals.var_spsub_a__blk871_dn6 = assign29340_e31760_d_n6;
        locals.var_spsub_a__blk871_dn7 = assign29340_e31760_d_n7;
        locals.var_spsub_a__blk871_dn8 = assign29340_e31760_d_n8;
        locals.var_spsub_a__blk871_dn9 = assign29340_e31760_d_n9;

        let (assign29350_e31782, assign29350_e31782_d_n4, assign29350_e31782_d_n6, assign29350_e31782_d_n7, assign29350_e31782_d_n8, assign29350_e31782_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29350_e31776: f64 = (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880);
        let assign29350_e31777: f64 = (locals.var_spsub_temp1__blk864 - assign29350_e31776);
        let assign29350_e31778: f64 = (locals.var_gfsub2 * assign29350_e31777);
        let assign29350_e31779: f64 = (0.5 * assign29350_e31778);
        let assign29350_e31780: f64 = (1.0 - assign29350_e31779);
        (assign29350_e31780, (-(0.5 * ((locals.var_gfsub2_dn4 * assign29350_e31777) + (locals.var_gfsub2 * (locals.var_spsub_temp1__blk864_dn4 - ((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn4))))))), (-(0.5 * ((locals.var_gfsub2_dn6 * assign29350_e31777) + (locals.var_gfsub2 * (locals.var_spsub_temp1__blk864_dn6 - ((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn6))))))), (-(0.5 * ((locals.var_gfsub2_dn7 * assign29350_e31777) + (locals.var_gfsub2 * (locals.var_spsub_temp1__blk864_dn7 - ((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn7))))))), (-(0.5 * ((locals.var_gfsub2_dn8 * assign29350_e31777) + (locals.var_gfsub2 * (locals.var_spsub_temp1__blk864_dn8 - ((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn8))))))), (-(0.5 * ((locals.var_gfsub2_dn9 * assign29350_e31777) + (locals.var_gfsub2 * (locals.var_spsub_temp1__blk864_dn9 - ((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn9))))))),)
    } else {
        (locals.var_spsub_b__blk872, locals.var_spsub_b__blk872_dn4, locals.var_spsub_b__blk872_dn6, locals.var_spsub_b__blk872_dn7, locals.var_spsub_b__blk872_dn8, locals.var_spsub_b__blk872_dn9,)
    }
};
        locals.var_spsub_b__blk872 = assign29350_e31782;
        locals.var_spsub_b__blk872_dn4 = assign29350_e31782_d_n4;
        locals.var_spsub_b__blk872_dn6 = assign29350_e31782_d_n6;
        locals.var_spsub_b__blk872_dn7 = assign29350_e31782_d_n7;
        locals.var_spsub_b__blk872_dn8 = assign29350_e31782_d_n8;
        locals.var_spsub_b__blk872_dn9 = assign29350_e31782_d_n9;

        let (assign29360_e31808, assign29360_e31808_d_n4, assign29360_e31808_d_n6, assign29360_e31808_d_n7, assign29360_e31808_d_n8, assign29360_e31808_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29360_e31794: f64 = (2.0 * locals.var_spsub_temp__blk863);
        let assign29360_e31798: f64 = (1.0 - locals.var_spsub_temp1__blk864);
        let assign29360_e31802: f64 = (1.0 + locals.var_spsub_xi1__blk879);
        let assign29360_e31803: f64 = (locals.var_spsub_delta__blk867 * assign29360_e31802);
        let assign29360_e31804: f64 = (assign29360_e31798 - assign29360_e31803);
        let assign29360_e31805: f64 = (locals.var_gfsub2 * assign29360_e31804);
        let assign29360_e31806: f64 = (assign29360_e31794 + assign29360_e31805);
        (assign29360_e31806, ((2.0 * locals.var_spsub_temp__blk863_dn4) + ((locals.var_gfsub2_dn4 * assign29360_e31804) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1__blk864_dn4) - ((locals.var_spsub_delta__blk867_dn4 * assign29360_e31802) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn4)))))), ((2.0 * locals.var_spsub_temp__blk863_dn6) + ((locals.var_gfsub2_dn6 * assign29360_e31804) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1__blk864_dn6) - ((locals.var_spsub_delta__blk867_dn6 * assign29360_e31802) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn6)))))), ((2.0 * locals.var_spsub_temp__blk863_dn7) + ((locals.var_gfsub2_dn7 * assign29360_e31804) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1__blk864_dn7) - ((locals.var_spsub_delta__blk867_dn7 * assign29360_e31802) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn7)))))), ((2.0 * locals.var_spsub_temp__blk863_dn8) + ((locals.var_gfsub2_dn8 * assign29360_e31804) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1__blk864_dn8) - ((locals.var_spsub_delta__blk867_dn8 * assign29360_e31802) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn8)))))), ((2.0 * locals.var_spsub_temp__blk863_dn9) + ((locals.var_gfsub2_dn9 * assign29360_e31804) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1__blk864_dn9) - ((locals.var_spsub_delta__blk867_dn9 * assign29360_e31802) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn9)))))),)
    } else {
        (locals.var_spsub_c__blk873, locals.var_spsub_c__blk873_dn4, locals.var_spsub_c__blk873_dn6, locals.var_spsub_c__blk873_dn7, locals.var_spsub_c__blk873_dn8, locals.var_spsub_c__blk873_dn9,)
    }
};
        locals.var_spsub_c__blk873 = assign29360_e31808;
        locals.var_spsub_c__blk873_dn4 = assign29360_e31808_d_n4;
        locals.var_spsub_c__blk873_dn6 = assign29360_e31808_d_n6;
        locals.var_spsub_c__blk873_dn7 = assign29360_e31808_d_n7;
        locals.var_spsub_c__blk873_dn8 = assign29360_e31808_d_n8;
        locals.var_spsub_c__blk873_dn9 = assign29360_e31808_d_n9;

        let (assign29370_e31827, assign29370_e31827_d_n4, assign29370_e31827_d_n6, assign29370_e31827_d_n7, assign29370_e31827_d_n8, assign29370_e31827_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29370_e31820: f64 = (locals.var_xn_sub - locals.var_spsub_eta__blk870);
        let assign29370_e31823: f64 = (locals.var_spsub_a__blk871 / locals.var_gfsub2);
        let assign29370_e31824: f64 = (assign29370_e31823).ln();
        let assign29370_e31825: f64 = (assign29370_e31820 + assign29370_e31824);
        (assign29370_e31825, ((locals.var_xn_sub_dn4 - locals.var_spsub_eta__blk870_dn4) + ((((locals.var_spsub_a__blk871_dn4 * locals.var_gfsub2) - (locals.var_spsub_a__blk871 * locals.var_gfsub2_dn4)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign29370_e31823)), ((locals.var_xn_sub_dn6 - locals.var_spsub_eta__blk870_dn6) + ((((locals.var_spsub_a__blk871_dn6 * locals.var_gfsub2) - (locals.var_spsub_a__blk871 * locals.var_gfsub2_dn6)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign29370_e31823)), ((locals.var_xn_sub_dn7 - locals.var_spsub_eta__blk870_dn7) + ((((locals.var_spsub_a__blk871_dn7 * locals.var_gfsub2) - (locals.var_spsub_a__blk871 * locals.var_gfsub2_dn7)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign29370_e31823)), ((locals.var_xn_sub_dn8 - locals.var_spsub_eta__blk870_dn8) + ((((locals.var_spsub_a__blk871_dn8 * locals.var_gfsub2) - (locals.var_spsub_a__blk871 * locals.var_gfsub2_dn8)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign29370_e31823)), ((locals.var_xn_sub_dn9 - locals.var_spsub_eta__blk870_dn9) + ((((locals.var_spsub_a__blk871_dn9 * locals.var_gfsub2) - (locals.var_spsub_a__blk871 * locals.var_gfsub2_dn9)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign29370_e31823)),)
    } else {
        (locals.var_spsub_tau__blk874, locals.var_spsub_tau__blk874_dn4, locals.var_spsub_tau__blk874_dn6, locals.var_spsub_tau__blk874_dn7, locals.var_spsub_tau__blk874_dn8, locals.var_spsub_tau__blk874_dn9,)
    }
};
        locals.var_spsub_tau__blk874 = assign29370_e31827;
        locals.var_spsub_tau__blk874_dn4 = assign29370_e31827_d_n4;
        locals.var_spsub_tau__blk874_dn6 = assign29370_e31827_d_n6;
        locals.var_spsub_tau__blk874_dn7 = assign29370_e31827_d_n7;
        locals.var_spsub_tau__blk874_dn8 = assign29370_e31827_d_n8;
        locals.var_spsub_tau__blk874_dn9 = assign29370_e31827_d_n9;

        let (assign29380_e31841, assign29380_e31841_d_n4, assign29380_e31841_d_n6, assign29380_e31841_d_n7, assign29380_e31841_d_n8, assign29380_e31841_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29380_e31839: f64 = (locals.var_spsub_a__blk871 + locals.var_spsub_c__blk873);
        (assign29380_e31839, (locals.var_spsub_a__blk871_dn4 + locals.var_spsub_c__blk873_dn4), (locals.var_spsub_a__blk871_dn6 + locals.var_spsub_c__blk873_dn6), (locals.var_spsub_a__blk871_dn7 + locals.var_spsub_c__blk873_dn7), (locals.var_spsub_a__blk871_dn8 + locals.var_spsub_c__blk873_dn8), (locals.var_spsub_a__blk871_dn9 + locals.var_spsub_c__blk873_dn9),)
    } else {
        (locals.var_nu__blk861, locals.var_nu__blk861_dn4, locals.var_nu__blk861_dn6, locals.var_nu__blk861_dn7, locals.var_nu__blk861_dn8, locals.var_nu__blk861_dn9,)
    }
};
        locals.var_nu__blk861 = assign29380_e31841;
        locals.var_nu__blk861_dn4 = assign29380_e31841_d_n4;
        locals.var_nu__blk861_dn6 = assign29380_e31841_d_n6;
        locals.var_nu__blk861_dn7 = assign29380_e31841_d_n7;
        locals.var_nu__blk861_dn8 = assign29380_e31841_d_n8;
        locals.var_nu__blk861_dn9 = assign29380_e31841_d_n9;

        let (assign29390_e31867, assign29390_e31867_d_n4, assign29390_e31867_d_n6, assign29390_e31867_d_n7, assign29390_e31867_d_n8, assign29390_e31867_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29390_e31853: f64 = (locals.var_nu__blk861 * locals.var_nu__blk861);
        let assign29390_e31857: f64 = (0.5 * locals.var_spsub_c__blk873);
        let assign29390_e31859: f64 = (assign29390_e31857 * locals.var_spsub_c__blk873);
        let assign29390_e31862: f64 = (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872);
        let assign29390_e31863: f64 = (assign29390_e31859 - assign29390_e31862);
        let assign29390_e31864: f64 = (locals.var_spsub_tau__blk874 * assign29390_e31863);
        let assign29390_e31865: f64 = (assign29390_e31853 + assign29390_e31864);
        (assign29390_e31865, (((locals.var_nu__blk861_dn4 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn4)) + ((locals.var_spsub_tau__blk874_dn4 * assign29390_e31863) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn4) * locals.var_spsub_c__blk873) + (assign29390_e31857 * locals.var_spsub_c__blk873_dn4)) - ((locals.var_spsub_a__blk871_dn4 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn4)))))), (((locals.var_nu__blk861_dn6 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn6)) + ((locals.var_spsub_tau__blk874_dn6 * assign29390_e31863) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn6) * locals.var_spsub_c__blk873) + (assign29390_e31857 * locals.var_spsub_c__blk873_dn6)) - ((locals.var_spsub_a__blk871_dn6 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn6)))))), (((locals.var_nu__blk861_dn7 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn7)) + ((locals.var_spsub_tau__blk874_dn7 * assign29390_e31863) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn7) * locals.var_spsub_c__blk873) + (assign29390_e31857 * locals.var_spsub_c__blk873_dn7)) - ((locals.var_spsub_a__blk871_dn7 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn7)))))), (((locals.var_nu__blk861_dn8 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn8)) + ((locals.var_spsub_tau__blk874_dn8 * assign29390_e31863) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn8) * locals.var_spsub_c__blk873) + (assign29390_e31857 * locals.var_spsub_c__blk873_dn8)) - ((locals.var_spsub_a__blk871_dn8 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn8)))))), (((locals.var_nu__blk861_dn9 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn9)) + ((locals.var_spsub_tau__blk874_dn9 * assign29390_e31863) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn9) * locals.var_spsub_c__blk873) + (assign29390_e31857 * locals.var_spsub_c__blk873_dn9)) - ((locals.var_spsub_a__blk871_dn9 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn9)))))),)
    } else {
        (locals.var_mutau__blk862, locals.var_mutau__blk862_dn4, locals.var_mutau__blk862_dn6, locals.var_mutau__blk862_dn7, locals.var_mutau__blk862_dn8, locals.var_mutau__blk862_dn9,)
    }
};
        locals.var_mutau__blk862 = assign29390_e31867;
        locals.var_mutau__blk862_dn4 = assign29390_e31867_d_n4;
        locals.var_mutau__blk862_dn6 = assign29390_e31867_d_n6;
        locals.var_mutau__blk862_dn7 = assign29390_e31867_d_n7;
        locals.var_mutau__blk862_dn8 = assign29390_e31867_d_n8;
        locals.var_mutau__blk862_dn9 = assign29390_e31867_d_n9;

        let (assign29400_e31907, assign29400_e31907_d_n4, assign29400_e31907_d_n6, assign29400_e31907_d_n7, assign29400_e31907_d_n8, assign29400_e31907_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29400_e31880: f64 = (locals.var_spsub_a__blk871 * locals.var_nu__blk861);
        let assign29400_e31882: f64 = (assign29400_e31880 * locals.var_spsub_tau__blk874);
        let assign29400_e31886: f64 = (locals.var_nu__blk861 / locals.var_mutau__blk862);
        let assign29400_e31888: f64 = (assign29400_e31886 * locals.var_spsub_tau__blk874);
        let assign29400_e31890: f64 = (assign29400_e31888 * locals.var_spsub_tau__blk874);
        let assign29400_e31892: f64 = (assign29400_e31890 * locals.var_spsub_c__blk873);
        let assign29400_e31895: f64 = (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873);
        let assign29400_e31897: f64 = (assign29400_e31895 * 0.3333333333333);
        let assign29400_e31900: f64 = (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872);
        let assign29400_e31901: f64 = (assign29400_e31897 - assign29400_e31900);
        let assign29400_e31902: f64 = (assign29400_e31892 * assign29400_e31901);
        let assign29400_e31903: f64 = (locals.var_mutau__blk862 + assign29400_e31902);
        let assign29400_e31904: f64 = (assign29400_e31882 / assign29400_e31903);
        let assign29400_e31905: f64 = (locals.var_spsub_eta__blk870 + assign29400_e31904);
        (assign29400_e31905, (locals.var_spsub_eta__blk870_dn4 + (((((((locals.var_spsub_a__blk871_dn4 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn4)) * locals.var_spsub_tau__blk874) + (assign29400_e31880 * locals.var_spsub_tau__blk874_dn4)) * assign29400_e31903) - (assign29400_e31882 * (locals.var_mutau__blk862_dn4 + (((((((((((locals.var_nu__blk861_dn4 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn4)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29400_e31886 * locals.var_spsub_tau__blk874_dn4)) * locals.var_spsub_tau__blk874) + (assign29400_e31888 * locals.var_spsub_tau__blk874_dn4)) * locals.var_spsub_c__blk873) + (assign29400_e31890 * locals.var_spsub_c__blk873_dn4)) * assign29400_e31901) + (assign29400_e31892 * ((((locals.var_spsub_c__blk873_dn4 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn4)) * 0.3333333333333) - ((locals.var_spsub_a__blk871_dn4 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn4)))))))) / (assign29400_e31903 * assign29400_e31903))), (locals.var_spsub_eta__blk870_dn6 + (((((((locals.var_spsub_a__blk871_dn6 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn6)) * locals.var_spsub_tau__blk874) + (assign29400_e31880 * locals.var_spsub_tau__blk874_dn6)) * assign29400_e31903) - (assign29400_e31882 * (locals.var_mutau__blk862_dn6 + (((((((((((locals.var_nu__blk861_dn6 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn6)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29400_e31886 * locals.var_spsub_tau__blk874_dn6)) * locals.var_spsub_tau__blk874) + (assign29400_e31888 * locals.var_spsub_tau__blk874_dn6)) * locals.var_spsub_c__blk873) + (assign29400_e31890 * locals.var_spsub_c__blk873_dn6)) * assign29400_e31901) + (assign29400_e31892 * ((((locals.var_spsub_c__blk873_dn6 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn6)) * 0.3333333333333) - ((locals.var_spsub_a__blk871_dn6 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn6)))))))) / (assign29400_e31903 * assign29400_e31903))), (locals.var_spsub_eta__blk870_dn7 + (((((((locals.var_spsub_a__blk871_dn7 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn7)) * locals.var_spsub_tau__blk874) + (assign29400_e31880 * locals.var_spsub_tau__blk874_dn7)) * assign29400_e31903) - (assign29400_e31882 * (locals.var_mutau__blk862_dn7 + (((((((((((locals.var_nu__blk861_dn7 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn7)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29400_e31886 * locals.var_spsub_tau__blk874_dn7)) * locals.var_spsub_tau__blk874) + (assign29400_e31888 * locals.var_spsub_tau__blk874_dn7)) * locals.var_spsub_c__blk873) + (assign29400_e31890 * locals.var_spsub_c__blk873_dn7)) * assign29400_e31901) + (assign29400_e31892 * ((((locals.var_spsub_c__blk873_dn7 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn7)) * 0.3333333333333) - ((locals.var_spsub_a__blk871_dn7 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn7)))))))) / (assign29400_e31903 * assign29400_e31903))), (locals.var_spsub_eta__blk870_dn8 + (((((((locals.var_spsub_a__blk871_dn8 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn8)) * locals.var_spsub_tau__blk874) + (assign29400_e31880 * locals.var_spsub_tau__blk874_dn8)) * assign29400_e31903) - (assign29400_e31882 * (locals.var_mutau__blk862_dn8 + (((((((((((locals.var_nu__blk861_dn8 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn8)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29400_e31886 * locals.var_spsub_tau__blk874_dn8)) * locals.var_spsub_tau__blk874) + (assign29400_e31888 * locals.var_spsub_tau__blk874_dn8)) * locals.var_spsub_c__blk873) + (assign29400_e31890 * locals.var_spsub_c__blk873_dn8)) * assign29400_e31901) + (assign29400_e31892 * ((((locals.var_spsub_c__blk873_dn8 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn8)) * 0.3333333333333) - ((locals.var_spsub_a__blk871_dn8 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn8)))))))) / (assign29400_e31903 * assign29400_e31903))), (locals.var_spsub_eta__blk870_dn9 + (((((((locals.var_spsub_a__blk871_dn9 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn9)) * locals.var_spsub_tau__blk874) + (assign29400_e31880 * locals.var_spsub_tau__blk874_dn9)) * assign29400_e31903) - (assign29400_e31882 * (locals.var_mutau__blk862_dn9 + (((((((((((locals.var_nu__blk861_dn9 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn9)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29400_e31886 * locals.var_spsub_tau__blk874_dn9)) * locals.var_spsub_tau__blk874) + (assign29400_e31888 * locals.var_spsub_tau__blk874_dn9)) * locals.var_spsub_c__blk873) + (assign29400_e31890 * locals.var_spsub_c__blk873_dn9)) * assign29400_e31901) + (assign29400_e31892 * ((((locals.var_spsub_c__blk873_dn9 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn9)) * 0.3333333333333) - ((locals.var_spsub_a__blk871_dn9 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn9)))))))) / (assign29400_e31903 * assign29400_e31903))),)
    } else {
        (locals.var_spsub_x0__blk889, locals.var_spsub_x0__blk889_dn4, locals.var_spsub_x0__blk889_dn6, locals.var_spsub_x0__blk889_dn7, locals.var_spsub_x0__blk889_dn8, locals.var_spsub_x0__blk889_dn9,)
    }
};
        locals.var_spsub_x0__blk889 = assign29400_e31907;
        locals.var_spsub_x0__blk889_dn4 = assign29400_e31907_d_n4;
        locals.var_spsub_x0__blk889_dn6 = assign29400_e31907_d_n6;
        locals.var_spsub_x0__blk889_dn7 = assign29400_e31907_d_n7;
        locals.var_spsub_x0__blk889_dn8 = assign29400_e31907_d_n8;
        locals.var_spsub_x0__blk889_dn9 = assign29400_e31907_d_n9;

        let assign29410_e31910: f64 = if locals.var_spsub_x0__blk889 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1091 = assign29410_e31910;

        let (assign29420_e31925, assign29420_e31925_d_n4, assign29420_e31925_d_n6, assign29420_e31925_d_n7, assign29420_e31925_d_n8, assign29420_e31925_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 != 0.0)) {
        let assign29420_e31923: f64 = (locals.var_spsub_x0__blk889).exp();
        (assign29420_e31923, (assign29420_e31923 * locals.var_spsub_x0__blk889_dn4), (assign29420_e31923 * locals.var_spsub_x0__blk889_dn6), (assign29420_e31923 * locals.var_spsub_x0__blk889_dn7), (assign29420_e31923 * locals.var_spsub_x0__blk889_dn8), (assign29420_e31923 * locals.var_spsub_x0__blk889_dn9),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29420_e31925;
        locals.var_spsub_delta0__blk876_dn4 = assign29420_e31925_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29420_e31925_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29420_e31925_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29420_e31925_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29420_e31925_d_n9;

    }

    pub(super) fn stamp_transient_block_75(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29430_e31941, assign29430_e31941_d_n4, assign29430_e31941_d_n6, assign29430_e31941_d_n7, assign29430_e31941_d_n8, assign29430_e31941_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 != 0.0)) {
        let assign29430_e31939: f64 = (1.0 / locals.var_spsub_delta0__blk876);
        (assign29430_e31939, (-(locals.var_spsub_delta0__blk876_dn4 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn6 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn7 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn8 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn9 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))),)
    } else {
        (locals.var_spsub_delta1__blk877, locals.var_spsub_delta1__blk877_dn4, locals.var_spsub_delta1__blk877_dn6, locals.var_spsub_delta1__blk877_dn7, locals.var_spsub_delta1__blk877_dn8, locals.var_spsub_delta1__blk877_dn9,)
    }
};
        locals.var_spsub_delta1__blk877 = assign29430_e31941;
        locals.var_spsub_delta1__blk877_dn4 = assign29430_e31941_d_n4;
        locals.var_spsub_delta1__blk877_dn6 = assign29430_e31941_d_n6;
        locals.var_spsub_delta1__blk877_dn7 = assign29430_e31941_d_n7;
        locals.var_spsub_delta1__blk877_dn8 = assign29430_e31941_d_n8;
        locals.var_spsub_delta1__blk877_dn9 = assign29430_e31941_d_n9;

        let (assign29440_e31957, assign29440_e31957_d_n4, assign29440_e31957_d_n6, assign29440_e31957_d_n7, assign29440_e31957_d_n8, assign29440_e31957_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 != 0.0)) {
        let assign29440_e31955: f64 = (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876);
        (assign29440_e31955, ((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_delta0__blk876) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn4)), ((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_delta0__blk876) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn6)), ((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_delta0__blk876) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn7)), ((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_delta0__blk876) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn8)), ((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_delta0__blk876) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn9)),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29440_e31957;
        locals.var_spsub_delta0__blk876_dn4 = assign29440_e31957_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29440_e31957_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29440_e31957_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29440_e31957_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29440_e31957_d_n9;

        let assign29450_e31961: f64 = (locals.var_xn_sub - 80.0);
        let assign29450_e31962: f64 = if locals.var_spsub_x0__blk889 > assign29450_e31961 { 1.0 } else { 0.0 };
        locals.var_guard1092 = assign29450_e31962;

        let (assign29460_e31982, assign29460_e31982_d_n4, assign29460_e31982_d_n6, assign29460_e31982_d_n7, assign29460_e31982_d_n8, assign29460_e31982_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 == 0.0)) && (locals.var_guard1092 != 0.0)) {
        let assign29460_e31979: f64 = (locals.var_spsub_x0__blk889 - locals.var_xn_sub);
        let assign29460_e31980: f64 = (assign29460_e31979).exp();
        (assign29460_e31980, (assign29460_e31980 * (locals.var_spsub_x0__blk889_dn4 - locals.var_xn_sub_dn4)), (assign29460_e31980 * (locals.var_spsub_x0__blk889_dn6 - locals.var_xn_sub_dn6)), (assign29460_e31980 * (locals.var_spsub_x0__blk889_dn7 - locals.var_xn_sub_dn7)), (assign29460_e31980 * (locals.var_spsub_x0__blk889_dn8 - locals.var_xn_sub_dn8)), (assign29460_e31980 * (locals.var_spsub_x0__blk889_dn9 - locals.var_xn_sub_dn9)),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29460_e31982;
        locals.var_spsub_delta0__blk876_dn4 = assign29460_e31982_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29460_e31982_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29460_e31982_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29460_e31982_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29460_e31982_d_n9;

        let (assign29470_e32001, assign29470_e32001_d_n4, assign29470_e32001_d_n6, assign29470_e32001_d_n7, assign29470_e32001_d_n8, assign29470_e32001_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 == 0.0)) && (locals.var_guard1092 != 0.0)) {
        let assign29470_e31999: f64 = (locals.var_spsub_delta__blk867 / locals.var_spsub_delta0__blk876);
        (assign29470_e31999, (((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_delta0__blk876) - (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn4)) / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876)), (((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_delta0__blk876) - (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn6)) / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876)), (((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_delta0__blk876) - (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn7)) / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876)), (((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_delta0__blk876) - (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn8)) / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876)), (((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_delta0__blk876) - (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn9)) / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876)),)
    } else {
        (locals.var_spsub_delta1__blk877, locals.var_spsub_delta1__blk877_dn4, locals.var_spsub_delta1__blk877_dn6, locals.var_spsub_delta1__blk877_dn7, locals.var_spsub_delta1__blk877_dn8, locals.var_spsub_delta1__blk877_dn9,)
    }
};
        locals.var_spsub_delta1__blk877 = assign29470_e32001;
        locals.var_spsub_delta1__blk877_dn4 = assign29470_e32001_d_n4;
        locals.var_spsub_delta1__blk877_dn6 = assign29470_e32001_d_n6;
        locals.var_spsub_delta1__blk877_dn7 = assign29470_e32001_d_n7;
        locals.var_spsub_delta1__blk877_dn8 = assign29470_e32001_d_n8;
        locals.var_spsub_delta1__blk877_dn9 = assign29470_e32001_d_n9;

        let (assign29480_e32047, assign29480_e32047_d_n4, assign29480_e32047_d_n6, assign29480_e32047_d_n7, assign29480_e32047_d_n8, assign29480_e32047_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 == 0.0)) && (locals.var_guard1092 == 0.0)) {
        let assign29480_e32021: f64 = (locals.var_xn_sub - locals.var_spsub_x0__blk889);
        let assign29480_e32023: f64 = (assign29480_e32021 - 80.0);
        let assign29480_e32028: f64 = (locals.var_xn_sub - locals.var_spsub_x0__blk889);
        let assign29480_e32030: f64 = (assign29480_e32028 - 80.0);
        let assign29480_e32031: f64 = (0.5 * assign29480_e32030);
        let assign29480_e32035: f64 = (locals.var_xn_sub - locals.var_spsub_x0__blk889);
        let assign29480_e32037: f64 = (assign29480_e32035 - 80.0);
        let assign29480_e32039: f64 = (assign29480_e32037 * 0.3333333333333);
        let assign29480_e32040: f64 = (1.0 + assign29480_e32039);
        let assign29480_e32041: f64 = (assign29480_e32031 * assign29480_e32040);
        let assign29480_e32042: f64 = (1.0 + assign29480_e32041);
        let assign29480_e32043: f64 = (assign29480_e32023 * assign29480_e32042);
        let assign29480_e32044: f64 = (1.0 + assign29480_e32043);
        let assign29480_e32045: f64 = (1.80485e-35 / assign29480_e32044);
        (assign29480_e32045, (-((1.80485e-35 * (((locals.var_xn_sub_dn4 - locals.var_spsub_x0__blk889_dn4) * assign29480_e32042) + (assign29480_e32023 * (((0.5 * (locals.var_xn_sub_dn4 - locals.var_spsub_x0__blk889_dn4)) * assign29480_e32040) + (assign29480_e32031 * ((locals.var_xn_sub_dn4 - locals.var_spsub_x0__blk889_dn4) * 0.3333333333333)))))) / (assign29480_e32044 * assign29480_e32044))), (-((1.80485e-35 * (((locals.var_xn_sub_dn6 - locals.var_spsub_x0__blk889_dn6) * assign29480_e32042) + (assign29480_e32023 * (((0.5 * (locals.var_xn_sub_dn6 - locals.var_spsub_x0__blk889_dn6)) * assign29480_e32040) + (assign29480_e32031 * ((locals.var_xn_sub_dn6 - locals.var_spsub_x0__blk889_dn6) * 0.3333333333333)))))) / (assign29480_e32044 * assign29480_e32044))), (-((1.80485e-35 * (((locals.var_xn_sub_dn7 - locals.var_spsub_x0__blk889_dn7) * assign29480_e32042) + (assign29480_e32023 * (((0.5 * (locals.var_xn_sub_dn7 - locals.var_spsub_x0__blk889_dn7)) * assign29480_e32040) + (assign29480_e32031 * ((locals.var_xn_sub_dn7 - locals.var_spsub_x0__blk889_dn7) * 0.3333333333333)))))) / (assign29480_e32044 * assign29480_e32044))), (-((1.80485e-35 * (((locals.var_xn_sub_dn8 - locals.var_spsub_x0__blk889_dn8) * assign29480_e32042) + (assign29480_e32023 * (((0.5 * (locals.var_xn_sub_dn8 - locals.var_spsub_x0__blk889_dn8)) * assign29480_e32040) + (assign29480_e32031 * ((locals.var_xn_sub_dn8 - locals.var_spsub_x0__blk889_dn8) * 0.3333333333333)))))) / (assign29480_e32044 * assign29480_e32044))), (-((1.80485e-35 * (((locals.var_xn_sub_dn9 - locals.var_spsub_x0__blk889_dn9) * assign29480_e32042) + (assign29480_e32023 * (((0.5 * (locals.var_xn_sub_dn9 - locals.var_spsub_x0__blk889_dn9)) * assign29480_e32040) + (assign29480_e32031 * ((locals.var_xn_sub_dn9 - locals.var_spsub_x0__blk889_dn9) * 0.3333333333333)))))) / (assign29480_e32044 * assign29480_e32044))),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29480_e32047;
        locals.var_spsub_delta0__blk876_dn4 = assign29480_e32047_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29480_e32047_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29480_e32047_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29480_e32047_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29480_e32047_d_n9;

        let (assign29490_e32087, assign29490_e32087_d_n4, assign29490_e32087_d_n6, assign29490_e32087_d_n7, assign29490_e32087_d_n8, assign29490_e32087_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 == 0.0)) && (locals.var_guard1092 == 0.0)) {
        let assign29490_e32067: f64 = (locals.var_spsub_x0__blk889 - 80.0);
        let assign29490_e32072: f64 = (locals.var_spsub_x0__blk889 - 80.0);
        let assign29490_e32073: f64 = (0.5 * assign29490_e32072);
        let assign29490_e32077: f64 = (locals.var_spsub_x0__blk889 - 80.0);
        let assign29490_e32079: f64 = (assign29490_e32077 * 0.3333333333333);
        let assign29490_e32080: f64 = (1.0 + assign29490_e32079);
        let assign29490_e32081: f64 = (assign29490_e32073 * assign29490_e32080);
        let assign29490_e32082: f64 = (1.0 + assign29490_e32081);
        let assign29490_e32083: f64 = (assign29490_e32067 * assign29490_e32082);
        let assign29490_e32084: f64 = (1.0 + assign29490_e32083);
        let assign29490_e32085: f64 = (1.80485e-35 / assign29490_e32084);
        (assign29490_e32085, (-((1.80485e-35 * ((locals.var_spsub_x0__blk889_dn4 * assign29490_e32082) + (assign29490_e32067 * (((0.5 * locals.var_spsub_x0__blk889_dn4) * assign29490_e32080) + (assign29490_e32073 * (locals.var_spsub_x0__blk889_dn4 * 0.3333333333333)))))) / (assign29490_e32084 * assign29490_e32084))), (-((1.80485e-35 * ((locals.var_spsub_x0__blk889_dn6 * assign29490_e32082) + (assign29490_e32067 * (((0.5 * locals.var_spsub_x0__blk889_dn6) * assign29490_e32080) + (assign29490_e32073 * (locals.var_spsub_x0__blk889_dn6 * 0.3333333333333)))))) / (assign29490_e32084 * assign29490_e32084))), (-((1.80485e-35 * ((locals.var_spsub_x0__blk889_dn7 * assign29490_e32082) + (assign29490_e32067 * (((0.5 * locals.var_spsub_x0__blk889_dn7) * assign29490_e32080) + (assign29490_e32073 * (locals.var_spsub_x0__blk889_dn7 * 0.3333333333333)))))) / (assign29490_e32084 * assign29490_e32084))), (-((1.80485e-35 * ((locals.var_spsub_x0__blk889_dn8 * assign29490_e32082) + (assign29490_e32067 * (((0.5 * locals.var_spsub_x0__blk889_dn8) * assign29490_e32080) + (assign29490_e32073 * (locals.var_spsub_x0__blk889_dn8 * 0.3333333333333)))))) / (assign29490_e32084 * assign29490_e32084))), (-((1.80485e-35 * ((locals.var_spsub_x0__blk889_dn9 * assign29490_e32082) + (assign29490_e32067 * (((0.5 * locals.var_spsub_x0__blk889_dn9) * assign29490_e32080) + (assign29490_e32073 * (locals.var_spsub_x0__blk889_dn9 * 0.3333333333333)))))) / (assign29490_e32084 * assign29490_e32084))),)
    } else {
        (locals.var_spsub_delta1__blk877, locals.var_spsub_delta1__blk877_dn4, locals.var_spsub_delta1__blk877_dn6, locals.var_spsub_delta1__blk877_dn7, locals.var_spsub_delta1__blk877_dn8, locals.var_spsub_delta1__blk877_dn9,)
    }
};
        locals.var_spsub_delta1__blk877 = assign29490_e32087;
        locals.var_spsub_delta1__blk877_dn4 = assign29490_e32087_d_n4;
        locals.var_spsub_delta1__blk877_dn6 = assign29490_e32087_d_n6;
        locals.var_spsub_delta1__blk877_dn7 = assign29490_e32087_d_n7;
        locals.var_spsub_delta1__blk877_dn8 = assign29490_e32087_d_n8;
        locals.var_spsub_delta1__blk877_dn9 = assign29490_e32087_d_n9;

        let (assign29500_e32105, assign29500_e32105_d_n4, assign29500_e32105_d_n6, assign29500_e32105_d_n7, assign29500_e32105_d_n8, assign29500_e32105_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29500_e32101: f64 = (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889);
        let assign29500_e32102: f64 = (2.0 + assign29500_e32101);
        let assign29500_e32103: f64 = (1.0 / assign29500_e32102);
        (assign29500_e32103, (-(((locals.var_spsub_x0__blk889_dn4 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn4)) / (assign29500_e32102 * assign29500_e32102))), (-(((locals.var_spsub_x0__blk889_dn6 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn6)) / (assign29500_e32102 * assign29500_e32102))), (-(((locals.var_spsub_x0__blk889_dn7 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn7)) / (assign29500_e32102 * assign29500_e32102))), (-(((locals.var_spsub_x0__blk889_dn8 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn8)) / (assign29500_e32102 * assign29500_e32102))), (-(((locals.var_spsub_x0__blk889_dn9 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn9)) / (assign29500_e32102 * assign29500_e32102))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29500_e32105;
        locals.var_spsub_temp__blk863_dn4 = assign29500_e32105_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29500_e32105_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29500_e32105_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29500_e32105_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29500_e32105_d_n9;

        let (assign29510_e32121, assign29510_e32121_d_n4, assign29510_e32121_d_n6, assign29510_e32121_d_n7, assign29510_e32121_d_n8, assign29510_e32121_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29510_e32117: f64 = (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889);
        let assign29510_e32119: f64 = (assign29510_e32117 * locals.var_spsub_temp__blk863);
        (assign29510_e32119, ((((locals.var_spsub_x0__blk889_dn4 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn4)) * locals.var_spsub_temp__blk863) + (assign29510_e32117 * locals.var_spsub_temp__blk863_dn4)), ((((locals.var_spsub_x0__blk889_dn6 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn6)) * locals.var_spsub_temp__blk863) + (assign29510_e32117 * locals.var_spsub_temp__blk863_dn6)), ((((locals.var_spsub_x0__blk889_dn7 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn7)) * locals.var_spsub_temp__blk863) + (assign29510_e32117 * locals.var_spsub_temp__blk863_dn7)), ((((locals.var_spsub_x0__blk889_dn8 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn8)) * locals.var_spsub_temp__blk863) + (assign29510_e32117 * locals.var_spsub_temp__blk863_dn8)), ((((locals.var_spsub_x0__blk889_dn9 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn9)) * locals.var_spsub_temp__blk863) + (assign29510_e32117 * locals.var_spsub_temp__blk863_dn9)),)
    } else {
        (locals.var_spsub_xi0__blk878, locals.var_spsub_xi0__blk878_dn4, locals.var_spsub_xi0__blk878_dn6, locals.var_spsub_xi0__blk878_dn7, locals.var_spsub_xi0__blk878_dn8, locals.var_spsub_xi0__blk878_dn9,)
    }
};
        locals.var_spsub_xi0__blk878 = assign29510_e32121;
        locals.var_spsub_xi0__blk878_dn4 = assign29510_e32121_d_n4;
        locals.var_spsub_xi0__blk878_dn6 = assign29510_e32121_d_n6;
        locals.var_spsub_xi0__blk878_dn7 = assign29510_e32121_d_n7;
        locals.var_spsub_xi0__blk878_dn8 = assign29510_e32121_d_n8;
        locals.var_spsub_xi0__blk878_dn9 = assign29510_e32121_d_n9;

        let (assign29520_e32139, assign29520_e32139_d_n4, assign29520_e32139_d_n6, assign29520_e32139_d_n7, assign29520_e32139_d_n8, assign29520_e32139_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29520_e32134: f64 = (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863);
        let assign29520_e32136: f64 = (assign29520_e32134 * locals.var_spsub_temp__blk863);
        let assign29520_e32137: f64 = (4.0 * assign29520_e32136);
        (assign29520_e32137, (4.0 * ((((locals.var_spsub_x0__blk889_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863_dn4)) * locals.var_spsub_temp__blk863) + (assign29520_e32134 * locals.var_spsub_temp__blk863_dn4))), (4.0 * ((((locals.var_spsub_x0__blk889_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863_dn6)) * locals.var_spsub_temp__blk863) + (assign29520_e32134 * locals.var_spsub_temp__blk863_dn6))), (4.0 * ((((locals.var_spsub_x0__blk889_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863_dn7)) * locals.var_spsub_temp__blk863) + (assign29520_e32134 * locals.var_spsub_temp__blk863_dn7))), (4.0 * ((((locals.var_spsub_x0__blk889_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863_dn8)) * locals.var_spsub_temp__blk863) + (assign29520_e32134 * locals.var_spsub_temp__blk863_dn8))), (4.0 * ((((locals.var_spsub_x0__blk889_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863_dn9)) * locals.var_spsub_temp__blk863) + (assign29520_e32134 * locals.var_spsub_temp__blk863_dn9))),)
    } else {
        (locals.var_spsub_xi1__blk879, locals.var_spsub_xi1__blk879_dn4, locals.var_spsub_xi1__blk879_dn6, locals.var_spsub_xi1__blk879_dn7, locals.var_spsub_xi1__blk879_dn8, locals.var_spsub_xi1__blk879_dn9,)
    }
};
        locals.var_spsub_xi1__blk879 = assign29520_e32139;
        locals.var_spsub_xi1__blk879_dn4 = assign29520_e32139_d_n4;
        locals.var_spsub_xi1__blk879_dn6 = assign29520_e32139_d_n6;
        locals.var_spsub_xi1__blk879_dn7 = assign29520_e32139_d_n7;
        locals.var_spsub_xi1__blk879_dn8 = assign29520_e32139_d_n8;
        locals.var_spsub_xi1__blk879_dn9 = assign29520_e32139_d_n9;

        let (assign29530_e32161, assign29530_e32161_d_n4, assign29530_e32161_d_n6, assign29530_e32161_d_n7, assign29530_e32161_d_n8, assign29530_e32161_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29530_e32151: f64 = (8.0 * locals.var_spsub_temp__blk863);
        let assign29530_e32154: f64 = (12.0 * locals.var_spsub_xi0__blk878);
        let assign29530_e32155: f64 = (assign29530_e32151 - assign29530_e32154);
        let assign29530_e32157: f64 = (assign29530_e32155 * locals.var_spsub_temp__blk863);
        let assign29530_e32159: f64 = (assign29530_e32157 * locals.var_spsub_temp__blk863);
        (assign29530_e32159, ((((((8.0 * locals.var_spsub_temp__blk863_dn4) - (12.0 * locals.var_spsub_xi0__blk878_dn4)) * locals.var_spsub_temp__blk863) + (assign29530_e32155 * locals.var_spsub_temp__blk863_dn4)) * locals.var_spsub_temp__blk863) + (assign29530_e32157 * locals.var_spsub_temp__blk863_dn4)), ((((((8.0 * locals.var_spsub_temp__blk863_dn6) - (12.0 * locals.var_spsub_xi0__blk878_dn6)) * locals.var_spsub_temp__blk863) + (assign29530_e32155 * locals.var_spsub_temp__blk863_dn6)) * locals.var_spsub_temp__blk863) + (assign29530_e32157 * locals.var_spsub_temp__blk863_dn6)), ((((((8.0 * locals.var_spsub_temp__blk863_dn7) - (12.0 * locals.var_spsub_xi0__blk878_dn7)) * locals.var_spsub_temp__blk863) + (assign29530_e32155 * locals.var_spsub_temp__blk863_dn7)) * locals.var_spsub_temp__blk863) + (assign29530_e32157 * locals.var_spsub_temp__blk863_dn7)), ((((((8.0 * locals.var_spsub_temp__blk863_dn8) - (12.0 * locals.var_spsub_xi0__blk878_dn8)) * locals.var_spsub_temp__blk863) + (assign29530_e32155 * locals.var_spsub_temp__blk863_dn8)) * locals.var_spsub_temp__blk863) + (assign29530_e32157 * locals.var_spsub_temp__blk863_dn8)), ((((((8.0 * locals.var_spsub_temp__blk863_dn9) - (12.0 * locals.var_spsub_xi0__blk878_dn9)) * locals.var_spsub_temp__blk863) + (assign29530_e32155 * locals.var_spsub_temp__blk863_dn9)) * locals.var_spsub_temp__blk863) + (assign29530_e32157 * locals.var_spsub_temp__blk863_dn9)),)
    } else {
        (locals.var_spsub_xi2__blk880, locals.var_spsub_xi2__blk880_dn4, locals.var_spsub_xi2__blk880_dn6, locals.var_spsub_xi2__blk880_dn7, locals.var_spsub_xi2__blk880_dn8, locals.var_spsub_xi2__blk880_dn9,)
    }
};
        locals.var_spsub_xi2__blk880 = assign29530_e32161;
        locals.var_spsub_xi2__blk880_dn4 = assign29530_e32161_d_n4;
        locals.var_spsub_xi2__blk880_dn6 = assign29530_e32161_d_n6;
        locals.var_spsub_xi2__blk880_dn7 = assign29530_e32161_d_n7;
        locals.var_spsub_xi2__blk880_dn8 = assign29530_e32161_d_n8;
        locals.var_spsub_xi2__blk880_dn9 = assign29530_e32161_d_n9;

        let (assign29540_e32175, assign29540_e32175_d_n4, assign29540_e32175_d_n6, assign29540_e32175_d_n7, assign29540_e32175_d_n8, assign29540_e32175_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29540_e32173: f64 = (locals.var_spsub_xgb__blk866 - locals.var_spsub_x0__blk889);
        (assign29540_e32173, (locals.var_spsub_xgb__blk866_dn4 - locals.var_spsub_x0__blk889_dn4), (locals.var_spsub_xgb__blk866_dn6 - locals.var_spsub_x0__blk889_dn6), (locals.var_spsub_xgb__blk866_dn7 - locals.var_spsub_x0__blk889_dn7), (locals.var_spsub_xgb__blk866_dn8 - locals.var_spsub_x0__blk889_dn8), (locals.var_spsub_xgb__blk866_dn9 - locals.var_spsub_x0__blk889_dn9),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29540_e32175;
        locals.var_spsub_temp__blk863_dn4 = assign29540_e32175_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29540_e32175_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29540_e32175_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29540_e32175_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29540_e32175_d_n9;

        let (assign29550_e32203, assign29550_e32203_d_n4, assign29550_e32203_d_n6, assign29550_e32203_d_n7, assign29550_e32203_d_n8, assign29550_e32203_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29550_e32187: f64 = (2.0 * locals.var_spsub_temp__blk863);
        let assign29550_e32191: f64 = (1.0 - locals.var_spsub_delta1__blk877);
        let assign29550_e32193: f64 = (assign29550_e32191 + locals.var_spsub_delta0__blk876);
        let assign29550_e32197: f64 = (1.0 + locals.var_spsub_xi1__blk879);
        let assign29550_e32198: f64 = (locals.var_spsub_delta__blk867 * assign29550_e32197);
        let assign29550_e32199: f64 = (assign29550_e32193 - assign29550_e32198);
        let assign29550_e32200: f64 = (locals.var_gfsub2 * assign29550_e32199);
        let assign29550_e32201: f64 = (assign29550_e32187 + assign29550_e32200);
        (assign29550_e32201, ((2.0 * locals.var_spsub_temp__blk863_dn4) + ((locals.var_gfsub2_dn4 * assign29550_e32199) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1__blk877_dn4) + locals.var_spsub_delta0__blk876_dn4) - ((locals.var_spsub_delta__blk867_dn4 * assign29550_e32197) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn4)))))), ((2.0 * locals.var_spsub_temp__blk863_dn6) + ((locals.var_gfsub2_dn6 * assign29550_e32199) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1__blk877_dn6) + locals.var_spsub_delta0__blk876_dn6) - ((locals.var_spsub_delta__blk867_dn6 * assign29550_e32197) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn6)))))), ((2.0 * locals.var_spsub_temp__blk863_dn7) + ((locals.var_gfsub2_dn7 * assign29550_e32199) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1__blk877_dn7) + locals.var_spsub_delta0__blk876_dn7) - ((locals.var_spsub_delta__blk867_dn7 * assign29550_e32197) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn7)))))), ((2.0 * locals.var_spsub_temp__blk863_dn8) + ((locals.var_gfsub2_dn8 * assign29550_e32199) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1__blk877_dn8) + locals.var_spsub_delta0__blk876_dn8) - ((locals.var_spsub_delta__blk867_dn8 * assign29550_e32197) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn8)))))), ((2.0 * locals.var_spsub_temp__blk863_dn9) + ((locals.var_gfsub2_dn9 * assign29550_e32199) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1__blk877_dn9) + locals.var_spsub_delta0__blk876_dn9) - ((locals.var_spsub_delta__blk867_dn9 * assign29550_e32197) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn9)))))),)
    } else {
        (locals.var_spsub_pc__blk881, locals.var_spsub_pc__blk881_dn4, locals.var_spsub_pc__blk881_dn6, locals.var_spsub_pc__blk881_dn7, locals.var_spsub_pc__blk881_dn8, locals.var_spsub_pc__blk881_dn9,)
    }
};
        locals.var_spsub_pc__blk881 = assign29550_e32203;
        locals.var_spsub_pc__blk881_dn4 = assign29550_e32203_d_n4;
        locals.var_spsub_pc__blk881_dn6 = assign29550_e32203_d_n6;
        locals.var_spsub_pc__blk881_dn7 = assign29550_e32203_d_n7;
        locals.var_spsub_pc__blk881_dn8 = assign29550_e32203_d_n8;
        locals.var_spsub_pc__blk881_dn9 = assign29550_e32203_d_n9;

        let (assign29560_e32235, assign29560_e32235_d_n4, assign29560_e32235_d_n6, assign29560_e32235_d_n7, assign29560_e32235_d_n8, assign29560_e32235_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29560_e32215: f64 = (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863);
        let assign29560_e32219: f64 = (locals.var_spsub_delta1__blk877 + locals.var_spsub_x0__blk889);
        let assign29560_e32221: f64 = (assign29560_e32219 - 1.0);
        let assign29560_e32223: f64 = (assign29560_e32221 + locals.var_spsub_delta0__blk876);
        let assign29560_e32227: f64 = (locals.var_spsub_x0__blk889 + 1.0);
        let assign29560_e32229: f64 = (assign29560_e32227 + locals.var_spsub_xi0__blk878);
        let assign29560_e32230: f64 = (locals.var_spsub_delta__blk867 * assign29560_e32229);
        let assign29560_e32231: f64 = (assign29560_e32223 - assign29560_e32230);
        let assign29560_e32232: f64 = (locals.var_gfsub2 * assign29560_e32231);
        let assign29560_e32233: f64 = (assign29560_e32215 - assign29560_e32232);
        (assign29560_e32233, (((locals.var_spsub_temp__blk863_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn4)) - ((locals.var_gfsub2_dn4 * assign29560_e32231) + (locals.var_gfsub2 * (((locals.var_spsub_delta1__blk877_dn4 + locals.var_spsub_x0__blk889_dn4) + locals.var_spsub_delta0__blk876_dn4) - ((locals.var_spsub_delta__blk867_dn4 * assign29560_e32229) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_x0__blk889_dn4 + locals.var_spsub_xi0__blk878_dn4))))))), (((locals.var_spsub_temp__blk863_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn6)) - ((locals.var_gfsub2_dn6 * assign29560_e32231) + (locals.var_gfsub2 * (((locals.var_spsub_delta1__blk877_dn6 + locals.var_spsub_x0__blk889_dn6) + locals.var_spsub_delta0__blk876_dn6) - ((locals.var_spsub_delta__blk867_dn6 * assign29560_e32229) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_x0__blk889_dn6 + locals.var_spsub_xi0__blk878_dn6))))))), (((locals.var_spsub_temp__blk863_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn7)) - ((locals.var_gfsub2_dn7 * assign29560_e32231) + (locals.var_gfsub2 * (((locals.var_spsub_delta1__blk877_dn7 + locals.var_spsub_x0__blk889_dn7) + locals.var_spsub_delta0__blk876_dn7) - ((locals.var_spsub_delta__blk867_dn7 * assign29560_e32229) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_x0__blk889_dn7 + locals.var_spsub_xi0__blk878_dn7))))))), (((locals.var_spsub_temp__blk863_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn8)) - ((locals.var_gfsub2_dn8 * assign29560_e32231) + (locals.var_gfsub2 * (((locals.var_spsub_delta1__blk877_dn8 + locals.var_spsub_x0__blk889_dn8) + locals.var_spsub_delta0__blk876_dn8) - ((locals.var_spsub_delta__blk867_dn8 * assign29560_e32229) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_x0__blk889_dn8 + locals.var_spsub_xi0__blk878_dn8))))))), (((locals.var_spsub_temp__blk863_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn9)) - ((locals.var_gfsub2_dn9 * assign29560_e32231) + (locals.var_gfsub2 * (((locals.var_spsub_delta1__blk877_dn9 + locals.var_spsub_x0__blk889_dn9) + locals.var_spsub_delta0__blk876_dn9) - ((locals.var_spsub_delta__blk867_dn9 * assign29560_e32229) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_x0__blk889_dn9 + locals.var_spsub_xi0__blk878_dn9))))))),)
    } else {
        (locals.var_spsub_qc__blk882, locals.var_spsub_qc__blk882_dn4, locals.var_spsub_qc__blk882_dn6, locals.var_spsub_qc__blk882_dn7, locals.var_spsub_qc__blk882_dn8, locals.var_spsub_qc__blk882_dn9,)
    }
};
        locals.var_spsub_qc__blk882 = assign29560_e32235;
        locals.var_spsub_qc__blk882_dn4 = assign29560_e32235_d_n4;
        locals.var_spsub_qc__blk882_dn6 = assign29560_e32235_d_n6;
        locals.var_spsub_qc__blk882_dn7 = assign29560_e32235_d_n7;
        locals.var_spsub_qc__blk882_dn8 = assign29560_e32235_d_n8;
        locals.var_spsub_qc__blk882_dn9 = assign29560_e32235_d_n9;

        let (assign29570_e32257, assign29570_e32257_d_n4, assign29570_e32257_d_n6, assign29570_e32257_d_n7, assign29570_e32257_d_n8, assign29570_e32257_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29570_e32249: f64 = (locals.var_spsub_delta1__blk877 + locals.var_spsub_delta0__blk876);
        let assign29570_e32252: f64 = (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880);
        let assign29570_e32253: f64 = (assign29570_e32249 - assign29570_e32252);
        let assign29570_e32254: f64 = (locals.var_gfsub2 * assign29570_e32253);
        let assign29570_e32255: f64 = (2.0 - assign29570_e32254);
        (assign29570_e32255, (-((locals.var_gfsub2_dn4 * assign29570_e32253) + (locals.var_gfsub2 * ((locals.var_spsub_delta1__blk877_dn4 + locals.var_spsub_delta0__blk876_dn4) - ((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn4)))))), (-((locals.var_gfsub2_dn6 * assign29570_e32253) + (locals.var_gfsub2 * ((locals.var_spsub_delta1__blk877_dn6 + locals.var_spsub_delta0__blk876_dn6) - ((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn6)))))), (-((locals.var_gfsub2_dn7 * assign29570_e32253) + (locals.var_gfsub2 * ((locals.var_spsub_delta1__blk877_dn7 + locals.var_spsub_delta0__blk876_dn7) - ((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn7)))))), (-((locals.var_gfsub2_dn8 * assign29570_e32253) + (locals.var_gfsub2 * ((locals.var_spsub_delta1__blk877_dn8 + locals.var_spsub_delta0__blk876_dn8) - ((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn8)))))), (-((locals.var_gfsub2_dn9 * assign29570_e32253) + (locals.var_gfsub2 * ((locals.var_spsub_delta1__blk877_dn9 + locals.var_spsub_delta0__blk876_dn9) - ((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn9)))))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29570_e32257;
        locals.var_spsub_temp__blk863_dn4 = assign29570_e32257_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29570_e32257_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29570_e32257_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29570_e32257_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29570_e32257_d_n9;

        let (assign29580_e32277, assign29580_e32277_d_n4, assign29580_e32277_d_n6, assign29580_e32277_d_n7, assign29580_e32277_d_n8, assign29580_e32277_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29580_e32269: f64 = (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881);
        let assign29580_e32273: f64 = (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863);
        let assign29580_e32274: f64 = (2.0 * assign29580_e32273);
        let assign29580_e32275: f64 = (assign29580_e32269 - assign29580_e32274);
        (assign29580_e32275, (((locals.var_spsub_pc__blk881_dn4 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn4)) - (2.0 * ((locals.var_spsub_qc__blk882_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn4)))), (((locals.var_spsub_pc__blk881_dn6 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn6)) - (2.0 * ((locals.var_spsub_qc__blk882_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn6)))), (((locals.var_spsub_pc__blk881_dn7 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn7)) - (2.0 * ((locals.var_spsub_qc__blk882_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn7)))), (((locals.var_spsub_pc__blk881_dn8 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn8)) - (2.0 * ((locals.var_spsub_qc__blk882_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn8)))), (((locals.var_spsub_pc__blk881_dn9 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn9)) - (2.0 * ((locals.var_spsub_qc__blk882_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn9)))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29580_e32277;
        locals.var_spsub_temp__blk863_dn4 = assign29580_e32277_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29580_e32277_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29580_e32277_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29580_e32277_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29580_e32277_d_n9;

        let (assign29590_e32298, assign29590_e32298_d_n4, assign29590_e32298_d_n6, assign29590_e32298_d_n7, assign29590_e32298_d_n8, assign29590_e32298_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29590_e32292: f64 = (locals.var_spsub_temp__blk863).sqrt();
        let assign29590_e32293: f64 = (locals.var_spsub_pc__blk881 + assign29590_e32292);
        let assign29590_e32294: f64 = (locals.var_spsub_qc__blk882 / assign29590_e32293);
        let assign29590_e32295: f64 = (2.0 * assign29590_e32294);
        let assign29590_e32296: f64 = (locals.var_spsub_x0__blk889 + assign29590_e32295);
        (assign29590_e32296, (locals.var_spsub_x0__blk889_dn4 + (2.0 * (((locals.var_spsub_qc__blk882_dn4 * assign29590_e32293) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn4 + (locals.var_spsub_temp__blk863_dn4 / (2.0 * assign29590_e32292))))) / (assign29590_e32293 * assign29590_e32293)))), (locals.var_spsub_x0__blk889_dn6 + (2.0 * (((locals.var_spsub_qc__blk882_dn6 * assign29590_e32293) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn6 + (locals.var_spsub_temp__blk863_dn6 / (2.0 * assign29590_e32292))))) / (assign29590_e32293 * assign29590_e32293)))), (locals.var_spsub_x0__blk889_dn7 + (2.0 * (((locals.var_spsub_qc__blk882_dn7 * assign29590_e32293) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn7 + (locals.var_spsub_temp__blk863_dn7 / (2.0 * assign29590_e32292))))) / (assign29590_e32293 * assign29590_e32293)))), (locals.var_spsub_x0__blk889_dn8 + (2.0 * (((locals.var_spsub_qc__blk882_dn8 * assign29590_e32293) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn8 + (locals.var_spsub_temp__blk863_dn8 / (2.0 * assign29590_e32292))))) / (assign29590_e32293 * assign29590_e32293)))), (locals.var_spsub_x0__blk889_dn9 + (2.0 * (((locals.var_spsub_qc__blk882_dn9 * assign29590_e32293) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn9 + (locals.var_spsub_temp__blk863_dn9 / (2.0 * assign29590_e32292))))) / (assign29590_e32293 * assign29590_e32293)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign29590_e32298;
        locals.var_temp3_dn4 = assign29590_e32298_d_n4;
        locals.var_temp3_dn6 = assign29590_e32298_d_n6;
        locals.var_temp3_dn7 = assign29590_e32298_d_n7;
        locals.var_temp3_dn8 = assign29590_e32298_d_n8;
        locals.var_temp3_dn9 = assign29590_e32298_d_n9;

        let (assign29600_e32308, assign29600_e32308_d_n4, assign29600_e32308_d_n6, assign29600_e32308_d_n7, assign29600_e32308_d_n8, assign29600_e32308_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign29600_e32305: f64 = (locals.var_temp3 + locals.var_temp2);
        let assign29600_e32306: f64 = (locals.var_temp * assign29600_e32305);
        (assign29600_e32306, ((locals.var_temp_dn4 * assign29600_e32305) + (locals.var_temp * (locals.var_temp3_dn4 + locals.var_temp2_dn4))), ((locals.var_temp_dn6 * assign29600_e32305) + (locals.var_temp * (locals.var_temp3_dn6 + locals.var_temp2_dn6))), ((locals.var_temp_dn7 * assign29600_e32305) + (locals.var_temp * (locals.var_temp3_dn7 + locals.var_temp2_dn7))), ((locals.var_temp_dn8 * assign29600_e32305) + (locals.var_temp * (locals.var_temp3_dn8 + locals.var_temp2_dn8))), ((locals.var_temp_dn9 * assign29600_e32305) + (locals.var_temp * (locals.var_temp3_dn9 + locals.var_temp2_dn9))),)
    } else {
        (locals.var_xg2eff__blk910, locals.var_xg2eff__blk910_dn4, locals.var_xg2eff__blk910_dn6, locals.var_xg2eff__blk910_dn7, locals.var_xg2eff__blk910_dn8, locals.var_xg2eff__blk910_dn9,)
    }
};
        locals.var_xg2eff__blk910 = assign29600_e32308;
        locals.var_xg2eff__blk910_dn4 = assign29600_e32308_d_n4;
        locals.var_xg2eff__blk910_dn6 = assign29600_e32308_d_n6;
        locals.var_xg2eff__blk910_dn7 = assign29600_e32308_d_n7;
        locals.var_xg2eff__blk910_dn8 = assign29600_e32308_d_n8;
        locals.var_xg2eff__blk910_dn9 = assign29600_e32308_d_n9;

        let (assign29610_e32315, assign29610_e32315_d_n4, assign29610_e32315_d_n6, assign29610_e32315_d_n7, assign29610_e32315_d_n8, assign29610_e32315_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 == 0.0)) {
        (locals.var_xg20__blk901, locals.var_xg20__blk901_dn4, locals.var_xg20__blk901_dn6, locals.var_xg20__blk901_dn7, locals.var_xg20__blk901_dn8, locals.var_xg20__blk901_dn9,)
    } else {
        (locals.var_xg2eff__blk910, locals.var_xg2eff__blk910_dn4, locals.var_xg2eff__blk910_dn6, locals.var_xg2eff__blk910_dn7, locals.var_xg2eff__blk910_dn8, locals.var_xg2eff__blk910_dn9,)
    }
};
        locals.var_xg2eff__blk910 = assign29610_e32315;
        locals.var_xg2eff__blk910_dn4 = assign29610_e32315_d_n4;
        locals.var_xg2eff__blk910_dn6 = assign29610_e32315_d_n6;
        locals.var_xg2eff__blk910_dn7 = assign29610_e32315_d_n7;
        locals.var_xg2eff__blk910_dn8 = assign29610_e32315_d_n8;
        locals.var_xg2eff__blk910_dn9 = assign29610_e32315_d_n9;

        let (assign29620_e32323, assign29620_e32323_d_n4, assign29620_e32323_d_n6, assign29620_e32323_d_n7, assign29620_e32323_d_n8, assign29620_e32323_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign29620_e32320: f64 = (locals.var_xg10__blk899 - locals.var_xg2eff__blk910);
        let assign29620_e32321: f64 = (locals.var_keq_1d * assign29620_e32320);
        (assign29620_e32321, (locals.var_keq_1d * (locals.var_xg10__blk899_dn4 - locals.var_xg2eff__blk910_dn4)), (locals.var_keq_1d * (locals.var_xg10__blk899_dn6 - locals.var_xg2eff__blk910_dn6)), (locals.var_keq_1d * (locals.var_xg10__blk899_dn7 - locals.var_xg2eff__blk910_dn7)), (locals.var_keq_1d * (locals.var_xg10__blk899_dn8 - locals.var_xg2eff__blk910_dn8)), (locals.var_keq_1d * (locals.var_xg10__blk899_dn9 - locals.var_xg2eff__blk910_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign29620_e32323;
        locals.var_temp_dn4 = assign29620_e32323_d_n4;
        locals.var_temp_dn6 = assign29620_e32323_d_n6;
        locals.var_temp_dn7 = assign29620_e32323_d_n7;
        locals.var_temp_dn8 = assign29620_e32323_d_n8;
        locals.var_temp_dn9 = assign29620_e32323_d_n9;

        let assign29630_e32326: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1093 = assign29630_e32326;

        let (assign29640_e32349, assign29640_e32349_d_n4, assign29640_e32349_d_n6, assign29640_e32349_d_n7, assign29640_e32349_d_n8, assign29640_e32349_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29640_e32333: f64 = (locals.var_temp + locals.var_emin);
        let assign29640_e32336: f64 = (locals.var_temp - locals.var_emin);
        let assign29640_e32339: f64 = (locals.var_temp - locals.var_emin);
        let assign29640_e32340: f64 = (assign29640_e32336 * assign29640_e32339);
        let assign29640_e32343: f64 = (locals.var_emin * locals.var_emin);
        let assign29640_e32344: f64 = (assign29640_e32340 + assign29640_e32343);
        let assign29640_e32345: f64 = (assign29640_e32344).sqrt();
        let assign29640_e32346: f64 = (assign29640_e32333 + assign29640_e32345);
        let assign29640_e32347: f64 = (0.5 * assign29640_e32346);
        (assign29640_e32347, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign29640_e32339) + (assign29640_e32336 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign29640_e32345)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign29640_e32339) + (assign29640_e32336 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign29640_e32345)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign29640_e32339) + (assign29640_e32336 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign29640_e32345)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign29640_e32339) + (assign29640_e32336 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign29640_e32345)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign29640_e32339) + (assign29640_e32336 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign29640_e32345)))),)
    } else {
        (locals.var_e1__blk911, locals.var_e1__blk911_dn4, locals.var_e1__blk911_dn6, locals.var_e1__blk911_dn7, locals.var_e1__blk911_dn8, locals.var_e1__blk911_dn9,)
    }
};
        locals.var_e1__blk911 = assign29640_e32349;
        locals.var_e1__blk911_dn4 = assign29640_e32349_d_n4;
        locals.var_e1__blk911_dn6 = assign29640_e32349_d_n6;
        locals.var_e1__blk911_dn7 = assign29640_e32349_d_n7;
        locals.var_e1__blk911_dn8 = assign29640_e32349_d_n8;
        locals.var_e1__blk911_dn9 = assign29640_e32349_d_n9;

        let (assign29650_e32375, assign29650_e32375_d_n4, assign29650_e32375_d_n6, assign29650_e32375_d_n7, assign29650_e32375_d_n8, assign29650_e32375_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29650_e32355: f64 = (-locals.var_temp);
        let assign29650_e32357: f64 = (assign29650_e32355 + locals.var_emin);
        let assign29650_e32359: f64 = (-locals.var_temp);
        let assign29650_e32361: f64 = (assign29650_e32359 - locals.var_emin);
        let assign29650_e32363: f64 = (-locals.var_temp);
        let assign29650_e32365: f64 = (assign29650_e32363 - locals.var_emin);
        let assign29650_e32366: f64 = (assign29650_e32361 * assign29650_e32365);
        let assign29650_e32369: f64 = (locals.var_emin * locals.var_emin);
        let assign29650_e32370: f64 = (assign29650_e32366 + assign29650_e32369);
        let assign29650_e32371: f64 = (assign29650_e32370).sqrt();
        let assign29650_e32372: f64 = (assign29650_e32357 + assign29650_e32371);
        let assign29650_e32373: f64 = (0.5 * assign29650_e32372);
        (assign29650_e32373, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign29650_e32365) + (assign29650_e32361 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign29650_e32371)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign29650_e32365) + (assign29650_e32361 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign29650_e32371)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign29650_e32365) + (assign29650_e32361 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign29650_e32371)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign29650_e32365) + (assign29650_e32361 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign29650_e32371)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign29650_e32365) + (assign29650_e32361 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign29650_e32371)))),)
    } else {
        (locals.var_e2__blk912, locals.var_e2__blk912_dn4, locals.var_e2__blk912_dn6, locals.var_e2__blk912_dn7, locals.var_e2__blk912_dn8, locals.var_e2__blk912_dn9,)
    }
};
        locals.var_e2__blk912 = assign29650_e32375;
        locals.var_e2__blk912_dn4 = assign29650_e32375_d_n4;
        locals.var_e2__blk912_dn6 = assign29650_e32375_d_n6;
        locals.var_e2__blk912_dn7 = assign29650_e32375_d_n7;
        locals.var_e2__blk912_dn8 = assign29650_e32375_d_n8;
        locals.var_e2__blk912_dn9 = assign29650_e32375_d_n9;

        let (assign29660_e32388, assign29660_e32388_d_n4, assign29660_e32388_d_n6, assign29660_e32388_d_n7, assign29660_e32388_d_n8, assign29660_e32388_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29660_e32381: f64 = (-0.3333333333333);
        let assign29660_e32383: f64 = (locals.var_e1__blk911).ln();
        let assign29660_e32384: f64 = (assign29660_e32381 * assign29660_e32383);
        let assign29660_e32385: f64 = (assign29660_e32384).exp();
        let assign29660_e32386: f64 = (locals.var_qq * assign29660_e32385);
        (assign29660_e32386, ((locals.var_qq_dn4 * assign29660_e32385) + (locals.var_qq * (assign29660_e32385 * (assign29660_e32381 * (locals.var_e1__blk911_dn4 / locals.var_e1__blk911))))), ((locals.var_qq_dn6 * assign29660_e32385) + (locals.var_qq * (assign29660_e32385 * (assign29660_e32381 * (locals.var_e1__blk911_dn6 / locals.var_e1__blk911))))), ((locals.var_qq_dn7 * assign29660_e32385) + (locals.var_qq * (assign29660_e32385 * (assign29660_e32381 * (locals.var_e1__blk911_dn7 / locals.var_e1__blk911))))), ((locals.var_qq_dn8 * assign29660_e32385) + (locals.var_qq * (assign29660_e32385 * (assign29660_e32381 * (locals.var_e1__blk911_dn8 / locals.var_e1__blk911))))), ((locals.var_qq_dn9 * assign29660_e32385) + (locals.var_qq * (assign29660_e32385 * (assign29660_e32381 * (locals.var_e1__blk911_dn9 / locals.var_e1__blk911))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign29660_e32388;
        locals.var_temp1_dn4 = assign29660_e32388_d_n4;
        locals.var_temp1_dn6 = assign29660_e32388_d_n6;
        locals.var_temp1_dn7 = assign29660_e32388_d_n7;
        locals.var_temp1_dn8 = assign29660_e32388_d_n8;
        locals.var_temp1_dn9 = assign29660_e32388_d_n9;

        let (assign29670_e32401, assign29670_e32401_d_n4, assign29670_e32401_d_n6, assign29670_e32401_d_n7, assign29670_e32401_d_n8, assign29670_e32401_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29670_e32394: f64 = (-0.3333333333333);
        let assign29670_e32396: f64 = (locals.var_e2__blk912).ln();
        let assign29670_e32397: f64 = (assign29670_e32394 * assign29670_e32396);
        let assign29670_e32398: f64 = (assign29670_e32397).exp();
        let assign29670_e32399: f64 = (locals.var_qq * assign29670_e32398);
        (assign29670_e32399, ((locals.var_qq_dn4 * assign29670_e32398) + (locals.var_qq * (assign29670_e32398 * (assign29670_e32394 * (locals.var_e2__blk912_dn4 / locals.var_e2__blk912))))), ((locals.var_qq_dn6 * assign29670_e32398) + (locals.var_qq * (assign29670_e32398 * (assign29670_e32394 * (locals.var_e2__blk912_dn6 / locals.var_e2__blk912))))), ((locals.var_qq_dn7 * assign29670_e32398) + (locals.var_qq * (assign29670_e32398 * (assign29670_e32394 * (locals.var_e2__blk912_dn7 / locals.var_e2__blk912))))), ((locals.var_qq_dn8 * assign29670_e32398) + (locals.var_qq * (assign29670_e32398 * (assign29670_e32394 * (locals.var_e2__blk912_dn8 / locals.var_e2__blk912))))), ((locals.var_qq_dn9 * assign29670_e32398) + (locals.var_qq * (assign29670_e32398 * (assign29670_e32394 * (locals.var_e2__blk912_dn9 / locals.var_e2__blk912))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign29670_e32401;
        locals.var_temp2_dn4 = assign29670_e32401_d_n4;
        locals.var_temp2_dn6 = assign29670_e32401_d_n6;
        locals.var_temp2_dn7 = assign29670_e32401_d_n7;
        locals.var_temp2_dn8 = assign29670_e32401_d_n8;
        locals.var_temp2_dn9 = assign29670_e32401_d_n9;

        let (assign29680_e32411, assign29680_e32411_d_n4, assign29680_e32411_d_n6, assign29680_e32411_d_n7, assign29680_e32411_d_n8, assign29680_e32411_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29680_e32407: f64 = (1.0 - locals.var_temp1);
        let assign29680_e32409: f64 = (assign29680_e32407 - locals.var_temp2);
        (assign29680_e32409, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign29680_e32411;
        locals.var_temp3_dn4 = assign29680_e32411_d_n4;
        locals.var_temp3_dn6 = assign29680_e32411_d_n6;
        locals.var_temp3_dn7 = assign29680_e32411_d_n7;
        locals.var_temp3_dn8 = assign29680_e32411_d_n8;
        locals.var_temp3_dn9 = assign29680_e32411_d_n9;

        let (assign29690_e32419, assign29690_e32419_d_n4, assign29690_e32419_d_n6, assign29690_e32419_d_n7, assign29690_e32419_d_n8, assign29690_e32419_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29690_e32417: f64 = (locals.var_csiprime_0 / locals.var_temp3);
        (assign29690_e32417, (-((locals.var_csiprime_0 * locals.var_temp3_dn4) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn6) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn7) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn8) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn9) / (locals.var_temp3 * locals.var_temp3))),)
    } else {
        (locals.var_csiprime__blk919, locals.var_csiprime__blk919_dn4, locals.var_csiprime__blk919_dn6, locals.var_csiprime__blk919_dn7, locals.var_csiprime__blk919_dn8, locals.var_csiprime__blk919_dn9,)
    }
};
        locals.var_csiprime__blk919 = assign29690_e32419;
        locals.var_csiprime__blk919_dn4 = assign29690_e32419_d_n4;
        locals.var_csiprime__blk919_dn6 = assign29690_e32419_d_n6;
        locals.var_csiprime__blk919_dn7 = assign29690_e32419_d_n7;
        locals.var_csiprime__blk919_dn8 = assign29690_e32419_d_n8;
        locals.var_csiprime__blk919_dn9 = assign29690_e32419_d_n9;

        let (assign29700_e32429, assign29700_e32429_d_n4, assign29700_e32429_d_n6, assign29700_e32429_d_n7, assign29700_e32429_d_n8, assign29700_e32429_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29700_e32426: f64 = (locals.var_k1_1d * locals.var_temp1);
        let assign29700_e32427: f64 = (1.0 + assign29700_e32426);
        (assign29700_e32427, (locals.var_k1_1d * locals.var_temp1_dn4), (locals.var_k1_1d * locals.var_temp1_dn6), (locals.var_k1_1d * locals.var_temp1_dn7), (locals.var_k1_1d * locals.var_temp1_dn8), (locals.var_k1_1d * locals.var_temp1_dn9),)
    } else {
        (locals.var_tox1fact__blk913, locals.var_tox1fact__blk913_dn4, locals.var_tox1fact__blk913_dn6, locals.var_tox1fact__blk913_dn7, locals.var_tox1fact__blk913_dn8, locals.var_tox1fact__blk913_dn9,)
    }
};
        locals.var_tox1fact__blk913 = assign29700_e32429;
        locals.var_tox1fact__blk913_dn4 = assign29700_e32429_d_n4;
        locals.var_tox1fact__blk913_dn6 = assign29700_e32429_d_n6;
        locals.var_tox1fact__blk913_dn7 = assign29700_e32429_d_n7;
        locals.var_tox1fact__blk913_dn8 = assign29700_e32429_d_n8;
        locals.var_tox1fact__blk913_dn9 = assign29700_e32429_d_n9;

        let (assign29710_e32439, assign29710_e32439_d_n4, assign29710_e32439_d_n6, assign29710_e32439_d_n7, assign29710_e32439_d_n8, assign29710_e32439_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29710_e32436: f64 = (locals.var_k2_1d * locals.var_temp2);
        let assign29710_e32437: f64 = (1.0 + assign29710_e32436);
        (assign29710_e32437, (locals.var_k2_1d * locals.var_temp2_dn4), (locals.var_k2_1d * locals.var_temp2_dn6), (locals.var_k2_1d * locals.var_temp2_dn7), (locals.var_k2_1d * locals.var_temp2_dn8), (locals.var_k2_1d * locals.var_temp2_dn9),)
    } else {
        (locals.var_tox2fact__blk914, locals.var_tox2fact__blk914_dn4, locals.var_tox2fact__blk914_dn6, locals.var_tox2fact__blk914_dn7, locals.var_tox2fact__blk914_dn8, locals.var_tox2fact__blk914_dn9,)
    }
};
        locals.var_tox2fact__blk914 = assign29710_e32439;
        locals.var_tox2fact__blk914_dn4 = assign29710_e32439_d_n4;
        locals.var_tox2fact__blk914_dn6 = assign29710_e32439_d_n6;
        locals.var_tox2fact__blk914_dn7 = assign29710_e32439_d_n7;
        locals.var_tox2fact__blk914_dn8 = assign29710_e32439_d_n8;
        locals.var_tox2fact__blk914_dn9 = assign29710_e32439_d_n9;

    }

    pub(super) fn stamp_transient_block_76(
        locals: &mut StampLocals,
    ) {
        let (assign29720_e32449, assign29720_e32449_d_n4, assign29720_e32449_d_n6, assign29720_e32449_d_n7, assign29720_e32449_d_n8, assign29720_e32449_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29720_e32445: f64 = (locals.var_k1_1d * locals.var_temp3);
        let assign29720_e32447: f64 = (assign29720_e32445 / locals.var_tox1fact__blk913);
        (assign29720_e32447, ((((locals.var_k1_1d * locals.var_temp3_dn4) * locals.var_tox1fact__blk913) - (assign29720_e32445 * locals.var_tox1fact__blk913_dn4)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * locals.var_tox1fact__blk913) - (assign29720_e32445 * locals.var_tox1fact__blk913_dn6)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * locals.var_tox1fact__blk913) - (assign29720_e32445 * locals.var_tox1fact__blk913_dn7)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * locals.var_tox1fact__blk913) - (assign29720_e32445 * locals.var_tox1fact__blk913_dn8)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * locals.var_tox1fact__blk913) - (assign29720_e32445 * locals.var_tox1fact__blk913_dn9)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)),)
    } else {
        (locals.var_k1_1d_qm__blk915, locals.var_k1_1d_qm__blk915_dn4, locals.var_k1_1d_qm__blk915_dn6, locals.var_k1_1d_qm__blk915_dn7, locals.var_k1_1d_qm__blk915_dn8, locals.var_k1_1d_qm__blk915_dn9,)
    }
};
        locals.var_k1_1d_qm__blk915 = assign29720_e32449;
        locals.var_k1_1d_qm__blk915_dn4 = assign29720_e32449_d_n4;
        locals.var_k1_1d_qm__blk915_dn6 = assign29720_e32449_d_n6;
        locals.var_k1_1d_qm__blk915_dn7 = assign29720_e32449_d_n7;
        locals.var_k1_1d_qm__blk915_dn8 = assign29720_e32449_d_n8;
        locals.var_k1_1d_qm__blk915_dn9 = assign29720_e32449_d_n9;

        let (assign29730_e32459, assign29730_e32459_d_n4, assign29730_e32459_d_n6, assign29730_e32459_d_n7, assign29730_e32459_d_n8, assign29730_e32459_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29730_e32455: f64 = (locals.var_k2_1d * locals.var_temp3);
        let assign29730_e32457: f64 = (assign29730_e32455 / locals.var_tox2fact__blk914);
        (assign29730_e32457, ((((locals.var_k2_1d * locals.var_temp3_dn4) * locals.var_tox2fact__blk914) - (assign29730_e32455 * locals.var_tox2fact__blk914_dn4)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * locals.var_tox2fact__blk914) - (assign29730_e32455 * locals.var_tox2fact__blk914_dn6)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * locals.var_tox2fact__blk914) - (assign29730_e32455 * locals.var_tox2fact__blk914_dn7)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * locals.var_tox2fact__blk914) - (assign29730_e32455 * locals.var_tox2fact__blk914_dn8)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * locals.var_tox2fact__blk914) - (assign29730_e32455 * locals.var_tox2fact__blk914_dn9)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)),)
    } else {
        (locals.var_k2_1d_qm__blk916, locals.var_k2_1d_qm__blk916_dn4, locals.var_k2_1d_qm__blk916_dn6, locals.var_k2_1d_qm__blk916_dn7, locals.var_k2_1d_qm__blk916_dn8, locals.var_k2_1d_qm__blk916_dn9,)
    }
};
        locals.var_k2_1d_qm__blk916 = assign29730_e32459;
        locals.var_k2_1d_qm__blk916_dn4 = assign29730_e32459_d_n4;
        locals.var_k2_1d_qm__blk916_dn6 = assign29730_e32459_d_n6;
        locals.var_k2_1d_qm__blk916_dn7 = assign29730_e32459_d_n7;
        locals.var_k2_1d_qm__blk916_dn8 = assign29730_e32459_d_n8;
        locals.var_k2_1d_qm__blk916_dn9 = assign29730_e32459_d_n9;

        let (assign29740_e32475, assign29740_e32475_d_n4, assign29740_e32475_d_n6, assign29740_e32475_d_n7, assign29740_e32475_d_n8, assign29740_e32475_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29740_e32467: f64 = (1.0 / locals.var_k1_1d_qm__blk915);
        let assign29740_e32468: f64 = (1.0 + assign29740_e32467);
        let assign29740_e32471: f64 = (1.0 / locals.var_k2_1d_qm__blk916);
        let assign29740_e32472: f64 = (assign29740_e32468 + assign29740_e32471);
        let assign29740_e32473: f64 = (1.0 / assign29740_e32472);
        (assign29740_e32473, (-(((-(locals.var_k1_1d_qm__blk915_dn4 / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + (-(locals.var_k2_1d_qm__blk916_dn4 / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916)))) / (assign29740_e32472 * assign29740_e32472))), (-(((-(locals.var_k1_1d_qm__blk915_dn6 / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + (-(locals.var_k2_1d_qm__blk916_dn6 / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916)))) / (assign29740_e32472 * assign29740_e32472))), (-(((-(locals.var_k1_1d_qm__blk915_dn7 / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + (-(locals.var_k2_1d_qm__blk916_dn7 / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916)))) / (assign29740_e32472 * assign29740_e32472))), (-(((-(locals.var_k1_1d_qm__blk915_dn8 / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + (-(locals.var_k2_1d_qm__blk916_dn8 / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916)))) / (assign29740_e32472 * assign29740_e32472))), (-(((-(locals.var_k1_1d_qm__blk915_dn9 / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + (-(locals.var_k2_1d_qm__blk916_dn9 / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916)))) / (assign29740_e32472 * assign29740_e32472))),)
    } else {
        (locals.var_keq_1d_qm__blk917, locals.var_keq_1d_qm__blk917_dn4, locals.var_keq_1d_qm__blk917_dn6, locals.var_keq_1d_qm__blk917_dn7, locals.var_keq_1d_qm__blk917_dn8, locals.var_keq_1d_qm__blk917_dn9,)
    }
};
        locals.var_keq_1d_qm__blk917 = assign29740_e32475;
        locals.var_keq_1d_qm__blk917_dn4 = assign29740_e32475_d_n4;
        locals.var_keq_1d_qm__blk917_dn6 = assign29740_e32475_d_n6;
        locals.var_keq_1d_qm__blk917_dn7 = assign29740_e32475_d_n7;
        locals.var_keq_1d_qm__blk917_dn8 = assign29740_e32475_d_n8;
        locals.var_keq_1d_qm__blk917_dn9 = assign29740_e32475_d_n9;

        let (assign29750_e32485, assign29750_e32485_d_n4, assign29750_e32485_d_n6, assign29750_e32485_d_n7, assign29750_e32485_d_n8, assign29750_e32485_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29750_e32482: f64 = (locals.var_k1_1d_qm__blk915 * locals.var_temp1);
        let assign29750_e32483: f64 = (1.0 + assign29750_e32482);
        (assign29750_e32483, ((locals.var_k1_1d_qm__blk915_dn4 * locals.var_temp1) + (locals.var_k1_1d_qm__blk915 * locals.var_temp1_dn4)), ((locals.var_k1_1d_qm__blk915_dn6 * locals.var_temp1) + (locals.var_k1_1d_qm__blk915 * locals.var_temp1_dn6)), ((locals.var_k1_1d_qm__blk915_dn7 * locals.var_temp1) + (locals.var_k1_1d_qm__blk915 * locals.var_temp1_dn7)), ((locals.var_k1_1d_qm__blk915_dn8 * locals.var_temp1) + (locals.var_k1_1d_qm__blk915 * locals.var_temp1_dn8)), ((locals.var_k1_1d_qm__blk915_dn9 * locals.var_temp1) + (locals.var_k1_1d_qm__blk915 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_tox1fact__blk913, locals.var_tox1fact__blk913_dn4, locals.var_tox1fact__blk913_dn6, locals.var_tox1fact__blk913_dn7, locals.var_tox1fact__blk913_dn8, locals.var_tox1fact__blk913_dn9,)
    }
};
        locals.var_tox1fact__blk913 = assign29750_e32485;
        locals.var_tox1fact__blk913_dn4 = assign29750_e32485_d_n4;
        locals.var_tox1fact__blk913_dn6 = assign29750_e32485_d_n6;
        locals.var_tox1fact__blk913_dn7 = assign29750_e32485_d_n7;
        locals.var_tox1fact__blk913_dn8 = assign29750_e32485_d_n8;
        locals.var_tox1fact__blk913_dn9 = assign29750_e32485_d_n9;

        let (assign29760_e32495, assign29760_e32495_d_n4, assign29760_e32495_d_n6, assign29760_e32495_d_n7, assign29760_e32495_d_n8, assign29760_e32495_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29760_e32492: f64 = (locals.var_k2_1d_qm__blk916 * locals.var_temp2);
        let assign29760_e32493: f64 = (1.0 + assign29760_e32492);
        (assign29760_e32493, ((locals.var_k2_1d_qm__blk916_dn4 * locals.var_temp2) + (locals.var_k2_1d_qm__blk916 * locals.var_temp2_dn4)), ((locals.var_k2_1d_qm__blk916_dn6 * locals.var_temp2) + (locals.var_k2_1d_qm__blk916 * locals.var_temp2_dn6)), ((locals.var_k2_1d_qm__blk916_dn7 * locals.var_temp2) + (locals.var_k2_1d_qm__blk916 * locals.var_temp2_dn7)), ((locals.var_k2_1d_qm__blk916_dn8 * locals.var_temp2) + (locals.var_k2_1d_qm__blk916 * locals.var_temp2_dn8)), ((locals.var_k2_1d_qm__blk916_dn9 * locals.var_temp2) + (locals.var_k2_1d_qm__blk916 * locals.var_temp2_dn9)),)
    } else {
        (locals.var_tox2fact__blk914, locals.var_tox2fact__blk914_dn4, locals.var_tox2fact__blk914_dn6, locals.var_tox2fact__blk914_dn7, locals.var_tox2fact__blk914_dn8, locals.var_tox2fact__blk914_dn9,)
    }
};
        locals.var_tox2fact__blk914 = assign29760_e32495;
        locals.var_tox2fact__blk914_dn4 = assign29760_e32495_d_n4;
        locals.var_tox2fact__blk914_dn6 = assign29760_e32495_d_n6;
        locals.var_tox2fact__blk914_dn7 = assign29760_e32495_d_n7;
        locals.var_tox2fact__blk914_dn8 = assign29760_e32495_d_n8;
        locals.var_tox2fact__blk914_dn9 = assign29760_e32495_d_n9;

        let (assign29770_e32502, assign29770_e32502_d_n4, assign29770_e32502_d_n6, assign29770_e32502_d_n7, assign29770_e32502_d_n8, assign29770_e32502_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (locals.var_csiprime_0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_csiprime__blk919, locals.var_csiprime__blk919_dn4, locals.var_csiprime__blk919_dn6, locals.var_csiprime__blk919_dn7, locals.var_csiprime__blk919_dn8, locals.var_csiprime__blk919_dn9,)
    }
};
        locals.var_csiprime__blk919 = assign29770_e32502;
        locals.var_csiprime__blk919_dn4 = assign29770_e32502_d_n4;
        locals.var_csiprime__blk919_dn6 = assign29770_e32502_d_n6;
        locals.var_csiprime__blk919_dn7 = assign29770_e32502_d_n7;
        locals.var_csiprime__blk919_dn8 = assign29770_e32502_d_n8;
        locals.var_csiprime__blk919_dn9 = assign29770_e32502_d_n9;

        let (assign29780_e32509, assign29780_e32509_d_n4, assign29780_e32509_d_n6, assign29780_e32509_d_n7, assign29780_e32509_d_n8, assign29780_e32509_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k1_1d_qm__blk915, locals.var_k1_1d_qm__blk915_dn4, locals.var_k1_1d_qm__blk915_dn6, locals.var_k1_1d_qm__blk915_dn7, locals.var_k1_1d_qm__blk915_dn8, locals.var_k1_1d_qm__blk915_dn9,)
    }
};
        locals.var_k1_1d_qm__blk915 = assign29780_e32509;
        locals.var_k1_1d_qm__blk915_dn4 = assign29780_e32509_d_n4;
        locals.var_k1_1d_qm__blk915_dn6 = assign29780_e32509_d_n6;
        locals.var_k1_1d_qm__blk915_dn7 = assign29780_e32509_d_n7;
        locals.var_k1_1d_qm__blk915_dn8 = assign29780_e32509_d_n8;
        locals.var_k1_1d_qm__blk915_dn9 = assign29780_e32509_d_n9;

        let (assign29790_e32516, assign29790_e32516_d_n4, assign29790_e32516_d_n6, assign29790_e32516_d_n7, assign29790_e32516_d_n8, assign29790_e32516_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k2_1d_qm__blk916, locals.var_k2_1d_qm__blk916_dn4, locals.var_k2_1d_qm__blk916_dn6, locals.var_k2_1d_qm__blk916_dn7, locals.var_k2_1d_qm__blk916_dn8, locals.var_k2_1d_qm__blk916_dn9,)
    }
};
        locals.var_k2_1d_qm__blk916 = assign29790_e32516;
        locals.var_k2_1d_qm__blk916_dn4 = assign29790_e32516_d_n4;
        locals.var_k2_1d_qm__blk916_dn6 = assign29790_e32516_d_n6;
        locals.var_k2_1d_qm__blk916_dn7 = assign29790_e32516_d_n7;
        locals.var_k2_1d_qm__blk916_dn8 = assign29790_e32516_d_n8;
        locals.var_k2_1d_qm__blk916_dn9 = assign29790_e32516_d_n9;

        let (assign29800_e32523, assign29800_e32523_d_n4, assign29800_e32523_d_n6, assign29800_e32523_d_n7, assign29800_e32523_d_n8, assign29800_e32523_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_keq_1d_qm__blk917, locals.var_keq_1d_qm__blk917_dn4, locals.var_keq_1d_qm__blk917_dn6, locals.var_keq_1d_qm__blk917_dn7, locals.var_keq_1d_qm__blk917_dn8, locals.var_keq_1d_qm__blk917_dn9,)
    }
};
        locals.var_keq_1d_qm__blk917 = assign29800_e32523;
        locals.var_keq_1d_qm__blk917_dn4 = assign29800_e32523_d_n4;
        locals.var_keq_1d_qm__blk917_dn6 = assign29800_e32523_d_n6;
        locals.var_keq_1d_qm__blk917_dn7 = assign29800_e32523_d_n7;
        locals.var_keq_1d_qm__blk917_dn8 = assign29800_e32523_d_n8;
        locals.var_keq_1d_qm__blk917_dn9 = assign29800_e32523_d_n9;

        let (assign29810_e32530, assign29810_e32530_d_n4, assign29810_e32530_d_n6, assign29810_e32530_d_n7, assign29810_e32530_d_n8, assign29810_e32530_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tox1fact__blk913, locals.var_tox1fact__blk913_dn4, locals.var_tox1fact__blk913_dn6, locals.var_tox1fact__blk913_dn7, locals.var_tox1fact__blk913_dn8, locals.var_tox1fact__blk913_dn9,)
    }
};
        locals.var_tox1fact__blk913 = assign29810_e32530;
        locals.var_tox1fact__blk913_dn4 = assign29810_e32530_d_n4;
        locals.var_tox1fact__blk913_dn6 = assign29810_e32530_d_n6;
        locals.var_tox1fact__blk913_dn7 = assign29810_e32530_d_n7;
        locals.var_tox1fact__blk913_dn8 = assign29810_e32530_d_n8;
        locals.var_tox1fact__blk913_dn9 = assign29810_e32530_d_n9;

        let (assign29820_e32537, assign29820_e32537_d_n4, assign29820_e32537_d_n6, assign29820_e32537_d_n7, assign29820_e32537_d_n8, assign29820_e32537_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tox2fact__blk914, locals.var_tox2fact__blk914_dn4, locals.var_tox2fact__blk914_dn6, locals.var_tox2fact__blk914_dn7, locals.var_tox2fact__blk914_dn8, locals.var_tox2fact__blk914_dn9,)
    }
};
        locals.var_tox2fact__blk914 = assign29820_e32537;
        locals.var_tox2fact__blk914_dn4 = assign29820_e32537_d_n4;
        locals.var_tox2fact__blk914_dn6 = assign29820_e32537_d_n6;
        locals.var_tox2fact__blk914_dn7 = assign29820_e32537_d_n7;
        locals.var_tox2fact__blk914_dn8 = assign29820_e32537_d_n8;
        locals.var_tox2fact__blk914_dn9 = assign29820_e32537_d_n9;

        let (assign29830_e32545, assign29830_e32545_d_n4, assign29830_e32545_d_n6, assign29830_e32545_d_n7, assign29830_e32545_d_n8, assign29830_e32545_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign29830_e32542: f64 = (locals.var_xg10__blk899 - locals.var_xg2eff__blk910);
        let assign29830_e32543: f64 = (locals.var_keq_1d_qm__blk917 * assign29830_e32542);
        (assign29830_e32543, ((locals.var_keq_1d_qm__blk917_dn4 * assign29830_e32542) + (locals.var_keq_1d_qm__blk917 * (locals.var_xg10__blk899_dn4 - locals.var_xg2eff__blk910_dn4))), ((locals.var_keq_1d_qm__blk917_dn6 * assign29830_e32542) + (locals.var_keq_1d_qm__blk917 * (locals.var_xg10__blk899_dn6 - locals.var_xg2eff__blk910_dn6))), ((locals.var_keq_1d_qm__blk917_dn7 * assign29830_e32542) + (locals.var_keq_1d_qm__blk917 * (locals.var_xg10__blk899_dn7 - locals.var_xg2eff__blk910_dn7))), ((locals.var_keq_1d_qm__blk917_dn8 * assign29830_e32542) + (locals.var_keq_1d_qm__blk917 * (locals.var_xg10__blk899_dn8 - locals.var_xg2eff__blk910_dn8))), ((locals.var_keq_1d_qm__blk917_dn9 * assign29830_e32542) + (locals.var_keq_1d_qm__blk917 * (locals.var_xg10__blk899_dn9 - locals.var_xg2eff__blk910_dn9))),)
    } else {
        (locals.var_dx_wi_1d__blk918, locals.var_dx_wi_1d__blk918_dn4, locals.var_dx_wi_1d__blk918_dn6, locals.var_dx_wi_1d__blk918_dn7, locals.var_dx_wi_1d__blk918_dn8, locals.var_dx_wi_1d__blk918_dn9,)
    }
};
        locals.var_dx_wi_1d__blk918 = assign29830_e32545;
        locals.var_dx_wi_1d__blk918_dn4 = assign29830_e32545_d_n4;
        locals.var_dx_wi_1d__blk918_dn6 = assign29830_e32545_d_n6;
        locals.var_dx_wi_1d__blk918_dn7 = assign29830_e32545_d_n7;
        locals.var_dx_wi_1d__blk918_dn8 = assign29830_e32545_d_n8;
        locals.var_dx_wi_1d__blk918_dn9 = assign29830_e32545_d_n9;

        let assign29840_e32548: f64 = if locals.var_dx_wi_1d__blk918 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1094 = assign29840_e32548;

        let assign29850_e32550: f64 = (-locals.var_dx_wi_1d__blk918);
        let assign29850_e32552: f64 = if assign29850_e32550 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1095 = assign29850_e32552;

        let (assign29860_e32565, assign29860_e32565_d_n4, assign29860_e32565_d_n6, assign29860_e32565_d_n7, assign29860_e32565_d_n8, assign29860_e32565_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1094 != 0.0)) && (locals.var_guard1095 != 0.0)) {
        let assign29860_e32560: f64 = (-locals.var_dx_wi_1d__blk918);
        let assign29860_e32561: f64 = (assign29860_e32560).exp();
        let assign29860_e32562: f64 = (1.0 + assign29860_e32561);
        let assign29860_e32563: f64 = (assign29860_e32562).ln();
        (assign29860_e32563, ((assign29860_e32561 * (-locals.var_dx_wi_1d__blk918_dn4)) / assign29860_e32562), ((assign29860_e32561 * (-locals.var_dx_wi_1d__blk918_dn6)) / assign29860_e32562), ((assign29860_e32561 * (-locals.var_dx_wi_1d__blk918_dn7)) / assign29860_e32562), ((assign29860_e32561 * (-locals.var_dx_wi_1d__blk918_dn8)) / assign29860_e32562), ((assign29860_e32561 * (-locals.var_dx_wi_1d__blk918_dn9)) / assign29860_e32562),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign29860_e32565;
        locals.var_temp_dn4 = assign29860_e32565_d_n4;
        locals.var_temp_dn6 = assign29860_e32565_d_n6;
        locals.var_temp_dn7 = assign29860_e32565_d_n7;
        locals.var_temp_dn8 = assign29860_e32565_d_n8;
        locals.var_temp_dn9 = assign29860_e32565_d_n9;

        let (assign29870_e32575, assign29870_e32575_d_n4, assign29870_e32575_d_n6, assign29870_e32575_d_n7, assign29870_e32575_d_n8, assign29870_e32575_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1094 != 0.0)) && (locals.var_guard1095 == 0.0)) {
        let assign29870_e32573: f64 = (-locals.var_dx_wi_1d__blk918);
        (assign29870_e32573, (-locals.var_dx_wi_1d__blk918_dn4), (-locals.var_dx_wi_1d__blk918_dn6), (-locals.var_dx_wi_1d__blk918_dn7), (-locals.var_dx_wi_1d__blk918_dn8), (-locals.var_dx_wi_1d__blk918_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign29870_e32575;
        locals.var_temp_dn4 = assign29870_e32575_d_n4;
        locals.var_temp_dn6 = assign29870_e32575_d_n6;
        locals.var_temp_dn7 = assign29870_e32575_d_n7;
        locals.var_temp_dn8 = assign29870_e32575_d_n8;
        locals.var_temp_dn9 = assign29870_e32575_d_n9;

        let (assign29880_e32589, assign29880_e32589_d_n4, assign29880_e32589_d_n6, assign29880_e32589_d_n7, assign29880_e32589_d_n8, assign29880_e32589_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1094 != 0.0)) {
        let assign29880_e32582: f64 = (locals.var_dx_wi_1d__blk918 / locals.var_k1_1d_qm__blk915);
        let assign29880_e32583: f64 = (locals.var_xg10__blk899 - assign29880_e32582);
        let assign29880_e32585: f64 = (assign29880_e32583 + locals.var_temp);
        let assign29880_e32587: f64 = (assign29880_e32585 - 0.6931471805599);
        (assign29880_e32587, ((locals.var_xg10__blk899_dn4 - (((locals.var_dx_wi_1d__blk918_dn4 * locals.var_k1_1d_qm__blk915) - (locals.var_dx_wi_1d__blk918 * locals.var_k1_1d_qm__blk915_dn4)) / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + locals.var_temp_dn4), ((locals.var_xg10__blk899_dn6 - (((locals.var_dx_wi_1d__blk918_dn6 * locals.var_k1_1d_qm__blk915) - (locals.var_dx_wi_1d__blk918 * locals.var_k1_1d_qm__blk915_dn6)) / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + locals.var_temp_dn6), ((locals.var_xg10__blk899_dn7 - (((locals.var_dx_wi_1d__blk918_dn7 * locals.var_k1_1d_qm__blk915) - (locals.var_dx_wi_1d__blk918 * locals.var_k1_1d_qm__blk915_dn7)) / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + locals.var_temp_dn7), ((locals.var_xg10__blk899_dn8 - (((locals.var_dx_wi_1d__blk918_dn8 * locals.var_k1_1d_qm__blk915) - (locals.var_dx_wi_1d__blk918 * locals.var_k1_1d_qm__blk915_dn8)) / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + locals.var_temp_dn8), ((locals.var_xg10__blk899_dn9 - (((locals.var_dx_wi_1d__blk918_dn9 * locals.var_k1_1d_qm__blk915) - (locals.var_dx_wi_1d__blk918 * locals.var_k1_1d_qm__blk915_dn9)) / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d__blk920, locals.var_x_wi_1d__blk920_dn4, locals.var_x_wi_1d__blk920_dn6, locals.var_x_wi_1d__blk920_dn7, locals.var_x_wi_1d__blk920_dn8, locals.var_x_wi_1d__blk920_dn9,)
    }
};
        locals.var_x_wi_1d__blk920 = assign29880_e32589;
        locals.var_x_wi_1d__blk920_dn4 = assign29880_e32589_d_n4;
        locals.var_x_wi_1d__blk920_dn6 = assign29880_e32589_d_n6;
        locals.var_x_wi_1d__blk920_dn7 = assign29880_e32589_d_n7;
        locals.var_x_wi_1d__blk920_dn8 = assign29880_e32589_d_n8;
        locals.var_x_wi_1d__blk920_dn9 = assign29880_e32589_d_n9;

        let assign29890_e32592: f64 = if locals.var_dx_wi_1d__blk918 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1096 = assign29890_e32592;

        let (assign29900_e32605, assign29900_e32605_d_n4, assign29900_e32605_d_n6, assign29900_e32605_d_n7, assign29900_e32605_d_n8, assign29900_e32605_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1094 == 0.0)) && (locals.var_guard1096 != 0.0)) {
        let assign29900_e32601: f64 = (locals.var_dx_wi_1d__blk918).exp();
        let assign29900_e32602: f64 = (1.0 + assign29900_e32601);
        let assign29900_e32603: f64 = (assign29900_e32602).ln();
        (assign29900_e32603, ((assign29900_e32601 * locals.var_dx_wi_1d__blk918_dn4) / assign29900_e32602), ((assign29900_e32601 * locals.var_dx_wi_1d__blk918_dn6) / assign29900_e32602), ((assign29900_e32601 * locals.var_dx_wi_1d__blk918_dn7) / assign29900_e32602), ((assign29900_e32601 * locals.var_dx_wi_1d__blk918_dn8) / assign29900_e32602), ((assign29900_e32601 * locals.var_dx_wi_1d__blk918_dn9) / assign29900_e32602),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign29900_e32605;
        locals.var_temp_dn4 = assign29900_e32605_d_n4;
        locals.var_temp_dn6 = assign29900_e32605_d_n6;
        locals.var_temp_dn7 = assign29900_e32605_d_n7;
        locals.var_temp_dn8 = assign29900_e32605_d_n8;
        locals.var_temp_dn9 = assign29900_e32605_d_n9;

        let (assign29910_e32615, assign29910_e32615_d_n4, assign29910_e32615_d_n6, assign29910_e32615_d_n7, assign29910_e32615_d_n8, assign29910_e32615_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1094 == 0.0)) && (locals.var_guard1096 == 0.0)) {
        (locals.var_dx_wi_1d__blk918, locals.var_dx_wi_1d__blk918_dn4, locals.var_dx_wi_1d__blk918_dn6, locals.var_dx_wi_1d__blk918_dn7, locals.var_dx_wi_1d__blk918_dn8, locals.var_dx_wi_1d__blk918_dn9,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign29910_e32615;
        locals.var_temp_dn4 = assign29910_e32615_d_n4;
        locals.var_temp_dn6 = assign29910_e32615_d_n6;
        locals.var_temp_dn7 = assign29910_e32615_d_n7;
        locals.var_temp_dn8 = assign29910_e32615_d_n8;
        locals.var_temp_dn9 = assign29910_e32615_d_n9;

        let (assign29920_e32630, assign29920_e32630_d_n4, assign29920_e32630_d_n6, assign29920_e32630_d_n7, assign29920_e32630_d_n8, assign29920_e32630_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1094 == 0.0)) {
        let assign29920_e32623: f64 = (locals.var_dx_wi_1d__blk918 / locals.var_k2_1d_qm__blk916);
        let assign29920_e32624: f64 = (locals.var_xg2eff__blk910 + assign29920_e32623);
        let assign29920_e32626: f64 = (assign29920_e32624 + locals.var_temp);
        let assign29920_e32628: f64 = (assign29920_e32626 - 0.6931471805599);
        (assign29920_e32628, ((locals.var_xg2eff__blk910_dn4 + (((locals.var_dx_wi_1d__blk918_dn4 * locals.var_k2_1d_qm__blk916) - (locals.var_dx_wi_1d__blk918 * locals.var_k2_1d_qm__blk916_dn4)) / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916))) + locals.var_temp_dn4), ((locals.var_xg2eff__blk910_dn6 + (((locals.var_dx_wi_1d__blk918_dn6 * locals.var_k2_1d_qm__blk916) - (locals.var_dx_wi_1d__blk918 * locals.var_k2_1d_qm__blk916_dn6)) / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916))) + locals.var_temp_dn6), ((locals.var_xg2eff__blk910_dn7 + (((locals.var_dx_wi_1d__blk918_dn7 * locals.var_k2_1d_qm__blk916) - (locals.var_dx_wi_1d__blk918 * locals.var_k2_1d_qm__blk916_dn7)) / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916))) + locals.var_temp_dn7), ((locals.var_xg2eff__blk910_dn8 + (((locals.var_dx_wi_1d__blk918_dn8 * locals.var_k2_1d_qm__blk916) - (locals.var_dx_wi_1d__blk918 * locals.var_k2_1d_qm__blk916_dn8)) / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916))) + locals.var_temp_dn8), ((locals.var_xg2eff__blk910_dn9 + (((locals.var_dx_wi_1d__blk918_dn9 * locals.var_k2_1d_qm__blk916) - (locals.var_dx_wi_1d__blk918 * locals.var_k2_1d_qm__blk916_dn9)) / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d__blk920, locals.var_x_wi_1d__blk920_dn4, locals.var_x_wi_1d__blk920_dn6, locals.var_x_wi_1d__blk920_dn7, locals.var_x_wi_1d__blk920_dn8, locals.var_x_wi_1d__blk920_dn9,)
    }
};
        locals.var_x_wi_1d__blk920 = assign29920_e32630;
        locals.var_x_wi_1d__blk920_dn4 = assign29920_e32630_d_n4;
        locals.var_x_wi_1d__blk920_dn6 = assign29920_e32630_d_n6;
        locals.var_x_wi_1d__blk920_dn7 = assign29920_e32630_d_n7;
        locals.var_x_wi_1d__blk920_dn8 = assign29920_e32630_d_n8;
        locals.var_x_wi_1d__blk920_dn9 = assign29920_e32630_d_n9;

        let (assign29930_e32649, assign29930_e32649_d_n4, assign29930_e32649_d_n6, assign29930_e32649_d_n7, assign29930_e32649_d_n8, assign29930_e32649_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign29930_e32635: f64 = (locals.var_x_wi_1d__blk920 + locals.var_xth_1d);
        let assign29930_e32638: f64 = (locals.var_x_wi_1d__blk920 - locals.var_xth_1d);
        let assign29930_e32641: f64 = (locals.var_x_wi_1d__blk920 - locals.var_xth_1d);
        let assign29930_e32642: f64 = (assign29930_e32638 * assign29930_e32641);
        let assign29930_e32644: f64 = (assign29930_e32642 + 4.0);
        let assign29930_e32645: f64 = (assign29930_e32644).sqrt();
        let assign29930_e32646: f64 = (assign29930_e32635 - assign29930_e32645);
        let assign29930_e32647: f64 = (0.5 * assign29930_e32646);
        (assign29930_e32647, (0.5 * ((locals.var_x_wi_1d__blk920_dn4 + locals.var_xth_1d_dn4) - ((((locals.var_x_wi_1d__blk920_dn4 - locals.var_xth_1d_dn4) * assign29930_e32641) + (assign29930_e32638 * (locals.var_x_wi_1d__blk920_dn4 - locals.var_xth_1d_dn4))) / (2.0 * assign29930_e32645)))), (0.5 * ((locals.var_x_wi_1d__blk920_dn6 + locals.var_xth_1d_dn6) - ((((locals.var_x_wi_1d__blk920_dn6 - locals.var_xth_1d_dn6) * assign29930_e32641) + (assign29930_e32638 * (locals.var_x_wi_1d__blk920_dn6 - locals.var_xth_1d_dn6))) / (2.0 * assign29930_e32645)))), (0.5 * ((locals.var_x_wi_1d__blk920_dn7 + locals.var_xth_1d_dn7) - ((((locals.var_x_wi_1d__blk920_dn7 - locals.var_xth_1d_dn7) * assign29930_e32641) + (assign29930_e32638 * (locals.var_x_wi_1d__blk920_dn7 - locals.var_xth_1d_dn7))) / (2.0 * assign29930_e32645)))), (0.5 * ((locals.var_x_wi_1d__blk920_dn8 + locals.var_xth_1d_dn8) - ((((locals.var_x_wi_1d__blk920_dn8 - locals.var_xth_1d_dn8) * assign29930_e32641) + (assign29930_e32638 * (locals.var_x_wi_1d__blk920_dn8 - locals.var_xth_1d_dn8))) / (2.0 * assign29930_e32645)))), (0.5 * ((locals.var_x_wi_1d__blk920_dn9 + locals.var_xth_1d_dn9) - ((((locals.var_x_wi_1d__blk920_dn9 - locals.var_xth_1d_dn9) * assign29930_e32641) + (assign29930_e32638 * (locals.var_x_wi_1d__blk920_dn9 - locals.var_xth_1d_dn9))) / (2.0 * assign29930_e32645)))),)
    } else {
        (locals.var_x_1d__blk921, locals.var_x_1d__blk921_dn4, locals.var_x_1d__blk921_dn6, locals.var_x_1d__blk921_dn7, locals.var_x_1d__blk921_dn8, locals.var_x_1d__blk921_dn9,)
    }
};
        locals.var_x_1d__blk921 = assign29930_e32649;
        locals.var_x_1d__blk921_dn4 = assign29930_e32649_d_n4;
        locals.var_x_1d__blk921_dn6 = assign29930_e32649_d_n6;
        locals.var_x_1d__blk921_dn7 = assign29930_e32649_d_n7;
        locals.var_x_1d__blk921_dn8 = assign29930_e32649_d_n8;
        locals.var_x_1d__blk921_dn9 = assign29930_e32649_d_n9;

        let (assign29940_e32664, assign29940_e32664_d_n4, assign29940_e32664_d_n6, assign29940_e32664_d_n7, assign29940_e32664_d_n8, assign29940_e32664_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign29940_e32655: f64 = (locals.var_xth_1d - locals.var_x_1d__blk921);
        let assign29940_e32656: f64 = (2.0 * assign29940_e32655);
        let assign29940_e32658: f64 = (assign29940_e32656 / locals.var_xsddep);
        let assign29940_e32659: f64 = (1.0 + assign29940_e32658);
        let assign29940_e32660: f64 = (assign29940_e32659).sqrt();
        let assign29940_e32662: f64 = (assign29940_e32660 - 1.0);
        (assign29940_e32662, (((((2.0 * (locals.var_xth_1d_dn4 - locals.var_x_1d__blk921_dn4)) * locals.var_xsddep) - (assign29940_e32656 * locals.var_xsddep_dn4)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign29940_e32660)), (((((2.0 * (locals.var_xth_1d_dn6 - locals.var_x_1d__blk921_dn6)) * locals.var_xsddep) - (assign29940_e32656 * locals.var_xsddep_dn6)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign29940_e32660)), (((((2.0 * (locals.var_xth_1d_dn7 - locals.var_x_1d__blk921_dn7)) * locals.var_xsddep) - (assign29940_e32656 * locals.var_xsddep_dn7)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign29940_e32660)), (((((2.0 * (locals.var_xth_1d_dn8 - locals.var_x_1d__blk921_dn8)) * locals.var_xsddep) - (assign29940_e32656 * locals.var_xsddep_dn8)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign29940_e32660)), (((((2.0 * (locals.var_xth_1d_dn9 - locals.var_x_1d__blk921_dn9)) * locals.var_xsddep) - (assign29940_e32656 * locals.var_xsddep_dn9)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign29940_e32660)),)
    } else {
        (locals.var_dleff__blk922, locals.var_dleff__blk922_dn4, locals.var_dleff__blk922_dn6, locals.var_dleff__blk922_dn7, locals.var_dleff__blk922_dn8, locals.var_dleff__blk922_dn9,)
    }
};
        locals.var_dleff__blk922 = assign29940_e32664;
        locals.var_dleff__blk922_dn4 = assign29940_e32664_d_n4;
        locals.var_dleff__blk922_dn6 = assign29940_e32664_d_n6;
        locals.var_dleff__blk922_dn7 = assign29940_e32664_d_n7;
        locals.var_dleff__blk922_dn8 = assign29940_e32664_d_n8;
        locals.var_dleff__blk922_dn9 = assign29940_e32664_d_n9;

        let (assign29950_e32672, assign29950_e32672_d_n4, assign29950_e32672_d_n6, assign29950_e32672_d_n7, assign29950_e32672_d_n8, assign29950_e32672_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign29950_e32669: f64 = (locals.var_xsddep * locals.var_dleff__blk922);
        let assign29950_e32670: f64 = (locals.var_x_1d__blk921 + assign29950_e32669);
        (assign29950_e32670, (locals.var_x_1d__blk921_dn4 + ((locals.var_xsddep_dn4 * locals.var_dleff__blk922) + (locals.var_xsddep * locals.var_dleff__blk922_dn4))), (locals.var_x_1d__blk921_dn6 + ((locals.var_xsddep_dn6 * locals.var_dleff__blk922) + (locals.var_xsddep * locals.var_dleff__blk922_dn6))), (locals.var_x_1d__blk921_dn7 + ((locals.var_xsddep_dn7 * locals.var_dleff__blk922) + (locals.var_xsddep * locals.var_dleff__blk922_dn7))), (locals.var_x_1d__blk921_dn8 + ((locals.var_xsddep_dn8 * locals.var_dleff__blk922) + (locals.var_xsddep * locals.var_dleff__blk922_dn8))), (locals.var_x_1d__blk921_dn9 + ((locals.var_xsddep_dn9 * locals.var_dleff__blk922) + (locals.var_xsddep * locals.var_dleff__blk922_dn9))),)
    } else {
        (locals.var_xedge__blk923, locals.var_xedge__blk923_dn4, locals.var_xedge__blk923_dn6, locals.var_xedge__blk923_dn7, locals.var_xedge__blk923_dn8, locals.var_xedge__blk923_dn9,)
    }
};
        locals.var_xedge__blk923 = assign29950_e32672;
        locals.var_xedge__blk923_dn4 = assign29950_e32672_d_n4;
        locals.var_xedge__blk923_dn6 = assign29950_e32672_d_n6;
        locals.var_xedge__blk923_dn7 = assign29950_e32672_d_n7;
        locals.var_xedge__blk923_dn8 = assign29950_e32672_d_n8;
        locals.var_xedge__blk923_dn9 = assign29950_e32672_d_n9;

        let (assign29960_e32703, assign29960_e32703_d_n4, assign29960_e32703_d_n6, assign29960_e32703_d_n7, assign29960_e32703_d_n8, assign29960_e32703_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign29960_e32678: f64 = (locals.var_pscedlb_i * locals.var_xg20shift__blk900);
        let assign29960_e32679: f64 = (1.0 + assign29960_e32678);
        let assign29960_e32681: f64 = (assign29960_e32679 + 0.5);
        let assign29960_e32685: f64 = (locals.var_pscedlb_i * locals.var_xg20shift__blk900);
        let assign29960_e32686: f64 = (1.0 + assign29960_e32685);
        let assign29960_e32688: f64 = (assign29960_e32686 - 0.5);
        let assign29960_e32692: f64 = (locals.var_pscedlb_i * locals.var_xg20shift__blk900);
        let assign29960_e32693: f64 = (1.0 + assign29960_e32692);
        let assign29960_e32695: f64 = (assign29960_e32693 - 0.5);
        let assign29960_e32696: f64 = (assign29960_e32688 * assign29960_e32695);
        let assign29960_e32698: f64 = (assign29960_e32696 + 0.01);
        let assign29960_e32699: f64 = (assign29960_e32698).sqrt();
        let assign29960_e32700: f64 = (assign29960_e32681 + assign29960_e32699);
        let assign29960_e32701: f64 = (0.5 * assign29960_e32700);
        (assign29960_e32701, (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn4) * assign29960_e32695) + (assign29960_e32688 * (locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn4))) / (2.0 * assign29960_e32699)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn6) * assign29960_e32695) + (assign29960_e32688 * (locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn6))) / (2.0 * assign29960_e32699)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn7) * assign29960_e32695) + (assign29960_e32688 * (locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn7))) / (2.0 * assign29960_e32699)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn8) * assign29960_e32695) + (assign29960_e32688 * (locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn8))) / (2.0 * assign29960_e32699)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn9) * assign29960_e32695) + (assign29960_e32688 * (locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn9))) / (2.0 * assign29960_e32699)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign29960_e32703;
        locals.var_temp_dn4 = assign29960_e32703_d_n4;
        locals.var_temp_dn6 = assign29960_e32703_d_n6;
        locals.var_temp_dn7 = assign29960_e32703_d_n7;
        locals.var_temp_dn8 = assign29960_e32703_d_n8;
        locals.var_temp_dn9 = assign29960_e32703_d_n9;

        let (assign29970_e32713, assign29970_e32713_d_n4, assign29970_e32713_d_n6, assign29970_e32713_d_n7, assign29970_e32713_d_n8, assign29970_e32713_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign29970_e32709: f64 = (locals.var_psce1_loc__blk892 * locals.var_temp);
        let assign29970_e32710: f64 = (1.0 + assign29970_e32709);
        let assign29970_e32711: f64 = (1.0 / assign29970_e32710);
        (assign29970_e32711, (-((locals.var_psce1_loc__blk892 * locals.var_temp_dn4) / (assign29970_e32710 * assign29970_e32710))), (-((locals.var_psce1_loc__blk892 * locals.var_temp_dn6) / (assign29970_e32710 * assign29970_e32710))), (-((locals.var_psce1_loc__blk892 * locals.var_temp_dn7) / (assign29970_e32710 * assign29970_e32710))), (-((locals.var_psce1_loc__blk892 * locals.var_temp_dn8) / (assign29970_e32710 * assign29970_e32710))), (-((locals.var_psce1_loc__blk892 * locals.var_temp_dn9) / (assign29970_e32710 * assign29970_e32710))),)
    } else {
        (locals.var_sce1__blk924, locals.var_sce1__blk924_dn4, locals.var_sce1__blk924_dn6, locals.var_sce1__blk924_dn7, locals.var_sce1__blk924_dn8, locals.var_sce1__blk924_dn9,)
    }
};
        locals.var_sce1__blk924 = assign29970_e32713;
        locals.var_sce1__blk924_dn4 = assign29970_e32713_d_n4;
        locals.var_sce1__blk924_dn6 = assign29970_e32713_d_n6;
        locals.var_sce1__blk924_dn7 = assign29970_e32713_d_n7;
        locals.var_sce1__blk924_dn8 = assign29970_e32713_d_n8;
        locals.var_sce1__blk924_dn9 = assign29970_e32713_d_n9;

        let (assign29980_e32723, assign29980_e32723_d_n4, assign29980_e32723_d_n6, assign29980_e32723_d_n7, assign29980_e32723_d_n8, assign29980_e32723_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign29980_e32719: f64 = (locals.var_psce2_loc__blk893 * locals.var_temp);
        let assign29980_e32720: f64 = (1.0 + assign29980_e32719);
        let assign29980_e32721: f64 = (1.0 / assign29980_e32720);
        (assign29980_e32721, (-((locals.var_psce2_loc__blk893 * locals.var_temp_dn4) / (assign29980_e32720 * assign29980_e32720))), (-((locals.var_psce2_loc__blk893 * locals.var_temp_dn6) / (assign29980_e32720 * assign29980_e32720))), (-((locals.var_psce2_loc__blk893 * locals.var_temp_dn7) / (assign29980_e32720 * assign29980_e32720))), (-((locals.var_psce2_loc__blk893 * locals.var_temp_dn8) / (assign29980_e32720 * assign29980_e32720))), (-((locals.var_psce2_loc__blk893 * locals.var_temp_dn9) / (assign29980_e32720 * assign29980_e32720))),)
    } else {
        (locals.var_sce2__blk925, locals.var_sce2__blk925_dn4, locals.var_sce2__blk925_dn6, locals.var_sce2__blk925_dn7, locals.var_sce2__blk925_dn8, locals.var_sce2__blk925_dn9,)
    }
};
        locals.var_sce2__blk925 = assign29980_e32723;
        locals.var_sce2__blk925_dn4 = assign29980_e32723_d_n4;
        locals.var_sce2__blk925_dn6 = assign29980_e32723_d_n6;
        locals.var_sce2__blk925_dn7 = assign29980_e32723_d_n7;
        locals.var_sce2__blk925_dn8 = assign29980_e32723_d_n8;
        locals.var_sce2__blk925_dn9 = assign29980_e32723_d_n9;

        let (assign29990_e32750, assign29990_e32750_d_n4, assign29990_e32750_d_n6, assign29990_e32750_d_n7, assign29990_e32750_d_n8, assign29990_e32750_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign29990_e32727: f64 = (2.0 * locals.var_xd0);
        let assign29990_e32731: f64 = (locals.var_xdsx / locals.var_xd0);
        let assign29990_e32732: f64 = (1.0 + assign29990_e32731);
        let assign29990_e32733: f64 = (assign29990_e32732).sqrt();
        let assign29990_e32735: f64 = (assign29990_e32733 - 1.0);
        let assign29990_e32736: f64 = (assign29990_e32727 * assign29990_e32735);
        let assign29990_e32740: f64 = (locals.var_cfdl_i * locals.var_dleff__blk922);
        let assign29990_e32741: f64 = (1.0 + assign29990_e32740);
        let assign29990_e32742: f64 = (assign29990_e32736 * assign29990_e32741);
        let assign29990_e32746: f64 = (locals.var_cfdlb_i * locals.var_xg20shift__blk900);
        let assign29990_e32747: f64 = (1.0 + assign29990_e32746);
        let assign29990_e32748: f64 = (assign29990_e32742 * assign29990_e32747);
        (assign29990_e32748, (((((((2.0 * locals.var_xd0_dn4) * assign29990_e32735) + (assign29990_e32727 * ((((locals.var_xdsx_dn4 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn4)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign29990_e32733)))) * assign29990_e32741) + (assign29990_e32736 * (locals.var_cfdl_i * locals.var_dleff__blk922_dn4))) * assign29990_e32747) + (assign29990_e32742 * (locals.var_cfdlb_i * locals.var_xg20shift__blk900_dn4))), (((((((2.0 * locals.var_xd0_dn6) * assign29990_e32735) + (assign29990_e32727 * ((((locals.var_xdsx_dn6 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn6)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign29990_e32733)))) * assign29990_e32741) + (assign29990_e32736 * (locals.var_cfdl_i * locals.var_dleff__blk922_dn6))) * assign29990_e32747) + (assign29990_e32742 * (locals.var_cfdlb_i * locals.var_xg20shift__blk900_dn6))), (((((((2.0 * locals.var_xd0_dn7) * assign29990_e32735) + (assign29990_e32727 * ((((locals.var_xdsx_dn7 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn7)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign29990_e32733)))) * assign29990_e32741) + (assign29990_e32736 * (locals.var_cfdl_i * locals.var_dleff__blk922_dn7))) * assign29990_e32747) + (assign29990_e32742 * (locals.var_cfdlb_i * locals.var_xg20shift__blk900_dn7))), (((((((2.0 * locals.var_xd0_dn8) * assign29990_e32735) + (assign29990_e32727 * ((((locals.var_xdsx_dn8 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn8)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign29990_e32733)))) * assign29990_e32741) + (assign29990_e32736 * (locals.var_cfdl_i * locals.var_dleff__blk922_dn8))) * assign29990_e32747) + (assign29990_e32742 * (locals.var_cfdlb_i * locals.var_xg20shift__blk900_dn8))), (((((((2.0 * locals.var_xd0_dn9) * assign29990_e32735) + (assign29990_e32727 * ((((locals.var_xdsx_dn9 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn9)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign29990_e32733)))) * assign29990_e32741) + (assign29990_e32736 * (locals.var_cfdl_i * locals.var_dleff__blk922_dn9))) * assign29990_e32747) + (assign29990_e32742 * (locals.var_cfdlb_i * locals.var_xg20shift__blk900_dn9))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign29990_e32750;
        locals.var_temp_dn4 = assign29990_e32750_d_n4;
        locals.var_temp_dn6 = assign29990_e32750_d_n6;
        locals.var_temp_dn7 = assign29990_e32750_d_n7;
        locals.var_temp_dn8 = assign29990_e32750_d_n8;
        locals.var_temp_dn9 = assign29990_e32750_d_n9;

        let (assign30000_e32756, assign30000_e32756_d_n4, assign30000_e32756_d_n6, assign30000_e32756_d_n7, assign30000_e32756_d_n8, assign30000_e32756_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30000_e32754: f64 = (locals.var_cf1_loc__blk894 * locals.var_temp);
        (assign30000_e32754, ((locals.var_cf1_loc__blk894_dn4 * locals.var_temp) + (locals.var_cf1_loc__blk894 * locals.var_temp_dn4)), ((locals.var_cf1_loc__blk894_dn6 * locals.var_temp) + (locals.var_cf1_loc__blk894 * locals.var_temp_dn6)), ((locals.var_cf1_loc__blk894_dn7 * locals.var_temp) + (locals.var_cf1_loc__blk894 * locals.var_temp_dn7)), ((locals.var_cf1_loc__blk894_dn8 * locals.var_temp) + (locals.var_cf1_loc__blk894 * locals.var_temp_dn8)), ((locals.var_cf1_loc__blk894_dn9 * locals.var_temp) + (locals.var_cf1_loc__blk894 * locals.var_temp_dn9)),)
    } else {
        (locals.var_dxg1_dibl__blk926, locals.var_dxg1_dibl__blk926_dn4, locals.var_dxg1_dibl__blk926_dn6, locals.var_dxg1_dibl__blk926_dn7, locals.var_dxg1_dibl__blk926_dn8, locals.var_dxg1_dibl__blk926_dn9,)
    }
};
        locals.var_dxg1_dibl__blk926 = assign30000_e32756;
        locals.var_dxg1_dibl__blk926_dn4 = assign30000_e32756_d_n4;
        locals.var_dxg1_dibl__blk926_dn6 = assign30000_e32756_d_n6;
        locals.var_dxg1_dibl__blk926_dn7 = assign30000_e32756_d_n7;
        locals.var_dxg1_dibl__blk926_dn8 = assign30000_e32756_d_n8;
        locals.var_dxg1_dibl__blk926_dn9 = assign30000_e32756_d_n9;

        let (assign30010_e32762, assign30010_e32762_d_n4, assign30010_e32762_d_n6, assign30010_e32762_d_n7, assign30010_e32762_d_n8, assign30010_e32762_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30010_e32760: f64 = (locals.var_cf2_loc__blk895 * locals.var_temp);
        (assign30010_e32760, ((locals.var_cf2_loc__blk895_dn4 * locals.var_temp) + (locals.var_cf2_loc__blk895 * locals.var_temp_dn4)), ((locals.var_cf2_loc__blk895_dn6 * locals.var_temp) + (locals.var_cf2_loc__blk895 * locals.var_temp_dn6)), ((locals.var_cf2_loc__blk895_dn7 * locals.var_temp) + (locals.var_cf2_loc__blk895 * locals.var_temp_dn7)), ((locals.var_cf2_loc__blk895_dn8 * locals.var_temp) + (locals.var_cf2_loc__blk895 * locals.var_temp_dn8)), ((locals.var_cf2_loc__blk895_dn9 * locals.var_temp) + (locals.var_cf2_loc__blk895 * locals.var_temp_dn9)),)
    } else {
        (locals.var_dxg2_dibl__blk927, locals.var_dxg2_dibl__blk927_dn4, locals.var_dxg2_dibl__blk927_dn6, locals.var_dxg2_dibl__blk927_dn7, locals.var_dxg2_dibl__blk927_dn8, locals.var_dxg2_dibl__blk927_dn9,)
    }
};
        locals.var_dxg2_dibl__blk927 = assign30010_e32762;
        locals.var_dxg2_dibl__blk927_dn4 = assign30010_e32762_d_n4;
        locals.var_dxg2_dibl__blk927_dn6 = assign30010_e32762_d_n6;
        locals.var_dxg2_dibl__blk927_dn7 = assign30010_e32762_d_n7;
        locals.var_dxg2_dibl__blk927_dn8 = assign30010_e32762_d_n8;
        locals.var_dxg2_dibl__blk927_dn9 = assign30010_e32762_d_n9;

        let (assign30020_e32776, assign30020_e32776_d_n4, assign30020_e32776_d_n6, assign30020_e32776_d_n7, assign30020_e32776_d_n8, assign30020_e32776_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30020_e32766: f64 = (locals.var_xg10__blk899 - locals.var_xedge__blk923);
        let assign30020_e32768: f64 = (assign30020_e32766 + locals.var_dxg1_dibl__blk926);
        let assign30020_e32770: f64 = (assign30020_e32768 * locals.var_sce1__blk924);
        let assign30020_e32772: f64 = (assign30020_e32770 + locals.var_xedge__blk923);
        let assign30020_e32774: f64 = (assign30020_e32772 + locals.var_dxdsx);
        (assign30020_e32774, ((((((locals.var_xg10__blk899_dn4 - locals.var_xedge__blk923_dn4) + locals.var_dxg1_dibl__blk926_dn4) * locals.var_sce1__blk924) + (assign30020_e32768 * locals.var_sce1__blk924_dn4)) + locals.var_xedge__blk923_dn4) + locals.var_dxdsx_dn4), ((((((locals.var_xg10__blk899_dn6 - locals.var_xedge__blk923_dn6) + locals.var_dxg1_dibl__blk926_dn6) * locals.var_sce1__blk924) + (assign30020_e32768 * locals.var_sce1__blk924_dn6)) + locals.var_xedge__blk923_dn6) + locals.var_dxdsx_dn6), ((((((locals.var_xg10__blk899_dn7 - locals.var_xedge__blk923_dn7) + locals.var_dxg1_dibl__blk926_dn7) * locals.var_sce1__blk924) + (assign30020_e32768 * locals.var_sce1__blk924_dn7)) + locals.var_xedge__blk923_dn7) + locals.var_dxdsx_dn7), ((((((locals.var_xg10__blk899_dn8 - locals.var_xedge__blk923_dn8) + locals.var_dxg1_dibl__blk926_dn8) * locals.var_sce1__blk924) + (assign30020_e32768 * locals.var_sce1__blk924_dn8)) + locals.var_xedge__blk923_dn8) + locals.var_dxdsx_dn8), ((((((locals.var_xg10__blk899_dn9 - locals.var_xedge__blk923_dn9) + locals.var_dxg1_dibl__blk926_dn9) * locals.var_sce1__blk924) + (assign30020_e32768 * locals.var_sce1__blk924_dn9)) + locals.var_xedge__blk923_dn9) + locals.var_dxdsx_dn9),)
    } else {
        (locals.var_xg1__blk928, locals.var_xg1__blk928_dn4, locals.var_xg1__blk928_dn6, locals.var_xg1__blk928_dn7, locals.var_xg1__blk928_dn8, locals.var_xg1__blk928_dn9,)
    }
};
        locals.var_xg1__blk928 = assign30020_e32776;
        locals.var_xg1__blk928_dn4 = assign30020_e32776_d_n4;
        locals.var_xg1__blk928_dn6 = assign30020_e32776_d_n6;
        locals.var_xg1__blk928_dn7 = assign30020_e32776_d_n7;
        locals.var_xg1__blk928_dn8 = assign30020_e32776_d_n8;
        locals.var_xg1__blk928_dn9 = assign30020_e32776_d_n9;

        let (assign30030_e32790, assign30030_e32790_d_n4, assign30030_e32790_d_n6, assign30030_e32790_d_n7, assign30030_e32790_d_n8, assign30030_e32790_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30030_e32780: f64 = (locals.var_xg2eff__blk910 - locals.var_xedge__blk923);
        let assign30030_e32782: f64 = (assign30030_e32780 + locals.var_dxg2_dibl__blk927);
        let assign30030_e32784: f64 = (assign30030_e32782 * locals.var_sce2__blk925);
        let assign30030_e32786: f64 = (assign30030_e32784 + locals.var_xedge__blk923);
        let assign30030_e32788: f64 = (assign30030_e32786 + locals.var_dxdsx);
        (assign30030_e32788, ((((((locals.var_xg2eff__blk910_dn4 - locals.var_xedge__blk923_dn4) + locals.var_dxg2_dibl__blk927_dn4) * locals.var_sce2__blk925) + (assign30030_e32782 * locals.var_sce2__blk925_dn4)) + locals.var_xedge__blk923_dn4) + locals.var_dxdsx_dn4), ((((((locals.var_xg2eff__blk910_dn6 - locals.var_xedge__blk923_dn6) + locals.var_dxg2_dibl__blk927_dn6) * locals.var_sce2__blk925) + (assign30030_e32782 * locals.var_sce2__blk925_dn6)) + locals.var_xedge__blk923_dn6) + locals.var_dxdsx_dn6), ((((((locals.var_xg2eff__blk910_dn7 - locals.var_xedge__blk923_dn7) + locals.var_dxg2_dibl__blk927_dn7) * locals.var_sce2__blk925) + (assign30030_e32782 * locals.var_sce2__blk925_dn7)) + locals.var_xedge__blk923_dn7) + locals.var_dxdsx_dn7), ((((((locals.var_xg2eff__blk910_dn8 - locals.var_xedge__blk923_dn8) + locals.var_dxg2_dibl__blk927_dn8) * locals.var_sce2__blk925) + (assign30030_e32782 * locals.var_sce2__blk925_dn8)) + locals.var_xedge__blk923_dn8) + locals.var_dxdsx_dn8), ((((((locals.var_xg2eff__blk910_dn9 - locals.var_xedge__blk923_dn9) + locals.var_dxg2_dibl__blk927_dn9) * locals.var_sce2__blk925) + (assign30030_e32782 * locals.var_sce2__blk925_dn9)) + locals.var_xedge__blk923_dn9) + locals.var_dxdsx_dn9),)
    } else {
        (locals.var_xg2__blk929, locals.var_xg2__blk929_dn4, locals.var_xg2__blk929_dn6, locals.var_xg2__blk929_dn7, locals.var_xg2__blk929_dn8, locals.var_xg2__blk929_dn9,)
    }
};
        locals.var_xg2__blk929 = assign30030_e32790;
        locals.var_xg2__blk929_dn4 = assign30030_e32790_d_n4;
        locals.var_xg2__blk929_dn6 = assign30030_e32790_d_n6;
        locals.var_xg2__blk929_dn7 = assign30030_e32790_d_n7;
        locals.var_xg2__blk929_dn8 = assign30030_e32790_d_n8;
        locals.var_xg2__blk929_dn9 = assign30030_e32790_d_n9;

    }

    pub(super) fn stamp_transient_block_77(
        locals: &mut StampLocals,
    ) {
        let (assign30040_e32827, assign30040_e32827_d_n4, assign30040_e32827_d_n6, assign30040_e32827_d_n7, assign30040_e32827_d_n8, assign30040_e32827_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30040_e32797: f64 = (locals.var_xg1__blk928 - locals.var_xg2__blk929);
        let assign30040_e32798: f64 = (locals.var_cic1_i * assign30040_e32797);
        let assign30040_e32799: f64 = (locals.var_xg2__blk929 + assign30040_e32798);
        let assign30040_e32801: f64 = (assign30040_e32799 + locals.var_xsatmax);
        let assign30040_e32806: f64 = (locals.var_xg1__blk928 - locals.var_xg2__blk929);
        let assign30040_e32807: f64 = (locals.var_cic1_i * assign30040_e32806);
        let assign30040_e32808: f64 = (locals.var_xg2__blk929 + assign30040_e32807);
        let assign30040_e32810: f64 = (assign30040_e32808 - locals.var_xsatmax);
        let assign30040_e32815: f64 = (locals.var_xg1__blk928 - locals.var_xg2__blk929);
        let assign30040_e32816: f64 = (locals.var_cic1_i * assign30040_e32815);
        let assign30040_e32817: f64 = (locals.var_xg2__blk929 + assign30040_e32816);
        let assign30040_e32819: f64 = (assign30040_e32817 - locals.var_xsatmax);
        let assign30040_e32820: f64 = (assign30040_e32810 * assign30040_e32819);
        let assign30040_e32822: f64 = (assign30040_e32820 + 0.01);
        let assign30040_e32823: f64 = (assign30040_e32822).sqrt();
        let assign30040_e32824: f64 = (assign30040_e32801 - assign30040_e32823);
        let assign30040_e32825: f64 = (0.5 * assign30040_e32824);
        (assign30040_e32825, (0.5 * (((locals.var_xg2__blk929_dn4 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn4 - locals.var_xg2__blk929_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg2__blk929_dn4 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn4 - locals.var_xg2__blk929_dn4))) - locals.var_xsatmax_dn4) * assign30040_e32819) + (assign30040_e32810 * ((locals.var_xg2__blk929_dn4 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn4 - locals.var_xg2__blk929_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign30040_e32823)))), (0.5 * (((locals.var_xg2__blk929_dn6 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn6 - locals.var_xg2__blk929_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg2__blk929_dn6 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn6 - locals.var_xg2__blk929_dn6))) - locals.var_xsatmax_dn6) * assign30040_e32819) + (assign30040_e32810 * ((locals.var_xg2__blk929_dn6 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn6 - locals.var_xg2__blk929_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign30040_e32823)))), (0.5 * (((locals.var_xg2__blk929_dn7 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn7 - locals.var_xg2__blk929_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg2__blk929_dn7 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn7 - locals.var_xg2__blk929_dn7))) - locals.var_xsatmax_dn7) * assign30040_e32819) + (assign30040_e32810 * ((locals.var_xg2__blk929_dn7 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn7 - locals.var_xg2__blk929_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign30040_e32823)))), (0.5 * (((locals.var_xg2__blk929_dn8 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn8 - locals.var_xg2__blk929_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg2__blk929_dn8 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn8 - locals.var_xg2__blk929_dn8))) - locals.var_xsatmax_dn8) * assign30040_e32819) + (assign30040_e32810 * ((locals.var_xg2__blk929_dn8 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn8 - locals.var_xg2__blk929_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign30040_e32823)))), (0.5 * (((locals.var_xg2__blk929_dn9 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn9 - locals.var_xg2__blk929_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg2__blk929_dn9 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn9 - locals.var_xg2__blk929_dn9))) - locals.var_xsatmax_dn9) * assign30040_e32819) + (assign30040_e32810 * ((locals.var_xg2__blk929_dn9 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn9 - locals.var_xg2__blk929_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign30040_e32823)))),)
    } else {
        (locals.var_xg1x__blk930, locals.var_xg1x__blk930_dn4, locals.var_xg1x__blk930_dn6, locals.var_xg1x__blk930_dn7, locals.var_xg1x__blk930_dn8, locals.var_xg1x__blk930_dn9,)
    }
};
        locals.var_xg1x__blk930 = assign30040_e32827;
        locals.var_xg1x__blk930_dn4 = assign30040_e32827_d_n4;
        locals.var_xg1x__blk930_dn6 = assign30040_e32827_d_n6;
        locals.var_xg1x__blk930_dn7 = assign30040_e32827_d_n7;
        locals.var_xg1x__blk930_dn8 = assign30040_e32827_d_n8;
        locals.var_xg1x__blk930_dn9 = assign30040_e32827_d_n9;

        let (assign30050_e32864, assign30050_e32864_d_n4, assign30050_e32864_d_n6, assign30050_e32864_d_n7, assign30050_e32864_d_n8, assign30050_e32864_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30050_e32834: f64 = (locals.var_xg2__blk929 - locals.var_xg1__blk928);
        let assign30050_e32835: f64 = (locals.var_cic2_i * assign30050_e32834);
        let assign30050_e32836: f64 = (locals.var_xg1__blk928 + assign30050_e32835);
        let assign30050_e32838: f64 = (assign30050_e32836 + locals.var_xsatmax);
        let assign30050_e32843: f64 = (locals.var_xg2__blk929 - locals.var_xg1__blk928);
        let assign30050_e32844: f64 = (locals.var_cic2_i * assign30050_e32843);
        let assign30050_e32845: f64 = (locals.var_xg1__blk928 + assign30050_e32844);
        let assign30050_e32847: f64 = (assign30050_e32845 - locals.var_xsatmax);
        let assign30050_e32852: f64 = (locals.var_xg2__blk929 - locals.var_xg1__blk928);
        let assign30050_e32853: f64 = (locals.var_cic2_i * assign30050_e32852);
        let assign30050_e32854: f64 = (locals.var_xg1__blk928 + assign30050_e32853);
        let assign30050_e32856: f64 = (assign30050_e32854 - locals.var_xsatmax);
        let assign30050_e32857: f64 = (assign30050_e32847 * assign30050_e32856);
        let assign30050_e32859: f64 = (assign30050_e32857 + 0.01);
        let assign30050_e32860: f64 = (assign30050_e32859).sqrt();
        let assign30050_e32861: f64 = (assign30050_e32838 - assign30050_e32860);
        let assign30050_e32862: f64 = (0.5 * assign30050_e32861);
        (assign30050_e32862, (0.5 * (((locals.var_xg1__blk928_dn4 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn4 - locals.var_xg1__blk928_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg1__blk928_dn4 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn4 - locals.var_xg1__blk928_dn4))) - locals.var_xsatmax_dn4) * assign30050_e32856) + (assign30050_e32847 * ((locals.var_xg1__blk928_dn4 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn4 - locals.var_xg1__blk928_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign30050_e32860)))), (0.5 * (((locals.var_xg1__blk928_dn6 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn6 - locals.var_xg1__blk928_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg1__blk928_dn6 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn6 - locals.var_xg1__blk928_dn6))) - locals.var_xsatmax_dn6) * assign30050_e32856) + (assign30050_e32847 * ((locals.var_xg1__blk928_dn6 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn6 - locals.var_xg1__blk928_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign30050_e32860)))), (0.5 * (((locals.var_xg1__blk928_dn7 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn7 - locals.var_xg1__blk928_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg1__blk928_dn7 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn7 - locals.var_xg1__blk928_dn7))) - locals.var_xsatmax_dn7) * assign30050_e32856) + (assign30050_e32847 * ((locals.var_xg1__blk928_dn7 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn7 - locals.var_xg1__blk928_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign30050_e32860)))), (0.5 * (((locals.var_xg1__blk928_dn8 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn8 - locals.var_xg1__blk928_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg1__blk928_dn8 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn8 - locals.var_xg1__blk928_dn8))) - locals.var_xsatmax_dn8) * assign30050_e32856) + (assign30050_e32847 * ((locals.var_xg1__blk928_dn8 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn8 - locals.var_xg1__blk928_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign30050_e32860)))), (0.5 * (((locals.var_xg1__blk928_dn9 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn9 - locals.var_xg1__blk928_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg1__blk928_dn9 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn9 - locals.var_xg1__blk928_dn9))) - locals.var_xsatmax_dn9) * assign30050_e32856) + (assign30050_e32847 * ((locals.var_xg1__blk928_dn9 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn9 - locals.var_xg1__blk928_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign30050_e32860)))),)
    } else {
        (locals.var_xg2x__blk931, locals.var_xg2x__blk931_dn4, locals.var_xg2x__blk931_dn6, locals.var_xg2x__blk931_dn7, locals.var_xg2x__blk931_dn8, locals.var_xg2x__blk931_dn9,)
    }
};
        locals.var_xg2x__blk931 = assign30050_e32864;
        locals.var_xg2x__blk931_dn4 = assign30050_e32864_d_n4;
        locals.var_xg2x__blk931_dn6 = assign30050_e32864_d_n6;
        locals.var_xg2x__blk931_dn7 = assign30050_e32864_d_n7;
        locals.var_xg2x__blk931_dn8 = assign30050_e32864_d_n8;
        locals.var_xg2x__blk931_dn9 = assign30050_e32864_d_n9;

        let (assign30060_e32870, assign30060_e32870_d_n4, assign30060_e32870_d_n6, assign30060_e32870_d_n7, assign30060_e32870_d_n8, assign30060_e32870_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30060_e32868: f64 = (locals.var_k1_1d_qm__blk915 / locals.var_sce1__blk924);
        (assign30060_e32868, (((locals.var_k1_1d_qm__blk915_dn4 * locals.var_sce1__blk924) - (locals.var_k1_1d_qm__blk915 * locals.var_sce1__blk924_dn4)) / (locals.var_sce1__blk924 * locals.var_sce1__blk924)), (((locals.var_k1_1d_qm__blk915_dn6 * locals.var_sce1__blk924) - (locals.var_k1_1d_qm__blk915 * locals.var_sce1__blk924_dn6)) / (locals.var_sce1__blk924 * locals.var_sce1__blk924)), (((locals.var_k1_1d_qm__blk915_dn7 * locals.var_sce1__blk924) - (locals.var_k1_1d_qm__blk915 * locals.var_sce1__blk924_dn7)) / (locals.var_sce1__blk924 * locals.var_sce1__blk924)), (((locals.var_k1_1d_qm__blk915_dn8 * locals.var_sce1__blk924) - (locals.var_k1_1d_qm__blk915 * locals.var_sce1__blk924_dn8)) / (locals.var_sce1__blk924 * locals.var_sce1__blk924)), (((locals.var_k1_1d_qm__blk915_dn9 * locals.var_sce1__blk924) - (locals.var_k1_1d_qm__blk915 * locals.var_sce1__blk924_dn9)) / (locals.var_sce1__blk924 * locals.var_sce1__blk924)),)
    } else {
        (locals.var_k1__blk932, locals.var_k1__blk932_dn4, locals.var_k1__blk932_dn6, locals.var_k1__blk932_dn7, locals.var_k1__blk932_dn8, locals.var_k1__blk932_dn9,)
    }
};
        locals.var_k1__blk932 = assign30060_e32870;
        locals.var_k1__blk932_dn4 = assign30060_e32870_d_n4;
        locals.var_k1__blk932_dn6 = assign30060_e32870_d_n6;
        locals.var_k1__blk932_dn7 = assign30060_e32870_d_n7;
        locals.var_k1__blk932_dn8 = assign30060_e32870_d_n8;
        locals.var_k1__blk932_dn9 = assign30060_e32870_d_n9;

        let (assign30070_e32876, assign30070_e32876_d_n4, assign30070_e32876_d_n6, assign30070_e32876_d_n7, assign30070_e32876_d_n8, assign30070_e32876_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30070_e32874: f64 = (locals.var_k2_1d_qm__blk916 / locals.var_sce2__blk925);
        (assign30070_e32874, (((locals.var_k2_1d_qm__blk916_dn4 * locals.var_sce2__blk925) - (locals.var_k2_1d_qm__blk916 * locals.var_sce2__blk925_dn4)) / (locals.var_sce2__blk925 * locals.var_sce2__blk925)), (((locals.var_k2_1d_qm__blk916_dn6 * locals.var_sce2__blk925) - (locals.var_k2_1d_qm__blk916 * locals.var_sce2__blk925_dn6)) / (locals.var_sce2__blk925 * locals.var_sce2__blk925)), (((locals.var_k2_1d_qm__blk916_dn7 * locals.var_sce2__blk925) - (locals.var_k2_1d_qm__blk916 * locals.var_sce2__blk925_dn7)) / (locals.var_sce2__blk925 * locals.var_sce2__blk925)), (((locals.var_k2_1d_qm__blk916_dn8 * locals.var_sce2__blk925) - (locals.var_k2_1d_qm__blk916 * locals.var_sce2__blk925_dn8)) / (locals.var_sce2__blk925 * locals.var_sce2__blk925)), (((locals.var_k2_1d_qm__blk916_dn9 * locals.var_sce2__blk925) - (locals.var_k2_1d_qm__blk916 * locals.var_sce2__blk925_dn9)) / (locals.var_sce2__blk925 * locals.var_sce2__blk925)),)
    } else {
        (locals.var_k2__blk933, locals.var_k2__blk933_dn4, locals.var_k2__blk933_dn6, locals.var_k2__blk933_dn7, locals.var_k2__blk933_dn8, locals.var_k2__blk933_dn9,)
    }
};
        locals.var_k2__blk933 = assign30070_e32876;
        locals.var_k2__blk933_dn4 = assign30070_e32876_d_n4;
        locals.var_k2__blk933_dn6 = assign30070_e32876_d_n6;
        locals.var_k2__blk933_dn7 = assign30070_e32876_d_n7;
        locals.var_k2__blk933_dn8 = assign30070_e32876_d_n8;
        locals.var_k2__blk933_dn9 = assign30070_e32876_d_n9;

        let (assign30080_e32882, assign30080_e32882_d_n4, assign30080_e32882_d_n6, assign30080_e32882_d_n7, assign30080_e32882_d_n8, assign30080_e32882_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30080_e32880: f64 = (1.0 / locals.var_k1__blk932);
        (assign30080_e32880, (-(locals.var_k1__blk932_dn4 / (locals.var_k1__blk932 * locals.var_k1__blk932))), (-(locals.var_k1__blk932_dn6 / (locals.var_k1__blk932 * locals.var_k1__blk932))), (-(locals.var_k1__blk932_dn7 / (locals.var_k1__blk932 * locals.var_k1__blk932))), (-(locals.var_k1__blk932_dn8 / (locals.var_k1__blk932 * locals.var_k1__blk932))), (-(locals.var_k1__blk932_dn9 / (locals.var_k1__blk932 * locals.var_k1__blk932))),)
    } else {
        (locals.var_inv_k1__blk906, locals.var_inv_k1__blk906_dn4, locals.var_inv_k1__blk906_dn6, locals.var_inv_k1__blk906_dn7, locals.var_inv_k1__blk906_dn8, locals.var_inv_k1__blk906_dn9,)
    }
};
        locals.var_inv_k1__blk906 = assign30080_e32882;
        locals.var_inv_k1__blk906_dn4 = assign30080_e32882_d_n4;
        locals.var_inv_k1__blk906_dn6 = assign30080_e32882_d_n6;
        locals.var_inv_k1__blk906_dn7 = assign30080_e32882_d_n7;
        locals.var_inv_k1__blk906_dn8 = assign30080_e32882_d_n8;
        locals.var_inv_k1__blk906_dn9 = assign30080_e32882_d_n9;

        let (assign30090_e32888, assign30090_e32888_d_n4, assign30090_e32888_d_n6, assign30090_e32888_d_n7, assign30090_e32888_d_n8, assign30090_e32888_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30090_e32886: f64 = (1.0 / locals.var_k2__blk933);
        (assign30090_e32886, (-(locals.var_k2__blk933_dn4 / (locals.var_k2__blk933 * locals.var_k2__blk933))), (-(locals.var_k2__blk933_dn6 / (locals.var_k2__blk933 * locals.var_k2__blk933))), (-(locals.var_k2__blk933_dn7 / (locals.var_k2__blk933 * locals.var_k2__blk933))), (-(locals.var_k2__blk933_dn8 / (locals.var_k2__blk933 * locals.var_k2__blk933))), (-(locals.var_k2__blk933_dn9 / (locals.var_k2__blk933 * locals.var_k2__blk933))),)
    } else {
        (locals.var_inv_k2__blk907, locals.var_inv_k2__blk907_dn4, locals.var_inv_k2__blk907_dn6, locals.var_inv_k2__blk907_dn7, locals.var_inv_k2__blk907_dn8, locals.var_inv_k2__blk907_dn9,)
    }
};
        locals.var_inv_k2__blk907 = assign30090_e32888;
        locals.var_inv_k2__blk907_dn4 = assign30090_e32888_d_n4;
        locals.var_inv_k2__blk907_dn6 = assign30090_e32888_d_n6;
        locals.var_inv_k2__blk907_dn7 = assign30090_e32888_d_n7;
        locals.var_inv_k2__blk907_dn8 = assign30090_e32888_d_n8;
        locals.var_inv_k2__blk907_dn9 = assign30090_e32888_d_n9;

        let (assign30100_e32898, assign30100_e32898_d_n4, assign30100_e32898_d_n6, assign30100_e32898_d_n7, assign30100_e32898_d_n8, assign30100_e32898_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30100_e32893: f64 = (1.0 + locals.var_inv_k1__blk906);
        let assign30100_e32895: f64 = (assign30100_e32893 + locals.var_inv_k2__blk907);
        let assign30100_e32896: f64 = (1.0 / assign30100_e32895);
        (assign30100_e32896, (-((locals.var_inv_k1__blk906_dn4 + locals.var_inv_k2__blk907_dn4) / (assign30100_e32895 * assign30100_e32895))), (-((locals.var_inv_k1__blk906_dn6 + locals.var_inv_k2__blk907_dn6) / (assign30100_e32895 * assign30100_e32895))), (-((locals.var_inv_k1__blk906_dn7 + locals.var_inv_k2__blk907_dn7) / (assign30100_e32895 * assign30100_e32895))), (-((locals.var_inv_k1__blk906_dn8 + locals.var_inv_k2__blk907_dn8) / (assign30100_e32895 * assign30100_e32895))), (-((locals.var_inv_k1__blk906_dn9 + locals.var_inv_k2__blk907_dn9) / (assign30100_e32895 * assign30100_e32895))),)
    } else {
        (locals.var_keq__blk934, locals.var_keq__blk934_dn4, locals.var_keq__blk934_dn6, locals.var_keq__blk934_dn7, locals.var_keq__blk934_dn8, locals.var_keq__blk934_dn9,)
    }
};
        locals.var_keq__blk934 = assign30100_e32898;
        locals.var_keq__blk934_dn4 = assign30100_e32898_d_n4;
        locals.var_keq__blk934_dn6 = assign30100_e32898_d_n6;
        locals.var_keq__blk934_dn7 = assign30100_e32898_d_n7;
        locals.var_keq__blk934_dn8 = assign30100_e32898_d_n8;
        locals.var_keq__blk934_dn9 = assign30100_e32898_d_n9;

        let (assign30110_e32906, assign30110_e32906_d_n4, assign30110_e32906_d_n6, assign30110_e32906_d_n7, assign30110_e32906_d_n8, assign30110_e32906_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30110_e32903: f64 = (locals.var_csiprime__blk919 * locals.var_csiprime__blk919);
        let assign30110_e32904: f64 = (locals.var_a0_csisq / assign30110_e32903);
        (assign30110_e32904, (((locals.var_a0_csisq_dn4 * assign30110_e32903) - (locals.var_a0_csisq * ((locals.var_csiprime__blk919_dn4 * locals.var_csiprime__blk919) + (locals.var_csiprime__blk919 * locals.var_csiprime__blk919_dn4)))) / (assign30110_e32903 * assign30110_e32903)), (((locals.var_a0_csisq_dn6 * assign30110_e32903) - (locals.var_a0_csisq * ((locals.var_csiprime__blk919_dn6 * locals.var_csiprime__blk919) + (locals.var_csiprime__blk919 * locals.var_csiprime__blk919_dn6)))) / (assign30110_e32903 * assign30110_e32903)), (((locals.var_a0_csisq_dn7 * assign30110_e32903) - (locals.var_a0_csisq * ((locals.var_csiprime__blk919_dn7 * locals.var_csiprime__blk919) + (locals.var_csiprime__blk919 * locals.var_csiprime__blk919_dn7)))) / (assign30110_e32903 * assign30110_e32903)), (((locals.var_a0_csisq_dn8 * assign30110_e32903) - (locals.var_a0_csisq * ((locals.var_csiprime__blk919_dn8 * locals.var_csiprime__blk919) + (locals.var_csiprime__blk919 * locals.var_csiprime__blk919_dn8)))) / (assign30110_e32903 * assign30110_e32903)), (((locals.var_a0_csisq_dn9 * assign30110_e32903) - (locals.var_a0_csisq * ((locals.var_csiprime__blk919_dn9 * locals.var_csiprime__blk919) + (locals.var_csiprime__blk919 * locals.var_csiprime__blk919_dn9)))) / (assign30110_e32903 * assign30110_e32903)),)
    } else {
        (locals.var_a0__blk905, locals.var_a0__blk905_dn4, locals.var_a0__blk905_dn6, locals.var_a0__blk905_dn7, locals.var_a0__blk905_dn8, locals.var_a0__blk905_dn9,)
    }
};
        locals.var_a0__blk905 = assign30110_e32906;
        locals.var_a0__blk905_dn4 = assign30110_e32906_d_n4;
        locals.var_a0__blk905_dn6 = assign30110_e32906_d_n6;
        locals.var_a0__blk905_dn7 = assign30110_e32906_d_n7;
        locals.var_a0__blk905_dn8 = assign30110_e32906_d_n8;
        locals.var_a0__blk905_dn9 = assign30110_e32906_d_n9;

        let (assign30120_e32916, assign30120_e32916_d_n4, assign30120_e32916_d_n6, assign30120_e32916_d_n7, assign30120_e32916_d_n8, assign30120_e32916_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30120_e32910: f64 = (1.0 + locals.var_k1__blk932);
        let assign30120_e32913: f64 = (1.0 + locals.var_k2__blk933);
        let assign30120_e32914: f64 = (assign30120_e32910 / assign30120_e32913);
        (assign30120_e32914, (((locals.var_k1__blk932_dn4 * assign30120_e32913) - (assign30120_e32910 * locals.var_k2__blk933_dn4)) / (assign30120_e32913 * assign30120_e32913)), (((locals.var_k1__blk932_dn6 * assign30120_e32913) - (assign30120_e32910 * locals.var_k2__blk933_dn6)) / (assign30120_e32913 * assign30120_e32913)), (((locals.var_k1__blk932_dn7 * assign30120_e32913) - (assign30120_e32910 * locals.var_k2__blk933_dn7)) / (assign30120_e32913 * assign30120_e32913)), (((locals.var_k1__blk932_dn8 * assign30120_e32913) - (assign30120_e32910 * locals.var_k2__blk933_dn8)) / (assign30120_e32913 * assign30120_e32913)), (((locals.var_k1__blk932_dn9 * assign30120_e32913) - (assign30120_e32910 * locals.var_k2__blk933_dn9)) / (assign30120_e32913 * assign30120_e32913)),)
    } else {
        (locals.var_exp_dxth__blk902, locals.var_exp_dxth__blk902_dn4, locals.var_exp_dxth__blk902_dn6, locals.var_exp_dxth__blk902_dn7, locals.var_exp_dxth__blk902_dn8, locals.var_exp_dxth__blk902_dn9,)
    }
};
        locals.var_exp_dxth__blk902 = assign30120_e32916;
        locals.var_exp_dxth__blk902_dn4 = assign30120_e32916_d_n4;
        locals.var_exp_dxth__blk902_dn6 = assign30120_e32916_d_n6;
        locals.var_exp_dxth__blk902_dn7 = assign30120_e32916_d_n7;
        locals.var_exp_dxth__blk902_dn8 = assign30120_e32916_d_n8;
        locals.var_exp_dxth__blk902_dn9 = assign30120_e32916_d_n9;

        let (assign30130_e32921, assign30130_e32921_d_n4, assign30130_e32921_d_n6, assign30130_e32921_d_n7, assign30130_e32921_d_n8, assign30130_e32921_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30130_e32919: f64 = (locals.var_exp_dxth__blk902).ln();
        (assign30130_e32919, (locals.var_exp_dxth__blk902_dn4 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn6 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn7 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn8 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn9 / locals.var_exp_dxth__blk902),)
    } else {
        (locals.var_dxth__blk903, locals.var_dxth__blk903_dn4, locals.var_dxth__blk903_dn6, locals.var_dxth__blk903_dn7, locals.var_dxth__blk903_dn8, locals.var_dxth__blk903_dn9,)
    }
};
        locals.var_dxth__blk903 = assign30130_e32921;
        locals.var_dxth__blk903_dn4 = assign30130_e32921_d_n4;
        locals.var_dxth__blk903_dn6 = assign30130_e32921_d_n6;
        locals.var_dxth__blk903_dn7 = assign30130_e32921_d_n7;
        locals.var_dxth__blk903_dn8 = assign30130_e32921_d_n8;
        locals.var_dxth__blk903_dn9 = assign30130_e32921_d_n9;

        let assign30140_e32924: f64 = if locals.var_dxth__blk903 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1097 = assign30140_e32924;

        let (assign30150_e32940, assign30150_e32940_d_n4, assign30150_e32940_d_n6, assign30150_e32940_d_n7, assign30150_e32940_d_n8, assign30150_e32940_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1097 != 0.0)) {
        let assign30150_e32930: f64 = (2.0 * locals.var_dxth__blk903);
        let assign30150_e32933: f64 = (locals.var_exp_dxth__blk902 + 1.0);
        let assign30150_e32934: f64 = (assign30150_e32930 * assign30150_e32933);
        let assign30150_e32937: f64 = (locals.var_exp_dxth__blk902 - 1.0);
        let assign30150_e32938: f64 = (assign30150_e32934 / assign30150_e32937);
        (assign30150_e32938, ((((((2.0 * locals.var_dxth__blk903_dn4) * assign30150_e32933) + (assign30150_e32930 * locals.var_exp_dxth__blk902_dn4)) * assign30150_e32937) - (assign30150_e32934 * locals.var_exp_dxth__blk902_dn4)) / (assign30150_e32937 * assign30150_e32937)), ((((((2.0 * locals.var_dxth__blk903_dn6) * assign30150_e32933) + (assign30150_e32930 * locals.var_exp_dxth__blk902_dn6)) * assign30150_e32937) - (assign30150_e32934 * locals.var_exp_dxth__blk902_dn6)) / (assign30150_e32937 * assign30150_e32937)), ((((((2.0 * locals.var_dxth__blk903_dn7) * assign30150_e32933) + (assign30150_e32930 * locals.var_exp_dxth__blk902_dn7)) * assign30150_e32937) - (assign30150_e32934 * locals.var_exp_dxth__blk902_dn7)) / (assign30150_e32937 * assign30150_e32937)), ((((((2.0 * locals.var_dxth__blk903_dn8) * assign30150_e32933) + (assign30150_e32930 * locals.var_exp_dxth__blk902_dn8)) * assign30150_e32937) - (assign30150_e32934 * locals.var_exp_dxth__blk902_dn8)) / (assign30150_e32937 * assign30150_e32937)), ((((((2.0 * locals.var_dxth__blk903_dn9) * assign30150_e32933) + (assign30150_e32930 * locals.var_exp_dxth__blk902_dn9)) * assign30150_e32937) - (assign30150_e32934 * locals.var_exp_dxth__blk902_dn9)) / (assign30150_e32937 * assign30150_e32937)),)
    } else {
        (locals.var_diff_min__blk904, locals.var_diff_min__blk904_dn4, locals.var_diff_min__blk904_dn6, locals.var_diff_min__blk904_dn7, locals.var_diff_min__blk904_dn8, locals.var_diff_min__blk904_dn9,)
    }
};
        locals.var_diff_min__blk904 = assign30150_e32940;
        locals.var_diff_min__blk904_dn4 = assign30150_e32940_d_n4;
        locals.var_diff_min__blk904_dn6 = assign30150_e32940_d_n6;
        locals.var_diff_min__blk904_dn7 = assign30150_e32940_d_n7;
        locals.var_diff_min__blk904_dn8 = assign30150_e32940_d_n8;
        locals.var_diff_min__blk904_dn9 = assign30150_e32940_d_n9;

        let (assign30160_e32951, assign30160_e32951_d_n4, assign30160_e32951_d_n6, assign30160_e32951_d_n7, assign30160_e32951_d_n8, assign30160_e32951_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1097 == 0.0)) {
        let assign30160_e32948: f64 = (2.0 + locals.var_dxth__blk903);
        let assign30160_e32949: f64 = (2.0 * assign30160_e32948);
        (assign30160_e32949, (2.0 * locals.var_dxth__blk903_dn4), (2.0 * locals.var_dxth__blk903_dn6), (2.0 * locals.var_dxth__blk903_dn7), (2.0 * locals.var_dxth__blk903_dn8), (2.0 * locals.var_dxth__blk903_dn9),)
    } else {
        (locals.var_diff_min__blk904, locals.var_diff_min__blk904_dn4, locals.var_diff_min__blk904_dn6, locals.var_diff_min__blk904_dn7, locals.var_diff_min__blk904_dn8, locals.var_diff_min__blk904_dn9,)
    }
};
        locals.var_diff_min__blk904 = assign30160_e32951;
        locals.var_diff_min__blk904_dn4 = assign30160_e32951_d_n4;
        locals.var_diff_min__blk904_dn6 = assign30160_e32951_d_n6;
        locals.var_diff_min__blk904_dn7 = assign30160_e32951_d_n7;
        locals.var_diff_min__blk904_dn8 = assign30160_e32951_d_n8;
        locals.var_diff_min__blk904_dn9 = assign30160_e32951_d_n9;

        let (assign30170_e32959, assign30170_e32959_d_n4, assign30170_e32959_d_n6, assign30170_e32959_d_n7, assign30170_e32959_d_n8, assign30170_e32959_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30170_e32956: f64 = (locals.var_xg1x__blk930 - locals.var_xg2x__blk931);
        let assign30170_e32957: f64 = (locals.var_keq__blk934 * assign30170_e32956);
        (assign30170_e32957, ((locals.var_keq__blk934_dn4 * assign30170_e32956) + (locals.var_keq__blk934 * (locals.var_xg1x__blk930_dn4 - locals.var_xg2x__blk931_dn4))), ((locals.var_keq__blk934_dn6 * assign30170_e32956) + (locals.var_keq__blk934 * (locals.var_xg1x__blk930_dn6 - locals.var_xg2x__blk931_dn6))), ((locals.var_keq__blk934_dn7 * assign30170_e32956) + (locals.var_keq__blk934 * (locals.var_xg1x__blk930_dn7 - locals.var_xg2x__blk931_dn7))), ((locals.var_keq__blk934_dn8 * assign30170_e32956) + (locals.var_keq__blk934 * (locals.var_xg1x__blk930_dn8 - locals.var_xg2x__blk931_dn8))), ((locals.var_keq__blk934_dn9 * assign30170_e32956) + (locals.var_keq__blk934 * (locals.var_xg1x__blk930_dn9 - locals.var_xg2x__blk931_dn9))),)
    } else {
        (locals.var_dx_wi__blk935, locals.var_dx_wi__blk935_dn4, locals.var_dx_wi__blk935_dn6, locals.var_dx_wi__blk935_dn7, locals.var_dx_wi__blk935_dn8, locals.var_dx_wi__blk935_dn9,)
    }
};
        locals.var_dx_wi__blk935 = assign30170_e32959;
        locals.var_dx_wi__blk935_dn4 = assign30170_e32959_d_n4;
        locals.var_dx_wi__blk935_dn6 = assign30170_e32959_d_n6;
        locals.var_dx_wi__blk935_dn7 = assign30170_e32959_d_n7;
        locals.var_dx_wi__blk935_dn8 = assign30170_e32959_d_n8;
        locals.var_dx_wi__blk935_dn9 = assign30170_e32959_d_n9;

        let (assign30180_e32965, assign30180_e32965_d_n4, assign30180_e32965_d_n6, assign30180_e32965_d_n7, assign30180_e32965_d_n8, assign30180_e32965_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30180_e32963: f64 = (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935);
        (assign30180_e32963, ((locals.var_dx_wi__blk935_dn4 * locals.var_dx_wi__blk935) + (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935_dn4)), ((locals.var_dx_wi__blk935_dn6 * locals.var_dx_wi__blk935) + (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935_dn6)), ((locals.var_dx_wi__blk935_dn7 * locals.var_dx_wi__blk935) + (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935_dn7)), ((locals.var_dx_wi__blk935_dn8 * locals.var_dx_wi__blk935) + (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935_dn8)), ((locals.var_dx_wi__blk935_dn9 * locals.var_dx_wi__blk935) + (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935_dn9)),)
    } else {
        (locals.var_dx_wisq__blk936, locals.var_dx_wisq__blk936_dn4, locals.var_dx_wisq__blk936_dn6, locals.var_dx_wisq__blk936_dn7, locals.var_dx_wisq__blk936_dn8, locals.var_dx_wisq__blk936_dn9,)
    }
};
        locals.var_dx_wisq__blk936 = assign30180_e32965;
        locals.var_dx_wisq__blk936_dn4 = assign30180_e32965_d_n4;
        locals.var_dx_wisq__blk936_dn6 = assign30180_e32965_d_n6;
        locals.var_dx_wisq__blk936_dn7 = assign30180_e32965_d_n7;
        locals.var_dx_wisq__blk936_dn8 = assign30180_e32965_d_n8;
        locals.var_dx_wisq__blk936_dn9 = assign30180_e32965_d_n9;

        let (assign30190_e32973, assign30190_e32973_d_n4, assign30190_e32973_d_n6, assign30190_e32973_d_n7, assign30190_e32973_d_n8, assign30190_e32973_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30190_e32970: f64 = (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906);
        let assign30190_e32971: f64 = (locals.var_xg1x__blk930 - assign30190_e32970);
        (assign30190_e32971, (locals.var_xg1x__blk930_dn4 - ((locals.var_dx_wi__blk935_dn4 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn4))), (locals.var_xg1x__blk930_dn6 - ((locals.var_dx_wi__blk935_dn6 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn6))), (locals.var_xg1x__blk930_dn7 - ((locals.var_dx_wi__blk935_dn7 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn7))), (locals.var_xg1x__blk930_dn8 - ((locals.var_dx_wi__blk935_dn8 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn8))), (locals.var_xg1x__blk930_dn9 - ((locals.var_dx_wi__blk935_dn9 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn9))),)
    } else {
        (locals.var_x1_wi0__blk908, locals.var_x1_wi0__blk908_dn4, locals.var_x1_wi0__blk908_dn6, locals.var_x1_wi0__blk908_dn7, locals.var_x1_wi0__blk908_dn8, locals.var_x1_wi0__blk908_dn9,)
    }
};
        locals.var_x1_wi0__blk908 = assign30190_e32973;
        locals.var_x1_wi0__blk908_dn4 = assign30190_e32973_d_n4;
        locals.var_x1_wi0__blk908_dn6 = assign30190_e32973_d_n6;
        locals.var_x1_wi0__blk908_dn7 = assign30190_e32973_d_n7;
        locals.var_x1_wi0__blk908_dn8 = assign30190_e32973_d_n8;
        locals.var_x1_wi0__blk908_dn9 = assign30190_e32973_d_n9;

        let (assign30200_e32981, assign30200_e32981_d_n4, assign30200_e32981_d_n6, assign30200_e32981_d_n7, assign30200_e32981_d_n8, assign30200_e32981_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30200_e32978: f64 = (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907);
        let assign30200_e32979: f64 = (locals.var_xg2x__blk931 + assign30200_e32978);
        (assign30200_e32979, (locals.var_xg2x__blk931_dn4 + ((locals.var_dx_wi__blk935_dn4 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn4))), (locals.var_xg2x__blk931_dn6 + ((locals.var_dx_wi__blk935_dn6 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn6))), (locals.var_xg2x__blk931_dn7 + ((locals.var_dx_wi__blk935_dn7 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn7))), (locals.var_xg2x__blk931_dn8 + ((locals.var_dx_wi__blk935_dn8 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn8))), (locals.var_xg2x__blk931_dn9 + ((locals.var_dx_wi__blk935_dn9 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn9))),)
    } else {
        (locals.var_x2_wi0__blk909, locals.var_x2_wi0__blk909_dn4, locals.var_x2_wi0__blk909_dn6, locals.var_x2_wi0__blk909_dn7, locals.var_x2_wi0__blk909_dn8, locals.var_x2_wi0__blk909_dn9,)
    }
};
        locals.var_x2_wi0__blk909 = assign30200_e32981;
        locals.var_x2_wi0__blk909_dn4 = assign30200_e32981_d_n4;
        locals.var_x2_wi0__blk909_dn6 = assign30200_e32981_d_n6;
        locals.var_x2_wi0__blk909_dn7 = assign30200_e32981_d_n7;
        locals.var_x2_wi0__blk909_dn8 = assign30200_e32981_d_n8;
        locals.var_x2_wi0__blk909_dn9 = assign30200_e32981_d_n9;

        let (assign30210_e32989, assign30210_e32989_d_n4, assign30210_e32989_d_n6, assign30210_e32989_d_n7, assign30210_e32989_d_n8, assign30210_e32989_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30210_e32986: f64 = (locals.var_k1__blk932 + 1.0);
        let assign30210_e32987: f64 = (1.0 / assign30210_e32986);
        (assign30210_e32987, (-(locals.var_k1__blk932_dn4 / (assign30210_e32986 * assign30210_e32986))), (-(locals.var_k1__blk932_dn6 / (assign30210_e32986 * assign30210_e32986))), (-(locals.var_k1__blk932_dn7 / (assign30210_e32986 * assign30210_e32986))), (-(locals.var_k1__blk932_dn8 / (assign30210_e32986 * assign30210_e32986))), (-(locals.var_k1__blk932_dn9 / (assign30210_e32986 * assign30210_e32986))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30210_e32989;
        locals.var_q_temp1__blk814_dn4 = assign30210_e32989_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30210_e32989_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30210_e32989_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30210_e32989_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30210_e32989_d_n9;

        let (assign30220_e32997, assign30220_e32997_d_n4, assign30220_e32997_d_n6, assign30220_e32997_d_n7, assign30220_e32997_d_n8, assign30220_e32997_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30220_e32994: f64 = (locals.var_k2__blk933 + 1.0);
        let assign30220_e32995: f64 = (1.0 / assign30220_e32994);
        (assign30220_e32995, (-(locals.var_k2__blk933_dn4 / (assign30220_e32994 * assign30220_e32994))), (-(locals.var_k2__blk933_dn6 / (assign30220_e32994 * assign30220_e32994))), (-(locals.var_k2__blk933_dn7 / (assign30220_e32994 * assign30220_e32994))), (-(locals.var_k2__blk933_dn8 / (assign30220_e32994 * assign30220_e32994))), (-(locals.var_k2__blk933_dn9 / (assign30220_e32994 * assign30220_e32994))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30220_e32997;
        locals.var_q_temp2__blk815_dn4 = assign30220_e32997_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30220_e32997_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30220_e32997_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30220_e32997_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30220_e32997_d_n9;

        let (assign30230_e33014, assign30230_e33014_d_n4, assign30230_e33014_d_n6, assign30230_e33014_d_n7, assign30230_e33014_d_n8, assign30230_e33014_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30230_e33002: f64 = (locals.var_k2__blk933 * locals.var_q_temp2__blk815);
        let assign30230_e33003: f64 = (locals.var_k1__blk932 + assign30230_e33002);
        let assign30230_e33005: f64 = (assign30230_e33003 * locals.var_diff_min__blk904);
        let assign30230_e33007: f64 = (assign30230_e33005 / locals.var_a0__blk905);
        let assign30230_e33008: f64 = (assign30230_e33007).ln();
        let assign30230_e33010: f64 = assign30230_e33008;
        let assign30230_e33012: f64 = (assign30230_e33010 + 3.0);
        (assign30230_e33012, (((((((locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn4))) * locals.var_diff_min__blk904) + (assign30230_e33003 * locals.var_diff_min__blk904_dn4)) * locals.var_a0__blk905) - (assign30230_e33005 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30230_e33007), (((((((locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn6))) * locals.var_diff_min__blk904) + (assign30230_e33003 * locals.var_diff_min__blk904_dn6)) * locals.var_a0__blk905) - (assign30230_e33005 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30230_e33007), (((((((locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn7))) * locals.var_diff_min__blk904) + (assign30230_e33003 * locals.var_diff_min__blk904_dn7)) * locals.var_a0__blk905) - (assign30230_e33005 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30230_e33007), (((((((locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn8))) * locals.var_diff_min__blk904) + (assign30230_e33003 * locals.var_diff_min__blk904_dn8)) * locals.var_a0__blk905) - (assign30230_e33005 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30230_e33007), (((((((locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn9))) * locals.var_diff_min__blk904) + (assign30230_e33003 * locals.var_diff_min__blk904_dn9)) * locals.var_a0__blk905) - (assign30230_e33005 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30230_e33007),)
    } else {
        (locals.var_q_x1sat__blk817, locals.var_q_x1sat__blk817_dn4, locals.var_q_x1sat__blk817_dn6, locals.var_q_x1sat__blk817_dn7, locals.var_q_x1sat__blk817_dn8, locals.var_q_x1sat__blk817_dn9,)
    }
};
        locals.var_q_x1sat__blk817 = assign30230_e33014;
        locals.var_q_x1sat__blk817_dn4 = assign30230_e33014_d_n4;
        locals.var_q_x1sat__blk817_dn6 = assign30230_e33014_d_n6;
        locals.var_q_x1sat__blk817_dn7 = assign30230_e33014_d_n7;
        locals.var_q_x1sat__blk817_dn8 = assign30230_e33014_d_n8;
        locals.var_q_x1sat__blk817_dn9 = assign30230_e33014_d_n9;

        let (assign30240_e33031, assign30240_e33031_d_n4, assign30240_e33031_d_n6, assign30240_e33031_d_n7, assign30240_e33031_d_n8, assign30240_e33031_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30240_e33019: f64 = (locals.var_k1__blk932 * locals.var_q_temp1__blk814);
        let assign30240_e33020: f64 = (locals.var_k2__blk933 + assign30240_e33019);
        let assign30240_e33022: f64 = (assign30240_e33020 * locals.var_diff_min__blk904);
        let assign30240_e33024: f64 = (assign30240_e33022 / locals.var_a0__blk905);
        let assign30240_e33025: f64 = (assign30240_e33024).ln();
        let assign30240_e33027: f64 = assign30240_e33025;
        let assign30240_e33029: f64 = (assign30240_e33027 + 3.0);
        (assign30240_e33029, (((((((locals.var_k2__blk933_dn4 + ((locals.var_k1__blk932_dn4 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn4))) * locals.var_diff_min__blk904) + (assign30240_e33020 * locals.var_diff_min__blk904_dn4)) * locals.var_a0__blk905) - (assign30240_e33022 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30240_e33024), (((((((locals.var_k2__blk933_dn6 + ((locals.var_k1__blk932_dn6 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn6))) * locals.var_diff_min__blk904) + (assign30240_e33020 * locals.var_diff_min__blk904_dn6)) * locals.var_a0__blk905) - (assign30240_e33022 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30240_e33024), (((((((locals.var_k2__blk933_dn7 + ((locals.var_k1__blk932_dn7 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn7))) * locals.var_diff_min__blk904) + (assign30240_e33020 * locals.var_diff_min__blk904_dn7)) * locals.var_a0__blk905) - (assign30240_e33022 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30240_e33024), (((((((locals.var_k2__blk933_dn8 + ((locals.var_k1__blk932_dn8 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn8))) * locals.var_diff_min__blk904) + (assign30240_e33020 * locals.var_diff_min__blk904_dn8)) * locals.var_a0__blk905) - (assign30240_e33022 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30240_e33024), (((((((locals.var_k2__blk933_dn9 + ((locals.var_k1__blk932_dn9 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn9))) * locals.var_diff_min__blk904) + (assign30240_e33020 * locals.var_diff_min__blk904_dn9)) * locals.var_a0__blk905) - (assign30240_e33022 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30240_e33024),)
    } else {
        (locals.var_q_x2sat__blk818, locals.var_q_x2sat__blk818_dn4, locals.var_q_x2sat__blk818_dn6, locals.var_q_x2sat__blk818_dn7, locals.var_q_x2sat__blk818_dn8, locals.var_q_x2sat__blk818_dn9,)
    }
};
        locals.var_q_x2sat__blk818 = assign30240_e33031;
        locals.var_q_x2sat__blk818_dn4 = assign30240_e33031_d_n4;
        locals.var_q_x2sat__blk818_dn6 = assign30240_e33031_d_n6;
        locals.var_q_x2sat__blk818_dn7 = assign30240_e33031_d_n7;
        locals.var_q_x2sat__blk818_dn8 = assign30240_e33031_d_n8;
        locals.var_q_x2sat__blk818_dn9 = assign30240_e33031_d_n9;

        let assign30250_e33034: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign30250_e33036: f64 = (assign30250_e33034 * 0.3333333333333);
        let assign30250_e33038: f64 = if assign30250_e33036 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1098 = assign30250_e33038;

        let (assign30260_e33052, assign30260_e33052_d_n4, assign30260_e33052_d_n6, assign30260_e33052_d_n7, assign30260_e33052_d_n8, assign30260_e33052_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1098 != 0.0)) {
        let assign30260_e33045: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign30260_e33047: f64 = (assign30260_e33045 * 0.3333333333333);
        let assign30260_e33048: f64 = (assign30260_e33047).exp();
        let assign30260_e33049: f64 = (1.0 + assign30260_e33048);
        let assign30260_e33050: f64 = (assign30260_e33049).ln();
        (assign30260_e33050, ((assign30260_e33048 * ((locals.var_q_x1sat__blk817_dn4 - locals.var_x1_wi0__blk908_dn4) * 0.3333333333333)) / assign30260_e33049), ((assign30260_e33048 * ((locals.var_q_x1sat__blk817_dn6 - locals.var_x1_wi0__blk908_dn6) * 0.3333333333333)) / assign30260_e33049), ((assign30260_e33048 * ((locals.var_q_x1sat__blk817_dn7 - locals.var_x1_wi0__blk908_dn7) * 0.3333333333333)) / assign30260_e33049), ((assign30260_e33048 * ((locals.var_q_x1sat__blk817_dn8 - locals.var_x1_wi0__blk908_dn8) * 0.3333333333333)) / assign30260_e33049), ((assign30260_e33048 * ((locals.var_q_x1sat__blk817_dn9 - locals.var_x1_wi0__blk908_dn9) * 0.3333333333333)) / assign30260_e33049),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30260_e33052;
        locals.var_q_temp3__blk816_dn4 = assign30260_e33052_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30260_e33052_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30260_e33052_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30260_e33052_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30260_e33052_d_n9;

        let (assign30270_e33063, assign30270_e33063_d_n4, assign30270_e33063_d_n6, assign30270_e33063_d_n7, assign30270_e33063_d_n8, assign30270_e33063_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1098 == 0.0)) {
        let assign30270_e33059: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign30270_e33061: f64 = (assign30270_e33059 * 0.3333333333333);
        (assign30270_e33061, ((locals.var_q_x1sat__blk817_dn4 - locals.var_x1_wi0__blk908_dn4) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn6 - locals.var_x1_wi0__blk908_dn6) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn7 - locals.var_x1_wi0__blk908_dn7) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn8 - locals.var_x1_wi0__blk908_dn8) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn9 - locals.var_x1_wi0__blk908_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30270_e33063;
        locals.var_q_temp3__blk816_dn4 = assign30270_e33063_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30270_e33063_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30270_e33063_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30270_e33063_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30270_e33063_d_n9;

        let (assign30280_e33071, assign30280_e33071_d_n4, assign30280_e33071_d_n6, assign30280_e33071_d_n7, assign30280_e33071_d_n8, assign30280_e33071_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30280_e33068: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign30280_e33069: f64 = (locals.var_q_x1sat__blk817 - assign30280_e33068);
        (assign30280_e33069, (locals.var_q_x1sat__blk817_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x1sat__blk817_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x1sat__blk817_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x1sat__blk817_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x1sat__blk817_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x1__blk821, locals.var_q_x1__blk821_dn4, locals.var_q_x1__blk821_dn6, locals.var_q_x1__blk821_dn7, locals.var_q_x1__blk821_dn8, locals.var_q_x1__blk821_dn9,)
    }
};
        locals.var_q_x1__blk821 = assign30280_e33071;
        locals.var_q_x1__blk821_dn4 = assign30280_e33071_d_n4;
        locals.var_q_x1__blk821_dn6 = assign30280_e33071_d_n6;
        locals.var_q_x1__blk821_dn7 = assign30280_e33071_d_n7;
        locals.var_q_x1__blk821_dn8 = assign30280_e33071_d_n8;
        locals.var_q_x1__blk821_dn9 = assign30280_e33071_d_n9;

        let assign30290_e33074: f64 = (locals.var_q_x2sat__blk818 - locals.var_x2_wi0__blk909);
        let assign30290_e33076: f64 = (assign30290_e33074 * 0.3333333333333);
        let assign30290_e33078: f64 = if assign30290_e33076 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1099 = assign30290_e33078;

        let (assign30300_e33092, assign30300_e33092_d_n4, assign30300_e33092_d_n6, assign30300_e33092_d_n7, assign30300_e33092_d_n8, assign30300_e33092_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1099 != 0.0)) {
        let assign30300_e33085: f64 = (locals.var_q_x2sat__blk818 - locals.var_x2_wi0__blk909);
        let assign30300_e33087: f64 = (assign30300_e33085 * 0.3333333333333);
        let assign30300_e33088: f64 = (assign30300_e33087).exp();
        let assign30300_e33089: f64 = (1.0 + assign30300_e33088);
        let assign30300_e33090: f64 = (assign30300_e33089).ln();
        (assign30300_e33090, ((assign30300_e33088 * ((locals.var_q_x2sat__blk818_dn4 - locals.var_x2_wi0__blk909_dn4) * 0.3333333333333)) / assign30300_e33089), ((assign30300_e33088 * ((locals.var_q_x2sat__blk818_dn6 - locals.var_x2_wi0__blk909_dn6) * 0.3333333333333)) / assign30300_e33089), ((assign30300_e33088 * ((locals.var_q_x2sat__blk818_dn7 - locals.var_x2_wi0__blk909_dn7) * 0.3333333333333)) / assign30300_e33089), ((assign30300_e33088 * ((locals.var_q_x2sat__blk818_dn8 - locals.var_x2_wi0__blk909_dn8) * 0.3333333333333)) / assign30300_e33089), ((assign30300_e33088 * ((locals.var_q_x2sat__blk818_dn9 - locals.var_x2_wi0__blk909_dn9) * 0.3333333333333)) / assign30300_e33089),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30300_e33092;
        locals.var_q_temp3__blk816_dn4 = assign30300_e33092_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30300_e33092_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30300_e33092_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30300_e33092_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30300_e33092_d_n9;

        let (assign30310_e33103, assign30310_e33103_d_n4, assign30310_e33103_d_n6, assign30310_e33103_d_n7, assign30310_e33103_d_n8, assign30310_e33103_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1099 == 0.0)) {
        let assign30310_e33099: f64 = (locals.var_q_x2sat__blk818 - locals.var_x2_wi0__blk909);
        let assign30310_e33101: f64 = (assign30310_e33099 * 0.3333333333333);
        (assign30310_e33101, ((locals.var_q_x2sat__blk818_dn4 - locals.var_x2_wi0__blk909_dn4) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn6 - locals.var_x2_wi0__blk909_dn6) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn7 - locals.var_x2_wi0__blk909_dn7) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn8 - locals.var_x2_wi0__blk909_dn8) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn9 - locals.var_x2_wi0__blk909_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30310_e33103;
        locals.var_q_temp3__blk816_dn4 = assign30310_e33103_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30310_e33103_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30310_e33103_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30310_e33103_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30310_e33103_d_n9;

        let (assign30320_e33111, assign30320_e33111_d_n4, assign30320_e33111_d_n6, assign30320_e33111_d_n7, assign30320_e33111_d_n8, assign30320_e33111_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30320_e33108: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign30320_e33109: f64 = (locals.var_q_x2sat__blk818 - assign30320_e33108);
        (assign30320_e33109, (locals.var_q_x2sat__blk818_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x2sat__blk818_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x2sat__blk818_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x2sat__blk818_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x2sat__blk818_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x2__blk822, locals.var_q_x2__blk822_dn4, locals.var_q_x2__blk822_dn6, locals.var_q_x2__blk822_dn7, locals.var_q_x2__blk822_dn8, locals.var_q_x2__blk822_dn9,)
    }
};
        locals.var_q_x2__blk822 = assign30320_e33111;
        locals.var_q_x2__blk822_dn4 = assign30320_e33111_d_n4;
        locals.var_q_x2__blk822_dn6 = assign30320_e33111_d_n6;
        locals.var_q_x2__blk822_dn7 = assign30320_e33111_d_n7;
        locals.var_q_x2__blk822_dn8 = assign30320_e33111_d_n8;
        locals.var_q_x2__blk822_dn9 = assign30320_e33111_d_n9;

        let (assign30330_e33121, assign30330_e33121_d_n4, assign30330_e33121_d_n6, assign30330_e33121_d_n7, assign30330_e33121_d_n8, assign30330_e33121_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30330_e33115: f64 = (locals.var_k1__blk932 * locals.var_xg1x__blk930);
        let assign30330_e33117: f64 = (assign30330_e33115 + locals.var_q_x2__blk822);
        let assign30330_e33119: f64 = (assign30330_e33117 * locals.var_q_temp1__blk814);
        (assign30330_e33119, (((((locals.var_k1__blk932_dn4 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn4)) + locals.var_q_x2__blk822_dn4) * locals.var_q_temp1__blk814) + (assign30330_e33117 * locals.var_q_temp1__blk814_dn4)), (((((locals.var_k1__blk932_dn6 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn6)) + locals.var_q_x2__blk822_dn6) * locals.var_q_temp1__blk814) + (assign30330_e33117 * locals.var_q_temp1__blk814_dn6)), (((((locals.var_k1__blk932_dn7 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn7)) + locals.var_q_x2__blk822_dn7) * locals.var_q_temp1__blk814) + (assign30330_e33117 * locals.var_q_temp1__blk814_dn7)), (((((locals.var_k1__blk932_dn8 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn8)) + locals.var_q_x2__blk822_dn8) * locals.var_q_temp1__blk814) + (assign30330_e33117 * locals.var_q_temp1__blk814_dn8)), (((((locals.var_k1__blk932_dn9 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn9)) + locals.var_q_x2__blk822_dn9) * locals.var_q_temp1__blk814) + (assign30330_e33117 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_x1_wi__blk819, locals.var_q_x1_wi__blk819_dn4, locals.var_q_x1_wi__blk819_dn6, locals.var_q_x1_wi__blk819_dn7, locals.var_q_x1_wi__blk819_dn8, locals.var_q_x1_wi__blk819_dn9,)
    }
};
        locals.var_q_x1_wi__blk819 = assign30330_e33121;
        locals.var_q_x1_wi__blk819_dn4 = assign30330_e33121_d_n4;
        locals.var_q_x1_wi__blk819_dn6 = assign30330_e33121_d_n6;
        locals.var_q_x1_wi__blk819_dn7 = assign30330_e33121_d_n7;
        locals.var_q_x1_wi__blk819_dn8 = assign30330_e33121_d_n8;
        locals.var_q_x1_wi__blk819_dn9 = assign30330_e33121_d_n9;

        let (assign30340_e33131, assign30340_e33131_d_n4, assign30340_e33131_d_n6, assign30340_e33131_d_n7, assign30340_e33131_d_n8, assign30340_e33131_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30340_e33125: f64 = (locals.var_k2__blk933 * locals.var_xg2x__blk931);
        let assign30340_e33127: f64 = (assign30340_e33125 + locals.var_q_x1__blk821);
        let assign30340_e33129: f64 = (assign30340_e33127 * locals.var_q_temp2__blk815);
        (assign30340_e33129, (((((locals.var_k2__blk933_dn4 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn4)) + locals.var_q_x1__blk821_dn4) * locals.var_q_temp2__blk815) + (assign30340_e33127 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_k2__blk933_dn6 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn6)) + locals.var_q_x1__blk821_dn6) * locals.var_q_temp2__blk815) + (assign30340_e33127 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_k2__blk933_dn7 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn7)) + locals.var_q_x1__blk821_dn7) * locals.var_q_temp2__blk815) + (assign30340_e33127 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_k2__blk933_dn8 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn8)) + locals.var_q_x1__blk821_dn8) * locals.var_q_temp2__blk815) + (assign30340_e33127 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_k2__blk933_dn9 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn9)) + locals.var_q_x1__blk821_dn9) * locals.var_q_temp2__blk815) + (assign30340_e33127 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_x2_wi__blk820, locals.var_q_x2_wi__blk820_dn4, locals.var_q_x2_wi__blk820_dn6, locals.var_q_x2_wi__blk820_dn7, locals.var_q_x2_wi__blk820_dn8, locals.var_q_x2_wi__blk820_dn9,)
    }
};
        locals.var_q_x2_wi__blk820 = assign30340_e33131;
        locals.var_q_x2_wi__blk820_dn4 = assign30340_e33131_d_n4;
        locals.var_q_x2_wi__blk820_dn6 = assign30340_e33131_d_n6;
        locals.var_q_x2_wi__blk820_dn7 = assign30340_e33131_d_n7;
        locals.var_q_x2_wi__blk820_dn8 = assign30340_e33131_d_n8;
        locals.var_q_x2_wi__blk820_dn9 = assign30340_e33131_d_n9;

    }

    pub(super) fn stamp_transient_block_78(
        locals: &mut StampLocals,
    ) {
        let assign30350_e33134: f64 = (locals.var_q_x1sat__blk817 - locals.var_q_x1_wi__blk819);
        let assign30350_e33136: f64 = (assign30350_e33134 * 0.3333333333333);
        let assign30350_e33138: f64 = if assign30350_e33136 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1100 = assign30350_e33138;

        let (assign30360_e33152, assign30360_e33152_d_n4, assign30360_e33152_d_n6, assign30360_e33152_d_n7, assign30360_e33152_d_n8, assign30360_e33152_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1100 != 0.0)) {
        let assign30360_e33145: f64 = (locals.var_q_x1sat__blk817 - locals.var_q_x1_wi__blk819);
        let assign30360_e33147: f64 = (assign30360_e33145 * 0.3333333333333);
        let assign30360_e33148: f64 = (assign30360_e33147).exp();
        let assign30360_e33149: f64 = (1.0 + assign30360_e33148);
        let assign30360_e33150: f64 = (assign30360_e33149).ln();
        (assign30360_e33150, ((assign30360_e33148 * ((locals.var_q_x1sat__blk817_dn4 - locals.var_q_x1_wi__blk819_dn4) * 0.3333333333333)) / assign30360_e33149), ((assign30360_e33148 * ((locals.var_q_x1sat__blk817_dn6 - locals.var_q_x1_wi__blk819_dn6) * 0.3333333333333)) / assign30360_e33149), ((assign30360_e33148 * ((locals.var_q_x1sat__blk817_dn7 - locals.var_q_x1_wi__blk819_dn7) * 0.3333333333333)) / assign30360_e33149), ((assign30360_e33148 * ((locals.var_q_x1sat__blk817_dn8 - locals.var_q_x1_wi__blk819_dn8) * 0.3333333333333)) / assign30360_e33149), ((assign30360_e33148 * ((locals.var_q_x1sat__blk817_dn9 - locals.var_q_x1_wi__blk819_dn9) * 0.3333333333333)) / assign30360_e33149),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30360_e33152;
        locals.var_q_temp3__blk816_dn4 = assign30360_e33152_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30360_e33152_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30360_e33152_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30360_e33152_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30360_e33152_d_n9;

        let (assign30370_e33163, assign30370_e33163_d_n4, assign30370_e33163_d_n6, assign30370_e33163_d_n7, assign30370_e33163_d_n8, assign30370_e33163_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1100 == 0.0)) {
        let assign30370_e33159: f64 = (locals.var_q_x1sat__blk817 - locals.var_q_x1_wi__blk819);
        let assign30370_e33161: f64 = (assign30370_e33159 * 0.3333333333333);
        (assign30370_e33161, ((locals.var_q_x1sat__blk817_dn4 - locals.var_q_x1_wi__blk819_dn4) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn6 - locals.var_q_x1_wi__blk819_dn6) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn7 - locals.var_q_x1_wi__blk819_dn7) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn8 - locals.var_q_x1_wi__blk819_dn8) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn9 - locals.var_q_x1_wi__blk819_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30370_e33163;
        locals.var_q_temp3__blk816_dn4 = assign30370_e33163_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30370_e33163_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30370_e33163_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30370_e33163_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30370_e33163_d_n9;

        let (assign30380_e33171, assign30380_e33171_d_n4, assign30380_e33171_d_n6, assign30380_e33171_d_n7, assign30380_e33171_d_n8, assign30380_e33171_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30380_e33168: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign30380_e33169: f64 = (locals.var_q_x1sat__blk817 - assign30380_e33168);
        (assign30380_e33169, (locals.var_q_x1sat__blk817_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x1sat__blk817_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x1sat__blk817_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x1sat__blk817_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x1sat__blk817_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x1__blk821, locals.var_q_x1__blk821_dn4, locals.var_q_x1__blk821_dn6, locals.var_q_x1__blk821_dn7, locals.var_q_x1__blk821_dn8, locals.var_q_x1__blk821_dn9,)
    }
};
        locals.var_q_x1__blk821 = assign30380_e33171;
        locals.var_q_x1__blk821_dn4 = assign30380_e33171_d_n4;
        locals.var_q_x1__blk821_dn6 = assign30380_e33171_d_n6;
        locals.var_q_x1__blk821_dn7 = assign30380_e33171_d_n7;
        locals.var_q_x1__blk821_dn8 = assign30380_e33171_d_n8;
        locals.var_q_x1__blk821_dn9 = assign30380_e33171_d_n9;

        let assign30390_e33174: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign30390_e33176: f64 = (assign30390_e33174 * 0.3333333333333);
        let assign30390_e33178: f64 = if assign30390_e33176 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1101 = assign30390_e33178;

        let (assign30400_e33192, assign30400_e33192_d_n4, assign30400_e33192_d_n6, assign30400_e33192_d_n7, assign30400_e33192_d_n8, assign30400_e33192_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1101 != 0.0)) {
        let assign30400_e33185: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign30400_e33187: f64 = (assign30400_e33185 * 0.3333333333333);
        let assign30400_e33188: f64 = (assign30400_e33187).exp();
        let assign30400_e33189: f64 = (1.0 + assign30400_e33188);
        let assign30400_e33190: f64 = (assign30400_e33189).ln();
        (assign30400_e33190, ((assign30400_e33188 * ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) * 0.3333333333333)) / assign30400_e33189), ((assign30400_e33188 * ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) * 0.3333333333333)) / assign30400_e33189), ((assign30400_e33188 * ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) * 0.3333333333333)) / assign30400_e33189), ((assign30400_e33188 * ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) * 0.3333333333333)) / assign30400_e33189), ((assign30400_e33188 * ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) * 0.3333333333333)) / assign30400_e33189),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30400_e33192;
        locals.var_q_temp3__blk816_dn4 = assign30400_e33192_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30400_e33192_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30400_e33192_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30400_e33192_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30400_e33192_d_n9;

        let (assign30410_e33203, assign30410_e33203_d_n4, assign30410_e33203_d_n6, assign30410_e33203_d_n7, assign30410_e33203_d_n8, assign30410_e33203_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1101 == 0.0)) {
        let assign30410_e33199: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign30410_e33201: f64 = (assign30410_e33199 * 0.3333333333333);
        (assign30410_e33201, ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30410_e33203;
        locals.var_q_temp3__blk816_dn4 = assign30410_e33203_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30410_e33203_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30410_e33203_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30410_e33203_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30410_e33203_d_n9;

        let (assign30420_e33211, assign30420_e33211_d_n4, assign30420_e33211_d_n6, assign30420_e33211_d_n7, assign30420_e33211_d_n8, assign30420_e33211_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30420_e33208: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign30420_e33209: f64 = (locals.var_q_x2sat__blk818 - assign30420_e33208);
        (assign30420_e33209, (locals.var_q_x2sat__blk818_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x2sat__blk818_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x2sat__blk818_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x2sat__blk818_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x2sat__blk818_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x2__blk822, locals.var_q_x2__blk822_dn4, locals.var_q_x2__blk822_dn6, locals.var_q_x2__blk822_dn7, locals.var_q_x2__blk822_dn8, locals.var_q_x2__blk822_dn9,)
    }
};
        locals.var_q_x2__blk822 = assign30420_e33211;
        locals.var_q_x2__blk822_dn4 = assign30420_e33211_d_n4;
        locals.var_q_x2__blk822_dn6 = assign30420_e33211_d_n6;
        locals.var_q_x2__blk822_dn7 = assign30420_e33211_d_n7;
        locals.var_q_x2__blk822_dn8 = assign30420_e33211_d_n8;
        locals.var_q_x2__blk822_dn9 = assign30420_e33211_d_n9;

        let (assign30430_e33217, assign30430_e33217_d_n4, assign30430_e33217_d_n6, assign30430_e33217_d_n7, assign30430_e33217_d_n8, assign30430_e33217_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30430_e33215: f64 = (locals.var_xg1x__blk930 - locals.var_q_x1__blk821);
        (assign30430_e33215, (locals.var_xg1x__blk930_dn4 - locals.var_q_x1__blk821_dn4), (locals.var_xg1x__blk930_dn6 - locals.var_q_x1__blk821_dn6), (locals.var_xg1x__blk930_dn7 - locals.var_q_x1__blk821_dn7), (locals.var_xg1x__blk930_dn8 - locals.var_q_x1__blk821_dn8), (locals.var_xg1x__blk930_dn9 - locals.var_q_x1__blk821_dn9),)
    } else {
        (locals.var_q1s__blk937, locals.var_q1s__blk937_dn4, locals.var_q1s__blk937_dn6, locals.var_q1s__blk937_dn7, locals.var_q1s__blk937_dn8, locals.var_q1s__blk937_dn9,)
    }
};
        locals.var_q1s__blk937 = assign30430_e33217;
        locals.var_q1s__blk937_dn4 = assign30430_e33217_d_n4;
        locals.var_q1s__blk937_dn6 = assign30430_e33217_d_n6;
        locals.var_q1s__blk937_dn7 = assign30430_e33217_d_n7;
        locals.var_q1s__blk937_dn8 = assign30430_e33217_d_n8;
        locals.var_q1s__blk937_dn9 = assign30430_e33217_d_n9;

        let (assign30440_e33223, assign30440_e33223_d_n4, assign30440_e33223_d_n6, assign30440_e33223_d_n7, assign30440_e33223_d_n8, assign30440_e33223_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30440_e33221: f64 = (locals.var_xg2x__blk931 - locals.var_q_x2__blk822);
        (assign30440_e33221, (locals.var_xg2x__blk931_dn4 - locals.var_q_x2__blk822_dn4), (locals.var_xg2x__blk931_dn6 - locals.var_q_x2__blk822_dn6), (locals.var_xg2x__blk931_dn7 - locals.var_q_x2__blk822_dn7), (locals.var_xg2x__blk931_dn8 - locals.var_q_x2__blk822_dn8), (locals.var_xg2x__blk931_dn9 - locals.var_q_x2__blk822_dn9),)
    } else {
        (locals.var_q2s__blk941, locals.var_q2s__blk941_dn4, locals.var_q2s__blk941_dn6, locals.var_q2s__blk941_dn7, locals.var_q2s__blk941_dn8, locals.var_q2s__blk941_dn9,)
    }
};
        locals.var_q2s__blk941 = assign30440_e33223;
        locals.var_q2s__blk941_dn4 = assign30440_e33223_d_n4;
        locals.var_q2s__blk941_dn6 = assign30440_e33223_d_n6;
        locals.var_q2s__blk941_dn7 = assign30440_e33223_d_n7;
        locals.var_q2s__blk941_dn8 = assign30440_e33223_d_n8;
        locals.var_q2s__blk941_dn9 = assign30440_e33223_d_n9;

        let (assign30450_e33227, assign30450_e33227_d_n4, assign30450_e33227_d_n6, assign30450_e33227_d_n7, assign30450_e33227_d_n8, assign30450_e33227_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign30450_e33227;
        locals.var_q_rac_qsq__blk828_dn4 = assign30450_e33227_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign30450_e33227_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign30450_e33227_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign30450_e33227_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign30450_e33227_d_n9;

        let (assign30460_e33231, assign30460_e33231_d_n4, assign30460_e33231_d_n6, assign30460_e33231_d_n7, assign30460_e33231_d_n8, assign30460_e33231_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign30460_e33231;
        locals.var_q_invexpq__blk831_dn4 = assign30460_e33231_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign30460_e33231_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign30460_e33231_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign30460_e33231_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign30460_e33231_d_n9;

        let (assign30470_e33237, assign30470_e33237_d_n4, assign30470_e33237_d_n6, assign30470_e33237_d_n7, assign30470_e33237_d_n8, assign30470_e33237_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30470_e33235: f64 = (locals.var_k1__blk932 * locals.var_q1s__blk937);
        (assign30470_e33235, ((locals.var_k1__blk932_dn4 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign30470_e33237;
        locals.var_q_k1q1__blk823_dn4 = assign30470_e33237_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign30470_e33237_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign30470_e33237_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign30470_e33237_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign30470_e33237_d_n9;

        let assign30480_e33240: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign30480_e33242: f64 = assign30480_e33240;
        let assign30480_e33244: f64 = if assign30480_e33242 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1102 = assign30480_e33244;

        let (assign30490_e33255, assign30490_e33255_d_n4, assign30490_e33255_d_n6, assign30490_e33255_d_n7, assign30490_e33255_d_n8, assign30490_e33255_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1102 != 0.0)) {
        let assign30490_e33250: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign30490_e33252: f64 = assign30490_e33250;
        let assign30490_e33253: f64 = (assign30490_e33252).exp();
        (assign30490_e33253, (assign30490_e33253 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)), (assign30490_e33253 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)), (assign30490_e33253 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)), (assign30490_e33253 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)), (assign30490_e33253 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30490_e33255;
        locals.var_q_temp1__blk814_dn4 = assign30490_e33255_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30490_e33255_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30490_e33255_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30490_e33255_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30490_e33255_d_n9;

        let (assign30500_e33296, assign30500_e33296_d_n4, assign30500_e33296_d_n6, assign30500_e33296_d_n7, assign30500_e33296_d_n8, assign30500_e33296_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1102 == 0.0)) {
        let assign30500_e33264: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign30500_e33266: f64 = assign30500_e33264;
        let assign30500_e33268: f64 = (assign30500_e33266 - 80.0);
        let assign30500_e33273: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign30500_e33275: f64 = assign30500_e33273;
        let assign30500_e33277: f64 = (assign30500_e33275 - 80.0);
        let assign30500_e33278: f64 = (0.5 * assign30500_e33277);
        let assign30500_e33282: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign30500_e33284: f64 = assign30500_e33282;
        let assign30500_e33286: f64 = (assign30500_e33284 - 80.0);
        let assign30500_e33288: f64 = (assign30500_e33286 * 0.3333333333333);
        let assign30500_e33289: f64 = (1.0 + assign30500_e33288);
        let assign30500_e33290: f64 = (assign30500_e33278 * assign30500_e33289);
        let assign30500_e33291: f64 = (1.0 + assign30500_e33290);
        let assign30500_e33292: f64 = (assign30500_e33268 * assign30500_e33291);
        let assign30500_e33293: f64 = (1.0 + assign30500_e33292);
        let assign30500_e33294: f64 = (5.54062e34 * assign30500_e33293);
        (assign30500_e33294, (5.54062e34 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * assign30500_e33291) + (assign30500_e33268 * (((0.5 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)) * assign30500_e33289) + (assign30500_e33278 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * assign30500_e33291) + (assign30500_e33268 * (((0.5 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)) * assign30500_e33289) + (assign30500_e33278 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * assign30500_e33291) + (assign30500_e33268 * (((0.5 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)) * assign30500_e33289) + (assign30500_e33278 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * assign30500_e33291) + (assign30500_e33268 * (((0.5 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)) * assign30500_e33289) + (assign30500_e33278 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * assign30500_e33291) + (assign30500_e33268 * (((0.5 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)) * assign30500_e33289) + (assign30500_e33278 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30500_e33296;
        locals.var_q_temp1__blk814_dn4 = assign30500_e33296_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30500_e33296_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30500_e33296_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30500_e33296_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30500_e33296_d_n9;

        let (assign30510_e33302, assign30510_e33302_d_n4, assign30510_e33302_d_n6, assign30510_e33302_d_n7, assign30510_e33302_d_n8, assign30510_e33302_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30510_e33300: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign30510_e33300, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign30510_e33302;
        locals.var_q_aexp__blk824_dn4 = assign30510_e33302_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign30510_e33302_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign30510_e33302_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign30510_e33302_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign30510_e33302_d_n9;

        let (assign30520_e33310, assign30520_e33310_d_n4, assign30520_e33310_d_n6, assign30520_e33310_d_n7, assign30520_e33310_d_n8, assign30520_e33310_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30520_e33306: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign30520_e33308: f64 = (assign30520_e33306 - locals.var_q_aexp__blk824);
        (assign30520_e33308, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign30520_e33310;
        locals.var_q_qsq__blk825_dn4 = assign30520_e33310_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign30520_e33310_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign30520_e33310_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign30520_e33310_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign30520_e33310_d_n9;

        let (assign30530_e33320, assign30530_e33320_d_n4, assign30530_e33320_d_n6, assign30530_e33320_d_n7, assign30530_e33320_d_n8, assign30530_e33320_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30530_e33314: f64 = (2.0 * locals.var_k1__blk932);
        let assign30530_e33316: f64 = (assign30530_e33314 * locals.var_q_k1q1__blk823);
        let assign30530_e33318: f64 = (assign30530_e33316 + locals.var_q_aexp__blk824);
        (assign30530_e33318, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign30530_e33314 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign30530_e33314 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign30530_e33314 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign30530_e33314 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign30530_e33314 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign30530_e33320;
        locals.var_q_d1_qsq__blk826_dn4 = assign30530_e33320_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign30530_e33320_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign30530_e33320_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign30530_e33320_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign30530_e33320_d_n9;

        let (assign30540_e33330, assign30540_e33330_d_n4, assign30540_e33330_d_n6, assign30540_e33330_d_n7, assign30540_e33330_d_n8, assign30540_e33330_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30540_e33324: f64 = (2.0 * locals.var_k1__blk932);
        let assign30540_e33326: f64 = (assign30540_e33324 * locals.var_k1__blk932);
        let assign30540_e33328: f64 = (assign30540_e33326 - locals.var_q_aexp__blk824);
        (assign30540_e33328, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign30540_e33324 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign30540_e33324 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign30540_e33324 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign30540_e33324 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign30540_e33324 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign30540_e33330;
        locals.var_q_d2_qsq__blk827_dn4 = assign30540_e33330_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign30540_e33330_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign30540_e33330_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign30540_e33330_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign30540_e33330_d_n9;

        let assign30550_e33333: f64 = (-0.005);
        let assign30550_e33334: f64 = if locals.var_q_qsq__blk825 < assign30550_e33333 { 1.0 } else { 0.0 };
        locals.var_guard1103 = assign30550_e33334;

        let (assign30560_e33342, assign30560_e33342_d_n4, assign30560_e33342_d_n6, assign30560_e33342_d_n7, assign30560_e33342_d_n8, assign30560_e33342_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30560_e33339: f64 = (locals.var_q_qsq__blk825).abs();
        let assign30560_e33340: f64 = (assign30560_e33339).sqrt();
        (assign30560_e33340, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign30560_e33340)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign30560_e33340)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign30560_e33340)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign30560_e33340)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign30560_e33340)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign30560_e33342;
        locals.var_q_rac_qsq__blk828_dn4 = assign30560_e33342_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign30560_e33342_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign30560_e33342_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign30560_e33342_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign30560_e33342_d_n9;

        let (assign30570_e33353, assign30570_e33353_d_n4, assign30570_e33353_d_n6, assign30570_e33353_d_n7, assign30570_e33353_d_n8, assign30570_e33353_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30570_e33349: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign30570_e33350: f64 = (assign30570_e33349).tan();
        let assign30570_e33351: f64 = (locals.var_q_rac_qsq__blk828 / assign30570_e33350);
        (assign30570_e33351, (((locals.var_q_rac_qsq__blk828_dn4 * assign30570_e33350) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign30570_e33349).cos() * (assign30570_e33349).cos())))) / (assign30570_e33350 * assign30570_e33350)), (((locals.var_q_rac_qsq__blk828_dn6 * assign30570_e33350) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign30570_e33349).cos() * (assign30570_e33349).cos())))) / (assign30570_e33350 * assign30570_e33350)), (((locals.var_q_rac_qsq__blk828_dn7 * assign30570_e33350) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign30570_e33349).cos() * (assign30570_e33349).cos())))) / (assign30570_e33350 * assign30570_e33350)), (((locals.var_q_rac_qsq__blk828_dn8 * assign30570_e33350) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign30570_e33349).cos() * (assign30570_e33349).cos())))) / (assign30570_e33350 * assign30570_e33350)), (((locals.var_q_rac_qsq__blk828_dn9 * assign30570_e33350) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign30570_e33349).cos() * (assign30570_e33349).cos())))) / (assign30570_e33350 * assign30570_e33350)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign30570_e33353;
        locals.var_q_qcoth__blk829_dn4 = assign30570_e33353_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign30570_e33353_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign30570_e33353_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign30570_e33353_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign30570_e33353_d_n9;

        let (assign30580_e33363, assign30580_e33363_d_n4, assign30580_e33363_d_n6, assign30580_e33363_d_n7, assign30580_e33363_d_n8, assign30580_e33363_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30580_e33359: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign30580_e33361: f64 = (assign30580_e33359 / locals.var_q_qsq__blk825);
        (assign30580_e33361, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign30580_e33359 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign30580_e33359 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign30580_e33359 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign30580_e33359 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign30580_e33359 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30580_e33363;
        locals.var_q_temp1__blk814_dn4 = assign30580_e33363_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30580_e33363_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30580_e33363_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30580_e33363_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30580_e33363_d_n9;

        let (assign30590_e33377, assign30590_e33377_d_n4, assign30590_e33377_d_n6, assign30590_e33377_d_n7, assign30590_e33377_d_n8, assign30590_e33377_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30590_e33371: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign30590_e33372: f64 = (locals.var_q_qcoth__blk829 * assign30590_e33371);
        let assign30590_e33373: f64 = (locals.var_q_qsq__blk825 + assign30590_e33372);
        let assign30590_e33375: f64 = (assign30590_e33373 * locals.var_q_temp1__blk814);
        (assign30590_e33375, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign30590_e33371) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign30590_e33373 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign30590_e33371) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign30590_e33373 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign30590_e33371) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign30590_e33373 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign30590_e33371) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign30590_e33373 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign30590_e33371) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign30590_e33373 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign30590_e33377;
        locals.var_q_d1_qcoth__blk830_dn4 = assign30590_e33377_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign30590_e33377_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign30590_e33377_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign30590_e33377_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign30590_e33377_d_n9;

        let (assign30600_e33399, assign30600_e33399_d_n4, assign30600_e33399_d_n6, assign30600_e33399_d_n7, assign30600_e33399_d_n8, assign30600_e33399_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30600_e33384: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign30600_e33387: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign30600_e33388: f64 = (assign30600_e33384 * assign30600_e33387);
        let assign30600_e33389: f64 = (locals.var_q_d1_qsq__blk826 - assign30600_e33388);
        let assign30600_e33391: f64 = (assign30600_e33389 * locals.var_q_temp1__blk814);
        let assign30600_e33394: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign30600_e33396: f64 = (assign30600_e33394 / locals.var_q_d1_qsq__blk826);
        let assign30600_e33397: f64 = (assign30600_e33391 + assign30600_e33396);
        (assign30600_e33397, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign30600_e33387) + (assign30600_e33384 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign30600_e33389 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign30600_e33394 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign30600_e33387) + (assign30600_e33384 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign30600_e33389 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign30600_e33394 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign30600_e33387) + (assign30600_e33384 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign30600_e33389 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign30600_e33394 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign30600_e33387) + (assign30600_e33384 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign30600_e33389 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign30600_e33394 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign30600_e33387) + (assign30600_e33384 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign30600_e33389 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign30600_e33394 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign30600_e33399;
        locals.var_q_d2_qcoth__blk832_dn4 = assign30600_e33399_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign30600_e33399_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign30600_e33399_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign30600_e33399_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign30600_e33399_d_n9;

        let (assign30610_e33409, assign30610_e33409_d_n4, assign30610_e33409_d_n6, assign30610_e33409_d_n7, assign30610_e33409_d_n8, assign30610_e33409_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30610_e33406: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign30610_e33407: f64 = (1.0 - assign30610_e33406);
        (assign30610_e33407, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30610_e33409;
        locals.var_q_temp2__blk815_dn4 = assign30610_e33409_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30610_e33409_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30610_e33409_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30610_e33409_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30610_e33409_d_n9;

        let (assign30620_e33419, assign30620_e33419_d_n4, assign30620_e33419_d_n6, assign30620_e33419_d_n7, assign30620_e33419_d_n8, assign30620_e33419_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30620_e33415: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign30620_e33417: f64 = (assign30620_e33415 * locals.var_q_temp2__blk815);
        (assign30620_e33417, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30620_e33415 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30620_e33415 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30620_e33415 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30620_e33415 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30620_e33415 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign30620_e33419;
        locals.var_q_d1_ln__blk835_dn4 = assign30620_e33419_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign30620_e33419_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign30620_e33419_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign30620_e33419_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign30620_e33419_d_n9;

        let (assign30630_e33437, assign30630_e33437_d_n4, assign30630_e33437_d_n6, assign30630_e33437_d_n7, assign30630_e33437_d_n8, assign30630_e33437_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30630_e33425: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign30630_e33430: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign30630_e33431: f64 = (locals.var_q_d1_ln__blk835 + assign30630_e33430);
        let assign30630_e33432: f64 = (locals.var_q_d1_qsq__blk826 * assign30630_e33431);
        let assign30630_e33433: f64 = (assign30630_e33425 - assign30630_e33432);
        let assign30630_e33435: f64 = (assign30630_e33433 / locals.var_q_qsq__blk825);
        (assign30630_e33435, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign30630_e33431) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign30630_e33433 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign30630_e33431) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign30630_e33433 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign30630_e33431) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign30630_e33433 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign30630_e33431) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign30630_e33433 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign30630_e33431) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign30630_e33433 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign30630_e33437;
        locals.var_q_d2_ln__blk836_dn4 = assign30630_e33437_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign30630_e33437_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign30630_e33437_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign30630_e33437_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign30630_e33437_d_n9;

        let assign30640_e33440: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1104 = assign30640_e33440;

        let (assign30650_e33451, assign30650_e33451_d_n4, assign30650_e33451_d_n6, assign30650_e33451_d_n7, assign30650_e33451_d_n8, assign30650_e33451_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30650_e33448: f64 = (locals.var_q_qsq__blk825).abs();
        let assign30650_e33449: f64 = (assign30650_e33448).sqrt();
        (assign30650_e33449, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign30650_e33449)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign30650_e33449)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign30650_e33449)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign30650_e33449)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign30650_e33449)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign30650_e33451;
        locals.var_q_rac_qsq__blk828_dn4 = assign30650_e33451_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign30650_e33451_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign30650_e33451_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign30650_e33451_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign30650_e33451_d_n9;

        let (assign30660_e33462, assign30660_e33462_d_n4, assign30660_e33462_d_n6, assign30660_e33462_d_n7, assign30660_e33462_d_n8, assign30660_e33462_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30660_e33459: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign30660_e33460: f64 = (assign30660_e33459).exp();
        (assign30660_e33460, (assign30660_e33460 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign30660_e33460 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign30660_e33460 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign30660_e33460 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign30660_e33460 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign30660_e33462;
        locals.var_q_invexpq__blk831_dn4 = assign30660_e33462_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign30660_e33462_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign30660_e33462_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign30660_e33462_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign30660_e33462_d_n9;

        let (assign30670_e33479, assign30670_e33479_d_n4, assign30670_e33479_d_n6, assign30670_e33479_d_n7, assign30670_e33479_d_n8, assign30670_e33479_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30670_e33472: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign30670_e33473: f64 = (locals.var_q_rac_qsq__blk828 * assign30670_e33472);
        let assign30670_e33476: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign30670_e33477: f64 = (assign30670_e33473 / assign30670_e33476);
        (assign30670_e33477, (((((locals.var_q_rac_qsq__blk828_dn4 * assign30670_e33472) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign30670_e33476) - (assign30670_e33473 * (-locals.var_q_invexpq__blk831_dn4))) / (assign30670_e33476 * assign30670_e33476)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign30670_e33472) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign30670_e33476) - (assign30670_e33473 * (-locals.var_q_invexpq__blk831_dn6))) / (assign30670_e33476 * assign30670_e33476)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign30670_e33472) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign30670_e33476) - (assign30670_e33473 * (-locals.var_q_invexpq__blk831_dn7))) / (assign30670_e33476 * assign30670_e33476)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign30670_e33472) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign30670_e33476) - (assign30670_e33473 * (-locals.var_q_invexpq__blk831_dn8))) / (assign30670_e33476 * assign30670_e33476)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign30670_e33472) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign30670_e33476) - (assign30670_e33473 * (-locals.var_q_invexpq__blk831_dn9))) / (assign30670_e33476 * assign30670_e33476)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign30670_e33479;
        locals.var_q_qcoth__blk829_dn4 = assign30670_e33479_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign30670_e33479_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign30670_e33479_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign30670_e33479_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign30670_e33479_d_n9;

    }

    pub(super) fn stamp_transient_block_79(
        locals: &mut StampLocals,
    ) {
        let (assign30680_e33492, assign30680_e33492_d_n4, assign30680_e33492_d_n6, assign30680_e33492_d_n7, assign30680_e33492_d_n8, assign30680_e33492_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30680_e33488: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign30680_e33490: f64 = (assign30680_e33488 / locals.var_q_qsq__blk825);
        (assign30680_e33490, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign30680_e33488 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign30680_e33488 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign30680_e33488 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign30680_e33488 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign30680_e33488 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30680_e33492;
        locals.var_q_temp1__blk814_dn4 = assign30680_e33492_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30680_e33492_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30680_e33492_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30680_e33492_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30680_e33492_d_n9;

        let (assign30690_e33509, assign30690_e33509_d_n4, assign30690_e33509_d_n6, assign30690_e33509_d_n7, assign30690_e33509_d_n8, assign30690_e33509_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30690_e33503: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign30690_e33504: f64 = (locals.var_q_qcoth__blk829 * assign30690_e33503);
        let assign30690_e33505: f64 = (locals.var_q_qsq__blk825 + assign30690_e33504);
        let assign30690_e33507: f64 = (assign30690_e33505 * locals.var_q_temp1__blk814);
        (assign30690_e33507, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign30690_e33503) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign30690_e33505 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign30690_e33503) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign30690_e33505 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign30690_e33503) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign30690_e33505 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign30690_e33503) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign30690_e33505 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign30690_e33503) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign30690_e33505 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign30690_e33509;
        locals.var_q_d1_qcoth__blk830_dn4 = assign30690_e33509_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign30690_e33509_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign30690_e33509_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign30690_e33509_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign30690_e33509_d_n9;

        let (assign30700_e33534, assign30700_e33534_d_n4, assign30700_e33534_d_n6, assign30700_e33534_d_n7, assign30700_e33534_d_n8, assign30700_e33534_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30700_e33519: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign30700_e33522: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign30700_e33523: f64 = (assign30700_e33519 * assign30700_e33522);
        let assign30700_e33524: f64 = (locals.var_q_d1_qsq__blk826 - assign30700_e33523);
        let assign30700_e33526: f64 = (assign30700_e33524 * locals.var_q_temp1__blk814);
        let assign30700_e33529: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign30700_e33531: f64 = (assign30700_e33529 / locals.var_q_d1_qsq__blk826);
        let assign30700_e33532: f64 = (assign30700_e33526 + assign30700_e33531);
        (assign30700_e33532, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign30700_e33522) + (assign30700_e33519 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign30700_e33524 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign30700_e33529 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign30700_e33522) + (assign30700_e33519 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign30700_e33524 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign30700_e33529 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign30700_e33522) + (assign30700_e33519 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign30700_e33524 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign30700_e33529 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign30700_e33522) + (assign30700_e33519 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign30700_e33524 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign30700_e33529 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign30700_e33522) + (assign30700_e33519 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign30700_e33524 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign30700_e33529 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign30700_e33534;
        locals.var_q_d2_qcoth__blk832_dn4 = assign30700_e33534_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign30700_e33534_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign30700_e33534_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign30700_e33534_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign30700_e33534_d_n9;

        let (assign30710_e33547, assign30710_e33547_d_n4, assign30710_e33547_d_n6, assign30710_e33547_d_n7, assign30710_e33547_d_n8, assign30710_e33547_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30710_e33544: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign30710_e33545: f64 = (1.0 - assign30710_e33544);
        (assign30710_e33545, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30710_e33547;
        locals.var_q_temp2__blk815_dn4 = assign30710_e33547_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30710_e33547_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30710_e33547_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30710_e33547_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30710_e33547_d_n9;

        let (assign30720_e33560, assign30720_e33560_d_n4, assign30720_e33560_d_n6, assign30720_e33560_d_n7, assign30720_e33560_d_n8, assign30720_e33560_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30720_e33556: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign30720_e33558: f64 = (assign30720_e33556 * locals.var_q_temp2__blk815);
        (assign30720_e33558, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30720_e33556 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30720_e33556 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30720_e33556 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30720_e33556 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30720_e33556 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign30720_e33560;
        locals.var_q_d1_ln__blk835_dn4 = assign30720_e33560_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign30720_e33560_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign30720_e33560_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign30720_e33560_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign30720_e33560_d_n9;

        let (assign30730_e33581, assign30730_e33581_d_n4, assign30730_e33581_d_n6, assign30730_e33581_d_n7, assign30730_e33581_d_n8, assign30730_e33581_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30730_e33569: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign30730_e33574: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign30730_e33575: f64 = (locals.var_q_d1_ln__blk835 + assign30730_e33574);
        let assign30730_e33576: f64 = (locals.var_q_d1_qsq__blk826 * assign30730_e33575);
        let assign30730_e33577: f64 = (assign30730_e33569 - assign30730_e33576);
        let assign30730_e33579: f64 = (assign30730_e33577 / locals.var_q_qsq__blk825);
        (assign30730_e33579, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign30730_e33575) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign30730_e33577 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign30730_e33575) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign30730_e33577 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign30730_e33575) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign30730_e33577 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign30730_e33575) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign30730_e33577 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign30730_e33575) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign30730_e33577 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign30730_e33581;
        locals.var_q_d2_ln__blk836_dn4 = assign30730_e33581_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign30730_e33581_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign30730_e33581_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign30730_e33581_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign30730_e33581_d_n9;

        let (assign30740_e33609, assign30740_e33609_d_n4, assign30740_e33609_d_n6, assign30740_e33609_d_n7, assign30740_e33609_d_n8, assign30740_e33609_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30740_e33593: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign30740_e33597: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign30740_e33601: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign30740_e33602: f64 = (1.0 - assign30740_e33601);
        let assign30740_e33603: f64 = (assign30740_e33597 * assign30740_e33602);
        let assign30740_e33604: f64 = (1.0 - assign30740_e33603);
        let assign30740_e33605: f64 = (assign30740_e33593 * assign30740_e33604);
        let assign30740_e33606: f64 = (1.0 - assign30740_e33605);
        let assign30740_e33607: f64 = (0.1666666666667 * assign30740_e33606);
        (assign30740_e33607, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign30740_e33604) + (assign30740_e33593 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign30740_e33602) + (assign30740_e33597 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign30740_e33604) + (assign30740_e33593 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign30740_e33602) + (assign30740_e33597 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign30740_e33604) + (assign30740_e33593 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign30740_e33602) + (assign30740_e33597 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign30740_e33604) + (assign30740_e33593 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign30740_e33602) + (assign30740_e33597 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign30740_e33604) + (assign30740_e33593 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign30740_e33602) + (assign30740_e33597 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30740_e33609;
        locals.var_q_temp3__blk816_dn4 = assign30740_e33609_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30740_e33609_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30740_e33609_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30740_e33609_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30740_e33609_d_n9;

        let (assign30750_e33623, assign30750_e33623_d_n4, assign30750_e33623_d_n6, assign30750_e33623_d_n7, assign30750_e33623_d_n8, assign30750_e33623_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30750_e33620: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign30750_e33621: f64 = (2.0 + assign30750_e33620);
        (assign30750_e33621, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign30750_e33623;
        locals.var_q_qcoth__blk829_dn4 = assign30750_e33623_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign30750_e33623_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign30750_e33623_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign30750_e33623_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign30750_e33623_d_n9;

        let (assign30760_e33651, assign30760_e33651_d_n4, assign30760_e33651_d_n6, assign30760_e33651_d_n7, assign30760_e33651_d_n8, assign30760_e33651_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30760_e33635: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign30760_e33639: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign30760_e33643: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign30760_e33644: f64 = (1.0 - assign30760_e33643);
        let assign30760_e33645: f64 = (assign30760_e33639 * assign30760_e33644);
        let assign30760_e33646: f64 = (1.0 - assign30760_e33645);
        let assign30760_e33647: f64 = (assign30760_e33635 * assign30760_e33646);
        let assign30760_e33648: f64 = (1.0 - assign30760_e33647);
        let assign30760_e33649: f64 = (0.1666666666667 * assign30760_e33648);
        (assign30760_e33649, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign30760_e33646) + (assign30760_e33635 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign30760_e33644) + (assign30760_e33639 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign30760_e33646) + (assign30760_e33635 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign30760_e33644) + (assign30760_e33639 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign30760_e33646) + (assign30760_e33635 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign30760_e33644) + (assign30760_e33639 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign30760_e33646) + (assign30760_e33635 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign30760_e33644) + (assign30760_e33639 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign30760_e33646) + (assign30760_e33635 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign30760_e33644) + (assign30760_e33639 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30760_e33651;
        locals.var_q_temp1__blk814_dn4 = assign30760_e33651_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30760_e33651_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30760_e33651_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30760_e33651_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30760_e33651_d_n9;

        let (assign30770_e33663, assign30770_e33663_d_n4, assign30770_e33663_d_n6, assign30770_e33663_d_n7, assign30770_e33663_d_n8, assign30770_e33663_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30770_e33661: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign30770_e33661, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign30770_e33663;
        locals.var_q_d1_qcoth__blk830_dn4 = assign30770_e33663_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign30770_e33663_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign30770_e33663_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign30770_e33663_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign30770_e33663_d_n9;

        let (assign30780_e33691, assign30780_e33691_d_n4, assign30780_e33691_d_n6, assign30780_e33691_d_n7, assign30780_e33691_d_n8, assign30780_e33691_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30780_e33675: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign30780_e33679: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign30780_e33683: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign30780_e33684: f64 = (1.0 - assign30780_e33683);
        let assign30780_e33685: f64 = (assign30780_e33679 * assign30780_e33684);
        let assign30780_e33686: f64 = (1.0 - assign30780_e33685);
        let assign30780_e33687: f64 = (assign30780_e33675 * assign30780_e33686);
        let assign30780_e33688: f64 = (1.0 - assign30780_e33687);
        let assign30780_e33689: f64 = (0.0055555555556 * assign30780_e33688);
        (assign30780_e33689, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign30780_e33686) + (assign30780_e33675 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign30780_e33684) + (assign30780_e33679 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign30780_e33686) + (assign30780_e33675 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign30780_e33684) + (assign30780_e33679 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign30780_e33686) + (assign30780_e33675 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign30780_e33684) + (assign30780_e33679 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign30780_e33686) + (assign30780_e33675 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign30780_e33684) + (assign30780_e33679 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign30780_e33686) + (assign30780_e33675 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign30780_e33684) + (assign30780_e33679 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30780_e33691;
        locals.var_q_temp2__blk815_dn4 = assign30780_e33691_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30780_e33691_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30780_e33691_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30780_e33691_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30780_e33691_d_n9;

        let (assign30790_e33709, assign30790_e33709_d_n4, assign30790_e33709_d_n6, assign30790_e33709_d_n7, assign30790_e33709_d_n8, assign30790_e33709_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30790_e33701: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign30790_e33704: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign30790_e33706: f64 = (assign30790_e33704 * locals.var_q_temp2__blk815);
        let assign30790_e33707: f64 = (assign30790_e33701 - assign30790_e33706);
        (assign30790_e33707, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign30790_e33704 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign30790_e33704 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign30790_e33704 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign30790_e33704 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign30790_e33704 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign30790_e33709;
        locals.var_q_d2_qcoth__blk832_dn4 = assign30790_e33709_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign30790_e33709_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign30790_e33709_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign30790_e33709_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign30790_e33709_d_n9;

        let (assign30800_e33724, assign30800_e33724_d_n4, assign30800_e33724_d_n6, assign30800_e33724_d_n7, assign30800_e33724_d_n8, assign30800_e33724_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30800_e33718: f64 = (-0.5);
        let assign30800_e33720: f64 = (assign30800_e33718 * locals.var_q_d1_qsq__blk826);
        let assign30800_e33722: f64 = (assign30800_e33720 * locals.var_q_temp3__blk816);
        (assign30800_e33722, (((assign30800_e33718 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign30800_e33720 * locals.var_q_temp3__blk816_dn4)), (((assign30800_e33718 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign30800_e33720 * locals.var_q_temp3__blk816_dn6)), (((assign30800_e33718 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign30800_e33720 * locals.var_q_temp3__blk816_dn7)), (((assign30800_e33718 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign30800_e33720 * locals.var_q_temp3__blk816_dn8)), (((assign30800_e33718 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign30800_e33720 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign30800_e33724;
        locals.var_q_d1_ln__blk835_dn4 = assign30800_e33724_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign30800_e33724_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign30800_e33724_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign30800_e33724_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign30800_e33724_d_n9;

        let (assign30810_e33759, assign30810_e33759_d_n4, assign30810_e33759_d_n6, assign30810_e33759_d_n7, assign30810_e33759_d_n8, assign30810_e33759_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30810_e33733: f64 = (-0.5);
        let assign30810_e33735: f64 = (assign30810_e33733 * locals.var_q_d2_qsq__blk827);
        let assign30810_e33737: f64 = (assign30810_e33735 * locals.var_q_temp3__blk816);
        let assign30810_e33740: f64 = (0.25 * 0.0055555555556);
        let assign30810_e33742: f64 = (assign30810_e33740 * locals.var_q_d1_qsq__blk826);
        let assign30810_e33744: f64 = (assign30810_e33742 * locals.var_q_d1_qsq__blk826);
        let assign30810_e33748: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign30810_e33752: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign30810_e33753: f64 = (2.0 - assign30810_e33752);
        let assign30810_e33754: f64 = (assign30810_e33748 * assign30810_e33753);
        let assign30810_e33755: f64 = (1.0 - assign30810_e33754);
        let assign30810_e33756: f64 = (assign30810_e33744 * assign30810_e33755);
        let assign30810_e33757: f64 = (assign30810_e33737 + assign30810_e33756);
        (assign30810_e33757, ((((assign30810_e33733 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign30810_e33735 * locals.var_q_temp3__blk816_dn4)) + (((((assign30810_e33740 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign30810_e33742 * locals.var_q_d1_qsq__blk826_dn4)) * assign30810_e33755) + (assign30810_e33744 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign30810_e33753) + (assign30810_e33748 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign30810_e33733 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign30810_e33735 * locals.var_q_temp3__blk816_dn6)) + (((((assign30810_e33740 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign30810_e33742 * locals.var_q_d1_qsq__blk826_dn6)) * assign30810_e33755) + (assign30810_e33744 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign30810_e33753) + (assign30810_e33748 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign30810_e33733 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign30810_e33735 * locals.var_q_temp3__blk816_dn7)) + (((((assign30810_e33740 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign30810_e33742 * locals.var_q_d1_qsq__blk826_dn7)) * assign30810_e33755) + (assign30810_e33744 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign30810_e33753) + (assign30810_e33748 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign30810_e33733 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign30810_e33735 * locals.var_q_temp3__blk816_dn8)) + (((((assign30810_e33740 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign30810_e33742 * locals.var_q_d1_qsq__blk826_dn8)) * assign30810_e33755) + (assign30810_e33744 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign30810_e33753) + (assign30810_e33748 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign30810_e33733 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign30810_e33735 * locals.var_q_temp3__blk816_dn9)) + (((((assign30810_e33740 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign30810_e33742 * locals.var_q_d1_qsq__blk826_dn9)) * assign30810_e33755) + (assign30810_e33744 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign30810_e33753) + (assign30810_e33748 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign30810_e33759;
        locals.var_q_d2_ln__blk836_dn4 = assign30810_e33759_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign30810_e33759_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign30810_e33759_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign30810_e33759_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign30810_e33759_d_n9;

        let assign30820_e33762: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1105 = assign30820_e33762;

        let (assign30830_e33778, assign30830_e33778_d_n4, assign30830_e33778_d_n6, assign30830_e33778_d_n7, assign30830_e33778_d_n8, assign30830_e33778_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1105 != 0.0)) {
        let assign30830_e33768: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign30830_e33773: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign30830_e33774: f64 = (locals.var_q_invexpq__blk831 * assign30830_e33773);
        let assign30830_e33775: f64 = (1.0 - assign30830_e33774);
        let assign30830_e33776: f64 = (assign30830_e33768 / assign30830_e33775);
        (assign30830_e33776, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign30830_e33775) - (assign30830_e33768 * (-((locals.var_q_invexpq__blk831_dn4 * assign30830_e33773) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign30830_e33775 * assign30830_e33775)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign30830_e33775) - (assign30830_e33768 * (-((locals.var_q_invexpq__blk831_dn6 * assign30830_e33773) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign30830_e33775 * assign30830_e33775)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign30830_e33775) - (assign30830_e33768 * (-((locals.var_q_invexpq__blk831_dn7 * assign30830_e33773) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign30830_e33775 * assign30830_e33775)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign30830_e33775) - (assign30830_e33768 * (-((locals.var_q_invexpq__blk831_dn8 * assign30830_e33773) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign30830_e33775 * assign30830_e33775)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign30830_e33775) - (assign30830_e33768 * (-((locals.var_q_invexpq__blk831_dn9 * assign30830_e33773) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign30830_e33775 * assign30830_e33775)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30830_e33778;
        locals.var_q_temp2__blk815_dn4 = assign30830_e33778_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30830_e33778_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30830_e33778_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30830_e33778_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30830_e33778_d_n9;

        let (assign30840_e33786, assign30840_e33786_d_n4, assign30840_e33786_d_n6, assign30840_e33786_d_n7, assign30840_e33786_d_n8, assign30840_e33786_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1105 != 0.0)) {
        let assign30840_e33784: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign30840_e33784, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign30840_e33786;
        locals.var_q_sh_term__blk833_dn4 = assign30840_e33786_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign30840_e33786_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign30840_e33786_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign30840_e33786_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign30840_e33786_d_n9;

        let (assign30850_e33795, assign30850_e33795_d_n4, assign30850_e33795_d_n6, assign30850_e33795_d_n7, assign30850_e33795_d_n8, assign30850_e33795_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1105 != 0.0)) {
        let assign30850_e33791: f64 = (locals.var_q_temp2__blk815).ln();
        let assign30850_e33793: f64 = (assign30850_e33791 - locals.var_q_rac_qsq__blk828);
        (assign30850_e33793, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign30850_e33795;
        locals.var_q_ln_term__blk834_dn4 = assign30850_e33795_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign30850_e33795_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign30850_e33795_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign30850_e33795_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign30850_e33795_d_n9;

        let assign30860_e33798: f64 = (-0.005);
        let assign30860_e33799: f64 = if locals.var_q_qsq__blk825 < assign30860_e33798 { 1.0 } else { 0.0 };
        locals.var_guard1106 = assign30860_e33799;

        let (assign30870_e33811, assign30870_e33811_d_n4, assign30870_e33811_d_n6, assign30870_e33811_d_n7, assign30870_e33811_d_n8, assign30870_e33811_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1105 == 0.0)) && (locals.var_guard1106 != 0.0)) {
        let assign30870_e33808: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign30870_e33809: f64 = (assign30870_e33808).sin();
        (assign30870_e33809, ((assign30870_e33808).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign30870_e33808).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign30870_e33808).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign30870_e33808).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign30870_e33808).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30870_e33811;
        locals.var_q_temp2__blk815_dn4 = assign30870_e33811_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30870_e33811_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30870_e33811_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30870_e33811_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30870_e33811_d_n9;

        let (assign30880_e33825, assign30880_e33825_d_n4, assign30880_e33825_d_n6, assign30880_e33825_d_n7, assign30880_e33825_d_n8, assign30880_e33825_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1105 == 0.0)) && (locals.var_guard1106 != 0.0)) {
        let assign30880_e33819: f64 = (-locals.var_q_qsq__blk825);
        let assign30880_e33822: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign30880_e33823: f64 = (assign30880_e33819 / assign30880_e33822);
        (assign30880_e33823, ((((-locals.var_q_qsq__blk825_dn4) * assign30880_e33822) - (assign30880_e33819 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign30880_e33822 * assign30880_e33822)), ((((-locals.var_q_qsq__blk825_dn6) * assign30880_e33822) - (assign30880_e33819 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign30880_e33822 * assign30880_e33822)), ((((-locals.var_q_qsq__blk825_dn7) * assign30880_e33822) - (assign30880_e33819 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign30880_e33822 * assign30880_e33822)), ((((-locals.var_q_qsq__blk825_dn8) * assign30880_e33822) - (assign30880_e33819 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign30880_e33822 * assign30880_e33822)), ((((-locals.var_q_qsq__blk825_dn9) * assign30880_e33822) - (assign30880_e33819 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign30880_e33822 * assign30880_e33822)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign30880_e33825;
        locals.var_q_sh_term__blk833_dn4 = assign30880_e33825_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign30880_e33825_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign30880_e33825_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign30880_e33825_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign30880_e33825_d_n9;

        let (assign30890_e33835, assign30890_e33835_d_n4, assign30890_e33835_d_n6, assign30890_e33835_d_n7, assign30890_e33835_d_n8, assign30890_e33835_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1105 == 0.0)) && (locals.var_guard1106 != 0.0)) {
        let assign30890_e33833: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign30890_e33833, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign30890_e33835;
        locals.var_q_ln_term__blk834_dn4 = assign30890_e33835_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign30890_e33835_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign30890_e33835_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign30890_e33835_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign30890_e33835_d_n9;

        let (assign30900_e33861, assign30900_e33861_d_n4, assign30900_e33861_d_n6, assign30900_e33861_d_n7, assign30900_e33861_d_n8, assign30900_e33861_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1105 == 0.0)) && (locals.var_guard1106 == 0.0)) {
        let assign30900_e33846: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign30900_e33850: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign30900_e33854: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign30900_e33855: f64 = (1.0 - assign30900_e33854);
        let assign30900_e33856: f64 = (assign30900_e33850 * assign30900_e33855);
        let assign30900_e33857: f64 = (1.0 - assign30900_e33856);
        let assign30900_e33858: f64 = (assign30900_e33846 * assign30900_e33857);
        let assign30900_e33859: f64 = (4.0 - assign30900_e33858);
        (assign30900_e33859, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign30900_e33857) + (assign30900_e33846 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign30900_e33855) + (assign30900_e33850 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign30900_e33857) + (assign30900_e33846 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign30900_e33855) + (assign30900_e33850 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign30900_e33857) + (assign30900_e33846 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign30900_e33855) + (assign30900_e33850 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign30900_e33857) + (assign30900_e33846 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign30900_e33855) + (assign30900_e33850 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign30900_e33857) + (assign30900_e33846 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign30900_e33855) + (assign30900_e33850 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign30900_e33861;
        locals.var_q_sh_term__blk833_dn4 = assign30900_e33861_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign30900_e33861_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign30900_e33861_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign30900_e33861_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign30900_e33861_d_n9;

        let (assign30910_e33872, assign30910_e33872_d_n4, assign30910_e33872_d_n6, assign30910_e33872_d_n7, assign30910_e33872_d_n8, assign30910_e33872_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1105 == 0.0)) && (locals.var_guard1106 == 0.0)) {
        let assign30910_e33870: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign30910_e33870, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign30910_e33872;
        locals.var_q_ln_term__blk834_dn4 = assign30910_e33872_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign30910_e33872_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign30910_e33872_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign30910_e33872_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign30910_e33872_d_n9;

        let assign30920_e33875: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign30920_e33877: f64 = (assign30920_e33875 + locals.var_q_qcoth__blk829);
        let assign30920_e33879: f64 = if assign30920_e33877 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1107 = assign30920_e33879;

        let (assign30930_e33887, assign30930_e33887_d_n4, assign30930_e33887_d_n6, assign30930_e33887_d_n7, assign30930_e33887_d_n8, assign30930_e33887_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 != 0.0)) {
        let assign30930_e33885: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign30930_e33885, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign30930_e33887;
        locals.var_q_expnum__blk837_dn4 = assign30930_e33887_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign30930_e33887_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign30930_e33887_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign30930_e33887_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign30930_e33887_d_n9;

        let (assign30940_e33895, assign30940_e33895_d_n4, assign30940_e33895_d_n6, assign30940_e33895_d_n7, assign30940_e33895_d_n8, assign30940_e33895_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 != 0.0)) {
        let assign30940_e33893: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign30940_e33893, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign30940_e33895;
        locals.var_q_d1_expnum__blk838_dn4 = assign30940_e33895_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign30940_e33895_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign30940_e33895_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign30940_e33895_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign30940_e33895_d_n9;

        let (assign30950_e33901, assign30950_e33901_d_n4, assign30950_e33901_d_n6, assign30950_e33901_d_n7, assign30950_e33901_d_n8, assign30950_e33901_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign30950_e33901;
        locals.var_q_d2_expnum__blk839_dn4 = assign30950_e33901_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign30950_e33901_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign30950_e33901_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign30950_e33901_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign30950_e33901_d_n9;

        let (assign30960_e33912, assign30960_e33912_d_n4, assign30960_e33912_d_n6, assign30960_e33912_d_n7, assign30960_e33912_d_n8, assign30960_e33912_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 == 0.0)) {
        let assign30960_e33909: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign30960_e33910: f64 = (1.0 / assign30960_e33909);
        (assign30960_e33910, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign30960_e33909 * assign30960_e33909))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign30960_e33909 * assign30960_e33909))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign30960_e33909 * assign30960_e33909))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign30960_e33909 * assign30960_e33909))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign30960_e33909 * assign30960_e33909))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30960_e33912;
        locals.var_q_temp2__blk815_dn4 = assign30960_e33912_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30960_e33912_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30960_e33912_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30960_e33912_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30960_e33912_d_n9;

        let (assign30970_e33921, assign30970_e33921_d_n4, assign30970_e33921_d_n6, assign30970_e33921_d_n7, assign30970_e33921_d_n8, assign30970_e33921_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 == 0.0)) {
        let assign30970_e33919: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign30970_e33919, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30970_e33921;
        locals.var_q_temp3__blk816_dn4 = assign30970_e33921_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30970_e33921_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30970_e33921_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30970_e33921_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30970_e33921_d_n9;

        let (assign30980_e33932, assign30980_e33932_d_n4, assign30980_e33932_d_n6, assign30980_e33932_d_n7, assign30980_e33932_d_n8, assign30980_e33932_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 == 0.0)) {
        let assign30980_e33928: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign30980_e33930: f64 = (assign30980_e33928 * locals.var_q_temp2__blk815);
        (assign30980_e33930, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign30980_e33928 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign30980_e33928 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign30980_e33928 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign30980_e33928 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign30980_e33928 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign30980_e33932;
        locals.var_q_expnum__blk837_dn4 = assign30980_e33932_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign30980_e33932_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign30980_e33932_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign30980_e33932_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign30980_e33932_d_n9;

    }
}
